//! How a locale resolves, through the C boundary.

use super::super::*;
use super::{measured, read_lines};

/// The chain of `en-AU` (`docs/systems/locale-fallback.md`, from UTS #35
/// and CLDR 48's `parentLocales`), and CLDR 48's cardinal rules: Russian 2
/// is `few`.
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
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_plural_category(
                c"ru".as_ptr(),
                c"2".as_ptr(),
                c"ordinal".as_ptr(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_NO_DATA
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
