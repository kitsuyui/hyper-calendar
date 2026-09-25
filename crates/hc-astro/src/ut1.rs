//! UT1, the time scale of the Earth's rotation.
//!
//! UT1 is measured, not defined: the IERS derives it from VLBI and publishes
//! `UT1 − UTC` (DUT1) in Bulletins A and B. This module gives two readings of
//! it, and which one a caller gets is visible in the type they use.
//!
//! * [`Ut1Offsets`] reads UT1 from a published series: `UT1 = UTC + DUT1`.
//!   It is as good as the series, and refuses to answer outside it.
//! * [`Ut1`], the [`TimeScale`] marker, needs no series. It reads
//!   `UT1 = TT − ΔT` from this crate's ΔT model, [`delta_t`].
//!
//! # How good the model is
//!
//! From 1974-01-01 to 2026-04-01 ΔT is the USNO's observed value, one
//! sample a year, interpolated between them (see [`crate::delta_t_table`]).
//! In the atomic era that observation *is* `32.184 s + (TAI − UTC) − DUT1`,
//! so at the samples the model's UT1 agrees with the IERS EOP 20 C04 series
//! to a few milliseconds, and between them to the 0.09 s that a straight
//! line through yearly points misses the monthly values by.
//!
//! From 2026-04-01 to 2033-10-01 ΔT is the USNO's prediction, one row a
//! quarter, interpolated. Against the C04 series where it now reaches past
//! the observed table, the predicted UT1 is 0.06 s late on 2026-07-01; the
//! USNO's own stated error for the predictions grows from 0.2 s there to
//! 1 s by 2032, and the rows the observations have since overtaken ran low
//! by up to 0.12 s, more than they stated.
//!
//! After 2033-10-01 the Espenak–Meeus polynomial takes over. Its 2005–2050
//! segment is a forecast made in 2006, and ΔT has since grown more slowly
//! than it forecast, so the polynomial's UT1 starts 8.9 s behind the last
//! prediction at the hand-over and the gap grows at the forecast's slope,
//! about 0.7 s a year, until the tables are extended. Before 1974 the
//! error is the polynomial's own, which the crate README describes: 0.1 s
//! at the join, seconds in 1900, minutes in 1000 CE, hours before 500 BCE.
//!
//! From 1972 the IERS keeps `|UT1 − UTC| < 0.9 s` by scheduling leap seconds
//! (ITU-R Recommendation TF.460-6), so past the predictions' end a UTC
//! reading is itself closer to UT1 than this model. A caller who needs UT1
//! to better than that there needs a DUT1 series.
//!
//! [`TimeScale`]: hc_core::TimeScale

use hc_calendar::fixed::{Moment, RD_OF_UNIX_EPOCH};
use hc_core::scale::TT_MINUS_TAI;
use hc_core::unix::{LeapPolicy, tai_minus_utc_at, utc_from_tai};
use hc_core::{Duration, Instant, Tai, TimeError, TimeResult, TimeScale, TimeScaleId};

use crate::time::{SECONDS_PER_DAY, delta_t};

/// UT1 — Universal Time, the scale of the Earth's rotation angle, read from
/// the ΔT model: `UT1 = TT − ΔT`.
///
/// The accuracy is the model's; the [module documentation](self) gives it
/// year by year against the IERS series. For UT1 from measured values, use
/// [`Ut1Offsets`].
///
/// ```
/// use hc_astro::ut1::Ut1;
/// use hc_core::{Duration, Instant, Tai};
///
/// // 2000-01-01T00:00:00 UTC, when TAI − UTC was 32 s.
/// let tai = Instant::<Tai>::from_epoch(Duration::from_secs(946_684_800 + 32));
/// let ut1: Instant<Ut1> = tai.convert();
/// // The IERS gives UT1 − UTC = +0.3555 s that day; the observed ΔT the
/// // model carries for 2000-01-01 is the same measurement.
/// let dut1 = ut1.since_epoch().as_secs_f64() - 946_684_800.0;
/// assert!((dut1 - 0.3555).abs() < 0.001, "UT1 - UTC was {dut1}");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Ut1;

/// The [`Moment`] of a reading measured in seconds from 1970-01-01T00:00:00.
fn moment_of(seconds_since_1970: f64) -> Moment {
    Moment(RD_OF_UNIX_EPOCH as f64 + seconds_since_1970 / SECONDS_PER_DAY)
}

/// `base` moved by `offset_secs`, saturating at the ends of [`Duration`]
/// rather than wrapping.
///
/// ΔT is small against the reading for any date a calendar reaches, so it is
/// computed in `f64` and applied to the exact reading, as `hc-core` does for
/// TDB.
fn shifted(base: Duration, offset_secs: f64) -> Duration {
    let saturated = if offset_secs < 0.0 {
        Duration::MIN
    } else {
        Duration::MAX
    };
    match Duration::from_secs_f64(offset_secs) {
        Ok(delta) => base.checked_add(delta).unwrap_or(saturated),
        Err(_) => saturated,
    }
}

impl TimeScale for Ut1 {
    const ID: TimeScaleId = TimeScaleId::Ut1;

    fn from_tai(tai: Duration) -> Duration {
        let tt = tai.checked_add(TT_MINUS_TAI).unwrap_or(Duration::MAX);
        let tt_secs = tt.as_secs_f64();
        // `delta_t` takes a Universal Time moment, which is the answer being
        // sought, so solve `UT1 = TT − ΔT(UT1)` by fixed-point iteration. ΔT
        // changes by at most seconds a year, so three rounds converge far
        // below the model's own error, and `to_tai` then inverts this exactly.
        let mut delta = delta_t(moment_of(tt_secs));
        for _ in 0..3 {
            delta = delta_t(moment_of(tt_secs - delta));
        }
        shifted(tt, -delta)
    }

    fn to_tai(value: Duration) -> Duration {
        let delta = delta_t(moment_of(value.as_secs_f64()));
        shifted(value, delta)
            .checked_sub(TT_MINUS_TAI)
            .unwrap_or(Duration::MIN)
    }
}

/// A published series of `UT1 − UTC` (DUT1), such as IERS Bulletin B or the
/// EOP C04 series, from which UT1 is read as `UTC + DUT1`.
///
/// The Earth's rotation is not predictable, so these values are measured.
/// Supply the series you trust; the library will not invent it for you, and
/// every reading refuses to extrapolate outside the samples.
///
/// Between samples the reading interpolates `UT1 − TAI` linearly rather than
/// DUT1 itself. DUT1 steps by a whole second at every leap second, and
/// interpolating across the step would put up to a second of error into the
/// day before it; `UT1 − TAI` is continuous.
#[derive(Debug, Clone, Copy, Default)]
pub struct Ut1Offsets<'a> {
    /// `(unix_seconds, dut1_seconds)` samples in ascending time order: the
    /// POSIX timestamp of each published epoch, usually 0h UTC, and DUT1
    /// there in seconds.
    pub samples: &'a [(i64, f64)],
}

impl Ut1Offsets<'_> {
    /// `UT1 − TAI` in seconds at a TAI reading, interpolated inside the
    /// series.
    ///
    /// # Errors
    ///
    /// [`TimeError::BeforeModelStart`] before the first sample and
    /// [`TimeError::AfterModelEnd`] after the last, and whatever `policy`
    /// makes [`tai_minus_utc_at`] return for a sample outside the
    /// leap-second table.
    pub fn ut1_minus_tai(&self, tai: Instant<Tai>, policy: LeapPolicy) -> TimeResult<f64> {
        let (Some(&first), Some(&last)) = (self.samples.first(), self.samples.last()) else {
            return Err(TimeError::BeforeModelStart);
        };
        let at = |sample: (i64, f64)| -> TimeResult<(f64, f64)> {
            let tai_minus_utc = tai_minus_utc_at(sample.0, policy)?.as_secs_f64();
            Ok((sample.0 as f64 + tai_minus_utc, sample.1 - tai_minus_utc))
        };
        let x = tai.since_epoch().as_secs_f64();
        let (x_first, y_first) = at(first)?;
        let (x_last, y_last) = at(last)?;
        if x < x_first {
            return Err(TimeError::BeforeModelStart);
        }
        if x > x_last {
            return Err(TimeError::AfterModelEnd);
        }
        if x == x_last {
            return Ok(y_last);
        }
        // The samples are ordered by UTC, and UTC is monotonic in TAI, so the
        // bracket can be found by UTC; inside a leap second the UTC label is
        // the following second, which still brackets correctly because the
        // interpolation is done in TAI.
        let utc = utc_from_tai(tai, policy)?.unix_seconds;
        let index = self.samples.partition_point(|&(t, _)| t <= utc);
        if index == 0 {
            return Ok(y_first);
        }
        let upper = index.min(self.samples.len() - 1);
        let lower = upper.saturating_sub(1);
        let (x0, y0) = at(self.samples[lower])?;
        let (x1, y1) = at(self.samples[upper])?;
        if x1 <= x0 {
            return Ok(y0);
        }
        Ok(y0 + (y1 - y0) * (x - x0) / (x1 - x0))
    }

    /// DUT1 = `UT1 − UTC` in seconds at a POSIX timestamp.
    ///
    /// # Errors
    ///
    /// As [`Ut1Offsets::ut1_minus_tai`].
    pub fn dut1_at(&self, unix_seconds: i64, policy: LeapPolicy) -> TimeResult<f64> {
        let tai_minus_utc = tai_minus_utc_at(unix_seconds, policy)?;
        let tai = Duration::from_secs(i128::from(unix_seconds)).checked_add(tai_minus_utc)?;
        let ut1_minus_tai = self.ut1_minus_tai(Instant::from_epoch(tai), policy)?;
        Ok(ut1_minus_tai + tai_minus_utc.as_secs_f64())
    }

    /// The UT1 reading of an event, from the series.
    ///
    /// # Errors
    ///
    /// As [`Ut1Offsets::ut1_minus_tai`].
    pub fn ut1_from_tai(&self, tai: Instant<Tai>, policy: LeapPolicy) -> TimeResult<Instant<Ut1>> {
        let offset = Duration::from_secs_f64(self.ut1_minus_tai(tai, policy)?)?;
        Ok(Instant::from_epoch(tai.since_epoch().checked_add(offset)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STRICT: LeapPolicy = LeapPolicy::Strict;

    /// 1 January of 2000, 2016 and 2026, and 1 July 2026, at 0h UTC.
    const Y2000: i64 = 946_684_800;
    const Y2016: i64 = 1_451_606_400;
    const Y2026: i64 = 1_767_225_600;
    const JULY_2026: i64 = 1_782_864_000;

    fn tai_at(unix_seconds: i64, tai_minus_utc: i128) -> Instant<Tai> {
        Instant::from_epoch(Duration::from_secs(
            i128::from(unix_seconds) + tai_minus_utc,
        ))
    }

    fn dut1_by_model(unix_seconds: i64, tai_minus_utc: i128) -> f64 {
        let ut1: Instant<Ut1> = tai_at(unix_seconds, tai_minus_utc).convert();
        ut1.since_epoch().as_secs_f64() - unix_seconds as f64
    }

    /// `UT1 − UTC` at 0h UTC from the IERS EOP 20 C04 series
    /// (hpiers.obspm.fr/iers/eop/eopc04/eopc04.1962-now, retrieved
    /// 2026-09-25): +0.3554724 s on 2000-01-01, +0.0740869 s on 2026-01-01,
    /// +0.0144916 s on 2026-07-01. The first two are inside the observed ΔT
    /// table and the third is past it, on the USNO's predictions; the
    /// errors are the ones the module documentation states, and the
    /// polynomial's is what the third would be without the predictions.
    #[test]
    fn the_model_matches_the_iers_series_inside_the_table_and_the_predictions_after() {
        let error_2000 = dut1_by_model(Y2000, 32) - 0.355_472_4;
        assert!(error_2000.abs() < 0.001, "error in 2000 was {error_2000}");
        let error_2026 = dut1_by_model(Y2026, 37) - 0.074_086_9;
        assert!(error_2026.abs() < 0.001, "error in 2026 was {error_2026}");
        let error_july_2026 = dut1_by_model(JULY_2026, 37) - 0.014_491_6;
        assert!(
            (error_july_2026 - 0.06).abs() < 0.01,
            "error in July 2026 was {error_july_2026}"
        );
        let observed_july_2026 = 32.184 + 37.0 - 0.014_491_6;
        let polynomial_error = observed_july_2026
            - crate::time::delta_t_polynomial(crate::time::decimal_year(moment_of(
                JULY_2026 as f64,
            )));
        assert!(
            (polynomial_error + 6.2).abs() < 0.1,
            "the polynomial alone would be {polynomial_error} s out in July 2026"
        );
    }

    /// UT1 is TAI less `TAI − UTC` (37 s) plus DUT1, so it trails TAI by
    /// tens of seconds today.
    #[test]
    fn the_model_trails_tai_by_about_tai_minus_utc() {
        let ut1: Instant<Ut1> = tai_at(Y2026, 37).convert();
        let behind_tai =
            tai_at(Y2026, 37).since_epoch().as_secs_f64() - ut1.since_epoch().as_secs_f64();
        assert!((30.0..50.0).contains(&behind_tai), "{behind_tai}");
    }

    #[test]
    fn the_model_round_trips_from_antiquity_to_the_far_future() {
        for seconds in [
            -100_000_000_000i128,
            -2_000_000_000,
            0,
            Y2026.into(),
            10_000_000_000,
        ] {
            let tai = Instant::<Tai>::from_epoch(Duration::from_secs(seconds));
            let ut1: Instant<Ut1> = tai.convert();
            let back: Instant<Tai> = ut1.convert();
            let error = back.duration_since(tai).unwrap().as_secs_f64().abs();
            assert!(error < 1e-3, "round trip error {error} s at {seconds}");
        }
    }

    /// The C04 samples either side of the leap second at the end of 2016:
    /// UT1 − UTC = −0.4077697 s on 2016-12-31 and +0.5912870 s on
    /// 2017-01-01, a second apart because of the leap second between them.
    const ACROSS_2016_LEAP: [(i64, f64); 2] =
        [(1_483_142_400, -0.407_769_7), (1_483_228_800, 0.591_287)];

    #[test]
    fn a_series_gives_ut1_as_utc_plus_dut1_at_its_samples() {
        let offsets = Ut1Offsets {
            samples: &ACROSS_2016_LEAP,
        };
        let ut1 = offsets
            .ut1_from_tai(tai_at(1_483_228_800, 37), STRICT)
            .unwrap();
        let dut1 = ut1.since_epoch().as_secs_f64() - 1_483_228_800.0;
        assert!((dut1 - 0.591_287).abs() < 1e-6, "{dut1}");
        let at_sample = offsets.dut1_at(1_483_142_400, STRICT).unwrap();
        assert!((at_sample + 0.407_769_7).abs() < 1e-6, "{at_sample}");
    }

    #[test]
    fn a_series_interpolates_ut1_minus_tai_across_a_leap_second() {
        let offsets = Ut1Offsets {
            samples: &ACROSS_2016_LEAP,
        };
        // Noon on 2016-12-31, before the leap second: UT1 − TAI runs from
        // −36.4077697 s to −36.408713 s, so DUT1 there is −0.40824 s, not
        // the +0.09 s that interpolating DUT1 itself would give.
        let noon = offsets.dut1_at(1_483_142_400 + 43_200, STRICT).unwrap();
        assert!((noon + 0.408_24).abs() < 1e-5, "{noon}");
    }

    #[test]
    fn a_series_refuses_to_extrapolate() {
        let offsets = Ut1Offsets {
            samples: &ACROSS_2016_LEAP,
        };
        assert_eq!(
            offsets.dut1_at(1_483_142_399, STRICT),
            Err(TimeError::BeforeModelStart)
        );
        assert_eq!(
            offsets.dut1_at(1_483_228_801, STRICT),
            Err(TimeError::AfterModelEnd)
        );
        assert_eq!(
            Ut1Offsets::default().dut1_at(Y2016, STRICT),
            Err(TimeError::BeforeModelStart)
        );
    }
}
