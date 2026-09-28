//! .NET's ticks and the six-hour clocks, called as a C caller calls them.

use super::super::*;
use super::read_lines;

/// `DateTime.MaxValue` is 3 155 378 975 999 999 999 ticks
/// (`ms-datetime-maxvalue`); the Kansas lesson's "7:00 pm is called
/// saa moja usiku" (`ku-kiswahili-lesson-17`).
#[test]
fn ticks_and_six_hour_clocks_cross_the_c_boundary() {
    let mut ticks = 0i64;
    assert_eq!(
        unsafe { hc_dotnet_ticks_from_unix(253_402_300_799, 999_999_900_000_000_000, &mut ticks) },
        HC_OK
    );
    assert_eq!(ticks, 3_155_378_975_999_999_999);
    assert_eq!(
        unsafe { hc_dotnet_ticks_from_unix(253_402_300_800, 0, &mut ticks) },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_dotnet_ticks_from_unix(0, 0, core::ptr::null_mut()) },
        HC_ERROR_NULL_POINTER
    );
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_unix_from_dotnet_ticks(0, buffer, capacity, written)
    });
    assert_eq!(text, "-62135596800\t0\n");
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_six_hour_clock(
            c"swahili-hours".as_ptr(),
            19 * 3_600,
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(text, "1\t0\t0\tnight\tusiku\tnight\n");
    let mut seconds = 0u32;
    assert_eq!(
        unsafe {
            hc_civil_from_six_hour_clock(c"swahili-hours".as_ptr(), 1, 0, 0, 1, &mut seconds)
        },
        HC_OK
    );
    assert_eq!(seconds, 19 * 3_600);
}
