use super::super::*;
use super::read_lines;

fn row(text: &str) -> Vec<String> {
    text.trim_end().split('\t').map(str::to_owned).collect()
}

#[test]
fn hc_units_crosses_the_boundary() {
    let text =
        read_lines(|buffer, capacity, written| unsafe { hc_units(buffer, capacity, written) });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 7),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "quectosecond");
    assert_eq!(first[2], "qs");
}

#[test]
fn hc_unit_convert_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_unit_convert(
            2,
            1,
            c"hour".as_ptr(),
            c"millisecond".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 10),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "hour");
    assert_eq!(first[1], "millisecond");
    assert_eq!(first[4], "7200000");
    assert_eq!(first[6], "1");
    assert_eq!(
        unsafe {
            hc_unit_convert(
                1,
                0,
                c"second".as_ptr(),
                c"day".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe {
            hc_unit_convert(
                1,
                1,
                c"second".as_ptr(),
                c"furlong".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
}

#[test]
fn hc_rates_crosses_the_boundary() {
    let text =
        read_lines(|buffer, capacity, written| unsafe { hc_rates(buffer, capacity, written) });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 4),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[1], "frame");
}

#[test]
fn hc_frame_period_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_frame_period(c"24".as_ptr(), c"flick".as_ptr(), buffer, capacity, written)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 11),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "24");
    assert_eq!(first[1], "frame");
    assert_eq!(first[6], "29400000");
    assert_eq!(first[10], "1");
    assert_eq!(
        unsafe {
            hc_frame_period(
                c"0".as_ptr(),
                c"second".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe {
            hc_frame_period(
                c"fast".as_ptr(),
                c"second".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
}

#[test]
fn hc_tempo_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_tempo(120, 1, 2, 0, 0, 0, 2, buffer, capacity, written)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 10),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[2], "1");
    assert_eq!(first[3], "2");
    assert_eq!(first[8], "500000");
    assert_eq!(first[9], "1");
    assert_eq!(
        unsafe {
            hc_tempo(
                0,
                1,
                2,
                0,
                0,
                0,
                2,
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe {
            hc_tempo(
                120,
                1,
                2,
                0,
                0,
                3,
                2,
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
}
