#!/usr/bin/env python3
"""Regenerate the rows of crates/hc-holiday/src/countries/india.rs from the
Reserve Bank of India's lists of holidays under the Negotiable Instruments
Act.

The `STATE_DAYS` table of India's states' days, from `lists_read` for each
state's years to the last `nia` row:

    python3 scripts/india-rbi-holidays.py            # rewrite the table
    python3 scripts/india-rbi-holidays.py --check    # exit 1 if it is stale
    python3 scripts/india-rbi-holidays.py --renamed  # print each name cut, a TSV

The rest of the file (its documentation, the states' constants, `nia` and
`lists_read`) is written by hand, and a state added here needs its
constants there.

The lists are those of rbi.org.in's "Holidays under Negotiable Instruments
Act" page (Scripts/HolidayMatrixDisplay.aspx), one for each of the 34
regional offices and each year from FIRST to LAST, asked for through the
page's form (a GET for the form's hidden fields, then a POST of the office,
"All Months" and the year) and read over HTTP into memory; nothing is
saved. That is 272 requests, a few at a time. The script needs the network
and cannot run offline, so CI does not run it.

A state's day is a day listed at the state's office, or at every one of its
offices. An election at a state's one office is left out, since the list
does not say how far in the state it runs.

The list gives one description per date for all the offices, joining with
`/` the names the day has wherever it is a holiday. A state's name is the
description at its office, split at each `/` outside brackets, less each
part the list shows the office not keeping (see `left_out`). Where no part
is left out the text is the list's, as printed. Maharashtra's names for
2026 are the parts its own list for the year names (MAHARASHTRA_2026).
Days with the same date and name are one rule, scoped to every state that
keeps them.
"""
import collections
import datetime
import html
import http.client
import http.cookiejar
import os
import re
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from concurrent.futures import ThreadPoolExecutor

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUTPUT = os.path.join(ROOT_DIR, 'crates/hc-holiday/src/countries/india.rs')
URL = 'https://www.rbi.org.in/Scripts/HolidayMatrixDisplay.aspx'
FIRST, LAST = 2019, 2026

# The form's value for each office.
OFFICES = {
    'Agartala': 72, 'Ahmedabad': 1, 'Aizawl': 75, 'Belapur': 66, 'Bengaluru': 3,
    'Bhopal': 10, 'Bhubaneswar': 11, 'Chandigarh': 14, 'Chennai': 16,
    'Dehradun': 71, 'Gangtok': 67, 'Guwahati': 17, 'Hyderabad': 18,
    'Imphal': 74, 'Itanagar': 78, 'Jaipur': 19, 'Jammu': 22, 'Kanpur': 13,
    'Kochi': 24, 'Kohima': 77, 'Kolkata': 26, 'Lucknow': 27, 'Mumbai': 28,
    'Nagpur': 29, 'New Delhi': 30, 'Panaji': 31, 'Patna': 32, 'Raipur': 70,
    'Ranchi': 69, 'Shillong': 73, 'Shimla': 68, 'Srinagar': 33,
    'Thiruvananthapuram': 34, 'Vijayawada': 76,
}

# The states carried, in the table's order: the code, the constant naming
# it in india.rs, the constant of the year before which its holidays are
# absent, and its offices. Chandigarh's office is not a state's: whose days
# its list gives was not established.
STATES = [
    ('IN-UP', 'UTTAR_PRADESH', 'NI_ACT_IN_FORCE', ['Lucknow', 'Kanpur']),
    ('IN-MH', 'MAHARASHTRA', 'BOMBAY_REORGANISATION', ['Mumbai', 'Belapur', 'Nagpur']),
    ('IN-BR', 'BIHAR', 'NI_ACT_IN_FORCE', ['Patna']),
    ('IN-WB', 'WEST_BENGAL', 'NI_ACT_IN_FORCE', ['Kolkata']),
    ('IN-MP', 'MADHYA_PRADESH', 'NI_ACT_IN_FORCE', ['Bhopal']),
    ('IN-TN', 'TAMIL_NADU', 'NI_ACT_IN_FORCE', ['Chennai']),
    ('IN-RJ', 'RAJASTHAN', 'NI_ACT_IN_FORCE', ['Jaipur']),
    ('IN-KA', 'KARNATAKA', 'NI_ACT_IN_FORCE', ['Bengaluru']),
    ('IN-GJ', 'GUJARAT', 'BOMBAY_REORGANISATION', ['Ahmedabad']),
    ('IN-AP', 'ANDHRA_PRADESH', 'STATES_REORGANISATION', ['Vijayawada']),
    ('IN-OD', 'ODISHA', 'NI_ACT_IN_FORCE', ['Bhubaneswar']),
    ('IN-TS', 'TELANGANA', 'ANDHRA_PRADESH_REORGANISATION', ['Hyderabad']),
    ('IN-KL', 'KERALA', 'STATES_REORGANISATION', ['Thiruvananthapuram', 'Kochi']),
    ('IN-AS', 'ASSAM', 'NOT_ESTABLISHED', ['Guwahati']),
    ('IN-JH', 'JHARKHAND', 'REORGANISATION_2000', ['Ranchi']),
    ('IN-DL', 'DELHI', 'NOT_ESTABLISHED', ['New Delhi']),
    ('IN-JK', 'JAMMU_AND_KASHMIR', 'NOT_ESTABLISHED', ['Jammu', 'Srinagar']),
    ('IN-UK', 'UTTARAKHAND', 'REORGANISATION_2000', ['Dehradun']),
    ('IN-CG', 'CHHATTISGARH', 'REORGANISATION_2000', ['Raipur']),
    ('IN-HP', 'HIMACHAL_PRADESH', 'NOT_ESTABLISHED', ['Shimla']),
    ('IN-TR', 'TRIPURA', 'NOT_ESTABLISHED', ['Agartala']),
    ('IN-ML', 'MEGHALAYA', 'NORTH_EASTERN_AREAS_REORGANISATION', ['Shillong']),
    ('IN-MN', 'MANIPUR', 'NOT_ESTABLISHED', ['Imphal']),
    ('IN-NL', 'NAGALAND', 'NAGALAND_STATEHOOD', ['Kohima']),
    ('IN-GA', 'GOA', 'GOA_ANNEXATION', ['Panaji']),
    ('IN-AR', 'ARUNACHAL_PRADESH', 'NORTH_EASTERN_AREAS_REORGANISATION', ['Itanagar']),
    ('IN-MZ', 'MIZORAM', 'NORTH_EASTERN_AREAS_REORGANISATION', ['Aizawl']),
    ('IN-SK', 'SIKKIM', 'SIKKIM_STATEHOOD', ['Gangtok']),
]

# Maharashtra's own list for 2026, notification PHD-1125/C.R.199/Japuk (29)
# of 5 December 2025, as The Live Nagpur reports it (2026-01-01, read
# 2026-09-29): on each of these days, the parts of the Reserve Bank's
# description that name the state's holidays, in the Reserve Bank's
# spelling. 15 January, not on the state's list, is the part that names
# the state's election.
MAHARASHTRA_2026 = {
    (1, 15): ['Election to Municipal Corporations in Maharashtra'],
    (3, 3): ['Holi (Second Day)'],
    (3, 19): ['Gudhi Padwa'],
    (3, 21): ['Ramzan-Id (Id-Ul-Fitr) (Shawal-1)'],
    (3, 31): ['Mahavir Janmakalyanak'],
    (4, 14): ['Dr. Babasaheb Ambedkar Jayanti'],
    (5, 1): ['Maharashtra Din', 'Buddha Pournima'],
    (8, 15): ['Independence Day', 'Parsi New Year (Shahenshahi)'],
    (8, 26): ['Id-E-Milad'],
    (9, 14): ['Ganesh Chaturthi'],
    (10, 20): ['Dasara'],
    (11, 10): ['Diwali (Bali Pratipada)'],
    (11, 24): ['Guru Nanak Jayanti'],
}

ELECTION = re.compile(r'(?i)election|poll')
MONTHS = ['January', 'February', 'March', 'April', 'May', 'June', 'July',
          'August', 'September', 'October', 'November', 'December']


# --- reading the lists ------------------------------------------------------

def request(opener, data=None):
    body = urllib.parse.urlencode(data).encode() if data is not None else None
    req = urllib.request.Request(URL, data=body, headers={'User-Agent': 'Mozilla/5.0'})
    with opener.open(req, timeout=60) as response:
        return response.read().decode('utf-8', 'replace')


def parse(page):
    """The office, the year and the rows (month, day, description) of a
    list, or None when the page has no list."""
    text = re.sub(r'<script.*?</script>|<style.*?</style>', '', page, flags=re.S)
    text = re.sub(r'</(tr|p|h\d|li|div)>', '\n', text)
    text = re.sub(r'</t[dh]>', ' | ', text)
    text = html.unescape(re.sub(r'<[^>]+>', '', text))
    text = re.sub(r'[ \t]+', ' ', text)
    found = re.search(r'(\S[^\n]*?) regional office holiday list for the year (\d{4})', text)
    if not found:
        return None
    office, year = found.group(1).strip(), int(found.group(2))
    body = text[found.end():text.find('All scheduled')]
    month, rows = None, []
    for line in body.split('\n'):
        line = line.strip()
        named = re.match(r'^(%s) \|$' % '|'.join(MONTHS), line)
        if named:
            month = MONTHS.index(named.group(1)) + 1
            continue
        row = re.match(r'^(\d+) \| (.*?) \| (.*?) \|$', line)
        if row and month:
            rows.append((month, int(row.group(1)), row.group(2).strip()))
    return office, year, rows


def fetch(office, year):
    """An office's list for a year: its rows, or None where it has none."""
    for attempt in range(8):
        try:
            opener = urllib.request.build_opener(
                urllib.request.HTTPCookieProcessor(http.cookiejar.CookieJar()))
            form = request(opener)
            data = {m.group(1): html.unescape(m.group(2)) for m in re.finditer(
                r'<input type="hidden" name="([^"]+)" id="[^"]*" value="([^"]*)"', form)}
            if '__VIEWSTATE' not in data:
                raise OSError('no form')
            data.update({'drRegionalOffice': str(OFFICES[office]), 'drMonth': '0',
                         'drYear': str(year), 'btnGo': 'GO'})
            page = request(opener, data)
            if 'regional office holiday list' not in page and 'There are no holidays' not in page:
                raise OSError('no list')
            parsed = parse(page)
            if parsed is None:
                return None
            if parsed[:2] != (office, year):
                raise OSError(f'asked for {office} {year}, got {parsed[:2]}')
            return parsed[2]
        except (OSError, http.client.HTTPException, urllib.error.URLError):
            if attempt == 7:
                raise
            time.sleep(2 + 3 * attempt)


def read_lists():
    keys = [(office, year) for office in OFFICES for year in range(FIRST, LAST + 1)]
    with ThreadPoolExecutor(4) as pool:
        lists = list(pool.map(lambda key: fetch(*key), keys))
    return {key: rows for key, rows in zip(keys, lists) if rows}


# --- the names ----------------------------------------------------------------

def split(description):
    """The parts of a description: split at each `/` outside brackets."""
    parts, depth, part = [], 0, ''
    for ch in description:
        if ch == '(':
            depth += 1
        if ch == ')':
            depth = max(0, depth - 1)
        if ch == '/' and depth == 0:
            parts.append(part.strip())
            part = ''
        else:
            part += ch
    parts.append(part.strip())
    return [p for p in parts if p]


def norm(part):
    return re.sub(r'\s+', ' ', part.replace('’', "'").lower()).strip(' .')


def near(date, days):
    day = datetime.date(*date) + datetime.timedelta(days=days)
    return (day.year, day.month, day.day)


class Lists:
    def __init__(self, lists):
        self.at = collections.defaultdict(set)  # date -> offices listing it
        self.desc = {}                           # (date, office) -> description
        self.listed = collections.defaultdict(set)
        self.years = collections.defaultdict(set)
        for (office, year), rows in lists.items():
            self.years[office].add(year)
            for month, day, description in rows:
                date = (year, month, day)
                self.at[date].add(office)
                self.desc[date, office] = description
                self.listed[office].add(date)
        self.parts = {key: split(text) for key, text in self.desc.items()}
        # (year, part) -> the dates bearing it, and those it is the whole of
        bearing, whole = collections.defaultdict(set), collections.defaultdict(set)
        for (date, _), parts in self.parts.items():
            for part in parts:
                bearing[date[0], norm(part)].add(date)
            if len(parts) == 1:
                whole[date[0], norm(parts[0])].add(date)
        # An office keeps a part if the part is the whole description of a
        # date it lists, in any year.
        self.keeps = {(office, part) for (_, part), dates in whole.items()
                      for date in dates for office in self.at[date]}
        # An office does not keep a part if, in some year, the part is the
        # whole description of a date, and the office lists no date that
        # year bearing it, nor the day before or after such a date unless
        # every office keeping the date keeps that day too.
        self.not_kept = set()
        for (year, part), dates in whole.items():
            for office in OFFICES:
                if year not in self.years[office]:
                    continue
                if any(office in self.at[d] for d in bearing[year, part]):
                    continue
                if any(near(d, n) in self.listed[office]
                       and not self.at[d] <= self.at.get(near(d, n), set())
                       for d in dates for n in (-1, 1)):
                    continue
                self.not_kept.add((office, part))

    def left_out(self, office, part):
        key = (office, norm(part))
        return key in self.not_kept and key not in self.keeps

    def kept(self, office, date):
        """The parts of a date's description the office keeps: all, unless
        a part left in is one the office is shown keeping."""
        parts = self.parts[date, office]
        left = [p for p in parts if not self.left_out(office, p)]
        if any((office, norm(p)) in self.keeps for p in left):
            return left
        return parts


def rows(lists):
    """The table's lines, the names cut, and the states' first years."""
    data = Lists(lists)
    days, renamed, firsts = [], [], {}
    for code, const, _, offices in STATES:
        years = [y for y in range(FIRST, LAST + 1) if all((o, y) in lists for o in offices)]
        firsts[code] = years[0]
        for year in years:
            dates = [{(year, m, d) for m, d, _ in lists[o, year]} for o in offices]
            for date in sorted(set.intersection(*dates)):
                parts = data.parts[date, offices[0]]
                if len(offices) == 1 and all(ELECTION.search(p) for p in parts):
                    continue
                kept = [{norm(p) for p in data.kept(o, date)} for o in offices]
                chosen = [p for p in parts if all(norm(p) in k for k in kept)]
                if code == 'IN-MH' and year == 2026 and date[1:] in MAHARASHTRA_2026:
                    chosen = MAHARASHTRA_2026[date[1:]]
                    missing = [p for p in chosen if p not in parts]
                    if missing:
                        sys.exit(f'Maharashtra 2026-{date[1]}-{date[2]}: {missing} not in {parts}')
                full = data.desc[date, offices[0]]
                name = full if chosen == parts else '/'.join(chosen)
                if name != full:
                    renamed.append(f'{code}\t{date[0]}-{date[1]:02}-{date[2]:02}\t{name}\t{full}')
                days.append((date, name, code))
    grouped = collections.defaultdict(list)
    for date, name, code in days:
        grouped[date, name].append(code)
    order = [state[0] for state in STATES]
    lines = []
    for code, const, established, _ in STATES:
        first = 'FIRST' if firsts[code] == FIRST else 'FIRST_NEW_OFFICES'
        lines.append(f'    lists_read({first}, {established}, {const}, {const}_SOURCE),')
    for (date, name) in sorted(grouped):
        regions = ', '.join(f'"{r}"' for r in sorted(grouped[date, name], key=order.index))
        text = name.replace('\\', '\\\\').replace('"', '\\"')
        lines.append(f'    nia({date[0]}, {date[1]}, {date[2]}, "{text}", &[{regions}]),')
    return lines, renamed, len(days)


# --- the file -------------------------------------------------------------------

START = "#[rustfmt::skip]\npub static STATE_DAYS: &[HolidayRule] = &[\n"
END = "\n];\n"


def render(lines):
    with open(OUTPUT, encoding='utf-8') as handle:
        current = handle.read()
    start = current.index(START) + len(START)
    end = current.index(END, start)
    return current, current[:start] + '\n'.join(lines) + current[end:]


def main():
    lines, renamed, count = rows(read_lists())
    if '--renamed' in sys.argv:
        print('\n'.join(renamed))
        return
    current, text = render(lines)
    if '--check' in sys.argv:
        if current != text:
            print(f'{OUTPUT} is stale: run scripts/india-rbi-holidays.py', file=sys.stderr)
            sys.exit(1)
        return
    with open(OUTPUT, 'w', encoding='utf-8') as handle:
        handle.write(text)
    print(f'{count} state days, {len(lines) - len(STATES)} rules', file=sys.stderr)


if __name__ == '__main__':
    main()
