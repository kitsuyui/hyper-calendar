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
    assert!(marked[0].starts_with("day\t8\tVidhi\t"));
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
    assert_eq!(text.split('\t').count(), 8);
    let unknown = unsafe { hc_drekkana_at(0, "x".as_ptr(), 1, core::ptr::null_mut(), 0) };
    assert_eq!(unknown, HC_ERR_UNKNOWN);
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
fn the_tithis_and_the_ayanamsas_cross_the_boundary() {
    let sky = "lahiri";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_tithi_at(1_736_683_200, sky.as_ptr(), sky.len(), buffer, capacity)
    });
    let cells: Vec<&str> = text.trim_end().split('\t').collect();
    assert_eq!(cells.len(), 8);
    assert_eq!(cells[..4], ["14", "shukla", "14", "Caturdaśī"]);
    let ends: i64 = cells[5].parse().expect("an instant");
    assert!((ends - (1_736_640_000 + 23 * 3_600 + 33 * 60)).abs() <= 120);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_tithis_of_day(
            739_264,
            TOKYO.0,
            TOKYO.1,
            0.0,
            sky.as_ptr(),
            sky.len(),
            buffer,
            capacity,
        )
    });
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert_eq!(rows[0][..4], ["14", "shukla", "14", "Caturdaśī"]);
    assert_eq!(rows[0][8..], ["1", "0", "0"]);
    assert_eq!(rows[1][3], "Pūrṇimā");
    let siddhanta = "surya-siddhanta";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_tithi_at(
            1_700_000_000,
            siddhanta.as_ptr(),
            siddhanta.len(),
            buffer,
            capacity,
        )
    });
    assert!(text.ends_with("\tsurya-siddhanta\n"));
    let text = read_lines(|buffer, capacity| unsafe { hc_ayanamsas(buffer, capacity) });
    assert!(text.lines().any(|line| {
        line.starts_with("lahiri-drik\tLahiri (Drik Panchang)\t2451544.5\t23.863776\t")
    }));
    let drik = "lahiri-drik";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_ayanamsa_at(1_735_689_600, drik.as_ptr(), drik.len(), buffer, capacity)
    });
    let degrees: f64 = text
        .split('\t')
        .next()
        .expect("degrees")
        .parse()
        .expect("a number");
    assert!((degrees - 24.213_067).abs() < 3e-4, "{degrees}");
    let text = read_lines(|buffer, capacity| unsafe {
        hc_ayanamsa_from_anchor(1_735_689_600, 2_460_676.5, 24.213_067, buffer, capacity)
    });
    let degrees: f64 = text
        .split('\t')
        .next()
        .expect("degrees")
        .parse()
        .expect("a number");
    assert!((degrees - 24.213_067).abs() < 1e-5, "{degrees}");
    assert!(text.contains("\tcustom\tcustom\t"));
    let unknown = unsafe { hc_ayanamsa_at(0, "mars".as_ptr(), 4, core::ptr::null_mut(), 0) };
    assert_eq!(unknown, HC_ERR_UNKNOWN);
    let far = unsafe {
        hc_tithi_at(
            200_000_000_000,
            sky.as_ptr(),
            sky.len(),
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(far, HC_ERR_OUT_OF_RANGE);
    let nan = unsafe { hc_ayanamsa_from_anchor(0, f64::NAN, 1.0, core::ptr::null_mut(), 0) };
    assert_eq!(nan, HC_ERR_OUT_OF_RANGE);
}
