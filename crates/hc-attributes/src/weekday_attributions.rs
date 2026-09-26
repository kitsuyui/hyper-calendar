//! What the seven days are associated with: planets, deities, colours,
//! stones.
//!
//! The seven-day week is one structure that five traditions have decorated
//! differently, and the decorations are *translations of each other*. The
//! Roman week named the days for the seven moving bodies; the Germanic
//! peoples substituted their own gods for the Roman ones in the process
//! called *interpretatio germanica*; East Asia took the planetary week with
//! the 五行 names attached; India had the same seven under the *navagraha*;
//! and Thailand, taking the Indian scheme, gave each day the colour of the
//! god who guards it.
//!
//! So the tables here line up. Tuesday is Mars in Latin, Týr in Norse, 火
//! (fire, Mars) in Japanese, Maṅgala in Sanskrit and pink in Thailand,
//! because Maṅgala is Mars and 火星 is Mars and Týr was read as Mars. A test
//! asserts the alignment rather than this comment claiming it.
//!
//! ```
//! use hc_attributes::weekday_attributions::{WEEKDAY_COLOURS_THAI, attribution};
//! use hc_calendar::Weekday;
//!
//! // Thailand's day colours are in everyday use: King Bhumibol was born on
//! // a Monday, so the country goes yellow on his birthday. Monday carries
//! // two readings and both are shipped.
//! assert_eq!(
//!     attribution(&WEEKDAY_COLOURS_THAI, Weekday::Monday),
//!     &["yellow", "cream"]
//! );
//! ```
//!
//! # Saturday is where the pattern breaks, and that is the interesting part
//!
//! *Interpretatio germanica* substituted a Germanic god for every Roman one
//! except Saturn, which was simply kept: Old English *sæternesdæġ* is the
//! Roman god's name with an English ending. The North Germanic languages did
//! not even do that — Old Norse *laugardagr* is "washing day" and refers to
//! no deity at all. [`WEEKDAY_DEITIES_NORSE`] says so in its Saturday entry
//! rather than inventing a Norse Saturn, and a test holds it to that.
//!
//! # 七曜 is `hc-almanac`'s, and this is not a copy of it
//!
//! `hc_almanac::seven_luminaries` implements 七曜 as an almanac annotation
//! over [`hc_calendar::Weekday`]. This module carries the *names* and
//! *associations* across traditions; it does not compute anything about a
//! day and does not duplicate that module's work. A caller wanting the
//! 七曜 of a date should ask `hc-almanac`.

use hc_calendar::Weekday;

use crate::authority::{AttributionDate, Authority, Provenance, Region, Validity, WeekdayTable};

/// The Roman planetary week: *dies Solis* through *dies Saturni*.
///
/// The seven bodies are the seven that move against the fixed stars to the
/// naked eye, which is why the Sun and Moon are in a list of "planets": the
/// scheme is pre-Copernican and *planeta* meant "wanderer". The same seven,
/// in the same order, are [`hc_seasons::RulingPlanet::CHALDEAN_ORDER`]
/// rearranged by the day-hour rule that produces the weekday sequence.
pub static WEEKDAY_PLANETS_GRECO_ROMAN: WeekdayTable = WeekdayTable::new(
    Authority {
        id: "weekday-planets-greco-roman",
        english_name: "Roman planetary names of the days",
        body: None,
        region: Region::ROMAN_WORLD,
        established: None,
        revised: None,
        validity: Validity::UNKNOWN,
        provenance: Provenance::Recorded,
        source: "Day names as Unicode CLDR 48 common/main/la.xml prints them (compared \
                 2026-09-26); Constantine made dies Solis a legal holiday, per Philip Schaff, \
                 History of the Christian Church, vol. III (T&T Clark, 1884), p. 380",
        caveat: None,
    },
    [
        &["dies Solis", "Sun"],
        &["dies Lunae", "Moon"],
        &["dies Martis", "Mars"],
        &["dies Mercurii", "Mercury"],
        &["dies Iovis", "Jupiter"],
        &["dies Veneris", "Venus"],
        &["dies Saturni", "Saturn"],
    ],
);

/// The Germanic deities substituted for the Roman ones.
///
/// The substitution happened after about 100 CE and before the conversions
/// of the sixth and seventh centuries, during the undifferentiated West
/// Germanic phase; the North Germanic names were taken from the West
/// Germanic ones rather than calqued from Latin afresh.
///
/// Saturday has no substitution, and its entry says so. This is not a hole
/// in the data: it is the datum.
pub static WEEKDAY_DEITIES_NORSE: WeekdayTable = WeekdayTable::new(
    Authority {
        id: "weekday-deities-germanic",
        english_name: "Germanic deities of the days",
        body: None,
        region: Region::GERMANIC,
        established: None,
        revised: None,
        validity: Validity::between(100, 1100),
        provenance: Provenance::Recorded,
        source: "Jacob Grimm, Teutonic Mythology (repr. Courier, 2004), pp. 122–123; \
                 Friggjarstjarna per the Dictionary of Old Norse Prose, University of \
                 Copenhagen",
        caveat: Some(
            "Saturday was never substituted. Old English sæternesdæġ keeps the Roman god; Old \
             Norse laugardagr means \"washing day\" and names no deity.",
        ),
    },
    [
        &["Sunna", "Sól"],
        &["Máni"],
        &["Týr", "Tiw"],
        &["Óðinn", "Wōden"],
        &["Þórr", "Þunor"],
        &["Frigg", "Frīg"],
        &["no Germanic substitution; Saturn retained"],
    ],
);

/// The Old English day names, which are what Modern English still uses.
///
/// Wednesday is *wōdnesdæġ*, Woden's day; German went the other way and
/// calls it *Mittwoch*, mid-week, as do Icelandic *miðvikudagur* and Finnish
/// *keskiviikko*.
pub static WEEKDAY_NAMES_OLD_ENGLISH: WeekdayTable = WeekdayTable::new(
    Authority {
        id: "weekday-names-old-english",
        english_name: "Old English names of the days",
        body: None,
        region: Region::EARLY_ENGLAND,
        established: None,
        revised: None,
        validity: Validity::between(500, 1100),
        provenance: Provenance::Recorded,
        source: "Old English forms as Grimm, Teutonic Mythology, and the Oxford English \
                 Dictionary's entries for the day names give them",
        caveat: Some(
            "Old English forms, superseded by their own descendants: Modern English Sunday \
             through Saturday come directly from these and are hc-i18n's business, not this \
             crate's.",
        ),
    },
    [
        &["sunnandæġ"],
        &["mōnandæġ"],
        &["tīwesdæġ"],
        &["wōdnesdæg"],
        &["þunresdæġ"],
        &["frīġedæġ"],
        &["sæternesdæġ"],
    ],
);

/// The Japanese 七曜 day names.
///
/// 日月火水木金土 — Sun, Moon, and the five phases that name the five
/// planets: 火星 Mars, 水星 Mercury, 木星 Jupiter, 金星 Venus, 土星 Saturn.
/// The sequence is the Roman one, which is how the planetary week reached
/// East Asia; what changed is that the planets carry 五行 names rather than
/// deity names.
///
/// 五行 is not the Greek four elements. See
/// [`hc_calendar::cycle::FIVE_PHASES`] for the cycle itself and
/// [`hc_seasons::Element`] for the Greek four, which are a different system.
pub static WEEKDAY_LUMINARIES_JAPANESE: WeekdayTable = WeekdayTable::new(
    Authority {
        id: "weekday-luminaries-japanese",
        english_name: "Japanese 七曜 day names",
        body: None,
        region: Region::JAPAN,
        established: None,
        revised: None,
        validity: Validity::UNKNOWN,
        provenance: Provenance::Vernacular,
        source: "Day names as Unicode CLDR 48 common/main/ja.xml prints them (compared \
                 2026-09-26); the 七曜 as an almanac annotation are implemented in hc-almanac's \
                 seven_luminaries module",
        caveat: None,
    },
    [
        &["日曜日", "nichiyōbi", "Sun"],
        &["月曜日", "getsuyōbi", "Moon"],
        &["火曜日", "kayōbi", "Mars"],
        &["水曜日", "suiyōbi", "Mercury"],
        &["木曜日", "mokuyōbi", "Jupiter"],
        &["金曜日", "kin'yōbi", "Venus"],
        &["土曜日", "doyōbi", "Saturn"],
    ],
);

/// The Sanskrit *vāsara* day names and their *navagraha*.
///
/// The earliest dated weekday in an Indian inscription is in the Eran
/// pillar inscription of Budhagupta, Gupta year 165 (484 CE), which dates
/// itself "on the day of Suraguru", a Thursday (J. F. Fleet, *Corpus
/// Inscriptionum Indicarum* III, *Inscriptions of the Early Gupta Kings*,
/// 1888). That is four centuries after the planetary week was in general
/// use in the Roman empire, where a Pompeian graffito dates 6 February 60 CE
/// by its *dies solis*.
///
/// *Śukra* is Venus, a son of Bhṛgu; *guru* here is a title of Bṛhaspati and
/// so of Jupiter; *budha*, Mercury, is a son of Soma, the Moon.
pub static WEEKDAY_VASARA_SANSKRIT: WeekdayTable = WeekdayTable::new(
    Authority {
        id: "weekday-vasara-sanskrit",
        english_name: "Sanskrit vāsara day names",
        body: None,
        region: Region::INDIA,
        established: Some(AttributionDate::year(484)),
        revised: None,
        validity: Validity::since(484),
        provenance: Provenance::Recorded,
        source: "Monier-Williams, Sanskrit-English Dictionary (1899), s.v. vāsara and vāra; \
                 the earliest dated use, the Eran pillar inscription of Budhagupta, 484 CE, \
                 in Fleet, Corpus Inscriptionum Indicarum III (1888)",
        caveat: None,
    },
    [
        &["ravivāra", "Sūrya", "Sun"],
        &["somavāra", "Chandra", "Moon"],
        &["maṅgalavāra", "Maṅgala", "Mars"],
        &["budhavāra", "Budha", "Mercury"],
        &["guruvāra", "Bṛhaspati", "Jupiter"],
        &["śukravāra", "Śukra", "Venus"],
        &["śanivāra", "Śani", "Saturn"],
    ],
);

/// Thailand's day colours, which are in everyday use.
///
/// Each day takes the colour of the god who guards it, the gods being the
/// *navagraha* of the Indian scheme. The custom is not antiquarian: King
/// Bhumibol and King Vajiralongkorn were both born on Mondays, so Thailand
/// is decorated in yellow on their birthdays, and people commonly wear the
/// colour of the day.
///
/// Monday's colour is given as yellow or cream and both are carried.
pub static WEEKDAY_COLOURS_THAI: WeekdayTable = WeekdayTable::new(
    Authority {
        id: "weekday-colours-thai",
        english_name: "Thai colours of the day",
        body: None,
        region: Region::THAILAND,
        established: None,
        revised: None,
        validity: Validity::UNKNOWN,
        provenance: Provenance::Vernacular,
        source: "Denis Segaller, Thai Ways (Silkworm Books, 2005); Tien-Rein Lee, \"The colour \
                 we use in our daily life\", ประชุมวิชาการ (2013), p. 22",
        caveat: None,
    },
    [
        &["red"],
        &["yellow", "cream"],
        &["pink"],
        &["green"],
        &["orange"],
        &["light blue"],
        &["purple"],
    ],
);

/// The Thai day gods, the *navagraha* under their Thai usage.
///
/// Shipped separately from the colours rather than folded into them,
/// because a colour and a deity are different attributions and a caller may
/// want one without the other.
pub static WEEKDAY_DEITIES_THAI: WeekdayTable = WeekdayTable::new(
    Authority {
        id: "weekday-deities-thai",
        english_name: "Thai guardian deities of the days",
        body: None,
        region: Region::THAILAND,
        established: None,
        revised: None,
        validity: Validity::UNKNOWN,
        provenance: Provenance::Vernacular,
        source: "Denis Segaller, Thai Ways (Silkworm Books, 2005); day names as Unicode CLDR 48 \
                 common/main/th.xml prints them (compared 2026-09-26)",
        caveat: None,
    },
    [
        &["Surya", "วันอาทิตย์", "wan athit"],
        &["Chandra", "วันจันทร์", "wan chan"],
        &["Mangala", "วันอังคาร", "wan angkhan"],
        &["Budha", "วันพุธ", "wan phut"],
        &["Brihaspati", "วันพฤหัสบดี", "wan phruehatsabodi"],
        &["Shukra", "วันศุกร์", "wan suk"],
        &["Shani", "วันเสาร์", "wan sao"],
    ],
);

/// The stones assigned to the days of the week, which are not the month
/// stones and not the sign stones either.
///
/// Kunz records a third, quite separate system: a stone per weekday. It is
/// the reason "birthday stone" and "birthstone" are not synonyms, though
/// they are used as if they were.
pub static WEEKDAY_STONES_KUNZ: WeekdayTable = WeekdayTable::new(
    Authority {
        id: "weekday-stones-kunz-1913",
        english_name: "stones of the days of the week",
        body: None,
        region: Region::UNSPECIFIED,
        established: None,
        revised: None,
        validity: Validity::UNKNOWN,
        provenance: Provenance::Recorded,
        source: "George F. Kunz, The Curious Lore of Precious Stones (Lippincott, 1913; kunz1913)",
        caveat: Some(
            "A third system, distinct from both the month birthstones and the zodiacal stones. \
             \"Birthday stone\" is sometimes used for all three.",
        ),
    },
    [
        &["topaz", "diamond"],
        &["pearl", "crystal"],
        &["ruby", "emerald"],
        &["amethyst", "lodestone"],
        &["sapphire", "carnelian"],
        &["emerald", "cat's-eye"],
        &["turquoise", "diamond"],
    ],
);

/// Every weekday table this crate ships.
pub static ALL: [&WeekdayTable; 8] = [
    &WEEKDAY_PLANETS_GRECO_ROMAN,
    &WEEKDAY_DEITIES_NORSE,
    &WEEKDAY_NAMES_OLD_ENGLISH,
    &WEEKDAY_LUMINARIES_JAPANESE,
    &WEEKDAY_VASARA_SANSKRIT,
    &WEEKDAY_COLOURS_THAI,
    &WEEKDAY_DEITIES_THAI,
    &WEEKDAY_STONES_KUNZ,
];

/// What one tradition attributes to a weekday.
///
/// Infallible: [`crate::authority::weekday_index`] is always 0..=6 and every
/// weekday table has seven entries, so the empty fallback is unreachable.
#[must_use]
pub fn attribution(table: &WeekdayTable, weekday: Weekday) -> &'static [&'static str] {
    table
        .at(crate::authority::weekday_index(weekday))
        .unwrap_or(&[])
}

/// Every tradition's answer for one weekday, paired with its authority.
pub fn all_for_weekday(
    weekday: Weekday,
) -> impl Iterator<Item = (&'static Authority, &'static [&'static str])> {
    let index = crate::authority::weekday_index(weekday);
    ALL.into_iter()
        .filter_map(move |table| Some((table.authority(), table.at(index).ok()?)))
}

/// The planet a weekday is named for, in English, as every tradition here
/// agrees it is.
///
/// This is the one attribution the traditions do not dispute, which is why
/// it can be a single function rather than a set of named tables: the
/// Germanic substitution, the 五行 naming and the *navagraha* are all
/// renamings of the same seven bodies in the same order. A test checks that
/// each table's own entries carry the same planet.
#[must_use]
pub fn planet(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Sunday => "Sun",
        Weekday::Monday => "Moon",
        Weekday::Tuesday => "Mars",
        Weekday::Wednesday => "Mercury",
        Weekday::Thursday => "Jupiter",
        Weekday::Friday => "Venus",
        Weekday::Saturday => "Saturn",
    }
}

#[cfg(test)]
mod tests {
    use hc_calendar::Rd;

    use super::*;

    #[test]
    fn every_tradition_has_an_entry_for_all_seven_days() {
        for table in ALL {
            assert_eq!(table.len(), 7, "{}", table.authority().id);
            assert!(table.is_complete(), "{}", table.authority().id);
            for weekday in Weekday::ALL {
                assert!(
                    !attribution(table, weekday).is_empty(),
                    "{} {weekday:?}",
                    table.authority().id
                );
            }
        }
    }

    #[test]
    fn every_tradition_names_a_source_and_has_a_unique_id() {
        for (position, table) in ALL.iter().enumerate() {
            assert!(!table.authority().source.is_empty());
            for other in &ALL[position + 1..] {
                assert_ne!(table.authority().id, other.authority().id);
            }
        }
    }

    #[test]
    fn the_tables_are_indexed_sunday_first_like_the_planetary_week() {
        assert_eq!(
            attribution(&WEEKDAY_PLANETS_GRECO_ROMAN, Weekday::Sunday)[0],
            "dies Solis"
        );
        assert_eq!(
            attribution(&WEEKDAY_PLANETS_GRECO_ROMAN, Weekday::Saturday)[0],
            "dies Saturni"
        );
        assert_eq!(crate::authority::weekday_index(Weekday::Sunday), 0);
    }

    /// The traditions are translations of one another, so the planet is the
    /// same in every one of them. This is what makes the lists comparable
    /// at all.
    #[test]
    fn the_latin_japanese_and_sanskrit_tables_all_name_the_same_planet() {
        for weekday in Weekday::ALL {
            let expected = planet(weekday);
            for table in [
                &WEEKDAY_PLANETS_GRECO_ROMAN,
                &WEEKDAY_LUMINARIES_JAPANESE,
                &WEEKDAY_VASARA_SANSKRIT,
            ] {
                assert!(
                    attribution(table, weekday).contains(&expected),
                    "{} does not name {expected} on {weekday:?}",
                    table.authority().id
                );
            }
        }
    }

    /// Saturday is where *interpretatio germanica* stopped, and the table
    /// must keep saying so rather than growing a plausible-looking Norse
    /// name.
    #[test]
    fn saturday_has_no_germanic_deity_and_the_table_says_so() {
        let saturday = attribution(&WEEKDAY_DEITIES_NORSE, Weekday::Saturday);
        assert_eq!(saturday.len(), 1);
        assert!(saturday[0].contains("no Germanic substitution"));
        assert!(
            WEEKDAY_DEITIES_NORSE
                .authority()
                .caveat
                .unwrap_or("")
                .contains("laugardagr")
        );
        // Every other day does have one.
        for weekday in [
            Weekday::Sunday,
            Weekday::Monday,
            Weekday::Tuesday,
            Weekday::Wednesday,
            Weekday::Thursday,
            Weekday::Friday,
        ] {
            assert!(
                !attribution(&WEEKDAY_DEITIES_NORSE, weekday)[0].contains("no Germanic"),
                "{weekday:?}"
            );
        }
        // And the Old English name for Saturday is the Roman god's.
        assert_eq!(
            attribution(&WEEKDAY_NAMES_OLD_ENGLISH, Weekday::Saturday),
            &["sæternesdæġ"]
        );
    }

    #[test]
    fn tuesday_is_mars_under_five_different_names() {
        assert!(attribution(&WEEKDAY_PLANETS_GRECO_ROMAN, Weekday::Tuesday).contains(&"Mars"));
        assert!(attribution(&WEEKDAY_DEITIES_NORSE, Weekday::Tuesday).contains(&"Týr"));
        assert_eq!(
            attribution(&WEEKDAY_NAMES_OLD_ENGLISH, Weekday::Tuesday),
            &["tīwesdæġ"]
        );
        assert!(attribution(&WEEKDAY_LUMINARIES_JAPANESE, Weekday::Tuesday).contains(&"火曜日"));
        assert!(attribution(&WEEKDAY_VASARA_SANSKRIT, Weekday::Tuesday).contains(&"Maṅgala"));
        assert_eq!(
            attribution(&WEEKDAY_COLOURS_THAI, Weekday::Tuesday),
            &["pink"]
        );
    }

    #[test]
    fn the_thai_colours_are_the_seven_everyday_ones() {
        let expected: [&str; 7] = [
            "red",
            "yellow",
            "pink",
            "green",
            "orange",
            "light blue",
            "purple",
        ];
        // Monday carries a second reading, cream, so compare the first
        // entry of each day rather than the whole entry.
        for (index, colour) in expected.into_iter().enumerate() {
            assert_eq!(
                WEEKDAY_COLOURS_THAI.at(index).unwrap_or(&[])[0],
                colour,
                "index {index}"
            );
        }
        assert_eq!(
            attribution(&WEEKDAY_COLOURS_THAI, Weekday::Monday),
            &["yellow", "cream"]
        );
    }

    #[test]
    fn the_thai_colours_and_deities_line_up_day_for_day() {
        for weekday in Weekday::ALL {
            assert!(!attribution(&WEEKDAY_COLOURS_THAI, weekday).is_empty());
            assert!(!attribution(&WEEKDAY_DEITIES_THAI, weekday).is_empty());
        }
        // Sunday's god is Surya, the Sun, and its colour is red.
        assert!(attribution(&WEEKDAY_DEITIES_THAI, Weekday::Sunday).contains(&"Surya"));
        assert_eq!(
            attribution(&WEEKDAY_COLOURS_THAI, Weekday::Sunday),
            &["red"]
        );
    }

    /// The weekday stones are a third system again, and share almost
    /// nothing with either of the other two.
    #[test]
    fn the_weekday_stones_are_not_the_month_stones_or_the_sign_stones() {
        // Sunday is topaz and diamond; no month list makes January's
        // stone either.
        assert_eq!(
            attribution(&WEEKDAY_STONES_KUNZ, Weekday::Sunday),
            &["topaz", "diamond"]
        );
        assert_eq!(
            crate::birthstones::BIRTHSTONES_US_2016.at(0),
            Ok(&["garnet"][..])
        );
        // Lodestone appears in no other table in the crate.
        assert!(WEEKDAY_STONES_KUNZ.names("lodestone"));
        assert!(!crate::birthstones::BIRTHSTONES_US_2016.names("lodestone"));
        assert!(!crate::zodiac_stones::ZODIAC_STONES_KUNZ.names("lodestone"));
    }

    #[test]
    fn asking_every_tradition_at_once_returns_one_row_each() {
        assert_eq!(all_for_weekday(Weekday::Friday).count(), ALL.len());
        for (authority, entry) in all_for_weekday(Weekday::Friday) {
            assert!(!entry.is_empty(), "{}", authority.id);
        }
    }

    /// RD 738 886 is 1 January 2024, a Monday. The attributions follow the
    /// weekday, so the whole chain from a fixed day to a Thai colour has to
    /// hold together.
    #[test]
    fn a_fixed_day_resolves_all_the_way_to_a_colour() {
        let weekday = Weekday::from_rd(Rd(738_886));
        assert_eq!(weekday, Weekday::Monday);
        assert_eq!(planet(weekday), "Moon");
        assert_eq!(
            attribution(&WEEKDAY_COLOURS_THAI, weekday),
            &["yellow", "cream"]
        );
        assert!(attribution(&WEEKDAY_LUMINARIES_JAPANESE, weekday).contains(&"月曜日"));
    }

    #[test]
    fn walking_a_fortnight_of_days_never_leaves_a_table_without_an_answer() {
        for offset in 0..14i64 {
            let weekday = Weekday::from_rd(Rd(738_886 + offset));
            for table in ALL {
                assert!(!attribution(table, weekday).is_empty());
            }
        }
    }
}

/// The table whose authority has this identifier.
#[must_use]
pub fn by_id(id: &str) -> Option<&'static WeekdayTable> {
    ALL.iter().copied().find(|table| table.authority().id == id)
}

hc_core::catalogue_tests! {
    type: &'static WeekdayTable,
    id: |table| table.authority().id,
    provenance: |table| table.authority().source,
    tests: weekday_table_tests,
    all: &ALL,
    lookup: by_id,
}
