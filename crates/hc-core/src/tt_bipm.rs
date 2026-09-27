//! TT(BIPM), the BIPM's realisations of Terrestrial Time, read from a
//! series the caller supplies.
//!
//! [`crate::Tt`] is TT(TAI): TAI plus 32.184 s, exactly and forever.
//! TT(BIPM) is a better realisation of the same scale. Each year the BIPM
//! recomputes it from the primary and secondary frequency standards and
//! publishes a realisation named for the year, TT(BIPM25) for the one
//! computed with data to December 2025, as a table of TT(BIPMxx) − TAI −
//! 32.184 s every ten days from MJD 42 589 (BIPM, `TTBIPM.2025`, the Time
//! Department's FTP server, `bipm-ttbipm-2025`, read 2026-09-27). The
//! difference is 27.67 µs at MJD 60 669, 25 December 2024. "When
//! accuracies of better than 30 µs are required, TT(BIPM) must be used"
//! (Eastman, Siverd and Gaudi, 2010, §2.2, `eastman2010`).
//!
//! The realisations are revised, so they are data and not a formula. A
//! later one replaces the recent part of an earlier one: TT(BIPM25) is
//! identical to TT(BIPM24) before MJD 58 479 and differs from it by up to
//! 0.2 ns after (the same file). This module does not carry any of them.
//! The caller supplies the realisation it trusts, as [`TtBipmSeries`], in
//! the same way a DUT1 series is supplied for UT1, and the realisation's
//! name travels with it.
//!
//! Between samples the offset is interpolated linearly in TAI; outside the
//! series it is refused. The BIPM gives a formula for extending each
//! realisation past its last sample. That is an extrapolation the caller
//! can choose by adding samples; the library does not make it.
//!
//! The reading is a [`Duration`] from the 1970 epoch of TT(BIPMxx), not an
//! [`Instant`] of [`crate::Tt`]: an `Instant<Tt>` is converted back to TAI
//! by the exact 32.184 s, and would lose the 27 µs this module is for.

use crate::duration::Duration;
use crate::error::{TimeError, TimeResult};
use crate::scale::{Instant, TT_MINUS_TAI, Tai};
use crate::unix::{LeapPolicy, tai_minus_utc_at};

/// The Modified Julian Date of 1970-01-01, the POSIX epoch.
const MJD_OF_UNIX_EPOCH: i64 = 40_587;

/// Seconds in a day.
const SECONDS_PER_DAY: i64 = 86_400;

/// A published realisation of TT(BIPM): the BIPM's table of TT(BIPMxx) −
/// TAI − 32.184 s.
#[derive(Debug, Clone, Copy)]
pub struct TtBipmSeries<'a> {
    /// The realisation's name as the BIPM writes it, such as
    /// `"TT(BIPM25)"`.
    pub realisation: &'a str,
    /// `(mjd, microseconds)` in ascending order: the first and third
    /// columns of the BIPM's file, the MJD at 0 h UTC and TT(BIPMxx) − TAI
    /// − 32.184 s there in microseconds.
    pub samples: &'a [(i64, f64)],
}

impl TtBipmSeries<'_> {
    /// The TAI reading, in seconds from the 1970 epoch, of 0 h UTC on the
    /// day with Modified Julian Date `mjd`.
    fn tai_seconds_at(mjd: i64, policy: LeapPolicy) -> TimeResult<f64> {
        let unix = (mjd - MJD_OF_UNIX_EPOCH) * SECONDS_PER_DAY;
        Ok(unix as f64 + tai_minus_utc_at(unix, policy)?.as_secs_f64())
    }

    /// TT(BIPMxx) − TT(TAI) in seconds at a TAI reading: the tabulated
    /// value, interpolated inside the series.
    ///
    /// # Errors
    ///
    /// [`TimeError::BeforeModelStart`] before the first sample or for an
    /// empty series, [`TimeError::AfterModelEnd`] after the last, and
    /// whatever `policy` makes [`tai_minus_utc_at`] return for a sample
    /// outside the leap-second table.
    pub fn offset_from_tt_tai(&self, tai: Instant<Tai>, policy: LeapPolicy) -> TimeResult<f64> {
        let (Some(&first), Some(&last)) = (self.samples.first(), self.samples.last()) else {
            return Err(TimeError::BeforeModelStart);
        };
        let x = tai.since_epoch().as_secs_f64();
        if x < Self::tai_seconds_at(first.0, policy)? {
            return Err(TimeError::BeforeModelStart);
        }
        if x > Self::tai_seconds_at(last.0, policy)? {
            return Err(TimeError::AfterModelEnd);
        }
        // The first sample whose instant is after `x`, and the one before
        // it. A sample the leap-second table cannot place sorts before `x`
        // here and fails below when it is read.
        let upper = self
            .samples
            .partition_point(|sample| {
                Self::tai_seconds_at(sample.0, policy).map_or(true, |at| at <= x)
            })
            .min(self.samples.len() - 1);
        let lower = upper.saturating_sub(1);
        let (x0, y0) = (
            Self::tai_seconds_at(self.samples[lower].0, policy)?,
            self.samples[lower].1,
        );
        let (x1, y1) = (
            Self::tai_seconds_at(self.samples[upper].0, policy)?,
            self.samples[upper].1,
        );
        let microseconds = if x1 <= x0 {
            y0
        } else {
            y0 + (y1 - y0) * (x - x0) / (x1 - x0)
        };
        Ok(microseconds * 1e-6)
    }

    /// TT(BIPMxx) − TAI at a TAI reading: 32.184 s plus the tabulated
    /// value.
    ///
    /// # Errors
    ///
    /// As [`TtBipmSeries::offset_from_tt_tai`].
    pub fn minus_tai(&self, tai: Instant<Tai>, policy: LeapPolicy) -> TimeResult<Duration> {
        let offset = Duration::from_secs_f64(self.offset_from_tt_tai(tai, policy)?)?;
        TT_MINUS_TAI.checked_add(offset)
    }

    /// The TT(BIPMxx) reading of an event, from the 1970 epoch of that
    /// scale.
    ///
    /// # Errors
    ///
    /// As [`TtBipmSeries::offset_from_tt_tai`].
    pub fn reading(&self, tai: Instant<Tai>, policy: LeapPolicy) -> TimeResult<Duration> {
        tai.since_epoch().checked_add(self.minus_tai(tai, policy)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STRICT: LeapPolicy = LeapPolicy::Strict;

    /// Rows of the BIPM's `TTBIPM.2025`, the first and third columns.
    const TT_BIPM25: [(i64, f64); 6] = [
        (42_589, 46.258),
        (42_599, 45.439),
        (58_479, 27.6740),
        (58_489, 27.6745),
        (60_659, 27.6711),
        (60_669, 27.6712),
    ];

    /// The same two days of 2024 in `TTBIPM.2024`.
    const TT_BIPM24: [(i64, f64); 2] = [(60_659, 27.6712), (60_669, 27.6713)];

    fn series(samples: &[(i64, f64)]) -> TtBipmSeries<'_> {
        TtBipmSeries {
            realisation: "TT(BIPM25)",
            samples,
        }
    }

    /// 0 h UTC on the day `mjd`, as a TAI instant.
    fn tai_at(mjd: i64) -> Instant<Tai> {
        let seconds = TtBipmSeries::tai_seconds_at(mjd, STRICT).unwrap();
        Instant::from_epoch(Duration::from_secs(seconds as i128))
    }

    #[test]
    fn a_sample_reads_as_the_table_gives_it() {
        // TTBIPM.2025: 27.6740 µs on MJD 58 479, 46.258 µs on MJD 42 589.
        let bipm25 = series(&TT_BIPM25);
        let offset = bipm25.offset_from_tt_tai(tai_at(58_479), STRICT).unwrap();
        assert!((offset - 27.674e-6).abs() < 1e-12, "{offset}");
        let first = bipm25.offset_from_tt_tai(tai_at(42_589), STRICT).unwrap();
        assert!((first - 46.258e-6).abs() < 1e-12, "{first}");
        // TT(BIPM25) − TAI is 32.184 s and 27.6740 µs.
        let minus = bipm25.minus_tai(tai_at(58_479), STRICT).unwrap();
        let expected = TT_MINUS_TAI
            .checked_add(Duration::from_secs_f64(27.674e-6).unwrap())
            .unwrap();
        let error = minus.checked_sub(expected).unwrap().as_secs_f64().abs();
        assert!(error < 1e-15, "{error}");
        let reading = bipm25.reading(tai_at(58_479), STRICT).unwrap();
        assert_eq!(
            reading,
            tai_at(58_479).since_epoch().checked_add(minus).unwrap()
        );
    }

    #[test]
    fn between_samples_the_offset_is_interpolated() {
        let bipm25 = series(&TT_BIPM25);
        // Five days after MJD 58 479: halfway to 27.6745 µs.
        let midway = tai_at(58_484);
        let offset = bipm25.offset_from_tt_tai(midway, STRICT).unwrap();
        assert!((offset - 27.674_25e-6).abs() < 1e-12, "{offset}");
    }

    #[test]
    fn two_realisations_of_one_day_differ() {
        // MJD 60 669: 27.6712 µs in TT(BIPM25), 27.6713 µs in TT(BIPM24).
        let day = tai_at(60_669);
        let bipm25 = series(&TT_BIPM25).offset_from_tt_tai(day, STRICT).unwrap();
        let bipm24 = TtBipmSeries {
            realisation: "TT(BIPM24)",
            samples: &TT_BIPM24,
        }
        .offset_from_tt_tai(day, STRICT)
        .unwrap();
        assert!(
            ((bipm24 - bipm25) - 0.1e-9).abs() < 1e-13,
            "{bipm24} {bipm25}"
        );
    }

    #[test]
    fn a_series_refuses_to_extrapolate() {
        let bipm25 = series(&TT_BIPM25);
        let before =
            Instant::<Tai>::from_epoch(tai_at(42_589).since_epoch() - Duration::from_secs(1));
        assert_eq!(
            bipm25.offset_from_tt_tai(before, STRICT),
            Err(TimeError::BeforeModelStart)
        );
        let after =
            Instant::<Tai>::from_epoch(tai_at(60_669).since_epoch() + Duration::from_secs(1));
        assert_eq!(
            bipm25.offset_from_tt_tai(after, STRICT),
            Err(TimeError::AfterModelEnd)
        );
        assert_eq!(
            series(&[]).minus_tai(tai_at(58_479), STRICT),
            Err(TimeError::BeforeModelStart)
        );
    }
}
