//! The planetary hours and GMAT, called as a page calls them.

use super::super::*;
use super::read_lines;

/// Twenty-four hours of Monday 25 March 1647 at London, the Moon's
/// first (`lilly-christian-astrology-1647`); and the 1924 almanac's
/// eclipse of 20 February at 4ʰ 12ᵐ 25ˢ from noon, 16:12:25 civil
/// (`nautical-almanac-1924`).
#[test]
fn planetary_hours_and_gmat_cross_the_boundary() {
    let locale = "en";
    let day = hc_gregorian_to_fixed(1647, 3, 25);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_planetary_hours_of_day(
            day,
            51.5,
            -0.1,
            0.0,
            locale.as_ptr(),
            locale.len(),
            buffer,
            capacity,
        )
    });
    assert_eq!(text.lines().count(), 24);
    let first: Vec<&str> = text.lines().next().expect("a line").split('\t').collect();
    assert_eq!(
        first[..6],
        [&*day.to_string(), "1", "moon", "Moon", "en", "1"]
    );
    let start: i64 = first[6].parse().expect("an instant");
    let text = read_lines(|buffer, capacity| unsafe {
        hc_planetary_hour(
            start + 60,
            51.5,
            -0.1,
            0.0,
            locale.as_ptr(),
            locale.len(),
            buffer,
            capacity,
        )
    });
    assert_eq!(text.split('\t').nth(2), Some("moon"));

    let eclipse = hc_gregorian_to_fixed(1924, 2, 20);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_gmat_from_gmt(eclipse, 16 * 3_600 + 12 * 60 + 25, 0, buffer, capacity)
    });
    assert_eq!(
        text,
        format!("{eclipse}\t{}\t0\n", 4 * 3_600 + 12 * 60 + 25)
    );
    let text = read_lines(|buffer, capacity| unsafe {
        hc_gmt_from_gmat(eclipse, 4 * 3_600 + 12 * 60 + 25, 0, buffer, capacity)
    });
    assert_eq!(
        text,
        format!("{eclipse}\t{}\t0\n", 16 * 3_600 + 12 * 60 + 25)
    );
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_gmat_from_gmt(eclipse, 86_400, 0, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
}
