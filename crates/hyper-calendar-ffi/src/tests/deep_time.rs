use super::super::*;
use super::read_lines;

fn row(text: &str) -> Vec<String> {
    text.trim_end().split('\t').map(str::to_owned).collect()
}

#[test]
fn deep_time_lines_decode_column_by_column() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_place_years_ago(66.0e6, 0.0, c"ja-JP".as_ptr(), buffer, capacity, written)
    });
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert!(rows.iter().all(|row| row.len() == 16), "{rows:?}");
    let kinds: Vec<&str> = rows.iter().map(|row| row[0]).collect();
    assert_eq!(
        kinds,
        [
            "moment",
            "moment",
            "cosmic-epoch",
            "cosmic-event",
            "eon",
            "era",
            "period",
            "epoch",
            "age"
        ]
    );
    assert_eq!(
        rows[8][..4],
        ["age", "maastrichtian", "Maastrichtian", "upper-cretaceous"]
    );
    assert_eq!(rows[8][12], "megayears-before-present");
    assert_eq!(rows[8][15], "マーストリヒチアン");
    assert_eq!(rows[2][15], "");
    // A few years ahead the whole present chain is still there.
    let near = read_lines(|buffer, capacity, written| unsafe {
        hc_place_years_ago(-3.0, 0.0, core::ptr::null(), buffer, capacity, written)
    });
    assert!(near.contains("\tMeghalayan\t"), "{near}");
    assert!(near.contains("\tModern period\t"), "{near}");
    let cosmic = read_lines(|buffer, capacity, written| unsafe {
        hc_cosmic_events(core::ptr::null(), buffer, capacity, written)
    });
    assert_eq!(
        cosmic.lines().count(),
        hc::hc_deep_time::universe::EPOCHS.len() + hc::hc_deep_time::universe::EVENTS.len()
    );
    let evidence = read_lines(|buffer, capacity, written| unsafe {
        hc_earliest_evidence(c"ja".as_ptr(), buffer, capacity, written)
    });
    assert_eq!(
        evidence.lines().count(),
        hc::hc_deep_time::evidence::EVIDENCE.len()
    );
    assert!(
        evidence.contains("\tearliest-writing-uruk-iv\t"),
        "{evidence}"
    );
    assert!(evidence.contains("\t原楔形文字\n"), "{evidence}");
    let periods = read_lines(|buffer, capacity, written| unsafe {
        hc_archaeological_periods(core::ptr::null(), buffer, capacity, written)
    });
    assert!(periods.starts_with("archaeological\tmodern-period\tModern period\t"));
    let future = read_lines(|buffer, capacity, written| unsafe {
        hc_future_events(core::ptr::null(), buffer, capacity, written)
    });
    assert_eq!(
        future.lines().count(),
        hc::hc_deep_time::future::EVENTS.len()
    );
    assert!(future.starts_with("future-event\tsun-leaves-main-sequence\t"));
    let eons = read_lines(|buffer, capacity, written| unsafe {
        hc_geologic_intervals(0, c"zh-Hans".as_ptr(), buffer, capacity, written)
    });
    assert!(
        eons.starts_with("eon\tphanerozoic\tPhanerozoic\t\t538.8\t0.6\t4\t0\t0\t0\t1\t0\t"),
        "{eons}"
    );
    assert!(
        eons.lines()
            .next()
            .is_some_and(|line| line.ends_with("\t显生宇"))
    );
    assert_eq!(
        unsafe {
            hc_geologic_intervals(
                5,
                core::ptr::null(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_place_years_ago(
                f64::NAN,
                0.0,
                core::ptr::null(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
    // A locale that is not UTF-8, for each table export.
    let not_utf8 = c"\xff".as_ptr();
    let null = core::ptr::null_mut();
    for status in unsafe {
        [
            hc_earliest_evidence(not_utf8, null, 0, core::ptr::null_mut()),
            hc_archaeological_periods(not_utf8, null, 0, core::ptr::null_mut()),
            hc_future_events(not_utf8, null, 0, core::ptr::null_mut()),
        ]
    } {
        assert_eq!(status, HC_ERROR_NOT_UTF8);
    }
}

#[test]
fn hc_planck_units_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_planck_units(buffer, capacity, written)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 10),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "c");
}

#[test]
fn hc_bp_convert_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_bp_convert(
            11650.0,
            5.0,
            c"bp".as_ptr(),
            c"b2k".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 8),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "bp");
    assert_eq!(first[1], "b2k");
    assert_eq!(first[4], "11700");
    assert_eq!(first[6], "11700 b2k");
    assert_eq!(
        unsafe {
            hc_bp_convert(
                3200.0,
                50.0,
                c"radiocarbon-bp".as_ptr(),
                c"bp".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_NO_DATA
    );
    assert_eq!(
        unsafe {
            hc_bp_convert(
                1.0,
                0.0,
                c"bp".as_ptr(),
                c"ad".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
}

#[test]
fn hc_deep_convert_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_deep_convert(
            13.787,
            0.02,
            c"gigayear".as_ptr(),
            c"second".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 13),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "gigayear");
    assert_eq!(first[1], "second");
    assert_eq!(first[11], "1");
    assert_eq!(
        unsafe {
            hc_deep_convert(
                1.0,
                0.0,
                c"gigayear".as_ptr(),
                c"furlong".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_deep_convert(
                1.0,
                -1.0,
                c"gigayear".as_ptr(),
                c"second".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
}

#[test]
fn hc_deep_compare_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_deep_compare(
            13.787,
            0.02,
            c"gigayear".as_ptr(),
            1.0,
            0.0,
            c"planck-time".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 12),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[9], "0");
    assert_eq!(first[10], "1");
    assert_eq!(
        unsafe {
            hc_deep_compare(
                0.0,
                0.0,
                c"day".as_ptr(),
                1.0,
                0.0,
                c"day".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe {
            hc_deep_compare(
                1.0,
                0.0,
                c"day".as_ptr(),
                1.0,
                0.0,
                c"fortnight".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
}
