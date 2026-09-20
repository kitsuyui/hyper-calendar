//! Errors raised while manipulating uncertain quantities.

use core::fmt;

/// Result alias for the uncertainty layer.
pub type UncertaintyResult<T> = Result<T, UncertaintyError>;

/// What can go wrong when a value that is only partly known is manipulated.
///
/// The variants separate three genuinely different failures: an input that was
/// never a legal uncertain value (a negative standard deviation), an operation
/// whose mathematics is undefined for the inputs given (the logarithm of a
/// non-positive number), and a request for an answer the data cannot support
/// (the intersection of two disjoint intervals). Collapsing them into one
/// "invalid" variant would hide which of the three a caller has to fix.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum UncertaintyError {
    /// An input or a result was NaN or infinite.
    NotFinite,
    /// A standard deviation, a spread or a resolution was negative.
    NegativeUncertainty,
    /// A resolution or a weight was zero where a positive value is required.
    NonPositive,
    /// A division by a quantity whose central value is zero was requested.
    DivideByZero,
    /// The operation is mathematically undefined for these inputs.
    ///
    /// First-order error propagation needs a differentiable function at the
    /// central value, so `ln` of a non-positive number and a fractional power
    /// of a negative number land here rather than silently producing NaN.
    OutOfDomain,
    /// An exact `hc_core::Duration` operation left the representable range.
    Overflow,
    /// The interval has no members, so the requested property does not exist.
    EmptyInterval,
    /// Two intervals do not meet, so their intersection is empty.
    Disjoint,
    /// A significant-figure count was outside the range `f64` can carry.
    ///
    /// A binary double holds between 15 and 17 decimal digits, so claiming
    /// more than 17 would be claiming precision the representation does not
    /// have, and claiming zero would be claiming no precision at all.
    InvalidSignificantFigures,
    /// The input was not well-formed. The payload names the expected shape.
    InvalidSyntax(&'static str),
    /// The input is legal but this build cannot represent it.
    ///
    /// EDTF's set and list forms need a growable collection, so they are only
    /// available with the `alloc` feature.
    Unsupported(&'static str),
}

impl fmt::Display for UncertaintyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFinite => f.write_str("value is not finite"),
            Self::NegativeUncertainty => f.write_str("uncertainty must not be negative"),
            Self::NonPositive => f.write_str("value must be strictly positive"),
            Self::DivideByZero => f.write_str("division by zero"),
            Self::OutOfDomain => f.write_str("operation is undefined for these inputs"),
            Self::Overflow => f.write_str("duration arithmetic overflowed"),
            Self::EmptyInterval => f.write_str("the interval is empty"),
            Self::Disjoint => f.write_str("the intervals do not overlap"),
            Self::InvalidSignificantFigures => {
                f.write_str("significant figures must be between 1 and 17")
            }
            Self::InvalidSyntax(expected) => write!(f, "malformed input, expected {expected}"),
            Self::Unsupported(what) => write!(f, "unsupported in this build: {what}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for UncertaintyError {}

impl From<hc_core::TimeError> for UncertaintyError {
    fn from(value: hc_core::TimeError) -> Self {
        match value {
            hc_core::TimeError::Overflow => Self::Overflow,
            hc_core::TimeError::DivideByZero => Self::DivideByZero,
            hc_core::TimeError::NotFinite => Self::NotFinite,
            _ => Self::OutOfDomain,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overflow_from_the_core_layer_stays_overflow() {
        let converted = UncertaintyError::from(hc_core::TimeError::Overflow);
        assert_eq!(converted, UncertaintyError::Overflow);
    }

    #[test]
    fn every_variant_renders_a_non_empty_message() {
        let variants = [
            UncertaintyError::NotFinite,
            UncertaintyError::NegativeUncertainty,
            UncertaintyError::NonPositive,
            UncertaintyError::DivideByZero,
            UncertaintyError::OutOfDomain,
            UncertaintyError::Overflow,
            UncertaintyError::EmptyInterval,
            UncertaintyError::Disjoint,
            UncertaintyError::InvalidSignificantFigures,
            UncertaintyError::InvalidSyntax("a year"),
            UncertaintyError::Unsupported("sets"),
        ];
        for variant in variants {
            #[cfg(feature = "alloc")]
            {
                use alloc::string::ToString as _;
                assert!(!variant.to_string().is_empty());
            }
            #[cfg(not(feature = "alloc"))]
            {
                let _ = variant;
            }
        }
    }
}
