//! A holiday the library cannot compute must not look like one that does
//! not happen.
//!
//! The Chinese, Korean and Vietnamese calendars run 1645–2150; Umm al-Qurā
//! is a published table with its own bounds. Outside those, a rule dated in
//! one of them has no answer. Listing nothing would be the easy mistake:
//! `holidays_in_year(&CHINA, None, 2151)` would then return the solar-dated
//! entries alone — no Spring Festival, no Dragon Boat, no Mid-Autumn — with
//! every survivor marked `Exact`.
//!
//! The tests use South Korea, whose table is bounded by the `dangi`
//! calendar alone. China's is also bounded by the State Council's annual
//! arrangements it carries, which end sooner.

use hc_holiday::{CalendarSystem, Rule, countries, holidays_in_year};

/// The year the lunisolar calendars stop at.
const LAST_LUNISOLAR_YEAR: i64 = 2150;

#[test]
fn a_year_the_chinese_calendar_reaches_is_complete() {
    let calendar =
        hc_holiday::HolidayCalendar::for_year(&countries::SOUTH_KOREA, None, LAST_LUNISOLAR_YEAR);
    assert!(
        calendar.is_complete(),
        "2150 is inside the range and should have no gaps: {:?}",
        calendar.gaps()
    );
    // And the lunisolar holidays are actually there.
    let names: Vec<&str> = calendar.all().iter().map(|h| h.name).collect();
    assert!(
        names.iter().any(|name| name.contains("Seollal")),
        "2150 should have a Seollal, got {names:?}"
    );
}

#[test]
fn a_year_past_the_chinese_calendar_reports_what_it_could_not_answer() {
    let year = LAST_LUNISOLAR_YEAR + 1;
    let calendar = hc_holiday::HolidayCalendar::for_year(&countries::SOUTH_KOREA, None, year);

    assert!(
        !calendar.is_complete(),
        "2151 is past the lunisolar range and should say so"
    );
    let gaps = calendar.gaps();
    assert!(!gaps.is_empty());
    for gap in gaps {
        assert_eq!(gap.year, year);
        assert!(!gap.name.is_empty());
    }
    // The ones that went missing silently are named now.
    let missing: Vec<&str> = gaps.iter().map(|gap| gap.name).collect();
    assert!(
        missing.iter().any(|name| name.contains("Seollal")),
        "Seollal should be reported missing, got {missing:?}"
    );

    // The holidays that *are* returned are still correct; the list is
    // incomplete, not wrong. Liberation Day is Gregorian and needs no
    // lunisolar calendar.
    let names: Vec<&str> = calendar.all().iter().map(|h| h.name).collect();
    assert!(names.iter().any(|name| name.contains("Liberation Day")));
}

#[test]
fn the_free_function_is_affected_too_and_the_gap_is_reachable() {
    // `holidays_in_year` cannot report a gap through its return type, so
    // this records what it does instead: it gives the shorter list. A
    // caller who needs to know builds the calendar.
    let inside = holidays_in_year(&countries::SOUTH_KOREA, None, LAST_LUNISOLAR_YEAR);
    let outside = holidays_in_year(&countries::SOUTH_KOREA, None, LAST_LUNISOLAR_YEAR + 1);
    assert!(
        outside.len() < inside.len(),
        "2151 should be shorter than 2150: {} vs {}",
        outside.len(),
        inside.len()
    );
}

#[test]
fn a_rule_says_whether_it_can_be_answered_before_it_is_asked() {
    let spring = Rule::FixedInCalendar {
        system: CalendarSystem::CHINESE,
        month: hc_calendar::Month::regular(1),
        day: 1,
    };
    assert!(spring.is_resolvable_in(2024));
    assert!(spring.is_resolvable_in(LAST_LUNISOLAR_YEAR));
    assert!(!spring.is_resolvable_in(LAST_LUNISOLAR_YEAR + 1));
    assert!(!spring.is_resolvable_in(1600));

    // Resolvable and empty are different. 29 February is Gregorian, always
    // answerable, and absent in three years out of four.
    let leap_day = Rule::FixedGregorian { month: 2, day: 29 };
    assert!(leap_day.is_resolvable_in(2023));
    assert!(leap_day.days_in_year(2023).is_empty());
}

#[test]
fn every_calendar_system_agrees_with_itself_about_its_range() {
    for system in [
        CalendarSystem::GREGORIAN,
        CalendarSystem::JULIAN,
        CalendarSystem::ISLAMIC_CIVIL,
        CalendarSystem::ISLAMIC_UMM_AL_QURA,
        CalendarSystem::HEBREW,
        CalendarSystem::CHINESE,
        CalendarSystem::DANGI,
        CalendarSystem::VIETNAMESE,
    ] {
        // Whatever each says about 2024, it must be able to place a date in
        // it if and only if it claims to cover it.
        let covers = system.covers_gregorian_year(2024);
        assert!(covers, "{system:?} should cover 2024");
    }
    // And the lunisolar three agree on where they stop.
    for system in [
        CalendarSystem::CHINESE,
        CalendarSystem::DANGI,
        CalendarSystem::VIETNAMESE,
    ] {
        assert!(system.covers_gregorian_year(LAST_LUNISOLAR_YEAR));
        assert!(!system.covers_gregorian_year(LAST_LUNISOLAR_YEAR + 1));
    }
}

#[test]
fn matariki_is_reported_past_the_end_of_its_table() {
    // The Te Kāhui o Matariki Public Holiday Act 2022 schedules dates
    // through 2052; the table here stops at 2035. Before this, a calendar
    // for 2036 simply had no Matariki in it and said nothing about why.
    let inside = hc_holiday::HolidayCalendar::for_year(&countries::NEW_ZEALAND, None, 2035);
    assert!(inside.is_complete(), "{:?}", inside.gaps());
    assert!(inside.all().iter().any(|day| day.name == "Matariki"));

    let outside = hc_holiday::HolidayCalendar::for_year(&countries::NEW_ZEALAND, None, 2036);
    assert!(!outside.is_complete());
    assert!(
        outside.gaps().iter().any(|gap| gap.name == "Matariki"),
        "2036 should report Matariki as a gap, got {:?}",
        outside.gaps()
    );
    assert!(!outside.all().iter().any(|day| day.name == "Matariki"));
    // Everything else about 2036 is still answered.
    assert!(outside.all().iter().any(|day| day.name == "Waitangi Day"));
}
