//! Errors raised while converting between units of time.

use core::fmt;

use hc_core::TimeError;

/// Result alias for the unit layer.
pub type UnitResult<T> = Result<T, UnitError>;

/// What can go wrong when an exact quantity is converted.
///
/// The variants separate the two failures a caller fixes differently. An
/// [`Inexact`](UnitError::Inexact) conversion succeeded arithmetically and
/// was refused on principle — the caller either accepts rounding or picks a
/// representation that can hold the value. An
/// [`Overflow`](UnitError::Overflow) means the numbers themselves did not
/// fit, and no amount of rounding helps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum UnitError {
    /// A numerator or denominator left the `i128` range.
    Overflow,
    /// A denominator of zero was requested.
    DivideByZero,
    /// The value is exact as a rational but not as a [`hc_core::Duration`].
    ///
    /// `Duration` counts attoseconds, so it holds any rational whose
    /// denominator divides 10¹⁸ and no others. A flick is 1/705 600 000 s,
    /// which is not one of them: it is 1 417 233 560.090 7… attoseconds.
    /// Rather than silently truncate the unit whose entire purpose is exact
    /// division, the conversion says so.
    Inexact,
    /// The exact core layer refused the operation.
    Time(TimeError),
}

impl fmt::Display for UnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Overflow => f.write_str("unit arithmetic overflowed the representable range"),
            Self::DivideByZero => f.write_str("division by zero"),
            Self::Inexact => {
                f.write_str("value has no exact attosecond representation; round explicitly")
            }
            Self::Time(inner) => write!(f, "{inner}"),
        }
    }
}

impl UnitError {
    /// The same conversion as the [`From`] impl below, usable in a `const fn`.
    #[must_use]
    pub const fn from_time(inner: TimeError) -> Self {
        Self::Time(inner)
    }
}

impl From<TimeError> for UnitError {
    fn from(inner: TimeError) -> Self {
        Self::Time(inner)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for UnitError {}
