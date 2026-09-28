//! The Orthodox fasts through the C boundary.

use super::super::*;
use super::read_lines;

/// Great Lent began on 3 March 2025 (`oca-fasting-seasons`).
#[test]
fn great_lent_2025_crosses_the_c_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_orthodox_fast_on(
            c"orthodox-fasts".as_ptr(),
            739_313,
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(
        text,
        "1\tperiod\tgreat-lent\tGreat Lent & Holy Week\tfast\tfast\n"
    );
    // Wednesday 18 February 2026, in Cheesefare week.
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_orthodox_fast_on(
            c"orthodox-fasts".as_ptr(),
            739_665,
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(text, "0\tperiod\tmeatfast\tMeatfast\tmeat-excluded\tmeat\n");
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_orthodox_fast_seasons(c"orthodox-fasts".as_ptr(), 2025, buffer, capacity, written)
    });
    assert_eq!(text.lines().count(), 12);
    let mut written = 0;
    assert_eq!(
        unsafe {
            hc_orthodox_fast_seasons(
                c"coptic".as_ptr(),
                2025,
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_UNKNOWN
    );
}
