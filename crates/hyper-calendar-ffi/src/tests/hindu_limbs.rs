//! The muhūrtas, amṛta siddhi, the nakṣatra, the drekkāṇa and the
//! Siddhānta's pañcāṅga through the C boundary.

use super::super::*;
use super::{measured, read_lines};

const DELHI: (f64, f64) = (
    28.0 + 38.0 / 60.0 + 8.0 / 3_600.0,
    77.0 + 13.0 / 60.0 + 28.0 / 3_600.0,
);

/// Drik Panchang's New Delhi page of Thursday 2 January 2025
/// (`drik-day-panchang-2025`): Abhijit the eighth muhūrta, Dur Muhurtam
/// the sixth and the twelfth.
#[test]
fn the_lines_are_the_modules() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_muhurtas(739_253, DELHI.0, DELHI.1, 0.0, buffer, capacity, written)
    });
    let marks: Vec<(&str, &str)> = text
        .lines()
        .map(|line| line.split('\t').collect::<Vec<_>>())
        .filter(|cells| !cells[5].is_empty())
        .map(|cells| (cells[1], cells[5]))
        .collect();
    assert_eq!(
        marks,
        [
            ("6", "dur-muhurtam"),
            ("8", "abhijit"),
            ("12", "dur-muhurtam")
        ]
    );
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_drekkana_at(1_700_000_000, c"lahiri".as_ptr(), buffer, capacity, written)
    });
    assert_eq!(text.split('\t').count(), 10);
    assert!(text.ends_with("\tlahiri\n"));
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_panchanga_of_day(
            691_000,
            18.52,
            73.87,
            0.0,
            c"surya-siddhanta".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(text.lines().count(), 2);
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_nakshatra_of_day(
            739_258,
            DELHI.0,
            DELHI.1,
            0.0,
            c"lahiri".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(text.split('\t').count(), 8);
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_amrita_siddhi(
                739_258,
                DELHI.0,
                DELHI.1,
                0.0,
                core::ptr::null(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_panchanga_at(
                i64::MIN,
                c"surya-siddhanta".as_ptr(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_OUT_OF_RANGE
    );
}
