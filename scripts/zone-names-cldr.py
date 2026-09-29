#!/usr/bin/env python3
"""Regenerate crates/hc-i18n/src/zone_names/cldr48.rs from Unicode CLDR 48.

What a locale calls a time zone, and the supplemental data UTS #35 Part 4
("Using Time Zone Names") composes the names with:

    python3 scripts/zone-names-cldr.py            # rewrite the file
    python3 scripts/zone-names-cldr.py --check    # exit 1 if it is stale
    python3 scripts/zone-names-cldr.py --dump     # print every value, a TSV

The files, read through scripts/cldr_xml.py (the `release-48` tag, over HTTP
into memory, nothing saved):

* `common/main/<file>.xml`, `dates/timeZoneNames`, of every hc-i18n entry
  with a CLDR file: the formats (`hourFormat`, `gmtFormat`,
  `gmtZeroFormat`, `gmtUnknownFormat`, `regionFormat` and its standard and
  daylight forms, `fallbackFormat`), the exemplar city of `Etc/Unknown`,
  the six names of each metazone (long and short, generic, standard and
  daylight) and the names a zone has of its own (Europe/London's British
  Summer Time). A language entry carries what its file states at a release
  level, a regional entry what its files resolve apart from its parent's,
  as `scripts/cldr_xml.py`'s `carried` says, so that the lookup inherits
  the rest; `root.xml` gives the formats' floor.
* `common/supplemental/metaZones.xml`: each zone's metazones and the
  periods it keeps them in (`metazoneInfo`), each metazone's golden zone
  and its preferred zone in a territory (`mapTimezones`), and the primary
  zones (`primaryZones`).
* `common/bcp47/timezone.xml`: each zone's short identifier (`uslax`), its
  CLDR identifier (the first of its aliases, `Asia/Calcutta`), its other
  aliases, its IANA name where CLDR's differs (`Asia/Kolkata`), and its
  region (the first two letters of a five-letter identifier, or the
  `region` attribute); a deprecated identifier's aliases join its
  preferred one's.
* `common/supplemental/likelySubtags.xml`: the region each carried
  language maximises to, the country UTS #35 takes where a tag names none.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cldr_xml import ENTRIES, carried, lit, path, resolve, rustfmt, write_or_check, xml  # noqa: E402

ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUTPUT = os.path.join(ROOT_DIR, 'crates/hc-i18n/src/zone_names/cldr48.rs')
READ = '2026-09-29'
FORMS = [('long', 'generic'), ('long', 'standard'), ('long', 'daylight'),
         ('short', 'generic'), ('short', 'standard'), ('short', 'daylight')]
SEPARATOR = '|'


def tzn(*rest):
    return path('dates', 'timeZoneNames', *rest)


FORMATS = [
    ('hour', tzn('hourFormat')),
    ('gmt', tzn('gmtFormat')),
    ('gmt_zero', tzn('gmtZeroFormat')),
    ('gmt_unknown', tzn('gmtUnknownFormat')),
    ('region', tzn('regionFormat')),
    ('region_standard', tzn('regionFormat[standard]')),
    ('region_daylight', tzn('regionFormat[daylight]')),
    ('fallback', tzn('fallbackFormat')),
    ('unknown_city', tzn('zone[Etc/Unknown]', 'exemplarCity')),
]


def minutes(stamp):
    """`yyyy-MM-dd HH:mm` UTC as minutes since 1970-01-01 00:00 UTC."""
    import datetime
    moment = datetime.datetime.strptime(stamp, '%Y-%m-%d %H:%M')
    delta = moment - datetime.datetime(1970, 1, 1)
    return delta.days * 1440 + delta.seconds // 60


def metazone_data():
    root = xml('supplemental/metaZones.xml')
    ids = sorted({e.get('mzone') for e in root.iter('usesMetazone')}
                 | {e.get('other') for group in root.iter('mapTimezones')
                    if group.get('type') == 'metazones' for e in group.findall('mapZone')})
    index = {name: i for i, name in enumerate(ids)}
    periods = []
    for zone in root.iter('timezone'):
        rows = []
        for use in zone.findall('usesMetazone'):
            start = minutes(use.get('from')) if use.get('from') else None
            end = minutes(use.get('to')) if use.get('to') else None
            rows.append((start, end, index[use.get('mzone')]))
        periods.append((zone.get('type'), rows))
    periods.sort()
    preferred = sorted((index[m.get('other')], m.get('territory'), m.get('type'))
                       for group in root.iter('mapTimezones') if group.get('type') == 'metazones'
                       for m in group.findall('mapZone'))
    primary = sorted((p.get('iso3166'), p.text.strip()) for p in root.iter('primaryZone'))
    return ids, periods, preferred, primary


def timezone_ids():
    root = xml('bcp47/timezone.xml')
    types = {}
    deprecated = []
    for key in root.iter('key'):
        if key.get('name') != 'tz':
            continue
        for entry in key.findall('type'):
            name = entry.get('name')
            aliases = (entry.get('alias') or '').split()
            if entry.get('deprecated') == 'true':
                deprecated.append((entry.get('preferred'), aliases))
                continue
            region = entry.get('region') or (name[:2].upper() if len(name) == 5 else '')
            types[name] = [aliases, entry.get('iana') or '', region]
    for preferred, aliases in deprecated:
        if preferred in types:
            types[preferred][0] += [a for a in aliases if a not in types[preferred][0]]
    return sorted((name, aliases, iana, region) for name, (aliases, iana, region) in types.items())


def likely_regions():
    root = xml('supplemental/likelySubtags.xml')
    wanted = {tag.split('-')[0] for tag, _, _ in ENTRIES}
    out = {}
    for row in root.iter('likelySubtag'):
        source = row.get('from')
        if source in wanted:
            out[source] = row.get('to').split('_')[-1]
    return sorted(out.items())


def entry_formats(tag, chain, regional):
    return {name: carried(chain, regional, at) or '' for name, at in FORMATS}


def root_formats():
    return {name: resolve([], at)[0] or '' for name, at in FORMATS}


def names_of(chain, regional, base):
    forms = []
    for length, kind in FORMS:
        value = carried(chain, regional, base + ((length, ()), (kind, ())))
        assert value is None or SEPARATOR not in value, value
        forms.append(value or '')
    return forms


def zone_keys():
    """Every zone some carried file names long or short."""
    from cldr_xml import load
    keys = set()
    for chain in [c for _, c, _ in ENTRIES] + [['root']]:
        for name in chain:
            leaves, _ = load(name)
            for at in leaves:
                if (len(at) >= 5 and at[1][0] == 'timeZoneNames' and at[2][0] == 'zone'
                        and at[3][0] in ('long', 'short')):
                    keys.add(dict(at[2][1])['type'])
    return sorted(keys)


def const_name(tag):
    return tag.replace('-', '_').upper()


def generate():
    ids, periods, preferred, primary = metazone_data()
    zones = zone_keys()
    out, dump = [], []
    out.append('/// Every carried locale\'s time zone formats, and root\'s.')
    out.append('pub(super) static FORMATS: &[(&str, ZoneFormats)] = &[')
    for tag, chain, regional in ENTRIES + [('und', [], False)]:
        values = root_formats() if tag == 'und' else entry_formats(tag, chain, regional)
        if not any(values.values()):
            continue
        fields = ', '.join(f'{name}: {lit(value)}' for name, value in values.items())
        out.append(f'    ({lit(tag)}, ZoneFormats {{ {fields} }}),')
        for name, value in values.items():
            if value:
                dump.append(f'{tag}\tformat\t{name}\t{value}')
    out.append('];')
    out.append('')
    out.append('/// The CLDR identifiers of the metazones, sorted.')
    out.append('#[cfg(feature = "zone-names")]')
    out.append('pub(super) static METAZONES: &[&str] = &[' + ', '.join(lit(i) for i in ids) + '];')
    out.append('')
    out.append('/// Each zone\'s metazones, by CLDR identifier, sorted: (from, before,')
    out.append('/// metazone), in minutes since 1970-01-01 00:00 UTC, the open ends')
    out.append('/// `i32::MIN` and `i32::MAX`.')
    out.append('#[cfg(feature = "zone-names")]')
    out.append('pub(super) static ZONE_METAZONES: &[(&str, Periods)] = &[')
    for zone, rows in periods:
        cells = ', '.join(
            f'({"i32::MIN" if a is None else a}, {"i32::MAX" if b is None else b}, {m})'
            for a, b, m in rows)
        out.append(f'    ({lit(zone)}, &[{cells}]),')
        for a, b, m in rows:
            dump.append(f'metazone\t{zone}\t{a}\t{b}\t{ids[m]}')
    out.append('];')
    out.append('')
    out.append('/// Each metazone\'s golden zone (territory `001`) and its preferred zone')
    out.append('/// in a territory: (metazone, territory, zone), sorted.')
    out.append('#[cfg(feature = "zone-names")]')
    out.append('pub(super) static PREFERRED_ZONES: &[(u16, &str, &str)] = &[')
    for m, territory, zone in preferred:
        out.append(f'    ({m}, {lit(territory)}, {lit(zone)}),')
    out.append('];')
    out.append('')
    out.append('/// `primaryZones`: (territory, zone).')
    out.append('#[cfg(feature = "zone-names")]')
    out.append('pub(super) static PRIMARY_ZONES: &[(&str, &str)] = &[')
    out += [f'    ({lit(t)}, {lit(z)}),' for t, z in primary]
    out.append('];')
    out.append('')
    out.append('/// `common/bcp47/timezone.xml`: (short identifier, aliases separated by')
    out.append('/// spaces, the first CLDR\'s own identifier; the IANA name where it differs;')
    out.append('/// the region), sorted by short identifier.')
    out.append('#[cfg(feature = "zone-names")]')
    out.append('pub(super) static TIMEZONES: &[(&str, &str, &str, &str)] = &[')
    for name, aliases, iana, region in timezone_ids():
        out.append(f'    ({lit(name)}, {lit(" ".join(aliases))}, {lit(iana)}, {lit(region)}),')
        dump.append(f'bcp47\t{name}\t{" ".join(aliases)}\t{iana}\t{region}')
    out.append('];')
    out.append('')
    out.append('/// The region each carried language maximises to (`likelySubtags.xml`).')
    out.append('#[cfg(feature = "zone-names")]')
    out.append('pub(super) static LIKELY_REGIONS: &[(&str, &str)] = &[')
    out += [f'    ({lit(language)}, {lit(region)}),' for language, region in likely_regions()]
    out.append('];')
    out.append('')
    tables = []
    for tag, chain, regional in ENTRIES:
        lines = []
        for mz in ids:
            forms = names_of(chain, regional, tzn(f'metazone[{mz}]'))
            lines.append(SEPARATOR.join(forms).rstrip(SEPARATOR))
            for (length, kind), value in zip(FORMS, forms):
                if value:
                    dump.append(f'{tag}\tmetazone\t{mz}\t{length}.{kind}\t{value}')
        own = []
        for zone in zones:
            forms = names_of(chain, regional, tzn(f'zone[{zone}]'))
            if any(forms):
                own.append((zone, SEPARATOR.join(forms).rstrip(SEPARATOR)))
                for (length, kind), value in zip(FORMS, forms):
                    if value:
                        dump.append(f'{tag}\tzone\t{zone}\t{length}.{kind}\t{value}')
        if not any(lines) and not own:
            continue
        tables.append((tag, lines, own))
    for tag, lines, own in tables:
        name = const_name(tag)
        feature = 'zone-names' if tag == 'en' else 'localized-zone-names'
        out.append(f'/// `{tag}`: a line per metazone of [`METAZONES`].')
        out.append(f'#[cfg(feature = "{feature}")]')
        out.append(f'const {name}: &str = concat!(')
        out += ['    ' + lit(line)[:-1] + '\\n",' for line in lines]
        out.append(');')
        out.append('')
        out.append(f'#[cfg(feature = "{feature}")]')
        out.append(f'const {name}_ZONES: &[(&str, &str)] = &[')
        out += [f'    ({lit(zone)}, {lit(forms)}),' for zone, forms in own]
        out.append('];')
        out.append('')
    root_lines, root_zones = [], []
    for mz in ids:
        forms = [resolve([], tzn(f'metazone[{mz}]', l, k))[0] or '' for l, k in FORMS]
        assert not any(forms), mz
    for zone in zones:
        forms = [resolve([], tzn(f'zone[{zone}]', l, k))[0] or '' for l, k in FORMS]
        if any(forms):
            root_zones.append((zone, SEPARATOR.join(forms).rstrip(SEPARATOR)))
    out.append('/// `root.xml`\'s names: no metazone\'s, and a zone\'s own where it gives one.')
    out.append('#[cfg(feature = "zone-names")]')
    out.append('pub(super) static ROOT_TABLE: ZoneNameTable = ZoneNameTable { tag: "und", '
               'metazones: "", zones: &[' + ', '.join(f'({lit(z)}, {lit(f)})' for z, f in root_zones)
               + '] };')
    out.append('')
    out.append('/// `en.xml`\'s names, where the build carries no other locale\'s.')
    out.append('#[cfg(all(feature = "zone-names", not(feature = "localized-zone-names")))]')
    out.append('pub(super) static ENGLISH_TABLE: ZoneNameTable = '
               'ZoneNameTable { tag: "en", metazones: EN, zones: EN_ZONES };')
    out.append('')
    out.append('/// Every locale with names, English among them, in tag order.')
    out.append('#[cfg(feature = "localized-zone-names")]')
    out.append('pub(super) static TABLES: &[ZoneNameTable] = &[')
    for tag, _, _ in tables:
        name = const_name(tag)
        out.append(f'    ZoneNameTable {{ tag: {lit(tag)}, metazones: {name}, zones: {name}_ZONES }},')
    out.append('];')
    header = f'''//! Time zone names and the data that composes them, generated.
//!
//! **Do not edit.** `scripts/zone-names-cldr.py` writes this file from
//! Unicode CLDR 48 (`release-48`, read {READ}, `cldr48-zone-names`):
//! `common/main/<file>.xml`'s `timeZoneNames` for every carried locale with a
//! file, `supplemental/metaZones.xml`, `bcp47/timezone.xml` and
//! `supplemental/likelySubtags.xml`, as the script's documentation says.
//! A metazone's line holds its long generic, standard and daylight names and
//! its short ones, separated by `{SEPARATOR}`, an empty field where the
//! locale states none, trailing empty fields left out.

use super::ZoneFormats;
#[cfg(feature = "zone-names")]
use super::{{Periods, ZoneNameTable}};

'''
    return header + '\n'.join(out) + '\n', dump, len(ids), len(tables)


def main():
    text, dump, count, tables = generate()
    if '--dump' in sys.argv:
        print('\n'.join(dump))
        return
    write_or_check(OUTPUT, rustfmt(text), 'scripts/zone-names-cldr.py')


if __name__ == '__main__':
    main()
