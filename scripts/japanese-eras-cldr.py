#!/usr/bin/env python3
"""Regenerate crates/hc-i18n/src/data/japanese_eras.rs from Unicode CLDR 48.

Every hc-i18n locale whose own CLDR 48 file states names for the eras of
`calendar type="japanese"` carries them, keyed by the era codes of the
nengō table (crates/hc-calendars-regional/src/nengo/table.rs):

    python3 scripts/japanese-eras-cldr.py            # rewrite the file
    python3 scripts/japanese-eras-cldr.py --check    # exit 1 if it is stale
    python3 scripts/japanese-eras-cldr.py --dump     # print every value, a TSV
    python3 scripts/japanese-eras-cldr.py --gaps     # print the mapping gaps

The files are the `release-48` tag of unicode-org/cldr, `common/main/
<file>.xml`, read over HTTP from raw.githubusercontent.com into memory;
nothing is saved. With CLDR_DIR set to a checkout of that tag, a file
present under `$CLDR_DIR/common/main/` is read from there instead.

What is carried, for one locale:

* the width `hc-format` writes a date's era in, `eraAbbr` (CLDR's
  `eraNames` for this calendar is an alias of it in `root.xml`), in the
  entry's wide slot, as the other Japanese entries of `hc-i18n` hold it,
  so that the other widths fall back to it;
* each era the file itself states at a release level (`approved` or
  `contributed`, or no draft attribute); a value at a lower draft level,
  the inheritance marker `↑↑↑`, an `alt` value or an era the file leaves
  out is not stated by that file, and the entry leaves it to the
  calendar's own name;
* the value exactly as the file writes it, the years with it where it
  gives them, *만엔 (1860 ~ 1861)*.

`en` carries `root.xml`'s names: `en.xml` states no Japanese eras and
inherits root's, and hc-i18n's root entry is language-free. `en` and `ja`
also carry the narrow width, resolved through `root.xml` as CLDR resolves
it, which gives the modern eras root's letters M, T, S, H and R; they are
the two entries that carried those letters before this file existed.

CLDR numbers its eras 0 to 236. Each is mapped to a code by its name in
`ja.xml`, which is unique among the eras on both sides, and the mapping is
checked to keep CLDR's order of the table's eras. An era with no code, or
a code no CLDR era covers, is a gap, listed in the generated file's header
and by `--gaps`, never guessed.
"""
import http.client
import os
import re
import subprocess
import sys
import urllib.request
import xml.etree.ElementTree as ET

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUTPUT = os.path.join(ROOT_DIR, 'crates/hc-i18n/src/data/japanese_eras.rs')
TABLE = os.path.join(ROOT_DIR, 'crates/hc-calendars-regional/src/nengo/table.rs')
BASE = 'https://raw.githubusercontent.com/unicode-org/cldr/release-48/common/'
READ = '2026-09-28'

# Every hc-i18n entry with a CLDR file, and the files whose statements it
# carries, child first. A tag with no CLDR file (ban, cop, mid, nah, yua,
# zap) has nothing to read.
LOCALES = [
    ('am', ['am']), ('ar', ['ar']), ('bn', ['bn']), ('bo', ['bo']), ('cs', ['cs']),
    ('de', ['de']), ('en', ['en', 'root']), ('es', ['es']), ('fa', ['fa']),
    ('fil', ['fil']), ('fr', ['fr']), ('ha', ['ha']), ('he', ['he']), ('hi', ['hi']),
    ('id', ['id']), ('it', ['it']), ('ja', ['ja']), ('jv', ['jv']), ('kab', ['kab']),
    ('ko', ['ko']), ('ml', ['ml']), ('mr', ['mr']), ('my', ['my']), ('ne', ['ne']),
    ('nl', ['nl']), ('pa-Arab', ['pa_Arab']), ('pa-Guru', ['pa']), ('pcm', ['pcm']),
    ('pl', ['pl']), ('ps', ['ps']), ('pt', ['pt']), ('pt-PT', ['pt_PT', 'pt']),
    ('ru', ['ru']), ('sa', ['sa']), ('sw', ['sw']), ('syr', ['syr']), ('ta', ['ta']),
    ('te', ['te']), ('th', ['th']), ('tr', ['tr']), ('ur', ['ur']), ('vi', ['vi']),
    ('yue-Hans', ['yue_Hans']), ('yue-Hant', ['yue']), ('zgh', ['zgh']),
    ('zh-Hans', ['zh']), ('zh-Hant', ['zh_Hant']),
]
# The entries that carry the narrow width too, resolved through root.
NARROW = {'en', 'ja'}

RELEASE = {None, 'approved', 'contributed'}
MARK = '↑↑↑'

_texts = {}


def fetch(name):
    if name in _texts:
        return _texts[name]
    local = os.environ.get('CLDR_DIR')
    if local:
        path = os.path.join(local, 'common/main', name + '.xml')
        if os.path.exists(path):
            with open(path, 'rb') as handle:
                _texts[name] = handle.read()
            return _texts[name]
    for attempt in range(4):
        try:
            with urllib.request.urlopen(f'{BASE}main/{name}.xml', timeout=60) as response:
                _texts[name] = response.read()
                break
        except (OSError, http.client.HTTPException):
            if attempt == 3:
                raise
    return _texts[name]


def eras(name, width):
    """The eras a file states at one width: CLDR number → value."""
    root = ET.fromstring(fetch(name))
    stated = {}
    for calendar in root.iter('calendar'):
        if calendar.get('type') != 'japanese':
            continue
        group = calendar.find(f'eras/{width}')
        if group is None:
            continue
        for era in group.findall('era'):
            if era.get('alt') or era.get('draft') not in RELEASE or era.text == MARK:
                continue
            stated[int(era.get('type'))] = era.text
    return stated


def table():
    source = open(TABLE, encoding='utf-8').read()
    rows = re.findall(r'id: "([^"]+)",\s*kanji: "([^"]+)"', source)
    assert len(rows) == 248, len(rows)
    # The table spells a kanji as the article it was read from titles it,
    # 延長 (元号), where the name alone is also another article's.
    return [(code, re.sub(r' \(元号\)$', '', kanji)) for code, kanji in rows]


def mapping():
    rows = table()
    position = {kanji: (index, code) for index, (code, kanji) in enumerate(rows)}
    assert len(position) == len(rows), 'two eras of the table share a name'
    names = eras('ja', 'eraAbbr')
    assert sorted(names) == list(range(237)), 'ja.xml does not name eras 0 to 236'
    assert len(set(names.values())) == len(names), 'two of ja.xml\'s eras share a name'
    codes, unmapped, reordered = {}, [], []
    last = -1
    for number in sorted(names):
        found = position.get(names[number])
        if found is None:
            unmapped.append((number, names[number]))
            continue
        index, code = found
        if index < last:
            reordered.append((number, names[number], code))
        last = max(last, index)
        codes[number] = code
    covered = set(codes.values())
    uncovered = [(code, kanji) for code, kanji in rows if code not in covered]
    return codes, unmapped, reordered, uncovered


def lit(text):
    return '"' + text.replace('\\', '\\\\').replace('"', '\\"') + '"'


def const_name(tag):
    return tag.replace('-', '_').upper()


def slice_rs(values):
    return '&[' + ', '.join(lit(value) for value in values) + ']'


def generate():
    codes, unmapped, reordered, uncovered = mapping()
    order = sorted(codes)
    all_codes = [codes[number] for number in order]
    out, dump, carried, silent = [], [], [], []
    for tag, files in LOCALES:
        own = {}
        for name in reversed(files):
            own.update(eras(name, 'eraAbbr'))
        stated = [number for number in order if number in own]
        for number in stated:
            dump.append(f'{tag}\t{number}\t{codes[number]}\t{own[number]}')
        if not stated:
            silent.append(tag)
            continue
        name = const_name(tag)
        carried.append((tag, files, len(stated)))
        files_md = ', '.join(f'`{f}.xml`' for f in files)
        if stated == order:
            code_list = 'CODES'
        else:
            code_list = f'{name}_CODES'
            out.append(f'/// The codes of the eras [`{name}`] names.')
            out.append(f'const {code_list}: &[&str] = '
                       + slice_rs(codes[number] for number in stated) + ';')
        out.append(f'/// `{tag}`: {files_md}, {len(stated)} eras.')
        narrow = '&[]'
        if tag in NARROW:
            resolved = {}
            for file in reversed(files + ['root']):
                resolved.update(eras(file, 'eraNarrow'))
            assert all(number in resolved for number in stated), tag
            narrow = slice_rs(resolved[number] for number in stated)
        out.append(f'pub(super) const {name}: EraNames = EraNames {{')
        out.append(f'    codes: {code_list},')
        out.append('    names: WidthSet {')
        out.append('        wide: ' + slice_rs(own[number] for number in stated) + ',')
        out.append('        abbreviated: &[],')
        out.append('        short: &[],')
        out.append(f'        narrow: {narrow},')
        out.append('    },')
        out.append('    calendars: &[],')
        out.append('};')
        out.append('')
    def bullet(text):
        return wrap('* ' + text, '  ')

    header = [
        '//! The Japanese era names of every locale whose CLDR file states them,',
        '//! generated.',
        '//!',
        '//! **Do not edit.** `scripts/japanese-eras-cldr.py` writes this file from',
        '//! Unicode CLDR 48 (`release-48`, `common/main/<file>.xml`, `calendar',
        f'//! type="japanese"`, `eraAbbr`; read {READ}, `cldr48-japanese-eras`),',
        '//! as the script\'s documentation says: each era the file itself states,',
        '//! at a release level, exactly as it writes it, keyed by the code of the',
        '//! `hc-calendars-regional` nengō table the era\'s name in `ja.xml` maps',
        '//! to. The entries in `super` take them from here.',
        '//!',
    ]
    header += wrap('Carried: ' + '; '.join(f'`{tag}` {count}' for tag, _, count in carried)
                   + '. No file of ' + ', '.join(f'`{tag}`' for tag in silent)
                   + ' states one at a release level.', '')
    header += ['//!', '//! The gaps of the mapping:', '//!']
    for number, kanji in unmapped:
        header += bullet(f'CLDR era {number}, {kanji}, is no era of the table, which has no row '
                         'for it; it is not carried.')
    for number, kanji, code in reordered:
        header += bullet(f'CLDR era {number}, {kanji} (`{code}`), comes after an era the table '
                         'lists after it; the codes here keep CLDR\'s order.')
    header += bullet('No CLDR era is ' + ', '.join(f'{kanji} (`{code}`)' for code, kanji in uncovered)
                     + '; the calendar\'s own names write them.')
    header += [
        '',
        'use crate::names::{EraNames, WidthSet};',
        '',
        f'/// The {len(all_codes)} codes of the eras CLDR numbers, in CLDR\'s order.',
        'const CODES: &[&str] = ' + slice_rs(all_codes) + ';',
        '',
    ]
    gaps = [f'unmapped\t{n}\t{k}' for n, k in unmapped]
    gaps += [f'reordered\t{n}\t{k}\t{c}' for n, k, c in reordered]
    gaps += [f'uncovered\t{c}\t{k}' for c, k in uncovered]
    gaps += [f'silent\t{tag}' for tag in silent]
    text = '\n'.join(header + out).rstrip('\n') + '\n'
    return text, dump, gaps


def wrap(text, indent):
    """A `//!` paragraph, wrapped at 80 columns."""
    import textwrap
    return ['//! ' + line for line in textwrap.wrap(
        text, 76, subsequent_indent=indent, break_on_hyphens=False, break_long_words=False)]


def rustfmt(text):
    result = subprocess.run(['rustfmt', '--edition', '2024', '--emit', 'stdout'],
                            input=text.encode(), capture_output=True, check=True)
    return result.stdout.decode()


def main():
    text, dump, gaps = generate()
    if '--dump' in sys.argv:
        print('\n'.join(dump))
        return
    if '--gaps' in sys.argv:
        print('\n'.join(gaps))
        return
    text = rustfmt(text)
    if '--check' in sys.argv:
        with open(OUTPUT, encoding='utf-8') as handle:
            if handle.read() != text:
                print(f'{OUTPUT} is stale: run scripts/japanese-eras-cldr.py', file=sys.stderr)
                sys.exit(1)
        return
    os.makedirs(os.path.dirname(OUTPUT), exist_ok=True)
    with open(OUTPUT, 'w', encoding='utf-8') as handle:
        handle.write(text)


if __name__ == '__main__':
    main()
