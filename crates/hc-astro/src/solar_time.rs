//! Local mean and local apparent (sundial) time on the Earth.
//!
//! Before standard time zones every place kept its own time. **Local mean
//! time** is Universal Time moved by the longitude, an hour for every 15°
//! east; **local apparent time** is what a sundial there reads, local mean
//! time plus the equation of time. Reingold and Dershowitz's
//! `local-from-universal`, `apparent-from-local` and their inverses in
//! `calendar-code2` state the two relations (`reingold2018code`); the
//! equation of time is this crate's own, [`equation_of_time`], Meeus's
//! (28.1) on the VSOP87 Sun, rather than the book's shorter series from
//! Meeus's page 185. `docs/systems/hours-of-the-day.md` describes the
//! system and its tests.
//!
//! A reading of either clock is returned as a [`Moment`], because that is
//! the shape of a day count with a fraction, but it is **not** Universal
//! Time: `moment.day()` is the local date and `moment.day_fraction()` the
//! local time of day. Every function here that takes a Universal Time says
//! so in its argument's name, and so does every one that takes a local
//! reading.
//!
//! # Unequal hours
//!
//! Two older reckonings count the hours from the Sun's events rather than
//! from midnight, and both are carried:
//!
//! * **Temporal (seasonal) hours** divide the daylight, sunrise to sunset,
//!   into twelve equal hours and the night into twelve more, so that an
//!   hour is long on a summer day and short on a summer night
//!   ([`daytime_temporal_hour`], [`nighttime_temporal_hour`],
//!   [`universal_from_temporal_time`] and [`temporal_time`];
//!   `daytime-temporal-hour`, `nighttime-temporal-hour` and
//!   `standard-from-sundial` in `calendar-code2`).
//! * **Italian hours** (*ore italiane*) count 24 hours from the "zero
//!   hour", half an hour after sunset taken at a depression of 16′
//!   ([`italian_zero_hour`], [`italian_time`] and
//!   [`universal_from_italian_time`]; `local-zero-hour`,
//!   `italian-from-local` and `local-from-italian`, which fix the place at
//!   Padua; here the place is the caller's).
//!
//! Where the Sun does not rise or set there is no temporal hour and no
//! zero hour, and the functions return a [`MissingSolarEvent`] rather than
//! a number: a length of daylight that is not there is not zero, it is
//! undefined.
//!
//! # Religious times of day
//!
//! Some communities fix a time of day by the Sun's altitude or a shadow's
//! length, and they differ on the angle. Each convention is its own
//! function, named for its rule or its authority (`docs/policy.md` §5):
//! [`asr_shafii`] and [`asr_hanafi`], the Islamic afternoon prayer by the
//! shadow rules (`alt-asr` and `asr` in `calendar-code2`), and
//! [`jewish_dusk_vilna_gaon`] at 4°40′ and [`jewish_sabbath_ends_cohn`] at
//! 7°5′ (`jewish-dusk` and `jewish-sabbath-ends`). The angles are
//! Reingold and Dershowitz's; the authorities' own texts were not read.

use core::fmt;

use hc_calendar::Rd;
use hc_calendar::fixed::Moment;
use hc_core::math::{RAD_TO_DEG, atan2, tan_deg};

use crate::riseset::{Location, solar_altitude, solar_noon, sun_crossing, sunrise, sunset};
use crate::solar::equation_of_time;

/// The difference between local mean time at a longitude and Universal
/// Time, as a fraction of a day: the east-positive longitude over 360°
/// (`zone-from-longitude` in `calendar-code2`).
#[must_use]
pub fn zone_from_longitude(longitude_degrees: f64) -> f64 {
    longitude_degrees / 360.0
}

/// Local mean time at a place for a Universal Time moment
/// (`local-from-universal`).
#[must_use]
pub fn local_mean_time(universal: Moment, location: Location) -> Moment {
    Moment(universal.0 + zone_from_longitude(location.longitude_degrees))
}

/// The Universal Time of a local mean time reading at a place
/// (`universal-from-local`).
#[must_use]
pub fn universal_from_local_mean_time(local_mean: Moment, location: Location) -> Moment {
    Moment(local_mean.0 - zone_from_longitude(location.longitude_degrees))
}

/// Local apparent (sundial) time at a place for a Universal Time moment:
/// local mean time plus the equation of time at that moment
/// (`apparent-from-universal`, through `apparent-from-local`).
///
/// It reads 12:00 when the true Sun crosses the local meridian, to the
/// second or so the equation of time and the transit search here share.
#[must_use]
pub fn local_apparent_time(universal: Moment, location: Location) -> Moment {
    Moment(local_mean_time(universal, location).0 + equation_of_time(universal))
}

/// Local apparent time at a place for a local mean time reading
/// (`apparent-from-local`).
#[must_use]
pub fn apparent_from_local_mean_time(local_mean: Moment, location: Location) -> Moment {
    local_apparent_time(
        universal_from_local_mean_time(local_mean, location),
        location,
    )
}

/// Local mean time at a place for a local apparent (sundial) reading: the
/// inverse of [`apparent_from_local_mean_time`].
///
/// `calendar-code2`'s `local-from-apparent` subtracts the equation of time
/// evaluated at the sundial reading taken as if it were mean time, which
/// is off by the equation's change over its own size, up to about 0.4 s.
/// This solves the relation exactly instead, by fixed-point iteration: the
/// equation of time changes by at most 30 s a day, so three rounds settle
/// far below a microsecond.
#[must_use]
pub fn local_mean_from_apparent_time(apparent: Moment, location: Location) -> Moment {
    let mut local_mean = apparent.0;
    for _ in 0..3 {
        let universal = local_mean - zone_from_longitude(location.longitude_degrees);
        local_mean = apparent.0 - equation_of_time(Moment(universal));
    }
    Moment(local_mean)
}

/// The Universal Time of a local apparent (sundial) reading at a place
/// (`universal-from-apparent`).
#[must_use]
pub fn universal_from_local_apparent_time(apparent: Moment, location: Location) -> Moment {
    universal_from_local_mean_time(local_mean_from_apparent_time(apparent, location), location)
}

/// A solar event a reckoning needs that does not happen on the day asked.
///
/// Above the polar circles the Sun can stay up or down all day, and in
/// summer at high latitudes it may never sink far enough for a twilight
/// angle. The reckonings in this module are undefined there, and say so.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MissingSolarEvent {
    /// The Sun does not rise on this local day.
    Sunrise(Rd),
    /// The Sun does not set on this local day.
    Sunset(Rd),
    /// The Sun does not reach the stated depression below the horizon, in
    /// arcminutes, on the evening of this local day.
    Depression {
        /// The local day.
        day: Rd,
        /// The depression sought, in arcminutes.
        arcminutes: u16,
    },
    /// The Sun is not above the horizon at noon on this local day, so
    /// nothing casts a shadow and the shadow rules for ʿaṣr have no
    /// answer.
    NoNoonShadow(Rd),
}

impl fmt::Display for MissingSolarEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sunrise(day) => write!(f, "the Sun does not rise on RD {}", day.0),
            Self::Sunset(day) => write!(f, "the Sun does not set on RD {}", day.0),
            Self::Depression { day, arcminutes } => write!(
                f,
                "the Sun does not reach {arcminutes}′ below the horizon on the evening of RD {}",
                day.0
            ),
            Self::NoNoonShadow(day) => {
                write!(
                    f,
                    "the Sun is not up at noon on RD {}, so there is no shadow",
                    day.0
                )
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for MissingSolarEvent {}

/// Sunrise on a local day, or the error naming its absence.
fn sunrise_or_error(day: Rd, location: Location) -> Result<Moment, MissingSolarEvent> {
    sunrise(day, location).ok_or(MissingSolarEvent::Sunrise(day))
}

/// Sunset on a local day, or the error naming its absence.
fn sunset_or_error(day: Rd, location: Location) -> Result<Moment, MissingSolarEvent> {
    sunset(day, location).ok_or(MissingSolarEvent::Sunset(day))
}

/// The moment in the evening of a local day when the Sun's centre is a
/// number of arcminutes below the geometric horizon, with no refraction,
/// as `calendar-code2`'s `dusk` takes its angle.
fn evening_depression(
    day: Rd,
    location: Location,
    arcminutes: u16,
) -> Result<Moment, MissingSolarEvent> {
    sun_crossing(day, location, -f64::from(arcminutes) / 60.0, false)
        .ok_or(MissingSolarEvent::Depression { day, arcminutes })
}

/// The length of a daytime temporal hour on a local day, as a fraction of
/// a day: a twelfth of sunrise to sunset (`daytime-temporal-hour`).
///
/// # Errors
///
/// [`MissingSolarEvent::Sunrise`] or [`MissingSolarEvent::Sunset`] where
/// the Sun does not rise or set that day.
pub fn daytime_temporal_hour(day: Rd, location: Location) -> Result<f64, MissingSolarEvent> {
    let rise = sunrise_or_error(day, location)?;
    let set = sunset_or_error(day, location)?;
    Ok((set.0 - rise.0) / 12.0)
}

/// The length of a nighttime temporal hour for the night that begins on a
/// local day, as a fraction of a day: a twelfth of that day's sunset to
/// the next day's sunrise (`nighttime-temporal-hour`).
///
/// # Errors
///
/// [`MissingSolarEvent::Sunset`] where the Sun does not set that day, or
/// [`MissingSolarEvent::Sunrise`] where it does not rise the next.
pub fn nighttime_temporal_hour(day: Rd, location: Location) -> Result<f64, MissingSolarEvent> {
    let set = sunset_or_error(day, location)?;
    let rise = sunrise_or_error(day + 1, location)?;
    Ok((rise.0 - set.0) / 12.0)
}

/// The Universal Time of a reading in temporal hours at a place
/// (`standard-from-sundial`, which returns standard time; this returns
/// Universal Time).
///
/// The reading is a [`Moment`] whose `day()` is the local date and whose
/// `day_fraction()` times 24 is the temporal hour: 6 is sunrise, 12 the
/// middle of the daylight, 18 sunset, and 0 the middle of the night
/// before. Hours 6 to 18 are daytime hours of the date; hours before 6 are
/// nighttime hours of the night that began the evening before, and hours
/// after 18 of the night that begins that evening.
///
/// # Errors
///
/// A [`MissingSolarEvent`] naming the sunrise or sunset the hour needs, where
/// it does not happen.
pub fn universal_from_temporal_time(
    temporal: Moment,
    location: Location,
) -> Result<Moment, MissingSolarEvent> {
    let day = temporal.day();
    let hour = temporal.day_fraction() * 24.0;
    if (6.0..=18.0).contains(&hour) {
        let length = daytime_temporal_hour(day, location)?;
        Ok(Moment(
            sunrise_or_error(day, location)?.0 + (hour - 6.0) * length,
        ))
    } else if hour < 6.0 {
        let length = nighttime_temporal_hour(day - 1, location)?;
        Ok(Moment(
            sunset_or_error(day - 1, location)?.0 + (hour + 6.0) * length,
        ))
    } else {
        let length = nighttime_temporal_hour(day, location)?;
        Ok(Moment(
            sunset_or_error(day, location)?.0 + (hour - 18.0) * length,
        ))
    }
}

/// The reading in temporal hours at a place of a Universal Time moment:
/// the inverse of [`universal_from_temporal_time`], in the same shape.
///
/// # Errors
///
/// A [`MissingSolarEvent`] where a sunrise or sunset around the moment does
/// not happen.
pub fn temporal_time(universal: Moment, location: Location) -> Result<Moment, MissingSolarEvent> {
    let local_day = local_mean_time(universal, location).day();
    for day in [local_day - 1, local_day, local_day + 1] {
        let rise = sunrise_or_error(day, location)?;
        let set = sunset_or_error(day, location)?;
        if (rise.0..set.0).contains(&universal.0) {
            let hour = 6.0 + (universal.0 - rise.0) / ((set.0 - rise.0) / 12.0);
            return Ok(Moment(day.0 as f64 + hour / 24.0));
        }
        let next_rise = sunrise_or_error(day + 1, location)?;
        if (set.0..next_rise.0).contains(&universal.0) {
            let hour = 18.0 + (universal.0 - set.0) / ((next_rise.0 - set.0) / 12.0);
            return Ok(Moment(day.0 as f64 + hour / 24.0));
        }
    }
    // The three local days around a moment always contain it between one
    // sunrise and the next when every one of those events happens.
    Err(MissingSolarEvent::Sunrise(local_day))
}

/// The depression of the Sun's centre at the sunset from which Italian
/// hours are counted, in arcminutes: 16′, its semidiameter, so that the
/// upper limb is on the geometric horizon (`local-zero-hour`).
pub const ITALIAN_SUNSET_DEPRESSION_ARCMINUTES: u16 = 16;

/// How long after that sunset the Italian zero hour falls, as a fraction
/// of a day: half an hour (`local-zero-hour`).
pub const ITALIAN_ZERO_HOUR_AFTER_SUNSET: f64 = 0.5 / 24.0;

/// The Italian zero hour on the evening of a local day, in Universal Time:
/// half an hour after the Sun's centre is 16′ below the geometric horizon
/// (`local-zero-hour`, which fixes the place at Padua).
///
/// # Errors
///
/// [`MissingSolarEvent::Depression`] where the Sun does not get that low.
pub fn italian_zero_hour(day: Rd, location: Location) -> Result<Moment, MissingSolarEvent> {
    let sunset = evening_depression(day, location, ITALIAN_SUNSET_DEPRESSION_ARCMINUTES)?;
    Ok(Moment(sunset.0 + ITALIAN_ZERO_HOUR_AFTER_SUNSET))
}

/// A reading in Italian hours: the date, which begins at the zero hour of
/// the evening before, and the hours since that zero hour.
///
/// The hours run from 0 to about 24; a day between two zero hours is not
/// exactly 24 hours long, so the last reading before the next zero hour
/// can be a minute or two either side of 24.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ItalianTime {
    /// The date the reading belongs to: the civil date after the evening
    /// whose zero hour began it.
    pub day: Rd,
    /// Hours since that zero hour.
    pub hours: f64,
}

/// The Italian-hours reading of a Universal Time moment at a place
/// (`italian-from-local`, which fixes the place at Padua and takes local
/// mean time).
///
/// # Errors
///
/// [`MissingSolarEvent::Depression`] where a zero hour around the moment
/// does not happen.
pub fn italian_time(
    universal: Moment,
    location: Location,
) -> Result<ItalianTime, MissingSolarEvent> {
    let local_day = local_mean_time(universal, location).day();
    for day in [local_day, local_day - 1, local_day - 2] {
        let zero = italian_zero_hour(day, location)?;
        if universal.0 >= zero.0 {
            return Ok(ItalianTime {
                day: day + 1,
                hours: (universal.0 - zero.0) * 24.0,
            });
        }
    }
    // A zero hour falls on every evening, so one of the last three is
    // before any moment of the local day.
    Err(MissingSolarEvent::Depression {
        day: local_day,
        arcminutes: ITALIAN_SUNSET_DEPRESSION_ARCMINUTES,
    })
}

/// The Universal Time of a reading in Italian hours at a place
/// (`local-from-italian`): the zero hour of the evening before the date,
/// plus the hours.
///
/// # Errors
///
/// [`MissingSolarEvent::Depression`] where that zero hour does not happen.
pub fn universal_from_italian_time(
    reading: ItalianTime,
    location: Location,
) -> Result<Moment, MissingSolarEvent> {
    let zero = italian_zero_hour(reading.day - 1, location)?;
    Ok(Moment(zero.0 + reading.hours / 24.0))
}

/// ʿAṣr by a shadow rule: the afternoon moment when a vertical object's
/// shadow is its noon shadow plus `lengths` times its height.
fn asr_by_shadow(day: Rd, location: Location, lengths: f64) -> Result<Moment, MissingSolarEvent> {
    let noon = solar_noon(day, location);
    let noon_altitude = solar_altitude(noon, location);
    if noon_altitude <= 0.0 {
        return Err(MissingSolarEvent::NoNoonShadow(day));
    }
    // The shadow of a unit height is cot h; at ʿaṣr it is cot A + lengths,
    // so tan h = tan A / (1 + lengths · tan A).
    let tangent = tan_deg(noon_altitude);
    let altitude = atan2(tangent, 1.0 + lengths * tangent) * RAD_TO_DEG;
    // The Sun always falls from its noon altitude to this lower one before
    // it sets, so the search cannot miss on a day with a noon shadow.
    sun_crossing(day, location, altitude, false).ok_or(MissingSolarEvent::NoNoonShadow(day))
}

/// ʿAṣr by the **Shafiʿi** rule, in Universal Time: when a shadow is its
/// noon length plus once the object's height (`alt-asr` in
/// `calendar-code2`, which names the rule).
///
/// # Errors
///
/// [`MissingSolarEvent::NoNoonShadow`] where the Sun is not up at noon.
pub fn asr_shafii(day: Rd, location: Location) -> Result<Moment, MissingSolarEvent> {
    asr_by_shadow(day, location, 1.0)
}

/// ʿAṣr by the **Hanafi** rule, in Universal Time: when a shadow is its
/// noon length plus twice the object's height (`asr` in `calendar-code2`).
///
/// # Errors
///
/// [`MissingSolarEvent::NoNoonShadow`] where the Sun is not up at noon.
pub fn asr_hanafi(day: Rd, location: Location) -> Result<Moment, MissingSolarEvent> {
    asr_by_shadow(day, location, 2.0)
}

/// The depression of Jewish dusk as the Vilna Gaon reckons it, in
/// arcminutes: 4°40′ (`jewish-dusk` in `calendar-code2`).
pub const JEWISH_DUSK_VILNA_GAON_ARCMINUTES: u16 = 4 * 60 + 40;

/// The depression at which the Sabbath ends as Berthold Cohn reckons it,
/// in arcminutes: 7°5′ (`jewish-sabbath-ends` in `calendar-code2`).
pub const JEWISH_SABBATH_ENDS_COHN_ARCMINUTES: u16 = 7 * 60 + 5;

/// Jewish dusk on the evening of a local day by the Vilna Gaon's angle,
/// 4°40′ below the geometric horizon, in Universal Time.
///
/// # Errors
///
/// [`MissingSolarEvent::Depression`] where the Sun does not get that low.
pub fn jewish_dusk_vilna_gaon(day: Rd, location: Location) -> Result<Moment, MissingSolarEvent> {
    evening_depression(day, location, JEWISH_DUSK_VILNA_GAON_ARCMINUTES)
}

/// The end of the Sabbath on the evening of a local day by Berthold
/// Cohn's angle, 7°5′ below the geometric horizon, in Universal Time.
///
/// # Errors
///
/// [`MissingSolarEvent::Depression`] where the Sun does not get that low.
pub fn jewish_sabbath_ends_cohn(day: Rd, location: Location) -> Result<Moment, MissingSolarEvent> {
    evening_depression(day, location, JEWISH_SABBATH_ENDS_COHN_ARCMINUTES)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::{gregorian_new_year, universal_from_dynamical_julian_date};

    const GREENWICH: Location = Location::new(51.4779, 0.0, 0.0);
    /// Padua, as `calendar-code2` places it: 45°24′28″ N, 11°53′9″ E.
    const PADUA: Location = Location::new(
        45.0 + 24.0 / 60.0 + 28.0 / 3600.0,
        11.0 + 53.0 / 60.0 + 9.0 / 3600.0,
        18.0,
    );

    #[test]
    fn local_mean_time_is_an_hour_for_every_fifteen_degrees_east() {
        let universal = Moment(738_886.25);
        assert_eq!(local_mean_time(universal, GREENWICH), universal);
        let east = Location::new(0.0, 15.0, 0.0);
        let local = local_mean_time(universal, east);
        // A day count near 738 886 resolves 10⁻¹⁰ day, 10 µs.
        assert!((local.0 - universal.0 - 1.0 / 24.0).abs() < 1e-9);
        let west = Location::new(0.0, -90.0, 0.0);
        let local = local_mean_time(universal, west);
        assert!((local.0 - universal.0 + 0.25).abs() < 1e-9);
        assert!((universal_from_local_mean_time(local, west).0 - universal.0).abs() < 1e-9);
    }

    /// Padua is 11°53′9″ east: 47 min 32.6 s ahead of Greenwich.
    #[test]
    fn padua_keeps_greenwich_time_plus_forty_seven_and_a_half_minutes() {
        let universal = Moment(738_886.0);
        let seconds = (local_mean_time(universal, PADUA).0 - universal.0) * 86_400.0;
        assert!((seconds - 2_852.6).abs() < 0.05, "offset {seconds} s");
    }

    /// Meeus, example 28.a: on 1992 October 13.0 TD the equation of time
    /// is +13 min 42.6 s, so a sundial at Greenwich reads that far ahead
    /// of the clock.
    #[test]
    fn a_greenwich_sundial_runs_ahead_by_meeus_example_28a() {
        let universal = universal_from_dynamical_julian_date(2_448_908.5);
        let apparent = local_apparent_time(universal, GREENWICH);
        let seconds = (apparent.0 - universal.0) * 86_400.0;
        assert!(
            (seconds - 822.6).abs() < 0.5,
            "sundial ahead by {seconds} s"
        );
    }

    #[test]
    fn the_sundial_reads_noon_at_the_suns_transit() {
        let start = gregorian_new_year(2024);
        for day in [0i64, 45, 100, 170, 230, 300, 355] {
            let noon = solar_noon(start + day, PADUA);
            let apparent = local_apparent_time(noon, PADUA);
            assert_eq!(apparent.day(), start + day);
            let error = (apparent.day_fraction() - 0.5) * 86_400.0;
            assert!(
                error.abs() < 2.0,
                "day {day}: sundial noon off by {error} s"
            );
        }
    }

    #[test]
    fn apparent_and_mean_time_invert_each_other() {
        let start = gregorian_new_year(2024).0 as f64;
        for step in 0..73 {
            let universal = Moment(start + f64::from(step) * 5.013);
            let local_mean = local_mean_time(universal, PADUA);
            let apparent = apparent_from_local_mean_time(local_mean, PADUA);
            assert_eq!(apparent, local_apparent_time(universal, PADUA));
            let back = local_mean_from_apparent_time(apparent, PADUA);
            assert!((back.0 - local_mean.0).abs() * 86_400.0 < 1e-4);
            let universal_back = universal_from_local_apparent_time(apparent, PADUA);
            assert!((universal_back.0 - universal.0).abs() * 86_400.0 < 1e-4);
        }
    }

    /// Tokyo at the point NAOJ computes for, as `riseset`'s tests have it.
    const TOKYO: Location = Location::new(35.6581, 139.7414, 0.0);
    /// Tromsø, inside the Arctic circle.
    const TROMSO: Location = Location::new(69.6496, 18.9560, 0.0);

    /// NAOJ's 暦計算室 gives sunrise in Tokyo on 2024-01-01 as 06:50 JST
    /// and sunset as 16:38 (`nao-koyomi-dni-tokyo-2024`): 9 h 48 min of
    /// daylight, so a daytime temporal hour of 49.0 minutes, good to the
    /// published minute over twelve, 0.17 min.
    #[test]
    fn a_tokyo_new_years_temporal_hour_matches_the_national_ephemeris() {
        let day = gregorian_new_year(2024);
        let minutes = daytime_temporal_hour(day, TOKYO).expect("Tokyo has a day") * 1440.0;
        assert!((minutes - 49.0).abs() < 0.17, "hour was {minutes} min");
        let night = nighttime_temporal_hour(day, TOKYO).expect("and a night") * 1440.0;
        // Day and night hours of one place sum to about two ordinary ones.
        assert!(
            (minutes + night - 120.0).abs() < 1.0,
            "night hour {night} min"
        );
    }

    #[test]
    fn temporal_hours_six_and_eighteen_are_sunrise_and_sunset() {
        let day = gregorian_new_year(2024) + 170;
        let rise = sunrise(day, PADUA).expect("Padua has a sunrise");
        let set = sunset(day, PADUA).expect("and a sunset");
        let at = |hour: f64| {
            universal_from_temporal_time(Moment(day.0 as f64 + hour / 24.0), PADUA)
                .expect("defined at Padua")
        };
        assert!((at(6.0).0 - rise.0).abs() < 1e-9);
        assert!((at(18.0).0 - set.0).abs() < 1e-9);
        // Midsummer daylight hours are long, the night's short.
        assert!((at(7.0).0 - at(6.0).0) * 1440.0 > 70.0);
        assert!((at(19.0).0 - at(18.0).0) * 1440.0 < 50.0);
        // Hour 0 is the middle of the night before, and hour 24 of the next.
        let midnight = at(0.0);
        let before = sunset(day - 1, PADUA).expect("sunset");
        assert!((midnight.0 - (before.0 + rise.0) / 2.0).abs() < 1e-9);
    }

    #[test]
    fn temporal_time_inverts_its_universal_time() {
        let start = gregorian_new_year(2024).0 as f64;
        for step in 0..200 {
            let universal = Moment(start + f64::from(step) * 1.83 + 0.011);
            let reading = temporal_time(universal, PADUA).expect("defined at Padua");
            let back = universal_from_temporal_time(reading, PADUA).expect("defined");
            assert!(
                (back.0 - universal.0).abs() * 86_400.0 < 1e-3,
                "step {step}: {reading:?}"
            );
        }
    }

    /// Where the Sun does not set there is no temporal hour: an error, not
    /// a number.
    #[test]
    fn temporal_hours_are_refused_under_the_midnight_sun_and_the_polar_night() {
        let midsummer = gregorian_new_year(2024) + 172;
        assert_eq!(
            daytime_temporal_hour(midsummer, TROMSO),
            Err(MissingSolarEvent::Sunrise(midsummer))
        );
        assert_eq!(
            nighttime_temporal_hour(midsummer, TROMSO),
            Err(MissingSolarEvent::Sunset(midsummer))
        );
        let midwinter = gregorian_new_year(2024) + 355;
        assert!(daytime_temporal_hour(midwinter, TROMSO).is_err());
        let reading = Moment(midwinter.0 as f64 + 0.5);
        assert!(universal_from_temporal_time(reading, TROMSO).is_err());
        assert!(temporal_time(reading, TROMSO).is_err());
        assert!(italian_zero_hour(midsummer, TROMSO).is_err());
        assert!(italian_time(Moment(midsummer.0 as f64 + 0.5), TROMSO).is_err());
        assert!(jewish_dusk_vilna_gaon(midsummer, TROMSO).is_err());
        assert_eq!(
            asr_hanafi(midwinter, TROMSO),
            Err(MissingSolarEvent::NoNoonShadow(midwinter))
        );
        let message = MissingSolarEvent::Depression {
            day: midsummer,
            arcminutes: 16,
        }
        .to_string();
        assert!(message.contains("16′"), "{message}");
    }

    #[test]
    fn the_italian_zero_hour_is_half_an_hour_after_the_suns_limb_meets_the_horizon() {
        let day = gregorian_new_year(2024) + 80;
        let zero = italian_zero_hour(day, PADUA).expect("Padua has one");
        let sunset_moment = Moment(zero.0 - ITALIAN_ZERO_HOUR_AFTER_SUNSET);
        let altitude = solar_altitude(sunset_moment, PADUA);
        assert!((altitude + 16.0 / 60.0).abs() < 1e-4, "altitude {altitude}");
        // The limb meets the geometric horizon before the refracted sunset,
        // when the centre is 50′ down.
        let visible = sunset(day, PADUA).expect("sunset");
        assert!(sunset_moment.0 < visible.0);
        let reading = italian_time(zero, PADUA).expect("defined");
        assert_eq!(reading.day, day + 1);
        assert!(reading.hours.abs() < 1e-6);
    }

    #[test]
    fn italian_hours_count_a_day_from_one_zero_hour_to_the_next() {
        let start = gregorian_new_year(2024);
        for offset in [0i64, 90, 180, 270] {
            let day = start + offset;
            let zero = italian_zero_hour(day, PADUA).expect("defined");
            let next = italian_zero_hour(day + 1, PADUA).expect("defined");
            let just_before = Moment(next.0 - 1e-6);
            let reading = italian_time(just_before, PADUA).expect("defined");
            assert_eq!(reading.day, day + 1);
            assert!((reading.hours - 24.0).abs() < 0.1, "{reading:?}");
            // Noon by the mean clock is about 17 or 18 Italian hours in
            // spring and autumn, 15 in summer and 19 in winter.
            let noon = universal_from_local_mean_time(Moment((day + 1).0 as f64 + 0.5), PADUA);
            let hours = italian_time(noon, PADUA).expect("defined").hours;
            assert!(
                (14.5..19.8).contains(&hours),
                "noon at {hours} Italian hours"
            );
            let back = universal_from_italian_time(italian_time(noon, PADUA).expect("ok"), PADUA)
                .expect("defined");
            assert!((back.0 - noon.0).abs() * 86_400.0 < 1e-3);
            assert!(zero.0 < noon.0 && noon.0 < next.0);
        }
    }

    /// At ʿaṣr the shadow of a unit height is its noon shadow plus one
    /// (Shafiʿi) or two (Hanafi), and the Hanafi time is the later.
    #[test]
    fn asr_is_where_the_shadow_rule_puts_it() {
        let day = gregorian_new_year(2024) + 100;
        let noon = solar_noon(day, PADUA);
        let noon_shadow = 1.0 / tan_deg(solar_altitude(noon, PADUA));
        let shadow = |moment: Moment| 1.0 / tan_deg(solar_altitude(moment, PADUA));
        let shafii = asr_shafii(day, PADUA).expect("defined");
        let hanafi = asr_hanafi(day, PADUA).expect("defined");
        assert!((shadow(shafii) - noon_shadow - 1.0).abs() < 1e-3);
        assert!((shadow(hanafi) - noon_shadow - 2.0).abs() < 1e-3);
        let set = sunset(day, PADUA).expect("sunset");
        assert!(noon.0 < shafii.0 && shafii.0 < hanafi.0 && hanafi.0 < set.0);
    }

    #[test]
    fn the_jewish_evening_times_sit_at_their_angles_in_order() {
        let day = gregorian_new_year(2024) + 200;
        let dusk = jewish_dusk_vilna_gaon(day, PADUA).expect("defined");
        let ends = jewish_sabbath_ends_cohn(day, PADUA).expect("defined");
        assert!((solar_altitude(dusk, PADUA) + (4.0 + 40.0 / 60.0)).abs() < 1e-4);
        assert!((solar_altitude(ends, PADUA) + (7.0 + 5.0 / 60.0)).abs() < 1e-4);
        let set = sunset(day, PADUA).expect("sunset");
        assert!(set.0 < dusk.0 && dusk.0 < ends.0);
    }
}
