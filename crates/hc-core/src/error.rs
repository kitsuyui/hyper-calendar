//! Error type shared by the time primitives.

use core::fmt;

/// Result alias used throughout `hyper-calendar`'s core layer.
pub type TimeResult<T> = Result<T, TimeError>;

/// Everything that can go wrong while manipulating a physical time value.
///
/// The variants are deliberately coarse: callers either recover by choosing
/// different inputs or they propagate. Finer diagnostics belong in the crate
/// that has the domain context (a calendar, a parser, a locale).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TimeError {
    /// An arithmetic operation left the representable range.
    Overflow,
    /// A division by zero was requested.
    DivideByZero,
    /// A field was outside its legal range (for example, 61 seconds).
    OutOfRange,
    /// The value predates the point where the requested model is defined.
    ///
    /// UTC, for instance, does not exist before 1960 and is only a
    /// leap-second scale from 1972 onward.
    BeforeModelStart,
    /// The value lies after the last point where the model has real data.
    ///
    /// Leap seconds and UT1 offsets are announced, not predicted, so any
    /// answer past the end of the published table would be a guess.
    AfterModelEnd,
    /// A floating-point computation produced a non-finite value.
    NotFinite,
}

impl fmt::Display for TimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Overflow => "time arithmetic overflowed the representable range",
            Self::DivideByZero => "division by zero",
            Self::OutOfRange => "value out of range",
            Self::BeforeModelStart => "value predates the start of the requested time model",
            Self::AfterModelEnd => "value postdates the published data for the requested model",
            Self::NotFinite => "computation produced a non-finite value",
        };
        f.write_str(message)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TimeError {}
