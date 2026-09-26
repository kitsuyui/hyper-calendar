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

use alloc::string::String;
use core::fmt::Write;

use hc_astro::earth::{earth_rotation_angle, mean_sidereal_time, mean_sidereal_time_iau2006};
use hc_astro::riseset::Location;
use hc_astro::solar_time::{self, MissingSolarEvent};
use hc_astro::ut_variants::ut2_minus_ut1;
use hc_calendar::Rd;
use hc_calendar::fixed::{Moment, RD_OF_UNIX_EPOCH};
use hc_calendar::gregorian::new_year;
use hc_core::math::floor;

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
    Ok(mean_sidereal_time(ut1_moment(ut1_unix_seconds)?))
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
pub const SOLAR_EVENTS: [&str; 5] = [
    "asr-shafii",
    "asr-hanafi",
    "jewish-dusk-vilna-gaon",
    "jewish-sabbath-ends-cohn",
    "italian-zero-hour",
];

/// The three cells naming a missing solar event: what is missing
/// (`sunrise`, `sunset`, `depression` or `no-noon-shadow`), the local day
/// it is missing on, and the depression sought in arcminutes, or empty.
fn push_missing(out: &mut String, missing: MissingSolarEvent) {
    let (name, day, arcminutes) = match missing {
        MissingSolarEvent::Sunrise(day) => ("sunrise", day, None),
        MissingSolarEvent::Sunset(day) => ("sunset", day, None),
        MissingSolarEvent::Depression { day, arcminutes } => ("depression", day, Some(arcminutes)),
        MissingSolarEvent::NoNoonShadow(day) => ("no-noon-shadow", day, None),
    };
    let _ = write!(out, "{name}\t{}\t", day.0);
    if let Some(arcminutes) = arcminutes {
        let _ = write!(out, "{arcminutes}");
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
/// the three cells of a missing solar event, empty when the time exists.
/// The times are [`SOLAR_EVENTS`]: ʿaṣr by the Shafiʿi and the Hanafi
/// shadow rules, Jewish dusk at the Vilna Gaon's 4°40′, the end of the
/// Sabbath at Berthold Cohn's 7°5′, and the Italian zero hour. A time that
/// does not happen that day has its first cell empty and names the event.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a time not named, and the range errors of
/// [`day_in_era`] and [`location`].
pub fn solar_event_line(event: &str, fixed: i64, place: Location) -> Answer<String> {
    let day = day_in_era(fixed)?;
    let answer = if names(event, "asr-shafii") {
        solar_time::asr_shafii(day, place)
    } else if names(event, "asr-hanafi") {
        solar_time::asr_hanafi(day, place)
    } else if names(event, "jewish-dusk-vilna-gaon") {
        solar_time::jewish_dusk_vilna_gaon(day, place)
    } else if names(event, "jewish-sabbath-ends-cohn") {
        solar_time::jewish_sabbath_ends_cohn(day, place)
    } else if names(event, "italian-zero-hour") {
        solar_time::italian_zero_hour(day, place)
    } else {
        return Err(Refusal::Unknown);
    };
    let mut out = String::new();
    match answer {
        Ok(moment) => {
            let _ = write!(out, "{}\t\t\t", unix_from_moment(moment));
        }
        Err(missing) => {
            out.push('\t');
            push_missing(&mut out, missing);
        }
    }
    out.push('\n');
    Ok(out)
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
}
