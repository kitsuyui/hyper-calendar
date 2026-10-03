//! One call to each entry point no other C-library test names (audit 10
//! b20), with the line it writes and the refusal it gives for bad input.
//! The values are the ones the WebAssembly module's and the JavaScript
//! binding's tests pin, from the sources `docs/systems` name.

use super::super::*;
use super::{measured, read_lines};

/// The cells of one line, tab-separated, the terminator dropped.
#[cfg(any(feature = "calendars", feature = "zone-names"))]
fn cells(text: &str) -> Vec<&str> {
    text.trim_end_matches('\n').split('\t').collect()
}

/// Five decimal hours of the French Republican day are noon, which is
/// 43,200 s of the civil one.
#[cfg(feature = "timestamps")]
#[test]
fn the_french_decimal_time_of_a_civil_time_is_a_line() {
    let noon = read_lines(|buffer, capacity, written| unsafe {
        hc_french_decimal_time(43_200, 0, buffer, capacity, written)
    });
    assert_eq!(noon, "5\t0\t0\t0\n");
    // A leap second has no place on the decimal clock.
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_french_decimal_time(86_400, 0, buffer, capacity, written)
        }),
        HC_ERROR_NO_DATA
    );
}

/// 27 September 2026 carries the Tamil samvatsara Parābhava.
#[cfg(feature = "calendars")]
#[test]
fn the_extra_fields_of_a_day_are_lines_and_an_unknown_calendar_is_refused() {
    let mut day = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2026, 9, 27, &mut day) },
        HC_OK
    );
    let all = read_lines(|buffer, capacity, written| unsafe {
        hc_day_extras(
            day,
            core::ptr::null(),
            c"en".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert!(all.lines().all(|line| cells(line).len() == 7), "{all}");
    let tamil = read_lines(|buffer, capacity, written| unsafe {
        hc_day_extras(
            day,
            c"hindu-solar-tamil".as_ptr(),
            c"en".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    let rows: Vec<Vec<&str>> = tamil.lines().map(cells).collect();
    assert_eq!(rows.len(), 2, "{tamil}");
    assert_eq!(rows[0][..3], ["hindu-solar-tamil", "samvatsara", "40"]);
    assert_eq!(
        rows[1][..3],
        ["hindu-solar-tamil", "tiruvalluvar-year", "2057"]
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_day_extras(
                day,
                c"no-such-calendar".as_ptr(),
                core::ptr::null(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_UNKNOWN
    );
}

/// 15:00 is in the afternoon of the second half of the day.
#[cfg(feature = "calendars")]
#[test]
fn a_time_of_the_civil_clock_has_day_periods_in_a_locale() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_day_period(15 * 3600, c"en".as_ptr(), buffer, capacity, written)
    });
    let row = cells(&text);
    assert_eq!(row.len(), 7, "{text}");
    assert_eq!(row[..3], ["pm", "PM", "afternoon1"]);
    assert_eq!(row[4], "in the afternoon");
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_day_period(86_400, c"en".as_ptr(), buffer, capacity, written)
        }),
        HC_ERROR_OUT_OF_RANGE
    );
}

/// 183 BE of the Badí‘ calendar is a fifth of a minute after the Tehran
/// sunset: the equinox falls just before the day that decides the year.
#[cfg(feature = "calendars")]
#[test]
fn the_equinox_margin_of_a_new_year_is_minutes_and_a_calendar() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_equinox_new_year_margin(
            c"bahai-astronomical".as_ptr(),
            183,
            buffer,
            capacity,
            written,
        )
    });
    let row = cells(&text);
    assert_eq!(row.len(), 2, "{text}");
    let minutes: f64 = row[0].parse().expect("minutes");
    assert!(minutes < 0.0 && minutes > -0.2, "{text}");
    assert_eq!(row[1], "bahai-astronomical");
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_equinox_new_year_margin(c"gregory".as_ptr(), 2026, buffer, capacity, written)
        }),
        HC_ERROR_UNKNOWN
    );
}

/// Shmuel's Tishrei *tekufah* of 5786 falls on 7 October 2025.
#[cfg(feature = "calendars")]
#[test]
fn a_tekufah_of_shmuel_is_a_line() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_shmuel_tekufah(5786, c"tishrei".as_ptr(), buffer, capacity, written)
    });
    let row = cells(&text);
    assert_eq!(row.len(), 5, "{text}");
    let mut tishrei = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2025, 10, 7, &mut tishrei) },
        HC_OK
    );
    assert_eq!(row[0], tishrei.to_string());
    assert_eq!(row[2], "tishrei");
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_shmuel_tekufah(5786, c"adar".as_ptr(), buffer, capacity, written)
        }),
        HC_ERROR_UNKNOWN
    );
}

/// The Qing almanac's twenty-four terms of 1700 run from 小寒 to 冬至.
#[cfg(feature = "calendars")]
#[test]
fn the_almanacs_solar_terms_run_from_xiaohan() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_chinese_almanac_solar_terms(1700, buffer, capacity, written)
    });
    let rows: Vec<Vec<&str>> = text.lines().map(cells).collect();
    assert_eq!(rows.len(), 24);
    assert_eq!(rows[0][..2], ["1", "小寒"]);
    assert_eq!(rows[23][1], "冬至");
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_chinese_almanac_solar_terms(1668, buffer, capacity, written)
        }),
        HC_ERROR_NO_DATA
    );
}

/// The Moon is in Revatī, the twenty-seventh nakṣatra, at New Delhi's
/// sunrise of 7 January 2025, 1,736,214,300 s after the epoch.
#[cfg(feature = "calendars")]
#[test]
fn the_nakshatra_at_an_instant_is_a_line() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_nakshatra_at(1_736_214_300, c"lahiri".as_ptr(), buffer, capacity, written)
    });
    let row = cells(&text);
    assert_eq!(row[..3], ["27", "revati", "Revatī"], "{text}");
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_nakshatra_at(0, c"tropical".as_ptr(), buffer, capacity, written)
        }),
        HC_ERROR_UNKNOWN
    );
}

/// The Bhutanese winter solstice of 2001 is read 2;51,38 and falls on
/// 1 January 2001.
#[cfg(feature = "calendars")]
#[test]
fn the_bhutanese_winter_solstice_is_a_reading_on_a_day() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_bhutanese_winter_solstice(2001, buffer, capacity, written)
    });
    let row = cells(&text);
    assert_eq!(row.len(), 3, "{text}");
    let mut day = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2001, 1, 1, &mut day) },
        HC_OK
    );
    assert_eq!(row[0], day.to_string());
    assert_eq!(row[1], "2;51,38");
}

/// 1 January 2026 at 00:00 UTC is 09:00 in Tokyo.
#[cfg(feature = "zone-names")]
#[test]
fn an_instant_is_written_by_a_pattern_in_a_zone() {
    let write = |syntax: &core::ffi::CStr, pattern: &core::ffi::CStr| {
        read_lines(|buffer, capacity, written| unsafe {
            hc_format_pattern(
                c"Asia/Tokyo".as_ptr(),
                1_767_225_600,
                c"en".as_ptr(),
                syntax.as_ptr(),
                pattern.as_ptr(),
                buffer,
                capacity,
                written,
            )
        })
    };
    assert_eq!(
        cells(&write(c"cldr", c"yyyy-MM-dd HH:mm zzzz"))[0],
        "2026-01-01 09:00 Japan Standard Time"
    );
    assert_eq!(
        cells(&write(c"strftime", c"%Y-%m-%d %H:%M %Z"))[0],
        "2026-01-01 09:00 JST"
    );
    let status = |syntax: &core::ffi::CStr, pattern: &core::ffi::CStr| {
        measured(|buffer, capacity, written| unsafe {
            hc_format_pattern(
                c"Asia/Tokyo".as_ptr(),
                0,
                c"en".as_ptr(),
                syntax.as_ptr(),
                pattern.as_ptr(),
                buffer,
                capacity,
                written,
            )
        })
    };
    assert_eq!(status(c"java", c"yyyy"), HC_ERROR_UNKNOWN);
    assert_eq!(status(c"cldr", c"'unclosed"), HC_ERROR_MALFORMED);
}
