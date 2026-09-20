//! The little bit of proleptic Gregorian arithmetic this crate needs.
//!
//! POSIX `TZ` rules are stated in Gregorian terms — "the second Sunday in
//! March", "the 60th day of the year" — so evaluating one means converting
//! between a Gregorian year and a fixed day number. That is `hc-calendars-solar`'s
//! job in the finished workspace; this module is a private stand-in so that
//! `hc-tz` does not have to depend on a calendar crate to answer a question
//! about a time zone. It is deliberately minimal: no era handling, no field
//! validation, no Julian calendar, no public surface.
//!
//! The formulae are the standard ones from Reingold and Dershowitz,
//! *Calendrical Calculations* (4th ed., §2.2), stated over [`hc_calendar::Rd`]
//! fixed day numbers where day 1 is 0001-01-01 proleptic Gregorian.

/// Whether a proleptic Gregorian year is a leap year.
///
/// Years are astronomical: 0 is 1 BC and is a leap year.
pub(crate) const fn is_leap_year(year: i64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Days in a proleptic Gregorian month. Month 0 and months past 12 answer 0.
pub(crate) const fn days_in_month(year: i64, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

/// The fixed day number of a proleptic Gregorian date.
pub(crate) const fn rd_from_ymd(year: i64, month: u8, day: u8) -> i64 {
    let prior = year - 1;
    let mut rd = 365 * prior + prior.div_euclid(4) - prior.div_euclid(100)
        + prior.div_euclid(400)
        + (367 * month as i64 - 362).div_euclid(12)
        + day as i64;
    if month > 2 {
        rd -= if is_leap_year(year) { 1 } else { 2 };
    }
    rd
}

/// The proleptic Gregorian year containing a fixed day number.
pub(crate) const fn year_from_rd(rd: i64) -> i64 {
    let days = rd - 1;
    let cycles_400 = days.div_euclid(146_097);
    let rest_400 = days.rem_euclid(146_097);
    let cycles_100 = rest_400 / 36_524;
    let rest_100 = rest_400 % 36_524;
    let cycles_4 = rest_100 / 1_461;
    let rest_4 = rest_100 % 1_461;
    let years_1 = rest_4 / 365;
    let year = 400 * cycles_400 + 100 * cycles_100 + 4 * cycles_4 + years_1;
    // A count of 4 means the last day of a leap year or of a 400-year cycle,
    // which belongs to the year already counted rather than to the next one.
    if cycles_100 == 4 || years_1 == 4 {
        year
    } else {
        year + 1
    }
}

/// The day of the week of a fixed day, with Sunday as 0.
///
/// `Rd(1)` is 0001-01-01, a Monday, so the residue class lines up with the
/// POSIX `Mm.w.d` convention without a correction term.
pub(crate) const fn weekday_from_rd(rd: i64) -> u8 {
    rd.rem_euclid(7) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_rata_die_epoch_is_the_first_of_january_year_one() {
        assert_eq!(rd_from_ymd(1, 1, 1), 1);
        assert_eq!(year_from_rd(1), 1);
        // Rd(1) is a Monday, which is 1 in the Sunday-zero convention.
        assert_eq!(weekday_from_rd(1), 1);
    }

    #[test]
    fn the_posix_epoch_is_rata_die_719163() {
        // Reingold and Dershowitz tabulate 1970-01-01 as R.D. 719 163.
        assert_eq!(rd_from_ymd(1970, 1, 1), 719_163);
        assert_eq!(year_from_rd(719_163), 1970);
        // 1970-01-01 was a Thursday.
        assert_eq!(weekday_from_rd(719_163), 4);
    }

    #[test]
    fn published_reference_dates_agree() {
        // Calendrical Calculations, Appendix C sample dates.
        assert_eq!(rd_from_ymd(1945, 11, 12), 710_347);
        assert_eq!(rd_from_ymd(2000, 1, 1), 730_120);
        assert_eq!(rd_from_ymd(1, 1, 1) - 1, 0);
    }

    #[test]
    fn the_century_rule_only_spares_multiples_of_four_hundred() {
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2100));
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2023));
        assert!(is_leap_year(0));
        assert!(!is_leap_year(-100));
        assert!(is_leap_year(-400));
    }

    #[test]
    fn february_has_twenty_nine_days_only_in_leap_years() {
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2023, 2), 28);
        assert_eq!(days_in_month(1900, 2), 28);
        assert_eq!(days_in_month(2024, 13), 0);
    }

    #[test]
    fn dates_round_trip_over_four_centuries_of_days() {
        // One full leap cycle either side of the POSIX epoch, day by day.
        for rd in 646_000..=792_100 {
            let year = year_from_rd(rd);
            let january_first = rd_from_ymd(year, 1, 1);
            assert!(january_first <= rd, "{rd}");
            let next_january_first = rd_from_ymd(year + 1, 1, 1);
            assert!(next_january_first > rd, "{rd}");
            let length = next_january_first - january_first;
            assert_eq!(length, if is_leap_year(year) { 366 } else { 365 }, "{year}");
        }
    }

    #[test]
    fn weekdays_advance_by_one_each_day() {
        let mut expected = weekday_from_rd(719_163);
        for rd in 719_163..719_163 + 4_000 {
            assert_eq!(weekday_from_rd(rd), expected, "{rd}");
            expected = (expected + 1) % 7;
        }
    }

    #[test]
    fn month_lengths_sum_to_the_year_length() {
        for year in [1900, 1999, 2000, 2023, 2024] {
            let total: i64 = (1..=12).map(|m| i64::from(days_in_month(year, m))).sum();
            assert_eq!(total, if is_leap_year(year) { 366 } else { 365 });
            for month in 1..=12u8 {
                let first = rd_from_ymd(year, month, 1);
                let last = rd_from_ymd(year, month, days_in_month(year, month));
                assert_eq!(last - first + 1, i64::from(days_in_month(year, month)));
            }
        }
    }
}
