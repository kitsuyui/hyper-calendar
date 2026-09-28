//! The calendars of another body's days that a date at an instant is
//! written in: the five circad calendars of [`crate::circad`] and Mars's
//! Martiana ([`crate::mars::martiana`]), whose day is a sol.
//!
//! One table, so that a caller who selects one of them by its identifier —
//! as the boundaries' `hc_circad_date` does — looks it up here and keeps no
//! list of its own (`docs/policy.md` §5).

use hc_calendar::Calendar as _;

use crate::circad::CircadCalendar;
use crate::mars::MartianaCalendar;

/// A calendar of another body's days whose date at an instant this crate
/// reckons.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DatedCalendar {
    /// A circad calendar: Titan's or a Galilean moon's.
    Circad(CircadCalendar),
    /// Gangale's Martiana, the Darian calendar with Aitken's week.
    Martiana,
}

/// Every calendar a date at an instant is written in, the circad calendars
/// in [`crate::circad::all`]'s order and then Martiana.
pub const ALL: [DatedCalendar; 6] = {
    let circads = crate::circad::all();
    [
        DatedCalendar::Circad(circads[0]),
        DatedCalendar::Circad(circads[1]),
        DatedCalendar::Circad(circads[2]),
        DatedCalendar::Circad(circads[3]),
        DatedCalendar::Circad(circads[4]),
        DatedCalendar::Martiana,
    ]
};

impl DatedCalendar {
    /// The calendar's identifier: the circad rule's, or `martiana`.
    #[must_use]
    pub fn id(&self) -> &'static str {
        match self {
            Self::Circad(calendar) => calendar.rule().id,
            Self::Martiana => MartianaCalendar.meta().id.0,
        }
    }
}

/// The calendar with this identifier, by [`hc_core::catalogue::matches`].
#[must_use]
pub fn by_id(id: &str) -> Option<DatedCalendar> {
    ALL.into_iter()
        .find(|calendar| hc_core::catalogue::matches(id, calendar.id()))
}

hc_core::catalogue_tests! {
    type: DatedCalendar,
    id: |calendar| calendar.id(),
    tests: dated_calendar_catalogue_tests,
    all: &ALL,
    lookup: by_id,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_circad_calendar_and_martiana_is_listed_once() {
        for calendar in crate::circad::all() {
            assert_eq!(
                by_id(calendar.rule().id),
                Some(DatedCalendar::Circad(calendar))
            );
        }
        assert_eq!(by_id("martiana"), Some(DatedCalendar::Martiana));
        assert_eq!(ALL.len(), crate::circad::all().len() + 1);
    }
}
