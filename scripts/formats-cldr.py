#!/usr/bin/env python3
"""Regenerate crates/hc-i18n/src/formats/cldr48.rs from Unicode CLDR 48.

The standard date, time and date-time formats of every carried locale, for
every calendar CLDR gives them, and the few `availableFormats` items the
workspace reads:

    python3 scripts/formats-cldr.py            # rewrite the file
    python3 scripts/formats-cldr.py --check    # exit 1 if it is stale
    python3 scripts/formats-cldr.py --dump     # print every resolved value, a TSV

The files, read through scripts/cldr_xml.py (the `release-48` tag, over HTTP
into memory, nothing saved): each carried entry's `common/main/<file>.xml`
over `root.xml`, `dates/calendars/calendar[@type]` for the fourteen calendar
types below, resolved as UTS #35 Part 1 resolves them, `root.xml`'s aliases
included (the Persian calendar's `dateFormats` are the `generic` calendar's,
its `timeFormats` the Gregorian's).

Each row is one locale and one calendar: eighteen fields separated by `|`, in
the order `FIELDS` gives — the `dateFormats` full, long, medium and short,
the `timeFormats`, the standard `dateTimeFormats`, then the items `hms`,
`Hms`, `Gy`, `d`, `yMMMMd` and `yMMMd`. A field is written only where the
lookup in `hc_i18n::formats` would not find the same value further on: the
Gregorian row of a language is whole; its `generic` row states what differs
from the Gregorian row, and another calendar's row what differs from the
`generic` row (`iso8601`'s from the Gregorian); a regional entry's rows state
what differs from the parent entry's resolved value. The lookup reads the
calendar asked for, then `generic`, then `gregorian`, and for each the
locale's chain and then `und`, root; the generator runs that lookup over
every value and adds any field it would otherwise get wrong, so that the
table and CLDR's resolution agree for every carried entry.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cldr_xml import ENTRIES, lit, path, resolve, rustfmt, write_or_check  # noqa: E402

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUTPUT = os.path.join(ROOT_DIR, 'crates/hc-i18n/src/formats/cldr48.rs')
READ = '2026-10-04'
LENGTHS = ['full', 'long', 'medium', 'short']
ITEMS = ['hms', 'Hms', 'Gy', 'd', 'yMMMMd', 'yMMMd']
FIELDS = ([f'date-{L}' for L in LENGTHS] + [f'time-{L}' for L in LENGTHS]
          + [f'datetime-{L}' for L in LENGTHS] + ITEMS)
CALENDARS = ['gregorian', 'generic', 'buddhist', 'chinese', 'coptic', 'dangi', 'ethiopic', 'hebrew',
             'indian', 'islamic', 'iso8601', 'japanese', 'persian', 'roc']


def cal(calendar, *rest):
    return path('dates', 'calendars', f'calendar[{calendar}]', *rest)


def resolved(chain, calendar):
    """Every field's resolved value for one chain and calendar, '' for none."""
    out = []
    for length in LENGTHS:
        out.append(resolve(chain, cal(calendar, 'dateFormats', f'dateFormatLength[{length}]',
                                      'dateFormat', 'pattern'))[0] or '')
    for length in LENGTHS:
        out.append(resolve(chain, cal(calendar, 'timeFormats', f'timeFormatLength[{length}]',
                                      'timeFormat', 'pattern'))[0] or '')
    for length in LENGTHS:
        out.append(resolve(chain, cal(calendar, 'dateTimeFormats',
                                      f'dateTimeFormatLength[{length}]', 'dateTimeFormat',
                                      'pattern'))[0] or '')
    for item in ITEMS:
        out.append(resolve(chain, cal(calendar, 'dateTimeFormats', 'availableFormats',
                                      ('dateFormatItem', {'id': item})))[0] or '')
    for value in out:
        assert '|' not in value and '\n' not in value, value
    return out


def base_of(calendar):
    if calendar == 'gregorian':
        return None
    return 'gregorian' if calendar in ('iso8601', 'generic') else 'generic'


def search_order(calendar):
    order = [calendar]
    while base_of(order[-1]):
        order.append(base_of(order[-1]))
    return order


def generate():
    file_tag = {chain[0]: tag for tag, chain, _ in ENTRIES}
    entries = [('und', [], None)] + [(tag, chain, chain[1:] or None) for tag, chain, _ in ENTRIES]
    # The chain of tags the Rust lookup walks for each entry, `und` last.
    tag_chain = {}
    for tag, chain, _ in entries:
        tags = [tag] + [file_tag[f] for f in chain[1:] if f in file_tag]
        if 'und' not in tags:
            tags.append('und')
        tag_chain[tag] = tags
    values = {}
    for tag, chain, _ in entries:
        for calendar in CALENDARS:
            values[(tag, calendar)] = resolved(chain, calendar)
    rows = {}
    for tag, chain, parent_chain in entries:
        for calendar in CALENDARS:
            own = values[(tag, calendar)]
            if parent_chain is not None and file_tag.get(parent_chain[0]):
                against = values[(file_tag[parent_chain[0]], calendar)]
            elif base_of(calendar):
                against = values[(tag, base_of(calendar))]
            else:
                against = [None] * len(FIELDS)
            rows[(tag, calendar)] = [v if v != a else '' for v, a in zip(own, against)]

    def lookup(tag, calendar, index):
        for step in search_order(calendar):
            for candidate in tag_chain[tag]:
                row = rows.get((candidate, step))
                if row and row[index]:
                    return row[index]
        return ''

    # Add whatever the lookup would get wrong, until it agrees everywhere. A
    # field a calendar resolves to nothing (`generic` has no `yMMMMd`) is
    # left to the base's value, which the lookup gives it.
    for _ in range(10):
        changed = False
        for tag, _, _ in entries:
            for calendar in CALENDARS:
                for index, value in enumerate(values[(tag, calendar)]):
                    if value and lookup(tag, calendar, index) != value:
                        rows[(tag, calendar)][index] = value
                        changed = True
        if not changed:
            break
    else:
        raise RuntimeError('the lookup and the resolution do not converge')
    out, dump = [], []
    out.append('/// Each carried locale\'s and root\'s (`und`) formats, one row per calendar,')
    out.append('/// sorted by tag and calendar: the eighteen fields of `super::Field` in')
    out.append('/// order, `|` between them, empty where the lookup finds the value further')
    out.append('/// on.')
    out.append('pub(super) static FORMATS: &[(&str, &str, &str)] = &[')
    for (tag, calendar), row in sorted(rows.items()):
        if not any(row):
            continue
        out.append(f'    ({lit(tag)}, {lit(calendar)}, {lit("|".join(row).rstrip("|"))}),')
    out.append('];')
    for (tag, calendar), row in sorted(values.items()):
        for field, value in zip(FIELDS, row):
            dump.append(f'{tag}\t{calendar}\t{field}\t{value}')
    header = f'''//! The date, time and date-time formats of every carried locale, generated.
//!
//! **Do not edit.** `scripts/formats-cldr.py` writes this file from Unicode
//! CLDR 48 (`release-48`, read {READ}, `cldr48-date-formats`): each carried
//! locale's `common/main/<file>.xml` over `root.xml`, the `dateFormats`,
//! `timeFormats`, standard `dateTimeFormats` and the `availableFormats`
//! items `hms`, `Hms`, `Gy`, `d`, `yMMMMd` and `yMMMd` of the calendars
//! `{' '.join(CALENDARS)}`, resolved through `root.xml`'s aliases, as the
//! script's documentation says.

'''
    return header + '\n'.join(out) + '\n', dump


def main():
    text, dump = generate()
    if '--dump' in sys.argv:
        print('\n'.join(dump))
        return
    write_or_check(OUTPUT, rustfmt(text), 'scripts/formats-cldr.py')


if __name__ == '__main__':
    main()
