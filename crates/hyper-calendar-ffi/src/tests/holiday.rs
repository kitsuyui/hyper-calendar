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
            "2026-01-01\tNew Year's Day\t元日\tpublic\texact\t0\t\t\t\tnew-years-day\t\n"
        ),
        "{text}"
    );
    assert!(
        text.contains("\t1\t2026-05-03\t\t\tconstitution-memorial-day\t\n"),
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
    assert!(rows.iter().all(|row| row.len() == 12), "{rows:?}");
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
