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
//! | Arithmetic | [`islamic_civil`], [`islamic_astronomical`], `islamic-fatimid` ([`tabular::FATIMID`]), [`hebrew`], [`tibetan`], [`javanese`], [`meyer_palmen`], [`yerm`], [`liberalia_lunar`], [`archetypes`] | a counting rule, exact by definition |
//! | Tabulated | [`islamic_umalqura`] | a table, exact where the table reaches |
//! | Computed | [`chinese`], [`dangi`], [`vietnamese`], [`japanese_tenpo`], [`islamic_observational`], [`hebrew_observational`], [`samaritan`], [`babylonian`] | an astronomical model, exact only to the model |
//! | Historical | [`japanese_historical`] | the system's *own* period constants, exact to the bureau that published it |
//!
//! The four rows behave differently and the crate does not pretend
//! otherwise. Arithmetic calendars are exact over the range each states,
//! and the range is a choice of the module rather than a limit of the
//! rule: the Hebrew and tabular Hijri calendars convert years 1 to 9 999,
//! the Tibetan 1000 to 3000, the Javanese its windu from 1555. The
//! tabulated one refuses every day outside 1300–1600 AH rather than
//! extrapolating. The computed ones carry bounded ranges, say what their
//! model is worth, and — in the observational Hijri case — say plainly that
//! they are predicting a human decision. The historical ones run on
//! ninth-, seventeenth- and eighteenth-century constants and reproduce those
//! systems' errors on purpose, because the errors are what the surviving
//! documents record.
//!
//! # Two engines under most of the calendars
//!
//! Almost nothing here is written twice.
//!
//! * [`tabular`] is the whole arithmetic Hijri calendar, with the
//!   intercalation scheme and the epoch as parameters. Four schemes times two
//!   epochs is eight calendars; [`islamic_civil`] and
//!   [`islamic_astronomical`] are the two CLDR names among them, and
//!   `islamic-fatimid` ([`tabular::FATIMID`]) the Bohra calendar's.
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
//! their own, though [`tibetan`] is itself one engine under four versions'
//! data — the Phugpa, the Tsurphu, the Bhutanese and the Mongolian — and
//! two conventions of its true date, registered as `tibetan-lochen`,
//! `tibetan-tsurphu-karana` and `tibetan-bhutan-lochen`; so does
//! [`javanese`], whose months are the tabular Hijri months but whose years
//! run in eight-year *windu* and 120-year *kurup*, three reckonings of one
//! rule; so does [`babylonian`], a nineteen-year cycle of intercalations
//! over months begun by a computed first sighting at Babylon; so does
//! [`samaritan`]; and so do the proposals, [`meyer_palmen`], a
//! lunisolar calendar of two remainders, [`archetypes`], another, with a
//! ten-day tweek, [`yerm`], a lunar one of 52-yerm cycles, and
//! [`liberalia_lunar`], a lunar one of three-day tridays; [`islamic_umalqura`]
//! stands alone because a table is not an algorithm, and
//! [`islamic_observational`] and [`hebrew_observational`] because they
//! predict a sighting.
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

/// How far apart the days of a day-by-day sweep in this crate's tests are:
/// every day in a release build, every `sampled`th in a debug build, and
/// about a third as many of those in a build instrumented for coverage
/// (`hc_core::sweep::thinned`).
///
/// The full sweeps take minutes under the coverage job's instrumentation,
/// so a debug or coverage run takes a fixed, deterministic sample of them
/// and CI's release-mode test job walks every day; the published anchors
/// are checked in full either way (docs/policy.md §7).
#[cfg(test)]
pub(crate) const fn sweep_stride(sampled: usize) -> usize {
    if cfg!(debug_assertions) {
        hc_core::sweep::thinned(sampled)
    } else {
        1
    }
}

/// The days of a day-by-day sweep over `first..=last` in this crate's
/// tests: every day in a release build; in a debug build every `sampled`th
/// day ([`sweep_stride`]) and, besides the sample, each of `boundaries` and
/// the day before it, where they fall in the range.
///
/// The boundaries are the days a year or a cycle begins, so that the
/// sample always holds the first and last day of every year it spans: an
/// error that falls on a year's first or last day cannot slip between the
/// sampled days (docs/policy.md §7).
#[cfg(test)]
pub(crate) fn sweep_days(
    first: i64,
    last: i64,
    sampled: usize,
    boundaries: impl IntoIterator<Item = i64>,
) -> impl Iterator<Item = i64> {
    let debug = cfg!(debug_assertions);
    (first..=last).step_by(sweep_stride(sampled)).chain(
        boundaries
            .into_iter()
            .filter(move |_| debug)
            .flat_map(|day| [day - 1, day])
            .filter(move |day| (first..=last).contains(day)),
    )
}

/// The years of a year-by-year sweep over `first..=last` in this crate's
/// tests, such as the years whose boundaries [`sweep_days`] is given: every
/// year in a release build; in a debug build every `sampled`th and the
/// last.
///
/// For a sweep whose year boundaries cost more than its sampled days,
/// hundreds of years of astronomical or lunisolar conversions. `sampled`
/// is chosen prime to the cycle the years run in, so that the sample still
/// holds every kind of year the test is about, leap and common and the
/// exceptions; a release build walks every year (docs/policy.md §7). A build
/// instrumented for coverage steps by the first prime of at least twice
/// `sampled` (`hc_core::sweep::thinned_year_step`).
#[cfg(test)]
pub(crate) fn sweep_years(first: i64, last: i64, sampled: usize) -> impl Iterator<Item = i64> {
    let stride = if cfg!(debug_assertions) {
        hc_core::sweep::thinned_year_step(sampled)
    } else {
        1
    };
    let last_is_sampled = (last - first) % stride as i64 == 0;
    (first..=last)
        .step_by(stride)
        .chain((!last_is_sampled && first <= last).then_some(last))
}

// `check_days`: run a check on each day of a sweep, spread over the
// machine's threads (`hc_core::sweep`).
hc_core::check_days_in_parallel!();

pub mod archetypes;
pub mod babylonian;
pub mod chinese;
pub mod chinese_historical;
pub mod dangi;
pub mod hebrew;
pub mod hebrew_observational;
pub mod islamic_astronomical;
pub mod islamic_civil;
pub mod islamic_fcna;
pub mod islamic_global;
pub mod islamic_observational;
pub mod islamic_umalqura;
pub mod japanese_historical;
pub mod japanese_tenpo;
pub mod javanese;
pub mod liberalia_lunar;
pub mod lunisolar;
pub mod meyer_palmen;
pub mod samaritan;
pub mod tabular;
pub mod tibetan;
pub mod vietnamese;
pub mod yerm;

pub use archetypes::{ArchetypesCalendar, ArchetypesDate};
pub use babylonian::{BabylonianCalendar, BabylonianDate};
pub use chinese::{ChineseCalendar, ChineseDate};
pub use chinese_historical::daming::DamingCalendar;
pub use chinese_historical::jingchu::JingchuCalendar;
pub use chinese_historical::kaihuang::KaihuangCalendar;
pub use chinese_historical::qianxiang::QianxiangCalendar;
pub use chinese_historical::sanji::SanjiCalendar;
pub use chinese_historical::sifen::SifenCalendar;
pub use chinese_historical::taichu::TaichuCalendar;
pub use chinese_historical::tianhe::TianheCalendar;
pub use chinese_historical::xinghe::XingheCalendar;
pub use chinese_historical::yuanjia::YuanjiaCalendar;
pub use chinese_historical::zhengguang::ZhengguangCalendar;
pub use dangi::{DangiCalendar, DangiDate, DangiKasiCalendar};
pub use hebrew::{HebrewCalendar, HebrewDate};
pub use hebrew_observational::ObservationalHebrewCalendar;
pub use islamic_astronomical::IslamicAstronomicalCalendar;
pub use islamic_civil::IslamicCivilCalendar;
pub use islamic_fcna::IslamicFcnaCalendar;
pub use islamic_global::{GlobalRule, IslamicGlobalCalendar};
pub use islamic_observational::{
    ArcOfLightCriterion, Frame, IslamicObservationalCalendar, NewMonthRule, ObservationSite,
    OdehZone, QTestCriterion, SunsetCriterion, VTestCriterion, VisibilityCriterion,
    YallopVisibility,
};
pub use islamic_umalqura::IslamicUmmAlQuraCalendar;
pub use japanese_historical::horyaku::HoryakuCalendar;
pub use japanese_historical::jokyo::JokyoCalendar;
pub use japanese_historical::kansei::KanseiCalendar;
pub use japanese_historical::senmyo::SenmyoCalendar;
pub use japanese_tenpo::{JapaneseTenpoCalendar, JapaneseTenpoDate};
pub use javanese::{JavaneseCalendar, JavaneseDate};
pub use liberalia_lunar::{LiberaliaLunarCalendar, LiberaliaLunarDate};
pub use lunisolar::{
    ConjunctionMode, EclipseSite, LunisolarCalendar, LunisolarDate, LunisolarParameters,
    MajorTermCorrection, MeanMotionModel, MeridianEra, MonthStartCorrection, SeasonalAdvance,
    SolarTermMode,
};
pub use meyer_palmen::{MeyerPalmenCalendar, MeyerPalmenDate};
pub use samaritan::{SamaritanCalendar, SamaritanDate};
pub use tabular::{IslamicDate, LeapYearRule, TabularIslamicCalendar};
pub use tibetan::{TibetanCalendar, TibetanDate};
pub use vietnamese::{VietnameseCalendar, VietnameseDate};
pub use yerm::{YermCalendar, YermDate};

/// Registration of every calendar in this crate, for the dynamic registry.
#[cfg(feature = "alloc")]
mod registration {
    hc_calendar::calendars! {
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
        /// Registering is idempotent: a second call replaces rather than
        /// duplicates, since [`hc_calendar::CalendarRegistry::register`] keys on the calendar's
        /// identifier.
        pub fn register_all;

        crate::ChineseCalendar,
        crate::DangiCalendar,
        crate::DangiKasiCalendar,
        crate::VietnameseCalendar,
        crate::JapaneseTenpoCalendar,
        crate::KanseiCalendar,
        crate::HoryakuCalendar,
        crate::JokyoCalendar,
        crate::SenmyoCalendar,
        crate::TaichuCalendar,
        crate::SifenCalendar,
        crate::QianxiangCalendar,
        crate::JingchuCalendar,
        crate::YuanjiaCalendar,
        crate::DamingCalendar,
        crate::XingheCalendar,
        crate::TianheCalendar,
        crate::KaihuangCalendar,
        crate::SanjiCalendar,
        crate::ZhengguangCalendar,
        crate::tabular::FATIMID,
        crate::HebrewCalendar,
        crate::ObservationalHebrewCalendar,
        crate::SamaritanCalendar,
        crate::BabylonianCalendar,
        crate::tibetan::TIBETAN,
        crate::tibetan::TIBETAN_TSURPHU,
        crate::tibetan::TIBETAN_BHUTAN,
        crate::tibetan::MONGOLIAN,
        crate::tibetan::TIBETAN_LOCHEN,
        crate::tibetan::TIBETAN_TSURPHU_KARANA,
        crate::tibetan::TIBETAN_BHUTAN_LOCHEN,
        crate::javanese::JAVANESE,
        crate::javanese::JAVANESE_YOGYAKARTA,
        crate::javanese::JAVANESE_ABOGE,
        crate::MeyerPalmenCalendar,
        crate::YermCalendar,
        crate::LiberaliaLunarCalendar,
        crate::ArchetypesCalendar,
        crate::IslamicCivilCalendar,
        crate::IslamicAstronomicalCalendar,
        crate::IslamicUmmAlQuraCalendar,
        crate::IslamicFcnaCalendar,
        crate::IslamicObservationalCalendar::MECCA,
        crate::IslamicObservationalCalendar::CAIRO_RD,
        crate::IslamicObservationalCalendar::SAUDI_RULE_RD,
        crate::IslamicGlobalCalendar::KHGT,
        crate::IslamicGlobalCalendar::ISTANBUL_2016,
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
        assert_eq!(registry.len(), 48);
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
            IslamicUmmAlQuraCalendar, babylonian, chinese, hebrew, islamic_umalqura, tabular,
            tibetan,
        };

        for year in (1..=400).step_by(3) {
            assert_eq!(
                DynAdapter::new(HebrewCalendar).is_leap_year(5_700 + year),
                Ok(hebrew::is_leap_year(5_700 + year))
            );
            for calendar in [
                tibetan::TIBETAN,
                tibetan::TIBETAN_TSURPHU,
                tibetan::TIBETAN_BHUTAN,
                tibetan::MONGOLIAN,
                tibetan::TIBETAN_LOCHEN,
                tibetan::TIBETAN_TSURPHU_KARANA,
                tibetan::TIBETAN_BHUTAN_LOCHEN,
            ] {
                assert_eq!(
                    DynAdapter::new(calendar).is_leap_year(1_900 + year),
                    Ok(calendar.is_leap_year(1_900 + year))
                );
            }
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
            assert_eq!(
                DynAdapter::new(crate::ArchetypesCalendar).is_leap_year(year),
                Ok(crate::archetypes::is_long_year(year))
            );
            assert_eq!(
                DynAdapter::new(crate::LiberaliaLunarCalendar).is_leap_year(year),
                Ok(crate::liberalia_lunar::is_long_year(year))
            );
        }
    }

    /// A Hebrew common year can be longer than the year before it; that
    /// does not make it leap, and the dynamic interface says it is not.
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
