use super::super::*;
use super::read_lines;

fn rows(text: &str) -> Vec<Vec<String>> {
    text.lines()
        .map(|line| line.split('\t').map(str::to_owned).collect())
        .collect()
}

fn number(cell: &str) -> f64 {
    cell.parse()
        .unwrap_or_else(|_| panic!("not a number: {cell}"))
}

/// The Last Glacial Maximum, 21 000 years before 1950, as the
/// author's own table and the PMIP experiments print it, and the
/// same line at the head of a series.
#[test]
fn the_orbit_and_a_series_decode_column_by_column() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_orbit_at(21_000.0, buffer, capacity, written)
    });
    let lines = rows(&text);
    assert_eq!(lines.len(), 1, "{text:?}");
    let columns = &lines[0];
    assert_eq!(columns.len(), 11, "{columns:?}");
    assert!(
        (number(&columns[0]) - 0.018_994).abs() < 1e-6,
        "{columns:?}"
    );
    assert_eq!(columns[1], "0.002");
    assert!((number(&columns[2]) - 22.949).abs() < 1e-3, "{columns:?}");
    assert!((number(&columns[4]) - 114.42).abs() < 0.01, "{columns:?}");
    assert!((number(&columns[6]) - 0.017_29).abs() < 5e-6, "{columns:?}");
    assert_eq!(columns[9], "1360");
    assert!(
        columns[10].contains("SOLAR_CONSTANT_BERGER_LOUTRE_1991"),
        "{}",
        columns[10]
    );

    let series = read_lines(|buffer, capacity, written| unsafe {
        hc_orbit_series(21_000.0, 23_000.0, 1_000.0, buffer, capacity, written)
    });
    let series = rows(&series);
    assert_eq!(series.len(), 3, "{series:?}");
    assert!(series.iter().all(|line| line.len() == 12), "{series:?}");
    assert_eq!(series[0][0], "21000");
    assert_eq!(series[0][1..], columns[..]);
    assert_eq!(series[2][0], "23000");
}

#[test]
fn epochs_off_the_span_and_series_too_long_are_refused() {
    let null = core::ptr::null_mut();
    let mut written = 0usize;
    for epoch in [1_000_001.0, -1_000_001.0, f64::NAN] {
        assert_eq!(
            unsafe { hc_orbit_at(epoch, null, 0, &mut written) },
            HC_ERROR_OUT_OF_RANGE,
            "{epoch}"
        );
    }
    for (from, to, step) in [
        (0.0, 1_000_001.0, 100.0),
        (0.0, 1_000.0, 0.0),
        (0.0, 1_000.0, f64::NAN),
        (0.0, 10_000.0, 1.0),
    ] {
        assert_eq!(
            unsafe { hc_orbit_series(from, to, step, null, 0, &mut written) },
            HC_ERROR_OUT_OF_RANGE,
            "{from} to {to} by {step}"
        );
    }
    // A `to` before `from` is an empty, terminated answer.
    let mut buffer = [7 as c_char; 4];
    assert_eq!(
        unsafe { hc_orbit_series(1_000.0, 0.0, 100.0, buffer.as_mut_ptr(), 4, &mut written) },
        HC_OK
    );
    assert_eq!(written, 1);
    assert_eq!(buffer[0], 0);
}
