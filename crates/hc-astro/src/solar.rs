//! The Sun: apparent longitude, the search that solar terms are built on,
//! and the equinoxes and solstices.
//!
//! The position is the Earth's from VSOP87 ([`crate::vsop87`]) turned
//! around, brought to the FK5 frame, then given nutation in longitude and
//! annual aberration — Meeus, *Astronomical Algorithms*, 2nd ed., chapter
//! 25, in its higher-accuracy form. The truncation of the series costs
//! under a quarter of a second of arc, so the apparent longitude is good to
//! about 1″ and a seasonal event to well under a minute of time; what
//! remains is ΔT.
//!
//! The chapter's low-accuracy series is not used. It is good to 0.01°, a
//! quarter of an hour of the Sun's motion, and a calendar decided by a noon,
//! a sunset or a midnight is decided at the minute.

use hc_calendar::Rd;
use hc_calendar::fixed::Moment;
use hc_core::math::{RAD_TO_DEG, atan2, cos_deg, normalize_degrees, sin_deg};

use crate::earth::{Equatorial, equatorial_from_ecliptic, true_obliquity_at_centuries};
use crate::search::invert_angular;
use crate::time::{gregorian_new_year, julian_centuries};
use crate::util::{max_of, modulo, poly};

/// The mean tropical year in days, as used to seed the longitude searches.
///
/// This is the value in Reingold & Dershowitz, *Calendrical Calculations*,
/// 4th ed.; it is a mean, and the actual interval between two equinoxes
/// varies by a fraction of a day either side of it.
pub const MEAN_TROPICAL_YEAR: f64 = 365.242_189;

/// The constant of annual aberration at unit distance, in arcseconds
/// (Meeus 25.10). The Sun appears displaced backwards along its apparent
/// path by this much divided by the Earth–Sun distance in astronomical units.
const ABERRATION_ARCSECONDS: f64 = 20.4898;

/// The Sun's geometric mean longitude, referred to the mean equinox of the
/// date, in degrees. Meeus (25.2).
#[must_use]
pub fn solar_mean_longitude_at_centuries(centuries: f64) -> f64 {
    normalize_degrees(poly(centuries, &[280.466_46, 36_000.769_83, 0.000_303_2]))
}

/// The Sun's geometric ecliptic longitude and latitude in degrees, referred
/// to the mean equinox of the date in the FK5 frame, and its distance in
/// astronomical units: the Earth's heliocentric VSOP87 position turned
/// around. Meeus (25.9).
#[must_use]
pub fn geometric_solar_ecliptic_at_centuries(centuries: f64) -> (f64, f64, f64) {
    let (earth_longitude, earth_latitude, radius) =
        crate::vsop87::earth_heliocentric(centuries / 10.0);
    let longitude = normalize_degrees(earth_longitude * RAD_TO_DEG + 180.0);
    let latitude = -earth_latitude * RAD_TO_DEG;
    // From VSOP87's dynamical ecliptic to the FK5 frame: a constant shift in
    // longitude and a small periodic one in latitude.
    let lambda_prime = longitude - 1.397 * centuries - 0.000_31 * centuries * centuries;
    let fk5_longitude = normalize_degrees(longitude - 0.090_33 / 3_600.0);
    let fk5_latitude =
        latitude + 0.039_16 / 3_600.0 * (cos_deg(lambda_prime) - sin_deg(lambda_prime));
    (fk5_longitude, fk5_latitude, radius)
}

/// The Sun's true geometric longitude in degrees, referred to the mean
/// equinox of the date — no nutation, no aberration.
#[must_use]
pub fn geometric_solar_longitude_at_centuries(centuries: f64) -> f64 {
    geometric_solar_ecliptic_at_centuries(centuries).0
}

/// The Sun's geometric ecliptic latitude in degrees: never more than about
/// 1.2″ from zero, and the reason "the Sun is on the ecliptic" is only
/// nearly true.
#[must_use]
pub fn solar_latitude_at_centuries(centuries: f64) -> f64 {
    geometric_solar_ecliptic_at_centuries(centuries).1
}

/// The Earth–Sun distance in astronomical units.
#[must_use]
pub fn solar_radius_vector_at_centuries(centuries: f64) -> f64 {
    geometric_solar_ecliptic_at_centuries(centuries).2
}

/// The Earth–Sun distance in astronomical units at a Universal Time moment.
#[must_use]
pub fn solar_radius_vector(moment: Moment) -> f64 {
    solar_radius_vector_at_centuries(julian_centuries(moment))
}

/// The Sun's apparent geometric longitude in degrees, referred to the true
/// equinox of the date: true longitude plus nutation in longitude, minus
/// annual aberration.
///
/// Accurate to about 1″ over the era this crate claims; see the crate
/// README.
#[must_use]
pub fn solar_longitude_at_centuries(centuries: f64) -> f64 {
    let (geometric, _, radius) = geometric_solar_ecliptic_at_centuries(centuries);
    let nutation = crate::earth::nutation_at_centuries(centuries).longitude_degrees;
    let aberration = -ABERRATION_ARCSECONDS / 3600.0 / radius;
    normalize_degrees(geometric + nutation + aberration)
}

/// The Sun's apparent geometric longitude in degrees at a Universal Time
/// moment.
///
/// This is *the* function the 24 solar terms, the Chinese calendar's major
/// terms and the seasonal holidays are all built on.
///
/// ```
/// use hc_astro::solar::solar_longitude;
/// use hc_calendar::fixed::Moment;
///
/// // RD 730199.316 is 2000-03-20 about 07:35 UT, the March equinox.
/// let longitude = solar_longitude(Moment(730_199.316));
/// assert!(longitude < 0.01 || longitude > 359.99, "longitude was {longitude}");
/// ```
#[must_use]
pub fn solar_longitude(moment: Moment) -> f64 {
    solar_longitude_at_centuries(julian_centuries(moment))
}

/// The Sun's apparent right ascension and declination at a Universal Time
/// moment.
#[must_use]
pub fn solar_position(moment: Moment) -> Equatorial {
    let centuries = julian_centuries(moment);
    equatorial_from_ecliptic(
        solar_longitude_at_centuries(centuries),
        solar_latitude_at_centuries(centuries),
        true_obliquity_at_centuries(centuries),
    )
}

/// The equation of time as a fraction of a day: apparent solar time minus
/// mean solar time.
///
/// Positive means the sundial is ahead of the clock, as it is in early
/// November. Meeus (28.1).
#[must_use]
pub fn equation_of_time(moment: Moment) -> f64 {
    let centuries = julian_centuries(moment);
    let obliquity = true_obliquity_at_centuries(centuries);
    let apparent = solar_longitude_at_centuries(centuries);
    let right_ascension = normalize_degrees(
        atan2(cos_deg(obliquity) * sin_deg(apparent), cos_deg(apparent)) * RAD_TO_DEG,
    );
    let mean_longitude = solar_mean_longitude_at_centuries(centuries);
    let nutation = crate::earth::nutation_at_centuries(centuries).longitude_degrees;
    let degrees = mean_longitude - 0.005_718_3 - right_ascension + nutation * cos_deg(obliquity);
    // The difference is always small; folding it into (−180, 180] removes the
    // 360° that appears whenever the two longitudes straddle the equinox.
    crate::util::signed_degrees(degrees) / 360.0
}

/// The first moment at or after `moment` when the Sun's apparent longitude
/// is `target_degrees`.
///
/// The search seeds itself with the mean tropical rate, which is never more
/// than about two days out because the equation of the centre is bounded by
/// 1.92°, then brackets ±5 days around that seed and bisects. The answer is
/// good to a few milliseconds *of the model*; the model itself is good to
/// about 1″ of longitude, which is under half a minute of time.
///
/// If `moment` is itself the answer to within floating-point noise, the
/// search may step a whole year forward; callers who need "on or before"
/// semantics should search from slightly earlier.
#[must_use]
pub fn solar_longitude_after(target_degrees: f64, moment: Moment) -> Moment {
    let rate = MEAN_TROPICAL_YEAR / 360.0;
    let to_go = modulo(target_degrees - solar_longitude(moment), 360.0);
    let estimate = moment.0 + rate * to_go;
    invert_angular(
        solar_longitude,
        normalize_degrees(target_degrees),
        Moment(max_of(moment.0, estimate - 5.0)),
        Moment(estimate + 5.0),
    )
}

/// Which of the two equinoxes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Equinox {
    /// The Sun crosses 0° — the northward equinox, in March.
    March,
    /// The Sun crosses 180° — the southward equinox, in September.
    September,
}

impl Equinox {
    /// The solar longitude this equinox is defined by.
    #[must_use]
    pub const fn solar_longitude_degrees(self) -> f64 {
        match self {
            Self::March => 0.0,
            Self::September => 180.0,
        }
    }
}

/// Which of the two solstices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Solstice {
    /// The Sun reaches 90° — the northern summer solstice, in June.
    June,
    /// The Sun reaches 270° — the northern winter solstice, in December.
    December,
}

impl Solstice {
    /// The solar longitude this solstice is defined by.
    #[must_use]
    pub const fn solar_longitude_degrees(self) -> f64 {
        match self {
            Self::June => 90.0,
            Self::December => 270.0,
        }
    }
}

/// The moment of an equinox in a given proleptic Gregorian year, in
/// Universal Time.
///
/// Both equinoxes fall inside the Gregorian year they are named for over the
/// whole era this crate supports, so the search simply starts at that year's
/// 1 January.
#[must_use]
pub fn equinox(year: i64, which: Equinox) -> Moment {
    seasonal_event(year, which.solar_longitude_degrees())
}

/// The moment of a solstice in a given proleptic Gregorian year, in
/// Universal Time.
#[must_use]
pub fn solstice(year: i64, which: Solstice) -> Moment {
    seasonal_event(year, which.solar_longitude_degrees())
}

/// The first moment in a Gregorian year at which the Sun reaches a given
/// apparent longitude.
#[must_use]
pub fn seasonal_event(year: i64, target_degrees: f64) -> Moment {
    solar_longitude_after(target_degrees, Moment(gregorian_new_year(year).0 as f64))
}

/// The Rata Die day on which a seasonal event falls, in Universal Time.
#[must_use]
pub fn seasonal_event_day(year: i64, target_degrees: f64) -> Rd {
    seasonal_event(year, target_degrees).day()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::{
        centuries_from_dynamical_julian_date, dynamical_from_julian_centuries, dynamical_time,
        universal_from_dynamical_julian_date,
    };

    /// Meeus, examples 25.a and 25.b: 1992 October 13.0 TD, JDE 2448908.5.
    const EXAMPLE_25_JDE: f64 = 2_448_908.5;

    fn example_25_centuries() -> f64 {
        centuries_from_dynamical_julian_date(EXAMPLE_25_JDE)
    }

    #[test]
    fn meeus_example_25_has_the_stated_julian_centuries() {
        assert!(
            (example_25_centuries() + 0.072_183_436).abs() < 1e-9,
            "centuries {}",
            example_25_centuries()
        );
    }

    #[test]
    fn the_solar_mean_longitude_matches_meeus_example_25a() {
        let value = solar_mean_longitude_at_centuries(example_25_centuries());
        assert!((value - 201.807_20).abs() < 1e-4, "L0 was {value}");
    }

    /// Meeus, example 25.b, from his truncation of VSOP87: L = 19.907372°,
    /// so Θ = 199.907372°, then −0.09033″ to FK5; apparent longitude
    /// λ = 199°54′21.56″; R = 0.99760853 AU.
    #[test]
    fn the_geometric_solar_position_matches_meeus_example_25b() {
        let (longitude, latitude, radius) =
            geometric_solar_ecliptic_at_centuries(example_25_centuries());
        // His tables and ours keep different small terms; the spread is a
        // fraction of a second of arc.
        assert!(
            (longitude - 199.907_347).abs() < 0.000_15,
            "geometric longitude {longitude}"
        );
        assert!(
            (0.0..0.000_35).contains(&latitude),
            "geometric latitude {latitude}"
        );
        assert!(
            (radius - 0.997_608_53).abs() < 0.000_000_5,
            "R was {radius}"
        );
    }

    #[test]
    fn the_apparent_solar_longitude_matches_meeus_example_25b() {
        let value = solar_longitude_at_centuries(example_25_centuries());
        let expected = 199.0 + 54.0 / 60.0 + 21.56 / 3_600.0;
        assert!(
            (value - expected).abs() < 0.000_15,
            "apparent longitude {value}, expected {expected}"
        );
    }

    /// Meeus, example 25.b: apparent α = 13h13m30.749s, δ = −7°47′01.74″.
    #[test]
    fn the_solar_position_matches_meeus_example_25b() {
        let moment = universal_from_dynamical_julian_date(EXAMPLE_25_JDE);
        let position = solar_position(moment);
        let alpha = (13.0 + 13.0 / 60.0 + 30.749 / 3_600.0) * 15.0;
        let delta = -(7.0 + 47.0 / 60.0 + 1.74 / 3_600.0);
        assert!(
            (position.right_ascension_degrees - alpha).abs() < 0.000_3,
            "α = {}",
            position.right_ascension_degrees
        );
        assert!(
            (position.declination_degrees - delta).abs() < 0.000_3,
            "δ = {}",
            position.declination_degrees
        );
    }

    #[test]
    fn the_earth_is_closest_to_the_sun_in_january() {
        // Perihelion is in the first week of January, aphelion in early July.
        let january = solar_radius_vector(Moment(gregorian_new_year(2024).0 as f64 + 3.0));
        let july = solar_radius_vector(Moment(gregorian_new_year(2024).0 as f64 + 185.0));
        assert!(january < 0.9840, "january distance {january}");
        assert!(july > 1.0160, "july distance {july}");
    }

    #[test]
    fn solar_longitude_stays_inside_a_full_turn() {
        for step in 0..2_000 {
            let moment = Moment(700_000.0 + f64::from(step) * 7.3);
            let longitude = solar_longitude(moment);
            assert!(
                (0.0..360.0).contains(&longitude),
                "longitude {longitude} at {}",
                moment.0
            );
        }
    }

    #[test]
    fn solar_longitude_increases_along_the_year() {
        // Sampled every five days, the apparent longitude must rise by
        // between 4.5 and 5.5 degrees: the Sun never goes backwards along the
        // ecliptic, and its rate varies only with the orbital eccentricity.
        let start = gregorian_new_year(2024).0 as f64;
        let mut previous = solar_longitude(Moment(start));
        for step in 1..73 {
            let current = solar_longitude(Moment(start + f64::from(step) * 5.0));
            let advance = modulo(current - previous, 360.0);
            assert!(
                (4.5..5.5).contains(&advance),
                "advance of {advance} degrees at step {step}"
            );
            previous = current;
        }
    }

    #[test]
    fn solar_longitude_after_lands_on_its_target() {
        for target_step in 0..24 {
            let target = f64::from(target_step) * 15.0;
            for year in [-500i64, 1, 1582, 1900, 2000, 2024, 2400] {
                let start = Moment(gregorian_new_year(year).0 as f64);
                let found = solar_longitude_after(target, start);
                let error = crate::util::signed_degrees(solar_longitude(found) - target);
                assert!(
                    error.abs() < 1e-5,
                    "target {target} in {year}: off by {error} degrees"
                );
                assert!(found.0 >= start.0, "search went backwards");
            }
        }
    }

    #[test]
    fn solar_longitude_after_never_returns_a_moment_in_the_past() {
        let start = Moment(738_000.0);
        for step in 0..360 {
            let found = solar_longitude_after(f64::from(step), start);
            assert!(found.0 >= start.0);
            assert!(found.0 <= start.0 + MEAN_TROPICAL_YEAR + 1.0);
        }
    }

    #[test]
    fn consecutive_solar_terms_are_fourteen_to_sixteen_days_apart() {
        // The 24 solar terms are the 15-degree multiples of solar longitude.
        let mut longitude = 285.0;
        let mut moment =
            solar_longitude_after(longitude, Moment(gregorian_new_year(2024).0 as f64));
        for _ in 0..24 {
            longitude = modulo(longitude + 15.0, 360.0);
            let next = solar_longitude_after(longitude, moment);
            let gap = next.0 - moment.0;
            assert!(
                (14.0..16.5).contains(&gap),
                "gap of {gap} days before longitude {longitude}"
            );
            moment = next;
        }
    }

    #[test]
    fn the_twenty_four_solar_terms_of_a_year_span_one_tropical_year() {
        let start = solar_longitude_after(315.0, Moment(gregorian_new_year(2024).0 as f64));
        let mut moment = start;
        for step in 1..=24 {
            moment = solar_longitude_after(modulo(315.0 + f64::from(step) * 15.0, 360.0), moment);
        }
        let span = moment.0 - start.0;
        assert!((span - MEAN_TROPICAL_YEAR).abs() < 0.5, "span was {span}");
    }

    /// Published equinoxes and solstices, as (Rata Die, hour, minute, the
    /// solar longitude that defines the event), all in Universal Time. The
    /// times are the ones the USNO "Earth's Seasons" table and the IMCCE
    /// publish; the Rata Die values are checked against the calendar date in
    /// the test below.
    const PUBLISHED_SEASONAL_EVENTS: [(i64, f64, f64, f64); 9] = [
        (737_504, 3.0, 50.0, 0.0),    // 2020-03-20
        (730_199, 7.0, 35.0, 0.0),    // 2000-03-20
        (730_385, 17.0, 28.0, 180.0), // 2000-09-22
        (730_475, 13.0, 37.0, 270.0), // 2000-12-21
        (738_876, 3.0, 27.0, 270.0),  // 2023-12-22
        (738_965, 3.0, 6.0, 0.0),     // 2024-03-20
        (739_057, 20.0, 51.0, 90.0),  // 2024-06-20
        (739_151, 12.0, 44.0, 180.0), // 2024-09-22
        (739_241, 9.0, 21.0, 270.0),  // 2024-12-21
    ];

    /// The tolerance the series earns, in minutes of time.
    ///
    /// The published tables give the minute, so half a minute of the
    /// spread is theirs; the rest is the truncation, a few seconds, and ΔT.
    const SEASONAL_TOLERANCE_MINUTES: f64 = 1.0;

    /// The March equinox of 2000 was 2000-03-20 07:35 UT.
    #[test]
    fn the_march_equinox_of_2000_falls_where_the_almanacs_put_it() {
        let found = equinox(2000, Equinox::March);
        assert_eq!(found.day(), Rd(730_199), "the equinox is on 2000-03-20");
        let expected = 730_199.0 + (7.0 + 35.0 / 60.0) / 24.0;
        let error_minutes = (found.0 - expected) * 24.0 * 60.0;
        assert!(
            error_minutes.abs() < SEASONAL_TOLERANCE_MINUTES,
            "off by {error_minutes} minutes (got {})",
            found.0
        );
    }

    /// The December solstice of 2023 was 2023-12-22 03:27 UT.
    #[test]
    fn the_december_solstice_of_2023_falls_where_the_almanacs_put_it() {
        let found = solstice(2023, Solstice::December);
        assert_eq!(found.day(), Rd(738_876), "the solstice is on 2023-12-22");
        let expected = 738_876.0 + (3.0 + 27.0 / 60.0) / 24.0;
        let error_minutes = (found.0 - expected) * 24.0 * 60.0;
        assert!(
            error_minutes.abs() < SEASONAL_TOLERANCE_MINUTES,
            "off by {error_minutes} minutes (got {})",
            found.0
        );
    }

    #[test]
    fn every_published_seasonal_event_falls_inside_the_claimed_accuracy() {
        let mut total_error = 0.0;
        let mut worst: f64 = 0.0;
        for (day, hour, minute, longitude) in PUBLISHED_SEASONAL_EVENTS {
            let year = crate::time::gregorian_year_from_rd(Rd(day));
            let found = seasonal_event(year, longitude);
            assert_eq!(
                found.day(),
                Rd(day),
                "the event at longitude {longitude} in {year} landed on the wrong day"
            );
            let expected = day as f64 + (hour + minute / 60.0) / 24.0;
            let error_minutes = (found.0 - expected) * 24.0 * 60.0;
            assert!(
                error_minutes.abs() < SEASONAL_TOLERANCE_MINUTES,
                "RD {day}: off by {error_minutes} minutes"
            );
            total_error += error_minutes;
            if error_minutes.abs() > worst {
                worst = error_minutes.abs();
            }
        }
        // No bias is left to speak of: the mean residual is inside the
        // rounding of the tables themselves.
        let mean_error = total_error / PUBLISHED_SEASONAL_EVENTS.len() as f64;
        assert!(mean_error.abs() < 0.5, "a bias of {mean_error} minutes");
        assert!(worst < SEASONAL_TOLERANCE_MINUTES, "worst case {worst}");
    }

    #[test]
    fn the_four_seasonal_events_of_a_year_fall_in_the_right_months() {
        let year = 2024;
        let start = gregorian_new_year(year).0;
        let march = equinox(year, Equinox::March).0 - start as f64;
        let june = solstice(year, Solstice::June).0 - start as f64;
        let september = equinox(year, Equinox::September).0 - start as f64;
        let december = solstice(year, Solstice::December).0 - start as f64;
        // Day-of-year ranges for the four events, zero-based.
        assert!((78.0..81.0).contains(&march), "march at day {march}");
        assert!((170.0..173.0).contains(&june), "june at day {june}");
        assert!(
            (264.0..267.0).contains(&september),
            "september at day {september}"
        );
        assert!(
            (354.0..357.0).contains(&december),
            "december at day {december}"
        );
        assert!(march < june && june < september && september < december);
    }

    #[test]
    fn seasonal_events_recur_once_a_tropical_year_for_five_centuries() {
        let mut previous = equinox(1600, Equinox::March).0;
        for year in 1601..2100 {
            let current = equinox(year, Equinox::March).0;
            let gap = current - previous;
            assert!(
                (365.0..366.0).contains(&gap),
                "equinox gap of {gap} days into {year}"
            );
            previous = current;
        }
    }

    #[test]
    fn the_seasons_are_not_of_equal_length() {
        // Northern spring is the longest season and winter the shortest,
        // because the Earth is near perihelion in January and moving fastest.
        let spring = solstice(2024, Solstice::June).0 - equinox(2024, Equinox::March).0;
        let summer = equinox(2024, Equinox::September).0 - solstice(2024, Solstice::June).0;
        let autumn = solstice(2024, Solstice::December).0 - equinox(2024, Equinox::September).0;
        let winter = equinox(2025, Equinox::March).0 - solstice(2024, Solstice::December).0;
        assert!(spring > 92.0 && spring < 93.5, "spring {spring}");
        assert!(summer > 93.0 && summer < 94.5, "summer {summer}");
        assert!(autumn > 89.0 && autumn < 90.5, "autumn {autumn}");
        assert!(winter > 88.5 && winter < 90.0, "winter {winter}");
        assert!(winter < autumn && autumn < spring && spring < summer);
    }

    #[test]
    fn the_equation_of_time_reaches_its_known_annual_extremes() {
        let start = gregorian_new_year(2024).0;
        let mut greatest: f64 = -1.0;
        let mut least: f64 = 1.0;
        for day in 0..366 {
            let value = equation_of_time(Moment(start as f64 + f64::from(day)));
            if value > greatest {
                greatest = value;
            }
            if value < least {
                least = value;
            }
        }
        let greatest_minutes = greatest * 24.0 * 60.0;
        let least_minutes = least * 24.0 * 60.0;
        // The sundial runs about 16.4 minutes fast in early November and
        // about 14.2 minutes slow in mid February.
        assert!(
            (greatest_minutes - 16.4).abs() < 0.4,
            "maximum was {greatest_minutes} minutes"
        );
        assert!(
            (least_minutes + 14.2).abs() < 0.4,
            "minimum was {least_minutes} minutes"
        );
    }

    #[test]
    fn the_equation_of_time_crosses_zero_four_times_a_year() {
        let start = gregorian_new_year(2024).0;
        let mut crossings = 0;
        let mut previous = equation_of_time(Moment(start as f64));
        for day in 1..366 {
            let current = equation_of_time(Moment(start as f64 + f64::from(day)));
            if (previous < 0.0) != (current < 0.0) {
                crossings += 1;
            }
            previous = current;
        }
        assert_eq!(crossings, 4, "expected four zero crossings a year");
    }

    #[test]
    fn dynamical_and_universal_bridges_agree_with_each_other() {
        let tt = Moment::from_julian_date(EXAMPLE_25_JDE);
        let ut = universal_from_dynamical_julian_date(EXAMPLE_25_JDE);
        assert!((dynamical_time(ut).0 - tt.0).abs() < 1e-8);
        assert!((dynamical_from_julian_centuries(example_25_centuries()).0 - tt.0).abs() < 1e-8);
    }

    #[test]
    fn the_seasonal_event_day_is_the_day_containing_the_event() {
        let day = seasonal_event_day(2024, 0.0);
        assert_eq!(day, equinox(2024, Equinox::March).day());
        assert_eq!(day, Rd(gregorian_new_year(2024).0 + 79));
    }

    #[test]
    fn the_equinox_and_solstice_enums_name_the_right_longitudes() {
        assert!((Equinox::March.solar_longitude_degrees() - 0.0).abs() < 1e-12);
        assert!((Equinox::September.solar_longitude_degrees() - 180.0).abs() < 1e-12);
        assert!((Solstice::June.solar_longitude_degrees() - 90.0).abs() < 1e-12);
        assert!((Solstice::December.solar_longitude_degrees() - 270.0).abs() < 1e-12);
    }
}
