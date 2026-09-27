//! Choghadiya: the day, sunrise to sunset, and the night, sunset to the
//! next sunrise, each cut into eight equal parts, and each part named for
//! one of seven kinds by the weekday.
//!
//! `docs/systems/choghadiya.md` in the repository describes the system,
//! works an example and states the sources; this page states the code's
//! own facts.
//!
//! The seven kinds run in a fixed cycle, [`Choghadiya::CYCLE`]: Udvega,
//! Chara, Labha, Amrita, Kala, Shubha, Roga. The day's first part is the
//! kind [`DAY_FIRST`] gives for the weekday, and each later part is the
//! next kind in the cycle, so the eighth part repeats the first. The
//! night's first part is the kind [`NIGHT_FIRST`] gives, and each later
//! part is five places further round the cycle. A night belongs to the
//! weekday of the sunset that begins it. The two tables and the two steps
//! are the sequences Drik Panchang prints for New Delhi on 1 to 7 January
//! 2025 (`drik-choghadiya-2025`), one page for each weekday.
//!
//! The parts are eighths of the daylight and of the night at a place, as
//! [`crate::kalam::by_sunrise`] divides the daylight, and are returned in
//! Universal Time.

use hc_astro::riseset::{Location, sunrise, sunset};
use hc_astro::solar_time::MissingSolarEvent;
use hc_calendar::{Rd, Weekday};

use crate::kalam::{PARTS_OF_THE_DAY, Span, eighth};

/// One of the seven kinds of choghadiya.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Choghadiya {
    /// The identifier.
    pub id: &'static str,
    /// The name, as Drik Panchang prints it in English.
    pub english_name: &'static str,
    /// Drik Panchang's one-word gloss of the kind: "Gain", "Best", "Loss",
    /// "Good", "Evil", "Bad" or "Neutral".
    pub gloss: &'static str,
    /// Whether the kind is auspicious, neutral or inauspicious.
    pub quality: Quality,
    /// The planet the kind is ruled by, in English.
    pub ruler: &'static str,
    /// Where the name, the gloss, the quality and the ruler come from.
    pub source: &'static str,
}

/// Whether a kind is good for starting work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Quality {
    /// Amrita, Shubha and Labha.
    Auspicious,
    /// Chara.
    Neutral,
    /// Udvega, Kala and Roga.
    Inauspicious,
}

/// The names and glosses from Drik Panchang, the qualities from Wikipedia
/// and AstroMedha, Chara's from AstroMedha and Drik Panchang alone, the
/// rulers from AstroMedha.
const SOURCE: &str = "Drik Panchang, \"Choghadiya\", New Delhi, 1-7 January 2025 \
                      (drik-choghadiya-2025), for the names and glosses; Wikipedia, \
                      \"Choghadiya\" (wikipedia-choghadiya), and AstroMedha, \
                      \"Choghadiya Explained\" (astromedha-choghadiya), for the qualities, \
                      Chara's neutral from AstroMedha and Drik Panchang, where Wikipedia \
                      calls it good; AstroMedha for the rulers; retrieved 2026-09-28";

hc_core::catalogue! {
    type: Choghadiya,
    id: |kind| kind.id,
    provenance: |kind| kind.source,
    tests: choghadiya_catalogue_tests,
    associated;

    /// The seven kinds in the order the cycle runs by day, Udvega first.
    pub const CYCLE;
    /// The kind with this identifier.
    pub fn by_id;

    entries: {
        /// Udvega, ruled by the Sun: inauspicious.
        pub const UDVEGA = Self {
            id: "udvega",
            english_name: "Udvega",
            gloss: "Bad",
            quality: Quality::Inauspicious,
            ruler: "Sun",
            source: SOURCE,
        };
        /// Chara, ruled by Venus: neutral.
        pub const CHARA = Self {
            id: "chara",
            english_name: "Chara",
            gloss: "Neutral",
            quality: Quality::Neutral,
            ruler: "Venus",
            source: SOURCE,
        };
        /// Labha, ruled by Mercury: auspicious.
        pub const LABHA = Self {
            id: "labha",
            english_name: "Labha",
            gloss: "Gain",
            quality: Quality::Auspicious,
            ruler: "Mercury",
            source: SOURCE,
        };
        /// Amrita, ruled by the Moon: auspicious.
        pub const AMRITA = Self {
            id: "amrita",
            english_name: "Amrita",
            gloss: "Best",
            quality: Quality::Auspicious,
            ruler: "Moon",
            source: SOURCE,
        };
        /// Kala, ruled by Saturn: inauspicious.
        pub const KALA = Self {
            id: "kala",
            english_name: "Kala",
            gloss: "Loss",
            quality: Quality::Inauspicious,
            ruler: "Saturn",
            source: SOURCE,
        };
        /// Shubha, ruled by Jupiter: auspicious.
        pub const SHUBHA = Self {
            id: "shubha",
            english_name: "Shubha",
            gloss: "Good",
            quality: Quality::Auspicious,
            ruler: "Jupiter",
            source: SOURCE,
        };
        /// Roga, ruled by Mars: inauspicious.
        pub const ROGA = Self {
            id: "roga",
            english_name: "Roga",
            gloss: "Evil",
            quality: Quality::Inauspicious,
            ruler: "Mars",
            source: SOURCE,
        };
    }
}

/// The kind of the day's first part on each weekday, Sunday first, as a
/// place in [`Choghadiya::CYCLE`]: Udvega on Sunday, Amrita on Monday,
/// Roga on Tuesday, Labha on Wednesday, Shubha on Thursday, Chara on
/// Friday, Kala on Saturday.
pub const DAY_FIRST: [u8; 7] = [0, 3, 6, 2, 5, 1, 4];

/// The kind of the night's first part on each weekday, Sunday first, as a
/// place in [`Choghadiya::CYCLE`]: Shubha on Sunday, Chara on Monday,
/// Kala on Tuesday, Udvega on Wednesday, Amrita on Thursday, Roga on
/// Friday, Labha on Saturday.
pub const NIGHT_FIRST: [u8; 7] = [5, 1, 4, 0, 3, 6, 2];

/// How far round the cycle each part of the day moves from the one before.
pub const DAY_STEP: u8 = 1;

/// How far round the cycle each part of the night moves from the one
/// before: five places, so Udvega is followed by Shubha.
pub const NIGHT_STEP: u8 = 5;

/// The daylight or the night.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Half {
    /// Sunrise to sunset.
    Day,
    /// Sunset to the next sunrise.
    Night,
}

/// The kind of part `part`, 1 to 8, of the day or the night of a weekday.
/// A `part` outside 1 to 8 is clamped.
#[must_use]
pub fn kind_of(weekday: Weekday, half: Half, part: u8) -> Choghadiya {
    let part = part.clamp(1, PARTS_OF_THE_DAY);
    let index = usize::from(weekday.sunday_first_number());
    let (first, step) = match half {
        Half::Day => (DAY_FIRST[index], DAY_STEP),
        Half::Night => (NIGHT_FIRST[index], NIGHT_STEP),
    };
    let cycle = Choghadiya::CYCLE.len();
    let place = (usize::from(first) + usize::from(step) * usize::from(part - 1)) % cycle;
    Choghadiya::CYCLE[place]
}

/// One part of a day or a night: its kind and when it runs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Period {
    /// The kind.
    pub kind: Choghadiya,
    /// When it starts and ends, in Universal Time.
    pub span: Span,
}

/// The eight parts of `start` to `end` on a weekday's day or night.
fn periods(weekday: Weekday, half: Half, start: f64, end: f64) -> [Period; 8] {
    core::array::from_fn(|index| {
        // `index` is below 8.
        let part = index as u8 + 1;
        Period {
            kind: kind_of(weekday, half, part),
            span: eighth(start, end, part),
        }
    })
}

/// The eight parts of the daylight of a local day at a place, sunrise to
/// sunset, in Universal Time.
///
/// # Errors
///
/// [`MissingSolarEvent::Sunrise`] or [`MissingSolarEvent::Sunset`] where
/// the Sun does not rise or set that day.
pub fn day(day: Rd, location: Location) -> Result<[Period; 8], MissingSolarEvent> {
    let rise = sunrise(day, location).ok_or(MissingSolarEvent::Sunrise(day))?;
    let set = sunset(day, location).ok_or(MissingSolarEvent::Sunset(day))?;
    Ok(periods(Weekday::from_rd(day), Half::Day, rise.0, set.0))
}

/// The eight parts of the night that follows a local day at a place, from
/// its sunset to the next day's sunrise, in Universal Time. The night takes
/// the weekday of `day`.
///
/// # Errors
///
/// [`MissingSolarEvent::Sunset`] where the Sun does not set on `day`, or
/// [`MissingSolarEvent::Sunrise`] where it does not rise on the next day.
pub fn night(day: Rd, location: Location) -> Result<[Period; 8], MissingSolarEvent> {
    let set = sunset(day, location).ok_or(MissingSolarEvent::Sunset(day))?;
    let next = day + 1;
    let rise = sunrise(next, location).ok_or(MissingSolarEvent::Sunrise(next))?;
    Ok(periods(Weekday::from_rd(day), Half::Night, set.0, rise.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::fixed::Moment;
    use hc_calendars_solar::gregorian;

    /// New Delhi, as Drik Panchang's pages place it: 28°38′08″ N,
    /// 77°13′28″ E, at sea level.
    const NEW_DELHI: Location = Location::new(
        28.0 + 38.0 / 60.0 + 8.0 / 3_600.0,
        77.0 + 13.0 / 60.0 + 28.0 / 3_600.0,
        0.0,
    );

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("a date")
    }

    /// Minutes after midnight IST, on `day`, of a Universal Time moment.
    fn ist_minutes(moment: Moment, day: Rd) -> f64 {
        (moment.0 - day.0 as f64) * 1_440.0 + 330.0
    }

    /// A day's page: the day of the month, the kinds of the day's and the
    /// night's parts, and their starts.
    type Page = (u8, [&'static str; 8], [&'static str; 8], [u16; 8], [u16; 8]);

    /// Drik Panchang's New Delhi pages, 1 to 7 January 2025
    /// (`drik-choghadiya-2025`, retrieved 2026-09-28): the day of the
    /// month, the eight kinds of the day and of the night, and the start of
    /// each part as hours and minutes IST, 714 for 07:14. A night's start
    /// before noon is on the next day.
    const DRIK: [Page; 7] = [
        (
            1, // Wednesday
            [
                "labha", "amrita", "kala", "shubha", "roga", "udvega", "chara", "labha",
            ],
            [
                "udvega", "shubha", "amrita", "chara", "roga", "kala", "labha", "udvega",
            ],
            [714, 832, 949, 1107, 1225, 1343, 1500, 1618],
            [1736, 1918, 2100, 2243, 25, 207, 350, 532],
        ),
        (
            2, // Thursday
            [
                "shubha", "roga", "udvega", "chara", "labha", "amrita", "kala", "shubha",
            ],
            [
                "amrita", "chara", "roga", "kala", "labha", "udvega", "shubha", "amrita",
            ],
            [714, 832, 950, 1108, 1225, 1343, 1501, 1619],
            [1736, 1919, 2101, 2243, 25, 208, 350, 532],
        ),
        (
            3, // Friday
            [
                "chara", "labha", "amrita", "kala", "shubha", "roga", "udvega", "chara",
            ],
            [
                "roga", "kala", "labha", "udvega", "shubha", "amrita", "chara", "roga",
            ],
            [714, 832, 950, 1108, 1226, 1344, 1501, 1619],
            [1737, 1919, 2101, 2244, 26, 208, 350, 532],
        ),
        (
            4, // Saturday
            [
                "kala", "shubha", "roga", "udvega", "chara", "labha", "amrita", "kala",
            ],
            [
                "labha", "udvega", "shubha", "amrita", "chara", "roga", "kala", "labha",
            ],
            [715, 833, 950, 1108, 1226, 1344, 1502, 1620],
            [1738, 1920, 2102, 2244, 26, 208, 351, 533],
        ),
        (
            5, // Sunday
            [
                "udvega", "chara", "labha", "amrita", "kala", "shubha", "roga", "udvega",
            ],
            [
                "shubha", "amrita", "chara", "roga", "kala", "labha", "udvega", "shubha",
            ],
            [715, 833, 951, 1109, 1227, 1345, 1503, 1621],
            [1739, 1921, 2103, 2245, 27, 209, 351, 533],
        ),
        (
            6, // Monday
            [
                "amrita", "kala", "shubha", "roga", "udvega", "chara", "labha", "amrita",
            ],
            [
                "chara", "roga", "kala", "labha", "udvega", "shubha", "amrita", "chara",
            ],
            [715, 833, 951, 1109, 1227, 1345, 1503, 1621],
            [1739, 1921, 2103, 2245, 27, 209, 351, 533],
        ),
        (
            7, // Tuesday
            [
                "roga", "udvega", "chara", "labha", "amrita", "kala", "shubha", "roga",
            ],
            [
                "kala", "labha", "udvega", "shubha", "amrita", "chara", "roga", "kala",
            ],
            [715, 833, 951, 1109, 1228, 1346, 1504, 1622],
            [1740, 1922, 2104, 2246, 28, 209, 351, 533],
        ),
    ];

    #[test]
    fn a_january_week_takes_the_kinds_and_times_drik_panchang_prints() {
        let mut worst: f64 = 0.0;
        for (date, day_kinds, night_kinds, day_starts, night_starts) in DRIK {
            let civil = ymd(2025, 1, date);
            let by_day = day(civil, NEW_DELHI).expect("Delhi has a day");
            let by_night = night(civil, NEW_DELHI).expect("and a night");
            for (periods, kinds, starts) in [
                (by_day, day_kinds, day_starts),
                (by_night, night_kinds, night_starts),
            ] {
                for ((period, kind), start) in periods.iter().zip(kinds).zip(starts) {
                    assert_eq!(period.kind.id, kind, "1-{date}");
                    let printed = f64::from(start / 100 * 60 + start % 100);
                    // A night's parts after midnight are read on the clock
                    // of the next day.
                    let computed = ist_minutes(period.span.start, civil) % 1_440.0;
                    worst = worst.max((computed - printed).abs());
                }
            }
        }
        // The pages print whole minutes, and the sunrise and sunset here
        // are within a minute of theirs.
        assert!(worst < 1.0, "worst offset {worst} min");
    }

    #[test]
    fn the_eighth_part_repeats_the_first_and_each_half_holds_all_seven() {
        for weekday in Weekday::ALL {
            for half in [Half::Day, Half::Night] {
                assert_eq!(kind_of(weekday, half, 8), kind_of(weekday, half, 1));
                let mut seen: alloc::vec::Vec<&str> = (1..=7)
                    .map(|part| kind_of(weekday, half, part).id)
                    .collect();
                seen.sort_unstable();
                seen.dedup();
                assert_eq!(seen.len(), 7, "{weekday:?} {half:?}");
            }
        }
    }

    #[test]
    fn the_day_takes_the_kinds_in_the_order_of_their_rulers() {
        // Sun, Venus, Mercury, Moon, Saturn, Jupiter, Mars: each ruler the
        // one after the last in this order, which starts again after Mars.
        let rulers: alloc::vec::Vec<&str> = (1..=7)
            .map(|part| kind_of(Weekday::Sunday, Half::Day, part).ruler)
            .collect();
        assert_eq!(
            rulers,
            [
                "Sun", "Venus", "Mercury", "Moon", "Saturn", "Jupiter", "Mars"
            ]
        );
        // The day begins with the weekday's own planet.
        for (weekday, ruler) in [
            (Weekday::Sunday, "Sun"),
            (Weekday::Monday, "Moon"),
            (Weekday::Tuesday, "Mars"),
            (Weekday::Wednesday, "Mercury"),
            (Weekday::Thursday, "Jupiter"),
            (Weekday::Friday, "Venus"),
            (Weekday::Saturday, "Saturn"),
        ] {
            assert_eq!(kind_of(weekday, Half::Day, 1).ruler, ruler, "{weekday:?}");
        }
    }

    #[test]
    fn a_night_is_an_eighth_of_sunset_to_sunrise() {
        // 1 January 2025 at New Delhi: sunset 17:36, sunrise 07:14, 818
        // minutes of night, so a part of 102.25 minutes.
        let periods = night(ymd(2025, 1, 1), NEW_DELHI).expect("a night");
        let length = (periods[0].span.end.0 - periods[0].span.start.0) * 1_440.0;
        assert!((101.5..103.0).contains(&length), "{length}");
        for pair in periods.windows(2) {
            assert!((pair[0].span.end.0 - pair[1].span.start.0).abs() < 1e-9);
        }
    }

    #[test]
    fn there_is_no_choghadiya_where_the_sun_does_not_set() {
        let tromso = Location::new(69.6496, 18.9560, 0.0);
        let midsummer = ymd(2024, 6, 21);
        assert!(day(midsummer, tromso).is_err());
        assert!(night(midsummer, tromso).is_err());
        assert_eq!(Choghadiya::by_id("amrita"), Some(Choghadiya::AMRITA));
        assert_eq!(kind_of(Weekday::Monday, Half::Day, 0), Choghadiya::AMRITA);
    }
}
