use super::super::*;

/// The leap second at the end of 2016: TAI second 1 483 228 836
/// from 1970 TAI is 23:59:60, named by POSIX 1 483 228 800, and
/// 00:00:00 on 1 January 2017 is TAI + 37 s.
#[test]
fn the_tai_bridge_names_the_leap_second_of_2016() {
    let new_year = 1_483_228_800;
    let (mut seconds, mut attos) = (0i64, 7u64);
    assert_eq!(
        unsafe { hc_tai_from_unix(new_year, 1, &mut seconds, &mut attos) },
        HC_OK
    );
    assert_eq!((seconds, attos), (new_year + 37, 0));
    let (mut unix, mut leap) = (0i64, 0 as core::ffi::c_int);
    assert_eq!(
        unsafe { hc_utc_from_tai(new_year + 36, 1, &mut unix, &mut leap) },
        HC_OK
    );
    assert_eq!((unix, leap), (new_year, 1));
    assert_eq!(
        unsafe { hc_tai_from_unix(-400_000_000, 1, &mut seconds, &mut attos) },
        HC_ERROR_NO_DATA
    );
    assert_eq!(
        unsafe { hc_tai_from_unix(i64::MAX, 0, &mut seconds, &mut attos) },
        HC_ERROR_OVERFLOW
    );
}

#[test]
fn gregorian_conversion_round_trips_through_the_boundary() {
    let mut fixed = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2026, 9, 21, &mut fixed) },
        HC_OK
    );
    assert_eq!(fixed, 739_880);

    let (mut year, mut month, mut day) = (0i64, 0u8, 0u8);
    assert_eq!(
        unsafe { hc_gregorian_from_fixed(fixed, &mut year, &mut month, &mut day) },
        HC_OK
    );
    assert_eq!((year, month, day), (2026, 9, 21));
}

#[test]
fn invalid_dates_return_a_status_rather_than_panicking() {
    let mut fixed = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2026, 2, 30, &mut fixed) },
        HC_ERROR_INVALID_DATE
    );
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2026, 13, 1, &mut fixed) },
        HC_ERROR_INVALID_DATE
    );
}

#[test]
fn null_out_parameters_are_refused() {
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2026, 9, 21, core::ptr::null_mut()) },
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        unsafe { hc_weekday(0, core::ptr::null_mut()) },
        HC_ERROR_NULL_POINTER
    );
}

#[test]
fn weekdays_use_iso_numbering() {
    let mut weekday = 0u8;
    // 1970-01-01 was a Thursday.
    assert_eq!(unsafe { hc_weekday(719_163, &mut weekday) }, HC_OK);
    assert_eq!(weekday, 4);
    for (fixed, iso) in [(i64::MIN, 6), (i64::MAX, 7)] {
        assert_eq!(unsafe { hc_weekday(fixed, &mut weekday) }, HC_OK);
        assert_eq!(weekday, iso, "{fixed}");
    }
}

/// The twins of the WebAssembly module's `hc_day_of_year`,
/// `hc_is_leap_year`, `hc_fixed_from_unix`, `hc_unix_from_fixed` and
/// `hc_parse_iso_date` answer as they do, through out-parameters.
#[test]
fn the_civil_twins_of_the_webassembly_exports() {
    // 2024-12-31 is day 366 of a leap year, 739 251.
    let (mut day_of_year, mut leap) = (0u32, 0 as core::ffi::c_int);
    assert_eq!(unsafe { hc_day_of_year(739_251, &mut day_of_year) }, HC_OK);
    assert_eq!(day_of_year, 366);
    assert_eq!(unsafe { hc_is_leap_year(739_251, &mut leap) }, HC_OK);
    assert_eq!(leap, 1);
    assert_eq!(unsafe { hc_is_leap_year(739_252, &mut leap) }, HC_OK);
    assert_eq!(leap, 0);
    for outside in [-3_652_425_000, 3_652_424_635] {
        assert_eq!(
            unsafe { hc_day_of_year(outside, &mut day_of_year) },
            HC_ERROR_OUT_OF_RANGE
        );
        assert_eq!(
            unsafe { hc_is_leap_year(outside, &mut leap) },
            HC_ERROR_OUT_OF_RANGE
        );
    }

    let mut fixed = 0i64;
    assert_eq!(unsafe { hc_fixed_from_unix(0, &mut fixed) }, HC_OK);
    assert_eq!(fixed, 719_163);
    assert_eq!(unsafe { hc_fixed_from_unix(-1, &mut fixed) }, HC_OK);
    assert_eq!(fixed, 719_162);
    assert_eq!(unsafe { hc_fixed_from_unix(i64::MIN, &mut fixed) }, HC_OK);
    assert_eq!(fixed, -106_751_990_448_138);
    assert_eq!(unsafe { hc_fixed_from_unix(i64::MAX, &mut fixed) }, HC_OK);
    assert_eq!(fixed, 106_751_991_886_463);

    let mut seconds = 0i64;
    assert_eq!(unsafe { hc_unix_from_fixed(739_880, &mut seconds) }, HC_OK);
    assert_eq!(seconds, 1_789_948_800);
    // No floor: the first day whose midnight fits is answered, as
    // `hc_unix_from_fixed_in_zone` answers it by UTC.
    assert_eq!(
        unsafe { hc_unix_from_fixed(-106_751_990_448_137, &mut seconds) },
        HC_OK
    );
    for outside in [
        -106_751_990_448_138,
        106_751_991_886_464,
        i64::MIN,
        i64::MAX,
    ] {
        assert_eq!(
            unsafe { hc_unix_from_fixed(outside, &mut seconds) },
            HC_ERROR_OUT_OF_RANGE,
            "{outside}"
        );
    }
    assert_eq!(
        unsafe { hc_unix_from_fixed(106_751_991_886_463, &mut seconds) },
        HC_OK
    );

    assert_eq!(
        unsafe { hc_parse_iso_date(c"2026-09-21".as_ptr(), &mut fixed) },
        HC_OK
    );
    assert_eq!(fixed, 739_880);
    assert_eq!(
        unsafe { hc_parse_iso_date(c"2026-02-30".as_ptr(), &mut fixed) },
        HC_ERROR_INVALID_DATE
    );
    assert_eq!(
        unsafe { hc_parse_iso_date(c"\xff".as_ptr(), &mut fixed) },
        HC_ERROR_NOT_UTF8
    );
    assert_eq!(
        unsafe { hc_parse_iso_date(core::ptr::null(), &mut fixed) },
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        unsafe { hc_parse_iso_date(c"2026-09-21".as_ptr(), core::ptr::null_mut()) },
        HC_ERROR_NULL_POINTER
    );
}

#[test]
fn a_short_buffer_reports_the_required_length_and_writes_nothing() {
    let mut buffer = [0 as c_char; 4];
    let mut written = 0usize;
    let status =
        unsafe { hc_format_iso_date(739_880, buffer.as_mut_ptr(), buffer.len(), &mut written) };
    assert_eq!(status, HC_ERROR_BUFFER_TOO_SMALL);
    assert_eq!(written, "2026-09-21".len() + 1);
    assert!(buffer.iter().all(|byte| *byte == 0));
}

#[test]
fn a_sufficient_buffer_receives_a_nul_terminated_iso_date() {
    let mut buffer = [0 as c_char; 32];
    let mut written = 0usize;
    let status =
        unsafe { hc_format_iso_date(739_880, buffer.as_mut_ptr(), buffer.len(), &mut written) };
    assert_eq!(status, HC_OK);
    assert_eq!(written, 11);
    let bytes: Vec<u8> = buffer[..10].iter().map(|byte| *byte as u8).collect();
    assert_eq!(core::str::from_utf8(&bytes).unwrap(), "2026-09-21");
    assert_eq!(buffer[10], 0);
}

#[test]
fn the_leap_second_offset_matches_the_published_table() {
    let mut offset = 0i64;
    assert_eq!(
        unsafe { hc_tai_minus_utc(1_700_000_000, 1, &mut offset) },
        HC_OK
    );
    assert_eq!(offset, 37);
    assert_eq!(
        unsafe { hc_tai_minus_utc(63_072_000, 1, &mut offset) },
        HC_OK
    );
    assert_eq!(offset, 10);
}

#[test]
fn the_strict_policy_refuses_to_forecast_across_the_boundary() {
    let mut offset = 0i64;
    assert_eq!(
        unsafe { hc_tai_minus_utc(4_000_000_000, 1, &mut offset) },
        HC_ERROR_NO_DATA
    );
    assert_eq!(
        unsafe { hc_tai_minus_utc(4_000_000_000, 0, &mut offset) },
        HC_OK
    );
    assert_eq!(offset, 37);
}

#[test]
fn the_leap_second_survives_the_boundary() {
    // 2016-12-31 ends with an inserted second.
    let mut has_leap = 0;
    assert_eq!(
        unsafe { hc_day_has_leap_second(1_483_142_400, 1, &mut has_leap) },
        HC_OK
    );
    assert_eq!(has_leap, 1);
    assert_eq!(
        unsafe { hc_day_has_leap_second(1_483_228_800, 1, &mut has_leap) },
        HC_OK
    );
    assert_eq!(has_leap, 0);
    // 2027-06-30 is past the table's validity (2027-06-28): refused under
    // `strict`, and answered no, as the last offset holds, when it is not.
    assert_eq!(
        unsafe { hc_day_has_leap_second(1_814_313_600, 1, &mut has_leap) },
        HC_ERROR_NO_DATA
    );
    assert_eq!(
        unsafe { hc_day_has_leap_second(1_814_313_600, 0, &mut has_leap) },
        HC_OK
    );
    assert_eq!(has_leap, 0);
}

#[test]
fn the_leap_second_question_refuses_a_day_with_no_int64_bounds() {
    let ask = |unix: i64| {
        let mut has_leap = 7;
        match unsafe { hc_day_has_leap_second(unix, 0, &mut has_leap) } {
            HC_OK => Ok(has_leap),
            status => Err(status),
        }
    };
    // The first whole day of the range, and the part-day before it.
    let first_whole = -9_223_372_036_854_720_000;
    assert_eq!(ask(first_whole), Ok(0));
    assert_eq!(ask(first_whole - 1), Err(HC_ERROR_OUT_OF_RANGE));
    assert_eq!(ask(i64::MIN), Err(HC_ERROR_OUT_OF_RANGE));
    // The last day whose end is an int64_t, and the part-day after.
    let last_end = 9_223_372_036_854_720_000;
    assert_eq!(ask(last_end - 1), Ok(0));
    assert_eq!(ask(last_end), Err(HC_ERROR_OUT_OF_RANGE));
    assert_eq!(ask(i64::MAX), Err(HC_ERROR_OUT_OF_RANGE));
}

#[test]
fn unix_and_tai_round_trip_across_the_boundary() {
    let (mut seconds, mut attos) = (0i64, 0u64);
    assert_eq!(
        unsafe { hc_tai_from_unix(1_700_000_000, 1, &mut seconds, &mut attos) },
        HC_OK
    );
    assert_eq!(seconds, 1_700_000_037);

    let (mut unix_seconds, mut is_leap) = (0i64, 0);
    assert_eq!(
        unsafe { hc_utc_from_tai(seconds, 1, &mut unix_seconds, &mut is_leap) },
        HC_OK
    );
    assert_eq!(unix_seconds, 1_700_000_000);
    assert_eq!(is_leap, 0);
}

#[test]
fn the_int64_results_have_the_readmes_ranges() {
    // The Gregorian range, and the first day on either side of it.
    let (mut fixed, mut year, mut month, mut day) = (0i64, 0i64, 0u8, 0u8);
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(-9_999_999, 1, 1, &mut fixed) },
        HC_OK
    );
    assert_eq!(fixed, -3_652_424_999);
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(9_999_999, 12, 31, &mut fixed) },
        HC_OK
    );
    assert_eq!(fixed, 3_652_424_634);
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(i64::MIN, 1, 1, &mut fixed) },
        HC_ERROR_INVALID_DATE
    );
    for outside in [-3_652_425_000, 3_652_424_635, i64::MIN, i64::MAX] {
        assert_eq!(
            unsafe { hc_gregorian_from_fixed(outside, &mut year, &mut month, &mut day) },
            HC_ERROR_OUT_OF_RANGE,
            "{outside}"
        );
    }
    // TAI runs 37 seconds ahead, so the last 37 POSIX seconds have
    // no TAI reading an int64_t holds.
    let (mut seconds, mut attos) = (0i64, 0u64);
    assert_eq!(
        unsafe { hc_tai_from_unix(i64::MAX - 37, 0, &mut seconds, &mut attos) },
        HC_OK
    );
    assert_eq!(seconds, i64::MAX);
    assert_eq!(
        unsafe { hc_tai_from_unix(i64::MAX - 36, 0, &mut seconds, &mut attos) },
        HC_ERROR_OVERFLOW
    );
    assert_eq!(
        unsafe { hc_tai_from_unix(i64::MIN, 0, &mut seconds, &mut attos) },
        HC_OK
    );
    assert_eq!(seconds, i64::MIN);
    // Every TAI reading has a UTC label.
    let (mut unix_seconds, mut is_leap) = (0i64, 0);
    for tai in [i64::MIN, i64::MAX] {
        assert_eq!(
            unsafe { hc_utc_from_tai(tai, 0, &mut unix_seconds, &mut is_leap) },
            HC_OK,
            "{tai}"
        );
    }
    let mut offset = 0i64;
    for unix in [i64::MIN, i64::MAX] {
        assert_eq!(
            unsafe { hc_tai_minus_utc(unix, 0, &mut offset) },
            HC_OK,
            "{unix}"
        );
    }
}

#[test]
fn the_inserted_second_is_reachable_through_the_boundary() {
    // The TAI reading one second before the 2017 step is 23:59:60 UTC.
    let (mut unix_seconds, mut is_leap) = (0i64, 0);
    assert_eq!(
        unsafe { hc_utc_from_tai(1_483_228_836, 1, &mut unix_seconds, &mut is_leap) },
        HC_OK
    );
    assert_eq!(unix_seconds, 1_483_228_800);
    assert_eq!(is_leap, 1);
}
