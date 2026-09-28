//! The Orthodox fasts, called as a page calls them.

use super::super::*;
use super::read_lines;

/// Great Lent began on 3 March 2025 (`oca-fasting-seasons`, as
/// `docs/systems/orthodox-fasts.md` works it).
#[test]
fn great_lent_2025_crosses_the_boundary() {
    let reckoning = "orthodox-fasts";
    let day = hc_gregorian_to_fixed(2025, 3, 3);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_orthodox_fast_on(reckoning.as_ptr(), reckoning.len(), day, buffer, capacity)
    });
    assert_eq!(
        text,
        "1\tperiod\tgreat-lent\tGreat Lent & Holy Week\tfast\tfast\n"
    );
    // Wednesday 18 February 2026, in Cheesefare week.
    let cheesefare = hc_gregorian_to_fixed(2026, 2, 18);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_orthodox_fast_on(
            reckoning.as_ptr(),
            reckoning.len(),
            cheesefare,
            buffer,
            capacity,
        )
    });
    assert_eq!(text, "0\tperiod\tmeatfast\tMeatfast\tmeat-excluded\tmeat\n");
    let text = read_lines(|buffer, capacity| unsafe {
        hc_orthodox_fast_seasons(reckoning.as_ptr(), reckoning.len(), 2025, buffer, capacity)
    });
    assert!(
        text.contains(&format!("\tfast\t{day}\t{}\n", day + 47)),
        "{text}"
    );
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_orthodox_fast_seasons(reckoning.as_ptr(), reckoning.len(), 5000, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
}
