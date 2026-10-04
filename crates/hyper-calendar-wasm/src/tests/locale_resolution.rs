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
/// `one` and 1.0 `other`. Its `ordinals.xml`: the English 3rd is `few`.
#[test]
fn a_number_has_a_cardinal_and_an_ordinal_category() {
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
    assert_eq!(category("en", "3", "ordinal"), "few\ten\t3\t0\t0\t0\t0\n");
    assert_eq!(category("de", "3", "ordinal"), "other\tde\t3\t0\t0\t0\t0\n");
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

/// 元 for the first year of an era and the Han numerals after it; the
/// templates `ja` writes a Japanese date with; and the categories Arabic,
/// Russian and Japanese plural rules answer.
#[test]
fn the_era_year_the_templates_and_the_plural_categories_cross_the_boundary() {
    let era_year = |year: i64| {
        read_lines(|buffer, capacity| unsafe { hc_japanese_era_year(year, buffer, capacity) })
    };
    assert_eq!(era_year(1), "元\n");
    assert_eq!(era_year(2), "二\n");
    assert_eq!(era_year(31), "三十一\n");
    assert_eq!(
        unsafe { hc_japanese_era_year(0, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
    let format = |locale: &str, calendar: &str| {
        read_lines(|buffer, capacity| unsafe {
            hc_locale_format(
                locale.as_ptr(),
                locale.len(),
                calendar.as_ptr(),
                calendar.len(),
                buffer,
                capacity,
            )
        })
    };
    // CLDR 48 de.xml, calendar type gregorian: the four date lengths, the
    // medium time and the medium date-time.
    let text = format("de", "gregory");
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert_eq!(rows.len(), 18, "{text}");
    assert!(
        rows.iter()
            .all(|row| row.len() == 4 && row[3] == "gregorian"),
        "{text}"
    );
    assert_eq!(
        rows[..4],
        [
            ["date", "full", "EEEE, d. MMMM y", "gregorian"],
            ["date", "long", "d. MMMM y", "gregorian"],
            ["date", "medium", "dd.MM.y", "gregorian"],
            ["date", "short", "dd.MM.yy", "gregorian"],
        ]
    );
    assert_eq!(rows[6][..3], ["time", "medium", "HH:mm:ss"]);
    assert_eq!(rows[10][..3], ["date-time", "medium", "{1}, {0}"]);
    assert_eq!(rows[12][..2], ["available", "hms"]);
    let hebrew = format("en", "hebrew");
    assert!(
        hebrew.lines().all(|line| line.ends_with("\thebrew")),
        "{hebrew}"
    );
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_locale_format("en".as_ptr(), 2, "x".as_ptr(), 1, null, 0) },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe { hc_locale_format("e!".as_ptr(), 2, "gregory".as_ptr(), 7, null, 0) },
        HC_ERR_MALFORMED
    );
    let categories = |locale: &str, kind: &str| {
        read_lines(|buffer, capacity| unsafe {
            hc_plural_categories(
                locale.as_ptr(),
                locale.len(),
                kind.as_ptr(),
                kind.len(),
                buffer,
                capacity,
            )
        })
    };
    assert_eq!(
        categories("ar", "cardinal"),
        "zero\t0\tar\none\t1\tar\ntwo\t2\tar\nfew\t3\tar\nmany\t11\tar\nother\t0.5\tar\n"
    );
    assert_eq!(
        categories("ru-RU", "Cardinal"),
        "one\t1\tru\nfew\t2\tru\nmany\t0\tru\nother\t0.0\tru\n"
    );
    assert_eq!(categories("ja", "cardinal"), "other\t0\tja\n");
    assert_eq!(categories("sa", "cardinal"), "other\t0\tund\n");
    // 1st, 2nd, 3rd, 4th.
    assert_eq!(
        categories("en", "ordinal"),
        "one\t1\ten\ntwo\t2\ten\nfew\t3\ten\nother\t0\ten\n"
    );
    assert_eq!(
        unsafe { hc_plural_categories("en".as_ptr(), 2, "x".as_ptr(), 1, null, 0) },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe { hc_plural_categories("e!".as_ptr(), 2, "cardinal".as_ptr(), 8, null, 0) },
        HC_ERR_MALFORMED
    );
}
