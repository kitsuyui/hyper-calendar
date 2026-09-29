use super::super::*;
use super::read_lines;

fn relative_time(then: i64, now: i64, style: &str, automatic: i32, locale: &str) -> String {
    read_lines(|buffer, capacity| unsafe {
        hc_relative_time(
            then,
            now,
            style.as_ptr(),
            style.len(),
            automatic,
            locale.as_ptr(),
            locale.len(),
            buffer,
            capacity,
        )
    })
}

/// CLDR 48's English and Russian relative-time patterns (`en.xml`,
/// `ru.xml`, `fields/field[@type="hour"]` and `[@type="day"]`).
#[test]
fn the_lines_are_the_facades() {
    let now = 1_700_000_000;
    assert_eq!(
        relative_time(now - 3 * 3_600, now, "long", 0, "en"),
        "3 hours ago\thour\t-3\ten\n"
    );
    assert_eq!(
        relative_time(now - 86_400, now, "LONG", 1, "en"),
        "yesterday\tday\t-1\ten\n"
    );
    assert_eq!(
        relative_time(now - 86_400, now, "long", 0, "ru"),
        "1 день назад\tday\t-1\tru\n"
    );
    let day = read_lines(|buffer, capacity| unsafe {
        hc_relative_day_at(
            739_887,
            739_888,
            15 * 3_600 + 5 * 60,
            "long".as_ptr(),
            4,
            1,
            "en".as_ptr(),
            2,
            buffer,
            capacity,
        )
    });
    assert_eq!(day, "yesterday at 15:05\tday\t-1\t15:05\ten\n");
    let today = read_lines(|buffer, capacity| unsafe {
        hc_relative_day(
            739_888,
            739_888,
            "long".as_ptr(),
            4,
            1,
            "ja".as_ptr(),
            2,
            buffer,
            capacity,
        )
    });
    assert_eq!(today, "今日\tday\t0\tja\n");
    let span = read_lines(|buffer, capacity| unsafe {
        hc_duration(
            9_000,
            "compact".as_ptr(),
            7,
            0,
            "en".as_ptr(),
            2,
            buffer,
            capacity,
        )
    });
    assert_eq!(span, "2h30m\t0\ten\n");
}

#[test]
fn a_style_not_named_or_a_span_too_long_is_refused() {
    let unknown = unsafe {
        hc_relative_time(
            0,
            0,
            "wide".as_ptr(),
            4,
            0,
            core::ptr::null(),
            0,
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(unknown, HC_ERR_UNKNOWN);
    let far = unsafe {
        hc_relative_time(
            i64::MIN,
            1,
            "long".as_ptr(),
            4,
            0,
            core::ptr::null(),
            0,
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(far, HC_ERR_OUT_OF_RANGE);
    let late = unsafe {
        hc_relative_day_at(
            0,
            0,
            86_400,
            "long".as_ptr(),
            4,
            0,
            core::ptr::null(),
            0,
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(late, HC_ERR_OUT_OF_RANGE);
    let empty = unsafe {
        hc_duration(
            1,
            core::ptr::null(),
            0,
            0,
            core::ptr::null(),
            0,
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(empty, HC_ERR_UNKNOWN);
}
