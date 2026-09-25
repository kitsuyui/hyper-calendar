//! The Julian calendar up to a country's reform, Gregorian afterwards.
//!
//! Neither [`crate::julian`] nor [`crate::gregorian`] tells you what was
//! written on a page on a given day, because the answer depends on where the
//! page was written. Great Britain went to bed on Wednesday 2 September 1752
//! and woke up on Thursday 14 September 1752; Russia was still on the Julian
//! calendar until 31 January 1918. A "reform calendar" is therefore
//! parameterised by one number — the fixed day on which the Gregorian
//! reckoning took effect — and [`ADOPTIONS`] carries that number for twelve
//! well-known polities.
//!
//! Three things follow from the cut-over, and all three are modelled here:
//!
//! * dates before it are Julian and dates from it on are Gregorian;
//! * the dates in between were skipped and never existed, so naming one is an
//!   error rather than a date;
//! * the weekday cycle is unbroken across the gap, which is why the day after
//!   Wednesday the 2nd was Thursday the 14th.
//!
//! # Sources
//!
//! The cut-over dates are the ones given in the national legislation or
//! gazette of each country as summarised by Reingold and Dershowitz,
//! *Calendrical Calculations* (4th ed.), appendix, and by the Explanatory
//! Supplement to the Astronomical Almanac (3rd ed., 2013), §15.3. Where a
//! state adopted the reform province by province — the Dutch Republic, the
//! German states, Switzerland — the entry names the province the date
//! belongs to rather than pretending the state moved at once.
//!
//! # The start of the year is a separate axis
//!
//! This module always uses 1 January. England ran the historical year from
//! 25 March (Lady Day) until 1752, so a document dated "12 February 1721"
//! usually means what this crate calls 1722-02-12 — and recovering that
//! needs the scribe's convention, not just the country.
//!
//! [`crate::year_style`] carries the conventions as named styles, which
//! is policy §5's answer to exactly this: a finite set the caller selects,
//! rather than one silent default. The reform date and the year start are
//! independent, so they stay two separate things to choose.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::{gregorian, julian};

/// The era code for a date still written in the Julian calendar.
pub const ERA_OLD_STYLE: &str = "OS";

/// The era code for a date written in the Gregorian calendar.
pub const ERA_NEW_STYLE: &str = "NS";

/// One polity's adoption of the Gregorian calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Adoption {
    /// The calendar identifier for this variant, for example
    /// `"julian-gregorian-gb"`.
    pub id: &'static str,
    /// The English name of the polity that adopted on this date.
    pub region: &'static str,
    /// The last date written in the Julian calendar, as year, month, day.
    pub last_julian: (i64, u8, u8),
    /// The first date written in the Gregorian calendar, as year, month, day.
    pub first_gregorian: (i64, u8, u8),
}

impl Adoption {
    /// The fixed day of the first Gregorian date.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the table entry is malformed, which
    /// a unit test in this module rules out for the shipped table.
    pub const fn cutover(&self) -> CalendarResult<Rd> {
        gregorian::to_fixed(
            self.first_gregorian.0,
            self.first_gregorian.1,
            self.first_gregorian.2,
        )
    }

    /// How many dates the reform skipped in this polity.
    ///
    /// This is the accumulated Julian drift at the moment of the reform: read
    /// the first Gregorian label as though it were a Julian one and the gap
    /// between it and the last real Julian date is what was dropped.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the table entry is malformed.
    pub const fn skipped_days(&self) -> CalendarResult<i64> {
        let last =
            match julian::to_fixed(self.last_julian.0, self.last_julian.1, self.last_julian.2) {
                Err(error) => return Err(error),
                Ok(rd) => rd,
            };
        match julian::to_fixed(
            self.first_gregorian.0,
            self.first_gregorian.1,
            self.first_gregorian.2,
        ) {
            Err(error) => Err(error),
            Ok(as_julian) => Ok(as_julian.0 - last.0 - 1),
        }
    }
}

/// Twelve well-known national adoptions of the Gregorian calendar.
///
/// The list is deliberately small and explicitly incomplete: over fifty
/// polities reformed at over twenty different moments, and several did so
/// twice. These are the ones a reader of European or Russian sources meets
/// most often, in chronological order.
pub const ADOPTIONS: [Adoption; 12] = [
    Adoption {
        id: "julian-gregorian-catholic",
        region: "Papal States, Spain, Portugal, Poland-Lithuania",
        last_julian: (1582, 10, 4),
        first_gregorian: (1582, 10, 15),
    },
    Adoption {
        id: "julian-gregorian-fr",
        region: "France",
        last_julian: (1582, 12, 9),
        first_gregorian: (1582, 12, 20),
    },
    Adoption {
        id: "julian-gregorian-nl",
        region: "Holland and Zeeland",
        last_julian: (1582, 12, 14),
        first_gregorian: (1582, 12, 25),
    },
    Adoption {
        id: "julian-gregorian-de-catholic",
        region: "Catholic Germany (Bavaria)",
        last_julian: (1583, 10, 5),
        first_gregorian: (1583, 10, 16),
    },
    Adoption {
        id: "julian-gregorian-hu",
        region: "Hungary",
        last_julian: (1587, 10, 21),
        first_gregorian: (1587, 11, 1),
    },
    Adoption {
        id: "julian-gregorian-de-protestant",
        region: "Protestant Germany, Denmark and Norway",
        last_julian: (1700, 2, 18),
        first_gregorian: (1700, 3, 1),
    },
    Adoption {
        id: "julian-gregorian-gb",
        region: "Great Britain and its colonies",
        last_julian: (1752, 9, 2),
        first_gregorian: (1752, 9, 14),
    },
    Adoption {
        id: "julian-gregorian-se",
        region: "Sweden and Finland",
        last_julian: (1753, 2, 17),
        first_gregorian: (1753, 3, 1),
    },
    Adoption {
        id: "julian-gregorian-bg",
        region: "Bulgaria",
        last_julian: (1916, 3, 31),
        first_gregorian: (1916, 4, 14),
    },
    Adoption {
        id: "julian-gregorian-ru",
        region: "Soviet Russia",
        last_julian: (1918, 1, 31),
        first_gregorian: (1918, 2, 14),
    },
    Adoption {
        id: "julian-gregorian-ro",
        region: "Romania and Serbia",
        last_julian: (1919, 3, 31),
        first_gregorian: (1919, 4, 14),
    },
    Adoption {
        id: "julian-gregorian-gr",
        region: "Greece",
        last_julian: (1923, 2, 15),
        first_gregorian: (1923, 3, 1),
    },
];

/// The adoption whose identifier is `id`.
#[must_use]
pub fn adoption_by_id(id: &str) -> Option<Adoption> {
    ADOPTIONS.iter().copied().find(|entry| entry.id == id)
}

/// A date as it would have been written in one polity.
///
/// Whether the label is Julian or Gregorian is not stored: it follows from
/// the date and the calendar's cut-over, and storing it would let a caller
/// build a date that contradicts itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReformDate {
    /// The astronomical year: 1 BC is year 0.
    pub year: i64,
    /// The month, 1 for January through 12 for December.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl ReformDate {
    /// A date, unvalidated: validity depends on which reform calendar it is
    /// read against.
    #[must_use]
    pub const fn new(year: i64, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }
}

/// The Julian calendar before a given cut-over and the Gregorian calendar
/// from it on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReformCalendar {
    id: &'static str,
    region: &'static str,
    cutover: Rd,
}

impl ReformCalendar {
    /// The calendar of a tabulated adoption.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the table entry is malformed.
    pub const fn new(adoption: Adoption) -> CalendarResult<Self> {
        match adoption.cutover() {
            Err(error) => Err(error),
            Ok(cutover) => Ok(Self {
                id: adoption.id,
                region: adoption.region,
                cutover,
            }),
        }
    }

    /// A calendar with an explicit cut-over, for a polity not in the table.
    ///
    /// `cutover` is the fixed day of the *first* Gregorian date.
    #[must_use]
    pub const fn with_cutover(id: &'static str, region: &'static str, cutover: Rd) -> Self {
        Self {
            id,
            region,
            cutover,
        }
    }

    /// The fixed day on which this polity started writing Gregorian dates.
    #[must_use]
    pub const fn cutover(&self) -> Rd {
        self.cutover
    }

    /// The polity this calendar describes.
    #[must_use]
    pub const fn region(&self) -> &'static str {
        self.region
    }

    /// Whether a fixed day falls in the Gregorian part of this calendar.
    #[must_use]
    pub const fn is_new_style(&self, rd: Rd) -> bool {
        rd.0 >= self.cutover.0
    }
}

impl Default for ReformCalendar {
    /// The Catholic adoption of October 1582, the reform itself.
    fn default() -> Self {
        Self {
            id: ADOPTIONS[0].id,
            region: ADOPTIONS[0].region,
            // `to_fixed` of 1582-10-15 cannot fail; the fallback keeps the
            // no-panic rule without pretending the failure is meaningful.
            cutover: match ADOPTIONS[0].cutover() {
                Ok(rd) => rd,
                Err(_) => Rd(577_736),
            },
        }
    }
}

impl Calendar for ReformCalendar {
    type Date = ReformDate;

    /// Twelve months and the seven-day week on both sides of the reform.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    /// A year in which 29 February was written, under whichever calendar
    /// was in force that day. Julian 1700 has one in Russia and not in
    /// Britain; a polity that reformed in February has none that year at
    /// all.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        match self.to_fixed(ReformDate::new(year, 2, 29)) {
            Ok(_) => Ok(true),
            Err(CalendarError::DayOutOfRange) => Ok(false),
            Err(error) => Err(error),
        }
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(self.id),
            english_name: "Julian/Gregorian reform",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(julian::EARLIEST),
            latest: Some(gregorian::LATEST),
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        // A label belongs to whichever calendar was in force on the day it
        // names. Labels that fall in the gap belong to neither.
        if let Ok(rd) = julian::to_fixed(date.year, date.month, date.day)
            && rd < self.cutover
        {
            return Ok(rd);
        }
        let rd = gregorian::to_fixed(date.year, date.month, date.day)?;
        if rd >= self.cutover {
            Ok(rd)
        } else {
            Err(CalendarError::DayOutOfRange)
        }
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = if self.is_new_style(rd) {
            gregorian::from_fixed(rd)?
        } else {
            julian::from_fixed(rd)?
        };
        Ok(ReformDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let rd = self.to_fixed(date)?;
        let era = if self.is_new_style(rd) {
            ERA_NEW_STYLE
        } else {
            ERA_OLD_STYLE
        };
        Ok(DateFields::ymd(date.year, date.month, date.day).with_era(era))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if let Some(era) = fields.era
            && era != ERA_OLD_STYLE
            && era != ERA_NEW_STYLE
        {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        Ok(ReformDate::new(
            fields.year,
            month.ordinal,
            fields.require_day()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::Weekday;

    fn calendar(id: &str) -> ReformCalendar {
        ReformCalendar::new(adoption_by_id(id).unwrap()).unwrap()
    }

    #[test]
    fn every_tabulated_adoption_is_internally_consistent() {
        // The day after the last Julian date must be the first Gregorian
        // date: an entry where the two do not meet is a typo, not a country.
        for adoption in ADOPTIONS {
            let last = julian::to_fixed(
                adoption.last_julian.0,
                adoption.last_julian.1,
                adoption.last_julian.2,
            )
            .unwrap();
            let first = adoption.cutover().unwrap();
            assert_eq!(first.0, last.0 + 1, "{}", adoption.region);
        }
    }

    #[test]
    fn the_reform_skipped_ten_days_and_later_adopters_skipped_more() {
        assert_eq!(
            adoption_by_id("julian-gregorian-catholic")
                .unwrap()
                .skipped_days(),
            Ok(10)
        );
        assert_eq!(
            adoption_by_id("julian-gregorian-gb")
                .unwrap()
                .skipped_days(),
            Ok(11)
        );
        assert_eq!(
            adoption_by_id("julian-gregorian-ru")
                .unwrap()
                .skipped_days(),
            Ok(13)
        );
        assert_eq!(
            adoption_by_id("julian-gregorian-gr")
                .unwrap()
                .skipped_days(),
            Ok(13)
        );
    }

    #[test]
    fn the_weekday_cycle_is_unbroken_across_every_gap() {
        // Britain: Wednesday 2 September 1752 was followed by Thursday 14
        // September 1752.
        let britain = calendar("julian-gregorian-gb");
        let last = britain.to_fixed(ReformDate::new(1752, 9, 2)).unwrap();
        let first = britain.to_fixed(ReformDate::new(1752, 9, 14)).unwrap();
        assert_eq!(first.0, last.0 + 1);
        assert_eq!(Weekday::from_rd(last), Weekday::Wednesday);
        assert_eq!(Weekday::from_rd(first), Weekday::Thursday);
    }

    #[test]
    fn dates_in_the_gap_never_existed() {
        let britain = calendar("julian-gregorian-gb");
        for day in 3..=13u8 {
            assert_eq!(
                britain.to_fixed(ReformDate::new(1752, 9, day)),
                Err(CalendarError::DayOutOfRange),
                "1752-09-{day}"
            );
        }
        assert!(britain.to_fixed(ReformDate::new(1752, 9, 2)).is_ok());
        assert!(britain.to_fixed(ReformDate::new(1752, 9, 14)).is_ok());
    }

    #[test]
    fn the_same_label_means_different_days_in_different_countries() {
        // 1918-02-01 was still January in Russia but ordinary February in
        // Britain, which had reformed 166 years earlier.
        let russia = calendar("julian-gregorian-ru");
        let britain = calendar("julian-gregorian-gb");
        let label = ReformDate::new(1917, 11, 7);
        // The October Revolution: 25 October 1917 Old Style in Russia is
        // 7 November 1917 in the Gregorian calendar Britain was using.
        let russian_label = russia.from_fixed(britain.to_fixed(label).unwrap()).unwrap();
        assert_eq!(russian_label, ReformDate::new(1917, 10, 25));
    }

    #[test]
    fn each_side_of_the_cutover_uses_the_right_calendar() {
        let catholic = calendar("julian-gregorian-catholic");
        let cutover = catholic.cutover();
        assert_eq!(
            catholic.from_fixed(Rd(cutover.0 - 1)),
            Ok(ReformDate::new(1582, 10, 4))
        );
        assert_eq!(
            catholic.from_fixed(cutover),
            Ok(ReformDate::new(1582, 10, 15))
        );
        assert!(!catholic.is_new_style(Rd(cutover.0 - 1)));
        assert!(catholic.is_new_style(cutover));
    }

    #[test]
    fn every_day_around_every_cutover_round_trips() {
        for adoption in ADOPTIONS {
            let reform = ReformCalendar::new(adoption).unwrap();
            let cutover = reform.cutover().0;
            for rd in (cutover - 1_200)..(cutover + 1_200) {
                let date = reform.from_fixed(Rd(rd)).unwrap();
                assert_eq!(reform.to_fixed(date), Ok(Rd(rd)), "{} rd {rd}", adoption.id);
            }
        }
    }

    #[test]
    fn a_wide_range_round_trips_for_the_british_calendar() {
        let britain = calendar("julian-gregorian-gb");
        for rd in (-200_000..=900_000).step_by(59) {
            let date = britain.from_fixed(Rd(rd)).unwrap();
            assert_eq!(britain.to_fixed(date), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn the_reform_year_is_short_by_the_days_it_skipped() {
        use hc_calendar::{DynAdapter, DynCalendar};

        let catholic = DynAdapter::new(calendar("julian-gregorian-catholic"));
        assert_eq!(catholic.days_in_year(1582), Ok(355));
        assert_eq!(catholic.days_in_year(1581), Ok(365));
        // October 1582 had 21 days in Catholic Europe.
        assert_eq!(
            catholic.days_in_month(&DateFields::ymd(1582, 10, 1)),
            Ok(21)
        );

        let britain = DynAdapter::new(calendar("julian-gregorian-gb"));
        assert_eq!(britain.days_in_year(1752), Ok(355));
        assert_eq!(britain.days_in_month(&DateFields::ymd(1752, 9, 1)), Ok(19));
    }

    #[test]
    fn fields_carry_old_and_new_style_markers() {
        let britain = calendar("julian-gregorian-gb");
        let old = britain.to_fields(ReformDate::new(1700, 1, 1)).unwrap();
        assert_eq!(old.era, Some(ERA_OLD_STYLE));
        let new = britain.to_fields(ReformDate::new(1800, 1, 1)).unwrap();
        assert_eq!(new.era, Some(ERA_NEW_STYLE));
        assert_eq!(britain.from_fields(&new), Ok(ReformDate::new(1800, 1, 1)));
        assert_eq!(
            britain.from_fields(&DateFields::ymd(1800, 1, 1).with_era("Showa")),
            Err(CalendarError::UnknownEra)
        );
    }

    #[test]
    fn a_custom_cutover_works_for_polities_not_in_the_table() {
        // Alaska is why the table stops at twelve entries: when it was sold
        // to the United States in 1867 it changed calendar and side of the
        // date line in the same act, so Friday 6 October (Julian) was
        // followed by Friday 18 October (Gregorian) and a weekday repeated.
        // The date-line half of that is a time-zone matter, so a reform
        // calendar can only model the calendar half — which is what a caller
        // gets by naming the cut-over themselves.
        let cutover = gregorian::to_fixed(1867, 10, 18).unwrap();
        let alaska = ReformCalendar::with_cutover("julian-gregorian-ak", "Alaska", cutover);
        assert_eq!(
            alaska.from_fixed(cutover),
            Ok(ReformDate::new(1867, 10, 18))
        );
        assert_eq!(
            alaska.from_fixed(Rd(cutover.0 - 1)),
            Ok(ReformDate::new(1867, 10, 5))
        );
        assert_eq!(alaska.meta().id, CalendarId("julian-gregorian-ak"));
        assert_eq!(alaska.region(), "Alaska");
    }

    #[test]
    fn a_leap_year_is_one_in_which_29_february_was_written() {
        // 1582 has 355 days in the Catholic reform and 1583 is an ordinary
        // 365: neither is leap, whatever a comparison of their lengths says.
        let catholic = ReformCalendar::default();
        assert_eq!(catholic.is_leap_year(1582), Ok(false));
        assert_eq!(catholic.is_leap_year(1583), Ok(false));
        assert_eq!(catholic.is_leap_year(1584), Ok(true));
        // 1700 was Julian, and leap, where the reform had not yet reached;
        // Gregorian, and common, where it had.
        assert_eq!(calendar("julian-gregorian-gb").is_leap_year(1700), Ok(true));
        assert_eq!(catholic.is_leap_year(1700), Ok(false));
        // The Protestant German states went from 18 February straight to
        // 1 March 1700, so their 1700 had no 29 February on either side.
        assert_eq!(
            calendar("julian-gregorian-de-protestant").is_leap_year(1700),
            Ok(false)
        );
        assert_eq!(calendar("julian-gregorian-ru").is_leap_year(1900), Ok(true));
        assert_eq!(calendar("julian-gregorian-ru").is_leap_year(2000), Ok(true));
        assert_eq!(
            calendar("julian-gregorian-ru").is_leap_year(2100),
            Ok(false)
        );
        assert!(catholic.is_leap_year(julian::MIN_YEAR - 1).is_err());
    }

    #[test]
    fn the_default_calendar_is_the_reform_itself() {
        let default = ReformCalendar::default();
        assert_eq!(
            default.cutover(),
            gregorian::to_fixed(1582, 10, 15).unwrap()
        );
        assert_eq!(adoption_by_id("nowhere"), None);
    }
}
