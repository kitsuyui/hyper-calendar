#!/usr/bin/env python3
"""Regenerate crates/hc-i18n/src/plural/cldr48.rs from Unicode CLDR 48.

The cardinal and ordinal plural rules of every language CLDR 48 gives one,
as data for `hc_i18n::plural`'s one evaluator:

    python3 scripts/plurals-cldr.py            # rewrite the file
    python3 scripts/plurals-cldr.py --check    # exit 1 if it is stale
    python3 scripts/plurals-cldr.py --dump     # print every rule, a TSV

The files, read through scripts/cldr_xml.py (the `release-48` tag, over HTTP
into memory, nothing saved):

* `common/supplemental/plurals.xml`, the cardinal rules, and
  `common/supplemental/ordinals.xml`, the ordinal rules. Each is blocks of
  `pluralRules` keyed by the locales that share them, and each block's
  `pluralRule` elements give a category's condition in the syntax of UTS #35
  Part 3, "Plural rules syntax", followed by its `@integer` and `@decimal`
  samples.

A condition is `and_condition ('or' and_condition)*`, an `and_condition`
`relation ('and' relation)*`, and a relation an operand (`n i v w f t c e`),
optionally `% value`, then `=` or `!=` and a list of values and ranges. The
file is parsed to that shape and written as nested slices of `Relation`,
one shared rule set per block, and a sorted table from each locale to its
block; `root` is left out, since a language with no row takes root's
`other` in the crate. The samples are written as text beside the first
locale of each block, under `cfg(test)`, for the crate's differential test
to expand and check; a sample with a compact exponent (`1c6`) is kept as
written and the test skips it, since the operands `c` and `e` are not
carried.
"""
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cldr_xml import lit, rustfmt, write_or_check, xml  # noqa: E402

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUTPUT = os.path.join(ROOT_DIR, 'crates/hc-i18n/src/plural/cldr48.rs')
READ = '2026-10-04'
CATEGORIES = ['zero', 'one', 'two', 'few', 'many', 'other']
RUST_CATEGORY = {c: c.capitalize() for c in CATEGORIES}
OPERANDS = {'n': 'N', 'i': 'I', 'v': 'V', 'w': 'W', 'f': 'F', 't': 'T', 'c': 'C', 'e': 'C'}
TOKEN = re.compile(r'\s*(\.\.|!=|=|%|,|[a-z]+|\d+)')


def tokens(text):
    out, at = [], 0
    text = text.strip()
    while at < len(text):
        match = TOKEN.match(text, at)
        if not match:
            raise ValueError(f'{text!r}: cannot read at {at}')
        out.append(match.group(1))
        at = match.end()
    return out


def parse_condition(text):
    """[[relation]]: an `or` of `and`s; a relation is (operand, modulus,
    negated, [(low, high)])."""
    toks = tokens(text)
    if not toks:
        return []
    out, group, at = [], [], 0
    while at < len(toks):
        operand = toks[at]
        if operand not in OPERANDS:
            raise ValueError(f'{text!r}: operand {operand}')
        at += 1
        modulus = 0
        if toks[at] in ('%', 'mod'):
            modulus = int(toks[at + 1])
            at += 2
        if toks[at] in ('=', 'in', 'is'):
            negated = False
        elif toks[at] == '!=':
            negated = True
        elif toks[at] == 'not' and toks[at + 1] in ('in', 'is'):
            negated, at = True, at + 1
        else:
            raise ValueError(f'{text!r}: relation {toks[at]}')
        at += 1
        ranges = []
        while True:
            low = int(toks[at])
            at += 1
            high = low
            if at < len(toks) and toks[at] == '..':
                high = int(toks[at + 1])
                at += 2
            ranges.append((low, high))
            if at < len(toks) and toks[at] == ',':
                at += 1
                continue
            break
        group.append((OPERANDS[operand], modulus, negated, ranges))
        if at < len(toks):
            if toks[at] == 'and':
                at += 1
            elif toks[at] == 'or':
                out.append(group)
                group = []
                at += 1
            else:
                raise ValueError(f'{text!r}: {toks[at]}')
    out.append(group)
    return out


def samples(text):
    """(integer samples, decimal samples), each the text after the marker."""
    integer = decimal = ''
    for part in re.split(r'(?=@)', text):
        if part.startswith('@integer'):
            integer = part[len('@integer'):].strip()
        elif part.startswith('@decimal'):
            decimal = part[len('@decimal'):].strip()
    return integer, decimal


def blocks(name):
    """[(locales, [(category, condition text, condition, integer samples,
    decimal samples)])] in file order."""
    root = xml(f'supplemental/{name}.xml')
    out = []
    for group in root.iter('pluralRules'):
        rules = []
        for rule in group.findall('pluralRule'):
            text = rule.text or ''
            condition = text.split('@', 1)[0].strip()
            rules.append((rule.get('count'), condition, parse_condition(condition), *samples(text)))
        out.append((group.get('locales').split(), rules))
    return out


def tag(locale):
    return locale.replace('_', '-')


def rust_relation(relation):
    operand, modulus, negated, ranges = relation
    pairs = ', '.join(f'({low}, {high})' for low, high in ranges)
    return (f'Relation {{ operand: Operand::{operand}, modulus: {modulus}, '
            f'negated: {"true" if negated else "false"}, ranges: &[{pairs}] }}')


def rust_rules(rules):
    items = []
    for category, text, condition, _, _ in rules:
        if category == 'other':
            continue
        groups = ', '.join('&[' + ', '.join(rust_relation(r) for r in group) + ']'
                           for group in condition)
        items.append(f'    // {category}: {text}\n'
                     f'    Rule {{ category: PluralCategory::{RUST_CATEGORY[category]}, '
                     f'condition: &[{groups}] }},')
    return '&[\n' + '\n'.join(items) + '\n]' if items else '&[]'


def section(name, kind, out, dump):
    rows, sample_rows = [], []
    for number, (locales, rules) in enumerate(blocks(name)):
        const = f'{kind.upper()}_{number}'
        out.append(f'/// The {kind} rules of `{" ".join(locales)}`.')
        out.append(f'static {const}: &[Rule] = {rust_rules(rules)};')
        out.append('')
        for locale in locales:
            if locale == 'root':
                continue
            rows.append((tag(locale), const))
            for category, text, _, integer, decimal in rules:
                dump.append(f'{kind}\t{tag(locale)}\t{category}\t{text}\t{integer}\t{decimal}')
        first = tag(next(locale for locale in locales if locale != 'root'))
        cells = ', '.join(f'(PluralCategory::{RUST_CATEGORY[category]}, {lit(integer)}, {lit(decimal)})'
                          for category, _, _, integer, decimal in rules)
        sample_rows.append(f'    ({lit(first)}, &[{cells}]),')
    rows.sort()
    out.append(f'/// Every locale of `{name}.xml` but `root`, sorted, with its {kind} rules.')
    out.append(f'pub(super) static {kind.upper()}: &[(&str, &[Rule])] = &[')
    out += [f'    ({lit(t)}, {c}),' for t, c in rows]
    out.append('];')
    out.append('')
    out.append(f'/// The samples of each {kind} block, under the first locale of the block:')
    out.append('/// (category, `@integer` samples, `@decimal` samples) as the file writes them.')
    out.append('#[cfg(test)]')
    out.append(f'pub(super) static {kind.upper()}_SAMPLES: &[(&str, Samples)] = &[')
    out += sample_rows
    out.append('];')
    out.append('')
    return len(rows)


def generate():
    out, dump = [], []
    cardinal = section('plurals', 'cardinal', out, dump)
    ordinal = section('ordinals', 'ordinal', out, dump)
    header = f'''//! The plural rules of every language CLDR 48 gives one, generated.
//!
//! **Do not edit.** `scripts/plurals-cldr.py` writes this file from Unicode
//! CLDR 48 (`release-48`, read {READ}, `cldr48-supplemental`):
//! `supplemental/plurals.xml`, the cardinal rules of {cardinal} locales in
//! {out_count(out, 'CARDINAL_')} blocks, and `supplemental/ordinals.xml`, the
//! ordinal rules of {ordinal} locales in {out_count(out, 'ORDINAL_')} blocks,
//! each block's conditions parsed to the relations of UTS #35 Part 3 and
//! written as data for `super`'s one evaluator, as the script's
//! documentation says.

use super::{{Operand, PluralCategory, Relation, Rule}};
#[cfg(test)]
use super::Samples;

'''
    return header + '\n'.join(out).rstrip('\n') + '\n', dump


def out_count(out, prefix):
    return sum(1 for line in out if line.startswith(f'static {prefix}'))


def main():
    text, dump = generate()
    if '--dump' in sys.argv:
        print('\n'.join(dump))
        return
    write_or_check(OUTPUT, rustfmt(text), 'scripts/plurals-cldr.py')


if __name__ == '__main__':
    main()
