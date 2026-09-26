//! The calendars of the Indian subcontinent for `hyper-calendar`.
//!
//! * [`hindu_lunar`] — the Hindu lunisolar calendar in its *amānta*
//!   (new-moon-ending) form, computed from the true Sun and Moon with the
//!   sidereal zodiac of `hc-seasons`, the way the Government of India's
//!   *Rashtriya Panchang* computes it. Months run new moon to new moon and
//!   are named for the saṅkrānti they contain, a month without one is
//!   intercalary, and the day is the tithi at sunrise; the year is named in
//!   the southern sixty-year cycle, as at Ugādi. `hindu-lunar`.
//! * [`hindu_purnimanta`] — the same tithis under the north's month names:
//!   a month ends at the full moon, so the dark fortnight comes first and
//!   takes the following bright fortnight's name; the year is named in the
//!   northern, Bārhaspatya, cycle. `hindu-lunar-purnimanta`.
//! * [`hindu_solar`] — the solar reckonings of Tamil Nadu, Kerala, Bengal
//!   and the Vikrami regions (Punjab and Haryana, whose months Odisha
//!   shares under its own years): a month is the Sun's stay in a sidereal
//!   sign, and each region has its own rule for the day the month begins.
//!   `hindu-solar-tamil`, `hindu-solar-malayalam`, `hindu-solar-bengali`,
//!   `hindu-solar-vikrami`; and the Bengali months under the Magi San of
//!   Chittagong, `magi-san`.
//! * [`tithi`] — the lunar day itself: which tithi is in progress at a
//!   moment, and which a civil day carries.
//! * [`nakshatra`] — the Moon's station among the twenty-seven: which is
//!   in progress at a moment, and when the Moon enters and leaves one; and
//!   the Sun's.
//! * [`panchanga`] — the two other limbs of the almanac: the yoga, from
//!   the sum of the Sun's and Moon's sidereal longitudes, and the karaṇa,
//!   the half-tithi.
//! * [`hindu_old`] — the mean-motion solar and lunisolar calendars of the
//!   *Ārya Siddhānta*, counted in the Kali Yuga: the arithmetic the true
//!   calendars replaced. `hindu-old-solar`, `hindu-old-lunar`.
//! * [`nepal_sambat`] — the lunisolar calendar of the Newar people: the
//!   amānta months under their Newar names, the year opening at Kachhalā,
//!   the day read at Kathmandu's sunrise. `nepal-sambat`.
//! * [`vira_nirvana`] — the Jain era of Mahāvīra's nirvāṇa over the same
//!   amānta months, the year opening at Kārtika śukla 1, the day after
//!   Dīpāvalī, 605 years before the Śaka year. `vira-nirvana-samvat`.
//! * [`odia_anka`] — the regnal years of the Gajapati of Puri, the
//!   *aṅka*, which turn at Suniā, Bhādrapada śukla 12, over the pūrṇimānta
//!   months, and never take a number ending in 6, or in 0 but 10, or 1.
//!   `odia-anka`.
//! * [`lunar_era`] — the eras of Sewell and Dikshit's Art. 71 over the
//!   lunisolar months: the Gujarati Vikrama year from Kārttika,
//!   Śivājī's Rājyābhiṣeka Śaka and the Saptarṣi era of Kashmir, and the
//!   year arithmetic of the Gupta, Valabhī and Kalachuri eras.
//!   `vikram-samvat-kartikadi`, `rajyabhisheka-saka`, `saptarshi`.
//! * [`fasli`] — the Faṣlī revenue year of Madras from 1 July and of Bombay
//!   from the Sun's entry into Mṛgaśira, and the Maratha Sūr-san, over the
//!   Gregorian days. `fasli-madras`, `fasli-bombay`, `sur-san`.
//! * [`year_start`] — where an era's year opens among the amānta months,
//!   the arithmetic the lunisolar eras share.
//! * [`samvatsara`] — the southern sixty-year cycle of year names,
//!   Prabhava to Kṣaya, which the Tamil solar year and the amānta
//!   lunisolar year carry.
//! * [`barhaspatya`] — the northern sixty-year cycle, the same names
//!   reckoned by Jupiter's mean motion, by Sewell and Dikshit's rules, with
//!   the names they expunge.
//! * [`bikram_sambat`] — the solar calendar of Nepal: the months the
//!   Government of Nepal gazettes for 2080–2083, and elsewhere the
//!   *Sūrya Siddhānta*'s saṅkrāntis on their civil day. `bikram-sambat`.
//! * [`surya_siddhanta`] — the Sun of the *Sūrya Siddhānta*, whose
//!   saṅkrāntis the traditional almanacs keep.
//! * [`places`] — the sunrise that reads the day: the Central Station of
//!   the national calendar, Ujjain of the classical almanacs, and
//!   Kathmandu.
//!
//! # What is here and what is not yet
//!
//! The amānta lunisolar calendar is the one festival dates are stated in
//! across most of India and the one the national almanac carries; the
//! pūrṇimānta form is the north's naming of the same days; the four solar
//! reckonings are the civil calendars of the south, the east and the
//! north-west; the Bikram Sambat is Nepal's. What is still to come — the
//! Odia Amli and Vilayati years with their solar months, the Bikram
//! Sambat's gazetted months outside 2080–2083 — `docs/calendars.md`
//! lists.
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

/// How far apart the days of a day-by-day sweep in this crate's tests are:
/// every day in a release build, every `sampled`th in a debug build.
///
/// The full sweeps take minutes under the coverage job's instrumentation,
/// so a debug or coverage run takes a fixed, deterministic sample of them
/// and CI's release-mode test job walks every day; the published anchors
/// are checked in full either way (docs/policy.md §7).
#[cfg(test)]
pub(crate) const fn sweep_stride(sampled: usize) -> usize {
    if cfg!(debug_assertions) { sampled } else { 1 }
}

pub mod barhaspatya;
pub mod bikram_sambat;
pub mod fasli;
pub mod hindu_lunar;
pub mod hindu_old;
pub mod hindu_purnimanta;
pub mod hindu_solar;
mod kartikadi;
pub mod lunar_era;
pub mod nakshatra;
pub mod nepal_sambat;
pub mod odia_anka;
pub mod panchanga;
pub mod places;
pub mod samvatsara;
pub mod surya_siddhanta;
pub mod tithi;
pub mod vira_nirvana;
pub mod year_start;

pub use bikram_sambat::{BikramSambatCalendar, BikramSambatDate};
pub use hindu_lunar::{HinduLunarCalendar, HinduLunarDate};
pub use hindu_old::{
    OldHinduLunarCalendar, OldHinduLunarDate, OldHinduSolarCalendar, OldHinduSolarDate,
};
pub use hindu_purnimanta::HinduPurnimantaCalendar;
pub use hindu_solar::{HinduSolarCalendar, HinduSolarDate, SankrantiRule, SolarModel};
pub use nepal_sambat::{NepalSambatCalendar, NepalSambatDate};
pub use odia_anka::{OdiaAnkaCalendar, OdiaAnkaDate};
pub use tithi::{Paksha, Prevalence};
pub use vira_nirvana::{ViraNirvanaCalendar, ViraNirvanaDate};

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
        registry.insert(Box::new(DynAdapter::new(
            crate::NepalSambatCalendar::KATHMANDU,
        )));
        registry.insert(Box::new(DynAdapter::new(crate::BikramSambatCalendar)));
        registry.insert(Box::new(DynAdapter::new(
            crate::ViraNirvanaCalendar::RASHTRIYA,
        )));
        registry.insert(Box::new(DynAdapter::new(crate::OdiaAnkaCalendar::PURI)));
        for calendar in crate::lunar_era::ALL {
            registry.insert(Box::new(DynAdapter::new(*calendar)));
        }
        for calendar in crate::fasli::ALL {
            registry.insert(Box::new(DynAdapter::new(*calendar)));
        }
    }
}

#[cfg(feature = "alloc")]
pub use registration::register_all;

#[cfg(test)]
mod tests {
    use hc_calendar::Calendar;

    use super::*;

    /// The number of calendars this crate registers.
    const CALENDAR_COUNT: usize = 19;

    /// Every calendar the crate registers, so that neither list can drift
    /// from the registry unnoticed.
    #[cfg(feature = "alloc")]
    fn all_metas() -> alloc::vec::Vec<hc_calendar::CalendarMeta> {
        let mut metas = alloc::vec![
            HinduLunarCalendar::RASHTRIYA.meta(),
            HinduPurnimantaCalendar::RASHTRIYA.meta(),
            OldHinduSolarCalendar.meta(),
            OldHinduLunarCalendar.meta(),
            NepalSambatCalendar::KATHMANDU.meta(),
            BikramSambatCalendar.meta(),
            ViraNirvanaCalendar::RASHTRIYA.meta(),
            OdiaAnkaCalendar::PURI.meta(),
        ];
        metas.extend(crate::hindu_solar::ALL.iter().map(Calendar::meta));
        metas.extend(crate::lunar_era::ALL.iter().map(Calendar::meta));
        metas.extend(crate::fasli::ALL.iter().map(Calendar::meta));
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
    /// are arithmetic, which is the whole difference between them, and so
    /// is the Madras Faṣlī year, fixed to 1 July.
    #[cfg(feature = "alloc")]
    #[test]
    fn only_the_mean_calendars_are_arithmetic() {
        for meta in all_metas() {
            let mean = meta.id.0.starts_with("hindu-old") || meta.id.0 == "fasli-madras";
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
        assert!(registry.get_by_name("nepal-sambat").is_some());
        assert!(registry.get_by_name("bikram-sambat").is_some());
        assert!(registry.get_by_name("vira-nirvana-samvat").is_some());
        assert!(registry.get_by_name("odia-anka").is_some());
        for id in [
            "vikram-samvat-kartikadi",
            "rajyabhisheka-saka",
            "saptarshi",
            "magi-san",
            "fasli-madras",
            "fasli-bombay",
            "sur-san",
        ] {
            assert!(registry.get_by_name(id).is_some(), "{id}");
        }
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

    /// The dynamic answer is the module's own rule.
    #[test]
    fn the_dynamic_leap_year_is_the_module_rule() {
        use hc_calendar::{DynAdapter, DynCalendar};

        for year in (1..=400).step_by(3) {
            assert_eq!(
                DynAdapter::new(OldHinduLunarCalendar).is_leap_year(year),
                Ok(OldHinduLunarCalendar.is_leap_year(year))
            );
            assert_eq!(
                DynAdapter::new(OldHinduSolarCalendar).is_leap_year(year),
                Ok(false)
            );
        }
        let lunar = HinduLunarCalendar::RASHTRIYA;
        for year in [1_940, 1_942, 1_945, 1_946] {
            assert_eq!(
                DynAdapter::new(lunar).is_leap_year(year),
                lunar.leap_month_of(year).map(|leap| leap.is_some())
            );
        }
    }
}
