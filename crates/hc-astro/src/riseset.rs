//! Sunrise, sunset, twilight, moonrise and moonset for a place on the Earth.
//!
//! Every function here returns an [`Option`], and the `None` is the point.
//! Above the Arctic and below the Antarctic circles there are days on which
//! the Sun does not rise and days on which it does not set, and there are
//! places and dates where astronomical twilight never ends all summer. A
//! library that invents a time for those days is lying to whoever asked, so
//! this one says `None` and lets the caller decide what to print.
//!
//! The geometry is Meeus, *Astronomical Algorithms*, 2nd ed., chapter 15,
//! but solved by bisection rather than by Meeus's interpolation: the event is
//! the moment the body's centre crosses a chosen altitude, and the altitude
//! is a continuous function of time that a bracket-and-halve search handles
//! without any of the special cases Meeus's method needs near the poles.
//!
//! # What "sunrise" means here
//!
//! The Sun's *upper limb* touching the *visible* horizon: the centre is
//! 50′ below the geometric horizon, being 16′ of semidiameter plus 34′ of
//! standard atmospheric refraction, plus the dip of the horizon if the
//! observer is above sea level. Those 34′ are a convention, not a
//! measurement — real refraction depends on the air, and a cold morning can
//! move an observed sunrise by a minute or more. That is the dominant error
//! in these results, not the astronomy.

use hc_calendar::Rd;
use hc_calendar::fixed::Moment;
use hc_core::math::{RAD_TO_DEG, acos};

use crate::earth::{altitude_degrees, apparent_sidereal_time, local_hour_angle};
use crate::lunar::{lunar_parallax, lunar_position};
use crate::search::{bisect_falling, bisect_rising};
use crate::solar::solar_position;
use crate::util::{clamp, signed_degrees};

/// A place on the Earth.
///
/// Longitude is **east-positive**, as ISO 6709 and every modern geodetic
/// convention have it. Meeus writes longitude west-positive; if you are
/// transcribing from the book, negate it.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Location {
    /// Geodetic latitude in degrees, positive north.
    pub latitude_degrees: f64,
    /// Longitude in degrees, positive east of Greenwich.
    pub longitude_degrees: f64,
    /// Height above sea level in metres. Only its effect on the dip of the
    /// horizon is modelled; terrain is not.
    pub elevation_metres: f64,
}

impl Location {
    /// A location at sea level.
    #[must_use]
    pub const fn new(latitude_degrees: f64, longitude_degrees: f64, elevation_metres: f64) -> Self {
        Self {
            latitude_degrees,
            longitude_degrees,
            elevation_metres,
        }
    }
}

/// How far the Sun must be below the horizon for a twilight to have ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Twilight {
    /// 6° — the Sun's centre six degrees below the horizon.
    Civil,
    /// 12° — the sea horizon is still discernible for a sextant sight.
    Nautical,
    /// 18° — the sky is as dark as it is going to get.
    Astronomical,
}

impl Twilight {
    /// The depression of the Sun's centre below the horizon, in degrees.
    #[must_use]
    pub const fn depression_degrees(self) -> f64 {
        match self {
            Self::Civil => 6.0,
            Self::Nautical => 12.0,
            Self::Astronomical => 18.0,
        }
    }
}

/// Standard atmospheric refraction at the horizon, in degrees.
pub const HORIZONTAL_REFRACTION_DEGREES: f64 = 34.0 / 60.0;

/// The Sun's apparent semidiameter, in degrees. It varies by about 1.7%
/// over the year with the Earth–Sun distance; the convention fixes it.
pub const SOLAR_SEMIDIAMETER_DEGREES: f64 = 16.0 / 60.0;

/// The Earth's radius in metres, as used for the dip of the horizon.
///
/// A round mean radius, this library's choice and not a published
/// constant: the dip goes as the square root of height over radius, so the
/// 0.1 % between this and the WGS 84 equatorial radius changes it by 0.05 %,
/// a fraction of an arcsecond at any height a person stands at.
const EARTH_RADIUS_METRES: f64 = 6_372_000.0;

/// How far below the geometric horizon the visible horizon lies, in degrees,
/// for an observer at a given height.
///
/// Zero at or below sea level; about 0.32° from a hundred metres up. Only the
/// geometry is modelled, not the extra refraction along a long grazing ray.
#[must_use]
pub fn horizon_dip_degrees(elevation_metres: f64) -> f64 {
    if elevation_metres <= 0.0 {
        return 0.0;
    }
    acos(clamp(
        EARTH_RADIUS_METRES / (EARTH_RADIUS_METRES + elevation_metres),
        -1.0,
        1.0,
    )) * RAD_TO_DEG
}

/// The altitude of the Sun's centre at the moment its upper limb touches the
/// visible horizon, in degrees — always slightly negative.
#[must_use]
pub fn sunrise_altitude_degrees(elevation_metres: f64) -> f64 {
    -(HORIZONTAL_REFRACTION_DEGREES
        + SOLAR_SEMIDIAMETER_DEGREES
        + horizon_dip_degrees(elevation_metres))
}

/// The geometric altitude of the Sun's centre above the horizon, in degrees,
/// with no allowance for refraction.
#[must_use]
pub fn solar_altitude(moment: Moment, location: Location) -> f64 {
    let position = solar_position(moment);
    altitude_degrees(
        position,
        local_hour_angle(position, moment, location.longitude_degrees),
        location.latitude_degrees,
    )
}

/// The geometric altitude of the Moon's centre above the horizon, in degrees,
/// as seen from the centre of the Earth.
#[must_use]
pub fn lunar_altitude(moment: Moment, location: Location) -> f64 {
    let position = lunar_position(moment);
    altitude_degrees(
        position,
        local_hour_angle(position, moment, location.longitude_degrees),
        location.latitude_degrees,
    )
}

/// The moment of apparent solar noon — the Sun's upper transit of the local
/// meridian — on a local day.
///
/// Unlike the rise and set functions this one always has an answer, even
/// under the midnight sun: the Sun crosses the meridian every day whether or
/// not it sets.
#[must_use]
pub fn solar_noon(day: Rd, location: Location) -> Moment {
    let mut moment = day.0 as f64 + 0.5 - location.longitude_degrees / 360.0;
    // Newton's method with the constant daily rate of the hour angle as the
    // derivative; the equation of time is at most 16 minutes, so four passes
    // are more than the two this needs.
    for _ in 0..4 {
        let position = solar_position(Moment(moment));
        let hour_angle = signed_degrees(
            apparent_sidereal_time(Moment(moment)) + location.longitude_degrees
                - position.right_ascension_degrees,
        );
        moment -= hour_angle / 360.0;
    }
    Moment(moment)
}

/// The moment of the Sun's lower transit — local apparent midnight — half a
/// day before the noon of the following local day.
#[must_use]
pub fn solar_midnight(day: Rd, location: Location) -> Moment {
    Moment(solar_noon(day, location).0 - 0.5)
}

/// The moment on a local day when the Sun's centre crosses a given altitude
/// on its way up.
///
/// Returns `None` when it does not: the Sun stays below that altitude all
/// day, or never drops to it.
fn sun_crossing(day: Rd, location: Location, target_altitude: f64, rising: bool) -> Option<Moment> {
    let noon = solar_noon(day, location).0;
    let altitude = |moment: Moment| solar_altitude(moment, location) - target_altitude;
    if rising {
        bisect_rising(altitude, Moment(noon - 0.5), Moment(noon))
    } else {
        bisect_falling(altitude, Moment(noon), Moment(noon + 0.5))
    }
}

/// Sunrise on a local day, or `None` under a polar day or a polar night.
///
/// ```
/// use hc_astro::riseset::{Location, sunrise};
/// use hc_calendar::Rd;
///
/// // Tokyo, 2024-01-01. RD 738886 is 2024-01-01.
/// let tokyo = Location::new(35.6895, 139.6917, 0.0);
/// let moment = sunrise(Rd(738_886), tokyo).expect("Tokyo is not in the Arctic");
/// // 09:00 ahead of UT, so local clock time is the UT moment plus nine hours.
/// let local_hour = (moment.0 + 9.0 / 24.0).fract() * 24.0;
/// assert!((local_hour - 6.85).abs() < 0.05, "local hour was {local_hour}");
/// ```
#[must_use]
pub fn sunrise(day: Rd, location: Location) -> Option<Moment> {
    sun_crossing(
        day,
        location,
        sunrise_altitude_degrees(location.elevation_metres),
        true,
    )
}

/// Sunset on a local day, or `None` under a polar day or a polar night.
#[must_use]
pub fn sunset(day: Rd, location: Location) -> Option<Moment> {
    sun_crossing(
        day,
        location,
        sunrise_altitude_degrees(location.elevation_metres),
        false,
    )
}

/// The start of a twilight in the morning: the moment the Sun's centre rises
/// to the twilight's depression below the horizon.
///
/// `None` when the Sun never gets that low (a light summer night) or never
/// gets that high (deep polar winter).
#[must_use]
pub fn dawn(day: Rd, location: Location, twilight: Twilight) -> Option<Moment> {
    sun_crossing(day, location, -twilight.depression_degrees(), true)
}

/// The end of a twilight in the evening.
#[must_use]
pub fn dusk(day: Rd, location: Location, twilight: Twilight) -> Option<Moment> {
    sun_crossing(day, location, -twilight.depression_degrees(), false)
}

/// The altitude of the Moon's centre at which its upper limb touches the
/// visible horizon, in degrees.
///
/// Meeus, chapter 15: `h₀ = 0.7275π − 34′`, where π is the Moon's horizontal
/// parallax. Unlike the Sun the Moon is near enough that parallax outweighs
/// refraction, so this value is slightly *positive*.
#[must_use]
pub fn moonrise_altitude_degrees(moment: Moment, elevation_metres: f64) -> f64 {
    0.7275 * lunar_parallax(moment)
        - HORIZONTAL_REFRACTION_DEGREES
        - horizon_dip_degrees(elevation_metres)
}

/// How finely the lunar day is sampled before the crossing is refined.
///
/// The Moon's altitude can swing by 20° in an hour at a mid latitude, so a
/// twenty-minute step cannot skip a rise and a set in the same interval.
const LUNAR_SCAN_STEPS: i32 = 72;

/// The moment on a local day when the Moon rises or sets, found by scanning
/// the day and then refining.
fn moon_crossing(day: Rd, location: Location, rising: bool) -> Option<Moment> {
    let start = day.0 as f64 - location.longitude_degrees / 360.0;
    let step = 1.0 / f64::from(LUNAR_SCAN_STEPS);
    let altitude = |moment: Moment| {
        lunar_altitude(moment, location)
            - moonrise_altitude_degrees(moment, location.elevation_metres)
    };
    let mut previous = altitude(Moment(start));
    for index in 1..=LUNAR_SCAN_STEPS {
        let moment = start + f64::from(index) * step;
        let current = altitude(Moment(moment));
        let crossed = if rising {
            previous < 0.0 && current >= 0.0
        } else {
            previous > 0.0 && current <= 0.0
        };
        if crossed {
            let low = Moment(moment - step);
            let high = Moment(moment);
            return if rising {
                bisect_rising(altitude, low, high)
            } else {
                bisect_falling(altitude, low, high)
            };
        }
        previous = current;
    }
    None
}

/// Moonrise on a local day, or `None` — which happens about once a month
/// everywhere, because the Moon rises roughly fifty minutes later each day
/// and so skips a calendar day entirely.
#[must_use]
pub fn moonrise(day: Rd, location: Location) -> Option<Moment> {
    moon_crossing(day, location, true)
}

/// Moonset on a local day, or `None`.
#[must_use]
pub fn moonset(day: Rd, location: Location) -> Option<Moment> {
    moon_crossing(day, location, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solar::{Solstice, solstice};
    use crate::time::gregorian_new_year;

    /// Tokyo, at the point NAOJ's 暦計算室 computes 「東京(東京都)」 for:
    /// latitude 35.6581°, longitude 139.7414°, elevation 0 m (`nao-koyomi-dni-tokyo-2024`).
    const TOKYO: Location = Location::new(35.6581, 139.7414, 0.0);
    /// Greenwich, where the prime meridian is by definition.
    const GREENWICH: Location = Location::new(51.4779, -0.0015, 0.0);
    /// Tromsø, well inside the Arctic circle.
    const TROMSO: Location = Location::new(69.6496, 18.9560, 0.0);
    /// Quito, within a fraction of a degree of the equator.
    const QUITO: Location = Location::new(-0.1807, -78.4678, 0.0);
    /// The geographic north pole.
    const NORTH_POLE: Location = Location::new(90.0, 0.0, 0.0);

    /// 2024-01-01 in Rata Die.
    const NEW_YEAR_2024: Rd = Rd(738_886);

    /// The local clock time of a moment, in hours, for a fixed offset from UT.
    fn local_hours(moment: Moment, utc_offset_hours: f64) -> f64 {
        let shifted = moment.0 + utc_offset_hours / 24.0;
        (shifted - hc_core::math::floor(shifted)) * 24.0
    }

    #[test]
    fn the_first_of_january_2024_is_where_we_think_it_is() {
        assert_eq!(gregorian_new_year(2024), NEW_YEAR_2024);
    }

    /// NAOJ's 暦計算室, 「日の出入り＠東京(東京都) 令和6年(2024)01月」
    /// (`nao-koyomi-dni-tokyo-2024`), gives sunrise in Tokyo on 2024-01-01
    /// as 06:50 JST and sunset as 16:38 JST, for the Sun's upper limb on the
    /// horizon.
    #[test]
    fn sunrise_in_tokyo_on_new_years_day_matches_the_national_ephemeris() {
        let moment = sunrise(NEW_YEAR_2024, TOKYO).expect("Tokyo sees the Sun in January");
        assert_eq!(moment.day(), Rd(738_885), "sunrise is 2023-12-31 in UT");
        let local = local_hours(moment, 9.0);
        let error_minutes = (local - (6.0 + 50.0 / 60.0)) * 60.0;
        assert!(
            error_minutes.abs() < 1.0,
            "sunrise was {local} JST, off by {error_minutes} minutes"
        );
    }

    #[test]
    fn sunset_in_tokyo_on_new_years_day_matches_the_national_ephemeris() {
        let moment = sunset(NEW_YEAR_2024, TOKYO).expect("Tokyo sees the Sun in January");
        let local = local_hours(moment, 9.0);
        let error_minutes = (local - (16.0 + 38.0 / 60.0)) * 60.0;
        assert!(
            error_minutes.abs() < 1.0,
            "sunset was {local} JST, off by {error_minutes} minutes"
        );
    }

    #[test]
    fn the_sun_rises_before_noon_and_sets_after_it() {
        for day in 0..366 {
            let date = NEW_YEAR_2024 + day;
            let noon = solar_noon(date, TOKYO).0;
            let rise = sunrise(date, TOKYO).expect("Tokyo").0;
            let set = sunset(date, TOKYO).expect("Tokyo").0;
            assert!(rise < noon, "sunrise after noon on day {day}");
            assert!(set > noon, "sunset before noon on day {day}");
            assert!(
                set - rise > 0.38 && set - rise < 0.62,
                "day length on {day}"
            );
        }
    }

    #[test]
    fn solar_noon_is_when_the_sun_is_highest() {
        let noon = solar_noon(NEW_YEAR_2024, TOKYO);
        let peak = solar_altitude(noon, TOKYO);
        for offset in [-0.05, -0.01, 0.01, 0.05] {
            let nearby = solar_altitude(Moment(noon.0 + offset), TOKYO);
            assert!(nearby < peak, "the Sun was higher {offset} days from noon");
        }
    }

    #[test]
    fn solar_noon_agrees_with_the_equation_of_time() {
        // Apparent noon is mean local noon minus the equation of time.
        for day in [0i64, 40, 100, 180, 260, 307, 350] {
            let date = NEW_YEAR_2024 + day;
            let noon = solar_noon(date, GREENWICH);
            let mean_noon = date.0 as f64 + 0.5 - GREENWICH.longitude_degrees / 360.0;
            let predicted = mean_noon - crate::solar::equation_of_time(noon);
            let error_seconds = (noon.0 - predicted) * 86_400.0;
            assert!(
                error_seconds.abs() < 2.0,
                "day {day}: noon and the equation of time differ by {error_seconds} s"
            );
        }
    }

    #[test]
    fn solar_midnight_is_half_a_day_before_the_following_noon() {
        let midnight = solar_midnight(NEW_YEAR_2024, TOKYO);
        let noon = solar_noon(NEW_YEAR_2024, TOKYO);
        assert!((noon.0 - midnight.0 - 0.5).abs() < 1e-12);
        assert!(solar_altitude(midnight, TOKYO) < 0.0);
    }

    #[test]
    fn the_polar_night_has_no_sunrise_and_the_midnight_sun_has_no_sunset() {
        // Tromsø: the Sun stays down from late November to mid January and
        // stays up from late May to late July.
        let midwinter = solstice(2023, Solstice::December).day();
        assert!(sunrise(midwinter, TROMSO).is_none(), "polar night");
        assert!(sunset(midwinter, TROMSO).is_none(), "polar night");

        let midsummer = solstice(2024, Solstice::June).day();
        assert!(sunrise(midsummer, TROMSO).is_none(), "midnight sun");
        assert!(sunset(midsummer, TROMSO).is_none(), "midnight sun");
    }

    #[test]
    fn the_polar_night_is_a_night_and_the_midnight_sun_is_a_day() {
        let midwinter = solstice(2023, Solstice::December).day();
        assert!(solar_altitude(solar_noon(midwinter, TROMSO), TROMSO) < 0.0);
        let midsummer = solstice(2024, Solstice::June).day();
        assert!(solar_altitude(solar_midnight(midsummer, TROMSO), TROMSO) > 0.0);
    }

    #[test]
    fn tromso_gets_its_sun_back_in_the_middle_of_january() {
        // The Sun returns to Tromsø around 15 January and is gone again
        // around 27 November.
        let mut first_sunrise = None;
        for day in 0..40 {
            let date = NEW_YEAR_2024 + day;
            if sunrise(date, TROMSO).is_some() {
                first_sunrise = Some(day);
                break;
            }
        }
        let day = first_sunrise.expect("the Sun comes back");
        assert!((12..18).contains(&day), "the Sun returned on day {day}");
    }

    #[test]
    fn the_north_pole_has_one_sunrise_and_one_sunset_a_year() {
        let mut sunrises = 0;
        let mut sunsets = 0;
        for day in 0..366 {
            let date = NEW_YEAR_2024 + day;
            if sunrise(date, NORTH_POLE).is_some() {
                sunrises += 1;
            }
            if sunset(date, NORTH_POLE).is_some() {
                sunsets += 1;
            }
        }
        assert_eq!(sunrises, 1, "the pole should see one sunrise a year");
        assert_eq!(sunsets, 1, "the pole should see one sunset a year");
    }

    #[test]
    fn the_equator_has_days_of_almost_exactly_twelve_hours_all_year() {
        for day in (0..366).step_by(10) {
            let date = NEW_YEAR_2024 + day;
            let rise = sunrise(date, QUITO).expect("the equator").0;
            let set = sunset(date, QUITO).expect("the equator").0;
            let hours = (set - rise) * 24.0;
            // Refraction and the Sun's semidiameter add about seven minutes
            // to the geometric twelve hours, everywhere on the equator.
            assert!(
                (12.05..12.20).contains(&hours),
                "day length {hours} h on day {day}"
            );
        }
    }

    #[test]
    fn the_longest_and_shortest_days_fall_at_the_solstices() {
        let mut longest = (0i64, 0.0f64);
        let mut shortest = (0i64, 24.0f64);
        for day in 0..366 {
            let date = NEW_YEAR_2024 + day;
            let rise = sunrise(date, GREENWICH).expect("London").0;
            let set = sunset(date, GREENWICH).expect("London").0;
            let hours = (set - rise) * 24.0;
            if hours > longest.1 {
                longest = (day, hours);
            }
            if hours < shortest.1 {
                shortest = (day, hours);
            }
        }
        let june = solstice(2024, Solstice::June).day() - NEW_YEAR_2024;
        let december = solstice(2024, Solstice::December).day() - NEW_YEAR_2024;
        assert!((longest.0 - june).abs() <= 1, "longest day {}", longest.0);
        assert!(
            (shortest.0 - december).abs() <= 1,
            "shortest day {}",
            shortest.0
        );
        assert!(
            (longest.1 - 16.63).abs() < 0.2,
            "longest was {} h",
            longest.1
        );
        assert!(
            (shortest.1 - 7.83).abs() < 0.2,
            "shortest was {} h",
            shortest.1
        );
    }

    #[test]
    fn the_twilights_nest_inside_one_another() {
        let date = NEW_YEAR_2024 + 60;
        let astronomical = dawn(date, TOKYO, Twilight::Astronomical).expect("dawn").0;
        let nautical = dawn(date, TOKYO, Twilight::Nautical).expect("dawn").0;
        let civil = dawn(date, TOKYO, Twilight::Civil).expect("dawn").0;
        let rise = sunrise(date, TOKYO).expect("sunrise").0;
        assert!(astronomical < nautical, "astronomical dawn is first");
        assert!(nautical < civil, "nautical dawn precedes civil");
        assert!(civil < rise, "civil dawn precedes sunrise");

        let set = sunset(date, TOKYO).expect("sunset").0;
        let civil_dusk = dusk(date, TOKYO, Twilight::Civil).expect("dusk").0;
        let nautical_dusk = dusk(date, TOKYO, Twilight::Nautical).expect("dusk").0;
        let astronomical_dusk = dusk(date, TOKYO, Twilight::Astronomical).expect("dusk").0;
        assert!(set < civil_dusk);
        assert!(civil_dusk < nautical_dusk);
        assert!(nautical_dusk < astronomical_dusk);
    }

    #[test]
    fn the_sun_is_at_the_stated_depression_at_each_twilight() {
        let date = NEW_YEAR_2024 + 60;
        for twilight in [Twilight::Civil, Twilight::Nautical, Twilight::Astronomical] {
            let moment = dawn(date, TOKYO, twilight).expect("dawn");
            let altitude = solar_altitude(moment, TOKYO);
            assert!(
                (altitude + twilight.depression_degrees()).abs() < 0.001,
                "{twilight:?}: altitude was {altitude}"
            );
        }
    }

    #[test]
    fn astronomical_night_never_arrives_in_a_british_midsummer() {
        // Above about 49° north the Sun stays within 18° of the horizon
        // through midsummer, so there is no astronomical darkness at all.
        let midsummer = solstice(2024, Solstice::June).day();
        assert!(dawn(midsummer, GREENWICH, Twilight::Astronomical).is_none());
        assert!(dusk(midsummer, GREENWICH, Twilight::Astronomical).is_none());
        // Civil twilight still ends, though.
        assert!(dusk(midsummer, GREENWICH, Twilight::Civil).is_some());
    }

    #[test]
    fn the_sun_is_on_the_horizon_at_sunrise_and_sunset() {
        let target = sunrise_altitude_degrees(0.0);
        for day in (0..366).step_by(7) {
            let date = NEW_YEAR_2024 + day;
            let rise = sunrise(date, TOKYO).expect("sunrise");
            let set = sunset(date, TOKYO).expect("sunset");
            assert!((solar_altitude(rise, TOKYO) - target).abs() < 0.001);
            assert!((solar_altitude(set, TOKYO) - target).abs() < 0.001);
        }
    }

    #[test]
    fn height_above_sea_level_brings_the_sunrise_forward() {
        let sea_level = Location::new(35.6895, 139.6917, 0.0);
        let mountain = Location::new(35.6895, 139.6917, 3_776.0);
        let low = sunrise(NEW_YEAR_2024, sea_level).expect("sunrise").0;
        let high = sunrise(NEW_YEAR_2024, mountain).expect("sunrise").0;
        assert!(high < low, "the summit should see the Sun first");
        let minutes = (low - high) * 24.0 * 60.0;
        assert!((10.0..25.0).contains(&minutes), "{minutes} minutes earlier");
    }

    #[test]
    fn the_dip_of_the_horizon_grows_with_the_square_root_of_height() {
        assert!(horizon_dip_degrees(0.0).abs() < 1e-12);
        assert!(horizon_dip_degrees(-5.0).abs() < 1e-12);
        let at_100 = horizon_dip_degrees(100.0);
        let at_400 = horizon_dip_degrees(400.0);
        assert!((at_100 - 0.321).abs() < 0.01, "dip at 100 m was {at_100}");
        assert!((at_400 / at_100 - 2.0).abs() < 0.01, "dip should double");
    }

    #[test]
    fn the_moon_rises_on_most_days_and_skips_about_one_a_month() {
        let mut risings = 0;
        for day in 0..366 {
            if moonrise(NEW_YEAR_2024 + day, TOKYO).is_some() {
                risings += 1;
            }
        }
        // Twelve or thirteen skipped days a year: 366 − 366/29.53.
        assert!(
            (350..=356).contains(&risings),
            "the Moon rose on {risings} days"
        );
    }

    #[test]
    fn the_moon_is_on_the_horizon_when_it_rises_and_sets() {
        for day in (0..90).step_by(3) {
            let date = NEW_YEAR_2024 + day;
            if let Some(moment) = moonrise(date, TOKYO) {
                let altitude =
                    lunar_altitude(moment, TOKYO) - moonrise_altitude_degrees(moment, 0.0);
                assert!(altitude.abs() < 0.01, "moonrise altitude off by {altitude}");
            }
            if let Some(moment) = moonset(date, TOKYO) {
                let altitude =
                    lunar_altitude(moment, TOKYO) - moonrise_altitude_degrees(moment, 0.0);
                assert!(altitude.abs() < 0.01, "moonset altitude off by {altitude}");
            }
        }
    }

    #[test]
    fn a_full_moon_rises_about_when_the_sun_sets() {
        // Opposition means the Moon is opposite the Sun, so it clears the
        // horizon within an hour or so of sunset.
        let full = crate::lunar::nth_moon_phase(300, crate::lunar::MoonPhase::Full);
        let day = Moment(full.0 + 9.0 / 24.0).day();
        let rise = moonrise(day, TOKYO).expect("the Moon rises");
        let set = sunset(day, TOKYO).expect("the Sun sets");
        let difference_hours = (rise.0 - set.0) * 24.0;
        assert!(
            difference_hours.abs() < 1.5,
            "moonrise was {difference_hours} hours from sunset"
        );
    }

    #[test]
    fn the_moons_horizon_altitude_is_slightly_positive() {
        // Parallax (0.90° at apogee to 1.02° at perigee, times 0.7275) beats
        // refraction (0.567°), so the Moon's centre is *above* the geometric
        // horizon when its upper limb appears — unlike the Sun's.
        let mut lowest = f64::INFINITY;
        let mut highest: f64 = 0.0;
        for step in 0..400 {
            let value = moonrise_altitude_degrees(Moment(738_886.0 + f64::from(step) * 0.9), 0.0);
            assert!(value > 0.0, "horizon altitude {value} should be positive");
            if value < lowest {
                lowest = value;
            }
            if value > highest {
                highest = value;
            }
        }
        assert!((0.08..0.10).contains(&lowest), "at apogee it was {lowest}");
        assert!(
            (0.17..0.19).contains(&highest),
            "at perigee it was {highest}"
        );
    }

    #[test]
    fn a_location_is_just_its_three_numbers() {
        let place = Location::new(35.0, 139.0, 40.0);
        assert!((place.latitude_degrees - 35.0).abs() < 1e-12);
        assert!((place.longitude_degrees - 139.0).abs() < 1e-12);
        assert!((place.elevation_metres - 40.0).abs() < 1e-12);
        assert_eq!(place, Location::new(35.0, 139.0, 40.0));
    }

    #[test]
    fn the_twilight_enum_names_the_conventional_depressions() {
        assert!((Twilight::Civil.depression_degrees() - 6.0).abs() < 1e-12);
        assert!((Twilight::Nautical.depression_degrees() - 12.0).abs() < 1e-12);
        assert!((Twilight::Astronomical.depression_degrees() - 18.0).abs() < 1e-12);
    }
}
