//! Patterns read against a text through the C boundary.

use super::super::*;
use super::{measured, read_lines};

/// Python's `strptime` of `%H:%M` is on 1900-01-01, RD 693 596.
#[test]
fn a_text_is_read_against_a_pattern() {
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_parse_pattern(
            c"python".as_ptr(),
            c"%H:%M".as_ptr(),
            c"14:30".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    let cells: Vec<&str> = line.trim_end().split('\t').collect();
    assert_eq!(cells.len(), 30);
    assert_eq!(cells[22..24], ["693596", "52200"]);
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_parse_pattern(
                c"regex".as_ptr(),
                c"%Y".as_ptr(),
                c"2026".as_ptr(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_UNKNOWN
    );
}

/// CLDR 48's German month names: *Dezember* in `MMMM`, read with `de`, is
/// month 12; Python's `strptime` takes no locale.
#[test]
fn a_text_is_read_against_a_pattern_in_a_locale() {
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_parse_pattern_in(
            c"cldr".as_ptr(),
            c"d. MMMM y".as_ptr(),
            c"21. Dezember 2026".as_ptr(),
            c"de".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    let cells: Vec<&str> = line.trim_end().split('\t').collect();
    assert_eq!(cells.len(), 30);
    assert_eq!(cells[..5], ["2026", "", "", "12", "21"]);
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_parse_pattern_in(
                c"python".as_ptr(),
                c"%b".as_ptr(),
                c"Dez".as_ptr(),
                c"de".as_ptr(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_UNKNOWN
    );
}
