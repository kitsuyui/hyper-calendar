//! The Chinese calendar against the Purple Mountain Observatory's table.
//!
//! `data/chinese_month_lengths_1900_2024.txt` holds the first day of the
//! year, the leap month and the length of every month of the 125 Chinese
//! years that began from 31 January 1900 to 10 February 2024, read from the
//! Observatory's day-by-day 1900–2025 calendar; its header says how. The
//! table is the calendar as it was promulgated — the Qing 時憲書 to 1911,
//! the Republic's almanacs to 1948 — so it tests the meridian history and
//! the almanac corrections as well as the rules.

use hc_calendar::{Calendar, Month, Rd};
use hc_calendars_lunar::ChineseCalendar;
use hc_calendars_lunar::chinese;
use hc_calendars_lunar::lunisolar::LunisolarParameters;

/// The published table.
const TABLE: &str = include_str!("data/chinese_month_lengths_1900_2024.txt");

/// The fixed day of a proleptic Gregorian date.
fn gregorian(year: i64, month: u8, day: u8) -> Rd {
    hc_calendar::gregorian::to_fixed(year, month, day)
        .unwrap_or_else(|error| panic!("{year}-{month}-{day}: {error:?}"))
}

/// Every month of the table as `(Chinese year, month, first day, length)`.
fn months() -> Vec<(i64, Month, Rd, i64)> {
    let mut out = Vec::new();
    for line in TABLE.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split_whitespace().collect();
        let [date, leap, lengths] = fields[..] else {
            panic!("malformed line {line:?}");
        };
        let parts: Vec<i64> = date
            .split('-')
            .map(|part| {
                part.parse()
                    .unwrap_or_else(|_| panic!("{part:?} in {line:?} is not a number"))
            })
            .collect();
        let start = gregorian(parts[0], parts[1] as u8, parts[2] as u8);
        // The first month begins in January or February of the Gregorian
        // year, and the year that began in 2024 is 4661.
        let year = parts[0] + 2_637;
        let leap: u8 = leap
            .parse()
            .unwrap_or_else(|_| panic!("{leap:?} in {line:?} is not a leap ordinal"));
        let mut cursor = start;
        let mut ordinal = 1u8;
        let mut seen_regular = false;
        for mark in lengths.chars() {
            let length = match mark {
                'L' => 30,
                'S' => 29,
                other => panic!("unknown length {other:?} in {line:?}"),
            };
            let month = if leap != 0 && ordinal == leap && seen_regular {
                Month::leap(ordinal)
            } else {
                Month::regular(ordinal)
            };
            out.push((year, month, cursor, length));
            cursor = Rd(cursor.0 + length);
            if leap != 0 && ordinal == leap && !seen_regular {
                seen_regular = true;
            } else {
                ordinal += 1;
            }
        }
        assert_eq!(ordinal, 13, "{line:?} does not hold twelve months");
    }
    out
}

#[test]
fn the_table_is_read_whole() {
    let months = months();
    // 125 years, 46 of them with a leap month.
    assert_eq!(months.len(), 125 * 12 + 46);
    assert_eq!(
        months.first().map(|month| month.2),
        Some(gregorian(1900, 1, 31))
    );
    // Each month begins the day after the last one ends.
    for pair in months.windows(2) {
        assert_eq!(pair[0].2.0 + pair[0].3, pair[1].2.0, "{:?}", pair[1]);
    }
}

#[test]
fn the_chinese_calendar_is_the_purple_mountain_observatorys_from_1900_to_2024() {
    for (year, month, first, length) in months() {
        assert_eq!(
            ChineseCalendar
                .from_fixed(first)
                .map(|date| (date.year, date.month, date.day)),
            Ok((year, month, 1)),
            "the first day of {year} {month:?}"
        );
        let last = Rd(first.0 + length - 1);
        assert_eq!(
            ChineseCalendar
                .from_fixed(last)
                .map(|date| (date.year, date.month, date.day)),
            Ok((year, month, length as u8)),
            "the last day of {year} {month:?}"
        );
    }
}

#[test]
fn without_the_almanac_the_rules_miss_one_month_in_the_table() {
    // The rules alone, at the same meridians, against the table: the one
    // month they begin on another day is the fourth of 1906, which is why
    // `chinese::ALMANAC_CORRECTIONS` holds that entry and no other.
    static RULES: LunisolarParameters = LunisolarParameters {
        month_start_corrections: &[],
        ..chinese::PARAMETERS
    };
    let missed: Vec<Rd> = months()
        .into_iter()
        .map(|(_, _, first, _)| first)
        .filter(|first| RULES.new_moon_on_or_after(*first) != *first)
        .collect();
    assert_eq!(missed, [gregorian(1906, 4, 24)]);
}
