//! The almanac's directions, 臘日, undertakings and a person's own days
//! through the C boundary.

use super::super::*;
use super::{measured, read_lines};

/// 29 September 2026, in a 丙午 year: 歳破神 on 子, opposite 太歳神
/// (古文書ネット, `komonjyo-hasshojin`); 2025, 乙巳, has 金神 on 辰 and 巳
/// (`komonjyo-konjin`).
#[test]
fn the_lines_are_the_modules() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_almanac_directions(739_888, c"japan".as_ptr(), buffer, capacity, written)
    });
    assert!(
        text.lines()
            .any(|line| line.starts_with("saiha\t歳破神\tsaihajin\t子\t0\t"))
    );
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_almanac_directions(739_403, c"japan".as_ptr(), buffer, capacity, written)
    });
    let konjin: Vec<&str> = text
        .lines()
        .filter(|line| line.starts_with("konjin\t"))
        .map(|line| line.split('\t').nth(3).unwrap_or(""))
        .collect();
    assert_eq!(konjin, ["辰", "巳"]);
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_mansion_undertakings(c"linderabell".as_ptr(), 739_624, buffer, capacity, written)
    });
    assert!(text.lines().all(|line| line.starts_with("1\t角\t")));
    // Japanese Wikipedia's person born in a 巳 year keeps 大禍日 on 15 May
    // 2025, a 申 day of 巳月.
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_almanac_person_days(739_386, 2025, c"japan".as_ptr(), buffer, capacity, written)
    });
    assert!(
        text.lines()
            .any(|line| line == "three-evil-day\ttaikanichi\t大禍日\t巳\t1")
    );
}

#[test]
fn rounichi_writes_its_day_or_refuses() {
    let mut day = 0i64;
    let status = unsafe {
        hc_rounichi(
            c"first-dog-after-major-cold".as_ptr(),
            2026,
            c"japan".as_ptr(),
            &mut day,
        )
    };
    assert_eq!(status, HC_OK);
    assert_eq!(day, 739_640);
    let status = unsafe { hc_rounichi(c"x".as_ptr(), 2026, c"japan".as_ptr(), &mut day) };
    assert_eq!(status, HC_ERROR_UNKNOWN);
    let status = unsafe { hc_rounichi(core::ptr::null(), 2026, c"japan".as_ptr(), &mut day) };
    assert_eq!(status, HC_ERROR_NULL_POINTER);
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_mansion_undertakings(c"almanac".as_ptr(), 739_624, buffer, capacity, written)
        }),
        HC_ERROR_UNKNOWN
    );
}

/// The South China Morning Post's widow year of 2024, Chinese year 4661,
/// with Wikipedia's Chinese names; a birth after the day asked has no age.
#[test]
fn the_augury_names_and_the_ages_cross() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_chinese_marriage_augury(4_661, buffer, capacity, written)
    });
    assert!(text.starts_with("widow\t0\t0\t無春年;"));
    let mut age = 0u32;
    let status = unsafe { hc_chinese_age(c"year-age".as_ptr(), 739_000, 738_000, &mut age) };
    assert_eq!(status, HC_ERROR_NO_DATA);
    let status =
        unsafe { hc_chinese_age(c"new-year-day-age".as_ptr(), 739_250, 739_252, &mut age) };
    assert_eq!((status, age), (HC_OK, 2));
}

/// SE 1 is 1 Seleucus I Nicator (van Gent's converter, as
/// `hc-calendars-lunar` holds it); 1956's doubt is refused.
#[test]
fn the_reckonings_of_the_calendars_cross() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_babylonian_regnal_year(1, buffer, capacity, written)
    });
    assert_eq!(text, "Seleucus I Nicator\t1\n");
    let mut olympiad = 0i64;
    let status = unsafe { hc_ioc_olympiad_on(714_262, &mut olympiad) };
    assert_eq!(status, HC_ERROR_NO_DATA);
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_day_name(
                c"armenian".as_ptr(),
                c"fr".as_ptr(),
                739_888,
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_UNKNOWN
    );
}

/// Five decimal hours are noon.
#[cfg(feature = "timestamps")]
#[test]
fn five_decimal_hours_are_noon() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_civil_from_french_decimal_time(5, 0, 0, 0, buffer, capacity, written)
    });
    assert_eq!(text, "43200\t0\n");
}

/// 令和 is among the Japanese eras under `ja`; Greek 2026 reads back.
#[test]
fn the_eras_and_the_numbers_cross() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_calendar_eras(
            c"japanese".as_ptr(),
            c"ja".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert!(text.lines().any(|line| line.starts_with("reiwa\t令和\t")));
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_format_number(c"grek".as_ptr(), 2_026, buffer, capacity, written)
    });
    let numeral = text.split('\t').next().unwrap_or("");
    let numeral = std::ffi::CString::new(numeral).expect("no NUL");
    let mut value = 0i64;
    let status = unsafe { hc_parse_number(c"grek".as_ptr(), numeral.as_ptr(), &mut value) };
    assert_eq!((status, value), (HC_OK, 2_026));
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_numbering_systems(buffer, capacity, written)
    });
    assert!(text.lines().any(|line| line == "latn\t0\t0123456789"));
}
