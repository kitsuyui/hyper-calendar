//! The text of `lines::describe_day` and `lines::calendars` for a spread of
//! days and locales, held to digests taken before the lines were made
//! faster.
//!
//! The two calls are what a page spends its time on, so they are the ones
//! that get cached, shared and short-cut. None of that may change a byte of
//! what they write, and this test is where that is proved: the digests in
//! `line_digests.txt` were written by the code before any of it, and every
//! answer since must hash the same. A digest is 64 bits of FNV-1a over the
//! whole text with its length beside it; the texts themselves would be
//! megabytes.
//!
//! A change that means to alter the text — a new calendar, a corrected
//! name — regenerates the file with `UPDATE_LINE_DIGESTS=1 cargo test -p
//! hyper-calendar --all-features --test line_digests`, and its diff is the
//! list of answers that moved. A change that means only to be faster must
//! leave the file alone.

#![cfg(all(
    feature = "alloc",
    feature = "civil",
    feature = "lunar",
    feature = "equinox",
    feature = "indic",
    feature = "regional",
    feature = "i18n",
    feature = "format"
))]

use std::fmt::Write as _;

use hyper_calendar::hc_calendar::Rd;
use hyper_calendar::lines;

/// Where the digests live, relative to this crate.
const DIGESTS: &str = "tests/line_digests.txt";

/// The days: both ends of what the astronomical calendars convert, the
/// Gregorian reform, the years the Hindu, Chinese and Hebrew engines are
/// busiest in, a leap day, a Chinese leap month (闰二月 of 2023), and days
/// outside every astronomical range.
const DAYS: [i64; 14] = [
    -1_000_000, // far before every astronomical calendar
    1,          // 1 January 1 CE
    577_736,    // 1582-10-15, the Gregorian reform
    620_700,    // 1700-03-15, early in the Hindu engines' range
    675_000,    // 1849-01-22
    693_761,    // 1900-06-15
    719_163,    // 1970-01-01
    730_179,    // 2000-02-29
    738_601,    // 2023-03-22, 闰二月初一
    739_617,    // 2026-01-01
    739_886,    // 2026-09-27
    767_000,    // 2101-01-01, past the observational Hebrew range
    839_900,    // 2300-07-26, past the Hindu engines' range
    3_652_424_634,
];

/// The locales every day is described in: a CJK locale with era names, the
/// root, a right-to-left one with leap-year month names, and each
/// calendar's own.
const LOCALES: [&str; 5] = ["ja", "en", "he", "und", lines::NATIVE];

/// FNV-1a, 64 bits.
fn fnv1a(text: &str) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in text.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// Every digest, one line each: the call, the day, the locale, the length
/// of the text and its hash.
fn digests() -> String {
    let registry = hyper_calendar::registry();
    let mut out = String::new();
    for day in DAYS {
        for locale in LOCALES {
            let text = lines::describe_day(&registry, Rd(day), locale);
            let _ = writeln!(
                out,
                "describe_day\t{day}\t{locale}\t{}\t{:016x}",
                text.len(),
                fnv1a(&text)
            );
            let text = lines::calendars(&registry, Rd(day), locale);
            let _ = writeln!(
                out,
                "calendars\t{day}\t{locale}\t{}\t{:016x}",
                text.len(),
                fnv1a(&text)
            );
        }
    }
    out
}

#[test]
fn describe_day_and_calendars_write_what_they_always_wrote() {
    let actual = digests();
    if std::env::var_os("UPDATE_LINE_DIGESTS").is_some() {
        std::fs::write(DIGESTS, &actual).expect("write the digests");
        return;
    }
    let expected = std::fs::read_to_string(DIGESTS).expect("read the digests");
    let moved: Vec<(&str, &str)> = expected
        .lines()
        .zip(actual.lines())
        .filter(|(want, got)| want != got)
        .collect();
    assert!(
        moved.is_empty() && expected.lines().count() == actual.lines().count(),
        "{} answers moved; first: {:?}. A change that means to alter them \
         regenerates with UPDATE_LINE_DIGESTS=1",
        moved.len(),
        moved.first()
    );
}
