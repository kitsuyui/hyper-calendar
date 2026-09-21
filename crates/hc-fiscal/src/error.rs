//! Errors raised while resolving a year that does not begin on 1 January.

use core::fmt;

use hc_calendar::CalendarError;

/// Result alias for fiscal-year operations.
pub type FiscalResult<T> = Result<T, FiscalError>;

/// What can go wrong when a date meets a [`YearSystem`](crate::YearSystem).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FiscalError {
    /// The underlying calendar refused the conversion.
    ///
    /// Every start date in this crate is a month and a day *in a named
    /// calendar*, so a Solar Hijri or Ethiopic year outside that calendar's
    /// supported range surfaces here rather than being silently clamped.
    Calendar(CalendarError),
    /// The year label lies outside the range over which this system was in
    /// force.
    ///
    /// The United States federal fiscal year is the motivating case: asking
    /// the October system for FY1970 is a question about a system that did
    /// not exist yet, and the July system is the one that answers it.
    OutsideValidity,
    /// The system's start calendar does not divide a year into twelve months
    /// of equal standing, so fiscal months, quarters and halves are not
    /// defined for it.
    ///
    /// Raised only by the Ethiopic calendar, whose year is twelve months of
    /// thirty days followed by Pagumen. This crate declines to say which
    /// quarter Pagumen belongs to rather than invent an answer.
    PeriodsNotDefined,
    /// No system of the requested kind was in force in the requested year.
    NoSystemInForce,
    /// Day arithmetic left the representable range.
    Overflow,
}

impl fmt::Display for FiscalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Calendar(inner) => write!(f, "calendar error: {inner}"),
            Self::OutsideValidity => {
                f.write_str("year label outside the range this system was in force")
            }
            Self::PeriodsNotDefined => {
                f.write_str("this start calendar has no twelve equal months, so fiscal months, quarters and halves are undefined")
            }
            Self::NoSystemInForce => {
                f.write_str("no year system of that kind was in force in that year")
            }
            Self::Overflow => f.write_str("day arithmetic overflowed"),
        }
    }
}

impl From<CalendarError> for FiscalError {
    fn from(value: CalendarError) -> Self {
        Self::Calendar(value)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FiscalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Calendar(inner) => Some(inner),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_calendar_error_is_carried_rather_than_flattened() {
        let error = FiscalError::from(CalendarError::MonthOutOfRange);
        assert_eq!(error, FiscalError::Calendar(CalendarError::MonthOutOfRange));
    }

    #[cfg(feature = "std")]
    #[test]
    fn every_error_renders_a_distinct_message() {
        use std::collections::BTreeSet;

        let messages: BTreeSet<_> = [
            FiscalError::Calendar(CalendarError::BeforeEpoch),
            FiscalError::OutsideValidity,
            FiscalError::PeriodsNotDefined,
            FiscalError::NoSystemInForce,
            FiscalError::Overflow,
        ]
        .iter()
        .map(ToString::to_string)
        .collect();
        assert_eq!(messages.len(), 5);
    }
}
