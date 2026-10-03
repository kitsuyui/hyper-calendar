use super::super::*;
use super::read_lines;

fn row(text: &str) -> Vec<String> {
    text.trim_end().split('\t').map(str::to_owned).collect()
}

#[test]
fn hc_attribution_authorities_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_attribution_authorities("".as_ptr(), "".len(), buffer, capacity)
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
                "gemstone".as_ptr(),
                "gemstone".len(),
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
}

#[test]
fn hc_attributions_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_attributions(
            "birthstone".as_ptr(),
            "birthstone".len(),
            1,
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
    assert_eq!(first[0], "birthstone");
    assert_eq!(first[5], "garnet");
    assert_eq!(first[12], "1");
    assert_eq!(
        unsafe {
            hc_attributions(
                "birthstone".as_ptr(),
                "birthstone".len(),
                13,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe {
            hc_attributions(
                "gemstone".as_ptr(),
                "gemstone".len(),
                1,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
}

#[test]
fn hc_attributions_on_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_attributions_on(739887, "".as_ptr(), "".len(), buffer, capacity)
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
                "nowhere".as_ptr(),
                "nowhere".len(),
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
}

#[test]
fn hc_harvest_moon_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_harvest_moon(2025, "".as_ptr(), "".len(), buffer, capacity)
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
        unsafe { hc_harvest_moon(5000, "".as_ptr(), "".len(), core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe {
            hc_harvest_moon(
                2026,
                "elsewhere".as_ptr(),
                "elsewhere".len(),
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
}
