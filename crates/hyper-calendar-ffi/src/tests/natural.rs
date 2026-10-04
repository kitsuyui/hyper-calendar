use super::super::*;
use super::{measured, read_lines};

/// The examples of `humanize` 4.16's documentation of its number functions,
/// its `naturalsize` and its `natural_list`: the module's lines.
#[test]
fn the_number_lines_are_pythons_humanize() {
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_apnumber(5, core::ptr::null(), buffer, capacity, written)
    });
    assert_eq!(line, "five\ten\n");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_fractional(1.3, buffer, capacity, written)
    });
    assert_eq!(line, "1 3/10\ten\n");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_scientific(0.3, 2, buffer, capacity, written)
    });
    assert_eq!(line, "3.00 x 10⁻¹\ten\n");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_metric(
            1500.0,
            c"V".as_ptr(),
            3,
            core::ptr::null(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, "1.50 kV\ten\n");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_naturalsize(
            3000.0,
            c"binary".as_ptr(),
            1,
            core::ptr::null(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, "2.9 KiB\ten\n");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_naturallist(
            c"one\ntwo\nthree".as_ptr(),
            core::ptr::null(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, "one, two and three\ten\n");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_intword(
            c"1234000".as_ptr(),
            3,
            core::ptr::null(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, "1.234 million\ten\n");
}

/// The catalogue of `humanize` 4.16.0 for the locale, `de_DE.po`: *fünf*,
/// *Millionen*; the module's lines, with the language cell of the
/// catalogue that wrote them, and English where the German catalogue has no
/// word of the function (`naturalsize`), or in none (`natural_list`).
#[test]
fn the_number_lines_follow_the_locales_catalogue() {
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_apnumber(5, c"de-AT".as_ptr(), buffer, capacity, written)
    });
    assert_eq!(line, "fünf\tde-DE\n");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_intword(
            c"2000000".as_ptr(),
            1,
            c"de".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, "2,0 Millionen\tde-DE\n");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_naturallist(
            c"a\nb\nc".as_ptr(),
            c"de".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, "a, b and c\ten\n");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_naturalsize(
            3_000_000.0,
            c"decimal".as_ptr(),
            1,
            c"de".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, "3.0 MB\ten\n");
}

#[test]
fn the_number_lines_refuse_what_python_refuses() {
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_intword(
                c"12x".as_ptr(),
                1,
                core::ptr::null(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_MALFORMED
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_naturalsize(
                f64::NAN,
                c"decimal".as_ptr(),
                1,
                core::ptr::null(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_naturalsize(
                1.0,
                c"wide".as_ptr(),
                1,
                core::ptr::null(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_naturalsize(
                1.0,
                core::ptr::null(),
                1,
                core::ptr::null(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_NULL_POINTER
    );
}

/// The examples of `humanize` 4.16's documentation of `precisedelta`, and
/// the German catalogue's `vor {0}`, `morgen` and separators.
#[test]
fn the_time_and_day_lines_are_pythons_humanize() {
    let seconds = 2 * 86_400 + 3_633;
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_precisedelta(
            seconds,
            123_000,
            c"seconds".as_ptr(),
            core::ptr::null(),
            2,
            c"en".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, "2 days, 1 hour and 33.12 seconds\ten\n");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_naturaldelta(
            7 * 86_400,
            0,
            1,
            c"seconds".as_ptr(),
            core::ptr::null(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, "7 days\ten\n");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_naturaltime(
            3,
            0,
            1,
            c"seconds".as_ptr(),
            c"de".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, "vor 3 Sekunden\tde-DE\n");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_naturalday(
            739_889,
            739_888,
            core::ptr::null(),
            c"de".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, "morgen\tde-DE\n");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_naturaldate(
            739_888 + 153,
            739_888,
            c"en".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, "Mar 01 2027\ten\n");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_ordinal(
            103,
            c"male".as_ptr(),
            core::ptr::null(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, "103rd\ten\n");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_intcomma(
            c"-1234567".as_ptr(),
            c"de".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, "-1.234.567\tde-DE\n");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_intcomma_float(12_345.678_9, 2, c"en".as_ptr(), buffer, capacity, written)
    });
    assert_eq!(line, "12,345.68\ten\n");
}

#[test]
fn the_time_lines_refuse_what_python_refuses() {
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_naturaldelta(
                1,
                0,
                1,
                c"hours".as_ptr(),
                core::ptr::null(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_precisedelta(
                1,
                0,
                c"wide".as_ptr(),
                core::ptr::null(),
                2,
                core::ptr::null(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_ordinal(
                1,
                core::ptr::null(),
                core::ptr::null(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_intcomma(
                c"12x".as_ptr(),
                core::ptr::null(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_MALFORMED
    );
}

/// `humanize`'s `clamp` examples: `clamp(0.0001, floor=0.01)` is `<0.01`
/// and `clamp(0.999, format="{:.0%}", ceil=0.99)` is `>99%`.
#[test]
fn clamp_holds_a_value_within_its_bounds() {
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_clamp(
            0.0001,
            c"display".as_ptr(),
            c"0.01".as_ptr(),
            core::ptr::null(),
            c"<".as_ptr(),
            c">".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, "<0.01\ten\n");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_clamp(
            0.999,
            c"percent:0".as_ptr(),
            core::ptr::null(),
            c"0.99".as_ptr(),
            core::ptr::null(),
            c">".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, ">99%\ten\n");
    let refused = |format: &core::ffi::CStr| {
        measured(|buffer, capacity, written| unsafe {
            hc_clamp(
                1.0,
                format.as_ptr(),
                core::ptr::null(),
                core::ptr::null(),
                core::ptr::null(),
                core::ptr::null(),
                buffer,
                capacity,
                written,
            )
        })
    };
    assert_eq!(refused(c"scientific"), HC_ERROR_UNKNOWN);
    assert_eq!(refused(c"fixed:x"), HC_ERROR_MALFORMED);
    assert_eq!(refused(c"fixed:300"), HC_ERROR_OUT_OF_RANGE);
}
