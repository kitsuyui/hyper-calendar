use super::super::*;
use super::read_lines;

#[test]
fn holiday_tables_answer_by_identifier() {
    use core::ffi::CStr;
    let jp = c"JP";
    let xnys = c"XNYS";
    let us = c"US";
    let zz = c"ZZ";
    let mut fixed = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2026, 4, 3, &mut fixed) },
        HC_OK
    );
    let mut answer = -1;
    assert_eq!(
        unsafe {
            hc_holiday_is_day_off(
                xnys.as_ptr(),
                core::ptr::null(),
                core::ptr::null(),
                fixed,
                &mut answer,
            )
        },
        HC_OK
    );
    assert_eq!(answer, 1);
    assert_eq!(
        unsafe {
            hc_holiday_is_day_off(
                us.as_ptr(),
                core::ptr::null(),
                core::ptr::null(),
                fixed,
                &mut answer,
            )
        },
        HC_OK
    );
    assert_eq!(answer, 0);
    assert_eq!(
        unsafe {
            hc_holiday_is_day_off(
                zz.as_ptr(),
                core::ptr::null(),
                core::ptr::null(),
                fixed,
                &mut answer,
            )
        },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_holiday_is_day_off(
                jp.as_ptr(),
                core::ptr::null(),
                core::ptr::null(),
                fixed,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        unsafe {
            hc_holiday_is_day_off(
                core::ptr::null(),
                core::ptr::null(),
                core::ptr::null(),
                fixed,
                &mut answer,
            )
        },
        HC_ERROR_NULL_POINTER
    );
    // A string that is not UTF-8 says so, in the code and in the region
    // alike, rather than passing for a null pointer or for no region.
    let not_utf8 = c"\xff";
    assert_eq!(
        unsafe {
            hc_holiday_is_day_off(
                not_utf8.as_ptr(),
                core::ptr::null(),
                core::ptr::null(),
                fixed,
                &mut answer,
            )
        },
        HC_ERROR_NOT_UTF8
    );
    assert_eq!(
        unsafe {
            hc_holiday_is_day_off(
                jp.as_ptr(),
                not_utf8.as_ptr(),
                core::ptr::null(),
                fixed,
                &mut answer,
            )
        },
        HC_ERROR_NOT_UTF8
    );
    let mut written = 0usize;
    assert_eq!(
        unsafe {
            hc_holidays_in_year(
                zz.as_ptr(),
                core::ptr::null(),
                core::ptr::null(),
                core::ptr::null(),
                2026,
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_UNKNOWN
    );
    // Too small reports the need; big enough receives the lines.
    assert_eq!(
        unsafe {
            hc_holidays_in_year(
                jp.as_ptr(),
                core::ptr::null(),
                core::ptr::null(),
                core::ptr::null(),
                2026,
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_BUFFER_TOO_SMALL
    );
    assert!(written > 1);
    let mut buffer = vec![0 as c_char; written];
    assert_eq!(
        unsafe {
            hc_holidays_in_year(
                jp.as_ptr(),
                core::ptr::null(),
                core::ptr::null(),
                core::ptr::null(),
                2026,
                buffer.as_mut_ptr(),
                buffer.len(),
                &mut written,
            )
        },
        HC_OK
    );
    let text = unsafe { CStr::from_ptr(buffer.as_ptr()) }
        .to_str()
        .expect("UTF-8");
    assert!(
        text.starts_with(
            "2026-01-01\tNew Year's Day\t元日\tpublic\texact\t0\t\t\t\tnew-years-day\t\t0\n"
        ),
        "{text}"
    );
    assert!(
        text.contains("\t1\t2026-05-03\t\t\tconstitution-memorial-day\t\t0\n"),
        "{text}"
    );
    assert_eq!(
        unsafe { hc_holiday_codes(core::ptr::null_mut(), 0, &mut written) },
        HC_ERROR_BUFFER_TOO_SMALL
    );
    let mut buffer = vec![0 as c_char; written];
    assert_eq!(
        unsafe { hc_holiday_codes(buffer.as_mut_ptr(), buffer.len(), &mut written) },
        HC_OK
    );
    let text = unsafe { CStr::from_ptr(buffer.as_ptr()) }
        .to_str()
        .expect("UTF-8");
    assert!(text.contains("\nJP\n") && text.contains("\nXNYS\n") && text.contains("un-days\n"));
}

#[test]
fn one_day_across_every_table_decodes_column_by_column() {
    let mut day = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2026, 5, 6, &mut day) },
        HC_OK
    );
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_holidays_on(day, buffer, capacity, written)
    });
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert!(rows.iter().all(|row| row.len() == 13), "{rows:?}");
    let substitute = rows
        .iter()
        .find(|row| row[0] == "JP" && row[7] == "1")
        .expect("Japan's substitute for Constitution Memorial Day");
    assert_eq!(
        substitute[1..7],
        [
            "Japan",
            "Constitution Memorial Day",
            "憲法記念日",
            "public",
            "exact",
            ""
        ]
    );
    assert_eq!(substitute[8], (day - 3).to_string());
    assert_eq!(
        unsafe { hc_holidays_on(i64::MAX, core::ptr::null_mut(), 0, core::ptr::null_mut()) },
        HC_ERROR_OUT_OF_RANGE
    );
}

#[test]
fn a_group_is_a_scope_of_its_country_s_table() {
    let cn = c"CN";
    let children = c"children";
    let mut day = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2026, 6, 1, &mut day) },
        HC_OK
    );
    let mut answer = -1;
    assert_eq!(
        unsafe {
            hc_holiday_is_day_off(
                cn.as_ptr(),
                core::ptr::null(),
                children.as_ptr(),
                day,
                &mut answer,
            )
        },
        HC_OK
    );
    assert_eq!(answer, 1);
    assert_eq!(
        unsafe {
            hc_holiday_is_day_off(
                cn.as_ptr(),
                core::ptr::null(),
                core::ptr::null(),
                day,
                &mut answer,
            )
        },
        HC_OK
    );
    assert_eq!(answer, 0);
    let women = c"women";
    let year = read_lines(|buffer, capacity, written| unsafe {
        hc_holidays_in_year(
            cn.as_ptr(),
            core::ptr::null(),
            women.as_ptr(),
            core::ptr::null(),
            2026,
            buffer,
            capacity,
            written,
        )
    });
    assert!(
        year.lines().any(|line| line.starts_with(
            "2026-03-08\tWomen's Day\t妇女节\thalf-day\texact\t0\t\t\twomen\twomens-day\t"
        )),
        "{year}"
    );
    let unknown = c"childrens";
    assert_eq!(
        unsafe {
            hc_holiday_is_day_off(
                cn.as_ptr(),
                core::ptr::null(),
                unknown.as_ptr(),
                day,
                &mut answer,
            )
        },
        HC_ERROR_UNKNOWN
    );
    let mut written = 0;
    assert_eq!(
        unsafe {
            hc_holidays_in_year(
                cn.as_ptr(),
                core::ptr::null(),
                unknown.as_ptr(),
                core::ptr::null(),
                2026,
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_UNKNOWN
    );
    let police = c"police";
    assert_eq!(
        unsafe {
            hc_holiday_is_day_off(
                cn.as_ptr(),
                core::ptr::null(),
                police.as_ptr(),
                day,
                &mut answer,
            )
        },
        HC_OK
    );
    let not_utf8 = c"\xff";
    assert_eq!(
        unsafe {
            hc_holiday_is_day_off(
                cn.as_ptr(),
                core::ptr::null(),
                not_utf8.as_ptr(),
                day,
                &mut answer,
            )
        },
        HC_ERROR_NOT_UTF8
    );
}

/// The fixed day of a Gregorian date.
fn day(year: i64, month: u8, day: u8) -> i64 {
    let mut fixed = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(year, month, day, &mut fixed) },
        HC_OK
    );
    fixed
}

/// A day the table cannot answer is refused, not answered "no": Victoria
/// Day 2025 is a gap in Newfoundland and Labrador, and Kedah's weekend law
/// of 2012 was not read.
#[test]
fn a_day_a_gap_leaves_open_is_refused_and_a_known_one_is_answered() {
    let day_off = |code: &core::ffi::CStr, region: &core::ffi::CStr, fixed: i64| {
        let mut answer = -1;
        let status = unsafe {
            hc_holiday_is_day_off(
                code.as_ptr(),
                region.as_ptr(),
                core::ptr::null(),
                fixed,
                &mut answer,
            )
        };
        (status, answer)
    };
    assert_eq!(
        day_off(c"CA", c"CA-NL", day(2025, 5, 19)),
        (HC_ERROR_NO_DATA, -1)
    );
    assert_eq!(day_off(c"CA", c"CA-NL", day(2025, 12, 25)), (HC_OK, 1));
    assert_eq!(day_off(c"JP", c"", day(2026, 3, 4)), (HC_OK, 0));
    assert_eq!(
        day_off(c"MY", c"MY-02", day(2012, 5, 11)),
        (HC_ERROR_OUT_OF_RANGE, -1)
    );
    // The walk across the same Victoria Day is open, and so is a count.
    let mut moved = 0i64;
    assert_eq!(
        unsafe {
            hc_holiday_add_business_days(
                c"CA".as_ptr(),
                c"CA-NL".as_ptr(),
                core::ptr::null(),
                day(2025, 5, 16),
                1,
                &mut moved,
            )
        },
        HC_ERROR_NO_DATA
    );
    assert_eq!(moved, 0);
    let mut count = 0i64;
    assert_eq!(
        unsafe {
            hc_holiday_business_days_between(
                c"MY".as_ptr(),
                c"MY-02".as_ptr(),
                core::ptr::null(),
                day(2012, 5, 9),
                day(2012, 5, 14),
                &mut count,
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
}

/// Kedah keeps Friday and Saturday from 25 November 2013 (ADR 0015); the
/// law before it was not read.
#[test]
fn the_weekend_of_a_region_is_one_or_zero_and_refused_where_unread() {
    let weekend = |code: &core::ffi::CStr, region: *const core::ffi::c_char, fixed: i64| {
        let mut answer = -1;
        let status = unsafe { hc_holiday_is_weekend(code.as_ptr(), region, fixed, &mut answer) };
        (status, answer)
    };
    let friday = day(2026, 3, 6);
    assert_eq!(weekend(c"MY", c"MY-02".as_ptr(), friday), (HC_OK, 1));
    assert_eq!(weekend(c"MY", core::ptr::null(), friday), (HC_OK, 0));
    assert_eq!(weekend(c"MY", core::ptr::null(), friday + 1), (HC_OK, 1));
    assert_eq!(weekend(c"AE", c"AE-SH".as_ptr(), friday), (HC_OK, 1));
    assert_eq!(
        weekend(c"MY", c"MY-02".as_ptr(), day(2013, 11, 24)),
        (HC_ERROR_OUT_OF_RANGE, -1)
    );
    assert_eq!(
        weekend(c"MY", c"MY-02".as_ptr(), day(2013, 11, 29)),
        (HC_OK, 1)
    );
    assert_eq!(
        weekend(c"ZZ", core::ptr::null(), friday),
        (HC_ERROR_UNKNOWN, -1)
    );
    assert_eq!(
        weekend(c"MY", c"MY-99".as_ptr(), friday),
        (HC_ERROR_UNKNOWN, -1)
    );
    assert_eq!(
        unsafe { hc_holiday_is_weekend(core::ptr::null(), core::ptr::null(), friday, &mut 0) },
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        unsafe {
            hc_holiday_is_weekend(
                c"MY".as_ptr(),
                core::ptr::null(),
                friday,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_NULL_POINTER
    );
}

/// `hc_holiday_next` or `hc_holiday_previous` for a table, a region and a
/// kind, with no group.
fn beyond(
    forward: bool,
    (code, region, kind): (&core::ffi::CStr, &core::ffi::CStr, &core::ffi::CStr),
    fixed: i64,
    buffer: *mut core::ffi::c_char,
    capacity: usize,
    written: *mut usize,
) -> HcStatus {
    let call = if forward {
        hc_holiday_next
    } else {
        hc_holiday_previous
    };
    unsafe {
        call(
            code.as_ptr(),
            region.as_ptr(),
            core::ptr::null(),
            kind.as_ptr(),
            fixed,
            buffer,
            capacity,
            written,
        )
    }
}

/// The next holiday of Japan after Tuesday 28 April 2026 is Shōwa Day; the
/// last before 7 May is the substitute for 3 May; a gap that could hide a
/// nearer one is refused.
#[test]
fn the_next_and_the_previous_holiday_are_a_line_of_the_year() {
    let jp = (c"JP", c"", c"");
    let line = |forward: bool, scope, fixed: i64| {
        read_lines(|buffer, capacity, written| {
            beyond(forward, scope, fixed, buffer, capacity, written)
        })
    };
    let next = line(true, jp, day(2026, 4, 28));
    let row: Vec<&str> = next.trim_end().split('\t').collect();
    assert_eq!(row.len(), 12, "{next}");
    assert_eq!(
        row[..5],
        ["2026-04-29", "Shōwa Day", "昭和の日", "public", "exact"]
    );
    assert_eq!(row[9], "showa-day");
    let previous = line(false, jp, day(2026, 5, 7));
    let row: Vec<&str> = previous.trim_end().split('\t').collect();
    assert_eq!(row[0], "2026-05-06");
    assert_eq!((row[5], row[6]), ("1", "2026-05-03"));
    // The bridge of 22 September 2009 says so in the last cell.
    let bridge = line(true, jp, day(2009, 9, 21));
    let row: Vec<&str> = bridge.trim_end().split('\t').collect();
    assert_eq!(row[..2], ["2009-09-22", "Citizens' Holiday"]);
    assert_eq!(row[11], "1");
    let refuse = |forward: bool, scope, fixed: i64| {
        let mut written = 0usize;
        beyond(
            forward,
            scope,
            fixed,
            core::ptr::null_mut(),
            0,
            &mut written,
        )
    };
    let nl = (c"CA", c"CA-NL", c"");
    assert_eq!(refuse(true, nl, day(2025, 5, 1)), HC_ERROR_NO_DATA);
    assert_eq!(refuse(false, nl, day(2025, 7, 15)), HC_ERROR_NO_DATA);
    let new_year = day(2026, 1, 1);
    assert_eq!(
        refuse(true, (c"un-days", c"", c""), new_year),
        HC_ERROR_NO_DATA
    );
    assert_eq!(refuse(true, (c"ZZ", c"", c""), new_year), HC_ERROR_UNKNOWN);
    assert_eq!(
        refuse(true, (c"JP", c"", c"festival"), new_year),
        HC_ERROR_UNKNOWN
    );
    assert_eq!(refuse(true, jp, 1 << 62), HC_ERROR_OUT_OF_RANGE);
}

/// A region that has only a weekend law has substitute days of its own:
/// Awal Muharram 2025 was Friday 27 June, which Kedah moves to Sunday the
/// 29th.
#[test]
fn a_regions_substitute_day_is_a_line_of_the_day_it_falls_on() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_holidays_on(day(2025, 6, 29), buffer, capacity, written)
    });
    let kedah: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect::<Vec<_>>())
        .filter(|row| row[0] == "MY" && row[9] == "MY-02")
        .collect();
    assert!(
        kedah.iter().any(|row| row[7] == "1"
            && row[8] == day(2025, 6, 27).to_string()
            && row[4] == "public"),
        "{kedah:?}"
    );
    assert!(text.lines().all(|line| line.split('\t').count() == 13));
}
