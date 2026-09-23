//! Errors raised by the relativistic calculations.

use core::fmt;

use hc_uncertainty::UncertaintyError;

/// Result alias for the relativity layer.
pub type RelativityResult<T> = Result<T, RelativityError>;

/// What can go wrong when a relativistic quantity is computed.
///
/// The point of this type is that none of these conditions is allowed to
/// become a `NaN`. A Lorentz factor at β = 1 is a division by zero, and a
/// Schwarzschild factor inside the horizon is the square root of a negative
/// number; both would propagate silently through a whole voyage calculation
/// and surface as a nonsensical arrival date. Each is caught where it
/// happens and named.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RelativityError {
    /// An input or a result was NaN or infinite.
    NotFinite,
    /// A speed reached or exceeded `c`.
    ///
    /// Nothing with mass can be given `|β| ≥ 1`, so this is a bad input
    /// rather than an extreme answer.
    FasterThanLight,
    /// A radius was zero or negative.
    NonPositiveRadius,
    /// A mass or standard gravitational parameter was negative.
    NegativeMass,
    /// A proper acceleration was zero or negative.
    ///
    /// The hyperbolic-motion formulas divide by it, and a burn from rest at
    /// no acceleration goes nowhere.
    NonPositiveAcceleration,
    /// A distance was negative.
    NegativeDistance,
    /// A Lorentz factor was below 1, which no real motion produces.
    LorentzFactorBelowOne,
    /// A cosine lay outside `[−1, 1]`.
    CosineOutOfRange,
    /// The radius lies at or inside the Schwarzschild radius, where the
    /// static time-dilation factor is undefined.
    ///
    /// No static observer can exist there, so there is no clock to compare.
    InsideHorizon,
    /// Exact `hc_core::Duration` arithmetic left the representable range.
    Overflow,
    /// A worldline segment had a negative coordinate duration.
    ///
    /// A worldline is traversed forwards; a negative span would be a
    /// different problem than the one this crate solves.
    NegativeDuration,
    /// An uncertain input could not be propagated.
    Uncertainty(UncertaintyError),
}

impl fmt::Display for RelativityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFinite => f.write_str("value is not finite"),
            Self::FasterThanLight => f.write_str("speed reached or exceeded the speed of light"),
            Self::NonPositiveRadius => f.write_str("radius must be strictly positive"),
            Self::NegativeMass => f.write_str("mass must not be negative"),
            Self::NonPositiveAcceleration => {
                f.write_str("proper acceleration must be strictly positive")
            }
            Self::NegativeDistance => f.write_str("distance must not be negative"),
            Self::LorentzFactorBelowOne => f.write_str("Lorentz factor must be at least 1"),
            Self::CosineOutOfRange => f.write_str("cosine must lie between -1 and 1"),
            Self::InsideHorizon => f.write_str("radius is at or inside the Schwarzschild radius"),
            Self::Overflow => f.write_str("duration arithmetic overflowed"),
            Self::NegativeDuration => f.write_str("a worldline segment ran backwards"),
            Self::Uncertainty(inner) => write!(f, "uncertainty propagation failed: {inner}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for RelativityError {}

impl From<hc_core::TimeError> for RelativityError {
    fn from(value: hc_core::TimeError) -> Self {
        match value {
            hc_core::TimeError::Overflow => Self::Overflow,
            hc_core::TimeError::NotFinite => Self::NotFinite,
            _ => Self::NotFinite,
        }
    }
}

impl From<UncertaintyError> for RelativityError {
    fn from(value: UncertaintyError) -> Self {
        Self::Uncertainty(value)
    }
}

/// Reject a non-finite input before it can turn into a `NaN` answer.
///
/// # Errors
///
/// Returns [`RelativityError::NotFinite`] for NaN or infinity.
pub(crate) fn finite(value: f64) -> RelativityResult<f64> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(RelativityError::NotFinite)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_core_overflow_stays_an_overflow() {
        assert_eq!(
            RelativityError::from(hc_core::TimeError::Overflow),
            RelativityError::Overflow
        );
    }

    #[test]
    fn an_uncertainty_failure_is_carried_rather_than_flattened() {
        let inner = UncertaintyError::NegativeUncertainty;
        assert_eq!(
            RelativityError::from(inner),
            RelativityError::Uncertainty(inner)
        );
    }

    #[test]
    fn non_finite_inputs_are_rejected_at_the_gate() {
        assert_eq!(finite(f64::NAN), Err(RelativityError::NotFinite));
        assert_eq!(finite(f64::INFINITY), Err(RelativityError::NotFinite));
        assert_eq!(finite(1.0), Ok(1.0));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn every_variant_renders_a_message() {
        use alloc::string::ToString as _;
        let variants = [
            RelativityError::NotFinite,
            RelativityError::FasterThanLight,
            RelativityError::NonPositiveRadius,
            RelativityError::NegativeMass,
            RelativityError::NonPositiveAcceleration,
            RelativityError::NegativeDistance,
            RelativityError::LorentzFactorBelowOne,
            RelativityError::CosineOutOfRange,
            RelativityError::InsideHorizon,
            RelativityError::Overflow,
            RelativityError::NegativeDuration,
            RelativityError::Uncertainty(UncertaintyError::NotFinite),
        ];
        for variant in variants {
            assert!(!variant.to_string().is_empty());
        }
    }
}
