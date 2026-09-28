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
