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
        // Born on 1 June 1928.
        hc_almanac_person_days(
            739_307,
            703_974,
            JAPAN.as_ptr(),
            JAPAN.len(),
            buffer,
            capacity,
        )
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
    // At the reckoning's nightfall on Tuesday 7 April 2009, 733 504: the
    // Hebrew day of Wednesday, after nightfall.
    assert_eq!(text, "733505\t1080\tnisan\t1\t733504\n");
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

/// Japanese Wikipedia's 元号一覧 (日本): 248 eras to 令和, which began on
/// 1 May 2019 (RD 737 180); Olympedia's editions: Paris 2024 opened on
/// 26 July, and the VI Summer Games of 1916 were not held.
#[test]
fn the_era_table_and_the_games_cross_the_boundary() {
    let table = |name: &str| {
        read_lines(|buffer, capacity| unsafe {
            hc_era_table(name.as_ptr(), name.len(), buffer, capacity)
        })
    };
    let japanese = table("Japanese");
    assert_eq!(japanese.lines().count(), 248);
    assert!(japanese.lines().any(|line| {
        line.starts_with("reiwa\t令和\tれいわ\tReiwa\tunified\t2019\t\t737180\t\tattested")
    }));
    assert_eq!(table("chinese-regnal").lines().count(), 37);
    assert_eq!(table("korean-regnal").lines().count(), 3);
    let games = |season: &str| {
        read_lines(|buffer, capacity| unsafe {
            hc_olympic_games(season.as_ptr(), season.len(), buffer, capacity)
        })
    };
    let summer = games("summer");
    assert!(
        summer
            .lines()
            .any(|line| line.starts_with("33\t2024\tParis\tcelebrated\t"))
    );
    assert!(
        summer
            .lines()
            .any(|line| line == "6\t1916\tBerlin\tnot-held\t\t")
    );
    assert!(
        games("Winter")
            .lines()
            .any(|line| line.starts_with("24\t2022\tBeijing\t"))
    );
    let unknown = unsafe { hc_era_table("x".as_ptr(), 1, core::ptr::null_mut(), 0) };
    assert_eq!(unknown, HC_ERR_UNKNOWN);
    let spring = unsafe { hc_olympic_games("spring".as_ptr(), 6, core::ptr::null_mut(), 0) };
    assert_eq!(spring, HC_ERR_UNKNOWN);
}

/// The Nguyễn table: twelve eras from Gia Long's 1 June 1802 to Bảo Đại's
/// abdication on 30 August 1945, Hiệp Hòa taken and never counted.
#[test]
fn the_nguyen_eras_cross_the_boundary_with_their_days() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_era_table("vietnamese-regnal-nguyen".as_ptr(), 24, buffer, capacity)
    });
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert_eq!(rows.len(), 12, "{text}");
    assert!(rows.iter().all(|row| row.len() == 12), "{text}");
    let gia_long = &rows[0];
    assert_eq!(
        gia_long[..6],
        ["gia-long", "嘉隆", "Gia Long", "Gia Long", "nguyen", "1802"]
    );
    assert_eq!(gia_long[6], "1820");
    assert_eq!(gia_long[7], hc_gregorian_to_fixed(1802, 6, 1).to_string());
    assert_eq!(
        gia_long[8],
        (hc_gregorian_to_fixed(1820, 2, 14) - 1).to_string()
    );
    assert_eq!(gia_long[9..11], ["kept", ""]);
    let hiep_hoa = rows
        .iter()
        .find(|row| row[0] == "hiep-hoa")
        .expect("Hiệp Hòa");
    // The year the name would have numbered, 1884, is its first year; it
    // has no last, never having been kept.
    assert_eq!(hiep_hoa[5..10], ["1884", "", hiep_hoa[7], "", "not-kept"]);
    assert!(
        !hiep_hoa[7].is_empty() && !hiep_hoa[11].is_empty(),
        "{hiep_hoa:?}"
    );
    let bao_dai = &rows[11];
    assert_eq!(bao_dai[0], "bao-dai");
    assert_eq!(bao_dai[6], "1945");
    assert_eq!(bao_dai[8], hc_gregorian_to_fixed(1945, 8, 30).to_string());
}
