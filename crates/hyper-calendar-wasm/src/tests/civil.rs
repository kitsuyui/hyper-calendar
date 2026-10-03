use super::super::*;

#[test]
fn gregorian_conversion_matches_the_fixed_day() {
    assert_eq!(hc_gregorian_to_fixed(2026, 9, 21), 739_880);
    assert_eq!(hc_gregorian_year(739_880), 2026);
    assert_eq!(hc_gregorian_month(739_880), 9);
    assert_eq!(hc_gregorian_day(739_880), 21);
    assert_eq!(hc_weekday(739_880), 1);
}

#[test]
fn errors_are_sentinels_rather_than_traps() {
    assert!(hc_gregorian_to_fixed(2026, 2, 30) <= HC_ERR_FLOOR);
    assert!(hc_gregorian_to_fixed(2026, 13, 1) <= HC_ERR_FLOOR);
    assert!(hc_gregorian_to_fixed(2026, 1, 300) <= HC_ERR_FLOOR);
    // A legitimate answer is never mistaken for a sentinel.
    assert!(hc_gregorian_to_fixed(-9_000, 1, 1) > HC_ERR_FLOOR);
}

#[test]
fn text_is_written_without_a_terminator_and_reports_its_length() {
    let mut buffer = [0u8; 32];
    let written = unsafe { hc_format_iso_date(739_880, buffer.as_mut_ptr(), buffer.len()) };
    assert_eq!(written, 10);
    assert_eq!(&buffer[..10], b"2026-09-21");
    // The byte after the text is untouched, not a NUL the caller must skip.
    assert_eq!(buffer[10], 0);
}

#[test]
fn a_short_buffer_is_reported_rather_than_truncated() {
    let mut buffer = [0u8; 4];
    let written = unsafe { hc_format_iso_date(739_880, buffer.as_mut_ptr(), buffer.len()) };
    assert_eq!(written, HC_ERR_BUFFER_TOO_SMALL);
    assert_eq!(buffer, [0u8; 4]);
}

#[test]
fn iso_dates_round_trip_through_the_boundary() {
    let text = "2026-09-21";
    let parsed = unsafe { hc_parse_iso_date(text.as_ptr(), text.len()) };
    assert_eq!(parsed, 739_880);

    let mut buffer = [0u8; 32];
    let written = unsafe { hc_format_iso_date(parsed, buffer.as_mut_ptr(), buffer.len()) };
    assert_eq!(&buffer[..written as usize], text.as_bytes());
}

#[test]
fn malformed_input_is_rejected_without_trapping() {
    for text in ["", "not a date", "2026-13-01", "2026-02-30"] {
        let result = unsafe { hc_parse_iso_date(text.as_ptr(), text.len()) };
        assert!(result <= HC_ERR_FLOOR, "{text} gave {result}");
    }
    assert_eq!(
        unsafe { hc_parse_iso_date(core::ptr::null(), 0) },
        HC_ERR_INVALID_DATE
    );
    assert_eq!(
        unsafe { hc_parse_iso_date(core::ptr::null(), 10) },
        HC_ERR_NULL_POINTER
    );
    let not_utf8 = [0xffu8, b'-', b'0'];
    assert_eq!(
        unsafe { hc_parse_iso_date(not_utf8.as_ptr(), not_utf8.len()) },
        HC_ERR_NOT_UTF8
    );
}

#[test]
fn unix_time_maps_onto_fixed_days() {
    assert_eq!(hc_fixed_from_unix(0), 719_163);
    assert_eq!(hc_unix_from_fixed(719_163), 0);
    assert_eq!(hc_fixed_from_unix(-1), 719_162);
}

#[test]
fn the_gregorian_range_is_the_readmes() {
    let earliest = -3_652_424_999;
    let latest = 3_652_424_634;
    assert_eq!(hc_gregorian_to_fixed(-9_999_999, 1, 1), earliest);
    assert_eq!(hc_gregorian_to_fixed(9_999_999, 12, 31), latest);
    assert_eq!(
        hc_gregorian_to_fixed(-10_000_000, 12, 31),
        HC_ERR_INVALID_DATE
    );
    assert_eq!(hc_gregorian_to_fixed(10_000_000, 1, 1), HC_ERR_INVALID_DATE);
    assert_eq!(hc_gregorian_year(earliest), -9_999_999);
    assert_eq!(hc_gregorian_year(latest), 9_999_999);
    assert_eq!(hc_gregorian_year(earliest - 1), HC_ERR_OUT_OF_RANGE);
    assert_eq!(hc_gregorian_year(latest + 1), HC_ERR_OUT_OF_RANGE);
    assert_eq!(hc_gregorian_year(i64::MIN), HC_ERR_OUT_OF_RANGE);
    assert_eq!(hc_gregorian_to_fixed(i64::MIN, 1, 1), HC_ERR_INVALID_DATE);
    assert_eq!(hc_gregorian_to_fixed(i64::MAX, 12, 31), HC_ERR_INVALID_DATE);
    for text in [
        "-9999999-01-01",
        "+9999999-12-31",
        "-10000000-12-31",
        "+99999999999999999999-01-01",
        "-99999999999999999999-01-01",
    ] {
        let parsed = unsafe { hc_parse_iso_date(text.as_ptr(), text.len()) };
        assert!(
            parsed == HC_ERR_INVALID_DATE || (earliest..=latest).contains(&parsed),
            "{text}: {parsed}"
        );
    }
    // Every day has a weekday, to both ends of an i64.
    assert_eq!(hc_weekday(i64::MIN), 6);
    assert_eq!(hc_weekday(i64::MAX), 7);
}

/// The first fixed day `hc_unix_from_fixed` answers for, and the
/// last: the README's range.
const UNIX_FROM_FIXED_FIRST: i64 = -104_165_947_503;
const UNIX_FROM_FIXED_LAST: i64 = 106_751_991_886_463;

#[test]
fn midnight_in_seconds_never_reads_as_a_sentinel() {
    // The last day whose midnight is above the floor, and the first
    // whose midnight would reach it.
    let first = hc_unix_from_fixed(UNIX_FROM_FIXED_FIRST);
    assert_eq!(first, -8_999_999_999_942_400);
    assert!(first > HC_ERR_FLOOR);
    assert_eq!(
        hc_unix_from_fixed(UNIX_FROM_FIXED_FIRST - 1),
        HC_ERR_OUT_OF_RANGE
    );
    // The day the report was about, some 54 billion years back.
    assert_eq!(hc_unix_from_fixed(-19_723_095_000_000), HC_ERR_OUT_OF_RANGE);
}

#[test]
fn midnight_in_seconds_refuses_rather_than_overflows() {
    // The last day whose midnight fits an i64, and the first that
    // would not.
    assert_eq!(
        hc_unix_from_fixed(UNIX_FROM_FIXED_LAST),
        9_223_372_036_854_720_000
    );
    assert_eq!(
        hc_unix_from_fixed(UNIX_FROM_FIXED_LAST + 1),
        HC_ERR_OUT_OF_RANGE
    );
    // 2^53 days, whose midnight a wrapping product would put at
    // 3458764451684857728.
    assert_eq!(hc_unix_from_fixed(1 << 53), HC_ERR_OUT_OF_RANGE);
    assert_eq!(hc_unix_from_fixed(i64::MAX), HC_ERR_OUT_OF_RANGE);
    // Overflow downwards too: the subtraction, then the product.
    assert_eq!(hc_unix_from_fixed(i64::MIN), HC_ERR_OUT_OF_RANGE);
    assert_eq!(hc_unix_from_fixed(i64::MIN / 86_400), HC_ERR_OUT_OF_RANGE);
}

#[test]
fn every_timestamp_has_a_fixed_day_above_the_floor() {
    assert_eq!(hc_fixed_from_unix(i64::MIN), -106_751_990_448_138);
    assert_eq!(hc_fixed_from_unix(i64::MAX), UNIX_FROM_FIXED_LAST);
    assert_eq!(
        hc_unix_from_fixed(hc_fixed_from_unix(i64::MAX)),
        9_223_372_036_854_720_000
    );
}

#[test]
fn the_leap_second_question_refuses_a_day_with_no_i64_bounds() {
    // The first whole day of the range, and the part-day before it.
    let first_whole = -9_223_372_036_854_720_000;
    assert_eq!(hc_day_has_leap_second(first_whole, 0), 0);
    assert_eq!(
        hc_day_has_leap_second(first_whole - 1, 0),
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(hc_day_has_leap_second(i64::MIN, 0), HC_ERR_OUT_OF_RANGE);
    // The last day whose end is an i64, and the part-day after it.
    let last_end = 9_223_372_036_854_720_000;
    assert_eq!(hc_day_has_leap_second(last_end - 1, 0), 0);
    assert_eq!(hc_day_has_leap_second(last_end, 0), HC_ERR_OUT_OF_RANGE);
    assert_eq!(hc_day_has_leap_second(i64::MAX, 0), HC_ERR_OUT_OF_RANGE);
}

#[test]
fn the_leap_second_table_is_reachable() {
    assert_eq!(hc_tai_minus_utc(1_700_000_000, 1), 37);
    assert_eq!(hc_day_has_leap_second(1_483_142_400, 1), 1);
    assert_eq!(hc_day_has_leap_second(1_483_228_800, 1), 0);
    // Past the announced table the strict policy refuses.
    assert!(hc_tai_minus_utc(4_000_000_000, 1) <= HC_ERR_FLOOR);
    assert_eq!(hc_tai_minus_utc(4_000_000_000, 0), 37);
    // 2027-06-30, past the table's validity (2027-06-28), is not announced:
    // refused under `strict`, and answered no, as the last offset holds, when
    // it is not.
    assert_eq!(hc_day_has_leap_second(1_814_313_600, 1), HC_ERR_NO_DATA);
    assert_eq!(hc_day_has_leap_second(1_814_313_600, 0), 0);
    assert_eq!(hc_day_has_leap_second(1_814_054_400, 1), 0);
}

#[test]
fn day_of_year_and_leap_years_are_exposed() {
    let leap_day = hc_gregorian_to_fixed(2024, 12, 31);
    assert_eq!(hc_day_of_year(leap_day), 366);
    assert_eq!(hc_is_leap_year(leap_day), 1);
    let common = hc_gregorian_to_fixed(2023, 12, 31);
    assert_eq!(hc_day_of_year(common), 365);
    assert_eq!(hc_is_leap_year(common), 0);
}
