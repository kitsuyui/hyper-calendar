//! Regional, cyclic and era calendars for `hyper-calendar`.
//!
//! The calendars here have one thing in common: **the day has a name before
//! it has a number.** A Maya day is *4 Ahau 8 Cumku* before it is the
//! 1 872 000th day of anything; a Balinese day is *Buda Kliwon Dungulan*,
//! the simultaneous position in three of ten concurrent week cycles; a
//! Japanese day belongs to an era that a government proclaimed. None of them
//! is a count of years from an epoch with months cut out of it, which is why
//! none of them fits in [`hc_calendars_solar`] or [`hc_calendars_lunar`].
//!
//! | Module | Calendars |
//! | --- | --- |
//! | [`japanese`] | `japanese` — imperial eras (和暦), Gregorian from 1873 and lunisolar before it |
//! | [`maya`] | `maya-longcount`, `maya-tzolkin`, `maya-haab`, `maya-round` |
//! | [`aztec`] | `aztec-tonalpohualli`, `aztec-xiuhpohualli` |
//! | [`balinese_pawukon`] | `balinese-pawukon` — ten concurrent week cycles over 210 days |
//! | [`javanese_pasaran`] | `javanese-pasaran` — the five-day market week and the 35-day wetonan |
//! | [`sexagenary`] | `sexagenary` — 干支 over years, months and days |
//!
//! # Cyclic calendars and the round-trip contract
//!
//! [`hc_calendar::Calendar`] demands a bijection between dates and fixed
//! days, and a cycle is by construction not a bijection: the 260-day
//! tzolk'in names the same day every 260 days for ever. Every cyclic
//! calendar here therefore carries a **round number** — how many complete
//! cycles have elapsed since its epoch — alongside the position within the
//! cycle. `13.0.0.0.0` is a long count; *4 Ahau* is a position;
//! `(round, 4 Ahau)` is a date.
//!
//! The round number lands in [`hc_calendar::DateFields::year`], because that
//! is the only field every calendar has and because the round really is the
//! coarsest unit these calendars count in. Where a calendar has something
//! more year-like — the Haab's 365 days, the Pawukon's 30 *wuku* — that goes
//! in the month and day fields, and the concurrent cycles go in
//! [`hc_calendar::ExtraFields`].
//!
//! # What this crate refuses to do
//!
//! It will not guess. Where a Japanese era's start date is disputed the data
//! says so; where only the month is known the day is absent rather than
//! invented; and where the Northern and Southern Courts ran two era systems
//! at once, [`nengo::era_at`] asks which court you mean instead of picking
//! one. Where a correlation constant is a scholarly choice rather than a
//! fact — the Maya GMT constant, the Aztec correlation — the module names
//! the constant it uses, names the alternative, and says what the difference
//! costs.
//!
//! # Example
//!
//! ```
//! use hc_calendar::Calendar;
//! use hc_calendars_regional::{JapaneseCalendar, MayaLongCountCalendar};
//! use hc_calendars_solar::gregorian;
//!
//! let end_of_the_thirteenth_baktun = gregorian::to_fixed(2012, 12, 21)?;
//! let long_count = MayaLongCountCalendar.from_fixed(end_of_the_thirteenth_baktun)?;
//! assert_eq!(long_count.to_string(), "13.0.0.0.0");
//!
//! let heisei = JapaneseCalendar.from_fixed(end_of_the_thirteenth_baktun)?;
//! assert_eq!(heisei.era.kanji, "平成");
//! assert_eq!((heisei.year, heisei.month.ordinal, heisei.day), (24, 12, 21));
//! # Ok::<(), hc_calendar::CalendarError>(())
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod aztec;
pub mod balinese_pawukon;
pub mod japanese;
pub mod javanese_pasaran;
pub mod maya;
pub mod nengo;
pub mod sexagenary;

pub use aztec::{
    AztecTonalpohualliCalendar, AztecTonalpohualliDate, AztecXiuhpohualliCalendar,
    AztecXiuhpohualliDate,
};
pub use balinese_pawukon::{BalinesePawukonCalendar, PawukonDate};
pub use japanese::{JapaneseCalendar, JapaneseDate};
pub use javanese_pasaran::{JavanesePasaranCalendar, WetonDate};
pub use maya::{
    MayaCalendarRoundCalendar, MayaCalendarRoundDate, MayaHaabCalendar, MayaHaabDate,
    MayaLongCountCalendar, MayaLongCountDate, MayaTzolkinCalendar, MayaTzolkinDate,
};
pub use nengo::{Certainty, Court, Nengo, WesternScale};
pub use sexagenary::{SexagenaryCalendar, SexagenaryDayDate};

pub use hc_calendar;
pub use hc_calendars_lunar;
pub use hc_calendars_solar;

#[cfg(feature = "alloc")]
mod registration {
    use alloc::boxed::Box;

    use hc_calendar::{CalendarRegistry, DynAdapter};

    /// Register every calendar in this crate with `registry`.
    ///
    /// Inserting is idempotent: a second call replaces rather than
    /// duplicates, since [`CalendarRegistry::insert`] keys on the
    /// calendar's identifier.
    ///
    /// The four Maya calendars are registered separately because they are
    /// four calendars and not four views of one: a Maya scribe who wrote a
    /// Calendar Round date had said something a long count date does not
    /// say, and vice versa.
    pub fn register_all(registry: &mut CalendarRegistry) {
        registry.insert(Box::new(DynAdapter::new(crate::JapaneseCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::MayaLongCountCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::MayaTzolkinCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::MayaHaabCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::MayaCalendarRoundCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::AztecTonalpohualliCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::AztecXiuhpohualliCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::BalinesePawukonCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::JavanesePasaranCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::SexagenaryCalendar)));
    }
}

#[cfg(feature = "alloc")]
pub use registration::register_all;

/// How many calendars [`register_all`] inserts.
#[cfg(test)]
const CALENDAR_COUNT: usize = 10;

#[cfg(test)]
mod tests {
    use hc_calendar::{Calendar, Rd};
    use hc_calendars_solar::gregorian;

    use super::*;

    /// Round-trip one fixed day through every calendar that supports it,
    /// returning how many did.
    ///
    /// A macro rather than a collection of trait objects so that the test
    /// exercises the *static* interface, where each calendar has its own
    /// date type, rather than only the erased one.
    macro_rules! round_trip_every_calendar {
        ($rd:expr, $($calendar:expr),+ $(,)?) => {{
            let mut checked = 0;
            $(
                {
                    let calendar = $calendar;
                    if calendar.meta().supports($rd) {
                        let date = calendar.from_fixed($rd).unwrap_or_else(|error| {
                            panic!("{} rejected {} in range: {error}", calendar.meta().id, $rd)
                        });
                        assert_eq!(
                            calendar.to_fixed(date),
                            Ok($rd),
                            "{} failed to round-trip {}",
                            calendar.meta().id,
                            $rd
                        );
                        let fields = calendar.to_fields(date).unwrap();
                        assert_eq!(
                            calendar.from_fields(&fields),
                            Ok(date),
                            "{} failed to round-trip fields at {}",
                            calendar.meta().id,
                            $rd
                        );
                        checked += 1;
                    }
                }
            )+
            checked
        }};
    }

    #[test]
    fn every_calendar_round_trips_every_day_of_a_shared_range() {
        // The step is deliberately coprime with 210, 260, 365 and 60 so that
        // the sweep does not land on the same cycle position every time.
        for rd in (-1_100_000..=800_000).step_by(1_009) {
            round_trip_every_calendar!(
                Rd(rd),
                MayaLongCountCalendar,
                MayaTzolkinCalendar,
                MayaHaabCalendar,
                MayaCalendarRoundCalendar,
                AztecTonalpohualliCalendar,
                AztecXiuhpohualliCalendar,
                BalinesePawukonCalendar,
                JavanesePasaranCalendar,
                SexagenaryCalendar,
            );
        }
    }

    #[test]
    fn the_japanese_calendar_round_trips_every_day_it_covers() {
        // Every single day, not a sample: the era boundaries and the 1873
        // switch from lunisolar to Gregorian are exactly where a sampled
        // sweep would step over the bug.
        let meta = JapaneseCalendar.meta();
        let first = meta.earliest.expect("bounded below");
        let last = gregorian::to_fixed(2100, 12, 31).expect("in range");
        for rd in first.0..=last.0 {
            let rd = Rd(rd);
            let date = JapaneseCalendar.from_fixed(rd).expect("in range");
            assert_eq!(JapaneseCalendar.to_fixed(date), Ok(rd), "{rd}");
            let fields = JapaneseCalendar.to_fields(date).expect("describable");
            assert_eq!(JapaneseCalendar.from_fields(&fields), Ok(date), "{rd}");
        }
    }

    #[test]
    fn bounded_calendars_refuse_days_outside_their_range() {
        for meta in [
            JapaneseCalendar.meta(),
            MayaLongCountCalendar.meta(),
            MayaCalendarRoundCalendar.meta(),
        ] {
            let first = meta.earliest.expect("bounded below");
            let last = meta.latest.expect("bounded above");
            assert!(meta.supports(first) && meta.supports(last), "{}", meta.id);
            assert!(!meta.supports(Rd(first.0 - 1)), "{}", meta.id);
            assert!(!meta.supports(Rd(last.0 + 1)), "{}", meta.id);
            assert!(meta.check_range(Rd(first.0 - 1)).is_err());
            assert!(meta.check_range(Rd(last.0 + 1)).is_err());
        }
        assert!(JapaneseCalendar.from_fixed(Rd(0)).is_err());
        assert!(MayaLongCountCalendar.from_fixed(Rd(-2_000_000)).is_err());
    }

    #[test]
    fn the_unbounded_cycles_name_a_day_arbitrarily_far_back() {
        // A cycle has no epoch in the sense a calendar does, so the only
        // honest bound is the arithmetic one.
        let deep = Rd(-5_000_000);
        assert!(MayaTzolkinCalendar.from_fixed(deep).is_ok());
        assert!(BalinesePawukonCalendar.from_fixed(deep).is_ok());
        assert!(SexagenaryCalendar.from_fixed(deep).is_ok());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn the_registry_holds_every_calendar_under_a_distinct_identifier() {
        use alloc::vec::Vec;
        use hc_calendar::{CalendarId, CalendarRegistry};

        let mut registry = CalendarRegistry::new();
        register_all(&mut registry);
        assert_eq!(registry.len(), CALENDAR_COUNT);

        let ids: Vec<CalendarId> = registry.metas().map(|meta| meta.id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "identifiers must be distinct");

        assert!(registry.get_by_name("japanese").is_some());
        assert!(registry.get_by_name("maya-longcount").is_some());
        assert!(registry.get_by_name("nonexistent").is_none());

        register_all(&mut registry);
        assert_eq!(registry.len(), CALENDAR_COUNT);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn one_day_renders_in_every_registered_calendar() {
        use hc_calendar::{CalendarId, CalendarRegistry};

        let mut registry = CalendarRegistry::new();
        register_all(&mut registry);

        let rd = gregorian::to_fixed(2026, 9, 21).expect("in range");
        let rendered = registry.describe_day(rd);
        assert_eq!(rendered.len(), registry.len());

        let japanese = registry
            .get(CalendarId("japanese"))
            .expect("registered")
            .fixed_to_fields(rd)
            .expect("in range");
        assert_eq!(japanese.era, Some("reiwa"));
        assert_eq!(japanese.year, 8);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn every_registered_calendar_reports_usable_metadata() {
        use hc_calendar::CalendarRegistry;

        let mut registry = CalendarRegistry::new();
        register_all(&mut registry);
        for meta in registry.metas() {
            assert!(!meta.id.as_str().is_empty());
            assert!(!meta.english_name.is_empty());
            // Only the Japanese calendar carries intercalary months, and
            // only in the lunisolar half of its range.
            assert!(!meta.has_leap_months || meta.id.as_str() == "japanese");
            if let (Some(first), Some(last)) = (meta.earliest, meta.latest) {
                assert!(first < last, "{} has an empty range", meta.id);
            }
        }
    }
}
