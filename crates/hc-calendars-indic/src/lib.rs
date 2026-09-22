//! The calendars of the Indian subcontinent for `hyper-calendar`.
//!
//! * [`hindu_lunar`] — the Hindu lunisolar calendar in its *amānta*
//!   (new-moon-ending) form, computed from the true Sun and Moon with the
//!   sidereal zodiac of `hc-seasons`, the way the Government of India's
//!   *Rashtriya Panchang* computes it. Months run new moon to new moon and
//!   are named for the saṅkrānti they contain, a month without one is
//!   intercalary, and the day is the tithi at sunrise. `hindu-lunar`.
//! * [`tithi`] — the lunar day itself: which tithi is in progress at a
//!   moment, and which a civil day carries.
//! * [`places`] — the sunrise that reads the day: the Central Station of
//!   the national calendar, Ujjain of the classical almanacs, New Delhi.
//!
//! # What is here and what is not yet
//!
//! The amānta lunisolar calendar is the one festival dates are stated in
//! across most of India and the one the national almanac carries. The
//! *pūrṇimānta* form of the north — the same fortnights, the dark one
//! counted first — and the solar calendars of the south and east (Tamil,
//! Malayalam, Bengali, Odia) that `hc-seasons` already names the months of
//! are the next two calendars, and `docs/calendars.md` says so.
//!
//! # Why a crate of its own
//!
//! `hc-calendars-lunar` holds the East Asian lunisolar engine and the
//! Hijri and Hebrew calendars; the Indian reckonings need the sidereal
//! zodiac, which lives in `hc-seasons`, and `hc-seasons` can route through
//! `hc-calendars-lunar` — so the Indian calendars sit downstream of both.
//! Selecting them is the `indic` feature of `hyper-calendar`.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod hindu_lunar;
pub mod places;
pub mod tithi;

pub use hindu_lunar::{HinduLunarCalendar, HinduLunarDate};
pub use tithi::{Paksha, Prevalence};

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
        registry.insert(Box::new(DynAdapter::new(
            crate::HinduLunarCalendar::RASHTRIYA,
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
    const CALENDAR_COUNT: usize = 1;

    #[test]
    fn every_calendar_here_is_astronomical_and_bounded() {
        let meta = HinduLunarCalendar::RASHTRIYA.meta();
        assert!(meta.is_astronomical, "{}", meta.id);
        assert!(meta.has_leap_months, "{}", meta.id);
        let first = meta.earliest.expect("bounded below");
        let last = meta.latest.expect("bounded above");
        assert!(first < last, "{} has an empty range", meta.id);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn the_registry_holds_every_calendar_under_a_distinct_identifier() {
        use hc_calendar::CalendarRegistry;

        let mut registry = CalendarRegistry::new();
        register_all(&mut registry);
        assert_eq!(registry.len(), CALENDAR_COUNT);
        assert!(registry.get_by_name("hindu-lunar").is_some());
        register_all(&mut registry);
        assert_eq!(registry.len(), CALENDAR_COUNT);
    }
}
