//! The tab-separated lines the WebAssembly module and the C library write
//! about Earth's orbit, written once.
//!
//! Everything here reads [`hc_orbital`]: Berger's 1978 series for the
//! orbital elements, a million years either side of 1950, and the daily
//! mean insolation at 65° N at the June solstice computed from them. Each
//! line names the series and the solar constant it was computed with, by
//! the name the crate gives it, so a figure can be traced to its source.

use alloc::string::String;

use hc_core::math::floor;
use hc_orbital::{
    LATITUDE_65N, MID_JUNE_SOLAR_LONGITUDE, SOLAR_CONSTANT_BERGER_LOUTRE_1991, SOURCE, VALID_SPAN,
    daily_insolation, elements_at,
};

use crate::boundary::{Answer, Line, Refusal};

/// How many columns a line of [`orbit_line`] has.
pub const ORBIT_COLUMNS: usize = 11;

/// The solar constant every insolation cell is computed with: the
/// 1360 W m⁻² of Berger & Loutre's 1991 tables, `hc-orbital`'s
/// `SOLAR_CONSTANT_BERGER_LOUTRE_1991`, so a figure here compares with the
/// NOAA `orbit91` table exactly. The value is written in its own column
/// and the constant's name in the source; a caller who wants another
/// value — 1361 W m⁻² for a modern paper — scales the cell, since the
/// insolation is proportional to the constant.
pub const SOLAR_CONSTANT: f64 = SOLAR_CONSTANT_BERGER_LOUTRE_1991;

/// The name the source cell gives the constant.
const SOLAR_CONSTANT_NAME: &str = "SOLAR_CONSTANT_BERGER_LOUTRE_1991";

/// The most samples one [`series_lines`] writes.
///
/// Ten thousand lines are about five megabytes of text, most of it the
/// source cell repeated, and some forty milliseconds; a page draws far
/// fewer columns than that, and a caller that wants more asks in pieces.
pub const MAX_SERIES_SAMPLES: usize = 10_000;

/// Append the eleven cells of one epoch; see [`orbit_line`] for the
/// columns.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for an epoch the crate refuses: not finite, or
/// beyond a million years either side of 1950.
fn push_epoch(line: &mut Line<'_>, years_before_present: f64) -> Answer<()> {
    let elements = elements_at(years_before_present).map_err(|_| Refusal::OutOfRange)?;
    let insolation = daily_insolation(
        &elements,
        LATITUDE_65N,
        MID_JUNE_SOLAR_LONGITUDE,
        SOLAR_CONSTANT,
    )
    .map_err(|_| Refusal::OutOfRange)?;
    line.value(elements.eccentricity.value)
        .value(elements.eccentricity.std_dev)
        .value(elements.obliquity_degrees.value)
        .value(elements.obliquity_degrees.std_dev)
        .value(elements.longitude_of_perihelion_degrees.value)
        .value(elements.longitude_of_perihelion_degrees.std_dev)
        .value(elements.climatic_precession.value)
        .value(elements.climatic_precession.std_dev)
        .value(insolation)
        .value(SOLAR_CONSTANT)
        .value(format_args!(
            "{SOURCE}; insolation: Berger (1978) daily mean at 65N for solar longitude 90, \
             solar constant {SOLAR_CONSTANT_NAME}"
        ));
    Ok(())
}

/// The line of `hc_orbit_at`: Earth's orbital elements and the June
/// insolation at 65° N at an epoch in years before 1950, negative for the
/// future. Tab-separated: the eccentricity and its spread, the obliquity in
/// degrees and its spread, the longitude of perihelion from the moving
/// equinox in degrees and its spread, the climatic precession *e* sin ϖ and
/// its spread, the insolation in W m⁻², the solar constant it was computed
/// with, [`SOLAR_CONSTANT`], and the source.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for an epoch the crate refuses: not finite, or
/// beyond a million years either side of 1950.
pub fn orbit_line(years_before_present: f64) -> Answer<String> {
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    push_epoch(&mut line, years_before_present)?;
    line.end();
    Ok(out)
}

/// The lines of `hc_orbit_series`: one per sample from `from` to `to` in
/// steps of `step`, each the epoch and then [`orbit_line`]'s columns. A
/// `to` before `from` is no line at all.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a step that is not finite and positive, an
/// end outside the span, or more than [`MAX_SERIES_SAMPLES`] samples.
pub fn series_lines(from: f64, to: f64, step: f64) -> Answer<String> {
    if !(from.is_finite() && to.is_finite() && step.is_finite()) || step <= 0.0 {
        return Err(Refusal::OutOfRange);
    }
    if !(VALID_SPAN.contains(&from) && VALID_SPAN.contains(&to)) {
        return Err(Refusal::OutOfRange);
    }
    let mut out = String::new();
    if to < from {
        return Ok(out);
    }
    // A step too small for the span divides to a count the cast saturates,
    // which the cap then refuses; the cap is checked before the first
    // sample is counted so that nothing overflows.
    let intervals = floor((to - from) / step) as usize;
    if intervals >= MAX_SERIES_SAMPLES {
        return Err(Refusal::OutOfRange);
    }
    for sample in 0..=intervals {
        // Each sample is computed from the start rather than accumulated,
        // so the error does not grow along the series, and the last one is
        // held to `to` against rounding.
        let epoch = (from + sample as f64 * step).min(to);
        let mut line = Line::new(&mut out);
        line.value(epoch);
        push_epoch(&mut line, epoch)?;
        line.end();
    }
    Ok(out)
}

/// How many columns a line of [`insolation_line`] has.
pub const INSOLATION_COLUMNS: usize = 6;

/// The daily mean insolation at any latitude and solar longitude, for the
/// orbit of an epoch in years before 1950, as one line: the epoch, the
/// latitude in degrees (north positive), the Sun's true longitude in
/// degrees, the insolation in W m⁻² (0 in the polar night), the solar
/// constant it was computed with, [`SOLAR_CONSTANT`], and the source.
///
/// The longitude is the Sun's true longitude on the ecliptic: 0 at the March
/// equinox, 90 at the June solstice, 180 at the September equinox, 270 at
/// the December solstice. It is not a date: Berger's program turns a date
/// into a longitude with a 365-day year, which differs from a calendar's by
/// up to a day, and the honest input is the longitude. `hc_orbit_at` is the
/// case of 65° N at 90°.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for an epoch the crate refuses (not finite, or
/// beyond a million years either side of 1950), a latitude that is not
/// finite or is beyond ±90°, and a longitude that is not finite or is
/// outside 0 to 360.
pub fn insolation_line(
    years_before_present: f64,
    latitude_degrees: f64,
    solar_longitude_degrees: f64,
) -> Answer<String> {
    if !(0.0..=360.0).contains(&solar_longitude_degrees) {
        return Err(Refusal::OutOfRange);
    }
    let elements = elements_at(years_before_present).map_err(|_| Refusal::OutOfRange)?;
    let insolation = daily_insolation(
        &elements,
        latitude_degrees,
        solar_longitude_degrees,
        SOLAR_CONSTANT,
    )
    .map_err(|_| Refusal::OutOfRange)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(years_before_present)
        .value(latitude_degrees)
        .value(solar_longitude_degrees)
        .value(insolation)
        .value(SOLAR_CONSTANT)
        .value(format_args!(
            "{SOURCE}; insolation: Berger (1978) daily mean for the Sun's true longitude, \
             solar constant {SOLAR_CONSTANT_NAME}"
        ));
    line.end();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::cells;

    #[test]
    fn a_line_has_its_columns_and_names_its_constant() {
        let line = orbit_line(0.0).expect("1950");
        let cells = cells(&line);
        assert_eq!(cells.len(), ORBIT_COLUMNS);
        assert_eq!(cells[9], "1360");
        assert!(cells[10].contains(SOLAR_CONSTANT_NAME), "{line}");
        assert_eq!(orbit_line(f64::NAN), Err(Refusal::OutOfRange));
        assert_eq!(orbit_line(1_000_001.0), Err(Refusal::OutOfRange));
    }

    #[test]
    fn a_series_is_its_epochs_and_refuses_too_many() {
        let text = series_lines(0.0, 2_000.0, 1_000.0).expect("in range");
        let epochs: alloc::vec::Vec<&str> = text.lines().map(|line| cells(line)[0]).collect();
        assert_eq!(epochs, ["0", "1000", "2000"]);
        assert_eq!(series_lines(1.0, 0.0, 1.0), Ok(String::new()));
        assert_eq!(series_lines(0.0, 1.0, 0.0), Err(Refusal::OutOfRange));
        assert_eq!(
            series_lines(0.0, MAX_SERIES_SAMPLES as f64, 1.0),
            Err(Refusal::OutOfRange)
        );
    }

    #[test]
    fn the_insolation_of_a_latitude_follows_berger_s_formula() {
        let at = |epoch, latitude, longitude| {
            let text = insolation_line(epoch, latitude, longitude).expect("in range");
            let row = cells(&text);
            assert_eq!(row.len(), INSOLATION_COLUMNS);
            row[3].parse::<f64>().expect("a number")
        };
        // 65 N at the June solstice is `hc_orbit_at`'s own column, 477.6 W/m2
        // at 1950 (hc-orbital's anchor against NOAA's orbit91 table).
        let orbit = orbit_line(0.0).expect("1950");
        let june: f64 = cells(&orbit)[8].parse().expect("a number");
        assert!((at(0.0, 65.0, 90.0) - june).abs() < 1e-9);
        assert!((june - 477.6).abs() < 0.1, "{june}");
        // At the equator on an equinox the day is 12 hours and the Sun is
        // overhead at noon, so the daily mean is S0/(pi rho^2) with the
        // Earth-Sun distance rho = (1 - e^2) / (1 + e cos(lambda - varpi -
        // 180)): derived here from the elements the orbit line writes.
        let cells_of = |epoch: f64| {
            let text = orbit_line(epoch).expect("in range");
            cells(&text)
                .iter()
                .map(|cell| cell.to_string())
                .collect::<alloc::vec::Vec<_>>()
        };
        let elements = cells_of(0.0);
        let eccentricity: f64 = elements[0].parse().expect("e");
        let perihelion: f64 = elements[4].parse().expect("varpi");
        for longitude in [0.0, 180.0] {
            let anomaly = (longitude - (perihelion + 180.0)).to_radians();
            let distance =
                (1.0 - eccentricity * eccentricity) / (1.0 + eccentricity * anomaly.cos());
            let expected = SOLAR_CONSTANT / (core::f64::consts::PI * distance * distance);
            let got = at(0.0, 0.0, longitude);
            assert!(
                (got - expected).abs() < 1e-9 * expected,
                "{longitude}: {got} against {expected}"
            );
        }
        // Polar night: no sunlight at 80 N at the December solstice; polar
        // day at the June one, S pi sin(phi) sin(delta) with sin(delta) =
        // sin(epsilon).
        assert_eq!(at(0.0, 80.0, 270.0), 0.0);
        let obliquity: f64 = elements[2].parse().expect("epsilon");
        let anomaly = (90.0 - (perihelion + 180.0)).to_radians();
        let distance = (1.0 - eccentricity * eccentricity) / (1.0 + eccentricity * anomaly.cos());
        let expected = SOLAR_CONSTANT / core::f64::consts::PI / (distance * distance)
            * core::f64::consts::PI
            * 80.0_f64.to_radians().sin()
            * obliquity.to_radians().sin();
        assert!((at(0.0, 80.0, 90.0) - expected).abs() < 1e-9 * expected);
        // The two hemispheres swap at the solstices.
        assert!(at(0.0, -65.0, 270.0) > 477.6);
        assert_eq!(insolation_line(0.0, 91.0, 90.0), Err(Refusal::OutOfRange));
        assert_eq!(
            insolation_line(0.0, f64::NAN, 90.0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(insolation_line(0.0, 65.0, 361.0), Err(Refusal::OutOfRange));
        assert_eq!(insolation_line(0.0, 65.0, -1.0), Err(Refusal::OutOfRange));
        assert_eq!(
            insolation_line(1_000_001.0, 65.0, 90.0),
            Err(Refusal::OutOfRange)
        );
    }
}
