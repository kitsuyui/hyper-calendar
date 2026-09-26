//! Solar calendars defined by an equinox observed at a place, for
//! `hyper-calendar`.
//!
//! Three calendars begin their year on the day an equinox falls, judged by
//! a clock at a named place, and no counting rule reproduces that exactly;
//! the first of them is carried three times, as Iran and as Afghanistan
//! name its months and under the second reading of its noon:
//!
//! * [`persian`] — the Solar Hijri calendar of Iran: Nowruz is the day of
//!   the March equinox if the equinox falls before noon, Iran Standard
//!   Time, and the day after otherwise. CLDR `persian`. The same days
//!   under the Dari month names are [`persian_afghan`], `persian-afghan`,
//!   and the same rule with the Sun's own noon at Tehran in place of the
//!   clock's is [`persian_apparent_noon`], `persian-apparent-noon`. The
//!   Solar Hijri calendars are written up in
//!   `docs/systems/solar-hijri.md` in the repository.
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
//! `persian-arithmetic` and `persian-arithmetic-33`, `bahai-arithmetic`,
//! `french-republican-arithmetic` — that approximates it by a cycle and
//! says so in its name, and the
//! Badíʿ calendar also has the *as kept* form `bahai`, which carries the
//! Bahá'í World Centre's published table for 172–221 BE. The date types and
//! the calendars' own month names are shared with those siblings, so a date
//! converts between variants without ceremony.
//!
//! The Badíʿ and French Republican calendars, with their siblings, are
//! written up in `docs/systems/equinox-calendars.md` in the repository:
//! the rules and their sources, a worked new year of each, the years too
//! close to call, and why each is a separately named calendar in this
//! crate rather than a switch on its sibling.
//!
//! # What "astronomical" buys, and what it costs
//!
//! The arithmetic siblings are exact about their own rule and wrong about
//! the world in the years the rule and the sky disagree: Birashk's cycle
//! puts Nowruz 1404 a day early, the arithmetic Badíʿ calendar puts
//! Naw-Rúz 173 a day late. These calendars are right about the world to
//! the extent that `hc-astro` places an equinox — within seconds, since it
//! carries VSOP87 — and the deciding clock can be placed: a noon in
//! standard time exactly, an apparent midnight to seconds, a sunset to a
//! minute or two and to whatever horizon the almanac assumed. Each module
//! states its `TOLERANCE_MINUTES` from those parts and exposes a
//! `new_year_margin`, how far the equinox fell from the deciding instant,
//! so a caller can tell a confident year from a marginal one.
//!
//! The tests are the published record: the fourteen new years France
//! actually kept, the leap years Iran actually had, and every row of the
//! Bahá'í World Centre's fifty-year table — every row the model can claim,
//! which is all but the two Naw-Rúzes within a sunset's tolerance, one of
//! them, 183 BE, an equinox within seconds of Tehran's sunset. The model
//! agrees with the table on both, and the test names them rather than
//! claims them, because a row like that is the table's to decide, not a
//! model's.
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
pub mod persian_afghan;
pub mod persian_apparent_noon;
pub mod places;

pub use bahai::AstronomicalBahaiCalendar;
pub use french_republican::EquinoxFrenchRepublicanCalendar;
pub use persian::PersianCalendar;
pub use persian_afghan::AfghanPersianCalendar;
pub use persian_apparent_noon::ApparentNoonPersianCalendar;

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
        registry.insert(Box::new(DynAdapter::new(crate::AfghanPersianCalendar)));
        registry.insert(Box::new(DynAdapter::new(
            crate::ApparentNoonPersianCalendar,
        )));
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
    const CALENDAR_COUNT: usize = 5;

    #[test]
    fn every_calendar_here_is_astronomical_and_bounded() {
        for meta in [
            PersianCalendar.meta(),
            AfghanPersianCalendar.meta(),
            ApparentNoonPersianCalendar.meta(),
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
        assert!(registry.get_by_name("persian-afghan").is_some());
        assert!(registry.get_by_name("persian-apparent-noon").is_some());
        assert!(registry.get_by_name("bahai-astronomical").is_some());
        assert!(registry.get_by_name("french-republican-equinox").is_some());
        register_all(&mut registry);
        assert_eq!(registry.len(), CALENDAR_COUNT);
    }

    /// Every registered calendar answers whether a year is leap, or says
    /// that its dates have no year; nothing falls through to an inference.
    #[cfg(feature = "alloc")]
    #[test]
    fn every_registered_calendar_says_whether_a_year_is_leap() {
        use hc_calendar::{CalendarError, CalendarRegistry, Rd, shape::MONTH};

        let mut registry = CalendarRegistry::new();
        register_all(&mut registry);
        for meta in registry.metas() {
            let calendar = registry.get(meta.id).unwrap();
            let inside = match (meta.earliest, meta.latest) {
                (Some(first), Some(last)) => Rd((first.0 + last.0) / 2),
                _ => Rd(738_000),
            };
            let year = calendar.fixed_to_fields(inside).unwrap().year;
            match calendar.is_leap_year(year) {
                Ok(_) => {}
                Err(CalendarError::UnsupportedField("year")) => assert!(
                    !calendar.cycles().iter().any(|cycle| cycle.kind == MONTH),
                    "{} has months and so a year",
                    meta.id
                ),
                Err(error) => panic!("{} could not answer for {year}: {error}", meta.id),
            }
        }
    }

    /// The dynamic answer is the module's own rule, read at the equinox.
    #[test]
    fn the_dynamic_leap_year_is_the_module_rule() {
        use hc_calendar::{CalendarError, DynAdapter, DynCalendar};
        use hc_calendars_solar::bahai::AYYAM_I_HA;

        for year in (1..=250).step_by(3) {
            assert_eq!(
                DynAdapter::new(PersianCalendar).is_leap_year(1_300 + year),
                persian::is_leap_year(1_300 + year).ok_or(CalendarError::YearOutOfRange)
            );
            assert_eq!(
                DynAdapter::new(ApparentNoonPersianCalendar).is_leap_year(1_300 + year),
                persian_apparent_noon::is_leap_year(1_300 + year)
                    .ok_or(CalendarError::YearOutOfRange)
            );
            assert_eq!(
                DynAdapter::new(EquinoxFrenchRepublicanCalendar).is_leap_year(year),
                french_republican::is_leap_year(year).ok_or(CalendarError::YearOutOfRange)
            );
            assert_eq!(
                DynAdapter::new(AstronomicalBahaiCalendar).is_leap_year(year),
                bahai::days_in_month(year, AYYAM_I_HA)
                    .map(|days| days == 5)
                    .ok_or(CalendarError::YearOutOfRange)
            );
        }
    }
}
