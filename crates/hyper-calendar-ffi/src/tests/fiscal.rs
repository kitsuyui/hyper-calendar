use super::super::*;
use super::read_lines;

fn row(text: &str) -> Vec<String> {
    text.trim_end().split('\t').map(str::to_owned).collect()
}

#[test]
fn hc_fiscal_profiles_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_fiscal_profiles(buffer, capacity, written)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 20),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "AO");
}

#[test]
fn hc_fiscal_year_on_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_fiscal_year_on(
            c"JP".as_ptr(),
            c"government".as_ptr(),
            739000,
            buffer,
            capacity,
            written,
        )
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 18),
        "{text}"
    );
    // The July year of 1875 to 1884 is a system of its own and is not in
    // force in 2024; the April year is.
    let all: Vec<Vec<String>> = text.lines().map(row).collect();
    assert_eq!(all[0][4], "outside-validity");
    let first = all
        .iter()
        .find(|line| line[4] == "in-force")
        .cloned()
        .unwrap_or_default();
    assert_eq!(first[0], "JP");
    assert_eq!(first[2], "government");
    assert_eq!(first[4], "in-force");
    assert_eq!(
        unsafe {
            hc_fiscal_year_on(
                c"XX".as_ptr(),
                c"".as_ptr(),
                739000,
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_fiscal_year_on(
                c"JP".as_ptr(),
                c"personal-tax".as_ptr(),
                739000,
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_NO_DATA
    );
}

#[test]
fn hc_fiscal_year_span_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_fiscal_year_span(
            c"JP".as_ptr(),
            c"government".as_ptr(),
            2024,
            buffer,
            capacity,
            written,
        )
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 9),
        "{text}"
    );
    let all: Vec<Vec<String>> = text.lines().map(row).collect();
    let first = all
        .iter()
        .find(|line| line[4] == "in-force")
        .cloned()
        .unwrap_or_default();
    assert_eq!(first[0], "JP");
    assert_eq!(first[4], "in-force");
    assert_eq!(first[5], "2024");
    assert_eq!(
        unsafe {
            hc_fiscal_year_span(
                c"ZZ".as_ptr(),
                c"".as_ptr(),
                2024,
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_fiscal_year_span(
                c"JP".as_ptr(),
                c"personal-tax".as_ptr(),
                2024,
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_NO_DATA
    );
}

#[test]
fn hc_week_year_systems_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_week_year_systems(buffer, capacity, written)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 10),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "nrf-4-5-4");
}

#[test]
fn hc_week_year_on_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_week_year_on(c"nrf-4-5-4".as_ptr(), 738580, buffer, capacity, written)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 14),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "nrf-4-5-4");
    assert_eq!(first[6], "1");
    assert_eq!(
        unsafe {
            hc_week_year_on(
                c"nrf-4-5-5".as_ptr(),
                0,
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_week_year_on(
                c"nrf-4-5-4".as_ptr(),
                9223372036854775807,
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
}
