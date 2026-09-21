//! One timeline, four chronologies, queried together.
//!
//! [`place`] takes a span since the Big Bang and reports which cosmic epoch,
//! which geological interval and which archaeological period it falls in, with
//! the uncertainty carried through every conversion. [`span_between`] takes
//! two and reports the gap with the errors propagated.
//!
//! # The three datums, reconciled
//!
//! The four tables this module joins count from three different origins:
//!
//! | Table | Counts from |
//! | --- | --- |
//! | [`crate::universe`], [`crate::future`] | the Big Bang, forward |
//! | [`crate::geologic`] | the present, backward, in megayears |
//! | [`crate::archaeology`] | 1950 CE, backward, in years |
//!
//! This module converts between them through
//! [`crate::universe::AGE_OF_UNIVERSE`], which means every "years ago" figure
//! inherits its 0.020 Gyr uncertainty — 6.3×10¹⁴ seconds, or twenty million
//! years. That is larger than the whole Quaternary, so a placement's error bar
//! is dominated by cosmology even when the question is archaeological, and the
//! individual chronologies should be queried directly when the question stays
//! inside one of them.
//!
//! The seventy-six years between the BP datum of 1950 and the present are
//! ignored. They are four decades below the smallest uncertainty in any table
//! here, and they matter only inside the Modern period, where nobody uses BP.

use crate::archaeology::{self, ArchaeologicalPeriod};
use crate::error::DeepTimeResult;
use crate::future::{self, FutureEra};
use crate::geologic::{self, GeologicInterval};
use crate::magnitude::{DeepTime, DeepUnit};
use crate::universe::{self, CosmicEpoch, CosmicEvent};

/// Everything the four chronologies have to say about one moment.
///
/// Fields are `None` where the moment falls outside that chronology's reach,
/// which is the normal case: a moment in the hadron epoch has no geology, and
/// a moment in the Devonian has no future era.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Placement {
    /// The moment itself, as a span since the Big Bang.
    pub since_big_bang: DeepTime,
    /// The same moment as a span before the present, negative in the future.
    pub before_present: DeepTime,
    /// Which epoch of cosmic history, if the moment is in the past.
    pub cosmic_epoch: Option<&'static CosmicEpoch>,
    /// The most recent dated cosmic event at or before the moment.
    pub cosmic_event: Option<&'static CosmicEvent>,
    /// Which of Adams & Laughlin's eras, if the moment is in the future.
    pub future_era: Option<&'static FutureEra>,
    /// The geological chain, eon down to age; see [`geologic::chain_at`].
    pub geologic: [Option<&'static GeologicInterval>; 5],
    /// Which conventional archaeological period, if the moment is inside one.
    pub archaeological: Option<&'static ArchaeologicalPeriod>,
}

impl Placement {
    /// The moment in megayears before present, central value only.
    #[must_use]
    pub fn megayears_ago(&self) -> f64 {
        self.before_present.central_seconds() / DeepUnit::Megayear.seconds()
    }

    /// The moment in years before present, central value only.
    ///
    /// Treated as years BP, on the approximation this module documents.
    #[must_use]
    pub fn years_ago(&self) -> f64 {
        self.before_present.central_seconds() / DeepUnit::JulianYear.seconds()
    }

    /// Whether the moment lies after the present day.
    #[must_use]
    pub fn is_in_the_future(&self) -> bool {
        self.before_present.central_seconds() < 0.0
    }

    /// The finest geological interval that has an answer, if any.
    ///
    /// The chain thins out with depth — the Archean has eras but no periods —
    /// so this walks back from the age rank to the first entry that exists.
    #[must_use]
    pub fn finest_geologic(&self) -> Option<&'static GeologicInterval> {
        self.geologic.iter().rev().find_map(|entry| *entry)
    }
}

/// Place a moment, given as a span since the Big Bang.
///
/// # Errors
///
/// Propagates the arithmetic errors of [`DeepTime::checked_sub`] and
/// [`DeepTime::in_unit`]; a finite input in the range `f64` can hold will not
/// produce one.
pub fn place(since_big_bang: DeepTime) -> DeepTimeResult<Placement> {
    let age = universe::AGE_OF_UNIVERSE.deep_time()?;
    let before_present = age.checked_sub(since_big_bang)?;
    assemble(since_big_bang, before_present)
}

/// Place a moment given as a count of years before the present day.
///
/// This entry point keeps the supplied age exact rather than recovering it
/// from the span since the Big Bang. The round trip through 4.35×10¹⁷ seconds
/// would cost about thirty seconds of floating-point resolution, which is
/// invisible everywhere except on a boundary query — and a boundary query is
/// exactly what a chronology gets asked.
///
/// # Errors
///
/// See [`place`].
pub fn place_years_ago(years_ago: f64, std_dev_years: f64) -> DeepTimeResult<Placement> {
    let before_present = DeepTime::from_julian_years(years_ago, std_dev_years)?;
    let age = universe::AGE_OF_UNIVERSE.deep_time()?;
    let since_big_bang = age.checked_sub(before_present)?;
    assemble(since_big_bang, before_present)
}

/// Shared tail of the three entry points.
fn assemble(since_big_bang: DeepTime, before_present: DeepTime) -> DeepTimeResult<Placement> {
    let seconds = since_big_bang.central_seconds();
    let years_ago = before_present.central_seconds() / DeepUnit::JulianYear.seconds();
    let megayears_ago = before_present.central_seconds() / DeepUnit::Megayear.seconds();

    let future_era = if before_present.central_seconds() < 0.0 {
        // The decade is measured from the Big Bang, as Adams & Laughlin do.
        let decade = since_big_bang.log10_seconds()?.value
            - hc_core::math::log10(DeepUnit::JulianYear.seconds());
        future::era_at_decade(decade)
    } else {
        None
    };

    Ok(Placement {
        since_big_bang,
        before_present,
        cosmic_epoch: universe::epoch_at(seconds),
        cosmic_event: universe::event_before(seconds),
        future_era,
        geologic: geologic::chain_at(megayears_ago),
        archaeological: archaeology::period_at_bp(years_ago),
    })
}

/// Place a moment given as a geological age in megayears before present.
///
/// # Errors
///
/// See [`place`].
pub fn place_megayears_ago(megayears_ago: f64, std_dev_ma: f64) -> DeepTimeResult<Placement> {
    place_years_ago(megayears_ago * 1e6, std_dev_ma * 1e6)
}

/// The span between two moments, with both error bars propagated.
///
/// The result is never negative: the two arguments are two moments, and the
/// span between them is a length. The uncertainties add in quadrature, which
/// assumes the two moments were dated independently — true when they come
/// from different tables, and false when both were derived from the age of
/// the universe, where the common term partly cancels and this function
/// overstates the error.
///
/// # Errors
///
/// See [`DeepTime::checked_sub`].
pub fn span_between(one: DeepTime, other: DeepTime) -> DeepTimeResult<DeepTime> {
    let difference = other.checked_sub(one)?;
    if difference.central_seconds() < 0.0 {
        difference.scaled(-1.0)
    } else {
        Ok(difference)
    }
}

/// How many decades of magnitude separate two moments' distances from the Big
/// Bang.
///
/// # Errors
///
/// See [`DeepTime::log10_ratio`].
pub fn orders_of_magnitude_between(one: DeepTime, other: DeepTime) -> DeepTimeResult<f64> {
    one.orders_of_magnitude_between(other)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64, relative: f64) -> bool {
        hc_core::math::abs(a - b) <= hc_core::math::abs(b) * relative
    }

    #[test]
    fn the_present_day_is_placed_in_every_chronology_at_once() {
        let placement = place_years_ago(0.0, 0.0).unwrap();
        assert_eq!(
            placement.cosmic_epoch.map(|e| e.name),
            Some("Era of galaxies")
        );
        assert_eq!(placement.geologic[2].map(|i| i.name), Some("Quaternary"));
        assert_eq!(placement.geologic[4].map(|i| i.name), Some("Meghalayan"));
        assert_eq!(
            placement.archaeological.map(|p| p.name),
            Some("Modern period")
        );
        assert!(placement.future_era.is_none());
        assert!(!placement.is_in_the_future());
    }

    #[test]
    fn the_end_cretaceous_extinction_is_placed_in_the_maastrichtian() {
        let placement = place_megayears_ago(66.0, 0.0).unwrap();
        assert_eq!(placement.geologic[0].map(|i| i.name), Some("Phanerozoic"));
        assert_eq!(placement.geologic[2].map(|i| i.name), Some("Cretaceous"));
        assert_eq!(
            placement.finest_geologic().map(|i| i.name),
            Some("Maastrichtian")
        );
        assert!(
            placement.archaeological.is_none(),
            "no archaeology in the Cretaceous"
        );
        assert_eq!(
            placement.cosmic_epoch.map(|e| e.name),
            Some("Era of galaxies")
        );
    }

    #[test]
    fn two_hundred_and_fifty_megayears_ago_is_the_triassic() {
        let placement = place_megayears_ago(250.0, 0.0).unwrap();
        assert_eq!(placement.geologic[2].map(|i| i.name), Some("Triassic"));
        assert!(close(placement.megayears_ago(), 250.0, 1e-9));
    }

    #[test]
    fn the_solar_system_forms_just_outside_the_chart() {
        // The ICS chart rounds the base of the Hadean to 4567 Ma. The Pb-Pb
        // age of the oldest solids is 4567.30 +/- 0.16 Ma, which is three
        // hundred thousand years older, so the Solar System's formation falls
        // off the top of the chart rather than at its edge. Reporting None
        // here is the honest answer; extending the Hadean to swallow the
        // difference would be editing the chart.
        let placement = place(universe::SOLAR_SYSTEM_FORMATION.deep_time().unwrap()).unwrap();
        assert!(close(placement.megayears_ago(), 4567.30, 1e-4));
        assert!(placement.geologic.iter().all(Option::is_none));
        assert_eq!(
            placement.cosmic_event.map(|e| e.name),
            Some("Formation of the Sun and Solar System")
        );
        // A hair younger and the chart does have an answer.
        let hadean = place_megayears_ago(4566.0, 0.0).unwrap();
        assert_eq!(hadean.geologic[0].map(|i| i.name), Some("Hadean"));
    }

    #[test]
    fn recombination_has_a_cosmic_epoch_and_nothing_else() {
        let placement = place(universe::RECOMBINATION.deep_time().unwrap()).unwrap();
        assert!(placement.cosmic_epoch.is_some());
        assert!(placement.geologic.iter().all(Option::is_none));
        assert!(placement.archaeological.is_none());
        assert!(placement.future_era.is_none());
    }

    #[test]
    fn the_hadron_epoch_has_no_geology() {
        let placement = place(DeepTime::from_seconds(1e-3, 0.0).unwrap()).unwrap();
        assert_eq!(placement.cosmic_epoch.map(|e| e.name), Some("Hadron epoch"));
        assert!(placement.finest_geologic().is_none());
    }

    #[test]
    fn a_neolithic_date_is_placed_in_the_holocene_and_the_neolithic() {
        let placement = place_years_ago(10_000.0, 0.0).unwrap();
        assert_eq!(placement.geologic[3].map(|i| i.name), Some("Holocene"));
        assert_eq!(placement.archaeological.map(|p| p.name), Some("Neolithic"));
        assert!(close(placement.years_ago(), 10_000.0, 1e-9));
    }

    #[test]
    fn an_upper_palaeolithic_date_is_placed_in_the_pleistocene() {
        let placement = place_years_ago(30_000.0, 0.0).unwrap();
        assert_eq!(placement.geologic[3].map(|i| i.name), Some("Pleistocene"));
        assert_eq!(
            placement.archaeological.map(|p| p.name),
            Some("Upper Palaeolithic")
        );
    }

    #[test]
    fn a_future_moment_lands_in_a_cosmological_era() {
        let sun = universe::AGE_OF_UNIVERSE
            .deep_time()
            .unwrap()
            .checked_add(future::SUN_RED_GIANT_TIP.from_now().unwrap())
            .unwrap();
        let placement = place(sun).unwrap();
        assert!(placement.is_in_the_future());
        assert_eq!(
            placement.future_era.map(|e| e.name),
            Some("Stelliferous Era")
        );
        assert!(placement.cosmic_epoch.is_none(), "cosmic history has ended");
        assert!(placement.geologic.iter().all(Option::is_none));
    }

    #[test]
    fn the_black_hole_era_is_reachable_from_the_timeline() {
        let late = future::years_from_decade(50.0).unwrap();
        let placement = place(late).unwrap();
        assert_eq!(placement.future_era.map(|e| e.name), Some("Black Hole Era"));
        assert!(placement.is_in_the_future());
    }

    #[test]
    fn the_uncertainty_is_carried_through_a_placement() {
        // 66.0 +/- 0.05 Ma, the K-Pg boundary's own error bar.
        let placement = place_megayears_ago(66.0, 0.05).unwrap();
        // The age of the universe contributes 0.020 Gyr = 20 Ma, which swamps
        // the 0.05 Ma on the boundary itself.
        let sigma_ma = placement.since_big_bang.std_dev() / DeepUnit::Megayear.seconds();
        assert!(sigma_ma > 19.0, "sigma was {sigma_ma} Ma");
        assert!(sigma_ma < 21.0, "sigma was {sigma_ma} Ma");
        // Going back the other way the cosmological term is gone again.
        let before_sigma = placement.before_present.std_dev() / DeepUnit::Megayear.seconds();
        assert!(before_sigma > 0.0);
    }

    #[test]
    fn the_span_between_two_moments_is_never_negative() {
        let early = DeepTime::from_seconds(10.0, 1.0).unwrap();
        let late = DeepTime::from_seconds(100.0, 2.0).unwrap();
        let forwards = span_between(early, late).unwrap();
        let backwards = span_between(late, early).unwrap();
        assert!(close(forwards.central_seconds(), 90.0, 1e-12));
        assert!(close(backwards.central_seconds(), 90.0, 1e-12));
        assert!(close(forwards.std_dev(), backwards.std_dev(), 1e-12));
    }

    #[test]
    fn the_span_between_two_moments_propagates_both_error_bars() {
        let early = DeepTime::from_seconds(10.0, 3.0).unwrap();
        let late = DeepTime::from_seconds(100.0, 4.0).unwrap();
        let span = span_between(early, late).unwrap();
        assert!(close(span.std_dev(), 5.0, 1e-9), "3-4-5 in quadrature");
    }

    #[test]
    fn the_span_from_the_first_stars_to_now_is_most_of_cosmic_history() {
        let first_stars = universe::FIRST_STARS.deep_time().unwrap();
        let now = universe::AGE_OF_UNIVERSE.deep_time().unwrap();
        let span = span_between(first_stars, now).unwrap();
        let gigayears = span.in_unit(DeepUnit::Gigayear).unwrap();
        assert!(close(gigayears.value, 13.61, 0.01), "{}", gigayears.value);
        assert!(gigayears.std_dev > 0.0, "the error bar survived");
    }

    #[test]
    fn the_planck_time_and_the_present_are_sixty_one_decades_apart() {
        let planck = DeepTime::from_planck_times(1.0, 0.0).unwrap();
        let now = universe::AGE_OF_UNIVERSE.deep_time().unwrap();
        let decades = orders_of_magnitude_between(now, planck).unwrap();
        assert!(close(decades, 60.907, 1e-4), "{decades}");
    }

    #[test]
    fn placing_a_moment_and_reading_it_back_is_stable() {
        for years_ago in [0.0, 1e4, 1e6, 6.6e7, 4.567e9] {
            let placement = place_years_ago(years_ago, 0.0).unwrap();
            assert!(
                close(placement.years_ago(), years_ago, 1e-6) || years_ago == 0.0,
                "{years_ago} came back as {}",
                placement.years_ago()
            );
        }
    }

    #[test]
    fn the_finest_geologic_interval_thins_out_with_depth() {
        assert_eq!(
            place_megayears_ago(100.0, 0.0)
                .unwrap()
                .finest_geologic()
                .map(|i| i.rank),
            Some(geologic::GeologicRank::Age)
        );
        assert_eq!(
            place_megayears_ago(3000.0, 0.0)
                .unwrap()
                .finest_geologic()
                .map(|i| i.rank),
            Some(geologic::GeologicRank::Era)
        );
        assert_eq!(
            place_megayears_ago(4500.0, 0.0)
                .unwrap()
                .finest_geologic()
                .map(|i| i.rank),
            Some(geologic::GeologicRank::Eon)
        );
    }

    #[test]
    fn a_moment_before_the_big_bang_has_no_chronology_at_all() {
        let placement = place(DeepTime::from_seconds(-1.0, 0.0).unwrap()).unwrap();
        assert!(placement.cosmic_epoch.is_none());
        assert!(placement.cosmic_event.is_none());
        assert!(placement.geologic.iter().all(Option::is_none));
        assert!(placement.archaeological.is_none());
    }
}
