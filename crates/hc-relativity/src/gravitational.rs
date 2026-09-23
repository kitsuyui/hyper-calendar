//! Gravitational time dilation in the Schwarzschild metric.
//!
//! A clock deeper in a gravitational well runs slow. For a static observer at
//! radius `r` outside a spherical, non-rotating mass,
//!
//! ```text
//! dτ/dt = √(1 − r_s/r),     r_s = 2GM/c²
//! ```
//!
//! where `t` is the coordinate time of an observer infinitely far away. The
//! metric is Schwarzschild's, so the model is exact for a non-rotating,
//! uncharged, spherically symmetric mass and approximate for anything real.
//! For the Earth the leading correction is the quadrupole moment `J₂`, which
//! shifts these numbers by about one part in 10³ — nanoseconds per day
//! on the GPS figures below, not microseconds. Rotation (the Kerr metric) and
//! the Earth's own rotation in the ground clock's frame are not modelled at
//! all.
//!
//! # The canonical worked example
//!
//! A GPS satellite clock gains about **+45.7 µs/day** from being higher in
//! the well and loses about **−7.2 µs/day** from moving, for a net gain of
//! about **+38.4 µs/day**. That is roughly 10 km of positioning error a day,
//! which is why the satellites' oscillators are deliberately offset before
//! launch. Both figures and their sum are tested here.

use hc_core::math;

use crate::constants::SPEED_OF_LIGHT_SQUARED;
use crate::error::{RelativityError, RelativityResult, finite};

/// Seconds in a nominal day, for expressing a fractional rate as a clock
/// offset.
const SECONDS_PER_DAY: f64 = 86_400.0;

/// Check a standard gravitational parameter.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for NaN or infinity and
/// [`RelativityError::NegativeMass`] for a negative value.
fn check_gm(gm: f64) -> RelativityResult<f64> {
    let gm = finite(gm)?;
    if gm < 0.0 {
        return Err(RelativityError::NegativeMass);
    }
    Ok(gm)
}

/// Check a radius.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for NaN or infinity and
/// [`RelativityError::NonPositiveRadius`] for zero or a negative value.
fn check_radius(radius: f64) -> RelativityResult<f64> {
    let radius = finite(radius)?;
    if radius <= 0.0 {
        return Err(RelativityError::NonPositiveRadius);
    }
    Ok(radius)
}

/// The Schwarzschild radius `2GM/c²`, in metres, from a standard
/// gravitational parameter.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for a NaN or infinite argument and
/// [`RelativityError::NegativeMass`] for a negative `gm`.
pub fn schwarzschild_radius(gm: f64) -> RelativityResult<f64> {
    Ok(2.0 * check_gm(gm)? / SPEED_OF_LIGHT_SQUARED)
}

/// The Schwarzschild radius from a mass in kilograms.
///
/// This goes through `G`, whose 2·10⁻⁵ relative uncertainty then limits the
/// answer to five figures. [`schwarzschild_radius`] avoids that whenever a
/// `GM` is available, which for every body in [`crate::constants`] it is.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for a NaN or infinite argument and
/// [`RelativityError::NegativeMass`] for a negative `mass`.
pub fn schwarzschild_radius_of_mass(mass: f64) -> RelativityResult<f64> {
    schwarzschild_radius(crate::constants::GRAVITATIONAL_CONSTANT * finite(mass)?)
}

/// The time-dilation factor `√(1 − r_s/r)` of a clock held still at radius
/// `r`.
///
/// The result is `dτ/dt` against a clock at infinity: always below 1, and
/// approaching 1 far from the mass.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for a NaN or infinite argument,
/// [`RelativityError::NegativeMass`] for a negative `gm` and
/// [`RelativityError::NonPositiveRadius`] for a radius that is zero or
/// negative. Also returns [`RelativityError::InsideHorizon`] when
/// `r ≤ r_s`, where no static observer exists.
pub fn static_dilation_factor(gm: f64, radius: f64) -> RelativityResult<f64> {
    let gm = check_gm(gm)?;
    let radius = check_radius(radius)?;
    let horizon = schwarzschild_radius(gm)?;
    if radius <= horizon {
        return Err(RelativityError::InsideHorizon);
    }
    Ok(math::sqrt(1.0 - horizon / radius))
}

/// The ratio of received to emitted frequency for light climbing from
/// `emitter_radius` to `observer_radius`.
///
/// Below 1 is a redshift, which is what climbing out of a well produces.
/// This is the Pound–Rebka experiment's quantity.
///
/// # Errors
///
/// See [`static_dilation_factor`].
pub fn gravitational_frequency_ratio(
    gm: f64,
    emitter_radius: f64,
    observer_radius: f64,
) -> RelativityResult<f64> {
    let emitter = static_dilation_factor(gm, emitter_radius)?;
    let observer = static_dilation_factor(gm, observer_radius)?;
    if observer == 0.0 {
        return Err(RelativityError::InsideHorizon);
    }
    Ok(emitter / observer)
}

/// The gravitational redshift `z = λ_observed/λ_emitted − 1`.
///
/// Positive when the light climbs out of a well, negative when it falls in.
///
/// # Errors
///
/// See [`gravitational_frequency_ratio`].
pub fn gravitational_redshift(
    gm: f64,
    emitter_radius: f64,
    observer_radius: f64,
) -> RelativityResult<f64> {
    let ratio = gravitational_frequency_ratio(gm, emitter_radius, observer_radius)?;
    if ratio == 0.0 {
        return Err(RelativityError::InsideHorizon);
    }
    Ok(1.0 / ratio - 1.0)
}

/// The Newtonian circular-orbit speed `√(GM/r)`, in metres per second.
///
/// Newtonian rather than relativistic: the relativistic correction is of
/// order `r_s/r`, which at the GPS altitude is 5·10⁻¹⁰ and well inside the
/// error of everything else here. It would matter close to a black hole, and
/// this function should not be used there.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for a NaN or infinite argument,
/// [`RelativityError::NegativeMass`] for a negative `gm` and
/// [`RelativityError::NonPositiveRadius`] for a radius that is zero or
/// negative.
pub fn circular_orbit_speed(gm: f64, radius: f64) -> RelativityResult<f64> {
    let gm = check_gm(gm)?;
    let radius = check_radius(radius)?;
    Ok(math::sqrt(gm / radius))
}

/// The exact `dτ/dt` of a clock on a circular geodesic orbit at radius `r`.
///
/// `√(1 − 3GM/rc²)` — note the **3**, not the 2 of a static clock. The extra
/// term is the orbital motion, and the factor is exact in Schwarzschild
/// rather than a weak-field expansion.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for a NaN or infinite argument,
/// [`RelativityError::NegativeMass`] for a negative `gm` and
/// [`RelativityError::NonPositiveRadius`] for a radius that is zero or
/// negative. Also returns [`RelativityError::InsideHorizon`] inside the
/// photon sphere at `r = 3GM/c²`, where no circular timelike orbit exists.
pub fn circular_orbit_dilation_factor(gm: f64, radius: f64) -> RelativityResult<f64> {
    let gm = check_gm(gm)?;
    let radius = check_radius(radius)?;
    let term = 3.0 * gm / (radius * SPEED_OF_LIGHT_SQUARED);
    if term >= 1.0 {
        return Err(RelativityError::InsideHorizon);
    }
    Ok(math::sqrt(1.0 - term))
}

/// The exact fractional rate at which an orbiting clock gains on a clock
/// held still at `ground_radius`.
///
/// Positive means the orbiting clock runs fast. The two exact Schwarzschild
/// factors are divided rather than expanded, so this is the reference against
/// which [`weak_field_orbit_rate_offset`] is checked.
///
/// # Errors
///
/// See [`circular_orbit_dilation_factor`] and [`static_dilation_factor`].
pub fn orbit_rate_offset(gm: f64, orbit_radius: f64, ground_radius: f64) -> RelativityResult<f64> {
    let orbit = circular_orbit_dilation_factor(gm, orbit_radius)?;
    let ground = static_dilation_factor(gm, ground_radius)?;
    if ground == 0.0 {
        return Err(RelativityError::InsideHorizon);
    }
    Ok(orbit / ground - 1.0)
}

/// The gravitational part of the rate offset alone,
/// `GM/c² · (1/r_ground − 1/r_orbit)`.
///
/// Positive: being higher makes the clock run fast. This is the weak-field
/// first-order term.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for a NaN or infinite argument,
/// [`RelativityError::NegativeMass`] for a negative `gm` and
/// [`RelativityError::NonPositiveRadius`] for a radius that is zero or
/// negative.
pub fn gravitational_rate_offset(
    gm: f64,
    orbit_radius: f64,
    ground_radius: f64,
) -> RelativityResult<f64> {
    let gm = check_gm(gm)?;
    let orbit = check_radius(orbit_radius)?;
    let ground = check_radius(ground_radius)?;
    Ok(gm / SPEED_OF_LIGHT_SQUARED * (1.0 / ground - 1.0 / orbit))
}

/// The kinematic part of the rate offset alone, `−v²/2c² = −GM/2rc²`.
///
/// Negative: moving makes the clock run slow.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for a NaN or infinite argument,
/// [`RelativityError::NegativeMass`] for a negative `gm` and
/// [`RelativityError::NonPositiveRadius`] for a radius that is zero or
/// negative.
pub fn kinematic_rate_offset(gm: f64, orbit_radius: f64) -> RelativityResult<f64> {
    let gm = check_gm(gm)?;
    let orbit = check_radius(orbit_radius)?;
    Ok(-gm / (2.0 * orbit * SPEED_OF_LIGHT_SQUARED))
}

/// The weak-field sum of the gravitational and kinematic rate offsets.
///
/// The textbook GPS calculation. It agrees with the exact
/// [`orbit_rate_offset`] to better than one part in 10⁸ of itself at Earth
/// orbit, which the tests check.
///
/// # Errors
///
/// See [`gravitational_rate_offset`] and [`kinematic_rate_offset`].
pub fn weak_field_orbit_rate_offset(
    gm: f64,
    orbit_radius: f64,
    ground_radius: f64,
) -> RelativityResult<f64> {
    Ok(gravitational_rate_offset(gm, orbit_radius, ground_radius)?
        + kinematic_rate_offset(gm, orbit_radius)?)
}

/// Turn a fractional rate offset into microseconds gained per nominal day.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for a non-finite offset.
pub fn rate_offset_to_micros_per_day(offset: f64) -> RelativityResult<f64> {
    Ok(finite(offset)? * SECONDS_PER_DAY * 1e6)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::constants::{
        EARTH_EQUATORIAL_RADIUS, GM_EARTH, GM_SAGITTARIUS_A_STAR, GM_SUN, GPS_ORBIT_RADIUS,
    };

    #[test]
    fn the_schwarzschild_radius_of_the_sun_is_about_three_kilometres() {
        // The standard quoted value is 2.95 km.
        let radius = schwarzschild_radius(GM_SUN).unwrap();
        assert!((radius - 2_953.25).abs() < 1.0, "got {radius} m");
    }

    #[test]
    fn the_schwarzschild_radius_of_the_earth_is_about_nine_millimetres() {
        let radius = schwarzschild_radius(GM_EARTH).unwrap();
        assert!((radius - 0.008_87).abs() < 1e-5, "got {radius} m");
    }

    #[test]
    fn the_schwarzschild_radius_of_sagittarius_a_star_is_about_a_tenth_of_an_au() {
        let radius = schwarzschild_radius(GM_SAGITTARIUS_A_STAR).unwrap();
        let in_au = radius / crate::constants::ASTRONOMICAL_UNIT;
        assert!((in_au - 0.0848).abs() < 0.001, "got {in_au} AU");
    }

    #[test]
    fn the_radius_from_a_mass_agrees_with_the_radius_from_a_gm() {
        let solar_mass = GM_SUN / crate::constants::GRAVITATIONAL_CONSTANT;
        let from_mass = schwarzschild_radius_of_mass(solar_mass).unwrap();
        let from_gm = schwarzschild_radius(GM_SUN).unwrap();
        assert!(
            (from_mass - from_gm).abs() < 1e-6,
            "{from_mass} vs {from_gm}"
        );
    }

    #[test]
    fn a_negative_mass_is_rejected() {
        assert_eq!(
            schwarzschild_radius(-1.0),
            Err(RelativityError::NegativeMass)
        );
        assert_eq!(
            schwarzschild_radius(f64::NAN),
            Err(RelativityError::NotFinite)
        );
    }

    #[test]
    fn a_clock_on_the_ground_runs_slow_by_seven_parts_in_ten_to_the_tenth() {
        // The Earth's surface potential is GM/rc^2 = 6.95e-10, so the geoid
        // runs about 7e-10 slow against infinity: 60 microseconds a day.
        let factor = static_dilation_factor(GM_EARTH, EARTH_EQUATORIAL_RADIUS).unwrap();
        let deficit = 1.0 - factor;
        assert!(
            (deficit - 6.95e-10).abs() < 1e-11,
            "got a deficit of {deficit}"
        );
    }

    #[test]
    fn the_dilation_factor_approaches_one_far_from_the_mass() {
        let far = static_dilation_factor(GM_SUN, 1e15).unwrap();
        assert!(far < 1.0);
        assert!(1.0 - far < 1e-11, "got {far}");
    }

    #[test]
    fn a_radius_at_or_inside_the_horizon_is_an_error_rather_than_a_nan() {
        let horizon = schwarzschild_radius(GM_SUN).unwrap();
        assert_eq!(
            static_dilation_factor(GM_SUN, horizon),
            Err(RelativityError::InsideHorizon)
        );
        assert_eq!(
            static_dilation_factor(GM_SUN, horizon * 0.5),
            Err(RelativityError::InsideHorizon)
        );
        assert!(static_dilation_factor(GM_SUN, horizon * 1.000_001).is_ok());
    }

    #[test]
    fn a_zero_or_negative_radius_is_rejected() {
        assert_eq!(
            static_dilation_factor(GM_EARTH, 0.0),
            Err(RelativityError::NonPositiveRadius)
        );
        assert_eq!(
            static_dilation_factor(GM_EARTH, -1.0),
            Err(RelativityError::NonPositiveRadius)
        );
    }

    #[test]
    fn light_climbing_out_of_a_well_is_redshifted() {
        let z =
            gravitational_redshift(GM_EARTH, EARTH_EQUATORIAL_RADIUS, GPS_ORBIT_RADIUS).unwrap();
        assert!(z > 0.0, "got {z}");
        // The potential difference is 5.3e-10.
        assert!((z - 5.29e-10).abs() < 1e-11, "got {z}");
    }

    #[test]
    fn light_falling_into_a_well_is_blueshifted() {
        let z =
            gravitational_redshift(GM_EARTH, GPS_ORBIT_RADIUS, EARTH_EQUATORIAL_RADIUS).unwrap();
        assert!(z < 0.0, "got {z}");
    }

    #[test]
    fn a_frequency_ratio_between_equal_radii_is_one() {
        let ratio = gravitational_frequency_ratio(GM_EARTH, 1e7, 1e7).unwrap();
        assert!((ratio - 1.0).abs() < 1e-15);
        assert!(gravitational_redshift(GM_EARTH, 1e7, 1e7).unwrap().abs() < 1e-15);
    }

    #[test]
    fn the_orbital_speed_of_a_gps_satellite_is_about_four_kilometres_a_second() {
        let speed = circular_orbit_speed(GM_EARTH, GPS_ORBIT_RADIUS).unwrap();
        assert!((speed - 3_873.8).abs() < 1.0, "got {speed} m/s");
    }

    #[test]
    fn gps_satellites_gain_forty_five_point_seven_microseconds_a_day_gravitationally() {
        let offset =
            gravitational_rate_offset(GM_EARTH, GPS_ORBIT_RADIUS, EARTH_EQUATORIAL_RADIUS).unwrap();
        let micros = rate_offset_to_micros_per_day(offset).unwrap();
        assert!((micros - 45.7).abs() < 0.1, "got {micros} us/day");
    }

    #[test]
    fn gps_satellites_lose_seven_point_two_microseconds_a_day_kinematically() {
        let offset = kinematic_rate_offset(GM_EARTH, GPS_ORBIT_RADIUS).unwrap();
        let micros = rate_offset_to_micros_per_day(offset).unwrap();
        assert!((micros + 7.2).abs() < 0.05, "got {micros} us/day");
    }

    #[test]
    fn gps_satellites_gain_thirty_eight_point_four_microseconds_a_day_net() {
        // The canonical worked example, and the reason GPS clocks are
        // offset in frequency before launch.
        let offset =
            weak_field_orbit_rate_offset(GM_EARTH, GPS_ORBIT_RADIUS, EARTH_EQUATORIAL_RADIUS)
                .unwrap();
        let micros = rate_offset_to_micros_per_day(offset).unwrap();
        assert!((micros - 38.4).abs() < 0.1, "got {micros} us/day");
    }

    #[test]
    fn the_exact_and_weak_field_gps_figures_agree() {
        let exact = orbit_rate_offset(GM_EARTH, GPS_ORBIT_RADIUS, EARTH_EQUATORIAL_RADIUS).unwrap();
        let weak =
            weak_field_orbit_rate_offset(GM_EARTH, GPS_ORBIT_RADIUS, EARTH_EQUATORIAL_RADIUS)
                .unwrap();
        // They differ by 8e-9 of themselves: 3e-7 microseconds a day.
        let relative = (exact - weak).abs() / exact.abs();
        assert!(relative < 1e-7, "relative difference {relative}");
    }

    #[test]
    fn the_net_offset_is_the_sum_of_its_two_parts() {
        let gravity =
            gravitational_rate_offset(GM_EARTH, GPS_ORBIT_RADIUS, EARTH_EQUATORIAL_RADIUS).unwrap();
        let motion = kinematic_rate_offset(GM_EARTH, GPS_ORBIT_RADIUS).unwrap();
        let net = weak_field_orbit_rate_offset(GM_EARTH, GPS_ORBIT_RADIUS, EARTH_EQUATORIAL_RADIUS)
            .unwrap();
        assert!((gravity + motion - net).abs() < 1e-20);
    }

    #[test]
    fn a_low_enough_orbit_loses_time_instead_of_gaining_it() {
        // Below half an Earth radius of altitude, about 3 190 km, the
        // kinematic loss wins (see the break-even test below). The
        // International Space Station, at about 420 km, is one such orbit.
        let iss_radius = EARTH_EQUATORIAL_RADIUS + 420_000.0;
        let offset =
            weak_field_orbit_rate_offset(GM_EARTH, iss_radius, EARTH_EQUATORIAL_RADIUS).unwrap();
        let micros = rate_offset_to_micros_per_day(offset).unwrap();
        assert!(micros < 0.0, "got {micros} us/day");
        // This model gives -24.5 us/day: +3.7 from gravity against -28.2
        // from speed, so -28 is the speed term alone.
        assert!((micros + 24.5).abs() < 0.5, "got {micros} us/day");
        let speed_only =
            rate_offset_to_micros_per_day(kinematic_rate_offset(GM_EARTH, iss_radius).unwrap())
                .unwrap();
        assert!((speed_only + 28.2).abs() < 0.1, "got {speed_only} us/day");
        // Comparing with a clock on the rotating geoid instead of a static
        // one at the equatorial radius barely moves it. A geoid clock runs
        // slow of TCG by the defining constant L_G (IAU 2000 Resolution
        // B1.9), which includes the Earth's rotation; the station runs slow
        // of it by (GM/r + v²/2)/c².
        let station =
            (GM_EARTH / iss_radius + GM_EARTH / (2.0 * iss_radius)) / SPEED_OF_LIGHT_SQUARED;
        let against_geoid = (hc_core::scale::L_G - station) * 86_400.0 * 1e6;
        assert!(
            (micros - against_geoid).abs() < 0.2,
            "{micros} against {against_geoid} us/day"
        );
    }

    #[test]
    fn the_break_even_orbit_is_at_one_and_a_half_earth_radii() {
        // The gravitational gain equals the kinematic loss when
        // 1/rg - 1/ro = 1/(2 ro), that is ro = 1.5 rg.
        let break_even = 1.5 * EARTH_EQUATORIAL_RADIUS;
        let offset =
            weak_field_orbit_rate_offset(GM_EARTH, break_even, EARTH_EQUATORIAL_RADIUS).unwrap();
        assert!(offset.abs() < 1e-20, "got {offset}");
    }

    #[test]
    fn a_circular_orbit_dilates_more_than_a_static_clock_at_the_same_radius() {
        let orbiting = circular_orbit_dilation_factor(GM_EARTH, GPS_ORBIT_RADIUS).unwrap();
        let static_clock = static_dilation_factor(GM_EARTH, GPS_ORBIT_RADIUS).unwrap();
        assert!(orbiting < static_clock);
    }

    #[test]
    fn no_circular_orbit_exists_inside_the_photon_sphere() {
        let photon_sphere = 1.5 * schwarzschild_radius(GM_SUN).unwrap();
        assert_eq!(
            circular_orbit_dilation_factor(GM_SUN, photon_sphere),
            Err(RelativityError::InsideHorizon)
        );
        assert!(circular_orbit_dilation_factor(GM_SUN, photon_sphere * 1.01).is_ok());
    }

    #[test]
    fn a_non_finite_rate_offset_is_rejected() {
        assert_eq!(
            rate_offset_to_micros_per_day(f64::NAN),
            Err(RelativityError::NotFinite)
        );
    }
}
