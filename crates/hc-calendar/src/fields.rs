//! Calendar-agnostic date fields.
//!
//! [`DateFields`] is the currency of the dynamic interface: a bag of named
//! numbers that any calendar can fill in and any calendar can read back. It
//! exists so that a registry, an FFI caller or a formatter can move dates
//! between calendars it was never compiled against.
//!
//! The bag is deliberately small and fixed-capacity so that `no_std` targets
//! without an allocator can still use it.

use core::fmt;

use crate::error::{CalendarError, CalendarResult};

/// How many calendar-specific extra fields a date can carry.
///
/// Five covers the Maya long count (baktun, katun, tun, uinal, kin); eight
/// leaves room for the Balinese Pawukon's concurrent week cycles without
/// forcing an allocation.
pub const MAX_EXTRA_FIELDS: usize = 8;

/// A month within a year.
///
/// Lunisolar calendars insert a thirteenth month in some years, and they name
/// it after the month it follows rather than giving it its own ordinal — the
/// Chinese leap fourth month is "闰四月", not "month 13". Carrying the leap
/// flag separately from the ordinal is what lets a single type describe both
/// a Gregorian and a Chinese month.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Month {
    /// The month's ordinal within the year, counting from 1.
    pub ordinal: u8,
    /// Whether this is the intercalary repetition of `ordinal`.
    pub leap: bool,
}

impl Month {
    /// An ordinary month.
    #[must_use]
    pub const fn regular(ordinal: u8) -> Self {
        Self {
            ordinal,
            leap: false,
        }
    }

    /// An intercalary month repeating `ordinal`.
    #[must_use]
    pub const fn leap(ordinal: u8) -> Self {
        Self {
            ordinal,
            leap: true,
        }
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.leap {
            write!(f, "leap {}", self.ordinal)
        } else {
            write!(f, "{}", self.ordinal)
        }
    }
}

/// How a calendar counts its years.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum YearKind {
    /// Years run continuously and can be negative, with no year zero skipped.
    ///
    /// This is the astronomical convention: 1 BC is year 0, 2 BC is year -1.
    Astronomical,
    /// Years are counted within an era and restart when the era changes.
    ///
    /// Japanese nengō and the Gregorian BC/AD split both work this way.
    EraRelative,
    /// Years count forward from an epoch and are never negative.
    ///
    /// The Hijri and Holocene calendars are like this in practice.
    EpochForward,
}

/// One calendar-specific extra field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtraField {
    /// A stable machine name, for example `"baktun"` or `"pancawara"`.
    pub name: &'static str,
    /// The field's value.
    pub value: i64,
}

/// A fixed-capacity set of calendar-specific extra fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ExtraFields {
    entries: [Option<ExtraField>; MAX_EXTRA_FIELDS],
}

impl ExtraFields {
    /// An empty set.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: [None; MAX_EXTRA_FIELDS],
        }
    }

    /// Add or replace a field.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::Overflow`] when the set is full.
    pub fn set(&mut self, name: &'static str, value: i64) -> CalendarResult<()> {
        for slot in &mut self.entries {
            match slot {
                Some(existing) if existing.name == name => {
                    existing.value = value;
                    return Ok(());
                }
                _ => {}
            }
        }
        for slot in &mut self.entries {
            if slot.is_none() {
                *slot = Some(ExtraField { name, value });
                return Ok(());
            }
        }
        Err(CalendarError::Overflow)
    }

    /// Look a field up by name.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<i64> {
        self.entries
            .iter()
            .flatten()
            .find(|field| field.name == name)
            .map(|field| field.value)
    }

    /// Look a field up, failing when it is absent.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::MissingField`] when the field is not present.
    pub fn require(&self, name: &'static str) -> CalendarResult<i64> {
        self.get(name).ok_or(CalendarError::MissingField(name))
    }

    /// Every field present, in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = ExtraField> + '_ {
        self.entries.iter().flatten().copied()
    }

    /// How many fields are set.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.iter().flatten().count()
    }

    /// Whether no fields are set.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A date expressed as named fields, independent of any particular calendar.
///
/// Not every calendar fills in every field. The Maya long count has no month
/// or day; the ISO week date has a week number instead of a month. Absent
/// fields are `None` rather than a sentinel value, so that "this calendar has
/// no months" and "the month is January" never collide.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateFields {
    /// The era code, when the calendar counts years within eras.
    pub era: Option<&'static str>,
    /// The year, interpreted according to the calendar's [`YearKind`].
    pub year: i64,
    /// The month, when the calendar has months.
    pub month: Option<Month>,
    /// The day within the month, counting from 1.
    pub day: Option<u8>,
    /// Calendar-specific fields.
    pub extra: ExtraFields,
}

impl Default for DateFields {
    fn default() -> Self {
        Self::new(0)
    }
}

impl DateFields {
    /// A date with only a year.
    #[must_use]
    pub const fn new(year: i64) -> Self {
        Self {
            era: None,
            year,
            month: None,
            day: None,
            extra: ExtraFields::new(),
        }
    }

    /// The common year-month-day shape.
    #[must_use]
    pub const fn ymd(year: i64, month: u8, day: u8) -> Self {
        Self {
            era: None,
            year,
            month: Some(Month::regular(month)),
            day: Some(day),
            extra: ExtraFields::new(),
        }
    }

    /// The year-month-day shape with an intercalary month.
    #[must_use]
    pub const fn ymd_leap_month(year: i64, month: u8, day: u8) -> Self {
        Self {
            era: None,
            year,
            month: Some(Month::leap(month)),
            day: Some(day),
            extra: ExtraFields::new(),
        }
    }

    /// The same date tagged with an era.
    #[must_use]
    pub const fn with_era(mut self, era: &'static str) -> Self {
        self.era = Some(era);
        self
    }

    /// The same date with one extra field added.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::Overflow`] when the extra-field set is full.
    pub fn with_extra(mut self, name: &'static str, value: i64) -> CalendarResult<Self> {
        self.extra.set(name, value)?;
        Ok(self)
    }

    /// The month, failing when the calendar requires one and it is absent.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::MissingField`] when there is no month.
    pub fn require_month(&self) -> CalendarResult<Month> {
        self.month.ok_or(CalendarError::MissingField("month"))
    }

    /// The day, failing when the calendar requires one and it is absent.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::MissingField`] when there is no day.
    pub fn require_day(&self) -> CalendarResult<u8> {
        self.day.ok_or(CalendarError::MissingField("day"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leap_months_keep_their_ordinal() {
        let leap = Month::leap(4);
        assert_eq!(leap.ordinal, 4);
        assert!(leap.leap);
        assert_ne!(leap, Month::regular(4));
        assert_eq!(leap.to_string(), "leap 4");
    }

    #[test]
    fn extra_fields_store_and_replace() {
        let mut extra = ExtraFields::new();
        extra.set("baktun", 13).unwrap();
        extra.set("katun", 0).unwrap();
        assert_eq!(extra.get("baktun"), Some(13));
        extra.set("baktun", 14).unwrap();
        assert_eq!(extra.get("baktun"), Some(14));
        assert_eq!(extra.len(), 2);
    }

    #[test]
    fn extra_fields_report_when_full() {
        let mut extra = ExtraFields::new();
        for index in 0..MAX_EXTRA_FIELDS {
            let name: &'static str = ["a", "b", "c", "d", "e", "f", "g", "h"][index];
            extra.set(name, index as i64).unwrap();
        }
        assert_eq!(extra.set("i", 0), Err(CalendarError::Overflow));
    }

    #[test]
    fn missing_fields_are_named_in_the_error() {
        let fields = DateFields::new(2026);
        assert_eq!(
            fields.require_month(),
            Err(CalendarError::MissingField("month"))
        );
        assert_eq!(
            fields.require_day(),
            Err(CalendarError::MissingField("day"))
        );
        assert_eq!(
            fields.extra.require("kin"),
            Err(CalendarError::MissingField("kin"))
        );
    }

    #[test]
    fn builders_compose() {
        let fields = DateFields::ymd(5, 4, 29)
            .with_era("reiwa")
            .with_extra("cycle", 60)
            .unwrap();
        assert_eq!(fields.era, Some("reiwa"));
        assert_eq!(fields.month, Some(Month::regular(4)));
        assert_eq!(fields.day, Some(29));
        assert_eq!(fields.extra.get("cycle"), Some(60));
    }
}
