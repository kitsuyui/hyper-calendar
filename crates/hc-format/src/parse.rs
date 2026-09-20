//! A tolerant front door for when all you have is "a date string".
//!
//! The rest of this crate asks the caller to know which format they are
//! holding. Very often they do not: a string arrives from a config file, a
//! spreadsheet column or an HTTP header, and the only honest description of
//! it is "a date, probably". [`detect`] sniffs which of the crate's grammars
//! it is and says what it found, rather than making the caller try each one
//! and collect four errors.
//!
//! # How it decides
//!
//! The rules are deliberately simple and are listed here so that a caller can
//! predict them:
//!
//! 1. `R` followed by a digit or `/` — a repeating interval.
//! 2. `P`, `+P` or `-P` — a duration.
//! 3. Anything containing `/` — an interval.
//! 4. Anything containing an ASCII letter other than the ISO 8601 designators
//!    `T`, `W` and `Z` — an RFC 5322 (email) date.
//! 5. Everything else — ISO 8601.
//!
//! Surrounding whitespace is trimmed first, and error offsets are reported
//! against the original string, not the trimmed one.
//!
//! ```
//! use hc_format::parse::{Detected, detect};
//!
//! assert!(matches!(detect("2026-09-21")?, Detected::DateTime(_)));
//! assert!(matches!(detect("Mon, 21 Sep 2026 14:30:05 +0900")?, Detected::Email(_)));
//! assert!(matches!(detect("P1Y")?, Detected::Duration(_)));
//! # Ok::<(), hc_format::ParseError>(())
//! ```

use crate::error::{ErrorKind, ParseError, ParseResult};
use crate::iso8601::{self, Interval, IsoDuration, RepeatingInterval, Strictness, interval};
use crate::value::{IsoDateTime, OffsetDateTime};
use crate::{rfc2822, value};

/// What a sniffing parse turned out to be holding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Detected {
    /// An ISO 8601 date, time or date-time, possibly of reduced accuracy.
    DateTime(IsoDateTime),
    /// An RFC 5322 date, as email and HTTP write them.
    Email(OffsetDateTime),
    /// An ISO 8601 duration.
    Duration(IsoDuration),
    /// An ISO 8601 interval.
    Interval(Interval),
    /// An ISO 8601 repeating interval.
    Repeating(RepeatingInterval),
}

impl Detected {
    /// The reading, for the two variants that name one.
    ///
    /// An interval names two and a duration names none, so both give `None`
    /// rather than a first-end-of-interval that the caller did not ask for.
    ///
    /// # Errors
    ///
    /// [`crate::ValueError`] when the value is a date of reduced accuracy or
    /// carries no time.
    pub fn to_offset_date_time(self) -> Option<crate::ValueResult<OffsetDateTime>> {
        match self {
            Self::DateTime(value) => Some(value.to_offset_date_time()),
            Self::Email(value) => Some(Ok(value)),
            Self::Duration(_) | Self::Interval(_) | Self::Repeating(_) => None,
        }
    }
}

/// Work out which grammar a string belongs to and parse it.
///
/// # Errors
///
/// The error of whichever grammar was chosen, with offsets measured against
/// the string as supplied.
pub fn detect(text: &str) -> ParseResult<Detected> {
    detect_with(text, Strictness::ISO)
}

/// Sniff and parse under a stated strictness.
///
/// The strictness reaches the ISO 8601 grammars. RFC 5322 has its own rules
/// about what a parser must accept and is unaffected by it.
///
/// # Errors
///
/// See [`detect`].
pub fn detect_with(text: &str, strictness: Strictness) -> ParseResult<Detected> {
    let leading = text.len() - text.trim_start().len();
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err(ParseError::new(ErrorKind::Empty, 0));
    }
    let shift = |error: ParseError| error.shifted(leading);
    let bytes = trimmed.as_bytes();

    if bytes.first() == Some(&b'R')
        && bytes
            .get(1)
            .is_some_and(|byte| byte.is_ascii_digit() || *byte == b'/')
    {
        return interval::parse_repeating_with(trimmed, strictness)
            .map(Detected::Repeating)
            .map_err(shift);
    }
    if matches!(bytes, [b'P', ..] | [b'-', b'P', ..] | [b'+', b'P', ..]) {
        return iso8601::duration::parse_with(trimmed, strictness)
            .map(Detected::Duration)
            .map_err(shift);
    }
    if trimmed.contains('/') {
        return interval::parse_with(trimmed, strictness)
            .map(Detected::Interval)
            .map_err(shift);
    }
    if looks_like_email_date(bytes) {
        return rfc2822::parse(trimmed).map(Detected::Email).map_err(shift);
    }
    iso8601::parse_with(trimmed, strictness)
        .map(Detected::DateTime)
        .map_err(shift)
}

/// Parse whatever the string is and insist on a reading.
///
/// # Errors
///
/// The grammar's own error, or [`ErrorKind::Invalid`] when the string turned
/// out to be a duration or an interval, which name no single reading.
pub fn date_time(text: &str) -> ParseResult<OffsetDateTime> {
    let detected = detect(text)?;
    let Some(result) = detected.to_offset_date_time() else {
        return Err(ParseError::new(
            ErrorKind::Invalid("a duration or interval where a date-time was wanted"),
            0,
        ));
    };
    result.map_err(|error| {
        let kind = match error {
            crate::ValueError::ReducedAccuracy => ErrorKind::Invalid("a date of reduced accuracy"),
            crate::ValueError::MissingTime => ErrorKind::Invalid("a date with no time of day"),
            _ => ErrorKind::Unrepresentable("the value"),
        };
        ParseError::new(kind, 0)
    })
}

/// Parse whatever the string is and insist on a day.
///
/// This is the "I have a date column" entry point: it accepts every date
/// form the crate knows and gives back a fixed day, or says why it could
/// not.
///
/// # Errors
///
/// See [`date_time`], except that a time of day is not required.
pub fn day(text: &str) -> ParseResult<hc_calendar::Rd> {
    let detected = detect(text)?;
    match detected {
        Detected::DateTime(value) => value
            .date
            .to_fixed()
            .map_err(|_| ParseError::new(ErrorKind::Invalid("a date of reduced accuracy"), 0)),
        Detected::Email(value) => Ok(value.local.day),
        Detected::Duration(_) | Detected::Interval(_) | Detected::Repeating(_) => {
            Err(ParseError::new(
                ErrorKind::Invalid("a duration or interval where a date was wanted"),
                0,
            ))
        }
    }
}

/// Whether the string holds a letter that ISO 8601 has no use for.
///
/// `T`, `W` and `Z` are ISO 8601's own designators, in either case; any
/// other letter means a month or weekday name, which only the email format
/// carries.
fn looks_like_email_date(bytes: &[u8]) -> bool {
    bytes.iter().any(|byte| {
        byte.is_ascii_alphabetic() && !matches!(byte.to_ascii_uppercase(), b'T' | b'W' | b'Z')
    })
}

/// Re-exported so that a caller matching on [`Detected`] does not have to
/// reach into two modules for the types inside it.
pub use value::ZoneInfo;

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::Rd;

    #[test]
    fn each_grammar_is_recognised_for_what_it_is() {
        assert!(matches!(
            detect("2026-09-21").unwrap(),
            Detected::DateTime(_)
        ));
        assert!(matches!(
            detect("2026-09-21T14:30:05Z").unwrap(),
            Detected::DateTime(_)
        ));
        assert!(matches!(detect("20260921").unwrap(), Detected::DateTime(_)));
        assert!(matches!(
            detect("2026-W38-1").unwrap(),
            Detected::DateTime(_)
        ));
        assert!(matches!(
            detect("Mon, 21 Sep 2026 14:30:05 +0900").unwrap(),
            Detected::Email(_)
        ));
        assert!(matches!(
            detect("P3Y6M4DT12H30M5S").unwrap(),
            Detected::Duration(_)
        ));
        assert!(matches!(
            detect("2026-01-01T00:00:00Z/P1Y").unwrap(),
            Detected::Interval(_)
        ));
        assert!(matches!(
            detect("R5/PT1H/2026-01-01T00:00:00Z").unwrap(),
            Detected::Repeating(_)
        ));
    }

    #[test]
    fn surrounding_whitespace_is_trimmed_and_offsets_still_point_at_the_input() {
        assert!(detect("  2026-09-21  ").is_ok());
        let error = detect("  2026-13-21").unwrap_err();
        assert_eq!(error.kind(), ErrorKind::OutOfRange("month"));
        assert_eq!(error.offset(), 7);
    }

    #[test]
    fn empty_input_says_so() {
        assert_eq!(detect("   ").unwrap_err().kind(), ErrorKind::Empty);
    }

    #[test]
    fn a_day_comes_out_of_every_date_form() {
        for text in [
            "2026-09-21",
            "20260921",
            "2026-264",
            "2026-W38-1",
            "2026-09-21T14:30:05Z",
            "Mon, 21 Sep 2026 14:30:05 +0900",
        ] {
            let day_number = day(text).unwrap();
            assert!(
                day_number == Rd(739_880) || day_number == Rd(739_873),
                "{text} gave {day_number:?}"
            );
        }
    }

    #[test]
    fn a_duration_is_not_a_date_time() {
        let error = date_time("P1Y").unwrap_err();
        assert_eq!(
            error.kind(),
            ErrorKind::Invalid("a duration or interval where a date-time was wanted")
        );
    }

    #[test]
    fn a_reduced_date_is_not_a_date_time_either() {
        assert_eq!(
            date_time("2026-09").unwrap_err().kind(),
            ErrorKind::Invalid("a date of reduced accuracy")
        );
        assert_eq!(
            day("2026-09").unwrap_err().kind(),
            ErrorKind::Invalid("a date of reduced accuracy")
        );
    }

    #[test]
    fn a_detected_date_time_resolves_to_an_instant_when_it_has_a_zone() {
        let value = date_time("2026-09-21T14:30:05+09:00").unwrap();
        assert_eq!(value.to_unix().unwrap().seconds(), 1_789_968_605);
    }

    #[test]
    fn an_email_date_and_an_iso_one_agree_about_the_instant() {
        let email = date_time("Mon, 21 Sep 2026 14:30:05 +0900").unwrap();
        let iso = date_time("2026-09-21T14:30:05+09:00").unwrap();
        assert_eq!(email.to_unix().unwrap(), iso.to_unix().unwrap());
    }

    #[test]
    fn a_duration_or_interval_never_pretends_to_be_a_reading() {
        assert!(detect("P1Y").unwrap().to_offset_date_time().is_none());
        assert!(
            detect("2026-01-01T00:00:00Z/P1Y")
                .unwrap()
                .to_offset_date_time()
                .is_none()
        );
    }

    #[test]
    fn the_strictness_reaches_the_iso_grammars() {
        assert!(detect_with("20260921", Strictness::ISO).is_ok());
        assert!(detect_with("20260921", Strictness::FULL).is_err());
    }
}
