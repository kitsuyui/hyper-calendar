//! The ranges the two boundary READMEs state for the sky and the Hindu
//! exports are the ranges the exports answer for.
//!
//! The "Answers for" tables of `hyper-calendar-wasm/README.md` and
//! `hyper-calendar-ffi/README.md` give each export's accepted input as a
//! range of POSIX seconds. A range written by hand drifts from the code: four
//! Hindu exports stated 0000-01-01 to 4926-09-24 for the years −1000 to 3000.
//! This reads the numbers out of the rows and probes each export at both ends
//! of the range and one second outside it.

#![cfg(all(feature = "astro", feature = "indic", feature = "jupiter"))]

use hyper_calendar::astro_lines::{EARLIEST_YEAR, LATEST_YEAR};
use hyper_calendar::boundary::Refusal;
use hyper_calendar::hc_calendar::gregorian::new_year;
use hyper_calendar::jupiter_lines::station_lines;
use hyper_calendar::panchanga_lines::{
    ayanamsa_at_line, ayanamsa_from_anchor_line, tithi_at_lines,
};

const READMES: [&str; 2] = [
    "../hyper-calendar-wasm/README.md",
    "../hyper-calendar-ffi/README.md",
];

/// Seconds in a day.
const DAY: i64 = 86_400;

/// The first and the last POSIX second of the years the sky exports answer for:
/// the start of `EARLIEST_YEAR` and the second before the start of the year
/// after `LATEST_YEAR`.
fn era() -> (i64, i64) {
    let unix_day =
        |year: i64| new_year(year).0 - hyper_calendar::hc_calendar::fixed::RD_OF_UNIX_EPOCH;
    (
        unix_day(EARLIEST_YEAR) * DAY,
        unix_day(LATEST_YEAR + 1) * DAY - 1,
    )
}

/// The integers a text writes with a space between groups of three digits
/// and U+2212 for a minus, in order.
fn integers(text: &str) -> Vec<i64> {
    let chars: Vec<char> = text.chars().collect();
    let mut found = Vec::new();
    let mut at = 0;
    while at < chars.len() {
        if !chars[at].is_ascii_digit() {
            at += 1;
            continue;
        }
        let negative = at > 0 && matches!(chars[at - 1], '\u{2212}' | '-');
        let mut value: i64 = 0;
        loop {
            while at < chars.len() && chars[at].is_ascii_digit() {
                value = value * 10 + i64::from(chars[at] as u8 - b'0');
                at += 1;
            }
            let grouped = at + 4 <= chars.len()
                && chars[at].is_whitespace()
                && chars[at + 1..at + 4].iter().all(char::is_ascii_digit)
                && chars.get(at + 4).is_none_or(|next| !next.is_ascii_digit());
            if grouped {
                at += 1;
            } else {
                break;
            }
        }
        found.push(if negative { -value } else { value });
    }
    found
}

/// The text of the row of the range table whose export cell names `export`.
fn row(readme: &str, export: &str) -> String {
    let marker = format!("`{export}`");
    readme
        .lines()
        .find(|line| {
            line.starts_with('|')
                && line
                    .split('|')
                    .nth(2)
                    .is_some_and(|cell| cell.contains(&marker))
                && line
                    .split('|')
                    .nth(3)
                    .is_some_and(|cell| cell.contains("through") || cell.contains("up to"))
        })
        .unwrap_or_else(|| panic!("no range row for {export}"))
        .to_owned()
}

/// The integers of a row's range cell that follow `after`.
fn range_after(row: &str, after: &str) -> Vec<i64> {
    let cell = row
        .split('|')
        .nth(3)
        .unwrap_or_else(|| panic!("no range cell in {row}"));
    let at = cell
        .find(after)
        .unwrap_or_else(|| panic!("no {after} in {cell}"));
    integers(&cell[at + after.len()..])
}

#[test]
fn the_integers_are_read_as_the_readmes_write_them() {
    assert_eq!(
        integers("−93 724 128 000 through 32 535 215 999, the years −1000 to 3000"),
        [-93_724_128_000, 32_535_215_999, -1000, 3000]
    );
}

#[test]
fn the_instant_ranges_of_the_hindu_exports_are_the_years_they_name() {
    let (first, last) = era();
    assert_eq!((first, last), (-93_724_128_000, 32_535_215_999));
    for readme in READMES {
        let text =
            std::fs::read_to_string(readme).unwrap_or_else(|error| panic!("{readme}: {error}"));
        let row = row(&text, "hc_tithi_at");
        assert!(row.contains("`hc_ayanamsa_at`") && row.contains("`hc_ayanamsa_from_anchor`"));
        let stated = range_after(&row, "`unix_seconds`");
        assert_eq!(&stated[..2], [first, last], "{readme}: {row}");

        for unix in [first, last] {
            assert!(
                tithi_at_lines(unix, "true").is_ok(),
                "{readme} hc_tithi_at {unix}"
            );
            assert!(
                ayanamsa_at_line(unix, "lahiri").is_ok(),
                "{readme} hc_ayanamsa_at {unix}"
            );
            assert!(
                ayanamsa_from_anchor_line(unix, 2_435_553.5, 23.25).is_ok(),
                "{readme} hc_ayanamsa_from_anchor {unix}"
            );
        }
        for unix in [first - 1, last + 1] {
            assert_eq!(
                tithi_at_lines(unix, "true"),
                Err(Refusal::OutOfRange),
                "{readme} {unix}"
            );
            assert_eq!(
                ayanamsa_at_line(unix, "lahiri"),
                Err(Refusal::OutOfRange),
                "{readme} {unix}"
            );
            assert_eq!(
                ayanamsa_from_anchor_line(unix, 2_435_553.5, 23.25),
                Err(Refusal::OutOfRange),
                "{readme} {unix}"
            );
        }
    }
}

#[test]
fn the_span_of_the_jupiter_stations_is_the_years_the_readme_names() {
    let (first, last) = era();
    for readme in READMES {
        let text =
            std::fs::read_to_string(readme).unwrap_or_else(|error| panic!("{readme}: {error}"));
        let row = row(&text, "hc_jupiter_stations");
        let from = range_after(&row, "`from_unix_seconds`");
        // `from` through `last`, and `to` up to the second after it.
        assert_eq!(&from[..3], [first, last, last + 1], "{readme}: {row}");
        assert!(station_lines(first, first + 1, "lahiri").is_ok());
        assert!(station_lines(last, last + 1, "lahiri").is_ok());
        assert_eq!(
            station_lines(first - 1, first, "lahiri"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            station_lines(last + 1, last + 2, "lahiri"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            station_lines(last, last + 2, "lahiri"),
            Err(Refusal::OutOfRange)
        );
    }
}
