use super::super::*;
use super::read_lines;

#[test]
fn the_term_and_pentad_in_effect_decode_column_by_column() {
    let mut day = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2024, 2, 10, &mut day) },
        HC_OK
    );
    let term = read_lines(|buffer, capacity, written| unsafe {
        hc_term_in_effect(day, c"japan".as_ptr(), buffer, capacity, written)
    });
    let columns: Vec<&str> = term.trim_end().split('\t').collect();
    assert_eq!(columns.len(), 7, "{columns:?}");
    assert_eq!(columns[..3], ["21", "立春", "立春"]);
    assert_eq!(columns[3], (day - 6).to_string());
    assert_eq!(columns[4], (day + 8).to_string());
    let pentad = read_lines(|buffer, capacity, written| unsafe {
        hc_pentad_in_effect(day, core::ptr::null(), buffer, capacity, written)
    });
    let columns: Vec<&str> = pentad.trim_end().split('\t').collect();
    assert_eq!(columns.len(), 7, "{columns:?}");
    assert_eq!(columns[..3], ["64", "蟄蟲始振", "黄鶯睍睆"]);
    assert_eq!(
        unsafe {
            hc_term_in_effect(
                day,
                c"mars".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_pentad_in_effect(
                day,
                c"\xff".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_NOT_UTF8
    );
}

/// KASI's 월력요항: 한식 on 6 April 2026.
#[test]
fn the_cold_food_day_crosses_through_an_out_parameter() {
    let (mut day, mut expected) = (0i64, 0i64);
    assert_eq!(
        unsafe { hc_cold_food_day(c"hansik".as_ptr(), 2026, &mut day) },
        HC_OK
    );
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2026, 4, 6, &mut expected) },
        HC_OK
    );
    assert_eq!(day, expected);
    assert_eq!(
        unsafe { hc_cold_food_day(c"hanshi".as_ptr(), 2026, &mut day) },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe { hc_cold_food_day(c"hansik".as_ptr(), 3001, &mut day) },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_cold_food_day(core::ptr::null(), 2026, &mut day) },
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        unsafe { hc_cold_food_day(c"hansik".as_ptr(), 2026, core::ptr::null_mut()) },
        HC_ERROR_NULL_POINTER
    );
}

/// The days of the years −1000 to 3000 answer, as `hc_sky_at`'s do;
/// the days either side of them are refused.
#[test]
fn the_term_and_pentad_refuse_days_outside_the_era() {
    let (mut first, mut last) = (0i64, 0i64);
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(-1000, 1, 1, &mut first) },
        HC_OK
    );
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(3000, 12, 31, &mut last) },
        HC_OK
    );
    let null = core::ptr::null_mut();
    for day in [first, last] {
        let mut written = 0usize;
        assert_eq!(
            unsafe { hc_term_in_effect(day, core::ptr::null(), null, 0, &mut written) },
            HC_ERROR_BUFFER_TOO_SMALL
        );
        assert!(written > 0);
    }
    for day in [first - 1, last + 1, i64::MIN, i64::MAX] {
        for status in [
            unsafe { hc_term_in_effect(day, core::ptr::null(), null, 0, null.cast()) },
            unsafe { hc_pentad_in_effect(day, core::ptr::null(), null, 0, null.cast()) },
        ] {
            assert_eq!(status, HC_ERROR_OUT_OF_RANGE, "{day}");
        }
    }
}
