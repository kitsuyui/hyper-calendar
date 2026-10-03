use super::super::*;
use super::read_lines;

fn columns(text: &str) -> Vec<&str> {
    let line = text.strip_suffix('\n').expect("one line");
    assert!(!line.contains('\n'), "{text:?}");
    line.split('\t').collect()
}

#[test]
fn the_term_in_effect_decodes_column_by_column() {
    // 2024-02-04 was 立春 in Japan; the term runs to 18 February.
    let day = hc_gregorian_to_fixed(2024, 2, 10);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_term_in_effect(day, "japan".as_ptr(), 5, buffer, capacity)
    });
    let columns = columns(&text);
    assert_eq!(columns.len(), 7, "{columns:?}");
    assert_eq!(columns[..3], ["21", "立春", "立春"]);
    assert_eq!(columns[3], hc_gregorian_to_fixed(2024, 2, 4).to_string());
    assert_eq!(columns[4], hc_gregorian_to_fixed(2024, 2, 18).to_string());
    assert!(
        !columns[5].is_empty() && !columns[6].is_empty(),
        "{columns:?}"
    );
}

#[test]
fn the_pentad_in_effect_decodes_column_by_column() {
    let day = hc_gregorian_to_fixed(2024, 2, 10);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_pentad_in_effect(day, "japan".as_ptr(), 5, buffer, capacity)
    });
    let columns = columns(&text);
    assert_eq!(columns.len(), 7, "{columns:?}");
    // 立春 is pentads 63, 64 and 65 from 春分; 10 February is in the
    // second of them, 黄鶯睍睆 in Japan and 蟄蟲始振 in China.
    assert_eq!(columns[..3], ["64", "蟄蟲始振", "黄鶯睍睆"]);
    assert_eq!(columns[3], hc_gregorian_to_fixed(2024, 2, 9).to_string());
    assert_eq!(columns[4], hc_gregorian_to_fixed(2024, 2, 13).to_string());
    assert!(
        !columns[5].is_empty() && !columns[6].is_empty(),
        "{columns:?}"
    );
}

#[test]
fn a_meridian_is_a_name_or_a_longitude() {
    let day = hc_gregorian_to_fixed(2024, 2, 4);
    let at = |name: &str| {
        columns(&read_lines(|buffer, capacity| unsafe {
            hc_term_in_effect(day, name.as_ptr(), name.len(), buffer, capacity)
        }))[3]
            .to_owned()
    };
    // 立春 2024 began at 08:27 UT on 4 February by this model: the
    // 4th from 120°W east to Tokyo, still the 3rd at 180°W.
    let fourth = day.to_string();
    let third = (day - 1).to_string();
    assert_eq!(at("japan"), fourth);
    assert_eq!(at("JAPAN"), fourth);
    assert_eq!(at("china"), fourth);
    assert_eq!(at("135"), fourth);
    assert_eq!(at("135.0"), fourth);
    assert_eq!(at("universal"), fourth);
    assert_eq!(at(""), fourth);
    assert_eq!(at("0"), fourth);
    assert_eq!(at("-120"), fourth);
    assert_eq!(at("-180"), third);
    for bad in ["mars", "181", "nan", "1e400"] {
        assert_eq!(
            unsafe { hc_term_in_effect(day, bad.as_ptr(), bad.len(), core::ptr::null_mut(), 0) },
            HC_ERR_UNKNOWN,
            "{bad}"
        );
    }
    let not_utf8 = [0xffu8];
    assert_eq!(
        unsafe { hc_pentad_in_effect(day, not_utf8.as_ptr(), 1, core::ptr::null_mut(), 0) },
        HC_ERR_NOT_UTF8
    );
}

/// KASI's 월력요항: 한식 on 5 April 2024 and 6 April 2026.
#[test]
fn the_cold_food_day_crosses_the_boundary() {
    let day = |id: &str, year: i64| unsafe { hc_cold_food_day(id.as_ptr(), id.len(), year) };
    assert_eq!(day("hansik", 2024), hc_gregorian_to_fixed(2024, 4, 5));
    assert_eq!(day("Hansik", 2026), hc_gregorian_to_fixed(2026, 4, 6));
    let eve = day("hanshi-eve-of-qingming", 2026);
    let older = day("hanshi-solstice-105", 2026);
    assert!(matches!(older - eve, 1 | 2), "{eve} {older}");
    assert_eq!(day("hanshi", 2026), HC_ERR_UNKNOWN);
    assert_eq!(day("hansik", -1000), HC_ERR_OUT_OF_RANGE);
    assert_eq!(day("hansik", 3001), HC_ERR_OUT_OF_RANGE);
    assert!(day("hansik", -999) > HC_ERR_FLOOR);
    let not_utf8 = [0xffu8];
    assert_eq!(
        unsafe { hc_cold_food_day(not_utf8.as_ptr(), 1, 2026) },
        HC_ERR_NOT_UTF8
    );
}

/// The days of the years −1000 to 3000 answer, as `hc_sky_at`'s do;
/// the days either side of them are refused rather than computed
/// from a series stated for that era only.
#[test]
fn the_term_and_pentad_refuse_days_outside_the_era() {
    let first = hc_gregorian_to_fixed(-1000, 1, 1);
    let last = hc_gregorian_to_fixed(3000, 12, 31);
    let null = core::ptr::null_mut();
    for day in [first, last] {
        assert!(unsafe { hc_term_in_effect(day, "".as_ptr(), 0, null, 0) } > 0);
        assert!(unsafe { hc_pentad_in_effect(day, "".as_ptr(), 0, null, 0) } > 0);
    }
    for day in [first - 1, last + 1, i64::MIN, i64::MAX] {
        assert_eq!(
            unsafe { hc_term_in_effect(day, "".as_ptr(), 0, null, 0) },
            HC_ERR_OUT_OF_RANGE,
            "{day}"
        );
        assert_eq!(
            unsafe { hc_pentad_in_effect(day, "".as_ptr(), 0, null, 0) },
            HC_ERR_OUT_OF_RANGE,
            "{day}"
        );
    }
}

/// The 暦Wiki's table of the 七十二候 (`nao-rekiwiki-72ko`): 立春次候 is
/// 蟄虫始振 in the 宣明暦's list, 梅花乃芳 in the 貞享暦's and 黄鶯睍睆 in the
/// 宝暦暦's, which the almanac prints today; 大雪次候's 虎始交 carries the
/// alternate 武始交 in the first.
#[test]
fn the_pentad_traditions_and_a_pentad_named_by_each_cross_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe { hc_pentad_traditions(buffer, capacity) });
    let ids: Vec<&str> = text
        .lines()
        .map(|line| line.split('\t').next().expect("an id"))
        .collect();
    assert_eq!(ids, ["chinese", "japanese", "jokyo", "senmyo"]);
    let day = hc_gregorian_to_fixed(2024, 2, 10);
    let named = |tradition: &str, day: i64| {
        let text = read_lines(|buffer, capacity| unsafe {
            hc_pentad_in_tradition(
                day,
                tradition.as_ptr(),
                tradition.len(),
                "japan".as_ptr(),
                5,
                buffer,
                capacity,
            )
        });
        columns(&text)
            .iter()
            .map(|cell| (*cell).to_owned())
            .collect::<Vec<_>>()
    };
    for (tradition, name) in [
        ("senmyo", "蟄虫始振"),
        ("JOKYO", "梅花乃芳"),
        ("japanese", "黄鶯睍睆"),
    ] {
        let row = named(tradition, day);
        assert_eq!(row.len(), 8, "{row:?}");
        assert_eq!(row[..2], ["64", name]);
        assert_eq!(row[4], hc_gregorian_to_fixed(2024, 2, 9).to_string());
    }
    let tiger = named("senmyo", hc_gregorian_to_fixed(2026, 12, 14));
    assert_eq!(tiger[..4], ["52", "虎始交", tiger[2].as_str(), "武始交"]);
    let none = unsafe {
        hc_pentad_in_tradition(
            day,
            "horyaku".as_ptr(),
            7,
            "japan".as_ptr(),
            5,
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(none, HC_ERR_UNKNOWN);
}

/// 暦要項 2024: 節分 on 3 February, 入梅 on 10 June, 土用の入り on 19 July with
/// its 丑の日 24 July and 5 August; and the three 伏 of 2026 in China begin
/// on 15 July, 25 July and 14 August.
#[test]
fn the_zassetsu_and_the_seasonal_days_cross_the_boundary() {
    let rows = |text: &str| {
        text.lines()
            .map(|line| line.split('\t').map(str::to_owned).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    };
    let japan = "japan";
    let zassetsu = read_lines(|buffer, capacity| unsafe {
        hc_zassetsu_in_year(2024, japan.as_ptr(), japan.len(), buffer, capacity)
    });
    let zassetsu = rows(&zassetsu);
    assert_eq!(zassetsu.len(), 25);
    let day = |year, month, date| hc_gregorian_to_fixed(year, month, date).to_string();
    let find = |id: &str| {
        zassetsu
            .iter()
            .find(|row| row[0] == id)
            .expect("a day")
            .clone()
    };
    assert_eq!(find("spring-setsubun")[5], day(2024, 2, 3));
    assert_eq!(find("nyubai")[5], day(2024, 6, 10));
    let summer = find("summer-doyo-entry");
    assert_eq!(summer[5], day(2024, 7, 19));
    assert_eq!(summer[7..9], [day(2024, 7, 24), day(2024, 8, 5)]);
    let china = "china";
    let seasonal = read_lines(|buffer, capacity| unsafe {
        hc_seasonal_days_in_year(2026, china.as_ptr(), china.len(), buffer, capacity)
    });
    let seasonal = rows(&seasonal);
    assert!(seasonal.iter().all(|row| row.len() == 7));
    let fu: Vec<&str> = seasonal
        .iter()
        .filter(|row| row[0] == "san-fu")
        .map(|row| row[4].as_str())
        .collect();
    assert_eq!(fu, [day(2026, 7, 15), day(2026, 7, 25), day(2026, 8, 14)]);
    let bad = unsafe { hc_zassetsu_in_year(2024, "mars".as_ptr(), 4, core::ptr::null_mut(), 0) };
    assert_eq!(bad, HC_ERR_UNKNOWN);
    let far = unsafe {
        hc_seasonal_days_in_year(3001, japan.as_ptr(), japan.len(), core::ptr::null_mut(), 0)
    };
    assert_eq!(far, HC_ERR_OUT_OF_RANGE);
}
