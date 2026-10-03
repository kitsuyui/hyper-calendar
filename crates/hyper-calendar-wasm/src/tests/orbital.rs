use super::super::*;
use super::read_lines;

fn row(text: &str) -> Vec<String> {
    text.trim_end().split('\t').map(str::to_owned).collect()
}

fn rows(text: &str) -> Vec<Vec<String>> {
    text.lines()
        .map(|line| line.split('\t').map(str::to_owned).collect())
        .collect()
}

fn orbit(years_before_1950: f64) -> Vec<String> {
    let text =
        read_lines(|buffer, capacity| unsafe { hc_orbit_at(years_before_1950, buffer, capacity) });
    let lines = rows(&text);
    assert_eq!(lines.len(), 1, "{text:?}");
    lines.into_iter().next().unwrap_or_default()
}

fn number(cell: &str) -> f64 {
    cell.parse()
        .unwrap_or_else(|_| panic!("not a number: {cell}"))
}

/// The author's own table (`bein1.dat`) and the PMIP experiments
/// put the Last Glacial Maximum, 21 000 years before 1950, at
/// e = 0.018994, ϖ = 114.42°, ε = 22.949°, e sin ϖ = 0.01729; the
/// spreads are the nearest tier's; and at 1950 the June insolation
/// at 65° N is 477.6 W m⁻² for 1360 W m⁻².
#[test]
fn the_last_glacial_maximum_decodes_column_by_column() {
    let columns = orbit(21_000.0);
    assert_eq!(columns.len(), 11, "{columns:?}");
    assert!(
        (number(&columns[0]) - 0.018_994).abs() < 1e-6,
        "{columns:?}"
    );
    assert_eq!(columns[1], "0.002");
    assert!((number(&columns[2]) - 22.949).abs() < 1e-3, "{columns:?}");
    assert_eq!(columns[3], "0.05");
    assert!((number(&columns[4]) - 114.42).abs() < 0.01, "{columns:?}");
    // asin(0.0025 / 0.018994), in degrees.
    assert!((number(&columns[5]) - 7.56).abs() < 0.01, "{columns:?}");
    assert!((number(&columns[6]) - 0.017_29).abs() < 5e-6, "{columns:?}");
    assert_eq!(columns[7], "0.0025");
    let lgm_june = number(&columns[8]);
    assert_eq!(columns[9], "1360");
    assert!(columns[10].contains("Berger, A. (1978)"), "{}", columns[10]);
    assert!(
        columns[10].contains("SOLAR_CONSTANT_BERGER_LOUTRE_1991"),
        "{}",
        columns[10]
    );
    assert!(!columns[10].contains('\t'));

    let now = orbit(0.0);
    assert!((number(&now[8]) - 477.6).abs() < 0.1, "{now:?}");
    assert!(lgm_june < number(&now[8]));
    let early_holocene = orbit(11_000.0);
    assert!(number(&early_holocene[8]) - number(&now[8]) > 45.0);
    // Numbers are written in plain decimal notation.
    assert!(
        now.iter().take(10).all(|cell| !cell.contains('e')),
        "{now:?}"
    );
}

#[test]
fn a_million_years_is_answered_and_a_year_more_is_refused() {
    let null = core::ptr::null_mut();
    assert!(unsafe { hc_orbit_at(1_000_000.0, null, 0) } > 0);
    assert!(unsafe { hc_orbit_at(-1_000_000.0, null, 0) } > 0);
    for epoch in [
        1_000_001.0,
        -1_000_001.0,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        assert_eq!(
            unsafe { hc_orbit_at(epoch, null, 0) },
            HC_ERR_OUT_OF_RANGE,
            "{epoch}"
        );
    }
}

#[test]
fn a_series_is_the_single_lines_with_the_epoch_first() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_orbit_series(0.0, 21_000.0, 1_000.0, buffer, capacity)
    });
    let lines = rows(&text);
    assert_eq!(lines.len(), 22, "{}", lines.len());
    assert!(lines.iter().all(|line| line.len() == 12), "{lines:?}");
    let epochs: Vec<f64> = lines.iter().map(|line| number(&line[0])).collect();
    let expected: Vec<f64> = (0..22).map(|kyr| f64::from(kyr) * 1_000.0).collect();
    assert_eq!(epochs, expected);
    assert_eq!(lines[0][1..], orbit(0.0)[..]);
    assert_eq!(lines[21][1..], orbit(21_000.0)[..]);
    // A step that does not divide the span stops before `to`.
    let text = read_lines(|buffer, capacity| unsafe {
        hc_orbit_series(0.0, 1_000.0, 300.0, buffer, capacity)
    });
    let epochs: Vec<String> = rows(&text).iter().map(|line| line[0].clone()).collect();
    assert_eq!(epochs, ["0", "300", "600", "900"]);
    // The whole span at the widest step that fits the cap.
    let text = read_lines(|buffer, capacity| unsafe {
        hc_orbit_series(-1_000_000.0, 1_000_000.0, 200.01, buffer, capacity)
    });
    let lines = rows(&text);
    assert_eq!(lines.len(), 10_000);
    assert!(number(&lines[9_999][0]) <= 1_000_000.0);
}

#[test]
fn a_series_that_is_empty_too_long_or_off_the_span_is_refused_or_empty() {
    let null = core::ptr::null_mut();
    // A `to` before `from` is an empty answer.
    assert_eq!(unsafe { hc_orbit_series(1_000.0, 0.0, 100.0, null, 0) }, 0);
    // One sample when they coincide.
    assert!(unsafe { hc_orbit_series(0.0, 0.0, 100.0, null, 0) } > 0);
    for (from, to, step) in [
        (0.0, 1_000_001.0, 100.0),
        (-1_000_001.0, 0.0, 100.0),
        (f64::NAN, 0.0, 100.0),
        (0.0, f64::INFINITY, 100.0),
        (0.0, 1_000.0, 0.0),
        (0.0, 1_000.0, -100.0),
        (0.0, 1_000.0, f64::NAN),
        (0.0, 1_000.0, f64::MIN_POSITIVE),
        // 10 001 samples.
        (0.0, 10_000.0, 1.0),
        (-1_000_000.0, 1_000_000.0, 200.0),
    ] {
        assert_eq!(
            unsafe { hc_orbit_series(from, to, step, null, 0) },
            HC_ERR_OUT_OF_RANGE,
            "{from} to {to} by {step}"
        );
    }
    // 10 000 samples is the most.
    assert!(unsafe { hc_orbit_series(0.0, 9_999.0, 1.0, null, 0) } > 0);
}

#[test]
fn hc_daily_insolation_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_daily_insolation(0.0, 65.0, 90.0, buffer, capacity)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 6),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[4], "1360");
    assert_eq!(
        unsafe { hc_daily_insolation(0.0, 91.0, 90.0, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_daily_insolation(0.0, 65.0, 361.0, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
}
