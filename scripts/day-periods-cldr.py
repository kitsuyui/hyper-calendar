#!/usr/bin/env python3
"""Regenerate crates/hc-i18n/src/day_periods/cldr48.rs from Unicode CLDR 48.

The day periods of the CLDR pattern fields `b` (am, pm, noon, midnight) and
`B` (the flexible periods, *in the morning*, *at night*):

    python3 scripts/day-periods-cldr.py            # rewrite the file
    python3 scripts/day-periods-cldr.py --check    # exit 1 if it is stale
    python3 scripts/day-periods-cldr.py --dump     # print every value, a TSV

The files, read through scripts/cldr_xml.py (the `release-48` tag, over HTTP
into memory, nothing saved):

* `common/supplemental/dayPeriods.xml`, the format rule set (the one with
  no `type`, which UTS #35 Part 4 uses "in conjunction with times"): for
  each rule set of a carried language, the language's own and those the
  file keys by its script or region (`hi_Latn`, `es_CO`), its periods and
  the times they cover, `at` for the fixed midnight and noon, `from` and
  `before` for the others;
* `common/main/<file>.xml`, `calendar type="gregorian"`, the format
  context's names of the periods other than am and pm (which `hc-i18n`'s
  entries already carry), in the abbreviated, wide and narrow widths,
  resolved through `root.xml`'s aliases: every name a language's file
  states at a release level, and every one a regional entry's files
  resolve apart from its parent's.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cldr_xml import (ENTRIES, NO_VALUE, carried, lit, path, rustfmt,  # noqa: E402
                      write_or_check, xml)

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUTPUT = os.path.join(ROOT_DIR, 'crates/hc-i18n/src/day_periods/cldr48.rs')
READ = '2026-09-29'
KINDS = ['midnight', 'noon', 'morning1', 'morning2', 'afternoon1', 'afternoon2', 'evening1',
         'evening2', 'night1', 'night2']
RUST = {'midnight': 'Midnight', 'noon': 'Noon', 'morning1': 'Morning1', 'morning2': 'Morning2',
        'afternoon1': 'Afternoon1', 'afternoon2': 'Afternoon2', 'evening1': 'Evening1',
        'evening2': 'Evening2', 'night1': 'Night1', 'night2': 'Night2'}
WIDTHS = ['abbreviated', 'wide', 'narrow']


def minutes(clock):
    hours, mins = clock.split(':')
    return int(hours) * 60 + int(mins)


def rules():
    root = xml('supplemental/dayPeriods.xml')
    # Every rule set of a carried language: the language's own, and those
    # keyed by its script or region (`hi_Latn`, `es_CO`), which
    # `hc_i18n::day_periods` finds by truncating a tag.
    languages = {tag.split('-')[0] for tag, _, _ in ENTRIES}
    out = {}
    for ruleset in root.findall('dayPeriodRuleSet'):
        if ruleset.get('type'):
            continue
        for group in ruleset.findall('dayPeriodRules'):
            for locale in group.get('locales').split():
                tag = locale.replace('_', '-')
                if tag.split('-')[0] not in languages:
                    continue
                rows = []
                for rule in group.findall('dayPeriodRule'):
                    kind = rule.get('type')
                    if kind in ('am', 'pm'):
                        continue
                    if rule.get('at'):
                        start = end = minutes(rule.get('at'))
                    else:
                        start, end = minutes(rule.get('from')), minutes(rule.get('before'))
                    rows.append((kind, start, end))
                out[tag] = rows
    return sorted(out.items())


def names(chain, regional):
    lines = []
    for kind in KINDS:
        cells = []
        for width in WIDTHS:
            at = path('dates', 'calendars', 'calendar[gregorian]', 'dayPeriods',
                      'dayPeriodContext[format]', f'dayPeriodWidth[{width}]', f'dayPeriod[{kind}]')
            value = carried(chain, regional, at)
            # No carried file writes the empty override over a day period.
            assert value is not NO_VALUE, (chain, kind, width)
            value = value or ''
            assert '|' not in value
            cells.append(value)
        lines.append('|'.join(cells).rstrip('|'))
    return lines


def generate():
    out, dump = [], []
    out.append('/// Each carried language\'s format rule set: (period, from, before), in')
    out.append('/// minutes after midnight, `from == before` for a fixed period\'s `at`.')
    out.append('pub(super) static RULES: &[(&str, Rules)] = &[')
    for tag, rows in rules():
        cells = ', '.join(f'(FlexibleDayPeriod::{RUST[k]}, {a}, {b})' for k, a, b in rows)
        out.append(f'    ({lit(tag)}, &[{cells}]),')
        for k, a, b in rows:
            dump.append(f'{tag}\trule\t{k}\t{a}\t{b}')
    out.append('];')
    out.append('')
    out.append('/// Each carried locale\'s names: a line per period, in the order of')
    out.append('/// `FlexibleDayPeriod`, its abbreviated, wide and narrow names separated')
    out.append('/// by `|`, empty where the locale states none.')
    out.append('pub(super) static NAMES: &[(&str, &str)] = &[')
    for tag, chain, regional in ENTRIES:
        lines = names(chain, regional)
        if not any(lines):
            continue
        text = lit(''.join(line + chr(10) for line in lines)).replace(chr(10), '\\n')
        out.append(f'    ({lit(tag)}, {text}),')
        for kind, line in zip(KINDS, lines):
            if line:
                dump.append(f'{tag}\tname\t{kind}\t{line}')
    out.append('];')
    header = f'''//! The day periods of every carried language, generated.
//!
//! **Do not edit.** `scripts/day-periods-cldr.py` writes this file from
//! Unicode CLDR 48 (`release-48`, read {READ}, `cldr48-day-periods`):
//! `supplemental/dayPeriods.xml`'s format rule set and each carried
//! locale's `common/main/<file>.xml` Gregorian format day periods, as the
//! script's documentation says.

use super::{{FlexibleDayPeriod, Rules}};

'''
    return header + '\n'.join(out) + '\n', dump


def main():
    text, dump = generate()
    if '--dump' in sys.argv:
        print('\n'.join(dump))
        return
    write_or_check(OUTPUT, rustfmt(text), 'scripts/day-periods-cldr.py')


if __name__ == '__main__':
    main()
