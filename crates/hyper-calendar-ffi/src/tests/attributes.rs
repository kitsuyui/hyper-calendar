use super::super::*;
use super::read_lines;

fn row(text: &str) -> Vec<String> {
    text.trim_end().split('\t').map(str::to_owned).collect()
}

#[test]
fn hc_attribution_authorities_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_attribution_authorities(c"".as_ptr(), buffer, capacity, written)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 17),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "authority");
    assert_eq!(
        unsafe {
            hc_attribution_authorities(
                c"gemstone".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
}

#[test]
fn hc_attributions_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_attributions(c"birthstone".as_ptr(), 1, buffer, capacity, written)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 13),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "birthstone");
    assert_eq!(first[5], "garnet");
    assert_eq!(first[12], "1");
    assert_eq!(
        unsafe {
            hc_attributions(
                c"birthstone".as_ptr(),
                13,
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe {
            hc_attributions(
                c"gemstone".as_ptr(),
                1,
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
}

#[test]
fn hc_attributions_on_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_attributions_on(739887, c"".as_ptr(), buffer, capacity, written)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 13),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "birthstone");
    assert_eq!(
        unsafe {
            hc_attributions_on(
                739887,
                c"nowhere".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
}

#[test]
fn hc_harvest_moon_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_harvest_moon(2025, c"".as_ptr(), buffer, capacity, written)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 7),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[3], "10");
    assert_eq!(first[4], "Corn Moon");
    assert_eq!(
        unsafe {
            hc_harvest_moon(
                5000,
                c"".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe {
            hc_harvest_moon(
                2026,
                c"elsewhere".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
}
