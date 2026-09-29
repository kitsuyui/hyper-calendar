//! The Tibetan almanac through the C boundary.

use super::super::*;
use super::{measured, read_lines};

/// Henning's worked planets of 6 January 2011: Mars's particular day 525
/// (`kalacakra-org`, as `hc-calendars-regional`'s test reads it).
#[test]
fn the_lines_are_the_modules() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_tibetan_planets(734_143, buffer, capacity, written)
    });
    assert!(text.lines().any(|line| line.starts_with("mars\t525\t")));
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_tibetan_almanac_day(c"tibetan".as_ptr(), 734_910, buffer, capacity, written)
    });
    assert!(text.lines().any(|line| line.starts_with("rahu\t")));
    let mut day = 0i64;
    let status = unsafe {
        hc_tibetan_festival_day(
            c"berzin".as_ptr(),
            c"tibetan-lochen".as_ptr(),
            1990,
            4,
            0,
            7,
            &mut day,
        )
    };
    assert_eq!(status, HC_OK);
    let status = unsafe {
        hc_tibetan_festival_day(
            c"berzin".as_ptr(),
            c"tibetan".as_ptr(),
            1990,
            4,
            0,
            31,
            &mut day,
        )
    };
    assert_eq!(status, HC_ERROR_INVALID_DATE);
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_tibetan_almanac_day(c"x".as_ptr(), 734_910, buffer, capacity, written)
        }),
        HC_ERROR_UNKNOWN
    );
}
