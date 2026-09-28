//! Ptolemy's day from noon: the Egyptian wandering year of the era of
//! Nabonassar and of the Era of Philip, with the day beginning at noon at
//! the meridian of Alexandria — `egyptian-ptolemy` and
//! `philip-era-ptolemy`.
//!
//! [`crate::egyptian`] and [`crate::philip_era`] keep the civil midnight.
//! Ptolemy's own day began at noon: the era of Nabonassar begins "at noon
//! on 26 February 747 BC" (Wikipedia, "Nabonassar", `wikipedia-nabonassar`,
//! re-read 2026-09-29), the *Handy Tables*' Era of Philip at "noon, −323
//! November 12" (Chabás, `chabas2013`), and "the astronomical day had
//! begun at noon ever since Ptolemy chose to begin the days for his
//! astronomical observations at noon", because "the transit of the Sun
//! across the observer's meridian occurs at the same apparent time every
//! day of the year" (Wikipedia, "Julian day", `wikipedia-julian-day`,
//! re-read 2026-09-29, citing Toomer's translation of the *Almagest*, not
//! read). The meridian is Alexandria's: "the meridian of Alexandria is
//! chosen as that to which Ptolemy refers the commencement of the era of
//! Nabonassar" (Herschel, quoted there). The civil Egyptian day of
//! Ptolemy's time began at dawn instead (van Gent, "Almagest calculator",
//! `vangent-almagest`, read 2026-09-29), so the two are competing
//! conventions for the same days and have names of their own (policy §5).
//!
//! A day here is named by the civil day it begins on: the first day of
//! the era, 1 Thoth 1, is the day that begins at the noon of 26 February
//! 747 BC, whose civil day is 1 Thoth 1 under [`crate::egyptian`], so that
//! the morning of 27 February is still 1 Thoth. That is the Julian Day's
//! convention too, and van Gent's calculator reads a time as "local time
//! since noon" of the Egyptian date it is given; no source read states the
//! naming as a rule in words. The dates, months and years are the civil
//! calendars' own; only [`hc_calendar::Calendar::day_boundary`] differs.
//!
//! The system document is `docs/systems/era-counts.md` in the repository.

use hc_calendar::shape::EraName;
use hc_calendar::{
    Calendar, CalendarId, CalendarMeta, CalendarResult, DateFields, DayBoundary, DayNaming, Rd,
    Usage,
};

use crate::{EgyptianCalendar, PhilipEraCalendar, egyptian, philip_era};

/// One of the civil wandering-year calendars, reckoned with Ptolemy's day
/// from noon.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PtolemaicDayCalendar<C> {
    inner: C,
    id: &'static str,
    english_name: &'static str,
    era: &'static str,
    era_name: EraName,
}

/// The era of Nabonassar with Ptolemy's day, `egyptian-ptolemy`.
pub const EGYPTIAN_PTOLEMY: PtolemaicDayCalendar<EgyptianCalendar> = PtolemaicDayCalendar {
    inner: EgyptianCalendar,
    id: "egyptian-ptolemy",
    english_name: "Egyptian (Ptolemy's day from noon)",
    era: egyptian::ERA,
    era_name: EraName::new("Era of Nabonassar", ""),
};

/// The Era of Philip with Ptolemy's day, `philip-era-ptolemy`.
pub const PHILIP_ERA_PTOLEMY: PtolemaicDayCalendar<PhilipEraCalendar> = PtolemaicDayCalendar {
    inner: PhilipEraCalendar,
    id: "philip-era-ptolemy",
    english_name: "Era of Philip (Ptolemy's day from noon)",
    era: philip_era::ERA,
    era_name: EraName::new("Era of Philip", ""),
};

impl<C> PtolemaicDayCalendar<C> {
    /// The civil calendar whose days these are.
    #[must_use]
    pub const fn civil(&self) -> &C {
        &self.inner
    }
}

impl<C: Calendar> Calendar for PtolemaicDayCalendar<C> {
    type Date = C::Date;

    /// Unrecorded, as the civil calendars are: no source read bounds the
    /// centuries astronomers counted Ptolemy's days.
    fn usage(&self) -> Usage {
        Usage::UNRECORDED
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        self.inner.cycles()
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        self.inner.is_leap_year(year)
    }

    /// Noon at the meridian of Alexandria, the day named by the civil day
    /// it begins on.
    fn day_boundary(&self) -> DayBoundary {
        DayBoundary::Noon(DayNaming::ByStart)
    }

    /// The era's English name, which the locales key by the civil
    /// calendar's identifier.
    fn era_name(&self, code: &str) -> Option<EraName> {
        (code == self.era).then_some(self.era_name)
    }

    fn era_code(&self, index: usize) -> Option<&'static str> {
        (index == 0).then_some(self.era)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(self.id),
            english_name: self.english_name,
            ..self.inner.meta()
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        self.inner.to_fixed(date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        self.inner.from_fixed(rd)
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        self.inner.to_fields(date)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        self.inner.from_fields(fields)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::CivilTime;

    #[test]
    fn the_day_begins_at_noon_and_is_named_by_its_start() {
        let boundary = EGYPTIAN_PTOLEMY.day_boundary();
        assert_eq!(boundary, DayBoundary::Noon(DayNaming::ByStart));
        assert_eq!(PHILIP_ERA_PTOLEMY.day_boundary(), boundary);
        assert_eq!(EgyptianCalendar.day_boundary(), DayBoundary::Midnight);
        // The morning of 27 February 747 BC, civil 1 Thoth 2 … is still the
        // day that began at the noon of 26 February, 1 Thoth 1.
        let morning = CivilTime::hms(9, 0, 0).unwrap();
        assert_eq!(boundary.civil_day_offset(morning), Some(-1));
        assert_eq!(boundary.civil_day_offset(CivilTime::NOON), Some(0));
        let epoch =
            EGYPTIAN_PTOLEMY.to_fixed(EGYPTIAN_PTOLEMY.from_fixed(egyptian::EPOCH).unwrap());
        assert_eq!(epoch, Ok(egyptian::EPOCH));
        let next_morning = Rd(egyptian::EPOCH.0 + 1 + boundary.civil_day_offset(morning).unwrap());
        assert_eq!(
            EGYPTIAN_PTOLEMY.from_fixed(next_morning),
            EgyptianCalendar.from_fixed(egyptian::EPOCH)
        );
        // The Julian Day that begins at the same noon is JD 1 448 638.
        assert_eq!(egyptian::EPOCH.to_julian_day_number(), 1_448_638);
        assert_eq!(
            PHILIP_ERA_PTOLEMY.from_fixed(philip_era::EPOCH),
            PhilipEraCalendar.from_fixed(philip_era::EPOCH)
        );
    }

    #[test]
    fn the_dates_are_the_civil_calendars() {
        for rd in (egyptian::EPOCH.0..egyptian::EPOCH.0 + 400_000).step_by(97) {
            let date = EGYPTIAN_PTOLEMY.from_fixed(Rd(rd)).unwrap();
            assert_eq!(Ok(date), EgyptianCalendar.from_fixed(Rd(rd)));
            assert_eq!(EGYPTIAN_PTOLEMY.to_fixed(date), Ok(Rd(rd)));
            let fields = EGYPTIAN_PTOLEMY.to_fields(date).unwrap();
            assert_eq!(EGYPTIAN_PTOLEMY.from_fields(&fields), Ok(date));
            assert_eq!(fields.era, Some(egyptian::ERA));
        }
        let meta = EGYPTIAN_PTOLEMY.meta();
        assert_eq!(meta.id, CalendarId("egyptian-ptolemy"));
        assert_eq!(meta.earliest, EgyptianCalendar.meta().earliest);
        assert_eq!(
            PHILIP_ERA_PTOLEMY.meta().id,
            CalendarId("philip-era-ptolemy")
        );
        assert_eq!(
            EGYPTIAN_PTOLEMY
                .era_name("nabonassar")
                .map(|name| name.native),
            Some("Era of Nabonassar")
        );
        assert_eq!(PHILIP_ERA_PTOLEMY.era_code(0), Some("philip"));
        assert_eq!(PHILIP_ERA_PTOLEMY.era_code(1), None);
        assert_eq!(EGYPTIAN_PTOLEMY.is_leap_year(1), Ok(false));
        assert!(!EGYPTIAN_PTOLEMY.usage().is_recorded());
        assert_eq!(EGYPTIAN_PTOLEMY.cycles(), EgyptianCalendar.cycles());
        assert_eq!(EGYPTIAN_PTOLEMY.civil(), &EgyptianCalendar);
    }
}
