//! The Tibetan almanac through the WebAssembly boundary.

use super::super::*;
use super::read_lines;

/// Henning's Tsurphu almanac's first day of 2013, 11 February
/// (`kalacakra-org`).
#[test]
fn the_lines_are_the_facades() {
    let calendar = "tibetan-tsurphu-karana";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_tibetan_almanac_day(calendar.as_ptr(), calendar.len(), 734_910, buffer, capacity)
    });
    assert!(text.starts_with("weekday\t3\tMonday\tzla ba\t2;11,24\t"));
    let text = read_lines(|buffer, capacity| unsafe {
        hc_bhutanese_winter_solstice(2001, buffer, capacity)
    });
    assert!(text.starts_with("730486\t2;51,38\t"));
    let (rule, phugpa) = ("henning-almanac", "tibetan-lochen");
    let none = unsafe {
        hc_tibetan_festival_day(
            rule.as_ptr(),
            rule.len(),
            phugpa.as_ptr(),
            phugpa.len(),
            1990,
            4,
            0,
            7,
        )
    };
    assert_eq!(none, HC_ERR_NO_DATA);
    let text =
        read_lines(|buffer, capacity| unsafe { hc_tibetan_planets(734_143, buffer, capacity) });
    assert_eq!(text.lines().count(), 5);
}
