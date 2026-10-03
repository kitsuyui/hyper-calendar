//! The date a table's sources were last checked is no earlier than any date
//! its `sources` text gives for reading a source.
//!
//! `RuleSet::sources_checked` is the day the table's sources were last
//! checked against their statute or gazette. A `sources` text that says a
//! page was retrieved, or a document read, on a later day than that records
//! a check the field does not know of: one of the two is wrong, and the
//! field, which says how fresh the table is, is the one that was left
//! behind.

use hc_holiday::rule::{RuleSet, SourceDate};
use hc_holiday::{countries, exchanges, international, traditions};

fn tables() -> impl Iterator<Item = &'static RuleSet> {
    countries::ALL
        .iter()
        .chain(exchanges::ALL)
        .chain(traditions::ALL)
        .chain(international::ALL)
        .copied()
}

/// Every `YYYY-MM-DD` in a text, as a date. A run of digits that is not
/// exactly a year, a month and a day, such as a catalogue number, is not one.
fn dates_in(text: &str) -> Vec<SourceDate> {
    let bytes = text.as_bytes();
    let mut found = Vec::new();
    let mut at = 0;
    while at + 10 <= bytes.len() {
        let window = &bytes[at..at + 10];
        let shape = window.iter().enumerate().all(|(i, byte)| match i {
            4 | 7 => *byte == b'-',
            _ => byte.is_ascii_digit(),
        });
        let before = at == 0 || !bytes[at - 1].is_ascii_digit();
        let after = at + 10 == bytes.len() || !bytes[at + 10].is_ascii_digit();
        if shape && before && after {
            let number = |from: usize, to: usize| {
                text[at + from..at + to].parse::<u32>().unwrap_or_default()
            };
            let (year, month, day) = (number(0, 4), number(5, 7), number(8, 10));
            if (1..=12).contains(&month) && (1..=31).contains(&day) {
                found.push(SourceDate::new(
                    i32::try_from(year).unwrap_or_default(),
                    u8::try_from(month).unwrap_or_default(),
                    u8::try_from(day).unwrap_or_default(),
                ));
            }
            at += 10;
        } else {
            at += 1;
        }
    }
    found
}

#[test]
fn the_dates_in_a_text_are_read_as_dates() {
    let found = dates_in("retrieved 2026-09-26; no. 2026-09-261 and 12026-09-26, read 2025-12-01.");
    assert_eq!(
        found,
        [SourceDate::new(2026, 9, 26), SourceDate::new(2025, 12, 1)]
    );
}

#[test]
fn a_table_was_checked_no_earlier_than_any_date_its_sources_give() {
    let mut late = Vec::new();
    for table in tables() {
        for date in dates_in(table.sources) {
            if date > table.sources_checked {
                late.push(format!(
                    "{}: sources_checked {:?} is before {date:?} in its sources",
                    table.code, table.sources_checked
                ));
            }
        }
    }
    assert!(late.is_empty(), "{}", late.join("\n"));
}
