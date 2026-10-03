//! The first year of the exchanges, the international days, the traditions
//! and four national tables, and the gap before it (ADR 0013).
//!
//! Each row gives a table's first year and the source `docs/systems/
//! holiday-first-years.md` names for it. Before the first year the table
//! answers no day and reports a gap; from it the table answers.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::countries;
use hc_holiday::engine::{HolidayCalendar, Unanswered};
use hc_holiday::international::{UNITED_NATIONS, UNITED_NATIONS_WEEKS};
use hc_holiday::rule::RuleSet;
use hc_holiday::{exchanges, traditions};

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

fn names(calendar: &HolidayCalendar<'_>, year: i64) -> Vec<&'static str> {
    calendar.in_year(year).iter().map(|day| day.name).collect()
}

fn gap_names(calendar: &HolidayCalendar<'_>) -> Vec<&'static str> {
    calendar.holiday_gaps().map(|gap| gap.name).collect()
}

/// The exchanges and the first year of the lists each is read from.
const EXCHANGES: &[(&str, i64)] = &[
    ("BVMF", 2021),
    ("MISX", 2023),
    ("XAMS", 2021),
    ("XASX", 2026),
    ("XBKK", 2022),
    ("XBOM", 2020),
    ("XBRU", 2021),
    ("XCSE", 2025),
    ("XDUB", 2021),
    ("XETR", 2026),
    ("XHEL", 2025),
    ("XHKG", 2026),
    ("XICE", 2025),
    ("XIDX", 2020),
    ("XIST", 2019),
    ("XJPX", 2026),
    ("XJSE", 2024),
    ("XKLS", 2020),
    ("XKRX", 2009),
    ("XLIS", 2021),
    ("XLON", 2026),
    ("XMAD", 2023),
    ("XMEX", 2019),
    ("XMIL", 2021),
    ("XNAS", 2026),
    ("XNSE", 2020),
    ("XNYS", 2026),
    ("XNZE", 2021),
    ("XOSL", 2021),
    ("XPAR", 2021),
    ("XPHS", 2020),
    ("XSAU", 2023),
    ("XSES", 2020),
    ("XSHE", 2015),
    ("XSHG", 2014),
    ("XSTO", 2025),
    ("XSWX", 2026),
    ("XTAE", 2024),
    ("XTAI", 2023),
    ("XTSE", 2025),
    ("XWAR", 2019),
    ("XWBO", 2019),
];

#[test]
fn an_exchange_answers_from_its_first_year_and_reports_a_gap_before_it() {
    assert_eq!(EXCHANGES.len(), exchanges::ALL.len());
    for &(code, first) in EXCHANGES {
        let Some(table) = exchanges::by_code(code) else {
            panic!("no exchange {code}");
        };
        for year in [1700, first - 10] {
            let calendar = HolidayCalendar::for_year(table, None, year);
            assert!(calendar.in_year(year).is_empty(), "{code} {year}");
            assert!(!calendar.is_complete(), "{code} {year}: no gap");
        }
        let calendar = HolidayCalendar::for_year(table, None, first);
        assert!(!calendar.in_year(first).is_empty(), "{code} {first}");
    }
}

#[test]
fn the_nyse_s_closures_are_read_in_their_own_years_and_its_calendar_from_2026() {
    let nyse = &exchanges::NEW_YORK_STOCK_EXCHANGE;
    // 5 December 2018, the day of President Bush's funeral: the one closure
    // read before 2026.
    let calendar = HolidayCalendar::for_year(nyse, None, 2018);
    assert!(calendar.is_holiday(ymd(2018, 12, 5)));
    assert_eq!(calendar.in_year(2018).len(), 1);
    // The rest of 2018 is a gap, and a Fourth of July is refused, not
    // answered as the closed day it was.
    assert_eq!(
        calendar.day_off(ymd(2018, 7, 4)),
        Err(Unanswered::Gap),
        "the calendar of 2018 was not read"
    );
    // Juneteenth, a market holiday from 2022, is closed in 2026.
    let calendar = HolidayCalendar::for_year(nyse, None, 2026);
    assert!(calendar.is_holiday(ymd(2026, 6, 19)));
}

#[test]
fn an_exchange_that_keeps_the_days_of_its_country_reads_them_from_its_own_first_year() {
    // Tokyo closes on Japan's days from 2026; before it Japan's own days are
    // not carried in, and the engine says so.
    let jpx = &exchanges::TOKYO_STOCK_EXCHANGE;
    let calendar = HolidayCalendar::for_year(jpx, None, 2000);
    assert!(calendar.in_year(2000).is_empty());
    assert!(
        calendar
            .holiday_gaps()
            .any(|gap| gap.name == hc_holiday::rule::UNREAD_INCLUDED)
    );
    let calendar = HolidayCalendar::for_year(jpx, None, 2026);
    assert!(calendar.is_holiday(ymd(2026, 1, 1)));
    assert!(
        calendar
            .holiday_gaps()
            .all(|gap| gap.name != hc_holiday::rule::UNREAD_INCLUDED)
    );
}

#[test]
fn every_international_day_and_week_begins_in_a_year_a_source_reads() {
    for set in [&UNITED_NATIONS, &UNITED_NATIONS_WEEKS] {
        for rule in set.rules {
            let Some(from) = rule.valid_from else {
                panic!("{} has no first year", rule.name);
            };
            // The earliest is International Women's Day, observed from 1914;
            // every other day is the United Nations'.
            assert!(from >= 1914, "{}: {from}", rule.name);
            if let Some(read) = rule.read_from {
                assert!(
                    read > from,
                    "{}: read from {read}, begins {from}",
                    rule.name
                );
            }
        }
        for year in [1, 1900] {
            let calendar = HolidayCalendar::for_year(set, None, year);
            assert!(calendar.in_year(year).is_empty(), "{} {year}", set.code);
            assert!(calendar.is_complete(), "{} {year}: no gap", set.code);
        }
        // The United Nations began in 1945: nothing of theirs before.
        let calendar = HolidayCalendar::for_year(set, None, 1944);
        assert!(calendar.in_year(1944).len() <= 1, "{}", set.code);
    }
}

#[test]
fn an_international_day_is_absent_before_it_began_and_a_gap_until_it_was_read() {
    // World Braille Day, A/RES/73/161: first observed on 4 January 2019.
    let braille = |year| HolidayCalendar::for_year(&UNITED_NATIONS, None, year);
    assert!(!names(&braille(2018), 2018).contains(&"World Braille Day"));
    assert!(!gap_names(&braille(2018)).contains(&"World Braille Day"));
    assert!(names(&braille(2019), 2019).contains(&"World Braille Day"));
    // The International Day of Clean Energy, A/RES/77/327, adopted in the
    // session that began in 2022: the day's first observance was not read, so
    // 2022 and 2023 are gaps and it is answered from 2024.
    for year in [2022, 2023] {
        let calendar = braille(year);
        assert!(!names(&calendar, year).contains(&"International Day of Clean Energy"));
        assert!(gap_names(&calendar).contains(&"International Day of Clean Energy"));
    }
    assert!(names(&braille(2024), 2024).contains(&"International Day of Clean Energy"));
    // Before 2022 it is absent, not a gap.
    assert!(!gap_names(&braille(2021)).contains(&"International Day of Clean Energy"));
}

/// The traditions whose table is read from a year, and why: the calendar or
/// instrument that year comes from.
const TRADITIONS: &[(&str, i64)] = &[
    ("christian-western", 1583),
    ("christian-orthodox", 326),
    ("christian-orthodox-revised-julian", 1925),
    ("christian-armenian", 1583),
    ("christian-armenian-jerusalem", 326),
    ("ethiopian-orthodox", 401),
    ("coptic-orthodox", 326),
    ("jewish", 359),
    ("shinto", 1873),
    ("kyuchu-saishi", 2020),
    ("plough-days", 1753),
    ("unlucky-fridays", 2000),
    ("sacred-wednesdays", 2000),
    ("balinese-pawukon-days", 2000),
    ("yazidi", 1901),
    ("qumran-festivals", -133),
    ("name-days-greek-movable", 1925),
    ("name-days-bulgarian-movable", 2010),
];

/// The tables that answer from year 1 for a reason of their own: their
/// days are solar terms and the table says it computes them for any year.
const COMPUTED_FROM_THE_SUN: &[&str] = &["chinese-folk", "korean-folk"];

#[test]
fn a_tradition_answers_from_its_first_year_and_reports_a_gap_before_it() {
    for &(code, first) in TRADITIONS {
        let Some(table) = traditions::by_code(code) else {
            panic!("no table {code}");
        };
        let mut years = vec![-1000, -500, 1, 300, 1000, 1500, first - 1];
        years.retain(|&year| year < first);
        years.sort_unstable();
        years.dedup();
        for year in years {
            let calendar = HolidayCalendar::for_year(table, None, year);
            assert!(calendar.in_year(year).is_empty(), "{code} {year}");
            assert!(!calendar.is_complete(), "{code} {year}: no gap");
        }
        let calendar = HolidayCalendar::for_year(table, None, first);
        assert!(!calendar.in_year(first).is_empty(), "{code} {first}");
    }
}

#[test]
fn no_other_tradition_answers_before_the_year_300() {
    for table in traditions::ALL {
        if TRADITIONS.iter().any(|(code, _)| *code == table.code)
            || COMPUTED_FROM_THE_SUN.contains(&table.code)
        {
            continue;
        }
        for year in [-500, 1, 200] {
            let calendar = HolidayCalendar::for_year(table, None, year);
            assert!(calendar.in_year(year).is_empty(), "{} {year}", table.code);
        }
    }
}

fn country(code: &str) -> &'static RuleSet {
    match countries::by_code(code) {
        Some(table) => table,
        None => panic!("no country {code}"),
    }
}

#[test]
fn the_united_states_has_no_thanksgiving_before_1870_and_a_gap_to_1941() {
    let us = country("US");
    // Before the Act of 28 June 1870 there was no federal holiday: no day,
    // no gap.
    let calendar = HolidayCalendar::for_year(us, None, 1869);
    assert!(calendar.in_year(1869).is_empty());
    assert!(calendar.is_complete());
    // In 1900 the President named the day each year, and the proclamations
    // were not read: a gap, beside the other six federal days of the year.
    let calendar = HolidayCalendar::for_year(us, None, 1900);
    assert_eq!(
        names(&calendar, 1900),
        [
            "New Year's Day",
            "Washington's Birthday",
            "Memorial Day",
            "Independence Day",
            "Labor Day",
            "Christmas Day"
        ]
    );
    assert_eq!(gap_names(&calendar), ["Thanksgiving Day"]);
    // The Joint Resolution of 1941 fixed the fourth Thursday from 1942.
    let calendar = HolidayCalendar::for_year(us, None, 1942);
    assert!(calendar.is_holiday(ymd(1942, 11, 26)));
    assert!(calendar.is_complete());
    // Armistice Day, 11 November, from 1938 to 1953.
    for (year, kept) in [(1937, false), (1938, true), (1953, true), (1954, false)] {
        let calendar = HolidayCalendar::for_year(us, None, year);
        assert_eq!(
            names(&calendar, year).contains(&"Armistice Day"),
            kept,
            "{year}"
        );
    }
    let calendar = HolidayCalendar::for_year(us, None, 1954);
    assert!(names(&calendar, 1954).contains(&"Veterans Day"));
}

#[test]
fn japan_has_a_gap_for_every_year_to_1948_and_none_from_1949() {
    let jp = country("JP");
    for year in [1000, 1900, 1947] {
        let calendar = HolidayCalendar::for_year(jp, None, year);
        assert!(calendar.in_year(year).is_empty(), "{year}");
        assert_eq!(
            gap_names(&calendar),
            ["Holidays before the Public Holidays Act"],
            "{year}"
        );
        // The weekend is read from 1992, so a day before it is refused for
        // that reason first, and never answered.
        assert!(calendar.day_off(ymd(year, 1, 1)).is_err(), "{year}");
    }
    // 1948 began under the old ordinance: the Act took effect on 20 July and
    // three of its days fall after it.
    let calendar = HolidayCalendar::for_year(jp, None, 1948);
    assert_eq!(
        names(&calendar, 1948),
        [
            "Autumnal Equinox Day",
            "Culture Day",
            "Labour Thanksgiving Day"
        ]
    );
    assert_eq!(gap_names(&calendar).len(), 1);
    let calendar = HolidayCalendar::for_year(jp, None, 1949);
    assert!(calendar.is_complete());
    assert!(names(&calendar, 1949).contains(&"New Year's Day"));
}

#[test]
fn canada_reads_the_federal_days_from_the_code_of_1985() {
    let ca = country("CA");
    let calendar = HolidayCalendar::for_year(ca, None, 1984);
    assert!(calendar.in_year(1984).is_empty());
    assert_eq!(gap_names(&calendar).len(), 9, "{:?}", gap_names(&calendar));
    assert!(gap_names(&calendar).contains(&"Canada Day"));
    // The National Day for Truth and Reconciliation is absent before 2021,
    // not a gap.
    assert!(!gap_names(&calendar).contains(&"National Day for Truth and Reconciliation"));
    let calendar = HolidayCalendar::for_year(ca, None, 1985);
    assert_eq!(names(&calendar, 1985).len(), 9);
    assert!(calendar.is_complete());
    let calendar = HolidayCalendar::for_year(ca, None, 2021);
    assert!(names(&calendar, 2021).contains(&"National Day for Truth and Reconciliation"));
}

#[test]
fn bosnia_and_herzegovina_has_no_nationwide_list_and_each_entity_its_own() {
    let ba = country("BA");
    // Asked for no entity, the table has no day and says so in every year.
    let calendar = HolidayCalendar::for_year(ba, None, 2026);
    assert!(calendar.in_year(2026).is_empty());
    assert_eq!(gap_names(&calendar), ["Holidays of the entities"]);
    assert_eq!(calendar.day_off(ymd(2026, 1, 1)), Err(Unanswered::Gap));
    // Asked for an entity, the list is the entity's and there is no such gap.
    let calendar = HolidayCalendar::for_year(ba, Some("BA-SRP"), 2026);
    assert!(
        calendar
            .holiday_gaps()
            .all(|gap| gap.name != "Holidays of the entities")
    );
    assert_eq!(calendar.day_off(ymd(2026, 1, 1)), Ok(true));
}
