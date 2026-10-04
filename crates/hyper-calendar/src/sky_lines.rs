//! The tab-separated lines the WebAssembly module and the C library write
//! about the Sun and the Moon, written once.
//!
//! * The Sun and the Moon at an instant, [`sky_line`]: their positions,
//!   the Moon's phase, the new moons either side and ΔT, with the series
//!   each figure came from.
//! * The solar terms within a span, [`term_lines`], and the principal moon
//!   phases within one, [`phase_lines`], each as an instant.
//! * The decan the Sun is in at an instant, [`decan_line`]: the tropical
//!   sign, which of its three 10° faces, and the face's ruler, from
//!   [`hc_seasons::zodiac::decans`].
//!
//! Every instant is a POSIX timestamp read as Universal Time, and every one
//! written is a whole POSIX second, rounded down
//! ([`crate::astro_lines::unix_from_moment`]). The era is the sky layer's,
//! the years [`EARLIEST_YEAR`] to [`LATEST_YEAR`] over which `hc-astro`
//! states its series hold; outside it the answer is
//! [`Refusal::OutOfRange`] rather than an extrapolation.

use alloc::string::String;

use hc_astro::lunar::{
    MEAN_SYNODIC_MONTH, MoonPhase, lunar_distance, lunar_illuminated_fraction, lunar_latitude,
    lunar_longitude, lunar_phase, new_moon_at_or_after, new_moon_before, nth_moon_phase,
    nth_new_moon,
};
use hc_astro::solar::{solar_longitude, solar_longitude_after, solar_radius_vector};
use hc_astro::time::{DeltaTRegime, decimal_year, delta_t, delta_t_regime};
use hc_calendar::fixed::Moment;
use hc_core::duration::SECONDS_PER_DAY;
use hc_core::math::floor;
use hc_seasons::solar_terms::{DEGREES_PER_TERM, SolarTerm, TermOrder};
use hc_seasons::zodiac::decans::{decan_at_moment, degrees_into_decan};
use hc_seasons::zodiac::drekkana::{degrees_into_drekkana, drekkana_at_moment};

#[cfg(doc)]
use crate::astro_lines::{EARLIEST_YEAR, LATEST_YEAR};
use crate::astro_lines::{moment_in_era, unix_from_moment};
use crate::boundary::{Answer, Line, Refusal};

/// How many columns [`sky_line`] writes.
pub const SKY_COLUMNS: usize = 12;

/// How many columns [`term_lines`] and [`phase_lines`] write.
pub const EVENT_COLUMNS: usize = 4;

/// How many columns [`decan_line`] writes.
pub const DECAN_COLUMNS: usize = 7;

/// The longest span [`term_lines`] and [`phase_lines`] accept, in seconds:
/// 400 Julian years, about 4 950 lunations and 9 600 solar terms.
pub const MAX_SPAN_SECONDS: i64 = 400 * 31_557_600;

/// The series behind the Sun's columns, as `hc-astro` names them.
const SUN_SOURCE: &str = "Sun: VSOP87D Earth series truncated at 1e-7 (213 terms), Meeus ch. 25";

/// The series behind the Moon's columns.
const MOON_SOURCE: &str = "Moon: ELP-2000/82 abridged to Meeus tables 47.A and 47.B (60 terms), illumination Meeus ch. 48";

/// The series behind the new moons and the phase list.
const PHASES_SOURCE: &str = "phases: Meeus ch. 49 phase series";

/// The word a [`DeltaTRegime`] is written as.
const fn regime_name(regime: DeltaTRegime) -> &'static str {
    match regime {
        DeltaTRegime::Observed => "observed",
        DeltaTRegime::Predicted => "predicted",
        DeltaTRegime::Fitted => "fitted",
        DeltaTRegime::Extrapolated => "extrapolated",
    }
}

/// The source ΔT was answered from in a regime, as `hc-astro`'s README
/// names it.
const fn delta_t_source(regime: DeltaTRegime) -> &'static str {
    match regime {
        DeltaTRegime::Observed => "USNO deltat.data, observed",
        DeltaTRegime::Predicted => "USNO deltat.preds, predicted",
        DeltaTRegime::Fitted => "Espenak-Meeus polynomials, fitted",
        DeltaTRegime::Extrapolated => "Espenak-Meeus parabola, extrapolated",
    }
}

/// The word a [`MoonPhase`] is written as, here and in
/// [`crate::season_lines::principal_phases_in_month_lines`].
pub(crate) const fn phase_name(phase: MoonPhase) -> &'static str {
    match phase {
        MoonPhase::New => "new",
        MoonPhase::FirstQuarter => "first-quarter",
        MoonPhase::Full => "full",
        MoonPhase::LastQuarter => "last-quarter",
    }
}

/// The line of `hc_sky_at`: the Sun's apparent longitude in degrees, the
/// Earth–Sun distance in astronomical units, the Moon's apparent longitude
/// and latitude in degrees and its distance in kilometres, the Moon's
/// elongation from the Sun in degrees, the illuminated fraction of its
/// disc, the last new moon before the instant and the first at or after
/// it as POSIX seconds, ΔT in seconds, the regime ΔT was answered from and
/// the series each figure came from.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] outside [`EARLIEST_YEAR`]..=[`LATEST_YEAR`].
pub fn sky_line(unix: i64) -> Answer<String> {
    let moment = moment_in_era(unix)?;
    let regime = delta_t_regime(decimal_year(moment));
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(solar_longitude(moment))
        .value(solar_radius_vector(moment))
        .value(lunar_longitude(moment))
        .value(lunar_latitude(moment))
        .value(lunar_distance(moment))
        .value(lunar_phase(moment))
        .value(lunar_illuminated_fraction(moment))
        .value(unix_from_moment(new_moon_before(moment)))
        .value(unix_from_moment(new_moon_at_or_after(moment)))
        .value(delta_t(moment))
        .cell(regime_name(regime))
        .value(format_args!(
            "{SUN_SOURCE}; {MOON_SOURCE}; {PHASES_SOURCE}; delta T: {}",
            delta_t_source(regime)
        ));
    line.end();
    Ok(out)
}

/// The moment a half-open span `[from, to)` of POSIX seconds begins at, or
/// `None` for an empty span.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for an end outside the era or a span longer
/// than [`MAX_SPAN_SECONDS`].
fn span(from: i64, to: i64) -> Answer<Option<Moment>> {
    let start = moment_in_era(from)?;
    if to <= from {
        return Ok(None);
    }
    moment_in_era(to - 1)?;
    if to - from > MAX_SPAN_SECONDS {
        return Err(Refusal::OutOfRange);
    }
    Ok(Some(start))
}

/// The lines of `hc_solar_terms_between`: every solar term in
/// `[from, to)`, one line each, in time order, with the Sun's longitude
/// that defines the term, the instant, and the term's name in traditional
/// Chinese and in Japanese.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for an end outside the era or a span longer
/// than [`MAX_SPAN_SECONDS`].
pub fn term_lines(from: i64, to: i64) -> Answer<String> {
    let mut out = String::new();
    let Some(start) = span(from, to)? else {
        return Ok(out);
    };
    // The term in effect at the start; the first line is the one after
    // it. A longitude that rounds to 360° names 春分 again.
    let index = floor(solar_longitude(start) / DEGREES_PER_TERM) as u8;
    let mut term = SolarTerm::from_index(TermOrder::SpringEquinoxFirst, index)
        .unwrap_or(SolarTerm::SPRING_EQUINOX);
    let mut moment = start;
    // The span is bounded, so the loop is; the cap is against a search
    // that fails to advance, which would otherwise spin.
    for _ in 0..(MAX_SPAN_SECONDS / (13 * SECONDS_PER_DAY)) {
        term = term.next();
        moment = solar_longitude_after(term.solar_longitude_degrees(), moment);
        let unix = unix_from_moment(moment);
        if unix >= to {
            break;
        }
        if unix < from {
            continue;
        }
        let mut line = Line::new(&mut out);
        line.value(term.solar_longitude_degrees())
            .value(unix)
            .cell(term.chinese_name())
            .cell(term.japanese_name());
        line.end();
    }
    Ok(out)
}

/// The lines of `hc_moon_phases_between`: every new moon, first quarter,
/// full moon and last quarter in `[from, to)`, one line each, in time
/// order, with the elongation that defines the phase, the instant, the
/// phase's name and an empty fourth column, so that the lines have the
/// shape of [`term_lines`]'.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for an end outside the era or a span longer
/// than [`MAX_SPAN_SECONDS`].
pub fn phase_lines(from: i64, to: i64) -> Answer<String> {
    let mut out = String::new();
    let Some(start) = span(from, to)? else {
        return Ok(out);
    };
    // The lunation containing the start: seeded from the mean synodic
    // month, which is never more than a lunation out, then walked.
    let mut lunation = floor((start.0 - nth_new_moon(0).0) / MEAN_SYNODIC_MONTH) as i64;
    for _ in 0..8 {
        if nth_new_moon(lunation).0 <= start.0 {
            break;
        }
        lunation -= 1;
    }
    for _ in 0..8 {
        if nth_new_moon(lunation + 1).0 > start.0 {
            break;
        }
        lunation += 1;
    }
    'lunations: for _ in 0..(MAX_SPAN_SECONDS / (29 * SECONDS_PER_DAY)) {
        for phase in [
            MoonPhase::New,
            MoonPhase::FirstQuarter,
            MoonPhase::Full,
            MoonPhase::LastQuarter,
        ] {
            let unix = unix_from_moment(nth_moon_phase(lunation, phase));
            if unix >= to {
                break 'lunations;
            }
            if unix < from {
                continue;
            }
            let mut line = Line::new(&mut out);
            line.value(phase.elongation_degrees())
                .value(unix)
                .cell(phase_name(phase))
                .empty();
            line.end();
        }
        lunation += 1;
    }
    Ok(out)
}

/// The line of `hc_decan_at`: the tropical sign the Sun is in at an
/// instant, 1 for Aries through 12 for Pisces, its identifier (`aries`)
/// and its English name; which
/// of the sign's three decans it is in, 1 to 3; the decan's ruler by
/// al-Bīrūnī's table, as its identifier and its English name; and how far
/// into the decan the Sun is, in degrees from 0 up to 10. The sign and the
/// decan are read from the Sun's apparent longitude, so their boundaries
/// are those of the solar terms.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] outside [`EARLIEST_YEAR`]..=[`LATEST_YEAR`].
pub fn decan_line(unix: i64) -> Answer<String> {
    let moment = moment_in_era(unix)?;
    let decan = decan_at_moment(moment);
    let sign = decan.sign();
    let ruler = decan.ruler();
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(sign.index() + 1)
        .cell(sign.id())
        .cell(sign.english_name())
        .value(decan.part())
        .cell(ruler.id)
        .cell(ruler.english_name())
        .value(degrees_into_decan(moment));
    line.end();
    Ok(out)
}

/// How many columns [`drekkana_line`] writes.
pub const DREKKANA_COLUMNS: usize = 10;

/// The line of `hc_drekkana_at`: the Hindu third of a sidereal sign the
/// Sun is in at an instant, in the zodiac of an ayanāṃśa, beside
/// [`decan_line`]'s tropical decan — the sidereal sign, 1 for Meṣa through
/// 12 for Mīna, its identifier (`kanya`) and its Sanskrit name (`Kanyā`); which of its three drekkāṇas, 1 to
/// 3; the drekkāṇa's lord, the planet that rules the sign it is given to,
/// as its identifier and its English name; how far into the drekkāṇa the
/// Sun is, in degrees from 0 up to 10; the lord's sign, the sign itself,
/// the fifth or the ninth from it, by its identifier and its Sanskrit
/// name; and the ayanāṃśa's
/// identifier.
///
/// # Errors
///
/// [`Refusal::Unknown`] for an ayanāṃśa
/// [`hc_seasons::zodiac::Ayanamsa::by_id`] does not name, and
/// [`Refusal::OutOfRange`] outside
/// [`EARLIEST_YEAR`]..=[`LATEST_YEAR`].
pub fn drekkana_line(unix: i64, ayanamsa: &str) -> Answer<String> {
    let ayanamsa = hc_seasons::zodiac::Ayanamsa::by_id(ayanamsa).ok_or(Refusal::Unknown)?;
    let moment = moment_in_era(unix)?;
    let drekkana = drekkana_at_moment(moment, ayanamsa);
    let sign = drekkana.sign();
    let lord = drekkana.lord();
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    let lord_sign = drekkana.lord_sign();
    line.value(sign.index() + 1)
        .cell(sign.id())
        .cell(sign.sanskrit_name())
        .value(drekkana.part())
        .cell(lord.id)
        .cell(lord.english_name())
        .value(degrees_into_drekkana(moment, ayanamsa))
        .cell(lord_sign.id())
        .cell(lord_sign.sanskrit_name())
        .cell(ayanamsa.id());
    line.end();
    Ok(out)
}

/// How many columns each line of [`planetary_hour_line`] and
/// [`planetary_hours_of_day_lines`] writes.
#[cfg(feature = "i18n")]
pub const PLANETARY_HOUR_COLUMNS: usize = 8 + crate::astro_lines::MISSING_COLUMNS;

/// One line of a planetary hour: the day, the hour, its ruler named in a
/// locale, whether it is of the daylight, and when it begins and ends.
#[cfg(feature = "i18n")]
fn push_planetary_hour(
    out: &mut String,
    day: hc_calendar::Rd,
    hour: u8,
    ruler: hc_seasons::zodiac::RulingPlanet,
    place: hc_astro::Location,
    locale: &str,
) {
    use hc_seasons::planetary_hours::planetary_hour_start;
    let mut line = Line::new(out);
    line.value(day.0).value(hour).cell(ruler.id);
    crate::boundary::reckoning_name(&mut line, locale, hc_i18n::reckonings::PLANET, ruler.id);
    line.flag(hour <= 12);
    let start = planetary_hour_start(day, hour, place);
    let end = planetary_hour_start(day, hour + 1, place);
    match (start, end) {
        (Some(Ok(start)), Some(Ok(end))) => {
            line.value(unix_from_moment(start))
                .value(unix_from_moment(end));
            crate::astro_lines::missing_cells(&mut line, None);
        }
        (Some(Err(missing)), _) | (_, Some(Err(missing))) => {
            line.empties(2);
            crate::astro_lines::missing_cells(&mut line, Some(missing));
        }
        // Both hours are from 1 to 25, which `planetary_hour_start` answers.
        _ => {
            line.empties(2);
            crate::astro_lines::missing_cells(&mut line, None);
        }
    }
    line.end();
}

/// The line of `hc_planetary_hour`: the planetary hour at a Universal Time
/// instant and a place ([`hc_seasons::planetary_hours`]), the twelve
/// temporal hours of the daylight and the twelve of the night, each ruled
/// by a planet of the Chaldean order from the weekday's own at sunrise.
///
/// The cells: the fixed day of the sunrise the planetary day began at — the
/// hours after midnight and before sunrise belong to the day before; the
/// hour, 1 to 24 from sunrise, 1 to 12 of the daylight and 13 to 24 of the
/// night; the ruler's identifier, `sun` to `saturn`, and its name in the
/// locale and the tag that named it, by
/// [`hc_i18n::reckonings::name_or_fallback`]; `1` for an hour of the
/// daylight, else `0`; the hour's start and end, as whole POSIX seconds of
/// Universal Time, rounded down; and the four cells of a missing solar
/// event, as `hc_solar_event` writes them. Where a sunrise or sunset
/// around the instant does not happen, every cell but those four is empty.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for an instant outside the sky layer's era.
#[cfg(feature = "i18n")]
pub fn planetary_hour_line(
    universal_unix: i64,
    place: hc_astro::Location,
    locale: &str,
) -> Answer<String> {
    let moment = moment_in_era(universal_unix)?;
    let mut out = String::new();
    match hc_seasons::planetary_hours::planetary_hour(moment, place) {
        Ok(hour) => push_planetary_hour(&mut out, hour.day, hour.hour, hour.ruler, place, locale),
        Err(missing) => {
            let mut line = Line::new(&mut out);
            line.empties(PLANETARY_HOUR_COLUMNS - crate::astro_lines::MISSING_COLUMNS);
            crate::astro_lines::missing_cells(&mut line, Some(missing));
            line.end();
        }
    }
    Ok(out)
}

/// The lines of `hc_planetary_hours_of_day`: the twenty-four planetary
/// hours of the planetary day that begins at the sunrise of a fixed day at
/// a place, in order, each in [`planetary_hour_line`]'s columns. The
/// rulers are the weekday's alone; where the sunrise or sunset an hour is
/// counted from does not happen, its start and end are empty and the
/// missing event is named.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a day outside the sky layer's era.
#[cfg(feature = "i18n")]
pub fn planetary_hours_of_day_lines(
    fixed: i64,
    place: hc_astro::Location,
    locale: &str,
) -> Answer<String> {
    use hc_seasons::planetary_hours::{HOURS_PER_DAY, ruler_of_hour};
    let day = crate::astro_lines::day_in_era(fixed)?;
    let weekday = hc_calendar::Weekday::from_rd(day);
    let mut out = String::new();
    for hour in 1..=HOURS_PER_DAY {
        if let Some(ruler) = ruler_of_hour(weekday, hour) {
            push_planetary_hour(&mut out, day, hour, ruler, place, locale);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    /// A POSIX timestamp from a proleptic Gregorian UTC date and time.
    fn at(year: i64, month: u8, day: u8, hour: i64, minute: i64) -> i64 {
        let fixed = hc_calendars_solar::gregorian::to_fixed(year, month, day)
            .expect("a date")
            .0;
        (fixed - hc_calendar::fixed::RD_OF_UNIX_EPOCH) * SECONDS_PER_DAY
            + hour * 3_600
            + minute * 60
    }

    fn rows(text: &str) -> Vec<Vec<&str>> {
        text.lines()
            .map(|line| line.split('\t').collect())
            .collect()
    }

    /// The 暦要項 of the National Astronomical Observatory of Japan for
    /// 2026 puts 秋分 at 23 September 09:05 JST, 00:05 UTC, and the new
    /// moon of September at 11 September 12:27 JST, 03:27 UTC, both to the
    /// minute; the boundary crates' tests check the same instants through
    /// their exports.
    #[test]
    fn september_2026_is_where_the_almanac_puts_it() {
        let (from, to) = (at(2026, 9, 1, 0, 0), at(2026, 10, 1, 0, 0));
        let terms = term_lines(from, to).expect("in the era");
        let terms = rows(&terms);
        assert!(terms.iter().all(|row| row.len() == EVENT_COLUMNS));
        assert_eq!(terms[1][0], "180");
        assert_eq!(terms[1][2..], ["秋分", "秋分"]);
        let equinox: i64 = terms[1][1].parse().expect("a timestamp");
        assert!((equinox - at(2026, 9, 23, 0, 5)).abs() <= 90, "{equinox}");

        let phases = phase_lines(from, to).expect("in the era");
        let phases = rows(&phases);
        assert!(phases.iter().all(|row| row.len() == EVENT_COLUMNS));
        assert_eq!(phases[1][..1], ["0"]);
        assert_eq!(phases[1][2], "new");
        let sky = sky_line(at(2026, 9, 11, 3, 0)).expect("in the era");
        let sky: Vec<&str> = sky.trim_end_matches('\n').split('\t').collect();
        assert_eq!(sky.len(), SKY_COLUMNS);
        assert_eq!(sky[8], phases[1][1]);
        let new_moon: i64 = sky[8].parse().expect("a timestamp");
        assert!(
            (new_moon - at(2026, 9, 11, 3, 27)).abs() <= 90,
            "{new_moon}"
        );
    }

    /// At the NAOJ's 秋分 of 2026, 23 September 00:05 UTC, the Sun enters
    /// Libra, whose first face al-Bīrūnī gives to the Moon; an hour before,
    /// it is in the last face of Virgo, Mercury's.
    #[test]
    fn the_sun_enters_the_moons_face_of_libra_at_the_equinox() {
        let after = decan_line(at(2026, 9, 23, 1, 5)).expect("in the era");
        let after: Vec<&str> = after.trim_end_matches('\n').split('\t').collect();
        assert_eq!(after.len(), DECAN_COLUMNS);
        assert_eq!(after[..6], ["7", "libra", "Libra", "1", "moon", "Moon"]);
        let into: f64 = after[6].parse().expect("degrees");
        assert!((0.0..0.1).contains(&into), "{into}");
        let before = decan_line(at(2026, 9, 22, 23, 5)).expect("in the era");
        assert!(
            before.starts_with("6\tvirgo\tVirgo\t3\tmercury\tMercury\t9.9"),
            "{before}"
        );
        assert_eq!(decan_line(i64::MAX), Err(Refusal::OutOfRange));
    }

    #[test]
    fn the_era_and_the_span_are_enforced() {
        assert_eq!(sky_line(at(3001, 1, 1, 0, 0)), Err(Refusal::OutOfRange));
        assert_eq!(sky_line(i64::MIN), Err(Refusal::OutOfRange));
        assert!(sky_line(at(-1000, 1, 1, 0, 0)).is_ok());
        let from = at(2000, 1, 1, 0, 0);
        assert_eq!(term_lines(from, from), Ok(String::new()));
        assert_eq!(phase_lines(from, from - 1), Ok(String::new()));
        assert_eq!(
            term_lines(from, from + MAX_SPAN_SECONDS + 1),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            phase_lines(from, at(3001, 1, 1, 0, 1)),
            Err(Refusal::OutOfRange)
        );
    }

    /// Lilly's three hours of Monday 15 March 1646, Old Style, 25 March
    /// 1647 Gregorian, at London (`lilly-christian-astrology-1647`, as
    /// `hc-seasons`'s test reads it): at 9:30 local apparent time the
    /// fourth hour, Mars's; at 17:20 the twelfth, the Sun's; at 23:10 the
    /// eighteenth, Mars's. His Sun rises at 5:47, where the first hour,
    /// the Moon's, begins within three minutes.
    #[cfg(feature = "i18n")]
    #[test]
    fn lillys_hours_of_monday_are_the_planets_he_names() {
        use hc_astro::solar_time::universal_from_local_apparent_time;
        let london = hc_astro::Location::new(51.5, -0.1, 0.0);
        let day = hc_calendars_solar::gregorian::to_fixed(1647, 3, 25)
            .expect("a date")
            .0;
        let apparent = |hour: u8, minute: u8| {
            let moment = universal_from_local_apparent_time(
                Moment(day as f64 + (f64::from(hour) + f64::from(minute) / 60.0) / 24.0),
                london,
            );
            unix_from_moment(moment)
        };
        for ((hour, minute), number, ruler, name) in [
            ((9, 30), "4", "mars", "Mars"),
            ((17, 20), "12", "sun", "Sun"),
            ((23, 10), "18", "mars", "Mars"),
        ] {
            let line = planetary_hour_line(apparent(hour, minute), london, "fr").expect("in range");
            let row = rows(&line).remove(0);
            assert_eq!(row.len(), PLANETARY_HOUR_COLUMNS);
            assert_eq!(
                row[..6],
                [
                    &*day.to_string(),
                    number,
                    ruler,
                    name,
                    "en",
                    if number == "18" { "0" } else { "1" }
                ]
            );
            assert_eq!(row[8..], ["", "", "", ""]);
        }
        let text = planetary_hours_of_day_lines(day, london, "native").expect("in range");
        let hours = rows(&text);
        assert_eq!(hours.len(), 24);
        let rulers: Vec<&str> = hours[..5].iter().map(|row| row[2]).collect();
        assert_eq!(rulers, ["moon", "saturn", "jupiter", "mars", "sun"]);
        let first: i64 = hours[0][6].parse().expect("an instant");
        assert!((first - apparent(5, 47)).abs() < 180, "{first}");
        for pair in hours.windows(2) {
            assert_eq!(pair[0][7], pair[1][6]);
        }
        // Above the polar circle at midsummer there is no sunset: the
        // hour at noon is only the missing event.
        let tromso = hc_astro::Location::new(69.65, 18.96, 0.0);
        let midsummer = at(2024, 6, 21, 12, 0);
        let polar = planetary_hour_line(midsummer, tromso, "en").expect("in range");
        let polar = rows(&polar).remove(0);
        assert_eq!(polar.len(), PLANETARY_HOUR_COLUMNS);
        assert!(polar[..8].iter().all(|cell| cell.is_empty()), "{polar:?}");
        assert!(!polar[8].is_empty());
        assert_eq!(
            planetary_hour_line(i64::MIN, london, "en"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            planetary_hours_of_day_lines(i64::MAX, london, "en"),
            Err(Refusal::OutOfRange)
        );
    }
}
