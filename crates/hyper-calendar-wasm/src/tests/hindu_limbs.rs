//! The muhūrtas, amṛta siddhi, the nakṣatra and the Siddhānta's pañcāṅga
//! through the WebAssembly boundary.

use super::super::*;
use super::read_lines;

const DELHI: (f64, f64) = (
    28.0 + 38.0 / 60.0 + 8.0 / 3_600.0,
    77.0 + 13.0 / 60.0 + 28.0 / 3_600.0,
);

/// Drik Panchang's New Delhi page of Wednesday 1 January 2025
/// (`drik-day-panchang-2025`): Dur Muhurtam the eighth of the day, no
/// Abhijit.
#[test]
fn the_lines_are_the_facades() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_muhurtas(739_252, DELHI.0, DELHI.1, 0.0, buffer, capacity)
    });
    assert_eq!(text.lines().count(), 30);
    let marked: Vec<&str> = text
        .lines()
        .filter(|line| line.contains("\tdur-muhurtam\t") || line.contains("\tabhijit\t"))
        .collect();
    assert_eq!(marked.len(), 1);
    assert!(marked[0].starts_with("day\t8\t"));
    let lahiri = "lahiri";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_amrita_siddhi(
            739_258,
            DELHI.0,
            DELHI.1,
            0.0,
            lahiri.as_ptr(),
            lahiri.len(),
            buffer,
            capacity,
        )
    });
    assert!(text.starts_with("Amrita Siddhi Yoga\tअमृत सिद्धि योग\t1\t"));
    assert!(text.ends_with("\t1\tlahiri\n"));
    let sky = "surya-siddhanta";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_panchanga_at(1_700_000_000, sky.as_ptr(), sky.len(), buffer, capacity)
    });
    assert!(
        text.lines()
            .all(|line| line.ends_with("\tsurya-siddhanta\tSūrya Siddhānta"))
    );
    let text = read_lines(|buffer, capacity| unsafe {
        hc_nakshatra_at(
            1_700_000_000,
            lahiri.as_ptr(),
            lahiri.len(),
            buffer,
            capacity,
        )
    });
    assert_eq!(text.split('\t').count(), 6);
    let unknown = unsafe { hc_drekkana_at(0, "x".as_ptr(), 1, core::ptr::null_mut(), 0) };
    assert_eq!(unknown, HC_ERR_UNKNOWN);
}
