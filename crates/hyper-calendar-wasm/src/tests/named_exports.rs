//! One call to each export no other boundary test names, with the line it
//! writes and the refusal it gives for bad input: the values are the ones
//! the JavaScript binding's tests pin, from the sources `docs/systems` name.

use super::super::*;
use super::read_lines;

/// The cells of one line, tab-separated, the terminator dropped.
#[cfg(any(feature = "calendars", feature = "zone-names", feature = "holiday"))]
fn cells(text: &str) -> Vec<&str> {
    text.trim_end_matches('\n').split('\t').collect()
}

/// Five decimal hours of the French Republican day are noon.
#[cfg(feature = "timestamps")]
#[test]
fn the_civil_time_of_a_french_decimal_time_is_a_line() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_civil_from_french_decimal_time(5, 0, 0, 0, buffer, capacity)
    });
    assert_eq!(text, "43200\t0\n");
    // Ten hours is the next day, and no time of this one.
    assert_eq!(
        unsafe { hc_civil_from_french_decimal_time(10, 0, 0, 0, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

/// 26 September 2026 carries the Tamil samvatsara Parābhava, which the
/// date's formatter writes.
#[cfg(feature = "calendars")]
#[test]
fn the_extra_fields_of_a_day_are_lines_and_an_unknown_calendar_is_refused() {
    let day = hc_gregorian_to_fixed(2026, 9, 27);
    let all = read_lines(|buffer, capacity| unsafe {
        hc_day_extras(
            day,
            core::ptr::null(),
            0,
            "en".as_ptr(),
            2,
            buffer,
            capacity,
        )
    });
    assert!(all.lines().all(|line| cells(line).len() == 7), "{all}");
    let id = "hindu-solar-tamil";
    let tamil = read_lines(|buffer, capacity| unsafe {
        hc_day_extras(
            day,
            id.as_ptr(),
            id.len(),
            "en".as_ptr(),
            2,
            buffer,
            capacity,
        )
    });
    let rows: Vec<Vec<&str>> = tamil.lines().map(cells).collect();
    assert_eq!(rows.len(), 2, "{tamil}");
    assert_eq!(rows[0][..3], ["hindu-solar-tamil", "samvatsara", "40"]);
    assert_eq!(rows[0][4], "Parabhava");
    assert_eq!(
        rows[1][..3],
        ["hindu-solar-tamil", "tiruvalluvar-year", "2057"]
    );
    let unknown = "no-such-calendar";
    assert_eq!(
        unsafe {
            hc_day_extras(
                day,
                unknown.as_ptr(),
                unknown.len(),
                core::ptr::null(),
                0,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
}

#[cfg(feature = "calendars")]
#[test]
fn the_numbering_systems_and_the_eras_are_lines() {
    let systems = read_lines(|buffer, capacity| unsafe { hc_numbering_systems(buffer, capacity) });
    let latn = systems
        .lines()
        .map(cells)
        .find(|row| row[0] == "latn")
        .expect("latn");
    assert_eq!(latn, ["latn", "0", "0123456789"]);
    let japanese = "japanese";
    let eras = read_lines(|buffer, capacity| unsafe {
        hc_calendar_eras(
            japanese.as_ptr(),
            japanese.len(),
            "ja".as_ptr(),
            2,
            buffer,
            capacity,
        )
    });
    let reiwa = eras
        .lines()
        .map(cells)
        .find(|row| row[0] == "reiwa")
        .expect("令和");
    assert_eq!(reiwa[1], "令和");
    assert_eq!(reiwa.len(), 6);
    let unknown = "no-such";
    assert_eq!(
        unsafe {
            hc_calendar_eras(
                unknown.as_ptr(),
                unknown.len(),
                core::ptr::null(),
                0,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
}

/// Year 1 of the Seleucid era is the first of Seleucus I Nicator.
#[cfg(feature = "calendars")]
#[test]
fn a_seleucid_year_is_a_kings_regnal_year() {
    let text =
        read_lines(|buffer, capacity| unsafe { hc_babylonian_regnal_year(1, buffer, capacity) });
    assert_eq!(text, "Seleucus I Nicator\t1\n");
}

/// 183 BE of the Badí‘ calendar is a fifth of a minute after the Tehran
/// sunset (the equinox falls just before the day that decides the year).
#[cfg(feature = "calendars")]
#[test]
fn the_equinox_margin_of_a_new_year_is_minutes_and_a_calendar() {
    let calendar = "bahai-astronomical";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_equinox_new_year_margin(calendar.as_ptr(), calendar.len(), 183, buffer, capacity)
    });
    let row = cells(&text);
    assert_eq!(row.len(), 2, "{text}");
    let minutes: f64 = row[0].parse().expect("minutes");
    assert!(minutes < 0.0 && minutes > -0.2, "{text}");
    assert_eq!(row[1], calendar);
    let gregorian = "gregory";
    assert_eq!(
        unsafe {
            hc_equinox_new_year_margin(
                gregorian.as_ptr(),
                gregorian.len(),
                2026,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
}

/// The Moon is in Revatī, the twenty-seventh, at New Delhi's sunrise on 7
/// January 2025, and enters Aśvinī that morning, which makes the day's
/// amṛta siddhi yoga (the JavaScript binding's tests).
#[cfg(feature = "calendars")]
#[test]
fn the_nakshatra_of_a_day_is_the_moons_at_sunrise() {
    let day = hc_gregorian_to_fixed(2025, 1, 7);
    let ayanamsa = "lahiri";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_nakshatra_of_day(
            day,
            28.6356,
            77.2244,
            0.0,
            ayanamsa.as_ptr(),
            ayanamsa.len(),
            buffer,
            capacity,
        )
    });
    let row = cells(&text);
    assert_eq!(row[..3], ["27", "revati", "Revatī"], "{text}");
    assert_eq!(row[6], "lahiri", "{text}");
    let tropical = "tropical";
    assert_eq!(
        unsafe {
            hc_nakshatra_of_day(
                day,
                28.6356,
                77.2244,
                0.0,
                tropical.as_ptr(),
                tropical.len(),
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
}

/// 日本 holds women as its first group, listed in English.
#[cfg(feature = "holiday")]
#[test]
fn the_groups_a_holiday_may_be_given_to_are_lines() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_holiday_groups("en".as_ptr(), 2, buffer, capacity)
    });
    let first = cells(text.lines().next().expect("a group"));
    assert_eq!(first.len(), 4);
    assert_eq!(first[0], "women");
    assert_eq!(first[3], "women");
}

/// Japan's civil service weekend is read from 1 May 1992, its days from the
/// Public Holidays Act of 1948, and Osaka's own days from 1989.
#[cfg(feature = "holiday")]
#[test]
fn the_years_a_holiday_table_answers_for_are_lines() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_holiday_coverage("JP".as_ptr(), 2, buffer, capacity)
    });
    let first = cells(text.lines().next().expect("a scope"));
    assert_eq!(
        first,
        [
            "",
            "1948",
            "1949",
            "",
            "1",
            "1992",
            "Holidays before the Public Holidays Act"
        ]
    );
    let osaka = text
        .lines()
        .map(cells)
        .find(|row| row[0] == "JP-27")
        .expect("Osaka");
    assert_eq!(osaka[2], "1989");
    assert_eq!(osaka[6], "JP-27");
    // A table read in no year has no first year to give.
    let bosnia = read_lines(|buffer, capacity| unsafe {
        hc_holiday_coverage("BA".as_ptr(), 2, buffer, capacity)
    });
    assert_eq!(cells(bosnia.lines().next().expect("a scope"))[4], "0");
    assert_eq!(
        unsafe { hc_holiday_coverage("ZZ".as_ptr(), 2, core::ptr::null_mut(), 0) },
        HC_ERR_UNKNOWN
    );
}

/// South Korea's Seollal moves for a Sunday alone from 2014, and the Korea
/// Exchange includes South Korea's days off from 2009 (column 16 of
/// `hc_holiday_tables`).
#[cfg(feature = "holiday")]
#[test]
fn the_rules_of_a_holiday_table_are_lines() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_holiday_rules("KR".as_ptr(), 2, buffer, capacity)
    });
    let rows: Vec<Vec<&str>> = text.lines().map(cells).collect();
    assert!(rows.iter().all(|row| row.len() == 12), "{text}");
    let seollal = rows
        .iter()
        .find(|row| row[0] == "seollal")
        .expect("Seollal");
    assert_eq!(seollal[1], "Seollal");
    assert_eq!(seollal[2], "설날");
    assert_eq!(seollal[3], "public");
    assert_eq!(seollal[10], "7//2014");
    assert_eq!(rows[0][10], "none");
    assert_eq!(
        unsafe { hc_holiday_rules("ZZ".as_ptr(), 2, core::ptr::null_mut(), 0) },
        HC_ERR_UNKNOWN
    );
    let tables = read_lines(|buffer, capacity| unsafe {
        hc_holiday_tables("en".as_ptr(), 2, buffer, capacity)
    });
    let exchange = tables
        .lines()
        .map(cells)
        .find(|row| row[0] == "XKRX")
        .expect("the Korea Exchange");
    assert_eq!(exchange[15], "KR//2009");
}

/// Nayrouz, 11 September 2025, is named by the Bohairic Coptic names.
#[cfg(feature = "holiday")]
#[test]
fn a_days_holidays_are_named_in_a_locale() {
    let day = hc_gregorian_to_fixed(2025, 9, 11);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_holidays_on_in(day, "cop".as_ptr(), 3, buffer, capacity)
    });
    let nayrouz = text
        .lines()
        .map(cells)
        .find(|row| row[0] == "coptic-orthodox")
        .expect("Nayrouz");
    assert_eq!(nayrouz.len(), 15, "{nayrouz:?}");
    assert_eq!(
        nayrouz[11..],
        ["ⲡⲓⲭⲗⲟⲙ ⲛ̀ⲧⲉ ϯⲣⲟⲙⲡⲓ", "cop", "nayrouz-new-year", "0"]
    );
    assert_eq!(
        unsafe { hc_holidays_on_in(1 << 62, "cop".as_ptr(), 3, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

/// Tuesday 28 April 2026 plus five business days is Monday 11 May, the
/// Golden Week holidays skipped; six lie between the Monday before and the
/// 11th.
#[cfg(feature = "holiday")]
#[test]
fn business_days_skip_the_holidays_and_refuse_what_they_cannot_count() {
    let jp = "JP";
    let tuesday = hc_gregorian_to_fixed(2026, 4, 28);
    let monday = hc_gregorian_to_fixed(2026, 5, 11);
    let add = |code: &str, region: &str, from: i64, count: i64| unsafe {
        hc_holiday_add_business_days(
            code.as_ptr(),
            code.len(),
            region.as_ptr(),
            region.len(),
            core::ptr::null(),
            0,
            from,
            count,
        )
    };
    assert_eq!(add(jp, "", tuesday, 5), monday);
    assert_eq!(add(jp, "", monday, -5), tuesday);
    assert_eq!(add("XX", "", tuesday, 1), HC_ERR_UNKNOWN);
    assert_eq!(add(jp, "", tuesday, 36_501), HC_ERR_OUT_OF_RANGE);
    let between = |from: i64, to: i64| unsafe {
        hc_holiday_business_days_between(
            jp.as_ptr(),
            jp.len(),
            core::ptr::null(),
            0,
            core::ptr::null(),
            0,
            from,
            to,
        )
    };
    assert_eq!(between(tuesday - 1, monday), 6);
    assert_eq!(between(monday, tuesday - 1), -6);
    assert_eq!(between(tuesday, tuesday), 0);
    // A day of 2151 may be one of China's lunisolar festivals, so a walk over
    // it is open, not counted on a guess; Canada's weekend of 2025 is read
    // from 2026.
    let monday = hc_gregorian_to_fixed(2151, 3, 3);
    assert_eq!(add("CN", "", monday, 1), HC_ERR_NO_DATA);
    let friday = hc_gregorian_to_fixed(2025, 5, 16);
    assert_eq!(add("CA", "CA-NL", friday, 1), HC_ERR_OUT_OF_RANGE);
    // Kedah's weekend law of 2012 was not read: the walk is out of range.
    let wednesday = hc_gregorian_to_fixed(2012, 5, 9);
    assert_eq!(add("MY", "MY-02", wednesday, 1), HC_ERR_OUT_OF_RANGE);
}

/// 1 January 2026 at 00:00 UTC in Tokyo.
#[cfg(feature = "zone-names")]
#[test]
fn an_instant_is_written_by_a_pattern_in_a_zone() {
    let write = |syntax: &str, pattern: &str, locale: &str| {
        read_lines(|buffer, capacity| unsafe {
            hc_format_pattern(
                "Asia/Tokyo".as_ptr(),
                10,
                1_767_225_600,
                locale.as_ptr(),
                locale.len(),
                syntax.as_ptr(),
                syntax.len(),
                pattern.as_ptr(),
                pattern.len(),
                buffer,
                capacity,
            )
        })
    };
    let cldr = write("cldr", "yyyy-MM-dd HH:mm zzzz", "en");
    assert_eq!(cells(&cldr)[0], "2026-01-01 09:00 Japan Standard Time");
    let posix = write("strftime", "%Y-%m-%d %H:%M %Z", "en");
    assert_eq!(cells(&posix)[0], "2026-01-01 09:00 JST");
    let unknown = unsafe {
        hc_format_pattern(
            "Asia/Tokyo".as_ptr(),
            10,
            0,
            "en".as_ptr(),
            2,
            "java".as_ptr(),
            4,
            "yyyy".as_ptr(),
            4,
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(unknown, HC_ERR_UNKNOWN);
    let unclosed = unsafe {
        hc_format_pattern(
            "Asia/Tokyo".as_ptr(),
            10,
            0,
            "en".as_ptr(),
            2,
            "cldr".as_ptr(),
            4,
            "'unclosed".as_ptr(),
            9,
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(unclosed, HC_ERR_MALFORMED);
}
