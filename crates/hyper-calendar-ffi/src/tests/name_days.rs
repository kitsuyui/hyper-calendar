use super::super::*;
use super::read_lines;

fn row(text: &str) -> Vec<String> {
    text.trim_end().split('\t').map(str::to_owned).collect()
}

#[test]
fn hc_name_day_lists_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_name_day_lists(buffer, capacity, written)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 17),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "list");
    assert_eq!(first[1], "lv-extended-2023");
}

#[test]
fn hc_name_days_on_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_name_days_on(c"lv".as_ptr(), 739061, buffer, capacity, written)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 14),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "list");
    assert_eq!(
        unsafe {
            hc_name_days_on(
                c"jp".as_ptr(),
                739061,
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
}

#[test]
fn hc_name_day_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_name_day(
            c"lv".as_ptr(),
            c"Jānis".as_ptr(),
            2024,
            buffer,
            capacity,
            written,
        )
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 13),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "list");
    assert_eq!(
        unsafe {
            hc_name_day(
                c"jp".as_ptr(),
                c"Jānis".as_ptr(),
                2026,
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
}
