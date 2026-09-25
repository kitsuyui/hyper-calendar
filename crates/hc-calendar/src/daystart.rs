//! Where a calendar's day begins, and when it was really in use.
//!
//! # Two things `Rd` deliberately does not say
//!
//! [`Rd`] is a day number and nothing else. Two facts about a
//! calendar sit just outside it, and this module states both:
//!
//! * **A day does not have to begin at midnight.** The Julian Day begins at
//!   noon, the Hebrew and Islamic day at sunset, and the traditional Chinese
//!   day at 23:00 — which is why the sexagenary day pillar changes an hour
//!   before the civil date does.
//! * **A calendar has a period when it was actually used**, which is usually
//!   much shorter than the range over which its arithmetic is defined. Both
//!   are real and they answer different questions.
//!
//! # Why this is a trait method and not a `CalendarMeta` field
//!
//! "Midnight, no recorded period of use" is the answer for most calendars.
//! As a `CalendarMeta` field it would be repeated in every calendar's struct
//! literal, where it tells a reader nothing. A default trait method says it
//! once, and a calendar that differs overrides it — so the override itself
//! becomes the documentation.

use core::fmt;

use crate::fixed::Rd;
use crate::time::CivilTime;

/// The point in the day at which a calendar's date changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DayBoundary {
    /// Local midnight, the civil convention and the default.
    #[default]
    Midnight,
    /// Local noon, as the Julian Day and the astronomical day use.
    ///
    /// Astronomers counted from noon so that a single night's observations
    /// carried one date. The Julian Day still does.
    Noon,
    /// Sunset, as the Hebrew, Islamic and Bahá'í days do.
    ///
    /// The moment depends on latitude and time of year, so resolving it needs
    /// a location and an ephemeris — `hc-astro`'s `sunset`. This value names
    /// the convention; it does not compute it.
    Sunset,
    /// Sunrise, used by several Hindu reckonings and by the Vedic day.
    Sunrise,
    /// A fixed local time, such as the 23:00 start of the traditional Chinese
    /// day.
    LocalTime(CivilTime),
}

impl DayBoundary {
    /// Whether resolving this boundary needs a location and an ephemeris.
    ///
    /// The arithmetic boundaries can be applied with nothing but a clock; the
    /// solar ones cannot, and a caller that has only a date needs to know
    /// which it is dealing with.
    #[must_use]
    pub const fn needs_observation(self) -> bool {
        matches!(self, Self::Sunset | Self::Sunrise)
    }

    /// The offset from midnight, for the boundaries that have a fixed one.
    ///
    /// Returns `None` for [`DayBoundary::Sunset`] and
    /// [`DayBoundary::Sunrise`], which vary with place and season.
    #[must_use]
    pub const fn fixed_offset(self) -> Option<CivilTime> {
        match self {
            Self::Midnight => Some(CivilTime::MIDNIGHT),
            Self::Noon => Some(CivilTime::NOON),
            Self::LocalTime(time) => Some(time),
            Self::Sunset | Self::Sunrise => None,
        }
    }

    /// Which calendar day a wall-clock reading belongs to, relative to the
    /// day the clock itself names.
    ///
    /// Returns 0 when the reading is inside the calendar day that shares its
    /// civil date, and 1 when the calendar has already rolled over — the
    /// 23:00 case, where 23:30 on the 5th is already the 6th in the
    /// traditional Chinese reckoning.
    ///
    /// Returns `None` for the observational boundaries.
    #[must_use]
    pub fn civil_day_offset(self, time: CivilTime) -> Option<i64> {
        let start = self.fixed_offset()?;
        Some(i64::from(
            time.since_midnight() >= start.since_midnight() && start != CivilTime::MIDNIGHT,
        ))
    }
}

impl fmt::Display for DayBoundary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Midnight => f.write_str("midnight"),
            Self::Noon => f.write_str("noon"),
            Self::Sunset => f.write_str("sunset"),
            Self::Sunrise => f.write_str("sunrise"),
            Self::LocalTime(time) => write!(f, "{time}"),
        }
    }
}

/// When a calendar was actually in use, as distinct from where its arithmetic
/// is defined.
///
/// The two are rarely the same, and conflating them is how a library ends up
/// reporting that today is Kōki 2686. The arithmetic is perfectly happy to
/// say so; nobody has written the imperial year since 1945.
///
/// # Every period names its source
///
/// A period of use is a claim about the world, and the library carries the
/// statement, not the world — so a `Usage` that has a period has a
/// [`source`](Self::source), named the way the module's own documentation
/// names it: the decree, the book, the page and the date it was read. The
/// constructors take it as an argument, because a period without a source
/// is exactly what a reader cannot check. [`Usage::UNRECORDED`] has none,
/// and is the answer for a proposal nobody adopted, a day count, or a
/// calendar whose period the sources contest.
///
/// # Civil use and continued use are two different ends
///
/// The Chinese calendar stopped being China's civil calendar on 1 January
/// 1912 and has fixed the Spring Festival, the Mid-Autumn Festival and
/// every other traditional date since. A single end day would misreport
/// that either way: `until` 1911 says today's Chinese New Year is an
/// extension nobody keeps, and no `until` at all says the calendar is
/// still what the state writes. So a period has two ends:
/// [`until`](Self::until), after which nobody keeps the calendar at all,
/// and [`civil_until`](Self::civil_until), after which it went on in
/// religious, festival or traditional use only. [`Usage::standing`] looks at
/// the first; [`Usage::is_civil`] at both.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Usage {
    /// The first day the calendar was in force, if it has one.
    pub from: Option<Rd>,
    /// The last day it was in force, if it has one.
    pub until: Option<Rd>,
    /// The last day it was a civil calendar, where it outlived that as a
    /// religious, festival or traditional one.
    ///
    /// `None` when the record draws no such line: the calendar is civil
    /// for the whole of its period, or was never civil at all, and
    /// [`source`](Self::source) says which.
    pub civil_until: Option<Rd>,
    /// Where the period comes from: the decree, the book, the table or the
    /// page, spelled as the module documentation spells it. Empty only for
    /// [`Usage::UNRECORDED`].
    pub source: &'static str,
}

/// How a date relates to the period its calendar was actually used in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Standing {
    /// The calendar was in force on this day.
    InUse,
    /// The day precedes adoption. The arithmetic is defined; nobody wrote
    /// dates this way at the time.
    ///
    /// Japan's imperial year applied to 1850 is this: well defined, and not
    /// what any document of 1850 says.
    Proleptic,
    /// The day follows the calendar's abandonment. The arithmetic is defined
    /// and is not what anybody uses.
    ///
    /// Kōki 2686, the imperial year of 2026, is this.
    Extended,
    /// The calendar has no recorded period of use, so there is nothing to be
    /// outside of. Proposed calendars and pure day counts are here.
    Unrecorded,
}

impl Standing {
    /// Whether a date of this standing is a historical reading rather than a
    /// back- or forward-projection.
    #[must_use]
    pub const fn is_historical(self) -> bool {
        matches!(self, Self::InUse)
    }
}

impl Usage {
    /// A calendar with no recorded period of use.
    pub const UNRECORDED: Self = Self {
        from: None,
        until: None,
        civil_until: None,
        source: "",
    };

    /// A calendar in use from a day onwards, on the authority of `source`.
    #[must_use]
    pub const fn since(from: Rd, source: &'static str) -> Self {
        Self {
            from: Some(from),
            until: None,
            civil_until: None,
            source,
        }
    }

    /// A calendar in use between two days, inclusive, on the authority of
    /// `source`.
    #[must_use]
    pub const fn between(from: Rd, until: Rd, source: &'static str) -> Self {
        Self {
            from: Some(from),
            until: Some(until),
            civil_until: None,
            source,
        }
    }

    /// A calendar in use up to a day, from a beginning `source` does not
    /// date.
    ///
    /// The Byzantine world era is this: Russia kept it until Peter I
    /// replaced it on 1 January 1700, and "from the seventh century" is as
    /// close as the sources come to a first day.
    #[must_use]
    pub const fn until(until: Rd, source: &'static str) -> Self {
        Self {
            from: None,
            until: Some(until),
            civil_until: None,
            source,
        }
    }

    /// A calendar `source` attests in use, from a beginning it does not
    /// date and with no end.
    ///
    /// The Balinese Pawukon and the Hindu solar calendars are this: kept
    /// today, older than any source read dates, and never abandoned. Every
    /// day is [`Standing::InUse`], which is right for every day such a
    /// calendar converts and says nothing about the days before its
    /// sources begin — a calendar whose beginning matters should find one.
    #[must_use]
    pub const fn undated(source: &'static str) -> Self {
        Self {
            from: None,
            until: None,
            civil_until: None,
            source,
        }
    }

    /// The same period, with civil use ending on `day` and the calendar
    /// going on in religious, festival or traditional use afterwards.
    ///
    /// The source of the period is expected to date this end as well.
    #[must_use]
    pub const fn civil_until(self, day: Rd) -> Self {
        Self {
            civil_until: Some(day),
            ..self
        }
    }

    /// Whether a period of use is recorded at all.
    ///
    /// False for [`Usage::UNRECORDED`] and for nothing else: every
    /// constructor but that one names a source, and a period with an end
    /// or a source is a record.
    #[must_use]
    pub const fn is_recorded(self) -> bool {
        self.from.is_some()
            || self.until.is_some()
            || self.civil_until.is_some()
            || !self.source.is_empty()
    }

    /// How a day relates to this period.
    #[must_use]
    pub fn standing(self, rd: Rd) -> Standing {
        if !self.is_recorded() {
            return Standing::Unrecorded;
        }
        match (self.from, self.until) {
            (Some(from), _) if rd < from => Standing::Proleptic,
            (_, Some(until)) if rd > until => Standing::Extended,
            _ => Standing::InUse,
        }
    }

    /// Whether the calendar was the civil calendar on this day: in use, and
    /// not past the end of its civil use where the record has one.
    #[must_use]
    pub fn is_civil(self, rd: Rd) -> bool {
        self.standing(rd).is_historical() && self.civil_until.is_none_or(|last| rd <= last)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn midnight_is_the_default_and_needs_nothing() {
        assert_eq!(DayBoundary::default(), DayBoundary::Midnight);
        assert!(!DayBoundary::Midnight.needs_observation());
        assert_eq!(
            DayBoundary::Midnight.fixed_offset(),
            Some(CivilTime::MIDNIGHT)
        );
    }

    #[test]
    fn the_solar_boundaries_admit_they_need_an_ephemeris() {
        assert!(DayBoundary::Sunset.needs_observation());
        assert!(DayBoundary::Sunrise.needs_observation());
        assert_eq!(DayBoundary::Sunset.fixed_offset(), None);
        assert_eq!(DayBoundary::Sunrise.civil_day_offset(CivilTime::NOON), None);
    }

    #[test]
    fn a_twenty_three_hundred_start_rolls_the_day_over_early() {
        let eleven_pm = CivilTime::hms(23, 0, 0).unwrap();
        let boundary = DayBoundary::LocalTime(eleven_pm);
        // Half past eleven is already the next calendar day.
        assert_eq!(
            boundary.civil_day_offset(CivilTime::hms(23, 30, 0).unwrap()),
            Some(1)
        );
        // Anything earlier is not.
        assert_eq!(
            boundary.civil_day_offset(CivilTime::hms(22, 59, 59).unwrap()),
            Some(0)
        );
        assert_eq!(boundary.civil_day_offset(CivilTime::NOON), Some(0));
    }

    #[test]
    fn midnight_never_rolls_the_day_over() {
        for hour in 0..24 {
            let time = CivilTime::hms(hour, 0, 0).unwrap();
            assert_eq!(DayBoundary::Midnight.civil_day_offset(time), Some(0));
        }
    }

    #[test]
    fn noon_rolls_over_in_the_afternoon() {
        assert_eq!(DayBoundary::Noon.civil_day_offset(CivilTime::NOON), Some(1));
        assert_eq!(
            DayBoundary::Noon.civil_day_offset(CivilTime::hms(11, 59, 59).unwrap()),
            Some(0)
        );
    }

    #[test]
    fn an_unrecorded_calendar_has_nothing_to_be_outside_of() {
        assert_eq!(Usage::UNRECORDED.standing(Rd(0)), Standing::Unrecorded);
        assert!(!Standing::Unrecorded.is_historical());
    }

    #[test]
    fn a_closed_period_separates_the_three_standings() {
        let usage = Usage::between(Rd(100), Rd(200), "a test");
        assert_eq!(usage.standing(Rd(99)), Standing::Proleptic);
        assert_eq!(usage.standing(Rd(100)), Standing::InUse);
        assert_eq!(usage.standing(Rd(200)), Standing::InUse);
        assert_eq!(usage.standing(Rd(201)), Standing::Extended);
        assert!(usage.standing(Rd(150)).is_historical());
        assert!(!usage.standing(Rd(201)).is_historical());
    }

    #[test]
    fn an_open_ended_period_never_ends() {
        let usage = Usage::since(Rd(100), "a test");
        assert_eq!(usage.standing(Rd(99)), Standing::Proleptic);
        assert_eq!(usage.standing(Rd(1_000_000)), Standing::InUse);
    }

    #[test]
    fn a_period_carries_its_source_and_unrecorded_has_none() {
        assert_eq!(Usage::since(Rd(1), "a decree").source, "a decree");
        assert!(Usage::since(Rd(1), "a decree").is_recorded());
        assert!(Usage::between(Rd(1), Rd(2), "a decree").is_recorded());
        assert!(!Usage::UNRECORDED.is_recorded());
        assert_eq!(Usage::UNRECORDED.source, "");
        assert_eq!(Usage::default(), Usage::UNRECORDED);
    }

    #[test]
    fn a_period_can_be_open_at_either_end() {
        let until = Usage::until(Rd(200), "a test");
        assert_eq!(until.standing(Rd(-1_000_000)), Standing::InUse);
        assert_eq!(until.standing(Rd(200)), Standing::InUse);
        assert_eq!(until.standing(Rd(201)), Standing::Extended);
        let undated = Usage::undated("a test");
        assert!(undated.is_recorded());
        assert_eq!(undated.standing(Rd(-1_000_000)), Standing::InUse);
        assert_eq!(undated.standing(Rd(1_000_000)), Standing::InUse);
        assert!(undated.is_civil(Rd(0)));
        assert!(!undated.civil_until(Rd(0)).is_civil(Rd(1)));
    }

    #[test]
    fn civil_use_can_end_before_use_does() {
        // In use from 100, civil until 150, kept for festivals afterwards.
        let usage = Usage::since(Rd(100), "a test").civil_until(Rd(150));
        assert_eq!(usage.standing(Rd(151)), Standing::InUse);
        assert!(usage.is_civil(Rd(100)));
        assert!(usage.is_civil(Rd(150)));
        assert!(!usage.is_civil(Rd(151)));
        assert!(!usage.is_civil(Rd(99)));
        // Without the line, civil use is the whole period.
        let plain = Usage::between(Rd(100), Rd(200), "a test");
        assert!(plain.is_civil(Rd(200)));
        assert!(!plain.is_civil(Rd(201)));
        assert!(!Usage::UNRECORDED.is_civil(Rd(0)));
    }

    #[test]
    fn boundaries_render_readably() {
        assert_eq!(DayBoundary::Midnight.to_string(), "midnight");
        assert_eq!(DayBoundary::Noon.to_string(), "noon");
        assert_eq!(DayBoundary::Sunset.to_string(), "sunset");
        assert_eq!(
            DayBoundary::LocalTime(CivilTime::hms(23, 0, 0).unwrap()).to_string(),
            "23:00:00"
        );
    }
}
