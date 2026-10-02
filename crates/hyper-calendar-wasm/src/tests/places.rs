use super::super::*;
use super::read_lines;

fn subdivisions(country: &str, locale: &str) -> String {
    read_lines(|buffer, capacity| unsafe {
        hc_subdivisions(
            country.as_ptr(),
            country.len(),
            locale.as_ptr(),
            locale.len(),
            buffer,
            capacity,
        )
    })
}

fn place(code: &str, locale: &str) -> i64 {
    unsafe {
        hc_place_name(
            code.as_ptr(),
            code.len(),
            locale.as_ptr(),
            locale.len(),
            core::ptr::null_mut(),
            0,
        )
    }
}

fn line(code: &str, locale: &str) -> String {
    read_lines(|buffer, capacity| unsafe {
        hc_place_name(
            code.as_ptr(),
            code.len(),
            locale.as_ptr(),
            locale.len(),
            buffer,
            capacity,
        )
    })
}

/// CLDR 48 `subdivisions/ja.xml` `jp13` 東京都, provisional, and
/// `subdivisions/en.xml` `jp13` Tokyo.
#[test]
fn tokyo_is_tokyo_to_in_japanese() {
    let japan = subdivisions("jp", "ja-JP");
    // The 47 prefectures CLDR names and the 21 municipalities the holiday
    // tables list, which CLDR does not.
    assert_eq!(japan.lines().count(), 47 + 21);
    assert_eq!(
        japan
            .lines()
            .filter(|line| line.ends_with("\tmunicipal"))
            .count(),
        21
    );
    assert!(japan.lines().all(|line| line.split('\t').count() == 6));
    assert!(
        japan
            .lines()
            .any(|line| line == "JP-13\t東京都\tTokyo\tja\tprovisional\tregular")
    );
    assert_eq!(
        line("jp-13", "en"),
        "JP-13\tTokyo\tTokyo\ten\tapproved\tregular\n"
    );
    assert_eq!(line("JP", "ja"), "JP\t日本\tJapan\tja\tapproved\tregular\n");
}

#[test]
fn every_subdivision_and_territory_has_a_line() {
    let every = subdivisions("", "de");
    assert_eq!(every.lines().count(), 5503 + 21);
    let territories = read_lines(|buffer, capacity| unsafe {
        hc_territories("de".as_ptr(), 2, buffer, capacity)
    });
    assert_eq!(territories.lines().count(), 295);
    assert!(
        territories
            .lines()
            .any(|line| line == "DE\tDeutschland\tGermany\tde\tapproved\tregular")
    );
}

#[test]
fn a_code_nothing_names_is_unknown() {
    assert_eq!(place("jp13", "ja"), HC_ERR_UNKNOWN);
    assert_eq!(place("XX", "ja"), HC_ERR_UNKNOWN);
    let (country, locale) = ("JPN", "ja");
    assert_eq!(
        unsafe {
            hc_subdivisions(
                country.as_ptr(),
                country.len(),
                locale.as_ptr(),
                locale.len(),
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
    let not_utf8 = [0xffu8];
    assert_eq!(
        unsafe {
            hc_place_name(
                not_utf8.as_ptr(),
                1,
                locale.as_ptr(),
                2,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_NOT_UTF8
    );
    let antarctica = "AQ";
    assert_eq!(
        unsafe {
            hc_subdivisions(
                antarctica.as_ptr(),
                2,
                locale.as_ptr(),
                2,
                core::ptr::null_mut(),
                0,
            )
        },
        0
    );
}
