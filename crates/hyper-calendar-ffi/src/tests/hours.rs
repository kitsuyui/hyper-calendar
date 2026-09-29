//! The prayer times, the zmanim and the Edo hours through the C
//! boundary.

use super::super::*;
use super::read_lines;

/// MUIS's Singapore timetable for 1 January 2026
/// (`muis-prayer-timetable-2026`), Hebcal's New York zmanim of
/// 1 January 2025 (`hebcal-zmanim-api`), and the Edo hour at Kyoto's
/// equinox dawn of 2020 (こよみのページ).
#[test]
fn the_hours_cross_the_c_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_prayer_times(
            c"singapore".as_ptr(),
            739_617,
            1.0 + 17.0 / 60.0,
            103.0 + 50.0 / 60.0,
            0.0,
            0,
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(text.lines().count(), 8);
    assert!(text.starts_with("fajr\t"));
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_prayer_methods(buffer, capacity, written)
    });
    assert!(text.lines().count() >= 11);
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_zmanim(
            c"mga-72-minutes".as_ptr(),
            739_252,
            40.71427,
            -74.00597,
            0.0,
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(text.lines().count(), 9);
    // The hour those times are counted in: Hebcal's 72-minute dawn 6:08
    // to its latest Shema 9:04 is three hours of 58⅔ minutes.
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_temporal_hour(
            c"mga-72-minutes".as_ptr(),
            739_252,
            40.71427,
            -74.00597,
            0.0,
            buffer,
            capacity,
            written,
        )
    });
    let cells: Vec<&str> = text.trim_end().split('\t').collect();
    assert_eq!(cells[0], "mga-72-minutes");
    let seconds: f64 = cells[1].parse().expect("a length");
    assert!((seconds - 3_520.0).abs() < 20.0, "{seconds}");
    let kyoto = (35.0 + 36.0 / 3_600.0, 135.7417);
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_unix_from_edo_time(
            737_504, 0, 0.0, kyoto.0, kyoto.1, 0.0, buffer, capacity, written,
        )
    });
    let dawn: i64 = text
        .split('\t')
        .next()
        .expect("a cell")
        .parse()
        .expect("an instant");
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_edo_time(dawn + 2, kyoto.0, kyoto.1, 0.0, buffer, capacity, written)
    });
    assert!(text.starts_with("737504\t0\t明六つ\t"), "{text}");
    let mut written = 0;
    assert_eq!(
        unsafe {
            hc_zmanim(
                core::ptr::null(),
                739_252,
                0.0,
                0.0,
                0.0,
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_NULL_POINTER
    );
}
