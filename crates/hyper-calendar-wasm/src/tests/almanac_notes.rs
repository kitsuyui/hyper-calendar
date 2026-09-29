//! The almanac's directions, 臘日, undertakings and a person's own days
//! through the WebAssembly boundary.

use super::super::*;
use super::read_lines;

const JAPAN: &str = "japan";

/// 29 September 2026, in a 丙午 year: 太歳神 on 午 (古文書ネット,
/// `komonjyo-hasshojin`).
#[test]
fn the_directions_are_the_facades() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_almanac_directions(739_888, JAPAN.as_ptr(), JAPAN.len(), buffer, capacity)
    });
    let first = text.lines().next().expect("a line");
    assert_eq!(
        first,
        "taisai\t太歳神\ttaisaijin\t午\t180\tfacing it everything goes well, but do not fell trees\t丙午\t7"
    );
    assert!(text.lines().all(|line| line.split('\t').count() == 8));
    let refused =
        unsafe { hc_almanac_directions(739_888, "mars".as_ptr(), 4, core::ptr::null_mut(), 0) };
    assert_eq!(refused, HC_ERR_UNKNOWN);
}

/// こよみる's 臘日 of 2026 by the 辰 nearest 大寒, 18 January
/// (`koyomil-rounichi`).
#[test]
fn rounichi_is_a_fixed_day() {
    let rule = "dragon-nearest-major-cold-earlier";
    let day = unsafe { hc_rounichi(rule.as_ptr(), rule.len(), 2026, JAPAN.as_ptr(), JAPAN.len()) };
    assert_eq!(day, 739_634);
    let early = unsafe {
        hc_rounichi(
            rule.as_ptr(),
            rule.len(),
            -1000,
            JAPAN.as_ptr(),
            JAPAN.len(),
        )
    };
    assert_eq!(early, HC_ERR_OUT_OF_RANGE);
    let unknown = unsafe { hc_rounichi("x".as_ptr(), 1, 2026, JAPAN.as_ptr(), JAPAN.len()) };
    assert_eq!(unknown, HC_ERR_UNKNOWN);
}

/// 8 January 2026 is 角宿, for which 歳事暦 favours 衣類裁断 first.
#[test]
fn the_undertakings_and_a_persons_days_are_the_facades() {
    let list = "saijigoyomi";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_mansion_undertakings(list.as_ptr(), list.len(), 739_624, buffer, capacity)
    });
    assert_eq!(text.lines().next(), Some("1\t角\tfavoured\t衣類裁断"));
    // こよみる's 2025 五墓日 of a person born in 1928: 25 February.
    let text = read_lines(|buffer, capacity| unsafe {
        hc_almanac_person_days(739_307, 1928, JAPAN.as_ptr(), JAPAN.len(), buffer, capacity)
    });
    assert_eq!(
        text.lines().next(),
        Some("grave-day\tgomunichi-wikipedia\t五墓日\t乙丑\t1")
    );
    assert_eq!(text.lines().count(), 5);
}

/// Wikipedia's child born on 1 June 2000 is 13 *suì* from the lunar new
/// year of 2012 (`wikipedia-en-east-asian-age-reckoning`).
#[test]
fn an_age_is_a_count_by_name() {
    let count = "chinese-age";
    let age = unsafe { hc_chinese_age(count.as_ptr(), count.len(), 730_272, 734_525) };
    assert_eq!(age, 13);
    let unknown = unsafe { hc_chinese_age("x".as_ptr(), 1, 730_272, 734_525) };
    assert_eq!(unknown, HC_ERR_UNKNOWN);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_chinese_almanac_solar_terms(1700, buffer, capacity)
    });
    assert_eq!(text.lines().count(), 24);
}

/// The XXXIII Olympiad of Paris 2024; the first day of
/// An II, Raisin.
#[test]
fn the_reckonings_of_the_calendars_cross() {
    assert_eq!(hc_ioc_olympiad_on(739_888), 33);
    let (calendar, naming) = ("french-republican-arithmetic", "fr");
    let text = read_lines(|buffer, capacity| unsafe {
        hc_day_name(
            calendar.as_ptr(),
            calendar.len(),
            naming.as_ptr(),
            naming.len(),
            654_780,
            buffer,
            capacity,
        )
    });
    assert!(text.starts_with("Raisin\tfr\t"));
    let tekufah = "nisan";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_shmuel_tekufah(5_769, tekufah.as_ptr(), tekufah.len(), buffer, capacity)
    });
    assert!(text.ends_with("\tnisan\n"));
}

/// Article XI's noon, five decimal hours.
#[cfg(feature = "timestamps")]
#[test]
fn noon_is_five_decimal_hours() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_french_decimal_time(43_200, 0, buffer, capacity)
    });
    assert_eq!(text, "5\t0\t0\t0\n");
}

/// The Hebrew numeral of 5786, ה׳תשפ״ו (`docs/systems/hebrew-numerals.md`),
/// written and read back.
#[test]
fn a_number_crosses_in_its_system() {
    let system = "hebr";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_format_number(system.as_ptr(), system.len(), 5_786, buffer, capacity)
    });
    assert_eq!(text, "ה׳תשפ״ו\thebr\n");
    let numeral = "ה׳תשפ״ו";
    let value = unsafe {
        hc_parse_number(
            system.as_ptr(),
            system.len(),
            numeral.as_ptr(),
            numeral.len(),
        )
    };
    assert_eq!(value, 5_786);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_day_period(54_000, "en".as_ptr(), 2, buffer, capacity)
    });
    assert!(text.starts_with("pm\tPM\tafternoon1\t"));
}
