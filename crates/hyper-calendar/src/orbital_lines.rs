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
}
