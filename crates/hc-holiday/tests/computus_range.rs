//! A day counted from Easter is answered only where Easter is (ADR 0013).
//!
//! The Gregorian computus begins with the Easter of 1583, the first after
//! the reform of October 1582 (*Inter gravissimas*, `computus.rs`), and the
//! Julian one with 326, the first after Nicaea; both stop at
//! 4099. Outside those years `Computus::easter` has no answer, so the
//! engine reports a gap for each day counted from Easter: `christian-western`
//! gives 16 entries and 14 gaps in 1500, the 14 days from Ash Wednesday to
//! Corpus Christi being the gaps, and the same in 9999.

use hc_holiday::engine::HolidayCalendar;
use hc_holiday::rule::{Rule, RuleSet};
use hc_holiday::{Computus, countries, exchanges, international, traditions};

/// The computus a rule is counted from, looking through the rules that
/// shift or span others.
fn computus_of(rule: &Rule) -> Option<&'static Computus> {
    match rule {
        Rule::EasterRelative { computus, .. } => Some(computus),
        Rule::Offset { base, .. } | Rule::MovedByWeekday { base, .. } => computus_of(base),
        Rule::Span { from, to } => computus_of(from).or_else(|| computus_of(to)),
        _ => None,
    }
}

fn every_table() -> impl Iterator<Item = &'static RuleSet> {
    countries::ALL
        .iter()
        .chain(exchanges::ALL)
        .chain(international::ALL)
        .chain(traditions::ALL)
        .copied()
}

/// The years `computus` has no Easter for, one before it begins and one
/// after it ends.
fn years_outside(computus: &Computus) -> [i64; 2] {
    [computus.first_year - 1, 4100]
}

#[test]
fn a_day_counted_from_easter_is_a_gap_where_there_is_no_easter() {
    let mut tables_checked = 0;
    let mut rules_checked = 0;
    for table in every_table() {
        let mut touched = false;
        for rule in table.rules {
            let Some(computus) = computus_of(&rule.rule) else {
                continue;
            };
            // A rule limited to a subdivision or to one group is not in
            // the nationwide answer this asks for.
            if !rule.regions.is_empty() || !rule.groups.is_empty() {
                continue;
            }
            for year in years_outside(computus) {
                if !rule.applies_in(year) {
                    continue;
                }
                let calendar = HolidayCalendar::for_year(table, None, year);
                assert!(
                    calendar.gaps().iter().any(|gap| gap.id == rule.id()),
                    "{} {year}: {} counted from {} Easter is neither a day nor a gap",
                    table.code,
                    rule.name,
                    computus.id,
                );
                // Another rule of the table may share the identifier and
                // not need Easter: the Seven Sorrows is the Friday of
                // Passion Week and 15 September.
                let shared = table
                    .rules
                    .iter()
                    .any(|other| other.id() == rule.id() && computus_of(&other.rule).is_none());
                assert!(
                    shared || calendar.all().iter().all(|holiday| holiday.id != rule.id()),
                    "{} {year}: {} is a day in a year with no Easter",
                    table.code,
                    rule.name,
                );
                rules_checked += 1;
            }
            touched = true;
        }
        tables_checked += usize::from(touched);
    }
    // A sweep that matched nothing proves nothing.
    assert!(tables_checked > 100, "{tables_checked} tables");
    assert!(rules_checked > 500, "{rules_checked} rules");
}

#[test]
fn the_western_christian_year_names_the_days_it_cannot_place() {
    for year in [1500, 4100, 9999] {
        let calendar = HolidayCalendar::for_year(&traditions::CHRISTIAN_WESTERN, None, year);
        let gaps: Vec<&str> = calendar.gaps().iter().map(|gap| gap.name).collect();
        for name in ["Ash Wednesday", "Good Friday", "Easter", "Pentecost"] {
            assert!(
                gaps.iter().any(|gap| gap.contains(name)),
                "{year}: {name} should be a gap, gaps are {gaps:?}"
            );
        }
        // The fixed feasts are still answered.
        assert!(
            calendar
                .all()
                .iter()
                .any(|holiday| holiday.name.contains("Christmas")),
            "{year}: Christmas is a fixed date and needs no Easter"
        );
    }
    // And a year inside the range has none of them missing.
    let calendar = HolidayCalendar::for_year(&traditions::CHRISTIAN_WESTERN, None, 2026);
    assert!(
        calendar.gaps().is_empty(),
        "2026 is inside the range: {:?}",
        calendar.gaps()
    );
}

#[test]
fn the_julian_computus_reaches_back_to_nicaea() {
    let before = HolidayCalendar::for_year(&traditions::CHRISTIAN_ORTHODOX, None, 325);
    let at = HolidayCalendar::for_year(&traditions::CHRISTIAN_ORTHODOX, None, 326);
    assert!(
        before.gaps().iter().any(|gap| gap.name.contains("Pascha")),
        "325: {:?}",
        before.gaps()
    );
    assert!(
        !at.gaps().iter().any(|gap| gap.name.contains("Pascha")),
        "326: {:?}",
        at.gaps()
    );
}

#[test]
fn a_day_computed_from_easter_is_a_gap_past_the_computus_too() {
    // The days the tables compute with a function of Easter, not an offset:
    // Elijah's first Sunday, the Greater Litanies, the last Sunday after
    // Pentecost. They answered with nothing in 4100 and nothing in 1500.
    for (code, name, year) in [
        ("church-of-the-east", "First Sunday of Elijah", 4100),
        ("chaldean", "First Sunday of Elijah", 4100),
        (
            "rogation-roman-1960",
            "Greater Litanies (Major Rogation)",
            4100,
        ),
        (
            "rogation-roman-1960",
            "Greater Litanies (Major Rogation)",
            1500,
        ),
        (
            "roman-general-1960",
            "Twenty-fourth and last Sunday after Pentecost",
            4100,
        ),
    ] {
        let Some(table) = traditions::by_code(code) else {
            panic!("no table {code}");
        };
        let calendar = HolidayCalendar::for_year(table, None, year);
        assert!(
            calendar.gaps().iter().any(|gap| gap.name.contains(name)),
            "{code} {year}: {name} is not a gap: {:?}",
            calendar
                .gaps()
                .iter()
                .map(|gap| gap.name)
                .collect::<Vec<_>>()
        );
        assert!(
            calendar
                .all()
                .iter()
                .all(|holiday| !holiday.name.contains(name)),
            "{code} {year}: {name} is a day"
        );
    }
}
