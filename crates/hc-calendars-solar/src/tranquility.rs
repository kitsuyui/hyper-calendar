//! The Tranquility calendar of Jeff Siggins (*Omni*, July 1989).
//!
//! Thirteen months of 28 days named for scientists in alphabetical order,
//! Archimedes to Mendel, "for a total of 364 days. One extra day is added
//! on at the end of the year to make 365. For leap years, a second extra
//! day is added." The year begins on 1 Archimedes, "July 21 on the
//! Gregorian calendar"; its last day is **Armstrong Day**, the anniversary
//! of Moon Landing Day, 20 July, so that "July 20, 1970, is the Gregorian
//! equivalent of Armstrong Day, 1 A.T., and July 20, 1989, is Armstrong
//! Day, 20 A.T."; and **Aldrin Day**, the leap day, "falls between the
//! twenty-seventh and twenty-eighth of Hippocrates (February 29)", every
//! four years less "the leap days in every 400 years that are dropped",
//! the Gregorian rule. "The first of every month is a Friday and the
//! eleventh a Monday", so the two added days are outside the week.
//!
//! The years After Tranquility count from Moon Landing Day, 20 July 1969,
//! which "stands alone. Not part of any month or year"; year 1 A.T. began
//! the next day. The article names the time before as Before Tranquility
//! and numbers no year of it, so this module converts from 1 Archimedes
//! 1 A.T. onwards: Moon Landing Day and everything before it are
//! [`CalendarError::BeforeEpoch`], and the years B.T. are not carried.
//!
//! Armstrong Day is numbered 29 Mendel, the only way it fits a
//! year-month-day shape, as [`crate::international_fixed`] numbers Year
//! Day. Aldrin Day is numbered 29 Hippocrates for the same reason, though
//! it falls *before* the 28th: within a leap year's Hippocrates the day
//! numbers 28 and 29 are therefore not in date order, and
//! [`TranquilityDate`] does not derive an ordering. Both added days are
//! flagged `outside-the-week` by `to_fields`, and
//! [`TranquilityDate::weekday`] gives the calendar's own weekday, Friday
//! for the first of a month, which is not the unbroken week's:
//! 21 July 1969 was a Monday by `hc_calendar::Weekday::from_rd`.
//!
//! # Sources
//!
//! * Jeff Siggins, "Tranquility Calendar", *Omni*, July 1989, as
//!   transcribed at "Tranquility Calendar Text From Omni Magazine",
//!   <https://www.mithrandir.com/Tranquility/tranquilityArticle.html>,
//!   retrieved 2026-10-04 (`siggins1989-tranquility`): every rule and
//!   quotation above. The printed issue was not read.
//!
//! # Exactness
//!
//! Exact: the article's rules are the definition, and the leap rule is the
//! Gregorian one applied to the February each year contains.

use hc_calendar::shape::{CycleShape, MONTH, WEEKDAY};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Weekday,
    YearKind,
};

use crate::{common, gregorian};

/// The calendar identifier.
pub const ID: &str = "tranquility";

/// The era code of the years After Tranquility.
pub const ERA: &str = "at";

/// The thirteen months, in the alphabetical order the article gives them.
pub const MONTHS: [&str; 13] = [
    "Archimedes",
    "Brahe",
    "Copernicus",
    "Darwin",
    "Einstein",
    "Faraday",
    "Galileo",
    "Hippocrates",
    "Imhotep",
    "Jung",
    "Kepler",
    "Lavoisier",
    "Mendel",
];

/// The length of every month before the added days.
pub const DAYS_IN_MONTH: u8 = 28;

/// The day number given to Armstrong Day and Aldrin Day within their month.
pub const INTERCALARY_DAY: u8 = 29;

/// The month Aldrin Day falls in, Hippocrates.
pub const ALDRIN_DAY_MONTH: u8 = 8;

/// The month Armstrong Day ends, Mendel.
pub const ARMSTRONG_DAY_MONTH: u8 = 13;

/// The name of the year's last day, outside the week.
pub const ARMSTRONG_DAY: &str = "Armstrong Day";

/// The name of the leap day, outside the week.
pub const ALDRIN_DAY: &str = "Aldrin Day";

/// Year *N* A.T. begins on 21 July of Gregorian year *N* + 1968.
pub const GREGORIAN_OFFSET: i64 = 1968;

/// The earliest year this implementation converts: 1 A.T.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 99_999;

/// Whether `year` has an Aldrin Day: the Gregorian year its Hippocrates
/// falls in, `year` + 1969, has a 29 February.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    gregorian::is_leap_year(year + GREGORIAN_OFFSET + 1)
}

/// The number of days in `month` of `year`, or `None` outside `1..=13`:
/// 28, 29 for Mendel, and 29 for Hippocrates in a leap year.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    if month == 0 || month > ARMSTRONG_DAY_MONTH {
        return None;
    }
    let extra = if month == ARMSTRONG_DAY_MONTH || (month == ALDRIN_DAY_MONTH && is_leap_year(year))
    {
        1
    } else {
        0
    };
    Some(DAYS_IN_MONTH + extra)
}

/// The number of days in `year`, 365 or 366.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// Days in the year before the first of `month`.
const fn days_before_month(year: i64, month: u8) -> i64 {
    let elapsed = DAYS_IN_MONTH as i64 * (month as i64 - 1);
    if month > ALDRIN_DAY_MONTH && is_leap_year(year) {
        elapsed + 1
    } else {
        elapsed
    }
}

/// Days into `month` of `year` of its day `day`: `day - 1`, except in a
/// leap year's Hippocrates, where Aldrin Day, numbered 29, is the 28th day
/// and the 28th is the 29th.
const fn days_before_day(year: i64, month: u8, day: u8) -> i64 {
    if month == ALDRIN_DAY_MONTH && is_leap_year(year) {
        match day {
            INTERCALARY_DAY => 27,
            DAYS_IN_MONTH => 28,
            _ => day as i64 - 1,
        }
    } else {
        day as i64 - 1
    }
}

/// The fixed day of 1 Archimedes of `year`, 21 July.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn new_year(year: i64) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    gregorian::to_fixed(year + GREGORIAN_OFFSET, 7, 21)
}

/// The earliest fixed day this implementation converts: 1 Archimedes
/// 1 A.T., 21 July 1969.
pub const EARLIEST: Rd = match new_year(MIN_YEAR) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// Moon Landing Day, 20 July 1969, the day before [`EARLIEST`], which
/// belongs to no year.
pub const MOON_LANDING_DAY: Rd = Rd(EARLIEST.0 - 1);

/// The latest fixed day this implementation converts: Armstrong Day of
/// [`MAX_YEAR`].
pub const LATEST: Rd = match gregorian::to_fixed(MAX_YEAR + GREGORIAN_OFFSET + 1, 7, 20) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The fixed day of a Tranquility date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`] —
/// the last also for Aldrin Day in a year that has none.
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    let start = match new_year(year) {
        Ok(start) => start,
        Err(error) => return Err(error),
    };
    match common::check_day(day, days_in_month(year, month)) {
        Err(error) => Err(error),
        Ok(()) => Ok(Rd(start.0
            + days_before_month(year, month)
            + days_before_day(year, month, day))),
    }
}

/// The Tranquility year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] up to and including Moon Landing
/// Day, or [`CalendarError::AfterSupportedRange`] after [`LATEST`].
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd.0 < EARLIEST.0 {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd.0 > LATEST.0 {
        return Err(CalendarError::AfterSupportedRange);
    }
    // The year begins on 21 July, so it is the Gregorian year of the day
    // 164 days on — 1 January is day 165 of the year — less 1969.
    let year = match gregorian::year_from_fixed(Rd(rd.0 + 164)) {
        Ok(gregorian_year) => gregorian_year - GREGORIAN_OFFSET - 1,
        Err(error) => return Err(error),
    };
    let start = match new_year(year) {
        Ok(start) => start,
        Err(error) => return Err(error),
    };
    let mut elapsed = rd.0 - start.0;
    // Day 223, counting from 0, is the day after 27 Hippocrates.
    let aldrin_day_at = DAYS_IN_MONTH as i64 * (ALDRIN_DAY_MONTH as i64 - 1) + 27;
    if is_leap_year(year) {
        if elapsed == aldrin_day_at {
            return Ok((year, ALDRIN_DAY_MONTH, INTERCALARY_DAY));
        }
        if elapsed > aldrin_day_at {
            elapsed -= 1;
        }
    }
    if elapsed == DAYS_IN_MONTH as i64 * ARMSTRONG_DAY_MONTH as i64 {
        return Ok((year, ARMSTRONG_DAY_MONTH, INTERCALARY_DAY));
    }
    let month = (elapsed / DAYS_IN_MONTH as i64 + 1) as u8;
    let day = (elapsed % DAYS_IN_MONTH as i64 + 1) as u8;
    Ok((year, month, day))
}

/// A Tranquility date.
///
/// No ordering is derived: in a leap year Aldrin Day, 29 Hippocrates,
/// precedes 28 Hippocrates. Compare fixed days instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TranquilityDate {
    /// The year After Tranquility, from 1.
    pub year: i64,
    /// The month, 1 for Archimedes through 13 for Mendel.
    pub month: u8,
    /// The day of the month, 1 through 28. Day 29 of Mendel is Armstrong
    /// Day and day 29 of Hippocrates is Aldrin Day; neither is in a week.
    pub day: u8,
}

impl TranquilityDate {
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

    /// The name of the month.
    #[must_use]
    pub const fn month_name(self) -> &'static str {
        MONTHS[self.month as usize - 1]
    }

    /// Whether this date is Armstrong Day, the year's last day.
    #[must_use]
    pub const fn is_armstrong_day(self) -> bool {
        self.month == ARMSTRONG_DAY_MONTH && self.day == INTERCALARY_DAY
    }

    /// Whether this date is Aldrin Day, the leap day.
    #[must_use]
    pub const fn is_aldrin_day(self) -> bool {
        self.month == ALDRIN_DAY_MONTH && self.day == INTERCALARY_DAY
    }

    /// Whether this date belongs to the seven-day week at all.
    #[must_use]
    pub const fn is_in_the_week(self) -> bool {
        !self.is_armstrong_day() && !self.is_aldrin_day()
    }

    /// The name of the added day this is, if it is one.
    #[must_use]
    pub const fn added_day(self) -> Option<&'static str> {
        if self.is_armstrong_day() {
            Some(ARMSTRONG_DAY)
        } else if self.is_aldrin_day() {
            Some(ALDRIN_DAY)
        } else {
            None
        }
    }

    /// The calendar's own weekday, or `None` for an added day.
    ///
    /// Every month opens on a Friday, so this depends only on the day of
    /// the month, and it is not [`Weekday::from_rd`]: see the module
    /// documentation.
    #[must_use]
    pub const fn weekday(self) -> Option<Weekday> {
        if !self.is_in_the_week() {
            return None;
        }
        Some(match (self.day - 1) % 7 {
            0 => Weekday::Friday,
            1 => Weekday::Saturday,
            2 => Weekday::Sunday,
            3 => Weekday::Monday,
            4 => Weekday::Tuesday,
            5 => Weekday::Wednesday,
            _ => Weekday::Thursday,
        })
    }
}

/// The Tranquility calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TranquilityCalendar;

/// Thirteen named months and the seven-day week, whose positions are the
/// seven although the added days sit outside it.
const SHAPE: &[CycleShape] = &[
    CycleShape::named(MONTH, &MONTHS),
    CycleShape::fixed(WEEKDAY, 7),
];

impl Calendar for TranquilityCalendar {
    type Date = TranquilityDate;

    /// Unrecorded: a proposal, adopted by nobody.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// The thirteen months and the week.
    fn cycles(&self) -> &'static [CycleShape] {
        SHAPE
    }

    /// A year with an Aldrin Day.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "Tranquility",
            year_kind: YearKind::EpochForward,
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
        Ok(TranquilityDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.month, date.day)
            .with_era(ERA)
            .with_extra("outside-the-week", i64::from(!date.is_in_the_week()))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        TranquilityDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gregorian(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn the_articles_dates() {
        // "The first day of each year is the first day of Archimedes (July
        // 21 on the Gregorian calendar)."
        assert_eq!(EARLIEST, gregorian(1969, 7, 21));
        assert_eq!(from_fixed(EARLIEST), Ok((1, 1, 1)));
        // "July 20, 1970, is the Gregorian equivalent of Armstrong Day,
        // 1 A.T., and July 20, 1989, is Armstrong Day, 20 A.T."
        assert_eq!(to_fixed(1, 13, 29), Ok(gregorian(1970, 7, 20)));
        assert_eq!(from_fixed(gregorian(1989, 7, 20)), Ok((20, 13, 29)));
        // "The Gregorian equivalent of Moon Landing Day is July 20, A.D.
        // 1969", and it is "Not part of any month or year".
        assert_eq!(MOON_LANDING_DAY, gregorian(1969, 7, 20));
        assert_eq!(
            from_fixed(MOON_LANDING_DAY),
            Err(CalendarError::BeforeEpoch)
        );
        // "Aldrin Day falls between the twenty-seventh and twenty-eighth of
        // Hippocrates (February 29)": the first one, in 3 A.T.
        assert!(is_leap_year(3));
        assert_eq!(to_fixed(3, 8, 27), Ok(gregorian(1972, 2, 28)));
        assert_eq!(to_fixed(3, 8, 29), Ok(gregorian(1972, 2, 29)));
        assert_eq!(to_fixed(3, 8, 28), Ok(gregorian(1972, 3, 1)));
        assert_eq!(from_fixed(gregorian(1972, 2, 29)), Ok((3, 8, 29)));
        assert!(!is_leap_year(2));
        assert_eq!(to_fixed(2, 8, 28), Ok(gregorian(1971, 3, 1)));
        assert_eq!(to_fixed(2, 8, 29), Err(CalendarError::DayOutOfRange));
        // Year 31 A.T. holds February 2000, leap; 131 A.T. holds February
        // 2100, one of the dropped centennial leap days.
        assert!(is_leap_year(31));
        assert_eq!(to_fixed(31, 8, 29), Ok(gregorian(2000, 2, 29)));
        assert!(!is_leap_year(131));
        assert_eq!(to_fixed(131, 8, 29), Err(CalendarError::DayOutOfRange));
    }

    #[test]
    fn the_first_of_every_month_is_a_friday_and_the_eleventh_a_monday() {
        for month in 1..=13 {
            let first = TranquilityDate::new(20, month, 1).unwrap();
            assert_eq!(first.weekday(), Some(Weekday::Friday));
            let eleventh = TranquilityDate::new(20, month, 11).unwrap();
            assert_eq!(eleventh.weekday(), Some(Weekday::Monday));
            let last = TranquilityDate::new(20, month, 28).unwrap();
            assert_eq!(last.weekday(), Some(Weekday::Thursday));
        }
        let armstrong = TranquilityDate::new(20, 13, 29).unwrap();
        assert_eq!(armstrong.weekday(), None);
        assert_eq!(armstrong.added_day(), Some(ARMSTRONG_DAY));
        let aldrin = TranquilityDate::new(3, 8, 29).unwrap();
        assert_eq!(aldrin.weekday(), None);
        assert_eq!(aldrin.added_day(), Some(ALDRIN_DAY));
        assert_eq!(TranquilityDate::new(3, 8, 27).unwrap().added_day(), None);
        // The unbroken week disagrees: 21 July 1969 was a Monday.
        assert_eq!(Weekday::from_rd(EARLIEST), Weekday::Monday);
        assert_eq!(MONTHS[7], "Hippocrates");
        assert_eq!(
            TranquilityDate::new(1, 13, 1).unwrap().month_name(),
            "Mendel"
        );
    }

    #[test]
    fn the_fields_carry_the_era_and_the_flag() {
        let calendar = TranquilityCalendar;
        let fields = calendar
            .to_fields(TranquilityDate::new(20, 13, 29).unwrap())
            .unwrap();
        assert_eq!(fields.era, Some(ERA));
        assert_eq!(fields.extra.get("outside-the-week"), Some(1));
        let fields = calendar
            .to_fields(TranquilityDate::new(20, 1, 1).unwrap())
            .unwrap();
        assert_eq!(fields.extra.get("outside-the-week"), Some(0));
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(20, 1, 1).with_era("AD")),
            Err(CalendarError::UnknownEra)
        );
        assert_eq!(
            calendar.from_fields(&DateFields::ymd_leap_month(20, 8, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(to_fixed(1, 14, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(days_in_month(1, 0), None);
        assert_eq!(
            calendar.is_leap_year(MAX_YEAR + 1),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn every_day_round_trips() {
        for rd in EARLIEST.0..EARLIEST.0 + 40_000 {
            let date = TranquilityCalendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(TranquilityCalendar.to_fixed(date), Ok(Rd(rd)), "{rd}");
            let fields = TranquilityCalendar.to_fields(date).unwrap();
            assert_eq!(TranquilityCalendar.from_fields(&fields), Ok(date));
        }
        // Every day in a release build; every 9 973rd in a debug one, with
        // each year's first and last day.
        let year_starts = (MIN_YEAR..=MAX_YEAR).map(|year| new_year(year).unwrap().0);
        for rd in crate::sweep_days(EARLIEST.0, LATEST.0, 9_973, year_starts) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)));
        }
        for year in [1, 2, 3, 31, 32, 131, 132, 431, 432] {
            let length = new_year(year + 1).unwrap().0 - new_year(year).unwrap().0;
            assert_eq!(length, i64::from(days_in_year(year)), "{year}");
        }
        assert_eq!(from_fixed(LATEST), Ok((MAX_YEAR, 13, 29)));
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }
}
