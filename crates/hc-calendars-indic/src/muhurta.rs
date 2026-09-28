//! The muhūrtas of the day and the night, and the two a pañcāṅga prints
//! by them: *Abhijit*, the auspicious middle of the daylight, and *Dur
//! Muhurtam*, the inauspicious ones of each weekday.
//!
//! `docs/systems/rahu-kalam.md` in the repository describes these beside
//! the kālam, works an example and states the sources; this page states
//! the code's own facts.
//!
//! The daylight, sunrise to sunset, is cut into fifteen equal muhūrtas,
//! and so is the night, sunset to the next sunrise. Drik Panchang states
//! the division and Abhijit's place in it: "Abhijit Muhurat is the 8th
//! Muhurat out of 15 Muhurats which prevail between the sunrise and the
//! sunset", and it "is not suitable on Wednesday as it forms a malefic
//! Muhurta on this weekday" (`drik-abhijit-muhurat`); its day pages print
//! "None" for Abhijit on every Wednesday of January 2025, and the eighth
//! muhūrta as that Wednesday's Dur Muhurtam (`drik-day-panchang-2025`).
//! [`abhijit`] is that.
//!
//! Dur Muhurtam takes one or two muhūrtas by the weekday. No statement of
//! the rule was read; the table here is the muhūrtas Drik Panchang's New
//! Delhi pages of 1 to 7 January 2025 put it in, one page for each
//! weekday, as `crate::kalam` reads its Yamaganda and Gulika columns from
//! the same pages, and the pages of 8 to 31 January are the test:
//!
//! | Weekday | Dur Muhurtam |
//! | --- | --- |
//! | Sunday | the 14th of the day |
//! | Monday | the 9th and the 12th of the day |
//! | Tuesday | the 4th of the day and the 7th of the night |
//! | Wednesday | the 8th of the day |
//! | Thursday | the 6th and the 12th of the day |
//! | Friday | the 4th and the 9th of the day |
//! | Saturday | the 1st and the 2nd of the day |
//!
//! The sunrise and sunset are `hc-astro`'s, as [`crate::kalam::by_sunrise`]
//! reads them; where the Sun does not rise or set there is no daylight to
//! divide, and the functions return [`MissingSolarEvent`].

use hc_astro::riseset::{Location, sunrise, sunset};
use hc_astro::solar_time::MissingSolarEvent;
use hc_calendar::{Rd, Weekday};

use crate::kalam::Span;

/// The muhūrtas in the daylight, and in the night.
pub const MUHURTAS_PER_HALF: u8 = 15;

/// The muhūrta of the daylight that is Abhijit: the middle one of fifteen.
pub const ABHIJIT: u8 = 8;

/// Which half of the day a muhūrta belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Half {
    /// Sunrise to sunset.
    Day,
    /// Sunset to the next sunrise.
    Night,
}

/// A muhūrta of a day: which half, and which of its fifteen, 1 to 15.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Muhurta {
    /// The day or the night after it.
    pub half: Half,
    /// Which fifteenth, counted from the half's start, 1 to 15.
    pub number: u8,
}

impl Muhurta {
    /// The `number`th muhūrta of the daylight.
    #[must_use]
    pub const fn day(number: u8) -> Self {
        Self {
            half: Half::Day,
            number,
        }
    }

    /// The `number`th muhūrta of the night.
    #[must_use]
    pub const fn night(number: u8) -> Self {
        Self {
            half: Half::Night,
            number,
        }
    }
}

/// The names of the two periods, as Drik Panchang's day pages print them in
/// English and in their Hindi edition (`drik-day-panchang-2025`).
pub const ABHIJIT_NAME: &str = "Abhijit";
/// Abhijit in Devanagari, "अभिजित मुहूर्त".
pub const ABHIJIT_NAME_DEVANAGARI: &str = "अभिजित मुहूर्त";
/// Dur Muhurtam, as the English pages print it.
pub const DUR_MUHURTAM_NAME: &str = "Dur Muhurtam";
/// Dur Muhurtam in Devanagari, "दुर्मुहूर्त".
pub const DUR_MUHURTAM_NAME_DEVANAGARI: &str = "दुर्मुहूर्त";

/// Dur Muhurtam's muhūrtas for each weekday, Sunday first: one or two, the
/// second `None` where there is one.
pub const DUR_MUHURTAM: [[Option<Muhurta>; 2]; 7] = [
    [Some(Muhurta::day(14)), None],
    [Some(Muhurta::day(9)), Some(Muhurta::day(12))],
    [Some(Muhurta::day(4)), Some(Muhurta::night(7))],
    [Some(Muhurta::day(8)), None],
    [Some(Muhurta::day(6)), Some(Muhurta::day(12))],
    [Some(Muhurta::day(4)), Some(Muhurta::day(9))],
    [Some(Muhurta::day(1)), Some(Muhurta::day(2))],
];

/// Where a muhūrta of a local day falls, in Universal Time: the daylight or
/// the night after it cut into fifteen. A `number` outside 1 to 15 is
/// clamped.
///
/// # Errors
///
/// [`MissingSolarEvent`] where the Sun does not rise or set on the day, or,
/// for the night, does not rise the next day.
pub fn muhurta(muhurta: Muhurta, day: Rd, location: Location) -> Result<Span, MissingSolarEvent> {
    let rise = sunrise(day, location).ok_or(MissingSolarEvent::Sunrise(day))?;
    let set = sunset(day, location).ok_or(MissingSolarEvent::Sunset(day))?;
    let (start, end) = match muhurta.half {
        Half::Day => (rise.0, set.0),
        Half::Night => {
            let next = Rd(day.0 + 1);
            let next_rise = sunrise(next, location).ok_or(MissingSolarEvent::Sunrise(next))?;
            (set.0, next_rise.0)
        }
    };
    let number = muhurta.number.clamp(1, MUHURTAS_PER_HALF);
    let length = (end - start) / f64::from(MUHURTAS_PER_HALF);
    let first = start + f64::from(number - 1) * length;
    Ok(Span {
        start: hc_calendar::fixed::Moment(first),
        end: hc_calendar::fixed::Moment(first + length),
    })
}

/// Abhijit on a local day: the eighth muhūrta of the daylight, or `None` on
/// a Wednesday, which Drik Panchang gives none.
///
/// # Errors
///
/// [`MissingSolarEvent`] where the Sun does not rise or set on the day.
pub fn abhijit(day: Rd, location: Location) -> Result<Option<Span>, MissingSolarEvent> {
    let span = muhurta(Muhurta::day(ABHIJIT), day, location)?;
    Ok((Weekday::from_rd(day) != Weekday::Wednesday).then_some(span))
}

/// Dur Muhurtam on a local day: the one or two muhūrtas of
/// [`DUR_MUHURTAM`] for its weekday, in order, the second `None` where
/// there is one.
///
/// # Errors
///
/// [`MissingSolarEvent`] where the Sun does not rise or set on the day, or,
/// on a Tuesday, does not rise the next day.
pub fn dur_muhurtam(day: Rd, location: Location) -> Result<[Option<Span>; 2], MissingSolarEvent> {
    let [first, second] = DUR_MUHURTAM[Weekday::from_rd(day).sunday_first_number() as usize];
    let at = |which: Option<Muhurta>| which.map(|which| muhurta(which, day, location)).transpose();
    Ok([at(first)?, at(second)?])
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::fixed::Moment;
    use hc_calendars_solar::gregorian;

    /// New Delhi, as Drik Panchang's pages place it, at sea level, as
    /// `crate::kalam`'s tests take it.
    const NEW_DELHI: Location = Location::new(
        28.0 + 38.0 / 60.0 + 8.0 / 3_600.0,
        77.0 + 13.0 / 60.0 + 28.0 / 3_600.0,
        0.0,
    );

    /// Minutes after midnight IST, on `day`, of a Universal Time moment.
    fn ist_minutes(moment: Moment, day: Rd) -> f64 {
        (moment.0 - day.0 as f64) * 1_440.0 + 330.0
    }

    /// Drik Panchang's New Delhi pages of 1 to 31 January 2025
    /// (`drik-day-panchang-2025`, retrieved 2026-09-29): the day, Abhijit's
    /// start and end as hour and minute IST, or `None` where the page
    /// prints "None", and the first Dur Muhurtam's start and end.
    #[allow(clippy::type_complexity)]
    const DRIK: [(u8, Option<(u8, u8, u8, u8)>, (u8, u8, u8, u8)); 31] = [
        (1, None, (12, 4, 12, 46)),
        (2, Some((12, 5, 12, 46)), (10, 42, 11, 23)),
        (3, Some((12, 5, 12, 47)), (9, 19, 10, 1)),
        (4, Some((12, 5, 12, 47)), (7, 15, 7, 56)),
        (5, Some((12, 6, 12, 47)), (16, 15, 16, 57)),
        (6, Some((12, 6, 12, 48)), (12, 48, 13, 30)),
        (7, Some((12, 7, 12, 48)), (9, 20, 10, 2)),
        (8, None, (12, 7, 12, 49)),
        (9, Some((12, 8, 12, 49)), (10, 44, 11, 26)),
        (10, Some((12, 8, 12, 50)), (9, 21, 10, 2)),
        (11, Some((12, 8, 12, 50)), (7, 15, 7, 57)),
        (12, Some((12, 9, 12, 51)), (16, 20, 17, 2)),
        (13, Some((12, 9, 12, 51)), (12, 51, 13, 33)),
        (14, Some((12, 9, 12, 51)), (9, 21, 10, 3)),
        (15, None, (12, 10, 12, 52)),
        (16, Some((12, 10, 12, 52)), (10, 46, 11, 28)),
        (17, Some((12, 10, 12, 52)), (9, 21, 10, 4)),
        (18, Some((12, 11, 12, 53)), (7, 15, 7, 57)),
        (19, Some((12, 11, 12, 53)), (16, 25, 17, 7)),
        (20, Some((12, 11, 12, 54)), (12, 54, 13, 36)),
        (21, Some((12, 11, 12, 54)), (9, 21, 10, 4)),
        (22, None, (12, 12, 12, 54)),
        (23, Some((12, 12, 12, 54)), (10, 46, 11, 29)),
        (24, Some((12, 12, 12, 55)), (9, 21, 10, 4)),
        (25, Some((12, 12, 12, 55)), (7, 13, 7, 55)),
        (26, Some((12, 12, 12, 55)), (16, 30, 17, 13)),
        (27, Some((12, 13, 12, 56)), (12, 56, 13, 39)),
        (28, Some((12, 13, 12, 56)), (9, 20, 10, 4)),
        (29, None, (12, 13, 12, 56)),
        (30, Some((12, 13, 12, 56)), (10, 46, 11, 30)),
        (31, Some((12, 13, 12, 56)), (9, 20, 10, 3)),
    ];

    /// The second Dur Muhurtam the same pages print, where there is one:
    /// the day and its start and end, hour and minute IST, the Tuesday's
    /// past midnight counted on from the day's own midnight.
    const DRIK_SECOND: [(u8, (u16, u16, u16, u16)); 7] = [
        (2, (14, 50, 15, 32)),
        (3, (12, 47, 13, 28)),
        (4, (7, 56, 8, 38)),
        (6, (14, 53, 15, 34)),
        (7, (23, 6, 24, 0)),
        (13, (14, 57, 15, 39)),
        (14, (23, 9, 24, 3)),
    ];

    fn check(span: Span, day: Rd, (h0, m0, h1, m1): (u16, u16, u16, u16), worst: &mut f64) {
        for (moment, (hour, minute)) in [(span.start, (h0, m0)), (span.end, (h1, m1))] {
            let offset = ist_minutes(moment, day) - f64::from(hour * 60 + minute);
            *worst = worst.max(offset.abs());
        }
    }

    #[test]
    fn the_muhurtas_of_january_2025_are_where_drik_panchang_prints_them() {
        let widen = |(a, b, c, d): (u8, u8, u8, u8)| (a.into(), b.into(), c.into(), d.into());
        let mut worst: f64 = 0.0;
        for (date, printed_abhijit, printed_dur) in DRIK {
            let day = gregorian::to_fixed(2025, 1, date).expect("a date");
            let computed = abhijit(day, NEW_DELHI).expect("Delhi has a day");
            assert_eq!(computed.is_some(), printed_abhijit.is_some(), "{date}");
            if let (Some(span), Some(printed)) = (computed, printed_abhijit) {
                check(span, day, widen(printed), &mut worst);
            }
            let [first, _] = dur_muhurtam(day, NEW_DELHI).expect("Delhi has a day");
            check(
                first.expect("one at least"),
                day,
                widen(printed_dur),
                &mut worst,
            );
        }
        for (date, printed) in DRIK_SECOND {
            let day = gregorian::to_fixed(2025, 1, date).expect("a date");
            let [_, second] = dur_muhurtam(day, NEW_DELHI).expect("Delhi has a day");
            check(second.expect("a second"), day, printed, &mut worst);
        }
        // The pages print whole minutes, and this sunrise and sunset are
        // within a minute of theirs.
        assert!(worst < 1.0, "worst offset {worst} min");
    }

    #[test]
    fn abhijit_straddles_the_middle_of_the_day() {
        // Drik Panchang's example: with sunrise at 6 and sunset at 6,
        // Abhijit starts 24 minutes before midday and ends 24 after.
        let day = gregorian::to_fixed(2025, 3, 20).expect("a date");
        let equator = Location::new(0.0, 0.0, 0.0);
        let span = muhurta(Muhurta::day(ABHIJIT), day, equator).expect("a day");
        let rise = sunrise(day, equator).expect("a sunrise").0;
        let set = sunset(day, equator).expect("a sunset").0;
        let middle = (rise + set) / 2.0;
        assert!(((middle - span.start.0) * 1_440.0 - 24.2).abs() < 0.5);
        assert!(((span.end.0 - middle) * 1_440.0 - 24.2).abs() < 0.5);
        // Fifteen muhūrtas tile the day and fifteen the night.
        let last = muhurta(Muhurta::day(15), day, equator).expect("a day");
        assert!((last.end.0 - set).abs() < 1e-9);
        let dawn = muhurta(Muhurta::night(15), day, equator).expect("a night");
        let next = sunrise(Rd(day.0 + 1), equator).expect("a sunrise").0;
        assert!((dawn.end.0 - next).abs() < 1e-9);
    }

    #[test]
    fn every_weekday_has_a_dur_muhurtam_and_only_tuesdays_is_at_night() {
        for weekday in Weekday::ALL {
            let [first, second] = DUR_MUHURTAM[weekday.sunday_first_number() as usize];
            assert!(first.is_some());
            let night = [first, second]
                .iter()
                .flatten()
                .any(|which| which.half == Half::Night);
            assert_eq!(night, weekday == Weekday::Tuesday, "{weekday:?}");
        }
        // Wednesday's is the eighth, the Abhijit it has none of.
        assert_eq!(
            DUR_MUHURTAM[Weekday::Wednesday.sunday_first_number() as usize][0],
            Some(Muhurta::day(ABHIJIT))
        );
    }

    #[test]
    fn there_is_no_muhurta_where_the_sun_does_not_rise() {
        let tromso = Location::new(69.6496, 18.9560, 0.0);
        let midsummer = gregorian::to_fixed(2024, 6, 21).expect("a date");
        assert_eq!(
            abhijit(midsummer, tromso),
            Err(MissingSolarEvent::Sunrise(midsummer))
        );
    }
}
