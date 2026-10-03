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

/// The examples of `humanize` 4.16's documentation of its number functions
/// (`apnumber`, `fractional`, `scientific`, `metric`, `intword`), of
/// `naturalsize` and of `natural_list`; the language cell is `en`.
#[test]
fn the_number_lines_are_pythons_humanize() {
    let apnumber =
        |value| read_lines(|buffer, capacity| unsafe { hc_apnumber(value, buffer, capacity) });
    assert_eq!(apnumber(5), "five\ten\n");
    assert_eq!(apnumber(10), "10\ten\n");
    assert_eq!(apnumber(-1), "-1\ten\n");
    let fractional =
        |value| read_lines(|buffer, capacity| unsafe { hc_fractional(value, buffer, capacity) });
    assert_eq!(fractional(1.3), "1 3/10\ten\n");
    assert_eq!(fractional(0.3), "3/10\ten\n");
    assert_eq!(fractional(f64::NAN), "NaN\ten\n");
    let scientific = |value, precision| {
        read_lines(|buffer, capacity| unsafe { hc_scientific(value, precision, buffer, capacity) })
    };
    assert_eq!(scientific(0.3, 2), "3.00 x 10⁻¹\ten\n");
    assert_eq!(scientific(1000.0, 3), "1.000 x 10³\ten\n");
    let metric = |value, unit: &str, precision| {
        read_lines(|buffer, capacity| unsafe {
            hc_metric(
                value,
                unit.as_ptr(),
                unit.len(),
                precision,
                buffer,
                capacity,
            )
        })
    };
    assert_eq!(metric(1500.0, "V", 3), "1.50 kV\ten\n");
    assert_eq!(metric(220e-6, "F", 3), "220 μF\ten\n");
    assert_eq!(metric(1e40, "", 3), "1.00 x 10⁴⁰\ten\n");
    let size = |value, style: &str, decimals| {
        read_lines(|buffer, capacity| unsafe {
            hc_naturalsize(
                value,
                style.as_ptr(),
                style.len(),
                decimals,
                buffer,
                capacity,
            )
        })
    };
    assert_eq!(size(3_000_000.0, "decimal", 1), "3.0 MB\ten\n");
    assert_eq!(size(3000.0, "BINARY", 1), "2.9 KiB\ten\n");
    assert_eq!(size(3000.0, "gnu", 1), "2.9K\ten\n");
    assert_eq!(size(300.0, "gnu", 1), "300B\ten\n");
    let list = |items: &str| {
        read_lines(|buffer, capacity| unsafe {
            hc_naturallist(items.as_ptr(), items.len(), buffer, capacity)
        })
    };
    assert_eq!(list("one\ntwo\nthree"), "one, two and three\ten\n");
    assert_eq!(list("one\ntwo"), "one and two\ten\n");
    assert_eq!(list("one"), "one\ten\n");
    assert_eq!(list(""), "\ten\n");
    let intword = |digits: &str, decimals| {
        read_lines(|buffer, capacity| unsafe {
            hc_intword(digits.as_ptr(), digits.len(), decimals, buffer, capacity)
        })
    };
    assert_eq!(intword("12400", 1), "12.4 thousand\ten\n");
    assert_eq!(intword("1234000", 3), "1.234 million\ten\n");
    assert_eq!(
        intword("8100000000000000000000000000000000", 1),
        "8.1 decillion\ten\n"
    );
    let mut googol = String::from("1");
    googol.push_str(&"0".repeat(100));
    assert_eq!(intword(&googol, 1), "1.0 googol\ten\n");
}

#[test]
fn the_number_lines_refuse_what_python_refuses() {
    let none = |digits: &str| unsafe {
        hc_intword(digits.as_ptr(), digits.len(), 1, core::ptr::null_mut(), 0)
    };
    assert_eq!(none("12x"), HC_ERR_MALFORMED);
    assert_eq!(none(&"9".repeat(400)), HC_ERR_OUT_OF_RANGE);
    let size =
        unsafe { hc_naturalsize(f64::NAN, "decimal".as_ptr(), 7, 1, core::ptr::null_mut(), 0) };
    assert_eq!(size, HC_ERR_OUT_OF_RANGE);
    let style = unsafe { hc_naturalsize(1.0, "wide".as_ptr(), 4, 1, core::ptr::null_mut(), 0) };
    assert_eq!(style, HC_ERR_UNKNOWN);
    let precision = unsafe { hc_scientific(1.0, 256, core::ptr::null_mut(), 0) };
    assert_eq!(precision, HC_ERR_OUT_OF_RANGE);
    let metric = unsafe { hc_metric(1e40, core::ptr::null(), 0, 0, core::ptr::null_mut(), 0) };
    assert_eq!(metric, HC_ERR_OUT_OF_RANGE);
}
