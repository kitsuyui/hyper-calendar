//! Gregorian helpers in the shapes this crate uses.
//!
//! The conversion itself lives in [`hc_calendar::gregorian`], which owns it
//! because it is what defines `Rd`. This module was a private copy, written
//! when nothing in the workspace held one; it is now three adapters that
//! change the shape and nothing else, so that the call sites here can pass
//! bare integers instead of threading `Rd` and `Result` through arithmetic
//! that cannot fail.
//!
//! Several of them are reached only from this crate's tests, which check
//! attributions against published Gregorian dates; the tables themselves are
//! keyed by month or by sign.

use hc_calendar::Rd;
use hc_calendar::gregorian;

/// The fixed day of 1 January of `year`.
#[allow(dead_code)]
pub(crate) const fn new_year(year: i64) -> Rd {
    gregorian::new_year(year)
}

/// Whether `year` is a Gregorian leap year.
#[allow(dead_code)]
pub(crate) const fn is_leap_year(year: i64) -> bool {
    gregorian::is_leap_year(year)
}

/// The fixed day of a Gregorian date.
///
/// Saturates to the year's first day for a date that does not exist. Every
/// caller here supplies a constant date from a published table, so an
/// impossible one is a transcription error that the surrounding tests catch,
/// not a runtime condition to propagate.
#[allow(dead_code)]
pub(crate) const fn from_year_month_day(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(rd) => rd,
        Err(_) => gregorian::new_year(year),
    }
}

/// The Gregorian year, month and day of a fixed day.
pub(crate) const fn year_month_day_from_rd(rd: Rd) -> (i64, u8, u8) {
    match gregorian::from_fixed(rd) {
        Ok(parts) => parts,
        // Unreachable for any representable day; `from_fixed` only fails on
        // arithmetic that cannot be completed.
        Err(_) => (0, 1, 1),
    }
}

/// The Gregorian month a fixed day falls in.
#[allow(dead_code)]
pub(crate) const fn month_from_rd(rd: Rd) -> u8 {
    year_month_day_from_rd(rd).1
}

/// The Gregorian year a fixed day falls in.
#[allow(dead_code)]
pub(crate) const fn year_from_rd(rd: Rd) -> i64 {
    gregorian::year_from_fixed(rd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_adapters_change_shape_and_nothing_else() {
        for rd in -50_000..50_000 {
            let day = Rd(rd);
            assert_eq!(
                year_month_day_from_rd(day),
                gregorian::from_fixed(day).unwrap()
            );
        }
        for year in [-400i64, 0, 1, 1582, 1900, 2000, 2024] {
            assert_eq!(new_year(year), gregorian::new_year(year));
            for month in 1..=12u8 {
                assert_eq!(
                    from_year_month_day(year, month, 1),
                    gregorian::to_fixed(year, month, 1).unwrap()
                );
            }
        }
    }

    #[test]
    fn published_reference_days_survive_the_shape_change() {
        assert_eq!(from_year_month_day(1970, 1, 1), Rd(719_163));
        assert_eq!(year_month_day_from_rd(Rd(719_163)), (1970, 1, 1));
        assert_eq!(month_from_rd(Rd(719_163)), 1);
        assert_eq!(year_from_rd(Rd(719_163)), 1970);
    }

    #[test]
    fn an_impossible_constant_saturates_rather_than_panicking() {
        assert_eq!(from_year_month_day(2023, 2, 30), gregorian::new_year(2023));
    }
}
