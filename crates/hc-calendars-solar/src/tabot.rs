//! The Tabot calendar of Mark Moore (Ras Mahitema Selassie), with the
//! rules Peter Meyer gave it in 2006.
//!
//! A Rastafari calendar first published on paper in 1997, whose year
//! begins on 2 November, the day of Haile Selassie's coronation in 1930,
//! and whose years count from that day: year 0 began on 2 November 1930,
//! and 27 September 2006 is Sawwara 30, 75 H.I.M. Moore asked for eleven
//! months of 30 days and a twelfth of 35, a day added every fourth year to
//! one of the first eleven, and New Year's Day always on 2 November;
//! Meyer's rules meet that by giving the fourth month, Ras, a 31st day in
//! a *long* year N, when N + 3 is divisible by 4 and N + 31 is not by 100,
//! or N + 331 is divisible by 400 — which is exactly when the February Ras
//! spans is a Gregorian leap February. So every month begins on the same
//! Gregorian date every year, Ras on 31 January and Sawwara on 29 August,
//! as Meyer tabulates, the same fixed-date shape as [`crate::bangladeshi`].
//! The calendar is written up with the other Hermetic Systems reforms in
//! `docs/systems/hermetic-reforms.md`.
//!
//! # Sources
//!
//! * Peter Meyer, "The Tabot Calendar", Hermetic Systems,
//!   <https://www.hermetic.ch/cal_stud/tabot.htm>, first published
//!   27 September 2006, retrieved 2026-09-26 (`meyer-tabot`): Moore's
//!   requirements, Meyer's definition, the month and weekday names, the
//!   month of Sawwara 75 and the table of the months' Gregorian first
//!   days. Moore's own publication of 1997 and the Tabot Ministries site
//!   it names, `www.tabot.co.uk`, were not read.
//!
//! # Exactness
//!
//! Exact: Meyer's rules are the definition.

use hc_calendar::shape::{CycleShape, MONTH, WEEKDAY};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Weekday,
    YearKind,
};

use crate::{common, gregorian};

/// The calendar identifier.
pub const ID: &str = "tabot";

/// The month names, as Meyer gives them.
pub const MONTHS: [&str; 12] = [
    "Anbassa",
    "Hymanot",
    "Immanuel",
    "Ras",
    "Ta'Berhan",
    "Manassa",
    "Danaffa",
    "Negest",
    "Tafari",
    "Emru",
    "Sawwara",
    "Negus & Dejazmatch",
];

/// The weekday names, Monday first as this crate orders the week; Meyer
/// lists them from Ergat, Sunday.
pub const WEEKDAYS: [&str; 7] = [
    "Tazajenat",
    "Kedusenant",
    "Ra'ee",
    "Makrab",
    "Mamlak",
    "Germa",
    "Ergat",
];

/// The month that takes the day of a long year.
pub const RAS: u8 = 4;

/// The Gregorian year in which year 0 begins.
const GREGORIAN_OFFSET: i64 = 1930;

/// The earliest year this implementation converts: year 0.
pub const MIN_YEAR: i64 = 0;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 99_999;

/// Whether year `year` is long, as Meyer states it: N + 3 divisible by 4
/// and N + 31 not by 100, or N + 331 divisible by 400.
#[must_use]
pub const fn is_long_year(year: i64) -> bool {
    ((year + 3).rem_euclid(4) == 0 && (year + 31).rem_euclid(100) != 0)
        || (year + 331).rem_euclid(400) == 0
}

/// The number of days in `month` of `year`, or `None` outside `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    match month {
        RAS => Some(if is_long_year(year) { 31 } else { 30 }),
        1..=11 => Some(30),
        12 => Some(35),
        _ => None,
    }
}

/// The number of days in `year`, 365 or 366.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_long_year(year) { 366 } else { 365 }
}

/// Days in the year before the first of `month`.
const fn days_before_month(year: i64, month: u8) -> i64 {
    let before = 30 * (month as i64 - 1);
    if month > RAS && is_long_year(year) {
        before + 1
    } else {
        before
    }
}

/// The fixed day of New Year's Day, 1 Anbassa, always 2 November.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn new_year(year: i64) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    gregorian::to_fixed(year + GREGORIAN_OFFSET, 11, 2)
}

/// The earliest fixed day this implementation converts: 2 November 1930.
pub const EARLIEST: Rd = match new_year(MIN_YEAR) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = match gregorian::to_fixed(MAX_YEAR + GREGORIAN_OFFSET + 1, 11, 1) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The fixed day of a Tabot date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    let start = match new_year(year) {
        Ok(start) => start,
        Err(error) => return Err(error),
    };
    match common::check_day(day, days_in_month(year, month)) {
        Err(error) => Err(error),
        Ok(()) => Ok(Rd(start.0 + days_before_month(year, month) + day as i64 - 1)),
    }
}

/// The Tabot year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EARLIEST`]..=[`LATEST`].
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd.0 < EARLIEST.0 {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd.0 > LATEST.0 {
        return Err(CalendarError::AfterSupportedRange);
    }
    // The year begins on 2 November, so it is the Gregorian year of the day
    // sixty days on — 1 January is day 60 of the Tabot year — less 1931.
    let year = match gregorian::year_from_fixed(Rd(rd.0 + 60)) {
        Ok(gregorian_year) => gregorian_year - GREGORIAN_OFFSET - 1,
        Err(error) => return Err(error),
    };
    let start = match new_year(year) {
        Ok(start) => start,
        Err(error) => return Err(error),
    };
    let day_of_year = rd.0 - start.0;
    let long = is_long_year(year);
    let (month, day) = if day_of_year < 90 {
        (day_of_year / 30 + 1, day_of_year % 30)
    } else if day_of_year < 120 + long as i64 {
        (RAS as i64, day_of_year - 90)
    } else {
        let after = day_of_year - 120 - long as i64;
        if after < 210 {
            (after / 30 + 5, after % 30)
        } else {
            (12, after - 210)
        }
    };
    Ok((year, month as u8, (day + 1) as u8))
}

/// A Tabot date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TabotDate {
    /// The year, from 0 on 2 November 1930.
    pub year: i64,
    /// The month, 1 for Anbassa through 12 for Negus & Dejazmatch.
    pub month: u8,
    /// The day of the month.
    pub day: u8,
}

impl TabotDate {
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
}

/// The Tabot calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TabotCalendar;

/// Twelve named months and the seven-day week under Moore's names.
const SHAPE: &[CycleShape] = &[
    CycleShape::named(MONTH, &MONTHS),
    CycleShape::named(WEEKDAY, &WEEKDAYS),
];

impl Calendar for TabotCalendar {
    type Date = TabotDate;

    /// Unrecorded: the sources give no period in which it was kept.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// Twelve named months and the named week.
    fn cycles(&self) -> &'static [CycleShape] {
        SHAPE
    }

    /// A long year, with Ras of 31 days.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(is_long_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "Tabot",
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
        Ok(TabotDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let weekday = Weekday::from_rd(to_fixed(date.year, date.month, date.day)?);
        DateFields::ymd(date.year, date.month, date.day)
            .with_extra("day-of-week", i64::from(weekday.iso_number()))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        TabotDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gregorian(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn the_page_was_published_on_sawwara_30_75() {
        // "first published September 27, 2006 CE = Sawwara 30, 75 H.I.M."
        assert_eq!(from_fixed(gregorian(2006, 9, 27)), Ok((75, 11, 30)));
        // "Sawwara 75 H.I.M. begins August 29th, 2006", a Kedusenant
        // (Tuesday) in Meyer's month table.
        let first = to_fixed(75, 11, 1).unwrap();
        assert_eq!(first, gregorian(2006, 8, 29));
        assert_eq!(Weekday::from_rd(first), Weekday::Tuesday);
        assert_eq!(
            WEEKDAYS[Weekday::from_rd(first).iso_number() as usize - 1],
            "Kedusenant"
        );
    }

    #[test]
    fn year_zero_begins_on_the_coronation() {
        assert_eq!(EARLIEST, gregorian(1930, 11, 2));
        assert_eq!(to_fixed(0, 1, 1), Ok(EARLIEST));
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
    }

    /// Meyer's table: the first day of each month on the same Gregorian
    /// date every year.
    #[test]
    fn every_month_begins_on_its_gregorian_date() {
        const FIRSTS: [(u8, u8); 12] = [
            (11, 2),
            (12, 2),
            (1, 1),
            (1, 31),
            (3, 2),
            (4, 1),
            (5, 1),
            (5, 31),
            (6, 30),
            (7, 30),
            (8, 29),
            (9, 28),
        ];
        for year in 0..=500 {
            for (index, (month, day)) in FIRSTS.iter().enumerate() {
                let gregorian_year = if index < 2 { 1930 + year } else { 1931 + year };
                assert_eq!(
                    to_fixed(year, index as u8 + 1, 1),
                    Ok(gregorian(gregorian_year, *month, *day)),
                    "year {year} month {}",
                    index + 1
                );
            }
        }
    }

    #[test]
    fn the_long_years_are_those_of_the_gregorian_leap_february() {
        for year in 0..=4_000 {
            assert_eq!(
                is_long_year(year),
                gregorian::is_leap_year(year + 1931),
                "{year}"
            );
            let length = to_fixed(year + 1, 1, 1).unwrap().0 - to_fixed(year, 1, 1).unwrap().0;
            assert_eq!(length, i64::from(days_in_year(year)));
        }
        assert_eq!(days_in_month(1, RAS), Some(31));
        assert_eq!(days_in_month(2, RAS), Some(30));
        assert_eq!(to_fixed(2, RAS, 31), Err(CalendarError::DayOutOfRange));
        assert_eq!(days_in_month(2, 12), Some(35));
        assert_eq!(days_in_month(2, 13), None);
    }

    #[test]
    fn every_day_round_trips() {
        for rd in EARLIEST.0..EARLIEST.0 + 40_000 {
            let date = TabotCalendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(TabotCalendar.to_fixed(date), Ok(Rd(rd)), "{rd}");
            let fields = TabotCalendar.to_fields(date).unwrap();
            assert_eq!(TabotCalendar.from_fields(&fields), Ok(date));
        }
        for rd in (EARLIEST.0..=LATEST.0).step_by(9_973) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)));
        }
        assert_eq!(from_fixed(LATEST).map(|(year, ..)| year), Ok(MAX_YEAR));
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }
}
