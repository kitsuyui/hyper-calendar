//! Rāhu kālam, Yamaganda and Gulika kālam: the three daily periods a
//! pañcāṅga marks as inauspicious, each an eighth of the day.
//!
//! The day is cut into eight equal parts and each period takes one of
//! them, the part set by the weekday. `docs/systems/rahu-kalam.md` in the
//! repository describes the system, works an example and states the
//! sources; this page states the code's own facts.
//!
//! | Weekday | Rāhu kālam | Yamaganda | Gulika kālam |
//! | --- | --- | --- | --- |
//! | Sunday | 8 | 5 | 7 |
//! | Monday | 2 | 4 | 6 |
//! | Tuesday | 7 | 3 | 5 |
//! | Wednesday | 5 | 2 | 4 |
//! | Thursday | 6 | 1 | 3 |
//! | Friday | 4 | 7 | 2 |
//! | Saturday | 3 | 6 | 1 |
//!
//! What "the day" is differs, and the two conventions are two functions
//! (`docs/policy.md` §5):
//!
//! * [`by_sunrise`], `rahu-kalam-sunrise`: the day runs from local sunrise
//!   to local sunset, as Drik Panchang computes it (its "Yamardha" method,
//!   `drik-rahu-kalam`), so the parts are longer in summer and move with
//!   the place. Returned in Universal Time.
//! * [`by_fixed_day`], `rahu-kalam-fixed`: the day is 06:00 to 18:00 of the
//!   local clock, so every part is an hour and a half and the times are the
//!   same everywhere, as Wikipedia states the rule
//!   (`wikipedia-rahukaalam`) and South Indian temple tables print it
//!   (`tirumala-kalam-table`). Returned as local clock readings.
//!
//! The Rāhu kālam column is Wikipedia's; the Yamaganda and Gulika
//! columns are the parts of the day Drik Panchang's New Delhi pages of
//! 1 to 7 January 2025 put them in (`drik-day-panchang-2025`), one page
//! for each weekday, and the parts the temple table's clock times fall in.

use hc_astro::riseset::{Location, sunrise, sunset};
use hc_astro::solar_time::MissingSolarEvent;
use hc_calendar::fixed::Moment;
use hc_calendar::{Rd, Weekday};

/// One of the three periods: its name and the eighth of the day it takes
/// on each weekday.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Kalam {
    /// The identifier.
    pub id: &'static str,
    /// The name, as Drik Panchang prints it in English.
    pub english_name: &'static str,
    /// The part of the day, 1 to 8 counted from the day's start, for each
    /// weekday, Sunday first.
    pub part_by_weekday: [u8; 7],
    /// Where the parts come from.
    pub source: &'static str,
}

impl Kalam {
    /// The part of the day, 1 to 8, the period takes on a weekday.
    #[must_use]
    pub const fn part(&self, weekday: Weekday) -> u8 {
        self.part_by_weekday[weekday.sunday_first_number() as usize]
    }
}

hc_core::catalogue! {
    type: Kalam,
    id: |kalam| kalam.id,
    provenance: |kalam| kalam.source,
    tests: kalam_catalogue_tests,
    associated;

    /// The three periods, in the order a pañcāṅga prints them.
    pub const ALL;
    /// The period with this identifier.
    pub fn by_id;

    entries: {
        /// Rāhu kālam: never the first part of the day.
        pub const RAHU = Self {
            id: "rahu-kalam",
            english_name: "Rahu Kalam",
            part_by_weekday: [8, 2, 7, 5, 6, 4, 3],
            source: "Wikipedia, \"Rahukaalam\", retrieved 2026-09-27 (wikipedia-rahukaalam), \
                     and Drik Panchang's New Delhi pages of 1-7 January 2025 \
                     (drik-day-panchang-2025)",
        };
        /// Yamaganda.
        pub const YAMAGANDA = Self {
            id: "yamaganda",
            english_name: "Yamaganda",
            part_by_weekday: [5, 4, 3, 2, 1, 7, 6],
            source: "Drik Panchang's New Delhi pages of 1-7 January 2025 \
                     (drik-day-panchang-2025), and the fixed-day table of \
                     tirupatitirumalainfo.com (tirumala-kalam-table), retrieved 2026-09-27",
        };
        /// Gulika kālam, which Drik Panchang spells Gulikai Kalam.
        pub const GULIKA = Self {
            id: "gulika-kalam",
            english_name: "Gulikai Kalam",
            part_by_weekday: [7, 6, 5, 4, 3, 2, 1],
            source: "Drik Panchang's New Delhi pages of 1-7 January 2025 \
                     (drik-day-panchang-2025), and the fixed-day table of \
                     tirupatitirumalainfo.com (tirumala-kalam-table), retrieved 2026-09-27",
        };
    }
}

/// Where a period starts and ends.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    /// The first moment of the period.
    pub start: Moment,
    /// The moment it ends.
    pub end: Moment,
}

/// The eighths of the day into which each period falls.
pub const PARTS_OF_THE_DAY: u8 = 8;

/// The part `part`, 1 to 8, of a day from `start` to `end`.
fn eighth(start: f64, end: f64, part: u8) -> Span {
    let length = (end - start) / f64::from(PARTS_OF_THE_DAY);
    let first = start + f64::from(part - 1) * length;
    Span {
        start: Moment(first),
        end: Moment(first + length),
    }
}

/// A period on a local day by the `rahu-kalam-sunrise` convention: the
/// daylight, sunrise to sunset at the place, cut into eight, in Universal
/// Time.
///
/// # Errors
///
/// [`MissingSolarEvent::Sunrise`] or [`MissingSolarEvent::Sunset`] where
/// the Sun does not rise or set that day: there is no daylight to divide,
/// and no stand-in is taken for it.
pub fn by_sunrise(kalam: &Kalam, day: Rd, location: Location) -> Result<Span, MissingSolarEvent> {
    let rise = sunrise(day, location).ok_or(MissingSolarEvent::Sunrise(day))?;
    let set = sunset(day, location).ok_or(MissingSolarEvent::Sunset(day))?;
    Ok(eighth(rise.0, set.0, kalam.part(Weekday::from_rd(day))))
}

/// The start of the `rahu-kalam-fixed` day, 06:00, as a fraction of a day.
pub const FIXED_DAY_START: f64 = 0.25;

/// The end of the `rahu-kalam-fixed` day, 18:00, as a fraction of a day.
pub const FIXED_DAY_END: f64 = 0.75;

/// A period on a day by the `rahu-kalam-fixed` convention: 06:00 to 18:00
/// cut into eight parts of an hour and a half.
///
/// The span is two readings of the local clock, not Universal Time: each
/// moment's `day()` is `day` and its `day_fraction()` the clock time, in
/// whatever zone the caller keeps. Rāhu kālam on a Monday is 07:30 to
/// 09:00 wherever it is read.
#[must_use]
pub fn by_fixed_day(kalam: &Kalam, day: Rd) -> Span {
    let midnight = day.0 as f64;
    eighth(
        midnight + FIXED_DAY_START,
        midnight + FIXED_DAY_END,
        kalam.part(Weekday::from_rd(day)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendars_solar::gregorian;

    /// New Delhi, as Drik Panchang's pages place it: 28°38′08″ N,
    /// 77°13′28″ E. At sea level, as the pages give no height.
    const NEW_DELHI: Location = Location::new(
        28.0 + 38.0 / 60.0 + 8.0 / 3_600.0,
        77.0 + 13.0 / 60.0 + 28.0 / 3_600.0,
        0.0,
    );

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("a date")
    }

    /// Minutes after midnight IST of a Universal Time moment on `day`.
    fn ist_minutes(moment: Moment, day: Rd) -> f64 {
        (moment.0 - day.0 as f64) * 1_440.0 + 330.0
    }

    /// Drik Panchang's New Delhi pages, 1 to 7 January 2025
    /// (`drik-day-panchang-2025`, retrieved 2026-09-27): the day, and the
    /// start of Rāhu kālam, Yamaganda and Gulikai kālam as hour and minute
    /// IST. The ends are each the next part's start and are not repeated.
    const DRIK: [(u8, [(u8, u8); 3]); 7] = [
        (1, [(12, 25), (8, 32), (11, 7)]),  // Wednesday
        (2, [(13, 43), (7, 14), (9, 50)]),  // Thursday
        (3, [(11, 8), (15, 1), (8, 32)]),   // Friday
        (4, [(9, 50), (13, 44), (7, 15)]),  // Saturday
        (5, [(16, 21), (12, 27), (15, 3)]), // Sunday
        (6, [(8, 33), (11, 9), (13, 45)]),  // Monday
        (7, [(15, 4), (9, 51), (12, 28)]),  // Tuesday
    ];

    #[test]
    fn the_periods_of_a_january_week_start_when_drik_panchang_says() {
        let mut worst: f64 = 0.0;
        for (date, starts) in DRIK {
            let day = ymd(2025, 1, date);
            for (kalam, (hour, minute)) in Kalam::ALL.iter().zip(starts) {
                let span = by_sunrise(kalam, day, NEW_DELHI).expect("Delhi has a day");
                let printed = f64::from(hour) * 60.0 + f64::from(minute);
                let offset = ist_minutes(span.start, day) - printed;
                worst = worst.max(offset.abs());
                // Each is an eighth of the daylight, about 78 minutes in
                // January.
                let length = (span.end.0 - span.start.0) * 1_440.0;
                assert!((77.0..79.5).contains(&length), "{date}: {length}");
            }
        }
        // The pages print whole minutes, and this sunrise and sunset are
        // within a minute of theirs.
        assert!(worst < 1.0, "worst offset {worst} min");
    }

    #[test]
    fn a_monday_rahu_kalam_is_seven_thirty_to_nine_on_the_fixed_day() {
        // Wikipedia's example: Monday 07:30 to 09:00.
        let monday = ymd(2025, 1, 6);
        let span = by_fixed_day(&Kalam::RAHU, monday);
        assert_eq!(span.start.day(), monday);
        assert!((span.start.day_fraction() * 24.0 - 7.5).abs() < 1e-9);
        assert!((span.end.day_fraction() * 24.0 - 9.0).abs() < 1e-9);
    }

    /// The fixed-day table of tirupatitirumalainfo.com
    /// (`tirumala-kalam-table`), Sunday first: the start hour of Rāhu
    /// kālam, Yamaganda and Gulika, and Wikipedia's for Rāhu kālam.
    #[test]
    fn the_fixed_day_table_is_the_eighths_of_six_to_six() {
        let starts: [[f64; 3]; 7] = [
            [16.5, 12.0, 15.0],
            [7.5, 10.5, 13.5],
            [15.0, 9.0, 12.0],
            [12.0, 7.5, 10.5],
            [13.5, 6.0, 9.0],
            [10.5, 15.0, 7.5],
            [9.0, 13.5, 6.0],
        ];
        // 5 January 2025 is a Sunday.
        for (offset, row) in starts.iter().enumerate() {
            let day = ymd(2025, 1, 5) + offset as i64;
            for (kalam, start) in Kalam::ALL.iter().zip(row) {
                let span = by_fixed_day(kalam, day);
                assert!(
                    (span.start.day_fraction() * 24.0 - start).abs() < 1e-9,
                    "{} on day {offset}",
                    kalam.id
                );
            }
        }
    }

    #[test]
    fn the_three_never_share_a_part_and_rahu_never_takes_the_first() {
        for weekday in Weekday::ALL {
            let parts =
                [Kalam::RAHU, Kalam::YAMAGANDA, Kalam::GULIKA].map(|kalam| kalam.part(weekday));
            assert!(parts[0] != parts[1] && parts[1] != parts[2] && parts[0] != parts[2]);
            assert_ne!(Kalam::RAHU.part(weekday), 1);
            assert!(parts.iter().all(|part| (1..=8).contains(part)));
        }
    }

    #[test]
    fn there_is_no_kalam_where_the_sun_does_not_rise() {
        let tromso = Location::new(69.6496, 18.9560, 0.0);
        let midsummer = ymd(2024, 6, 21);
        assert_eq!(
            by_sunrise(&Kalam::RAHU, midsummer, tromso),
            Err(MissingSolarEvent::Sunrise(midsummer))
        );
        assert_eq!(Kalam::by_id("yamaganda"), Some(Kalam::YAMAGANDA));
    }
}
