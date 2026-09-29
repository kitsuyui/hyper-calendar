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

/// Each Oriental reckoning through the C boundary, as the WebAssembly
/// module's test takes it: the Armenian Great Lent of 2026, the Jerusalem
/// Catechumens, the forty and the fifty days after Easter, and the Nativity
/// and Genna of 7 January 2024.
#[test]
fn each_oriental_reckoning_crosses_the_c_boundary() {
    let line = |reckoning: &core::ffi::CStr, day: i64| {
        read_lines(|buffer, capacity, written| unsafe {
            hc_orthodox_fast_on(reckoning.as_ptr(), day, buffer, capacity, written)
        })
    };
    let day = |year, month, date| {
        let mut fixed = 0i64;
        assert_eq!(
            unsafe { hc_gregorian_to_fixed(year, month, date, &mut fixed) },
            HC_OK
        );
        fixed
    };
    assert_eq!(
        line(c"armenian-fasts", day(2026, 2, 16)),
        "1\tperiod\tgreat-lent\tGreat Lent and Holy Week\tfast\tfast\n"
    );
    assert_eq!(
        line(c"armenian-fasts-jerusalem", day(2026, 2, 2)),
        "1\tperiod\tcatechumens-fast\tFast of the Catechumens (Aradjavorats)\tfast\tfast\n"
    );
    assert_eq!(
        line(c"armenian-fasts", day(2026, 5, 20)),
        "1\tweekly-fast\t\t\t\tfast\n"
    );
    assert_eq!(
        line(c"armenian-fasts-fifty-days", day(2026, 5, 20)),
        "0\tperiod\teaster-to-pentecost\tThe fifty days after Easter\tfast-free\tnothing\n"
    );
    assert_eq!(
        line(c"coptic-fasts", day(2024, 1, 7)),
        "0\tperiod\tnativity-feast\tThe Nativity\tfast-free\tnothing\n"
    );
    assert_eq!(
        line(c"ethiopian-fasts", day(2024, 1, 7)),
        "0\tperiod\tgenna\tGenna\tfast-free\tnothing\n"
    );
    assert_eq!(
        line(c"ethiopian-fasts", day(2024, 1, 8)),
        "0\tnone\t\t\t\tnothing\n"
    );
    let mut written = 0;
    assert_eq!(
        unsafe {
            hc_orthodox_fast_on(
                c"armenian-fasts".as_ptr(),
                day(1582, 12, 31),
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
}
