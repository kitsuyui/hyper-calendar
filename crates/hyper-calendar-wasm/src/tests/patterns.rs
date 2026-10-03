//! Patterns read against a text, called as a page calls them.

use super::super::*;
use super::read_lines;

/// Python's `strptime` of `%H:%M` is on 1900-01-01, RD 693 596.
#[test]
fn a_text_is_read_against_a_pattern() {
    let line = read_lines(|buffer, capacity| unsafe {
        hc_parse_pattern(
            "python".as_ptr(),
            6,
            "%H:%M".as_ptr(),
            5,
            "14:30".as_ptr(),
            5,
            buffer,
            capacity,
        )
    });
    let cells: Vec<&str> = line.trim_end().split('\t').collect();
    assert_eq!(cells.len(), 30);
    assert_eq!(cells[0], "1900");
    assert_eq!(cells[22..24], ["693596", "52200"]);
    let unknown = unsafe {
        hc_parse_pattern(
            "regex".as_ptr(),
            5,
            "%Y".as_ptr(),
            2,
            "2026".as_ptr(),
            4,
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(unknown, HC_ERR_UNKNOWN);
}

/// CLDR 48's German month names: *September* in `MMMM`, read with `de`, is
/// month 9; the C locale does not know *Sept.*; Python's `strptime` takes no
/// locale.
#[test]
fn a_text_is_read_against_a_pattern_in_a_locale() {
    let line = read_lines(|buffer, capacity| unsafe {
        hc_parse_pattern_in(
            "cldr".as_ptr(),
            4,
            "d. MMMM y".as_ptr(),
            9,
            "21. Dezember 2026".as_ptr(),
            17,
            "de".as_ptr(),
            2,
            buffer,
            capacity,
        )
    });
    let cells: Vec<&str> = line.trim_end().split('\t').collect();
    assert_eq!(cells.len(), 30);
    assert_eq!(cells[..5], ["2026", "", "", "12", "21"]);
    let python = unsafe {
        hc_parse_pattern_in(
            "python".as_ptr(),
            6,
            "%b".as_ptr(),
            2,
            "Dez".as_ptr(),
            3,
            "de".as_ptr(),
            2,
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(python, HC_ERR_UNKNOWN);
}
