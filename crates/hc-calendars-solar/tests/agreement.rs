//! The canonical arithmetic and this crate's calendar must agree everywhere.
//!
//! `hc-calendar::gregorian` now owns the conversion that defines `Rd`, and
//! this crate builds eras, validation and the `Calendar` implementation on
//! top of it. That only works if the two produce the same answer, so this
//! checks a full Gregorian leap cycle and a stretch before the Common Era
//! rather than trusting that they were written from the same source.

use hc_calendar::gregorian as core_gregorian;
use hc_calendars_solar::gregorian as solar;

#[test]
fn the_two_implementations_agree_over_a_full_leap_cycle() {
    let start = core_gregorian::new_year(1600).0;
    for rd in start..start + 146_097 {
        let day = hc_calendar::Rd(rd);
        assert_eq!(
            core_gregorian::from_fixed(day),
            solar::from_fixed(day),
            "rd {rd}"
        );
    }
}

#[test]
fn the_two_implementations_agree_before_the_common_era() {
    for rd in -300_000..-299_000 {
        let day = hc_calendar::Rd(rd);
        assert_eq!(
            core_gregorian::from_fixed(day),
            solar::from_fixed(day),
            "rd {rd}"
        );
    }
}

#[test]
fn writing_a_date_agrees_in_both_directions() {
    for year in [-999i64, -1, 0, 1, 1582, 1700, 1900, 2000, 2024, 9_999] {
        for month in 1..=12u8 {
            let length = core_gregorian::days_in_month(year, month).unwrap();
            assert_eq!(Some(length), solar::days_in_month(year, month));
            for day in [1u8, length / 2, length] {
                assert_eq!(
                    core_gregorian::to_fixed(year, month, day),
                    solar::to_fixed(year, month, day),
                    "{year}-{month}-{day}"
                );
            }
        }
        assert_eq!(
            core_gregorian::is_leap_year(year),
            solar::is_leap_year(year),
            "leap {year}"
        );
        assert_eq!(
            core_gregorian::days_in_year(year),
            solar::days_in_year(year),
            "length {year}"
        );
    }
}

#[test]
fn they_refuse_the_same_impossible_dates() {
    for (year, month, day) in [
        (2023i64, 2u8, 29u8),
        (2023, 4, 31),
        (2023, 13, 1),
        (2023, 0, 1),
        (2023, 1, 0),
    ] {
        assert_eq!(
            core_gregorian::to_fixed(year, month, day).is_err(),
            solar::to_fixed(year, month, day).is_err(),
            "{year}-{month}-{day}"
        );
    }
}
