//! Comparing a travelling clock with a clock that stayed behind.
//!
//! A [`crate::worldline::Worldline`] says how much proper time a path costs.
//! This module attaches that to a real epoch: given the TAI instant a ship
//! departs, it gives both the instant it arrives by the coordinate clock and
//! the reading its own clock shows on arrival. The gap between the two is the
//! twin paradox in units a calendar can print.
//!
//! # Why the ship's reading is still an `Instant<Tai>`
//!
//! It is not a TAI reading of the arrival event — the ship is not on the
//! geoid and its clock is not a TAI contributor. It is the label the ship's
//! clock shows, having been set to TAI at departure and then left to run.
//! Keeping it in the same type is what lets the two be subtracted and
//! formatted side by side, which is the whole purpose; the distinction is
//! real and is why [`ClockComparison::lag`] exists as a named quantity
//! rather than something a caller is expected to work out.
//!
//! # Uncertain inputs
//!
//! A velocity known only as `0.60 ± 0.01` makes the proper time uncertain
//! too. [`proper_time_uncertain`] propagates it analytically rather than
//! through the generic [`hc_uncertainty::Uncertain`] operators, because β
//! appears twice in `√(1 − β²)` and those operators assume independent
//! inputs: composing them would over-state the error by treating one
//! variable as two.

use hc_core::{Duration, Instant, Tai, math};
use hc_uncertainty::Uncertain;

use crate::error::{RelativityResult, finite};
use crate::special::check_beta;
use crate::worldline::Worldline;

/// A travelling clock and a stay-at-home clock, compared over one journey.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClockComparison {
    departure: Instant<Tai>,
    coordinate_elapsed: Duration,
    proper_elapsed: Duration,
}

impl ClockComparison {
    /// The instant both clocks were synchronised.
    #[must_use]
    pub const fn departure(self) -> Instant<Tai> {
        self.departure
    }

    /// How much coordinate time the journey took.
    #[must_use]
    pub const fn coordinate_elapsed(self) -> Duration {
        self.coordinate_elapsed
    }

    /// How much proper time the travelling clock recorded.
    #[must_use]
    pub const fn proper_elapsed(self) -> Duration {
        self.proper_elapsed
    }

    /// The arrival instant by the clock that stayed behind.
    ///
    /// # Errors
    ///
    /// Returns [`crate::RelativityError::Overflow`] when the sum leaves the
    /// representable range.
    pub fn coordinate_arrival(self) -> RelativityResult<Instant<Tai>> {
        Ok(self.departure.checked_add(self.coordinate_elapsed)?)
    }

    /// What the travelling clock reads on arrival, having been set to TAI at
    /// departure.
    ///
    /// # Errors
    ///
    /// Returns [`crate::RelativityError::Overflow`] when the sum leaves the
    /// representable range.
    pub fn ship_reading(self) -> RelativityResult<Instant<Tai>> {
        Ok(self.departure.checked_add(self.proper_elapsed)?)
    }

    /// How far behind the travelling clock ends up.
    ///
    /// Never negative: no worldline between two events beats the straight
    /// one, which is the whole content of the twin paradox.
    ///
    /// # Errors
    ///
    /// Returns [`crate::RelativityError::Overflow`] when the difference
    /// leaves the representable range.
    pub fn lag(self) -> RelativityResult<Duration> {
        Ok(self.coordinate_elapsed.checked_sub(self.proper_elapsed)?)
    }
}

/// Run a worldline from a departure instant and compare the two clocks.
///
/// ```
/// use hc_core::{Duration, Instant, Tai};
/// use hc_relativity::dilated::compare_clocks;
/// use hc_relativity::worldline::{Segment, Worldline};
///
/// let legs = [
///     Segment::constant(Duration::from_days(365), 0.6).unwrap(),
///     Segment::constant(Duration::from_days(365), -0.6).unwrap(),
/// ];
/// let departure = Instant::<Tai>::from_epoch(Duration::ZERO);
/// let trip = compare_clocks(departure, Worldline::new(&legs)).unwrap();
/// // Two years away, but the ship's clock shows one year and 219 days.
/// assert!((trip.lag().unwrap().as_days_f64() - 146.0).abs() < 1e-6);
/// ```
///
/// # Errors
///
/// See [`Worldline::proper_time`] and [`Worldline::coordinate_time`].
pub fn compare_clocks(
    departure: Instant<Tai>,
    worldline: Worldline<'_>,
) -> RelativityResult<ClockComparison> {
    Ok(ClockComparison {
        departure,
        coordinate_elapsed: worldline.coordinate_time()?,
        proper_elapsed: worldline.proper_time()?,
    })
}

/// The instant a ship's clock reads on arrival, for callers who want only
/// that.
///
/// # Errors
///
/// See [`compare_clocks`].
pub fn dilated_instant(
    departure: Instant<Tai>,
    worldline: Worldline<'_>,
) -> RelativityResult<Instant<Tai>> {
    compare_clocks(departure, worldline)?.ship_reading()
}

/// The proper time of a constant-speed leg when the speed is uncertain.
///
/// Returns seconds with a 1σ error bar. The derivative of `τ = t√(1 − β²)` is
/// `−tβ/√(1 − β²)`, so a small `σ_β` near `β = 0` barely matters and the same
/// `σ_β` near `β = 1` dominates everything — which is the practical reason an
/// interstellar timeline cannot be quoted to more digits than the cruise
/// velocity is known to.
///
/// # Errors
///
/// See [`check_beta`], which is applied to the central value; also returns
/// [`crate::RelativityError::Uncertainty`] if the propagated value is not a
/// legal [`Uncertain`].
pub fn proper_time_uncertain(
    coordinate_time: Duration,
    beta: Uncertain,
) -> RelativityResult<Uncertain> {
    let central = check_beta(beta.value)?;
    let elapsed = finite(coordinate_time.as_secs_f64())?;
    let factor = math::sqrt(1.0 - central * central);
    let value = elapsed * factor;
    // d/dbeta of t*sqrt(1-beta^2) is -t*beta/sqrt(1-beta^2); the sign drops
    // out because a standard deviation is unsigned.
    let slope = elapsed * math::abs(central) / factor;
    Ok(Uncertain::new(
        finite(value)?,
        finite(slope * beta.std_dev)?,
    )?)
}

/// What a ship's clock reads on arrival, in seconds since the 1970 TAI
/// epoch, when the cruise velocity is uncertain.
///
/// # Errors
///
/// See [`proper_time_uncertain`].
pub fn ship_reading_uncertain(
    departure: Instant<Tai>,
    coordinate_time: Duration,
    beta: Uncertain,
) -> RelativityResult<Uncertain> {
    let elapsed = proper_time_uncertain(coordinate_time, beta)?;
    Ok(elapsed.shifted(finite(departure.since_epoch().as_secs_f64())?)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::constants::{JULIAN_YEAR_SECONDS, LIGHT_YEAR, STANDARD_GRAVITY};
    use crate::worldline::Segment;

    fn epoch() -> Instant<Tai> {
        Instant::from_epoch(Duration::ZERO)
    }

    fn out_and_back() -> [Segment; 2] {
        [
            Segment::constant(Duration::from_days(365), 0.6).expect("a legal leg"),
            Segment::constant(Duration::from_days(365), -0.6).expect("a legal leg"),
        ]
    }

    #[test]
    fn the_travelling_twin_arrives_younger() {
        let legs = out_and_back();
        let trip = compare_clocks(epoch(), Worldline::new(&legs)).unwrap();
        assert_eq!(trip.departure(), epoch());
        assert_eq!(trip.coordinate_elapsed(), Duration::from_days(730));
        assert!((trip.proper_elapsed().as_days_f64() - 584.0).abs() < 1e-6);
        assert!(trip.ship_reading().unwrap() < trip.coordinate_arrival().unwrap());
    }

    #[test]
    fn the_lag_is_the_difference_between_the_two_arrivals() {
        let legs = out_and_back();
        let trip = compare_clocks(epoch(), Worldline::new(&legs)).unwrap();
        let by_subtraction = trip
            .coordinate_arrival()
            .unwrap()
            .duration_since(trip.ship_reading().unwrap())
            .unwrap();
        assert_eq!(trip.lag().unwrap(), by_subtraction);
        assert!((trip.lag().unwrap().as_days_f64() - 146.0).abs() < 1e-6);
    }

    #[test]
    fn the_lag_is_never_negative() {
        for beta in [0.0, 0.1, 0.5, 0.9, 0.999] {
            let legs = [Segment::constant(Duration::from_days(30), beta).unwrap()];
            let trip = compare_clocks(epoch(), Worldline::new(&legs)).unwrap();
            assert!(!trip.lag().unwrap().is_negative(), "negative at {beta}");
        }
    }

    #[test]
    fn a_journey_at_rest_leaves_the_clocks_agreed() {
        let legs = [Segment::constant(Duration::from_days(1000), 0.0).unwrap()];
        let trip = compare_clocks(epoch(), Worldline::new(&legs)).unwrap();
        assert_eq!(trip.lag().unwrap(), Duration::ZERO);
        assert_eq!(
            trip.ship_reading().unwrap(),
            trip.coordinate_arrival().unwrap()
        );
    }

    #[test]
    fn the_departure_instant_is_carried_through() {
        let departure = Instant::<Tai>::from_epoch(Duration::from_days(10_000));
        let legs = out_and_back();
        let trip = compare_clocks(departure, Worldline::new(&legs)).unwrap();
        assert_eq!(
            trip.coordinate_arrival().unwrap(),
            Instant::<Tai>::from_epoch(Duration::from_days(10_730))
        );
        assert!((trip.ship_reading().unwrap().since_epoch().as_days_f64() - 10_584.0).abs() < 1e-6);
    }

    #[test]
    fn the_shortcut_agrees_with_the_full_comparison() {
        let legs = out_and_back();
        let path = Worldline::new(&legs);
        assert_eq!(
            dilated_instant(epoch(), path).unwrap(),
            compare_clocks(epoch(), path)
                .unwrap()
                .ship_reading()
                .unwrap()
        );
    }

    #[test]
    fn an_interstellar_voyage_compresses_by_orders_of_magnitude() {
        // Four light years of flip-and-burn at 1 g, built as one worldline
        // out of the closed-form relations.
        let half = 2.0 * LIGHT_YEAR;
        let burn =
            crate::worldline::rocket_coordinate_time_for_distance(STANDARD_GRAVITY, half).unwrap();
        let outbound = Segment::accelerating(
            Duration::from_secs_f64(burn).unwrap(),
            STANDARD_GRAVITY,
            0.0,
        )
        .unwrap();
        let cruise_beta = outbound.final_beta().unwrap();
        let inbound = Segment::accelerating(
            Duration::from_secs_f64(burn).unwrap(),
            -STANDARD_GRAVITY,
            cruise_beta,
        )
        .unwrap();
        let legs = [outbound, inbound];
        let trip = compare_clocks(epoch(), Worldline::new(&legs)).unwrap();
        let shipboard = trip.proper_elapsed().as_secs_f64() / JULIAN_YEAR_SECONDS;
        let home = trip.coordinate_elapsed().as_secs_f64() / JULIAN_YEAR_SECONDS;
        // About 3.46 years aboard against about 5.61 at home.
        assert!(
            (shipboard - 3.46).abs() < 0.05,
            "shipboard {shipboard} years"
        );
        assert!((home - 5.61).abs() < 0.05, "home {home} years");
        assert!(shipboard < home);
    }

    #[test]
    fn an_exact_velocity_gives_an_exact_proper_time() {
        let beta = Uncertain::exact(0.6).unwrap();
        let elapsed = proper_time_uncertain(Duration::from_secs(100), beta).unwrap();
        assert!((elapsed.value - 80.0).abs() < 1e-9);
        assert!(elapsed.is_exact());
    }

    #[test]
    fn an_uncertain_velocity_makes_the_proper_time_uncertain() {
        // At beta = 0.6, dtau/dbeta = -t * 0.6/0.8 = -0.75 t.
        let beta = Uncertain::new(0.6, 0.01).unwrap();
        let elapsed = proper_time_uncertain(Duration::from_secs(100), beta).unwrap();
        assert!((elapsed.value - 80.0).abs() < 1e-9);
        assert!((elapsed.std_dev - 0.75).abs() < 1e-9, "{}", elapsed.std_dev);
    }

    #[test]
    fn the_error_bar_vanishes_at_rest_and_explodes_near_light_speed() {
        let slow = proper_time_uncertain(
            Duration::from_secs(1_000),
            Uncertain::new(0.0, 0.01).unwrap(),
        )
        .unwrap();
        let fast = proper_time_uncertain(
            Duration::from_secs(1_000),
            Uncertain::new(0.999, 0.01).unwrap(),
        )
        .unwrap();
        assert!(slow.std_dev.abs() < 1e-12, "got {}", slow.std_dev);
        assert!(fast.std_dev > 100.0, "got {}", fast.std_dev);
    }

    #[test]
    fn a_superluminal_central_value_is_rejected() {
        let beta = Uncertain::new(1.0, 0.01).unwrap();
        assert_eq!(
            proper_time_uncertain(Duration::from_secs(1), beta),
            Err(crate::RelativityError::FasterThanLight)
        );
    }

    #[test]
    fn an_uncertain_ship_reading_is_offset_by_the_departure() {
        let departure = Instant::<Tai>::from_epoch(Duration::from_secs(1_000_000));
        let beta = Uncertain::new(0.6, 0.01).unwrap();
        let reading = ship_reading_uncertain(departure, Duration::from_secs(100), beta).unwrap();
        assert!((reading.value - 1_000_080.0).abs() < 1e-6);
        // Shifting by an exact constant leaves the error bar alone.
        assert!((reading.std_dev - 0.75).abs() < 1e-9);
    }

    #[test]
    fn an_uncertain_reading_overlaps_the_exact_one_it_brackets() {
        let beta = Uncertain::new(0.6, 0.05).unwrap();
        let uncertain = proper_time_uncertain(Duration::from_secs(100), beta).unwrap();
        let exact =
            proper_time_uncertain(Duration::from_secs(100), Uncertain::new(0.62, 0.0).unwrap())
                .unwrap();
        assert!(uncertain.overlaps(exact), "{uncertain} against {exact}");
    }
}
