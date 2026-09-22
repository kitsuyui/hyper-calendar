//! Solar calendars defined by an equinox observed at a place, for
//! `hyper-calendar`.
//!
//! Three calendars begin their year on the day an equinox falls, judged by
//! a clock at a named place, and no counting rule reproduces that exactly:
//!
//! * [`persian`] — the Solar Hijri calendar of Iran: Nowruz is the day of
//!   the March equinox if the equinox falls before noon, Iran Standard
//!   Time, and the day after otherwise. CLDR `persian`.
//! * [`bahai`] — the Badíʿ calendar under the rules unified in 172 BE:
//!   Naw-Rúz is the Badíʿ day, sunset to sunset at Tehran, in which the
//!   March equinox falls, and the Twin Holy Birthdays follow the eighth new
//!   moon after it. `bahai-astronomical`.
//! * [`french_republican`] — the calendar France kept from 1793 to 1805:
//!   1 Vendémiaire is the day, midnight to midnight in true solar time at
//!   the Paris Observatory, in which the September equinox falls.
//!   `french-republican-equinox`.
//!
//! Each has an arithmetic sibling in `hc-calendars-solar` —
//! `persian-arithmetic`, `bahai-arithmetic`, `french-republican-arithmetic`
//! — that approximates it by a cycle and says so in its name, and the
//! Badíʿ calendar also has the *as kept* form `bahai`, which carries the
//! Bahá'í World Centre's published table for 172–221 BE. The date types and
//! the calendars' own month names are shared with those siblings, so a date
//! converts between variants without ceremony.
//!
//! # What "astronomical" buys, and what it costs
//!
//! The arithmetic siblings are exact about their own rule and wrong about
//! the world in the years the rule and the sky disagree: Birashk's cycle
//! puts Nowruz 1404 a day early, the arithmetic Badíʿ calendar puts
//! Naw-Rúz 173 a day late. These calendars are right about the world to
//! the extent that `hc-astro` places an equinox, which is within about a
//! quarter of an hour ([`EQUINOX_TOLERANCE_MINUTES`]) — so a year whose
//! equinox falls that close to the deciding noon, sunset or midnight is a
//! year decided here by a model where the country or the community decided
//! by an ephemeris. Each module exposes a `new_year_margin` that says how
//! close the call was, so a caller can tell a confident year from a
//! marginal one.
//!
//! The tests are the published record: the fourteen new years France
//! actually kept, the leap years Iran actually had, and every row of the
//! Bahá'í World Centre's fifty-year table — every row the model can claim,
//! which is all but the two Naw-Rúzes that fell within the tolerance of
//! Tehran's sunset. One of those, 183 BE, fell within a minute of it and
//! the model gets it wrong; the test names it, because a row like that is
//! the table's to decide, not a model's.
//!
//! # Why a crate of its own
//!
//! `hc-calendars-solar` draws its line at counting: nothing there asks
//! where the Sun is, so nothing there pays for the astronomy. These three
//! do ask, and they are solar, so they are neither that crate's nor
//! `hc-calendars-lunar`'s. Selecting them is the `equinox` feature of
//! `hyper-calendar`.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod bahai;
pub mod french_republican;
pub mod persian;
pub mod places;

/// How far, in minutes, the equinox instants behind these calendars can be
/// trusted: `hc-astro` places a seasonal event to within about twelve
/// minutes of the almanacs, and a sunset to within a minute or two of its
/// own geometry. A year whose `new_year_margin` is smaller than this is a
/// year the calendar decides by a model, and the tests in each module do
/// not claim it.
pub const EQUINOX_TOLERANCE_MINUTES: f64 = 15.0;

pub use bahai::AstronomicalBahaiCalendar;
pub use french_republican::EquinoxFrenchRepublicanCalendar;
pub use persian::PersianCalendar;

#[cfg(feature = "alloc")]
mod registration {
    use alloc::boxed::Box;

    use hc_calendar::{CalendarRegistry, DynAdapter};

    /// Register every calendar in this crate with `registry`.
    ///
    /// Inserting is idempotent: a second call replaces rather than
    /// duplicates, since [`CalendarRegistry::insert`] keys on the
    /// calendar's identifier.
    pub fn register_all(registry: &mut CalendarRegistry) {
        registry.insert(Box::new(DynAdapter::new(crate::PersianCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::AstronomicalBahaiCalendar)));
        registry.insert(Box::new(DynAdapter::new(
            crate::EquinoxFrenchRepublicanCalendar,
        )));
    }
}

#[cfg(feature = "alloc")]
pub use registration::register_all;

#[cfg(test)]
mod tests {
    use hc_calendar::Calendar;

    use super::*;

    /// The number of calendars this crate registers.
    const CALENDAR_COUNT: usize = 3;

    #[test]
    fn every_calendar_here_is_astronomical_and_bounded() {
        for meta in [
            PersianCalendar.meta(),
            AstronomicalBahaiCalendar.meta(),
            EquinoxFrenchRepublicanCalendar.meta(),
        ] {
            assert!(meta.is_astronomical, "{}", meta.id);
            let first = meta.earliest.expect("bounded below");
            let last = meta.latest.expect("bounded above");
            assert!(first < last, "{} has an empty range", meta.id);
            assert!(meta.supports(first) && meta.supports(last), "{}", meta.id);
            assert!(!meta.supports(hc_calendar::Rd(first.0 - 1)), "{}", meta.id);
            assert!(!meta.supports(hc_calendar::Rd(last.0 + 1)), "{}", meta.id);
        }
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
        assert!(registry.get_by_name("persian").is_some());
        assert!(registry.get_by_name("bahai-astronomical").is_some());
        assert!(registry.get_by_name("french-republican-equinox").is_some());
        register_all(&mut registry);
        assert_eq!(registry.len(), CALENDAR_COUNT);
    }
}
