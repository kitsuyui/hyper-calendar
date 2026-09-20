//! The ergonomic layer, checked against an independent implementation.
//!
//! Python's `datetime` is the reference requirement 2 of the project brief
//! names, and its `date.toordinal()` is the same proleptic Gregorian day
//! number as this library's `Rd`. That makes it a genuinely independent
//! oracle rather than a restatement of our own arithmetic, so the anchors
//! below were produced by running CPython and are quoted as its output.

use hyper_calendar::civil::{Date, DateTime, Time, TimeDelta};

/// `(year, month, day, toordinal(), isoweekday())`, from CPython.
const PYTHON_ANCHORS: &[(i64, u8, u8, i64, u8)] = &[
    (1, 1, 1, 1, 1),
    (1970, 1, 1, 719_163, 4),
    (2000, 1, 1, 730_120, 6),
    (2026, 9, 21, 739_880, 1),
    (9999, 12, 31, 3_652_059, 5),
];

#[test]
fn ordinals_and_weekdays_match_python() {
    for &(year, month, day, ordinal, iso_weekday) in PYTHON_ANCHORS {
        let date = Date::new(year, month, day).unwrap();
        assert_eq!(date.to_ordinal(), ordinal, "{year}-{month}-{day}");
        assert_eq!(date.iso_weekday(), iso_weekday, "{year}-{month}-{day}");
        assert_eq!(Date::from_ordinal(ordinal).unwrap(), date);
    }
}

#[test]
fn the_unix_epoch_constant_agrees_with_the_computed_date() {
    assert_eq!(Date::UNIX_EPOCH, Date::new(1970, 1, 1).unwrap());
    assert_eq!(Date::UNIX_EPOCH.to_ordinal(), 719_163);
}

#[test]
fn dates_round_trip_through_their_ordinal_over_four_centuries() {
    // A full Gregorian leap cycle is 146 097 days, so this covers every
    // pattern the calendar can produce.
    let start = Date::new(1600, 1, 1).unwrap().to_ordinal();
    for offset in 0..146_097 {
        let ordinal = start + offset;
        let date = Date::from_ordinal(ordinal).unwrap();
        assert_eq!(date.to_ordinal(), ordinal);
        assert_eq!(
            Date::new(date.year(), date.month(), date.day()).unwrap(),
            date
        );
    }
}

#[test]
fn impossible_dates_are_rejected() {
    assert!(Date::new(2026, 2, 30).is_err());
    assert!(Date::new(2026, 4, 31).is_err());
    assert!(Date::new(2026, 13, 1).is_err());
    assert!(Date::new(2026, 0, 1).is_err());
    assert!(Date::new(2026, 1, 0).is_err());
    assert!(Date::new(1900, 2, 29).is_err());
    assert!(Date::new(2000, 2, 29).is_ok());
}

#[test]
fn month_arithmetic_clamps_rather_than_failing() {
    // 31 January plus one month has no exact answer; clamping is the
    // documented behaviour and the trap is that it is not reversible.
    let january = Date::new(2026, 1, 31).unwrap();
    let february = january.add_months(1).unwrap();
    assert_eq!(february, Date::new(2026, 2, 28).unwrap());
    assert_ne!(february.add_months(-1).unwrap(), january);

    // The leap-year case.
    let leap = Date::new(2024, 2, 29).unwrap();
    assert_eq!(leap.add_years(1).unwrap(), Date::new(2025, 2, 28).unwrap());
    assert_eq!(leap.add_years(4).unwrap(), Date::new(2028, 2, 29).unwrap());
}

#[test]
fn replace_reports_the_impossible_instead_of_clamping() {
    let january = Date::new(2026, 1, 31).unwrap();
    assert!(january.replace(None, Some(2), None).is_err());
    assert_eq!(
        january.replace(Some(2027), None, None).unwrap(),
        Date::new(2027, 1, 31).unwrap()
    );
}

#[test]
fn month_arithmetic_crosses_year_boundaries_in_both_directions() {
    let date = Date::new(2026, 3, 15).unwrap();
    assert_eq!(
        date.add_months(-3).unwrap(),
        Date::new(2025, 12, 15).unwrap()
    );
    assert_eq!(
        date.add_months(10).unwrap(),
        Date::new(2027, 1, 15).unwrap()
    );
    assert_eq!(
        date.add_months(-15).unwrap(),
        Date::new(2024, 12, 15).unwrap()
    );
}

#[test]
fn day_arithmetic_crosses_a_leap_day() {
    let before = Date::new(2024, 2, 28).unwrap();
    assert_eq!(before.add_days(1).unwrap(), Date::new(2024, 2, 29).unwrap());
    assert_eq!(before.add_days(2).unwrap(), Date::new(2024, 3, 1).unwrap());
    assert_eq!(
        Date::new(2023, 2, 28).unwrap().add_days(1).unwrap(),
        Date::new(2023, 3, 1).unwrap()
    );
}

#[test]
fn day_of_year_and_leap_years_agree() {
    assert_eq!(Date::new(2024, 12, 31).unwrap().day_of_year(), 366);
    assert_eq!(Date::new(2023, 12, 31).unwrap().day_of_year(), 365);
    assert_eq!(Date::new(2024, 1, 1).unwrap().day_of_year(), 1);
    assert!(Date::new(2024, 1, 1).unwrap().is_leap_year());
    assert!(!Date::new(1900, 1, 1).unwrap().is_leap_year());
    assert!(Date::new(2000, 1, 1).unwrap().is_leap_year());
    assert_eq!(Date::new(2024, 2, 1).unwrap().days_in_month(), 29);
    assert_eq!(Date::new(2023, 2, 1).unwrap().days_in_month(), 28);
}

#[test]
fn dates_render_as_iso_8601() {
    assert_eq!(Date::new(2026, 9, 21).unwrap().to_string(), "2026-09-21");
    assert_eq!(Date::new(1, 1, 1).unwrap().to_string(), "0001-01-01");
    // Outside the four-digit range ISO 8601 requires an explicit sign.
    assert_eq!(Date::new(-1, 1, 1).unwrap().to_string(), "-00001-01-01");
    assert_eq!(Date::new(12_345, 6, 7).unwrap().to_string(), "+12345-06-07");
}

#[test]
fn times_keep_the_leap_second() {
    let leap = Time::hms(23, 59, 60).unwrap();
    assert!(leap.is_leap_second());
    assert_eq!(leap.to_string(), "23:59:60");
    // It exists only where UTC puts one.
    assert!(Time::hms(12, 0, 60).is_err());
    assert!(Time::hms(24, 0, 0).is_err());
}

#[test]
fn sub_second_accessors_truncate_consistently() {
    let time = Time::new(1, 2, 3, 123_456_789).unwrap();
    assert_eq!(time.nanosecond(), 123_456_789);
    assert_eq!(time.microsecond(), 123_456);
    assert_eq!(time.hour(), 1);
    assert_eq!(time.minute(), 2);
    assert_eq!(time.second(), 3);
}

#[test]
fn date_times_render_without_a_zone_because_they_have_none() {
    let moment = DateTime::from_parts(2026, 9, 21, 14, 30, 5, 0).unwrap();
    assert_eq!(moment.to_string(), "2026-09-21T14:30:05");
}

#[test]
fn nominal_spans_count_every_day_as_86400_seconds() {
    let start = DateTime::midnight(Date::new(2026, 1, 1).unwrap());
    let end = DateTime::new(Date::new(2026, 1, 3).unwrap(), Time::NOON);
    let span = end.nominal_duration_since(start).unwrap();
    assert_eq!(span.total_seconds(), 2.0 * 86_400.0 + 43_200.0);
    assert_eq!(span.whole_days(), 2);
}

#[test]
fn time_deltas_compose_like_python_timedeltas() {
    let span = TimeDelta::new(1, 2, 3, 4);
    assert_eq!(span.total_seconds(), 86_400.0 + 7_200.0 + 180.0 + 4.0);
    let doubled = span.checked_add(span).unwrap();
    assert_eq!(doubled.total_seconds(), span.total_seconds() * 2.0);
    assert_eq!(
        span.checked_sub(span).unwrap().total_seconds(),
        TimeDelta::ZERO.total_seconds()
    );
}

#[test]
fn negative_spans_floor_towards_the_past() {
    let back = TimeDelta::from_hours(-1);
    assert_eq!(back.total_seconds(), -3_600.0);
    // A day is 86 400 s, so minus one hour is still inside the day before.
    assert_eq!(back.whole_days(), -1);
}

#[test]
fn sub_second_spans_keep_their_precision() {
    let span = TimeDelta::from_nanos(1);
    assert_eq!(span.inner().subsec_attos(), 1_000_000_000);
    assert_eq!(
        TimeDelta::from_micros(1).inner().subsec_attos(),
        1_000_000_000_000
    );
    assert_eq!(
        TimeDelta::from_millis(1).inner().subsec_attos(),
        1_000_000_000_000_000
    );
}

#[test]
fn dates_order_chronologically() {
    let mut dates = [
        Date::new(2026, 9, 21).unwrap(),
        Date::new(1970, 1, 1).unwrap(),
        Date::new(2000, 2, 29).unwrap(),
    ];
    dates.sort();
    assert_eq!(dates[0], Date::new(1970, 1, 1).unwrap());
    assert_eq!(dates[2], Date::new(2026, 9, 21).unwrap());
}

#[test]
fn spans_between_dates_are_signed() {
    let earlier = Date::new(2026, 1, 1).unwrap();
    let later = Date::new(2026, 12, 31).unwrap();
    assert_eq!(later.days_since(earlier), 364);
    assert_eq!(earlier.days_since(later), -364);
    assert_eq!(later.duration_since(earlier).whole_days(), 364);
}
