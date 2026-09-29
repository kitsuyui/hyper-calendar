//! What a locale calls a time zone, as data: Unicode CLDR 48's time zone
//! names and the formats and supplemental data UTS #35 composes them from.
//!
//! UTS #35 Part 4, *Dates*, version 48.2, "Using Time Zone Names", names a
//! zone in three kinds of way: by an offset (the *localized GMT format*,
//! `GMT+9`, `UTC+01:00`), by the wall clock of a group of zones (the
//! *generic* names, *Pacific Time*), or by one of its standard and daylight
//! readings (the *specific* names, *Pacific Daylight Time*). A group of
//! zones that share their names for a period is a *metazone*: the zone
//! `America/Los_Angeles` is in the metazone `America_Pacific`, and a zone
//! may change metazone, as `Europe/London` did from `British` to `GMT` on
//! 31 October 1971. The names are keyed by metazone, and a zone can have
//! names of its own besides (*British Summer Time*, `Europe/London`'s
//! daylight name in English). `hc-format` composes them into the pattern
//! fields `z`, `v`, `V` and `O`; this module is the data.
//!
//! Everything here is `scripts/zone-names-cldr.py`'s, generated from the
//! `release-48` tag of CLDR [cldr48-zone-names]:
//!
//! * [`formats`], always carried: each carried locale's `gmtFormat`,
//!   `hourFormat`, `gmtZeroFormat`, `gmtUnknownFormat`, `regionFormat` in
//!   its three forms, `fallbackFormat` and the exemplar city of the unknown
//!   zone, with `root.xml`'s as the floor. They are a few hundred bytes a
//!   locale, and the localized GMT format needs them.
//! * With the `zone-names` feature, the supplemental data — each zone's
//!   metazones and their periods ([`metazone_at`]), each metazone's golden
//!   and preferred zones ([`preferred_zone`]), the primary zones
//!   ([`primary_zone`]), CLDR's short zone identifiers and canonical names
//!   ([`zone_id`]) and each language's likely region ([`likely_region`]) —
//!   and English's names, `en.xml`'s and `root.xml`'s.
//! * With `localized-zone-names`, every other carried locale's names:
//!   some 600 kB of text, so a feature of its own.
//!
//! A language entry carries the names its file states at a release level
//! (`approved` or `contributed`), and a regional entry (`en-GB`, `pt-PT`)
//! those its files resolve apart from its parent's, so that the lookup,
//! which walks the locale's fallback chain, inherits the rest as CLDR's
//! inheritance does. Where a file writes CLDR's empty override `∅∅∅` over
//! a name its parent has — `en_001.xml` over `en.xml`'s *PT*, UTS #35
//! version 48.2, Part 1, "Empty Override": "no value for a path, even if
//! the parent locale has a value" — the table says so, and the lookup stops
//! there with no name, so that a formatter takes the field's fallback.
//! `docs/systems/zone-names.md` explains the composition with worked
//! examples.

#[cfg(feature = "zone-names")]
use crate::locale::Locale;

mod cldr48;

/// Where the data comes from, for a `source` cell.
pub const SOURCE: &str = "Unicode CLDR 48, common/main/<locale>.xml dates/timeZoneNames \
     (approved and contributed values), supplemental/metaZones.xml, bcp47/timezone.xml and \
     supplemental/likelySubtags.xml, release-48, read 2026-09-29 (cldr48-zone-names)";

/// A locale's time zone formats: UTS #35's `timeZoneNames` patterns.
///
/// An empty field is one the locale does not state; [`formats`] fills it
/// from the next locale in the chain and, last, from `root.xml`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZoneFormats {
    /// `hourFormat`, the offset: `+HH:mm;-HH:mm`, the positive pattern and
    /// the negative one.
    pub hour: &'static str,
    /// `gmtFormat`, the offset in its words: `GMT{0}`, `UTC{0}`.
    pub gmt: &'static str,
    /// `gmtZeroFormat`, the zero offset: `GMT`.
    pub gmt_zero: &'static str,
    /// `gmtUnknownFormat`, an offset not known: `GMT+?`.
    pub gmt_unknown: &'static str,
    /// `regionFormat`, a place's generic time: `{0} Time`.
    pub region: &'static str,
    /// `regionFormat type="standard"`: `{0} Standard Time`.
    pub region_standard: &'static str,
    /// `regionFormat type="daylight"`: `{0} Daylight Time`.
    pub region_daylight: &'static str,
    /// `fallbackFormat`, a metazone's name qualified by a place:
    /// `{1} ({0})`, the name in `{1}` and the place in `{0}`.
    pub fallback: &'static str,
    /// The exemplar city of `Etc/Unknown`: *Unknown City*.
    pub unknown_city: &'static str,
}

impl ZoneFormats {
    const EMPTY: Self = Self {
        hour: "",
        gmt: "",
        gmt_zero: "",
        gmt_unknown: "",
        region: "",
        region_standard: "",
        region_daylight: "",
        fallback: "",
        unknown_city: "",
    };

    /// Each empty field filled from `other`.
    const fn or(self, other: Self) -> Self {
        const fn pick(own: &'static str, other: &'static str) -> &'static str {
            if own.is_empty() { other } else { own }
        }
        Self {
            hour: pick(self.hour, other.hour),
            gmt: pick(self.gmt, other.gmt),
            gmt_zero: pick(self.gmt_zero, other.gmt_zero),
            gmt_unknown: pick(self.gmt_unknown, other.gmt_unknown),
            region: pick(self.region, other.region),
            region_standard: pick(self.region_standard, other.region_standard),
            region_daylight: pick(self.region_daylight, other.region_daylight),
            fallback: pick(self.fallback, other.fallback),
            unknown_city: pick(self.unknown_city, other.unknown_city),
        }
    }
}

/// `root.xml`'s formats: `GMT{0}`, `+HH:mm;-HH:mm`, `GMT`, `{0}`, `{1} ({0})`.
#[must_use]
pub fn root_formats() -> ZoneFormats {
    own_formats("und").unwrap_or(ZoneFormats::EMPTY)
}

fn own_formats(tag: &str) -> Option<ZoneFormats> {
    cldr48::FORMATS
        .iter()
        .find(|(candidate, _)| *candidate == tag)
        .map(|(_, formats)| *formats)
}

/// The formats a locale writes zones with, each field from the first
/// locale in its fallback chain that states it, and from `root.xml`
/// behind them all.
#[must_use]
pub fn formats(locale: &crate::Locale) -> ZoneFormats {
    let mut found = ZoneFormats::EMPTY;
    for candidate in locale.fallback() {
        let Some(rendered) = candidate.rendered() else {
            continue;
        };
        if let Some(own) = own_formats(rendered.as_str()) {
            found = found.or(own);
        }
    }
    found.or(root_formats())
}

/// Which of a zone's names: its wall-clock time, or one of its readings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NameType {
    /// *Pacific Time*: the wall clock, whatever it reads.
    Generic,
    /// *Pacific Standard Time*.
    Standard,
    /// *Pacific Daylight Time*.
    Daylight,
}

/// How long a name: *Pacific Time* or *PT*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NameLength {
    /// The full name.
    Long,
    /// The abbreviation, where the locale has one.
    Short,
}

/// A zone name and the tag of the data that gave it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZoneName {
    /// The name.
    pub name: &'static str,
    /// The tag of the table that answered: `en`, `und` for root's.
    pub tag: &'static str,
}

/// One locale's names: a line per metazone of [`metazones`], its six names
/// separated by `|`, and the zones with names of their own.
#[cfg(feature = "zone-names")]
#[derive(Debug, Clone, Copy)]
pub struct ZoneNameTable {
    /// The BCP 47 tag, as [`crate::data::LOCALES`] spells it, or `und`.
    pub tag: &'static str,
    metazones: &'static str,
    zones: &'static [(&'static str, &'static str)],
}

#[cfg(feature = "zone-names")]
const fn field(length: NameLength, kind: NameType) -> usize {
    let base = match length {
        NameLength::Long => 0,
        NameLength::Short => 3,
    };
    base + match kind {
        NameType::Generic => 0,
        NameType::Standard => 1,
        NameType::Daylight => 2,
    }
}

/// What a table's field says: nothing, so that the lookup goes on to the
/// next table; a name; or that the locale has none, CLDR's empty override,
/// which the generated tables write `~`.
#[cfg(feature = "zone-names")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Unstated,
    Name(&'static str),
    NoName,
}

#[cfg(feature = "zone-names")]
impl Cell {
    const NO_NAME: &'static str = "~";

    fn name(self) -> Option<&'static str> {
        match self {
            Self::Name(name) => Some(name),
            Self::Unstated | Self::NoName => None,
        }
    }
}

#[cfg(feature = "zone-names")]
fn pick(forms: &'static str, length: NameLength, kind: NameType) -> Cell {
    match forms.split('|').nth(field(length, kind)) {
        None | Some("") => Cell::Unstated,
        Some(Cell::NO_NAME) => Cell::NoName,
        Some(name) => Cell::Name(name),
    }
}

#[cfg(feature = "zone-names")]
impl ZoneNameTable {
    /// The table's name of the metazone at `index` in [`metazones`];
    /// `None` where the table states none, or states that the locale has
    /// none.
    #[must_use]
    pub fn metazone(
        &self,
        index: usize,
        length: NameLength,
        kind: NameType,
    ) -> Option<&'static str> {
        self.metazone_cell(index, length, kind).name()
    }

    /// The table's own name of a zone, by CLDR's identifier for it; `None`
    /// as for [`ZoneNameTable::metazone`].
    #[must_use]
    pub fn zone(&self, zone: &str, length: NameLength, kind: NameType) -> Option<&'static str> {
        self.zone_cell(zone, length, kind).name()
    }

    fn metazone_cell(&self, index: usize, length: NameLength, kind: NameType) -> Cell {
        self.metazones
            .split('\n')
            .nth(index)
            .map_or(Cell::Unstated, |forms| pick(forms, length, kind))
    }

    fn zone_cell(&self, zone: &str, length: NameLength, kind: NameType) -> Cell {
        self.zones
            .iter()
            .find(|(candidate, _)| *candidate == zone)
            .map_or(Cell::Unstated, |(_, forms)| pick(forms, length, kind))
    }
}

/// A zone's metazones: (from, before, metazone), the instants in minutes
/// since 1970-01-01 00:00 UTC and the metazone by its place in
/// [`metazones`].
#[cfg(feature = "zone-names")]
type Periods = &'static [(i32, i32, u16)];

/// The CLDR identifiers of every metazone, sorted: `Acre` … `Yukon`.
#[cfg(feature = "zone-names")]
#[must_use]
pub fn metazones() -> &'static [&'static str] {
    cldr48::METAZONES
}

/// A zone as `common/bcp47/timezone.xml` identifies it.
#[cfg(feature = "zone-names")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZoneId {
    /// The short identifier of the `-u-tz-` key and of the pattern field
    /// `V`: `uslax`.
    pub short: &'static str,
    /// CLDR's own identifier, the first of its aliases, by which its data
    /// is keyed: `America/Los_Angeles`, `Asia/Calcutta`.
    pub canonical: &'static str,
    /// The IANA name where CLDR's differs, `Asia/Kolkata`; else empty.
    pub iana: &'static str,
    /// The region the zone is associated with, `US`, or empty for a zone
    /// with none (`Etc/UTC`).
    pub region: &'static str,
}

/// A zone by any of its names — its IANA name, CLDR's identifier or an
/// alias — matched without regard to ASCII case.
#[cfg(feature = "zone-names")]
#[must_use]
pub fn zone_id(name: &str) -> Option<ZoneId> {
    cldr48::TIMEZONES
        .iter()
        .find(|(_, aliases, iana, _)| {
            iana.eq_ignore_ascii_case(name)
                || aliases
                    .split(' ')
                    .any(|alias| alias.eq_ignore_ascii_case(name))
        })
        .map(|(short, aliases, iana, region)| ZoneId {
            short,
            canonical: aliases.split(' ').next().unwrap_or(""),
            iana,
            region,
        })
}

/// How many zones `common/bcp47/timezone.xml` associates with a region.
#[cfg(feature = "zone-names")]
#[must_use]
pub fn zones_in_region(region: &str) -> usize {
    cldr48::TIMEZONES
        .iter()
        .filter(|(_, _, _, own)| own.eq_ignore_ascii_case(region))
        .count()
}

/// The metazone a zone is in at an instant, by CLDR's identifier for the
/// zone and the instant in minutes since 1970-01-01 00:00 UTC; `None`
/// where the zone is in none then.
#[cfg(feature = "zone-names")]
#[must_use]
pub fn metazone_at(canonical: &str, utc_minutes: i64) -> Option<&'static str> {
    let index = cldr48::ZONE_METAZONES
        .binary_search_by(|(zone, _)| (*zone).cmp(canonical))
        .ok()?;
    let (_, periods) = cldr48::ZONE_METAZONES[index];
    periods
        .iter()
        .find(|(from, before, _)| {
            i64::from(*from) <= utc_minutes && utc_minutes < i64::from(*before)
        })
        .and_then(|(_, _, metazone)| cldr48::METAZONES.get(usize::from(*metazone)).copied())
}

/// A metazone's preferred zone in a territory, else `None`; for `001`,
/// its golden zone.
#[cfg(feature = "zone-names")]
#[must_use]
pub fn preferred_zone(metazone: &str, territory: &str) -> Option<&'static str> {
    let index = cldr48::METAZONES.binary_search(&metazone).ok()?;
    cldr48::PREFERRED_ZONES
        .iter()
        .find(|(candidate, region, _)| {
            usize::from(*candidate) == index && region.eq_ignore_ascii_case(territory)
        })
        .map(|(_, _, zone)| *zone)
}

/// The zone `primaryZones` names for a region: `Asia/Shanghai` for `CN`.
#[cfg(feature = "zone-names")]
#[must_use]
pub fn primary_zone(region: &str) -> Option<&'static str> {
    cldr48::PRIMARY_ZONES
        .iter()
        .find(|(candidate, _)| candidate.eq_ignore_ascii_case(region))
        .map(|(_, zone)| *zone)
}

/// The region a language's likely subtags give it: `JP` for `ja`.
#[cfg(feature = "zone-names")]
#[must_use]
pub fn likely_region(language: &str) -> Option<&'static str> {
    cldr48::LIKELY_REGIONS
        .iter()
        .find(|(candidate, _)| *candidate == language)
        .map(|(_, region)| *region)
}

/// The table of names for a data tag, where the build carries it.
#[cfg(feature = "zone-names")]
#[must_use]
pub fn table(tag: &str) -> Option<&'static ZoneNameTable> {
    if tag == "und" {
        return Some(&cldr48::ROOT_TABLE);
    }
    #[cfg(feature = "localized-zone-names")]
    {
        cldr48::TABLES.iter().find(|table| table.tag == tag)
    }
    #[cfg(not(feature = "localized-zone-names"))]
    {
        (tag == cldr48::ENGLISH_TABLE.tag).then_some(&cldr48::ENGLISH_TABLE)
    }
}

/// The name the first table in the chain that states the field gives,
/// or `None` where that table states that the locale has none.
#[cfg(feature = "zone-names")]
fn first_in_chain(
    locale: &Locale,
    get: impl Fn(&'static ZoneNameTable) -> Cell,
) -> Option<ZoneName> {
    for candidate in locale.fallback() {
        let Some(rendered) = candidate.rendered() else {
            continue;
        };
        let Some(table) = table(rendered.as_str()) else {
            continue;
        };
        match get(table) {
            Cell::Unstated => {}
            Cell::NoName => return None,
            Cell::Name(name) => {
                return Some(ZoneName {
                    name,
                    tag: table.tag,
                });
            }
        }
    }
    None
}

/// What a locale calls a metazone, in one length and type exactly, from
/// the first table in its fallback chain that names it.
#[cfg(feature = "zone-names")]
#[must_use]
pub fn metazone_name(
    locale: &Locale,
    metazone: &str,
    length: NameLength,
    kind: NameType,
) -> Option<ZoneName> {
    let index = cldr48::METAZONES.binary_search(&metazone).ok()?;
    first_in_chain(locale, |table| table.metazone_cell(index, length, kind))
}

/// What a locale calls a zone of its own, by CLDR's identifier for it, in
/// one length and type exactly.
#[cfg(feature = "zone-names")]
#[must_use]
pub fn own_zone_name(
    locale: &Locale,
    canonical: &str,
    length: NameLength,
    kind: NameType,
) -> Option<ZoneName> {
    first_in_chain(locale, |table| table.zone_cell(canonical, length, kind))
}

/// UTS #35's *type fallback*, over the three names of one length: where
/// the daylight name does not exist, "the metazone doesn't require
/// daylight support", and every type takes the generic name, else the
/// standard one; where it exists, a type is its own name or nothing.
#[cfg(feature = "zone-names")]
#[must_use]
pub fn with_type_fallback(
    names: impl Fn(NameType) -> Option<ZoneName>,
    kind: NameType,
) -> Option<ZoneName> {
    if names(NameType::Daylight).is_some() {
        return names(kind);
    }
    names(NameType::Generic).or_else(|| names(NameType::Standard))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Locale;

    fn locale(tag: &str) -> Locale {
        Locale::parse(tag).unwrap()
    }

    #[test]
    fn the_formats_are_each_files_with_roots_behind_them() {
        let root = root_formats();
        assert_eq!(root.gmt, "GMT{0}");
        assert_eq!(root.hour, "+HH:mm;-HH:mm");
        assert_eq!(root.fallback, "{1} ({0})");
        let english = formats(&locale("en"));
        assert_eq!(english.region, "{0} Time");
        assert_eq!(english.region_daylight, "{0} Daylight Time");
        assert_eq!(english.unknown_city, "Unknown Location");
        // `fr.xml`'s gmtFormat is "UTC{0}" and its hourFormat "+HH:mm;−HH:mm",
        // with a minus sign.
        let french = formats(&locale("fr-CA"));
        assert_eq!(french.gmt, "UTC{0}");
        assert_eq!(french.hour, "+HH:mm;−HH:mm");
        // British English inherits `en_001.xml`'s and `en.xml`'s.
        assert_eq!(formats(&locale("en-GB")).region, "{0} Time");
    }

    #[cfg(feature = "zone-names")]
    #[test]
    fn a_zone_is_found_by_its_iana_name_and_by_cldrs() {
        let kolkata = zone_id("Asia/Kolkata").unwrap();
        assert_eq!(kolkata.short, "inccu");
        assert_eq!(kolkata.canonical, "Asia/Calcutta");
        assert_eq!(kolkata.iana, "Asia/Kolkata");
        assert_eq!(kolkata.region, "IN");
        assert_eq!(zone_id("asia/calcutta"), Some(kolkata));
        assert_eq!(zone_id("America/Los_Angeles").unwrap().short, "uslax");
        // A deprecated identifier's aliases lead to its preferred one.
        assert_eq!(zone_id("Etc/UTC").unwrap().region, "");
        assert_eq!(zones_in_region("JP"), 1);
        assert!(zones_in_region("US") > 20);
    }

    /// The TR 35 example: America/Cambridge_Bay was in America_Mountain
    /// until 1999-10-31 08:00 UTC and in America_Central after it.
    #[cfg(feature = "zone-names")]
    #[test]
    fn a_zone_changes_metazone_at_the_minute_the_data_gives() {
        // 1999-10-31 08:00 UTC is 10 895 days and 480 minutes after 1970.
        let change = 10_895 * 1_440 + 480;
        assert_eq!(
            metazone_at("America/Cambridge_Bay", change - 1),
            Some("America_Mountain")
        );
        assert_eq!(
            metazone_at("America/Cambridge_Bay", change),
            Some("America_Central")
        );
        // Europe/London left British for GMT on 1971-10-31 02:00 UTC.
        let london = (365 + 303) * 1_440 + 120;
        assert_eq!(metazone_at("Europe/London", london - 1), Some("British"));
        assert_eq!(metazone_at("Europe/London", london), Some("GMT"));
        assert_eq!(
            preferred_zone("America_Pacific", "001"),
            Some("America/Los_Angeles")
        );
        assert_eq!(
            preferred_zone("America_Pacific", "CA"),
            Some("America/Vancouver")
        );
        assert_eq!(primary_zone("CN"), Some("Asia/Shanghai"));
        assert_eq!(likely_region("ja"), Some("JP"));
    }

    #[cfg(feature = "zone-names")]
    #[test]
    fn english_names_a_metazone_and_a_zone_of_its_own() {
        let en = locale("en-US");
        let name = |metazone, length, kind| {
            metazone_name(&en, metazone, length, kind).map(|found| found.name)
        };
        assert_eq!(
            name("America_Pacific", NameLength::Long, NameType::Daylight),
            Some("Pacific Daylight Time")
        );
        assert_eq!(
            name("America_Pacific", NameLength::Short, NameType::Generic),
            Some("PT")
        );
        assert_eq!(
            name("Japan", NameLength::Long, NameType::Standard),
            Some("Japan Standard Time")
        );
        assert_eq!(name("Japan", NameLength::Short, NameType::Standard), None);
        assert_eq!(
            own_zone_name(&en, "Europe/London", NameLength::Long, NameType::Daylight)
                .map(|found| found.name),
            Some("British Summer Time")
        );
        // Root's short name of UTC answers every locale.
        assert_eq!(
            own_zone_name(
                &locale("ja"),
                "Etc/UTC",
                NameLength::Short,
                NameType::Standard
            ),
            Some(ZoneName {
                name: "UTC",
                tag: "und"
            })
        );
        // Type fallback: the GMT metazone has no daylight name, so every
        // type takes its standard one, "Greenwich Mean Time".
        assert_eq!(
            with_type_fallback(
                |kind| metazone_name(&en, "GMT", NameLength::Long, kind),
                NameType::Generic
            )
            .map(|found| found.name),
            Some("Greenwich Mean Time")
        );
    }

    #[cfg(feature = "localized-zone-names")]
    #[test]
    fn a_locale_names_the_metazones_its_file_names() {
        let name = |tag: &str, metazone, length, kind| {
            metazone_name(&locale(tag), metazone, length, kind).map(|found| found.name)
        };
        assert_eq!(
            name("ja", "Japan", NameLength::Long, NameType::Standard),
            Some("日本標準時")
        );
        assert_eq!(
            name(
                "de-AT",
                "Europe_Central",
                NameLength::Long,
                NameType::Daylight
            ),
            Some("Mitteleuropäische Sommerzeit")
        );
        // `en_GB.xml`'s short names of the European metazones.
        assert_eq!(
            name(
                "en-GB",
                "Europe_Central",
                NameLength::Short,
                NameType::Daylight
            ),
            Some("CEST")
        );
        assert_eq!(
            name(
                "en-US",
                "Europe_Central",
                NameLength::Short,
                NameType::Daylight
            ),
            None
        );
    }

    /// CLDR's empty override: `en_001.xml` writes `∅∅∅` for the short
    /// names of `America_Pacific`, which `en.xml` gives as *PT*, *PST* and
    /// *PDT*; `es_419.xml` for `Europe_Eastern`'s short names, which
    /// `es.xml` gives as *EET* and *EEST*; `pt_PT.xml` for `Brasilia`'s; and
    /// `ja.xml` for `Japan`'s short generic name beside its *JST* and *JDT*.
    /// The lookup stops at the file that says so, with no name, rather than
    /// inherit the parent's or print the marker.
    #[test]
    fn the_empty_override_stops_the_lookup_with_no_name() {
        let name = |tag: &str, metazone, kind| {
            metazone_name(&locale(tag), metazone, NameLength::Short, kind).map(|found| found.name)
        };
        assert_eq!(name("en", "America_Pacific", NameType::Generic), Some("PT"));
        assert_eq!(
            name("en", "America_Pacific", NameType::Daylight),
            Some("PDT")
        );
        for tag in ["en-001", "en-GB", "en-AU", "en-IN"] {
            for kind in [NameType::Generic, NameType::Standard, NameType::Daylight] {
                assert_eq!(name(tag, "America_Pacific", kind), None, "{tag} {kind:?}");
            }
        }
        assert_eq!(
            name("es", "Europe_Eastern", NameType::Standard),
            Some("EET")
        );
        assert_eq!(name("es-419", "Europe_Eastern", NameType::Standard), None);
        assert_eq!(name("es-MX", "Europe_Eastern", NameType::Standard), None);
        assert_eq!(name("pt-PT", "Brasilia", NameType::Standard), None);
        assert_eq!(name("ja", "Japan", NameType::Generic), None);
        assert_eq!(name("ja", "Japan", NameType::Standard), Some("JST"));
        let table = table("en-001").unwrap();
        let pacific = metazones().binary_search(&"America_Pacific").unwrap();
        assert_eq!(
            table.metazone(pacific, NameLength::Short, NameType::Generic),
            None
        );
    }
}
