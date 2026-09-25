//! Solar and purely arithmetic calendars for `hyper-calendar`.
//!
//! Every calendar here decides what day it is by counting — days, months,
//! years, cycles — and never by asking where the sun is. That is the line
//! this crate draws: a calendar whose rule is "every fourth year" lives
//! here; one whose rule is "the day the equinox falls at Tehran" does not,
//! because the answer would depend on an ephemeris and would change when the
//! model behind it improved.
//!
//! Three calendars sit exactly on that line and are here anyway, under names
//! that say so: [`persian::ArithmeticPersianCalendar`] and
//! [`french_republican::ArithmeticFrenchRepublicanCalendar`] implement the
//! arithmetic *approximations* of calendars that are astronomically defined,
//! and [`bahai::ArithmeticBahaiCalendar`] implements the pre-2015 Western
//! form of a calendar that has since become astronomical. Each of those
//! modules documents what it is not, and the astronomical calendars they
//! approximate are in `hc-calendars-equinox`, under `persian`,
//! `bahai-astronomical` and `french-republican-equinox`. A fourth,
//! [`bahai_kept::BahaiCalendar`], is a *published table* rather than
//! astronomy — the Bahá'í World Centre's dates for 172–221 BE, joined to the
//! arithmetic calendar that was kept before — and stops where the table
//! does.
//!
//! # The shape of the crate
//!
//! | Family | Calendars |
//! | --- | --- |
//! | Julian/Gregorian structure | [`gregorian`], [`julian`], [`julian_gregorian`], [`swedish`], [`revised_julian`], [`byzantine`], [`roman`], [`rumi`] |
//! | Other namings of a Gregorian day | [`iso_week`], [`ordinal`], [`buddhist`], [`minguo`], [`juche`], [`holocene`], [`koki`], [`indian`], [`nanakshahi`], [`bangladeshi`], [`discordian`] |
//! | Twelve thirties plus epagomenal days | [`coptic`], [`ethiopic`], [`egyptian`], [`armenian`], [`armenian_fixed`], [`french_republican`], [`zoroastrian`] |
//! | Day counts | [`julian_day`], [`day_counts`] |
//! | Cycle-based | [`persian`], [`bahai`], [`bahai_kept`] |
//! | Proposed reforms | [`symmetry454`], [`symmetry010`] (both on [`symmetry`]), [`world_calendar`], [`international_fixed`], [`positivist`] |
//! | Not calendars | [`cycles`] (the computus cycles), [`year_style`] (where the year began) |
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

pub mod armenian;
pub mod armenian_fixed;
pub mod bahai;
pub mod bahai_kept;
pub mod bangladeshi;
pub mod buddhist;
pub mod byzantine;
pub mod coptic;
pub mod cycles;
pub mod day_counts;
pub mod discordian;
pub mod egyptian;
pub mod ethiopic;
pub mod french_republican;
pub mod gregorian;
pub mod holocene;
pub mod indian;
pub mod international_fixed;
pub mod iso_week;
pub mod juche;
pub mod julian;
pub mod julian_day;
pub mod julian_gregorian;
pub mod koki;
pub mod minguo;
pub mod nanakshahi;
pub mod ordinal;
pub mod persian;
pub mod positivist;
pub mod revised_julian;
pub mod roman;
pub mod rumi;
pub mod swedish;
pub mod symmetry;
pub mod symmetry010;
pub mod symmetry454;
pub mod world_calendar;
pub mod year_style;
pub mod zoroastrian;

pub use armenian::{ArmenianCalendar, ArmenianDate};
pub use armenian_fixed::{ArmenianFixedCalendar, ArmenianFixedDate};
pub use bahai::{ArithmeticBahaiCalendar, BahaiDate};
pub use bahai_kept::BahaiCalendar;
pub use bangladeshi::{BangladeshiCalendar, BangladeshiDate};
pub use buddhist::{BuddhistCalendar, BuddhistDate};
pub use byzantine::{ByzantineCalendar, ByzantineDate};
pub use coptic::{CopticCalendar, CopticDate};
pub use discordian::{DiscordianCalendar, DiscordianDate};
pub use egyptian::{EgyptianCalendar, EgyptianDate};
pub use ethiopic::{EthiopicCalendar, EthiopicDate};
pub use french_republican::{ArithmeticFrenchRepublicanCalendar, FrenchRepublicanDate};
pub use gregorian::{GregorianCalendar, GregorianDate};
pub use holocene::{HoloceneCalendar, HoloceneDate};
pub use indian::{IndianCalendar, IndianDate};
pub use international_fixed::{InternationalFixedCalendar, InternationalFixedDate};
pub use iso_week::{IsoWeekCalendar, IsoWeekDate};
pub use juche::{JucheCalendar, JucheDate};
pub use julian::{JulianCalendar, JulianDate};
pub use julian_day::{
    JulianDayCalendar, JulianDayNumber, ModifiedJulianDay, ModifiedJulianDayCalendar,
};
pub use julian_gregorian::{Adoption, ReformCalendar, ReformDate};
pub use koki::{KokiCalendar, KokiDate};
pub use minguo::{MinguoCalendar, MinguoDate};
pub use nanakshahi::{NanakshahiCalendar, NanakshahiDate};
pub use ordinal::{OrdinalCalendar, OrdinalDate};
pub use persian::{ArithmeticPersianCalendar, PersianDate};
pub use positivist::{PositivistCalendar, PositivistDate};
pub use revised_julian::{RevisedJulianCalendar, RevisedJulianDate};
pub use roman::{RomanCalendar, RomanDate};
pub use rumi::{RumiCalendar, RumiDate};
pub use swedish::{SwedishCalendar, SwedishDate};
pub use symmetry010::{Symmetry010Calendar, Symmetry010Date};
pub use symmetry454::{Symmetry454Calendar, Symmetry454Date};
pub use world_calendar::{WorldCalendar, WorldCalendarDate};
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
        registry.insert(Box::new(DynAdapter::new(crate::CopticCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::EthiopicCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::EgyptianCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::ArmenianCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::ArmenianFixedCalendar)));
        registry.insert(Box::new(DynAdapter::new(crate::ArithmeticPersianCalendar)));
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
        for zoroastrian in crate::ZoroastrianCalendar::ALL {
            registry.insert(Box::new(DynAdapter::new(zoroastrian)));
        }

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
const CALENDAR_COUNT: usize = 44;

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
                ZoroastrianCalendar::QADIMI,
                ZoroastrianCalendar::SHAHANSHAHI,
                ZoroastrianCalendar::FASLI,
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
            IndianCalendar.meta(),
            ArithmeticBahaiCalendar.meta(),
            BahaiCalendar.meta(),
            ByzantineCalendar.meta(),
            RomanCalendar.meta(),
            ArithmeticFrenchRepublicanCalendar.meta(),
            Symmetry454Calendar.meta(),
            SwedishCalendar.meta(),
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
        // Every calendar that claims the day renders it; the ones that do
        // not claim it are the Rumi calendar, kept only from 1840 to 1925,
        // and the Swedish calendar of 1700 to 1712.
        let supporting = registry.metas().filter(|meta| meta.supports(rd)).count();
        assert_eq!(rendered.len(), supporting);
        let unsupporting: Vec<&str> = registry
            .metas()
            .filter(|meta| !meta.supports(rd))
            .map(|meta| meta.id.0)
            .collect();
        assert_eq!(unsupporting, ["rumi", "swedish-1700"]);

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
}
