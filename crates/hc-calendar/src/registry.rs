//! A run-time registry of calendars.
//!
//! The registry is what makes requirement 4 usable from an application: it
//! holds any number of calendars behind one interface, so a UI can render the
//! same day in Gregorian, Hijri and Japanese-era form without any of the
//! three knowing the others exist.
//!
//! It needs an allocator, so it lives behind the `alloc` feature.

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::error::{CalendarError, CalendarResult};
use crate::fields::DateFields;
use crate::fixed::Rd;
use crate::traits::{CalendarId, CalendarMeta, DynCalendar};

/// A collection of calendars addressable by [`CalendarId`].
#[derive(Default)]
pub struct CalendarRegistry {
    entries: Vec<(CalendarId, Box<dyn DynCalendar + Send + Sync>)>,
}

impl core::fmt::Debug for CalendarRegistry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CalendarRegistry")
            .field("len", &self.entries.len())
            .finish()
    }
}

impl CalendarRegistry {
    /// An empty registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Register a calendar, replacing any calendar already under its id.
    pub fn insert(&mut self, calendar: Box<dyn DynCalendar + Send + Sync>) {
        let id = calendar.meta().id;
        if let Some(slot) = self.entries.iter_mut().find(|(known, _)| *known == id) {
            slot.1 = calendar;
        } else {
            self.entries.push((id, calendar));
        }
    }

    /// Look a calendar up.
    #[must_use]
    pub fn get(&self, id: CalendarId) -> Option<&(dyn DynCalendar + Send + Sync)> {
        self.entries
            .iter()
            .find(|(known, _)| *known == id)
            .map(|(_, calendar)| calendar.as_ref())
    }

    /// Look a calendar up by its identifier string.
    #[must_use]
    pub fn get_by_name(&self, name: &str) -> Option<&(dyn DynCalendar + Send + Sync)> {
        self.entries
            .iter()
            .find(|(known, _)| known.as_str() == name)
            .map(|(_, calendar)| calendar.as_ref())
    }

    /// Every registered calendar's metadata.
    pub fn metas(&self) -> impl Iterator<Item = CalendarMeta> + '_ {
        self.entries.iter().map(|(_, calendar)| calendar.meta())
    }

    /// How many calendars are registered.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Render one fixed day in every registered calendar, in registry
    /// order.
    ///
    /// Every calendar answers, and a calendar that cannot represent the day
    /// answers with its refusal rather than being left out: asking for
    /// "today in every calendar" should not break because one of them
    /// starts in 1873, and it should not pretend that calendar does not
    /// exist either. "This calendar was not in use" is an answer a reader
    /// wants to see next to the ones that converted, which is why the
    /// refusal is carried as a value and not filtered away here.
    #[must_use]
    pub fn describe_day(&self, rd: Rd) -> Vec<(CalendarId, CalendarResult<DateFields>)> {
        self.entries
            .iter()
            .map(|(id, calendar)| (*id, calendar.fixed_to_fields(rd)))
            .collect()
    }

    /// Convert a date from one calendar to another by name.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::UnknownCalendar`] when either calendar is not
    /// registered, or the underlying conversion error.
    pub fn convert(
        &self,
        from: CalendarId,
        fields: &DateFields,
        to: CalendarId,
    ) -> CalendarResult<DateFields> {
        let source = self.get(from).ok_or(CalendarError::UnknownCalendar)?;
        let target = self.get(to).ok_or(CalendarError::UnknownCalendar)?;
        target.fixed_to_fields(source.fields_to_fixed(fields)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::YearKind;

    struct OffsetCalendar {
        id: CalendarId,
        offset: i64,
    }

    impl DynCalendar for OffsetCalendar {
        fn meta(&self) -> CalendarMeta {
            CalendarMeta {
                id: self.id,
                english_name: "Offset test calendar",
                year_kind: YearKind::Astronomical,
                has_leap_months: false,
                is_astronomical: false,
                earliest: None,
                latest: None,
                native_locales: &[],
            }
        }

        fn fields_to_fixed(&self, fields: &DateFields) -> CalendarResult<Rd> {
            Ok(Rd(fields.year + self.offset))
        }

        fn fixed_to_fields(&self, rd: Rd) -> CalendarResult<DateFields> {
            Ok(DateFields::new(rd.0 - self.offset))
        }

        fn days_in_year(&self, _year: i64) -> CalendarResult<u16> {
            Ok(1)
        }

        fn is_leap_year(&self, _year: i64) -> CalendarResult<bool> {
            Ok(false)
        }

        fn cycles(&self) -> &'static [crate::shape::CycleShape] {
            &[]
        }
    }

    fn registry() -> CalendarRegistry {
        let mut registry = CalendarRegistry::new();
        registry.insert(Box::new(OffsetCalendar {
            id: CalendarId("zero"),
            offset: 0,
        }));
        registry.insert(Box::new(OffsetCalendar {
            id: CalendarId("hundred"),
            offset: 100,
        }));
        registry
    }

    #[test]
    fn calendars_are_addressable_by_id_and_by_name() {
        let registry = registry();
        assert_eq!(registry.len(), 2);
        assert!(registry.get(CalendarId("zero")).is_some());
        assert!(registry.get_by_name("hundred").is_some());
        assert!(registry.get_by_name("missing").is_none());
    }

    #[test]
    fn inserting_the_same_id_replaces_rather_than_duplicates() {
        let mut registry = registry();
        registry.insert(Box::new(OffsetCalendar {
            id: CalendarId("zero"),
            offset: 7,
        }));
        assert_eq!(registry.len(), 2);
        let fields = registry
            .get(CalendarId("zero"))
            .unwrap()
            .fixed_to_fields(Rd(7))
            .unwrap();
        assert_eq!(fields.year, 0);
    }

    #[test]
    fn one_day_renders_in_every_calendar_at_once() {
        let registry = registry();
        let rendered = registry.describe_day(Rd(500));
        assert_eq!(rendered.len(), 2);
        let years: Vec<i64> = rendered
            .iter()
            .map(|(_, fields)| fields.as_ref().map(|fields| fields.year).unwrap())
            .collect();
        assert_eq!(years, [500, 400]);
    }

    #[test]
    fn a_calendar_that_refuses_the_day_is_listed_with_its_refusal() {
        struct Bounded;

        impl DynCalendar for Bounded {
            fn meta(&self) -> CalendarMeta {
                CalendarMeta {
                    id: CalendarId("bounded"),
                    english_name: "Bounded test calendar",
                    year_kind: YearKind::Astronomical,
                    has_leap_months: false,
                    is_astronomical: false,
                    earliest: Some(Rd(1_000)),
                    latest: None,
                    native_locales: &[],
                }
            }

            fn fields_to_fixed(&self, fields: &DateFields) -> CalendarResult<Rd> {
                Ok(Rd(fields.year))
            }

            fn fixed_to_fields(&self, rd: Rd) -> CalendarResult<DateFields> {
                self.meta().check_range(rd)?;
                Ok(DateFields::new(rd.0))
            }

            fn days_in_year(&self, _year: i64) -> CalendarResult<u16> {
                Ok(1)
            }

            fn is_leap_year(&self, _year: i64) -> CalendarResult<bool> {
                Ok(false)
            }

            fn cycles(&self) -> &'static [crate::shape::CycleShape] {
                &[]
            }
        }

        let mut registry = registry();
        registry.insert(Box::new(Bounded));
        let rendered = registry.describe_day(Rd(500));
        // Registry order, refusals included, so a caller can lay the rows
        // out without a second call.
        let ids: Vec<&str> = rendered.iter().map(|(id, _)| id.0).collect();
        assert_eq!(ids, ["zero", "hundred", "bounded"]);
        assert_eq!(rendered[2].1, Err(CalendarError::BeforeEpoch));
        assert!(rendered[0].1.is_ok() && rendered[1].1.is_ok());
    }

    #[test]
    fn conversion_goes_through_the_fixed_day() {
        let registry = registry();
        let converted = registry
            .convert(
                CalendarId("zero"),
                &DateFields::new(500),
                CalendarId("hundred"),
            )
            .unwrap();
        assert_eq!(converted.year, 400);
    }

    #[test]
    fn unknown_calendars_are_reported() {
        let registry = registry();
        assert_eq!(
            registry.convert(CalendarId("nope"), &DateFields::new(1), CalendarId("zero")),
            Err(CalendarError::UnknownCalendar)
        );
    }

    #[test]
    fn an_empty_registry_reports_itself_empty() {
        let registry = CalendarRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.describe_day(Rd(0)).len(), 0);
    }
}
