use super::super::*;

const TZIF_V2_EASTERN: &[u8] = &[
    0x54, 0x5a, 0x69, 0x66, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01,
    0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x08, 0x65, 0xed, 0x5a, 0x70,
    0x67, 0x27, 0x11, 0x60, 0x67, 0xcd, 0x3c, 0x70, 0x69, 0x06, 0xf3, 0x60, 0x01, 0x00, 0x01, 0x00,
    0xff, 0xff, 0xb9, 0xb0, 0x00, 0x00, 0xff, 0xff, 0xc7, 0xc0, 0x01, 0x04, 0x45, 0x53, 0x54, 0x00,
    0x45, 0x44, 0x54, 0x00, 0x58, 0x68, 0x46, 0x80, 0x00, 0x00, 0x00, 0x1b, 0x00, 0x00, 0x00, 0x00,
    0x54, 0x5a, 0x69, 0x66, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01,
    0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00,
    0x65, 0xed, 0x5a, 0x70, 0x00, 0x00, 0x00, 0x00, 0x67, 0x27, 0x11, 0x60, 0x00, 0x00, 0x00, 0x00,
    0x67, 0xcd, 0x3c, 0x70, 0x00, 0x00, 0x00, 0x00, 0x69, 0x06, 0xf3, 0x60, 0x01, 0x00, 0x01, 0x00,
    0xff, 0xff, 0xb9, 0xb0, 0x00, 0x00, 0xff, 0xff, 0xc7, 0xc0, 0x01, 0x04, 0x45, 0x53, 0x54, 0x00,
    0x45, 0x44, 0x54, 0x00, 0x00, 0x00, 0x00, 0x00, 0x58, 0x68, 0x46, 0x80, 0x00, 0x00, 0x00, 0x1b,
    0x00, 0x00, 0x00, 0x00, 0x0a, 0x45, 0x53, 0x54, 0x35, 0x45, 0x44, 0x54, 0x2c, 0x4d, 0x33, 0x2e,
    0x32, 0x2e, 0x30, 0x2c, 0x4d, 0x31, 0x31, 0x2e, 0x31, 0x2e, 0x30, 0x0a,
];

/// America/Denver as a TZif file: MST and MDT, the two transitions
/// of 2026, 8 March 09:00 UTC and 1 November 08:00 UTC, and the
/// footer `MST7MDT,M3.2.0,M11.1.0`, which is tzdata 2026d's
/// (unchanged since 2026c).
const TZIF_V2_DENVER: &[u8] = &[
    0x54, 0x5a, 0x69, 0x66, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x08, 0x69, 0xad, 0x3a, 0x90,
    0x6a, 0xe6, 0xf1, 0x80, 0x01, 0x00, 0xff, 0xff, 0x9d, 0x90, 0x00, 0x00, 0xff, 0xff, 0xab, 0xa0,
    0x01, 0x04, 0x4d, 0x53, 0x54, 0x00, 0x4d, 0x44, 0x54, 0x00, 0x54, 0x5a, 0x69, 0x66, 0x32, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00,
    0x00, 0x02, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x69, 0xad, 0x3a, 0x90, 0x00, 0x00,
    0x00, 0x00, 0x6a, 0xe6, 0xf1, 0x80, 0x01, 0x00, 0xff, 0xff, 0x9d, 0x90, 0x00, 0x00, 0xff, 0xff,
    0xab, 0xa0, 0x01, 0x04, 0x4d, 0x53, 0x54, 0x00, 0x4d, 0x44, 0x54, 0x00, 0x0a, 0x4d, 0x53, 0x54,
    0x37, 0x4d, 0x44, 0x54, 0x2c, 0x4d, 0x33, 0x2e, 0x32, 0x2e, 0x30, 0x2c, 0x4d, 0x31, 0x31, 0x2e,
    0x31, 0x2e, 0x30, 0x0a,
];

fn offset(zone: &core::ffi::CStr, unix: i64) -> String {
    super::read_lines(|buffer, capacity, written| unsafe {
        hc_zone_offset(zone.as_ptr(), unix, buffer, capacity, written)
    })
}

fn load_denver() {
    assert_eq!(
        unsafe {
            hc_zone_load(
                c"America/Denver".as_ptr(),
                TZIF_V2_DENVER.as_ptr(),
                TZIF_V2_DENVER.len(),
            )
        },
        HC_OK
    );
}

/// The 2026 changes of Europe/Berlin, built in, and America/Denver,
/// loaded: the WebAssembly module's lines.
#[test]
fn the_offset_at_each_change_of_2026() {
    load_denver();
    let cases = [
        (
            c"Europe/Berlin",
            1_774_745_999,
            "3600\t0\tCET\t1774746000\t7200\tbuiltin\n",
        ),
        (
            c"Europe/Berlin",
            1_792_890_000,
            "3600\t0\tCET\t1806195600\t7200\tbuiltin\n",
        ),
        (
            c"America/Denver",
            1_772_960_400,
            "-21600\t1\tMDT\t1793520000\t-25200\tloaded\n",
        ),
        (
            c"America/Denver",
            1_793_520_000,
            "-25200\t0\tMST\t1805014800\t-21600\tloaded\n",
        ),
        (c"Asia/Kathmandu", 0, "20700\t0\t\t\t\tbuiltin\n"),
    ];
    for (zone, unix, line) in cases {
        assert_eq!(offset(zone, unix), line, "{zone:?} {unix}");
    }
    let mut written = 0usize;
    assert_eq!(
        unsafe {
            hc_zone_offset(
                c"Mars/Olympus".as_ptr(),
                0,
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe { hc_zone_offset(core::ptr::null(), 0, core::ptr::null_mut(), 0, &mut written) },
        HC_ERROR_NULL_POINTER
    );
}

/// WWVB's bits 57 and 58 from Denver's loaded rules on its day of
/// change, 8 March 2026, with the phase code's `dst_next` for
/// 1 November, `011011`, 27, and DCF77's A1 from Berlin's.
#[cfg(feature = "time-codes")]
#[test]
fn a_radio_frame_reads_its_summer_time_from_the_zone() {
    load_denver();
    let frame = |code: &core::ffi::CStr, unix: i64, summer: &core::ffi::CStr, change, next| {
        super::read_lines(|buffer, capacity, written| unsafe {
            hc_radio_encode(
                code.as_ptr(),
                unix,
                0,
                summer.as_ptr(),
                change,
                0,
                next,
                buffer,
                capacity,
                written,
            )
        })
    };
    let encode = |code: &core::ffi::CStr, unix: i64, summer: &core::ffi::CStr, change| {
        frame(code, unix, summer, change, 0)
    };
    assert_eq!(
        frame(c"wwvb-pm", 1_772_928_000, c"zone:America/Denver", 0, 0),
        frame(c"wwvb-pm", 1_772_928_000, c"begins-today", 0, 27)
    );
    assert_eq!(
        encode(c"wwvb-am", 1_772_928_000, c"zone:America/Denver", 0),
        encode(c"wwvb-am", 1_772_928_000, c"begins-today", 0)
    );
    // New York keeps the same rule and is built in, the README's
    // example of a zone that needs no file.
    assert_eq!(
        encode(c"wwvb-am", 1_772_928_000, c"zone:America/New_York", 0),
        encode(c"wwvb-am", 1_772_928_000, c"begins-today", 0)
    );
    assert_eq!(
        encode(c"dcf77", 1_774_746_000, c"ZONE:Europe/Berlin", 0),
        encode(c"dcf77", 1_774_746_000, c"cest", 1)
    );
}

#[test]
fn days_follow_the_zones_wall_clock() {
    // 08:00 on 25 September 2026 in Tokyo is 23:00 UTC on the 24th.
    let mut september_24 = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2026, 9, 24, &mut september_24) },
        HC_OK
    );
    let instant = (september_24 - 719_163) * 86_400 + 23 * 3_600;
    let mut day = 0i64;
    assert_eq!(
        unsafe { hc_fixed_from_unix_in_zone(instant, c"Asia/Tokyo".as_ptr(), &mut day) },
        HC_OK
    );
    assert_eq!(day, september_24 + 1);
    let mut start = 0i64;
    assert_eq!(
        unsafe { hc_unix_from_fixed_in_zone(september_24 + 1, c"Asia/Tokyo".as_ptr(), &mut start) },
        HC_OK
    );
    // The Tokyo day began at 15:00 UTC on the 24th.
    assert_eq!(start, instant - 8 * 3_600);
    // Cairo's clocks go forward at midnight: 24 April 2026 begins
    // at 01:00 EEST, 22:00 UTC on the 23rd.
    let april_24 = september_24 - 153;
    assert_eq!(
        unsafe { hc_unix_from_fixed_in_zone(april_24, c"Africa/Cairo".as_ptr(), &mut start) },
        HC_OK
    );
    assert_eq!(start, (april_24 - 719_163) * 86_400 - 2 * 3_600);
    assert_eq!(
        unsafe { hc_fixed_from_unix_in_zone(0, c"Mars/Olympus".as_ptr(), &mut day) },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe { hc_fixed_from_unix_in_zone(0, core::ptr::null(), &mut day) },
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        unsafe { hc_fixed_from_unix_in_zone(0, c"UTC".as_ptr(), core::ptr::null_mut()) },
        HC_ERROR_NULL_POINTER
    );
}

/// `hc_unix_from_fixed_in_zone` as a `Result`.
fn starts(fixed: i64, zone: &core::ffi::CStr) -> Result<i64, HcStatus> {
    let mut start = 0i64;
    match unsafe { hc_unix_from_fixed_in_zone(fixed, zone.as_ptr(), &mut start) } {
        HC_OK => Ok(start),
        status => Err(status),
    }
}

/// `hc_fixed_from_unix_in_zone` as a `Result`.
fn day(unix: i64, zone: &core::ffi::CStr) -> Result<i64, HcStatus> {
    let mut day = 0i64;
    match unsafe { hc_fixed_from_unix_in_zone(unix, zone.as_ptr(), &mut day) } {
        HC_OK => Ok(day),
        status => Err(status),
    }
}

/// The day, its start and the offset answer from the rules at both
/// ends of their years, −9 999 994 to 9 999 994, Sydney's summer
/// time included, and refuse a second or a day beyond, where the
/// rules would give standard time.
#[test]
fn a_zone_answers_for_the_years_its_rules_do_and_refuses_beyond() {
    use hc::hc_calendar::gregorian::new_year;
    use hc::hc_tz::posix::{FIRST_RULE_SECOND, FIRST_RULE_YEAR, LAST_RULE_SECOND, LAST_RULE_YEAR};
    let (first, last) = (
        new_year(FIRST_RULE_YEAR).0,
        new_year(LAST_RULE_YEAR + 1).0 - 1,
    );
    assert_eq!((first, last), (-3_652_423_173, 3_652_422_808));
    let (first_second, last_second) = (FIRST_RULE_SECOND, LAST_RULE_SECOND);
    assert_eq!(first_second, -315_631_497_830_400);
    assert_eq!(last_second, 315_507_195_014_399);
    for zone in [
        c"UTC",
        c"Asia/Tokyo",
        c"Australia/Sydney",
        c"America/New_York",
    ] {
        for fixed in [first - 1, last + 1, i64::MIN, i64::MAX] {
            assert_eq!(starts(fixed, zone), Err(HC_ERROR_OUT_OF_RANGE), "{zone:?}");
        }
        for unix in [first_second - 1, last_second + 1, i64::MIN, i64::MAX] {
            assert_eq!(day(unix, zone), Err(HC_ERROR_OUT_OF_RANGE), "{zone:?}");
            let mut written = 0usize;
            assert_eq!(
                unsafe {
                    hc_zone_offset(zone.as_ptr(), unix, core::ptr::null_mut(), 0, &mut written)
                },
                HC_ERROR_OUT_OF_RANGE,
                "{zone:?} {unix}"
            );
        }
    }
    assert_eq!(starts(first, c"UTC"), Ok(first_second));
    assert_eq!(starts(last, c"UTC"), Ok(last_second - 86_399));
    assert_eq!(day(first_second, c"UTC"), Ok(first));
    assert_eq!(day(last_second, c"UTC"), Ok(last));
    // Sydney is on AEDT, eleven hours east, at both ends.
    assert_eq!(
        starts(first, c"Australia/Sydney"),
        Ok(first_second - 11 * 3_600)
    );
    assert_eq!(day(last_second, c"Australia/Sydney"), Ok(last + 1));
    assert!(offset(c"Australia/Sydney", first_second).starts_with("39600\t1\tAEDT\t"));
    assert!(offset(c"Australia/Sydney", last_second).starts_with("39600\t1\tAEDT\t"));
    assert_eq!(day(first_second, c"America/New_York"), Ok(first - 1));
}

#[test]
fn a_loaded_zone_answers_by_name() {
    let name = c"Test/Eastern";
    let mut day = 0i64;
    assert_eq!(
        unsafe { hc_fixed_from_unix_in_zone(0, name.as_ptr(), &mut day) },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_zone_load(
                name.as_ptr(),
                TZIF_V2_EASTERN.as_ptr(),
                TZIF_V2_EASTERN.len(),
            )
        },
        HC_OK
    );
    // 2025-03-09 07:00 UTC is 03:00 EDT, just after the gap.
    let mut march_9 = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2025, 3, 9, &mut march_9) },
        HC_OK
    );
    let midnight_utc = (march_9 - 719_163) * 86_400;
    assert_eq!(
        unsafe { hc_fixed_from_unix_in_zone(midnight_utc + 7 * 3_600, name.as_ptr(), &mut day) },
        HC_OK
    );
    assert_eq!(day, march_9);
    let junk = b"not a zone";
    assert_eq!(
        unsafe { hc_zone_load(c"Test/Junk".as_ptr(), junk.as_ptr(), junk.len()) },
        HC_ERROR_MALFORMED
    );
    assert_eq!(
        unsafe { hc_zone_load(c"".as_ptr(), junk.as_ptr(), junk.len()) },
        HC_ERROR_NULL_POINTER
    );
}
