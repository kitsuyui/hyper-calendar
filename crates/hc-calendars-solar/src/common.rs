//! Arithmetic shared by more than one calendar in this crate.
//!
//! Three shapes recur often enough to be worth factoring out:
//!
//! * the irregular Julian/Gregorian month table, used by the Gregorian,
//!   Julian, reform, Byzantine and Roman calendars;
//! * the "twelve months of thirty days plus a short thirteenth" year of the
//!   Coptic, Ethiopic, Egyptian, Armenian and French Republican calendars;
//! * "Gregorian structure with a different year number", which is all the
//!   Buddhist, Minguo, Juche and Holocene calendars are.
//!
//! Sharing the arithmetic rather than copying it is the point: a bug fixed in
//! the month table is fixed everywhere, and the per-calendar modules are left
//! holding only what actually distinguishes their calendar — the epoch, the
//! leap rule and the names.

use hc_calendar::{CalendarError, CalendarResult, Rd};

use crate::gregorian;

/// Month lengths of an ordinary Julian/Gregorian year.
pub(crate) const MONTH_LENGTHS: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// Days elapsed before the first of each month in an ordinary year.
pub(crate) const MONTH_OFFSETS: [u16; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

/// The length of `month` in a Julian/Gregorian-shaped year.
///
/// Returns `None` for a month index outside `1..=12`.
pub(crate) const fn julian_style_days_in_month(month: u8, leap: bool) -> Option<u8> {
    if month == 0 || month > 12 {
        return None;
    }
    if month == 2 && leap {
        return Some(29);
    }
    Some(MONTH_LENGTHS[month as usize - 1])
}

/// The 1-based day of the year for a Julian/Gregorian-shaped date.
///
/// `month` must already be known to be in `1..=12`.
pub(crate) const fn julian_style_day_of_year(month: u8, day: u8, leap: bool) -> u16 {
    let leap_adjust = if leap && month > 2 { 1 } else { 0 };
    MONTH_OFFSETS[month as usize - 1] + leap_adjust + day as u16
}

/// The month and day of the month for a 1-based day of the year.
///
/// The closed form for the month is Reingold and Dershowitz's, in
/// `gregorian-from-fixed` (`reingold2018code`): the correction term absorbs
/// the February irregularity so that the remaining eleven months fall out
/// of a single division, which keeps `from_fixed` branch-light.
pub(crate) const fn julian_style_month_day(day_of_year: u16, leap: bool) -> (u8, u8) {
    let prior = day_of_year as i64 - 1;
    let prior_before_march = if leap { 60 } else { 59 };
    let correction = if prior < prior_before_march {
        0
    } else if leap {
        1
    } else {
        2
    };
    let month = ((12 * (prior + correction) + 373) / 367) as u8;
    let leap_adjust = if leap && month > 2 { 1 } else { 0 };
    let day = day_of_year - MONTH_OFFSETS[month as usize - 1] - leap_adjust;
    (month, day as u8)
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_month_table_sums_to_a_year() {
        let ordinary: u16 = MONTH_LENGTHS.iter().map(|length| u16::from(*length)).sum();
        assert_eq!(ordinary, 365);
        assert_eq!(julian_style_day_of_year(12, 31, false), 365);
        assert_eq!(julian_style_day_of_year(12, 31, true), 366);
    }

    #[test]
    fn the_month_offsets_agree_with_the_month_lengths() {
        let mut running = 0u16;
        for (index, length) in MONTH_LENGTHS.iter().enumerate() {
            assert_eq!(MONTH_OFFSETS[index], running);
            running += u16::from(*length);
        }
    }

    #[test]
    fn day_of_year_and_month_day_are_inverses() {
        for leap in [false, true] {
            let length = if leap { 366 } else { 365 };
            for day_of_year in 1..=length {
                let (month, day) = julian_style_month_day(day_of_year, leap);
                assert!((1..=12).contains(&month), "month {month} for {day_of_year}");
                assert_eq!(
                    julian_style_day_of_year(month, day, leap),
                    day_of_year,
                    "leap {leap} day {day_of_year} decoded to {month}-{day}"
                );
            }
        }
    }

    #[test]
    fn february_is_the_only_month_that_changes_length() {
        for month in 1..=12u8 {
            let ordinary = julian_style_days_in_month(month, false);
            let leap = julian_style_days_in_month(month, true);
            if month == 2 {
                assert_eq!((ordinary, leap), (Some(28), Some(29)));
            } else {
                assert_eq!(ordinary, leap);
            }
        }
        assert_eq!(julian_style_days_in_month(0, false), None);
        assert_eq!(julian_style_days_in_month(13, false), None);
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
