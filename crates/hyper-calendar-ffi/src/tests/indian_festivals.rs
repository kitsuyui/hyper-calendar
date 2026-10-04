//! The Smārta and Vaiṣṇava readings, Rāhu, the Viṣṭi-free span and the
//! eras' new years through the C boundary.

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

fn fixed(year: i64, month: u8, day: u8) -> i64 {
    let mut fixed = 0;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(year, month, day, &mut fixed) },
        HC_OK
    );
    fixed
}

/// Midnight UTC of a Gregorian date, in POSIX seconds.
fn unix(year: i64, month: u8, day: u8) -> i64 {
    (fixed(year, month, day) - 719_163) * 86_400
}

/// Drik Panchang's ISKCON Janmashtami for Tokyo, 16 August 2025
/// (`drik-iskcon-janmashtami`), and the central government's list of 2025,
/// which keeps that day where the *Rashtriya Panchang*'s is the 15th.
#[test]
fn the_two_readings_of_janmashtami_cross_the_c_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_festival_readings(buffer, capacity, written)
    });
    let table = rows(&text);
    assert_eq!((table[0][0], table[1][0]), ("smarta", "vaishnava"));
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_janmashtami(
            2025,
            c"vaishnava".as_ptr(),
            TOKYO.0,
            TOKYO.1,
            0.0,
            c"lahiri".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(rows(&text)[0][3..6], ["2025", "8", "16"]);
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_janmashtami(
            2025,
            c"smarta".as_ptr(),
            STATION.0,
            STATION.1,
            0.0,
            c"lahiri".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(rows(&text)[0][3..6], ["2025", "8", "15"]);
    let mut written = 0usize;
    assert_eq!(
        unsafe {
            hc_janmashtami(
                2025,
                c"iskcon".as_ptr(),
                TOKYO.0,
                TOKYO.1,
                0.0,
                c"lahiri".as_ptr(),
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_janmashtami(
                2300,
                c"smarta".as_ptr(),
                TOKYO.0,
                TOKYO.1,
                0.0,
                c"lahiri".as_ptr(),
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe {
            hc_janmashtami(
                2025,
                core::ptr::null(),
                TOKYO.0,
                TOKYO.1,
                0.0,
                c"lahiri".as_ptr(),
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_NULL_POINTER
    );
}

#[test]
fn the_vaishnava_day_and_the_free_span_of_a_tithi_cross_the_c_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_vaishnava_day(
            1947,
            5,
            23,
            TOKYO.0,
            TOKYO.1,
            0.0,
            c"lahiri".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(rows(&text)[0][3], fixed(2025, 8, 16).to_string());
    let mut written = 0usize;
    assert_eq!(
        unsafe {
            hc_vaishnava_day(
                1947,
                5,
                31,
                TOKYO.0,
                TOKYO.1,
                0.0,
                c"lahiri".as_ptr(),
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_INVALID_DATE
    );
    // Bhadra of 19 August 2024 ended at 13:33 IST, 08:03 UT
    // (`onlinejyotish-rakhi-2024`, secondary).
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_vishti_free_span(
            1946,
            5,
            15,
            STATION.0,
            STATION.1,
            0.0,
            c"lahiri".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    let from: i64 = rows(&text)[0][4].parse().expect("an instant");
    assert!((from - (unix(2024, 8, 19) + 8 * 3_600 + 3 * 60)).abs() <= 600);
}

/// Drik Panchang's mean Rāhu transit of 18 May 2025, 16:30 IST, into Kumbha
/// with its own ayanāṃśa (`drik-rahu-transit`).
#[test]
fn rahu_and_the_eras_new_years_cross_the_c_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_rahu_at(
            unix(2025, 5, 19) + 12 * 3_600,
            c"lahiri-drik".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    let row = &rows(&text)[0];
    assert_eq!((row[0], row[3], row[7]), ("mean", "kumbha", "simha"));
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_rahu_ingresses(
            unix(2025, 1, 1),
            unix(2026, 1, 1),
            c"lahiri-drik".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    let table = rows(&text);
    assert_eq!(table.len(), 1);
    let moment: i64 = table[0][0].parse().expect("an instant");
    assert!((moment - (unix(2025, 5, 18) + 11 * 3_600)).abs() <= 180);
    let mut written = 0usize;
    assert_eq!(
        unsafe {
            hc_rahu_ingresses(
                0,
                4_000_000_000,
                c"lahiri".as_ptr(),
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
    // Chedi year 1 opens on 5 September 248 (Julian), Kielhorn's day.
    let mut out = 0i64;
    assert_eq!(
        unsafe { hc_era_new_year(c"kalachuri".as_ptr(), 1, &mut out) },
        HC_OK
    );
    assert_eq!(
        out,
        hc::hc_calendars_solar::julian::to_fixed(248, 9, 5)
            .expect("a date")
            .0
    );
    assert_eq!(
        unsafe { hc_era_new_year(c"hindu-lunar".as_ptr(), 1, &mut out) },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe { hc_era_new_year(c"gupta".as_ptr(), 1, core::ptr::null_mut()) },
        HC_ERROR_NULL_POINTER
    );
}

/// Thai 1 of 2021, 14 January, began Tiruvaḷḷuvar 2052, and the Sun's
/// entries into the nakṣatras are the module's lines, 27 in 2025.
#[test]
fn the_tiruvalluvar_year_and_the_suns_nakshatra_entries_cross_the_c_boundary() {
    let mut year = 0i64;
    assert_eq!(
        unsafe { hc_tiruvalluvar_year(fixed(2021, 1, 14), &mut year) },
        HC_OK
    );
    assert_eq!(year, 2052);
    assert_eq!(
        unsafe { hc_tiruvalluvar_year(fixed(2021, 1, 13), &mut year) },
        HC_OK
    );
    assert_eq!(year, 2051);
    assert_eq!(
        unsafe { hc_tiruvalluvar_year(fixed(1699, 12, 31), &mut year) },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_tiruvalluvar_year(fixed(2021, 1, 14), core::ptr::null_mut()) },
        HC_ERROR_NULL_POINTER
    );
    let (from, to) = (unix(2025, 1, 1), unix(2026, 1, 1));
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_solar_nakshatra_ingresses(from, to, c"lahiri".as_ptr(), buffer, capacity, written)
    });
    assert_eq!(
        Ok(text.clone()),
        hc::panchanga_lines::solar_nakshatra_ingresses_lines(from, to, "lahiri")
    );
    assert_eq!(rows(&text).len(), 27);
    let mut written = 0usize;
    for (ayanamsa, status) in [
        (c"x".as_ptr(), HC_ERROR_UNKNOWN),
        (core::ptr::null(), HC_ERROR_NULL_POINTER),
    ] {
        assert_eq!(
            unsafe {
                hc_solar_nakshatra_ingresses(
                    from,
                    to,
                    ayanamsa,
                    core::ptr::null_mut(),
                    0,
                    &mut written,
                )
            },
            status
        );
    }
    assert_eq!(
        unsafe {
            hc_solar_nakshatra_ingresses(
                0,
                4_000_000_000,
                c"lahiri".as_ptr(),
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
}
