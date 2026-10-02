//! Jupiter's position, its ingresses and the festivals found from them,
//! across the C boundary.

use super::super::*;
use super::read_lines;

fn rows(text: &str) -> Vec<Vec<&str>> {
    text.lines()
        .map(|line| line.split('\t').collect())
        .collect()
}

/// 0 h UT on 2024-12-07, the day of Jupiter's opposition, where Horizons'
/// apparent ecliptic longitude is 76.3751533° (`jpl-horizons`).
const OPPOSITION_2024: i64 = 1_733_529_600;

#[test]
fn jupiter_at_its_opposition_of_2024_crosses_the_c_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_jupiter_at(
            OPPOSITION_2024,
            c"lahiri".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    let row = &rows(&text)[0];
    assert_eq!(row.len(), 12);
    let longitude: f64 = row[0].parse().expect("a number");
    assert!((longitude - 76.375_153_3).abs() < 0.0002);
    assert_eq!((row[4], row[8]), ("vrishabha", "1"));
    let mut written = 0usize;
    assert_eq!(
        unsafe {
            hc_jupiter_at(
                OPPOSITION_2024,
                core::ptr::null(),
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        unsafe {
            hc_jupiter_at(
                OPPOSITION_2024,
                c"no-such".as_ptr(),
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_UNKNOWN
    );
}

#[test]
fn the_ingresses_and_the_festivals_found_cross_the_c_boundary() {
    // 2019-01-01 to 2020-01-01.
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_jupiter_ingresses(
            1_546_300_800,
            1_577_836_800,
            c"lahiri".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(rows(&text).len(), 3);
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_kumbh_by_sky(
            c"kumbh-prayag-vrishabha".as_ptr(),
            2025,
            c"lahiri".as_ptr(),
            c"en".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    let row = &rows(&text)[0];
    assert_eq!((row[1], row[12], row[13]), ("prayag", "1", "vrishabha"));
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_pushkaram_by_sky(
            c"simha".as_ptr(),
            2015,
            c"lahiri".as_ptr(),
            c"pushkaram-final-entry".as_ptr(),
            28.6356,
            77.2244,
            0.0,
            c"india".as_ptr(),
            c"en".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(rows(&text)[0][..2], ["pushkaram-godavari", "Godavari"]);
}

#[test]
fn the_rising_of_2026_crosses_the_c_boundary() {
    // 2026-01-01 to 2027-01-01.
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_jupiter_risings(
            1_767_225_600,
            1_798_761_600,
            c"lahiri".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    let rows = rows(&text);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0][4], "pushya");
}
