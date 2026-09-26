//! UT2, UT1R and UT1S: UT1 with a named part of its known variation
//! removed.
//!
//! UT1 is the Earth's rotation as measured, and it carries variations the
//! older time services smoothed out before they published it. Each
//! smoothing is a convention with its own name, and each function here
//! takes the caller's UT1 — from [`crate::ut1::Ut1Offsets`], from the ΔT
//! model, or from anywhere else — and returns the smoothed reading. None
//! of them is a time scale anyone now disseminates; they are here so that
//! a historical reading labelled with one of these names can be put back
//! on UT1.
//!
//! * **UT2** removes the conventional *seasonal* variation, the formula
//!   the time services adopted in 1955 and the USNO still states:
//!   UT2 − UT1 = 0.022 sin 2π*T* − 0.012 cos 2π*T* − 0.006 sin 4π*T* +
//!   0.007 cos 4π*T* seconds, with *T* = 2000.000 + (MJD − 51 544.03) /
//!   365.2422 the Besselian year (USNO, "Common Units and Conversions in
//!   Earth Orientation", `usno-eo-values`). SOFA's *Time Scale and
//!   Calendar Tools* calls it "no longer used" (`sofa-ts`).
//! * **UT1R** removes the *zonal tides* with periods under 35 days, as the
//!   IAU's 18th General Assembly (Patras, 1982) adopted it. The tides are
//!   the 41 terms of IERS Conventions 2010, TN 36, chapter 8, Table 8.1,
//!   with periods from 5.64 to 34.85 days.
//! * **UT1S** removes *all* the zonal tides, to the 18.6-year nodal term:
//!   the 62 terms of the same table. The Conventions name it as a past
//!   definition beside UT1R, and Table 8.1 supports it because the table
//!   runs from 5 days to 18.6 years.
//!
//! The tidal model is part of the name. Table 8.1 is the IERS 2010 model —
//! the Yoder, Williams and Parke (1981) elastic tide with the Wahr and
//! Bergen (1986) inelastic body tide and the Kantha et al. (1998) ocean
//! tide — and it differs from Yoder's own 1981 tables, which the IAU's
//! 1982 definition names, by about 6 µs at the fortnightly term. The
//! functions are therefore [`ut1r_iers2010`] and [`ut1s_iers2010`], and a
//! UT1R from Yoder's tables, if one is added, is a separate function
//! (`docs/policy.md` §5). The IERS itself recommends exchanging UT1 and
//! the length of day only, and naming the tidal model wherever a
//! regularised value is used.
//!
//! The tidal arguments are the Delaunay arguments of IERS Conventions
//! 2010, chapter 5, equation 5.43, in Julian centuries of TDB, for which
//! the Conventions allow TT; the TT here is this crate's `UT1 + ΔT`, and a
//! minute's error in ΔT moves the fortnightly term by under 20 ns. The
//! whole-table sum reproduces the test case printed in the IERS routine
//! `RG_ZONT2.F` to 10⁻¹² s.
//!
//! `docs/time-scales.md` places these beside UT1 and UTC.

use hc_calendar::fixed::Moment;
use hc_core::math::{DEG_TO_RAD, cos, sin};

use crate::time::{SECONDS_PER_DAY, julian_centuries};
use crate::util::poly;

/// The Modified Julian Date of the Besselian epoch B2000.0 as the USNO's
/// UT2 formula writes it: *T* = 2000.000 + (MJD − 51 544.03) / 365.2422.
const UT2_EPOCH_MJD: f64 = 51_544.03;

/// The length of the Besselian (tropical) year in the USNO's UT2 formula,
/// in days.
const UT2_YEAR_DAYS: f64 = 365.2422;

/// The Julian Date of MJD 0.
const MJD_ZERO_JULIAN_DATE: f64 = 2_400_000.5;

/// The period below which a zonal tide counts as short for UT1R, in days
/// (IAU 18th General Assembly, 1982, as IERS Conventions 2010, chapter 8,
/// footnote 1, reports it).
pub const UT1R_PERIOD_LIMIT_DAYS: f64 = 35.0;

/// The Besselian year *T* of a UT1 moment, by the USNO's UT2 formula.
#[must_use]
pub fn ut2_besselian_year(ut1: Moment) -> f64 {
    let mjd = ut1.to_julian_date() - MJD_ZERO_JULIAN_DATE;
    2000.0 + (mjd - UT2_EPOCH_MJD) / UT2_YEAR_DAYS
}

/// UT2 − UT1, in seconds, at a UT1 moment: the conventional seasonal
/// variation of the Earth's rotation.
///
/// 0.022 sin 2π*T* − 0.012 cos 2π*T* − 0.006 sin 4π*T* + 0.007 cos 4π*T*,
/// *T* the Besselian year of [`ut2_besselian_year`] (USNO, "Common Units
/// and Conversions in Earth Orientation", retrieved 2026-09-26). The
/// correction is a convention and exact as a formula; its extremes are
/// ±0.031 s.
#[must_use]
pub fn ut2_minus_ut1(ut1: Moment) -> f64 {
    let year = ut2_besselian_year(ut1);
    // Only the fraction of the year matters, and taking it first keeps the
    // sines' argument small.
    let phase = 2.0 * core::f64::consts::PI * crate::util::modulo(year, 1.0);
    0.022 * sin(phase) - 0.012 * cos(phase) - 0.006 * sin(2.0 * phase) + 0.007 * cos(2.0 * phase)
}

/// The UT2 reading of a UT1 moment: UT1 plus [`ut2_minus_ut1`].
#[must_use]
pub fn ut2(ut1: Moment) -> Moment {
    Moment(ut1.0 + ut2_minus_ut1(ut1) / SECONDS_PER_DAY)
}

/// One zonal tide term of IERS Conventions 2010, Table 8.1, in its UT1
/// columns.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ZonalTide {
    /// The integer multipliers of the Delaunay arguments *l*, *l*′, *F*,
    /// *D* and Ω, the table's first five columns.
    pub multipliers: [i8; 5],
    /// The approximate period in days, negative for a retrograde term, as
    /// the table prints it.
    pub period_days: f64,
    /// *B*ᵢ, the coefficient of the sine in δUT1, in units of 10⁻⁴ s.
    pub sine: f64,
    /// *C*ᵢ, the coefficient of the cosine in δUT1, in units of 10⁻⁴ s.
    pub cosine: f64,
}

/// The 62 zonal tide terms of IERS Conventions 2010, TN 36, chapter 8,
/// Table 8.1, with the UT1 columns only; the length-of-day and
/// angular-velocity columns are not carried. Transcribed from the chapter
/// as published at <https://iers-conventions.obspm.fr/content/chapter8/icc8.pdf>,
/// retrieved 2026-09-26, in the table's order: the 41 terms under 35 days
/// first, then the long-period ones to the 18.6-year nodal term.
pub const ZONAL_TIDES_IERS2010: [ZonalTide; 62] = [
    ZonalTide {
        multipliers: [1, 0, 2, 2, 2],
        period_days: 5.64,
        sine: -0.0235,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [2, 0, 2, 0, 1],
        period_days: 6.85,
        sine: -0.0404,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [2, 0, 2, 0, 2],
        period_days: 6.86,
        sine: -0.0987,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, 0, 2, 2, 1],
        period_days: 7.09,
        sine: -0.0508,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, 0, 2, 2, 2],
        period_days: 7.10,
        sine: -0.1231,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [1, 0, 2, 0, 0],
        period_days: 9.11,
        sine: -0.0385,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [1, 0, 2, 0, 1],
        period_days: 9.12,
        sine: -0.4108,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [1, 0, 2, 0, 2],
        period_days: 9.13,
        sine: -0.9926,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [3, 0, 0, 0, 0],
        period_days: 9.18,
        sine: -0.0179,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [-1, 0, 2, 2, 1],
        period_days: 9.54,
        sine: -0.0818,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [-1, 0, 2, 2, 2],
        period_days: 9.56,
        sine: -0.1974,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [1, 0, 0, 2, 0],
        period_days: 9.61,
        sine: -0.0761,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [2, 0, 2, -2, 2],
        period_days: 12.81,
        sine: 0.0216,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, 1, 2, 0, 2],
        period_days: 13.17,
        sine: 0.0254,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, 0, 2, 0, 0],
        period_days: 13.61,
        sine: -0.2989,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, 0, 2, 0, 1],
        period_days: 13.63,
        sine: -3.1873,
        cosine: 0.2010,
    },
    ZonalTide {
        multipliers: [0, 0, 2, 0, 2],
        period_days: 13.66,
        sine: -7.8468,
        cosine: 0.5320,
    },
    ZonalTide {
        multipliers: [2, 0, 0, 0, -1],
        period_days: 13.75,
        sine: 0.0216,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [2, 0, 0, 0, 0],
        period_days: 13.78,
        sine: -0.3384,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [2, 0, 0, 0, 1],
        period_days: 13.81,
        sine: 0.0179,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, -1, 2, 0, 2],
        period_days: 14.19,
        sine: -0.0244,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, 0, 0, 2, -1],
        period_days: 14.73,
        sine: 0.0470,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, 0, 0, 2, 0],
        period_days: 14.77,
        sine: -0.7341,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, 0, 0, 2, 1],
        period_days: 14.80,
        sine: -0.0526,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, -1, 0, 2, 0],
        period_days: 15.39,
        sine: -0.0508,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [1, 0, 2, -2, 1],
        period_days: 23.86,
        sine: 0.0498,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [1, 0, 2, -2, 2],
        period_days: 23.94,
        sine: 0.1006,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [1, 1, 0, 0, 0],
        period_days: 25.62,
        sine: 0.0395,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [-1, 0, 2, 0, 0],
        period_days: 26.88,
        sine: 0.0470,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [-1, 0, 2, 0, 1],
        period_days: 26.98,
        sine: 0.1767,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [-1, 0, 2, 0, 2],
        period_days: 27.09,
        sine: 0.4352,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [1, 0, 0, 0, -1],
        period_days: 27.44,
        sine: 0.5339,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [1, 0, 0, 0, 0],
        period_days: 27.56,
        sine: -8.4046,
        cosine: 0.2500,
    },
    ZonalTide {
        multipliers: [1, 0, 0, 0, 1],
        period_days: 27.67,
        sine: 0.5443,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, 0, 0, 1, 0],
        period_days: 29.53,
        sine: 0.0470,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [1, -1, 0, 0, 0],
        period_days: 29.80,
        sine: -0.0555,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [-1, 0, 0, 2, -1],
        period_days: 31.66,
        sine: 0.1175,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [-1, 0, 0, 2, 0],
        period_days: 31.81,
        sine: -1.8236,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [-1, 0, 0, 2, 1],
        period_days: 31.96,
        sine: 0.1316,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [1, 0, -2, 2, -1],
        period_days: 32.61,
        sine: 0.0179,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [-1, -1, 0, 2, 0],
        period_days: 34.85,
        sine: -0.0855,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, 2, 2, -2, 2],
        period_days: 91.31,
        sine: -0.0573,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, 1, 2, -2, 1],
        period_days: 119.61,
        sine: 0.0329,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, 1, 2, -2, 2],
        period_days: 121.75,
        sine: -1.8847,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, 0, 2, -2, 0],
        period_days: 173.31,
        sine: 0.2510,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, 0, 2, -2, 1],
        period_days: 177.84,
        sine: 1.1703,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, 0, 2, -2, 2],
        period_days: 182.62,
        sine: -49.7174,
        cosine: 0.4330,
    },
    ZonalTide {
        multipliers: [0, 2, 0, 0, 0],
        period_days: 182.63,
        sine: -0.1936,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [2, 0, 0, -2, -1],
        period_days: 199.84,
        sine: 0.0489,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [2, 0, 0, -2, 0],
        period_days: 205.89,
        sine: -0.5471,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [2, 0, 0, -2, 1],
        period_days: 212.32,
        sine: 0.0367,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, -1, 2, -2, 1],
        period_days: 346.60,
        sine: -0.0451,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, 1, 0, 0, -1],
        period_days: 346.64,
        sine: 0.0921,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, -1, 2, -2, 2],
        period_days: 365.22,
        sine: 0.8281,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, 1, 0, 0, 0],
        period_days: 365.26,
        sine: -15.8887,
        cosine: 0.1530,
    },
    ZonalTide {
        multipliers: [0, 1, 0, 0, 1],
        period_days: 386.00,
        sine: -0.1382,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [1, 0, 0, -1, 0],
        period_days: 411.78,
        sine: 0.0348,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [2, 0, -2, 0, 0],
        period_days: -1095.18,
        sine: -0.1372,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [-2, 0, 2, 0, 1],
        period_days: 1305.48,
        sine: 0.4211,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [-1, 1, 0, 1, 0],
        period_days: 3232.86,
        sine: -0.0404,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, 0, 0, 0, 2],
        period_days: -3399.19,
        sine: 7.8998,
        cosine: 0.0000,
    },
    ZonalTide {
        multipliers: [0, 0, 0, 0, 1],
        period_days: -6798.38,
        sine: -1617.2681,
        cosine: 0.0000,
    },
];

/// The five Delaunay arguments *l*, *l*′, *F*, *D* and Ω, in radians, at
/// a count of Julian centuries of TDB (or TT) since J2000.0.
///
/// IERS Conventions 2010, chapter 5, §5.7.2, equation 5.43 (Simon et al.
/// 1994, not read here): a constant in degrees and a polynomial in
/// arcseconds.
#[must_use]
pub fn delaunay_arguments(centuries: f64) -> [f64; 5] {
    const SERIES: [(f64, [f64; 4]); 5] = [
        (
            134.963_402_51,
            [1_717_915_923.217_8, 31.879_2, 0.051_635, -0.000_244_70],
        ),
        (
            357.529_109_18,
            [129_596_581.048_1, -0.553_2, 0.000_136, -0.000_011_49],
        ),
        (
            93.272_090_62,
            [1_739_527_262.847_8, -12.751_2, -0.001_037, 0.000_004_17],
        ),
        (
            297.850_195_47,
            [1_602_961_601.209_0, -6.370_6, 0.006_593, -0.000_031_69],
        ),
        (
            125.044_555_01,
            [-6_962_890.543_1, 7.472_2, 0.007_702, -0.000_059_39],
        ),
    ];
    const ARCSECONDS_PER_TURN: f64 = 1_296_000.0;
    SERIES.map(|(constant, rates)| {
        let arcseconds = constant * 3600.0 + centuries * poly(centuries, &rates);
        crate::util::modulo(arcseconds, ARCSECONDS_PER_TURN) / 3600.0 * DEG_TO_RAD
    })
}

/// The effect on UT1 of the zonal tides whose period is under a limit, in
/// seconds: δUT1 = Σ *B*ᵢ sin ξᵢ + *C*ᵢ cos ξᵢ over those terms of
/// [`ZONAL_TIDES_IERS2010`], at a count of Julian centuries of TT.
///
/// Pass [`UT1R_PERIOD_LIMIT_DAYS`] for UT1R's tides and
/// [`f64::INFINITY`] for all of them, UT1S's. The regularised reading is
/// UT1 *minus* this: the Conventions' tables give the tides' effect, "to be
/// subtracted from the observed UT1".
#[must_use]
pub fn zonal_tide_ut1_effect(centuries: f64, period_limit_days: f64) -> f64 {
    let arguments = delaunay_arguments(centuries);
    let mut sum = 0.0;
    for tide in &ZONAL_TIDES_IERS2010 {
        if tide.period_days.abs() >= period_limit_days {
            continue;
        }
        let mut argument = 0.0;
        for (multiplier, value) in tide.multipliers.iter().zip(arguments) {
            argument += f64::from(*multiplier) * value;
        }
        sum += tide.sine * sin(argument) + tide.cosine * cos(argument);
    }
    sum * 1.0e-4
}

/// UT1R − UT1, in seconds, at a UT1 moment, by the IERS 2010 zonal tide
/// model: minus the effect of the 41 tides under 35 days.
///
/// The fortnightly and monthly tides dominate, and the 41 amplitudes sum
/// to 2.75 ms, which bounds the whole.
#[must_use]
pub fn ut1r_minus_ut1_iers2010(ut1: Moment) -> f64 {
    -zonal_tide_ut1_effect(julian_centuries(ut1), UT1R_PERIOD_LIMIT_DAYS)
}

/// The UT1R reading of a UT1 moment, by the IERS 2010 zonal tide model:
/// UT1 plus [`ut1r_minus_ut1_iers2010`]. See the [module
/// documentation](self) for why the model is in the name.
#[must_use]
pub fn ut1r_iers2010(ut1: Moment) -> Moment {
    Moment(ut1.0 + ut1r_minus_ut1_iers2010(ut1) / SECONDS_PER_DAY)
}

/// UT1S − UT1, in seconds, at a UT1 moment, by the IERS 2010 zonal tide
/// model: minus the effect of all 62 tides, to the 18.6-year nodal term.
///
/// The nodal term alone is 0.16 s, and the 62 amplitudes sum to 0.173 s,
/// which bounds the whole.
#[must_use]
pub fn ut1s_minus_ut1_iers2010(ut1: Moment) -> f64 {
    -zonal_tide_ut1_effect(julian_centuries(ut1), f64::INFINITY)
}

/// The UT1S reading of a UT1 moment, by the IERS 2010 zonal tide model:
/// UT1 plus [`ut1s_minus_ut1_iers2010`].
#[must_use]
pub fn ut1s_iers2010(ut1: Moment) -> Moment {
    Moment(ut1.0 + ut1s_minus_ut1_iers2010(ut1) / SECONDS_PER_DAY)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The Moment of a Modified Julian Date.
    fn mjd(value: f64) -> Moment {
        Moment::from_julian_date(value + MJD_ZERO_JULIAN_DATE)
    }

    /// At *T* = 2000.000, MJD 51 544.03, the sines vanish and the cosines
    /// are one: −0.012 + 0.007 = −0.005 s, by hand from the USNO's formula.
    #[test]
    fn ut2_at_the_formulas_epoch_is_the_sum_of_its_cosine_terms() {
        let moment = mjd(51_544.03);
        assert!((ut2_besselian_year(moment) - 2000.0).abs() < 1e-12);
        let value = ut2_minus_ut1(moment);
        assert!((value + 0.005).abs() < 1e-9, "UT2 − UT1 was {value}");
    }

    /// A quarter of a Besselian year later, sin 2π*T* = 1 and cos 4π*T* =
    /// −1: 0.022 − 0.007 = +0.015 s.
    #[test]
    fn ut2_a_quarter_year_on_is_the_first_sine_less_the_second_cosine() {
        let moment = mjd(51_544.03 + 365.2422 / 4.0);
        let value = ut2_minus_ut1(moment);
        assert!((value - 0.015).abs() < 1e-9, "UT2 − UT1 was {value}");
    }

    #[test]
    fn ut2_repeats_every_besselian_year_and_stays_inside_its_amplitudes() {
        for step in 0..365 {
            let moment = mjd(58_849.0 + f64::from(step));
            let value = ut2_minus_ut1(moment);
            assert!(value.abs() < 0.031, "UT2 − UT1 was {value}");
            let later = ut2_minus_ut1(mjd(58_849.0 + f64::from(step) + 365.2422));
            assert!((value - later).abs() < 1e-9);
        }
        let moment = mjd(60_000.0);
        let shifted = ut2(moment);
        assert!(((shifted.0 - moment.0) * SECONDS_PER_DAY - ut2_minus_ut1(moment)).abs() < 1e-5);
    }

    /// The test case in the header of the IERS routine `RG_ZONT2.F`
    /// (IERS Conventions software, chapter 8, retrieved 2026-09-26): at
    /// *T* = 0.079 958 932 238 193 02 Julian centuries of TT, MJD 54 465,
    /// the whole table gives DUT = 7.983 287 678 576 557 467 × 10⁻² s.
    #[test]
    fn the_whole_table_reproduces_the_iers_test_case() {
        let value = zonal_tide_ut1_effect(0.079_958_932_238_193_02, f64::INFINITY);
        assert!(
            (value - 0.079_832_876_785_765_57).abs() < 1e-12,
            "δUT1 was {value}"
        );
    }

    #[test]
    fn the_table_is_the_published_one() {
        assert_eq!(ZONAL_TIDES_IERS2010.len(), 62);
        let short = ZONAL_TIDES_IERS2010
            .iter()
            .filter(|tide| tide.period_days.abs() < UT1R_PERIOD_LIMIT_DAYS)
            .count();
        assert_eq!(short, 41);
        // The first and last rows, and the fortnightly and nodal terms.
        assert_eq!(ZONAL_TIDES_IERS2010[0].multipliers, [1, 0, 2, 2, 2]);
        assert!((ZONAL_TIDES_IERS2010[16].sine + 7.8468).abs() < 1e-12);
        assert!((ZONAL_TIDES_IERS2010[16].cosine - 0.5320).abs() < 1e-12);
        assert!((ZONAL_TIDES_IERS2010[61].sine + 1617.2681).abs() < 1e-12);
        assert!((ZONAL_TIDES_IERS2010[61].period_days + 6798.38).abs() < 1e-9);
        // The table is sorted by the magnitude of its period.
        for pair in ZONAL_TIDES_IERS2010.windows(2) {
            assert!(pair[0].period_days.abs() <= pair[1].period_days.abs());
        }
    }

    /// Each row's printed period is what its multipliers of the Delaunay
    /// rates give, to the table's rounding.
    #[test]
    fn each_period_follows_from_its_multipliers() {
        let now = delaunay_arguments(0.0);
        let step = 1.0e-4;
        let later = delaunay_arguments(step);
        let days = step * 36_525.0;
        let two_pi = 2.0 * core::f64::consts::PI;
        for tide in &ZONAL_TIDES_IERS2010 {
            let mut rate = 0.0;
            for (index, multiplier) in tide.multipliers.iter().enumerate() {
                let change = crate::util::signed_degrees((later[index] - now[index]).to_degrees());
                rate += f64::from(*multiplier) * change.to_radians() / days;
            }
            let period = two_pi / rate;
            let tolerance = 0.006 + 2.0e-5 * tide.period_days.abs();
            assert!(
                (period - tide.period_days).abs() < tolerance,
                "{:?} gives {period}",
                tide.multipliers
            );
        }
    }

    #[test]
    fn ut1r_takes_the_short_tides_and_ut1s_takes_them_all() {
        for step in 0..400 {
            let moment = mjd(58_849.0 + f64::from(step) * 0.7);
            let centuries = julian_centuries(moment);
            let short = zonal_tide_ut1_effect(centuries, UT1R_PERIOD_LIMIT_DAYS);
            let all = zonal_tide_ut1_effect(centuries, f64::INFINITY);
            assert!((ut1r_minus_ut1_iers2010(moment) + short).abs() < 1e-15);
            assert!((ut1s_minus_ut1_iers2010(moment) + all).abs() < 1e-15);
            // Under 35 days the terms' amplitudes sum to 2.75 ms; with the
            // long-period ones, to 0.173 s.
            assert!(short.abs() < 0.002_75, "short-period δUT1 {short}");
            assert!(all.abs() < 0.173, "δUT1 {all}");
        }
        let moment = mjd(60_000.0);
        let seconds = |reading: Moment| (reading.0 - moment.0) * SECONDS_PER_DAY;
        assert!((seconds(ut1r_iers2010(moment)) - ut1r_minus_ut1_iers2010(moment)).abs() < 1e-5);
        assert!((seconds(ut1s_iers2010(moment)) - ut1s_minus_ut1_iers2010(moment)).abs() < 1e-5);
    }

    /// The short-period tides are fortnightly and monthly: over a 27.6-day
    /// month UT1R − UT1 changes sign.
    #[test]
    fn the_short_period_correction_swings_within_a_month() {
        let values: Vec<f64> = (0..28)
            .map(|day| ut1r_minus_ut1_iers2010(mjd(60_000.0 + f64::from(day))))
            .collect();
        assert!(values.iter().any(|v| *v > 0.000_3));
        assert!(values.iter().any(|v| *v < -0.000_3));
    }
}
