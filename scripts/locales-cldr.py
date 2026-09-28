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
  its Latin narrow weekdays) is left empty and answered by the next wider
  one, and a group whose format wide names are root's is left out.
* A regional variant (`en-001`, `en-GB`, `es-419`, `zh-Hant-HK`, `ur-IN`,
  `ar-EG`): the groups its files resolve to other values than its CLDR
  parent does — the parent `parentLocales` names, else the truncated tag —
  each whole, with the same widths; every other group is left empty, so
  that the lookup inherits the parent entry's, as CLDR's inheritance does.
  A regional file that writes the inheritance marker `↑↑↑` or repeats its
  parent's value states nothing of its own here.

Every entry also has its date templates, which do not inherit between
entries, read from the resolved Gregorian `Gy` item (the year with its
era), `d` item (the day) and `yMMMMd` item, else the long date format (the
whole date); its calendar names, where they differ from the parent's; its
default numbering system; and its first day of the week, the region's
`weekData/firstDay` (the language's likely region for `mn`, Mongolia). A
regional entry takes its script, direction, casing and capitalisation from
its parent entry.

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
from cldr_xml import lit, path, resolve, rustfmt, stated, write_or_check, xml  # noqa: E402

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUTPUT = os.path.join(ROOT_DIR, 'crates/hc-i18n/src/data/cldr48_locales.rs')
READ = '2026-09-29'

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
            if run in ('M', 'MM') and pattern[j:j + 1] == '月':
                # The numbered month with its counter is the abbreviated
                # name in the Chinese and Japanese files, 9月.
                key, j = 'M月', j + 1
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
    whole = item('yMMMMd') or resolve(chain, cal('gregorian', 'dateFormats',
                                                  'dateFormatLength[long]', 'dateFormat',
                                                  'pattern'))[0]
    year = convert(gy, {'G': '{era}', 'y': '{year}'})
    day = convert(day_item, {'d': '{day}', 'dd': '{day}'})
    date = convert(whole, {'y': '{year}', 'MMMM': '{month}', 'LLLL': '{month}',
                           'MMM': '{month:abbreviated}', 'M月': '{month:abbreviated}',
                           'd': '{day}', 'dd': '{day}'})
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

def supplemental():
    return xml('supplemental/supplementalData.xml')


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


# How the entry's comment names a CLDR calendar.
PROSE = {'buddhist': 'Buddhist', 'roc': 'Minguo', 'islamic': 'Hijri', 'hebrew': 'Hebrew',
         'coptic': 'Coptic', 'ethiopic': 'Ethiopic', 'persian': 'Persian',
         'indian': 'Indian national', 'chinese': 'Chinese', 'dangi': 'Dangi'}


# --- one entry --------------------------------------------------------------

def const_name(tag):
    return tag.replace('-', '_').upper()


def entry(tag, chain, parent, region):
    language = parent is None
    name = const_name(tag)
    carried, dump = [], []
    calendars = []

    def month_entry(calendar, calendars_rs, count, codes, keys, era_calendars=None):
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
            calendars.append(text)
            carried.append(f'the {PROSE[calendar]} ' + ('months' if cycle else '')
                           + (' and eras' if cycle and era != 'EraNames::EMPTY' else
                              'eras' if era != 'EraNames::EMPTY' else ''))
        if eras and era_calendars:
            calendars.append(f'calendar_entry({era_calendars}, &[], {era_rust(codes, eras)})')
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

    # The Gregorian months, quarters and eras.
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
            calendars.append(f'calendar_entry(&[CalendarId("berber")], &[month_cycle({months_rs})], '
                             'EraNames::EMPTY)')
        carried += [what for what, present in (('the Gregorian months', months_rs),
                                               ('the quarters', quarters_rs),
                                               ('the Gregorian eras', eras)) if present]

    def era_entry(calendar, calendars_rs, codes, keys):
        found = era_group(tag, chain, calendar, keys, language)
        if found:
            calendars.append(f'calendar_entry({calendars_rs}, &[], {era_rust(codes, found)})')
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
    lines_before = []
    for calendar, calendars_rs in (('chinese', 'CHINESE_AND_VIETNAMESE_CALENDARS'),
                                   ('dangi', 'DANGI_CALENDARS')):
        group = own_group(tag, chain, calendar, 'months', 12, language)
        if group is None:
            continue
        if language:
            log.append(f'{tag} {calendar}: a language entry\'s Chinese months are not read here')
            continue
        leap = resolve(chain, cal(calendar, 'monthPatterns', 'monthPatternContext[format]',
                                  'monthPatternWidth[wide]', ('monthPattern', {'type': 'leap'})))[0]
        assert leap.endswith('{0}'), leap
        long = resolve(chain, cal(calendar, 'dateFormats', 'dateFormatLength[long]', 'dateFormat',
                                  'pattern'))[0]
        chinese_templates = chinese_date(tag, calendar, long, lines_before)
        calendars.append(f'lunisolar({calendars_rs}, &[month_cycle({contextual("months", group)})], '
                         f'{lit(leap[:-3])}).with_templates({chinese_templates})')
        carried.append(f'the {PROSE[calendar]} months')

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
    lines.append(f'    weekdays: {weekdays_rs or "ContextualNames::EMPTY"},')
    lines.append(f'    day_periods: {periods_rs or "ContextualNames::EMPTY"},')
    lines.append('    cycle: SexagenaryNames::EMPTY,')
    lines.append(f'    calendars: {name}_CALENDARS,')
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
                    'HEBREW_CALENDARS', 'ISLAMIC_CALENDARS', 'PERSIAN_CALENDARS',
                    'SOLAR_HIJRI_CALENDARS', 'calendar_entry', 'era_names', 'gregorian',
                    'gregorian_eras', 'lunisolar', 'month_cycle', 'weekday_widths', 'widths'])
    types = used(['CalendarDisplayName', 'CalendarNames', 'ContextualNames', 'DateTemplates',
                  'EraNames', 'LeapMonthNames', 'LocaleData', 'SexagenaryNames'])
    header = HEADER_TEMPLATE.replace('{imports}', 'use super::{' + ', '.join(helpers) + '};\n')
    header = header.replace('{names_imports}', 'use crate::names::{' + ', '.join(types) + '};\n')
    text = header + body
    return text, dump


def main():
    text, dump = generate()
    if '--dump' in sys.argv:
        print('\n'.join(dump + log))
        return
    write_or_check(OUTPUT, rustfmt(text), 'scripts/locales-cldr.py')


if __name__ == '__main__':
    main()
