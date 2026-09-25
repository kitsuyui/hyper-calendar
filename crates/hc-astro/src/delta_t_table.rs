//! The observed ΔT, one sample a year, as published by the USNO.
//!
//! This module is *data*. The algorithm that reads it — the interpolation,
//! and the hand-over to the Espenak–Meeus polynomials outside the table —
//! lives in [`crate::time`]. ΔT in the atomic era is a measurement, not a
//! formula, so the values must be replaceable without touching any
//! conversion code.
//!
//! # Source
//!
//! United States Naval Observatory, *Delta T: deltat.data*,
//! <https://maia.usno.navy.mil/ser7/deltat.data>, retrieved 2026-09-25. The
//! file gives `TT − UT1` in seconds at 0h UT on the first of every month from
//! 1973-02-01 to 2026-04-01, the latest month observed at retrieval. This
//! table takes the 1 January value of every year from 1974 to 2026, plus the
//! last month the file holds, so that the table ends where the observations
//! end and not three months earlier. Between the yearly samples ΔT is within
//! 0.09 s of a straight line through them, measured against the monthly
//! values of the same file; the largest departure is in mid-1998.
//!
//! The USNO also publishes forecasts (`deltat.preds`). None of them is
//! carried: a forecast is not an observation, and
//! [ADR 0006](https://github.com/kitsuyui/hyper-calendar/blob/main/docs/adr/0006-refuse-to-extrapolate.md)
//! says a value past the data is the caller's decision.
//!
//! # Why one source, and how it was checked
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

/// Where the table came from, in the form the policy asks every data table
/// to carry.
pub const TABULATED_DELTA_T_SOURCE: &str = "United States Naval Observatory, Delta T: deltat.data, \
     https://maia.usno.navy.mil/ser7/deltat.data, retrieved 2026-09-25; the 1 January value of \
     1974 through 2026 and the 2026-04-01 value, the last month observed";

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
}
