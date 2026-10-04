//! How a locale resolves, through the C boundary.

use super::super::*;
use super::{measured, read_lines};

/// The chain of `en-AU` (`docs/systems/locale-fallback.md`, from UTS #35
/// and CLDR 48's `parentLocales`), and CLDR 48's rules: Russian 2 is `few`
/// (`plurals.xml`), and the 2nd is `two` in English (`ordinals.xml`).
#[test]
fn the_chain_the_description_and_the_category_of_a_locale() {
    let chain = read_lines(|buffer, capacity, written| unsafe {
        hc_locale_chain(c"en-AU".as_ptr(), buffer, capacity, written)
    });
    assert_eq!(
        chain,
        "0\ten-AU\trequested\t0\n1\ten-001\tparent-locales\t1\n2\ten\tregion\t1\n3\tund\troot\t0\n"
    );
    let info = read_lines(|buffer, capacity, written| unsafe {
        hc_locale_info(c"de-DE".as_ptr(), buffer, capacity, written)
    });
    assert_eq!(info.trim_end().split('\t').count(), 19);
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_plural_category(
            c"ru".as_ptr(),
            c"2".as_ptr(),
            c"cardinal".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, "few\tru\t2\t0\t0\t0\t0\n");
    let ordinal = read_lines(|buffer, capacity, written| unsafe {
        hc_plural_category(
            c"en".as_ptr(),
            c"2".as_ptr(),
            c"ordinal".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(ordinal, "two\ten\t2\t0\t0\t0\t0\n");
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_plural_category(
                c"ru".as_ptr(),
                c"2".as_ptr(),
                c"fraction".as_ptr(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_locale_chain(c"not a tag".as_ptr(), buffer, capacity, written)
        }),
        HC_ERROR_MALFORMED
    );
}

/// German Monday is *Montag*; Turkish `iyi` in capitals is `İYİ`; an Arabic
/// locale embeds a Latin date in an LTR isolate.
#[test]
fn the_names_the_case_and_the_isolates_of_a_locale() {
    let names = read_lines(|buffer, capacity, written| unsafe {
        hc_names(
            c"de".as_ptr(),
            c"gregory".as_ptr(),
            c"wide".as_ptr(),
            c"format".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert!(names.contains("weekday\t1\tMontag\t"));
    let case = read_lines(|buffer, capacity, written| unsafe {
        hc_case(
            c"tr".as_ptr(),
            c"upper".as_ptr(),
            c"iyi".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert!(case.starts_with("İYİ\tupper\tturkic\t"), "{case}");
    let isolated = read_lines(|buffer, capacity, written| unsafe {
        hc_isolate(
            c"ar".as_ptr(),
            c"field".as_ptr(),
            c"Sep 21".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(isolated, "\u{2066}Sep 21\u{2069}\trtl\tltr\t1\tfield\n");
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_isolate(
                c"ar".as_ptr(),
                c"wrap".as_ptr(),
                c"x".as_ptr(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_UNKNOWN
    );
}

/// The era year, the templates and the plural categories are the module's
/// lines, with the C library's own refusals for a null name.
#[test]
fn the_era_year_the_templates_and_the_plural_categories_cross_the_c_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_japanese_era_year(1, buffer, capacity, written)
    });
    assert_eq!(text, "元\n");
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_japanese_era_year(0, buffer, capacity, written)
        }),
        HC_ERROR_OUT_OF_RANGE
    );
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_locale_format(
            c"de".as_ptr(),
            c"gregory".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(
        Ok(text.clone()),
        hc::i18n_lines::locale_format_lines("de", "gregory")
    );
    assert_eq!(text.lines().count(), 18);
    assert!(
        text.starts_with("date\tfull\tEEEE, d. MMMM y\tgregorian\n"),
        "{text}"
    );
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_locale_format(
            core::ptr::null(),
            c"gregory".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(Ok(text), hc::i18n_lines::locale_format_lines("", "gregory"));
    for (locale, calendar, status) in [
        (c"en".as_ptr(), c"x".as_ptr(), HC_ERROR_UNKNOWN),
        (c"e!".as_ptr(), c"gregory".as_ptr(), HC_ERROR_MALFORMED),
        (c"en".as_ptr(), core::ptr::null(), HC_ERROR_NULL_POINTER),
    ] {
        assert_eq!(
            measured(|buffer, capacity, written| unsafe {
                hc_locale_format(locale, calendar, buffer, capacity, written)
            }),
            status
        );
    }
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_plural_categories(
            c"ru".as_ptr(),
            c"cardinal".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(
        text,
        "one\t1\tru\nfew\t2\tru\nmany\t0\tru\nother\t0.0\tru\n"
    );
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_plural_categories(
            c"en".as_ptr(),
            c"ordinal".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(text, "one\t1\ten\ntwo\t2\ten\nfew\t3\ten\nother\t0\ten\n");
    for (kind, status) in [
        (c"x".as_ptr(), HC_ERROR_UNKNOWN),
        (core::ptr::null(), HC_ERROR_NULL_POINTER),
    ] {
        assert_eq!(
            measured(|buffer, capacity, written| unsafe {
                hc_plural_categories(c"ru".as_ptr(), kind, buffer, capacity, written)
            }),
            status
        );
    }
}
