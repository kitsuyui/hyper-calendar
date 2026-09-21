//! Special relativity: Lorentz factors, rapidity, Doppler and aberration.
//!
//! Everything here is flat-spacetime kinematics between inertial frames. The
//! gravitational half lives in [`crate::gravitational`], and a trajectory
//! that changes speed lives in [`crate::worldline`].
//!
//! # β, and why it is the argument
//!
//! Every function takes the speed as `β = v/c`, a pure number, rather than
//! metres per second. That is not only the convention of the literature: it
//! is the only parameterisation in which the guard is exact. `|β| ≥ 1` is a
//! clean comparison; `v ≥ 299 792 458` is a comparison against a rounded
//! division. [`lorentz_factor_from_speed`] exists for callers who have an
//! SI velocity, and does that division once, in one place.
//!
//! # Rapidity
//!
//! Velocities do not add; rapidities do. `w = artanh β` turns the velocity
//! addition law into ordinary addition, makes a Lorentz boost a rotation
//! through an imaginary angle, and is what the relativistic rocket
//! integrates linearly in proper time. Anything that looks awkward in β is
//! usually trivial in `w`.

use hc_core::math;

use crate::constants::SPEED_OF_LIGHT;
use crate::error::{RelativityError, RelativityResult, finite};
use crate::hyperbolic;

/// Check that a speed is a legal fraction of `c`.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for NaN or infinity and
/// [`RelativityError::FasterThanLight`] for `|β| ≥ 1`.
pub fn check_beta(beta: f64) -> RelativityResult<f64> {
    let beta = finite(beta)?;
    if math::abs(beta) >= 1.0 {
        return Err(RelativityError::FasterThanLight);
    }
    Ok(beta)
}

/// The Lorentz factor `γ = 1/√(1 − β²)`.
///
/// ```
/// use hc_relativity::special::lorentz_factor;
///
/// // The textbook anchor: six tenths of c is exactly five quarters.
/// assert!((lorentz_factor(0.6).unwrap() - 1.25).abs() < 1e-15);
/// ```
///
/// # Errors
///
/// See [`check_beta`].
pub fn lorentz_factor(beta: f64) -> RelativityResult<f64> {
    let beta = check_beta(beta)?;
    Ok(1.0 / math::sqrt(1.0 - beta * beta))
}

/// The Lorentz factor from a speed in metres per second.
///
/// # Errors
///
/// See [`check_beta`].
pub fn lorentz_factor_from_speed(metres_per_second: f64) -> RelativityResult<f64> {
    lorentz_factor(finite(metres_per_second)? / SPEED_OF_LIGHT)
}

/// The speed fraction corresponding to a Lorentz factor.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for a non-finite argument and
/// [`RelativityError::NonPositive`] for `γ < 1`, which no real motion
/// produces.
pub fn beta_from_lorentz(gamma: f64) -> RelativityResult<f64> {
    let gamma = finite(gamma)?;
    if gamma < 1.0 {
        return Err(RelativityError::NonPositive);
    }
    Ok(math::sqrt(1.0 - 1.0 / (gamma * gamma)))
}

/// The rapidity `w = artanh β`.
///
/// # Errors
///
/// See [`check_beta`].
pub fn rapidity(beta: f64) -> RelativityResult<f64> {
    hyperbolic::atanh(check_beta(beta)?)
}

/// The speed fraction corresponding to a rapidity, `β = tanh w`.
///
/// Unlike [`rapidity`] this cannot fail on the physics: `tanh` maps the whole
/// real line into `(−1, 1)`, which is the geometric statement that no finite
/// boost reaches `c`.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for a non-finite argument.
pub fn beta_from_rapidity(rapidity: f64) -> RelativityResult<f64> {
    Ok(hyperbolic::tanh(finite(rapidity)?))
}

/// The Lorentz factor from a rapidity, `γ = cosh w`.
///
/// The hyperbolic form is the numerically better-behaved one at high speed:
/// `1/√(1 − β²)` has to subtract two numbers that are both very close to 1,
/// while `cosh w` never subtracts anything. A rapidity of 20 is a γ of
/// 2.4·10⁸, computed here to full precision; the same γ from β would need β
/// written to 18 digits, which a double does not have.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for a non-finite rapidity, or one
/// large enough that `cosh` overflows (about 710).
pub fn lorentz_factor_from_rapidity(rapidity: f64) -> RelativityResult<f64> {
    finite(hyperbolic::cosh(finite(rapidity)?))
}

/// Compose two collinear velocities, `(β₁ + β₂)/(1 + β₁β₂)`.
///
/// # Errors
///
/// See [`check_beta`]; both arguments are checked.
pub fn add_velocities(first: f64, second: f64) -> RelativityResult<f64> {
    let first = check_beta(first)?;
    let second = check_beta(second)?;
    let denominator = 1.0 + first * second;
    if denominator == 0.0 {
        return Err(RelativityError::NotFinite);
    }
    check_beta((first + second) / denominator)
}

/// The proper time a clock records while coordinate time `coordinate_time`
/// passes, moving at `β`.
///
/// This is the moving clock's own reading: `dτ = dt/γ`.
///
/// # Errors
///
/// See [`check_beta`]; also returns [`RelativityError::Overflow`] when the
/// result leaves the representable range of [`hc_core::Duration`].
pub fn proper_time_of(
    coordinate_time: hc_core::Duration,
    beta: f64,
) -> RelativityResult<hc_core::Duration> {
    let beta = check_beta(beta)?;
    Ok(coordinate_time.scale_f64(math::sqrt(1.0 - beta * beta))?)
}

/// The coordinate time that passes while a clock moving at `β` records
/// `proper_time`.
///
/// The inverse of [`proper_time_of`]: `dt = γ dτ`.
///
/// # Errors
///
/// See [`check_beta`]; also returns [`RelativityError::Overflow`] when the
/// result leaves the representable range of [`hc_core::Duration`].
pub fn coordinate_time_of(
    proper_time: hc_core::Duration,
    beta: f64,
) -> RelativityResult<hc_core::Duration> {
    Ok(proper_time.scale_f64(lorentz_factor(beta)?)?)
}

/// The relativistic Doppler factor, `f_observed / f_emitted`.
///
/// `cos_theta` is the cosine of the angle between the source's velocity and
/// the direction from source to observer, in the observer's frame: `+1` is
/// straight towards the observer, `−1` straight away, `0` transverse. A
/// factor above 1 is a blueshift.
///
/// # Errors
///
/// See [`check_beta`]; also returns [`RelativityError::NotFinite`] for a
/// non-finite `cos_theta` or a vanishing denominator.
pub fn doppler_factor(beta: f64, cos_theta: f64) -> RelativityResult<f64> {
    let beta = check_beta(beta)?;
    let cos_theta = finite(cos_theta)?;
    let gamma = lorentz_factor(beta)?;
    let denominator = gamma * (1.0 - beta * cos_theta);
    if denominator == 0.0 {
        return Err(RelativityError::NotFinite);
    }
    finite(1.0 / denominator)
}

/// The Doppler factor for a source approaching head-on, `√((1+β)/(1−β))`.
///
/// A negative `β` is a receding source and gives a redshift.
///
/// # Errors
///
/// See [`check_beta`].
pub fn longitudinal_doppler(beta: f64) -> RelativityResult<f64> {
    let beta = check_beta(beta)?;
    Ok(math::sqrt((1.0 + beta) / (1.0 - beta)))
}

/// The transverse Doppler factor, `1/γ`.
///
/// This one has no classical counterpart at all: a source passing at closest
/// approach is redshifted purely by time dilation, and measuring it is one of
/// the direct confirmations of special relativity (Ives and Stilwell, 1938).
///
/// # Errors
///
/// See [`check_beta`].
pub fn transverse_doppler(beta: f64) -> RelativityResult<f64> {
    Ok(1.0 / lorentz_factor(beta)?)
}

/// Relativistic aberration: where a direction appears to move.
///
/// Given `cos θ` in the source frame, returns `cos θ'` in a frame moving at
/// `β`: `(cos θ + β)/(1 + β cos θ)`. This is why a fast ship sees the stars
/// crowd towards its bow.
///
/// # Errors
///
/// See [`check_beta`]; also returns [`RelativityError::NotFinite`] for a
/// non-finite or out-of-range `cos_theta`, and
/// [`RelativityError::NonPositive`] when `|cos θ| > 1`.
pub fn aberrated_cosine(cos_theta: f64, beta: f64) -> RelativityResult<f64> {
    let beta = check_beta(beta)?;
    let cos_theta = finite(cos_theta)?;
    if math::abs(cos_theta) > 1.0 {
        return Err(RelativityError::NonPositive);
    }
    let denominator = 1.0 + beta * cos_theta;
    if denominator == 0.0 {
        return Err(RelativityError::NotFinite);
    }
    Ok(((cos_theta + beta) / denominator).clamp(-1.0, 1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    use hc_core::Duration;

    #[test]
    fn the_lorentz_factor_at_six_tenths_of_c_is_exactly_five_quarters() {
        // 1/sqrt(1 - 0.36) = 1/0.8 = 1.25, the canonical textbook anchor.
        assert!((lorentz_factor(0.6).unwrap() - 1.25).abs() < 1e-15);
        assert!((lorentz_factor(-0.6).unwrap() - 1.25).abs() < 1e-15);
    }

    #[test]
    fn the_lorentz_factor_at_four_fifths_of_c_is_five_thirds() {
        assert!((lorentz_factor(0.8).unwrap() - 5.0 / 3.0).abs() < 1e-14);
    }

    #[test]
    fn the_lorentz_factor_at_rest_is_one() {
        assert!((lorentz_factor(0.0).unwrap() - 1.0).abs() < 1e-15);
    }

    #[test]
    fn the_lorentz_factor_grows_without_bound_towards_c() {
        let mut previous = 1.0;
        for beta in [0.1, 0.5, 0.9, 0.99, 0.999, 0.999_999] {
            let gamma = lorentz_factor(beta).unwrap();
            assert!(gamma > previous, "not monotonic at {beta}");
            previous = gamma;
        }
        assert!(lorentz_factor(0.999_999).unwrap() > 700.0);
    }

    #[test]
    fn light_speed_and_beyond_are_errors_rather_than_infinities() {
        assert_eq!(lorentz_factor(1.0), Err(RelativityError::FasterThanLight));
        assert_eq!(lorentz_factor(-1.0), Err(RelativityError::FasterThanLight));
        assert_eq!(lorentz_factor(1.5), Err(RelativityError::FasterThanLight));
        assert_eq!(lorentz_factor(f64::NAN), Err(RelativityError::NotFinite));
        assert_eq!(
            lorentz_factor(f64::INFINITY),
            Err(RelativityError::NotFinite)
        );
    }

    #[test]
    fn a_speed_in_metres_per_second_gives_the_same_factor() {
        let from_beta = lorentz_factor(0.6).unwrap();
        let from_speed = lorentz_factor_from_speed(0.6 * SPEED_OF_LIGHT).unwrap();
        assert!((from_beta - from_speed).abs() < 1e-12);
        assert_eq!(
            lorentz_factor_from_speed(SPEED_OF_LIGHT),
            Err(RelativityError::FasterThanLight)
        );
    }

    #[test]
    fn the_orbital_speed_of_the_earth_barely_dilates_anything() {
        // About 29.8 km/s: gamma - 1 is 5e-9, which is why calendars got
        // away without relativity for four hundred years.
        let gamma = lorentz_factor_from_speed(29_780.0).unwrap();
        assert!((gamma - 1.0) < 1e-8 && (gamma - 1.0) > 4e-9, "got {gamma}");
    }

    #[test]
    fn beta_and_gamma_round_trip() {
        for beta in [0.0, 0.1, 0.5, 0.6, 0.9, 0.99, 0.999_9] {
            let gamma = lorentz_factor(beta).unwrap();
            let recovered = beta_from_lorentz(gamma).unwrap();
            assert!((recovered - beta).abs() < 1e-9, "failed at {beta}");
        }
    }

    #[test]
    fn a_lorentz_factor_below_one_is_rejected() {
        assert_eq!(beta_from_lorentz(0.5), Err(RelativityError::NonPositive));
        assert_eq!(beta_from_lorentz(f64::NAN), Err(RelativityError::NotFinite));
    }

    #[test]
    fn rapidity_and_beta_round_trip() {
        for beta in [-0.99, -0.5, 0.0, 0.3, 0.6, 0.999] {
            let w = rapidity(beta).unwrap();
            assert!(
                (beta_from_rapidity(w).unwrap() - beta).abs() < 1e-12,
                "failed at {beta}"
            );
        }
    }

    #[test]
    fn the_lorentz_factor_is_the_cosine_hyperbolic_of_the_rapidity() {
        for beta in [0.0, 0.3, 0.6, 0.9, 0.999] {
            let from_beta = lorentz_factor(beta).unwrap();
            let from_rapidity = lorentz_factor_from_rapidity(rapidity(beta).unwrap()).unwrap();
            assert!(
                (from_beta - from_rapidity).abs() / from_beta < 1e-12,
                "failed at {beta}"
            );
        }
    }

    #[test]
    fn a_rapidity_beyond_what_beta_can_express_still_gives_a_factor() {
        // gamma = cosh(20) = 2.4e8. Recovering that from a beta would need
        // 1 - beta = 8.6e-18, which a double cannot hold.
        let gamma = lorentz_factor_from_rapidity(20.0).unwrap();
        assert!((gamma - 2.4258e8).abs() / gamma < 1e-4, "got {gamma}");
        assert_eq!(
            lorentz_factor_from_rapidity(f64::NAN),
            Err(RelativityError::NotFinite)
        );
    }

    #[test]
    fn rapidities_add_where_velocities_do_not() {
        // This is the whole point of rapidity: the composition law becomes
        // ordinary addition.
        let a = 0.6;
        let b = 0.7;
        let composed = add_velocities(a, b).unwrap();
        let summed = rapidity(a).unwrap() + rapidity(b).unwrap();
        assert!((rapidity(composed).unwrap() - summed).abs() < 1e-12);
    }

    #[test]
    fn velocity_addition_never_reaches_light_speed() {
        let composed = add_velocities(0.999_999, 0.999_999).unwrap();
        assert!(composed < 1.0, "got {composed}");
        assert!(composed > 0.999_999);
    }

    #[test]
    fn velocity_addition_is_commutative_and_has_zero_as_identity() {
        assert!(
            (add_velocities(0.3, 0.5).unwrap() - add_velocities(0.5, 0.3).unwrap()).abs() < 1e-15
        );
        assert!((add_velocities(0.42, 0.0).unwrap() - 0.42).abs() < 1e-15);
        assert!(add_velocities(0.5, -0.5).unwrap().abs() < 1e-15);
    }

    #[test]
    fn velocity_addition_rejects_a_light_speed_argument() {
        assert_eq!(
            add_velocities(1.0, 0.5),
            Err(RelativityError::FasterThanLight)
        );
    }

    #[test]
    fn a_moving_clock_runs_slow_by_exactly_the_lorentz_factor() {
        let coordinate = Duration::from_secs(100);
        let proper = proper_time_of(coordinate, 0.6).unwrap();
        // gamma = 1.25, so 100 s of coordinate time is 80 s aboard.
        assert!((proper.as_secs_f64() - 80.0).abs() < 1e-9, "{proper}");
    }

    #[test]
    fn proper_and_coordinate_time_are_inverse() {
        let coordinate = Duration::from_secs(86_400);
        for beta in [0.0, 0.1, 0.6, 0.95] {
            let proper = proper_time_of(coordinate, beta).unwrap();
            let recovered = coordinate_time_of(proper, beta).unwrap();
            let error = (recovered.as_secs_f64() - coordinate.as_secs_f64()).abs();
            assert!(error < 1e-6, "failed at {beta}: {error}");
        }
    }

    #[test]
    fn a_clock_at_rest_records_coordinate_time_unchanged() {
        let coordinate = Duration::from_secs(12_345);
        assert_eq!(proper_time_of(coordinate, 0.0).unwrap(), coordinate);
    }

    #[test]
    fn the_longitudinal_doppler_factor_is_the_head_on_limit() {
        for beta in [0.1, 0.5, 0.9] {
            let head_on = doppler_factor(beta, 1.0).unwrap();
            assert!(
                (head_on - longitudinal_doppler(beta).unwrap()).abs() < 1e-12,
                "failed at {beta}"
            );
        }
    }

    #[test]
    fn approaching_blueshifts_and_receding_redshifts() {
        assert!(longitudinal_doppler(0.5).unwrap() > 1.0);
        assert!(longitudinal_doppler(-0.5).unwrap() < 1.0);
        // The two are reciprocals, which is the symmetry of the boost.
        let product = longitudinal_doppler(0.5).unwrap() * longitudinal_doppler(-0.5).unwrap();
        assert!((product - 1.0).abs() < 1e-14);
    }

    #[test]
    fn the_transverse_doppler_shift_is_pure_time_dilation() {
        let transverse = doppler_factor(0.6, 0.0).unwrap();
        assert!((transverse - 0.8).abs() < 1e-14, "got {transverse}");
        assert!((transverse - transverse_doppler(0.6).unwrap()).abs() < 1e-15);
        // It is always a redshift, whichever way the source is going.
        assert!(transverse < 1.0);
    }

    #[test]
    fn a_doppler_factor_at_rest_is_unity_in_every_direction() {
        for cos_theta in [-1.0, -0.5, 0.0, 0.5, 1.0] {
            assert!((doppler_factor(0.0, cos_theta).unwrap() - 1.0).abs() < 1e-15);
        }
    }

    #[test]
    fn aberration_crowds_directions_towards_the_bow() {
        // A direction transverse in the source frame tilts forwards.
        let tilted = aberrated_cosine(0.0, 0.6).unwrap();
        assert!((tilted - 0.6).abs() < 1e-14, "got {tilted}");
        assert!(tilted > 0.0);
    }

    #[test]
    fn aberration_fixes_the_forward_and_backward_directions() {
        assert!((aberrated_cosine(1.0, 0.9).unwrap() - 1.0).abs() < 1e-14);
        assert!((aberrated_cosine(-1.0, 0.9).unwrap() + 1.0).abs() < 1e-14);
    }

    #[test]
    fn aberration_is_reversed_by_the_opposite_boost() {
        for cos_theta in [-0.9, -0.3, 0.0, 0.4, 0.95] {
            let forward = aberrated_cosine(cos_theta, 0.7).unwrap();
            let back = aberrated_cosine(forward, -0.7).unwrap();
            assert!((back - cos_theta).abs() < 1e-12, "failed at {cos_theta}");
        }
    }

    #[test]
    fn aberration_rejects_a_cosine_outside_the_unit_range() {
        assert_eq!(
            aberrated_cosine(1.5, 0.5),
            Err(RelativityError::NonPositive)
        );
        assert_eq!(
            aberrated_cosine(f64::NAN, 0.5),
            Err(RelativityError::NotFinite)
        );
    }

    #[test]
    fn the_doppler_factor_rejects_a_non_finite_angle() {
        assert_eq!(
            doppler_factor(0.5, f64::INFINITY),
            Err(RelativityError::NotFinite)
        );
    }
}
