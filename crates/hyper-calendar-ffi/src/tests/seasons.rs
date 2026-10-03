use super::super::*;
use super::read_lines;

#[test]
fn the_term_and_pentad_in_effect_decode_column_by_column() {
    let mut day = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2024, 2, 10, &mut day) },
        HC_OK
    );
    let term = read_lines(|buffer, capacity, written| unsafe {
        hc_term_in_effect(day, c"japan".as_ptr(), buffer, capacity, written)
    });
    let columns: Vec<&str> = term.trim_end().split('\t').collect();
    assert_eq!(columns.len(), 7, "{columns:?}");
    assert_eq!(columns[..3], ["21", "立春", "立春"]);
    assert_eq!(columns[3], (day - 6).to_string());
    assert_eq!(columns[4], (day + 8).to_string());
    let pentad = read_lines(|buffer, capacity, written| unsafe {
        hc_pentad_in_effect(day, core::ptr::null(), buffer, capacity, written)
    });
    let columns: Vec<&str> = pentad.trim_end().split('\t').collect();
    assert_eq!(columns.len(), 7, "{columns:?}");
    assert_eq!(columns[..3], ["64", "蟄蟲始振", "黄鶯睍睆"]);
    assert_eq!(
        unsafe {
            hc_term_in_effect(
                day,
                c"mars".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_pentad_in_effect(
                day,
                c"\xff".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_NOT_UTF8
    );
}

/// KASI's 월력요항: 한식 on 6 April 2026.
#[test]
fn the_cold_food_day_crosses_through_an_out_parameter() {
    let (mut day, mut expected) = (0i64, 0i64);
    assert_eq!(
        unsafe { hc_cold_food_day(c"hansik".as_ptr(), 2026, &mut day) },
        HC_OK
    );
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2026, 4, 6, &mut expected) },
        HC_OK
    );
    assert_eq!(day, expected);
    assert_eq!(
        unsafe { hc_cold_food_day(c"hanshi".as_ptr(), 2026, &mut day) },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe { hc_cold_food_day(c"hansik".as_ptr(), 3001, &mut day) },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_cold_food_day(core::ptr::null(), 2026, &mut day) },
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        unsafe { hc_cold_food_day(c"hansik".as_ptr(), 2026, core::ptr::null_mut()) },
        HC_ERROR_NULL_POINTER
    );
}

/// The days of the years −1000 to 3000 answer, as `hc_sky_at`'s do;
/// the days either side of them are refused.
#[test]
fn the_term_and_pentad_refuse_days_outside_the_era() {
    let (mut first, mut last) = (0i64, 0i64);
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(-1000, 1, 1, &mut first) },
        HC_OK
    );
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(3000, 12, 31, &mut last) },
        HC_OK
    );
    let null = core::ptr::null_mut();
    for day in [first, last] {
        let mut written = 0usize;
        assert_eq!(
            unsafe { hc_term_in_effect(day, core::ptr::null(), null, 0, &mut written) },
            HC_ERROR_BUFFER_TOO_SMALL
        );
        assert!(written > 0);
    }
    for day in [first - 1, last + 1, i64::MIN, i64::MAX] {
        for status in [
            unsafe { hc_term_in_effect(day, core::ptr::null(), null, 0, null.cast()) },
            unsafe { hc_pentad_in_effect(day, core::ptr::null(), null, 0, null.cast()) },
        ] {
            assert_eq!(status, HC_ERROR_OUT_OF_RANGE, "{day}");
        }
    }
}

/// The 暦Wiki's table of the 七十二候 (`nao-rekiwiki-72ko`): 立春次候 is
/// 蟄虫始振 in the 宣明暦's list, 梅花乃芳 in the 貞享暦's and 黄鶯睍睆 in the
/// 宝暦暦's, which the almanac prints today; 大雪次候's 虎始交 carries the
/// alternate 武始交 in the first.
#[test]
fn the_pentad_traditions_and_a_pentad_named_by_each_cross_the_c_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_pentad_traditions(buffer, capacity, written)
    });
    let ids: Vec<&str> = text
        .lines()
        .map(|line| line.split('\t').next().expect("an id"))
        .collect();
    assert_eq!(ids, ["chinese", "japanese", "jokyo", "senmyo"]);
    let mut day = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2024, 2, 10, &mut day) },
        HC_OK
    );
    for (tradition, name) in [
        (c"senmyo", "蟄虫始振"),
        (c"JOKYO", "梅花乃芳"),
        (c"japanese", "黄鶯睍睆"),
    ] {
        let text = read_lines(|buffer, capacity, written| unsafe {
            hc_pentad_in_tradition(
                day,
                tradition.as_ptr(),
                c"japan".as_ptr(),
                buffer,
                capacity,
                written,
            )
        });
        let columns: Vec<&str> = text.trim_end().split('\t').collect();
        assert_eq!(columns.len(), 8, "{columns:?}");
        assert_eq!(columns[..2], ["64", name]);
    }
    let mut tiger_day = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2026, 12, 14, &mut tiger_day) },
        HC_OK
    );
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_pentad_in_tradition(
            tiger_day,
            c"senmyo".as_ptr(),
            c"japan".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(text.trim_end().split('\t').nth(3), Some("武始交"));
    assert_eq!(
        unsafe {
            hc_pentad_in_tradition(
                day,
                c"horyaku".as_ptr(),
                c"japan".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_pentad_in_tradition(
                day,
                core::ptr::null(),
                c"japan".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_NULL_POINTER
    );
}

/// 暦要項 2024: 節分 on 3 February, 入梅 on 10 June, 土用の入り on 19 July with
/// its 丑の日 24 July and 5 August; and the three 伏 of 2026 in China begin
/// on 15 July, 25 July and 14 August.
#[test]
fn the_zassetsu_and_the_seasonal_days_cross_the_c_boundary() {
    let day = |year, month, date| {
        let mut out = 0i64;
        assert_eq!(
            unsafe { hc_gregorian_to_fixed(year, month, date, &mut out) },
            HC_OK
        );
        out.to_string()
    };
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_zassetsu_in_year(2024, c"japan".as_ptr(), buffer, capacity, written)
    });
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert_eq!(rows.len(), 25);
    let summer = rows
        .iter()
        .find(|row| row[0] == "summer-doyo-entry")
        .expect("a day");
    assert_eq!(summer[5], day(2024, 7, 19));
    assert_eq!(summer[7..9], [day(2024, 7, 24), day(2024, 8, 5)]);
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_seasonal_days_in_year(2026, c"china".as_ptr(), buffer, capacity, written)
    });
    let fu: Vec<&str> = text
        .lines()
        .filter(|line| line.starts_with("san-fu\t"))
        .map(|line| line.split('\t').nth(4).expect("a day"))
        .collect();
    assert_eq!(fu, [day(2026, 7, 15), day(2026, 7, 25), day(2026, 8, 14)]);
    assert_eq!(
        unsafe {
            hc_zassetsu_in_year(
                2024,
                c"mars".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_seasonal_days_in_year(
                3001,
                c"japan".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
}
