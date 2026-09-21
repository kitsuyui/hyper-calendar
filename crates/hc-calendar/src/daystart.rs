//! Where a calendar's day begins, and when it was really in use.
//!
//! # Two things `Rd` deliberately does not say
//!
//! [`Rd`](crate::Rd) is a day number and nothing else. Two facts about a
//! calendar sit just outside it, and both were previously only in prose:
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
//! `CalendarMeta` is built as a struct literal in ninety-six places across
//! five crates. Adding fields would have meant editing all of them to say
//! "midnight, no restriction", which is the answer for most of them and tells
//! a reader nothing. A default trait method says the same thing once, and a
//! calendar that differs overrides it — so the override itself becomes the
//! documentation.

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
/// reporting that today is Shōwa 101. The arithmetic is perfectly happy to
/// say so; nobody has written it since 1989.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Usage {
    /// The first day the calendar was in force, if it has one.
    pub from: Option<Rd>,
    /// The last day it was in force, if it has one.
    pub until: Option<Rd>,
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
    /// Shōwa 101 is this.
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
    };

    /// A calendar in use from a day onwards.
    #[must_use]
    pub const fn since(from: Rd) -> Self {
        Self {
            from: Some(from),
            until: None,
        }
    }

    /// A calendar in use between two days, inclusive.
    #[must_use]
    pub const fn between(from: Rd, until: Rd) -> Self {
        Self {
            from: Some(from),
            until: Some(until),
        }
    }

    /// How a day relates to this period.
    #[must_use]
    pub fn standing(self, rd: Rd) -> Standing {
        match (self.from, self.until) {
            (None, None) => Standing::Unrecorded,
            (Some(from), _) if rd < from => Standing::Proleptic,
            (_, Some(until)) if rd > until => Standing::Extended,
            _ => Standing::InUse,
        }
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
        let usage = Usage::between(Rd(100), Rd(200));
        assert_eq!(usage.standing(Rd(99)), Standing::Proleptic);
        assert_eq!(usage.standing(Rd(100)), Standing::InUse);
        assert_eq!(usage.standing(Rd(200)), Standing::InUse);
        assert_eq!(usage.standing(Rd(201)), Standing::Extended);
        assert!(usage.standing(Rd(150)).is_historical());
        assert!(!usage.standing(Rd(201)).is_historical());
    }

    #[test]
    fn an_open_ended_period_never_ends() {
        let usage = Usage::since(Rd(100));
        assert_eq!(usage.standing(Rd(99)), Standing::Proleptic);
        assert_eq!(usage.standing(Rd(1_000_000)), Standing::InUse);
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
