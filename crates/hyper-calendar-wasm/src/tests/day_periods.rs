//! Rāhu kālam and the almanac's cycles, called as a page calls them.

use super::super::*;
use super::read_lines;

/// Drik Panchang's New Delhi page of 1 January 2025
/// (`drik-day-panchang-2025`): a Wednesday, whose Rāhu kālam is the
/// fifth eighth, 12:00 to 13:30 on the fixed day.
#[test]
fn a_wednesday_rahu_kalam_is_the_fifth_eighth() {
    let convention = "rahu-kalam-fixed";
    let day = hc_gregorian_to_fixed(2025, 1, 1);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_kalam(
            convention.as_ptr(),
            convention.len(),
            day,
            28.6356,
            77.2244,
            0.0,
            "hi".as_ptr(),
            2,
            buffer,
            capacity,
        )
    });
    let first = text.lines().next().expect("three lines");
    assert_eq!(
        first,
        "rahu-kalam\tRahu Kalam\tराहुकाल\thi\t5\tlocal\t43200\t48600\t\t\t\t"
    );
    assert_eq!(text.lines().count(), 3);
}

/// The crescent criteria added with the Unified Hijri calendars.
/// Djamaluddin's analysis of 1447 AH by Neo-MABIMS
/// (`djamaluddin-kalender-1447`): met in Indonesia, at Banda Aceh or
/// Jakarta, on the evening of 25 July 2025 and not on 25 June. KHGT
/// and the Istanbul 2016 parameters judge at sunset, and KHGT's
/// verdict is its two geocentric thresholds, an elongation of 8° and
/// an altitude of 5°; Odeh's V is judged at Bruin's best time, as
/// Yallop's q is.
#[test]
fn the_crescent_criteria_of_the_unified_hijri_calendars_cross_the_boundary() {
    let line = |criterion: &str, day: i64, latitude: f64, longitude: f64| -> Vec<String> {
        let text = read_lines(|buffer, capacity| unsafe {
            hc_crescent_visible(
                criterion.as_ptr(),
                criterion.len(),
                day,
                latitude,
                longitude,
                0.0,
                buffer,
                capacity,
            )
        });
        let cells: Vec<String> = text
            .trim_end_matches('\n')
            .split('\t')
            .map(str::to_owned)
            .collect();
        assert_eq!(cells.len(), 7, "{criterion}: {text}");
        cells
    };
    let indonesia = [(5.55, 95.3175), (-6.18, 106.83)];
    for criterion in [
        "mabims-2021-topocentric",
        "mabims-2021-geocentric-elongation",
    ] {
        let met = |day: i64| {
            indonesia
                .iter()
                .any(|&(latitude, longitude)| line(criterion, day, latitude, longitude)[0] == "1")
        };
        assert!(met(hc_gregorian_to_fixed(2025, 7, 26)), "{criterion}");
        assert!(!met(hc_gregorian_to_fixed(2025, 6, 26)), "{criterion}");
    }
    let mecca = (21.423_333, 39.823_333);
    for date in [18, 19, 20] {
        let day = hc_gregorian_to_fixed(2026, 2, date);
        let khgt = line("khgt", day, mecca.0, mecca.1);
        let arc_of_light: f64 = khgt[3].parse().expect("an arc of light");
        let altitude: f64 = khgt[4].parse().expect("an altitude");
        assert_eq!(
            khgt[0] == "1",
            arc_of_light >= 8.0 && altitude >= 5.0,
            "{khgt:?}"
        );
        let istanbul = line("istanbul-2016", day, mecca.0, mecca.1);
        assert_eq!(istanbul[1], khgt[1], "both judge at sunset");
        let odeh = line("odeh", day, mecca.0, mecca.1);
        let yallop = line("yallop", day, mecca.0, mecca.1);
        assert_eq!(odeh[1], yallop[1], "both judge at Bruin's best time");
    }
}

/// 節分 2026 faces 丙, 南南東 (`allabout-eho-2026`), in 九運.
#[test]
fn setsubun_2026_crosses_the_boundary() {
    let meridian = "japan";
    let day = hc_gregorian_to_fixed(2026, 2, 3);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_almanac_cycles(day, meridian.as_ptr(), meridian.len(), buffer, capacity)
    });
    assert!(
        text.starts_with("丙\thinoe\t165\t南南東\tsouth-south-east\t9\t九運\t"),
        "{text}"
    );
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_almanac_cycles(day, "mars".as_ptr(), 4, null, 0) },
        HC_ERR_UNKNOWN
    );
}

/// 21 December 2025 is 赤口 with 天赦日 and 一粒万倍日
/// (`arachne-taian-2025-12`), in Japanese under `ja` and in the
/// Hepburn reading under `en`.
#[test]
fn the_almanac_day_crosses_the_boundary() {
    let meridian = "japan";
    let day = hc_gregorian_to_fixed(2025, 12, 21);
    let lines = |locale: &str| {
        read_lines(|buffer, capacity| unsafe {
            hc_almanac_day(
                day,
                meridian.as_ptr(),
                meridian.len(),
                locale.as_ptr(),
                locale.len(),
                buffer,
                capacity,
            )
        })
    };
    let japanese = lines("ja");
    assert!(
        japanese.contains("\nrokuyo\t6\t赤口\tja\t赤口\tshakkō\t\t\n"),
        "{japanese}"
    );
    assert!(japanese.contains("\nlower-register\ttenshanichi\t天赦日\tja\t"));
    assert!(lines("en").contains("\nrokuyo\t6\tshakkō\ten\t赤口\t"));
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_almanac_day(day, "mars".as_ptr(), 4, "ja".as_ptr(), 2, null, 0) },
        HC_ERR_UNKNOWN
    );
}
