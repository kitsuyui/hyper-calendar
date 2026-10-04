#!/usr/bin/env python3
"""Regenerate crates/hc-i18n/src/place_names/cldr48.rs from Unicode CLDR 48.

What every locale hc-i18n carries calls each territory and each ISO 3166-2
subdivision CLDR 48 names, for hc-i18n's `place_names` module, which is the
one source of territory names in the workspace: the holiday tables' country
names and the zone names' countries are read from it too.

    python3 scripts/place-names-cldr.py            # rewrite the file
    python3 scripts/place-names-cldr.py --check    # exit 1 if it is stale
    python3 scripts/place-names-cldr.py --dump     # print every value, a TSV
    python3 scripts/place-names-cldr.py --stats    # print the sizes
    python3 scripts/place-names-cldr.py --fallbacks  # print each table's
                                                   # language-matching fallbacks

The files are the `release-48` tag of unicode-org/cldr, read over HTTP from
raw.githubusercontent.com into memory; nothing is saved:

* `common/main/<file>.xml`, `localeDisplayNames/territories/territory`,
  for the territory names, the plain value and each `alt` form (`short`,
  `variant`, `biot`, `chagos`);
* `common/subdivisions/<file>.xml`, `localeDisplayNames/subdivisions/
  subdivision`, for the subdivision names;
* `common/validity/region.xml` and `common/validity/subdivision.xml` for
  each code's status (`regular`, `deprecated`, `macroregion` ...);
* `common/supplemental/supplementalMetadata.xml`, `territoryAlias` and
  `subdivisionAlias`, to check that no deprecated code a file names has a
  replacement (in CLDR 48 the aliases of those codes are commented out);
* `common/supplemental/supplementalData.xml`, `parentLocales`, for each
  file's parent, and `territoryContainment`, for the regions a matching
  variable such as `$americas` (`019`) stands for;
* `common/supplemental/subdivisions.xml`, `subdivisionContainment`, for
  which subdivision lies in which;
* `common/supplemental/languageInfo.xml`, `languageMatching`, and
  `common/supplemental/likelySubtags.xml`, for each table's fallback
  locales (below).

A value is one at the draft levels `approved`, `contributed` or
`provisional`: TR35 Part 1, "Attribute draft", lets an implementation
"accept the provisional data, especially if there is no translated
alternative", and all but three of a locale's subdivision names are
provisional. Each value keeps its level, so that a caller that wants only
CLDR's release levels, as the holiday tables do, can ask for those. An
`unconfirmed` value, and CLDR's inheritance marker `↑↑↑`, are absent, so
that the lookup goes on to the parent.

A table holds the values of one carried locale's own files: its file, and
the files between it and the next file that is itself a carried locale or
root. Each table names its parent table, if any, so that the runtime walks
CLDR's chain (TR35 Part 1, "Inherited item lookup"); root names nothing
here, and after the chain the lookup takes English, `en.xml`, as the
fallback locale TR35's "Lookup" section allows before root.

Before English, a table lists the carried locales language matching makes
its fallbacks (TR35 Part 1, "Language Matching": "The language matching data
can be used to get the closest fallback locales (of those supported) to a
given locale"). The distance between two locales is TR35's: each is
maximized with the likely subtags, and for the language, the script and
the region in turn, where the two differ, the first rule of
`languageMatching` that matches the pair, one way or, unless it is
`oneway`, either way, adds its distance. TR35 leaves the threshold to the
implementation, "typically set to greater than a default region
difference, and less than a default script difference"; the region
default here is 4 (`*_*_*`) and the script default 50 (`*_*`), and a
locale below 50 is a fallback, nearest first. A matching variable's
regions are expanded through `territoryContainment`, groupings and
macroregions included, so that `419` is among `$americas`. Tibetan's are
the Simplified Chinese table (`bo` ⇒ `zh`, distance 20, then `bo_Tibt` ⇒
`zh_Hans`, 10), for example; `zh-Hant`'s is `zh-Hant-HK` only, since the
two scripts of Chinese are the script default apart.

Only the names that differ from English are stored: a table has a bit per
code saying whether its files name the code, and one line of text per
named code, empty where the name is English's. The `alt` forms are few, and
a table lists them by index.

CLDR writes a subdivision as a region code and a suffix in lower case with
no hyphen, `jp13`; ISO 3166-2 writes `JP-13`. The module keeps the ISO
form: the first two letters in upper case, a hyphen, and the rest in upper
case. Every id CLDR 48 names has that shape (a region of two letters and a
suffix of one to three letters or digits); an id of CLDR's own, with a
four-character suffix or a numeric region, would stop the script.

The subdivisions' containment and names are written under
`#[cfg(feature = "place-names")]`, and their codes and statuses under
`#[cfg(any(feature = "place-names", feature = "subdivision-codes"))]`, so
that a build with only the `territories` feature carries the territories
alone and one with `subdivision-codes` carries the codes without a name.
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
    ('aeb-Latn', 'aeb_Latn'), ('am', 'am'), ('ar', 'ar'), ('ar-EG', 'ar_EG'),
    ('ayl-Latn', 'ayl_Latn'), ('ban', 'ban'), ('bn', 'bn'), ('bo', 'bo'), ('cop', 'cop'),
    ('cs', 'cs'), ('de', 'de'), ('en', 'en'), ('en-001', 'en_001'), ('en-GB', 'en_GB'),
    ('es', 'es'), ('es-419', 'es_419'), ('fa', 'fa'), ('fil', 'fil'), ('fr', 'fr'),
    ('ha', 'ha'), ('he', 'he'), ('hi', 'hi'), ('id', 'id'), ('it', 'it'), ('ja', 'ja'),
    ('jv', 'jv'), ('kab', 'kab'), ('ko', 'ko'), ('mid', 'mid'), ('mix', 'mix'), ('ml', 'ml'),
    ('mn', 'mn'), ('mr', 'mr'), ('my', 'my'), ('nah', 'nah'), ('ne', 'ne'), ('nl', 'nl'),
    ('pa-Arab', 'pa_Arab'), ('pa-Guru', 'pa'), ('pcm', 'pcm'), ('pl', 'pl'), ('ps', 'ps'),
    ('pt', 'pt'), ('pt-PT', 'pt_PT'), ('rif', 'rif'), ('ru', 'ru'), ('sa', 'sa'),
    ('shi-Latn', 'shi_Latn'), ('sw', 'sw'), ('syr', 'syr'), ('ta', 'ta'), ('te', 'te'),
    ('th', 'th'), ('tr', 'tr'), ('ur', 'ur'), ('ur-IN', 'ur_IN'), ('vi', 'vi'), ('yua', 'yua'),
    ('yue-Hans', 'yue_Hans'), ('yue-Hant', 'yue'), ('zap', 'zap'), ('zgh', 'zgh'),
    ('zh-Hans', 'zh'), ('zh-Hant', 'zh_Hant'), ('zh-Hant-HK', 'zh_Hant_HK'),
]
ENGLISH = 'en'
ACCEPTED = {'approved', 'contributed', 'provisional'}
MARK = '↑↑↑'
DRAFTS = ['approved', 'contributed', 'provisional']
KINDS = ['territories', 'subdivisions']
# The `alt` forms of a territory name, in the order the Rust enum lists them.
ALTS = ['short', 'variant', 'biot', 'chagos']
# TR35's threshold lies between the default region and script differences.
THRESHOLD = 50

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
    """Every value of `element` in a file, at the accepted levels, the
    marker left out: the plain values, type → (text, draft), and each `alt`
    form's, alt → type → (text, draft). The draft of an element is its own,
    else its nearest ancestor's, else `approved`."""
    data = fetch(path)
    if data is None:
        return {}, {}
    root = ET.fromstring(data)
    out = {}
    alts = {}

    def walk(node, draft):
        draft = node.get('draft', draft)
        if node.tag == element:
            text = (node.text or '').strip()
            assert text != '∅∅∅', ("CLDR's empty override ∅∅∅ at a path this script reads: it has no encoding for it yet (scripts/cldr_xml.py's resolve stops the lookup there)", path, node.get('type'))
            alt = node.get('alt')
            if draft in ACCEPTED and text and text != MARK:
                assert alt is None or alt in ALTS, (path, alt)
                into = out if alt is None else alts.setdefault(alt, {})
                assert node.get('type') not in into, (path, node.get('type'), alt)
                assert '\t' not in text and '\n' not in text, (path, text)
                into[node.get('type')] = (text, draft)
            return
        for child in node:
            if isinstance(child.tag, str):
                walk(child, draft)

    walk(root, 'approved')
    return out, alts


def own_values(kind, name):
    if kind == 'territories':
        return values(f'main/{name}.xml', 'territory')
    plain, alts = values(f'subdivisions/{name}.xml', 'subdivision')
    assert not alts, f'subdivisions/{name}.xml: an alt form, which the tables have no room for'
    return plain, alts


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


def containment():
    """supplementalData.xml's territoryContainment: region → the regions it
    contains, groupings and deprecated groups included."""
    out = collections.defaultdict(set)
    root = ET.fromstring(fetch('supplemental/supplementalData.xml'))
    for node in root.iter('group'):
        if node.get('status') == 'deprecated':
            continue
        out[node.get('type')].update(node.get('contains').split())
    return out


def within(region, contains):
    """A region and every region it contains, recursively."""
    out = {region}
    for inner in contains.get(region, ()):
        out |= within(inner, contains)
    return out


# --- language matching (TR35 Part 1, "Language Matching") --------------------


def likely():
    root = ET.fromstring(fetch('supplemental/likelySubtags.xml'))
    return {node.get('from'): node.get('to') for node in root.iter('likelySubtag')}


def split(name):
    """A CLDR locale name's language, script and region, '' where absent."""
    parts = name.split('_')
    language, script, region = parts[0], '', ''
    for part in parts[1:]:
        if len(part) == 4:
            script = part
        else:
            region = part
    return language, script, region


def maximize(name, table):
    """TR35's "Add Likely Subtags": the first of language_script_region,
    language_region, language_script and language the table has fills the
    subtags the name lacks."""
    language, script, region = split(name)
    keys = []
    if script and region:
        keys.append(f'{language}_{script}_{region}')
    if region:
        keys.append(f'{language}_{region}')
    if script:
        keys.append(f'{language}_{script}')
    keys.append(language)
    for key in keys:
        if key in table:
            _, likely_script, likely_region = split(table[key])
            return language, script or likely_script, region or likely_region
    return language, script, region


def matching():
    """languageInfo.xml's rules in order, and its variables' regions."""
    root = ET.fromstring(fetch('supplemental/languageInfo.xml'))
    groups = [g for g in root.iter('languageMatches') if g.get('type') == 'written_new']
    assert len(groups) == 1, 'one written_new languageMatches'
    contains = containment()
    variables = {}
    for node in groups[0].iter('matchVariable'):
        regions = set()
        for sign, code in re.findall(r'([+-]?)([A-Z0-9]+)', node.get('value')):
            if sign == '-':
                regions -= within(code, contains)
            else:
                regions |= within(code, contains)
        variables[node.get('id')] = regions
    rules = []
    for node in groups[0].iter('languageMatch'):
        rules.append((node.get('desired').split('_'), node.get('supported').split('_'),
                      int(node.get('distance')), node.get('oneway') == 'true'))
    return rules, variables


def field_matches(pattern, value, variables):
    if pattern == '*':
        return True
    if pattern.startswith('$!'):
        return value not in variables['$' + pattern[2:]]
    if pattern.startswith('$'):
        return value in variables[pattern]
    return pattern == value


def distance(desired, supported, rules, variables):
    """TR35's matching distance between two maximized locales."""
    total = 0
    for level in (1, 2, 3):
        if desired[level - 1] == supported[level - 1]:
            continue
        wanted, offered = desired[:level], supported[:level]

        def fits(rule_desired, rule_supported, d, s):
            return all(field_matches(p, v, variables) for p, v in zip(rule_desired, d)) and all(
                field_matches(p, v, variables) for p, v in zip(rule_supported, s))

        for rule_desired, rule_supported, value, oneway in rules:
            if len(rule_desired) != level:
                continue
            if fits(rule_desired, rule_supported, wanted, offered) or (
                    not oneway and fits(rule_desired, rule_supported, offered, wanted)):
                total += value
                break
        else:
            raise AssertionError(f'no rule for {wanted} and {offered}')
    return total


def fallbacks(tables):
    """Each table's fallback tables, nearest first, ties in tag order: the
    carried locales with a table whose distance from it is below THRESHOLD,
    leaving out itself and its own parents. The list ends before English,
    which the lookup takes last in any case, as TR35's example list ends in
    `en`: a locale after English would be read only for a code English
    does not name."""
    table_of_likely = likely()
    rules, variables = matching()
    names = {t['tag']: t['name'] for t in tables}
    maximized = {tag: maximize(name, table_of_likely) for tag, name in names.items()}
    parents_of = {t['tag']: t['parent'] for t in tables}
    out = {}
    for table in tables:
        own = set()
        here = table['tag']
        while here:
            own.add(here)
            here = parents_of[here]
        found = []
        for other in tables:
            tag = other['tag']
            if tag in own or not other['has_data']:
                continue
            gap = distance(maximized[table['tag']], maximized[tag], rules, variables)
            if gap < THRESHOLD:
                found.append((gap, tag))
        found.sort()
        tags = [tag for _, tag in found]
        if ENGLISH in tags:
            found = found[:tags.index(ENGLISH)]
        out[table['tag']] = found
    return out, maximized


def subdivision_containment():
    """subdivisions.xml: each contained subdivision → its container, a
    country's code in upper case or a subdivision's id."""
    root = ET.fromstring(fetch('supplemental/subdivisions.xml'))
    out = {}
    for node in root.iter('subgroup'):
        for code in node.get('contains').split():
            assert code not in out, code
            out[code] = node.get('type')
    return out


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
                'supplemental/supplementalMetadata.xml', 'supplemental/supplementalData.xml',
                'supplemental/subdivisions.xml', 'supplemental/languageInfo.xml',
                'supplemental/likelySubtags.xml'])
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
        alts = {}
        for kind in KINDS:
            merged[kind] = {}
            for f in reversed(files_of):
                plain, alt_values = own_values(kind, f)
                merged[kind].update(plain)
                if kind == 'territories':
                    for alt, found in alt_values.items():
                        alts.setdefault(alt, {}).update(found)
        has_data = any(merged[kind] for kind in KINDS) or any(alts.values())
        tables.append({'tag': tag, 'name': name, 'files': files_of, 'parent': parent,
                       'values': merged, 'alts': alts, 'has_data': has_data})

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
    for table in tables:
        for alt, found in table['alts'].items():
            for code in found:
                assert code in codes['territories'], (table['tag'], alt, code)
        assert table['parent'] is None or next(
            t for t in tables if t['tag'] == table['parent'])['has_data'], table['tag']
    near, maximized = fallbacks(tables)
    for table in tables:
        table['fallbacks'] = near[table['tag']]
        table['maximized'] = maximized[table['tag']]
        table['emitted'] = table['has_data'] or bool(table['fallbacks'])

    # Containment: every contained code a carried locale names, in its
    # container's country. The others are deprecated codes no file names
    # (CLDR 48 lists 19, `usgu` Guam among them), which are not carried.
    contained = subdivision_containment()
    index = {code: i for i, code in enumerate(codes['subdivisions'])}
    top, nested = [], []
    unnamed = [code for code in contained if code not in index]
    assert all(status['subdivisions'].get(code) == 'deprecated' for code in unnamed), unnamed
    for code, container in sorted(((c, p) for c, p in contained.items() if c in index),
                                  key=lambda item: index[item[0]]):
        if container.isupper():
            assert code[:2] == container.lower(), (code, container)
            top.append(index[code])
        else:
            assert container in index, container
            assert code[:2] == container[:2], (code, container)
            nested.append((index[code], index[container]))
    return tables, codes, status, (top, nested, unnamed)


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


CFG = '#[cfg(feature = "place-names")]'
# The codes and their validity statuses are carried by the `subdivision-codes`
# feature too, which holds no name.
CODES_CFG = '#[cfg(any(feature = "place-names", feature = "subdivision-codes"))]'


def codes_rs(kind, codes, status, width):
    name = 'TERRITORY' if kind == 'territories' else 'SUBDIVISION'
    noun = 'territory' if kind == 'territories' else 'subdivision'
    cfg = [CODES_CFG] if kind == 'subdivisions' else []
    isos = [iso(kind, c) for c in codes]
    assert all(len(i) <= width for i in isos)
    assert isos == sorted(isos)
    out = []
    out.append(f'/// The code of every {noun} a table can name, in the form ISO writes it, each')
    out.append(f'/// padded with spaces to {width} bytes, in code order.')
    out.extend(cfg)
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
    out.extend(cfg)
    out.append(f'pub(super) const {name}_STATUS: &[u8] = concat!(')
    for start in range(0, len(marks), 96):
        out.append(f'    {lit(marks[start:start + 96])},')
    out.append(').as_bytes();')
    return out


def containment_rs(codes, containment):
    top, nested, unnamed = containment
    bits = bytearray((len(codes) + 7) // 8)
    for index in top:
        bits[index // 8] |= 1 << (index % 8)
    while bits and bits[-1] == 0:
        bits.pop()
    out = []
    out.append('/// A bit per code of [`SUBDIVISION_CODES`], least significant first, set where')
    out.append("/// `subdivisions.xml` lists the subdivision directly within its country:")
    out.append(f'/// {len(top)} of them.')
    out.append(CFG)
    out.append(f'pub(super) const SUBDIVISION_IN_COUNTRY: &[u8] = {bytes_lit(bytes(bits))};')
    out.append('')
    out.append('/// Each subdivision `subdivisions.xml` lists within another, as its index and')
    out.append(f'/// the index of the one it lies in, in index order: {len(nested)} of them. The')
    out.append(f'/// {len(unnamed)} deprecated codes the file lists and no carried locale names are')
    out.append('/// left out.')
    out.append(CFG)
    out.append('pub(super) const SUBDIVISION_WITHIN: &[(u16, u16)] = &[')
    out.append('    ' + ', '.join(f'({child}, {parent})' for child, parent in nested) + ',')
    out.append('];')
    return out


def alternatives_rs(table, codes):
    items = []
    index = {code: i for i, code in enumerate(codes)}
    for alt in ALTS:
        for code, (text, draft) in table['alts'].get(alt, {}).items():
            items.append((index[code], ALTS.index(alt), alt, text, draft))
    items.sort()
    if not items:
        return '&[]'
    return '&[\n' + ''.join(
        f'        ({i}, Alt::{alt.capitalize()}, {lit(text)}, Draft::{draft.capitalize()}),\n'
        for i, _, alt, text, draft in items) + '    ]'


HEADER = """\
//! The names of the territories and the subdivisions, generated.
//!
//! **Do not edit.** `scripts/place-names-cldr.py` writes this file from
//! Unicode CLDR 48 (`release-48`): `common/main/<file>.xml` for the
//! territories, `common/subdivisions/<file>.xml` for the subdivisions,
//! `common/validity/` for each code's status, `supplemental/subdivisions.xml`
//! for their containment and `supplemental/languageInfo.xml` for each
//! table's fallbacks. `super` says how a table is read;
//! `python3 scripts/place-names-cldr.py --dump` prints every value beside
//! its code.

use super::{Alt, Draft, Names, Table};

"""


def render():
    tables, codes, status, containment = generate()
    english = next(t for t in tables if t['tag'] == ENGLISH)['values']
    out = []
    out.extend(codes_rs('territories', codes['territories'], status['territories'], 3))
    out.append('')
    out.extend(codes_rs('subdivisions', codes['subdivisions'], status['subdivisions'], 6))
    out.append('')
    out.extend(containment_rs(codes['subdivisions'], containment))
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
        if not table['emitted']:
            continue
        name = const_name(table['tag'])
        files = ', '.join(f'`{f}.xml`' for f in table['files'])
        parent = f', then the `{table["parent"]}` table' if table['parent'] else ''
        near = ', '.join(f'`{tag}` ({gap})' for gap, tag in table['fallbacks'])
        near = f'; language matching\n/// falls back to {near}' if near else ''
        consts.append(f'/// `{table["tag"]}`: {files}{parent}{near}.')
        consts.append(f'const {name}: Table = Table {{')
        consts.append(f'    tag: {lit(table["tag"])},')
        consts.append('    parent: ' + (f'Some({lit(table["parent"])})' if table['parent'] else 'None')
                      + ',')
        consts.append('    fallbacks: &[' + ', '.join(lit(tag) for _, tag in table['fallbacks'])
                      + '],')
        consts.append('    territories: ' + textwrap.indent(blocks['territories'], '    ').lstrip()
                      + ',')
        consts.append('    alternatives: ' + alternatives_rs(table, codes['territories']) + ',')
        consts.append('    ' + CFG)
        consts.append('    subdivisions: ' + textwrap.indent(blocks['subdivisions'], '    ').lstrip()
                      + ',')
        consts.append('};')
        consts.append('')
    out.append('/// Every carried locale whose files name a territory or a subdivision, or')
    out.append('/// which language matching gives a fallback, in tag order.')
    out.append('pub(super) static TABLES: &[Table] = &[')
    for table in tables:
        if table['emitted']:
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
        for alt in ALTS:
            for code in codes['territories']:
                if code in table['alts'].get(alt, {}):
                    text, draft = table['alts'][alt][code]
                    dump.append(f'territories/{alt}\t{code}\t{table["tag"]}\t{text}\t{draft}')
    return HEADER + '\n'.join(out).rstrip('\n') + '\n', dump, stats, codes, tables


def rustfmt(text):
    # From the repository root, so that rustfmt reads its rustfmt.toml.
    result = subprocess.run(['rustfmt', '--edition', '2024', '--emit', 'stdout'],
                            input=text.encode(), capture_output=True, check=True, cwd=ROOT_DIR)
    return result.stdout.decode()


def main():
    text, dump, stats, codes, tables = render()
    if '--dump' in sys.argv:
        print('\n'.join(dump))
        return
    if '--fallbacks' in sys.argv:
        for table in tables:
            near = ' '.join(f'{tag}:{gap}' for gap, tag in table['fallbacks'])
            print(f"{table['tag']}\t{'_'.join(filter(None, table['maximized']))}\t{near}")
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
        alts = sum(len(v) for t in tables for v in t['alts'].values())
        print(f'alternatives: {alts} names')
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
