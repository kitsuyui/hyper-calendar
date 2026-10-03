#!/usr/bin/env python3
"""Regenerate crates/hc-humanize/src/natural/catalogues.rs from the gettext
catalogues of the Python `humanize` package, release 4.16.0.

`humanize` translates through gettext: one `humanize.po` for each of its
languages, compiled by `msgfmt` (scripts/generate-translation-binaries.sh in
the project, with no `--use-fuzzy`) into the `humanize.mo` that
`humanize.i18n.activate("ru_RU")` loads. This script reads each `.po` of the
tag `4.16.0` of github.com/python-humanize/humanize, together with
`src/humanize/i18n.py` for the number separators it keeps in code, over
HTTP from raw.githubusercontent.com into memory; nothing is saved.

    python3 scripts/humanize-gettext.py            # rewrite the file
    python3 scripts/humanize-gettext.py --check    # exit 1 if it is stale

What a catalogue gives is what `msgfmt` compiles: an entry that is fuzzy, or
whose translation is empty, is left out, and `gettext` then answers the
English of the source; for a plural message, the singular for 1 and the
plural for any other count. The script writes such a message as the English
one (`Plural::english`), a translated one with its forms
(`Plural::translated`), and the form a count takes is chosen at run time by
the catalogue's own `Plural-Forms` expression (`natural::gettext`), which
this script copies as written.

The ordinal suffixes are `pgettext` entries whose contexts read `3 (male)`;
`apnumber`'s words, the names of the powers, the `%d Byte` and the sizes'
suffixes are plain or plural messages.
"""
import http.client
import json
import os
import re
import sys
import time
import urllib.request

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUTPUT = os.path.join(ROOT_DIR, 'crates/hc-humanize/src/natural/catalogues.rs')
REF = '4.16.0'
RAW = f'https://raw.githubusercontent.com/python-humanize/humanize/{REF}/'
API = 'https://api.github.com/repos/python-humanize/humanize/contents/src/humanize/locale'


def fetch(url):
    for attempt in range(6):
        try:
            request = urllib.request.Request(url, headers={'User-Agent': 'hyper-calendar'})
            with urllib.request.urlopen(request, timeout=60) as response:
                return response.read().decode('utf-8')
        except (OSError, http.client.HTTPException):
            if attempt == 5:
                raise
            time.sleep(1 + attempt)


# --- reading a catalogue -----------------------------------------------------

def unquote(token):
    return json.loads(token)


def parse(text):
    """The entries of a .po file: dicts of msgctxt, msgid, msgid_plural,
    msgstr (a list) and flags."""
    entries, cur, field, flags = [], None, None, []

    def finish():
        nonlocal cur, field, flags
        if cur is not None and 'msgid' in cur:
            cur['flags'] = flags
            cur['msgstr'] = [cur[k] for k in sorted((k for k in cur if k.startswith('msgstr')))]
            entries.append(cur)
        cur, field, flags = None, None, []

    for raw in text.splitlines() + ['']:
        line = raw.strip()
        if not line:
            finish()
            continue
        if cur is None:
            cur = {}
        if line.startswith('#,'):
            flags = [f.strip() for f in line[2:].split(',')]
            continue
        if line.startswith('#'):
            continue
        match = re.match(r'(msgctxt|msgid_plural|msgid|msgstr\[\d+\]|msgstr)\s+(".*")$', line)
        if match:
            field = match.group(1)
            cur[field] = unquote(match.group(2))
        elif line.startswith('"') and field:
            cur[field] += unquote(line)
        else:
            raise SystemExit(f'cannot read the line: {raw}')
    finish()
    return entries


def compiled(entries):
    """What msgfmt keeps: (msgctxt, msgid) → the list of forms, for each entry
    that is not fuzzy and not empty."""
    kept = {}
    for entry in entries:
        if not entry['msgid']:
            continue
        forms = entry['msgstr']
        if 'fuzzy' in entry['flags'] or not any(forms):
            continue
        if not all(forms):
            raise SystemExit(f'a plural entry with an empty form: {entry}')
        kept[(entry.get('msgctxt'), entry['msgid'])] = forms
    return kept


def header(entries):
    text = next(e for e in entries if e['msgid'] == '')['msgstr'][0]
    fields = dict(line.split(': ', 1) for line in text.splitlines() if ': ' in line)
    return fields


# --- what each field of NaturalPhrases takes ---------------------------------

SIMPLE = [
    ('a_moment', 'a moment'), ('a_second', 'a second'), ('a_minute', 'a minute'),
    ('an_hour', 'an hour'), ('a_day', 'a day'), ('a_month', 'a month'),
    ('a_year', 'a year'), ('one_year_one_month', '1 year, 1 month'),
    ('now', 'now'), ('ago', '%s ago'), ('from_now', '%s from now'),
    ('today', 'today'), ('tomorrow', 'tomorrow'), ('yesterday', 'yesterday'),
    ('byte', '%d Byte'), ('bytes', '%d Bytes'),
]
PLURAL = [
    ('one_year_days', '1 year, %d day', '1 year, %d days'),
    ('one_year_months', '1 year, %d month', '1 year, %d months'),
    ('microseconds', '%d microsecond', '%d microseconds'),
    ('milliseconds', '%d millisecond', '%d milliseconds'),
    ('seconds', '%d second', '%d seconds'),
    ('minutes', '%d minute', '%d minutes'),
    ('hours', '%d hour', '%d hours'),
    ('days', '%d day', '%d days'),
    ('months', '%d month', '%d months'),
    ('years', '%d year', '%d years'),
]
POWERS = ['thousand', 'million', 'billion', 'trillion', 'quadrillion', 'quintillion',
          'sextillion', 'septillion', 'octillion', 'nonillion', 'decillion', 'googol']
WORDS = ['zero', 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']
DECIMAL = ['kB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB', 'RB', 'QB']
BINARY = ['KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB', 'ZiB', 'YiB', 'RiB', 'QiB']
ORDINAL_MSGIDS = ['th', 'st', 'nd', 'rd', 'th', 'th', 'th', 'th', 'th', 'th']

# Every message the code asks gettext for, so that an entry the catalogue has
# and the code never reads is noticed.
ASKED = set()


def convert(text, source):
    """`%d` and `%s` as the `{0}` this crate writes; `%s and %s` as `{0}` and
    `{1}`."""
    if source == '%s and %s':
        placeholders = re.findall(r'%s', text)
        if len(placeholders) != 2:
            raise SystemExit(f'{text!r} does not hold two %s')
        return text.replace('%s', '{0}', 1).replace('%s', '{1}', 1)
    wanted = len(re.findall(r'%[ds]', source))
    out = re.sub(r'%[ds]', '{0}', text)
    if '%' in out or out.count('{0}') != wanted:
        raise SystemExit(f'{text!r} does not hold what {source!r} does')
    return out


def rust(text):
    return json.dumps(text, ensure_ascii=False)


def phrases_for(name, kept, head, separators):
    out = []
    asked = ASKED
    english = 'NaturalPhrases::ENGLISH'

    def simple(field, msgid, ctx=None):
        asked.add((ctx, msgid))
        forms = kept.get((ctx, msgid))
        return rust(convert(forms[0], msgid)) if forms else f'{english}.{field}'

    def plural(field, one, other):
        asked.add((None, one))
        forms = kept.get((None, one))
        if not forms:
            return f'{english}.{field}'
        listed = ', '.join(rust(convert(form, one)) for form in forms)
        return f'Plural::translated({rust(convert(one, one))}, {rust(convert(other, other))}, &[{listed}])'

    language = name.replace('_', '-')
    thousands, decimal = separators.get(name, (',', '.'))
    out.append(f'        catalogue: {rust(name)},')
    out.append(f'        language: {rust(language)},')
    out.append(f'        plural_expression: {rust(head["Plural-Forms"].split("plural=", 1)[1].rstrip(";"))},')
    out.append(f'        grouping: Grouping {{ thousands: {rust(thousands)}, decimal: {rust(decimal)} }},')
    for field, msgid in SIMPLE:
        out.append(f'        {field}: {simple(field, msgid)},')
    for field, one, other in PLURAL:
        out.append(f'        {field}: {plural(field, one, other)},')
    out.append(f'        list_separator: {english}.list_separator,')
    asked.add((None, '%s and %s'))
    forms = kept.get((None, '%s and %s'))
    out.append('        list_last: ' + (rust(convert(forms[0], '%s and %s')) if forms else f'{english}.list_last') + ',')
    for gender in ('male', 'female'):
        field = 'ordinal_by_last_digit' + ('' if gender == 'male' else '_female')
        cells = []
        for digit, msgid in enumerate(ORDINAL_MSGIDS):
            ctx = f'{digit} ({gender})'
            asked.add((ctx, msgid))
            forms = kept.get((ctx, msgid))
            cells.append(rust(forms[0]) if forms else f'{english}.{field}[{digit}]')
        out.append(f'        {field}: [{", ".join(cells)}],')
    powers = []
    for index, word in enumerate(POWERS):
        asked.add((None, word))
        forms = kept.get((None, word))
        powers.append(
            f'Plural::translated({rust(word)}, {rust(word)}, &[{", ".join(rust(f) for f in forms)}])'
            if forms else f'{english}.powers[{index}]')
    out.append('        powers: [' + ', '.join(powers) + '],')
    cells = []
    for index, word in enumerate(WORDS):
        asked.add((None, word))
        forms = kept.get((None, word))
        cells.append(rust(forms[0]) if forms else f'{english}.apnumber[{index}]')
    out.append('        apnumber: [' + ', '.join(cells) + '],')
    for field, msgids in (('size_decimal', DECIMAL), ('size_binary', BINARY)):
        cells = []
        for index, msgid in enumerate(msgids):
            asked.add((None, msgid))
            forms = kept.get((None, msgid))
            cells.append(rust(forms[0]) if forms else f'{english}.{field}[{index}]')
        out.append(f'        {field}: [{", ".join(cells)}],')
    out.append(f'        size_gnu: {english}.size_gnu,')
    return out


def separators_of(i18n):
    """The thousands and decimal separators i18n.py keeps in code."""
    def table(name):
        block = re.search(name + r'[^=]*=\s*\{(.*?)\}', i18n, re.S).group(1)
        return dict(re.findall(r'"([A-Za-z_]+)":\s*"(.)"', block))
    thousands, decimal = table('_THOUSANDS_SEPARATOR'), table('_DECIMAL_SEPARATOR')
    return {key: (thousands[key], decimal[key]) for key in thousands}


def rustfmt(text):
    import subprocess
    result = subprocess.run(['rustfmt', '--edition', '2024', '--emit', 'stdout'],
                            input=text.encode(), capture_output=True, check=True)
    return result.stdout.decode()


def main():
    listing = json.loads(fetch(f'{API}?ref={REF}'))
    names = sorted(entry['name'] for entry in listing if entry['type'] == 'dir')
    i18n = fetch(RAW + 'src/humanize/i18n.py')
    separators = separators_of(i18n)
    lines = [
        '//! The gettext catalogues of the Python `humanize` package, as',
        '//! [`NaturalPhrases`] values.',
        '//!',
        f'//! Generated by `scripts/humanize-gettext.py` from the `.po` files of the',
        f'//! tag `{REF}` of <https://github.com/python-humanize/humanize> (MIT',
        '//! licence; the strings are its translators\') and its `i18n.py`, read into',
        '//! memory on 2026-10-03; do not edit. A message `msgfmt` leaves out of the',
        '//! compiled catalogue, because it is fuzzy or untranslated, is the English',
        '//! one here, as it is in Python.',
        '',
        'use super::{Grouping, NaturalPhrases, Plural};',
        '',
    ]
    consts = []
    stats = []
    for name in names:
        text = fetch(RAW + f'src/humanize/locale/{name}/LC_MESSAGES/humanize.po')
        entries = parse(text)
        head = header(entries)
        kept = compiled(entries)
        total = len([e for e in entries if e['msgid']])
        const = name.upper().replace('-', '_')
        consts.append((name, const))
        stats.append((name, total, len(kept)))
        body = phrases_for(name, kept, head, separators)
        unused = [k for k in kept if k not in ASKED]
        if unused:
            raise SystemExit(f'{name}: entries the code never asks for: {unused}')
        lines.append(f'/// `{name}`: {len(kept)} of its {total} entries, the rest fuzzy or untranslated.')
        lines.append(f'pub(super) const {const}: NaturalPhrases = NaturalPhrases {{')
        lines.extend(body)
        lines.append('};')
        lines.append('')
    lines.append('/// Every catalogue, by the name `humanize.i18n.activate` takes.')
    lines.append(f'pub(super) const ALL: [&NaturalPhrases; {len(consts)}] = [')
    for _, const in consts:
        lines.append(f'    &{const},')
    lines.append('];')
    text = rustfmt('\n'.join(lines) + '\n')
    if '--check' in sys.argv:
        current = open(OUTPUT, encoding='utf-8').read() if os.path.exists(OUTPUT) else ''
        if current != text:
            print(f'{OUTPUT} is stale; run scripts/humanize-gettext.py', file=sys.stderr)
            sys.exit(1)
        return
    with open(OUTPUT, 'w', encoding='utf-8') as out:
        out.write(text)
    for name, total, kept in stats:
        print(f'{name}: {kept} of {total}')


if __name__ == '__main__':
    main()
