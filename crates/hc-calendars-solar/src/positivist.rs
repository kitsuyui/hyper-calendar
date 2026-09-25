//! The Positivist calendar.
//!
//! Auguste Comte's calendar of 1849, the *Calendrier positiviste*: the year
//! divided into thirteen equal months of four weeks, each month named for
//! the type of a stage of human history, from Moïse to Bichat, and each
//! beginning on a Monday and ending on a Sunday; after the thirteenth month
//! one complementary day, the *Fête universelle des Morts*, and in a leap
//! year a second, the *Fête générale des Saintes Femmes*, to neither of
//! which Comte gives a weekday name. The year "begins and ends like the
//! Christian year" on the Gregorian leap rule, and is numbered from the
//! Revolution: year 1 is 1789, so the Positivist year is the Gregorian one
//! less 1788, and the fourth edition of the calendar, of May 1852, dates
//! itself the sixty-fourth year of the great revolution.
//!
//! This module numbers the two complementary days as the 29th and 30th of
//! Bichat, the only way they fit a year-month-day shape; they follow the
//! month rather than belong to it, and [`PositivistDate::festival`] names
//! them. The month names are declared with the shape as [`MONTHS`], in the
//! spelling the 1852 edition prints — *Guttemberg*, not Gutenberg — since
//! the calendar was defined in French and every other language borrows the
//! names.
//!
//! # The week
//!
//! Every month begins on a Monday, and the complementary days belong to no
//! week: that is what makes the calendar perpetual, and it is also where it
//! parts from the seven-day week that no reform has ever interrupted —
//! 1 January 1789 was a Thursday. So there are **two weekdays** for the
//! same day, both correct: `hc_calendar::Weekday::from_rd` gives the
//! unbroken cycle, and [`PositivistDate::weekday`] gives the calendar's own
//! naming, `None` on the complementary days. `to_fields` flags those days
//! `outside-the-week`; the week cycle is declared as the seven positions it
//! has, as [`crate::world_calendar`] does for the same reason.
//!
//! # Sources
//!
//! Comte, *Calendrier positiviste, ou Système général de commémoration
//! publique*, fourth edition (Paris: Librairie scientifique-industrielle de
//! Mme Ve Mathias, May 1852), the archive.org copy `calendrierposit00comtgoog`:
//!
//! * p. 4: the year "commence et finit comme l'année chrétienne" and differs
//!   only "par sa division en treize mois égaux"; after the last month "un
//!   jour complémentaire ou deux, selon que l'année est commune ou
//!   bissextile, d'après la règle julio-grégorienne"; "Je n'attribue aucun
//!   nom hebdomadaire à ce jour exceptionnel"; "Chaque mois y commencera
//!   toujours par un Lundi et finira par un Dimanche".
//! * p. 8: "Jour complémentaire — Fête universelle des MORTS" and "Jour
//!   additionnel des années bissextiles — Fête générale des SAINTES
//!   FEMMES".
//! * p. 11, note 1: the era needs no change in the start of the year, and
//!   reduces to "retrancher le nombre constant 1788 aux millésimes encore
//!   usités".
//! * pp. 19–20, the table of the *culte concret*: the thirteen monthly
//!   types in order, MOÏSE, HOMÈRE, ARISTOTE, ARCHIMÈDE, CÉSAR, SAINT-PAUL,
//!   CHARLEMAGNE, DANTE, GUTTEMBERG, SHAKESPEARE, DESCARTES, FRÉDÉRIC,
//!   BICHAT.
//!
//! # Exactness
//!
//! Exact — arithmetic, proleptic and unbounded like the Gregorian calendar
//! it is laid over: the whole Gregorian range converts both ways, with the
//! year running through 0 (1788) into negative numbers before the
//! Revolution.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Weekday,
    YearKind,
};

use crate::gregorian;

/// The calendar identifier.
pub const ID: &str = "positivist";

/// What to subtract from the Gregorian year: year 1 is 1789.
pub const YEAR_OFFSET: i64 = 1_788;

/// The thirteen months, in the spelling of the 1852 edition.
pub const MONTHS: [&str; 13] = [
    "Moïse",
    "Homère",
    "Aristote",
    "Archimède",
    "César",
    "Saint-Paul",
    "Charlemagne",
    "Dante",
    "Guttemberg",
    "Shakespeare",
    "Descartes",
    "Frédéric",
    "Bichat",
];

/// The length of every month before the complementary days are added.
pub const DAYS_IN_MONTH: u8 = 28;

/// The month the complementary days follow, Bichat.
pub const LAST_MONTH: u8 = 13;

/// The day number given to the complementary day of every year.
pub const FESTIVAL_OF_THE_DEAD_DAY: u8 = 29;

/// The day number given to the additional day of a leap year.
pub const FESTIVAL_OF_HOLY_WOMEN_DAY: u8 = 30;

/// The complementary day of every year, as the 1852 edition names it.
pub const FESTIVAL_OF_THE_DEAD: &str = "Fête universelle des Morts";

/// The additional day of a leap year, as the 1852 edition names it.
pub const FESTIVAL_OF_HOLY_WOMEN: &str = "Fête générale des Saintes Femmes";

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = gregorian::MIN_YEAR - YEAR_OFFSET;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = gregorian::MAX_YEAR - YEAR_OFFSET;

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = gregorian::EARLIEST;

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = gregorian::LATEST;

/// The fixed day of 1 Moïse 1, which is 1 January 1789.
pub const EPOCH: Rd = match gregorian::to_fixed(1789, 1, 1) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// Whether `year` has the additional day, which is exactly when the
/// Gregorian year it names has a 29 February.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    gregorian::is_leap_year(year + YEAR_OFFSET)
}

/// The number of days in `month` of `year`, or `None` when `month` is not in
/// `1..=13`. Bichat is 29 days because the complementary day is counted in
/// it, and 30 in a leap year.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    match month {
        1..=12 => Some(DAYS_IN_MONTH),
        13 => Some(if is_leap_year(year) {
            FESTIVAL_OF_HOLY_WOMEN_DAY
        } else {
            FESTIVAL_OF_THE_DEAD_DAY
        }),
        _ => None,
    }
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// The fixed day of a Positivist date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`] —
/// the last also for the additional day in a year that has none.
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    if let Err(error) = crate::common::check_day(day, days_in_month(year, month)) {
        return Err(error);
    }
    match gregorian::new_year(year + YEAR_OFFSET) {
        Err(error) => Err(error),
        Ok(start) => Ok(Rd(start.0
            + DAYS_IN_MONTH as i64 * (month as i64 - 1)
            + day as i64
            - 1)),
    }
}

/// The Positivist year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EARLIEST`]..=[`LATEST`].
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    let gregorian_year = match gregorian::year_from_fixed(rd) {
        Err(error) => return Err(error),
        Ok(year) => year,
    };
    let start = match gregorian::new_year(gregorian_year) {
        Err(error) => return Err(error),
        Ok(start) => start,
    };
    let year = gregorian_year - YEAR_OFFSET;
    // The day's place in the year, counting from 0. Both complementary
    // days come after the thirteen months, so nothing shifts mid-year.
    let elapsed = rd.0 - start.0;
    let months = DAYS_IN_MONTH as i64 * LAST_MONTH as i64;
    if elapsed >= months {
        return Ok((
            year,
            LAST_MONTH,
            (elapsed - months + 1 + DAYS_IN_MONTH as i64) as u8,
        ));
    }
    let month = (elapsed / DAYS_IN_MONTH as i64 + 1) as u8;
    let day = (elapsed % DAYS_IN_MONTH as i64 + 1) as u8;
    Ok((year, month, day))
}

/// A Positivist date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PositivistDate {
    /// The year of the Revolution, 1 for 1789; 0 and below before it.
    pub year: i64,
    /// The month, 1 for Moïse through 13 for Bichat.
    pub month: u8,
    /// The day of the month, 1 through 28. Day 29 of month 13 is the
    /// *Fête universelle des Morts* and day 30 the *Fête générale des
    /// Saintes Femmes*; neither belongs to a week.
    pub day: u8,
}

impl PositivistDate {
    /// A validated date.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    pub const fn new(year: i64, month: u8, day: u8) -> CalendarResult<Self> {
        match to_fixed(year, month, day) {
            Err(error) => Err(error),
            Ok(_) => Ok(Self { year, month, day }),
        }
    }

    /// The Gregorian year this date falls in.
    #[must_use]
    pub const fn gregorian_year(self) -> i64 {
        self.year + YEAR_OFFSET
    }

    /// The name of the month.
    #[must_use]
    pub const fn month_name(self) -> &'static str {
        MONTHS[self.month as usize - 1]
    }

    /// Whether this date is one of the complementary days.
    #[must_use]
    pub const fn is_complementary_day(self) -> bool {
        self.month == LAST_MONTH && self.day > DAYS_IN_MONTH
    }

    /// Whether this date belongs to the seven-day week at all.
    #[must_use]
    pub const fn is_in_the_week(self) -> bool {
        !self.is_complementary_day()
    }

    /// The festival this date is, if it is a complementary day.
    #[must_use]
    pub const fn festival(self) -> Option<&'static str> {
        if self.month != LAST_MONTH {
            None
        } else if self.day == FESTIVAL_OF_THE_DEAD_DAY {
            Some(FESTIVAL_OF_THE_DEAD)
        } else if self.day == FESTIVAL_OF_HOLY_WOMEN_DAY {
            Some(FESTIVAL_OF_HOLY_WOMEN)
        } else {
            None
        }
    }

    /// The calendar's own weekday, or `None` for a complementary day.
    ///
    /// Every month opens on a Monday, so this depends only on the day of
    /// the month — never on the year or the month. It is deliberately
    /// **not** the same function as [`Weekday::from_rd`]: see the module
    /// documentation for why the two disagree.
    #[must_use]
    pub const fn weekday(self) -> Option<Weekday> {
        if !self.is_in_the_week() {
            return None;
        }
        Some(match (self.day - 1) % 7 {
            0 => Weekday::Monday,
            1 => Weekday::Tuesday,
            2 => Weekday::Wednesday,
            3 => Weekday::Thursday,
            4 => Weekday::Friday,
            5 => Weekday::Saturday,
            _ => Weekday::Sunday,
        })
    }
}

/// The Positivist calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PositivistCalendar;

/// Thirteen named months and the seven-day week.
///
/// The complementary days sit outside the week, and `to_fields` flags them
/// `outside-the-week`, but the week's positions are still the seven.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for PositivistCalendar {
    type Date = PositivistDate;

    /// Unrecorded: Comte's proposal of 1849, kept by Positivist societies in
    /// ways no source read dates.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// A year with the *Fête générale des Saintes Femmes*, which is when
    /// the Gregorian year it names has a 29 February.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(is_leap_year(year))
    }

    /// Thirteen months named for their types, and the seven-day week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "Positivist",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(PositivistDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.month, date.day)
            .with_extra("outside-the-week", i64::from(!date.is_in_the_week()))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        PositivistDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gregorian(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn year_one_begins_on_1_january_1789() {
        assert_eq!(EPOCH, gregorian(1789, 1, 1));
        assert_eq!(to_fixed(1, 1, 1), Ok(EPOCH));
        assert_eq!(from_fixed(EPOCH), Ok((1, 1, 1)));
        // The fourth edition, of May 1852, calls itself the sixty-fourth
        // year of the great revolution.
        let edition = PositivistCalendar
            .from_fixed(gregorian(1852, 5, 1))
            .unwrap();
        assert_eq!(edition.year, 64);
        assert_eq!(edition.gregorian_year(), 1852);
        assert_eq!(
            PositivistCalendar
                .from_fixed(gregorian(2026, 1, 1))
                .unwrap()
                .year,
            238
        );
        // 1 January 1789 was a Thursday in the week that never broke, and
        // a Monday in this calendar's naming.
        assert_eq!(Weekday::from_rd(EPOCH), Weekday::Thursday);
        assert_eq!(
            PositivistDate::new(1, 1, 1).unwrap().weekday(),
            Some(Weekday::Monday)
        );
    }

    #[test]
    fn the_months_are_the_thirteen_types_of_the_1852_edition() {
        assert_eq!(MONTHS[0], "Moïse");
        assert_eq!(MONTHS[5], "Saint-Paul");
        assert_eq!(MONTHS[8], "Guttemberg");
        assert_eq!(MONTHS[12], "Bichat");
        assert_eq!(
            PositivistDate::new(64, 13, 1).unwrap().month_name(),
            "Bichat"
        );
        assert_eq!(PositivistCalendar.cycles()[0].names.len(), 13);
        // Every month is four weeks: Moïse 1 is a Monday, Moïse 28 a
        // Sunday, and Homère 1 a Monday again.
        for month in 1..=13u8 {
            let first = PositivistDate::new(64, month, 1).unwrap();
            assert_eq!(first.weekday(), Some(Weekday::Monday), "{month}");
            let last = PositivistDate::new(64, month, 28).unwrap();
            assert_eq!(last.weekday(), Some(Weekday::Sunday), "{month}");
        }
        assert_eq!(
            to_fixed(64, 2, 1).unwrap().0,
            to_fixed(64, 1, 28).unwrap().0 + 1
        );
    }

    #[test]
    fn the_complementary_days_follow_bichat_and_have_no_weekday() {
        // In a common year the Fête des Morts is 31 December; in a leap
        // year it is 30 December and the Fête des Saintes Femmes follows.
        assert_eq!(to_fixed(237, 13, 28), Ok(gregorian(2025, 12, 30)));
        assert_eq!(to_fixed(237, 13, 29), Ok(gregorian(2025, 12, 31)));
        assert_eq!(to_fixed(237, 13, 30), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(236, 13, 28), Ok(gregorian(2024, 12, 29)));
        assert_eq!(to_fixed(236, 13, 29), Ok(gregorian(2024, 12, 30)));
        assert_eq!(to_fixed(236, 13, 30), Ok(gregorian(2024, 12, 31)));
        assert_eq!(days_in_month(236, 13), Some(30));
        assert_eq!(days_in_month(237, 13), Some(29));
        assert_eq!(days_in_month(237, 12), Some(28));
        assert_eq!(days_in_month(237, 14), None);
        assert_eq!(days_in_year(236), 366);
        assert_eq!(days_in_year(237), 365);
        assert!(is_leap_year(236));
        assert!(!is_leap_year(112)); // 1900

        let dead = PositivistDate::new(236, 13, 29).unwrap();
        assert!(dead.is_complementary_day());
        assert!(!dead.is_in_the_week());
        assert_eq!(dead.weekday(), None);
        assert_eq!(dead.festival(), Some(FESTIVAL_OF_THE_DEAD));
        let women = PositivistDate::new(236, 13, 30).unwrap();
        assert_eq!(women.weekday(), None);
        assert_eq!(women.festival(), Some(FESTIVAL_OF_HOLY_WOMEN));
        assert_eq!(PositivistDate::new(236, 13, 28).unwrap().festival(), None);
        assert!(PositivistDate::new(236, 1, 29).is_err());
    }

    #[test]
    fn the_calendar_is_proleptic_before_the_revolution() {
        assert_eq!(from_fixed(gregorian(1788, 1, 1)), Ok((0, 1, 1)));
        assert_eq!(from_fixed(gregorian(1787, 1, 1)), Ok((-1, 1, 1)));
        assert_eq!(to_fixed(-1787, 1, 1), Ok(Rd(1)));
        assert_eq!(from_fixed(Rd(1)), Ok((-1787, 1, 1)));
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (-500_000..=1_500_000).step_by(67) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
        for rd in [EARLIEST.0, EARLIEST.0 + 1, LATEST.0 - 1, LATEST.0] {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn every_single_day_of_a_leap_cycle_round_trips() {
        let start = gregorian(1896, 1, 1).0;
        let end = gregorian(1912, 1, 1).0;
        for rd in start..end {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
            assert!(month <= 13 && day <= 30);
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = PositivistCalendar;
        for rd in (-100_000..=900_000).step_by(257) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(
                fields.extra.get("outside-the-week"),
                Some(i64::from(!date.is_in_the_week()))
            );
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId(ID));
    }

    #[test]
    fn dates_outside_the_supported_range_are_refused() {
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(to_fixed(238, 1, 29), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(238, 14, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(to_fixed(238, 0, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(
            to_fixed(MAX_YEAR + 1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            to_fixed(MIN_YEAR - 1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            PositivistCalendar.from_fields(&DateFields::ymd_leap_month(238, 1, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
    }
}
