//! The planetary hours and GMAT, across the C boundary.

use super::super::*;
use super::read_lines;

/// Monday 25 March 1647 at London, the Moon's first hour
/// (`lilly-christian-astrology-1647`); the 1924 almanac's eclipse at
/// 4ʰ 12ᵐ 25ˢ from noon (`nautical-almanac-1924`).
#[test]
fn planetary_hours_and_gmat_cross_the_c_boundary() {
    let day = 601_273;
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_planetary_hours_of_day(
            day,
            51.5,
            -0.1,
            0.0,
            c"en".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(text.lines().count(), 24);
    assert!(text.starts_with("601273\t1\tmoon\tMoon\ten\t1\t"), "{text}");
    let start: i64 = text
        .split('\t')
        .nth(6)
        .expect("a cell")
        .parse()
        .expect("an instant");
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_planetary_hour(
            start + 60,
            51.5,
            -0.1,
            0.0,
            core::ptr::null(),
            buffer,
            capacity,
            written,
        )
    });
    assert!(text.starts_with("601273\t1\tmoon\t"), "{text}");
    let eclipse = 702_411;
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_gmat_from_gmt(eclipse, 58_345, 0, buffer, capacity, written)
    });
    assert_eq!(text, "702411\t15145\t0\n");
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_gmt_from_gmat(eclipse, 15_145, 0, buffer, capacity, written)
    });
    assert_eq!(text, "702411\t58345\t0\n");
}
