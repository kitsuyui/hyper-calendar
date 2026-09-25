//! Errors raised while converting between calendars.

use core::fmt;

/// Result alias for calendar operations.
pub type CalendarResult<T> = Result<T, CalendarError>;

/// What can go wrong when a set of date fields meets a calendar.
///
/// Every variant has a stable numeric code and a stable name, listed beside
/// it and returned by [`CalendarError::code`] and [`CalendarError::name`].
/// They exist so that a refusal can cross an ABI — the C library, the
/// WebAssembly module — as a number and a word rather than as a Rust value,
/// and a caller on the other side can match on either. A code is never
/// reused: a variant added later takes the next number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CalendarError {
    /// The year is outside the range this calendar can represent.
    ///
    /// Code 1, `year-out-of-range`.
    YearOutOfRange,
    /// The month index does not exist in this year of this calendar.
    ///
    /// Lunisolar calendars make this year-dependent: a leap month exists in
    /// some years and not others.
    ///
    /// Code 2, `month-out-of-range`.
    MonthOutOfRange,
    /// The day index does not exist in this month of this year.
    ///
    /// Code 3, `day-out-of-range`.
    DayOutOfRange,
    /// A required field was not supplied.
    ///
    /// Code 4, `missing-field`.
    MissingField(&'static str),
    /// A field was supplied that this calendar has no meaning for.
    ///
    /// Code 5, `unsupported-field`.
    UnsupportedField(&'static str),
    /// The date predates the calendar's epoch or its historical adoption.
    ///
    /// Code 6, `before-epoch`.
    BeforeEpoch,
    /// The date lies beyond where this calendar's data or rules are defined.
    ///
    /// Code 7, `after-supported-range`.
    AfterSupportedRange,
    /// The era name or code is not one this calendar knows.
    ///
    /// Code 8, `unknown-era`.
    UnknownEra,
    /// The requested calendar is not registered.
    ///
    /// Code 9, `unknown-calendar`.
    UnknownCalendar,
    /// Day arithmetic left the representable range.
    ///
    /// Code 10, `overflow`.
    Overflow,
    /// The underlying astronomical model could not produce an answer.
    ///
    /// Code 11, `astronomical-model-failure`.
    AstronomicalModelFailure,
}

impl CalendarError {
    /// The stable numeric code of this error, from 1 upwards.
    ///
    /// The codes are listed on the variants and never change or move; they
    /// are what a refusal is reported as across an ABI.
    #[must_use]
    pub const fn code(&self) -> u32 {
        match self {
            Self::YearOutOfRange => 1,
            Self::MonthOutOfRange => 2,
            Self::DayOutOfRange => 3,
            Self::MissingField(_) => 4,
            Self::UnsupportedField(_) => 5,
            Self::BeforeEpoch => 6,
            Self::AfterSupportedRange => 7,
            Self::UnknownEra => 8,
            Self::UnknownCalendar => 9,
            Self::Overflow => 10,
            Self::AstronomicalModelFailure => 11,
        }
    }

    /// The stable name of this error: the variant, lower-case and
    /// hyphenated, without the field name a `MissingField` or
    /// `UnsupportedField` carries.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::YearOutOfRange => "year-out-of-range",
            Self::MonthOutOfRange => "month-out-of-range",
            Self::DayOutOfRange => "day-out-of-range",
            Self::MissingField(_) => "missing-field",
            Self::UnsupportedField(_) => "unsupported-field",
            Self::BeforeEpoch => "before-epoch",
            Self::AfterSupportedRange => "after-supported-range",
            Self::UnknownEra => "unknown-era",
            Self::UnknownCalendar => "unknown-calendar",
            Self::Overflow => "overflow",
            Self::AstronomicalModelFailure => "astronomical-model-failure",
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    const EVERY: [CalendarError; 11] = [
        CalendarError::YearOutOfRange,
        CalendarError::MonthOutOfRange,
        CalendarError::DayOutOfRange,
        CalendarError::MissingField("month"),
        CalendarError::UnsupportedField("era"),
        CalendarError::BeforeEpoch,
        CalendarError::AfterSupportedRange,
        CalendarError::UnknownEra,
        CalendarError::UnknownCalendar,
        CalendarError::Overflow,
        CalendarError::AstronomicalModelFailure,
    ];

    #[test]
    fn every_variant_has_a_distinct_code_from_one_upwards() {
        let codes: Vec<u32> = EVERY.iter().map(CalendarError::code).collect();
        assert_eq!(codes, (1..=11).collect::<Vec<u32>>());
    }

    #[test]
    fn every_variant_has_a_distinct_lower_case_hyphenated_name() {
        let mut names: Vec<&str> = EVERY.iter().map(CalendarError::name).collect();
        assert!(names.iter().all(|name| {
            !name.is_empty()
                && name
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte == b'-')
        }));
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), EVERY.len());
    }

    #[test]
    fn the_codes_are_the_documented_ones() {
        // The numbers are part of the ABI contract, so they are pinned
        // here rather than derived from declaration order.
        assert_eq!(CalendarError::BeforeEpoch.code(), 6);
        assert_eq!(CalendarError::BeforeEpoch.name(), "before-epoch");
        assert_eq!(CalendarError::AfterSupportedRange.code(), 7);
        assert_eq!(CalendarError::MissingField("day").code(), 4);
        assert_eq!(CalendarError::MissingField("day").name(), "missing-field");
        assert_eq!(CalendarError::AstronomicalModelFailure.code(), 11);
    }
}
