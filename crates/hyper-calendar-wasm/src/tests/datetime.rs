//! Date-times, durations, intervals and patterns as text, called as a page
//! calls them.

use super::super::*;
use super::read_lines;

/// RFC 3339 §5.8's `1996-12-19T16:39:57-08:00`, 851 042 397 s by Python's
/// `datetime`; a text with no zone is a reading with no instant.
#[test]
fn a_date_time_is_read_and_written() {
    let parse = |syntax: &str, text: &str| {
        read_lines(|buffer, capacity| unsafe {
            hc_parse_datetime(
                syntax.as_ptr(),
                syntax.len(),
                text.as_ptr(),
                text.len(),
                buffer,
                capacity,
            )
        })
    };
    assert_eq!(
        parse("rfc3339", "1996-12-19T16:39:57-08:00"),
        "729012\t59997\t0\toffset\t-28800\t851042397\t0\t0\n"
    );
    assert_eq!(
        parse("iso8601", "2026-09-21T14:30:05"),
        "739880\t52205\t0\tnone\t\t\t0\t0\n"
    );
    let written = read_lines(|buffer, capacity| unsafe {
        hc_format_datetime(
            "rfc2822".as_ptr(),
            7,
            1_789_968_605,
            0,
            32_400,
            "auto".as_ptr(),
            4,
            buffer,
            capacity,
        )
    });
    assert_eq!(written, "Mon, 21 Sep 2026 14:30:05 +0900\trfc2822\n");
    let refused = unsafe {
        hc_parse_datetime(
            "iso8601".as_ptr(),
            7,
            "2026-02-30T00:00:00Z".as_ptr(),
            20,
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(refused, HC_ERR_INVALID_DATE);
    let unknown = unsafe {
        hc_format_datetime(
            "julian".as_ptr(),
            6,
            0,
            0,
            0,
            "auto".as_ptr(),
            4,
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(unknown, HC_ERR_UNKNOWN);
}

/// Python's `isocalendar`: 2026-09-21 is 2026-W39-1 and day 264.
#[test]
fn a_date_is_a_week_date_and_a_reduced_date_has_parts() {
    let date = read_lines(|buffer, capacity| unsafe {
        hc_format_iso_date_as(
            739_880,
            "week".as_ptr(),
            4,
            "basic".as_ptr(),
            5,
            buffer,
            capacity,
        )
    });
    assert_eq!(date, "2026W391\tweek\tbasic\n");
    let parts = read_lines(|buffer, capacity| unsafe {
        hc_iso_date_parts("2026-W39".as_ptr(), 8, buffer, capacity)
    });
    assert_eq!(parts, "week\t2026\t\t\t\t39\t\t\textended\n");
    let bad = unsafe { hc_iso_date_parts("2026-W54-1".as_ptr(), 10, core::ptr::null_mut(), 0) };
    assert_eq!(bad, HC_ERR_INVALID_DATE);
}

/// ISO 8601's `PT36H` is 129 600 s as Python's `timedelta` counts, and
/// `P3Y6M4DT12H30M5S` is nominal; the repeating interval is Wikipedia's.
#[test]
fn a_duration_and_an_interval_are_read_and_written() {
    let duration = read_lines(|buffer, capacity| unsafe {
        hc_iso_duration("PT36H".as_ptr(), 5, buffer, capacity)
    });
    assert_eq!(
        duration,
        "0\t\t\t\t\t36\t\t\t\tdesignators\tPT36H\t0\t129600\t0\n"
    );
    let written = read_lines(|buffer, capacity| unsafe {
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
            0,
            buffer,
            capacity,
        )
    });
    assert_eq!(written, "P3Y6M4DT12H30M5S\t1\t\t\n");
    let interval = read_lines(|buffer, capacity| unsafe {
        hc_iso_interval(
            "R5/2008-03-01T13:00:00Z/P1Y2M10DT2H30M".as_ptr(),
            38,
            buffer,
            capacity,
        )
    });
    assert!(
        interval.starts_with(
            "5\tstart-duration\t2008-03-01T13:00:00Z\t1204376400\t\t\tP1Y2M10DT2H30M\t1\t"
        ),
        "{interval}"
    );
    let empty = unsafe {
        hc_format_iso_duration(
            0,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            core::ptr::null(),
            0,
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(empty, HC_ERR_MALFORMED);
    assert_eq!(
        unsafe { hc_iso_interval("2008".as_ptr(), 4, core::ptr::null_mut(), 0) },
        HC_ERR_MALFORMED
    );
}
