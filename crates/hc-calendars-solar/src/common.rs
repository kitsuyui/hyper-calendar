//! Arithmetic shared by more than one calendar in this crate.
//!
//! Two shapes recur often enough to be worth factoring out:
//!
//! * the "twelve months of thirty days plus a short thirteenth" year of the
//!   Coptic, Ethiopic, Egyptian, Armenian and French Republican calendars;
//! * "Gregorian structure with a different year number", which is all the
//!   Buddhist, Minguo, Juche and Holocene calendars are.
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

#[cfg(test)]
mod tests {
    use super::*;

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
