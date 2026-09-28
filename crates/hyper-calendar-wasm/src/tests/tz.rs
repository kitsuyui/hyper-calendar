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

fn load(name: &str, bytes: &[u8]) {
    assert_eq!(
        unsafe { hc_zone_load(name.as_ptr(), name.len(), bytes.as_ptr(), bytes.len()) },
        0
    );
}

fn offset(zone: &str, unix: i64) -> String {
    super::read_lines(|buffer, capacity| unsafe {
        hc_zone_offset(zone.as_ptr(), zone.len(), unix, buffer, capacity)
    })
}

fn in_zone(unix: i64, zone: &str) -> i64 {
    unsafe { hc_fixed_from_unix_in_zone(unix, zone.as_ptr(), zone.len()) }
}

fn starts(fixed: i64, zone: &str) -> i64 {
    unsafe { hc_unix_from_fixed_in_zone(fixed, zone.as_ptr(), zone.len()) }
}

/// The first and the last day of the years the rules answer for,
/// −9 999 994 to 9 999 994, and the first and last second.
fn rule_range() -> (i64, i64, i64, i64) {
    use hc::hc_calendar::gregorian::new_year;
    use hc::hc_tz::posix::{FIRST_RULE_SECOND, FIRST_RULE_YEAR, LAST_RULE_SECOND, LAST_RULE_YEAR};
    (
        new_year(FIRST_RULE_YEAR).0,
        new_year(LAST_RULE_YEAR + 1).0 - 1,
        FIRST_RULE_SECOND,
        LAST_RULE_SECOND,
    )
}

/// The day, its start and the offset answer from the rules at both
/// ends of their years, Sydney's summer time included, and refuse
/// a second or a day beyond, where the rules would give standard
/// time.
#[test]
fn a_zone_answers_for_the_years_its_rules_do_and_refuses_beyond() {
    let (first, last, first_second, last_second) = rule_range();
    assert_eq!(first, -3_652_423_173);
    assert_eq!(last, 3_652_422_808);
    assert_eq!(first_second, (first - 719_163) * 86_400);
    assert_eq!(last_second, (last - 719_163) * 86_400 + 86_399);
    for zone in ["UTC", "Asia/Tokyo", "Australia/Sydney", "America/New_York"] {
        assert_eq!(starts(first - 1, zone), HC_ERR_OUT_OF_RANGE, "{zone}");
        assert_eq!(starts(last + 1, zone), HC_ERR_OUT_OF_RANGE, "{zone}");
        assert_eq!(
            in_zone(first_second - 1, zone),
            HC_ERR_OUT_OF_RANGE,
            "{zone}"
        );
        assert_eq!(
            in_zone(last_second + 1, zone),
            HC_ERR_OUT_OF_RANGE,
            "{zone}"
        );
        for unix in [first_second - 1, last_second + 1, i64::MIN, i64::MAX] {
            let measured = unsafe {
                hc_zone_offset(zone.as_ptr(), zone.len(), unix, core::ptr::null_mut(), 0)
            };
            assert_eq!(measured, HC_ERR_OUT_OF_RANGE, "{zone} {unix}");
        }
        for fixed in [i64::MIN, i64::MAX] {
            assert_eq!(starts(fixed, zone), HC_ERR_OUT_OF_RANGE, "{zone}");
        }
        for unix in [i64::MIN, i64::MAX] {
            assert_eq!(in_zone(unix, zone), HC_ERR_OUT_OF_RANGE, "{zone}");
        }
    }
    assert_eq!(starts(first, "UTC"), first_second);
    assert_eq!(starts(last, "UTC"), last_second - 86_399);
    assert_eq!(in_zone(first_second, "UTC"), first);
    assert_eq!(in_zone(last_second, "UTC"), last);
    // Sydney is on AEDT, eleven hours east, at both ends: its first
    // day begins eleven hours before UTC's, and the last second of
    // the last UTC day is 10:59:59 on its next.
    assert_eq!(starts(first, "Australia/Sydney"), first_second - 11 * 3_600);
    assert_eq!(
        starts(last, "Australia/Sydney"),
        last_second - 86_399 - 11 * 3_600
    );
    assert_eq!(in_zone(last_second, "Australia/Sydney"), last + 1);
    assert!(offset("Australia/Sydney", first_second).starts_with("39600\t1\tAEDT\t"));
    assert!(offset("Australia/Sydney", last_second).starts_with("39600\t1\tAEDT\t"));
    // New York on EST, five hours west: the first second is 19:00
    // on the day before.
    assert_eq!(in_zone(first_second, "America/New_York"), first - 1);
    assert!(offset("America/New_York", last_second).starts_with("-18000\t0\tEST\t"));
}

#[test]
fn the_readers_day_is_the_zones_day_not_utcs() {
    // 08:00 on 25 September 2026 in Tokyo is 23:00 UTC on the 24th.
    let september_24 = hc_gregorian_to_fixed(2026, 9, 24);
    let instant = hc_unix_from_fixed(september_24) + 23 * 3_600;
    assert_eq!(hc_fixed_from_unix(instant), september_24);
    assert_eq!(in_zone(instant, "Asia/Tokyo"), september_24 + 1);
    assert_eq!(in_zone(instant, "asia/tokyo"), september_24 + 1);
    assert_eq!(in_zone(instant, "UTC"), september_24);
    // And the Tokyo day begins nine hours before the UTC one.
    assert_eq!(
        starts(september_24 + 1, "Asia/Tokyo"),
        hc_unix_from_fixed(september_24 + 1) - 9 * 3_600
    );
}

#[test]
fn a_day_that_begins_in_a_gap_begins_after_it() {
    // New York's clocks go forward at 02:00 on 8 March 2026: the
    // day begins at its ordinary midnight, 05:00 UTC, and 03:00 EDT
    // is 07:00 UTC, still the 8th.
    let march_8 = hc_gregorian_to_fixed(2026, 3, 8);
    let midnight_utc = hc_unix_from_fixed(march_8);
    assert_eq!(
        starts(march_8, "America/New_York"),
        midnight_utc + 5 * 3_600
    );
    assert_eq!(
        in_zone(midnight_utc + 7 * 3_600, "America/New_York"),
        march_8
    );
    assert_eq!(
        in_zone(midnight_utc + 4 * 3_600, "America/New_York"),
        march_8 - 1
    );
    // Cairo's clocks go forward at 00:00 on the last Friday of
    // April, so 24 April 2026 has no midnight: it begins at 01:00
    // EEST, which is 22:00 UTC on the 23rd.
    let april_24 = hc_gregorian_to_fixed(2026, 4, 24);
    assert_eq!(
        starts(april_24, "Africa/Cairo"),
        hc_unix_from_fixed(april_24) - 2 * 3_600
    );
    assert_eq!(
        starts(april_24 + 1, "Africa/Cairo"),
        hc_unix_from_fixed(april_24 + 1) - 3 * 3_600
    );
}

/// The 2026 changes of Europe/Berlin, 29 March and 25 October at
/// 01:00 UTC, from the built-in rules, and of America/Denver,
/// 8 March and 1 November, from a loaded file — its record, then
/// its footer — each a second before and at the change.
#[test]
fn the_offset_at_each_change_of_2026() {
    load("America/Denver", TZIF_V2_DENVER);
    let cases = [
        (
            "Europe/Berlin",
            1_774_745_999,
            "3600\t0\tCET\t1774746000\t7200\tbuiltin\n",
        ),
        (
            "Europe/Berlin",
            1_774_746_000,
            "7200\t1\tCEST\t1792890000\t3600\tbuiltin\n",
        ),
        (
            "Europe/Berlin",
            1_792_889_999,
            "7200\t1\tCEST\t1792890000\t3600\tbuiltin\n",
        ),
        (
            "europe/berlin",
            1_792_890_000,
            "3600\t0\tCET\t1806195600\t7200\tbuiltin\n",
        ),
        (
            "America/Denver",
            1_767_225_600,
            "-25200\t0\tMST\t1772960400\t-21600\tloaded\n",
        ),
        (
            "America/Denver",
            1_772_960_399,
            "-25200\t0\tMST\t1772960400\t-21600\tloaded\n",
        ),
        (
            "America/Denver",
            1_772_960_400,
            "-21600\t1\tMDT\t1793520000\t-25200\tloaded\n",
        ),
        (
            "America/Denver",
            1_793_519_999,
            "-21600\t1\tMDT\t1793520000\t-25200\tloaded\n",
        ),
        (
            "America/Denver",
            1_793_520_000,
            "-25200\t0\tMST\t1805014800\t-21600\tloaded\n",
        ),
    ];
    for (zone, unix, line) in cases {
        assert_eq!(offset(zone, unix), line, "{zone} {unix}");
    }
}

#[test]
fn zones_without_summer_time_have_no_next_transition() {
    let cases = [
        ("UTC", "0\t0\tUTC\t\t\tbuiltin\n"),
        ("Asia/Tokyo", "32400\t0\tJST\t\t\tbuiltin\n"),
        ("Asia/Kolkata", "19800\t0\tIST\t\t\tbuiltin\n"),
        ("Asia/Kathmandu", "20700\t0\t\t\t\tbuiltin\n"),
        ("America/Sao_Paulo", "-10800\t0\t\t\t\tbuiltin\n"),
        ("Europe/Moscow", "10800\t0\tMSK\t\t\tbuiltin\n"),
    ];
    for (zone, line) in cases {
        assert_eq!(offset(zone, 1_774_746_000), line, "{zone}");
    }
}

/// The day and the offset come from the same rules: every hour of
/// 2026, the zone's day is the UTC day of the instant moved by the
/// offset.
#[test]
fn the_day_in_a_zone_is_the_instant_moved_by_its_offset() {
    load("America/Denver", TZIF_V2_DENVER);
    for zone in [
        "Europe/Berlin",
        "America/Denver",
        "Australia/Lord_Howe",
        "Asia/Kathmandu",
    ] {
        for unix in (1_767_225_600..1_798_761_600).step_by(3_600) {
            let line = offset(zone, unix);
            let seconds: i64 = line.split('\t').next().unwrap().parse().unwrap();
            assert_eq!(
                in_zone(unix, zone),
                hc_fixed_from_unix(unix + seconds),
                "{zone} {unix}"
            );
        }
    }
}

#[test]
fn an_unknown_zone_has_no_offset() {
    let call = |zone: &[u8]| unsafe {
        hc_zone_offset(zone.as_ptr(), zone.len(), 0, core::ptr::null_mut(), 0)
    };
    assert_eq!(call(b"Mars/Olympus"), HC_ERR_UNKNOWN);
    assert_eq!(call(&[0xff]), HC_ERR_NOT_UTF8);
    assert_eq!(
        unsafe { hc_zone_offset(core::ptr::null(), 3, 0, core::ptr::null_mut(), 0) },
        HC_ERR_NULL_POINTER
    );
}

/// The radio codes' summer time from the zones' rules: DCF77's
/// frames around Berlin's change of 29 March 2026, WWVB's bits 57
/// and 58 on Denver's day of change, 8 March, and the phase code's
/// `dst_next` either side of 00:00 UTC that day: 8 March, then
/// 1 November, both `011011`, 27.
#[cfg(feature = "time-codes")]
#[test]
fn a_radio_frame_reads_its_summer_time_from_the_zone() {
    load("America/Denver", TZIF_V2_DENVER);
    let frame = |code: &str, unix: i64, summer: &str, change: i32, next: u32| {
        super::read_lines(|buffer, capacity| unsafe {
            hc_radio_encode(
                code.as_ptr(),
                code.len(),
                unix,
                0,
                summer.as_ptr(),
                summer.len(),
                change,
                0,
                next,
                buffer,
                capacity,
            )
        })
    };
    let encode =
        |code: &str, unix: i64, summer: &str, change: i32| frame(code, unix, summer, change, 0);
    let change = 1_774_746_000;
    for (minute, summer, a1) in [
        (change - 3_600, "cet", 0),
        (change - 3_540, "cet", 1),
        (change, "cest", 1),
        (change + 60, "cest", 0),
    ] {
        assert_eq!(
            encode("dcf77", minute, "zone:Europe/Berlin", 0),
            encode("dcf77", minute, summer, a1),
            "{minute}"
        );
    }
    let march_8 = 1_772_928_000;
    for (minute, summer) in [
        (march_8 - 60, "standard"),
        (march_8, "begins-today"),
        (march_8 + 86_400, "in-effect"),
    ] {
        assert_eq!(
            encode("wwvb-am", minute, "zone:America/Denver", 0),
            encode("wwvb-am", minute, summer, 0),
            "{minute}"
        );
        assert_eq!(
            frame("wwvb-pm", minute, "zone:America/Denver", 0, 0),
            frame("wwvb-pm", minute, summer, 0, 27),
            "{minute}"
        );
    }
    let refused = |code: &str, summer: &str| unsafe {
        hc_radio_encode(
            code.as_ptr(),
            code.len(),
            change,
            0,
            summer.as_ptr(),
            summer.len(),
            0,
            0,
            0,
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(refused("dcf77", "zone:Mars/Olympus"), HC_ERR_UNKNOWN);
    assert_eq!(refused("dcf77", "zone:Europe/London"), HC_ERR_OUT_OF_RANGE);
    assert_eq!(refused("jjy", "zone:Asia/Tokyo"), HC_ERR_UNKNOWN);
}

#[test]
fn unknown_zones_and_bad_names_are_sentinels() {
    assert_eq!(in_zone(0, "Mars/Olympus"), HC_ERR_UNKNOWN);
    assert_eq!(starts(0, ""), HC_ERR_UNKNOWN);
    let not_utf8 = [0xffu8];
    assert_eq!(
        unsafe { hc_fixed_from_unix_in_zone(0, not_utf8.as_ptr(), 1) },
        HC_ERR_NOT_UTF8
    );
    assert_eq!(
        unsafe { hc_unix_from_fixed_in_zone(0, core::ptr::null(), 3) },
        HC_ERR_NULL_POINTER
    );
}

#[test]
fn a_loaded_zone_answers_by_name_and_outranks_the_builtin() {
    let name = "Test/Eastern";
    assert_eq!(in_zone(0, name), HC_ERR_UNKNOWN);
    assert_eq!(
        unsafe {
            hc_zone_load(
                name.as_ptr(),
                name.len(),
                TZIF_V2_EASTERN.as_ptr(),
                TZIF_V2_EASTERN.len(),
            )
        },
        0
    );
    // 2025-03-09 07:00 UTC is 03:00 EDT, just after the gap.
    let march_9 = hc_gregorian_to_fixed(2025, 3, 9);
    let midnight_utc = hc_unix_from_fixed(march_9);
    assert_eq!(in_zone(midnight_utc + 7 * 3_600, name), march_9);
    assert_eq!(in_zone(midnight_utc + 4 * 3_600, name), march_9 - 1);
    assert_eq!(starts(march_9, name), midnight_utc + 5 * 3_600);
    // The same bytes under a built-in name take precedence over it.
    let builtin = "America/Los_Angeles";
    assert_eq!(starts(march_9, builtin), midnight_utc + 8 * 3_600);
    assert_eq!(
        unsafe {
            hc_zone_load(
                builtin.as_ptr(),
                builtin.len(),
                TZIF_V2_EASTERN.as_ptr(),
                TZIF_V2_EASTERN.len(),
            )
        },
        0
    );
    assert_eq!(starts(march_9, builtin), midnight_utc + 5 * 3_600);
    // Bytes that are not TZif are refused and nothing is kept.
    let junk = b"not a zone";
    let other = "Test/Junk";
    assert_eq!(
        unsafe { hc_zone_load(other.as_ptr(), other.len(), junk.as_ptr(), junk.len()) },
        HC_ERR_MALFORMED
    );
    assert_eq!(in_zone(0, other), HC_ERR_UNKNOWN);
    assert_eq!(
        unsafe { hc_zone_load(core::ptr::null(), 0, junk.as_ptr(), junk.len()) },
        HC_ERR_UNKNOWN
    );
}
