//! Names a government gave the months and weekdays for a period.
//!
//! A locale's month names are the names the language uses. Sometimes a
//! state renamed them by law, and the new names held only while the law
//! did. A [`NamingPeriod`] is that: a language, a calendar, the names, and
//! the days they were in force. It sits beside a locale's ordinary
//! vocabulary and does not replace it: outside the period, the locale's own
//! names apply.
//!
//! Where the sources do not date a change to the day, the period says so.
//! [`NamingPeriod::in_force_on`] answers `None` for the days between the
//! earliest day the change can have taken effect and the first day every
//! source read has it in force.
//!
//! The one period carried is Turkmenistan's, 2002 to 2008,
//! [`TURKMEN_2002`].

use hc_calendar::{CalendarId, Rd, Weekday};

use crate::locale::Locale;

/// Month and weekday names in force for a period.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NamingPeriod {
    /// A short identifier.
    pub id: &'static str,
    /// The language subtag the names belong to.
    pub language: &'static str,
    /// The calendar whose months and weekdays are renamed.
    pub calendar: CalendarId,
    /// The first day on which the names can have been in force: the day of
    /// the decision.
    pub earliest: Rd,
    /// The first day by which every source read has the names in force.
    pub in_force_by: Rd,
    /// The first day on which the old names were back.
    pub ended: Rd,
    /// The month names, the first month first.
    pub months: &'static [&'static str],
    /// The weekday names, Monday first.
    pub weekdays: &'static [&'static str],
    /// The weekday names' meanings in English, Monday first, as the source
    /// glosses them.
    pub weekday_meanings: &'static [&'static str],
    /// Where the names and the dates come from.
    pub source: &'static str,
}

impl NamingPeriod {
    /// Whether the names were in force on `day`: `Some(true)` from
    /// [`Self::in_force_by`] to the day before [`Self::ended`],
    /// `Some(false)` before [`Self::earliest`] and from [`Self::ended`],
    /// and `None` between the two starts, which no source read decides.
    #[must_use]
    pub const fn in_force_on(&self, day: Rd) -> Option<bool> {
        if day.0 < self.earliest.0 || day.0 >= self.ended.0 {
            Some(false)
        } else if day.0 >= self.in_force_by.0 {
            Some(true)
        } else {
            None
        }
    }

    /// The name of `month`, counting from 1.
    #[must_use]
    pub fn month_name(&self, month: u8) -> Option<&'static str> {
        self.months.get(usize::from(month).checked_sub(1)?).copied()
    }

    /// The name of `weekday`.
    #[must_use]
    pub fn weekday_name(&self, weekday: Weekday) -> Option<&'static str> {
        self.weekdays
            .get(usize::from(weekday.monday_first_number()))
            .copied()
    }

    /// Whether the period's names are `locale`'s language's.
    #[must_use]
    pub fn applies_to(&self, locale: &Locale) -> bool {
        locale.language().eq_ignore_ascii_case(self.language)
    }
}

/// The first day of a Gregorian date, for the tables below.
const fn day(year: i64, month: u8, day: u8) -> Rd {
    match hc_calendar::gregorian::to_fixed(year, month, day) {
        Ok(rd) => rd,
        Err(_) => Rd(0),
    }
}

/// The Turkmen month and weekday names of 2002 to 2008.
///
/// The People's Council voted the new names on 8 August 2002 (RFE/RL,
/// "Turkmenistan: National Council Renames Days, Months", 8 August 2002,
/// `rferl-2002-turkmen-months`), and Wikipedia dates the law to 10 August
/// (`wikipedia-turkmen-renaming`). No source read gives the day the law
/// took effect. RIA Novosti says the names "were changed in 2002" and
/// Lenta.ru that they were used "in the following years", so from
/// 1 January 2003 they are in force, and from 8 August to 31 December 2002
/// the answer is `None`. The Majlis resolved on 24 May 2008 to return to
/// the old names "from 1 July", and they returned on 1 July 2008 (RIA
/// Novosti, 1 July 2008, `ria-2008-turkmen-calendar`; Lenta.ru, 1 July
/// 2008, `lenta-2008-turkmen-calendar`). The spellings and the weekday
/// meanings are Wikipedia's.
pub const TURKMEN_2002: NamingPeriod = NamingPeriod {
    id: "turkmen-2002",
    language: "tk",
    calendar: CalendarId("gregory"),
    earliest: day(2002, 8, 8),
    in_force_by: day(2003, 1, 1),
    ended: day(2008, 7, 1),
    months: &[
        "Türkmenbaşy",
        "Baýdak",
        "Nowruz",
        "Gurbansoltan",
        "Magtymguly",
        "Oguz",
        "Gorkut",
        "Alp Arslan",
        "Ruhnama",
        "Garaşsyzlyk",
        "Sanjar",
        "Bitaraplyk",
    ],
    weekdays: &[
        "Başgün",
        "Ýaşgün",
        "Hoşgün",
        "Sogapgün",
        "Annagün",
        "Ruhgün",
        "Dynçgün",
    ],
    weekday_meanings: &[
        "First day",
        "Youth day",
        "Favourable day",
        "Justice day",
        "Mother day",
        "Spirit day",
        "Rest day",
    ],
    source: "Wikipedia, \"Renaming of Turkmen months and days of week\", retrieved 2026-09-27 \
        [wikipedia-turkmen-renaming]: the law of 10 August 2002, the names and the weekday \
        meanings; RFE/RL, 8 August 2002 [rferl-2002-turkmen-months]: the People's Council's vote \
        on the first day of its session; RIA Novosti, \"Реформа календаря в Туркмении. \
        Справка\", 1 July 2008 [ria-2008-turkmen-calendar]: the names changed in 2002, the \
        Majlis's resolution of 24 May 2008 and the return from 1 July 2008; Lenta.ru, \
        \"Туркменам вернули прежний календарь\", 1 July 2008 [lenta-2008-turkmen-calendar]: the \
        old names in use again from 1 July, the new ones used in official documents in the \
        years after 2002",
};

/// Every naming period carried.
pub const ALL: &[NamingPeriod] = &[TURKMEN_2002];

/// Which names a locale writes for a calendar on a day.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeriodOn {
    /// A period's names were in force.
    InForce(&'static NamingPeriod),
    /// A period applies, and no source read says whether it was yet in
    /// force.
    Undecided(&'static NamingPeriod),
    /// No period applies: the locale's own names.
    Ordinary,
}

/// Which names `locale` writes for `calendar` on `day`.
#[must_use]
pub fn period_on(locale: &Locale, calendar: CalendarId, day: Rd) -> PeriodOn {
    for period in ALL {
        if period.calendar != calendar || !period.applies_to(locale) {
            continue;
        }
        match period.in_force_on(day) {
            Some(true) => return PeriodOn::InForce(period),
            Some(false) => {}
            None => return PeriodOn::Undecided(period),
        }
    }
    PeriodOn::Ordinary
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tk() -> Locale {
        "tk".parse().unwrap()
    }

    #[test]
    fn the_names_are_in_force_from_2003_to_the_end_of_june_2008() {
        let period = TURKMEN_2002;
        assert_eq!(period.in_force_on(day(2002, 8, 7)), Some(false));
        assert_eq!(period.in_force_on(day(2002, 8, 8)), None);
        assert_eq!(period.in_force_on(day(2002, 12, 31)), None);
        assert_eq!(period.in_force_on(day(2003, 1, 1)), Some(true));
        assert_eq!(period.in_force_on(day(2008, 6, 30)), Some(true));
        // RIA Novosti and Lenta.ru: the old names from 1 July 2008.
        assert_eq!(period.in_force_on(day(2008, 7, 1)), Some(false));
    }

    #[test]
    fn january_is_turkmenbashy_and_monday_bashgun() {
        let period = TURKMEN_2002;
        // Wikipedia: January "Türkmenbaşy", April "Gurbansoltan",
        // September "Ruhnama"; Lenta.ru names the same three, in Russian.
        assert_eq!(period.month_name(1), Some("Türkmenbaşy"));
        assert_eq!(period.month_name(4), Some("Gurbansoltan"));
        assert_eq!(period.month_name(9), Some("Ruhnama"));
        assert_eq!(period.month_name(12), Some("Bitaraplyk"));
        assert_eq!(period.month_name(0), None);
        assert_eq!(period.month_name(13), None);
        assert_eq!(period.weekday_name(Weekday::Monday), Some("Başgün"));
        assert_eq!(period.weekday_name(Weekday::Saturday), Some("Ruhgün"));
        assert_eq!(period.weekday_name(Weekday::Sunday), Some("Dynçgün"));
        assert_eq!(period.months.len(), 12);
        assert_eq!(period.weekdays.len(), 7);
        assert_eq!(period.weekday_meanings.len(), 7);
    }

    #[test]
    fn a_turkmen_date_of_2005_takes_the_period_and_other_days_do_not() {
        let gregory = CalendarId("gregory");
        let in_2005 = day(2005, 3, 21);
        assert_eq!(
            period_on(&tk(), gregory, in_2005),
            PeriodOn::InForce(&TURKMEN_2002)
        );
        assert_eq!(
            period_on(&"tk-TM".parse().unwrap(), gregory, in_2005),
            PeriodOn::InForce(&TURKMEN_2002)
        );
        assert_eq!(
            period_on(&tk(), gregory, day(2010, 1, 1)),
            PeriodOn::Ordinary
        );
        assert_eq!(
            period_on(&tk(), gregory, day(2001, 1, 1)),
            PeriodOn::Ordinary
        );
        assert_eq!(
            period_on(&tk(), gregory, day(2002, 10, 1)),
            PeriodOn::Undecided(&TURKMEN_2002)
        );
        // Another language, or another calendar, is not renamed.
        assert_eq!(
            period_on(&"ru".parse().unwrap(), gregory, in_2005),
            PeriodOn::Ordinary
        );
        assert_eq!(
            period_on(&tk(), CalendarId("julian"), in_2005),
            PeriodOn::Ordinary
        );
    }
}
