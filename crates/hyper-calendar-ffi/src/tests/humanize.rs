use super::super::*;
use super::{measured, read_lines};

/// CLDR 48's English relative-time patterns and conjunction list.
#[test]
fn the_lines_are_the_modules() {
    let now = 1_700_000_000;
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_relative_time(
            now - 3 * 3_600,
            now,
            c"long".as_ptr(),
            0,
            c"en".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, "3 hours ago\thour\t-3\ten\n");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_relative_day(
            739_887,
            739_888,
            c"long".as_ptr(),
            1,
            c"en".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, "yesterday\tday\t-1\ten\n");
    // A null locale is the root locale, CLDR's root.xml: `-1 d`.
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_relative_day(
            739_887,
            739_888,
            c"long".as_ptr(),
            1,
            core::ptr::null(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, "-1 d\tday\t-1\tund\n");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_relative_day_at(
            739_887,
            739_888,
            15 * 3_600 + 5 * 60,
            c"long".as_ptr(),
            1,
            c"en".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, "yesterday at 15:05\tday\t-1\t15:05\ten\n");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_duration(
            9_000,
            c"long".as_ptr(),
            0,
            c"en".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, "2 hours and 30 minutes\t0\ten\n");
}

#[test]
fn a_style_not_named_is_refused() {
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_relative_time(
                0,
                0,
                c"wide".as_ptr(),
                0,
                c"en".as_ptr(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_duration(
                0,
                core::ptr::null(),
                0,
                c"en".as_ptr(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_relative_day(
                i64::MAX,
                -1,
                c"long".as_ptr(),
                0,
                c"en".as_ptr(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_OUT_OF_RANGE
    );
}
