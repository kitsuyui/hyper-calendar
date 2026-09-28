//! Rāhu kālam and the almanac's cycles through the C boundary.

use super::super::*;
use super::read_lines;

/// The crescent criteria added with the Unified Hijri calendars:
/// Djamaluddin's Neo-MABIMS verdicts for 1447 AH in Indonesia
/// (`djamaluddin-kalender-1447`), met on the evening of 25 July 2025
/// and not on 25 June; KHGT's geocentric 8° and 5° at sunset, which
/// the Istanbul 2016 parameters also judge at; and Odeh's V at
/// Bruin's best time, as Yallop's q.
#[test]
fn the_crescent_criteria_of_the_unified_hijri_calendars_cross_the_c_boundary() {
    let line = |criterion: &core::ffi::CStr, day: i64, latitude: f64, longitude: f64| {
        let text = read_lines(|buffer, capacity, written| unsafe {
            hc_crescent_visible(
                criterion.as_ptr(),
                day,
                latitude,
                longitude,
                0.0,
                buffer,
                capacity,
                written,
            )
        });
        let cells: Vec<String> = text
            .trim_end_matches('\n')
            .split('\t')
            .map(str::to_owned)
            .collect();
        assert_eq!(cells.len(), 7, "{criterion:?}: {text}");
        cells
    };
    let indonesia = [(5.55, 95.3175), (-6.18, 106.83)];
    for criterion in [
        c"mabims-2021-topocentric",
        c"mabims-2021-geocentric-elongation",
    ] {
        let met = |day: i64| {
            indonesia
                .iter()
                .any(|&(latitude, longitude)| line(criterion, day, latitude, longitude)[0] == "1")
        };
        // 26 July and 26 June 2025, whose eves are the 25th.
        assert!(met(739_458), "{criterion:?}");
        assert!(!met(739_428), "{criterion:?}");
    }
    let mecca = (21.423_333, 39.823_333);
    // 18 to 20 February 2026.
    for day in 739_665..=739_667 {
        let khgt = line(c"khgt", day, mecca.0, mecca.1);
        let arc_of_light: f64 = khgt[3].parse().expect("an arc of light");
        let altitude: f64 = khgt[4].parse().expect("an altitude");
        assert_eq!(
            khgt[0] == "1",
            arc_of_light >= 8.0 && altitude >= 5.0,
            "{khgt:?}"
        );
        assert_eq!(line(c"istanbul-2016", day, mecca.0, mecca.1)[1], khgt[1]);
        assert_eq!(
            line(c"odeh", day, mecca.0, mecca.1)[1],
            line(c"yallop", day, mecca.0, mecca.1)[1]
        );
    }
}

/// Drik Panchang's New Delhi page of 1 January 2025, a Wednesday
/// (`drik-day-panchang-2025`), and 節分 2026, whose 恵方 is 丙
/// (`allabout-eho-2026`).
#[test]
fn kalam_and_almanac_cycles_cross_the_c_boundary() {
    let day = 739_252;
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_kalam(
            c"rahu-kalam-fixed".as_ptr(),
            day,
            28.6356,
            77.2244,
            0.0,
            core::ptr::null(),
            buffer,
            capacity,
            written,
        )
    });
    assert!(
        text.starts_with("rahu-kalam\tRahu Kalam\tRahu Kalam\ten\t5\tlocal\t43200\t48600\t"),
        "{text}"
    );
    let setsubun = 739_650;
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_almanac_cycles(setsubun, c"japan".as_ptr(), buffer, capacity, written)
    });
    assert!(text.starts_with("丙\thinoe\t165\t"), "{text}");
    // 21 December 2025, 赤口 with 天赦日 (`arachne-taian-2025-12`).
    let day = 739_606;
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_almanac_day(
            day,
            c"japan".as_ptr(),
            c"ja".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert!(text.contains("\nrokuyo\t6\t赤口\tja\t"), "{text}");
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_almanac_day(
            day,
            c"japan".as_ptr(),
            core::ptr::null(),
            buffer,
            capacity,
            written,
        )
    });
    assert!(text.contains("\nrokuyo\t6\tshakkō\ten\t"), "{text}");
    assert_eq!(
        unsafe {
            hc_almanac_day(
                day,
                core::ptr::null(),
                c"ja".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_NULL_POINTER
    );
}
