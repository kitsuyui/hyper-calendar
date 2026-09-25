//! The elements at an epoch: eccentricity, obliquity, the longitude of
//! perihelion and the climatic precession.
//!
//! Berger's 1978 solution is three trigonometric series in the time `t`, in
//! years from 1950 and negative in the past, combined as his program
//! `insol14.f` combines them:
//!
//! ```text
//! e sin π = Σ Aᵢ sin(Bᵢ t + Cᵢ)          e cos π = Σ Aᵢ cos(Bᵢ t + Cᵢ)   (19 terms)
//! e       = √((e sin π)² + (e cos π)²)    π = atan2(e sin π, e cos π)
//! ψ       = ζ + ψ̄ t + Σ Aᵢ sin(Bᵢ t + Cᵢ)                              (78 terms)
//! ϖ       = π + ψ, reduced to [0°, 360°)
//! ε       = ε* + Σ Aᵢ cos(Bᵢ t + Cᵢ)                                   (47 terms)
//! climatic precession = e sin ϖ
//! ```
//!
//! π is the longitude of perihelion from the fixed equinox of the
//! reference frame, ψ the general precession in longitude that moves the
//! equinox, and ϖ their sum: the longitude of perihelion from the *moving*
//! equinox, the quantity Milankovitch theory uses. It is a heliocentric
//! longitude — the direction of Earth's perihelion — and is about 102° at
//! present. The Sun's geocentric longitude at perihelion is ϖ + 180°, which
//! is what the insolation formula needs and what some tables (PMIP's
//! "perihelion", NASA GISS's `PERI`) print; the relation is stated here
//! because the two are confused constantly.
//!
//! The explanation, the worked example at 21 000 years before present and
//! the accuracy measurements are in `docs/systems/orbital-elements.md`.

use core::ops::RangeInclusive;

use hc_core::math::{DEG_TO_RAD, RAD_TO_DEG, abs, asin, atan2, cos, sin, sqrt};
use hc_uncertainty::Uncertain;

use crate::error::{OrbitalError, OrbitalResult};
use crate::series::{
    ECCENTRICITY, OBLIQUITY, OBLIQUITY_CONSTANT_DEGREES, PRECESSION, PRECESSION_CONSTANT_DEGREES,
    PRECESSION_RATE_ARCSEC_PER_YEAR, Term,
};

/// The year the series counts from: 1950 CE, its `t = 0`.
///
/// This is Berger's own epoch, and it coincides with the radiocarbon "before
/// present" datum that `hc-deep-time`'s `BP_DATUM_YEAR` carries, so a
/// count of years BP from that crate is the argument this one takes. It is
/// not J2000: the present-day values this crate returns are for 1950, and
/// [`elements_at`] at `-50.0` gives the year 2000.
pub const EPOCH_YEAR: i32 = 1950;

/// The span, in years before [`EPOCH_YEAR`], over which the series is
/// answered: a million years either side of 1950. Negative is the future.
///
/// The bound is the one the author's program, NASA GISS and NCAR all
/// state: "valid only for 1 000 000 years" past or hence. It is a bound on
/// answering, not a promise of accuracy: Berger's own later solution
/// (Berger & Loutre 1991) is preferred beyond 800 000 years before present,
/// and the disagreement between the two grows across the span, as
/// [`spread_at`] records.
pub const VALID_SPAN: RangeInclusive<f64> = -1_000_000.0..=1_000_000.0;

/// Where the series comes from, named well enough to look up.
pub const SOURCE: &str = "Berger, A. (1978), Long-term variations of daily insolation and Quaternary \
     climatic changes, J. Atmos. Sci. 35, 2362-2367; coefficient tables from the author's deposit \
     (Zenodo 7198109, INSOL.IN, BERGER 78 version 2014)";

/// A count of years before 1950 from a calendar year in astronomical
/// numbering (year 0 is 1 BCE), which is what every chronological
/// computation and `hc-calendar` use.
///
/// `years_before_present_from_year(2000.0)` is `-50.0`: the year 2000 is
/// fifty years *after* the epoch.
#[must_use]
pub fn years_before_present_from_year(year: f64) -> f64 {
    f64::from(EPOCH_YEAR) - year
}

/// The calendar year, in astronomical numbering, of a count of years before
/// 1950. The inverse of [`years_before_present_from_year`].
#[must_use]
pub fn year_from_years_before_present(years_before_present: f64) -> f64 {
    f64::from(EPOCH_YEAR) - years_before_present
}

/// The measured disagreement between this series and the author's later
/// solution, for the tier of the span an epoch falls in.
///
/// These are not standard uncertainties of a measurement. The 1978 paper's
/// own accuracy statement could not be read (see the system document);
/// what can be measured is how far the series drifts from Berger & Loutre
/// (1991), the solution the same author published to replace it, over the
/// NOAA `orbit91` table at 1 000-year steps. Each value below is the
/// largest absolute difference found in the tier, rounded up, and the
/// tiers are the spans NOAA's readme distinguishes: "equivalent through
/// 800 kyr BP", and the 1991 solution "preferred" beyond. The future half
/// of the span was not measured — `orbit91` stops at 1950 — and is given
/// the past's figures.
///
/// The longitude of perihelion has no entry: it is the angle of a vector
/// whose length is the eccentricity, and when `e` passes near zero (it
/// reaches 0.0006 in the span) the angle is unconstrained. Its spread is
/// derived per epoch from the climatic precession's, in
/// [`OrbitalElements::longitude_of_perihelion_degrees`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spread {
    /// The span this tier covers, in years either side of 1950.
    pub within_years: f64,
    /// In the eccentricity.
    pub eccentricity: f64,
    /// In the obliquity, in degrees.
    pub obliquity_degrees: f64,
    /// In the climatic precession *e* sin ϖ.
    pub climatic_precession: f64,
}

/// The tiers of [`Spread`], nearest first.
pub const SPREAD_TIERS: [Spread; 3] = [
    // Measured 0.00178, 0.0415°, 0.00237 over 0–100 kyr BP.
    Spread {
        within_years: 100_000.0,
        eccentricity: 0.002,
        obliquity_degrees: 0.05,
        climatic_precession: 0.0025,
    },
    // Measured 0.00804, 0.195°, 0.00762 over 0–800 kyr BP.
    Spread {
        within_years: 800_000.0,
        eccentricity: 0.009,
        obliquity_degrees: 0.20,
        climatic_precession: 0.008,
    },
    // Measured 0.00904, 0.262°, 0.0141 over 0–1 000 kyr BP.
    Spread {
        within_years: 1_000_000.0,
        eccentricity: 0.010,
        obliquity_degrees: 0.27,
        climatic_precession: 0.015,
    },
];

/// The [`Spread`] tier an epoch falls in.
///
/// # Errors
///
/// [`OrbitalError::NotFinite`] and [`OrbitalError::OutsideValidSpan`], as
/// [`elements_at`].
pub fn spread_at(years_before_present: f64) -> OrbitalResult<Spread> {
    check_epoch(years_before_present)?;
    let distance = abs(years_before_present);
    SPREAD_TIERS
        .iter()
        .copied()
        .find(|tier| distance <= tier.within_years)
        .ok_or(OrbitalError::OutsideValidSpan)
}

/// The elements at one epoch, each with its [`Spread`] as an error bar.
///
/// The `std_dev` of every field is a measured disagreement between
/// solutions, not a Gaussian width; see [`Spread`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrbitalElements {
    /// The epoch, in years before 1950; negative in the future.
    pub years_before_present: f64,
    /// The eccentricity *e* of Earth's orbit.
    pub eccentricity: Uncertain,
    /// The obliquity of the ecliptic ε, in degrees.
    pub obliquity_degrees: Uncertain,
    /// The longitude of perihelion ϖ from the moving vernal equinox, in
    /// degrees, in `[0, 360)`: the heliocentric direction of perihelion,
    /// about 102° at present. Add 180° for the Sun's longitude at perihelion.
    ///
    /// Its spread is `asin(spread of e sin ϖ / e)` in degrees, or 180° when
    /// `e` is smaller than that spread, because the angle of a vector shorter
    /// than its own error bar is unknown.
    pub longitude_of_perihelion_degrees: Uncertain,
    /// The climatic precession *e* sin ϖ, positive when perihelion falls in
    /// northern summer.
    pub climatic_precession: Uncertain,
}

impl OrbitalElements {
    /// The obliquity in radians.
    ///
    /// # Errors
    ///
    /// Propagates the uncertainty layer's scaling error, which a finite
    /// value cannot raise.
    pub fn obliquity_radians(&self) -> OrbitalResult<Uncertain> {
        Ok(self.obliquity_degrees.scaled(DEG_TO_RAD)?)
    }

    /// The longitude of perihelion in radians.
    ///
    /// # Errors
    ///
    /// As [`OrbitalElements::obliquity_radians`].
    pub fn longitude_of_perihelion_radians(&self) -> OrbitalResult<Uncertain> {
        Ok(self.longitude_of_perihelion_degrees.scaled(DEG_TO_RAD)?)
    }

    /// The Sun's geocentric longitude at perihelion, ϖ + 180° reduced to
    /// `[0, 360)`: the angle the insolation formula and the PMIP tables use.
    #[must_use]
    pub fn perihelion_solar_longitude_degrees(&self) -> f64 {
        hc_core::math::normalize_degrees(self.longitude_of_perihelion_degrees.value + 180.0)
    }

    /// The calendar year of the epoch, in astronomical numbering.
    #[must_use]
    pub fn year(&self) -> f64 {
        year_from_years_before_present(self.years_before_present)
    }

    /// Where the values came from: [`SOURCE`].
    #[must_use]
    pub const fn source(&self) -> &'static str {
        SOURCE
    }
}

/// Refuse a non-finite epoch or one outside [`VALID_SPAN`].
fn check_epoch(years_before_present: f64) -> OrbitalResult<()> {
    if !years_before_present.is_finite() {
        return Err(OrbitalError::NotFinite);
    }
    if !VALID_SPAN.contains(&years_before_present) {
        return Err(OrbitalError::OutsideValidSpan);
    }
    Ok(())
}

/// Sum a table's terms with `sin` or `cos`, for `t` in years from 1950.
fn series_sum(table: &[Term], t: f64, trig: fn(f64) -> f64) -> f64 {
    let arcsec_to_rad = DEG_TO_RAD / 3600.0;
    table
        .iter()
        .map(|term| term.amplitude * trig(term.rate * arcsec_to_rad * t + term.phase * DEG_TO_RAD))
        .sum()
}

/// The central values only, with no error bars: `(e, ϖ°, ε°)`.
///
/// Kept separate so the insolation path and the tests can evaluate the
/// series without the uncertainty layer.
pub(crate) fn raw_elements(t: f64) -> (f64, f64, f64) {
    let e_sin_pi = series_sum(&ECCENTRICITY, t, sin);
    let e_cos_pi = series_sum(&ECCENTRICITY, t, cos);
    let eccentricity = sqrt(e_sin_pi * e_sin_pi + e_cos_pi * e_cos_pi);
    let fixed_perihelion = atan2(e_sin_pi, e_cos_pi) * RAD_TO_DEG;

    let precession = PRECESSION_CONSTANT_DEGREES
        + (PRECESSION_RATE_ARCSEC_PER_YEAR * t + series_sum(&PRECESSION, t, sin)) / 3600.0;
    let perihelion = hc_core::math::normalize_degrees(fixed_perihelion + precession);

    let obliquity = OBLIQUITY_CONSTANT_DEGREES + series_sum(&OBLIQUITY, t, cos) / 3600.0;
    (eccentricity, perihelion, obliquity)
}

/// The orbital elements at a count of years before 1950.
///
/// # Errors
///
/// [`OrbitalError::NotFinite`] for a NaN or infinite epoch and
/// [`OrbitalError::OutsideValidSpan`] beyond a million years either side of
/// 1950. The series would return numbers there; they would be fiction.
///
/// ```
/// use hc_orbital::elements_at;
///
/// // The Last Glacial Maximum, 21 000 years before 1950.
/// let lgm = elements_at(21_000.0).unwrap();
/// assert!((lgm.eccentricity.value - 0.018994).abs() < 1e-6);
/// assert!((lgm.obliquity_degrees.value - 22.949).abs() < 1e-3);
/// assert!((lgm.longitude_of_perihelion_degrees.value - 114.42).abs() < 0.01);
/// assert!(elements_at(1_000_001.0).is_err());
/// ```
pub fn elements_at(years_before_present: f64) -> OrbitalResult<OrbitalElements> {
    let spread = spread_at(years_before_present)?;
    let t = -years_before_present;
    let (eccentricity, perihelion, obliquity) = raw_elements(t);
    let climatic_precession = eccentricity * sin(perihelion * DEG_TO_RAD);

    let perihelion_spread = if eccentricity <= spread.climatic_precession {
        180.0
    } else {
        asin(spread.climatic_precession / eccentricity) * RAD_TO_DEG
    };

    Ok(OrbitalElements {
        years_before_present,
        eccentricity: Uncertain::new(eccentricity, spread.eccentricity)?,
        obliquity_degrees: Uncertain::new(obliquity, spread.obliquity_degrees)?,
        longitude_of_perihelion_degrees: Uncertain::new(perihelion, perihelion_spread)?,
        climatic_precession: Uncertain::new(climatic_precession, spread.climatic_precession)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(actual: f64, expected: f64, tolerance: f64) -> bool {
        abs(actual - expected) <= tolerance
    }

    fn assert_elements(
        years_before_present: f64,
        (e, perihelion, obliquity): (f64, f64, f64),
        (e_tol, perihelion_tol, obliquity_tol): (f64, f64, f64),
    ) {
        let elements = elements_at(years_before_present).expect("inside the span");
        assert!(
            close(elements.eccentricity.value, e, e_tol),
            "{years_before_present} BP: e {} vs {e}",
            elements.eccentricity.value
        );
        assert!(
            close(
                elements.longitude_of_perihelion_degrees.value,
                perihelion,
                perihelion_tol
            ),
            "{years_before_present} BP: ϖ {} vs {perihelion}",
            elements.longitude_of_perihelion_degrees.value
        );
        assert!(
            close(elements.obliquity_degrees.value, obliquity, obliquity_tol),
            "{years_before_present} BP: ε {} vs {obliquity}",
            elements.obliquity_degrees.value
        );
    }

    /// NASA GISS evaluates the same 1978 series with `ORBPAR` and publishes
    /// the result for 0–2100 CE to seven figures
    /// (`pmip3_lm_orbital_parameters.txt`, read 2026-09-25). Its `PERI-180`
    /// is Berger's ϖ. Agreement to the last printed digit is the check that
    /// the transcription and the combination of the three sums are right.
    #[test]
    fn the_giss_transcription_of_the_same_series_is_reproduced_to_seven_figures() {
        for (year, e, obliquity, perihelion) in [
            (1.0, 0.017_465_4, 23.695_250, 68.836_61),
            (1000.0, 0.017_097_4, 23.569_266, 85.811_29),
            (1950.0, 0.016_723_9, 23.446_271, 102.039_05),
            (2000.0, 0.016_703_7, 23.439_768, 102.895_49),
            (2100.0, 0.016_662_9, 23.426_761, 104.609_09),
        ] {
            assert_elements(
                years_before_present_from_year(year),
                (e, perihelion, obliquity),
                (1e-7, 1e-5, 1e-6),
            );
        }
    }

    /// The author's own tabulation of the 1978 solution, NOAA
    /// paleoclimatology file `bein1.dat` (0–100 kyr BP at 1 kyr steps), read
    /// 2026-09-25. Rounded as the file prints it: e to 1e-6, angles to 0.01°
    /// and 0.001°, the precession parameter to 1e-5.
    #[test]
    fn the_authors_own_1978_tables_are_reproduced_to_their_printed_digits() {
        for (kyr, e, perihelion, obliquity, precession) in [
            (0.0, 0.016_724, 102.04, 23.446, 0.016_36),
            (6.0, 0.018_682, 0.87, 24.105, 0.000_28),
            (11.0, 0.019_529, 278.41, 24.201, -0.019_32),
            (21.0, 0.018_994, 114.42, 22.949, 0.017_29),
            (100.0, 0.038_742, 358.49, 23.709, -0.001_02),
        ] {
            assert_elements(kyr * 1000.0, (e, perihelion, obliquity), (5e-7, 5e-3, 5e-4));
            let elements = elements_at(kyr * 1000.0).expect("inside the span");
            assert!(
                close(elements.climatic_precession.value, precession, 5e-6),
                "{kyr} kyr: e sin ϖ {}",
                elements.climatic_precession.value
            );
        }
    }

    /// PMIP3 and PMIP4 prescribe Berger (1978) for their mid-Holocene and
    /// Last Glacial Maximum experiments and print the values: 6 ka
    /// e = 0.018682, ε = 24.105°, perihelion − 180 = 0.87°; 21 ka
    /// e = 0.018994, ε = 22.949°, perihelion − 180 = 114.42°
    /// (Braconnot et al. 2012; the PMIP design pages, read 2026-09-25).
    #[test]
    fn the_pmip_experiment_epochs_match() {
        assert_elements(6_000.0, (0.018_682, 0.87, 24.105), (1e-6, 0.01, 1e-3));
        assert_elements(21_000.0, (0.018_994, 114.42, 22.949), (1e-6, 0.01, 1e-3));
    }

    /// The present-day values against the short-term models: Meeus's mean
    /// obliquity (Laskar's polynomial, *Astronomical Algorithms* 22.3, the
    /// one `hc-astro` carries) at T = −0.5 centuries, 23.4457°, and the
    /// J2000 eccentricity 0.016 709 (Meeus 31.A). Berger's series is coarser
    /// than either and agrees with both to its own spread.
    #[test]
    fn the_present_day_obliquity_and_eccentricity_match_the_short_term_models() {
        let now = elements_at(0.0).expect("the epoch is inside the span");
        // Meeus 22.3 at T = -0.5: 23°26′21.448″ + 46.8150″ × 0.5 − 0.00059″ × 0.25 + ...
        let meeus_1950 = 23.0 + 26.0 / 60.0 + (21.448 + 46.815 * 0.5 - 0.000_59 * 0.25) / 3600.0;
        assert!(
            close(now.obliquity_degrees.value, meeus_1950, 0.001),
            "ε(1950) {} vs Meeus {meeus_1950}",
            now.obliquity_degrees.value
        );
        assert!(close(now.obliquity_degrees.value, 23.44, 0.01));
        let j2000 = elements_at(-50.0).expect("the year 2000 is inside the span");
        assert!(close(j2000.eccentricity.value, 0.016_709, 1e-5));
        assert!(close(now.eccentricity.value, 0.0167, 1e-4));
        // And J2000's longitude of perihelion, 102.937° (Meeus 31.A), within a tenth
        // of a degree — the series was fitted to older constants.
        assert!(close(
            j2000.longitude_of_perihelion_degrees.value,
            102.937,
            0.1
        ));
    }

    #[test]
    fn a_million_years_is_answered_and_a_year_more_is_refused() {
        assert!(elements_at(1_000_000.0).is_ok());
        assert!(elements_at(-1_000_000.0).is_ok());
        assert_eq!(
            elements_at(1_000_001.0).unwrap_err(),
            OrbitalError::OutsideValidSpan
        );
        assert_eq!(
            elements_at(-1_000_001.0).unwrap_err(),
            OrbitalError::OutsideValidSpan
        );
        assert_eq!(elements_at(f64::NAN).unwrap_err(), OrbitalError::NotFinite);
        assert_eq!(
            elements_at(f64::INFINITY).unwrap_err(),
            OrbitalError::NotFinite
        );
        assert_eq!(
            spread_at(2_000_000.0).unwrap_err(),
            OrbitalError::OutsideValidSpan
        );
    }

    #[test]
    fn the_year_conversion_round_trips_and_counts_from_1950() {
        assert_eq!(years_before_present_from_year(1950.0), 0.0);
        assert_eq!(years_before_present_from_year(2000.0), -50.0);
        assert_eq!(years_before_present_from_year(-19_050.0), 21_000.0);
        for year in [-998_050.0, -50.0, 0.0, 1.0, 1950.0, 2026.0, 1_001_950.0] {
            assert_eq!(
                year_from_years_before_present(years_before_present_from_year(year)),
                year
            );
        }
        let lgm = elements_at(21_000.0).expect("inside the span");
        assert_eq!(lgm.year(), -19_050.0);
    }

    /// Across the whole span at 1 kyr steps the elements stay in the ranges
    /// the literature gives them: e in (0, 0.06), ε between 22° and 24.5°,
    /// ϖ a bearing, and e sin ϖ what its name says.
    #[test]
    fn the_elements_stay_physical_across_the_span() {
        let mut kyr = -1000;
        while kyr <= 1000 {
            let elements = elements_at(f64::from(kyr) * 1000.0).expect("inside the span");
            let e = elements.eccentricity.value;
            assert!(e > 0.0 && e < 0.06, "{kyr} kyr: e {e}");
            let obliquity = elements.obliquity_degrees.value;
            assert!(
                (22.0..24.5).contains(&obliquity),
                "{kyr} kyr: ε {obliquity}"
            );
            let perihelion = elements.longitude_of_perihelion_degrees.value;
            assert!(
                (0.0..360.0).contains(&perihelion),
                "{kyr} kyr: ϖ {perihelion}"
            );
            assert!(close(
                elements.climatic_precession.value,
                e * sin(perihelion * DEG_TO_RAD),
                1e-12
            ));
            kyr += 1;
        }
    }

    #[test]
    fn the_spread_widens_with_distance_and_the_perihelion_spread_follows_the_eccentricity() {
        let near = spread_at(50_000.0).expect("inside");
        let mid = spread_at(500_000.0).expect("inside");
        let far = spread_at(-900_000.0).expect("inside");
        assert!(near.eccentricity < mid.eccentricity && mid.eccentricity < far.eccentricity);
        assert!(near.obliquity_degrees < far.obliquity_degrees);
        // At present e ≈ 0.0167 and the precession spread 0.0025: about 8.6°.
        let now = elements_at(0.0).expect("inside");
        assert!(close(now.longitude_of_perihelion_degrees.std_dev, 8.6, 0.2));
        assert_eq!(now.eccentricity.std_dev, near.eccentricity);
        // Somewhere in the span e falls below its own spread and ϖ is unknown.
        let mut unknown = false;
        let mut kyr = -1000;
        while kyr <= 1000 {
            let elements = elements_at(f64::from(kyr) * 1000.0).expect("inside the span");
            if elements.longitude_of_perihelion_degrees.std_dev == 180.0 {
                unknown = true;
                assert!(elements.eccentricity.value <= elements.climatic_precession.std_dev);
            }
            kyr += 1;
        }
        assert!(unknown, "e never fell below the precession spread?");
    }

    #[test]
    fn the_derived_angles_are_consistent() {
        let now = elements_at(0.0).expect("inside");
        let radians = now.obliquity_radians().expect("finite");
        assert!(close(
            radians.value * RAD_TO_DEG,
            now.obliquity_degrees.value,
            1e-12
        ));
        let perihelion = now.longitude_of_perihelion_radians().expect("finite");
        assert!(close(
            perihelion.value * RAD_TO_DEG,
            now.longitude_of_perihelion_degrees.value,
            1e-12
        ));
        // The Sun's longitude at perihelion in early January, about 282°.
        assert!(close(
            now.perihelion_solar_longitude_degrees(),
            282.04,
            0.01
        ));
        assert_eq!(now.source(), SOURCE);
    }
}
