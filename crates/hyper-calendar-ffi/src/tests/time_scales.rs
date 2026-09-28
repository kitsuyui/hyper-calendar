use super::super::*;
use super::{measured, read_lines};

/// `TTBIPM.2025`'s 27.6740 µs on MJD 58 479, as the module reads it.
#[test]
fn the_tt_bipm_line_is_the_modules() {
    let series = c"58479\t27.6740\n58489\t27.6745\n";
    let tai = (58_479 - 40_587) * 86_400 + 37;
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_tt_bipm(series.as_ptr(), tai, 0, 1, buffer, capacity, written)
    });
    let expected = hc::time_lines::tt_bipm_line(series.to_str().expect("UTF-8"), tai, 0, true);
    assert_eq!(Ok(text), expected);
    let mut written = 0usize;
    assert_eq!(
        unsafe {
            hc_tt_bipm(
                series.as_ptr(),
                tai - 1,
                0,
                1,
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_NO_DATA
    );
}

/// POSIX 0 is `@400000000000000a` on daemontools' ordinary clock.
#[test]
fn the_posix_plus_10_labels_cross_both_ways() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_tai64_posix_plus_10_encode(
            1,
            500_000_000_000_000_000,
            c"tai64n".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(text, "400000000000000b1dcd6500\n");
    let (mut seconds, mut attos) = (7i64, 7u64);
    assert_eq!(
        unsafe {
            hc_tai64_posix_plus_10_decode(c"400000000000000A".as_ptr(), &mut seconds, &mut attos)
        },
        HC_OK
    );
    assert_eq!((seconds, attos), (0, 0));
    assert_eq!(
        unsafe {
            hc_tai64_posix_plus_10_encode(
                0,
                0,
                c"tai64na".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe { hc_tai64_posix_plus_10_decode(core::ptr::null(), &mut seconds, &mut attos) },
        HC_ERROR_NULL_POINTER
    );
}

/// RFC 9562's version 6 vector, Appendix A, and its version 4 one.
#[test]
fn a_uuid_timestamp_crosses_the_boundary() {
    let (mut version, mut timestamp, mut unix, mut attos) = (0, 0u64, 0i64, 7u64);
    assert_eq!(
        unsafe {
            hc_uuid_timestamp(
                c"1EC9414C-232A-6B00-B3C8-9F6BDECED846".as_ptr(),
                &mut version,
                &mut timestamp,
                &mut unix,
                &mut attos,
            )
        },
        HC_OK
    );
    assert_eq!(
        (version, timestamp, unix, attos),
        (6, 0x1EC_9414_C232_AB00, 1_645_557_742, 0)
    );
    assert_eq!(
        unsafe {
            hc_uuid_timestamp(
                c"919108f7-52d1-4320-9bac-f847db4148a8".as_ptr(),
                &mut version,
                &mut timestamp,
                &mut unix,
                &mut attos,
            )
        },
        HC_ERROR_NO_DATA
    );
    assert_eq!(
        unsafe {
            hc_uuid_timestamp(
                c"not a uuid".as_ptr(),
                &mut version,
                &mut timestamp,
                &mut unix,
                &mut attos,
            )
        },
        HC_ERROR_MALFORMED
    );
}

/// RFC 5905's Figure 4: 8 February 2036 is era 1, offset 63 104.
#[test]
fn an_ntp_timestamp_crosses_with_its_era() {
    let (mut era, mut offset, mut fraction, mut unix, mut attos) = (7, 7u32, 7u64, 7i64, 7u64);
    assert_eq!(
        unsafe {
            hc_ntp_resolve(
                63_104,
                0,
                1_893_456_000,
                &mut era,
                &mut offset,
                &mut fraction,
                &mut unix,
                &mut attos,
            )
        },
        HC_OK
    );
    assert_eq!(
        (era, offset, fraction, unix, attos),
        (1, 63_104, 0, 2_086_041_600, 0)
    );
    assert_eq!(
        unsafe {
            hc_ntp_resolve(
                0,
                0,
                0,
                &mut era,
                &mut offset,
                &mut fraction,
                &mut unix,
                &mut attos,
            )
        },
        HC_ERROR_NO_DATA
    );
}

/// RFC 9562's example instant, POSIX 1 645 557 742, is the timestamp
/// of its vectors, and RFC 5905's Figure 4 puts 1 January 1970 at
/// era 0, offset 2 208 988 800.
#[test]
fn a_posix_instant_crosses_as_a_uuid_timestamp_and_an_ntp_date() {
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_uuid_timestamp_encode(1_645_557_742, 0, buffer, capacity, written)
    });
    assert_eq!(
        line,
        "138648505420000000\tc232ab00-9414-11ec\t1ec9414c-232a-6b00\n"
    );
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_ntp_encode(0, 0, buffer, capacity, written)
    });
    assert_eq!(
        line,
        "0\t2208988800\t0\t0000000083aa7e800000000000000000\t83aa7e8000000000\n"
    );
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_uuid_timestamp_encode(-12_219_292_801, 0, null, 0, null.cast()) },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_ntp_encode(i64::MAX, 0, null, 0, null.cast()) },
        HC_ERROR_OVERFLOW
    );
    assert_eq!(
        unsafe { hc_ntp_encode(0, 1_000_000_000_000_000_000, null, 0, null.cast()) },
        HC_ERROR_OUT_OF_RANGE
    );
}

/// 2026-09-26 23:59:58 is the FAT words 23 866 and 49 021.
#[test]
fn the_fat_words_cross_both_ways() {
    let mut day = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2026, 9, 26, &mut day) },
        HC_OK
    );
    let (mut fixed, mut seconds) = (0i64, 0u32);
    assert_eq!(
        unsafe { hc_fat_decode(23_866, 49_021, &mut fixed, &mut seconds) },
        HC_OK
    );
    assert_eq!((fixed, seconds), (day, 86_398));
    let (mut date, mut time) = (0u16, 0u16);
    assert_eq!(
        unsafe { hc_fat_encode(day, 86_399, &mut date, &mut time) },
        HC_OK
    );
    assert_eq!((date, time), (23_866, 49_021));
    assert_eq!(
        unsafe { hc_fat_decode(46 << 9 | 2 << 5 | 30, 0, &mut fixed, &mut seconds) },
        HC_ERROR_INVALID_DATE
    );
    assert_eq!(
        unsafe { hc_fat_encode(day, 86_400, &mut date, &mut time) },
        HC_ERROR_OUT_OF_RANGE
    );
}

/// @248 is 04:57:07.2 UTC; SOFA's JD 2457073.05631 TT is
/// J2015.1349933196, and J2000.0 946 728 000 TT seconds.
#[test]
fn a_beat_and_the_epochs_cross_the_boundary() {
    let mut beat = 0u16;
    assert_eq!(
        unsafe { hc_swatch_beat((4 * 60 + 57) * 60 + 7, 200_000_000_000_000_000, &mut beat) },
        HC_OK
    );
    assert_eq!(beat, 248);
    let mut year = 0.0f64;
    assert_eq!(
        unsafe {
            hc_epoch_from_tt(
                c"J".as_ptr(),
                1_424_352_065,
                184_000_000_000_000_000,
                &mut year,
            )
        },
        HC_OK
    );
    assert!((year - 2_015.134_993_319_6).abs() < 1e-10, "{year}");
    let (mut seconds, mut attos) = (0i64, 7u64);
    assert_eq!(
        unsafe { hc_tt_from_epoch(core::ptr::null(), 2000.0, &mut seconds, &mut attos) },
        HC_OK
    );
    assert_eq!((seconds, attos), (946_728_000, 0));
    assert_eq!(
        unsafe { hc_epoch_from_tt(core::ptr::null(), 0, 0, &mut year) },
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        unsafe { hc_tt_from_epoch(c"J".as_ptr(), 1e300, &mut seconds, &mut attos) },
        HC_ERROR_OVERFLOW
    );
}

#[test]
fn a_tai64_label_crosses_both_ways() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_tai64_encode(0, 0, c"TAI64".as_ptr(), buffer, capacity, written)
    });
    assert_eq!(text, "4000000000000000\n");
    let (mut seconds, mut attos) = (7i64, 7u64);
    assert_eq!(
        unsafe {
            hc_tai64_decode(
                c"3fffffffffffffff3b9ac9ff3b9ac9ff".as_ptr(),
                &mut seconds,
                &mut attos,
            )
        },
        HC_OK
    );
    assert_eq!((seconds, attos), (-1, 999_999_999_999_999_999));
    // The last label below 2⁶³ is the last second the int64_t holds.
    assert_eq!(
        unsafe { hc_tai64_decode(c"7fffffffffffffff".as_ptr(), &mut seconds, &mut attos) },
        HC_OK
    );
    assert_eq!(seconds, 4_611_686_018_427_387_903);
    assert_eq!(
        unsafe { hc_tai64_decode(c"8000000000000000".as_ptr(), &mut seconds, &mut attos) },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_tai64_decode(c"xyz".as_ptr(), &mut seconds, &mut attos) },
        HC_ERROR_MALFORMED
    );
    assert_eq!(
        unsafe { hc_tai64_decode(core::ptr::null(), &mut seconds, &mut attos) },
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        unsafe {
            hc_tai64_encode(
                0,
                0,
                c"tai32".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
}

/// GPS week 2048 began at 2019-04-06 23:59:42 UTC, TAI second
/// 1 554 595 219.
#[test]
fn the_april_2019_rollover_crosses_the_boundary() {
    let tai = 1_554_595_219;
    let (mut week, mut broadcast, mut tow, mut tow_attos) = (0u32, 0u32, 0u32, 0u64);
    let id = c"gps-lnav-week".as_ptr();
    assert_eq!(
        unsafe {
            hc_gnss_week(
                id,
                tai - 1,
                5,
                &mut week,
                &mut broadcast,
                &mut tow,
                &mut tow_attos,
            )
        },
        HC_OK
    );
    assert_eq!((week, broadcast, tow, tow_attos), (2047, 1023, 604_799, 5));
    let (mut seconds, mut attos) = (0i64, 0u64);
    assert_eq!(
        unsafe { hc_gnss_to_tai(id, 2048, 0, 0, &mut seconds, &mut attos) },
        HC_OK
    );
    assert_eq!((seconds, attos), (tai, 0));
    // The latest instant the table of ranges gives.
    assert_eq!(
        unsafe {
            hc_gnss_to_tai(
                c"beidou-week".as_ptr(),
                u32::MAX,
                604_799,
                0,
                &mut seconds,
                &mut attos,
            )
        },
        HC_OK
    );
    assert_eq!(seconds, 2_597_597_356_694_432);
    assert_eq!(
        unsafe { hc_gnss_resolve_week(id, 1023, c"nearest".as_ptr(), tai, &mut week) },
        HC_OK
    );
    assert_eq!(week, 2047);
    assert_eq!(
        unsafe { hc_gnss_resolve_week(id, 1023, core::ptr::null(), tai, &mut week) },
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        unsafe {
            hc_gnss_week(
                id,
                0,
                0,
                &mut week,
                &mut broadcast,
                &mut tow,
                &mut tow_attos,
            )
        },
        HC_ERROR_NO_DATA
    );
}

#[test]
fn glonass_ole_and_excel_dates_cross_the_boundary() {
    let (mut interval, mut day) = (0u32, 0u32);
    assert_eq!(
        unsafe { hc_glonass_date(1_704_067_200 + 37, 0, 1, &mut interval, &mut day) },
        HC_OK
    );
    assert_eq!((interval, day), (8, 1));
    let mut new_year_1900 = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(1900, 1, 1, &mut new_year_1900) },
        HC_OK
    );
    let (mut fixed, mut seconds) = (0i64, 0f64);
    assert_eq!(
        unsafe { hc_fixed_from_ole_automation(-1.25, &mut fixed, &mut seconds) },
        HC_OK
    );
    assert_eq!((fixed, seconds), (new_year_1900 - 3, 21_600.0));
    let mut value = 0f64;
    assert_eq!(
        unsafe { hc_ole_automation_from_fixed(new_year_1900 - 3, 21_600.0, &mut value) },
        HC_OK
    );
    assert_eq!(value, -1.25);
    let (mut serial_day, mut phantom) = (7i64, 7);
    assert_eq!(
        unsafe { hc_excel_1900_day(60, &mut serial_day, &mut phantom) },
        HC_OK
    );
    assert_eq!((serial_day, phantom), (7, 1), "serial 60 writes no day");
    assert_eq!(
        unsafe { hc_excel_1900_day(1, &mut serial_day, &mut phantom) },
        HC_OK
    );
    assert_eq!((serial_day, phantom), (new_year_1900, 0));
    assert_eq!(
        unsafe { hc_excel_1900_day(2_958_466, &mut serial_day, &mut phantom) },
        HC_ERROR_OUT_OF_RANGE
    );
}

/// Every refusal of the time-scale entry points that their
/// documentation names and the tests above do not reach.
#[test]
fn the_time_scale_entry_points_refuse_as_documented() {
    const TOO_MANY_ATTOSECONDS: u64 = 1_000_000_000_000_000_000;
    let series = c"58479\t27.6740\n58489\t27.6745\n";
    let tai = (58_479 - 40_587) * 86_400 + 37;
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_tt_bipm(
                series.as_ptr(),
                tai,
                TOO_MANY_ATTOSECONDS,
                1,
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_tt_bipm(c"".as_ptr(), tai, 0, 1, buffer, capacity, written)
        }),
        HC_ERROR_NO_DATA
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_tt_bipm(c"58479 27.6".as_ptr(), tai, 0, 1, buffer, capacity, written)
        }),
        HC_ERROR_MALFORMED
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_tt_bipm(core::ptr::null(), tai, 0, 1, buffer, capacity, written)
        }),
        HC_ERROR_NULL_POINTER
    );

    let (mut seconds, mut attos) = (7i64, 7u64);
    for (label, expected) in [
        (c"xyz", HC_ERROR_MALFORMED),
        (c"8000000000000000", HC_ERROR_OUT_OF_RANGE),
        (c"400000000000000a3b9aca00", HC_ERROR_OUT_OF_RANGE),
    ] {
        assert_eq!(
            unsafe { hc_tai64_posix_plus_10_decode(label.as_ptr(), &mut seconds, &mut attos) },
            expected,
            "{label:?}"
        );
    }

    let (mut era, mut offset, mut fraction, mut unix) = (7, 7u32, 7u64, 7i64);
    assert_eq!(
        unsafe {
            hc_ntp_resolve(
                1,
                0,
                i64::MAX,
                &mut era,
                &mut offset,
                &mut fraction,
                &mut unix,
                &mut attos,
            )
        },
        HC_ERROR_OVERFLOW
    );
    assert_eq!(
        unsafe {
            hc_ntp_resolve(
                1,
                0,
                0,
                &mut era,
                &mut offset,
                core::ptr::null_mut(),
                &mut unix,
                &mut attos,
            )
        },
        HC_ERROR_NULL_POINTER
    );

    let (mut fixed, mut seconds_of_day) = (0i64, 0u32);
    assert_eq!(
        unsafe { hc_fat_decode(23_866, 49_021, core::ptr::null_mut(), &mut seconds_of_day) },
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        unsafe { hc_fat_decode(23_866, 49_021, &mut fixed, core::ptr::null_mut()) },
        HC_ERROR_NULL_POINTER
    );
    let (mut date, mut time) = (0u16, 0u16);
    assert_eq!(
        unsafe { hc_fat_encode(739_885, 0, core::ptr::null_mut(), &mut time) },
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        unsafe { hc_fat_encode(739_885, 0, &mut date, core::ptr::null_mut()) },
        HC_ERROR_NULL_POINTER
    );

    let mut beat = 0u16;
    assert_eq!(
        unsafe { hc_swatch_beat(0, TOO_MANY_ATTOSECONDS, &mut beat) },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_swatch_beat(0, 0, core::ptr::null_mut()) },
        HC_ERROR_NULL_POINTER
    );

    let mut year = 0.0f64;
    assert_eq!(
        unsafe { hc_epoch_from_tt(c"X".as_ptr(), 0, 0, &mut year) },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe { hc_epoch_from_tt(c"J".as_ptr(), 0, TOO_MANY_ATTOSECONDS, &mut year) },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_tt_from_epoch(c"X".as_ptr(), 2000.0, &mut seconds, &mut attos) },
        HC_ERROR_UNKNOWN
    );
    for year in [f64::NAN, f64::INFINITY] {
        assert_eq!(
            unsafe { hc_tt_from_epoch(c"J".as_ptr(), year, &mut seconds, &mut attos) },
            HC_ERROR_OUT_OF_RANGE,
            "{year}"
        );
    }
    assert_eq!(
        unsafe { hc_tt_from_epoch(c"J".as_ptr(), 2000.0, core::ptr::null_mut(), &mut attos) },
        HC_ERROR_NULL_POINTER
    );
}
