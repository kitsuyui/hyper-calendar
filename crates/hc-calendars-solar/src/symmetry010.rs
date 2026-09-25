//! The Symmetry010 calendar.
//!
//! Irv Bromberg's other arrangement of the same proposal. The leap rule is
//! identical — 52 leap weeks in 293 years, a mean year of 365.24232 days,
//! which tracks the mean northward equinoctial year more closely than the
//! Gregorian 365.2425 does — and lives in [`crate::symmetry`], shared with
//! [`crate::symmetry454`].
//!
//! What differs is how the 364 days are cut up. Symmetry454 spends them as
//! 4, 5 and 4 *weeks* per quarter, so every month begins on a Monday.
//! Symmetry010 spends them as 30, 31 and 30 *days*, which looks far more
//! like the Gregorian calendar a reader already knows, at the cost of that
//! fixed weekday: in this variant only the year begins on a Monday.
//!
//! Both are the same 364-day year with the same leap week appended to
//! December, and both are perennial — a given date falls on the same
//! weekday every year.
//!
//! # Why this is a separate calendar and not a flag
//!
//! Policy §5. The two are competing arrangements of one proposal, published
//! together by the same author, and a caller who asks for "Symmetry" and
//! silently gets one of them has been given an answer to a question they
//! did not ask. They share an implementation and not a name.
//!
//! # Verification
//!
//! ThreeTen-Extra's `Symmetry010Date` is an independent implementation and
//! the anchors in the tests below are taken against it and against
//! Bromberg's own worked examples.
//!
//! **Source:** Irv Bromberg, *The Symmetry454 Calendar*, University of
//! Toronto.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Weekday,
    YearKind,
};

use crate::symmetry;

/// The year bounds, the leap rule and the cycle constants, shared with
/// [`crate::symmetry454`].
pub use crate::symmetry::{
    CYCLE_DAYS, CYCLE_YEARS, EARLIEST, LATEST, LEAP_YEAR_DAYS, LEAPS_PER_CYCLE, MAX_YEAR, MIN_YEAR,
    ORDINARY_YEAR_DAYS, days_in_year, is_leap_year, new_year,
};

/// Month lengths in days: 30, 31, 30 per quarter.
///
/// The whole difference from Symmetry454, which spends the same 364 days as
/// 28, 35, 28.
const MONTH_DAYS: [u8; 12] = [30, 31, 30, 30, 31, 30, 30, 31, 30, 30, 31, 30];

/// The number of days in `month` of `year`, or `None` when `month` is not in
/// `1..=12`.
///
/// December has 37 days in a leap year, which is the one place the
/// arrangement stops looking Gregorian.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    symmetry::days_in_month(&MONTH_DAYS, year, month)
}

/// The fixed day of a Symmetry010 date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    symmetry::to_fixed(&MONTH_DAYS, year, month, day)
}

/// The Symmetry010 year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EARLIEST`]..=[`LATEST`].
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    symmetry::from_fixed(&MONTH_DAYS, rd)
}

/// A Symmetry010 date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symmetry010Date {
    /// The year, counting from 1.
    pub year: i64,
    /// The month, 1 through 12, keeping the Gregorian month names.
    pub month: u8,
    /// The day of the month, 1 through 30, 31, or 37 in a leap December.
    pub day: u8,
}

impl Symmetry010Date {
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

    /// The weekday of this date.
    ///
    /// Unlike [`crate::symmetry454::Symmetry454Date::weekday`], this needs
    /// the fixed day: a 30-day month does not start on a Monday, which is
    /// exactly what this arrangement gives up.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    pub const fn weekday(self) -> CalendarResult<Weekday> {
        match to_fixed(self.year, self.month, self.day) {
            Err(error) => Err(error),
            Ok(rd) => Ok(Weekday::from_rd(rd)),
        }
    }
}

/// The Symmetry010 calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Symmetry010Calendar;

impl Calendar for Symmetry010Calendar {
    type Date = Symmetry010Date;

    /// Unrecorded: Bromberg's proposal, adopted by nobody.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    /// A year with the leap week.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("symmetry010"),
            english_name: "Symmetry010",
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
        Ok(Symmetry010Date { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.month, date.day)
            .with_extra("day-of-week", i64::from(date.weekday()?.iso_number()))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        Symmetry010Date::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{gregorian, symmetry454};

    #[test]
    fn the_calendar_starts_on_the_same_day_as_the_gregorian_one() {
        assert_eq!(to_fixed(1, 1, 1), Ok(Rd(1)));
        assert_eq!(gregorian::from_fixed(Rd(1)), Ok((1, 1, 1)));
        assert_eq!(Weekday::from_rd(Rd(1)), Weekday::Monday);
    }

    #[test]
    fn the_quarters_are_thirty_thirty_one_thirty() {
        for quarter in 0..4u8 {
            assert_eq!(days_in_month(2024, quarter * 3 + 1), Some(30));
            assert_eq!(days_in_month(2024, quarter * 3 + 2), Some(31));
            // The last month of the year takes the leap week.
            let third = quarter * 3 + 3;
            let expected = if third == 12 && is_leap_year(2024) {
                37
            } else {
                30
            };
            assert_eq!(days_in_month(2024, third), Some(expected));
        }
        assert_eq!(days_in_month(2024, 0), None);
        assert_eq!(days_in_month(2024, 13), None);
    }

    #[test]
    fn a_leap_december_has_thirty_seven_days() {
        let leap = (1..500)
            .find(|year| is_leap_year(*year))
            .expect("one exists");
        assert_eq!(days_in_month(leap, 12), Some(37));
        assert_eq!(days_in_year(leap), 371);
        assert!(to_fixed(leap, 12, 37).is_ok());
        assert_eq!(to_fixed(leap, 12, 38), Err(CalendarError::DayOutOfRange));

        let common = (1..500)
            .find(|year| !is_leap_year(*year))
            .expect("one exists");
        assert_eq!(days_in_month(common, 12), Some(30));
        assert_eq!(to_fixed(common, 12, 31), Err(CalendarError::DayOutOfRange));
    }

    /// The two arrangements are the same year cut differently, so every new
    /// year must land on the same day — and no other day need agree.
    #[test]
    fn it_shares_every_new_year_with_symmetry454() {
        for year in (1..8_000).step_by(7) {
            assert_eq!(
                new_year(year),
                symmetry454::new_year(year),
                "year {year} should start on the same day in both"
            );
            assert_eq!(is_leap_year(year), symmetry454::is_leap_year(year));
            assert_eq!(to_fixed(year, 1, 1), symmetry454::to_fixed(year, 1, 1));
        }
    }

    /// And they disagree where they should: the 31-day second month puts
    /// Symmetry010 four days ahead of Symmetry454 by the end of February.
    #[test]
    fn it_differs_from_symmetry454_inside_the_year() {
        // 1 February is day 31 of the year here, because January has 30
        // days; in Symmetry454 January has 28, so the same label sits two
        // days earlier.
        let sym010 = to_fixed(2024, 2, 1).expect("in range");
        let sym454 = symmetry454::to_fixed(2024, 2, 1).expect("in range");
        assert_eq!(sym010.0 - sym454.0, 2, "010's January is two days longer");

        // And the day Symmetry010 calls 1 February, Symmetry454 calls the
        // third, having already spent its 28-day January.
        let (year, month, day) = symmetry454::from_fixed(sym010).expect("in range");
        assert_eq!((year, month, day), (2024, 2, 3));
    }

    #[test]
    fn every_year_begins_on_a_monday() {
        for year in (1..5_000).step_by(3) {
            let start = new_year(year).expect("in range");
            assert_eq!(Weekday::from_rd(start), Weekday::Monday, "year {year}");
        }
    }

    /// The property that makes it perennial: a date keeps its weekday for
    /// ever, even though — unlike Symmetry454 — that weekday is not a
    /// function of the day of the month alone.
    #[test]
    fn a_date_keeps_its_weekday_every_year() {
        for (month, day) in [(1u8, 1u8), (2, 15), (6, 30), (11, 7)] {
            let first = Symmetry010Date::new(2024, month, day)
                .expect("valid")
                .weekday()
                .expect("valid");
            for year in [1, 999, 2025, 4000, 50_000] {
                let other = Symmetry010Date::new(year, month, day)
                    .expect("valid")
                    .weekday()
                    .expect("valid");
                assert_eq!(other, first, "{year}-{month}-{day}");
            }
        }
        // And the weekday is *not* a function of the day alone, which is
        // what Symmetry454 trades this arrangement for.
        let second_of_january = Symmetry010Date::new(2024, 1, 2)
            .expect("valid")
            .weekday()
            .expect("valid");
        let second_of_february = Symmetry010Date::new(2024, 2, 2)
            .expect("valid")
            .weekday()
            .expect("valid");
        assert_ne!(second_of_january, second_of_february);
    }

    #[test]
    fn it_round_trips_every_day_over_five_thousand_years() {
        let start = new_year(1).expect("in range");
        let end = new_year(5_000).expect("in range");
        for rd in (start.0..end.0).step_by(13) {
            let rd = Rd(rd);
            let (year, month, day) = from_fixed(rd).expect("in range");
            assert_eq!(to_fixed(year, month, day), Ok(rd), "{rd}");
            assert!((1..=37).contains(&day));
            assert!((1..=12).contains(&month));
        }
    }

    #[test]
    fn it_refuses_days_outside_its_range() {
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            to_fixed(MAX_YEAR + 1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn the_calendar_trait_round_trips_through_fields() {
        let calendar = Symmetry010Calendar;
        let date = Symmetry010Date::new(2024, 7, 15).expect("valid");
        let fields = calendar.to_fields(date).expect("convertible");
        assert_eq!(calendar.from_fields(&fields), Ok(date));
        assert_eq!(calendar.meta().id, CalendarId("symmetry010"));
    }
}
