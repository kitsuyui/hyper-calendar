//! The regions a holiday table lists are exactly the regions it answers
//! for: column 9 of `hc_holiday_tables` is `RuleSet::answered_regions`, the
//! subdivisions its rules, its weekend laws and its substitution policies
//! are scoped to, and every export that takes a region accepts each of them
//! and no region that has days of its own but is missing from the list
//! (audit 10, front-end request 30; ADR 0015).
//!
//! Kedah, `MY-02`, keeps Friday and Saturday and has no rule of its own, so
//! it appears in column 14 of the table and, before this definition, not in
//! column 9, which is where a front end builds the menu of regions.

#![cfg(all(feature = "holiday", feature = "place-names"))]

use std::collections::BTreeSet;

use hyper_calendar::boundary::Refusal;
use hyper_calendar::hc_calendars_solar::gregorian;
use hyper_calendar::hc_holiday::countries;
use hyper_calendar::hc_i18n::place_names;
use hyper_calendar::holiday_lines::{
    add_business_days, business_days_between, holiday_beyond, holiday_codes, holiday_tables,
    holidays_in_year, is_day_off, is_weekend, rule_set,
};
use hyper_calendar::place_lines::place_name;

fn day(year: i64, month: u8, day: u8) -> i64 {
    gregorian::to_fixed(year, month, day)
        .unwrap_or_else(|_| panic!("{year}-{month}-{day} is a date"))
        .0
}

/// A table's code and the regions column 9 lists, in the order of the line.
fn listed() -> Vec<(String, Vec<String>)> {
    holiday_tables("en")
        .lines()
        .map(|line| {
            let cells: Vec<&str> = line.split('\t').collect();
            assert_eq!(cells.len(), 14, "{line}");
            let regions = cells[8]
                .split(';')
                .filter(|code| !code.is_empty())
                .map(String::from)
                .collect();
            (String::from(cells[0]), regions)
        })
        .collect()
}

/// Days on which an unread weekend, a Friday weekend and the ordinary one
/// differ, in years the regional weekends of Malaysia and the Emirates have
/// changed in.
fn sample_days() -> Vec<i64> {
    let mut days = Vec::new();
    for (year, month, first) in [(2012, 5, 7), (2014, 1, 6), (2022, 1, 3), (2026, 3, 2)] {
        for offset in 0..7 {
            days.push(day(year, month, first + offset));
        }
    }
    days
}

#[test]
fn column_9_lists_the_regions_the_table_answers_for() {
    let tables = listed();
    assert_eq!(
        tables.len(),
        holiday_codes().lines().count(),
        "a line for every table"
    );
    assert!(tables.len() >= 195, "{}", tables.len());
    for (code, regions) in &tables {
        let set = rule_set(code).expect("a table");
        let answered = set.answered_regions();
        assert_eq!(regions, &answered, "{code}");
        // In code order, once each.
        let mut sorted = regions.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(regions, &sorted, "{code}");
        // The regions of the rules, of the weekend laws and of the
        // substitution policies, and nothing else.
        let mut expected: BTreeSet<&str> = set.region_codes().collect();
        for policy in set.weekend {
            expected.extend(policy.regions.iter().copied());
        }
        for policy in set.substitution {
            expected.extend(policy.regions.iter().copied());
        }
        assert_eq!(
            regions.iter().map(String::as_str).collect::<BTreeSet<_>>(),
            expected,
            "{code}"
        );
    }
    // The weekend-only regions are listed (front-end request 30).
    let row = |code: &str| -> &Vec<String> {
        &tables
            .iter()
            .find(|(table, _)| table == code)
            .expect("a table")
            .1
    };
    for region in ["MY-01", "MY-02", "MY-03", "MY-09", "MY-11"] {
        assert!(row("MY").iter().any(|listed| listed == region), "{region}");
    }
    assert!(row("AE").iter().any(|listed| listed == "AE-SH"));
}

#[test]
fn every_listed_region_is_accepted_by_every_export_that_takes_one() {
    let unknown = |answer: Result<(), Refusal>, what: &str| {
        assert_ne!(answer, Err(Refusal::Unknown), "{what}");
    };
    let days = sample_days();
    let mut checked = 0;
    for (code, regions) in listed() {
        for name in &regions {
            let region = Some(name.as_str());
            let what = |export: &str| format!("{export}({code}, {region:?})");
            unknown(
                holidays_in_year(&code, region, None, None, 2026).map(|_| ()),
                &what("hc_holidays_in_year"),
            );
            // The refusal of an unread weekend or a gap is not an unknown
            // region.
            for fixed in &days {
                unknown(
                    is_weekend(&code, region, *fixed).map(|_| ()),
                    &what("hc_holiday_is_weekend"),
                );
            }
            unknown(
                is_day_off(&code, region, None, day(2026, 3, 6)).map(|_| ()),
                &what("hc_holiday_is_day_off"),
            );
            unknown(
                holiday_beyond(&code, region, None, None, day(2026, 3, 6), true).map(|_| ()),
                &what("hc_holiday_next"),
            );
            unknown(
                holiday_beyond(&code, region, None, None, day(2026, 3, 6), false).map(|_| ()),
                &what("hc_holiday_previous"),
            );
            unknown(
                add_business_days(&code, region, None, day(2026, 3, 6), 1).map(|_| ()),
                &what("hc_holiday_add_business_days"),
            );
            unknown(
                business_days_between(&code, region, None, day(2026, 3, 2), day(2026, 3, 9))
                    .map(|_| ()),
                &what("hc_holiday_business_days_between"),
            );
            // A listed region has a name (`hc_place_name`).
            assert!(
                place_name(name, "en").is_ok(),
                "{name} has no line of hc_place_name"
            );
            checked += 1;
        }
    }
    assert!(checked > 250, "{checked}");
}

/// Whether a line of `hc_holidays_in_year` is the gap of a subdivision not
/// read, which any region the table keeps no days of has.
fn is_unread_gap(line: &str) -> bool {
    line.split('\t').nth(9) == Some("unread-subdivision")
}

#[test]
fn no_region_a_table_keeps_days_for_is_missing_from_the_list() {
    // Every region any table lists, every subdivision CLDR 48 has for the
    // table's country, and codes that are none: each is asked of every
    // table. A region that is accepted and not listed must be one with no
    // day of its own, the nationwide days and a weekend that is the
    // table's; a bogus one must be refused.
    let tables = listed();
    let all_listed: BTreeSet<&str> = tables
        .iter()
        .flat_map(|(_, regions)| regions.iter().map(String::as_str))
        .collect();
    let bogus = [
        "XX-01",
        "US-ZZ",
        "MY-99",
        "MY-1",
        "AE-XX",
        "JP garbage",
        "MY 02",
        "MY-02-1-1",
        "Kedah",
        "AE-SH-1-1",
        "MY-",
        "-",
        "ZZ",
        "0",
        "MY-02;MY-03",
    ];
    let days = sample_days();
    let mut accepted_unlisted = 0;
    for (code, regions) in &tables {
        let set = rule_set(code).expect("a table");
        let country = countries::by_code(code).map(|_| code.clone());
        let own: Vec<&str> = country
            .and_then(|country| place_names::subdivisions_of(&country))
            .into_iter()
            .flatten()
            .map(|place| place.code())
            .collect();
        let candidates: BTreeSet<&str> =
            all_listed.iter().copied().chain(own).chain(bogus).collect();
        let is_listed = |region: &str| {
            regions
                .iter()
                .any(|listed| listed.eq_ignore_ascii_case(region))
        };
        let nationwide: Vec<String> = [2025]
            .into_iter()
            .map(|year| holidays_in_year(code, None, None, None, year).expect("a table"))
            .collect();
        for region in candidates {
            let accepted = !matches!(
                is_weekend(code, Some(region), days[0]),
                Err(Refusal::Unknown)
            );
            if bogus.contains(&region) && !is_listed(region) {
                assert!(!accepted, "{code} accepts the bogus region {region:?}");
            }
            if is_listed(region) {
                assert!(accepted, "{code} lists {region} and refuses it");
                continue;
            }
            if !accepted {
                continue;
            }
            accepted_unlisted += 1;
            // No weekend law of its own.
            for fixed in &days {
                assert_eq!(
                    is_weekend(code, Some(region), *fixed),
                    is_weekend(code, None, *fixed),
                    "{code} keeps a weekend for {region} and does not list it"
                );
            }
            // No day of its own: the lines are the nationwide ones, less
            // the gap of a subdivision not read.
            for (year, expected) in [2025].into_iter().zip(&nationwide) {
                let text = holidays_in_year(code, Some(region), None, None, year)
                    .expect("an accepted region");
                let lines: Vec<&str> = text.lines().filter(|line| !is_unread_gap(line)).collect();
                let expected: Vec<&str> = expected.lines().collect();
                assert_eq!(
                    lines, expected,
                    "{code} keeps days for {region} in {year} and does not list it"
                );
            }
            // And the table's own set is the one that says so.
            assert!(
                !set.answered_regions()
                    .iter()
                    .any(|listed| listed.eq_ignore_ascii_case(region)),
                "{code} {region}"
            );
        }
    }
    // The CLDR subdivisions of the countries that were not read are
    // accepted, and answered with the nationwide days and a gap.
    assert!(accepted_unlisted > 100, "{accepted_unlisted}");
}
