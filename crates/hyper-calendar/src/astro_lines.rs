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
use core::fmt::Write;

use hc_astro::earth::{
    earth_rotation_angle, mean_sidereal_time_iau1982, mean_sidereal_time_iau2006,
};
use hc_astro::hjd::{self, Target, heliocentric_correction_seconds};
use hc_astro::horizon::{HORIZONS, Horizon};
use hc_astro::riseset::{self, Location};
use hc_astro::solar_time::{self, MissingSolarEvent};
use hc_astro::ut_variants::ut2_minus_ut1;
use hc_calendar::Rd;
use hc_calendar::fixed::{Moment, RD_OF_UNIX_EPOCH};
use hc_calendar::gregorian::new_year;
use hc_core::math::floor;
use hc_core::scale::TT_MINUS_TAI;
use hc_core::unix::{LeapPolicy, tai_minus_utc_at};

use crate::boundary::{Answer, Refusal, names};

/// The first proleptic Gregorian year the sky exports answer for.
pub const EARLIEST_YEAR: i64 = -1000;

/// The last proleptic Gregorian year the sky exports answer for.
pub const LATEST_YEAR: i64 = 3000;

/// The first fixed day of the era.
const FIRST_DAY: i64 = new_year(EARLIEST_YEAR).0;

/// The first fixed day after the era.
const END_DAY: i64 = new_year(LATEST_YEAR + 1).0;

/// Seconds in a day, as the astronomical series count them.
const SECONDS_PER_DAY: i64 = 86_400;

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
    let day = unix
        .div_euclid(SECONDS_PER_DAY)
        .checked_add(RD_OF_UNIX_EPOCH)
        .ok_or(Refusal::OutOfRange)?;
    day_in_era(day)?;
    let seconds = unix.rem_euclid(SECONDS_PER_DAY);
    Ok(Moment(day as f64 + seconds as f64 / SECONDS_PER_DAY as f64))
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
    let days = ut1_unix_seconds / SECONDS_PER_DAY as f64 + RD_OF_UNIX_EPOCH as f64;
    if !(FIRST_DAY as f64..END_DAY as f64).contains(&days) {
        return Err(Refusal::OutOfRange);
    }
    Ok(Moment(days))
}

/// The POSIX second a Universal Time moment falls in, rounded down, as
/// the sky exports write every instant.
#[must_use]
pub fn unix_from_moment(moment: Moment) -> i64 {
    floor((moment.0 - RD_OF_UNIX_EPOCH as f64) * SECONDS_PER_DAY as f64) as i64
}

/// One number as a line.
fn number_line(value: Answer<f64>) -> Answer<String> {
    Ok(alloc::format!("{}\n", value?))
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

/// The clocks `hc_solar_time` reads, by name.
pub const SOLAR_TIMES: [&str; 4] = ["local-mean", "local-apparent", "temporal", "italian"];

/// The times of day `hc_solar_event` answers, by name.
pub const SOLAR_EVENTS: [&str; 9] = [
    "asr-shafii",
    "asr-hanafi",
    "jewish-dusk-vilna-gaon",
    "jewish-sabbath-ends-cohn",
    "italian-zero-hour",
    "japanese-dawn-kansei",
    "japanese-dusk-kansei",
    "japanese-dawn-naoj",
    "japanese-dusk-naoj",
];

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
/// [`push_missing_cells`]'s fourth cell.
fn push_missing(out: &mut String, missing: MissingSolarEvent) {
    let (name, day) = match missing {
        MissingSolarEvent::Sunrise(day) => ("sunrise", day),
        MissingSolarEvent::Sunset(day) => ("sunset", day),
        MissingSolarEvent::Depression { day, .. }
        | MissingSolarEvent::DawnDepression { day, .. }
        | MissingSolarEvent::Twilight { day, .. } => ("depression", day),
        MissingSolarEvent::NoNoonShadow(day) => ("no-noon-shadow", day),
    };
    let _ = write!(out, "{name}\t{}\t", day.0);
    if let Some(arcseconds) = missing_arcseconds(missing)
        && arcseconds % 60 == 0
    {
        let _ = write!(out, "{}", arcseconds / 60);
    }
}

/// How many cells [`push_missing_cells`] writes.
pub const MISSING_COLUMNS: usize = 4;

/// The four cells naming a missing solar event, or four empty ones for
/// `None`: the three of `hc_solar_time`'s line and the depression sought
/// in arcseconds, which is the one exact figure for a depression that is
/// not a whole number of arcminutes.
pub fn push_missing_cells(out: &mut String, missing: Option<MissingSolarEvent>) {
    match missing {
        None => out.push_str("\t\t\t"),
        Some(missing) => {
            push_missing(out, missing);
            out.push('\t');
            if let Some(arcseconds) = missing_arcseconds(missing) {
                let _ = write!(out, "{arcseconds}");
            }
        }
    }
}

/// An instant, or the missing solar event it needs: the whole POSIX
/// seconds of Universal Time, rounded down, and the four cells of
/// [`push_missing_cells`], the first cell empty when the time does not
/// happen.
pub fn push_moment_or_missing(out: &mut String, answer: Result<Moment, MissingSolarEvent>) {
    match answer {
        Ok(moment) => {
            let _ = write!(out, "{}\t", unix_from_moment(moment));
            push_missing_cells(out, None);
        }
        Err(missing) => {
            out.push('\t');
            push_missing_cells(out, Some(missing));
        }
    }
}

/// The line of `hc_solar_time`: a clock's reading at a Universal Time
/// instant at a place, as the local date's fixed day and the hours into
/// it, then the three cells of a missing solar event, empty when the
/// reading exists. The clocks are [`SOLAR_TIMES`]: `local-mean`,
/// `local-apparent` (the sundial), `temporal` (6 at sunrise, 18 at
/// sunset) and `italian` (hours since the zero hour of the evening
/// before). A reading that needs a solar event that does not happen has
/// its first two cells empty and names the event.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a clock not named, and the range errors of
/// [`moment_in_era`] and [`location`].
pub fn solar_time_line(clock: &str, universal_unix: i64, place: Location) -> Answer<String> {
    let universal = moment_in_era(universal_unix)?;
    let clock_reading = |moment: Moment| (moment.day(), moment.day_fraction() * 24.0);
    let reading = if names(clock, "local-mean") {
        Ok(clock_reading(solar_time::local_mean_time(universal, place)))
    } else if names(clock, "local-apparent") {
        Ok(clock_reading(solar_time::local_apparent_time(
            universal, place,
        )))
    } else if names(clock, "temporal") {
        solar_time::temporal_time(universal, place).map(clock_reading)
    } else if names(clock, "italian") {
        // The reading's own date and hours: the hours can run a minute or
        // two past 24 before the next zero hour.
        solar_time::italian_time(universal, place).map(|reading| (reading.day, reading.hours))
    } else {
        return Err(Refusal::Unknown);
    };
    let mut out = String::new();
    match reading {
        Ok((day, hours)) => {
            let _ = write!(out, "{}\t{hours}\t\t\t", day.0);
        }
        Err(missing) => {
            out.push_str("\t\t");
            push_missing(&mut out, missing);
        }
    }
    out.push('\n');
    Ok(out)
}

/// The line of `hc_solar_event`: a named time of day on a local day at a
/// place, as whole POSIX seconds of Universal Time, rounded down, then
/// the four cells of [`push_missing_cells`] naming a missing solar event,
/// empty when the time exists. The times are [`SOLAR_EVENTS`]: ʿaṣr by
/// the Shafiʿi and the Hanafi shadow rules, Jewish dusk at the Vilna
/// Gaon's 4°40′, the end of the Sabbath at Berthold Cohn's 7°5′, the
/// Italian zero hour, and the Japanese dawn and dusk, 明け六つ and 暮れ六つ
/// by the 寛政暦's 7°21′41″ and the Observatory's 夜明 and 日暮 at
/// 7°21′40″. A time that does not happen that day has its first cell
/// empty and names the event.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a time not named, and the range errors of
/// [`day_in_era`] and [`location`].
pub fn solar_event_line(event: &str, fixed: i64, place: Location) -> Answer<String> {
    let day = day_in_era(fixed)?;
    let reckon: fn(Rd, Location) -> Result<Moment, MissingSolarEvent> =
        if names(event, "asr-shafii") {
            solar_time::asr_shafii
        } else if names(event, "asr-hanafi") {
            solar_time::asr_hanafi
        } else if names(event, "jewish-dusk-vilna-gaon") {
            solar_time::jewish_dusk_vilna_gaon
        } else if names(event, "jewish-sabbath-ends-cohn") {
            solar_time::jewish_sabbath_ends_cohn
        } else if names(event, "italian-zero-hour") {
            solar_time::italian_zero_hour
        } else if names(event, "japanese-dawn-kansei") {
            solar_time::japanese_dawn_kansei
        } else if names(event, "japanese-dusk-kansei") {
            solar_time::japanese_dusk_kansei
        } else if names(event, "japanese-dawn-naoj") {
            solar_time::japanese_dawn_naoj
        } else if names(event, "japanese-dusk-naoj") {
            solar_time::japanese_dusk_naoj
        } else {
            return Err(Refusal::Unknown);
        };
    let mut out = String::new();
    push_moment_or_missing(&mut out, reckon(day, place));
    out.push('\n');
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

/// The lines of `hc_horizons`: every horizon [`HORIZONS`] carries, one a
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
    for horizon in HORIZONS {
        let named = horizon_name(horizon, locale);
        for (index, cell) in [
            horizon.id,
            horizon.english_name,
            horizon.description,
            horizon.source,
            horizon.short_name,
            named.name,
            named.tag,
        ]
        .into_iter()
        .enumerate()
        {
            if index > 0 {
                out.push('\t');
            }
            crate::boundary::push_cell(&mut out, cell);
        }
        out.push('\n');
    }
    out
}

/// The horizon an identifier names, one of [`HORIZONS`], in any ASCII
/// case.
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other text, the empty string included: a
/// rising is measured against a horizon, so none is assumed.
pub fn horizon(given: &str) -> Answer<&'static Horizon> {
    HORIZONS
        .iter()
        .find(|horizon| names(given, horizon.id))
        .ok_or(Refusal::Unknown)
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
    match event(day, place, horizon) {
        Some(moment) => {
            let _ = write!(out, "{}\t\t\t", unix_from_moment(moment));
        }
        None => {
            let _ = write!(out, "\t{name}\t{}\t", day.0);
        }
    }
    let _ = writeln!(
        out,
        "\t{}",
        horizon.sunrise_altitude_degrees(place.elevation_metres)
    );
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
/// [`Refusal::Unknown`] for a horizon [`horizon`] does not name, and the
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

/// The Julian Date at which fixed day 0 begins.
const JULIAN_DATE_OF_RD_ZERO: f64 = 1_721_424.5;

/// The Julian Date at which the POSIX epoch, 1970-01-01 00:00, falls.
const JULIAN_DATE_OF_UNIX_EPOCH: f64 = 2_440_587.5;

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
fn julian_date_in_era(julian_date: f64) -> Answer<f64> {
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
    Ok(alloc::format!(
        "{}\t{}\n",
        hjd::hjd_tt(date, target),
        heliocentric_correction_seconds(date, target)
    ))
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
    let unix = floor((date - JULIAN_DATE_OF_UNIX_EPOCH) * SECONDS_PER_DAY as f64) as i64;
    let tt_minus_utc = TT_MINUS_TAI.as_secs_f64() + tai_minus_utc_at(unix, policy)?.as_secs_f64();
    let tt = date + tt_minus_utc / SECONDS_PER_DAY as f64;
    Ok(alloc::format!(
        "{}\t{}\t{tt_minus_utc}\n",
        hjd::hjd_utc(date, tt_minus_utc, target),
        heliocentric_correction_seconds(tt, target)
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cells(line: &str) -> alloc::vec::Vec<&str> {
        line.strip_suffix('\n')
            .expect("a line")
            .split('\t')
            .collect()
    }

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
        assert_eq!(rows.len(), HORIZONS.len());
        for (row, horizon) in rows.iter().zip(HORIZONS) {
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
}
