//! The 33 sample dates of *Calendrical Calculations* through every calendar
//! here that carries a column of the book's tables.
//!
//! Reingold and Dershowitz's published code, `calendar-code2`, lists the
//! sample dates of the book's Appendix C in `dates.l` — R.D. −214 193 to
//! 764 652, 586 BCE to 2094 — and the calendars its seven tables convert
//! them into, and writes the tables by running `calendar.l`
//! (`reingold2018code`). `data/calendrica_sample_dates.txt` is that
//! output, produced by the code itself; its header says how, under which
//! licence, and how it was checked. This file maps every column to what
//! this library has, converts every date through it, compares, converts
//! back, and holds every disagreement to a list that says why.
//!
//! Three outcomes are allowed for a date and a mapping, and nothing else:
//!
//! * **The same value**, after the relabelling the mapping names (the
//!   Hebrew months counted from Tishri here and from Nisan there, the haab
//!   day from 1 here and from 0 there), and the date converts back to its
//!   R.D.
//! * **A refusal**, `BeforeEpoch` or `AfterSupportedRange` for a calendar
//!   (or `None` from a function), exactly when the date is outside the range
//!   the calendar or function declares. The book computes most of its
//!   calendars proleptically back to 586 BCE; this library does not guess
//!   there, and the test holds it to that.
//! * **A known difference**, listed in [`KNOWN`] with its reason: a
//!   different place or a published table against a computed rule, a
//!   crescent on the criterion's edge, or the reference code's own time
//!   scale, which its errata correct (`reingold2018errata`, correction 15).
//!
//! Over the 55 mappings that is 1 275 agreements, 490 refusals and 17 known
//! differences, and 1 134 round trips; [`SAME`], [`REFUSED`] and
//! [`ROUND_TRIPS`] hold the counts so that they move only deliberately. The
//! astronomical columns are compared to a bound instead, in
//! [`the_astronomy_is_within_seconds_of_the_books`], and the rising and
//! setting columns, each under the book's own horizon, in
//! [`the_rising_and_setting_are_within_seconds_of_the_books`]. The columns
//! with no counterpart here, or none held to the book's, are
//! [`NOT_CARRIED`], each with the reason, and a test checks that every
//! column of the file is one of the four. The Japanese, Korean and Vietnamese calendars are not
//! among the columns: `dates.l` has none for them.

#![cfg(all(
    feature = "alloc",
    feature = "civil",
    feature = "lunar",
    feature = "regional",
    feature = "indic",
    feature = "equinox",
    feature = "holiday",
    feature = "format",
))]
#![expect(
    clippy::expect_used,
    reason = "a table line that does not parse or a calendar that is not \
              registered is a failed test, and the message says which"
)]

use std::collections::{BTreeMap, BTreeSet};

use hyper_calendar::hc_astro::{Horizon, Location, Moment};
use hyper_calendar::hc_calendar::weekday::Weekday;
use hyper_calendar::hc_calendar::{
    CalendarError, CalendarId, CalendarRegistry, DateFields, DynAdapter, DynCalendar, Rd,
};
use hyper_calendar::hc_calendars_indic::HinduLunarCalendar;
use hyper_calendar::hc_calendars_indic::hindu_solar::{self, SankrantiRule, SolarModel};
use hyper_calendar::hc_calendars_indic::places::UJJAIN;
use hyper_calendar::hc_calendars_solar::{gregorian, julian};
use hyper_calendar::hc_format::roman::{Anchor, BissextileStyle, RomanDayName};
use hyper_calendar::hc_holiday::computus;
use hyper_calendar::hc_seasons::zodiac::Ayanamsa;

const TABLE: &str = include_str!("data/calendrica_sample_dates.txt");

/// Every line of the table, by column, in the file's order of dates.
fn table() -> BTreeMap<&'static str, Vec<(Rd, Vec<&'static str>)>> {
    let mut columns: BTreeMap<_, Vec<_>> = BTreeMap::new();
    for line in TABLE.lines().filter(|line| !line.starts_with('#')) {
        let mut parts = line.split('\t');
        let name = parts.next().expect("a column name");
        let rd = Rd(parts
            .next()
            .and_then(|rd| rd.parse().ok())
            .expect("an R.D."));
        let fields = parts.next().expect("fields").split(' ').collect();
        columns.entry(name).or_default().push((rd, fields));
    }
    columns
}

fn n(value: impl Into<i64>) -> String {
    value.into().to_string()
}

fn b(value: bool) -> String {
    String::from(if value { "t" } else { "f" })
}

fn month(fields: &DateFields) -> u8 {
    fields.month.expect("a month").ordinal
}

fn leap_month(fields: &DateFields) -> bool {
    fields.month.expect("a month").leap
}

fn day(fields: &DateFields) -> u8 {
    fields.day.expect("a day")
}

fn extra(fields: &DateFields, name: &str) -> String {
    n(fields.extra.get(name).expect(name))
}

/// The year the book writes: no year zero, so 1 BCE is −1.
fn without_year_zero(year: i64) -> i64 {
    if year <= 0 { year - 1 } else { year }
}

/// How a calendar's fields are written as the book's.
type Adapter = fn(&DateFields, Rd, &dyn DynCalendar) -> Vec<String>;

fn ymd(fields: &DateFields, _: Rd, _: &dyn DynCalendar) -> Vec<String> {
    vec![n(fields.year), n(month(fields)), n(day(fields))]
}

/// The Hebrew months, public here from Tishri with Adar I as a leap 5 and
/// Adar or Adar II as 6, in the book from Nisan with Adar or Adar I as 12
/// and Adar II as 13. Which Adar a 6 is depends on the year, and the
/// observational calendar's leap years are its own.
fn hebrew(fields: &DateFields, _: Rd, calendar: &dyn DynCalendar) -> Vec<String> {
    let ordinal = month(fields);
    let leap_year = calendar.is_leap_year(fields.year).expect("a year in range");
    let book = match (ordinal, leap_month(fields)) {
        (5, true) => 12,
        (6, false) => {
            if leap_year {
                13
            } else {
                12
            }
        }
        (ordinal, _) if ordinal >= 7 => ordinal - 6,
        (ordinal, _) => ordinal + 6,
    };
    vec![n(fields.year), n(book), n(day(fields))]
}

/// The Badíʿ date as the book writes it: Kull-i-Shayʾ, Váḥid, year of the
/// Váḥid, month, day.
fn bahai(fields: &DateFields, _: Rd, _: &dyn DynCalendar) -> Vec<String> {
    vec![
        extra(fields, "kull-i-shay"),
        extra(fields, "vahid"),
        extra(fields, "year-of-vahid"),
        n(month(fields)),
        n(day(fields)),
    ]
}

/// A Hindu lunisolar date in the book's order, the year in the Vikrama
/// era.
fn hindu_lunar(fields: &DateFields, _: Rd, _: &dyn DynCalendar) -> Vec<String> {
    vec![
        extra(fields, "vikrama-year"),
        n(month(fields)),
        b(leap_month(fields)),
        n(day(fields)),
        b(fields.leap_day),
    ]
}

/// A calendar a mapping converts through: one of the registry's, or one
/// the test builds.
enum Carrier<'a> {
    Registered(&'a (dyn DynCalendar + Send + Sync)),
    Built(Box<dyn DynCalendar>),
}

impl Carrier<'_> {
    fn calendar(&self) -> &dyn DynCalendar {
        match self {
            Self::Registered(calendar) => *calendar,
            Self::Built(calendar) => calendar.as_ref(),
        }
    }
}

/// Naw-Rúz 172 BE, from which the Badíʿ as kept is astronomical.
fn bahai_reform() -> Rd {
    gregorian::to_fixed(2015, 3, 21).expect("a Gregorian date")
}

/// A column and what carries it here.
struct Mapping<'a> {
    column: &'static str,
    /// The registered identifier, or a description of the calendar the
    /// test builds.
    ours: &'static str,
    carrier: Carrier<'a>,
    adapter: Adapter,
    /// The dates the mapping holds for; every date unless the calendar
    /// changes rule within the table's span.
    applies: fn(Rd) -> bool,
}

/// Every column this library carries as a calendar, and how.
fn mappings(registry: &CalendarRegistry) -> Vec<Mapping<'_>> {
    let registered = |column: &'static str, id: &'static str, adapter: Adapter| Mapping {
        column,
        ours: id,
        carrier: Carrier::Registered(registry.get_by_name(id).expect(id)),
        adapter,
        applies: |_| true,
    };
    let built = |column: &'static str,
                 ours: &'static str,
                 calendar: Box<dyn DynCalendar>,
                 adapter: Adapter| Mapping {
        column,
        ours,
        carrier: Carrier::Built(calendar),
        adapter,
        applies: |_| true,
    };
    vec![
        registered("jd", "julian-day", |f, _, _| {
            // The book's Julian date is the moment of the day's midnight,
            // half a day before the Julian Day Number's noon.
            vec![format!("{:?}", f.year as f64 - 0.5)]
        }),
        registered("mjd", "modified-julian-day", |f, _, _| vec![n(f.year)]),
        registered("gregorian", "gregory", ymd),
        registered("julian", "julian", |f, _, _| {
            let year = if f.era == Some("bc") { -f.year } else { f.year };
            vec![n(year), n(month(f)), n(day(f))]
        }),
        registered("olympiad", "olympiad", |f, _, _| {
            vec![extra(f, "olympiad"), extra(f, "year-of-olympiad")]
        }),
        registered("egyptian", "egyptian", ymd),
        registered("armenian", "armenian", ymd),
        registered("akan-name", "akan", |f, _, _| {
            // The book numbers both weeks from its epoch, R.D. 37, and names
            // neither in the code; here the six-day week counts from Fo and
            // the seven-day from Sunday. One relabelling maps all 33 days,
            // so the two cycles are in phase.
            let prefix = f.extra.get("nnanson").expect("nnanson");
            let stem = f.extra.get("nnawotwe").expect("nnawotwe");
            vec![
                n((prefix + 4).rem_euclid(6) + 1),
                n((stem + 3).rem_euclid(7) + 1),
            ]
        }),
        registered("coptic", "coptic", ymd),
        registered("ethiopic", "ethiopic", ymd),
        registered("iso", "iso8601-week", |f, _, _| {
            vec![n(f.year), extra(f, "week"), extra(f, "day-of-week")]
        }),
        registered("icelandic", "icelandic", |f, rd, _| {
            // Seasons 1 and 2 here are the book's summer (90) and winter
            // (270); its weekday counts from Sunday as 0, as C's `tm_wday` does.
            let season = match f.extra.get("season") {
                Some(1) => 90,
                Some(2) => 270,
                other => panic!("season {other:?}"),
            };
            vec![
                n(f.year),
                n(season),
                extra(f, "week"),
                n(Weekday::from_rd(rd).sunday_first_number()),
            ]
        }),
        registered("islamic", "islamic-civil", ymd),
        // The book's observational Islamic calendar is judged at Cairo,
        // `islamic-location`; islamic-rgsa is the same prediction at Mecca.
        registered(
            "observational-islamic",
            "islamic-observational-cairo-rd",
            ymd,
        ),
        registered("observational-islamic", "islamic-rgsa", ymd),
        // The book's computed Saudi rule, and the published table.
        registered("saudi-islamic", "islamic-saudi-rule-rd", ymd),
        registered("saudi-islamic", "islamic-umalqura", ymd),
        registered("hebrew", "hebrew", hebrew),
        registered("observational-hebrew", "hebrew-observational", hebrew),
        registered("persian", "persian", ymd),
        registered("persian", "persian-apparent-noon", ymd),
        registered("arithmetic-persian", "persian-arithmetic", ymd),
        registered("bahai", "bahai-arithmetic", bahai),
        // The Badíʿ as kept is the arithmetic calendar until 171 BE and the
        // astronomical one, as the Bahá'í World Centre tabulates it, from
        // 172 BE, 21 March 2015.
        Mapping {
            applies: |rd| rd < bahai_reform(),
            ..registered("bahai", "bahai", bahai)
        },
        registered("astro-bahai", "bahai-astronomical", bahai),
        Mapping {
            applies: |rd| rd >= bahai_reform(),
            ..registered("astro-bahai", "bahai", bahai)
        },
        registered("french", "french-republican-equinox", ymd),
        registered("arithmetic-french", "french-republican-arithmetic", ymd),
        registered("mayan-long-count", "maya-longcount", |f, _, _| {
            ["baktun", "katun", "tun", "uinal", "kin"]
                .iter()
                .map(|name| extra(f, name))
                .collect()
        }),
        registered("mayan-haab", "maya-haab", |f, _, _| {
            // The haab counts its days from 0; the generic day field from 1.
            vec![n(month(f)), n(day(f) - 1)]
        }),
        registered("mayan-tzolkin", "maya-tzolkin", |f, _, _| {
            vec![extra(f, "tzolkin_number"), extra(f, "tzolkin_name")]
        }),
        registered("aztec-xihuitl", "aztec-xiuhpohualli", |f, _, _| {
            vec![n(month(f)), n(day(f))]
        }),
        registered("aztec-tonalpohualli", "aztec-tonalpohualli", |f, _, _| {
            vec![
                extra(f, "tonalpohualli_number"),
                extra(f, "tonalpohualli_sign"),
            ]
        }),
        registered("bali-pawukon", "balinese-pawukon", |f, _, _| {
            // The book's ten-day week runs 0 to 9 and this one 1 to 10, the
            // book's 0 being 10 here; Luang is the book's even ten-day day.
            // The seven-day week is the generic day field.
            let dasawara = f.extra.get("dasawara").expect("dasawara");
            let mut out = vec![b(dasawara % 2 == 0)];
            for name in ["dwiwara", "triwara", "caturwara", "pancawara", "sadwara"] {
                out.push(extra(f, name));
            }
            out.push(n(day(f)));
            out.push(extra(f, "astawara"));
            out.push(extra(f, "sangawara"));
            out.push(n(dasawara % 10));
            out
        }),
        registered("babylonian", "babylonian", |f, _, _| {
            vec![n(f.year), n(month(f)), b(leap_month(f)), n(day(f))]
        }),
        registered("samaritan", "samaritan", |f, _, _| {
            // Months here from where the year number changes, the Sixth
            // Month as 1 and the Thirteenth as a leap 7; in the book from
            // the month of Passover.
            let ordinal = month(f);
            let book = if leap_month(f) {
                13
            } else if ordinal <= 7 {
                ordinal + 5
            } else {
                ordinal - 7
            };
            vec![n(f.year), n(book), n(day(f))]
        }),
        registered("chinese", "chinese", |f, _, _| {
            vec![
                extra(f, "cycle"),
                extra(f, "year_of_cycle"),
                n(month(f)),
                b(leap_month(f)),
                n(day(f)),
            ]
        }),
        registered("chinese-day-name", "sexagenary", |f, _, _| {
            vec![extra(f, "stem"), extra(f, "branch")]
        }),
        registered("old-hindu-solar", "hindu-old-solar", ymd),
        registered("old-hindu-lunar", "hindu-old-lunar", |f, _, _| {
            vec![n(f.year), n(month(f)), b(leap_month(f)), n(day(f))]
        }),
        // The book's modern solar calendar, on the Sūrya Siddhānta's Sun and
        // read at its sunrise at Ujjain, in Śaka years.
        registered("hindu-solar", "hindu-solar-surya-siddhanta", ymd),
        // The book's two astronomical calendars are read at Ujjain, and are
        // built here rather than registered: they are hindu-solar-tamil and
        // hindu-lunar at another place, which is a parameter and not a
        // convention (policy §5), and docs/systems/hindu-calendars.md says
        // how often the place moves a date.
        built(
            "astro-hindu-solar",
            "the Tamil rule at Ujjain with the true Sun and Lahiri's ayanamsa, built",
            Box::new(DynAdapter::new(hindu_solar::TAMIL.new(
                CalendarId("x-hindu-solar-tamil-ujjain"),
                UJJAIN,
                SolarModel::Modern(Ayanamsa::LAHIRI),
            ))),
            ymd,
        ),
        // The same at Ujjain with the book's own ayanāṃśa, zero at the Sūrya
        // Siddhānta's Meṣa saṅkrānti of 285 CE, and its own sunset, the
        // Sun's centre on the geometric horizon (`sidereal-start`,
        // `astro-hindu-sunset`).
        built(
            "astro-hindu-solar",
            "the Tamil rule at Ujjain with the book's ayanamsa and sunset, built",
            Box::new(DynAdapter::new(hindu_solar::HinduSolarCalendar {
                rule: SankrantiRule::BeforeCentreSets,
                ..hindu_solar::TAMIL.new(
                    CalendarId("x-hindu-solar-tamil-book"),
                    UJJAIN,
                    SolarModel::Modern(Ayanamsa::REINGOLD_DERSHOWITZ),
                )
            })),
            ymd,
        ),
        registered("astro-hindu-solar", "hindu-solar-tamil", ymd),
        built(
            "astro-hindu-lunar",
            "HinduLunarCalendar::UJJAIN",
            Box::new(DynAdapter::new(HinduLunarCalendar::UJJAIN)),
            hindu_lunar,
        ),
        built(
            "astro-hindu-lunar",
            "the lunisolar calendar at Ujjain with the book's ayanamsa, built",
            Box::new(DynAdapter::new(HinduLunarCalendar::new(
                UJJAIN,
                Ayanamsa::REINGOLD_DERSHOWITZ,
            ))),
            hindu_lunar,
        ),
        registered("astro-hindu-lunar", "hindu-lunar", hindu_lunar),
        // The book's modern lunisolar calendar, on the Sūrya Siddhānta's
        // Sun and Moon and read at its sunrise at Ujjain.
        registered("hindu-lunar", "hindu-lunar-surya-siddhanta", hindu_lunar),
        registered("tibetan", "tibetan", |f, _, _| {
            // The year here is the Western year it begins in; the book's is
            // 127 more, its epoch year being −127.
            vec![
                n(f.year + 127),
                n(month(f)),
                b(leap_month(f)),
                n(day(f)),
                b(f.leap_day),
            ]
        }),
    ]
}

/// A column carried by a function rather than a calendar.
struct Function {
    column: &'static str,
    ours: &'static str,
    /// The value, or `None` where the function declares the date outside
    /// its range.
    value: fn(Rd) -> Option<Vec<String>>,
    /// Whether the function's range holds the date.
    in_range: fn(Rd) -> bool,
}

fn gregorian_year(rd: Rd) -> i64 {
    gregorian::year_from_fixed(rd).expect("a Gregorian year")
}

fn gregorian_date(rd: Rd) -> Vec<String> {
    let (year, month, day) = gregorian::from_fixed(rd).expect("a Gregorian date");
    vec![n(year), n(month), n(day)]
}

fn functions() -> Vec<Function> {
    vec![
        Function {
            column: "day",
            ours: "Weekday::from_rd",
            value: |rd| Some(vec![String::from(Weekday::from_rd(rd).english_name())]),
            in_range: |_| true,
        },
        Function {
            column: "unix",
            ours: "Rd::to_unix_days",
            value: |rd| Some(vec![n(rd.to_unix_days() * 86_400)]),
            in_range: |_| true,
        },
        Function {
            column: "roman",
            ours: "hc_format::roman::RomanDayName::of_fixed",
            value: |rd| {
                let name = RomanDayName::of_fixed(rd, BissextileStyle::Doubled).ok()?;
                let (year, month, _) = julian::from_fixed(rd).ok()?;
                // A day after the Ides of December counts to the Kalends of
                // the next year's January.
                let year = if name.anchor_month == 1 && month == 12 {
                    year + 1
                } else {
                    year
                };
                let event = match name.anchor {
                    Anchor::Kalends => 1,
                    Anchor::Nones => 2,
                    Anchor::Ides => 3,
                };
                Some(vec![
                    n(without_year_zero(year)),
                    n(name.anchor_month),
                    n(event),
                    n(name.count),
                    b(name.bissextile),
                ])
            },
            in_range: |_| true,
        },
        Function {
            column: "orthodox-easter",
            ours: "hc_holiday::computus::orthodox_easter",
            value: |rd| computus::orthodox_easter(gregorian_year(rd)).map(gregorian_date),
            in_range: |rd| (326..=computus::COMPUTUS_LAST_YEAR).contains(&gregorian_year(rd)),
        },
        Function {
            column: "easter",
            ours: "hc_holiday::computus::gregorian_easter",
            value: |rd| computus::gregorian_easter(gregorian_year(rd)).map(gregorian_date),
            in_range: |rd| (1583..=computus::COMPUTUS_LAST_YEAR).contains(&gregorian_year(rd)),
        },
        Function {
            column: "astronomical-easter",
            ours: "hc_holiday::computus::astronomical_easter",
            value: |rd| computus::astronomical_easter(gregorian_year(rd)).map(gregorian_date),
            in_range: |rd| {
                (computus::ASTRONOMICAL_EASTER_FIRST_YEAR..=computus::ASTRONOMICAL_EASTER_LAST_YEAR)
                    .contains(&gregorian_year(rd))
            },
        },
    ]
}

/// A disagreement that is understood, and why.
struct Known {
    column: &'static str,
    ours: &'static str,
    rd: i64,
    reason: &'static str,
}

/// Every disagreement, with its reason. `docs/systems/hijri.md`,
/// `hebrew-observational.md` and `hindu-calendars.md` give the
/// measurements behind each.
const KNOWN: &[Known] = &[
    Known {
        column: "observational-islamic",
        ours: "islamic-rgsa",
        rd: 709_409,
        reason: "a different place: islamic-rgsa observes from Mecca, the book's \
                 sample calendar from Cairo, and at Cairo this library gives the \
                 book's date",
    },
    Known {
        column: "saudi-islamic",
        ours: "islamic-umalqura",
        rd: 708_842,
        reason: "the Umm al-Qura table against the book's computed rule, in 1360 AH, \
                 before 1392 AH, the first year the rules are known for",
    },
    Known {
        column: "saudi-islamic",
        ours: "islamic-umalqura",
        rd: 709_580,
        reason: "the Umm al-Qura table against the book's computed rule, in 1362 AH, \
                 before 1392 AH",
    },
    Known {
        column: "saudi-islamic",
        ours: "islamic-umalqura",
        rd: 764_652,
        reason: "the Umm al-Qura table against the book's computed rule, in 1518 AH: \
                 on the evening of 12 July 2094 the book's Moon sets 4.3 minutes after \
                 the Sun at Mecca, and the table begins Rabīʿ I a day later",
    },
    Known {
        column: "observational-hebrew",
        ours: "hebrew-observational",
        rd: 25_469,
        reason: "a crescent on the criterion's edge: on the evening that begins \
                 Tishri 3831 (70 CE) the book's Moon is 4.13° high against the 4.1° \
                 the criterion asks; this library judges the evening 20 s later, from \
                 its interpolated 4.5° instant and its ΔT, and finds it at 4.06°",
    },
    Known {
        column: "astro-hindu-solar",
        ours: "the Tamil rule at Ujjain with the true Sun and Lahiri's ayanamsa, built",
        rd: 664_224,
        reason: "the book's code reads the sign at Ujjain's sunset in standard time \
                 as Universal Time, five hours late, which its errata correct \
                 (reingold2018errata, correction 15); the saṅkrānti of 1819 fell 3.6 h \
                 after sunset, so the Tamil rule begins the month the next day",
    },
    Known {
        column: "astro-hindu-solar",
        ours: "the Tamil rule at Ujjain with the true Sun and Lahiri's ayanamsa, built",
        rd: 694_799,
        reason: "as for 664224: the saṅkrānti of 1903 fell 1.9 h after sunset",
    },
    Known {
        column: "astro-hindu-solar",
        ours: "the Tamil rule at Ujjain with the true Sun and Lahiri's ayanamsa, built",
        rd: 744_313,
        reason: "as for 664224: the saṅkrānti of 2038 fell 4.1 h after sunset",
    },
    Known {
        column: "astro-hindu-solar",
        ours: "hindu-solar-tamil",
        rd: 664_224,
        reason: "as for the Tamil rule at Ujjain",
    },
    Known {
        column: "astro-hindu-solar",
        ours: "hindu-solar-tamil",
        rd: 694_799,
        reason: "as for the Tamil rule at Ujjain",
    },
    Known {
        column: "astro-hindu-solar",
        ours: "hindu-solar-tamil",
        rd: 744_313,
        reason: "as for the Tamil rule at Ujjain",
    },
    Known {
        column: "astro-hindu-lunar",
        ours: "HinduLunarCalendar::UJJAIN",
        rd: 764_652,
        reason: "the book's code reads the tithi at Ujjain's sunrise in standard time \
                 as Universal Time, five hours late, which its errata correct \
                 (reingold2018errata, correction 15); at the sunrise itself its own \
                 functions give tithi 5 on 17 and 18 July 2094, the second a leap day, \
                 as this library does, and five hours later tithi 6",
    },
    Known {
        column: "astro-hindu-lunar",
        ours: "hindu-lunar",
        rd: 764_652,
        reason: "as for HinduLunarCalendar::UJJAIN",
    },
    Known {
        column: "astro-hindu-lunar",
        ours: "the lunisolar calendar at Ujjain with the book's ayanamsa, built",
        rd: 764_652,
        reason: "as for HinduLunarCalendar::UJJAIN: the book's own ayanāṃśa does not \
                 move it",
    },
    Known {
        column: "astro-hindu-solar",
        ours: "the Tamil rule at Ujjain with the book's ayanamsa and sunset, built",
        rd: 664_224,
        reason: "as for the Tamil rule at Ujjain with Lahiri's ayanāṃśa: the book's own \
                 ayanāṃśa and sunset do not move it",
    },
    Known {
        column: "astro-hindu-solar",
        ours: "the Tamil rule at Ujjain with the book's ayanamsa and sunset, built",
        rd: 694_799,
        reason: "as for the Tamil rule at Ujjain with Lahiri's ayanāṃśa: the book's own \
                 ayanāṃśa and sunset do not move it",
    },
    Known {
        column: "astro-hindu-solar",
        ours: "the Tamil rule at Ujjain with the book's ayanamsa and sunset, built",
        rd: 744_313,
        reason: "as for the Tamil rule at Ujjain with Lahiri's ayanāṃśa: the book's own \
                 ayanāṃśa and sunset do not move it",
    },
];

/// The columns this library has no counterpart for, or does not hold to
/// the book's, and why.
const NOT_CARRIED: &[(&str, &str)] = &[];

/// The astronomical columns [`the_astronomy_is_within_seconds_of_the_books`]
/// compares, each with its bound, in seconds of time or of arc.
const ASTRONOMY: &[(&str, f64)] = &[
    ("ephem-corr", 11.0),
    // The book's equation of time takes the mean Sun at dynamical time
    // against a Universal Time clock, and this library's takes it at
    // Universal Time, from sidereal time: the two part by the book's
    // sundial lead, [`sundial_lead`], 38.4 s in 586 BCE, 2.7 s in 1013 and
    // 0.2 s in 2038. The comparison adds that lead to this library's value,
    // and what is left is the book's shorter series against this library's
    // VSOP87 Sun, at most 2.87 s, on R.D. 434 355 (1189).
    ("eqn-of-time", 3.0),
    ("solar-long", 1.6),
    ("solstice", 45.0),
    ("lunar-long", 6.0),
    ("lunar-lat", 0.4),
    ("lunar-alt", 20.0),
    ("new-moon-after", 12.0),
    ("major-solar-term-on-or-after", 40.0),
];

/// What one mapping or function did with the 33 dates.
#[derive(Default)]
struct Tally {
    same: usize,
    refused: usize,
    round_trips: usize,
    differ: Vec<i64>,
}

fn is_range_refusal(error: CalendarError) -> bool {
    matches!(
        error,
        CalendarError::BeforeEpoch | CalendarError::AfterSupportedRange
    )
}

fn run_calendar(mapping: &Mapping, rows: &[(Rd, Vec<&str>)]) -> Tally {
    let calendar = mapping.carrier.calendar();
    let meta = calendar.meta();
    let mut tally = Tally::default();
    for (rd, book) in rows.iter().filter(|(rd, _)| (mapping.applies)(*rd)) {
        let label = format!("{} as {} at R.D. {}", mapping.column, mapping.ours, rd.0);
        match calendar.fixed_to_fields(*rd) {
            Err(error) => {
                assert!(is_range_refusal(error), "{label}: refused with {error:?}");
                assert!(
                    !meta.supports(*rd),
                    "{label}: refused inside the declared range"
                );
                tally.refused += 1;
            }
            Ok(fields) => {
                assert!(
                    meta.supports(*rd),
                    "{label}: answered outside the declared range"
                );
                assert_eq!(
                    calendar.fields_to_fixed(&fields),
                    Ok(*rd),
                    "{label}: {fields:?} does not convert back"
                );
                tally.round_trips += 1;
                let ours = (mapping.adapter)(&fields, *rd, calendar);
                if ours == *book {
                    tally.same += 1;
                } else {
                    tally.differ.push(rd.0);
                }
            }
        }
    }
    tally
}

fn run_function(function: &Function, rows: &[(Rd, Vec<&str>)]) -> Tally {
    let mut tally = Tally::default();
    for (rd, book) in rows {
        let label = format!("{} as {} at R.D. {}", function.column, function.ours, rd.0);
        match (function.value)(*rd) {
            None => {
                assert!(
                    !(function.in_range)(*rd),
                    "{label}: refused inside its range"
                );
                tally.refused += 1;
            }
            Some(ours) => {
                assert!(
                    (function.in_range)(*rd),
                    "{label}: answered outside its range"
                );
                if ours == *book {
                    tally.same += 1;
                } else {
                    tally.differ.push(rd.0);
                }
            }
        }
    }
    tally
}

#[test]
fn every_column_is_carried_or_said_not_to_be() {
    let columns = table();
    let registry = hyper_calendar::registry();
    let mapped: BTreeSet<&str> = mappings(&registry)
        .iter()
        .map(|m| m.column)
        .chain(functions().iter().map(|f| f.column))
        .collect();
    let not_carried: BTreeSet<&str> = NOT_CARRIED
        .iter()
        .map(|(column, _)| *column)
        .chain(ASTRONOMY.iter().map(|(column, _)| *column))
        .chain(RISE_AND_SET.iter().map(|(column, _)| *column))
        .collect();
    assert!(mapped.is_disjoint(&not_carried));
    let named: BTreeSet<&str> = mapped.union(&not_carried).copied().collect();
    let in_file: BTreeSet<&str> = columns.keys().copied().collect();
    assert_eq!(named, in_file);
    // Sixty columns of the thirty-three dates, as dates.l's seven lists.
    assert_eq!(in_file.len(), 60);
    for rows in columns.values() {
        assert_eq!(rows.len(), 33);
    }
}

#[test]
fn every_sample_date_agrees_or_is_refused_or_is_a_known_difference() {
    let columns = table();
    let mut disagreements = BTreeSet::new();
    let (mut same, mut refused, mut round_trips) = (0, 0, 0);
    let mut record = |column: &'static str, ours: &'static str, tally: Tally| {
        same += tally.same;
        refused += tally.refused;
        round_trips += tally.round_trips;
        for rd in tally.differ {
            disagreements.insert((column, ours, rd));
        }
    };
    let registry = hyper_calendar::registry();
    for mapping in mappings(&registry) {
        let rows = columns.get(mapping.column).expect(mapping.column);
        let tally = run_calendar(&mapping, rows);
        record(mapping.column, mapping.ours, tally);
    }
    for function in functions() {
        let rows = columns.get(function.column).expect(function.column);
        let tally = run_function(&function, rows);
        record(function.column, function.ours, tally);
    }
    let known: BTreeSet<_> = KNOWN
        .iter()
        .map(|known| (known.column, known.ours, known.rd))
        .collect();
    assert_eq!(
        disagreements, known,
        "the disagreements are exactly the known ones"
    );
    assert!(KNOWN.iter().all(|known| !known.reason.is_empty()));
    // The counts the documentation states.
    assert_eq!((same, refused, round_trips), (SAME, REFUSED, ROUND_TRIPS));
}

/// Values that agree, dates refused as outside a range, and round trips.
const SAME: usize = 1_275;
const REFUSED: usize = 490;
const ROUND_TRIPS: usize = 1_134;

/// The rising and setting columns
/// [`the_rising_and_setting_are_within_seconds_of_the_books`] compares,
/// each with its bound in seconds of time. `dates.l` fixes the place and
/// `calendar.l` the convention of each, and every value is in the place's
/// standard time:
///
/// * `dawn` — Paris Observatory, UT+1: `dawn` at 18°, the Sun's centre
///   18° below the geometric horizon, with no refraction and no dip;
///   `riseset::dawn` with `Twilight::Astronomical`.
/// * `set` — Jerusalem, 740 m, UT+2: `sunset`, the upper limb on the
///   book's horizon of 34′ + dip + 19″·√h; `riseset::sunset_with` and
///   `horizon::CALENDRICAL_CALCULATIONS`.
/// * `mid-day` — Tehran, UT+3:30: `midday`, local apparent noon;
///   `riseset::solar_noon`.
/// * `moonrise`, `moonset` — Mecca, 298 m, UT+3: the upper limb, 16′,
///   on the same horizon, above the Moon's topocentric altitude;
///   `riseset::moonrise_with` and `moonset_with` and
///   `horizon::CALENDRICAL_CALCULATIONS`.
///
/// The book has no sunrise column.
const RISE_AND_SET: &[(&str, f64)] = &[
    ("dawn", 3.0),
    ("set", 3.0),
    ("mid-day", 3.0),
    ("moonrise", 23.0),
    ("moonset", 23.0),
];

/// Jerusalem as `calendar-code2` places it: 31.78° N, 35.24° E, 740 m
/// (`reingold2018code`, `jerusalem`).
const JERUSALEM: Location = Location::new(31.78, 35.24, 740.0);

/// How far the book's sundial time runs ahead of the Sun's hour angle at a
/// moment, in days.
///
/// The book finds its solar events in sundial time and puts them on the
/// clock through its equation of time (`approx-moment-of-depression`,
/// `local-from-apparent`, `equation-of-time`), which takes the mean Sun's
/// longitude at dynamical time while mean time runs in Universal Time;
/// this library solves for the hour angle, whose mean Sun is the sidereal
/// time of the Universal Time moment. The lead is the first mean Sun less
/// the second: the Sun's mean longitude of Meeus (25.2) at TT, less
/// 0.005 718 3° to make it a right ascension as Meeus's (28.1) does, less
/// the right ascension of the mean Sun whose hour angle is the Universal
/// Time, the IAU 1982 mean sidereal time less that hour angle. The two
/// part by ΔT's worth of the Sun's mean motion and the difference of the
/// two polynomials' squared terms: 38 s in 586 BCE, under a second after
/// 1500 CE. The place does not enter.
fn sundial_lead(moment: Moment) -> f64 {
    use hyper_calendar::hc_astro::{earth, solar, time};
    let at_dynamical_time =
        solar::solar_mean_longitude_at_centuries(time::julian_centuries(moment)) - 0.005_718_3;
    let hour_angle = 360.0 * moment.0.rem_euclid(1.0) - 180.0;
    let of_universal_time = earth::mean_sidereal_time_iau1982(moment) - hour_angle;
    ((at_dynamical_time - of_universal_time) / 360.0 + 0.5).rem_euclid(1.0) - 0.5
}

/// This library's value for a rising and setting column, as the book
/// states it: the moment in the place's standard time, less `rd`; `None`
/// where there is no such event that day.
fn rising_and_setting(column: &str, rd: Rd) -> Option<f64> {
    use hyper_calendar::hc_astro::horizon::CALENDRICAL_CALCULATIONS;
    use hyper_calendar::hc_astro::{Twilight, riseset};
    use hyper_calendar::hc_calendars_equinox::places::{PARIS_OBSERVATORY, TEHRAN_PERSIAN};
    use hyper_calendar::hc_calendars_lunar::islamic_observational::MECCA;
    let solar = |moment: Option<Moment>, zone_hours: f64| {
        moment.map(|moment| moment.0 + zone_hours / 24.0 - sundial_lead(moment) - rd.0 as f64)
    };
    // The book looks for the Moon between two standard-time midnights at
    // Mecca, this library between two local mean ones, 20.7 minutes
    // apart: take the event in the book's day from the days around it.
    let lunar = |find: fn(Rd, Location, &Horizon) -> Option<Moment>| {
        let zone = 3.0 / 24.0;
        (-1..=1)
            .filter_map(|offset| find(rd + offset, MECCA, &CALENDRICAL_CALCULATIONS))
            .map(|moment| moment.0 + zone - rd.0 as f64)
            .find(|standard| (0.0..1.0).contains(standard))
    };
    match column {
        "dawn" => solar(
            riseset::dawn(rd, PARIS_OBSERVATORY, Twilight::Astronomical),
            1.0,
        ),
        "set" => solar(
            riseset::sunset_with(rd, JERUSALEM, &CALENDRICAL_CALCULATIONS),
            2.0,
        ),
        "mid-day" => solar(Some(riseset::solar_noon(rd, TEHRAN_PERSIAN)), 3.5),
        "moonrise" => lunar(riseset::moonrise_with),
        "moonset" => lunar(riseset::moonset_with),
        other => panic!("no rising or setting for {other}"),
    }
}

/// The book's rising and setting against this library's, at every one of
/// the 33 dates, 2094 included: the Sun and its transit within 3 s once
/// the book's sundial lead is taken out (without it, 39 s in 586 BCE), and
/// the Moon within 23 s, because the book halves a twelve-hour bracket
/// ten times, to 42.2 s, and returns its midpoint. A day with no event
/// in the book (`bogus`) has none here either. How each difference was
/// traced is in `docs/systems/rise-and-set.md`.
#[test]
fn the_rising_and_setting_are_within_seconds_of_the_books() {
    let columns = table();
    for (column, bound) in RISE_AND_SET {
        let rows = columns.get(column).expect(column);
        let (mut compared, mut bogus) = (0, 0);
        for (rd, book) in rows {
            let ours = rising_and_setting(column, *rd);
            if book[0] == "bogus" {
                assert_eq!(ours, None, "{column} at R.D. {}", rd.0);
                bogus += 1;
                continue;
            }
            let book: f64 = book[0].parse().expect("a number");
            let ours = ours.unwrap_or_else(|| panic!("{column} at R.D. {}: none here", rd.0));
            let difference = (ours - book).abs() * 86_400.0;
            assert!(
                difference <= *bound,
                "{column} at R.D. {}: {difference:.2} s against a bound of {bound}",
                rd.0
            );
            compared += 1;
        }
        assert_eq!(compared + bogus, 33, "{column}");
    }
}

/// Beijing's standard time, in which the book states its solar terms: the
/// local mean time of 116°25′ E before 1929 and UTC+8 from it
/// (`chinese-location`, `reingold2018code`).
fn beijing_offset(rd: Rd) -> f64 {
    if gregorian_year(rd) < 1929 {
        1397.0 / 180.0 / 24.0
    } else {
        8.0 / 24.0
    }
}

/// This library's value for an astronomical column, in the book's units.
fn astronomy(column: &str, rd: Rd) -> f64 {
    use hyper_calendar::hc_astro::{Moment, delta_t, lunar, riseset, solar};
    use hyper_calendar::hc_calendars_lunar::islamic_observational::MECCA;
    let midnight = Moment(rd.0 as f64);
    let longitude = solar::solar_longitude(midnight);
    let next = |step: f64| (step * (longitude / step).ceil()).rem_euclid(360.0);
    match column {
        "ephem-corr" => delta_t(midnight) / 86_400.0,
        "eqn-of-time" => solar::equation_of_time(midnight) + sundial_lead(midnight),
        "solar-long" => solar::solar_longitude(Moment(midnight.0 + 0.5)),
        "solstice" => solar::solar_longitude_after(next(90.0), midnight).0,
        "lunar-long" => lunar::lunar_longitude(midnight),
        "lunar-lat" => lunar::lunar_latitude(midnight),
        "lunar-alt" => riseset::lunar_altitude(midnight, MECCA),
        "new-moon-after" => lunar::new_moon_at_or_after(midnight).0,
        "major-solar-term-on-or-after" => {
            solar::solar_longitude_after(next(30.0), midnight).0 + beijing_offset(rd)
        }
        other => panic!("no astronomy for {other}"),
    }
}

/// hc-astro is not the book's astronomy — VSOP87 for the Sun, its own
/// truncation of the lunar series, the USNO's ΔT where it has one — so the
/// columns agree to a bound, not exactly. The bounds are the largest
/// differences measured over the 32 dates of 586 BCE to 2038, rounded up:
/// the Sun within 1.6″, the Moon within 6″, and ΔT within 11 s, which is
/// what moves the Moon's longitude and the new moon most in the early
/// dates. 2094 is left out: the book's code computes ΔT for 2051–2150
/// with the sign of the term in 0.5628 reversed, which its errata correct
/// (`reingold2018errata`, correction 14), and is 62 s from this library's
/// Espenak–Meeus polynomial there, and every column moves with it.
#[test]
fn the_astronomy_is_within_seconds_of_the_books() {
    let columns = table();
    let last = gregorian::to_fixed(2050, 12, 31).expect("a Gregorian date");
    for (column, bound) in ASTRONOMY {
        let arc = matches!(
            column,
            &"solar-long" | &"lunar-long" | &"lunar-lat" | &"lunar-alt"
        );
        let scale = if arc { 3_600.0 } else { 86_400.0 };
        let rows = columns.get(column).expect(column);
        let mut compared = 0;
        for (rd, book) in rows.iter().filter(|(rd, _)| *rd <= last) {
            let book: f64 = book[0].parse().expect("a number");
            let mut difference = astronomy(column, *rd) - book;
            if arc {
                difference = (difference + 180.0).rem_euclid(360.0) - 180.0;
            }
            let difference = difference.abs() * scale;
            assert!(
                difference <= *bound,
                "{column} at R.D. {}: {difference:.2} against a bound of {bound}",
                rd.0
            );
            compared += 1;
        }
        assert_eq!(compared, 32, "{column}");
    }
}
