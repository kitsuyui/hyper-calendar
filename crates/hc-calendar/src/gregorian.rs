//! The arithmetic that defines [`Rd`].
//!
//! # Why a calendar lives in the crate that has no calendars
//!
//! [`Rd`] is defined as "day 1 is `0001-01-01` in the proleptic Gregorian
//! calendar". That sentence is not a fact *about* the Gregorian calendar; it
//! is what fixes the origin of the pivot, and without it `Rd(719_163)` names
//! no particular day. So the conversion between a fixed day and a proleptic
//! Gregorian year, month and day belongs here, beside the definition it
//! implements.
//!
//! The practical reason is sharper. Five crates had grown their own private
//! copy of these six functions, each with its own tests — `hc-tz` for POSIX
//! transition rules, `hc-seasons` and `hc-attributes` for month arithmetic,
//! `hc-calendars-lunar` for its epochs, `hc-planetary` for landing dates,
//! `hc-astro` for the start of a solar year.
//! Each was written because the crate could not depend on
//! `hc-calendars-solar` without inverting the layering or waiting for it to
//! exist. Every one of them said in a comment that it should be deleted
//! later. This is that deletion.
//!
//! # What is *not* here
//!
//! Eras, validation ranges, the `Calendar` implementation, the reform
//! calendars and everything else that makes the Gregorian calendar a calendar
//! rather than an origin. Those stay in
//! [`hc-calendars-solar`](https://docs.rs/hc-calendars-solar), which builds
//! on this.
//!
//! Years are astronomical throughout: 1 BC is year 0 and 2 BC is year −1, so
//! the arithmetic never has to step over a year that does not exist.

use crate::error::{CalendarError, CalendarResult};
use crate::fixed::Rd;

/// Days in each month of a common year, January first.
const COMMON_MONTH_LENGTHS: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// Whether `year` is a leap year under the Gregorian rule.
///
/// Every fourth year, except centuries, except every fourth century.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    year.rem_euclid(4) == 0 && (year.rem_euclid(100) != 0 || year.rem_euclid(400) == 0)
}

/// The number of days in `month` of `year`, or `None` when `month` is not in
/// `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    if month == 0 || month > 12 {
        return None;
    }
    if month == 2 && is_leap_year(year) {
        return Some(29);
    }
    Some(COMMON_MONTH_LENGTHS[(month - 1) as usize])
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// The earliest year this implementation converts.
///
/// The bound is what keeps `365 * year` inside `i64`. Without it
/// [`to_fixed`] overflows and panics — a `Result`-returning function that
/// aborts before it can return its error, which policy §8 forbids. The
/// constant used to live only in `hc-calendars-solar`, so its callers were
/// safe and this crate's were not.
pub const MIN_YEAR: i64 = -9_999_999;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 9_999_999;

/// Whether a year is one this module will convert.
#[must_use]
pub const fn year_in_range(year: i64) -> bool {
    year >= MIN_YEAR && year <= MAX_YEAR
}

/// The fixed day of 1 January of `year`.
///
/// Uses the proleptic Gregorian rule in both directions, so it is defined for
/// negative years as well.
///
/// # Panics
///
/// Outside [`MIN_YEAR`]..=[`MAX_YEAR`] the multiplication overflows. Callers
/// that take a year from outside the library should check
/// [`year_in_range`] first, or use [`to_fixed`], which checks.
#[must_use]
pub const fn new_year(year: i64) -> Rd {
    let previous = year - 1;
    Rd(
        365 * previous + previous.div_euclid(4) - previous.div_euclid(100)
            + previous.div_euclid(400)
            + 1,
    )
}

/// The fixed day of a proleptic Gregorian date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`], and [`CalendarError::MonthOutOfRange`] or
/// [`CalendarError::DayOutOfRange`] when the date does not exist.
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if !year_in_range(year) {
        return Err(CalendarError::YearOutOfRange);
    }
    let length = match days_in_month(year, month) {
        Some(length) => length,
        None => return Err(CalendarError::MonthOutOfRange),
    };
    if day == 0 || day > length {
        return Err(CalendarError::DayOutOfRange);
    }
    let mut rd = new_year(year).0;
    let mut index = 1u8;
    while index < month {
        rd += match days_in_month(year, index) {
            Some(value) => value as i64,
            None => return Err(CalendarError::MonthOutOfRange),
        };
        index += 1;
    }
    Ok(Rd(rd + day as i64 - 1))
}

/// The proleptic Gregorian year containing a fixed day.
#[must_use]
pub const fn year_from_fixed(rd: Rd) -> i64 {
    // Divide the day count into 400-, 100-, 4- and 1-year cycles. The
    // 146_097-day Gregorian cycle is exact, so this needs no search.
    let days = rd.0 - 1;
    let cycles_400 = days.div_euclid(146_097);
    let within_400 = days.rem_euclid(146_097);
    let cycles_100 = within_400.div_euclid(36_524);
    let within_100 = within_400.rem_euclid(36_524);
    let cycles_4 = within_100.div_euclid(1_461);
    let within_4 = within_100.rem_euclid(1_461);
    let years = within_4.div_euclid(365);
    let year = 400 * cycles_400 + 100 * cycles_100 + 4 * cycles_4 + years;
    // The last day of a leap year and of a 400-year cycle both land one past
    // the end of their bucket rather than at the start of the next.
    if cycles_100 == 4 || years == 4 {
        year
    } else {
        year + 1
    }
}

/// The proleptic Gregorian year, month and day of a fixed day.
///
/// # Errors
///
/// Returns a [`CalendarError`] only if the day count is so extreme that the
/// intermediate arithmetic cannot be completed; every ordinary day succeeds.
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    let year = year_from_fixed(rd);
    let mut remaining = rd.0 - new_year(year).0;
    let mut month = 1u8;
    while month <= 12 {
        let length = match days_in_month(year, month) {
            Some(value) => value as i64,
            None => return Err(CalendarError::MonthOutOfRange),
        };
        if remaining < length {
            return Ok((year, month, (remaining + 1) as u8));
        }
        remaining -= length;
        month += 1;
    }
    Err(CalendarError::DayOutOfRange)
}

/// The 1-based day of the year.
///
/// # Errors
///
/// Returns a [`CalendarError`] when the date does not exist.
pub const fn day_of_year(year: i64, month: u8, day: u8) -> CalendarResult<u16> {
    match to_fixed(year, month, day) {
        Ok(rd) => Ok((rd.0 - new_year(year).0 + 1) as u16),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::weekday::Weekday;

    /// `to_fixed` returns a `Result`, so it must not abort before returning
    /// one. It used to: the year bound that stops `365 * year` overflowing
    /// lived in `hc-calendars-solar`, so a year past it reached the
    /// multiplication here and panicked.
    #[test]
    fn a_year_past_the_bound_is_an_error_and_not_a_panic() {
        assert_eq!(
            to_fixed(MAX_YEAR + 1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            to_fixed(MIN_YEAR - 1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            to_fixed(3_000_000_000_000_000, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            to_fixed(i64::MAX, 12, 31),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(to_fixed(i64::MIN, 1, 1), Err(CalendarError::YearOutOfRange));
        // And the bounds themselves still convert.
        assert!(to_fixed(MAX_YEAR, 12, 31).is_ok());
        assert!(to_fixed(MIN_YEAR, 1, 1).is_ok());
    }

    #[test]
    fn the_epoch_is_the_first_of_january_of_year_one() {
        assert_eq!(to_fixed(1, 1, 1), Ok(Rd(1)));
        assert_eq!(from_fixed(Rd(1)), Ok((1, 1, 1)));
        assert_eq!(new_year(1), Rd(1));
        // And RD 1 was a Monday, which is how the weekday cycle is anchored.
        assert_eq!(Weekday::from_rd(Rd(1)), Weekday::Monday);
    }

    #[test]
    fn published_reference_days_agree() {
        // 1970-01-01 is RD 719163, JDN 2440588, a Thursday.
        assert_eq!(to_fixed(1970, 1, 1), Ok(Rd(719_163)));
        assert_eq!(Weekday::from_rd(Rd(719_163)), Weekday::Thursday);
        // 2000-01-01 is RD 730120, a Saturday.
        assert_eq!(to_fixed(2000, 1, 1), Ok(Rd(730_120)));
        assert_eq!(Weekday::from_rd(Rd(730_120)), Weekday::Saturday);
        // The Gregorian reform's first day, 1582-10-15.
        assert_eq!(to_fixed(1582, 10, 15), Ok(Rd(577_736)));
    }

    #[test]
    fn the_century_rule_spares_only_multiples_of_four_hundred() {
        assert!(is_leap_year(2000));
        assert!(is_leap_year(1600));
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2100));
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2023));
        // And backwards, where the astronomical numbering matters: year 0 is
        // 1 BC and is a leap year under the proleptic rule.
        assert!(is_leap_year(0));
        assert!(!is_leap_year(-1));
        assert!(is_leap_year(-4));
    }

    #[test]
    fn february_has_twenty_nine_days_only_in_a_leap_year() {
        assert_eq!(days_in_month(2024, 2), Some(29));
        assert_eq!(days_in_month(2023, 2), Some(28));
        assert_eq!(days_in_month(1900, 2), Some(28));
        assert_eq!(days_in_month(2000, 2), Some(29));
        assert_eq!(days_in_month(2024, 0), None);
        assert_eq!(days_in_month(2024, 13), None);
    }

    #[test]
    fn month_lengths_sum_to_the_year_length() {
        for year in [1900, 1999, 2000, 2023, 2024, 0, -1, -400] {
            let total: u16 = (1..=12)
                .map(|month| u16::from(days_in_month(year, month).unwrap()))
                .sum();
            assert_eq!(total, days_in_year(year), "year {year}");
        }
    }

    #[test]
    fn dates_round_trip_over_a_full_leap_cycle() {
        // 146 097 days is one complete Gregorian cycle, so this covers every
        // pattern the rule can produce.
        let start = new_year(1600).0;
        for rd in start..start + 146_097 {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn dates_round_trip_before_the_common_era() {
        for rd in -200_000..-199_000 {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
            assert!(year < 0);
        }
    }

    #[test]
    fn impossible_dates_are_refused() {
        assert_eq!(to_fixed(2023, 2, 29), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(2023, 4, 31), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(2023, 13, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(to_fixed(2023, 0, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(to_fixed(2023, 1, 0), Err(CalendarError::DayOutOfRange));
    }

    #[test]
    fn the_day_of_the_year_counts_from_one() {
        assert_eq!(day_of_year(2024, 1, 1), Ok(1));
        assert_eq!(day_of_year(2024, 12, 31), Ok(366));
        assert_eq!(day_of_year(2023, 12, 31), Ok(365));
        assert_eq!(day_of_year(2024, 3, 1), Ok(61));
        assert_eq!(day_of_year(2023, 3, 1), Ok(60));
    }

    #[test]
    fn the_year_of_a_day_matches_the_year_it_falls_in() {
        for year in [-400i64, -1, 0, 1, 1582, 1900, 2000, 2024, 9999] {
            let first = new_year(year);
            let last = Rd(new_year(year + 1).0 - 1);
            assert_eq!(year_from_fixed(first), year, "first of {year}");
            assert_eq!(year_from_fixed(last), year, "last of {year}");
        }
    }
}
