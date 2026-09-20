//! Errors raised while parsing zone data or resolving a local time.

use core::fmt;

/// Result alias for time-zone operations.
pub type TzResult<T> = Result<T, TzError>;

/// What can go wrong in this crate.
///
/// The parsing variants are deliberately fine-grained: a POSIX `TZ` string and
/// a TZif file are both user-supplied data, and "it did not parse" is not a
/// useful diagnostic when the caller has to decide whether to fall back to a
/// built-in zone or to refuse the input altogether.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TzError {
    /// The offset is outside `-25:59:59 ..= +25:59:59`.
    ///
    /// That is the range RFC 8536 allows a TZif local time type to carry, and
    /// it is wider than any offset ever used civilly (`+14:00`, Kiribati).
    OffsetOutOfRange,
    /// The text is not an offset in any accepted form.
    MalformedOffset,
    /// A zone abbreviation is empty, too long, or uses characters POSIX does
    /// not allow there.
    MalformedAbbreviation,
    /// A POSIX daylight-saving rule (`Jn`, `n` or `Mm.w.d`) is malformed or
    /// out of range.
    MalformedRule,
    /// Input remained after a complete `TZ` string had been read.
    TrailingInput,
    /// The input ended in the middle of a construct.
    UnexpectedEnd,
    /// The local time occurs twice and the caller asked to be told rather than
    /// to have one occurrence picked.
    AmbiguousLocalTime,
    /// The local time never occurs and the caller asked to be told rather than
    /// to have it shifted.
    NonexistentLocalTime,
    /// No zone is known by that name.
    UnknownZone,
    /// Arithmetic left the representable range.
    Overflow,
    /// The byte slice does not begin with the TZif magic number.
    NotTzifData,
    /// The TZif version byte is not one of `\0`, `2` or `3`.
    UnsupportedTzifVersion,
    /// A TZif structure claims more bytes than the slice holds.
    TruncatedTzifData,
    /// A TZif structure is internally inconsistent: a designation index past
    /// the end of the string table, an out-of-range offset, a footer that is
    /// not newline-delimited.
    MalformedTzifData,
    /// The system zone database could not be read.
    #[cfg(feature = "std")]
    Io(std::io::ErrorKind),
}

impl fmt::Display for TzError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OffsetOutOfRange => f.write_str("offset outside -25:59:59..=+25:59:59"),
            Self::MalformedOffset => f.write_str("malformed UTC offset"),
            Self::MalformedAbbreviation => f.write_str("malformed zone abbreviation"),
            Self::MalformedRule => f.write_str("malformed POSIX transition rule"),
            Self::TrailingInput => f.write_str("trailing input after a complete TZ string"),
            Self::UnexpectedEnd => f.write_str("input ended unexpectedly"),
            Self::AmbiguousLocalTime => f.write_str("local time occurs twice"),
            Self::NonexistentLocalTime => f.write_str("local time never occurs"),
            Self::UnknownZone => f.write_str("unknown time zone"),
            Self::Overflow => f.write_str("time arithmetic overflowed"),
            Self::NotTzifData => f.write_str("not TZif data"),
            Self::UnsupportedTzifVersion => f.write_str("unsupported TZif version"),
            Self::TruncatedTzifData => f.write_str("truncated TZif data"),
            Self::MalformedTzifData => f.write_str("malformed TZif data"),
            #[cfg(feature = "std")]
            Self::Io(kind) => write!(f, "could not read the zone database: {kind}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TzError {}

impl From<hc_core::TimeError> for TzError {
    fn from(value: hc_core::TimeError) -> Self {
        match value {
            hc_core::TimeError::OutOfRange => Self::OffsetOutOfRange,
            _ => Self::Overflow,
        }
    }
}

impl From<hc_calendar::CalendarError> for TzError {
    fn from(value: hc_calendar::CalendarError) -> Self {
        match value {
            hc_calendar::CalendarError::DayOutOfRange => Self::OffsetOutOfRange,
            _ => Self::Overflow,
        }
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for TzError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value.kind())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_error_renders_a_distinct_sentence() {
        let messages = [
            TzError::OffsetOutOfRange.to_string(),
            TzError::MalformedOffset.to_string(),
            TzError::MalformedRule.to_string(),
            TzError::AmbiguousLocalTime.to_string(),
            TzError::NonexistentLocalTime.to_string(),
            TzError::NotTzifData.to_string(),
        ];
        for (index, message) in messages.iter().enumerate() {
            assert!(!message.is_empty());
            assert!(!messages[..index].contains(message));
        }
    }

    #[test]
    fn calendar_and_core_errors_map_onto_zone_errors() {
        assert_eq!(
            TzError::from(hc_core::TimeError::Overflow),
            TzError::Overflow
        );
        assert_eq!(
            TzError::from(hc_calendar::CalendarError::DayOutOfRange),
            TzError::OffsetOutOfRange
        );
    }
}
