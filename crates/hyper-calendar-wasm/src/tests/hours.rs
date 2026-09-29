//! The prayer times, the zmanim and the Edo hours, called as a page
//! calls them.

use super::super::*;
use super::read_lines;

/// MUIS's timetable for Singapore, 1 January 2026
/// (`muis-prayer-timetable-2026`): Subuh 5:44, UTC+8, the computed
/// *fajr* up to a minute and a half earlier.
#[test]
fn singapore_prayer_times_cross_the_boundary() {
    let method = "singapore";
    let day = hc_gregorian_to_fixed(2026, 1, 1);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_prayer_times(
            method.as_ptr(),
            method.len(),
            day,
            1.0 + 17.0 / 60.0,
            103.0 + 50.0 / 60.0,
            0.0,
            0,
            buffer,
            capacity,
        )
    });
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert_eq!(rows.len(), 8);
    assert_eq!(rows[0][0], "fajr");
    let fajr: i64 = rows[0][1].parse().expect("an instant");
    let subuh = hc_unix_from_fixed(day) + (5 * 60 + 44 - 8 * 60) * 60;
    assert!((0..=90).contains(&(subuh - fajr)), "{}", subuh - fajr);
    let text = read_lines(|buffer, capacity| unsafe { hc_prayer_methods(buffer, capacity) });
    assert!(text.lines().any(|line| line.starts_with("singapore\t")));
}

/// Hebcal's zmanim for New York City, 1 January 2025
/// (`hebcal-zmanim-api`): the latest Shema by the GRA at 9:40 EST.
#[test]
fn new_york_zmanim_cross_the_boundary() {
    let reckoning = "zmanim-gra";
    let day = hc_gregorian_to_fixed(2025, 1, 1);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_zmanim(
            reckoning.as_ptr(),
            reckoning.len(),
            day,
            40.71427,
            -74.00597,
            0.0,
            buffer,
            capacity,
        )
    });
    assert_eq!(text.lines().count(), 9);
    let first: Vec<&str> = text.lines().next().expect("a line").split('\t').collect();
    let at: i64 = first[3].parse().expect("an instant");
    let printed = hc_unix_from_fixed(day) + (9 * 60 + 40 + 5 * 60) * 60;
    assert!((at - printed).abs() < 60, "{}", at - printed);
    // Hebcal's sunrise 7:20 and sunset 16:40: an hour of 46⅔ minutes.
    let text = read_lines(|buffer, capacity| unsafe {
        hc_temporal_hour(
            reckoning.as_ptr(),
            reckoning.len(),
            day,
            40.71427,
            -74.00597,
            0.0,
            buffer,
            capacity,
        )
    });
    let seconds: f64 = text
        .split('\t')
        .nth(1)
        .expect("a cell")
        .parse()
        .expect("a length");
    assert!((seconds - 2_800.0).abs() < 20.0, "{seconds}");
}

/// Kyoto, 20 March 2020: 夜明 at 5:28:47 JST (こよみのページ), and
/// the Edo reading there is 明六つ.
#[test]
fn kyoto_dawn_crosses_the_boundary() {
    let event = "japanese-dawn-naoj";
    let day = hc_gregorian_to_fixed(2020, 3, 20);
    let (latitude, longitude) = (35.0 + 36.0 / 3_600.0, 135.7417);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_solar_event(
            event.as_ptr(),
            event.len(),
            day,
            latitude,
            longitude,
            0.0,
            buffer,
            capacity,
        )
    });
    let dawn: i64 = text
        .split('\t')
        .next()
        .expect("a cell")
        .parse()
        .expect("an instant");
    let printed = hc_unix_from_fixed(day) + 5 * 3_600 + 28 * 60 + 47 - 9 * 3_600;
    assert!((dawn - printed).abs() < 10, "{}", dawn - printed);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_edo_time(dawn + 2, latitude, longitude, 0.0, buffer, capacity)
    });
    assert!(text.starts_with(&format!("{day}\t0\t明六つ\t")), "{text}");
    let text = read_lines(|buffer, capacity| unsafe {
        hc_unix_from_edo_time(day, 0, 0.0, latitude, longitude, 0.0, buffer, capacity)
    });
    let back: i64 = text
        .split('\t')
        .next()
        .expect("a cell")
        .parse()
        .expect("an instant");
    assert!((back - dawn).abs() <= 1);
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_unix_from_edo_time(day, 12, 0.0, latitude, longitude, 0.0, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
}
