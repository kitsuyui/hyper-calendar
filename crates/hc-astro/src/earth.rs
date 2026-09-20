//! The orientation of the Earth: obliquity, nutation and sidereal time.
//!
//! These three are what turn a position on the ecliptic into a position in
//! the sky over a particular place, so the solar-term, lunar-phase and
//! rise/set code all funnel through here.
//!
//! Sources: Meeus, *Astronomical Algorithms*, 2nd ed., chapters 12 (sidereal
//! time), 13 (coordinate transformation) and 22 (nutation and obliquity).
//! The nutation series is Meeus's abridged one — four terms in Δψ and four
//! in Δε — which he states as good to 0.5″ in longitude and 0.1″ in
//! obliquity. That is two orders of magnitude finer than anything a calendar
//! rule can notice, so the full 63-term IAU 1980 table is deliberately not
//! carried here.

use hc_calendar::fixed::Moment;
use hc_core::math::{DEG_TO_RAD, RAD_TO_DEG, asin, atan2, cos_deg, normalize_degrees, sin_deg};

use crate::time::{J2000, JULIAN_CENTURY_DAYS, julian_centuries};
use crate::util::{clamp, poly};

/// The two nutation angles, in degrees.
///
/// Nutation is the short-period wobble of the Earth's axis, dominated by the
/// 18.6-year precession of the Moon's orbital node. It shifts apparent
/// longitudes by up to about 17″, which is well above this crate's claimed
/// accuracy and so cannot be dropped.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Nutation {
    /// Nutation in longitude, Δψ, in degrees.
    pub longitude_degrees: f64,
    /// Nutation in obliquity, Δε, in degrees.
    pub obliquity_degrees: f64,
}

/// A direction in equatorial coordinates, in degrees.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Equatorial {
    /// Right ascension, measured eastward from the equinox, in `[0, 360)`.
    pub right_ascension_degrees: f64,
    /// Declination, positive north of the celestial equator.
    pub declination_degrees: f64,
}

/// The mean and true obliquity of the ecliptic together, in degrees.
///
/// They differ by the nutation in obliquity, at most about 9.2″. Which one a
/// caller wants depends on whether its ecliptic longitude is a mean or an
/// apparent one; mixing them is the classic way to lose ten arcseconds.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Obliquity {
    /// The mean obliquity, with no nutation.
    pub mean_degrees: f64,
    /// The true obliquity: mean plus nutation in obliquity.
    pub true_degrees: f64,
}

/// One arcsecond in degrees, the unit the nutation and obliquity series are
/// printed in.
const ARCSECOND: f64 = 1.0 / 3600.0;

/// The nutation angles for a Universal Time moment.
#[must_use]
pub fn nutation(moment: Moment) -> Nutation {
    nutation_at_centuries(julian_centuries(moment))
}

/// The nutation angles for a count of Julian centuries of TT since J2000.0.
///
/// Meeus (22.1). Ω is the longitude of the Moon's ascending node, `L` and
/// `l` the mean longitudes of the Sun and the Moon.
#[must_use]
pub fn nutation_at_centuries(centuries: f64) -> Nutation {
    let t = centuries;
    let node = poly(
        t,
        &[125.044_52, -1_934.136_261, 0.002_070_8, 1.0 / 450_000.0],
    );
    let solar_longitude = poly(t, &[280.4665, 36_000.769_8]);
    let lunar_longitude = poly(t, &[218.3165, 481_267.881_3]);

    let longitude_arcseconds = -17.20 * sin_deg(node)
        - 1.32 * sin_deg(2.0 * solar_longitude)
        - 0.23 * sin_deg(2.0 * lunar_longitude)
        + 0.21 * sin_deg(2.0 * node);
    let obliquity_arcseconds = 9.20 * cos_deg(node)
        + 0.57 * cos_deg(2.0 * solar_longitude)
        + 0.10 * cos_deg(2.0 * lunar_longitude)
        - 0.09 * cos_deg(2.0 * node);

    Nutation {
        longitude_degrees: longitude_arcseconds * ARCSECOND,
        obliquity_degrees: obliquity_arcseconds * ARCSECOND,
    }
}

/// The mean obliquity of the ecliptic, in degrees, for a Universal Time
/// moment.
#[must_use]
pub fn mean_obliquity(moment: Moment) -> f64 {
    mean_obliquity_at_centuries(julian_centuries(moment))
}

/// The mean obliquity of the ecliptic, in degrees, for a count of Julian
/// centuries of TT since J2000.0.
///
/// Meeus (22.3), the Laskar expansion in `U = T/100`. Laskar quotes 0.01″
/// over 1000 years either side of J2000 and a few arcseconds over 10 000
/// years; outside ±10 000 years the series diverges and this function must
/// not be believed.
#[must_use]
pub fn mean_obliquity_at_centuries(centuries: f64) -> f64 {
    let u = centuries / 100.0;
    let base = 23.0 + 26.0 / 60.0 + 21.448 * ARCSECOND;
    base + ARCSECOND
        * poly(
            u,
            &[
                0.0, -4_680.93, -1.55, 1_999.25, -51.38, -249.67, -39.05, 7.12, 27.87, 5.79, 2.45,
            ],
        )
}

/// The true obliquity of the ecliptic, in degrees: mean obliquity plus
/// nutation in obliquity.
#[must_use]
pub fn true_obliquity(moment: Moment) -> f64 {
    true_obliquity_at_centuries(julian_centuries(moment))
}

/// The true obliquity of the ecliptic for a count of Julian centuries of TT
/// since J2000.0.
#[must_use]
pub fn true_obliquity_at_centuries(centuries: f64) -> f64 {
    mean_obliquity_at_centuries(centuries) + nutation_at_centuries(centuries).obliquity_degrees
}

/// Both obliquities for a Universal Time moment.
#[must_use]
pub fn obliquity(moment: Moment) -> Obliquity {
    obliquity_at_centuries(julian_centuries(moment))
}

/// Both obliquities for a count of Julian centuries of TT since J2000.0.
#[must_use]
pub fn obliquity_at_centuries(centuries: f64) -> Obliquity {
    let mean = mean_obliquity_at_centuries(centuries);
    Obliquity {
        mean_degrees: mean,
        true_degrees: mean + nutation_at_centuries(centuries).obliquity_degrees,
    }
}

/// Mean sidereal time at Greenwich, in degrees, for a Universal Time moment.
///
/// Meeus (12.4). Sidereal time is a measure of the Earth's rotation, so
/// unlike everything else in this crate it takes Universal Time directly and
/// must not be handed a TT moment.
#[must_use]
pub fn mean_sidereal_time(moment: Moment) -> f64 {
    let days = moment.0 - J2000.0;
    let t = days / JULIAN_CENTURY_DAYS;
    normalize_degrees(
        280.460_618_37 + 360.985_647_366_29 * days + 0.000_387_933 * t * t
            - t * t * t / 38_710_000.0,
    )
}

/// Apparent sidereal time at Greenwich, in degrees: mean sidereal time
/// corrected for nutation (the "equation of the equinoxes").
#[must_use]
pub fn apparent_sidereal_time(moment: Moment) -> f64 {
    let centuries = julian_centuries(moment);
    let correction = nutation_at_centuries(centuries).longitude_degrees
        * cos_deg(true_obliquity_at_centuries(centuries));
    normalize_degrees(mean_sidereal_time(moment) + correction)
}

/// Convert an ecliptic longitude and latitude into equatorial coordinates.
///
/// Meeus (13.3) and (13.4). All three arguments and both results are in
/// degrees.
#[must_use]
pub fn equatorial_from_ecliptic(
    longitude_degrees: f64,
    latitude_degrees: f64,
    obliquity_degrees: f64,
) -> Equatorial {
    let sin_lon = sin_deg(longitude_degrees);
    let cos_lon = cos_deg(longitude_degrees);
    let sin_lat = sin_deg(latitude_degrees);
    let cos_lat = cos_deg(latitude_degrees);
    let sin_obl = sin_deg(obliquity_degrees);
    let cos_obl = cos_deg(obliquity_degrees);

    let right_ascension = atan2(sin_lon * cos_obl - (sin_lat / cos_lat) * sin_obl, cos_lon);
    let declination = asin(sin_lat * cos_obl + cos_lat * sin_obl * sin_lon);

    Equatorial {
        right_ascension_degrees: normalize_degrees(right_ascension * RAD_TO_DEG),
        declination_degrees: declination * RAD_TO_DEG,
    }
}

/// The altitude of a direction above the horizon at a moment and a longitude
/// and latitude, in degrees, ignoring refraction.
///
/// This is the plain spherical-triangle relation
/// `sin h = sin φ sin δ + cos φ cos δ cos H`.
#[must_use]
pub fn altitude_degrees(
    position: Equatorial,
    local_hour_angle_degrees: f64,
    latitude_degrees: f64,
) -> f64 {
    let sine = sin_deg(latitude_degrees) * sin_deg(position.declination_degrees)
        + cos_deg(latitude_degrees)
            * cos_deg(position.declination_degrees)
            * cos_deg(local_hour_angle_degrees);
    asin(clamp(sine, -1.0, 1.0)) * RAD_TO_DEG
}

/// The hour angle of a direction, in degrees, at a moment and an east-positive
/// longitude.
#[must_use]
pub fn local_hour_angle(position: Equatorial, moment: Moment, longitude_degrees: f64) -> f64 {
    normalize_degrees(
        apparent_sidereal_time(moment) + longitude_degrees - position.right_ascension_degrees,
    )
}

/// Degrees to radians, re-exported so that callers converting their own
/// angles do not reach past this crate for it.
pub const DEGREES_TO_RADIANS: f64 = DEG_TO_RAD;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::universal_time;

    /// Meeus, example 22.a: 1987 April 10.0 TD, T = −0.127296372348.
    const EXAMPLE_22A_CENTURIES: f64 = -0.127_296_372_348;

    #[test]
    fn the_mean_obliquity_matches_meeus_example_22a() {
        // Meeus gives ε₀ = 23° 26′ 27.407″.
        let expected = 23.0 + 26.0 / 60.0 + 27.407 / 3600.0;
        let actual = mean_obliquity_at_centuries(EXAMPLE_22A_CENTURIES);
        assert!(
            (actual - expected).abs() < 1e-6,
            "obliquity was {actual}, expected {expected}"
        );
    }

    #[test]
    fn the_true_obliquity_matches_meeus_example_22a() {
        // Meeus gives ε = 23° 26′ 36.850″ from the full nutation table; the
        // abridged series is quoted as good to 0.1″ in Δε.
        let expected = 23.0 + 26.0 / 60.0 + 36.850 / 3600.0;
        let actual = true_obliquity_at_centuries(EXAMPLE_22A_CENTURIES);
        assert!(
            (actual - expected).abs() < 0.2 / 3600.0,
            "obliquity was {actual}, expected {expected}"
        );
    }

    #[test]
    fn the_nutation_in_longitude_matches_meeus_example_22a() {
        // Meeus gives Δψ = −3.788″ from the full table; the abridged series
        // is quoted as good to 0.5″.
        let actual = nutation_at_centuries(EXAMPLE_22A_CENTURIES).longitude_degrees * 3600.0;
        assert!((actual + 3.788).abs() < 0.5, "nutation was {actual}\"");
    }

    #[test]
    fn the_obliquity_at_j2000_is_the_standard_value() {
        // IAU 1976 / Laskar: 23° 26′ 21.448″ at J2000.0 exactly.
        let value = mean_obliquity_at_centuries(0.0);
        assert!((value - 23.439_291_111).abs() < 1e-8, "obliquity {value}");
    }

    #[test]
    fn the_obliquity_decreases_by_about_forty_seven_arcseconds_a_century() {
        let now = mean_obliquity_at_centuries(0.0);
        let later = mean_obliquity_at_centuries(1.0);
        let change = (later - now) * 3600.0;
        assert!((change + 46.81).abs() < 0.2, "change was {change}\"");
    }

    #[test]
    fn nutation_in_longitude_stays_inside_its_known_amplitude() {
        // The 18.6-year term has an amplitude of 17.2″; the whole abridged
        // series can never exceed the sum of its coefficients.
        for step in 0..400 {
            let centuries = -2.0 + f64::from(step) / 100.0;
            let value = nutation_at_centuries(centuries).longitude_degrees * 3600.0;
            assert!(value.abs() < 19.0, "nutation {value}\" at {centuries}");
        }
    }

    #[test]
    fn nutation_repeats_on_the_eighteen_point_six_year_node_cycle() {
        let base = nutation_at_centuries(0.0).longitude_degrees;
        let later = nutation_at_centuries(18.613 / 100.0).longitude_degrees;
        assert!(
            (base - later).abs() < 1.5 / 3600.0,
            "difference {}\"",
            (base - later) * 3600.0
        );
    }

    /// Meeus, example 12.a: at 1987 April 10, 0h UT (JD 2446895.5) the mean
    /// sidereal time at Greenwich is 13h10m46.3668s = 197.693195°.
    #[test]
    fn mean_sidereal_time_matches_meeus_example_12a() {
        let moment = Moment::from_julian_date(2_446_895.5);
        let actual = mean_sidereal_time(moment);
        assert!(
            (actual - 197.693_195).abs() < 1e-5,
            "sidereal time was {actual}"
        );
    }

    /// Meeus, example 12.b: 1987 April 10 at 19h21m00s UT gives
    /// 8h34m57.0896s = 128.737873°.
    #[test]
    fn mean_sidereal_time_matches_meeus_example_12b() {
        let moment = Moment::from_julian_date(2_446_896.306_25);
        let actual = mean_sidereal_time(moment);
        assert!(
            (actual - 128.737_873).abs() < 1e-5,
            "sidereal time was {actual}"
        );
    }

    #[test]
    fn apparent_sidereal_time_differs_from_mean_by_the_equation_of_the_equinoxes() {
        let moment = Moment::from_julian_date(2_446_895.5);
        let difference = (apparent_sidereal_time(moment) - mean_sidereal_time(moment)) * 3600.0;
        // Meeus gives −0.2317 s of time = −3.4755″ of arc for this instant.
        assert!(difference.abs() < 20.0, "difference was {difference}\"");
        assert!(
            (difference + 3.48).abs() < 0.6,
            "difference was {difference}\""
        );
    }

    #[test]
    fn sidereal_time_advances_by_a_full_turn_plus_a_degree_each_day() {
        let a = mean_sidereal_time(Moment(730_000.0));
        let b = mean_sidereal_time(Moment(730_001.0));
        let advance = crate::util::modulo(b - a, 360.0);
        assert!((advance - 0.985_647).abs() < 1e-4, "advance was {advance}");
    }

    /// Meeus, example 13.a: λ = 113.215630°, β = 6.684170°, ε = 23.4392911°
    /// gives α = 116.328942°, δ = 28.026183°.
    #[test]
    fn ecliptic_to_equatorial_matches_meeus_example_13a() {
        let position = equatorial_from_ecliptic(113.215_630, 6.684_170, 23.439_291_1);
        assert!(
            (position.right_ascension_degrees - 116.328_942).abs() < 1e-5,
            "right ascension {}",
            position.right_ascension_degrees
        );
        assert!(
            (position.declination_degrees - 28.026_183).abs() < 1e-5,
            "declination {}",
            position.declination_degrees
        );
    }

    #[test]
    fn the_equinoxes_and_solstices_sit_where_the_obliquity_puts_them() {
        let obliquity = 23.439_291_1;
        let spring = equatorial_from_ecliptic(0.0, 0.0, obliquity);
        assert!(spring.right_ascension_degrees.abs() < 1e-9);
        assert!(spring.declination_degrees.abs() < 1e-9);

        let summer = equatorial_from_ecliptic(90.0, 0.0, obliquity);
        assert!((summer.right_ascension_degrees - 90.0).abs() < 1e-9);
        assert!((summer.declination_degrees - obliquity).abs() < 1e-9);

        let winter = equatorial_from_ecliptic(270.0, 0.0, obliquity);
        assert!((winter.declination_degrees + obliquity).abs() < 1e-9);
    }

    #[test]
    fn altitude_peaks_at_transit_and_bottoms_half_a_turn_later() {
        let position = Equatorial {
            right_ascension_degrees: 0.0,
            declination_degrees: 0.0,
        };
        let transit = altitude_degrees(position, 0.0, 35.0);
        let anti = altitude_degrees(position, 180.0, 35.0);
        assert!((transit - 55.0).abs() < 1e-9, "transit altitude {transit}");
        assert!((anti + 55.0).abs() < 1e-9, "anti-transit altitude {anti}");
    }

    #[test]
    fn a_hour_angle_is_zero_when_a_body_crosses_the_local_meridian() {
        // Construct the direction whose right ascension equals the apparent
        // sidereal time at Greenwich, then look at it from Greenwich.
        let moment = Moment(730_500.25);
        let position = Equatorial {
            right_ascension_degrees: apparent_sidereal_time(moment),
            declination_degrees: 10.0,
        };
        let angle = local_hour_angle(position, moment, 0.0);
        assert!(
            !(1e-9..=360.0 - 1e-9).contains(&angle),
            "hour angle {angle}"
        );
    }

    #[test]
    fn moment_based_wrappers_agree_with_the_centuries_based_ones() {
        let tt = Moment::from_julian_date(2_446_895.5);
        let ut = universal_time(tt);
        let centuries = crate::time::julian_centuries_from_dynamical(tt);
        assert!((mean_obliquity(ut) - mean_obliquity_at_centuries(centuries)).abs() < 1e-12);
        assert!((true_obliquity(ut) - true_obliquity_at_centuries(centuries)).abs() < 1e-12);
        assert!(
            (nutation(ut).longitude_degrees - nutation_at_centuries(centuries).longitude_degrees)
                .abs()
                < 1e-12
        );
    }

    #[test]
    fn the_obliquity_pair_carries_both_values() {
        let pair = obliquity_at_centuries(EXAMPLE_22A_CENTURIES);
        assert!(
            (pair.mean_degrees - mean_obliquity_at_centuries(EXAMPLE_22A_CENTURIES)).abs() < 1e-15
        );
        assert!(
            (pair.true_degrees - true_obliquity_at_centuries(EXAMPLE_22A_CENTURIES)).abs() < 1e-15
        );
        assert!(pair.true_degrees > pair.mean_degrees);
        let from_moment = obliquity(universal_time(Moment::from_julian_date(2_446_895.5)));
        assert!((from_moment.mean_degrees - pair.mean_degrees).abs() < 1e-9);
    }

    #[test]
    fn the_degrees_to_radians_constant_is_the_one_from_core() {
        assert!((DEGREES_TO_RADIANS * 180.0 - core::f64::consts::PI).abs() < 1e-15);
    }
}
