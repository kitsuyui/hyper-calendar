//! Earth's Moon: lunation numbers, the age of the Moon, and the
//! selenographic colongitude that lunar observers use.
//!
//! # On Coordinated Lunar Time
//!
//! **There is no Coordinated Lunar Time in this crate, because there is not
//! yet one to implement.**
//!
//! A clock on the Moon does not tick at the same rate as one on Earth: the
//! weaker gravitational potential and the relative motion make a lunar clock
//! gain roughly 56 microseconds a day, and that is before anyone decides where
//! the origin is or which point on the Moon the scale is referred to. The
//! United States Office of Science and Technology Policy directed NASA in
//! April 2024 to deliver a standard, Coordinated Lunar Time (LTC), by the end
//! of 2026, and the IAU and the CCTF have work in progress. At the time this
//! module was written no definition had been published.
//!
//! So this module offers the things that *are* defined — lunation numbers, the
//! Moon's age, the selenographic position of the Sun — and refuses to invent a
//! scale that standards bodies are still arguing about. For the rate part of
//! the problem, the relativistic rate difference between a lunar and a
//! terrestrial clock, see `hc-relativity`. For a *mean solar* time on the
//! Moon — which is a different question, and answerable —
//! [`mean_solar_time`] gives one, under a zero point this crate declares and
//! labels as such.
//!
//! # Accuracy
//!
//! The Moon's position comes from `hc-astro`, whose truncated ELP series is
//! good to about 10″ in longitude and places a new moon within about a minute.
//! The selenographic quantities here are Meeus's chapter 53 applied to that
//! position, so the colongitude is good to a few hundredths of a degree of the
//! model — comfortably better than the tenth of a degree that lunar
//! observation tables are quoted to.

use hc_astro::MEAN_SYNODIC_MONTH;
use hc_astro::Moment;
use hc_core::math::{RAD_TO_DEG, asin, atan2, cos_deg, normalize_degrees, round, sin_deg};
use hc_core::{Instant, Tai};

use crate::bodies::by_name;
use crate::clock::{BodyClock, LocalTime};
use crate::util::{j2000_offset_days, signed_degrees};

/// A one-sentence statement of where Coordinated Lunar Time stands, for
/// callers that want to display something rather than silently omit it.
pub const COORDINATED_LUNAR_TIME_STATUS: &str = "Coordinated Lunar Time (LTC) is being standardised and is not yet defined; \
     this crate deliberately does not provide one.";

/// The inclination of the Moon's equator to the ecliptic, in degrees. Meeus,
/// chapter 53.
pub const LUNAR_EQUATOR_INCLINATION: f64 = 1.542_42;

/// The Brown lunation number of Meeus lunation 0.
///
/// Ernest W. Brown's count begins with lunation 1 at the new moon of **1923
/// January 17**; Meeus's begins with lunation 0 at the first new moon of 2000,
/// on **2000 January 6**. The two are a fixed 953 lunations apart.
pub const BROWN_MINUS_MEEUS_LUNATION: i64 = 953;

/// The astronomical algorithms in `hc-astro` take Universal Time; this is the
/// bridge from a TAI reading.
///
/// The TT-to-UT step is `hc-astro`'s ΔT model, so a moment produced here
/// carries that model's uncertainty — sub-second for the modern era, growing
/// to hours in antiquity.
#[must_use]
pub fn universal_time_moment(instant: Instant<Tai>) -> Moment {
    let terrestrial = Moment(hc_astro::time::J2000.0 + j2000_offset_days(instant));
    hc_astro::universal_time(terrestrial)
}

/// The **Meeus lunation number** containing an instant: lunation 0 begins at
/// the new moon of 2000 January 6.
///
/// The number is the one in Meeus's *Astronomical Algorithms* chapter 49 and
/// the one `hc_astro::nth_new_moon` is indexed by.
#[must_use]
pub fn lunation_meeus(instant: Instant<Tai>) -> i64 {
    let moment = universal_time_moment(instant);
    let mut lunation = round((moment.0 - hc_astro::nth_new_moon(0).0) / MEAN_SYNODIC_MONTH) as i64;
    // The mean-rate seed is never more than one lunation out, because a true
    // new moon departs from the mean one by at most about fourteen hours.
    for _ in 0..3 {
        if hc_astro::nth_new_moon(lunation).0 > moment.0 {
            lunation -= 1;
        } else {
            break;
        }
    }
    for _ in 0..3 {
        if hc_astro::nth_new_moon(lunation + 1).0 <= moment.0 {
            lunation += 1;
        } else {
            break;
        }
    }
    lunation
}

/// The **Brown lunation number** containing an instant: lunation 1 begins at
/// the new moon of 1923 January 17.
#[must_use]
pub fn lunation_brown(instant: Instant<Tai>) -> i64 {
    lunation_meeus(instant) + BROWN_MINUS_MEEUS_LUNATION
}

/// The **age of the Moon**: days elapsed since the new moon that began the
/// current lunation.
///
/// Runs from 0 at new moon to about 29.5 just before the next one. This is the
/// quantity almanacs print; it is not the same as the phase angle, because the
/// Moon's angular speed varies by about 12% between perigee and apogee.
#[must_use]
pub fn age_days(instant: Instant<Tai>) -> f64 {
    let moment = universal_time_moment(instant);
    moment.0 - hc_astro::nth_new_moon(lunation_meeus(instant)).0
}

/// The fraction of the Moon's disc that is lit, from 0 at new to 1 at full.
///
/// A thin re-export of `hc-astro` so that a caller asking this module about
/// the Moon does not have to reach past it.
#[must_use]
pub fn illuminated_fraction(instant: Instant<Tai>) -> f64 {
    hc_astro::lunar_illuminated_fraction(universal_time_moment(instant))
}

/// The selenographic longitude and latitude of the sub-solar point, in
/// degrees.
///
/// Meeus, *Astronomical Algorithms*, chapter 53: the optical-libration
/// formulae applied not to the Earth's direction but to the Sun's, which is
/// what puts the Sun somewhere in the Moon's own coordinate system.
#[must_use]
pub fn subsolar_selenographic_position(instant: Instant<Tai>) -> (f64, f64) {
    let moment = universal_time_moment(instant);
    let centuries = hc_astro::julian_centuries(moment);

    let lunar_longitude = hc_astro::lunar_longitude(moment);
    let lunar_latitude = hc_astro::lunar::lunar_latitude(moment);
    let lunar_distance_km = hc_astro::lunar::lunar_distance(moment);
    let solar_longitude = hc_astro::solar_longitude(moment);
    let solar_distance_km = hc_astro::solar::solar_radius_vector(moment) * ASTRONOMICAL_UNIT_KM;
    let nutation_longitude = hc_astro::nutation(moment).longitude_degrees;

    // Meeus (47.7) and (47.1): the argument of latitude and the longitude of
    // the ascending node of the Moon's mean orbit.
    let argument_of_latitude = polynomial(
        centuries,
        &[
            93.272_095_0,
            483_202.017_523_3,
            -0.003_653_9,
            -1.0 / 3_526_000.0,
            1.0 / 863_310_000.0,
        ],
    );
    let node = polynomial(
        centuries,
        &[
            125.044_547_9,
            -1_934.136_289_1,
            0.002_075_4,
            1.0 / 467_441.0,
            -1.0 / 60_616_000.0,
        ],
    );

    // The Sun's geocentric direction, corrected to a selenocentric one: the
    // parallax of the Earth-Moon separation as seen from the Sun.
    let ratio = lunar_distance_km / solar_distance_km;
    let heliocentric_longitude = solar_longitude
        + 180.0
        + ratio * 57.296 * cos_deg(lunar_latitude) * sin_deg(solar_longitude - lunar_longitude);
    let heliocentric_latitude = ratio * lunar_latitude;

    let w = normalize_degrees(heliocentric_longitude - nutation_longitude - node);
    let sin_w = sin_deg(w);
    let cos_w = cos_deg(w);
    let cos_beta = cos_deg(heliocentric_latitude);
    let sin_beta = sin_deg(heliocentric_latitude);
    let sin_i = sin_deg(LUNAR_EQUATOR_INCLINATION);
    let cos_i = cos_deg(LUNAR_EQUATOR_INCLINATION);

    let a = atan2(
        sin_w * cos_beta * cos_i - sin_beta * sin_i,
        cos_w * cos_beta,
    ) * RAD_TO_DEG;
    let longitude = signed_degrees(a - argument_of_latitude);
    let latitude = asin(-sin_w * cos_beta * sin_i - sin_beta * cos_i) * RAD_TO_DEG;
    (longitude, latitude)
}

/// The **selenographic colongitude of the Sun**, in degrees.
///
/// This is the number in every lunar observing guide, because it says where
/// the terminator is: `90° − l₀`, the selenographic longitude of the morning
/// terminator measured the useful way round. It runs
///
/// | phase | colongitude |
/// |---|---|
/// | new moon | 270° |
/// | first quarter | 0° |
/// | full moon | 90° |
/// | last quarter | 180° |
///
/// and advances by about 12.2° per day, which is why an observer plans a
/// session by colongitude rather than by date: the same colongitude always
/// shows the same craters at the same sun angle.
#[must_use]
pub fn selenographic_colongitude(instant: Instant<Tai>) -> f64 {
    normalize_degrees(90.0 - subsolar_selenographic_position(instant).0)
}

/// The selenographic latitude of the sub-solar point, in degrees.
///
/// It stays within about ±1.6°, because the Moon's equator is nearly in the
/// ecliptic; that is why the lunar poles have craters that never see the Sun.
#[must_use]
pub fn subsolar_latitude(instant: Instant<Tai>) -> f64 {
    subsolar_selenographic_position(instant).1
}

/// Local mean solar time on the Moon at a west longitude.
///
/// **This is not Coordinated Lunar Time**; see the module documentation. It is
/// a mean solar clock whose day is the mean synodic month and whose midnight
/// at the prime meridian is mean new moon — which is the physically right
/// choice, since the near side faces away from the Sun at new moon — with its
/// day numbered by the Meeus lunation.
#[must_use]
pub fn mean_solar_time(instant: Instant<Tai>, west_longitude_degrees: f64) -> LocalTime {
    lunar_clock().at_west_longitude(instant, west_longitude_degrees)
}

/// The Moon's entry in the generic clock interface.
///
/// # Panics
///
/// Never: the Moon is always in [`crate::bodies::ALL`] and always has a solar
/// day. The `unwrap_or_else` exists only because the table is data and a
/// future edit could in principle remove the row.
fn lunar_clock() -> BodyClock {
    match by_name("Moon").and_then(BodyClock::new) {
        Some(clock) => clock,
        None => unreachable!("the Moon is always in the body table"),
    }
}

/// The astronomical unit in kilometres, IAU 2012 definition.
const ASTRONOMICAL_UNIT_KM: f64 = 149_597_870.7;

/// Horner evaluation of a polynomial in `x`.
fn polynomial(x: f64, coefficients: &[f64]) -> f64 {
    let mut total = 0.0;
    for coefficient in coefficients.iter().rev() {
        total = total * x + coefficient;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::tai_from_utc_fields;

    fn at(year: i64, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Instant<Tai> {
        tai_from_utc_fields(year, month, day, hour, minute, second).unwrap()
    }

    #[test]
    fn meeus_lunation_zero_is_the_new_moon_of_january_2000() {
        // Meeus (49.1): the new moon of 2000 January 6, at 18:14 UT.
        assert_eq!(lunation_meeus(at(2000, 1, 6, 20, 0, 0)), 0);
        assert_eq!(lunation_meeus(at(2000, 1, 6, 12, 0, 0)), -1);
        assert_eq!(lunation_meeus(at(2000, 2, 6, 0, 0, 0)), 1);
    }

    #[test]
    fn the_brown_lunation_count_begins_in_january_1923() {
        // Brown lunation 1 begins at the new moon of 1923 January 17.
        assert_eq!(lunation_brown(at(1923, 1, 18, 0, 0, 0)), 1);
        assert_eq!(lunation_brown(at(1923, 1, 16, 0, 0, 0)), 0);
        // And the two counts differ by exactly the documented constant.
        for when in [
            at(1900, 5, 5, 0, 0, 0),
            at(2000, 1, 6, 20, 0, 0),
            at(2024, 12, 1, 0, 0, 0),
        ] {
            assert_eq!(
                lunation_brown(when) - lunation_meeus(when),
                BROWN_MINUS_MEEUS_LUNATION
            );
        }
    }

    #[test]
    fn the_lunation_number_advances_by_one_per_lunation() {
        let mut previous = lunation_meeus(at(2020, 1, 1, 0, 0, 0));
        for month in 2..=12u8 {
            let current = lunation_meeus(at(2020, month, 1, 0, 0, 0));
            assert!(
                current == previous || current == previous + 1,
                "month {month}: {previous} -> {current}"
            );
            previous = current;
        }
        // Thirteen months is twelve or thirteen lunations.
        let span =
            lunation_meeus(at(2021, 1, 1, 0, 0, 0)) - lunation_meeus(at(2020, 1, 1, 0, 0, 0));
        assert_eq!(span, 12);
    }

    #[test]
    fn the_age_of_the_moon_stays_inside_one_lunation() {
        for day in 1..=28u8 {
            let age = age_days(at(2023, 6, day, 12, 0, 0));
            assert!(
                (0.0..MEAN_SYNODIC_MONTH + 1.0).contains(&age),
                "{day}: {age}"
            );
        }
    }

    #[test]
    fn the_moon_is_new_when_it_is_nought_days_old() {
        // The published new moon of 2024 January 11 is 11:57 UT. hc-astro's
        // truncated series places its conjunction about twenty minutes later,
        // so the age at the published instant is a hair short of a whole
        // lunation rather than a hair past zero; either side of the boundary
        // is a new moon, and the disc is dark at both.
        let when = at(2024, 1, 11, 11, 57, 0);
        let age = age_days(when);
        assert!(!(0.03..=MEAN_SYNODIC_MONTH - 0.03).contains(&age), "{age}");
        assert!(illuminated_fraction(when) < 0.005);
        // Half an hour later the count has certainly rolled over.
        let after = age_days(at(2024, 1, 11, 13, 0, 0));
        assert!(after < 0.05, "{after}");
    }

    #[test]
    fn the_moon_is_full_about_halfway_through_its_age() {
        // The full moon of 2024 January 25, 17:54 UT.
        let when = at(2024, 1, 25, 17, 54, 0);
        let age = age_days(when);
        assert!((age - 14.25).abs() < 0.5, "{age}");
        assert!(illuminated_fraction(when) > 0.99);
    }

    /// Meeus, *Astronomical Algorithms*, example 53.a: for 1992 April 12.0 TD
    /// the selenographic longitude of the Sun is 67.9°, its latitude +1.46°,
    /// and the colongitude 22.1°.
    #[test]
    fn the_meeus_worked_example_for_the_suns_selenographic_position_reproduces() {
        // 1992 April 12.0 TD is 1992-04-11T23:59:01 UTC, because TT - UTC was
        // 58.184 s in 1992.
        let when = at(1992, 4, 11, 23, 59, 1);
        let (longitude, latitude) = subsolar_selenographic_position(when);
        assert!((longitude - 67.9).abs() < 0.1, "longitude {longitude}");
        assert!((latitude - 1.46).abs() < 0.05, "latitude {latitude}");
        let colongitude = selenographic_colongitude(when);
        assert!(
            (colongitude - 22.1).abs() < 0.1,
            "colongitude {colongitude}"
        );
    }

    #[test]
    fn the_colongitude_is_two_hundred_and_seventy_at_new_moon() {
        // The four quarter values are the ones every observing guide prints.
        let checks: [(Instant<Tai>, f64); 4] = [
            (at(2024, 1, 11, 11, 57, 0), 270.0),
            (at(2024, 1, 18, 3, 52, 0), 0.0),
            (at(2024, 1, 25, 17, 54, 0), 90.0),
            (at(2024, 2, 2, 23, 18, 0), 180.0),
        ];
        for (when, expected) in checks {
            let colongitude = selenographic_colongitude(when);
            let error = signed_degrees(colongitude - expected);
            // The rule of thumb is exact only for a mean Sun on a circular
            // orbit; the real colongitude wanders several degrees either side
            // because of libration in longitude. The Meeus worked example
            // above is the test that pins the absolute value.
            assert!(error.abs() < 8.0, "{expected}: got {colongitude}");
        }
    }

    #[test]
    fn the_colongitude_advances_about_twelve_degrees_a_day() {
        let first = selenographic_colongitude(at(2025, 3, 1, 0, 0, 0));
        let second = selenographic_colongitude(at(2025, 3, 2, 0, 0, 0));
        let step = signed_degrees(second - first);
        assert!((step - 12.2).abs() < 0.5, "{step}");
        // And a full turn in one synodic month.
        let later = selenographic_colongitude(at(2025, 3, 30, 10, 21, 0));
        assert!(signed_degrees(later - first).abs() < 5.0);
    }

    #[test]
    fn the_subsolar_latitude_stays_within_the_lunar_obliquity() {
        let mut extreme: f64 = 0.0;
        for day in 1..=28u8 {
            let latitude = subsolar_latitude(at(2022, 9, day, 6, 0, 0));
            extreme = extreme.max(latitude.abs());
        }
        assert!(extreme < 1.7, "{extreme}");
        assert!(extreme > 0.5, "{extreme}");
    }

    #[test]
    fn the_lunar_mean_solar_day_is_the_synodic_month() {
        let clock = lunar_clock();
        let day = clock.solar_day_seconds() / 86_400.0;
        assert!((day - MEAN_SYNODIC_MONTH).abs() < 1e-6, "{day}");
        // A lunar "hour" is therefore about 29.5 Earth hours.
        let hour = clock.solar_day_seconds() / 24.0 / 3_600.0;
        assert!((hour - 29.53).abs() < 0.01, "{hour}");
    }

    #[test]
    fn the_lunar_clock_numbers_its_days_by_the_meeus_lunation() {
        // The clock is mean-rate and the lunation is the true new moon, so
        // they agree except close to a new moon; check away from one.
        for (year, month, day) in [(2000, 1, 20), (2015, 7, 5), (2030, 11, 11)] {
            let when = at(year, month, day, 0, 0, 0);
            let difference = mean_solar_time(when, 0.0).day_number() - lunation_meeus(when);
            assert!(difference.abs() <= 1, "{year}-{month}-{day}: {difference}");
        }
    }

    #[test]
    fn lunar_midnight_at_the_prime_meridian_is_new_moon() {
        // The physical claim behind the chosen zero point: the near side is
        // unlit at new moon, so that is midnight at the centre of the near
        // side.
        let when = at(2024, 1, 11, 11, 57, 0);
        let fraction = mean_solar_time(when, 0.0).day_fraction();
        assert!(!(0.03..=0.97).contains(&fraction), "{fraction}");
        // And noon at full moon.
        let full = mean_solar_time(at(2024, 1, 25, 17, 54, 0), 0.0).day_fraction();
        assert!((full - 0.5).abs() < 0.03, "{full}");
    }

    #[test]
    fn the_crate_says_plainly_that_coordinated_lunar_time_does_not_exist_yet() {
        assert!(COORDINATED_LUNAR_TIME_STATUS.contains("not yet defined"));
    }

    #[test]
    fn the_universal_time_bridge_lands_where_delta_t_says_it_should() {
        // At the J2000 epoch, TT ran 63.83 s ahead of UT1 by the Espenak
        // and Meeus fit, so the UT moment is that much before the TT one.
        let epoch = crate::util::instant_from_j2000_offset(0.0).unwrap();
        let moment = universal_time_moment(epoch);
        let gap = (hc_astro::time::J2000.0 - moment.0) * 86_400.0;
        assert!((gap - 63.83).abs() < 1.0, "{gap}");
    }
}
