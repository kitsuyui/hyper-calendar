#!/usr/bin/env python3
"""Regenerate crates/hc-humanize/tests/data/cldr48_resolved.tsv.

`tests/cldr48_resolved.rs` compares every value hc-humanize takes from
CLDR, in every one of its locales, with CLDR 48's own resolution of it:

* the past, future and duration patterns of all eight units in each
  plural category of the language, in the long, short and narrow styles;
* the words for two units before to two after (*the day before
  yesterday* to *the day after tomorrow*, *last year*) of all eight units,
  in the three styles;
* the standard, unit and narrow unit list patterns (`2`, `start`,
  `middle` and `end`);
* the decimal separator of the digits hc-i18n writes the locale's numbers
  in: CLDR's default numbering system, `latn` for `ar` and `arab` for
  `ar-EG`;
* the long `relative` date-time pattern (UTS #35 Part 4, "Element
  dateTimeFormat"), written in the crate's order: CLDR's `{1}` is the date
  and `{0}` the time, the crate's `{0}` is the day phrase and `{1}` the
  time, and CLDR's quoted literals are unquoted.

They are read from the `cldr-json` 48.0.0 packages, which CLDR's own tools
resolve from the XML — every parent, alias and inheritance marker already
followed — so that the test is a check on scripts/humanize-cldr.py's
resolution that does not share its code:

    python3 scripts/humanize-cldr-sample.py

`cldr-dates-full` `main/<locale>/dateFields.json` and `ca-gregorian.json`,
`cldr-units-full` `main/<locale>/units.json`, `cldr-misc-full`
`main/<locale>/listPatterns.json`, `cldr-numbers-full`
`main/<locale>/numbers.json` and `cldr-core` `supplemental/plurals.json`,
read over HTTP from cdn.jsdelivr.net into memory; nothing else is saved. A
category the file has no key for is the style's `other`, as a reader of
those files takes it; a word it has no key for is none.

Each line is a tag and then `key=value` cells, the keys those of
crates/hc-humanize/src/data/cldr48_overrides.tsv.
"""
import http.client
import json
import os
import urllib.request

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUTPUT = os.path.join(ROOT_DIR, 'crates/hc-humanize/tests/data/cldr48_resolved.tsv')
CDN = 'https://cdn.jsdelivr.net/npm/'

# Each carried tag and the cldr-json locale that holds it.
TAGS = [
    ('ar', 'ar'), ('ar-EG', 'ar-EG'), ('cs', 'cs'), ('cy', 'cy'), ('de', 'de'), ('en', 'en'),
    ('en-001', 'en-001'), ('en-GB', 'en-GB'), ('es', 'es'), ('es-419', 'es-419'),
    ('fil', 'fil'), ('fr', 'fr'), ('ha', 'ha'), ('hi', 'hi'), ('id', 'id'), ('it', 'it'),
    ('ja', 'ja'), ('ko', 'ko'), ('mn', 'mn'), ('mr', 'mr'), ('nl', 'nl'), ('pa-Guru', 'pa'),
    ('pcm', 'pcm'), ('pl', 'pl'), ('pt', 'pt'), ('pt-PT', 'pt-PT'), ('ru', 'ru'), ('sw', 'sw'),
    ('te', 'te'), ('th', 'th'), ('tr', 'tr'), ('ur', 'ur'), ('ur-IN', 'ur-IN'), ('vi', 'vi'),
    ('yue-Hans', 'yue-Hans'), ('yue-Hant', 'yue'), ('zh', 'zh'), ('zh-Hant', 'zh-Hant'),
    ('zh-Hant-HK', 'zh-Hant-HK'),
]
# The locales whose numbers hc-i18n writes in CLDR's `native` system rather
# than the default one: none, since hc-i18n writes each locale's default;
# the test holds hc-i18n to it.
NATIVE_DIGITS = set()
UNITS = ['second', 'minute', 'hour', 'day', 'week', 'month', 'quarter', 'year']
STYLES = [('long', ''), ('short', '-short'), ('narrow', '-narrow')]
ORDER = ['zero', 'one', 'two', 'few', 'many', 'other']
OFFSETS = [-2, -1, 0, 1, 2]
LISTS = [('standard', 'standard'), ('unit', 'unit'), ('narrow', 'unit-narrow')]
LIST_PARTS = ['2', 'start', 'middle', 'end']


def get(path):
    for attempt in range(4):
        try:
            with urllib.request.urlopen(CDN + path, timeout=60) as response:
                return json.loads(response.read())
        except (OSError, http.client.HTTPException):
            if attempt == 3:
                raise
    raise AssertionError


def main_of(package, name, file):
    return get(f'{package}@48.0.0/main/{name}/{file}.json')['main'][name]


def unquote(pattern):
    """A date-time pattern's text with its quoting removed: `'at'` is `at`
    and `''` an apostrophe."""
    out, quoted, i = [], False, 0
    while i < len(pattern):
        char = pattern[i]
        if char == "'":
            if pattern[i + 1:i + 2] == "'":
                out.append("'")
                i += 1
            else:
                quoted = not quoted
        else:
            out.append(char)
        i += 1
    return ''.join(out)


def crate_order(pattern):
    return pattern.replace('{0}', '\x00').replace('{1}', '{0}').replace('\x00', '{1}')


def main():
    rules = get('cldr-core@48.0.0/supplemental/plurals.json')['supplemental'][
        'plurals-type-cardinal']

    def categories(locale):
        while locale not in rules:
            locale = locale.rsplit('-', 1)[0]
        stated = {key.rsplit('-', 1)[1] for key in rules[locale]}
        return [c for c in ORDER if c in stated]

    out = []
    for tag, name in TAGS:
        fields = main_of('cldr-dates-full', name, 'dateFields')['dates']['fields']
        units = main_of('cldr-units-full', name, 'units')['units']
        lists = main_of('cldr-misc-full', name, 'listPatterns')['listPatterns']
        numbers = main_of('cldr-numbers-full', name, 'numbers')['numbers']
        gregorian = main_of('cldr-dates-full', name, 'ca-gregorian')['dates']['calendars'][
            'gregorian']
        cats = categories(name)
        for style, suffix in STYLES:
            for unit in UNITS:
                field = fields[unit + suffix]
                for kind, table, key in [
                    ('past', field['relativeTime-type-past'], 'relativeTimePattern'),
                    ('future', field['relativeTime-type-future'], 'relativeTimePattern'),
                    ('count', units[style]['duration-' + unit], 'unitPattern'),
                ]:
                    cells = [f'{style}.{unit}.{kind}.{c}='
                             + table.get(f'{key}-count-{c}', table[f'{key}-count-other'])
                             for c in cats]
                    out.append('\t'.join([tag] + cells))
                cells = [f'{style}.{unit}.relative.{o}=' + field.get(f'relative-type-{o}', '')
                         for o in OFFSETS]
                out.append('\t'.join([tag] + cells))
        for kind, cldr in LISTS:
            parts = lists[f'listPattern-type-{cldr}']
            out.append('\t'.join([tag] + [f'list.{kind}.{p}={parts[p]}' for p in LIST_PARTS]))
        if name in NATIVE_DIGITS:
            system = numbers['otherNumberingSystems']['native']
        else:
            system = numbers['defaultNumberingSystem']
        decimal = numbers[f'symbols-numberSystem-{system}']['decimal']
        relative = gregorian['dateTimeFormats-relative']['standard']['long']
        out.append('\t'.join([tag, f'decimal={decimal}',
                              f'at={crate_order(unquote(relative))}']))
    assert not any('\n' in line for line in out)
    with open(OUTPUT, 'w', encoding='utf-8') as handle:
        handle.write('\n'.join(out) + '\n')


if __name__ == '__main__':
    main()
