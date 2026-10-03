use super::super::*;
use super::read_lines;

fn row(text: &str) -> Vec<String> {
    text.trim_end().split('\t').map(str::to_owned).collect()
}

fn cells(text: &str) -> Vec<Vec<&str>> {
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert!(rows.iter().all(|row| row.len() == 16), "{rows:?}");
    rows
}

#[test]
fn a_moment_is_placed_in_every_chronology() {
    // The end-Cretaceous extinction, 66 million years ago.
    let text = read_lines(|buffer, capacity| unsafe {
        hc_place_years_ago(66.0e6, 0.0, "ja".as_ptr(), 2, buffer, capacity)
    });
    let rows = cells(&text);
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
    assert_eq!(rows[0][2], "since-big-bang");
    assert_eq!(rows[0][12], "seconds-since-big-bang");
    assert_eq!(rows[1][2], "before-present");
    assert_eq!(rows[1][12], "seconds-before-present");
    assert_eq!(rows[2][1..3], ["era-of-galaxies", "Era of galaxies"]);
    let age = &rows[8];
    assert_eq!(
        age[..4],
        ["age", "maastrichtian", "Maastrichtian", "upper-cretaceous"]
    );
    // The older bound, then the younger, each with its own figures.
    assert_eq!(age[4..8], ["72.2", "0.2", "3", "0"]);
    assert_eq!(age[8..12], ["66", "0", "4", "0"]);
    assert_eq!(age[12], "megayears-before-present");
    assert!(age[14].contains("v2026/06"), "{age:?}");
    // The chart's Japanese name, from the Geological Society of
    // Japan's translation; the cosmic rows have none.
    assert_eq!(age[15], "マーストリヒチアン");
    assert_eq!(rows[6][15], "白亜系／紀");
    assert_eq!(rows[2][15], "");
}

#[test]
fn the_present_and_the_future_are_placed_too() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_place_years_ago(0.0, 0.0, core::ptr::null(), 0, buffer, capacity)
    });
    let rows = cells(&text);
    let archaeological = rows
        .iter()
        .find(|row| row[0] == "archaeological")
        .expect("now");
    assert_eq!(archaeological[2], "Modern period");
    assert_eq!(archaeological[12], "years-before-1950");
    assert!(!archaeological[14].is_empty());
    // Eight billion years ahead the Sun is a red giant and the
    // cosmic history tables have ended.
    let text = read_lines(|buffer, capacity| unsafe {
        hc_place_years_ago(-8.0e9, 0.0, core::ptr::null(), 0, buffer, capacity)
    });
    let rows = cells(&text);
    let kinds: Vec<&str> = rows.iter().map(|row| row[0]).collect();
    // The present day is the last dated event before any future
    // moment, so it is still listed.
    assert_eq!(kinds, ["moment", "moment", "cosmic-event", "future-era"]);
    assert_eq!(rows[2][2], "The present");
    assert_eq!(rows[3][2], "Stelliferous Era");
    assert_eq!(rows[3][12], "log10-years-from-now");
}

#[test]
fn the_near_future_keeps_the_whole_chain() {
    // Six hours and three years ahead: still the Meghalayan, the
    // Modern period and the era of galaxies, and the future era too.
    for years_ago in [-6.0 / (24.0 * 365.25), -3.0] {
        let text = read_lines(|buffer, capacity| unsafe {
            hc_place_years_ago(years_ago, 0.0, "en".as_ptr(), 2, buffer, capacity)
        });
        let rows = cells(&text);
        let kinds: Vec<&str> = rows.iter().map(|row| row[0]).collect();
        assert_eq!(
            kinds,
            [
                "moment",
                "moment",
                "cosmic-epoch",
                "cosmic-event",
                "future-era",
                "eon",
                "era",
                "period",
                "epoch",
                "age",
                "archaeological"
            ],
            "{years_ago}"
        );
        assert_eq!(rows[2][2], "Era of galaxies");
        assert_eq!(rows[9][2], "Meghalayan");
        assert_eq!(rows[10][2], "Modern period");
    }
    // Beyond a century ahead the future begins.
    let text = read_lines(|buffer, capacity| unsafe {
        hc_place_years_ago(-101.0, 0.0, core::ptr::null(), 0, buffer, capacity)
    });
    let kinds: Vec<Vec<&str>> = cells(&text);
    let kinds: Vec<&str> = kinds.iter().map(|row| row[0]).collect();
    assert_eq!(kinds, ["moment", "moment", "cosmic-event", "future-era"]);
}

#[test]
fn a_value_the_crate_refuses_is_a_sentinel() {
    assert_eq!(
        unsafe {
            hc_place_years_ago(
                f64::NAN,
                0.0,
                core::ptr::null(),
                0,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_place_years_ago(1.0, -1.0, core::ptr::null(), 0, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
    let not_utf8 = [0xff_u8];
    assert_eq!(
        unsafe { hc_place_years_ago(0.0, 0.0, not_utf8.as_ptr(), 1, core::ptr::null_mut(), 0) },
        HC_ERR_NOT_UTF8
    );
}

#[test]
fn the_cosmic_tables_are_listed_with_their_sources() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_cosmic_events("ja".as_ptr(), 2, buffer, capacity)
    });
    let rows = cells(&text);
    let epochs = rows.iter().filter(|row| row[0] == "cosmic-epoch").count();
    let events = rows.iter().filter(|row| row[0] == "cosmic-event").count();
    assert_eq!(epochs, hc::hc_deep_time::universe::EPOCHS.len());
    assert_eq!(events, hc::hc_deep_time::universe::EVENTS.len());
    assert!(
        rows.iter().all(|row| !row[14].is_empty()),
        "every row cites"
    );
    assert_eq!(rows.len(), epochs + events);
    // Named in Japanese where an established term was read.
    let named = |id: &str| rows.iter().find(|row| row[1] == id).map(|row| row[15]);
    assert_eq!(named("planck-epoch"), Some("プランク時代"));
    assert_eq!(named("era-of-galaxies"), Some(""));
    // A minimum age has no older bound and a stated sigma.
    let planck = &rows[0];
    assert_eq!(planck[2], "Planck epoch");
    assert_eq!(planck[4], "0");
    // Values are written in plain decimal notation, however small.
    let end: f64 = planck[8].parse().expect("a number");
    assert!(end > 0.0 && end < 1e-40, "{planck:?}");
    assert!(!planck[8].contains('e'), "{planck:?}");
}

#[test]
fn the_earliest_evidence_the_periods_and_the_future_have_lists_of_their_own() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_earliest_evidence("ja".as_ptr(), 2, buffer, capacity)
    });
    let rows = cells(&text);
    assert_eq!(rows.len(), hc::hc_deep_time::evidence::EVIDENCE.len());
    assert!(rows.iter().all(|row| row[0] == "earliest-evidence"));
    let named = |id: &str| rows.iter().find(|row| row[1] == id).map(|row| row[15]);
    assert_eq!(named("earliest-writing-uruk-iv"), Some("原楔形文字"));
    // A minimum age has no older bound and a stated sigma.
    let omo = rows
        .iter()
        .find(|row| row[1] == "earliest-homo-sapiens-omo-kibish")
        .expect("listed");
    assert_eq!(omo[3], "earliest-homo-sapiens");
    assert_eq!(omo[4..9], ["", "", "", "", "233000"]);
    assert_eq!(omo[9], "11000");
    let text = read_lines(|buffer, capacity| unsafe {
        hc_archaeological_periods(core::ptr::null(), 0, buffer, capacity)
    });
    let rows = cells(&text);
    assert_eq!(rows.len(), hc::hc_deep_time::archaeology::PERIODS.len());
    assert_eq!(
        rows[0][..3],
        ["archaeological", "modern-period", "Modern period"]
    );
    let text = read_lines(|buffer, capacity| unsafe {
        hc_future_events(core::ptr::null(), 0, buffer, capacity)
    });
    let rows = cells(&text);
    assert_eq!(rows.len(), hc::hc_deep_time::future::EVENTS.len());
    let proton = rows
        .iter()
        .find(|row| row[1] == "proton-decay-lower-bound")
        .expect("listed");
    assert_eq!(proton[3], "experimental-bound");
    assert_eq!(proton[8], "", "a bound has no end");
    assert_eq!(proton[12], "years-from-now");
    let not_utf8 = [0xff_u8];
    for export in [
        hc_earliest_evidence,
        hc_archaeological_periods,
        hc_future_events,
    ] {
        assert_eq!(
            unsafe { export(not_utf8.as_ptr(), 1, core::ptr::null_mut(), 0) },
            HC_ERR_NOT_UTF8
        );
    }
}

#[test]
fn the_geologic_ranks_are_numbered_coarsest_first() {
    for (number, expected) in [
        (0, "eon"),
        (1, "era"),
        (2, "period"),
        (3, "epoch"),
        (4, "age"),
    ] {
        let text = read_lines(|buffer, capacity| unsafe {
            hc_geologic_intervals(number, "zh-Hans".as_ptr(), 7, buffer, capacity)
        });
        let rows = cells(&text);
        assert!(
            rows.iter().all(|row| row[0] == expected),
            "{number}: {rows:?}"
        );
    }
    let eons = read_lines(|buffer, capacity| unsafe {
        hc_geologic_intervals(0, core::ptr::null(), 0, buffer, capacity)
    });
    assert!(
        eons.starts_with("eon\tphanerozoic\tPhanerozoic\t\t538.8\t0.6\t4\t0\t0\t0\t1\t0\t"),
        "{eons}"
    );
    assert!(cells(&eons).iter().all(|row| row[15].is_empty()));
    let eons = read_lines(|buffer, capacity| unsafe {
        hc_geologic_intervals(0, "zh-Hans".as_ptr(), 7, buffer, capacity)
    });
    assert_eq!(cells(&eons)[0][15], "显生宇");
    assert_eq!(
        unsafe { hc_geologic_intervals(5, core::ptr::null(), 0, core::ptr::null_mut(), 0) },
        HC_ERR_UNKNOWN
    );
}

#[test]
fn hc_planck_units_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe { hc_planck_units(buffer, capacity) });
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
    let text = read_lines(|buffer, capacity| unsafe {
        hc_bp_convert(
            11650.0,
            5.0,
            "bp".as_ptr(),
            "bp".len(),
            "b2k".as_ptr(),
            "b2k".len(),
            buffer,
            capacity,
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
                "radiocarbon-bp".as_ptr(),
                "radiocarbon-bp".len(),
                "bp".as_ptr(),
                "bp".len(),
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_NO_DATA
    );
    assert_eq!(
        unsafe {
            hc_bp_convert(
                1.0,
                0.0,
                "bp".as_ptr(),
                "bp".len(),
                "ad".as_ptr(),
                "ad".len(),
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
}

#[test]
fn hc_deep_convert_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_deep_convert(
            13.787,
            0.02,
            "gigayear".as_ptr(),
            "gigayear".len(),
            "second".as_ptr(),
            "second".len(),
            buffer,
            capacity,
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
                "gigayear".as_ptr(),
                "gigayear".len(),
                "furlong".as_ptr(),
                "furlong".len(),
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_deep_convert(
                1.0,
                -1.0,
                "gigayear".as_ptr(),
                "gigayear".len(),
                "second".as_ptr(),
                "second".len(),
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_OUT_OF_RANGE
    );
}

#[test]
fn hc_deep_compare_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_deep_compare(
            13.787,
            0.02,
            "gigayear".as_ptr(),
            "gigayear".len(),
            1.0,
            0.0,
            "planck-time".as_ptr(),
            "planck-time".len(),
            buffer,
            capacity,
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
                "day".as_ptr(),
                "day".len(),
                1.0,
                0.0,
                "day".as_ptr(),
                "day".len(),
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe {
            hc_deep_compare(
                1.0,
                0.0,
                "day".as_ptr(),
                "day".len(),
                1.0,
                0.0,
                "fortnight".as_ptr(),
                "fortnight".len(),
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
}
