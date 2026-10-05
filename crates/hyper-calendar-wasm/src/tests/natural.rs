use super::super::*;
use super::read_lines;

/// The examples of `humanize` 4.16's documentation of its number functions
/// (`apnumber`, `fractional`, `scientific`, `metric`, `intword`), of
/// `naturalsize` and of `natural_list`, with no locale; the language cell
/// is `en`.
#[test]
fn the_number_lines_are_pythons_humanize() {
    let apnumber = |value, locale: &str| {
        read_lines(|buffer, capacity| unsafe {
            hc_apnumber(value, locale.as_ptr(), locale.len(), buffer, capacity)
        })
    };
    assert_eq!(apnumber(5, ""), "five\ten\n");
    assert_eq!(apnumber(10, ""), "10\ten\n");
    assert_eq!(apnumber(-1, ""), "-1\ten\n");
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
                core::ptr::null(),
                0,
                buffer,
                capacity,
            )
        })
    };
    assert_eq!(metric(1500.0, "V", 3), "1.50 kV\ten\n");
    assert_eq!(metric(220e-6, "F", 3), "220 μF\ten\n");
    assert_eq!(metric(1e40, "", 3), "1.00 x 10⁴⁰\ten\n");
    let size = |value, style: &str, decimals, locale: &str| {
        read_lines(|buffer, capacity| unsafe {
            hc_naturalsize(
                value,
                style.as_ptr(),
                style.len(),
                decimals,
                locale.as_ptr(),
                locale.len(),
                buffer,
                capacity,
            )
        })
    };
    assert_eq!(size(3_000_000.0, "decimal", 1, ""), "3.0 MB\ten\n");
    assert_eq!(size(3000.0, "BINARY", 1, ""), "2.9 KiB\ten\n");
    assert_eq!(size(3000.0, "gnu", 1, ""), "2.9K\ten\n");
    assert_eq!(size(300.0, "gnu", 1, ""), "300B\ten\n");
    let list = |items: &str, locale: &str| {
        read_lines(|buffer, capacity| unsafe {
            hc_naturallist(
                items.as_ptr(),
                items.len(),
                locale.as_ptr(),
                locale.len(),
                buffer,
                capacity,
            )
        })
    };
    assert_eq!(list("one\ntwo\nthree", ""), "one, two and three\ten\n");
    assert_eq!(list("one\ntwo", ""), "one and two\ten\n");
    assert_eq!(list("one", ""), "one\ten\n");
    assert_eq!(list("", ""), "\ten\n");
    let intword = |digits: &str, decimals, locale: &str| {
        read_lines(|buffer, capacity| unsafe {
            hc_intword(
                digits.as_ptr(),
                digits.len(),
                decimals,
                locale.as_ptr(),
                locale.len(),
                buffer,
                capacity,
            )
        })
    };
    assert_eq!(intword("12400", 1, ""), "12.4 thousand\ten\n");
    assert_eq!(intword("1234000", 3, ""), "1.234 million\ten\n");
    assert_eq!(
        intword("8100000000000000000000000000000000", 1, ""),
        "8.1 decillion\ten\n"
    );
    let mut googol = String::from("1");
    googol.push_str(&"0".repeat(100));
    assert_eq!(intword(&googol, 1, ""), "1.0 googol\ten\n");
}

/// The words come from the catalogues of `humanize` 4.16.0 (`de_DE.po`,
/// `ru_RU.po`, `pt_BR.po`, `pt_PT.po`), which the crate's tests hold to
/// GNU gettext. The language cell is the catalogue's, and a catalogue that
/// does not translate a function's words is passed over for the next step
/// of the locale's chain, then English, so no line mixes two languages: the
/// German catalogue has *Millionen* and *fünf* but no word of
/// `naturalsize`, and `natural_list` is in no catalogue.
#[test]
fn the_number_lines_follow_the_locales_catalogue() {
    let apnumber = |value, locale: &str| {
        read_lines(|buffer, capacity| unsafe {
            hc_apnumber(value, locale.as_ptr(), locale.len(), buffer, capacity)
        })
    };
    assert_eq!(apnumber(5, "de"), "fünf\tde-DE\n");
    assert_eq!(apnumber(5, "de-AT"), "fünf\tde-DE\n");
    assert_eq!(apnumber(5, "pt-AO"), apnumber(5, "pt-PT"));
    assert_eq!(apnumber(5, "pt"), "five\ten\n");
    assert_eq!(apnumber(5, "tlh-x-nothing"), "five\ten\n");
    assert_eq!(apnumber(5, "en-GB"), "five\ten\n");
    let intword = |digits: &str, decimals, locale: &str| {
        read_lines(|buffer, capacity| unsafe {
            hc_intword(
                digits.as_ptr(),
                digits.len(),
                decimals,
                locale.as_ptr(),
                locale.len(),
                buffer,
                capacity,
            )
        })
    };
    assert_eq!(intword("2000000", 1, "de"), "2,0 Millionen\tde-DE\n");
    let size = |locale: &str| {
        read_lines(|buffer, capacity| unsafe {
            hc_naturalsize(
                3_000_000.0,
                "decimal".as_ptr(),
                7,
                1,
                locale.as_ptr(),
                locale.len(),
                buffer,
                capacity,
            )
        })
    };
    assert_eq!(size("de"), "3.0 MB\ten\n");
    let list = |locale: &str| {
        read_lines(|buffer, capacity| unsafe {
            hc_naturallist(
                "a\nb\nc".as_ptr(),
                5,
                locale.as_ptr(),
                locale.len(),
                buffer,
                capacity,
            )
        })
    };
    assert_eq!(list("de"), "a, b and c\ten\n");
}

#[test]
fn the_number_lines_refuse_what_python_refuses() {
    let none = |digits: &str| unsafe {
        hc_intword(
            digits.as_ptr(),
            digits.len(),
            1,
            core::ptr::null(),
            0,
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(none("12x"), HC_ERR_MALFORMED);
    assert_eq!(none(&"9".repeat(400)), HC_ERR_OUT_OF_RANGE);
    let size = unsafe {
        hc_naturalsize(
            f64::NAN,
            "decimal".as_ptr(),
            7,
            1,
            core::ptr::null(),
            0,
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(size, HC_ERR_OUT_OF_RANGE);
    let style = unsafe {
        hc_naturalsize(
            1.0,
            "wide".as_ptr(),
            4,
            1,
            core::ptr::null(),
            0,
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(style, HC_ERR_UNKNOWN);
    let precision = unsafe { hc_scientific(1.0, 256, core::ptr::null_mut(), 0) };
    assert_eq!(precision, HC_ERR_OUT_OF_RANGE);
    let metric = unsafe {
        hc_metric(
            1e40,
            core::ptr::null(),
            0,
            0,
            core::ptr::null(),
            0,
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(metric, HC_ERR_OUT_OF_RANGE);
}

/// The examples of `humanize` 4.16's documentation of `precisedelta` (two
/// days, 3 633 seconds and 123 000 microseconds), `naturaltime`,
/// `naturalday` and `ordinal`, and CLDR-independent catalogue words from
/// `de_DE.po`.
#[test]
fn the_time_and_day_lines_are_pythons_humanize() {
    let (e, el) = s!("");
    let (en, enl) = s!("en");
    let (sec, secl) = s!("seconds");
    let (us, usl) = s!("microseconds");
    let (days, daysl) = s!("days");
    let seconds = 2 * 86_400 + 3_633;
    let precise = read_lines(|buffer, capacity| unsafe {
        hc_precisedelta(
            seconds, 123_000, sec, secl, e, el, 2, en, enl, buffer, capacity,
        )
    });
    assert_eq!(precise, "2 days, 1 hour and 33.12 seconds\ten\n");
    let precise = read_lines(|buffer, capacity| unsafe {
        hc_precisedelta(
            seconds, 123_000, us, usl, e, el, 2, en, enl, buffer, capacity,
        )
    });
    assert_eq!(
        precise,
        "2 days, 1 hour, 33 seconds and 123 milliseconds\ten\n"
    );
    let precise = read_lines(|buffer, capacity| unsafe {
        hc_precisedelta(
            seconds, 123_000, sec, secl, days, daysl, 4, en, enl, buffer, capacity,
        )
    });
    assert_eq!(precise, "49 hours and 33.1230 seconds\ten\n");
    let delta = read_lines(|buffer, capacity| unsafe {
        hc_naturaldelta(7 * 86_400, 0, 1, sec, secl, en, enl, buffer, capacity)
    });
    assert_eq!(delta, "7 days\ten\n");
    let time = read_lines(|buffer, capacity| unsafe {
        hc_naturaltime(-3, 0, 1, sec, secl, en, enl, buffer, capacity)
    });
    assert_eq!(time, "3 seconds from now\ten\n");
    let (de, del) = s!("de");
    let time = read_lines(|buffer, capacity| unsafe {
        hc_naturaltime(3, 0, 1, sec, secl, de, del, buffer, capacity)
    });
    assert_eq!(time, "vor 3 Sekunden\tde-DE\n");
    let (pattern, patternl) = s!("%Y-%m-%d");
    let day = read_lines(|buffer, capacity| unsafe {
        hc_naturalday(
            739_898, 739_888, pattern, patternl, en, enl, buffer, capacity,
        )
    });
    assert_eq!(day, "2026-10-09\ten\n");
    let day = read_lines(|buffer, capacity| unsafe {
        hc_naturalday(739_889, 739_888, e, el, de, del, buffer, capacity)
    });
    assert_eq!(day, "morgen\tde-DE\n");
    // A day written by `strftime` is English whatever the catalogue, and says so.
    let day = read_lines(|buffer, capacity| unsafe {
        hc_naturalday(739_898, 739_888, e, el, de, del, buffer, capacity)
    });
    assert_eq!(day, "Oct 09\ten\n");
    let date = read_lines(|buffer, capacity| unsafe {
        hc_naturaldate(739_888 + 153, 739_888, en, enl, buffer, capacity)
    });
    assert_eq!(date, "Mar 01 2027\ten\n");
    let (male, malel) = s!("male");
    let ordinal = read_lines(|buffer, capacity| unsafe {
        hc_ordinal(103, male, malel, en, enl, buffer, capacity)
    });
    assert_eq!(ordinal, "103rd\ten\n");
    let (digits, digitsl) = s!("-1234567");
    let comma = read_lines(|buffer, capacity| unsafe {
        hc_intcomma(digits, digitsl, de, del, buffer, capacity)
    });
    assert_eq!(comma, "-1.234.567\tde-DE\n");
    let float = read_lines(|buffer, capacity| unsafe {
        hc_intcomma_float(12_345.678_9, 2, en, enl, buffer, capacity)
    });
    assert_eq!(float, "12,345.68\ten\n");
}

#[test]
fn the_time_lines_refuse_what_python_refuses() {
    let (e, el) = s!("");
    let (hours, hoursl) = s!("hours");
    let (sec, secl) = s!("seconds");
    let (wide, widel) = s!("wide");
    let none = core::ptr::null_mut();
    let refuse = |minimum: (*const u8, usize), micros: i32| unsafe {
        hc_naturaldelta(1, micros, 1, minimum.0, minimum.1, e, el, none, 0)
    };
    assert_eq!(refuse((hours, hoursl), 0), HC_ERR_OUT_OF_RANGE);
    assert_eq!(refuse((wide, widel), 0), HC_ERR_UNKNOWN);
    assert_eq!(refuse((sec, secl), 1_000_000), HC_ERR_OUT_OF_RANGE);
    let precise = unsafe { hc_precisedelta(1, 0, wide, widel, e, el, 2, e, el, none, 0) };
    assert_eq!(precise, HC_ERR_UNKNOWN);
    let (junk, junkl) = s!("12x");
    assert_eq!(
        unsafe { hc_intcomma(junk, junkl, e, el, none, 0) },
        HC_ERR_MALFORMED
    );
    assert_eq!(
        unsafe { hc_ordinal(1, wide, widel, e, el, none, 0) },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe { hc_naturalday(i64::MAX, 0, e, el, e, el, none, 0) },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_intcomma_float(1.0, 256, e, el, none, 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

/// `humanize`'s `clamp` examples: `clamp(0.0001, floor=0.01)` is `<0.01`,
/// `clamp(0.999, format="{:.0%}", ceil=0.99)` is `>99%`, and a value
/// within the bounds is written as `str` writes a float.
#[test]
fn clamp_holds_a_value_within_its_bounds() {
    let clamp = |value: f64, format: &str, floor: &str, ceil: &str, tokens: (&str, &str)| {
        read_lines(|buffer, capacity| unsafe {
            hc_clamp(
                value,
                format.as_ptr(),
                format.len(),
                floor.as_ptr(),
                floor.len(),
                ceil.as_ptr(),
                ceil.len(),
                tokens.0.as_ptr(),
                tokens.0.len(),
                tokens.1.as_ptr(),
                tokens.1.len(),
                buffer,
                capacity,
            )
        })
    };
    assert_eq!(
        clamp(123.456, "display", "", "", ("<", ">")),
        "123.456\ten\n"
    );
    assert_eq!(
        clamp(0.0001, "display", "0.01", "", ("<", ">")),
        "<0.01\ten\n"
    );
    assert_eq!(
        clamp(0.999, "PERCENT:0", "", "0.99", ("<", ">")),
        ">99%\ten\n"
    );
    assert_eq!(clamp(0.5, "fixed:2", "", "", ("<", ">")), "0.50\ten\n");
    assert_eq!(
        clamp(1e9, "display", "", "1e6", ("", "over ")),
        "over 1000000.0\ten\n"
    );
    assert_eq!(clamp(f64::NAN, "display", "", "", ("", "")), "NaN\ten\n");
    let null = core::ptr::null_mut();
    let refused = |format: &str, floor: &str| unsafe {
        hc_clamp(
            1.0,
            format.as_ptr(),
            format.len(),
            floor.as_ptr(),
            floor.len(),
            core::ptr::null(),
            0,
            core::ptr::null(),
            0,
            core::ptr::null(),
            0,
            null,
            0,
        )
    };
    assert_eq!(refused("scientific", ""), HC_ERR_UNKNOWN);
    assert_eq!(refused("fixed:x", ""), HC_ERR_MALFORMED);
    assert_eq!(refused("fixed:300", ""), HC_ERR_OUT_OF_RANGE);
    assert_eq!(refused("display", "low"), HC_ERR_MALFORMED);
}
