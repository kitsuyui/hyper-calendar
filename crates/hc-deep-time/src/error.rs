//! Errors raised while manipulating deep-time magnitudes.

use core::fmt;

use hc_core::TimeError;
use hc_uncertainty::UncertaintyError;

/// Result alias for the deep-time layer.
pub type DeepTimeResult<T> = Result<T, DeepTimeError>;

/// What can go wrong when a published magnitude is manipulated.
///
/// The variants separate failures that need different fixes: an arithmetic
/// problem in the uncertainty layer, a span that simply will not fit in the
/// exact [`hc_core::Duration`] representation, and a request that is
/// scientifically ill-posed — converting an uncalibrated radiocarbon age to a
/// calendar year without a calibration curve.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeepTimeError {
    /// A value or a propagated uncertainty was NaN or infinite.
    NotFinite,
    /// The magnitude lies outside the range an exact `Duration` can hold.
    ///
    /// `Duration` counts whole seconds in an `i128`, so it reaches about
    /// 4×10²⁰ times the age of the universe upwards but stops dead at one
    /// attosecond downwards. A Planck time is twenty-six decades below that
    /// floor and simply has no exact representation.
    OutOfDurationRange,
    /// An uncalibrated radiocarbon age was asked for a calendar year.
    ///
    /// Radiocarbon years are not calendar years, and turning one into the
    /// other needs a calibration curve (IntCal20 and its marine and southern
    /// hemisphere companions), which this crate deliberately does not carry.
    /// See [`crate::archaeology`].
    CalibrationRequired,
    /// The operation is mathematically undefined for these inputs.
    ///
    /// A logarithm needs a strictly positive argument, so the ratio of a
    /// zero-length span to anything lands here rather than producing −∞.
    OutOfDomain,
    /// The uncertainty layer refused the operation.
    Uncertainty(UncertaintyError),
    /// The exact core layer refused the operation.
    Time(TimeError),
}

impl fmt::Display for DeepTimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFinite => f.write_str("value is not finite"),
            Self::OutOfDurationRange => {
                f.write_str("magnitude does not fit an exact hc_core::Duration")
            }
            Self::CalibrationRequired => f.write_str(
                "uncalibrated radiocarbon years need a calibration curve before they are calendar years",
            ),
            Self::OutOfDomain => f.write_str("operation is undefined for these inputs"),
            Self::Uncertainty(inner) => write!(f, "uncertainty layer: {inner}"),
            Self::Time(inner) => write!(f, "core time layer: {inner}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DeepTimeError {}

impl From<UncertaintyError> for DeepTimeError {
    fn from(value: UncertaintyError) -> Self {
        match value {
            UncertaintyError::NotFinite => Self::NotFinite,
            UncertaintyError::OutOfDomain => Self::OutOfDomain,
            other => Self::Uncertainty(other),
        }
    }
}

impl From<TimeError> for DeepTimeError {
    fn from(value: TimeError) -> Self {
        match value {
            TimeError::NotFinite => Self::NotFinite,
            TimeError::Overflow => Self::OutOfDurationRange,
            other => Self::Time(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    use alloc::string::ToString as _;

    #[test]
    fn a_core_overflow_becomes_an_out_of_duration_range_error() {
        assert_eq!(
            DeepTimeError::from(TimeError::Overflow),
            DeepTimeError::OutOfDurationRange
        );
    }

    #[test]
    fn a_non_finite_uncertainty_keeps_its_meaning_across_the_boundary() {
        assert_eq!(
            DeepTimeError::from(UncertaintyError::NotFinite),
            DeepTimeError::NotFinite
        );
        assert_eq!(
            DeepTimeError::from(UncertaintyError::OutOfDomain),
            DeepTimeError::OutOfDomain
        );
    }

    #[test]
    fn an_unmapped_uncertainty_error_is_carried_verbatim() {
        assert_eq!(
            DeepTimeError::from(UncertaintyError::DivideByZero),
            DeepTimeError::Uncertainty(UncertaintyError::DivideByZero)
        );
        assert_eq!(
            DeepTimeError::from(TimeError::OutOfRange),
            DeepTimeError::Time(TimeError::OutOfRange)
        );
    }

    #[test]
    fn every_variant_renders_a_non_empty_message() {
        let variants = [
            DeepTimeError::NotFinite,
            DeepTimeError::OutOfDurationRange,
            DeepTimeError::CalibrationRequired,
            DeepTimeError::OutOfDomain,
            DeepTimeError::Uncertainty(UncertaintyError::DivideByZero),
            DeepTimeError::Time(TimeError::Overflow),
        ];
        for variant in variants {
            #[cfg(feature = "alloc")]
            assert!(!variant.to_string().is_empty());
            #[cfg(not(feature = "alloc"))]
            let _ = variant;
        }
    }
}
