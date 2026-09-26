//! Solar and purely arithmetic calendars for `hyper-calendar`.
//!
//! Every calendar here decides what day it is by counting — days, months,
//! years, cycles — and never by asking where the sun is. That is the line
//! this crate draws: a calendar whose rule is "every fourth year" lives
//! here; one whose rule is "the day the equinox falls at Tehran" does not,
//! because the answer would depend on an ephemeris and would change when the
//! model behind it improved.
//!
//! Four calendars sit exactly on that line and are here anyway, under names
//! that say so: [`persian::ArithmeticPersianCalendar`] and
//! [`persian_33::ThirtyThreeYearPersianCalendar`], the two published cycles
//! for the Iranian year, and
//! [`french_republican::ArithmeticFrenchRepublicanCalendar`] implement the
//! arithmetic *approximations* of calendars that are astronomically defined,
//! and [`bahai::ArithmeticBahaiCalendar`] implements the pre-2015 Western
//! form of a calendar that has since become astronomical. Each of those
//! modules documents what it is not, and the astronomical calendars they
//! approximate are in `hc-calendars-equinox`, under `persian`,
//! `bahai-astronomical` and `french-republican-equinox`. A fifth,
//! [`bahai_kept::BahaiCalendar`], is a *published table* rather than
//! astronomy — the Bahá'í World Centre's dates for 172–221 BE, joined to the
//! arithmetic calendar that was kept before — and stops where the table
//! does. Two more are arithmetic rules for an astronomical calendar as
//! well: [`french_republican_richards::RichardsFrenchRepublicanCalendar`],
//! Richards's rule for the Republican year, and
//! [`jalali_tusi::JalaliTusiCalendar`], Ṭūsī's table for the Jalālī year,
//! whose astronomical form is not carried.
//!
//! # The shape of the crate
//!
//! | Family | Calendars |
//! | --- | --- |
//! | Julian/Gregorian structure | [`gregorian`], [`julian`], [`julian_gregorian`], [`swedish`], [`revised_julian`], [`byzantine`], [`roman`], [`rumi`], [`berber`], [`yazidi`], [`icelandic`] |
//! | Year counts over the Julian or Gregorian year | [`year_counts`] (the Spanish era, the Masonic years, ADA), [`era_fascista`], [`syro_macedonian`] (the Seleucid era in its Syrian form, the eras of Antioch and Gaza) |
//! | Other namings of a Gregorian day | [`iso8601`], [`iso_week`], [`ordinal`], [`buddhist`], [`minguo`], [`juche`], [`holocene`], [`koki`], [`indian`], [`nanakshahi`], [`bangladeshi`], [`discordian`], [`assyrian`], [`soviet_week`] |
//! | Twelve thirties plus epagomenal days | [`coptic`], [`ethiopic`], [`egyptian`], [`philip_era`], [`bostran`], [`armenian`], [`armenian_fixed`], [`french_republican`], [`french_republican_richards`], [`zoroastrian`], [`mandaean`], [`jalali_tusi`] |
//! | Day counts | [`julian_day`], [`day_counts`], [`spreadsheet`] |
//! | Cycle-based | [`persian`], [`persian_33`], [`bahai`], [`bahai_kept`] |
//! | Proposed reforms | [`symmetry454`], [`symmetry010`] (both on [`symmetry`]), [`hermetic_leap_week`] (on the same leap-week engine), [`world_calendar`], [`international_fixed`], [`positivist`], [`hanke_henry`], [`week_and_month`], [`dee`] (the Dee–Cecil and Dee calendars), [`liberalia`], [`tabot`] |
//! | An instant system over TAI | [`terran`] (the Terran Computational Calendar, not a day calendar and not registered) |
//! | A pure week cycle | [`qumran`] |
//! | Not calendars | [`cycles`] (the computus cycles), [`year_style`] (where the year began), [`adoption`] (when each country took the Gregorian calendar) |
//!
//! Everything converts through [`hc_calendar::Rd`], so any two of them can
//! be put side by side without either knowing the other exists:
//!
//! ```
//! use hc_calendar::Calendar;
//! use hc_calendars_solar::{EthiopicCalendar, GregorianCalendar, GregorianDate};
//!
//! // Ten days after Enkutatash, so the Ethiopian year has already turned.
//! let day = GregorianDate::new(2026, 9, 21)?;
//! let ethiopian = GregorianCalendar.convert_to(day, &EthiopicCalendar)?;
//! assert_eq!((ethiopian.year, ethiopian.month, ethiopian.day), (2019, 1, 11));
//! # Ok::<(), hc_calendar::CalendarError>(())
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod common;
mod leap_week;

pub mod adoption;
pub mod armenian;
pub mod armenian_fixed;
pub mod assyrian;
pub mod bahai;
pub mod bahai_kept;
pub mod bangladeshi;
pub mod berber;
pub mod bostran;
pub mod buddhist;
pub mod byzantine;
pub mod coptic;
pub mod cycles;
pub mod day_counts;
pub mod dee;
pub mod discordian;
pub mod egyptian;
pub mod era_fascista;
pub mod ethiopic;
pub mod french_republican;
pub mod french_republican_richards;
pub mod gregorian;
pub mod hanke_henry;
pub mod hermetic_leap_week;
pub mod holocene;
pub mod icelandic;
pub mod indian;
pub mod international_fixed;
pub mod iso8601;
pub mod iso_week;
pub mod jalali_tusi;
pub mod juche;
pub mod julian;
pub mod julian_day;
pub mod julian_gregorian;
pub mod koki;
pub mod liberalia;
pub mod mandaean;
pub mod minguo;
pub mod nanakshahi;
pub mod ordinal;
pub mod persian;
pub mod persian_33;
pub mod philip_era;
pub mod positivist;
pub mod qumran;
pub mod revised_julian;
pub mod roman;
pub mod rumi;
pub mod soviet_week;
pub mod spreadsheet;
pub mod swedish;
pub mod symmetry;
pub mod symmetry010;
pub mod symmetry454;
pub mod syro_macedonian;
pub mod tabot;
pub mod terran;
pub mod week_and_month;
pub mod world_calendar;
pub mod yazidi;
pub mod year_counts;
pub mod year_style;
pub mod zoroastrian;

pub use armenian::{ArmenianCalendar, ArmenianDate};
pub use armenian_fixed::{ArmenianFixedCalendar, ArmenianFixedDate};
pub use assyrian::{AssyrianCalendar, AssyrianDate};
pub use bahai::{ArithmeticBahaiCalendar, BahaiDate};
pub use bahai_kept::BahaiCalendar;
pub use bangladeshi::{BangladeshiCalendar, BangladeshiDate};
pub use berber::{BerberCalendar, BerberDate};
pub use bostran::{BostranCalendar, BostranDate};
pub use buddhist::{BuddhistCalendar, BuddhistDate};
pub use byzantine::{ByzantineCalendar, ByzantineDate};
pub use coptic::{CopticCalendar, CopticDate};
pub use dee::{DeeCalendar, DeeDate};
pub use discordian::{DiscordianCalendar, DiscordianDate};
pub use egyptian::{EgyptianCalendar, EgyptianDate};
pub use era_fascista::{EraFascistaCalendar, EraFascistaDate};
pub use ethiopic::{EthiopicCalendar, EthiopicDate};
pub use french_republican::{ArithmeticFrenchRepublicanCalendar, FrenchRepublicanDate};
pub use french_republican_richards::RichardsFrenchRepublicanCalendar;
pub use gregorian::{GregorianCalendar, GregorianDate};
pub use hanke_henry::{HankeHenryCalendar, HankeHenryDate};
pub use hermetic_leap_week::{HermeticLeapWeekCalendar, HermeticLeapWeekDate};
pub use holocene::{HoloceneCalendar, HoloceneDate};
pub use icelandic::{IcelandicCalendar, IcelandicDate};
pub use indian::{IndianCalendar, IndianDate};
pub use international_fixed::{InternationalFixedCalendar, InternationalFixedDate};
pub use iso_week::{IsoWeekCalendar, IsoWeekDate};
pub use iso8601::IsoCalendar;
pub use jalali_tusi::{JalaliDate, JalaliTusiCalendar};
pub use juche::{JucheCalendar, JucheDate};
pub use julian::{JulianCalendar, JulianDate};
pub use julian_day::{
    JulianDayCalendar, JulianDayNumber, ModifiedJulianDay, ModifiedJulianDayCalendar,
};
pub use julian_gregorian::{Adoption, ReformCalendar, ReformDate};
pub use koki::{KokiCalendar, KokiDate};
pub use liberalia::{LiberaliaSolarCalendar, LiberaliaSolarDate};
pub use mandaean::{MandaeanCalendar, MandaeanDate};
pub use minguo::{MinguoCalendar, MinguoDate};
pub use nanakshahi::{NanakshahiCalendar, NanakshahiDate};
pub use ordinal::{OrdinalCalendar, OrdinalDate};
pub use persian::{ArithmeticPersianCalendar, PersianDate};
pub use persian_33::ThirtyThreeYearPersianCalendar;
pub use philip_era::{PhilipEraCalendar, PhilipEraDate};
pub use positivist::{PositivistCalendar, PositivistDate};
pub use qumran::{QumranCalendar, QumranDate};
pub use revised_julian::{RevisedJulianCalendar, RevisedJulianDate};
pub use roman::{RomanCalendar, RomanDate};
pub use rumi::{RumiCalendar, RumiDate};
pub use soviet_week::{SovietWeekCalendar, SovietWeekDate};
pub use swedish::{SwedishCalendar, SwedishDate};
pub use symmetry010::{Symmetry010Calendar, Symmetry010Date};
pub use symmetry454::{Symmetry454Calendar, Symmetry454Date};
pub use syro_macedonian::{JulianEra, JulianEraCalendar, JulianEraDate};
pub use tabot::{TabotCalendar, TabotDate};
pub use week_and_month::{WeekAndMonthCalendar, WeekAndMonthDate};
pub use world_calendar::{WorldCalendar, WorldCalendarDate};
pub use yazidi::{YazidiCalendar, YazidiDate};
pub use year_counts::{YearCount, YearCountCalendar, YearCountDate};
pub use zoroastrian::{Reckoning, ZoroastrianCalendar, ZoroastrianDate};

#[cfg(feature = "alloc")]
mod registration {
    use alloc::boxed::Box;

    use hc_calendar::{CalendarRegistry, DynAdapter};

    use crate::julian_gregorian::{ADOPTIONS, ReformCalendar};

    /// Register every calendar in this crate with `registry`.
    ///
    /// The reform calendars are registered one per polity, under the
    /// identifiers in [`ADOPTIONS`], because a date written in Britain in
    /// 1700 and the same date written in Russia belong to genuinely
    /// different calendars rather than to one calendar with a parameter.
    /// That makes this function insert more entries than there are modules.
    ///
    /// Inserting is idempotent: a second call replaces rather than
    /// duplicates, since [`CalendarRegistry::insert`] keys on the
    /// calendar's identifier.
    pub fn register_all(registry: &mut CalendarRegistry) {
        registry.insert(Box::new(DynAdapter::new(crate::GregorianCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::JulianCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::IsoWeekCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::OrdinalCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::JulianDayCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::ModifiedJulianDayCalendar)));
        for count in crate::day_counts::ALL {
            registry.insert(Box::new(DynAdapter::new(
                crate::day_counts::DayCountCalendar(*count),
            )));
        }
        registry.insert(Box::new(DynAdapter::new(
            crate::spreadsheet::Excel1900Calendar,
        )));
        registry.insert(Box::new(DynAdapter::new(crate::CopticCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::EthiopicCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::EgyptianCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::ArmenianCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::ArmenianFixedCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::ArithmeticPersianCalendar)));
        registry.insert(Box::new(DynAdapter::new(
            crate::ThirtyThreeYearPersianCalendar,
        )));
        registry.insert(Box::new(DynAdapter::new(crate::IndianCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::NanakshahiCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::BangladeshiCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::DiscordianCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::BuddhistCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::MinguoCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::JucheCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::KokiCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::HoloceneCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::ByzantineCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::RomanCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::RumiCalendar)));
        registry.insert(Box::new(DynAdapter::new(
            crate::ArithmeticFrenchRepublicanCalendar,
        )));
        registry.insert(Box::new(DynAdapter::new(crate::ArithmeticBahaiCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::BahaiCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::Symmetry454Calendar)));
        registry.insert(Box::new(DynAdapter::new(crate::Symmetry010Calendar)));
        registry.insert(Box::new(DynAdapter::new(crate::RevisedJulianCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::WorldCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::InternationalFixedCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::PositivistCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::SwedishCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::BerberCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::MandaeanCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::AssyrianCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::YazidiCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::HankeHenryCalendar)));
        for icelandic in crate::IcelandicCalendar::ALL {
            registry.insert(Box::new(DynAdapter::new(icelandic)));
        }
        registry.insert(Box::new(DynAdapter::new(crate::QumranCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::SovietWeekCalendar)));
        for zoroastrian in crate::ZoroastrianCalendar::ALL {
            registry.insert(Box::new(DynAdapter::new(zoroastrian)));
        }
        registry.insert(Box::new(DynAdapter::new(crate::IsoCalendar)));
        for count in crate::year_counts::ALL {
            registry.insert(Box::new(DynAdapter::new(crate::YearCountCalendar(*count))));
        }
        registry.insert(Box::new(DynAdapter::new(crate::PhilipEraCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::BostranCalendar)));
        for era in crate::syro_macedonian::ALL {
            registry.insert(Box::new(DynAdapter::new(crate::JulianEraCalendar(*era))));
        }
        registry.insert(Box::new(DynAdapter::new(crate::EraFascistaCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::JalaliTusiCalendar)));
        registry.insert(Box::new(DynAdapter::new(
            crate::RichardsFrenchRepublicanCalendar,
        )));
        for dee in crate::dee::ALL {
            registry.insert(Box::new(DynAdapter::new(dee)));
        }
        registry.insert(Box::new(DynAdapter::new(crate::HermeticLeapWeekCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::WeekAndMonthCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::LiberaliaSolarCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::TabotCalendar)));

        for adoption in ADOPTIONS {
            if let Ok(reform) = ReformCalendar::new(adoption) {
                registry.insert(Box::new(DynAdapter::new(reform)));
            }
        }
    }
}

#[cfg(feature = "alloc")]
pub use registration::register_all;

/// How many calendars [`register_all`] inserts, not counting the reform
/// variants.
#[cfg(test)]
const CALENDAR_COUNT: usize = 81;

#[cfg(test)]
mod tests {
    use hc_calendar::{Calendar, Rd};

    use super::*;

    /// Round-trip one fixed day through every calendar that supports it,
    /// returning how many did.
    ///
    /// Written as a macro rather than a collection of trait objects so that
    /// the test exercises the *static* interface, where each calendar has
    /// its own date type, rather than only the erased one.
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
        // One loop over the whole crate: whatever a calendar claims to
        // support it must convert both ways, and must survive the trip
        // through calendar-agnostic fields as well.
        for rd in (-400_000..=1_100_000).step_by(313) {
            let checked = round_trip_every_calendar!(
                Rd(rd),
                GregorianCalendar,
                JulianCalendar,
                IsoWeekCalendar,
                OrdinalCalendar,
                JulianDayCalendar,
                ModifiedJulianDayCalendar,
                CopticCalendar,
                EthiopicCalendar,
                EgyptianCalendar,
                ArmenianCalendar,
                ArithmeticPersianCalendar,
                ThirtyThreeYearPersianCalendar,
                IndianCalendar,
                NanakshahiCalendar,
                BangladeshiCalendar,
                DiscordianCalendar,
                BuddhistCalendar,
                MinguoCalendar,
                JucheCalendar,
                HoloceneCalendar,
                ByzantineCalendar,
                RomanCalendar,
                RumiCalendar,
                ArithmeticFrenchRepublicanCalendar,
                ArithmeticBahaiCalendar,
                BahaiCalendar,
                Symmetry454Calendar,
                WorldCalendar,
                InternationalFixedCalendar,
                PositivistCalendar,
                SwedishCalendar,
                BerberCalendar,
                MandaeanCalendar,
                AssyrianCalendar,
                YazidiCalendar,
                HankeHenryCalendar,
                IcelandicCalendar::GREGORIAN,
                IcelandicCalendar::JULIAN,
                QumranCalendar,
                SovietWeekCalendar,
                ZoroastrianCalendar::QADIMI,
                ZoroastrianCalendar::SHAHANSHAHI,
                ZoroastrianCalendar::FASLI,
                IsoCalendar,
                YearCountCalendar(year_counts::SPANISH_ERA),
                YearCountCalendar(year_counts::ANNO_LUCIS),
                YearCountCalendar(year_counts::ANNO_INVENTIONIS),
                YearCountCalendar(year_counts::ANNO_DEPOSITIONIS),
                YearCountCalendar(year_counts::ANNO_ORDINIS),
                YearCountCalendar(year_counts::ADA),
                PhilipEraCalendar,
                BostranCalendar,
                JulianEraCalendar(syro_macedonian::SELEUCID_SYRIAN),
                JulianEraCalendar(syro_macedonian::ANTIOCH_OCTOBER),
                JulianEraCalendar(syro_macedonian::ANTIOCH_SEPTEMBER),
                JulianEraCalendar(syro_macedonian::GAZA),
                EraFascistaCalendar,
                JalaliTusiCalendar,
                RichardsFrenchRepublicanCalendar,
                dee::DEE_CECIL,
                dee::DEE,
                HermeticLeapWeekCalendar,
                WeekAndMonthCalendar,
                LiberaliaSolarCalendar,
                TabotCalendar,
                ReformCalendar::default(),
            );
            assert!(checked > 0, "no calendar covered rd {rd}");
        }
    }

    #[test]
    fn bounded_calendars_refuse_days_outside_their_range() {
        // Each calendar is asked about the day just past either end of the
        // range it advertises, which is the only place the two can disagree.
        for meta in [
            GregorianCalendar.meta(),
            JulianCalendar.meta(),
            CopticCalendar.meta(),
            EthiopicCalendar.meta(),
            EgyptianCalendar.meta(),
            ArmenianCalendar.meta(),
            ArithmeticPersianCalendar.meta(),
            ThirtyThreeYearPersianCalendar.meta(),
            IndianCalendar.meta(),
            ArithmeticBahaiCalendar.meta(),
            BahaiCalendar.meta(),
            ByzantineCalendar.meta(),
            RomanCalendar.meta(),
            ArithmeticFrenchRepublicanCalendar.meta(),
            Symmetry454Calendar.meta(),
            SwedishCalendar.meta(),
            BerberCalendar.meta(),
            MandaeanCalendar.meta(),
            AssyrianCalendar.meta(),
            YazidiCalendar.meta(),
            HankeHenryCalendar.meta(),
            IcelandicCalendar::GREGORIAN.meta(),
            IcelandicCalendar::JULIAN.meta(),
            QumranCalendar.meta(),
            SovietWeekCalendar.meta(),
            IsoCalendar.meta(),
            YearCountCalendar(year_counts::SPANISH_ERA).meta(),
            YearCountCalendar(year_counts::ANNO_ORDINIS).meta(),
            PhilipEraCalendar.meta(),
            BostranCalendar.meta(),
            JulianEraCalendar(syro_macedonian::SELEUCID_SYRIAN).meta(),
            JulianEraCalendar(syro_macedonian::GAZA).meta(),
            EraFascistaCalendar.meta(),
            JalaliTusiCalendar.meta(),
            RichardsFrenchRepublicanCalendar.meta(),
            dee::DEE_CECIL.meta(),
            dee::DEE.meta(),
            HermeticLeapWeekCalendar.meta(),
            WeekAndMonthCalendar.meta(),
            LiberaliaSolarCalendar.meta(),
            TabotCalendar.meta(),
        ] {
            let first = meta.earliest.expect("bounded below");
            let last = meta.latest.expect("bounded above");
            assert!(meta.supports(first) && meta.supports(last), "{}", meta.id);
            assert!(
                !meta.supports(Rd(first.0 - 1)),
                "{} claimed one too early",
                meta.id
            );
            assert!(
                !meta.supports(Rd(last.0 + 1)),
                "{} claimed one too late",
                meta.id
            );
            assert!(meta.check_range(Rd(first.0 - 1)).is_err());
            assert!(meta.check_range(Rd(last.0 + 1)).is_err());
        }
        assert!(CopticCalendar.from_fixed(Rd(0)).is_err());
        assert!(IndianCalendar.from_fixed(Rd(0)).is_err());
        assert!(ArithmeticBahaiCalendar.from_fixed(Rd(0)).is_err());
        assert!(BahaiCalendar.from_fixed(Rd(0)).is_err());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn the_registry_holds_every_calendar_under_a_distinct_identifier() {
        use alloc::vec::Vec;
        use hc_calendar::{CalendarId, CalendarRegistry};

        let mut registry = CalendarRegistry::new();
        register_all(&mut registry);
        assert_eq!(
            registry.len(),
            CALENDAR_COUNT + julian_gregorian::ADOPTIONS.len()
        );

        let ids: Vec<CalendarId> = registry.metas().map(|meta| meta.id).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "identifiers must be distinct");

        assert!(registry.get_by_name("gregory").is_some());
        assert!(registry.get_by_name("julian").is_some());
        assert!(registry.get_by_name("julian-gregorian-gb").is_some());
        assert!(registry.get_by_name("nonexistent").is_none());

        // Registering twice replaces rather than duplicates.
        register_all(&mut registry);
        assert_eq!(
            registry.len(),
            CALENDAR_COUNT + julian_gregorian::ADOPTIONS.len()
        );
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn one_day_renders_in_every_registered_calendar() {
        use hc_calendar::{CalendarId, CalendarRegistry};

        let mut registry = CalendarRegistry::new();
        register_all(&mut registry);

        let rd = gregorian::to_fixed(2026, 9, 21).unwrap();
        let rendered = registry.describe_day(rd);
        // Every calendar answers; the ones that refuse the day are the Rumi
        // calendar, kept only from 1840 to 1925, the Swedish calendar of 1700
        // to 1712, the Soviet weeks of 1929 to 1940, the Era Fascista's
        // Anni of 1922 to 1945 and the 295 years of Ṭūsī's Jalālī table.
        assert_eq!(rendered.len(), registry.len());
        let converted = rendered.iter().filter(|(_, fields)| fields.is_ok()).count();
        let supporting = registry.metas().filter(|meta| meta.supports(rd)).count();
        assert_eq!(converted, supporting);
        let refusing: Vec<&str> = rendered
            .iter()
            .filter(|(_, fields)| fields.is_err())
            .map(|(id, _)| id.0)
            .collect();
        assert_eq!(
            refusing,
            [
                "rumi",
                "swedish-1700",
                "soviet-week",
                "era-fascista",
                "jalali-tusi"
            ]
        );

        let gregorian_fields = registry
            .get(CalendarId("gregory"))
            .unwrap()
            .fixed_to_fields(rd)
            .unwrap();
        let converted = registry
            .convert(
                CalendarId("gregory"),
                &gregorian_fields,
                CalendarId("holocene"),
            )
            .unwrap();
        assert_eq!(converted.year, 12_026);
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
            // Only the Badíʿ calendars carry an intercalary period in the
            // month field, as an intercalary repetition of month 18.
            assert!(!meta.has_leap_months || meta.id.as_str().starts_with("bahai"));
            // Nothing in this crate depends on an astronomical model; the
            // ones that approximate an astronomically defined calendar say
            // so in their name and their documentation instead.
            assert!(!meta.is_astronomical, "{} claims to be", meta.id);
            if let (Some(first), Some(last)) = (meta.earliest, meta.latest) {
                assert!(first < last, "{} has an empty range", meta.id);
            }
        }
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

    /// The dynamic answer is the module's own rule, not a reading of year
    /// lengths.
    #[test]
    fn the_dynamic_leap_year_is_the_module_rule() {
        use hc_calendar::{DynAdapter, DynCalendar};

        let reform =
            ReformCalendar::new(julian_gregorian::adoption_by_id("julian-gregorian-gb").unwrap())
                .unwrap();
        for year in (1..=2_400).step_by(37) {
            assert_eq!(
                DynAdapter::new(GregorianCalendar).is_leap_year(year),
                Ok(gregorian::is_leap_year(year))
            );
            assert_eq!(
                DynAdapter::new(JulianCalendar).is_leap_year(year),
                Ok(julian::is_leap_year(year))
            );
            assert_eq!(
                DynAdapter::new(CopticCalendar).is_leap_year(year),
                Ok(coptic::is_leap_year(year))
            );
            assert_eq!(
                DynAdapter::new(ArithmeticPersianCalendar).is_leap_year(year),
                Ok(persian::is_leap_year(year))
            );
            assert_eq!(
                DynAdapter::new(ThirtyThreeYearPersianCalendar).is_leap_year(year),
                Ok(persian_33::is_leap_year(year))
            );
            assert_eq!(
                DynAdapter::new(ArithmeticFrenchRepublicanCalendar).is_leap_year(year),
                Ok(french_republican::is_leap_year(year))
            );
            assert_eq!(
                DynAdapter::new(RichardsFrenchRepublicanCalendar).is_leap_year(year),
                Ok(french_republican_richards::is_leap_year(year))
            );
            assert_eq!(
                DynAdapter::new(BostranCalendar).is_leap_year(year),
                Ok(bostran::is_leap_year(year))
            );
            assert_eq!(
                DynAdapter::new(ArithmeticBahaiCalendar).is_leap_year(year),
                Ok(bahai::is_leap_year(year))
            );
            assert_eq!(
                DynAdapter::new(Symmetry454Calendar).is_leap_year(year),
                Ok(symmetry::is_leap_year(year))
            );
            assert_eq!(
                DynAdapter::new(RevisedJulianCalendar).is_leap_year(year),
                Ok(revised_julian::is_leap_year(year))
            );
            assert_eq!(
                DynAdapter::new(WorldCalendar).is_leap_year(year),
                Ok(world_calendar::is_leap_year(year))
            );
            assert_eq!(
                DynAdapter::new(DiscordianCalendar).is_leap_year(year),
                Ok(discordian::is_leap_year(year))
            );
            assert_eq!(
                DynAdapter::new(ZoroastrianCalendar::FASLI).is_leap_year(year),
                Ok(ZoroastrianCalendar::FASLI.reckoning.is_leap_year(year))
            );
            assert_eq!(
                DynAdapter::new(IsoWeekCalendar).is_leap_year(year),
                iso_week::is_long_year(year)
            );
            assert_eq!(
                DynAdapter::new(HankeHenryCalendar).is_leap_year(year),
                iso_week::is_long_year(year)
            );
            assert_eq!(
                DynAdapter::new(WeekAndMonthCalendar).is_leap_year(year),
                iso_week::is_long_year(year)
            );
            for calendar in dee::ALL {
                assert_eq!(
                    DynAdapter::new(calendar).is_leap_year(year),
                    Ok(dee::is_leap_year(year))
                );
            }
            assert_eq!(
                DynAdapter::new(HermeticLeapWeekCalendar).is_leap_year(year),
                Ok(hermetic_leap_week::is_leap_year(year))
            );
            assert_eq!(
                DynAdapter::new(LiberaliaSolarCalendar).is_leap_year(year),
                Ok(!liberalia::is_short_year(year))
            );
            assert_eq!(
                DynAdapter::new(TabotCalendar).is_leap_year(year),
                Ok(tabot::is_long_year(year))
            );
            assert_eq!(
                DynAdapter::new(IcelandicCalendar::GREGORIAN).is_leap_year(year),
                Ok(icelandic::Rule::Gregorian.is_leap_year(year))
            );
            // Julian until 1752 in Britain, Gregorian after.
            let expected = if year < 1752 {
                julian::is_leap_year(year)
            } else {
                gregorian::is_leap_year(year)
            };
            assert_eq!(
                DynAdapter::new(reform).is_leap_year(year),
                Ok(expected),
                "{year}"
            );
        }
        for year in bahai_kept::MIN_YEAR..=bahai_kept::MAX_YEAR {
            assert_eq!(
                DynAdapter::new(BahaiCalendar).is_leap_year(year),
                Ok(bahai_kept::days_in_year(year) == Some(366))
            );
        }
    }
}
