//! Universal Time, Terrestrial Time, and the correction between them.
//!
//! Every series in this crate is stated in Terrestrial Time (TT), a uniform
//! scale, while a [`Moment`] is a Universal Time reading — the time scale a
//! calendar actually cares about, because a calendar day is a day of the
//! rotating Earth. The bridge between them is ΔT = TT − UT1, which is not a
//! formula but an observation: the Earth's rotation is irregular, so ΔT can
//! only be measured, and outside the measurements fitted or extrapolated.
//!
//! [`delta_t`] therefore answers from two sources, and [`delta_t_regime`]
//! says which:
//!
//! * From 1974-01-01 to 2026-04-01, the **observed** values the USNO
//!   publishes, one a year, interpolated linearly between them. The table
//!   is [`crate::delta_t_table`], with its source and the check against the
//!   leap-second table; [`TABULATED_DELTA_T_FIRST`] and
//!   [`TABULATED_DELTA_T_LAST`] are its ends. It is not extrapolated.
//! * Outside that, the NASA/Espenak–Meeus "Polynomial Expressions for
//!   Delta T" (Espenak & Meeus, *Five Millennium Canon of Solar Eclipses*,
//!   NASA/TP-2006-214141, and the derived polynomial set), which covers
//!   −1999 to +3000 in fifteen segments, with the parabola
//!   ΔT = −20 + 32u², u = (year − 1820)/100, used outside −500…+2150.
//!   [`delta_t_polynomial`] is that fit alone.
//!
//! The polynomial segments meet at their joins to a few tenths of a second,
//! which is well inside the uncertainty of the underlying eclipse-timing
//! data, so this crate does not smooth them. The table meets the polynomial
//! to 0.1 s at its start. At its end it does not: the 2005–2050 segment is a
//! forecast made in 2006, and the Earth has since rotated faster than it
//! forecast, so on 2026-04-01 the polynomial is 6.1 s above the observed
//! value and the step is left visible rather than hidden by a blend. A
//! Universal Time instant computed after the table's end is therefore about
//! six seconds early, and the gap grows with the forecast's slope until the
//! table is extended.

use hc_calendar::Rd;
use hc_calendar::fixed::Moment;

use crate::delta_t_table::{DeltaTSample, TABULATED_DELTA_T};
use crate::util::poly;

/// The J2000.0 epoch as a Rata Die moment: 2000-01-01 12:00 TT.
///
/// `RD 730120` is 2000-01-01 in the proleptic Gregorian calendar, and the
/// epoch is at noon, so the moment is half a day later.
pub const J2000: Moment = Moment(730_120.5);

/// Days in a Julian century, the unit every one of these series uses.
pub const JULIAN_CENTURY_DAYS: f64 = 36_525.0;

/// Seconds in a day, as the astronomical series count them: a day is exactly
/// 86 400 seconds here, with no leap second, because ΔT already carries the
/// whole of the Earth's rotational irregularity.
pub const SECONDS_PER_DAY: f64 = 86_400.0;

/// The earliest year over which the ΔT fit is anything other than an
/// extrapolation.
pub const EARLIEST_FITTED_YEAR: f64 = -500.0;

/// The latest year over which the ΔT fit is anything other than an
/// extrapolation.
pub const LATEST_FITTED_YEAR: f64 = 2150.0;

/// The first decimal year the observed ΔT table covers: 1974-01-01.
pub const TABULATED_DELTA_T_FIRST: f64 = decimal_year_of_sample(TABULATED_DELTA_T[0]);

/// The last decimal year the observed ΔT table covers: 2026-04-01, the
/// last month the USNO had observed when the table was taken. After it the
/// polynomial answers, six seconds high; see the [module
/// documentation](self).
pub const TABULATED_DELTA_T_LAST: f64 =
    decimal_year_of_sample(TABULATED_DELTA_T[TABULATED_DELTA_T.len() - 1]);

/// The decimal Gregorian year of a table sample, by the same rule as
/// [`decimal_year`]: the elapsed fraction of the actual year.
///
/// The table is a literal, so a sample naming a month that does not exist
/// is a compile-time error here rather than a runtime one.
const fn decimal_year_of_sample(sample: DeltaTSample) -> f64 {
    let day_of_year = match hc_calendar::gregorian::day_of_year(sample.year, sample.month, 1) {
        Ok(day) => day,
        Err(_) => panic!("a ΔT sample names a month that does not exist"),
    };
    let length = hc_calendar::gregorian::days_in_year(sample.year);
    sample.year as f64 + (day_of_year as f64 - 1.0) / length as f64
}

/// Which source answered a ΔT query; see [`delta_t_regime`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeltaTRegime {
    /// The observed table, interpolated between its yearly samples:
    /// [`TABULATED_DELTA_T_FIRST`] to [`TABULATED_DELTA_T_LAST`].
    Observed,
    /// The Espenak–Meeus polynomials inside the span they were fitted to,
    /// [`EARLIEST_FITTED_YEAR`] to [`LATEST_FITTED_YEAR`], outside the table.
    Fitted,
    /// The long-term parabola beyond the fitted span.
    Extrapolated,
}

/// ΔT = TT − UT1 in seconds, for a moment in Universal Time.
///
/// The argument is nominally a UT moment, but ΔT changes by at most a second
/// or two per year, so feeding it a TT moment instead changes the answer by
/// far less than the fit's own error.
#[must_use]
pub fn delta_t(moment: Moment) -> f64 {
    delta_t_for_year(decimal_year(moment))
}

/// ΔT = TT − UT1 in seconds, for a decimal Gregorian year.
///
/// Inside [`TABULATED_DELTA_T_FIRST`]`..=`[`TABULATED_DELTA_T_LAST`] this is
/// the observed table, [`delta_t_tabulated`]; outside it, the polynomials,
/// [`delta_t_polynomial`]. Beyond −500…+2150 those are the Espenak–Meeus
/// parabola, whose uncertainty grows to hours at the ends of the historical
/// record; see the crate README for what that means for a date you actually
/// care about.
#[must_use]
pub fn delta_t_for_year(year: f64) -> f64 {
    delta_t_tabulated(year).unwrap_or_else(|| delta_t_polynomial(year))
}

/// Which source [`delta_t_for_year`] answers from for a decimal year.
#[must_use]
pub fn delta_t_regime(year: f64) -> DeltaTRegime {
    if (TABULATED_DELTA_T_FIRST..=TABULATED_DELTA_T_LAST).contains(&year) {
        DeltaTRegime::Observed
    } else if is_fitted_year(year) {
        DeltaTRegime::Fitted
    } else {
        DeltaTRegime::Extrapolated
    }
}

/// ΔT from the observed table alone, interpolated linearly between the
/// samples either side of `year`, or `None` outside the table.
///
/// The table is not extrapolated: a year past its last sample gets `None`,
/// not the last value carried forward.
#[must_use]
pub fn delta_t_tabulated(year: f64) -> Option<f64> {
    if !(TABULATED_DELTA_T_FIRST..=TABULATED_DELTA_T_LAST).contains(&year) {
        return None;
    }
    let (&first, &last) = (TABULATED_DELTA_T.first()?, TABULATED_DELTA_T.last()?);
    let index = TABULATED_DELTA_T.partition_point(|&sample| decimal_year_of_sample(sample) <= year);
    if index == 0 {
        return Some(first.seconds);
    }
    if index == TABULATED_DELTA_T.len() {
        return Some(last.seconds);
    }
    let (lower, upper) = (TABULATED_DELTA_T[index - 1], TABULATED_DELTA_T[index]);
    let (x0, x1) = (decimal_year_of_sample(lower), decimal_year_of_sample(upper));
    if x1 <= x0 {
        return Some(lower.seconds);
    }
    Some(lower.seconds + (upper.seconds - lower.seconds) * (year - x0) / (x1 - x0))
}

/// ΔT from the Espenak–Meeus polynomials alone, for any decimal year,
/// ignoring the observed table.
///
/// This is what [`delta_t_for_year`] answers outside the table, and inside
/// it the value the crate would give without the table: 73.9 s for 2024
/// against the 69.2 s observed.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn delta_t_polynomial(year: f64) -> f64 {
    if year < -500.0 {
        let u = (year - 1820.0) / 100.0;
        -20.0 + 32.0 * u * u
    } else if year < 500.0 {
        let u = year / 100.0;
        poly(
            u,
            &[
                10_583.6,
                -1_014.41,
                33.783_11,
                -5.952_053,
                -0.179_845_2,
                0.022_174_192,
                0.009_031_652_1,
            ],
        )
    } else if year < 1600.0 {
        let u = (year - 1000.0) / 100.0;
        poly(
            u,
            &[
                1_574.2,
                -556.01,
                71.234_72,
                0.319_781,
                -0.850_346_3,
                -0.005_050_998,
                0.008_357_207_3,
            ],
        )
    } else if year < 1700.0 {
        let t = year - 1600.0;
        poly(t, &[120.0, -0.980_8, -0.015_32, 1.0 / 7_129.0])
    } else if year < 1800.0 {
        let t = year - 1700.0;
        poly(
            t,
            &[
                8.83,
                0.160_3,
                -0.005_928_5,
                0.000_133_36,
                -1.0 / 1_174_000.0,
            ],
        )
    } else if year < 1860.0 {
        let t = year - 1800.0;
        poly(
            t,
            &[
                13.72,
                -0.332_447,
                0.006_861_2,
                0.004_111_6,
                -0.000_374_36,
                0.000_012_127_2,
                -0.000_000_169_9,
                0.000_000_000_875,
            ],
        )
    } else if year < 1900.0 {
        let t = year - 1860.0;
        poly(
            t,
            &[
                7.62,
                0.573_7,
                -0.251_754,
                0.016_806_68,
                -0.000_447_362_4,
                1.0 / 233_174.0,
            ],
        )
    } else if year < 1920.0 {
        let t = year - 1900.0;
        poly(
            t,
            &[-2.79, 1.494_119, -0.059_893_9, 0.006_196_6, -0.000_197],
        )
    } else if year < 1941.0 {
        let t = year - 1920.0;
        poly(t, &[21.20, 0.844_93, -0.076_100, 0.002_093_6])
    } else if year < 1961.0 {
        let t = year - 1950.0;
        poly(t, &[29.07, 0.407, -1.0 / 233.0, 1.0 / 2_547.0])
    } else if year < 1986.0 {
        let t = year - 1975.0;
        poly(t, &[45.45, 1.067, -1.0 / 260.0, -1.0 / 718.0])
    } else if year < 2005.0 {
        let t = year - 2000.0;
        poly(
            t,
            &[
                63.86,
                0.334_5,
                -0.060_374,
                0.001_727_5,
                0.000_651_814,
                0.000_023_735_99,
            ],
        )
    } else if year < 2050.0 {
        let t = year - 2000.0;
        poly(t, &[62.92, 0.322_17, 0.005_589])
    } else if year < 2150.0 {
        // The 2050…2150 branch is the long-term parabola pulled back onto the
        // observed 2050 value, so that the two meet without a step.
        let u = (year - 1820.0) / 100.0;
        -20.0 + 32.0 * u * u - 0.5628 * (2150.0 - year)
    } else {
        let u = (year - 1820.0) / 100.0;
        -20.0 + 32.0 * u * u
    }
}

/// Whether a year lies inside the span the ΔT polynomials were fitted to,
/// rather than the parabolic extrapolation beyond it.
///
/// This is about the polynomials. Inside the observed table, which lies
/// within the fitted span, the table answers instead; [`delta_t_regime`]
/// tells the three apart.
#[must_use]
pub fn is_fitted_year(year: f64) -> bool {
    (EARLIEST_FITTED_YEAR..LATEST_FITTED_YEAR).contains(&year)
}

/// The Terrestrial Time reading of a Universal Time moment.
#[must_use]
pub fn dynamical_time(moment: Moment) -> Moment {
    Moment(moment.0 + delta_t(moment) / SECONDS_PER_DAY)
}

/// The Universal Time reading of a Terrestrial Time moment.
///
/// ΔT is evaluated at the TT argument rather than at the UT answer it is
/// looking for. The resulting inconsistency is ΔT's own slope times ΔT: under
/// a microsecond today, and still under a hundredth of a second in the sixth
/// century BCE, where ΔT is three hours and changing fastest. Against the
/// several-minute uncertainty of ΔT itself at that date, iterating would be
/// polishing a guess.
#[must_use]
pub fn universal_time(dynamical: Moment) -> Moment {
    Moment(dynamical.0 - delta_t(dynamical) / SECONDS_PER_DAY)
}

/// Julian centuries of Terrestrial Time since J2000.0, for a Universal Time
/// moment. This is the argument every series in this crate takes.
#[must_use]
pub fn julian_centuries(moment: Moment) -> f64 {
    julian_centuries_from_dynamical(dynamical_time(moment))
}

/// Julian centuries since J2000.0 for a moment already expressed in
/// Terrestrial Time.
#[must_use]
pub fn julian_centuries_from_dynamical(dynamical: Moment) -> f64 {
    (dynamical.0 - J2000.0) / JULIAN_CENTURY_DAYS
}

/// The Terrestrial Time moment a count of Julian centuries since J2000.0
/// refers to.
#[must_use]
pub fn dynamical_from_julian_centuries(centuries: f64) -> Moment {
    Moment(J2000.0 + centuries * JULIAN_CENTURY_DAYS)
}

/// The Universal Time moment of an instant given as a Julian Date in
/// Terrestrial Time.
///
/// Published worked examples are stated in TT as a Julian Ephemeris Day, so
/// anything reproducing them — including this crate's own tests — needs this
/// bridge.
#[must_use]
pub fn universal_from_dynamical_julian_date(julian_date: f64) -> Moment {
    universal_time(Moment::from_julian_date(julian_date))
}

/// Julian centuries since J2000.0 for a Julian Date already expressed in
/// Terrestrial Time.
#[must_use]
pub fn centuries_from_dynamical_julian_date(julian_date: f64) -> f64 {
    julian_centuries_from_dynamical(Moment::from_julian_date(julian_date))
}

/// The fixed day on which a proleptic Gregorian year begins.
///
/// An adapter over [`hc_calendar::gregorian::new_year`], which owns this
/// arithmetic. The owner is `hc-calendar` rather than `hc-calendars-solar`,
/// so this crate can use it without depending on the solar calendars, which
/// depend on the astronomy.
#[must_use]
pub const fn gregorian_new_year(year: i64) -> Rd {
    hc_calendar::gregorian::new_year(year)
}

/// The proleptic Gregorian year containing a fixed day.
///
/// An adapter over [`hc_calendar::gregorian::year_from_fixed`]; see
/// [`gregorian_new_year`].
#[must_use]
pub const fn gregorian_year_from_rd(rd: Rd) -> i64 {
    hc_calendar::gregorian::year_from_fixed(rd)
}

/// A moment as a decimal Gregorian year, e.g. 2000.5 for midsummer 2000.
///
/// The fraction is the elapsed part of the actual year rather than Espenak's
/// `(month − 0.5)/12`, because a stepwise year makes ΔT discontinuous and the
/// root finders in this crate need a continuous function to bisect.
#[must_use]
pub fn decimal_year(moment: Moment) -> f64 {
    let year = gregorian_year_from_rd(moment.day());
    let start = gregorian_new_year(year).0;
    let length = gregorian_new_year(year + 1).0 - start;
    year as f64 + (moment.0 - start as f64) / length as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2000-01-01 is RD 730120 and the J2000 epoch is noon that day.
    #[test]
    fn the_j2000_epoch_sits_at_noon_on_the_first_of_january_2000() {
        assert_eq!(gregorian_new_year(2000), Rd(730_120));
        assert!((J2000.0 - 730_120.5).abs() < 1e-12);
        // Julian Date 2451545.0 is the conventional statement of J2000.0.
        assert!((J2000.to_julian_date() - 2_451_545.0).abs() < 1e-9);
        assert!(julian_centuries_from_dynamical(J2000).abs() < 1e-15);
    }

    #[test]
    fn gregorian_years_and_new_years_round_trip_over_four_millennia() {
        for year in -1000..3000 {
            let start = gregorian_new_year(year);
            assert_eq!(gregorian_year_from_rd(start), year, "new year of {year}");
            assert_eq!(
                gregorian_year_from_rd(gregorian_new_year(year + 1) - 1),
                year,
                "last day of {year}"
            );
        }
    }

    #[test]
    fn decimal_years_run_from_the_year_number_to_the_next() {
        assert!((decimal_year(Moment(gregorian_new_year(2000).0 as f64)) - 2000.0).abs() < 1e-12);
        let midyear = decimal_year(Moment(gregorian_new_year(2000).0 as f64 + 183.0));
        assert!((midyear - 2000.5).abs() < 0.01, "midyear was {midyear}");
        assert!(decimal_year(Moment(gregorian_new_year(-44).0 as f64)) < -43.9);
    }

    /// ΔT at 2000-01-01 is 63.8285 s by observation (USNO `deltat.data`).
    /// The Espenak–Meeus fit is built to land on 63.86 s there, and the
    /// table answers with the observation.
    #[test]
    fn delta_t_matches_the_observed_value_at_j2000() {
        let value = delta_t_for_year(2000.0);
        assert!((value - 63.8285).abs() < 1e-9, "delta t was {value}");
        let fitted = delta_t_polynomial(2000.0);
        assert!((fitted - 63.86).abs() < 0.01, "the fit gave {fitted}");
    }

    /// Observed ΔT: 1900.0 ≈ −2.7 s, 1950.0 ≈ 29.1 s, 1970.0 ≈ 40.2 s,
    /// 1980.0 ≈ 50.5 s (IERS Bulletin B / Espenak's tabulation). The first
    /// three are before the table and come from the polynomials.
    #[test]
    fn delta_t_tracks_the_twentieth_century_record() {
        assert!((delta_t_for_year(1900.0) - -2.7).abs() < 0.5);
        assert!((delta_t_for_year(1950.0) - 29.1).abs() < 0.5);
        assert!((delta_t_for_year(1970.0) - 40.2).abs() < 1.0);
        assert!((delta_t_for_year(1980.0) - 50.5387).abs() < 1e-9);
    }

    /// The table's ends, as decimal years by the crate's own rule.
    #[test]
    fn the_table_runs_from_new_year_1974_to_the_first_of_april_2026() {
        assert!((TABULATED_DELTA_T_FIRST - 1974.0).abs() < 1e-12);
        let april = Moment(hc_calendar::gregorian::to_fixed(2026, 4, 1).unwrap().0 as f64);
        assert!((TABULATED_DELTA_T_LAST - decimal_year(april)).abs() < 1e-12);
        assert!((TABULATED_DELTA_T_LAST - (2026.0 + 90.0 / 365.0)).abs() < 1e-12);
    }

    #[test]
    fn the_table_answers_inside_its_range_and_the_polynomial_outside() {
        assert_eq!(delta_t_regime(1973.9), DeltaTRegime::Fitted);
        assert_eq!(delta_t_regime(1974.0), DeltaTRegime::Observed);
        assert_eq!(delta_t_regime(2024.5), DeltaTRegime::Observed);
        assert_eq!(
            delta_t_regime(TABULATED_DELTA_T_LAST),
            DeltaTRegime::Observed
        );
        assert_eq!(
            delta_t_regime(TABULATED_DELTA_T_LAST + 1e-9),
            DeltaTRegime::Fitted
        );
        assert_eq!(delta_t_regime(0.0), DeltaTRegime::Fitted);
        assert_eq!(delta_t_regime(-501.0), DeltaTRegime::Extrapolated);
        assert_eq!(delta_t_regime(2200.0), DeltaTRegime::Extrapolated);

        assert!(delta_t_tabulated(1973.9).is_none());
        assert!(delta_t_tabulated(TABULATED_DELTA_T_LAST + 1e-9).is_none());
        assert!(delta_t_tabulated(f64::NAN).is_none());
        for year in [-1000.0, 1900.0, 1973.9, 2030.0, 2200.0] {
            assert!((delta_t_for_year(year) - delta_t_polynomial(year)).abs() < 1e-12);
        }
        for sample in TABULATED_DELTA_T {
            let year = decimal_year_of_sample(*sample);
            assert!(
                (delta_t_for_year(year) - sample.seconds).abs() < 1e-9,
                "at {year}"
            );
        }
    }

    #[test]
    fn the_table_interpolates_linearly_between_its_samples() {
        // 2019-01-01: 69.2202 s; 2020-01-01: 69.3612 s.
        let midway = delta_t_for_year(2019.5);
        assert!((midway - 69.2907).abs() < 1e-9, "{midway}");
        // The last, three-month interval: 69.1099 s to 69.1330 s.
        let year = TABULATED_DELTA_T_LAST - (TABULATED_DELTA_T_LAST - 2026.0) / 3.0;
        let two_thirds = delta_t_for_year(year);
        assert!((two_thirds - 69.1253).abs() < 1e-9, "{two_thirds}");
    }

    /// Unix seconds at 0h UT on the first of a month, for the identity check.
    fn unix_seconds_of(year: i64, month: u8) -> i64 {
        let rd = hc_calendar::gregorian::to_fixed(year, month, 1).unwrap().0;
        (rd - hc_calendar::fixed::RD_OF_UNIX_EPOCH) * 86_400
    }

    /// In the atomic era ΔT = 32.184 s + (TAI − UTC) − DUT1, and the IERS
    /// keeps |DUT1| under 0.9 s, so every sample must sit within 0.9 s of
    /// 32.184 s plus the leap-second table's value on its date.
    #[test]
    fn every_sample_is_consistent_with_the_leap_second_table() {
        use hc_core::unix::{LeapPolicy, tai_minus_utc_at};
        for sample in TABULATED_DELTA_T {
            let tai_minus_utc = tai_minus_utc_at(
                unix_seconds_of(sample.year, sample.month),
                LeapPolicy::Strict,
            )
            .unwrap()
            .as_secs_f64();
            let implied_dut1 = 32.184 + tai_minus_utc - sample.seconds;
            assert!(
                implied_dut1.abs() < 0.9,
                "{}-{:02}: ΔT {} implies DUT1 {implied_dut1}",
                sample.year,
                sample.month,
                sample.seconds
            );
        }
    }

    /// The same identity with DUT1 read from the IERS EOP 20 C04 series at
    /// 0h UTC (hpiers.obspm.fr/iers/eop/eopc04/eopc04.1962-now, retrieved
    /// 2026-09-25): the table and the series say the same thing to a few
    /// milliseconds, the C04 having been revised since the USNO computed
    /// its earliest values.
    #[test]
    fn the_samples_agree_with_the_iers_series_through_the_identity() {
        use hc_core::unix::{LeapPolicy, tai_minus_utc_at};
        let c04_dut1 = [
            (1974, 1, 0.699_299_6),
            (2000, 1, 0.355_472_4),
            (2016, 1, 0.081_512_2),
            (2026, 1, 0.074_086_9),
            (2026, 4, 0.050_976_5),
        ];
        for (year, month, dut1) in c04_dut1 {
            let tai_minus_utc = tai_minus_utc_at(unix_seconds_of(year, month), LeapPolicy::Strict)
                .unwrap()
                .as_secs_f64();
            let by_identity = 32.184 + tai_minus_utc - dut1;
            let sample = TABULATED_DELTA_T
                .iter()
                .find(|sample| (sample.year, sample.month) == (year, month))
                .unwrap();
            assert!(
                (sample.seconds - by_identity).abs() < 0.002,
                "{year}-{month:02}: table {} against {by_identity}",
                sample.seconds
            );
        }
    }

    /// The table joins the polynomial to a tenth of a second at its start,
    /// where the 1961–1986 segment was fitted to the same observations, and
    /// leaves a step of six seconds at its end, where the 2005–2050 segment
    /// is a forecast that ran high. The module documentation quotes both.
    #[test]
    fn the_table_meets_the_polynomial_at_its_start_and_not_at_its_end() {
        let start_step =
            delta_t_polynomial(TABULATED_DELTA_T_FIRST) - delta_t_for_year(TABULATED_DELTA_T_FIRST);
        assert!(
            start_step.abs() < 0.15,
            "step of {start_step} s at the start"
        );
        let end_step =
            delta_t_polynomial(TABULATED_DELTA_T_LAST) - delta_t_for_year(TABULATED_DELTA_T_LAST);
        assert!(
            (end_step - 6.1).abs() < 0.1,
            "step of {end_step} s at the end"
        );
        let at_2024 = delta_t_polynomial(2024.0) - delta_t_for_year(2024.0);
        assert!((at_2024 - 4.7).abs() < 0.1, "{at_2024}");
    }

    /// Historically important anchors: ΔT was about two hours around 1000 CE
    /// and about three hours around 500 BCE.
    #[test]
    fn delta_t_grows_to_hours_in_antiquity() {
        let millennium = delta_t_for_year(1000.0);
        assert!(
            (millennium - 1574.0).abs() < 5.0,
            "delta t 1000 CE was {millennium}"
        );
        let classical = delta_t_for_year(-500.0);
        assert!(
            (classical - 17_190.0).abs() < 50.0,
            "delta t 500 BCE was {classical}"
        );
    }

    /// The segments of the fit are independent least-squares solutions, so
    /// they are allowed to disagree at the joins — but only by an amount
    /// small against the data they were fitted to.
    #[test]
    fn the_delta_t_segments_join_without_a_visible_step() {
        for boundary in [
            -500.0, 500.0, 1600.0, 1700.0, 1800.0, 1860.0, 1900.0, 1920.0, 1941.0, 1961.0, 1986.0,
            2005.0, 2050.0, 2150.0,
        ] {
            let before = delta_t_polynomial(boundary - 1e-6);
            let after = delta_t_polynomial(boundary);
            assert!(
                (before - after).abs() < 2.0,
                "step of {} s at {boundary}",
                before - after
            );
        }
    }

    #[test]
    fn delta_t_is_symmetric_and_large_in_the_far_future_and_past() {
        // The long-term parabola is symmetric about 1820.
        let past = delta_t_polynomial(-2180.0);
        let future = delta_t_polynomial(5820.0);
        assert!((past - future).abs() < 1e-6);
        assert!(past > 10_000.0);
    }

    #[test]
    fn the_fitted_era_is_reported_honestly() {
        assert!(is_fitted_year(0.0));
        assert!(is_fitted_year(2024.0));
        assert!(!is_fitted_year(-501.0));
        assert!(!is_fitted_year(2200.0));
    }

    #[test]
    fn dynamical_and_universal_time_invert_each_other() {
        for rd in [-200_000i64, 0, 500_000, 730_120, 800_000] {
            let ut = Moment(rd as f64 + 0.25);
            let round_trip = universal_time(dynamical_time(ut));
            assert!(
                (round_trip.0 - ut.0).abs() < 1e-6,
                "round trip error {} at RD {rd}",
                round_trip.0 - ut.0
            );
        }
    }

    #[test]
    fn the_time_scale_round_trip_is_tightest_where_delta_t_is_flattest() {
        // Today ΔT changes by under a second a year, so the single-pass
        // inversion is exact to the last bit; in antiquity it costs
        // milliseconds, which is still far inside ΔT's own uncertainty.
        let modern = Moment(739_000.25);
        assert!((universal_time(dynamical_time(modern)).0 - modern.0).abs() < 1e-9);
        let ancient = Moment(-200_000.0);
        let error_seconds =
            (universal_time(dynamical_time(ancient)).0 - ancient.0).abs() * SECONDS_PER_DAY;
        assert!(error_seconds < 0.02, "round trip cost {error_seconds} s");
    }

    #[test]
    fn dynamical_time_runs_ahead_of_universal_time_today() {
        let now = Moment(739_000.0);
        assert!(dynamical_time(now).0 > now.0);
        let seconds = (dynamical_time(now).0 - now.0) * SECONDS_PER_DAY;
        assert!((60.0..90.0).contains(&seconds), "delta t was {seconds}");
    }

    #[test]
    fn the_julian_ephemeris_day_bridge_round_trips() {
        // JDE 2448908.5 is 1992 October 13.0 TD, Meeus's example 25.a.
        let ut = universal_from_dynamical_julian_date(2_448_908.5);
        assert!((dynamical_time(ut).to_julian_date() - 2_448_908.5).abs() < 1e-8);
        let centuries = centuries_from_dynamical_julian_date(2_448_908.5);
        assert!(
            (centuries + 0.072_183_436).abs() < 1e-9,
            "centuries {centuries}"
        );
    }

    #[test]
    fn julian_centuries_advance_by_one_every_thirty_six_thousand_days() {
        let a = julian_centuries(Moment(730_120.5));
        let b = julian_centuries(Moment(730_120.5 + JULIAN_CENTURY_DAYS));
        assert!((b - a - 1.0).abs() < 1e-6);
        assert!((dynamical_from_julian_centuries(1.0).0 - (J2000.0 + 36_525.0)).abs() < 1e-9);
    }
}
