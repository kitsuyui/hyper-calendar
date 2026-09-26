//! The Hermetic Leap Week Calendar.
//!
//! Peter Meyer's proposal of 8 January 2007: a year of 52 weeks, or 53 in
//! a leap year, beginning on a Monday in late December of the Gregorian
//! year before, cut into twelve months of five, four and four weeks, the
//! twelfth taking the leap week. Meyer defined the leap years by *hexades*
//! of five or six years whose third year is the leap year, a hexade
//! beginning in year Y being short when `71 · Y mod 100 < 26`; Karl Palmen
//! showed the same years are those with `(71 · Y + 203) mod 400 < 71`,
//! which is the rule computed here: 71 leap weeks in 400 years, a mean year
//! of exactly the Gregorian 365.2425 days. Year 1 begins on Monday
//! 25 December 1 BC, Julian Day Number 1 721 419, the first Monday after
//! the solstice of year 0, and years are numbered astronomically.
//!
//! This is Meyer's year-month-day form, "LPM", with the months named after
//! stars, Arcturus to Lesath. His year-week-day form, "LPW", names the
//! same days by the week of the year and the weekday, and the fields carry
//! both as `week` and `day-of-week`. The arithmetic is `leap_week`'s,
//! which the Symmetry calendars share. The calendar is written up with the
//! other Hermetic Systems reforms in `docs/systems/hermetic-reforms.md`.
//!
//! # Sources
//!
//! * Peter Meyer, "The Hermetic Leap Week Calendar", Hermetic Systems,
//!   <https://www.hermetic.ch/cal_stud/hlpwk/hlpwk.htm>, retrieved
//!   2026-09-26 (`meyer-hermetic-leap-week`): the definition by hexades,
//!   the first day of year 1, Palmen's direct rule, the month layout and
//!   names, the new years of 2007 to 2012, and the dated examples.
//! * Karl Palmen, "Properties of the Hermetic Leap Week Calendar", Hermetic
//!   Systems, <https://www.hermetic.ch/cal_stud/hlpwk/hlpwk_prop.htm>,
//!   retrieved 2026-09-26 (`palmen-hermetic-leap-week`): the proof that the
//!   two rules agree, and the list of the leap years of a 400-year cycle.
//!
//! # Exactness
//!
//! Exact: the rule is the definition. The day runs from local midnight to
//! midnight, as Meyer defines it.

use hc_calendar::shape::{CycleShape, MONTH, WEEKDAY};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Weekday,
    YearKind,
};

use crate::leap_week::LeapWeekRule;

/// The calendar identifier.
pub const ID: &str = "hermetic-leap-week";

/// The month names, stars, as Meyer gives them.
pub const MONTHS: [&str; 12] = [
    "Arcturus",
    "Bellatrix",
    "Canopus",
    "Deneb",
    "Elnath",
    "Fomalhaut",
    "Girtab",
    "Hadar",
    "Izar",
    "Jabbah",
    "Kochab",
    "Lesath",
];

/// Month lengths in an ordinary year: five, four and four weeks a quarter.
/// Lesath takes the leap week.
const MONTH_DAYS: [u8; 12] = [35, 28, 28, 35, 28, 28, 35, 28, 28, 35, 28, 28];

/// The first day of year 1: Julian Day Number 1 721 419, Monday
/// 25 December 1 BC (proleptic Gregorian 0-12-25).
pub const EPOCH: Rd = Rd::from_julian_day_number(1_721_419);

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = -99_999;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 99_999;

/// Palmen's direct form of Meyer's rule, `(71 · Y + 203) mod 400 < 71`.
const RULE: LeapWeekRule = LeapWeekRule {
    leaps: 71,
    cycle: 400,
    offset: 203,
    epoch: EPOCH.0,
    min_year: MIN_YEAR,
    max_year: MAX_YEAR,
};

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = RULE.earliest();

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = RULE.latest();

/// Whether `year` has a leap week: `(71 · year + 203) mod 400 < 71`.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    RULE.is_leap_year(year)
}

/// The number of days in `year`, 364 or 371.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    RULE.days_in_year(year)
}

/// The number of days in `month` of `year`, or `None` outside `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    RULE.days_in_month(&MONTH_DAYS, year, month)
}

/// The fixed day of New Year's Day, 1 Arcturus, always a Monday.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn new_year(year: i64) -> CalendarResult<Rd> {
    RULE.new_year(year)
}

/// The fixed day of a date in the month form (LPM).
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    RULE.fixed_day(&MONTH_DAYS, year, month, day)
}

/// The year, month and day (LPM) of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EARLIEST`]..=[`LATEST`].
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    RULE.date_of(&MONTH_DAYS, rd)
}

/// A Hermetic Leap Week date in the month form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HermeticLeapWeekDate {
    /// The year, numbered astronomically.
    pub year: i64,
    /// The month, 1 for Arcturus through 12 for Lesath.
    pub month: u8,
    /// The day of the month, 1 through 28 or 35.
    pub day: u8,
}

impl HermeticLeapWeekDate {
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

    /// The day of the year, from 0.
    const fn day_of_year(self) -> i64 {
        let mut total = 0;
        let mut index = 0;
        while index < self.month as usize - 1 {
            total += MONTH_DAYS[index] as i64;
            index += 1;
        }
        total + self.day as i64 - 1
    }

    /// The week of the year, 1 to 53, as the week form (LPW) writes it.
    #[must_use]
    pub const fn week(self) -> u8 {
        (self.day_of_year() / 7 + 1) as u8
    }

    /// The weekday: every month is whole weeks and begins on a Monday.
    #[must_use]
    pub const fn weekday(self) -> Weekday {
        match (self.day - 1) % 7 {
            0 => Weekday::Monday,
            1 => Weekday::Tuesday,
            2 => Weekday::Wednesday,
            3 => Weekday::Thursday,
            4 => Weekday::Friday,
            5 => Weekday::Saturday,
            _ => Weekday::Sunday,
        }
    }
}

/// The Hermetic Leap Week Calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct HermeticLeapWeekCalendar;

/// Twelve months named after stars, and the seven-day week.
const SHAPE: &[CycleShape] = &[
    CycleShape::named(MONTH, &MONTHS),
    CycleShape::fixed(WEEKDAY, 7),
];

impl Calendar for HermeticLeapWeekCalendar {
    type Date = HermeticLeapWeekDate;

    /// Unrecorded: a proposal, adopted by nobody.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// Twelve named months and the week.
    fn cycles(&self) -> &'static [CycleShape] {
        SHAPE
    }

    /// A year with the leap week.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "Hermetic Leap Week",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &[],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(HermeticLeapWeekDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.month, date.day)
            .with_extra("week", i64::from(date.week()))?
            .with_extra("day-of-week", i64::from(date.weekday().iso_number()))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        HermeticLeapWeekDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian;

    fn gregorian(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn year_one_begins_on_monday_25_december_1_bc() {
        // "day 1 of year 1 is JDN 1,721,419 = 0-12-25 CE" (Meyer).
        assert_eq!(new_year(1), Ok(gregorian(0, 12, 25)));
        assert_eq!(EPOCH.to_julian_day_number(), 1_721_419);
        assert_eq!(Weekday::from_rd(EPOCH), Weekday::Monday);
    }

    #[test]
    fn the_new_years_of_the_hexade_of_2007_are_meyers() {
        // Meyer's table: 2007 to 2012, 2009 the leap year.
        for (year, gregorian_year, day, leap) in [
            (2007, 2006, 25, false),
            (2008, 2007, 24, false),
            (2009, 2008, 22, true),
            (2010, 2009, 28, false),
            (2011, 2010, 27, false),
            (2012, 2011, 26, false),
        ] {
            assert_eq!(
                new_year(year),
                Ok(gregorian(gregorian_year, 12, day)),
                "{year}"
            );
            assert_eq!(is_leap_year(year), leap, "{year}");
        }
    }

    #[test]
    fn the_dated_examples_are_meyers() {
        // Published 2007-01-08 CE = 2007-03-1 LPW = 2007-01-15 LPM.
        let published = gregorian(2007, 1, 8);
        assert_eq!(from_fixed(published), Ok((2007, 1, 15)));
        let date = HermeticLeapWeekDate::new(2007, 1, 15).unwrap();
        assert_eq!((date.week(), date.weekday()), (3, Weekday::Monday));
        // "2011-12-14 LPM = 2011-12-11 CE".
        assert_eq!(to_fixed(2011, 12, 14), Ok(gregorian(2011, 12, 11)));
        // "2007-10-10 LPM ... on a Wednesday".
        let tenth = HermeticLeapWeekDate::new(2007, 10, 10).unwrap();
        assert_eq!(tenth.weekday(), Weekday::Wednesday);
        assert_eq!(
            Weekday::from_rd(to_fixed(2007, 10, 10).unwrap()),
            Weekday::Wednesday
        );
    }

    #[test]
    fn the_leap_years_of_a_cycle_are_palmens_list() {
        // Palmen, "List of Leap Week Years".
        const LIST: [i64; 71] = [
            3, 9, 15, 20, 26, 31, 37, 43, 48, 54, 60, 65, 71, 77, 82, 88, 93, 99, 105, 110, 116,
            122, 127, 133, 138, 144, 150, 155, 161, 167, 172, 178, 184, 189, 195, 200, 206, 212,
            217, 223, 229, 234, 240, 246, 251, 257, 262, 268, 274, 279, 285, 291, 296, 302, 307,
            313, 319, 324, 330, 336, 341, 347, 353, 358, 364, 369, 375, 381, 386, 392, 398,
        ];
        assert!((1..=400).filter(|year| is_leap_year(*year)).eq(LIST));
        assert_eq!(to_fixed(401, 1, 1).unwrap().0 - EPOCH.0, 146_097);
    }

    /// Meyer's own definition — hexades from year 1, short when
    /// `71 · Y mod 100 < 26`, the third year leap — gives the leap years
    /// Palmen's rule does.
    #[test]
    fn the_hexade_definition_and_the_direct_rule_agree() {
        let mut start: i64 = 1;
        while start < 8_000 {
            let short = (start * 71).rem_euclid(100) < 26;
            let length = if short { 5 } else { 6 };
            for year in start..start + length {
                assert_eq!(is_leap_year(year), year == start + 2, "{year}");
            }
            start += length;
        }
    }

    #[test]
    fn new_year_falls_between_21_and_30_december() {
        // Meyer's table of New Year's Days for 1600-4000.
        for year in 1_600..=4_000 {
            let (y, month, day) = gregorian::from_fixed(new_year(year).unwrap()).unwrap();
            assert_eq!((y, month), (year - 1, 12), "{year}");
            assert!((21..=30).contains(&day), "{year}: {day}");
        }
    }

    #[test]
    fn every_day_round_trips_and_the_week_form_agrees() {
        for rd in (EARLIEST.0..=LATEST.0).step_by(997) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)));
        }
        let start = new_year(2000).unwrap().0;
        for rd in start..start + 3_000 {
            let date = HermeticLeapWeekCalendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(date.weekday(), Weekday::from_rd(Rd(rd)));
            let week_start = new_year(date.year).unwrap().0 + 7 * (i64::from(date.week()) - 1);
            assert_eq!(week_start + i64::from(date.weekday().iso_number()) - 1, rd);
            let fields = HermeticLeapWeekCalendar.to_fields(date).unwrap();
            assert_eq!(HermeticLeapWeekCalendar.from_fields(&fields), Ok(date));
        }
    }

    #[test]
    fn lesath_has_the_leap_week_and_nothing_else_does() {
        assert_eq!(days_in_month(2009, 12), Some(35));
        assert_eq!(days_in_month(2008, 12), Some(28));
        assert_eq!(to_fixed(2008, 12, 29), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(2008, 13, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(days_in_year(2009), 371);
        assert_eq!(
            to_fixed(MAX_YEAR + 1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            HermeticLeapWeekCalendar.is_leap_year(MIN_YEAR - 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
    }
}
