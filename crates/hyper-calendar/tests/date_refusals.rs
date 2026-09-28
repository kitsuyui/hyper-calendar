//! Every written list of the reader's refusals is `DateRefusal`'s codes
//! and names.
//!
//! `hc-format`'s `label::DateRefusal` gives each refusal a stable code
//! and name, and the boundaries hand both back in a line's error columns.
//! The documents a caller reads them from — the WebAssembly README's table
//! and `docs/systems/written-dates.md`'s, the C README, the rustdoc of
//! `hc_parse_date` and `lines::parse_date`, and the binding's JSDoc and
//! `.d.ts` — each write the list out. Each is held here to the codes and
//! names of every refusal, both ways: every refusal has its row or its
//! place in the list, and every row or listed name is a refusal's, with
//! the refusal's code. The row for a calendar's own refusal gives the range
//! of `CalendarError`'s codes.
//!
//! `DateRefusal` is `#[non_exhaustive]` and lists no variants, so the
//! refusals are built one of each, as `hc-format`'s own
//! `every_refusal_has_its_code_and_name` builds them; a refusal added
//! there and to the documents but not here fails as a row no refusal has.

#![cfg(feature = "full")]

use std::collections::BTreeSet;

use hyper_calendar::hc_format::label::DateRefusal;
use hyper_calendar::{CalendarError, Rd, Weekday};

#[path = "support/code_lists.rs"]
mod code_lists;

const WASM_README: &str = "../hyper-calendar-wasm/README.md";
const FFI_README: &str = "../hyper-calendar-ffi/README.md";
const WRITTEN_DATES: &str = "../../docs/systems/written-dates.md";
const EXPORTS: &str = "src/exports.rs";
const LINES: &str = "src/lines.rs";
const BINDING: &str = "../hyper-calendar-wasm/js/hyper-calendar.js";
const DTS: &str = "../hyper-calendar-wasm/js/hyper-calendar.d.ts";

fn read(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("could not read {path}: {error}"))
}

/// One of each of the reader's own refusals.
fn refusals() -> [DateRefusal; 7] {
    [
        DateRefusal::Empty,
        DateRefusal::NotRecognised { offset: 0 },
        DateRefusal::Ambiguous {
            first: Rd(1),
            second: Rd(2),
        },
        DateRefusal::TwoDigitYear,
        DateRefusal::YearNotWritten,
        DateRefusal::WeekdayMismatch {
            written: Weekday::Monday,
            actual: Weekday::Sunday,
        },
        DateRefusal::FieldMismatch,
    ]
}

/// The refusals' `(code, name)`.
fn codes_and_names() -> BTreeSet<(u32, String)> {
    refusals()
        .iter()
        .map(|refusal| (refusal.code(), refusal.name().to_owned()))
        .collect()
}

fn names() -> BTreeSet<String> {
    codes_and_names()
        .into_iter()
        .map(|(_, name)| name)
        .collect()
}

/// The first and last code of a calendar's own refusal, which
/// `DateRefusal::NoSuchDate` passes on.
fn calendar_codes() -> (u32, u32) {
    let codes: Vec<u32> = [
        CalendarError::YearOutOfRange,
        CalendarError::MonthOutOfRange,
        CalendarError::DayOutOfRange,
        CalendarError::MissingField("day"),
        CalendarError::UnsupportedField("era"),
        CalendarError::BeforeEpoch,
        CalendarError::AfterSupportedRange,
        CalendarError::UnknownEra,
        CalendarError::UnknownCalendar,
        CalendarError::Overflow,
        CalendarError::AstronomicalModelFailure,
    ]
    .iter()
    .map(|error| DateRefusal::NoSuchDate(*error).code())
    .collect();
    let first = codes.iter().copied().fold(u32::MAX, u32::min);
    let last = codes.iter().copied().fold(0, u32::max);
    (first, last)
}

/// The rows of the Markdown table whose header is `header`, each as its
/// cells, trimmed.
fn table_rows(markdown: &str, header: &str) -> Vec<Vec<String>> {
    let at = markdown
        .find(header)
        .unwrap_or_else(|| panic!("no table headed {header}"));
    markdown[at..]
        .lines()
        .skip(2)
        .take_while(|line| line.starts_with('|'))
        .map(|line| {
            line.trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().to_owned())
                .collect()
        })
        .collect()
}

/// What a code table gets wrong: `rows` are `(code cell, name cell)`.
/// Every refusal has a row with its code and name, every other row is the
/// calendar's own with the calendar errors' range, and that row is there.
fn table_problems(place: &str, rows: &[(String, String)]) -> Vec<String> {
    let mut out = Vec::new();
    let mut listed = BTreeSet::new();
    let mut calendar_row = false;
    let (first, last) = calendar_codes();
    for (code, name) in rows {
        if let Some(name) = name
            .strip_prefix('`')
            .and_then(|name| name.strip_suffix('`'))
        {
            match code.parse::<u32>() {
                Ok(code) => {
                    listed.insert((code, name.to_owned()));
                }
                Err(_) => out.push(format!("{place}: `{name}` has the code {code}")),
            }
        } else if name == "the calendar's" {
            calendar_row = true;
            if *code != format!("{first}–{last}") {
                out.push(format!(
                    "{place}: the calendar's own refusals are {code}, not {first}–{last}"
                ));
            }
        } else {
            out.push(format!("{place}: a row names {name}"));
        }
    }
    let expected = codes_and_names();
    for (code, name) in expected.difference(&listed) {
        out.push(format!("{place}: no row gives `{name}` code {code}"));
    }
    for (code, name) in listed.difference(&expected) {
        out.push(format!(
            "{place}: a row gives `{name}` code {code}, which no refusal has"
        ));
    }
    if !calendar_row {
        out.push(format!("{place}: no row for the calendar's own refusal"));
    }
    out
}

/// The list of `text` that holds `ambiguous`, as a set.
fn list_of_names(text: &str) -> BTreeSet<String> {
    code_lists::code_lists(text)
        .into_iter()
        .find(|list| list.contains(&"ambiguous"))
        .unwrap_or_else(|| panic!("no list holds `ambiguous`: {text}"))
        .into_iter()
        .map(str::to_owned)
        .collect()
}

/// The text from the first line that holds `from` to the next that holds
/// `to`, the `///` and ` * ` of a doc comment taken off, one line.
fn doc_between(source: &str, from: &str, to: &str) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let start = lines
        .iter()
        .position(|line| line.contains(from))
        .unwrap_or_else(|| panic!("no line holds {from}"));
    let end = start
        + lines[start..]
            .iter()
            .position(|line| line.contains(to))
            .unwrap_or_else(|| panic!("no line after {from} holds {to}"));
    lines[start..end]
        .iter()
        .map(|line| {
            let line = line.trim_start();
            line.strip_prefix("///")
                .or_else(|| line.strip_prefix('*'))
                .unwrap_or(line)
                .trim()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Every `` `name` (code) `` of `text`.
fn named_codes(text: &str) -> Vec<(u32, String)> {
    let pieces: Vec<&str> = text.split('`').collect();
    pieces
        .iter()
        .enumerate()
        .skip(1)
        .step_by(2)
        .filter_map(|(index, name)| {
            let after = pieces.get(index + 1)?.strip_prefix(" (")?;
            let code = after[..after.find(')')?].parse().ok()?;
            Some((code, (*name).to_owned()))
        })
        .collect()
}

#[test]
fn the_code_tables_are_the_refusals() {
    let mut found = Vec::new();
    let wasm: Vec<(String, String)> = table_rows(&read(WASM_README), "| Code | Name | The text |")
        .into_iter()
        .map(|cells| (cells[0].clone(), cells[1].clone()))
        .collect();
    found.extend(table_problems(WASM_README, &wasm));
    let written: Vec<(String, String)> =
        table_rows(&read(WRITTEN_DATES), "| Refusal | Code | When | Example |")
            .into_iter()
            .map(|cells| (cells[1].clone(), cells[0].clone()))
            .collect();
    found.extend(table_problems(WRITTEN_DATES, &written));
    assert!(found.is_empty(), "{}", found.join("\n"));
}

#[test]
fn every_list_of_the_refusals_names_them_all() {
    let names = names();
    let codes: Vec<u32> = codes_and_names().iter().map(|(code, _)| *code).collect();
    let (first, last) = (codes[0], codes[codes.len() - 1]);
    let ffi = read(FFI_README);
    let ffi = doc_between(
        &ffi,
        "`hc_parse_date(calendar, locale, text",
        "carry `HC_ERROR_UNKNOWN`",
    );
    let lines = read(LINES);
    let lines = doc_between(&lines, "are the refusal's", "calendar's own for fields");
    let binding = read(BINDING);
    let dts = read(DTS);
    let places = [
        (FFI_README, ffi.clone()),
        (LINES, lines.clone()),
        (
            BINDING,
            doc_between(&binding, "whose `error` says why", "`fixed` is `null`"),
        ),
        (
            DTS,
            doc_between(&dts, "`error` says why", "the calendar's own"),
        ),
    ];
    for (place, text) in &places {
        assert_eq!(&list_of_names(text), &names, "{place}: {text}");
    }
    assert!(
        ffi.contains(&format!("codes {first} to {last}")),
        "{FFI_README}: {ffi}"
    );
    assert!(
        lines.contains(&format!("from {first} up")),
        "{LINES}: {lines}"
    );
    // The rustdoc of `hc_parse_date`, the C library's and the WebAssembly
    // module's, gives each name its code.
    let exports = read(EXPORTS);
    let tail = &exports[exports
        .find("fn hc_calendar_units(")
        .unwrap_or_else(|| panic!("no hc_calendar_units"))..];
    for (side, docs) in [
        ("c", doc_between(tail, "fn hc_calendar_units(", "wasm {")),
        ("wasm", doc_between(tail, "wasm {", "fn hc_parse_date(")),
    ] {
        let pairs = named_codes(&docs);
        let set: BTreeSet<(u32, String)> = pairs.iter().cloned().collect();
        assert_eq!(
            set,
            codes_and_names(),
            "{EXPORTS} {side} hc_parse_date: {docs}"
        );
        assert_eq!(pairs.len(), set.len(), "{EXPORTS} {side}: {pairs:?}");
    }
}

/// The checks fail a table that leaves a refusal out, gives one the wrong
/// code, adds one no refusal is, or gives the calendar's own the wrong
/// range, and they find a list and the codes a text gives.
#[test]
fn the_checks_catch_a_missing_row_and_a_wrong_code() {
    let (first, last) = calendar_codes();
    let mut rows: Vec<(String, String)> = codes_and_names()
        .into_iter()
        .map(|(code, name)| (code.to_string(), format!("`{name}`")))
        .collect();
    rows.push((format!("{first}–{last}"), "the calendar's".to_owned()));
    assert!(table_problems("t", &rows).is_empty());
    let mut wrong = rows.clone();
    wrong.remove(0);
    wrong[0].0 = "199".to_owned();
    wrong.push(("108".to_owned(), "`no-such-refusal`".to_owned()));
    let calendar = wrong.len() - 2;
    wrong[calendar].0 = "1–10".to_owned();
    let found = table_problems("t", &wrong);
    assert_eq!(found.len(), 5, "{found:?}");
    assert_eq!(
        list_of_names("is `a`, `ambiguous` or `b`, as for `c`"),
        BTreeSet::from(["a".to_owned(), "ambiguous".to_owned(), "b".to_owned()])
    );
    assert_eq!(
        named_codes("`ambiguous` (103), `empty` (101) or `x`"),
        vec![(103, "ambiguous".to_owned()), (101, "empty".to_owned())]
    );
}
