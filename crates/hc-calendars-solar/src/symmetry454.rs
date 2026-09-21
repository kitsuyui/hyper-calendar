//! The Symmetry454 calendar.
//!
//! Irv Bromberg's proposed reform, and a useful stress test for this crate's
//! abstraction: it has months, but its year is a whole number of weeks, its
//! leap rule is a single modulo over a 293-year cycle, and nothing about it
//! is shaped like the Gregorian calendar except the twelve month names.
//!
//! Each quarter runs 4–5–4 weeks, so the months are 28, 35 and 28 days
//! long: 91 days a quarter, 364 a year. A leap year appends a seventh week
//! to December, making it 35 days and the year 371. Because the year is
//! always a multiple of seven, **every date falls on the same weekday every
//! year** — the 1st of any month is always a Monday.
//!
//! The leap rule is `(52 × year + 146) mod 293 < 52`: 52 long years in every
//! 293, giving a mean year of 365.2423 days, slightly closer to the mean
//! northward-equinox year than the Gregorian 365.2425. The calendar is
//! aligned so that its year 1 begins on Monday 1 January 1 — [`Rd(1)`][Rd]
//! itself — and thereafter each year begins on the Monday nearest to the
//! Gregorian 1 January.
//!
//! # The other variant
//!
//! Bromberg also defines Symmetry010, with months of 30, 31 and 30 days
//! instead of 4, 5 and 4 weeks. It is [`crate::symmetry010`], and it shares
//! everything but the month layout — the 293-year cycle in
//! [`crate::symmetry`] is the actual proposal, and the two arrangements are
//! two ways of spending the same 364 days.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Weekday,
    YearKind,
};

use crate::symmetry;

/// The year bounds, the leap rule and the cycle constants, which are the
/// actual proposal and are shared with [`crate::symmetry010`].
pub use crate::symmetry::{
    CYCLE_DAYS, CYCLE_YEARS, EARLIEST, LATEST, LEAP_YEAR_DAYS, LEAPS_PER_CYCLE, MAX_YEAR, MIN_YEAR,
    ORDINARY_YEAR_DAYS, days_in_year, is_leap_year, new_year,
};

/// Month lengths in days: four, five, four weeks per quarter.
///
/// This is the whole difference from Symmetry010. Because every month is a
/// whole number of weeks, every month begins on a Monday — which is the
/// property the variant exists for, and the one Symmetry010 trades away.
const MONTH_DAYS: [u8; 12] = [28, 35, 28, 28, 35, 28, 28, 35, 28, 28, 35, 28];

/// The number of days in `month` of `year`, or `None` when `month` is not in
/// `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    symmetry::days_in_month(&MONTH_DAYS, year, month)
}

/// The fixed day of a Symmetry454 date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    symmetry::to_fixed(&MONTH_DAYS, year, month, day)
}

/// The Symmetry454 year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EARLIEST`]..=[`LATEST`].
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    symmetry::from_fixed(&MONTH_DAYS, rd)
}

/// A Symmetry454 date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symmetry454Date {
    /// The year, counting from 1.
    pub year: i64,
    /// The month, 1 through 12, keeping the Gregorian month names.
    pub month: u8,
    /// The day of the month, 1 through 28 or 35.
    pub day: u8,
}

impl Symmetry454Date {
    /// A validated date.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    pub const fn new(year: i64, month: u8, day: u8) -> CalendarResult<Self> {
        match to_fixed(year, month, day) {
            Err(error) => Err(error),
            Ok(_) => Ok(Self { year, month, day }),
        }
    }

    /// The weekday of this date, which depends only on the day of the month.
    ///
    /// Every month starts on a Monday, so this needs no year and no epoch.
    #[must_use]
    pub const fn weekday(self) -> Weekday {
        match (self.day - 1) % 7 {
            0 => Weekday::Monday,
            1 => Weekday::Tuesday,
            2 => Weekday::Wednesday,
            3 => Weekday::Thursday,
            4 => Weekday::Friday,
            5 => Weekday::Saturday,
            _ => Weekday::Sunday,
        }
    }
}

/// The Symmetry454 calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Symmetry454Calendar;

impl Calendar for Symmetry454Calendar {
    type Date = Symmetry454Date;

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("symmetry454"),
            english_name: "Symmetry454",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(Symmetry454Date { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.month, date.day)
            .with_extra("day-of-week", i64::from(date.weekday().iso_number()))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        Symmetry454Date::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian;

    #[test]
    fn the_calendar_starts_on_the_same_day_as_the_gregorian_one() {
        assert_eq!(to_fixed(1, 1, 1), Ok(Rd(1)));
        assert_eq!(gregorian::from_fixed(Rd(1)), Ok((1, 1, 1)));
        assert_eq!(Weekday::from_rd(Rd(1)), Weekday::Monday);
    }

    #[test]
    fn every_year_begins_on_a_monday() {
        for year in (1..5_000).step_by(3) {
            let start = new_year(year).unwrap();
            assert_eq!(Weekday::from_rd(start), Weekday::Monday, "year {year}");
        }
    }

    #[test]
    fn the_year_never_strays_more_than_four_days_from_january_first() {
        // The calendar shadows the Gregorian one: if the leap rule or the
        // alignment were wrong, this would drift without bound instead of
        // oscillating. Four days, not three, because the 293-year cycle
        // tracks the mean equinoctial year rather than the Gregorian rule,
        // so the two run slightly out of step within a cycle — and beyond
        // a few thousand years they part company for good, 365.2423 against
        // 365.2425, which is the whole argument for the reform.
        for year in 1..4_000i64 {
            let start = new_year(year).unwrap().0;
            let gregorian_start = gregorian::to_fixed(year, 1, 1).unwrap().0;
            let drift = start - gregorian_start;
            assert!((-4..=4).contains(&drift), "year {year} drifted {drift}");
        }
    }

    #[test]
    fn the_year_2005_opens_where_bromberg_tabulates_it() {
        // The worked example in the calendar's own documentation:
        // Symmetry454 New Year Day of 2005 is Monday 3 January 2005.
        let start = new_year(2_005).unwrap();
        assert_eq!(gregorian::from_fixed(start), Ok((2_005, 1, 3)));
        assert_eq!(Weekday::from_rd(start), Weekday::Monday);
    }

    #[test]
    fn the_year_is_always_a_whole_number_of_weeks() {
        for year in 1..3_000i64 {
            let length = new_year(year + 1).unwrap().0 - new_year(year).unwrap().0;
            assert_eq!(length % 7, 0, "year {year} was {length} days");
            assert_eq!(length, i64::from(days_in_year(year)), "year {year}");
            assert!(length == ORDINARY_YEAR_DAYS || length == LEAP_YEAR_DAYS);
        }
    }

    #[test]
    fn the_cycle_holds_fifty_two_long_years_in_two_hundred_and_ninety_three() {
        let leaps = (1..=CYCLE_YEARS).filter(|year| is_leap_year(*year)).count();
        assert_eq!(leaps as i64, LEAPS_PER_CYCLE);
        // Which gives the mean year the design is built around.
        let cycle_days = CYCLE_YEARS * ORDINARY_YEAR_DAYS + LEAPS_PER_CYCLE * 7;
        let mean = cycle_days as f64 / CYCLE_YEARS as f64;
        assert!((mean - 365.242_32).abs() < 1e-4, "mean year {mean}");
        assert_eq!(
            new_year(1 + CYCLE_YEARS).unwrap().0 - new_year(1).unwrap().0,
            cycle_days
        );
    }

    #[test]
    fn the_quarters_run_four_five_four_weeks() {
        assert_eq!(days_in_month(2_026, 1), Some(28));
        assert_eq!(days_in_month(2_026, 2), Some(35));
        assert_eq!(days_in_month(2_026, 3), Some(28));
        for quarter in 0..4u8 {
            let first = 1 + quarter * 3;
            let total: i64 = (0..3)
                .map(|offset| i64::from(days_in_month(2_026, first + offset).unwrap()))
                .sum();
            let expected = if quarter == 3 && is_leap_year(2_026) {
                98
            } else {
                91
            };
            assert_eq!(total, expected, "quarter {quarter}");
        }
        assert_eq!(days_in_month(2_026, 13), None);
    }

    #[test]
    fn december_takes_the_extra_week_in_a_leap_year() {
        let leap = (1..500i64).find(|year| is_leap_year(*year)).unwrap();
        assert_eq!(days_in_month(leap, 12), Some(35));
        assert_eq!(days_in_month(leap + 1, 12), Some(28));
        assert!(to_fixed(leap, 12, 35).is_ok());
        assert_eq!(
            to_fixed(leap + 1, 12, 35),
            Err(CalendarError::DayOutOfRange)
        );
    }

    #[test]
    fn a_date_always_falls_on_the_same_weekday() {
        // The property the whole design exists for: the weekday depends on
        // the day of the month and nothing else.
        for year in (1..2_000).step_by(7) {
            for month in [1u8, 5, 12] {
                for day in [1u8, 8, 15, 28] {
                    let rd = to_fixed(year, month, day).unwrap();
                    let date = Symmetry454Date::new(year, month, day).unwrap();
                    assert_eq!(Weekday::from_rd(rd), date.weekday(), "{year}-{month}-{day}");
                }
            }
        }
        assert_eq!(
            Symmetry454Date::new(2_026, 1, 1).unwrap().weekday(),
            Weekday::Monday
        );
        assert_eq!(
            Symmetry454Date::new(2_026, 1, 7).unwrap().weekday(),
            Weekday::Sunday
        );
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (EARLIEST.0..=1_500_000).step_by(127) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn every_single_day_of_a_cycle_round_trips() {
        let start = new_year(1).unwrap().0;
        let end = new_year(1 + CYCLE_YEARS).unwrap().0;
        for rd in start..end {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = Symmetry454Calendar;
        for rd in (EARLIEST.0..=900_000).step_by(383) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(
                fields.extra.get("day-of-week"),
                Some(i64::from(date.weekday().iso_number()))
            );
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId("symmetry454"));
    }

    #[test]
    fn dates_outside_the_supported_range_are_refused() {
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(to_fixed(2_026, 1, 29), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(2_026, 0, 1), Err(CalendarError::MonthOutOfRange));
    }
}
