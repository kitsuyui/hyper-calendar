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
fn a_years_pushkarams_cross_the_c_boundary_as_the_sign_by_sign_ones_do() {
    let in_year = |year: i64, rule: &core::ffi::CStr| {
        read_lines(|buffer, capacity, written| unsafe {
            hc_pushkarams_in_year(
                year,
                c"lahiri".as_ptr(),
                rule.as_ptr(),
                28.6356,
                77.2244,
                0.0,
                c"india".as_ptr(),
                c"en".as_ptr(),
                buffer,
                capacity,
                written,
            )
        })
    };
    // 2019: the final entry into Dhanus, on 5 November, is the Tapti and
    // Brahmaputra festivals' beginning.
    let text = in_year(2019, c"pushkaram-final-entry");
    let rows = rows(&text);
    assert_eq!(rows.len(), 2, "{text}");
    assert!(rows.iter().all(|row| row.len() == 14));
    assert_eq!(rows[0][0], "pushkaram-tapti");
    let by_sign = read_lines(|buffer, capacity, written| unsafe {
        hc_pushkaram_by_sky(
            c"dhanus".as_ptr(),
            2019,
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
    assert_eq!(text, by_sign);
    let mut written = 0usize;
    assert_eq!(
        unsafe {
            hc_pushkarams_in_year(
                2019,
                c"lahiri".as_ptr(),
                core::ptr::null(),
                28.6356,
                77.2244,
                0.0,
                c"india".as_ptr(),
                c"en".as_ptr(),
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        unsafe {
            hc_pushkarams_in_year(
                3001,
                c"lahiri".as_ptr(),
                c"pushkaram-final-entry".as_ptr(),
                28.6356,
                77.2244,
                0.0,
                c"india".as_ptr(),
                c"en".as_ptr(),
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
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

/// The Maha Kumbh of 2025 at Prayag (`wikipedia-kumbh-mela`) is the one
/// condition of seven the sky meets; Drik Panchang's "Jupiter becomes
/// Retrograde" on 9 October 2024 at 12:33 IST (`drik-guru-retrograde`) is
/// the first of the two stations of the next five months.
#[test]
fn the_kumbhs_of_a_year_and_jupiters_stations_cross_the_c_boundary() {
    let rules = read_lines(|buffer, capacity, written| unsafe {
        hc_pushkaram_rules(buffer, capacity, written)
    });
    assert!(rules.starts_with("pushkaram-final-entry\t"));
    assert_eq!(rules.lines().count(), 2);
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_kumbhs_in_year_by_sky(
            2025,
            c"lahiri".as_ptr(),
            c"en".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    let met: Vec<&str> = rows(&text)
        .into_iter()
        .filter(|cells| cells[12] == "1")
        .map(|cells| cells[0])
        .collect();
    assert_eq!(met, ["kumbh-prayag-vrishabha"]);
    // 2024-09-01 and 2025-03-01, 00:00 UT.
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_jupiter_stations(
            1_725_148_800,
            1_740_787_200,
            c"lahiri".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    let stations = rows(&text);
    assert_eq!(stations.len(), 2);
    assert_eq!((stations[0][1], stations[1][1]), ("retrograde", "direct"));
    // 9 October 2024, 12:33 IST.
    let drik = 1_728_432_000 + 12 * 3_600 + 33 * 60 - 19_800;
    let found: i64 = stations[0][0].parse().expect("an instant");
    assert!((found - drik).abs() < 7 * 60, "{}", found - drik);
    let mut written = 0usize;
    assert_eq!(
        unsafe {
            hc_jupiter_stations(
                0,
                1,
                c"nope".as_ptr(),
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_UNKNOWN
    );
}
