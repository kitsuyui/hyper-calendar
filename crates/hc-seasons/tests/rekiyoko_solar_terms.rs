//! The 24 solar terms of 2024, 2025 and 2026 against the 暦要項.
//!
//! The National Astronomical Observatory of Japan publishes the instant of
//! every term to the minute in Japan Standard Time, in the 暦要項 in the
//! *Official Gazette* each February, one year ahead. The three years below
//! are the ones the system document
//! `docs/systems/solar-terms-and-pentads.md` measures the crate against,
//! read from
//!
//! * <https://eco.mtk.nao.ac.jp/koyomi/yoko/2024/rekiyou242.html>,
//! * <https://eco.mtk.nao.ac.jp/koyomi/yoko/2025/rekiyou252.html>,
//! * <https://eco.mtk.nao.ac.jp/koyomi/yoko/2026/rekiyou262.html>,
//!
//! all retrieved 2026-09-25.
//!
//! # What the comparison measures
//!
//! The published minute is the instant rounded to the nearest minute, so a
//! computed instant within thirty seconds of it is on the published minute,
//! and anything the crate gets wrong shows up as a bias in the residuals:
//! the Sun's longitude from VSOP87 is good to about 1″, 24 s of the Sun's
//! motion, and ΔT moves every Universal Time instant by its whole error.
//! That is what these tests report, and what the document's Accuracy
//! section quotes.
//!
//! The observed ΔT table `hc-astro` carries ends on 2026-04-01, the last
//! month the USNO had observed when it was taken; the terms after it are
//! computed from the Espenak–Meeus polynomial and are expected to run about
//! six seconds early. The tests count the two groups separately.

use hc_astro::time::{TABULATED_DELTA_T_LAST, decimal_year, delta_t, delta_t_polynomial};
use hc_calendar::fixed::Moment;
use hc_seasons::Meridian;
use hc_seasons::solar_terms::{SolarTerm, term_moment};

/// One published term: the Sun's longitude in degrees, then the JST month,
/// day, hour and minute the 暦要項 prints.
type Published = (u16, u8, u8, u8, u8);

/// 令和6年(2024)暦要項, 二十四節気, in the order printed, from 小寒.
const TERMS_2024: [Published; 24] = [
    (285, 1, 6, 5, 49),
    (300, 1, 20, 23, 7),
    (315, 2, 4, 17, 27),
    (330, 2, 19, 13, 13),
    (345, 3, 5, 11, 23),
    (0, 3, 20, 12, 6),
    (15, 4, 4, 16, 2),
    (30, 4, 19, 23, 0),
    (45, 5, 5, 9, 10),
    (60, 5, 20, 22, 0),
    (75, 6, 5, 13, 10),
    (90, 6, 21, 5, 51),
    (105, 7, 6, 23, 20),
    (120, 7, 22, 16, 44),
    (135, 8, 7, 9, 9),
    (150, 8, 22, 23, 55),
    (165, 9, 7, 12, 11),
    (180, 9, 22, 21, 44),
    (195, 10, 8, 4, 0),
    (210, 10, 23, 7, 15),
    (225, 11, 7, 7, 20),
    (240, 11, 22, 4, 56),
    (255, 12, 7, 0, 17),
    (270, 12, 21, 18, 21),
];

/// 令和7年(2025)暦要項.
const TERMS_2025: [Published; 24] = [
    (285, 1, 5, 11, 33),
    (300, 1, 20, 5, 0),
    (315, 2, 3, 23, 10),
    (330, 2, 18, 19, 7),
    (345, 3, 5, 17, 7),
    (0, 3, 20, 18, 1),
    (15, 4, 4, 21, 49),
    (30, 4, 20, 4, 56),
    (45, 5, 5, 14, 57),
    (60, 5, 21, 3, 55),
    (75, 6, 5, 18, 57),
    (90, 6, 21, 11, 42),
    (105, 7, 7, 5, 5),
    (120, 7, 22, 22, 29),
    (135, 8, 7, 14, 52),
    (150, 8, 23, 5, 34),
    (165, 9, 7, 17, 52),
    (180, 9, 23, 3, 19),
    (195, 10, 8, 9, 41),
    (210, 10, 23, 12, 51),
    (225, 11, 7, 13, 4),
    (240, 11, 22, 10, 36),
    (255, 12, 7, 6, 5),
    (270, 12, 22, 0, 3),
];

/// 令和8年(2026)暦要項.
const TERMS_2026: [Published; 24] = [
    (285, 1, 5, 17, 23),
    (300, 1, 20, 10, 45),
    (315, 2, 4, 5, 2),
    (330, 2, 19, 0, 52),
    (345, 3, 5, 22, 59),
    (0, 3, 20, 23, 46),
    (15, 4, 5, 3, 40),
    (30, 4, 20, 10, 39),
    (45, 5, 5, 20, 49),
    (60, 5, 21, 9, 37),
    (75, 6, 6, 0, 48),
    (90, 6, 21, 17, 25),
    (105, 7, 7, 10, 57),
    (120, 7, 23, 4, 13),
    (135, 8, 7, 20, 43),
    (150, 8, 23, 11, 19),
    (165, 9, 7, 23, 41),
    (180, 9, 23, 9, 5),
    (195, 10, 8, 15, 29),
    (210, 10, 23, 18, 38),
    (225, 11, 7, 18, 52),
    (240, 11, 22, 16, 23),
    (255, 12, 7, 11, 53),
    (270, 12, 22, 5, 50),
];

const YEARS: [(i64, &[Published; 24]); 3] = [
    (2024, &TERMS_2024),
    (2025, &TERMS_2025),
    (2026, &TERMS_2026),
];

/// The term at a longitude, by the crate's own numbering.
fn term_at(longitude: u16) -> SolarTerm {
    SolarTerm::all(hc_seasons::TermOrder::SpringEquinoxFirst)
        .into_iter()
        .find(|term| term.solar_longitude_degrees() == f64::from(longitude))
        .unwrap_or_else(|| panic!("no term at {longitude}°"))
}

/// One computed term against its published minute.
struct Residual {
    year: i64,
    term: SolarTerm,
    /// The computed instant in Universal Time.
    moment: Moment,
    /// Computed less published, in seconds, as the crate answers now.
    seconds: f64,
    /// The same with ΔT from the polynomial alone.
    polynomial_seconds: f64,
    /// Whether the instant is inside the observed ΔT table.
    observed: bool,
}

fn residuals() -> Vec<Residual> {
    let mut out = Vec::new();
    for (year, terms) in YEARS {
        for &(longitude, month, day, hour, minute) in terms {
            let term = term_at(longitude);
            let moment = term_moment(year, term);
            let published_day = hc_calendar::gregorian::to_fixed(year, month, day)
                .unwrap_or_else(|_| panic!("{year}-{month}-{day} is not a date"));
            assert_eq!(
                Meridian::JAPAN.day_of(moment),
                published_day,
                "{} of {year} is not on the published day",
                term.japanese_name()
            );
            let published =
                published_day.0 as f64 + (f64::from(hour) + f64::from(minute) / 60.0) / 24.0;
            let local = moment.0 + 9.0 / 24.0;
            let seconds = (local - published) * 86_400.0;
            let year_of = decimal_year(moment);
            // UT = TT − ΔT, and the TT instant does not depend on ΔT, so a
            // larger ΔT is an earlier UT instant by the same amount.
            let polynomial_seconds = seconds - (delta_t_polynomial(year_of) - delta_t(moment));
            out.push(Residual {
                year,
                term,
                moment,
                seconds,
                polynomial_seconds,
                observed: year_of <= TABULATED_DELTA_T_LAST,
            });
        }
    }
    out
}

/// Whether a residual in seconds rounds to the published minute.
fn on_the_minute(seconds: f64) -> bool {
    seconds.abs() <= 30.0
}

fn summary(label: &str, residuals: &[f64]) {
    let n = residuals.len();
    let mean = residuals.iter().sum::<f64>() / n as f64;
    let min = residuals.iter().copied().fold(f64::INFINITY, f64::min);
    let max = residuals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let matched = residuals.iter().filter(|&&s| on_the_minute(s)).count();
    println!(
        "{label}: {n} terms, {matched} on the published minute, residual {min:+.0} s to {max:+.0} s, mean {mean:+.1} s"
    );
}

#[test]
fn every_term_of_2024_to_2026_falls_on_the_published_day() {
    // The assertion is inside `residuals`; this test names it, and prints
    // the worked example of the system document, 大雪 of 2024.
    let all = residuals();
    assert_eq!(all.len(), 72);
    let heavy_snow = all
        .iter()
        .find(|r| r.year == 2024 && r.term.solar_longitude_degrees() == 255.0)
        .unwrap();
    let day = heavy_snow.moment.day();
    let seconds = (heavy_snow.moment.0 - day.0 as f64) * 86_400.0;
    println!(
        "大雪 2024: RD {} at {:02}:{:02}:{:02} UT, {:+.1} s from the published minute",
        day.0,
        (seconds / 3600.0).floor(),
        ((seconds % 3600.0) / 60.0).floor(),
        (seconds % 60.0).floor(),
        heavy_snow.seconds
    );
}

/// Inside the observed ΔT table the residual against the published minute
/// has no bias left beyond the rounding of the minute and the series'
/// own error: the mean is within two seconds of zero. The tests print the
/// rate rather than asserting every term on its minute, and a computed
/// instant that misses the minute has to sit at the half-minute, where the
/// rounding decides.
#[test]
fn inside_the_observed_table_the_terms_are_on_the_published_minute() {
    let all = residuals();
    let inside: Vec<f64> = all
        .iter()
        .filter(|r| r.observed)
        .map(|r| r.seconds)
        .collect();
    let polynomial: Vec<f64> = all
        .iter()
        .filter(|r| r.observed)
        .map(|r| r.polynomial_seconds)
        .collect();
    summary("observed ΔT, 2024-01-01 to 2026-04-01", &inside);
    summary("the polynomial alone, same terms", &polynomial);
    for r in all
        .iter()
        .filter(|r| r.observed && !on_the_minute(r.seconds))
    {
        println!(
            "  {} of {} is {:+.1} s from the published minute (UT {:.5})",
            r.term.japanese_name(),
            r.year,
            r.seconds,
            r.moment.0
        );
        assert!(
            (30.0..32.0).contains(&r.seconds.abs()),
            "{} of {} is {:+.1} s from the published minute",
            r.term.japanese_name(),
            r.year,
            r.seconds
        );
    }
    let mean = inside.iter().sum::<f64>() / inside.len() as f64;
    assert!(mean.abs() < 2.0, "a bias of {mean:+.1} s");
    let polynomial_mean = polynomial.iter().sum::<f64>() / polynomial.len() as f64;
    assert!(
        (polynomial_mean + 4.5).abs() < 1.0,
        "the polynomial alone would leave a bias of {polynomial_mean:+.1} s"
    );
}

/// Past the table's end the polynomial answers, about six seconds high, so
/// those terms run about six seconds early: still on the published day, and
/// on the published minute unless the true instant is within six seconds of
/// a half-minute.
#[test]
fn past_the_observed_table_the_terms_run_about_six_seconds_early() {
    let all = residuals();
    let outside: Vec<f64> = all
        .iter()
        .filter(|r| !r.observed)
        .map(|r| r.seconds)
        .collect();
    summary("polynomial ΔT, after 2026-04-01", &outside);
    assert!(
        !outside.is_empty(),
        "the table now covers all of 2026; retire this test"
    );
    let mean = outside.iter().sum::<f64>() / outside.len() as f64;
    assert!((mean + 6.0).abs() < 2.0, "a bias of {mean:+.1} s");
    for r in all.iter().filter(|r| !r.observed) {
        assert!(
            r.seconds.abs() < 60.0,
            "{} of {} is {:+.0} s from the published minute",
            r.term.japanese_name(),
            r.year,
            r.seconds
        );
    }
}
