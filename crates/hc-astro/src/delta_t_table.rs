//! The observed ΔT, one sample a year, and the predicted ΔT, one a
//! quarter, as published by the USNO.
//!
//! This module is *data*. The algorithm that reads it — the interpolation,
//! the precedence of the observations over the predictions, and the
//! hand-over to the Espenak–Meeus polynomials outside both — lives in
//! [`crate::time`]. ΔT in the atomic era is a measurement, not a formula,
//! so the values must be replaceable without touching any conversion code.
//!
//! # The observations
//!
//! United States Naval Observatory, *Delta T: deltat.data*,
//! <https://maia.usno.navy.mil/ser7/deltat.data>, retrieved 2026-09-25. The
//! file gives `TT − UT1` in seconds at 0h UT on the first of every month from
//! 1973-02-01 to 2026-04-01, the latest month observed at retrieval.
//! [`TABULATED_DELTA_T`] takes the 1 January value of every year from 1974
//! to 2026, plus the last month the file holds, so that the table ends
//! where the observations end and not three months earlier. Between the
//! yearly samples ΔT is within 0.09 s of a straight line through them,
//! measured against the monthly values of the same file; the largest
//! departure is in mid-1998.
//!
//! # The predictions
//!
//! United States Naval Observatory, *Delta T: deltat.preds*,
//! <https://maia.usno.navy.mil/ser7/deltat.preds>, retrieved 2026-09-25.
//! The file's columns are the Modified Julian Date, the same date as a
//! decimal year to two places, `TT − UT` in seconds, `UT1 − UTC` where it
//! had been observed when the file was made, and the prediction's error in
//! seconds; its rows are quarterly from MJD 59762 (2022-07-02) to MJD 63871
//! (2033-10-01). [`PREDICTED_DELTA_T`] carries every row's date, value and
//! error; the `UT1 − UTC` column is not carried, since the observations
//! above already say what UT1 was. The error column stops at 1 s, which is
//! where the file stops stating one and not where the uncertainty stops
//! growing.
//!
//! A prediction is not an observation, and the file's early rows overlap
//! the observed table. There the observation wins: [`crate::time::delta_t`]
//! reads the predictions only after the observed table's last sample. The
//! overlap is also a measurement of the predictions: against the
//! observations that have since arrived, the rows from 2022-07 to 2026-01
//! differ from the observed table, as [`crate::time`] interpolates it, by
//! +0.04 s to −0.12 s, the predictions running low, and from 2023-10 to
//! 2025-01 the difference is up to 2.7 times the error the file states for
//! the row (against the USNO's own monthly observations the figures are
//! −0.10 s and 2.5 times). A caller reading the error column should read
//! it as the file's own estimate, which the record so far has exceeded.
//!
//! Carrying the predictions is a choice against the first reading of
//! [ADR 0006](https://github.com/kitsuyui/hyper-calendar/blob/main/docs/adr/0006-refuse-to-extrapolate.md),
//! and it is made because the alternative is worse: the crate cannot
//! refuse to answer ΔT, since every date needs one, and past the
//! observations the only other answer is a polynomial forecast made in
//! 2006 that is now six seconds high, where the USNO's forecast of this
//! year is within a tenth of a second of what was then observed. The
//! predictions are the better of two forecasts, they are the authority's
//! own and not this crate's, and nothing here extends them: past their
//! last row the polynomial answers, with the step left visible.
//! [`crate::time::delta_t_regime`] tells a caller which of the three
//! answered.
//!
//! # Why one observed source, and how it was checked
//!
//! In the atomic era ΔT is not independent of the leap-second table that
//! `hc-core` carries: `TT = TAI + 32.184 s` by definition, and
//! `UT1 = UTC + DUT1`, so
//!
//! ```text
//! ΔT = 32.184 s + (TAI − UTC) − DUT1.
//! ```
//!
//! A DUT1 series such as the IERS EOP 20 C04 would therefore say the same
//! thing as this table, and carrying both would be two statements of one
//! fact. The USNO file is the one carried because it is already in the unit
//! the astronomy wants and because it is the file everybody quotes; the
//! identity is what the tests use to *check* it. Against the C04 series
//! (<https://hpiers.obspm.fr/iers/eop/eopc04/eopc04.1962-now>, retrieved
//! 2026-09-25) and `hc_core::leap::TABLE`, every sample here agrees with the
//! identity to within 0.002 s, the difference being later revisions of the
//! C04 values for the 1970s and 1980s.

/// One sample of the observed ΔT: `TT − UT1` in seconds at 0h UT on the
/// first day of a month.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeltaTSample {
    /// The Gregorian year of the sample.
    pub year: i64,
    /// The month, 1–12, on whose first day at 0h UT the sample was taken.
    pub month: u8,
    /// `TT − UT1` in seconds.
    pub seconds: f64,
}

const fn sample(year: i64, month: u8, seconds: f64) -> DeltaTSample {
    DeltaTSample {
        year,
        month,
        seconds,
    }
}

/// The observed ΔT at 0h UT on 1 January of every year from 1974 to 2026,
/// and on 2026-04-01, the last month observed. Ascending in time.
///
/// See the [module documentation](self) for the source and the check.
pub const TABULATED_DELTA_T: &[DeltaTSample] = &[
    sample(1974, 1, 44.4841),
    sample(1975, 1, 45.4761),
    sample(1976, 1, 46.4567),
    sample(1977, 1, 47.5214),
    sample(1978, 1, 48.5344),
    sample(1979, 1, 49.5861),
    sample(1980, 1, 50.5387),
    sample(1981, 1, 51.3808),
    sample(1982, 1, 52.1668),
    sample(1983, 1, 52.9565),
    sample(1984, 1, 53.7882),
    sample(1985, 1, 54.3427),
    sample(1986, 1, 54.8713),
    sample(1987, 1, 55.3222),
    sample(1988, 1, 55.8197),
    sample(1989, 1, 56.3000),
    sample(1990, 1, 56.8553),
    sample(1991, 1, 57.5653),
    sample(1992, 1, 58.3092),
    sample(1993, 1, 59.1218),
    sample(1994, 1, 59.9845),
    sample(1995, 1, 60.7853),
    sample(1996, 1, 61.6287),
    sample(1997, 1, 62.2950),
    sample(1998, 1, 62.9659),
    sample(1999, 1, 63.4673),
    sample(2000, 1, 63.8285),
    sample(2001, 1, 64.0908),
    sample(2002, 1, 64.2998),
    sample(2003, 1, 64.4734),
    sample(2004, 1, 64.5736),
    sample(2005, 1, 64.6876),
    sample(2006, 1, 64.8452),
    sample(2007, 1, 65.1464),
    sample(2008, 1, 65.4573),
    sample(2009, 1, 65.7768),
    sample(2010, 1, 66.0699),
    sample(2011, 1, 66.3246),
    sample(2012, 1, 66.6030),
    sample(2013, 1, 66.9069),
    sample(2014, 1, 67.2810),
    sample(2015, 1, 67.6439),
    sample(2016, 1, 68.1024),
    sample(2017, 1, 68.5927),
    sample(2018, 1, 68.9676),
    sample(2019, 1, 69.2202),
    sample(2020, 1, 69.3612),
    sample(2021, 1, 69.3594),
    sample(2022, 1, 69.2945),
    sample(2023, 1, 69.2039),
    sample(2024, 1, 69.1752),
    sample(2025, 1, 69.1377),
    sample(2026, 1, 69.1099),
    sample(2026, 4, 69.1330),
];

/// Where the observed table came from, in the form the policy asks every
/// data table to carry.
pub const TABULATED_DELTA_T_SOURCE: &str = "United States Naval Observatory, Delta T: deltat.data, \
     https://maia.usno.navy.mil/ser7/deltat.data, retrieved 2026-09-25; the 1 January value of \
     1974 through 2026 and the 2026-04-01 value, the last month observed";

/// One row of the USNO's ΔT predictions: `TT − UT1` in seconds at 0h UT on
/// a Modified Julian Date, with the error the file states for it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeltaTPredictionSample {
    /// The Modified Julian Date of the row, as the file gives it.
    pub mjd: i64,
    /// `TT − UT1` in seconds.
    pub seconds: f64,
    /// The file's stated error of the prediction, in seconds. The file
    /// stops stating one past 1 s and prints `1` instead.
    pub uncertainty: f64,
}

const fn prediction(mjd: i64, seconds: f64, uncertainty: f64) -> DeltaTPredictionSample {
    DeltaTPredictionSample {
        mjd,
        seconds,
        uncertainty,
    }
}

/// The USNO's predicted ΔT, every quarterly row of `deltat.preds` from
/// 2022-07-02 to 2033-10-01. Ascending in time.
///
/// See the [module documentation](self) for the source, the overlap with
/// the observations and why the rows before 2026-04-01 are not the ones
/// [`crate::time::delta_t`] reads.
pub const PREDICTED_DELTA_T: &[DeltaTPredictionSample] = &[
    prediction(59762, 69.29, 0.031),
    prediction(59853, 69.21, 0.021),
    prediction(59945, 69.21, 0.019),
    prediction(60036, 69.20, 0.021),
    prediction(60127, 69.18, 0.024),
    prediction(60219, 69.11, 0.027),
    prediction(60310, 69.11, 0.033),
    prediction(60401, 69.11, 0.043),
    prediction(60493, 69.10, 0.056),
    prediction(60584, 69.03, 0.070),
    prediction(60675, 69.04, 0.088),
    prediction(60767, 69.07, 0.109),
    prediction(60858, 69.06, 0.133),
    prediction(60949, 69.01, 0.159),
    prediction(61041, 69.05, 0.189),
    prediction(61132, 69.09, 0.223),
    prediction(61223, 69.11, 0.257),
    prediction(61314, 69.09, 0.291),
    prediction(61406, 69.14, 0.327),
    prediction(61497, 69.21, 0.367),
    prediction(61588, 69.26, 0.408),
    prediction(61680, 69.26, 0.446),
    prediction(61771, 69.34, 0.486),
    prediction(61862, 69.44, 0.525),
    prediction(61954, 69.51, 0.566),
    prediction(62045, 69.54, 0.603),
    prediction(62136, 69.63, 0.637),
    prediction(62228, 69.75, 0.672),
    prediction(62319, 69.83, 0.711),
    prediction(62410, 69.87, 0.742),
    prediction(62502, 69.97, 0.768),
    prediction(62593, 70.08, 0.794),
    prediction(62684, 70.17, 0.823),
    prediction(62775, 70.21, 0.849),
    prediction(62867, 70.32, 0.871),
    prediction(62958, 70.42, 0.891),
    prediction(63049, 70.51, 0.913),
    prediction(63141, 70.53, 0.926),
    prediction(63232, 70.62, 0.937),
    prediction(63323, 70.72, 0.952),
    prediction(63415, 70.82, 0.975),
    prediction(63506, 70.86, 1.000),
    prediction(63597, 70.98, 1.000),
    prediction(63689, 71.10, 1.000),
    prediction(63780, 71.20, 1.000),
    prediction(63871, 71.25, 1.000),
];

/// Where the predictions came from.
pub const PREDICTED_DELTA_T_SOURCE: &str = "United States Naval Observatory, Delta T: deltat.preds, \
     https://maia.usno.navy.mil/ser7/deltat.preds, retrieved 2026-09-25; every quarterly row, \
     MJD 59762 (2022-07-02) to MJD 63871 (2033-10-01), with its stated error";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_samples_are_in_ascending_time_order_and_on_real_months() {
        for pair in TABULATED_DELTA_T.windows(2) {
            let (earlier, later) = (pair[0], pair[1]);
            assert!(
                (earlier.year, earlier.month) < (later.year, later.month),
                "{}-{} is not before {}-{}",
                earlier.year,
                earlier.month,
                later.year,
                later.month
            );
        }
        for sample in TABULATED_DELTA_T {
            assert!((1..=12).contains(&sample.month), "{sample:?}");
            assert!(sample.seconds.is_finite(), "{sample:?}");
        }
    }

    #[test]
    fn there_is_one_sample_for_every_new_year_from_1974_to_2026() {
        let january = TABULATED_DELTA_T
            .iter()
            .filter(|sample| sample.month == 1)
            .map(|sample| sample.year);
        assert!(january.eq(1974..=2026));
        assert_eq!(TABULATED_DELTA_T.len(), 54);
    }

    /// The rows are a quarter apart — 91 or 92 days — and from 2023 on,
    /// past the rows that had been observed when the file was made, the
    /// error the file states never shrinks with time.
    #[test]
    fn the_predictions_are_quarterly_with_a_growing_stated_error() {
        assert_eq!(PREDICTED_DELTA_T.len(), 46);
        assert_eq!(PREDICTED_DELTA_T[0].mjd, 59_762);
        assert_eq!(PREDICTED_DELTA_T[45].mjd, 63_871);
        for pair in PREDICTED_DELTA_T.windows(2) {
            let gap = pair[1].mjd - pair[0].mjd;
            assert!(
                (91..=92).contains(&gap),
                "gap of {gap} days at MJD {}",
                pair[0].mjd
            );
        }
        for pair in PREDICTED_DELTA_T[2..].windows(2) {
            assert!(
                pair[1].uncertainty >= pair[0].uncertainty,
                "at MJD {}",
                pair[1].mjd
            );
        }
        for sample in PREDICTED_DELTA_T {
            assert!(
                sample.seconds.is_finite() && sample.uncertainty > 0.0,
                "{sample:?}"
            );
            assert!(sample.uncertainty <= 1.0, "{sample:?}");
        }
    }
}
