//! The tab-separated lines the WebAssembly module and the C library write
//! about relativistic time dilation, written once.
//!
//! Everything here reads [`hc_relativity`], whose metric is
//! Schwarzschild's: non-rotating, uncharged, spherically symmetric. The two
//! calculations are the two that crate anchors — a clock moving at a
//! constant velocity, and a clock held still at a radius from a mass — and
//! each line names the crate's constants it was computed with, by the
//! names the crate gives them, so a figure can be traced to its source.
//!
//! **The offsets are computed without cancellation.** A rate such as
//! `dτ/dt = 1 − 3·10⁻¹⁰` holds only six figures of its distance from 1
//! once written as a double, so the fractional offset is not `rate − 1`
//! but the same quantity by the identity `√(1 − x) − 1 = −x / (1 + √(1 −
//! x))`, with `x` the crate's `β²` or `r_s / r`.

use alloc::string::String;

use hc_core::Duration;
use hc_relativity::constants::{self, GRAVITATING_BODIES, GravitatingBody, SPEED_OF_LIGHT};
use hc_relativity::gravitational::{
    rate_offset_to_micros_per_day, schwarzschild_radius, static_dilation_factor,
};
use hc_relativity::special::{lorentz_factor_from_speed, proper_time_of};

use crate::boundary::{Answer, Line, Refusal};

/// How many columns a line of [`proper_time_line`] has.
pub const PROPER_TIME_COLUMNS: usize = 7;

/// How many columns a line of [`gravitational_dilation_line`] has.
pub const GRAVITATIONAL_COLUMNS: usize = 8;

/// How many columns a line of [`gravitating_bodies_lines`] has.
pub const GRAVITATING_BODY_COLUMNS: usize = 5;

/// What the last cell of a proper-time line names.
const SPECIAL_SOURCE: &str = "hc-relativity special::lorentz_factor_from_speed and \
     special::proper_time_of; SPEED_OF_LIGHT is exact by the 2019 SI";

/// A clock moving at a constant speed while `coordinate_seconds` pass in
/// the frame it moves through, as one line: `β`, the Lorentz factor `γ`,
/// the proper time the moving clock records in seconds, its rate `dτ/dt =
/// 1/γ`, the rate's offset from 1 in microseconds per 86 400-second day
/// (negative: the moving clock runs slow), the constants used, and the
/// source.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a speed that is not finite or is at or
/// beyond the speed of light either way, and for a coordinate time that
/// is not finite or whose proper time leaves the range of a duration.
pub fn proper_time_line(speed_metres_per_second: f64, coordinate_seconds: f64) -> Answer<String> {
    let gamma =
        lorentz_factor_from_speed(speed_metres_per_second).map_err(|_| Refusal::OutOfRange)?;
    let beta = speed_metres_per_second / SPEED_OF_LIGHT;
    let coordinate =
        Duration::from_secs_f64(coordinate_seconds).map_err(|_| Refusal::OutOfRange)?;
    let proper = proper_time_of(coordinate, beta).map_err(|_| Refusal::OutOfRange)?;
    let rate = 1.0 / gamma;
    let offset = -(beta * beta) / (1.0 + rate);
    let micros = rate_offset_to_micros_per_day(offset).map_err(|_| Refusal::OutOfRange)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(beta)
        .value(gamma)
        .value(proper.as_secs_f64())
        .value(rate)
        .value(micros)
        .cell("SPEED_OF_LIGHT")
        .cell(SPECIAL_SOURCE);
    line.end();
    Ok(out)
}

/// The body with the identifier `id`, by [`constants::by_id`]: `earth`,
/// `sagittarius-a-star` and the rest of [`gravitating_bodies_lines`]' first
/// column.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a body `hc-relativity` carries no `GM` for, and
/// for a name such as `Sagittarius A*`.
pub fn gravitating_body(id: &str) -> Answer<GravitatingBody> {
    constants::by_id(id).ok_or(Refusal::Unknown)
}

/// A clock held still at a radius from a body's centre, against one far
/// from it, as one line: the body's identifier, its `GM` in m³ s⁻², the
/// name of the `hc-relativity` constant that holds it, the Schwarzschild
/// radius `2GM/c²` in metres, the static dilation factor `dτ/dt =
/// √(1 − r_s/r)`, its offset from 1 in microseconds per 86 400-second day
/// (negative: the deeper clock runs slow), the constants used, separated
/// by `;`, and the body's source.
///
/// Two radii compare by dividing their factors: a GPS clock at
/// `GPS_ORBIT_RADIUS` over one at `EARTH_EQUATORIAL_RADIUS` gains the
/// +45.7 µs a day of the crate's anchor, before its motion is counted.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a body the crate carries no `GM` for, and
/// [`Refusal::OutOfRange`] for a radius that is not finite, not positive,
/// or at or inside the Schwarzschild radius, where no clock stands still.
pub fn gravitational_dilation_line(given: &str, radius_metres: f64) -> Answer<String> {
    let body = gravitating_body(given)?;
    let factor = static_dilation_factor(body.gm, radius_metres).map_err(|_| Refusal::OutOfRange)?;
    let horizon = schwarzschild_radius(body.gm).map_err(|_| Refusal::OutOfRange)?;
    let offset = -(horizon / radius_metres) / (1.0 + factor);
    let micros = rate_offset_to_micros_per_day(offset).map_err(|_| Refusal::OutOfRange)?;
    let name = body.gm_constant;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell(body.id)
        .value(body.gm)
        .cell(name)
        .value(horizon)
        .value(factor)
        .value(micros)
        .value(format_args!("{name};SPEED_OF_LIGHT_SQUARED"))
        .cell(body.source);
    line.end();
    Ok(out)
}

/// Every body `hc-relativity` carries a `GM` for, one line each: the
/// identifier, the English name, `GM` in m³ s⁻², the name of the constant
/// that holds it, and the source.
#[must_use]
pub fn gravitating_bodies_lines() -> String {
    let mut out = String::new();
    for body in GRAVITATING_BODIES {
        let mut line = Line::new(&mut out);
        line.cell(body.id)
            .cell(body.english_name)
            .value(body.gm)
            .cell(body.gm_constant)
            .cell(body.source);
        line.end();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_relativity::constants::{
        EARTH_EQUATORIAL_RADIUS, GM_EARTH, GM_JUPITER, GM_MARS, GM_MOON, GM_SAGITTARIUS_A_STAR,
        GM_SUN, GPS_ORBIT_RADIUS,
    };

    /// The value of the constant a body's `gm_constant` names, for the test
    /// that holds the pairing.
    fn gm_constant_value(name: &str) -> Option<f64> {
        match name {
            "GM_SUN" => Some(GM_SUN),
            "GM_EARTH" => Some(GM_EARTH),
            "GM_MOON" => Some(GM_MOON),
            "GM_MARS" => Some(GM_MARS),
            "GM_JUPITER" => Some(GM_JUPITER),
            "GM_SAGITTARIUS_A_STAR" => Some(GM_SAGITTARIUS_A_STAR),
            _ => None,
        }
    }

    use crate::boundary::cells;

    fn number(cell: &str) -> f64 {
        cell.parse().unwrap_or(f64::NAN)
    }

    #[test]
    fn every_line_has_every_column() {
        let proper = proper_time_line(1e8, 1.0).unwrap_or_default();
        assert_eq!(cells(&proper).len(), PROPER_TIME_COLUMNS);
        let still =
            gravitational_dilation_line("earth", EARTH_EQUATORIAL_RADIUS).unwrap_or_default();
        assert_eq!(cells(&still).len(), GRAVITATIONAL_COLUMNS);
        let listed = gravitating_bodies_lines();
        assert_eq!(listed.lines().count(), GRAVITATING_BODIES.len());
        assert!(
            listed
                .lines()
                .all(|line| line.split('\t').count() == GRAVITATING_BODY_COLUMNS)
        );
    }

    #[test]
    fn six_tenths_of_c_is_five_quarters() {
        let line = proper_time_line(0.6 * SPEED_OF_LIGHT, 10.0).unwrap_or_default();
        let row = cells(&line);
        assert!((number(row[0]) - 0.6).abs() < 1e-15);
        assert!((number(row[1]) - 1.25).abs() < 1e-12);
        assert!((number(row[2]) - 8.0).abs() < 1e-9);
        assert!((number(row[3]) - 0.8).abs() < 1e-12);
        assert!((number(row[4]) - -0.2 * 86_400e6).abs() < 1e-3);
        assert_eq!(row[5], "SPEED_OF_LIGHT");
        assert_eq!(
            proper_time_line(SPEED_OF_LIGHT, 1.0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            proper_time_line(-SPEED_OF_LIGHT, 1.0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(proper_time_line(0.0, f64::NAN), Err(Refusal::OutOfRange));
    }

    #[test]
    fn a_gps_orbit_speed_loses_seven_microseconds_a_day() {
        // The crate's anchor: the kinematic loss of a GPS clock is
        // -7.21 us/day at the circular speed sqrt(GM/r) of its orbit.
        let speed = hc_core::math::sqrt(GM_EARTH / GPS_ORBIT_RADIUS);
        let line = proper_time_line(speed, 86_400.0).unwrap_or_default();
        let micros = number(cells(&line)[4]);
        assert!((micros - -7.21).abs() < 0.01, "{micros}");
    }

    #[test]
    fn a_gps_clock_gains_forty_five_microseconds_a_day_over_the_ground() {
        let ground =
            gravitational_dilation_line("earth", EARTH_EQUATORIAL_RADIUS).unwrap_or_default();
        let orbit = gravitational_dilation_line("EARTH", GPS_ORBIT_RADIUS).unwrap_or_default();
        let (ground, orbit) = (cells(&ground), cells(&orbit));
        assert_eq!(ground[2], "GM_EARTH");
        assert_eq!(ground[6], "GM_EARTH;SPEED_OF_LIGHT_SQUARED");
        let gain = number(orbit[5]) - number(ground[5]);
        assert!((gain - 45.65).abs() < 0.01, "{gain}");
        // The Sun's Schwarzschild radius is 2953.25 m.
        let sun = gravitational_dilation_line("sun", 6.957e8).unwrap_or_default();
        assert!((number(cells(&sun)[3]) - 2_953.25).abs() < 0.01);
    }

    #[test]
    fn a_radius_inside_the_horizon_or_an_unknown_body_is_refused() {
        assert_eq!(
            gravitational_dilation_line("sun", 2_000.0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            gravitational_dilation_line("earth", 0.0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            gravitational_dilation_line("vulcan", 1e7),
            Err(Refusal::Unknown)
        );
        // A body is named by its identifier, not its name.
        for name in ["Sagittarius A*", "Mars system", "Jupiter system"] {
            assert_eq!(
                gravitational_dilation_line(name, 1e12),
                Err(Refusal::Unknown)
            );
        }
    }

    #[test]
    fn every_body_names_the_constant_that_holds_its_gm() {
        for body in GRAVITATING_BODIES {
            let name = body.gm_constant;
            assert_eq!(gm_constant_value(name), Some(body.gm), "{}", body.id);
        }
    }
}
