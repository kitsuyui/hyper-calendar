//! Lunar and lunisolar calendars for `hyper-calendar`.
//!
//! A lunar calendar counts months and lets the seasons drift; a lunisolar one
//! counts months and then patches a thirteenth in so that the seasons do not.
//! Both kinds are here, and the crate is organised around the only question
//! that really separates one of them from another: **where does the rule come
//! from?**
//!
//! | | Calendar | Rule comes from |
//! |---|---|---|
//! | Arithmetic | [`islamic_civil`], [`islamic_astronomical`], [`hebrew`], [`tibetan`], [`meyer_palmen`], [`yerm`] | a counting rule, exact by definition |
//! | Tabulated | [`islamic_umalqura`] | a published table, exact where the table reaches |
//! | Computed | [`chinese`], [`dangi`], [`vietnamese`], [`japanese_tenpo`], [`islamic_observational`] | an astronomical model, exact only to the model |
//! | Historical | [`japanese_historical`] | the system's *own* period constants, exact to the bureau that published it |
//!
//! The four rows behave differently and the crate does not pretend
//! otherwise. Arithmetic calendars answer for any year you like. The
//! tabulated one refuses every day outside 1300–1600 AH rather than
//! extrapolating. The computed ones carry bounded ranges, say what their
//! model is worth, and — in the observational Hijri case — say plainly that
//! they are predicting a human decision. The historical ones run on
//! ninth-, seventeenth- and eighteenth-century constants and reproduce those
//! systems' errors on purpose, because the errors are what the surviving
//! documents record.
//!
//! # Two engines, eighteen calendars
//!
//! Almost nothing here is written twice.
//!
//! * [`tabular`] is the whole arithmetic Hijri calendar, with the
//!   intercalation scheme and the epoch as parameters. Four schemes times two
//!   epochs is eight calendars; [`islamic_civil`] and
//!   [`islamic_astronomical`] are the two CLDR names for them.
//! * [`lunisolar`] is the whole East Asian machinery — conjunction-to-
//!   conjunction months, the winter-solstice anchor, the no-zhōngqì leap
//!   rule — with the meridian, the epoch, the year numbering, the solar-term
//!   convention and, optionally, a whole set of historical period constants
//!   as parameters. [`chinese`], [`dangi`], [`vietnamese`] and
//!   [`japanese_tenpo`] are four parameter sets and no algorithm at all;
//!   [`japanese_historical`] is four more, carrying the 歳実 and 朔実 of
//!   Senmyō-reki, Jōkyō-reki, Hōryaku-reki and Kansei-reki so that those
//!   calendars drift away from the sky exactly as they historically did.
//!
//! [`hebrew`] and [`tibetan`] stand alone because their rules genuinely are
//! their own, and so do the two proposals, [`meyer_palmen`], a lunisolar
//! calendar of two remainders, and [`yerm`], a lunar one of 52-yerm
//! cycles; [`islamic_umalqura`] stands alone because a table is not an
//! algorithm, and [`islamic_observational`] because it predicts a sighting.
//!
//! # What this crate will not tell you
//!
//! It will not tell you what any authority announced. The Hijri months of
//! religious practice are proclaimed on sighting; the Chinese, Korean,
//! Vietnamese and Japanese calendars were promulgated by bureaux with their
//! own tables and their own solar theories. Every date here is what the
//! stated rule gives, computed now. Where a published table exists, the
//! crate compares itself against it and reports the disagreement rate in a
//! test rather than quietly matching on the cases that happen to agree.
//!
//! The four calendars in [`japanese_historical`] are the exception that
//! proves the rule: they *do* aim at what the bureau published, they are
//! measured against 982 years of 内田正男『日本暦日原典』-derived table, and
//! they agree with it on 96.4% to 99.1% of days. The README states every one
//! of those rates, including the ones that are not 100%.
//!
//! # Example
//!
//! ```
//! use hc_calendar::{Calendar, Month};
//! use hc_calendars_lunar::{ChineseCalendar, LunisolarDate};
//!
//! // Chinese New Year 2024 began the year of the Wood Dragon.
//! let new_year = ChineseCalendar
//!     .to_fixed(LunisolarDate::new(4_661, Month::regular(1), 1))
//!     .expect("in range");
//! assert_eq!(new_year.to_julian_day_number(), 2_460_351);
//!
//! let cycle = hc_calendars_lunar::chinese::PARAMETERS.sexagenary_year(4_661);
//! assert_eq!((cycle.stem_name(), cycle.zodiac_animal()), ("jia", "dragon"));
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod civil;

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

pub mod babylonian;
pub mod chinese;
pub mod dangi;
pub mod hebrew;
pub mod islamic_astronomical;
pub mod islamic_civil;
pub mod islamic_observational;
pub mod islamic_umalqura;
pub mod japanese_historical;
pub mod japanese_tenpo;
pub mod lunisolar;
pub mod meyer_palmen;
pub mod tabular;
pub mod tibetan;
pub mod vietnamese;
pub mod yerm;

pub use babylonian::{BabylonianCalendar, BabylonianDate};
pub use chinese::{ChineseCalendar, ChineseDate};
pub use dangi::{DangiCalendar, DangiDate};
pub use hebrew::{HebrewCalendar, HebrewDate};
pub use islamic_astronomical::IslamicAstronomicalCalendar;
pub use islamic_civil::IslamicCivilCalendar;
pub use islamic_observational::{
    IslamicObservationalCalendar, ObservationSite, VisibilityCriterion,
};
pub use islamic_umalqura::IslamicUmmAlQuraCalendar;
pub use japanese_historical::horyaku::HoryakuCalendar;
pub use japanese_historical::jokyo::JokyoCalendar;
pub use japanese_historical::kansei::KanseiCalendar;
pub use japanese_historical::senmyo::SenmyoCalendar;
pub use japanese_tenpo::{JapaneseTenpoCalendar, JapaneseTenpoDate};
pub use lunisolar::{
    ConjunctionMode, LunisolarCalendar, LunisolarDate, LunisolarParameters, MeanMotionModel,
    MeridianEra, SolarTermMode,
};
pub use meyer_palmen::{MeyerPalmenCalendar, MeyerPalmenDate};
pub use tabular::{IslamicDate, LeapYearRule, TabularIslamicCalendar};
pub use tibetan::{TibetanCalendar, TibetanDate};
pub use vietnamese::{VietnameseCalendar, VietnameseDate};
pub use yerm::{YermCalendar, YermDate};

pub use hc_astro;
pub use hc_calendar;

/// Registration of every calendar in this crate, for the dynamic registry.
#[cfg(feature = "alloc")]
mod registration {
    extern crate alloc;

    use alloc::boxed::Box;

    use hc_calendar::{CalendarRegistry, DynAdapter};

    /// Insert every calendar in this crate into a registry.
    ///
    /// The observational Hijri calendar is registered at its Mecca default.
    /// It is a *prediction of a human decision* rather than a computation,
    /// so a caller who cares about a particular country's announcements
    /// should build an [`crate::IslamicObservationalCalendar`] with that
    /// site and insert it themselves, replacing this entry.
    ///
    /// Tabular Hijri variants beyond the two canonical epochs get names of
    /// their own rather than being withheld. A competing convention that
    /// would otherwise share the `islamic-civil` or `islamic-tbla`
    /// identifier is what policy §5 says to solve by minting a name, and
    /// `islamic-fatimid` is that name for the Ṭayyibī Bohra *Misri*
    /// calendar, which the community uses for every religious date and
    /// which is defined by an authority that publishes it.
    ///
    /// The Kūshyār ibn Labbān and Ḥabash al-Ḥāsib schemes stay
    /// constructible rather than registered. They are medieval *zīj*
    /// variants with no community keeping them and no authority publishing
    /// them today, so under policy §10 there is nobody who could say the
    /// registry was wrong about them. Build one from
    /// [`crate::TabularIslamicCalendar`] when a specific scheme is wanted.
    ///
    /// Inserting is idempotent: a second call replaces rather than
    /// duplicates, since [`CalendarRegistry::insert`] keys on the calendar's
    /// identifier.
    pub fn register_all(registry: &mut CalendarRegistry) {
        registry.insert(Box::new(DynAdapter::new(crate::ChineseCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::DangiCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::VietnameseCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::JapaneseTenpoCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::KanseiCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::HoryakuCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::JokyoCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::SenmyoCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::tabular::FATIMID)));
        registry.insert(Box::new(DynAdapter::new(crate::HebrewCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::BabylonianCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::TibetanCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::MeyerPalmenCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::YermCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::IslamicCivilCalendar)));
        registry.insert(Box::new(DynAdapter::new(
            crate::IslamicAstronomicalCalendar,
        )));
        registry.insert(Box::new(DynAdapter::new(crate::IslamicUmmAlQuraCalendar)));
        registry.insert(Box::new(DynAdapter::new(
            crate::IslamicObservationalCalendar::MECCA,
        )));
    }
}

#[cfg(feature = "alloc")]
pub use registration::register_all;

#[cfg(all(test, feature = "alloc"))]
mod registration_tests {
    use hc_calendar::CalendarRegistry;

    #[test]
    fn every_calendar_registers_under_a_distinct_identifier() {
        let mut registry = CalendarRegistry::new();
        super::register_all(&mut registry);
        assert_eq!(registry.len(), 18);
    }

    #[test]
    fn registering_twice_replaces_rather_than_duplicates() {
        let mut registry = CalendarRegistry::new();
        super::register_all(&mut registry);
        let first = registry.len();
        super::register_all(&mut registry);
        assert_eq!(registry.len(), first);
    }

    /// Every registered calendar answers whether a year is leap, or says
    /// that its dates have no year; nothing falls through to an inference.
    #[test]
    fn every_registered_calendar_says_whether_a_year_is_leap() {
        use hc_calendar::{CalendarError, CalendarRegistry, Rd, shape::MONTH};

        let mut registry = CalendarRegistry::new();
        super::register_all(&mut registry);
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

    /// The dynamic answer is the module's own rule, not a reading of year
    /// lengths — which for a lunar or lunisolar calendar would be wrong.
    #[test]
    fn the_dynamic_leap_year_is_the_module_rule() {
        use hc_calendar::{DynAdapter, DynCalendar};

        use crate::{
            BabylonianCalendar, ChineseCalendar, HebrewCalendar, IslamicCivilCalendar,
            IslamicUmmAlQuraCalendar, TibetanCalendar, babylonian, chinese, hebrew,
            islamic_umalqura, tabular, tibetan,
        };

        for year in (1..=400).step_by(3) {
            assert_eq!(
                DynAdapter::new(HebrewCalendar).is_leap_year(5_700 + year),
                Ok(hebrew::is_leap_year(5_700 + year))
            );
            assert_eq!(
                DynAdapter::new(TibetanCalendar).is_leap_year(1_900 + year),
                Ok(tibetan::is_leap_year(1_900 + year))
            );
            assert_eq!(
                DynAdapter::new(BabylonianCalendar).is_leap_year(year),
                Ok(babylonian::is_leap_year(year))
            );
            assert_eq!(
                DynAdapter::new(IslamicCivilCalendar).is_leap_year(1_300 + year),
                Ok(tabular::is_leap_year(
                    tabular::LeapYearRule::CIVIL,
                    1_300 + year
                ))
            );
        }
        for year in (islamic_umalqura::FIRST_YEAR..=islamic_umalqura::LAST_YEAR).step_by(3) {
            assert_eq!(
                DynAdapter::new(IslamicUmmAlQuraCalendar).is_leap_year(year),
                Ok(islamic_umalqura::days_in_year(year) == Some(355))
            );
        }
        for year in (4_600..=4_700).step_by(7) {
            assert_eq!(
                DynAdapter::new(ChineseCalendar).is_leap_year(year),
                chinese::PARAMETERS.is_leap_year(year)
            );
        }
    }

    /// A Hebrew common year can be longer than the year before it; that
    /// never made it leap, and the dynamic interface no longer says so.
    #[test]
    fn a_longer_common_year_is_not_a_leap_year() {
        use hc_calendar::{DynAdapter, DynCalendar};

        use crate::{HebrewCalendar, hebrew};

        let calendar = DynAdapter::new(HebrewCalendar);
        let mut seen = 0;
        for year in 5_700..=5_800 {
            if !hebrew::is_leap_year(year)
                && hebrew::days_in_year(year) > hebrew::days_in_year(year - 1)
            {
                assert_eq!(calendar.is_leap_year(year), Ok(false), "{year}");
                seen += 1;
            }
        }
        assert!(seen > 0, "the century contains such years");
    }
}
