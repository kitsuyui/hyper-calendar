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

const TOKYO: (f64, f64) = (
    35.0 + 41.0 / 60.0 + 22.0 / 3_600.0,
    139.0 + 41.0 / 60.0 + 30.0 / 3_600.0,
);

/// Drik Panchang's day page for Tokyo of 13 January 2025
/// (`drik-day-panchang-tokyo-2025`): "Chaturdashi upto 08:33 AM", the
/// bright fortnight's fourteenth, ending at 23:33 UT on the 12th, with
/// Purnima next; and its Lahiri ayanāṃśa of 24.213067 on 1 January 2025
/// (`drik-day-panchang-ayanamsha`).
#[test]
fn the_tithis_and_the_ayanamsas_cross_the_c_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_tithi_at(1_736_683_200, c"lahiri".as_ptr(), buffer, capacity, written)
    });
    let cells: Vec<&str> = text.trim_end().split('\t').collect();
    assert_eq!(cells.len(), 8);
    assert_eq!(cells[..4], ["14", "shukla", "14", "Caturdaśī"]);
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_tithis_of_day(
            739_264,
            TOKYO.0,
            TOKYO.1,
            0.0,
            c"true".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert_eq!(rows[0][8..], ["1", "0", "0"]);
    assert_eq!(rows[1][3], "Pūrṇimā");
    let text =
        read_lines(|buffer, capacity, written| unsafe { hc_ayanamsas(buffer, capacity, written) });
    assert!(text.lines().any(|line| {
        line.starts_with("lahiri-drik\tLahiri (Drik Panchang)\t2451544.5\t23.863776\t")
    }));
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_ayanamsa_at(
            1_735_689_600,
            c"lahiri-drik".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    let degrees: f64 = text
        .split('\t')
        .next()
        .expect("degrees")
        .parse()
        .expect("a number");
    assert!((degrees - 24.213_067).abs() < 3e-4, "{degrees}");
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_ayanamsa_from_anchor(
            1_735_689_600,
            2_460_676.5,
            24.213_067,
            buffer,
            capacity,
            written,
        )
    });
    assert!(text.contains("\tcustom\tcustom\t"));
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_ayanamsa_at(0, c"mars".as_ptr(), buffer, capacity, written)
        }),
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_tithi_at(
                200_000_000_000,
                c"lahiri".as_ptr(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_ayanamsa_from_anchor(0, f64::NAN, 1.0, buffer, capacity, written)
        }),
        HC_ERROR_OUT_OF_RANGE
    );
}
