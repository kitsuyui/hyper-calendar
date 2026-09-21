//! The scrap of Gregorian arithmetic this crate needs, kept private.
//!
//! `hc-calendars-solar` owns the real proleptic Gregorian calendar, and this
//! crate deliberately does not depend on it: a seasonal subdivision is
//! astronomy plus day arithmetic, and a dependency the other way round would
//! make the solar calendars unbuildable without the seasons. So the four
//! formulae below are repeated here, from Reingold & Dershowitz,
//! *Calendrical Calculations*, 4th ed., §2.2. A test checks the two of them
//! that `hc_astro::time` also carries against that copy, so the duplication
//! cannot drift.
//!
//! Only the meteorological seasons and the README's worked examples need
//! month and day at all; everything else in this crate counts in `Rd`.

use hc_calendar::Rd;

/// Whether a proleptic Gregorian year has 366 days.
pub(crate) const fn is_leap_year(year: i64) -> bool {
    year.rem_euclid(4) == 0 && year.rem_euclid(100) != 0 || year.rem_euclid(400) == 0
}

/// The fixed day on which a proleptic Gregorian year begins.
pub(crate) const fn new_year(year: i64) -> Rd {
    let y = year - 1;
    Rd(365 * y + y.div_euclid(4) - y.div_euclid(100) + y.div_euclid(400) + 1)
}

/// The proleptic Gregorian year containing a fixed day.
pub(crate) const fn year_from_rd(rd: Rd) -> i64 {
    let d0 = rd.0 - 1;
    let n400 = d0.div_euclid(146_097);
    let d1 = d0.rem_euclid(146_097);
    let n100 = d1.div_euclid(36_524);
    let d2 = d1.rem_euclid(36_524);
    let n4 = d2.div_euclid(1_461);
    let d3 = d2.rem_euclid(1_461);
    let n1 = d3.div_euclid(365);
    let year = 400 * n400 + 100 * n100 + 4 * n4 + n1;
    if n100 == 4 || n1 == 4 { year } else { year + 1 }
}

/// The fixed day of a proleptic Gregorian year, month and day.
///
/// No range checking: this is an internal helper fed only by literals and by
/// [`year_month_day_from_rd`].
pub(crate) const fn from_year_month_day(year: i64, month: u8, day: u8) -> Rd {
    let correction = if month <= 2 {
        0
    } else if is_leap_year(year) {
        -1
    } else {
        -2
    };
    Rd(new_year(year).0 - 1 + (367 * month as i64 - 362).div_euclid(12) + correction + day as i64)
}

/// The proleptic Gregorian year, month and day of a fixed day.
pub(crate) const fn year_month_day_from_rd(rd: Rd) -> (i64, u8, u8) {
    let year = year_from_rd(rd);
    let prior_days = rd.0 - new_year(year).0;
    let correction = if rd.0 < from_year_month_day(year, 3, 1).0 {
        0
    } else if is_leap_year(year) {
        1
    } else {
        2
    };
    let month = ((12 * (prior_days + correction) + 373).div_euclid(367)) as u8;
    let day = (rd.0 - from_year_month_day(year, month, 1).0 + 1) as u8;
    (year, month, day)
}

/// The month number, 1 to 12, of a fixed day.
pub(crate) const fn month_from_rd(rd: Rd) -> u8 {
    year_month_day_from_rd(rd).1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_rata_die_epoch_is_the_first_of_january_of_year_one() {
        assert_eq!(from_year_month_day(1, 1, 1), Rd(1));
        assert_eq!(year_month_day_from_rd(Rd(1)), (1, 1, 1));
    }

    #[test]
    fn the_unix_epoch_lands_where_the_standard_says() {
        assert_eq!(from_year_month_day(1970, 1, 1), Rd::UNIX_EPOCH);
        assert_eq!(year_month_day_from_rd(Rd::UNIX_EPOCH), (1970, 1, 1));
    }

    #[test]
    fn the_private_gregorian_arithmetic_agrees_with_the_astronomy_crates_copy() {
        for year in -500..=3000 {
            assert_eq!(new_year(year), hc_astro::time::gregorian_new_year(year));
        }
        for day in 600_000..640_000 {
            let rd = Rd(day);
            assert_eq!(year_from_rd(rd), hc_astro::time::gregorian_year_from_rd(rd));
        }
    }

    #[test]
    fn year_month_day_round_trips_over_four_centuries() {
        for day in 693_596..=839_692 {
            let rd = Rd(day);
            let (year, month, this_day) = year_month_day_from_rd(rd);
            assert!((1..=12).contains(&month), "month {month} at {rd}");
            assert!((1..=31).contains(&this_day), "day {this_day} at {rd}");
            assert_eq!(from_year_month_day(year, month, this_day), rd);
        }
    }

    #[test]
    fn the_gregorian_leap_rule_skips_three_centurial_years_in_four() {
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2100));
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2023));
        // The rule runs backwards through the proleptic era unchanged.
        assert!(is_leap_year(0));
        assert!(!is_leap_year(-100));
        assert!(is_leap_year(-400));
    }

    #[test]
    fn february_has_the_right_length_either_way() {
        assert_eq!(
            from_year_month_day(2024, 3, 1).0 - from_year_month_day(2024, 2, 1).0,
            29
        );
        assert_eq!(
            from_year_month_day(2023, 3, 1).0 - from_year_month_day(2023, 2, 1).0,
            28
        );
    }

    #[test]
    fn the_month_of_a_day_is_the_month_it_falls_in() {
        assert_eq!(month_from_rd(from_year_month_day(2024, 6, 15)), 6);
        assert_eq!(month_from_rd(from_year_month_day(2024, 12, 31)), 12);
        assert_eq!(month_from_rd(from_year_month_day(2024, 1, 1)), 1);
    }
}
