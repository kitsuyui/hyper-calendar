//! The arithmetic shared by the Symmetry calendars.
//!
//! Irv Bromberg's two proposals differ in exactly one thing: how the 364
//! days of an ordinary year are cut into months. Symmetry454 uses weeks —
//! 4, 5, 4 per quarter — so every month begins on a Monday. Symmetry010
//! uses days — 30, 31, 30 — which reads more like the Gregorian calendar
//! and gives up the fixed weekday within a month.
//!
//! Everything else is identical, including the leap rule that is the actual
//! proposal: 52 leap weeks in 293 years, chosen so the mean year is
//! 365.24232 days, closer to the mean northward-equinoctial year than the
//! Gregorian 365.2425. So the rule lives here once and the two layouts are
//! thin layers over it, per policy §2.
//!
//! **Source:** Irv Bromberg, *The Symmetry454 Calendar*, University of
//! Toronto, which specifies both variants.

use hc_calendar::{CalendarError, CalendarResult, Rd};

use crate::common;

/// Days in an ordinary year: 52 weeks.
pub const ORDINARY_YEAR_DAYS: i64 = 364;

/// Days in a leap year: 53 weeks.
pub const LEAP_YEAR_DAYS: i64 = 371;

/// Years in the leap cycle.
pub const CYCLE_YEARS: i64 = 293;

/// Leap years per cycle.
pub const LEAPS_PER_CYCLE: i64 = 52;

/// Days in the whole cycle.
pub const CYCLE_DAYS: i64 = CYCLE_YEARS * ORDINARY_YEAR_DAYS + LEAPS_PER_CYCLE * 7;

/// The earliest year these implementations convert.
pub const MIN_YEAR: i64 = 1;

/// The latest year these implementations convert.
pub const MAX_YEAR: i64 = 99_999;

/// Whether `year` is a leap year, with a fifty-third week in December.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    (LEAPS_PER_CYCLE * year + 146).rem_euclid(CYCLE_YEARS) < LEAPS_PER_CYCLE
}

/// The number of leap years strictly before `year`, counting from year 1.
///
/// The leap test is equivalent to "this year crosses a multiple of 293", so
/// counting the crossings is one division rather than a loop.
const fn leap_years_before(year: i64) -> i64 {
    (LEAPS_PER_CYCLE * (year - 1) + 146).div_euclid(CYCLE_YEARS)
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) {
        LEAP_YEAR_DAYS as u16
    } else {
        ORDINARY_YEAR_DAYS as u16
    }
}

/// The fixed day on which `year` begins, without validation.
#[must_use]
pub const fn new_year_raw(year: i64) -> i64 {
    1 + ORDINARY_YEAR_DAYS * (year - 1) + 7 * leap_years_before(year)
}

/// The fixed day on which `year` begins, always a Monday.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn new_year(year: i64) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    Ok(Rd(new_year_raw(year)))
}

/// The earliest fixed day these implementations convert.
pub const EARLIEST: Rd = Rd(new_year_raw(MIN_YEAR));

/// The latest fixed day these implementations convert.
pub const LATEST: Rd = Rd(new_year_raw(MAX_YEAR + 1) - 1);

/// The days in `month` of `year` under a given ordinary-year layout.
///
/// The leap week is always added to December, in both variants.
#[must_use]
pub const fn days_in_month(layout: &[u8; 12], year: i64, month: u8) -> Option<u8> {
    if month == 0 || month > 12 {
        return None;
    }
    let base = layout[month as usize - 1];
    if month == 12 && is_leap_year(year) {
        Some(base + 7)
    } else {
        Some(base)
    }
}

/// Days elapsed in the year before the first of `month`.
const fn days_before_month(layout: &[u8; 12], month: u8) -> i64 {
    let mut total = 0;
    let mut index = 0;
    while index < month as usize - 1 {
        total += layout[index] as i64;
        index += 1;
    }
    total
}

/// The fixed day of a date under a given layout.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(layout: &[u8; 12], year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    match common::check_day(day, days_in_month(layout, year, month)) {
        Err(error) => Err(error),
        Ok(()) => Ok(Rd(new_year_raw(year)
            + days_before_month(layout, month)
            + day as i64
            - 1)),
    }
}

/// The year, month and day of a fixed day under a given layout.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EARLIEST`]..=[`LATEST`].
pub const fn from_fixed(layout: &[u8; 12], rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd.0 < EARLIEST.0 {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd.0 > LATEST.0 {
        return Err(CalendarError::AfterSupportedRange);
    }
    // Dividing by the exact mean year — as the cycle's own ratio, not a
    // float — lands within one year; the corrections below close the gap.
    let mut year = (rd.0 - 1) * CYCLE_YEARS / CYCLE_DAYS + 1;
    while new_year_raw(year) > rd.0 {
        year -= 1;
    }
    while new_year_raw(year + 1) <= rd.0 {
        year += 1;
    }
    let day_of_year = rd.0 - new_year_raw(year);
    let mut month = 1u8;
    let mut elapsed = 0;
    while month < 12 {
        let length = layout[month as usize - 1] as i64;
        if day_of_year < elapsed + length {
            break;
        }
        elapsed += length;
        month += 1;
    }
    Ok((year, month, (day_of_year - elapsed + 1) as u8))
}
