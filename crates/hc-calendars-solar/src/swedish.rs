//! The Swedish calendar of 1700–1712.
//!
//! In November 1699 Sweden resolved to reach the Gregorian calendar
//! gradually, by leaving out the eleven Julian leap days from 1700 to 1740
//! instead of dropping eleven days at once. Only the first step was taken:
//! 29 February 1700 was left out, so that from 1 March 1700 Sweden was one
//! day ahead of the Julian calendar and ten days behind the Gregorian.
//! Then, in the middle of the Great Northern War, the leap days of 1704 and
//! 1708 were kept, and the country was on a calendar of its own. In January
//! 1711 Charles XII ordered a return to the Julian calendar, and it was done
//! by giving February 1712 two leap days: Friday 30 February 1712, the
//! *tillökningsdag*, was followed by 1 March 1712 in the Julian reckoning.
//! Sweden went Gregorian in 1753, which is `julian-gregorian-se`.
//!
//! So this calendar is the Julian calendar with every date label one day
//! early — its 1 March 1700 is Julian 29 February 1700, its 29 February
//! 1704 is Julian 28 February 1704 — plus the one date no other calendar
//! has. It is bounded to the days it was kept, [`EARLIEST`] to [`LATEST`]:
//! before 1 March 1700 the Swedish date is [`crate::julian`], and from
//! 1 March 1712 it is [`crate::julian`] again, until 1753. Asking for a day
//! outside is refused rather than answered with a label nobody wrote. The
//! seven-day week ran on unbroken, as it did through every reform.
//!
//! The month numbering is the Julian one, January through December, and the
//! year is the Julian year; February 1712 has thirty days, and the months of
//! 1700 before March and of 1712 after February do not exist here.
//!
//! # Sources
//!
//! * Wikipedia, "Swedish calendar" and "Svenska kalendern", retrieved
//!   2026-09-25, for the dates and the equivalence 30 February 1712 =
//!   29 February 1712 Julian = 11 March 1712 Gregorian; they cite Emil
//!   Hildebrand, *Den svenska tidräkningen 1700–1712* (Stockholm, 1882),
//!   and Roscoe Lamont, "The reform of the Julian calendar (II)", *Popular
//!   Astronomy* 28 (1920), which were not read.
//! * Toke Nørby, "The Perpetual Calendar", `norbyhus.dk/calendar.php`,
//!   retrieved 2026-09-25: the transitional calendar of 1700.03.01 to
//!   1712.02.30 runs one day ahead of the Julian calendar throughout.
//! * Hans Högman, "Tideräkning", `hhogman.se/tiderakning.htm`, retrieved
//!   2026-09-25: the decision of November 1699, the leap day of 1700 taken
//!   out and no further one, the decision of January 1711, and the 1753
//!   cut-over from 17 February to 1 March.
//!
//! # Exactness
//!
//! Exact — arithmetic. Every day of the twelve years converts both ways,
//! and the tests below walk all of them.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Usage,
    YearKind,
};

use crate::{common, julian};

/// The calendar identifier.
pub const ID: &str = "swedish-1700";

/// The first year the calendar was kept, from 1 March.
pub const FIRST_YEAR: i64 = 1700;

/// The last year the calendar was kept, to 30 February.
pub const LAST_YEAR: i64 = 1712;

/// The day the calendar returned to the Julian reckoning on: 30 February
/// 1712, the *tillökningsdag*, as year, month and day.
pub const DOUBLE_LEAP_DAY: (i64, u8, u8) = (1712, 2, 30);

/// The earliest fixed day this calendar converts: 1 March 1700, which is
/// Julian 29 February 1700 and Gregorian 11 March 1700.
pub const EARLIEST: Rd = match julian::to_fixed(1700, 2, 29) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The latest fixed day this calendar converts: 30 February 1712, which is
/// Julian 29 February 1712 and Gregorian 11 March 1712.
pub const LATEST: Rd = match julian::to_fixed(1712, 2, 29) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// Whether `year` has a 29 February in this calendar: 1704 and 1708, the
/// leap days that should have been left out and were not, and 1712, which
/// has a 30 February as well. 1700 is a Julian leap year, but its February
/// precedes the calendar.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    matches!(year, 1704 | 1708 | 1712)
}

/// The number of days in `month` of `year`, or `None` when the calendar
/// never had that month: any month outside `1..=12`, January and February
/// 1700, and March 1712 onwards.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    if month == 0 || month > 12 {
        return None;
    }
    match (year, month) {
        (1700, 1 | 2) | (1712, 3..=12) => None,
        (1712, 2) => Some(30),
        (_, 2) => Some(if is_leap_year(year) { 29 } else { 28 }),
        _ => julian::days_in_month(year, month),
    }
}

/// The fixed day of a Swedish date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`FIRST_YEAR`]..=[`LAST_YEAR`], [`CalendarError::MonthOutOfRange`] for
/// a month the calendar never had, or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if year < FIRST_YEAR || year > LAST_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    if let Err(error) = common::check_day(day, days_in_month(year, month)) {
        return Err(error);
    }
    if year == DOUBLE_LEAP_DAY.0 && month == DOUBLE_LEAP_DAY.1 && day == DOUBLE_LEAP_DAY.2 {
        return Ok(LATEST);
    }
    // Every other label is the Julian label of the day after.
    match julian::to_fixed(year, month, day) {
        Ok(rd) => Ok(Rd(rd.0 - 1)),
        Err(error) => Err(error),
    }
}

/// The Swedish year, month and day of a fixed day.
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
    if rd.0 == LATEST.0 {
        return Ok(DOUBLE_LEAP_DAY);
    }
    julian::from_fixed(Rd(rd.0 + 1))
}

/// A date in the Swedish calendar of 1700–1712.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SwedishDate {
    /// The year, 1700 through 1712.
    pub year: i64,
    /// The month, 1 for January through 12 for December.
    pub month: u8,
    /// The day of the month, counting from 1; 30 in February 1712.
    pub day: u8,
}

impl SwedishDate {
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

    /// Whether this is 30 February 1712, the day that returned Sweden to
    /// the Julian calendar.
    #[must_use]
    pub const fn is_double_leap_day(self) -> bool {
        self.year == DOUBLE_LEAP_DAY.0
            && self.month == DOUBLE_LEAP_DAY.1
            && self.day == DOUBLE_LEAP_DAY.2
    }
}

/// The Swedish calendar of 1700–1712.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SwedishCalendar;

impl Calendar for SwedishCalendar {
    type Date = SwedishDate;

    /// 1704, 1708 and 1712 have a 29 February here; 1700's February
    /// precedes the calendar, and a year outside it is refused.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(FIRST_YEAR..=LAST_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(is_leap_year(year))
    }

    /// Twelve months and the seven-day week, which the reform never broke.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    /// Kept from 1 March 1700 to 30 February 1712, which is also the whole
    /// of the range it converts.
    fn usage(&self) -> Usage {
        Usage::between(EARLIEST, LATEST)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "Swedish (1700–1712)",
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
        Ok(SwedishDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields::ymd(date.year, date.month, date.day))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        SwedishDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian;
    use crate::julian_gregorian::{ReformCalendar, adoption_by_id};
    use hc_calendar::Weekday;

    #[test]
    fn the_calendar_begins_by_leaving_out_29_february_1700() {
        // 1 March 1700 in Sweden was 29 February 1700 in the Julian
        // calendar and 11 March 1700 in the Gregorian: a day ahead of the
        // one, ten behind the other.
        assert_eq!(EARLIEST, julian::to_fixed(1700, 2, 29).unwrap());
        assert_eq!(EARLIEST, gregorian::to_fixed(1700, 3, 11).unwrap());
        assert_eq!(to_fixed(1700, 3, 1), Ok(EARLIEST));
        assert_eq!(from_fixed(EARLIEST), Ok((1700, 3, 1)));
        assert_eq!(Weekday::from_rd(EARLIEST), Weekday::Thursday);
        // February 1700 was still Julian, so it is not a month of this
        // calendar at all.
        assert_eq!(days_in_month(1700, 2), None);
        assert_eq!(days_in_month(1700, 1), None);
        assert_eq!(to_fixed(1700, 2, 28), Err(CalendarError::MonthOutOfRange));
        assert_eq!(to_fixed(1700, 2, 29), Err(CalendarError::MonthOutOfRange));
        assert_eq!(days_in_month(1700, 3), Some(31));
    }

    #[test]
    fn thirty_february_1712_returns_it_to_the_julian_calendar() {
        // Friday 30 February 1712 was 29 February 1712 in the Julian
        // calendar and 11 March 1712 in the Gregorian; the day after it was
        // Julian 1 March 1712.
        assert_eq!(LATEST, julian::to_fixed(1712, 2, 29).unwrap());
        assert_eq!(LATEST, gregorian::to_fixed(1712, 3, 11).unwrap());
        assert_eq!(to_fixed(1712, 2, 30), Ok(LATEST));
        assert_eq!(from_fixed(LATEST), Ok(DOUBLE_LEAP_DAY));
        assert_eq!(Weekday::from_rd(LATEST), Weekday::Friday);
        assert_eq!(julian::from_fixed(Rd(LATEST.0 + 1)), Ok((1712, 3, 1)));
        assert_eq!(days_in_month(1712, 2), Some(30));
        assert_eq!(to_fixed(1712, 2, 29), julian::to_fixed(1712, 2, 28));
        assert!(SwedishDate::new(1712, 2, 30).unwrap().is_double_leap_day());
        assert!(!SwedishDate::new(1712, 2, 29).unwrap().is_double_leap_day());
        // From March 1712 the Swedish date is the Julian date again.
        assert_eq!(days_in_month(1712, 3), None);
        assert_eq!(to_fixed(1712, 3, 1), Err(CalendarError::MonthOutOfRange));
    }

    #[test]
    fn every_day_is_one_ahead_of_julian_and_ten_behind_gregorian() {
        let sweden_1753 =
            ReformCalendar::new(adoption_by_id("julian-gregorian-se").unwrap()).unwrap();
        for rd in EARLIEST.0..LATEST.0 {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            // The label is the Julian label of the following day.
            assert_eq!(
                julian::from_fixed(Rd(rd + 1)),
                Ok((year, month, day)),
                "rd {rd}"
            );
            // Which is why `julian-gregorian-se`, Julian until 1753, is
            // wrong by a day for every one of these days.
            let reform = sweden_1753.from_fixed(Rd(rd)).unwrap();
            assert_ne!(
                (reform.year, reform.month, reform.day),
                (year, month, day),
                "rd {rd}"
            );
        }
        // Ten days behind the Gregorian calendar, in the middle of the
        // period as at either end.
        assert_eq!(to_fixed(1706, 1, 1), gregorian::to_fixed(1706, 1, 11));
        assert_eq!(to_fixed(1710, 12, 31), gregorian::to_fixed(1711, 1, 10));
    }

    #[test]
    fn the_leap_days_of_1704_and_1708_were_kept() {
        assert!(is_leap_year(1704));
        assert!(is_leap_year(1708));
        assert!(!is_leap_year(1700));
        assert!(!is_leap_year(1706));
        assert_eq!(days_in_month(1704, 2), Some(29));
        assert_eq!(days_in_month(1708, 2), Some(29));
        assert_eq!(days_in_month(1705, 2), Some(28));
        // Swedish 29 February 1704 was Julian 28 February 1704, and
        // Swedish 1 March 1704 was the Julian leap day.
        assert_eq!(to_fixed(1704, 2, 29), julian::to_fixed(1704, 2, 28));
        assert_eq!(to_fixed(1704, 3, 1), julian::to_fixed(1704, 2, 29));
        assert_eq!(to_fixed(1704, 2, 30), Err(CalendarError::DayOutOfRange));
    }

    #[test]
    fn every_single_day_of_the_twelve_years_round_trips() {
        // 1 March 1700 to 30 February 1712 inclusive: twelve years of 365
        // days, the leap days of 1704, 1708 and 1712, and the first day.
        assert_eq!(LATEST.0 - EARLIEST.0 + 1, 12 * 365 + 3 + 1);
        let calendar = SwedishCalendar;
        for rd in EARLIEST.0..=LATEST.0 {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
    }

    #[test]
    fn days_outside_the_twelve_years_are_refused() {
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(to_fixed(1699, 12, 31), Err(CalendarError::YearOutOfRange));
        assert_eq!(to_fixed(1713, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(to_fixed(1705, 13, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(to_fixed(1705, 0, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(to_fixed(1712, 2, 31), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(1705, 4, 31), Err(CalendarError::DayOutOfRange));
        assert_eq!(days_in_month(1705, 13), None);
        assert_eq!(
            SwedishCalendar.from_fields(&DateFields::ymd_leap_month(1705, 4, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        let meta = SwedishCalendar.meta();
        assert_eq!(meta.id, CalendarId(ID));
        assert_eq!(SwedishCalendar.usage(), Usage::between(EARLIEST, LATEST));
    }
}
