//! The first year each table of the Middle East, Africa and Oceania answers
//! for (ADR 0013, audit 10 a1).
//!
//! A table's rules that have no establishment of their own are carried from
//! the earliest year its sources support: the year of the oldest dated
//! instrument or list the table cites for its days. Before it the engine
//! reports a gap, never `exact`; and long before any source — the year 1700,
//! when none of these states or laws existed — it answers with no day at
//! all. The instrument each year comes from is the second column of
//! [`FIRST_YEARS`]; where no instrument with a date was read, the year is
//! that of the dated list read (2026), and the doc of the table says so.

use hc_holiday::countries;
use hc_holiday::engine::HolidayCalendar;
use hc_holiday::rule::Rule;

/// `(code, first year, the source that year comes from)`.
const FIRST_YEARS: &[(&str, i64, &str)] = &[
    (
        "IL",
        1951,
        "the Hours of Work and Rest Law of 1951, sections 2(b) and 7, the oldest text read for the days of rest",
    ),
    (
        "IR",
        2026,
        "the lists of Wikipedia, \"Public holidays in Iran\", retrieved 2026-09-22, the only list read; no instrument with a date was read",
    ),
    (
        "SA",
        2025,
        "the Labour Law's executive regulation, article 24, as published in April 2025; the regulation's own date was not read",
    ),
    (
        "AE",
        2025,
        "Cabinet Resolution No. 27 of 2024, in force from 1 January 2025; the Resolution of 2019 it repealed was not read",
    ),
    (
        "TR",
        1981,
        "Law 2429 of 17 March 1981, whose text and amendments were read",
    ),
    (
        "EG",
        2026,
        "Labour Law No. 14 of 2025, in force from 1 September 2025, so 2026 is its first whole year",
    ),
    (
        "NG",
        2004,
        "the Laws of the Federation of Nigeria 2004 text of the Public Holidays Act, Cap. P40, the edition read",
    ),
    (
        "ZA",
        1995,
        "the Public Holidays Act 36 of 1994, in force 1 January 1995",
    ),
    (
        "KE",
        2022,
        "the Revised Edition 2022 of the Public Holidays Act, Cap. 110, the edition read",
    ),
    (
        "MA",
        1977,
        "décret n° 2-77-169 of 28 February 1977, which the later décrets amend",
    ),
    ("ET", 2024, "Proclamation No. 1334/2024 of 14 August 2024"),
    (
        "GH",
        2019,
        "the 2019 list of public holidays, the oldest list read",
    ),
    ("BH", 2012, "Law 36 of 2012, article 64"),
    (
        "JO",
        2007,
        "the Prime Minister's Official Bulletin No. 6 of 2007",
    ),
    ("KW", 2010, "Law 6 of 2010, article 68"),
    ("LB", 2005, "Decree 15215 of 27 September 2005"),
    (
        "TZ",
        2026,
        "the 2026 list, the only dated list read; the Act's own date was not read",
    ),
    (
        "UG",
        2026,
        "the 2026 list of the Consulate in Arusha, the only dated list read; no Act was read",
    ),
    (
        "ZM",
        2026,
        "HONO's 2026 list, the only dated list read; the Act's own date was not read",
    ),
    (
        "ZW",
        2026,
        "General Notice 1361 of 2025, the 2026 list; the Act's own date was not read",
    ),
    (
        "DZ",
        1963,
        "law 63-278 of 26 July 1963, articles 1, 3 and 4",
    ),
    (
        "TN",
        1961,
        "décret 61-144 of 1961 and the decrees after it, as the French Wikipedia chronicles them",
    ),
    ("SN", 1974, "loi n° 74-52 of 4 November 1974"),
    ("CI", 1996, "décret n° 96-205 of 7 March 1996"),
    ("BJ", 1990, "loi n° 90-019 of 27 July 1990"),
    ("BF", 2015, "loi n° 079-2015/CNT"),
    ("CV", 1991, "Lei n.º 16/IV/91 of 30 December 1991"),
    ("GN", 2022, "décret D/2022/0526 of 2 November 2022"),
    ("ML", 2005, "loi n° 05-040 of 22 July 2005"),
    (
        "OM",
        2020,
        "Royal Decree 56/2020, read with Royal Decree 88/2022",
    ),
    (
        "QA",
        2025,
        "Emiri Decision No. 57 of 2025; Cabinet Decision No. 6 of 2008, which a decision of 2025 amends, was read only as amended",
    ),
    ("IQ", 2024, "Official Holidays Law No. 12 of 2024"),
    (
        "BW",
        2006,
        "the Public Holidays Act, Cap. 03:07 (Act 17 of 2006)",
    ),
    ("NA", 1990, "the Public Holidays Act 26 of 1990"),
    (
        "MU",
        2017,
        "the Prime Minister's Office's General Notice No. 814 of 2016 for the public holidays of 2017, the oldest dated list read; the Act (22 of 1968) was read as amended to 2019",
    ),
    (
        "MW",
        2026,
        "the Public Holidays Act Cap. 18:05 and the 2026 list; the Act's own date was not read",
    ),
    ("SY", 2025, "Decree No. 188 of 2025"),
    ("PS", 2025, "the Council of Ministers' 2025 tables"),
    ("LY", 2012, "Law No. 5 of 2012"),
    ("YE", 2000, "Law No. 2 of 2000"),
    (
        "CM",
        1976,
        "loi n° 73/5 of 7 December 1973 as loi n° 76/8 of 8 July 1976 amended it",
    ),
    ("CG", 1994, "loi n° 2-94 of 1 March 1994"),
    ("CD", 2014, "ordonnance n° 14/010 of 14 May 2014"),
    ("AO", 2011, "Lei n.º 10/11 of 16 February 2011"),
    (
        "RW",
        2017,
        "Presidential Order n° 54/01 of 24 February 2017",
    ),
    (
        "BI",
        2021,
        "décret n° 100/150 of 7 June 2021, which amends the décret of 2006",
    ),
    (
        "MG",
        2023,
        "the décret of 4 January 2023, the oldest of the yearly décrets read",
    ),
    (
        "SC",
        1991,
        "the Public Holidays Act, Cap. 190, 1991 edition",
    ),
    (
        "MZ",
        2023,
        "Lei n.º 13/2023 of 25 August 2023; the table under Lei n.º 23/2007 was read only as a secondary source reports it",
    ),
    (
        "LS",
        1996,
        "the Public Holidays Act 1995 (Act No. 7 of 1995), in its first year, 1996",
    ),
    ("TD", 1997, "décret n° 97-413 of 30 September 1997"),
    ("MR", 1992, "loi n° 92-018 of 7 December 1992"),
    (
        "DJ",
        1978,
        "arrêté n° 77-347/PR/MI of 4 October 1977 and the arrêtés that rectify it, from 1978, its first full year",
    ),
    (
        "KM",
        2026,
        "décret n° 25-147 of 19 December 2025, from 2026, its first full year",
    ),
    ("GQ", 2007, "Decreto núm. 9/2007 of 5 February 2007"),
    (
        "LR",
        2012,
        "the President's holiday proclamations of 2012 to 2026, the oldest dated list read",
    ),
    (
        "SO",
        2025,
        "Law No. 36 of 24 December 2024, the Labour Code of 2024, from 2025",
    ),
    (
        "SS",
        2022,
        "the Labour Act, 2017 (Act No. 64), read with the Ministry of Labour's calendar for 2022, from which the days are carried",
    ),
    (
        "SD",
        2025,
        "the Council of Ministers' announcements of December 2025 to August 2026",
    ),
    (
        "GW",
        2023,
        "Decreto n.º 1/2023 of 18 January 2023, read as a newspaper quotes it",
    ),
    (
        "SL",
        2020,
        "the Government Notices of 2020, the oldest dated list read; the Act of 1960 was read as an undated revised-edition copy",
    ),
    (
        "GM",
        2021,
        "the Office of the President's declarations of 2021 to 2026",
    ),
    (
        "SZ",
        1998,
        "EswatiniLII's consolidation of the Public Holidays Act 1938 as at 1 December 1998",
    ),
    ("TG", 1987, "loi n° 87-08 of 9 June 1987"),
    (
        "NE",
        2023,
        "the Council of Ministers' text of 2023 restoring 3 August, the oldest dated source read; loi n° 97-20 of 1997 was not read",
    ),
    (
        "GA",
        2024,
        "the Ministry of Labour's communiqué of August 2024, the oldest dated source read; décret n° 00727 of 1998 was not read",
    ),
    (
        "AU",
        2000,
        "the Holidays Act 1983 (Qld) and the Statutory Holidays Act 2000 (Tas), the two Acts read, the later of which began in 2000, for the days every state keeps",
    ),
    (
        "NZ",
        2010,
        "Employment New Zealand's list of 2010 to 2025, the oldest dated list read; the Acts were not read",
    ),
    (
        "FM",
        2014,
        "the Code of the Federated States of Micronesia (2014), title 1, chapter 6",
    ),
    ("MH", 1988, "the Public Holidays Act 1988"),
    (
        "NR",
        2023,
        "the Government Gazette of 30 December 2022, the list for 2023, the oldest dated list read",
    ),
    (
        "PW",
        2026,
        "the Palau National Code Annotated, Supp. 12, whose date was not read, retrieved 2026-09-23",
    ),
    (
        "PG",
        1982,
        "the 1982 revised edition of the Public Holidays Act 1953 (Chapter 321)",
    ),
    (
        "SB",
        1996,
        "the 1996 edition of the Public Holidays Act (Cap. 151)",
    ),
    (
        "TO",
        2020,
        "the Public Holidays Act, Chapter 8.11, 2020 Revised Edition",
    ),
    (
        "TV",
        2022,
        "the Public Holidays Act, Cap. 4.50, 2022 Revised Edition",
    ),
    (
        "VU",
        2006,
        "the Public Holidays Act [Cap. 114], Consolidated Edition 2006",
    ),
    (
        "WS",
        2023,
        "the Public Holidays Act 2008 as revised to 31 December 2023",
    ),
    (
        "FJ",
        1985,
        "the Public Holidays Act (Cap. 101), 1985 edition",
    ),
    (
        "KI",
        1977,
        "the Public Holidays Ordinance (Cap. 81), 1977 revised edition",
    ),
];

/// Tables whose every rule is established in a year of its own, so that no
/// rule needs a first year read: Tunisia's, from décret 61-144 of 1961.
const ALL_ESTABLISHED: &[&str] = &["TN"];

/// Tables that name no day of their first year: Togo's days of the law of
/// 1987 are each a gap, their dates for the years before 2024 not read.
const NO_DAY_AT_FIRST_YEAR: &[&str] = &["TG"];

#[test]
fn every_table_is_a_gap_before_its_first_year_and_answers_from_it() {
    assert_eq!(FIRST_YEARS.len(), 80);
    let mut failures: Vec<String> = Vec::new();
    for &(code, first, source) in FIRST_YEARS {
        let table = countries::by_code(code).unwrap_or_else(|| panic!("{code} is not a country"));
        let at = HolidayCalendar::for_year(table, None, first);
        if at.in_year(first).is_empty() && !NO_DAY_AT_FIRST_YEAR.contains(&code) {
            failures.push(format!("{code} {first} ({source}) has no day"));
        }
        // Long before any source, and before any of these states: no day,
        // and a gap wherever a rule's establishment does not rule it out.
        let long_before = HolidayCalendar::for_year(table, None, 1700);
        if !long_before.in_year(1700).is_empty() {
            failures.push(format!("{code} answers for 1700"));
        }
        if ALL_ESTABLISHED.contains(&code) {
            continue;
        }
        let before = HolidayCalendar::for_year(table, None, first - 1);
        if long_before.gaps().is_empty() {
            failures.push(format!("{code} 1700 is not a gap ({source})"));
        }
        if before.gaps().is_empty() {
            failures.push(format!("{code} {} is not a gap ({source})", first - 1));
        }
        if before.gaps().len() <= at.gaps().len() && !NO_DAY_AT_FIRST_YEAR.contains(&code) {
            failures.push(format!("{code}: {first} has as many gaps as {}", first - 1));
        }
        // The year before is never a complete answer.
        if before.is_complete() {
            failures.push(format!("{code} {} is complete", first - 1));
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn a_table_whose_every_rule_is_established_has_no_gap_before_its_first_year() {
    let tunisia = countries::by_code("TN").unwrap_or_else(|| panic!("no TN"));
    for year in [1700, 1960] {
        let calendar = HolidayCalendar::for_year(tunisia, None, year);
        assert!(calendar.in_year(year).is_empty(), "TN {year}");
        assert!(calendar.is_complete(), "TN {year}");
    }
    assert!(
        !HolidayCalendar::for_year(tunisia, None, 1961)
            .in_year(1961)
            .is_empty()
    );
}

#[test]
fn a_day_established_before_the_first_year_is_a_gap_from_its_establishment() {
    let gap_names = |code: &str, year: i64| -> Vec<&'static str> {
        let table = countries::by_code(code).unwrap_or_else(|| panic!("no {code}"));
        HolidayCalendar::for_year(table, None, year)
            .gaps()
            .iter()
            .map(|gap| gap.name)
            .collect()
    };
    let day_names = |code: &str, year: i64| -> Vec<&'static str> {
        let table = countries::by_code(code).unwrap_or_else(|| panic!("no {code}"));
        HolidayCalendar::for_year(table, None, year)
            .in_year(year)
            .iter()
            .map(|holiday| holiday.name)
            .collect()
    };
    // Saudi Arabia: Founding Day by Royal Order A/371 of 27 January 2022, in
    // a table read from 2025. Before 2022 the day is absent, from 2022 to
    // 2024 it is a gap (the regulation of 2025 was not read for those
    // years), from 2025 it is a day.
    assert!(!gap_names("SA", 2021).contains(&"Founding Day"));
    assert!(!day_names("SA", 2021).contains(&"Founding Day"));
    assert!(gap_names("SA", 2022).contains(&"Founding Day"));
    assert!(gap_names("SA", 2024).contains(&"Founding Day"));
    assert!(day_names("SA", 2025).contains(&"Founding Day"));
    // Israel: Independence Day by the law of 1949, two years before the
    // first text of the days of rest read (1951).
    assert!(!gap_names("IL", 1948).contains(&"Yom HaAtzmaut"));
    assert!(gap_names("IL", 1949).contains(&"Yom HaAtzmaut"));
    assert!(day_names("IL", 1951).contains(&"Yom HaAtzmaut"));
}

#[test]
fn australia_is_carried_from_2000_and_a_state_s_own_days_from_2026() {
    let au = countries::by_code("AU").unwrap_or_else(|| panic!("no AU"));
    // The two Acts read, the Holidays Act 1983 (Qld) and the Statutory
    // Holidays Act 2000 (Tas), give the days every state keeps.
    assert!(!HolidayCalendar::for_year(au, None, 1999).gaps().is_empty());
    assert!(HolidayCalendar::for_year(au, None, 2000).gaps().is_empty());
    // A state's own day rests on a list of 2026 and on sources that are not
    // the state's Act, so an earlier year is a gap, not an answer.
    let vic_2025 = HolidayCalendar::for_year(au, Some("AU-VIC"), 2025);
    assert!(!vic_2025.gaps().is_empty());
    assert!(
        !vic_2025
            .in_year(2025)
            .iter()
            .any(|holiday| holiday.name == "Melbourne Cup Day")
    );
    let vic_2026 = HolidayCalendar::for_year(au, Some("AU-VIC"), 2026);
    assert!(
        vic_2026
            .in_year(2026)
            .iter()
            .any(|holiday| holiday.name == "Melbourne Cup Day")
    );
}

#[test]
fn no_rule_of_these_tables_is_unread_by_accident() {
    // A day answered for 2026 in every table is not a gap by a `read_from`
    // later than the retrieval year.
    for &(code, first, _) in FIRST_YEARS {
        assert!(first <= 2026, "{code}");
    }
    let _ = Rule::UNREAD;
}

#[test]
fn no_rule_answers_every_year_without_an_establishment_or_a_first_year_read() {
    // The sweep above sees only the tables in which every rule is silent.
    // This one looks at each rule: it names the year it was established, or
    // the year it was first read, and that is not earlier than the table's.
    let mut failures: Vec<String> = Vec::new();
    for &(code, first, _) in FIRST_YEARS {
        let table = countries::by_code(code).unwrap_or_else(|| panic!("{code} is not a country"));
        for rule in table.rules {
            let first_read = rule.read_from.map(i64::from);
            let established = rule.valid_from.map(i64::from);
            // A day listed for the years read only reports a gap in every
            // other year by itself.
            if matches!(rule.rule, Rule::Listed { .. }) {
                continue;
            }
            match (established, first_read) {
                (None, None) => failures.push(format!("{code} {}: neither", rule.name)),
                (_, Some(read)) if read < first => {
                    failures.push(format!("{code} {}: read from {read}", rule.name));
                }
                _ => {}
            }
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}
