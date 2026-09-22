//! The calendars of the Indian subcontinent for `hyper-calendar`.
//!
//! * [`hindu_lunar`] — the Hindu lunisolar calendar in its *amānta*
//!   (new-moon-ending) form, computed from the true Sun and Moon with the
//!   sidereal zodiac of `hc-seasons`, the way the Government of India's
//!   *Rashtriya Panchang* computes it. Months run new moon to new moon and
//!   are named for the saṅkrānti they contain, a month without one is
//!   intercalary, and the day is the tithi at sunrise. `hindu-lunar`.
//! * [`hindu_purnimanta`] — the same tithis under the north's month names:
//!   a month ends at the full moon, so the dark fortnight comes first and
//!   takes the following bright fortnight's name. `hindu-lunar-purnimanta`.
//! * [`hindu_solar`] — the solar reckonings of Tamil Nadu, Kerala, Bengal
//!   and the Vikrami regions (Punjab, Odisha, Nepal): a month is the Sun's
//!   stay in a sidereal sign, and each region has its own rule for the day
//!   the month begins. `hindu-solar-tamil`, `hindu-solar-malayalam`,
//!   `hindu-solar-bengali`, `hindu-solar-vikrami`.
//! * [`tithi`] — the lunar day itself: which tithi is in progress at a
//!   moment, and which a civil day carries.
//! * [`nakshatra`] — the Moon's station among the twenty-seven: which is
//!   in progress at a moment, and when the Moon enters and leaves one.
//! * [`hindu_old`] — the mean-motion solar and lunisolar calendars of the
//!   *Ārya Siddhānta*, counted in the Kali Yuga: the arithmetic the true
//!   calendars replaced. `hindu-old-solar`, `hindu-old-lunar`.
//! * [`places`] — the sunrise that reads the day: the Central Station of
//!   the national calendar, Ujjain of the classical almanacs, New Delhi.
//!
//! # What is here and what is not yet
//!
//! The amānta lunisolar calendar is the one festival dates are stated in
//! across most of India and the one the national almanac carries; the
//! pūrṇimānta form is the north's naming of the same days; the four solar
//! reckonings are the civil calendars of the south, the east and the
//! north-west. What is still to come — the Nepali Bikram Sambat as its
//! committee publishes it, the Odia year counts, the Tamil sixty-year
//! names — `docs/calendars.md` lists.
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
pub mod hindu_old;
pub mod hindu_purnimanta;
pub mod hindu_solar;
pub mod nakshatra;
pub mod places;
pub mod tithi;

pub use hindu_lunar::{HinduLunarCalendar, HinduLunarDate};
pub use hindu_old::{
    OldHinduLunarCalendar, OldHinduLunarDate, OldHinduSolarCalendar, OldHinduSolarDate,
};
pub use hindu_purnimanta::HinduPurnimantaCalendar;
pub use hindu_solar::{HinduSolarCalendar, HinduSolarDate, SankrantiRule};
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
        registry.insert(Box::new(DynAdapter::new(
            crate::HinduPurnimantaCalendar::RASHTRIYA,
        )));
        for calendar in crate::hindu_solar::ALL {
            registry.insert(Box::new(DynAdapter::new(*calendar)));
        }
        registry.insert(Box::new(DynAdapter::new(crate::OldHinduSolarCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::OldHinduLunarCalendar)));
    }
}

#[cfg(feature = "alloc")]
pub use registration::register_all;

#[cfg(test)]
mod tests {
    use hc_calendar::Calendar;

    use super::*;

    /// The number of calendars this crate registers.
    const CALENDAR_COUNT: usize = 8;

    /// Every calendar the crate registers, so that neither list can drift
    /// from the registry unnoticed.
    #[cfg(feature = "alloc")]
    fn all_metas() -> alloc::vec::Vec<hc_calendar::CalendarMeta> {
        let mut metas = alloc::vec![
            HinduLunarCalendar::RASHTRIYA.meta(),
            HinduPurnimantaCalendar::RASHTRIYA.meta(),
            OldHinduSolarCalendar.meta(),
            OldHinduLunarCalendar.meta(),
        ];
        metas.extend(crate::hindu_solar::ALL.iter().map(Calendar::meta));
        metas
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn every_registered_calendar_is_bounded() {
        let metas = all_metas();
        assert_eq!(metas.len(), CALENDAR_COUNT);
        for meta in metas {
            let first = meta.earliest.expect("bounded below");
            let last = meta.latest.expect("bounded above");
            assert!(first < last, "{} has an empty range", meta.id);
        }
    }

    /// The true calendars read the Sun and Moon; the two Old Hindu ones
    /// are arithmetic, which is the whole difference between them.
    #[cfg(feature = "alloc")]
    #[test]
    fn only_the_mean_calendars_are_arithmetic() {
        for meta in all_metas() {
            let mean = meta.id.0.starts_with("hindu-old");
            assert_eq!(meta.is_astronomical, !mean, "{}", meta.id);
        }
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn the_registry_holds_every_calendar_under_a_distinct_identifier() {
        use hc_calendar::CalendarRegistry;

        let mut registry = CalendarRegistry::new();
        register_all(&mut registry);
        assert_eq!(registry.len(), CALENDAR_COUNT);
        assert!(registry.get_by_name("hindu-lunar").is_some());
        assert!(registry.get_by_name("hindu-lunar-purnimanta").is_some());
        assert!(registry.get_by_name("hindu-solar-tamil").is_some());
        assert!(registry.get_by_name("hindu-solar-malayalam").is_some());
        assert!(registry.get_by_name("hindu-solar-bengali").is_some());
        assert!(registry.get_by_name("hindu-solar-vikrami").is_some());
        assert!(registry.get_by_name("hindu-old-solar").is_some());
        assert!(registry.get_by_name("hindu-old-lunar").is_some());
        register_all(&mut registry);
        assert_eq!(registry.len(), CALENDAR_COUNT);
    }
}
