//! The Smārta and Vaiṣṇava readings, Rāhu, the Viṣṭi-free span and the
//! eras' new years through the WebAssembly boundary.

use super::super::*;
use super::read_lines;

const TOKYO: (f64, f64) = (35.6894, 139.6917);
/// The Central Station of the national almanac, 82.5° E.
const STATION: (f64, f64) = (23.2, 82.5);

fn rows(text: &str) -> Vec<Vec<&str>> {
    text.lines()
        .map(|line| line.split('\t').collect())
        .collect()
}

/// Drik Panchang's ISKCON Janmashtami for Tokyo, 16 August 2025
/// (`drik-iskcon-janmashtami`), and the central government's list of 2025,
/// which keeps that day where the *Rashtriya Panchang*'s is the 15th.
#[test]
fn the_two_readings_of_janmashtami_cross_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe { hc_festival_readings(buffer, capacity) });
    let table = rows(&text);
    assert_eq!(table.len(), 2);
    assert_eq!((table[0][0], table[1][0]), ("smarta", "vaishnava"));
    assert!(table.iter().all(|row| row.len() == 4));
    let (vaishnava, smarta, lahiri) = ("vaishnava", "smarta", "lahiri");
    let text = read_lines(|buffer, capacity| unsafe {
        hc_janmashtami(
            2025,
            vaishnava.as_ptr(),
            vaishnava.len(),
            TOKYO.0,
            TOKYO.1,
            0.0,
            lahiri.as_ptr(),
            lahiri.len(),
            buffer,
            capacity,
        )
    });
    let row = &rows(&text)[0];
    assert_eq!(row.len(), 8);
    assert_eq!(row[0], "vaishnava");
    assert_eq!(row[2], hc_gregorian_to_fixed(2025, 8, 16).to_string());
    assert_eq!(row[3..6], ["2025", "8", "16"]);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_janmashtami(
            2025,
            smarta.as_ptr(),
            smarta.len(),
            STATION.0,
            STATION.1,
            0.0,
            lahiri.as_ptr(),
            lahiri.len(),
            buffer,
            capacity,
        )
    });
    assert_eq!(rows(&text)[0][3..6], ["2025", "8", "15"]);
    let unknown = "iskcon";
    assert_eq!(
        unsafe {
            hc_janmashtami(
                2025,
                unknown.as_ptr(),
                unknown.len(),
                TOKYO.0,
                TOKYO.1,
                0.0,
                lahiri.as_ptr(),
                lahiri.len(),
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_janmashtami(
                1699,
                smarta.as_ptr(),
                smarta.len(),
                TOKYO.0,
                TOKYO.1,
                0.0,
                lahiri.as_ptr(),
                lahiri.len(),
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_OUT_OF_RANGE
    );
}

#[test]
fn the_vaishnava_day_and_the_free_span_of_a_tithi_cross_the_boundary() {
    let lahiri = "lahiri";
    // Śrāvaṇa kṛṣṇa 8 of Śaka 1947 at Tokyo is Drik's 16 August 2025.
    let text = read_lines(|buffer, capacity| unsafe {
        hc_vaishnava_day(
            1947,
            5,
            23,
            TOKYO.0,
            TOKYO.1,
            0.0,
            lahiri.as_ptr(),
            lahiri.len(),
            buffer,
            capacity,
        )
    });
    let row = &rows(&text)[0];
    assert_eq!(row.len(), 9);
    assert_eq!(row[3], hc_gregorian_to_fixed(2025, 8, 16).to_string());
    assert_eq!(
        unsafe {
            hc_vaishnava_day(
                1947,
                5,
                31,
                TOKYO.0,
                TOKYO.1,
                0.0,
                lahiri.as_ptr(),
                lahiri.len(),
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_INVALID_DATE
    );
    // Bhadra of 19 August 2024 ended at 13:33 IST, 08:03 UT
    // (`onlinejyotish-rakhi-2024`, secondary).
    let text = read_lines(|buffer, capacity| unsafe {
        hc_vishti_free_span(
            1946,
            5,
            15,
            STATION.0,
            STATION.1,
            0.0,
            lahiri.as_ptr(),
            lahiri.len(),
            buffer,
            capacity,
        )
    });
    let row = &rows(&text)[0];
    assert_eq!(row.len(), 7);
    let from: i64 = row[4].parse().expect("an instant");
    let expected = hc_unix_from_fixed(hc_gregorian_to_fixed(2024, 8, 19)) + 8 * 3_600 + 3 * 60;
    assert!((from - expected).abs() <= 600, "{from}");
    assert_eq!(
        unsafe {
            hc_vishti_free_span(
                1946,
                13,
                15,
                STATION.0,
                STATION.1,
                0.0,
                lahiri.as_ptr(),
                lahiri.len(),
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_OUT_OF_RANGE
    );
}

/// Drik Panchang's mean Rāhu transit of 18 May 2025, 16:30 IST, into Kumbha
/// with its own ayanāṃśa (`drik-rahu-transit`).
#[test]
fn rahu_and_the_eras_new_years_cross_the_boundary() {
    let drik = "lahiri-drik";
    let after = hc_unix_from_fixed(hc_gregorian_to_fixed(2025, 5, 19)) + 12 * 3_600;
    let text = read_lines(|buffer, capacity| unsafe {
        hc_rahu_at(after, drik.as_ptr(), drik.len(), buffer, capacity)
    });
    let row = &rows(&text)[0];
    assert_eq!(row.len(), 11);
    assert_eq!((row[0], row[3], row[7]), ("mean", "kumbha", "simha"));
    let from = hc_unix_from_fixed(hc_gregorian_to_fixed(2025, 1, 1));
    let to = hc_unix_from_fixed(hc_gregorian_to_fixed(2026, 1, 1));
    let text = read_lines(|buffer, capacity| unsafe {
        hc_rahu_ingresses(from, to, drik.as_ptr(), drik.len(), buffer, capacity)
    });
    let table = rows(&text);
    assert_eq!(table.len(), 1);
    assert_eq!((table[0][3], table[0][5]), ("kumbha", "simha"));
    let moment: i64 = table[0][0].parse().expect("an instant");
    let drik_time = hc_unix_from_fixed(hc_gregorian_to_fixed(2025, 5, 18)) + 11 * 3_600;
    assert!((moment - drik_time).abs() <= 180, "{moment}");
    assert_eq!(
        unsafe {
            hc_rahu_ingresses(
                0,
                4_000_000_000,
                drik.as_ptr(),
                drik.len(),
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_OUT_OF_RANGE
    );
    // Chedi year 1 opens on 5 September 248 (Julian), Kielhorn's day.
    let chedi = "kalachuri";
    let julian_248 = hc::hc_calendars_solar::julian::to_fixed(248, 9, 5)
        .expect("a date")
        .0;
    assert_eq!(
        unsafe { hc_era_new_year(chedi.as_ptr(), chedi.len(), 1) },
        julian_248
    );
    let unknown = "hindu-lunar";
    assert_eq!(
        unsafe { hc_era_new_year(unknown.as_ptr(), unknown.len(), 1) },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe { hc_era_new_year(chedi.as_ptr(), chedi.len(), 100_000) },
        HC_ERR_OUT_OF_RANGE
    );
}
