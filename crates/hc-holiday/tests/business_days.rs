//! Business-day arithmetic, under four different weekends.
//!
//! The weekend is data, not an assumption. These tests walk the same
//! arithmetic through a Saturday–Sunday weekend, a Friday–Saturday one, a
//! Thursday–Friday one and a Saturday-only one, and across holidays in each.

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;
use hc_holiday::countries;
use hc_holiday::engine::HolidayCalendar;
use hc_holiday::rule::{
    HolidayRule, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate, SubstituteDirection,
    SubstitutionPolicy, WeekendPolicy,
};

/// Panics rather than returning a `Result`, because every date in this file
/// is a literal the author typed and a bad one is a bug in the test.
///
/// # Panics
///
/// When the year, month and day are not a Gregorian date.
fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

/// # Panics
///
/// When `code` is not a country this crate carries.
fn calendar(code: &str, first: i64, last: i64) -> HolidayCalendar<'static> {
    let Some(country) = countries::by_code(code) else {
        panic!("{code} is not a registered country");
    };
    HolidayCalendar::new(country, None, first, last)
}

#[test]
fn adding_zero_business_days_returns_the_day_itself_even_on_a_holiday() {
    let japan = calendar("JP", 2024, 2024);
    let holiday = ymd(2024, 1, 1);
    assert!(!japan.is_business_day(holiday));
    assert_eq!(japan.add_business_days(holiday, 0), Some(holiday));
}

#[test]
fn adding_business_days_steps_over_a_japanese_holiday() {
    let japan = calendar("JP", 2024, 2024);
    // Friday 26 April 2024. Golden Week: 29 April is 昭和の日, and 3, 4, 5
    // and 6 May are holidays, so the next five business days are 30 April,
    // 1, 2 May, then 7 and 8 May.
    let friday = ymd(2024, 4, 26);
    assert_eq!(japan.add_business_days(friday, 1), Some(ymd(2024, 4, 30)));
    assert_eq!(japan.add_business_days(friday, 3), Some(ymd(2024, 5, 2)));
    assert_eq!(japan.add_business_days(friday, 4), Some(ymd(2024, 5, 7)));
    assert_eq!(japan.add_business_days(friday, 5), Some(ymd(2024, 5, 8)));
}

#[test]
fn subtracting_business_days_walks_the_same_holidays_backwards() {
    let japan = calendar("JP", 2024, 2024);
    let wednesday = ymd(2024, 5, 8);
    assert_eq!(
        japan.add_business_days(wednesday, -1),
        Some(ymd(2024, 5, 7))
    );
    assert_eq!(
        japan.add_business_days(wednesday, -2),
        Some(ymd(2024, 5, 2))
    );
    assert_eq!(
        japan.add_business_days(wednesday, -5),
        Some(ymd(2024, 4, 26))
    );
}

#[test]
fn adding_and_subtracting_business_days_are_inverses_from_a_business_day() {
    let uk = calendar("GB", 2024, 2026);
    let mut cursor = ymd(2024, 6, 3);
    for step in 1..=200 {
        let forward = uk.add_business_days(cursor, 1).expect("in range");
        let back = uk.add_business_days(forward, -1).expect("in range");
        assert_eq!(back, cursor, "step {step}");
        cursor = forward;
    }
}

#[test]
fn business_days_between_counts_the_half_open_interval() {
    let us = calendar("US", 2024, 2024);
    // Monday 25 November to Monday 2 December 2024: Thanksgiving on the
    // 28th removes one of the five weekdays.
    assert_eq!(
        us.business_days_between(ymd(2024, 11, 25), ymd(2024, 12, 2)),
        Some(4)
    );
    // The same interval measured backwards.
    assert_eq!(
        us.business_days_between(ymd(2024, 12, 2), ymd(2024, 11, 25)),
        Some(-4)
    );
    assert_eq!(
        us.business_days_between(ymd(2024, 11, 25), ymd(2024, 11, 25)),
        Some(0)
    );
}

#[test]
fn business_days_between_composes_over_a_split_point() {
    let germany = calendar("DE", 2025, 2025);
    let start = ymd(2025, 1, 2);
    let middle = ymd(2025, 6, 2);
    let end = ymd(2025, 12, 1);
    let first = germany
        .business_days_between(start, middle)
        .expect("in range");
    let second = germany
        .business_days_between(middle, end)
        .expect("in range");
    let whole = germany.business_days_between(start, end).expect("in range");
    assert_eq!(first + second, whole);
    assert!(whole > 200 && whole < 250, "implausible count {whole}");
}

#[test]
fn a_friday_saturday_weekend_moves_the_arithmetic_by_two_days() {
    // Egypt rests on Friday and Saturday.
    let egypt = calendar("EG", 2025, 2025);
    let wednesday = ymd(2025, 3, 5);
    assert_eq!(Weekday::from_rd(wednesday), Weekday::Wednesday);
    assert!(egypt.is_business_day(wednesday));
    assert!(egypt.is_business_day(ymd(2025, 3, 6)));
    assert!(!egypt.is_business_day(ymd(2025, 3, 7)));
    assert!(!egypt.is_business_day(ymd(2025, 3, 8)));
    assert!(egypt.is_business_day(ymd(2025, 3, 9)));
    // Thursday plus one business day is Sunday.
    assert_eq!(
        egypt.add_business_days(ymd(2025, 3, 6), 1),
        Some(ymd(2025, 3, 9))
    );
    // A full week is five business days, as everywhere.
    assert_eq!(
        egypt.business_days_between(ymd(2025, 3, 2), ymd(2025, 3, 9)),
        Some(5)
    );
}

#[test]
fn the_gulf_arithmetic_crosses_a_holiday_and_a_friday_saturday_weekend() {
    // 25 January 2025 (Revolution Day) was a Saturday and 26 January a
    // Sunday. Egypt does not move a weekend holiday, so the working week
    // resumes on the Sunday.
    let egypt = calendar("EG", 2025, 2025);
    assert!(egypt.is_holiday(ymd(2025, 1, 25)));
    assert!(!egypt.is_business_day(ymd(2025, 1, 24)));
    assert_eq!(
        egypt.add_business_days(ymd(2025, 1, 23), 1),
        Some(ymd(2025, 1, 26))
    );
}

#[test]
fn saudi_arabia_before_2013_rested_on_thursday_and_friday() {
    let saudi = calendar("SA", 2010, 2010);
    // 2010-03-03 was a Wednesday.
    assert!(saudi.is_business_day(ymd(2010, 3, 3)));
    assert!(!saudi.is_business_day(ymd(2010, 3, 4)));
    assert!(!saudi.is_business_day(ymd(2010, 3, 5)));
    assert!(saudi.is_business_day(ymd(2010, 3, 6)));
    assert_eq!(
        saudi.add_business_days(ymd(2010, 3, 3), 1),
        Some(ymd(2010, 3, 6))
    );
}

#[test]
fn nepal_gets_six_business_days_a_week_before_2026() {
    let nepal = calendar("NP", 2024, 2024);
    // 2024-03-17 was a Sunday; only Saturday is the weekend, and no Nepali
    // holiday falls in that week.
    assert_eq!(
        nepal.business_days_between(ymd(2024, 3, 17), ymd(2024, 3, 24)),
        Some(6)
    );
    // The week before has one fewer, because 8 March is a holiday.
    assert_eq!(
        nepal.business_days_between(ymd(2024, 3, 3), ymd(2024, 3, 10)),
        Some(5)
    );
    assert_eq!(
        nepal.add_business_days(ymd(2024, 3, 8), 1),
        Some(ymd(2024, 3, 10))
    );
}

#[test]
fn a_weekend_law_that_changes_changes_the_arithmetic_with_it() {
    let country = countries::by_code("AE").expect("the Emirates");
    let before = HolidayCalendar::new(country, None, 2021, 2021);
    let after = HolidayCalendar::new(country, None, 2023, 2023);
    // 2021-03-07 and 2023-03-05 were both Sundays.
    assert!(before.is_business_day(ymd(2021, 3, 7)));
    assert!(!after.is_business_day(ymd(2023, 3, 5)));
    // 2021-03-05 and 2023-03-03 were both Fridays.
    assert!(!before.is_business_day(ymd(2021, 3, 5)));
    assert!(after.is_business_day(ymd(2023, 3, 3)));
}

#[test]
fn business_day_arithmetic_refuses_to_walk_off_the_evaluated_span() {
    let japan = calendar("JP", 2024, 2024);
    assert_eq!(japan.add_business_days(ymd(2024, 12, 30), 10), None);
    assert_eq!(japan.add_business_days(ymd(2024, 1, 2), -10), None);
    assert_eq!(
        japan.business_days_between(ymd(2023, 12, 1), ymd(2024, 2, 1)),
        None
    );
}

#[test]
fn a_caller_can_supply_their_own_rule_set_and_get_the_same_engine() {
    // A four-day working week with Wednesday off, one company holiday, and
    // a substitution rule of its own. Nothing about this is a country.
    static RULES: [HolidayRule; 1] = [HolidayRule::public(
        "Founders' Day",
        "",
        Rule::gregorian(3, 15),
    )];
    static WEEKEND: [WeekendPolicy; 1] = [WeekendPolicy {
        days: &[Weekday::Wednesday, Weekday::Saturday, Weekday::Sunday],
        valid_from: None,
        valid_from_day: None,
        valid_until: None,
        valid_until_day: None,
    }];
    static SUBSTITUTION: [SubstitutionPolicy; 1] = [SubstitutionPolicy {
        trigger: &[Weekday::Saturday, Weekday::Sunday],
        direction: SubstituteDirection::Forward,
        skip_occupied: true,
        on_collision: false,
        valid_from: None,
        valid_until: None,
    }];
    static COMPANY: RuleSet = RuleSet {
        code: "acme",
        english_name: "A company calendar",
        rules: &RULES,
        substitution: &SUBSTITUTION,
        bridges: &[],
        includes: &[],
        weekend: &WEEKEND,
        sources_checked: SourceDate::new(2026, 9, 21),
        sources: "the staff handbook",
    };
    let company = HolidayCalendar::new(&COMPANY, None, 2025, 2025);
    assert!(!company.is_business_day(ymd(2025, 3, 12)));
    assert!(company.is_business_day(ymd(2025, 3, 13)));
    // 15 March 2025 was a Saturday, so Founders' Day was kept on the Monday.
    assert!(company.is_holiday(ymd(2025, 3, 17)));
    // Monday to Monday is three business days: Tuesday, Thursday, Friday.
    assert_eq!(
        company.business_days_between(ymd(2025, 3, 3), ymd(2025, 3, 10)),
        Some(4)
    );
}

#[test]
fn the_default_weekend_is_saturday_and_sunday_when_a_table_says_nothing() {
    static NOTHING: [HolidayRule; 0] = [];
    static SILENT: RuleSet = RuleSet {
        code: "silent",
        english_name: "A table that makes no weekend claim",
        rules: &NOTHING,
        substitution: &[],
        bridges: &[],
        includes: &[],
        weekend: &[],
        sources_checked: SourceDate::new(2026, 9, 21),
        sources: "nothing at all",
    };
    let calendar = HolidayCalendar::new(&SILENT, None, 2025, 2025);
    assert!(calendar.is_weekend(ymd(2025, 3, 8)));
    assert!(calendar.is_weekend(ymd(2025, 3, 9)));
    assert!(calendar.is_business_day(ymd(2025, 3, 10)));
    // And the shared constant says the same thing.
    assert_eq!(SATURDAY_SUNDAY.len(), 1);
    assert_eq!(SATURDAY_SUNDAY[0].days.len(), 2);
}

#[test]
fn a_year_has_roughly_the_number_of_business_days_it_should() {
    // Between 240 and 255 working days is the range every Saturday–Sunday
    // country in this crate falls into, and it is a cheap guard against a
    // table that has accidentally duplicated or lost a holiday.
    for code in ["JP", "US", "GB", "DE", "FR", "CA", "AU", "NZ", "SE", "PL"] {
        let calendar = calendar(code, 2025, 2025);
        let count = calendar
            .business_days_between(ymd(2025, 1, 1), ymd(2026, 1, 1))
            .or_else(|| calendar.business_days_between(ymd(2025, 1, 1), ymd(2025, 12, 31)))
            .expect("in range");
        assert!(
            (235..=256).contains(&count),
            "{code} has {count} business days in 2025"
        );
    }
}

#[test]
fn is_business_day_agrees_with_the_weekend_and_holiday_predicates() {
    let japan = calendar("JP", 2024, 2024);
    let mut day = ymd(2024, 1, 1);
    while day <= ymd(2024, 12, 31) {
        assert_eq!(
            japan.is_business_day(day),
            !japan.is_weekend(day) && !japan.is_holiday(day)
        );
        day = Rd(day.0 + 1);
    }
}
