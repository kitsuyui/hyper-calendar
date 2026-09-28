use super::super::*;
use super::read_lines;

/// `TTBIPM.2025`'s 27.6740 µs on MJD 58 479, 2018-12-27, when TAI −
/// UTC was 37 s, and 27.6745 µs ten days later.
#[test]
fn tt_bipm_reads_the_callers_series() {
    let series = "58479\t27.6740\n58489\t27.6745\n";
    let tai = (58_479 - 40_587) * 86_400 + 37;
    let cells = line(|buffer, capacity| unsafe {
        hc_tt_bipm(series.as_ptr(), series.len(), tai, 0, 1, buffer, capacity)
    });
    assert_eq!(cells.len(), 5);
    let offset: f64 = cells[0].parse().expect("seconds");
    assert!((offset - 27.674e-6).abs() < 1e-15, "{offset}");
    assert_eq!(cells[1], "32");
    assert_eq!(cells[3], (tai + 32).to_string());
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_tt_bipm(series.as_ptr(), series.len(), tai - 1, 0, 1, null, 0) },
        HC_ERR_NO_DATA
    );
    assert_eq!(
        unsafe { hc_tt_bipm("58479 27.6".as_ptr(), 10, tai, 0, 1, null, 0) },
        HC_ERR_MALFORMED
    );
    assert_eq!(
        unsafe {
            hc_tt_bipm(
                series.as_ptr(),
                series.len(),
                tai,
                1_000_000_000_000_000_000,
                1,
                null,
                0,
            )
        },
        HC_ERR_OUT_OF_RANGE
    );
}

fn line(call: impl Fn(*mut u8, usize) -> i64) -> Vec<String> {
    let text = read_lines(call);
    let line = text.strip_suffix('\n').expect("one line");
    assert!(!line.contains('\n'), "{text:?}");
    line.split('\t').map(str::to_owned).collect()
}

#[test]
fn a_tai64_label_crosses_both_ways_and_refuses_by_name() {
    let label = line(|buffer, capacity| unsafe {
        hc_tai64_encode(0, 0, "tai64n".as_ptr(), 6, buffer, capacity)
    });
    assert_eq!(label, ["400000000000000000000000"]);
    let hex = "3FFFFFFFFFFFFFFF3B9AC9FF3B9AC9FF";
    let decoded = line(|buffer, capacity| unsafe {
        hc_tai64_decode(hex.as_ptr(), hex.len(), buffer, capacity)
    });
    assert_eq!(decoded, ["tai64na", "-1", "999999999999999999"]);
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_tai64_encode(0, 0, "tai32".as_ptr(), 5, null, 0) },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe { hc_tai64_encode(0, 1_000_000_000_000_000_000, "tai64".as_ptr(), 5, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_tai64_decode("40".as_ptr(), 2, null, 0) },
        HC_ERR_MALFORMED
    );
    assert_eq!(
        unsafe { hc_tai64_decode(core::ptr::null(), 1, null, 0) },
        HC_ERR_NULL_POINTER
    );
    assert_eq!(
        unsafe { hc_tai64_decode([0xffu8; 16].as_ptr(), 16, null, 0) },
        HC_ERR_NOT_UTF8
    );
}

/// The leap second at the end of 2016 crosses as a line: 23:59:59
/// UTC was TAI + 36 s, so TAI second 1 483 228 836 is 23:59:60,
/// named by the POSIX second after it, and 00:00:00 is TAI + 37 s.
#[test]
fn the_tai_bridge_writes_one_line_each_way() {
    let read = |call: &dyn Fn(*mut u8, usize) -> i64| {
        let measured = call(core::ptr::null_mut(), 0);
        let mut buffer = [0u8; 64];
        let len = call(buffer.as_mut_ptr(), buffer.len());
        assert_eq!(len, measured);
        String::from_utf8(buffer[..len as usize].to_vec()).expect("UTF-8")
    };
    let new_year = 1_483_228_800;
    assert_eq!(
        read(&|buffer, capacity| unsafe { hc_tai_from_unix(new_year, 1, buffer, capacity) }),
        "1483228837\t0\n"
    );
    assert_eq!(
        read(&|buffer, capacity| unsafe { hc_utc_from_tai(new_year + 36, 1, buffer, capacity) }),
        "1483228800\t1\n"
    );
    assert_eq!(
        read(&|buffer, capacity| unsafe { hc_utc_from_tai(new_year + 37, 0, buffer, capacity) }),
        "1483228800\t0\n"
    );
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_tai_from_unix(-400_000_000, 1, null, 0) },
        HC_ERR_NO_DATA
    );
    assert_eq!(
        unsafe { hc_tai_from_unix(i64::MAX, 0, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
    let mut small = [0u8; 4];
    assert_eq!(
        unsafe { hc_tai_from_unix(new_year, 1, small.as_mut_ptr(), small.len()) },
        HC_ERR_BUFFER_TOO_SMALL
    );
}

/// POSIX 0 is `@400000000000000a` on daemontools' ordinary clock,
/// which the true-TAI decoder reads as ten seconds after 1970 TAI.
#[test]
fn the_posix_plus_10_labels_cross_both_ways() {
    let label = line(|buffer, capacity| unsafe {
        hc_tai64_posix_plus_10_encode(0, 0, "TAI64".as_ptr(), 5, buffer, capacity)
    });
    assert_eq!(label, ["400000000000000a"]);
    let hex = "400000000000000b1dcd6500";
    let decoded = line(|buffer, capacity| unsafe {
        hc_tai64_posix_plus_10_decode(hex.as_ptr(), hex.len(), buffer, capacity)
    });
    assert_eq!(decoded, ["tai64n", "1", "500000000000000000"]);
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_tai64_posix_plus_10_encode(0, 0, "tai64na".as_ptr(), 7, null, 0) },
        HC_ERR_UNKNOWN
    );
    let na = "3fffffffffffffff3b9ac9ff3b9ac9ff";
    assert_eq!(
        unsafe { hc_tai64_posix_plus_10_decode(na.as_ptr(), na.len(), null, 0) },
        HC_ERR_MALFORMED
    );
}

/// RFC 9562's version 1 and version 6 vectors, Appendix A: both
/// 0x1EC9414C232AB00, POSIX 1 645 557 742.
#[test]
fn a_uuid_timestamp_crosses_the_boundary() {
    for (uuid, version) in [
        ("C232AB00-9414-11EC-B3C8-9F6BDECED846", "1"),
        ("1EC9414C-232A-6B00-B3C8-9F6BDECED846", "6"),
    ] {
        let cells = line(|buffer, capacity| unsafe {
            hc_uuid_timestamp(uuid.as_ptr(), uuid.len(), buffer, capacity)
        });
        assert_eq!(cells, [version, "138648505420000000", "1645557742", "0"]);
    }
    let v4 = "919108f7-52d1-4320-9bac-f847db4148a8";
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_uuid_timestamp(v4.as_ptr(), v4.len(), null, 0) },
        HC_ERR_NO_DATA
    );
    assert_eq!(
        unsafe { hc_uuid_timestamp("C232AB00".as_ptr(), 8, null, 0) },
        HC_ERR_MALFORMED
    );
}

/// RFC 5905's Figure 4: 8 February 2036 is era 1, offset 63 104,
/// read against a clock in 2030; the zero timestamp is unknown.
#[test]
fn an_ntp_timestamp_crosses_with_its_era() {
    let cells = line(|buffer, capacity| unsafe {
        hc_ntp_resolve(63_104, 0, 1_893_456_000, buffer, capacity)
    });
    assert_eq!(cells, ["1", "63104", "0", "2086041600", "0"]);
    assert_eq!(
        unsafe { hc_ntp_resolve(0, 0, 0, core::ptr::null_mut(), 0) },
        HC_ERR_NO_DATA
    );
}

/// RFC 9562's vectors the other way: POSIX 1 645 557 742 is the
/// timestamp 0x1EC9414C232AB00, `C232AB00-9414-11EC` in version 1
/// and `1EC9414C-232A-6B00` in version 6.
#[test]
fn a_posix_instant_crosses_as_a_uuid_timestamp() {
    let cells = line(|buffer, capacity| unsafe {
        hc_uuid_timestamp_encode(1_645_557_742, 0, buffer, capacity)
    });
    assert_eq!(
        cells,
        [
            "138648505420000000",
            "c232ab00-9414-11ec",
            "1ec9414c-232a-6b00"
        ]
    );
    let null = core::ptr::null_mut();
    for (seconds, attoseconds) in [
        (-12_219_292_801, 0),
        (103_072_857_661, 0),
        (0, 1_000_000_000_000_000_000),
    ] {
        assert_eq!(
            unsafe { hc_uuid_timestamp_encode(seconds, attoseconds, null, 0) },
            HC_ERR_OUT_OF_RANGE,
            "{seconds}"
        );
    }
}

/// RFC 5905's Figure 4: 1 January 1970 is era 0, offset
/// 2 208 988 800, and 8 February 2036 is era 1, offset 63 104.
#[test]
fn a_posix_instant_crosses_as_an_ntp_date() {
    let cells = line(|buffer, capacity| unsafe { hc_ntp_encode(0, 0, buffer, capacity) });
    assert_eq!(
        cells,
        [
            "0",
            "2208988800",
            "0",
            "0000000083aa7e800000000000000000",
            "83aa7e8000000000"
        ]
    );
    let cells =
        line(|buffer, capacity| unsafe { hc_ntp_encode(2_086_041_600, 0, buffer, capacity) });
    assert_eq!(cells[..2], ["1", "63104"]);
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_ntp_encode(i64::MAX, 0, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_ntp_encode(0, 1_000_000_000_000_000_000, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

/// The worked example of `docs/systems/binary-timestamps.md`:
/// 2026-09-26 23:59:58 is the words 23 866 and 49 021.
#[test]
fn the_fat_words_cross_both_ways() {
    let day = hc_gregorian_to_fixed(2026, 9, 26);
    let cells = line(|buffer, capacity| unsafe { hc_fat_decode(23_866, 49_021, buffer, capacity) });
    assert_eq!(cells, [day.to_string(), "86398".to_owned()]);
    let cells = line(|buffer, capacity| unsafe { hc_fat_encode(day, 86_399, buffer, capacity) });
    assert_eq!(cells, ["23866", "49021"]);
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_fat_decode(65_536, 0, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_fat_decode(46 << 9 | 1, 0, null, 0) },
        HC_ERR_INVALID_DATE
    );
    assert_eq!(
        unsafe { hc_fat_encode(hc_gregorian_to_fixed(1979, 12, 31), 0, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

/// Wikipedia's example, @248 at 04:57:07.2 UTC; the POSIX epoch is
/// @041.
#[test]
fn a_swatch_beat_is_a_value() {
    let at_248 = (4 * 60 + 57) * 60 + 7;
    assert_eq!(hc_swatch_beat(at_248, 200_000_000_000_000_000), 248);
    assert_eq!(hc_swatch_beat(0, 0), 41);
    assert_eq!(
        hc_swatch_beat(0, 1_000_000_000_000_000_000),
        HC_ERR_OUT_OF_RANGE
    );
}

/// SOFA's §2.4 example, JD 2457073.05631 TT: J2015.1349933196 and
/// B2015.1365941021; and J2000.0 is 946 728 000 TT seconds.
#[test]
fn the_epochs_cross_both_ways() {
    let epoch = |notation: &str, seconds: i64, attos: u64| {
        line(|buffer, capacity| unsafe {
            hc_epoch_from_tt(
                notation.as_ptr(),
                notation.len(),
                seconds,
                attos,
                buffer,
                capacity,
            )
        })
    };
    let julian = epoch("J", 1_424_352_065, 184_000_000_000_000_000);
    assert_eq!(julian[0], "J");
    let year: f64 = julian[1].parse().expect("a number");
    assert!((year - 2_015.134_993_319_6).abs() < 1e-10, "{year}");
    let besselian = epoch("B", 1_424_352_065, 184_000_000_000_000_000);
    let year: f64 = besselian[1].parse().expect("a number");
    assert!((year - 2_015.136_594_102_1).abs() < 1e-10, "{year}");
    assert_eq!(epoch("julian-epoch", 946_728_000, 0), ["J", "2000"]);
    let back = line(|buffer, capacity| unsafe {
        hc_tt_from_epoch("".as_ptr(), 0, 2000.0, buffer, capacity)
    });
    assert_eq!(back, ["J", "946728000", "0"]);
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_epoch_from_tt("X".as_ptr(), 1, 0, 0, null, 0) },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe { hc_tt_from_epoch("J".as_ptr(), 1, f64::NAN, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

/// GPS week 2048 began at 2019-04-06 23:59:42 UTC, TAI second
/// 1 554 595 219.
#[test]
fn the_april_2019_rollover_crosses_the_boundary() {
    let tai = 1_554_595_219;
    let id = "gps-lnav-week";
    let week = line(|buffer, capacity| unsafe {
        hc_gnss_week(id.as_ptr(), id.len(), tai, 0, buffer, capacity)
    });
    assert_eq!(week, ["2048", "0", "0", "0"]);
    let back = line(|buffer, capacity| unsafe {
        hc_gnss_to_tai(id.as_ptr(), id.len(), 2048, 0, 0, buffer, capacity)
    });
    assert_eq!(back, [tai.to_string(), "0".to_owned()]);
    let resolve = |broadcast: u32, rule: &str| unsafe {
        hc_gnss_resolve_week(
            id.as_ptr(),
            id.len(),
            broadcast,
            rule.as_ptr(),
            rule.len(),
            tai,
        )
    };
    assert_eq!(resolve(0, "not-before"), 2048);
    assert_eq!(resolve(1023, "nearest"), 2047);
    assert_eq!(resolve(1024, "nearest"), HC_ERR_OUT_OF_RANGE);
    assert_eq!(resolve(0, "latest"), HC_ERR_UNKNOWN);
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_gnss_week(id.as_ptr(), id.len(), 0, 0, null, 0) },
        HC_ERR_NO_DATA
    );
    assert_eq!(
        unsafe { hc_gnss_to_tai(id.as_ptr(), id.len(), 0, 604_800, 0, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

#[test]
fn glonass_ole_and_excel_dates_cross_the_boundary() {
    // 2024-01-01 00:00 UTC: N4 = 8, N_T = 1.
    let tai = 1_704_067_200 + 37;
    let date = line(|buffer, capacity| unsafe { hc_glonass_date(tai, 0, 1, buffer, capacity) });
    assert_eq!(date, ["8", "1"]);
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_glonass_date(0, 0, 0, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
    let new_year_1900 = hc_gregorian_to_fixed(1900, 1, 1);
    let ole =
        line(|buffer, capacity| unsafe { hc_fixed_from_ole_automation(-1.25, buffer, capacity) });
    assert_eq!(ole, [(new_year_1900 - 3).to_string(), "21600".to_owned()]);
    let value = line(|buffer, capacity| unsafe {
        hc_ole_automation_from_fixed(new_year_1900 - 3, 21_600.0, buffer, capacity)
    });
    assert_eq!(value, ["-1.25"]);
    assert_eq!(
        unsafe { hc_fixed_from_ole_automation(f64::INFINITY, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
    let phantom = line(|buffer, capacity| unsafe { hc_excel_1900_day(60, buffer, capacity) });
    assert_eq!(phantom, ["", "1"]);
    let march = line(|buffer, capacity| unsafe { hc_excel_1900_day(61, buffer, capacity) });
    assert_eq!(
        march,
        [
            hc_gregorian_to_fixed(1900, 3, 1).to_string(),
            "0".to_owned()
        ]
    );
    assert_eq!(
        unsafe { hc_excel_1900_day(0, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
}
