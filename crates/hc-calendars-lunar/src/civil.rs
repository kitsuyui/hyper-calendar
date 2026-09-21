//! The scrap of proleptic Gregorian arithmetic this crate needs for itself.
//!
//! Every calendar here is defined against a fixed day, not against the
//! Gregorian calendar, so the library proper never needs this. Three things
//! do: the era boundaries of the East Asian meridian tables, which the
//! sources state as Gregorian dates; the Tenpō cutover of 1872-12-02; and the
//! published reference dates the tests anchor on.
//!
//! `hc-calendars-solar` owns the real Gregorian calendar. This module is
//! deliberately *not* that: it is forty lines of arithmetic with no
//! validation, no era handling and no `Calendar` implementation, kept private
//! so that the two crates stay independent of one another.

use hc_calendar::Rd;

/// Days elapsed before the first of each month in an ordinary year.
const MONTH_OFFSETS: [i64; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

/// Whether `year` is a leap year in the proleptic Gregorian calendar.
pub(crate) const fn is_leap_year(year: i64) -> bool {
    year.rem_euclid(4) == 0 && (year.rem_euclid(100) != 0 || year.rem_euclid(400) == 0)
}

/// The fixed day of a proleptic Gregorian date.
///
/// No validation: `month` must be in `1..=12` and `day` within the month.
pub(crate) const fn to_rd(year: i64, month: u8, day: u8) -> Rd {
    let prior = year - 1;
    let leap_adjust = if month > 2 && is_leap_year(year) {
        1
    } else {
        0
    };
    Rd(365 * prior + prior.div_euclid(4) - prior.div_euclid(100)
        + prior.div_euclid(400)
        + MONTH_OFFSETS[month as usize - 1]
        + leap_adjust
        + day as i64)
}

/// The proleptic Gregorian year containing a fixed day.
///
/// The four nested divisions are the usual 400/100/4/1-year cascade; the
/// final correction catches the last day of a leap year, which the plain
/// division would push into the following year.
pub(crate) const fn year_from_rd(rd: Rd) -> i64 {
    let days = rd.0 - 1;
    let cycles_400 = days.div_euclid(146_097);
    let within_400 = days.rem_euclid(146_097);
    let cycles_100 = within_400.div_euclid(36_524);
    let within_100 = within_400.rem_euclid(36_524);
    let cycles_4 = within_100.div_euclid(1_461);
    let within_4 = within_100.rem_euclid(1_461);
    let years_1 = within_4.div_euclid(365);
    let year = 400 * cycles_400 + 100 * cycles_100 + 4 * cycles_4 + years_1;
    if cycles_100 == 4 || years_1 == 4 {
        year
    } else {
        year + 1
    }
}

/// The proleptic Gregorian year, month and day of a fixed day.
///
/// Only the tests need this direction — the library itself converts *into*
/// fixed days — but it is what makes a failing assertion readable, so it is
/// kept rather than inlined into each test module.
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) const fn from_rd(rd: Rd) -> (i64, u8, u8) {
    let year = year_from_rd(rd);
    let day_of_year = rd.0 - to_rd(year, 1, 1).0;
    let leap = is_leap_year(year);
    // The correction absorbs February's irregularity so that the other
    // eleven months fall out of a single division; the form is the one in
    // Reingold and Dershowitz, *Calendrical Calculations*.
    let correction = if day_of_year < (if leap { 60 } else { 59 }) {
        0
    } else if leap {
        1
    } else {
        2
    };
    let month = ((12 * (day_of_year + correction) + 373) / 367) as u8;
    let leap_adjust = if month > 2 && leap { 1 } else { 0 };
    let day = (day_of_year + 1 - MONTH_OFFSETS[month as usize - 1] - leap_adjust) as u8;
    (year, month, day)
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
