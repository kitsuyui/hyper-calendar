//! The functions of Python's `time` and `calendar` modules, and the week of
//! the year under a week rule, across the boundary: the values are the
//! Python 3.13 documentation's examples and UTS #35's.

use super::super::*;
use super::read_lines;

/// `time.gmtime(0)` from the `time` module's documentation: the epoch is a
/// Thursday, day 1 of 1970; `calendar.timegm` is its inverse, and a day 32
/// is the first of the next month.
#[test]
fn gmtime_and_timegm_are_each_others_inverse() {
    let text = read_lines(|buffer, capacity| unsafe { hc_gmtime(0, buffer, capacity) });
    assert_eq!(text, "1970\t1\t1\t0\t0\t0\t3\t1\t0\n");
    assert_eq!(hc_timegm(1970, 1, 1, 0, 0, 0), 0);
    assert_eq!(
        hc_timegm(2026, 1, 32, 0, 0, 0),
        hc_timegm(2026, 2, 1, 0, 0, 0)
    );
    assert_eq!(hc_timegm(2026, 13, 1, 0, 0, 0), HC_ERR_INVALID_DATE);
    assert_eq!(hc_timegm(i64::MAX, 1, 1, 0, 0, 0), HC_ERR_OUT_OF_RANGE);
    assert_eq!(
        hc_timegm(9_999_999, 12, 31, i64::MAX, 0, 0),
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_gmtime(i64::MAX, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

/// The `calendar` module: `monthrange(2026, 2)` is a Sunday and 28 days,
/// `weekday(2026, 9, 21)` is Monday 0, and `monthcalendar` lays February
/// 2026 out from Monday with the 1st alone in its first week.
#[test]
fn the_calendar_functions_answer_as_pythons() {
    assert_eq!(hc_isleap(2024), 1);
    assert_eq!(hc_isleap(2100), 0);
    assert_eq!(hc_leapdays(2000, 2026), 7);
    assert_eq!(hc_leapdays(2026, 2000), -7);
    assert_eq!(hc_leapdays(2000, i64::MIN), HC_ERR_OUT_OF_RANGE);
    assert_eq!(hc_calendar_weekday(2026, 9, 21), 0);
    assert_eq!(hc_calendar_weekday(2026, 2, 30), HC_ERR_INVALID_DATE);
    assert_eq!(hc_calendar_weekday(2026, 300, 1), HC_ERR_INVALID_DATE);
    let range = read_lines(|buffer, capacity| unsafe { hc_monthrange(2026, 2, buffer, capacity) });
    assert_eq!(range, "6\t28\n");
    assert_eq!(
        unsafe { hc_monthrange(2026, 0, core::ptr::null_mut(), 0) },
        HC_ERR_INVALID_DATE
    );
    let february =
        read_lines(|buffer, capacity| unsafe { hc_monthcalendar(2026, 2, 0, buffer, capacity) });
    let rows: Vec<&str> = february.lines().collect();
    assert_eq!(rows.len(), 5, "{february}");
    assert_eq!(rows[0], "0\t0\t0\t0\t0\t0\t1");
    assert_eq!(rows[4], "23\t24\t25\t26\t27\t28\t0");
    assert_eq!(
        unsafe { hc_monthcalendar(2026, 2, 7, core::ptr::null_mut(), 0) },
        HC_ERR_INVALID_DATE
    );
}

/// UTS #35's example and ISO 8601's: 1 January 2021, a Friday, is week 1 of
/// 2021 under Sunday and 1 and week 53 of 2020 under Monday and 4.
#[test]
fn the_week_of_the_year_follows_the_rule() {
    let new_year = hc_gregorian_to_fixed(2021, 1, 1);
    let week = |first: u32, min: u32| {
        read_lines(|buffer, capacity| unsafe {
            hc_week_of_year(new_year, first, min, buffer, capacity)
        })
    };
    assert_eq!(week(7, 1), "2021\t1\t52\t1\n");
    assert_eq!(week(1, 4), "2020\t53\t53\t0\n");
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_week_of_year(new_year, 0, 4, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_week_of_year(new_year, 1, 8, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_week_of_year(1 << 62, 1, 4, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

/// `time.localtime` and `time.mktime` in a zone: the epoch is 09:00 in
/// Tokyo, and Berlin's 02:30 on 29 March 2026 does not exist.
#[cfg(feature = "tz")]
#[test]
fn localtime_and_mktime_read_a_zone() {
    let tokyo = "Asia/Tokyo";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_localtime(0, tokyo.as_ptr(), tokyo.len(), buffer, capacity)
    });
    assert_eq!(text, "1970\t1\t1\t9\t0\t0\t3\t1\t0\n");
    let mktime = |fields: [i64; 6], zone: &str, policy: &str| unsafe {
        hc_mktime(
            fields[0],
            fields[1],
            fields[2],
            fields[3],
            fields[4],
            fields[5],
            zone.as_ptr(),
            zone.len(),
            policy.as_ptr(),
            policy.len(),
        )
    };
    assert_eq!(mktime([1970, 1, 1, 9, 0, 0], tokyo, "reject"), 0);
    let berlin = "Europe/Berlin";
    assert_eq!(
        mktime([2026, 3, 29, 2, 30, 0], berlin, "reject"),
        HC_ERR_INVALID_DATE
    );
    let earliest = mktime([2026, 10, 25, 2, 30, 0], berlin, "earliest");
    let latest = mktime([2026, 10, 25, 2, 30, 0], berlin, "latest");
    assert_eq!(latest - earliest, 3600);
    assert_eq!(
        mktime([2026, 1, 1, 0, 0, 0], "Mars/Olympus", "reject"),
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        mktime([2026, 1, 1, 0, 0, 0], tokyo, "guess"),
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        mktime([2026, 2, 30, 0, 0, 0], tokyo, "reject"),
        HC_ERR_INVALID_DATE
    );
    let mars = "Mars/Olympus";
    assert_eq!(
        unsafe { hc_localtime(0, mars.as_ptr(), mars.len(), core::ptr::null_mut(), 0) },
        HC_ERR_UNKNOWN
    );
}
