//! Gregorian helpers in the shapes this crate's rule arithmetic wants.
//!
//! The conversion itself lives in [`hc_calendar::gregorian`], which owns it
//! because it is what defines `Rd`. This module holds no arithmetic of its
//! own: it is five adapters over the shared implementation, kept because
//! POSIX transition rules are stated in bare integers and threading `Rd` and
//! `Result` through them would obscure the rules rather than the arithmetic.

use hc_calendar::Rd;
use hc_calendar::gregorian;
use hc_calendar::weekday::Weekday;

/// Whether `year` is a Gregorian leap year.
pub(crate) const fn is_leap_year(year: i64) -> bool {
    gregorian::is_leap_year(year)
}

/// The number of days in `month` of `year`.
///
/// Saturates at 31 for an out-of-range month rather than returning an option:
/// every caller here has already validated the month against the POSIX
/// grammar, which cannot produce one.
pub(crate) const fn days_in_month(year: i64, month: u8) -> u8 {
    match gregorian::days_in_month(year, month) {
        Some(length) => length,
        None => 31,
    }
}

/// The fixed day of a Gregorian date, as a bare integer.
///
/// Saturates to the year's first day for a date that does not exist, for the
/// same reason as above: the POSIX grammar cannot express one, and a
/// `Result` here would propagate through every transition rule without ever
/// being `Err`.
pub(crate) const fn rd_from_ymd(year: i64, month: u8, day: u8) -> i64 {
    match gregorian::to_fixed(year, month, day) {
        Ok(rd) => rd.0,
        Err(_) => gregorian::new_year(year).0,
    }
}

/// The Gregorian year containing a fixed day.
pub(crate) const fn year_from_rd(rd: i64) -> i64 {
    gregorian::year_from_fixed(Rd(rd))
}

/// The weekday of a fixed day, Sunday = 0 through Saturday = 6.
///
/// POSIX numbers its `Mm.w.d` rules from Sunday, so this is the shape the
/// rule evaluator wants rather than the ISO one.
pub(crate) const fn weekday_from_rd(rd: i64) -> u8 {
    Weekday::from_rd(Rd(rd)).sunday_first_number()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_adapters_agree_with_the_shared_implementation() {
        // The point of this module is to be a thin shape change, so the test
        // is that it changes nothing else.
        for rd in -100_000..100_000 {
            assert_eq!(year_from_rd(rd), gregorian::year_from_fixed(Rd(rd)));
        }
        for year in [-400i64, 0, 1, 1582, 1900, 2000, 2024] {
            assert_eq!(is_leap_year(year), gregorian::is_leap_year(year));
            for month in 1..=12u8 {
                assert_eq!(
                    days_in_month(year, month),
                    gregorian::days_in_month(year, month).unwrap()
                );
                assert_eq!(
                    rd_from_ymd(year, month, 1),
                    gregorian::to_fixed(year, month, 1).unwrap().0
                );
            }
        }
    }

    #[test]
    fn the_posix_epoch_is_rata_die_719163() {
        assert_eq!(rd_from_ymd(1970, 1, 1), 719_163);
        // A Thursday, which POSIX numbers 4 counting from Sunday.
        assert_eq!(weekday_from_rd(719_163), 4);
    }

    #[test]
    fn weekdays_advance_by_one_each_day_and_wrap_at_seven() {
        let start = rd_from_ymd(2026, 1, 1);
        for offset in 0..14 {
            let expected = (weekday_from_rd(start) + offset as u8) % 7;
            assert_eq!(weekday_from_rd(start + offset), expected);
        }
    }

    #[test]
    fn an_impossible_date_saturates_rather_than_panicking() {
        // The POSIX grammar cannot produce one, but the shape has to be
        // total, and a silent wrong answer is worse than a defined one.
        assert_eq!(rd_from_ymd(2023, 2, 30), gregorian::new_year(2023).0);
        assert_eq!(days_in_month(2023, 13), 31);
    }
}
