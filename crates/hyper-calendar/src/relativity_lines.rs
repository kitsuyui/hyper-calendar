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
use hc_core::epoch_notation::JULIAN_YEAR_SECONDS;
use hc_relativity::constants::{
    self, GRAVITATING_BODIES, GravitatingBody, LIGHT_YEAR, SPEED_OF_LIGHT, SPEED_OF_LIGHT_SQUARED,
};
use hc_relativity::dilated::proper_time_uncertain;
use hc_relativity::gravitational::{
    circular_orbit_speed, gravitational_rate_offset, kinematic_rate_offset, orbit_rate_offset,
    rate_offset_to_micros_per_day, schwarzschild_radius, static_dilation_factor,
    weak_field_orbit_rate_offset,
};
use hc_relativity::special::{
    add_velocities, add_velocities_complement, composed_lorentz_factor, doppler_factor,
    longitudinal_doppler, lorentz_factor_from_rapidity, lorentz_factor_from_speed, proper_time_of,
    rapidity, transverse_doppler,
};
use hc_relativity::worldline::{
    rocket_beta, rocket_coordinate_time, rocket_distance, rocket_one_minus_beta,
};
use hc_uncertainty::Uncertain;

use crate::boundary::{Answer, Line, Refusal, out_of_range};

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

/// How many columns a line of [`orbit_rate_offset_line`] has.
pub const ORBIT_RATE_COLUMNS: usize = 10;

/// How many columns a line of [`rocket_line`] has.
pub const ROCKET_COLUMNS: usize = 10;

/// How many columns a line of [`flip_and_burn_line`] has.
pub const FLIP_AND_BURN_COLUMNS: usize = 11;

/// How many columns a line of [`doppler_line`] has.
pub const DOPPLER_COLUMNS: usize = 7;

/// How many columns a line of [`velocity_add_line`] has.
pub const VELOCITY_ADD_COLUMNS: usize = 10;

/// How many columns a line of [`schwarzschild_radius_line`] has.
pub const SCHWARZSCHILD_COLUMNS: usize = 6;

/// How many columns a line of [`proper_time_uncertain_line`] has.
pub const PROPER_TIME_UNCERTAIN_COLUMNS: usize = 7;

/// What the last cell of an orbit line names.
const ORBIT_SOURCE: &str = "hc-relativity gravitational::orbit_rate_offset (exact, \
     Schwarzschild), gravitational_rate_offset, kinematic_rate_offset and \
     weak_field_orbit_rate_offset; the body's GM from its own source";

/// What the last cell of a rocket line names.
const ROCKET_SOURCE: &str = "hc-relativity worldline::rocket_coordinate_time, rocket_distance, \
     rocket_beta and rocket_one_minus_beta, the hyperbolic motion of constant proper \
     acceleration from rest; SPEED_OF_LIGHT is exact by the 2019 SI and LIGHT_YEAR is \
     the IAU's Julian year times it";

/// What the last cell of a flip-and-burn line names.
const FLIP_AND_BURN_SOURCE: &str = "hc-relativity worldline::flip_and_burn_proper_time and \
     flip_and_burn_coordinate_time: accelerate for half the distance at constant proper \
     acceleration, turn over, decelerate for the other half, arriving at rest";

/// What the last cell of a Doppler line names.
const DOPPLER_SOURCE: &str = "hc-relativity special::doppler_factor, longitudinal_doppler and \
     transverse_doppler; the angle is the source's direction in the observer's frame";

/// What the last cell of a velocity-addition line names.
const VELOCITY_SOURCE: &str = "hc-relativity special::add_velocities, \
     add_velocities_complement, composed_lorentz_factor and rapidity; collinear velocities";

/// What the last cell of a Schwarzschild-radius line names.
const SCHWARZSCHILD_SOURCE: &str = "hc-relativity gravitational::schwarzschild_radius, 2GM/c^2";

/// What the last cell of an uncertain proper-time line names.
const UNCERTAIN_SOURCE: &str = "hc-relativity dilated::proper_time_uncertain, first-order \
     propagation of the speed's standard deviation through t sqrt(1 - beta^2); the value \
     is printed to the figures its own standard deviation supports";

/// A clock on a circular orbit against one held still on the ground, as
/// one line: the body's identifier, its `GM` in m³ s⁻², the name of the
/// `hc-relativity` constant that holds it, the circular speed `√(GM/r)` in
/// metres per second, the gravitational part of the rate in microseconds
/// per 86 400-second day (positive: the higher clock runs fast), the
/// kinematic part (negative), their weak-field sum, the exact Schwarzschild
/// figure for the same two clocks, the constants used separated by `;`, and
/// the source.
///
/// The GPS figures are the anchor: a clock at `GPS_ORBIT_RADIUS` against one
/// at `EARTH_EQUATORIAL_RADIUS` gains +45.65 µs a day from the potential,
/// loses 7.21 from its speed, and nets +38.44.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a body the crate carries no `GM` for, and
/// [`Refusal::OutOfRange`] for a radius that is not finite or not positive,
/// a ground radius at or inside the Schwarzschild radius, or an orbit radius
/// at or inside the photon sphere `3GM/c²`, where no circular orbit exists.
pub fn orbit_rate_offset_line(
    given: &str,
    orbit_radius_metres: f64,
    ground_radius_metres: f64,
) -> Answer<String> {
    let body = gravitating_body(given)?;
    let exact = orbit_rate_offset(body.gm, orbit_radius_metres, ground_radius_metres)
        .map_err(out_of_range)?;
    let speed = circular_orbit_speed(body.gm, orbit_radius_metres).map_err(out_of_range)?;
    let gravity = gravitational_rate_offset(body.gm, orbit_radius_metres, ground_radius_metres)
        .map_err(out_of_range)?;
    let motion = kinematic_rate_offset(body.gm, orbit_radius_metres).map_err(out_of_range)?;
    let weak = weak_field_orbit_rate_offset(body.gm, orbit_radius_metres, ground_radius_metres)
        .map_err(out_of_range)?;
    let micros = |offset| {
        rate_offset_to_micros_per_day(offset)
            .map_err(out_of_range::<hc_relativity::RelativityError>)
    };
    let name = body.gm_constant;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell(body.id)
        .value(body.gm)
        .cell(name)
        .value(speed)
        .value(micros(gravity)?)
        .value(micros(motion)?)
        .value(micros(weak)?)
        .value(micros(exact)?)
        .value(format_args!("{name};SPEED_OF_LIGHT_SQUARED"))
        .value(format_args!("{ORBIT_SOURCE}: {}", body.source));
    line.end();
    Ok(out)
}

/// A rocket of constant proper acceleration burning from rest, as one line:
/// the acceleration in m s⁻², the proper time aboard in seconds, the
/// coordinate time that passes elsewhere in seconds, the distance covered in
/// metres and in light-years, β, `1 − β` computed without cancellation, the
/// Lorentz factor, the constants used separated by `;`, and the source.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for an acceleration that is not finite and
/// positive, a proper time that is not finite or is negative, and a burn
/// long enough that a value leaves the range of a double: a few hundred
/// years of proper time at 1 g.
pub fn rocket_line(proper_acceleration: f64, proper_seconds: f64) -> Answer<String> {
    if !proper_seconds.is_finite() || proper_seconds < 0.0 {
        return Err(Refusal::OutOfRange);
    }
    let coordinate =
        rocket_coordinate_time(proper_acceleration, proper_seconds).map_err(out_of_range)?;
    let distance = rocket_distance(proper_acceleration, proper_seconds).map_err(out_of_range)?;
    let beta = rocket_beta(proper_acceleration, proper_seconds).map_err(out_of_range)?;
    let complement =
        rocket_one_minus_beta(proper_acceleration, proper_seconds).map_err(out_of_range)?;
    let gamma = lorentz_factor_from_rapidity(proper_acceleration * proper_seconds / SPEED_OF_LIGHT)
        .map_err(out_of_range)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(proper_acceleration)
        .value(proper_seconds)
        .value(coordinate)
        .value(distance)
        .value(distance / LIGHT_YEAR)
        .value(beta)
        .value(complement)
        .value(gamma)
        .cell("SPEED_OF_LIGHT;LIGHT_YEAR")
        .cell(ROCKET_SOURCE);
    line.end();
    Ok(out)
}

/// A flip-and-burn voyage between two points at rest, as one line: the
/// proper acceleration in m s⁻², the distance in metres, the proper time
/// aboard and the coordinate time at home in seconds and in Julian years,
/// the peak β at the turnover, its `1 − β` computed without cancellation,
/// the peak Lorentz factor, and the source.
///
/// 1 g to Andromeda, 2.5 million light-years, is 28.60 years aboard and
/// 2 500 001.94 at home.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for an acceleration that is not finite and
/// positive, a distance that is not finite or is negative, and a voyage that
/// leaves the range of a double.
pub fn flip_and_burn_line(proper_acceleration: f64, distance_metres: f64) -> Answer<String> {
    let proper =
        hc_relativity::worldline::flip_and_burn_proper_time(proper_acceleration, distance_metres)
            .map_err(out_of_range)?;
    let coordinate = hc_relativity::worldline::flip_and_burn_coordinate_time(
        proper_acceleration,
        distance_metres,
    )
    .map_err(out_of_range)?;
    // The Lorentz factor at the turnover is cosh of the half burn's
    // rapidity, which is 1 + a (d/2) / c^2 for a burn over half the distance.
    let gamma = 1.0 + proper_acceleration * (distance_metres / 2.0) / SPEED_OF_LIGHT_SQUARED;
    let beta = hc_core::math::sqrt((1.0 - 1.0 / gamma) * (1.0 + 1.0 / gamma));
    let complement = 1.0 / (gamma * gamma * (1.0 + beta));
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(proper_acceleration)
        .value(distance_metres)
        .value(proper)
        .value(coordinate)
        .value(proper / JULIAN_YEAR_SECONDS)
        .value(coordinate / JULIAN_YEAR_SECONDS)
        .value(beta)
        .value(complement)
        .value(gamma)
        .cell("SPEED_OF_LIGHT;JULIAN_YEAR_SECONDS")
        .cell(FLIP_AND_BURN_SOURCE);
    line.end();
    Ok(out)
}

/// The Doppler shift of a source moving at β, seen at an angle, as one
/// line: β, the cosine of the angle between the source's velocity and the
/// direction from source to observer in the observer's frame, the frequency
/// ratio `f_observed / f_emitted` (above 1 is a blueshift), the redshift
/// `z = λ_observed / λ_emitted − 1`, the head-on factor `√((1+β)/(1−β))`,
/// the transverse factor `1/γ`, and the source.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a β that is not finite or whose magnitude is
/// 1 or more, and for a cosine that is not finite or whose magnitude is above
/// 1.
pub fn doppler_line(beta: f64, cos_theta: f64) -> Answer<String> {
    if !cos_theta.is_finite() || cos_theta.abs() > 1.0 {
        return Err(Refusal::OutOfRange);
    }
    let factor = doppler_factor(beta, cos_theta).map_err(out_of_range)?;
    let head_on = longitudinal_doppler(beta).map_err(out_of_range)?;
    let transverse = transverse_doppler(beta).map_err(out_of_range)?;
    let gamma = 1.0 / transverse;
    // z = gamma (1 - beta cos) - 1, with gamma - 1 = gamma^2 beta^2 / (gamma + 1)
    // so that a slow source keeps its figures.
    let redshift = gamma * gamma * beta * beta / (gamma + 1.0) - gamma * beta * cos_theta;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(beta)
        .value(cos_theta)
        .value(factor)
        .value(redshift)
        .value(head_on)
        .value(transverse)
        .cell(DOPPLER_SOURCE);
    line.end();
    Ok(out)
}

/// The composition of two collinear velocities, as one line: the two β, the
/// composed β, the composed speed in m s⁻¹, `1 − β` of the composition
/// computed without cancellation, the two rapidities, their sum (the
/// composed velocity's rapidity), the composed Lorentz factor, and the
/// source.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a β that is not finite or whose magnitude is
/// 1 or more.
pub fn velocity_add_line(first: f64, second: f64) -> Answer<String> {
    let composed = add_velocities(first, second).map_err(out_of_range)?;
    let complement = add_velocities_complement(first, second).map_err(out_of_range)?;
    let gamma = composed_lorentz_factor(first, second).map_err(out_of_range)?;
    let first_rapidity = rapidity(first).map_err(out_of_range)?;
    let second_rapidity = rapidity(second).map_err(out_of_range)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(first)
        .value(second)
        .value(composed)
        .value(composed * SPEED_OF_LIGHT)
        .value(complement)
        .value(first_rapidity)
        .value(second_rapidity)
        .value(first_rapidity + second_rapidity)
        .value(gamma)
        .cell(VELOCITY_SOURCE);
    line.end();
    Ok(out)
}

/// The Schwarzschild radius of a body, as one line: its identifier, its `GM`
/// in m³ s⁻², the name of the `hc-relativity` constant that holds it, the
/// radius `2GM/c²` in metres, the constants used separated by `;`, and the
/// source of the `GM`.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a body the crate carries no `GM` for.
pub fn schwarzschild_radius_line(given: &str) -> Answer<String> {
    let body = gravitating_body(given)?;
    let horizon = schwarzschild_radius(body.gm).map_err(|_| Refusal::OutOfRange)?;
    let name = body.gm_constant;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell(body.id)
        .value(body.gm)
        .cell(name)
        .value(horizon)
        .value(format_args!("{name};SPEED_OF_LIGHT_SQUARED"))
        .value(format_args!("{SCHWARZSCHILD_SOURCE}: {}", body.source));
    line.end();
    Ok(out)
}

/// A clock moving at a constant speed that is not exactly known, as one
/// line: β and its standard deviation, the proper time in seconds and its
/// standard deviation, the proper time printed to the figures that standard
/// deviation supports, the constants used, and the source.
///
/// The standard deviation is `t β σ_β / √(1 − β²)`: a speed known to a
/// metre a second near the speed of light is an error of seconds a year.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a speed that is not finite or is at or beyond
/// the speed of light either way, a standard deviation that is not finite or
/// is negative, and a coordinate time that is not finite or whose proper
/// time leaves the range of a duration.
pub fn proper_time_uncertain_line(
    speed_metres_per_second: f64,
    speed_std_dev: f64,
    coordinate_seconds: f64,
) -> Answer<String> {
    lorentz_factor_from_speed(speed_metres_per_second).map_err(out_of_range)?;
    let beta = Uncertain::new(
        speed_metres_per_second / SPEED_OF_LIGHT,
        speed_std_dev / SPEED_OF_LIGHT,
    )
    .map_err(out_of_range)?;
    let coordinate = Duration::from_secs_f64(coordinate_seconds).map_err(out_of_range)?;
    let proper = proper_time_uncertain(coordinate, beta).map_err(out_of_range)?;
    let text = proper.to_significant().map_err(|_| Refusal::OutOfRange)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(beta.value)
        .value(beta.std_dev)
        .value(proper.value)
        .value(proper.std_dev)
        .value(text)
        .cell("SPEED_OF_LIGHT")
        .cell(UNCERTAIN_SOURCE);
    line.end();
    Ok(out)
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

    /// `actual` is within `relative` of `expected`.
    fn close(actual: f64, expected: f64, relative: f64) -> bool {
        (actual - expected).abs() <= expected.abs() * relative
    }

    #[test]
    fn a_gps_orbit_against_the_ground_is_forty_five_less_seven() {
        // 60-digit decimal arithmetic (Python's `decimal`) from GM =
        // 3.986004418e14, c = 299 792 458, r = 26 561 750 m, R = 6 378 137 m.
        let line = orbit_rate_offset_line("earth", GPS_ORBIT_RADIUS, EARTH_EQUATORIAL_RADIUS)
            .unwrap_or_default();
        let row = cells(&line);
        assert_eq!(row.len(), ORBIT_RATE_COLUMNS);
        assert_eq!(row[0], "earth");
        assert_eq!(row[2], "GM_EARTH");
        assert!(
            close(number(row[3]), 3_873.829_887_089_528, 1e-13),
            "{}",
            row[3]
        );
        assert!(
            close(number(row[4]), 45.651_861_866_919_7, 1e-13),
            "{}",
            row[4]
        );
        assert!(
            close(number(row[5]), -7.213_124_560_312_606, 1e-13),
            "{}",
            row[5]
        );
        assert!(
            close(number(row[6]), 38.438_737_306_607_09, 1e-13),
            "{}",
            row[6]
        );
        // The exact figure keeps its digits: it agrees with the 60-digit
        // value to 13 figures, where the quotient of the two factors minus 1
        // would hold nine.
        assert!(
            close(number(row[7]), 38.438_737_351_513_17, 1e-12),
            "{}",
            row[7]
        );
        assert_eq!(row[8], "GM_EARTH;SPEED_OF_LIGHT_SQUARED");
        assert!(row[9].contains("IERS Conventions (2010)"), "{}", row[9]);
    }

    #[test]
    fn an_orbit_line_refuses_what_has_no_orbit() {
        let ground = EARTH_EQUATORIAL_RADIUS;
        assert_eq!(
            orbit_rate_offset_line("vulcan", GPS_ORBIT_RADIUS, ground),
            Err(Refusal::Unknown)
        );
        // Inside the photon sphere no circular orbit exists; inside the
        // horizon no clock stands still.
        assert_eq!(
            orbit_rate_offset_line("sun", 4_000.0, 1e9),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            orbit_rate_offset_line("sun", 1e9, 2_000.0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            orbit_rate_offset_line("earth", f64::NAN, ground),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            orbit_rate_offset_line("earth", 0.0, ground),
            Err(Refusal::OutOfRange)
        );
    }

    #[test]
    fn a_year_of_one_g_is_three_quarters_of_c() {
        // 60-digit decimal arithmetic, a = 9.80665 m s^-2, tau = one Julian
        // year of 31 557 600 s: t = (c/a) sinh(a tau / c), d = (c^2/a)
        // (cosh(a tau / c) - 1).
        let line = rocket_line(9.806_65, 31_557_600.0).unwrap_or_default();
        let row = cells(&line);
        assert_eq!(row.len(), ROCKET_COLUMNS);
        assert!(
            close(number(row[2]), 37_468_729.848_254_34, 1e-13),
            "{}",
            row[2]
        );
        assert!(
            close(number(row[3]), 5.332_469_445_011_027e15, 1e-13),
            "{}",
            row[3]
        );
        assert!(
            close(number(row[4]), 0.563_642_465_078_743_3, 1e-13),
            "{}",
            row[4]
        );
        assert!(
            close(number(row[5]), 0.774_827_262_642_545_3, 1e-13),
            "{}",
            row[5]
        );
        assert!(
            close(number(row[7]), 1.581_845_453_802_17, 1e-13),
            "{}",
            row[7]
        );
        assert_eq!(row[8], "SPEED_OF_LIGHT;LIGHT_YEAR");
        // Ten years in, 1 - beta is 2.16e-9, held to its figures.
        let ten = rocket_line(9.806_65, 10.0 * 31_557_600.0).unwrap_or_default();
        let row = cells(&ten);
        assert!(
            close(number(row[6]), 2.160_862_624_905_664_6e-9, 1e-13),
            "{}",
            row[6]
        );
        assert!(
            close(number(row[7]), 15_211.478_343_954_89, 1e-13),
            "{}",
            row[7]
        );
        assert_eq!(rocket_line(0.0, 1.0), Err(Refusal::OutOfRange));
        assert_eq!(rocket_line(9.806_65, -1.0), Err(Refusal::OutOfRange));
        assert_eq!(rocket_line(9.806_65, f64::NAN), Err(Refusal::OutOfRange));
        assert_eq!(rocket_line(9.806_65, 1e12), Err(Refusal::OutOfRange));
    }

    #[test]
    fn one_g_to_andromeda_is_twenty_nine_years_aboard() {
        // The same arithmetic for a flip-and-burn over 2.5 million
        // light-years of 9 460 730 472 580 800 m: 28.60341834 years aboard,
        // 2 500 001.937429 at home, a peak gamma of 1 290 370.0944 at
        // 1 - beta = 3.0029e-13.
        let distance = 2.5e6 * LIGHT_YEAR;
        let line = flip_and_burn_line(9.806_65, distance).unwrap_or_default();
        let row = cells(&line);
        assert_eq!(row.len(), FLIP_AND_BURN_COLUMNS);
        assert!(
            close(number(row[2]), 902_655_234.736_913_2, 1e-12),
            "{}",
            row[2]
        );
        assert!(
            close(number(row[3]), 78_894_061_140_622.3, 1e-12),
            "{}",
            row[3]
        );
        assert!(
            close(number(row[4]), 28.603_418_344_136_22, 1e-12),
            "{}",
            row[4]
        );
        assert!(
            close(number(row[5]), 2_500_001.937_429_408, 1e-12),
            "{}",
            row[5]
        );
        assert!(
            close(number(row[7]), 3.002_903_842_915_858e-13, 1e-9),
            "{}",
            row[7]
        );
        assert!(
            close(number(row[8]), 1_290_370.094_441_996, 1e-12),
            "{}",
            row[8]
        );
        assert_eq!(flip_and_burn_line(0.0, distance), Err(Refusal::OutOfRange));
        assert_eq!(flip_and_burn_line(9.806_65, -1.0), Err(Refusal::OutOfRange));
        // A voyage of no distance goes nowhere, at rest.
        let still = flip_and_burn_line(9.806_65, 0.0).unwrap_or_default();
        let row = cells(&still);
        assert_eq!(number(row[2]), 0.0);
        assert_eq!(number(row[6]), 0.0);
        assert_eq!(number(row[8]), 1.0);
    }

    #[test]
    fn six_tenths_of_c_has_the_doppler_factors_of_the_textbook() {
        // Head-on at 0.6 c is 2 (a blueshift of z = -1/2), receding 1/2
        // (z = 1), transverse 4/5 (z = 1/4), and at cos = 1/2 it is
        // 8/7 with z = -1/8.
        let doppler = |cosine| doppler_line(0.6, cosine).unwrap_or_default();
        let (head_on, receding, side, oblique) =
            (doppler(1.0), doppler(-1.0), doppler(0.0), doppler(0.5));
        for (line, factor, z) in [
            (&head_on, 2.0, -0.5),
            (&receding, 0.5, 1.0),
            (&side, 0.8, 0.25),
            (&oblique, 8.0 / 7.0, -0.125),
        ] {
            let row = cells(line);
            assert_eq!(row.len(), DOPPLER_COLUMNS);
            assert!(close(number(row[2]), factor, 1e-14), "{}", row[2]);
            assert!((number(row[3]) - z).abs() < 1e-14, "{}", row[3]);
            assert!(close(number(row[4]), 2.0, 1e-14), "{}", row[4]);
            assert!(close(number(row[5]), 0.8, 1e-14), "{}", row[5]);
        }
        assert_eq!(doppler_line(0.6, 1.5), Err(Refusal::OutOfRange));
        assert_eq!(doppler_line(1.0, 0.0), Err(Refusal::OutOfRange));
        assert_eq!(doppler_line(f64::NAN, 0.0), Err(Refusal::OutOfRange));
        assert_eq!(doppler_line(0.6, f64::NAN), Err(Refusal::OutOfRange));
        // A slow source keeps its redshift: 100 m/s transverse is beta^2 / 2.
        let slow = doppler_line(100.0 / SPEED_OF_LIGHT, 0.0).unwrap_or_default();
        let beta = 100.0 / SPEED_OF_LIGHT;
        assert!(close(number(cells(&slow)[3]), beta * beta / 2.0, 1e-6));
    }

    #[test]
    fn two_ships_at_999_thousandths_compose_to_five_parts_in_ten_million() {
        // 50-digit decimal arithmetic: (a + b) / (1 + ab) = 0.99999949949975,
        // 1 - that = 5.0050024999987487e-7, gamma = 999.50025012506253.
        let line = velocity_add_line(0.999, 0.999).unwrap_or_default();
        let row = cells(&line);
        assert_eq!(row.len(), VELOCITY_ADD_COLUMNS);
        assert!(
            close(number(row[2]), 0.999_999_499_499_75, 1e-14),
            "{}",
            row[2]
        );
        assert!(close(number(row[3]), 299_792_307.8, 1e-9), "{}", row[3]);
        assert!(
            close(number(row[4]), 5.005_002_499_998_749e-7, 1e-13),
            "{}",
            row[4]
        );
        assert!(
            close(number(row[5]), 3.800_201_167_250_199, 1e-13),
            "{}",
            row[5]
        );
        assert!(
            close(number(row[7]), 7.600_402_334_500_399, 1e-13),
            "{}",
            row[7]
        );
        assert!(
            close(number(row[8]), 999.500_250_125_062_5, 1e-13),
            "{}",
            row[8]
        );
        // 0.6 and 0.8 compose to 35/37.
        let exact = velocity_add_line(0.6, 0.8).unwrap_or_default();
        assert!(close(number(cells(&exact)[2]), 35.0 / 37.0, 1e-15));
        assert_eq!(velocity_add_line(1.0, 0.5), Err(Refusal::OutOfRange));
        assert_eq!(velocity_add_line(0.5, f64::NAN), Err(Refusal::OutOfRange));
    }

    #[test]
    fn the_radius_of_a_body_is_two_gm_over_c_squared() {
        // 2GM/c^2 with the 60-digit arithmetic: the Sun 2953.2500770188 m,
        // the Earth 8.870056078235341 mm.
        let sun = schwarzschild_radius_line("sun").unwrap_or_default();
        let row = cells(&sun);
        assert_eq!(row.len(), SCHWARZSCHILD_COLUMNS);
        assert!(
            close(number(row[3]), 2_953.250_077_018_84, 1e-13),
            "{}",
            row[3]
        );
        assert_eq!(row[2], "GM_SUN");
        let earth = schwarzschild_radius_line("EARTH").unwrap_or_default();
        assert!(close(
            number(cells(&earth)[3]),
            8.870_056_078_235_341e-3,
            1e-13
        ));
        assert_eq!(
            schwarzschild_radius_line("Sagittarius A*"),
            Err(Refusal::Unknown)
        );
    }

    #[test]
    fn an_uncertain_speed_gives_an_uncertain_proper_time() {
        // beta = 0.6 with sigma_v = 1000 m/s, 1e9 s: tau = 8e8 s, and
        // sigma_tau = t beta sigma_beta / sqrt(1 - beta^2) = 2501.7307 s.
        let line =
            proper_time_uncertain_line(0.6 * SPEED_OF_LIGHT, 1_000.0, 1e9).unwrap_or_default();
        let row = cells(&line);
        assert_eq!(row.len(), PROPER_TIME_UNCERTAIN_COLUMNS);
        assert!(close(number(row[0]), 0.6, 1e-15));
        assert!(
            close(number(row[1]), 3.335_640_951_981_52e-6, 1e-13),
            "{}",
            row[1]
        );
        assert!(close(number(row[2]), 8e8, 1e-12), "{}", row[2]);
        assert!(
            close(number(row[3]), 2_501.730_713_986_14, 1e-9),
            "{}",
            row[3]
        );
        // Two figures of sigma are 2.5e3, so the value keeps its last figure
        // at the hundreds: 8.000e8 s is 800 000 000 to the thousands.
        assert!(row[4].starts_with("8.000"), "{}", row[4]);
        assert_eq!(
            proper_time_uncertain_line(0.6 * SPEED_OF_LIGHT, -1.0, 1e9),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            proper_time_uncertain_line(SPEED_OF_LIGHT, 1.0, 1e9),
            Err(Refusal::OutOfRange)
        );
        // No error bar is the exact line of `hc_proper_time`.
        let exact = proper_time_uncertain_line(0.6 * SPEED_OF_LIGHT, 0.0, 10.0).unwrap_or_default();
        let row = cells(&exact);
        assert!(close(number(row[2]), 8.0, 1e-12));
        assert_eq!(number(row[3]), 0.0);
    }
}
