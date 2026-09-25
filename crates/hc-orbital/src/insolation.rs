//! Daily mean insolation at the top of the atmosphere, from the elements.
//!
//! Berger's formula for the mean irradiance over one day at latitude φ when
//! the Sun's true longitude is λ, as his program `insol14.f` (subroutine
//! `DAYINS`) evaluates it:
//!
//! ```text
//! ρ  = (1 − e²) / (1 + e cos(λ − (ϖ + 180°)))       Earth–Sun distance in AU
//! S  = S₀ / (π ρ²)
//! sin δ = sin ε sin λ                               the declination
//! H₀ = arccos(−tan φ tan δ)                          the half day arc
//! W  = S (H₀ sin φ sin δ + cos φ cos δ sin H₀)        W m⁻²
//! ```
//!
//! with `W = S π sin φ sin δ` in polar day and `0` in polar night. The
//! `ϖ + 180°` is the Sun's longitude at perihelion; ϖ itself is the
//! heliocentric one the elements carry (see [`crate::elements`]).
//!
//! The solar constant is a parameter, because it is a measurement with a
//! history: Berger's 1978 tables took 1.95 cal cm⁻² min⁻¹, the 1991 tables
//! 1360 W m⁻², the 2014 program 1368 W m⁻², and the modern value is
//! 1361 W m⁻² (Kopp & Lean 2011). The named constants below are the ones
//! the reference tables used, so a comparison with them is exact; a caller
//! reproducing a modern paper passes that paper's value.
//!
//! What this module does not do: turn a calendar date into a solar
//! longitude. Berger's program does that with the mean anomaly and a
//! 365-day year, a convention that differs from a real calendar's by up to
//! a day; the honest input is the longitude, and `hc-astro` gives the
//! present-day date of a longitude if one is needed.

use hc_core::math::{DEG_TO_RAD, abs, acos, cos, sin, sqrt};

use crate::elements::{OrbitalElements, elements_at};
use crate::error::{OrbitalError, OrbitalResult};

/// The solar constant of Berger's 1978 tables, in W m⁻²: 1.95 cal cm⁻² min⁻¹
/// (NOAA `readme_insolation.txt`: "Solar constant taken as 1.95 cal/cm2/min
/// for 1978 solution"), converted at 4.184 J per calorie.
pub const SOLAR_CONSTANT_BERGER_1978: f64 = 1.95 * 4.184e4 / 60.0;

/// The solar constant of Berger & Loutre's 1991 tables, in W m⁻², 1360
/// (the same readme: "and 1360 W/m2 for 1991").
pub const SOLAR_CONSTANT_BERGER_LOUTRE_1991: f64 = 1360.0;

/// The solar constant of the author's 2014 program `insol14.f`, in W m⁻²:
/// `SS=1368.0D0`.
pub const SOLAR_CONSTANT_INSOL14: f64 = 1368.0;

/// The Sun's true longitude at Berger's "mid-June", 90°: the June solstice.
///
/// Berger's mid-month insolations are taken at longitudes 0°, 30°, …, 330°
/// for March, April, …, February, so mid-June is the solstice and mid-July
/// is 120°. The NOAA `orbit91` "65N Jul" column is the latter.
pub const MID_JUNE_SOLAR_LONGITUDE: f64 = 90.0;

/// The latitude the ice-age literature quotes: 65° north.
pub const LATITUDE_65N: f64 = 65.0;

/// The daily mean insolation, in W m⁻², at a latitude when the Sun's true
/// longitude is `solar_longitude_degrees`, for the given elements and solar
/// constant.
///
/// # Errors
///
/// [`OrbitalError::LatitudeOutOfRange`] outside ±90°, and
/// [`OrbitalError::NotFinite`] for a non-finite longitude or solar constant.
pub fn daily_insolation(
    elements: &OrbitalElements,
    latitude_degrees: f64,
    solar_longitude_degrees: f64,
    solar_constant: f64,
) -> OrbitalResult<f64> {
    if !latitude_degrees.is_finite() || abs(latitude_degrees) > 90.0 {
        return Err(OrbitalError::LatitudeOutOfRange);
    }
    if !solar_longitude_degrees.is_finite() || !solar_constant.is_finite() {
        return Err(OrbitalError::NotFinite);
    }
    let e = elements.eccentricity.value;
    let perihelion_solar = elements.perihelion_solar_longitude_degrees();
    let obliquity = elements.obliquity_degrees.value;

    let true_anomaly = (solar_longitude_degrees - perihelion_solar) * DEG_TO_RAD;
    let distance = (1.0 - e * e) / (1.0 + e * cos(true_anomaly));
    let s = solar_constant / core::f64::consts::PI / (distance * distance);

    let sin_declination = sin(obliquity * DEG_TO_RAD) * sin(solar_longitude_degrees * DEG_TO_RAD);
    let cos_declination = sqrt(1.0 - sin_declination * sin_declination);
    let latitude = latitude_degrees * DEG_TO_RAD;
    let sp = sin_declination * sin(latitude);
    let cp = cos_declination * cos(latitude);

    // Polar day or night: the Sun's declination reaches past the co-latitude.
    // `cos(H₀) = -sp/cp` then leaves [-1, 1].
    if abs(sp) >= cp {
        return Ok(if sp > 0.0 {
            s * sp * core::f64::consts::PI
        } else {
            0.0
        });
    }
    let cos_half_arc = -sp / cp;
    let half_arc = acos(cos_half_arc);
    Ok(s * (half_arc * sp + cp * sqrt(1.0 - cos_half_arc * cos_half_arc)))
}

/// The daily mean insolation at 65° N at the June solstice, in W m⁻², for
/// an epoch in years before 1950 and a solar constant.
///
/// This is the curve Milankovitch theory is usually told with. In this
/// series it peaks at about 526.5 W m⁻² near 11 100 years before present
/// (for [`SOLAR_CONSTANT_BERGER_LOUTRE_1991`]), against 477.6 at 1950.
///
/// # Errors
///
/// As [`elements_at`], and [`OrbitalError::NotFinite`] for a non-finite
/// solar constant.
///
/// ```
/// use hc_orbital::{SOLAR_CONSTANT_BERGER_LOUTRE_1991, insolation_65n_june};
///
/// let now = insolation_65n_june(0.0, SOLAR_CONSTANT_BERGER_LOUTRE_1991).unwrap();
/// let early_holocene = insolation_65n_june(11_000.0, SOLAR_CONSTANT_BERGER_LOUTRE_1991).unwrap();
/// assert!((now - 477.6).abs() < 0.1);
/// assert!(early_holocene - now > 45.0);
/// ```
pub fn insolation_65n_june(years_before_present: f64, solar_constant: f64) -> OrbitalResult<f64> {
    let elements = elements_at(years_before_present)?;
    daily_insolation(
        &elements,
        LATITUDE_65N,
        MID_JUNE_SOLAR_LONGITUDE,
        solar_constant,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(actual: f64, expected: f64, tolerance: f64) -> bool {
        abs(actual - expected) <= tolerance
    }

    /// The author's own 1978 insolation tables, NOAA `bein1.dat`, print
    /// mid-June (λ = 90°) at 60° N and 70° N in whole langleys per day at
    /// 1.95 cal cm⁻² min⁻¹; one langley per day is 0.4843 W m⁻², so the
    /// rounding is ±0.25 W m⁻².
    #[test]
    fn the_authors_own_1978_insolation_tables_are_reproduced_within_their_rounding() {
        for (kyr, latitude, langleys) in [
            (0.0, 60.0, 984.0),
            (0.0, 70.0, 1016.0),
            (6.0, 60.0, 1035.0),
            (6.0, 70.0, 1078.0),
            (11.0, 60.0, 1079.0),
            (11.0, 70.0, 1125.0),
            (21.0, 60.0, 969.0),
            (21.0, 70.0, 994.0),
            (100.0, 60.0, 1029.0),
            (100.0, 70.0, 1066.0),
        ] {
            let elements = elements_at(kyr * 1000.0).expect("inside the span");
            let w = daily_insolation(
                &elements,
                latitude,
                MID_JUNE_SOLAR_LONGITUDE,
                SOLAR_CONSTANT_BERGER_1978,
            )
            .expect("a valid latitude");
            let expected = langleys * 0.4843;
            assert!(
                close(w, expected, 0.3),
                "{kyr} kyr, {latitude}°N: {w} vs {expected}"
            );
        }
    }

    /// Berger & Loutre's 1991 table (NOAA `orbit91`, column "65NJul", mid-July
    /// λ = 120°, S₀ = 1360) is a different solution; agreement to under
    /// 1 W m⁻² over the last 100 kyr is the measured closeness of the two.
    #[test]
    fn the_1991_tables_65n_july_agree_to_under_one_watt_over_the_last_100_kyr() {
        for (kyr, expected) in [
            (0.0, 426.76),
            (6.0, 458.31),
            (10.0, 469.44),
            (11.0, 467.92),
            (21.0, 418.62),
            (100.0, 464.41),
        ] {
            let elements = elements_at(kyr * 1000.0).expect("inside the span");
            let w = daily_insolation(&elements, 65.0, 120.0, SOLAR_CONSTANT_BERGER_LOUTRE_1991)
                .expect("a valid latitude");
            assert!(close(w, expected, 1.0), "{kyr} kyr: {w} vs {expected}");
        }
    }

    /// The early-Holocene maximum of 65° N June insolation, the peak that
    /// ended the last glaciation, lies between 10 and 12 kyr BP and is
    /// about 50 W m⁻² above the present value.
    #[test]
    fn the_65n_june_maximum_of_the_last_30_kyr_is_in_the_early_holocene() {
        let now = insolation_65n_june(0.0, SOLAR_CONSTANT_BERGER_LOUTRE_1991).expect("inside");
        assert!(close(now, 477.63, 0.05), "now {now}");
        let mut best = (0.0_f64, 0.0_f64);
        let mut step = 0;
        while step <= 300 {
            let bp = f64::from(step) * 100.0;
            let w = insolation_65n_june(bp, SOLAR_CONSTANT_BERGER_LOUTRE_1991).expect("inside");
            if w > best.0 {
                best = (w, bp);
            }
            step += 1;
        }
        assert!(
            (10_000.0..=12_000.0).contains(&best.1),
            "peak at {} BP",
            best.1
        );
        assert!(close(best.0, 526.5, 0.5), "peak {}", best.0);
        // The Last Glacial Maximum sits below the present.
        let lgm = insolation_65n_june(21_000.0, SOLAR_CONSTANT_BERGER_LOUTRE_1991).expect("inside");
        assert!(lgm < now);
    }

    #[test]
    fn polar_night_is_zero_and_polar_day_is_the_whole_circle() {
        let now = elements_at(0.0).expect("inside");
        let s0 = SOLAR_CONSTANT_BERGER_LOUTRE_1991;
        // The south pole at the June solstice: night.
        assert_eq!(daily_insolation(&now, -90.0, 90.0, s0).expect("valid"), 0.0);
        // The north pole at the June solstice: S π sin δ, with δ = ε.
        let pole = daily_insolation(&now, 90.0, 90.0, s0).expect("valid");
        assert!(close(pole, 523.9, 1.0), "pole {pole}");
        // And the equator at an equinox gets S₀/π scaled by the distance.
        let equator = daily_insolation(&now, 0.0, 0.0, s0).expect("valid");
        assert!(close(equator, 436.2, 1.0), "equator {equator}");
        // Continuity across the polar circle: just inside and just outside.
        let inside = daily_insolation(&now, 66.55, 90.0, s0).expect("valid");
        let outside = daily_insolation(&now, 66.56, 90.0, s0).expect("valid");
        assert!(close(inside, outside, 0.1));
    }

    #[test]
    fn a_latitude_off_the_globe_and_a_non_finite_input_are_refused() {
        let now = elements_at(0.0).expect("inside");
        assert_eq!(
            daily_insolation(&now, 91.0, 90.0, 1360.0).unwrap_err(),
            OrbitalError::LatitudeOutOfRange
        );
        assert_eq!(
            daily_insolation(&now, f64::NAN, 90.0, 1360.0).unwrap_err(),
            OrbitalError::LatitudeOutOfRange
        );
        assert_eq!(
            daily_insolation(&now, 65.0, f64::NAN, 1360.0).unwrap_err(),
            OrbitalError::NotFinite
        );
        assert_eq!(
            insolation_65n_june(0.0, f64::INFINITY).unwrap_err(),
            OrbitalError::NotFinite
        );
        assert_eq!(
            insolation_65n_june(2_000_000.0, 1360.0).unwrap_err(),
            OrbitalError::OutsideValidSpan
        );
    }

    #[test]
    fn the_1978_solar_constant_is_the_langley_value_in_watts() {
        assert!(close(SOLAR_CONSTANT_BERGER_1978, 1359.8, 0.01));
    }
}
