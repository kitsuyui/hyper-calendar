//! `HolidayCalendar::for_day` says about a day exactly what `for_year`
//! says about it, in every table, and says it faster.
//!
//! The engine evaluates the base rules over every year within `REACH_DAYS`
//! of the span asked for. For a year that is three years; for a day it is
//! one, or two near New Year. The saving is only honest if the modifiers —
//! substitutes pushed along by other substitutes, bridges between
//! neighbours, collisions — see everything that can reach the day, and
//! that is what these tests hold every table to: the day's entries and the
//! year's gaps must be the same whichever way they were asked for.
//!
//! The last test is a measurement, not a gate. It times one day across
//! every table both ways and prints the two figures; run it with
//! `cargo test --release -p hc-holiday --test on_day -- --nocapture`.

use std::time::Instant;

use hc_calendar::Rd;
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

/// The year's answer about `day`, and the day's own, must agree.
fn agree(table: &RuleSet, year: &HolidayCalendar<'_>, day: Rd) {
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
}

/// Days that exercise every modifier: the New Year cluster where a
/// substitute or a bridge can cross 1 January, Japan's Golden Week bridge,
/// the Christmas pair that pushes one substitute past another, and an
/// ordinary day for contrast.
const DAYS_OF_2026: [(u8, u8); 12] = [
    (1, 1),
    (1, 2),
    (1, 3),
    (1, 4),
    (1, 5),
    (5, 4),
    (5, 6),
    (9, 25),
    (12, 25),
    (12, 26),
    (12, 28),
    (12, 31),
];

#[test]
fn a_day_answers_as_its_year_does_in_every_table() {
    for table in every_table() {
        let year = HolidayCalendar::for_year(table, None, 2026);
        for (month, day) in DAYS_OF_2026 {
            agree(table, &year, ymd(2026, month, day));
        }
    }
}

#[test]
fn a_day_reports_the_gaps_its_year_does_in_every_table() {
    // 2150 is past the Chinese, Korean and Vietnamese tables' range, so
    // every lunisolar-dated holiday is a gap that year, and the day must
    // list the same gaps the year does.
    for table in every_table() {
        let year = HolidayCalendar::for_year(table, None, 2150);
        agree(table, &year, ymd(2150, 2, 1));
    }
}

#[test]
fn the_substitution_heavy_tables_agree_across_a_whole_year() {
    // Every third day of a year in the tables whose laws push days around
    // the most: Japan's 振替休日 and 国民の休日, Korea's 대체공휴일 with its
    // collisions, Britain's Christmas pair, and an exchange that includes
    // its country's table.
    for code in ["JP", "KR", "GB", "US", "CN", "XHKG", "XNYS"] {
        let table = countries::by_code(code)
            .or_else(|| exchanges::by_code(code))
            .expect("a known table");
        for year in [2025, 2026] {
            let calendar = HolidayCalendar::for_year(table, None, year);
            let (first, last) = (ymd(year, 1, 1), ymd(year, 12, 31));
            let mut day = first;
            while day <= last {
                agree(table, &calendar, day);
                day = Rd(day.0 + 3);
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

    assert_eq!(by_day, by_year);
    eprintln!(
        "one 2026 day across {tables} tables: by the year {year_way:?}, by the day {day_way:?}"
    );
}
