//! The Kurdish solar year, as kept in the Kurdistan Region of Iraq.
//!
//! The year begins on Newroz, 21 March, with six months of 31 days,
//! Xakelêwe to Xermanan, then five of 30, Rezber to Rêbendan, and Reşeme,
//! the twelfth, of 29 or 30 days, 20 February to 20 March: each month
//! spans the same Gregorian dates in every year, so Reşeme has a 30th
//! day exactly when the Gregorian February it spans has a 29th, which is
//! its 10th. That is
//! the same fixed-date shape as [`crate::bangladeshi`] and
//! [`crate::tabot`]. The year is numbered 700 ahead of the Gregorian year
//! it begins in: the year that began on 21 March 2026 is 2726, in which
//! 4 October 2026 is 12 Rezber.
//!
//! The source says the calendar "is formally recognized for cultural and
//! official use in the Kurdistan Region of Iraq", and gives no date for
//! that, so its period of use is unrecorded. It also says the count begins
//! with "the Battle of Nineveh (612 BC)", which does not give 2726 for
//! 2026 by any reckoning of years, so that epoch is not carried; the year
//! number is taken from the dates the source writes.
//!
//! # Sources
//!
//! * Wikipedia, "Kurdish calendar",
//!   <https://en.wikipedia.org/wiki/Kurdish_calendar>, retrieved
//!   2026-10-04 (`wikipedia-kurdish-calendar`): the months, their lengths
//!   and Gregorian spans, Newroz on 21 March, the recognition in the
//!   Kurdistan Region, and the date it wrote for the day it was read,
//!   12 Rezber 2726. A secondary source; the Kurdistan Region's own
//!   instrument was not found and would replace it. The article writes the
//!   months in Sorani script as well, خاکەلێوە to ڕەشەمە; no Kurdish locale
//!   carries them yet, so the romanised forms are the shape's names.
//!
//! # Exactness
//!
//! Exact over the Gregorian dates the source tabulates.

use hc_calendar::shape::{CycleShape, MONTH, WEEKDAY};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::{common, gregorian};

/// The calendar identifier.
pub const ID: &str = "kurdish";

/// The month names, romanised as the source gives them.
pub const MONTHS: [&str; 12] = [
    "Xakelêwe",
    "Gulan",
    "Cozerdan",
    "Pûşper",
    "Gelawêj",
    "Xermanan",
    "Rezber",
    "Gelarêzan",
    "Sermawez",
    "Befranbar",
    "Rêbendan",
    "Reşeme",
];

/// How far the Kurdish year runs ahead of the Gregorian year it begins in.
pub const YEAR_OFFSET: i64 = 700;

/// The month whose length varies, Reşeme.
pub const RESHEME: u8 = 12;

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 9_999;

/// Whether `year` has a 30th of Reşeme: the Gregorian year it ends in has
/// a 29 February.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    gregorian::is_leap_year(year - YEAR_OFFSET + 1)
}

/// The number of days in `month` of `year`, or `None` outside `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    match month {
        1..=6 => Some(31),
        7..=11 => Some(30),
        RESHEME => Some(if is_leap_year(year) { 30 } else { 29 }),
        _ => None,
    }
}

/// The number of days in `year`, 365 or 366.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// Days in the year before the first of `month`.
const fn days_before_month(month: u8) -> i64 {
    let month = month as i64 - 1;
    if month <= 6 {
        31 * month
    } else {
        186 + 30 * (month - 6)
    }
}

/// The fixed day of Newroz, 1 Xakelêwe, 21 March.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn new_year(year: i64) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    gregorian::to_fixed(year - YEAR_OFFSET, 3, 21)
}

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = match new_year(MIN_YEAR) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The latest fixed day this implementation converts, 20 March of the
/// Gregorian year after [`MAX_YEAR`] began.
pub const LATEST: Rd = match gregorian::to_fixed(MAX_YEAR - YEAR_OFFSET + 1, 3, 20) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The fixed day of a Kurdish date.
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
        Ok(()) => Ok(Rd(start.0 + days_before_month(month) + day as i64 - 1)),
    }
}

/// The Kurdish year, month and day of a fixed day.
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
    // The year begins on 21 March, so it is the Gregorian year of the day
    // 286 days on — 1 January is day 287 of the year — less 1, plus 700.
    let year = match gregorian::year_from_fixed(Rd(rd.0 + 286)) {
        Ok(gregorian_year) => gregorian_year - 1 + YEAR_OFFSET,
        Err(error) => return Err(error),
    };
    let start = match new_year(year) {
        Ok(start) => start,
        Err(error) => return Err(error),
    };
    let day_of_year = rd.0 - start.0;
    let (month, day) = if day_of_year < 186 {
        (day_of_year / 31 + 1, day_of_year % 31)
    } else {
        let after = day_of_year - 186;
        if after < 150 {
            (after / 30 + 7, after % 30)
        } else {
            (RESHEME as i64, after - 150)
        }
    };
    Ok((year, month as u8, (day + 1) as u8))
}

/// A Kurdish date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KurdishDate {
    /// The year, 700 ahead of the Gregorian year it begins in.
    pub year: i64,
    /// The month, 1 for Xakelêwe through 12 for Reşeme.
    pub month: u8,
    /// The day of the month.
    pub day: u8,
}

impl KurdishDate {
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
}

/// The Kurdish calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct KurdishCalendar;

/// Twelve named months and the seven-day week.
const SHAPE: &[CycleShape] = &[
    CycleShape::named(MONTH, &MONTHS),
    CycleShape::fixed(WEEKDAY, 7),
];

impl Calendar for KurdishCalendar {
    type Date = KurdishDate;

    /// Unrecorded: the source says the calendar is recognised in the
    /// Kurdistan Region of Iraq and gives no day it was recognised from.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// Twelve named months and the week.
    fn cycles(&self) -> &'static [CycleShape] {
        SHAPE
    }

    /// A year with a 30th of Reşeme.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "Kurdish",
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
        Ok(KurdishDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields::ymd(date.year, date.month, date.day))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        KurdishDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gregorian(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn the_source_wrote_12_rezber_2726_for_4_october_2026() {
        assert_eq!(from_fixed(gregorian(2026, 10, 4)), Ok((2726, 7, 12)));
        assert_eq!(
            KurdishDate::new(2726, 7, 12).unwrap().month_name(),
            "Rezber"
        );
        // Newroz, 21 March 2026, begins 2726.
        assert_eq!(new_year(2726), Ok(gregorian(2026, 3, 21)));
        assert_eq!(from_fixed(gregorian(2026, 3, 20)), Ok((2725, 12, 29)));
    }

    /// The source's table: each month's first and last Gregorian day, the
    /// same in every year.
    #[test]
    fn every_month_spans_its_gregorian_dates() {
        const SPANS: [((u8, u8), (u8, u8)); 12] = [
            ((3, 21), (4, 20)),
            ((4, 21), (5, 21)),
            ((5, 22), (6, 21)),
            ((6, 22), (7, 22)),
            ((7, 23), (8, 22)),
            ((8, 23), (9, 22)),
            ((9, 23), (10, 22)),
            ((10, 23), (11, 21)),
            ((11, 22), (12, 21)),
            ((12, 22), (1, 20)),
            ((1, 21), (2, 19)),
            ((2, 20), (3, 20)),
        ];
        for year in 2600..=2800 {
            for (index, ((first_month, first_day), (last_month, last_day))) in
                SPANS.iter().enumerate()
            {
                let month = index as u8 + 1;
                // Befranbar begins on 22 December and ends on 20 January.
                let first_year = if month <= 10 { year - 700 } else { year - 699 };
                let last_year = if month <= 9 { year - 700 } else { year - 699 };
                assert_eq!(
                    to_fixed(year, month, 1),
                    Ok(gregorian(first_year, *first_month, *first_day)),
                    "{year} month {month}"
                );
                let length = days_in_month(year, month).unwrap();
                assert_eq!(
                    to_fixed(year, month, length),
                    Ok(gregorian(last_year, *last_month, *last_day)),
                    "{year} month {month}"
                );
            }
        }
    }

    #[test]
    fn resheme_has_thirty_days_when_february_has_twenty_nine() {
        for year in MIN_YEAR..=MAX_YEAR {
            assert_eq!(
                is_leap_year(year),
                gregorian::is_leap_year(year - 699),
                "{year}"
            );
            let length =
                new_year(year + 1).map_or(LATEST.0 + 1, |rd| rd.0) - new_year(year).unwrap().0;
            assert_eq!(length, i64::from(days_in_year(year)), "{year}");
        }
        assert_eq!(days_in_month(2727, RESHEME), Some(30));
        assert_eq!(to_fixed(2727, RESHEME, 10), Ok(gregorian(2028, 2, 29)));
        assert_eq!(to_fixed(2727, RESHEME, 30), Ok(gregorian(2028, 3, 20)));
        assert_eq!(to_fixed(2726, RESHEME, 10), Ok(gregorian(2027, 3, 1)));
        assert_eq!(days_in_month(2726, RESHEME), Some(29));
        assert_eq!(
            to_fixed(2726, RESHEME, 30),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(days_in_month(2726, 13), None);
        assert_eq!(to_fixed(2726, 0, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            KurdishCalendar.from_fields(&DateFields::ymd_leap_month(2726, 1, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
    }

    #[test]
    fn every_day_round_trips() {
        let recent = new_year(2600).unwrap().0;
        for rd in recent..recent + 40_000 {
            let date = KurdishCalendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(KurdishCalendar.to_fixed(date), Ok(Rd(rd)), "{rd}");
            let fields = KurdishCalendar.to_fields(date).unwrap();
            assert_eq!(KurdishCalendar.from_fields(&fields), Ok(date));
        }
        // Every day in a release build; every 9 973rd in a debug one, with
        // each year's first and last day.
        let year_starts = (MIN_YEAR..=MAX_YEAR).map(|year| new_year(year).unwrap().0);
        for rd in crate::sweep_days(EARLIEST.0, LATEST.0, 9_973, year_starts) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)));
        }
        assert_eq!(from_fixed(EARLIEST), Ok((MIN_YEAR, 1, 1)));
        assert_eq!(from_fixed(LATEST), Ok((MAX_YEAR, RESHEME, 29)));
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }
}
