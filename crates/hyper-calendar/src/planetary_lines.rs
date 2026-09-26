//! The tab-separated lines the WebAssembly module and the C library write
//! about time on other bodies, written once.
//!
//! Everything here reads [`hc_planetary`]: Mars time from the Mars24
//! restatement of Allison and McEwen, the mission clocks, and the body
//! table's solar days. Each quantity is one function and one line format,
//! and a calendar added to `hc-planetary` later — Titan's, the Galilean
//! moons', another Martian one — is one more function and one more export
//! beside [`mars_time_line`], with its own columns, not a new column here
//! (`docs/policy.md` §5: competing conventions get separate identifiers).
//!
//! **The instant** is a POSIX timestamp in seconds, a double, so a page
//! can pass `Date.now() / 1000` as it is. It is read as UTC and carried to
//! TAI through the leap-second table with the last published offset held
//! into the future and UTC taken as TAI before 1961, as
//! [`hc_planetary::Mission::landing_instant`] reads a landing; the step
//! to TT is then the fixed 32.184 s.
//!
//! **The range rule** is Allison and McEwen's own: their series is stated
//! good over ±100 years of J2000 and is an extrapolation outside it, so
//! every function here answers only for an instant within
//! [`SPAN_DAYS`] TT days of J2000.0 — 1900-01-01T12:00 to 2100-01-01T12:00
//! TT — and refuses any other with [`Refusal::OutOfRange`] rather than
//! write numbers that would be fiction. The body table has no stated span
//! of its own, and its README calls a century of propagation already more
//! than the fact-sheet figures bear, so the same span holds for it.
//!
//! **No convention is invented.** A mission whose operators published no
//! sol numbering — Zhurong, in `hc-planetary`'s table — lists with its
//! convention cells empty, and asking for its sol is
//! [`Refusal::NoData`], although the crate carries a stated choice of its
//! own for it.

use alloc::string::String;
use core::fmt::Write;

use hc_core::math::floor;
use hc_core::unix::{self, LeapPolicy};
use hc_core::{ATTOS_PER_SEC, Instant, Tai, UnixTime};
use hc_planetary::bodies::{self, Body, BodyKind, EpochBasis};
use hc_planetary::clock::BodyClock;
use hc_planetary::mars::missions::{MISSIONS, Mission, SolConvention};
use hc_planetary::mars::{DarianCalendar, MarsMoment};
use hc_planetary::moon::COORDINATED_LUNAR_TIME_STATUS;

use crate::boundary::{Answer, Refusal, names, push_cell};

/// How far either side of J2000.0, in TT days, the planetary exports
/// answer: 100 Julian years, the span Allison and McEwen state their
/// series for.
pub const SPAN_DAYS: f64 = 36_525.0;

/// How many columns a line of [`mars_time_line`] has.
pub const MARS_TIME_COLUMNS: usize = 16;

/// How many columns a line of [`missions_lines`] has.
pub const MISSION_COLUMNS: usize = 11;

/// How many columns a line of [`bodies_lines`] has.
pub const BODY_COLUMNS: usize = 12;

/// How many columns a line of [`body_time_line`] has.
pub const BODY_TIME_COLUMNS: usize = 8;

/// What the last cell of a Mars time line names.
const MARS_SOURCE: &str = "NASA GISS Mars24, Algorithm and Worked Examples (Allison and McEwen \
     2000, revised 2015), MSD_MIDNIGHT_ADJUSTMENT 0.0009626; Mars year: Clancy et al. 2000, from \
     MARS_YEAR_1_START_MSD, this library's own Ls = 0 solution near 1955-04-11; Darian date: \
     Gangale, The Darian Calendar for Mars, at Airy-0";

/// Where a mission with a published clock comes from.
const MISSION_SOURCE: &str = "NASA GISS Mars24 Technical Notes, Lander Mission Times \
     (giss.nasa.gov/tools/mars24/help/notes.html); landing and site: Mars24 Mars Lander Missions \
     (giss.nasa.gov/tools/mars24/help/landers.html)";

/// Where a mission without one comes from.
const UNPUBLISHED_MISSION_SOURCE: &str = "NASA GISS Mars24 Mars Lander Missions \
     (giss.nasa.gov/tools/mars24/help/landers.html); no mission clock or sol numbering published";

/// The TAI instant of a POSIX timestamp in seconds, if it is finite and
/// lies within [`SPAN_DAYS`] of J2000.0.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a value that is not finite or lies outside
/// the span.
pub fn instant(unix_seconds: f64) -> Answer<Instant<Tai>> {
    // 10^11 seconds, some three thousand years, is far outside the span
    // and far inside an `i64`, so the cast below cannot saturate into an
    // answer.
    if !unix_seconds.is_finite() || unix_seconds.abs() > 1e11 {
        return Err(Refusal::OutOfRange);
    }
    let whole = floor(unix_seconds);
    let attos = ((unix_seconds - whole) * ATTOS_PER_SEC as f64) as u64;
    let unix = UnixTime::new(whole as i64, attos.min(ATTOS_PER_SEC - 1))
        .map_err(|_| Refusal::OutOfRange)?;
    let instant = unix::tai_from_unix(unix, LeapPolicy::Extrapolate)?;
    if MarsMoment::from_tai(instant).j2000_offset().abs() > SPAN_DAYS {
        return Err(Refusal::OutOfRange);
    }
    Ok(instant)
}

/// A longitude a caller passed, if it is finite.
fn longitude(degrees: f64) -> Answer<f64> {
    if degrees.is_finite() {
        Ok(degrees)
    } else {
        Err(Refusal::OutOfRange)
    }
}

/// Mars at an instant and an east longitude, as one line: the Mars Sol
/// Date; Coordinated Mars Time, local mean solar time and local true solar
/// time, each as `HH:MM:SS` on the 24-hour Martian clock and in decimal
/// Martian hours; the equation of time in Martian minutes; the areocentric
/// solar longitude `Ls` in degrees; the Mars year under the Clancy
/// convention; the Darian year, month, sol of the month, month name and
/// sol-of-week name at Airy-0; and the source.
///
/// The longitude is planetocentric, east-positive, in degrees, and wraps.
///
/// # Errors
///
/// As [`instant`], and [`Refusal::OutOfRange`] for a longitude that is not
/// finite.
pub fn mars_time_line(unix_seconds: f64, east_longitude_degrees: f64) -> Answer<String> {
    let moment = MarsMoment::from_tai(instant(unix_seconds)?);
    let east = longitude(east_longitude_degrees)?;
    let date = DarianCalendar
        .date_at(moment)
        .map_err(|_| Refusal::OutOfRange)?;
    let month = date.month_name().map_err(|_| Refusal::OutOfRange)?;
    let weekday = date.weekday_name().map_err(|_| Refusal::OutOfRange)?;
    let mtc = moment.coordinated_mars_time();
    let lmst = moment.local_mean_solar_time_east(east);
    let ltst = moment.local_true_solar_time_east(east);
    let mut out = String::new();
    let _ = write!(
        out,
        "{}\t{mtc}\t{}\t{lmst}\t{}\t{ltst}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t",
        moment.mars_sol_date(),
        mtc.decimal_hours(),
        lmst.decimal_hours(),
        ltst.decimal_hours(),
        moment.equation_of_time_minutes(),
        moment.solar_longitude(),
        moment.mars_year(),
        date.year,
        date.month,
        date.day,
    );
    push_cell(&mut out, month);
    out.push('\t');
    push_cell(&mut out, weekday);
    out.push('\t');
    push_cell(&mut out, MARS_SOURCE);
    out.push('\n');
    Ok(out)
}

/// The identifier of a name: lower case, with a hyphen for each space, so
/// `Viking 1` is `viking-1` and `Mars Pathfinder` is `mars-pathfinder`.
fn identifier(out: &mut String, name: &str) {
    for character in name.chars() {
        if character == ' ' {
            out.push('-');
        } else {
            out.extend(character.to_lowercase());
        }
    }
}

/// Whether `given` names a table entry by its identifier or its English
/// name, in any ASCII case.
fn names_entry(given: &str, name: &str) -> bool {
    let mut id = String::new();
    identifier(&mut id, name);
    names(given, &id) || names(given, name)
}

/// The mission `given` names, by identifier or by name.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a name the table does not carry.
pub fn mission(given: &str) -> Answer<&'static Mission> {
    MISSIONS
        .iter()
        .find(|mission| names_entry(given, mission.name))
        .ok_or(Refusal::Unknown)
}

/// The identifier a clock convention is written as.
const fn convention_id(convention: SolConvention) -> &'static str {
    match convention {
        SolConvention::LocalMeanSolarTime => "local-mean-solar-time",
        SolConvention::LocalTrueSolarTimeAtLanding => "local-true-solar-time-at-landing",
    }
}

/// Every surface mission, in landing order, one line each: the
/// identifier, the name, the landing instant as UTC text and as a POSIX
/// timestamp, the number of the landing sol, the clock's midnight
/// (`local-mean-solar-time`, or `local-true-solar-time-at-landing`, a true
/// midnight on the landing sol ticked at the mean rate), the clock
/// meridian's east longitude, the achieved site's east longitude, `1`
/// where the operators published the convention, the note and the
/// source. Where no convention was published the landing sol, the clock
/// and its meridian are empty.
#[must_use]
pub fn missions_lines() -> String {
    let mut out = String::new();
    for mission in MISSIONS {
        identifier(&mut out, mission.name);
        out.push('\t');
        push_cell(&mut out, mission.name);
        out.push('\t');
        push_cell(&mut out, mission.landing_utc);
        let _ = write!(out, "\t{}\t", mission.landing_unix_seconds);
        if mission.convention_is_published {
            let _ = write!(
                out,
                "{}\t{}\t{}",
                mission.first_sol,
                convention_id(mission.convention),
                mission.clock_east_longitude_degrees,
            );
        } else {
            out.push_str("\t\t");
        }
        let _ = write!(
            out,
            "\t{}\t{}\t",
            mission.site_east_longitude_degrees,
            u8::from(mission.convention_is_published),
        );
        push_cell(&mut out, mission.note);
        out.push('\t');
        push_cell(
            &mut out,
            if mission.convention_is_published {
                MISSION_SOURCE
            } else {
                UNPUBLISHED_MISSION_SOURCE
            },
        );
        out.push('\n');
    }
    out
}

/// The sol number of a mission at an instant, by the mission's own clock:
/// its meridian, its mean or true midnight, and its landing sol numbered 0
/// or 1.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a mission the table does not carry;
/// [`Refusal::NoData`] for one whose operators published no sol
/// numbering; [`Refusal::OutOfRange`] as [`instant`], and for an instant
/// before the midnight that began the landing sol, where the mission's
/// count does not reach.
pub fn mission_sol(given: &str, unix_seconds: f64) -> Answer<i64> {
    let mission = mission(given)?;
    if !mission.convention_is_published {
        return Err(Refusal::NoData);
    }
    let moment = MarsMoment::from_tai(instant(unix_seconds)?);
    let clock = mission.clock()?;
    if moment.mars_sol_date() < clock.epoch_mars_sol_date() {
        return Err(Refusal::OutOfRange);
    }
    Ok(clock.sol(moment))
}

/// The identifier a body kind is written as.
const fn kind_id(kind: BodyKind) -> &'static str {
    match kind {
        BodyKind::Star => "star",
        BodyKind::Planet => "planet",
        BodyKind::DwarfPlanet => "dwarf-planet",
        BodyKind::Moon => "moon",
    }
}

/// The identifier a clock zero point is written as.
const fn basis_id(basis: EpochBasis) -> &'static str {
    match basis {
        EpochBasis::Standard => "standard",
        EpochBasis::Convention => "convention",
    }
}

/// What stands where a body's timekeeping standard is still being drawn
/// up: the Moon's Coordinated Lunar Time status, and nothing for any other
/// body.
fn status(body: &Body) -> &'static str {
    if body.name == "Moon" {
        COORDINATED_LUNAR_TIME_STATUS
    } else {
        ""
    }
}

/// The body `given` names, by identifier or by name.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a body the table does not carry.
pub fn body(given: &str) -> Answer<&'static Body> {
    bodies::ALL
        .iter()
        .find(|body| names_entry(given, body.name))
        .ok_or(Refusal::Unknown)
}

/// Every body `hc-planetary` carries, outward from the Sun with each
/// planet's moons after it, one line each: the identifier, the name, the
/// kind (`star`, `planet`, `dwarf-planet`, `moon`), the identifier of the
/// body it orbits, the sidereal rotation period in hours (negative for a
/// retrograde rotator), the solar day in SI seconds, whether that day is
/// `measured` (Earth, Mars, the Moon) or `derived` from the rotation and
/// the heliocentric year, the year in local solar days, whether the
/// clock's zero point is a `standard` or a `convention` this library
/// declares, what the zero point is, the source, and the status of a
/// standard still being drawn up (the Moon's Coordinated Lunar Time).
/// The Sun, which has no solar day, has the three day cells empty.
#[must_use]
pub fn bodies_lines() -> String {
    let mut out = String::new();
    for body in bodies::ALL {
        identifier(&mut out, body.name);
        out.push('\t');
        push_cell(&mut out, body.name);
        let _ = write!(out, "\t{}\t", kind_id(body.kind));
        if let Some(primary) = body.primary {
            identifier(&mut out, primary);
        }
        let _ = write!(out, "\t{}\t", body.sidereal_rotation_hours);
        if let Some(seconds) = body.solar_day_seconds() {
            let origin = if body.measured_solar_day_seconds.is_some() {
                "measured"
            } else {
                "derived"
            };
            let _ = write!(out, "{seconds}\t{origin}");
        } else {
            out.push('\t');
        }
        out.push('\t');
        if let Some(year) = body.year_in_local_days() {
            let _ = write!(out, "{year}");
        }
        let _ = write!(out, "\t{}\t", basis_id(body.clock_epoch.basis));
        push_cell(&mut out, body.clock_epoch.note);
        out.push('\t');
        push_cell(&mut out, body.source);
        out.push('\t');
        push_cell(&mut out, status(body));
        out.push('\n');
    }
    out
}

/// Local mean solar time on a body at an instant and an east longitude,
/// as one line: the local day number, the fraction of it elapsed, the
/// reading as `HH:MM:SS` and in decimal local hours on a 24-hour face, the
/// solar day and the local hour in SI seconds, whether the zero point is a
/// `standard` or a `convention`, and what it is.
///
/// A retrograde rotator's longitude is handled by `hc-planetary`'s
/// [`BodyClock`]: east is east, and the Sun rises in the west.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a body the table does not carry;
/// [`Refusal::NoData`] for the Sun, which has no solar day; and as
/// [`mars_time_line`] for the instant and the longitude.
pub fn body_time_line(
    given: &str,
    unix_seconds: f64,
    east_longitude_degrees: f64,
) -> Answer<String> {
    let body = body(given)?;
    let clock = BodyClock::new(body).ok_or(Refusal::NoData)?;
    let instant = instant(unix_seconds)?;
    let east = longitude(east_longitude_degrees)?;
    let local = clock.at_east_longitude(instant, east);
    let mut out = String::new();
    let _ = write!(
        out,
        "{}\t{}\t{local}\t{}\t{}\t{}\t{}\t",
        local.day_number(),
        local.day_fraction(),
        local.decimal_hours(),
        local.solar_day_seconds(),
        local.local_hour_seconds(),
        basis_id(body.clock_epoch.basis),
    );
    push_cell(&mut out, body.clock_epoch.note);
    out.push('\n');
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    fn cells(line: &str) -> Vec<&str> {
        line.trim_end_matches('\n').split('\t').collect()
    }

    fn number(cell: &str) -> f64 {
        cell.parse().unwrap_or(f64::NAN)
    }

    #[test]
    fn every_line_has_every_column() {
        let mars = mars_time_line(947_116_800.0, 0.0).unwrap_or_default();
        assert_eq!(cells(&mars).len(), MARS_TIME_COLUMNS);
        let missions = missions_lines();
        assert_eq!(missions.lines().count(), MISSIONS.len());
        assert!(
            missions
                .lines()
                .all(|line| line.split('\t').count() == MISSION_COLUMNS)
        );
        let bodies = bodies_lines();
        assert_eq!(bodies.lines().count(), bodies::count());
        assert!(
            bodies
                .lines()
                .all(|line| line.split('\t').count() == BODY_COLUMNS)
        );
        let titan = body_time_line("titan", 947_116_800.0, 0.0).unwrap_or_default();
        assert_eq!(cells(&titan).len(), BODY_TIME_COLUMNS);
    }

    #[test]
    fn the_first_mars24_worked_example_reproduces() {
        // Mars24, worked example A: 2000-01-06T00:00:00Z at the prime
        // meridian is MTC 23:59:39, Ls 277.18758 and EOT -5.18774 degrees,
        // which is -20.75 Martian minutes; LTST 23:38:54.
        let line = mars_time_line(947_116_800.0, 0.0).unwrap_or_default();
        let row = cells(&line);
        assert_eq!(row[1], "23:59:39");
        assert_eq!(row[3], "23:59:39");
        assert_eq!(row[5], "23:38:54");
        let msd: f64 = row[0].parse().unwrap_or_default();
        assert!((msd - 44_795.999_760_4).abs() < 1e-6, "{msd}");
        let ls: f64 = row[8].parse().unwrap_or_default();
        assert!((ls - 277.187_58).abs() < 1e-5, "{ls}");
        let eot: f64 = row[7].parse().unwrap_or_default();
        assert!((eot - -5.187_74 * 4.0).abs() < 1e-3, "{eot}");
        // Mars Year 24 began in 1998; the Darian year is 183 more.
        assert_eq!(row[9], "24");
        assert_eq!(row[10], "207");
    }

    #[test]
    fn the_second_worked_example_is_true_midnight_at_spirits_planned_site() {
        // Mars24, worked example B: 2004-01-03T13:46:31Z at 184.702 degrees
        // west, 175.298 east, Spirit's planned site, is MTC 13:09:55, LMST
        // 00:51:07 and LTST 00:00:00, with Ls 327.32416. Mars24 prints
        // rounded seconds and the clock cells truncate, so the mean times
        // are held to a Martian second in decimal hours.
        let line = mars_time_line(1_073_137_591.0, 175.298).unwrap_or_default();
        let row = cells(&line);
        let hours = |h: f64, m: f64, s: f64| h + m / 60.0 + s / 3_600.0;
        assert!(
            (number(row[2]) - hours(13.0, 9.0, 55.0)).abs() < 1.0 / 3_600.0,
            "{}",
            row[2]
        );
        assert!(
            (number(row[4]) - hours(0.0, 51.0, 7.0)).abs() < 1.0 / 3_600.0,
            "{}",
            row[4]
        );
        assert_eq!(row[5], "00:00:00");
        assert!((number(row[8]) - 327.324_16).abs() < 1e-4, "{}", row[8]);
    }

    #[test]
    fn the_span_is_a_century_either_side_of_j2000() {
        // 1900-01-02 and 2099-12-31 are inside; 1899 and 2101 are not.
        assert!(mars_time_line(-2_208_902_400.0, 0.0).is_ok());
        assert!(mars_time_line(4_102_358_400.0, 0.0).is_ok());
        for outside in [-2_240_524_800.0, 4_133_980_800.0, f64::NAN, f64::INFINITY] {
            assert_eq!(mars_time_line(outside, 0.0), Err(Refusal::OutOfRange));
        }
        assert_eq!(
            mars_time_line(947_116_800.0, f64::NAN),
            Err(Refusal::OutOfRange)
        );
    }

    #[test]
    fn a_mission_sol_follows_the_missions_own_clock() {
        // Curiosity landed on its sol 0, and its sol 1000 fell within
        // 2015-05-30 UTC and its sol 4032 within 2023-12-09, two of the
        // published anchors `hc-planetary`'s own tests pin.
        assert_eq!(mission_sol("curiosity", 1_344_230_277.0), Ok(0));
        for (sol, day) in [(1_000, 1_432_944_000.0), (4_032, 1_702_080_000.0)] {
            let first = mission_sol("Curiosity", day).unwrap_or_default();
            let last = mission_sol("Curiosity", day + 86_399.0).unwrap_or_default();
            assert!((first..=last).contains(&sol), "{first}..={last}");
        }
        // Viking 1's clock ran from true solar midnight: sol 2243 fell
        // within 1982-11-11.
        let first = mission_sol("viking-1", 405_820_800.0).unwrap_or_default();
        let last = mission_sol("VIKING 1", 405_820_800.0 + 86_399.0).unwrap_or_default();
        assert!((first..=last).contains(&2_243), "{first}..={last}");
        // Pathfinder and Spirit numbered the landing sol 1.
        assert_eq!(mission_sol("mars-pathfinder", 868_035_415.0), Ok(1));
        assert_eq!(mission_sol("spirit", 1_073_190_413.0), Ok(1));
        // Before the landing sol began, the count does not reach.
        assert_eq!(
            mission_sol("curiosity", 1_344_230_277.0 - 86_400.0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            mission_sol("zhurong", 1_700_000_000.0),
            Err(Refusal::NoData)
        );
        assert_eq!(
            mission_sol("beagle-2", 1_700_000_000.0),
            Err(Refusal::Unknown)
        );
    }

    #[test]
    fn an_unpublished_convention_is_listed_empty() {
        let text = missions_lines();
        let zhurong = text
            .lines()
            .map(cells)
            .find(|row| row[0] == "zhurong")
            .unwrap_or_default();
        assert_eq!(&zhurong[4..7], ["", "", ""]);
        assert_eq!(zhurong[8], "0");
        let viking = text
            .lines()
            .map(cells)
            .find(|row| row[0] == "viking-1")
            .unwrap_or_default();
        assert_eq!(viking[4], "0");
        assert_eq!(viking[5], "local-true-solar-time-at-landing");
        assert_eq!(viking[8], "1");
    }

    #[test]
    fn the_body_table_names_its_measured_days_and_the_lunar_status() {
        let text = bodies_lines();
        let rows: Vec<Vec<&str>> = text.lines().map(cells).collect();
        let sun = &rows[0];
        assert_eq!(sun[0], "sun");
        assert_eq!(&sun[5..8], ["", "", ""]);
        let moon = rows.iter().find(|row| row[0] == "moon");
        assert_eq!(moon.map(|row| row[6]), Some("measured"));
        assert_eq!(moon.map(|row| row[3]), Some("earth"));
        assert_eq!(moon.map(|row| row[11]), Some(COORDINATED_LUNAR_TIME_STATUS));
        let titan = rows.iter().find(|row| row[0] == "titan");
        assert_eq!(titan.map(|row| row[6]), Some("derived"));
        assert_eq!(titan.map(|row| row[11]), Some(""));
        let mars = rows.iter().find(|row| row[0] == "mars");
        assert_eq!(mars.map(|row| row[8]), Some("standard"));
    }

    #[test]
    fn the_generic_mars_clock_agrees_with_coordinated_mars_time() {
        let mars = body_time_line("Mars", 947_116_800.0, 0.0).unwrap_or_default();
        assert_eq!(cells(&mars)[0], "44795");
        assert_eq!(cells(&mars)[2], "23:59:39");
        assert_eq!(
            body_time_line("sun", 947_116_800.0, 0.0),
            Err(Refusal::NoData)
        );
        assert_eq!(
            body_time_line("arrakis", 947_116_800.0, 0.0),
            Err(Refusal::Unknown)
        );
    }
}
