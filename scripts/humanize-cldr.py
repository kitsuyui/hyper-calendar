#!/usr/bin/env python3
"""Regenerate crates/hc-humanize/src/data/cldr48.rs from Unicode CLDR 48.

Every one of the 39 locales hc-humanize carries takes its relative-time
phrases, its duration patterns, its list patterns, its decimal separator
and its relative date-time pattern (UTS #35 Part 4's `relative`
`dateTimeFormat`) from its CLDR 48 file, resolved as CLDR resolves it, and
then the documented overrides of
crates/hc-humanize/src/data/cldr48_overrides.tsv, each with its reason:

    python3 scripts/humanize-cldr.py            # rewrite the file
    python3 scripts/humanize-cldr.py --check    # exit 1 if it is stale
    python3 scripts/humanize-cldr.py --dump     # print every value, a TSV

The files are the `release-48` tag of unicode-org/cldr,
`common/main/<file>.xml`, `root.xml` for its aliases and
`common/supplemental/plurals.xml` for the plural categories, read over
HTTP from raw.githubusercontent.com into memory; nothing is saved.

The resolution, for one value at one path, is TR35's (Part 1, "Inheritance
and Validity"): the locale's own files, child first, then `root.xml`; a
value at a draft level below `contributed` is absent, and so is the
inheritance marker `↑↑↑`; where no file states the path, the longest
prefix `root.xml` aliases is rewritten through the alias (a narrow style
to the short one, the short style to the long one); and last, a plural
count nobody states takes the style's `other` (lateral inheritance). On
top of that, three choices are this crate's:

* a plural category no file writes, directly or through root's aliases,
  is left empty, and the crate reads an empty category as `other`, as a
  reader of CLDR's resolved data does;
* a style that says nothing the wider style does not is left empty, and
  falls back to it;
* a relative word root alone gives (root's English *yesterday*) is not
  carried, since this crate's root is language-free.

The strings CLDR has no field for — the hedges, the weekday phrases, the
half-unit idioms, the compact suffixes and the indefinite units — are not
in the generated file: they are hand-written in src/data.rs, which puts
each entry together from this file's CLDR part and its own.
"""
import http.client
import os
import re
import sys
import urllib.request
import xml.etree.ElementTree as ET

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUTPUT = os.path.join(ROOT_DIR, 'crates/hc-humanize/src/data/cldr48.rs')
OVERRIDES = os.path.join(ROOT_DIR, 'crates/hc-humanize/src/data/cldr48_overrides.tsv')
BASE = 'https://raw.githubusercontent.com/unicode-org/cldr/release-48/common/'

# Each carried tag and its CLDR files, child first, root left out. A tag
# names its script where CLDR's likely subtags would give a plain tag a
# different file than the entry holds: `pa` is Gurmukhi, `yue` Traditional.
LOCALES = [
    ('ar', ['ar']),
    ('ar-EG', ['ar_EG', 'ar']),
    ('cs', ['cs']),
    ('cy', ['cy']),
    ('de', ['de']),
    ('en', ['en']),
    ('en-001', ['en_001', 'en']),
    ('en-GB', ['en_GB', 'en_001', 'en']),
    ('es', ['es']),
    ('es-419', ['es_419', 'es']),
    ('fil', ['fil']),
    ('fr', ['fr']),
    ('ha', ['ha']),
    ('hi', ['hi']),
    ('id', ['id']),
    ('it', ['it']),
    ('ja', ['ja']),
    ('ko', ['ko']),
    ('mn', ['mn']),
    ('mr', ['mr']),
    ('nl', ['nl']),
    ('pa-Guru', ['pa']),
    ('pcm', ['pcm']),
    ('pl', ['pl']),
    ('pt', ['pt']),
    ('pt-PT', ['pt_PT', 'pt']),
    ('ru', ['ru']),
    ('sw', ['sw']),
    ('te', ['te']),
    ('th', ['th']),
    ('tr', ['tr']),
    ('ur', ['ur']),
    ('ur-IN', ['ur_IN', 'ur']),
    ('vi', ['vi']),
    ('yue-Hans', ['yue_Hans']),
    ('yue-Hant', ['yue']),
    ('zh', ['zh']),
    ('zh-Hant', ['zh_Hant']),
    ('zh-Hant-HK', ['zh_Hant_HK', 'zh_Hant']),
]

# The digits hc-i18n writes a locale's numbers in where they are not CLDR's
# default system (`hc_i18n::data`, `numbering`): none, since hc-i18n writes
# each entry's CLDR default (`hc_i18n::data::DEFAULT_NUMBERING`), `ar`'s
# `latn` and `ar-EG`'s `arab`. tests/cldr48_resolved.rs holds hc-i18n to
# this.
NUMBERING = {}

UNITS = ['second', 'minute', 'hour', 'day', 'week', 'month', 'quarter', 'year']
CATS = ['zero', 'one', 'two', 'few', 'many', 'other']
OFFSETS = ['-2', '-1', '0', '1', '2']
STYLES = [('long', '', 'long'), ('short', '-short', 'short'), ('narrow', '-narrow', 'narrow')]
LISTS = [('standard', None), ('unit', 'unit'), ('narrow', 'unit-narrow')]
LIST_PARTS = ['2', 'start', 'middle', 'end']
RELEASE = {None, 'approved', 'contributed'}
MARK = '↑↑↑'
DISTINGUISHING = {'type', 'alt', 'id', 'count', 'key', 'yeartype', 'numberSystem', 'scope',
                  'menu', 'case', 'gender'}

# --- reading the files ------------------------------------------------------

_texts = {}


def fetch(path):
    for attempt in range(4):
        if path in _texts:
            break
        try:
            with urllib.request.urlopen(BASE + path, timeout=60) as response:
                _texts[path] = response.read()
        except (OSError, http.client.HTTPException):
            if attempt == 3:
                raise
    return _texts[path]


def segment(element):
    attrs = tuple(sorted((k, v) for k, v in element.attrib.items() if k in DISTINGUISHING))
    return (element.tag, attrs)


_files = {}


def load(name):
    """The leaves of a file, path → (value, draft), and root's aliases."""
    if name in _files:
        return _files[name]
    root = ET.fromstring(fetch(f'main/{name}.xml'))
    leaves, aliases = {}, {}

    def walk(element, path):
        for child in element:
            if not isinstance(child.tag, str):
                continue
            here = path + (segment(child),)
            if child.tag == 'alias':
                aliases[path] = child.get('path')
            elif not any(isinstance(k.tag, str) for k in child):
                leaves[here] = ((child.text or '').strip(), child.get('draft'))
            else:
                walk(child, here)

    walk(root, ())
    _files[name] = (leaves, aliases)
    return _files[name]


def through_alias(base, relative):
    """Apply an alias path such as `../field[@type='day']` to a base path."""
    out = list(base)
    for part in relative.split('/'):
        if part == '..':
            out.pop()
            continue
        match = re.match(r"([A-Za-z-]+)((?:\[@[a-zA-Z]+='[^']*'\])*)$", part)
        attrs = tuple(sorted(re.findall(r"\[@([a-zA-Z]+)='([^']*)'\]", match.group(2))))
        if (match.group(1), attrs) == ('dateTimeFormat', (('type', 'standard'),)):
            # the DTD's default type, which the files leave unwritten
            attrs = ()
        out.append((match.group(1), attrs))
    return tuple(out)


def path(*parts):
    """A path from `tag`, `tag[type]` or (`tag`, {attrs}) parts."""
    out = []
    for part in parts:
        if isinstance(part, str):
            if '[' in part:
                tag, value = part[:-1].split('[')
                out.append((tag, (('type', value),)))
            else:
                out.append((part, ()))
        else:
            tag, attrs = part
            out.append((tag, tuple(sorted(attrs.items()))))
    return tuple(out)


def resolve(chain, at, depth=0, marked=False, lateral=True):
    """(value, file, marked), or (None, None, marked). `marked` says a file
    of the chain wrote the inheritance marker on the way."""
    for name in chain + ['root']:
        leaves, _ = load(name)
        if at in leaves:
            value, draft = leaves[at]
            if draft not in RELEASE:
                continue
            if value == MARK:
                marked = marked or name != 'root'
                continue
            assert value != '∅∅∅', ("CLDR's empty override ∅∅∅ at a path this script reads: it has no encoding for it yet (scripts/cldr_xml.py's resolve stops the lookup there)", name, at)
            return value, name, marked
    _, aliases = load('root')
    for n in range(len(at), 0, -1):
        prefix = at[:n]
        if prefix in aliases and depth < 20:
            return resolve(chain, through_alias(prefix, aliases[prefix]) + at[n:], depth + 1,
                           marked, lateral)
    tag, attrs = at[-1]
    counts = dict(attrs)
    if lateral and counts.get('count') not in (None, 'other') and depth < 20:
        counts['count'] = 'other'
        return resolve(chain, at[:-1] + ((tag, tuple(sorted(counts.items()))),), depth + 1,
                       marked)
    return None, None, marked


def exists(chain, at, depth=0):
    """Whether a file of the chain, root included, writes the path, directly
    or through root's aliases: whether CLDR's resolved data has it at all."""
    for name in chain + ['root']:
        if at in load(name)[0]:
            return True
    _, aliases = load('root')
    for n in range(len(at), 0, -1):
        prefix = at[:n]
        if prefix in aliases and depth < 20:
            return exists(chain, through_alias(prefix, aliases[prefix]) + at[n:], depth + 1)
    return False


def own(chain, at, lateral=True):
    """The resolved value where the locale's files state it, a value or the
    marker; a value only root gives is None."""
    value, name, marked = resolve(chain, at, lateral=lateral)
    return value if name is not None and (name != 'root' or marked) else None


_plurals = None


def categories(chain):
    global _plurals
    if _plurals is None:
        _plurals = {}
        rules = ET.fromstring(fetch('supplemental/plurals.xml'))
        for group in rules.iter('pluralRules'):
            for locale in group.get('locales').split():
                _plurals[locale] = [r.get('count') for r in group.iter('pluralRule')]
    locale = chain[0]
    while locale not in _plurals:
        locale = locale.rsplit('_', 1)[0]
    return _plurals[locale]


# --- the values of one locale ----------------------------------------------


def plural(chain, base):
    kind = 'relativeTimePattern' if base[-1][0] == 'relativeTime' else 'unitPattern'
    stated = categories(chain)

    def at(category):
        return base + ((kind, (('count', category),)),)

    forms = {}
    for category in CATS:
        if category in stated and (category == 'other' or exists(chain, at(category))):
            forms[category] = own(chain, at(category)) or ''
        else:
            forms[category] = ''
    # A category only lateral inheritance fills is left empty where it is
    # the style's `other`, which the crate reads an empty category as.
    for category in CATS[:-1]:
        if forms[category] and forms[category] == forms['other']:
            if own(chain, at(category), lateral=False) is None:
                forms[category] = ''
    return forms


def values(chain):
    """Every CLDR value of one locale, keyed by override path."""
    out = {}
    for style, suffix, length in STYLES:
        for unit in UNITS:
            field = path('dates', 'fields', f'field[{unit}{suffix}]')
            for kind, base in [
                ('past', field + (('relativeTime', (('type', 'past'),)),)),
                ('future', field + (('relativeTime', (('type', 'future'),)),)),
                ('count', path('units', ('unitLength', {'type': length}),
                               ('unit', {'type': 'duration-' + unit}))),
            ]:
                for category, value in plural(chain, base).items():
                    out[f'{style}.{unit}.{kind}.{category}'] = value
            for offset in OFFSETS:
                word = own(chain, field + (('relative', (('type', offset),)),))
                out[f'{style}.{unit}.relative.{offset}'] = word or ''
    for name, kind in LISTS:
        attrs = {} if kind is None else {'type': kind}
        for part in LIST_PARTS:
            value = resolve(chain, path('listPatterns', ('listPattern', attrs),
                                        ('listPatternPart', {'type': part})))[0]
            out[f'list.{name}.{part}'] = value
    # The separator of the digits the crate writes the locale's numbers in,
    # which hc-i18n chooses: CLDR's default numbering system but where
    # NUMBERING names another.
    system = NUMBERING.get(chain[0]) or resolve(chain, path('numbers',
                                                             'defaultNumberingSystem'))[0]
    out['decimal'] = resolve(chain, path('numbers', ('symbols', {'numberSystem': system}),
                                         'decimal'))[0]
    out['at'] = at_pattern(chain)
    return out


def at_pattern(chain):
    """How a relative day and a time of day combine, in this crate's order:
    CLDR's `{1}` is the date and `{0}` the time, this crate's `{0}` is the
    day phrase and `{1}` the time.

    UTS #35 Part 4 ("Element dateTimeFormat"): "For a relative date with a
    single time, by default use the relative pattern (if available) to
    produce an event time". So the long `dateTimeFormat[@type='relative']`
    first, then the long `atTime` pattern, then the standard one. root.xml
    aliases the relative pattern to the standard one, so for every file
    CLDR 48 resolves the first and the other two are never reached."""
    greg = ('dates', 'calendars', 'calendar[gregorian]', 'dateTimeFormats',
            'dateTimeFormatLength[long]')
    for kind in ({'type': 'relative'}, {'type': 'atTime'}, {}):
        found = resolve(chain, path(*greg, ('dateTimeFormat', kind), 'pattern'))
        if found[0] is not None:
            break
    value, name, marked = found
    assert value and (name != 'root' or marked), (chain, found)
    swapped = value.replace('{0}', '\x00').replace('{1}', '{0}').replace('\x00', '{1}')
    return swapped.replace("'", '')


# --- overrides --------------------------------------------------------------


def read_overrides():
    """(tag, key) → (value, reason). A line is `tag<TAB>key<TAB>value<TAB>
    reason`; a value of `-` states no value."""
    out = {}
    with open(OVERRIDES, encoding='utf-8') as handle:
        for number, line in enumerate(handle, 1):
            line = line.rstrip('\n')
            if not line or line.startswith('#'):
                continue
            cells = line.split('\t')
            assert len(cells) == 4, f'{OVERRIDES}:{number}: four cells'
            tag, key, value, reason = cells
            assert (tag, key) not in out, f'{OVERRIDES}:{number}: listed twice'
            out[(tag, key)] = ('' if value == '-' else value, reason)
    return out


# --- writing Rust -----------------------------------------------------------


def lit(text):
    return '"' + text.replace('\\', '\\\\').replace('"', '\\"') + '"'


def forms_rs(forms):
    if not forms['other']:
        return None
    if not any(forms[c] for c in ('zero', 'one', 'two', 'few', 'many')):
        return f'p1({lit(forms["other"])})'
    if not any(forms[c] for c in ('zero', 'two', 'few', 'many')):
        return f'p2({lit(forms["one"])}, {lit(forms["other"])})'
    if not any(forms[c] for c in ('zero', 'two')):
        return 'p4(' + ', '.join(lit(forms[c]) for c in ('one', 'few', 'many', 'other')) + ')'
    return 'p6(' + ', '.join(lit(forms[c]) for c in CATS) + ')'


def unit_rs(data, style, unit):
    parts = []
    for kind in ('past', 'future', 'count'):
        parts.append(forms_rs({c: data[f'{style}.{unit}.{kind}.{c}'] for c in CATS}))
    if None in parts:
        return None
    words = {o: data[f'{style}.{unit}.relative.{o}'] for o in OFFSETS}
    if words['-2'] or words['2']:
        return (f'u_day({", ".join(parts)}, '
                + ', '.join(lit(words[o]) for o in OFFSETS) + ')')
    return f'u({", ".join(parts)}, ' + ', '.join(lit(words[o]) for o in ('-1', '0', '1')) + ')'


def comments(notes, prefix):
    """The overrides under a key prefix, one comment per reason."""
    import textwrap
    grouped = {}
    for key, reason in notes:
        if key.startswith(prefix):
            grouped.setdefault(reason, []).append(key)
    lines = []
    for reason, keys in grouped.items():
        text = 'Override ' + ', '.join(f'`{k}`' for k in keys) + ': ' + reason
        lines.extend('    // ' + line for line in textwrap.wrap(text, 92, break_on_hyphens=False))
    return lines


def const_name(tag):
    return tag.replace('-', '_').upper()


def locale_rs(tag, chain, data, notes):
    name = const_name(tag)
    styles = {}
    for style, _, _ in STYLES:
        styles[style] = {u: unit_rs(data, style, u) for u in UNITS}
    assert all(styles['long'].values()), (tag, styles['long'])
    stated = ['long']
    if styles['short'] != styles['long'] and all(styles['short'].values()):
        stated.append('short')
    wider = styles['short'] if 'short' in stated else styles['long']
    if styles['narrow'] != wider and all(styles['narrow'].values()):
        stated.append('narrow')
    files = ', '.join(f'`{f}.xml`' for f in chain + ['root'])
    out = [f'// --- {tag}: {files} ' + '-' * max(3, 70 - len(tag) - len(files)), '']
    for style in stated:
        out.append(f'const {name}_{style.upper()}: StyleData = StyleData {{')
        for unit in UNITS:
            out.extend(comments(notes, f'{style}.{unit}.'))
            out.append(f'    {unit}: {styles[style][unit]},')
        out.append('};')
        out.append('')

    def list_rs(kind):
        parts = {p: data[f'list.{kind}.{p}'] for p in LIST_PARTS}
        return (f'ListForms {{ two: {lit(parts["2"])}, start: {lit(parts["start"])}, '
                f'middle: {lit(parts["middle"])}, end: {lit(parts["end"])} }}')

    out.append(f'/// `{tag}` as CLDR 48 resolves it, with its overrides.')
    out.append(f'pub(super) const {name}: LocaleData = LocaleData {{')
    out.append(f'    tag: {lit(tag)},')
    for style, _, _ in STYLES:
        if style in stated:
            value = f'{name}_{style.upper()}'
        else:
            # The style says what the wider one does, overrides included.
            out.extend(comments(notes, f'{style}.'))
            value = 'StyleData::EMPTY'
        out.append(f'    {style}: {value},')
    out.append('    compact: UnitStrings::EMPTY,')
    out.append('    indefinite: UnitStrings::EMPTY,')
    out.extend(comments(notes, 'list.'))
    out.append('    list: ListPatterns {')
    for kind, _ in LISTS:
        out.append(f'        {kind}: {list_rs(kind)},')
    out.append('    },')
    out.append('    approximate: LANGUAGE_FREE_HEDGES,')
    out.append('    weekday: LANGUAGE_FREE_WEEKDAYS,')
    out.extend(comments(notes, 'decimal'))
    out.append(f'    decimal_separator: {lit(data["decimal"])},')
    out.extend(comments(notes, 'at'))
    out.append(f'    at_pattern: {lit(data["at"])},')
    out.append('};')
    out.append('')
    return out


HEADER = '''\
//! The CLDR part of every locale's entry, generated.
//!
//! **Do not edit.** `scripts/humanize-cldr.py` writes this file from the
//! Unicode CLDR 48 files (`release-48`, `common/main/<file>.xml` over
//! `root.xml`, and `supplemental/plurals.xml`), resolved as the script's
//! documentation says, and then applies `cldr48_overrides.tsv`; each
//! override is written above the value it replaces, with its reason. The
//! strings CLDR has no field for are not here: `super` adds them.

use super::{LANGUAGE_FREE_HEDGES, LANGUAGE_FREE_WEEKDAYS, p1, p2, p4, p6, u, u_day};
use crate::pattern::{ListForms, ListPatterns, LocaleData, StyleData, UnitStrings};

'''


def generate():
    overrides = read_overrides()
    known = {tag for tag, _ in LOCALES}
    for tag, key in overrides:
        assert tag in known, f'override for {tag}, which is not carried'
    out = []
    dump = []
    for tag, chain in LOCALES:
        data = values(chain)
        notes = []
        for (otag, key), (value, reason) in sorted(overrides.items()):
            if otag != tag:
                continue
            assert key in data, f'{tag} {key}: no such value'
            # An `(r)` row records a value where this resolution, UTS #35's,
            # and cldr-json's differ, and states the value both carry.
            assert data[key] != value or reason.startswith('(r)'), \
                f'{tag} {key}: the override is CLDR\'s own value'
            data[key] = value
            notes.append((key, reason))
        for key, value in data.items():
            dump.append(f'{tag}\t{key}\t{value}')
        out.extend(locale_rs(tag, chain, data, notes))
    return HEADER + '\n'.join(out).rstrip('\n') + '\n', dump


def rustfmt(text):
    import subprocess
    result = subprocess.run(['rustfmt', '--edition', '2024', '--emit', 'stdout'],
                            input=text.encode(), capture_output=True, check=True)
    return result.stdout.decode()


def main():
    text, dump = generate()
    if '--dump' in sys.argv:
        print('\n'.join(dump))
        return
    text = rustfmt(text)
    if '--check' in sys.argv:
        with open(OUTPUT, encoding='utf-8') as handle:
            if handle.read() != text:
                print(f'{OUTPUT} is stale: run scripts/humanize-cldr.py', file=sys.stderr)
                sys.exit(1)
        return
    with open(OUTPUT, 'w', encoding='utf-8') as handle:
        handle.write(text)


if __name__ == '__main__':
    main()
