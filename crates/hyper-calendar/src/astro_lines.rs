//! The tab-separated lines the WebAssembly module and the C library write
//! about the Earth's rotation and the Sun's hours, written once.
//!
//! **The range rule** is the sky layer's: `hc-astro`'s README states the
//! era over which its series hold as roughly 1000 BCE to 3000 CE, so every
//! function here answers only for an instant or a day whose proleptic
//! Gregorian year is [`EARLIEST_YEAR`] through [`LATEST_YEAR`], and
//! refuses any other with [`Refusal::OutOfRange`] rather than extrapolate.
//!
//! * The Earth Rotation Angle, the Greenwich mean sidereal time under the
//!   IAU 2006 and the IAU 1982 conventions — two conventions, so two
//!   functions (`docs/policy.md` §5) — and UT2 − UT1, from
//!   [`hc_astro::earth`] and [`hc_astro::ut_variants`]. The instant is a
//!   UT1 reading counted as POSIX seconds are, 86 400 to a day from
//!   1970-01-01 00:00 UT1, as a double.
//! * The clocks of [`hc_astro::solar_time`] read at a Universal Time
//!   instant, and its named times of day on a local day, each by name. A
//!   reckoning that needs a sunrise, a sunset, a depression of the Sun or
//!   a noon shadow that does not happen answers with a line naming what is
//!   missing, as a calendar that refuses a day is a line naming its
//!   refusal, never with a number.
//! * The named horizons of [`hc_astro::horizon`], and sunrise and sunset
//!   on a local day against one of them, by its identifier.
//! * The Heliocentric Julian Date of [`hc_astro::hjd`] in its two time
//!   scales, HJD_TT and HJD_UTC — two scales, so two functions — for a
//!   target's J2000 right ascension and declination. The date crosses as a
//!   Julian Date, a double, and answers for the same years.

use alloc::string::String;

use hc_astro::earth::{
    earth_rotation_angle, mean_sidereal_time_iau1982, mean_sidereal_time_iau2006,
};
use hc_astro::hjd::{self, Target, heliocentric_correction_seconds};
use hc_astro::horizon::{self, Horizon};
use hc_astro::riseset::{self, Location, Twilight};
use hc_astro::solar_time::{MissingSolarEvent, SolarClock, SolarEvent};
use hc_astro::ut_variants::{self, ut2_minus_ut1};
use hc_calendar::Rd;
use hc_calendar::fixed::{Moment, RD_OF_UNIX_EPOCH};
use hc_calendar::gregorian::new_year;
use hc_core::duration::{SECONDS_PER_DAY_F64, days_and_seconds};
use hc_core::math::floor;
use hc_core::scale::TT_MINUS_TAI;
use hc_core::unix::{LeapPolicy, tai_minus_utc_at};

use crate::boundary::{Answer, Line, Refusal};

/// The first proleptic Gregorian year the sky exports answer for.
pub const EARLIEST_YEAR: i64 = -1000;

/// The last proleptic Gregorian year the sky exports answer for.
pub const LATEST_YEAR: i64 = 3000;

/// The first fixed day of the era.
const FIRST_DAY: i64 = new_year(EARLIEST_YEAR).0;

/// The first fixed day after the era.
const END_DAY: i64 = new_year(LATEST_YEAR + 1).0;

/// A fixed day, if it lies in the era.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] outside [`EARLIEST_YEAR`]..=[`LATEST_YEAR`].
pub fn day_in_era(fixed: i64) -> Answer<Rd> {
    if (FIRST_DAY..END_DAY).contains(&fixed) {
        Ok(Rd(fixed))
    } else {
        Err(Refusal::OutOfRange)
    }
}

/// The Universal Time moment of a POSIX timestamp, if it lies in the era.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] outside [`EARLIEST_YEAR`]..=[`LATEST_YEAR`].
pub fn moment_in_era(unix: i64) -> Answer<Moment> {
    moment_on_day(unix, day_in_era)
}

/// The Universal Time moment of a POSIX timestamp, if `day` answers for
/// the fixed day it falls on: the check shared by the exports that take
/// an instant, each with its own range of days.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a day whose number overflows, and what
/// `day` answers for the day.
pub(crate) fn moment_on_day(unix: i64, day: fn(i64) -> Answer<Rd>) -> Answer<Moment> {
    let (unix_day, seconds) = days_and_seconds(unix);
    let fixed = unix_day
        .checked_add(RD_OF_UNIX_EPOCH)
        .ok_or(Refusal::OutOfRange)?;
    day(fixed)?;
    Ok(Moment(fixed as f64 + seconds as f64 / SECONDS_PER_DAY_F64))
}

/// The moment of a UT1 reading given as seconds from 1970-01-01 00:00
/// UT1, if it is finite and lies in the era.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a value that is not finite or lies outside
/// [`EARLIEST_YEAR`]..=[`LATEST_YEAR`].
pub fn ut1_moment(ut1_unix_seconds: f64) -> Answer<Moment> {
    if !ut1_unix_seconds.is_finite() {
        return Err(Refusal::OutOfRange);
    }
    let days = ut1_unix_seconds / SECONDS_PER_DAY_F64 + RD_OF_UNIX_EPOCH as f64;
    if !(FIRST_DAY as f64..END_DAY as f64).contains(&days) {
        return Err(Refusal::OutOfRange);
    }
    Ok(Moment(days))
}

/// The POSIX second a Universal Time moment falls in, rounded down, as
/// the sky exports write every instant.
#[must_use]
pub fn unix_from_moment(moment: Moment) -> i64 {
    floor((moment.0 - RD_OF_UNIX_EPOCH as f64) * SECONDS_PER_DAY_F64) as i64
}

/// One number as a line.
fn number_line(value: Answer<f64>) -> Answer<String> {
    let value = value?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(value);
    line.end();
    Ok(out)
}

/// The Earth Rotation Angle at a UT1 reading, in degrees, 0 to 360.
///
/// # Errors
///
/// As [`ut1_moment`].
pub fn earth_rotation_angle_degrees(ut1_unix_seconds: f64) -> Answer<f64> {
    Ok(earth_rotation_angle(ut1_moment(ut1_unix_seconds)?))
}

/// The mean sidereal time at Greenwich by the IAU 2006 convention at a
/// UT1 reading, in degrees, 0 to 360, with TT as `UT1 + ΔT`.
///
/// # Errors
///
/// As [`ut1_moment`].
pub fn gmst_iau2006_degrees(ut1_unix_seconds: f64) -> Answer<f64> {
    Ok(mean_sidereal_time_iau2006(ut1_moment(ut1_unix_seconds)?))
}

/// The mean sidereal time at Greenwich by the IAU 1982 convention,
/// Meeus's (12.4), at a UT1 reading, in degrees, 0 to 360.
///
/// # Errors
///
/// As [`ut1_moment`].
pub fn gmst_iau1982_degrees(ut1_unix_seconds: f64) -> Answer<f64> {
    Ok(mean_sidereal_time_iau1982(ut1_moment(ut1_unix_seconds)?))
}

/// UT2 − UT1 at a UT1 reading, in seconds: the conventional seasonal
/// variation.
///
/// # Errors
///
/// As [`ut1_moment`].
pub fn ut2_minus_ut1_seconds(ut1_unix_seconds: f64) -> Answer<f64> {
    Ok(ut2_minus_ut1(ut1_moment(ut1_unix_seconds)?))
}

/// The line of `hc_earth_rotation_angle`: [`earth_rotation_angle_degrees`].
///
/// # Errors
///
/// As [`ut1_moment`].
pub fn earth_rotation_angle_line(ut1_unix_seconds: f64) -> Answer<String> {
    number_line(earth_rotation_angle_degrees(ut1_unix_seconds))
}

/// The line of `hc_gmst_iau2006`: [`gmst_iau2006_degrees`].
///
/// # Errors
///
/// As [`ut1_moment`].
pub fn gmst_iau2006_line(ut1_unix_seconds: f64) -> Answer<String> {
    number_line(gmst_iau2006_degrees(ut1_unix_seconds))
}

/// The line of `hc_gmst_iau1982`: [`gmst_iau1982_degrees`].
///
/// # Errors
///
/// As [`ut1_moment`].
pub fn gmst_iau1982_line(ut1_unix_seconds: f64) -> Answer<String> {
    number_line(gmst_iau1982_degrees(ut1_unix_seconds))
}

/// The line of `hc_ut2_minus_ut1`: [`ut2_minus_ut1_seconds`].
///
/// # Errors
///
/// As [`ut1_moment`].
pub fn ut2_minus_ut1_line(ut1_unix_seconds: f64) -> Answer<String> {
    number_line(ut2_minus_ut1_seconds(ut1_unix_seconds))
}

/// A place on the Earth.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a latitude outside −90° to 90°, a
/// longitude outside −180° to 180° or an elevation that is not finite.
pub fn location(latitude: f64, longitude: f64, elevation: f64) -> Answer<Location> {
    if (-90.0..=90.0).contains(&latitude)
        && (-180.0..=180.0).contains(&longitude)
        && elevation.is_finite()
    {
        Ok(Location::new(latitude, longitude, elevation))
    } else {
        Err(Refusal::OutOfRange)
    }
}

/// The depression a missing solar event sought, in arcseconds, if it is a
/// depression.
const fn missing_arcseconds(missing: MissingSolarEvent) -> Option<u32> {
    match missing {
        MissingSolarEvent::Depression { arcminutes, .. }
        | MissingSolarEvent::DawnDepression { arcminutes, .. } => Some(arcminutes as u32 * 60),
        MissingSolarEvent::Twilight { arcseconds, .. } => Some(arcseconds),
        MissingSolarEvent::Sunrise(_)
        | MissingSolarEvent::Sunset(_)
        | MissingSolarEvent::NoNoonShadow(_) => None,
    }
}

/// The three cells naming a missing solar event: what is missing
/// (`sunrise`, `sunset`, `depression` or `no-noon-shadow`), the local day
/// it is missing on, and the depression sought in arcminutes, or empty —
/// empty too for a depression that is not a whole number of arcminutes,
/// the Japanese dawn and dusk, whose figure is
/// [`missing_cells`]'s fourth cell.
fn missing(line: &mut Line<'_>, missing: MissingSolarEvent) {
    let (name, day) = match missing {
        MissingSolarEvent::Sunrise(day) => ("sunrise", day),
        MissingSolarEvent::Sunset(day) => ("sunset", day),
        MissingSolarEvent::Depression { day, .. }
        | MissingSolarEvent::DawnDepression { day, .. }
        | MissingSolarEvent::Twilight { day, .. } => ("depression", day),
        MissingSolarEvent::NoNoonShadow(day) => ("no-noon-shadow", day),
    };
    let arcminutes = missing_arcseconds(missing)
        .filter(|arcseconds| arcseconds % 60 == 0)
        .map(|arcseconds| arcseconds / 60);
    line.cell(name).value(day.0).value_or_empty(arcminutes);
}

/// How many cells [`missing_cells`] writes.
pub const MISSING_COLUMNS: usize = 4;

/// The four cells naming a missing solar event, or four empty ones for
/// `None`: the three of `hc_solar_time`'s line and the depression sought
/// in arcseconds, which is the one exact figure for a depression that is
/// not a whole number of arcminutes.
pub fn missing_cells(line: &mut Line<'_>, missing: Option<MissingSolarEvent>) {
    match missing {
        None => {
            line.empties(MISSING_COLUMNS);
        }
        Some(event) => {
            self::missing(line, event);
            line.value_or_empty(missing_arcseconds(event));
        }
    }
}

/// An instant, or the missing solar event it needs: the whole POSIX
/// seconds of Universal Time, rounded down, and the four cells of
/// [`missing_cells`], the first cell empty when the time does not happen.
pub fn moment_or_missing(line: &mut Line<'_>, answer: Result<Moment, MissingSolarEvent>) {
    match answer {
        Ok(moment) => {
            line.value(unix_from_moment(moment));
            missing_cells(line, None);
        }
        Err(event) => {
            line.empty();
            missing_cells(line, Some(event));
        }
    }
}

/// The line of `hc_solar_time`: a clock's reading at a Universal Time
/// instant at a place, as the local date's fixed day and the hours into
/// it, then the three cells of a missing solar event, empty when the
/// reading exists. The clocks are [`SolarClock::ALL`], selected by
/// identifier: `local-mean`, `local-apparent` (the sundial), `temporal` (6
/// at sunrise, 18 at sunset) and `italian` (hours since the zero hour of
/// the evening before). A reading that needs a solar event that does not
/// happen has its first two cells empty and names the event.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a clock not named, and the range errors of
/// [`moment_in_era`] and [`location`].
pub fn solar_time_line(clock: &str, universal_unix: i64, place: Location) -> Answer<String> {
    let universal = moment_in_era(universal_unix)?;
    let clock = SolarClock::by_id(clock).ok_or(Refusal::Unknown)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    match (clock.read)(universal, place) {
        Ok(reading) => {
            line.value(reading.day.0).value(reading.hours).empties(3);
        }
        Err(event) => {
            line.empties(2);
            missing(&mut line, event);
        }
    }
    line.end();
    Ok(out)
}

/// The line of `hc_solar_event`: a named time of day on a local day at a
/// place, as whole POSIX seconds of Universal Time, rounded down, then
/// the four cells of [`missing_cells`] naming a missing solar event,
/// empty when the time exists. The times are [`SolarEvent::ALL`],
/// selected by identifier: ʿaṣr by the Shafiʿi and the Hanafi shadow
/// rules, Jewish dusk at the Vilna Gaon's 4°40′, the end of the Sabbath at
/// Berthold Cohn's 7°5′, the Italian zero hour, and the Japanese dawn and
/// dusk, 明け六つ and 暮れ六つ by the 寛政暦's 7°21′41″ and the
/// Observatory's 夜明 and 日暮 at 7°21′40″. A time that does not happen
/// that day has its first cell empty and names the event.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a time not named, and the range errors of
/// [`day_in_era`] and [`location`].
pub fn solar_event_line(event: &str, fixed: i64, place: Location) -> Answer<String> {
    let day = day_in_era(fixed)?;
    let event = SolarEvent::by_id(event).ok_or(Refusal::Unknown)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    moment_or_missing(&mut line, (event.reckon)(day, place));
    line.end();
    Ok(out)
}

/// How many columns [`horizons_lines`] writes.
#[cfg(feature = "i18n")]
pub const HORIZONS_COLUMNS: usize = 7;

/// A horizon's name in the locale a tag asks for, by
/// [`hc_i18n::horizons::horizon_name`], and the tag of the table that
/// answered; else its English name, answered by `en`, as a table's name in
/// `hc_holiday_tables` falls back. A tag that does not parse is the root
/// locale, which names nothing.
#[cfg(feature = "i18n")]
#[must_use]
pub fn horizon_name(horizon: &Horizon, tag: &str) -> hc_i18n::horizons::HorizonName {
    let locale = hc_i18n::Locale::parse(tag).unwrap_or(hc_i18n::Locale::ROOT);
    hc_i18n::horizons::horizon_name(&locale, horizon.id).unwrap_or(hc_i18n::horizons::HorizonName {
        name: horizon.english_name,
        tag: "en",
    })
}

/// The lines of `hc_horizons`: every horizon [`horizon::HORIZONS`] carries, one a
/// line, as its identifier, its English name, what it takes the visible
/// horizon to be, its source, its short English name for a label
/// (`geometric dip`, `USNO`, `Calendrical Calculations`), and its name in
/// the locale `locale` names with the tag of the data that named it, by
/// [`horizon_name`]'s rule: only an observatory's or an almanac office's
/// own wording is carried (`hc_i18n::horizons`), so most horizons in most
/// locales are their English name, with `en`.
#[cfg(feature = "i18n")]
#[must_use]
pub fn horizons_lines(locale: &str) -> String {
    let mut out = String::new();
    for horizon in horizon::HORIZONS {
        let named = horizon_name(horizon, locale);
        let mut line = Line::new(&mut out);
        line.cell(horizon.id)
            .cell(horizon.english_name)
            .cell(horizon.description)
            .cell(horizon.source)
            .cell(horizon.short_name)
            .cell(named.name)
            .cell(named.tag);
        line.end();
    }
    out
}

/// The horizon an identifier names, one of [`horizon::HORIZONS`], by
/// [`horizon::by_id`].
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other text, the empty string included: a
/// rising is measured against a horizon, so none is assumed.
pub fn horizon(given: &str) -> Answer<Horizon> {
    horizon::by_id(given).ok_or(Refusal::Unknown)
}

/// The line of a crossing of the horizon: the instant, the three cells of
/// a missing solar event, and the altitude of the Sun's centre at the
/// crossing.
fn crossing_line(
    event: fn(Rd, Location, &Horizon) -> Option<hc_calendar::fixed::Moment>,
    name: &str,
    horizon_id: &str,
    fixed: i64,
    place: Location,
) -> Answer<String> {
    let horizon = horizon(horizon_id)?;
    let day = day_in_era(fixed)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    match event(day, place, &horizon) {
        Some(moment) => line.value(unix_from_moment(moment)).empties(3),
        None => line.empty().cell(name).value(day.0).empty(),
    };
    line.value(horizon.sunrise_altitude_degrees(place.elevation_metres));
    line.end();
    Ok(out)
}

/// The line of `hc_sunrise`: the Sun's upper limb rising over a named
/// horizon on a local day at a place, as whole POSIX seconds of Universal
/// Time, rounded down; then the three cells of a missing solar event,
/// `sunrise`, the day and an empty depression, where the Sun does not
/// rise that day, with the first cell empty instead; then the geometric
/// altitude of the Sun's centre at the crossing, in degrees, which is what
/// the horizon and the place's height make it. The local day runs from
/// local mean midnight to local mean midnight, as
/// [`hc_astro::riseset`]'s does.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a horizon [`horizon()`] does not name, and the
/// range errors of [`day_in_era`] and [`location`].
pub fn sunrise_line(horizon_id: &str, fixed: i64, place: Location) -> Answer<String> {
    crossing_line(riseset::sunrise_with, "sunrise", horizon_id, fixed, place)
}

/// The line of `hc_sunset`: as [`sunrise_line`], for the upper limb's
/// setting, with `sunset` as the missing event.
///
/// # Errors
///
/// As [`sunrise_line`].
pub fn sunset_line(horizon_id: &str, fixed: i64, place: Location) -> Answer<String> {
    crossing_line(riseset::sunset_with, "sunset", horizon_id, fixed, place)
}

/// How many columns [`moonrise_line`] and [`moonset_line`] write.
pub const MOON_CROSSING_COLUMNS: usize = 3;

/// The line of a crossing of the horizon by the Moon's upper limb: the
/// instant as whole POSIX seconds of Universal Time, rounded down, then
/// the name of the missing event and the fixed day, both empty when the
/// Moon crosses, and the first cell empty instead when it does not — which
/// happens about once a month everywhere, the Moon rising some fifty
/// minutes later each day and so skipping a local day.
fn moon_crossing_line(
    event: fn(Rd, Location, &Horizon) -> Option<hc_calendar::fixed::Moment>,
    name: &str,
    horizon_id: &str,
    fixed: i64,
    place: Location,
) -> Answer<String> {
    let horizon = horizon(horizon_id)?;
    let day = day_in_era(fixed)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    match event(day, place, &horizon) {
        Some(moment) => line.value(unix_from_moment(moment)).empties(2),
        None => line.empty().cell(name).value(day.0),
    };
    line.end();
    Ok(out)
}

/// The line of `hc_moonrise`: the Moon's upper limb rising over a named
/// horizon on a local day at a place, as whole POSIX seconds of Universal
/// Time, rounded down; then `moonrise` and the day where the Moon does not
/// rise that day, with the first cell empty instead. The horizon is read
/// as `hc_sunrise` reads it, and its Moon rule — the limb's altitude
/// scaled by the parallax, or the fixed depression a tradition states —
/// is the one [`Horizon::lunar_limb_altitude_degrees`] applies. The local
/// day runs from local mean midnight to local mean midnight, as
/// [`hc_astro::riseset`]'s does.
///
/// # Errors
///
/// As [`sunrise_line`].
pub fn moonrise_line(horizon_id: &str, fixed: i64, place: Location) -> Answer<String> {
    moon_crossing_line(riseset::moonrise_with, "moonrise", horizon_id, fixed, place)
}

/// The line of `hc_moonset`: as [`moonrise_line`], for the upper limb's
/// setting, with `moonset` as the missing event.
///
/// # Errors
///
/// As [`sunrise_line`].
pub fn moonset_line(horizon_id: &str, fixed: i64, place: Location) -> Answer<String> {
    moon_crossing_line(riseset::moonset_with, "moonset", horizon_id, fixed, place)
}

/// The Julian Date at which fixed day 0 begins, 1 721 424.5, from
/// `hc-calendar`'s day counts.
const JULIAN_DATE_OF_RD_ZERO: f64 = hc_calendar::fixed::JDN_OF_RD_ZERO as f64 - 0.5;

/// The Julian Date at which the POSIX epoch, 1970-01-01 00:00, falls,
/// 2 440 587.5.
const JULIAN_DATE_OF_UNIX_EPOCH: f64 =
    (hc_calendar::fixed::JDN_OF_RD_ZERO + hc_calendar::fixed::RD_OF_UNIX_EPOCH) as f64 - 0.5;

/// How many columns [`hjd_tt_line`] writes.
pub const HJD_TT_COLUMNS: usize = 2;

/// How many columns [`hjd_utc_line`] writes.
pub const HJD_UTC_COLUMNS: usize = 3;

/// A Julian Date, if it is finite and its day lies in the era.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a date that is not finite or lies outside
/// [`EARLIEST_YEAR`]..=[`LATEST_YEAR`].
pub(crate) fn julian_date_in_era(julian_date: f64) -> Answer<f64> {
    let days = julian_date - JULIAN_DATE_OF_RD_ZERO;
    if julian_date.is_finite() && (FIRST_DAY as f64..END_DAY as f64).contains(&days) {
        Ok(julian_date)
    } else {
        Err(Refusal::OutOfRange)
    }
}

/// A target's direction: its right ascension, 0° to 360°, and its
/// declination, −90° to 90°, on the mean equator and equinox of J2000.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for either outside its range.
pub fn hjd_target(right_ascension_degrees: f64, declination_degrees: f64) -> Answer<Target> {
    if (0.0..=360.0).contains(&right_ascension_degrees)
        && (-90.0..=90.0).contains(&declination_degrees)
    {
        Ok(Target {
            right_ascension_degrees,
            declination_degrees,
        })
    } else {
        Err(Refusal::OutOfRange)
    }
}

/// The line of `hc_hjd_tt`: the HJD_TT of a Julian Date of TT for a
/// target, and the heliocentric light-time correction added to it, in
/// seconds, negative when the light reaches the Sun first.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a date outside the era or a direction
/// [`hjd_target`] refuses.
pub fn hjd_tt_line(
    tt_julian_date: f64,
    right_ascension_degrees: f64,
    declination_degrees: f64,
) -> Answer<String> {
    let date = julian_date_in_era(tt_julian_date)?;
    let target = hjd_target(right_ascension_degrees, declination_degrees)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(hjd::hjd_tt(date, target))
        .value(heliocentric_correction_seconds(date, target));
    line.end();
    Ok(out)
}

/// The line of `hc_hjd_utc`: the HJD_UTC of a Julian Date of UTC for a
/// target, the correction added to it in seconds, and the TT − UTC in
/// seconds at which the Earth's position was taken: 32.184 s plus TAI −
/// UTC from the leap-second table. `strict` refuses a date outside the
/// table; otherwise the table's ends are held, and before 1961 TAI − UTC
/// is taken as 0.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a date outside the era or a direction
/// [`hjd_target`] refuses, and [`Refusal::NoData`] under `strict` for a
/// date outside the leap-second table.
pub fn hjd_utc_line(
    utc_julian_date: f64,
    right_ascension_degrees: f64,
    declination_degrees: f64,
    strict: bool,
) -> Answer<String> {
    let date = julian_date_in_era(utc_julian_date)?;
    let target = hjd_target(right_ascension_degrees, declination_degrees)?;
    let policy = if strict {
        LeapPolicy::Strict
    } else {
        LeapPolicy::Extrapolate
    };
    let unix = floor((date - JULIAN_DATE_OF_UNIX_EPOCH) * SECONDS_PER_DAY_F64) as i64;
    let tt_minus_utc = TT_MINUS_TAI.as_secs_f64() + tai_minus_utc_at(unix, policy)?.as_secs_f64();
    let tt = date + tt_minus_utc / SECONDS_PER_DAY_F64;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(hjd::hjd_utc(date, tt_minus_utc, target))
        .value(heliocentric_correction_seconds(tt, target))
        .value(tt_minus_utc);
    line.end();
    Ok(out)
}

/// The fixed days of the Gregorian years −9 999 999 through 9 999 999,
/// the days the civil exports answer for, which a reading of GMT or GMAT
/// is taken on.
pub const CIVIL_DAYS: core::ops::RangeInclusive<i64> = -3_652_424_999..=3_652_424_634;

/// A reading of a clock: a fixed day, whole seconds after its midnight,
/// 86 400 being the leap second 23:59:60, and attoseconds.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a day outside [`CIVIL_DAYS`], seconds past
/// 86 400 or attoseconds from 10¹⁸.
fn clock_reading(
    fixed: i64,
    seconds_of_day: u32,
    attoseconds: u64,
) -> Answer<hc_calendar::CivilDateTime> {
    if !CIVIL_DAYS.contains(&fixed) || seconds_of_day > 86_400 {
        return Err(Refusal::OutOfRange);
    }
    let (hour, minute, second) = if seconds_of_day == 86_400 {
        (23, 59, 60)
    } else {
        // Each below 24, 60 and 60 by the check above.
        (
            (seconds_of_day / 3_600) as u8,
            (seconds_of_day / 60 % 60) as u8,
            (seconds_of_day % 60) as u8,
        )
    };
    let time = hc_calendar::CivilTime::new(hour, minute, second, attoseconds)
        .map_err(|_| Refusal::OutOfRange)?;
    Ok(hc_calendar::CivilDateTime::new(Rd(fixed), time))
}

/// A reading as a line: the fixed day, the whole seconds after midnight
/// and the attoseconds.
fn reading_line(reading: hc_calendar::CivilDateTime) -> String {
    let time = reading.time;
    let seconds =
        u32::from(time.hour()) * 3_600 + u32::from(time.minute()) * 60 + u32::from(time.second());
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(reading.day.0)
        .value(seconds)
        .value(time.subsec_attos());
    line.end();
    out
}

/// How many columns [`gmat_from_gmt_line`] and [`gmt_from_gmat_line`]
/// write.
pub const GMAT_COLUMNS: usize = 3;

/// The line of `hc_gmat_from_gmt`: the astronomical date and the
/// Greenwich Mean Astronomical Time of a reading of GMT, mean solar time at
/// Greenwich counted from midnight ([`hc_astro::gmat`]). The astronomical
/// day begins at noon and is named by the civil day it begins on, so
/// GMAT is GMT − 12 h: the fixed day, the whole seconds after the
/// astronomical day's noon, and the attoseconds. The *Nautical Almanac*
/// counted G.M.T. so to 1924, and from midnight from 1925; which a
/// document used is the document's to say.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a day outside [`CIVIL_DAYS`], seconds past
/// 86 400, attoseconds from 10¹⁸, and for 23:59:60, 86 400 s, which has no
/// reading twelve hours earlier.
pub fn gmat_from_gmt_line(fixed: i64, seconds_of_day: u32, attoseconds: u64) -> Answer<String> {
    let gmt = clock_reading(fixed, seconds_of_day, attoseconds)?;
    let gmat = hc_astro::gmat::gmat_from_gmt(gmt).map_err(|_| Refusal::OutOfRange)?;
    Ok(reading_line(gmat))
}

/// The line of `hc_gmt_from_gmat`: the civil date and the GMT of a reading
/// of Greenwich Mean Astronomical Time, the inverse of
/// [`gmat_from_gmt_line`], in its columns.
///
/// # Errors
///
/// As [`gmat_from_gmt_line`]'s, a second 60 included, which GMAT, whose
/// day ends at noon, does not read.
pub fn gmt_from_gmat_line(fixed: i64, seconds_of_day: u32, attoseconds: u64) -> Answer<String> {
    let gmat = clock_reading(fixed, seconds_of_day, attoseconds)?;
    let gmt = hc_astro::gmat::gmt_from_gmat(gmat).map_err(|_| Refusal::OutOfRange)?;
    Ok(reading_line(gmt))
}

/// How many columns the lines of [`ut1r_iers2010_line`] and
/// [`ut1s_iers2010_line`] write.
pub const REGULARIZED_UT1_COLUMNS: usize = 2;

/// A UT1 moment as a UT1 reading counted as POSIX time counts UTC, 86 400
/// seconds a day from 1970-01-01 00:00 UT1: the inverse of [`ut1_moment`].
fn ut1_unix_seconds(moment: Moment) -> f64 {
    (moment.0 - RD_OF_UNIX_EPOCH as f64) * SECONDS_PER_DAY_F64
}

/// The line of a regularised UT1: the regularised reading less UT1, in
/// seconds, and the regularised reading itself, counted as the UT1 given
/// is.
fn regularized_ut1_line(
    ut1_unix_seconds_given: f64,
    minus_ut1: fn(Moment) -> f64,
    regularized: fn(Moment) -> Moment,
) -> Answer<String> {
    let moment = ut1_moment(ut1_unix_seconds_given)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(minus_ut1(moment))
        .value(ut1_unix_seconds(regularized(moment)));
    line.end();
    Ok(out)
}

/// The line of `hc_ut1r_iers2010`: UT1R − UT1 in seconds at a UT1 reading,
/// and the UT1R reading counted as the UT1 is, by
/// [`hc_astro::ut_variants::ut1r_iers2010`] — UT1 with the 41 zonal tides
/// of periods under 35 days of IERS Conventions 2010, Table 8.1, removed.
///
/// # Errors
///
/// As [`ut1_moment`].
pub fn ut1r_iers2010_line(ut1_unix_seconds_given: f64) -> Answer<String> {
    regularized_ut1_line(
        ut1_unix_seconds_given,
        ut_variants::ut1r_minus_ut1_iers2010,
        ut_variants::ut1r_iers2010,
    )
}

/// The line of `hc_ut1s_iers2010`: UT1S − UT1 in seconds at a UT1 reading,
/// and the UT1S reading counted as the UT1 is, by
/// [`hc_astro::ut_variants::ut1s_iers2010`] — UT1 with all 62 zonal tides
/// of IERS Conventions 2010, Table 8.1, to the 18.6-year nodal term,
/// removed.
///
/// # Errors
///
/// As [`ut1_moment`].
pub fn ut1s_iers2010_line(ut1_unix_seconds_given: f64) -> Answer<String> {
    regularized_ut1_line(
        ut1_unix_seconds_given,
        ut_variants::ut1s_minus_ut1_iers2010,
        ut_variants::ut1s_iers2010,
    )
}

/// The line of `hc_zonal_tide_ut1_effect`: the effect on UT1, in seconds,
/// of the zonal tides of IERS Conventions 2010, Table 8.1, whose period is
/// under `period_limit_days`, at a UT1 reading, by
/// [`hc_astro::ut_variants::zonal_tide_ut1_effect`] with
/// [`hc_astro::julian_centuries`] of the moment. 35 days is UT1R's set and
/// an infinite limit UT1S's; the regularised reading is UT1 *minus* this.
///
/// # Errors
///
/// As [`ut1_moment`], and [`Refusal::OutOfRange`] for a limit that is not
/// positive, NaN included.
pub fn zonal_tide_ut1_effect_line(
    ut1_unix_seconds_given: f64,
    period_limit_days: f64,
) -> Answer<String> {
    let moment = ut1_moment(ut1_unix_seconds_given)?;
    if period_limit_days.is_nan() || period_limit_days <= 0.0 {
        return Err(Refusal::OutOfRange);
    }
    number_line(Ok(ut_variants::zonal_tide_ut1_effect(
        hc_astro::julian_centuries(moment),
        period_limit_days,
    )))
}

/// The line of `hc_equation_of_time`: apparent solar time less mean solar
/// time at a Universal Time instant, in seconds, by
/// [`hc_astro::solar::equation_of_time`]: positive when a sundial is ahead
/// of the mean clock, up to about 16 minutes either way over the year.
///
/// # Errors
///
/// As [`moment_in_era`].
pub fn equation_of_time_line(universal_unix: i64) -> Answer<String> {
    let moment = moment_in_era(universal_unix)?;
    number_line(Ok(
        hc_astro::solar::equation_of_time(moment) * SECONDS_PER_DAY_F64
    ))
}

/// The value of `hc_solar_noon`: the Sun's upper transit of the local
/// meridian on a local day at a place, as whole POSIX seconds of Universal
/// Time, rounded down, by [`riseset::solar_noon`]. Every day has one,
/// under the midnight sun too.
///
/// # Errors
///
/// As [`day_in_era`].
pub fn solar_noon(fixed: i64, place: Location) -> Answer<i64> {
    let day = day_in_era(fixed)?;
    Ok(unix_from_moment(riseset::solar_noon(day, place)))
}

/// The value of `hc_solar_midnight`: the Sun's lower transit that opens a
/// local day at a place, half a day before its noon, as whole POSIX seconds
/// of Universal Time, rounded down, by [`riseset::solar_midnight`].
///
/// # Errors
///
/// As [`day_in_era`].
pub fn solar_midnight(fixed: i64, place: Location) -> Answer<i64> {
    let day = day_in_era(fixed)?;
    Ok(unix_from_moment(riseset::solar_midnight(day, place)))
}

/// The twilight a name selects: `civil`, the Sun's centre 6° below the
/// horizon; `nautical`, 12°; or `astronomical`, 18°; in any case.
///
/// # Errors
///
/// [`Refusal::Unknown`] for anything else.
pub fn twilight(name: &str) -> Answer<Twilight> {
    [
        ("civil", Twilight::Civil),
        ("nautical", Twilight::Nautical),
        ("astronomical", Twilight::Astronomical),
    ]
    .into_iter()
    .find(|(id, _)| hc_core::catalogue::matches(name, id))
    .map(|(_, twilight)| twilight)
    .ok_or(Refusal::Unknown)
}

/// How many columns the lines of [`dawn_line`] and [`dusk_line`] write:
/// the instant and the cells of [`missing_cells`].
pub const TWILIGHT_COLUMNS: usize = 1 + MISSING_COLUMNS;

/// The depression a twilight asks for, in whole arcminutes.
fn twilight_arcminutes(twilight: Twilight) -> u16 {
    (twilight.depression_degrees() * 60.0) as u16
}

/// The line of `hc_dawn`: the start of a named twilight on a local day at
/// a place, the moment the Sun's centre rises to the twilight's depression,
/// as whole POSIX seconds of Universal Time, rounded down, then the cells
/// of [`missing_cells`], which name the missing `depression` and its
/// arcminutes when the Sun does not cross it that morning. By
/// [`riseset::dawn`].
///
/// # Errors
///
/// [`Refusal::Unknown`] for a twilight [`twilight`] does not read, and the
/// range errors of [`day_in_era`] and [`location`].
pub fn dawn_line(twilight_name: &str, fixed: i64, place: Location) -> Answer<String> {
    let twilight = twilight(twilight_name)?;
    let day = day_in_era(fixed)?;
    let answer = riseset::dawn(day, place, twilight).ok_or(MissingSolarEvent::DawnDepression {
        day,
        arcminutes: twilight_arcminutes(twilight),
    });
    Ok(twilight_line(answer))
}

/// The line of `hc_dusk`: the end of a named twilight on a local day at a
/// place, the moment the Sun's centre sets to the twilight's depression,
/// in the columns of [`dawn_line`]. By [`riseset::dusk`].
///
/// # Errors
///
/// As [`dawn_line`].
pub fn dusk_line(twilight_name: &str, fixed: i64, place: Location) -> Answer<String> {
    let twilight = twilight(twilight_name)?;
    let day = day_in_era(fixed)?;
    let answer = riseset::dusk(day, place, twilight).ok_or(MissingSolarEvent::Depression {
        day,
        arcminutes: twilight_arcminutes(twilight),
    });
    Ok(twilight_line(answer))
}

/// A twilight crossing, or the depression it misses, as one line.
fn twilight_line(answer: Result<Moment, MissingSolarEvent>) -> String {
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    moment_or_missing(&mut line, answer);
    line.end();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::boundary::cells;

    /// ERFA's `t_era00`, which `hc-astro`'s own test reads:
    /// `eraEra00(2400000.5, 54388.0)` is 0.402 283 724 002 815 810 2 rad,
    /// JD 2 454 388.5 UT1, which is 1 192 406 400 s after 1970 UT1.
    #[test]
    fn the_rotation_angle_is_erfas() {
        let line = earth_rotation_angle_line(1_192_406_400.0).expect("in the era");
        let degrees: f64 = cells(&line)[0].parse().expect("a number");
        let expected = 0.402_283_724_002_815_8_f64.to_degrees();
        assert!((degrees - expected).abs() < 1e-8, "{degrees}");
        assert_eq!(ut1_moment(f64::NAN), Err(Refusal::OutOfRange));
        assert_eq!(
            gmst_iau1982_line(1e15).map(|_| ()),
            Err(Refusal::OutOfRange)
        );
    }

    #[test]
    fn a_missing_sunrise_is_named_not_numbered() {
        let tromso = location(69.6496, 18.9560, 0.0).expect("a place");
        // Noon UTC, 21 December 2024, in the polar night.
        let midwinter = 1_734_782_400;
        let line = solar_time_line("temporal", midwinter, tromso).expect("an answer");
        let cells = cells(&line);
        assert_eq!(cells.len(), 5);
        assert_eq!(cells[..2], ["", ""]);
        assert!(["sunrise", "sunset"].contains(&cells[2]), "{line}");
        assert_eq!(cells[4], "");
        assert_eq!(
            solar_time_line("babylonian", midwinter, tromso),
            Err(Refusal::Unknown)
        );
        assert_eq!(location(91.0, 0.0, 0.0), Err(Refusal::OutOfRange));
    }

    /// The Hong Kong Observatory's name for the USNO, in its traditional
    /// and simplified pages (`hko-astronomy-portal`), under `zh-TW` and
    /// `zh-Hans`; the IMCCE's under `fr` (`imcce-promenade-usno`); the
    /// English name, with `en`, under `ja`, which has no name for any of
    /// the three, and for the two horizons no locale names.
    #[cfg(feature = "i18n")]
    #[test]
    fn every_horizon_is_a_line_of_seven_cells_named_in_the_locale() {
        let text = horizons_lines("en");
        let rows: alloc::vec::Vec<alloc::vec::Vec<&str>> = text
            .lines()
            .map(|line| line.split('\t').collect())
            .collect();
        assert_eq!(rows.len(), horizon::HORIZONS.len());
        for (row, horizon) in rows.iter().zip(horizon::HORIZONS) {
            assert_eq!(row.len(), HORIZONS_COLUMNS);
            assert_eq!(row[0], horizon.id);
            assert_eq!(row[4], horizon.short_name);
            assert_eq!(row[5..], [horizon.english_name, "en"]);
        }
        let usno = |tag: &str| {
            let text = horizons_lines(tag);
            let line = text
                .lines()
                .find(|line| line.starts_with("usno\t"))
                .expect("usno");
            let cells: alloc::vec::Vec<&str> = line.split('\t').collect();
            (String::from(cells[5]), String::from(cells[6]))
        };
        assert_eq!(
            usno("zh-TW"),
            ("美國海軍天文氣象台".into(), "zh-Hant".into())
        );
        assert_eq!(
            usno("zh-Hans"),
            ("美国海军天文气象台".into(), "zh-Hans".into())
        );
        assert_eq!(
            usno("fr"),
            ("Observatoire naval de Washington D.C.".into(), "fr".into())
        );
        assert_eq!(
            usno("ja"),
            ("US Naval Observatory, sea level".into(), "en".into())
        );
        assert_eq!(usno("not a tag"), usno("ja"));
        for line in horizons_lines("zh-Hant").lines() {
            if !line.starts_with("usno\t") {
                assert!(line.ends_with("\ten"), "{line}");
            }
        }
        assert_eq!(rows[0][0], "geometric-dip");
        assert_eq!(horizon("USNO").map(|horizon| horizon.id), Ok("usno"));
        assert_eq!(horizon("").map(|horizon| horizon.id), Err(Refusal::Unknown));
    }

    /// The USNO's "Complete Sun and Moon Data for One Day" for Jerusalem,
    /// 31.78° N, 35.24° E, on 2024-01-01 (`usno-api-rstt`, as
    /// `hc_astro::riseset`'s test reads it): sunrise 06:39 and sunset
    /// 16:46 at UT+2, 04:39 and 14:46 UTC, POSIX 1 704 083 940 and
    /// 1 704 120 360, to the minute. The service takes no height, and its
    /// own horizon gives its minutes at 740 m; the book's horizon, lowered
    /// by 61′ there, rises about five minutes sooner.
    #[test]
    fn the_usno_horizon_rises_and_sets_on_the_usnos_minutes() {
        let jerusalem = location(31.78, 35.24, 740.0).expect("a place");
        let day = 738_886;
        for (line, published) in [
            (sunrise_line("usno", day, jerusalem), 1_704_083_940),
            (sunset_line("usno", day, jerusalem), 1_704_120_360),
        ] {
            let line = line.expect("a crossing");
            let cells = cells(&line);
            assert_eq!(cells.len(), 5);
            let instant: i64 = cells[0].parse().expect("an instant");
            assert!((instant - published).abs() <= 31, "{instant}");
            assert_eq!(cells[1..4], ["", "", ""]);
            let altitude: f64 = cells[4].parse().expect("an altitude");
            assert!((altitude + 50.0 / 60.0).abs() < 1e-9, "{altitude}");
        }
        let book = sunrise_line("calendrical-calculations", day, jerusalem).expect("a sunrise");
        let usno = sunrise_line("usno", day, jerusalem).expect("a sunrise");
        let earlier: i64 = cells(&usno)[0].parse::<i64>().expect("an instant")
            - cells(&book)[0].parse::<i64>().expect("an instant");
        assert!((270..330).contains(&earlier), "{earlier} s");
        assert_eq!(sunrise_line("naoj", day, jerusalem), Err(Refusal::Unknown));
    }

    #[test]
    fn a_polar_night_names_the_missing_sunrise() {
        let tromso = location(69.6496, 18.9560, 0.0).expect("a place");
        // 21 December 2024.
        let line = sunrise_line("geometric-dip", 739_241, tromso).expect("an answer");
        let cells = cells(&line);
        assert_eq!(cells[..4], ["", "sunrise", "739241", ""]);
        assert_eq!(
            sunset_line("usno", 2_000_000, tromso),
            Err(Refusal::OutOfRange)
        );
    }

    /// The Julian Date of a proleptic Gregorian date and a time of day.
    fn julian_date(year: i64, month: u8, day: u8, seconds: f64) -> f64 {
        let fixed = hc_calendar::gregorian::to_fixed(year, month, day).expect("a date");
        fixed.0 as f64 + JULIAN_DATE_OF_RD_ZERO + seconds / 86_400.0
    }

    /// Warren's row for 29 February 1992, 03:15:56.2, and the J2000
    /// position 12h 56m 27.4s, +42° 10′ 17″, in the IDL Astronomy
    /// Library's `helio_jd`: HJD − JD is 350.9 s.
    #[test]
    fn hjd_tt_is_the_idl_tables_correction_in_days() {
        let date = julian_date(1992, 2, 29, 3.0 * 3_600.0 + 15.0 * 60.0 + 56.2);
        let (alpha, delta) = (
            15.0 * (12.0 + 56.0 / 60.0 + 27.4 / 3_600.0),
            42.0 + 10.0 / 60.0 + 17.0 / 3_600.0,
        );
        let line = hjd_tt_line(date, alpha, delta).expect("in the era");
        let row = cells(&line);
        assert_eq!(row.len(), HJD_TT_COLUMNS);
        let hjd: f64 = row[0].parse().expect("a date");
        let correction: f64 = row[1].parse().expect("seconds");
        assert!((correction - 350.9).abs() < 0.1, "{correction}");
        assert!(((hjd - date) * 86_400.0 - correction).abs() < 1e-3);
        assert_eq!(hjd_tt_line(f64::NAN, 0.0, 0.0), Err(Refusal::OutOfRange));
        assert_eq!(hjd_tt_line(date, 361.0, 0.0), Err(Refusal::OutOfRange));
        assert_eq!(hjd_tt_line(date, 0.0, -90.5), Err(Refusal::OutOfRange));
        assert_eq!(
            hjd_tt_line(julian_date(3001, 1, 1, 0.0), 0.0, 0.0),
            Err(Refusal::OutOfRange)
        );
    }

    /// On 1 January 2017, TAI − UTC was 37 s, so TT − UTC is 69.184 s, and
    /// HJD_TT and HJD_UTC of one event differ by it.
    #[test]
    fn hjd_utc_takes_tt_minus_utc_from_the_leap_second_table() {
        let utc = julian_date(2017, 1, 1, 0.0);
        let line = hjd_utc_line(utc, 90.0, 23.4, true).expect("in the table");
        let row = cells(&line);
        assert_eq!(row.len(), HJD_UTC_COLUMNS);
        let tt_minus_utc: f64 = row[2].parse().expect("seconds");
        assert!((tt_minus_utc - 69.184).abs() < 1e-9, "{tt_minus_utc}");
        let hjd_utc: f64 = row[0].parse().expect("a date");
        let tt = hjd_tt_line(utc + tt_minus_utc / 86_400.0, 90.0, 23.4).expect("in the era");
        let hjd_tt: f64 = cells(&tt)[0].parse().expect("a date");
        assert!(((hjd_tt - hjd_utc) * 86_400.0 - 69.184).abs() < 1e-3);
        let before_utc = julian_date(1950, 1, 1, 0.0);
        assert_eq!(
            hjd_utc_line(before_utc, 90.0, 23.4, true),
            Err(Refusal::NoData)
        );
        let held = hjd_utc_line(before_utc, 90.0, 23.4, false).expect("held");
        assert_eq!(cells(&held)[2], "32.184");
    }

    /// The *Nautical Almanac* for 1924 (`nautical-almanac-1924`, as
    /// `hc-astro`'s test reads it) gives the lunar eclipse of 20 February
    /// 1924 at "February 20ᵈ 4ʰ 12ᵐ 25ˢ·7" G.M.T. reckoned from noon, 16:12
    /// civil; and the almanac for 1925 calls 1924 December 31, 12ʰ from
    /// noon 1925 January 1, 0ʰ.
    #[test]
    fn the_1924_almanacs_eclipse_is_twelve_hours_from_noon() {
        let day = hc_calendars_solar::gregorian::to_fixed(1924, 2, 20)
            .expect("a date")
            .0;
        let tenths = 700_000_000_000_000_000;
        let civil = 16 * 3_600 + 12 * 60 + 25;
        let astronomical = 4 * 3_600 + 12 * 60 + 25;
        let gmat = gmat_from_gmt_line(day, civil, tenths).expect("a reading");
        assert_eq!(
            cells(&gmat),
            [
                &*day.to_string(),
                &*astronomical.to_string(),
                "700000000000000000"
            ]
        );
        let gmt = gmt_from_gmat_line(day, astronomical, tenths).expect("a reading");
        assert_eq!(cells(&gmt)[..2], [&*day.to_string(), &*civil.to_string()]);
        let new_year = hc_calendars_solar::gregorian::to_fixed(1925, 1, 1)
            .expect("a date")
            .0;
        let change = gmat_from_gmt_line(new_year, 0, 0).expect("a reading");
        assert_eq!(cells(&change), [&*(new_year - 1).to_string(), "43200", "0"]);
        assert_eq!(
            cells(&gmat_from_gmt_line(new_year, 43_200, 0).expect("noon"))[1],
            "0"
        );
        // 23:59:60 has no reading twelve hours earlier, and GMAT reads no
        // second 60; seconds past a day and attoseconds past a second are
        // refused.
        assert_eq!(
            gmat_from_gmt_line(new_year, 86_400, 0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            gmt_from_gmat_line(new_year, 86_400, 0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            gmat_from_gmt_line(new_year, 86_401, 0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            gmt_from_gmat_line(new_year, 0, 1_000_000_000_000_000_000),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(gmat_from_gmt_line(i64::MIN, 0, 0), Err(Refusal::OutOfRange));
    }
}
