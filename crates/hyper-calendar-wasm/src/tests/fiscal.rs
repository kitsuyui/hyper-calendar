use super::super::*;
use super::read_lines;

fn row(text: &str) -> Vec<String> {
    text.trim_end().split('\t').map(str::to_owned).collect()
}

#[test]
fn hc_fiscal_profiles_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe { hc_fiscal_profiles(buffer, capacity) });
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
    let text = read_lines(|buffer, capacity| unsafe {
        hc_fiscal_year_on(
            "JP".as_ptr(),
            "JP".len(),
            "government".as_ptr(),
            "government".len(),
            739000,
            buffer,
            capacity,
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
                "XX".as_ptr(),
                "XX".len(),
                "".as_ptr(),
                "".len(),
                739000,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_fiscal_year_on(
                "JP".as_ptr(),
                "JP".len(),
                "personal-tax".as_ptr(),
                "personal-tax".len(),
                739000,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_NO_DATA
    );
}

#[test]
fn hc_fiscal_year_span_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_fiscal_year_span(
            "JP".as_ptr(),
            "JP".len(),
            "government".as_ptr(),
            "government".len(),
            2024,
            buffer,
            capacity,
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
                "ZZ".as_ptr(),
                "ZZ".len(),
                "".as_ptr(),
                "".len(),
                2024,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_fiscal_year_span(
                "JP".as_ptr(),
                "JP".len(),
                "personal-tax".as_ptr(),
                "personal-tax".len(),
                2024,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_NO_DATA
    );
}

#[test]
fn hc_week_year_systems_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_week_year_systems("und".as_ptr(), 3, buffer, capacity)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 12),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "nrf-4-5-4");
    assert_eq!(first[1], "NRF 4-5-4 retail calendar");
    assert_eq!(first[2], "NRF 4-5-4 retail calendar");
    assert_eq!(first[3], "en");
    let zh = read_lines(|buffer, capacity| unsafe {
        hc_week_year_systems("zh-Hant".as_ptr(), 7, buffer, capacity)
    });
    let iso = zh
        .lines()
        .find(|line| line.starts_with("iso-8601-week-year\t"))
        .map(row)
        .unwrap_or_default();
    assert_eq!(iso[1], "ISO 8601 week-numbering year");
    assert_eq!(iso[2], "ISO週數年");
    assert_eq!(iso[3], "zh-Hant");
}

#[test]
fn hc_week_year_on_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_week_year_on(
            "nrf-4-5-4".as_ptr(),
            "nrf-4-5-4".len(),
            738580,
            buffer,
            capacity,
        )
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
                "nrf-4-5-5".as_ptr(),
                "nrf-4-5-5".len(),
                0,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_week_year_on(
                "nrf-4-5-4".as_ptr(),
                "nrf-4-5-4".len(),
                9223372036854775807,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_OUT_OF_RANGE
    );
}
