//! Errors raised while parsing text into values and rendering values as text.
//!
//! # Why a byte offset
//!
//! A date parser is almost always sitting in a validation path, and the
//! caller's next move is to tell a human what is wrong with the string they
//! typed. "Invalid date" cannot do that; "expected two digits at byte 8" can.
//! Every [`ParseError`] therefore carries the byte offset into the *original*
//! input where the parser stopped, so a caller can underline it.

use core::fmt;

/// The result of parsing text.
pub type ParseResult<T> = Result<T, ParseError>;

/// The result of rendering a value as text.
pub type FormatResult<T> = Result<T, FormatError>;

/// The result of turning a parsed value into a calendar or instant value.
pub type ValueResult<T> = Result<T, ValueError>;

/// What a parser expected, and where it expected it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseError {
    kind: ErrorKind,
    offset: usize,
}

impl ParseError {
    /// Build an error from a reason and a byte offset into the input.
    #[must_use]
    pub const fn new(kind: ErrorKind, offset: usize) -> Self {
        Self { kind, offset }
    }

    /// What was expected.
    #[must_use]
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// The byte offset into the input where the parser stopped.
    ///
    /// This is a byte offset, not a character offset: every grammar in this
    /// crate is ASCII, so the two coincide for any input that parses at all,
    /// and a byte offset is what slicing the input needs.
    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// The same error, reported at a different offset.
    ///
    /// Used when a sub-parser ran against a slice of the input and its
    /// offsets need shifting back into the caller's coordinates.
    #[must_use]
    pub const fn shifted(self, by: usize) -> Self {
        Self {
            kind: self.kind,
            offset: self.offset + by,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at byte {}", self.kind, self.offset)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}

/// Why a parse stopped.
///
/// The variants name the *grammar* expectation rather than the field, because
/// the field is usually obvious from the offset and the expectation is not.
/// Where the field genuinely disambiguates — a range violation, an internal
/// inconsistency — it is carried as a `&'static str`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// The input was empty, or held nothing but whitespace.
    Empty,
    /// A decimal digit was required here.
    Digit,
    /// Exactly this many decimal digits were required here.
    DigitCount(u8),
    /// This literal character or string was required here.
    Literal(&'static str),
    /// One of these characters was required here.
    OneOf(&'static str),
    /// The input ended in the middle of a value.
    UnexpectedEnd,
    /// A complete value was read and text remained after it.
    TrailingText,
    /// A field parsed as a number but lies outside its legal range.
    OutOfRange(&'static str),
    /// A field is legal on its own but inconsistent with the rest of the
    /// value: 31 February, week 53 of a 52-week year, `24:00:01`.
    Invalid(&'static str),
    /// Basic and extended format were mixed inside one value, which ISO 8601
    /// forbids: separators are all present or all absent.
    MixedFormat,
    /// The form is valid ISO 8601 but the [`crate::iso8601::Strictness`] in
    /// force refuses it.
    Forbidden(&'static str),
    /// The value parsed but cannot be represented by this library.
    Unrepresentable(&'static str),
    /// A pattern-driven parse found input that does not match the pattern's
    /// literal text.
    PatternMismatch,
    /// A name — a month, a weekday, a day period, a zone abbreviation — was
    /// expected here and none of the known ones matched.
    UnknownName(&'static str),
    /// The pattern used a field this crate cannot parse. The offset is into
    /// the *input*, not the pattern, because that is what the other variants
    /// mean by it; the character names the field.
    UnsupportedPatternField(char),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("expected a value, found empty input"),
            Self::Digit => f.write_str("expected a digit"),
            Self::DigitCount(n) => write!(f, "expected {n} digits"),
            Self::Literal(text) => write!(f, "expected {text:?}"),
            Self::OneOf(set) => write!(f, "expected one of {set:?}"),
            Self::UnexpectedEnd => f.write_str("input ended in the middle of a value"),
            Self::TrailingText => f.write_str("unexpected text after a complete value"),
            Self::OutOfRange(field) => write!(f, "{field} is out of range"),
            Self::Invalid(field) => write!(f, "{field} is inconsistent with the rest of the value"),
            Self::MixedFormat => f.write_str("basic and extended format mixed in one value"),
            Self::Forbidden(what) => write!(f, "{what} is not allowed by the strictness in force"),
            Self::Unrepresentable(what) => write!(f, "{what} cannot be represented"),
            Self::PatternMismatch => f.write_str("input does not match the pattern"),
            Self::UnknownName(kind) => write!(f, "not a known {kind} name"),
            Self::UnsupportedPatternField(field) => {
                write!(f, "pattern field {field:?} cannot be parsed")
            }
        }
    }
}

/// Why rendering a value as text stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FormatError {
    /// The [`core::fmt::Write`] sink refused the bytes.
    ///
    /// `core::fmt::Error` carries no detail by design; a caller writing into
    /// its own buffer knows why its buffer refused.
    Sink,
    /// A pattern used a conversion this crate does not implement.
    UnknownField(char),
    /// A CLDR pattern opened a `'` literal and never closed it.
    UnterminatedLiteral(usize),
    /// The value cannot be written in the requested form: a year outside
    /// `0000..=9999` in a four-digit field, a `%s` with no zone to resolve
    /// it against.
    Unrepresentable(&'static str),
}

impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sink => f.write_str("the output sink refused the text"),
            Self::UnknownField(field) => write!(f, "unsupported pattern field {field:?}"),
            Self::UnterminatedLiteral(at) => write!(f, "unterminated quoted literal at byte {at}"),
            Self::Unrepresentable(what) => write!(f, "{what} cannot be written in this form"),
        }
    }
}

impl From<fmt::Error> for FormatError {
    fn from(_: fmt::Error) -> Self {
        Self::Sink
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FormatError {}

/// Why a parsed value could not become a calendar date or an instant.
///
/// Parsing and interpreting are separate steps here on purpose: `2026-09` is
/// a perfectly good ISO 8601 date and a perfectly hopeless argument to
/// "which day is this", and a parser that conflated the two would have to
/// pick one of those two truths.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ValueError {
    /// The value names a year or a month, not a day.
    ReducedAccuracy,
    /// The value carries no time of day.
    MissingTime,
    /// The value carries no zone, so it names a reading rather than an
    /// instant. Supply a [`hc_tz::TimeZone`] to resolve it.
    MissingZone,
    /// The local reading does not exist in the supplied zone, or occurs twice
    /// in it, and the caller asked to be told rather than to have one picked.
    UnresolvableLocalTime,
    /// A pattern-driven parse read no value for a field the result needs.
    MissingField(&'static str),
    /// The duration carries years or months, which are not spans of fixed
    /// length: a month is 28 to 31 days depending on which month it is
    /// added to, so the value is only meaningful relative to a date.
    NominalDuration,
    /// The calendar refused the fields.
    Calendar(hc_calendar::CalendarError),
    /// The zone refused the reading.
    Zone(hc_tz::TzError),
    /// Time arithmetic left the representable range.
    Time(hc_core::TimeError),
}

impl fmt::Display for ValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReducedAccuracy => f.write_str("the value names no single day"),
            Self::MissingTime => f.write_str("the value carries no time of day"),
            Self::MissingZone => f.write_str("the value carries no zone"),
            Self::UnresolvableLocalTime => {
                f.write_str("the local reading is ambiguous or does not exist in that zone")
            }
            Self::MissingField(field) => write!(f, "no {field} was read"),
            Self::NominalDuration => {
                f.write_str("a duration in years or months has no fixed length")
            }
            Self::Calendar(error) => write!(f, "{error}"),
            Self::Zone(error) => write!(f, "{error}"),
            Self::Time(error) => write!(f, "{error}"),
        }
    }
}

impl From<hc_calendar::CalendarError> for ValueError {
    fn from(error: hc_calendar::CalendarError) -> Self {
        Self::Calendar(error)
    }
}

impl From<hc_tz::TzError> for ValueError {
    fn from(error: hc_tz::TzError) -> Self {
        Self::Zone(error)
    }
}

impl From<hc_core::TimeError> for ValueError {
    fn from(error: hc_core::TimeError) -> Self {
        Self::Time(error)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ValueError {}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString as _;

    #[test]
    fn a_parse_error_reports_what_and_where() {
        let error = ParseError::new(ErrorKind::DigitCount(2), 8);
        assert_eq!(error.offset(), 8);
        assert_eq!(error.kind(), ErrorKind::DigitCount(2));
        assert_eq!(error.to_string(), "expected 2 digits at byte 8");
    }

    #[test]
    fn shifting_an_error_moves_only_its_offset() {
        let error = ParseError::new(ErrorKind::Digit, 3).shifted(10);
        assert_eq!(error.offset(), 13);
        assert_eq!(error.kind(), ErrorKind::Digit);
    }

    #[test]
    fn a_sink_failure_becomes_a_format_error() {
        assert_eq!(FormatError::from(fmt::Error), FormatError::Sink);
    }

    #[test]
    fn value_errors_carry_the_underlying_cause() {
        let error = ValueError::from(hc_calendar::CalendarError::DayOutOfRange);
        assert!(matches!(error, ValueError::Calendar(_)));
        assert!(!error.to_string().is_empty());
    }
}
