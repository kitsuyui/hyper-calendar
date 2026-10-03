//! Date-times, durations, intervals and patterns through the C boundary.

use super::super::*;
use super::{measured, read_lines};

/// RFC 3339 §5.8's `1996-12-19T16:39:57-08:00`, 851 042 397 s by Python's
/// `datetime`; `Mon, 21 Sep 2026 14:30:05 +0900` is 1 789 968 605 s.
#[test]
fn a_date_time_is_read_and_written() {
    let parsed = read_lines(|buffer, capacity, written| unsafe {
        hc_parse_datetime(
            c"rfc3339".as_ptr(),
            c"1996-12-19T16:39:57-08:00".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(
        parsed,
        "729012\t59997\t0\toffset\t-28800\t851042397\t0\t0\n"
    );
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_format_datetime(
            c"rfc2822".as_ptr(),
            1_789_968_605,
            0,
            32_400,
            c"auto".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(text, "Mon, 21 Sep 2026 14:30:05 +0900\trfc2822\n");
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_parse_datetime(
                c"iso8601".as_ptr(),
                c"2026-02-30T00:00:00Z".as_ptr(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_INVALID_DATE
    );
}

/// Python's `isocalendar`: 2026-09-21 is 2026-W39-1; ISO 8601's `PT36H`
/// is 129 600 s.
#[test]
fn a_date_a_duration_and_an_interval_are_read_and_written() {
    let date = read_lines(|buffer, capacity, written| unsafe {
        hc_format_iso_date_as(
            739_880,
            c"week".as_ptr(),
            c"basic".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(date, "2026W391\tweek\tbasic\n");
    let parts = read_lines(|buffer, capacity, written| unsafe {
        hc_iso_date_parts(c"2026-W39".as_ptr(), buffer, capacity, written)
    });
    assert_eq!(parts, "week\t2026\t\t\t\t39\t\t\textended\n");
    let duration = read_lines(|buffer, capacity, written| unsafe {
        hc_iso_duration(c"PT36H".as_ptr(), buffer, capacity, written)
    });
    assert_eq!(
        duration,
        "0\t\t\t\t\t36\t\t\t\tdesignators\tPT36H\t0\t129600\t0\n"
    );
    let written = read_lines(|buffer, capacity, written| unsafe {
        hc_format_iso_duration(
            0,
            3,
            6,
            -1,
            4,
            12,
            30,
            5,
            core::ptr::null(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(written, "P3Y6M4DT12H30M5S\t1\t\t\n");
    let interval = read_lines(|buffer, capacity, written| unsafe {
        hc_iso_interval(
            c"2007-03-01T13:00:00Z/2008-05-11T15:30:00Z".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(
        interval,
        "\tstart-end\t2007-03-01T13:00:00Z\t1172754000\t2008-05-11T15:30:00Z\t1210519800\t\t\t\t\n"
    );
}
