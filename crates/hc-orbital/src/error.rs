//! Errors raised while evaluating the orbital series.

use core::fmt;

use hc_uncertainty::UncertaintyError;

/// Result alias for the orbital layer.
pub type OrbitalResult<T> = Result<T, OrbitalError>;

/// What can go wrong when the series is asked for a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum OrbitalError {
    /// The requested epoch is not a finite number of years.
    NotFinite,
    /// The requested epoch lies outside [`crate::VALID_SPAN`].
    ///
    /// The series is a fit, and a trigonometric fit does not degrade
    /// gracefully: past its span it keeps returning plausible-looking
    /// numbers that are simply wrong. The library refuses instead
    /// (ADR 0006). A caller who needs elements further back needs a longer
    /// numerical solution, such as Laskar et al. (2004), which this crate
    /// names and does not carry.
    OutsideValidSpan,
    /// A latitude outside −90° to +90° was asked for insolation.
    LatitudeOutOfRange,
    /// The uncertainty layer refused the operation.
    Uncertainty(UncertaintyError),
}

impl fmt::Display for OrbitalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFinite => f.write_str("epoch is not finite"),
            Self::OutsideValidSpan => f.write_str(
                "epoch is outside the million years either side of 1950 over which Berger's 1978 series holds",
            ),
            Self::LatitudeOutOfRange => f.write_str("latitude is outside -90 to +90 degrees"),
            Self::Uncertainty(inner) => write!(f, "uncertainty layer: {inner}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for OrbitalError {}

impl From<UncertaintyError> for OrbitalError {
    fn from(value: UncertaintyError) -> Self {
        match value {
            UncertaintyError::NotFinite => Self::NotFinite,
            other => Self::Uncertainty(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    use alloc::string::ToString as _;

    #[test]
    fn a_non_finite_uncertainty_keeps_its_meaning_across_the_boundary() {
        assert_eq!(
            OrbitalError::from(UncertaintyError::NotFinite),
            OrbitalError::NotFinite
        );
        assert_eq!(
            OrbitalError::from(UncertaintyError::DivideByZero),
            OrbitalError::Uncertainty(UncertaintyError::DivideByZero)
        );
    }

    #[test]
    fn every_variant_renders_a_non_empty_message() {
        let variants = [
            OrbitalError::NotFinite,
            OrbitalError::OutsideValidSpan,
            OrbitalError::LatitudeOutOfRange,
            OrbitalError::Uncertainty(UncertaintyError::DivideByZero),
        ];
        for variant in variants {
            #[cfg(feature = "alloc")]
            assert!(!variant.to_string().is_empty());
            #[cfg(not(feature = "alloc"))]
            let _ = variant;
        }
    }
}
