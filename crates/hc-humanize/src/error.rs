//! Errors raised while humanising a span of time.

use core::fmt;

use hc_core::TimeError;
use hc_i18n::I18nError;

/// Result alias for humanising operations.
pub type HumanizeResult<T> = Result<T, HumanizeError>;

/// What can go wrong on the way from a [`hc_core::Duration`] to a phrase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum HumanizeError {
    /// No entry in the locale's fallback chain — root included — carries a
    /// pattern for that unit, style and direction.
    ///
    /// The root entry states every unit, so in practice this only happens
    /// when a data entry is incomplete.
    NoPattern,
    /// The span, or a count derived from it, does not fit the integer type
    /// it has to be rendered from.
    Overflow,
    /// The requested component list was empty, so there is nothing to say.
    NoComponents,
    /// The output sink refused a write.
    WriteFailed,
    /// The number could not be rendered in the locale's numbering system.
    Number(I18nError),
    /// The options ask for something the function does not do, such as a
    /// `naturaldelta` minimum unit above seconds, which `humanize` refuses
    /// with a `ValueError`.
    Unsupported(&'static str),
}

impl fmt::Display for HumanizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoPattern => f.write_str("no relative-time pattern for that unit and style"),
            Self::Overflow => f.write_str("the span is too large to express in that unit"),
            Self::NoComponents => f.write_str("no components were requested"),
            Self::WriteFailed => f.write_str("the output sink refused the write"),
            Self::Number(error) => write!(f, "number formatting failed: {error}"),
            Self::Unsupported(what) => write!(f, "not supported: {what}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for HumanizeError {}

impl From<fmt::Error> for HumanizeError {
    fn from(_: fmt::Error) -> Self {
        Self::WriteFailed
    }
}

impl From<I18nError> for HumanizeError {
    fn from(error: I18nError) -> Self {
        match error {
            I18nError::WriteFailed => Self::WriteFailed,
            other => Self::Number(other),
        }
    }
}

impl From<TimeError> for HumanizeError {
    fn from(_: TimeError) -> Self {
        Self::Overflow
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString as _;

    #[test]
    fn every_variant_says_what_went_wrong() {
        let messages = [
            HumanizeError::NoPattern.to_string(),
            HumanizeError::Overflow.to_string(),
            HumanizeError::NoComponents.to_string(),
            HumanizeError::WriteFailed.to_string(),
            HumanizeError::Number(I18nError::NumberOutOfRange).to_string(),
            HumanizeError::Unsupported("that").to_string(),
        ];
        for message in &messages {
            assert!(!message.is_empty());
            assert!(!message.ends_with('.'));
        }
        assert!(messages[4].contains("out of range"));
    }

    #[test]
    fn a_refused_write_is_a_refused_write_whichever_layer_reports_it() {
        // Both a raw `fmt::Error` and the i18n crate's own wrapper have to
        // land on the same variant, or a caller would have to match twice.
        assert_eq!(HumanizeError::from(fmt::Error), HumanizeError::WriteFailed);
        assert_eq!(
            HumanizeError::from(I18nError::WriteFailed),
            HumanizeError::WriteFailed
        );
    }

    #[test]
    fn a_number_that_will_not_render_is_reported_with_its_cause() {
        assert_eq!(
            HumanizeError::from(I18nError::UnknownNumberingSystem),
            HumanizeError::Number(I18nError::UnknownNumberingSystem)
        );
    }

    #[test]
    fn an_arithmetic_failure_upstream_becomes_an_overflow_here() {
        assert_eq!(
            HumanizeError::from(TimeError::Overflow),
            HumanizeError::Overflow
        );
        assert_eq!(
            HumanizeError::from(TimeError::OutOfRange),
            HumanizeError::Overflow
        );
    }
}
