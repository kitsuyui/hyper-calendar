//! Gregorian helpers in the shapes this crate uses.
//!
//! The conversion itself lives in [`hc_calendar::gregorian`], which owns it
//! because it is what defines `Rd`. This module was a private copy, written
//! before anything in the workspace held one. It is now three adapters that
//! change the shape and nothing else: the epochs and validation ranges here
//! are `const` expressions built from published dates, so they need a total
//! function rather than one that returns `Result`.

use hc_calendar::Rd;
use hc_calendar::gregorian;

/// Whether `year` is a Gregorian leap year.
///
/// Used only by this crate's tests, which check epochs and reference dates
/// against published Gregorian equivalents; the calendars themselves work in
/// fixed days.
#[allow(dead_code)]
pub(crate) const fn is_leap_year(year: i64) -> bool {
    gregorian::is_leap_year(year)
}

/// The fixed day of a Gregorian date.
///
/// Saturates to the year's first day for a date that does not exist. Every
/// call site is a constant from a published table, so an impossible date is a
/// transcription error the surrounding tests catch, not a runtime condition.
pub(crate) const fn to_rd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(rd) => rd,
        Err(_) => gregorian::new_year(year),
    }
}

/// The Gregorian year containing a fixed day.
pub(crate) const fn year_from_rd(rd: Rd) -> i64 {
    gregorian::year_from_fixed(rd)
}

/// The Gregorian year, month and day of a fixed day.
///
/// Used only by this crate's tests, for the same reason as
/// [`is_leap_year`].
#[allow(dead_code)]
pub(crate) const fn from_rd(rd: Rd) -> (i64, u8, u8) {
    match gregorian::from_fixed(rd) {
        Ok(parts) => parts,
        Err(_) => (0, 1, 1),
    }
}

#[cfg(test)]
mod adapter_tests {
    use super::*;

    #[test]
    fn the_adapters_change_shape_and_nothing_else() {
        for rd in -80_000..80_000 {
            let day = Rd(rd);
            assert_eq!(from_rd(day), gregorian::from_fixed(day).unwrap());
            assert_eq!(year_from_rd(day), gregorian::year_from_fixed(day));
        }
        for year in [-400i64, 0, 1, 622, 1582, 1873, 2024] {
            assert_eq!(is_leap_year(year), gregorian::is_leap_year(year));
            assert_eq!(to_rd(year, 1, 1), gregorian::new_year(year));
        }
    }

    #[test]
    fn an_impossible_constant_saturates_rather_than_panicking() {
        assert_eq!(to_rd(2023, 2, 30), gregorian::new_year(2023));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_rata_die_epoch_is_the_first_of_january_year_one() {
        assert_eq!(to_rd(1, 1, 1), Rd(1));
        assert_eq!(from_rd(Rd(1)), (1, 1, 1));
    }

    #[test]
    fn the_unix_epoch_lands_where_the_standard_says() {
        assert_eq!(to_rd(1970, 1, 1), Rd::UNIX_EPOCH);
        assert_eq!(from_rd(Rd::UNIX_EPOCH), (1970, 1, 1));
    }

    #[test]
    fn gregorian_dates_round_trip_across_four_centuries() {
        for rd in 693_595..695_000 {
            let (year, month, day) = from_rd(Rd(rd));
            assert_eq!(to_rd(year, month, day), Rd(rd), "RD {rd}");
        }
        // Straddle 1900, which is not a leap year, and 2000, which is.
        for rd in 693_000..694_500 {
            let (year, month, day) = from_rd(Rd(rd));
            assert_eq!(to_rd(year, month, day), Rd(rd), "RD {rd}");
        }
    }

    #[test]
    fn the_century_rule_is_applied() {
        assert!(!is_leap_year(1900));
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2023));
        assert_eq!(to_rd(1900, 3, 1).0 - to_rd(1900, 2, 28).0, 1);
        assert_eq!(to_rd(2000, 3, 1).0 - to_rd(2000, 2, 28).0, 2);
    }

    #[test]
    fn dates_before_the_common_era_use_the_astronomical_year_numbering() {
        // 1 BCE is year 0, and it is a leap year in the proleptic Gregorian
        // reckoning.
        assert!(is_leap_year(0));
        for rd in -5_000..-4_000 {
            let (year, month, day) = from_rd(Rd(rd));
            assert_eq!(to_rd(year, month, day), Rd(rd), "RD {rd}");
        }
    }
}
