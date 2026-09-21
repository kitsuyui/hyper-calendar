//! Errors raised while converting between calendars.

use core::fmt;

/// Result alias for calendar operations.
pub type CalendarResult<T> = Result<T, CalendarError>;

/// What can go wrong when a set of date fields meets a calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CalendarError {
    /// The year is outside the range this calendar can represent.
    YearOutOfRange,
    /// The month index does not exist in this year of this calendar.
    ///
    /// Lunisolar calendars make this year-dependent: a leap month exists in
    /// some years and not others.
    MonthOutOfRange,
    /// The day index does not exist in this month of this year.
    DayOutOfRange,
    /// A required field was not supplied.
    MissingField(&'static str),
    /// A field was supplied that this calendar has no meaning for.
    UnsupportedField(&'static str),
    /// The date predates the calendar's epoch or its historical adoption.
    BeforeEpoch,
    /// The date lies beyond where this calendar's data or rules are defined.
    AfterSupportedRange,
    /// The era name or code is not one this calendar knows.
    UnknownEra,
    /// The requested calendar is not registered.
    UnknownCalendar,
    /// Day arithmetic left the representable range.
    Overflow,
    /// The underlying astronomical model could not produce an answer.
    AstronomicalModelFailure,
}

impl fmt::Display for CalendarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::YearOutOfRange => f.write_str("year out of range for this calendar"),
            Self::MonthOutOfRange => f.write_str("month out of range for this year"),
            Self::DayOutOfRange => f.write_str("day out of range for this month"),
            Self::MissingField(name) => write!(f, "missing required field: {name}"),
            Self::UnsupportedField(name) => {
                write!(f, "field not supported by this calendar: {name}")
            }
            Self::BeforeEpoch => f.write_str("date precedes this calendar's epoch"),
            Self::AfterSupportedRange => f.write_str("date beyond this calendar's supported range"),
            Self::UnknownEra => f.write_str("unknown era"),
            Self::UnknownCalendar => f.write_str("unknown calendar"),
            Self::Overflow => f.write_str("day arithmetic overflowed"),
            Self::AstronomicalModelFailure => {
                f.write_str("the astronomical model could not produce an answer")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CalendarError {}

impl From<hc_core::TimeError> for CalendarError {
    fn from(value: hc_core::TimeError) -> Self {
        match value {
            hc_core::TimeError::Overflow => Self::Overflow,
            hc_core::TimeError::OutOfRange => Self::DayOutOfRange,
            hc_core::TimeError::BeforeModelStart => Self::BeforeEpoch,
            hc_core::TimeError::AfterModelEnd => Self::AfterSupportedRange,
            _ => Self::AstronomicalModelFailure,
        }
    }
}
