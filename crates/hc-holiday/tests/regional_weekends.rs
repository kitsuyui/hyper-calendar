//! A region's own weekend (ADR 0015): Malaysia's states that keep Friday,
//! Sharjah's government, and what each does to substitution, business-day
//! arithmetic and the years its sources do not reach.

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;
use hc_holiday::countries;
use hc_holiday::engine::{Holiday, HolidayCalendar, Unanswered};
use hc_holiday::rule::{RuleSet, Scope, UNREAD_SUBDIVISION, UNREAD_WEEKEND};

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
fn table(code: &str) -> &'static RuleSet {
    match countries::by_code(code) {
        Some(table) => table,
        None => panic!("{code} is not a registered country"),
    }
}

fn year_in<'a>(code: &str, region: Option<&'a str>, year: i64) -> HolidayCalendar<'a> {
    HolidayCalendar::for_year_scoped(table(code), Scope::new(region, None), year)
}

fn on(calendar: &HolidayCalendar<'_>, day: Rd) -> Vec<Holiday> {
    calendar.on(day)
}

#[test]
fn a_state_that_keeps_friday_has_a_friday_and_saturday_weekend() {
    // Kedah, Kelantan and Terengganu keep Friday and Saturday (the Jakarta
    // Post, 25 November 2013; MKN, 31 December 2024); the rest of Malaysia
    // keeps Saturday and Sunday. 2026-03-06 was a Friday.
    let federal = year_in("MY", None, 2026);
    let friday = ymd(2026, 3, 6);
    let sunday = ymd(2026, 3, 8);
    assert!(!federal.is_weekend(friday));
    assert!(federal.is_weekend(sunday));
    for code in ["MY-02", "MY-03", "MY-11"] {
        let state = year_in("MY", Some(code), 2026);
        assert!(state.is_weekend(friday), "{code}");
        assert!(state.is_weekend(ymd(2026, 3, 7)), "{code}");
        assert!(!state.is_weekend(sunday), "{code}");
        assert!(state.is_business_day(sunday), "{code}");
        assert!(!state.is_business_day(friday), "{code}");
    }
    // Perlis keeps the country's weekend, as does every other state.
    for code in ["MY-09", "MY-10", "MY-14", "MY-12"] {
        let state = year_in("MY", Some(code), 2026);
        assert!(!state.is_weekend(friday), "{code}");
        assert!(state.is_weekend(sunday), "{code}");
    }
}

#[test]
fn a_municipality_has_the_weekend_of_its_subdivision() {
    let malaysia = table("MY");
    let friday = ymd(2026, 3, 6);
    assert_eq!(
        malaysia.weekend_in(Some("MY-02-100"), friday),
        malaysia.weekend_in(Some("MY-02"), friday)
    );
}

#[test]
fn johor_changed_its_weekend_on_1_january_2014_and_back_on_1_january_2025() {
    // Sultan Ibrahim's decree of 2013 for Friday and Saturday from 1
    // January 2014; the Regent's announcement of 7 October 2024 for
    // Saturday and Sunday from 1 January 2025.
    let johor = table("MY");
    let friday = |y, m, d| Weekday::from_rd(ymd(y, m, d)) == Weekday::Friday;
    assert!(friday(2013, 12, 27) && friday(2014, 1, 3) && friday(2024, 12, 27));
    let kept = |y, m, d| {
        johor
            .weekend_in(Some("MY-01"), ymd(y, m, d))
            .map(|days| days.contains(&Weekday::from_rd(ymd(y, m, d))))
    };
    assert_eq!(
        kept(2013, 12, 27),
        Some(false),
        "a Friday of the old weekend"
    );
    assert_eq!(kept(2013, 12, 28), Some(true), "a Saturday");
    assert_eq!(kept(2013, 12, 29), Some(true), "a Sunday");
    assert_eq!(kept(2013, 12, 31), Some(false), "the last Tuesday of it");
    assert_eq!(kept(2014, 1, 3), Some(true), "the first Friday of the new");
    assert_eq!(kept(2014, 1, 5), Some(false), "the first Sunday of it");
    assert_eq!(kept(2024, 12, 27), Some(true));
    assert_eq!(kept(2024, 12, 29), Some(false), "the last Sunday of it");
    assert_eq!(
        kept(2025, 1, 3),
        Some(false),
        "the first Friday of the second"
    );
    assert_eq!(kept(2025, 1, 4), Some(true));
    assert_eq!(kept(2025, 1, 5), Some(true));
}

#[test]
fn the_years_the_sources_do_not_reach_are_a_gap_not_a_weekend() {
    let malaysia = table("MY");
    // Johor to 1994, and Kedah, Kelantan, Terengganu and Perlis to the day
    // before the Jakarta Post's report of 25 November 2013.
    assert_eq!(malaysia.weekend_in(Some("MY-01"), ymd(1994, 6, 3)), None);
    assert!(
        malaysia
            .weekend_in(Some("MY-01"), ymd(1995, 6, 3))
            .is_some()
    );
    for code in ["MY-02", "MY-03", "MY-09", "MY-11"] {
        assert_eq!(
            malaysia.weekend_in(Some(code), ymd(2010, 6, 4)),
            None,
            "{code}"
        );
        assert_eq!(
            malaysia.weekend_in(Some(code), ymd(2013, 11, 24)),
            None,
            "{code}"
        );
    }
    assert!(
        malaysia
            .weekend_in(Some("MY-09"), ymd(2013, 11, 25))
            .is_some()
    );
    assert!(
        malaysia
            .weekend_in(Some("MY-02"), ymd(2013, 11, 25))
            .is_some()
    );
    // The nationwide table has none: it was never unread.
    assert!(malaysia.weekend_in(None, ymd(1994, 6, 3)).is_some());
    assert!(
        malaysia
            .weekend_in(Some("MY-14"), ymd(1994, 6, 3))
            .is_some()
    );

    let kedah = year_in("MY", Some("MY-02"), 2010);
    assert!(
        kedah
            .gaps()
            .iter()
            .any(|gap| gap.name == UNREAD_WEEKEND && gap.year == 2010),
        "Kedah's weekend of 2010 is a gap"
    );
    assert!(!kedah.weekend_is_read(ymd(2010, 6, 4)));
    assert!(!kedah.is_weekend(ymd(2010, 6, 4)));
    // Business-day arithmetic refuses rather than guess.
    let walk = HolidayCalendar::scoped(table("MY"), Scope::region("MY-02"), 2010, 2011);
    assert_eq!(walk.add_business_days(ymd(2010, 6, 1), 3), None);
    assert_eq!(
        walk.business_days_between(ymd(2010, 6, 1), ymd(2010, 6, 8)),
        None
    );
    // The nationwide answer is not a gap, and the state's own days are
    // still the gap they were.
    let federal = HolidayCalendar::for_year(table("MY"), None, 2010);
    assert!(federal.gaps().iter().all(|gap| gap.name != UNREAD_WEEKEND));
    assert!(
        kedah
            .gaps()
            .iter()
            .any(|gap| gap.name == UNREAD_SUBDIVISION)
    );
    // A year after the report is not.
    let later = year_in("MY", Some("MY-02"), 2026);
    assert!(later.gaps().iter().all(|gap| gap.name != UNREAD_WEEKEND));
    // The year of the report is: its first days were.
    let report = year_in("MY", Some("MY-02"), 2013);
    assert!(report.gaps().iter().any(|gap| gap.name == UNREAD_WEEKEND));
}

#[test]
fn business_days_count_the_states_own_weekend() {
    // Thursday 5 March 2026. The next working day is Friday in the country
    // and Sunday in a state that keeps Friday and Saturday; the second is
    // Monday in both. Kedah's own days were not read (ADR 0013), so the walk
    // there is refused as a gap, however its weekend falls; the arithmetic
    // of a state's own weekend is held to a table that was read, in
    // `open_days.rs`.
    let thursday = ymd(2026, 3, 5);
    let federal = year_in("MY", None, 2026);
    let kedah = year_in("MY", Some("MY-02"), 2026);
    assert_eq!(
        federal.add_business_days(thursday, 1),
        Some(ymd(2026, 3, 6))
    );
    assert_eq!(kedah.add_business_days(thursday, 1), None);
    assert_eq!(
        kedah.try_add_business_days(thursday, 1),
        Err(Unanswered::Gap)
    );
    assert_eq!(
        federal.add_business_days(thursday, 2),
        Some(ymd(2026, 3, 9))
    );
    // The weekend itself is read: Kedah's Friday and Saturday.
    assert!(kedah.weekend_is_read(thursday));
    assert!(kedah.is_weekend(ymd(2026, 3, 6)) && !kedah.is_weekend(ymd(2026, 3, 8)));
    // Backwards from Monday 9 March: Friday for the country.
    let monday = ymd(2026, 3, 9);
    assert_eq!(federal.add_business_days(monday, -1), Some(ymd(2026, 3, 6)));
    assert_eq!(kedah.add_business_days(monday, -1), None);
    // Friday to Monday holds Friday for the country; Saturday to Monday
    // holds none.
    assert_eq!(
        federal.business_days_between(ymd(2026, 3, 6), monday),
        Some(1)
    );
    assert_eq!(kedah.business_days_between(ymd(2026, 3, 6), monday), None);
    assert_eq!(
        federal.business_days_between(ymd(2026, 3, 7), monday),
        Some(0)
    );
    // A whole week from Sunday to Sunday: five days.
    assert_eq!(
        federal.business_days_between(ymd(2026, 3, 8), ymd(2026, 3, 15)),
        Some(5)
    );
}

#[test]
fn a_friday_holiday_is_moved_to_the_sunday_in_kedah_and_not_elsewhere() {
    // Awal Muharram 2025 is Friday 27 June, and Hari Raya Haji Saturday 7
    // June (the Prime Minister's Department's list).
    let friday = ymd(2025, 6, 27);
    let sunday = ymd(2025, 6, 29);
    let federal = year_in("MY", None, 2025);
    assert!(federal.is_holiday(friday));
    assert!(!federal.is_holiday(sunday));
    assert!(!federal.is_holiday(ymd(2025, 6, 30)));
    let kedah = year_in("MY", Some("MY-02"), 2025);
    assert!(kedah.is_holiday(friday));
    assert!(
        kedah.is_holiday(sunday),
        "the replacement, past the Saturday"
    );
    let moved = on(&kedah, sunday);
    assert_eq!(moved.len(), 1);
    assert_eq!(moved[0].observed_for, Some(friday));
    assert!(!kedah.is_holiday(ymd(2025, 6, 28)));
    // Kedah moves no Saturday's holiday: Hari Raya Haji stays on the 7th.
    assert!(kedah.is_holiday(ymd(2025, 6, 7)));
    assert!(!kedah.is_holiday(ymd(2025, 6, 8)));
    // Kelantan and Terengganu move a Saturday's holiday, not a Friday's.
    for code in ["MY-03", "MY-11"] {
        let state = year_in("MY", Some(code), 2025);
        assert!(state.is_holiday(friday), "{code}");
        assert!(!state.is_holiday(sunday), "{code}");
        assert!(state.is_holiday(ymd(2025, 6, 8)), "{code}");
    }
}

#[test]
fn a_saturday_holiday_is_moved_to_the_sunday_in_kelantan_and_terengganu() {
    // National Day 2024 was a Saturday, 31 August.
    let saturday = ymd(2024, 8, 31);
    let sunday = ymd(2024, 9, 1);
    let federal = year_in("MY", None, 2024);
    assert!(federal.is_holiday(saturday));
    assert!(!federal.is_holiday(sunday));
    for code in ["MY-03", "MY-11"] {
        let state = year_in("MY", Some(code), 2024);
        assert!(state.is_holiday(saturday), "{code}");
        assert!(state.is_holiday(sunday), "{code}");
        assert_eq!(on(&state, sunday)[0].observed_for, Some(saturday), "{code}");
    }
    // Kedah moves a Friday's, and Johor, which kept Friday and Saturday
    // that year, a Friday's.
    for code in ["MY-02", "MY-01"] {
        let state = year_in("MY", Some(code), 2024);
        assert!(state.is_holiday(saturday), "{code}");
        assert!(!state.is_holiday(sunday), "{code}");
    }
}

#[test]
fn johor_moved_a_friday_holiday_to_the_sunday_only_while_it_kept_friday() {
    // Christmas Day 2020 was a Friday and 2026's is too.
    let johor_2020 = year_in("MY", Some("MY-01"), 2020);
    assert!(johor_2020.is_holiday(ymd(2020, 12, 25)));
    assert!(johor_2020.is_holiday(ymd(2020, 12, 27)));
    assert_eq!(
        on(&johor_2020, ymd(2020, 12, 27))[0].observed_for,
        Some(ymd(2020, 12, 25))
    );
    let johor_2026 = year_in("MY", Some("MY-01"), 2026);
    assert!(johor_2026.is_holiday(ymd(2026, 12, 25)));
    assert!(!johor_2026.is_holiday(ymd(2026, 12, 27)));
    // A Sunday holiday goes to the Monday under the country's law, in
    // Johor since 2025 and elsewhere in every year: Christmas 2022 was a
    // Sunday.
    let johor_2022 = year_in("MY", Some("MY-01"), 2022);
    assert!(johor_2022.is_holiday(ymd(2022, 12, 25)));
    assert!(!johor_2022.is_holiday(ymd(2022, 12, 26)));
    let federal_2022 = year_in("MY", None, 2022);
    assert!(federal_2022.is_holiday(ymd(2022, 12, 26)));
}

#[test]
fn sharjahs_government_keeps_friday_to_sunday_from_2022() {
    // The Sharjah Executive Council's four-day week, from 1 January 2022;
    // the federal weekend was Friday and Saturday until then, and is
    // Saturday and Sunday after.
    let federal_2021 = year_in("AE", None, 2021);
    let sharjah_2021 = year_in("AE", Some("AE-SH"), 2021);
    let friday_2021 = ymd(2021, 12, 31);
    assert!(federal_2021.is_weekend(friday_2021));
    assert!(sharjah_2021.is_weekend(friday_2021));
    let federal = year_in("AE", None, 2026);
    let sharjah = year_in("AE", Some("AE-SH"), 2026);
    for (day, federal_off, sharjah_off) in [
        (ymd(2026, 3, 5), false, false),
        (ymd(2026, 3, 6), false, true),
        (ymd(2026, 3, 7), true, true),
        (ymd(2026, 3, 8), true, true),
        (ymd(2026, 3, 9), false, false),
    ] {
        assert_eq!(federal.is_weekend(day), federal_off, "federal {day:?}");
        assert_eq!(sharjah.is_weekend(day), sharjah_off, "Sharjah {day:?}");
    }
    // Sharjah's own days were not read, so the walk is refused as a gap;
    // the weekend arithmetic is held to a table that was read, in
    // `open_days.rs`.
    assert_eq!(
        sharjah.try_add_business_days(ymd(2026, 3, 5), 1),
        Err(Unanswered::Gap)
    );
    assert_eq!(
        federal.add_business_days(ymd(2026, 3, 5), 1),
        Some(ymd(2026, 3, 6))
    );
    // The first Sunday of the new law.
    let sharjah_2022 = year_in("AE", Some("AE-SH"), 2022);
    let federal_2022 = year_in("AE", None, 2022);
    assert!(sharjah_2022.is_weekend(ymd(2022, 1, 2)));
    assert!(federal_2022.is_weekend(ymd(2022, 1, 2)));
    assert!(sharjah_2022.is_weekend(ymd(2022, 1, 7)));
    assert!(!federal_2022.is_weekend(ymd(2022, 1, 7)));
}

#[test]
fn every_scoped_weekend_names_a_region_the_table_lists() {
    // A weekend scoped to a region the engine cannot be asked for would be
    // dead data: every code is an ISO 3166-2 code of its table's country.
    for code in ["MY", "AE"] {
        let set = table(code);
        for region in set.weekend_regions() {
            assert!(region.starts_with(&format!("{code}-")), "{region}");
        }
    }
    assert_eq!(
        table("MY").weekend_regions(),
        ["MY-01", "MY-02", "MY-03", "MY-09", "MY-11"]
    );
    assert_eq!(table("AE").weekend_regions(), ["AE-SH"]);
    assert!(table("JP").weekend_regions().is_empty());
}
