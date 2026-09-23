//! Every calendar a holiday can be dated in must be one the registry has.
//!
//! The companion of `vocabulary.rs`, for the same reason and against the
//! same failure.
//!
//! `hc_holiday::CalendarSystem` is a struct of two function pointers and a
//! [`CalendarId`] — `Copy`, `const`, `static`-safe, and open, so a feast
//! kept in the Ethiopic, Coptic, Solar Hijri or Badíʿ calendar can be
//! written down in its own calendar without editing `hc-holiday`. A closed
//! enum would guarantee for free that every system names something real;
//! an open struct does not. This file is that guarantee, paid for
//! explicitly.

#![cfg(all(
    feature = "alloc",
    feature = "civil",
    feature = "lunar",
    feature = "regional",
    feature = "holiday",
))]

use std::collections::BTreeSet;

use hyper_calendar::hc_calendar::{CalendarId, Rd};
use hyper_calendar::hc_calendars_solar::gregorian;
use hyper_calendar::hc_holiday::{CalendarSystem, Rule, countries, traditions};

/// Every identifier the registry answers to.
fn registered() -> Vec<CalendarId> {
    hyper_calendar::registry()
        .metas()
        .map(|meta| meta.id)
        .collect()
}

#[test]
fn every_calendar_system_names_a_registered_calendar() {
    let registered = registered();
    let unknown: Vec<&str> = CalendarSystem::ALL
        .iter()
        .filter(|system| !registered.contains(&system.id))
        .map(|system| system.id.0)
        .collect();
    assert!(
        unknown.is_empty(),
        "holiday rules can be dated in calendars the registry does not have: {unknown:?}"
    );
}

/// Each system must round-trip a date through the calendar it names, or it
/// is not wired to what it claims.
#[test]
fn every_calendar_system_converts_both_ways() {
    for system in CalendarSystem::ALL {
        let today = gregorian::to_fixed(2024, 6, 15).expect("a real date");
        let year = system
            .year_containing(today)
            .unwrap_or_else(|| panic!("{} cannot place 2024-06-15", system.id.0));

        // The first day of that year must land back inside it.
        let first = system
            .to_fixed(year, hyper_calendar::hc_calendar::Month::regular(1), 1)
            .unwrap_or_else(|| panic!("{} has no first day of {year}", system.id.0));
        assert_eq!(
            system.year_containing(first),
            Some(year),
            "{} put its own new year in another year",
            system.id.0
        );
        assert!(system.covers_gregorian_year(2024), "{}", system.id.0);
    }
}

#[test]
fn the_identifiers_are_distinct() {
    let ids: BTreeSet<&str> = CalendarSystem::ALL.iter().map(|s| s.id.0).collect();
    assert_eq!(ids.len(), CalendarSystem::ALL.len());
}

/// The Ethiopian feasts are the proof that the set is open: ordinary fixed
/// dates in a calendar that could not be named at all before.
#[test]
fn a_feast_can_now_be_dated_in_the_ethiopic_calendar() {
    let table = traditions::by_code("ethiopian-orthodox").expect("the table is registered");
    let calendar = hyper_calendar::hc_holiday::HolidayCalendar::for_year(table, None, 2024);
    assert!(calendar.is_complete(), "{:?}", calendar.gaps());

    let names: Vec<&str> = calendar.all().iter().map(|day| day.name).collect();
    assert!(
        names.iter().any(|name| name.contains("Genna")),
        "expected Genna in 2024, got {names:?}"
    );

    // Genna is 29 Tahsas. In an ordinary year that is 7 January; in
    // Ethiopic 2016, which is a leap year, the whole year sits a day later
    // and it is the 8th. The arithmetic is cross-checked by Enkutatash
    // below, whose 12 September 2023 is widely reported.
    //
    // The *observance* is a separate fact and the table does not claim it:
    // Genna is kept on 7 January across most of Ethiopia even in those
    // years, with Lalibela the reported exception. See the table's own
    // documentation — a calendrical date and a kept date coming apart is
    // something to state, not to quietly resolve.
    let genna = calendar
        .all()
        .iter()
        .find(|day| day.name.contains("Genna"))
        .expect("Genna is in the table");
    assert_eq!(gregorian::from_fixed(genna.date), Ok((2024, 1, 8)));

    // Enkutatash is 1 Meskerem: 11 September, or the 12th in the same leap
    // years. Ethiopic 2017 begins on 11 September 2024.
    let new_year = calendar
        .all()
        .iter()
        .find(|day| day.name.contains("Enkutatash"))
        .expect("Enkutatash is in the table");
    assert_eq!(gregorian::from_fixed(new_year.date), Ok((2024, 9, 11)));
}

/// No country or tradition table may name a system the registry lacks,
/// including systems a caller built themselves.
#[test]
fn every_table_dates_its_holidays_in_a_registered_calendar() {
    fn systems(rule: &Rule, out: &mut Vec<CalendarId>) {
        match rule {
            Rule::FixedInCalendar { system, .. } => out.push(system.id),
            Rule::Offset { base, .. } => systems(base, out),
            _ => {}
        }
    }
    let registered = registered();
    let mut used: Vec<CalendarId> = Vec::new();
    for set in countries::ALL
        .iter()
        .copied()
        .chain(traditions::ALL.iter().copied())
    {
        for rule in set.rules {
            systems(&rule.rule, &mut used);
        }
    }
    let unknown: BTreeSet<&str> = used
        .iter()
        .filter(|id| !registered.contains(id))
        .map(|id| id.0)
        .collect();
    assert!(unknown.is_empty(), "{unknown:?}");
    assert!(!used.is_empty(), "no table dates anything in a calendar?");
    let _ = Rd(0);
}
