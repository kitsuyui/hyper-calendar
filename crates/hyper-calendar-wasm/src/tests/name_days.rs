use super::super::*;
use super::read_lines;

fn row(text: &str) -> Vec<String> {
    text.trim_end().split('\t').map(str::to_owned).collect()
}

#[test]
fn hc_name_day_lists_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe { hc_name_day_lists(buffer, capacity) });
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
    let text = read_lines(|buffer, capacity| unsafe {
        hc_name_days_on("lv".as_ptr(), "lv".len(), 739061, buffer, capacity)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 14),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "list");
    assert_eq!(
        unsafe { hc_name_days_on("jp".as_ptr(), "jp".len(), 739061, core::ptr::null_mut(), 0) },
        HC_ERR_UNKNOWN
    );
}

#[test]
fn hc_name_day_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_name_day(
            "lv".as_ptr(),
            "lv".len(),
            "Jānis".as_ptr(),
            "Jānis".len(),
            2024,
            buffer,
            capacity,
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
                "jp".as_ptr(),
                "jp".len(),
                "Jānis".as_ptr(),
                "Jānis".len(),
                2026,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
}
