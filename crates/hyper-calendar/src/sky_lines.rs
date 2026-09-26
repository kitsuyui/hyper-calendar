//! The tab-separated lines the WebAssembly module and the C library write
//! about the Sun and the Moon, written once.
//!
//! * The Sun and the Moon at an instant, [`sky_line`]: their positions,
//!   the Moon's phase, the new moons either side and ΔT, with the series
//!   each figure came from.
//! * The solar terms within a span, [`term_lines`], and the principal moon
//!   phases within one, [`phase_lines`], each as an instant.
//!
//! Every instant is a POSIX timestamp read as Universal Time, and every one
//! written is a whole POSIX second, rounded down
//! ([`crate::astro_lines::unix_from_moment`]). The era is the sky layer's,
//! the years [`EARLIEST_YEAR`] to [`LATEST_YEAR`] over which `hc-astro`
//! states its series hold; outside it the answer is
//! [`Refusal::OutOfRange`] rather than an extrapolation.

use alloc::format;
use alloc::string::String;
use core::fmt::Write;

use hc_astro::lunar::{
    MEAN_SYNODIC_MONTH, MoonPhase, lunar_distance, lunar_illuminated_fraction, lunar_latitude,
    lunar_longitude, lunar_phase, new_moon_at_or_after, new_moon_before, nth_moon_phase,
    nth_new_moon,
};
use hc_astro::solar::{solar_longitude, solar_longitude_after, solar_radius_vector};
use hc_astro::time::{DeltaTRegime, decimal_year, delta_t, delta_t_regime};
use hc_calendar::fixed::Moment;
use hc_core::math::floor;
use hc_seasons::solar_terms::{DEGREES_PER_TERM, SolarTerm, TermOrder};

#[cfg(doc)]
use crate::astro_lines::{EARLIEST_YEAR, LATEST_YEAR};
use crate::astro_lines::{moment_in_era, unix_from_moment};
use crate::boundary::{Answer, Refusal, push_cell};

/// How many columns [`sky_line`] writes.
pub const SKY_COLUMNS: usize = 12;

/// How many columns [`term_lines`] and [`phase_lines`] write.
pub const EVENT_COLUMNS: usize = 4;

/// The longest span [`term_lines`] and [`phase_lines`] accept, in seconds:
/// 400 Julian years, about 4 950 lunations and 9 600 solar terms.
pub const MAX_SPAN_SECONDS: i64 = 400 * 31_557_600;

/// Seconds in a day, as the astronomical series count them: no leap
/// second, because ΔT carries the Earth's rotational irregularity.
const SECONDS_PER_DAY: i64 = 86_400;

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

/// The word a [`MoonPhase`] is written as.
const fn phase_name(phase: MoonPhase) -> &'static str {
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
    let _ = write!(
        out,
        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t",
        solar_longitude(moment),
        solar_radius_vector(moment),
        lunar_longitude(moment),
        lunar_latitude(moment),
        lunar_distance(moment),
        lunar_phase(moment),
        lunar_illuminated_fraction(moment),
        unix_from_moment(new_moon_before(moment)),
        unix_from_moment(new_moon_at_or_after(moment)),
        delta_t(moment),
        regime_name(regime),
    );
    let source = format!(
        "{SUN_SOURCE}; {MOON_SOURCE}; {PHASES_SOURCE}; delta T: {}",
        delta_t_source(regime)
    );
    push_cell(&mut out, &source);
    out.push('\n');
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
        let _ = writeln!(
            out,
            "{}\t{unix}\t{}\t{}",
            term.solar_longitude_degrees(),
            term.chinese_name(),
            term.japanese_name()
        );
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
            let _ = writeln!(
                out,
                "{}\t{unix}\t{}\t",
                phase.elongation_degrees(),
                phase_name(phase)
            );
        }
        lunation += 1;
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
}
