//! Arithmetic shared by more than one calendar in this crate.
//!
//! Four shapes recur often enough to be worth factoring out:
//!
//! * the "twelve months of thirty days plus a short thirteenth" year of the
//!   Coptic, Ethiopic, Egyptian, Armenian and French Republican calendars;
//! * "Gregorian structure with a different year number", which is all the
//!   Buddhist, Minguo, Juche and Holocene calendars are;
//! * the perennial "thirteen months of 28 days" of the International Fixed,
//!   Tranquility and Pax calendars, with a day or a week outside them;
//! * the Solar Hijri "six months of 31 days, five of 30 and a last of 29 or
//!   30" of the two arithmetic Persian calendars and the Kurdish year over
//!   one of them.
//!
//! Sharing the arithmetic rather than copying it is the point: a bug fixed in
//! the month table is fixed everywhere, and the per-calendar modules are left
//! holding only what actually distinguishes their calendar — the epoch, the
//! leap rule and the names.
//!
//! The irregular Julian/Gregorian month table is the third shape, and it
//! lives in [`hc_calendar::gregorian`] with the rest of the arithmetic that
//! defines `Rd` ([`month_length`](hc_calendar::gregorian::month_length),
//! [`ordinal_day`](hc_calendar::gregorian::ordinal_day),
//! [`month_day`](hc_calendar::gregorian::month_day)).

use hc_calendar::{CalendarError, CalendarResult, Rd};

use crate::gregorian;

/// The Coptic/Ethiopic year shape: twelve thirty-day months and a thirteenth
/// month of five days, six when the year number leaves 3 modulo 4.
pub(crate) const fn coptic_style_is_leap(year: i64) -> bool {
    year.rem_euclid(4) == 3
}

/// Fixed day of a Coptic-shaped date, without validation.
pub(crate) const fn coptic_style_to_fixed(epoch: i64, year: i64, month: u8, day: u8) -> i64 {
    epoch - 1 + 365 * (year - 1) + year.div_euclid(4) + 30 * (month as i64 - 1) + day as i64
}

/// Year, month and day of a Coptic-shaped fixed day.
pub(crate) const fn coptic_style_from_fixed(epoch: i64, fixed: i64) -> (i64, u8, u8) {
    let year = (4 * (fixed - epoch) + 1463).div_euclid(1461);
    let day_of_year = fixed - coptic_style_to_fixed(epoch, year, 1, 1);
    let month = (day_of_year.div_euclid(30) + 1) as u8;
    let day = (fixed - coptic_style_to_fixed(epoch, year, month, 1) + 1) as u8;
    (year, month, day)
}

/// The length of a Coptic-shaped month.
pub(crate) const fn coptic_style_days_in_month(year: i64, month: u8) -> Option<u8> {
    match month {
        1..=12 => Some(30),
        13 => Some(if coptic_style_is_leap(year) { 6 } else { 5 }),
        _ => None,
    }
}

/// The wandering-year shape: twelve thirty-day months and five epagomenal
/// days, never intercalated, so the year is always 365 days long.
pub(crate) const fn wandering_to_fixed(epoch: i64, year: i64, month: u8, day: u8) -> i64 {
    epoch + 365 * (year - 1) + 30 * (month as i64 - 1) + day as i64 - 1
}

/// Year, month and day of a wandering-year fixed day.
pub(crate) const fn wandering_from_fixed(epoch: i64, fixed: i64) -> (i64, u8, u8) {
    let elapsed = fixed - epoch;
    let year = elapsed.div_euclid(365) + 1;
    let day_of_year = elapsed.rem_euclid(365);
    let month = (day_of_year.div_euclid(30) + 1) as u8;
    let day = (day_of_year.rem_euclid(30) + 1) as u8;
    (year, month, day)
}

/// The length of a wandering-year month.
pub(crate) const fn wandering_days_in_month(month: u8) -> Option<u8> {
    match month {
        1..=12 => Some(30),
        13 => Some(5),
        _ => None,
    }
}

/// Validate a day against a month length, naming the field that is wrong.
pub(crate) const fn check_day(day: u8, length: Option<u8>) -> CalendarResult<()> {
    match length {
        None => Err(CalendarError::MonthOutOfRange),
        Some(length) => {
            if day == 0 || day > length {
                Err(CalendarError::DayOutOfRange)
            } else {
                Ok(())
            }
        }
    }
}

/// Fixed day of a date whose structure is Gregorian but whose year number is
/// offset — Buddhist, Minguo, Juche and Holocene all reduce to this.
///
/// # Errors
///
/// Propagates the Gregorian range and field checks.
pub(crate) fn offset_to_fixed(year: i64, month: u8, day: u8, offset: i64) -> CalendarResult<Rd> {
    let gregorian_year = year
        .checked_sub(offset)
        .ok_or(CalendarError::YearOutOfRange)?;
    gregorian::to_fixed(gregorian_year, month, day)
}

/// The offset-year date of a fixed day.
///
/// # Errors
///
/// Propagates the Gregorian range check.
pub(crate) fn offset_from_fixed(rd: Rd, offset: i64) -> CalendarResult<(i64, u8, u8)> {
    let (year, month, day) = gregorian::from_fixed(rd)?;
    let shifted = year
        .checked_add(offset)
        .ok_or(CalendarError::YearOutOfRange)?;
    Ok((shifted, month, day))
}

/// The length of every ordinary month of a perennial 13 × 28 year.
pub(crate) const PERENNIAL_MONTH: i64 = 28;

/// The day number the perennial calendars give a day outside the week,
/// which they attach to the month before it.
pub(crate) const PERENNIAL_ADDED_DAY: u8 = 29;

/// Days in a 13 × 28 year before the first of `month`, where a leap year's
/// added day follows the days of `leap_day_month`.
pub(crate) const fn perennial_days_before_month(month: u8, leap: bool, leap_day_month: u8) -> i64 {
    let elapsed = PERENNIAL_MONTH * (month as i64 - 1);
    if leap && month > leap_day_month {
        elapsed + 1
    } else {
        elapsed
    }
}

/// The month and day of the `elapsed`th day of a 13 × 28 year, counting
/// from 0, with every month 28 days long.
pub(crate) const fn perennial_month_and_day(elapsed: i64) -> (u8, u8) {
    (
        (elapsed / PERENNIAL_MONTH + 1) as u8,
        (elapsed % PERENNIAL_MONTH + 1) as u8,
    )
}

/// The month and day of the `elapsed`th day of a 13 × 28 year that ends in
/// a 365th day outside the week, numbered day 29 of month 13, and in a leap
/// year holds one more, the `at`th day counting from 0, numbered day 29 of
/// `month`: `leap_day` is `Some((at, month))` in a leap year.
pub(crate) const fn perennial_from_elapsed(elapsed: i64, leap_day: Option<(i64, u8)>) -> (u8, u8) {
    let mut elapsed = elapsed;
    if let Some((at, month)) = leap_day {
        if elapsed == at {
            return (month, PERENNIAL_ADDED_DAY);
        }
        if elapsed > at {
            elapsed -= 1;
        }
    }
    if elapsed == 13 * PERENNIAL_MONTH {
        return (13, PERENNIAL_ADDED_DAY);
    }
    perennial_month_and_day(elapsed)
}

/// The Gregorian year in which the year of a calendar that begins on
/// `month`/`day` of the Gregorian calendar, and contains `rd`, began.
///
/// # Errors
///
/// Propagates the Gregorian range check.
pub(crate) const fn gregorian_year_begun_on(rd: Rd, month: u8, day: u8) -> CalendarResult<i64> {
    match gregorian::from_fixed(rd) {
        Err(error) => Err(error),
        Ok((year, m, d)) => {
            if m < month || (m == month && d < day) {
                Ok(year - 1)
            } else {
                Ok(year)
            }
        }
    }
}

/// Days of a Solar Hijri-shaped year before the first of `month`: six
/// months of 31 days, then months of 30.
pub(crate) const fn six_thirty_ones_days_before_month(month: u8) -> i64 {
    if month <= 7 {
        31 * (month as i64 - 1)
    } else {
        30 * (month as i64 - 1) + 6
    }
}

/// The month and day of the `day_of_year`th day, counting from 1, of a
/// Solar Hijri-shaped year: the month falls out of one division on each
/// side of day 186, the end of the sixth month.
pub(crate) const fn six_thirty_ones_month_and_day(day_of_year: i64) -> (u8, u8) {
    let ordinal = if day_of_year <= 186 {
        (day_of_year + 30) / 31
    } else {
        (day_of_year + 23) / 30
    };
    let month = ordinal as u8;
    (
        month,
        (day_of_year - six_thirty_ones_days_before_month(month)) as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_perennial_shape_places_its_added_days() {
        // A common year: 364 days in the months and Year Day after them.
        assert_eq!(perennial_from_elapsed(0, None), (1, 1));
        assert_eq!(perennial_from_elapsed(27, None), (1, 28));
        assert_eq!(perennial_from_elapsed(28, None), (2, 1));
        assert_eq!(perennial_from_elapsed(363, None), (13, 28));
        assert_eq!(perennial_from_elapsed(364, None), (13, 29));
        // A leap day after the 28th of month 6 (the International Fixed
        // Leap Day) shifts everything after it by one.
        let after_june = Some((6 * PERENNIAL_MONTH, 6));
        assert_eq!(perennial_from_elapsed(167, after_june), (6, 28));
        assert_eq!(perennial_from_elapsed(168, after_june), (6, 29));
        assert_eq!(perennial_from_elapsed(169, after_june), (7, 1));
        assert_eq!(perennial_from_elapsed(365, after_june), (13, 29));
        // A leap day between the 27th and 28th of month 8 (Aldrin Day).
        let in_month_8 = Some((7 * PERENNIAL_MONTH + 27, 8));
        assert_eq!(perennial_from_elapsed(222, in_month_8), (8, 27));
        assert_eq!(perennial_from_elapsed(223, in_month_8), (8, 29));
        assert_eq!(perennial_from_elapsed(224, in_month_8), (8, 28));
        assert_eq!(perennial_days_before_month(7, false, 6), 168);
        assert_eq!(perennial_days_before_month(7, true, 6), 169);
        assert_eq!(perennial_days_before_month(6, true, 6), 140);
    }

    #[test]
    fn the_year_begun_on_a_gregorian_day_is_found_either_side_of_it() {
        let rd = |y, m, d| gregorian::to_fixed(y, m, d).unwrap();
        assert_eq!(gregorian_year_begun_on(rd(2026, 7, 21), 7, 21), Ok(2026));
        assert_eq!(gregorian_year_begun_on(rd(2026, 7, 20), 7, 21), Ok(2025));
        assert_eq!(gregorian_year_begun_on(rd(2026, 1, 1), 11, 2), Ok(2025));
        assert_eq!(gregorian_year_begun_on(rd(2026, 11, 2), 11, 2), Ok(2026));
    }

    #[test]
    fn the_six_thirty_ones_shape_turns_at_day_186() {
        assert_eq!(six_thirty_ones_month_and_day(1), (1, 1));
        assert_eq!(six_thirty_ones_month_and_day(31), (1, 31));
        assert_eq!(six_thirty_ones_month_and_day(32), (2, 1));
        assert_eq!(six_thirty_ones_month_and_day(186), (6, 31));
        assert_eq!(six_thirty_ones_month_and_day(187), (7, 1));
        assert_eq!(six_thirty_ones_month_and_day(216), (7, 30));
        assert_eq!(six_thirty_ones_month_and_day(217), (8, 1));
        assert_eq!(six_thirty_ones_month_and_day(365), (12, 29));
        assert_eq!(six_thirty_ones_month_and_day(366), (12, 30));
        for month in 1..=12u8 {
            let first = six_thirty_ones_days_before_month(month) + 1;
            assert_eq!(six_thirty_ones_month_and_day(first), (month, 1));
        }
    }

    #[test]
    fn the_wandering_year_never_grows() {
        for year in 1..50i64 {
            let start = wandering_to_fixed(0, year, 1, 1);
            let next = wandering_to_fixed(0, year + 1, 1, 1);
            assert_eq!(next - start, 365);
        }
    }

    #[test]
    fn the_coptic_shape_intercalates_every_fourth_year() {
        for year in 1..40i64 {
            let start = coptic_style_to_fixed(1, year, 1, 1);
            let next = coptic_style_to_fixed(1, year + 1, 1, 1);
            let expected = if coptic_style_is_leap(year) { 366 } else { 365 };
            assert_eq!(next - start, expected, "year {year}");
        }
    }

    #[test]
    fn day_checks_name_the_field_that_is_wrong() {
        assert_eq!(check_day(1, None), Err(CalendarError::MonthOutOfRange));
        assert_eq!(check_day(0, Some(30)), Err(CalendarError::DayOutOfRange));
        assert_eq!(check_day(31, Some(30)), Err(CalendarError::DayOutOfRange));
        assert_eq!(check_day(30, Some(30)), Ok(()));
    }
}
