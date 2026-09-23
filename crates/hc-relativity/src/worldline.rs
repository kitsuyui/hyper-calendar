//! Piecewise worldlines, and the relativistic rocket.
//!
//! A [`Worldline`] is a sequence of [`Segment`]s, each with a coordinate
//! duration, a velocity profile and an optional gravitational potential.
//! Integrating it gives the proper time elapsed along the path — the reading
//! on the clock that travelled it — so a twin paradox or a four-leg
//! interstellar voyage is a list of segments and one call.
//!
//! # The two profiles
//!
//! * [`VelocityProfile::Constant`] is an inertial leg: `dτ = dt/γ`, and the
//!   integral is a multiplication.
//! * [`VelocityProfile::ConstantProperAcceleration`] is a rocket burning at
//!   a fixed acceleration *as felt on board*. Rapidity is then linear in
//!   proper time, `w = w₀ + aτ/c`, which is what makes the whole family of
//!   relations below closed-form rather than numerical:
//!
//! ```text
//! t(τ) = (c/a)(sinh w − sinh w₀)      x(τ) = (c²/a)(cosh w − cosh w₀)
//! τ(t) = (c/a)(arsinh(sinh w₀ + at/c) − w₀)        β(τ) = tanh w
//! ```
//!
//! Every function here is exact for its model; there is no step size and no
//! accumulated integration error.
//!
//! # Combining gravity with motion
//!
//! A segment's optional [`GravitationalPotential`] multiplies the kinematic
//! factor by the static Schwarzschild factor `√(1 − r_s/r)`. That product is
//! the weak-field composition, exact only when the velocity is the one a
//! local static observer would measure. At the field strengths a spacecraft
//! or a satellite sees, the error in the product is of order the product of
//! the two small terms — parts in 10¹⁸ — and can be ignored. Near a black
//! hole it cannot, and this crate does not pretend otherwise.

use hc_core::{Duration, math};

use crate::constants::SPEED_OF_LIGHT;
use crate::error::{RelativityError, RelativityResult, finite};
use crate::gravitational::static_dilation_factor;
use crate::hyperbolic;
use crate::special::check_beta;

/// A static gravitational potential a segment sits in.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GravitationalPotential {
    /// The standard gravitational parameter of the central body, in m³ s⁻².
    pub gm: f64,
    /// The radius the clock is held at, in metres.
    pub radius: f64,
}

impl GravitationalPotential {
    /// Build a potential.
    ///
    /// # Errors
    ///
    /// See [`static_dilation_factor`]: the potential is validated on
    /// construction so that a worldline cannot be built with one that has no
    /// static observer.
    pub fn new(gm: f64, radius: f64) -> RelativityResult<Self> {
        static_dilation_factor(gm, radius)?;
        Ok(Self { gm, radius })
    }

    /// The static time-dilation factor at this potential.
    ///
    /// # Errors
    ///
    /// See [`static_dilation_factor`].
    pub fn dilation_factor(self) -> RelativityResult<f64> {
        static_dilation_factor(self.gm, self.radius)
    }
}

/// How a segment's speed behaves over its coordinate duration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VelocityProfile {
    /// A constant speed, as a fraction of `c`.
    Constant {
        /// The speed fraction, `|β| < 1`.
        beta: f64,
    },
    /// A constant proper acceleration — what the crew feel as weight.
    ConstantProperAcceleration {
        /// The acceleration felt on board, in m s⁻². Negative decelerates.
        proper_acceleration: f64,
        /// The speed fraction at the start of the segment.
        initial_beta: f64,
    },
}

impl VelocityProfile {
    /// A leg at rest in the coordinate frame.
    pub const AT_REST: Self = Self::Constant { beta: 0.0 };

    /// The rapidity at the start of the segment.
    ///
    /// # Errors
    ///
    /// See [`check_beta`].
    fn initial_rapidity(self) -> RelativityResult<f64> {
        match self {
            Self::Constant { beta } => hyperbolic::atanh(check_beta(beta)?),
            Self::ConstantProperAcceleration { initial_beta, .. } => {
                hyperbolic::atanh(check_beta(initial_beta)?)
            }
        }
    }
}

/// One leg of a worldline.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment {
    /// How long the leg lasts in coordinate time.
    pub coordinate_duration: Duration,
    /// How the speed behaves during it.
    pub velocity: VelocityProfile,
    /// The static potential the clock sits in, if any.
    pub potential: Option<GravitationalPotential>,
}

impl Segment {
    /// A leg at a constant speed.
    ///
    /// # Errors
    ///
    /// See [`check_beta`]; also returns
    /// [`RelativityError::NegativeDuration`] for a backwards span.
    pub fn constant(coordinate_duration: Duration, beta: f64) -> RelativityResult<Self> {
        check_beta(beta)?;
        check_duration(coordinate_duration)?;
        Ok(Self {
            coordinate_duration,
            velocity: VelocityProfile::Constant { beta },
            potential: None,
        })
    }

    /// A leg under a constant proper acceleration.
    ///
    /// # Errors
    ///
    /// See [`check_beta`]; also returns
    /// [`RelativityError::NegativeDuration`] for a backwards span and
    /// [`RelativityError::NotFinite`] for a non-finite acceleration.
    pub fn accelerating(
        coordinate_duration: Duration,
        proper_acceleration: f64,
        initial_beta: f64,
    ) -> RelativityResult<Self> {
        finite(proper_acceleration)?;
        check_beta(initial_beta)?;
        check_duration(coordinate_duration)?;
        Ok(Self {
            coordinate_duration,
            velocity: VelocityProfile::ConstantProperAcceleration {
                proper_acceleration,
                initial_beta,
            },
            potential: None,
        })
    }

    /// The same segment held at a gravitational potential.
    #[must_use]
    pub const fn in_potential(mut self, potential: GravitationalPotential) -> Self {
        self.potential = Some(potential);
        self
    }

    /// The rapidity at the end of the segment.
    ///
    /// # Errors
    ///
    /// See [`check_beta`] and [`RelativityError::NotFinite`].
    fn final_rapidity(self) -> RelativityResult<f64> {
        let initial = self.velocity.initial_rapidity()?;
        match self.velocity {
            VelocityProfile::Constant { .. } => Ok(initial),
            VelocityProfile::ConstantProperAcceleration {
                proper_acceleration,
                ..
            } => {
                if proper_acceleration == 0.0 {
                    return Ok(initial);
                }
                let elapsed = self.coordinate_duration.as_secs_f64();
                let target =
                    hyperbolic::sinh(initial) + proper_acceleration * elapsed / SPEED_OF_LIGHT;
                finite(hyperbolic::asinh(target))
            }
        }
    }

    /// The speed fraction at the end of the segment.
    ///
    /// # Errors
    ///
    /// See [`check_beta`].
    pub fn final_beta(self) -> RelativityResult<f64> {
        Ok(hyperbolic::tanh(self.final_rapidity()?))
    }

    /// The proper time the travelling clock records over this segment.
    ///
    /// # Errors
    ///
    /// See [`check_beta`] and [`GravitationalPotential::dilation_factor`];
    /// also returns [`RelativityError::Overflow`] when the result leaves the
    /// representable range.
    pub fn proper_time(self) -> RelativityResult<Duration> {
        check_duration(self.coordinate_duration)?;
        let kinematic = match self.velocity {
            VelocityProfile::Constant { beta } => {
                let beta = check_beta(beta)?;
                self.coordinate_duration
                    .scale_f64(math::sqrt(1.0 - beta * beta))?
            }
            VelocityProfile::ConstantProperAcceleration {
                proper_acceleration,
                ..
            } => {
                if proper_acceleration == 0.0 {
                    let beta = check_beta(self.initial_beta())?;
                    self.coordinate_duration
                        .scale_f64(math::sqrt(1.0 - beta * beta))?
                } else {
                    let initial = self.velocity.initial_rapidity()?;
                    let final_rapidity = self.final_rapidity()?;
                    let seconds = SPEED_OF_LIGHT / proper_acceleration * (final_rapidity - initial);
                    Duration::from_secs_f64(finite(seconds)?)?
                }
            }
        };
        match self.potential {
            Some(potential) => Ok(kinematic.scale_f64(potential.dilation_factor()?)?),
            None => Ok(kinematic),
        }
    }

    /// How far the clock moves in the coordinate frame, in metres.
    ///
    /// # Errors
    ///
    /// See [`check_beta`] and [`RelativityError::NotFinite`].
    pub fn displacement(self) -> RelativityResult<f64> {
        let elapsed = self.coordinate_duration.as_secs_f64();
        match self.velocity {
            VelocityProfile::Constant { beta } => Ok(check_beta(beta)? * SPEED_OF_LIGHT * elapsed),
            VelocityProfile::ConstantProperAcceleration {
                proper_acceleration,
                initial_beta,
            } => {
                if proper_acceleration == 0.0 {
                    return Ok(check_beta(initial_beta)? * SPEED_OF_LIGHT * elapsed);
                }
                let initial = self.velocity.initial_rapidity()?;
                let ending = self.final_rapidity()?;
                finite(
                    SPEED_OF_LIGHT * SPEED_OF_LIGHT / proper_acceleration
                        * hyperbolic::cosh_difference(ending, initial),
                )
            }
        }
    }

    /// The speed fraction at the start of the segment.
    const fn initial_beta(self) -> f64 {
        match self.velocity {
            VelocityProfile::Constant { beta } => beta,
            VelocityProfile::ConstantProperAcceleration { initial_beta, .. } => initial_beta,
        }
    }
}

/// Reject a segment that runs backwards in coordinate time.
fn check_duration(duration: Duration) -> RelativityResult<Duration> {
    if duration.is_negative() {
        return Err(RelativityError::NegativeDuration);
    }
    Ok(duration)
}

/// A path through spacetime, given as a sequence of legs.
///
/// The worldline borrows its segments so that it works without `alloc`; a
/// fixed array on the stack is the usual way to build one.
///
/// ```
/// use hc_core::Duration;
/// use hc_relativity::worldline::{Segment, Worldline};
///
/// // The twin paradox: out at 0.6c for a year, back at 0.6c for a year.
/// let legs = [
///     Segment::constant(Duration::from_days(365), 0.6).unwrap(),
///     Segment::constant(Duration::from_days(365), -0.6).unwrap(),
/// ];
/// let travelled = Worldline::new(&legs).proper_time().unwrap();
/// // gamma is 1.25, so the traveller ages 80 % of two years.
/// assert!((travelled.as_days_f64() - 584.0).abs() < 1e-6);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Worldline<'a> {
    segments: &'a [Segment],
}

impl<'a> Worldline<'a> {
    /// Build a worldline from its legs.
    #[must_use]
    pub const fn new(segments: &'a [Segment]) -> Self {
        Self { segments }
    }

    /// The legs.
    #[must_use]
    pub const fn segments(self) -> &'a [Segment] {
        self.segments
    }

    /// Whether the worldline has no legs at all.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.segments.is_empty()
    }

    /// How many legs the worldline has.
    #[must_use]
    pub const fn len(self) -> usize {
        self.segments.len()
    }

    /// The total coordinate time the worldline spans.
    ///
    /// # Errors
    ///
    /// Returns [`RelativityError::Overflow`] when the sum leaves the
    /// representable range and [`RelativityError::NegativeDuration`] for a
    /// backwards leg.
    pub fn coordinate_time(self) -> RelativityResult<Duration> {
        let mut total = Duration::ZERO;
        for segment in self.segments {
            check_duration(segment.coordinate_duration)?;
            total = total.checked_add(segment.coordinate_duration)?;
        }
        Ok(total)
    }

    /// The proper time the travelling clock records along the whole path.
    ///
    /// # Errors
    ///
    /// See [`Segment::proper_time`].
    pub fn proper_time(self) -> RelativityResult<Duration> {
        let mut total = Duration::ZERO;
        for segment in self.segments {
            total = total.checked_add(segment.proper_time()?)?;
        }
        Ok(total)
    }

    /// The coordinate time the path loses against a clock left behind.
    ///
    /// Always non-negative: the travelling clock can only fall behind.
    ///
    /// # Errors
    ///
    /// See [`Worldline::coordinate_time`] and [`Worldline::proper_time`].
    pub fn elapsed_difference(self) -> RelativityResult<Duration> {
        Ok(self.coordinate_time()?.checked_sub(self.proper_time()?)?)
    }

    /// The speed fraction at the end of the path.
    ///
    /// # Errors
    ///
    /// See [`Segment::final_beta`]. An empty worldline is at rest.
    pub fn final_beta(self) -> RelativityResult<f64> {
        match self.segments.last() {
            Some(segment) => segment.final_beta(),
            None => Ok(0.0),
        }
    }

    /// The net displacement in the coordinate frame, in metres.
    ///
    /// # Errors
    ///
    /// See [`Segment::displacement`].
    pub fn displacement(self) -> RelativityResult<f64> {
        let mut total = 0.0;
        for segment in self.segments {
            total += segment.displacement()?;
        }
        finite(total)
    }
}

/// Check a proper acceleration that must be strictly positive.
fn check_acceleration(proper_acceleration: f64) -> RelativityResult<f64> {
    let value = finite(proper_acceleration)?;
    if value <= 0.0 {
        return Err(RelativityError::NonPositive);
    }
    Ok(value)
}

/// Check a distance that must not be negative.
fn check_distance(distance: f64) -> RelativityResult<f64> {
    let value = finite(distance)?;
    if value < 0.0 {
        return Err(RelativityError::NonPositive);
    }
    Ok(value)
}

/// Coordinate time elapsed after `proper_time` of burn from rest:
/// `(c/a) sinh(aτ/c)`.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for a NaN or infinite argument and
/// [`RelativityError::NonPositive`] for a proper acceleration that is zero or
/// negative. Also returns [`RelativityError::NotFinite`] when the result
/// overflows, which happens after a few hundred years of proper time at 1 g.
pub fn rocket_coordinate_time(proper_acceleration: f64, proper_time: f64) -> RelativityResult<f64> {
    let acceleration = check_acceleration(proper_acceleration)?;
    let tau = finite(proper_time)?;
    finite(SPEED_OF_LIGHT / acceleration * hyperbolic::sinh(acceleration * tau / SPEED_OF_LIGHT))
}

/// Proper time elapsed after `coordinate_time` of burn from rest:
/// `(c/a) arsinh(at/c)`.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for a NaN or infinite argument and
/// [`RelativityError::NonPositive`] for a proper acceleration that is zero or
/// negative.
pub fn rocket_proper_time(proper_acceleration: f64, coordinate_time: f64) -> RelativityResult<f64> {
    let acceleration = check_acceleration(proper_acceleration)?;
    let elapsed = finite(coordinate_time)?;
    finite(
        SPEED_OF_LIGHT / acceleration * hyperbolic::asinh(acceleration * elapsed / SPEED_OF_LIGHT),
    )
}

/// Distance covered after `proper_time` of burn from rest, in metres:
/// `(c²/a)(cosh(aτ/c) − 1)`.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for a NaN or infinite argument and
/// [`RelativityError::NonPositive`] for a proper acceleration that is zero or
/// negative.
pub fn rocket_distance(proper_acceleration: f64, proper_time: f64) -> RelativityResult<f64> {
    let acceleration = check_acceleration(proper_acceleration)?;
    let tau = finite(proper_time)?;
    finite(
        SPEED_OF_LIGHT * SPEED_OF_LIGHT / acceleration
            * hyperbolic::cosh_difference(acceleration * tau / SPEED_OF_LIGHT, 0.0),
    )
}

/// The speed fraction after `proper_time` of burn from rest: `tanh(aτ/c)`.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for a NaN or infinite argument and
/// [`RelativityError::NonPositive`] for a proper acceleration that is zero or
/// negative.
pub fn rocket_beta(proper_acceleration: f64, proper_time: f64) -> RelativityResult<f64> {
    let acceleration = check_acceleration(proper_acceleration)?;
    let tau = finite(proper_time)?;
    Ok(hyperbolic::tanh(acceleration * tau / SPEED_OF_LIGHT))
}

/// The proper time needed to cover `distance` from rest under one continuous
/// burn: `(c/a) arcosh(ad/c² + 1)`.
///
/// The `+ 1` is never actually added: the inverse is evaluated through the
/// arsinh form so that a short burn, where `ad/c²` is far below the last bit
/// of 1, still gives the right answer.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for a NaN or infinite argument and
/// [`RelativityError::NonPositive`] for a proper acceleration that is zero or
/// negative or a negative distance.
pub fn rocket_proper_time_for_distance(
    proper_acceleration: f64,
    distance: f64,
) -> RelativityResult<f64> {
    let acceleration = check_acceleration(proper_acceleration)?;
    let metres = check_distance(distance)?;
    let excess = acceleration * metres / (SPEED_OF_LIGHT * SPEED_OF_LIGHT);
    Ok(SPEED_OF_LIGHT / acceleration * hyperbolic::acosh_one_plus(excess)?)
}

/// The coordinate time needed to cover `distance` from rest under one
/// continuous burn: `√((d/c)² + 2d/a)`.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for a NaN or infinite argument and
/// [`RelativityError::NonPositive`] for a proper acceleration that is zero or
/// negative or a negative distance.
pub fn rocket_coordinate_time_for_distance(
    proper_acceleration: f64,
    distance: f64,
) -> RelativityResult<f64> {
    let acceleration = check_acceleration(proper_acceleration)?;
    let metres = check_distance(distance)?;
    let light_time = metres / SPEED_OF_LIGHT;
    finite(math::sqrt(
        light_time * light_time + 2.0 * metres / acceleration,
    ))
}

/// The proper time for a flip-and-burn voyage: accelerate for half the
/// distance, turn over, decelerate for the other half, arriving at rest.
///
/// This is the profile that makes the famous interstellar figures: 2.5
/// million light years to Andromeda in a shade under 29 years of shipboard
/// time at 1 g, while 2.5 million years pass at home.
///
/// # Errors
///
/// See [`rocket_proper_time_for_distance`].
pub fn flip_and_burn_proper_time(proper_acceleration: f64, distance: f64) -> RelativityResult<f64> {
    Ok(
        2.0 * rocket_proper_time_for_distance(
            proper_acceleration,
            check_distance(distance)? / 2.0,
        )?,
    )
}

/// The coordinate time for a flip-and-burn voyage.
///
/// # Errors
///
/// See [`rocket_coordinate_time_for_distance`].
pub fn flip_and_burn_coordinate_time(
    proper_acceleration: f64,
    distance: f64,
) -> RelativityResult<f64> {
    Ok(2.0
        * rocket_coordinate_time_for_distance(
            proper_acceleration,
            check_distance(distance)? / 2.0,
        )?)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::constants::{
        EARTH_EQUATORIAL_RADIUS, GM_EARTH, GPS_ORBIT_RADIUS, JULIAN_YEAR_SECONDS, LIGHT_YEAR,
        STANDARD_GRAVITY,
    };

    fn years(seconds: f64) -> f64 {
        seconds / JULIAN_YEAR_SECONDS
    }

    #[test]
    fn a_constant_segment_dilates_by_the_lorentz_factor() {
        let leg = Segment::constant(Duration::from_secs(100), 0.6).unwrap();
        assert!((leg.proper_time().unwrap().as_secs_f64() - 80.0).abs() < 1e-9);
    }

    #[test]
    fn a_segment_at_rest_records_coordinate_time_unchanged() {
        let leg = Segment::constant(Duration::from_days(1), 0.0).unwrap();
        assert_eq!(leg.proper_time().unwrap(), Duration::from_days(1));
        assert!(leg.displacement().unwrap().abs() < 1e-9);
    }

    #[test]
    fn the_twin_paradox_comes_out_at_eighty_per_cent() {
        let legs = [
            Segment::constant(Duration::from_days(365), 0.6).unwrap(),
            Segment::constant(Duration::from_days(365), -0.6).unwrap(),
        ];
        let path = Worldline::new(&legs);
        assert_eq!(path.coordinate_time().unwrap(), Duration::from_days(730));
        let travelled = path.proper_time().unwrap().as_days_f64();
        assert!((travelled - 584.0).abs() < 1e-6, "got {travelled} days");
        // The traveller returns home, so the net displacement is zero.
        assert!(path.displacement().unwrap().abs() < 1.0);
    }

    #[test]
    fn the_travelling_twin_always_falls_behind() {
        let legs = [
            Segment::constant(Duration::from_days(365), 0.6).unwrap(),
            Segment::constant(Duration::from_days(365), -0.6).unwrap(),
        ];
        let lost = Worldline::new(&legs).elapsed_difference().unwrap();
        assert!(!lost.is_negative());
        assert!((lost.as_days_f64() - 146.0).abs() < 1e-6);
    }

    #[test]
    fn an_empty_worldline_is_at_rest_and_takes_no_time() {
        let path = Worldline::new(&[]);
        assert!(path.is_empty());
        assert_eq!(path.len(), 0);
        assert_eq!(path.coordinate_time().unwrap(), Duration::ZERO);
        assert_eq!(path.proper_time().unwrap(), Duration::ZERO);
        assert!(path.final_beta().unwrap().abs() < 1e-15);
        assert!(path.displacement().unwrap().abs() < 1e-15);
    }

    #[test]
    fn a_backwards_segment_is_rejected() {
        assert_eq!(
            Segment::constant(Duration::from_secs(-1), 0.5),
            Err(RelativityError::NegativeDuration)
        );
    }

    #[test]
    fn a_superluminal_segment_is_rejected() {
        assert_eq!(
            Segment::constant(Duration::from_secs(1), 1.0),
            Err(RelativityError::FasterThanLight)
        );
        assert_eq!(
            Segment::accelerating(Duration::from_secs(1), 1.0, 1.2),
            Err(RelativityError::FasterThanLight)
        );
    }

    #[test]
    fn an_accelerating_segment_with_zero_acceleration_is_a_constant_one() {
        let coasting = Segment::constant(Duration::from_days(10), 0.5).unwrap();
        let burning = Segment::accelerating(Duration::from_days(10), 0.0, 0.5).unwrap();
        assert_eq!(
            coasting.proper_time().unwrap(),
            burning.proper_time().unwrap()
        );
        assert!((coasting.displacement().unwrap() - burning.displacement().unwrap()).abs() < 1.0);
        assert!((burning.final_beta().unwrap() - 0.5).abs() < 1e-12);
    }

    #[test]
    fn a_one_g_burn_reaches_three_quarters_of_c_in_a_year_of_ship_time() {
        let beta = rocket_beta(STANDARD_GRAVITY, JULIAN_YEAR_SECONDS).unwrap();
        assert!((beta - 0.7748).abs() < 1e-4, "got {beta}");
    }

    #[test]
    fn a_one_g_burn_covers_half_a_light_year_in_the_first_year() {
        let distance = rocket_distance(STANDARD_GRAVITY, JULIAN_YEAR_SECONDS).unwrap();
        let in_light_years = distance / LIGHT_YEAR;
        assert!(
            (in_light_years - 0.5637).abs() < 1e-3,
            "got {in_light_years}"
        );
    }

    #[test]
    fn the_rocket_time_relations_are_mutually_inverse() {
        for tau in [0.0, 1e6, JULIAN_YEAR_SECONDS, 10.0 * JULIAN_YEAR_SECONDS] {
            let coordinate = rocket_coordinate_time(STANDARD_GRAVITY, tau).unwrap();
            let recovered = rocket_proper_time(STANDARD_GRAVITY, coordinate).unwrap();
            let scale = tau.max(1.0);
            assert!((recovered - tau).abs() / scale < 1e-9, "failed at {tau}");
        }
    }

    #[test]
    fn the_rocket_distance_relations_agree_with_each_other() {
        for tau in [1e5, 1e7, JULIAN_YEAR_SECONDS, 5.0 * JULIAN_YEAR_SECONDS] {
            let distance = rocket_distance(STANDARD_GRAVITY, tau).unwrap();
            let recovered = rocket_proper_time_for_distance(STANDARD_GRAVITY, distance).unwrap();
            assert!((recovered - tau).abs() / tau < 1e-7, "failed at {tau}");
        }
    }

    #[test]
    fn the_coordinate_time_for_a_distance_matches_the_proper_time_relation() {
        let distance = 4.0 * LIGHT_YEAR;
        let proper = rocket_proper_time_for_distance(STANDARD_GRAVITY, distance).unwrap();
        let from_proper = rocket_coordinate_time(STANDARD_GRAVITY, proper).unwrap();
        let direct = rocket_coordinate_time_for_distance(STANDARD_GRAVITY, distance).unwrap();
        assert!((from_proper - direct).abs() / direct < 1e-9);
    }

    #[test]
    fn a_short_burn_reduces_to_the_newtonian_answer() {
        // At one second of 1 g the relativistic and Newtonian distances must
        // agree to the part in 10^17 that (at/c)^2 amounts to.
        let distance = rocket_distance(STANDARD_GRAVITY, 1.0).unwrap();
        let newtonian = 0.5 * STANDARD_GRAVITY;
        assert!((distance - newtonian).abs() / newtonian < 1e-14);
        let beta = rocket_beta(STANDARD_GRAVITY, 1.0).unwrap();
        assert!((beta * SPEED_OF_LIGHT - STANDARD_GRAVITY).abs() < 1e-9);
    }

    #[test]
    fn a_one_g_flip_and_burn_reaches_andromeda_in_under_twenty_nine_years() {
        // 2.5 million light years, accelerate to the midpoint and decelerate
        // from it: the canonical relativistic-rocket figure.
        let distance = 2.5e6 * LIGHT_YEAR;
        let proper = years(flip_and_burn_proper_time(STANDARD_GRAVITY, distance).unwrap());
        assert!((proper - 28.6).abs() < 0.2, "got {proper} years");
        assert!(proper > 28.0 && proper < 29.0);
    }

    #[test]
    fn the_same_voyage_takes_two_and_a_half_million_years_at_home() {
        let distance = 2.5e6 * LIGHT_YEAR;
        let coordinate = years(flip_and_burn_coordinate_time(STANDARD_GRAVITY, distance).unwrap());
        // It can never beat light, so it is just over 2.5 million years.
        assert!(coordinate > 2.5e6, "got {coordinate} years");
        assert!((coordinate - 2.5e6) < 10.0, "got {coordinate} years");
    }

    #[test]
    fn a_one_g_flip_and_burn_reaches_proxima_centauri_in_under_four_years() {
        // 4.24 light years: the figure usually quoted is 3.5 years aboard.
        let proper = years(flip_and_burn_proper_time(STANDARD_GRAVITY, 4.24 * LIGHT_YEAR).unwrap());
        assert!((proper - 3.54).abs() < 0.1, "got {proper} years");
    }

    #[test]
    fn a_zero_or_negative_acceleration_is_rejected() {
        assert_eq!(rocket_distance(0.0, 1.0), Err(RelativityError::NonPositive));
        assert_eq!(
            rocket_proper_time_for_distance(-1.0, 1.0),
            Err(RelativityError::NonPositive)
        );
        assert_eq!(rocket_beta(f64::NAN, 1.0), Err(RelativityError::NotFinite));
    }

    #[test]
    fn a_negative_distance_is_rejected() {
        assert_eq!(
            rocket_proper_time_for_distance(STANDARD_GRAVITY, -1.0),
            Err(RelativityError::NonPositive)
        );
        assert_eq!(
            flip_and_burn_proper_time(STANDARD_GRAVITY, -1.0),
            Err(RelativityError::NonPositive)
        );
    }

    #[test]
    fn a_zero_distance_voyage_takes_no_time() {
        assert!(
            flip_and_burn_proper_time(STANDARD_GRAVITY, 0.0)
                .unwrap()
                .abs()
                < 1e-12
        );
        assert!(
            flip_and_burn_coordinate_time(STANDARD_GRAVITY, 0.0)
                .unwrap()
                .abs()
                < 1e-12
        );
    }

    #[test]
    fn an_accelerating_segment_agrees_with_the_closed_form_relations() {
        let burn_seconds = 3.0 * JULIAN_YEAR_SECONDS;
        let coordinate = rocket_coordinate_time(STANDARD_GRAVITY, burn_seconds).unwrap();
        let leg = Segment::accelerating(
            Duration::from_secs_f64(coordinate).unwrap(),
            STANDARD_GRAVITY,
            0.0,
        )
        .unwrap();
        let proper = leg.proper_time().unwrap().as_secs_f64();
        assert!(
            (proper - burn_seconds).abs() / burn_seconds < 1e-9,
            "got {proper} against {burn_seconds}"
        );
        let expected_distance = rocket_distance(STANDARD_GRAVITY, burn_seconds).unwrap();
        let distance = leg.displacement().unwrap();
        assert!((distance - expected_distance).abs() / expected_distance < 1e-9);
        let expected_beta = rocket_beta(STANDARD_GRAVITY, burn_seconds).unwrap();
        assert!((leg.final_beta().unwrap() - expected_beta).abs() < 1e-9);
    }

    #[test]
    fn a_four_leg_voyage_integrates_leg_by_leg() {
        // Accelerate, coast, decelerate, stay: the proper time is the sum,
        // and each leg's own answer is unchanged by its neighbours.
        let legs = [
            Segment::accelerating(Duration::from_days(100), STANDARD_GRAVITY, 0.0).unwrap(),
            Segment::constant(Duration::from_days(400), 0.28).unwrap(),
            Segment::accelerating(Duration::from_days(100), -STANDARD_GRAVITY, 0.28).unwrap(),
            Segment::constant(Duration::from_days(50), 0.0).unwrap(),
        ];
        let path = Worldline::new(&legs);
        assert_eq!(path.len(), 4);
        assert_eq!(path.coordinate_time().unwrap(), Duration::from_days(650));
        let mut summed = Duration::ZERO;
        for leg in path.segments() {
            summed = summed.checked_add(leg.proper_time().unwrap()).unwrap();
        }
        assert_eq!(path.proper_time().unwrap(), summed);
        assert!(path.proper_time().unwrap() < path.coordinate_time().unwrap());
        // The last leg is at rest, so the voyage ends at rest.
        assert!(path.final_beta().unwrap().abs() < 1e-15);
    }

    #[test]
    fn a_deceleration_brings_the_ship_back_towards_rest() {
        let up = Segment::accelerating(Duration::from_days(30), STANDARD_GRAVITY, 0.0).unwrap();
        let reached = up.final_beta().unwrap();
        assert!(reached > 0.0);
        let down =
            Segment::accelerating(Duration::from_days(30), -STANDARD_GRAVITY, reached).unwrap();
        assert!(down.final_beta().unwrap().abs() < 1e-9, "not back at rest");
    }

    #[test]
    fn a_potential_slows_a_segment_further() {
        let potential = GravitationalPotential::new(GM_EARTH, EARTH_EQUATORIAL_RADIUS).unwrap();
        let free = Segment::constant(Duration::from_days(1), 0.0).unwrap();
        let held = free.in_potential(potential);
        assert!(held.proper_time().unwrap() < free.proper_time().unwrap());
        let lost =
            free.proper_time().unwrap().as_secs_f64() - held.proper_time().unwrap().as_secs_f64();
        // 6.95e-10 of a day is 60 microseconds.
        assert!((lost * 1e6 - 60.1).abs() < 1.0, "lost {} us", lost * 1e6);
    }

    #[test]
    fn a_higher_potential_loses_less_time() {
        let ground = GravitationalPotential::new(GM_EARTH, EARTH_EQUATORIAL_RADIUS).unwrap();
        let orbit = GravitationalPotential::new(GM_EARTH, GPS_ORBIT_RADIUS).unwrap();
        assert!(orbit.dilation_factor().unwrap() > ground.dilation_factor().unwrap());
        let day = Duration::from_days(1);
        let below = Segment::constant(day, 0.0).unwrap().in_potential(ground);
        let above = Segment::constant(day, 0.0).unwrap().in_potential(orbit);
        assert!(above.proper_time().unwrap() > below.proper_time().unwrap());
    }

    #[test]
    fn a_potential_inside_a_horizon_cannot_be_built() {
        let horizon = crate::gravitational::schwarzschild_radius(GM_EARTH).unwrap();
        assert_eq!(
            GravitationalPotential::new(GM_EARTH, horizon * 0.5),
            Err(RelativityError::InsideHorizon)
        );
    }

    #[test]
    fn the_gps_satellite_reconstructed_as_a_worldline_gains_its_canonical_offset() {
        // The same worked example as in `gravitational`, but built out of
        // worldline segments: an orbiting clock against a ground clock.
        let day = Duration::from_days(1);
        let speed = crate::gravitational::circular_orbit_speed(GM_EARTH, GPS_ORBIT_RADIUS).unwrap()
            / SPEED_OF_LIGHT;
        let satellite = Segment::constant(day, speed)
            .unwrap()
            .in_potential(GravitationalPotential::new(GM_EARTH, GPS_ORBIT_RADIUS).unwrap());
        let ground = Segment::constant(day, 0.0)
            .unwrap()
            .in_potential(GravitationalPotential::new(GM_EARTH, EARTH_EQUATORIAL_RADIUS).unwrap());
        let gain = satellite.proper_time().unwrap().as_secs_f64()
            - ground.proper_time().unwrap().as_secs_f64();
        assert!((gain * 1e6 - 38.4).abs() < 0.2, "got {} us/day", gain * 1e6);
    }
}
