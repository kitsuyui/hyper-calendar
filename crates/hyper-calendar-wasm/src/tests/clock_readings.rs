//! .NET's ticks and the six-hour clocks, called as a page calls them.

use super::super::*;
use super::read_lines;

/// `DateTime.MaxValue` is 3 155 378 975 999 999 999 ticks
/// (`ms-datetime-maxvalue`); the UNDP page's "8 am is 2 o'clock"
/// (`undp-eue-ethiopian-time`).
#[test]
fn ticks_and_six_hour_clocks_cross_the_boundary() {
    assert_eq!(
        hc_dotnet_ticks_from_unix(253_402_300_799, 999_999_900_000_000_000),
        3_155_378_975_999_999_999
    );
    assert_eq!(
        hc_dotnet_ticks_from_unix(253_402_300_800, 0),
        HC_ERR_OUT_OF_RANGE
    );
    let text = read_lines(|buffer, capacity| unsafe {
        hc_unix_from_dotnet_ticks(621_355_968_000_000_000, buffer, capacity)
    });
    assert_eq!(text, "0\t0\n");
    let reckoning = "ethiopian-hours";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_six_hour_clock(
            reckoning.as_ptr(),
            reckoning.len(),
            8 * 3_600,
            buffer,
            capacity,
        )
    });
    assert_eq!(text, "2\t0\t0\tday\t\t\n");
    assert_eq!(
        unsafe { hc_civil_from_six_hour_clock(reckoning.as_ptr(), reckoning.len(), 2, 0, 0, 0) },
        8 * 3_600
    );
    assert_eq!(
        unsafe { hc_civil_from_six_hour_clock("x".as_ptr(), 1, 2, 0, 0, 0) },
        HC_ERR_UNKNOWN
    );
}
