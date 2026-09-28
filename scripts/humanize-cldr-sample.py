#!/usr/bin/env python3
"""Regenerate crates/hc-humanize/tests/data/cldr48_resolved.tsv.

The sample `tests/cldr48_resolved.rs` compares every hc-humanize locale
with: the past, future and duration patterns of the hour and the day in
each plural category of the language, and the words for the day before
yesterday to the day after tomorrow (the hour's for -1 to 1), in the long,
short and narrow styles. They are read from the `cldr-json` 48.0.0
packages, which CLDR's own tools resolve from the XML — every parent,
alias and inheritance marker already followed — so that the test is a
check on scripts/humanize-cldr.py's resolution that does not share its
code:

    python3 scripts/humanize-cldr-sample.py

`cldr-dates-full` `main/<locale>/dateFields.json`, `cldr-units-full`
`main/<locale>/units.json` and `cldr-core` `supplemental/plurals.json`,
read over HTTP from cdn.jsdelivr.net into memory; nothing else is saved. A
category the file has no key for is the style's `other`, as a reader of
those files takes it; a word it has no key for is none.
"""
import json
import os
import urllib.request

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUTPUT = os.path.join(ROOT_DIR, 'crates/hc-humanize/tests/data/cldr48_resolved.tsv')
CDN = 'https://cdn.jsdelivr.net/npm/'

# Each carried tag and the cldr-json locale that holds it.
TAGS = [
    ('ar', 'ar'), ('cs', 'cs'), ('cy', 'cy'), ('de', 'de'), ('en', 'en'), ('es', 'es'),
    ('fil', 'fil'), ('fr', 'fr'), ('ha', 'ha'), ('hi', 'hi'), ('id', 'id'), ('it', 'it'),
    ('ja', 'ja'), ('ko', 'ko'), ('mr', 'mr'), ('nl', 'nl'), ('pa-Guru', 'pa'), ('pcm', 'pcm'),
    ('pl', 'pl'), ('pt', 'pt'), ('pt-PT', 'pt-PT'), ('ru', 'ru'), ('sw', 'sw'), ('te', 'te'),
    ('th', 'th'), ('tr', 'tr'), ('ur', 'ur'), ('vi', 'vi'), ('yue-Hans', 'yue-Hans'),
    ('yue-Hant', 'yue'), ('zh', 'zh'), ('zh-Hant', 'zh-Hant'),
]
UNITS = ['hour', 'day']
STYLES = [('long', ''), ('short', '-short'), ('narrow', '-narrow')]
ORDER = ['zero', 'one', 'two', 'few', 'many', 'other']


def get(path):
    for attempt in range(4):
        try:
            with urllib.request.urlopen(CDN + path, timeout=60) as response:
                return json.loads(response.read())
        except OSError:
            if attempt == 3:
                raise
    raise AssertionError


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
        fields = get(f'cldr-dates-full@48.0.0/main/{name}/dateFields.json')['main'][name][
            'dates']['fields']
        units = get(f'cldr-units-full@48.0.0/main/{name}/units.json')['main'][name]['units']
        cats = categories(name)
        for style, suffix in STYLES:
            for unit in UNITS:
                field = fields[unit + suffix]
                for kind, table, key in [
                    ('past', field['relativeTime-type-past'], 'relativeTimePattern'),
                    ('future', field['relativeTime-type-future'], 'relativeTimePattern'),
                    ('count', units[style]['duration-' + unit], 'unitPattern'),
                ]:
                    cells = [f'{c}=' + table.get(f'{key}-count-{c}', table[f'{key}-count-other'])
                             for c in cats]
                    out.append('\t'.join([tag, style, unit, kind] + cells))
                offsets = [-2, -1, 0, 1, 2] if unit == 'day' else [-1, 0, 1]
                cells = [f'{o}=' + field.get(f'relative-type-{o}', '') for o in offsets]
                out.append('\t'.join([tag, style, unit, 'relative'] + cells))
    assert not any('\n' in line for line in out)
    with open(OUTPUT, 'w', encoding='utf-8') as handle:
        handle.write('\n'.join(out) + '\n')


if __name__ == '__main__':
    main()
