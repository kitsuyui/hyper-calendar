//! `HolidayCalendar::for_day` says about a day exactly what `for_year`
//! says about it, in every table, and says it faster.
//!
//! The engine evaluates the base rules over every year within `REACH_DAYS`
//! of the span asked for, and lets a rule skip the astronomy of anything
//! that provably falls further from the span than that. For a year that is
//! three years; for a day it is one, or two near New Year, and within them
//! only the months around the day. The saving is only honest if the
//! modifiers — substitutes pushed along by other substitutes, bridges
//! between neighbours, collisions — see everything that can reach the day,
//! and that is what these tests hold every table to: the day's entries and
//! the year's gaps must be the same whichever way they were asked for,
//! through a fresh [`EvaluationContext`] or one shared with every other
//! table, and the year's must be what a wider evaluation that skips
//! nothing near it says.
//!
//! The last test is a measurement, not a gate. It times one day across
//! every table three ways and prints the figures; run it with
//! `cargo test --release -p hc-holiday --test on_day -- --nocapture`.

use std::time::Instant;

use hc_calendar::Rd;
use hc_holiday::EvaluationContext;
use hc_holiday::engine::{Gap, Holiday, HolidayCalendar};
use hc_holiday::hc_calendars_solar::gregorian;
use hc_holiday::rule::RuleSet;
use hc_holiday::{countries, exchanges, international, traditions};

fn every_table() -> impl Iterator<Item = &'static RuleSet> {
    countries::ALL
        .iter()
        .chain(exchanges::ALL)
        .chain(traditions::ALL)
        .chain(international::ALL)
        .copied()
}

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    gregorian::to_fixed(year, month, day)
        .unwrap_or_else(|error| panic!("{year}-{month}-{day}: {error}"))
}

/// The year's answer about `day`, and the day's own — through a fresh
/// context and through `shared`, which has answered every table and day
/// before this one — must agree.
fn agree(table: &RuleSet, year: &HolidayCalendar<'_>, day: Rd, shared: &mut EvaluationContext) {
    let by_day = HolidayCalendar::for_day(table, None, day);
    let from_year: Vec<Holiday> = year.on(day);
    let from_day: Vec<Holiday> = by_day.on(day);
    assert_eq!(
        from_day, from_year,
        "{}: {} on fixed day {}",
        table.code, table.english_name, day.0
    );
    assert_eq!(
        by_day.all(),
        from_day.as_slice(),
        "{}: the day carries only itself",
        table.code
    );
    let year_gaps: Vec<&Gap> = year.gaps().iter().collect();
    let day_gaps: Vec<&Gap> = by_day.gaps().iter().collect();
    assert_eq!(
        day_gaps, year_gaps,
        "{}: gaps for fixed day {}",
        table.code, day.0
    );
    assert!(by_day.covers(day));
    assert!(!by_day.covers(Rd(day.0 + 1)));
    assert!(!by_day.covers(Rd(day.0 - 1)));
    let by_shared = HolidayCalendar::for_day_with(table, None, day, shared);
    assert_eq!(
        by_shared.all(),
        by_day.all(),
        "{}: fixed day {} through a shared context",
        table.code,
        day.0
    );
    assert_eq!(
        by_shared.gaps(),
        by_day.gaps(),
        "{}: gaps for fixed day {} through a shared context",
        table.code,
        day.0
    );
}

/// Days that exercise every modifier and every kind of rule: the New Year
/// cluster where a substitute or a bridge can cross 1 January, Japan's
/// Golden Week bridge, the Christmas pair that pushes one substitute past
/// another, the days the lunisolar and Hindu festivals fall — 春節 and its
/// eve, Holi, Eid al-Fiṭr, Nowruz, Vesak, Rosh Hashanah, Dashain, Diwali —
/// and ordinary days for contrast.
const DAYS_OF_2026: [(u8, u8); 24] = [
    (1, 1),
    (1, 2),
    (1, 3),
    (1, 4),
    (1, 5),
    (1, 14),
    (2, 16),
    (2, 17),
    (3, 3),
    (3, 20),
    (3, 21),
    (4, 14),
    (5, 4),
    (5, 6),
    (5, 31),
    (9, 12),
    (9, 25),
    (10, 20),
    (11, 8),
    (12, 25),
    (12, 26),
    (12, 28),
    (12, 30),
    (12, 31),
];

/// How many times sparser the sweeps below are in a debug build, which the
/// coverage job runs instrumented; a release build, which CI's
/// release-mode job runs, checks every pairing.
const SAMPLED: usize = if cfg!(debug_assertions) { 3 } else { 1 };

#[test]
fn a_day_answers_as_its_year_does_in_every_table() {
    // Every table against every day in a release build. A debug build
    // checks each table on every third day, a different third for
    // neighbouring tables, so that every day is still checked against a
    // third of the tables and every table on eight of the days.
    let mut shared = EvaluationContext::new();
    for (index, table) in every_table().enumerate() {
        let year = HolidayCalendar::for_year(table, None, 2026);
        for (position, (month, day)) in DAYS_OF_2026.into_iter().enumerate() {
            if (index + position) % SAMPLED != 0 {
                continue;
            }
            agree(table, &year, ymd(2026, month, day), &mut shared);
        }
    }
}

#[test]
fn a_day_reports_the_gaps_its_year_does_in_every_table() {
    // 2150 is past the Chinese, Korean and Vietnamese tables' range, so
    // every lunisolar-dated holiday is a gap that year, and the day must
    // list the same gaps the year does.
    let mut shared = EvaluationContext::new();
    for table in every_table() {
        let year = HolidayCalendar::for_year(table, None, 2150);
        agree(table, &year, ymd(2150, 2, 1), &mut shared);
    }
}

#[test]
fn a_year_answers_as_a_wider_evaluation_does_in_every_table() {
    // A year's evaluation reaches sixty-two days into its neighbours and
    // skips what lies beyond; one over the year and both its neighbours
    // skips nothing within a year of it. The two must say the same about
    // the year.
    for table in every_table() {
        let year = HolidayCalendar::for_year(table, None, 2026);
        let wider = HolidayCalendar::new(table, None, 2025, 2027);
        assert_eq!(
            year.all(),
            wider.in_year(2026).as_slice(),
            "{}: {}",
            table.code,
            table.english_name
        );
        let wider_gaps: Vec<&Gap> = wider.gaps().iter().filter(|gap| gap.year == 2026).collect();
        let year_gaps: Vec<&Gap> = year.gaps().iter().collect();
        assert_eq!(year_gaps, wider_gaps, "{}: gaps", table.code);
    }
}

#[test]
fn the_substitution_heavy_tables_agree_across_a_whole_year() {
    // Every third day of a year in the tables whose laws push days around
    // the most: Japan's 振替休日 and 国民の休日, Korea's 대체공휴일 with its
    // collisions, Britain's Christmas pair, and an exchange that includes
    // its country's table. A debug build takes every ninth day instead,
    // and every holiday of the year with the day either side of it, which
    // is where a substitute or a bridge lands.
    for code in ["JP", "KR", "GB", "US", "CN", "XHKG", "XNYS"] {
        let table = countries::by_code(code)
            .or_else(|| exchanges::by_code(code))
            .expect("a known table");
        for year in [2025, 2026] {
            let calendar = HolidayCalendar::for_year(table, None, year);
            let (first, last) = (ymd(year, 1, 1), ymd(year, 12, 31));
            let mut shared = EvaluationContext::new();
            let mut days: Vec<i64> = (first.0..=last.0).step_by(3 * SAMPLED).collect();
            if SAMPLED > 1 {
                for holiday in calendar.all() {
                    days.extend(holiday.date.0 - 1..=holiday.date.0 + 1);
                }
                days.retain(|day| (first.0..=last.0).contains(day));
                days.sort_unstable();
                days.dedup();
            }
            for day in days {
                agree(table, &calendar, Rd(day), &mut shared);
            }
        }
    }
}

#[test]
fn one_day_across_every_table_is_cheaper_by_the_day_than_by_the_year() {
    let day = ymd(2026, 9, 25);
    let tables = every_table().count();

    let start = Instant::now();
    let mut by_year = 0usize;
    for table in every_table() {
        let calendar = HolidayCalendar::for_year(table, None, 2026);
        by_year += calendar.on(day).len() + calendar.gaps().len();
    }
    let year_way = start.elapsed();

    let start = Instant::now();
    let mut by_day = 0usize;
    for table in every_table() {
        let calendar = HolidayCalendar::for_day(table, None, day);
        by_day += calendar.on(day).len() + calendar.gaps().len();
    }
    let day_way = start.elapsed();

    let start = Instant::now();
    let mut shared = EvaluationContext::new();
    let mut by_shared = 0usize;
    for table in every_table() {
        let calendar = HolidayCalendar::for_day_with(table, None, day, &mut shared);
        by_shared += calendar.on(day).len() + calendar.gaps().len();
    }
    let shared_way = start.elapsed();

    assert_eq!(by_day, by_year);
    assert_eq!(by_shared, by_year);
    eprintln!(
        "one 2026 day across {tables} tables: by the year {year_way:?}, by the day {day_way:?}, by the day through one context {shared_way:?}"
    );
}
