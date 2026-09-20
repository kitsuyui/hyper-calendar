//! ISO 8601 time intervals and repeating intervals.
//!
//! Four shapes are defined by the standard and all four are here:
//! `start/end`, `start/duration`, `duration/end` and a duration on its own.
//! A repeating interval prefixes any of them with `Rn/`, or `R/` for an
//! unbounded repetition.
//!
//! # What is deliberately not here
//!
//! ISO 8601-1:2019 §5.5.4 allows the end of a `start/end` interval to be
//! written at reduced accuracy, inheriting the missing high-order components
//! from the start: `2026-09-21T14:00/16:00`. That inheritance is ambiguous
//! whenever the start is itself of reduced accuracy, and getting it wrong
//! silently produces an interval of the wrong length, so this module refuses
//! it rather than guessing. Both ends must be complete.

use core::fmt;

use crate::error::{ErrorKind, ParseError, ParseResult};
use crate::iso8601::duration::{IsoDuration, scan_duration};
use crate::iso8601::{Strictness, scan_date_time};
use crate::scan::Scanner;
use crate::value::IsoDateTime;

/// The largest number of `/`-separated parts any ISO 8601 interval has.
const MAX_PARTS: usize = 3;

/// A time interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Interval {
    /// `2026-01-01T00:00:00Z/2026-12-31T23:59:59Z`.
    StartEnd(IsoDateTime, IsoDateTime),
    /// `2026-01-01T00:00:00Z/P1Y`.
    StartDuration(IsoDateTime, IsoDuration),
    /// `P1Y/2026-12-31T23:59:59Z`.
    DurationEnd(IsoDuration, IsoDateTime),
    /// `P1Y` — a length with neither end fixed.
    Duration(IsoDuration),
}

impl Interval {
    /// The duration component, for the three shapes that have one.
    #[must_use]
    pub const fn duration(self) -> Option<IsoDuration> {
        match self {
            Self::StartEnd(_, _) => None,
            Self::StartDuration(_, duration)
            | Self::DurationEnd(duration, _)
            | Self::Duration(duration) => Some(duration),
        }
    }

    /// Write the interval.
    ///
    /// # Errors
    ///
    /// See [`crate::FormatError`].
    pub fn write<W: fmt::Write>(self, out: &mut W) -> crate::FormatResult<()> {
        match self {
            Self::StartEnd(start, end) => {
                start.write(out)?;
                out.write_char('/')?;
                end.write(out)
            }
            Self::StartDuration(start, duration) => {
                start.write(out)?;
                out.write_char('/')?;
                duration.write(out)
            }
            Self::DurationEnd(duration, end) => {
                duration.write(out)?;
                out.write_char('/')?;
                end.write(out)
            }
            Self::Duration(duration) => duration.write(out),
        }
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write(f).map_err(|_| fmt::Error)
    }
}

/// An interval repeated a stated number of times, or without limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RepeatingInterval {
    /// How many times, or `None` for `R/`, the unbounded repetition.
    pub repetitions: Option<u64>,
    /// The interval being repeated.
    pub interval: Interval,
}

impl RepeatingInterval {
    /// Write the repeating interval.
    ///
    /// # Errors
    ///
    /// See [`crate::FormatError`].
    pub fn write<W: fmt::Write>(self, out: &mut W) -> crate::FormatResult<()> {
        out.write_char('R')?;
        if let Some(count) = self.repetitions {
            write!(out, "{count}")?;
        }
        out.write_char('/')?;
        self.interval.write(out)
    }
}

impl fmt::Display for RepeatingInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write(f).map_err(|_| fmt::Error)
    }
}

/// Parse an ISO 8601 interval.
///
/// # Errors
///
/// See [`ParseError`].
pub fn parse(text: &str) -> ParseResult<Interval> {
    parse_with(text, Strictness::ISO)
}

/// Parse an ISO 8601 interval under a stated strictness.
///
/// # Errors
///
/// See [`ParseError`].
pub fn parse_with(text: &str, strictness: Strictness) -> ParseResult<Interval> {
    let (parts, count) = split(text)?;
    match count {
        1 => Ok(Interval::Duration(part_duration(parts[0], &strictness)?)),
        2 => build(parts[0], parts[1], &strictness),
        _ => Err(ParseError::new(
            ErrorKind::Invalid("an interval of more than two parts"),
            parts[2].1,
        )),
    }
}

/// Parse an ISO 8601 repeating interval, `Rn/…`.
///
/// # Errors
///
/// See [`ParseError`].
pub fn parse_repeating(text: &str) -> ParseResult<RepeatingInterval> {
    parse_repeating_with(text, Strictness::ISO)
}

/// Parse an ISO 8601 repeating interval under a stated strictness.
///
/// # Errors
///
/// See [`ParseError`].
pub fn parse_repeating_with(text: &str, strictness: Strictness) -> ParseResult<RepeatingInterval> {
    let (parts, count) = split(text)?;
    let (head, head_at) = parts[0];
    let mut scanner = Scanner::new(head);
    scanner.expect(b'R', "R")?;
    let repetitions = if scanner.is_empty() {
        None
    } else {
        let run = scanner.digit_run();
        if run == 0 || run > 18 {
            return Err(ParseError::new(
                ErrorKind::OutOfRange("repetition count"),
                head_at + scanner.pos(),
            ));
        }
        Some(
            scanner
                .take_digits(run)
                .map_err(|error| error.shifted(head_at))?,
        )
    };
    scanner.finish().map_err(|error| error.shifted(head_at))?;
    let interval = match count {
        2 => Interval::Duration(part_duration(parts[1], &strictness)?),
        3 => build(parts[1], parts[2], &strictness)?,
        _ => {
            return Err(ParseError::new(
                ErrorKind::UnexpectedEnd,
                head_at + head.len(),
            ));
        }
    };
    Ok(RepeatingInterval {
        repetitions,
        interval,
    })
}

/// Split on `/`, remembering where each part started.
///
/// A fixed-size array keeps this allocation-free; more than three parts is
/// not a shape ISO 8601 defines.
fn split(text: &str) -> ParseResult<([(&str, usize); MAX_PARTS + 1], usize)> {
    if text.is_empty() {
        return Err(ParseError::new(ErrorKind::Empty, 0));
    }
    let mut parts = [("", 0usize); MAX_PARTS + 1];
    let mut count = 0usize;
    let mut start = 0usize;
    for (index, byte) in text.bytes().enumerate() {
        if byte != b'/' {
            continue;
        }
        if count > MAX_PARTS {
            return Err(ParseError::new(
                ErrorKind::Invalid("an interval of more than three parts"),
                index,
            ));
        }
        parts[count] = (text.get(start..index).unwrap_or(""), start);
        count += 1;
        start = index + 1;
    }
    if count > MAX_PARTS {
        return Err(ParseError::new(
            ErrorKind::Invalid("an interval of more than three parts"),
            start,
        ));
    }
    parts[count] = (text.get(start..).unwrap_or(""), start);
    count += 1;
    Ok((parts, count))
}

/// Whether a part is a duration rather than a date-time.
fn looks_like_duration(text: &str) -> bool {
    matches!(
        text.as_bytes(),
        [b'P', ..] | [b'-', b'P', ..] | [b'+', b'P', ..]
    )
}

fn part_duration((text, at): (&str, usize), strictness: &Strictness) -> ParseResult<IsoDuration> {
    let mut scanner = Scanner::new(text);
    if scanner.is_empty() {
        return Err(ParseError::new(ErrorKind::Empty, at));
    }
    let value = scan_duration(&mut scanner, strictness).map_err(|error| error.shifted(at))?;
    scanner.finish().map_err(|error| error.shifted(at))?;
    Ok(value)
}

fn part_date_time((text, at): (&str, usize), strictness: &Strictness) -> ParseResult<IsoDateTime> {
    let mut scanner = Scanner::new(text);
    if scanner.is_empty() {
        return Err(ParseError::new(ErrorKind::Empty, at));
    }
    let value = scan_date_time(&mut scanner, strictness).map_err(|error| error.shifted(at))?;
    scanner.finish().map_err(|error| error.shifted(at))?;
    Ok(value)
}

fn build(
    first: (&str, usize),
    second: (&str, usize),
    strictness: &Strictness,
) -> ParseResult<Interval> {
    match (looks_like_duration(first.0), looks_like_duration(second.0)) {
        (false, false) => Ok(Interval::StartEnd(
            part_date_time(first, strictness)?,
            part_date_time(second, strictness)?,
        )),
        (false, true) => Ok(Interval::StartDuration(
            part_date_time(first, strictness)?,
            part_duration(second, strictness)?,
        )),
        (true, false) => Ok(Interval::DurationEnd(
            part_duration(first, strictness)?,
            part_date_time(second, strictness)?,
        )),
        (true, true) => Err(ParseError::new(
            ErrorKind::Invalid("an interval of two durations"),
            second.1,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::{String, ToString as _};

    fn round_trip(text: &str) -> String {
        parse(text).unwrap().to_string()
    }

    #[test]
    fn all_four_interval_shapes_round_trip() {
        for text in [
            "2026-01-01T00:00:00Z/2026-12-31T23:59:59Z",
            "2026-01-01T00:00:00Z/P1Y",
            "P1Y/2026-12-31T23:59:59Z",
            "P1Y",
        ] {
            assert_eq!(round_trip(text), text);
        }
    }

    #[test]
    fn each_shape_is_recognised_for_what_it_is() {
        assert!(matches!(
            parse("2026-01-01T00:00:00Z/2026-12-31T23:59:59Z").unwrap(),
            Interval::StartEnd(_, _)
        ));
        assert!(matches!(
            parse("2026-01-01T00:00:00Z/P1Y").unwrap(),
            Interval::StartDuration(_, _)
        ));
        assert!(matches!(
            parse("P1Y/2026-12-31T23:59:59Z").unwrap(),
            Interval::DurationEnd(_, _)
        ));
        assert!(matches!(parse("P1Y").unwrap(), Interval::Duration(_)));
    }

    #[test]
    fn a_repeating_interval_round_trips_with_its_count() {
        let text = "R5/PT1H/2026-01-01T00:00:00Z";
        let value = parse_repeating(text).unwrap();
        assert_eq!(value.repetitions, Some(5));
        assert!(matches!(value.interval, Interval::DurationEnd(_, _)));
        assert_eq!(value.to_string(), text);
    }

    #[test]
    fn an_unbounded_repetition_has_no_count() {
        let value = parse_repeating("R/P1D").unwrap();
        assert_eq!(value.repetitions, None);
        assert_eq!(value.to_string(), "R/P1D");
    }

    #[test]
    fn zero_repetitions_are_not_the_same_as_unbounded() {
        assert_eq!(parse_repeating("R0/P1D").unwrap().repetitions, Some(0));
        assert_eq!(parse_repeating("R0/P1D").unwrap().to_string(), "R0/P1D");
    }

    #[test]
    fn errors_inside_the_second_part_are_reported_in_whole_string_offsets() {
        let error = parse("2026-01-01T00:00:00Z/2026-13-01T00:00:00Z").unwrap_err();
        assert_eq!(error.kind(), ErrorKind::OutOfRange("month"));
        assert_eq!(error.offset(), 26);
    }

    #[test]
    fn two_durations_do_not_make_an_interval() {
        let error = parse("P1Y/P1M").unwrap_err();
        assert_eq!(
            error.kind(),
            ErrorKind::Invalid("an interval of two durations")
        );
        assert_eq!(error.offset(), 4);
    }

    #[test]
    fn an_interval_of_three_parts_is_not_an_interval() {
        let error = parse("P1Y/P1M/P1D").unwrap_err();
        assert_eq!(
            error.kind(),
            ErrorKind::Invalid("an interval of more than two parts")
        );
    }

    #[test]
    fn a_repeating_interval_needs_something_to_repeat() {
        assert!(parse_repeating("R5").is_err());
    }

    #[test]
    fn the_duration_of_an_interval_is_reachable_without_a_match() {
        assert_eq!(
            parse("2026-01-01T00:00:00Z/P1Y").unwrap().duration(),
            Some(IsoDuration {
                years: Some(1),
                ..IsoDuration::default()
            })
        );
        assert_eq!(
            parse("2026-01-01T00:00:00Z/2026-12-31T23:59:59Z")
                .unwrap()
                .duration(),
            None
        );
    }
}
