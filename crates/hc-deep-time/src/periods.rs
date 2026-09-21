//! Recurrences longer than a calendar can hold.
//!
//! A calendar's longest cycle is a few thousand years. Above that there are
//! still periodicities, and they still matter — a galactic year is the unit
//! the Sun's journey is told in, and the Earth's orbital cycles are what
//! paced the ice ages. They belong here rather than in `hc-units` for one
//! reason: every one of them is a *measurement*. `hc-units` holds lengths
//! that an authority defined, which are exact; these have error bars, they
//! are revised as the data improve, and several of them are not even
//! constant over the span they are quoted for.
//!
//! # Honesty about the galactic year
//!
//! [`GALACTIC_YEAR`] is the one most likely to be quoted without its error
//! bar, so it carries the largest warning. The published range is 225–250
//! Myr, and the number that reaches popular writing is 230 Myr. But that
//! figure descends from older values of the Sun's galactocentric distance
//! and circular speed; Gaia-era determinations of both push the period
//! toward the low end, near 210–230 Myr. The stored uncertainty is wide on
//! purpose. Anyone who needs a galactic year to better than ten per cent
//! needs a specific paper's parameters, not a library constant.

use hc_uncertainty::Uncertain;

use crate::error::DeepTimeResult;
use crate::magnitude::DeepTime;

/// A long astronomical recurrence, with its uncertainty and its source.
///
/// Deliberately not a [`crate::constants::PhysicalConstant`]: those are
/// CODATA quantities whose uncertainty is a standard uncertainty in a
/// well-defined sense. These are periods whose spread comes from
/// disagreement between determinations, and sometimes from the period
/// genuinely varying, which is a different kind of "± 10" and should not be
/// mistaken for the other.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AstronomicalPeriod {
    /// A stable identifier.
    pub id: &'static str,
    /// The name in English.
    pub name: &'static str,
    /// The period, in Julian years.
    pub julian_years: f64,
    /// A one-standard-deviation spread, in Julian years.
    ///
    /// Where the literature gives a range rather than an uncertainty, this
    /// is half the range, and the doc comment on the constant says so.
    pub spread_julian_years: f64,
    /// Whether the period is genuinely constant, or itself drifts.
    pub stability: Stability,
    /// Where the value came from, named well enough to look up.
    pub source: &'static str,
}

/// Whether a period stays put.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Stability {
    /// Constant to within the stated spread over the span it is quoted for.
    Steady,
    /// The period itself changes over time, so the stated value is a mean
    /// over some window and is wrong outside it.
    Drifting,
}

impl AstronomicalPeriod {
    /// The period as a Gaussian number of seconds.
    ///
    /// # Errors
    ///
    /// Propagates [`crate::DeepTimeError::NotFinite`] for a non-finite pair.
    pub fn uncertain_seconds(&self) -> DeepTimeResult<Uncertain> {
        let seconds = crate::constants::JULIAN_YEAR_SECONDS;
        Ok(Uncertain::new(
            self.julian_years * seconds,
            self.spread_julian_years * seconds,
        )?)
    }

    /// The period as a [`DeepTime`], for comparison with everything else on
    /// the logarithmic scale.
    ///
    /// # Errors
    ///
    /// As [`AstronomicalPeriod::uncertain_seconds`].
    pub fn deep_time(&self) -> DeepTimeResult<DeepTime> {
        DeepTime::from_uncertain_seconds(self.uncertain_seconds()?)
    }

    /// How many of this period fit in a span of Julian years.
    ///
    /// # Errors
    ///
    /// Propagates the uncertainty layer's division errors.
    pub fn count_in_julian_years(&self, years: Uncertain) -> DeepTimeResult<Uncertain> {
        let period = Uncertain::new(self.julian_years, self.spread_julian_years)?;
        Ok(years.checked_div(period)?)
    }
}

hc_core::catalogue! {
    type: AstronomicalPeriod,
    id: |entry| entry.id,
    sorted_by: |entry| entry.julian_years,
    provenance: |entry| entry.source,
    tests: period_catalogue,

    /// Every period in this module, shortest first.
    pub const ALL;

    /// The period with this id, if this module has one.
    pub fn by_id;

    entries: {
    /// The precession of the equinoxes: the Great Year of classical astronomy.
    ///
    /// Hipparchus found it; the modern value is about 25 772 years. It drifts:
    /// the precession rate is itself changing, so a count of cycles over
    /// millions of years is not this number times that span.
    pub const PRECESSION_OF_THE_EQUINOXES = AstronomicalPeriod {
        id: "precession-of-the-equinoxes",
        name: "precession of the equinoxes",
        julian_years: 25_772.0,
        spread_julian_years: 2.0,
        stability: Stability::Drifting,
        source: "IAU 2006 precession model (Capitaine, Wallace & Chapront 2003)",
    };

    /// The Sun's orbital period about the Galactic centre: one galactic year.
    ///
    /// The spread here is half the published 225–250 Myr range, widened rather
    /// than narrowed because Gaia-era parameters favour the low end. The Earth
    /// is about twenty galactic years old; the Sun has made roughly the same
    /// number of laps since it formed.
    pub const GALACTIC_YEAR = AstronomicalPeriod {
        id: "galactic-year",
        name: "galactic year",
        julian_years: 230e6,
        spread_julian_years: 15e6,
        stability: Stability::Drifting,
        source: "Literature range 225–250 Myr; cf. Bland-Hawthorn & Gerhard, ARA&A 54 (2016) 529",
    };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_galactic_year_is_about_seven_quadrillion_seconds() {
        let seconds = GALACTIC_YEAR
            .uncertain_seconds()
            .expect("the period is finite");
        // 230e6 × 31_557_600 = 7.258e15 s.
        assert!((seconds.value - 7.258_248e15).abs() < 1e10);
        // The point of the entry: the error bar is six per cent, not absent.
        let relative = seconds.relative().expect("the value is nonzero");
        assert!(relative > 0.06, "the galactic year's spread is not small");
    }

    /// Both periods drift, and both carry a spread. A period stated
    /// without one is a decoration, which is what the module header says
    /// and what this keeps true.
    #[test]
    fn every_period_states_a_spread_and_whether_it_drifts() {
        for period in ALL {
            assert!(period.spread_julian_years > 0.0, "{} is exact?", period.id);
            assert_eq!(period.stability, Stability::Drifting, "{}", period.id);
        }
    }

    #[test]
    fn the_earth_is_about_twenty_galactic_years_old() {
        let age = hc_uncertainty::Uncertain::new(4.567e9, 0.001e9).expect("the age is finite");
        let laps = GALACTIC_YEAR
            .count_in_julian_years(age)
            .expect("the division is defined");
        assert!(
            (18.0..=22.0).contains(&laps.value),
            "expected about 20 laps, got {}",
            laps.value
        );
        // And the answer inherits the period's uncertainty rather than
        // pretending the age's five figures carried through.
        assert!(laps.std_dev > 1.0);
    }
}
