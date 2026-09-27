//! Local mean and local apparent (sundial) time on the Earth.
//!
//! Before standard time zones every place kept its own time. **Local mean
//! time** is Universal Time moved by the longitude, an hour for every 15°
//! east; **local apparent time** is what a sundial there reads, local mean
//! time plus the equation of time. Reingold and Dershowitz's
//! `local-from-universal`, `apparent-from-local` and their inverses in
//! `calendar-code2` state the two relations (`reingold2018code`); the
//! equation of time is this crate's own, [`equation_of_time`], the hour
//! angle of the VSOP87 Sun against Universal Time, rather than the book's
//! shorter series from Meeus's page 185, whose mean Sun is taken at
//! dynamical time. `docs/systems/hours-of-the-day.md` describes the system
//! and its tests.
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
//! Three older reckonings count the hours from the Sun's events rather than
//! from midnight, and all three are carried:
//!
//! * **Temporal (seasonal) hours** divide the daylight, sunrise to sunset,
//!   into twelve equal hours and the night into twelve more, so that an
//!   hour is long on a summer day and short on a summer night
//!   ([`daytime_temporal_hour`], [`nighttime_temporal_hour`],
//!   [`universal_from_temporal_time`] and [`temporal_time`];
//!   `daytime-temporal-hour`, `nighttime-temporal-hour` and
//!   `standard-from-sundial` in `calendar-code2`). Of Jewish law's two
//!   reckonings this is the Vilna Gaon's; the Magen Avraham's, daybreak
//!   to nightfall, is below, under each pair of daybreak and nightfall it
//!   is reckoned with.
//! * **Italian hours** (*ore italiane*) count 24 hours from the "zero
//!   hour", half an hour after sunset taken at a depression of 16′
//!   ([`italian_zero_hour`], [`italian_time`] and
//!   [`universal_from_italian_time`]; `local-zero-hour`,
//!   `italian-from-local` and `local-from-italian`, which fix the place at
//!   Padua; here the place is the caller's).
//! * **The Edo 不定時法** divides the daylight from 明け六つ to 暮れ六つ into
//!   six hours and the night into six more, with dawn and dusk where the
//!   Sun's centre is 7°21′41″ below the horizon, the 寛政暦's angle
//!   ([`edo_time_kansei`], [`universal_from_edo_time_kansei`],
//!   [`japanese_dawn_kansei`] and [`japanese_dusk_kansei`], with the hours
//!   named in [`EdoHour`]). The Observatory's 夜明 and 日暮 at 7°21′40″ are
//!   [`japanese_dawn_naoj`] and [`japanese_dusk_naoj`].
//!
//! Where the Sun does not rise or set, or not sink far enough, there is no
//! temporal hour, no zero hour and no Edo hour, and the functions return a
//! [`MissingSolarEvent`] rather than a number: a length of daylight that is
//! not there is not zero, it is undefined.
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
//!
//! The Jewish times counted in temporal hours (*zmanim*) are a table,
//! [`Zman`]: the latest Shema at three hours, the latest morning prayer at
//! four, *minḥah gedolah* at six and a half, *minḥah ketanah* at nine and
//! a half and *plag ha-minḥah* at ten and three quarters. Each reckoning of
//! the hour is its own function: [`zman_gra`], the Vilna Gaon's, from
//! sunrise with a twelfth of sunrise to sunset; [`zman_mga_72_minutes`],
//! the Magen Avraham's, from a dawn 72 minutes before sunrise with a
//! twelfth of that dawn to a nightfall 72 minutes after sunset; and
//! [`zman_mga_16_1_degrees`], the same with dawn and nightfall at 16.1°.
//! Dawn and nightfall have their own functions: [`jewish_dawn_16_1_degrees`],
//! [`jewish_dawn_72_minutes`], [`jewish_nightfall_8_5_degrees`] and
//! [`jewish_nightfall_72_minutes`]. The rules are as KosherJava's
//! `ZmanimCalendar` documentation and Hebcal's `Zmanim` state them
//! (`kosherjava-zmanim`, `hebcal-zmanim-api`), and the times are checked
//! against Hebcal's published zmanim; the Vilna Gaon's, the Magen
//! Avraham's and Rabbi Meir Posen's own texts were not read.

use core::fmt;

use hc_calendar::Rd;
use hc_calendar::fixed::Moment;
use hc_core::math::{RAD_TO_DEG, atan2, floor, round, tan_deg};

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
/// tolerance of the transit search, at every date: the equation of time is
/// the Sun's hour angle against Universal Time, the quantity the transit
/// search solves for. `universal` is Universal Time; the reading returned
/// is not, as the module documentation says.
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
    /// The Sun does not reach the stated depression below the horizon, in
    /// arcminutes, in the morning of this local day.
    DawnDepression {
        /// The local day.
        day: Rd,
        /// The depression sought, in arcminutes.
        arcminutes: u16,
    },
    /// The Sun is not above the horizon at noon on this local day, so
    /// nothing casts a shadow and the shadow rules for ʿaṣr have no
    /// answer.
    NoNoonShadow(Rd),
    /// The Sun does not reach the stated depression below the horizon, in
    /// arcseconds, on the morning or the evening of this local day: the
    /// Japanese dawn and dusk, whose depressions are not whole arcminutes.
    Twilight {
        /// The local day.
        day: Rd,
        /// The depression sought, in arcseconds, rounded.
        arcseconds: u32,
        /// Whether it is the morning's crossing, rather than the evening's.
        morning: bool,
    },
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
            Self::DawnDepression { day, arcminutes } => write!(
                f,
                "the Sun does not reach {arcminutes}′ below the horizon in the morning of RD {}",
                day.0
            ),
            Self::NoNoonShadow(day) => {
                write!(
                    f,
                    "the Sun is not up at noon on RD {}, so there is no shadow",
                    day.0
                )
            }
            Self::Twilight {
                day,
                arcseconds,
                morning,
            } => write!(
                f,
                "the Sun does not reach {}°{}′{}″ below the horizon on the {} of RD {}",
                arcseconds / 3_600,
                arcseconds / 60 % 60,
                arcseconds % 60,
                if *morning { "morning" } else { "evening" },
                day.0
            ),
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

/// The latitude of the 改暦所 at 西三条台 in Kyoto from which the 寛政暦
/// took its dawn and dusk, 35°00′36″ N, as the 寛政暦書 gives it
/// (`nao-rekiwiki-yoake`; the 寛政暦書 itself not read). The 天保暦 used
/// the same value.
pub const KANSEI_OBSERVATORY_LATITUDE_DEGREES: f64 = 35.0 + 36.0 / 3_600.0;

/// The hour angle the Sun moves through in 二刻半, two and a half of the
/// hundred 刻 of a day: 360° × 2.5 / 100 = 9°. Before the 寛政暦 the
/// almanacs put 明け六つ this long before sunrise and 暮れ六つ this long
/// after sunset (`nao-rekiwiki-yoake`).
pub const KANSEI_TWILIGHT_HOUR_ANGLE_DEGREES: f64 = 9.0;

/// The depression of the Sun's centre below the geometric horizon at
/// which the 寛政暦 and the 天保暦 put 明け六つ and 暮れ六つ, in degrees:
/// the Sun's altitude two and a half 刻 after sunset at an equinox in
/// Kyoto, `sin h = cos φ sin 9°` with φ the
/// [`KANSEI_OBSERVATORY_LATITUDE_DEGREES`], which is 7°21′41″
/// (`nao-rekiwiki-yoake`). The value is that formula evaluated, and a
/// test evaluates it again.
pub const KANSEI_DEPRESSION_DEGREES: f64 = 7.361_427_044_417_415;

/// The depression of the Sun's centre at which the National Astronomical
/// Observatory's almanacs and the 理科年表 put 夜明 and 日暮, in
/// arcseconds: 7°21′40″, printed since the almanac for 1912 as the time
/// "明治五年以前明六つ暮六つと称したる時刻に相当す" (`nao-rekiwiki-yoake`;
/// `koyomi8-yoake-higure`, quoting the 理科年表 of 2013). It is one second
/// of arc less than the 寛政暦's, [`KANSEI_DEPRESSION_DEGREES`], which
/// moves the moment by about a tenth of a second.
pub const NAOJ_DAWN_DUSK_DEPRESSION_ARCSECONDS: u32 = 7 * 3_600 + 21 * 60 + 40;

/// The moment on a local day when the Sun's centre rises (`morning`) or
/// sets through a depression in degrees below the geometric horizon, with
/// no refraction, or the error naming the depression to the arcsecond.
fn twilight_at(
    day: Rd,
    location: Location,
    depression_degrees: f64,
    morning: bool,
) -> Result<Moment, MissingSolarEvent> {
    sun_crossing(day, location, -depression_degrees, morning).ok_or(MissingSolarEvent::Twilight {
        day,
        arcseconds: round(depression_degrees * 3_600.0) as u32,
        morning,
    })
}

/// 明け六つ on a local day by the 寛政暦's rule, in Universal Time: the
/// Sun's centre [`KANSEI_DEPRESSION_DEGREES`] below the geometric horizon
/// in the morning.
///
/// # Errors
///
/// [`MissingSolarEvent::Twilight`] where the Sun does not get that low, or
/// that high, that morning.
pub fn japanese_dawn_kansei(day: Rd, location: Location) -> Result<Moment, MissingSolarEvent> {
    twilight_at(day, location, KANSEI_DEPRESSION_DEGREES, true)
}

/// 暮れ六つ on a local day by the 寛政暦's rule, in Universal Time: the
/// Sun's centre [`KANSEI_DEPRESSION_DEGREES`] below the geometric horizon
/// in the evening.
///
/// # Errors
///
/// [`MissingSolarEvent::Twilight`] where the Sun does not get that low, or
/// that high, that evening.
pub fn japanese_dusk_kansei(day: Rd, location: Location) -> Result<Moment, MissingSolarEvent> {
    twilight_at(day, location, KANSEI_DEPRESSION_DEGREES, false)
}

/// 夜明 on a local day as the National Astronomical Observatory computes
/// it, in Universal Time: the Sun's centre 7°21′40″ below the horizon in
/// the morning ([`NAOJ_DAWN_DUSK_DEPRESSION_ARCSECONDS`]).
///
/// # Errors
///
/// [`MissingSolarEvent::Twilight`] where the Sun does not get that low, or
/// that high, that morning.
pub fn japanese_dawn_naoj(day: Rd, location: Location) -> Result<Moment, MissingSolarEvent> {
    twilight_at(
        day,
        location,
        f64::from(NAOJ_DAWN_DUSK_DEPRESSION_ARCSECONDS) / 3_600.0,
        true,
    )
}

/// 日暮 on a local day as the National Astronomical Observatory computes
/// it, in Universal Time: the Sun's centre 7°21′40″ below the horizon in
/// the evening ([`NAOJ_DAWN_DUSK_DEPRESSION_ARCSECONDS`]).
///
/// # Errors
///
/// [`MissingSolarEvent::Twilight`] where the Sun does not get that low, or
/// that high, that evening.
pub fn japanese_dusk_naoj(day: Rd, location: Location) -> Result<Moment, MissingSolarEvent> {
    twilight_at(
        day,
        location,
        f64::from(NAOJ_DAWN_DUSK_DEPRESSION_ARCSECONDS) / 3_600.0,
        false,
    )
}

/// One of the twelve hours of the Edo 不定時法, counted from 明け六つ: six
/// of the daylight, 明け六つ to 暮れ六つ, and six of the night.
///
/// An hour is named by the number of strokes of the bell that opened it,
/// nine at noon and at midnight and one fewer at each hour after, down to
/// four. The names follow the Observatory's list, 今暁九時, 八時, 七時,
/// 明六時, 朝五時, 四時, 昼九時, 八時, 夕七時, 暮六時, 夜五時, 四時, in which a
/// prefix also covers the unprefixed hour after it; here つ stands for 時
/// and 暁 for 今暁 (`nao-rekiwiki-futeiji`). Each is also paired with an
/// earthly branch, 卯 for 明け六つ round to 寅 (`wikipedia-ja-jikoku`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EdoHour(u8);

impl EdoHour {
    /// The twelve, 明け六つ first.
    pub const ALL: [Self; 12] = [
        Self(0),
        Self(1),
        Self(2),
        Self(3),
        Self(4),
        Self(5),
        Self(6),
        Self(7),
        Self(8),
        Self(9),
        Self(10),
        Self(11),
    ];

    /// 明け六つ, the first hour of the daylight.
    pub const DAWN: Self = Self(0);

    /// 昼九つ, which begins at the middle of the daylight.
    pub const NOON: Self = Self(3);

    /// 暮れ六つ, the first hour of the night.
    pub const DUSK: Self = Self(6);

    /// 暁九つ, which begins at the middle of the night.
    pub const MIDNIGHT: Self = Self(9);

    /// The hour at a place in the count from 明け六つ, 0 to 11, or `None`.
    #[must_use]
    pub const fn from_index(index: u8) -> Option<Self> {
        if index < 12 { Some(Self(index)) } else { None }
    }

    /// The place in the count from 明け六つ, 0 to 11.
    #[must_use]
    pub const fn index(self) -> u8 {
        self.0
    }

    /// Whether this is one of the six hours of the daylight.
    #[must_use]
    pub const fn is_daytime(self) -> bool {
        self.0 < 6
    }

    /// The number the hour is named by, the strokes of the bell: 6, 5, 4
    /// in the morning, 9, 8, 7 after noon, and the same again at night.
    #[must_use]
    pub const fn strokes(self) -> u8 {
        [6, 5, 4, 9, 8, 7][(self.0 % 6) as usize]
    }

    /// The name, with the prefix that tells day from night: 明六つ, 朝五つ,
    /// 朝四つ, 昼九つ, 昼八つ, 夕七つ, 暮六つ, 夜五つ, 夜四つ, 暁九つ, 暁八つ,
    /// 暁七つ. The type's documentation says where the names come from.
    #[must_use]
    pub const fn japanese_name(self) -> &'static str {
        [
            "明六つ",
            "朝五つ",
            "朝四つ",
            "昼九つ",
            "昼八つ",
            "夕七つ",
            "暮六つ",
            "夜五つ",
            "夜四つ",
            "暁九つ",
            "暁八つ",
            "暁七つ",
        ][self.0 as usize]
    }

    /// The name in Hepburn romaji, e.g. `"ake mutsu"`.
    #[must_use]
    pub const fn romaji(self) -> &'static str {
        [
            "ake mutsu",
            "asa itsutsu",
            "asa yotsu",
            "hiru kokonotsu",
            "hiru yatsu",
            "yū nanatsu",
            "kure mutsu",
            "yoru itsutsu",
            "yoru yotsu",
            "akatsuki kokonotsu",
            "akatsuki yatsu",
            "akatsuki nanatsu",
        ][self.0 as usize]
    }

    /// The earthly branch the hour is paired with, 卯 for 明け六つ, 午 for
    /// 昼九つ, 酉 for 暮れ六つ and 子 for 暁九つ (`wikipedia-ja-jikoku`).
    ///
    /// This is a pairing of names. The source gives each branch's hour as
    /// about an hour either side of a clock time, not where it begins in
    /// the unequal hours, so it says nothing about where a branch's span
    /// begins here.
    #[must_use]
    pub const fn branch(self) -> &'static str {
        [
            "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥", "子", "丑", "寅",
        ][self.0 as usize]
    }
}

/// A reading of the Edo 不定時法: the day, which begins at its 明け六つ,
/// the hour, and how far into the hour, from 0 to 1.
///
/// The 天保暦 wrote the fraction in tenths, 分: its 暮六時六分 is six
/// tenths of an hour after 暮れ六つ (`nao-rekiwiki-futeiji`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdoTime {
    /// The local date whose 明け六つ began the reading's day. The night
    /// hours after midnight belong to the day before the civil date.
    pub day: Rd,
    /// The hour.
    pub hour: EdoHour,
    /// The part of the hour gone, from 0 to 1.
    pub fraction: f64,
}

impl EdoTime {
    /// The fraction in the 天保暦's tenths, 分, rounded down: 0 to 9.
    #[must_use]
    pub fn tenths(self) -> u8 {
        floor(self.fraction * 10.0).clamp(0.0, 9.0) as u8
    }
}

/// The 不定時法 reading of a Universal Time moment at a place, by the
/// 寛政暦's 明け六つ and 暮れ六つ: the daylight from [`japanese_dawn_kansei`]
/// to [`japanese_dusk_kansei`] cut into six equal hours, and the night from
/// that dusk to the next dawn into six more.
///
/// # Errors
///
/// [`MissingSolarEvent::Twilight`] where a dawn or dusk around the moment
/// does not happen.
pub fn edo_time_kansei(
    universal: Moment,
    location: Location,
) -> Result<EdoTime, MissingSolarEvent> {
    let local_day = local_mean_time(universal, location).day();
    let day = if universal.0 >= japanese_dawn_kansei(local_day, location)?.0 {
        local_day
    } else {
        local_day - 1
    };
    let dawn = japanese_dawn_kansei(day, location)?;
    let dusk = japanese_dusk_kansei(day, location)?;
    let (start, end, first) = if universal.0 < dusk.0 {
        (dawn, dusk, 0.0)
    } else {
        (dusk, japanese_dawn_kansei(day + 1, location)?, 6.0)
    };
    let hours = first + 6.0 * (universal.0 - start.0) / (end.0 - start.0);
    let index = floor(hours).clamp(0.0, 11.0);
    Ok(EdoTime {
        day,
        hour: EdoHour(index as u8),
        fraction: hours - index,
    })
}

/// The Universal Time of a 不定時法 reading at a place: the inverse of
/// [`edo_time_kansei`].
///
/// # Errors
///
/// [`MissingSolarEvent::Twilight`] where the dawn or dusk the hour needs
/// does not happen.
pub fn universal_from_edo_time_kansei(
    reading: EdoTime,
    location: Location,
) -> Result<Moment, MissingSolarEvent> {
    let dawn = japanese_dawn_kansei(reading.day, location)?;
    let dusk = japanese_dusk_kansei(reading.day, location)?;
    let hours = f64::from(reading.hour.index()) + reading.fraction;
    if reading.hour.is_daytime() {
        Ok(Moment(dawn.0 + hours * (dusk.0 - dawn.0) / 6.0))
    } else {
        let next = japanese_dawn_kansei(reading.day + 1, location)?;
        Ok(Moment(dusk.0 + (hours - 6.0) * (next.0 - dusk.0) / 6.0))
    }
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

/// The moment in the morning of a local day when the Sun's centre is a
/// number of arcminutes below the geometric horizon, with no refraction.
fn morning_depression(
    day: Rd,
    location: Location,
    arcminutes: u16,
) -> Result<Moment, MissingSolarEvent> {
    sun_crossing(day, location, -f64::from(arcminutes) / 60.0, true)
        .ok_or(MissingSolarEvent::DawnDepression { day, arcminutes })
}

/// The depression of dawn (*alos ha-shachar*) at 16.1°, in arcminutes:
/// "based on the calculation that the time between dawn and sunrise is 72
/// minutes, the time it takes to walk 4 mil at 18 minutes a mil", as
/// KosherJava's `getAlosHashachar` states it (`kosherjava-zmanim`).
pub const JEWISH_DAWN_16_1_DEGREES_ARCMINUTES: u16 = 16 * 60 + 6;

/// The depression of nightfall (*tzais*) at 8.5°, in arcminutes: when three
/// small stars are visible, as Rabbi Meir Posen computed it in *Ohr Meir*
/// (not read), by KosherJava's `getTzais` (`kosherjava-zmanim`).
pub const JEWISH_NIGHTFALL_8_5_DEGREES_ARCMINUTES: u16 = 8 * 60 + 30;

/// The 72 minutes between dawn and sunrise, and between sunset and
/// nightfall, in the fixed-minute reckoning, as a fraction of a day.
pub const JEWISH_TWILIGHT_72_MINUTES: f64 = 72.0 / 1_440.0;

/// Dawn (*alos ha-shachar*) on a local day at 16.1° below the geometric
/// horizon, in Universal Time.
///
/// # Errors
///
/// [`MissingSolarEvent::DawnDepression`] where the Sun does not get that
/// low, as in London in June.
pub fn jewish_dawn_16_1_degrees(day: Rd, location: Location) -> Result<Moment, MissingSolarEvent> {
    morning_depression(day, location, JEWISH_DAWN_16_1_DEGREES_ARCMINUTES)
}

/// Dawn (*alos ha-shachar*) on a local day as 72 minutes before sunrise,
/// in Universal Time (KosherJava's `getAlos72`).
///
/// # Errors
///
/// [`MissingSolarEvent::Sunrise`] where the Sun does not rise.
pub fn jewish_dawn_72_minutes(day: Rd, location: Location) -> Result<Moment, MissingSolarEvent> {
    Ok(Moment(
        sunrise_or_error(day, location)?.0 - JEWISH_TWILIGHT_72_MINUTES,
    ))
}

/// Nightfall (*tzais*) on the evening of a local day at 8.5° below the
/// geometric horizon, in Universal Time.
///
/// # Errors
///
/// [`MissingSolarEvent::Depression`] where the Sun does not get that low.
pub fn jewish_nightfall_8_5_degrees(
    day: Rd,
    location: Location,
) -> Result<Moment, MissingSolarEvent> {
    evening_depression(day, location, JEWISH_NIGHTFALL_8_5_DEGREES_ARCMINUTES)
}

/// Nightfall (*tzais*) on the evening of a local day as 72 minutes after
/// sunset, Rabbeinu Tam's reckoning as KosherJava's `getTzais72` states
/// it, in Universal Time.
///
/// # Errors
///
/// [`MissingSolarEvent::Sunset`] where the Sun does not set.
pub fn jewish_nightfall_72_minutes(
    day: Rd,
    location: Location,
) -> Result<Moment, MissingSolarEvent> {
    Ok(Moment(
        sunset_or_error(day, location)?.0 + JEWISH_TWILIGHT_72_MINUTES,
    ))
}

/// A time of the Jewish day counted in temporal hours (*shaʿot
/// zmaniyot*) from the start of the day: the latest time for the morning
/// Shema, the earliest for the afternoon prayer, and so on.
///
/// Where the day starts and how long its hour is depends on the
/// reckoning, and each reckoning is its own function: [`zman_gra`],
/// [`zman_mga_72_minutes`] and [`zman_mga_16_1_degrees`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Zman {
    /// The identifier.
    pub id: &'static str,
    /// The name, as Hebcal prints it in English.
    pub english_name: &'static str,
    /// Temporal hours from the start of the day.
    pub hours: f64,
    /// Where the count of hours comes from.
    pub source: &'static str,
}

hc_core::catalogue! {
    type: Zman,
    id: |zman| zman.id,
    provenance: |zman| zman.source,
    tests: zman_catalogue_tests,
    associated;

    /// The five, in the order of the day.
    pub const ALL;
    /// The time with this identifier.
    pub fn by_id;

    entries: {
        /// *Sof zman kriʾat shemaʿ*, the latest time for the morning Shema:
        /// three hours.
        pub const SOF_ZMAN_SHMA = Self {
            id: "sof-zman-shma",
            english_name: "Latest Shema",
            hours: 3.0,
            source: "KosherJava, ZmanimCalendar, getSofZmanShmaGRA and getSofZmanShmaMGA \
                     (kosherjava-zmanim); Hebcal, Zmanim, sofZmanShma (hebcal-zmanim-api)",
        };
        /// *Sof zman tefillah*, the latest time for the morning prayer:
        /// four hours.
        pub const SOF_ZMAN_TFILA = Self {
            id: "sof-zman-tfila",
            english_name: "Latest Shacharit",
            hours: 4.0,
            source: "KosherJava, ZmanimCalendar, getSofZmanTfilaGRA (kosherjava-zmanim); \
                     Hebcal, Zmanim, sofZmanTfilla (hebcal-zmanim-api)",
        };
        /// *Minḥah gedolah*, the earliest time for the afternoon prayer:
        /// six and a half hours.
        pub const MINCHA_GEDOLA = Self {
            id: "mincha-gedola",
            english_name: "Earliest Mincha",
            hours: 6.5,
            source: "KosherJava, ZmanimCalendar, getMinchaGedola (kosherjava-zmanim); Hebcal, \
                     Zmanim, minchaGedola (hebcal-zmanim-api)",
        };
        /// *Minḥah ketanah*, the preferred time for the afternoon prayer:
        /// nine and a half hours.
        pub const MINCHA_KETANA = Self {
            id: "mincha-ketana",
            english_name: "Preferable earliest time to recite Minchah",
            hours: 9.5,
            source: "KosherJava, ZmanimCalendar, getMinchaKetana (kosherjava-zmanim); Hebcal, \
                     Zmanim, minchaKetana (hebcal-zmanim-api)",
        };
        /// *Plag ha-minḥah*, the earliest time the Sabbath may be begun:
        /// ten and three quarter hours.
        pub const PLAG_HAMINCHA = Self {
            id: "plag-hamincha",
            english_name: "Plag haMincha",
            hours: 10.75,
            source: "KosherJava, ZmanimCalendar, getPlagHamincha (kosherjava-zmanim); Hebcal, \
                     Zmanim, plagHaMincha (hebcal-zmanim-api)",
        };
    }
}

/// The GRA's (the Vilna Gaon's) temporal hour on a local day, as a
/// fraction of a day: a twelfth of sunrise to sunset. It is
/// [`daytime_temporal_hour`], under the name of its reckoning.
///
/// # Errors
///
/// [`MissingSolarEvent::Sunrise`] or [`MissingSolarEvent::Sunset`] where
/// the Sun does not rise or set.
pub fn temporal_hour_gra(day: Rd, location: Location) -> Result<f64, MissingSolarEvent> {
    daytime_temporal_hour(day, location)
}

/// The Magen Avraham's temporal hour on a local day with dawn and
/// nightfall 72 minutes from sunrise and sunset, as a fraction of a day: a
/// twelfth of the 72-minute dawn to the 72-minute nightfall (KosherJava's
/// `getShaahZmanisMGA`).
///
/// # Errors
///
/// [`MissingSolarEvent::Sunrise`] or [`MissingSolarEvent::Sunset`] where
/// the Sun does not rise or set.
pub fn temporal_hour_mga_72_minutes(day: Rd, location: Location) -> Result<f64, MissingSolarEvent> {
    let dawn = jewish_dawn_72_minutes(day, location)?;
    let nightfall = jewish_nightfall_72_minutes(day, location)?;
    Ok((nightfall.0 - dawn.0) / 12.0)
}

/// The Magen Avraham's temporal hour on a local day with dawn and
/// nightfall both at 16.1° below the horizon, as a fraction of a day, as
/// Hebcal's `sofZmanShmaMGA16Point1` reckons the day.
///
/// # Errors
///
/// [`MissingSolarEvent::DawnDepression`] or
/// [`MissingSolarEvent::Depression`] where the Sun does not get 16.1° low.
pub fn temporal_hour_mga_16_1_degrees(
    day: Rd,
    location: Location,
) -> Result<f64, MissingSolarEvent> {
    let dawn = jewish_dawn_16_1_degrees(day, location)?;
    let nightfall = evening_depression(day, location, JEWISH_DAWN_16_1_DEGREES_ARCMINUTES)?;
    Ok((nightfall.0 - dawn.0) / 12.0)
}

/// A [`Zman`] on a local day by the GRA: its hours of [`temporal_hour_gra`]
/// after sunrise, in Universal Time.
///
/// # Errors
///
/// [`MissingSolarEvent::Sunrise`] or [`MissingSolarEvent::Sunset`] where
/// the Sun does not rise or set.
pub fn zman_gra(zman: &Zman, day: Rd, location: Location) -> Result<Moment, MissingSolarEvent> {
    let hour = temporal_hour_gra(day, location)?;
    Ok(Moment(
        sunrise_or_error(day, location)?.0 + zman.hours * hour,
    ))
}

/// A [`Zman`] on a local day by the Magen Avraham with the 72-minute dawn
/// and nightfall: its hours of [`temporal_hour_mga_72_minutes`] after the
/// 72-minute dawn, in Universal Time.
///
/// # Errors
///
/// [`MissingSolarEvent::Sunrise`] or [`MissingSolarEvent::Sunset`] where
/// the Sun does not rise or set.
pub fn zman_mga_72_minutes(
    zman: &Zman,
    day: Rd,
    location: Location,
) -> Result<Moment, MissingSolarEvent> {
    let hour = temporal_hour_mga_72_minutes(day, location)?;
    Ok(Moment(
        jewish_dawn_72_minutes(day, location)?.0 + zman.hours * hour,
    ))
}

/// A [`Zman`] on a local day by the Magen Avraham with dawn and nightfall
/// at 16.1°: its hours of [`temporal_hour_mga_16_1_degrees`] after the
/// 16.1° dawn, in Universal Time.
///
/// # Errors
///
/// [`MissingSolarEvent::DawnDepression`] or
/// [`MissingSolarEvent::Depression`] where the Sun does not get 16.1° low.
pub fn zman_mga_16_1_degrees(
    zman: &Zman,
    day: Rd,
    location: Location,
) -> Result<Moment, MissingSolarEvent> {
    let hour = temporal_hour_mga_16_1_degrees(day, location)?;
    Ok(Moment(
        jewish_dawn_16_1_degrees(day, location)?.0 + zman.hours * hour,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::{gregorian_new_year, universal_from_dynamical_julian_date};
    use hc_core::math::{asin, cos_deg, sin_deg};

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
    /// of the clock. Meeus takes his mean Sun at TD, which puts it 59 s of
    /// its motion, 0.16 s of time, ahead of the one Universal Time counts.
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

    /// Hebcal's zmanim (`hebcal-zmanim-api`, retrieved 2026-09-27) for a
    /// place and day: the place, the zone offset in hours, the date, and the
    /// printed minutes of the day, in the order of [`ZMANIM_ORDER`].
    /// A place, its zone offset in hours, a date, and sixteen printed
    /// times of day as hour and minute.
    type Printed = (Location, f64, (i64, u8, u8), [(u8, u8); 16]);

    const HEBCAL: [Printed; 3] = [
        (
            // New York City, 2025-01-01, UTC−5.
            Location::new(40.71427, -74.00597, 0.0),
            -5.0,
            (2025, 1, 1),
            [
                (5, 52),
                (7, 20),
                (8, 56),
                (9, 4),
                (9, 40),
                (9, 57),
                (10, 3),
                (10, 27),
                (12, 23),
                (12, 29),
                (14, 43),
                (15, 25),
                (15, 41),
                (16, 40),
                (17, 25),
                (17, 52),
            ],
        ),
        (
            // Jerusalem, 2025-06-21, UTC+3.
            Location::new(31.76904, 35.21633, 0.0),
            3.0,
            (2025, 6, 21),
            [
                (4, 6),
                (5, 34),
                (8, 24),
                (8, 32),
                (9, 8),
                (9, 49),
                (9, 55),
                (10, 19),
                (13, 17),
                (13, 23),
                (16, 50),
                (17, 32),
                (18, 19),
                (19, 48),
                (20, 30),
                (21, 0),
            ],
        ),
        (
            // London, 2025-03-20, UTC.
            Location::new(51.50853, -0.12574, 0.0),
            0.0,
            (2025, 3, 20),
            [
                (4, 23),
                (6, 3),
                (8, 16),
                (8, 30),
                (9, 6),
                (9, 33),
                (9, 43),
                (10, 7),
                (12, 39),
                (12, 45),
                (15, 42),
                (16, 24),
                (16, 58),
                (18, 14),
                (19, 4),
                (19, 26),
            ],
        ),
    ];

    /// Hebcal's names for the sixteen columns of [`HEBCAL`].
    const ZMANIM_ORDER: [&str; 16] = [
        "alotHaShachar",
        "sunrise",
        "sofZmanShmaMGA16Point1",
        "sofZmanShmaMGA",
        "sofZmanShma",
        "sofZmanTfillaMGA16Point1",
        "sofZmanTfillaMGA",
        "sofZmanTfilla",
        "minchaGedola",
        "minchaGedolaMGA",
        "minchaKetana",
        "minchaKetanaMGA",
        "plagHaMincha",
        "sunset",
        "tzeit85deg",
        "tzeit72min",
    ];

    #[test]
    fn the_zmanim_fall_where_hebcal_prints_them() {
        let mut worst: f64 = 0.0;
        for (place, zone, (year, month, date), printed) in HEBCAL {
            let day = hc_calendar::gregorian::to_fixed(year, month, date).expect("a date");
            let computed = [
                jewish_dawn_16_1_degrees(day, place),
                sunrise(day, place).ok_or(MissingSolarEvent::Sunrise(day)),
                zman_mga_16_1_degrees(&Zman::SOF_ZMAN_SHMA, day, place),
                zman_mga_72_minutes(&Zman::SOF_ZMAN_SHMA, day, place),
                zman_gra(&Zman::SOF_ZMAN_SHMA, day, place),
                zman_mga_16_1_degrees(&Zman::SOF_ZMAN_TFILA, day, place),
                zman_mga_72_minutes(&Zman::SOF_ZMAN_TFILA, day, place),
                zman_gra(&Zman::SOF_ZMAN_TFILA, day, place),
                zman_gra(&Zman::MINCHA_GEDOLA, day, place),
                zman_mga_72_minutes(&Zman::MINCHA_GEDOLA, day, place),
                zman_gra(&Zman::MINCHA_KETANA, day, place),
                zman_mga_72_minutes(&Zman::MINCHA_KETANA, day, place),
                zman_gra(&Zman::PLAG_HAMINCHA, day, place),
                sunset(day, place).ok_or(MissingSolarEvent::Sunset(day)),
                jewish_nightfall_8_5_degrees(day, place),
                jewish_nightfall_72_minutes(day, place),
            ];
            for ((name, moment), (hour, minute)) in ZMANIM_ORDER.iter().zip(computed).zip(printed) {
                let moment = moment.expect("every one of these happens");
                let local = (moment.0 - day.0 as f64) * 1_440.0 + zone * 60.0;
                let offset = local - (f64::from(hour) * 60.0 + f64::from(minute));
                worst = worst.max(offset.abs());
                assert!(
                    offset.abs() < 1.0,
                    "{year}-{month}-{date} {name}: {offset} min"
                );
            }
        }
        // Hebcal prints whole minutes, rounded, and its sunrise and this
        // one differ by seconds: every one of the 48 is within half a
        // minute.
        assert!(worst < 0.5, "worst {worst} min");
    }

    #[test]
    fn the_mga_day_is_the_gra_day_and_two_twilights() {
        let day = gregorian_new_year(2025) + 100;
        let gra = temporal_hour_gra(day, PADUA).expect("defined");
        let mga = temporal_hour_mga_72_minutes(day, PADUA).expect("defined");
        assert!((12.0 * (mga - gra) - 2.0 * JEWISH_TWILIGHT_72_MINUTES).abs() < 1e-9);
        assert_eq!(Some(gra), daytime_temporal_hour(day, PADUA).ok());
        // The Magen Avraham's latest Shema is before the Gra's, and the
        // 16.1° day sits between the two in length at Padua in April.
        let mga_shma = zman_mga_72_minutes(&Zman::SOF_ZMAN_SHMA, day, PADUA).expect("defined");
        let gra_shma = zman_gra(&Zman::SOF_ZMAN_SHMA, day, PADUA).expect("defined");
        assert!(mga_shma.0 < gra_shma.0);
        let dawn = jewish_dawn_16_1_degrees(day, PADUA).expect("defined");
        assert!((solar_altitude(dawn, PADUA) + 16.1).abs() < 1e-4);
        let nightfall = jewish_nightfall_8_5_degrees(day, PADUA).expect("defined");
        assert!((solar_altitude(nightfall, PADUA) + 8.5).abs() < 1e-4);
        assert_eq!(Zman::by_id("plag-hamincha"), Some(Zman::PLAG_HAMINCHA));
    }

    /// Hebcal prints no dawn at 16.1° for London on 21 June 2025, nor the
    /// Magen Avraham's times that need it (`hebcal-zmanim-api`): the Sun
    /// gets only about 15° below the horizon there that night.
    #[test]
    fn there_is_no_sixteen_degree_dawn_in_a_london_june() {
        let london = Location::new(51.50853, -0.12574, 0.0);
        let day = gregorian_new_year(2025) + 171;
        assert_eq!(
            jewish_dawn_16_1_degrees(day, london),
            Err(MissingSolarEvent::DawnDepression {
                day,
                arcminutes: JEWISH_DAWN_16_1_DEGREES_ARCMINUTES
            })
        );
        assert!(zman_mga_16_1_degrees(&Zman::SOF_ZMAN_SHMA, day, london).is_err());
        // The fixed-minute reckonings still answer.
        assert!(zman_mga_72_minutes(&Zman::SOF_ZMAN_SHMA, day, london).is_ok());
        assert!(jewish_nightfall_8_5_degrees(day, london).is_ok());
        let message = jewish_dawn_16_1_degrees(day, london)
            .expect_err("no dawn")
            .to_string();
        assert!(message.contains("morning"), "{message}");
    }

    /// Kyoto, near the old 改暦所 at 西三条台, 35°00′36″ N.
    const KYOTO: Location = Location::new(KANSEI_OBSERVATORY_LATITUDE_DEGREES, 135.7417, 0.0);
    /// Helsinki, where the midsummer Sun stays within 6.4° of the horizon.
    const HELSINKI: Location = Location::new(60.1699, 24.9384, 0.0);

    /// 暦Wiki 「夜明と日暮」: `sin h = − cos φ sin 9°` at φ = 35°00′36″ gives
    /// h = −7°21′41″, the 寛政暦's and the 天保暦's depression; the
    /// Observatory's almanacs print 7°21′40″.
    #[test]
    fn the_kansei_depression_is_nine_degrees_of_hour_angle_after_an_equinox_sunset_in_kyoto() {
        let formula = asin(
            cos_deg(KANSEI_OBSERVATORY_LATITUDE_DEGREES)
                * sin_deg(KANSEI_TWILIGHT_HOUR_ANGLE_DEGREES),
        ) * RAD_TO_DEG;
        assert!((formula - KANSEI_DEPRESSION_DEGREES).abs() < 1e-12);
        assert_eq!(
            round(KANSEI_DEPRESSION_DEGREES * 3_600.0) as u32,
            7 * 3_600 + 21 * 60 + 41
        );
        assert_eq!(NAOJ_DAWN_DUSK_DEPRESSION_ARCSECONDS, 26_500);
        // 二刻半 is 36 minutes: at the March equinox of 2024 the 寛政暦's
        // dusk at Kyoto falls that long after the Sun's centre reaches the
        // geometric horizon, to the Sun's small declination that day.
        let day = gregorian_new_year(2024) + 79;
        let horizon = sun_crossing(day, KYOTO, 0.0, false).expect("an equinox sunset");
        let dusk = japanese_dusk_kansei(day, KYOTO).expect("and a dusk");
        let minutes = (dusk.0 - horizon.0) * 1_440.0;
        assert!((minutes - 36.0).abs() < 0.2, "{minutes} min");
        let dawn = japanese_dawn_kansei(day, KYOTO).expect("a dawn");
        assert!((solar_altitude(dawn, KYOTO) + KANSEI_DEPRESSION_DEGREES).abs() < 1e-4);
        let naoj = japanese_dusk_naoj(day, KYOTO).expect("the Observatory's dusk");
        assert!((naoj.0 - dusk.0).abs() * 86_400.0 < 0.5);
        assert!(naoj.0 < dusk.0);
    }

    /// こよみのページ, 「理科年表の「夜明」と「日暮」の角度・補稿」
    /// (2020-02-16): at Kyoto, 夜明 by the Observatory's 7°21′40″ at
    /// 5:28:47 JST on 20 March and 5:12:54 on 22 September, and the Sun's
    /// centre on the geometric horizon 35 min 56 s and 36 min 0 s later,
    /// the 二刻半 the 寛政暦 meant. The page gives no year and no
    /// coordinates; the year is taken as 2020, the year it was written,
    /// whose equinoxes fell on those days, and the place as the 改暦所. The
    /// intervals hardly depend on either and agree to 2 s; the clock times
    /// come 7 s late, as a point 26″ of longitude east of the 改暦所 would
    /// make them, and are checked to 10 s.
    #[test]
    fn kyoto_dawn_at_the_equinoxes_is_two_and_a_half_koku_before_the_centre_rises() {
        let start = gregorian_new_year(2020);
        for (offset, dawn_jst, interval) in [
            (79, 5.0 * 3_600.0 + 28.0 * 60.0 + 47.0, 35.0 * 60.0 + 56.0),
            (265, 5.0 * 3_600.0 + 12.0 * 60.0 + 54.0, 36.0 * 60.0),
        ] {
            let day = start + offset;
            let dawn = japanese_dawn_naoj(day, KYOTO).expect("a dawn");
            let centre = sun_crossing(day, KYOTO, 0.0, true).expect("a sunrise");
            let seconds = (centre.0 - dawn.0) * 86_400.0;
            assert!(
                (seconds - interval).abs() < 3.0,
                "day {offset}: {seconds} s"
            );
            let jst = (dawn.0 + 9.0 / 24.0 - day.0 as f64) * 86_400.0;
            assert!(
                (jst - dawn_jst).abs() < 10.0,
                "day {offset}: dawn at {jst} s"
            );
        }
    }

    /// 天文学辞典, 「不定時法」: near the summer solstice a daytime hour of
    /// the Edo reckoning was about 2 h 39 min and a night hour about
    /// 1 h 21 min. The entry names no place; Kyoto's, at the solstice of
    /// 2024, are 2 h 37.8 min and 1 h 22.3 min, each within 1.5 min.
    #[test]
    fn a_midsummer_edo_hour_is_about_two_hours_thirty_nine_minutes() {
        let day = gregorian_new_year(2024) + 171;
        let dawn = japanese_dawn_kansei(day, KYOTO).expect("dawn");
        let dusk = japanese_dusk_kansei(day, KYOTO).expect("dusk");
        let next = japanese_dawn_kansei(day + 1, KYOTO).expect("dawn");
        let day_hour = (dusk.0 - dawn.0) * 1_440.0 / 6.0;
        let night_hour = (next.0 - dusk.0) * 1_440.0 / 6.0;
        assert!((day_hour - 159.0).abs() < 1.5, "day hour {day_hour} min");
        assert!(
            (night_hour - 81.0).abs() < 1.5,
            "night hour {night_hour} min"
        );
    }

    #[test]
    fn the_edo_hours_run_from_dawn_through_noon_and_midnight() {
        let day = gregorian_new_year(2024) + 200;
        let at = |hour: EdoHour, fraction: f64| {
            universal_from_edo_time_kansei(
                EdoTime {
                    day,
                    hour,
                    fraction,
                },
                KYOTO,
            )
            .expect("defined at Kyoto")
        };
        let dawn = japanese_dawn_kansei(day, KYOTO).expect("dawn");
        let dusk = japanese_dusk_kansei(day, KYOTO).expect("dusk");
        assert!((at(EdoHour::DAWN, 0.0).0 - dawn.0).abs() < 1e-9);
        assert!((at(EdoHour::DUSK, 0.0).0 - dusk.0).abs() < 1e-9);
        // 昼九つ begins halfway from dawn to dusk, within a minute of the
        // Sun's transit, and 暁九つ halfway through the night.
        let noon = at(EdoHour::NOON, 0.0);
        assert!((noon.0 - solar_noon(day, KYOTO).0).abs() * 1_440.0 < 1.0);
        let midnight = at(EdoHour::MIDNIGHT, 0.0);
        let next = japanese_dawn_kansei(day + 1, KYOTO).expect("dawn");
        assert!((midnight.0 - (dusk.0 + next.0) / 2.0).abs() < 1e-9);
        // The last hour of a day ends at the next day's dawn.
        assert!((at(EdoHour::ALL[11], 1.0).0 - next.0).abs() < 1e-9);
        // 暮六時六分 is six tenths of an hour after 暮れ六つ.
        let reading = edo_time_kansei(at(EdoHour::DUSK, 0.65), KYOTO).expect("defined");
        assert_eq!(reading.hour, EdoHour::DUSK);
        assert_eq!(reading.tenths(), 6);
        // After midnight the reading still belongs to the day that began at
        // the last dawn.
        let reading = edo_time_kansei(Moment(midnight.0 + 0.01), KYOTO).expect("defined");
        assert_eq!(reading.day, day);
        assert_eq!(reading.hour, EdoHour::MIDNIGHT);
    }

    #[test]
    fn the_edo_hours_are_named_by_their_strokes_and_branches() {
        let names: Vec<&str> = EdoHour::ALL
            .iter()
            .map(|hour| hour.japanese_name())
            .collect();
        assert_eq!(
            names,
            [
                "明六つ",
                "朝五つ",
                "朝四つ",
                "昼九つ",
                "昼八つ",
                "夕七つ",
                "暮六つ",
                "夜五つ",
                "夜四つ",
                "暁九つ",
                "暁八つ",
                "暁七つ"
            ]
        );
        let strokes: Vec<u8> = EdoHour::ALL.iter().map(|hour| hour.strokes()).collect();
        assert_eq!(strokes, [6, 5, 4, 9, 8, 7, 6, 5, 4, 9, 8, 7]);
        assert_eq!(EdoHour::DAWN.branch(), "卯");
        assert_eq!(EdoHour::NOON.branch(), "午");
        assert_eq!(EdoHour::DUSK.branch(), "酉");
        assert_eq!(EdoHour::MIDNIGHT.branch(), "子");
        assert_eq!(
            EdoHour::from_index(11).map(EdoHour::romaji),
            Some("akatsuki nanatsu")
        );
        assert_eq!(EdoHour::from_index(12), None);
        assert!(EdoHour::ALL[5].is_daytime() && !EdoHour::ALL[6].is_daytime());
    }

    #[test]
    fn edo_time_inverts_its_universal_time() {
        let start = gregorian_new_year(2024).0 as f64;
        for step in 0..200 {
            let universal = Moment(start + f64::from(step) * 1.83 + 0.011);
            let reading = edo_time_kansei(universal, KYOTO).expect("defined at Kyoto");
            assert!((0.0..1.0).contains(&reading.fraction), "{reading:?}");
            let back = universal_from_edo_time_kansei(reading, KYOTO).expect("defined");
            assert!(
                (back.0 - universal.0).abs() * 86_400.0 < 1e-3,
                "step {step}: {reading:?}"
            );
        }
    }

    /// At Helsinki the midsummer Sun never sinks 7°21′ below the horizon,
    /// so there is no 明け六つ and no Edo hour: an error, not a number.
    #[test]
    fn the_edo_hours_are_refused_on_a_white_night() {
        let midsummer = gregorian_new_year(2024) + 172;
        let missing = japanese_dusk_kansei(midsummer, HELSINKI);
        assert_eq!(
            missing,
            Err(MissingSolarEvent::Twilight {
                day: midsummer,
                arcseconds: 26_501,
                morning: false,
            })
        );
        assert!(japanese_dawn_naoj(midsummer, HELSINKI).is_err());
        assert!(edo_time_kansei(Moment(midsummer.0 as f64 + 0.5), HELSINKI).is_err());
        let message = missing.map(|_| ()).unwrap_err().to_string();
        assert!(
            message.contains("7°21′41″") && message.contains("evening"),
            "{message}"
        );
        // The same day in winter has both.
        assert!(japanese_dawn_kansei(midsummer + 180, HELSINKI).is_ok());
    }
}
