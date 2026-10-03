//! How a locale resolves, called as a page calls it: the fallback chain,
//! what a locale is, a plural category, the names, the casing and the
//! isolates.

use super::super::*;
use super::read_lines;

/// The chain of `en-AU` as `docs/systems/locale-fallback.md` works it from
/// UTS #35 and CLDR 48's `parentLocales`: `en-001`, `en`, `und`.
#[test]
fn the_chain_and_the_description_of_a_locale() {
    let (locale, locale_len) = ("en-AU".as_ptr(), 5);
    let chain = read_lines(|buffer, capacity| unsafe {
        hc_locale_chain(locale, locale_len, buffer, capacity)
    });
    assert_eq!(
        chain,
        "0\ten-AU\trequested\t0\n1\ten-001\tparent-locales\t1\n2\ten\tregion\t1\n3\tund\troot\t0\n"
    );
    let info = read_lines(|buffer, capacity| unsafe {
        hc_locale_info(locale, locale_len, buffer, capacity)
    });
    let cells: Vec<&str> = info.trim_end().split('\t').collect();
    assert_eq!(cells.len(), 19);
    assert_eq!(cells[0], "en-AU");
    assert_eq!(cells[10], "en-001");
    assert_eq!(cells[11], "parent-locales");
    let bad = unsafe { hc_locale_chain("not a tag".as_ptr(), 9, core::ptr::null_mut(), 0) };
    assert_eq!(bad, HC_ERR_MALFORMED);
}

/// CLDR 48's `plurals.xml`: Russian 2 is `few` and 5 `many`; English 1 is
/// `one` and 1.0 `other`.
#[test]
fn a_number_has_a_category_and_ordinals_are_not_carried() {
    let category = |locale: &str, number: &str, kind: &str| {
        read_lines(|buffer, capacity| unsafe {
            hc_plural_category(
                locale.as_ptr(),
                locale.len(),
                number.as_ptr(),
                number.len(),
                kind.as_ptr(),
                kind.len(),
                buffer,
                capacity,
            )
        })
    };
    assert_eq!(category("ru", "2", "cardinal"), "few\tru\t2\t0\t0\t0\t0\n");
    assert_eq!(category("ru", "5", "Cardinal"), "many\tru\t5\t0\t0\t0\t0\n");
    assert_eq!(
        category("en", "1.0", "cardinal"),
        "other\ten\t1\t1\t0\t0\t0\n"
    );
    let ordinal = unsafe {
        hc_plural_category(
            "en".as_ptr(),
            2,
            "1".as_ptr(),
            1,
            "ordinal".as_ptr(),
            7,
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(ordinal, HC_ERR_NO_DATA);
    let unknown = unsafe {
        hc_plural_category(
            "en".as_ptr(),
            2,
            "1".as_ptr(),
            1,
            "".as_ptr(),
            0,
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(unknown, HC_ERR_UNKNOWN);
}

/// German Monday is *Montag*, Russian September *сентября* inside a date
/// and *сентябрь* alone; Turkish `iyi` in capitals is `İYİ`; an Arabic locale
/// embeds a Latin date in an LTR isolate.
#[test]
fn the_names_the_case_and_the_isolates_of_a_locale() {
    let names = |locale: &str, context: &str| {
        read_lines(|buffer, capacity| unsafe {
            hc_names(
                locale.as_ptr(),
                locale.len(),
                "gregory".as_ptr(),
                7,
                "wide".as_ptr(),
                4,
                context.as_ptr(),
                context.len(),
                buffer,
                capacity,
            )
        })
    };
    assert!(names("de", "format").contains("weekday\t1\tMontag\t"));
    assert!(names("ru", "format").contains("month\t9\tсентября\t"));
    assert!(names("ru", "standalone").contains("month\t9\tсентябрь\t"));
    let case = read_lines(|buffer, capacity| unsafe {
        hc_case(
            "tr".as_ptr(),
            2,
            "upper".as_ptr(),
            5,
            "iyi".as_ptr(),
            3,
            buffer,
            capacity,
        )
    });
    assert!(case.starts_with("İYİ\tupper\tturkic\t"), "{case}");
    let isolated = read_lines(|buffer, capacity| unsafe {
        hc_isolate(
            "ar".as_ptr(),
            2,
            "field".as_ptr(),
            5,
            "Sep 21".as_ptr(),
            6,
            buffer,
            capacity,
        )
    });
    assert_eq!(isolated, "\u{2066}Sep 21\u{2069}\trtl\tltr\t1\tfield\n");
    let unknown = unsafe {
        hc_names(
            "de".as_ptr(),
            2,
            "gregory".as_ptr(),
            7,
            "huge".as_ptr(),
            4,
            "format".as_ptr(),
            6,
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(unknown, HC_ERR_UNKNOWN);
}
