//! What a locale calls the city of a time zone, as data: Unicode CLDR 48's
//! exemplar cities for the zones `hc-tz` locates.
//!
//! CLDR names each time zone by a city in it — `Tokyo`, 東京, `Токио` —
//! so that a zone can be shown to a reader who does not know its IANA
//! name: the *exemplar city*, `dates/timeZoneNames/zone/exemplarCity` in
//! `common/main/<locale>.xml`. The values here are the `release-48` tag of
//! <https://github.com/unicode-org/cldr>, read 2026-09-27
//! [cldr48-exemplar-cities], and for the twelve locales of the most-spoken
//! languages 2026-09-28 [cldr48-most-spoken], at CLDR's release levels, `approved` and
//! `contributed`, as the holiday tables' territory names are; a
//! `provisional` or `unconfirmed` value is left out. The system document
//! `docs/systems/zone-locations.md` explains the lookup with examples.
//!
//! # Which zones
//!
//! The [`ZONES`], 418 names: the 312 zones of the IANA database's
//! `zone1970.tab` 2026d and the 106 links its `zone.tab` gives a place of
//! their own, in the order `hc_tz::location::rows` gives them. CLDR keys
//! its data by its own zone identifiers, which for 19 of these are older
//! IANA names — `Asia/Calcutta` for `Asia/Kolkata`, `Europe/Kiev` for
//! `Europe/Kyiv` — so each value was read under the identifier that
//! `common/bcp47/timezone.xml` lists first among the zone's aliases.
//!
//! # English, and the name derived from the zone
//!
//! English is always carried, with the `exemplar-cities` feature: the 33
//! values of `en.xml`, then the 72 of `root.xml`, which English inherits,
//! and for every other zone the name UTS #35 derives from the zone's
//! identifier: "use as the exemplar city the last field of the raw TZID,
//! stripping off the prefix and turning _ into space" (Part 4, *Dates*,
//! §Using Time Zone Names, version 48.2). So `America/Los_Angeles` is `Los
//! Angeles`. The rule is applied to the IANA name the caller holds. Where
//! that differs from CLDR's identifier, either the last fields agree
//! (`America/Argentina/Buenos_Aires` and `America/Buenos_Aires`) or
//! `en.xml` or `root.xml` has a value (`Kolkata` for `Asia/Calcutta`), so
//! the result is the one CLDR's identifier would give.
//!
//! # Every other locale
//!
//! With the `localized-exemplar-cities` feature, every locale `hc-i18n`
//! carries whose CLDR file has at least one value at a release level: 41
//! besides English. Where a file writes CLDR's inheritance marker `↑↑↑` at
//! a release level, the locale's own data says the name is its parent's,
//! which for every carried locale but one is root's: `root.xml`'s value,
//! else the derived name. So German's `Europe/Berlin` is `Berlin`, answered
//! by `de`. The one is European Portuguese, whose parent is `pt.xml`: its
//! table leaves a marker's line empty, so that the lookup goes on to the `pt`
//! table, as CLDR's inheritance does, and keeps the 83 names `pt_PT.xml`
//! gives of its own. A zone a file has no release-level value or marker for
//! is left unnamed there, and [`exemplar_city`] falls back to English.
//! Tibetan (`bo`), Sanskrit (`sa`), Standard Moroccan Tamazight (`zgh`) and
//! Punjabi in the Arabic script (`pa-Arab`) have no exemplar cities in CLDR
//! 48, and every Coptic (`cop`) and Kabyle (`kab`) value is unconfirmed, so
//! those six have no table; Balinese, Middle Egyptian, Nahuatl, Yucatec Maya
//! and Zapotec have no CLDR file.
//!
//! # Compact by design
//!
//! Each table is one string, a line per zone in [`ZONES`] order: the name,
//! [`INHERITED`] for the marker, or nothing. The source keeps each name
//! beside its zone through the `exemplar_cities!` macro, which checks the
//! zones against [`ZONES`] at compile time and keeps only the names, so
//! [`ZONES`] itself is not carried by a build that only looks names up by
//! position. The tables of the other locales are some 230 kB of text, which
//! is why they are a feature of their own: a build that shows zones in
//! English carries only English's.

use core::fmt;

#[cfg(feature = "localized-exemplar-cities")]
use crate::locale::Locale;

/// Where the names come from, for a `source` cell.
pub const SOURCE: &str = "Unicode CLDR 48, common/main/<locale>.xml, \
     dates/timeZoneNames/zone/exemplarCity (approved and contributed values), \
     release-48, read 2026-09-27, and for the twelve locales of the most-spoken languages \
     2026-09-28 (cldr48-most-spoken); UTS #35 Part 4 for the name derived from the zone";

/// The line a table holds where its file writes CLDR's inheritance marker
/// `↑↑↑` at a release level.
pub const INHERITED: &str = "↑";

/// The tag English answers under.
pub const ENGLISH: &str = "en";

/// A city's name: a CLDR value, or the name UTS #35 derives from a zone's
/// identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CityName<'a> {
    /// A value from a CLDR file.
    Cldr(&'static str),
    /// The last field of this zone identifier, written with its
    /// underscores as spaces: `America/Los_Angeles` is `Los Angeles`.
    Derived(&'a str),
}

impl fmt::Display for CityName<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cldr(name) => f.write_str(name),
            Self::Derived(zone) => {
                let field = zone.rsplit('/').next().unwrap_or(zone);
                for part in field.split('_').enumerate() {
                    if part.0 > 0 {
                        f.write_str(" ")?;
                    }
                    f.write_str(part.1)?;
                }
                Ok(())
            }
        }
    }
}

/// A zone's exemplar city and the tag of the data that answered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExemplarCity<'a> {
    /// The city's name.
    pub name: CityName<'a>,
    /// The tag of the table that answered: `ja`, `de` for a request for
    /// `de-AT`, or [`ENGLISH`].
    pub tag: &'static str,
}

/// One locale's exemplar cities for the [`ZONES`].
#[derive(Debug, Clone, Copy)]
pub struct ExemplarCities {
    /// The BCP 47 tag, spelled as [`crate::data::LOCALES`] spells it, or
    /// `und` for the root locale's table.
    pub tag: &'static str,
    /// A line per zone in [`ZONES`] order, each followed by a line feed:
    /// the name, [`INHERITED`], or nothing.
    names: &'static str,
}

impl ExemplarCities {
    /// The line for the zone at `index` in [`ZONES`]: the name,
    /// [`INHERITED`], or `None` where the table has neither.
    #[must_use]
    pub fn line_at(&self, index: usize) -> Option<&'static str> {
        self.names
            .split('\n')
            .nth(index)
            .filter(|line| !line.is_empty())
    }

    /// The name the table itself gives the zone at `index` in [`ZONES`],
    /// if it gives one; `None` for the marker and for no value.
    #[must_use]
    pub fn name_at(&self, index: usize) -> Option<&'static str> {
        self.line_at(index).filter(|line| *line != INHERITED)
    }
}

/// The position of `zone` in [`ZONES`], matched without regard to ASCII
/// case.
#[must_use]
pub fn zone_index(zone: &str) -> Option<usize> {
    ZONES
        .iter()
        .position(|candidate| candidate.eq_ignore_ascii_case(zone))
}

/// `root.xml`'s table.
pub static ROOT_TABLE: ExemplarCities = ExemplarCities {
    tag: "und",
    names: ROOT,
};

/// `en.xml`'s table.
pub static ENGLISH_TABLE: ExemplarCities = ExemplarCities {
    tag: ENGLISH,
    names: EN,
};

/// The root locale's name for the zone at `index` in [`ZONES`], whose
/// identifier is `zone`: `root.xml`'s value, else the derived name.
#[must_use]
pub fn root_city(index: usize, zone: &str) -> CityName<'_> {
    ROOT_TABLE
        .name_at(index)
        .map_or(CityName::Derived(zone), CityName::Cldr)
}

/// The English exemplar city of the zone at `index` in [`ZONES`], whose
/// identifier is `zone`: `en.xml`'s value, else [`root_city`], answered
/// by [`ENGLISH`].
#[must_use]
pub fn english_city(index: usize, zone: &str) -> ExemplarCity<'_> {
    ExemplarCity {
        name: ENGLISH_TABLE
            .name_at(index)
            .map_or_else(|| root_city(index, zone), CityName::Cldr),
        tag: ENGLISH,
    }
}

/// What `locale` calls the city of the zone at `index` in [`ZONES`], whose
/// identifier is `zone`, from the first table in its fallback chain with a
/// line for it; `None` when none has, the root locale included.
///
/// A table's [`INHERITED`] line answers with [`root_city`] under the
/// table's own tag.
#[cfg(feature = "localized-exemplar-cities")]
#[must_use]
pub fn localized_city<'a>(
    locale: &Locale,
    index: usize,
    zone: &'a str,
) -> Option<ExemplarCity<'a>> {
    locale.fallback().find_map(|candidate| {
        let rendered = candidate.rendered()?;
        TABLES
            .iter()
            .filter(|table| rendered.as_str() == table.tag)
            .find_map(|table| {
                let line = table.line_at(index)?;
                let name = if line == INHERITED {
                    root_city(index, zone)
                } else {
                    CityName::Cldr(line)
                };
                Some(ExemplarCity {
                    name,
                    tag: table.tag,
                })
            })
    })
}

/// What `locale` calls the city of the zone at `index` in [`ZONES`], whose
/// identifier is `zone`: [`localized_city`], else [`english_city`].
#[cfg(feature = "localized-exemplar-cities")]
#[must_use]
pub fn exemplar_city<'a>(locale: &Locale, index: usize, zone: &'a str) -> ExemplarCity<'a> {
    localized_city(locale, index, zone).unwrap_or_else(|| english_city(index, zone))
}

/// The table for a data tag, spelled as [`ExemplarCities::tag`] is.
#[cfg(feature = "localized-exemplar-cities")]
#[must_use]
pub fn table(tag: &str) -> Option<&'static ExemplarCities> {
    TABLES.iter().find(|table| table.tag == tag)
}

/// Whether `zones` are exactly [`ZONES`], in order: the compile-time check
/// of `exemplar_cities!`.
const fn same_zones(zones: &[&str]) -> bool {
    if zones.len() != ZONES.len() {
        return false;
    }
    let mut index = 0;
    while index < zones.len() {
        let (left, right) = (zones[index].as_bytes(), ZONES[index].as_bytes());
        if left.len() != right.len() {
            return false;
        }
        let mut byte = 0;
        while byte < left.len() {
            if left[byte] != right[byte] {
                return false;
            }
            byte += 1;
        }
        index += 1;
    }
    true
}

/// A line of a table: the name, or [`INHERITED`] for the token `inherited`.
macro_rules! exemplar_line {
    (inherited) => {
        "↑"
    };
    ($name:literal) => {
        $name
    };
}

/// One table: every zone followed by its name, `""` where the locale has
/// none, or `inherited` where its file writes the marker. The zones must
/// be [`ZONES`], in order, or the build fails; only the lines, each
/// followed by a line feed, are kept.
macro_rules! exemplar_cities {
    ($($zone:literal $name:tt)*) => {{
        const _: () = assert!(
            same_zones(&[$($zone),*]),
            "an exemplar-city table's zones are not ZONES in order"
        );
        concat!($(exemplar_line!($name), "\n"),*)
    }};
}

/// Every locale with a table beside English, and English, in tag order.
#[cfg(feature = "localized-exemplar-cities")]
pub static TABLES: &[ExemplarCities] = &[
    ExemplarCities {
        tag: "am",
        names: AM,
    },
    ExemplarCities {
        tag: "ar",
        names: AR,
    },
    ExemplarCities {
        tag: "bn",
        names: BN,
    },
    ExemplarCities {
        tag: "cs",
        names: CS,
    },
    ExemplarCities {
        tag: "de",
        names: DE,
    },
    ExemplarCities {
        tag: "en",
        names: EN,
    },
    ExemplarCities {
        tag: "es",
        names: ES,
    },
    ExemplarCities {
        tag: "fa",
        names: FA,
    },
    ExemplarCities {
        tag: "fil",
        names: FIL,
    },
    ExemplarCities {
        tag: "fr",
        names: FR,
    },
    ExemplarCities {
        tag: "ha",
        names: HA,
    },
    ExemplarCities {
        tag: "he",
        names: HE,
    },
    ExemplarCities {
        tag: "hi",
        names: HI,
    },
    ExemplarCities {
        tag: "id",
        names: ID,
    },
    ExemplarCities {
        tag: "it",
        names: IT,
    },
    ExemplarCities {
        tag: "ja",
        names: JA,
    },
    ExemplarCities {
        tag: "jv",
        names: JV,
    },
    ExemplarCities {
        tag: "ko",
        names: KO,
    },
    ExemplarCities {
        tag: "ml",
        names: ML,
    },
    ExemplarCities {
        tag: "mr",
        names: MR,
    },
    ExemplarCities {
        tag: "my",
        names: MY,
    },
    ExemplarCities {
        tag: "ne",
        names: NE,
    },
    ExemplarCities {
        tag: "nl",
        names: NL,
    },
    ExemplarCities {
        tag: "pa-Guru",
        names: PA_GURU,
    },
    ExemplarCities {
        tag: "pcm",
        names: PCM,
    },
    ExemplarCities {
        tag: "pl",
        names: PL,
    },
    ExemplarCities {
        tag: "ps",
        names: PS,
    },
    ExemplarCities {
        tag: "pt",
        names: PT,
    },
    ExemplarCities {
        tag: "pt-PT",
        names: PT_PT,
    },
    ExemplarCities {
        tag: "ru",
        names: RU,
    },
    ExemplarCities {
        tag: "sw",
        names: SW,
    },
    ExemplarCities {
        tag: "syr",
        names: SYR,
    },
    ExemplarCities {
        tag: "ta",
        names: TA,
    },
    ExemplarCities {
        tag: "te",
        names: TE,
    },
    ExemplarCities {
        tag: "th",
        names: TH,
    },
    ExemplarCities {
        tag: "tr",
        names: TR,
    },
    ExemplarCities {
        tag: "ur",
        names: UR,
    },
    ExemplarCities {
        tag: "vi",
        names: VI,
    },
    ExemplarCities {
        tag: "yue-Hans",
        names: YUE_HANS,
    },
    ExemplarCities {
        tag: "yue-Hant",
        names: YUE_HANT,
    },
    ExemplarCities {
        tag: "zh-Hans",
        names: ZH_HANS,
    },
    ExemplarCities {
        tag: "zh-Hant",
        names: ZH_HANT,
    },
];

// --- the zones and the tables -------------------------------------------

/// The zones the tables name, in the order of `hc_tz::location::rows`: the
/// rows of `zone1970.tab` 2026d, then the rows of `zone.tab` for the names
/// `zone1970.tab` does not list. The facade's tests hold the two lists
/// together.
pub const ZONES: &[&str] = &[
    "Europe/Andorra",
    "Asia/Dubai",
    "Asia/Kabul",
    "Europe/Tirane",
    "Asia/Yerevan",
    "Antarctica/Casey",
    "Antarctica/Davis",
    "Antarctica/Mawson",
    "Antarctica/Palmer",
    "Antarctica/Rothera",
    "Antarctica/Troll",
    "Antarctica/Vostok",
    "America/Argentina/Buenos_Aires",
    "America/Argentina/Cordoba",
    "America/Argentina/Salta",
    "America/Argentina/Jujuy",
    "America/Argentina/Tucuman",
    "America/Argentina/Catamarca",
    "America/Argentina/La_Rioja",
    "America/Argentina/San_Juan",
    "America/Argentina/Mendoza",
    "America/Argentina/San_Luis",
    "America/Argentina/Rio_Gallegos",
    "America/Argentina/Ushuaia",
    "Pacific/Pago_Pago",
    "Europe/Vienna",
    "Australia/Lord_Howe",
    "Antarctica/Macquarie",
    "Australia/Hobart",
    "Australia/Melbourne",
    "Australia/Sydney",
    "Australia/Broken_Hill",
    "Australia/Brisbane",
    "Australia/Lindeman",
    "Australia/Adelaide",
    "Australia/Darwin",
    "Australia/Perth",
    "Australia/Eucla",
    "Asia/Baku",
    "America/Barbados",
    "Asia/Dhaka",
    "Europe/Brussels",
    "Europe/Sofia",
    "Atlantic/Bermuda",
    "America/La_Paz",
    "America/Noronha",
    "America/Belem",
    "America/Fortaleza",
    "America/Recife",
    "America/Araguaina",
    "America/Maceio",
    "America/Bahia",
    "America/Sao_Paulo",
    "America/Campo_Grande",
    "America/Cuiaba",
    "America/Santarem",
    "America/Porto_Velho",
    "America/Boa_Vista",
    "America/Manaus",
    "America/Eirunepe",
    "America/Rio_Branco",
    "Asia/Thimphu",
    "Europe/Minsk",
    "America/Belize",
    "America/St_Johns",
    "America/Halifax",
    "America/Glace_Bay",
    "America/Moncton",
    "America/Goose_Bay",
    "America/Toronto",
    "America/Iqaluit",
    "America/Winnipeg",
    "America/Resolute",
    "America/Rankin_Inlet",
    "America/Regina",
    "America/Swift_Current",
    "America/Edmonton",
    "America/Cambridge_Bay",
    "America/Inuvik",
    "America/Vancouver",
    "America/Dawson_Creek",
    "America/Fort_Nelson",
    "America/Whitehorse",
    "America/Dawson",
    "Europe/Zurich",
    "Africa/Abidjan",
    "Pacific/Rarotonga",
    "America/Santiago",
    "America/Coyhaique",
    "America/Punta_Arenas",
    "Pacific/Easter",
    "Asia/Shanghai",
    "Asia/Urumqi",
    "America/Bogota",
    "America/Costa_Rica",
    "America/Havana",
    "Atlantic/Cape_Verde",
    "Asia/Nicosia",
    "Asia/Famagusta",
    "Europe/Prague",
    "Europe/Berlin",
    "America/Santo_Domingo",
    "Africa/Algiers",
    "America/Guayaquil",
    "Pacific/Galapagos",
    "Europe/Tallinn",
    "Africa/Cairo",
    "Africa/El_Aaiun",
    "Europe/Madrid",
    "Africa/Ceuta",
    "Atlantic/Canary",
    "Europe/Helsinki",
    "Pacific/Fiji",
    "Atlantic/Stanley",
    "Pacific/Kosrae",
    "Atlantic/Faroe",
    "Europe/Paris",
    "Europe/London",
    "Asia/Tbilisi",
    "America/Cayenne",
    "Europe/Gibraltar",
    "America/Nuuk",
    "America/Danmarkshavn",
    "America/Scoresbysund",
    "America/Thule",
    "Europe/Athens",
    "Atlantic/South_Georgia",
    "America/Guatemala",
    "Pacific/Guam",
    "Africa/Bissau",
    "America/Guyana",
    "Asia/Hong_Kong",
    "America/Tegucigalpa",
    "America/Port-au-Prince",
    "Europe/Budapest",
    "Asia/Jakarta",
    "Asia/Pontianak",
    "Asia/Makassar",
    "Asia/Jayapura",
    "Europe/Dublin",
    "Asia/Jerusalem",
    "Asia/Kolkata",
    "Indian/Chagos",
    "Asia/Baghdad",
    "Asia/Tehran",
    "Europe/Rome",
    "America/Jamaica",
    "Asia/Amman",
    "Asia/Tokyo",
    "Africa/Nairobi",
    "Asia/Bishkek",
    "Pacific/Tarawa",
    "Pacific/Kanton",
    "Pacific/Kiritimati",
    "Asia/Pyongyang",
    "Asia/Seoul",
    "Asia/Almaty",
    "Asia/Qyzylorda",
    "Asia/Qostanay",
    "Asia/Aqtobe",
    "Asia/Aqtau",
    "Asia/Atyrau",
    "Asia/Oral",
    "Asia/Beirut",
    "Asia/Colombo",
    "Africa/Monrovia",
    "Europe/Vilnius",
    "Europe/Riga",
    "Africa/Tripoli",
    "Africa/Casablanca",
    "Europe/Chisinau",
    "Pacific/Kwajalein",
    "Asia/Yangon",
    "Asia/Ulaanbaatar",
    "Asia/Hovd",
    "Asia/Macau",
    "America/Martinique",
    "Europe/Malta",
    "Indian/Mauritius",
    "Indian/Maldives",
    "America/Mexico_City",
    "America/Cancun",
    "America/Merida",
    "America/Monterrey",
    "America/Matamoros",
    "America/Chihuahua",
    "America/Ciudad_Juarez",
    "America/Ojinaga",
    "America/Mazatlan",
    "America/Bahia_Banderas",
    "America/Hermosillo",
    "America/Tijuana",
    "Asia/Kuching",
    "Africa/Maputo",
    "Africa/Windhoek",
    "Pacific/Noumea",
    "Pacific/Norfolk",
    "Africa/Lagos",
    "America/Managua",
    "Asia/Kathmandu",
    "Pacific/Nauru",
    "Pacific/Niue",
    "Pacific/Auckland",
    "Pacific/Chatham",
    "America/Panama",
    "America/Lima",
    "Pacific/Tahiti",
    "Pacific/Marquesas",
    "Pacific/Gambier",
    "Pacific/Port_Moresby",
    "Pacific/Bougainville",
    "Asia/Manila",
    "Asia/Karachi",
    "Europe/Warsaw",
    "America/Miquelon",
    "Pacific/Pitcairn",
    "America/Puerto_Rico",
    "Asia/Gaza",
    "Asia/Hebron",
    "Europe/Lisbon",
    "Atlantic/Madeira",
    "Atlantic/Azores",
    "Pacific/Palau",
    "America/Asuncion",
    "Asia/Qatar",
    "Europe/Bucharest",
    "Europe/Belgrade",
    "Europe/Kaliningrad",
    "Europe/Moscow",
    "Europe/Simferopol",
    "Europe/Kirov",
    "Europe/Volgograd",
    "Europe/Astrakhan",
    "Europe/Saratov",
    "Europe/Ulyanovsk",
    "Europe/Samara",
    "Asia/Yekaterinburg",
    "Asia/Omsk",
    "Asia/Novosibirsk",
    "Asia/Barnaul",
    "Asia/Tomsk",
    "Asia/Novokuznetsk",
    "Asia/Krasnoyarsk",
    "Asia/Irkutsk",
    "Asia/Chita",
    "Asia/Yakutsk",
    "Asia/Khandyga",
    "Asia/Vladivostok",
    "Asia/Ust-Nera",
    "Asia/Magadan",
    "Asia/Sakhalin",
    "Asia/Srednekolymsk",
    "Asia/Kamchatka",
    "Asia/Anadyr",
    "Asia/Riyadh",
    "Pacific/Guadalcanal",
    "Africa/Khartoum",
    "Asia/Singapore",
    "America/Paramaribo",
    "Africa/Juba",
    "Africa/Sao_Tome",
    "America/El_Salvador",
    "Asia/Damascus",
    "America/Grand_Turk",
    "Africa/Ndjamena",
    "Asia/Bangkok",
    "Asia/Dushanbe",
    "Pacific/Fakaofo",
    "Asia/Dili",
    "Asia/Ashgabat",
    "Africa/Tunis",
    "Pacific/Tongatapu",
    "Europe/Istanbul",
    "Asia/Taipei",
    "Europe/Kyiv",
    "America/New_York",
    "America/Detroit",
    "America/Kentucky/Louisville",
    "America/Kentucky/Monticello",
    "America/Indiana/Indianapolis",
    "America/Indiana/Vincennes",
    "America/Indiana/Winamac",
    "America/Indiana/Marengo",
    "America/Indiana/Petersburg",
    "America/Indiana/Vevay",
    "America/Chicago",
    "America/Indiana/Tell_City",
    "America/Indiana/Knox",
    "America/Menominee",
    "America/North_Dakota/Center",
    "America/North_Dakota/New_Salem",
    "America/North_Dakota/Beulah",
    "America/Denver",
    "America/Boise",
    "America/Phoenix",
    "America/Los_Angeles",
    "America/Anchorage",
    "America/Juneau",
    "America/Sitka",
    "America/Metlakatla",
    "America/Yakutat",
    "America/Nome",
    "America/Adak",
    "Pacific/Honolulu",
    "America/Montevideo",
    "Asia/Samarkand",
    "Asia/Tashkent",
    "America/Caracas",
    "Asia/Ho_Chi_Minh",
    "Pacific/Efate",
    "Pacific/Apia",
    "Africa/Johannesburg",
    "America/Antigua",
    "America/Anguilla",
    "Africa/Luanda",
    "Antarctica/McMurdo",
    "Antarctica/DumontDUrville",
    "Antarctica/Syowa",
    "America/Aruba",
    "Europe/Mariehamn",
    "Europe/Sarajevo",
    "Africa/Ouagadougou",
    "Asia/Bahrain",
    "Africa/Bujumbura",
    "Africa/Porto-Novo",
    "America/St_Barthelemy",
    "Asia/Brunei",
    "America/Kralendijk",
    "America/Nassau",
    "Africa/Gaborone",
    "America/Blanc-Sablon",
    "America/Atikokan",
    "America/Creston",
    "Indian/Cocos",
    "Africa/Kinshasa",
    "Africa/Lubumbashi",
    "Africa/Bangui",
    "Africa/Brazzaville",
    "Africa/Douala",
    "America/Curacao",
    "Indian/Christmas",
    "Europe/Busingen",
    "Africa/Djibouti",
    "Europe/Copenhagen",
    "America/Dominica",
    "Africa/Asmara",
    "Africa/Addis_Ababa",
    "Pacific/Chuuk",
    "Pacific/Pohnpei",
    "Africa/Libreville",
    "America/Grenada",
    "Europe/Guernsey",
    "Africa/Accra",
    "Africa/Banjul",
    "Africa/Conakry",
    "America/Guadeloupe",
    "Africa/Malabo",
    "Europe/Zagreb",
    "Europe/Isle_of_Man",
    "Atlantic/Reykjavik",
    "Europe/Jersey",
    "Asia/Phnom_Penh",
    "Indian/Comoro",
    "America/St_Kitts",
    "Asia/Kuwait",
    "America/Cayman",
    "Asia/Vientiane",
    "America/St_Lucia",
    "Europe/Vaduz",
    "Africa/Maseru",
    "Europe/Luxembourg",
    "Europe/Monaco",
    "Europe/Podgorica",
    "America/Marigot",
    "Indian/Antananarivo",
    "Pacific/Majuro",
    "Europe/Skopje",
    "Africa/Bamako",
    "Pacific/Saipan",
    "Africa/Nouakchott",
    "America/Montserrat",
    "Africa/Blantyre",
    "Asia/Kuala_Lumpur",
    "Africa/Niamey",
    "Europe/Amsterdam",
    "Europe/Oslo",
    "Asia/Muscat",
    "Indian/Reunion",
    "Africa/Kigali",
    "Indian/Mahe",
    "Europe/Stockholm",
    "Atlantic/St_Helena",
    "Europe/Ljubljana",
    "Arctic/Longyearbyen",
    "Europe/Bratislava",
    "Africa/Freetown",
    "Europe/San_Marino",
    "Africa/Dakar",
    "Africa/Mogadishu",
    "America/Lower_Princes",
    "Africa/Mbabane",
    "Indian/Kerguelen",
    "Africa/Lome",
    "America/Port_of_Spain",
    "Pacific/Funafuti",
    "Africa/Dar_es_Salaam",
    "Africa/Kampala",
    "Pacific/Midway",
    "Pacific/Wake",
    "Europe/Vatican",
    "America/St_Vincent",
    "America/Tortola",
    "America/St_Thomas",
    "Pacific/Wallis",
    "Asia/Aden",
    "Indian/Mayotte",
    "Africa/Lusaka",
    "Africa/Harare",
];

// `common/main/root.xml`: 72 of the 418 zones named.
const ROOT: &str = exemplar_cities! {
    "Europe/Andorra" ""
    "Asia/Dubai" ""
    "Asia/Kabul" ""
    "Europe/Tirane" "Tirana"
    "Asia/Yerevan" ""
    "Antarctica/Casey" ""
    "Antarctica/Davis" ""
    "Antarctica/Mawson" ""
    "Antarctica/Palmer" ""
    "Antarctica/Rothera" ""
    "Antarctica/Troll" ""
    "Antarctica/Vostok" ""
    "America/Argentina/Buenos_Aires" ""
    "America/Argentina/Cordoba" "Córdoba"
    "America/Argentina/Salta" ""
    "America/Argentina/Jujuy" ""
    "America/Argentina/Tucuman" "Tucumán"
    "America/Argentina/Catamarca" ""
    "America/Argentina/La_Rioja" ""
    "America/Argentina/San_Juan" ""
    "America/Argentina/Mendoza" ""
    "America/Argentina/San_Luis" ""
    "America/Argentina/Rio_Gallegos" "Río Gallegos"
    "America/Argentina/Ushuaia" ""
    "Pacific/Pago_Pago" ""
    "Europe/Vienna" ""
    "Australia/Lord_Howe" ""
    "Antarctica/Macquarie" ""
    "Australia/Hobart" ""
    "Australia/Melbourne" ""
    "Australia/Sydney" ""
    "Australia/Broken_Hill" ""
    "Australia/Brisbane" ""
    "Australia/Lindeman" ""
    "Australia/Adelaide" ""
    "Australia/Darwin" ""
    "Australia/Perth" ""
    "Australia/Eucla" ""
    "Asia/Baku" ""
    "America/Barbados" ""
    "Asia/Dhaka" ""
    "Europe/Brussels" ""
    "Europe/Sofia" ""
    "Atlantic/Bermuda" ""
    "America/La_Paz" ""
    "America/Noronha" "Fernando de Noronha"
    "America/Belem" "Belém"
    "America/Fortaleza" ""
    "America/Recife" ""
    "America/Araguaina" "Araguaína"
    "America/Maceio" "Maceió"
    "America/Bahia" ""
    "America/Sao_Paulo" "São Paulo"
    "America/Campo_Grande" ""
    "America/Cuiaba" "Cuiabá"
    "America/Santarem" "Santarém"
    "America/Porto_Velho" ""
    "America/Boa_Vista" ""
    "America/Manaus" ""
    "America/Eirunepe" "Eirunepé"
    "America/Rio_Branco" ""
    "Asia/Thimphu" ""
    "Europe/Minsk" ""
    "America/Belize" ""
    "America/St_Johns" "St. John’s"
    "America/Halifax" ""
    "America/Glace_Bay" ""
    "America/Moncton" ""
    "America/Goose_Bay" ""
    "America/Toronto" ""
    "America/Iqaluit" ""
    "America/Winnipeg" ""
    "America/Resolute" ""
    "America/Rankin_Inlet" ""
    "America/Regina" ""
    "America/Swift_Current" ""
    "America/Edmonton" ""
    "America/Cambridge_Bay" ""
    "America/Inuvik" ""
    "America/Vancouver" ""
    "America/Dawson_Creek" ""
    "America/Fort_Nelson" ""
    "America/Whitehorse" ""
    "America/Dawson" ""
    "Europe/Zurich" ""
    "Africa/Abidjan" ""
    "Pacific/Rarotonga" ""
    "America/Santiago" ""
    "America/Coyhaique" ""
    "America/Punta_Arenas" ""
    "Pacific/Easter" ""
    "Asia/Shanghai" ""
    "Asia/Urumqi" "Ürümqi"
    "America/Bogota" "Bogotá"
    "America/Costa_Rica" ""
    "America/Havana" ""
    "Atlantic/Cape_Verde" ""
    "Asia/Nicosia" ""
    "Asia/Famagusta" ""
    "Europe/Prague" ""
    "Europe/Berlin" ""
    "America/Santo_Domingo" ""
    "Africa/Algiers" ""
    "America/Guayaquil" ""
    "Pacific/Galapagos" "Galápagos"
    "Europe/Tallinn" ""
    "Africa/Cairo" ""
    "Africa/El_Aaiun" "El Aaiún"
    "Europe/Madrid" ""
    "Africa/Ceuta" ""
    "Atlantic/Canary" "Canarias"
    "Europe/Helsinki" ""
    "Pacific/Fiji" ""
    "Atlantic/Stanley" ""
    "Pacific/Kosrae" ""
    "Atlantic/Faroe" "Faroe"
    "Europe/Paris" ""
    "Europe/London" ""
    "Asia/Tbilisi" ""
    "America/Cayenne" ""
    "Europe/Gibraltar" ""
    "America/Nuuk" "Nuuk"
    "America/Danmarkshavn" ""
    "America/Scoresbysund" "Ittoqqortoormiit"
    "America/Thule" ""
    "Europe/Athens" ""
    "Atlantic/South_Georgia" ""
    "America/Guatemala" ""
    "Pacific/Guam" ""
    "Africa/Bissau" ""
    "America/Guyana" ""
    "Asia/Hong_Kong" ""
    "America/Tegucigalpa" ""
    "America/Port-au-Prince" ""
    "Europe/Budapest" ""
    "Asia/Jakarta" ""
    "Asia/Pontianak" ""
    "Asia/Makassar" ""
    "Asia/Jayapura" ""
    "Europe/Dublin" ""
    "Asia/Jerusalem" ""
    "Asia/Kolkata" "Kolkata"
    "Indian/Chagos" ""
    "Asia/Baghdad" ""
    "Asia/Tehran" ""
    "Europe/Rome" ""
    "America/Jamaica" ""
    "Asia/Amman" ""
    "Asia/Tokyo" ""
    "Africa/Nairobi" ""
    "Asia/Bishkek" ""
    "Pacific/Tarawa" ""
    "Pacific/Kanton" "Canton"
    "Pacific/Kiritimati" ""
    "Asia/Pyongyang" ""
    "Asia/Seoul" ""
    "Asia/Almaty" ""
    "Asia/Qyzylorda" ""
    "Asia/Qostanay" ""
    "Asia/Aqtobe" ""
    "Asia/Aqtau" ""
    "Asia/Atyrau" ""
    "Asia/Oral" ""
    "Asia/Beirut" ""
    "Asia/Colombo" ""
    "Africa/Monrovia" ""
    "Europe/Vilnius" ""
    "Europe/Riga" ""
    "Africa/Tripoli" ""
    "Africa/Casablanca" ""
    "Europe/Chisinau" "Chișinău"
    "Pacific/Kwajalein" ""
    "Asia/Yangon" "Yangon"
    "Asia/Ulaanbaatar" ""
    "Asia/Hovd" "Khovd"
    "Asia/Macau" "Macao"
    "America/Martinique" ""
    "Europe/Malta" ""
    "Indian/Mauritius" ""
    "Indian/Maldives" ""
    "America/Mexico_City" "Ciudad de México"
    "America/Cancun" "Cancún"
    "America/Merida" "Mérida"
    "America/Monterrey" ""
    "America/Matamoros" ""
    "America/Chihuahua" ""
    "America/Ciudad_Juarez" "Ciudad Juárez"
    "America/Ojinaga" ""
    "America/Mazatlan" "Mazatlán"
    "America/Bahia_Banderas" "Bahía de Banderas"
    "America/Hermosillo" ""
    "America/Tijuana" ""
    "Asia/Kuching" ""
    "Africa/Maputo" ""
    "Africa/Windhoek" ""
    "Pacific/Noumea" "Nouméa"
    "Pacific/Norfolk" ""
    "Africa/Lagos" ""
    "America/Managua" ""
    "Asia/Kathmandu" "Kathmandu"
    "Pacific/Nauru" ""
    "Pacific/Niue" ""
    "Pacific/Auckland" ""
    "Pacific/Chatham" ""
    "America/Panama" ""
    "America/Lima" ""
    "Pacific/Tahiti" ""
    "Pacific/Marquesas" ""
    "Pacific/Gambier" ""
    "Pacific/Port_Moresby" ""
    "Pacific/Bougainville" ""
    "Asia/Manila" ""
    "Asia/Karachi" ""
    "Europe/Warsaw" ""
    "America/Miquelon" "Saint-Pierre"
    "Pacific/Pitcairn" ""
    "America/Puerto_Rico" ""
    "Asia/Gaza" ""
    "Asia/Hebron" ""
    "Europe/Lisbon" ""
    "Atlantic/Madeira" ""
    "Atlantic/Azores" ""
    "Pacific/Palau" ""
    "America/Asuncion" "Asunción"
    "Asia/Qatar" ""
    "Europe/Bucharest" ""
    "Europe/Belgrade" ""
    "Europe/Kaliningrad" ""
    "Europe/Moscow" ""
    "Europe/Simferopol" ""
    "Europe/Kirov" ""
    "Europe/Volgograd" ""
    "Europe/Astrakhan" ""
    "Europe/Saratov" ""
    "Europe/Ulyanovsk" ""
    "Europe/Samara" ""
    "Asia/Yekaterinburg" ""
    "Asia/Omsk" ""
    "Asia/Novosibirsk" ""
    "Asia/Barnaul" ""
    "Asia/Tomsk" ""
    "Asia/Novokuznetsk" ""
    "Asia/Krasnoyarsk" ""
    "Asia/Irkutsk" ""
    "Asia/Chita" ""
    "Asia/Yakutsk" ""
    "Asia/Khandyga" ""
    "Asia/Vladivostok" ""
    "Asia/Ust-Nera" ""
    "Asia/Magadan" ""
    "Asia/Sakhalin" ""
    "Asia/Srednekolymsk" ""
    "Asia/Kamchatka" ""
    "Asia/Anadyr" ""
    "Asia/Riyadh" ""
    "Pacific/Guadalcanal" ""
    "Africa/Khartoum" ""
    "Asia/Singapore" ""
    "America/Paramaribo" ""
    "Africa/Juba" ""
    "Africa/Sao_Tome" "São Tomé"
    "America/El_Salvador" ""
    "Asia/Damascus" ""
    "America/Grand_Turk" ""
    "Africa/Ndjamena" "N’Djamena"
    "Asia/Bangkok" ""
    "Asia/Dushanbe" ""
    "Pacific/Fakaofo" ""
    "Asia/Dili" ""
    "Asia/Ashgabat" ""
    "Africa/Tunis" ""
    "Pacific/Tongatapu" ""
    "Europe/Istanbul" ""
    "Asia/Taipei" ""
    "Europe/Kyiv" "Kyiv"
    "America/New_York" ""
    "America/Detroit" ""
    "America/Kentucky/Louisville" ""
    "America/Kentucky/Monticello" "Monticello, Kentucky"
    "America/Indiana/Indianapolis" ""
    "America/Indiana/Vincennes" "Vincennes, Indiana"
    "America/Indiana/Winamac" "Winamac, Indiana"
    "America/Indiana/Marengo" "Marengo, Indiana"
    "America/Indiana/Petersburg" "Petersburg, Indiana"
    "America/Indiana/Vevay" "Vevay, Indiana"
    "America/Chicago" ""
    "America/Indiana/Tell_City" "Tell City, Indiana"
    "America/Indiana/Knox" "Knox, Indiana"
    "America/Menominee" ""
    "America/North_Dakota/Center" "Center, North Dakota"
    "America/North_Dakota/New_Salem" "New Salem, North Dakota"
    "America/North_Dakota/Beulah" "Beulah, North Dakota"
    "America/Denver" ""
    "America/Boise" ""
    "America/Phoenix" ""
    "America/Los_Angeles" ""
    "America/Anchorage" ""
    "America/Juneau" ""
    "America/Sitka" ""
    "America/Metlakatla" ""
    "America/Yakutat" ""
    "America/Nome" ""
    "America/Adak" ""
    "Pacific/Honolulu" ""
    "America/Montevideo" ""
    "Asia/Samarkand" ""
    "Asia/Tashkent" ""
    "America/Caracas" ""
    "Asia/Ho_Chi_Minh" "Ho Chi Minh"
    "Pacific/Efate" ""
    "Pacific/Apia" ""
    "Africa/Johannesburg" ""
    "America/Antigua" ""
    "America/Anguilla" ""
    "Africa/Luanda" ""
    "Antarctica/McMurdo" ""
    "Antarctica/DumontDUrville" "Dumont-d’Urville"
    "Antarctica/Syowa" "Showa"
    "America/Aruba" ""
    "Europe/Mariehamn" ""
    "Europe/Sarajevo" ""
    "Africa/Ouagadougou" ""
    "Asia/Bahrain" ""
    "Africa/Bujumbura" ""
    "Africa/Porto-Novo" ""
    "America/St_Barthelemy" "St. Barthélemy"
    "Asia/Brunei" ""
    "America/Kralendijk" ""
    "America/Nassau" ""
    "Africa/Gaborone" ""
    "America/Blanc-Sablon" ""
    "America/Atikokan" "Atikokan"
    "America/Creston" ""
    "Indian/Cocos" ""
    "Africa/Kinshasa" ""
    "Africa/Lubumbashi" ""
    "Africa/Bangui" ""
    "Africa/Brazzaville" ""
    "Africa/Douala" ""
    "America/Curacao" "Curaçao"
    "Indian/Christmas" ""
    "Europe/Busingen" "Büsingen"
    "Africa/Djibouti" ""
    "Europe/Copenhagen" ""
    "America/Dominica" ""
    "Africa/Asmara" "Asmara"
    "Africa/Addis_Ababa" ""
    "Pacific/Chuuk" "Chuuk"
    "Pacific/Pohnpei" "Pohnpei"
    "Africa/Libreville" ""
    "America/Grenada" ""
    "Europe/Guernsey" ""
    "Africa/Accra" ""
    "Africa/Banjul" ""
    "Africa/Conakry" ""
    "America/Guadeloupe" ""
    "Africa/Malabo" ""
    "Europe/Zagreb" ""
    "Europe/Isle_of_Man" ""
    "Atlantic/Reykjavik" ""
    "Europe/Jersey" ""
    "Asia/Phnom_Penh" ""
    "Indian/Comoro" "Comores"
    "America/St_Kitts" "St. Kitts"
    "Asia/Kuwait" ""
    "America/Cayman" ""
    "Asia/Vientiane" ""
    "America/St_Lucia" "St. Lucia"
    "Europe/Vaduz" ""
    "Africa/Maseru" ""
    "Europe/Luxembourg" ""
    "Europe/Monaco" ""
    "Europe/Podgorica" ""
    "America/Marigot" ""
    "Indian/Antananarivo" ""
    "Pacific/Majuro" ""
    "Europe/Skopje" ""
    "Africa/Bamako" ""
    "Pacific/Saipan" ""
    "Africa/Nouakchott" ""
    "America/Montserrat" ""
    "Africa/Blantyre" ""
    "Asia/Kuala_Lumpur" ""
    "Africa/Niamey" ""
    "Europe/Amsterdam" ""
    "Europe/Oslo" ""
    "Asia/Muscat" ""
    "Indian/Reunion" "Réunion"
    "Africa/Kigali" ""
    "Indian/Mahe" "Mahé"
    "Europe/Stockholm" ""
    "Atlantic/St_Helena" "St. Helena"
    "Europe/Ljubljana" ""
    "Arctic/Longyearbyen" ""
    "Europe/Bratislava" ""
    "Africa/Freetown" ""
    "Europe/San_Marino" ""
    "Africa/Dakar" ""
    "Africa/Mogadishu" ""
    "America/Lower_Princes" "Lower Prince’s Quarter"
    "Africa/Mbabane" ""
    "Indian/Kerguelen" ""
    "Africa/Lome" "Lomé"
    "America/Port_of_Spain" ""
    "Pacific/Funafuti" ""
    "Africa/Dar_es_Salaam" ""
    "Africa/Kampala" ""
    "Pacific/Midway" ""
    "Pacific/Wake" ""
    "Europe/Vatican" ""
    "America/St_Vincent" "St. Vincent"
    "America/Tortola" ""
    "America/St_Thomas" "St. Thomas"
    "Pacific/Wallis" "Wallis & Futuna"
    "Asia/Aden" ""
    "Indian/Mayotte" ""
    "Africa/Lusaka" ""
    "Africa/Harare" ""
};

// `common/main/en.xml`: 33 of the 418 zones named.
const EN: &str = exemplar_cities! {
    "Europe/Andorra" ""
    "Asia/Dubai" ""
    "Asia/Kabul" ""
    "Europe/Tirane" ""
    "Asia/Yerevan" ""
    "Antarctica/Casey" "Casey Station"
    "Antarctica/Davis" ""
    "Antarctica/Mawson" "Mawson Station"
    "Antarctica/Palmer" "Palmer Land"
    "Antarctica/Rothera" "Rothera Station"
    "Antarctica/Troll" "Troll Station"
    "Antarctica/Vostok" "Vostok Station"
    "America/Argentina/Buenos_Aires" ""
    "America/Argentina/Cordoba" ""
    "America/Argentina/Salta" ""
    "America/Argentina/Jujuy" ""
    "America/Argentina/Tucuman" ""
    "America/Argentina/Catamarca" ""
    "America/Argentina/La_Rioja" ""
    "America/Argentina/San_Juan" ""
    "America/Argentina/Mendoza" ""
    "America/Argentina/San_Luis" ""
    "America/Argentina/Rio_Gallegos" ""
    "America/Argentina/Ushuaia" ""
    "Pacific/Pago_Pago" ""
    "Europe/Vienna" ""
    "Australia/Lord_Howe" "Lord Howe Island"
    "Antarctica/Macquarie" "Macquarie Island"
    "Australia/Hobart" ""
    "Australia/Melbourne" ""
    "Australia/Sydney" ""
    "Australia/Broken_Hill" ""
    "Australia/Brisbane" ""
    "Australia/Lindeman" ""
    "Australia/Adelaide" ""
    "Australia/Darwin" ""
    "Australia/Perth" ""
    "Australia/Eucla" ""
    "Asia/Baku" ""
    "America/Barbados" ""
    "Asia/Dhaka" ""
    "Europe/Brussels" ""
    "Europe/Sofia" ""
    "Atlantic/Bermuda" ""
    "America/La_Paz" ""
    "America/Noronha" ""
    "America/Belem" ""
    "America/Fortaleza" ""
    "America/Recife" ""
    "America/Araguaina" ""
    "America/Maceio" ""
    "America/Bahia" ""
    "America/Sao_Paulo" ""
    "America/Campo_Grande" ""
    "America/Cuiaba" ""
    "America/Santarem" ""
    "America/Porto_Velho" ""
    "America/Boa_Vista" ""
    "America/Manaus" ""
    "America/Eirunepe" ""
    "America/Rio_Branco" ""
    "Asia/Thimphu" ""
    "Europe/Minsk" ""
    "America/Belize" ""
    "America/St_Johns" ""
    "America/Halifax" ""
    "America/Glace_Bay" ""
    "America/Moncton" ""
    "America/Goose_Bay" ""
    "America/Toronto" ""
    "America/Iqaluit" ""
    "America/Winnipeg" ""
    "America/Resolute" ""
    "America/Rankin_Inlet" ""
    "America/Regina" ""
    "America/Swift_Current" ""
    "America/Edmonton" ""
    "America/Cambridge_Bay" ""
    "America/Inuvik" ""
    "America/Vancouver" ""
    "America/Dawson_Creek" ""
    "America/Fort_Nelson" ""
    "America/Whitehorse" ""
    "America/Dawson" ""
    "Europe/Zurich" ""
    "Africa/Abidjan" ""
    "Pacific/Rarotonga" ""
    "America/Santiago" ""
    "America/Coyhaique" ""
    "America/Punta_Arenas" ""
    "Pacific/Easter" "Easter Island"
    "Asia/Shanghai" ""
    "Asia/Urumqi" ""
    "America/Bogota" ""
    "America/Costa_Rica" ""
    "America/Havana" ""
    "Atlantic/Cape_Verde" ""
    "Asia/Nicosia" ""
    "Asia/Famagusta" ""
    "Europe/Prague" ""
    "Europe/Berlin" ""
    "America/Santo_Domingo" ""
    "Africa/Algiers" ""
    "America/Guayaquil" ""
    "Pacific/Galapagos" "Galápagos Islands"
    "Europe/Tallinn" ""
    "Africa/Cairo" ""
    "Africa/El_Aaiun" ""
    "Europe/Madrid" ""
    "Africa/Ceuta" ""
    "Atlantic/Canary" "Canaries"
    "Europe/Helsinki" ""
    "Pacific/Fiji" ""
    "Atlantic/Stanley" ""
    "Pacific/Kosrae" ""
    "Atlantic/Faroe" "Faroes"
    "Europe/Paris" ""
    "Europe/London" ""
    "Asia/Tbilisi" ""
    "America/Cayenne" ""
    "Europe/Gibraltar" ""
    "America/Nuuk" ""
    "America/Danmarkshavn" ""
    "America/Scoresbysund" ""
    "America/Thule" ""
    "Europe/Athens" ""
    "Atlantic/South_Georgia" ""
    "America/Guatemala" ""
    "Pacific/Guam" ""
    "Africa/Bissau" ""
    "America/Guyana" ""
    "Asia/Hong_Kong" ""
    "America/Tegucigalpa" ""
    "America/Port-au-Prince" ""
    "Europe/Budapest" ""
    "Asia/Jakarta" ""
    "Asia/Pontianak" ""
    "Asia/Makassar" ""
    "Asia/Jayapura" ""
    "Europe/Dublin" ""
    "Asia/Jerusalem" ""
    "Asia/Kolkata" ""
    "Indian/Chagos" "Chagos Archipelago"
    "Asia/Baghdad" ""
    "Asia/Tehran" ""
    "Europe/Rome" ""
    "America/Jamaica" ""
    "Asia/Amman" ""
    "Asia/Tokyo" ""
    "Africa/Nairobi" ""
    "Asia/Bishkek" ""
    "Pacific/Tarawa" ""
    "Pacific/Kanton" "Canton Island"
    "Pacific/Kiritimati" ""
    "Asia/Pyongyang" ""
    "Asia/Seoul" ""
    "Asia/Almaty" ""
    "Asia/Qyzylorda" "Kyzylorda"
    "Asia/Qostanay" "Kostanay"
    "Asia/Aqtobe" ""
    "Asia/Aqtau" "Aktau"
    "Asia/Atyrau" ""
    "Asia/Oral" ""
    "Asia/Beirut" ""
    "Asia/Colombo" ""
    "Africa/Monrovia" ""
    "Europe/Vilnius" ""
    "Europe/Riga" ""
    "Africa/Tripoli" ""
    "Africa/Casablanca" ""
    "Europe/Chisinau" ""
    "Pacific/Kwajalein" "Kwajalein Atoll"
    "Asia/Yangon" ""
    "Asia/Ulaanbaatar" ""
    "Asia/Hovd" ""
    "Asia/Macau" ""
    "America/Martinique" ""
    "Europe/Malta" ""
    "Indian/Mauritius" ""
    "Indian/Maldives" ""
    "America/Mexico_City" "Mexico City"
    "America/Cancun" ""
    "America/Merida" ""
    "America/Monterrey" ""
    "America/Matamoros" ""
    "America/Chihuahua" ""
    "America/Ciudad_Juarez" ""
    "America/Ojinaga" ""
    "America/Mazatlan" ""
    "America/Bahia_Banderas" ""
    "America/Hermosillo" ""
    "America/Tijuana" ""
    "Asia/Kuching" ""
    "Africa/Maputo" ""
    "Africa/Windhoek" ""
    "Pacific/Noumea" ""
    "Pacific/Norfolk" "Norfolk Island"
    "Africa/Lagos" ""
    "America/Managua" ""
    "Asia/Kathmandu" ""
    "Pacific/Nauru" ""
    "Pacific/Niue" ""
    "Pacific/Auckland" ""
    "Pacific/Chatham" "Chatham Islands"
    "America/Panama" ""
    "America/Lima" ""
    "Pacific/Tahiti" ""
    "Pacific/Marquesas" "Marquesas Islands"
    "Pacific/Gambier" ""
    "Pacific/Port_Moresby" ""
    "Pacific/Bougainville" ""
    "Asia/Manila" ""
    "Asia/Karachi" ""
    "Europe/Warsaw" ""
    "America/Miquelon" ""
    "Pacific/Pitcairn" "Pitcairn Islands"
    "America/Puerto_Rico" ""
    "Asia/Gaza" ""
    "Asia/Hebron" ""
    "Europe/Lisbon" ""
    "Atlantic/Madeira" ""
    "Atlantic/Azores" ""
    "Pacific/Palau" ""
    "America/Asuncion" ""
    "Asia/Qatar" ""
    "Europe/Bucharest" ""
    "Europe/Belgrade" ""
    "Europe/Kaliningrad" ""
    "Europe/Moscow" ""
    "Europe/Simferopol" ""
    "Europe/Kirov" ""
    "Europe/Volgograd" ""
    "Europe/Astrakhan" ""
    "Europe/Saratov" ""
    "Europe/Ulyanovsk" ""
    "Europe/Samara" ""
    "Asia/Yekaterinburg" ""
    "Asia/Omsk" ""
    "Asia/Novosibirsk" ""
    "Asia/Barnaul" ""
    "Asia/Tomsk" ""
    "Asia/Novokuznetsk" ""
    "Asia/Krasnoyarsk" ""
    "Asia/Irkutsk" ""
    "Asia/Chita" ""
    "Asia/Yakutsk" ""
    "Asia/Khandyga" ""
    "Asia/Vladivostok" ""
    "Asia/Ust-Nera" ""
    "Asia/Magadan" ""
    "Asia/Sakhalin" ""
    "Asia/Srednekolymsk" ""
    "Asia/Kamchatka" ""
    "Asia/Anadyr" ""
    "Asia/Riyadh" ""
    "Pacific/Guadalcanal" ""
    "Africa/Khartoum" ""
    "Asia/Singapore" ""
    "America/Paramaribo" ""
    "Africa/Juba" ""
    "Africa/Sao_Tome" ""
    "America/El_Salvador" ""
    "Asia/Damascus" ""
    "America/Grand_Turk" ""
    "Africa/Ndjamena" ""
    "Asia/Bangkok" ""
    "Asia/Dushanbe" ""
    "Pacific/Fakaofo" ""
    "Asia/Dili" ""
    "Asia/Ashgabat" ""
    "Africa/Tunis" ""
    "Pacific/Tongatapu" ""
    "Europe/Istanbul" ""
    "Asia/Taipei" ""
    "Europe/Kyiv" ""
    "America/New_York" ""
    "America/Detroit" ""
    "America/Kentucky/Louisville" ""
    "America/Kentucky/Monticello" ""
    "America/Indiana/Indianapolis" ""
    "America/Indiana/Vincennes" ""
    "America/Indiana/Winamac" ""
    "America/Indiana/Marengo" ""
    "America/Indiana/Petersburg" ""
    "America/Indiana/Vevay" ""
    "America/Chicago" ""
    "America/Indiana/Tell_City" ""
    "America/Indiana/Knox" ""
    "America/Menominee" ""
    "America/North_Dakota/Center" ""
    "America/North_Dakota/New_Salem" ""
    "America/North_Dakota/Beulah" ""
    "America/Denver" ""
    "America/Boise" ""
    "America/Phoenix" ""
    "America/Los_Angeles" ""
    "America/Anchorage" ""
    "America/Juneau" ""
    "America/Sitka" ""
    "America/Metlakatla" ""
    "America/Yakutat" ""
    "America/Nome" ""
    "America/Adak" ""
    "Pacific/Honolulu" ""
    "America/Montevideo" ""
    "Asia/Samarkand" ""
    "Asia/Tashkent" ""
    "America/Caracas" ""
    "Asia/Ho_Chi_Minh" "Ho Chi Minh City"
    "Pacific/Efate" ""
    "Pacific/Apia" ""
    "Africa/Johannesburg" ""
    "America/Antigua" ""
    "America/Anguilla" ""
    "Africa/Luanda" ""
    "Antarctica/McMurdo" "McMurdo Station"
    "Antarctica/DumontDUrville" "Dumont d’Urville Station"
    "Antarctica/Syowa" "Showa Station"
    "America/Aruba" ""
    "Europe/Mariehamn" ""
    "Europe/Sarajevo" ""
    "Africa/Ouagadougou" ""
    "Asia/Bahrain" ""
    "Africa/Bujumbura" ""
    "Africa/Porto-Novo" ""
    "America/St_Barthelemy" ""
    "Asia/Brunei" ""
    "America/Kralendijk" ""
    "America/Nassau" ""
    "Africa/Gaborone" ""
    "America/Blanc-Sablon" ""
    "America/Atikokan" ""
    "America/Creston" ""
    "Indian/Cocos" "Cocos Islands"
    "Africa/Kinshasa" ""
    "Africa/Lubumbashi" ""
    "Africa/Bangui" ""
    "Africa/Brazzaville" ""
    "Africa/Douala" ""
    "America/Curacao" ""
    "Indian/Christmas" "Christmas Island"
    "Europe/Busingen" ""
    "Africa/Djibouti" ""
    "Europe/Copenhagen" ""
    "America/Dominica" ""
    "Africa/Asmara" ""
    "Africa/Addis_Ababa" ""
    "Pacific/Chuuk" ""
    "Pacific/Pohnpei" ""
    "Africa/Libreville" ""
    "America/Grenada" ""
    "Europe/Guernsey" ""
    "Africa/Accra" ""
    "Africa/Banjul" ""
    "Africa/Conakry" ""
    "America/Guadeloupe" ""
    "Africa/Malabo" ""
    "Europe/Zagreb" ""
    "Europe/Isle_of_Man" ""
    "Atlantic/Reykjavik" ""
    "Europe/Jersey" ""
    "Asia/Phnom_Penh" ""
    "Indian/Comoro" "Comoros"
    "America/St_Kitts" ""
    "Asia/Kuwait" ""
    "America/Cayman" ""
    "Asia/Vientiane" ""
    "America/St_Lucia" ""
    "Europe/Vaduz" ""
    "Africa/Maseru" ""
    "Europe/Luxembourg" ""
    "Europe/Monaco" ""
    "Europe/Podgorica" ""
    "America/Marigot" ""
    "Indian/Antananarivo" ""
    "Pacific/Majuro" ""
    "Europe/Skopje" ""
    "Africa/Bamako" ""
    "Pacific/Saipan" ""
    "Africa/Nouakchott" ""
    "America/Montserrat" ""
    "Africa/Blantyre" ""
    "Asia/Kuala_Lumpur" ""
    "Africa/Niamey" ""
    "Europe/Amsterdam" ""
    "Europe/Oslo" ""
    "Asia/Muscat" ""
    "Indian/Reunion" ""
    "Africa/Kigali" ""
    "Indian/Mahe" ""
    "Europe/Stockholm" ""
    "Atlantic/St_Helena" ""
    "Europe/Ljubljana" ""
    "Arctic/Longyearbyen" ""
    "Europe/Bratislava" ""
    "Africa/Freetown" ""
    "Europe/San_Marino" ""
    "Africa/Dakar" ""
    "Africa/Mogadishu" ""
    "America/Lower_Princes" ""
    "Africa/Mbabane" ""
    "Indian/Kerguelen" "Kerguelen Islands"
    "Africa/Lome" ""
    "America/Port_of_Spain" ""
    "Pacific/Funafuti" ""
    "Africa/Dar_es_Salaam" ""
    "Africa/Kampala" ""
    "Pacific/Midway" "Midway Atoll"
    "Pacific/Wake" "Wake Island"
    "Europe/Vatican" ""
    "America/St_Vincent" ""
    "America/Tortola" ""
    "America/St_Thomas" ""
    "Pacific/Wallis" ""
    "Asia/Aden" ""
    "Indian/Mayotte" ""
    "Africa/Lusaka" ""
    "Africa/Harare" ""
};

// `common/main/am.xml`: 417 of the 418 zones named, 1 inherited.
#[cfg(feature = "localized-exemplar-cities")]
const AM: &str = exemplar_cities! {
    "Europe/Andorra" "አንዶራ"
    "Asia/Dubai" "ዱባይ"
    "Asia/Kabul" "ካቡል"
    "Europe/Tirane" "ቴራን"
    "Asia/Yerevan" "ይሬቫን"
    "Antarctica/Casey" "ካዚይ"
    "Antarctica/Davis" "ዳቪስ"
    "Antarctica/Mawson" "ናውሰን"
    "Antarctica/Palmer" "ፓልመር"
    "Antarctica/Rothera" "ሮቴራ"
    "Antarctica/Troll" "ትሮል"
    "Antarctica/Vostok" "ቭስቶክ"
    "America/Argentina/Buenos_Aires" "ቦነስ አይረስ"
    "America/Argentina/Cordoba" "ኮርዶባ"
    "America/Argentina/Salta" "ሳልታ"
    "America/Argentina/Jujuy" "ጁጁይ"
    "America/Argentina/Tucuman" "ቱኩማን"
    "America/Argentina/Catamarca" "ካታማርካ"
    "America/Argentina/La_Rioja" "ላ ሪኦጃ"
    "America/Argentina/San_Juan" "ሳን ጁአን"
    "America/Argentina/Mendoza" "ሜንዶዛ"
    "America/Argentina/San_Luis" "ሳን ሊውስ"
    "America/Argentina/Rio_Gallegos" "ሪዮ ጋሌጎስ"
    "America/Argentina/Ushuaia" "ኡሹአኢ"
    "Pacific/Pago_Pago" "ፓጎ ፓጎ"
    "Europe/Vienna" "ቪየና"
    "Australia/Lord_Howe" "ሎርድ ሆዊ"
    "Antarctica/Macquarie" "ማከሪ"
    "Australia/Hobart" "ሆባርት"
    "Australia/Melbourne" "ሜልቦርን"
    "Australia/Sydney" "ሲድኒ"
    "Australia/Broken_Hill" "ብሮክን ሂል"
    "Australia/Brisbane" "ብሪስቤን"
    "Australia/Lindeman" "ሊንድማን"
    "Australia/Adelaide" "አዴሌእድ"
    "Australia/Darwin" "ዳርዊን"
    "Australia/Perth" "ፐርዝ"
    "Australia/Eucla" "ኡክላ"
    "Asia/Baku" "ባኩ"
    "America/Barbados" "ባርቤዶስ"
    "Asia/Dhaka" "ዳካ"
    "Europe/Brussels" "ብራሰልስ"
    "Europe/Sofia" "ሶፊያ"
    "Atlantic/Bermuda" "ቤርሙዳ"
    "America/La_Paz" "ላ ፓዝ"
    "America/Noronha" "ኖሮኛ"
    "America/Belem" "ቤለም"
    "America/Fortaleza" "ፎርታሌዛ"
    "America/Recife" "ረሲፍ"
    "America/Araguaina" "አራጉየና"
    "America/Maceio" "ሜሲኦ"
    "America/Bahia" "ባሂአ"
    "America/Sao_Paulo" "ሳኦ ፖሎ"
    "America/Campo_Grande" "ካምፖ ግራንዴ"
    "America/Cuiaba" "ኩየአባ"
    "America/Santarem" "ሳንታሬም"
    "America/Porto_Velho" "ፔትሮ ቬልሆ"
    "America/Boa_Vista" "ቦአ ቪስታ"
    "America/Manaus" "ማናኡስ"
    "America/Eirunepe" "ኢሩኔፕ"
    "America/Rio_Branco" "ሪዮ ብራንኮ"
    "Asia/Thimphu" "ቲምፉ"
    "Europe/Minsk" "ሚንስክ"
    "America/Belize" "ቤሊዝ"
    "America/St_Johns" "ቅዱስ ዮሐንስ"
    "America/Halifax" "ሃሊፋክስ"
    "America/Glace_Bay" "ግሌስ ቤይ"
    "America/Moncton" "ሞንክቶን"
    "America/Goose_Bay" "ጉዝ ቤይ"
    "America/Toronto" "ቶሮንቶ"
    "America/Iqaluit" "ኢኳሊውት"
    "America/Winnipeg" "ዊኒፔግ"
    "America/Resolute" "ሪዞሊዩት"
    "America/Rankin_Inlet" "ራንኪን ኢንሌት"
    "America/Regina" "ረጂና"
    "America/Swift_Current" "የሐዋላ ገንዘብ"
    "America/Edmonton" "ኤድመንተን"
    "America/Cambridge_Bay" "ካምብሪጅ ቤይ"
    "America/Inuvik" "ኢኑቪክ"
    "America/Vancouver" "ቫንኮቨር"
    "America/Dawson_Creek" "ዳውሰን ክሬክ"
    "America/Fort_Nelson" "ፎርት ኔልሰን"
    "America/Whitehorse" "ኋይትሆርስ"
    "America/Dawson" "ዳውሰን"
    "Europe/Zurich" "ዙሪክ"
    "Africa/Abidjan" "አቢጃን"
    "Pacific/Rarotonga" "ራሮቶንጋ"
    "America/Santiago" "ሳንቲያጎ"
    "America/Coyhaique" inherited
    "America/Punta_Arenas" "ፑንታ አሬናስ"
    "Pacific/Easter" "ፋሲካ"
    "Asia/Shanghai" "ሻንጋይ"
    "Asia/Urumqi" "ኡሩምኪ"
    "America/Bogota" "ቦጎታ"
    "America/Costa_Rica" "ኮስታሪካ"
    "America/Havana" "ሃቫና"
    "Atlantic/Cape_Verde" "ኬፕ ቬርደ"
    "Asia/Nicosia" "ኒኮሲአ"
    "Asia/Famagusta" "ፋማጉስታ"
    "Europe/Prague" "ፕራግ"
    "Europe/Berlin" "በርሊን"
    "America/Santo_Domingo" "ሳንቶ ዶሚንጎ"
    "Africa/Algiers" "አልጀርስ"
    "America/Guayaquil" "ጉያኩይል"
    "Pacific/Galapagos" "ጋላፓጎስ"
    "Europe/Tallinn" "ታሊን"
    "Africa/Cairo" "ካይሮ"
    "Africa/El_Aaiun" "ኤል አዩአን"
    "Europe/Madrid" "ማድሪድ"
    "Africa/Ceuta" "ሲኡታ"
    "Atlantic/Canary" "ካናሪ"
    "Europe/Helsinki" "ሄልሲንኪ"
    "Pacific/Fiji" "ፊጂ"
    "Atlantic/Stanley" "ስታንሌይ"
    "Pacific/Kosrae" "ኮስሬ"
    "Atlantic/Faroe" "ፋሮእ"
    "Europe/Paris" "ፓሪስ"
    "Europe/London" "ለንደን"
    "Asia/Tbilisi" "ትብሊሲ"
    "America/Cayenne" "ካይንኤ"
    "Europe/Gibraltar" "ጂብራልታር"
    "America/Nuuk" "ጋድታብ"
    "America/Danmarkshavn" "ዳንማርክሻቭን"
    "America/Scoresbysund" "ስኮርስባይሰንድ"
    "America/Thule" "ቱሌ"
    "Europe/Athens" "አቴንስ"
    "Atlantic/South_Georgia" "ደቡብ ጆርጂያ"
    "America/Guatemala" "ጓቲማላ"
    "Pacific/Guam" "ጉአም"
    "Africa/Bissau" "ቢሳኦ"
    "America/Guyana" "ጉያና"
    "Asia/Hong_Kong" "ሆንግ ኮንግ"
    "America/Tegucigalpa" "ቴጉሲጋልፓ"
    "America/Port-au-Prince" "ፖርት ኦ ፕሪንስ"
    "Europe/Budapest" "ቡዳፔስት"
    "Asia/Jakarta" "ጃካርታ"
    "Asia/Pontianak" "ፖንቲአናክ"
    "Asia/Makassar" "ማካሳር"
    "Asia/Jayapura" "ጃያፑራ"
    "Europe/Dublin" "ደብሊን"
    "Asia/Jerusalem" "እየሩሳሌም"
    "Asia/Kolkata" "ኮልካታ"
    "Indian/Chagos" "ቻጎስ"
    "Asia/Baghdad" "ባግዳድ"
    "Asia/Tehran" "ቴህራን"
    "Europe/Rome" "ሮም"
    "America/Jamaica" "ጃማይካ"
    "Asia/Amman" "አማን"
    "Asia/Tokyo" "ቶኪዮ"
    "Africa/Nairobi" "ናይሮቢ"
    "Asia/Bishkek" "ቢሽኬክ"
    "Pacific/Tarawa" "ታራዋ"
    "Pacific/Kanton" "ካንቶን"
    "Pacific/Kiritimati" "ኪሪቲማቲ"
    "Asia/Pyongyang" "ፕዮንግያንግ"
    "Asia/Seoul" "ሴኦል"
    "Asia/Almaty" "አልማትይ"
    "Asia/Qyzylorda" "ኩይዚሎርዳ"
    "Asia/Qostanay" "ኮስታናይ"
    "Asia/Aqtobe" "አኩቶቤ"
    "Asia/Aqtau" "አኩታኡ"
    "Asia/Atyrau" "አትይራኡ"
    "Asia/Oral" "ኦራል"
    "Asia/Beirut" "ቤሩት"
    "Asia/Colombo" "ኮሎምቦ"
    "Africa/Monrovia" "ሞንሮቪያ"
    "Europe/Vilnius" "ቪሊነስ"
    "Europe/Riga" "ሪጋ"
    "Africa/Tripoli" "ትሪፖሊ"
    "Africa/Casablanca" "ካዛብላንካ"
    "Europe/Chisinau" "ቺስናኡ"
    "Pacific/Kwajalein" "ክዋጃሊን"
    "Asia/Yangon" "ያንጎን"
    "Asia/Ulaanbaatar" "ኡላአንባአታር"
    "Asia/Hovd" "ሆቭድ"
    "Asia/Macau" "ማካኡ"
    "America/Martinique" "ማርቲኒክ"
    "Europe/Malta" "ማልታ"
    "Indian/Mauritius" "ሞሪሽየስ"
    "Indian/Maldives" "ማልዲቨ"
    "America/Mexico_City" "ሜክሲኮ ከተማ"
    "America/Cancun" "ካንኩን"
    "America/Merida" "ሜሪዳ"
    "America/Monterrey" "ሞንተርሬይ"
    "America/Matamoros" "ማታሞሮስ"
    "America/Chihuahua" "ቺሁዋውአ"
    "America/Ciudad_Juarez" "ሳዮዳድ ሁዋሬዝ"
    "America/Ojinaga" "ኦዪናጋ"
    "America/Mazatlan" "ማዛትላን"
    "America/Bahia_Banderas" "ባሂያ ባንደራስ"
    "America/Hermosillo" "ኸርሞዚሎ"
    "America/Tijuana" "ቲጁአና"
    "Asia/Kuching" "ኩቺንግ"
    "Africa/Maputo" "ማፑቱ"
    "Africa/Windhoek" "ዊንድሆክ"
    "Pacific/Noumea" "ናኦሚአ"
    "Pacific/Norfolk" "ኖርፎልክ"
    "Africa/Lagos" "ሌጎስ"
    "America/Managua" "ማናጉአ"
    "Asia/Kathmandu" "ካትማንዱ"
    "Pacific/Nauru" "ናውሩ"
    "Pacific/Niue" "ኒዌ"
    "Pacific/Auckland" "ኦክላንድ"
    "Pacific/Chatham" "ቻታም"
    "America/Panama" "ፓናማ"
    "America/Lima" "ሊማ"
    "Pacific/Tahiti" "ታሂቲ"
    "Pacific/Marquesas" "ማርክዌሳስ"
    "Pacific/Gambier" "ጋምቢየር"
    "Pacific/Port_Moresby" "ፖርት ሞሬስባይ"
    "Pacific/Bougainville" "ቦጌይንቪል"
    "Asia/Manila" "ማኒላ"
    "Asia/Karachi" "ካራቺ"
    "Europe/Warsaw" "ዋርሶው"
    "America/Miquelon" "ሚኮውሎን"
    "Pacific/Pitcairn" "ፒትከይርን"
    "America/Puerto_Rico" "ፖርቶሪኮ"
    "Asia/Gaza" "ጋዛ"
    "Asia/Hebron" "ኬብሮን"
    "Europe/Lisbon" "ሊዝበን"
    "Atlantic/Madeira" "ማዴራ"
    "Atlantic/Azores" "አዞረስ"
    "Pacific/Palau" "ፓላው"
    "America/Asuncion" "አሱንሲዮን"
    "Asia/Qatar" "ኳታር"
    "Europe/Bucharest" "ቡካሬስት"
    "Europe/Belgrade" "ቤልግሬድ"
    "Europe/Kaliningrad" "ካሊኒንግራድ"
    "Europe/Moscow" "ሞስኮ"
    "Europe/Simferopol" "ሲምፈሮፖል"
    "Europe/Kirov" "ኪሮቭ"
    "Europe/Volgograd" "ቮልጎራድ"
    "Europe/Astrakhan" "አስትራክሃን"
    "Europe/Saratov" "ሳራቶቭ"
    "Europe/Ulyanovsk" "ኡልያኖቭስክ"
    "Europe/Samara" "ሳማራ"
    "Asia/Yekaterinburg" "የካተሪንበርግ"
    "Asia/Omsk" "ኦምስክ"
    "Asia/Novosibirsk" "ኖቮሲቢሪስክ"
    "Asia/Barnaul" "ባርናኡል"
    "Asia/Tomsk" "ቶምስክ"
    "Asia/Novokuznetsk" "ኖቮኩትዝኔክ"
    "Asia/Krasnoyarsk" "ክራስኖያርስክ"
    "Asia/Irkutsk" "ኢርኩትስክ"
    "Asia/Chita" "ቺታ"
    "Asia/Yakutsk" "ያኩትስክ"
    "Asia/Khandyga" "ካንዲጋ"
    "Asia/Vladivostok" "ቭላዲቮስቶክ"
    "Asia/Ust-Nera" "ኡስት-ኔራ"
    "Asia/Magadan" "ማጋዳን"
    "Asia/Sakhalin" "ሳክሃሊን"
    "Asia/Srednekolymsk" "ስሬድኔስኮልምስክ"
    "Asia/Kamchatka" "ካምቻትካ"
    "Asia/Anadyr" "አናድይር"
    "Asia/Riyadh" "ሪያድ"
    "Pacific/Guadalcanal" "ጉዋዳልካናል"
    "Africa/Khartoum" "ካርቱም"
    "Asia/Singapore" "ሲንጋፖር"
    "America/Paramaribo" "ፓራማሪቦ"
    "Africa/Juba" "ጁባ"
    "Africa/Sao_Tome" "ሳኦ ቶሜ"
    "America/El_Salvador" "ኤልሳልቫዶር"
    "Asia/Damascus" "ደማስቆ"
    "America/Grand_Turk" "ግራንድ ተርክ"
    "Africa/Ndjamena" "ንጃሜና"
    "Asia/Bangkok" "ባንኮክ"
    "Asia/Dushanbe" "ደሻንቤ"
    "Pacific/Fakaofo" "ፋካኦፎ"
    "Asia/Dili" "ዲሊ"
    "Asia/Ashgabat" "አሽጋባት"
    "Africa/Tunis" "ቱኒዝ"
    "Pacific/Tongatapu" "ቶንጋታፑ"
    "Europe/Istanbul" "ኢስታንቡል"
    "Asia/Taipei" "ታይፓይ"
    "Europe/Kyiv" "ኪየቭ"
    "America/New_York" "ኒውዮርክ"
    "America/Detroit" "ዲትሮይት"
    "America/Kentucky/Louisville" "ሊውስቪል"
    "America/Kentucky/Monticello" "ሞንቲሴሎ, ኪንታኪ"
    "America/Indiana/Indianapolis" "ኢንዲያናፖሊስ"
    "America/Indiana/Vincennes" "ቪንቼንስ, ኢንዲያና"
    "America/Indiana/Winamac" "ዊናማክ, ኢንዲያና"
    "America/Indiana/Marengo" "ማሬንጎ, ኢንዲያና"
    "America/Indiana/Petersburg" "ፒተርስበርግ, ኢንዲያና"
    "America/Indiana/Vevay" "ቪቫይ, ኢንዲያና"
    "America/Chicago" "ቺካጎ"
    "America/Indiana/Tell_City" "ቴል ከተማ, ኢንዲያና"
    "America/Indiana/Knox" "ኖክስ, ኢንዲያና"
    "America/Menominee" "ሜኖሚኒ"
    "America/North_Dakota/Center" "መካከለኛ, ሰሜን ዳኮታ"
    "America/North_Dakota/New_Salem" "አዲስ ሳሌም, ሰሜን ዳኮታ"
    "America/North_Dakota/Beulah" "ቤኡላህ, ሰሜን ዳኮታ"
    "America/Denver" "ዴንቨር"
    "America/Boise" "ቦይዝ"
    "America/Phoenix" "ፊኒክስ"
    "America/Los_Angeles" "ሎስ አንጀለስ"
    "America/Anchorage" "አንኮራጅ"
    "America/Juneau" "ጁኒዩ"
    "America/Sitka" "ሲትካ"
    "America/Metlakatla" "መትላካትላ"
    "America/Yakutat" "ያኩታት"
    "America/Nome" "ኖሜ"
    "America/Adak" "አዳክ"
    "Pacific/Honolulu" "ሆኖሉሉ"
    "America/Montevideo" "ሞንቴቪድዮ"
    "Asia/Samarkand" "ሳማርካንድ"
    "Asia/Tashkent" "ታሽኬንት"
    "America/Caracas" "ካራካስ"
    "Asia/Ho_Chi_Minh" "ሆ ቺ ሚንህ ከተማ"
    "Pacific/Efate" "ኢፋቴ"
    "Pacific/Apia" "አፒአ"
    "Africa/Johannesburg" "ጆሃንስበርግ"
    "America/Antigua" "አንቲጓ"
    "America/Anguilla" "አንጉይላ"
    "Africa/Luanda" "ሉአንዳ"
    "Antarctica/McMurdo" "ማክመርዶ"
    "Antarctica/DumontDUrville" "ደሞንት ዲኡርቪል"
    "Antarctica/Syowa" "ስዮዋ"
    "America/Aruba" "አሩባ"
    "Europe/Mariehamn" "ሜሪሃምን"
    "Europe/Sarajevo" "ሳሪየቮ"
    "Africa/Ouagadougou" "ኡጋዱጉ"
    "Asia/Bahrain" "ባህሬን"
    "Africa/Bujumbura" "ቡጁምብራ"
    "Africa/Porto-Novo" "ፖርቶ - ኖቮ"
    "America/St_Barthelemy" "ቅድስት ቤርተሎሜ"
    "Asia/Brunei" "ብሩናይ"
    "America/Kralendijk" "ክራለንዲይክ"
    "America/Nassau" "ናሳው"
    "Africa/Gaborone" "ጋቦሮን"
    "America/Blanc-Sablon" "ብላንክ- ሳብሎን"
    "America/Atikokan" "አቲኮካን"
    "America/Creston" "ክረስተን"
    "Indian/Cocos" "ኮኮስ"
    "Africa/Kinshasa" "ኪንሻሳ"
    "Africa/Lubumbashi" "ሉቡምባሺ"
    "Africa/Bangui" "ባንጉኢ"
    "Africa/Brazzaville" "ብራዛቪል"
    "Africa/Douala" "ዱአላ"
    "America/Curacao" "ኩራሳዎ"
    "Indian/Christmas" "ገና"
    "Europe/Busingen" "ቡሲንገን"
    "Africa/Djibouti" "ጅቡቲ"
    "Europe/Copenhagen" "ኮፐንሃገን"
    "America/Dominica" "ዶሜኒካ"
    "Africa/Asmara" "አስመራ"
    "Africa/Addis_Ababa" "አዲስ አበባ"
    "Pacific/Chuuk" "ቹክ"
    "Pacific/Pohnpei" "ፖህንፔ"
    "Africa/Libreville" "ሊበርቪል"
    "America/Grenada" "ግሬናዳ"
    "Europe/Guernsey" "ጉርነሲ"
    "Africa/Accra" "አክራ"
    "Africa/Banjul" "ባንጁል"
    "Africa/Conakry" "ኮናክሬ"
    "America/Guadeloupe" "ጕዳሉፕ"
    "Africa/Malabo" "ማላቡ"
    "Europe/Zagreb" "ዛግሬብ"
    "Europe/Isle_of_Man" "አይስል ኦፍ ማን"
    "Atlantic/Reykjavik" "ሬይክጃቪክ"
    "Europe/Jersey" "ጀርሲ"
    "Asia/Phnom_Penh" "ፍኖም ፔንህ"
    "Indian/Comoro" "ኮሞሮ"
    "America/St_Kitts" "ቅዱስ ኪትስ"
    "Asia/Kuwait" "ኩዌት"
    "America/Cayman" "ካይማን"
    "Asia/Vientiane" "ቬንቲአን"
    "America/St_Lucia" "ቅድስት ሉሲያ"
    "Europe/Vaduz" "ቫዱዝ"
    "Africa/Maseru" "ማሴሩ"
    "Europe/Luxembourg" "ሉክሰምበርግ"
    "Europe/Monaco" "ሞናኮ"
    "Europe/Podgorica" "ፖድጎሪካ"
    "America/Marigot" "ማርጎት"
    "Indian/Antananarivo" "አንታናናሪቮ"
    "Pacific/Majuro" "ማጁሩ"
    "Europe/Skopje" "ስኮፕየ"
    "Africa/Bamako" "ባማኮ"
    "Pacific/Saipan" "ሴይፓን"
    "Africa/Nouakchott" "ኑአክቾት"
    "America/Montserrat" "ሞንትሴራት"
    "Africa/Blantyre" "ብላንታየር"
    "Asia/Kuala_Lumpur" "ኩዋላ ላምፑር"
    "Africa/Niamey" "ኒያሜይ"
    "Europe/Amsterdam" "አምስተርዳም"
    "Europe/Oslo" "ኦስሎ"
    "Asia/Muscat" "ሙስካት"
    "Indian/Reunion" "ሬዩኒየን"
    "Africa/Kigali" "ኪጋሊ"
    "Indian/Mahe" "ማሄ"
    "Europe/Stockholm" "ስቶክሆልም"
    "Atlantic/St_Helena" "ቅድስት ሄለና"
    "Europe/Ljubljana" "ልጁብልጃና"
    "Arctic/Longyearbyen" "ሎንግይርባየን"
    "Europe/Bratislava" "ብራቲስላቫ"
    "Africa/Freetown" "ፍሪታውን"
    "Europe/San_Marino" "ሳን ማሪኖ"
    "Africa/Dakar" "ዳካር"
    "Africa/Mogadishu" "ሞቃዲሹ"
    "America/Lower_Princes" "የታችኛው ልዑል ሩብ"
    "Africa/Mbabane" "ምባባኔ"
    "Indian/Kerguelen" "ኬርጉለን"
    "Africa/Lome" "ሎሜ"
    "America/Port_of_Spain" "የእስፔን ወደብ"
    "Pacific/Funafuti" "ፈናፉቲ"
    "Africa/Dar_es_Salaam" "ዳሬ ሰላም"
    "Africa/Kampala" "ካምፓላ"
    "Pacific/Midway" "ሚድወይ"
    "Pacific/Wake" "ዋኬ"
    "Europe/Vatican" "ቫቲካን"
    "America/St_Vincent" "ቅዱስ ቪንሰንት"
    "America/Tortola" "ቶርቶላ"
    "America/St_Thomas" "ቅዱስ ቶማስ"
    "Pacific/Wallis" "ዋሊስ"
    "Asia/Aden" "ኤደን"
    "Indian/Mayotte" "ማዮቴ"
    "Africa/Lusaka" "ሉሳካ"
    "Africa/Harare" "ሃራሬ"
};

// `common/main/ar.xml`: 418 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const AR: &str = exemplar_cities! {
    "Europe/Andorra" "أندورا"
    "Asia/Dubai" "دبي"
    "Asia/Kabul" "كابول"
    "Europe/Tirane" "تيرانا"
    "Asia/Yerevan" "يريفان"
    "Antarctica/Casey" "كاساي"
    "Antarctica/Davis" "دافيز"
    "Antarctica/Mawson" "ماوسون"
    "Antarctica/Palmer" "بالمير"
    "Antarctica/Rothera" "روثيرا"
    "Antarctica/Troll" "ترول"
    "Antarctica/Vostok" "فوستوك"
    "America/Argentina/Buenos_Aires" "بوينوس أيرس"
    "America/Argentina/Cordoba" "كوردوبا"
    "America/Argentina/Salta" "سالطا"
    "America/Argentina/Jujuy" "جوجو"
    "America/Argentina/Tucuman" "تاكمان"
    "America/Argentina/Catamarca" "كاتاماركا"
    "America/Argentina/La_Rioja" "لا ريوجا"
    "America/Argentina/San_Juan" "سان خوان"
    "America/Argentina/Mendoza" "ميندوزا"
    "America/Argentina/San_Luis" "سان لويس"
    "America/Argentina/Rio_Gallegos" "ريو جالييوس"
    "America/Argentina/Ushuaia" "أشوا"
    "Pacific/Pago_Pago" "باغو باغو"
    "Europe/Vienna" "فيينا"
    "Australia/Lord_Howe" "لورد هاو"
    "Antarctica/Macquarie" "ماكواري"
    "Australia/Hobart" "هوبارت"
    "Australia/Melbourne" "ميلبورن"
    "Australia/Sydney" "سيدني"
    "Australia/Broken_Hill" "بروكن هيل"
    "Australia/Brisbane" "برسيبان"
    "Australia/Lindeman" "ليندمان"
    "Australia/Adelaide" "أديليد"
    "Australia/Darwin" "دارون"
    "Australia/Perth" "برثا"
    "Australia/Eucla" "أوكلا"
    "Asia/Baku" "باكو"
    "America/Barbados" "بربادوس"
    "Asia/Dhaka" "دكا"
    "Europe/Brussels" "بروكسل"
    "Europe/Sofia" "صوفيا"
    "Atlantic/Bermuda" "برمودا"
    "America/La_Paz" "لا باز"
    "America/Noronha" "نوروناه"
    "America/Belem" "بلم"
    "America/Fortaleza" "فورتاليزا"
    "America/Recife" "ريسيف"
    "America/Araguaina" "أروجوانيا"
    "America/Maceio" "ماشيو"
    "America/Bahia" "باهيا"
    "America/Sao_Paulo" "ساو باولو"
    "America/Campo_Grande" "كومبو جراند"
    "America/Cuiaba" "كيابا"
    "America/Santarem" "سانتاريم"
    "America/Porto_Velho" "بورتو فيلو"
    "America/Boa_Vista" "باو فيستا"
    "America/Manaus" "ماناوس"
    "America/Eirunepe" "ايرونبي"
    "America/Rio_Branco" "ريوبرانكو"
    "Asia/Thimphu" "تيمفو"
    "Europe/Minsk" "مينسك"
    "America/Belize" "بليز"
    "America/St_Johns" "سانت جونس"
    "America/Halifax" "هاليفاكس"
    "America/Glace_Bay" "جلاس باي"
    "America/Moncton" "وينكتون"
    "America/Goose_Bay" "جوس باي"
    "America/Toronto" "تورونتو"
    "America/Iqaluit" "اكويلت"
    "America/Winnipeg" "وينيبيج"
    "America/Resolute" "ريزولوت"
    "America/Rankin_Inlet" "رانكن انلت"
    "America/Regina" "ريجينا"
    "America/Swift_Current" "سوفت كارنت"
    "America/Edmonton" "ايدمونتون"
    "America/Cambridge_Bay" "كامبرديج باي"
    "America/Inuvik" "اينوفيك"
    "America/Vancouver" "فانكوفر"
    "America/Dawson_Creek" "داوسن كريك"
    "America/Fort_Nelson" "فورت نيلسون"
    "America/Whitehorse" "وايت هورس"
    "America/Dawson" "داوسان"
    "Europe/Zurich" "زيورخ"
    "Africa/Abidjan" "أبيدجان"
    "Pacific/Rarotonga" "راروتونغا"
    "America/Santiago" "سانتياغو"
    "America/Coyhaique" "كويهايكيو"
    "America/Punta_Arenas" "بونتا أريناز"
    "Pacific/Easter" "استر"
    "Asia/Shanghai" "شنغهاي"
    "Asia/Urumqi" "أرومكي"
    "America/Bogota" "بوغوتا"
    "America/Costa_Rica" "كوستاريكا"
    "America/Havana" "هافانا"
    "Atlantic/Cape_Verde" "الرأس الأخضر"
    "Asia/Nicosia" "نيقوسيا"
    "Asia/Famagusta" "فاماغوستا"
    "Europe/Prague" "براغ"
    "Europe/Berlin" "برلين"
    "America/Santo_Domingo" "سانتو دومينغو"
    "Africa/Algiers" "الجزائر"
    "America/Guayaquil" "غواياكويل"
    "Pacific/Galapagos" "جلاباجوس"
    "Europe/Tallinn" "تالين"
    "Africa/Cairo" "القاهرة"
    "Africa/El_Aaiun" "العيون"
    "Europe/Madrid" "مدريد"
    "Africa/Ceuta" "سيتا"
    "Atlantic/Canary" "كناري"
    "Europe/Helsinki" "هلسنكي"
    "Pacific/Fiji" "فيجي"
    "Atlantic/Stanley" "استانلي"
    "Pacific/Kosrae" "كوسرا"
    "Atlantic/Faroe" "فارو"
    "Europe/Paris" "باريس"
    "Europe/London" "لندن"
    "Asia/Tbilisi" "تبليسي"
    "America/Cayenne" "كايين"
    "Europe/Gibraltar" "جبل طارق"
    "America/Nuuk" "غودثاب"
    "America/Danmarkshavn" "دانمرك شافن"
    "America/Scoresbysund" "سكورسبيسند"
    "America/Thule" "ثيل"
    "Europe/Athens" "أثينا"
    "Atlantic/South_Georgia" "جورجيا الجنوبية"
    "America/Guatemala" "غواتيمالا"
    "Pacific/Guam" "غوام"
    "Africa/Bissau" "بيساو"
    "America/Guyana" "غيانا"
    "Asia/Hong_Kong" "هونغ كونغ"
    "America/Tegucigalpa" "تيغوسيغالبا"
    "America/Port-au-Prince" "بورت أو برنس"
    "Europe/Budapest" "بودابست"
    "Asia/Jakarta" "جاكرتا"
    "Asia/Pontianak" "بونتيانك"
    "Asia/Makassar" "ماكسار"
    "Asia/Jayapura" "جايابيورا"
    "Europe/Dublin" "دبلن"
    "Asia/Jerusalem" "القدس"
    "Asia/Kolkata" "كالكتا"
    "Indian/Chagos" "تشاغوس"
    "Asia/Baghdad" "بغداد"
    "Asia/Tehran" "طهران"
    "Europe/Rome" "روما"
    "America/Jamaica" "جامايكا"
    "Asia/Amman" "عمّان"
    "Asia/Tokyo" "طوكيو"
    "Africa/Nairobi" "نيروبي"
    "Asia/Bishkek" "بشكيك"
    "Pacific/Tarawa" "تاراوا"
    "Pacific/Kanton" "كانتون"
    "Pacific/Kiritimati" "كيريتي ماتي"
    "Asia/Pyongyang" "بيونغ يانغ"
    "Asia/Seoul" "سول"
    "Asia/Almaty" "ألماتي"
    "Asia/Qyzylorda" "كيزيلوردا"
    "Asia/Qostanay" "قوستاناي"
    "Asia/Aqtobe" "أكتوب"
    "Asia/Aqtau" "أكتاو"
    "Asia/Atyrau" "أتيراو"
    "Asia/Oral" "أورال"
    "Asia/Beirut" "بيروت"
    "Asia/Colombo" "كولومبو"
    "Africa/Monrovia" "مونروفيا"
    "Europe/Vilnius" "فيلنيوس"
    "Europe/Riga" "ريغا"
    "Africa/Tripoli" "طرابلس"
    "Africa/Casablanca" "الدار البيضاء"
    "Europe/Chisinau" "تشيسيناو"
    "Pacific/Kwajalein" "كواجالين"
    "Asia/Yangon" "رانغون"
    "Asia/Ulaanbaatar" "آلانباتار"
    "Asia/Hovd" "هوفد"
    "Asia/Macau" "ماكاو"
    "America/Martinique" "المارتينيك"
    "Europe/Malta" "مالطة"
    "Indian/Mauritius" "موريشيوس"
    "Indian/Maldives" "المالديف"
    "America/Mexico_City" "مكسيكو سيتي"
    "America/Cancun" "كانكون"
    "America/Merida" "ميريدا"
    "America/Monterrey" "مونتيري"
    "America/Matamoros" "ماتاموروس"
    "America/Chihuahua" "تشيواوا"
    "America/Ciudad_Juarez" "سيوداد خواريز"
    "America/Ojinaga" "أوجيناجا"
    "America/Mazatlan" "مازاتلان"
    "America/Bahia_Banderas" "باهيا بانديراس"
    "America/Hermosillo" "هيرموسيلو"
    "America/Tijuana" "تيخوانا"
    "Asia/Kuching" "كيشينج"
    "Africa/Maputo" "مابوتو"
    "Africa/Windhoek" "ويندهوك"
    "Pacific/Noumea" "نوميا"
    "Pacific/Norfolk" "نورفولك"
    "Africa/Lagos" "لاغوس"
    "America/Managua" "ماناغوا"
    "Asia/Kathmandu" "كاتماندو"
    "Pacific/Nauru" "ناورو"
    "Pacific/Niue" "نيوي"
    "Pacific/Auckland" "أوكلاند"
    "Pacific/Chatham" "تشاثام"
    "America/Panama" "بنما"
    "America/Lima" "ليما"
    "Pacific/Tahiti" "تاهيتي"
    "Pacific/Marquesas" "ماركيساس"
    "Pacific/Gambier" "جامبير"
    "Pacific/Port_Moresby" "بور مورسبي"
    "Pacific/Bougainville" "بوغانفيل"
    "Asia/Manila" "مانيلا"
    "Asia/Karachi" "كراتشي"
    "Europe/Warsaw" "وارسو"
    "America/Miquelon" "مكويلون"
    "Pacific/Pitcairn" "بيتكيرن"
    "America/Puerto_Rico" "بورتوريكو"
    "Asia/Gaza" "غزة"
    "Asia/Hebron" "هيبرون (مدينة الخليل)"
    "Europe/Lisbon" "لشبونة"
    "Atlantic/Madeira" "ماديرا"
    "Atlantic/Azores" "أزورس"
    "Pacific/Palau" "بالاو"
    "America/Asuncion" "أسونسيون"
    "Asia/Qatar" "قطر"
    "Europe/Bucharest" "بوخارست"
    "Europe/Belgrade" "بلغراد"
    "Europe/Kaliningrad" "كالينجراد"
    "Europe/Moscow" "موسكو"
    "Europe/Simferopol" "سيمفروبول"
    "Europe/Kirov" "كيروف"
    "Europe/Volgograd" "فولوجراد"
    "Europe/Astrakhan" "أستراخان"
    "Europe/Saratov" "ساراتوف"
    "Europe/Ulyanovsk" "أوليانوفسك"
    "Europe/Samara" "سمراء"
    "Asia/Yekaterinburg" "يكاترنبيرج"
    "Asia/Omsk" "أومسك"
    "Asia/Novosibirsk" "نوفوسبيرسك"
    "Asia/Barnaul" "بارناول"
    "Asia/Tomsk" "تومسك"
    "Asia/Novokuznetsk" "نوفوكوزنتسك"
    "Asia/Krasnoyarsk" "كراسنويارسك"
    "Asia/Irkutsk" "ايركيتسك"
    "Asia/Chita" "تشيتا"
    "Asia/Yakutsk" "ياكتسك"
    "Asia/Khandyga" "خانديجا"
    "Asia/Vladivostok" "فلاديفوستك"
    "Asia/Ust-Nera" "أوست نيرا"
    "Asia/Magadan" "مجادن"
    "Asia/Sakhalin" "سكالين"
    "Asia/Srednekolymsk" "سريدنكوليمسك"
    "Asia/Kamchatka" "كامتشاتكا"
    "Asia/Anadyr" "أندير"
    "Asia/Riyadh" "الرياض"
    "Pacific/Guadalcanal" "غوادالكانال"
    "Africa/Khartoum" "الخرطوم"
    "Asia/Singapore" "سنغافورة"
    "America/Paramaribo" "باراماريبو"
    "Africa/Juba" "جوبا"
    "Africa/Sao_Tome" "ساو تومي"
    "America/El_Salvador" "السلفادور"
    "Asia/Damascus" "دمشق"
    "America/Grand_Turk" "غراند ترك"
    "Africa/Ndjamena" "نجامينا"
    "Asia/Bangkok" "بانكوك"
    "Asia/Dushanbe" "دوشانبي"
    "Pacific/Fakaofo" "فاكاوفو"
    "Asia/Dili" "ديلي"
    "Asia/Ashgabat" "عشق آباد"
    "Africa/Tunis" "تونس"
    "Pacific/Tongatapu" "تونغاتابو"
    "Europe/Istanbul" "إسطنبول"
    "Asia/Taipei" "تايبيه"
    "Europe/Kyiv" "كييف"
    "America/New_York" "نيويورك"
    "America/Detroit" "ديترويت"
    "America/Kentucky/Louisville" "لويس فيل"
    "America/Kentucky/Monticello" "مونتيسيلو"
    "America/Indiana/Indianapolis" "إنديانابوليس"
    "America/Indiana/Vincennes" "فينسينس"
    "America/Indiana/Winamac" "ويناماك"
    "America/Indiana/Marengo" "مارنجو"
    "America/Indiana/Petersburg" "بيترسبرغ"
    "America/Indiana/Vevay" "فيفاي"
    "America/Chicago" "شيكاغو"
    "America/Indiana/Tell_City" "مدينة تل، إنديانا"
    "America/Indiana/Knox" "كونكس"
    "America/Menominee" "مينوميني"
    "America/North_Dakota/Center" "سنتر"
    "America/North_Dakota/New_Salem" "نيو ساليم"
    "America/North_Dakota/Beulah" "بيولا، داكوتا الشمالية"
    "America/Denver" "دنفر"
    "America/Boise" "بويس"
    "America/Phoenix" "فينكس"
    "America/Los_Angeles" "لوس انجلوس"
    "America/Anchorage" "أنشوراج"
    "America/Juneau" "جوني"
    "America/Sitka" "سيتكا"
    "America/Metlakatla" "ميتلاكاتلا"
    "America/Yakutat" "ياكوتات"
    "America/Nome" "نوم"
    "America/Adak" "أداك"
    "Pacific/Honolulu" "هونولولو"
    "America/Montevideo" "مونتفيديو"
    "Asia/Samarkand" "سمرقند"
    "Asia/Tashkent" "طشقند"
    "America/Caracas" "كاراكاس"
    "Asia/Ho_Chi_Minh" "مدينة هو تشي منة"
    "Pacific/Efate" "إيفات"
    "Pacific/Apia" "أبيا"
    "Africa/Johannesburg" "جوهانسبرغ"
    "America/Antigua" "أنتيغوا"
    "America/Anguilla" "أنغويلا"
    "Africa/Luanda" "لواندا"
    "Antarctica/McMurdo" "ماك موردو"
    "Antarctica/DumontDUrville" "دي مونت دو روفيل"
    "Antarctica/Syowa" "سايووا"
    "America/Aruba" "أروبا"
    "Europe/Mariehamn" "ماريهامن"
    "Europe/Sarajevo" "سراييفو"
    "Africa/Ouagadougou" "واغادوغو"
    "Asia/Bahrain" "البحرين"
    "Africa/Bujumbura" "بوجومبورا"
    "Africa/Porto-Novo" "بورتو نوفو"
    "America/St_Barthelemy" "سانت بارتيليمي"
    "Asia/Brunei" "بروناي"
    "America/Kralendijk" "كرالنديك"
    "America/Nassau" "ناسو"
    "Africa/Gaborone" "غابورون"
    "America/Blanc-Sablon" "بلانك-سابلون"
    "America/Atikokan" "كورال هاربر"
    "America/Creston" "كريستون"
    "Indian/Cocos" "كوكوس"
    "Africa/Kinshasa" "كينشاسا"
    "Africa/Lubumbashi" "لومبباشا"
    "Africa/Bangui" "بانغوي"
    "Africa/Brazzaville" "برازافيل"
    "Africa/Douala" "دوالا"
    "America/Curacao" "كوراساو"
    "Indian/Christmas" "كريسماس"
    "Europe/Busingen" "بوسنغن"
    "Africa/Djibouti" "جيبوتي"
    "Europe/Copenhagen" "كوبنهاغن"
    "America/Dominica" "دومينيكا"
    "Africa/Asmara" "أسمرة"
    "Africa/Addis_Ababa" "أديس أبابا"
    "Pacific/Chuuk" "ترك"
    "Pacific/Pohnpei" "باناب"
    "Africa/Libreville" "ليبرفيل"
    "America/Grenada" "غرينادا"
    "Europe/Guernsey" "غيرنزي"
    "Africa/Accra" "أكرا"
    "Africa/Banjul" "بانجول"
    "Africa/Conakry" "كوناكري"
    "America/Guadeloupe" "غوادلوب"
    "Africa/Malabo" "مالابو"
    "Europe/Zagreb" "زغرب"
    "Europe/Isle_of_Man" "جزيرة مان"
    "Atlantic/Reykjavik" "ريكيافيك"
    "Europe/Jersey" "جيرسي"
    "Asia/Phnom_Penh" "بنوم بنه"
    "Indian/Comoro" "جزر القمر"
    "America/St_Kitts" "سانت كيتس"
    "Asia/Kuwait" "الكويت"
    "America/Cayman" "كايمان"
    "Asia/Vientiane" "فيانتيان"
    "America/St_Lucia" "سانت لوشيا"
    "Europe/Vaduz" "فادوز"
    "Africa/Maseru" "ماسيرو"
    "Europe/Luxembourg" "لوكسمبورغ"
    "Europe/Monaco" "موناكو"
    "Europe/Podgorica" "بودغوريكا"
    "America/Marigot" "ماريغوت"
    "Indian/Antananarivo" "أنتاناناريفو"
    "Pacific/Majuro" "ماجورو"
    "Europe/Skopje" "سكوبي"
    "Africa/Bamako" "باماكو"
    "Pacific/Saipan" "سايبان"
    "Africa/Nouakchott" "نواكشوط"
    "America/Montserrat" "مونتسيرات"
    "Africa/Blantyre" "بلانتاير"
    "Asia/Kuala_Lumpur" "كوالا لامبور"
    "Africa/Niamey" "نيامي"
    "Europe/Amsterdam" "أمستردام"
    "Europe/Oslo" "أوسلو"
    "Asia/Muscat" "مسقط"
    "Indian/Reunion" "ريونيون"
    "Africa/Kigali" "كيغالي"
    "Indian/Mahe" "ماهي"
    "Europe/Stockholm" "ستوكهولم"
    "Atlantic/St_Helena" "سانت هيلينا"
    "Europe/Ljubljana" "ليوبليانا"
    "Arctic/Longyearbyen" "لونجيربين"
    "Europe/Bratislava" "براتيسلافا"
    "Africa/Freetown" "فري تاون"
    "Europe/San_Marino" "سان مارينو"
    "Africa/Dakar" "داكار"
    "Africa/Mogadishu" "مقديشيو"
    "America/Lower_Princes" "حي الأمير السفلي"
    "Africa/Mbabane" "مباباني"
    "Indian/Kerguelen" "كيرغويلين"
    "Africa/Lome" "لومي"
    "America/Port_of_Spain" "بورت أوف سبين"
    "Pacific/Funafuti" "فونافوتي"
    "Africa/Dar_es_Salaam" "دار السلام"
    "Africa/Kampala" "كامبالا"
    "Pacific/Midway" "ميدواي"
    "Pacific/Wake" "واك"
    "Europe/Vatican" "الفاتيكان"
    "America/St_Vincent" "سانت فنسنت"
    "America/Tortola" "تورتولا"
    "America/St_Thomas" "سانت توماس"
    "Pacific/Wallis" "واليس"
    "Asia/Aden" "عدن"
    "Indian/Mayotte" "مايوت"
    "Africa/Lusaka" "لوساكا"
    "Africa/Harare" "هراري"
};

// `common/main/bn.xml`: 418 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const BN: &str = exemplar_cities! {
    "Europe/Andorra" "অ্যান্ডোরা"
    "Asia/Dubai" "দুবাই"
    "Asia/Kabul" "কাবুল"
    "Europe/Tirane" "তিরানা"
    "Asia/Yerevan" "ইয়েরাভান"
    "Antarctica/Casey" "কেইসি"
    "Antarctica/Davis" "ডেভিস"
    "Antarctica/Mawson" "মসোন"
    "Antarctica/Palmer" "পালমার"
    "Antarctica/Rothera" "রথেরা"
    "Antarctica/Troll" "ট্রল"
    "Antarctica/Vostok" "ভস্টোক"
    "America/Argentina/Buenos_Aires" "বুয়েনোস আয়েরেস"
    "America/Argentina/Cordoba" "কর্ডোবা"
    "America/Argentina/Salta" "স্যালটা"
    "America/Argentina/Jujuy" "জুজুই"
    "America/Argentina/Tucuman" "টুকুমান"
    "America/Argentina/Catamarca" "ক্যাটামার্কা"
    "America/Argentina/La_Rioja" "লা রিওহা"
    "America/Argentina/San_Juan" "সান জুয়ান"
    "America/Argentina/Mendoza" "মেন্ডোজা"
    "America/Argentina/San_Luis" "সান লুইস"
    "America/Argentina/Rio_Gallegos" "রিও গায়েগোস"
    "America/Argentina/Ushuaia" "উশুয়াইয়া"
    "Pacific/Pago_Pago" "প্যাগো প্যাগো"
    "Europe/Vienna" "ভিয়েনা"
    "Australia/Lord_Howe" "লর্ড হাও"
    "Antarctica/Macquarie" "ম্যাককুয়্যারি"
    "Australia/Hobart" "হোবার্ট"
    "Australia/Melbourne" "মেলবোর্ন"
    "Australia/Sydney" "সিডনি"
    "Australia/Broken_Hill" "ব্রোকেন হিল"
    "Australia/Brisbane" "ব্রিসবেন"
    "Australia/Lindeman" "লিনডেম্যান"
    "Australia/Adelaide" "এ্যাডেলেইড"
    "Australia/Darwin" "ডারউইন"
    "Australia/Perth" "পার্থ"
    "Australia/Eucla" "ইউক্লা"
    "Asia/Baku" "বাকু"
    "America/Barbados" "বার্বাডোজ"
    "Asia/Dhaka" "ঢাকা"
    "Europe/Brussels" "ব্রাসেলস"
    "Europe/Sofia" "সোফিয়া"
    "Atlantic/Bermuda" "বারমুডা"
    "America/La_Paz" "লা পাজ"
    "America/Noronha" "নরোন্‌হা"
    "America/Belem" "বেলেম"
    "America/Fortaleza" "ফোর্টালেজা"
    "America/Recife" "রেসিফে"
    "America/Araguaina" "আরাগুয়াইনা"
    "America/Maceio" "মাসেয়ো"
    "America/Bahia" "বাহিয়া"
    "America/Sao_Paulo" "সাও পাউলো"
    "America/Campo_Grande" "কাম্পো গ্রান্ডে"
    "America/Cuiaba" "কুইয়াবা"
    "America/Santarem" "সেনটুরেম"
    "America/Porto_Velho" "পোর্তো ভেল্‌হো"
    "America/Boa_Vista" "বোয়া ভিস্তা"
    "America/Manaus" "মানাউস"
    "America/Eirunepe" "আইরুনেপে"
    "America/Rio_Branco" "রিও ব্রাঙ্কো"
    "Asia/Thimphu" "থিম্ফু"
    "Europe/Minsk" "মিন্সক"
    "America/Belize" "বেলিজ"
    "America/St_Johns" "সেন্ট জন্স"
    "America/Halifax" "হ্যালিফ্যাক্স"
    "America/Glace_Bay" "গ্লাস বে"
    "America/Moncton" "মঙ্কটোন"
    "America/Goose_Bay" "গুস বে"
    "America/Toronto" "টোরন্টো"
    "America/Iqaluit" "ইকুয়ালুইট"
    "America/Winnipeg" "উইনিপেগ"
    "America/Resolute" "রেসোলুট"
    "America/Rankin_Inlet" "র‍্যাঙ্কিন ইনলেট"
    "America/Regina" "রেজিনা"
    "America/Swift_Current" "সুইফ্ট কারেন্ট"
    "America/Edmonton" "এডমন্টোন"
    "America/Cambridge_Bay" "কেমব্রিজ বে"
    "America/Inuvik" "ইনুভ্যাক"
    "America/Vancouver" "ভ্যাঙ্কুভার"
    "America/Dawson_Creek" "ডসোন ক্রিক"
    "America/Fort_Nelson" "ফোর্ট নেলসন"
    "America/Whitehorse" "হোয়াইটহর্স"
    "America/Dawson" "ডসোন"
    "Europe/Zurich" "জুরিখ"
    "Africa/Abidjan" "আবিদজান"
    "Pacific/Rarotonga" "রারউহতুঙ্গা"
    "America/Santiago" "সান্টিয়াগো"
    "America/Coyhaique" "কোয়আইকে"
    "America/Punta_Arenas" "পুন্টা আরেনাস"
    "Pacific/Easter" "ইস্টার"
    "Asia/Shanghai" "সাংহাই"
    "Asia/Urumqi" "উরুমকি"
    "America/Bogota" "বোগোটা"
    "America/Costa_Rica" "কোস্টারিকা"
    "America/Havana" "হাভানা"
    "Atlantic/Cape_Verde" "কেপ ভার্দ"
    "Asia/Nicosia" "নিকোসিয়া"
    "Asia/Famagusta" "ফামাগাস্তা"
    "Europe/Prague" "প্রাগ"
    "Europe/Berlin" "বার্লিন"
    "America/Santo_Domingo" "স্যান্টো ডোমিংগো"
    "Africa/Algiers" "আলজিয়ার্স"
    "America/Guayaquil" "গোয়াইয়াকিল"
    "Pacific/Galapagos" "গ্যালাপ্যাগোস"
    "Europe/Tallinn" "তাহলিন"
    "Africa/Cairo" "কায়রো"
    "Africa/El_Aaiun" "এল আহইউন"
    "Europe/Madrid" "মাদ্রিদ"
    "Africa/Ceuta" "সেউটা"
    "Atlantic/Canary" "কানেরি"
    "Europe/Helsinki" "হেলসিঙ্কি"
    "Pacific/Fiji" "ফিজি"
    "Atlantic/Stanley" "স্টানলী"
    "Pacific/Kosrae" "কোসরায়"
    "Atlantic/Faroe" "ফ্যারো"
    "Europe/Paris" "প্যারিস"
    "Europe/London" "লন্ডন"
    "Asia/Tbilisi" "সিবিলিশি"
    "America/Cayenne" "কাহেন"
    "Europe/Gibraltar" "জিব্রাল্টার"
    "America/Nuuk" "নুক"
    "America/Danmarkshavn" "ডানমার্কশ্যাভন"
    "America/Scoresbysund" "ইট্টকুয়োরটুরমিট"
    "America/Thule" "থুলি"
    "Europe/Athens" "এথেন্স"
    "Atlantic/South_Georgia" "দক্ষিণ জর্জিয়া"
    "America/Guatemala" "গুয়াতেমালা"
    "Pacific/Guam" "গুয়াম"
    "Africa/Bissau" "বিসোউ"
    "America/Guyana" "গায়ানা"
    "Asia/Hong_Kong" "হং কং"
    "America/Tegucigalpa" "তেগুসিগালপা"
    "America/Port-au-Prince" "পোর্ট-অহ-প্রিন্স"
    "Europe/Budapest" "বুডাপেস্ট"
    "Asia/Jakarta" "জাকার্তা"
    "Asia/Pontianak" "পন্টিয়ান্যাক"
    "Asia/Makassar" "মাকাসসার"
    "Asia/Jayapura" "জয়াপুরা"
    "Europe/Dublin" "ডাবলিন"
    "Asia/Jerusalem" "জেরুজালেম"
    "Asia/Kolkata" "কোলকাতা"
    "Indian/Chagos" "ছাগোস"
    "Asia/Baghdad" "বাগদাদ"
    "Asia/Tehran" "তেহেরান"
    "Europe/Rome" "রোম"
    "America/Jamaica" "জামাইকা"
    "Asia/Amman" "আম্মান"
    "Asia/Tokyo" "টোকিও"
    "Africa/Nairobi" "নাইরোবি"
    "Asia/Bishkek" "বিশকেক"
    "Pacific/Tarawa" "টারাওয়া"
    "Pacific/Kanton" "ক্যান্টন"
    "Pacific/Kiritimati" "কিরিতিমাতি"
    "Asia/Pyongyang" "পিয়ংইয়ং"
    "Asia/Seoul" "সিওল"
    "Asia/Almaty" "আলমাটি"
    "Asia/Qyzylorda" "কিজিলর্ডা"
    "Asia/Qostanay" "কোস্টানয়"
    "Asia/Aqtobe" "আকটোবে"
    "Asia/Aqtau" "আকটাউ"
    "Asia/Atyrau" "অতিরাউ"
    "Asia/Oral" "ওরাল"
    "Asia/Beirut" "বেইরুট"
    "Asia/Colombo" "কলম্বো"
    "Africa/Monrovia" "মনরোভিয়া"
    "Europe/Vilnius" "ভিলনিওস"
    "Europe/Riga" "রিগা"
    "Africa/Tripoli" "ত্রিপোলি"
    "Africa/Casablanca" "কাসাব্লাঙ্কা"
    "Europe/Chisinau" "কিসিনাহু"
    "Pacific/Kwajalein" "কোয়াজালেইন"
    "Asia/Yangon" "রেঙ্গুন"
    "Asia/Ulaanbaatar" "উলানবাতার"
    "Asia/Hovd" "হোভ্ড"
    "Asia/Macau" "ম্যাকাও"
    "America/Martinique" "মারটিনিক"
    "Europe/Malta" "মাল্টা"
    "Indian/Mauritius" "মরিশাস"
    "Indian/Maldives" "মালদ্বীপ"
    "America/Mexico_City" "মেক্সিকো সিটি"
    "America/Cancun" "ক্যানকুন"
    "America/Merida" "মেরিডা"
    "America/Monterrey" "মন্টেরি"
    "America/Matamoros" "মাতামোরস"
    "America/Chihuahua" "চিহুয়াহুয়া"
    "America/Ciudad_Juarez" "সিউদাদ জুয়ারেজ"
    "America/Ojinaga" "ওজিনাগা"
    "America/Mazatlan" "মাজাটলান"
    "America/Bahia_Banderas" "বাহিয়া বান্দেরাস"
    "America/Hermosillo" "হারমোসিল্লো"
    "America/Tijuana" "তিজুয়ানা"
    "Asia/Kuching" "কুচিং"
    "Africa/Maputo" "মাপুতো"
    "Africa/Windhoek" "উইনধোক"
    "Pacific/Noumea" "নুমিয়া"
    "Pacific/Norfolk" "নরফক"
    "Africa/Lagos" "লাগোস"
    "America/Managua" "মানাগুয়া"
    "Asia/Kathmandu" "কাঠমান্ডু"
    "Pacific/Nauru" "নাউরু"
    "Pacific/Niue" "নিউয়ি"
    "Pacific/Auckland" "অকল্যান্ড"
    "Pacific/Chatham" "চ্যাঠাম"
    "America/Panama" "পানামা"
    "America/Lima" "লিমা"
    "Pacific/Tahiti" "তাহিতি"
    "Pacific/Marquesas" "মার্কেসাস"
    "Pacific/Gambier" "গাম্বিয়ের"
    "Pacific/Port_Moresby" "পোর্ট মৌরজবি"
    "Pacific/Bougainville" "বুগেনভিলে"
    "Asia/Manila" "ম্যানিলা"
    "Asia/Karachi" "করাচি"
    "Europe/Warsaw" "ওয়ারশ"
    "America/Miquelon" "মিকুলন"
    "Pacific/Pitcairn" "পিটকেয়ার্ন"
    "America/Puerto_Rico" "পুয়ের্তো রিকো"
    "Asia/Gaza" "গাজা"
    "Asia/Hebron" "হেব্রোন"
    "Europe/Lisbon" "লিসবন"
    "Atlantic/Madeira" "মাডেইরা"
    "Atlantic/Azores" "আজোরেস"
    "Pacific/Palau" "পালাউ"
    "America/Asuncion" "আসুনসিয়ন"
    "Asia/Qatar" "কাতার"
    "Europe/Bucharest" "বুখারেস্ট"
    "Europe/Belgrade" "বেলগ্রেড"
    "Europe/Kaliningrad" "কালিনিঙগ্রাড"
    "Europe/Moscow" "মস্কো"
    "Europe/Simferopol" "সিমফেরোপোল"
    "Europe/Kirov" "কিরোভ"
    "Europe/Volgograd" "ভোল্গোগ্রাদ"
    "Europe/Astrakhan" "আসট্রাখান"
    "Europe/Saratov" "সারাটোভ"
    "Europe/Ulyanovsk" "উলিয়ানোভস্ক"
    "Europe/Samara" "সামারা"
    "Asia/Yekaterinburg" "ইয়েকাটেরিনবার্গ"
    "Asia/Omsk" "ওম্স্ক"
    "Asia/Novosibirsk" "নভোসিবির্স্ক"
    "Asia/Barnaul" "বার্নৌল"
    "Asia/Tomsk" "তোমস্ক"
    "Asia/Novokuznetsk" "নভকুয়েতস্নক"
    "Asia/Krasnoyarsk" "ক্রাসনোইয়ার্স্ক"
    "Asia/Irkutsk" "ইরকুটস্ক"
    "Asia/Chita" "চিতা"
    "Asia/Yakutsk" "ইয়াকুটস্ক"
    "Asia/Khandyga" "খানডিয়াগা"
    "Asia/Vladivostok" "ভ্লাদিভস্তোক"
    "Asia/Ust-Nera" "উস্ত- নেরা"
    "Asia/Magadan" "ম্যাগাডান"
    "Asia/Sakhalin" "সাখালিন"
    "Asia/Srednekolymsk" "স্রেদনেকোলয়মস্ক"
    "Asia/Kamchatka" "কামচাটকা"
    "Asia/Anadyr" "অ্যানাডির"
    "Asia/Riyadh" "রিয়াধ"
    "Pacific/Guadalcanal" "গোয়াদালকুনাল"
    "Africa/Khartoum" "খার্তুম"
    "Asia/Singapore" "সিঙ্গাপুর"
    "America/Paramaribo" "প্যারামেরিবো"
    "Africa/Juba" "জুবা"
    "Africa/Sao_Tome" "সাও টোম"
    "America/El_Salvador" "এল সালভাদোর"
    "Asia/Damascus" "দামাস্কাস"
    "America/Grand_Turk" "গ্র্যান্ড তুর্ক"
    "Africa/Ndjamena" "এনজমেনা"
    "Asia/Bangkok" "ব্যাংকক"
    "Asia/Dushanbe" "দুশানবে"
    "Pacific/Fakaofo" "ফ্যাকাওফো"
    "Asia/Dili" "দিলি"
    "Asia/Ashgabat" "আশগাবাত"
    "Africa/Tunis" "টিউনিস"
    "Pacific/Tongatapu" "টোঙ্গাটাপু"
    "Europe/Istanbul" "ইস্তানবুল"
    "Asia/Taipei" "তাইপেই"
    "Europe/Kyiv" "কিয়েভ"
    "America/New_York" "নিউইয়র্ক"
    "America/Detroit" "ডেট্রোইট"
    "America/Kentucky/Louisville" "লুইসভিল"
    "America/Kentucky/Monticello" "মন্টিচেলো, কেন্টাকি"
    "America/Indiana/Indianapolis" "ইন্ডিয়ানাপোলিস"
    "America/Indiana/Vincennes" "ভিনসেন্নেস, ইন্ডিয়ানা"
    "America/Indiana/Winamac" "উইনাম্যাক, ইন্ডিয়ানা"
    "America/Indiana/Marengo" "মারেঙ্গো, ইন্ডিয়ানা"
    "America/Indiana/Petersburg" "পিটারর্সবার্গ, ইন্ডিয়ানা"
    "America/Indiana/Vevay" "ভেভেয়, ইন্ডিয়ানা"
    "America/Chicago" "শিকাগো"
    "America/Indiana/Tell_City" "টেলসিটি, ইন্ডিয়ানা"
    "America/Indiana/Knox" "নক্স, ইন্ডিয়ানা"
    "America/Menominee" "মেনোমিনি"
    "America/North_Dakota/Center" "মধ্য, উত্তর ডাকোটা"
    "America/North_Dakota/New_Salem" "নিউ সালেম, উত্তর ডাকোটা"
    "America/North_Dakota/Beulah" "বেউলা, উত্তর ডাকোটা"
    "America/Denver" "ডেনভার"
    "America/Boise" "বয়জি"
    "America/Phoenix" "ফিনিক্স"
    "America/Los_Angeles" "লস অ্যাঞ্জেলেস"
    "America/Anchorage" "এনকোরেজ"
    "America/Juneau" "জুনো"
    "America/Sitka" "শিটকা"
    "America/Metlakatla" "মেটলাকাটলা"
    "America/Yakutat" "ইয়াকুটাট"
    "America/Nome" "নোম"
    "America/Adak" "আডক"
    "Pacific/Honolulu" "হনোলুলু"
    "America/Montevideo" "মন্টেভিডিও"
    "Asia/Samarkand" "সমরখন্দ"
    "Asia/Tashkent" "তাসখন্দ"
    "America/Caracas" "ক্যারাকাস"
    "Asia/Ho_Chi_Minh" "হো চি মিন শহর"
    "Pacific/Efate" "ইফাতে"
    "Pacific/Apia" "আপিয়া"
    "Africa/Johannesburg" "জোহানেসবার্গ"
    "America/Antigua" "অ্যান্টিগুয়া"
    "America/Anguilla" "অ্যাঙ্গুইলা"
    "Africa/Luanda" "লোয়ান্ডা"
    "Antarctica/McMurdo" "ম্যাকমুর্ডো"
    "Antarctica/DumontDUrville" "ডুমন্ট ডি’উরভিল"
    "Antarctica/Syowa" "সিওয়া"
    "America/Aruba" "এরুবা"
    "Europe/Mariehamn" "মরিয়েহামেন"
    "Europe/Sarajevo" "সারাজিভো"
    "Africa/Ouagadougou" "ওয়াহগুডোগু"
    "Asia/Bahrain" "বাহারিন"
    "Africa/Bujumbura" "বুজুমবুরহু"
    "Africa/Porto-Novo" "পোর্টো-নোভো"
    "America/St_Barthelemy" "সেন্ট.বার্থেলেমি"
    "Asia/Brunei" "ব্রুনেই"
    "America/Kralendijk" "ক্রেলেন্ডাজিক"
    "America/Nassau" "নাসাউ"
    "Africa/Gaborone" "গ্যাবুরনি"
    "America/Blanc-Sablon" "ব্লাঙ্ক-সাব্লোন"
    "America/Atikokan" "আটিকোকান"
    "America/Creston" "ক্রিস্টান"
    "Indian/Cocos" "কোকোস"
    "Africa/Kinshasa" "কিনশাসা"
    "Africa/Lubumbashi" "লুবুম্বাশি"
    "Africa/Bangui" "বাঙ্গুই"
    "Africa/Brazzaville" "ব্রাজাভিলি"
    "Africa/Douala" "ডোয়ালা"
    "America/Curacao" "কুরাসাও"
    "Indian/Christmas" "ক্রিসমাস"
    "Europe/Busingen" "বুসিনগেন"
    "Africa/Djibouti" "জিবুটি"
    "Europe/Copenhagen" "কোপেনহেগেন"
    "America/Dominica" "ডোমিনিকা"
    "Africa/Asmara" "অ্যাসমারাহু"
    "Africa/Addis_Ababa" "আদ্দিস আবাবা"
    "Pacific/Chuuk" "চুক"
    "Pacific/Pohnpei" "পোনাপে"
    "Africa/Libreville" "লিব্রুভিল"
    "America/Grenada" "গ্রেনাডা"
    "Europe/Guernsey" "গুয়ার্নসি"
    "Africa/Accra" "আক্রা"
    "Africa/Banjul" "বাঞ্জুল"
    "Africa/Conakry" "কনাক্রি"
    "America/Guadeloupe" "গুয়াদেলোপ"
    "Africa/Malabo" "মালাবো"
    "Europe/Zagreb" "জাগ্রেব"
    "Europe/Isle_of_Man" "আইল অফ ম্যান"
    "Atlantic/Reykjavik" "রিকজাভিক"
    "Europe/Jersey" "জার্সি"
    "Asia/Phnom_Penh" "নম পেন"
    "Indian/Comoro" "কোমোরো"
    "America/St_Kitts" "সেন্ট. কিটস"
    "Asia/Kuwait" "কুয়েত"
    "America/Cayman" "কামেন"
    "Asia/Vientiane" "ভিয়েনতায়েন"
    "America/St_Lucia" "সেন্ট. লুসিয়া"
    "Europe/Vaduz" "ভাদুজ"
    "Africa/Maseru" "মাহসুরু"
    "Europe/Luxembourg" "লুক্সেমবার্গ"
    "Europe/Monaco" "মোনাকো"
    "Europe/Podgorica" "পডগরিত্সা"
    "America/Marigot" "মারিগো"
    "Indian/Antananarivo" "আন্তুনানারিভো"
    "Pacific/Majuro" "মাজুরো"
    "Europe/Skopje" "স্কপয়ে"
    "Africa/Bamako" "বাম্যাকো"
    "Pacific/Saipan" "সাইপান"
    "Africa/Nouakchott" "নোয়াকশট"
    "America/Montserrat" "মন্তসেরাত"
    "Africa/Blantyre" "ব্ল্যানটায়ের"
    "Asia/Kuala_Lumpur" "কুয়ালালামপুর"
    "Africa/Niamey" "নিয়ামে"
    "Europe/Amsterdam" "আমস্টারডাম"
    "Europe/Oslo" "অসলো"
    "Asia/Muscat" "মাসকট"
    "Indian/Reunion" "রিইউনিয়ন"
    "Africa/Kigali" "কিগালি"
    "Indian/Mahe" "মাহে"
    "Europe/Stockholm" "স্টকহোম"
    "Atlantic/St_Helena" "সেন্ট. হেলেনা"
    "Europe/Ljubljana" "লুবলিয়ানা"
    "Arctic/Longyearbyen" "লঞ্জিয়বিয়েঁন"
    "Europe/Bratislava" "ব্রাতিস্লাভা"
    "Africa/Freetown" "ফ্রীটাউন"
    "Europe/San_Marino" "সান মেরিনো"
    "Africa/Dakar" "ডাকার"
    "Africa/Mogadishu" "মাওগাদিসু"
    "America/Lower_Princes" "লোয়ার প্রিন্সেস কোয়ার্টার"
    "Africa/Mbabane" "অমবাবান"
    "Indian/Kerguelen" "কার্গুলেন"
    "Africa/Lome" "লোমে"
    "America/Port_of_Spain" "পোর্ট অফ স্পেন"
    "Pacific/Funafuti" "ফুনাফুটি"
    "Africa/Dar_es_Salaam" "দার এস সালাম"
    "Africa/Kampala" "কামপালা"
    "Pacific/Midway" "মিডওয়ে"
    "Pacific/Wake" "ওয়েক"
    "Europe/Vatican" "ভাটিকান"
    "America/St_Vincent" "সেন্ট. ভিনসেন্ট"
    "America/Tortola" "টরটোলা"
    "America/St_Thomas" "সেন্ট. থমাস"
    "Pacific/Wallis" "ওলিস"
    "Asia/Aden" "আহদেন"
    "Indian/Mayotte" "মায়োতো"
    "Africa/Lusaka" "লুসাকা"
    "Africa/Harare" "হারারে"
};

// `common/main/cs.xml`: 135 of the 418 zones named, 283 inherited.
#[cfg(feature = "localized-exemplar-cities")]
const CS: &str = exemplar_cities! {
    "Europe/Andorra" inherited
    "Asia/Dubai" "Dubaj"
    "Asia/Kabul" "Kábul"
    "Europe/Tirane" inherited
    "Asia/Yerevan" "Jerevan"
    "Antarctica/Casey" inherited
    "Antarctica/Davis" inherited
    "Antarctica/Mawson" inherited
    "Antarctica/Palmer" inherited
    "Antarctica/Rothera" inherited
    "Antarctica/Troll" inherited
    "Antarctica/Vostok" inherited
    "America/Argentina/Buenos_Aires" inherited
    "America/Argentina/Cordoba" inherited
    "America/Argentina/Salta" inherited
    "America/Argentina/Jujuy" inherited
    "America/Argentina/Tucuman" inherited
    "America/Argentina/Catamarca" inherited
    "America/Argentina/La_Rioja" inherited
    "America/Argentina/San_Juan" inherited
    "America/Argentina/Mendoza" inherited
    "America/Argentina/San_Luis" inherited
    "America/Argentina/Rio_Gallegos" inherited
    "America/Argentina/Ushuaia" inherited
    "Pacific/Pago_Pago" inherited
    "Europe/Vienna" "Vídeň"
    "Australia/Lord_Howe" inherited
    "Antarctica/Macquarie" inherited
    "Australia/Hobart" inherited
    "Australia/Melbourne" inherited
    "Australia/Sydney" inherited
    "Australia/Broken_Hill" inherited
    "Australia/Brisbane" inherited
    "Australia/Lindeman" inherited
    "Australia/Adelaide" inherited
    "Australia/Darwin" inherited
    "Australia/Perth" inherited
    "Australia/Eucla" inherited
    "Asia/Baku" inherited
    "America/Barbados" inherited
    "Asia/Dhaka" "Dháka"
    "Europe/Brussels" "Brusel"
    "Europe/Sofia" "Sofie"
    "Atlantic/Bermuda" "Bermudy"
    "America/La_Paz" inherited
    "America/Noronha" inherited
    "America/Belem" inherited
    "America/Fortaleza" inherited
    "America/Recife" inherited
    "America/Araguaina" inherited
    "America/Maceio" inherited
    "America/Bahia" "Bahía"
    "America/Sao_Paulo" inherited
    "America/Campo_Grande" inherited
    "America/Cuiaba" inherited
    "America/Santarem" inherited
    "America/Porto_Velho" inherited
    "America/Boa_Vista" inherited
    "America/Manaus" inherited
    "America/Eirunepe" inherited
    "America/Rio_Branco" inherited
    "Asia/Thimphu" "Thimbú"
    "Europe/Minsk" inherited
    "America/Belize" inherited
    "America/St_Johns" inherited
    "America/Halifax" inherited
    "America/Glace_Bay" inherited
    "America/Moncton" inherited
    "America/Goose_Bay" inherited
    "America/Toronto" inherited
    "America/Iqaluit" inherited
    "America/Winnipeg" inherited
    "America/Resolute" inherited
    "America/Rankin_Inlet" inherited
    "America/Regina" inherited
    "America/Swift_Current" inherited
    "America/Edmonton" inherited
    "America/Cambridge_Bay" inherited
    "America/Inuvik" inherited
    "America/Vancouver" inherited
    "America/Dawson_Creek" inherited
    "America/Fort_Nelson" inherited
    "America/Whitehorse" inherited
    "America/Dawson" inherited
    "Europe/Zurich" "Curych"
    "Africa/Abidjan" "Abidžan"
    "Pacific/Rarotonga" inherited
    "America/Santiago" inherited
    "America/Coyhaique" inherited
    "America/Punta_Arenas" inherited
    "Pacific/Easter" "Velikonoční ostrov"
    "Asia/Shanghai" "Šanghaj"
    "Asia/Urumqi" "Urumči"
    "America/Bogota" inherited
    "America/Costa_Rica" "Kostarika"
    "America/Havana" inherited
    "Atlantic/Cape_Verde" "Kapverdy"
    "Asia/Nicosia" "Nikósie"
    "Asia/Famagusta" inherited
    "Europe/Prague" "Praha"
    "Europe/Berlin" "Berlín"
    "America/Santo_Domingo" inherited
    "Africa/Algiers" "Alžír"
    "America/Guayaquil" inherited
    "Pacific/Galapagos" "Galapágy"
    "Europe/Tallinn" inherited
    "Africa/Cairo" "Káhira"
    "Africa/El_Aaiun" inherited
    "Europe/Madrid" inherited
    "Africa/Ceuta" inherited
    "Atlantic/Canary" "Kanárské ostrovy"
    "Europe/Helsinki" "Helsinky"
    "Pacific/Fiji" "Fidži"
    "Atlantic/Stanley" inherited
    "Pacific/Kosrae" inherited
    "Atlantic/Faroe" "Faerské ostrovy"
    "Europe/Paris" "Paříž"
    "Europe/London" "Londýn"
    "Asia/Tbilisi" inherited
    "America/Cayenne" inherited
    "Europe/Gibraltar" inherited
    "America/Nuuk" inherited
    "America/Danmarkshavn" inherited
    "America/Scoresbysund" inherited
    "America/Thule" inherited
    "Europe/Athens" "Athény"
    "Atlantic/South_Georgia" "Jižní Georgie"
    "America/Guatemala" inherited
    "Pacific/Guam" inherited
    "Africa/Bissau" inherited
    "America/Guyana" inherited
    "Asia/Hong_Kong" "Hongkong"
    "America/Tegucigalpa" inherited
    "America/Port-au-Prince" inherited
    "Europe/Budapest" "Budapešť"
    "Asia/Jakarta" inherited
    "Asia/Pontianak" inherited
    "Asia/Makassar" inherited
    "Asia/Jayapura" inherited
    "Europe/Dublin" inherited
    "Asia/Jerusalem" "Jeruzalém"
    "Asia/Kolkata" "Kalkata"
    "Indian/Chagos" inherited
    "Asia/Baghdad" "Bagdád"
    "Asia/Tehran" "Teherán"
    "Europe/Rome" "Řím"
    "America/Jamaica" "Jamajka"
    "Asia/Amman" "Ammán"
    "Asia/Tokyo" "Tokio"
    "Africa/Nairobi" inherited
    "Asia/Bishkek" "Biškek"
    "Pacific/Tarawa" inherited
    "Pacific/Kanton" "Kanton (ostrov)"
    "Pacific/Kiritimati" inherited
    "Asia/Pyongyang" "Pchjongjang"
    "Asia/Seoul" "Soul"
    "Asia/Almaty" inherited
    "Asia/Qyzylorda" "Kyzylorda"
    "Asia/Qostanay" "Kostanaj"
    "Asia/Aqtobe" "Aktobe"
    "Asia/Aqtau" "Aktau"
    "Asia/Atyrau" inherited
    "Asia/Oral" "Uralsk"
    "Asia/Beirut" "Bejrút"
    "Asia/Colombo" "Kolombo"
    "Africa/Monrovia" inherited
    "Europe/Vilnius" inherited
    "Europe/Riga" inherited
    "Africa/Tripoli" "Tripolis"
    "Africa/Casablanca" inherited
    "Europe/Chisinau" "Kišiněv"
    "Pacific/Kwajalein" inherited
    "Asia/Yangon" "Rangún"
    "Asia/Ulaanbaatar" "Ulánbátar"
    "Asia/Hovd" inherited
    "Asia/Macau" inherited
    "America/Martinique" "Martinik"
    "Europe/Malta" inherited
    "Indian/Mauritius" "Mauricius"
    "Indian/Maldives" "Maledivy"
    "America/Mexico_City" inherited
    "America/Cancun" inherited
    "America/Merida" "Merida"
    "America/Monterrey" inherited
    "America/Matamoros" inherited
    "America/Chihuahua" inherited
    "America/Ciudad_Juarez" inherited
    "America/Ojinaga" inherited
    "America/Mazatlan" inherited
    "America/Bahia_Banderas" "Bahia Banderas"
    "America/Hermosillo" inherited
    "America/Tijuana" inherited
    "Asia/Kuching" "Kučing"
    "Africa/Maputo" inherited
    "Africa/Windhoek" inherited
    "Pacific/Noumea" inherited
    "Pacific/Norfolk" inherited
    "Africa/Lagos" inherited
    "America/Managua" inherited
    "Asia/Kathmandu" "Káthmándú"
    "Pacific/Nauru" inherited
    "Pacific/Niue" inherited
    "Pacific/Auckland" inherited
    "Pacific/Chatham" "Chathamské ostrovy"
    "America/Panama" inherited
    "America/Lima" inherited
    "Pacific/Tahiti" inherited
    "Pacific/Marquesas" "Markézy"
    "Pacific/Gambier" "Gambierovy ostrovy"
    "Pacific/Port_Moresby" inherited
    "Pacific/Bougainville" inherited
    "Asia/Manila" inherited
    "Asia/Karachi" "Karáčí"
    "Europe/Warsaw" "Varšava"
    "America/Miquelon" inherited
    "Pacific/Pitcairn" "Pitcairnovy ostrovy"
    "America/Puerto_Rico" "Portoriko"
    "Asia/Gaza" inherited
    "Asia/Hebron" inherited
    "Europe/Lisbon" "Lisabon"
    "Atlantic/Madeira" inherited
    "Atlantic/Azores" "Azorské ostrovy"
    "Pacific/Palau" inherited
    "America/Asuncion" inherited
    "Asia/Qatar" "Katar"
    "Europe/Bucharest" "Bukurešť"
    "Europe/Belgrade" "Bělehrad"
    "Europe/Kaliningrad" inherited
    "Europe/Moscow" "Moskva"
    "Europe/Simferopol" inherited
    "Europe/Kirov" inherited
    "Europe/Volgograd" inherited
    "Europe/Astrakhan" "Astrachaň"
    "Europe/Saratov" inherited
    "Europe/Ulyanovsk" "Uljanovsk"
    "Europe/Samara" inherited
    "Asia/Yekaterinburg" "Jekatěrinburg"
    "Asia/Omsk" inherited
    "Asia/Novosibirsk" inherited
    "Asia/Barnaul" inherited
    "Asia/Tomsk" inherited
    "Asia/Novokuznetsk" "Novokuzněck"
    "Asia/Krasnoyarsk" "Krasnojarsk"
    "Asia/Irkutsk" inherited
    "Asia/Chita" "Čita"
    "Asia/Yakutsk" "Jakutsk"
    "Asia/Khandyga" "Chandyga"
    "Asia/Vladivostok" inherited
    "Asia/Ust-Nera" inherited
    "Asia/Magadan" inherited
    "Asia/Sakhalin" "Sachalin"
    "Asia/Srednekolymsk" "Sredněkolymsk"
    "Asia/Kamchatka" "Kamčatka"
    "Asia/Anadyr" inherited
    "Asia/Riyadh" "Rijád"
    "Pacific/Guadalcanal" inherited
    "Africa/Khartoum" "Chartúm"
    "Asia/Singapore" "Singapur"
    "America/Paramaribo" inherited
    "Africa/Juba" inherited
    "Africa/Sao_Tome" "Svatý Tomáš"
    "America/El_Salvador" "Salvador"
    "Asia/Damascus" "Damašek"
    "America/Grand_Turk" inherited
    "Africa/Ndjamena" "Ndžamena"
    "Asia/Bangkok" inherited
    "Asia/Dushanbe" "Dušanbe"
    "Pacific/Fakaofo" inherited
    "Asia/Dili" inherited
    "Asia/Ashgabat" "Ašchabad"
    "Africa/Tunis" inherited
    "Pacific/Tongatapu" inherited
    "Europe/Istanbul" inherited
    "Asia/Taipei" "Tchaj-pej"
    "Europe/Kyiv" "Kyjev"
    "America/New_York" inherited
    "America/Detroit" inherited
    "America/Kentucky/Louisville" inherited
    "America/Kentucky/Monticello" inherited
    "America/Indiana/Indianapolis" inherited
    "America/Indiana/Vincennes" inherited
    "America/Indiana/Winamac" inherited
    "America/Indiana/Marengo" inherited
    "America/Indiana/Petersburg" inherited
    "America/Indiana/Vevay" inherited
    "America/Chicago" inherited
    "America/Indiana/Tell_City" inherited
    "America/Indiana/Knox" inherited
    "America/Menominee" inherited
    "America/North_Dakota/Center" "Center, Severní Dakota"
    "America/North_Dakota/New_Salem" "New Salem, Severní Dakota"
    "America/North_Dakota/Beulah" "Beulah, Severní Dakota"
    "America/Denver" inherited
    "America/Boise" inherited
    "America/Phoenix" inherited
    "America/Los_Angeles" inherited
    "America/Anchorage" inherited
    "America/Juneau" inherited
    "America/Sitka" inherited
    "America/Metlakatla" inherited
    "America/Yakutat" inherited
    "America/Nome" inherited
    "America/Adak" inherited
    "Pacific/Honolulu" "Honolulu"
    "America/Montevideo" inherited
    "Asia/Samarkand" inherited
    "Asia/Tashkent" "Taškent"
    "America/Caracas" inherited
    "Asia/Ho_Chi_Minh" "Ho Či Minovo město"
    "Pacific/Efate" "Éfaté"
    "Pacific/Apia" inherited
    "Africa/Johannesburg" inherited
    "America/Antigua" inherited
    "America/Anguilla" inherited
    "Africa/Luanda" inherited
    "Antarctica/McMurdo" inherited
    "Antarctica/DumontDUrville" inherited
    "Antarctica/Syowa" inherited
    "America/Aruba" inherited
    "Europe/Mariehamn" inherited
    "Europe/Sarajevo" inherited
    "Africa/Ouagadougou" inherited
    "Asia/Bahrain" "Bahrajn"
    "Africa/Bujumbura" inherited
    "Africa/Porto-Novo" inherited
    "America/St_Barthelemy" "Svatý Bartoloměj"
    "Asia/Brunei" "Brunej"
    "America/Kralendijk" inherited
    "America/Nassau" inherited
    "Africa/Gaborone" inherited
    "America/Blanc-Sablon" inherited
    "America/Atikokan" inherited
    "America/Creston" inherited
    "Indian/Cocos" "Kokosové ostrovy"
    "Africa/Kinshasa" inherited
    "Africa/Lubumbashi" inherited
    "Africa/Bangui" inherited
    "Africa/Brazzaville" inherited
    "Africa/Douala" inherited
    "America/Curacao" inherited
    "Indian/Christmas" "Vánoční ostrov"
    "Europe/Busingen" inherited
    "Africa/Djibouti" "Džibuti"
    "Europe/Copenhagen" "Kodaň"
    "America/Dominica" "Dominika"
    "Africa/Asmara" inherited
    "Africa/Addis_Ababa" "Addis Abeba"
    "Pacific/Chuuk" "Chuukské ostrovy"
    "Pacific/Pohnpei" inherited
    "Africa/Libreville" inherited
    "America/Grenada" inherited
    "Europe/Guernsey" inherited
    "Africa/Accra" inherited
    "Africa/Banjul" inherited
    "Africa/Conakry" inherited
    "America/Guadeloupe" inherited
    "Africa/Malabo" inherited
    "Europe/Zagreb" "Záhřeb"
    "Europe/Isle_of_Man" "Ostrov Man"
    "Atlantic/Reykjavik" "Reykjavík"
    "Europe/Jersey" inherited
    "Asia/Phnom_Penh" "Phnompenh"
    "Indian/Comoro" "Komory"
    "America/St_Kitts" "Svatý Kryštof"
    "Asia/Kuwait" "Kuvajt"
    "America/Cayman" "Kajmanské ostrovy"
    "Asia/Vientiane" inherited
    "America/St_Lucia" "Svatá Lucie"
    "Europe/Vaduz" inherited
    "Africa/Maseru" inherited
    "Europe/Luxembourg" "Lucemburk"
    "Europe/Monaco" "Monako"
    "Europe/Podgorica" inherited
    "America/Marigot" inherited
    "Indian/Antananarivo" inherited
    "Pacific/Majuro" inherited
    "Europe/Skopje" inherited
    "Africa/Bamako" inherited
    "Pacific/Saipan" inherited
    "Africa/Nouakchott" "Nuakšott"
    "America/Montserrat" inherited
    "Africa/Blantyre" inherited
    "Asia/Kuala_Lumpur" inherited
    "Africa/Niamey" inherited
    "Europe/Amsterdam" inherited
    "Europe/Oslo" inherited
    "Asia/Muscat" "Maskat"
    "Indian/Reunion" inherited
    "Africa/Kigali" inherited
    "Indian/Mahe" inherited
    "Europe/Stockholm" inherited
    "Atlantic/St_Helena" "Svatá Helena"
    "Europe/Ljubljana" "Lublaň"
    "Arctic/Longyearbyen" inherited
    "Europe/Bratislava" inherited
    "Africa/Freetown" inherited
    "Europe/San_Marino" inherited
    "Africa/Dakar" inherited
    "Africa/Mogadishu" "Mogadišu"
    "America/Lower_Princes" inherited
    "Africa/Mbabane" inherited
    "Indian/Kerguelen" "Kerguelenovy ostrovy"
    "Africa/Lome" inherited
    "America/Port_of_Spain" inherited
    "Pacific/Funafuti" inherited
    "Africa/Dar_es_Salaam" inherited
    "Africa/Kampala" inherited
    "Pacific/Midway" inherited
    "Pacific/Wake" inherited
    "Europe/Vatican" "Vatikán"
    "America/St_Vincent" "Svatý Vincenc"
    "America/Tortola" inherited
    "America/St_Thomas" "Svatý Tomáš (Karibik)"
    "Pacific/Wallis" inherited
    "Asia/Aden" inherited
    "Indian/Mayotte" inherited
    "Africa/Lusaka" inherited
    "Africa/Harare" inherited
};

// `common/main/de.xml`: 87 of the 418 zones named, 330 inherited.
#[cfg(feature = "localized-exemplar-cities")]
const DE: &str = exemplar_cities! {
    "Europe/Andorra" inherited
    "Asia/Dubai" inherited
    "Asia/Kabul" inherited
    "Europe/Tirane" inherited
    "Asia/Yerevan" "Eriwan"
    "Antarctica/Casey" inherited
    "Antarctica/Davis" inherited
    "Antarctica/Mawson" inherited
    "Antarctica/Palmer" inherited
    "Antarctica/Rothera" inherited
    "Antarctica/Troll" inherited
    "Antarctica/Vostok" "Wostok"
    "America/Argentina/Buenos_Aires" inherited
    "America/Argentina/Cordoba" inherited
    "America/Argentina/Salta" inherited
    "America/Argentina/Jujuy" inherited
    "America/Argentina/Tucuman" inherited
    "America/Argentina/Catamarca" inherited
    "America/Argentina/La_Rioja" inherited
    "America/Argentina/San_Juan" inherited
    "America/Argentina/Mendoza" inherited
    "America/Argentina/San_Luis" inherited
    "America/Argentina/Rio_Gallegos" inherited
    "America/Argentina/Ushuaia" inherited
    "Pacific/Pago_Pago" inherited
    "Europe/Vienna" "Wien"
    "Australia/Lord_Howe" inherited
    "Antarctica/Macquarie" inherited
    "Australia/Hobart" inherited
    "Australia/Melbourne" inherited
    "Australia/Sydney" inherited
    "Australia/Broken_Hill" inherited
    "Australia/Brisbane" inherited
    "Australia/Lindeman" inherited
    "Australia/Adelaide" inherited
    "Australia/Darwin" inherited
    "Australia/Perth" inherited
    "Australia/Eucla" inherited
    "Asia/Baku" inherited
    "America/Barbados" inherited
    "Asia/Dhaka" inherited
    "Europe/Brussels" "Brüssel"
    "Europe/Sofia" inherited
    "Atlantic/Bermuda" inherited
    "America/La_Paz" inherited
    "America/Noronha" inherited
    "America/Belem" inherited
    "America/Fortaleza" inherited
    "America/Recife" inherited
    "America/Araguaina" inherited
    "America/Maceio" inherited
    "America/Bahia" inherited
    "America/Sao_Paulo" inherited
    "America/Campo_Grande" inherited
    "America/Cuiaba" inherited
    "America/Santarem" inherited
    "America/Porto_Velho" inherited
    "America/Boa_Vista" inherited
    "America/Manaus" inherited
    "America/Eirunepe" inherited
    "America/Rio_Branco" inherited
    "Asia/Thimphu" inherited
    "Europe/Minsk" inherited
    "America/Belize" inherited
    "America/St_Johns" inherited
    "America/Halifax" inherited
    "America/Glace_Bay" inherited
    "America/Moncton" inherited
    "America/Goose_Bay" inherited
    "America/Toronto" inherited
    "America/Iqaluit" inherited
    "America/Winnipeg" inherited
    "America/Resolute" inherited
    "America/Rankin_Inlet" inherited
    "America/Regina" inherited
    "America/Swift_Current" inherited
    "America/Edmonton" inherited
    "America/Cambridge_Bay" inherited
    "America/Inuvik" inherited
    "America/Vancouver" inherited
    "America/Dawson_Creek" inherited
    "America/Fort_Nelson" inherited
    "America/Whitehorse" inherited
    "America/Dawson" inherited
    "Europe/Zurich" "Zürich"
    "Africa/Abidjan" inherited
    "Pacific/Rarotonga" inherited
    "America/Santiago" inherited
    "America/Coyhaique" ""
    "America/Punta_Arenas" inherited
    "Pacific/Easter" "Osterinsel"
    "Asia/Shanghai" inherited
    "Asia/Urumqi" inherited
    "America/Bogota" inherited
    "America/Costa_Rica" inherited
    "America/Havana" "Havanna"
    "Atlantic/Cape_Verde" "Cabo Verde"
    "Asia/Nicosia" "Nikosia"
    "Asia/Famagusta" inherited
    "Europe/Prague" "Prag"
    "Europe/Berlin" inherited
    "America/Santo_Domingo" inherited
    "Africa/Algiers" "Algier"
    "America/Guayaquil" inherited
    "Pacific/Galapagos" inherited
    "Europe/Tallinn" inherited
    "Africa/Cairo" "Kairo"
    "Africa/El_Aaiun" inherited
    "Europe/Madrid" inherited
    "Africa/Ceuta" inherited
    "Atlantic/Canary" "Kanaren"
    "Europe/Helsinki" inherited
    "Pacific/Fiji" "Fidschi"
    "Atlantic/Stanley" inherited
    "Pacific/Kosrae" inherited
    "Atlantic/Faroe" "Färöer"
    "Europe/Paris" inherited
    "Europe/London" inherited
    "Asia/Tbilisi" "Tiflis"
    "America/Cayenne" inherited
    "Europe/Gibraltar" inherited
    "America/Nuuk" inherited
    "America/Danmarkshavn" inherited
    "America/Scoresbysund" inherited
    "America/Thule" inherited
    "Europe/Athens" "Athen"
    "Atlantic/South_Georgia" "Südgeorgien"
    "America/Guatemala" inherited
    "Pacific/Guam" inherited
    "Africa/Bissau" inherited
    "America/Guyana" inherited
    "Asia/Hong_Kong" "Hongkong"
    "America/Tegucigalpa" inherited
    "America/Port-au-Prince" inherited
    "Europe/Budapest" inherited
    "Asia/Jakarta" inherited
    "Asia/Pontianak" inherited
    "Asia/Makassar" inherited
    "Asia/Jayapura" inherited
    "Europe/Dublin" inherited
    "Asia/Jerusalem" inherited
    "Asia/Kolkata" "Kalkutta"
    "Indian/Chagos" inherited
    "Asia/Baghdad" "Bagdad"
    "Asia/Tehran" "Teheran"
    "Europe/Rome" "Rom"
    "America/Jamaica" "Jamaika"
    "Asia/Amman" inherited
    "Asia/Tokyo" "Tokio"
    "Africa/Nairobi" inherited
    "Asia/Bishkek" "Bischkek"
    "Pacific/Tarawa" inherited
    "Pacific/Kanton" inherited
    "Pacific/Kiritimati" inherited
    "Asia/Pyongyang" "Pjöngjang"
    "Asia/Seoul" inherited
    "Asia/Almaty" inherited
    "Asia/Qyzylorda" "Qysylorda"
    "Asia/Qostanay" "Qostanai"
    "Asia/Aqtobe" "Aktobe"
    "Asia/Aqtau" inherited
    "Asia/Atyrau" inherited
    "Asia/Oral" inherited
    "Asia/Beirut" inherited
    "Asia/Colombo" inherited
    "Africa/Monrovia" inherited
    "Europe/Vilnius" inherited
    "Europe/Riga" inherited
    "Africa/Tripoli" "Tripolis"
    "Africa/Casablanca" inherited
    "Europe/Chisinau" inherited
    "Pacific/Kwajalein" inherited
    "Asia/Yangon" "Rangun"
    "Asia/Ulaanbaatar" inherited
    "Asia/Hovd" "Chowd"
    "Asia/Macau" "Macau"
    "America/Martinique" inherited
    "Europe/Malta" inherited
    "Indian/Mauritius" inherited
    "Indian/Maldives" "Malediven"
    "America/Mexico_City" "Mexiko-Stadt"
    "America/Cancun" inherited
    "America/Merida" "Merida"
    "America/Monterrey" inherited
    "America/Matamoros" inherited
    "America/Chihuahua" inherited
    "America/Ciudad_Juarez" inherited
    "America/Ojinaga" inherited
    "America/Mazatlan" inherited
    "America/Bahia_Banderas" "Bahia Banderas"
    "America/Hermosillo" inherited
    "America/Tijuana" inherited
    "Asia/Kuching" inherited
    "Africa/Maputo" inherited
    "Africa/Windhoek" inherited
    "Pacific/Noumea" inherited
    "Pacific/Norfolk" inherited
    "Africa/Lagos" inherited
    "America/Managua" inherited
    "Asia/Kathmandu" inherited
    "Pacific/Nauru" inherited
    "Pacific/Niue" inherited
    "Pacific/Auckland" inherited
    "Pacific/Chatham" inherited
    "America/Panama" inherited
    "America/Lima" inherited
    "Pacific/Tahiti" inherited
    "Pacific/Marquesas" inherited
    "Pacific/Gambier" inherited
    "Pacific/Port_Moresby" inherited
    "Pacific/Bougainville" inherited
    "Asia/Manila" inherited
    "Asia/Karachi" "Karatschi"
    "Europe/Warsaw" "Warschau"
    "America/Miquelon" inherited
    "Pacific/Pitcairn" inherited
    "America/Puerto_Rico" inherited
    "Asia/Gaza" inherited
    "Asia/Hebron" inherited
    "Europe/Lisbon" "Lissabon"
    "Atlantic/Madeira" inherited
    "Atlantic/Azores" "Azoren"
    "Pacific/Palau" inherited
    "America/Asuncion" inherited
    "Asia/Qatar" "Katar"
    "Europe/Bucharest" "Bukarest"
    "Europe/Belgrade" "Belgrad"
    "Europe/Kaliningrad" inherited
    "Europe/Moscow" "Moskau"
    "Europe/Simferopol" inherited
    "Europe/Kirov" "Kirow"
    "Europe/Volgograd" "Wolgograd"
    "Europe/Astrakhan" "Astrachan"
    "Europe/Saratov" "Saratow"
    "Europe/Ulyanovsk" "Uljanowsk"
    "Europe/Samara" inherited
    "Asia/Yekaterinburg" "Jekaterinburg"
    "Asia/Omsk" inherited
    "Asia/Novosibirsk" "Nowosibirsk"
    "Asia/Barnaul" inherited
    "Asia/Tomsk" inherited
    "Asia/Novokuznetsk" "Nowokuznetsk"
    "Asia/Krasnoyarsk" "Krasnojarsk"
    "Asia/Irkutsk" inherited
    "Asia/Chita" "Tschita"
    "Asia/Yakutsk" "Jakutsk"
    "Asia/Khandyga" "Chandyga"
    "Asia/Vladivostok" "Wladiwostok"
    "Asia/Ust-Nera" inherited
    "Asia/Magadan" inherited
    "Asia/Sakhalin" "Sachalin"
    "Asia/Srednekolymsk" inherited
    "Asia/Kamchatka" "Kamtschatka"
    "Asia/Anadyr" inherited
    "Asia/Riyadh" "Riad"
    "Pacific/Guadalcanal" inherited
    "Africa/Khartoum" "Khartum"
    "Asia/Singapore" "Singapur"
    "America/Paramaribo" inherited
    "Africa/Juba" inherited
    "Africa/Sao_Tome" inherited
    "America/El_Salvador" inherited
    "Asia/Damascus" "Damaskus"
    "America/Grand_Turk" inherited
    "Africa/Ndjamena" inherited
    "Asia/Bangkok" inherited
    "Asia/Dushanbe" "Duschanbe"
    "Pacific/Fakaofo" inherited
    "Asia/Dili" inherited
    "Asia/Ashgabat" "Aşgabat"
    "Africa/Tunis" inherited
    "Pacific/Tongatapu" inherited
    "Europe/Istanbul" inherited
    "Asia/Taipei" "Taipeh"
    "Europe/Kyiv" "Kiew"
    "America/New_York" inherited
    "America/Detroit" inherited
    "America/Kentucky/Louisville" inherited
    "America/Kentucky/Monticello" inherited
    "America/Indiana/Indianapolis" inherited
    "America/Indiana/Vincennes" inherited
    "America/Indiana/Winamac" inherited
    "America/Indiana/Marengo" inherited
    "America/Indiana/Petersburg" inherited
    "America/Indiana/Vevay" inherited
    "America/Chicago" inherited
    "America/Indiana/Tell_City" inherited
    "America/Indiana/Knox" inherited
    "America/Menominee" inherited
    "America/North_Dakota/Center" inherited
    "America/North_Dakota/New_Salem" inherited
    "America/North_Dakota/Beulah" inherited
    "America/Denver" inherited
    "America/Boise" inherited
    "America/Phoenix" inherited
    "America/Los_Angeles" inherited
    "America/Anchorage" inherited
    "America/Juneau" inherited
    "America/Sitka" inherited
    "America/Metlakatla" inherited
    "America/Yakutat" inherited
    "America/Nome" inherited
    "America/Adak" inherited
    "Pacific/Honolulu" "Honolulu"
    "America/Montevideo" inherited
    "Asia/Samarkand" inherited
    "Asia/Tashkent" "Taschkent"
    "America/Caracas" inherited
    "Asia/Ho_Chi_Minh" "Ho-Chi-Minh-Stadt"
    "Pacific/Efate" inherited
    "Pacific/Apia" inherited
    "Africa/Johannesburg" inherited
    "America/Antigua" inherited
    "America/Anguilla" inherited
    "Africa/Luanda" inherited
    "Antarctica/McMurdo" inherited
    "Antarctica/DumontDUrville" inherited
    "Antarctica/Syowa" inherited
    "America/Aruba" inherited
    "Europe/Mariehamn" inherited
    "Europe/Sarajevo" inherited
    "Africa/Ouagadougou" inherited
    "Asia/Bahrain" inherited
    "Africa/Bujumbura" inherited
    "Africa/Porto-Novo" "Porto Novo"
    "America/St_Barthelemy" "Saint-Barthélemy"
    "Asia/Brunei" "Brunei Darussalam"
    "America/Kralendijk" inherited
    "America/Nassau" inherited
    "Africa/Gaborone" inherited
    "America/Blanc-Sablon" inherited
    "America/Atikokan" inherited
    "America/Creston" inherited
    "Indian/Cocos" inherited
    "Africa/Kinshasa" inherited
    "Africa/Lubumbashi" inherited
    "Africa/Bangui" inherited
    "Africa/Brazzaville" inherited
    "Africa/Douala" inherited
    "America/Curacao" inherited
    "Indian/Christmas" "Weihnachtsinsel"
    "Europe/Busingen" inherited
    "Africa/Djibouti" "Dschibuti"
    "Europe/Copenhagen" "Kopenhagen"
    "America/Dominica" inherited
    "Africa/Asmara" inherited
    "Africa/Addis_Ababa" "Addis Abeba"
    "Pacific/Chuuk" inherited
    "Pacific/Pohnpei" inherited
    "Africa/Libreville" inherited
    "America/Grenada" inherited
    "Europe/Guernsey" inherited
    "Africa/Accra" inherited
    "Africa/Banjul" inherited
    "Africa/Conakry" inherited
    "America/Guadeloupe" inherited
    "Africa/Malabo" inherited
    "Europe/Zagreb" inherited
    "Europe/Isle_of_Man" inherited
    "Atlantic/Reykjavik" "Reyk­ja­vík"
    "Europe/Jersey" inherited
    "Asia/Phnom_Penh" inherited
    "Indian/Comoro" "Komoren"
    "America/St_Kitts" inherited
    "Asia/Kuwait" inherited
    "America/Cayman" "Kaimaninseln"
    "Asia/Vientiane" inherited
    "America/St_Lucia" inherited
    "Europe/Vaduz" inherited
    "Africa/Maseru" inherited
    "Europe/Luxembourg" "Luxemburg"
    "Europe/Monaco" inherited
    "Europe/Podgorica" inherited
    "America/Marigot" inherited
    "Indian/Antananarivo" inherited
    "Pacific/Majuro" inherited
    "Europe/Skopje" inherited
    "Africa/Bamako" inherited
    "Pacific/Saipan" inherited
    "Africa/Nouakchott" inherited
    "America/Montserrat" inherited
    "Africa/Blantyre" inherited
    "Asia/Kuala_Lumpur" inherited
    "Africa/Niamey" inherited
    "Europe/Amsterdam" inherited
    "Europe/Oslo" inherited
    "Asia/Muscat" "Maskat"
    "Indian/Reunion" inherited
    "Africa/Kigali" inherited
    "Indian/Mahe" inherited
    "Europe/Stockholm" inherited
    "Atlantic/St_Helena" inherited
    "Europe/Ljubljana" inherited
    "Arctic/Longyearbyen" inherited
    "Europe/Bratislava" inherited
    "Africa/Freetown" inherited
    "Europe/San_Marino" inherited
    "Africa/Dakar" inherited
    "Africa/Mogadishu" "Mogadischu"
    "America/Lower_Princes" inherited
    "Africa/Mbabane" inherited
    "Indian/Kerguelen" inherited
    "Africa/Lome" inherited
    "America/Port_of_Spain" inherited
    "Pacific/Funafuti" inherited
    "Africa/Dar_es_Salaam" "Daressalam"
    "Africa/Kampala" inherited
    "Pacific/Midway" inherited
    "Pacific/Wake" inherited
    "Europe/Vatican" "Vatikan"
    "America/St_Vincent" inherited
    "America/Tortola" inherited
    "America/St_Thomas" inherited
    "Pacific/Wallis" inherited
    "Asia/Aden" inherited
    "Indian/Mayotte" inherited
    "Africa/Lusaka" inherited
    "Africa/Harare" inherited
};

// `common/main/es.xml`: 157 of the 418 zones named, 261 inherited.
#[cfg(feature = "localized-exemplar-cities")]
const ES: &str = exemplar_cities! {
    "Europe/Andorra" inherited
    "Asia/Dubai" "Dubái"
    "Asia/Kabul" inherited
    "Europe/Tirane" inherited
    "Asia/Yerevan" "Ereván"
    "Antarctica/Casey" inherited
    "Antarctica/Davis" inherited
    "Antarctica/Mawson" inherited
    "Antarctica/Palmer" inherited
    "Antarctica/Rothera" inherited
    "Antarctica/Troll" inherited
    "Antarctica/Vostok" inherited
    "America/Argentina/Buenos_Aires" inherited
    "America/Argentina/Cordoba" inherited
    "America/Argentina/Salta" inherited
    "America/Argentina/Jujuy" inherited
    "America/Argentina/Tucuman" inherited
    "America/Argentina/Catamarca" inherited
    "America/Argentina/La_Rioja" inherited
    "America/Argentina/San_Juan" inherited
    "America/Argentina/Mendoza" inherited
    "America/Argentina/San_Luis" inherited
    "America/Argentina/Rio_Gallegos" inherited
    "America/Argentina/Ushuaia" inherited
    "Pacific/Pago_Pago" inherited
    "Europe/Vienna" "Viena"
    "Australia/Lord_Howe" inherited
    "Antarctica/Macquarie" inherited
    "Australia/Hobart" inherited
    "Australia/Melbourne" inherited
    "Australia/Sydney" "Sídney"
    "Australia/Broken_Hill" inherited
    "Australia/Brisbane" inherited
    "Australia/Lindeman" inherited
    "Australia/Adelaide" "Adelaida"
    "Australia/Darwin" inherited
    "Australia/Perth" inherited
    "Australia/Eucla" inherited
    "Asia/Baku" "Bakú"
    "America/Barbados" inherited
    "Asia/Dhaka" "Daca"
    "Europe/Brussels" "Bruselas"
    "Europe/Sofia" "Sofía"
    "Atlantic/Bermuda" "Bermudas"
    "America/La_Paz" inherited
    "America/Noronha" inherited
    "America/Belem" "Belén"
    "America/Fortaleza" inherited
    "America/Recife" inherited
    "America/Araguaina" inherited
    "America/Maceio" inherited
    "America/Bahia" "Bahía"
    "America/Sao_Paulo" inherited
    "America/Campo_Grande" inherited
    "America/Cuiaba" inherited
    "America/Santarem" inherited
    "America/Porto_Velho" inherited
    "America/Boa_Vista" inherited
    "America/Manaus" "Manaos"
    "America/Eirunepe" inherited
    "America/Rio_Branco" "Río Branco"
    "Asia/Thimphu" "Timbu"
    "Europe/Minsk" inherited
    "America/Belize" "Belice"
    "America/St_Johns" "San Juan de Terranova"
    "America/Halifax" inherited
    "America/Glace_Bay" inherited
    "America/Moncton" inherited
    "America/Goose_Bay" inherited
    "America/Toronto" inherited
    "America/Iqaluit" inherited
    "America/Winnipeg" inherited
    "America/Resolute" inherited
    "America/Rankin_Inlet" inherited
    "America/Regina" inherited
    "America/Swift_Current" inherited
    "America/Edmonton" inherited
    "America/Cambridge_Bay" inherited
    "America/Inuvik" inherited
    "America/Vancouver" inherited
    "America/Dawson_Creek" inherited
    "America/Fort_Nelson" inherited
    "America/Whitehorse" inherited
    "America/Dawson" inherited
    "Europe/Zurich" "Zúrich"
    "Africa/Abidjan" "Abiyán"
    "Pacific/Rarotonga" inherited
    "America/Santiago" "Santiago de Chile"
    "America/Coyhaique" inherited
    "America/Punta_Arenas" inherited
    "Pacific/Easter" "Isla de Pascua"
    "Asia/Shanghai" "Shanghái"
    "Asia/Urumqi" inherited
    "America/Bogota" inherited
    "America/Costa_Rica" inherited
    "America/Havana" "La Habana"
    "Atlantic/Cape_Verde" "Cabo Verde"
    "Asia/Nicosia" inherited
    "Asia/Famagusta" inherited
    "Europe/Prague" "Praga"
    "Europe/Berlin" "Berlín"
    "America/Santo_Domingo" inherited
    "Africa/Algiers" "Argel"
    "America/Guayaquil" inherited
    "Pacific/Galapagos" inherited
    "Europe/Tallinn" "Tallin"
    "Africa/Cairo" "El Cairo"
    "Africa/El_Aaiun" inherited
    "Europe/Madrid" inherited
    "Africa/Ceuta" inherited
    "Atlantic/Canary" inherited
    "Europe/Helsinki" inherited
    "Pacific/Fiji" "Fiyi"
    "Atlantic/Stanley" inherited
    "Pacific/Kosrae" inherited
    "Atlantic/Faroe" "Islas Feroe"
    "Europe/Paris" "París"
    "Europe/London" "Londres"
    "Asia/Tbilisi" "Tiflis"
    "America/Cayenne" "Cayena"
    "Europe/Gibraltar" inherited
    "America/Nuuk" inherited
    "America/Danmarkshavn" inherited
    "America/Scoresbysund" inherited
    "America/Thule" inherited
    "Europe/Athens" "Atenas"
    "Atlantic/South_Georgia" "Georgia del Sur"
    "America/Guatemala" inherited
    "Pacific/Guam" inherited
    "Africa/Bissau" "Bisáu"
    "America/Guyana" inherited
    "Asia/Hong_Kong" inherited
    "America/Tegucigalpa" inherited
    "America/Port-au-Prince" "Puerto Príncipe"
    "Europe/Budapest" inherited
    "Asia/Jakarta" "Yakarta"
    "Asia/Pontianak" inherited
    "Asia/Makassar" "Makasar"
    "Asia/Jayapura" inherited
    "Europe/Dublin" "Dublín"
    "Asia/Jerusalem" "Jerusalén"
    "Asia/Kolkata" "Calcuta"
    "Indian/Chagos" inherited
    "Asia/Baghdad" "Bagdad"
    "Asia/Tehran" "Teherán"
    "Europe/Rome" "Roma"
    "America/Jamaica" inherited
    "Asia/Amman" "Ammán"
    "Asia/Tokyo" "Tokio"
    "Africa/Nairobi" inherited
    "Asia/Bishkek" inherited
    "Pacific/Tarawa" inherited
    "Pacific/Kanton" "Isla Kanton"
    "Pacific/Kiritimati" inherited
    "Asia/Pyongyang" inherited
    "Asia/Seoul" "Seúl"
    "Asia/Almaty" inherited
    "Asia/Qyzylorda" "Kyzylorda"
    "Asia/Qostanay" "Kostanái"
    "Asia/Aqtobe" "Aktobe"
    "Asia/Aqtau" "Aktau"
    "Asia/Atyrau" inherited
    "Asia/Oral" inherited
    "Asia/Beirut" inherited
    "Asia/Colombo" inherited
    "Africa/Monrovia" inherited
    "Europe/Vilnius" "Vilna"
    "Europe/Riga" inherited
    "Africa/Tripoli" "Trípoli"
    "Africa/Casablanca" inherited
    "Europe/Chisinau" "Chisináu"
    "Pacific/Kwajalein" inherited
    "Asia/Yangon" "Yangón (Rangún)"
    "Asia/Ulaanbaatar" "Ulán Bator"
    "Asia/Hovd" inherited
    "Asia/Macau" inherited
    "America/Martinique" "Martinica"
    "Europe/Malta" inherited
    "Indian/Mauritius" "Mauricio"
    "Indian/Maldives" "Maldivas"
    "America/Mexico_City" inherited
    "America/Cancun" inherited
    "America/Merida" inherited
    "America/Monterrey" inherited
    "America/Matamoros" inherited
    "America/Chihuahua" inherited
    "America/Ciudad_Juarez" inherited
    "America/Ojinaga" inherited
    "America/Mazatlan" inherited
    "America/Bahia_Banderas" inherited
    "America/Hermosillo" inherited
    "America/Tijuana" inherited
    "Asia/Kuching" inherited
    "Africa/Maputo" inherited
    "Africa/Windhoek" inherited
    "Pacific/Noumea" "Numea"
    "Pacific/Norfolk" inherited
    "Africa/Lagos" inherited
    "America/Managua" inherited
    "Asia/Kathmandu" "Katmandú"
    "Pacific/Nauru" inherited
    "Pacific/Niue" inherited
    "Pacific/Auckland" inherited
    "Pacific/Chatham" inherited
    "America/Panama" "Panamá"
    "America/Lima" inherited
    "Pacific/Tahiti" "Tahití"
    "Pacific/Marquesas" inherited
    "Pacific/Gambier" inherited
    "Pacific/Port_Moresby" inherited
    "Pacific/Bougainville" inherited
    "Asia/Manila" inherited
    "Asia/Karachi" inherited
    "Europe/Warsaw" "Varsovia"
    "America/Miquelon" "Miquelón"
    "Pacific/Pitcairn" inherited
    "America/Puerto_Rico" inherited
    "Asia/Gaza" inherited
    "Asia/Hebron" "Hebrón"
    "Europe/Lisbon" "Lisboa"
    "Atlantic/Madeira" inherited
    "Atlantic/Azores" inherited
    "Pacific/Palau" "Palaos"
    "America/Asuncion" inherited
    "Asia/Qatar" "Catar"
    "Europe/Bucharest" "Bucarest"
    "Europe/Belgrade" "Belgrado"
    "Europe/Kaliningrad" "Kaliningrado"
    "Europe/Moscow" "Moscú"
    "Europe/Simferopol" "Simferópol"
    "Europe/Kirov" "Kírov"
    "Europe/Volgograd" "Volgogrado"
    "Europe/Astrakhan" "Astracán"
    "Europe/Saratov" "Sarátov"
    "Europe/Ulyanovsk" "Uliánovsk"
    "Europe/Samara" inherited
    "Asia/Yekaterinburg" "Ekaterimburgo"
    "Asia/Omsk" inherited
    "Asia/Novosibirsk" inherited
    "Asia/Barnaul" "Barnaúl"
    "Asia/Tomsk" inherited
    "Asia/Novokuznetsk" inherited
    "Asia/Krasnoyarsk" inherited
    "Asia/Irkutsk" inherited
    "Asia/Chita" "Chitá"
    "Asia/Yakutsk" inherited
    "Asia/Khandyga" "Khandiga"
    "Asia/Vladivostok" inherited
    "Asia/Ust-Nera" inherited
    "Asia/Magadan" "Magadán"
    "Asia/Sakhalin" "Sajalín"
    "Asia/Srednekolymsk" "Srednekolimsk"
    "Asia/Kamchatka" inherited
    "Asia/Anadyr" "Anádyr"
    "Asia/Riyadh" "Riad"
    "Pacific/Guadalcanal" inherited
    "Africa/Khartoum" "Jartum"
    "Asia/Singapore" "Singapur"
    "America/Paramaribo" inherited
    "Africa/Juba" inherited
    "Africa/Sao_Tome" "Santo Tomé"
    "America/El_Salvador" inherited
    "Asia/Damascus" "Damasco"
    "America/Grand_Turk" "Gran Turca"
    "Africa/Ndjamena" "Yamena"
    "Asia/Bangkok" inherited
    "Asia/Dushanbe" "Dusambé"
    "Pacific/Fakaofo" inherited
    "Asia/Dili" inherited
    "Asia/Ashgabat" "Asjabad"
    "Africa/Tunis" "Túnez"
    "Pacific/Tongatapu" inherited
    "Europe/Istanbul" "Estambul"
    "Asia/Taipei" "Taipéi"
    "Europe/Kyiv" "Kiev"
    "America/New_York" "Nueva York"
    "America/Detroit" inherited
    "America/Kentucky/Louisville" inherited
    "America/Kentucky/Monticello" inherited
    "America/Indiana/Indianapolis" "Indianápolis"
    "America/Indiana/Vincennes" inherited
    "America/Indiana/Winamac" inherited
    "America/Indiana/Marengo" inherited
    "America/Indiana/Petersburg" inherited
    "America/Indiana/Vevay" inherited
    "America/Chicago" inherited
    "America/Indiana/Tell_City" inherited
    "America/Indiana/Knox" inherited
    "America/Menominee" inherited
    "America/North_Dakota/Center" "Center, Dakota del Norte"
    "America/North_Dakota/New_Salem" "New Salem, Dakota del Norte"
    "America/North_Dakota/Beulah" "Beulah, Dakota del Norte"
    "America/Denver" inherited
    "America/Boise" inherited
    "America/Phoenix" inherited
    "America/Los_Angeles" "Los Ángeles"
    "America/Anchorage" inherited
    "America/Juneau" inherited
    "America/Sitka" inherited
    "America/Metlakatla" inherited
    "America/Yakutat" inherited
    "America/Nome" inherited
    "America/Adak" inherited
    "Pacific/Honolulu" "Honolulú"
    "America/Montevideo" inherited
    "Asia/Samarkand" "Samarcanda"
    "Asia/Tashkent" "Taskent"
    "America/Caracas" inherited
    "Asia/Ho_Chi_Minh" "Ciudad Ho Chi Minh"
    "Pacific/Efate" inherited
    "Pacific/Apia" inherited
    "Africa/Johannesburg" "Johannesburgo"
    "America/Antigua" inherited
    "America/Anguilla" "Anguila"
    "Africa/Luanda" inherited
    "Antarctica/McMurdo" inherited
    "Antarctica/DumontDUrville" inherited
    "Antarctica/Syowa" inherited
    "America/Aruba" inherited
    "Europe/Mariehamn" inherited
    "Europe/Sarajevo" inherited
    "Africa/Ouagadougou" "Uagadugú"
    "Asia/Bahrain" "Baréin"
    "Africa/Bujumbura" inherited
    "Africa/Porto-Novo" "Portonovo"
    "America/St_Barthelemy" "San Bartolomé"
    "Asia/Brunei" "Brunéi"
    "America/Kralendijk" inherited
    "America/Nassau" inherited
    "Africa/Gaborone" inherited
    "America/Blanc-Sablon" inherited
    "America/Atikokan" inherited
    "America/Creston" inherited
    "Indian/Cocos" inherited
    "Africa/Kinshasa" inherited
    "Africa/Lubumbashi" inherited
    "Africa/Bangui" inherited
    "Africa/Brazzaville" inherited
    "Africa/Douala" "Duala"
    "America/Curacao" "Curazao"
    "Indian/Christmas" "Navidad"
    "Europe/Busingen" inherited
    "Africa/Djibouti" "Yibuti"
    "Europe/Copenhagen" "Copenhague"
    "America/Dominica" inherited
    "Africa/Asmara" inherited
    "Africa/Addis_Ababa" "Adís Abeba"
    "Pacific/Chuuk" inherited
    "Pacific/Pohnpei" inherited
    "Africa/Libreville" inherited
    "America/Grenada" "Granada"
    "Europe/Guernsey" "Guernesey"
    "Africa/Accra" "Acra"
    "Africa/Banjul" inherited
    "Africa/Conakry" "Conakri"
    "America/Guadeloupe" "Guadalupe"
    "Africa/Malabo" inherited
    "Europe/Zagreb" inherited
    "Europe/Isle_of_Man" "Isla de Man"
    "Atlantic/Reykjavik" "Reikiavik"
    "Europe/Jersey" inherited
    "Asia/Phnom_Penh" inherited
    "Indian/Comoro" "Comoras"
    "America/St_Kitts" "San Cristóbal"
    "Asia/Kuwait" inherited
    "America/Cayman" "Caimán"
    "Asia/Vientiane" "Vientián"
    "America/St_Lucia" "Santa Lucía"
    "Europe/Vaduz" inherited
    "Africa/Maseru" inherited
    "Europe/Luxembourg" "Luxemburgo"
    "Europe/Monaco" "Mónaco"
    "Europe/Podgorica" inherited
    "America/Marigot" inherited
    "Indian/Antananarivo" inherited
    "Pacific/Majuro" inherited
    "Europe/Skopje" "Skopie"
    "Africa/Bamako" inherited
    "Pacific/Saipan" "Saipán"
    "Africa/Nouakchott" "Nuakchot"
    "America/Montserrat" inherited
    "Africa/Blantyre" inherited
    "Asia/Kuala_Lumpur" inherited
    "Africa/Niamey" inherited
    "Europe/Amsterdam" "Ámsterdam"
    "Europe/Oslo" inherited
    "Asia/Muscat" "Mascate"
    "Indian/Reunion" "Reunión"
    "Africa/Kigali" inherited
    "Indian/Mahe" inherited
    "Europe/Stockholm" "Estocolmo"
    "Atlantic/St_Helena" "Santa Elena"
    "Europe/Ljubljana" "Liubliana"
    "Arctic/Longyearbyen" inherited
    "Europe/Bratislava" inherited
    "Africa/Freetown" inherited
    "Europe/San_Marino" inherited
    "Africa/Dakar" inherited
    "Africa/Mogadishu" "Mogadiscio"
    "America/Lower_Princes" inherited
    "Africa/Mbabane" inherited
    "Indian/Kerguelen" inherited
    "Africa/Lome" inherited
    "America/Port_of_Spain" "Puerto España"
    "Pacific/Funafuti" inherited
    "Africa/Dar_es_Salaam" "Dar es-Salam"
    "Africa/Kampala" inherited
    "Pacific/Midway" inherited
    "Pacific/Wake" inherited
    "Europe/Vatican" "El Vaticano"
    "America/St_Vincent" "San Vicente"
    "America/Tortola" "Tórtola"
    "America/St_Thomas" inherited
    "Pacific/Wallis" inherited
    "Asia/Aden" "Adén"
    "Indian/Mayotte" inherited
    "Africa/Lusaka" inherited
    "Africa/Harare" inherited
};

// `common/main/fa.xml`: 418 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const FA: &str = exemplar_cities! {
    "Europe/Andorra" "آندورا"
    "Asia/Dubai" "دبی"
    "Asia/Kabul" "کابل"
    "Europe/Tirane" "تیرانا"
    "Asia/Yerevan" "ایروان"
    "Antarctica/Casey" "کیسی"
    "Antarctica/Davis" "دیویس"
    "Antarctica/Mawson" "ماوسون"
    "Antarctica/Palmer" "پالمر"
    "Antarctica/Rothera" "روترا"
    "Antarctica/Troll" "ترول"
    "Antarctica/Vostok" "وستوک"
    "America/Argentina/Buenos_Aires" "بوئنوس‌آیرس"
    "America/Argentina/Cordoba" "کوردووا"
    "America/Argentina/Salta" "سالتا"
    "America/Argentina/Jujuy" "خوخوی"
    "America/Argentina/Tucuman" "توکومن"
    "America/Argentina/Catamarca" "کاتامارکا"
    "America/Argentina/La_Rioja" "لاریوخا"
    "America/Argentina/San_Juan" "سن‌خوان"
    "America/Argentina/Mendoza" "مندوسا"
    "America/Argentina/San_Luis" "سن‌لوئیس"
    "America/Argentina/Rio_Gallegos" "ریوگالگوس"
    "America/Argentina/Ushuaia" "اوشوایا"
    "Pacific/Pago_Pago" "پاگوپاگو"
    "Europe/Vienna" "وین"
    "Australia/Lord_Howe" "لردهاو"
    "Antarctica/Macquarie" "مکواری"
    "Australia/Hobart" "هوبارت"
    "Australia/Melbourne" "ملبورن"
    "Australia/Sydney" "سیدنی"
    "Australia/Broken_Hill" "بروکن‌هیل"
    "Australia/Brisbane" "بریسبین"
    "Australia/Lindeman" "لیندمن"
    "Australia/Adelaide" "آدلاید"
    "Australia/Darwin" "داروین"
    "Australia/Perth" "پرت"
    "Australia/Eucla" "اوکلا"
    "Asia/Baku" "باکو"
    "America/Barbados" "باربادوس"
    "Asia/Dhaka" "داکا"
    "Europe/Brussels" "بروکسل"
    "Europe/Sofia" "صوفیه"
    "Atlantic/Bermuda" "برمودا"
    "America/La_Paz" "لاپاز"
    "America/Noronha" "نورونیا"
    "America/Belem" "بلم"
    "America/Fortaleza" "فورتالزا"
    "America/Recife" "ریسیفی"
    "America/Araguaina" "آراگواینا"
    "America/Maceio" "ماسیو"
    "America/Bahia" "بایا"
    "America/Sao_Paulo" "سائوپائولو"
    "America/Campo_Grande" "کمپو گرانده"
    "America/Cuiaba" "کویاوا"
    "America/Santarem" "سنتارم"
    "America/Porto_Velho" "پورتوولیو"
    "America/Boa_Vista" "بوئاویستا"
    "America/Manaus" "ماناوس"
    "America/Eirunepe" "ایرونپه"
    "America/Rio_Branco" "ریوبرانکو"
    "Asia/Thimphu" "تیمفو"
    "Europe/Minsk" "مینسک"
    "America/Belize" "بلیز"
    "America/St_Johns" "سنت جان"
    "America/Halifax" "هلیفکس"
    "America/Glace_Bay" "گلیس‌بی"
    "America/Moncton" "مانکتون"
    "America/Goose_Bay" "گوس‌بی"
    "America/Toronto" "تورنتو"
    "America/Iqaluit" "ایکلوئت"
    "America/Winnipeg" "وینیپگ"
    "America/Resolute" "رزولوت"
    "America/Rankin_Inlet" "خلیجک رنکین"
    "America/Regina" "رجاینا"
    "America/Swift_Current" "سویفت‌کارنت"
    "America/Edmonton" "ادمونتون"
    "America/Cambridge_Bay" "کمبریج‌بی"
    "America/Inuvik" "اینوویک"
    "America/Vancouver" "ونکوور"
    "America/Dawson_Creek" "داوسن کریک"
    "America/Fort_Nelson" "فورت نلسون"
    "America/Whitehorse" "وایت‌هورس"
    "America/Dawson" "داوسن"
    "Europe/Zurich" "زوریخ"
    "Africa/Abidjan" "آبیجان"
    "Pacific/Rarotonga" "راروتونگا"
    "America/Santiago" "سانتیاگو"
    "America/Coyhaique" "کویهایکیو"
    "America/Punta_Arenas" "پونتا آرناس"
    "Pacific/Easter" "ایستر"
    "Asia/Shanghai" "شانگهای"
    "Asia/Urumqi" "ارومچی"
    "America/Bogota" "بوگوتا"
    "America/Costa_Rica" "کاستاریکا"
    "America/Havana" "هاوانا"
    "Atlantic/Cape_Verde" "کیپ‌ورد"
    "Asia/Nicosia" "نیکوزیا"
    "Asia/Famagusta" "فاماگوستا"
    "Europe/Prague" "پراگ"
    "Europe/Berlin" "برلین"
    "America/Santo_Domingo" "سانتو دومینگو"
    "Africa/Algiers" "الجزیره"
    "America/Guayaquil" "گوایاکیل"
    "Pacific/Galapagos" "گالاپاگوس"
    "Europe/Tallinn" "تالین"
    "Africa/Cairo" "قاهره"
    "Africa/El_Aaiun" "العیون"
    "Europe/Madrid" "مادرید"
    "Africa/Ceuta" "سبته"
    "Atlantic/Canary" "قناری"
    "Europe/Helsinki" "هلسینکی"
    "Pacific/Fiji" "فیجی"
    "Atlantic/Stanley" "استانلی"
    "Pacific/Kosrae" "کوسرای"
    "Atlantic/Faroe" "فارو"
    "Europe/Paris" "پاریس"
    "Europe/London" "لندن"
    "Asia/Tbilisi" "تفلیس"
    "America/Cayenne" "کاین"
    "Europe/Gibraltar" "جبل‌الطارق"
    "America/Nuuk" "نووک"
    "America/Danmarkshavn" "دانمارکس‌هاون"
    "America/Scoresbysund" "اسکورسبیسوند"
    "America/Thule" "تول"
    "Europe/Athens" "آتن"
    "Atlantic/South_Georgia" "جورجیای جنوبی"
    "America/Guatemala" "گواتمالا"
    "Pacific/Guam" "گوام"
    "Africa/Bissau" "بیسائو"
    "America/Guyana" "گویان"
    "Asia/Hong_Kong" "هنگ‌کنگ"
    "America/Tegucigalpa" "تگوسیگالپا"
    "America/Port-au-Prince" "پورتوپرنس"
    "Europe/Budapest" "بوداپست"
    "Asia/Jakarta" "جاکارتا"
    "Asia/Pontianak" "پونتیاناک"
    "Asia/Makassar" "ماکاسار"
    "Asia/Jayapura" "جایاپورا"
    "Europe/Dublin" "دوبلین"
    "Asia/Jerusalem" "اورشلیم"
    "Asia/Kolkata" "کلکته"
    "Indian/Chagos" "شاگوس"
    "Asia/Baghdad" "بغداد"
    "Asia/Tehran" "تهران"
    "Europe/Rome" "رم"
    "America/Jamaica" "جامائیکا"
    "Asia/Amman" "عَمان"
    "Asia/Tokyo" "توکیو"
    "Africa/Nairobi" "نایروبی"
    "Asia/Bishkek" "بیشکک"
    "Pacific/Tarawa" "تاراوا"
    "Pacific/Kanton" "کانتون"
    "Pacific/Kiritimati" "کریتیماتی"
    "Asia/Pyongyang" "پیونگ‌یانگ"
    "Asia/Seoul" "سئول"
    "Asia/Almaty" "آلماتی"
    "Asia/Qyzylorda" "قیزیل‌اوردا"
    "Asia/Qostanay" "قوستانای"
    "Asia/Aqtobe" "آقتوبه"
    "Asia/Aqtau" "آقتاو"
    "Asia/Atyrau" "آتیراو"
    "Asia/Oral" "اورال"
    "Asia/Beirut" "بیروت"
    "Asia/Colombo" "کلمبو"
    "Africa/Monrovia" "مونروویا"
    "Europe/Vilnius" "ویلنیوس"
    "Europe/Riga" "ریگا"
    "Africa/Tripoli" "طرابلس"
    "Africa/Casablanca" "کازابلانکا"
    "Europe/Chisinau" "کیشیناو"
    "Pacific/Kwajalein" "کواجیلین"
    "Asia/Yangon" "یانگون"
    "Asia/Ulaanbaatar" "اولان‌باتور"
    "Asia/Hovd" "خوود"
    "Asia/Macau" "ماکائو"
    "America/Martinique" "مارتینیک"
    "Europe/Malta" "مالت"
    "Indian/Mauritius" "موریس"
    "Indian/Maldives" "مالدیو"
    "America/Mexico_City" "مکزیکوسیتی"
    "America/Cancun" "کانکون"
    "America/Merida" "مریدا"
    "America/Monterrey" "مونتری"
    "America/Matamoros" "ماتاموروس"
    "America/Chihuahua" "چیواوا"
    "America/Ciudad_Juarez" "سیوداد خوارز"
    "America/Ojinaga" "اوجیناگا"
    "America/Mazatlan" "ماساتلان"
    "America/Bahia_Banderas" "باهیا باندراس"
    "America/Hermosillo" "ارموسیو"
    "America/Tijuana" "تیخوانا"
    "Asia/Kuching" "کوچینگ"
    "Africa/Maputo" "ماپوتو"
    "Africa/Windhoek" "ویندهوک"
    "Pacific/Noumea" "نومئا"
    "Pacific/Norfolk" "نورفولک"
    "Africa/Lagos" "لاگوس"
    "America/Managua" "ماناگوا"
    "Asia/Kathmandu" "کاتماندو"
    "Pacific/Nauru" "نائورو"
    "Pacific/Niue" "نیوئه"
    "Pacific/Auckland" "اوکلند"
    "Pacific/Chatham" "چت‌هام"
    "America/Panama" "پاناما"
    "America/Lima" "لیما"
    "Pacific/Tahiti" "تاهیتی"
    "Pacific/Marquesas" "مارکوزه"
    "Pacific/Gambier" "گامبیر"
    "Pacific/Port_Moresby" "پورت‌مورزبی"
    "Pacific/Bougainville" "بوگنویل"
    "Asia/Manila" "مانیل"
    "Asia/Karachi" "کراچی"
    "Europe/Warsaw" "ورشو"
    "America/Miquelon" "میکلون"
    "Pacific/Pitcairn" "پیت‌کرن"
    "America/Puerto_Rico" "پورتوریکو"
    "Asia/Gaza" "غزه"
    "Asia/Hebron" "الخلیل"
    "Europe/Lisbon" "لیسبون"
    "Atlantic/Madeira" "مادیرا"
    "Atlantic/Azores" "آزور"
    "Pacific/Palau" "پالائو"
    "America/Asuncion" "آسونسیون"
    "Asia/Qatar" "قطر"
    "Europe/Bucharest" "بخارست"
    "Europe/Belgrade" "بلگراد"
    "Europe/Kaliningrad" "کالینینگراد"
    "Europe/Moscow" "مسکو"
    "Europe/Simferopol" "سیمفروپل"
    "Europe/Kirov" "کیروف"
    "Europe/Volgograd" "ولگاگراد"
    "Europe/Astrakhan" "آستراخان"
    "Europe/Saratov" "ساراتوف"
    "Europe/Ulyanovsk" "اولیانوفسک"
    "Europe/Samara" "سامارا"
    "Asia/Yekaterinburg" "یکاترینبرگ"
    "Asia/Omsk" "اومسک"
    "Asia/Novosibirsk" "نووسیبیریسک"
    "Asia/Barnaul" "بارنائول"
    "Asia/Tomsk" "تومسک"
    "Asia/Novokuznetsk" "نوووکوزنتسک"
    "Asia/Krasnoyarsk" "کراسنویارسک"
    "Asia/Irkutsk" "ایرکوتسک"
    "Asia/Chita" "چیتا"
    "Asia/Yakutsk" "یاکوتسک"
    "Asia/Khandyga" "خاندیگا"
    "Asia/Vladivostok" "ولادی‌وستوک"
    "Asia/Ust-Nera" "اوست نرا"
    "Asia/Magadan" "ماگادان"
    "Asia/Sakhalin" "ساخالین"
    "Asia/Srednekolymsk" "اسردنکولیمسک"
    "Asia/Kamchatka" "کامچاتکا"
    "Asia/Anadyr" "آنادیر"
    "Asia/Riyadh" "ریاض"
    "Pacific/Guadalcanal" "گوادال‌کانال"
    "Africa/Khartoum" "خارطوم"
    "Asia/Singapore" "سنگاپور"
    "America/Paramaribo" "پاراماریبو"
    "Africa/Juba" "جوبا"
    "Africa/Sao_Tome" "سائوتومه"
    "America/El_Salvador" "السالوادور"
    "Asia/Damascus" "دمشق"
    "America/Grand_Turk" "گراند تورک"
    "Africa/Ndjamena" "انجامنا"
    "Asia/Bangkok" "بانکوک"
    "Asia/Dushanbe" "دوشنبه"
    "Pacific/Fakaofo" "فاکائوفو"
    "Asia/Dili" "دیلی"
    "Asia/Ashgabat" "عشق‌آباد"
    "Africa/Tunis" "تونس"
    "Pacific/Tongatapu" "تونگاتاپو"
    "Europe/Istanbul" "استانبول"
    "Asia/Taipei" "تایپه"
    "Europe/Kyiv" "کیف"
    "America/New_York" "نیویورک"
    "America/Detroit" "دیترویت"
    "America/Kentucky/Louisville" "لوئیزویل"
    "America/Kentucky/Monticello" "مانتیسلو، کنتاکی"
    "America/Indiana/Indianapolis" "ایندیاناپولیس"
    "America/Indiana/Vincennes" "وینسنس، اندیانا"
    "America/Indiana/Winamac" "ویناماک، ایندیانا"
    "America/Indiana/Marengo" "مارنگو، ایندیانا"
    "America/Indiana/Petersburg" "پیترزبرگ، ایندیانا"
    "America/Indiana/Vevay" "ویوی، ایندیانا"
    "America/Chicago" "شیکاگو"
    "America/Indiana/Tell_City" "تل‌سیتی، ایندیانا"
    "America/Indiana/Knox" "ناکس، ایندیانا"
    "America/Menominee" "منامینی"
    "America/North_Dakota/Center" "سنتر، داکوتای شمالی"
    "America/North_Dakota/New_Salem" "نیوسالم، داکوتای شمالی"
    "America/North_Dakota/Beulah" "بیولا، داکوتای شمالی"
    "America/Denver" "دنور"
    "America/Boise" "بویسی"
    "America/Phoenix" "فینکس"
    "America/Los_Angeles" "لوس‌آنجلس"
    "America/Anchorage" "انکوریج"
    "America/Juneau" "جونو"
    "America/Sitka" "سیتکا"
    "America/Metlakatla" "متالاکاتلا"
    "America/Yakutat" "یاکوتات"
    "America/Nome" "نوم"
    "America/Adak" "ایدک"
    "Pacific/Honolulu" "هونولولو"
    "America/Montevideo" "مونته‌ویدئو"
    "Asia/Samarkand" "سمرقند"
    "Asia/Tashkent" "تاشکند"
    "America/Caracas" "کاراکاس"
    "Asia/Ho_Chi_Minh" "هوشی‌مین‌سیتی"
    "Pacific/Efate" "افاته"
    "Pacific/Apia" "آپیا"
    "Africa/Johannesburg" "ژوهانسبورگ"
    "America/Antigua" "آنتیگوا"
    "America/Anguilla" "آنگوئیلا"
    "Africa/Luanda" "لواندا"
    "Antarctica/McMurdo" "مک‌موردو"
    "Antarctica/DumontDUrville" "دومون دورویل"
    "Antarctica/Syowa" "شووا"
    "America/Aruba" "اروبا"
    "Europe/Mariehamn" "ماریه‌هامن"
    "Europe/Sarajevo" "سارایوو"
    "Africa/Ouagadougou" "اوآگادوگو"
    "Asia/Bahrain" "بحرین"
    "Africa/Bujumbura" "بوجومبورا"
    "Africa/Porto-Novo" "پورتو نووو"
    "America/St_Barthelemy" "سنت بارتلمی"
    "Asia/Brunei" "برونئی"
    "America/Kralendijk" "کرالندیک"
    "America/Nassau" "ناسائو"
    "Africa/Gaborone" "گابورون"
    "America/Blanc-Sablon" "بلان‐سابلون"
    "America/Atikokan" "اتکوکان"
    "America/Creston" "کرستون"
    "Indian/Cocos" "کوکوس"
    "Africa/Kinshasa" "کینشاسا"
    "Africa/Lubumbashi" "لوبومباشی"
    "Africa/Bangui" "بانگی"
    "Africa/Brazzaville" "برازویل"
    "Africa/Douala" "دوآلا"
    "America/Curacao" "کوراسائو"
    "Indian/Christmas" "کریسمس"
    "Europe/Busingen" "بازنگن"
    "Africa/Djibouti" "جیبوتی"
    "Europe/Copenhagen" "کپنهاگ"
    "America/Dominica" "دومینیکا"
    "Africa/Asmara" "اسمره"
    "Africa/Addis_Ababa" "آدیس آبابا"
    "Pacific/Chuuk" "چوک"
    "Pacific/Pohnpei" "پانپی"
    "Africa/Libreville" "لیبرویل"
    "America/Grenada" "گرنادا"
    "Europe/Guernsey" "گرنزی"
    "Africa/Accra" "اکرا"
    "Africa/Banjul" "بانجول"
    "Africa/Conakry" "کوناکری"
    "America/Guadeloupe" "گوادلوپ"
    "Africa/Malabo" "مالابو"
    "Europe/Zagreb" "زاگرب"
    "Europe/Isle_of_Man" "جزیرهٔ من"
    "Atlantic/Reykjavik" "ریکیاویک"
    "Europe/Jersey" "جرزی"
    "Asia/Phnom_Penh" "پنوم‌پن"
    "Indian/Comoro" "کومورو"
    "America/St_Kitts" "سنت کیتس"
    "Asia/Kuwait" "کویت"
    "America/Cayman" "کیمن"
    "Asia/Vientiane" "وینتیان"
    "America/St_Lucia" "سنت لوسیا"
    "Europe/Vaduz" "فادوتس"
    "Africa/Maseru" "ماسرو"
    "Europe/Luxembourg" "لوکزامبورگ"
    "Europe/Monaco" "موناکو"
    "Europe/Podgorica" "پادگاریتسا"
    "America/Marigot" "ماریگات"
    "Indian/Antananarivo" "آنتاناناریوو"
    "Pacific/Majuro" "ماجورو"
    "Europe/Skopje" "اسکوپیه"
    "Africa/Bamako" "باماکو"
    "Pacific/Saipan" "سایپان"
    "Africa/Nouakchott" "نوآکشوت"
    "America/Montserrat" "مونتسرات"
    "Africa/Blantyre" "بلانتیره"
    "Asia/Kuala_Lumpur" "کوالالامپور"
    "Africa/Niamey" "نیامی"
    "Europe/Amsterdam" "آمستردام"
    "Europe/Oslo" "اسلو"
    "Asia/Muscat" "مسقط"
    "Indian/Reunion" "رئونیون"
    "Africa/Kigali" "کیگالی"
    "Indian/Mahe" "ماهه"
    "Europe/Stockholm" "استکهلم"
    "Atlantic/St_Helena" "سنت هلنا"
    "Europe/Ljubljana" "لیوبلیانا"
    "Arctic/Longyearbyen" "لانگ‌یربین"
    "Europe/Bratislava" "براتیسلاوا"
    "Africa/Freetown" "فری‌تاون"
    "Europe/San_Marino" "سان‌مارینو"
    "Africa/Dakar" "داکار"
    "Africa/Mogadishu" "موگادیشو"
    "America/Lower_Princes" "بخش شاهزاده‌‌نشین پایین"
    "Africa/Mbabane" "مبابانه"
    "Indian/Kerguelen" "کرگولن"
    "Africa/Lome" "لومه"
    "America/Port_of_Spain" "پورت‌آواسپین"
    "Pacific/Funafuti" "فونافوتی"
    "Africa/Dar_es_Salaam" "دارالسلام"
    "Africa/Kampala" "کامپالا"
    "Pacific/Midway" "میدوی"
    "Pacific/Wake" "ویک"
    "Europe/Vatican" "واتیکان"
    "America/St_Vincent" "سنت وینسنت"
    "America/Tortola" "تورتولا"
    "America/St_Thomas" "سنت توماس"
    "Pacific/Wallis" "والیس"
    "Asia/Aden" "عدن"
    "Indian/Mayotte" "مایوت"
    "Africa/Lusaka" "لوزاکا"
    "Africa/Harare" "هراره"
};

// `common/main/fr.xml`: 118 of the 418 zones named, 300 inherited.
#[cfg(feature = "localized-exemplar-cities")]
const FR: &str = exemplar_cities! {
    "Europe/Andorra" "Andorre"
    "Asia/Dubai" "Dubaï"
    "Asia/Kabul" "Kaboul"
    "Europe/Tirane" inherited
    "Asia/Yerevan" "Erevan"
    "Antarctica/Casey" inherited
    "Antarctica/Davis" inherited
    "Antarctica/Mawson" inherited
    "Antarctica/Palmer" inherited
    "Antarctica/Rothera" inherited
    "Antarctica/Troll" inherited
    "Antarctica/Vostok" inherited
    "America/Argentina/Buenos_Aires" inherited
    "America/Argentina/Cordoba" inherited
    "America/Argentina/Salta" inherited
    "America/Argentina/Jujuy" inherited
    "America/Argentina/Tucuman" inherited
    "America/Argentina/Catamarca" inherited
    "America/Argentina/La_Rioja" inherited
    "America/Argentina/San_Juan" inherited
    "America/Argentina/Mendoza" inherited
    "America/Argentina/San_Luis" inherited
    "America/Argentina/Rio_Gallegos" inherited
    "America/Argentina/Ushuaia" "Ushuaïa"
    "Pacific/Pago_Pago" inherited
    "Europe/Vienna" "Vienne"
    "Australia/Lord_Howe" inherited
    "Antarctica/Macquarie" inherited
    "Australia/Hobart" inherited
    "Australia/Melbourne" inherited
    "Australia/Sydney" inherited
    "Australia/Broken_Hill" inherited
    "Australia/Brisbane" inherited
    "Australia/Lindeman" inherited
    "Australia/Adelaide" "Adélaïde"
    "Australia/Darwin" inherited
    "Australia/Perth" inherited
    "Australia/Eucla" inherited
    "Asia/Baku" "Bakou"
    "America/Barbados" "La Barbade"
    "Asia/Dhaka" inherited
    "Europe/Brussels" "Bruxelles"
    "Europe/Sofia" inherited
    "Atlantic/Bermuda" "Bermudes"
    "America/La_Paz" inherited
    "America/Noronha" inherited
    "America/Belem" inherited
    "America/Fortaleza" inherited
    "America/Recife" inherited
    "America/Araguaina" inherited
    "America/Maceio" inherited
    "America/Bahia" inherited
    "America/Sao_Paulo" inherited
    "America/Campo_Grande" inherited
    "America/Cuiaba" inherited
    "America/Santarem" inherited
    "America/Porto_Velho" inherited
    "America/Boa_Vista" inherited
    "America/Manaus" "Manaos"
    "America/Eirunepe" inherited
    "America/Rio_Branco" inherited
    "Asia/Thimphu" inherited
    "Europe/Minsk" inherited
    "America/Belize" inherited
    "America/St_Johns" "Saint-Jean de Terre-Neuve"
    "America/Halifax" inherited
    "America/Glace_Bay" inherited
    "America/Moncton" inherited
    "America/Goose_Bay" inherited
    "America/Toronto" inherited
    "America/Iqaluit" inherited
    "America/Winnipeg" inherited
    "America/Resolute" inherited
    "America/Rankin_Inlet" inherited
    "America/Regina" inherited
    "America/Swift_Current" inherited
    "America/Edmonton" inherited
    "America/Cambridge_Bay" inherited
    "America/Inuvik" inherited
    "America/Vancouver" inherited
    "America/Dawson_Creek" inherited
    "America/Fort_Nelson" inherited
    "America/Whitehorse" inherited
    "America/Dawson" inherited
    "Europe/Zurich" inherited
    "Africa/Abidjan" inherited
    "Pacific/Rarotonga" inherited
    "America/Santiago" inherited
    "America/Coyhaique" inherited
    "America/Punta_Arenas" inherited
    "Pacific/Easter" "Île de Pâques"
    "Asia/Shanghai" inherited
    "Asia/Urumqi" inherited
    "America/Bogota" inherited
    "America/Costa_Rica" inherited
    "America/Havana" "La Havane"
    "Atlantic/Cape_Verde" "Cap-Vert"
    "Asia/Nicosia" "Nicosie"
    "Asia/Famagusta" "Famagouste"
    "Europe/Prague" inherited
    "Europe/Berlin" inherited
    "America/Santo_Domingo" "Saint-Domingue"
    "Africa/Algiers" "Alger"
    "America/Guayaquil" inherited
    "Pacific/Galapagos" inherited
    "Europe/Tallinn" inherited
    "Africa/Cairo" "Le Caire"
    "Africa/El_Aaiun" "Laâyoune"
    "Europe/Madrid" inherited
    "Africa/Ceuta" inherited
    "Atlantic/Canary" "Îles Canaries"
    "Europe/Helsinki" inherited
    "Pacific/Fiji" "Fidji"
    "Atlantic/Stanley" inherited
    "Pacific/Kosrae" inherited
    "Atlantic/Faroe" "Îles Féroé"
    "Europe/Paris" inherited
    "Europe/London" "Londres"
    "Asia/Tbilisi" "Tbilissi"
    "America/Cayenne" inherited
    "Europe/Gibraltar" inherited
    "America/Nuuk" inherited
    "America/Danmarkshavn" inherited
    "America/Scoresbysund" inherited
    "America/Thule" "Thulé"
    "Europe/Athens" "Athènes"
    "Atlantic/South_Georgia" "Géorgie du Sud"
    "America/Guatemala" inherited
    "Pacific/Guam" inherited
    "Africa/Bissau" inherited
    "America/Guyana" inherited
    "Asia/Hong_Kong" inherited
    "America/Tegucigalpa" inherited
    "America/Port-au-Prince" inherited
    "Europe/Budapest" inherited
    "Asia/Jakarta" inherited
    "Asia/Pontianak" inherited
    "Asia/Makassar" "Macassar"
    "Asia/Jayapura" inherited
    "Europe/Dublin" inherited
    "Asia/Jerusalem" "Jérusalem"
    "Asia/Kolkata" "Calcutta"
    "Indian/Chagos" inherited
    "Asia/Baghdad" "Bagdad"
    "Asia/Tehran" "Téhéran"
    "Europe/Rome" inherited
    "America/Jamaica" "Jamaïque"
    "Asia/Amman" inherited
    "Asia/Tokyo" inherited
    "Africa/Nairobi" inherited
    "Asia/Bishkek" "Bichkek"
    "Pacific/Tarawa" inherited
    "Pacific/Kanton" inherited
    "Pacific/Kiritimati" inherited
    "Asia/Pyongyang" inherited
    "Asia/Seoul" "Séoul"
    "Asia/Almaty" "Alma Ata"
    "Asia/Qyzylorda" "Kzyl Orda"
    "Asia/Qostanay" "Kostanaï"
    "Asia/Aqtobe" "Aktioubinsk"
    "Asia/Aqtau" "Aktaou"
    "Asia/Atyrau" "Atyraou"
    "Asia/Oral" "Ouralsk"
    "Asia/Beirut" "Beyrouth"
    "Asia/Colombo" inherited
    "Africa/Monrovia" inherited
    "Europe/Vilnius" inherited
    "Europe/Riga" inherited
    "Africa/Tripoli" "Tripoli (Libye)"
    "Africa/Casablanca" inherited
    "Europe/Chisinau" inherited
    "Pacific/Kwajalein" inherited
    "Asia/Yangon" "Rangoun"
    "Asia/Ulaanbaatar" "Oulan-Bator"
    "Asia/Hovd" inherited
    "Asia/Macau" inherited
    "America/Martinique" inherited
    "Europe/Malta" "Malte"
    "Indian/Mauritius" "Maurice"
    "Indian/Maldives" inherited
    "America/Mexico_City" "Mexico"
    "America/Cancun" inherited
    "America/Merida" inherited
    "America/Monterrey" inherited
    "America/Matamoros" inherited
    "America/Chihuahua" inherited
    "America/Ciudad_Juarez" inherited
    "America/Ojinaga" inherited
    "America/Mazatlan" inherited
    "America/Bahia_Banderas" "Bahia de Banderas"
    "America/Hermosillo" inherited
    "America/Tijuana" inherited
    "Asia/Kuching" inherited
    "Africa/Maputo" inherited
    "Africa/Windhoek" inherited
    "Pacific/Noumea" inherited
    "Pacific/Norfolk" inherited
    "Africa/Lagos" inherited
    "America/Managua" inherited
    "Asia/Kathmandu" "Katmandou"
    "Pacific/Nauru" inherited
    "Pacific/Niue" inherited
    "Pacific/Auckland" inherited
    "Pacific/Chatham" inherited
    "America/Panama" inherited
    "America/Lima" inherited
    "Pacific/Tahiti" inherited
    "Pacific/Marquesas" "Marquises"
    "Pacific/Gambier" inherited
    "Pacific/Port_Moresby" inherited
    "Pacific/Bougainville" inherited
    "Asia/Manila" "Manille"
    "Asia/Karachi" inherited
    "Europe/Warsaw" "Varsovie"
    "America/Miquelon" inherited
    "Pacific/Pitcairn" inherited
    "America/Puerto_Rico" "Porto Rico"
    "Asia/Gaza" inherited
    "Asia/Hebron" "Hébron"
    "Europe/Lisbon" "Lisbonne"
    "Atlantic/Madeira" "Madère"
    "Atlantic/Azores" "Açores"
    "Pacific/Palau" "Palaos"
    "America/Asuncion" inherited
    "Asia/Qatar" inherited
    "Europe/Bucharest" "Bucarest"
    "Europe/Belgrade" inherited
    "Europe/Kaliningrad" inherited
    "Europe/Moscow" "Moscou"
    "Europe/Simferopol" inherited
    "Europe/Kirov" inherited
    "Europe/Volgograd" inherited
    "Europe/Astrakhan" inherited
    "Europe/Saratov" inherited
    "Europe/Ulyanovsk" "Oulianovsk"
    "Europe/Samara" inherited
    "Asia/Yekaterinburg" "Ekaterinbourg"
    "Asia/Omsk" inherited
    "Asia/Novosibirsk" "Novossibirsk"
    "Asia/Barnaul" inherited
    "Asia/Tomsk" inherited
    "Asia/Novokuznetsk" inherited
    "Asia/Krasnoyarsk" "Krasnoïarsk"
    "Asia/Irkutsk" "Irkoutsk"
    "Asia/Chita" "Tchita"
    "Asia/Yakutsk" "Iakoutsk"
    "Asia/Khandyga" inherited
    "Asia/Vladivostok" inherited
    "Asia/Ust-Nera" inherited
    "Asia/Magadan" inherited
    "Asia/Sakhalin" "Sakhaline"
    "Asia/Srednekolymsk" inherited
    "Asia/Kamchatka" "Kamtchatka"
    "Asia/Anadyr" inherited
    "Asia/Riyadh" "Riyad"
    "Pacific/Guadalcanal" inherited
    "Africa/Khartoum" inherited
    "Asia/Singapore" "Singapour"
    "America/Paramaribo" inherited
    "Africa/Juba" inherited
    "Africa/Sao_Tome" inherited
    "America/El_Salvador" inherited
    "Asia/Damascus" "Damas"
    "America/Grand_Turk" inherited
    "Africa/Ndjamena" inherited
    "Asia/Bangkok" inherited
    "Asia/Dushanbe" "Douchanbé"
    "Pacific/Fakaofo" inherited
    "Asia/Dili" inherited
    "Asia/Ashgabat" "Achgabat"
    "Africa/Tunis" inherited
    "Pacific/Tongatapu" inherited
    "Europe/Istanbul" inherited
    "Asia/Taipei" inherited
    "Europe/Kyiv" "Kiev"
    "America/New_York" inherited
    "America/Detroit" "Détroit"
    "America/Kentucky/Louisville" inherited
    "America/Kentucky/Monticello" "Monticello [Kentucky]"
    "America/Indiana/Indianapolis" inherited
    "America/Indiana/Vincennes" "Vincennes [Indiana]"
    "America/Indiana/Winamac" "Winamac [Indiana]"
    "America/Indiana/Marengo" "Marengo [Indiana]"
    "America/Indiana/Petersburg" "Petersburg [Indiana]"
    "America/Indiana/Vevay" "Vevay [Indiana]"
    "America/Chicago" inherited
    "America/Indiana/Tell_City" "Tell City [Indiana]"
    "America/Indiana/Knox" "Knox [Indiana]"
    "America/Menominee" inherited
    "America/North_Dakota/Center" "Center (Dakota du Nord)"
    "America/North_Dakota/New_Salem" "New Salem (Dakota du Nord)"
    "America/North_Dakota/Beulah" "Beulah (Dakota du Nord)"
    "America/Denver" inherited
    "America/Boise" inherited
    "America/Phoenix" inherited
    "America/Los_Angeles" inherited
    "America/Anchorage" inherited
    "America/Juneau" inherited
    "America/Sitka" inherited
    "America/Metlakatla" inherited
    "America/Yakutat" inherited
    "America/Nome" inherited
    "America/Adak" inherited
    "Pacific/Honolulu" "Honolulu"
    "America/Montevideo" inherited
    "Asia/Samarkand" "Samarcande"
    "Asia/Tashkent" "Tachkent"
    "America/Caracas" inherited
    "Asia/Ho_Chi_Minh" "Hô-Chi-Minh-Ville"
    "Pacific/Efate" "Éfaté"
    "Pacific/Apia" inherited
    "Africa/Johannesburg" inherited
    "America/Antigua" inherited
    "America/Anguilla" inherited
    "Africa/Luanda" inherited
    "Antarctica/McMurdo" inherited
    "Antarctica/DumontDUrville" inherited
    "Antarctica/Syowa" inherited
    "America/Aruba" inherited
    "Europe/Mariehamn" inherited
    "Europe/Sarajevo" inherited
    "Africa/Ouagadougou" inherited
    "Asia/Bahrain" "Bahreïn"
    "Africa/Bujumbura" inherited
    "Africa/Porto-Novo" inherited
    "America/St_Barthelemy" "Saint-Barthélemy"
    "Asia/Brunei" inherited
    "America/Kralendijk" inherited
    "America/Nassau" inherited
    "Africa/Gaborone" inherited
    "America/Blanc-Sablon" inherited
    "America/Atikokan" inherited
    "America/Creston" inherited
    "Indian/Cocos" inherited
    "Africa/Kinshasa" inherited
    "Africa/Lubumbashi" inherited
    "Africa/Bangui" inherited
    "Africa/Brazzaville" inherited
    "Africa/Douala" inherited
    "America/Curacao" inherited
    "Indian/Christmas" inherited
    "Europe/Busingen" inherited
    "Africa/Djibouti" inherited
    "Europe/Copenhagen" "Copenhague"
    "America/Dominica" "Dominique"
    "Africa/Asmara" inherited
    "Africa/Addis_Ababa" "Addis-Abeba"
    "Pacific/Chuuk" inherited
    "Pacific/Pohnpei" inherited
    "Africa/Libreville" inherited
    "America/Grenada" "Grenade"
    "Europe/Guernsey" "Guernesey"
    "Africa/Accra" inherited
    "Africa/Banjul" inherited
    "Africa/Conakry" inherited
    "America/Guadeloupe" inherited
    "Africa/Malabo" inherited
    "Europe/Zagreb" inherited
    "Europe/Isle_of_Man" "Île de Man"
    "Atlantic/Reykjavik" inherited
    "Europe/Jersey" inherited
    "Asia/Phnom_Penh" inherited
    "Indian/Comoro" inherited
    "America/St_Kitts" "Saint-Christophe"
    "Asia/Kuwait" "Koweït"
    "America/Cayman" "Caïmans"
    "Asia/Vientiane" inherited
    "America/St_Lucia" "Sainte-Lucie"
    "Europe/Vaduz" inherited
    "Africa/Maseru" inherited
    "Europe/Luxembourg" inherited
    "Europe/Monaco" inherited
    "Europe/Podgorica" inherited
    "America/Marigot" inherited
    "Indian/Antananarivo" inherited
    "Pacific/Majuro" inherited
    "Europe/Skopje" inherited
    "Africa/Bamako" inherited
    "Pacific/Saipan" inherited
    "Africa/Nouakchott" inherited
    "America/Montserrat" inherited
    "Africa/Blantyre" inherited
    "Asia/Kuala_Lumpur" inherited
    "Africa/Niamey" inherited
    "Europe/Amsterdam" inherited
    "Europe/Oslo" inherited
    "Asia/Muscat" "Mascate"
    "Indian/Reunion" "La Réunion"
    "Africa/Kigali" inherited
    "Indian/Mahe" inherited
    "Europe/Stockholm" inherited
    "Atlantic/St_Helena" "Sainte-Hélène"
    "Europe/Ljubljana" inherited
    "Arctic/Longyearbyen" inherited
    "Europe/Bratislava" inherited
    "Africa/Freetown" inherited
    "Europe/San_Marino" "Saint-Marin"
    "Africa/Dakar" inherited
    "Africa/Mogadishu" "Mogadiscio"
    "America/Lower_Princes" inherited
    "Africa/Mbabane" inherited
    "Indian/Kerguelen" inherited
    "Africa/Lome" inherited
    "America/Port_of_Spain" "Port-d’Espagne"
    "Pacific/Funafuti" inherited
    "Africa/Dar_es_Salaam" inherited
    "Africa/Kampala" inherited
    "Pacific/Midway" inherited
    "Pacific/Wake" inherited
    "Europe/Vatican" "Le Vatican"
    "America/St_Vincent" "Saint-Vincent"
    "America/Tortola" inherited
    "America/St_Thomas" "Saint-Thomas"
    "Pacific/Wallis" inherited
    "Asia/Aden" inherited
    "Indian/Mayotte" inherited
    "Africa/Lusaka" inherited
    "Africa/Harare" inherited
};

// `common/main/he.xml`: 418 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const HE: &str = exemplar_cities! {
    "Europe/Andorra" "אנדורה"
    "Asia/Dubai" "דובאי"
    "Asia/Kabul" "קאבול"
    "Europe/Tirane" "טירנה"
    "Asia/Yerevan" "ירוואן"
    "Antarctica/Casey" "קייסי"
    "Antarctica/Davis" "דיוויס"
    "Antarctica/Mawson" "מוסון"
    "Antarctica/Palmer" "פאלמר"
    "Antarctica/Rothera" "רות׳רה"
    "Antarctica/Troll" "טרול"
    "Antarctica/Vostok" "ווסטוק"
    "America/Argentina/Buenos_Aires" "בואנוס איירס"
    "America/Argentina/Cordoba" "קורדובה"
    "America/Argentina/Salta" "סלטה"
    "America/Argentina/Jujuy" "חוחוי"
    "America/Argentina/Tucuman" "טוקומן"
    "America/Argentina/Catamarca" "קטמרקה"
    "America/Argentina/La_Rioja" "לה ריוחה"
    "America/Argentina/San_Juan" "סן חואן"
    "America/Argentina/Mendoza" "מנדוזה"
    "America/Argentina/San_Luis" "סן לואיס"
    "America/Argentina/Rio_Gallegos" "ריו גאייגוס"
    "America/Argentina/Ushuaia" "אושוואיה"
    "Pacific/Pago_Pago" "פאגו פאגו"
    "Europe/Vienna" "וינה"
    "Australia/Lord_Howe" "אי הלורד האו"
    "Antarctica/Macquarie" "מקווארי"
    "Australia/Hobart" "הוברט"
    "Australia/Melbourne" "מלבורן"
    "Australia/Sydney" "סידני"
    "Australia/Broken_Hill" "ברוקן היל"
    "Australia/Brisbane" "בריסביין"
    "Australia/Lindeman" "לינדמן"
    "Australia/Adelaide" "אדלייד"
    "Australia/Darwin" "דרווין"
    "Australia/Perth" "פרת׳"
    "Australia/Eucla" "יוקלה"
    "Asia/Baku" "באקו"
    "America/Barbados" "ברבדוס"
    "Asia/Dhaka" "דאקה"
    "Europe/Brussels" "בריסל"
    "Europe/Sofia" "סופיה"
    "Atlantic/Bermuda" "ברמודה"
    "America/La_Paz" "לה פאס"
    "America/Noronha" "נורוניה"
    "America/Belem" "בלם"
    "America/Fortaleza" "פורטאלזה"
    "America/Recife" "רסיפה"
    "America/Araguaina" "אראגואינה"
    "America/Maceio" "מסייאו"
    "America/Bahia" "באהיה"
    "America/Sao_Paulo" "סאו פאולו"
    "America/Campo_Grande" "קמפו גרנדה"
    "America/Cuiaba" "קויאבה"
    "America/Santarem" "סנטרם"
    "America/Porto_Velho" "פורטו וליו"
    "America/Boa_Vista" "בואה ויסטה"
    "America/Manaus" "מנאוס"
    "America/Eirunepe" "אירונפי"
    "America/Rio_Branco" "ריו ברנקו"
    "Asia/Thimphu" "טהימפהו"
    "Europe/Minsk" "מינסק"
    "America/Belize" "בליז"
    "America/St_Johns" "סנט ג׳ונס"
    "America/Halifax" "הליפקס"
    "America/Glace_Bay" "גלייס ביי"
    "America/Moncton" "מונקטון"
    "America/Goose_Bay" "גוס ביי"
    "America/Toronto" "טורונטו"
    "America/Iqaluit" "איקלואיט"
    "America/Winnipeg" "וויניפג"
    "America/Resolute" "רזולוט"
    "America/Rankin_Inlet" "רנקין אינלט"
    "America/Regina" "רג׳ינה"
    "America/Swift_Current" "סוויפט קרנט"
    "America/Edmonton" "אדמונטון"
    "America/Cambridge_Bay" "קיימברידג׳ ביי"
    "America/Inuvik" "אינוויק"
    "America/Vancouver" "ונקובר"
    "America/Dawson_Creek" "דוסון קריק"
    "America/Fort_Nelson" "פורט נלסון"
    "America/Whitehorse" "ווייטהורס"
    "America/Dawson" "דוסון"
    "Europe/Zurich" "ציריך"
    "Africa/Abidjan" "אביג׳אן"
    "Pacific/Rarotonga" "רארוטונגה"
    "America/Santiago" "סנטיאגו"
    "America/Coyhaique" "קויאיקה"
    "America/Punta_Arenas" "פונטה ארנס"
    "Pacific/Easter" "אי הפסחא"
    "Asia/Shanghai" "שנחאי"
    "Asia/Urumqi" "אורומקי"
    "America/Bogota" "בוגוטה"
    "America/Costa_Rica" "קוסטה ריקה"
    "America/Havana" "הוואנה"
    "Atlantic/Cape_Verde" "כף ורדה"
    "Asia/Nicosia" "ניקוסיה"
    "Asia/Famagusta" "פמגוסטה"
    "Europe/Prague" "פראג"
    "Europe/Berlin" "ברלין"
    "America/Santo_Domingo" "סנטו דומינגו"
    "Africa/Algiers" "אלג׳יר"
    "America/Guayaquil" "גואיאקיל"
    "Pacific/Galapagos" "גלפאגוס"
    "Europe/Tallinn" "טאלין"
    "Africa/Cairo" "קהיר"
    "Africa/El_Aaiun" "אל עיון"
    "Europe/Madrid" "מדריד"
    "Africa/Ceuta" "סאוטה"
    "Atlantic/Canary" "האיים הקנריים"
    "Europe/Helsinki" "הלסינקי"
    "Pacific/Fiji" "פיג׳י"
    "Atlantic/Stanley" "סטנלי"
    "Pacific/Kosrae" "קוסרה"
    "Atlantic/Faroe" "פארו"
    "Europe/Paris" "פריז"
    "Europe/London" "לונדון"
    "Asia/Tbilisi" "טביליסי"
    "America/Cayenne" "קאיין"
    "Europe/Gibraltar" "גיברלטר"
    "America/Nuuk" "נואוק"
    "America/Danmarkshavn" "דנמרקסהוון"
    "America/Scoresbysund" "סקורסביסונד"
    "America/Thule" "תולה"
    "Europe/Athens" "אתונה"
    "Atlantic/South_Georgia" "דרום ג׳ורג׳יה"
    "America/Guatemala" "גואטמלה"
    "Pacific/Guam" "גואם"
    "Africa/Bissau" "ביסאו"
    "America/Guyana" "גיאנה"
    "Asia/Hong_Kong" "הונג קונג"
    "America/Tegucigalpa" "טגוסיגלפה"
    "America/Port-au-Prince" "פורט או פראנס"
    "Europe/Budapest" "בודפשט"
    "Asia/Jakarta" "ג׳קרטה"
    "Asia/Pontianak" "פונטיאנק"
    "Asia/Makassar" "מאקאסאר"
    "Asia/Jayapura" "ג׳איאפורה"
    "Europe/Dublin" "דבלין"
    "Asia/Jerusalem" "ירושלים"
    "Asia/Kolkata" "קולקטה"
    "Indian/Chagos" "צ׳אגוס"
    "Asia/Baghdad" "בגדד"
    "Asia/Tehran" "טהרן"
    "Europe/Rome" "רומא"
    "America/Jamaica" "ג׳מייקה"
    "Asia/Amman" "עמאן"
    "Asia/Tokyo" "טוקיו"
    "Africa/Nairobi" "ניירובי"
    "Asia/Bishkek" "בישקק"
    "Pacific/Tarawa" "טאראווה"
    "Pacific/Kanton" "קנטון"
    "Pacific/Kiritimati" "קיריטימאטי"
    "Asia/Pyongyang" "פיונגיאנג"
    "Asia/Seoul" "סיאול"
    "Asia/Almaty" "אלמאטי"
    "Asia/Qyzylorda" "קיזילורדה"
    "Asia/Qostanay" "קוסטנאי"
    "Asia/Aqtobe" "אקטובה"
    "Asia/Aqtau" "אקטאו"
    "Asia/Atyrau" "אטיראו"
    "Asia/Oral" "אורל"
    "Asia/Beirut" "ביירות"
    "Asia/Colombo" "קולומבו"
    "Africa/Monrovia" "מונרוביה"
    "Europe/Vilnius" "וילנה"
    "Europe/Riga" "ריגה"
    "Africa/Tripoli" "טריפולי"
    "Africa/Casablanca" "קזבלנקה"
    "Europe/Chisinau" "קישינב"
    "Pacific/Kwajalein" "קוואג׳ליין"
    "Asia/Yangon" "רנגון"
    "Asia/Ulaanbaatar" "אולאן באטור"
    "Asia/Hovd" "חובד"
    "Asia/Macau" "מקאו"
    "America/Martinique" "מרטיניק"
    "Europe/Malta" "מלטה"
    "Indian/Mauritius" "מאוריציוס"
    "Indian/Maldives" "האיים המלדיביים"
    "America/Mexico_City" "מקסיקו סיטי"
    "America/Cancun" "קנקון"
    "America/Merida" "מרידה"
    "America/Monterrey" "מונטריי"
    "America/Matamoros" "מטמורוס"
    "America/Chihuahua" "צ׳יוואווה"
    "America/Ciudad_Juarez" "סיודד חוארס"
    "America/Ojinaga" "אוג׳ינאגה"
    "America/Mazatlan" "מזטלן"
    "America/Bahia_Banderas" "באהיה בנדרס"
    "America/Hermosillo" "הרמוסיו"
    "America/Tijuana" "טיחואנה"
    "Asia/Kuching" "קוצ׳ינג"
    "Africa/Maputo" "מאפוטו"
    "Africa/Windhoek" "וינדהוק"
    "Pacific/Noumea" "נומאה"
    "Pacific/Norfolk" "נורפוק"
    "Africa/Lagos" "לאגוס"
    "America/Managua" "מנגואה"
    "Asia/Kathmandu" "קטמנדו"
    "Pacific/Nauru" "נאורו"
    "Pacific/Niue" "ניואה"
    "Pacific/Auckland" "אוקלנד"
    "Pacific/Chatham" "צ׳אטהאם"
    "America/Panama" "פנמה"
    "America/Lima" "לימה"
    "Pacific/Tahiti" "טהיטי"
    "Pacific/Marquesas" "איי מרקיז"
    "Pacific/Gambier" "איי גמבייה"
    "Pacific/Port_Moresby" "פורט מורסבי"
    "Pacific/Bougainville" "בוגנוויל"
    "Asia/Manila" "מנילה"
    "Asia/Karachi" "קראצ׳י"
    "Europe/Warsaw" "ורשה"
    "America/Miquelon" "מיקלון"
    "Pacific/Pitcairn" "פיטקרן"
    "America/Puerto_Rico" "פוארטו ריקו"
    "Asia/Gaza" "עזה"
    "Asia/Hebron" "חברון"
    "Europe/Lisbon" "ליסבון"
    "Atlantic/Madeira" "מדיירה"
    "Atlantic/Azores" "האיים האזוריים"
    "Pacific/Palau" "פלאו"
    "America/Asuncion" "אסונסיון"
    "Asia/Qatar" "קטאר"
    "Europe/Bucharest" "בוקרשט"
    "Europe/Belgrade" "בלגרד"
    "Europe/Kaliningrad" "קלינינגרד"
    "Europe/Moscow" "מוסקבה"
    "Europe/Simferopol" "סימפרופול"
    "Europe/Kirov" "קירוב"
    "Europe/Volgograd" "וולגוגרד"
    "Europe/Astrakhan" "אסטרחן"
    "Europe/Saratov" "סראטוב"
    "Europe/Ulyanovsk" "אוליאנובסק"
    "Europe/Samara" "סמרה"
    "Asia/Yekaterinburg" "יקטרינבורג"
    "Asia/Omsk" "אומסק"
    "Asia/Novosibirsk" "נובוסיבירסק"
    "Asia/Barnaul" "ברנאול"
    "Asia/Tomsk" "טומסק"
    "Asia/Novokuznetsk" "נובוקוזנטסק"
    "Asia/Krasnoyarsk" "קרסנויארסק"
    "Asia/Irkutsk" "אירקוטסק"
    "Asia/Chita" "צ׳יטה"
    "Asia/Yakutsk" "יקוטסק"
    "Asia/Khandyga" "חנדיגה"
    "Asia/Vladivostok" "ולדיווסטוק"
    "Asia/Ust-Nera" "אוסט-נרה"
    "Asia/Magadan" "מגדן"
    "Asia/Sakhalin" "סחלין"
    "Asia/Srednekolymsk" "סרדנייקולימסק"
    "Asia/Kamchatka" "קמצ׳טקה"
    "Asia/Anadyr" "אנדיר"
    "Asia/Riyadh" "ריאד"
    "Pacific/Guadalcanal" "גוודלקנאל"
    "Africa/Khartoum" "חרטום"
    "Asia/Singapore" "סינגפור"
    "America/Paramaribo" "פרמריבו"
    "Africa/Juba" "ג׳ובה"
    "Africa/Sao_Tome" "סאו טומה"
    "America/El_Salvador" "אל סלבדור"
    "Asia/Damascus" "דמשק"
    "America/Grand_Turk" "גרנד טורק"
    "Africa/Ndjamena" "נג׳מנה"
    "Asia/Bangkok" "בנגקוק"
    "Asia/Dushanbe" "דושנבה"
    "Pacific/Fakaofo" "פקאופו"
    "Asia/Dili" "דילי"
    "Asia/Ashgabat" "אשגבט"
    "Africa/Tunis" "תוניס"
    "Pacific/Tongatapu" "טונגטאפו"
    "Europe/Istanbul" "איסטנבול"
    "Asia/Taipei" "טאיפיי"
    "Europe/Kyiv" "קייב"
    "America/New_York" "ניו יורק"
    "America/Detroit" "דטרויט"
    "America/Kentucky/Louisville" "לואיוויל"
    "America/Kentucky/Monticello" "מונטיצ׳לו, קנטאקי"
    "America/Indiana/Indianapolis" "אינדיאנפוליס"
    "America/Indiana/Vincennes" "וינסנס, אינדיאנה"
    "America/Indiana/Winamac" "וינמאק, אינדיאנה"
    "America/Indiana/Marengo" "מרנגו, אינדיאנה"
    "America/Indiana/Petersburg" "פיטרסבורג, אינדיאנה"
    "America/Indiana/Vevay" "ויוואיי, אינדיאנה"
    "America/Chicago" "שיקגו"
    "America/Indiana/Tell_City" "טל סיטי, אינדיאנה"
    "America/Indiana/Knox" "נוקס, אינדיאנה"
    "America/Menominee" "מנומיני"
    "America/North_Dakota/Center" "סנטר, דקוטה הצפונית"
    "America/North_Dakota/New_Salem" "ניו סיילם, דקוטה הצפונית"
    "America/North_Dakota/Beulah" "ביולה, דקוטה הצפונית"
    "America/Denver" "דנוור"
    "America/Boise" "בויסי"
    "America/Phoenix" "פיניקס"
    "America/Los_Angeles" "לוס אנג׳לס"
    "America/Anchorage" "אנקורג׳"
    "America/Juneau" "ג׳ונו"
    "America/Sitka" "סיטקה"
    "America/Metlakatla" "מטלקטלה"
    "America/Yakutat" "יקוטאט"
    "America/Nome" "נום"
    "America/Adak" "אדאק"
    "Pacific/Honolulu" "הונולולו"
    "America/Montevideo" "מונטווידאו"
    "Asia/Samarkand" "סמרקנד"
    "Asia/Tashkent" "טשקנט"
    "America/Caracas" "קראקס"
    "Asia/Ho_Chi_Minh" "הו צ׳י מין סיטי"
    "Pacific/Efate" "אפטה"
    "Pacific/Apia" "אפיה"
    "Africa/Johannesburg" "יוהנסבורג"
    "America/Antigua" "אנטיגואה"
    "America/Anguilla" "אנגווילה"
    "Africa/Luanda" "לואנדה"
    "Antarctica/McMurdo" "מק-מרדו"
    "Antarctica/DumontDUrville" "דומון ד׳אורוויל"
    "Antarctica/Syowa" "סייווה"
    "America/Aruba" "ארובה"
    "Europe/Mariehamn" "מרייהאמן"
    "Europe/Sarajevo" "סרייבו"
    "Africa/Ouagadougou" "וואגאדוגו"
    "Asia/Bahrain" "בחריין"
    "Africa/Bujumbura" "בוג׳ומבורה"
    "Africa/Porto-Novo" "פורטו נובו"
    "America/St_Barthelemy" "סנט ברתלמי"
    "Asia/Brunei" "ברוניי"
    "America/Kralendijk" "קרלנדייק"
    "America/Nassau" "נסאו"
    "Africa/Gaborone" "גבורונה"
    "America/Blanc-Sablon" "בלאן-סבלון"
    "America/Atikokan" "אטיקוקן"
    "America/Creston" "קרסטון"
    "Indian/Cocos" "קוקוס"
    "Africa/Kinshasa" "קינשסה"
    "Africa/Lubumbashi" "לובומבאשי"
    "Africa/Bangui" "בנגואי"
    "Africa/Brazzaville" "ברזוויל"
    "Africa/Douala" "דואלה"
    "America/Curacao" "קוראסאו"
    "Indian/Christmas" "האי כריסטמס"
    "Europe/Busingen" "ביזינגן"
    "Africa/Djibouti" "ג׳יבוטי"
    "Europe/Copenhagen" "קופנהגן"
    "America/Dominica" "דומיניקה"
    "Africa/Asmara" "אסמרה"
    "Africa/Addis_Ababa" "אדיס אבבה"
    "Pacific/Chuuk" "צ׳וק"
    "Pacific/Pohnpei" "פונפיי"
    "Africa/Libreville" "ליברוויל"
    "America/Grenada" "גרנדה"
    "Europe/Guernsey" "גרנזי"
    "Africa/Accra" "אקרה"
    "Africa/Banjul" "בנג׳ול"
    "Africa/Conakry" "קונאקרי"
    "America/Guadeloupe" "גואדלופ"
    "Africa/Malabo" "מלבו"
    "Europe/Zagreb" "זאגרב"
    "Europe/Isle_of_Man" "האי מאן"
    "Atlantic/Reykjavik" "רייקיאוויק"
    "Europe/Jersey" "ג׳רזי"
    "Asia/Phnom_Penh" "פנום פן"
    "Indian/Comoro" "קומורו"
    "America/St_Kitts" "סנט קיטס"
    "Asia/Kuwait" "כווית"
    "America/Cayman" "קיימן"
    "Asia/Vientiane" "ויינטיאן"
    "America/St_Lucia" "סנט לוסיה"
    "Europe/Vaduz" "ואדוץ"
    "Africa/Maseru" "מסרו"
    "Europe/Luxembourg" "לוקסמבורג"
    "Europe/Monaco" "מונקו"
    "Europe/Podgorica" "פודגוריצה"
    "America/Marigot" "מריגו"
    "Indian/Antananarivo" "אנטננריבו"
    "Pacific/Majuro" "מאג׳ורו"
    "Europe/Skopje" "סקופיה"
    "Africa/Bamako" "במאקו"
    "Pacific/Saipan" "סאיפאן"
    "Africa/Nouakchott" "נואקצ׳וט"
    "America/Montserrat" "מונסראט"
    "Africa/Blantyre" "בלנטיר"
    "Asia/Kuala_Lumpur" "קואלה לומפור"
    "Africa/Niamey" "ניאמיי"
    "Europe/Amsterdam" "אמסטרדם"
    "Europe/Oslo" "אוסלו"
    "Asia/Muscat" "מוסקט"
    "Indian/Reunion" "ראוניון"
    "Africa/Kigali" "קיגלי"
    "Indian/Mahe" "מהא"
    "Europe/Stockholm" "שטוקהולם"
    "Atlantic/St_Helena" "סנט הלנה"
    "Europe/Ljubljana" "לובליאנה"
    "Arctic/Longyearbyen" "לונגיירבין"
    "Europe/Bratislava" "ברטיסלבה"
    "Africa/Freetown" "פריטאון"
    "Europe/San_Marino" "סן מרינו"
    "Africa/Dakar" "דקאר"
    "Africa/Mogadishu" "מוגדישו"
    "America/Lower_Princes" "לואוור פרינסס קוורטר"
    "Africa/Mbabane" "מבבנה"
    "Indian/Kerguelen" "קרגוולן"
    "Africa/Lome" "לומה"
    "America/Port_of_Spain" "פורט אוף ספיין"
    "Pacific/Funafuti" "פונפוטי"
    "Africa/Dar_es_Salaam" "דאר א-סלאם"
    "Africa/Kampala" "קמפאלה"
    "Pacific/Midway" "מידוויי"
    "Pacific/Wake" "וייק"
    "Europe/Vatican" "הוותיקן"
    "America/St_Vincent" "סנט וינסנט"
    "America/Tortola" "טורטולה"
    "America/St_Thomas" "סנט תומאס"
    "Pacific/Wallis" "ווליס"
    "Asia/Aden" "עדן"
    "Indian/Mayotte" "מאיוט"
    "Africa/Lusaka" "לוסקה"
    "Africa/Harare" "הרארה"
};

// `common/main/hi.xml`: 418 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const HI: &str = exemplar_cities! {
    "Europe/Andorra" "अंडोरा"
    "Asia/Dubai" "दुबई"
    "Asia/Kabul" "काबुल"
    "Europe/Tirane" "टाइरेन"
    "Asia/Yerevan" "येरेवान"
    "Antarctica/Casey" "केसी"
    "Antarctica/Davis" "डेविस"
    "Antarctica/Mawson" "मॉसन"
    "Antarctica/Palmer" "पॉमर"
    "Antarctica/Rothera" "रोथेरा"
    "Antarctica/Troll" "ट्रोल"
    "Antarctica/Vostok" "वोस्तोक"
    "America/Argentina/Buenos_Aires" "ब्यूनस आयरस"
    "America/Argentina/Cordoba" "कोर्डोबा"
    "America/Argentina/Salta" "साल्टा"
    "America/Argentina/Jujuy" "जुजोए"
    "America/Argentina/Tucuman" "टोकूमन"
    "America/Argentina/Catamarca" "काटामार्का"
    "America/Argentina/La_Rioja" "ला रिओजा"
    "America/Argentina/San_Juan" "सैन ह्वान"
    "America/Argentina/Mendoza" "मेंडोज़ा"
    "America/Argentina/San_Luis" "सैन लूई"
    "America/Argentina/Rio_Gallegos" "रियो गालेगोस"
    "America/Argentina/Ushuaia" "उशुआइया"
    "Pacific/Pago_Pago" "पागो पागो"
    "Europe/Vienna" "विएना"
    "Australia/Lord_Howe" "लॉर्ड होवे"
    "Antarctica/Macquarie" "मक्वारी"
    "Australia/Hobart" "होबार्ट"
    "Australia/Melbourne" "मेलबोर्न"
    "Australia/Sydney" "सिडनी"
    "Australia/Broken_Hill" "ब्रोकन हिल"
    "Australia/Brisbane" "ब्रिस्बन"
    "Australia/Lindeman" "लिंडेमान"
    "Australia/Adelaide" "एडिलेड"
    "Australia/Darwin" "डार्विन"
    "Australia/Perth" "पर्थ"
    "Australia/Eucla" "यूक्ला"
    "Asia/Baku" "बाकु"
    "America/Barbados" "बारबाडोस"
    "Asia/Dhaka" "ढाका"
    "Europe/Brussels" "ब्रूसेल्स"
    "Europe/Sofia" "सोफ़िया"
    "Atlantic/Bermuda" "बरमूडा"
    "America/La_Paz" "ला पाज़"
    "America/Noronha" "नोरोन्हा"
    "America/Belem" "बेलेम"
    "America/Fortaleza" "फ़ोर्टालेज़ा"
    "America/Recife" "रेसाइफ़"
    "America/Araguaina" "आराग्वेना"
    "America/Maceio" "मेसीओ"
    "America/Bahia" "बहिया"
    "America/Sao_Paulo" "साओ पाउलो"
    "America/Campo_Grande" "कैंपो ग्रांडे"
    "America/Cuiaba" "क्यूआबा"
    "America/Santarem" "सैंटारेम"
    "America/Porto_Velho" "पोर्टो वेल्हो"
    "America/Boa_Vista" "बोआ विस्ता"
    "America/Manaus" "मनौस"
    "America/Eirunepe" "ईरुनेपे"
    "America/Rio_Branco" "रियो ब्रांको"
    "Asia/Thimphu" "थिंपू"
    "Europe/Minsk" "मिंस्क"
    "America/Belize" "बेलीज़"
    "America/St_Johns" "सेंट जोंस"
    "America/Halifax" "हेलिफ़ैक्स"
    "America/Glace_Bay" "ग्लेस खाड़ी"
    "America/Moncton" "मोंकटन"
    "America/Goose_Bay" "गूस खाड़ी"
    "America/Toronto" "टोरंटो"
    "America/Iqaluit" "इकालुईट"
    "America/Winnipeg" "विनीपेग"
    "America/Resolute" "रिसोल्यूट"
    "America/Rankin_Inlet" "रेंकिन इनलेट"
    "America/Regina" "रेजिना"
    "America/Swift_Current" "स्विफ़्ट करंट"
    "America/Edmonton" "एडमंटन"
    "America/Cambridge_Bay" "कैम्ब्रिज खाड़ी"
    "America/Inuvik" "इनूविक"
    "America/Vancouver" "वैंकूवर"
    "America/Dawson_Creek" "डॉसन क्रीक"
    "America/Fort_Nelson" "फ़ोर्ट नेल्सन"
    "America/Whitehorse" "व्हाइटहोर्स"
    "America/Dawson" "डॉसन"
    "Europe/Zurich" "ज़्यूरिख़"
    "Africa/Abidjan" "अबिदजान"
    "Pacific/Rarotonga" "रारोटोंगा"
    "America/Santiago" "सैंटियागो"
    "America/Coyhaique" "कॉयहेक"
    "America/Punta_Arenas" "पुंटा एरिनास"
    "Pacific/Easter" "ईस्टर"
    "Asia/Shanghai" "शंघाई"
    "Asia/Urumqi" "उरूम्की"
    "America/Bogota" "बोगोटा"
    "America/Costa_Rica" "कोस्टा रिका"
    "America/Havana" "हवाना"
    "Atlantic/Cape_Verde" "केप वर्ड"
    "Asia/Nicosia" "निकोसिया"
    "Asia/Famagusta" "फ़ामागुस्ता"
    "Europe/Prague" "प्राग"
    "Europe/Berlin" "बर्लिन"
    "America/Santo_Domingo" "सेंटो डोमिंगो"
    "Africa/Algiers" "अल्जीयर्स"
    "America/Guayaquil" "ग्वायाकील"
    "Pacific/Galapagos" "गेलापागोस"
    "Europe/Tallinn" "तेलिन"
    "Africa/Cairo" "कायरो"
    "Africa/El_Aaiun" "अल आइयून"
    "Europe/Madrid" "मैड्रिड"
    "Africa/Ceuta" "सेउटा"
    "Atlantic/Canary" "कैनेरी"
    "Europe/Helsinki" "हेलसिंकी"
    "Pacific/Fiji" "फ़िजी"
    "Atlantic/Stanley" "स्टैनली"
    "Pacific/Kosrae" "कोसराए"
    "Atlantic/Faroe" "फ़ैरो"
    "Europe/Paris" "पेरिस"
    "Europe/London" "लंदन"
    "Asia/Tbilisi" "टबिलिसी"
    "America/Cayenne" "कायेन"
    "Europe/Gibraltar" "जिब्राल्टर"
    "America/Nuuk" "नुक"
    "America/Danmarkshavn" "डेनमार्कशॉन"
    "America/Scoresbysund" "इटोकोर्टोरमिट"
    "America/Thule" "थ्यूले"
    "Europe/Athens" "एथेंस"
    "Atlantic/South_Georgia" "दक्षिण जॉर्जिया"
    "America/Guatemala" "ग्वाटेमाला"
    "Pacific/Guam" "गुआम"
    "Africa/Bissau" "बिसाऊ"
    "America/Guyana" "गयाना"
    "Asia/Hong_Kong" "हाँग काँग"
    "America/Tegucigalpa" "टेगुसिगल्पा"
    "America/Port-au-Prince" "पोर्ट-ऑ-प्रिंस"
    "Europe/Budapest" "बुडापेस्ट"
    "Asia/Jakarta" "जकार्ता"
    "Asia/Pontianak" "पोंटीयांक"
    "Asia/Makassar" "मकस्सर"
    "Asia/Jayapura" "जयापुरा"
    "Europe/Dublin" "डबलिन"
    "Asia/Jerusalem" "यरूशलम"
    "Asia/Kolkata" "कोलकाता"
    "Indian/Chagos" "शागोस"
    "Asia/Baghdad" "बगदाद"
    "Asia/Tehran" "तेहरान"
    "Europe/Rome" "रोम"
    "America/Jamaica" "जमैका"
    "Asia/Amman" "अम्मान"
    "Asia/Tokyo" "टोक्यो"
    "Africa/Nairobi" "नैरोबी"
    "Asia/Bishkek" "बिश्केक"
    "Pacific/Tarawa" "टारावा"
    "Pacific/Kanton" "कैंटन"
    "Pacific/Kiritimati" "किरीतिमाति"
    "Asia/Pyongyang" "प्योंगयांग"
    "Asia/Seoul" "सिओल"
    "Asia/Almaty" "अल्माटी"
    "Asia/Qyzylorda" "केज़ेलोर्डा"
    "Asia/Qostanay" "कोस्टाने"
    "Asia/Aqtobe" "अक्तोब"
    "Asia/Aqtau" "अक्ताउ"
    "Asia/Atyrau" "एतराउ"
    "Asia/Oral" "ओरल"
    "Asia/Beirut" "बेरुत"
    "Asia/Colombo" "कोलंबो"
    "Africa/Monrovia" "मोनरोविया"
    "Europe/Vilnius" "विल्नियस"
    "Europe/Riga" "रीगा"
    "Africa/Tripoli" "त्रिपोली"
    "Africa/Casablanca" "कासाब्लांका"
    "Europe/Chisinau" "चिसीनाउ"
    "Pacific/Kwajalein" "क्वाज़ालीन"
    "Asia/Yangon" "रंगून"
    "Asia/Ulaanbaatar" "उलानबातर"
    "Asia/Hovd" "होव्ड"
    "Asia/Macau" "मकाऊ"
    "America/Martinique" "मार्टिनिक"
    "Europe/Malta" "माल्टा"
    "Indian/Mauritius" "मॉरीशस"
    "Indian/Maldives" "मालदीव"
    "America/Mexico_City" "मेक्सिको सिटी"
    "America/Cancun" "कैनकुन"
    "America/Merida" "मेरिडा"
    "America/Monterrey" "मोंटेरेरी"
    "America/Matamoros" "माटामोरोस"
    "America/Chihuahua" "चिहुआहुआ"
    "America/Ciudad_Juarez" "स्युदाद ह्वारेज़"
    "America/Ojinaga" "ओखाजीनागा"
    "America/Mazatlan" "माज़ाटलान"
    "America/Bahia_Banderas" "बेहिया बांडेरास"
    "America/Hermosillo" "हर्मोसिल्लो"
    "America/Tijuana" "तिजुआना"
    "Asia/Kuching" "कूचिंग"
    "Africa/Maputo" "मापुटो"
    "Africa/Windhoek" "विंडहोक"
    "Pacific/Noumea" "नौमिया"
    "Pacific/Norfolk" "नॉरफ़ॉक"
    "Africa/Lagos" "लागोस"
    "America/Managua" "मानागुआ"
    "Asia/Kathmandu" "काठमांडू"
    "Pacific/Nauru" "नौरु"
    "Pacific/Niue" "नीयू"
    "Pacific/Auckland" "ऑकलैंड"
    "Pacific/Chatham" "चैथम"
    "America/Panama" "पनामा"
    "America/Lima" "लीमा"
    "Pacific/Tahiti" "ताहिती"
    "Pacific/Marquesas" "मार्केसस"
    "Pacific/Gambier" "गैंबियर"
    "Pacific/Port_Moresby" "पोर्ट मोरेस्बी"
    "Pacific/Bougainville" "बोगनविले"
    "Asia/Manila" "मनीला"
    "Asia/Karachi" "कराची"
    "Europe/Warsaw" "वॉरसॉ"
    "America/Miquelon" "मिकेलॉन"
    "Pacific/Pitcairn" "पिटकैर्न"
    "America/Puerto_Rico" "पोर्टो रिको"
    "Asia/Gaza" "गाज़ा"
    "Asia/Hebron" "हेब्रोन"
    "Europe/Lisbon" "लिस्बन"
    "Atlantic/Madeira" "मडेरा"
    "Atlantic/Azores" "अज़ोरेस"
    "Pacific/Palau" "पलाऊ"
    "America/Asuncion" "एसनशियॉन"
    "Asia/Qatar" "कतर"
    "Europe/Bucharest" "बुख़ारेस्ट"
    "Europe/Belgrade" "बेलग्रेड"
    "Europe/Kaliningrad" "कालीनिनग्राड"
    "Europe/Moscow" "मॉस्को"
    "Europe/Simferopol" "सिम्फ़ेरोपोल"
    "Europe/Kirov" "किरोव"
    "Europe/Volgograd" "वोल्गोग्राड"
    "Europe/Astrakhan" "आस्ट्राखान"
    "Europe/Saratov" "सारातोव"
    "Europe/Ulyanovsk" "उल्यानोव्स्क"
    "Europe/Samara" "समारा"
    "Asia/Yekaterinburg" "येकातेरिनबर्ग"
    "Asia/Omsk" "ओम्स्क"
    "Asia/Novosibirsk" "नोवोसिबिर्स्क"
    "Asia/Barnaul" "बर्नोल"
    "Asia/Tomsk" "तोम्स्क"
    "Asia/Novokuznetsk" "नोवोकुज़्नेत्स्क"
    "Asia/Krasnoyarsk" "क्रास्नोयार्स्क"
    "Asia/Irkutsk" "इर्कुत्स्क"
    "Asia/Chita" "त्शिता"
    "Asia/Yakutsk" "याकूत्स्क"
    "Asia/Khandyga" "खांडिगा"
    "Asia/Vladivostok" "व्लादिवोस्तोक"
    "Asia/Ust-Nera" "यूस्ट–नेरा"
    "Asia/Magadan" "मागादान"
    "Asia/Sakhalin" "सखालिन"
    "Asia/Srednekolymsk" "स्रेद्निकोलिमस्क"
    "Asia/Kamchatka" "कमचत्का"
    "Asia/Anadyr" "अनाडिर"
    "Asia/Riyadh" "रियाद"
    "Pacific/Guadalcanal" "ग्वाडलकनाल"
    "Africa/Khartoum" "खार्तूम"
    "Asia/Singapore" "सिंगापुर"
    "America/Paramaribo" "पारामारिबो"
    "Africa/Juba" "जुबा"
    "Africa/Sao_Tome" "साओ टोम"
    "America/El_Salvador" "अल सल्वाडोर"
    "Asia/Damascus" "दमास्कस"
    "America/Grand_Turk" "ग्रांड टर्क"
    "Africa/Ndjamena" "नेद्जामीना"
    "Asia/Bangkok" "बैंकॉक"
    "Asia/Dushanbe" "दुशांबे"
    "Pacific/Fakaofo" "फ़ाकाओफ़ो"
    "Asia/Dili" "डिलि"
    "Asia/Ashgabat" "अश्गाबात"
    "Africa/Tunis" "ट्यूनिस"
    "Pacific/Tongatapu" "टोंगाटापू"
    "Europe/Istanbul" "इस्तांबुल"
    "Asia/Taipei" "ताईपेई"
    "Europe/Kyiv" "कीव"
    "America/New_York" "न्यूयॉर्क"
    "America/Detroit" "डेट्रॉयट"
    "America/Kentucky/Louisville" "लुइसविले"
    "America/Kentucky/Monticello" "मोंटीसेलो, केंटकी"
    "America/Indiana/Indianapolis" "इंडियानापोलिस"
    "America/Indiana/Vincennes" "विंसेनेस, इंडियाना"
    "America/Indiana/Winamac" "विनामेक, इंडियाना"
    "America/Indiana/Marengo" "मारेंगो, इंडियाना"
    "America/Indiana/Petersburg" "पीटर्सबर्ग, इंडियाना"
    "America/Indiana/Vevay" "वेवे, इंडियाना"
    "America/Chicago" "शिकागो"
    "America/Indiana/Tell_City" "टेल सिटी, इंडियाना"
    "America/Indiana/Knox" "नौक्स, इंडियाना"
    "America/Menominee" "मेनोमिनी"
    "America/North_Dakota/Center" "मध्य, उत्तरी दाकोता"
    "America/North_Dakota/New_Salem" "न्यू सालेम, उत्तरी डकोटा"
    "America/North_Dakota/Beulah" "ब्यूला, उत्तरी डकोटा"
    "America/Denver" "डेनवर"
    "America/Boise" "बॉइसी"
    "America/Phoenix" "फ़ीनिक्स"
    "America/Los_Angeles" "लॉस एंजिल्स"
    "America/Anchorage" "एंकरेज"
    "America/Juneau" "ज्यूनाउ"
    "America/Sitka" "सिट्का"
    "America/Metlakatla" "मेट्लेकाट्ला"
    "America/Yakutat" "याकूटाट"
    "America/Nome" "नोम"
    "America/Adak" "अडक"
    "Pacific/Honolulu" "होनोलुलु"
    "America/Montevideo" "मोंटेवीडियो"
    "Asia/Samarkand" "समरकंद"
    "Asia/Tashkent" "ताशकंद"
    "America/Caracas" "काराकस"
    "Asia/Ho_Chi_Minh" "हो ची मिन्ह सिटी"
    "Pacific/Efate" "एफ़ेट"
    "Pacific/Apia" "एपिया"
    "Africa/Johannesburg" "जोहांसबर्ग"
    "America/Antigua" "एंटीगुआ"
    "America/Anguilla" "एंग्विला"
    "Africa/Luanda" "लुआंडा"
    "Antarctica/McMurdo" "मैकमुर्डो"
    "Antarctica/DumontDUrville" "ड्यूमोंट डी अर्विले"
    "Antarctica/Syowa" "स्योवा"
    "America/Aruba" "अरूबा"
    "Europe/Mariehamn" "मारियाहैम"
    "Europe/Sarajevo" "साराजेवो"
    "Africa/Ouagadougou" "औगाडोगू"
    "Asia/Bahrain" "बहरीन"
    "Africa/Bujumbura" "बुजुंबूरा"
    "Africa/Porto-Novo" "पोर्टो-नोवो"
    "America/St_Barthelemy" "सेंट बार्थेलेमी"
    "Asia/Brunei" "ब्रूनेई"
    "America/Kralendijk" "क्रालैंडिजिक"
    "America/Nassau" "नासाउ"
    "Africa/Gaborone" "गाबोरोन"
    "America/Blanc-Sablon" "ब्लांक-सेबलोन"
    "America/Atikokan" "अटिकोकान"
    "America/Creston" "क्रेस्टन"
    "Indian/Cocos" "कोकोस"
    "Africa/Kinshasa" "किंशासा"
    "Africa/Lubumbashi" "लुबुमबाशी"
    "Africa/Bangui" "बांगुइ"
    "Africa/Brazzaville" "ब्राज़ाविले"
    "Africa/Douala" "डूआला"
    "America/Curacao" "कुराकाओ"
    "Indian/Christmas" "क्रिसमस"
    "Europe/Busingen" "ब्यूसिनजेन"
    "Africa/Djibouti" "जिबूती"
    "Europe/Copenhagen" "कोपेनहेगन"
    "America/Dominica" "डोमिनिका"
    "Africa/Asmara" "अस्मारा"
    "Africa/Addis_Ababa" "अदीस अबाबा"
    "Pacific/Chuuk" "चक"
    "Pacific/Pohnpei" "पोनपेई"
    "Africa/Libreville" "लिब्रेविले"
    "America/Grenada" "ग्रेनाडा"
    "Europe/Guernsey" "गर्नसी"
    "Africa/Accra" "एक्रा"
    "Africa/Banjul" "बैंजुल"
    "Africa/Conakry" "कोनाक्री"
    "America/Guadeloupe" "ग्वाडेलोप"
    "Africa/Malabo" "मलाबो"
    "Europe/Zagreb" "ज़ाग्रेब"
    "Europe/Isle_of_Man" "आइल ऑफ़ मैन"
    "Atlantic/Reykjavik" "रेक्याविक"
    "Europe/Jersey" "जर्सी"
    "Asia/Phnom_Penh" "नॉम पेन्ह"
    "Indian/Comoro" "कोमोरो"
    "America/St_Kitts" "सेंट किट्स"
    "Asia/Kuwait" "कुवैत"
    "America/Cayman" "कैमेन"
    "Asia/Vientiane" "विएनतियान"
    "America/St_Lucia" "सेंट लूसिया"
    "Europe/Vaduz" "वादुज़"
    "Africa/Maseru" "मासेरू"
    "Europe/Luxembourg" "लक्ज़मबर्ग"
    "Europe/Monaco" "मोनाको"
    "Europe/Podgorica" "पोड्गोरिका"
    "America/Marigot" "मैरीगोट"
    "Indian/Antananarivo" "एंटानानरीवो"
    "Pacific/Majuro" "माजुरो"
    "Europe/Skopje" "स्कोप्जे"
    "Africa/Bamako" "बामाको"
    "Pacific/Saipan" "सायपान"
    "Africa/Nouakchott" "नौआकशॉट"
    "America/Montserrat" "मोंटसेरात"
    "Africa/Blantyre" "ब्लांटायर"
    "Asia/Kuala_Lumpur" "कुआलालंपुर"
    "Africa/Niamey" "नियामी"
    "Europe/Amsterdam" "एम्स्टर्डम"
    "Europe/Oslo" "ओस्लो"
    "Asia/Muscat" "मस्कट"
    "Indian/Reunion" "रीयूनियन"
    "Africa/Kigali" "किगाली"
    "Indian/Mahe" "माहे"
    "Europe/Stockholm" "स्टॉकहोम"
    "Atlantic/St_Helena" "सेंट हेलेना"
    "Europe/Ljubljana" "ल्यूबेलजाना"
    "Arctic/Longyearbyen" "लॉन्गईयरबायेन"
    "Europe/Bratislava" "ब्रातिस्लावा"
    "Africa/Freetown" "फ़्रीटाउन"
    "Europe/San_Marino" "सैन मारीनो"
    "Africa/Dakar" "डकार"
    "Africa/Mogadishu" "मोगादिशु"
    "America/Lower_Princes" "लोअर प्रिंसेस क्वार्टर"
    "Africa/Mbabane" "एमबाबेन"
    "Indian/Kerguelen" "करगुलेन"
    "Africa/Lome" "लोम"
    "America/Port_of_Spain" "पोर्ट ऑफ़ स्पेन"
    "Pacific/Funafuti" "फ़्यूनाफ़ुटी"
    "Africa/Dar_es_Salaam" "दार अस सलाम"
    "Africa/Kampala" "कंपाला"
    "Pacific/Midway" "मिडवे"
    "Pacific/Wake" "वेक"
    "Europe/Vatican" "वेटिकन"
    "America/St_Vincent" "सेंट विंसेंट"
    "America/Tortola" "टोर्टोला"
    "America/St_Thomas" "सेंट थॉमस"
    "Pacific/Wallis" "वालिस"
    "Asia/Aden" "आदेन"
    "Indian/Mayotte" "मायोत्ते"
    "Africa/Lusaka" "लुसाका"
    "Africa/Harare" "हरारे"
};

// `common/main/id.xml`: 47 of the 418 zones named, 371 inherited.
#[cfg(feature = "localized-exemplar-cities")]
const ID: &str = exemplar_cities! {
    "Europe/Andorra" inherited
    "Asia/Dubai" inherited
    "Asia/Kabul" inherited
    "Europe/Tirane" inherited
    "Asia/Yerevan" inherited
    "Antarctica/Casey" inherited
    "Antarctica/Davis" inherited
    "Antarctica/Mawson" inherited
    "Antarctica/Palmer" inherited
    "Antarctica/Rothera" inherited
    "Antarctica/Troll" inherited
    "Antarctica/Vostok" inherited
    "America/Argentina/Buenos_Aires" inherited
    "America/Argentina/Cordoba" inherited
    "America/Argentina/Salta" inherited
    "America/Argentina/Jujuy" inherited
    "America/Argentina/Tucuman" inherited
    "America/Argentina/Catamarca" inherited
    "America/Argentina/La_Rioja" inherited
    "America/Argentina/San_Juan" inherited
    "America/Argentina/Mendoza" inherited
    "America/Argentina/San_Luis" inherited
    "America/Argentina/Rio_Gallegos" inherited
    "America/Argentina/Ushuaia" inherited
    "Pacific/Pago_Pago" inherited
    "Europe/Vienna" "Wina"
    "Australia/Lord_Howe" inherited
    "Antarctica/Macquarie" inherited
    "Australia/Hobart" inherited
    "Australia/Melbourne" inherited
    "Australia/Sydney" inherited
    "Australia/Broken_Hill" inherited
    "Australia/Brisbane" inherited
    "Australia/Lindeman" inherited
    "Australia/Adelaide" inherited
    "Australia/Darwin" inherited
    "Australia/Perth" inherited
    "Australia/Eucla" inherited
    "Asia/Baku" inherited
    "America/Barbados" inherited
    "Asia/Dhaka" inherited
    "Europe/Brussels" inherited
    "Europe/Sofia" inherited
    "Atlantic/Bermuda" inherited
    "America/La_Paz" inherited
    "America/Noronha" inherited
    "America/Belem" inherited
    "America/Fortaleza" inherited
    "America/Recife" inherited
    "America/Araguaina" inherited
    "America/Maceio" inherited
    "America/Bahia" inherited
    "America/Sao_Paulo" inherited
    "America/Campo_Grande" inherited
    "America/Cuiaba" inherited
    "America/Santarem" inherited
    "America/Porto_Velho" inherited
    "America/Boa_Vista" inherited
    "America/Manaus" inherited
    "America/Eirunepe" inherited
    "America/Rio_Branco" inherited
    "Asia/Thimphu" inherited
    "Europe/Minsk" inherited
    "America/Belize" inherited
    "America/St_Johns" inherited
    "America/Halifax" inherited
    "America/Glace_Bay" inherited
    "America/Moncton" inherited
    "America/Goose_Bay" inherited
    "America/Toronto" inherited
    "America/Iqaluit" inherited
    "America/Winnipeg" inherited
    "America/Resolute" inherited
    "America/Rankin_Inlet" inherited
    "America/Regina" inherited
    "America/Swift_Current" inherited
    "America/Edmonton" inherited
    "America/Cambridge_Bay" inherited
    "America/Inuvik" inherited
    "America/Vancouver" inherited
    "America/Dawson_Creek" inherited
    "America/Fort_Nelson" inherited
    "America/Whitehorse" inherited
    "America/Dawson" inherited
    "Europe/Zurich" inherited
    "Africa/Abidjan" inherited
    "Pacific/Rarotonga" inherited
    "America/Santiago" inherited
    "America/Coyhaique" inherited
    "America/Punta_Arenas" inherited
    "Pacific/Easter" inherited
    "Asia/Shanghai" inherited
    "Asia/Urumqi" inherited
    "America/Bogota" inherited
    "America/Costa_Rica" "Kosta Rika"
    "America/Havana" inherited
    "Atlantic/Cape_Verde" "Tanjung Verde"
    "Asia/Nicosia" "Nikosia"
    "Asia/Famagusta" inherited
    "Europe/Prague" "Praha"
    "Europe/Berlin" inherited
    "America/Santo_Domingo" inherited
    "Africa/Algiers" "Aljir"
    "America/Guayaquil" inherited
    "Pacific/Galapagos" inherited
    "Europe/Tallinn" inherited
    "Africa/Cairo" "Kairo"
    "Africa/El_Aaiun" inherited
    "Europe/Madrid" inherited
    "Africa/Ceuta" inherited
    "Atlantic/Canary" inherited
    "Europe/Helsinki" inherited
    "Pacific/Fiji" inherited
    "Atlantic/Stanley" inherited
    "Pacific/Kosrae" inherited
    "Atlantic/Faroe" inherited
    "Europe/Paris" inherited
    "Europe/London" inherited
    "Asia/Tbilisi" inherited
    "America/Cayenne" inherited
    "Europe/Gibraltar" inherited
    "America/Nuuk" inherited
    "America/Danmarkshavn" inherited
    "America/Scoresbysund" inherited
    "America/Thule" inherited
    "Europe/Athens" "Athena"
    "Atlantic/South_Georgia" "Georgia Selatan"
    "America/Guatemala" inherited
    "Pacific/Guam" inherited
    "Africa/Bissau" inherited
    "America/Guyana" inherited
    "Asia/Hong_Kong" inherited
    "America/Tegucigalpa" inherited
    "America/Port-au-Prince" inherited
    "Europe/Budapest" inherited
    "Asia/Jakarta" inherited
    "Asia/Pontianak" inherited
    "Asia/Makassar" inherited
    "Asia/Jayapura" inherited
    "Europe/Dublin" inherited
    "Asia/Jerusalem" "Yerusalem"
    "Asia/Kolkata" inherited
    "Indian/Chagos" inherited
    "Asia/Baghdad" inherited
    "Asia/Tehran" "Teheran"
    "Europe/Rome" "Roma"
    "America/Jamaica" inherited
    "Asia/Amman" inherited
    "Asia/Tokyo" inherited
    "Africa/Nairobi" inherited
    "Asia/Bishkek" inherited
    "Pacific/Tarawa" inherited
    "Pacific/Kanton" "Pulau Canton"
    "Pacific/Kiritimati" inherited
    "Asia/Pyongyang" inherited
    "Asia/Seoul" inherited
    "Asia/Almaty" inherited
    "Asia/Qyzylorda" inherited
    "Asia/Qostanay" "Kostanay"
    "Asia/Aqtobe" "Aktobe"
    "Asia/Aqtau" "Aktau"
    "Asia/Atyrau" inherited
    "Asia/Oral" inherited
    "Asia/Beirut" inherited
    "Asia/Colombo" "Kolombo"
    "Africa/Monrovia" inherited
    "Europe/Vilnius" inherited
    "Europe/Riga" inherited
    "Africa/Tripoli" inherited
    "Africa/Casablanca" inherited
    "Europe/Chisinau" "Kishinev"
    "Pacific/Kwajalein" inherited
    "Asia/Yangon" "Rangoon"
    "Asia/Ulaanbaatar" inherited
    "Asia/Hovd" inherited
    "Asia/Macau" "Makau"
    "America/Martinique" "Martinik"
    "Europe/Malta" inherited
    "Indian/Mauritius" inherited
    "Indian/Maldives" "Maladewa"
    "America/Mexico_City" inherited
    "America/Cancun" "Cancun"
    "America/Merida" "Merida"
    "America/Monterrey" inherited
    "America/Matamoros" inherited
    "America/Chihuahua" inherited
    "America/Ciudad_Juarez" "Ciudad Juarez"
    "America/Ojinaga" inherited
    "America/Mazatlan" inherited
    "America/Bahia_Banderas" "Bahia Banderas"
    "America/Hermosillo" inherited
    "America/Tijuana" inherited
    "Asia/Kuching" inherited
    "Africa/Maputo" inherited
    "Africa/Windhoek" inherited
    "Pacific/Noumea" inherited
    "Pacific/Norfolk" inherited
    "Africa/Lagos" inherited
    "America/Managua" inherited
    "Asia/Kathmandu" inherited
    "Pacific/Nauru" inherited
    "Pacific/Niue" inherited
    "Pacific/Auckland" inherited
    "Pacific/Chatham" inherited
    "America/Panama" inherited
    "America/Lima" inherited
    "Pacific/Tahiti" inherited
    "Pacific/Marquesas" inherited
    "Pacific/Gambier" inherited
    "Pacific/Port_Moresby" inherited
    "Pacific/Bougainville" inherited
    "Asia/Manila" inherited
    "Asia/Karachi" inherited
    "Europe/Warsaw" "Warsawa"
    "America/Miquelon" inherited
    "Pacific/Pitcairn" inherited
    "America/Puerto_Rico" inherited
    "Asia/Gaza" inherited
    "Asia/Hebron" inherited
    "Europe/Lisbon" inherited
    "Atlantic/Madeira" inherited
    "Atlantic/Azores" inherited
    "Pacific/Palau" inherited
    "America/Asuncion" inherited
    "Asia/Qatar" inherited
    "Europe/Bucharest" inherited
    "Europe/Belgrade" "Beograd"
    "Europe/Kaliningrad" inherited
    "Europe/Moscow" "Moskwa"
    "Europe/Simferopol" inherited
    "Europe/Kirov" inherited
    "Europe/Volgograd" inherited
    "Europe/Astrakhan" inherited
    "Europe/Saratov" inherited
    "Europe/Ulyanovsk" inherited
    "Europe/Samara" inherited
    "Asia/Yekaterinburg" inherited
    "Asia/Omsk" inherited
    "Asia/Novosibirsk" inherited
    "Asia/Barnaul" inherited
    "Asia/Tomsk" inherited
    "Asia/Novokuznetsk" inherited
    "Asia/Krasnoyarsk" inherited
    "Asia/Irkutsk" inherited
    "Asia/Chita" inherited
    "Asia/Yakutsk" inherited
    "Asia/Khandyga" inherited
    "Asia/Vladivostok" inherited
    "Asia/Ust-Nera" inherited
    "Asia/Magadan" inherited
    "Asia/Sakhalin" inherited
    "Asia/Srednekolymsk" inherited
    "Asia/Kamchatka" inherited
    "Asia/Anadyr" inherited
    "Asia/Riyadh" inherited
    "Pacific/Guadalcanal" "Guadalkanal"
    "Africa/Khartoum" inherited
    "Asia/Singapore" "Singapura"
    "America/Paramaribo" inherited
    "Africa/Juba" inherited
    "Africa/Sao_Tome" inherited
    "America/El_Salvador" inherited
    "Asia/Damascus" "Damaskus"
    "America/Grand_Turk" inherited
    "Africa/Ndjamena" inherited
    "Asia/Bangkok" inherited
    "Asia/Dushanbe" inherited
    "Pacific/Fakaofo" inherited
    "Asia/Dili" inherited
    "Asia/Ashgabat" inherited
    "Africa/Tunis" inherited
    "Pacific/Tongatapu" inherited
    "Europe/Istanbul" inherited
    "Asia/Taipei" inherited
    "Europe/Kyiv" "Kiev"
    "America/New_York" inherited
    "America/Detroit" inherited
    "America/Kentucky/Louisville" inherited
    "America/Kentucky/Monticello" inherited
    "America/Indiana/Indianapolis" inherited
    "America/Indiana/Vincennes" inherited
    "America/Indiana/Winamac" inherited
    "America/Indiana/Marengo" inherited
    "America/Indiana/Petersburg" inherited
    "America/Indiana/Vevay" inherited
    "America/Chicago" inherited
    "America/Indiana/Tell_City" inherited
    "America/Indiana/Knox" inherited
    "America/Menominee" inherited
    "America/North_Dakota/Center" "Center, Dakota Utara"
    "America/North_Dakota/New_Salem" "New Salem, Dakota Utara"
    "America/North_Dakota/Beulah" "Beulah, Dakota Utara"
    "America/Denver" inherited
    "America/Boise" inherited
    "America/Phoenix" inherited
    "America/Los_Angeles" inherited
    "America/Anchorage" inherited
    "America/Juneau" inherited
    "America/Sitka" inherited
    "America/Metlakatla" inherited
    "America/Yakutat" inherited
    "America/Nome" inherited
    "America/Adak" inherited
    "Pacific/Honolulu" "Honolulu"
    "America/Montevideo" inherited
    "Asia/Samarkand" inherited
    "Asia/Tashkent" inherited
    "America/Caracas" inherited
    "Asia/Ho_Chi_Minh" inherited
    "Pacific/Efate" inherited
    "Pacific/Apia" inherited
    "Africa/Johannesburg" inherited
    "America/Antigua" inherited
    "America/Anguilla" "Anguila"
    "Africa/Luanda" inherited
    "Antarctica/McMurdo" inherited
    "Antarctica/DumontDUrville" inherited
    "Antarctica/Syowa" inherited
    "America/Aruba" inherited
    "Europe/Mariehamn" inherited
    "Europe/Sarajevo" inherited
    "Africa/Ouagadougou" inherited
    "Asia/Bahrain" inherited
    "Africa/Bujumbura" inherited
    "Africa/Porto-Novo" inherited
    "America/St_Barthelemy" inherited
    "Asia/Brunei" inherited
    "America/Kralendijk" inherited
    "America/Nassau" inherited
    "Africa/Gaborone" inherited
    "America/Blanc-Sablon" inherited
    "America/Atikokan" inherited
    "America/Creston" inherited
    "Indian/Cocos" inherited
    "Africa/Kinshasa" inherited
    "Africa/Lubumbashi" inherited
    "Africa/Bangui" inherited
    "Africa/Brazzaville" inherited
    "Africa/Douala" inherited
    "America/Curacao" inherited
    "Indian/Christmas" inherited
    "Europe/Busingen" inherited
    "Africa/Djibouti" inherited
    "Europe/Copenhagen" "Kopenhagen"
    "America/Dominica" "Dominika"
    "Africa/Asmara" inherited
    "Africa/Addis_Ababa" inherited
    "Pacific/Chuuk" inherited
    "Pacific/Pohnpei" inherited
    "Africa/Libreville" inherited
    "America/Grenada" inherited
    "Europe/Guernsey" inherited
    "Africa/Accra" inherited
    "Africa/Banjul" inherited
    "Africa/Conakry" inherited
    "America/Guadeloupe" inherited
    "Africa/Malabo" inherited
    "Europe/Zagreb" inherited
    "Europe/Isle_of_Man" "Pulau Man"
    "Atlantic/Reykjavik" inherited
    "Europe/Jersey" inherited
    "Asia/Phnom_Penh" inherited
    "Indian/Comoro" "Komoro"
    "America/St_Kitts" inherited
    "Asia/Kuwait" inherited
    "America/Cayman" inherited
    "Asia/Vientiane" inherited
    "America/St_Lucia" inherited
    "Europe/Vaduz" inherited
    "Africa/Maseru" inherited
    "Europe/Luxembourg" "Luksemburg"
    "Europe/Monaco" "Monako"
    "Europe/Podgorica" inherited
    "America/Marigot" inherited
    "Indian/Antananarivo" inherited
    "Pacific/Majuro" inherited
    "Europe/Skopje" inherited
    "Africa/Bamako" inherited
    "Pacific/Saipan" inherited
    "Africa/Nouakchott" inherited
    "America/Montserrat" inherited
    "Africa/Blantyre" inherited
    "Asia/Kuala_Lumpur" inherited
    "Africa/Niamey" inherited
    "Europe/Amsterdam" inherited
    "Europe/Oslo" inherited
    "Asia/Muscat" "Muskat"
    "Indian/Reunion" inherited
    "Africa/Kigali" inherited
    "Indian/Mahe" inherited
    "Europe/Stockholm" inherited
    "Atlantic/St_Helena" inherited
    "Europe/Ljubljana" inherited
    "Arctic/Longyearbyen" inherited
    "Europe/Bratislava" inherited
    "Africa/Freetown" inherited
    "Europe/San_Marino" inherited
    "Africa/Dakar" inherited
    "Africa/Mogadishu" inherited
    "America/Lower_Princes" inherited
    "Africa/Mbabane" inherited
    "Indian/Kerguelen" inherited
    "Africa/Lome" inherited
    "America/Port_of_Spain" inherited
    "Pacific/Funafuti" inherited
    "Africa/Dar_es_Salaam" inherited
    "Africa/Kampala" inherited
    "Pacific/Midway" inherited
    "Pacific/Wake" "Pulau Wake"
    "Europe/Vatican" "Vatikan"
    "America/St_Vincent" inherited
    "America/Tortola" inherited
    "America/St_Thomas" inherited
    "Pacific/Wallis" inherited
    "Asia/Aden" inherited
    "Indian/Mayotte" inherited
    "Africa/Lusaka" inherited
    "Africa/Harare" inherited
};

// `common/main/it.xml`: 86 of the 418 zones named, 332 inherited.
#[cfg(feature = "localized-exemplar-cities")]
const IT: &str = exemplar_cities! {
    "Europe/Andorra" inherited
    "Asia/Dubai" inherited
    "Asia/Kabul" inherited
    "Europe/Tirane" inherited
    "Asia/Yerevan" inherited
    "Antarctica/Casey" inherited
    "Antarctica/Davis" inherited
    "Antarctica/Mawson" inherited
    "Antarctica/Palmer" inherited
    "Antarctica/Rothera" inherited
    "Antarctica/Troll" inherited
    "Antarctica/Vostok" inherited
    "America/Argentina/Buenos_Aires" inherited
    "America/Argentina/Cordoba" inherited
    "America/Argentina/Salta" inherited
    "America/Argentina/Jujuy" inherited
    "America/Argentina/Tucuman" inherited
    "America/Argentina/Catamarca" inherited
    "America/Argentina/La_Rioja" inherited
    "America/Argentina/San_Juan" inherited
    "America/Argentina/Mendoza" inherited
    "America/Argentina/San_Luis" inherited
    "America/Argentina/Rio_Gallegos" inherited
    "America/Argentina/Ushuaia" inherited
    "Pacific/Pago_Pago" inherited
    "Europe/Vienna" inherited
    "Australia/Lord_Howe" inherited
    "Antarctica/Macquarie" inherited
    "Australia/Hobart" inherited
    "Australia/Melbourne" inherited
    "Australia/Sydney" inherited
    "Australia/Broken_Hill" inherited
    "Australia/Brisbane" inherited
    "Australia/Lindeman" inherited
    "Australia/Adelaide" inherited
    "Australia/Darwin" inherited
    "Australia/Perth" inherited
    "Australia/Eucla" inherited
    "Asia/Baku" inherited
    "America/Barbados" inherited
    "Asia/Dhaka" "Dacca"
    "Europe/Brussels" "Bruxelles"
    "Europe/Sofia" inherited
    "Atlantic/Bermuda" inherited
    "America/La_Paz" inherited
    "America/Noronha" inherited
    "America/Belem" inherited
    "America/Fortaleza" inherited
    "America/Recife" inherited
    "America/Araguaina" inherited
    "America/Maceio" inherited
    "America/Bahia" inherited
    "America/Sao_Paulo" "San Paolo"
    "America/Campo_Grande" inherited
    "America/Cuiaba" inherited
    "America/Santarem" inherited
    "America/Porto_Velho" inherited
    "America/Boa_Vista" inherited
    "America/Manaus" inherited
    "America/Eirunepe" inherited
    "America/Rio_Branco" inherited
    "Asia/Thimphu" inherited
    "Europe/Minsk" inherited
    "America/Belize" inherited
    "America/St_Johns" inherited
    "America/Halifax" inherited
    "America/Glace_Bay" inherited
    "America/Moncton" inherited
    "America/Goose_Bay" inherited
    "America/Toronto" inherited
    "America/Iqaluit" inherited
    "America/Winnipeg" inherited
    "America/Resolute" inherited
    "America/Rankin_Inlet" inherited
    "America/Regina" inherited
    "America/Swift_Current" inherited
    "America/Edmonton" inherited
    "America/Cambridge_Bay" inherited
    "America/Inuvik" inherited
    "America/Vancouver" inherited
    "America/Dawson_Creek" inherited
    "America/Fort_Nelson" inherited
    "America/Whitehorse" inherited
    "America/Dawson" inherited
    "Europe/Zurich" "Zurigo"
    "Africa/Abidjan" inherited
    "Pacific/Rarotonga" inherited
    "America/Santiago" inherited
    "America/Coyhaique" inherited
    "America/Punta_Arenas" inherited
    "Pacific/Easter" "Pasqua"
    "Asia/Shanghai" inherited
    "Asia/Urumqi" inherited
    "America/Bogota" inherited
    "America/Costa_Rica" inherited
    "America/Havana" "L’Avana"
    "Atlantic/Cape_Verde" "Capo Verde"
    "Asia/Nicosia" inherited
    "Asia/Famagusta" "Famagosta"
    "Europe/Prague" "Praga"
    "Europe/Berlin" "Berlino"
    "America/Santo_Domingo" inherited
    "Africa/Algiers" "Algeri"
    "America/Guayaquil" inherited
    "Pacific/Galapagos" inherited
    "Europe/Tallinn" inherited
    "Africa/Cairo" "Il Cairo"
    "Africa/El_Aaiun" "El Ayun"
    "Europe/Madrid" inherited
    "Africa/Ceuta" inherited
    "Atlantic/Canary" "Canarie"
    "Europe/Helsinki" inherited
    "Pacific/Fiji" "Figi"
    "Atlantic/Stanley" inherited
    "Pacific/Kosrae" inherited
    "Atlantic/Faroe" "Isole Fær Øer"
    "Europe/Paris" "Parigi"
    "Europe/London" "Londra"
    "Asia/Tbilisi" inherited
    "America/Cayenne" "Caienna"
    "Europe/Gibraltar" "Gibilterra"
    "America/Nuuk" inherited
    "America/Danmarkshavn" inherited
    "America/Scoresbysund" inherited
    "America/Thule" inherited
    "Europe/Athens" "Atene"
    "Atlantic/South_Georgia" "Georgia del Sud"
    "America/Guatemala" inherited
    "Pacific/Guam" inherited
    "Africa/Bissau" inherited
    "America/Guyana" inherited
    "Asia/Hong_Kong" inherited
    "America/Tegucigalpa" inherited
    "America/Port-au-Prince" inherited
    "Europe/Budapest" inherited
    "Asia/Jakarta" "Giacarta"
    "Asia/Pontianak" inherited
    "Asia/Makassar" inherited
    "Asia/Jayapura" inherited
    "Europe/Dublin" "Dublino"
    "Asia/Jerusalem" "Gerusalemme"
    "Asia/Kolkata" "Calcutta"
    "Indian/Chagos" inherited
    "Asia/Baghdad" inherited
    "Asia/Tehran" "Teheran"
    "Europe/Rome" "Roma"
    "America/Jamaica" "Giamaica"
    "Asia/Amman" inherited
    "Asia/Tokyo" inherited
    "Africa/Nairobi" inherited
    "Asia/Bishkek" inherited
    "Pacific/Tarawa" inherited
    "Pacific/Kanton" inherited
    "Pacific/Kiritimati" inherited
    "Asia/Pyongyang" inherited
    "Asia/Seoul" "Seul"
    "Asia/Almaty" inherited
    "Asia/Qyzylorda" inherited
    "Asia/Qostanay" inherited
    "Asia/Aqtobe" "Aqtöbe"
    "Asia/Aqtau" inherited
    "Asia/Atyrau" inherited
    "Asia/Oral" inherited
    "Asia/Beirut" inherited
    "Asia/Colombo" inherited
    "Africa/Monrovia" inherited
    "Europe/Vilnius" inherited
    "Europe/Riga" inherited
    "Africa/Tripoli" inherited
    "Africa/Casablanca" inherited
    "Europe/Chisinau" inherited
    "Pacific/Kwajalein" inherited
    "Asia/Yangon" "Rangoon"
    "Asia/Ulaanbaatar" "Ulan Bator"
    "Asia/Hovd" inherited
    "Asia/Macau" inherited
    "America/Martinique" "Martinica"
    "Europe/Malta" inherited
    "Indian/Mauritius" inherited
    "Indian/Maldives" "Maldive"
    "America/Mexico_City" "Città del Messico"
    "America/Cancun" inherited
    "America/Merida" inherited
    "America/Monterrey" inherited
    "America/Matamoros" inherited
    "America/Chihuahua" inherited
    "America/Ciudad_Juarez" inherited
    "America/Ojinaga" inherited
    "America/Mazatlan" inherited
    "America/Bahia_Banderas" inherited
    "America/Hermosillo" inherited
    "America/Tijuana" inherited
    "Asia/Kuching" inherited
    "Africa/Maputo" inherited
    "Africa/Windhoek" inherited
    "Pacific/Noumea" inherited
    "Pacific/Norfolk" inherited
    "Africa/Lagos" inherited
    "America/Managua" inherited
    "Asia/Kathmandu" inherited
    "Pacific/Nauru" inherited
    "Pacific/Niue" inherited
    "Pacific/Auckland" inherited
    "Pacific/Chatham" inherited
    "America/Panama" inherited
    "America/Lima" inherited
    "Pacific/Tahiti" inherited
    "Pacific/Marquesas" "Marchesi"
    "Pacific/Gambier" inherited
    "Pacific/Port_Moresby" inherited
    "Pacific/Bougainville" inherited
    "Asia/Manila" inherited
    "Asia/Karachi" inherited
    "Europe/Warsaw" "Varsavia"
    "America/Miquelon" inherited
    "Pacific/Pitcairn" inherited
    "America/Puerto_Rico" "Portorico"
    "Asia/Gaza" inherited
    "Asia/Hebron" inherited
    "Europe/Lisbon" "Lisbona"
    "Atlantic/Madeira" inherited
    "Atlantic/Azores" "Azzorre"
    "Pacific/Palau" inherited
    "America/Asuncion" inherited
    "Asia/Qatar" inherited
    "Europe/Bucharest" "Bucarest"
    "Europe/Belgrade" "Belgrado"
    "Europe/Kaliningrad" inherited
    "Europe/Moscow" "Mosca"
    "Europe/Simferopol" "Sinferopoli"
    "Europe/Kirov" inherited
    "Europe/Volgograd" inherited
    "Europe/Astrakhan" inherited
    "Europe/Saratov" inherited
    "Europe/Ulyanovsk" inherited
    "Europe/Samara" inherited
    "Asia/Yekaterinburg" "Ekaterinburg"
    "Asia/Omsk" inherited
    "Asia/Novosibirsk" inherited
    "Asia/Barnaul" inherited
    "Asia/Tomsk" inherited
    "Asia/Novokuznetsk" "Novokuzneck"
    "Asia/Krasnoyarsk" "Krasnojarsk"
    "Asia/Irkutsk" inherited
    "Asia/Chita" "Čita"
    "Asia/Yakutsk" "Jakutsk"
    "Asia/Khandyga" "Chandyga"
    "Asia/Vladivostok" inherited
    "Asia/Ust-Nera" "Ust’-Nera"
    "Asia/Magadan" inherited
    "Asia/Sakhalin" "Sachalin"
    "Asia/Srednekolymsk" inherited
    "Asia/Kamchatka" inherited
    "Asia/Anadyr" "Anadyr’"
    "Asia/Riyadh" "Riyad"
    "Pacific/Guadalcanal" inherited
    "Africa/Khartoum" "Khartum"
    "Asia/Singapore" inherited
    "America/Paramaribo" inherited
    "Africa/Juba" "Giuba"
    "Africa/Sao_Tome" inherited
    "America/El_Salvador" inherited
    "Asia/Damascus" "Damasco"
    "America/Grand_Turk" inherited
    "Africa/Ndjamena" inherited
    "Asia/Bangkok" inherited
    "Asia/Dushanbe" inherited
    "Pacific/Fakaofo" inherited
    "Asia/Dili" inherited
    "Asia/Ashgabat" inherited
    "Africa/Tunis" "Tunisi"
    "Pacific/Tongatapu" inherited
    "Europe/Istanbul" inherited
    "Asia/Taipei" inherited
    "Europe/Kyiv" "Kiev"
    "America/New_York" inherited
    "America/Detroit" inherited
    "America/Kentucky/Louisville" inherited
    "America/Kentucky/Monticello" inherited
    "America/Indiana/Indianapolis" inherited
    "America/Indiana/Vincennes" inherited
    "America/Indiana/Winamac" inherited
    "America/Indiana/Marengo" inherited
    "America/Indiana/Petersburg" inherited
    "America/Indiana/Vevay" inherited
    "America/Chicago" inherited
    "America/Indiana/Tell_City" inherited
    "America/Indiana/Knox" inherited
    "America/Menominee" inherited
    "America/North_Dakota/Center" "Center, Dakota del nord"
    "America/North_Dakota/New_Salem" "New Salem, Dakota del nord"
    "America/North_Dakota/Beulah" "Beulah, Dakota del nord"
    "America/Denver" inherited
    "America/Boise" inherited
    "America/Phoenix" inherited
    "America/Los_Angeles" inherited
    "America/Anchorage" inherited
    "America/Juneau" inherited
    "America/Sitka" inherited
    "America/Metlakatla" inherited
    "America/Yakutat" inherited
    "America/Nome" inherited
    "America/Adak" inherited
    "Pacific/Honolulu" "Honolulu"
    "America/Montevideo" inherited
    "Asia/Samarkand" "Samarcanda"
    "Asia/Tashkent" inherited
    "America/Caracas" inherited
    "Asia/Ho_Chi_Minh" inherited
    "Pacific/Efate" inherited
    "Pacific/Apia" inherited
    "Africa/Johannesburg" inherited
    "America/Antigua" inherited
    "America/Anguilla" inherited
    "Africa/Luanda" inherited
    "Antarctica/McMurdo" inherited
    "Antarctica/DumontDUrville" inherited
    "Antarctica/Syowa" inherited
    "America/Aruba" inherited
    "Europe/Mariehamn" inherited
    "Europe/Sarajevo" inherited
    "Africa/Ouagadougou" inherited
    "Asia/Bahrain" "Bahrein"
    "Africa/Bujumbura" inherited
    "Africa/Porto-Novo" inherited
    "America/St_Barthelemy" "Saint-Barthélemy"
    "Asia/Brunei" inherited
    "America/Kralendijk" inherited
    "America/Nassau" inherited
    "Africa/Gaborone" inherited
    "America/Blanc-Sablon" inherited
    "America/Atikokan" inherited
    "America/Creston" inherited
    "Indian/Cocos" inherited
    "Africa/Kinshasa" inherited
    "Africa/Lubumbashi" inherited
    "Africa/Bangui" inherited
    "Africa/Brazzaville" inherited
    "Africa/Douala" inherited
    "America/Curacao" inherited
    "Indian/Christmas" inherited
    "Europe/Busingen" inherited
    "Africa/Djibouti" "Gibuti"
    "Europe/Copenhagen" "Copenaghen"
    "America/Dominica" inherited
    "Africa/Asmara" inherited
    "Africa/Addis_Ababa" "Addis Abeba"
    "Pacific/Chuuk" inherited
    "Pacific/Pohnpei" inherited
    "Africa/Libreville" inherited
    "America/Grenada" inherited
    "Europe/Guernsey" inherited
    "Africa/Accra" inherited
    "Africa/Banjul" inherited
    "Africa/Conakry" inherited
    "America/Guadeloupe" "Guadalupa"
    "Africa/Malabo" inherited
    "Europe/Zagreb" "Zagabria"
    "Europe/Isle_of_Man" "Isola di Man"
    "Atlantic/Reykjavik" "Reykjavík"
    "Europe/Jersey" inherited
    "Asia/Phnom_Penh" inherited
    "Indian/Comoro" "Comore"
    "America/St_Kitts" inherited
    "Asia/Kuwait" inherited
    "America/Cayman" inherited
    "Asia/Vientiane" inherited
    "America/St_Lucia" "Santa Lucia"
    "Europe/Vaduz" inherited
    "Africa/Maseru" inherited
    "Europe/Luxembourg" "Lussemburgo"
    "Europe/Monaco" inherited
    "Europe/Podgorica" inherited
    "America/Marigot" inherited
    "Indian/Antananarivo" inherited
    "Pacific/Majuro" inherited
    "Europe/Skopje" inherited
    "Africa/Bamako" inherited
    "Pacific/Saipan" inherited
    "Africa/Nouakchott" inherited
    "America/Montserrat" inherited
    "Africa/Blantyre" inherited
    "Asia/Kuala_Lumpur" inherited
    "Africa/Niamey" inherited
    "Europe/Amsterdam" inherited
    "Europe/Oslo" inherited
    "Asia/Muscat" "Mascate"
    "Indian/Reunion" "La Riunione"
    "Africa/Kigali" inherited
    "Indian/Mahe" inherited
    "Europe/Stockholm" "Stoccolma"
    "Atlantic/St_Helena" "Sant’Elena"
    "Europe/Ljubljana" "Lubiana"
    "Arctic/Longyearbyen" inherited
    "Europe/Bratislava" inherited
    "Africa/Freetown" inherited
    "Europe/San_Marino" inherited
    "Africa/Dakar" inherited
    "Africa/Mogadishu" "Mogadiscio"
    "America/Lower_Princes" inherited
    "Africa/Mbabane" inherited
    "Indian/Kerguelen" inherited
    "Africa/Lome" inherited
    "America/Port_of_Spain" inherited
    "Pacific/Funafuti" inherited
    "Africa/Dar_es_Salaam" inherited
    "Africa/Kampala" inherited
    "Pacific/Midway" inherited
    "Pacific/Wake" inherited
    "Europe/Vatican" "Città del Vaticano"
    "America/St_Vincent" "Saint Vincent"
    "America/Tortola" inherited
    "America/St_Thomas" "Saint Thomas"
    "Pacific/Wallis" inherited
    "Asia/Aden" inherited
    "Indian/Mayotte" inherited
    "Africa/Lusaka" inherited
    "Africa/Harare" inherited
};

// `common/main/ja.xml`: 418 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const JA: &str = exemplar_cities! {
    "Europe/Andorra" "アンドラ"
    "Asia/Dubai" "ドバイ"
    "Asia/Kabul" "カブール"
    "Europe/Tirane" "ティラナ"
    "Asia/Yerevan" "エレバン"
    "Antarctica/Casey" "ケーシー基地"
    "Antarctica/Davis" "デービス基地"
    "Antarctica/Mawson" "モーソン基地"
    "Antarctica/Palmer" "パーマー基地"
    "Antarctica/Rothera" "ロゼラ基地"
    "Antarctica/Troll" "トロル基地"
    "Antarctica/Vostok" "ボストーク基地"
    "America/Argentina/Buenos_Aires" "ブエノスアイレス"
    "America/Argentina/Cordoba" "コルドバ"
    "America/Argentina/Salta" "サルタ"
    "America/Argentina/Jujuy" "フフイ"
    "America/Argentina/Tucuman" "トゥクマン"
    "America/Argentina/Catamarca" "カタマルカ"
    "America/Argentina/La_Rioja" "ラリオハ"
    "America/Argentina/San_Juan" "サンファン"
    "America/Argentina/Mendoza" "メンドーサ"
    "America/Argentina/San_Luis" "サンルイス"
    "America/Argentina/Rio_Gallegos" "リオガジェゴス"
    "America/Argentina/Ushuaia" "ウシュアイア"
    "Pacific/Pago_Pago" "パゴパゴ"
    "Europe/Vienna" "ウィーン"
    "Australia/Lord_Howe" "ロードハウ"
    "Antarctica/Macquarie" "マッコリー"
    "Australia/Hobart" "ホバート"
    "Australia/Melbourne" "メルボルン"
    "Australia/Sydney" "シドニー"
    "Australia/Broken_Hill" "ブロークンヒル"
    "Australia/Brisbane" "ブリスベン"
    "Australia/Lindeman" "リンデマン"
    "Australia/Adelaide" "アデレード"
    "Australia/Darwin" "ダーウィン"
    "Australia/Perth" "パース"
    "Australia/Eucla" "ユークラ"
    "Asia/Baku" "バクー"
    "America/Barbados" "バルバドス"
    "Asia/Dhaka" "ダッカ"
    "Europe/Brussels" "ブリュッセル"
    "Europe/Sofia" "ソフィア"
    "Atlantic/Bermuda" "バミューダ"
    "America/La_Paz" "ラパス"
    "America/Noronha" "ノローニャ"
    "America/Belem" "ベレン"
    "America/Fortaleza" "フォルタレザ"
    "America/Recife" "レシフェ"
    "America/Araguaina" "アラグァイナ"
    "America/Maceio" "マセイオ"
    "America/Bahia" "バイーア"
    "America/Sao_Paulo" "サンパウロ"
    "America/Campo_Grande" "カンポグランデ"
    "America/Cuiaba" "クイアバ"
    "America/Santarem" "サンタレム"
    "America/Porto_Velho" "ポルトベーリョ"
    "America/Boa_Vista" "ボアビスタ"
    "America/Manaus" "マナウス"
    "America/Eirunepe" "エイルネペ"
    "America/Rio_Branco" "リオブランコ"
    "Asia/Thimphu" "ティンプー"
    "Europe/Minsk" "ミンスク"
    "America/Belize" "ベリーズ"
    "America/St_Johns" "セントジョンズ"
    "America/Halifax" "ハリファクス"
    "America/Glace_Bay" "グレースベイ"
    "America/Moncton" "モンクトン"
    "America/Goose_Bay" "グースベイ"
    "America/Toronto" "トロント"
    "America/Iqaluit" "イカルイット"
    "America/Winnipeg" "ウィニペグ"
    "America/Resolute" "レゾリュート"
    "America/Rankin_Inlet" "ランキンインレット"
    "America/Regina" "レジャイナ"
    "America/Swift_Current" "スウィフトカレント"
    "America/Edmonton" "エドモントン"
    "America/Cambridge_Bay" "ケンブリッジベイ"
    "America/Inuvik" "イヌヴィク"
    "America/Vancouver" "バンクーバー"
    "America/Dawson_Creek" "ドーソンクリーク"
    "America/Fort_Nelson" "フォートネルソン"
    "America/Whitehorse" "ホワイトホース"
    "America/Dawson" "ドーソン"
    "Europe/Zurich" "チューリッヒ"
    "Africa/Abidjan" "アビジャン"
    "Pacific/Rarotonga" "ラロトンガ"
    "America/Santiago" "サンチアゴ"
    "America/Coyhaique" "コジャイケ"
    "America/Punta_Arenas" "プンタアレナス"
    "Pacific/Easter" "イースター島"
    "Asia/Shanghai" "上海"
    "Asia/Urumqi" "ウルムチ"
    "America/Bogota" "ボゴタ"
    "America/Costa_Rica" "コスタリカ"
    "America/Havana" "ハバナ"
    "Atlantic/Cape_Verde" "カーボベルデ"
    "Asia/Nicosia" "ニコシア"
    "Asia/Famagusta" "ファマグスタ"
    "Europe/Prague" "プラハ"
    "Europe/Berlin" "ベルリン"
    "America/Santo_Domingo" "サントドミンゴ"
    "Africa/Algiers" "アルジェ"
    "America/Guayaquil" "グアヤキル"
    "Pacific/Galapagos" "ガラパゴス"
    "Europe/Tallinn" "タリン"
    "Africa/Cairo" "カイロ"
    "Africa/El_Aaiun" "アイウン"
    "Europe/Madrid" "マドリード"
    "Africa/Ceuta" "セウタ"
    "Atlantic/Canary" "カナリア"
    "Europe/Helsinki" "ヘルシンキ"
    "Pacific/Fiji" "フィジー"
    "Atlantic/Stanley" "スタンレー"
    "Pacific/Kosrae" "コスラエ"
    "Atlantic/Faroe" "フェロー"
    "Europe/Paris" "パリ"
    "Europe/London" "ロンドン"
    "Asia/Tbilisi" "トビリシ"
    "America/Cayenne" "カイエンヌ"
    "Europe/Gibraltar" "ジブラルタル"
    "America/Nuuk" "ヌーク"
    "America/Danmarkshavn" "デンマークシャウン"
    "America/Scoresbysund" "イトコルトルミット"
    "America/Thule" "チューレ"
    "Europe/Athens" "アテネ"
    "Atlantic/South_Georgia" "サウスジョージア"
    "America/Guatemala" "グアテマラ"
    "Pacific/Guam" "グアム"
    "Africa/Bissau" "ビサウ"
    "America/Guyana" "ガイアナ"
    "Asia/Hong_Kong" "香港"
    "America/Tegucigalpa" "テグシガルパ"
    "America/Port-au-Prince" "ポルトープランス"
    "Europe/Budapest" "ブダペスト"
    "Asia/Jakarta" "ジャカルタ"
    "Asia/Pontianak" "ポンティアナック"
    "Asia/Makassar" "マカッサル"
    "Asia/Jayapura" "ジャヤプラ"
    "Europe/Dublin" "ダブリン"
    "Asia/Jerusalem" "エルサレム"
    "Asia/Kolkata" "コルカタ"
    "Indian/Chagos" "チャゴス"
    "Asia/Baghdad" "バグダッド"
    "Asia/Tehran" "テヘラン"
    "Europe/Rome" "ローマ"
    "America/Jamaica" "ジャマイカ"
    "Asia/Amman" "アンマン"
    "Asia/Tokyo" "東京"
    "Africa/Nairobi" "ナイロビ"
    "Asia/Bishkek" "ビシュケク"
    "Pacific/Tarawa" "タラワ"
    "Pacific/Kanton" "カントン島"
    "Pacific/Kiritimati" "キリスィマスィ島"
    "Asia/Pyongyang" "平壌"
    "Asia/Seoul" "ソウル"
    "Asia/Almaty" "アルマトイ"
    "Asia/Qyzylorda" "クズロルダ"
    "Asia/Qostanay" "コスタナイ"
    "Asia/Aqtobe" "アクトベ"
    "Asia/Aqtau" "アクタウ"
    "Asia/Atyrau" "アティラウ"
    "Asia/Oral" "オラル"
    "Asia/Beirut" "ベイルート"
    "Asia/Colombo" "コロンボ"
    "Africa/Monrovia" "モンロビア"
    "Europe/Vilnius" "ヴィリニュス"
    "Europe/Riga" "リガ"
    "Africa/Tripoli" "トリポリ"
    "Africa/Casablanca" "カサブランカ"
    "Europe/Chisinau" "キシナウ"
    "Pacific/Kwajalein" "クェゼリン"
    "Asia/Yangon" "ヤンゴン"
    "Asia/Ulaanbaatar" "ウランバートル"
    "Asia/Hovd" "ホブド"
    "Asia/Macau" "マカオ"
    "America/Martinique" "マルティニーク"
    "Europe/Malta" "マルタ"
    "Indian/Mauritius" "モーリシャス"
    "Indian/Maldives" "モルディブ"
    "America/Mexico_City" "メキシコシティー"
    "America/Cancun" "カンクン"
    "America/Merida" "メリダ"
    "America/Monterrey" "モンテレイ"
    "America/Matamoros" "マタモロス"
    "America/Chihuahua" "チワワ"
    "America/Ciudad_Juarez" "シウダー・フアレス"
    "America/Ojinaga" "オヒナガ"
    "America/Mazatlan" "マサトラン"
    "America/Bahia_Banderas" "バイアバンデラ"
    "America/Hermosillo" "エルモシヨ"
    "America/Tijuana" "ティフアナ"
    "Asia/Kuching" "クチン"
    "Africa/Maputo" "マプト"
    "Africa/Windhoek" "ウィントフック"
    "Pacific/Noumea" "ヌメア"
    "Pacific/Norfolk" "ノーフォーク島"
    "Africa/Lagos" "ラゴス"
    "America/Managua" "マナグア"
    "Asia/Kathmandu" "カトマンズ"
    "Pacific/Nauru" "ナウル"
    "Pacific/Niue" "ニウエ"
    "Pacific/Auckland" "オークランド"
    "Pacific/Chatham" "チャタム"
    "America/Panama" "パナマ"
    "America/Lima" "リマ"
    "Pacific/Tahiti" "タヒチ"
    "Pacific/Marquesas" "マルキーズ"
    "Pacific/Gambier" "ガンビエ諸島"
    "Pacific/Port_Moresby" "ポートモレスビー"
    "Pacific/Bougainville" "ブーゲンビル"
    "Asia/Manila" "マニラ"
    "Asia/Karachi" "カラチ"
    "Europe/Warsaw" "ワルシャワ"
    "America/Miquelon" "ミクロン島"
    "Pacific/Pitcairn" "ピトケアン諸島"
    "America/Puerto_Rico" "プエルトリコ"
    "Asia/Gaza" "ガザ"
    "Asia/Hebron" "ヘブロン"
    "Europe/Lisbon" "リスボン"
    "Atlantic/Madeira" "マデイラ"
    "Atlantic/Azores" "アゾレス"
    "Pacific/Palau" "パラオ"
    "America/Asuncion" "アスンシオン"
    "Asia/Qatar" "カタール"
    "Europe/Bucharest" "ブカレスト"
    "Europe/Belgrade" "ベオグラード"
    "Europe/Kaliningrad" "カリーニングラード"
    "Europe/Moscow" "モスクワ"
    "Europe/Simferopol" "シンフェロポリ"
    "Europe/Kirov" "キーロフ"
    "Europe/Volgograd" "ボルゴグラード"
    "Europe/Astrakhan" "アストラハン"
    "Europe/Saratov" "サラトフ"
    "Europe/Ulyanovsk" "ウリヤノフスク"
    "Europe/Samara" "サマラ"
    "Asia/Yekaterinburg" "エカテリンブルグ"
    "Asia/Omsk" "オムスク"
    "Asia/Novosibirsk" "ノヴォシビルスク"
    "Asia/Barnaul" "バルナウル"
    "Asia/Tomsk" "トムスク"
    "Asia/Novokuznetsk" "ノヴォクズネツク"
    "Asia/Krasnoyarsk" "クラスノヤルスク"
    "Asia/Irkutsk" "イルクーツク"
    "Asia/Chita" "チタ"
    "Asia/Yakutsk" "ヤクーツク"
    "Asia/Khandyga" "ハンドゥイガ"
    "Asia/Vladivostok" "ウラジオストク"
    "Asia/Ust-Nera" "ウスチネラ"
    "Asia/Magadan" "マガダン"
    "Asia/Sakhalin" "サハリン"
    "Asia/Srednekolymsk" "スレドネコリムスク"
    "Asia/Kamchatka" "カムチャッカ"
    "Asia/Anadyr" "アナディリ"
    "Asia/Riyadh" "リヤド"
    "Pacific/Guadalcanal" "ガダルカナル"
    "Africa/Khartoum" "ハルツーム"
    "Asia/Singapore" "シンガポール"
    "America/Paramaribo" "パラマリボ"
    "Africa/Juba" "ジュバ"
    "Africa/Sao_Tome" "サントメ"
    "America/El_Salvador" "エルサルバドル"
    "Asia/Damascus" "ダマスカス"
    "America/Grand_Turk" "グランドターク"
    "Africa/Ndjamena" "ンジャメナ"
    "Asia/Bangkok" "バンコク"
    "Asia/Dushanbe" "ドゥシャンベ"
    "Pacific/Fakaofo" "ファカオフォ"
    "Asia/Dili" "ディリ"
    "Asia/Ashgabat" "アシガバード"
    "Africa/Tunis" "チュニス"
    "Pacific/Tongatapu" "トンガタプ"
    "Europe/Istanbul" "イスタンブール"
    "Asia/Taipei" "台北"
    "Europe/Kyiv" "キーウ"
    "America/New_York" "ニューヨーク"
    "America/Detroit" "デトロイト"
    "America/Kentucky/Louisville" "ルイビル"
    "America/Kentucky/Monticello" "ケンタッキー州モンティチェロ"
    "America/Indiana/Indianapolis" "インディアナポリス"
    "America/Indiana/Vincennes" "インディアナ州ビンセンス"
    "America/Indiana/Winamac" "インディアナ州ウィナマック"
    "America/Indiana/Marengo" "インディアナ州マレンゴ"
    "America/Indiana/Petersburg" "インディアナ州ピーターズバーグ"
    "America/Indiana/Vevay" "インディアナ州ビベー"
    "America/Chicago" "シカゴ"
    "America/Indiana/Tell_City" "インディアナ州テルシティ"
    "America/Indiana/Knox" "インディアナ州ノックス"
    "America/Menominee" "メノミニー"
    "America/North_Dakota/Center" "ノースダコタ州センター"
    "America/North_Dakota/New_Salem" "ノースダコタ州ニューセーラム"
    "America/North_Dakota/Beulah" "ノースダコタ州ビューラー"
    "America/Denver" "デンバー"
    "America/Boise" "ボイシ"
    "America/Phoenix" "フェニックス"
    "America/Los_Angeles" "ロサンゼルス"
    "America/Anchorage" "アンカレッジ"
    "America/Juneau" "ジュノー"
    "America/Sitka" "シトカ"
    "America/Metlakatla" "メトラカトラ"
    "America/Yakutat" "ヤクタット"
    "America/Nome" "ノーム"
    "America/Adak" "アダック"
    "Pacific/Honolulu" "ホノルル"
    "America/Montevideo" "モンテビデオ"
    "Asia/Samarkand" "サマルカンド"
    "Asia/Tashkent" "タシケント"
    "America/Caracas" "カラカス"
    "Asia/Ho_Chi_Minh" "ホーチミン"
    "Pacific/Efate" "エフェテ島"
    "Pacific/Apia" "アピア"
    "Africa/Johannesburg" "ヨハネスブルグ"
    "America/Antigua" "アンティグア"
    "America/Anguilla" "アンギラ"
    "Africa/Luanda" "ルアンダ"
    "Antarctica/McMurdo" "マクマード基地"
    "Antarctica/DumontDUrville" "デュモン・デュルヴィル基地"
    "Antarctica/Syowa" "昭和基地"
    "America/Aruba" "アルバ"
    "Europe/Mariehamn" "マリエハムン"
    "Europe/Sarajevo" "サラエボ"
    "Africa/Ouagadougou" "ワガドゥグー"
    "Asia/Bahrain" "バーレーン"
    "Africa/Bujumbura" "ブジュンブラ"
    "Africa/Porto-Novo" "ポルトノボ"
    "America/St_Barthelemy" "サン・バルテルミー"
    "Asia/Brunei" "ブルネイ"
    "America/Kralendijk" "クラレンダイク"
    "America/Nassau" "ナッソー"
    "Africa/Gaborone" "ハボローネ"
    "America/Blanc-Sablon" "ブラン・サブロン"
    "America/Atikokan" "アティコカン"
    "America/Creston" "クレストン"
    "Indian/Cocos" "ココス諸島"
    "Africa/Kinshasa" "キンシャサ"
    "Africa/Lubumbashi" "ルブンバシ"
    "Africa/Bangui" "バンギ"
    "Africa/Brazzaville" "ブラザビル"
    "Africa/Douala" "ドゥアラ"
    "America/Curacao" "キュラソー"
    "Indian/Christmas" "クリスマス島"
    "Europe/Busingen" "ビュージンゲン"
    "Africa/Djibouti" "ジブチ"
    "Europe/Copenhagen" "コペンハーゲン"
    "America/Dominica" "ドミニカ"
    "Africa/Asmara" "アスマラ"
    "Africa/Addis_Ababa" "アジスアベバ"
    "Pacific/Chuuk" "チューク"
    "Pacific/Pohnpei" "ポンペイ島"
    "Africa/Libreville" "リーブルヴィル"
    "America/Grenada" "グレナダ"
    "Europe/Guernsey" "ガーンジー"
    "Africa/Accra" "アクラ"
    "Africa/Banjul" "バンジュール"
    "Africa/Conakry" "コナクリ"
    "America/Guadeloupe" "グアドループ"
    "Africa/Malabo" "マラボ"
    "Europe/Zagreb" "ザグレブ"
    "Europe/Isle_of_Man" "マン島"
    "Atlantic/Reykjavik" "レイキャビク"
    "Europe/Jersey" "ジャージー"
    "Asia/Phnom_Penh" "プノンペン"
    "Indian/Comoro" "コモロ"
    "America/St_Kitts" "セントクリストファー"
    "Asia/Kuwait" "クウェート"
    "America/Cayman" "ケイマン"
    "Asia/Vientiane" "ビエンチャン"
    "America/St_Lucia" "セントルシア"
    "Europe/Vaduz" "ファドゥーツ"
    "Africa/Maseru" "マセル"
    "Europe/Luxembourg" "ルクセンブルク"
    "Europe/Monaco" "モナコ"
    "Europe/Podgorica" "ポドゴリツァ"
    "America/Marigot" "マリゴ"
    "Indian/Antananarivo" "アンタナナリボ"
    "Pacific/Majuro" "マジュロ"
    "Europe/Skopje" "スコピエ"
    "Africa/Bamako" "バマコ"
    "Pacific/Saipan" "サイパン"
    "Africa/Nouakchott" "ヌアクショット"
    "America/Montserrat" "モントセラト"
    "Africa/Blantyre" "ブランタイヤ"
    "Asia/Kuala_Lumpur" "クアラルンプール"
    "Africa/Niamey" "ニアメ"
    "Europe/Amsterdam" "アムステルダム"
    "Europe/Oslo" "オスロ"
    "Asia/Muscat" "マスカット"
    "Indian/Reunion" "レユニオン"
    "Africa/Kigali" "キガリ"
    "Indian/Mahe" "マヘ"
    "Europe/Stockholm" "ストックホルム"
    "Atlantic/St_Helena" "セントヘレナ"
    "Europe/Ljubljana" "リュブリャナ"
    "Arctic/Longyearbyen" "ロングイェールビーン"
    "Europe/Bratislava" "ブラチスラバ"
    "Africa/Freetown" "フリータウン"
    "Europe/San_Marino" "サンマリノ"
    "Africa/Dakar" "ダカール"
    "Africa/Mogadishu" "モガディシオ"
    "America/Lower_Princes" "ローワー・プリンセズ・クウォーター"
    "Africa/Mbabane" "ムババーネ"
    "Indian/Kerguelen" "ケルゲレン諸島"
    "Africa/Lome" "ロメ"
    "America/Port_of_Spain" "ポートオブスペイン"
    "Pacific/Funafuti" "フナフティ"
    "Africa/Dar_es_Salaam" "ダルエスサラーム"
    "Africa/Kampala" "カンパラ"
    "Pacific/Midway" "ミッドウェー島"
    "Pacific/Wake" "ウェーク島"
    "Europe/Vatican" "バチカン"
    "America/St_Vincent" "セントビンセント"
    "America/Tortola" "トルトーラ"
    "America/St_Thomas" "セントトーマス"
    "Pacific/Wallis" "ウォリス諸島"
    "Asia/Aden" "アデン"
    "Indian/Mayotte" "マヨット"
    "Africa/Lusaka" "ルサカ"
    "Africa/Harare" "ハラレ"
};

// `common/main/jv.xml`: 72 of the 418 zones named, 344 inherited.
#[cfg(feature = "localized-exemplar-cities")]
const JV: &str = exemplar_cities! {
    "Europe/Andorra" inherited
    "Asia/Dubai" inherited
    "Asia/Kabul" inherited
    "Europe/Tirane" inherited
    "Asia/Yerevan" inherited
    "Antarctica/Casey" inherited
    "Antarctica/Davis" inherited
    "Antarctica/Mawson" inherited
    "Antarctica/Palmer" inherited
    "Antarctica/Rothera" inherited
    "Antarctica/Troll" inherited
    "Antarctica/Vostok" inherited
    "America/Argentina/Buenos_Aires" inherited
    "America/Argentina/Cordoba" "Kordoba"
    "America/Argentina/Salta" inherited
    "America/Argentina/Jujuy" inherited
    "America/Argentina/Tucuman" inherited
    "America/Argentina/Catamarca" "Katamarka"
    "America/Argentina/La_Rioja" inherited
    "America/Argentina/San_Juan" inherited
    "America/Argentina/Mendoza" "Mendosa"
    "America/Argentina/San_Luis" inherited
    "America/Argentina/Rio_Gallegos" inherited
    "America/Argentina/Ushuaia" inherited
    "Pacific/Pago_Pago" inherited
    "Europe/Vienna" inherited
    "Australia/Lord_Howe" inherited
    "Antarctica/Macquarie" inherited
    "Australia/Hobart" inherited
    "Australia/Melbourne" inherited
    "Australia/Sydney" inherited
    "Australia/Broken_Hill" inherited
    "Australia/Brisbane" inherited
    "Australia/Lindeman" inherited
    "Australia/Adelaide" inherited
    "Australia/Darwin" inherited
    "Australia/Perth" inherited
    "Australia/Eucla" inherited
    "Asia/Baku" inherited
    "America/Barbados" inherited
    "Asia/Dhaka" inherited
    "Europe/Brussels" inherited
    "Europe/Sofia" inherited
    "Atlantic/Bermuda" inherited
    "America/La_Paz" inherited
    "America/Noronha" inherited
    "America/Belem" inherited
    "America/Fortaleza" inherited
    "America/Recife" inherited
    "America/Araguaina" inherited
    "America/Maceio" inherited
    "America/Bahia" inherited
    "America/Sao_Paulo" inherited
    "America/Campo_Grande" "Kampo Grande"
    "America/Cuiaba" "Kuiaba"
    "America/Santarem" inherited
    "America/Porto_Velho" inherited
    "America/Boa_Vista" inherited
    "America/Manaus" inherited
    "America/Eirunepe" inherited
    "America/Rio_Branco" inherited
    "Asia/Thimphu" inherited
    "Europe/Minsk" inherited
    "America/Belize" "Belise"
    "America/St_Johns" "Santa John"
    "America/Halifax" "Halifak"
    "America/Glace_Bay" "Teluk Glace"
    "America/Moncton" inherited
    "America/Goose_Bay" "Teluk Goose"
    "America/Toronto" inherited
    "America/Iqaluit" inherited
    "America/Winnipeg" inherited
    "America/Resolute" inherited
    "America/Rankin_Inlet" inherited
    "America/Regina" inherited
    "America/Swift_Current" "Arus Banter"
    "America/Edmonton" inherited
    "America/Cambridge_Bay" "Teluk Cambridge"
    "America/Inuvik" inherited
    "America/Vancouver" inherited
    "America/Dawson_Creek" inherited
    "America/Fort_Nelson" "Benteng Nelson"
    "America/Whitehorse" inherited
    "America/Dawson" inherited
    "Europe/Zurich" inherited
    "Africa/Abidjan" inherited
    "Pacific/Rarotonga" inherited
    "America/Santiago" inherited
    "America/Coyhaique" ""
    "America/Punta_Arenas" inherited
    "Pacific/Easter" "Paskah"
    "Asia/Shanghai" inherited
    "Asia/Urumqi" inherited
    "America/Bogota" inherited
    "America/Costa_Rica" "Kosta Rika"
    "America/Havana" inherited
    "Atlantic/Cape_Verde" "Kape Verde"
    "Asia/Nicosia" inherited
    "Asia/Famagusta" inherited
    "Europe/Prague" inherited
    "Europe/Berlin" inherited
    "America/Santo_Domingo" inherited
    "Africa/Algiers" inherited
    "America/Guayaquil" inherited
    "Pacific/Galapagos" inherited
    "Europe/Tallinn" inherited
    "Africa/Cairo" "Kairo"
    "Africa/El_Aaiun" inherited
    "Europe/Madrid" inherited
    "Africa/Ceuta" inherited
    "Atlantic/Canary" "Kanari"
    "Europe/Helsinki" inherited
    "Pacific/Fiji" inherited
    "Atlantic/Stanley" inherited
    "Pacific/Kosrae" inherited
    "Atlantic/Faroe" inherited
    "Europe/Paris" inherited
    "Europe/London" inherited
    "Asia/Tbilisi" inherited
    "America/Cayenne" "Kayenne"
    "Europe/Gibraltar" inherited
    "America/Nuuk" inherited
    "America/Danmarkshavn" inherited
    "America/Scoresbysund" inherited
    "America/Thule" inherited
    "Europe/Athens" "Athena"
    "Atlantic/South_Georgia" "Georgia Kidul"
    "America/Guatemala" inherited
    "Pacific/Guam" inherited
    "Africa/Bissau" inherited
    "America/Guyana" inherited
    "Asia/Hong_Kong" inherited
    "America/Tegucigalpa" inherited
    "America/Port-au-Prince" inherited
    "Europe/Budapest" inherited
    "Asia/Jakarta" inherited
    "Asia/Pontianak" inherited
    "Asia/Makassar" "Makasar"
    "Asia/Jayapura" inherited
    "Europe/Dublin" inherited
    "Asia/Jerusalem" "Yerusalem"
    "Asia/Kolkata" "Kalkuta"
    "Indian/Chagos" "Khagos"
    "Asia/Baghdad" inherited
    "Asia/Tehran" "Teheran"
    "Europe/Rome" "Roma"
    "America/Jamaica" inherited
    "Asia/Amman" inherited
    "Asia/Tokyo" inherited
    "Africa/Nairobi" inherited
    "Asia/Bishkek" inherited
    "Pacific/Tarawa" inherited
    "Pacific/Kanton" inherited
    "Pacific/Kiritimati" inherited
    "Asia/Pyongyang" inherited
    "Asia/Seoul" inherited
    "Asia/Almaty" inherited
    "Asia/Qyzylorda" inherited
    "Asia/Qostanay" "Kostanai"
    "Asia/Aqtobe" inherited
    "Asia/Aqtau" inherited
    "Asia/Atyrau" inherited
    "Asia/Oral" inherited
    "Asia/Beirut" inherited
    "Asia/Colombo" "Kolombo"
    "Africa/Monrovia" inherited
    "Europe/Vilnius" inherited
    "Europe/Riga" inherited
    "Africa/Tripoli" inherited
    "Africa/Casablanca" "Kasablanka"
    "Europe/Chisinau" inherited
    "Pacific/Kwajalein" inherited
    "Asia/Yangon" inherited
    "Asia/Ulaanbaatar" inherited
    "Asia/Hovd" inherited
    "Asia/Macau" "Macau"
    "America/Martinique" "Martinik"
    "Europe/Malta" inherited
    "Indian/Mauritius" inherited
    "Indian/Maldives" "Maladewa"
    "America/Mexico_City" "Kutho Meksiko"
    "America/Cancun" "Cancun"
    "America/Merida" "Merida"
    "America/Monterrey" inherited
    "America/Matamoros" inherited
    "America/Chihuahua" inherited
    "America/Ciudad_Juarez" "Ciudad Juáres"
    "America/Ojinaga" inherited
    "America/Mazatlan" inherited
    "America/Bahia_Banderas" "Bahia Banderas"
    "America/Hermosillo" inherited
    "America/Tijuana" inherited
    "Asia/Kuching" inherited
    "Africa/Maputo" inherited
    "Africa/Windhoek" inherited
    "Pacific/Noumea" inherited
    "Pacific/Norfolk" inherited
    "Africa/Lagos" inherited
    "America/Managua" inherited
    "Asia/Kathmandu" inherited
    "Pacific/Nauru" inherited
    "Pacific/Niue" inherited
    "Pacific/Auckland" inherited
    "Pacific/Chatham" inherited
    "America/Panama" inherited
    "America/Lima" inherited
    "Pacific/Tahiti" inherited
    "Pacific/Marquesas" inherited
    "Pacific/Gambier" inherited
    "Pacific/Port_Moresby" "Pelabuhan Moresby"
    "Pacific/Bougainville" inherited
    "Asia/Manila" inherited
    "Asia/Karachi" inherited
    "Europe/Warsaw" inherited
    "America/Miquelon" inherited
    "Pacific/Pitcairn" inherited
    "America/Puerto_Rico" "Puerto Riko"
    "Asia/Gaza" inherited
    "Asia/Hebron" inherited
    "Europe/Lisbon" inherited
    "Atlantic/Madeira" inherited
    "Atlantic/Azores" inherited
    "Pacific/Palau" inherited
    "America/Asuncion" inherited
    "Asia/Qatar" inherited
    "Europe/Bucharest" inherited
    "Europe/Belgrade" inherited
    "Europe/Kaliningrad" inherited
    "Europe/Moscow" inherited
    "Europe/Simferopol" inherited
    "Europe/Kirov" inherited
    "Europe/Volgograd" inherited
    "Europe/Astrakhan" inherited
    "Europe/Saratov" inherited
    "Europe/Ulyanovsk" inherited
    "Europe/Samara" inherited
    "Asia/Yekaterinburg" inherited
    "Asia/Omsk" inherited
    "Asia/Novosibirsk" inherited
    "Asia/Barnaul" inherited
    "Asia/Tomsk" inherited
    "Asia/Novokuznetsk" inherited
    "Asia/Krasnoyarsk" inherited
    "Asia/Irkutsk" inherited
    "Asia/Chita" inherited
    "Asia/Yakutsk" inherited
    "Asia/Khandyga" inherited
    "Asia/Vladivostok" inherited
    "Asia/Ust-Nera" inherited
    "Asia/Magadan" inherited
    "Asia/Sakhalin" inherited
    "Asia/Srednekolymsk" inherited
    "Asia/Kamchatka" inherited
    "Asia/Anadyr" inherited
    "Asia/Riyadh" inherited
    "Pacific/Guadalcanal" inherited
    "Africa/Khartoum" inherited
    "Asia/Singapore" "Singapura"
    "America/Paramaribo" inherited
    "Africa/Juba" inherited
    "Africa/Sao_Tome" inherited
    "America/El_Salvador" inherited
    "Asia/Damascus" "Damaskus"
    "America/Grand_Turk" inherited
    "Africa/Ndjamena" inherited
    "Asia/Bangkok" inherited
    "Asia/Dushanbe" inherited
    "Pacific/Fakaofo" inherited
    "Asia/Dili" inherited
    "Asia/Ashgabat" inherited
    "Africa/Tunis" inherited
    "Pacific/Tongatapu" inherited
    "Europe/Istanbul" inherited
    "Asia/Taipei" inherited
    "Europe/Kyiv" "Kiev"
    "America/New_York" inherited
    "America/Detroit" inherited
    "America/Kentucky/Louisville" inherited
    "America/Kentucky/Monticello" "Monticello [Kentucky]"
    "America/Indiana/Indianapolis" inherited
    "America/Indiana/Vincennes" "Vincennes [Indiana]"
    "America/Indiana/Winamac" "Winamac [Indiana]"
    "America/Indiana/Marengo" "Marengo [Indiana]"
    "America/Indiana/Petersburg" "Petersburg [Indiana]"
    "America/Indiana/Vevay" "Vevay [Indiana]"
    "America/Chicago" inherited
    "America/Indiana/Tell_City" "Tell City [Indiana]"
    "America/Indiana/Knox" "Knox [Indiana]"
    "America/Menominee" inherited
    "America/North_Dakota/Center" "Tengah [Dakota Lor]"
    "America/North_Dakota/New_Salem" "Salem Anyar [Dakota Lor]"
    "America/North_Dakota/Beulah" "Beulah [Dakota Lor]"
    "America/Denver" inherited
    "America/Boise" inherited
    "America/Phoenix" inherited
    "America/Los_Angeles" inherited
    "America/Anchorage" inherited
    "America/Juneau" inherited
    "America/Sitka" inherited
    "America/Metlakatla" inherited
    "America/Yakutat" inherited
    "America/Nome" inherited
    "America/Adak" inherited
    "Pacific/Honolulu" ""
    "America/Montevideo" inherited
    "Asia/Samarkand" inherited
    "Asia/Tashkent" inherited
    "America/Caracas" "Karakas"
    "Asia/Ho_Chi_Minh" inherited
    "Pacific/Efate" inherited
    "Pacific/Apia" inherited
    "Africa/Johannesburg" inherited
    "America/Antigua" inherited
    "America/Anguilla" inherited
    "Africa/Luanda" inherited
    "Antarctica/McMurdo" inherited
    "Antarctica/DumontDUrville" inherited
    "Antarctica/Syowa" inherited
    "America/Aruba" inherited
    "Europe/Mariehamn" inherited
    "Europe/Sarajevo" inherited
    "Africa/Ouagadougou" inherited
    "Asia/Bahrain" inherited
    "Africa/Bujumbura" inherited
    "Africa/Porto-Novo" inherited
    "America/St_Barthelemy" "Santa Barthelemy"
    "Asia/Brunei" inherited
    "America/Kralendijk" inherited
    "America/Nassau" inherited
    "Africa/Gaborone" inherited
    "America/Blanc-Sablon" inherited
    "America/Atikokan" inherited
    "America/Creston" inherited
    "Indian/Cocos" inherited
    "Africa/Kinshasa" inherited
    "Africa/Lubumbashi" inherited
    "Africa/Bangui" inherited
    "Africa/Brazzaville" inherited
    "Africa/Douala" inherited
    "America/Curacao" inherited
    "Indian/Christmas" "Natal"
    "Europe/Busingen" inherited
    "Africa/Djibouti" inherited
    "Europe/Copenhagen" "Kopenhagen"
    "America/Dominica" "Dominika"
    "Africa/Asmara" inherited
    "Africa/Addis_Ababa" inherited
    "Pacific/Chuuk" inherited
    "Pacific/Pohnpei" inherited
    "Africa/Libreville" inherited
    "America/Grenada" inherited
    "Europe/Guernsey" inherited
    "Africa/Accra" inherited
    "Africa/Banjul" inherited
    "Africa/Conakry" "Konakri"
    "America/Guadeloupe" inherited
    "Africa/Malabo" inherited
    "Europe/Zagreb" inherited
    "Europe/Isle_of_Man" "Pulo Man"
    "Atlantic/Reykjavik" inherited
    "Europe/Jersey" inherited
    "Asia/Phnom_Penh" inherited
    "Indian/Comoro" "Komoro"
    "America/St_Kitts" "Santa Kitts"
    "Asia/Kuwait" inherited
    "America/Cayman" "Caiman"
    "Asia/Vientiane" inherited
    "America/St_Lucia" "Santa Lucia"
    "Europe/Vaduz" inherited
    "Africa/Maseru" inherited
    "Europe/Luxembourg" "Luksemburk"
    "Europe/Monaco" "Monako"
    "Europe/Podgorica" inherited
    "America/Marigot" inherited
    "Indian/Antananarivo" inherited
    "Pacific/Majuro" inherited
    "Europe/Skopje" inherited
    "Africa/Bamako" inherited
    "Pacific/Saipan" inherited
    "Africa/Nouakchott" inherited
    "America/Montserrat" inherited
    "Africa/Blantyre" inherited
    "Asia/Kuala_Lumpur" inherited
    "Africa/Niamey" inherited
    "Europe/Amsterdam" inherited
    "Europe/Oslo" inherited
    "Asia/Muscat" inherited
    "Indian/Reunion" inherited
    "Africa/Kigali" inherited
    "Indian/Mahe" inherited
    "Europe/Stockholm" inherited
    "Atlantic/St_Helena" "Santa Helena"
    "Europe/Ljubljana" inherited
    "Arctic/Longyearbyen" inherited
    "Europe/Bratislava" inherited
    "Africa/Freetown" inherited
    "Europe/San_Marino" inherited
    "Africa/Dakar" inherited
    "Africa/Mogadishu" inherited
    "America/Lower_Princes" inherited
    "Africa/Mbabane" inherited
    "Indian/Kerguelen" inherited
    "Africa/Lome" inherited
    "America/Port_of_Spain" "Palabuhan Spanyol"
    "Pacific/Funafuti" inherited
    "Africa/Dar_es_Salaam" inherited
    "Africa/Kampala" inherited
    "Pacific/Midway" inherited
    "Pacific/Wake" inherited
    "Europe/Vatican" "Vatikan"
    "America/St_Vincent" "Santa Vincent"
    "America/Tortola" inherited
    "America/St_Thomas" "Santa Thomas"
    "Pacific/Wallis" inherited
    "Asia/Aden" inherited
    "Indian/Mayotte" inherited
    "Africa/Lusaka" inherited
    "Africa/Harare" inherited
};

// `common/main/ko.xml`: 418 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const KO: &str = exemplar_cities! {
    "Europe/Andorra" "안도라"
    "Asia/Dubai" "두바이"
    "Asia/Kabul" "카불"
    "Europe/Tirane" "티라나"
    "Asia/Yerevan" "예레반"
    "Antarctica/Casey" "케이시"
    "Antarctica/Davis" "데이비스"
    "Antarctica/Mawson" "모슨"
    "Antarctica/Palmer" "파머"
    "Antarctica/Rothera" "로데라"
    "Antarctica/Troll" "트롤"
    "Antarctica/Vostok" "보스토크"
    "America/Argentina/Buenos_Aires" "부에노스 아이레스"
    "America/Argentina/Cordoba" "코르도바"
    "America/Argentina/Salta" "살타"
    "America/Argentina/Jujuy" "후후이"
    "America/Argentina/Tucuman" "투쿠만"
    "America/Argentina/Catamarca" "카타마르카"
    "America/Argentina/La_Rioja" "라 리오하"
    "America/Argentina/San_Juan" "산후안"
    "America/Argentina/Mendoza" "멘도사"
    "America/Argentina/San_Luis" "산루이스"
    "America/Argentina/Rio_Gallegos" "리오 가예고스"
    "America/Argentina/Ushuaia" "우수아이아"
    "Pacific/Pago_Pago" "파고파고"
    "Europe/Vienna" "비엔나"
    "Australia/Lord_Howe" "로드 하우"
    "Antarctica/Macquarie" "맥쿼리"
    "Australia/Hobart" "호바트"
    "Australia/Melbourne" "멜버른"
    "Australia/Sydney" "시드니"
    "Australia/Broken_Hill" "브로컨힐"
    "Australia/Brisbane" "브리스베인"
    "Australia/Lindeman" "린데만"
    "Australia/Adelaide" "애들레이드"
    "Australia/Darwin" "다윈"
    "Australia/Perth" "퍼스"
    "Australia/Eucla" "유클라"
    "Asia/Baku" "바쿠"
    "America/Barbados" "바베이도스"
    "Asia/Dhaka" "다카"
    "Europe/Brussels" "브뤼셀"
    "Europe/Sofia" "소피아"
    "Atlantic/Bermuda" "버뮤다"
    "America/La_Paz" "라파스"
    "America/Noronha" "노롱야"
    "America/Belem" "벨렘"
    "America/Fortaleza" "포르탈레자"
    "America/Recife" "레시페"
    "America/Araguaina" "아라과이나"
    "America/Maceio" "마세이오"
    "America/Bahia" "바히아"
    "America/Sao_Paulo" "상파울루"
    "America/Campo_Grande" "캄포 그란데"
    "America/Cuiaba" "쿠이아바"
    "America/Santarem" "산타렘"
    "America/Porto_Velho" "포르토벨료"
    "America/Boa_Vista" "보아 비스타"
    "America/Manaus" "마나우스"
    "America/Eirunepe" "아이루네페"
    "America/Rio_Branco" "히우 브랑쿠"
    "Asia/Thimphu" "팀부"
    "Europe/Minsk" "민스크"
    "America/Belize" "벨리즈"
    "America/St_Johns" "세인트존스"
    "America/Halifax" "핼리팩스"
    "America/Glace_Bay" "글라스베이"
    "America/Moncton" "몽턴"
    "America/Goose_Bay" "구즈베이"
    "America/Toronto" "토론토"
    "America/Iqaluit" "이칼루이트"
    "America/Winnipeg" "위니펙"
    "America/Resolute" "리졸루트"
    "America/Rankin_Inlet" "랭킹 인렛"
    "America/Regina" "리자이나"
    "America/Swift_Current" "스위프트커런트"
    "America/Edmonton" "에드먼턴"
    "America/Cambridge_Bay" "케임브리지 베이"
    "America/Inuvik" "이누빅"
    "America/Vancouver" "벤쿠버"
    "America/Dawson_Creek" "도슨크릭"
    "America/Fort_Nelson" "포트 넬슨"
    "America/Whitehorse" "화이트호스"
    "America/Dawson" "도슨"
    "Europe/Zurich" "취리히"
    "Africa/Abidjan" "아비장"
    "Pacific/Rarotonga" "라로통가"
    "America/Santiago" "산티아고"
    "America/Coyhaique" "코이아이케"
    "America/Punta_Arenas" "푼타아레나스"
    "Pacific/Easter" "이스터 섬"
    "Asia/Shanghai" "상하이"
    "Asia/Urumqi" "우루무치"
    "America/Bogota" "보고타"
    "America/Costa_Rica" "코스타리카"
    "America/Havana" "하바나"
    "Atlantic/Cape_Verde" "카보 베르데"
    "Asia/Nicosia" "니코시아"
    "Asia/Famagusta" "파마구스타"
    "Europe/Prague" "프라하"
    "Europe/Berlin" "베를린"
    "America/Santo_Domingo" "산토도밍고"
    "Africa/Algiers" "알제"
    "America/Guayaquil" "과야킬"
    "Pacific/Galapagos" "갈라파고스"
    "Europe/Tallinn" "탈린"
    "Africa/Cairo" "카이로"
    "Africa/El_Aaiun" "엘아이운"
    "Europe/Madrid" "마드리드"
    "Africa/Ceuta" "세우타"
    "Atlantic/Canary" "카나리아 제도"
    "Europe/Helsinki" "헬싱키"
    "Pacific/Fiji" "피지"
    "Atlantic/Stanley" "스탠리"
    "Pacific/Kosrae" "코스레"
    "Atlantic/Faroe" "페로 제도"
    "Europe/Paris" "파리"
    "Europe/London" "런던"
    "Asia/Tbilisi" "트빌리시"
    "America/Cayenne" "카옌"
    "Europe/Gibraltar" "지브롤터"
    "America/Nuuk" "고드호프"
    "America/Danmarkshavn" "덴마크샤븐"
    "America/Scoresbysund" "스코레스바이선드"
    "America/Thule" "툴레"
    "Europe/Athens" "아테네"
    "Atlantic/South_Georgia" "사우스조지아"
    "America/Guatemala" "과테말라"
    "Pacific/Guam" "괌"
    "Africa/Bissau" "비사우"
    "America/Guyana" "가이아나"
    "Asia/Hong_Kong" "홍콩"
    "America/Tegucigalpa" "테구시갈파"
    "America/Port-au-Prince" "포르토프랭스"
    "Europe/Budapest" "부다페스트"
    "Asia/Jakarta" "자카르타"
    "Asia/Pontianak" "폰티아나크"
    "Asia/Makassar" "마카사르"
    "Asia/Jayapura" "자야푸라"
    "Europe/Dublin" "더블린"
    "Asia/Jerusalem" "예루살렘"
    "Asia/Kolkata" "콜카타"
    "Indian/Chagos" "차고스"
    "Asia/Baghdad" "바그다드"
    "Asia/Tehran" "테헤란"
    "Europe/Rome" "로마"
    "America/Jamaica" "자메이카"
    "Asia/Amman" "암만"
    "Asia/Tokyo" "도쿄"
    "Africa/Nairobi" "나이로비"
    "Asia/Bishkek" "비슈케크"
    "Pacific/Tarawa" "타라와"
    "Pacific/Kanton" "칸톤"
    "Pacific/Kiritimati" "키리티마티"
    "Asia/Pyongyang" "평양"
    "Asia/Seoul" "서울"
    "Asia/Almaty" "알마티"
    "Asia/Qyzylorda" "키질로르다"
    "Asia/Qostanay" "코스타나이"
    "Asia/Aqtobe" "악토브"
    "Asia/Aqtau" "아크타우"
    "Asia/Atyrau" "아티라우"
    "Asia/Oral" "오랄"
    "Asia/Beirut" "베이루트"
    "Asia/Colombo" "콜롬보"
    "Africa/Monrovia" "몬로비아"
    "Europe/Vilnius" "빌니우스"
    "Europe/Riga" "리가"
    "Africa/Tripoli" "트리폴리"
    "Africa/Casablanca" "카사블랑카"
    "Europe/Chisinau" "키시나우"
    "Pacific/Kwajalein" "콰잘렌"
    "Asia/Yangon" "랑군"
    "Asia/Ulaanbaatar" "울란바토르"
    "Asia/Hovd" "호브드"
    "Asia/Macau" "마카오"
    "America/Martinique" "마티니크"
    "Europe/Malta" "몰타"
    "Indian/Mauritius" "모리셔스"
    "Indian/Maldives" "몰디브"
    "America/Mexico_City" "멕시코 시티"
    "America/Cancun" "칸쿤"
    "America/Merida" "메리다"
    "America/Monterrey" "몬테레이"
    "America/Matamoros" "마타모로스"
    "America/Chihuahua" "치와와"
    "America/Ciudad_Juarez" "시우다드후아레스"
    "America/Ojinaga" "오히나가"
    "America/Mazatlan" "마사틀란"
    "America/Bahia_Banderas" "바이아 반데라스"
    "America/Hermosillo" "에르모시요"
    "America/Tijuana" "티후아나"
    "Asia/Kuching" "쿠칭"
    "Africa/Maputo" "마푸토"
    "Africa/Windhoek" "빈트후크"
    "Pacific/Noumea" "누메아"
    "Pacific/Norfolk" "노퍽"
    "Africa/Lagos" "라고스"
    "America/Managua" "마나과"
    "Asia/Kathmandu" "카트만두"
    "Pacific/Nauru" "나우루"
    "Pacific/Niue" "니우에"
    "Pacific/Auckland" "오클랜드"
    "Pacific/Chatham" "채텀"
    "America/Panama" "파나마"
    "America/Lima" "리마"
    "Pacific/Tahiti" "타히티"
    "Pacific/Marquesas" "마퀘사스"
    "Pacific/Gambier" "감비어"
    "Pacific/Port_Moresby" "포트모르즈비"
    "Pacific/Bougainville" "부갱빌"
    "Asia/Manila" "마닐라"
    "Asia/Karachi" "카라치"
    "Europe/Warsaw" "바르샤바"
    "America/Miquelon" "미클롱"
    "Pacific/Pitcairn" "핏케언"
    "America/Puerto_Rico" "푸에르토리코"
    "Asia/Gaza" "가자"
    "Asia/Hebron" "헤브론"
    "Europe/Lisbon" "리스본"
    "Atlantic/Madeira" "마데이라"
    "Atlantic/Azores" "아조레스"
    "Pacific/Palau" "팔라우"
    "America/Asuncion" "아순시온"
    "Asia/Qatar" "카타르"
    "Europe/Bucharest" "부쿠레슈티"
    "Europe/Belgrade" "베오그라드"
    "Europe/Kaliningrad" "칼리닌그라드"
    "Europe/Moscow" "모스크바"
    "Europe/Simferopol" "심페로폴"
    "Europe/Kirov" "키로프"
    "Europe/Volgograd" "볼고그라트"
    "Europe/Astrakhan" "아스트라한"
    "Europe/Saratov" "사라토프"
    "Europe/Ulyanovsk" "울리야노프스크"
    "Europe/Samara" "사마라"
    "Asia/Yekaterinburg" "예카테린부르크"
    "Asia/Omsk" "옴스크"
    "Asia/Novosibirsk" "노보시비르스크"
    "Asia/Barnaul" "바르나울"
    "Asia/Tomsk" "톰스크"
    "Asia/Novokuznetsk" "노보쿠즈네츠크"
    "Asia/Krasnoyarsk" "크라스노야르스크"
    "Asia/Irkutsk" "이르쿠츠크"
    "Asia/Chita" "치타"
    "Asia/Yakutsk" "야쿠츠크"
    "Asia/Khandyga" "한디가"
    "Asia/Vladivostok" "블라디보스토크"
    "Asia/Ust-Nera" "우스티네라"
    "Asia/Magadan" "마가단"
    "Asia/Sakhalin" "사할린"
    "Asia/Srednekolymsk" "스레드네콜림스크"
    "Asia/Kamchatka" "캄차카"
    "Asia/Anadyr" "아나디리"
    "Asia/Riyadh" "리야드"
    "Pacific/Guadalcanal" "과달카날"
    "Africa/Khartoum" "카르툼"
    "Asia/Singapore" "싱가포르"
    "America/Paramaribo" "파라마리보"
    "Africa/Juba" "주바"
    "Africa/Sao_Tome" "상투메"
    "America/El_Salvador" "엘살바도르"
    "Asia/Damascus" "다마스쿠스"
    "America/Grand_Turk" "그랜드 터크"
    "Africa/Ndjamena" "엔자메나"
    "Asia/Bangkok" "방콕"
    "Asia/Dushanbe" "두샨베"
    "Pacific/Fakaofo" "파카오푸"
    "Asia/Dili" "딜리"
    "Asia/Ashgabat" "아슈하바트"
    "Africa/Tunis" "튀니스"
    "Pacific/Tongatapu" "통가타푸"
    "Europe/Istanbul" "이스탄불"
    "Asia/Taipei" "타이베이"
    "Europe/Kyiv" "키예프"
    "America/New_York" "뉴욕"
    "America/Detroit" "디트로이트"
    "America/Kentucky/Louisville" "루이빌"
    "America/Kentucky/Monticello" "켄터키주, 몬티첼로"
    "America/Indiana/Indianapolis" "인디애나폴리스"
    "America/Indiana/Vincennes" "인디애나주, 빈센스"
    "America/Indiana/Winamac" "인디애나주, 위너맥"
    "America/Indiana/Marengo" "인디애나주, 머렝고"
    "America/Indiana/Petersburg" "인디애나주, 피츠버그"
    "America/Indiana/Vevay" "인디애나주, 비비"
    "America/Chicago" "시카고"
    "America/Indiana/Tell_City" "인디애나주, 텔시티"
    "America/Indiana/Knox" "인디애나주, 녹스"
    "America/Menominee" "메노미니"
    "America/North_Dakota/Center" "중부, 노스다코타"
    "America/North_Dakota/New_Salem" "노스다코타주, 뉴살렘"
    "America/North_Dakota/Beulah" "노스다코타주, 베라"
    "America/Denver" "덴버"
    "America/Boise" "보이시"
    "America/Phoenix" "피닉스"
    "America/Los_Angeles" "로스앤젤레스"
    "America/Anchorage" "앵커리지"
    "America/Juneau" "주노"
    "America/Sitka" "싯카"
    "America/Metlakatla" "메틀라카틀라"
    "America/Yakutat" "야쿠타트"
    "America/Nome" "놈"
    "America/Adak" "에이닥"
    "Pacific/Honolulu" "호놀룰루"
    "America/Montevideo" "몬테비데오"
    "Asia/Samarkand" "사마르칸트"
    "Asia/Tashkent" "타슈켄트"
    "America/Caracas" "카라카스"
    "Asia/Ho_Chi_Minh" "사이공"
    "Pacific/Efate" "에파테"
    "Pacific/Apia" "아피아"
    "Africa/Johannesburg" "요하네스버그"
    "America/Antigua" "안티과"
    "America/Anguilla" "앙귈라"
    "Africa/Luanda" "루안다"
    "Antarctica/McMurdo" "맥머도"
    "Antarctica/DumontDUrville" "뒤몽 뒤르빌"
    "Antarctica/Syowa" "쇼와"
    "America/Aruba" "아루바"
    "Europe/Mariehamn" "마리에함"
    "Europe/Sarajevo" "사라예보"
    "Africa/Ouagadougou" "와가두구"
    "Asia/Bahrain" "바레인"
    "Africa/Bujumbura" "부줌부라"
    "Africa/Porto-Novo" "포르토노보"
    "America/St_Barthelemy" "생바르텔레미"
    "Asia/Brunei" "브루나이"
    "America/Kralendijk" "크라렌디즈크"
    "America/Nassau" "나소"
    "Africa/Gaborone" "가보로네"
    "America/Blanc-Sablon" "블랑 사블롱"
    "America/Atikokan" "코랄하버"
    "America/Creston" "크레스톤"
    "Indian/Cocos" "코코스"
    "Africa/Kinshasa" "킨샤사"
    "Africa/Lubumbashi" "루붐바시"
    "Africa/Bangui" "방기"
    "Africa/Brazzaville" "브라자빌"
    "Africa/Douala" "두알라"
    "America/Curacao" "퀴라소"
    "Indian/Christmas" "크리스마스"
    "Europe/Busingen" "뷔지겐"
    "Africa/Djibouti" "지부티"
    "Europe/Copenhagen" "코펜하겐"
    "America/Dominica" "도미니카"
    "Africa/Asmara" "아스메라"
    "Africa/Addis_Ababa" "아디스아바바"
    "Pacific/Chuuk" "트루크"
    "Pacific/Pohnpei" "포나페"
    "Africa/Libreville" "리브르빌"
    "America/Grenada" "그레나다"
    "Europe/Guernsey" "건지"
    "Africa/Accra" "아크라"
    "Africa/Banjul" "반줄"
    "Africa/Conakry" "코나크리"
    "America/Guadeloupe" "과들루프"
    "Africa/Malabo" "말라보"
    "Europe/Zagreb" "자그레브"
    "Europe/Isle_of_Man" "맨섬"
    "Atlantic/Reykjavik" "레이캬비크"
    "Europe/Jersey" "저지"
    "Asia/Phnom_Penh" "프놈펜"
    "Indian/Comoro" "코모로"
    "America/St_Kitts" "세인트키츠"
    "Asia/Kuwait" "쿠웨이트"
    "America/Cayman" "케이맨"
    "Asia/Vientiane" "비엔티안"
    "America/St_Lucia" "세인트루시아"
    "Europe/Vaduz" "파두츠"
    "Africa/Maseru" "마세루"
    "Europe/Luxembourg" "룩셈부르크"
    "Europe/Monaco" "모나코"
    "Europe/Podgorica" "포드고리차"
    "America/Marigot" "마리곳"
    "Indian/Antananarivo" "안타나나리보"
    "Pacific/Majuro" "마주로"
    "Europe/Skopje" "스코페"
    "Africa/Bamako" "바마코"
    "Pacific/Saipan" "사이판"
    "Africa/Nouakchott" "누악쇼트"
    "America/Montserrat" "몬세라트"
    "Africa/Blantyre" "블랜타이어"
    "Asia/Kuala_Lumpur" "쿠알라룸푸르"
    "Africa/Niamey" "니아메"
    "Europe/Amsterdam" "암스테르담"
    "Europe/Oslo" "오슬로"
    "Asia/Muscat" "무스카트"
    "Indian/Reunion" "레위니옹"
    "Africa/Kigali" "키갈리"
    "Indian/Mahe" "마헤"
    "Europe/Stockholm" "스톡홀름"
    "Atlantic/St_Helena" "세인트 헬레나"
    "Europe/Ljubljana" "류블랴나"
    "Arctic/Longyearbyen" "롱이어비엔"
    "Europe/Bratislava" "브라티슬라바"
    "Africa/Freetown" "프리타운"
    "Europe/San_Marino" "산마리노"
    "Africa/Dakar" "다카르"
    "Africa/Mogadishu" "모가디슈"
    "America/Lower_Princes" "로워 프린스 쿼터"
    "Africa/Mbabane" "음바바네"
    "Indian/Kerguelen" "케르켈렌"
    "Africa/Lome" "로메"
    "America/Port_of_Spain" "포트오브스페인"
    "Pacific/Funafuti" "푸나푸티"
    "Africa/Dar_es_Salaam" "다르에스살람"
    "Africa/Kampala" "캄팔라"
    "Pacific/Midway" "미드웨이"
    "Pacific/Wake" "웨이크"
    "Europe/Vatican" "바티칸"
    "America/St_Vincent" "세인트빈센트"
    "America/Tortola" "토르톨라"
    "America/St_Thomas" "세인트토마스"
    "Pacific/Wallis" "월리스"
    "Asia/Aden" "아덴"
    "Indian/Mayotte" "메요트"
    "Africa/Lusaka" "루사카"
    "Africa/Harare" "하라레"
};

// `common/main/ml.xml`: 418 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const ML: &str = exemplar_cities! {
    "Europe/Andorra" "അണ്ടോറ"
    "Asia/Dubai" "ദുബായ്"
    "Asia/Kabul" "കാബൂൾ"
    "Europe/Tirane" "ടിരാനെ"
    "Asia/Yerevan" "യേരവൻ‌"
    "Antarctica/Casey" "കാസെ"
    "Antarctica/Davis" "ഡെയ്‌വിസ്"
    "Antarctica/Mawson" "മാവ്സൺ"
    "Antarctica/Palmer" "പാമർ"
    "Antarctica/Rothera" "റൊതീറ"
    "Antarctica/Troll" "ട്രോൾ"
    "Antarctica/Vostok" "വോസ്റ്റോക്"
    "America/Argentina/Buenos_Aires" "ബ്യൂണസ് ഐറിസ്"
    "America/Argentina/Cordoba" "കോർഡോബ"
    "America/Argentina/Salta" "സാൽട്ട"
    "America/Argentina/Jujuy" "ജുജുയ്"
    "America/Argentina/Tucuman" "റ്റുകുമാൻ"
    "America/Argentina/Catamarca" "‍ക്യാറ്റമാർക്ക"
    "America/Argentina/La_Rioja" "ലാ റിയോജ"
    "America/Argentina/San_Juan" "സാൻ ജുവാൻ"
    "America/Argentina/Mendoza" "മെൻഡോസ"
    "America/Argentina/San_Luis" "സാൻ ലൂയിസ്"
    "America/Argentina/Rio_Gallegos" "റിയോ ഗ്യാലഗോസ്"
    "America/Argentina/Ushuaia" "ഉഷിയ"
    "Pacific/Pago_Pago" "പാഗോ പാഗോ"
    "Europe/Vienna" "വിയന്ന"
    "Australia/Lord_Howe" "ലോഡ് ഹോവ്"
    "Antarctica/Macquarie" "മക്വയറി"
    "Australia/Hobart" "ഹൊബാർട്ട്"
    "Australia/Melbourne" "മെൽബൺ"
    "Australia/Sydney" "സിഡ്നി"
    "Australia/Broken_Hill" "ബ്രോക്കൺ ഹിൽ"
    "Australia/Brisbane" "ബ്രിസ്‌ബെയിൻ"
    "Australia/Lindeman" "ലിൻഡെമാൻ"
    "Australia/Adelaide" "അഡിലെയ്‌ഡ്"
    "Australia/Darwin" "ഡാർവിൻ"
    "Australia/Perth" "പെർത്ത്"
    "Australia/Eucla" "യൂക്ല"
    "Asia/Baku" "ബാക്കു"
    "America/Barbados" "ബാർബഡോസ്"
    "Asia/Dhaka" "ധാക്ക"
    "Europe/Brussels" "ബ്രസ്സൽ‌സ്"
    "Europe/Sofia" "സോഫിയ"
    "Atlantic/Bermuda" "ബർമുഡ"
    "America/La_Paz" "ലാ പാസ്"
    "America/Noronha" "നൊറോന"
    "America/Belem" "ബെലം"
    "America/Fortaleza" "ഫോർട്ടലീസ"
    "America/Recife" "റെസീഫെ"
    "America/Araguaina" "അറഗ്വൈന"
    "America/Maceio" "മാസിയോ"
    "America/Bahia" "ബഹിയ"
    "America/Sao_Paulo" "സാവോപോളോ"
    "America/Campo_Grande" "ക്യാമ്പോ ഗ്രാൻഡെ"
    "America/Cuiaba" "കുയ്‌ബ"
    "America/Santarem" "സാന്ററെം"
    "America/Porto_Velho" "പോർട്ടോ വെല്ലോ"
    "America/Boa_Vista" "ബോവ വിസ്റ്റ"
    "America/Manaus" "മനൗസ്"
    "America/Eirunepe" "യെറുനീപ്പെ"
    "America/Rio_Branco" "റിയോ ബ്രാങ്കോ"
    "Asia/Thimphu" "തിംഫു"
    "Europe/Minsk" "മിൻ‌സ്ക്"
    "America/Belize" "ബെലീസ്"
    "America/St_Johns" "സെന്റ് ജോൺസ്"
    "America/Halifax" "ഹാലിഫാക്സ്"
    "America/Glace_Bay" "ഗ്ലെയ്സ് ബേ"
    "America/Moncton" "മോംഗ്‌ടൻ"
    "America/Goose_Bay" "ഗൂസ് ബേ"
    "America/Toronto" "ടൊറന്റോ"
    "America/Iqaluit" "ഇഖാലിത്"
    "America/Winnipeg" "വിന്നിപെഗ്"
    "America/Resolute" "റെസല്യൂട്ട്"
    "America/Rankin_Inlet" "റാങ്കിൻ ഇൻലെറ്റ്"
    "America/Regina" "റിജീന"
    "America/Swift_Current" "സ്വിഫ്‌റ്റ് കറന്റ്"
    "America/Edmonton" "എഡ്മോൺടൺ"
    "America/Cambridge_Bay" "കേംബ്രിഡ്‌ജ് ബേ"
    "America/Inuvik" "ഇനുവിക്"
    "America/Vancouver" "വാൻ‌കൂവർ"
    "America/Dawson_Creek" "ഡോവ്സൺ ക്രീക്ക്"
    "America/Fort_Nelson" "ഫോർട്ട് നെൽസൺ"
    "America/Whitehorse" "വൈറ്റ്ഹോഴ്സ്"
    "America/Dawson" "ഡോവ്സൺ"
    "Europe/Zurich" "സൂറിച്ച്"
    "Africa/Abidjan" "അബിദ്‌ജാൻ‌"
    "Pacific/Rarotonga" "റാരോടോംഗ"
    "America/Santiago" "സാന്റിയാഗോ"
    "America/Coyhaique" "കൊയൈകേ"
    "America/Punta_Arenas" "പുന്റ അരീനസ്"
    "Pacific/Easter" "ഈസ്റ്റർ"
    "Asia/Shanghai" "ഷാങ്‌ഹായി"
    "Asia/Urumqi" "ഉറുംഖി"
    "America/Bogota" "ബൊഗോട്ട"
    "America/Costa_Rica" "കോസ്റ്റ റിക്ക"
    "America/Havana" "ഹവാന"
    "Atlantic/Cape_Verde" "കേപ് വെർദെ"
    "Asia/Nicosia" "നിക്കോഷ്യ"
    "Asia/Famagusta" "ഫാമഗുസ്‌റ്റ"
    "Europe/Prague" "പ്രാഗ്"
    "Europe/Berlin" "ബെർ‌ലിൻ‌"
    "America/Santo_Domingo" "സാന്തോ ഡോമിംഗോ"
    "Africa/Algiers" "അൾജിയേഴ്‌സ്"
    "America/Guayaquil" "ഗുവായക്വിൽ"
    "Pacific/Galapagos" "ഗാലപ്പാഗോസ്"
    "Europe/Tallinn" "ടാലിൻ‌"
    "Africa/Cairo" "കെയ്‌റോ"
    "Africa/El_Aaiun" "എൽ‌ ഐയുൻ‌"
    "Europe/Madrid" "മാഡ്രിഡ്"
    "Africa/Ceuta" "ക്യൂട്ട"
    "Atlantic/Canary" "ക്യാനറി"
    "Europe/Helsinki" "ഹെൽ‌സിങ്കി"
    "Pacific/Fiji" "ഫിജി"
    "Atlantic/Stanley" "സ്റ്റാൻ‌ലി"
    "Pacific/Kosrae" "കൊസ്രേ"
    "Atlantic/Faroe" "ഫെറോ"
    "Europe/Paris" "പാരീസ്"
    "Europe/London" "ലണ്ടൻ‌"
    "Asia/Tbilisi" "തിബിലിസി"
    "America/Cayenne" "കയീൻ‌"
    "Europe/Gibraltar" "ജിബ്രാൾട്ടർ"
    "America/Nuuk" "നൂക്ക്"
    "America/Danmarkshavn" "ഡാൻമാർക്ക്ഷാവ്ൻ"
    "America/Scoresbysund" "ഇറ്റ്വാഖ്വാർടൂർമിറ്റ്"
    "America/Thule" "തൂളി"
    "Europe/Athens" "ഏതൻ‌സ്"
    "Atlantic/South_Georgia" "ദക്ഷിണ ജോർജിയ"
    "America/Guatemala" "ഗ്വാട്ടിമാല"
    "Pacific/Guam" "ഗ്വാം"
    "Africa/Bissau" "ബിസ്സാവു"
    "America/Guyana" "ഗയാന"
    "Asia/Hong_Kong" "ഹോങ്കോംഗ്"
    "America/Tegucigalpa" "ടെഗൂസിഗാൽപ"
    "America/Port-au-Prince" "പോർട്ടോപ്രിൻസ്"
    "Europe/Budapest" "ബുഡാപെസ്റ്റ്"
    "Asia/Jakarta" "ജക്കാർത്ത"
    "Asia/Pontianak" "പൊന്റിയാനക്"
    "Asia/Makassar" "മകസ്സർ"
    "Asia/Jayapura" "ജയപുര"
    "Europe/Dublin" "ഡബ്ലിൻ"
    "Asia/Jerusalem" "ജെറുസലേം"
    "Asia/Kolkata" "കൊൽ‌ക്കത്ത"
    "Indian/Chagos" "ചാഗോസ്"
    "Asia/Baghdad" "ബാഗ്‌ദാദ്"
    "Asia/Tehran" "ടെഹ്‌റാൻ‌"
    "Europe/Rome" "റോം"
    "America/Jamaica" "ജമൈക്ക"
    "Asia/Amman" "അമ്മാൻ‌"
    "Asia/Tokyo" "ടോക്കിയോ"
    "Africa/Nairobi" "നയ്‌റോബി"
    "Asia/Bishkek" "ബിഷ്‌കേക്"
    "Pacific/Tarawa" "തരാവ"
    "Pacific/Kanton" "കാന്റൺ ദ്വീപ്"
    "Pacific/Kiritimati" "കിരിറ്റിമാറ്റി"
    "Asia/Pyongyang" "പ്യോംഗ്‌യാംഗ്"
    "Asia/Seoul" "സോൾ"
    "Asia/Almaty" "അൽമാട്ടി"
    "Asia/Qyzylorda" "ഖിസിലോർഡ"
    "Asia/Qostanay" "കോസ്റ്റനേ"
    "Asia/Aqtobe" "അഖ്‌തോബ്"
    "Asia/Aqtau" "അക്തൗ"
    "Asia/Atyrau" "അറ്റിറോ"
    "Asia/Oral" "ഓറൽ"
    "Asia/Beirut" "ബെയ്‌റൂട്ട്"
    "Asia/Colombo" "കൊളം‌ബോ"
    "Africa/Monrovia" "മൺ‌റോവിയ"
    "Europe/Vilnius" "വിൽ‌നിയസ്"
    "Europe/Riga" "റിഗ"
    "Africa/Tripoli" "ട്രിപൊളി"
    "Africa/Casablanca" "കാസബ്ലാങ്ക"
    "Europe/Chisinau" "ചിസിനാവു"
    "Pacific/Kwajalein" "ക്വാജലെയ്ൻ"
    "Asia/Yangon" "റങ്കൂൺ‌"
    "Asia/Ulaanbaatar" "ഉലാൻബാത്തർ"
    "Asia/Hovd" "ഹോഡ്"
    "Asia/Macau" "മക്കാവു"
    "America/Martinique" "മാർട്ടിനിക്"
    "Europe/Malta" "മാൾട്ട"
    "Indian/Mauritius" "മൗറീഷ്യസ്"
    "Indian/Maldives" "മാലിദ്വീപ്"
    "America/Mexico_City" "മെക്സിക്കോ സിറ്റി"
    "America/Cancun" "കാൻകൂൺ"
    "America/Merida" "മെരിഡ"
    "America/Monterrey" "മോണ്ടെറി"
    "America/Matamoros" "മറ്റാമൊറോസ്"
    "America/Chihuahua" "ചിഹ്വാഹ"
    "America/Ciudad_Juarez" "സിയുഡാഡ് ഹുവാരസ്"
    "America/Ojinaga" "ഒജിൻഗ"
    "America/Mazatlan" "മസറ്റ്‌ലാൻ"
    "America/Bahia_Banderas" "ബഹിയ ബൻഡാരസ്"
    "America/Hermosillo" "ഹെർമോസില്ലോ"
    "America/Tijuana" "തിയുവാന"
    "Asia/Kuching" "കുചിങ്"
    "Africa/Maputo" "മാപ്യുട്ടോ"
    "Africa/Windhoek" "വിൻഡ്‌ഹോക്"
    "Pacific/Noumea" "നോമിയ"
    "Pacific/Norfolk" "നോർ‌ഫോക്ക്"
    "Africa/Lagos" "ലാഗോസ്"
    "America/Managua" "മനാഗ്വ"
    "Asia/Kathmandu" "കാഠ്‌മണ്ഡു"
    "Pacific/Nauru" "നൗറു"
    "Pacific/Niue" "നിയു"
    "Pacific/Auckland" "ഓക്ക്‌ലാന്റ്"
    "Pacific/Chatham" "ചാത്തം"
    "America/Panama" "പനാമ"
    "America/Lima" "ലിമ"
    "Pacific/Tahiti" "താഹിതി"
    "Pacific/Marquesas" "മാർക്യുസാസ്"
    "Pacific/Gambier" "ഗാമ്പിയർ"
    "Pacific/Port_Moresby" "പോർട്ട് മോഴ്‌സ്ബൈ"
    "Pacific/Bougainville" "ബോഗൺവില്ലെ"
    "Asia/Manila" "മനില"
    "Asia/Karachi" "കറാച്ചി"
    "Europe/Warsaw" "വാർസോ"
    "America/Miquelon" "മിക്വലൻ"
    "Pacific/Pitcairn" "പിറ്റ്കയിൻ‌"
    "America/Puerto_Rico" "പ്യൂർട്ടോ റിക്കോ"
    "Asia/Gaza" "ഗാസ"
    "Asia/Hebron" "ഹെബ്‌റോൺ"
    "Europe/Lisbon" "ലിസ്‌ബൺ‌"
    "Atlantic/Madeira" "മഡെയ്റ"
    "Atlantic/Azores" "അസോറസ്"
    "Pacific/Palau" "പലാവു"
    "America/Asuncion" "അസൻ‌ഷ്യൻ‌"
    "Asia/Qatar" "ഖത്തർ"
    "Europe/Bucharest" "ബുച്ചാറെസ്റ്റ്"
    "Europe/Belgrade" "ബെൽഗ്രേഡ്"
    "Europe/Kaliningrad" "കലിനിൻഗ്രാഡ്"
    "Europe/Moscow" "മോസ്കോ"
    "Europe/Simferopol" "സിംഫെറോപോൾ"
    "Europe/Kirov" "കിറോ"
    "Europe/Volgograd" "വോൾഗോഗ്രാഡ്"
    "Europe/Astrakhan" "അസ്‌ട്രഖാൻ"
    "Europe/Saratov" "സരാറ്റോവ്"
    "Europe/Ulyanovsk" "ഉല്ല്യാനോവ്‌സ്‌ക്"
    "Europe/Samara" "സമാറ"
    "Asia/Yekaterinburg" "യാകാറ്റെറിൻബർഗ്"
    "Asia/Omsk" "ഒംസ്ക്"
    "Asia/Novosibirsk" "നൊവോസിബിർസ്ക്"
    "Asia/Barnaul" "ബർണോൽ"
    "Asia/Tomsk" "ടോംസ്ക്"
    "Asia/Novokuznetsk" "നോവോകുസെൻസ്‌ക്"
    "Asia/Krasnoyarsk" "ക്രാസ്നോയാസ്ക്"
    "Asia/Irkutsk" "ഇർകസ്ക്"
    "Asia/Chita" "ചീറ്റ"
    "Asia/Yakutsk" "യാക്കറ്റ്സ്‌ക്"
    "Asia/Khandyga" "കാൻഡിഗ"
    "Asia/Vladivostok" "വ്ളാഡിവോസ്റ്റോക്"
    "Asia/Ust-Nera" "യുസ്-നേര"
    "Asia/Magadan" "മഗഡാൻ"
    "Asia/Sakhalin" "സഖാലിൻ"
    "Asia/Srednekolymsk" "സ്രിഡ്‌നികോളിംസ്ക്"
    "Asia/Kamchatka" "കാംചട്ക"
    "Asia/Anadyr" "അനാഡിർ"
    "Asia/Riyadh" "റിയാദ്"
    "Pacific/Guadalcanal" "ഗ്വാഡൽകനാൽ"
    "Africa/Khartoum" "ഖാർ‌തൌം"
    "Asia/Singapore" "സിംഗപ്പൂർ"
    "America/Paramaribo" "പരാമാരിബോ"
    "Africa/Juba" "ജുബ"
    "Africa/Sao_Tome" "സാവോ ടോം‌"
    "America/El_Salvador" "എൽ സാൽ‌വദോർ"
    "Asia/Damascus" "ദമാസ്കസ്"
    "America/Grand_Turk" "ഗ്രാൻഡ് ടർക്ക്"
    "Africa/Ndjamena" "ജമെന"
    "Asia/Bangkok" "ബാങ്കോക്ക്"
    "Asia/Dushanbe" "ദുഷൻ‌ബെ"
    "Pacific/Fakaofo" "ഫക്കാവോഫോ"
    "Asia/Dili" "ദിലി"
    "Asia/Ashgabat" "ആഷ്‌ഗാബട്ട്"
    "Africa/Tunis" "ട്യൂണിസ്"
    "Pacific/Tongatapu" "ടോംഗാടാപു"
    "Europe/Istanbul" "ഇസ്താം‌ബുൾ‌"
    "Asia/Taipei" "തായ്‌പെയ്"
    "Europe/Kyiv" "കീവ്"
    "America/New_York" "ന്യൂയോർക്ക്"
    "America/Detroit" "ഡെട്രോയിറ്റ്"
    "America/Kentucky/Louisville" "ലൂയിസ്‌വില്ലെ"
    "America/Kentucky/Monticello" "മോണ്ടിസെല്ലോ, കെന്റക്കി"
    "America/Indiana/Indianapolis" "ഇൻഡ്യാനാപോളിസ്"
    "America/Indiana/Vincennes" "വിൻസെൻസ്, ഇൻഡ്യാന"
    "America/Indiana/Winamac" "വിനാമാക്, ഇൻഡ്യാന"
    "America/Indiana/Marengo" "മരെങ്കോ, ഇൻഡ്യാന"
    "America/Indiana/Petersburg" "പീറ്റേഴ്സ്ബർഗ്, ഇൻഡ്യാന"
    "America/Indiana/Vevay" "വിവെയ്, ഇൻഡ്യാന"
    "America/Chicago" "ചിക്കാഗോ"
    "America/Indiana/Tell_City" "റ്റെൽ സിറ്റി, ഇൻഡ്യാന"
    "America/Indiana/Knox" "നോക്സ്, ഇൻഡ്യാന"
    "America/Menominee" "മെനോമിനീ"
    "America/North_Dakota/Center" "സെന്റർ, വടക്കൻ ഡെക്കോട്ട"
    "America/North_Dakota/New_Salem" "ന്യൂ സെയ്‌ലം, വടക്കൻ ഡെക്കോട്ട"
    "America/North_Dakota/Beulah" "ബ്യൂല, വടക്കൻ ഡെക്കോട്ട"
    "America/Denver" "ഡെൻ‌വർ"
    "America/Boise" "ബൊയ്സി"
    "America/Phoenix" "ഫീനിക്സ്"
    "America/Los_Angeles" "ലോസ് എയ്ഞ്ചലസ്"
    "America/Anchorage" "ആങ്കറേജ്"
    "America/Juneau" "ജൂനോ"
    "America/Sitka" "സിറ്റ്‌കാ"
    "America/Metlakatla" "മെഡ്‌ലകട്‌ലെ"
    "America/Yakutat" "യാകുറ്റാറ്റ്"
    "America/Nome" "നോം"
    "America/Adak" "അഡാക്"
    "Pacific/Honolulu" "ഹോണലൂലു"
    "America/Montevideo" "മൊണ്ടെ‌വീഡിയോ"
    "Asia/Samarkand" "സമർക്കന്ദ്"
    "Asia/Tashkent" "താഷ്‌ക്കന്റ്"
    "America/Caracas" "കരാക്കസ്"
    "Asia/Ho_Chi_Minh" "ഹോ ചി മിൻ സിറ്റി"
    "Pacific/Efate" "ഇഫാതെ"
    "Pacific/Apia" "ആപിയ"
    "Africa/Johannesburg" "ജോഹന്നാസ്ബർ‌ഗ്"
    "America/Antigua" "ആൻറിഗ്വ"
    "America/Anguilla" "ആൻഗ്വില്ല"
    "Africa/Luanda" "ലുവാൻഡ"
    "Antarctica/McMurdo" "മാക്മർഡോ"
    "Antarctica/DumontDUrville" "ഡ്യൂമണ്ട് ഡി യുർവിൽ"
    "Antarctica/Syowa" "സ്യോവ"
    "America/Aruba" "അറൂബ"
    "Europe/Mariehamn" "മരിയാഹാമൻ"
    "Europe/Sarajevo" "സരയേവോ"
    "Africa/Ouagadougou" "ഔഗാദൗഗൗ"
    "Asia/Bahrain" "ബഹ്റിൻ"
    "Africa/Bujumbura" "ബുജും‌ബുര"
    "Africa/Porto-Novo" "പോർ‌ട്ടോ-നോവോ"
    "America/St_Barthelemy" "സെന്റ് ബർത്തലെമി"
    "Asia/Brunei" "ബ്രൂണൈ"
    "America/Kralendijk" "കാർലൻഡിജെക്ക്"
    "America/Nassau" "നാസൗ"
    "Africa/Gaborone" "ഗാബറോൺ"
    "America/Blanc-Sablon" "ബ്ലാങ്ക് സാബ്ലോൺ"
    "America/Atikokan" "ഏറ്റികോക്കൺ"
    "America/Creston" "ക്രെസ്റ്റൺ"
    "Indian/Cocos" "കോക്കോസ്"
    "Africa/Kinshasa" "കിൻഷാസ"
    "Africa/Lubumbashi" "ലൂബുംബാഷി"
    "Africa/Bangui" "ബംഗുയി"
    "Africa/Brazzaville" "ബ്രാസവിൽ"
    "Africa/Douala" "ഡൗല"
    "America/Curacao" "കുറാക്കാവോ"
    "Indian/Christmas" "ക്രിസ്തുമസ്"
    "Europe/Busingen" "ബുസിൻജൻ"
    "Africa/Djibouti" "ദിജിബൗട്ടി"
    "Europe/Copenhagen" "കോപ്പൻ‌ഹേഗൻ‌"
    "America/Dominica" "ഡൊമിനിക്ക"
    "Africa/Asmara" "അസ്‍മാര"
    "Africa/Addis_Ababa" "അഡിസ് അബാബ"
    "Pacific/Chuuk" "ചക്"
    "Pacific/Pohnpei" "പോൺപെ"
    "Africa/Libreville" "ലിബ്രെവില്ല"
    "America/Grenada" "ഗ്രനേഡ"
    "Europe/Guernsey" "ഗേൺസേ"
    "Africa/Accra" "ആക്ര"
    "Africa/Banjul" "ബഞ്ചുൽ"
    "Africa/Conakry" "കൊണാക്രി"
    "America/Guadeloupe" "ഗ്വാഡലൂപ്പ്"
    "Africa/Malabo" "മലാബോ"
    "Europe/Zagreb" "സാക്രെബ്"
    "Europe/Isle_of_Man" "ഐൽ‌ ഓഫ് മാൻ‌"
    "Atlantic/Reykjavik" "റേയ്‌ജാവിക്"
    "Europe/Jersey" "ജേഴ്‌സി"
    "Asia/Phnom_Penh" "ഫെനോം പെൻ"
    "Indian/Comoro" "കൊമോറോ"
    "America/St_Kitts" "സെന്റ് കിറ്റ്സ്"
    "Asia/Kuwait" "കുവൈത്ത്"
    "America/Cayman" "കേമാൻ"
    "Asia/Vientiane" "വെന്റിയാൻ"
    "America/St_Lucia" "സെന്റ് ലൂസിയ"
    "Europe/Vaduz" "വാദുസ്"
    "Africa/Maseru" "മസേറു"
    "Europe/Luxembourg" "ലക്‌സംബർഗ്"
    "Europe/Monaco" "മൊണാക്കോ"
    "Europe/Podgorica" "പൊഡ്‍ഗൊറിസ"
    "America/Marigot" "മാരിഗോ"
    "Indian/Antananarivo" "അൻറാനനറിവോ"
    "Pacific/Majuro" "മജൂറോ"
    "Europe/Skopje" "സ്കോപ്പിയെ"
    "Africa/Bamako" "ബമാകോ"
    "Pacific/Saipan" "സെയ്‌പ്പാൻ‌"
    "Africa/Nouakchott" "നൗവാക്‌ഷോട്ട്"
    "America/Montserrat" "മൊണ്ടെസരത്ത്"
    "Africa/Blantyre" "ബ്ലാണ്ടെയർ‌"
    "Asia/Kuala_Lumpur" "ക്വാലലം‌പൂർ‌‌"
    "Africa/Niamey" "നിയാമി"
    "Europe/Amsterdam" "ആം‌സ്റ്റർ‌ഡാം"
    "Europe/Oslo" "ഓസ്ലോ"
    "Asia/Muscat" "മസ്കറ്റ്"
    "Indian/Reunion" "റീയൂണിയൻ"
    "Africa/Kigali" "കിഗാലി"
    "Indian/Mahe" "മാഹി"
    "Europe/Stockholm" "സ്റ്റോക്ക്ഹോം"
    "Atlantic/St_Helena" "സെന്റ് ഹെലെന"
    "Europe/Ljubljana" "ലുബ്‍ലിയാന"
    "Arctic/Longyearbyen" "ലംഗ്‍യെർബിൻ"
    "Europe/Bratislava" "ബ്രാട്ടിസ്‍ലാവ"
    "Africa/Freetown" "ഫ്രീടൗൺ"
    "Europe/San_Marino" "സാൻ മാരിനോ"
    "Africa/Dakar" "ഡാക്കർ‌"
    "Africa/Mogadishu" "മൊഗാദിഷു"
    "America/Lower_Princes" "ലോവർ പ്രിൻസസ് ക്വാർട്ടർ"
    "Africa/Mbabane" "മബാബെയ്‌ൻ‌"
    "Indian/Kerguelen" "കെർഗുലെൻ"
    "Africa/Lome" "ലോം"
    "America/Port_of_Spain" "പോർ‌ട്ട് ഓഫ് സ്‌പെയിൻ‌"
    "Pacific/Funafuti" "ഫുണാഫുട്ടി"
    "Africa/Dar_es_Salaam" "ദാർ എസ് സലാം"
    "Africa/Kampala" "കമ്പാല"
    "Pacific/Midway" "മിഡ്‌വേ"
    "Pacific/Wake" "വെയ്ക്"
    "Europe/Vatican" "വത്തിക്കാൻ"
    "America/St_Vincent" "സെന്റ് വിൻസെന്റ്"
    "America/Tortola" "ടോർ‌ട്ടോള"
    "America/St_Thomas" "സെന്റ് തോമസ്"
    "Pacific/Wallis" "വാല്ലിസ്"
    "Asia/Aden" "ഏദെൻ"
    "Indian/Mayotte" "മയോട്ടി"
    "Africa/Lusaka" "ലുസാക"
    "Africa/Harare" "ഹരാരെ"
};

// `common/main/my.xml`: 418 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const MY: &str = exemplar_cities! {
    "Europe/Andorra" "အန်ဒိုရာ"
    "Asia/Dubai" "ဒူဘိုင်း"
    "Asia/Kabul" "ကာဘူးလ်"
    "Europe/Tirane" "တီရာနီ"
    "Asia/Yerevan" "ရဲယ်ရေဗန်း"
    "Antarctica/Casey" "ကေစီ"
    "Antarctica/Davis" "ဒေးဗစ်"
    "Antarctica/Mawson" "မော်စွန်"
    "Antarctica/Palmer" "ပါလ်မာ"
    "Antarctica/Rothera" "ရိုသီရာ"
    "Antarctica/Troll" "ထရိုလ်"
    "Antarctica/Vostok" "ဗိုစ်တိုခ်"
    "America/Argentina/Buenos_Aires" "ဗျူနိုအေးရိစ်"
    "America/Argentina/Cordoba" "ကိုဒိုဘာ"
    "America/Argentina/Salta" "ဆာလ်တာ"
    "America/Argentina/Jujuy" "ဂျုဂျေ"
    "America/Argentina/Tucuman" "တူကူမန်"
    "America/Argentina/Catamarca" "ကာတာမာရကာ"
    "America/Argentina/La_Rioja" "လာ ရီယိုဟာ"
    "America/Argentina/San_Juan" "ဆန် ဂွမ်"
    "America/Argentina/Mendoza" "မန်ဒိုဇာ"
    "America/Argentina/San_Luis" "ဆန် လူဝီစ်"
    "America/Argentina/Rio_Gallegos" "ရီယို ဂါလီဂိုစ်"
    "America/Argentina/Ushuaia" "ဥဆွာအီအာ"
    "Pacific/Pago_Pago" "ပါဂိုပါဂို"
    "Europe/Vienna" "ဗီယင်နာ"
    "Australia/Lord_Howe" "လော့ဒ် ဟောင်"
    "Antarctica/Macquarie" "မက်ကွယ်ရီ"
    "Australia/Hobart" "ဟိုးဘားတ်"
    "Australia/Melbourne" "မဲလ်ဘုန်း"
    "Australia/Sydney" "ဆစ်ဒနီ"
    "Australia/Broken_Hill" "ဘရိုကင်ဟီးလ်"
    "Australia/Brisbane" "ဘရစ္စဘိန်း"
    "Australia/Lindeman" "လင်းဒီမန်း"
    "Australia/Adelaide" "အန္ဒီလိတ်ဒ်"
    "Australia/Darwin" "ဒါဝင်"
    "Australia/Perth" "ပါးသ်"
    "Australia/Eucla" "ယူးခလာ"
    "Asia/Baku" "ဘာကူ"
    "America/Barbados" "ဘာဘေးဒိုးစ်"
    "Asia/Dhaka" "ဒက်ကာ"
    "Europe/Brussels" "ဘရပ်ဆဲလ်"
    "Europe/Sofia" "ဆိုဖီအာ"
    "Atlantic/Bermuda" "ဘာမြူဒါ"
    "America/La_Paz" "လာပါဇ်"
    "America/Noronha" "နိုရိုညာ"
    "America/Belem" "ဘီလင်မ်"
    "America/Fortaleza" "ဖို့တ်တာလီဇာ"
    "America/Recife" "ဟေစီဖီလ်"
    "America/Araguaina" "အာရာဂွါအီနာ"
    "America/Maceio" "မာဆဲသွာ"
    "America/Bahia" "ဘာဟီအာ"
    "America/Sao_Paulo" "ဆော်ပိုလို"
    "America/Campo_Grande" "ကိမ်ပို ဂရန်ဒီ"
    "America/Cuiaba" "ကွီရာဘာ"
    "America/Santarem" "ဆန်တာရမ်"
    "America/Porto_Velho" "ပို့တ်တို ဗဲလီယို"
    "America/Boa_Vista" "ဘိုအာဗီစ်တာ"
    "America/Manaus" "မာနောက်စ်"
    "America/Eirunepe" "အီရူနီပီ"
    "America/Rio_Branco" "ရီယို ဘရန်ကို"
    "Asia/Thimphu" "တင်ဖူး"
    "Europe/Minsk" "မင်းစခ်"
    "America/Belize" "ဘလိဇ်"
    "America/St_Johns" "စိန့်ဂျွန်း"
    "America/Halifax" "ဟလီဖက်စ်"
    "America/Glace_Bay" "ဂလဲစ်ဘေး"
    "America/Moncton" "မွန်ခ်တွန်"
    "America/Goose_Bay" "ဂူးစ်ဘေး"
    "America/Toronto" "တိုရန်တို"
    "America/Iqaluit" "အီကာလူအီတ်"
    "America/Winnipeg" "ဝီနီဗက်ဂ်"
    "America/Resolute" "ရီဆိုလုပ်(တ်)"
    "America/Rankin_Inlet" "ရန်ကင် အင်းလက်"
    "America/Regina" "ရယ်ဂျီနာ"
    "America/Swift_Current" "စွတ်ဖ်တ် ကားရင့်"
    "America/Edmonton" "အက်ဒ်မွန်တန်"
    "America/Cambridge_Bay" "ကိန်းဘရစ်ချ် ဘေး"
    "America/Inuvik" "အီနုဗီခ်"
    "America/Vancouver" "ဗန်ကူးဗား"
    "America/Dawson_Creek" "ဒေါ်ဆန် ခရိခ်"
    "America/Fort_Nelson" "ဖို့တ် နယ်လ်ဆင်"
    "America/Whitehorse" "ဝိုက်(တ်)ဟိုစ်"
    "America/Dawson" "ဒေါ်ဆန်"
    "Europe/Zurich" "ဇူးရစ်ချ်"
    "Africa/Abidjan" "အာဘီဂျန်"
    "Pacific/Rarotonga" "ရာရိုတွန်းဂါ"
    "America/Santiago" "ဆန်တီအာဂို"
    "America/Coyhaique" "ကွိုင်းဟိုင်ခ်"
    "America/Punta_Arenas" "ပွန်တာ အရီနာစ်"
    "Pacific/Easter" "အီစတာ"
    "Asia/Shanghai" "ရှန်ဟိုင်း"
    "Asia/Urumqi" "အူရုမ်ချီ"
    "America/Bogota" "ဘိုဂိုတာ"
    "America/Costa_Rica" "ကို့စတာရီကာ"
    "America/Havana" "ဟာဗာနာ"
    "Atlantic/Cape_Verde" "ကိတ်ပ် ဗာဒီ"
    "Asia/Nicosia" "နီကိုရှား"
    "Asia/Famagusta" "ဖာမာဂူစတာ"
    "Europe/Prague" "ပရက်ဂ်"
    "Europe/Berlin" "ဘာလင်"
    "America/Santo_Domingo" "ဆန်တို ဒိုမင်းဂို"
    "Africa/Algiers" "အယ်လ်ဂျီးရီးယား"
    "America/Guayaquil" "ဂွါရာကွီးလ်"
    "Pacific/Galapagos" "ဂါလာပါကပ်စ်"
    "Europe/Tallinn" "ထားလင်"
    "Africa/Cairo" "ကိုင်ရို"
    "Africa/El_Aaiun" "အယ်လ်အာယွန်း"
    "Europe/Madrid" "မဒရစ်"
    "Africa/Ceuta" "ဆီရူးတာ"
    "Atlantic/Canary" "ကနေရီ"
    "Europe/Helsinki" "ဟဲလ်စင်ကီ"
    "Pacific/Fiji" "ဖီဂျီ"
    "Atlantic/Stanley" "စတန်လေ"
    "Pacific/Kosrae" "ခိုစ်ရိုင်"
    "Atlantic/Faroe" "ဖါရို"
    "Europe/Paris" "ပဲရစ်"
    "Europe/London" "လန်ဒန်"
    "Asia/Tbilisi" "တဘီးလီစီ"
    "America/Cayenne" "ကေညင်န်"
    "Europe/Gibraltar" "ဂျီဘရော်လ်တာ"
    "America/Nuuk" "နုခ်"
    "America/Danmarkshavn" "ဒန်မတ်ရှ်ဗာန်"
    "America/Scoresbysund" "အစ်တာကာ တိုးမိရက်တ်"
    "America/Thule" "သုလီ"
    "Europe/Athens" "အေသင်"
    "Atlantic/South_Georgia" "တောင်ဂျော်ဂျီယာ"
    "America/Guatemala" "ဂွါတီမာလာ"
    "Pacific/Guam" "ဂူအမ်"
    "Africa/Bissau" "ဘီစာအို"
    "America/Guyana" "ဂိုင်ယာနာ"
    "Asia/Hong_Kong" "ဟောင်ကောင်"
    "America/Tegucigalpa" "တီဂူစီဂလ်ပါ"
    "America/Port-au-Prince" "ပို့တ်-အို-ပရင့်စ်"
    "Europe/Budapest" "ဘူဒါပက်စ်"
    "Asia/Jakarta" "ဂျကာတာ"
    "Asia/Pontianak" "ပွန်တီအားနာ့ခ်"
    "Asia/Makassar" "မခက်စ်ဆာ"
    "Asia/Jayapura" "ဂျာရာပူရာ"
    "Europe/Dublin" "ဒတ်ဘလင်"
    "Asia/Jerusalem" "ဂျေရုဆလင်"
    "Asia/Kolkata" "ကိုလျကတ်တား"
    "Indian/Chagos" "ချာဂိုစ်"
    "Asia/Baghdad" "ဘဂ္ဂဒက်"
    "Asia/Tehran" "တီဟီရန်"
    "Europe/Rome" "ရောမ"
    "America/Jamaica" "ဂျမေကာ"
    "Asia/Amman" "အာမာန်း"
    "Asia/Tokyo" "တိုကျို"
    "Africa/Nairobi" "နိုင်ရိုဘီ"
    "Asia/Bishkek" "ဘီရှ်ခက်"
    "Pacific/Tarawa" "တာရာဝါ"
    "Pacific/Kanton" "ကန်တွန်"
    "Pacific/Kiritimati" "ခရိဒီမတီ"
    "Asia/Pyongyang" "ပြုံယန်း"
    "Asia/Seoul" "ဆိုးလ်"
    "Asia/Almaty" "အော်မာတီ"
    "Asia/Qyzylorda" "ကီဇလော်ဒါ"
    "Asia/Qostanay" "ကော့စ်တနေ"
    "Asia/Aqtobe" "အာချတူးဘီ"
    "Asia/Aqtau" "အက်တာဥု"
    "Asia/Atyrau" "အာတီရအူ"
    "Asia/Oral" "အော်ရဲလ်"
    "Asia/Beirut" "ဘေရွတ်"
    "Asia/Colombo" "ကိုလံဘို"
    "Africa/Monrovia" "မွန်ရိုးဗီးယား"
    "Europe/Vilnius" "ဗီးလ်နီအိုးစ်"
    "Europe/Riga" "ရီဂါ"
    "Africa/Tripoli" "ထရီပိုလီ"
    "Africa/Casablanca" "ကာဆာဘလန်ကာ"
    "Europe/Chisinau" "ချီရှီနားအူ"
    "Pacific/Kwajalein" "ခွာဂျာလိန်"
    "Asia/Yangon" "ရန်ကုန်"
    "Asia/Ulaanbaatar" "ဥလန်ဘာတော"
    "Asia/Hovd" "ဟိုးဗျ"
    "Asia/Macau" "မကာအို"
    "America/Martinique" "မာတီနီဂ်"
    "Europe/Malta" "မော်လ်တာ"
    "Indian/Mauritius" "မောရစ်ရှ"
    "Indian/Maldives" "မော်လဒိုက်"
    "America/Mexico_City" "မက်ကဆီကို စီးတီး"
    "America/Cancun" "ကန်ခန်"
    "America/Merida" "မီရီဒါ"
    "America/Monterrey" "မွန်တဲရေး"
    "America/Matamoros" "မာတာမိုရိုစ်"
    "America/Chihuahua" "ချီဟူအာဟူအာ"
    "America/Ciudad_Juarez" "စီယူဒတ်စ် ဟွာရက်စ်"
    "America/Ojinaga" "အိုခီနဂါ"
    "America/Mazatlan" "မာဇတ်လန်"
    "America/Bahia_Banderas" "ဘာဟီအာ ဘန်ဒရက်စ်"
    "America/Hermosillo" "ဟာမိုစ်စီလို"
    "America/Tijuana" "တီဂွါနာ"
    "Asia/Kuching" "ကူချင်"
    "Africa/Maputo" "မာပူးတို"
    "Africa/Windhoek" "ဗင်းဟူးခ်"
    "Pacific/Noumea" "နူမယ်အာ"
    "Pacific/Norfolk" "နော်ဖော့ခ်"
    "Africa/Lagos" "လာဂိုစ်"
    "America/Managua" "မာနာဂွါ"
    "Asia/Kathmandu" "ခတ်တမန်ဒူ"
    "Pacific/Nauru" "နာဥူရူ"
    "Pacific/Niue" "နီဦးအေ"
    "Pacific/Auckland" "အော့ကလန်"
    "Pacific/Chatham" "ချားသမ်"
    "America/Panama" "ပနားမား"
    "America/Lima" "လီမာ"
    "Pacific/Tahiti" "တဟီတီ"
    "Pacific/Marquesas" "မာခေးအပ်စ်"
    "Pacific/Gambier" "ဂမ်ဘီယာ"
    "Pacific/Port_Moresby" "ဖို့တ် မိုရက်စ်ဘီ"
    "Pacific/Bougainville" "ဘူဂန်ဗီးလီးယား"
    "Asia/Manila" "မနီလာ"
    "Asia/Karachi" "ကရာချိ"
    "Europe/Warsaw" "ဝါဆော"
    "America/Miquelon" "မီကွီလွန်"
    "Pacific/Pitcairn" "ပါတ်ကယ်ရင်"
    "America/Puerto_Rico" "ပေါ်တိုရီကို"
    "Asia/Gaza" "ဂါဇာ"
    "Asia/Hebron" "ဟီဘရွန်"
    "Europe/Lisbon" "လစ္စဘွန်း"
    "Atlantic/Madeira" "မဒီးရာ"
    "Atlantic/Azores" "အေဇိုးရီးစ်"
    "Pacific/Palau" "ပလာအို"
    "America/Asuncion" "အာဆူစီအွန်း"
    "Asia/Qatar" "ကာတာ"
    "Europe/Bucharest" "ဘူခါရက်စ်"
    "Europe/Belgrade" "ဘဲလ်ဂရိတ်"
    "Europe/Kaliningrad" "ခါလီနင်ဂရက်"
    "Europe/Moscow" "မော်စကို"
    "Europe/Simferopol" "စင်ဖာရိုးဖို"
    "Europe/Kirov" "ခီရိုဗ်"
    "Europe/Volgograd" "ဗိုလ်ဂိုဂရက်"
    "Europe/Astrakhan" "အားစ်တရခန်း"
    "Europe/Saratov" "ဆာရာတို့ဖ်"
    "Europe/Ulyanovsk" "အူလီယာနိုစကစ်ဖ်"
    "Europe/Samara" "ဆာမားရာ"
    "Asia/Yekaterinburg" "ရယ်ခါးတီရင်ဘားခ်"
    "Asia/Omsk" "အွမ်းစ်ခ်"
    "Asia/Novosibirsk" "နိုဗိုစဲဘီအဲယ်စ်"
    "Asia/Barnaul" "ဘရ်နာအူ"
    "Asia/Tomsk" "တွန်မ်စ်ခ်"
    "Asia/Novokuznetsk" "နိုဗိုခူဇ်နက်စ်"
    "Asia/Krasnoyarsk" "ခရာ့စ်နိုရာစ်"
    "Asia/Irkutsk" "အီရူခူတ်"
    "Asia/Chita" "ချီတာ"
    "Asia/Yakutsk" "ယူခူးတ်စ်"
    "Asia/Khandyga" "ခန်ဒိုင်ဂါ"
    "Asia/Vladivostok" "ဗလာဒီဗော့စတော့ခ်"
    "Asia/Ust-Nera" "အူးစ် နီရား"
    "Asia/Magadan" "မာဂါဒန်း"
    "Asia/Sakhalin" "ဆာခါလင်"
    "Asia/Srednekolymsk" "ဆရစ်နစ်ကာလင်မ်စ်"
    "Asia/Kamchatka" "ခမ်ချာ့ခါ"
    "Asia/Anadyr" "အန်အာဒီအာ"
    "Asia/Riyadh" "ရီယားဒ်"
    "Pacific/Guadalcanal" "ဂွါဒါကနဲလ်"
    "Africa/Khartoum" "ခါတိုအန်"
    "Asia/Singapore" "စင်္ကာပူ"
    "America/Paramaribo" "ပါရာမာရီဘို"
    "Africa/Juba" "ဂျုဘာ"
    "Africa/Sao_Tome" "ဆောင်တူမေး"
    "America/El_Salvador" "အယ်လ်ဆာဗေဒို"
    "Asia/Damascus" "ဒမားစကပ်"
    "America/Grand_Turk" "ဂရန်ဒ် တခ်"
    "Africa/Ndjamena" "အင်ဂျာမီနာ"
    "Asia/Bangkok" "ဘန်ကောက်"
    "Asia/Dushanbe" "ဒူရှန်းဘဲ"
    "Pacific/Fakaofo" "ဖာခါအိုဖို"
    "Asia/Dili" "ဒစ်လီ"
    "Asia/Ashgabat" "အာရှ်ဂါဘာဒ်"
    "Africa/Tunis" "တူနီစ်"
    "Pacific/Tongatapu" "တွန်ဂါတာပု"
    "Europe/Istanbul" "အစ္စတန်ဘူလ်"
    "Asia/Taipei" "တိုင်ပေ"
    "Europe/Kyiv" "ခီးအက်ဖ်"
    "America/New_York" "နယူးယောက်"
    "America/Detroit" "ဒက်ထရွိုက်"
    "America/Kentucky/Louisville" "လူဝီဗီးလ်"
    "America/Kentucky/Monticello" "မွန်တီချယ်လို၊ ကင်တပ်ကီ"
    "America/Indiana/Indianapolis" "အင်ဒီယားနား ပိုလိစ်"
    "America/Indiana/Vincennes" "ဗင်ဆင့်စ်၊ အင်ဒီယားနား"
    "America/Indiana/Winamac" "ဝီနာမက်ခ်၊ အင်ဒီယားနား"
    "America/Indiana/Marengo" "မာရန်ဂို၊ အင်ဒီယားနား"
    "America/Indiana/Petersburg" "ပီတာစ်ဘတ်ခ်၊ အင်ဒီယားနား"
    "America/Indiana/Vevay" "ဗီဗဲ၊ အင်ဒီယားနား"
    "America/Chicago" "ချီကာကို"
    "America/Indiana/Tell_City" "တဲလ်စီးတီး၊ အင်ဒီယားနား"
    "America/Indiana/Knox" "နောက်ခ်စ်၊ အင်ဒီယားနား"
    "America/Menominee" "မီနိုမီနီး"
    "America/North_Dakota/Center" "စင်တာ၊ မြောက်ဒါကိုတာ"
    "America/North_Dakota/New_Salem" "နယူးဆေးလမ်၊ မြောက်ဒါကိုတာ"
    "America/North_Dakota/Beulah" "ဗြူလာ၊ မြောက်ဒါကိုတာ"
    "America/Denver" "ဒင်န်ဗာ"
    "America/Boise" "ဗွိုက်စီ"
    "America/Phoenix" "ဖီးနစ်"
    "America/Los_Angeles" "လော့စ်အိန်ဂျယ်လိစ်"
    "America/Anchorage" "အန်ကာရေ့ဂျ်"
    "America/Juneau" "ဂျုနိုအော"
    "America/Sitka" "စစ်ကာ"
    "America/Metlakatla" "မက်တ်လာကက်လာ"
    "America/Yakutat" "ရာကုတတ်"
    "America/Nome" "နိုမီ"
    "America/Adak" "အာဒချ"
    "Pacific/Honolulu" "ဟိုနိုလူလူ"
    "America/Montevideo" "မွန်တီဗီဒီအို"
    "Asia/Samarkand" "ဆမ်းမာခန်းဒ်"
    "Asia/Tashkent" "တာရှ်ကဲန့်"
    "America/Caracas" "ကာရာကာစ်"
    "Asia/Ho_Chi_Minh" "ဟိုချီမင်းစီးတီး"
    "Pacific/Efate" "အီဖာတီ"
    "Pacific/Apia" "အားပီအား"
    "Africa/Johannesburg" "ဂျိုဟန်းနက်စဘတ်"
    "America/Antigua" "အန်တီဂွါ"
    "America/Anguilla" "အန်ဂီလာ"
    "Africa/Luanda" "လူဝမ်ဒါ"
    "Antarctica/McMurdo" "မက်မူဒိုး"
    "Antarctica/DumontDUrville" "ဒူးမော့တ် ဒါရ်ဗီးလ်"
    "Antarctica/Syowa" "ရှိုးဝါ"
    "America/Aruba" "အာရူးဗာ"
    "Europe/Mariehamn" "မရီအာ ဟားမန်"
    "Europe/Sarajevo" "ဆာရာယေဗို"
    "Africa/Ouagadougou" "ဝါဂါဒူးဂူ"
    "Asia/Bahrain" "ဘာရိန်း"
    "Africa/Bujumbura" "ဘူဂျွန်ဘူးရာ"
    "Africa/Porto-Novo" "ပိုတို-နိုဗို"
    "America/St_Barthelemy" "စိန့်ဘာသယ်လမီ"
    "Asia/Brunei" "ဘရူနိုင်း"
    "America/Kralendijk" "ခရာလဲန်းဒစ်ချ်"
    "America/Nassau" "နာ့ဆော်"
    "Africa/Gaborone" "ဂါဘာရွန်းနီ"
    "America/Blanc-Sablon" "ဘလွန်ခ်-စာဘလွန်"
    "America/Atikokan" "အာတီကိုကန်"
    "America/Creston" "ကရစ်စတွန်"
    "Indian/Cocos" "ကိုကိုးစ်"
    "Africa/Kinshasa" "ကင်ရှာစာ"
    "Africa/Lubumbashi" "လူဘွန်းဘာရှီ"
    "Africa/Bangui" "ဘာန်ဂီး"
    "Africa/Brazzaville" "ဘရားဇာဗီးလ်"
    "Africa/Douala" "ဒိုအူအာလာ"
    "America/Curacao" "ကျူရေးကိုး"
    "Indian/Christmas" "ခရစ်စမတ်"
    "Europe/Busingen" "ဘူရှင်ဂျင်"
    "Africa/Djibouti" "ဂျီဘူတီ"
    "Europe/Copenhagen" "ကိုပင်ဟေဂင်"
    "America/Dominica" "ဒိုမီနီကာ"
    "Africa/Asmara" "အားစ်မားရာ"
    "Africa/Addis_Ababa" "အားဒစ် အဘာဘာ"
    "Pacific/Chuuk" "ချုခ်"
    "Pacific/Pohnpei" "ဖိုနာဖဲအ်"
    "Africa/Libreville" "လီဗရာဗီးလ်"
    "America/Grenada" "ဂရီနေဒါ"
    "Europe/Guernsey" "ဂွန်းဇီ"
    "Africa/Accra" "အက်ကရာ"
    "Africa/Banjul" "ဘန်ဂျုးလ်"
    "Africa/Conakry" "ကိုနာကရီး"
    "America/Guadeloupe" "ဂွါဒီလုပ်"
    "Africa/Malabo" "မာလာဘို"
    "Europe/Zagreb" "ဇာဂ်ဂရက်ဘ်"
    "Europe/Isle_of_Man" "မန်းကျွန်း"
    "Atlantic/Reykjavik" "ရေးကီဗစ်ခ်"
    "Europe/Jersey" "ဂျာစီ"
    "Asia/Phnom_Penh" "ဖနွမ်ပင်"
    "Indian/Comoro" "ကိုမိုရို"
    "America/St_Kitts" "စိန့်ကိစ်"
    "Asia/Kuwait" "ကူဝိတ်"
    "America/Cayman" "ကေမန်"
    "Asia/Vientiane" "ဗီယင်ကျန်း"
    "America/St_Lucia" "စိန့်လူစီယာ"
    "Europe/Vaduz" "ဗာဒူးစ်"
    "Africa/Maseru" "မာဆူရူး"
    "Europe/Luxembourg" "လူဇင်ဘတ်"
    "Europe/Monaco" "မိုနာကို"
    "Europe/Podgorica" "ပေါ့ဂိုရီကာ"
    "America/Marigot" "မာရီဂေါ့"
    "Indian/Antananarivo" "အန်တာနာနာရီးဘို"
    "Pacific/Majuro" "မာဂျူးရို"
    "Europe/Skopje" "စကော့ပ်ရာ"
    "Africa/Bamako" "ဘာမာကို"
    "Pacific/Saipan" "ဆိုင်ပန်"
    "Africa/Nouakchott" "နိုအာ့ခ်ရှော့တ်"
    "America/Montserrat" "မွန့်(တ်)ဆေးရတ်"
    "Africa/Blantyre" "ဘလန်တိုင်းရဲလ်"
    "Asia/Kuala_Lumpur" "ကွာလာလမ်ပူ"
    "Africa/Niamey" "ညာမဲယ်"
    "Europe/Amsterdam" "အမ်စတာဒမ်"
    "Europe/Oslo" "အော်စလို"
    "Asia/Muscat" "မတ်စ်ကက်တ်"
    "Indian/Reunion" "ရီယူနီယန်"
    "Africa/Kigali" "ကီဂါးလီ"
    "Indian/Mahe" "မာဟီ"
    "Europe/Stockholm" "စတော့ဟုမ်း"
    "Atlantic/St_Helena" "စိန့်ဟယ်လယ်နာ"
    "Europe/Ljubljana" "လူဘလီအားနား"
    "Arctic/Longyearbyen" "လောင်ရီယားဘရံ"
    "Europe/Bratislava" "ဘရာတီးစ်လားဗာ"
    "Africa/Freetown" "ဖရီးတောင်းန်"
    "Europe/San_Marino" "ဆန်မရီးနို"
    "Africa/Dakar" "ဒကျကား"
    "Africa/Mogadishu" "မော်ဂါဒီးသျုး"
    "America/Lower_Princes" "လိုအာပရင့်စ် ကွာတာ"
    "Africa/Mbabane" "ဘားဘာန်း"
    "Indian/Kerguelen" "ခါဂါလန်"
    "Africa/Lome" "လိုမီ"
    "America/Port_of_Spain" "ပို့တ် အော့ဖ် စပိန်"
    "Pacific/Funafuti" "ဖူနာဖူတီ"
    "Africa/Dar_es_Salaam" "ဒါရက်စ်ဆာလမ်"
    "Africa/Kampala" "ကမ်ပါလာ"
    "Pacific/Midway" "မစ်ဒ်ဝေး"
    "Pacific/Wake" "ဝိတ်ခ်"
    "Europe/Vatican" "ဗာတီကန်"
    "America/St_Vincent" "စိန့်ဗင်းဆင့်"
    "America/Tortola" "တောတိုလာ"
    "America/St_Thomas" "စိန့်သောမတ်စ်"
    "Pacific/Wallis" "ဝေါလီစ်"
    "Asia/Aden" "အာဒင်"
    "Indian/Mayotte" "မာယိုတဲ"
    "Africa/Lusaka" "လူစာကာ"
    "Africa/Harare" "ဟာရားရဲယ်"
};

// `common/main/ne.xml`: 418 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const NE: &str = exemplar_cities! {
    "Europe/Andorra" "आन्डोर्रा"
    "Asia/Dubai" "दुबही"
    "Asia/Kabul" "काबुल"
    "Europe/Tirane" "टिराने"
    "Asia/Yerevan" "येरेभान"
    "Antarctica/Casey" "केजे"
    "Antarctica/Davis" "डेभिस"
    "Antarctica/Mawson" "माउसन"
    "Antarctica/Palmer" "पाल्मेर"
    "Antarctica/Rothera" "रोथेरा"
    "Antarctica/Troll" "ट्रोल"
    "Antarctica/Vostok" "भास्टोक"
    "America/Argentina/Buenos_Aires" "ब्यनेश आयर्स"
    "America/Argentina/Cordoba" "कोरडोवा"
    "America/Argentina/Salta" "साल्टा"
    "America/Argentina/Jujuy" "जुजुई"
    "America/Argentina/Tucuman" "टुकुमान"
    "America/Argentina/Catamarca" "कातामार्का"
    "America/Argentina/La_Rioja" "ला रियोजा"
    "America/Argentina/San_Juan" "सान जुवान"
    "America/Argentina/Mendoza" "मेन्डोजा"
    "America/Argentina/San_Luis" "सान लुइस"
    "America/Argentina/Rio_Gallegos" "रियो ग्यालेगोस"
    "America/Argentina/Ushuaia" "उशुआइआ"
    "Pacific/Pago_Pago" "पागो पागो"
    "Europe/Vienna" "भियना"
    "Australia/Lord_Howe" "लर्ड होवे"
    "Antarctica/Macquarie" "मक्वारिई"
    "Australia/Hobart" "होभार्ट"
    "Australia/Melbourne" "मेल्बर्न"
    "Australia/Sydney" "सिड्नी"
    "Australia/Broken_Hill" "ब्रोकन हिल"
    "Australia/Brisbane" "ब्रिस्बेन"
    "Australia/Lindeman" "लिन्डेम्यान"
    "Australia/Adelaide" "एडेलेड"
    "Australia/Darwin" "डार्विन"
    "Australia/Perth" "पर्थ"
    "Australia/Eucla" "इयुक्ला"
    "Asia/Baku" "बाकु"
    "America/Barbados" "बार्बाडोस"
    "Asia/Dhaka" "ढाका"
    "Europe/Brussels" "ब्रसेल्स"
    "Europe/Sofia" "सोफिया"
    "Atlantic/Bermuda" "बर्मुडा"
    "America/La_Paz" "ला पाज"
    "America/Noronha" "नोरोन्हा"
    "America/Belem" "बेलेम"
    "America/Fortaleza" "फोर्टालेजा"
    "America/Recife" "रिसाइफ"
    "America/Araguaina" "आरागुवाना"
    "America/Maceio" "मासेइओ"
    "America/Bahia" "बाहिया"
    "America/Sao_Paulo" "साओ पाउलो"
    "America/Campo_Grande" "क्याम्पो ग्रान्डे"
    "America/Cuiaba" "क्युइआबा"
    "America/Santarem" "सान्टारेम"
    "America/Porto_Velho" "पोर्टो भेल्हो"
    "America/Boa_Vista" "बोआ भिष्टा"
    "America/Manaus" "मानाउस"
    "America/Eirunepe" "आइरनेपे"
    "America/Rio_Branco" "रियो ब्रान्को"
    "Asia/Thimphu" "थिम्पु"
    "Europe/Minsk" "मिन्स्क"
    "America/Belize" "बेलिज"
    "America/St_Johns" "सेन्ट जोन्स"
    "America/Halifax" "ह्यालिफ्याक्स"
    "America/Glace_Bay" "ग्लेस बे"
    "America/Moncton" "मोन्कटन"
    "America/Goose_Bay" "गुज बे"
    "America/Toronto" "टोरोन्टो"
    "America/Iqaluit" "इक्वालुइट"
    "America/Winnipeg" "विन्निपेग"
    "America/Resolute" "रिजोलुट"
    "America/Rankin_Inlet" "रान्किन इन्लेट"
    "America/Regina" "रेजिना"
    "America/Swift_Current" "स्विफ्ट करेन्ट"
    "America/Edmonton" "एड्मोन्टन"
    "America/Cambridge_Bay" "क्याम्ब्रिज बे"
    "America/Inuvik" "इनुभिक"
    "America/Vancouver" "भ्यानकोभर"
    "America/Dawson_Creek" "डसन क्रिक"
    "America/Fort_Nelson" "फोर्ट नेल्सन"
    "America/Whitehorse" "ह्वाइटहर्स"
    "America/Dawson" "डसन"
    "Europe/Zurich" "जुरिक"
    "Africa/Abidjan" "अविड्जान"
    "Pacific/Rarotonga" "राओतोंगा"
    "America/Santiago" "सान्टिआगो"
    "America/Coyhaique" "कोहाक्व"
    "America/Punta_Arenas" "पुन्टा अरिनाज"
    "Pacific/Easter" "इस्टर"
    "Asia/Shanghai" "सान्घाई"
    "Asia/Urumqi" "उरूम्की"
    "America/Bogota" "बोगोटा"
    "America/Costa_Rica" "कोष्टा रिका"
    "America/Havana" "हभाना"
    "Atlantic/Cape_Verde" "केप भर्डे"
    "Asia/Nicosia" "निकोसिया"
    "Asia/Famagusta" "फामागुस्ता"
    "Europe/Prague" "प्राग"
    "Europe/Berlin" "बर्लिन"
    "America/Santo_Domingo" "सान्टो डोमिङ्गो"
    "Africa/Algiers" "अल्जियर्स"
    "America/Guayaquil" "गुयाक्विल"
    "Pacific/Galapagos" "गलापागोस"
    "Europe/Tallinn" "ताल्लिन"
    "Africa/Cairo" "काइरो"
    "Africa/El_Aaiun" "एल् आइयुन"
    "Europe/Madrid" "म्याड्रिड"
    "Africa/Ceuta" "सेउटा"
    "Atlantic/Canary" "क्यानारी"
    "Europe/Helsinki" "हेल्सिन्की"
    "Pacific/Fiji" "फिजी"
    "Atlantic/Stanley" "स्ट्यान्ली"
    "Pacific/Kosrae" "कोस्राए"
    "Atlantic/Faroe" "फारोइ"
    "Europe/Paris" "पेरिस"
    "Europe/London" "लण्डन"
    "Asia/Tbilisi" "तिबिलिसी"
    "America/Cayenne" "कायेन्ने"
    "Europe/Gibraltar" "जिब्राल्टार"
    "America/Nuuk" "नूक"
    "America/Danmarkshavn" "डान्मार्कशाभन"
    "America/Scoresbysund" "ईट्टोक्कोरटूर्मिट"
    "America/Thule" "थुले"
    "Europe/Athens" "एथेन्स"
    "Atlantic/South_Georgia" "दक्षिण जर्जिया"
    "America/Guatemala" "ग्वाटेमाला"
    "Pacific/Guam" "गुवाम"
    "Africa/Bissau" "बिसाउ"
    "America/Guyana" "गुयाना"
    "Asia/Hong_Kong" "हङकङ"
    "America/Tegucigalpa" "टेगुसिगाल्पा"
    "America/Port-au-Prince" "पोर्ट-अउ-प्रिन्स"
    "Europe/Budapest" "बुडापेस्ट"
    "Asia/Jakarta" "जाकार्ता"
    "Asia/Pontianak" "पोन्टिआनाक"
    "Asia/Makassar" "माकास्सार"
    "Asia/Jayapura" "जयापुरा"
    "Europe/Dublin" "डब्लिन"
    "Asia/Jerusalem" "जेरुसलेम"
    "Asia/Kolkata" "कोलकाता"
    "Indian/Chagos" "चागोस"
    "Asia/Baghdad" "बगदाद"
    "Asia/Tehran" "तेहेरान"
    "Europe/Rome" "रोम"
    "America/Jamaica" "जमाइका"
    "Asia/Amman" "आम्मान"
    "Asia/Tokyo" "टोकियो"
    "Africa/Nairobi" "नाइरोबी"
    "Asia/Bishkek" "बिसकेक्"
    "Pacific/Tarawa" "तरवा"
    "Pacific/Kanton" "कान्टोन"
    "Pacific/Kiritimati" "किरितिमाटी"
    "Asia/Pyongyang" "प्योङयाङ"
    "Asia/Seoul" "सिओल"
    "Asia/Almaty" "आल्माटी"
    "Asia/Qyzylorda" "किजिलोर्डा"
    "Asia/Qostanay" "कस्टाने"
    "Asia/Aqtobe" "आक्टोब"
    "Asia/Aqtau" "आक्टाउ"
    "Asia/Atyrau" "अटिराउ"
    "Asia/Oral" "ओरल"
    "Asia/Beirut" "बेईरुट"
    "Asia/Colombo" "कोलम्बो"
    "Africa/Monrovia" "मोन्रोभिया"
    "Europe/Vilnius" "भिल्निअस"
    "Europe/Riga" "रिगा"
    "Africa/Tripoli" "त्रिपोली"
    "Africa/Casablanca" "कासाब्लान्का"
    "Europe/Chisinau" "चिसिनाउ"
    "Pacific/Kwajalein" "क्वाजालेइन"
    "Asia/Yangon" "रान्गुन"
    "Asia/Ulaanbaatar" "उलानबटार"
    "Asia/Hovd" "होभ्ड"
    "Asia/Macau" "मकाउ"
    "America/Martinique" "मार्टिनिक"
    "Europe/Malta" "माल्टा"
    "Indian/Mauritius" "मउरिटिअस"
    "Indian/Maldives" "माल्दिभ्स"
    "America/Mexico_City" "मेक्सिको सिटी"
    "America/Cancun" "कानकुन"
    "America/Merida" "मेरिडा"
    "America/Monterrey" "मोन्टेर्रे"
    "America/Matamoros" "माट्तामोरोस्"
    "America/Chihuahua" "चिहुवाहुवा"
    "America/Ciudad_Juarez" "जुआरेज सहर"
    "America/Ojinaga" "ओजिनागा"
    "America/Mazatlan" "माजाट्लान"
    "America/Bahia_Banderas" "बाहिया बान्डेराश"
    "America/Hermosillo" "हेर्मोसिल्लो"
    "America/Tijuana" "तिजुआना"
    "Asia/Kuching" "कुचिङ"
    "Africa/Maputo" "मापुतो"
    "Africa/Windhoek" "विन्डहोएक"
    "Pacific/Noumea" "नोउमेअ"
    "Pacific/Norfolk" "नरफोल्क"
    "Africa/Lagos" "लागोस"
    "America/Managua" "मानागुवा"
    "Asia/Kathmandu" "काठमाण्डौं"
    "Pacific/Nauru" "नाउरु"
    "Pacific/Niue" "निउई"
    "Pacific/Auckland" "अकल्यान्ड"
    "Pacific/Chatham" "चाथाम"
    "America/Panama" "पानामा"
    "America/Lima" "लिमा"
    "Pacific/Tahiti" "ताहिती"
    "Pacific/Marquesas" "मार्केसास"
    "Pacific/Gambier" "ग्याम्बियर"
    "Pacific/Port_Moresby" "पोर्ट मोरेस्बी"
    "Pacific/Bougainville" "बुगेनभिल्ले"
    "Asia/Manila" "मनिला"
    "Asia/Karachi" "कराची"
    "Europe/Warsaw" "वारसअ"
    "America/Miquelon" "मिक्विलन"
    "Pacific/Pitcairn" "पितकाईरन"
    "America/Puerto_Rico" "प्युर्टो रिको"
    "Asia/Gaza" "गाजा"
    "Asia/Hebron" "हिब्रोन"
    "Europe/Lisbon" "लिस्बोन"
    "Atlantic/Madeira" "माडेइरा"
    "Atlantic/Azores" "आजोर्स"
    "Pacific/Palau" "पलाउ"
    "America/Asuncion" "असन्सियन"
    "Asia/Qatar" "कतार"
    "Europe/Bucharest" "वुचारेस्ट"
    "Europe/Belgrade" "बेलग्रेड"
    "Europe/Kaliningrad" "कालिनिनग्राद"
    "Europe/Moscow" "मस्को"
    "Europe/Simferopol" "सिम्फेरोपोल"
    "Europe/Kirov" "किरोभ"
    "Europe/Volgograd" "भोल्गोग्राद"
    "Europe/Astrakhan" "अस्त्रखान"
    "Europe/Saratov" "साराटोभ"
    "Europe/Ulyanovsk" "उल्यानोभ्स्क"
    "Europe/Samara" "सामारा"
    "Asia/Yekaterinburg" "एकटरिनबुर्ग"
    "Asia/Omsk" "ओम्स्क"
    "Asia/Novosibirsk" "नोबोसिबिर्स्क"
    "Asia/Barnaul" "बरनौल"
    "Asia/Tomsk" "टोम्स्क"
    "Asia/Novokuznetsk" "नेभोकुजनेस्क"
    "Asia/Krasnoyarsk" "क्रास्नोयार्स्क"
    "Asia/Irkutsk" "इर्कुत्स्क"
    "Asia/Chita" "चिता"
    "Asia/Yakutsk" "याकुत्स्क"
    "Asia/Khandyga" "खान्दिगा"
    "Asia/Vladivostok" "भ्लाडिभास्टोक"
    "Asia/Ust-Nera" "उस्ट-नेरा"
    "Asia/Magadan" "मागाडान"
    "Asia/Sakhalin" "साखालिन"
    "Asia/Srednekolymsk" "स्रेद्निकोलिम्स्क"
    "Asia/Kamchatka" "कामचट्का"
    "Asia/Anadyr" "आनाडियर"
    "Asia/Riyadh" "रियाद"
    "Pacific/Guadalcanal" "गुअडालकनाल"
    "Africa/Khartoum" "खार्टउम"
    "Asia/Singapore" "सिंगापुर"
    "America/Paramaribo" "पारामारिवो"
    "Africa/Juba" "जुबा"
    "Africa/Sao_Tome" "साओ टोमे"
    "America/El_Salvador" "एल् साल्भाडोर"
    "Asia/Damascus" "दामास्कस्"
    "America/Grand_Turk" "ग्रान्ड टर्क"
    "Africa/Ndjamena" "एन्‌जामेना"
    "Asia/Bangkok" "बैंकक"
    "Asia/Dushanbe" "दस्सान्बे"
    "Pacific/Fakaofo" "फाकाओफो"
    "Asia/Dili" "दिल्ली"
    "Asia/Ashgabat" "अस्काबाट"
    "Africa/Tunis" "टुनिस"
    "Pacific/Tongatapu" "टंगातपु"
    "Europe/Istanbul" "ईस्टानबुल"
    "Asia/Taipei" "ताईपे"
    "Europe/Kyiv" "किभ"
    "America/New_York" "न्युयोर्क"
    "America/Detroit" "डिट्रोइट"
    "America/Kentucky/Louisville" "लुइसभिल्ले"
    "America/Kentucky/Monticello" "मोन्टिसेल्लो,केन्टकी"
    "America/Indiana/Indianapolis" "इन्डियानापोलिस"
    "America/Indiana/Vincennes" "भिन्सेन्स"
    "America/Indiana/Winamac" "विनामाक, इन्डियाना"
    "America/Indiana/Marengo" "मारेन्गो, इन्डियाना"
    "America/Indiana/Petersburg" "पिटर्सबर्ग, इन्डियाना"
    "America/Indiana/Vevay" "भेभे, इन्डियाना"
    "America/Chicago" "शिकागो"
    "America/Indiana/Tell_City" "टेल सिटी, इन्डियाना"
    "America/Indiana/Knox" "नोक्स इन्डियाना"
    "America/Menominee" "मेनोमिनी"
    "America/North_Dakota/Center" "उत्तर डाकोटा, केन्द्र"
    "America/North_Dakota/New_Salem" "नयाँ सालेम, उत्तर डाकोटा"
    "America/North_Dakota/Beulah" "बेउला, उत्तर डाकोटा"
    "America/Denver" "डेन्भर"
    "America/Boise" "बोइज"
    "America/Phoenix" "फिनिक्स"
    "America/Los_Angeles" "लस् एन्जेलस"
    "America/Anchorage" "एङ्कोरेज"
    "America/Juneau" "जुनिउ"
    "America/Sitka" "सिट्का"
    "America/Metlakatla" "मेट्लाक्टला"
    "America/Yakutat" "याकुटाट"
    "America/Nome" "नोम"
    "America/Adak" "आडाक"
    "Pacific/Honolulu" "होनोलुलु"
    "America/Montevideo" "मोन्टेभिडियो"
    "Asia/Samarkand" "समारकण्ड"
    "Asia/Tashkent" "तास्केन्ट"
    "America/Caracas" "काराकास"
    "Asia/Ho_Chi_Minh" "हो ची मिन्ह शहर"
    "Pacific/Efate" "ईफाते"
    "Pacific/Apia" "अपिया"
    "Africa/Johannesburg" "जोहानेसवर्ग"
    "America/Antigua" "एन्टिगुवा"
    "America/Anguilla" "एङ्ग्विल्ला"
    "Africa/Luanda" "लुवान्डा"
    "Antarctica/McMurdo" "माकमुर्डो"
    "Antarctica/DumontDUrville" "दुमोन्ट डि उर्भेल्ले"
    "Antarctica/Syowa" "सिओआ"
    "America/Aruba" "अरुबा"
    "Europe/Mariehamn" "म्यारिह्याम्न"
    "Europe/Sarajevo" "साराजेभो"
    "Africa/Ouagadougou" "औआगाडौगौ"
    "Asia/Bahrain" "बहराईन"
    "Africa/Bujumbura" "बुजुम्बुरा"
    "Africa/Porto-Novo" "पोर्टो-नोभो"
    "America/St_Barthelemy" "सेन्ट बार्थेलेमी"
    "Asia/Brunei" "ब्रुनाइ"
    "America/Kralendijk" "कालेन्देजिक"
    "America/Nassau" "नास्साउ"
    "Africa/Gaborone" "गावोरोन"
    "America/Blanc-Sablon" "ब्लान्क-साब्लोन"
    "America/Atikokan" "एटिकोकान"
    "America/Creston" "क्रेस्टन"
    "Indian/Cocos" "कोकोस"
    "Africa/Kinshasa" "किन्शासा"
    "Africa/Lubumbashi" "लुबुम्बासी"
    "Africa/Bangui" "बाङ्गुवी"
    "Africa/Brazzaville" "ब्राजाभिल्ले"
    "Africa/Douala" "डोउआला"
    "America/Curacao" "कुराकाओ"
    "Indian/Christmas" "ख्रिस्टमस"
    "Europe/Busingen" "बुसिन्नगन"
    "Africa/Djibouti" "जिबौंटी"
    "Europe/Copenhagen" "कोपेनह्यागन"
    "America/Dominica" "डोमिनिका"
    "Africa/Asmara" "आस्मारा"
    "Africa/Addis_Ababa" "एड्डिस आबाबा"
    "Pacific/Chuuk" "चूक"
    "Pacific/Pohnpei" "पोनापे"
    "Africa/Libreville" "लिब्रेभिल्ले"
    "America/Grenada" "ग्रेनाडा"
    "Europe/Guernsey" "गुएर्नसे"
    "Africa/Accra" "अक्रा"
    "Africa/Banjul" "बन्जुल"
    "Africa/Conakry" "कोनाक्री"
    "America/Guadeloupe" "ग्वाडेलुप"
    "Africa/Malabo" "मालाबो"
    "Europe/Zagreb" "जाग्रेब"
    "Europe/Isle_of_Man" "इजल अफ् म्यान"
    "Atlantic/Reykjavik" "रेक्जाभिक"
    "Europe/Jersey" "जर्सी"
    "Asia/Phnom_Penh" "फेनोम फेन"
    "Indian/Comoro" "कोमोरो"
    "America/St_Kitts" "सेन्ट् किट्स"
    "Asia/Kuwait" "कुवेत"
    "America/Cayman" "केम्यान"
    "Asia/Vientiane" "भियन्तिन"
    "America/St_Lucia" "सेन्ट लुसिया"
    "Europe/Vaduz" "भाडुज"
    "Africa/Maseru" "मासेरू"
    "Europe/Luxembourg" "लक्जेम्वर्ग"
    "Europe/Monaco" "मोनाको"
    "Europe/Podgorica" "पड्गोरिका"
    "America/Marigot" "म्यारिगट"
    "Indian/Antananarivo" "अन्टानारिभो"
    "Pacific/Majuro" "माजुरो"
    "Europe/Skopje" "स्कोपजे"
    "Africa/Bamako" "बोमाको"
    "Pacific/Saipan" "साईपन"
    "Africa/Nouakchott" "नोउआकचोट"
    "America/Montserrat" "मन्टसेर्राट"
    "Africa/Blantyre" "ब्लान्टायर"
    "Asia/Kuala_Lumpur" "कुआ लाम्पुर"
    "Africa/Niamey" "नायमे"
    "Europe/Amsterdam" "एम्स्ट्र्डम"
    "Europe/Oslo" "ओस्लो"
    "Asia/Muscat" "मस्क्याट"
    "Indian/Reunion" "रियुनियन"
    "Africa/Kigali" "किगाली"
    "Indian/Mahe" "माहे"
    "Europe/Stockholm" "स्टकहोल्म"
    "Atlantic/St_Helena" "सेन्ट हेलेना"
    "Europe/Ljubljana" "लजुबिजाना"
    "Arctic/Longyearbyen" "लङयिअरबाइएन"
    "Europe/Bratislava" "ब्राटिस्लाभा"
    "Africa/Freetown" "फ्रिटाउन"
    "Europe/San_Marino" "सान मारिनो"
    "Africa/Dakar" "डाकार"
    "Africa/Mogadishu" "मोगाडिशु"
    "America/Lower_Princes" "लोअर प्रिन्स्स क्वार्टर"
    "Africa/Mbabane" "एमबाबेन"
    "Indian/Kerguelen" "केर्गुएलेन"
    "Africa/Lome" "लोम"
    "America/Port_of_Spain" "पोर्ट अफ् स्पेन"
    "Pacific/Funafuti" "फुनाफुति"
    "Africa/Dar_es_Salaam" "डार एस् सलाम"
    "Africa/Kampala" "काम्पाला"
    "Pacific/Midway" "मिडवे"
    "Pacific/Wake" "वेक"
    "Europe/Vatican" "भ्याटिकन"
    "America/St_Vincent" "सेन्ट भिन्सेन्ट"
    "America/Tortola" "टार्टोला"
    "America/St_Thomas" "सेन्ट थोमस"
    "Pacific/Wallis" "वालिस"
    "Asia/Aden" "एडेन"
    "Indian/Mayotte" "मायोट्टे"
    "Africa/Lusaka" "लुसाका"
    "Africa/Harare" "हरारे"
};

// `common/main/nl.xml`: 81 of the 418 zones named, 337 inherited.
#[cfg(feature = "localized-exemplar-cities")]
const NL: &str = exemplar_cities! {
    "Europe/Andorra" inherited
    "Asia/Dubai" inherited
    "Asia/Kabul" inherited
    "Europe/Tirane" inherited
    "Asia/Yerevan" "Jerevan"
    "Antarctica/Casey" inherited
    "Antarctica/Davis" inherited
    "Antarctica/Mawson" inherited
    "Antarctica/Palmer" inherited
    "Antarctica/Rothera" inherited
    "Antarctica/Troll" inherited
    "Antarctica/Vostok" inherited
    "America/Argentina/Buenos_Aires" inherited
    "America/Argentina/Cordoba" inherited
    "America/Argentina/Salta" inherited
    "America/Argentina/Jujuy" inherited
    "America/Argentina/Tucuman" inherited
    "America/Argentina/Catamarca" inherited
    "America/Argentina/La_Rioja" inherited
    "America/Argentina/San_Juan" inherited
    "America/Argentina/Mendoza" inherited
    "America/Argentina/San_Luis" inherited
    "America/Argentina/Rio_Gallegos" inherited
    "America/Argentina/Ushuaia" inherited
    "Pacific/Pago_Pago" inherited
    "Europe/Vienna" "Wenen"
    "Australia/Lord_Howe" inherited
    "Antarctica/Macquarie" inherited
    "Australia/Hobart" inherited
    "Australia/Melbourne" inherited
    "Australia/Sydney" inherited
    "Australia/Broken_Hill" inherited
    "Australia/Brisbane" inherited
    "Australia/Lindeman" inherited
    "Australia/Adelaide" inherited
    "Australia/Darwin" inherited
    "Australia/Perth" inherited
    "Australia/Eucla" inherited
    "Asia/Baku" "Bakoe"
    "America/Barbados" inherited
    "Asia/Dhaka" inherited
    "Europe/Brussels" "Brussel"
    "Europe/Sofia" inherited
    "Atlantic/Bermuda" inherited
    "America/La_Paz" inherited
    "America/Noronha" inherited
    "America/Belem" inherited
    "America/Fortaleza" inherited
    "America/Recife" inherited
    "America/Araguaina" inherited
    "America/Maceio" inherited
    "America/Bahia" inherited
    "America/Sao_Paulo" inherited
    "America/Campo_Grande" inherited
    "America/Cuiaba" inherited
    "America/Santarem" inherited
    "America/Porto_Velho" inherited
    "America/Boa_Vista" inherited
    "America/Manaus" inherited
    "America/Eirunepe" inherited
    "America/Rio_Branco" inherited
    "Asia/Thimphu" inherited
    "Europe/Minsk" inherited
    "America/Belize" inherited
    "America/St_Johns" "Saint John’s"
    "America/Halifax" inherited
    "America/Glace_Bay" inherited
    "America/Moncton" inherited
    "America/Goose_Bay" inherited
    "America/Toronto" inherited
    "America/Iqaluit" inherited
    "America/Winnipeg" inherited
    "America/Resolute" inherited
    "America/Rankin_Inlet" inherited
    "America/Regina" inherited
    "America/Swift_Current" inherited
    "America/Edmonton" inherited
    "America/Cambridge_Bay" inherited
    "America/Inuvik" inherited
    "America/Vancouver" inherited
    "America/Dawson_Creek" inherited
    "America/Fort_Nelson" inherited
    "America/Whitehorse" inherited
    "America/Dawson" inherited
    "Europe/Zurich" "Zürich"
    "Africa/Abidjan" inherited
    "Pacific/Rarotonga" inherited
    "America/Santiago" inherited
    "America/Coyhaique" inherited
    "America/Punta_Arenas" inherited
    "Pacific/Easter" "Paaseiland"
    "Asia/Shanghai" "Sjanghai"
    "Asia/Urumqi" inherited
    "America/Bogota" inherited
    "America/Costa_Rica" inherited
    "America/Havana" inherited
    "Atlantic/Cape_Verde" "Kaapverdië"
    "Asia/Nicosia" inherited
    "Asia/Famagusta" inherited
    "Europe/Prague" "Praag"
    "Europe/Berlin" "Berlijn"
    "America/Santo_Domingo" inherited
    "Africa/Algiers" inherited
    "America/Guayaquil" inherited
    "Pacific/Galapagos" inherited
    "Europe/Tallinn" inherited
    "Africa/Cairo" "Caïro"
    "Africa/El_Aaiun" inherited
    "Europe/Madrid" inherited
    "Africa/Ceuta" inherited
    "Atlantic/Canary" "Canarische Eilanden"
    "Europe/Helsinki" inherited
    "Pacific/Fiji" inherited
    "Atlantic/Stanley" inherited
    "Pacific/Kosrae" inherited
    "Atlantic/Faroe" "Faeröer"
    "Europe/Paris" "Parijs"
    "Europe/London" "Londen"
    "Asia/Tbilisi" inherited
    "America/Cayenne" inherited
    "Europe/Gibraltar" inherited
    "America/Nuuk" inherited
    "America/Danmarkshavn" inherited
    "America/Scoresbysund" inherited
    "America/Thule" inherited
    "Europe/Athens" "Athene"
    "Atlantic/South_Georgia" "Zuid-Georgia"
    "America/Guatemala" inherited
    "Pacific/Guam" inherited
    "Africa/Bissau" inherited
    "America/Guyana" inherited
    "Asia/Hong_Kong" "Hongkong"
    "America/Tegucigalpa" inherited
    "America/Port-au-Prince" inherited
    "Europe/Budapest" "Boedapest"
    "Asia/Jakarta" inherited
    "Asia/Pontianak" inherited
    "Asia/Makassar" inherited
    "Asia/Jayapura" inherited
    "Europe/Dublin" inherited
    "Asia/Jerusalem" "Jeruzalem"
    "Asia/Kolkata" "Calcutta"
    "Indian/Chagos" "Chagosarchipel"
    "Asia/Baghdad" "Bagdad"
    "Asia/Tehran" "Teheran"
    "Europe/Rome" inherited
    "America/Jamaica" inherited
    "Asia/Amman" inherited
    "Asia/Tokyo" "Tokio"
    "Africa/Nairobi" inherited
    "Asia/Bishkek" "Bisjkek"
    "Pacific/Tarawa" inherited
    "Pacific/Kanton" "Kanton"
    "Pacific/Kiritimati" inherited
    "Asia/Pyongyang" inherited
    "Asia/Seoul" inherited
    "Asia/Almaty" "Alma-Ata"
    "Asia/Qyzylorda" inherited
    "Asia/Qostanay" inherited
    "Asia/Aqtobe" "Aqtöbe"
    "Asia/Aqtau" inherited
    "Asia/Atyrau" "Atıraw"
    "Asia/Oral" inherited
    "Asia/Beirut" "Beiroet"
    "Asia/Colombo" inherited
    "Africa/Monrovia" inherited
    "Europe/Vilnius" inherited
    "Europe/Riga" inherited
    "Africa/Tripoli" inherited
    "Africa/Casablanca" inherited
    "Europe/Chisinau" inherited
    "Pacific/Kwajalein" inherited
    "Asia/Yangon" "Rangoon"
    "Asia/Ulaanbaatar" inherited
    "Asia/Hovd" inherited
    "Asia/Macau" "Macau"
    "America/Martinique" inherited
    "Europe/Malta" inherited
    "Indian/Mauritius" inherited
    "Indian/Maldives" "Maldiven"
    "America/Mexico_City" "Mexico-Stad"
    "America/Cancun" "Cancun"
    "America/Merida" inherited
    "America/Monterrey" inherited
    "America/Matamoros" inherited
    "America/Chihuahua" inherited
    "America/Ciudad_Juarez" inherited
    "America/Ojinaga" inherited
    "America/Mazatlan" inherited
    "America/Bahia_Banderas" inherited
    "America/Hermosillo" inherited
    "America/Tijuana" inherited
    "Asia/Kuching" inherited
    "Africa/Maputo" inherited
    "Africa/Windhoek" inherited
    "Pacific/Noumea" inherited
    "Pacific/Norfolk" inherited
    "Africa/Lagos" inherited
    "America/Managua" inherited
    "Asia/Kathmandu" inherited
    "Pacific/Nauru" inherited
    "Pacific/Niue" inherited
    "Pacific/Auckland" inherited
    "Pacific/Chatham" inherited
    "America/Panama" inherited
    "America/Lima" inherited
    "Pacific/Tahiti" inherited
    "Pacific/Marquesas" "Marquesaseilanden"
    "Pacific/Gambier" "Îles Gambier"
    "Pacific/Port_Moresby" inherited
    "Pacific/Bougainville" inherited
    "Asia/Manila" "Manilla"
    "Asia/Karachi" inherited
    "Europe/Warsaw" "Warschau"
    "America/Miquelon" inherited
    "Pacific/Pitcairn" inherited
    "America/Puerto_Rico" inherited
    "Asia/Gaza" inherited
    "Asia/Hebron" inherited
    "Europe/Lisbon" "Lissabon"
    "Atlantic/Madeira" inherited
    "Atlantic/Azores" "Azoren"
    "Pacific/Palau" inherited
    "America/Asuncion" inherited
    "Asia/Qatar" inherited
    "Europe/Bucharest" "Boekarest"
    "Europe/Belgrade" "Belgrado"
    "Europe/Kaliningrad" inherited
    "Europe/Moscow" "Moskou"
    "Europe/Simferopol" inherited
    "Europe/Kirov" inherited
    "Europe/Volgograd" "Wolgograd"
    "Europe/Astrakhan" inherited
    "Europe/Saratov" inherited
    "Europe/Ulyanovsk" inherited
    "Europe/Samara" inherited
    "Asia/Yekaterinburg" "Jekaterinenburg"
    "Asia/Omsk" inherited
    "Asia/Novosibirsk" inherited
    "Asia/Barnaul" inherited
    "Asia/Tomsk" inherited
    "Asia/Novokuznetsk" inherited
    "Asia/Krasnoyarsk" "Krasnojarsk"
    "Asia/Irkutsk" "Irkoetsk"
    "Asia/Chita" inherited
    "Asia/Yakutsk" "Jakoetsk"
    "Asia/Khandyga" inherited
    "Asia/Vladivostok" inherited
    "Asia/Ust-Nera" inherited
    "Asia/Magadan" inherited
    "Asia/Sakhalin" "Sachalin"
    "Asia/Srednekolymsk" inherited
    "Asia/Kamchatka" "Kamtsjatka"
    "Asia/Anadyr" inherited
    "Asia/Riyadh" "Riyad"
    "Pacific/Guadalcanal" inherited
    "Africa/Khartoum" "Khartoem"
    "Asia/Singapore" inherited
    "America/Paramaribo" inherited
    "Africa/Juba" inherited
    "Africa/Sao_Tome" "Sao Tomé"
    "America/El_Salvador" inherited
    "Asia/Damascus" inherited
    "America/Grand_Turk" inherited
    "Africa/Ndjamena" inherited
    "Asia/Bangkok" inherited
    "Asia/Dushanbe" "Doesjanbe"
    "Pacific/Fakaofo" inherited
    "Asia/Dili" inherited
    "Asia/Ashgabat" "Asjchabad"
    "Africa/Tunis" inherited
    "Pacific/Tongatapu" inherited
    "Europe/Istanbul" "Istanboel"
    "Asia/Taipei" inherited
    "Europe/Kyiv" "Kiev"
    "America/New_York" inherited
    "America/Detroit" inherited
    "America/Kentucky/Louisville" inherited
    "America/Kentucky/Monticello" inherited
    "America/Indiana/Indianapolis" inherited
    "America/Indiana/Vincennes" inherited
    "America/Indiana/Winamac" inherited
    "America/Indiana/Marengo" inherited
    "America/Indiana/Petersburg" inherited
    "America/Indiana/Vevay" inherited
    "America/Chicago" inherited
    "America/Indiana/Tell_City" inherited
    "America/Indiana/Knox" inherited
    "America/Menominee" inherited
    "America/North_Dakota/Center" "Center, Noord-Dakota"
    "America/North_Dakota/New_Salem" "New Salem, Noord-Dakota"
    "America/North_Dakota/Beulah" "Beulah, Noord-Dakota"
    "America/Denver" inherited
    "America/Boise" inherited
    "America/Phoenix" inherited
    "America/Los_Angeles" inherited
    "America/Anchorage" inherited
    "America/Juneau" inherited
    "America/Sitka" inherited
    "America/Metlakatla" inherited
    "America/Yakutat" inherited
    "America/Nome" inherited
    "America/Adak" inherited
    "Pacific/Honolulu" "Honolulu"
    "America/Montevideo" inherited
    "Asia/Samarkand" inherited
    "Asia/Tashkent" "Tasjkent"
    "America/Caracas" inherited
    "Asia/Ho_Chi_Minh" "Ho Chi Minhstad"
    "Pacific/Efate" inherited
    "Pacific/Apia" inherited
    "Africa/Johannesburg" inherited
    "America/Antigua" inherited
    "America/Anguilla" inherited
    "Africa/Luanda" inherited
    "Antarctica/McMurdo" inherited
    "Antarctica/DumontDUrville" inherited
    "Antarctica/Syowa" inherited
    "America/Aruba" inherited
    "Europe/Mariehamn" inherited
    "Europe/Sarajevo" inherited
    "Africa/Ouagadougou" inherited
    "Asia/Bahrain" "Bahrein"
    "Africa/Bujumbura" inherited
    "Africa/Porto-Novo" inherited
    "America/St_Barthelemy" "Saint-Barthélemy"
    "Asia/Brunei" inherited
    "America/Kralendijk" inherited
    "America/Nassau" inherited
    "Africa/Gaborone" inherited
    "America/Blanc-Sablon" inherited
    "America/Atikokan" inherited
    "America/Creston" inherited
    "Indian/Cocos" "Cocoseilanden"
    "Africa/Kinshasa" inherited
    "Africa/Lubumbashi" inherited
    "Africa/Bangui" inherited
    "Africa/Brazzaville" inherited
    "Africa/Douala" inherited
    "America/Curacao" inherited
    "Indian/Christmas" "Christmaseiland"
    "Europe/Busingen" inherited
    "Africa/Djibouti" inherited
    "Europe/Copenhagen" "Kopenhagen"
    "America/Dominica" inherited
    "Africa/Asmara" inherited
    "Africa/Addis_Ababa" "Addis Abeba"
    "Pacific/Chuuk" inherited
    "Pacific/Pohnpei" inherited
    "Africa/Libreville" inherited
    "America/Grenada" inherited
    "Europe/Guernsey" inherited
    "Africa/Accra" inherited
    "Africa/Banjul" inherited
    "Africa/Conakry" inherited
    "America/Guadeloupe" inherited
    "Africa/Malabo" inherited
    "Europe/Zagreb" inherited
    "Europe/Isle_of_Man" inherited
    "Atlantic/Reykjavik" inherited
    "Europe/Jersey" inherited
    "Asia/Phnom_Penh" inherited
    "Indian/Comoro" inherited
    "America/St_Kitts" "Saint Kitts"
    "Asia/Kuwait" "Koeweit"
    "America/Cayman" inherited
    "Asia/Vientiane" inherited
    "America/St_Lucia" "Saint Lucia"
    "Europe/Vaduz" inherited
    "Africa/Maseru" inherited
    "Europe/Luxembourg" "Luxemburg"
    "Europe/Monaco" inherited
    "Europe/Podgorica" inherited
    "America/Marigot" inherited
    "Indian/Antananarivo" inherited
    "Pacific/Majuro" inherited
    "Europe/Skopje" inherited
    "Africa/Bamako" inherited
    "Pacific/Saipan" inherited
    "Africa/Nouakchott" inherited
    "America/Montserrat" inherited
    "Africa/Blantyre" inherited
    "Asia/Kuala_Lumpur" inherited
    "Africa/Niamey" inherited
    "Europe/Amsterdam" inherited
    "Europe/Oslo" inherited
    "Asia/Muscat" inherited
    "Indian/Reunion" inherited
    "Africa/Kigali" inherited
    "Indian/Mahe" inherited
    "Europe/Stockholm" inherited
    "Atlantic/St_Helena" "Sint-Helena"
    "Europe/Ljubljana" inherited
    "Arctic/Longyearbyen" inherited
    "Europe/Bratislava" inherited
    "Africa/Freetown" inherited
    "Europe/San_Marino" inherited
    "Africa/Dakar" inherited
    "Africa/Mogadishu" inherited
    "America/Lower_Princes" "Beneden Prinsen Kwartier"
    "Africa/Mbabane" inherited
    "Indian/Kerguelen" inherited
    "Africa/Lome" inherited
    "America/Port_of_Spain" inherited
    "Pacific/Funafuti" inherited
    "Africa/Dar_es_Salaam" inherited
    "Africa/Kampala" inherited
    "Pacific/Midway" inherited
    "Pacific/Wake" inherited
    "Europe/Vatican" "Vaticaanstad"
    "America/St_Vincent" "Saint Vincent"
    "America/Tortola" inherited
    "America/St_Thomas" "Saint Thomas"
    "Pacific/Wallis" inherited
    "Asia/Aden" inherited
    "Indian/Mayotte" inherited
    "Africa/Lusaka" inherited
    "Africa/Harare" inherited
};

// `common/main/pl.xml`: 171 of the 418 zones named, 247 inherited.
#[cfg(feature = "localized-exemplar-cities")]
const PL: &str = exemplar_cities! {
    "Europe/Andorra" "Andora"
    "Asia/Dubai" "Dubaj"
    "Asia/Kabul" inherited
    "Europe/Tirane" inherited
    "Asia/Yerevan" "Erywań"
    "Antarctica/Casey" inherited
    "Antarctica/Davis" inherited
    "Antarctica/Mawson" inherited
    "Antarctica/Palmer" inherited
    "Antarctica/Rothera" inherited
    "Antarctica/Troll" inherited
    "Antarctica/Vostok" "Wostok"
    "America/Argentina/Buenos_Aires" inherited
    "America/Argentina/Cordoba" inherited
    "America/Argentina/Salta" inherited
    "America/Argentina/Jujuy" inherited
    "America/Argentina/Tucuman" inherited
    "America/Argentina/Catamarca" inherited
    "America/Argentina/La_Rioja" inherited
    "America/Argentina/San_Juan" inherited
    "America/Argentina/Mendoza" inherited
    "America/Argentina/San_Luis" inherited
    "America/Argentina/Rio_Gallegos" inherited
    "America/Argentina/Ushuaia" inherited
    "Pacific/Pago_Pago" inherited
    "Europe/Vienna" "Wiedeń"
    "Australia/Lord_Howe" inherited
    "Antarctica/Macquarie" inherited
    "Australia/Hobart" inherited
    "Australia/Melbourne" inherited
    "Australia/Sydney" inherited
    "Australia/Broken_Hill" inherited
    "Australia/Brisbane" inherited
    "Australia/Lindeman" inherited
    "Australia/Adelaide" inherited
    "Australia/Darwin" inherited
    "Australia/Perth" inherited
    "Australia/Eucla" inherited
    "Asia/Baku" inherited
    "America/Barbados" inherited
    "Asia/Dhaka" inherited
    "Europe/Brussels" "Bruksela"
    "Europe/Sofia" inherited
    "Atlantic/Bermuda" "Bermudy"
    "America/La_Paz" inherited
    "America/Noronha" inherited
    "America/Belem" inherited
    "America/Fortaleza" inherited
    "America/Recife" inherited
    "America/Araguaina" inherited
    "America/Maceio" inherited
    "America/Bahia" "Salvador"
    "America/Sao_Paulo" inherited
    "America/Campo_Grande" inherited
    "America/Cuiaba" inherited
    "America/Santarem" inherited
    "America/Porto_Velho" inherited
    "America/Boa_Vista" inherited
    "America/Manaus" inherited
    "America/Eirunepe" inherited
    "America/Rio_Branco" inherited
    "Asia/Thimphu" inherited
    "Europe/Minsk" "Mińsk"
    "America/Belize" inherited
    "America/St_Johns" inherited
    "America/Halifax" inherited
    "America/Glace_Bay" inherited
    "America/Moncton" inherited
    "America/Goose_Bay" inherited
    "America/Toronto" inherited
    "America/Iqaluit" inherited
    "America/Winnipeg" inherited
    "America/Resolute" inherited
    "America/Rankin_Inlet" inherited
    "America/Regina" inherited
    "America/Swift_Current" inherited
    "America/Edmonton" inherited
    "America/Cambridge_Bay" inherited
    "America/Inuvik" inherited
    "America/Vancouver" inherited
    "America/Dawson_Creek" inherited
    "America/Fort_Nelson" inherited
    "America/Whitehorse" inherited
    "America/Dawson" inherited
    "Europe/Zurich" "Zurych"
    "Africa/Abidjan" "Abidżan"
    "Pacific/Rarotonga" inherited
    "America/Santiago" inherited
    "America/Coyhaique" inherited
    "America/Punta_Arenas" inherited
    "Pacific/Easter" "Wyspa Wielkanocna"
    "Asia/Shanghai" "Szanghaj"
    "Asia/Urumqi" "Urumczi"
    "America/Bogota" inherited
    "America/Costa_Rica" "Kostaryka"
    "America/Havana" "Hawana"
    "Atlantic/Cape_Verde" "Republika Zielonego Przylądka"
    "Asia/Nicosia" "Nikozja"
    "Asia/Famagusta" inherited
    "Europe/Prague" "Praga"
    "Europe/Berlin" inherited
    "America/Santo_Domingo" inherited
    "Africa/Algiers" "Algier"
    "America/Guayaquil" inherited
    "Pacific/Galapagos" inherited
    "Europe/Tallinn" "Tallin"
    "Africa/Cairo" "Kair"
    "Africa/El_Aaiun" "Al-Ujun"
    "Europe/Madrid" "Madryt"
    "Africa/Ceuta" inherited
    "Atlantic/Canary" "Wyspy Kanaryjskie"
    "Europe/Helsinki" inherited
    "Pacific/Fiji" "Fidżi"
    "Atlantic/Stanley" inherited
    "Pacific/Kosrae" inherited
    "Atlantic/Faroe" "Wyspy Owcze"
    "Europe/Paris" "Paryż"
    "Europe/London" "Londyn"
    "Asia/Tbilisi" inherited
    "America/Cayenne" "Kajenna"
    "Europe/Gibraltar" inherited
    "America/Nuuk" inherited
    "America/Danmarkshavn" inherited
    "America/Scoresbysund" inherited
    "America/Thule" "Qaanaaq"
    "Europe/Athens" "Ateny"
    "Atlantic/South_Georgia" "Georgia Południowa"
    "America/Guatemala" "Gwatemala"
    "Pacific/Guam" inherited
    "Africa/Bissau" inherited
    "America/Guyana" "Gujana"
    "Asia/Hong_Kong" "Hongkong"
    "America/Tegucigalpa" inherited
    "America/Port-au-Prince" inherited
    "Europe/Budapest" "Budapeszt"
    "Asia/Jakarta" "Dżakarta"
    "Asia/Pontianak" inherited
    "Asia/Makassar" inherited
    "Asia/Jayapura" inherited
    "Europe/Dublin" inherited
    "Asia/Jerusalem" "Jerozolima"
    "Asia/Kolkata" "Kalkuta"
    "Indian/Chagos" "Czagos"
    "Asia/Baghdad" "Bagdad"
    "Asia/Tehran" "Teheran"
    "Europe/Rome" "Rzym"
    "America/Jamaica" "Jamajka"
    "Asia/Amman" inherited
    "Asia/Tokyo" "Tokio"
    "Africa/Nairobi" inherited
    "Asia/Bishkek" "Biszkek"
    "Pacific/Tarawa" inherited
    "Pacific/Kanton" "Kanton"
    "Pacific/Kiritimati" inherited
    "Asia/Pyongyang" "Pjongjang"
    "Asia/Seoul" "Seul"
    "Asia/Almaty" "Ałmaty"
    "Asia/Qyzylorda" "Kyzyłorda"
    "Asia/Qostanay" "Kustanaj"
    "Asia/Aqtobe" "Aktiubińsk"
    "Asia/Aqtau" "Aktau"
    "Asia/Atyrau" inherited
    "Asia/Oral" "Uralsk"
    "Asia/Beirut" "Bejrut"
    "Asia/Colombo" "Kolombo"
    "Africa/Monrovia" inherited
    "Europe/Vilnius" "Wilno"
    "Europe/Riga" "Ryga"
    "Africa/Tripoli" "Trypolis"
    "Africa/Casablanca" inherited
    "Europe/Chisinau" "Kiszyniów"
    "Pacific/Kwajalein" inherited
    "Asia/Yangon" "Rangun"
    "Asia/Ulaanbaatar" "Ułan Bator"
    "Asia/Hovd" "Kobdo"
    "Asia/Macau" "Makau"
    "America/Martinique" "Martynika"
    "Europe/Malta" inherited
    "Indian/Mauritius" inherited
    "Indian/Maldives" "Malediwy"
    "America/Mexico_City" "Meksyk (miasto)"
    "America/Cancun" inherited
    "America/Merida" "Merida"
    "America/Monterrey" inherited
    "America/Matamoros" inherited
    "America/Chihuahua" inherited
    "America/Ciudad_Juarez" inherited
    "America/Ojinaga" inherited
    "America/Mazatlan" inherited
    "America/Bahia_Banderas" "Bahia Banderas"
    "America/Hermosillo" inherited
    "America/Tijuana" inherited
    "Asia/Kuching" inherited
    "Africa/Maputo" inherited
    "Africa/Windhoek" "Windhuk"
    "Pacific/Noumea" "Numea"
    "Pacific/Norfolk" inherited
    "Africa/Lagos" inherited
    "America/Managua" inherited
    "Asia/Kathmandu" "Katmandu"
    "Pacific/Nauru" inherited
    "Pacific/Niue" inherited
    "Pacific/Auckland" inherited
    "Pacific/Chatham" inherited
    "America/Panama" inherited
    "America/Lima" inherited
    "Pacific/Tahiti" inherited
    "Pacific/Marquesas" "Markizy"
    "Pacific/Gambier" "Wyspy Gambiera"
    "Pacific/Port_Moresby" inherited
    "Pacific/Bougainville" "Wyspa Bougainville’a"
    "Asia/Manila" inherited
    "Asia/Karachi" "Karaczi"
    "Europe/Warsaw" "Warszawa"
    "America/Miquelon" inherited
    "Pacific/Pitcairn" inherited
    "America/Puerto_Rico" "Portoryko"
    "Asia/Gaza" inherited
    "Asia/Hebron" inherited
    "Europe/Lisbon" "Lizbona"
    "Atlantic/Madeira" "Madera"
    "Atlantic/Azores" "Azory"
    "Pacific/Palau" inherited
    "America/Asuncion" inherited
    "Asia/Qatar" "Katar"
    "Europe/Bucharest" "Bukareszt"
    "Europe/Belgrade" "Belgrad"
    "Europe/Kaliningrad" inherited
    "Europe/Moscow" "Moskwa"
    "Europe/Simferopol" "Symferopol"
    "Europe/Kirov" "Kirow"
    "Europe/Volgograd" "Wołgograd"
    "Europe/Astrakhan" "Astrachań"
    "Europe/Saratov" "Saratów"
    "Europe/Ulyanovsk" "Uljanowsk"
    "Europe/Samara" inherited
    "Asia/Yekaterinburg" "Jekaterynburg"
    "Asia/Omsk" inherited
    "Asia/Novosibirsk" "Nowosybirsk"
    "Asia/Barnaul" "Barnauł"
    "Asia/Tomsk" inherited
    "Asia/Novokuznetsk" "Nowokuźnieck"
    "Asia/Krasnoyarsk" "Krasnojarsk"
    "Asia/Irkutsk" "Irkuck"
    "Asia/Chita" "Czyta"
    "Asia/Yakutsk" "Jakuck"
    "Asia/Khandyga" "Chandyga"
    "Asia/Vladivostok" "Władywostok"
    "Asia/Ust-Nera" "Ust-Niera"
    "Asia/Magadan" inherited
    "Asia/Sakhalin" "Sachalin"
    "Asia/Srednekolymsk" "Sriedniekołymsk"
    "Asia/Kamchatka" "Kamczatka"
    "Asia/Anadyr" inherited
    "Asia/Riyadh" "Rijad"
    "Pacific/Guadalcanal" inherited
    "Africa/Khartoum" "Chartum"
    "Asia/Singapore" "Singapur"
    "America/Paramaribo" inherited
    "Africa/Juba" "Dżuba"
    "Africa/Sao_Tome" inherited
    "America/El_Salvador" "Salwador"
    "Asia/Damascus" "Damaszek"
    "America/Grand_Turk" inherited
    "Africa/Ndjamena" "Ndżamena"
    "Asia/Bangkok" inherited
    "Asia/Dushanbe" "Duszanbe"
    "Pacific/Fakaofo" inherited
    "Asia/Dili" inherited
    "Asia/Ashgabat" "Aszchabad"
    "Africa/Tunis" inherited
    "Pacific/Tongatapu" inherited
    "Europe/Istanbul" "Stambuł"
    "Asia/Taipei" "Tajpej"
    "Europe/Kyiv" "Kijów"
    "America/New_York" "Nowy Jork"
    "America/Detroit" inherited
    "America/Kentucky/Louisville" inherited
    "America/Kentucky/Monticello" inherited
    "America/Indiana/Indianapolis" inherited
    "America/Indiana/Vincennes" inherited
    "America/Indiana/Winamac" inherited
    "America/Indiana/Marengo" inherited
    "America/Indiana/Petersburg" inherited
    "America/Indiana/Vevay" inherited
    "America/Chicago" inherited
    "America/Indiana/Tell_City" inherited
    "America/Indiana/Knox" inherited
    "America/Menominee" inherited
    "America/North_Dakota/Center" "Center, Dakota Północna"
    "America/North_Dakota/New_Salem" "New Salem, Dakota Północna"
    "America/North_Dakota/Beulah" "Beulah, Dakota Północna"
    "America/Denver" inherited
    "America/Boise" inherited
    "America/Phoenix" inherited
    "America/Los_Angeles" inherited
    "America/Anchorage" inherited
    "America/Juneau" inherited
    "America/Sitka" inherited
    "America/Metlakatla" inherited
    "America/Yakutat" inherited
    "America/Nome" inherited
    "America/Adak" inherited
    "Pacific/Honolulu" "Honolulu"
    "America/Montevideo" inherited
    "Asia/Samarkand" "Samarkanda"
    "Asia/Tashkent" "Taszkient"
    "America/Caracas" inherited
    "Asia/Ho_Chi_Minh" inherited
    "Pacific/Efate" inherited
    "Pacific/Apia" inherited
    "Africa/Johannesburg" inherited
    "America/Antigua" inherited
    "America/Anguilla" inherited
    "Africa/Luanda" inherited
    "Antarctica/McMurdo" inherited
    "Antarctica/DumontDUrville" inherited
    "Antarctica/Syowa" inherited
    "America/Aruba" inherited
    "Europe/Mariehamn" "Maarianhamina"
    "Europe/Sarajevo" "Sarajewo"
    "Africa/Ouagadougou" "Wagadugu"
    "Asia/Bahrain" "Bahrajn"
    "Africa/Bujumbura" "Bużumbura"
    "Africa/Porto-Novo" "Porto Novo"
    "America/St_Barthelemy" "Saint-Barthélemy"
    "Asia/Brunei" inherited
    "America/Kralendijk" inherited
    "America/Nassau" inherited
    "Africa/Gaborone" inherited
    "America/Blanc-Sablon" inherited
    "America/Atikokan" inherited
    "America/Creston" inherited
    "Indian/Cocos" "Wyspy Kokosowe"
    "Africa/Kinshasa" "Kinszasa"
    "Africa/Lubumbashi" inherited
    "Africa/Bangui" "Bangi"
    "Africa/Brazzaville" inherited
    "Africa/Douala" "Duala"
    "America/Curacao" inherited
    "Indian/Christmas" "Wyspa Bożego Narodzenia"
    "Europe/Busingen" "Büsingen am Hochrhein"
    "Africa/Djibouti" "Dżibuti"
    "Europe/Copenhagen" "Kopenhaga"
    "America/Dominica" "Dominika"
    "Africa/Asmara" inherited
    "Africa/Addis_Ababa" "Addis Abeba"
    "Pacific/Chuuk" inherited
    "Pacific/Pohnpei" inherited
    "Africa/Libreville" inherited
    "America/Grenada" inherited
    "Europe/Guernsey" inherited
    "Africa/Accra" "Akra"
    "Africa/Banjul" "Bandżul"
    "Africa/Conakry" "Konakry"
    "America/Guadeloupe" "Gwadelupa"
    "Africa/Malabo" inherited
    "Europe/Zagreb" "Zagrzeb"
    "Europe/Isle_of_Man" "Wyspa Man"
    "Atlantic/Reykjavik" inherited
    "Europe/Jersey" inherited
    "Asia/Phnom_Penh" inherited
    "Indian/Comoro" "Komory"
    "America/St_Kitts" "Saint Kitts"
    "Asia/Kuwait" "Kuwejt"
    "America/Cayman" "Kajmany"
    "Asia/Vientiane" "Wientian"
    "America/St_Lucia" "Saint Lucia"
    "Europe/Vaduz" inherited
    "Africa/Maseru" inherited
    "Europe/Luxembourg" "Luksemburg"
    "Europe/Monaco" "Monako"
    "Europe/Podgorica" inherited
    "America/Marigot" inherited
    "Indian/Antananarivo" "Antananarywa"
    "Pacific/Majuro" inherited
    "Europe/Skopje" inherited
    "Africa/Bamako" inherited
    "Pacific/Saipan" inherited
    "Africa/Nouakchott" "Nawakszut"
    "America/Montserrat" inherited
    "Africa/Blantyre" inherited
    "Asia/Kuala_Lumpur" inherited
    "Africa/Niamey" inherited
    "Europe/Amsterdam" inherited
    "Europe/Oslo" inherited
    "Asia/Muscat" "Maskat"
    "Indian/Reunion" inherited
    "Africa/Kigali" inherited
    "Indian/Mahe" inherited
    "Europe/Stockholm" "Sztokholm"
    "Atlantic/St_Helena" "Święta Helena"
    "Europe/Ljubljana" "Lublana"
    "Arctic/Longyearbyen" inherited
    "Europe/Bratislava" "Bratysława"
    "Africa/Freetown" inherited
    "Europe/San_Marino" inherited
    "Africa/Dakar" inherited
    "Africa/Mogadishu" "Mogadiszu"
    "America/Lower_Princes" inherited
    "Africa/Mbabane" inherited
    "Indian/Kerguelen" "Wyspy Kerguelena"
    "Africa/Lome" inherited
    "America/Port_of_Spain" "Port-of-Spain"
    "Pacific/Funafuti" inherited
    "Africa/Dar_es_Salaam" inherited
    "Africa/Kampala" inherited
    "Pacific/Midway" inherited
    "Pacific/Wake" inherited
    "Europe/Vatican" "Watykan"
    "America/St_Vincent" "Saint Vincent"
    "America/Tortola" inherited
    "America/St_Thomas" "Saint Thomas"
    "Pacific/Wallis" inherited
    "Asia/Aden" inherited
    "Indian/Mayotte" "Majotta"
    "Africa/Lusaka" inherited
    "Africa/Harare" inherited
};

// `common/main/ps.xml`: 418 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const PS: &str = exemplar_cities! {
    "Europe/Andorra" "اندورا"
    "Asia/Dubai" "دوبی"
    "Asia/Kabul" "کابل"
    "Europe/Tirane" "تيران"
    "Asia/Yerevan" "يريوان"
    "Antarctica/Casey" "کیسي"
    "Antarctica/Davis" "ډيوس"
    "Antarctica/Mawson" "ماوسن"
    "Antarctica/Palmer" "پالمر"
    "Antarctica/Rothera" "رودرا"
    "Antarctica/Troll" "ټرول"
    "Antarctica/Vostok" "واستوک"
    "America/Argentina/Buenos_Aires" "بينوس اييرز"
    "America/Argentina/Cordoba" "کورډوبا"
    "America/Argentina/Salta" "سالټا"
    "America/Argentina/Jujuy" "جوجوي"
    "America/Argentina/Tucuman" "ټيکووم"
    "America/Argentina/Catamarca" "کټامارکا"
    "America/Argentina/La_Rioja" "لاريوجا"
    "America/Argentina/San_Juan" "سان جوان"
    "America/Argentina/Mendoza" "مینډوزا"
    "America/Argentina/San_Luis" "سان لویس"
    "America/Argentina/Rio_Gallegos" "ريو ګيليګوس"
    "America/Argentina/Ushuaia" "اوشوایا"
    "Pacific/Pago_Pago" "پيګو پيګو"
    "Europe/Vienna" "ویانا"
    "Australia/Lord_Howe" "لارډ هوي"
    "Antarctica/Macquarie" "مکواري"
    "Australia/Hobart" "هوبارټ"
    "Australia/Melbourne" "میلبورن"
    "Australia/Sydney" "سډني"
    "Australia/Broken_Hill" "بروکن هل"
    "Australia/Brisbane" "بریسبن"
    "Australia/Lindeman" "لینډامین"
    "Australia/Adelaide" "اډیلایډ"
    "Australia/Darwin" "ډارون"
    "Australia/Perth" "پرت"
    "Australia/Eucla" "ايوکلا"
    "Asia/Baku" "باکو"
    "America/Barbados" "باربادوس"
    "Asia/Dhaka" "ډهاکه"
    "Europe/Brussels" "بروسلز"
    "Europe/Sofia" "صوفیا"
    "Atlantic/Bermuda" "برمودا"
    "America/La_Paz" "لا پاز"
    "America/Noronha" "نورونها"
    "America/Belem" "بلم"
    "America/Fortaleza" "فورتیلزا"
    "America/Recife" "ریسیفي"
    "America/Araguaina" "ارګینیا"
    "America/Maceio" "ماسيو"
    "America/Bahia" "بهیا"
    "America/Sao_Paulo" "ساو پاولو"
    "America/Campo_Grande" "کمپو ګرډی"
    "America/Cuiaba" "کویابا"
    "America/Santarem" "سناترم"
    "America/Porto_Velho" "پورټو ویلهو"
    "America/Boa_Vista" "بوا ویسټا"
    "America/Manaus" "مناوس"
    "America/Eirunepe" "اییرونپ"
    "America/Rio_Branco" "ریو برانکو"
    "Asia/Thimphu" "تهيمفو"
    "Europe/Minsk" "منسک"
    "America/Belize" "بلیز"
    "America/St_Johns" "سینټ جانز"
    "America/Halifax" "هیلفکس"
    "America/Glace_Bay" "ګیسس بيی"
    "America/Moncton" "مونکټون"
    "America/Goose_Bay" "گوز بي"
    "America/Toronto" "ټورنټو"
    "America/Iqaluit" "اقلیټ"
    "America/Winnipeg" "وینپیګ"
    "America/Resolute" "ريسالوټ"
    "America/Rankin_Inlet" "رينکن انلټ"
    "America/Regina" "ریګینا"
    "America/Swift_Current" "سويفټ کرنټ"
    "America/Edmonton" "ایډمونټن"
    "America/Cambridge_Bay" "کیمبرج بي"
    "America/Inuvik" "انوک"
    "America/Vancouver" "وینکوور"
    "America/Dawson_Creek" "داسن کریک"
    "America/Fort_Nelson" "فورټ نیلسن"
    "America/Whitehorse" "وايټ هارس"
    "America/Dawson" "داوسن"
    "Europe/Zurich" "زریچ"
    "Africa/Abidjan" "ابيجان"
    "Pacific/Rarotonga" "راروټونګا"
    "America/Santiago" "سنتياګو"
    "America/Coyhaique" "کوهيک"
    "America/Punta_Arenas" "پنټا آریناس"
    "Pacific/Easter" "ایسټر"
    "Asia/Shanghai" "شنگھائی"
    "Asia/Urumqi" "اورومقي"
    "America/Bogota" "بوګټا"
    "America/Costa_Rica" "کوستاریکا"
    "America/Havana" "هوانا"
    "Atlantic/Cape_Verde" "کيپ ورډ"
    "Asia/Nicosia" "نیکوسیا"
    "Asia/Famagusta" "فاماګستا"
    "Europe/Prague" "پراګ"
    "Europe/Berlin" "برلن"
    "America/Santo_Domingo" "سنتو ډومینګو"
    "Africa/Algiers" "الجييرز"
    "America/Guayaquil" "ګوياکل"
    "Pacific/Galapagos" "ګالپګوس"
    "Europe/Tallinn" "تالين"
    "Africa/Cairo" "قاهره"
    "Africa/El_Aaiun" "الیون"
    "Europe/Madrid" "میډریډ"
    "Africa/Ceuta" "سيوټا"
    "Atlantic/Canary" "کناري"
    "Europe/Helsinki" "هیلسنکی"
    "Pacific/Fiji" "فجي"
    "Atlantic/Stanley" "سټنلي"
    "Pacific/Kosrae" "کوسراي"
    "Atlantic/Faroe" "فارو"
    "Europe/Paris" "پاریس"
    "Europe/London" "لندن"
    "Asia/Tbilisi" "تبلیسي"
    "America/Cayenne" "کیین"
    "Europe/Gibraltar" "جبل الطارق"
    "America/Nuuk" "نووک"
    "America/Danmarkshavn" "ډنمارکشان"
    "America/Scoresbysund" "اټوکوټورمیټ"
    "America/Thule" "تول"
    "Europe/Athens" "ايتنز"
    "Atlantic/South_Georgia" "سويلي جورجيا"
    "America/Guatemala" "ګواتمالا"
    "Pacific/Guam" "ګوام"
    "Africa/Bissau" "بساؤ"
    "America/Guyana" "ګیانا"
    "Asia/Hong_Kong" "هانګ کانګ"
    "America/Tegucigalpa" "ټګسیګالپا"
    "America/Port-au-Prince" "پورټ ایو - پرنس"
    "Europe/Budapest" "بداپسټ"
    "Asia/Jakarta" "جکارتا"
    "Asia/Pontianak" "پونټینیک"
    "Asia/Makassar" "مکاسار"
    "Asia/Jayapura" "جاياپورا"
    "Europe/Dublin" "ډبلن"
    "Asia/Jerusalem" "يروشلم"
    "Asia/Kolkata" "کولکته"
    "Indian/Chagos" "چاګوس"
    "Asia/Baghdad" "بغداد"
    "Asia/Tehran" "تهران"
    "Europe/Rome" "روم"
    "America/Jamaica" "جمایکه"
    "Asia/Amman" "اممان"
    "Asia/Tokyo" "ټوکیو"
    "Africa/Nairobi" "نايروبي"
    "Asia/Bishkek" "بشکیک"
    "Pacific/Tarawa" "تاراوا"
    "Pacific/Kanton" "کانټون"
    "Pacific/Kiritimati" "کيريټماټي"
    "Asia/Pyongyang" "پيانګ يانګ"
    "Asia/Seoul" "سیول"
    "Asia/Almaty" "الماتی"
    "Asia/Qyzylorda" "قيزي لورډا"
    "Asia/Qostanay" "کوستانې"
    "Asia/Aqtobe" "اکتوب"
    "Asia/Aqtau" "اکټاو"
    "Asia/Atyrau" "اېټراو"
    "Asia/Oral" "اورل"
    "Asia/Beirut" "بیروت"
    "Asia/Colombo" "کولمبو"
    "Africa/Monrovia" "مونروفیا"
    "Europe/Vilnius" "ويلنيوس"
    "Europe/Riga" "ريګا"
    "Africa/Tripoli" "تريپولي"
    "Africa/Casablanca" "کاسابلانکا"
    "Europe/Chisinau" "چیسینو"
    "Pacific/Kwajalein" "کواجلين"
    "Asia/Yangon" "یانګون"
    "Asia/Ulaanbaatar" "اولان باټر"
    "Asia/Hovd" "هاوډ"
    "Asia/Macau" "مکاو"
    "America/Martinique" "مارټینیک"
    "Europe/Malta" "مالټا"
    "Indian/Mauritius" "ماريشيس"
    "Indian/Maldives" "مالديپ"
    "America/Mexico_City" "مکسيکو ښار"
    "America/Cancun" "کينکن"
    "America/Merida" "ميريډا"
    "America/Monterrey" "منټرري"
    "America/Matamoros" "ميټاموروس"
    "America/Chihuahua" "چھواھوا"
    "America/Ciudad_Juarez" "سیوداد جیوریز"
    "America/Ojinaga" "اوجنګا"
    "America/Mazatlan" "مزاتلان"
    "America/Bahia_Banderas" "بهیا بینډراس"
    "America/Hermosillo" "هرموسیلو"
    "America/Tijuana" "تجوانا"
    "Asia/Kuching" "کوچنګ"
    "Africa/Maputo" "ماپوټو"
    "Africa/Windhoek" "وینهوک"
    "Pacific/Noumea" "نوميا"
    "Pacific/Norfolk" "نورفک"
    "Africa/Lagos" "لاگوس"
    "America/Managua" "منګوا"
    "Asia/Kathmandu" "کټمنډو"
    "Pacific/Nauru" "نایرو"
    "Pacific/Niue" "نیوو"
    "Pacific/Auckland" "اکلند"
    "Pacific/Chatham" "چاتام"
    "America/Panama" "پاناما"
    "America/Lima" "لیما"
    "Pacific/Tahiti" "ټهيټي"
    "Pacific/Marquesas" "مارکيساس"
    "Pacific/Gambier" "ګيمبير"
    "Pacific/Port_Moresby" "پورټ مورسبی"
    "Pacific/Bougainville" "بوګن ویل"
    "Asia/Manila" "منیلا"
    "Asia/Karachi" "کراچي"
    "Europe/Warsaw" "وارسا"
    "America/Miquelon" "ميکويلان"
    "Pacific/Pitcairn" "پيټيکيرن"
    "America/Puerto_Rico" "پورتو ریکو"
    "Asia/Gaza" "غزه"
    "Asia/Hebron" "هبرون"
    "Europe/Lisbon" "لیسبون"
    "Atlantic/Madeira" "مديرا"
    "Atlantic/Azores" "ايزورس"
    "Pacific/Palau" "پلاو"
    "America/Asuncion" "اسونسيون"
    "Asia/Qatar" "قطر"
    "Europe/Bucharest" "بخارست"
    "Europe/Belgrade" "بلغاد"
    "Europe/Kaliningrad" "کيلنينګراډ"
    "Europe/Moscow" "ماسکو"
    "Europe/Simferopol" "سیمفروپول"
    "Europe/Kirov" "کیروف"
    "Europe/Volgograd" "والګوګراډ"
    "Europe/Astrakhan" "استرا خان"
    "Europe/Saratov" "سراتف"
    "Europe/Ulyanovsk" "اليانوسک"
    "Europe/Samara" "سمارا"
    "Asia/Yekaterinburg" "يکاټيرنبرګ"
    "Asia/Omsk" "اومسک"
    "Asia/Novosibirsk" "نووسيبرسک"
    "Asia/Barnaul" "برنول"
    "Asia/Tomsk" "توماس"
    "Asia/Novokuznetsk" "نووکوزنیټک"
    "Asia/Krasnoyarsk" "کريسنويارسک"
    "Asia/Irkutsk" "ارکوټسک"
    "Asia/Chita" "چيتا"
    "Asia/Yakutsk" "ياکوټسک"
    "Asia/Khandyga" "خنديګا"
    "Asia/Vladivostok" "ولادیوستاک"
    "Asia/Ust-Nera" "اوستنيرا"
    "Asia/Magadan" "مګدان"
    "Asia/Sakhalin" "سخالين"
    "Asia/Srednekolymsk" "سريډنيکوليمسک"
    "Asia/Kamchatka" "کامچاتکا"
    "Asia/Anadyr" "اناډير"
    "Asia/Riyadh" "رياض"
    "Pacific/Guadalcanal" "ګواډلکينال"
    "Africa/Khartoum" "خرتوم"
    "Asia/Singapore" "سینګاپور"
    "America/Paramaribo" "پاراماربو"
    "Africa/Juba" "جوبا"
    "Africa/Sao_Tome" "ساو ټوم"
    "America/El_Salvador" "ايل سلوادور"
    "Asia/Damascus" "دمشق"
    "America/Grand_Turk" "لوی ترک"
    "Africa/Ndjamena" "نجامینا"
    "Asia/Bangkok" "بنکاک"
    "Asia/Dushanbe" "دوشنبي"
    "Pacific/Fakaofo" "فوکافو"
    "Asia/Dili" "دلي"
    "Asia/Ashgabat" "اشغ آباد"
    "Africa/Tunis" "تونس"
    "Pacific/Tongatapu" "ټونګاتاپو"
    "Europe/Istanbul" "استنبول"
    "Asia/Taipei" "تايپي"
    "Europe/Kyiv" "کیف"
    "America/New_York" "نیویارک"
    "America/Detroit" "ډایټروټ"
    "America/Kentucky/Louisville" "لوئس ویل"
    "America/Kentucky/Monticello" "مونټيسيلو، کونټکی"
    "America/Indiana/Indianapolis" "انډيانا پوليس"
    "America/Indiana/Vincennes" "وينسينس، انډيانا"
    "America/Indiana/Winamac" "وينامک انډيانا"
    "America/Indiana/Marengo" "مورينګو انډيانا"
    "America/Indiana/Petersburg" "پيټسبرګ، انډيانا"
    "America/Indiana/Vevay" "ویوی، انډيانا"
    "America/Chicago" "شیکاګو"
    "America/Indiana/Tell_City" "ټل سټي، انډيانا"
    "America/Indiana/Knox" "نوکس انډيانا"
    "America/Menominee" "مینومین"
    "America/North_Dakota/Center" "مرکز، شمالي ډاکوټا"
    "America/North_Dakota/New_Salem" "نوی سلیم، شمالي داکوتا"
    "America/North_Dakota/Beulah" "بيولا، شمالي ډاکوټا"
    "America/Denver" "ډنور"
    "America/Boise" "بوز"
    "America/Phoenix" "فینکس"
    "America/Los_Angeles" "لاس اینجلس"
    "America/Anchorage" "اینکریج"
    "America/Juneau" "جونو"
    "America/Sitka" "سیټکا"
    "America/Metlakatla" "میتلاکاټلا"
    "America/Yakutat" "ياکوټټ"
    "America/Nome" "نوم"
    "America/Adak" "اداک"
    "Pacific/Honolulu" "هینولولو"
    "America/Montevideo" "مونټ وډیو"
    "Asia/Samarkand" "سمرقند"
    "Asia/Tashkent" "تاشقند"
    "America/Caracas" "کاراکاس"
    "Asia/Ho_Chi_Minh" "هو چي من ښار"
    "Pacific/Efate" "عفات"
    "Pacific/Apia" "اپیا"
    "Africa/Johannesburg" "جوهانسبرګ"
    "America/Antigua" "انټيګ"
    "America/Anguilla" "انګیلا"
    "Africa/Luanda" "لونده"
    "Antarctica/McMurdo" "مکمرډو"
    "Antarctica/DumontDUrville" "ډومونټ ډي ارول"
    "Antarctica/Syowa" "سیوا"
    "America/Aruba" "آروبا"
    "Europe/Mariehamn" "ميريهام"
    "Europe/Sarajevo" "سيراجيوا"
    "Africa/Ouagadougou" "اوګوډوګو"
    "Asia/Bahrain" "بحرین"
    "Africa/Bujumbura" "بجوګورا"
    "Africa/Porto-Novo" "پورټو - نوو"
    "America/St_Barthelemy" "سینټ بارټیلیم"
    "Asia/Brunei" "برویني"
    "America/Kralendijk" "کلینډیزج"
    "America/Nassau" "نیساو"
    "Africa/Gaborone" "ګابرون"
    "America/Blanc-Sablon" "بلانک-سابلون"
    "America/Atikokan" "اتیکوکن"
    "America/Creston" "کرسټون"
    "Indian/Cocos" "کوکوز"
    "Africa/Kinshasa" "کينشاسا"
    "Africa/Lubumbashi" "لبوباشي"
    "Africa/Bangui" "بانګوي"
    "Africa/Brazzaville" "برازاويل"
    "Africa/Douala" "دوالا"
    "America/Curacao" "کوراکاؤ"
    "Indian/Christmas" "کريسمس"
    "Europe/Busingen" "بوسينجن"
    "Africa/Djibouti" "جبوتي"
    "Europe/Copenhagen" "کوپن هيګن"
    "America/Dominica" "دومینیکا"
    "Africa/Asmara" "اسماره"
    "Africa/Addis_Ababa" "اديس ابابا"
    "Pacific/Chuuk" "چوک"
    "Pacific/Pohnpei" "پونپي"
    "Africa/Libreville" "لیبریل"
    "America/Grenada" "ګرنادا"
    "Europe/Guernsey" "ګرنسي"
    "Africa/Accra" "اکرا"
    "Africa/Banjul" "بانجول"
    "Africa/Conakry" "کونکري"
    "America/Guadeloupe" "ګالډیپ"
    "Africa/Malabo" "مالابو"
    "Europe/Zagreb" "زګرب"
    "Europe/Isle_of_Man" "د آئل آف مین"
    "Atlantic/Reykjavik" "ريکجاويک"
    "Europe/Jersey" "جرسی"
    "Asia/Phnom_Penh" "پنوم پن"
    "Indian/Comoro" "کومورو"
    "America/St_Kitts" "سینټ کټس"
    "Asia/Kuwait" "کوېت"
    "America/Cayman" "کیمن"
    "Asia/Vientiane" "وينټين"
    "America/St_Lucia" "سینټ لوسیا"
    "Europe/Vaduz" "واډوز"
    "Africa/Maseru" "مسيرو"
    "Europe/Luxembourg" "لوګزامبورګ"
    "Europe/Monaco" "موناکو"
    "Europe/Podgorica" "پوډګوريکا"
    "America/Marigot" "ميريګاټ"
    "Indian/Antananarivo" "انتانناريوو"
    "Pacific/Majuro" "مجورو"
    "Europe/Skopje" "سکپوګ"
    "Africa/Bamako" "بامیکو"
    "Pacific/Saipan" "سيپان"
    "Africa/Nouakchott" "نوکوچټ"
    "America/Montserrat" "مانټیسیرت"
    "Africa/Blantyre" "بلنټاير"
    "Asia/Kuala_Lumpur" "کولالمپور"
    "Africa/Niamey" "نیمي"
    "Europe/Amsterdam" "امستردام"
    "Europe/Oslo" "اوسلو"
    "Asia/Muscat" "مسقط"
    "Indian/Reunion" "ري يونين"
    "Africa/Kigali" "کيگالي"
    "Indian/Mahe" "ماهي"
    "Europe/Stockholm" "استولوم"
    "Atlantic/St_Helena" "سینټ هیلینا"
    "Europe/Ljubljana" "لوبجانا"
    "Arctic/Longyearbyen" "لانګيربين"
    "Europe/Bratislava" "براټسلاوا"
    "Africa/Freetown" "فریټون"
    "Europe/San_Marino" "سان مارینو"
    "Africa/Dakar" "ډاکار"
    "Africa/Mogadishu" "موگديشو"
    "America/Lower_Princes" "لوور پرنس کوارټر"
    "Africa/Mbabane" "مبابانې"
    "Indian/Kerguelen" "کرګولين"
    "Africa/Lome" "لووم"
    "America/Port_of_Spain" "د اسپانیا بندر"
    "Pacific/Funafuti" "فونافوتي"
    "Africa/Dar_es_Salaam" "دار السلام"
    "Africa/Kampala" "کمپاله"
    "Pacific/Midway" "ميډوی"
    "Pacific/Wake" "ویک"
    "Europe/Vatican" "ویټیکان"
    "America/St_Vincent" "سېنټ ویسنټ"
    "America/Tortola" "ټورتولا"
    "America/St_Thomas" "سينټ تهامس"
    "Pacific/Wallis" "والس"
    "Asia/Aden" "اډن"
    "Indian/Mayotte" "میټوت"
    "Africa/Lusaka" "لوساکا"
    "Africa/Harare" "هرارې"
};

// `common/main/pt.xml`: 123 of the 418 zones named, 295 inherited.
#[cfg(feature = "localized-exemplar-cities")]
const PT: &str = exemplar_cities! {
    "Europe/Andorra" inherited
    "Asia/Dubai" inherited
    "Asia/Kabul" "Cabul"
    "Europe/Tirane" inherited
    "Asia/Yerevan" inherited
    "Antarctica/Casey" inherited
    "Antarctica/Davis" inherited
    "Antarctica/Mawson" inherited
    "Antarctica/Palmer" inherited
    "Antarctica/Rothera" inherited
    "Antarctica/Troll" inherited
    "Antarctica/Vostok" inherited
    "America/Argentina/Buenos_Aires" inherited
    "America/Argentina/Cordoba" inherited
    "America/Argentina/Salta" inherited
    "America/Argentina/Jujuy" inherited
    "America/Argentina/Tucuman" "Tucumã"
    "America/Argentina/Catamarca" inherited
    "America/Argentina/La_Rioja" inherited
    "America/Argentina/San_Juan" inherited
    "America/Argentina/Mendoza" inherited
    "America/Argentina/San_Luis" inherited
    "America/Argentina/Rio_Gallegos" inherited
    "America/Argentina/Ushuaia" inherited
    "Pacific/Pago_Pago" inherited
    "Europe/Vienna" "Viena"
    "Australia/Lord_Howe" inherited
    "Antarctica/Macquarie" inherited
    "Australia/Hobart" inherited
    "Australia/Melbourne" inherited
    "Australia/Sydney" inherited
    "Australia/Broken_Hill" inherited
    "Australia/Brisbane" inherited
    "Australia/Lindeman" inherited
    "Australia/Adelaide" inherited
    "Australia/Darwin" inherited
    "Australia/Perth" inherited
    "Australia/Eucla" inherited
    "Asia/Baku" inherited
    "America/Barbados" inherited
    "Asia/Dhaka" "Dacca"
    "Europe/Brussels" "Bruxelas"
    "Europe/Sofia" "Sófia"
    "Atlantic/Bermuda" "Bermudas"
    "America/La_Paz" inherited
    "America/Noronha" inherited
    "America/Belem" inherited
    "America/Fortaleza" inherited
    "America/Recife" inherited
    "America/Araguaina" inherited
    "America/Maceio" inherited
    "America/Bahia" inherited
    "America/Sao_Paulo" inherited
    "America/Campo_Grande" inherited
    "America/Cuiaba" inherited
    "America/Santarem" inherited
    "America/Porto_Velho" inherited
    "America/Boa_Vista" inherited
    "America/Manaus" inherited
    "America/Eirunepe" inherited
    "America/Rio_Branco" inherited
    "Asia/Thimphu" inherited
    "Europe/Minsk" inherited
    "America/Belize" inherited
    "America/St_Johns" "Saint John’s"
    "America/Halifax" inherited
    "America/Glace_Bay" inherited
    "America/Moncton" inherited
    "America/Goose_Bay" inherited
    "America/Toronto" inherited
    "America/Iqaluit" inherited
    "America/Winnipeg" inherited
    "America/Resolute" inherited
    "America/Rankin_Inlet" inherited
    "America/Regina" inherited
    "America/Swift_Current" inherited
    "America/Edmonton" inherited
    "America/Cambridge_Bay" inherited
    "America/Inuvik" inherited
    "America/Vancouver" inherited
    "America/Dawson_Creek" inherited
    "America/Fort_Nelson" inherited
    "America/Whitehorse" inherited
    "America/Dawson" inherited
    "Europe/Zurich" "Zurique"
    "Africa/Abidjan" inherited
    "Pacific/Rarotonga" inherited
    "America/Santiago" inherited
    "America/Coyhaique" "Coihaique"
    "America/Punta_Arenas" inherited
    "Pacific/Easter" "Ilha de Páscoa"
    "Asia/Shanghai" "Xangai"
    "Asia/Urumqi" inherited
    "America/Bogota" inherited
    "America/Costa_Rica" inherited
    "America/Havana" inherited
    "Atlantic/Cape_Verde" "Cabo Verde"
    "Asia/Nicosia" "Nicósia"
    "Asia/Famagusta" inherited
    "Europe/Prague" "Praga"
    "Europe/Berlin" "Berlim"
    "America/Santo_Domingo" inherited
    "Africa/Algiers" "Argel"
    "America/Guayaquil" "Guaiaquil"
    "Pacific/Galapagos" inherited
    "Europe/Tallinn" inherited
    "Africa/Cairo" inherited
    "Africa/El_Aaiun" inherited
    "Europe/Madrid" "Madri"
    "Africa/Ceuta" inherited
    "Atlantic/Canary" "Canárias"
    "Europe/Helsinki" "Helsinque"
    "Pacific/Fiji" inherited
    "Atlantic/Stanley" inherited
    "Pacific/Kosrae" inherited
    "Atlantic/Faroe" "Ilhas Faroé"
    "Europe/Paris" inherited
    "Europe/London" "Londres"
    "Asia/Tbilisi" inherited
    "America/Cayenne" "Caiena"
    "Europe/Gibraltar" inherited
    "America/Nuuk" inherited
    "America/Danmarkshavn" inherited
    "America/Scoresbysund" inherited
    "America/Thule" inherited
    "Europe/Athens" "Atenas"
    "Atlantic/South_Georgia" "Geórgia do Sul"
    "America/Guatemala" inherited
    "Pacific/Guam" inherited
    "Africa/Bissau" inherited
    "America/Guyana" "Guiana"
    "Asia/Hong_Kong" inherited
    "America/Tegucigalpa" inherited
    "America/Port-au-Prince" "Porto Príncipe"
    "Europe/Budapest" "Budapeste"
    "Asia/Jakarta" "Jacarta"
    "Asia/Pontianak" inherited
    "Asia/Makassar" inherited
    "Asia/Jayapura" inherited
    "Europe/Dublin" inherited
    "Asia/Jerusalem" "Jerusalém"
    "Asia/Kolkata" "Calcutá"
    "Indian/Chagos" inherited
    "Asia/Baghdad" "Bagdá"
    "Asia/Tehran" "Teerã"
    "Europe/Rome" "Roma"
    "America/Jamaica" inherited
    "Asia/Amman" "Amã"
    "Asia/Tokyo" "Tóquio"
    "Africa/Nairobi" "Nairóbi"
    "Asia/Bishkek" inherited
    "Pacific/Tarawa" "Taraua"
    "Pacific/Kanton" "Ilha de Canton"
    "Pacific/Kiritimati" inherited
    "Asia/Pyongyang" inherited
    "Asia/Seoul" "Seul"
    "Asia/Almaty" inherited
    "Asia/Qyzylorda" inherited
    "Asia/Qostanay" inherited
    "Asia/Aqtobe" "Aktobe"
    "Asia/Aqtau" "Aktau"
    "Asia/Atyrau" inherited
    "Asia/Oral" inherited
    "Asia/Beirut" "Beirute"
    "Asia/Colombo" inherited
    "Africa/Monrovia" "Monróvia"
    "Europe/Vilnius" inherited
    "Europe/Riga" inherited
    "Africa/Tripoli" "Trípoli"
    "Africa/Casablanca" inherited
    "Europe/Chisinau" inherited
    "Pacific/Kwajalein" inherited
    "Asia/Yangon" "Rangum"
    "Asia/Ulaanbaatar" "Ulan Bator"
    "Asia/Hovd" inherited
    "Asia/Macau" "Macau"
    "America/Martinique" "Martinica"
    "Europe/Malta" inherited
    "Indian/Mauritius" "Maurício"
    "Indian/Maldives" "Maldivas"
    "America/Mexico_City" "Cidade do México"
    "America/Cancun" inherited
    "America/Merida" inherited
    "America/Monterrey" inherited
    "America/Matamoros" inherited
    "America/Chihuahua" inherited
    "America/Ciudad_Juarez" inherited
    "America/Ojinaga" inherited
    "America/Mazatlan" inherited
    "America/Bahia_Banderas" "Bahia de Banderas"
    "America/Hermosillo" inherited
    "America/Tijuana" inherited
    "Asia/Kuching" inherited
    "Africa/Maputo" inherited
    "Africa/Windhoek" inherited
    "Pacific/Noumea" inherited
    "Pacific/Norfolk" inherited
    "Africa/Lagos" inherited
    "America/Managua" "Manágua"
    "Asia/Kathmandu" "Katmandu"
    "Pacific/Nauru" inherited
    "Pacific/Niue" inherited
    "Pacific/Auckland" inherited
    "Pacific/Chatham" "Chatnam"
    "America/Panama" "Panamá"
    "America/Lima" inherited
    "Pacific/Tahiti" "Taiti"
    "Pacific/Marquesas" inherited
    "Pacific/Gambier" inherited
    "Pacific/Port_Moresby" inherited
    "Pacific/Bougainville" inherited
    "Asia/Manila" inherited
    "Asia/Karachi" inherited
    "Europe/Warsaw" "Varsóvia"
    "America/Miquelon" inherited
    "Pacific/Pitcairn" inherited
    "America/Puerto_Rico" "Porto Rico"
    "Asia/Gaza" inherited
    "Asia/Hebron" inherited
    "Europe/Lisbon" "Lisboa"
    "Atlantic/Madeira" inherited
    "Atlantic/Azores" "Açores"
    "Pacific/Palau" inherited
    "America/Asuncion" "Assunção"
    "Asia/Qatar" "Catar"
    "Europe/Bucharest" "Bucareste"
    "Europe/Belgrade" "Belgrado"
    "Europe/Kaliningrad" "Kaliningrado"
    "Europe/Moscow" "Moscou"
    "Europe/Simferopol" inherited
    "Europe/Kirov" inherited
    "Europe/Volgograd" "Volgogrado"
    "Europe/Astrakhan" "Astracã"
    "Europe/Saratov" inherited
    "Europe/Ulyanovsk" "Ulianovsk"
    "Europe/Samara" inherited
    "Asia/Yekaterinburg" "Ecaterimburgo"
    "Asia/Omsk" inherited
    "Asia/Novosibirsk" inherited
    "Asia/Barnaul" inherited
    "Asia/Tomsk" inherited
    "Asia/Novokuznetsk" inherited
    "Asia/Krasnoyarsk" inherited
    "Asia/Irkutsk" inherited
    "Asia/Chita" inherited
    "Asia/Yakutsk" inherited
    "Asia/Khandyga" inherited
    "Asia/Vladivostok" inherited
    "Asia/Ust-Nera" inherited
    "Asia/Magadan" inherited
    "Asia/Sakhalin" "Sacalina"
    "Asia/Srednekolymsk" inherited
    "Asia/Kamchatka" inherited
    "Asia/Anadyr" inherited
    "Asia/Riyadh" "Riade"
    "Pacific/Guadalcanal" inherited
    "Africa/Khartoum" "Cartum"
    "Asia/Singapore" "Singapura"
    "America/Paramaribo" inherited
    "Africa/Juba" inherited
    "Africa/Sao_Tome" inherited
    "America/El_Salvador" inherited
    "Asia/Damascus" "Damasco"
    "America/Grand_Turk" inherited
    "Africa/Ndjamena" inherited
    "Asia/Bangkok" inherited
    "Asia/Dushanbe" "Duchambe"
    "Pacific/Fakaofo" inherited
    "Asia/Dili" inherited
    "Asia/Ashgabat" "Asgabate"
    "Africa/Tunis" "Túnis"
    "Pacific/Tongatapu" inherited
    "Europe/Istanbul" "Istambul"
    "Asia/Taipei" inherited
    "Europe/Kyiv" "Kiev"
    "America/New_York" "Nova York"
    "America/Detroit" inherited
    "America/Kentucky/Louisville" inherited
    "America/Kentucky/Monticello" inherited
    "America/Indiana/Indianapolis" "Indianápolis"
    "America/Indiana/Vincennes" inherited
    "America/Indiana/Winamac" inherited
    "America/Indiana/Marengo" inherited
    "America/Indiana/Petersburg" inherited
    "America/Indiana/Vevay" inherited
    "America/Chicago" inherited
    "America/Indiana/Tell_City" inherited
    "America/Indiana/Knox" inherited
    "America/Menominee" inherited
    "America/North_Dakota/Center" "Center, Dakota do Norte"
    "America/North_Dakota/New_Salem" "New Salen, Dakota do Norte"
    "America/North_Dakota/Beulah" "Beulah, Dakota do Norte"
    "America/Denver" inherited
    "America/Boise" inherited
    "America/Phoenix" inherited
    "America/Los_Angeles" inherited
    "America/Anchorage" inherited
    "America/Juneau" inherited
    "America/Sitka" inherited
    "America/Metlakatla" inherited
    "America/Yakutat" inherited
    "America/Nome" inherited
    "America/Adak" inherited
    "Pacific/Honolulu" "Honolulu"
    "America/Montevideo" "Montevidéu"
    "Asia/Samarkand" "Samarcanda"
    "Asia/Tashkent" inherited
    "America/Caracas" inherited
    "Asia/Ho_Chi_Minh" "Cidade de Ho Chi Minh"
    "Pacific/Efate" "Éfaté"
    "Pacific/Apia" inherited
    "Africa/Johannesburg" "Joanesburgo"
    "America/Antigua" "Antígua"
    "America/Anguilla" "Anguila"
    "Africa/Luanda" inherited
    "Antarctica/McMurdo" inherited
    "Antarctica/DumontDUrville" inherited
    "Antarctica/Syowa" inherited
    "America/Aruba" inherited
    "Europe/Mariehamn" inherited
    "Europe/Sarajevo" inherited
    "Africa/Ouagadougou" inherited
    "Asia/Bahrain" "Bahrein"
    "Africa/Bujumbura" inherited
    "Africa/Porto-Novo" "Porto Novo"
    "America/St_Barthelemy" "São Bartolomeu"
    "Asia/Brunei" inherited
    "America/Kralendijk" inherited
    "America/Nassau" inherited
    "Africa/Gaborone" inherited
    "America/Blanc-Sablon" inherited
    "America/Atikokan" inherited
    "America/Creston" inherited
    "Indian/Cocos" inherited
    "Africa/Kinshasa" inherited
    "Africa/Lubumbashi" inherited
    "Africa/Bangui" inherited
    "Africa/Brazzaville" inherited
    "Africa/Douala" inherited
    "America/Curacao" inherited
    "Indian/Christmas" inherited
    "Europe/Busingen" inherited
    "Africa/Djibouti" "Djibuti"
    "Europe/Copenhagen" "Copenhague"
    "America/Dominica" inherited
    "Africa/Asmara" inherited
    "Africa/Addis_Ababa" "Adis Abeba"
    "Pacific/Chuuk" inherited
    "Pacific/Pohnpei" inherited
    "Africa/Libreville" inherited
    "America/Grenada" "Granada"
    "Europe/Guernsey" inherited
    "Africa/Accra" "Acra"
    "Africa/Banjul" inherited
    "Africa/Conakry" "Conacri"
    "America/Guadeloupe" "Guadalupe"
    "Africa/Malabo" inherited
    "Europe/Zagreb" inherited
    "Europe/Isle_of_Man" "Ilha de Man"
    "Atlantic/Reykjavik" "Reykjavík"
    "Europe/Jersey" inherited
    "Asia/Phnom_Penh" inherited
    "Indian/Comoro" inherited
    "America/St_Kitts" "São Cristóvão"
    "Asia/Kuwait" inherited
    "America/Cayman" inherited
    "Asia/Vientiane" inherited
    "America/St_Lucia" "Santa Lúcia"
    "Europe/Vaduz" inherited
    "Africa/Maseru" inherited
    "Europe/Luxembourg" "Luxemburgo"
    "Europe/Monaco" "Mônaco"
    "Europe/Podgorica" inherited
    "America/Marigot" inherited
    "Indian/Antananarivo" inherited
    "Pacific/Majuro" inherited
    "Europe/Skopje" inherited
    "Africa/Bamako" inherited
    "Pacific/Saipan" inherited
    "Africa/Nouakchott" inherited
    "America/Montserrat" inherited
    "Africa/Blantyre" inherited
    "Asia/Kuala_Lumpur" inherited
    "Africa/Niamey" inherited
    "Europe/Amsterdam" "Amsterdã"
    "Europe/Oslo" inherited
    "Asia/Muscat" "Mascate"
    "Indian/Reunion" "Reunião"
    "Africa/Kigali" inherited
    "Indian/Mahe" inherited
    "Europe/Stockholm" "Estocolmo"
    "Atlantic/St_Helena" "Santa Helena"
    "Europe/Ljubljana" "Liubliana"
    "Arctic/Longyearbyen" inherited
    "Europe/Bratislava" inherited
    "Africa/Freetown" inherited
    "Europe/San_Marino" inherited
    "Africa/Dakar" inherited
    "Africa/Mogadishu" "Mogadíscio"
    "America/Lower_Princes" inherited
    "Africa/Mbabane" inherited
    "Indian/Kerguelen" inherited
    "Africa/Lome" inherited
    "America/Port_of_Spain" inherited
    "Pacific/Funafuti" inherited
    "Africa/Dar_es_Salaam" inherited
    "Africa/Kampala" inherited
    "Pacific/Midway" inherited
    "Pacific/Wake" inherited
    "Europe/Vatican" "Vaticano"
    "America/St_Vincent" "São Vicente"
    "America/Tortola" inherited
    "America/St_Thomas" "Saint Thomas"
    "Pacific/Wallis" inherited
    "Asia/Aden" "Áden"
    "Indian/Mayotte" inherited
    "Africa/Lusaka" inherited
    "Africa/Harare" inherited
};

// `common/main/ru.xml`: 418 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const RU: &str = exemplar_cities! {
    "Europe/Andorra" "Андорра"
    "Asia/Dubai" "Дубай"
    "Asia/Kabul" "Кабул"
    "Europe/Tirane" "Тирана"
    "Asia/Yerevan" "Ереван"
    "Antarctica/Casey" "Кейси"
    "Antarctica/Davis" "Дейвис"
    "Antarctica/Mawson" "Моусон"
    "Antarctica/Palmer" "Палмер"
    "Antarctica/Rothera" "Ротера"
    "Antarctica/Troll" "Тролль"
    "Antarctica/Vostok" "Восток"
    "America/Argentina/Buenos_Aires" "Буэнос-Айрес"
    "America/Argentina/Cordoba" "Кордова"
    "America/Argentina/Salta" "Сальта"
    "America/Argentina/Jujuy" "Жужуй"
    "America/Argentina/Tucuman" "Тукуман"
    "America/Argentina/Catamarca" "Катамарка"
    "America/Argentina/La_Rioja" "Ла-Риоха"
    "America/Argentina/San_Juan" "Сан-Хуан"
    "America/Argentina/Mendoza" "Мендоса"
    "America/Argentina/San_Luis" "Сан-Луис"
    "America/Argentina/Rio_Gallegos" "Рио-Гальегос"
    "America/Argentina/Ushuaia" "Ушуая"
    "Pacific/Pago_Pago" "Паго-Паго"
    "Europe/Vienna" "Вена"
    "Australia/Lord_Howe" "Лорд-Хау"
    "Antarctica/Macquarie" "Маккуори"
    "Australia/Hobart" "Хобарт"
    "Australia/Melbourne" "Мельбурн"
    "Australia/Sydney" "Сидней"
    "Australia/Broken_Hill" "Брокен-Хилл"
    "Australia/Brisbane" "Брисбен"
    "Australia/Lindeman" "Линдеман"
    "Australia/Adelaide" "Аделаида"
    "Australia/Darwin" "Дарвин"
    "Australia/Perth" "Перт"
    "Australia/Eucla" "Юкла"
    "Asia/Baku" "Баку"
    "America/Barbados" "Барбадос"
    "Asia/Dhaka" "Дакка"
    "Europe/Brussels" "Брюссель"
    "Europe/Sofia" "София"
    "Atlantic/Bermuda" "Бермудские о-ва"
    "America/La_Paz" "Ла-Пас"
    "America/Noronha" "Норонья"
    "America/Belem" "Белен"
    "America/Fortaleza" "Форталеза"
    "America/Recife" "Ресифи"
    "America/Araguaina" "Арагуаина"
    "America/Maceio" "Масейо"
    "America/Bahia" "Баия"
    "America/Sao_Paulo" "Сан-Паулу"
    "America/Campo_Grande" "Кампу-Гранди"
    "America/Cuiaba" "Куяба"
    "America/Santarem" "Сантарен"
    "America/Porto_Velho" "Порту-Велью"
    "America/Boa_Vista" "Боа-Виста"
    "America/Manaus" "Манаус"
    "America/Eirunepe" "Эйрунепе"
    "America/Rio_Branco" "Риу-Бранку"
    "Asia/Thimphu" "Тхимпху"
    "Europe/Minsk" "Минск"
    "America/Belize" "Белиз"
    "America/St_Johns" "Сент-Джонс"
    "America/Halifax" "Галифакс"
    "America/Glace_Bay" "Глейс-Бей"
    "America/Moncton" "Монктон"
    "America/Goose_Bay" "Гус-Бей"
    "America/Toronto" "Торонто"
    "America/Iqaluit" "Икалуит"
    "America/Winnipeg" "Виннипег"
    "America/Resolute" "Резольют"
    "America/Rankin_Inlet" "Ранкин-Инлет"
    "America/Regina" "Реджайна"
    "America/Swift_Current" "Свифт-Керрент"
    "America/Edmonton" "Эдмонтон"
    "America/Cambridge_Bay" "Кеймбридж-Бей"
    "America/Inuvik" "Инувик"
    "America/Vancouver" "Ванкувер"
    "America/Dawson_Creek" "Доусон-Крик"
    "America/Fort_Nelson" "Форт Нельсон"
    "America/Whitehorse" "Уайтхорс"
    "America/Dawson" "Доусон"
    "Europe/Zurich" "Цюрих"
    "Africa/Abidjan" "Абиджан"
    "Pacific/Rarotonga" "Раротонга"
    "America/Santiago" "Сантьяго"
    "America/Coyhaique" "Койайке"
    "America/Punta_Arenas" "Пунта-Аренас"
    "Pacific/Easter" "о-в Пасхи"
    "Asia/Shanghai" "Шанхай"
    "Asia/Urumqi" "Урумчи"
    "America/Bogota" "Богота"
    "America/Costa_Rica" "Коста-Рика"
    "America/Havana" "Гавана"
    "Atlantic/Cape_Verde" "Кабо-Верде"
    "Asia/Nicosia" "Никосия"
    "Asia/Famagusta" "Фамагуста"
    "Europe/Prague" "Прага"
    "Europe/Berlin" "Берлин"
    "America/Santo_Domingo" "Санто-Доминго"
    "Africa/Algiers" "Алжир"
    "America/Guayaquil" "Гуаякиль"
    "Pacific/Galapagos" "Галапагосские о-ва"
    "Europe/Tallinn" "Таллин"
    "Africa/Cairo" "Каир"
    "Africa/El_Aaiun" "Эль-Аюн"
    "Europe/Madrid" "Мадрид"
    "Africa/Ceuta" "Сеута"
    "Atlantic/Canary" "Канарские о-ва"
    "Europe/Helsinki" "Хельсинки"
    "Pacific/Fiji" "Фиджи"
    "Atlantic/Stanley" "Стэнли"
    "Pacific/Kosrae" "Косрае"
    "Atlantic/Faroe" "Фарерские о-ва"
    "Europe/Paris" "Париж"
    "Europe/London" "Лондон"
    "Asia/Tbilisi" "Тбилиси"
    "America/Cayenne" "Кайенна"
    "Europe/Gibraltar" "Гибралтар"
    "America/Nuuk" "Нуук"
    "America/Danmarkshavn" "Денмарксхавн"
    "America/Scoresbysund" "Скорсбисунн"
    "America/Thule" "Туле"
    "Europe/Athens" "Афины"
    "Atlantic/South_Georgia" "Южная Георгия"
    "America/Guatemala" "Гватемала"
    "Pacific/Guam" "Гуам"
    "Africa/Bissau" "Бисау"
    "America/Guyana" "Гайана"
    "Asia/Hong_Kong" "Гонконг"
    "America/Tegucigalpa" "Тегусигальпа"
    "America/Port-au-Prince" "Порт-о-Пренс"
    "Europe/Budapest" "Будапешт"
    "Asia/Jakarta" "Джакарта"
    "Asia/Pontianak" "Понтианак"
    "Asia/Makassar" "Макасар"
    "Asia/Jayapura" "Джаяпура"
    "Europe/Dublin" "Дублин"
    "Asia/Jerusalem" "Иерусалим"
    "Asia/Kolkata" "Калькутта"
    "Indian/Chagos" "Чагос"
    "Asia/Baghdad" "Багдад"
    "Asia/Tehran" "Тегеран"
    "Europe/Rome" "Рим"
    "America/Jamaica" "Ямайка"
    "Asia/Amman" "Амман"
    "Asia/Tokyo" "Токио"
    "Africa/Nairobi" "Найроби"
    "Asia/Bishkek" "Бишкек"
    "Pacific/Tarawa" "Тарава"
    "Pacific/Kanton" "о-в Кантон"
    "Pacific/Kiritimati" "Киритимати"
    "Asia/Pyongyang" "Пхеньян"
    "Asia/Seoul" "Сеул"
    "Asia/Almaty" "Алматы"
    "Asia/Qyzylorda" "Кызылорда"
    "Asia/Qostanay" "Костанай"
    "Asia/Aqtobe" "Актобе"
    "Asia/Aqtau" "Актау"
    "Asia/Atyrau" "Атырау"
    "Asia/Oral" "Уральск"
    "Asia/Beirut" "Бейрут"
    "Asia/Colombo" "Коломбо"
    "Africa/Monrovia" "Монровия"
    "Europe/Vilnius" "Вильнюс"
    "Europe/Riga" "Рига"
    "Africa/Tripoli" "Триполи"
    "Africa/Casablanca" "Касабланка"
    "Europe/Chisinau" "Кишинев"
    "Pacific/Kwajalein" "Кваджалейн"
    "Asia/Yangon" "Янгон"
    "Asia/Ulaanbaatar" "Улан-Батор"
    "Asia/Hovd" "Ховд"
    "Asia/Macau" "Макао"
    "America/Martinique" "Мартиника"
    "Europe/Malta" "Мальта"
    "Indian/Mauritius" "Маврикий"
    "Indian/Maldives" "Мальдивы"
    "America/Mexico_City" "Мехико"
    "America/Cancun" "Канкун"
    "America/Merida" "Мерида"
    "America/Monterrey" "Монтеррей"
    "America/Matamoros" "Матаморос"
    "America/Chihuahua" "Чиуауа"
    "America/Ciudad_Juarez" "Сьюдад-Хуарес"
    "America/Ojinaga" "Охинага"
    "America/Mazatlan" "Масатлан"
    "America/Bahia_Banderas" "Баия-де-Бандерас"
    "America/Hermosillo" "Эрмосильо"
    "America/Tijuana" "Тихуана"
    "Asia/Kuching" "Кучинг"
    "Africa/Maputo" "Мапуту"
    "Africa/Windhoek" "Виндхук"
    "Pacific/Noumea" "Нумеа"
    "Pacific/Norfolk" "Норфолк"
    "Africa/Lagos" "Лагос"
    "America/Managua" "Манагуа"
    "Asia/Kathmandu" "Катманду"
    "Pacific/Nauru" "Науру"
    "Pacific/Niue" "Ниуэ"
    "Pacific/Auckland" "Окленд"
    "Pacific/Chatham" "Чатем"
    "America/Panama" "Панама"
    "America/Lima" "Лима"
    "Pacific/Tahiti" "Таити"
    "Pacific/Marquesas" "Маркизские о-ва"
    "Pacific/Gambier" "о-ва Гамбье"
    "Pacific/Port_Moresby" "Порт-Морсби"
    "Pacific/Bougainville" "Бугенвиль"
    "Asia/Manila" "Манила"
    "Asia/Karachi" "Карачи"
    "Europe/Warsaw" "Варшава"
    "America/Miquelon" "Микелон"
    "Pacific/Pitcairn" "Питкэрн"
    "America/Puerto_Rico" "Пуэрто-Рико"
    "Asia/Gaza" "Газа"
    "Asia/Hebron" "Хеврон"
    "Europe/Lisbon" "Лиссабон"
    "Atlantic/Madeira" "Мадейра"
    "Atlantic/Azores" "Азорские о-ва"
    "Pacific/Palau" "Палау"
    "America/Asuncion" "Асунсьон"
    "Asia/Qatar" "Катар"
    "Europe/Bucharest" "Бухарест"
    "Europe/Belgrade" "Белград"
    "Europe/Kaliningrad" "Калининград"
    "Europe/Moscow" "Москва"
    "Europe/Simferopol" "Симферополь"
    "Europe/Kirov" "Киров"
    "Europe/Volgograd" "Волгоград"
    "Europe/Astrakhan" "Астрахань"
    "Europe/Saratov" "Саратов"
    "Europe/Ulyanovsk" "Ульяновск"
    "Europe/Samara" "Самара"
    "Asia/Yekaterinburg" "Екатеринбург"
    "Asia/Omsk" "Омск"
    "Asia/Novosibirsk" "Новосибирск"
    "Asia/Barnaul" "Барнаул"
    "Asia/Tomsk" "Томск"
    "Asia/Novokuznetsk" "Новокузнецк"
    "Asia/Krasnoyarsk" "Красноярск"
    "Asia/Irkutsk" "Иркутск"
    "Asia/Chita" "Чита"
    "Asia/Yakutsk" "Якутск"
    "Asia/Khandyga" "Хандыга"
    "Asia/Vladivostok" "Владивосток"
    "Asia/Ust-Nera" "Усть-Нера"
    "Asia/Magadan" "Магадан"
    "Asia/Sakhalin" "о-в Сахалин"
    "Asia/Srednekolymsk" "Среднеколымск"
    "Asia/Kamchatka" "Петропавловск-Камчатский"
    "Asia/Anadyr" "Анадырь"
    "Asia/Riyadh" "Эр-Рияд"
    "Pacific/Guadalcanal" "Гуадалканал"
    "Africa/Khartoum" "Хартум"
    "Asia/Singapore" "Сингапур"
    "America/Paramaribo" "Парамарибо"
    "Africa/Juba" "Джуба"
    "Africa/Sao_Tome" "Сан-Томе"
    "America/El_Salvador" "Сальвадор"
    "Asia/Damascus" "Дамаск"
    "America/Grand_Turk" "Гранд-Терк"
    "Africa/Ndjamena" "Нджамена"
    "Asia/Bangkok" "Бангкок"
    "Asia/Dushanbe" "Душанбе"
    "Pacific/Fakaofo" "Факаофо"
    "Asia/Dili" "Дили"
    "Asia/Ashgabat" "Ашхабад"
    "Africa/Tunis" "Тунис"
    "Pacific/Tongatapu" "Тонгатапу"
    "Europe/Istanbul" "Стамбул"
    "Asia/Taipei" "Тайбэй"
    "Europe/Kyiv" "Киев"
    "America/New_York" "Нью-Йорк"
    "America/Detroit" "Детройт"
    "America/Kentucky/Louisville" "Луисвилл"
    "America/Kentucky/Monticello" "Монтиселло, Кентукки"
    "America/Indiana/Indianapolis" "Индианаполис"
    "America/Indiana/Vincennes" "Винсеннес"
    "America/Indiana/Winamac" "Уинамак"
    "America/Indiana/Marengo" "Маренго, Индиана"
    "America/Indiana/Petersburg" "Питерсберг, Индиана"
    "America/Indiana/Vevay" "Вевей, Индиана"
    "America/Chicago" "Чикаго"
    "America/Indiana/Tell_City" "Телл-Сити"
    "America/Indiana/Knox" "Нокс, Индиана"
    "America/Menominee" "Меномини"
    "America/North_Dakota/Center" "Центр, Северная Дакота"
    "America/North_Dakota/New_Salem" "Нью-Сейлем, Северная Дакота"
    "America/North_Dakota/Beulah" "Бойла, Северная Дакота"
    "America/Denver" "Денвер"
    "America/Boise" "Бойсе"
    "America/Phoenix" "Финикс"
    "America/Los_Angeles" "Лос-Анджелес"
    "America/Anchorage" "Анкоридж"
    "America/Juneau" "Джуно"
    "America/Sitka" "Ситка"
    "America/Metlakatla" "Метлакатла"
    "America/Yakutat" "Якутат"
    "America/Nome" "Ном"
    "America/Adak" "Адак"
    "Pacific/Honolulu" "Гонолулу"
    "America/Montevideo" "Монтевидео"
    "Asia/Samarkand" "Самарканд"
    "Asia/Tashkent" "Ташкент"
    "America/Caracas" "Каракас"
    "Asia/Ho_Chi_Minh" "Хошимин"
    "Pacific/Efate" "Эфате"
    "Pacific/Apia" "Апиа"
    "Africa/Johannesburg" "Йоханнесбург"
    "America/Antigua" "Антигуа"
    "America/Anguilla" "Ангилья"
    "Africa/Luanda" "Луанда"
    "Antarctica/McMurdo" "Мак-Мердо"
    "Antarctica/DumontDUrville" "Дюмон-д’Юрвиль"
    "Antarctica/Syowa" "Сёва"
    "America/Aruba" "Аруба"
    "Europe/Mariehamn" "Мариехамн"
    "Europe/Sarajevo" "Сараево"
    "Africa/Ouagadougou" "Уагадугу"
    "Asia/Bahrain" "Бахрейн"
    "Africa/Bujumbura" "Бужумбура"
    "Africa/Porto-Novo" "Порто-Ново"
    "America/St_Barthelemy" "Сен-Бартелеми"
    "Asia/Brunei" "Бруней"
    "America/Kralendijk" "Кралендейк"
    "America/Nassau" "Нассау"
    "Africa/Gaborone" "Габороне"
    "America/Blanc-Sablon" "Бланк-Саблон"
    "America/Atikokan" "Корал-Харбор"
    "America/Creston" "Крестон"
    "Indian/Cocos" "Кокосовые о-ва"
    "Africa/Kinshasa" "Киншаса"
    "Africa/Lubumbashi" "Лубумбаши"
    "Africa/Bangui" "Банги"
    "Africa/Brazzaville" "Браззавиль"
    "Africa/Douala" "Дуала"
    "America/Curacao" "Кюрасао"
    "Indian/Christmas" "о-в Рождества"
    "Europe/Busingen" "Бюзинген-на-Верхнем-Рейне"
    "Africa/Djibouti" "Джибути"
    "Europe/Copenhagen" "Копенгаген"
    "America/Dominica" "Доминика"
    "Africa/Asmara" "Асмэра"
    "Africa/Addis_Ababa" "Аддис-Абеба"
    "Pacific/Chuuk" "Трук"
    "Pacific/Pohnpei" "Понпеи"
    "Africa/Libreville" "Либревиль"
    "America/Grenada" "Гренада"
    "Europe/Guernsey" "Гернси"
    "Africa/Accra" "Аккра"
    "Africa/Banjul" "Банжул"
    "Africa/Conakry" "Конакри"
    "America/Guadeloupe" "Гваделупа"
    "Africa/Malabo" "Малабо"
    "Europe/Zagreb" "Загреб"
    "Europe/Isle_of_Man" "о-в Мэн"
    "Atlantic/Reykjavik" "Рейкьявик"
    "Europe/Jersey" "Джерси"
    "Asia/Phnom_Penh" "Пномпень"
    "Indian/Comoro" "Коморы"
    "America/St_Kitts" "Сент-Китс"
    "Asia/Kuwait" "Кувейт"
    "America/Cayman" "Острова Кайман"
    "Asia/Vientiane" "Вьентьян"
    "America/St_Lucia" "Сент-Люсия"
    "Europe/Vaduz" "Вадуц"
    "Africa/Maseru" "Масеру"
    "Europe/Luxembourg" "Люксембург"
    "Europe/Monaco" "Монако"
    "Europe/Podgorica" "Подгорица"
    "America/Marigot" "Мариго"
    "Indian/Antananarivo" "Антананариву"
    "Pacific/Majuro" "Маджуро"
    "Europe/Skopje" "Скопье"
    "Africa/Bamako" "Бамако"
    "Pacific/Saipan" "Сайпан"
    "Africa/Nouakchott" "Нуакшот"
    "America/Montserrat" "Монтсеррат"
    "Africa/Blantyre" "Блантайр"
    "Asia/Kuala_Lumpur" "Куала-Лумпур"
    "Africa/Niamey" "Ниамей"
    "Europe/Amsterdam" "Амстердам"
    "Europe/Oslo" "Осло"
    "Asia/Muscat" "Маскат"
    "Indian/Reunion" "Реюньон"
    "Africa/Kigali" "Кигали"
    "Indian/Mahe" "Маэ"
    "Europe/Stockholm" "Стокгольм"
    "Atlantic/St_Helena" "о-в Святой Елены"
    "Europe/Ljubljana" "Любляна"
    "Arctic/Longyearbyen" "Лонгйир"
    "Europe/Bratislava" "Братислава"
    "Africa/Freetown" "Фритаун"
    "Europe/San_Marino" "Сан-Марино"
    "Africa/Dakar" "Дакар"
    "Africa/Mogadishu" "Могадишо"
    "America/Lower_Princes" "Лоуэр-Принс-Куотер"
    "Africa/Mbabane" "Мбабане"
    "Indian/Kerguelen" "Кергелен"
    "Africa/Lome" "Ломе"
    "America/Port_of_Spain" "Порт-оф-Спейн"
    "Pacific/Funafuti" "Фунафути"
    "Africa/Dar_es_Salaam" "Дар-эс-Салам"
    "Africa/Kampala" "Кампала"
    "Pacific/Midway" "о-ва Мидуэй"
    "Pacific/Wake" "Уэйк"
    "Europe/Vatican" "Ватикан"
    "America/St_Vincent" "Сент-Винсент"
    "America/Tortola" "Тортола"
    "America/St_Thomas" "Сент-Томас"
    "Pacific/Wallis" "Уоллис"
    "Asia/Aden" "Аден"
    "Indian/Mayotte" "Майотта"
    "Africa/Lusaka" "Лусака"
    "Africa/Harare" "Хараре"
};

// `common/main/syr.xml`: 417 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const SYR: &str = exemplar_cities! {
    "Europe/Andorra" "ܐܢܕܘܪܐ"
    "Asia/Dubai" "ܕܘܒܝ"
    "Asia/Kabul" "ܟܐܒܘܠ"
    "Europe/Tirane" "ܬܝܪܐܢ"
    "Asia/Yerevan" "ܝܪܒܐܢ"
    "Antarctica/Casey" "ܟܐܝܣܝ"
    "Antarctica/Davis" "ܕܒܝܣ"
    "Antarctica/Mawson" "ܡܐܘܣܘܢ"
    "Antarctica/Palmer" "ܦܐܠܡܝܪ"
    "Antarctica/Rothera" "ܪܘܬܝܪܐ"
    "Antarctica/Troll" "ܬܪܘܠ"
    "Antarctica/Vostok" "ܒܘܣܬܘܟ"
    "America/Argentina/Buenos_Aires" "ܒܘܐܝܢܘܣ ܥܝܪܣ"
    "America/Argentina/Cordoba" "ܟܘܪܕܘܒܐ"
    "America/Argentina/Salta" "ܣܠܬܐ"
    "America/Argentina/Jujuy" "ܓܘܓܘܝ"
    "America/Argentina/Tucuman" "ܬܘܟܘܡܐܢ"
    "America/Argentina/Catamarca" "ܟܐܬܐܡܪܟܐ"
    "America/Argentina/La_Rioja" "ܠܐ ܪܝܘܗܐ"
    "America/Argentina/San_Juan" "ܣܐܢ ܘܐܢ"
    "America/Argentina/Mendoza" "ܡܢܕܘܙܐ"
    "America/Argentina/San_Luis" "ܣܐܢ ܠܘܝܣ"
    "America/Argentina/Rio_Gallegos" "ܪܝܘ ܓܝܓܘܣ"
    "America/Argentina/Ushuaia" "ܐܘܫܘܐܝܐ"
    "Pacific/Pago_Pago" "ܦܐܓܘ ܦܐܓܘ"
    "Europe/Vienna" "ܒܝܝܢܐ"
    "Australia/Lord_Howe" "ܠܘܪܕ ܗܐܘ"
    "Antarctica/Macquarie" "ܡܐܟܐܘܪܝ"
    "Australia/Hobart" "ܗܘܒܪܬ"
    "Australia/Melbourne" "ܡܝܠܒܘܪܢ"
    "Australia/Sydney" "ܣܝܕܢܝ"
    "Australia/Broken_Hill" "ܒܪܘܟܝܢ ܗܝܠ"
    "Australia/Brisbane" "ܒܪܝܣܒܐܢ"
    "Australia/Lindeman" "ܠܝܢܕܡܐܢ"
    "Australia/Adelaide" "ܐܕܝܠܝܕ"
    "Australia/Darwin" "ܕܪܘܝܢ"
    "Australia/Perth" "ܦܝܪܬ"
    "Australia/Eucla" "ܐܘܟܠܐ"
    "Asia/Baku" "ܒܐܟܘ"
    "America/Barbados" "ܒܐܪܒܕܘܣ"
    "Asia/Dhaka" "ܕܟܐ"
    "Europe/Brussels" "ܒܪܘܟܣܠ"
    "Europe/Sofia" "ܣܘܦܝܐ"
    "Atlantic/Bermuda" "ܒܝܪܡܘܕܐ"
    "America/La_Paz" "ܠܐ ܦܐܙ"
    "America/Noronha" "ܢܘܪܘܢܗܐ"
    "America/Belem" "ܒܝܠܝܡ"
    "America/Fortaleza" "ܦܘܪܬܐܠܝܙܐ"
    "America/Recife" "ܪܝܣܝܦܝ"
    "America/Araguaina" "ܐܪܐܓܐܘܝܢܐ"
    "America/Maceio" "ܡܐܣܝܐܘ"
    "America/Bahia" "ܒܐܗܝܐ"
    "America/Sao_Paulo" "ܣܐܘ ܦܐܘܠܘ"
    "America/Campo_Grande" "ܟܐܡܦܘ ܓܪܢܕܝ"
    "America/Cuiaba" "ܟܘܝܐܒܐ"
    "America/Santarem" "ܣܐܢܬܐܪܡ"
    "America/Porto_Velho" "ܦܘܪܬܘ ܒܝܠܗܘ"
    "America/Boa_Vista" "ܒܘܥ ܒܝܣܬܐ"
    "America/Manaus" "ܡܐܢܐܘܣ"
    "America/Eirunepe" "ܐܝܪܘܢܝܦܝ"
    "America/Rio_Branco" "ܪܝܘ ܒܪܢܟܘ"
    "Asia/Thimphu" "ܬܝܡܦܘ"
    "Europe/Minsk" "ܡܝܢܣܟ"
    "America/Belize" "ܒܝܠܝܙ"
    "America/St_Johns" "ܡܪܝ ܝܘܚܢܢ"
    "America/Halifax" "ܗܠܝܦܐܟܣ"
    "America/Glace_Bay" "ܓܠܝܣ ܒܐܝ"
    "America/Moncton" "ܡܘܢܟܬܘܢ"
    "America/Goose_Bay" "ܓܘܣ ܒܐܝ"
    "America/Toronto" "ܬܘܪܘܢܬܘ"
    "America/Iqaluit" "ܐܝܩܠܘܝܬ"
    "America/Winnipeg" "ܘܝܢܝܦܓ"
    "America/Resolute" "ܪܝܣܘܠܘܬ"
    "America/Rankin_Inlet" "ܪܐܢܟܢ ܐܢܠܝܬ"
    "America/Regina" "ܪܝܓܝܢܐ"
    "America/Swift_Current" "ܢܕܘܪܬܐ"
    "America/Edmonton" "ܐܕܡܘܢܬܘܢ"
    "America/Cambridge_Bay" "ܟܡܒܪܓ ܒܐܝ"
    "America/Inuvik" "ܐܢܘܒܝܟ"
    "America/Vancouver" "ܒܢܟܘܒܝܪ"
    "America/Dawson_Creek" "ܕܐܣܘܢ ܟܪܝܟ"
    "America/Fort_Nelson" "ܦܘܪܬ ܢܝܠܣܘܢ"
    "America/Whitehorse" "ܣܘܣܬܐ ܚܘܪܬܐ"
    "America/Dawson" "ܕܐܣܘܢ"
    "Europe/Zurich" "ܙܝܘܪܟ"
    "Africa/Abidjan" "ܐܒܕܓܢ"
    "Pacific/Rarotonga" "ܪܐܪܘܬܘܢܓܐ"
    "America/Santiago" "ܣܐܢܬܝܐܓܘ"
    "America/Coyhaique" "ܟܘܝܐܝܟܐ"
    "America/Punta_Arenas" "ܦܘܢܬܐ ܥܪܝܢܣ"
    "Pacific/Easter" "ܦܨܚܐ"
    "Asia/Shanghai" "ܫܢܓܗܐܝܝ"
    "Asia/Urumqi" "ܐܘܪܘܡܟܝ"
    "America/Bogota" "ܒܘܓܘܬܐ"
    "America/Costa_Rica" "ܟܘܣܬܐ ܪܝܟܐ"
    "America/Havana" "ܗܐܒܐܢܐ"
    "Atlantic/Cape_Verde" "ܟܐܦ ܒܝܪܕܝ"
    "Asia/Nicosia" "ܢܝܩܘܣܝܐ"
    "Asia/Famagusta" "ܦܐܡܐܓܘܣܬܐ"
    "Europe/Prague" "ܦܪܐܓ"
    "Europe/Berlin" "ܒܪܠܝܢ"
    "America/Santo_Domingo" "ܣܢܬܘ ܕܘܡܝܢܓܘ"
    "Africa/Algiers" "ܓܙܐܐܪ"
    "America/Guayaquil" "ܓܘܐܝܐܩܘܝܠ"
    "Pacific/Galapagos" "ܓܐܠܐܦܓܘܣ"
    "Europe/Tallinn" "ܬܐܠܝܢ"
    "Africa/Cairo" "ܩܐܗܪܗ"
    "Africa/El_Aaiun" "ܐܠ ܥܝܘܢ"
    "Europe/Madrid" "ܡܕܪܝܕ"
    "Africa/Ceuta" "ܣܒܬܐ"
    "Atlantic/Canary" "ܟܐܢܪܝ"
    "Europe/Helsinki" "ܗܠܣܢܟܝ"
    "Pacific/Fiji" "ܦܝܓܝ"
    "Atlantic/Stanley" "ܣܬܐܢܠܝ"
    "Pacific/Kosrae" "ܟܘܣܪܐܝ"
    "Atlantic/Faroe" "ܦܐܪܘ"
    "Europe/Paris" "ܦܐܪܝܣ"
    "Europe/London" "ܠܘܢܕܘܢ"
    "Asia/Tbilisi" "ܬܦܠܝܣ"
    "America/Cayenne" "ܟܐܝܐܢ"
    "Europe/Gibraltar" "ܓܒܪܠܛܪ"
    "America/Nuuk" "ܢܘܟ"
    "America/Danmarkshavn" "ܕܐܢܡܪܟܫܒܝܢ"
    "America/Scoresbysund" "ܐܝܛܘܩܘܪܡܝܬ"
    "America/Thule" "ܬܘܠ"
    "Europe/Athens" "ܐܬܢܘܣ"
    "Atlantic/South_Georgia" "ܬܡܝܢ ܓܘܪܓܝܐ"
    "America/Guatemala" "ܓܘܐܬܡܐܠܐ"
    "Pacific/Guam" "ܓܘܐܡ"
    "Africa/Bissau" "ܒܝܣܐܘ"
    "America/Guyana" "ܓܘܝܐܢܐ"
    "Asia/Hong_Kong" "ܗܘܢܓ ܟܘܢܓ"
    "America/Tegucigalpa" "ܬܝܓܘܣܝܓܐܠܦܐ"
    "America/Port-au-Prince" "ܦܘܪܬ ܐܘ ܦܪܝܢܣ"
    "Europe/Budapest" "ܒܘܕܦܫܛ"
    "Asia/Jakarta" "ܓܐܟܐܪܬܐ"
    "Asia/Pontianak" "ܦܘܢܬܝܐܢܐܟ"
    "Asia/Makassar" "ܡܐܟܐܣܐܪ"
    "Asia/Jayapura" "ܓܐܝܦܘܪܐ"
    "Europe/Dublin" "ܕܒܠܢ"
    "Asia/Jerusalem" "ܐܘܪܫܠܡ"
    "Asia/Kolkata" "ܟܘܠܟܬܐ"
    "Indian/Chagos" "ܬܫܓܘܣ"
    "Asia/Baghdad" "ܒܓܕܕ"
    "Asia/Tehran" "ܬܗܪܢ"
    "Europe/Rome" "ܪܗܘܡܐ"
    "America/Jamaica" "ܓܡܐܝܟܐ"
    "Asia/Amman" "ܥܡܐܢ"
    "Asia/Tokyo" "ܛܘܟܝܘ"
    "Africa/Nairobi" "ܢܝܪܘܒܝ"
    "Asia/Bishkek" "ܒܝܫܟܝܟ"
    "Pacific/Tarawa" "ܬܐܪܐܘܐ"
    "Pacific/Kanton" "ܟܐܢܬܘܢ"
    "Pacific/Kiritimati" "ܟܝܪܝܡܐܬܝ"
    "Asia/Pyongyang" "ܦܝܘܢܓܝܢܓ"
    "Asia/Seoul" "ܣܐܘܠ"
    "Asia/Almaty" "ܐܠܡܐܬܝ"
    "Asia/Qyzylorda" "ܟܝܙܝܠܘܪܕܐ"
    "Asia/Qostanay" "ܟܘܣܬܐܢܐܝ"
    "Asia/Aqtobe" "ܐܟܬܘܒ"
    "Asia/Aqtau" "ܐܟܬܐܘ"
    "Asia/Atyrau" "ܐܬܝܪܘ"
    "Asia/Oral" "ܐܘܪܐܠ"
    "Asia/Beirut" "ܒܝܪܘܬ"
    "Asia/Colombo" "ܟܘܠܘܡܒܘ"
    "Africa/Monrovia" "ܡܘܢܪܘܒܝܐ"
    "Europe/Vilnius" "ܒܠܢܘܣ"
    "Europe/Riga" "ܪܝܓܐ"
    "Africa/Tripoli" "ܛܪܝܦܘܠܝܣ"
    "Africa/Casablanca" "ܟܐܣܐܒܠܢܟܐ"
    "Europe/Chisinau" "ܟܝܣܝܢܐܘ"
    "Pacific/Kwajalein" "ܟܘܐܓܐܠܝܢ"
    "Asia/Yangon" "ܝܢܓܘܢ"
    "Asia/Ulaanbaatar" "ܐܘܠܐܢܒܐܬܘܪ"
    "Asia/Hovd" "ܗܘܒܕ"
    "Asia/Macau" "ܡܐܟܐܘ"
    "America/Martinique" "ܡܐܪܬܝܢܝܩ"
    "Europe/Malta" "ܡܝܠܛܐ"
    "Indian/Mauritius" "ܡܘܪܝܫܘܣ"
    "Indian/Maldives" "ܓܙܪܬܐ ܡܐܠܕܝܒܝܬܐ"
    "America/Mexico_City" "ܡܕܝܢܬܐ ܕܡܟܣܝܟܘ"
    "America/Cancun" "ܟܐܢܟܘܢ"
    "America/Merida" "ܡܪܝܕܐ"
    "America/Monterrey" "ܡܘܢܛܪܐܝ"
    "America/Matamoros" "ܡܐܬܐܡܘܪܘܣ"
    "America/Chihuahua" "ܟܝܘܐܘܐ"
    "America/Ciudad_Juarez" "ܣܝܘܕܐܕ ܐܘܪܝܙ"
    "America/Ojinaga" "ܘܓܝܢܐܓܐ"
    "America/Mazatlan" "ܡܙܛܠܐܢ"
    "America/Bahia_Banderas" "ܒܐܗܝܐ ܒܐܢܝܪܣ"
    "America/Hermosillo" "ܗܝܪܡܘܣܝܐ"
    "America/Tijuana" "ܬܝܐܘܐܢܐ"
    "Asia/Kuching" "ܟܘܫܝܢܓ"
    "Africa/Maputo" "ܡܐܦܘܬܘ"
    "Africa/Windhoek" "ܘܝܢܕܗܘܟ"
    "Pacific/Noumea" "ܢܘܡܝܐ"
    "Pacific/Norfolk" "ܢܘܪܦܠܟ"
    "Africa/Lagos" "ܠܐܓܘܣ"
    "America/Managua" "ܡܐܢܐܓܘܐ"
    "Asia/Kathmandu" "ܟܐܬܡܐܢܕܘ"
    "Pacific/Nauru" "ܢܐܘܪܘ"
    "Pacific/Niue" "ܢܝܘܝ"
    "Pacific/Auckland" "ܐܟܠܐܢܕ"
    "Pacific/Chatham" "ܬܫܐܬܡ"
    "America/Panama" "ܦܢܡܐ"
    "America/Lima" "ܠܝܡܐ"
    "Pacific/Tahiti" "ܬܐܗܝܬܝ"
    "Pacific/Marquesas" "ܡܐܪܟܐܘܣܐܣ"
    "Pacific/Gambier" "ܓܡܒܝܪ"
    "Pacific/Port_Moresby" "ܦܘܪܬ ܡܘܪܝܣܒܐܝ"
    "Pacific/Bougainville" "ܒܘܓܐܝܢܒܝܠ"
    "Asia/Manila" "ܡܐܢܝܠܐ"
    "Asia/Karachi" "ܟܪܐܟܝ"
    "Europe/Warsaw" "ܘܐܪܣܘ"
    "America/Miquelon" "ܡܩܘܠܘܢ"
    "Pacific/Pitcairn" "ܦܝܬܟܐܝܪܢ"
    "America/Puerto_Rico" "ܦܘܪܬܘ ܪܝܟܘ"
    "Asia/Gaza" "ܥܙܐ"
    "Asia/Hebron" "ܚܒܪܘܢ"
    "Europe/Lisbon" "ܠܫܒܘܢܐ"
    "Atlantic/Madeira" "ܡܕܐܝܪܐ"
    "Atlantic/Azores" "ܓܙܪܬܐ ܕܐܙܘܪ"
    "Pacific/Palau" "ܦܠܐܘ"
    "America/Asuncion" "ܐܣܘܢܟܣܝܘܢ"
    "Asia/Qatar" "ܩܛܪ"
    "Europe/Bucharest" "ܒܘܩܘܪܫܛ"
    "Europe/Belgrade" "ܒܠܓܪܕ"
    "Europe/Kaliningrad" "ܟܐܠܝܢܝܢܓܪܐܕ"
    "Europe/Moscow" "ܡܘܣܟܘ"
    "Europe/Simferopol" "ܣܡܦܪܘܦܠ"
    "Europe/Kirov" "ܟܝܪܘܒ"
    "Europe/Volgograd" "ܒܘܠܓܘܓܪܐܕ"
    "Europe/Astrakhan" "ܐܣܬܪܐܚܢ"
    "Europe/Saratov" "ܣܪܐܬܘܒ"
    "Europe/Ulyanovsk" "ܐܘܠܝܢܘܒܣܟ"
    "Europe/Samara" "ܣܡܐܪܐ"
    "Asia/Yekaterinburg" "ܝܟܐܬܝܪܢܒܝܪܓ"
    "Asia/Omsk" "ܐܘܡܣܟ"
    "Asia/Novosibirsk" "ܢܘܒܘܣܝܒܪܣܟ"
    "Asia/Barnaul" "ܒܐܪܢܐܘܠ"
    "Asia/Tomsk" "ܬܘܡܣܟ"
    "Asia/Novokuznetsk" "ܢܘܒܘܟܘܙܢܝܬܣܟ"
    "Asia/Krasnoyarsk" "ܟܪܐܣܢܘܝܪܣܟ"
    "Asia/Irkutsk" "ܐܝܪܟܘܬܣܟ"
    "Asia/Chita" "ܬܫܝܬܐ"
    "Asia/Yakutsk" "ܝܐܟܘܬܣܟ"
    "Asia/Khandyga" "ܚܐܢܕܝܓܐ"
    "Asia/Vladivostok" "ܒܠܐܕܝܒܘܣܬܘܟ"
    "Asia/Ust-Nera" "ܐܘܣܬ-ܢܝܪܐ"
    "Asia/Magadan" "ܡܐܓܐܕܐܢ"
    "Asia/Sakhalin" "ܣܐܚܐܠܝܢ"
    "Asia/Srednekolymsk" "ܣܪܝܕܢܝܟܘܠܝܡܣܟ"
    "Asia/Kamchatka" "ܟܐܡܬܫܐܬܟܐ"
    "Asia/Anadyr" "ܐܢܐܕܝܪ"
    "Asia/Riyadh" "ܪܝܐܨ"
    "Pacific/Guadalcanal" "ܓܘܐܕܐܠܟܐܢܐܠ"
    "Africa/Khartoum" "ܚܪܛܘܡ"
    "Asia/Singapore" "ܣܝܢܓܐܦܘܪ"
    "America/Paramaribo" "ܦܐܪܐܡܐܪܝܒܘ"
    "Africa/Juba" "ܓܘܒܐ"
    "Africa/Sao_Tome" "ܣܐܘ ܬܘܡܝ"
    "America/El_Salvador" "ܐܠ ܣܠܒܐܕܘܪ"
    "Asia/Damascus" "ܕܪܡܣܘܩ"
    "America/Grand_Turk" "ܓܪܐܢܕ ܬܘܪܟ"
    "Africa/Ndjamena" "ܢܓܡܝܢܐ"
    "Asia/Bangkok" "ܒܐܢܟܘܟ"
    "Asia/Dushanbe" "ܕܘܫܐܢܒܝ"
    "Pacific/Fakaofo" "ܦܐܟܐܘܦܘ"
    "Asia/Dili" "ܕܝܠܝ"
    "Asia/Ashgabat" "ܥܫܩܐܒܐܕ"
    "Africa/Tunis" "ܬܘܢܣ"
    "Pacific/Tongatapu" "ܬܘܢܓܐܬܐܦܘ"
    "Europe/Istanbul" "ܐܣܛܢܒܘܠ"
    "Asia/Taipei" "ܬܐܝܦܐܝ"
    "Europe/Kyiv" "ܟܝܝܒ"
    "America/New_York" "ܢܝܘ ܝܘܪܟ"
    "America/Detroit" "ܕܝܬܪܘܝܬ"
    "America/Kentucky/Louisville" "ܠܘܝܣܒܝܠ"
    "America/Kentucky/Monticello" "ܡܘܢܬܐܟܝܠܘ، ܟܝܢܬܐܟܝ"
    "America/Indiana/Indianapolis" "ܐܢܕܝܐܢܐܦܘܠܝܣ"
    "America/Indiana/Vincennes" "ܒܝܢܣܝܢܝܣ، ܐܢܕܝܐܢܐ"
    "America/Indiana/Winamac" "ܘܝܢܐܡܐܟ، ܐܢܕܝܐܢܐ"
    "America/Indiana/Marengo" "ܡܪܝܢܓܘ، ܐܢܕܝܐܢܐ"
    "America/Indiana/Petersburg" "ܦܝܬܝܪܣܒܝܪܓ، ܐܢܕܝܐܢܐ"
    "America/Indiana/Vevay" "ܒܝܒܐܝ، ܐܢܕܝܐܢܐ"
    "America/Chicago" "ܫܟܓܘ"
    "America/Indiana/Tell_City" "ܡܢܕܝܬܐ ܕܬܝܠ، ܐܢܕܝܐܢܐ"
    "America/Indiana/Knox" "ܢܘܟܣ، ܐܢܕܝܐܢܐ"
    "America/Menominee" "ܡܢܘܡܝܢܝ"
    "America/North_Dakota/Center" "ܣܝܢܬܪ، ܕܐܟܘܬܐ ܓܪܒܝܝܬܐ"
    "America/North_Dakota/New_Salem" "ܢܝܘ ܣܐܠܝܡ،‌ ܕܐܟܘܬܐ ܓܪܒܝܝܬܐ"
    "America/North_Dakota/Beulah" "ܒܝܘܠܐ، ܕܐܟܘܬܐ ܓܪܒܝܝܬܐ"
    "America/Denver" "ܕܢܒܪ"
    "America/Boise" "ܒܘܝܙܝ"
    "America/Phoenix" "ܦܝܢܝܟܣ"
    "America/Los_Angeles" "ܠܘܣ ܐܢܓܠܘܣ"
    "America/Anchorage" "ܐܢܟܘܪܓ"
    "America/Juneau" "ܓܘܢܘ"
    "America/Sitka" "ܣܝܛܟܐ"
    "America/Metlakatla" "ܡܛܠܟܐܬܠܐ"
    "America/Yakutat" "ܝܩܘܬܐܬ"
    "America/Nome" "ܢܘܡ"
    "America/Adak" "ܐܕܐܟ"
    "Pacific/Honolulu" ""
    "America/Montevideo" "ܡܘܢܬܝܒܝܕܝܘ"
    "Asia/Samarkand" "ܣܡܪܟܢܕ"
    "Asia/Tashkent" "ܬܫܟܝܢܬ"
    "America/Caracas" "ܟܐܪܐܟܣ"
    "Asia/Ho_Chi_Minh" "ܡܕܝܢܬܐ ܕܗܘ ܟܝ ܡܝܢ"
    "Pacific/Efate" "ܝܦܐܬ"
    "Pacific/Apia" "ܐܦܝܐ"
    "Africa/Johannesburg" "ܝܘܗܢܝܣܒܘܪܓ"
    "America/Antigua" "ܐܢܬܝܓܘܐ"
    "America/Anguilla" "ܐܢܓܘܝܐ"
    "Africa/Luanda" "ܠܘܐܢܕܐ"
    "Antarctica/McMurdo" "ܡܟܡܘܪܕܘ"
    "Antarctica/DumontDUrville" "ܕܘܡܘܢܬ ܕܐܘܪܒܝܠ"
    "Antarctica/Syowa" "ܣܝܘܐ"
    "America/Aruba" "ܐܪܘܒܐ"
    "Europe/Mariehamn" "ܡܐܪܝܗܐܡ"
    "Europe/Sarajevo" "ܣܪܐܝܝܒܘ"
    "Africa/Ouagadougou" "ܐܘܐܓܐܕܐܘܓܐܘ"
    "Asia/Bahrain" "ܒܚܪܝܢ"
    "Africa/Bujumbura" "ܒܘܓܘܡܒܘܪܐ"
    "Africa/Porto-Novo" "ܦܘܪܬܘ-ܢܘܒܘ"
    "America/St_Barthelemy" "ܡܪܝ ܒܪ ܬܘܠܡܝ"
    "Asia/Brunei" "ܒܪܘܢܐܝ"
    "America/Kralendijk" "ܟܪܠܝܢܓܩ"
    "America/Nassau" "ܢܐܣܐܘ"
    "Africa/Gaborone" "ܓܒܘܪܘܢ"
    "America/Blanc-Sablon" "ܒܠܐܢܟ-ܣܐܒܠܘܢ"
    "America/Atikokan" "ܐܬܝܟܘܟܐܢ"
    "America/Creston" "ܟܪܝܣܬܘܢ"
    "Indian/Cocos" "ܟܘܟܘܣ"
    "Africa/Kinshasa" "ܟܝܢܫܐܣܐ"
    "Africa/Lubumbashi" "ܠܘܒܘܡܒܫܝ"
    "Africa/Bangui" "ܒܐܢܓܐܘܝ"
    "Africa/Brazzaville" "ܒܪܐܙܐܒܝܠ"
    "Africa/Douala" "ܕܘܐܠܐ"
    "America/Curacao" "ܟܘܪܐܟܐܘ"
    "Indian/Christmas" "ܟܪܝܣܬܡܣ"
    "Europe/Busingen" "ܒܘܣܝܢܓܢ"
    "Africa/Djibouti" "ܓܝܒܘܬܝ"
    "Europe/Copenhagen" "ܟܘܦܢܗܐܓܢ"
    "America/Dominica" "ܕܘܡܝܢܝܟܐ"
    "Africa/Asmara" "ܐܣܡܐܪܐ"
    "Africa/Addis_Ababa" "ܐܕܝܣ ܐܒܒܐ"
    "Pacific/Chuuk" "ܬܫܘܟ"
    "Pacific/Pohnpei" "ܦܘܗܢܦܐܝ"
    "Africa/Libreville" "ܠܝܒܪܝܒܝܠ"
    "America/Grenada" "ܓܪܝܢܕܐ"
    "Europe/Guernsey" "ܓܘܪܢܙܝ"
    "Africa/Accra" "ܐܟܪܐ"
    "Africa/Banjul" "ܒܐܢܓܘܠ"
    "Africa/Conakry" "ܟܘܢܐܟܪܝ"
    "America/Guadeloupe" "ܓܘܐܕܐܠܘܦܝ"
    "Africa/Malabo" "ܡܐܠܐܒܘ"
    "Europe/Zagreb" "ܙܐܓܪܒ"
    "Europe/Isle_of_Man" "ܓܙܪܬܐ ܕܡܐܢ"
    "Atlantic/Reykjavik" "ܪܐܝܟܒܝܟ"
    "Europe/Jersey" "ܓܝܪܙܝ"
    "Asia/Phnom_Penh" "ܦܢܘܡ ܦܢ"
    "Indian/Comoro" "ܟܘܡܘܪܘ"
    "America/St_Kitts" "ܣܐܢܬ ܟܬܣ"
    "Asia/Kuwait" "ܟܘܝܬ"
    "America/Cayman" "ܟܐܝܡܝܢ"
    "Asia/Vientiane" "ܒܝܐܢܬܝܐܢ"
    "America/St_Lucia" "ܡܪܬܝ ܠܘܫܐ"
    "Europe/Vaduz" "ܒܕܘܙ"
    "Africa/Maseru" "ܡܐܣܝܪܘ"
    "Europe/Luxembourg" "ܠܘܟܣܡܒܘܪܓ"
    "Europe/Monaco" "ܡܘܢܐܟܘ"
    "Europe/Podgorica" "ܦܘܕܓܘܪܝܟܐ"
    "America/Marigot" "ܡܪܝܓܘܬ"
    "Indian/Antananarivo" "ܐܢܬܐܢܐܢܪܝܒܘ"
    "Pacific/Majuro" "ܡܐܓܘܪܘ"
    "Europe/Skopje" "ܣܩܘܦܝܐ"
    "Africa/Bamako" "ܒܐܡܐܟܘ"
    "Pacific/Saipan" "ܣܐܝܦܐܢ"
    "Africa/Nouakchott" "ܢܘܐܟܫܘܬ"
    "America/Montserrat" "ܡܘܢܬܣܝܪܐܬ"
    "Africa/Blantyre" "ܒܠܢܬܝܪ"
    "Asia/Kuala_Lumpur" "ܟܘܐܠܐ ܠܘܡܦܘܪ"
    "Africa/Niamey" "ܢܝܐܡܝ"
    "Europe/Amsterdam" "ܐܡܣܬܪܕܡ"
    "Europe/Oslo" "ܐܘܣܠܘ"
    "Asia/Muscat" "ܡܣܩܛ"
    "Indian/Reunion" "ܪܝܘܢܝܘܢ"
    "Africa/Kigali" "ܟܝܓܐܠܝ"
    "Indian/Mahe" "ܡܐܗܝ"
    "Europe/Stockholm" "ܣܬܘܟܗܘܠܡ"
    "Atlantic/St_Helena" "ܡܪܬܝ ܗܝܠܝܢܐ"
    "Europe/Ljubljana" "ܠܝܘܒܠܝܐܢܐ"
    "Arctic/Longyearbyen" "ܠܘܢܓܝܥܪܒܝܝܢ"
    "Europe/Bratislava" "ܒܪܬܝܣܠܒܐ‏"
    "Africa/Freetown" "ܦܪܝܬܐܘܢ"
    "Europe/San_Marino" "ܣܢ ܡܪܝܢܘ"
    "Africa/Dakar" "ܕܐܟܐܪ"
    "Africa/Mogadishu" "ܡܘܩܕܝܫܘ"
    "America/Lower_Princes" "ܪܘܒ݂ܥܐ ܕܫܠܝܛܐ ܬܚܬܝܐ"
    "Africa/Mbabane" "ܡܒܐܒܐܢܝ"
    "Indian/Kerguelen" "ܟܝܪܓܘܠܝܢ"
    "Africa/Lome" "ܠܘܡܝ"
    "America/Port_of_Spain" "ܦܘܪܬ ܕܐܣܦܢܝܐ"
    "Pacific/Funafuti" "ܦܘܢܐܦܘܬܝ"
    "Africa/Dar_es_Salaam" "ܕܐܪ ܫܠܡܐ"
    "Africa/Kampala" "ܟܐܡܦܐܠܐ"
    "Pacific/Midway" "ܡܝܕܘܐܝ"
    "Pacific/Wake" "ܘܐܝܟ"
    "Europe/Vatican" "ܘܐܬܝܩܐܢ"
    "America/St_Vincent" "ܡܪܝ ܒܢܣܢܬ"
    "America/Tortola" "ܬܘܪܬܘܠܐ"
    "America/St_Thomas" "ܡܪܝ ܬܐܘܡܐ"
    "Pacific/Wallis" "ܘܝܠܝܣ"
    "Asia/Aden" "ܥܕܢ"
    "Indian/Mayotte" "ܡܐܝܘܬ"
    "Africa/Lusaka" "ܠܘܣܐܟܐ"
    "Africa/Harare" "ܗܪܐܪܝ"
};

// `common/main/ta.xml`: 418 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const TA: &str = exemplar_cities! {
    "Europe/Andorra" "அண்டோரா"
    "Asia/Dubai" "துபாய்"
    "Asia/Kabul" "காபூல்"
    "Europe/Tirane" "திரானே"
    "Asia/Yerevan" "ஏரேவன்"
    "Antarctica/Casey" "கேஸி"
    "Antarctica/Davis" "டேவிஸ்"
    "Antarctica/Mawson" "மாசன்"
    "Antarctica/Palmer" "பால்மர்"
    "Antarctica/Rothera" "ரோதேரா"
    "Antarctica/Troll" "ட்ரோல்"
    "Antarctica/Vostok" "வோஸ்டோக்"
    "America/Argentina/Buenos_Aires" "ப்யூனோஸ் ஏர்ஸ்"
    "America/Argentina/Cordoba" "கார்டோபா"
    "America/Argentina/Salta" "சால்டா"
    "America/Argentina/Jujuy" "ஜூஜுய்"
    "America/Argentina/Tucuman" "டுகுமன்"
    "America/Argentina/Catamarca" "கடமார்கா"
    "America/Argentina/La_Rioja" "லா ரியோஜா"
    "America/Argentina/San_Juan" "சான் ஜுவான்"
    "America/Argentina/Mendoza" "மென்டோஸா"
    "America/Argentina/San_Luis" "சான் லூயிஸ்"
    "America/Argentina/Rio_Gallegos" "ரியோ கேலெகோஸ்"
    "America/Argentina/Ushuaia" "உஷுவாயா"
    "Pacific/Pago_Pago" "பேகோ பேகோ"
    "Europe/Vienna" "வியன்னா"
    "Australia/Lord_Howe" "லார்ட் ஹோவே"
    "Antarctica/Macquarie" "மாக்கியூரி"
    "Australia/Hobart" "ஹோபர்ட்"
    "Australia/Melbourne" "மெல்போர்ன்"
    "Australia/Sydney" "சிட்னி"
    "Australia/Broken_Hill" "புரோக்கன் ஹில்"
    "Australia/Brisbane" "பிரிஸ்பேன்"
    "Australia/Lindeman" "லின்டெமன்"
    "Australia/Adelaide" "அடிலெய்ட்"
    "Australia/Darwin" "டார்வின்"
    "Australia/Perth" "பெர்த்"
    "Australia/Eucla" "யூக்லா"
    "Asia/Baku" "பாக்கூ"
    "America/Barbados" "பார்படாஸ்"
    "Asia/Dhaka" "டாக்கா"
    "Europe/Brussels" "புரூசல்ஸ்"
    "Europe/Sofia" "சோஃபியா"
    "Atlantic/Bermuda" "பெர்முடா"
    "America/La_Paz" "லா பாஸ்"
    "America/Noronha" "நோரன்ஹா"
    "America/Belem" "பெலெம்"
    "America/Fortaleza" "ஃபோர்டாலெசா"
    "America/Recife" "ரெஸிஃபி"
    "America/Araguaina" "அரகுவாய்னா"
    "America/Maceio" "மேசியோ"
    "America/Bahia" "பாஹியா"
    "America/Sao_Paulo" "சாவோ பவுலோ"
    "America/Campo_Grande" "கேம்போ கிராண்டே"
    "America/Cuiaba" "குயாபே"
    "America/Santarem" "சான்டரெம்"
    "America/Porto_Velho" "போர்ட்டோ வெல்ஹோ"
    "America/Boa_Vista" "போவா விஸ்டா"
    "America/Manaus" "மனாஸ்"
    "America/Eirunepe" "ஈருனெபே"
    "America/Rio_Branco" "ரியோ பிரான்கோ"
    "Asia/Thimphu" "திம்பு"
    "Europe/Minsk" "மின்ஸ்க்"
    "America/Belize" "பெலிஸ்"
    "America/St_Johns" "செயின்ட் ஜான்ஸ்"
    "America/Halifax" "ஹலிஃபேக்ஸ்"
    "America/Glace_Bay" "கிலேஸ் வளைகுடா"
    "America/Moncton" "மாங்டான்"
    "America/Goose_Bay" "கூஸ் பே"
    "America/Toronto" "டொரொன்டோ"
    "America/Iqaluit" "இகாலூயித்"
    "America/Winnipeg" "வின்னிபெக்"
    "America/Resolute" "ரெசலூட்"
    "America/Rankin_Inlet" "ரான்கின் இன்லெட்"
    "America/Regina" "ரெஜினா"
    "America/Swift_Current" "ஸ்விஃப்ட் கரண்ட்"
    "America/Edmonton" "எட்மான்டான்"
    "America/Cambridge_Bay" "கேம்பிரிட்ஜ் வளைகுடா"
    "America/Inuvik" "இனுவிக்"
    "America/Vancouver" "வான்கூவர்"
    "America/Dawson_Creek" "டாவ்சன் கிரீக்"
    "America/Fort_Nelson" "ஃபோர்ட் நெல்சன்"
    "America/Whitehorse" "வொயிட்ஹார்ஸ்"
    "America/Dawson" "டாவ்சன்"
    "Europe/Zurich" "ஜூரிச்"
    "Africa/Abidjan" "அபிட்ஜான்"
    "Pacific/Rarotonga" "ரரோடோங்கா"
    "America/Santiago" "சாண்டியாகோ"
    "America/Coyhaique" "கொயாய்கே"
    "America/Punta_Arenas" "புன்டா அரீனாஸ்"
    "Pacific/Easter" "ஈஸ்டர்"
    "Asia/Shanghai" "ஷாங்காய்"
    "Asia/Urumqi" "உரும்கி"
    "America/Bogota" "போகோடா"
    "America/Costa_Rica" "கோஸ்டா ரிகா"
    "America/Havana" "ஹவானா"
    "Atlantic/Cape_Verde" "கேப் வெர்டே"
    "Asia/Nicosia" "நிகோசியா"
    "Asia/Famagusta" "ஃபாமகுஸ்டா"
    "Europe/Prague" "ப்ராக்"
    "Europe/Berlin" "பெர்லின்"
    "America/Santo_Domingo" "சாண்டோ டோமிங்கோ"
    "Africa/Algiers" "அல்ஜியர்ஸ்"
    "America/Guayaquil" "குவாயகில்"
    "Pacific/Galapagos" "கலபகோஸ்"
    "Europe/Tallinn" "டலின்"
    "Africa/Cairo" "கெய்ரோ"
    "Africa/El_Aaiun" "எல் ஆயுன்"
    "Europe/Madrid" "மேட்ரிட்"
    "Africa/Ceuta" "சியூட்டா"
    "Atlantic/Canary" "கேனரி"
    "Europe/Helsinki" "ஹெல்சிங்கி"
    "Pacific/Fiji" "ஃபிஜி"
    "Atlantic/Stanley" "ஸ்டேன்லி"
    "Pacific/Kosrae" "கோஸ்ரே"
    "Atlantic/Faroe" "ஃபரோ"
    "Europe/Paris" "பாரீஸ்"
    "Europe/London" "லண்டன்"
    "Asia/Tbilisi" "த்பிலிசி"
    "America/Cayenne" "கெய்ன்"
    "Europe/Gibraltar" "ஜிப்ரால்டர்"
    "America/Nuuk" "நூக்"
    "America/Danmarkshavn" "டென்மார்க்ஷாவ்ன்"
    "America/Scoresbysund" "இடோகோர்டோர்மிட்"
    "America/Thule" "துலே"
    "Europe/Athens" "ஏதன்ஸ்"
    "Atlantic/South_Georgia" "தெற்கு ஜார்ஜியா"
    "America/Guatemala" "கவுதமாலா"
    "Pacific/Guam" "குவாம்"
    "Africa/Bissau" "பிஸாவ்"
    "America/Guyana" "கயானா"
    "Asia/Hong_Kong" "ஹாங்காங்"
    "America/Tegucigalpa" "தெகுசிகல்பா"
    "America/Port-au-Prince" "போர்ட்-அவ்-பிரின்ஸ்"
    "Europe/Budapest" "புடாபெஸ்ட்"
    "Asia/Jakarta" "ஜகார்த்தா"
    "Asia/Pontianak" "போன்டியானாக்"
    "Asia/Makassar" "மக்கஸர்"
    "Asia/Jayapura" "ஜெயபூரா"
    "Europe/Dublin" "டப்ளின்"
    "Asia/Jerusalem" "ஜெருசலேம்"
    "Asia/Kolkata" "கொல்கத்தா"
    "Indian/Chagos" "சாகோஸ்"
    "Asia/Baghdad" "பாக்தாத்"
    "Asia/Tehran" "டெஹ்ரான்"
    "Europe/Rome" "ரோம்"
    "America/Jamaica" "ஜமைக்கா"
    "Asia/Amman" "அம்மான்"
    "Asia/Tokyo" "டோக்கியோ"
    "Africa/Nairobi" "நைரோபி"
    "Asia/Bishkek" "பிஷ்கெக்"
    "Pacific/Tarawa" "தராவா"
    "Pacific/Kanton" "கேன்டன்"
    "Pacific/Kiritimati" "கிரிடிமாட்டி"
    "Asia/Pyongyang" "பியாங்யாங்"
    "Asia/Seoul" "சியோல்"
    "Asia/Almaty" "அல்மாதி"
    "Asia/Qyzylorda" "கிஸிலோர்டா"
    "Asia/Qostanay" "கோஸ்டானே"
    "Asia/Aqtobe" "அக்டோப்"
    "Asia/Aqtau" "அக்தவ்"
    "Asia/Atyrau" "அடிரா"
    "Asia/Oral" "ஓரல்"
    "Asia/Beirut" "பெய்ரூட்"
    "Asia/Colombo" "கொழும்பு"
    "Africa/Monrovia" "மான்ரோவியா"
    "Europe/Vilnius" "வில்னியஸ்"
    "Europe/Riga" "ரிகா"
    "Africa/Tripoli" "த்ரிபோலி"
    "Africa/Casablanca" "காஸாபிளான்கா"
    "Europe/Chisinau" "சிசினவ்"
    "Pacific/Kwajalein" "க்வாஜாலீயன்"
    "Asia/Yangon" "ரங்கூன்"
    "Asia/Ulaanbaatar" "உலான்பாட்டர்"
    "Asia/Hovd" "ஹோவ்த்"
    "Asia/Macau" "மகாவு"
    "America/Martinique" "மார்ட்டினிக்"
    "Europe/Malta" "மால்டா"
    "Indian/Mauritius" "மொரிஷியஸ்"
    "Indian/Maldives" "மாலத்தீவுகள்"
    "America/Mexico_City" "மெக்ஸிகோ நகரம்"
    "America/Cancun" "கன்குன்"
    "America/Merida" "மெரிடா"
    "America/Monterrey" "மான்டெர்ரே"
    "America/Matamoros" "மடமோராஸ்"
    "America/Chihuahua" "சுவாவா"
    "America/Ciudad_Juarez" "சியுடாட் வாரஸ்"
    "America/Ojinaga" "ஒஜினகா"
    "America/Mazatlan" "மஸட்லன்"
    "America/Bahia_Banderas" "பஹியா பந்தேராஸ்"
    "America/Hermosillo" "ஹெர்மோசிலோ"
    "America/Tijuana" "டிஜுவானா"
    "Asia/Kuching" "குசிங்"
    "Africa/Maputo" "மபுடோ"
    "Africa/Windhoek" "வைண்ட்ஹோக்"
    "Pacific/Noumea" "நோவுமியா"
    "Pacific/Norfolk" "நார்ஃபோக்"
    "Africa/Lagos" "லாகோஸ்"
    "America/Managua" "மானாகுவா"
    "Asia/Kathmandu" "காத்மாண்டு"
    "Pacific/Nauru" "நவ்ரூ"
    "Pacific/Niue" "நியு"
    "Pacific/Auckland" "ஆக்லாந்து"
    "Pacific/Chatham" "சத்தாம்"
    "America/Panama" "பனாமா"
    "America/Lima" "லிமா"
    "Pacific/Tahiti" "தஹிதி"
    "Pacific/Marquesas" "மார்கியூசாஸ்"
    "Pacific/Gambier" "கேம்பியர்"
    "Pacific/Port_Moresby" "போர்ட் மோர்ஸ்பை"
    "Pacific/Bougainville" "போகெய்ன்வில்லே"
    "Asia/Manila" "மணிலா"
    "Asia/Karachi" "கராச்சி"
    "Europe/Warsaw" "வார்ஸா"
    "America/Miquelon" "மிக்யூலன்"
    "Pacific/Pitcairn" "பிட்கெய்ர்ன்"
    "America/Puerto_Rico" "பியூர்டோ ரிகோ"
    "Asia/Gaza" "காஸா"
    "Asia/Hebron" "ஹெப்ரான்"
    "Europe/Lisbon" "லிஸ்பன்"
    "Atlantic/Madeira" "மடிரா"
    "Atlantic/Azores" "அசோரஸ்"
    "Pacific/Palau" "பாலவ்"
    "America/Asuncion" "அஸன்சியன்"
    "Asia/Qatar" "கத்தார்"
    "Europe/Bucharest" "புசாரெஸ்ட்"
    "Europe/Belgrade" "பெல்கிரேட்"
    "Europe/Kaliningrad" "கலினின்கிராட்"
    "Europe/Moscow" "மாஸ்கோ"
    "Europe/Simferopol" "சிம்ஃபெரோபோல்"
    "Europe/Kirov" "கிரோவ்"
    "Europe/Volgograd" "வோல்கோகிராட்"
    "Europe/Astrakhan" "அஸ்ட்ராகான்"
    "Europe/Saratov" "சரடோவ்"
    "Europe/Ulyanovsk" "உல்யானோஸ்க்"
    "Europe/Samara" "சமாரா"
    "Asia/Yekaterinburg" "யெகாடிரின்பர்க்"
    "Asia/Omsk" "ஓம்ஸ்க்"
    "Asia/Novosibirsk" "நோவோசீபிர்ஸ்க்"
    "Asia/Barnaul" "பார்னால்"
    "Asia/Tomsk" "டாம்ஸ்க்"
    "Asia/Novokuznetsk" "நோவோகுஸ்நெட்ஸ்க்"
    "Asia/Krasnoyarsk" "கிராஸ்னோயார்க்ஸ்"
    "Asia/Irkutsk" "இர்குட்ஸ்க்"
    "Asia/Chita" "சிடா"
    "Asia/Yakutsk" "யகுட்ஸ்க்"
    "Asia/Khandyga" "கான்டிகா"
    "Asia/Vladivostok" "விளாடிவொஸ்தோக்"
    "Asia/Ust-Nera" "உஸ்ட்-நேரா"
    "Asia/Magadan" "மகதன்"
    "Asia/Sakhalin" "சகலின்"
    "Asia/Srednekolymsk" "ஸ்ரெட்நிகோலிம்ஸ்க்"
    "Asia/Kamchatka" "காம்சட்கா"
    "Asia/Anadyr" "அனடீர்"
    "Asia/Riyadh" "ரியாத்"
    "Pacific/Guadalcanal" "க்வாடால்கேனல்"
    "Africa/Khartoum" "கார்டோம்"
    "Asia/Singapore" "சிங்கப்பூர்"
    "America/Paramaribo" "பரமரிபோ"
    "Africa/Juba" "ஜுபா"
    "Africa/Sao_Tome" "சாவோ டோமே"
    "America/El_Salvador" "எல் சால்வடோர்"
    "Asia/Damascus" "டமாஸ்கஸ்"
    "America/Grand_Turk" "கிராண்ட் டர்க்"
    "Africa/Ndjamena" "ஜமேனா"
    "Asia/Bangkok" "பாங்காக்"
    "Asia/Dushanbe" "துஷன்பே"
    "Pacific/Fakaofo" "ஃபகாஃபோ"
    "Asia/Dili" "டிலி"
    "Asia/Ashgabat" "அஷ்காபாத்"
    "Africa/Tunis" "டுனிஸ்"
    "Pacific/Tongatapu" "டோன்கடப்பு"
    "Europe/Istanbul" "இஸ்தான்புல்"
    "Asia/Taipei" "தாய்பே"
    "Europe/Kyiv" "கீவ்"
    "America/New_York" "நியூயார்க்"
    "America/Detroit" "டெட்ராய்ட்"
    "America/Kentucky/Louisville" "லூயிஸ்வில்லே"
    "America/Kentucky/Monticello" "மான்டிசெல்லோ, கென்டகி"
    "America/Indiana/Indianapolis" "இண்டியானாபொலிஸ்"
    "America/Indiana/Vincennes" "வின்செனேஸ், இண்டியானா"
    "America/Indiana/Winamac" "வினாமேக், இண்டியானா"
    "America/Indiana/Marengo" "மரென்கோ, இண்டியானா"
    "America/Indiana/Petersburg" "பீட்டர்ஸ்பெர்க், இண்டியானா"
    "America/Indiana/Vevay" "வேவே, இண்டியானா"
    "America/Chicago" "சிகாகோ"
    "America/Indiana/Tell_City" "டெல் சிட்டி, இண்டியானா"
    "America/Indiana/Knox" "நாக்ஸ், இண்டியானா"
    "America/Menominee" "மெனோமினி"
    "America/North_Dakota/Center" "சென்டர், வடக்கு டகோடா"
    "America/North_Dakota/New_Salem" "நியூ சலேம், வடக்கு டகோடா"
    "America/North_Dakota/Beulah" "பெவுலா, வடக்கு டகோட்டா"
    "America/Denver" "டென்வர்"
    "America/Boise" "போய்ஸ்"
    "America/Phoenix" "ஃபோனிக்ஸ்"
    "America/Los_Angeles" "லாஸ் ஏஞ்சல்ஸ்"
    "America/Anchorage" "அங்கோரேஜ்"
    "America/Juneau" "ஜுனியூ"
    "America/Sitka" "சிட்கா"
    "America/Metlakatla" "மெட்லகட்லா"
    "America/Yakutat" "யகுடட்"
    "America/Nome" "நோம்"
    "America/Adak" "அடக்"
    "Pacific/Honolulu" "ஹோனோலூலூ"
    "America/Montevideo" "மான்டேவீடியோ"
    "Asia/Samarkand" "சமார்கண்ட்"
    "Asia/Tashkent" "தாஷ்கண்ட்"
    "America/Caracas" "கரகாஸ்"
    "Asia/Ho_Chi_Minh" "ஹோ சி மின் சிட்டி"
    "Pacific/Efate" "ஈஃபேட்"
    "Pacific/Apia" "அபியா"
    "Africa/Johannesburg" "ஜோஹன்னஸ்பெர்க்"
    "America/Antigua" "ஆன்டிகுவா"
    "America/Anguilla" "அங்குயுலா"
    "Africa/Luanda" "லுவான்டா"
    "Antarctica/McMurdo" "மெக்மர்டோ"
    "Antarctica/DumontDUrville" "டுமோண்ட்-டி உர்வில்லே"
    "Antarctica/Syowa" "ஸ்யோவா"
    "America/Aruba" "அரூபா"
    "Europe/Mariehamn" "மரிஹம்"
    "Europe/Sarajevo" "சரயேவோ"
    "Africa/Ouagadougou" "அவுகடவ்கு"
    "Asia/Bahrain" "பஹ்ரைன்"
    "Africa/Bujumbura" "புஜும்புரா"
    "Africa/Porto-Novo" "போர்ட்டோ-நோவோ"
    "America/St_Barthelemy" "செயின்ட் பார்தேலெமி"
    "Asia/Brunei" "புருனே"
    "America/Kralendijk" "கிரெலன்டிஜ்"
    "America/Nassau" "நசவ்"
    "Africa/Gaborone" "கபோரோன்"
    "America/Blanc-Sablon" "ப்லாங்க்-சப்லான்"
    "America/Atikokan" "அடிகோகன்"
    "America/Creston" "க்ரெஸ்டான்"
    "Indian/Cocos" "கோகோஸ்"
    "Africa/Kinshasa" "கின்ஷசா"
    "Africa/Lubumbashi" "லுபும்பாஷி"
    "Africa/Bangui" "பாங்குயீ"
    "Africa/Brazzaville" "பிராஸாவில்லி"
    "Africa/Douala" "தவுலா"
    "America/Curacao" "க்யூராகோ"
    "Indian/Christmas" "கிறிஸ்துமஸ்"
    "Europe/Busingen" "பசிங்ஜென்"
    "Africa/Djibouti" "ஜிபௌட்டி"
    "Europe/Copenhagen" "கோபன்ஹேகன்"
    "America/Dominica" "டொமினிகா"
    "Africa/Asmara" "அஸ்மாரா"
    "Africa/Addis_Ababa" "அடிஸ் அபாபா"
    "Pacific/Chuuk" "சுக்"
    "Pacific/Pohnpei" "ஃபோன்பெய்"
    "Africa/Libreville" "லிப்ரேவில்லே"
    "America/Grenada" "கிரனடா"
    "Europe/Guernsey" "கர்னஸே"
    "Africa/Accra" "அக்ரா"
    "Africa/Banjul" "பஞ்சுல்"
    "Africa/Conakry" "கோனக்ரே"
    "America/Guadeloupe" "கவுடேலூப்"
    "Africa/Malabo" "மாலபோ"
    "Europe/Zagreb" "ஸக்ரெப்"
    "Europe/Isle_of_Man" "ஐல் ஆஃப் மேன்"
    "Atlantic/Reykjavik" "ரேக்ஜாவிக்"
    "Europe/Jersey" "ஜெர்சி"
    "Asia/Phnom_Penh" "ஃப்னோம் பென்"
    "Indian/Comoro" "கொமரோ"
    "America/St_Kitts" "செயின்ட் கீட்ஸ்"
    "Asia/Kuwait" "குவைத்"
    "America/Cayman" "கேமன்"
    "Asia/Vientiane" "வியன்டியன்"
    "America/St_Lucia" "செயின்ட் லூசியா"
    "Europe/Vaduz" "வதுஸ்"
    "Africa/Maseru" "மசேரு"
    "Europe/Luxembourg" "லக்சம்பர்க்"
    "Europe/Monaco" "மொனாக்கோ"
    "Europe/Podgorica" "போட்கோரிகா"
    "America/Marigot" "மாரிகாட்"
    "Indian/Antananarivo" "ஆண்டனநரிவோ"
    "Pacific/Majuro" "மஜுரோ"
    "Europe/Skopje" "ஸ்கோப்ஜே"
    "Africa/Bamako" "பமாகோ"
    "Pacific/Saipan" "சைபன்"
    "Africa/Nouakchott" "நோவாக்சோட்"
    "America/Montserrat" "மான்செரேட்"
    "Africa/Blantyre" "பிளான்டையர்"
    "Asia/Kuala_Lumpur" "கோலாலம்பூர்"
    "Africa/Niamey" "நியாமே"
    "Europe/Amsterdam" "ஆம்ஸ்ட்ரடாம்"
    "Europe/Oslo" "ஓஸ்லோ"
    "Asia/Muscat" "மஸ்கட்"
    "Indian/Reunion" "ரீயூனியன்"
    "Africa/Kigali" "கிகலி"
    "Indian/Mahe" "மாஹே"
    "Europe/Stockholm" "ஸ்டாக்ஹோம்"
    "Atlantic/St_Helena" "செயின்ட் ஹெலெனா"
    "Europe/Ljubljana" "ஜுப்லானா"
    "Arctic/Longyearbyen" "லாங்இயர்பியன்"
    "Europe/Bratislava" "பிரடிஸ்லாவா"
    "Africa/Freetown" "ஃப்ரீடவுன்"
    "Europe/San_Marino" "சான் மரினோ"
    "Africa/Dakar" "டாகர்"
    "Africa/Mogadishu" "மொகாதிஷு"
    "America/Lower_Princes" "லோயர் பிரின்ஸஸ் குவார்ட்டர்"
    "Africa/Mbabane" "பபான்"
    "Indian/Kerguelen" "கெர்யூலென்"
    "Africa/Lome" "லோம்"
    "America/Port_of_Spain" "போர்ட் ஆஃப் ஸ்பெயின்"
    "Pacific/Funafuti" "ஃபுனாஃபுடி"
    "Africa/Dar_es_Salaam" "தார் எஸ் சலாம்"
    "Africa/Kampala" "கம்பாலா"
    "Pacific/Midway" "மிட்வே"
    "Pacific/Wake" "வேக்"
    "Europe/Vatican" "வாடிகன்"
    "America/St_Vincent" "செயின்ட் வின்சென்ட்"
    "America/Tortola" "டோர்டோலா"
    "America/St_Thomas" "செயின்ட் தாமஸ்"
    "Pacific/Wallis" "வாலிஸ்"
    "Asia/Aden" "ஏடன்"
    "Indian/Mayotte" "மயோட்டி"
    "Africa/Lusaka" "லுசாகா"
    "Africa/Harare" "ஹராரே"
};

// `common/main/th.xml`: 418 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const TH: &str = exemplar_cities! {
    "Europe/Andorra" "อันดอร์รา"
    "Asia/Dubai" "ดูไบ"
    "Asia/Kabul" "คาบูล"
    "Europe/Tirane" "ติรานา"
    "Asia/Yerevan" "เยเรวาน"
    "Antarctica/Casey" "เคซีย์"
    "Antarctica/Davis" "เดวิส"
    "Antarctica/Mawson" "มอว์สัน"
    "Antarctica/Palmer" "พาล์เมอร์"
    "Antarctica/Rothera" "โรธีรา"
    "Antarctica/Troll" "โทรล"
    "Antarctica/Vostok" "วอสตอค"
    "America/Argentina/Buenos_Aires" "บัวโนสไอเรส"
    "America/Argentina/Cordoba" "คอร์โดบา"
    "America/Argentina/Salta" "ซัลตา"
    "America/Argentina/Jujuy" "จูจิว"
    "America/Argentina/Tucuman" "ทูคูแมน"
    "America/Argentina/Catamarca" "กาตามาร์กา"
    "America/Argentina/La_Rioja" "ลาริโอจา"
    "America/Argentina/San_Juan" "ซานฮวน"
    "America/Argentina/Mendoza" "เมนดูซา"
    "America/Argentina/San_Luis" "ซันลูอิส"
    "America/Argentina/Rio_Gallegos" "ริโอกาลเลกอส"
    "America/Argentina/Ushuaia" "อูชูเอีย"
    "Pacific/Pago_Pago" "ปาโก ปาโก"
    "Europe/Vienna" "เวียนนา"
    "Australia/Lord_Howe" "ลอร์ดโฮว์"
    "Antarctica/Macquarie" "แมคควอรี"
    "Australia/Hobart" "โฮบาร์ต"
    "Australia/Melbourne" "เมลเบิร์น"
    "Australia/Sydney" "ซิดนีย์"
    "Australia/Broken_Hill" "โบรกเคนฮิลล์"
    "Australia/Brisbane" "บริสเบน"
    "Australia/Lindeman" "ลินดีแมน"
    "Australia/Adelaide" "แอดิเลด"
    "Australia/Darwin" "ดาร์วิน"
    "Australia/Perth" "เพิร์ท"
    "Australia/Eucla" "ยูคลา"
    "Asia/Baku" "บากู"
    "America/Barbados" "บาร์เบโดส"
    "Asia/Dhaka" "ดากา"
    "Europe/Brussels" "บรัสเซลส์"
    "Europe/Sofia" "โซเฟีย"
    "Atlantic/Bermuda" "เบอร์มิวดา"
    "America/La_Paz" "ลาปาซ"
    "America/Noronha" "โนรอนฮา"
    "America/Belem" "เบเลง"
    "America/Fortaleza" "ฟอร์ตาเลซา"
    "America/Recife" "เรซีเฟ"
    "America/Araguaina" "อารากัวนา"
    "America/Maceio" "มาเซโอ"
    "America/Bahia" "บาเยีย"
    "America/Sao_Paulo" "เซาเปาลู"
    "America/Campo_Grande" "กัมปูกรันดี"
    "America/Cuiaba" "กุยาบา"
    "America/Santarem" "ซันตาเรม"
    "America/Porto_Velho" "ปอร์ตูเวลโย"
    "America/Boa_Vista" "บัววีชตา"
    "America/Manaus" "มาเนาส์"
    "America/Eirunepe" "เอรูเนเป"
    "America/Rio_Branco" "รีโอบรังโก"
    "Asia/Thimphu" "ทิมพู"
    "Europe/Minsk" "มินสก์"
    "America/Belize" "เบลีซ"
    "America/St_Johns" "เซนต์จอนส์"
    "America/Halifax" "แฮลิแฟกซ์"
    "America/Glace_Bay" "เกลซเบย์"
    "America/Moncton" "มองตัน"
    "America/Goose_Bay" "กูสเบย์"
    "America/Toronto" "โทรอนโต"
    "America/Iqaluit" "อีกวาลิต"
    "America/Winnipeg" "วินนิเพก"
    "America/Resolute" "เรโซลูท"
    "America/Rankin_Inlet" "แรงกินอินเล็ต"
    "America/Regina" "ริไจนา"
    "America/Swift_Current" "สวิฟต์เคอร์เรนต์"
    "America/Edmonton" "เอดมันตัน"
    "America/Cambridge_Bay" "อ่าวแคมบริดจ์"
    "America/Inuvik" "อินูวิก"
    "America/Vancouver" "แวนคูเวอร์"
    "America/Dawson_Creek" "ดอว์สัน ครีก"
    "America/Fort_Nelson" "ฟอร์ตเนลสัน"
    "America/Whitehorse" "ไวต์ฮอร์ส"
    "America/Dawson" "ดอว์สัน"
    "Europe/Zurich" "ซูริค"
    "Africa/Abidjan" "อาบีจาน"
    "Pacific/Rarotonga" "ราโรตองกา"
    "America/Santiago" "ซันติอาโก"
    "America/Coyhaique" "โกไยเก"
    "America/Punta_Arenas" "ปุนตาอาเรนัส"
    "Pacific/Easter" "อีสเตอร์"
    "Asia/Shanghai" "เซี่ยงไฮ้"
    "Asia/Urumqi" "อุรุมชี"
    "America/Bogota" "โบโกตา"
    "America/Costa_Rica" "คอสตาริกา"
    "America/Havana" "ฮาวานา"
    "Atlantic/Cape_Verde" "เคปเวิร์ด"
    "Asia/Nicosia" "นิโคเซีย"
    "Asia/Famagusta" "แฟมากุสตา"
    "Europe/Prague" "ปราก"
    "Europe/Berlin" "เบอร์ลิน"
    "America/Santo_Domingo" "ซานโต โดมิงโก"
    "Africa/Algiers" "แอลเจียร์"
    "America/Guayaquil" "กัวยากิล"
    "Pacific/Galapagos" "กาลาปาโกส"
    "Europe/Tallinn" "ทาลลินน์"
    "Africa/Cairo" "ไคโร"
    "Africa/El_Aaiun" "เอลไอย์อุง"
    "Europe/Madrid" "มาดริด"
    "Africa/Ceuta" "เซวตา"
    "Atlantic/Canary" "คะเนรี"
    "Europe/Helsinki" "เฮลซิงกิ"
    "Pacific/Fiji" "ฟิจิ"
    "Atlantic/Stanley" "สแตนลีย์"
    "Pacific/Kosrae" "คอสไร"
    "Atlantic/Faroe" "แฟโร"
    "Europe/Paris" "ปารีส"
    "Europe/London" "ลอนดอน"
    "Asia/Tbilisi" "ทบิลิซิ"
    "America/Cayenne" "กาแยน"
    "Europe/Gibraltar" "ยิบรอลตาร์"
    "America/Nuuk" "กอดแธบ"
    "America/Danmarkshavn" "ดานมาร์กสฮาวน์"
    "America/Scoresbysund" "สกอเรสไบซันด์"
    "America/Thule" "ทูเล"
    "Europe/Athens" "เอเธนส์"
    "Atlantic/South_Georgia" "เซาท์ จอร์เจีย"
    "America/Guatemala" "กัวเตมาลา"
    "Pacific/Guam" "กวม"
    "Africa/Bissau" "บิสเซา"
    "America/Guyana" "กายอานา"
    "Asia/Hong_Kong" "ฮ่องกง"
    "America/Tegucigalpa" "เตกูซิกัลปา"
    "America/Port-au-Prince" "ปอร์โตแปรงซ์"
    "Europe/Budapest" "บูดาเปส"
    "Asia/Jakarta" "จาการ์ตา"
    "Asia/Pontianak" "พอนเทียนัก"
    "Asia/Makassar" "มากัสซาร์"
    "Asia/Jayapura" "จายาปุระ"
    "Europe/Dublin" "ดับบลิน"
    "Asia/Jerusalem" "เยรูซาเลม"
    "Asia/Kolkata" "โกลกาตา"
    "Indian/Chagos" "ชากัส"
    "Asia/Baghdad" "แบกแดด"
    "Asia/Tehran" "เตหะราน"
    "Europe/Rome" "โรม"
    "America/Jamaica" "จาเมกา"
    "Asia/Amman" "อัมมาน"
    "Asia/Tokyo" "โตเกียว"
    "Africa/Nairobi" "ไนโรเบีย"
    "Asia/Bishkek" "บิชเคก"
    "Pacific/Tarawa" "ตาระวา"
    "Pacific/Kanton" "แคนทอน"
    "Pacific/Kiritimati" "คิริทิมาตี"
    "Asia/Pyongyang" "เปียงยาง"
    "Asia/Seoul" "โซล"
    "Asia/Almaty" "อัลมาตี"
    "Asia/Qyzylorda" "ไคซีลอร์ดา"
    "Asia/Qostanay" "คอสตาเนย์"
    "Asia/Aqtobe" "อัคโทบี"
    "Asia/Aqtau" "อัคตาอู"
    "Asia/Atyrau" "อทีราว"
    "Asia/Oral" "ออรัล"
    "Asia/Beirut" "เบรุต"
    "Asia/Colombo" "โคลัมโบ"
    "Africa/Monrovia" "มันโรเวีย"
    "Europe/Vilnius" "วิลนีอุส"
    "Europe/Riga" "ริกา"
    "Africa/Tripoli" "ตรีโปลี"
    "Africa/Casablanca" "คาสซาบลางก้า"
    "Europe/Chisinau" "คีชีเนา"
    "Pacific/Kwajalein" "ควาจาเลน"
    "Asia/Yangon" "ย่างกุ้ง"
    "Asia/Ulaanbaatar" "อูลานบาตอร์"
    "Asia/Hovd" "ฮอฟด์"
    "Asia/Macau" "มาเก๊า"
    "America/Martinique" "มาร์ตินีก"
    "Europe/Malta" "มอลตา"
    "Indian/Mauritius" "มอริเชียส"
    "Indian/Maldives" "มัลดีฟส์"
    "America/Mexico_City" "เม็กซิโกซิตี"
    "America/Cancun" "แคนคุน"
    "America/Merida" "เมรีดา"
    "America/Monterrey" "มอนเตร์เรย์"
    "America/Matamoros" "มาตาโมรอส"
    "America/Chihuahua" "ชีวาวา"
    "America/Ciudad_Juarez" "ซิวดัดฮัวเรซ"
    "America/Ojinaga" "โอจินากา"
    "America/Mazatlan" "มาซาทลาน"
    "America/Bahia_Banderas" "บาเอียบันเดรัส"
    "America/Hermosillo" "เอร์โมซีโย"
    "America/Tijuana" "ทิฮัวนา"
    "Asia/Kuching" "กูชิง"
    "Africa/Maputo" "มาปูโต"
    "Africa/Windhoek" "วินด์ฮุก"
    "Pacific/Noumea" "นูเมอา"
    "Pacific/Norfolk" "นอร์ฟอล์ก"
    "Africa/Lagos" "ลากอส"
    "America/Managua" "มานากัว"
    "Asia/Kathmandu" "กาตมันดุ"
    "Pacific/Nauru" "นาอูรู"
    "Pacific/Niue" "นีอูเอ"
    "Pacific/Auckland" "โอคแลนด์"
    "Pacific/Chatham" "แชทัม"
    "America/Panama" "ปานามา"
    "America/Lima" "ลิมา"
    "Pacific/Tahiti" "ตาฮีตี"
    "Pacific/Marquesas" "มาร์เคซัส"
    "Pacific/Gambier" "แกมเบียร์"
    "Pacific/Port_Moresby" "พอร์ตมอร์สบี"
    "Pacific/Bougainville" "บูเกนวิลล์"
    "Asia/Manila" "มะนิลา"
    "Asia/Karachi" "การาจี"
    "Europe/Warsaw" "วอร์ซอ"
    "America/Miquelon" "มีเกอลง"
    "Pacific/Pitcairn" "พิตแคร์น"
    "America/Puerto_Rico" "เปอโตริโก"
    "Asia/Gaza" "กาซา"
    "Asia/Hebron" "เฮบรอน"
    "Europe/Lisbon" "ลิสบอน"
    "Atlantic/Madeira" "มาเดรา"
    "Atlantic/Azores" "อะโซร์ส"
    "Pacific/Palau" "ปาเลา"
    "America/Asuncion" "อะซุนซิออง"
    "Asia/Qatar" "กาตาร์"
    "Europe/Bucharest" "บูคาเรส"
    "Europe/Belgrade" "เบลเกรด"
    "Europe/Kaliningrad" "คาลินิงกราด"
    "Europe/Moscow" "มอสโก"
    "Europe/Simferopol" "ซิมเฟอโรโปล"
    "Europe/Kirov" "คิรอฟ"
    "Europe/Volgograd" "วอลโกกราด"
    "Europe/Astrakhan" "แอสตราคาน"
    "Europe/Saratov" "ซาราทอฟ"
    "Europe/Ulyanovsk" "อะลิยานอฟ"
    "Europe/Samara" "ซามารา"
    "Asia/Yekaterinburg" "ยีคาเตอรินเบิร์ก"
    "Asia/Omsk" "โอมสก์"
    "Asia/Novosibirsk" "โนโวซิบิร์สก์"
    "Asia/Barnaul" "บาร์เนาว์"
    "Asia/Tomsk" "ตอมสค์"
    "Asia/Novokuznetsk" "โนโวคุซเนตสค์"
    "Asia/Krasnoyarsk" "ครัสโนยาร์สก์"
    "Asia/Irkutsk" "อีร์คุตสค์"
    "Asia/Chita" "ชิตา"
    "Asia/Yakutsk" "ยาคุตสค์"
    "Asia/Khandyga" "ฮันดืยกา"
    "Asia/Vladivostok" "วลาดิโวสต็อก"
    "Asia/Ust-Nera" "อุสต์เนรา"
    "Asia/Magadan" "มากาดาน"
    "Asia/Sakhalin" "ซาคาลิน"
    "Asia/Srednekolymsk" "ซเรดเนคโคลิมสก์"
    "Asia/Kamchatka" "คามชัตกา"
    "Asia/Anadyr" "อานาดีร์"
    "Asia/Riyadh" "ริยาร์ด"
    "Pacific/Guadalcanal" "กัวดัลคานัล"
    "Africa/Khartoum" "คาร์ทูม"
    "Asia/Singapore" "สิงคโปร์"
    "America/Paramaribo" "ปารามาริโบ"
    "Africa/Juba" "จูบา"
    "Africa/Sao_Tome" "เซาตูเม"
    "America/El_Salvador" "เอลซัลวาดอร์"
    "Asia/Damascus" "ดามัสกัส"
    "America/Grand_Turk" "แกรนด์เติร์ก"
    "Africa/Ndjamena" "เอ็นจาเมนา"
    "Asia/Bangkok" "กรุงเทพ"
    "Asia/Dushanbe" "ดูชานเบ"
    "Pacific/Fakaofo" "ฟาเคาโฟ"
    "Asia/Dili" "ดิลี"
    "Asia/Ashgabat" "อาชกาบัต"
    "Africa/Tunis" "ตูนิส"
    "Pacific/Tongatapu" "ตองกาตาปู"
    "Europe/Istanbul" "อิสตันบูล"
    "Asia/Taipei" "ไทเป"
    "Europe/Kyiv" "เคียฟ"
    "America/New_York" "นิวยอร์ก"
    "America/Detroit" "ดีทรอยต์"
    "America/Kentucky/Louisville" "ลูส์วิลล์"
    "America/Kentucky/Monticello" "มอนติเซลโล, เคนตักกี"
    "America/Indiana/Indianapolis" "อินเดียแนโพลิส"
    "America/Indiana/Vincennes" "วินเซนเนส, อินดีแอนา"
    "America/Indiana/Winamac" "วินาแมค, อินดีแอนา"
    "America/Indiana/Marengo" "มาเรงโก, อินดีแอนา"
    "America/Indiana/Petersburg" "ปีเตอร์สเบิร์ก, อินดีแอนา"
    "America/Indiana/Vevay" "วีเวย์, อินดีแอนา"
    "America/Chicago" "ชิคาโก"
    "America/Indiana/Tell_City" "เทลล์ซิตี, อินดีแอนา"
    "America/Indiana/Knox" "นอกซ์, อินดีแอนา"
    "America/Menominee" "เมโนมินี"
    "America/North_Dakota/Center" "เซนเตอร์, นอร์ทดาโคตา"
    "America/North_Dakota/New_Salem" "นิวเซเลม, นอร์ทดาโคตา"
    "America/North_Dakota/Beulah" "โบลาห์, นอร์ทดาโคตา"
    "America/Denver" "เดนเวอร์"
    "America/Boise" "บอยซี"
    "America/Phoenix" "ฟินิกซ์"
    "America/Los_Angeles" "ลอสแองเจลิส"
    "America/Anchorage" "แองเคอเรจ"
    "America/Juneau" "จูโน"
    "America/Sitka" "ซิตกา"
    "America/Metlakatla" "เมทลากาตละ"
    "America/Yakutat" "ยากูทัต"
    "America/Nome" "นอม"
    "America/Adak" "เอดัก"
    "Pacific/Honolulu" "โฮโนลูลู"
    "America/Montevideo" "มอนเตวิเดโอ"
    "Asia/Samarkand" "ซามาร์กานด์"
    "Asia/Tashkent" "ทาชเคนต์"
    "America/Caracas" "คาราคัส"
    "Asia/Ho_Chi_Minh" "นครโฮจิมินห์"
    "Pacific/Efate" "เอฟาเต"
    "Pacific/Apia" "อาปีอา"
    "Africa/Johannesburg" "โจฮันเนสเบอร์ก"
    "America/Antigua" "แอนติกา"
    "America/Anguilla" "แองกิลลา"
    "Africa/Luanda" "ลูอันดา"
    "Antarctica/McMurdo" "แมคมัวโด"
    "Antarctica/DumontDUrville" "ดูมองต์ดูร์วิลล์"
    "Antarctica/Syowa" "ไซโยวา"
    "America/Aruba" "อารูบา"
    "Europe/Mariehamn" "มารีฮามน์"
    "Europe/Sarajevo" "ซาราเยโว"
    "Africa/Ouagadougou" "วากาดูกู"
    "Asia/Bahrain" "บาห์เรน"
    "Africa/Bujumbura" "บูจุมบูรา"
    "Africa/Porto-Novo" "ปอร์โต-โนโว"
    "America/St_Barthelemy" "เซนต์บาร์เธเลมี"
    "Asia/Brunei" "บรูไน"
    "America/Kralendijk" "คราเลนดิจค์"
    "America/Nassau" "แนสซอ"
    "Africa/Gaborone" "กาโบโรเน"
    "America/Blanc-Sablon" "บลังค์-ซาบลอน"
    "America/Atikokan" "คอรัลฮาร์เบอร์"
    "America/Creston" "เครสตัน"
    "Indian/Cocos" "โคโคส"
    "Africa/Kinshasa" "กินชาซา"
    "Africa/Lubumbashi" "ลูบัมบาชิ"
    "Africa/Bangui" "บังกี"
    "Africa/Brazzaville" "บราซซาวิล"
    "Africa/Douala" "ดูอาลา"
    "America/Curacao" "คูราเซา"
    "Indian/Christmas" "คริสต์มาส"
    "Europe/Busingen" "บุสซิงเง็น"
    "Africa/Djibouti" "จิบูตี"
    "Europe/Copenhagen" "โคเปนเฮเกน"
    "America/Dominica" "โดมินิกา"
    "Africa/Asmara" "แอสมารา"
    "Africa/Addis_Ababa" "แอดดิสอาบาบา"
    "Pacific/Chuuk" "ทรัก"
    "Pacific/Pohnpei" "โปนาเป"
    "Africa/Libreville" "ลีเบรอวิล"
    "America/Grenada" "เกรนาดา"
    "Europe/Guernsey" "เกิร์นซีย์"
    "Africa/Accra" "อักกรา"
    "Africa/Banjul" "บันจูล"
    "Africa/Conakry" "โกนากรี"
    "America/Guadeloupe" "กวาเดอลูป"
    "Africa/Malabo" "มาลาโบ"
    "Europe/Zagreb" "ซาเกร็บ"
    "Europe/Isle_of_Man" "เกาะแมน"
    "Atlantic/Reykjavik" "เรคยาวิก"
    "Europe/Jersey" "เจอร์ซีย์"
    "Asia/Phnom_Penh" "พนมเปญ"
    "Indian/Comoro" "โคโมโร"
    "America/St_Kitts" "เซนต์คิตส์"
    "Asia/Kuwait" "คูเวต"
    "America/Cayman" "เคย์แมน"
    "Asia/Vientiane" "เวียงจันทน์"
    "America/St_Lucia" "เซนต์ลูเซีย"
    "Europe/Vaduz" "วาดุซ"
    "Africa/Maseru" "มาเซรู"
    "Europe/Luxembourg" "ลักเซมเบิร์ก"
    "Europe/Monaco" "โมนาโก"
    "Europe/Podgorica" "พอดกอรีตซา"
    "America/Marigot" "มาริโกต์"
    "Indian/Antananarivo" "อันตานานาริโว"
    "Pacific/Majuro" "มาจูโร"
    "Europe/Skopje" "สโกเปีย"
    "Africa/Bamako" "บามาโก"
    "Pacific/Saipan" "ไซปัน"
    "Africa/Nouakchott" "นูแอกชอต"
    "America/Montserrat" "มอนเซอร์รัต"
    "Africa/Blantyre" "แบลนไทร์"
    "Asia/Kuala_Lumpur" "กัวลาลัมเปอร์"
    "Africa/Niamey" "นีอาเมย์"
    "Europe/Amsterdam" "อัมสเตอดัม"
    "Europe/Oslo" "ออสโล"
    "Asia/Muscat" "มัสกัต"
    "Indian/Reunion" "เรอูนียง"
    "Africa/Kigali" "คิกาลี"
    "Indian/Mahe" "มาเอ"
    "Europe/Stockholm" "สตอกโฮล์ม"
    "Atlantic/St_Helena" "เซนต์เฮเลนา"
    "Europe/Ljubljana" "ลูบลิยานา"
    "Arctic/Longyearbyen" "ลองเยียร์เบียน"
    "Europe/Bratislava" "บราติสลาวา"
    "Africa/Freetown" "ฟรีทาวน์"
    "Europe/San_Marino" "ซานมารีโน"
    "Africa/Dakar" "ดาการ์"
    "Africa/Mogadishu" "โมกาดิชู"
    "America/Lower_Princes" "โลเวอร์พรินซ์ ควอเตอร์"
    "Africa/Mbabane" "อัมบาบาเน"
    "Indian/Kerguelen" "แกร์เกอลอง"
    "Africa/Lome" "โลเม"
    "America/Port_of_Spain" "พอร์ทออฟสเปน"
    "Pacific/Funafuti" "ฟูนะฟูตี"
    "Africa/Dar_es_Salaam" "ดาร์เอสซาลาม"
    "Africa/Kampala" "คัมพาลา"
    "Pacific/Midway" "มิดเวย์"
    "Pacific/Wake" "เวก"
    "Europe/Vatican" "วาติกัน"
    "America/St_Vincent" "เซนต์วินเซนต์"
    "America/Tortola" "ตอร์โตลา"
    "America/St_Thomas" "เซนต์โธมัส"
    "Pacific/Wallis" "วาลลิส"
    "Asia/Aden" "เอเดน"
    "Indian/Mayotte" "มาโยเต"
    "Africa/Lusaka" "ลูซากา"
    "Africa/Harare" "ฮาราเร"
};

// `common/main/tr.xml`: 116 of the 418 zones named, 302 inherited.
#[cfg(feature = "localized-exemplar-cities")]
const TR: &str = exemplar_cities! {
    "Europe/Andorra" inherited
    "Asia/Dubai" inherited
    "Asia/Kabul" "Kabil"
    "Europe/Tirane" "Tiran"
    "Asia/Yerevan" "Erivan"
    "Antarctica/Casey" inherited
    "Antarctica/Davis" inherited
    "Antarctica/Mawson" inherited
    "Antarctica/Palmer" inherited
    "Antarctica/Rothera" inherited
    "Antarctica/Troll" inherited
    "Antarctica/Vostok" inherited
    "America/Argentina/Buenos_Aires" inherited
    "America/Argentina/Cordoba" inherited
    "America/Argentina/Salta" inherited
    "America/Argentina/Jujuy" inherited
    "America/Argentina/Tucuman" inherited
    "America/Argentina/Catamarca" inherited
    "America/Argentina/La_Rioja" inherited
    "America/Argentina/San_Juan" inherited
    "America/Argentina/Mendoza" inherited
    "America/Argentina/San_Luis" inherited
    "America/Argentina/Rio_Gallegos" inherited
    "America/Argentina/Ushuaia" inherited
    "Pacific/Pago_Pago" inherited
    "Europe/Vienna" "Viyana"
    "Australia/Lord_Howe" inherited
    "Antarctica/Macquarie" inherited
    "Australia/Hobart" inherited
    "Australia/Melbourne" inherited
    "Australia/Sydney" "Sidney"
    "Australia/Broken_Hill" inherited
    "Australia/Brisbane" inherited
    "Australia/Lindeman" inherited
    "Australia/Adelaide" inherited
    "Australia/Darwin" inherited
    "Australia/Perth" inherited
    "Australia/Eucla" inherited
    "Asia/Baku" "Bakü"
    "America/Barbados" inherited
    "Asia/Dhaka" "Dakka"
    "Europe/Brussels" "Brüksel"
    "Europe/Sofia" "Sofya"
    "Atlantic/Bermuda" inherited
    "America/La_Paz" inherited
    "America/Noronha" inherited
    "America/Belem" inherited
    "America/Fortaleza" inherited
    "America/Recife" inherited
    "America/Araguaina" inherited
    "America/Maceio" inherited
    "America/Bahia" inherited
    "America/Sao_Paulo" inherited
    "America/Campo_Grande" inherited
    "America/Cuiaba" inherited
    "America/Santarem" inherited
    "America/Porto_Velho" inherited
    "America/Boa_Vista" inherited
    "America/Manaus" inherited
    "America/Eirunepe" inherited
    "America/Rio_Branco" inherited
    "Asia/Thimphu" inherited
    "Europe/Minsk" inherited
    "America/Belize" inherited
    "America/St_Johns" inherited
    "America/Halifax" inherited
    "America/Glace_Bay" inherited
    "America/Moncton" inherited
    "America/Goose_Bay" inherited
    "America/Toronto" inherited
    "America/Iqaluit" inherited
    "America/Winnipeg" inherited
    "America/Resolute" inherited
    "America/Rankin_Inlet" inherited
    "America/Regina" inherited
    "America/Swift_Current" inherited
    "America/Edmonton" inherited
    "America/Cambridge_Bay" inherited
    "America/Inuvik" inherited
    "America/Vancouver" inherited
    "America/Dawson_Creek" inherited
    "America/Fort_Nelson" inherited
    "America/Whitehorse" inherited
    "America/Dawson" inherited
    "Europe/Zurich" "Zürih"
    "Africa/Abidjan" inherited
    "Pacific/Rarotonga" inherited
    "America/Santiago" inherited
    "America/Coyhaique" inherited
    "America/Punta_Arenas" inherited
    "Pacific/Easter" "Paskalya Adası"
    "Asia/Shanghai" "Şanghay"
    "Asia/Urumqi" "Urumçi"
    "America/Bogota" inherited
    "America/Costa_Rica" "Kosta Rika"
    "America/Havana" inherited
    "Atlantic/Cape_Verde" inherited
    "Asia/Nicosia" "Lefkoşa"
    "Asia/Famagusta" "Gazimağusa"
    "Europe/Prague" "Prag"
    "Europe/Berlin" inherited
    "America/Santo_Domingo" inherited
    "Africa/Algiers" "Cezayir"
    "America/Guayaquil" inherited
    "Pacific/Galapagos" inherited
    "Europe/Tallinn" inherited
    "Africa/Cairo" "Kahire"
    "Africa/El_Aaiun" "Layun"
    "Europe/Madrid" inherited
    "Africa/Ceuta" "Septe"
    "Atlantic/Canary" "Kanarya Adaları"
    "Europe/Helsinki" inherited
    "Pacific/Fiji" inherited
    "Atlantic/Stanley" inherited
    "Pacific/Kosrae" inherited
    "Atlantic/Faroe" inherited
    "Europe/Paris" inherited
    "Europe/London" "Londra"
    "Asia/Tbilisi" "Tiflis"
    "America/Cayenne" inherited
    "Europe/Gibraltar" "Cebelitarık"
    "America/Nuuk" inherited
    "America/Danmarkshavn" inherited
    "America/Scoresbysund" inherited
    "America/Thule" inherited
    "Europe/Athens" "Atina"
    "Atlantic/South_Georgia" "Güney Georgia"
    "America/Guatemala" inherited
    "Pacific/Guam" inherited
    "Africa/Bissau" inherited
    "America/Guyana" inherited
    "Asia/Hong_Kong" inherited
    "America/Tegucigalpa" inherited
    "America/Port-au-Prince" inherited
    "Europe/Budapest" "Budapeşte"
    "Asia/Jakarta" "Cakarta"
    "Asia/Pontianak" inherited
    "Asia/Makassar" inherited
    "Asia/Jayapura" inherited
    "Europe/Dublin" inherited
    "Asia/Jerusalem" "Kudüs"
    "Asia/Kolkata" "Kalküta"
    "Indian/Chagos" inherited
    "Asia/Baghdad" "Bağdat"
    "Asia/Tehran" "Tahran"
    "Europe/Rome" "Roma"
    "America/Jamaica" "Jamaika"
    "Asia/Amman" inherited
    "Asia/Tokyo" inherited
    "Africa/Nairobi" inherited
    "Asia/Bishkek" "Bişkek"
    "Pacific/Tarawa" inherited
    "Pacific/Kanton" "Canton Adası"
    "Pacific/Kiritimati" inherited
    "Asia/Pyongyang" inherited
    "Asia/Seoul" "Seul"
    "Asia/Almaty" "Almatı"
    "Asia/Qyzylorda" "Kızılorda"
    "Asia/Qostanay" "Kostanay"
    "Asia/Aqtobe" "Aktöbe"
    "Asia/Aqtau" "Aktav"
    "Asia/Atyrau" "Atırav"
    "Asia/Oral" inherited
    "Asia/Beirut" "Beyrut"
    "Asia/Colombo" "Kolombo"
    "Africa/Monrovia" inherited
    "Europe/Vilnius" inherited
    "Europe/Riga" inherited
    "Africa/Tripoli" "Trablus"
    "Africa/Casablanca" "Kazablanka"
    "Europe/Chisinau" "Kişinev"
    "Pacific/Kwajalein" inherited
    "Asia/Yangon" inherited
    "Asia/Ulaanbaatar" "Ulan Batur"
    "Asia/Hovd" inherited
    "Asia/Macau" "Makao"
    "America/Martinique" inherited
    "Europe/Malta" inherited
    "Indian/Mauritius" inherited
    "Indian/Maldives" "Maldivler"
    "America/Mexico_City" inherited
    "America/Cancun" "Cancun"
    "America/Merida" "Merida"
    "America/Monterrey" inherited
    "America/Matamoros" inherited
    "America/Chihuahua" inherited
    "America/Ciudad_Juarez" inherited
    "America/Ojinaga" inherited
    "America/Mazatlan" inherited
    "America/Bahia_Banderas" "Bahia Banderas"
    "America/Hermosillo" inherited
    "America/Tijuana" inherited
    "Asia/Kuching" "Kuçing"
    "Africa/Maputo" inherited
    "Africa/Windhoek" inherited
    "Pacific/Noumea" inherited
    "Pacific/Norfolk" inherited
    "Africa/Lagos" inherited
    "America/Managua" inherited
    "Asia/Kathmandu" "Katmandu"
    "Pacific/Nauru" inherited
    "Pacific/Niue" inherited
    "Pacific/Auckland" inherited
    "Pacific/Chatham" inherited
    "America/Panama" inherited
    "America/Lima" inherited
    "Pacific/Tahiti" inherited
    "Pacific/Marquesas" "Markiz Adaları"
    "Pacific/Gambier" inherited
    "Pacific/Port_Moresby" inherited
    "Pacific/Bougainville" inherited
    "Asia/Manila" inherited
    "Asia/Karachi" "Karaçi"
    "Europe/Warsaw" "Varşova"
    "America/Miquelon" inherited
    "Pacific/Pitcairn" inherited
    "America/Puerto_Rico" "Porto Riko"
    "Asia/Gaza" "Gazze"
    "Asia/Hebron" "El Halil"
    "Europe/Lisbon" "Lizbon"
    "Atlantic/Madeira" "Madeira Adaları"
    "Atlantic/Azores" "Azor Adaları"
    "Pacific/Palau" inherited
    "America/Asuncion" inherited
    "Asia/Qatar" "Katar"
    "Europe/Bucharest" "Bükreş"
    "Europe/Belgrade" "Belgrad"
    "Europe/Kaliningrad" inherited
    "Europe/Moscow" "Moskova"
    "Europe/Simferopol" inherited
    "Europe/Kirov" inherited
    "Europe/Volgograd" inherited
    "Europe/Astrakhan" "Astrahan"
    "Europe/Saratov" inherited
    "Europe/Ulyanovsk" inherited
    "Europe/Samara" inherited
    "Asia/Yekaterinburg" inherited
    "Asia/Omsk" inherited
    "Asia/Novosibirsk" inherited
    "Asia/Barnaul" inherited
    "Asia/Tomsk" inherited
    "Asia/Novokuznetsk" inherited
    "Asia/Krasnoyarsk" inherited
    "Asia/Irkutsk" "İrkutsk"
    "Asia/Chita" "Çita"
    "Asia/Yakutsk" inherited
    "Asia/Khandyga" "Handiga"
    "Asia/Vladivostok" inherited
    "Asia/Ust-Nera" inherited
    "Asia/Magadan" inherited
    "Asia/Sakhalin" "Sahalin"
    "Asia/Srednekolymsk" inherited
    "Asia/Kamchatka" "Kamçatka"
    "Asia/Anadyr" "Anadır"
    "Asia/Riyadh" "Riyad"
    "Pacific/Guadalcanal" inherited
    "Africa/Khartoum" "Hartum"
    "Asia/Singapore" "Singapur"
    "America/Paramaribo" inherited
    "Africa/Juba" "Cuba"
    "Africa/Sao_Tome" inherited
    "America/El_Salvador" inherited
    "Asia/Damascus" "Şam"
    "America/Grand_Turk" inherited
    "Africa/Ndjamena" inherited
    "Asia/Bangkok" inherited
    "Asia/Dushanbe" "Duşanbe"
    "Pacific/Fakaofo" inherited
    "Asia/Dili" inherited
    "Asia/Ashgabat" "Aşkabat"
    "Africa/Tunis" "Tunus"
    "Pacific/Tongatapu" inherited
    "Europe/Istanbul" "İstanbul"
    "Asia/Taipei" inherited
    "Europe/Kyiv" "Kiev"
    "America/New_York" inherited
    "America/Detroit" inherited
    "America/Kentucky/Louisville" inherited
    "America/Kentucky/Monticello" inherited
    "America/Indiana/Indianapolis" inherited
    "America/Indiana/Vincennes" inherited
    "America/Indiana/Winamac" inherited
    "America/Indiana/Marengo" inherited
    "America/Indiana/Petersburg" inherited
    "America/Indiana/Vevay" inherited
    "America/Chicago" inherited
    "America/Indiana/Tell_City" inherited
    "America/Indiana/Knox" inherited
    "America/Menominee" inherited
    "America/North_Dakota/Center" "Merkez, Kuzey Dakota"
    "America/North_Dakota/New_Salem" "New Salem, Kuzey Dakota"
    "America/North_Dakota/Beulah" "Beulah, Kuzey Dakota"
    "America/Denver" inherited
    "America/Boise" inherited
    "America/Phoenix" inherited
    "America/Los_Angeles" inherited
    "America/Anchorage" inherited
    "America/Juneau" inherited
    "America/Sitka" inherited
    "America/Metlakatla" inherited
    "America/Yakutat" inherited
    "America/Nome" inherited
    "America/Adak" inherited
    "Pacific/Honolulu" "Honolulu"
    "America/Montevideo" inherited
    "Asia/Samarkand" "Semerkand"
    "Asia/Tashkent" "Taşkent"
    "America/Caracas" inherited
    "Asia/Ho_Chi_Minh" "Ho Chi Minh Kenti"
    "Pacific/Efate" inherited
    "Pacific/Apia" inherited
    "Africa/Johannesburg" inherited
    "America/Antigua" inherited
    "America/Anguilla" inherited
    "Africa/Luanda" inherited
    "Antarctica/McMurdo" inherited
    "Antarctica/DumontDUrville" inherited
    "Antarctica/Syowa" inherited
    "America/Aruba" inherited
    "Europe/Mariehamn" inherited
    "Europe/Sarajevo" "Saraybosna"
    "Africa/Ouagadougou" inherited
    "Asia/Bahrain" "Bahreyn"
    "Africa/Bujumbura" inherited
    "Africa/Porto-Novo" inherited
    "America/St_Barthelemy" "Saint Barthelemy"
    "Asia/Brunei" inherited
    "America/Kralendijk" inherited
    "America/Nassau" inherited
    "Africa/Gaborone" inherited
    "America/Blanc-Sablon" inherited
    "America/Atikokan" inherited
    "America/Creston" inherited
    "Indian/Cocos" inherited
    "Africa/Kinshasa" "Kinşasa"
    "Africa/Lubumbashi" inherited
    "Africa/Bangui" inherited
    "Africa/Brazzaville" "Brazzavil"
    "Africa/Douala" inherited
    "America/Curacao" inherited
    "Indian/Christmas" inherited
    "Europe/Busingen" inherited
    "Africa/Djibouti" "Cibuti"
    "Europe/Copenhagen" "Kopenhag"
    "America/Dominica" "Dominika"
    "Africa/Asmara" inherited
    "Africa/Addis_Ababa" inherited
    "Pacific/Chuuk" inherited
    "Pacific/Pohnpei" inherited
    "Africa/Libreville" "Librevil"
    "America/Grenada" inherited
    "Europe/Guernsey" inherited
    "Africa/Accra" "Akra"
    "Africa/Banjul" inherited
    "Africa/Conakry" "Konakri"
    "America/Guadeloupe" inherited
    "Africa/Malabo" inherited
    "Europe/Zagreb" inherited
    "Europe/Isle_of_Man" "Man Adası"
    "Atlantic/Reykjavik" inherited
    "Europe/Jersey" inherited
    "Asia/Phnom_Penh" inherited
    "Indian/Comoro" "Komor"
    "America/St_Kitts" inherited
    "Asia/Kuwait" "Kuveyt"
    "America/Cayman" inherited
    "Asia/Vientiane" inherited
    "America/St_Lucia" inherited
    "Europe/Vaduz" inherited
    "Africa/Maseru" inherited
    "Europe/Luxembourg" "Lüksemburg"
    "Europe/Monaco" "Monako"
    "Europe/Podgorica" inherited
    "America/Marigot" inherited
    "Indian/Antananarivo" inherited
    "Pacific/Majuro" inherited
    "Europe/Skopje" "Üsküp"
    "Africa/Bamako" inherited
    "Pacific/Saipan" inherited
    "Africa/Nouakchott" inherited
    "America/Montserrat" inherited
    "Africa/Blantyre" inherited
    "Asia/Kuala_Lumpur" inherited
    "Africa/Niamey" inherited
    "Europe/Amsterdam" inherited
    "Europe/Oslo" inherited
    "Asia/Muscat" "Maskat"
    "Indian/Reunion" inherited
    "Africa/Kigali" inherited
    "Indian/Mahe" inherited
    "Europe/Stockholm" "Stokholm"
    "Atlantic/St_Helena" inherited
    "Europe/Ljubljana" inherited
    "Arctic/Longyearbyen" inherited
    "Europe/Bratislava" inherited
    "Africa/Freetown" inherited
    "Europe/San_Marino" inherited
    "Africa/Dakar" inherited
    "Africa/Mogadishu" "Mogadişu"
    "America/Lower_Princes" inherited
    "Africa/Mbabane" inherited
    "Indian/Kerguelen" inherited
    "Africa/Lome" inherited
    "America/Port_of_Spain" inherited
    "Pacific/Funafuti" inherited
    "Africa/Dar_es_Salaam" "Darüsselam"
    "Africa/Kampala" inherited
    "Pacific/Midway" inherited
    "Pacific/Wake" inherited
    "Europe/Vatican" "Vatikan"
    "America/St_Vincent" inherited
    "America/Tortola" inherited
    "America/St_Thomas" inherited
    "Pacific/Wallis" inherited
    "Asia/Aden" inherited
    "Indian/Mayotte" inherited
    "Africa/Lusaka" inherited
    "Africa/Harare" inherited
};

// `common/main/vi.xml`: 23 of the 418 zones named, 395 inherited.
#[cfg(feature = "localized-exemplar-cities")]
const VI: &str = exemplar_cities! {
    "Europe/Andorra" inherited
    "Asia/Dubai" inherited
    "Asia/Kabul" inherited
    "Europe/Tirane" inherited
    "Asia/Yerevan" inherited
    "Antarctica/Casey" inherited
    "Antarctica/Davis" inherited
    "Antarctica/Mawson" inherited
    "Antarctica/Palmer" inherited
    "Antarctica/Rothera" inherited
    "Antarctica/Troll" inherited
    "Antarctica/Vostok" inherited
    "America/Argentina/Buenos_Aires" inherited
    "America/Argentina/Cordoba" inherited
    "America/Argentina/Salta" inherited
    "America/Argentina/Jujuy" inherited
    "America/Argentina/Tucuman" inherited
    "America/Argentina/Catamarca" inherited
    "America/Argentina/La_Rioja" inherited
    "America/Argentina/San_Juan" inherited
    "America/Argentina/Mendoza" inherited
    "America/Argentina/San_Luis" inherited
    "America/Argentina/Rio_Gallegos" inherited
    "America/Argentina/Ushuaia" inherited
    "Pacific/Pago_Pago" inherited
    "Europe/Vienna" inherited
    "Australia/Lord_Howe" inherited
    "Antarctica/Macquarie" inherited
    "Australia/Hobart" inherited
    "Australia/Melbourne" inherited
    "Australia/Sydney" inherited
    "Australia/Broken_Hill" inherited
    "Australia/Brisbane" inherited
    "Australia/Lindeman" inherited
    "Australia/Adelaide" inherited
    "Australia/Darwin" inherited
    "Australia/Perth" inherited
    "Australia/Eucla" inherited
    "Asia/Baku" inherited
    "America/Barbados" inherited
    "Asia/Dhaka" inherited
    "Europe/Brussels" inherited
    "Europe/Sofia" inherited
    "Atlantic/Bermuda" inherited
    "America/La_Paz" inherited
    "America/Noronha" inherited
    "America/Belem" inherited
    "America/Fortaleza" inherited
    "America/Recife" inherited
    "America/Araguaina" inherited
    "America/Maceio" inherited
    "America/Bahia" inherited
    "America/Sao_Paulo" inherited
    "America/Campo_Grande" inherited
    "America/Cuiaba" inherited
    "America/Santarem" inherited
    "America/Porto_Velho" inherited
    "America/Boa_Vista" inherited
    "America/Manaus" inherited
    "America/Eirunepe" inherited
    "America/Rio_Branco" inherited
    "Asia/Thimphu" inherited
    "Europe/Minsk" inherited
    "America/Belize" inherited
    "America/St_Johns" inherited
    "America/Halifax" inherited
    "America/Glace_Bay" inherited
    "America/Moncton" inherited
    "America/Goose_Bay" inherited
    "America/Toronto" inherited
    "America/Iqaluit" inherited
    "America/Winnipeg" inherited
    "America/Resolute" inherited
    "America/Rankin_Inlet" inherited
    "America/Regina" inherited
    "America/Swift_Current" inherited
    "America/Edmonton" inherited
    "America/Cambridge_Bay" inherited
    "America/Inuvik" inherited
    "America/Vancouver" inherited
    "America/Dawson_Creek" inherited
    "America/Fort_Nelson" inherited
    "America/Whitehorse" inherited
    "America/Dawson" inherited
    "Europe/Zurich" inherited
    "Africa/Abidjan" inherited
    "Pacific/Rarotonga" inherited
    "America/Santiago" inherited
    "America/Coyhaique" inherited
    "America/Punta_Arenas" inherited
    "Pacific/Easter" inherited
    "Asia/Shanghai" "Thượng Hải"
    "Asia/Urumqi" inherited
    "America/Bogota" inherited
    "America/Costa_Rica" inherited
    "America/Havana" inherited
    "Atlantic/Cape_Verde" inherited
    "Asia/Nicosia" inherited
    "Asia/Famagusta" inherited
    "Europe/Prague" "Praha"
    "Europe/Berlin" inherited
    "America/Santo_Domingo" inherited
    "Africa/Algiers" inherited
    "America/Guayaquil" inherited
    "Pacific/Galapagos" inherited
    "Europe/Tallinn" inherited
    "Africa/Cairo" inherited
    "Africa/El_Aaiun" inherited
    "Europe/Madrid" inherited
    "Africa/Ceuta" inherited
    "Atlantic/Canary" inherited
    "Europe/Helsinki" inherited
    "Pacific/Fiji" inherited
    "Atlantic/Stanley" inherited
    "Pacific/Kosrae" inherited
    "Atlantic/Faroe" inherited
    "Europe/Paris" inherited
    "Europe/London" inherited
    "Asia/Tbilisi" inherited
    "America/Cayenne" inherited
    "Europe/Gibraltar" inherited
    "America/Nuuk" inherited
    "America/Danmarkshavn" inherited
    "America/Scoresbysund" inherited
    "America/Thule" inherited
    "Europe/Athens" inherited
    "Atlantic/South_Georgia" "Nam Georgia"
    "America/Guatemala" inherited
    "Pacific/Guam" inherited
    "Africa/Bissau" inherited
    "America/Guyana" inherited
    "Asia/Hong_Kong" "Hồng Kông"
    "America/Tegucigalpa" inherited
    "America/Port-au-Prince" inherited
    "Europe/Budapest" inherited
    "Asia/Jakarta" inherited
    "Asia/Pontianak" inherited
    "Asia/Makassar" inherited
    "Asia/Jayapura" inherited
    "Europe/Dublin" inherited
    "Asia/Jerusalem" inherited
    "Asia/Kolkata" inherited
    "Indian/Chagos" inherited
    "Asia/Baghdad" inherited
    "Asia/Tehran" inherited
    "Europe/Rome" inherited
    "America/Jamaica" inherited
    "Asia/Amman" inherited
    "Asia/Tokyo" inherited
    "Africa/Nairobi" inherited
    "Asia/Bishkek" inherited
    "Pacific/Tarawa" inherited
    "Pacific/Kanton" "Đảo Canton"
    "Pacific/Kiritimati" inherited
    "Asia/Pyongyang" "Bình Nhưỡng"
    "Asia/Seoul" inherited
    "Asia/Almaty" inherited
    "Asia/Qyzylorda" inherited
    "Asia/Qostanay" "Kostanay"
    "Asia/Aqtobe" inherited
    "Asia/Aqtau" inherited
    "Asia/Atyrau" inherited
    "Asia/Oral" inherited
    "Asia/Beirut" inherited
    "Asia/Colombo" inherited
    "Africa/Monrovia" inherited
    "Europe/Vilnius" inherited
    "Europe/Riga" inherited
    "Africa/Tripoli" inherited
    "Africa/Casablanca" inherited
    "Europe/Chisinau" inherited
    "Pacific/Kwajalein" inherited
    "Asia/Yangon" "Rangoon"
    "Asia/Ulaanbaatar" "Ulan Bator"
    "Asia/Hovd" inherited
    "Asia/Macau" "Ma Cao"
    "America/Martinique" inherited
    "Europe/Malta" inherited
    "Indian/Mauritius" inherited
    "Indian/Maldives" inherited
    "America/Mexico_City" inherited
    "America/Cancun" "Cancun"
    "America/Merida" "Merida"
    "America/Monterrey" inherited
    "America/Matamoros" inherited
    "America/Chihuahua" inherited
    "America/Ciudad_Juarez" inherited
    "America/Ojinaga" inherited
    "America/Mazatlan" inherited
    "America/Bahia_Banderas" "Bahia Banderas"
    "America/Hermosillo" inherited
    "America/Tijuana" inherited
    "Asia/Kuching" inherited
    "Africa/Maputo" inherited
    "Africa/Windhoek" inherited
    "Pacific/Noumea" inherited
    "Pacific/Norfolk" inherited
    "Africa/Lagos" inherited
    "America/Managua" inherited
    "Asia/Kathmandu" inherited
    "Pacific/Nauru" inherited
    "Pacific/Niue" inherited
    "Pacific/Auckland" inherited
    "Pacific/Chatham" inherited
    "America/Panama" inherited
    "America/Lima" inherited
    "Pacific/Tahiti" inherited
    "Pacific/Marquesas" inherited
    "Pacific/Gambier" inherited
    "Pacific/Port_Moresby" inherited
    "Pacific/Bougainville" inherited
    "Asia/Manila" inherited
    "Asia/Karachi" inherited
    "Europe/Warsaw" inherited
    "America/Miquelon" inherited
    "Pacific/Pitcairn" inherited
    "America/Puerto_Rico" inherited
    "Asia/Gaza" inherited
    "Asia/Hebron" inherited
    "Europe/Lisbon" inherited
    "Atlantic/Madeira" inherited
    "Atlantic/Azores" inherited
    "Pacific/Palau" inherited
    "America/Asuncion" inherited
    "Asia/Qatar" inherited
    "Europe/Bucharest" inherited
    "Europe/Belgrade" inherited
    "Europe/Kaliningrad" inherited
    "Europe/Moscow" "Mát-xcơ-va"
    "Europe/Simferopol" inherited
    "Europe/Kirov" inherited
    "Europe/Volgograd" inherited
    "Europe/Astrakhan" inherited
    "Europe/Saratov" inherited
    "Europe/Ulyanovsk" inherited
    "Europe/Samara" inherited
    "Asia/Yekaterinburg" inherited
    "Asia/Omsk" inherited
    "Asia/Novosibirsk" inherited
    "Asia/Barnaul" inherited
    "Asia/Tomsk" inherited
    "Asia/Novokuznetsk" inherited
    "Asia/Krasnoyarsk" inherited
    "Asia/Irkutsk" inherited
    "Asia/Chita" inherited
    "Asia/Yakutsk" inherited
    "Asia/Khandyga" inherited
    "Asia/Vladivostok" inherited
    "Asia/Ust-Nera" inherited
    "Asia/Magadan" inherited
    "Asia/Sakhalin" inherited
    "Asia/Srednekolymsk" inherited
    "Asia/Kamchatka" inherited
    "Asia/Anadyr" inherited
    "Asia/Riyadh" inherited
    "Pacific/Guadalcanal" inherited
    "Africa/Khartoum" inherited
    "Asia/Singapore" inherited
    "America/Paramaribo" inherited
    "Africa/Juba" inherited
    "Africa/Sao_Tome" inherited
    "America/El_Salvador" inherited
    "Asia/Damascus" inherited
    "America/Grand_Turk" inherited
    "Africa/Ndjamena" inherited
    "Asia/Bangkok" inherited
    "Asia/Dushanbe" inherited
    "Pacific/Fakaofo" inherited
    "Asia/Dili" inherited
    "Asia/Ashgabat" inherited
    "Africa/Tunis" inherited
    "Pacific/Tongatapu" inherited
    "Europe/Istanbul" inherited
    "Asia/Taipei" "Đài Bắc"
    "Europe/Kyiv" "Kiev"
    "America/New_York" inherited
    "America/Detroit" inherited
    "America/Kentucky/Louisville" inherited
    "America/Kentucky/Monticello" inherited
    "America/Indiana/Indianapolis" inherited
    "America/Indiana/Vincennes" inherited
    "America/Indiana/Winamac" inherited
    "America/Indiana/Marengo" inherited
    "America/Indiana/Petersburg" inherited
    "America/Indiana/Vevay" inherited
    "America/Chicago" inherited
    "America/Indiana/Tell_City" inherited
    "America/Indiana/Knox" inherited
    "America/Menominee" inherited
    "America/North_Dakota/Center" "Center, Bắc Dakota"
    "America/North_Dakota/New_Salem" "New Salem, Bắc Dakota"
    "America/North_Dakota/Beulah" "Beulah, Bắc Dakota"
    "America/Denver" inherited
    "America/Boise" inherited
    "America/Phoenix" inherited
    "America/Los_Angeles" inherited
    "America/Anchorage" inherited
    "America/Juneau" inherited
    "America/Sitka" inherited
    "America/Metlakatla" inherited
    "America/Yakutat" inherited
    "America/Nome" inherited
    "America/Adak" inherited
    "Pacific/Honolulu" "Honolulu"
    "America/Montevideo" inherited
    "Asia/Samarkand" inherited
    "Asia/Tashkent" inherited
    "America/Caracas" inherited
    "Asia/Ho_Chi_Minh" "TP Hồ Chí Minh"
    "Pacific/Efate" inherited
    "Pacific/Apia" inherited
    "Africa/Johannesburg" inherited
    "America/Antigua" inherited
    "America/Anguilla" inherited
    "Africa/Luanda" inherited
    "Antarctica/McMurdo" inherited
    "Antarctica/DumontDUrville" inherited
    "Antarctica/Syowa" inherited
    "America/Aruba" inherited
    "Europe/Mariehamn" inherited
    "Europe/Sarajevo" inherited
    "Africa/Ouagadougou" inherited
    "Asia/Bahrain" inherited
    "Africa/Bujumbura" inherited
    "Africa/Porto-Novo" inherited
    "America/St_Barthelemy" inherited
    "Asia/Brunei" inherited
    "America/Kralendijk" inherited
    "America/Nassau" inherited
    "Africa/Gaborone" inherited
    "America/Blanc-Sablon" inherited
    "America/Atikokan" inherited
    "America/Creston" inherited
    "Indian/Cocos" inherited
    "Africa/Kinshasa" inherited
    "Africa/Lubumbashi" inherited
    "Africa/Bangui" inherited
    "Africa/Brazzaville" inherited
    "Africa/Douala" inherited
    "America/Curacao" inherited
    "Indian/Christmas" inherited
    "Europe/Busingen" inherited
    "Africa/Djibouti" inherited
    "Europe/Copenhagen" inherited
    "America/Dominica" inherited
    "Africa/Asmara" inherited
    "Africa/Addis_Ababa" inherited
    "Pacific/Chuuk" inherited
    "Pacific/Pohnpei" inherited
    "Africa/Libreville" inherited
    "America/Grenada" inherited
    "Europe/Guernsey" inherited
    "Africa/Accra" inherited
    "Africa/Banjul" inherited
    "Africa/Conakry" inherited
    "America/Guadeloupe" inherited
    "Africa/Malabo" inherited
    "Europe/Zagreb" inherited
    "Europe/Isle_of_Man" "Đảo Man"
    "Atlantic/Reykjavik" inherited
    "Europe/Jersey" inherited
    "Asia/Phnom_Penh" inherited
    "Indian/Comoro" inherited
    "America/St_Kitts" inherited
    "Asia/Kuwait" inherited
    "America/Cayman" inherited
    "Asia/Vientiane" "Viêng Chăn"
    "America/St_Lucia" inherited
    "Europe/Vaduz" inherited
    "Africa/Maseru" inherited
    "Europe/Luxembourg" inherited
    "Europe/Monaco" inherited
    "Europe/Podgorica" inherited
    "America/Marigot" inherited
    "Indian/Antananarivo" inherited
    "Pacific/Majuro" inherited
    "Europe/Skopje" inherited
    "Africa/Bamako" inherited
    "Pacific/Saipan" inherited
    "Africa/Nouakchott" inherited
    "America/Montserrat" inherited
    "Africa/Blantyre" inherited
    "Asia/Kuala_Lumpur" inherited
    "Africa/Niamey" inherited
    "Europe/Amsterdam" inherited
    "Europe/Oslo" inherited
    "Asia/Muscat" inherited
    "Indian/Reunion" inherited
    "Africa/Kigali" inherited
    "Indian/Mahe" inherited
    "Europe/Stockholm" inherited
    "Atlantic/St_Helena" inherited
    "Europe/Ljubljana" inherited
    "Arctic/Longyearbyen" inherited
    "Europe/Bratislava" inherited
    "Africa/Freetown" inherited
    "Europe/San_Marino" inherited
    "Africa/Dakar" inherited
    "Africa/Mogadishu" inherited
    "America/Lower_Princes" inherited
    "Africa/Mbabane" inherited
    "Indian/Kerguelen" inherited
    "Africa/Lome" inherited
    "America/Port_of_Spain" inherited
    "Pacific/Funafuti" inherited
    "Africa/Dar_es_Salaam" inherited
    "Africa/Kampala" inherited
    "Pacific/Midway" inherited
    "Pacific/Wake" inherited
    "Europe/Vatican" inherited
    "America/St_Vincent" inherited
    "America/Tortola" inherited
    "America/St_Thomas" inherited
    "Pacific/Wallis" inherited
    "Asia/Aden" inherited
    "Indian/Mayotte" inherited
    "Africa/Lusaka" inherited
    "Africa/Harare" inherited
};

// `common/main/zh.xml`: 418 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const ZH_HANS: &str = exemplar_cities! {
    "Europe/Andorra" "安道尔"
    "Asia/Dubai" "迪拜"
    "Asia/Kabul" "喀布尔"
    "Europe/Tirane" "地拉那"
    "Asia/Yerevan" "埃里温"
    "Antarctica/Casey" "卡塞"
    "Antarctica/Davis" "戴维斯"
    "Antarctica/Mawson" "莫森"
    "Antarctica/Palmer" "帕尔默"
    "Antarctica/Rothera" "罗瑟拉"
    "Antarctica/Troll" "特罗尔"
    "Antarctica/Vostok" "沃斯托克"
    "America/Argentina/Buenos_Aires" "布宜诺斯艾利斯"
    "America/Argentina/Cordoba" "科尔多瓦"
    "America/Argentina/Salta" "萨尔塔"
    "America/Argentina/Jujuy" "胡胡伊"
    "America/Argentina/Tucuman" "图库曼"
    "America/Argentina/Catamarca" "卡塔马卡"
    "America/Argentina/La_Rioja" "拉里奥哈"
    "America/Argentina/San_Juan" "圣胡安"
    "America/Argentina/Mendoza" "门多萨"
    "America/Argentina/San_Luis" "圣路易斯"
    "America/Argentina/Rio_Gallegos" "里奥加耶戈斯"
    "America/Argentina/Ushuaia" "乌斯怀亚"
    "Pacific/Pago_Pago" "帕果帕果"
    "Europe/Vienna" "维也纳"
    "Australia/Lord_Howe" "豪勋爵岛"
    "Antarctica/Macquarie" "麦夸里岛"
    "Australia/Hobart" "霍巴特"
    "Australia/Melbourne" "墨尔本"
    "Australia/Sydney" "悉尼"
    "Australia/Broken_Hill" "布罗肯希尔"
    "Australia/Brisbane" "布里斯班"
    "Australia/Lindeman" "林德曼"
    "Australia/Adelaide" "阿德莱德"
    "Australia/Darwin" "达尔文"
    "Australia/Perth" "珀斯"
    "Australia/Eucla" "尤克拉"
    "Asia/Baku" "巴库"
    "America/Barbados" "巴巴多斯"
    "Asia/Dhaka" "达卡"
    "Europe/Brussels" "布鲁塞尔"
    "Europe/Sofia" "索非亚"
    "Atlantic/Bermuda" "百慕大"
    "America/La_Paz" "拉巴斯"
    "America/Noronha" "洛罗尼亚"
    "America/Belem" "贝伦"
    "America/Fortaleza" "福塔雷萨"
    "America/Recife" "累西腓"
    "America/Araguaina" "阿拉瓜伊纳"
    "America/Maceio" "马塞约"
    "America/Bahia" "巴伊亚"
    "America/Sao_Paulo" "圣保罗"
    "America/Campo_Grande" "大坎普"
    "America/Cuiaba" "库亚巴"
    "America/Santarem" "圣塔伦"
    "America/Porto_Velho" "波多韦柳"
    "America/Boa_Vista" "博阿维斯塔"
    "America/Manaus" "马瑙斯"
    "America/Eirunepe" "依伦尼贝"
    "America/Rio_Branco" "里奥布郎库"
    "Asia/Thimphu" "廷布"
    "Europe/Minsk" "明斯克"
    "America/Belize" "伯利兹"
    "America/St_Johns" "圣约翰斯"
    "America/Halifax" "哈利法克斯"
    "America/Glace_Bay" "格莱斯贝"
    "America/Moncton" "蒙克顿"
    "America/Goose_Bay" "古斯湾"
    "America/Toronto" "多伦多"
    "America/Iqaluit" "伊魁特"
    "America/Winnipeg" "温尼伯"
    "America/Resolute" "雷索卢特"
    "America/Rankin_Inlet" "兰今湾"
    "America/Regina" "里贾纳"
    "America/Swift_Current" "斯威夫特卡伦特"
    "America/Edmonton" "埃德蒙顿"
    "America/Cambridge_Bay" "剑桥湾"
    "America/Inuvik" "伊努维克"
    "America/Vancouver" "温哥华"
    "America/Dawson_Creek" "道森克里克"
    "America/Fort_Nelson" "纳尔逊堡"
    "America/Whitehorse" "怀特霍斯"
    "America/Dawson" "道森"
    "Europe/Zurich" "苏黎世"
    "Africa/Abidjan" "阿比让"
    "Pacific/Rarotonga" "拉罗汤加"
    "America/Santiago" "圣地亚哥"
    "America/Coyhaique" "科伊艾克"
    "America/Punta_Arenas" "蓬塔阿雷纳斯"
    "Pacific/Easter" "复活节岛"
    "Asia/Shanghai" "上海"
    "Asia/Urumqi" "乌鲁木齐"
    "America/Bogota" "波哥大"
    "America/Costa_Rica" "哥斯达黎加"
    "America/Havana" "哈瓦那"
    "Atlantic/Cape_Verde" "佛得角"
    "Asia/Nicosia" "尼科西亚"
    "Asia/Famagusta" "法马古斯塔"
    "Europe/Prague" "布拉格"
    "Europe/Berlin" "柏林"
    "America/Santo_Domingo" "圣多明各"
    "Africa/Algiers" "阿尔及尔"
    "America/Guayaquil" "瓜亚基尔"
    "Pacific/Galapagos" "科隆群岛"
    "Europe/Tallinn" "塔林"
    "Africa/Cairo" "开罗"
    "Africa/El_Aaiun" "阿尤恩"
    "Europe/Madrid" "马德里"
    "Africa/Ceuta" "休达"
    "Atlantic/Canary" "加那利"
    "Europe/Helsinki" "赫尔辛基"
    "Pacific/Fiji" "斐济"
    "Atlantic/Stanley" "斯坦利"
    "Pacific/Kosrae" "库赛埃"
    "Atlantic/Faroe" "法罗"
    "Europe/Paris" "巴黎"
    "Europe/London" "伦敦"
    "Asia/Tbilisi" "第比利斯"
    "America/Cayenne" "卡宴"
    "Europe/Gibraltar" "直布罗陀"
    "America/Nuuk" "努克"
    "America/Danmarkshavn" "丹马沙文"
    "America/Scoresbysund" "斯科列斯比桑德"
    "America/Thule" "图勒"
    "Europe/Athens" "雅典"
    "Atlantic/South_Georgia" "南乔治亚"
    "America/Guatemala" "危地马拉"
    "Pacific/Guam" "关岛"
    "Africa/Bissau" "比绍"
    "America/Guyana" "圭亚那"
    "Asia/Hong_Kong" "香港"
    "America/Tegucigalpa" "特古西加尔巴"
    "America/Port-au-Prince" "太子港"
    "Europe/Budapest" "布达佩斯"
    "Asia/Jakarta" "雅加达"
    "Asia/Pontianak" "坤甸"
    "Asia/Makassar" "望加锡"
    "Asia/Jayapura" "查亚普拉"
    "Europe/Dublin" "都柏林"
    "Asia/Jerusalem" "耶路撒冷"
    "Asia/Kolkata" "加尔各答"
    "Indian/Chagos" "查戈斯"
    "Asia/Baghdad" "巴格达"
    "Asia/Tehran" "德黑兰"
    "Europe/Rome" "罗马"
    "America/Jamaica" "牙买加"
    "Asia/Amman" "安曼"
    "Asia/Tokyo" "东京"
    "Africa/Nairobi" "内罗毕"
    "Asia/Bishkek" "比什凯克"
    "Pacific/Tarawa" "塔拉瓦"
    "Pacific/Kanton" "坎顿岛"
    "Pacific/Kiritimati" "基里地马地岛"
    "Asia/Pyongyang" "平壤"
    "Asia/Seoul" "首尔"
    "Asia/Almaty" "阿拉木图"
    "Asia/Qyzylorda" "克孜洛尔达"
    "Asia/Qostanay" "库斯塔奈"
    "Asia/Aqtobe" "阿克托别"
    "Asia/Aqtau" "阿克套"
    "Asia/Atyrau" "阿特劳"
    "Asia/Oral" "乌拉尔"
    "Asia/Beirut" "贝鲁特"
    "Asia/Colombo" "科伦坡"
    "Africa/Monrovia" "蒙罗维亚"
    "Europe/Vilnius" "维尔纽斯"
    "Europe/Riga" "里加"
    "Africa/Tripoli" "的黎波里"
    "Africa/Casablanca" "卡萨布兰卡"
    "Europe/Chisinau" "基希讷乌"
    "Pacific/Kwajalein" "夸贾林"
    "Asia/Yangon" "仰光"
    "Asia/Ulaanbaatar" "乌兰巴托"
    "Asia/Hovd" "科布多"
    "Asia/Macau" "澳门"
    "America/Martinique" "马提尼克"
    "Europe/Malta" "马耳他"
    "Indian/Mauritius" "毛里求斯"
    "Indian/Maldives" "马尔代夫"
    "America/Mexico_City" "墨西哥城"
    "America/Cancun" "坎昆"
    "America/Merida" "梅里达"
    "America/Monterrey" "蒙特雷"
    "America/Matamoros" "马塔莫罗斯"
    "America/Chihuahua" "奇瓦瓦"
    "America/Ciudad_Juarez" "华雷斯城"
    "America/Ojinaga" "奥希纳加"
    "America/Mazatlan" "马萨特兰"
    "America/Bahia_Banderas" "巴伊亚班德拉斯"
    "America/Hermosillo" "埃莫西约"
    "America/Tijuana" "蒂华纳"
    "Asia/Kuching" "古晋"
    "Africa/Maputo" "马普托"
    "Africa/Windhoek" "温得和克"
    "Pacific/Noumea" "努美阿"
    "Pacific/Norfolk" "诺福克"
    "Africa/Lagos" "拉各斯"
    "America/Managua" "马那瓜"
    "Asia/Kathmandu" "加德满都"
    "Pacific/Nauru" "瑙鲁"
    "Pacific/Niue" "纽埃"
    "Pacific/Auckland" "奥克兰"
    "Pacific/Chatham" "查塔姆"
    "America/Panama" "巴拿马"
    "America/Lima" "利马"
    "Pacific/Tahiti" "塔希提"
    "Pacific/Marquesas" "马克萨斯"
    "Pacific/Gambier" "甘比尔"
    "Pacific/Port_Moresby" "莫尔兹比港"
    "Pacific/Bougainville" "布干维尔"
    "Asia/Manila" "马尼拉"
    "Asia/Karachi" "卡拉奇"
    "Europe/Warsaw" "华沙"
    "America/Miquelon" "密克隆"
    "Pacific/Pitcairn" "皮特凯恩"
    "America/Puerto_Rico" "波多黎各"
    "Asia/Gaza" "加沙"
    "Asia/Hebron" "希伯伦"
    "Europe/Lisbon" "里斯本"
    "Atlantic/Madeira" "马德拉"
    "Atlantic/Azores" "亚速尔群岛"
    "Pacific/Palau" "帕劳"
    "America/Asuncion" "亚松森"
    "Asia/Qatar" "卡塔尔"
    "Europe/Bucharest" "布加勒斯特"
    "Europe/Belgrade" "贝尔格莱德"
    "Europe/Kaliningrad" "加里宁格勒"
    "Europe/Moscow" "莫斯科"
    "Europe/Simferopol" "辛菲罗波尔"
    "Europe/Kirov" "基洛夫"
    "Europe/Volgograd" "伏尔加格勒"
    "Europe/Astrakhan" "阿斯特拉罕"
    "Europe/Saratov" "萨拉托夫"
    "Europe/Ulyanovsk" "乌里扬诺夫斯克"
    "Europe/Samara" "萨马拉"
    "Asia/Yekaterinburg" "叶卡捷琳堡"
    "Asia/Omsk" "鄂木斯克"
    "Asia/Novosibirsk" "新西伯利亚"
    "Asia/Barnaul" "巴尔瑙尔"
    "Asia/Tomsk" "托木斯克"
    "Asia/Novokuznetsk" "新库兹涅茨克"
    "Asia/Krasnoyarsk" "克拉斯诺亚尔斯克"
    "Asia/Irkutsk" "伊尔库茨克"
    "Asia/Chita" "赤塔"
    "Asia/Yakutsk" "雅库茨克"
    "Asia/Khandyga" "汉德加"
    "Asia/Vladivostok" "海参崴"
    "Asia/Ust-Nera" "乌斯内拉"
    "Asia/Magadan" "马加丹"
    "Asia/Sakhalin" "萨哈林"
    "Asia/Srednekolymsk" "中科雷姆斯克"
    "Asia/Kamchatka" "堪察加"
    "Asia/Anadyr" "阿纳德尔"
    "Asia/Riyadh" "利雅得"
    "Pacific/Guadalcanal" "瓜达尔卡纳尔"
    "Africa/Khartoum" "喀土穆"
    "Asia/Singapore" "新加坡"
    "America/Paramaribo" "帕拉马里博"
    "Africa/Juba" "朱巴"
    "Africa/Sao_Tome" "圣多美"
    "America/El_Salvador" "萨尔瓦多"
    "Asia/Damascus" "大马士革"
    "America/Grand_Turk" "大特克"
    "Africa/Ndjamena" "恩贾梅纳"
    "Asia/Bangkok" "曼谷"
    "Asia/Dushanbe" "杜尚别"
    "Pacific/Fakaofo" "法考福"
    "Asia/Dili" "帝力"
    "Asia/Ashgabat" "阿什哈巴德"
    "Africa/Tunis" "突尼斯"
    "Pacific/Tongatapu" "东加塔布"
    "Europe/Istanbul" "伊斯坦布尔"
    "Asia/Taipei" "台北"
    "Europe/Kyiv" "基辅"
    "America/New_York" "纽约"
    "America/Detroit" "底特律"
    "America/Kentucky/Louisville" "路易斯维尔"
    "America/Kentucky/Monticello" "肯塔基州蒙蒂塞洛"
    "America/Indiana/Indianapolis" "印第安纳波利斯"
    "America/Indiana/Vincennes" "印第安纳州温森斯"
    "America/Indiana/Winamac" "印第安纳州威纳马克"
    "America/Indiana/Marengo" "印第安纳州马伦戈"
    "America/Indiana/Petersburg" "印第安纳州彼得斯堡"
    "America/Indiana/Vevay" "印第安纳州维维市"
    "America/Chicago" "芝加哥"
    "America/Indiana/Tell_City" "印第安纳州特尔城"
    "America/Indiana/Knox" "印第安纳州诺克斯"
    "America/Menominee" "梅诺米尼"
    "America/North_Dakota/Center" "北达科他州申特"
    "America/North_Dakota/New_Salem" "北达科他州新塞勒姆"
    "America/North_Dakota/Beulah" "北达科他州比尤拉"
    "America/Denver" "丹佛"
    "America/Boise" "博伊西"
    "America/Phoenix" "凤凰城"
    "America/Los_Angeles" "洛杉矶"
    "America/Anchorage" "安克雷奇"
    "America/Juneau" "朱诺"
    "America/Sitka" "锡特卡"
    "America/Metlakatla" "梅特拉卡特拉"
    "America/Yakutat" "亚库塔特"
    "America/Nome" "诺姆"
    "America/Adak" "埃达克"
    "Pacific/Honolulu" "檀香山"
    "America/Montevideo" "蒙得维的亚"
    "Asia/Samarkand" "撒马尔罕"
    "Asia/Tashkent" "塔什干"
    "America/Caracas" "加拉加斯"
    "Asia/Ho_Chi_Minh" "胡志明市"
    "Pacific/Efate" "埃法特"
    "Pacific/Apia" "阿皮亚"
    "Africa/Johannesburg" "约翰内斯堡"
    "America/Antigua" "安提瓜"
    "America/Anguilla" "安圭拉"
    "Africa/Luanda" "罗安达"
    "Antarctica/McMurdo" "麦克默多"
    "Antarctica/DumontDUrville" "迪蒙·迪维尔"
    "Antarctica/Syowa" "昭和"
    "America/Aruba" "阿鲁巴"
    "Europe/Mariehamn" "玛丽港"
    "Europe/Sarajevo" "萨拉热窝"
    "Africa/Ouagadougou" "瓦加杜古"
    "Asia/Bahrain" "巴林"
    "Africa/Bujumbura" "布琼布拉"
    "Africa/Porto-Novo" "波多诺伏"
    "America/St_Barthelemy" "圣巴泰勒米岛"
    "Asia/Brunei" "文莱"
    "America/Kralendijk" "克拉伦代克"
    "America/Nassau" "拿骚"
    "Africa/Gaborone" "哈博罗内"
    "America/Blanc-Sablon" "布兰克萨布隆"
    "America/Atikokan" "阿蒂科肯"
    "America/Creston" "克雷斯顿"
    "Indian/Cocos" "可可斯"
    "Africa/Kinshasa" "金沙萨"
    "Africa/Lubumbashi" "卢本巴希"
    "Africa/Bangui" "班吉"
    "Africa/Brazzaville" "布拉柴维尔"
    "Africa/Douala" "杜阿拉"
    "America/Curacao" "库拉索"
    "Indian/Christmas" "圣诞岛"
    "Europe/Busingen" "布辛根"
    "Africa/Djibouti" "吉布提"
    "Europe/Copenhagen" "哥本哈根"
    "America/Dominica" "多米尼加"
    "Africa/Asmara" "阿斯马拉"
    "Africa/Addis_Ababa" "亚的斯亚贝巴"
    "Pacific/Chuuk" "特鲁克群岛"
    "Pacific/Pohnpei" "波纳佩岛"
    "Africa/Libreville" "利伯维尔"
    "America/Grenada" "格林纳达"
    "Europe/Guernsey" "根西岛"
    "Africa/Accra" "阿克拉"
    "Africa/Banjul" "班珠尔"
    "Africa/Conakry" "科纳克里"
    "America/Guadeloupe" "瓜德罗普"
    "Africa/Malabo" "马拉博"
    "Europe/Zagreb" "萨格勒布"
    "Europe/Isle_of_Man" "马恩岛"
    "Atlantic/Reykjavik" "雷克雅未克"
    "Europe/Jersey" "泽西岛"
    "Asia/Phnom_Penh" "金边"
    "Indian/Comoro" "科摩罗"
    "America/St_Kitts" "圣基茨"
    "Asia/Kuwait" "科威特"
    "America/Cayman" "开曼"
    "Asia/Vientiane" "万象"
    "America/St_Lucia" "圣卢西亚"
    "Europe/Vaduz" "瓦杜兹"
    "Africa/Maseru" "马塞卢"
    "Europe/Luxembourg" "卢森堡"
    "Europe/Monaco" "摩纳哥"
    "Europe/Podgorica" "波德戈里察"
    "America/Marigot" "马里戈特"
    "Indian/Antananarivo" "安塔那那利佛"
    "Pacific/Majuro" "马朱罗"
    "Europe/Skopje" "斯科普里"
    "Africa/Bamako" "巴马科"
    "Pacific/Saipan" "塞班"
    "Africa/Nouakchott" "努瓦克肖特"
    "America/Montserrat" "蒙特塞拉特"
    "Africa/Blantyre" "布兰太尔"
    "Asia/Kuala_Lumpur" "吉隆坡"
    "Africa/Niamey" "尼亚美"
    "Europe/Amsterdam" "阿姆斯特丹"
    "Europe/Oslo" "奥斯陆"
    "Asia/Muscat" "马斯喀特"
    "Indian/Reunion" "留尼汪"
    "Africa/Kigali" "基加利"
    "Indian/Mahe" "马埃岛"
    "Europe/Stockholm" "斯德哥尔摩"
    "Atlantic/St_Helena" "圣赫勒拿"
    "Europe/Ljubljana" "卢布尔雅那"
    "Arctic/Longyearbyen" "朗伊尔城"
    "Europe/Bratislava" "布拉迪斯拉发"
    "Africa/Freetown" "弗里敦"
    "Europe/San_Marino" "圣马力诺"
    "Africa/Dakar" "达喀尔"
    "Africa/Mogadishu" "摩加迪沙"
    "America/Lower_Princes" "下太子区"
    "Africa/Mbabane" "姆巴巴纳"
    "Indian/Kerguelen" "凯尔盖朗"
    "Africa/Lome" "洛美"
    "America/Port_of_Spain" "西班牙港"
    "Pacific/Funafuti" "富纳富提"
    "Africa/Dar_es_Salaam" "达累斯萨拉姆"
    "Africa/Kampala" "坎帕拉"
    "Pacific/Midway" "中途岛"
    "Pacific/Wake" "威克"
    "Europe/Vatican" "梵蒂冈"
    "America/St_Vincent" "圣文森特"
    "America/Tortola" "托尔托拉"
    "America/St_Thomas" "圣托马斯"
    "Pacific/Wallis" "瓦利斯"
    "Asia/Aden" "亚丁"
    "Indian/Mayotte" "马约特"
    "Africa/Lusaka" "卢萨卡"
    "Africa/Harare" "哈拉雷"
};

// `common/main/zh_Hant.xml`: 418 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const ZH_HANT: &str = exemplar_cities! {
    "Europe/Andorra" "安道爾"
    "Asia/Dubai" "杜拜"
    "Asia/Kabul" "喀布爾"
    "Europe/Tirane" "地拉那"
    "Asia/Yerevan" "葉里溫"
    "Antarctica/Casey" "凱西"
    "Antarctica/Davis" "戴維斯"
    "Antarctica/Mawson" "莫森"
    "Antarctica/Palmer" "帕麥"
    "Antarctica/Rothera" "羅瑟拉"
    "Antarctica/Troll" "綽爾"
    "Antarctica/Vostok" "沃斯托克"
    "America/Argentina/Buenos_Aires" "布宜諾斯艾利斯"
    "America/Argentina/Cordoba" "哥多華"
    "America/Argentina/Salta" "薩爾塔"
    "America/Argentina/Jujuy" "胡胡伊"
    "America/Argentina/Tucuman" "吐庫曼"
    "America/Argentina/Catamarca" "卡塔馬卡"
    "America/Argentina/La_Rioja" "拉略哈"
    "America/Argentina/San_Juan" "聖胡安"
    "America/Argentina/Mendoza" "門多薩"
    "America/Argentina/San_Luis" "聖路易"
    "America/Argentina/Rio_Gallegos" "里奧加耶戈斯"
    "America/Argentina/Ushuaia" "烏斯懷亞"
    "Pacific/Pago_Pago" "巴哥巴哥"
    "Europe/Vienna" "維也納"
    "Australia/Lord_Howe" "豪勳爵島"
    "Antarctica/Macquarie" "麥覺理"
    "Australia/Hobart" "荷巴特"
    "Australia/Melbourne" "墨爾本"
    "Australia/Sydney" "雪梨"
    "Australia/Broken_Hill" "布羅肯希爾"
    "Australia/Brisbane" "布利斯班"
    "Australia/Lindeman" "林德曼"
    "Australia/Adelaide" "阿得雷德"
    "Australia/Darwin" "達爾文"
    "Australia/Perth" "伯斯"
    "Australia/Eucla" "尤克拉"
    "Asia/Baku" "巴庫"
    "America/Barbados" "巴貝多"
    "Asia/Dhaka" "達卡"
    "Europe/Brussels" "布魯塞爾"
    "Europe/Sofia" "索菲亞"
    "Atlantic/Bermuda" "百慕達"
    "America/La_Paz" "拉巴斯"
    "America/Noronha" "諾倫哈"
    "America/Belem" "貝倫"
    "America/Fortaleza" "福塔力莎"
    "America/Recife" "雷西非"
    "America/Araguaina" "阿拉圭那"
    "America/Maceio" "馬瑟歐"
    "America/Bahia" "巴伊阿"
    "America/Sao_Paulo" "聖保羅"
    "America/Campo_Grande" "格蘭場"
    "America/Cuiaba" "古雅巴"
    "America/Santarem" "聖塔倫"
    "America/Porto_Velho" "維留港"
    "America/Boa_Vista" "保維斯塔"
    "America/Manaus" "瑪瑙斯"
    "America/Eirunepe" "艾魯內佩"
    "America/Rio_Branco" "里約布蘭"
    "Asia/Thimphu" "廷布"
    "Europe/Minsk" "明斯克"
    "America/Belize" "貝里斯"
    "America/St_Johns" "聖約翰"
    "America/Halifax" "哈里法克斯"
    "America/Glace_Bay" "格雷斯貝"
    "America/Moncton" "蒙克頓"
    "America/Goose_Bay" "鵝灣"
    "America/Toronto" "多倫多"
    "America/Iqaluit" "伊魁特"
    "America/Winnipeg" "溫尼伯"
    "America/Resolute" "羅斯魯特"
    "America/Rankin_Inlet" "蘭今灣"
    "America/Regina" "里賈納"
    "America/Swift_Current" "斯威夫特卡倫特"
    "America/Edmonton" "艾德蒙吞"
    "America/Cambridge_Bay" "劍橋灣"
    "America/Inuvik" "伊奴維克"
    "America/Vancouver" "溫哥華"
    "America/Dawson_Creek" "道森克里克"
    "America/Fort_Nelson" "納爾遜堡"
    "America/Whitehorse" "懷特霍斯"
    "America/Dawson" "道森"
    "Europe/Zurich" "蘇黎世"
    "Africa/Abidjan" "阿比讓"
    "Pacific/Rarotonga" "拉羅湯加"
    "America/Santiago" "聖地牙哥"
    "America/Coyhaique" "科伊艾克"
    "America/Punta_Arenas" "蓬塔阿雷納斯"
    "Pacific/Easter" "復活島"
    "Asia/Shanghai" "上海"
    "Asia/Urumqi" "烏魯木齊"
    "America/Bogota" "波哥大"
    "America/Costa_Rica" "哥斯大黎加"
    "America/Havana" "哈瓦那"
    "Atlantic/Cape_Verde" "維德角"
    "Asia/Nicosia" "尼古西亞"
    "Asia/Famagusta" "法馬古斯塔"
    "Europe/Prague" "布拉格"
    "Europe/Berlin" "柏林"
    "America/Santo_Domingo" "聖多明哥"
    "Africa/Algiers" "阿爾及爾"
    "America/Guayaquil" "瓜亞基爾"
    "Pacific/Galapagos" "加拉巴哥群島"
    "Europe/Tallinn" "塔林"
    "Africa/Cairo" "開羅"
    "Africa/El_Aaiun" "阿尤恩"
    "Europe/Madrid" "馬德里"
    "Africa/Ceuta" "休達"
    "Atlantic/Canary" "加納利"
    "Europe/Helsinki" "赫爾辛基"
    "Pacific/Fiji" "斐濟"
    "Atlantic/Stanley" "史坦利"
    "Pacific/Kosrae" "科斯瑞"
    "Atlantic/Faroe" "法羅群島"
    "Europe/Paris" "巴黎"
    "Europe/London" "倫敦"
    "Asia/Tbilisi" "第比利斯"
    "America/Cayenne" "開雲"
    "Europe/Gibraltar" "直布羅陀"
    "America/Nuuk" "努克"
    "America/Danmarkshavn" "丹馬沙文"
    "America/Scoresbysund" "伊托科爾托米特"
    "America/Thule" "杜里"
    "Europe/Athens" "雅典"
    "Atlantic/South_Georgia" "南喬治亞"
    "America/Guatemala" "瓜地馬拉"
    "Pacific/Guam" "關島"
    "Africa/Bissau" "比紹"
    "America/Guyana" "蓋亞那"
    "Asia/Hong_Kong" "香港"
    "America/Tegucigalpa" "德古斯加巴"
    "America/Port-au-Prince" "太子港"
    "Europe/Budapest" "布達佩斯"
    "Asia/Jakarta" "雅加達"
    "Asia/Pontianak" "坤甸"
    "Asia/Makassar" "馬卡沙爾"
    "Asia/Jayapura" "加亞布拉"
    "Europe/Dublin" "都柏林"
    "Asia/Jerusalem" "耶路撒冷"
    "Asia/Kolkata" "加爾各答"
    "Indian/Chagos" "查戈斯"
    "Asia/Baghdad" "巴格達"
    "Asia/Tehran" "德黑蘭"
    "Europe/Rome" "羅馬"
    "America/Jamaica" "牙買加"
    "Asia/Amman" "安曼"
    "Asia/Tokyo" "東京"
    "Africa/Nairobi" "奈洛比"
    "Asia/Bishkek" "比什凱克"
    "Pacific/Tarawa" "塔拉瓦"
    "Pacific/Kanton" "坎頓島"
    "Pacific/Kiritimati" "基里地馬地島"
    "Asia/Pyongyang" "平壤"
    "Asia/Seoul" "首爾"
    "Asia/Almaty" "阿拉木圖"
    "Asia/Qyzylorda" "克孜勒奧爾達"
    "Asia/Qostanay" "庫斯塔奈"
    "Asia/Aqtobe" "阿克托比"
    "Asia/Aqtau" "阿克套"
    "Asia/Atyrau" "阿特勞"
    "Asia/Oral" "烏拉爾"
    "Asia/Beirut" "貝魯特"
    "Asia/Colombo" "可倫坡"
    "Africa/Monrovia" "蒙羅維亞"
    "Europe/Vilnius" "維爾紐斯"
    "Europe/Riga" "里加"
    "Africa/Tripoli" "的黎波里"
    "Africa/Casablanca" "卡薩布蘭卡"
    "Europe/Chisinau" "基西紐"
    "Pacific/Kwajalein" "瓜加林島"
    "Asia/Yangon" "仰光"
    "Asia/Ulaanbaatar" "烏蘭巴托"
    "Asia/Hovd" "科布多"
    "Asia/Macau" "澳門"
    "America/Martinique" "馬丁尼克"
    "Europe/Malta" "馬爾他"
    "Indian/Mauritius" "模里西斯"
    "Indian/Maldives" "馬爾地夫"
    "America/Mexico_City" "墨西哥市"
    "America/Cancun" "坎昆"
    "America/Merida" "梅里達"
    "America/Monterrey" "蒙特瑞"
    "America/Matamoros" "馬塔莫羅斯"
    "America/Chihuahua" "奇華華"
    "America/Ciudad_Juarez" "華雷斯"
    "America/Ojinaga" "奧希納加"
    "America/Mazatlan" "馬薩特蘭"
    "America/Bahia_Banderas" "巴伊亞班德拉斯"
    "America/Hermosillo" "埃莫西約"
    "America/Tijuana" "提華納"
    "Asia/Kuching" "古晉"
    "Africa/Maputo" "馬普托"
    "Africa/Windhoek" "溫得和克"
    "Pacific/Noumea" "諾美亞"
    "Pacific/Norfolk" "諾福克"
    "Africa/Lagos" "拉哥斯"
    "America/Managua" "馬拿瓜"
    "Asia/Kathmandu" "加德滿都"
    "Pacific/Nauru" "諾魯"
    "Pacific/Niue" "紐埃島"
    "Pacific/Auckland" "奧克蘭"
    "Pacific/Chatham" "查坦"
    "America/Panama" "巴拿馬"
    "America/Lima" "利馬"
    "Pacific/Tahiti" "大溪地"
    "Pacific/Marquesas" "馬可薩斯島"
    "Pacific/Gambier" "甘比爾群島"
    "Pacific/Port_Moresby" "莫士比港"
    "Pacific/Bougainville" "布干維爾"
    "Asia/Manila" "馬尼拉"
    "Asia/Karachi" "喀拉蚩"
    "Europe/Warsaw" "華沙"
    "America/Miquelon" "密啟崙"
    "Pacific/Pitcairn" "皮特肯群島"
    "America/Puerto_Rico" "波多黎各"
    "Asia/Gaza" "加薩"
    "Asia/Hebron" "赫布隆"
    "Europe/Lisbon" "里斯本"
    "Atlantic/Madeira" "馬得拉群島"
    "Atlantic/Azores" "亞速爾群島"
    "Pacific/Palau" "帛琉"
    "America/Asuncion" "亞松森"
    "Asia/Qatar" "卡達"
    "Europe/Bucharest" "布加勒斯特"
    "Europe/Belgrade" "貝爾格勒"
    "Europe/Kaliningrad" "加里寧格勒"
    "Europe/Moscow" "莫斯科"
    "Europe/Simferopol" "辛非洛浦"
    "Europe/Kirov" "基洛夫"
    "Europe/Volgograd" "伏爾加格勒"
    "Europe/Astrakhan" "阿斯特拉罕"
    "Europe/Saratov" "薩拉托夫"
    "Europe/Ulyanovsk" "烏里揚諾夫斯克"
    "Europe/Samara" "沙馬拉"
    "Asia/Yekaterinburg" "葉卡捷林堡"
    "Asia/Omsk" "鄂木斯克"
    "Asia/Novosibirsk" "新西伯利亞"
    "Asia/Barnaul" "巴爾瑙爾"
    "Asia/Tomsk" "托木斯克"
    "Asia/Novokuznetsk" "新庫茲涅茨克"
    "Asia/Krasnoyarsk" "克拉斯諾亞爾斯克"
    "Asia/Irkutsk" "伊爾庫次克"
    "Asia/Chita" "赤塔"
    "Asia/Yakutsk" "雅庫次克"
    "Asia/Khandyga" "堪地加"
    "Asia/Vladivostok" "海參崴"
    "Asia/Ust-Nera" "烏斯內拉"
    "Asia/Magadan" "馬加丹"
    "Asia/Sakhalin" "庫頁島"
    "Asia/Srednekolymsk" "中科雷姆斯克"
    "Asia/Kamchatka" "堪察加"
    "Asia/Anadyr" "阿那底"
    "Asia/Riyadh" "利雅德"
    "Pacific/Guadalcanal" "瓜達康納爾島"
    "Africa/Khartoum" "喀土穆"
    "Asia/Singapore" "新加坡"
    "America/Paramaribo" "巴拉馬利波"
    "Africa/Juba" "朱巴"
    "Africa/Sao_Tome" "聖多美"
    "America/El_Salvador" "薩爾瓦多"
    "Asia/Damascus" "大馬士革"
    "America/Grand_Turk" "大特克島"
    "Africa/Ndjamena" "恩賈梅納"
    "Asia/Bangkok" "曼谷"
    "Asia/Dushanbe" "杜桑貝"
    "Pacific/Fakaofo" "法考福"
    "Asia/Dili" "帝力"
    "Asia/Ashgabat" "阿什哈巴特"
    "Africa/Tunis" "突尼斯"
    "Pacific/Tongatapu" "東加塔布島"
    "Europe/Istanbul" "伊斯坦堡"
    "Asia/Taipei" "台北"
    "Europe/Kyiv" "基輔"
    "America/New_York" "紐約"
    "America/Detroit" "底特律"
    "America/Kentucky/Louisville" "路易斯維爾"
    "America/Kentucky/Monticello" "肯塔基州蒙地卻羅"
    "America/Indiana/Indianapolis" "印第安那波里斯"
    "America/Indiana/Vincennes" "印第安那州溫森斯"
    "America/Indiana/Winamac" "印第安那州威納馬克"
    "America/Indiana/Marengo" "印第安那州馬倫哥"
    "America/Indiana/Petersburg" "印第安那州彼得堡"
    "America/Indiana/Vevay" "印第安那州維威"
    "America/Chicago" "芝加哥"
    "America/Indiana/Tell_City" "印第安那州泰爾城"
    "America/Indiana/Knox" "印第安那州諾克斯"
    "America/Menominee" "美諾米尼"
    "America/North_Dakota/Center" "北達科他州中心"
    "America/North_Dakota/New_Salem" "北達科他州紐沙倫"
    "America/North_Dakota/Beulah" "北達科他州布由拉"
    "America/Denver" "丹佛"
    "America/Boise" "波夕"
    "America/Phoenix" "鳳凰城"
    "America/Los_Angeles" "洛杉磯"
    "America/Anchorage" "安克拉治"
    "America/Juneau" "朱諾"
    "America/Sitka" "錫特卡"
    "America/Metlakatla" "梅特拉卡特拉"
    "America/Yakutat" "雅庫塔"
    "America/Nome" "諾姆"
    "America/Adak" "艾達克"
    "Pacific/Honolulu" "檀香山"
    "America/Montevideo" "蒙特維多"
    "Asia/Samarkand" "撒馬爾罕"
    "Asia/Tashkent" "塔什干"
    "America/Caracas" "卡拉卡斯"
    "Asia/Ho_Chi_Minh" "胡志明市"
    "Pacific/Efate" "埃法特"
    "Pacific/Apia" "阿皮亞"
    "Africa/Johannesburg" "約翰尼斯堡"
    "America/Antigua" "安地卡"
    "America/Anguilla" "安奎拉"
    "Africa/Luanda" "羅安達"
    "Antarctica/McMurdo" "麥克默多"
    "Antarctica/DumontDUrville" "杜蒙杜比爾"
    "Antarctica/Syowa" "昭和基地"
    "America/Aruba" "荷屬阿魯巴"
    "Europe/Mariehamn" "瑪麗港"
    "Europe/Sarajevo" "塞拉耶佛"
    "Africa/Ouagadougou" "瓦加杜古"
    "Asia/Bahrain" "巴林"
    "Africa/Bujumbura" "布松布拉"
    "Africa/Porto-Novo" "波多諾佛"
    "America/St_Barthelemy" "聖巴托洛繆島"
    "Asia/Brunei" "汶萊"
    "America/Kralendijk" "克拉倫代克"
    "America/Nassau" "拿索"
    "Africa/Gaborone" "嘉柏隆里"
    "America/Blanc-Sablon" "白朗薩布隆"
    "America/Atikokan" "阿蒂科肯"
    "America/Creston" "克雷斯頓"
    "Indian/Cocos" "科科斯群島"
    "Africa/Kinshasa" "金夏沙"
    "Africa/Lubumbashi" "盧本巴希"
    "Africa/Bangui" "班吉"
    "Africa/Brazzaville" "布拉柴維爾"
    "Africa/Douala" "杜阿拉"
    "America/Curacao" "庫拉索"
    "Indian/Christmas" "聖誕島"
    "Europe/Busingen" "布辛根"
    "Africa/Djibouti" "吉布地"
    "Europe/Copenhagen" "哥本哈根"
    "America/Dominica" "多米尼克"
    "Africa/Asmara" "阿斯瑪拉"
    "Africa/Addis_Ababa" "阿迪斯阿貝巴"
    "Pacific/Chuuk" "楚克"
    "Pacific/Pohnpei" "波納佩"
    "Africa/Libreville" "自由市"
    "America/Grenada" "格瑞納達"
    "Europe/Guernsey" "根息島"
    "Africa/Accra" "阿克拉"
    "Africa/Banjul" "班竹"
    "Africa/Conakry" "柯那克里"
    "America/Guadeloupe" "瓜地洛普"
    "Africa/Malabo" "馬拉博"
    "Europe/Zagreb" "札格瑞布"
    "Europe/Isle_of_Man" "曼島"
    "Atlantic/Reykjavik" "雷克雅維克"
    "Europe/Jersey" "澤西島"
    "Asia/Phnom_Penh" "金邊"
    "Indian/Comoro" "科摩羅群島"
    "America/St_Kitts" "聖基茨"
    "Asia/Kuwait" "科威特"
    "America/Cayman" "開曼群島"
    "Asia/Vientiane" "永珍"
    "America/St_Lucia" "聖露西亞"
    "Europe/Vaduz" "瓦都茲"
    "Africa/Maseru" "馬賽魯"
    "Europe/Luxembourg" "盧森堡"
    "Europe/Monaco" "摩納哥"
    "Europe/Podgorica" "波多里察"
    "America/Marigot" "馬里戈特"
    "Indian/Antananarivo" "安塔那那利弗"
    "Pacific/Majuro" "馬朱諾"
    "Europe/Skopje" "史高比耶"
    "Africa/Bamako" "巴馬科"
    "Pacific/Saipan" "塞班"
    "Africa/Nouakchott" "諾克少"
    "America/Montserrat" "蒙哲臘"
    "Africa/Blantyre" "布蘭太爾"
    "Asia/Kuala_Lumpur" "吉隆坡"
    "Africa/Niamey" "尼亞美"
    "Europe/Amsterdam" "阿姆斯特丹"
    "Europe/Oslo" "奧斯陸"
    "Asia/Muscat" "馬斯開特"
    "Indian/Reunion" "留尼旺島"
    "Africa/Kigali" "基加利"
    "Indian/Mahe" "馬埃島"
    "Europe/Stockholm" "斯德哥爾摩"
    "Atlantic/St_Helena" "聖赫勒拿島"
    "Europe/Ljubljana" "盧比安納"
    "Arctic/Longyearbyen" "隆意耳拜恩"
    "Europe/Bratislava" "布拉提斯拉瓦"
    "Africa/Freetown" "自由城"
    "Europe/San_Marino" "聖馬利諾"
    "Africa/Dakar" "達喀爾"
    "Africa/Mogadishu" "摩加迪休"
    "America/Lower_Princes" "下太子區"
    "Africa/Mbabane" "墨巴本"
    "Indian/Kerguelen" "凱爾蓋朗島"
    "Africa/Lome" "洛美"
    "America/Port_of_Spain" "西班牙港"
    "Pacific/Funafuti" "富那富提"
    "Africa/Dar_es_Salaam" "沙蘭港"
    "Africa/Kampala" "坎帕拉"
    "Pacific/Midway" "中途島"
    "Pacific/Wake" "威克"
    "Europe/Vatican" "梵蒂岡"
    "America/St_Vincent" "聖文森"
    "America/Tortola" "托爾托拉"
    "America/St_Thomas" "聖托馬斯"
    "Pacific/Wallis" "瓦利斯"
    "Asia/Aden" "亞丁"
    "Indian/Mayotte" "馬約特島"
    "Africa/Lusaka" "路沙卡"
    "Africa/Harare" "哈拉雷"
};

// `common/main/fil.xml`: 13 of the 418 zones named, 405 inherited.
#[cfg(feature = "localized-exemplar-cities")]
const FIL: &str = exemplar_cities! {
    "Europe/Andorra" inherited
    "Asia/Dubai" inherited
    "Asia/Kabul" inherited
    "Europe/Tirane" inherited
    "Asia/Yerevan" inherited
    "Antarctica/Casey" inherited
    "Antarctica/Davis" inherited
    "Antarctica/Mawson" inherited
    "Antarctica/Palmer" inherited
    "Antarctica/Rothera" inherited
    "Antarctica/Troll" inherited
    "Antarctica/Vostok" inherited
    "America/Argentina/Buenos_Aires" inherited
    "America/Argentina/Cordoba" inherited
    "America/Argentina/Salta" inherited
    "America/Argentina/Jujuy" inherited
    "America/Argentina/Tucuman" inherited
    "America/Argentina/Catamarca" inherited
    "America/Argentina/La_Rioja" inherited
    "America/Argentina/San_Juan" inherited
    "America/Argentina/Mendoza" inherited
    "America/Argentina/San_Luis" inherited
    "America/Argentina/Rio_Gallegos" inherited
    "America/Argentina/Ushuaia" inherited
    "Pacific/Pago_Pago" inherited
    "Europe/Vienna" inherited
    "Australia/Lord_Howe" inherited
    "Antarctica/Macquarie" inherited
    "Australia/Hobart" inherited
    "Australia/Melbourne" inherited
    "Australia/Sydney" inherited
    "Australia/Broken_Hill" inherited
    "Australia/Brisbane" inherited
    "Australia/Lindeman" inherited
    "Australia/Adelaide" inherited
    "Australia/Darwin" inherited
    "Australia/Perth" inherited
    "Australia/Eucla" inherited
    "Asia/Baku" inherited
    "America/Barbados" inherited
    "Asia/Dhaka" inherited
    "Europe/Brussels" inherited
    "Europe/Sofia" inherited
    "Atlantic/Bermuda" inherited
    "America/La_Paz" inherited
    "America/Noronha" inherited
    "America/Belem" inherited
    "America/Fortaleza" inherited
    "America/Recife" inherited
    "America/Araguaina" inherited
    "America/Maceio" inherited
    "America/Bahia" inherited
    "America/Sao_Paulo" inherited
    "America/Campo_Grande" inherited
    "America/Cuiaba" inherited
    "America/Santarem" inherited
    "America/Porto_Velho" inherited
    "America/Boa_Vista" inherited
    "America/Manaus" inherited
    "America/Eirunepe" inherited
    "America/Rio_Branco" inherited
    "Asia/Thimphu" inherited
    "Europe/Minsk" inherited
    "America/Belize" inherited
    "America/St_Johns" inherited
    "America/Halifax" inherited
    "America/Glace_Bay" inherited
    "America/Moncton" inherited
    "America/Goose_Bay" inherited
    "America/Toronto" inherited
    "America/Iqaluit" inherited
    "America/Winnipeg" inherited
    "America/Resolute" inherited
    "America/Rankin_Inlet" "Makipot na Look ng Rankin"
    "America/Regina" inherited
    "America/Swift_Current" inherited
    "America/Edmonton" inherited
    "America/Cambridge_Bay" inherited
    "America/Inuvik" inherited
    "America/Vancouver" inherited
    "America/Dawson_Creek" inherited
    "America/Fort_Nelson" inherited
    "America/Whitehorse" inherited
    "America/Dawson" inherited
    "Europe/Zurich" inherited
    "Africa/Abidjan" inherited
    "Pacific/Rarotonga" inherited
    "America/Santiago" inherited
    "America/Coyhaique" inherited
    "America/Punta_Arenas" inherited
    "Pacific/Easter" inherited
    "Asia/Shanghai" inherited
    "Asia/Urumqi" inherited
    "America/Bogota" inherited
    "America/Costa_Rica" inherited
    "America/Havana" inherited
    "Atlantic/Cape_Verde" inherited
    "Asia/Nicosia" inherited
    "Asia/Famagusta" inherited
    "Europe/Prague" inherited
    "Europe/Berlin" inherited
    "America/Santo_Domingo" inherited
    "Africa/Algiers" inherited
    "America/Guayaquil" inherited
    "Pacific/Galapagos" inherited
    "Europe/Tallinn" inherited
    "Africa/Cairo" inherited
    "Africa/El_Aaiun" inherited
    "Europe/Madrid" inherited
    "Africa/Ceuta" inherited
    "Atlantic/Canary" inherited
    "Europe/Helsinki" inherited
    "Pacific/Fiji" inherited
    "Atlantic/Stanley" inherited
    "Pacific/Kosrae" inherited
    "Atlantic/Faroe" inherited
    "Europe/Paris" inherited
    "Europe/London" inherited
    "Asia/Tbilisi" inherited
    "America/Cayenne" inherited
    "Europe/Gibraltar" inherited
    "America/Nuuk" inherited
    "America/Danmarkshavn" inherited
    "America/Scoresbysund" inherited
    "America/Thule" inherited
    "Europe/Athens" inherited
    "Atlantic/South_Georgia" inherited
    "America/Guatemala" inherited
    "Pacific/Guam" inherited
    "Africa/Bissau" inherited
    "America/Guyana" inherited
    "Asia/Hong_Kong" inherited
    "America/Tegucigalpa" inherited
    "America/Port-au-Prince" inherited
    "Europe/Budapest" inherited
    "Asia/Jakarta" inherited
    "Asia/Pontianak" inherited
    "Asia/Makassar" inherited
    "Asia/Jayapura" inherited
    "Europe/Dublin" inherited
    "Asia/Jerusalem" inherited
    "Asia/Kolkata" inherited
    "Indian/Chagos" inherited
    "Asia/Baghdad" inherited
    "Asia/Tehran" inherited
    "Europe/Rome" inherited
    "America/Jamaica" inherited
    "Asia/Amman" inherited
    "Asia/Tokyo" inherited
    "Africa/Nairobi" inherited
    "Asia/Bishkek" inherited
    "Pacific/Tarawa" inherited
    "Pacific/Kanton" "Canton Island"
    "Pacific/Kiritimati" inherited
    "Asia/Pyongyang" inherited
    "Asia/Seoul" inherited
    "Asia/Almaty" inherited
    "Asia/Qyzylorda" inherited
    "Asia/Qostanay" "Kostanay"
    "Asia/Aqtobe" inherited
    "Asia/Aqtau" inherited
    "Asia/Atyrau" inherited
    "Asia/Oral" inherited
    "Asia/Beirut" inherited
    "Asia/Colombo" inherited
    "Africa/Monrovia" inherited
    "Europe/Vilnius" inherited
    "Europe/Riga" inherited
    "Africa/Tripoli" inherited
    "Africa/Casablanca" inherited
    "Europe/Chisinau" inherited
    "Pacific/Kwajalein" inherited
    "Asia/Yangon" inherited
    "Asia/Ulaanbaatar" inherited
    "Asia/Hovd" inherited
    "Asia/Macau" "Macau"
    "America/Martinique" inherited
    "Europe/Malta" inherited
    "Indian/Mauritius" inherited
    "Indian/Maldives" inherited
    "America/Mexico_City" "Lungsod ng Mexico"
    "America/Cancun" "Cancun"
    "America/Merida" "Merida"
    "America/Monterrey" inherited
    "America/Matamoros" inherited
    "America/Chihuahua" inherited
    "America/Ciudad_Juarez" "Lungsod ng Juárez"
    "America/Ojinaga" inherited
    "America/Mazatlan" inherited
    "America/Bahia_Banderas" "Bahia Banderas"
    "America/Hermosillo" inherited
    "America/Tijuana" inherited
    "Asia/Kuching" inherited
    "Africa/Maputo" inherited
    "Africa/Windhoek" inherited
    "Pacific/Noumea" inherited
    "Pacific/Norfolk" inherited
    "Africa/Lagos" inherited
    "America/Managua" inherited
    "Asia/Kathmandu" inherited
    "Pacific/Nauru" inherited
    "Pacific/Niue" inherited
    "Pacific/Auckland" inherited
    "Pacific/Chatham" inherited
    "America/Panama" inherited
    "America/Lima" inherited
    "Pacific/Tahiti" inherited
    "Pacific/Marquesas" inherited
    "Pacific/Gambier" inherited
    "Pacific/Port_Moresby" inherited
    "Pacific/Bougainville" inherited
    "Asia/Manila" inherited
    "Asia/Karachi" inherited
    "Europe/Warsaw" inherited
    "America/Miquelon" inherited
    "Pacific/Pitcairn" inherited
    "America/Puerto_Rico" inherited
    "Asia/Gaza" inherited
    "Asia/Hebron" inherited
    "Europe/Lisbon" inherited
    "Atlantic/Madeira" inherited
    "Atlantic/Azores" inherited
    "Pacific/Palau" inherited
    "America/Asuncion" inherited
    "Asia/Qatar" inherited
    "Europe/Bucharest" inherited
    "Europe/Belgrade" inherited
    "Europe/Kaliningrad" inherited
    "Europe/Moscow" inherited
    "Europe/Simferopol" inherited
    "Europe/Kirov" inherited
    "Europe/Volgograd" inherited
    "Europe/Astrakhan" inherited
    "Europe/Saratov" inherited
    "Europe/Ulyanovsk" inherited
    "Europe/Samara" inherited
    "Asia/Yekaterinburg" inherited
    "Asia/Omsk" inherited
    "Asia/Novosibirsk" inherited
    "Asia/Barnaul" inherited
    "Asia/Tomsk" inherited
    "Asia/Novokuznetsk" inherited
    "Asia/Krasnoyarsk" inherited
    "Asia/Irkutsk" inherited
    "Asia/Chita" inherited
    "Asia/Yakutsk" inherited
    "Asia/Khandyga" inherited
    "Asia/Vladivostok" inherited
    "Asia/Ust-Nera" inherited
    "Asia/Magadan" inherited
    "Asia/Sakhalin" inherited
    "Asia/Srednekolymsk" inherited
    "Asia/Kamchatka" inherited
    "Asia/Anadyr" inherited
    "Asia/Riyadh" inherited
    "Pacific/Guadalcanal" inherited
    "Africa/Khartoum" inherited
    "Asia/Singapore" inherited
    "America/Paramaribo" inherited
    "Africa/Juba" inherited
    "Africa/Sao_Tome" inherited
    "America/El_Salvador" inherited
    "Asia/Damascus" inherited
    "America/Grand_Turk" inherited
    "Africa/Ndjamena" inherited
    "Asia/Bangkok" inherited
    "Asia/Dushanbe" inherited
    "Pacific/Fakaofo" inherited
    "Asia/Dili" inherited
    "Asia/Ashgabat" inherited
    "Africa/Tunis" inherited
    "Pacific/Tongatapu" inherited
    "Europe/Istanbul" inherited
    "Asia/Taipei" inherited
    "Europe/Kyiv" "Kiev"
    "America/New_York" inherited
    "America/Detroit" inherited
    "America/Kentucky/Louisville" inherited
    "America/Kentucky/Monticello" inherited
    "America/Indiana/Indianapolis" inherited
    "America/Indiana/Vincennes" inherited
    "America/Indiana/Winamac" inherited
    "America/Indiana/Marengo" inherited
    "America/Indiana/Petersburg" inherited
    "America/Indiana/Vevay" inherited
    "America/Chicago" inherited
    "America/Indiana/Tell_City" inherited
    "America/Indiana/Knox" inherited
    "America/Menominee" inherited
    "America/North_Dakota/Center" inherited
    "America/North_Dakota/New_Salem" inherited
    "America/North_Dakota/Beulah" inherited
    "America/Denver" inherited
    "America/Boise" inherited
    "America/Phoenix" inherited
    "America/Los_Angeles" inherited
    "America/Anchorage" inherited
    "America/Juneau" inherited
    "America/Sitka" inherited
    "America/Metlakatla" inherited
    "America/Yakutat" inherited
    "America/Nome" inherited
    "America/Adak" inherited
    "Pacific/Honolulu" "Honolulu"
    "America/Montevideo" inherited
    "Asia/Samarkand" inherited
    "Asia/Tashkent" inherited
    "America/Caracas" inherited
    "Asia/Ho_Chi_Minh" "Lungsod ng Ho Chi Minh"
    "Pacific/Efate" inherited
    "Pacific/Apia" inherited
    "Africa/Johannesburg" inherited
    "America/Antigua" inherited
    "America/Anguilla" inherited
    "Africa/Luanda" inherited
    "Antarctica/McMurdo" inherited
    "Antarctica/DumontDUrville" inherited
    "Antarctica/Syowa" inherited
    "America/Aruba" inherited
    "Europe/Mariehamn" inherited
    "Europe/Sarajevo" inherited
    "Africa/Ouagadougou" inherited
    "Asia/Bahrain" inherited
    "Africa/Bujumbura" inherited
    "Africa/Porto-Novo" inherited
    "America/St_Barthelemy" inherited
    "Asia/Brunei" inherited
    "America/Kralendijk" inherited
    "America/Nassau" inherited
    "Africa/Gaborone" inherited
    "America/Blanc-Sablon" inherited
    "America/Atikokan" inherited
    "America/Creston" inherited
    "Indian/Cocos" inherited
    "Africa/Kinshasa" inherited
    "Africa/Lubumbashi" inherited
    "Africa/Bangui" inherited
    "Africa/Brazzaville" inherited
    "Africa/Douala" inherited
    "America/Curacao" inherited
    "Indian/Christmas" inherited
    "Europe/Busingen" inherited
    "Africa/Djibouti" inherited
    "Europe/Copenhagen" inherited
    "America/Dominica" inherited
    "Africa/Asmara" inherited
    "Africa/Addis_Ababa" inherited
    "Pacific/Chuuk" inherited
    "Pacific/Pohnpei" inherited
    "Africa/Libreville" inherited
    "America/Grenada" inherited
    "Europe/Guernsey" inherited
    "Africa/Accra" inherited
    "Africa/Banjul" inherited
    "Africa/Conakry" inherited
    "America/Guadeloupe" inherited
    "Africa/Malabo" inherited
    "Europe/Zagreb" inherited
    "Europe/Isle_of_Man" inherited
    "Atlantic/Reykjavik" inherited
    "Europe/Jersey" inherited
    "Asia/Phnom_Penh" inherited
    "Indian/Comoro" inherited
    "America/St_Kitts" inherited
    "Asia/Kuwait" inherited
    "America/Cayman" inherited
    "Asia/Vientiane" inherited
    "America/St_Lucia" inherited
    "Europe/Vaduz" inherited
    "Africa/Maseru" inherited
    "Europe/Luxembourg" inherited
    "Europe/Monaco" inherited
    "Europe/Podgorica" inherited
    "America/Marigot" inherited
    "Indian/Antananarivo" inherited
    "Pacific/Majuro" inherited
    "Europe/Skopje" inherited
    "Africa/Bamako" inherited
    "Pacific/Saipan" inherited
    "Africa/Nouakchott" inherited
    "America/Montserrat" inherited
    "Africa/Blantyre" inherited
    "Asia/Kuala_Lumpur" inherited
    "Africa/Niamey" inherited
    "Europe/Amsterdam" inherited
    "Europe/Oslo" inherited
    "Asia/Muscat" inherited
    "Indian/Reunion" inherited
    "Africa/Kigali" inherited
    "Indian/Mahe" inherited
    "Europe/Stockholm" inherited
    "Atlantic/St_Helena" inherited
    "Europe/Ljubljana" inherited
    "Arctic/Longyearbyen" inherited
    "Europe/Bratislava" inherited
    "Africa/Freetown" inherited
    "Europe/San_Marino" inherited
    "Africa/Dakar" inherited
    "Africa/Mogadishu" inherited
    "America/Lower_Princes" inherited
    "Africa/Mbabane" inherited
    "Indian/Kerguelen" inherited
    "Africa/Lome" inherited
    "America/Port_of_Spain" "Puwerto ng Espanya"
    "Pacific/Funafuti" inherited
    "Africa/Dar_es_Salaam" inherited
    "Africa/Kampala" inherited
    "Pacific/Midway" inherited
    "Pacific/Wake" inherited
    "Europe/Vatican" inherited
    "America/St_Vincent" inherited
    "America/Tortola" inherited
    "America/St_Thomas" inherited
    "Pacific/Wallis" inherited
    "Asia/Aden" inherited
    "Indian/Mayotte" inherited
    "Africa/Lusaka" inherited
    "Africa/Harare" inherited
};

// `common/main/ha.xml`: 4 of the 418 zones named, 413 inherited.
#[cfg(feature = "localized-exemplar-cities")]
const HA: &str = exemplar_cities! {
    "Europe/Andorra" inherited
    "Asia/Dubai" inherited
    "Asia/Kabul" inherited
    "Europe/Tirane" inherited
    "Asia/Yerevan" inherited
    "Antarctica/Casey" inherited
    "Antarctica/Davis" inherited
    "Antarctica/Mawson" inherited
    "Antarctica/Palmer" inherited
    "Antarctica/Rothera" inherited
    "Antarctica/Troll" inherited
    "Antarctica/Vostok" inherited
    "America/Argentina/Buenos_Aires" inherited
    "America/Argentina/Cordoba" inherited
    "America/Argentina/Salta" inherited
    "America/Argentina/Jujuy" inherited
    "America/Argentina/Tucuman" inherited
    "America/Argentina/Catamarca" inherited
    "America/Argentina/La_Rioja" inherited
    "America/Argentina/San_Juan" inherited
    "America/Argentina/Mendoza" inherited
    "America/Argentina/San_Luis" inherited
    "America/Argentina/Rio_Gallegos" inherited
    "America/Argentina/Ushuaia" inherited
    "Pacific/Pago_Pago" inherited
    "Europe/Vienna" inherited
    "Australia/Lord_Howe" inherited
    "Antarctica/Macquarie" inherited
    "Australia/Hobart" inherited
    "Australia/Melbourne" inherited
    "Australia/Sydney" inherited
    "Australia/Broken_Hill" inherited
    "Australia/Brisbane" inherited
    "Australia/Lindeman" inherited
    "Australia/Adelaide" inherited
    "Australia/Darwin" inherited
    "Australia/Perth" inherited
    "Australia/Eucla" inherited
    "Asia/Baku" inherited
    "America/Barbados" inherited
    "Asia/Dhaka" inherited
    "Europe/Brussels" inherited
    "Europe/Sofia" inherited
    "Atlantic/Bermuda" inherited
    "America/La_Paz" inherited
    "America/Noronha" inherited
    "America/Belem" inherited
    "America/Fortaleza" inherited
    "America/Recife" inherited
    "America/Araguaina" inherited
    "America/Maceio" inherited
    "America/Bahia" inherited
    "America/Sao_Paulo" inherited
    "America/Campo_Grande" inherited
    "America/Cuiaba" inherited
    "America/Santarem" inherited
    "America/Porto_Velho" inherited
    "America/Boa_Vista" inherited
    "America/Manaus" inherited
    "America/Eirunepe" inherited
    "America/Rio_Branco" inherited
    "Asia/Thimphu" inherited
    "Europe/Minsk" inherited
    "America/Belize" inherited
    "America/St_Johns" inherited
    "America/Halifax" inherited
    "America/Glace_Bay" inherited
    "America/Moncton" inherited
    "America/Goose_Bay" inherited
    "America/Toronto" inherited
    "America/Iqaluit" inherited
    "America/Winnipeg" inherited
    "America/Resolute" inherited
    "America/Rankin_Inlet" inherited
    "America/Regina" inherited
    "America/Swift_Current" inherited
    "America/Edmonton" inherited
    "America/Cambridge_Bay" inherited
    "America/Inuvik" inherited
    "America/Vancouver" inherited
    "America/Dawson_Creek" inherited
    "America/Fort_Nelson" inherited
    "America/Whitehorse" inherited
    "America/Dawson" inherited
    "Europe/Zurich" inherited
    "Africa/Abidjan" inherited
    "Pacific/Rarotonga" inherited
    "America/Santiago" inherited
    "America/Coyhaique" inherited
    "America/Punta_Arenas" inherited
    "Pacific/Easter" inherited
    "Asia/Shanghai" inherited
    "Asia/Urumqi" inherited
    "America/Bogota" inherited
    "America/Costa_Rica" inherited
    "America/Havana" inherited
    "Atlantic/Cape_Verde" inherited
    "Asia/Nicosia" inherited
    "Asia/Famagusta" inherited
    "Europe/Prague" inherited
    "Europe/Berlin" inherited
    "America/Santo_Domingo" inherited
    "Africa/Algiers" inherited
    "America/Guayaquil" inherited
    "Pacific/Galapagos" inherited
    "Europe/Tallinn" inherited
    "Africa/Cairo" inherited
    "Africa/El_Aaiun" inherited
    "Europe/Madrid" inherited
    "Africa/Ceuta" inherited
    "Atlantic/Canary" inherited
    "Europe/Helsinki" inherited
    "Pacific/Fiji" inherited
    "Atlantic/Stanley" inherited
    "Pacific/Kosrae" inherited
    "Atlantic/Faroe" inherited
    "Europe/Paris" inherited
    "Europe/London" inherited
    "Asia/Tbilisi" inherited
    "America/Cayenne" inherited
    "Europe/Gibraltar" inherited
    "America/Nuuk" inherited
    "America/Danmarkshavn" inherited
    "America/Scoresbysund" inherited
    "America/Thule" inherited
    "Europe/Athens" inherited
    "Atlantic/South_Georgia" inherited
    "America/Guatemala" inherited
    "Pacific/Guam" inherited
    "Africa/Bissau" inherited
    "America/Guyana" inherited
    "Asia/Hong_Kong" inherited
    "America/Tegucigalpa" inherited
    "America/Port-au-Prince" inherited
    "Europe/Budapest" inherited
    "Asia/Jakarta" inherited
    "Asia/Pontianak" inherited
    "Asia/Makassar" inherited
    "Asia/Jayapura" inherited
    "Europe/Dublin" inherited
    "Asia/Jerusalem" inherited
    "Asia/Kolkata" inherited
    "Indian/Chagos" inherited
    "Asia/Baghdad" inherited
    "Asia/Tehran" inherited
    "Europe/Rome" inherited
    "America/Jamaica" inherited
    "Asia/Amman" inherited
    "Asia/Tokyo" inherited
    "Africa/Nairobi" inherited
    "Asia/Bishkek" inherited
    "Pacific/Tarawa" inherited
    "Pacific/Kanton" "Tsibirin Canton"
    "Pacific/Kiritimati" inherited
    "Asia/Pyongyang" inherited
    "Asia/Seoul" inherited
    "Asia/Almaty" inherited
    "Asia/Qyzylorda" inherited
    "Asia/Qostanay" inherited
    "Asia/Aqtobe" inherited
    "Asia/Aqtau" inherited
    "Asia/Atyrau" inherited
    "Asia/Oral" inherited
    "Asia/Beirut" inherited
    "Asia/Colombo" inherited
    "Africa/Monrovia" inherited
    "Europe/Vilnius" inherited
    "Europe/Riga" inherited
    "Africa/Tripoli" inherited
    "Africa/Casablanca" inherited
    "Europe/Chisinau" inherited
    "Pacific/Kwajalein" inherited
    "Asia/Yangon" inherited
    "Asia/Ulaanbaatar" inherited
    "Asia/Hovd" inherited
    "Asia/Macau" inherited
    "America/Martinique" inherited
    "Europe/Malta" inherited
    "Indian/Mauritius" inherited
    "Indian/Maldives" inherited
    "America/Mexico_City" inherited
    "America/Cancun" inherited
    "America/Merida" inherited
    "America/Monterrey" inherited
    "America/Matamoros" inherited
    "America/Chihuahua" inherited
    "America/Ciudad_Juarez" inherited
    "America/Ojinaga" inherited
    "America/Mazatlan" inherited
    "America/Bahia_Banderas" inherited
    "America/Hermosillo" inherited
    "America/Tijuana" inherited
    "Asia/Kuching" inherited
    "Africa/Maputo" inherited
    "Africa/Windhoek" inherited
    "Pacific/Noumea" inherited
    "Pacific/Norfolk" inherited
    "Africa/Lagos" inherited
    "America/Managua" inherited
    "Asia/Kathmandu" inherited
    "Pacific/Nauru" inherited
    "Pacific/Niue" inherited
    "Pacific/Auckland" inherited
    "Pacific/Chatham" inherited
    "America/Panama" inherited
    "America/Lima" inherited
    "Pacific/Tahiti" inherited
    "Pacific/Marquesas" inherited
    "Pacific/Gambier" inherited
    "Pacific/Port_Moresby" inherited
    "Pacific/Bougainville" inherited
    "Asia/Manila" inherited
    "Asia/Karachi" inherited
    "Europe/Warsaw" inherited
    "America/Miquelon" inherited
    "Pacific/Pitcairn" inherited
    "America/Puerto_Rico" inherited
    "Asia/Gaza" inherited
    "Asia/Hebron" inherited
    "Europe/Lisbon" inherited
    "Atlantic/Madeira" inherited
    "Atlantic/Azores" inherited
    "Pacific/Palau" inherited
    "America/Asuncion" inherited
    "Asia/Qatar" inherited
    "Europe/Bucharest" inherited
    "Europe/Belgrade" inherited
    "Europe/Kaliningrad" inherited
    "Europe/Moscow" inherited
    "Europe/Simferopol" inherited
    "Europe/Kirov" inherited
    "Europe/Volgograd" inherited
    "Europe/Astrakhan" inherited
    "Europe/Saratov" inherited
    "Europe/Ulyanovsk" inherited
    "Europe/Samara" inherited
    "Asia/Yekaterinburg" inherited
    "Asia/Omsk" inherited
    "Asia/Novosibirsk" inherited
    "Asia/Barnaul" inherited
    "Asia/Tomsk" inherited
    "Asia/Novokuznetsk" inherited
    "Asia/Krasnoyarsk" inherited
    "Asia/Irkutsk" inherited
    "Asia/Chita" inherited
    "Asia/Yakutsk" inherited
    "Asia/Khandyga" inherited
    "Asia/Vladivostok" inherited
    "Asia/Ust-Nera" inherited
    "Asia/Magadan" inherited
    "Asia/Sakhalin" inherited
    "Asia/Srednekolymsk" inherited
    "Asia/Kamchatka" inherited
    "Asia/Anadyr" inherited
    "Asia/Riyadh" inherited
    "Pacific/Guadalcanal" inherited
    "Africa/Khartoum" inherited
    "Asia/Singapore" inherited
    "America/Paramaribo" inherited
    "Africa/Juba" inherited
    "Africa/Sao_Tome" inherited
    "America/El_Salvador" inherited
    "Asia/Damascus" inherited
    "America/Grand_Turk" inherited
    "Africa/Ndjamena" inherited
    "Asia/Bangkok" inherited
    "Asia/Dushanbe" inherited
    "Pacific/Fakaofo" inherited
    "Asia/Dili" inherited
    "Asia/Ashgabat" inherited
    "Africa/Tunis" inherited
    "Pacific/Tongatapu" inherited
    "Europe/Istanbul" inherited
    "Asia/Taipei" inherited
    "Europe/Kyiv" inherited
    "America/New_York" inherited
    "America/Detroit" inherited
    "America/Kentucky/Louisville" inherited
    "America/Kentucky/Monticello" inherited
    "America/Indiana/Indianapolis" inherited
    "America/Indiana/Vincennes" inherited
    "America/Indiana/Winamac" inherited
    "America/Indiana/Marengo" inherited
    "America/Indiana/Petersburg" inherited
    "America/Indiana/Vevay" inherited
    "America/Chicago" inherited
    "America/Indiana/Tell_City" inherited
    "America/Indiana/Knox" inherited
    "America/Menominee" inherited
    "America/North_Dakota/Center" "Center, Arewacin Dakota"
    "America/North_Dakota/New_Salem" "New Salem, Arewacin Dakota"
    "America/North_Dakota/Beulah" "Beulah, Arewacin Dakota"
    "America/Denver" inherited
    "America/Boise" inherited
    "America/Phoenix" inherited
    "America/Los_Angeles" inherited
    "America/Anchorage" inherited
    "America/Juneau" inherited
    "America/Sitka" inherited
    "America/Metlakatla" inherited
    "America/Yakutat" inherited
    "America/Nome" inherited
    "America/Adak" inherited
    "Pacific/Honolulu" ""
    "America/Montevideo" inherited
    "Asia/Samarkand" inherited
    "Asia/Tashkent" inherited
    "America/Caracas" inherited
    "Asia/Ho_Chi_Minh" inherited
    "Pacific/Efate" inherited
    "Pacific/Apia" inherited
    "Africa/Johannesburg" inherited
    "America/Antigua" inherited
    "America/Anguilla" inherited
    "Africa/Luanda" inherited
    "Antarctica/McMurdo" inherited
    "Antarctica/DumontDUrville" inherited
    "Antarctica/Syowa" inherited
    "America/Aruba" inherited
    "Europe/Mariehamn" inherited
    "Europe/Sarajevo" inherited
    "Africa/Ouagadougou" inherited
    "Asia/Bahrain" inherited
    "Africa/Bujumbura" inherited
    "Africa/Porto-Novo" inherited
    "America/St_Barthelemy" inherited
    "Asia/Brunei" inherited
    "America/Kralendijk" inherited
    "America/Nassau" inherited
    "Africa/Gaborone" inherited
    "America/Blanc-Sablon" inherited
    "America/Atikokan" inherited
    "America/Creston" inherited
    "Indian/Cocos" inherited
    "Africa/Kinshasa" inherited
    "Africa/Lubumbashi" inherited
    "Africa/Bangui" inherited
    "Africa/Brazzaville" inherited
    "Africa/Douala" inherited
    "America/Curacao" inherited
    "Indian/Christmas" inherited
    "Europe/Busingen" inherited
    "Africa/Djibouti" inherited
    "Europe/Copenhagen" inherited
    "America/Dominica" inherited
    "Africa/Asmara" inherited
    "Africa/Addis_Ababa" inherited
    "Pacific/Chuuk" inherited
    "Pacific/Pohnpei" inherited
    "Africa/Libreville" inherited
    "America/Grenada" inherited
    "Europe/Guernsey" inherited
    "Africa/Accra" inherited
    "Africa/Banjul" inherited
    "Africa/Conakry" inherited
    "America/Guadeloupe" inherited
    "Africa/Malabo" inherited
    "Europe/Zagreb" inherited
    "Europe/Isle_of_Man" inherited
    "Atlantic/Reykjavik" inherited
    "Europe/Jersey" inherited
    "Asia/Phnom_Penh" inherited
    "Indian/Comoro" inherited
    "America/St_Kitts" inherited
    "Asia/Kuwait" inherited
    "America/Cayman" inherited
    "Asia/Vientiane" inherited
    "America/St_Lucia" inherited
    "Europe/Vaduz" inherited
    "Africa/Maseru" inherited
    "Europe/Luxembourg" inherited
    "Europe/Monaco" inherited
    "Europe/Podgorica" inherited
    "America/Marigot" inherited
    "Indian/Antananarivo" inherited
    "Pacific/Majuro" inherited
    "Europe/Skopje" inherited
    "Africa/Bamako" inherited
    "Pacific/Saipan" inherited
    "Africa/Nouakchott" inherited
    "America/Montserrat" inherited
    "Africa/Blantyre" inherited
    "Asia/Kuala_Lumpur" inherited
    "Africa/Niamey" inherited
    "Europe/Amsterdam" inherited
    "Europe/Oslo" inherited
    "Asia/Muscat" inherited
    "Indian/Reunion" inherited
    "Africa/Kigali" inherited
    "Indian/Mahe" inherited
    "Europe/Stockholm" inherited
    "Atlantic/St_Helena" inherited
    "Europe/Ljubljana" inherited
    "Arctic/Longyearbyen" inherited
    "Europe/Bratislava" inherited
    "Africa/Freetown" inherited
    "Europe/San_Marino" inherited
    "Africa/Dakar" inherited
    "Africa/Mogadishu" inherited
    "America/Lower_Princes" inherited
    "Africa/Mbabane" inherited
    "Indian/Kerguelen" inherited
    "Africa/Lome" inherited
    "America/Port_of_Spain" inherited
    "Pacific/Funafuti" inherited
    "Africa/Dar_es_Salaam" inherited
    "Africa/Kampala" inherited
    "Pacific/Midway" inherited
    "Pacific/Wake" inherited
    "Europe/Vatican" inherited
    "America/St_Vincent" inherited
    "America/Tortola" inherited
    "America/St_Thomas" inherited
    "Pacific/Wallis" inherited
    "Asia/Aden" inherited
    "Indian/Mayotte" inherited
    "Africa/Lusaka" inherited
    "Africa/Harare" inherited
};

// `common/main/mr.xml`: 418 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const MR: &str = exemplar_cities! {
    "Europe/Andorra" "अँडोरा"
    "Asia/Dubai" "दुबई"
    "Asia/Kabul" "काबूल"
    "Europe/Tirane" "टिराने"
    "Asia/Yerevan" "येरेवन"
    "Antarctica/Casey" "कॅसे"
    "Antarctica/Davis" "डेव्हिस"
    "Antarctica/Mawson" "मॉसन"
    "Antarctica/Palmer" "पामेर"
    "Antarctica/Rothera" "रोथेरा"
    "Antarctica/Troll" "ट्रोल"
    "Antarctica/Vostok" "वोस्टोक"
    "America/Argentina/Buenos_Aires" "ब्युनोस आयर्स"
    "America/Argentina/Cordoba" "कॉर्डोबा"
    "America/Argentina/Salta" "सॉल्ता"
    "America/Argentina/Jujuy" "जुजुय"
    "America/Argentina/Tucuman" "टुकुमान"
    "America/Argentina/Catamarca" "कॅटामार्का"
    "America/Argentina/La_Rioja" "ला रियोजा"
    "America/Argentina/San_Juan" "सान जुआन"
    "America/Argentina/Mendoza" "मेंदोझा"
    "America/Argentina/San_Luis" "सान ल्युइस"
    "America/Argentina/Rio_Gallegos" "रियो गॅलेगॉस"
    "America/Argentina/Ushuaia" "उस्वाइया"
    "Pacific/Pago_Pago" "पॅगो पॅगो"
    "Europe/Vienna" "व्हिएन्ना"
    "Australia/Lord_Howe" "लॉर्ड होवे"
    "Antarctica/Macquarie" "मॅक्वायर"
    "Australia/Hobart" "होबार्ट"
    "Australia/Melbourne" "मेलबोर्न"
    "Australia/Sydney" "सिडनी"
    "Australia/Broken_Hill" "ब्रोकन हिल"
    "Australia/Brisbane" "ब्रिस्बेन"
    "Australia/Lindeman" "लिंडेमन"
    "Australia/Adelaide" "एडलेड"
    "Australia/Darwin" "डार्विन"
    "Australia/Perth" "पर्थ"
    "Australia/Eucla" "उक्ला"
    "Asia/Baku" "बाकु"
    "America/Barbados" "बार्बाडोस"
    "Asia/Dhaka" "ढाका"
    "Europe/Brussels" "ब्रुसेल्स"
    "Europe/Sofia" "सोफिया"
    "Atlantic/Bermuda" "बर्मुडा"
    "America/La_Paz" "ला पाझ"
    "America/Noronha" "नोरोन्हा"
    "America/Belem" "बेलेम"
    "America/Fortaleza" "फोर्टालेझा"
    "America/Recife" "रेसिफे"
    "America/Araguaina" "अरागायना"
    "America/Maceio" "मेसेइओ"
    "America/Bahia" "बहिया"
    "America/Sao_Paulo" "साओ पावलो"
    "America/Campo_Grande" "कॅम्पो ग्रँडे"
    "America/Cuiaba" "कुयाबा"
    "America/Santarem" "सँटारेम"
    "America/Porto_Velho" "पोर्टो वेल्हो"
    "America/Boa_Vista" "बोआ व्हिस्टा"
    "America/Manaus" "मनौस"
    "America/Eirunepe" "यूरुनीपे"
    "America/Rio_Branco" "रियो ब्रांको"
    "Asia/Thimphu" "थिंफू"
    "Europe/Minsk" "मिन्स्क"
    "America/Belize" "बेलिझे"
    "America/St_Johns" "सेंट जॉन्स"
    "America/Halifax" "हॅलिफॅक्स"
    "America/Glace_Bay" "ग्लेस उपसागर"
    "America/Moncton" "माँकटन"
    "America/Goose_Bay" "गूस उपसागर"
    "America/Toronto" "टोरोंटो"
    "America/Iqaluit" "इकालुइत"
    "America/Winnipeg" "विनीपेग"
    "America/Resolute" "रेजोल्यूट"
    "America/Rankin_Inlet" "रॅनकिन इनलेट"
    "America/Regina" "रेजिना"
    "America/Swift_Current" "स्विफ्ट करंट"
    "America/Edmonton" "एडमाँटन"
    "America/Cambridge_Bay" "केंब्रिज उपसागर"
    "America/Inuvik" "इनुविक"
    "America/Vancouver" "व्हॅनकुव्हर"
    "America/Dawson_Creek" "डॉसन क्रीक"
    "America/Fort_Nelson" "फोर्ट नेल्सन"
    "America/Whitehorse" "व्हाइटहॉर्स"
    "America/Dawson" "डॉसन"
    "Europe/Zurich" "झुरिक"
    "Africa/Abidjan" "अबिद्जान"
    "Pacific/Rarotonga" "रारोटोंगा"
    "America/Santiago" "सॅन्टिएगो"
    "America/Coyhaique" "कोयाइके"
    "America/Punta_Arenas" "पुंता अरीनास"
    "Pacific/Easter" "ईस्टर"
    "Asia/Shanghai" "शांघाय"
    "Asia/Urumqi" "उरुम्की"
    "America/Bogota" "बोगोटा"
    "America/Costa_Rica" "कोस्टा रिका"
    "America/Havana" "हवाना"
    "Atlantic/Cape_Verde" "केप व्हर्डे"
    "Asia/Nicosia" "निकोसिया"
    "Asia/Famagusta" "फॅमगुस्ता"
    "Europe/Prague" "प्राग"
    "Europe/Berlin" "बर्लिन"
    "America/Santo_Domingo" "सॅन्टो डोमिंगो"
    "Africa/Algiers" "अल्जिअर्स"
    "America/Guayaquil" "गयाक्विल"
    "Pacific/Galapagos" "गॅलापागोस"
    "Europe/Tallinn" "तालिन"
    "Africa/Cairo" "कैरो"
    "Africa/El_Aaiun" "एल ऐउन"
    "Europe/Madrid" "माद्रिद"
    "Africa/Ceuta" "सेउटा"
    "Atlantic/Canary" "कॅनरी"
    "Europe/Helsinki" "हेलसिंकी"
    "Pacific/Fiji" "फिजी"
    "Atlantic/Stanley" "स्टॅनले"
    "Pacific/Kosrae" "कोशाय"
    "Atlantic/Faroe" "फॅरो"
    "Europe/Paris" "पॅरिस"
    "Europe/London" "लंडन"
    "Asia/Tbilisi" "बिलिसी"
    "America/Cayenne" "कायेने"
    "Europe/Gibraltar" "जिब्राल्टर"
    "America/Nuuk" "नूक"
    "America/Danmarkshavn" "डेन्मार्कशॉन"
    "America/Scoresbysund" "इटोकॉरटॉर्मीट"
    "America/Thule" "थुले"
    "Europe/Athens" "अथेन्स"
    "Atlantic/South_Georgia" "दक्षिण जॉर्जिया"
    "America/Guatemala" "ग्वाटेमाला"
    "Pacific/Guam" "गुआम"
    "Africa/Bissau" "बिसाउ"
    "America/Guyana" "गयाना"
    "Asia/Hong_Kong" "हाँगकाँग"
    "America/Tegucigalpa" "टेगुसिगाल्पा"
    "America/Port-au-Prince" "पोर्ट-औ-प्रिंस"
    "Europe/Budapest" "बुडापेस्ट"
    "Asia/Jakarta" "जकार्ता"
    "Asia/Pontianak" "पाँटियानाक"
    "Asia/Makassar" "मकस्सार"
    "Asia/Jayapura" "जयापुरा"
    "Europe/Dublin" "डब्लिन"
    "Asia/Jerusalem" "जेरुसलेम"
    "Asia/Kolkata" "कोलकाता"
    "Indian/Chagos" "चागोस"
    "Asia/Baghdad" "बगदाद"
    "Asia/Tehran" "तेहरान"
    "Europe/Rome" "रोम"
    "America/Jamaica" "जमैका"
    "Asia/Amman" "अम्मान"
    "Asia/Tokyo" "टोकियो"
    "Africa/Nairobi" "नैरोबी"
    "Asia/Bishkek" "बिश्केक"
    "Pacific/Tarawa" "तारावा"
    "Pacific/Kanton" "कँटन"
    "Pacific/Kiritimati" "किरितिमाती"
    "Asia/Pyongyang" "प्योंगयांग"
    "Asia/Seoul" "सेउल"
    "Asia/Almaty" "अल्माटी"
    "Asia/Qyzylorda" "किझीलोर्डा"
    "Asia/Qostanay" "कोस्टाने"
    "Asia/Aqtobe" "अ‍ॅक्टोबे"
    "Asia/Aqtau" "अ‍ॅक्टौ"
    "Asia/Atyrau" "अतिरॉ"
    "Asia/Oral" "ओरल"
    "Asia/Beirut" "बैरुत"
    "Asia/Colombo" "कोलंबो"
    "Africa/Monrovia" "मोनरोव्हिया"
    "Europe/Vilnius" "विलनियस"
    "Europe/Riga" "रिगा"
    "Africa/Tripoli" "त्रिपोली"
    "Africa/Casablanca" "कॅसाब्लान्का"
    "Europe/Chisinau" "चिसिनौ"
    "Pacific/Kwajalein" "क्वाजालेईन"
    "Asia/Yangon" "रंगून"
    "Asia/Ulaanbaatar" "उलानबातर"
    "Asia/Hovd" "होव्ड"
    "Asia/Macau" "मकाऊ"
    "America/Martinique" "मार्टिनिक"
    "Europe/Malta" "माल्टा"
    "Indian/Mauritius" "मॉरिशस"
    "Indian/Maldives" "मालदीव"
    "America/Mexico_City" "मेक्सिको सिटी"
    "America/Cancun" "कानकुन"
    "America/Merida" "मेरिडा"
    "America/Monterrey" "मॉन्टेरे"
    "America/Matamoros" "माटामोरोस"
    "America/Chihuahua" "चिहुआहुआ"
    "America/Ciudad_Juarez" "सिउदाद हुआरेझ"
    "America/Ojinaga" "ओजिनागा"
    "America/Mazatlan" "माझातलान"
    "America/Bahia_Banderas" "बाहिया बांदेरास"
    "America/Hermosillo" "हर्मोसिलो"
    "America/Tijuana" "तिजुआना"
    "Asia/Kuching" "कुचिंग"
    "Africa/Maputo" "मापुटो"
    "Africa/Windhoek" "विंडहोएक"
    "Pacific/Noumea" "नौमिआ"
    "Pacific/Norfolk" "नॉरफोक"
    "Africa/Lagos" "लागोस"
    "America/Managua" "मानागुआ"
    "Asia/Kathmandu" "काठमांडू"
    "Pacific/Nauru" "नउरु"
    "Pacific/Niue" "न्युए"
    "Pacific/Auckland" "ऑकलंड"
    "Pacific/Chatham" "चॅटहॅम"
    "America/Panama" "पनामा"
    "America/Lima" "लीमा"
    "Pacific/Tahiti" "ताहिती"
    "Pacific/Marquesas" "मारक्विसास"
    "Pacific/Gambier" "गॅम्बियर"
    "Pacific/Port_Moresby" "पोर्ट मोरेस्बे"
    "Pacific/Bougainville" "बॉगॅनव्हिल"
    "Asia/Manila" "मनिला"
    "Asia/Karachi" "कराची"
    "Europe/Warsaw" "वॉर्सा"
    "America/Miquelon" "मिक्वेलोन"
    "Pacific/Pitcairn" "पिटकेर्न"
    "America/Puerto_Rico" "प्युएर्तो रिको"
    "Asia/Gaza" "गाझा"
    "Asia/Hebron" "हेब्रॉन"
    "Europe/Lisbon" "लिस्बन"
    "Atlantic/Madeira" "मडीयरा"
    "Atlantic/Azores" "अझोरेस"
    "Pacific/Palau" "पलाऊ"
    "America/Asuncion" "आसुन्सियोन"
    "Asia/Qatar" "कतार"
    "Europe/Bucharest" "बुखारेस्ट"
    "Europe/Belgrade" "बेलग्रेड"
    "Europe/Kaliningrad" "कलिनिनग्राड"
    "Europe/Moscow" "मॉस्को"
    "Europe/Simferopol" "सिम्फरोपोल"
    "Europe/Kirov" "किरोव"
    "Europe/Volgograd" "व्होल्गोग्राड"
    "Europe/Astrakhan" "आस्त्राखान"
    "Europe/Saratov" "सारातोव"
    "Europe/Ulyanovsk" "उल्यानोव्स्क"
    "Europe/Samara" "समारा"
    "Asia/Yekaterinburg" "येक्तेरिनबर्ग"
    "Asia/Omsk" "ओम्स्क"
    "Asia/Novosibirsk" "नोवोसिबिर्स्क"
    "Asia/Barnaul" "बर्नौल"
    "Asia/Tomsk" "तोमसक"
    "Asia/Novokuznetsk" "नोवोकुझ्नेत्स्क"
    "Asia/Krasnoyarsk" "क्रास्नोयार्स्क"
    "Asia/Irkutsk" "ईर्कुत्स्क"
    "Asia/Chita" "चिता"
    "Asia/Yakutsk" "यकुत्स्क"
    "Asia/Khandyga" "खंदिगा"
    "Asia/Vladivostok" "व्लादिवोस्टोक"
    "Asia/Ust-Nera" "उस्त-नेरा"
    "Asia/Magadan" "मेगाडन"
    "Asia/Sakhalin" "साखालिन"
    "Asia/Srednekolymsk" "स्रेदनेकोलीम्स्क"
    "Asia/Kamchatka" "कॅमचाटका"
    "Asia/Anadyr" "एनाडीयर"
    "Asia/Riyadh" "रियाध"
    "Pacific/Guadalcanal" "ग्वाडलकनाल"
    "Africa/Khartoum" "खार्टुम"
    "Asia/Singapore" "सिंगापूर"
    "America/Paramaribo" "पारमरीबो"
    "Africa/Juba" "जुबा"
    "Africa/Sao_Tome" "साओ तोमे"
    "America/El_Salvador" "एल साल्वाडोर"
    "Asia/Damascus" "दमास्कस"
    "America/Grand_Turk" "ग्रँड टर्क"
    "Africa/Ndjamena" "इंजामेना"
    "Asia/Bangkok" "बँकॉक"
    "Asia/Dushanbe" "दुशान्बे"
    "Pacific/Fakaofo" "फाकाओफो"
    "Asia/Dili" "डिलि"
    "Asia/Ashgabat" "अश्गाबात"
    "Africa/Tunis" "टयूनिस"
    "Pacific/Tongatapu" "टोंगाटापू"
    "Europe/Istanbul" "इस्तंबूल"
    "Asia/Taipei" "तैपेई"
    "Europe/Kyiv" "कीव"
    "America/New_York" "न्यूयॉर्क"
    "America/Detroit" "डेट्रॉइट"
    "America/Kentucky/Louisville" "ल्युइसव्हिल"
    "America/Kentucky/Monticello" "माँटिसेलो, केंटुकी"
    "America/Indiana/Indianapolis" "इंडियानापोलिस"
    "America/Indiana/Vincennes" "विंसेनस, इंडियाना"
    "America/Indiana/Winamac" "विनमॅक, इंडियाना"
    "America/Indiana/Marengo" "मारेंगो, इंडियाना"
    "America/Indiana/Petersburg" "पीटर्सबर्ग, इंडियाना"
    "America/Indiana/Vevay" "वेवाय-इंडियाना"
    "America/Chicago" "शिकागो"
    "America/Indiana/Tell_City" "टेल सिटी, इंडियाना"
    "America/Indiana/Knox" "नॉक्स, इंडियाना"
    "America/Menominee" "मेनोमिनी"
    "America/North_Dakota/Center" "मध्य, उत्तर डकोटा"
    "America/North_Dakota/New_Salem" "न्यू सालेम, उत्तर डकोटा"
    "America/North_Dakota/Beulah" "ब्युलाह, उत्तर डकोटा"
    "America/Denver" "डेन्व्हर"
    "America/Boise" "बोइसी"
    "America/Phoenix" "फॉनिक्स"
    "America/Los_Angeles" "लॉस एंजेलिस"
    "America/Anchorage" "अँकरेज"
    "America/Juneau" "ज्यूनौ"
    "America/Sitka" "सिटका"
    "America/Metlakatla" "मेतलाकतला"
    "America/Yakutat" "यकुतात"
    "America/Nome" "नोम"
    "America/Adak" "अडॅक"
    "Pacific/Honolulu" "होनोलुलू"
    "America/Montevideo" "मोन्टेव्हिडियो"
    "Asia/Samarkand" "समरकंद"
    "Asia/Tashkent" "ताश्कंद"
    "America/Caracas" "कराकास"
    "Asia/Ho_Chi_Minh" "हो चि मिन्ह शहर"
    "Pacific/Efate" "इफेट"
    "Pacific/Apia" "अपिया"
    "Africa/Johannesburg" "जोहान्सबर्ग"
    "America/Antigua" "अँटिग्वा"
    "America/Anguilla" "अँग्विला"
    "Africa/Luanda" "लुआंडा"
    "Antarctica/McMurdo" "मॅक्मुरडो"
    "Antarctica/DumontDUrville" "ड्युमॉन्ट ड्युर्विल"
    "Antarctica/Syowa" "स्योवा"
    "America/Aruba" "अरुबा"
    "Europe/Mariehamn" "मरियेहामेन"
    "Europe/Sarajevo" "साराजेव्हो"
    "Africa/Ouagadougou" "वागडूगू"
    "Asia/Bahrain" "बहारिन"
    "Africa/Bujumbura" "बुजुंबुरा"
    "Africa/Porto-Novo" "पोर्टो-नोव्हो"
    "America/St_Barthelemy" "सेंट बार्थेलेमी"
    "Asia/Brunei" "ब्रुनेई"
    "America/Kralendijk" "क्रालेंदिजिक"
    "America/Nassau" "नसाऊ"
    "Africa/Gaborone" "गाबोरोन"
    "America/Blanc-Sablon" "ब्लांक सॅबलोन"
    "America/Atikokan" "अॅटिकोकन"
    "America/Creston" "क्रेस्टन"
    "Indian/Cocos" "कोकोस"
    "Africa/Kinshasa" "किन्शासा"
    "Africa/Lubumbashi" "लुबंबाशी"
    "Africa/Bangui" "बांगुई"
    "Africa/Brazzaville" "ब्राझाव्हिले"
    "Africa/Douala" "दोउआला"
    "America/Curacao" "क्युरासाओ"
    "Indian/Christmas" "ख्रिसमस"
    "Europe/Busingen" "बुसिंजेन"
    "Africa/Djibouti" "जिबौटी"
    "Europe/Copenhagen" "कोपेनहेगन"
    "America/Dominica" "डोमिनिका"
    "Africa/Asmara" "एस्मारा"
    "Africa/Addis_Ababa" "आदिस अबाबा"
    "Pacific/Chuuk" "चूक"
    "Pacific/Pohnpei" "पोनपेई"
    "Africa/Libreville" "लिबरव्हिल"
    "America/Grenada" "ग्रेनेडा"
    "Europe/Guernsey" "ग्वेर्नसे"
    "Africa/Accra" "अ‍ॅक्रा"
    "Africa/Banjul" "बंजुल"
    "Africa/Conakry" "कोनाक्रि"
    "America/Guadeloupe" "ग्वाडेलोउपे"
    "Africa/Malabo" "मलाबो"
    "Europe/Zagreb" "झॅग्रेब"
    "Europe/Isle_of_Man" "आयल ऑफ मॅन"
    "Atlantic/Reykjavik" "रेयक्जाविक"
    "Europe/Jersey" "जर्सी"
    "Asia/Phnom_Penh" "प्नोम पेन्ह"
    "Indian/Comoro" "कोमोरो"
    "America/St_Kitts" "सेंट किट्स"
    "Asia/Kuwait" "कुवेत"
    "America/Cayman" "केमन"
    "Asia/Vientiane" "व्हिएन्टाइन"
    "America/St_Lucia" "सेंट लुसिया"
    "Europe/Vaduz" "वडूझ"
    "Africa/Maseru" "मसेरु"
    "Europe/Luxembourg" "लक्झेंबर्ग"
    "Europe/Monaco" "मोनॅको"
    "Europe/Podgorica" "पॉडगोरिका"
    "America/Marigot" "मेरीगोट"
    "Indian/Antananarivo" "अंटानानारिवो"
    "Pacific/Majuro" "मजुरो"
    "Europe/Skopje" "स्कॉप्जे"
    "Africa/Bamako" "बामको"
    "Pacific/Saipan" "सैपान"
    "Africa/Nouakchott" "नुवाकसुत"
    "America/Montserrat" "माँन्टसेरात"
    "Africa/Blantyre" "ब्लँटायर"
    "Asia/Kuala_Lumpur" "क्वालालंपूर"
    "Africa/Niamey" "नियामे"
    "Europe/Amsterdam" "अ‍ॅमस्टरडॅम"
    "Europe/Oslo" "ऑस्लो"
    "Asia/Muscat" "मस्कत"
    "Indian/Reunion" "रियुनियन"
    "Africa/Kigali" "कीगाली"
    "Indian/Mahe" "माहे"
    "Europe/Stockholm" "स्टॉकहोम"
    "Atlantic/St_Helena" "सेंट. हेलेना"
    "Europe/Ljubljana" "लुब्लियाना"
    "Arctic/Longyearbyen" "लाँगइयरबीयेन"
    "Europe/Bratislava" "ब्रातिस्लाव्हा"
    "Africa/Freetown" "फ्रीटाउन"
    "Europe/San_Marino" "सॅन मरिनो"
    "Africa/Dakar" "डकर"
    "Africa/Mogadishu" "मोगादिशु"
    "America/Lower_Princes" "लोअर प्रिन्सस क्वार्टर"
    "Africa/Mbabane" "अंबाबाने"
    "Indian/Kerguelen" "करग्यूलेन"
    "Africa/Lome" "लोम"
    "America/Port_of_Spain" "पोर्ट ऑफ स्पेन"
    "Pacific/Funafuti" "फुनाफुती"
    "Africa/Dar_es_Salaam" "दार ए सलाम"
    "Africa/Kampala" "कंपाला"
    "Pacific/Midway" "मिडवे"
    "Pacific/Wake" "वेक"
    "Europe/Vatican" "व्हॅटिकन"
    "America/St_Vincent" "सेंट विन्सेंट"
    "America/Tortola" "टोर्टोला"
    "America/St_Thomas" "सेंट थॉमस"
    "Pacific/Wallis" "वालिस"
    "Asia/Aden" "एडेन"
    "Indian/Mayotte" "मायोट्टे"
    "Africa/Lusaka" "लुसाका"
    "Africa/Harare" "हरारे"
};

// `common/main/pa.xml`: 418 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const PA_GURU: &str = exemplar_cities! {
    "Europe/Andorra" "ਅੰਡੋਰਾ"
    "Asia/Dubai" "ਦੁਬਈ"
    "Asia/Kabul" "ਕਾਬੁਲ"
    "Europe/Tirane" "ਤਿਰਾਨੇ"
    "Asia/Yerevan" "ਯੇਰੇਵਨ"
    "Antarctica/Casey" "ਕਾਸੇ"
    "Antarctica/Davis" "ਡੇਵਿਸ"
    "Antarctica/Mawson" "ਮੌਸਨ"
    "Antarctica/Palmer" "ਪਾਮਰ"
    "Antarctica/Rothera" "ਰੋਥੇਰਾ"
    "Antarctica/Troll" "ਟਰੋਲ"
    "Antarctica/Vostok" "ਵੋਸਟੋਕ"
    "America/Argentina/Buenos_Aires" "ਬੂਈਨਸ ਆਇਰਸ"
    "America/Argentina/Cordoba" "ਕੋਰਡੋਬਾ"
    "America/Argentina/Salta" "ਸਾਲਟਾ"
    "America/Argentina/Jujuy" "ਜੂਜੁਏ"
    "America/Argentina/Tucuman" "ਟੁਕੁਮਨ"
    "America/Argentina/Catamarca" "ਕੈਟਾਮਾਰਕਾ"
    "America/Argentina/La_Rioja" "ਲਾ ਰਿਉਜਾ"
    "America/Argentina/San_Juan" "ਸੇਨ ਜੁਆਨ"
    "America/Argentina/Mendoza" "ਮੈਂਡੋਜ਼ਾ"
    "America/Argentina/San_Luis" "ਸੇਨ ਲੂਈਸ"
    "America/Argentina/Rio_Gallegos" "ਰਿਓ ਗੈਲੇਗੋਸ"
    "America/Argentina/Ushuaia" "ਉਸ਼ਵਾਇਆ"
    "Pacific/Pago_Pago" "ਪਾਗੋ ਪਾਗੋ"
    "Europe/Vienna" "ਵਿਆਨਾ"
    "Australia/Lord_Howe" "ਲੌਰਡ ਹੋਵੇ"
    "Antarctica/Macquarie" "ਮੈਕਕਵੈਰੀ"
    "Australia/Hobart" "ਹੋਬਾਰਟ"
    "Australia/Melbourne" "ਮੈਲਬੋਰਨ"
    "Australia/Sydney" "ਸਿਡਨੀ"
    "Australia/Broken_Hill" "ਬ੍ਰੋਕਨ ਹਿਲ"
    "Australia/Brisbane" "ਬ੍ਰਿਸਬੇਨ"
    "Australia/Lindeman" "ਲਿੰਡੇਮਨ"
    "Australia/Adelaide" "ਐਡੀਲੇਡ"
    "Australia/Darwin" "ਡਾਰਵਿਨ"
    "Australia/Perth" "ਪਰਥ"
    "Australia/Eucla" "ਯੂਕਲਾ"
    "Asia/Baku" "ਬਾਕੂ"
    "America/Barbados" "ਬਾਰਬਾਡੋਸ"
    "Asia/Dhaka" "ਢਾਕਾ"
    "Europe/Brussels" "ਬਰੱਸਲਜ"
    "Europe/Sofia" "ਸੋਫੀਆ"
    "Atlantic/Bermuda" "ਬਰਮੂਡਾ"
    "America/La_Paz" "ਲਾ ਪਾਜ਼"
    "America/Noronha" "ਨੌਰੋਨਹਾ"
    "America/Belem" "ਬੇਲੇਮ"
    "America/Fortaleza" "ਫੋਰਟਾਲੇਜ਼ਾ"
    "America/Recife" "ਰੇਸੀਫੇ"
    "America/Araguaina" "ਆਰਗੁਆਇਨਾ"
    "America/Maceio" "ਮੈਸੀਓ"
    "America/Bahia" "ਬਾਹੀਆ"
    "America/Sao_Paulo" "ਸਾਓ ਪੌਲੋ"
    "America/Campo_Grande" "ਕੈਂਪੋ ਗ੍ਰਾਂਡੇ"
    "America/Cuiaba" "ਕਯੁਏਬਾ"
    "America/Santarem" "ਸੇਂਟਾਰਮ"
    "America/Porto_Velho" "ਪੋਰਟੋ ਵੇਲ੍ਹੋ"
    "America/Boa_Vista" "ਬੋਆ ਵਿਸਟਾ"
    "America/Manaus" "ਮਨੌਸ"
    "America/Eirunepe" "ਯੁਰੂਨੀਪੇ"
    "America/Rio_Branco" "ਰੀਓ ਬ੍ਰਾਂਕੋ"
    "Asia/Thimphu" "ਥਿੰਫੂ"
    "Europe/Minsk" "ਮਿੰਸਕ"
    "America/Belize" "ਬੇਲੀਜ਼"
    "America/St_Johns" "ਸੇਂਟ ਜੌਹਨਸ"
    "America/Halifax" "ਹੈਲੀਫੈਕਸ"
    "America/Glace_Bay" "ਗਲੇਸ ਬੇ"
    "America/Moncton" "ਮੋਂਕਟਨ"
    "America/Goose_Bay" "ਗੂਜ਼ ਬੇ"
    "America/Toronto" "ਟੋਰਾਂਟੋ"
    "America/Iqaluit" "ਇਕਾਲੁਈਟ"
    "America/Winnipeg" "ਵਿਨੀਪੈਗ"
    "America/Resolute" "ਰੈਜ਼ੋਲਿਊਟ"
    "America/Rankin_Inlet" "ਰੈਂਕਿਨ ਇਨਲੈਟ"
    "America/Regina" "ਰੈਜੀਨਾ"
    "America/Swift_Current" "ਸਵਿਫਟ ਕਰੰਟ"
    "America/Edmonton" "ਐਡਮੋਂਟਨ"
    "America/Cambridge_Bay" "ਕੈਮਬ੍ਰਿਜ ਬੇ"
    "America/Inuvik" "ਇਨੁਵਿਕ"
    "America/Vancouver" "ਵੈਨਕੂਵਰ"
    "America/Dawson_Creek" "ਡੌਅਸਨ ਕ੍ਰੀਕ"
    "America/Fort_Nelson" "ਫੋਰਟ ਨੈਲਸਨ"
    "America/Whitehorse" "ਵਾਈਟਹੌਰਸ"
    "America/Dawson" "ਡੌਅਸਨ"
    "Europe/Zurich" "ਜਿਊਰਿਖ"
    "Africa/Abidjan" "ਅਬੀਦਜਾਨ"
    "Pacific/Rarotonga" "ਰਾਰੋਟੋਂਗਾ"
    "America/Santiago" "ਸੇਂਟੀਆਗੋ"
    "America/Coyhaique" "ਕੋਹੇਕੇ"
    "America/Punta_Arenas" "ਪੰਟਾ ਅਰੇਨਸ"
    "Pacific/Easter" "ਈਸਟਰ"
    "Asia/Shanghai" "ਸ਼ੰਘਾਈ"
    "Asia/Urumqi" "ਊਰੂਮਕੀ"
    "America/Bogota" "ਬੋਗੋਟਾ"
    "America/Costa_Rica" "ਕੋਸਟਾ ਰੀਕਾ"
    "America/Havana" "ਹਵਾਨਾ"
    "Atlantic/Cape_Verde" "ਕੇਪ ਵਰਡ"
    "Asia/Nicosia" "ਨਿਕੋਸੀਆ"
    "Asia/Famagusta" "ਫਾਮਾਗੁਸਟਾ"
    "Europe/Prague" "ਪ੍ਰਾਗ"
    "Europe/Berlin" "ਬਰਲਿਨ"
    "America/Santo_Domingo" "ਸੇਂਟੋ ਡੋਮਿੰਗੋ"
    "Africa/Algiers" "ਅਲਜੀਅਰਸ"
    "America/Guayaquil" "ਗੁਆਇਕਵਿਲ"
    "Pacific/Galapagos" "ਗਲਪੇਗੋਸ"
    "Europe/Tallinn" "ਟੱਲਿਨ"
    "Africa/Cairo" "ਕੈਰੋ"
    "Africa/El_Aaiun" "ਅਲ ਅਯੂਨ"
    "Europe/Madrid" "ਮੈਡ੍ਰਿਡ"
    "Africa/Ceuta" "ਸੀਊਟਾ"
    "Atlantic/Canary" "ਕੇਨੇਰੀ"
    "Europe/Helsinki" "ਹੇਲਸਿੰਕੀ"
    "Pacific/Fiji" "ਫ਼ਿਜੀ"
    "Atlantic/Stanley" "ਸਟੇਨਲੀ"
    "Pacific/Kosrae" "ਕੋਸ੍ਰਾਏ"
    "Atlantic/Faroe" "ਫੈਰੋ"
    "Europe/Paris" "ਪੈਰਿਸ"
    "Europe/London" "ਲੰਡਨ"
    "Asia/Tbilisi" "ਟਬਿਲਿਸੀ"
    "America/Cayenne" "ਕੇਯੇਨੇ"
    "Europe/Gibraltar" "ਜਿਬਰਾਲਟਰ"
    "America/Nuuk" "ਨੂਕ"
    "America/Danmarkshavn" "ਡੈਨਮਾਰਕਸ਼ੌਨ"
    "America/Scoresbysund" "ਇੱਟੋਕੋਰਟੂਰਮੀਟ"
    "America/Thule" "ਥੁਲੇ"
    "Europe/Athens" "ਏਥਨਸ"
    "Atlantic/South_Georgia" "ਦੱਖਣੀ ਜਾਰਜੀਆ"
    "America/Guatemala" "ਗੁਆਟੇਮਾਲਾ"
    "Pacific/Guam" "ਗੁਆਮ"
    "Africa/Bissau" "ਬਿਸਾਉ"
    "America/Guyana" "ਗੁਆਨਾ"
    "Asia/Hong_Kong" "ਹਾਂਗ ਕਾਂਗ"
    "America/Tegucigalpa" "ਟੇਗੁਸੀਗਲਪਾ"
    "America/Port-au-Prince" "ਪੋਰਟ-ਔ-ਪ੍ਰਿੰਸ"
    "Europe/Budapest" "ਬੁਡਾਪੈਸਟ"
    "Asia/Jakarta" "ਜਕਾਰਤਾ"
    "Asia/Pontianak" "ਪੌਂਟੀਆਨਾਕ"
    "Asia/Makassar" "ਮਕਸਾਰ"
    "Asia/Jayapura" "ਜਯਾਪੁਰਾ"
    "Europe/Dublin" "ਡਬਲਿਨ"
    "Asia/Jerusalem" "ਜੇਰੂਸਲਮ"
    "Asia/Kolkata" "ਕੋਲਕਾਤਾ"
    "Indian/Chagos" "ਚਾਗੋਸ"
    "Asia/Baghdad" "ਬਗਦਾਦ"
    "Asia/Tehran" "ਤੇਹਰਾਨ"
    "Europe/Rome" "ਰੋਮ"
    "America/Jamaica" "ਜਮਾਇਕਾ"
    "Asia/Amman" "ਅਮਾਨ"
    "Asia/Tokyo" "ਟੋਕੀਓ"
    "Africa/Nairobi" "ਨੈਰੋਬੀ"
    "Asia/Bishkek" "ਬਿਸ਼ਕੇਕ"
    "Pacific/Tarawa" "ਟਾਰਾਵਾ"
    "Pacific/Kanton" "ਕੈਂਟੋਨ"
    "Pacific/Kiritimati" "ਕਿਰਿਤਿਮਤੀ"
    "Asia/Pyongyang" "ਪਯੋਂਗਯਾਂਗ"
    "Asia/Seoul" "ਸਿਉਲ"
    "Asia/Almaty" "ਅਲਮੇਟੀ"
    "Asia/Qyzylorda" "ਕਿਜ਼ੀਲੋਰਡਾ"
    "Asia/Qostanay" "ਕੋਸਤਾਨਾਏ"
    "Asia/Aqtobe" "ਅਕਤੋਬੇ"
    "Asia/Aqtau" "ਅਕਤੌ"
    "Asia/Atyrau" "ਏਤੇਰਾਓ"
    "Asia/Oral" "ਓਰਲ"
    "Asia/Beirut" "ਬੈਰੂਤ"
    "Asia/Colombo" "ਕੋਲੰਬੋ"
    "Africa/Monrovia" "ਮੋਨਰੋਵੀਆ"
    "Europe/Vilnius" "ਵਿਲਨਿਅਸ"
    "Europe/Riga" "ਰਿਗਾ"
    "Africa/Tripoli" "ਤ੍ਰਿਪੋਲੀ"
    "Africa/Casablanca" "ਕਾਸਾਬਲਾਂਕਾ"
    "Europe/Chisinau" "ਚਿਸਿਨੌ"
    "Pacific/Kwajalein" "ਕਵਾਜਾਲੀਨ"
    "Asia/Yangon" "ਰੰਗੂਨ"
    "Asia/Ulaanbaatar" "ਉਲਾਨਬਾਤਰ"
    "Asia/Hovd" "ਹੋਵਡ"
    "Asia/Macau" "ਮਕਾਉ"
    "America/Martinique" "ਮਾਰਟੀਨਿਕ"
    "Europe/Malta" "ਮਾਲਟਾ"
    "Indian/Mauritius" "ਮੌਰਿਸ਼ਸ"
    "Indian/Maldives" "ਮਾਲਦੀਵ"
    "America/Mexico_City" "ਮੈਕਸੀਕੋ ਸਿਟੀ"
    "America/Cancun" "ਕੈਨਕੁਨ"
    "America/Merida" "ਮੇਰਿਡਾ"
    "America/Monterrey" "ਮੋਨਟੇਰੀ"
    "America/Matamoros" "ਮਾਟਾਮੋਰਸ"
    "America/Chihuahua" "ਚਿਹੁਆਹੁਆ"
    "America/Ciudad_Juarez" "ਸਿਉਡਾਡ ਹੁਆਰੇਜ਼"
    "America/Ojinaga" "ਓਜੀਨਾਗਾ"
    "America/Mazatlan" "ਮਜ਼ੇਤਲਾਨ"
    "America/Bahia_Banderas" "ਬਾਹੀਆ ਬਾਂਦੇਰਸ"
    "America/Hermosillo" "ਹਰਮੋਸਿੱਲੋ"
    "America/Tijuana" "ਟਿਜੂਆਨਾ"
    "Asia/Kuching" "ਕੁਚਿੰਗ"
    "Africa/Maputo" "ਮਾਪੁਟੋ"
    "Africa/Windhoek" "ਵਿੰਡਹੋਇਕ"
    "Pacific/Noumea" "ਨੌਮਿਆ"
    "Pacific/Norfolk" "ਨੋਰਫੌਕ"
    "Africa/Lagos" "ਲਾਗੋਸ"
    "America/Managua" "ਮਨਾਗੁਆ"
    "Asia/Kathmandu" "ਕਾਠਮਾਂਡੂ"
    "Pacific/Nauru" "ਨਾਉਰੂ"
    "Pacific/Niue" "ਨਿਯੂ"
    "Pacific/Auckland" "ਆਕਲੈਂਡ"
    "Pacific/Chatham" "ਚੈਥਮ"
    "America/Panama" "ਪਨਾਮਾ"
    "America/Lima" "ਲੀਮਾ"
    "Pacific/Tahiti" "ਤਹਿਤੀ"
    "Pacific/Marquesas" "ਮਾਰਕਿਸਾਸ"
    "Pacific/Gambier" "ਗੈਂਬੀਅਰ"
    "Pacific/Port_Moresby" "ਪੋਰਟ ਮੋਰੇਸਬੀ"
    "Pacific/Bougainville" "ਬੋਗਨਵਿਲੇ"
    "Asia/Manila" "ਮਨੀਲਾ"
    "Asia/Karachi" "ਕਰਾਚੀ"
    "Europe/Warsaw" "ਵਾਰਸਾਅ"
    "America/Miquelon" "ਮਿਕੇਲਨ"
    "Pacific/Pitcairn" "ਪਿਟਕੈਰਨ"
    "America/Puerto_Rico" "ਪਿਊਰਟੋ ਰੀਕੋ"
    "Asia/Gaza" "ਗਾਜ਼ਾ"
    "Asia/Hebron" "ਹੇਬਰਾਨ"
    "Europe/Lisbon" "ਲਿਸਬਨ"
    "Atlantic/Madeira" "ਮਡੀਅਰਾ"
    "Atlantic/Azores" "ਅਜੋਰੇਸ"
    "Pacific/Palau" "ਪਲਾਉ"
    "America/Asuncion" "ਐਸੁੰਕੀਅਨ"
    "Asia/Qatar" "ਕਤਰ"
    "Europe/Bucharest" "ਬੂਕਾਰੈਸਟ"
    "Europe/Belgrade" "ਬੈਲਗ੍ਰੇਡ"
    "Europe/Kaliningrad" "ਕਲੀਨਿੰਗ੍ਰੇਡ"
    "Europe/Moscow" "ਮਾਸਕੋ"
    "Europe/Simferopol" "ਸਿਮਫਰੋਪੋਲ"
    "Europe/Kirov" "ਕੀਰੋਵ"
    "Europe/Volgograd" "ਵੋਲਗੋਗ੍ਰੇਡ"
    "Europe/Astrakhan" "ਆਸਟ੍ਰਾਖਾਨ"
    "Europe/Saratov" "ਸੈਰਾਟੋਵ"
    "Europe/Ulyanovsk" "ਯੁਲਿਆਨੋਸਕ"
    "Europe/Samara" "ਸਮਾਰਾ"
    "Asia/Yekaterinburg" "ਯਕੇਤਰਿਨਬਰਗ"
    "Asia/Omsk" "ਓਮਸਕ"
    "Asia/Novosibirsk" "ਨੋਵੋਸਿਬੀਰਸਕ"
    "Asia/Barnaul" "ਬਰਨੌਲ"
    "Asia/Tomsk" "ਟੋਮਸਕ"
    "Asia/Novokuznetsk" "ਨੋਵੋਕੁਜ਼ਨੇਟਸਕ"
    "Asia/Krasnoyarsk" "ਕਰੈਸਨੇਜਰਸ"
    "Asia/Irkutsk" "ਇਰਕੁਤਸਕ"
    "Asia/Chita" "ਚਿਤਾ"
    "Asia/Yakutsk" "ਯਕੁਤਸਕ"
    "Asia/Khandyga" "ਖਾਨਡਿਗਾ"
    "Asia/Vladivostok" "ਵਲਾਦੀਵੋਸਤਕ"
    "Asia/Ust-Nera" "ਉਸਤ-ਨੇਰਾ"
    "Asia/Magadan" "ਮੈਗੇਡਨ"
    "Asia/Sakhalin" "ਸਖਲੀਨ"
    "Asia/Srednekolymsk" "ਸਰਿਡਨੀਕੋਲਿਸਕ"
    "Asia/Kamchatka" "ਕਮਚਟਕਾ"
    "Asia/Anadyr" "ਐਨਾਡਾਇਰ"
    "Asia/Riyadh" "ਰਿਆਧ"
    "Pacific/Guadalcanal" "ਗੁਆਡਾਕੇਨਲ"
    "Africa/Khartoum" "ਖਾਰਟੌਮ"
    "Asia/Singapore" "ਸਿੰਗਾਪੁਰ"
    "America/Paramaribo" "ਪੈਰਾਮਰੀਬੋ"
    "Africa/Juba" "ਜੂਬਾ"
    "Africa/Sao_Tome" "ਸਾਓ ਟੋਮ"
    "America/El_Salvador" "ਅਲ ਸਲਵਾਡੋਰ"
    "Asia/Damascus" "ਡੈਮਸਕਸ"
    "America/Grand_Turk" "ਗਰਾਂਡ ਤੁਰਕ"
    "Africa/Ndjamena" "ਐਂਜਾਮੇਨਾ"
    "Asia/Bangkok" "ਬੈਂਕਾਕ"
    "Asia/Dushanbe" "ਦੁਸ਼ਾਂਬੇ"
    "Pacific/Fakaofo" "ਫਕਾਉਫੋ"
    "Asia/Dili" "ਡਿਲੀ"
    "Asia/Ashgabat" "ਅਸ਼ਗਾਬਾਟ"
    "Africa/Tunis" "ਟੁਨਿਸ"
    "Pacific/Tongatapu" "ਟੋਂਗਾਟਾਪੂ"
    "Europe/Istanbul" "ਇਸਤਾਂਬੁਲ"
    "Asia/Taipei" "ਤੈਪਈ"
    "Europe/Kyiv" "ਕੀਵ"
    "America/New_York" "ਨਿਊ ਯਾਰਕ"
    "America/Detroit" "ਡਿਟਰੋਇਟ"
    "America/Kentucky/Louisville" "ਲੁਈਸਵਿਲੇ"
    "America/Kentucky/Monticello" "ਮੋਂਟੀਸੈਲੋ, ਕੈਂਟਕੀ"
    "America/Indiana/Indianapolis" "ਇੰਡੀਆਨਾਪੋਲਿਸ"
    "America/Indiana/Vincennes" "ਵਿੰਸੇਨੇਸ, ਇੰਡੀਆਨਾ"
    "America/Indiana/Winamac" "ਵਿਨਮੈਕ, ਇੰਡੀਆਨਾ"
    "America/Indiana/Marengo" "ਮਾਰੇਂਗੋ, ਇੰਡੀਆਨਾ"
    "America/Indiana/Petersburg" "ਪੀਟਰਸਬਰਗ, ਇੰਡੀਆਨਾ"
    "America/Indiana/Vevay" "ਵੇਵੇ, ਇੰਡੀਆਨਾ"
    "America/Chicago" "ਸ਼ਿਕਾਗੋ"
    "America/Indiana/Tell_City" "ਟੈਲ ਸਿਟੀ, ਇੰਡੀਆਨਾ"
    "America/Indiana/Knox" "ਨੋਕਸ, ਇੰਡੀਆਨਾ"
    "America/Menominee" "ਮੈਨੋਮਿਨੀ"
    "America/North_Dakota/Center" "ਸੇਂਟਰ, ਉੱਤਰੀ ਡਕੋਟਾ"
    "America/North_Dakota/New_Salem" "ਨਿਊ ਸਲੇਮ, ਉੱਤਰੀ ਡਕੋਟਾ"
    "America/North_Dakota/Beulah" "ਬਿਉਲਾ, ਉੱਤਰੀ ਡਕੋਟਾ"
    "America/Denver" "ਡੇਨਵਰ"
    "America/Boise" "ਬੋਇਸ"
    "America/Phoenix" "ਫਿਨਿਕਸ"
    "America/Los_Angeles" "ਲਾਸ ਐਂਜਲਸ"
    "America/Anchorage" "ਐਂਕਰੇਜ"
    "America/Juneau" "ਜਯੂਨੋ"
    "America/Sitka" "ਸਿਟਕਾ"
    "America/Metlakatla" "ਮੇਟਲਾਕਾਟਲਾ"
    "America/Yakutat" "ਯਕੁਤਤ"
    "America/Nome" "ਨੋਮ"
    "America/Adak" "ਏਡਕ"
    "Pacific/Honolulu" "ਹੋਨੋਲੁਲੂ"
    "America/Montevideo" "ਮੋਂਟੇਵੀਡੀਓ"
    "Asia/Samarkand" "ਸਮਰਕੰਦ"
    "Asia/Tashkent" "ਤਾਸ਼ਕੰਦ"
    "America/Caracas" "ਕੈਰਾਕਾਸ"
    "Asia/Ho_Chi_Minh" "ਹੋ ਚੀ ਮਿਨ੍ਹ ਸਿਟੀ"
    "Pacific/Efate" "ਇਫੇਟ"
    "Pacific/Apia" "ਐਪੀਆ"
    "Africa/Johannesburg" "ਜੋਹਨਸਬਰਗ"
    "America/Antigua" "ਐਂਟੀਗੁਆ"
    "America/Anguilla" "ਅੰਗੁਇਲਾ"
    "Africa/Luanda" "ਲੁਆਂਡਾ"
    "Antarctica/McMurdo" "ਮੈਕਮੁਰਡੋ"
    "Antarctica/DumontDUrville" "ਡਿਉਮੋਂਟ ਡਿਉਰਵਿਲੇ"
    "Antarctica/Syowa" "ਸਵੋਯਾ"
    "America/Aruba" "ਅਰੂਬਾ"
    "Europe/Mariehamn" "ਮਾਰੀਏਹਾਮੇਨ"
    "Europe/Sarajevo" "ਸਾਰਾਜੇਵੋ"
    "Africa/Ouagadougou" "ਉਆਗਾਡੂਗੂ"
    "Asia/Bahrain" "ਬਹਿਰੀਨ"
    "Africa/Bujumbura" "ਬੁਜੁੰਬੁਰਾ"
    "Africa/Porto-Novo" "ਪੋਰਟੋ-ਨੋਵੋ"
    "America/St_Barthelemy" "ਸੇਂਟ ਬਾਰਥੇਲੇਮੀ"
    "Asia/Brunei" "ਬਰੂਨੇਈ"
    "America/Kralendijk" "ਕ੍ਰਾਲੇਂਦਿਜਕ"
    "America/Nassau" "ਨਾਸਾਓ"
    "Africa/Gaborone" "ਗਾਬੋਰੋਨ"
    "America/Blanc-Sablon" "ਬਲੈਂਕ-ਸੈਬਲਾਨ"
    "America/Atikokan" "ਐਟੀਕੋਕਨ"
    "America/Creston" "ਕ੍ਰੈਸਟਨ"
    "Indian/Cocos" "ਕੋਕੋਜ਼"
    "Africa/Kinshasa" "ਕਿੰਸ਼ਾਸਾ"
    "Africa/Lubumbashi" "ਲੁਬੁਮਬਾਸ਼ੀ"
    "Africa/Bangui" "ਬਾਂਗੁਈ"
    "Africa/Brazzaville" "ਬ੍ਰਾਜ਼ਾਵਿਲੇ"
    "Africa/Douala" "ਡੌਆਲਾ"
    "America/Curacao" "ਕੁਰਾਕਾਓ"
    "Indian/Christmas" "ਕ੍ਰਿਸਮਸ"
    "Europe/Busingen" "ਬੁਸਿੰਜੇਨ"
    "Africa/Djibouti" "ਜਿਬੂਤੀ"
    "Europe/Copenhagen" "ਕੋਪਨਹੇਗਨ"
    "America/Dominica" "ਡੋਮੀਨਿਕਾ"
    "Africa/Asmara" "ਅਸਮਾਰਾ"
    "Africa/Addis_Ababa" "ਐਡਿਸ ਅਬਾਬਾ"
    "Pacific/Chuuk" "ਚੂਕ"
    "Pacific/Pohnpei" "ਪੋਹਨਪੇਈ"
    "Africa/Libreville" "ਲਿਬਰਵਿਲੇ"
    "America/Grenada" "ਗ੍ਰੇਨਾਡਾ"
    "Europe/Guernsey" "ਗਰਨਜੀ"
    "Africa/Accra" "ਅੱਕਰਾ"
    "Africa/Banjul" "ਬਾਂਜੁਲ"
    "Africa/Conakry" "ਕੋਨੇਕਰੀ"
    "America/Guadeloupe" "ਗੁਆਡੇਲੋਪ"
    "Africa/Malabo" "ਮਾਲਾਬੋ"
    "Europe/Zagreb" "ਜ਼ਗਰੇਬ"
    "Europe/Isle_of_Man" "ਆਇਲ ਆਫ ਮੈਨ"
    "Atlantic/Reykjavik" "ਰੇਕਜਾਵਿਕ"
    "Europe/Jersey" "ਜਰਸੀ"
    "Asia/Phnom_Penh" "ਫਨੋਮ ਪੇਨਹ"
    "Indian/Comoro" "ਕੋਮੋਰੋ"
    "America/St_Kitts" "ਸੇਂਟ ਕਿਟਸ"
    "Asia/Kuwait" "ਕੁਵੈਤ"
    "America/Cayman" "ਕੇਮੈਨ"
    "Asia/Vientiane" "ਵਾਏਨਟਿਆਨੇ"
    "America/St_Lucia" "ਸੇਂਟ ਲੁਸੀਆ"
    "Europe/Vaduz" "ਵਾਡੁਜ਼"
    "Africa/Maseru" "ਮਸੇਰੂ"
    "Europe/Luxembourg" "ਲਕਜ਼ਮਬਰਗ"
    "Europe/Monaco" "ਮੋਨਾਕੋ"
    "Europe/Podgorica" "ਪੋਡਗੋਰੀਕਾ"
    "America/Marigot" "ਮੈਰੀਗੋਟ"
    "Indian/Antananarivo" "ਅੰਟਾਨਨੇਰਿਵੋ"
    "Pacific/Majuro" "ਮੇਜੁਰੋ"
    "Europe/Skopje" "ਸਕੋਪਜੇ"
    "Africa/Bamako" "ਬਮੇਕੋ"
    "Pacific/Saipan" "ਸੈਪਾਨ"
    "Africa/Nouakchott" "ਨੌਆਕਸ਼ਾਟ"
    "America/Montserrat" "ਮੋਂਟਸੇਰਾਤ"
    "Africa/Blantyre" "ਬਲੰਟਾਇਰ"
    "Asia/Kuala_Lumpur" "ਕੁਆਲਾਲੰਪੁਰ"
    "Africa/Niamey" "ਨਿਆਮੇ"
    "Europe/Amsterdam" "ਐਮਸਟਰਡਮ"
    "Europe/Oslo" "ਓਸਲੋ"
    "Asia/Muscat" "ਮਸਕਟ"
    "Indian/Reunion" "ਰਿਯੂਨੀਅਨ"
    "Africa/Kigali" "ਕਿਗਾਲੀ"
    "Indian/Mahe" "ਮਾਹੇ"
    "Europe/Stockholm" "ਸਟਾਕਹੋਮ"
    "Atlantic/St_Helena" "ਸੇਂਟ ਹੇਲੇਨਾ"
    "Europe/Ljubljana" "ਲਜੁਬਲਜਾਨਾ"
    "Arctic/Longyearbyen" "ਲੋਂਗਈਅਰਬਾਇਨ"
    "Europe/Bratislava" "ਬ੍ਰਾਟਿਸਲਾਵਾ"
    "Africa/Freetown" "ਫਰੀਟਾਉਨ"
    "Europe/San_Marino" "ਸੈਨ ਮਰੀਨੋ"
    "Africa/Dakar" "ਡਕਾਰ"
    "Africa/Mogadishu" "ਮੋਗਾਦਿਸ਼ੂ"
    "America/Lower_Princes" "ਲੋਅਰ ਪ੍ਰਿੰਸ’ਸ ਕਵਾਰਟਰ"
    "Africa/Mbabane" "ਏਮਬਾਬਾਨੇ"
    "Indian/Kerguelen" "ਕਰਗਯੂਲੇਨ"
    "Africa/Lome" "ਲੋਮ"
    "America/Port_of_Spain" "ਪੋਰਟ ਔਫ ਸਪੇਨ"
    "Pacific/Funafuti" "ਫੁਨਾਫੁਟੀ"
    "Africa/Dar_es_Salaam" "ਦਾਰ ਏਸ ਸਲਾਮ"
    "Africa/Kampala" "ਕੰਪਾਲਾ"
    "Pacific/Midway" "ਮਿਡਵੇ"
    "Pacific/Wake" "ਵੇਕ"
    "Europe/Vatican" "ਵੈਟਿਕਨ"
    "America/St_Vincent" "ਸੇਂਟ ਵਿਨਸੇਂਟ"
    "America/Tortola" "ਟੋਰਟੋਲਾ"
    "America/St_Thomas" "ਸੇਂਟ ਥੋਮਸ"
    "Pacific/Wallis" "ਵਾਲਿਸ"
    "Asia/Aden" "ਅਡੇਨ"
    "Indian/Mayotte" "ਮਾਯੋਟੀ"
    "Africa/Lusaka" "ਲੁਸਾਕਾ"
    "Africa/Harare" "ਹਰਾਰੇ"
};

// `common/main/pcm.xml`: 399 of the 418 zones named, 18 inherited.
#[cfg(feature = "localized-exemplar-cities")]
const PCM: &str = exemplar_cities! {
    "Europe/Andorra" "Andọ́ra"
    "Asia/Dubai" inherited
    "Asia/Kabul" inherited
    "Europe/Tirane" "Tiránẹ"
    "Asia/Yerevan" "Yẹrẹ́van"
    "Antarctica/Casey" "Kési"
    "Antarctica/Davis" "Dévis"
    "Antarctica/Mawson" "Mọ́sọn"
    "Antarctica/Palmer" "Páma"
    "Antarctica/Rothera" "Rotẹ́ra"
    "Antarctica/Troll" "Trol"
    "Antarctica/Vostok" "Vọ́stọk"
    "America/Argentina/Buenos_Aires" "Buẹnos Aírẹs"
    "America/Argentina/Cordoba" "Kórdoba"
    "America/Argentina/Salta" "Sálta"
    "America/Argentina/Jujuy" "Huhui"
    "America/Argentina/Tucuman" "Túkúman"
    "America/Argentina/Catamarca" "Katamáka"
    "America/Argentina/La_Rioja" "La Riókha"
    "America/Argentina/San_Juan" "Sán Hwán"
    "America/Argentina/Mendoza" "Mẹndóza"
    "America/Argentina/San_Luis" "Sán Luis"
    "America/Argentina/Rio_Gallegos" "Rió Galẹ́gọs"
    "America/Argentina/Ushuaia" "Usuáya"
    "Pacific/Pago_Pago" "Págo Págo"
    "Europe/Vienna" "Viẹ́na"
    "Australia/Lord_Howe" "Lọd Haú"
    "Antarctica/Macquarie" "Makwuéí"
    "Australia/Hobart" "Hóbat"
    "Australia/Melbourne" "Mẹ́lbọn"
    "Australia/Sydney" "Sídni"
    "Australia/Broken_Hill" "Brókún Hil"
    "Australia/Brisbane" "Brísben"
    "Australia/Lindeman" "Líndẹman"
    "Australia/Adelaide" "Adleid"
    "Australia/Darwin" "Dárwin"
    "Australia/Perth" "Pẹrt"
    "Australia/Eucla" "Yúkla"
    "Asia/Baku" "Báku"
    "America/Barbados" "Barbédọs"
    "Asia/Dhaka" "Dáka"
    "Europe/Brussels" "Brúsuls"
    "Europe/Sofia" "Sofía"
    "Atlantic/Bermuda" "Bẹmiúda"
    "America/La_Paz" inherited
    "America/Noronha" "Nọrónia"
    "America/Belem" "Bẹlẹm"
    "America/Fortaleza" "Fọtalẹ́za"
    "America/Recife" "Rẹsífẹ"
    "America/Araguaina" "Aragwuaína"
    "America/Maceio" "Masẹ́io"
    "America/Bahia" "Bahía"
    "America/Sao_Paulo" "Sao Paúlo"
    "America/Campo_Grande" "Kampó Grándẹ"
    "America/Cuiaba" "Kúyábaa"
    "America/Santarem" "Santarẹm"
    "America/Porto_Velho" "Pọto Vẹ́lho"
    "America/Boa_Vista" "Bóa Vísta"
    "America/Manaus" "Manáus"
    "America/Eirunepe" "Ẹirunẹpẹ"
    "America/Rio_Branco" "Rió Bránko"
    "Asia/Thimphu" "Tímfu"
    "Europe/Minsk" inherited
    "America/Belize" "Bẹliz"
    "America/St_Johns" "Sent Jọn"
    "America/Halifax" "Hálífaks"
    "America/Glace_Bay" "Glás Bè"
    "America/Moncton" "Mọ́nktọn"
    "America/Goose_Bay" "Gúz Bè"
    "America/Toronto" "Torónto"
    "America/Iqaluit" "Ikáluit"
    "America/Winnipeg" "Wínípẹg"
    "America/Resolute" "Rẹ́zólut"
    "America/Rankin_Inlet" "Ránkín Ínlẹt"
    "America/Regina" "Rẹjína"
    "America/Swift_Current" "Swíft Kọ́rẹnt"
    "America/Edmonton" "Ẹ́dmọ́ntọn"
    "America/Cambridge_Bay" "Kémbríj Bè"
    "America/Inuvik" "Inúvik"
    "America/Vancouver" "Vankúva"
    "America/Dawson_Creek" "Dọ́sọn Krik"
    "America/Fort_Nelson" "Fọt Nẹ́lson"
    "America/Whitehorse" "Waíthọs"
    "America/Dawson" "Dọ́sọn"
    "Europe/Zurich" "Zúrik"
    "Africa/Abidjan" "Ábijan"
    "Pacific/Rarotonga" "Raratónga"
    "America/Santiago" "Santiágo"
    "America/Coyhaique" inherited
    "America/Punta_Arenas" "Púntá Arẹ́nas"
    "Pacific/Easter" "Ísta"
    "Asia/Shanghai" "Shánghai"
    "Asia/Urumqi" "Yurọ́mki"
    "America/Bogota" inherited
    "America/Costa_Rica" "Kósta Ríka"
    "America/Havana" "Havána"
    "Atlantic/Cape_Verde" "Kép Vẹd"
    "Asia/Nicosia" "Nikosia"
    "Asia/Famagusta" "Fagústa"
    "Europe/Prague" "Prag"
    "Europe/Berlin" "Bẹlin"
    "America/Santo_Domingo" "Sántó Domíngo"
    "Africa/Algiers" "Aljíẹz"
    "America/Guayaquil" "Guáyakil"
    "Pacific/Galapagos" "Galápágọs"
    "Europe/Tallinn" "Tálin"
    "Africa/Cairo" "Kaíro"
    "Africa/El_Aaiun" "Ẹl Aiun"
    "Europe/Madrid" inherited
    "Africa/Ceuta" "Sẹúta"
    "Atlantic/Canary" "Kenerí"
    "Europe/Helsinki" "Hẹlsínki"
    "Pacific/Fiji" "Fíji"
    "Atlantic/Stanley" "Stánli"
    "Pacific/Kosrae" "Kọ́sraẹ"
    "Atlantic/Faroe" "Fáróis"
    "Europe/Paris" "Páris"
    "Europe/London" "Lọ́ndọn"
    "Asia/Tbilisi" "Tiblísi"
    "America/Cayenne" "Kayẹn"
    "Europe/Gibraltar" "Jibrọ́lta"
    "America/Nuuk" inherited
    "America/Danmarkshavn" "Danmákshávun"
    "America/Scoresbysund" "Itókotúrmit"
    "America/Thule" "Túli"
    "Europe/Athens" "Átẹns"
    "Atlantic/South_Georgia" "Saút Jọ́jia"
    "America/Guatemala" "Guátẹmála"
    "Pacific/Guam" inherited
    "Africa/Bissau" "Bisau"
    "America/Guyana" "Gayána"
    "Asia/Hong_Kong" "Họng Kọng"
    "America/Tegucigalpa" "Tẹgúsigálpa"
    "America/Port-au-Prince" "Pọt-o-Prins"
    "Europe/Budapest" "Búdápẹst"
    "Asia/Jakarta" "Jakáta"
    "Asia/Pontianak" "Pọntiának"
    "Asia/Makassar" "Makása"
    "Asia/Jayapura" "Jayapúra"
    "Europe/Dublin" "Dọ́blin"
    "Asia/Jerusalem" "Jẹrúsálẹm"
    "Asia/Kolkata" "Kolkáta"
    "Indian/Chagos" "Chágọs"
    "Asia/Baghdad" "Bágdad"
    "Asia/Tehran" "Tẹran"
    "Europe/Rome" "Rom"
    "America/Jamaica" "Jamaíka"
    "Asia/Amman" "Aman"
    "Asia/Tokyo" "Tókyo"
    "Africa/Nairobi" "Naíróbi"
    "Asia/Bishkek" "Bishkẹk"
    "Pacific/Tarawa" "Taráwa"
    "Pacific/Kanton" inherited
    "Pacific/Kiritimati" "Kritímáti"
    "Asia/Pyongyang" "Piọngyang"
    "Asia/Seoul" "Sol"
    "Asia/Almaty" "Álmáti"
    "Asia/Qyzylorda" "Kízilọ́da"
    "Asia/Qostanay" "Kostánai"
    "Asia/Aqtobe" "Aktóbẹ"
    "Asia/Aqtau" "Aktáu"
    "Asia/Atyrau" "Átírau"
    "Asia/Oral" "Ọ́ral"
    "Asia/Beirut" "Bẹrut"
    "Asia/Colombo" "Kolómbo"
    "Africa/Monrovia" "Monróvia"
    "Europe/Vilnius" "Vílnius"
    "Europe/Riga" "Ríga"
    "Africa/Tripoli" "Trípọ́li"
    "Africa/Casablanca" "Kasablánka"
    "Europe/Chisinau" "Chisináu"
    "Pacific/Kwajalein" "Kwájalẹn"
    "Asia/Yangon" "Yangọn"
    "Asia/Ulaanbaatar" "Ulanbáta"
    "Asia/Hovd" inherited
    "Asia/Macau" "Makáo"
    "America/Martinique" "Matínik"
    "Europe/Malta" "Mọ́lta"
    "Indian/Mauritius" "Mọríshọs"
    "Indian/Maldives" "Mọ́ldivs"
    "America/Mexico_City" "Mẹ́ksíkó Síti"
    "America/Cancun" "Kankun"
    "America/Merida" "Mẹ́rída"
    "America/Monterrey" "Mọntẹrẹẹ"
    "America/Matamoros" "Mátamóros"
    "America/Chihuahua" "Chiwuáwua"
    "America/Ciudad_Juarez" inherited
    "America/Ojinaga" "Okhinága"
    "America/Mazatlan" "Mazátlan"
    "America/Bahia_Banderas" "Bahía Bandẹ́ras"
    "America/Hermosillo" "Hẹ́mósílo"
    "America/Tijuana" "Tikhuána"
    "Asia/Kuching" inherited
    "Africa/Maputo" "Mapúto"
    "Africa/Windhoek" "Wíndhok"
    "Pacific/Noumea" "Númẹ́a"
    "Pacific/Norfolk" "Nọ́rfọ́lk"
    "Africa/Lagos" "Légos"
    "America/Managua" "Manágua"
    "Asia/Kathmandu" "Katmándu"
    "Pacific/Nauru" "Naúru"
    "Pacific/Niue" "Niú"
    "Pacific/Auckland" "Ọ́kland"
    "Pacific/Chatham" "Chátam"
    "America/Panama" "Pánáma"
    "America/Lima" "Líma"
    "Pacific/Tahiti" "Tahíti"
    "Pacific/Marquesas" "Makwẹ́sas"
    "Pacific/Gambier" "Gámbiẹr"
    "Pacific/Port_Moresby" "Pọt Mọrẹ́sbi"
    "Pacific/Bougainville" "Bugenvília"
    "Asia/Manila" "Maníla"
    "Asia/Karachi" "Karáchi"
    "Europe/Warsaw" "Wọ́sọ"
    "America/Miquelon" "Míkẹlọn"
    "Pacific/Pitcairn" "Pítkan"
    "America/Puerto_Rico" "Puẹ́rto Ríkọ"
    "Asia/Gaza" "Gáza"
    "Asia/Hebron" "Hẹ́brọn"
    "Europe/Lisbon" "Lísbọn"
    "Atlantic/Madeira" "Madíra"
    "Atlantic/Azores" "Azọz"
    "Pacific/Palau" "Paláu"
    "America/Asuncion" "Asunsiọn"
    "Asia/Qatar" "Káta"
    "Europe/Bucharest" "Búkárẹst"
    "Europe/Belgrade" "Bẹ́lgréd"
    "Europe/Kaliningrad" "Kalíníngrad"
    "Europe/Moscow" "Mọ́sko"
    "Europe/Simferopol" "Símfẹrópol"
    "Europe/Kirov" "Kirọv"
    "Europe/Volgograd" "Volvógrad"
    "Europe/Astrakhan" "Ástrahán"
    "Europe/Saratov" "Sárátov"
    "Europe/Ulyanovsk" "Uliánọvsk"
    "Europe/Samara" "Samára"
    "Asia/Yekaterinburg" "Yẹketẹrínbug"
    "Asia/Omsk" "Ọmsk"
    "Asia/Novosibirsk" "Novosibisk"
    "Asia/Barnaul" "Bárnául"
    "Asia/Tomsk" inherited
    "Asia/Novokuznetsk" "Novokuznẹ́sk"
    "Asia/Krasnoyarsk" "Krasnoyask"
    "Asia/Irkutsk" "Irkútsk"
    "Asia/Chita" "Chítá"
    "Asia/Yakutsk" "Yékútsk"
    "Asia/Khandyga" "Kandíga"
    "Asia/Vladivostok" "Vladivọstọk"
    "Asia/Ust-Nera" "Ust-Nẹ́ra"
    "Asia/Magadan" "Mágádan"
    "Asia/Sakhalin" "Sákhalin"
    "Asia/Srednekolymsk" "Srẹ́dnẹkolimsk"
    "Asia/Kamchatka" "Kamchátké"
    "Asia/Anadyr" "Ánadiar"
    "Asia/Riyadh" "Riyád"
    "Pacific/Guadalcanal" "Guádálkanal"
    "Africa/Khartoum" "Kartum"
    "Asia/Singapore" "Singapọ"
    "America/Paramaribo" "Párámaribo"
    "Africa/Juba" "Júba"
    "Africa/Sao_Tome" "Sao Tómẹ"
    "America/El_Salvador" "El Sálvádọ"
    "Asia/Damascus" "Damáskọs"
    "America/Grand_Turk" "Gránd Tọk"
    "Africa/Ndjamena" "Njamẹ́na"
    "Asia/Bangkok" "Bánkọk"
    "Asia/Dushanbe" "Dushánbẹ"
    "Pacific/Fakaofo" "Fakáófo"
    "Asia/Dili" "Díli"
    "Asia/Ashgabat" "Áshgabat"
    "Africa/Tunis" "Túnis"
    "Pacific/Tongatapu" "Tongatápu"
    "Europe/Istanbul" "Ístánbul"
    "Asia/Taipei" "Taipẹi"
    "Europe/Kyiv" "Kiẹv"
    "America/New_York" "Niú Yọk"
    "America/Detroit" "Ditrọit"
    "America/Kentucky/Louisville" "Luívil"
    "America/Kentucky/Monticello" "Mọntẹchẹ́lo, Kẹ́ntọ́ki"
    "America/Indiana/Indianapolis" "Indiánápọ́lis"
    "America/Indiana/Vincennes" "Vínsẹn, Indiána"
    "America/Indiana/Winamac" "Wínámak, Indiána"
    "America/Indiana/Marengo" "Marẹ́ngo, Indiána"
    "America/Indiana/Petersburg" "Pításbọg, Indiána"
    "America/Indiana/Vevay" "Vẹ́ve, Indiána"
    "America/Chicago" "Chikágo"
    "America/Indiana/Tell_City" "Tẹ́l Síti, Indiána"
    "America/Indiana/Knox" "Nọks, Indiána"
    "America/Menominee" "Mẹnọ́minii"
    "America/North_Dakota/Center" "Sẹ́nta, Nọ́t Dakóta"
    "America/North_Dakota/New_Salem" "Niú Sélẹm, Nọ́t Dakóta"
    "America/North_Dakota/Beulah" "Biúla, Nọ́t Dakóta"
    "America/Denver" "Dẹ́nva"
    "America/Boise" "Bọísi"
    "America/Phoenix" "Fíniks"
    "America/Los_Angeles" "Lọs Ánjẹ́lis"
    "America/Anchorage" "Ánkọ́rej"
    "America/Juneau" "Júno"
    "America/Sitka" inherited
    "America/Metlakatla" "Mẹtlakátla"
    "America/Yakutat" "Yakútat"
    "America/Nome" "Noom"
    "America/Adak" "Ádak"
    "Pacific/Honolulu" ""
    "America/Montevideo" "Mọntẹvidẹo"
    "Asia/Samarkand" "Sámákand"
    "Asia/Tashkent" "Táshkẹnt"
    "America/Caracas" "Karákas"
    "Asia/Ho_Chi_Minh" "Hó Chi Mín Síti"
    "Pacific/Efate" "Ẹfátẹ"
    "Pacific/Apia" "Ápia"
    "Africa/Johannesburg" "Johánísbọg"
    "America/Antigua" "Antígwua"
    "America/Anguilla" "Angwíla"
    "Africa/Luanda" "Luánda"
    "Antarctica/McMurdo" "McMọ́do"
    "Antarctica/DumontDUrville" "Diúmọ́n-d’Uvil"
    "Antarctica/Syowa" "Siówa"
    "America/Aruba" "Arúba"
    "Europe/Mariehamn" "Maríahámn"
    "Europe/Sarajevo" "Sarayẹ́vo"
    "Africa/Ouagadougou" "Ouagadúgu"
    "Asia/Bahrain" "Bahrén"
    "Africa/Bujumbura" "Bujumbúra"
    "Africa/Porto-Novo" "Pọto-Nóvo"
    "America/St_Barthelemy" "Sent Batẹlẹ́mi"
    "Asia/Brunei" "Brunẹi"
    "America/Kralendijk" "Králẹ́ndijk"
    "America/Nassau" "Nássọu"
    "Africa/Gaborone" "Háborónẹ"
    "America/Blanc-Sablon" "Blank-Sáblọn"
    "America/Atikokan" "Atíkókan"
    "America/Creston" "Krẹ́stọn"
    "Indian/Cocos" "Kókos"
    "Africa/Kinshasa" "Kinshásha"
    "Africa/Lubumbashi" "Lubumbáshi"
    "Africa/Bangui" "Bangúi"
    "Africa/Brazzaville" "Brázavil"
    "Africa/Douala" "Duála"
    "America/Curacao" "Kiurásao"
    "Indian/Christmas" "Krísmas"
    "Europe/Busingen" "Busíngẹn"
    "Africa/Djibouti" "Jibúti"
    "Europe/Copenhagen" "Kọpẹnhágẹn"
    "America/Dominica" "Dọmíníka"
    "Africa/Asmara" "Asmára"
    "Africa/Addis_Ababa" "Adí Abába"
    "Pacific/Chuuk" "Chuk"
    "Pacific/Pohnpei" "Pọnpẹ́i"
    "Africa/Libreville" "Líbrẹvil"
    "America/Grenada" "Grẹnéda"
    "Europe/Guernsey" "Guẹnzi"
    "Africa/Accra" "Akrá"
    "Africa/Banjul" inherited
    "Africa/Conakry" "Kọnákri"
    "America/Guadeloupe" "Guadalúpẹ"
    "Africa/Malabo" "Malábo"
    "Europe/Zagreb" "Zágrẹb"
    "Europe/Isle_of_Man" "Aíl ọf Man"
    "Atlantic/Reykjavik" "Rẹ́kjávik"
    "Europe/Jersey" "Jẹ́si"
    "Asia/Phnom_Penh" "Fnọ́m Pẹn"
    "Indian/Comoro" "Kọ́mọ́ros"
    "America/St_Kitts" "Sent Kits"
    "Asia/Kuwait" "Kuwet"
    "America/Cayman" "Kéman"
    "Asia/Vientiane" "Viẹ́ntiẹn"
    "America/St_Lucia" "Sent Lúshia"
    "Europe/Vaduz" inherited
    "Africa/Maseru" "Masẹ́ru"
    "Europe/Luxembourg" "Lọ́ksẹ́mbọg"
    "Europe/Monaco" "Mọ́náko"
    "Europe/Podgorica" "Pọ́jóríka"
    "America/Marigot" "Márígọt"
    "Indian/Antananarivo" "Antánánarívo"
    "Pacific/Majuro" "Majúro"
    "Europe/Skopje" "Skọ́pyẹ"
    "Africa/Bamako" "Bamáko"
    "Pacific/Saipan" inherited
    "Africa/Nouakchott" "Nouákshọt"
    "America/Montserrat" "Mọntsẹrat"
    "Africa/Blantyre" "Blantáya"
    "Asia/Kuala_Lumpur" "Kuála Lúmpọ"
    "Africa/Niamey" "Niáme"
    "Europe/Amsterdam" "Ámstádam"
    "Europe/Oslo" "Ọ́slo"
    "Asia/Muscat" "Múskat"
    "Indian/Reunion" "Riyúniọn"
    "Africa/Kigali" "Kigáli"
    "Indian/Mahe" "Mahẹ́"
    "Europe/Stockholm" "Stọ́khọm"
    "Atlantic/St_Helena" "Sent Hẹlẹ́na"
    "Europe/Ljubljana" "Lubliána"
    "Arctic/Longyearbyen" "Lọngyẹ́abiẹn"
    "Europe/Bratislava" "Bratísláva"
    "Africa/Freetown" "Frítaun"
    "Europe/San_Marino" "San Maríno"
    "Africa/Dakar" "Dakár"
    "Africa/Mogadishu" "Mọgádíshu"
    "America/Lower_Princes" "Lówá Príns Im Kwọ́ta"
    "Africa/Mbabane" "Mbabánẹ"
    "Indian/Kerguelen" "Kẹ́rgúlẹn"
    "Africa/Lome" "Lómẹ"
    "America/Port_of_Spain" "Pọ́t ọf Spen"
    "Pacific/Funafuti" "Funafúti"
    "Africa/Dar_es_Salaam" "Dar ẹ́s Salam"
    "Africa/Kampala" "Kampála"
    "Pacific/Midway" "Mídwè"
    "Pacific/Wake" "Wek"
    "Europe/Vatican" "Vátíkan"
    "America/St_Vincent" "Sent Vínsẹnt"
    "America/Tortola" "Tọtóla"
    "America/St_Thomas" "Sent Tọmọs"
    "Pacific/Wallis" "Wáli"
    "Asia/Aden" "Édẹn"
    "Indian/Mayotte" "Meyọt"
    "Africa/Lusaka" "Lusáka"
    "Africa/Harare" "Harárẹ"
};

// `common/main/pt_PT.xml`: 83 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const PT_PT: &str = exemplar_cities! {
    "Europe/Andorra" ""
    "Asia/Dubai" ""
    "Asia/Kabul" ""
    "Europe/Tirane" ""
    "Asia/Yerevan" "Erevan"
    "Antarctica/Casey" "Estação Casey"
    "Antarctica/Davis" ""
    "Antarctica/Mawson" "Estação Mawson"
    "Antarctica/Palmer" "Terra de Palmer"
    "Antarctica/Rothera" "Estação Rothera"
    "Antarctica/Troll" "Estação Troll"
    "Antarctica/Vostok" "Estação Vostok"
    "America/Argentina/Buenos_Aires" ""
    "America/Argentina/Cordoba" ""
    "America/Argentina/Salta" ""
    "America/Argentina/Jujuy" ""
    "America/Argentina/Tucuman" "Tucumán"
    "America/Argentina/Catamarca" ""
    "America/Argentina/La_Rioja" ""
    "America/Argentina/San_Juan" ""
    "America/Argentina/Mendoza" ""
    "America/Argentina/San_Luis" ""
    "America/Argentina/Rio_Gallegos" ""
    "America/Argentina/Ushuaia" ""
    "Pacific/Pago_Pago" ""
    "Europe/Vienna" ""
    "Australia/Lord_Howe" "Ilha de Lord Howe"
    "Antarctica/Macquarie" "Ilha Macquarie"
    "Australia/Hobart" ""
    "Australia/Melbourne" ""
    "Australia/Sydney" ""
    "Australia/Broken_Hill" ""
    "Australia/Brisbane" ""
    "Australia/Lindeman" ""
    "Australia/Adelaide" ""
    "Australia/Darwin" ""
    "Australia/Perth" ""
    "Australia/Eucla" ""
    "Asia/Baku" ""
    "America/Barbados" ""
    "Asia/Dhaka" "Daca"
    "Europe/Brussels" ""
    "Europe/Sofia" ""
    "Atlantic/Bermuda" ""
    "America/La_Paz" ""
    "America/Noronha" ""
    "America/Belem" ""
    "America/Fortaleza" ""
    "America/Recife" ""
    "America/Araguaina" ""
    "America/Maceio" ""
    "America/Bahia" "Baía"
    "America/Sao_Paulo" ""
    "America/Campo_Grande" ""
    "America/Cuiaba" ""
    "America/Santarem" ""
    "America/Porto_Velho" ""
    "America/Boa_Vista" ""
    "America/Manaus" ""
    "America/Eirunepe" ""
    "America/Rio_Branco" ""
    "Asia/Thimphu" "Timphu"
    "Europe/Minsk" ""
    "America/Belize" ""
    "America/St_Johns" "St. John’s"
    "America/Halifax" ""
    "America/Glace_Bay" ""
    "America/Moncton" ""
    "America/Goose_Bay" ""
    "America/Toronto" ""
    "America/Iqaluit" ""
    "America/Winnipeg" ""
    "America/Resolute" ""
    "America/Rankin_Inlet" ""
    "America/Regina" ""
    "America/Swift_Current" ""
    "America/Edmonton" ""
    "America/Cambridge_Bay" ""
    "America/Inuvik" ""
    "America/Vancouver" ""
    "America/Dawson_Creek" ""
    "America/Fort_Nelson" ""
    "America/Whitehorse" ""
    "America/Dawson" ""
    "Europe/Zurich" ""
    "Africa/Abidjan" ""
    "Pacific/Rarotonga" ""
    "America/Santiago" ""
    "America/Coyhaique" ""
    "America/Punta_Arenas" ""
    "Pacific/Easter" "Ilha da Páscoa"
    "Asia/Shanghai" ""
    "Asia/Urumqi" ""
    "America/Bogota" ""
    "America/Costa_Rica" ""
    "America/Havana" ""
    "Atlantic/Cape_Verde" ""
    "Asia/Nicosia" ""
    "Asia/Famagusta" ""
    "Europe/Prague" ""
    "Europe/Berlin" ""
    "America/Santo_Domingo" ""
    "Africa/Algiers" ""
    "America/Guayaquil" ""
    "Pacific/Galapagos" "Ilhas Galápagos"
    "Europe/Tallinn" "Talim"
    "Africa/Cairo" ""
    "Africa/El_Aaiun" ""
    "Europe/Madrid" "Madrid"
    "Africa/Ceuta" ""
    "Atlantic/Canary" ""
    "Europe/Helsinki" "Helsínquia"
    "Pacific/Fiji" ""
    "Atlantic/Stanley" ""
    "Pacific/Kosrae" ""
    "Atlantic/Faroe" "Faroé"
    "Europe/Paris" ""
    "Europe/London" ""
    "Asia/Tbilisi" ""
    "America/Cayenne" ""
    "Europe/Gibraltar" ""
    "America/Nuuk" ""
    "America/Danmarkshavn" ""
    "America/Scoresbysund" ""
    "America/Thule" ""
    "Europe/Athens" ""
    "Atlantic/South_Georgia" ""
    "America/Guatemala" ""
    "Pacific/Guam" ""
    "Africa/Bissau" ""
    "America/Guyana" ""
    "Asia/Hong_Kong" ""
    "America/Tegucigalpa" ""
    "America/Port-au-Prince" "Port-au-Prince"
    "Europe/Budapest" ""
    "Asia/Jakarta" ""
    "Asia/Pontianak" ""
    "Asia/Makassar" "Macassar"
    "Asia/Jayapura" ""
    "Europe/Dublin" ""
    "Asia/Jerusalem" ""
    "Asia/Kolkata" ""
    "Indian/Chagos" "Arquipélago de Chagos"
    "Asia/Baghdad" "Bagdade"
    "Asia/Tehran" "Teerão"
    "Europe/Rome" ""
    "America/Jamaica" ""
    "Asia/Amman" ""
    "Asia/Tokyo" ""
    "Africa/Nairobi" "Nairobi"
    "Asia/Bishkek" ""
    "Pacific/Tarawa" "Tarawa"
    "Pacific/Kanton" "Ilha Canton"
    "Pacific/Kiritimati" ""
    "Asia/Pyongyang" ""
    "Asia/Seoul" ""
    "Asia/Almaty" ""
    "Asia/Qyzylorda" ""
    "Asia/Qostanay" "Kostanay"
    "Asia/Aqtobe" "Aqtobe"
    "Asia/Aqtau" "Aqtau"
    "Asia/Atyrau" ""
    "Asia/Oral" ""
    "Asia/Beirut" ""
    "Asia/Colombo" ""
    "Africa/Monrovia" ""
    "Europe/Vilnius" ""
    "Europe/Riga" ""
    "Africa/Tripoli" "Tripoli"
    "Africa/Casablanca" ""
    "Europe/Chisinau" ""
    "Pacific/Kwajalein" "Atol de Kwajalein"
    "Asia/Yangon" "Yangon"
    "Asia/Ulaanbaatar" ""
    "Asia/Hovd" ""
    "Asia/Macau" ""
    "America/Martinique" ""
    "Europe/Malta" ""
    "Indian/Mauritius" "Maurícia"
    "Indian/Maldives" ""
    "America/Mexico_City" ""
    "America/Cancun" "Cancun"
    "America/Merida" ""
    "America/Monterrey" ""
    "America/Matamoros" ""
    "America/Chihuahua" ""
    "America/Ciudad_Juarez" ""
    "America/Ojinaga" ""
    "America/Mazatlan" ""
    "America/Bahia_Banderas" "Bahia Banderas"
    "America/Hermosillo" ""
    "America/Tijuana" ""
    "Asia/Kuching" ""
    "Africa/Maputo" ""
    "Africa/Windhoek" ""
    "Pacific/Noumea" ""
    "Pacific/Norfolk" "Ilha Norfolk"
    "Africa/Lagos" ""
    "America/Managua" ""
    "Asia/Kathmandu" "Catmandu"
    "Pacific/Nauru" ""
    "Pacific/Niue" ""
    "Pacific/Auckland" ""
    "Pacific/Chatham" "Ilhas Chatham"
    "America/Panama" ""
    "America/Lima" ""
    "Pacific/Tahiti" ""
    "Pacific/Marquesas" "Ilhas Marquesas"
    "Pacific/Gambier" ""
    "Pacific/Port_Moresby" ""
    "Pacific/Bougainville" ""
    "Asia/Manila" ""
    "Asia/Karachi" "Carachi"
    "Europe/Warsaw" ""
    "America/Miquelon" ""
    "Pacific/Pitcairn" "Ilhas Pitcairn"
    "America/Puerto_Rico" ""
    "Asia/Gaza" ""
    "Asia/Hebron" ""
    "Europe/Lisbon" ""
    "Atlantic/Madeira" ""
    "Atlantic/Azores" ""
    "Pacific/Palau" ""
    "America/Asuncion" ""
    "Asia/Qatar" ""
    "Europe/Bucharest" ""
    "Europe/Belgrade" ""
    "Europe/Kaliningrad" "Caliningrado"
    "Europe/Moscow" "Moscovo"
    "Europe/Simferopol" ""
    "Europe/Kirov" ""
    "Europe/Volgograd" ""
    "Europe/Astrakhan" ""
    "Europe/Saratov" ""
    "Europe/Ulyanovsk" ""
    "Europe/Samara" ""
    "Asia/Yekaterinburg" ""
    "Asia/Omsk" ""
    "Asia/Novosibirsk" ""
    "Asia/Barnaul" ""
    "Asia/Tomsk" ""
    "Asia/Novokuznetsk" ""
    "Asia/Krasnoyarsk" ""
    "Asia/Irkutsk" ""
    "Asia/Chita" ""
    "Asia/Yakutsk" ""
    "Asia/Khandyga" ""
    "Asia/Vladivostok" ""
    "Asia/Ust-Nera" ""
    "Asia/Magadan" ""
    "Asia/Sakhalin" ""
    "Asia/Srednekolymsk" ""
    "Asia/Kamchatka" ""
    "Asia/Anadyr" ""
    "Asia/Riyadh" ""
    "Pacific/Guadalcanal" ""
    "Africa/Khartoum" ""
    "Asia/Singapore" ""
    "America/Paramaribo" ""
    "Africa/Juba" ""
    "Africa/Sao_Tome" ""
    "America/El_Salvador" "Salvador"
    "Asia/Damascus" ""
    "America/Grand_Turk" ""
    "Africa/Ndjamena" "Ndjamena"
    "Asia/Bangkok" "Banguecoque"
    "Asia/Dushanbe" ""
    "Pacific/Fakaofo" ""
    "Asia/Dili" ""
    "Asia/Ashgabat" ""
    "Africa/Tunis" "Tunes"
    "Pacific/Tongatapu" ""
    "Europe/Istanbul" ""
    "Asia/Taipei" "Taipé"
    "Europe/Kyiv" ""
    "America/New_York" "Nova Iorque"
    "America/Detroit" ""
    "America/Kentucky/Louisville" ""
    "America/Kentucky/Monticello" ""
    "America/Indiana/Indianapolis" ""
    "America/Indiana/Vincennes" ""
    "America/Indiana/Winamac" ""
    "America/Indiana/Marengo" ""
    "America/Indiana/Petersburg" ""
    "America/Indiana/Vevay" ""
    "America/Chicago" ""
    "America/Indiana/Tell_City" ""
    "America/Indiana/Knox" ""
    "America/Menominee" ""
    "America/North_Dakota/Center" ""
    "America/North_Dakota/New_Salem" ""
    "America/North_Dakota/Beulah" ""
    "America/Denver" ""
    "America/Boise" ""
    "America/Phoenix" ""
    "America/Los_Angeles" ""
    "America/Anchorage" ""
    "America/Juneau" ""
    "America/Sitka" ""
    "America/Metlakatla" ""
    "America/Yakutat" ""
    "America/Nome" ""
    "America/Adak" ""
    "Pacific/Honolulu" ""
    "America/Montevideo" "Montevideu"
    "Asia/Samarkand" ""
    "Asia/Tashkent" ""
    "America/Caracas" ""
    "Asia/Ho_Chi_Minh" ""
    "Pacific/Efate" "Efate"
    "Pacific/Apia" ""
    "Africa/Johannesburg" ""
    "America/Antigua" ""
    "America/Anguilla" ""
    "Africa/Luanda" ""
    "Antarctica/McMurdo" "Estação McMurdo"
    "Antarctica/DumontDUrville" "Estação Dumont-d’Urville"
    "Antarctica/Syowa" "Estação Showa"
    "America/Aruba" ""
    "Europe/Mariehamn" ""
    "Europe/Sarajevo" ""
    "Africa/Ouagadougou" ""
    "Asia/Bahrain" "Barém"
    "Africa/Bujumbura" ""
    "Africa/Porto-Novo" "Porto-Novo"
    "America/St_Barthelemy" ""
    "Asia/Brunei" ""
    "America/Kralendijk" ""
    "America/Nassau" ""
    "Africa/Gaborone" ""
    "America/Blanc-Sablon" ""
    "America/Atikokan" ""
    "America/Creston" ""
    "Indian/Cocos" "Ilhas Cocos"
    "Africa/Kinshasa" ""
    "Africa/Lubumbashi" ""
    "Africa/Bangui" ""
    "Africa/Brazzaville" ""
    "Africa/Douala" ""
    "America/Curacao" "Curaçau"
    "Indian/Christmas" "Ilha do Natal"
    "Europe/Busingen" ""
    "Africa/Djibouti" "Jibuti"
    "Europe/Copenhagen" "Copenhaga"
    "America/Dominica" "Domínica"
    "Africa/Asmara" ""
    "Africa/Addis_Ababa" "Adis-Abeba"
    "Pacific/Chuuk" ""
    "Pacific/Pohnpei" ""
    "Africa/Libreville" ""
    "America/Grenada" ""
    "Europe/Guernsey" ""
    "Africa/Accra" ""
    "Africa/Banjul" ""
    "Africa/Conakry" ""
    "America/Guadeloupe" ""
    "Africa/Malabo" ""
    "Europe/Zagreb" ""
    "Europe/Isle_of_Man" ""
    "Atlantic/Reykjavik" "Reiquiavique"
    "Europe/Jersey" ""
    "Asia/Phnom_Penh" ""
    "Indian/Comoro" ""
    "America/St_Kitts" ""
    "Asia/Kuwait" "Koweit"
    "America/Cayman" "Caimão"
    "Asia/Vientiane" ""
    "America/St_Lucia" ""
    "Europe/Vaduz" ""
    "Africa/Maseru" ""
    "Europe/Luxembourg" ""
    "Europe/Monaco" "Mónaco"
    "Europe/Podgorica" ""
    "America/Marigot" ""
    "Indian/Antananarivo" ""
    "Pacific/Majuro" ""
    "Europe/Skopje" ""
    "Africa/Bamako" "Bamaco"
    "Pacific/Saipan" ""
    "Africa/Nouakchott" ""
    "America/Montserrat" "Monserrate"
    "Africa/Blantyre" ""
    "Asia/Kuala_Lumpur" ""
    "Africa/Niamey" "Niamei"
    "Europe/Amsterdam" "Amesterdão"
    "Europe/Oslo" ""
    "Asia/Muscat" ""
    "Indian/Reunion" ""
    "Africa/Kigali" ""
    "Indian/Mahe" ""
    "Europe/Stockholm" ""
    "Atlantic/St_Helena" ""
    "Europe/Ljubljana" ""
    "Arctic/Longyearbyen" ""
    "Europe/Bratislava" ""
    "Africa/Freetown" ""
    "Europe/San_Marino" "São Marinho"
    "Africa/Dakar" "Dacar"
    "Africa/Mogadishu" ""
    "America/Lower_Princes" ""
    "Africa/Mbabane" ""
    "Indian/Kerguelen" "Ilhas Kerguelen"
    "Africa/Lome" ""
    "America/Port_of_Spain" "Porto de Espanha"
    "Pacific/Funafuti" ""
    "Africa/Dar_es_Salaam" ""
    "Africa/Kampala" "Campala"
    "Pacific/Midway" "Atol de Midway"
    "Pacific/Wake" "Ilha Wake"
    "Europe/Vatican" ""
    "America/St_Vincent" ""
    "America/Tortola" ""
    "America/St_Thomas" "St. Thomas"
    "Pacific/Wallis" ""
    "Asia/Aden" "Adem"
    "Indian/Mayotte" ""
    "Africa/Lusaka" "Lusaca"
    "Africa/Harare" ""
};

// `common/main/sw.xml`: 11 of the 418 zones named, 407 inherited.
#[cfg(feature = "localized-exemplar-cities")]
const SW: &str = exemplar_cities! {
    "Europe/Andorra" inherited
    "Asia/Dubai" inherited
    "Asia/Kabul" inherited
    "Europe/Tirane" inherited
    "Asia/Yerevan" inherited
    "Antarctica/Casey" inherited
    "Antarctica/Davis" inherited
    "Antarctica/Mawson" inherited
    "Antarctica/Palmer" inherited
    "Antarctica/Rothera" inherited
    "Antarctica/Troll" inherited
    "Antarctica/Vostok" inherited
    "America/Argentina/Buenos_Aires" inherited
    "America/Argentina/Cordoba" inherited
    "America/Argentina/Salta" inherited
    "America/Argentina/Jujuy" inherited
    "America/Argentina/Tucuman" inherited
    "America/Argentina/Catamarca" inherited
    "America/Argentina/La_Rioja" inherited
    "America/Argentina/San_Juan" inherited
    "America/Argentina/Mendoza" inherited
    "America/Argentina/San_Luis" inherited
    "America/Argentina/Rio_Gallegos" inherited
    "America/Argentina/Ushuaia" inherited
    "Pacific/Pago_Pago" inherited
    "Europe/Vienna" inherited
    "Australia/Lord_Howe" inherited
    "Antarctica/Macquarie" inherited
    "Australia/Hobart" inherited
    "Australia/Melbourne" inherited
    "Australia/Sydney" inherited
    "Australia/Broken_Hill" inherited
    "Australia/Brisbane" inherited
    "Australia/Lindeman" inherited
    "Australia/Adelaide" inherited
    "Australia/Darwin" inherited
    "Australia/Perth" inherited
    "Australia/Eucla" inherited
    "Asia/Baku" inherited
    "America/Barbados" inherited
    "Asia/Dhaka" inherited
    "Europe/Brussels" inherited
    "Europe/Sofia" inherited
    "Atlantic/Bermuda" inherited
    "America/La_Paz" inherited
    "America/Noronha" inherited
    "America/Belem" inherited
    "America/Fortaleza" inherited
    "America/Recife" inherited
    "America/Araguaina" inherited
    "America/Maceio" inherited
    "America/Bahia" inherited
    "America/Sao_Paulo" inherited
    "America/Campo_Grande" inherited
    "America/Cuiaba" inherited
    "America/Santarem" inherited
    "America/Porto_Velho" inherited
    "America/Boa_Vista" inherited
    "America/Manaus" inherited
    "America/Eirunepe" inherited
    "America/Rio_Branco" inherited
    "Asia/Thimphu" inherited
    "Europe/Minsk" inherited
    "America/Belize" inherited
    "America/St_Johns" inherited
    "America/Halifax" inherited
    "America/Glace_Bay" inherited
    "America/Moncton" inherited
    "America/Goose_Bay" inherited
    "America/Toronto" inherited
    "America/Iqaluit" inherited
    "America/Winnipeg" inherited
    "America/Resolute" inherited
    "America/Rankin_Inlet" inherited
    "America/Regina" inherited
    "America/Swift_Current" inherited
    "America/Edmonton" inherited
    "America/Cambridge_Bay" inherited
    "America/Inuvik" inherited
    "America/Vancouver" inherited
    "America/Dawson_Creek" inherited
    "America/Fort_Nelson" inherited
    "America/Whitehorse" inherited
    "America/Dawson" inherited
    "Europe/Zurich" inherited
    "Africa/Abidjan" inherited
    "Pacific/Rarotonga" inherited
    "America/Santiago" inherited
    "America/Coyhaique" inherited
    "America/Punta_Arenas" inherited
    "Pacific/Easter" inherited
    "Asia/Shanghai" inherited
    "Asia/Urumqi" inherited
    "America/Bogota" inherited
    "America/Costa_Rica" inherited
    "America/Havana" inherited
    "Atlantic/Cape_Verde" inherited
    "Asia/Nicosia" inherited
    "Asia/Famagusta" inherited
    "Europe/Prague" inherited
    "Europe/Berlin" inherited
    "America/Santo_Domingo" inherited
    "Africa/Algiers" inherited
    "America/Guayaquil" inherited
    "Pacific/Galapagos" inherited
    "Europe/Tallinn" inherited
    "Africa/Cairo" inherited
    "Africa/El_Aaiun" inherited
    "Europe/Madrid" inherited
    "Africa/Ceuta" inherited
    "Atlantic/Canary" inherited
    "Europe/Helsinki" inherited
    "Pacific/Fiji" inherited
    "Atlantic/Stanley" inherited
    "Pacific/Kosrae" inherited
    "Atlantic/Faroe" inherited
    "Europe/Paris" inherited
    "Europe/London" inherited
    "Asia/Tbilisi" inherited
    "America/Cayenne" inherited
    "Europe/Gibraltar" inherited
    "America/Nuuk" inherited
    "America/Danmarkshavn" inherited
    "America/Scoresbysund" inherited
    "America/Thule" inherited
    "Europe/Athens" inherited
    "Atlantic/South_Georgia" "Georgia Kusini"
    "America/Guatemala" inherited
    "Pacific/Guam" inherited
    "Africa/Bissau" inherited
    "America/Guyana" inherited
    "Asia/Hong_Kong" inherited
    "America/Tegucigalpa" inherited
    "America/Port-au-Prince" inherited
    "Europe/Budapest" inherited
    "Asia/Jakarta" inherited
    "Asia/Pontianak" inherited
    "Asia/Makassar" inherited
    "Asia/Jayapura" inherited
    "Europe/Dublin" inherited
    "Asia/Jerusalem" inherited
    "Asia/Kolkata" inherited
    "Indian/Chagos" inherited
    "Asia/Baghdad" inherited
    "Asia/Tehran" inherited
    "Europe/Rome" inherited
    "America/Jamaica" inherited
    "Asia/Amman" inherited
    "Asia/Tokyo" inherited
    "Africa/Nairobi" inherited
    "Asia/Bishkek" inherited
    "Pacific/Tarawa" inherited
    "Pacific/Kanton" inherited
    "Pacific/Kiritimati" inherited
    "Asia/Pyongyang" inherited
    "Asia/Seoul" inherited
    "Asia/Almaty" inherited
    "Asia/Qyzylorda" inherited
    "Asia/Qostanay" "Kostanay"
    "Asia/Aqtobe" inherited
    "Asia/Aqtau" inherited
    "Asia/Atyrau" inherited
    "Asia/Oral" inherited
    "Asia/Beirut" inherited
    "Asia/Colombo" inherited
    "Africa/Monrovia" inherited
    "Europe/Vilnius" inherited
    "Europe/Riga" inherited
    "Africa/Tripoli" inherited
    "Africa/Casablanca" inherited
    "Europe/Chisinau" inherited
    "Pacific/Kwajalein" inherited
    "Asia/Yangon" "Rangoon"
    "Asia/Ulaanbaatar" inherited
    "Asia/Hovd" inherited
    "Asia/Macau" "Macau"
    "America/Martinique" inherited
    "Europe/Malta" inherited
    "Indian/Mauritius" inherited
    "Indian/Maldives" inherited
    "America/Mexico_City" "Jiji la Mexico"
    "America/Cancun" "Cancun"
    "America/Merida" "Merida"
    "America/Monterrey" inherited
    "America/Matamoros" inherited
    "America/Chihuahua" inherited
    "America/Ciudad_Juarez" "Ciudad Juarez"
    "America/Ojinaga" inherited
    "America/Mazatlan" inherited
    "America/Bahia_Banderas" "Bahia Banderas"
    "America/Hermosillo" inherited
    "America/Tijuana" inherited
    "Asia/Kuching" inherited
    "Africa/Maputo" inherited
    "Africa/Windhoek" inherited
    "Pacific/Noumea" inherited
    "Pacific/Norfolk" inherited
    "Africa/Lagos" inherited
    "America/Managua" inherited
    "Asia/Kathmandu" inherited
    "Pacific/Nauru" inherited
    "Pacific/Niue" inherited
    "Pacific/Auckland" inherited
    "Pacific/Chatham" inherited
    "America/Panama" inherited
    "America/Lima" inherited
    "Pacific/Tahiti" inherited
    "Pacific/Marquesas" inherited
    "Pacific/Gambier" inherited
    "Pacific/Port_Moresby" inherited
    "Pacific/Bougainville" inherited
    "Asia/Manila" inherited
    "Asia/Karachi" inherited
    "Europe/Warsaw" inherited
    "America/Miquelon" inherited
    "Pacific/Pitcairn" inherited
    "America/Puerto_Rico" inherited
    "Asia/Gaza" inherited
    "Asia/Hebron" inherited
    "Europe/Lisbon" inherited
    "Atlantic/Madeira" inherited
    "Atlantic/Azores" inherited
    "Pacific/Palau" inherited
    "America/Asuncion" inherited
    "Asia/Qatar" inherited
    "Europe/Bucharest" inherited
    "Europe/Belgrade" inherited
    "Europe/Kaliningrad" inherited
    "Europe/Moscow" inherited
    "Europe/Simferopol" inherited
    "Europe/Kirov" inherited
    "Europe/Volgograd" inherited
    "Europe/Astrakhan" inherited
    "Europe/Saratov" inherited
    "Europe/Ulyanovsk" inherited
    "Europe/Samara" inherited
    "Asia/Yekaterinburg" inherited
    "Asia/Omsk" inherited
    "Asia/Novosibirsk" inherited
    "Asia/Barnaul" inherited
    "Asia/Tomsk" inherited
    "Asia/Novokuznetsk" inherited
    "Asia/Krasnoyarsk" inherited
    "Asia/Irkutsk" inherited
    "Asia/Chita" inherited
    "Asia/Yakutsk" inherited
    "Asia/Khandyga" inherited
    "Asia/Vladivostok" inherited
    "Asia/Ust-Nera" inherited
    "Asia/Magadan" inherited
    "Asia/Sakhalin" inherited
    "Asia/Srednekolymsk" inherited
    "Asia/Kamchatka" inherited
    "Asia/Anadyr" inherited
    "Asia/Riyadh" inherited
    "Pacific/Guadalcanal" inherited
    "Africa/Khartoum" inherited
    "Asia/Singapore" inherited
    "America/Paramaribo" inherited
    "Africa/Juba" inherited
    "Africa/Sao_Tome" inherited
    "America/El_Salvador" inherited
    "Asia/Damascus" inherited
    "America/Grand_Turk" inherited
    "Africa/Ndjamena" inherited
    "Asia/Bangkok" inherited
    "Asia/Dushanbe" inherited
    "Pacific/Fakaofo" inherited
    "Asia/Dili" inherited
    "Asia/Ashgabat" inherited
    "Africa/Tunis" inherited
    "Pacific/Tongatapu" inherited
    "Europe/Istanbul" inherited
    "Asia/Taipei" inherited
    "Europe/Kyiv" "Kiev"
    "America/New_York" inherited
    "America/Detroit" inherited
    "America/Kentucky/Louisville" inherited
    "America/Kentucky/Monticello" inherited
    "America/Indiana/Indianapolis" inherited
    "America/Indiana/Vincennes" inherited
    "America/Indiana/Winamac" inherited
    "America/Indiana/Marengo" inherited
    "America/Indiana/Petersburg" inherited
    "America/Indiana/Vevay" inherited
    "America/Chicago" inherited
    "America/Indiana/Tell_City" inherited
    "America/Indiana/Knox" inherited
    "America/Menominee" inherited
    "America/North_Dakota/Center" inherited
    "America/North_Dakota/New_Salem" inherited
    "America/North_Dakota/Beulah" inherited
    "America/Denver" inherited
    "America/Boise" inherited
    "America/Phoenix" inherited
    "America/Los_Angeles" inherited
    "America/Anchorage" inherited
    "America/Juneau" inherited
    "America/Sitka" inherited
    "America/Metlakatla" inherited
    "America/Yakutat" inherited
    "America/Nome" inherited
    "America/Adak" inherited
    "Pacific/Honolulu" "Honolulu"
    "America/Montevideo" inherited
    "Asia/Samarkand" inherited
    "Asia/Tashkent" inherited
    "America/Caracas" inherited
    "Asia/Ho_Chi_Minh" inherited
    "Pacific/Efate" inherited
    "Pacific/Apia" inherited
    "Africa/Johannesburg" inherited
    "America/Antigua" inherited
    "America/Anguilla" inherited
    "Africa/Luanda" inherited
    "Antarctica/McMurdo" inherited
    "Antarctica/DumontDUrville" inherited
    "Antarctica/Syowa" inherited
    "America/Aruba" inherited
    "Europe/Mariehamn" inherited
    "Europe/Sarajevo" inherited
    "Africa/Ouagadougou" inherited
    "Asia/Bahrain" inherited
    "Africa/Bujumbura" inherited
    "Africa/Porto-Novo" inherited
    "America/St_Barthelemy" inherited
    "Asia/Brunei" inherited
    "America/Kralendijk" inherited
    "America/Nassau" inherited
    "Africa/Gaborone" inherited
    "America/Blanc-Sablon" inherited
    "America/Atikokan" inherited
    "America/Creston" inherited
    "Indian/Cocos" inherited
    "Africa/Kinshasa" inherited
    "Africa/Lubumbashi" inherited
    "Africa/Bangui" inherited
    "Africa/Brazzaville" inherited
    "Africa/Douala" inherited
    "America/Curacao" inherited
    "Indian/Christmas" inherited
    "Europe/Busingen" inherited
    "Africa/Djibouti" inherited
    "Europe/Copenhagen" inherited
    "America/Dominica" inherited
    "Africa/Asmara" inherited
    "Africa/Addis_Ababa" inherited
    "Pacific/Chuuk" inherited
    "Pacific/Pohnpei" inherited
    "Africa/Libreville" inherited
    "America/Grenada" inherited
    "Europe/Guernsey" inherited
    "Africa/Accra" inherited
    "Africa/Banjul" inherited
    "Africa/Conakry" inherited
    "America/Guadeloupe" inherited
    "Africa/Malabo" inherited
    "Europe/Zagreb" inherited
    "Europe/Isle_of_Man" inherited
    "Atlantic/Reykjavik" inherited
    "Europe/Jersey" inherited
    "Asia/Phnom_Penh" inherited
    "Indian/Comoro" inherited
    "America/St_Kitts" inherited
    "Asia/Kuwait" inherited
    "America/Cayman" inherited
    "Asia/Vientiane" inherited
    "America/St_Lucia" inherited
    "Europe/Vaduz" inherited
    "Africa/Maseru" inherited
    "Europe/Luxembourg" inherited
    "Europe/Monaco" inherited
    "Europe/Podgorica" inherited
    "America/Marigot" inherited
    "Indian/Antananarivo" inherited
    "Pacific/Majuro" inherited
    "Europe/Skopje" inherited
    "Africa/Bamako" inherited
    "Pacific/Saipan" inherited
    "Africa/Nouakchott" inherited
    "America/Montserrat" inherited
    "Africa/Blantyre" inherited
    "Asia/Kuala_Lumpur" inherited
    "Africa/Niamey" inherited
    "Europe/Amsterdam" inherited
    "Europe/Oslo" inherited
    "Asia/Muscat" inherited
    "Indian/Reunion" inherited
    "Africa/Kigali" inherited
    "Indian/Mahe" inherited
    "Europe/Stockholm" inherited
    "Atlantic/St_Helena" inherited
    "Europe/Ljubljana" inherited
    "Arctic/Longyearbyen" inherited
    "Europe/Bratislava" inherited
    "Africa/Freetown" inherited
    "Europe/San_Marino" inherited
    "Africa/Dakar" inherited
    "Africa/Mogadishu" inherited
    "America/Lower_Princes" inherited
    "Africa/Mbabane" inherited
    "Indian/Kerguelen" inherited
    "Africa/Lome" inherited
    "America/Port_of_Spain" inherited
    "Pacific/Funafuti" inherited
    "Africa/Dar_es_Salaam" inherited
    "Africa/Kampala" inherited
    "Pacific/Midway" inherited
    "Pacific/Wake" inherited
    "Europe/Vatican" inherited
    "America/St_Vincent" inherited
    "America/Tortola" inherited
    "America/St_Thomas" inherited
    "Pacific/Wallis" inherited
    "Asia/Aden" inherited
    "Indian/Mayotte" inherited
    "Africa/Lusaka" inherited
    "Africa/Harare" inherited
};

// `common/main/te.xml`: 418 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const TE: &str = exemplar_cities! {
    "Europe/Andorra" "అండోరా"
    "Asia/Dubai" "దుబాయి"
    "Asia/Kabul" "కాబుల్"
    "Europe/Tirane" "టిరేన్"
    "Asia/Yerevan" "యెరెవన్"
    "Antarctica/Casey" "కేసీ"
    "Antarctica/Davis" "డెవిస్"
    "Antarctica/Mawson" "మాసన్"
    "Antarctica/Palmer" "పాల్మర్"
    "Antarctica/Rothera" "రొతేరా"
    "Antarctica/Troll" "ట్రోల్"
    "Antarctica/Vostok" "వోస్టోక్"
    "America/Argentina/Buenos_Aires" "బ్యూనోస్ ఎయిర్స్"
    "America/Argentina/Cordoba" "కోర్డోబా"
    "America/Argentina/Salta" "సాల్టా"
    "America/Argentina/Jujuy" "జుజుయ్"
    "America/Argentina/Tucuman" "టుకుమన్"
    "America/Argentina/Catamarca" "కటమార్కా"
    "America/Argentina/La_Rioja" "లా రియోజ"
    "America/Argentina/San_Juan" "శాన్ జ్యూన్"
    "America/Argentina/Mendoza" "మెండోజా"
    "America/Argentina/San_Luis" "శాన్ లూయిస్"
    "America/Argentina/Rio_Gallegos" "రియో గల్లేగోస్"
    "America/Argentina/Ushuaia" "ఉష్యూయ"
    "Pacific/Pago_Pago" "పాగో పాగో"
    "Europe/Vienna" "వియన్నా"
    "Australia/Lord_Howe" "లార్డ్ హౌ దీవి"
    "Antarctica/Macquarie" "మకారీ దీవి"
    "Australia/Hobart" "హోబర్ట్"
    "Australia/Melbourne" "మెల్బోర్న్"
    "Australia/Sydney" "సిడ్నీ"
    "Australia/Broken_Hill" "బ్రోకెన్ హిల్"
    "Australia/Brisbane" "బ్రిస్‌బెయిన్"
    "Australia/Lindeman" "లిండెమాన్"
    "Australia/Adelaide" "అడెలైడ్"
    "Australia/Darwin" "డార్విన్"
    "Australia/Perth" "పెర్త్"
    "Australia/Eucla" "యుక్లా"
    "Asia/Baku" "బాకు"
    "America/Barbados" "బార్బడోస్"
    "Asia/Dhaka" "ఢాకా"
    "Europe/Brussels" "బ్రస్సెల్స్"
    "Europe/Sofia" "సోఫియా"
    "Atlantic/Bermuda" "బెర్ముడా"
    "America/La_Paz" "లా పాజ్"
    "America/Noronha" "నరోన్హా"
    "America/Belem" "బెలెమ్"
    "America/Fortaleza" "ఫోర్టలేజా"
    "America/Recife" "రెసిఫీ"
    "America/Araguaina" "అరాగ్వేయీనా"
    "America/Maceio" "మాసియో"
    "America/Bahia" "బహియ"
    "America/Sao_Paulo" "సావో పాలో"
    "America/Campo_Grande" "కాంపో గ్రాండ్"
    "America/Cuiaba" "కుయబా"
    "America/Santarem" "సాంటరెమ్"
    "America/Porto_Velho" "పోర్టో వెల్హో"
    "America/Boa_Vista" "బోవా విస్టా"
    "America/Manaus" "మనాస్"
    "America/Eirunepe" "ఇరునెప్"
    "America/Rio_Branco" "రియో బ్రాంకో"
    "Asia/Thimphu" "థింఫు"
    "Europe/Minsk" "మిన్స్క్"
    "America/Belize" "బెలీజ్"
    "America/St_Johns" "సెయింట్ జాన్స్"
    "America/Halifax" "హాలిఫాక్స్"
    "America/Glace_Bay" "గ్లేస్ బే"
    "America/Moncton" "మోన్‌క్టోన్"
    "America/Goose_Bay" "గూస్ బే"
    "America/Toronto" "టొరంటో"
    "America/Iqaluit" "ఇక్వాలిట్"
    "America/Winnipeg" "విన్నిపెగ్"
    "America/Resolute" "రిజల్యూట్"
    "America/Rankin_Inlet" "రన్‌కిన్ ఇన్‌లెట్"
    "America/Regina" "రెజీనా"
    "America/Swift_Current" "స్విఫ్ట్ కరెంట్"
    "America/Edmonton" "ఎడ్మోంటన్"
    "America/Cambridge_Bay" "కేంబ్రిడ్జ్ బే"
    "America/Inuvik" "ఇనువిక్"
    "America/Vancouver" "వాన్కూవర్"
    "America/Dawson_Creek" "డాసన్ క్రీక్"
    "America/Fort_Nelson" "ఫోర్ట్ నెల్సన్"
    "America/Whitehorse" "వైట్‌హార్స్"
    "America/Dawson" "డాసన్"
    "Europe/Zurich" "జ్యూరిచ్"
    "Africa/Abidjan" "అబిడ్జాన్"
    "Pacific/Rarotonga" "రరోటోంగా"
    "America/Santiago" "శాంటియాగో"
    "America/Coyhaique" "కొయాయ్కె"
    "America/Punta_Arenas" "పుంటా అరీనస్"
    "Pacific/Easter" "ఈస్టర్"
    "Asia/Shanghai" "షాంఘై"
    "Asia/Urumqi" "ఉరుమ్‌కీ"
    "America/Bogota" "బగోటా"
    "America/Costa_Rica" "కోస్టా రికా"
    "America/Havana" "హవానా"
    "Atlantic/Cape_Verde" "కేప్ వెర్డె"
    "Asia/Nicosia" "నికోసియా"
    "Asia/Famagusta" "ఫామగుస్టా"
    "Europe/Prague" "ప్రాగ్"
    "Europe/Berlin" "బెర్లిన్"
    "America/Santo_Domingo" "శాంటో డోమింగో"
    "Africa/Algiers" "అల్జియర్స్"
    "America/Guayaquil" "గయాక్విల్"
    "Pacific/Galapagos" "గాలాపాగోస్"
    "Europe/Tallinn" "తాల్లిన్"
    "Africa/Cairo" "కైరో"
    "Africa/El_Aaiun" "ఎల్ ఎయున్"
    "Europe/Madrid" "మాడ్రిడ్"
    "Africa/Ceuta" "స్యూటా"
    "Atlantic/Canary" "కెనరీ"
    "Europe/Helsinki" "హెల్సింకి"
    "Pacific/Fiji" "ఫీజీ"
    "Atlantic/Stanley" "స్టాన్లీ"
    "Pacific/Kosrae" "కోస్రే"
    "Atlantic/Faroe" "ఫారో"
    "Europe/Paris" "ప్యారిస్"
    "Europe/London" "లండన్"
    "Asia/Tbilisi" "టిబిలిసి"
    "America/Cayenne" "కయేన్"
    "Europe/Gibraltar" "జిబ్రాల్టర్"
    "America/Nuuk" "నూక్"
    "America/Danmarkshavn" "డెన్మార్క్‌షాన్"
    "America/Scoresbysund" "ఇటోక్కోర్టూర్మిట్"
    "America/Thule" "థులే"
    "Europe/Athens" "ఏథెన్స్"
    "Atlantic/South_Georgia" "దక్షిణ జార్జియా"
    "America/Guatemala" "గ్వాటిమాలా"
    "Pacific/Guam" "గ్వామ్"
    "Africa/Bissau" "బిస్సావ్"
    "America/Guyana" "గయానా"
    "Asia/Hong_Kong" "హాంకాంగ్"
    "America/Tegucigalpa" "తెగుసిగల్పా"
    "America/Port-au-Prince" "పోర్ట్-అవ్-ప్రిన్స్"
    "Europe/Budapest" "బుడాపెస్ట్"
    "Asia/Jakarta" "జకార్తా"
    "Asia/Pontianak" "పొన్టియనాక్"
    "Asia/Makassar" "మకాస్సర్"
    "Asia/Jayapura" "జయపుర"
    "Europe/Dublin" "డబ్లిన్"
    "Asia/Jerusalem" "జరూసలేం"
    "Asia/Kolkata" "కోల్‌కతా"
    "Indian/Chagos" "చాగోస్"
    "Asia/Baghdad" "బాగ్దాద్"
    "Asia/Tehran" "టెహ్రాన్"
    "Europe/Rome" "రోమ్"
    "America/Jamaica" "జమైకా"
    "Asia/Amman" "అమ్మన్"
    "Asia/Tokyo" "టోక్యో"
    "Africa/Nairobi" "నైరోబీ"
    "Asia/Bishkek" "బిష్కెక్"
    "Pacific/Tarawa" "టరావా"
    "Pacific/Kanton" "క్యాంటన్ దీవి"
    "Pacific/Kiritimati" "కిరీటిమాటి"
    "Asia/Pyongyang" "ప్యోంగాంగ్"
    "Asia/Seoul" "సియోల్"
    "Asia/Almaty" "ఆల్మాటి"
    "Asia/Qyzylorda" "క్విజిలోర్డా"
    "Asia/Qostanay" "కోస్తానే"
    "Asia/Aqtobe" "అక్టోబ్"
    "Asia/Aqtau" "అక్టావ్"
    "Asia/Atyrau" "ఆటిరా"
    "Asia/Oral" "ఓరల్"
    "Asia/Beirut" "బీరట్"
    "Asia/Colombo" "కొలంబో"
    "Africa/Monrovia" "మోన్రోవియా"
    "Europe/Vilnius" "విల్నియస్"
    "Europe/Riga" "రీగా"
    "Africa/Tripoli" "ట్రిపోలి"
    "Africa/Casablanca" "కాసాబ్లాంకా"
    "Europe/Chisinau" "చిసినావ్"
    "Pacific/Kwajalein" "క్వాజాలైన్"
    "Asia/Yangon" "యాంగన్"
    "Asia/Ulaanbaatar" "ఉలాన్బాటర్"
    "Asia/Hovd" "హోవ్డ్"
    "Asia/Macau" "మకావ్"
    "America/Martinique" "మార్టినీక్"
    "Europe/Malta" "మాల్టా"
    "Indian/Mauritius" "మారిషస్"
    "Indian/Maldives" "మాల్దీవులు"
    "America/Mexico_City" "మెక్సికో నగరం"
    "America/Cancun" "కన్‌కూన్"
    "America/Merida" "మెరిడా"
    "America/Monterrey" "మోంటెర్రే"
    "America/Matamoros" "మాటమొరోస్"
    "America/Chihuahua" "చువావా"
    "America/Ciudad_Juarez" "సియుదాద్ హ్వారెజ్"
    "America/Ojinaga" "ఒజినగ"
    "America/Mazatlan" "మాసట్‌లాన్"
    "America/Bahia_Banderas" "బహియా బండరాస్"
    "America/Hermosillo" "హెర్మోసిల్లో"
    "America/Tijuana" "టిజువానా"
    "Asia/Kuching" "కుచింగ్"
    "Africa/Maputo" "మాపుటో"
    "Africa/Windhoek" "విండ్హోక్"
    "Pacific/Noumea" "నౌమియా"
    "Pacific/Norfolk" "నార్ఫక్ దీవి"
    "Africa/Lagos" "లాగోస్"
    "America/Managua" "మనాగువా"
    "Asia/Kathmandu" "ఖాట్మండు"
    "Pacific/Nauru" "నౌరు"
    "Pacific/Niue" "నియూ"
    "Pacific/Auckland" "ఆక్లాండ్"
    "Pacific/Chatham" "చాథమ్ దీవులు"
    "America/Panama" "పనామా"
    "America/Lima" "లిమా"
    "Pacific/Tahiti" "తహితి"
    "Pacific/Marquesas" "మార్క్వేసాస్"
    "Pacific/Gambier" "గాంబియేర్"
    "Pacific/Port_Moresby" "పోర్ట్ మోరెస్బే"
    "Pacific/Bougainville" "బొగెయిన్‌విల్లే"
    "Asia/Manila" "మనీలా"
    "Asia/Karachi" "కరాచీ"
    "Europe/Warsaw" "వార్షా"
    "America/Miquelon" "మికెలాన్"
    "Pacific/Pitcairn" "పిట్‌కైర్న్"
    "America/Puerto_Rico" "ప్యూర్టో రికో"
    "Asia/Gaza" "గాజా"
    "Asia/Hebron" "హెబ్రాన్"
    "Europe/Lisbon" "లిస్బన్"
    "Atlantic/Madeira" "మదైరా"
    "Atlantic/Azores" "అజోర్స్"
    "Pacific/Palau" "పాలావ్"
    "America/Asuncion" "అసున్సియోన్"
    "Asia/Qatar" "ఖతార్"
    "Europe/Bucharest" "బుకారెస్ట్"
    "Europe/Belgrade" "బెల్‌గ్రేడ్"
    "Europe/Kaliningrad" "కలినిన్‌గ్రద్"
    "Europe/Moscow" "మాస్కో"
    "Europe/Simferopol" "సిమ్‌ఫెరోపోల్"
    "Europe/Kirov" "కిరోవ్"
    "Europe/Volgograd" "వోల్గోగ్రాడ్"
    "Europe/Astrakhan" "అస్ట్రఖాన్"
    "Europe/Saratov" "సరాటవ్"
    "Europe/Ulyanovsk" "ఉల్యనోవ్స్క్"
    "Europe/Samara" "సమార"
    "Asia/Yekaterinburg" "యెకటెరింబర్గ్"
    "Asia/Omsk" "ఓమ్స్క్"
    "Asia/Novosibirsk" "నవోసిబిర్స్క్"
    "Asia/Barnaul" "బార్నాల్"
    "Asia/Tomsk" "టామ్స్క్"
    "Asia/Novokuznetsk" "నొవొకుజ్‌నెట్‌స్క్"
    "Asia/Krasnoyarsk" "క్రసనోయార్స్క్"
    "Asia/Irkutsk" "ఇర్కుట్స్క్"
    "Asia/Chita" "చితా"
    "Asia/Yakutsk" "యకుట్స్క్"
    "Asia/Khandyga" "కంద్యాగ"
    "Asia/Vladivostok" "వ్లాడివోస్టోక్"
    "Asia/Ust-Nera" "అస్ట్-నెరా"
    "Asia/Magadan" "మగడాన్"
    "Asia/Sakhalin" "సఖాలిన్"
    "Asia/Srednekolymsk" "స్రెడ్నెకొలిమ్స్క్"
    "Asia/Kamchatka" "కమ్‌చత్కా"
    "Asia/Anadyr" "అనడైర్"
    "Asia/Riyadh" "రియాధ్"
    "Pacific/Guadalcanal" "గ్వాడల్కెనాల్"
    "Africa/Khartoum" "ఖార్టోమ్"
    "Asia/Singapore" "సింగపూర్"
    "America/Paramaribo" "పరామారిబో"
    "Africa/Juba" "జుబా"
    "Africa/Sao_Tome" "సావో టోమ్"
    "America/El_Salvador" "ఎల్ సాల్వడోర్"
    "Asia/Damascus" "డమాస్కస్"
    "America/Grand_Turk" "గ్రాండ్ టర్క్"
    "Africa/Ndjamena" "డ్జామెనా"
    "Asia/Bangkok" "బ్యాంకాక్"
    "Asia/Dushanbe" "డుషన్బీ"
    "Pacific/Fakaofo" "ఫాకోఫో"
    "Asia/Dili" "డిలి"
    "Asia/Ashgabat" "యాష్గాబాట్"
    "Africa/Tunis" "ట్యునిస్"
    "Pacific/Tongatapu" "టోంగాటాపు"
    "Europe/Istanbul" "ఇస్తాంబుల్"
    "Asia/Taipei" "తైపీ"
    "Europe/Kyiv" "కీవ్"
    "America/New_York" "న్యూయార్క్"
    "America/Detroit" "డిట్రోయిట్"
    "America/Kentucky/Louisville" "లూయివిల్"
    "America/Kentucky/Monticello" "మోంటిసెల్లో, కెన్‌టుక్కీ"
    "America/Indiana/Indianapolis" "ఇండియానపోలిస్"
    "America/Indiana/Vincennes" "విన్‌సెన్నెస్, ఇండియాన"
    "America/Indiana/Winamac" "వినామాక్, ఇండియాన"
    "America/Indiana/Marengo" "మరెంగో, ఇండియాన"
    "America/Indiana/Petersburg" "పీటర్స్‌బర్గ్, ఇండియాన"
    "America/Indiana/Vevay" "వెవయ్, ఇండియాన"
    "America/Chicago" "చికాగో"
    "America/Indiana/Tell_City" "టెల్ నగరం, ఇండియాన"
    "America/Indiana/Knox" "నోక్స్, ఇండియాన"
    "America/Menominee" "మెనోమినీ"
    "America/North_Dakota/Center" "సెంటర్, ఉత్తర డకోటా"
    "America/North_Dakota/New_Salem" "న్యూ సలేమ్, ఉత్తర డకోట"
    "America/North_Dakota/Beulah" "బ్యులా, ఉత్తర డకోట"
    "America/Denver" "డెన్వెర్"
    "America/Boise" "బొయిసీ"
    "America/Phoenix" "ఫినిక్స్"
    "America/Los_Angeles" "లాస్ ఏంజల్స్"
    "America/Anchorage" "యాంకరేజ్"
    "America/Juneau" "జూనో"
    "America/Sitka" "సిట్కా"
    "America/Metlakatla" "మెట్లకట్ల"
    "America/Yakutat" "యకుటాట్"
    "America/Nome" "నోమ్"
    "America/Adak" "అడాక్"
    "Pacific/Honolulu" "హోనోలులు"
    "America/Montevideo" "మోంటెవీడియో"
    "Asia/Samarkand" "సమర్కాండ్"
    "Asia/Tashkent" "తాష్కెంట్"
    "America/Caracas" "కారాకస్"
    "Asia/Ho_Chi_Minh" "హో చి మిన్హ్ నగరం"
    "Pacific/Efate" "ఇఫేట్"
    "Pacific/Apia" "ఏపియా"
    "Africa/Johannesburg" "జొహెన్స్‌బర్గ్"
    "America/Antigua" "ఆంటిగ్వా"
    "America/Anguilla" "ఆంగ్విల్లా"
    "Africa/Luanda" "లువాండా"
    "Antarctica/McMurdo" "మెక్‌ముర్డో"
    "Antarctica/DumontDUrville" "డ్యూమాంట్ డి’ఉర్విల్లే"
    "Antarctica/Syowa" "స్యోవా"
    "America/Aruba" "అరుబా"
    "Europe/Mariehamn" "మారీయుహమ్"
    "Europe/Sarajevo" "సరాజోవో"
    "Africa/Ouagadougou" "ఔగాడౌగోవ్"
    "Asia/Bahrain" "బహ్రెయిన్"
    "Africa/Bujumbura" "బుజమ్బురా"
    "Africa/Porto-Novo" "పోర్టో-నోవో"
    "America/St_Barthelemy" "సెయింట్ బర్తెలెమీ"
    "Asia/Brunei" "బ్రూనై"
    "America/Kralendijk" "క్రలెండ్జిక్"
    "America/Nassau" "నాస్సావ్"
    "Africa/Gaborone" "గబోరోన్"
    "America/Blanc-Sablon" "బ్లాంక్-సబ్లోన్"
    "America/Atikokan" "అటికోకన్"
    "America/Creston" "క్రెస్టన్"
    "Indian/Cocos" "కోకోస్ దీవులు"
    "Africa/Kinshasa" "కిన్షాసా"
    "Africa/Lubumbashi" "లుబంబాషి"
    "Africa/Bangui" "బాంగుయ్"
    "Africa/Brazzaville" "బ్రాజావిల్లే"
    "Africa/Douala" "డౌలా"
    "America/Curacao" "కురాకవో"
    "Indian/Christmas" "క్రిస్మస్ దీవి"
    "Europe/Busingen" "బసింజన్"
    "Africa/Djibouti" "జిబూటి"
    "Europe/Copenhagen" "కోపెన్హాగన్"
    "America/Dominica" "డొమినికా"
    "Africa/Asmara" "అస్మారా"
    "Africa/Addis_Ababa" "యాడిస్ అబాబా"
    "Pacific/Chuuk" "చుక్"
    "Pacific/Pohnpei" "పోన్‌పై"
    "Africa/Libreville" "లెబర్విల్లే"
    "America/Grenada" "గ్రెనడా"
    "Europe/Guernsey" "గ్వెర్న్సే"
    "Africa/Accra" "అక్రా"
    "Africa/Banjul" "బంజూల్"
    "Africa/Conakry" "కోనాక్రీ"
    "America/Guadeloupe" "గ్వాడెలోప్"
    "Africa/Malabo" "మలాబో"
    "Europe/Zagreb" "జాగ్రెబ్"
    "Europe/Isle_of_Man" "ఐల్ ఆఫ్ మేన్"
    "Atlantic/Reykjavik" "రెక్జావిక్"
    "Europe/Jersey" "జెర్సీ"
    "Asia/Phnom_Penh" "నోమ్‌పెన్హ్"
    "Indian/Comoro" "కొమోరో"
    "America/St_Kitts" "సెయింట్ కిట్స్"
    "Asia/Kuwait" "కువైట్"
    "America/Cayman" "కేమాన్"
    "Asia/Vientiane" "వియన్టైన్"
    "America/St_Lucia" "సెయింట్ లూసియా"
    "Europe/Vaduz" "వాడుజ్"
    "Africa/Maseru" "మసేరు"
    "Europe/Luxembourg" "లక్సెంబర్గ్"
    "Europe/Monaco" "మొనాకో"
    "Europe/Podgorica" "పోడ్గోరికా"
    "America/Marigot" "మారిగోట్"
    "Indian/Antananarivo" "అంటానానారివో"
    "Pacific/Majuro" "మజురో"
    "Europe/Skopje" "స్కోప్‌యే"
    "Africa/Bamako" "బామాకో"
    "Pacific/Saipan" "సాయ్పాన్"
    "Africa/Nouakchott" "న్వాక్షోట్"
    "America/Montserrat" "మాంట్సెరాట్"
    "Africa/Blantyre" "బ్లాన్టైర్"
    "Asia/Kuala_Lumpur" "కౌలాలంపూర్"
    "Africa/Niamey" "నియామే"
    "Europe/Amsterdam" "ఆమ్‌స్టర్‌డామ్"
    "Europe/Oslo" "ఓస్లో"
    "Asia/Muscat" "మస్కట్"
    "Indian/Reunion" "రీయూనియన్"
    "Africa/Kigali" "కీగలి"
    "Indian/Mahe" "మాహె"
    "Europe/Stockholm" "స్టాక్హోమ్"
    "Atlantic/St_Helena" "సెయింట్ హెలెనా"
    "Europe/Ljubljana" "ల్యూబ్ల్యానా"
    "Arctic/Longyearbyen" "లాంగ్‌యియర్‌బైయన్"
    "Europe/Bratislava" "బ్రాటిస్లావా"
    "Africa/Freetown" "ఫ్రీటౌన్"
    "Europe/San_Marino" "శాన్ మారినో"
    "Africa/Dakar" "డకార్"
    "Africa/Mogadishu" "మోగాదిషు"
    "America/Lower_Princes" "లోయర్ ప్రిన్స్ క్వార్టర్"
    "Africa/Mbabane" "బాబెన్"
    "Indian/Kerguelen" "కెర్గ్యూలెన్"
    "Africa/Lome" "లోమ్"
    "America/Port_of_Spain" "పోర్ట్ ఆఫ్ స్పెయిన్"
    "Pacific/Funafuti" "ఫునాఫుటి"
    "Africa/Dar_es_Salaam" "దార్ ఎస్ సలామ్"
    "Africa/Kampala" "కంపాలా"
    "Pacific/Midway" "మిడ్వే"
    "Pacific/Wake" "వేక్ దీవి"
    "Europe/Vatican" "వాటికన్"
    "America/St_Vincent" "సెయింట్ విన్సెంట్"
    "America/Tortola" "టోర్టోలా"
    "America/St_Thomas" "సెయింట్ థామస్"
    "Pacific/Wallis" "వాల్లిస్ & ఫ్యూటునా"
    "Asia/Aden" "ఎడెన్"
    "Indian/Mayotte" "మయోట్"
    "Africa/Lusaka" "లుసాకా"
    "Africa/Harare" "హరారే"
};

// `common/main/ur.xml`: 418 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const UR: &str = exemplar_cities! {
    "Europe/Andorra" "انڈورا"
    "Asia/Dubai" "دبئی"
    "Asia/Kabul" "کابل"
    "Europe/Tirane" "ٹیرانی"
    "Asia/Yerevan" "یریوان"
    "Antarctica/Casey" "کیسی"
    "Antarctica/Davis" "ڈیوس"
    "Antarctica/Mawson" "ماؤسن"
    "Antarctica/Palmer" "پلمیر"
    "Antarctica/Rothera" "روتھیرا"
    "Antarctica/Troll" "ٹرول"
    "Antarctica/Vostok" "ووستوک"
    "America/Argentina/Buenos_Aires" "بیونس آئرس"
    "America/Argentina/Cordoba" "کورڈوبا"
    "America/Argentina/Salta" "سالٹا"
    "America/Argentina/Jujuy" "جوجوئی"
    "America/Argentina/Tucuman" "ٹوکومین"
    "America/Argentina/Catamarca" "کیٹامارکا"
    "America/Argentina/La_Rioja" "لا ریئوجا"
    "America/Argentina/San_Juan" "سان جوآن"
    "America/Argentina/Mendoza" "مینڈوزا"
    "America/Argentina/San_Luis" "سان لوئس"
    "America/Argentina/Rio_Gallegos" "ریو گالیگوس"
    "America/Argentina/Ushuaia" "اوشوآئیا"
    "Pacific/Pago_Pago" "پاگو پاگو"
    "Europe/Vienna" "ویانا"
    "Australia/Lord_Howe" "لارڈ ہووے"
    "Antarctica/Macquarie" "میکواری"
    "Australia/Hobart" "ہوبارٹ"
    "Australia/Melbourne" "ملبورن"
    "Australia/Sydney" "سڈنی"
    "Australia/Broken_Hill" "بروکن ہِل"
    "Australia/Brisbane" "برسبین"
    "Australia/Lindeman" "لِنڈمین"
    "Australia/Adelaide" "ایڈیلیڈ"
    "Australia/Darwin" "ڈارون"
    "Australia/Perth" "پرتھ"
    "Australia/Eucla" "ایوکلا"
    "Asia/Baku" "باکو"
    "America/Barbados" "بارباڈوس"
    "Asia/Dhaka" "ڈھاکہ"
    "Europe/Brussels" "برسلز"
    "Europe/Sofia" "صوفیہ"
    "Atlantic/Bermuda" "برمودا"
    "America/La_Paz" "لا پاز"
    "America/Noronha" "نورونہا"
    "America/Belem" "بیلیم"
    "America/Fortaleza" "فورٹالیزا"
    "America/Recife" "ریسائف"
    "America/Araguaina" "اراگویانا"
    "America/Maceio" "میسیئو"
    "America/Bahia" "باہیا"
    "America/Sao_Paulo" "ساؤ پالو"
    "America/Campo_Grande" "کیمپو گرینڈ"
    "America/Cuiaba" "کوئیابا"
    "America/Santarem" "سنٹارین"
    "America/Porto_Velho" "پورٹو ویلہو"
    "America/Boa_Vista" "بوآ وسٹا"
    "America/Manaus" "مناؤس"
    "America/Eirunepe" "ایرونیپ"
    "America/Rio_Branco" "ریئو برینکو"
    "Asia/Thimphu" "تھمپو"
    "Europe/Minsk" "مِنسک"
    "America/Belize" "بیلائز"
    "America/St_Johns" "سینٹ جانز"
    "America/Halifax" "ہیلیفیکس"
    "America/Glace_Bay" "گلیس کی کھاڑی"
    "America/Moncton" "مونکٹن"
    "America/Goose_Bay" "گوس کی کھاڑی"
    "America/Toronto" "ٹورنٹو"
    "America/Iqaluit" "ایکالوئٹ"
    "America/Winnipeg" "ونّیپیگ"
    "America/Resolute" "ریزولیوٹ"
    "America/Rankin_Inlet" "رینکن انلیٹ"
    "America/Regina" "ریجینا"
    "America/Swift_Current" "سوِفٹ کرنٹ"
    "America/Edmonton" "ایڈمونٹن"
    "America/Cambridge_Bay" "کیمبرج کی کھاڑی"
    "America/Inuvik" "انووِک"
    "America/Vancouver" "وینکوور"
    "America/Dawson_Creek" "ڈاؤسن کریک"
    "America/Fort_Nelson" "فورٹ نیلسن"
    "America/Whitehorse" "وہائٹ ہارس"
    "America/Dawson" "ڈاؤسن"
    "Europe/Zurich" "زیورخ"
    "Africa/Abidjan" "عابدجان"
    "Pacific/Rarotonga" "راروٹونگا"
    "America/Santiago" "سنٹیاگو"
    "America/Coyhaique" "کویائیکے"
    "America/Punta_Arenas" "پنٹا اریناس"
    "Pacific/Easter" "ایسٹر"
    "Asia/Shanghai" "شنگھائی"
    "Asia/Urumqi" "یورومکی"
    "America/Bogota" "بگوٹا"
    "America/Costa_Rica" "کوسٹا ریکا"
    "America/Havana" "ہوانا"
    "Atlantic/Cape_Verde" "کیپ ورڈی"
    "Asia/Nicosia" "نکوسیا"
    "Asia/Famagusta" "فاماگوسٹا"
    "Europe/Prague" "پراگ"
    "Europe/Berlin" "برلن"
    "America/Santo_Domingo" "سانتو ڈومنگو"
    "Africa/Algiers" "الجیئرس"
    "America/Guayaquil" "گوآیاکوئل"
    "Pacific/Galapagos" "گیلاپیگوس"
    "Europe/Tallinn" "ٹالن"
    "Africa/Cairo" "قاہرہ"
    "Africa/El_Aaiun" "العیون"
    "Europe/Madrid" "میڈرڈ"
    "Africa/Ceuta" "سیوٹا"
    "Atlantic/Canary" "کینری"
    "Europe/Helsinki" "ہیلسنکی"
    "Pacific/Fiji" "فجی"
    "Atlantic/Stanley" "اسٹینلے"
    "Pacific/Kosrae" "کوسرائی"
    "Atlantic/Faroe" "فارو"
    "Europe/Paris" "پیرس"
    "Europe/London" "لندن"
    "Asia/Tbilisi" "طبلیسی"
    "America/Cayenne" "کائین"
    "Europe/Gibraltar" "جبل الطارق"
    "America/Nuuk" "نوک"
    "America/Danmarkshavn" "ڈنمارک شاون"
    "America/Scoresbysund" "اسکورز بائی سنڈ"
    "America/Thule" "تھولو"
    "Europe/Athens" "ایتھنز"
    "Atlantic/South_Georgia" "جنوبی جارجیا"
    "America/Guatemala" "گواٹے مالا"
    "Pacific/Guam" "گوآم"
    "Africa/Bissau" "بِساؤ"
    "America/Guyana" "گیانا"
    "Asia/Hong_Kong" "ہانگ کانگ"
    "America/Tegucigalpa" "ٹیگوسیگالپے"
    "America/Port-au-Prince" "پورٹ او پرنس"
    "Europe/Budapest" "بڈاپسٹ"
    "Asia/Jakarta" "جکارتہ"
    "Asia/Pontianak" "پونٹیانک"
    "Asia/Makassar" "مکاسر"
    "Asia/Jayapura" "جے پورہ"
    "Europe/Dublin" "ڈبلن"
    "Asia/Jerusalem" "یروشلم"
    "Asia/Kolkata" "کولکاتا"
    "Indian/Chagos" "چاگوس"
    "Asia/Baghdad" "بغداد"
    "Asia/Tehran" "تہران"
    "Europe/Rome" "روم"
    "America/Jamaica" "جمائیکا"
    "Asia/Amman" "امّان"
    "Asia/Tokyo" "ٹوکیو"
    "Africa/Nairobi" "نیروبی"
    "Asia/Bishkek" "بشکیک"
    "Pacific/Tarawa" "ٹراوا"
    "Pacific/Kanton" "کانٹن"
    "Pacific/Kiritimati" "کریتیماٹی"
    "Asia/Pyongyang" "پیونگ یانگ"
    "Asia/Seoul" "سیئول"
    "Asia/Almaty" "الماٹی"
    "Asia/Qyzylorda" "کیزیلورڈا"
    "Asia/Qostanay" "کوستانے"
    "Asia/Aqtobe" "اکٹوب"
    "Asia/Aqtau" "اکتاؤ"
    "Asia/Atyrau" "آتیراؤ"
    "Asia/Oral" "اورال"
    "Asia/Beirut" "بیروت"
    "Asia/Colombo" "کولمبو"
    "Africa/Monrovia" "مونروویا"
    "Europe/Vilnius" "وِلنیئس"
    "Europe/Riga" "ریگا"
    "Africa/Tripoli" "ٹریپولی"
    "Africa/Casablanca" "کیسا بلانکا"
    "Europe/Chisinau" "چیسیناؤ"
    "Pacific/Kwajalein" "کواجیلین"
    "Asia/Yangon" "رنگون"
    "Asia/Ulaanbaatar" "اولان باتار"
    "Asia/Hovd" "ہووارڈ"
    "Asia/Macau" "مکاؤ"
    "America/Martinique" "مارٹینک"
    "Europe/Malta" "مالٹا"
    "Indian/Mauritius" "ماریشس"
    "Indian/Maldives" "مالدیپ"
    "America/Mexico_City" "میکسیکو سٹی"
    "America/Cancun" "کنکیون"
    "America/Merida" "میریڈا"
    "America/Monterrey" "مونٹیری"
    "America/Matamoros" "میٹاموروس"
    "America/Chihuahua" "چیہوآہوآ"
    "America/Ciudad_Juarez" "سیوداد جیوریز"
    "America/Ojinaga" "اوجیناگا"
    "America/Mazatlan" "میزٹلان"
    "America/Bahia_Banderas" "بہیا بندراز"
    "America/Hermosillo" "ہرموسیلو"
    "America/Tijuana" "تیجوآنا"
    "Asia/Kuching" "کیوچنگ"
    "Africa/Maputo" "مپوٹو"
    "Africa/Windhoek" "ونڈہوک"
    "Pacific/Noumea" "نؤمیا"
    "Pacific/Norfolk" "نورفوک"
    "Africa/Lagos" "لاگوس"
    "America/Managua" "مناگوآ"
    "Asia/Kathmandu" "کاٹھمنڈو"
    "Pacific/Nauru" "ناؤرو"
    "Pacific/Niue" "نیئو"
    "Pacific/Auckland" "آکلینڈ"
    "Pacific/Chatham" "چیتھم"
    "America/Panama" "پنامہ"
    "America/Lima" "لیما"
    "Pacific/Tahiti" "تاہیتی"
    "Pacific/Marquesas" "مارکیساس"
    "Pacific/Gambier" "گامبیئر"
    "Pacific/Port_Moresby" "پورٹ موریسبی"
    "Pacific/Bougainville" "بوگینولے"
    "Asia/Manila" "منیلا"
    "Asia/Karachi" "کراچی"
    "Europe/Warsaw" "وارسا"
    "America/Miquelon" "میکلیئون"
    "Pacific/Pitcairn" "پٹکائرن"
    "America/Puerto_Rico" "پیورٹو ریکو"
    "Asia/Gaza" "غزہ"
    "Asia/Hebron" "ہیبرون"
    "Europe/Lisbon" "لسبن"
    "Atlantic/Madeira" "مڈیئرا"
    "Atlantic/Azores" "ازوریس"
    "Pacific/Palau" "پلاؤ"
    "America/Asuncion" "اسنسیئن"
    "Asia/Qatar" "قطر"
    "Europe/Bucharest" "بخارسٹ"
    "Europe/Belgrade" "بلغراد"
    "Europe/Kaliningrad" "کالينينغراد"
    "Europe/Moscow" "ماسکو"
    "Europe/Simferopol" "سمفروپول"
    "Europe/Kirov" "کیروف"
    "Europe/Volgograd" "وولگوگراد"
    "Europe/Astrakhan" "استراخان"
    "Europe/Saratov" "سیراٹو"
    "Europe/Ulyanovsk" "الیانوسک"
    "Europe/Samara" "سمارا"
    "Asia/Yekaterinburg" "یکاٹیرِنبرگ"
    "Asia/Omsk" "اومسک"
    "Asia/Novosibirsk" "نوووسِبِرسک"
    "Asia/Barnaul" "برنال"
    "Asia/Tomsk" "ٹامسک"
    "Asia/Novokuznetsk" "نوووکیوزنیسک"
    "Asia/Krasnoyarsk" "کریسنویارسک"
    "Asia/Irkutsk" "ارکتسک"
    "Asia/Chita" "چیتا"
    "Asia/Yakutsk" "یکوتسک"
    "Asia/Khandyga" "خندیگا"
    "Asia/Vladivostok" "ولادی ووستک"
    "Asia/Ust-Nera" "اوست-نیرا"
    "Asia/Magadan" "میگیدن"
    "Asia/Sakhalin" "سخالین"
    "Asia/Srednekolymsk" "سرہدنیکولیمسک"
    "Asia/Kamchatka" "کیمچٹکا"
    "Asia/Anadyr" "انیدر"
    "Asia/Riyadh" "ریاض"
    "Pacific/Guadalcanal" "گواڈل کینال"
    "Africa/Khartoum" "خرطوم"
    "Asia/Singapore" "سنگاپور"
    "America/Paramaribo" "پراماریبو"
    "Africa/Juba" "جوبا"
    "Africa/Sao_Tome" "ساؤ ٹوم"
    "America/El_Salvador" "ال سلواڈور"
    "Asia/Damascus" "دمشق"
    "America/Grand_Turk" "عظیم ترک"
    "Africa/Ndjamena" "اینجامینا"
    "Asia/Bangkok" "بنکاک"
    "Asia/Dushanbe" "دوشانبے"
    "Pacific/Fakaofo" "فکاؤفو"
    "Asia/Dili" "ڈلی"
    "Asia/Ashgabat" "اشغبت"
    "Africa/Tunis" "تیونس"
    "Pacific/Tongatapu" "ٹونگاٹاپو"
    "Europe/Istanbul" "استنبول"
    "Asia/Taipei" "تائپے"
    "Europe/Kyiv" "کیو"
    "America/New_York" "نیو یارک"
    "America/Detroit" "ڈیٹرائٹ"
    "America/Kentucky/Louisville" "لوئس ویلے"
    "America/Kentucky/Monticello" "مونٹیسیلو، کینٹوکی"
    "America/Indiana/Indianapolis" "انڈیاناپولس"
    "America/Indiana/Vincennes" "ونسینیز، انڈیانا"
    "America/Indiana/Winamac" "وینامیک، انڈیانا"
    "America/Indiana/Marengo" "مرینگو، انڈیانا"
    "America/Indiana/Petersburg" "پیٹرزبرگ، انڈیانا"
    "America/Indiana/Vevay" "ویوے، انڈیانا"
    "America/Chicago" "شکاگو"
    "America/Indiana/Tell_City" "ٹیل سٹی، انڈیانا"
    "America/Indiana/Knox" "کنوکس، انڈیانا"
    "America/Menominee" "مینومینی"
    "America/North_Dakota/Center" "وسط، شمالی ڈکوٹا"
    "America/North_Dakota/New_Salem" "نیو سلیم، شمالی ڈکوٹا"
    "America/North_Dakota/Beulah" "بیولاہ، شمالی ڈکوٹا"
    "America/Denver" "ڈینور"
    "America/Boise" "بوائس"
    "America/Phoenix" "فینکس"
    "America/Los_Angeles" "لاس اینجلس"
    "America/Anchorage" "اینکریج"
    "America/Juneau" "جونیئو"
    "America/Sitka" "سیٹکا"
    "America/Metlakatla" "میٹلا کاٹلا"
    "America/Yakutat" "یکوٹیٹ"
    "America/Nome" "نوم"
    "America/Adak" "اداک"
    "Pacific/Honolulu" "ہونولولو"
    "America/Montevideo" "مونٹی ویڈیو"
    "Asia/Samarkand" "سمرقند"
    "Asia/Tashkent" "تاشقند"
    "America/Caracas" "کراکاس"
    "Asia/Ho_Chi_Minh" "ہو چی منہ سٹی"
    "Pacific/Efate" "ایفیٹ"
    "Pacific/Apia" "اپیا"
    "Africa/Johannesburg" "جوہانسبرگ"
    "America/Antigua" "انٹیگوا"
    "America/Anguilla" "انگویلا"
    "Africa/Luanda" "لوانڈا"
    "Antarctica/McMurdo" "میک مرڈو"
    "Antarctica/DumontDUrville" "ڈومونٹ ڈی ارویلے"
    "Antarctica/Syowa" "سیووا"
    "America/Aruba" "اروبا"
    "Europe/Mariehamn" "میریہام"
    "Europe/Sarajevo" "سراجیوو"
    "Africa/Ouagadougou" "اؤگاڈؤگوو"
    "Asia/Bahrain" "بحرین"
    "Africa/Bujumbura" "بجمبرا"
    "Africa/Porto-Novo" "پورٹو نووو"
    "America/St_Barthelemy" "سینٹ برتھیلمی"
    "Asia/Brunei" "برونئی"
    "America/Kralendijk" "کرالینڈیجک"
    "America/Nassau" "نساؤ"
    "Africa/Gaborone" "گبرون"
    "America/Blanc-Sablon" "بلانک سبلون"
    "America/Atikokan" "اٹیکوکن"
    "America/Creston" "کریسٹون"
    "Indian/Cocos" "کوکوس"
    "Africa/Kinshasa" "کنشاسا"
    "Africa/Lubumbashi" "لوبمباشی"
    "Africa/Bangui" "بنگوئی"
    "Africa/Brazzaville" "برازاویلے"
    "Africa/Douala" "ڈوآلا"
    "America/Curacao" "کیوراکاؤ"
    "Indian/Christmas" "کرسمس"
    "Europe/Busingen" "بزنجن"
    "Africa/Djibouti" "جبوتی"
    "Europe/Copenhagen" "کوپن ہیگن"
    "America/Dominica" "ڈومنیکا"
    "Africa/Asmara" "اسمارا"
    "Africa/Addis_Ababa" "عدیس ابابا"
    "Pacific/Chuuk" "چیوک"
    "Pacific/Pohnpei" "پونپیئی"
    "Africa/Libreville" "لبرے ویلے"
    "America/Grenada" "غرناطہ"
    "Europe/Guernsey" "گرنزی"
    "Africa/Accra" "اکّرا"
    "Africa/Banjul" "بنجول"
    "Africa/Conakry" "کونکری"
    "America/Guadeloupe" "گواڈیلوپ"
    "Africa/Malabo" "ملابو"
    "Europe/Zagreb" "زیگریب"
    "Europe/Isle_of_Man" "آئل آف مین"
    "Atlantic/Reykjavik" "ریکجاوک"
    "Europe/Jersey" "جرسی"
    "Asia/Phnom_Penh" "پنوم پن"
    "Indian/Comoro" "کومورو"
    "America/St_Kitts" "سینٹ کٹس"
    "Asia/Kuwait" "کویت"
    "America/Cayman" "کیمین"
    "Asia/Vientiane" "وینٹیانا"
    "America/St_Lucia" "سینٹ لوسیا"
    "Europe/Vaduz" "ویڈوز"
    "Africa/Maseru" "مسیرو"
    "Europe/Luxembourg" "لگژمبرگ"
    "Europe/Monaco" "موناکو"
    "Europe/Podgorica" "پوڈگورسیا"
    "America/Marigot" "میریگوٹ"
    "Indian/Antananarivo" "انٹاناناریوو"
    "Pacific/Majuro" "مجورو"
    "Europe/Skopje" "اسکوپجے"
    "Africa/Bamako" "بماکو"
    "Pacific/Saipan" "سائپین"
    "Africa/Nouakchott" "نواکشوط"
    "America/Montserrat" "مونٹسیراٹ"
    "Africa/Blantyre" "بلینٹائر"
    "Asia/Kuala_Lumpur" "کوالا لمپور"
    "Africa/Niamey" "نیامی"
    "Europe/Amsterdam" "ایمسٹرڈم"
    "Europe/Oslo" "اوسلو"
    "Asia/Muscat" "مسقط"
    "Indian/Reunion" "ری یونین"
    "Africa/Kigali" "کگالی"
    "Indian/Mahe" "ماہی"
    "Europe/Stockholm" "اسٹاک ہوم"
    "Atlantic/St_Helena" "سینٹ ہیلینا"
    "Europe/Ljubljana" "لیوبلیانا"
    "Arctic/Longyearbyen" "لانگ ایئر بین"
    "Europe/Bratislava" "بریٹِسلاوا"
    "Africa/Freetown" "فری ٹاؤن"
    "Europe/San_Marino" "سان ماریانو"
    "Africa/Dakar" "ڈکار"
    "Africa/Mogadishu" "موگادیشو"
    "America/Lower_Princes" "لوور پرنسس کوارٹر"
    "Africa/Mbabane" "مبابین"
    "Indian/Kerguelen" "کرگیولین"
    "Africa/Lome" "لوم"
    "America/Port_of_Spain" "پورٹ آف اسپین"
    "Pacific/Funafuti" "فیونافیوٹی"
    "Africa/Dar_es_Salaam" "دار السلام"
    "Africa/Kampala" "کیمپالا"
    "Pacific/Midway" "مڈوے"
    "Pacific/Wake" "ویک"
    "Europe/Vatican" "واٹیکن"
    "America/St_Vincent" "سینٹ ونسنٹ"
    "America/Tortola" "ٹورٹولا"
    "America/St_Thomas" "سینٹ تھامس"
    "Pacific/Wallis" "ولّیس"
    "Asia/Aden" "عدن"
    "Indian/Mayotte" "مایوٹ"
    "Africa/Lusaka" "لیوساکا"
    "Africa/Harare" "ہرارے"
};

// `common/main/yue_Hans.xml`: 418 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const YUE_HANS: &str = exemplar_cities! {
    "Europe/Andorra" "安道尔"
    "Asia/Dubai" "杜拜"
    "Asia/Kabul" "喀布尔"
    "Europe/Tirane" "地拉那"
    "Asia/Yerevan" "叶里温"
    "Antarctica/Casey" "凯西"
    "Antarctica/Davis" "戴维斯"
    "Antarctica/Mawson" "莫森"
    "Antarctica/Palmer" "帕麦"
    "Antarctica/Rothera" "罗瑟拉"
    "Antarctica/Troll" "绰尔"
    "Antarctica/Vostok" "沃斯托克"
    "America/Argentina/Buenos_Aires" "布宜诺斯艾利斯"
    "America/Argentina/Cordoba" "哥多华"
    "America/Argentina/Salta" "萨尔塔"
    "America/Argentina/Jujuy" "胡胡伊"
    "America/Argentina/Tucuman" "吐库曼"
    "America/Argentina/Catamarca" "卡塔马卡"
    "America/Argentina/La_Rioja" "拉略哈"
    "America/Argentina/San_Juan" "圣胡安"
    "America/Argentina/Mendoza" "门多萨"
    "America/Argentina/San_Luis" "圣路易"
    "America/Argentina/Rio_Gallegos" "里奥加耶戈斯"
    "America/Argentina/Ushuaia" "乌斯怀亚"
    "Pacific/Pago_Pago" "巴哥巴哥"
    "Europe/Vienna" "维也纳"
    "Australia/Lord_Howe" "豪勋爵岛"
    "Antarctica/Macquarie" "麦觉理"
    "Australia/Hobart" "荷巴特"
    "Australia/Melbourne" "墨尔本"
    "Australia/Sydney" "雪梨"
    "Australia/Broken_Hill" "布罗肯希尔"
    "Australia/Brisbane" "布利斯班"
    "Australia/Lindeman" "林德曼"
    "Australia/Adelaide" "阿得雷德"
    "Australia/Darwin" "达尔文"
    "Australia/Perth" "伯斯"
    "Australia/Eucla" "尤克拉"
    "Asia/Baku" "巴库"
    "America/Barbados" "巴贝多"
    "Asia/Dhaka" "达卡"
    "Europe/Brussels" "布鲁塞尔"
    "Europe/Sofia" "索菲亚"
    "Atlantic/Bermuda" "百慕达"
    "America/La_Paz" "拉巴斯"
    "America/Noronha" "诺伦哈"
    "America/Belem" "贝伦"
    "America/Fortaleza" "福塔力莎"
    "America/Recife" "雷西非"
    "America/Araguaina" "阿拉圭那"
    "America/Maceio" "马瑟欧"
    "America/Bahia" "巴伊阿"
    "America/Sao_Paulo" "圣保罗"
    "America/Campo_Grande" "格兰场"
    "America/Cuiaba" "古雅巴"
    "America/Santarem" "圣塔伦"
    "America/Porto_Velho" "维留港"
    "America/Boa_Vista" "保维斯塔"
    "America/Manaus" "玛瑙斯"
    "America/Eirunepe" "艾鲁内佩"
    "America/Rio_Branco" "里约布兰"
    "Asia/Thimphu" "廷布"
    "Europe/Minsk" "明斯克"
    "America/Belize" "贝里斯"
    "America/St_Johns" "圣约翰"
    "America/Halifax" "哈里法克斯"
    "America/Glace_Bay" "格雷斯贝"
    "America/Moncton" "蒙克顿"
    "America/Goose_Bay" "鹅湾"
    "America/Toronto" "多伦多"
    "America/Iqaluit" "伊魁特"
    "America/Winnipeg" "温尼伯"
    "America/Resolute" "罗斯鲁特"
    "America/Rankin_Inlet" "兰今湾"
    "America/Regina" "里贾纳"
    "America/Swift_Current" "斯威夫特卡伦特"
    "America/Edmonton" "艾德蒙吞"
    "America/Cambridge_Bay" "剑桥湾"
    "America/Inuvik" "伊奴维克"
    "America/Vancouver" "温哥华"
    "America/Dawson_Creek" "道森克里克"
    "America/Fort_Nelson" "纳尔逊堡"
    "America/Whitehorse" "怀特霍斯"
    "America/Dawson" "道森"
    "Europe/Zurich" "苏黎世"
    "Africa/Abidjan" "阿比让"
    "Pacific/Rarotonga" "拉罗汤加"
    "America/Santiago" "圣地牙哥"
    "America/Coyhaique" "科伊艾克"
    "America/Punta_Arenas" "蓬塔阿雷纳斯"
    "Pacific/Easter" "复活岛"
    "Asia/Shanghai" "上海"
    "Asia/Urumqi" "乌鲁木齐"
    "America/Bogota" "波哥大"
    "America/Costa_Rica" "哥斯大黎加"
    "America/Havana" "哈瓦那"
    "Atlantic/Cape_Verde" "维德角"
    "Asia/Nicosia" "尼古西亚"
    "Asia/Famagusta" "法马古斯塔"
    "Europe/Prague" "布拉格"
    "Europe/Berlin" "柏林"
    "America/Santo_Domingo" "圣多明哥"
    "Africa/Algiers" "阿尔及尔"
    "America/Guayaquil" "瓜亚基尔"
    "Pacific/Galapagos" "加拉巴哥群岛"
    "Europe/Tallinn" "塔林"
    "Africa/Cairo" "开罗"
    "Africa/El_Aaiun" "阿尤恩"
    "Europe/Madrid" "马德里"
    "Africa/Ceuta" "休达"
    "Atlantic/Canary" "加纳利"
    "Europe/Helsinki" "赫尔辛基"
    "Pacific/Fiji" "斐济"
    "Atlantic/Stanley" "史坦利"
    "Pacific/Kosrae" "科斯瑞"
    "Atlantic/Faroe" "法罗群岛"
    "Europe/Paris" "巴黎"
    "Europe/London" "伦敦"
    "Asia/Tbilisi" "第比利斯"
    "America/Cayenne" "开云"
    "Europe/Gibraltar" "直布罗陀"
    "America/Nuuk" "努克"
    "America/Danmarkshavn" "丹马沙文"
    "America/Scoresbysund" "伊托科尔托米特"
    "America/Thule" "杜里"
    "Europe/Athens" "雅典"
    "Atlantic/South_Georgia" "南乔治亚"
    "America/Guatemala" "瓜地马拉"
    "Pacific/Guam" "关岛"
    "Africa/Bissau" "比绍"
    "America/Guyana" "盖亚那"
    "Asia/Hong_Kong" "中华人民共和国香港特别行政区"
    "America/Tegucigalpa" "德古斯加巴"
    "America/Port-au-Prince" "太子港"
    "Europe/Budapest" "布达佩斯"
    "Asia/Jakarta" "雅加达"
    "Asia/Pontianak" "坤甸"
    "Asia/Makassar" "马卡沙尔"
    "Asia/Jayapura" "加亚布拉"
    "Europe/Dublin" "都柏林"
    "Asia/Jerusalem" "耶路撒冷"
    "Asia/Kolkata" "加尔各答"
    "Indian/Chagos" "查戈斯"
    "Asia/Baghdad" "巴格达"
    "Asia/Tehran" "德黑兰"
    "Europe/Rome" "罗马"
    "America/Jamaica" "牙买加"
    "Asia/Amman" "安曼"
    "Asia/Tokyo" "东京"
    "Africa/Nairobi" "奈洛比"
    "Asia/Bishkek" "比什凯克"
    "Pacific/Tarawa" "塔拉瓦"
    "Pacific/Kanton" "坎顿"
    "Pacific/Kiritimati" "基里地马地岛"
    "Asia/Pyongyang" "平壤"
    "Asia/Seoul" "首尔"
    "Asia/Almaty" "阿拉木图"
    "Asia/Qyzylorda" "克孜勒奥尔达"
    "Asia/Qostanay" "科斯塔奈"
    "Asia/Aqtobe" "阿克托比"
    "Asia/Aqtau" "阿克套"
    "Asia/Atyrau" "阿特劳"
    "Asia/Oral" "乌拉尔"
    "Asia/Beirut" "贝鲁特"
    "Asia/Colombo" "可伦坡"
    "Africa/Monrovia" "蒙罗维亚"
    "Europe/Vilnius" "维尔纽斯"
    "Europe/Riga" "里加"
    "Africa/Tripoli" "的黎波里"
    "Africa/Casablanca" "卡萨布兰卡"
    "Europe/Chisinau" "奇西瑙"
    "Pacific/Kwajalein" "瓜加林岛"
    "Asia/Yangon" "仰光"
    "Asia/Ulaanbaatar" "乌兰巴托"
    "Asia/Hovd" "科布多"
    "Asia/Macau" "中华人民共和国澳门特别行政区"
    "America/Martinique" "马丁尼克"
    "Europe/Malta" "马尔他"
    "Indian/Mauritius" "模里西斯"
    "Indian/Maldives" "马尔地夫"
    "America/Mexico_City" "墨西哥市"
    "America/Cancun" "坎昆"
    "America/Merida" "梅里达"
    "America/Monterrey" "蒙特瑞"
    "America/Matamoros" "马塔莫罗斯"
    "America/Chihuahua" "奇华华"
    "America/Ciudad_Juarez" "华雷斯城"
    "America/Ojinaga" "奥希纳加"
    "America/Mazatlan" "马萨特兰"
    "America/Bahia_Banderas" "巴伊亚班德拉斯"
    "America/Hermosillo" "埃莫西约"
    "America/Tijuana" "提华纳"
    "Asia/Kuching" "古晋"
    "Africa/Maputo" "马普托"
    "Africa/Windhoek" "温得和克"
    "Pacific/Noumea" "诺美亚"
    "Pacific/Norfolk" "诺福克"
    "Africa/Lagos" "拉哥斯"
    "America/Managua" "马拿瓜"
    "Asia/Kathmandu" "加德满都"
    "Pacific/Nauru" "诺鲁"
    "Pacific/Niue" "纽埃岛"
    "Pacific/Auckland" "奥克兰"
    "Pacific/Chatham" "查坦"
    "America/Panama" "巴拿马"
    "America/Lima" "利马"
    "Pacific/Tahiti" "大溪地"
    "Pacific/Marquesas" "马可萨斯岛"
    "Pacific/Gambier" "甘比尔群岛"
    "Pacific/Port_Moresby" "莫士比港"
    "Pacific/Bougainville" "布干维尔"
    "Asia/Manila" "马尼拉"
    "Asia/Karachi" "喀拉蚩"
    "Europe/Warsaw" "华沙"
    "America/Miquelon" "密启仑"
    "Pacific/Pitcairn" "皮特肯群岛"
    "America/Puerto_Rico" "波多黎各"
    "Asia/Gaza" "加萨"
    "Asia/Hebron" "赫布隆"
    "Europe/Lisbon" "里斯本"
    "Atlantic/Madeira" "马得拉群岛"
    "Atlantic/Azores" "亚速尔群岛"
    "Pacific/Palau" "帛琉"
    "America/Asuncion" "亚松森"
    "Asia/Qatar" "卡达"
    "Europe/Bucharest" "布加勒斯特"
    "Europe/Belgrade" "贝尔格勒"
    "Europe/Kaliningrad" "加里宁格勒"
    "Europe/Moscow" "莫斯科"
    "Europe/Simferopol" "辛非洛浦"
    "Europe/Kirov" "基洛夫"
    "Europe/Volgograd" "伏尔加格勒"
    "Europe/Astrakhan" "阿斯特拉罕"
    "Europe/Saratov" "萨拉托夫"
    "Europe/Ulyanovsk" "乌里扬诺夫斯克"
    "Europe/Samara" "沙马拉"
    "Asia/Yekaterinburg" "叶卡捷林堡"
    "Asia/Omsk" "鄂木斯克"
    "Asia/Novosibirsk" "新西伯利亚"
    "Asia/Barnaul" "巴尔瑙尔"
    "Asia/Tomsk" "托木斯克"
    "Asia/Novokuznetsk" "新库兹涅茨克"
    "Asia/Krasnoyarsk" "克拉斯诺亚尔斯克"
    "Asia/Irkutsk" "伊尔库次克"
    "Asia/Chita" "赤塔"
    "Asia/Yakutsk" "雅库次克"
    "Asia/Khandyga" "堪地加"
    "Asia/Vladivostok" "海参崴"
    "Asia/Ust-Nera" "乌斯内拉"
    "Asia/Magadan" "马加丹"
    "Asia/Sakhalin" "库页岛"
    "Asia/Srednekolymsk" "中科雷姆斯克"
    "Asia/Kamchatka" "堪察加"
    "Asia/Anadyr" "阿那底"
    "Asia/Riyadh" "利雅德"
    "Pacific/Guadalcanal" "瓜达康纳尔岛"
    "Africa/Khartoum" "喀土穆"
    "Asia/Singapore" "新加坡"
    "America/Paramaribo" "巴拉马利波"
    "Africa/Juba" "朱巴"
    "Africa/Sao_Tome" "圣多美"
    "America/El_Salvador" "萨尔瓦多"
    "Asia/Damascus" "大马士革"
    "America/Grand_Turk" "大特克岛"
    "Africa/Ndjamena" "恩贾梅纳"
    "Asia/Bangkok" "曼谷"
    "Asia/Dushanbe" "杜桑贝"
    "Pacific/Fakaofo" "法考福"
    "Asia/Dili" "帝力"
    "Asia/Ashgabat" "阿什哈巴特"
    "Africa/Tunis" "突尼斯"
    "Pacific/Tongatapu" "东加塔布岛"
    "Europe/Istanbul" "伊斯坦堡"
    "Asia/Taipei" "台北"
    "Europe/Kyiv" "基辅"
    "America/New_York" "纽约"
    "America/Detroit" "底特律"
    "America/Kentucky/Louisville" "路易斯维尔"
    "America/Kentucky/Monticello" "肯塔基州蒙地却罗"
    "America/Indiana/Indianapolis" "印第安那波里斯"
    "America/Indiana/Vincennes" "印第安那州温森斯"
    "America/Indiana/Winamac" "印第安那州威纳马克"
    "America/Indiana/Marengo" "印第安那州马伦哥"
    "America/Indiana/Petersburg" "印第安那州彼得堡"
    "America/Indiana/Vevay" "印第安那州维威"
    "America/Chicago" "芝加哥"
    "America/Indiana/Tell_City" "印第安那州泰尔城"
    "America/Indiana/Knox" "印第安那州诺克斯"
    "America/Menominee" "美诺米尼"
    "America/North_Dakota/Center" "北达科他州中心"
    "America/North_Dakota/New_Salem" "北达科他州纽沙伦"
    "America/North_Dakota/Beulah" "北达科他州布由拉"
    "America/Denver" "丹佛"
    "America/Boise" "波夕"
    "America/Phoenix" "凤凰城"
    "America/Los_Angeles" "洛杉矶"
    "America/Anchorage" "安克拉治"
    "America/Juneau" "朱诺"
    "America/Sitka" "锡特卡"
    "America/Metlakatla" "梅特拉卡特拉"
    "America/Yakutat" "雅库塔"
    "America/Nome" "诺姆"
    "America/Adak" "艾达克"
    "Pacific/Honolulu" "檀香山"
    "America/Montevideo" "蒙特维多"
    "Asia/Samarkand" "撒马尔罕"
    "Asia/Tashkent" "塔什干"
    "America/Caracas" "卡拉卡斯"
    "Asia/Ho_Chi_Minh" "胡志明市"
    "Pacific/Efate" "埃法特"
    "Pacific/Apia" "阿皮亚"
    "Africa/Johannesburg" "约翰尼斯堡"
    "America/Antigua" "安地卡"
    "America/Anguilla" "安吉拉"
    "Africa/Luanda" "罗安达"
    "Antarctica/McMurdo" "麦克默多"
    "Antarctica/DumontDUrville" "杜蒙杜比尔"
    "Antarctica/Syowa" "昭和基地"
    "America/Aruba" "阿路巴"
    "Europe/Mariehamn" "玛丽港"
    "Europe/Sarajevo" "塞拉耶佛"
    "Africa/Ouagadougou" "瓦加杜古"
    "Asia/Bahrain" "巴林"
    "Africa/Bujumbura" "布松布拉"
    "Africa/Porto-Novo" "波多诺佛"
    "America/St_Barthelemy" "圣巴托洛缪岛"
    "Asia/Brunei" "汶莱"
    "America/Kralendijk" "克拉伦代克"
    "America/Nassau" "拿索"
    "Africa/Gaborone" "嘉柏隆里"
    "America/Blanc-Sablon" "白朗萨布隆"
    "America/Atikokan" "阿蒂科肯"
    "America/Creston" "克雷斯顿"
    "Indian/Cocos" "科科斯群岛"
    "Africa/Kinshasa" "金夏沙"
    "Africa/Lubumbashi" "卢本巴希"
    "Africa/Bangui" "班吉"
    "Africa/Brazzaville" "布拉柴维尔"
    "Africa/Douala" "杜阿拉"
    "America/Curacao" "库拉索"
    "Indian/Christmas" "圣诞岛"
    "Europe/Busingen" "布辛根"
    "Africa/Djibouti" "吉布地"
    "Europe/Copenhagen" "哥本哈根"
    "America/Dominica" "多明尼加"
    "Africa/Asmara" "阿斯玛拉"
    "Africa/Addis_Ababa" "阿迪斯阿贝巴"
    "Pacific/Chuuk" "楚克"
    "Pacific/Pohnpei" "波纳佩"
    "Africa/Libreville" "自由市"
    "America/Grenada" "格瑞纳达"
    "Europe/Guernsey" "根息岛"
    "Africa/Accra" "阿克拉"
    "Africa/Banjul" "班竹"
    "Africa/Conakry" "柯那克里"
    "America/Guadeloupe" "瓜地洛普"
    "Africa/Malabo" "马拉博"
    "Europe/Zagreb" "札格瑞布"
    "Europe/Isle_of_Man" "曼岛"
    "Atlantic/Reykjavik" "雷克雅维克"
    "Europe/Jersey" "泽西岛"
    "Asia/Phnom_Penh" "金边"
    "Indian/Comoro" "科摩罗群岛"
    "America/St_Kitts" "圣基茨"
    "Asia/Kuwait" "科威特"
    "America/Cayman" "开曼群岛"
    "Asia/Vientiane" "永珍"
    "America/St_Lucia" "圣露西亚"
    "Europe/Vaduz" "瓦都兹"
    "Africa/Maseru" "马赛鲁"
    "Europe/Luxembourg" "卢森堡"
    "Europe/Monaco" "摩纳哥"
    "Europe/Podgorica" "波多里察"
    "America/Marigot" "马里戈特"
    "Indian/Antananarivo" "安塔那那利佛"
    "Pacific/Majuro" "马朱诺"
    "Europe/Skopje" "史高比耶"
    "Africa/Bamako" "巴马科"
    "Pacific/Saipan" "塞班"
    "Africa/Nouakchott" "诺克少"
    "America/Montserrat" "蒙哲腊"
    "Africa/Blantyre" "布兰太尔"
    "Asia/Kuala_Lumpur" "吉隆坡"
    "Africa/Niamey" "尼亚美"
    "Europe/Amsterdam" "阿姆斯特丹"
    "Europe/Oslo" "奥斯陆"
    "Asia/Muscat" "马斯开特"
    "Indian/Reunion" "留尼旺岛"
    "Africa/Kigali" "基加利"
    "Indian/Mahe" "马埃岛"
    "Europe/Stockholm" "斯德哥尔摩"
    "Atlantic/St_Helena" "圣赫勒拿岛"
    "Europe/Ljubljana" "卢比安纳"
    "Arctic/Longyearbyen" "隆意耳拜恩"
    "Europe/Bratislava" "布拉提斯拉瓦"
    "Africa/Freetown" "自由城"
    "Europe/San_Marino" "圣马利诺"
    "Africa/Dakar" "达喀尔"
    "Africa/Mogadishu" "摩加迪休"
    "America/Lower_Princes" "下太子区"
    "Africa/Mbabane" "墨巴本"
    "Indian/Kerguelen" "凯尔盖朗岛"
    "Africa/Lome" "洛美"
    "America/Port_of_Spain" "西班牙港"
    "Pacific/Funafuti" "富那富提"
    "Africa/Dar_es_Salaam" "沙兰港"
    "Africa/Kampala" "坎帕拉"
    "Pacific/Midway" "中途岛"
    "Pacific/Wake" "威克"
    "Europe/Vatican" "梵蒂冈"
    "America/St_Vincent" "圣文森"
    "America/Tortola" "托尔托拉"
    "America/St_Thomas" "圣托马斯"
    "Pacific/Wallis" "瓦利斯"
    "Asia/Aden" "亚丁"
    "Indian/Mayotte" "马约特岛"
    "Africa/Lusaka" "路沙卡"
    "Africa/Harare" "哈拉雷"
};

// `common/main/yue.xml`: 418 of the 418 zones named.
#[cfg(feature = "localized-exemplar-cities")]
const YUE_HANT: &str = exemplar_cities! {
    "Europe/Andorra" "安道爾"
    "Asia/Dubai" "杜拜"
    "Asia/Kabul" "喀布爾"
    "Europe/Tirane" "地拉那"
    "Asia/Yerevan" "葉里溫"
    "Antarctica/Casey" "凱西"
    "Antarctica/Davis" "戴維斯"
    "Antarctica/Mawson" "莫森"
    "Antarctica/Palmer" "帕麥"
    "Antarctica/Rothera" "羅瑟拉"
    "Antarctica/Troll" "綽爾"
    "Antarctica/Vostok" "沃斯托克"
    "America/Argentina/Buenos_Aires" "布宜諾斯艾利斯"
    "America/Argentina/Cordoba" "哥多華"
    "America/Argentina/Salta" "薩爾塔"
    "America/Argentina/Jujuy" "胡胡伊"
    "America/Argentina/Tucuman" "吐庫曼"
    "America/Argentina/Catamarca" "卡塔馬卡"
    "America/Argentina/La_Rioja" "拉略哈"
    "America/Argentina/San_Juan" "聖胡安"
    "America/Argentina/Mendoza" "門多薩"
    "America/Argentina/San_Luis" "聖路易"
    "America/Argentina/Rio_Gallegos" "里奧加耶戈斯"
    "America/Argentina/Ushuaia" "烏斯懷亞"
    "Pacific/Pago_Pago" "巴哥巴哥"
    "Europe/Vienna" "維也納"
    "Australia/Lord_Howe" "豪勳爵島"
    "Antarctica/Macquarie" "麥覺理"
    "Australia/Hobart" "荷巴特"
    "Australia/Melbourne" "墨爾本"
    "Australia/Sydney" "雪梨"
    "Australia/Broken_Hill" "布羅肯希爾"
    "Australia/Brisbane" "布利斯班"
    "Australia/Lindeman" "林德曼"
    "Australia/Adelaide" "阿得雷德"
    "Australia/Darwin" "達爾文"
    "Australia/Perth" "伯斯"
    "Australia/Eucla" "尤克拉"
    "Asia/Baku" "巴庫"
    "America/Barbados" "巴貝多"
    "Asia/Dhaka" "達卡"
    "Europe/Brussels" "布魯塞爾"
    "Europe/Sofia" "索菲亞"
    "Atlantic/Bermuda" "百慕達"
    "America/La_Paz" "拉巴斯"
    "America/Noronha" "諾倫哈"
    "America/Belem" "貝倫"
    "America/Fortaleza" "福塔力莎"
    "America/Recife" "雷西非"
    "America/Araguaina" "阿拉圭那"
    "America/Maceio" "馬瑟歐"
    "America/Bahia" "巴伊阿"
    "America/Sao_Paulo" "聖保羅"
    "America/Campo_Grande" "格蘭場"
    "America/Cuiaba" "古雅巴"
    "America/Santarem" "聖塔倫"
    "America/Porto_Velho" "維留港"
    "America/Boa_Vista" "保維斯塔"
    "America/Manaus" "瑪瑙斯"
    "America/Eirunepe" "艾魯內佩"
    "America/Rio_Branco" "里約布蘭"
    "Asia/Thimphu" "廷布"
    "Europe/Minsk" "明斯克"
    "America/Belize" "貝里斯"
    "America/St_Johns" "聖約翰"
    "America/Halifax" "哈里法克斯"
    "America/Glace_Bay" "格雷斯貝"
    "America/Moncton" "蒙克頓"
    "America/Goose_Bay" "鵝灣"
    "America/Toronto" "多倫多"
    "America/Iqaluit" "伊魁特"
    "America/Winnipeg" "溫尼伯"
    "America/Resolute" "羅斯魯特"
    "America/Rankin_Inlet" "蘭今灣"
    "America/Regina" "里賈納"
    "America/Swift_Current" "斯威夫特卡倫特"
    "America/Edmonton" "艾德蒙吞"
    "America/Cambridge_Bay" "劍橋灣"
    "America/Inuvik" "伊奴維克"
    "America/Vancouver" "溫哥華"
    "America/Dawson_Creek" "道森克里克"
    "America/Fort_Nelson" "納爾遜堡"
    "America/Whitehorse" "懷特霍斯"
    "America/Dawson" "道森"
    "Europe/Zurich" "蘇黎世"
    "Africa/Abidjan" "阿比讓"
    "Pacific/Rarotonga" "拉羅湯加"
    "America/Santiago" "聖地牙哥"
    "America/Coyhaique" "科伊艾克"
    "America/Punta_Arenas" "蓬塔阿雷納斯"
    "Pacific/Easter" "復活島"
    "Asia/Shanghai" "上海"
    "Asia/Urumqi" "烏魯木齊"
    "America/Bogota" "波哥大"
    "America/Costa_Rica" "哥斯大黎加"
    "America/Havana" "哈瓦那"
    "Atlantic/Cape_Verde" "維德角"
    "Asia/Nicosia" "尼古西亞"
    "Asia/Famagusta" "法馬古斯塔"
    "Europe/Prague" "布拉格"
    "Europe/Berlin" "柏林"
    "America/Santo_Domingo" "聖多明哥"
    "Africa/Algiers" "阿爾及爾"
    "America/Guayaquil" "瓜亞基爾"
    "Pacific/Galapagos" "加拉巴哥群島"
    "Europe/Tallinn" "塔林"
    "Africa/Cairo" "開羅"
    "Africa/El_Aaiun" "阿尤恩"
    "Europe/Madrid" "馬德里"
    "Africa/Ceuta" "休達"
    "Atlantic/Canary" "加納利"
    "Europe/Helsinki" "赫爾辛基"
    "Pacific/Fiji" "斐濟"
    "Atlantic/Stanley" "史坦利"
    "Pacific/Kosrae" "科斯瑞"
    "Atlantic/Faroe" "法羅群島"
    "Europe/Paris" "巴黎"
    "Europe/London" "倫敦"
    "Asia/Tbilisi" "第比利斯"
    "America/Cayenne" "開雲"
    "Europe/Gibraltar" "直布羅陀"
    "America/Nuuk" "努克"
    "America/Danmarkshavn" "丹馬沙文"
    "America/Scoresbysund" "伊托科爾托米特"
    "America/Thule" "杜里"
    "Europe/Athens" "雅典"
    "Atlantic/South_Georgia" "南喬治亞"
    "America/Guatemala" "瓜地馬拉"
    "Pacific/Guam" "關島"
    "Africa/Bissau" "比紹"
    "America/Guyana" "蓋亞那"
    "Asia/Hong_Kong" "中華人民共和國香港特別行政區"
    "America/Tegucigalpa" "德古斯加巴"
    "America/Port-au-Prince" "太子港"
    "Europe/Budapest" "布達佩斯"
    "Asia/Jakarta" "雅加達"
    "Asia/Pontianak" "坤甸"
    "Asia/Makassar" "馬卡沙爾"
    "Asia/Jayapura" "加亞布拉"
    "Europe/Dublin" "都柏林"
    "Asia/Jerusalem" "耶路撒冷"
    "Asia/Kolkata" "加爾各答"
    "Indian/Chagos" "查戈斯"
    "Asia/Baghdad" "巴格達"
    "Asia/Tehran" "德黑蘭"
    "Europe/Rome" "羅馬"
    "America/Jamaica" "牙買加"
    "Asia/Amman" "安曼"
    "Asia/Tokyo" "東京"
    "Africa/Nairobi" "奈洛比"
    "Asia/Bishkek" "比什凱克"
    "Pacific/Tarawa" "塔拉瓦"
    "Pacific/Kanton" "坎頓"
    "Pacific/Kiritimati" "基里地馬地島"
    "Asia/Pyongyang" "平壤"
    "Asia/Seoul" "首爾"
    "Asia/Almaty" "阿拉木圖"
    "Asia/Qyzylorda" "克孜勒奧爾達"
    "Asia/Qostanay" "科斯塔奈"
    "Asia/Aqtobe" "阿克托比"
    "Asia/Aqtau" "阿克套"
    "Asia/Atyrau" "阿特勞"
    "Asia/Oral" "烏拉爾"
    "Asia/Beirut" "貝魯特"
    "Asia/Colombo" "可倫坡"
    "Africa/Monrovia" "蒙羅維亞"
    "Europe/Vilnius" "維爾紐斯"
    "Europe/Riga" "里加"
    "Africa/Tripoli" "的黎波里"
    "Africa/Casablanca" "卡薩布蘭卡"
    "Europe/Chisinau" "奇西瑙"
    "Pacific/Kwajalein" "瓜加林島"
    "Asia/Yangon" "仰光"
    "Asia/Ulaanbaatar" "烏蘭巴托"
    "Asia/Hovd" "科布多"
    "Asia/Macau" "中華人民共和國澳門特別行政區"
    "America/Martinique" "馬丁尼克"
    "Europe/Malta" "馬爾他"
    "Indian/Mauritius" "模里西斯"
    "Indian/Maldives" "馬爾地夫"
    "America/Mexico_City" "墨西哥市"
    "America/Cancun" "坎昆"
    "America/Merida" "梅里達"
    "America/Monterrey" "蒙特瑞"
    "America/Matamoros" "馬塔莫羅斯"
    "America/Chihuahua" "奇華華"
    "America/Ciudad_Juarez" "華雷斯城"
    "America/Ojinaga" "奧希納加"
    "America/Mazatlan" "馬薩特蘭"
    "America/Bahia_Banderas" "巴伊亞班德拉斯"
    "America/Hermosillo" "埃莫西約"
    "America/Tijuana" "提華納"
    "Asia/Kuching" "古晉"
    "Africa/Maputo" "馬普托"
    "Africa/Windhoek" "溫得和克"
    "Pacific/Noumea" "諾美亞"
    "Pacific/Norfolk" "諾福克"
    "Africa/Lagos" "拉哥斯"
    "America/Managua" "馬拿瓜"
    "Asia/Kathmandu" "加德滿都"
    "Pacific/Nauru" "諾魯"
    "Pacific/Niue" "紐埃島"
    "Pacific/Auckland" "奧克蘭"
    "Pacific/Chatham" "查坦"
    "America/Panama" "巴拿馬"
    "America/Lima" "利馬"
    "Pacific/Tahiti" "大溪地"
    "Pacific/Marquesas" "馬可薩斯島"
    "Pacific/Gambier" "甘比爾群島"
    "Pacific/Port_Moresby" "莫士比港"
    "Pacific/Bougainville" "布干維爾"
    "Asia/Manila" "馬尼拉"
    "Asia/Karachi" "喀拉蚩"
    "Europe/Warsaw" "華沙"
    "America/Miquelon" "密啟崙"
    "Pacific/Pitcairn" "皮特肯群島"
    "America/Puerto_Rico" "波多黎各"
    "Asia/Gaza" "加薩"
    "Asia/Hebron" "赫布隆"
    "Europe/Lisbon" "里斯本"
    "Atlantic/Madeira" "馬得拉群島"
    "Atlantic/Azores" "亞速爾群島"
    "Pacific/Palau" "帛琉"
    "America/Asuncion" "亞松森"
    "Asia/Qatar" "卡達"
    "Europe/Bucharest" "布加勒斯特"
    "Europe/Belgrade" "貝爾格勒"
    "Europe/Kaliningrad" "加里寧格勒"
    "Europe/Moscow" "莫斯科"
    "Europe/Simferopol" "辛非洛浦"
    "Europe/Kirov" "基洛夫"
    "Europe/Volgograd" "伏爾加格勒"
    "Europe/Astrakhan" "阿斯特拉罕"
    "Europe/Saratov" "薩拉托夫"
    "Europe/Ulyanovsk" "烏里揚諾夫斯克"
    "Europe/Samara" "沙馬拉"
    "Asia/Yekaterinburg" "葉卡捷林堡"
    "Asia/Omsk" "鄂木斯克"
    "Asia/Novosibirsk" "新西伯利亞"
    "Asia/Barnaul" "巴爾瑙爾"
    "Asia/Tomsk" "托木斯克"
    "Asia/Novokuznetsk" "新庫茲涅茨克"
    "Asia/Krasnoyarsk" "克拉斯諾亞爾斯克"
    "Asia/Irkutsk" "伊爾庫次克"
    "Asia/Chita" "赤塔"
    "Asia/Yakutsk" "雅庫次克"
    "Asia/Khandyga" "堪地加"
    "Asia/Vladivostok" "海參崴"
    "Asia/Ust-Nera" "烏斯內拉"
    "Asia/Magadan" "馬加丹"
    "Asia/Sakhalin" "庫頁島"
    "Asia/Srednekolymsk" "中科雷姆斯克"
    "Asia/Kamchatka" "堪察加"
    "Asia/Anadyr" "阿那底"
    "Asia/Riyadh" "利雅德"
    "Pacific/Guadalcanal" "瓜達康納爾島"
    "Africa/Khartoum" "喀土穆"
    "Asia/Singapore" "新加坡"
    "America/Paramaribo" "巴拉馬利波"
    "Africa/Juba" "朱巴"
    "Africa/Sao_Tome" "聖多美"
    "America/El_Salvador" "薩爾瓦多"
    "Asia/Damascus" "大馬士革"
    "America/Grand_Turk" "大特克島"
    "Africa/Ndjamena" "恩賈梅納"
    "Asia/Bangkok" "曼谷"
    "Asia/Dushanbe" "杜桑貝"
    "Pacific/Fakaofo" "法考福"
    "Asia/Dili" "帝力"
    "Asia/Ashgabat" "阿什哈巴特"
    "Africa/Tunis" "突尼斯"
    "Pacific/Tongatapu" "東加塔布島"
    "Europe/Istanbul" "伊斯坦堡"
    "Asia/Taipei" "台北"
    "Europe/Kyiv" "基輔"
    "America/New_York" "紐約"
    "America/Detroit" "底特律"
    "America/Kentucky/Louisville" "路易斯維爾"
    "America/Kentucky/Monticello" "肯塔基州蒙地卻羅"
    "America/Indiana/Indianapolis" "印第安那波里斯"
    "America/Indiana/Vincennes" "印第安那州溫森斯"
    "America/Indiana/Winamac" "印第安那州威納馬克"
    "America/Indiana/Marengo" "印第安那州馬倫哥"
    "America/Indiana/Petersburg" "印第安那州彼得堡"
    "America/Indiana/Vevay" "印第安那州維威"
    "America/Chicago" "芝加哥"
    "America/Indiana/Tell_City" "印第安那州泰爾城"
    "America/Indiana/Knox" "印第安那州諾克斯"
    "America/Menominee" "美諾米尼"
    "America/North_Dakota/Center" "北達科他州中心"
    "America/North_Dakota/New_Salem" "北達科他州紐沙倫"
    "America/North_Dakota/Beulah" "北達科他州布由拉"
    "America/Denver" "丹佛"
    "America/Boise" "波夕"
    "America/Phoenix" "鳳凰城"
    "America/Los_Angeles" "洛杉磯"
    "America/Anchorage" "安克拉治"
    "America/Juneau" "朱諾"
    "America/Sitka" "錫特卡"
    "America/Metlakatla" "梅特拉卡特拉"
    "America/Yakutat" "雅庫塔"
    "America/Nome" "諾姆"
    "America/Adak" "艾達克"
    "Pacific/Honolulu" "檀香山"
    "America/Montevideo" "蒙特維多"
    "Asia/Samarkand" "撒馬爾罕"
    "Asia/Tashkent" "塔什干"
    "America/Caracas" "卡拉卡斯"
    "Asia/Ho_Chi_Minh" "胡志明市"
    "Pacific/Efate" "埃法特"
    "Pacific/Apia" "阿皮亞"
    "Africa/Johannesburg" "約翰尼斯堡"
    "America/Antigua" "安地卡"
    "America/Anguilla" "安吉拉"
    "Africa/Luanda" "羅安達"
    "Antarctica/McMurdo" "麥克默多"
    "Antarctica/DumontDUrville" "杜蒙杜比爾"
    "Antarctica/Syowa" "昭和基地"
    "America/Aruba" "阿路巴"
    "Europe/Mariehamn" "瑪麗港"
    "Europe/Sarajevo" "塞拉耶佛"
    "Africa/Ouagadougou" "瓦加杜古"
    "Asia/Bahrain" "巴林"
    "Africa/Bujumbura" "布松布拉"
    "Africa/Porto-Novo" "波多諾佛"
    "America/St_Barthelemy" "聖巴托洛繆島"
    "Asia/Brunei" "汶萊"
    "America/Kralendijk" "克拉倫代克"
    "America/Nassau" "拿索"
    "Africa/Gaborone" "嘉柏隆里"
    "America/Blanc-Sablon" "白朗薩布隆"
    "America/Atikokan" "阿蒂科肯"
    "America/Creston" "克雷斯頓"
    "Indian/Cocos" "科科斯群島"
    "Africa/Kinshasa" "金夏沙"
    "Africa/Lubumbashi" "盧本巴希"
    "Africa/Bangui" "班吉"
    "Africa/Brazzaville" "布拉柴維爾"
    "Africa/Douala" "杜阿拉"
    "America/Curacao" "庫拉索"
    "Indian/Christmas" "聖誕島"
    "Europe/Busingen" "布辛根"
    "Africa/Djibouti" "吉布地"
    "Europe/Copenhagen" "哥本哈根"
    "America/Dominica" "多明尼加"
    "Africa/Asmara" "阿斯瑪拉"
    "Africa/Addis_Ababa" "阿迪斯阿貝巴"
    "Pacific/Chuuk" "楚克"
    "Pacific/Pohnpei" "波納佩"
    "Africa/Libreville" "自由市"
    "America/Grenada" "格瑞納達"
    "Europe/Guernsey" "根息島"
    "Africa/Accra" "阿克拉"
    "Africa/Banjul" "班竹"
    "Africa/Conakry" "柯那克里"
    "America/Guadeloupe" "瓜地洛普"
    "Africa/Malabo" "馬拉博"
    "Europe/Zagreb" "札格瑞布"
    "Europe/Isle_of_Man" "曼島"
    "Atlantic/Reykjavik" "雷克雅維克"
    "Europe/Jersey" "澤西島"
    "Asia/Phnom_Penh" "金邊"
    "Indian/Comoro" "科摩羅群島"
    "America/St_Kitts" "聖基茨"
    "Asia/Kuwait" "科威特"
    "America/Cayman" "開曼群島"
    "Asia/Vientiane" "永珍"
    "America/St_Lucia" "聖露西亞"
    "Europe/Vaduz" "瓦都茲"
    "Africa/Maseru" "馬賽魯"
    "Europe/Luxembourg" "盧森堡"
    "Europe/Monaco" "摩納哥"
    "Europe/Podgorica" "波多里察"
    "America/Marigot" "馬里戈特"
    "Indian/Antananarivo" "安塔那那利佛"
    "Pacific/Majuro" "馬朱諾"
    "Europe/Skopje" "史高比耶"
    "Africa/Bamako" "巴馬科"
    "Pacific/Saipan" "塞班"
    "Africa/Nouakchott" "諾克少"
    "America/Montserrat" "蒙哲臘"
    "Africa/Blantyre" "布蘭太爾"
    "Asia/Kuala_Lumpur" "吉隆坡"
    "Africa/Niamey" "尼亞美"
    "Europe/Amsterdam" "阿姆斯特丹"
    "Europe/Oslo" "奧斯陸"
    "Asia/Muscat" "馬斯開特"
    "Indian/Reunion" "留尼旺島"
    "Africa/Kigali" "基加利"
    "Indian/Mahe" "馬埃島"
    "Europe/Stockholm" "斯德哥爾摩"
    "Atlantic/St_Helena" "聖赫勒拿島"
    "Europe/Ljubljana" "盧比安納"
    "Arctic/Longyearbyen" "隆意耳拜恩"
    "Europe/Bratislava" "布拉提斯拉瓦"
    "Africa/Freetown" "自由城"
    "Europe/San_Marino" "聖馬利諾"
    "Africa/Dakar" "達喀爾"
    "Africa/Mogadishu" "摩加迪休"
    "America/Lower_Princes" "下太子區"
    "Africa/Mbabane" "墨巴本"
    "Indian/Kerguelen" "凱爾蓋朗島"
    "Africa/Lome" "洛美"
    "America/Port_of_Spain" "西班牙港"
    "Pacific/Funafuti" "富那富提"
    "Africa/Dar_es_Salaam" "沙蘭港"
    "Africa/Kampala" "坎帕拉"
    "Pacific/Midway" "中途島"
    "Pacific/Wake" "威克"
    "Europe/Vatican" "梵蒂岡"
    "America/St_Vincent" "聖文森"
    "America/Tortola" "托爾托拉"
    "America/St_Thomas" "聖托馬斯"
    "Pacific/Wallis" "瓦利斯"
    "Asia/Aden" "亞丁"
    "Indian/Mayotte" "馬約特島"
    "Africa/Lusaka" "路沙卡"
    "Africa/Harare" "哈拉雷"
};

#[cfg(test)]
mod tests {
    use super::*;

    fn every_table() -> impl Iterator<Item = &'static ExemplarCities> {
        let english = [&ROOT_TABLE, &ENGLISH_TABLE].into_iter();
        #[cfg(feature = "localized-exemplar-cities")]
        let english = english.chain(TABLES.iter());
        english
    }

    fn render(city: ExemplarCity<'_>) -> (String, &'static str) {
        (city.name.to_string(), city.tag)
    }

    #[test]
    fn the_zones_are_distinct_and_every_table_has_a_line_for_each() {
        assert_eq!(ZONES.len(), 418);
        let mut sorted: Vec<String> = ZONES.iter().map(|zone| zone.to_ascii_lowercase()).collect();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), ZONES.len());
        for table in every_table() {
            assert_eq!(
                table.names.split('\n').count(),
                ZONES.len() + 1,
                "{}",
                table.tag
            );
            assert!(table.names.ends_with('\n'), "{}", table.tag);
        }
        assert_eq!(zone_index("asia/tokyo"), zone_index("Asia/Tokyo"));
        assert_eq!(zone_index("Etc/UTC"), None);
    }

    #[test]
    fn no_name_is_padded_or_holds_a_marker_or_a_tab() {
        for table in every_table() {
            for (index, zone) in ZONES.iter().enumerate() {
                let Some(name) = table.name_at(index) else {
                    continue;
                };
                assert_eq!(name, name.trim(), "{} {zone}", table.tag);
                assert!(!name.contains(['\t', '\r', '↑']), "{} {zone}", table.tag);
            }
        }
        for index in 0..ZONES.len() {
            assert_ne!(ROOT_TABLE.line_at(index), Some(INHERITED));
            assert_ne!(ENGLISH_TABLE.line_at(index), Some(INHERITED));
        }
    }

    /// CLDR 48 `en.xml`: `Asia/Saigon` is `Ho Chi Minh City` and
    /// `Antarctica/Rothera` `Rothera Station`; `root.xml`: `Asia/Saigon` is
    /// `Ho Chi Minh` and `Asia/Calcutta` `Kolkata`; neither names
    /// `America/Los_Angeles`, which is derived.
    #[test]
    fn english_is_en_xml_then_root_xml_then_the_derived_name() {
        let english = |zone: &str| render(english_city(zone_index(zone).expect(zone), zone));
        assert_eq!(
            english("Asia/Ho_Chi_Minh"),
            ("Ho Chi Minh City".into(), "en")
        );
        assert_eq!(
            english("Antarctica/Rothera"),
            ("Rothera Station".into(), "en")
        );
        assert_eq!(english("Asia/Kolkata"), ("Kolkata".into(), "en"));
        assert_eq!(english("Europe/Tirane"), ("Tirana".into(), "en"));
        assert_eq!(english("America/Los_Angeles"), ("Los Angeles".into(), "en"));
        assert_eq!(
            english("America/Argentina/Buenos_Aires"),
            ("Buenos Aires".into(), "en")
        );
        assert_eq!(
            english("America/Port-au-Prince"),
            ("Port-au-Prince".into(), "en")
        );
        let index = zone_index("Asia/Ho_Chi_Minh").expect("Saigon");
        assert_eq!(
            root_city(index, "Asia/Ho_Chi_Minh"),
            CityName::Cldr("Ho Chi Minh")
        );
        assert_eq!(
            CityName::Derived("America/North_Dakota/New_Salem").to_string(),
            "New Salem"
        );
    }

    /// CLDR 48 `ja.xml`: `<zone type="Asia/Tokyo"><exemplarCity>東京`;
    /// `zh_Hant.xml` 東京, `zh.xml` 东京; `de.xml`: `Europe/Vienna` Wien,
    /// and `Europe/Berlin` and `Antarctica/Rothera` the marker `↑↑↑`, which
    /// is root's value: the derived `Berlin` and `Rothera`, not English's
    /// `Rothera Station`.
    #[cfg(feature = "localized-exemplar-cities")]
    #[test]
    fn a_locale_names_the_city_and_its_marker_inherits_from_root() {
        let city = |tag: &str, zone: &str| {
            let locale = Locale::parse(tag).expect("a tag");
            render(exemplar_city(&locale, zone_index(zone).expect(zone), zone))
        };
        assert_eq!(city("ja", "Asia/Tokyo"), ("東京".into(), "ja"));
        assert_eq!(city("ja-JP", "Europe/Oslo"), ("オスロ".into(), "ja"));
        assert_eq!(city("zh-TW", "Asia/Tokyo"), ("東京".into(), "zh-Hant"));
        assert_eq!(city("zh", "Asia/Tokyo"), ("东京".into(), "zh-Hans"));
        assert_eq!(city("de-AT", "Europe/Vienna"), ("Wien".into(), "de"));
        assert_eq!(city("de", "Europe/Berlin"), ("Berlin".into(), "de"));
        assert_eq!(city("de", "Antarctica/Rothera"), ("Rothera".into(), "de"));
        assert_eq!(
            city("en-GB", "Antarctica/Rothera"),
            ("Rothera Station".into(), "en")
        );
        assert_eq!(city("en", "Asia/Tokyo"), ("Tokyo".into(), "en"));
    }

    /// CLDR 48 `ur.xml` `Asia/Karachi` کراچی; `pt_PT.xml` Carachi, and the
    /// marker for `Europe/Lisbon`, which is `pt.xml`'s Lisboa; `sw.xml` the
    /// marker for `Asia/Karachi`, root's derived Karachi.
    #[cfg(feature = "localized-exemplar-cities")]
    #[test]
    fn the_most_spoken_languages_name_their_cities() {
        let city = |tag: &str, zone: &str| {
            let locale = Locale::parse(tag).expect("a tag");
            render(exemplar_city(&locale, zone_index(zone).expect(zone), zone))
        };
        assert_eq!(city("ur", "Asia/Karachi"), ("کراچی".into(), "ur"));
        assert_eq!(city("pt-PT", "Asia/Karachi"), ("Carachi".into(), "pt-PT"));
        assert_eq!(city("pt-PT", "Europe/Lisbon"), ("Lisboa".into(), "pt"));
        assert_eq!(city("sw", "Asia/Karachi"), ("Karachi".into(), "sw"));
        assert_eq!(city("yue", "Europe/Lisbon"), ("里斯本".into(), "yue-Hant"));
        assert_eq!(city("pa-PK", "Asia/Karachi"), ("Karachi".into(), "en"));
    }

    /// Kabyle's values are all unconfirmed in CLDR 48 `kab.xml`, Tibetan
    /// names no zone, and German has no value for `America/Coyhaique`,
    /// which tzdata added in 2025: each falls back to English.
    #[cfg(feature = "localized-exemplar-cities")]
    #[test]
    fn a_zone_a_locale_does_not_name_falls_back_to_english() {
        let city = |tag: &str, zone: &str| {
            let locale = Locale::parse(tag).expect("a tag");
            render(exemplar_city(&locale, zone_index(zone).expect(zone), zone))
        };
        assert!(table("kab").is_none());
        assert!(table("bo").is_none());
        assert_eq!(city("kab", "Africa/Algiers"), ("Algiers".into(), "en"));
        assert_eq!(city("bo", "Asia/Shanghai"), ("Shanghai".into(), "en"));
        assert_eq!(city("de", "America/Coyhaique"), ("Coyhaique".into(), "en"));
        assert_eq!(city("und", "Asia/Tokyo"), ("Tokyo".into(), "en"));
        let root = Locale::ROOT;
        assert_eq!(localized_city(&root, 0, ZONES[0]), None);
    }

    #[cfg(feature = "localized-exemplar-cities")]
    #[test]
    fn every_table_is_a_carried_locale_in_tag_order() {
        use crate::data::LOCALES;
        assert_eq!(TABLES.len(), 42);
        for pair in TABLES.windows(2) {
            assert!(
                pair[0].tag < pair[1].tag,
                "{} !< {}",
                pair[0].tag,
                pair[1].tag
            );
        }
        for table in TABLES {
            assert!(
                LOCALES.iter().any(|data| data.tag == table.tag),
                "{} is not a locale hc-i18n carries",
                table.tag
            );
            assert!(
                (0..ZONES.len()).any(|index| table.name_at(index).is_some()),
                "{} names nothing",
                table.tag
            );
        }
        assert_eq!(
            table(ENGLISH).map(|table| table.names),
            Some(ENGLISH_TABLE.names)
        );
    }
}
