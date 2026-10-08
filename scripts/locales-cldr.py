#!/usr/bin/env python3
"""Regenerate crates/hc-i18n/src/data/cldr48_locales.rs from Unicode CLDR 48.

The file holds the hc-i18n entries this script reads out of CLDR 48 rather
than the hand-checked entries of src/data.rs, and CLDR 48's `parentLocales`:

    python3 scripts/locales-cldr.py            # rewrite the file
    python3 scripts/locales-cldr.py --check    # exit 1 if it is stale
    python3 scripts/locales-cldr.py --dump     # print every carried value, a TSV

The files are the `release-48` tag of unicode-org/cldr, `common/main/
<file>.xml` with `root.xml` for its aliases, `common/main/en.xml` for the
languages' English names and `common/supplemental/supplementalData.xml` for
`parentLocales` and `weekData`, read over HTTP from raw.githubusercontent.com
into memory; nothing is saved. With CLDR_DIR set to a checkout of that tag,
a file present under it is read from there instead.

Two kinds of entry are generated.

* A language (`mn`): every group of names its file states at a release
  level (`approved` or `contributed`, or no draft attribute) — each
  calendar's months, the weekdays, the day periods, the quarters and each
  calendar's eras — as the entries of the most-spoken languages in
  src/data.rs were read, with the widths CLDR's inheritance and `root.xml`'s
  aliases give them. A width only root gives (root's numbered narrow months,
  its English initials for the narrow weekdays, which are not the language's
  own names), or that the file states in part beside root's numerals (`ps`'s
  stand-alone narrow months), is left empty and answered by the next wider
  one, and a group whose format wide names are root's is left out.
* A regional variant (`en-001`, `en-GB`, `es-419`, `zh-Hant-HK`, `ur-IN`,
  `ar-EG`): the groups its files resolve to other values than its CLDR
  parent does — the parent `parentLocales` names, else the truncated tag —
  each whole, with the same widths; every other group is left empty, so
  that the lookup inherits the parent entry's, as CLDR's inheritance does.
  A regional file that writes the inheritance marker `↑↑↑` or repeats its
  parent's value states nothing of its own here.

The hand-written entries of src/data.rs that follow a CLDR file (`HAND_ENTRIES`,
the tags `cldr_xml.ENTRIES` lists apart from the regional ones, and `pt-PT`)
are also written by this script, in place: each entry's `gregorian(...)` call,
its months, eras and quarters, and its `weekdays:` field are replaced by the
groups its files resolve to, by the rules above, with `--check` failing where
one differs. A group no file of an entry states is left as written, and so is
the rest of the file.

The other calendars of the hand-written entries are generated too, into
crates/hc-i18n/src/data/cldr48_calendars.rs: for each entry of `HAND_ENTRIES`,
every CLDR calendar its files state months or eras for — Buddhist, Minguo,
Hijri, Hebrew, Coptic, Ethiopic, Persian, Indian national, and the Chinese
and Dangi where the file names the months — that no hand-written entry of
`src/data.rs` already serves, each with the templates its own date formats
give where they differ from the entry's Gregorian ones (the `Gy` item, the
`d` item and the long date format, resolved through `root.xml`'s aliases to
the `generic` calendar's). The entry's `cldr_calendars:` field names the
constant, and the lookup reads it after the hand-written `calendars`.

Every entry also has its date templates, which do not inherit between
entries, read from the resolved Gregorian `Gy` item (the year with its
era), `d` item (the day) and `yMMMMd` item, else the long date format (the
whole date); its calendar names, where they differ from the parent's; its
default numbering system; and its first day of the week, the region's
`weekData/firstDay` (the language's likely region for `mn`, Mongolia). A
regional entry takes its script, direction, casing and capitalisation from
its parent entry.

Each carried entry's default numbering system is also carried beside the
entries, resolved from its files, with every regional file of a carried
language that resolves to another system than the entry its tag falls back
to: `ar_SA.xml` writes `arab` where `ar.xml` inherits root's `latn`. The
regional files are those `supplementalData.xml`'s `territoryInfo` lists the
language for (`ar_SA`, `ar_MA`, …) that the release has, read like the rest.

`parentLocales` (the default component) is carried whole, as (child,
parent) pairs, for `hc_i18n::locale::Locale::parent`: `en-GB` → `en-001`,
`es-MX` → `es-419`, `zh-Hant-MO` → `zh-Hant-HK`, `pt-AO` → `pt-PT`, and the
`nonlikelyScript` rows such as `zh-Hant` → root, whose listed locales are
carried and whose rule for the unlisted ones is not.
"""
import os
import re
import sys
import textwrap

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cldr_xml import (ENTRIES, exists, lit, path, resolve, rustfmt, stated,  # noqa: E402
                      write_or_check, xml)

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUTPUT = os.path.join(ROOT_DIR, 'crates/hc-i18n/src/data/cldr48_locales.rs')
CALENDARS = os.path.join(ROOT_DIR, 'crates/hc-i18n/src/data/cldr48_calendars.rs')
DATA = os.path.join(ROOT_DIR, 'crates/hc-i18n/src/data.rs')
READ = '2026-10-09'

# tag, CLDR files child first (root left out), the entry the tag inherits
# from (None for a language), the region whose weekData gives the first day.
LOCALES = [
    ('ar-EG', ['ar_EG', 'ar'], 'AR', 'EG'),
    ('en-001', ['en_001', 'en'], 'EN', '001'),
    ('en-GB', ['en_GB', 'en_001', 'en'], 'EN_001', 'GB'),
    ('es-419', ['es_419', 'es'], 'ES', '419'),
    ('mn', ['mn'], None, 'MN'),
    ('shi-Latn', ['shi_Latn'], None, 'MA'),
    ('ur-IN', ['ur_IN', 'ur'], 'UR', 'IN'),
    ('zh-Hant-HK', ['zh_Hant_HK', 'zh_Hant'], 'ZH_HANT', 'HK'),
]
# A language entry's scalars, which no CLDR field states in this crate's
# terms: script, direction, casing, whether month names are capitalised.
LANGUAGE = {
    'mn': ('Cyrl', 'LeftToRight', 'Standard', False),
    'shi-Latn': ('Latn', 'LeftToRight', 'Standard', False),
}
# The languages of the Berber agrarian calendar whose Gregorian months serve
# it too, as `kab` and `zgh` in src/data.rs: the Encyclopédie berbère's
# "Calendrier" gives the agrarian months as the Julian ones under the same
# Latin-derived names (`gast1992`), and Wikipedia's "Berber calendar" prints
# Shilha's, innayr … dujambir, among them.
BERBER = {'shi-Latn'}
# The calendar entries of a language that no CLDR file states, hand-written
# in src/data.rs from the source their comment names, which the language's
# entry carries after CLDR's: the constant, and what it carries.
HAND_WRITTEN = {
    'mn': [('super::MN_MONGOLIAN', "the Mongolian calendar's months, from Gantumur's calendar"),
           ('super::MN_NUMBERED_CALENDARS', "the Buddhist and Minguo calendars' long dates, with the month by its number, from mn.xml")],
}
# The chains CLDR files the hc-i18n entries of src/data.rs follow, for the
# era names those entries already carry, which a regional entry compares
# against its parent's.


def cal(calendar, *rest):
    return path('dates', 'calendars', f'calendar[{calendar}]', *rest)


# --- groups -----------------------------------------------------------------

WIDER = {'narrow': 'abbreviated', 'short': 'abbreviated', 'abbreviated': 'wide', 'wide': None}
GROUPS = {
    'months': ('months', 'monthContext', 'monthWidth', 'month'),
    'days': ('days', 'dayContext', 'dayWidth', 'day'),
    'quarters': ('quarters', 'quarterContext', 'quarterWidth', 'quarter'),
    'dayperiods': ('dayPeriods', 'dayPeriodContext', 'dayPeriodWidth', 'dayPeriod'),
}
DAY_KEYS = ['mon', 'tue', 'wed', 'thu', 'fri', 'sat', 'sun']
log = []


def keys_of(kind, count):
    if kind == 'days':
        return DAY_KEYS
    if kind == 'dayperiods':
        return ['am', 'pm']
    return [str(i) for i in range(1, count + 1)]


def widths_of(kind):
    return ['wide', 'abbreviated', 'short', 'narrow'] if kind == 'days' else \
        ['wide', 'abbreviated', 'narrow']


def table(chain, calendar, kind, count=12):
    """{(context, width): [(value, file)]} for a group."""
    group, context_tag, width_tag, item = GROUPS[kind]
    out = {}
    for context in ('format', 'stand-alone'):
        for width in widths_of(kind):
            out[(context, width)] = [
                resolve(chain, cal(calendar, group, f'{context_tag}[{context}]',
                                   f'{width_tag}[{width}]', (item, {'type': key})))
                for key in keys_of(kind, count)]
    return out


def values(tab):
    return {k: [v for v, _ in row] for k, row in tab.items()}


def own_group(tag, chain, calendar, kind, count=12, language=True):
    """The group's lists by context and width, where the entry carries it,
    else None. A language carries a group its own files state; a regional
    variant one whose resolution differs from its parent's."""
    tab = table(chain, calendar, kind, count)
    if language:
        group, context_tag, width_tag, item = GROUPS[kind]
        if not any(stated(chain, cal(calendar, group, f'{context_tag}[{c}]', f'{width_tag}[{w}]',
                                     (item, {'type': key})))
                   for c in ('format', 'stand-alone') for w in widths_of(kind)
                   for key in keys_of(kind, count)):
            return None
    else:
        if values(tab) == values(table(chain[1:], calendar, kind, count)):
            return None
    wide = tab[('format', 'wide')]
    if kind != 'dayperiods' and not any(f not in (None, 'root') for _, f in wide):
        log.append(f'{tag} {calendar} {kind}: the format wide names are root\'s, left out')
        return None

    def pick(context, width):
        row = tab[(context, width)]
        if context == 'format' and width == 'wide':
            return [v for v, _ in row] if all(v for v, _ in row) else None
        if not any(f not in (None, 'root') for _, f in row):
            return None
        # A width the files state in part, the rest being root's numerals, is
        # not stated: ps.xml's stand-alone narrow months are two letters and
        # root's 3 to 12, which no reader wants for a month. A letter that
        # root gives beside the file's own is the file's, written once because
        # the two agree: cs.xml leaves its Saturday S to root's.
        if any(f in (None, 'root') and v.isdigit() for v, f in row):
            return None
        return [v for v, _ in row] if all(v for v, _ in row) else None

    return ({w: pick('format', w) for w in widths_of(kind)},
            {w: pick('stand-alone', w) for w in widths_of(kind)})


def arr(items):
    return '&[' + ', '.join(lit(x) for x in items) + ']' if items else '&[]'


def reduce_widths(desired, order, parent=None):
    """The widths to write so that the crate's fallback (narrow and short to
    abbreviated to wide, then `parent`) gives `desired`."""
    out = {}

    def effective(width):
        current = width
        while current:
            if current in out:
                return out[current]
            current = WIDER[current]
        return parent(width) if parent else None

    for width in order:
        wanted = desired.get(width)
        if wanted is not None and effective(width) != wanted:
            out[width] = wanted
    return out


def contextual(kind, group):
    fmt, standalone = group
    order = widths_of(kind)
    if fmt.get('wide') is None:
        return None
    written = reduce_widths(fmt, order)

    def format_effective(width):
        current = width
        while current:
            if current in written:
                return written[current]
            current = WIDER[current]

    alone = reduce_widths(standalone, order, parent=format_effective)

    def rust(sets):
        if kind == 'days':
            return (f"weekday_widths({arr(sets.get('wide'))}, {arr(sets.get('abbreviated'))}, "
                    f"{arr(sets.get('short'))}, {arr(sets.get('narrow'))})")
        return (f"widths({arr(sets.get('wide'))}, {arr(sets.get('abbreviated'))}, "
                f"{arr(sets.get('narrow'))})")

    if not alone:
        return f'ContextualNames::same({rust(written)})'
    return f'ContextualNames {{ format: {rust(written)}, standalone: {rust(alone)} }}'


def era_group(tag, chain, calendar, keys, language):
    def one(files, width):
        return [resolve(files, cal(calendar, 'eras', width, ('era', {'type': k}))) for k in keys]

    tab = {w: one(chain, w) for w in ('eraNames', 'eraAbbr', 'eraNarrow')}
    if language:
        if not any(stated(chain, cal(calendar, 'eras', w, ('era', {'type': k})))
                   for w in tab for k in keys):
            return None
    else:
        parent = {w: one(chain[1:], w) for w in tab}
        if values(tab) == values(parent):
            return None
    if not any(f not in (None, 'root') for w in tab for _, f in tab[w]):
        return None

    def full(width):
        row = tab[width]
        return [v for v, _ in row] if all(v for v, _ in row) else None

    wide, abbreviated, narrow = full('eraNames'), full('eraAbbr'), full('eraNarrow')
    if wide is None:
        wide = abbreviated
    if wide is None:
        log.append(f'{tag} {calendar} eras: partial, left out')
        return None
    return reduce_widths({'wide': wide, 'abbreviated': abbreviated, 'narrow': narrow},
                         ['wide', 'abbreviated', 'narrow'])


def era_rust(codes, sets):
    return (f"era_names(&[{', '.join(lit(c) for c in codes)}], {arr(sets.get('wide'))}, "
            f"{arr(sets.get('abbreviated'))}, {arr(sets.get('narrow'))})")


# --- templates --------------------------------------------------------------

def convert(pattern, fields):
    """A CLDR pattern in this crate's placeholders."""
    out, i = '', 0
    while i < len(pattern):
        ch = pattern[i]
        if ch == "'":
            j = pattern.index("'", i + 1)
            out += pattern[i + 1:j] if j > i + 1 else "'"
            i = j + 1
            continue
        if ch.isascii() and ch.isalpha():
            j = i
            while j < len(pattern) and pattern[j] == ch:
                j += 1
            run = pattern[i:j]
            key = 'y' if ch == 'y' else run
            if run in ('M', 'MM') and pattern[j:j + 1] in ('月', '월'):
                # The numbered month with its counter is the abbreviated
                # name in the Chinese, Japanese and Korean files, 9月, 9월,
                # and MM月 is the two-digit number, 09月.
                key, j = run + pattern[j], j + 1
            if key not in fields:
                raise ValueError(f'{pattern!r}: {run} is not carried')
            out += fields[key]
            i = j
            continue
        out += ch
        i += 1
    return out


def templates(chain):
    item = lambda ident: resolve(chain, cal('gregorian', 'dateTimeFormats', 'availableFormats',
                                            ('dateFormatItem', {'id': ident})))[0]
    gy = item('Gy')
    day_item = item('d')
    # The long date format is what a long date is, with its era where the
    # file writes one: Thai's "d MMMM G y". `yMMMMd` is the flexible
    # skeleton's, which differs from it only there.
    whole = resolve(chain, cal('gregorian', 'dateFormats', 'dateFormatLength[long]',
                               'dateFormat', 'pattern'))[0] or item('yMMMMd')
    year = convert(gy, {'G': '{era}', 'y': '{year}'})
    day = convert(day_item, {'d': '{day}', 'dd': '{day}'})
    fields = {'y': '{year}', 'MMMM': '{month}', 'LLLL': '{month}',
              'MMM': '{month:abbreviated}', 'M月': '{month:abbreviated}',
              'M월': '{month:abbreviated}', 'd': '{day}', 'dd': '{day}'}
    if 'G' in whole.replace("'", ''):
        # The era in the date, beside the year's number: the year's own
        # template would write it a second time.
        fields.update({'G': '{era}', 'y': '{year:1}'})
    date = convert(whole, fields)
    # A day or year unit's own suffix is the unit template's, so that a date
    # without a day loses it with it: `d日` and `y年M月d日`.
    if day == '{day}':
        day = ''
    elif day.startswith('{day}'):
        date = date.replace(day, '{day}')
    suffix = year.split('{year}', 1)[1]
    if suffix and not suffix.strip().startswith('{') and '{era}' in year.split('{year}')[0]:
        date = date.replace('{year}' + suffix, '{year}')
    return (gy, day_item, whole), year, day, date


# `M` is the month's number in a CLDR pattern, and in another calendar than
# the Gregorian its abbreviated name is not the number with its counter, so
# `M月` and `M월` are the number here, 4月, as ICU writes a Hijri date in
# Japanese.
CALENDAR_FIELDS = {'G': '{era}', 'GGGGG': '{era}', 'y': '{year:1}', 'MMMM': '{month}',
                   'LLLL': '{month}', 'MMM': '{month:abbreviated}', 'M月': '{month:1}月',
                   'M월': '{month:1}월', 'MM月': '{month:2}月', 'MM월': '{month:2}월',
                   'd': '{day}', 'dd': '{day}', 'U': '{sexagenary}',
                   'r': '{extra:related-gregorian-year}'}


# The calendars whose years may carry a leap month, which a numbered month
# in a date would not tell from the month before it.
LEAP_MONTH_CALENDARS = {'hebrew', 'chinese', 'dangi'}


# The calendars that count their years in one era, whose long date, where its
# pattern has no era, writes the year's number alone: Thai's Buddhist
# "d MMMM y" is 2569 and no BE, though the year by itself is พ.ศ. 2569.
SINGLE_ERA = {'buddhist', 'roc', 'islamic', 'hebrew', 'coptic', 'ethiopic', 'persian', 'indian'}


def calendar_templates(chain, calendar, gregorian):
    """The templates a calendar's own resolved formats give, as Rust fields
    (year, day, date) that differ from the entry's Gregorian ones, else
    None: the `Gy` item, the `d` item and the long date format, which
    `root.xml` aliases to the `generic` calendar's for most calendars. A
    date that writes its era beside its year, "d. MMMM y G", takes the year
    as a number, `{year:1}`, and the era where the format puts it; one
    that writes the year alone takes the rendered year, `{year}`, whose
    template carries the era."""
    item = lambda ident: resolve(chain, cal(calendar, 'dateTimeFormats', 'availableFormats',
                                            ('dateFormatItem', {'id': ident})))[0]
    gy = item('Gy') or 'G y'
    day_item = item('d') or 'd'
    long = resolve(chain, cal(calendar, 'dateFormats', 'dateFormatLength[long]', 'dateFormat',
                              'pattern'))[0]
    if long is None or 'E' in long.replace("'", ''):
        return None
    try:
        year = convert(gy, {'G': '{era}', 'y': '{year}', 'U': '{sexagenary}',
                            'r': '{extra:related-gregorian-year}'})
        day = convert(day_item, {'d': '{day}', 'dd': '{day}'})
        fields = dict(CALENDAR_FIELDS)
        if 'G' not in long and 'U' not in long:
            fields['y'] = '{year:1}' if calendar in SINGLE_ERA else '{year}'
        if calendar in LEAP_MONTH_CALENDARS:
            # CLDR numbers the Hebrew months Tishri 1 to Elul 13 with Adar I
            # as 6, and a Chinese leap month by its own pattern; the crate's
            # month number is the ordinal, 5 for Shevat and for Adar I, so a
            # numbered month would write two first-of-months alike. The
            # name tells them apart: シェバト and アダル I, 五月 and 六月.
            fields['M月'] = fields['M월'] = fields['MM月'] = fields['MM월'] = '{month}'
        date = convert(long, fields)
    except ValueError as error:
        log.append(f'{chain[0]} {calendar}: templates not read, {error}')
        return None
    if day == '{day}':
        day = ''
        # The day's point or counter is the day template's, and a calendar
        # whose own `d` item is plain takes the entry's, so the date does
        # not write it a second time: de's "d. MMMM y G" is "{day} ..." with
        # the "{day}." of its Gregorian `d` item behind it, not "30..".
        entry_day = gregorian.get('day', '')
        if entry_day.startswith('{day}') and entry_day != '{day}':
            date = date.replace(entry_day, '{day}')
    elif day.startswith('{day}'):
        date = date.replace(day, '{day}')
    if '{year}' in date:
        suffix = year.split('{year}', 1)[1]
        if suffix and not suffix.strip().startswith('{') and '{era}' in year.split('{year}')[0]:
            date = date.replace('{year}' + suffix, '{year}')
    out = {k: v for k, v in (('year', year), ('day', day), ('date', date))
           if v != gregorian.get(k, '')}
    return out or None


def templates_rust(fields):
    body = ', '.join(f'{k}: {lit(v)}' for k, v in fields.items())
    return f'DateTemplates {{ {body}, ..DateTemplates::NONE }}'


def chinese_date(tag, calendar, long, lines):
    """The templates of a Chinese-family calendar's date, from its long date
    pattern: `zh.xml`'s "rU年MMMd" is `CHINESE_TEMPLATES`, r the related
    Gregorian year and U the cyclic year; another order of the two is that
    with its own year template."""
    if long == 'rU年MMMd':
        return 'CHINESE_TEMPLATES'
    head, tail = long.split('MMM')
    assert tail == 'd', long
    year = head.replace('r', '{extra:related-gregorian-year}').replace('U', '{sexagenary}')
    name = f'{const_name(tag)}_{calendar.upper()}_TEMPLATES'
    lines += [f'/// The {calendar} date, the long pattern "{long}".',
              f'const {name}: DateTemplates = DateTemplates {{',
              f'    year: {lit(year)},',
              '    ..CHINESE_TEMPLATES',
              '};', '']
    return name


# --- calendar names ---------------------------------------------------------

CALENDAR_TYPES = [('buddhist', 'buddhist'), ('chinese', 'chinese'), ('coptic', 'coptic'),
                  ('dangi', 'dangi'), ('ethiopic', 'ethiopic'), ('gregorian', 'gregory'),
                  ('hebrew', 'hebrew'), ('indian', 'indian'), ('islamic-civil', 'islamic-civil'),
                  ('islamic-tbla', 'islamic-tbla'), ('islamic-umalqura', 'islamic-umalqura'),
                  ('islamic-rgsa', 'islamic-rgsa'), ('iso8601', 'iso8601'),
                  ('japanese', 'japanese'), ('persian', 'persian'), ('roc', 'roc')]


def calendar_names(chain, language):
    out = []
    for cldr_type, registry in CALENDAR_TYPES:
        at = path('localeDisplayNames', 'types', ('type', {'key': 'calendar', 'type': cldr_type}))
        value, name = resolve(chain, at)
        if value is None or name == 'root':
            continue
        if language:
            if name not in chain:
                continue
        elif resolve(chain[1:], at)[0] == value:
            continue
        out.append((registry, value))
        if registry == 'iso8601':
            out.append(('iso8601-week', f'{value} (W)'))
        if registry == 'persian':
            out.append(('persian-arithmetic', f'{value} (2820)'))
    return out


# --- locale display names ---------------------------------------------------

def display_name(chain, tag):
    """The tag's name in a locale: its `languages` entry for the whole tag,
    else the language's with the script and region by the locale's
    `localeDisplayPattern`."""
    parts = tag.split('-')
    language = parts[0]
    script = next((p for p in parts[1:] if len(p) == 4), None)
    region = next((p for p in parts[1:] if len(p) in (2, 3)), None)
    names = lambda kind, key: resolve(chain, path('localeDisplayNames', kind,
                                                  (kind[:-1] if kind != 'territories'
                                                   else 'territory', {'type': key})))[0]
    whole = names('languages', tag.replace('-', '_'))
    if whole:
        return whole
    base = names('languages', language)
    if base is None:
        return None
    extras = []
    if script:
        extras.append(names('scripts', script))
    if region:
        extras.append(names('territories', region))
    extras = [extra for extra in extras if extra]
    if not extras:
        return base
    pattern = resolve(chain, path('localeDisplayNames', 'localeDisplayPattern',
                                  'localePattern'))[0]
    separator = resolve(chain, path('localeDisplayNames', 'localeDisplayPattern',
                                    'localeSeparator'))[0]
    joined = extras[0]
    for extra in extras[1:]:
        joined = separator.replace('{0}', joined).replace('{1}', extra)
    return pattern.replace('{0}', base).replace('{1}', joined)


# --- supplemental -----------------------------------------------------------

_supplemental = []


def supplemental():
    if not _supplemental:
        _supplemental.append(xml('supplemental/supplementalData.xml'))
    return _supplemental[0]


def parent_locales():
    out = []
    for group in supplemental().iter('parentLocales'):
        if group.get('component'):
            continue
        for row in group.findall('parentLocale'):
            parent = row.get('parent')
            for child in row.get('locales').split():
                out.append((child, parent))
    return sorted(out)


def cldr_chain(name):
    """A file's inheritance chain, root left out: the file, then the parent
    `parentLocales` names, else its truncated name, keeping the files the
    release has."""
    parents = dict(parent_locales())
    chain = []
    while name and name != 'root':
        if exists(f'main/{name}.xml'):
            chain.append(name)
        name = parents.get(name) or (name.rsplit('_', 1)[0] if '_' in name else None)
    return chain


def default_numbering():
    """(tag, numbering system): every carried entry's default system, and
    every regional file of a carried language whose system differs from
    that of the entry its tag falls back to, sorted by tag."""
    from cldr_xml import ENTRIES
    at = path('numbers', 'defaultNumberingSystem')
    entries = {chain[0]: tag for tag, chain, _ in ENTRIES}
    rows = {tag: resolve(chain, at)[0] or 'latn' for tag, chain, _ in ENTRIES}
    languages = {chain[0].split('_')[0] for _, chain, _ in ENTRIES}
    candidates = set()
    for territory in supplemental().iter('territory'):
        for population in territory.findall('languagePopulation'):
            language = population.get('type')
            if language.split('_')[0] in languages:
                candidates.add(f"{language}_{territory.get('type')}")
    for name in sorted(candidates - set(entries)):
        if not exists(f'main/{name}.xml'):
            continue
        chain = cldr_chain(name)
        entry = next((entries[f] for f in chain if f in entries), None)
        if entry is None:
            continue
        value = resolve(chain, at)[0] or 'latn'
        if value != rows[entry]:
            rows[name.replace('_', '-')] = value
    return sorted(rows.items())


def first_day(region):
    """`weekData/firstDay` for a region, the world's (`001`) where CLDR lists
    the region under no day."""
    days = {'mon': 'Monday', 'tue': 'Tuesday', 'wed': 'Wednesday', 'thu': 'Thursday',
            'fri': 'Friday', 'sat': 'Saturday', 'sun': 'Sunday'}
    rows = [(row.get('day'), row.get('territories').split())
            for row in supplemental().iter('firstDay') if not row.get('alt')]
    for wanted in (region, '001'):
        for day, territories in rows:
            if wanted in territories:
                return days[day]
    raise ValueError(region)


def min_days(region):
    """`weekData/minDays` for a region, the world's (`001`) where CLDR lists
    the region under no count."""
    rows = [(int(row.get('count')), row.get('territories').split())
            for row in supplemental().iter('minDays') if not row.get('alt')]
    for wanted in (region, '001'):
        for count, territories in rows:
            if wanted in territories:
                return count
    raise ValueError(region)


def likely_region(tag):
    """The region `likelySubtags.xml` gives a tag's language, with its script
    where the tag has one, else the language alone."""
    rows = {row.get('from'): row.get('to') for row in xml('supplemental/likelySubtags.xml')
            .iter('likelySubtag')}
    parts = tag.split('-')
    explicit = [part for part in parts[1:] if len(part) == 2 and part.isupper()]
    if explicit:
        return explicit[0]
    for key in ('_'.join(parts[:2]), parts[0]):
        if key in rows:
            return rows[key].split('_')[-1]
    raise ValueError(tag)


# The region the week data of an entry CLDR has no file for comes from: the
# language's own country, `weekData/firstDay` and `minDays` giving the world's
# for it, as the entry's comment says.
HAND_REGIONS = {'und': '001', 'ban': 'ID', 'cop': 'EG', 'mid': 'IQ', 'nah': 'MX', 'zap': 'MX',
                'yua': 'MX', 'mix': 'MX', 'rif': 'MA', 'aeb-Latn': 'TN', 'ayl-Latn': 'LY'}


# How the entry's comment names a CLDR calendar.
PROSE = {'buddhist': 'Buddhist', 'roc': 'Minguo', 'japanese': 'Japanese', 'generic': 'generic',
         'islamic': 'Hijri', 'hebrew': 'Hebrew',
         'coptic': 'Coptic', 'ethiopic': 'Ethiopic', 'persian': 'Persian',
         'indian': 'Indian national', 'chinese': 'Chinese', 'dangi': 'Dangi'}


# --- the calendars a file states --------------------------------------------

def cldr_calendars(tag, chain, language, skip, gregorian_fields):
    """(Rust calendar entries, what they carry in prose, lines to write
    before them): every CLDR calendar the entry's files state months or eras
    for, less those in `skip` (the CLDR keys a hand-written entry already
    serves), each with its own templates where they differ from the
    Gregorian ones."""
    calendars, carried, lines_before = [], [], []

    def with_templates(text, calendar):
        fields = calendar_templates(chain, calendar, gregorian_fields)
        return f'{text}.with_templates({templates_rust(fields)})' if fields else text

    def month_entry(calendar, calendars_rs, count, codes, keys, era_calendars=None):
        if calendar in skip:
            return
        group = own_group(tag, chain, calendar, 'months', count, language)
        cycle = None
        if group is not None and calendar == 'hebrew':
            cycle = hebrew(group)
        elif group is not None:
            wide = group[0]['wide']
            if wide and all(re.fullmatch(r'\d+月?|M\d+', x) for x in wide):
                log.append(f'{tag} {calendar}: numbered months, not carried')
            else:
                cycle = contextual('months', group)
        eras = era_group(tag, chain, calendar, keys, language) if keys else None
        if cycle is None and eras is None:
            return
        cycles = f'&[month_cycle({cycle})]' if cycle else '&[]'
        era = era_rust(codes, eras) if eras and not era_calendars else 'EraNames::EMPTY'
        text = f'calendar_entry({calendars_rs}, {cycles}, {era})'
        if calendar == 'hebrew' and cycle:
            text += (f'.with_leap_names(LeapMonthNames {{ intercalary: &[(5, {lit(HEBREW[0])})], '
                     f'in_leap_years: &[(6, {lit(HEBREW[1])})], leap_day: None }})')
        if cycle or era != 'EraNames::EMPTY':
            calendars.append(with_templates(text, calendar))
            carried.append(f'the {PROSE[calendar]} ' + ('months' if cycle else '')
                           + (' and eras' if cycle and era != 'EraNames::EMPTY' else
                              'eras' if era != 'EraNames::EMPTY' else ''))
        if eras and era_calendars:
            calendars.append(with_templates(
                f'calendar_entry({era_calendars}, &[], {era_rust(codes, eras)})', calendar))
            carried.append(f'the {PROSE[calendar]} eras')

    HEBREW = [None, None]

    def hebrew(group):
        fmt, standalone = group
        cut = lambda xs: None if xs is None else xs[:5] + xs[6:]
        leap = resolve(chain, cal('hebrew', 'months', 'monthContext[format]', 'monthWidth[wide]',
                                  ('month', {'type': '7', 'yeartype': 'leap'})))[0]
        HEBREW[0], HEBREW[1] = fmt['wide'][5], leap
        return contextual('months', ({w: cut(v) for w, v in fmt.items()},
                                     {w: cut(v) for w, v in standalone.items()}))

    # The Gregorian months, quarters and eras, for a generated entry.
    if 'gregorian' not in skip:
        months = own_group(tag, chain, 'gregorian', 'months', 12, language)
        months_rs = contextual('months', months) if months else None
        quarters = own_group(tag, chain, 'gregorian', 'quarters', 4, language)
        quarters_rs = contextual('quarters', quarters) if quarters else None
        eras = era_group(tag, chain, 'gregorian', ['0', '1'], language)
        if months_rs or quarters_rs or eras:
            cycles = f'&[month_cycle({months_rs})]' if months_rs else '&[]'
            era = (f"gregorian_eras({arr(eras.get('wide'))}, {arr(eras.get('abbreviated'))}, "
                   f"{arr(eras.get('narrow'))})") if eras else 'EraNames::EMPTY'
            calendars.append(f"gregorian({cycles}, {era}, {quarters_rs or 'ContextualNames::EMPTY'})")
            if tag in BERBER and months_rs:
                calendars.append(f'calendar_entry(&[CalendarId("berber")], '
                                 f'&[month_cycle({months_rs})], EraNames::EMPTY)')
            carried += [what for what, present in (('the Gregorian months', months_rs),
                                                   ('the quarters', quarters_rs),
                                                   ('the Gregorian eras', eras)) if present]

    def era_entry(calendar, calendars_rs, codes, keys):
        if calendar in skip:
            return
        found = era_group(tag, chain, calendar, keys, language)
        if found:
            calendars.append(with_templates(
                f'calendar_entry({calendars_rs}, &[], {era_rust(codes, found)})', calendar))
            carried.append(f'the {PROSE[calendar]} era' + ('s' if len(codes) > 1 else ''))

    era_entry('buddhist', 'BUDDHIST_CALENDARS', ['be'], ['0'])
    era_entry('roc', '&[CalendarId("roc")]', ['broc', 'roc'], ['0', '1'])
    month_entry('islamic', 'ISLAMIC_CALENDARS', 12, ['ah'], ['0'])
    month_entry('hebrew', 'HEBREW_CALENDARS', 13, ['am'], ['0'])
    month_entry('coptic', 'COPTIC_CALENDARS', 13, ['am'], ['1'])
    month_entry('ethiopic', 'ETHIOPIC_CALENDARS', 13, ['aa', 'am'], ['0', '1'])
    month_entry('persian', 'PERSIAN_CALENDARS', 12, ['ap'], ['0'],
                era_calendars='SOLAR_HIJRI_CALENDARS')
    month_entry('indian', '&[CalendarId("indian")]', 12, ['saka'], ['0'])
    for calendar, calendars_rs in (('chinese', 'CHINESE_AND_VIETNAMESE_CALENDARS'),
                                   ('dangi', 'DANGI_CALENDARS')):
        if calendar in skip:
            continue
        group = own_group(tag, chain, calendar, 'months', 12, language)
        if group is None:
            continue
        wide = group[0]['wide']
        if wide and all(re.fullmatch(r'\d+月?|M\d+', x) for x in wide):
            log.append(f'{tag} {calendar}: numbered months, not carried')
            continue
        leap = resolve(chain, cal(calendar, 'monthPatterns', 'monthPatternContext[format]',
                                  'monthPatternWidth[wide]', ('monthPattern', {'type': 'leap'})))[0]
        if not leap.endswith('{0}'):
            log.append(f'{tag} {calendar}: the leap month pattern {leap!r} is a suffix, not carried')
            continue
        long = resolve(chain, cal(calendar, 'dateFormats', 'dateFormatLength[long]', 'dateFormat',
                                  'pattern'))[0]
        try:
            chinese_templates = chinese_date(tag, calendar, long, lines_before)
        except (AssertionError, ValueError):
            fields = calendar_templates(chain, calendar, gregorian_fields)
            if fields is None:
                log.append(f'{tag} {calendar}: the long date {long!r} is not read, not carried')
                continue
            chinese_templates = templates_rust(fields)
        calendars.append(f'lunisolar({calendars_rs}, &[month_cycle({contextual("months", group)})], '
                         f'{lit(leap[:-3])}).with_templates({chinese_templates})')
        carried.append(f'the {PROSE[calendar]} months')
    # The date formats of the calendars the entry's Gregorian months serve,
    # which count their years in an era of their own: each of the Buddhist,
    # Minguo and Japanese calendars by its own formats, where the file states
    # them, and every other, by the formats CLDR's `generic` calendar gives
    # a calendar it has none for (`GENERIC_DATE_CALENDARS`), the era where
    # the file puts it. A field the entry's hand-written templates state
    # stays theirs, which the lookup takes first.
    for calendar, calendars_rs in (('buddhist', 'BUDDHIST_CALENDARS'),
                                   ('roc', '&[CalendarId("roc")]'),
                                   ('islamic', 'ISLAMIC_CALENDARS'),
                                   ('hebrew', 'HEBREW_CALENDARS'),
                                   ('coptic', 'COPTIC_CALENDARS'),
                                   ('ethiopic', 'ETHIOPIC_CALENDARS'),
                                   ('persian', 'PERSIAN_CALENDARS'),
                                   ('indian', '&[CalendarId("indian")]'),
                                   ('japanese', 'JAPANESE_CALENDARS'),
                                   ('generic', 'GENERIC_DATE_CALENDARS')):
        if calendar in SINGLE_ERA and calendar not in skip:
            continue
        if re.search('[年년]', gregorian_fields.get('year', '')):
            # The Chinese, Japanese and Korean entries write a year with
            # its counter, whose first year of an era is 元年, and a month
            # by the abbreviated name that carries its counter: their
            # hand-written templates are the ones for every such calendar.
            continue
        fields = calendar_templates(chain, calendar, gregorian_fields)
        if fields:
            calendars.append(f'calendar_entry({calendars_rs}, &[], EraNames::EMPTY)'
                             f'.with_templates({templates_rust(fields)})')
            carried.append(f'the {PROSE[calendar]} date formats')
    return calendars, carried, lines_before


# --- one entry --------------------------------------------------------------

def const_name(tag):
    return tag.replace('-', '_').upper()


def entry(tag, chain, parent, region):
    language = parent is None
    name = const_name(tag)
    carried, dump = [], []
    gregorian_templates = templates(chain)[1:]
    gregorian_fields = dict(zip(('year', 'day', 'date'), gregorian_templates))
    calendars, carried, lines_before = cldr_calendars(tag, chain, language, set(),
                                                      gregorian_fields)
    hand = []
    for constant, what in HAND_WRITTEN.get(tag, []):
        calendars.append(constant)
        hand.append(what)

    weekdays = own_group(tag, chain, 'gregorian', 'days', 7, language)
    weekdays_rs = contextual('days', weekdays) if weekdays else None
    periods = own_group(tag, chain, 'gregorian', 'dayperiods', 2, language)
    periods_rs = contextual('dayperiods', periods) if periods else None
    carried = ([x for x, p in (('the weekdays', weekdays_rs), ('the day periods', periods_rs)) if p]
               + carried)

    patterns, year, day, date = templates(chain)
    names = calendar_names(chain, language)
    numbering = resolve(chain, path('numbers', 'defaultNumberingSystem'))[0]
    english = display_name(['en'], tag)
    # A file that names neither its language nor its script (`shi_Latn.xml`)
    # is named as its language's own file names it (`shi.xml`, in Tifinagh),
    # else in English.
    native = (display_name(chain, tag) or display_name([chain[0].split('_')[0]], tag)
              or english)

    files = ', '.join(f'`{f}.xml`' for f in chain)
    lines = [f'// --- {tag}: {english} ' + '-' * max(3, 70 - len(tag) - len(english)), '//']
    listed = ', '.join(carried[:-1]) + ' and ' + carried[-1] if len(carried) > 1 else \
        ''.join(carried)
    text = (f'CLDR 48 {files}. '
            + ('It carries ' + listed + '.' if carried else
               'Its files state no group of names apart from its parent\'s.'))
    if hand:
        text += (' Hand-written in `src/data.rs` beside them: ' + ', '.join(hand)
                 + '.')
    if not language:
        text += (f' Every other group is the parent entry\'s, `{parent_tag(parent)}`, as CLDR\'s '
                 'inheritance gives it.')
    text += (f' Templates from `Gy` "{patterns[0]}", `d` "{patterns[1]}" and the whole date '
             f'"{patterns[2]}"; digits `{numbering}`.')
    lines += ['// ' + x for x in textwrap.wrap(text, 76, break_long_words=False,
                                                 break_on_hyphens=False)]
    lines.append('')
    lines += lines_before
    lines.append(f'const {name}_TEMPLATES: DateTemplates = DateTemplates {{')
    lines.append(f'    year: {lit(year)},')
    if day:
        lines.append(f'    day: {lit(day)},')
    lines.append(f'    date: {lit(date)},')
    lines.append('    ..DateTemplates::NONE')
    lines.append('};')
    lines.append('')
    if names:
        lines.append(f'const {name}_CALENDAR_NAMES: &[CalendarDisplayName] = &[')
        for registry, value in names:
            lines.append(f'    CalendarDisplayName::new({lit(registry)}, {lit(value)}),')
        lines.append('];')
        lines.append('')
    lines.append(f'const {name}_CALENDARS: &[CalendarNames] = &[')
    lines += [f'    {c},' for c in calendars]
    lines.append('];')
    lines.append('')
    lines.append(f'/// The `{tag}` entry.')
    lines.append(f'pub(super) const {name}: LocaleData = LocaleData {{')
    lines.append(f'    tag: {lit(tag)},')
    lines.append(f'    sources: "Unicode CLDR 48, {", ".join("common/main/" + f + ".xml" for f in chain)} '
                 f'(cldr48-regional-locales), read {READ}: '
                 + ('every group of names the file states' if language else
                    'every group of names the files resolve apart from the parent') + '",')
    lines.append(f'    english_name: {lit(english)},')
    lines.append(f'    native_name: {lit(native)},')
    if language:
        script, direction, casing, capital = LANGUAGE[tag]
        lines.append(f'    script: {lit(script)},')
        lines.append(f'    direction: Direction::{direction},')
        lines.append(f'    casing: CasingStyle::{casing},')
        lines.append(f'    capitalises_month_names: {"true" if capital else "false"},')
    else:
        for field in ('script', 'direction', 'casing', 'capitalises_month_names'):
            lines.append(f'    {field}: {parent_path(parent)}.{field},')
    lines.append(f'    templates: {name}_TEMPLATES,')
    lines.append(f'    calendar_names: {name + "_CALENDAR_NAMES" if names else "&[]"},')
    lines.append(f'    numbering: {lit(numbering)},')
    lines.append(f'    first_day_of_week: Weekday::{first_day(region)},')
    lines.append(f'    min_days: {min_days(region)},')
    lines.append(f'    weekdays: {weekdays_rs or "ContextualNames::EMPTY"},')
    lines.append(f'    day_periods: {periods_rs or "ContextualNames::EMPTY"},')
    lines.append('    cycle: SexagenaryNames::EMPTY,')
    lines.append(f'    calendars: {name}_CALENDARS,')
    lines.append('    cldr_calendars: &[],')
    lines.append('};')
    lines.append('')
    dump.append(f'{tag}\tcarried\t{"; ".join(carried)}')
    dump.append(f'{tag}\ttemplates\t{year}\t{day}\t{date}')
    dump.append(f'{tag}\tnames\t{names}')
    dump.append(f'{tag}\tnumbering\t{numbering}\t{first_day(region)}\t{english}\t{native}')
    return lines, dump


def parent_path(parent):
    """The parent entry as the generated module reaches it."""
    generated = {const_name(tag) for tag, _, _, _ in LOCALES}
    return parent if parent in generated else f'super::{parent}'


def parent_tag(parent):
    for tag, _, _, _ in LOCALES:
        if const_name(tag) == parent:
            return tag
    return parent.lower().replace('_', '-').replace('-hant', '-Hant')


HEADER_TEMPLATE = '''\
//! The locale entries read out of Unicode CLDR 48, and its parent locales,
//! generated.
//!
//! **Do not edit.** `scripts/locales-cldr.py` writes this file from CLDR 48
//! (`release-48`, `common/main/<file>.xml` over `root.xml`, and
//! `common/supplemental/supplementalData.xml`; `cldr48-regional-locales`),
//! as the script's documentation says: a language entry carries every
//! group of names its file states, and a regional entry the groups its
//! files resolve to other values than its parent does, so that everything
//! else inherits its parent's, as CLDR's inheritance has it. `super` lists
//! the entries in `LOCALES`.

use hc_calendar::{CalendarId, Weekday};

{imports}use crate::casing::CasingStyle;
use crate::direction::Direction;
{names_imports}
'''


def generate():
    out, dump = [], []
    for tag, chain, parent, region in LOCALES:
        lines, values_ = entry(tag, chain, parent, region)
        out += lines
        dump += values_
    parents = parent_locales()
    out.append('/// CLDR 48\'s `parentLocales` (`supplementalData.xml`, the default component):')
    out.append('/// each child with the parent that CLDR\'s inheritance takes it to in place of')
    out.append('/// its truncated tag, in BCP 47 spelling, `und` for root, sorted by child so')
    out.append('/// that a lookup can search it.')
    out.append('pub static PARENT_LOCALES: &[(&str, &str)] = &[')
    bcp = lambda t: 'und' if t == 'root' else t.replace('_', '-')
    for child, parent in sorted((bcp(c), bcp(p)) for c, p in parents):
        out.append(f'    ({lit(child)}, {lit(parent)}),')
    out.append('];')
    dump += [f'parent\t{c}\t{p}' for c, p in parents]
    out.append('')
    out.append('/// CLDR 48\'s `numbers/defaultNumberingSystem`, resolved: each carried')
    out.append('/// entry\'s, and each regional file\'s whose system is not that of the entry')
    out.append('/// its tag falls back to (`ar-SA`\'s `arab` beside `ar`\'s `latn`), sorted by tag.')
    out.append('pub static DEFAULT_NUMBERING: &[(&str, &str)] = &[')
    for tag, system in default_numbering():
        out.append(f'    ({lit(tag)}, {lit(system)}),')
        dump.append(f'default-numbering\t{tag}\t{system}')
    out.append('];')
    out.append('')
    out.append('/// CLDR 48\'s `weekData/minDays` (`supplementalData.xml`): the regions whose')
    out.append('/// first week of a year or month needs more than the world\'s (`001`) one day,')
    out.append('/// with the count, sorted by region so that a lookup can search it.')
    out.append('pub static REGION_MIN_DAYS: &[(&str, u8)] = &[')
    regions = sorted((territory, int(row.get('count')))
                     for row in supplemental().iter('minDays') if not row.get('alt')
                     for territory in row.get('territories').split()
                     if int(row.get('count')) != min_days('001') and not territory.isdigit())
    for territory, count in regions:
        out.append(f'    ({lit(territory)}, {count}),')
        dump.append(f'min-days\t{territory}\t{count}')
    out.append('];')
    out.append('')
    out.append('/// Each carried locale\'s other numbering systems, CLDR 48\'s')
    out.append('/// `numbers/otherNumberingSystems`, resolved: (tag, `native`, `traditional`),')
    out.append('/// empty where the locale\'s files state none.')
    out.append('pub static OTHER_NUMBERING: &[(&str, &str, &str)] = &[')
    from cldr_xml import ENTRIES
    for tag, chain, _ in ENTRIES:
        native = resolve(chain, path('numbers', 'otherNumberingSystems', 'native'))[0] or ''
        traditional = resolve(chain, path('numbers', 'otherNumberingSystems',
                                          'traditional'))[0] or ''
        out.append(f'    ({lit(tag)}, {lit(native)}, {lit(traditional)}),')
        dump.append(f'numbering\t{tag}\t{native}\t{traditional}')
    out.append('];')
    body = '\n'.join(out).rstrip('\n') + '\n'
    used = lambda names: sorted((n for n in names if re.search(rf'\b{n}\b', body)),
                                key=lambda n: (not n.isupper(), n))
    helpers = used(['BUDDHIST_CALENDARS', 'CHINESE_AND_VIETNAMESE_CALENDARS', 'CHINESE_TEMPLATES',
                    'COPTIC_CALENDARS', 'DANGI_CALENDARS', 'ETHIOPIC_CALENDARS',
                    'GENERIC_DATE_CALENDARS', 'HEBREW_CALENDARS', 'ISLAMIC_CALENDARS',
                    'JAPANESE_CALENDARS', 'PERSIAN_CALENDARS',
                    'SOLAR_HIJRI_CALENDARS', 'calendar_entry', 'era_names', 'gregorian',
                    'gregorian_eras', 'lunisolar', 'month_cycle', 'weekday_widths', 'widths'])
    types = used(['CalendarDisplayName', 'CalendarNames', 'ContextualNames', 'DateTemplates',
                  'EraNames', 'LeapMonthNames', 'LocaleData', 'SexagenaryNames'])
    header = HEADER_TEMPLATE.replace('{imports}', 'use super::{' + ', '.join(helpers) + '};\n')
    header = header.replace('{names_imports}', 'use crate::names::{' + ', '.join(types) + '};\n')
    text = header + body
    return text, dump


# --- the hand-written entries' CLDR groups ------------------------------------

# The entries of src/data.rs whose Gregorian months, weekdays, quarters and
# eras are CLDR's, with the files each reads child first. They are written
# into data.rs in place, over the `gregorian(...)` call and the `weekdays:`
# field of the entry, so that the hand-written entries' CLDR vocabulary is
# regenerated and checked beside the generated entries'; everything else in
# data.rs is hand-written. `pt-PT` is a regional file whose entry carries
# its groups whole.
HAND_ENTRIES = {tag: chain for tag, chain, regional in ENTRIES if not regional}
HAND_ENTRIES['pt-PT'] = ['pt_PT', 'pt']


def balanced(text, start):
    """The index just past the bracket at `text[start]`'s partner."""
    depth, at, quoted = 0, start, False
    while True:
        char = text[at]
        if quoted:
            if char == '\\':
                at += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in '([{':
            depth += 1
        elif char in ')]}':
            depth -= 1
            if depth == 0:
                return at + 1
        at += 1


def split_arguments(body):
    arguments, depth, start, quoted, at = [], 0, 0, False, 0
    while at < len(body):
        char = body[at]
        if quoted:
            if char == '\\':
                at += 1
            elif char == '"':
                quoted = False
        elif char == '"':
            quoted = True
        elif char in '([{':
            depth += 1
        elif char in ')]}':
            depth -= 1
        elif char == ',' and depth == 0:
            arguments.append(body[start:at])
            start = at + 1
        at += 1
    if body[start:].strip():
        arguments.append(body[start:])
    return [argument.strip() for argument in arguments]


def hand_entries(source):
    """src/data.rs with each hand-written CLDR entry's `gregorian(...)` call
    and `weekdays:` field written from its files. A group a file does not
    state, and an entry whose call is empty (`pt-PT`'s, whose parent's
    names are right), is left as written."""
    items = [(m.start(), m.group(1), m.group(2).strip()) for m in
             re.finditer(r'^(?:pub )?(?:const|static) (\w+): ([^=]+) = ', source, re.M)]
    owner = {}
    for number, (start, name, kind) in enumerate(items):
        end = items[number + 1][0] if number + 1 < len(items) else len(source)
        block = source[start:end]
        found = re.search(r'tag: "([^"]+)"', block)
        if kind == 'LocaleData' and found:
            owner[name] = found.group(1)
            calendars = re.search(r'calendars: &?([A-Z_0-9]+),', block)
            if calendars:
                owner[calendars.group(1)] = found.group(1)
    edits = []
    for call in re.finditer(r'(?<![\w_])gregorian\(', source):
        if source[call.start() - 3:call.start()] == 'fn ':
            continue
        opening = call.end() - 1
        closing = balanced(source, opening)
        item = max((i for i in items if i[0] <= call.start()), key=lambda i: i[0])
        tag = owner.get(item[1])
        if tag not in HAND_ENTRIES:
            continue
        chain = HAND_ENTRIES[tag]
        body = '\n'.join(line for line in source[opening + 1:closing - 1].split('\n')
                         if not line.strip().startswith('//'))
        arguments = split_arguments(body)
        if arguments[0] == '&[]':
            continue
        assert arguments[0].startswith('&[month_cycle('), (tag, arguments[0][:40])
        months = own_group(tag, chain, 'gregorian', 'months', 12)
        quarters = own_group(tag, chain, 'gregorian', 'quarters', 4)
        eras = era_group(tag, chain, 'gregorian', ['0', '1'], True)
        arguments = [
            f'&[month_cycle({contextual("months", months)})]' if months else arguments[0],
            (f"gregorian_eras({arr(eras.get('wide'))}, {arr(eras.get('abbreviated'))}, "
             f"{arr(eras.get('narrow'))})") if eras else arguments[1],
            contextual('quarters', quarters) if quarters else arguments[2]]
        edits.append((opening + 1, closing - 1, ',\n'.join(arguments) + ','))
    for number, (start, name, kind) in enumerate(items):
        tag = owner.get(name)
        if kind != 'LocaleData' or tag not in HAND_ENTRIES:
            continue
        end = items[number + 1][0] if number + 1 < len(items) else len(source)
        field = re.search(r'\n    weekdays: ', source[start:end])
        group = own_group(tag, HAND_ENTRIES[tag], 'gregorian', 'days', 7)
        if not field or not group:
            continue
        begin = start + field.end()
        at, depth = begin, 0
        while not (source[at] == ',' and depth == 0):
            depth += source[at] in '([{'
            depth -= source[at] in ')]}'
            at += 1
        edits.append((begin, at, contextual('days', group)))
    # The week data: `min_days:` after `first_day_of_week:`, from the region
    # the entry's language is likeliest in, whose `firstDay` the entry must
    # state.
    for number, (start, name, kind) in enumerate(items):
        tag = owner.get(name)
        if kind != 'LocaleData' or tag is None:
            continue
        end = items[number + 1][0] if number + 1 < len(items) else len(source)
        found = re.search(r'\n    first_day_of_week: Weekday::(\w+),(\n    min_days: \d+,)?',
                          source[start:end])
        if not found:
            continue
        region = HAND_REGIONS.get(tag) or likely_region(tag)
        assert found.group(1) == first_day(region) or tag in HAND_REGIONS, (
            f'{tag}: first day {found.group(1)}, `weekData/firstDay` for {region} gives '
            f'{first_day(region)}')
        edits.append((start + found.start(), start + found.end(),
                      f'\n    first_day_of_week: Weekday::{found.group(1)},'
                      f'\n    min_days: {min_days(region)},'))
    # The other calendars the files state, into cldr48_calendars.rs, and the
    # `cldr_calendars:` field that names each entry's constant.
    generated = []
    for number, (start, name, kind) in enumerate(items):
        tag = owner.get(name)
        if kind != 'LocaleData' or tag is None or name == 'ROOT':
            continue
        end = items[number + 1][0] if number + 1 < len(items) else len(source)
        block = source[start:end]
        field = re.search(r'\n    cldr_calendars: [^,]+,', block)
        if not field:
            continue
        value = '&[]'
        if tag in HAND_ENTRIES:
            chain = HAND_ENTRIES[tag]
            served = served_calendars(source, items, block)
            try:
                gregorian_fields = dict(zip(('year', 'day', 'date'), templates(chain)[1:]))
            except ValueError:
                # An entry whose Gregorian patterns no template matches yet
                # (`ta`, `bn`): every calendar template is written whole.
                gregorian_fields = {}
            calendars, carried, before = cldr_calendars(tag, chain, tag != 'pt-PT', served,
                                                        gregorian_fields)
            if calendars:
                constant = f'{const_name(tag)}_CLDR'
                value = f'cldr48_calendars::{constant}'
                files = ', '.join(f'`{f}.xml`' for f in chain)
                listed = (', '.join(carried[:-1]) + ' and ' + carried[-1] if len(carried) > 1
                          else ''.join(carried))
                text = (f'The other calendars of `{tag}`, CLDR 48 {files}: {listed}; the '
                        'hand-written entry serves ' + (', '.join(sorted(served)) or 'no other')
                        + ' already.')
                generated += ['/// ' + x for x in textwrap.wrap(text, 76, break_long_words=False,
                                                                 break_on_hyphens=False)]
                generated += before
                generated.append(f'pub(super) const {constant}: &[CalendarNames] = &[')
                generated += [f'    {c},' for c in calendars]
                generated.append('];')
                generated.append('')
        edits.append((start + field.start(), start + field.end(),
                      f'\n    cldr_calendars: {value},'))
    for begin, end, text in sorted(edits, reverse=True):
        source = source[:begin] + text + source[end:]
    return source, generated


# The CLDR calendar each constant or identifier of a hand-written entry's
# `calendars` list serves, so that the generated entries leave it alone.
SERVED = {'ISLAMIC_CALENDARS': ['islamic'], 'HEBREW_CALENDARS': ['hebrew'],
          'COPTIC_CALENDARS': ['coptic'], 'ETHIOPIC_CALENDARS': ['ethiopic'],
          'PERSIAN_CALENDARS': ['persian'], 'SOLAR_HIJRI_CALENDARS': ['persian'],
          'BUDDHIST_CALENDARS': ['buddhist'], 'JAPANESE_CALENDARS': ['japanese'],
          'CHINESE_FAMILY_CALENDARS': ['chinese', 'dangi'],
          'CHINESE_AND_VIETNAMESE_CALENDARS': ['chinese'],
          'CHINESE_REGNAL_CALENDARS': ['chinese'], 'DANGI_CALENDARS': ['dangi']}
SERVED_IDS = {'indian': 'indian', 'roc': 'roc', 'hebrew': 'hebrew', 'coptic': 'coptic',
              'ethiopic': 'ethiopic', 'buddhist': 'buddhist', 'persian': 'persian',
              'persian-afghan': 'persian', 'chinese': 'chinese', 'dangi': 'dangi'}


def served_calendars(source, items, block):
    """The CLDR calendars a hand-written entry's `calendars` list already
    serves, by the constants and identifiers its items name; the Gregorian
    and the Japanese eras are every entry's own."""
    served = {'gregorian', 'japanese'}
    found = re.search(r'\n    calendars: &?([A-Z_0-9]+),', block)
    text = block
    if found:
        for number, (start, name, _) in enumerate(items):
            if name == found.group(1):
                end = items[number + 1][0] if number + 1 < len(items) else len(source)
                text += source[start:end]
    for constant, calendars in SERVED.items():
        if re.search(rf'\b{constant}\b', text):
            served.update(calendars)
    for ident in re.findall(r'CalendarId\("([a-z0-9-]+)"\)', text):
        if ident in SERVED_IDS:
            served.add(SERVED_IDS[ident])
        elif ident.startswith('islamic'):
            served.add('islamic')
    return served


def main():
    text, dump = generate()
    if '--dump' in sys.argv:
        print('\n'.join(dump + log))
        return
    write_or_check(OUTPUT, rustfmt(text), 'scripts/locales-cldr.py')
    with open(DATA, encoding='utf-8') as handle:
        data, generated = hand_entries(handle.read())
    write_or_check(DATA, rustfmt(data), 'scripts/locales-cldr.py')
    body = '\n'.join(generated).rstrip('\n') + '\n'
    used = lambda names: sorted((n for n in names if re.search(rf'\b{n}\b', body)),
                                key=lambda n: (not n.isupper(), n))
    helpers = used(['BUDDHIST_CALENDARS', 'CHINESE_AND_VIETNAMESE_CALENDARS', 'CHINESE_TEMPLATES',
                    'COPTIC_CALENDARS', 'DANGI_CALENDARS', 'ETHIOPIC_CALENDARS',
                    'GENERIC_DATE_CALENDARS', 'HEBREW_CALENDARS', 'ISLAMIC_CALENDARS',
                    'JAPANESE_CALENDARS', 'PERSIAN_CALENDARS',
                    'SOLAR_HIJRI_CALENDARS', 'calendar_entry', 'era_names', 'lunisolar',
                    'month_cycle', 'widths'])
    types = used(['CalendarNames', 'ContextualNames', 'DateTemplates', 'EraNames',
                  'LeapMonthNames'])
    header = CALENDARS_HEADER.replace('{read}', READ)
    header = header.replace('{imports}', 'use super::{' + ', '.join(helpers) + '};\n')
    header = header.replace('{names_imports}', 'use crate::names::{' + ', '.join(types) + '};\n')
    if 'CalendarId(' in body:
        header = header.replace('{calendar_id}', 'use hc_calendar::CalendarId;\n\n')
    else:
        header = header.replace('{calendar_id}', '')
    write_or_check(CALENDARS, rustfmt(header + body), 'scripts/locales-cldr.py')


CALENDARS_HEADER = '''\
//! The other calendars of the hand-written locale entries, read out of
//! Unicode CLDR 48, generated.
//!
//! **Do not edit.** `scripts/locales-cldr.py` writes this file from CLDR 48
//! (`release-48`, `common/main/<file>.xml` over `root.xml`, read {read};
//! `cldr48-regional-locales`), as the script's documentation says: for each
//! hand-written entry of `super` that follows a CLDR file, every calendar
//! its files state months or eras for that the entry does not serve itself,
//! with the templates the calendar's own date formats give where they
//! differ from the entry's Gregorian ones. Each entry's `cldr_calendars`
//! field names its constant, and `LocaleData::entries_for` reads it after
//! `calendars`.

{calendar_id}{imports}{names_imports}
'''


if __name__ == '__main__':
    main()
