//! The Heliocentric Julian Date: the Julian Date at which light from a
//! distant object would have reached the Sun.
//!
//! Light from a star reaches the Earth up to 8.3 minutes before or after it
//! passes the Sun, as the Earth goes round its orbit. The correction is the
//! Rømer delay, "the dot product of the unit vector from the observer to
//! the object, n̂, and the vector from the origin of the new reference frame
//! to the observer, r", divided by the speed of light, with n̂ and r "in the
//! same coordinate system" (Eastman, Siverd and Gaudi, *PASP* 122, 2010,
//! §2.1, equations 2 and 3, `eastman2010`, read 2026-09-27). With the Sun as
//! the origin, the corrected date is the HJD.
//!
//! # The time scale is part of the name
//!
//! A Julian Date can be written in any time scale, and "it is critical that
//! any stated BJD or HJD also specify the time standard used" (§2.2). So
//! nothing here is called plain HJD:
//!
//! * [`hjd_tt`] is HJD_TT, JD_TT plus the delay, with the Earth's position
//!   taken at that TT instant.
//! * [`hjd_utc`] is HJD_UTC, JD_UTC plus the same delay, the Earth's
//!   position still taken at the TT instant the caller's TT − UTC gives.
//!   Taking it at the UTC reading as if it were TT is what Eastman and
//!   his coauthors call HJD′_UTC, a different and drifting quantity, and
//!   no function here computes it.
//!
//! # How good it is
//!
//! The HJD itself "is only accurate to 8 s because of the acceleration of
//! the Sun due primarily to Jupiter and Saturn" (§2.1): the Sun is not an
//! inertial origin, and the barycentric date, BJD_TDB, is the one the IAU
//! recommends. **Not carried: BJD_TDB.** The workspace has no barycentre
//! computation yet.
//!
//! As a computation of the HJD, this one takes the Earth from VSOP87
//! ([`crate::vsop87`]) on the ecliptic of date, turns its longitude back to
//! the J2000 equinox by the general precession, and so ignores the
//! ecliptic's own motion, 47″ a century, which moves the delay by at most
//! 0.12 s a century from 2000. The IDL Astronomy Library's `helio_jd`
//! documents a comparison of its delays with SLALIB's for six objects from
//! 1940 to 2100 (`idl-helio-jd`, read 2026-09-27). This agrees with IDL's
//! in all six to 0.1 s, the precision they are printed to, and with
//! SLALIB's in the four within ten years of 2000. SLALIB's other two, 3.9 s
//! and 4.8 s off, are what the Earth on the equinox of the date gives
//! against a J2000 direction, a mixed frame. The object's
//! direction is its right ascension and declination on the mean equator and
//! equinox of J2000, and it is taken to be infinitely far away.

use hc_core::math::{DEG_TO_RAD, cos, sin};

use crate::earth::{general_precession_arcseconds, mean_obliquity_at_centuries};
use hc_core::duration::SECONDS_PER_DAY_F64;
use hc_core::epoch_notation::JULIAN_CENTURY_DAYS;

use crate::time::J2000_JULIAN_DATE;
use crate::vsop87::earth_heliocentric;

/// The seconds light takes to cross one astronomical unit: the unit of
/// 149 597 870 700 m (IAU 2012 Resolution B2, `iau-2012-b2`) over the
/// speed of light, 299 792 458 m/s, both exact.
pub const LIGHT_TIME_PER_AU_SECONDS: f64 = 149_597_870_700.0 / 299_792_458.0;

/// Arcseconds in a degree.
const ARCSECONDS_PER_DEGREE: f64 = 3_600.0;

/// The direction of a distant object.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Target {
    /// Right ascension on the mean equator and equinox of J2000, in
    /// degrees.
    pub right_ascension_degrees: f64,
    /// Declination on the mean equator and equinox of J2000, in degrees.
    pub declination_degrees: f64,
}

/// The Earth's heliocentric position on the mean equator and equinox of
/// J2000, in astronomical units, at a Julian Date of TT.
fn earth_j2000_equatorial(tt_julian_date: f64) -> [f64; 3] {
    let centuries = (tt_julian_date - J2000_JULIAN_DATE) / JULIAN_CENTURY_DAYS;
    let (longitude, latitude, radius) = earth_heliocentric(centuries / 10.0);
    // Back from the equinox of date to J2000's.
    let longitude =
        longitude - general_precession_arcseconds(centuries) / ARCSECONDS_PER_DEGREE * DEG_TO_RAD;
    let x = radius * cos(latitude) * cos(longitude);
    let y = radius * cos(latitude) * sin(longitude);
    let z = radius * sin(latitude);
    // From the J2000 ecliptic to the J2000 equator.
    let obliquity = mean_obliquity_at_centuries(0.0) * DEG_TO_RAD;
    [
        x,
        y * cos(obliquity) - z * sin(obliquity),
        y * sin(obliquity) + z * cos(obliquity),
    ]
}

/// The heliocentric light-time correction in seconds: how much later the
/// light reaches the Sun than the Earth, negative when it reaches the Sun
/// first. Eastman, Siverd and Gaudi's Δ_R⊙ = r · n̂ / c, with the Earth's
/// position at the Julian Date of TT given.
#[must_use]
pub fn heliocentric_correction_seconds(tt_julian_date: f64, target: Target) -> f64 {
    let earth = earth_j2000_equatorial(tt_julian_date);
    let alpha = target.right_ascension_degrees * DEG_TO_RAD;
    let delta = target.declination_degrees * DEG_TO_RAD;
    let direction = [cos(delta) * cos(alpha), cos(delta) * sin(alpha), sin(delta)];
    let projection = earth[0] * direction[0] + earth[1] * direction[1] + earth[2] * direction[2];
    projection * LIGHT_TIME_PER_AU_SECONDS
}

/// HJD_TT: a Julian Date of TT corrected to the Sun.
#[must_use]
pub fn hjd_tt(tt_julian_date: f64, target: Target) -> f64 {
    tt_julian_date + heliocentric_correction_seconds(tt_julian_date, target) / SECONDS_PER_DAY_F64
}

/// HJD_UTC: a Julian Date of UTC corrected to the Sun, the Earth's
/// position taken at the TT instant `tt_minus_utc_seconds` later — 32.184 s
/// plus TAI − UTC, which `hc_core::unix::tai_minus_utc_at` gives.
#[must_use]
pub fn hjd_utc(utc_julian_date: f64, tt_minus_utc_seconds: f64, target: Target) -> f64 {
    let tt_julian_date = utc_julian_date + tt_minus_utc_seconds / SECONDS_PER_DAY_F64;
    utc_julian_date + heliocentric_correction_seconds(tt_julian_date, target) / SECONDS_PER_DAY_F64
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A right ascension in hours, minutes and seconds and a declination in
    /// degrees, minutes and seconds, as a target.
    fn target(
        hours: f64,
        minutes: f64,
        seconds: f64,
        sign: f64,
        degrees: f64,
        arcmin: f64,
        arcsec: f64,
    ) -> Target {
        Target {
            right_ascension_degrees: 15.0 * (hours + minutes / 60.0 + seconds / 3_600.0),
            declination_degrees: sign * (degrees + arcmin / 60.0 + arcsec / 3_600.0),
        }
    }

    /// The Julian Date of a proleptic Gregorian date and time.
    fn julian_date(year: i64, month: u8, day: u8, hour: f64, minute: f64, second: f64) -> f64 {
        let rd = hc_calendar::gregorian::to_fixed(year, month, day).unwrap();
        // RD 0 began at JD 1 721 424.5.
        rd.0 as f64 + 1_721_424.5 + (hour + minute / 60.0 + second / 3_600.0) / 24.0
    }

    /// Wayne Warren's comparison of IDL's HELIO_JD with SLALIB, as the IDL
    /// Astronomy Library's helio_jd.pro documents it: the date, the J2000
    /// position, and HJD − JD in seconds by SLALIB and by IDL.
    fn warren_table() -> [(f64, Target, f64, f64); 6] {
        [
            (
                julian_date(1999, 10, 29, 0.0, 0.0, 0.0),
                target(21.0, 8.0, 25.0, -1.0, 67.0, 22.0, 0.0),
                -59.0,
                -59.0,
            ),
            (
                julian_date(1999, 10, 29, 0.0, 0.0, 0.0),
                target(2.0, 56.0, 33.4, 1.0, 0.0, 26.0, 55.0),
                474.1,
                474.1,
            ),
            (
                julian_date(1940, 12, 11, 6.0, 55.0, 0.0),
                target(7.0, 34.0, 41.9, -1.0, 0.0, 30.0, 42.0),
                366.3,
                370.2,
            ),
            (
                julian_date(1992, 2, 29, 3.0, 15.0, 56.2),
                target(12.0, 56.0, 27.4, 1.0, 42.0, 10.0, 17.0),
                350.8,
                350.9,
            ),
            (
                julian_date(2000, 3, 1, 10.0, 26.0, 31.8),
                target(14.0, 28.0, 36.7, -1.0, 20.0, 42.0, 11.0),
                243.7,
                243.7,
            ),
            (
                julian_date(2100, 2, 26, 9.0, 18.0, 24.2),
                target(8.0, 26.0, 51.7, 1.0, 85.0, 47.0, 28.0),
                104.0,
                108.8,
            ),
        ]
    }

    #[test]
    fn the_correction_matches_idl_and_slalib_near_2000() {
        for (date, position, slalib, idl) in warren_table() {
            let seconds = heliocentric_correction_seconds(date, position);
            // IDL carries the object to its Sun's equinox, B1950, and agrees to
            // the tenth of a second the table prints, in every row.
            assert!(
                (seconds - idl).abs() < 0.1,
                "{date}: {seconds} against IDL {idl}"
            );
            // SLALIB agrees within ten years of 2000 and not beyond.
            if (date - J2000_JULIAN_DATE).abs() < 3_653.0 {
                assert!(
                    (seconds - slalib).abs() < 0.1,
                    "{date}: {seconds} against {slalib}"
                );
            }
        }
    }

    #[test]
    fn slalibs_column_is_the_earth_on_the_equinox_of_date() {
        // Far from 2000, SLALIB's figures are 3.9 s and 4.8 s off IDL's.
        // They are what the same Earth gives on the mean equator and
        // equinox of the date against the J2000 position: a mixed frame,
        // which equation 2 of Eastman, Siverd and Gaudi rules out.
        for (date, position, slalib, _) in warren_table() {
            let centuries = (date - J2000_JULIAN_DATE) / JULIAN_CENTURY_DAYS;
            let (longitude, latitude, radius) = earth_heliocentric(centuries / 10.0);
            let (x, y, z) = (
                radius * cos(latitude) * cos(longitude),
                radius * cos(latitude) * sin(longitude),
                radius * sin(latitude),
            );
            let obliquity = mean_obliquity_at_centuries(centuries) * DEG_TO_RAD;
            let earth = [
                x,
                y * cos(obliquity) - z * sin(obliquity),
                y * sin(obliquity) + z * cos(obliquity),
            ];
            let (alpha, delta) = (
                position.right_ascension_degrees * DEG_TO_RAD,
                position.declination_degrees * DEG_TO_RAD,
            );
            let mixed = (earth[0] * cos(delta) * cos(alpha)
                + earth[1] * cos(delta) * sin(alpha)
                + earth[2] * sin(delta))
                * LIGHT_TIME_PER_AU_SECONDS;
            assert!(
                (mixed - slalib).abs() < 0.1,
                "{date}: {mixed} against {slalib}"
            );
        }
    }

    #[test]
    fn hjd_tt_adds_the_correction_in_days() {
        let date = julian_date(1992, 2, 29, 3.0, 15.0, 56.2);
        let position = target(12.0, 56.0, 27.4, 1.0, 42.0, 10.0, 17.0);
        let correction = heliocentric_correction_seconds(date, position);
        assert!((hjd_tt(date, position) - date - correction / 86_400.0).abs() < 1e-8);
    }

    #[test]
    fn hjd_utc_takes_the_earth_at_the_tt_instant() {
        // 1 January 2017, TAI − UTC 37 s, so TT − UTC 69.184 s.
        let utc = julian_date(2017, 1, 1, 0.0, 0.0, 0.0);
        let tt_minus_utc = 69.184;
        let position = target(6.0, 0.0, 0.0, 1.0, 23.0, 26.0, 0.0);
        let tt = utc + tt_minus_utc / 86_400.0;
        let hjd = hjd_utc(utc, tt_minus_utc, position);
        assert!(
            (hjd - utc - heliocentric_correction_seconds(tt, position) / 86_400.0).abs() < 1e-8
        );
        // HJD_UTC and HJD_TT of one event differ by TT − UTC.
        assert!(((hjd_tt(tt, position) - hjd) * 86_400.0 - tt_minus_utc).abs() < 1e-4);
    }

    #[test]
    fn the_correction_is_at_most_the_light_time_across_the_orbit() {
        // Never more than 1.017 AU of light, 507.5 s, at aphelion.
        let limit = 1.017 * LIGHT_TIME_PER_AU_SECONDS;
        assert!((LIGHT_TIME_PER_AU_SECONDS - 499.004_783_8).abs() < 1e-6);
        for step in 0..=73 {
            let date = J2000_JULIAN_DATE + f64::from(step) * 5.0;
            for (alpha, delta) in [
                (0.0, 0.0),
                (90.0, 23.44),
                (180.0, 0.0),
                (270.0, -23.44),
                (0.0, 90.0),
            ] {
                let position = Target {
                    right_ascension_degrees: alpha,
                    declination_degrees: delta,
                };
                assert!(heliocentric_correction_seconds(date, position).abs() < limit);
            }
        }
        // Opposite directions have opposite corrections.
        let date = julian_date(2026, 9, 27, 0.0, 0.0, 0.0);
        let east = target(3.0, 0.0, 0.0, 1.0, 10.0, 0.0, 0.0);
        let west = target(15.0, 0.0, 0.0, -1.0, 10.0, 0.0, 0.0);
        let sum = heliocentric_correction_seconds(date, east)
            + heliocentric_correction_seconds(date, west);
        assert!(sum.abs() < 1e-9, "{sum}");
    }
}
