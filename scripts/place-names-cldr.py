#!/usr/bin/env python3
"""Regenerate crates/hc-i18n/src/place_names/cldr48.rs from Unicode CLDR 48.

What every locale hc-i18n carries calls each territory and each ISO 3166-2
subdivision CLDR 48 names, for hc-i18n's `place_names` module:

    python3 scripts/place-names-cldr.py            # rewrite the file
    python3 scripts/place-names-cldr.py --check    # exit 1 if it is stale
    python3 scripts/place-names-cldr.py --dump     # print every value, a TSV
    python3 scripts/place-names-cldr.py --stats    # print the sizes

The files are the `release-48` tag of unicode-org/cldr, read over HTTP from
raw.githubusercontent.com into memory; nothing is saved:

* `common/main/<file>.xml`, `localeDisplayNames/territories/territory`,
  for the territory names;
* `common/subdivisions/<file>.xml`, `localeDisplayNames/subdivisions/
  subdivision`, for the subdivision names;
* `common/validity/region.xml` and `common/validity/subdivision.xml` for
  each code's status (`regular`, `deprecated`, `macroregion` ...);
* `common/supplemental/supplementalMetadata.xml`, `territoryAlias` and
  `subdivisionAlias`, to check that no deprecated code a file names has a
  replacement (in CLDR 48 the aliases of those codes are commented out);
* `common/supplemental/supplementalData.xml`, `parentLocales`, for each
  file's parent.

A value is a plain one (no `alt`), at the draft levels `approved`,
`contributed` or `provisional`: TR35 Part 1, "Attribute draft", lets an
implementation "accept the provisional data, especially if there is no
translated alternative", and all but three of a locale's subdivision names
are provisional. An `unconfirmed` value, and CLDR's inheritance marker
`↑↑↑`, are absent, so that the lookup goes on to the parent.

A table holds the values of one carried locale's own files: its file, and
the files between it and the next file that is itself a carried locale or
root. Each table names its parent table, if any, so that the runtime walks
CLDR's chain (TR35 Part 1, "Inherited item lookup"); root names nothing
here, and after the chain the lookup takes English, `en.xml`, as the
fallback locale TR35's "Lookup" section allows before root.

Only the names that differ from English are stored: a table has a bit per
code saying whether its files name the code, and one line of text per
named code, empty where the name is English's.

CLDR writes a subdivision as a region code and a suffix in lower case with
no hyphen, `jp13`; ISO 3166-2 writes `JP-13`. The module keeps the ISO
form: the first two letters in upper case, a hyphen, and the rest in upper
case. Every id CLDR 48 names has that shape (a region of two letters and a
suffix of one to three letters or digits); an id of CLDR's own, with a
four-character suffix or a numeric region, would stop the script.
"""
import collections
import http.client
import os
import re
import subprocess
import sys
import textwrap
import urllib.error
import urllib.request
import xml.etree.ElementTree as ET
from concurrent.futures import ThreadPoolExecutor

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUTPUT = os.path.join(ROOT_DIR, 'crates/hc-i18n/src/place_names/cldr48.rs')
BASE = 'https://raw.githubusercontent.com/unicode-org/cldr/release-48/common/'

# Every locale hc-i18n carries (`hc_i18n::data::LOCALES`) and the CLDR file
# its data comes from: the tag's own, but where CLDR's default content puts
# the script's data in the plain file (`pa.xml` is Gurmukhi, `yue.xml`
# Traditional, `zh.xml` Simplified). A file CLDR 48 does not have is
# answered by English.
LOCALES = [
    ('am', 'am'), ('ar', 'ar'), ('ban', 'ban'), ('bn', 'bn'), ('bo', 'bo'), ('cop', 'cop'),
    ('cs', 'cs'), ('de', 'de'), ('en', 'en'), ('es', 'es'), ('fa', 'fa'), ('fil', 'fil'),
    ('fr', 'fr'), ('ha', 'ha'), ('he', 'he'), ('hi', 'hi'), ('id', 'id'), ('it', 'it'),
    ('ja', 'ja'), ('jv', 'jv'), ('kab', 'kab'), ('ko', 'ko'), ('mid', 'mid'), ('ml', 'ml'),
    ('mr', 'mr'), ('my', 'my'), ('nah', 'nah'), ('ne', 'ne'), ('nl', 'nl'),
    ('pa-Arab', 'pa_Arab'), ('pa-Guru', 'pa'), ('pcm', 'pcm'), ('pl', 'pl'), ('ps', 'ps'),
    ('pt', 'pt'), ('pt-PT', 'pt_PT'), ('ru', 'ru'), ('sa', 'sa'), ('sw', 'sw'), ('syr', 'syr'),
    ('ta', 'ta'), ('te', 'te'), ('th', 'th'), ('tr', 'tr'), ('ur', 'ur'), ('vi', 'vi'),
    ('yua', 'yua'), ('yue-Hans', 'yue_Hans'), ('yue-Hant', 'yue'), ('zap', 'zap'),
    ('zgh', 'zgh'), ('zh-Hans', 'zh'), ('zh-Hant', 'zh_Hant'),
]
ENGLISH = 'en'
ACCEPTED = {'approved', 'contributed', 'provisional'}
MARK = '↑↑↑'
DRAFTS = ['approved', 'contributed', 'provisional']
KINDS = ['territories', 'subdivisions']

# --- reading the files ------------------------------------------------------

_texts = {}


def fetch(path):
    """The bytes of a file of the release, or None where it has none."""
    if path in _texts:
        return _texts[path]
    for attempt in range(6):
        try:
            with urllib.request.urlopen(BASE + path, timeout=60) as response:
                _texts[path] = response.read()
            break
        except urllib.error.HTTPError as error:
            if error.code == 404:
                _texts[path] = None
                break
            if attempt == 5:
                raise
        except (OSError, http.client.HTTPException):
            if attempt == 5:
                raise
    return _texts[path]


def prefetch(paths):
    with ThreadPoolExecutor(8) as pool:
        list(pool.map(fetch, paths))


def values(path, element):
    """Every plain value of `element` in a file: type → (text, draft), at
    the accepted levels, the marker left out. The draft of an element is
    its own, else its nearest ancestor's, else `approved`."""
    data = fetch(path)
    if data is None:
        return {}
    root = ET.fromstring(data)
    out = {}

    def walk(node, draft):
        draft = node.get('draft', draft)
        if node.tag == element:
            text = (node.text or '').strip()
            if node.get('alt') is None and draft in ACCEPTED and text and text != MARK:
                assert node.get('type') not in out, (path, node.get('type'))
                assert '\t' not in text and '\n' not in text, (path, text)
                out[node.get('type')] = (text, draft)
            return
        for child in node:
            if isinstance(child.tag, str):
                walk(child, draft)

    walk(root, 'approved')
    return out


def own_values(kind, name):
    if kind == 'territories':
        return values(f'main/{name}.xml', 'territory')
    return values(f'subdivisions/{name}.xml', 'subdivision')


def expand(tokens):
    """The ids of a validity list: `lv003~6` is lv003 to lv006, the range
    running over the last characters (TR35 Part 1, "String Range")."""
    out = []
    for token in tokens.split():
        if '~' not in token:
            out.append(token)
            continue
        start, end = token.split('~')
        prefix = start[:len(start) - len(end)]
        tails = ['']
        for low, high in zip(start[len(prefix):], end):
            tails = [t + chr(c) for t in tails for c in range(ord(low), ord(high) + 1)]
        out.extend(prefix + tail for tail in tails)
    return out


def validity(name):
    status = {}
    for node in ET.fromstring(fetch(f'validity/{name}.xml')).iter('id'):
        for code in expand(node.text or ''):
            status[code] = node.get('idStatus')
    return status


_aliases = {}


def aliases():
    if _aliases:
        return _aliases
    root = ET.fromstring(fetch('supplemental/supplementalMetadata.xml'))
    out = {'territories': {}, 'subdivisions': {}}
    for node in root.iter('territoryAlias'):
        out['territories'][node.get('type')] = node.get('replacement').split()
    for node in root.iter('subdivisionAlias'):
        out['subdivisions'][node.get('type')] = node.get('replacement').split()
    _aliases.update(out)
    return out


def parents():
    """The explicit parents of supplementalData.xml's main component."""
    out = {}
    root = ET.fromstring(fetch('supplemental/supplementalData.xml'))
    for group in root.iter('parentLocales'):
        if group.get('component'):
            continue
        for node in group.iter('parentLocale'):
            for locale in node.get('locales').split():
                out[locale] = node.get('parent')
    return out


def chain(name, explicit):
    """A file's parents, child first, root left out: the explicit parent,
    else the file with the last subtag cut (TR35 Part 1, "Parent
    Locales")."""
    out = [name]
    while True:
        here = out[-1]
        parent = explicit.get(here) or (here.rsplit('_', 1)[0] if '_' in here else 'root')
        if parent == 'root':
            return out
        out.append(parent)


# --- codes ------------------------------------------------------------------


def iso(kind, code):
    """A CLDR id in the form ISO writes it: `jp13` → `JP-13`, `JP` stays."""
    if kind == 'territories':
        return code
    match = re.fullmatch(r'([a-z]{2})([a-z0-9]{1,3})', code)
    assert match, f'{code}: not an ISO 3166-2 shape'
    return f'{match.group(1).upper()}-{match.group(2).upper()}'


# --- the tables -------------------------------------------------------------


def generate():
    explicit = parents()
    by_file = {name: chain(name, explicit) for _, name in LOCALES}
    files = sorted({f for c in by_file.values() for f in c})
    prefetch([f'main/{f}.xml' for f in files] + [f'subdivisions/{f}.xml' for f in files]
             + ['validity/region.xml', 'validity/subdivision.xml',
                'supplemental/supplementalMetadata.xml'])
    carried = {name: tag for tag, name in LOCALES}
    status = {'territories': validity('region'), 'subdivisions': validity('subdivision')}

    # Each table: its own files (up to the next carried file) and its parent tag.
    tables = []
    for tag, name in sorted(LOCALES):
        files_of = []
        parent = None
        for f in by_file[name]:
            if f != name and f in carried:
                parent = carried[f]
                break
            files_of.append(f)
        merged = {}
        for kind in KINDS:
            merged[kind] = {}
            for f in reversed(files_of):
                merged[kind].update(own_values(kind, f))
        tables.append({'tag': tag, 'files': files_of, 'parent': parent, 'values': merged})

    english = next(t for t in tables if t['tag'] == ENGLISH)
    codes = {}
    for kind in KINDS:
        named = set()
        for table in tables:
            named |= set(table['values'][kind])
        regular = {c for c, s in status[kind].items() if s == 'regular'}
        if kind == 'subdivisions':
            missing = regular - set(english['values'][kind])
            assert not missing, f'regular subdivisions English does not name: {sorted(missing)}'
        for code in named:
            assert code in status[kind], f'{kind} {code}: not in the validity data'
            # A deprecated code with an alias would want its replacement
            # beside it; CLDR 48 gives none for a code it names.
            assert status[kind][code] != 'deprecated' or code not in aliases()[kind], code
        codes[kind] = sorted(named, key=lambda c: iso(kind, c))
    return tables, codes, status


# --- writing Rust -----------------------------------------------------------


def lit(text):
    escaped = text.replace('\\', '\\\\').replace('"', '\\"').replace('\n', '\\n')
    return '"' + escaped + '"'


def bytes_lit(data):
    out = []
    for byte in data:
        char = chr(byte)
        if char in '"\\' or not 0x20 <= byte < 0x7f:
            out.append(f'\\x{byte:02x}')
        else:
            out.append(char)
    return 'b"' + ''.join(out) + '"'


def const_name(tag):
    return tag.replace('-', '_').upper()


def names_block(kind, table, codes, english):
    """A `Names` value: the bit set, the text as one literal a country (or,
    for territories, one a first character), the usual draft and the
    exceptions."""
    values = table['values'][kind]
    is_english = table['tag'] == ENGLISH
    bits = bytearray((len(codes) + 7) // 8)
    groups = collections.OrderedDict()
    drafts = []
    for index, code in enumerate(codes):
        if code not in values:
            continue
        bits[index // 8] |= 1 << (index % 8)
        text, draft = values[code]
        if not is_english and english.get(code, (None,))[0] == text:
            text = ''
        key = code[:2] if kind == 'subdivisions' else code[:1]
        groups.setdefault(key, []).append(text)
        drafts.append((index, draft))
    if not drafts:
        return 'Names::EMPTY', 0, 0
    usual = collections.Counter(d for _, d in drafts).most_common(1)[0][0]
    exceptions = [(i, d) for i, d in drafts if d != usual]
    while bits and bits[-1] == 0:
        bits.pop()
    lines = ['Names {', f'    named: {bytes_lit(bytes(bits))},']
    literals = [lit(''.join(t + '\n' for t in texts)) for texts in groups.values()]
    if len(literals) == 1:
        lines.append(f'    text: {literals[0]},')
    else:
        lines.append('    text: concat!(')
        lines.extend(f'        {literal},' for literal in literals)
        lines.append('    ),')
    lines.append(f'    draft: Draft::{usual.capitalize()},')
    if exceptions:
        items = ', '.join(f'({i}, Draft::{d.capitalize()})' for i, d in exceptions)
        lines.append(f'    exceptions: &[{items}],')
    else:
        lines.append('    exceptions: &[],')
    lines.append('}')
    text_bytes = sum(len(t.encode()) + 1 for texts in groups.values() for t in texts)
    return '\n'.join(lines), text_bytes, len(bits)


def codes_rs(kind, codes, status, width):
    name = 'TERRITORY' if kind == 'territories' else 'SUBDIVISION'
    noun = 'territory' if kind == 'territories' else 'subdivision'
    isos = [iso(kind, c) for c in codes]
    assert all(len(i) <= width for i in isos)
    assert isos == sorted(isos)
    out = []
    out.append(f'/// The code of every {noun} a table can name, in the form ISO writes it, each')
    out.append(f'/// padded with spaces to {width} bytes, in code order.')
    out.append(f'pub(super) const {name}_CODES: &str = concat!(')
    for start in range(0, len(isos), 12):
        chunk = ''.join(i.ljust(width) for i in isos[start:start + 12])
        out.append(f'    {lit(chunk)},')
    out.append(');')
    # `Status::from_byte` reads these; a status CLDR adds stops the script.
    letters = {'regular': 'r', 'deprecated': 'd', 'macroregion': 'm', 'special': 's',
               'unknown': 'u'}
    marks = ''.join(letters[status[c]] for c in codes)
    out.append('')
    out.append(f'/// Each code\'s CLDR validity status, a byte a code in [`{name}_CODES`] order;')
    out.append('/// `Status::from_byte` reads it.')
    out.append(f'pub(super) const {name}_STATUS: &[u8] = concat!(')
    for start in range(0, len(marks), 96):
        out.append(f'    {lit(marks[start:start + 96])},')
    out.append(').as_bytes();')
    return out


HEADER = '''\
//! The names of the territories and the subdivisions, generated.
//!
//! **Do not edit.** `scripts/place-names-cldr.py` writes this file from
//! Unicode CLDR 48 (`release-48`): `common/main/<file>.xml` for the
//! territories, `common/subdivisions/<file>.xml` for the subdivisions and
//! `common/validity/` for each code's status. `super` says how a table is
//! read; `python3 scripts/place-names-cldr.py --dump` prints every value
//! beside its code.

use super::{Draft, Names, Table};

'''


def render():
    tables, codes, status = generate()
    english = next(t for t in tables if t['tag'] == ENGLISH)['values']
    out = []
    out.extend(codes_rs('territories', codes['territories'], status['territories'], 3))
    out.append('')
    out.extend(codes_rs('subdivisions', codes['subdivisions'], status['subdivisions'], 6))
    out.append('')
    stats = []
    consts = []
    for table in tables:
        blocks = {}
        sizes = {}
        for kind in KINDS:
            block, text_bytes, bit_bytes = names_block(kind, table, codes[kind], english[kind])
            blocks[kind] = block
            sizes[kind] = (text_bytes, bit_bytes, len(table['values'][kind]))
        stats.append((table['tag'], sizes))
        if all(b == 'Names::EMPTY' for b in blocks.values()):
            continue
        name = const_name(table['tag'])
        files = ', '.join(f'`{f}.xml`' for f in table['files'])
        parent = f', then the `{table["parent"]}` table' if table['parent'] else ''
        consts.append(f'/// `{table["tag"]}`: {files}{parent}.')
        consts.append(f'const {name}: Table = Table {{')
        consts.append(f'    tag: {lit(table["tag"])},')
        consts.append('    parent: ' + (f'Some({lit(table["parent"])})' if table['parent'] else 'None')
                      + ',')
        for kind in KINDS:
            consts.append(f'    {kind}: ' + textwrap.indent(blocks[kind], '    ').lstrip() + ',')
        consts.append('};')
        consts.append('')
        table['emitted'] = True
    out.append('/// Every carried locale whose files name a territory or a subdivision, in tag')
    out.append('/// order.')
    out.append('pub(super) static TABLES: &[Table] = &[')
    for table in tables:
        if table.get('emitted'):
            out.append(f'    {const_name(table["tag"])},')
    out.append('];')
    out.append('')
    out.extend(consts)
    dump = []
    for table in tables:
        for kind in KINDS:
            for code in codes[kind]:
                if code in table['values'][kind]:
                    text, draft = table['values'][kind][code]
                    dump.append(f'{kind}\t{iso(kind, code)}\t{table["tag"]}\t{text}\t{draft}')
    return HEADER + '\n'.join(out).rstrip('\n') + '\n', dump, stats, codes


def rustfmt(text):
    # From the repository root, so that rustfmt reads its rustfmt.toml.
    result = subprocess.run(['rustfmt', '--edition', '2024', '--emit', 'stdout'],
                            input=text.encode(), capture_output=True, check=True, cwd=ROOT_DIR)
    return result.stdout.decode()


def main():
    text, dump, stats, codes = render()
    if '--dump' in sys.argv:
        print('\n'.join(dump))
        return
    if '--stats' in sys.argv:
        total = collections.Counter()
        for tag, sizes in stats:
            cells = []
            for kind in KINDS:
                text_bytes, bit_bytes, count = sizes[kind]
                total[kind + ' text'] += text_bytes
                total[kind + ' bits'] += bit_bytes
                total[kind + ' names'] += count
                cells.append(f'{count}\t{text_bytes}\t{bit_bytes}')
            print(f'{tag}\t' + '\t'.join(cells))
        for kind in KINDS:
            print(f'{kind}: {len(codes[kind])} codes, {total[kind + " names"]} names, '
                  f'{total[kind + " text"]} bytes of text, {total[kind + " bits"]} of bits')
        return
    text = rustfmt(text)
    if '--check' in sys.argv:
        with open(OUTPUT, encoding='utf-8') as handle:
            if handle.read() != text:
                print(f'{OUTPUT} is stale: run scripts/place-names-cldr.py', file=sys.stderr)
                sys.exit(1)
        return
    os.makedirs(os.path.dirname(OUTPUT), exist_ok=True)
    with open(OUTPUT, 'w', encoding='utf-8') as handle:
        handle.write(text)


if __name__ == '__main__':
    main()
