//! The arithmetic every Mesoamerican 365-day year in this crate shares:
//! eighteen months of twenty days and a nineteenth period of five, with no
//! intercalation, counted from 1.
//!
//! The Aztec xiuhpōhualli and the Zapotec *yza* differ in their names, their
//! anchors and the day that names the year, and in nothing else; this is the
//! nothing else. The Maya Haabʼ counts its days from 0 and keeps its own.

use hc_calendar::{CalendarError, CalendarResult};

/// Days in the year.
pub(crate) const YEAR_DAYS: i64 = 365;

/// Positions in the year: eighteen months and the five-day remainder.
pub(crate) const MONTHS: u8 = 19;

/// The ordinal, 0 to 364, of day `day` of month `month`, both counted from 1.
///
/// # Errors
///
/// [`CalendarError::MonthOutOfRange`] outside months 1 to 19;
/// [`CalendarError::DayOutOfRange`] outside days 1 to 20, or 1 to 5 in the
/// nineteenth.
pub(crate) const fn ordinal(month: u8, day: u8) -> CalendarResult<i64> {
    if month == 0 || month > MONTHS {
        return Err(CalendarError::MonthOutOfRange);
    }
    let limit = if month == MONTHS { 5 } else { 20 };
    if day == 0 || day > limit {
        return Err(CalendarError::DayOutOfRange);
    }
    Ok((month as i64 - 1) * 20 + day as i64 - 1)
}

/// The month and day, both from 1, `ordinal` days into the year.
pub(crate) const fn position(ordinal: i64) -> (u8, u8) {
    let ordinal = ordinal.rem_euclid(YEAR_DAYS);
    ((ordinal / 20 + 1) as u8, (ordinal % 20 + 1) as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_ordinal_is_one_position_and_back() {
        for ordinal in 0..YEAR_DAYS {
            let (month, day) = position(ordinal);
            assert_eq!(super::ordinal(month, day), Ok(ordinal));
        }
        assert_eq!(position(365), (1, 1));
        assert_eq!(position(-1), (19, 5));
        assert_eq!(super::ordinal(19, 6), Err(CalendarError::DayOutOfRange));
        assert_eq!(super::ordinal(18, 21), Err(CalendarError::DayOutOfRange));
        assert_eq!(super::ordinal(0, 1), Err(CalendarError::MonthOutOfRange));
    }
}
