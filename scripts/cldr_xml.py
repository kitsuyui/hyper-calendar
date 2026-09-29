"""Reading and resolving Unicode CLDR 48's XML, for the generators that share it.

`scripts/locales-cldr.py`, `scripts/zone-names-cldr.py` and
`scripts/day-periods-cldr.py` import this module. The files are the
`release-48` tag of unicode-org/cldr, `common/<path>`, read over HTTP from
raw.githubusercontent.com into memory; nothing is saved. With CLDR_DIR set to
a checkout of that tag, a file present under it is read from there instead.

The resolution, for one value at one path, is UTS #35's (Part 1,
"Inheritance and Validity"): the locale's own files, child first, then
`root.xml`; a value at a draft level below `contributed` is absent, and so is
the inheritance marker `↑↑↑`, which "conformant implementations" read as the
element being absent; where no file states the path, the longest prefix
`root.xml` aliases is rewritten through the alias and looked up again from the
child. The empty override `∅∅∅` (Part 1, "Empty Override") is the other
reserved value: it says that the locale "is to have no value for a path, even
if the parent locale has a value for that path", so the lookup stops there
with no value, and a formatter takes the field's own fallback, as UTS #35
Part 4's zone names fall back to the localized GMT format. `resolve` gives
no value there, and `carried` gives `NO_VALUE` where the entry must say so
to stop its lookup from inheriting a value.
"""
import http.client
import os
import re
import subprocess
import urllib.error
import urllib.request
import xml.etree.ElementTree as ET

BASE = 'https://raw.githubusercontent.com/unicode-org/cldr/release-48/common/'
RELEASE = {None, 'approved', 'contributed'}
MARK = '↑↑↑'
EMPTY = '∅∅∅'


class _NoValue:
    """What `carried` gives where an entry's files write the empty override
    over a value its lookup would otherwise inherit."""

    def __repr__(self):
        return 'NO_VALUE'

    def __bool__(self):
        return True


NO_VALUE = _NoValue()
DISTINGUISHING = {'type', 'alt', 'id', 'count', 'key', 'yeartype', 'numberSystem', 'scope',
                  'menu', 'case', 'gender'}

_texts = {}


def fetch(path):
    """The bytes of `common/<path>`."""
    if path in _texts:
        return _texts[path]
    local = os.environ.get('CLDR_DIR')
    if local:
        full = os.path.join(local, 'common', path)
        if os.path.exists(full):
            with open(full, 'rb') as handle:
                _texts[path] = handle.read()
            return _texts[path]
    for attempt in range(4):
        try:
            with urllib.request.urlopen(BASE + path, timeout=60) as response:
                _texts[path] = response.read()
                break
        except urllib.error.HTTPError as error:
            if error.code == 404 or attempt == 3:
                raise
        except (OSError, http.client.HTTPException):
            if attempt == 3:
                raise
    return _texts[path]


def exists(path):
    """Whether `common/<path>` is a file of the release: a 404 is an
    answer, not a failure to retry."""
    if path in _texts:
        return True
    local = os.environ.get('CLDR_DIR')
    if local and os.path.exists(os.path.join(local, 'common', path)):
        return True
    try:
        fetch(path)
    except urllib.error.HTTPError as error:
        if error.code == 404:
            return False
        raise
    return True


def xml(path):
    return ET.fromstring(fetch(path))


def segment(element):
    attrs = tuple(sorted((k, v) for k, v in element.attrib.items() if k in DISTINGUISHING))
    return (element.tag, attrs)


_files = {}


def load(name):
    """The leaves of `main/<name>.xml`, path → (value, draft), and its
    aliases, path → alias path."""
    if name in _files:
        return _files[name]
    root = xml(f'main/{name}.xml')
    leaves, aliases = {}, {}

    def walk(element, here):
        for child in element:
            if not isinstance(child.tag, str):
                continue
            there = here + (segment(child),)
            if child.tag == 'alias':
                aliases[here] = child.get('path')
            elif not any(isinstance(k.tag, str) for k in child):
                leaves[there] = ((child.text or '').strip(), child.get('draft'))
            else:
                walk(child, there)

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
        out.append((match.group(1), attrs))
    return tuple(out)


def path(*parts):
    """A path from `tag`, `tag[type]` or (`tag`, {attrs}) parts."""
    out = []
    for part in parts:
        if isinstance(part, str):
            if '[' in part:
                tag, value = part[:-1].split('[', 1)
                out.append((tag, (('type', value),)))
            else:
                out.append((part, ()))
        else:
            tag, attrs = part
            out.append((tag, tuple(sorted(attrs.items()))))
    return tuple(out)


def resolve(chain, at, depth=0):
    """(value, file): the value CLDR resolves `at` to through the chain
    (child first, root left out) and root's aliases; (None, file) where
    that file writes the empty override `∅∅∅`, which stops the lookup
    with no value; (None, None) where no file gives one."""
    for name in chain + ['root']:
        leaves, _ = load(name)
        if at in leaves:
            value, draft = leaves[at]
            if draft not in RELEASE or value == MARK:
                continue
            if value == EMPTY:
                return None, name
            return value, name
    _, aliases = load('root')
    for n in range(len(at), 0, -1):
        prefix = at[:n]
        if prefix in aliases and depth < 20:
            return resolve(chain, through_alias(prefix, aliases[prefix]) + at[n:], depth + 1)
    return None, None


def stated(chain, at):
    """Whether a file of the chain writes `at` at a release level, a value or
    the marker, directly or through root's aliases."""
    for name in chain:
        leaves, _ = load(name)
        if at in leaves and leaves[at][1] in RELEASE:
            return True
    _, aliases = load('root')
    for n in range(len(at), 0, -1):
        prefix = at[:n]
        if prefix in aliases:
            return stated(chain, through_alias(prefix, aliases[prefix]) + at[n:])
    return False


def lit(text):
    """A Rust string literal."""
    return '"' + text.replace('\\', '\\\\').replace('"', '\\"') + '"'


def rustfmt(text):
    result = subprocess.run(['rustfmt', '--edition', '2024', '--emit', 'stdout'],
                            input=text.encode(), capture_output=True, check=True)
    return result.stdout.decode()


def write_or_check(output, text, script):
    """Write the generated file, or with `--check` exit 1 if it is stale."""
    import sys
    if '--check' in sys.argv:
        with open(output, encoding='utf-8') as handle:
            if handle.read() != text:
                print(f'{output} is stale: run {script}', file=sys.stderr)
                sys.exit(1)
        return
    os.makedirs(os.path.dirname(output), exist_ok=True)
    with open(output, 'w', encoding='utf-8') as handle:
        handle.write(text)


# Every hc-i18n entry with a CLDR file: its tag, its files child first (root
# left out), and whether it is a regional entry, which carries only what its
# files resolve apart from its parent (its files less the first).
ENTRIES = [
    ('am', ['am'], False), ('ar', ['ar'], False), ('ar-EG', ['ar_EG', 'ar'], True),
    ('bn', ['bn'], False), ('bo', ['bo'], False), ('cs', ['cs'], False), ('de', ['de'], False),
    ('en', ['en'], False), ('en-001', ['en_001', 'en'], True),
    ('en-GB', ['en_GB', 'en_001', 'en'], True), ('es', ['es'], False),
    ('es-419', ['es_419', 'es'], True), ('fa', ['fa'], False), ('fil', ['fil'], False),
    ('fr', ['fr'], False), ('ha', ['ha'], False), ('he', ['he'], False), ('hi', ['hi'], False),
    ('id', ['id'], False), ('it', ['it'], False), ('ja', ['ja'], False), ('jv', ['jv'], False),
    ('kab', ['kab'], False), ('ko', ['ko'], False), ('ml', ['ml'], False), ('mn', ['mn'], False),
    ('mr', ['mr'], False), ('my', ['my'], False), ('ne', ['ne'], False), ('nl', ['nl'], False),
    ('pa-Arab', ['pa_Arab'], False), ('pa-Guru', ['pa'], False), ('pcm', ['pcm'], False),
    ('pl', ['pl'], False), ('ps', ['ps'], False), ('pt', ['pt'], False),
    ('pt-PT', ['pt_PT', 'pt'], True), ('ru', ['ru'], False), ('sa', ['sa'], False),
    ('sw', ['sw'], False), ('syr', ['syr'], False), ('ta', ['ta'], False), ('te', ['te'], False),
    ('th', ['th'], False), ('tr', ['tr'], False), ('ur', ['ur'], False),
    ('ur-IN', ['ur_IN', 'ur'], True), ('vi', ['vi'], False), ('yue-Hans', ['yue_Hans'], False),
    ('yue-Hant', ['yue'], False), ('zgh', ['zgh'], False), ('zh-Hans', ['zh'], False),
    ('zh-Hant', ['zh_Hant'], False), ('zh-Hant-HK', ['zh_Hant_HK', 'zh_Hant'], True),
]


def carried(chain, regional, at):
    """The value an entry carries at `at`: for a language, the resolved value
    where its own file states the path; for a regional entry, the resolved
    value where it differs from its parent's. `NO_VALUE` where a file of the
    entry's own writes the empty override and the entry's parent (root, for
    a language) resolves a value it would otherwise inherit; None
    otherwise."""
    value, name = resolve(chain, at)
    if value is None and name is not None and name != 'root':
        own = chain[:1] if regional else chain
        if name not in own:
            return None
        return NO_VALUE if resolve(chain[1:] if regional else [], at)[0] is not None else None
    if value is None or name == 'root':
        return None
    if regional:
        return value if resolve(chain[1:], at)[0] != value else None
    return value if name in chain else None
