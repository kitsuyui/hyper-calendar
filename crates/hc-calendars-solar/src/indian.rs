//! The Indian national civil calendar (Śaka era).
//!
//! The calendar the Calendar Reform Committee under Meghnad Saha proposed in
//! 1955 and India adopted on 22 March 1957 — 1 Chaitra 1879 Śaka — for the
//! Gazette of India, All India Radio and government communications. It is a
//! deliberately simple construction: take the Śaka era year number, tie the
//! new year to a fixed Gregorian date, and inherit the Gregorian leap rule
//! so that the two calendars never drift apart.
//!
//! * The year is the Gregorian year minus 78.
//! * The year begins on 22 March, or on 21 March when the Gregorian year it
//!   begins in is a leap year.
//! * Chaitra has 30 days, or 31 in those same leap years; Vaisakha to
//!   Bhadra have 31; Asvina to Phalguna have 30.
//!
//! # What this deliberately does not do
//!
//! This is the *civil* calendar, not the religious one. The many regional
//! Hindu calendars — lunisolar, with tithis, adhika months and sunrise-based
//! day boundaries — are a different problem living in another crate; the
//! reform committee designed this calendar precisely so that civil dating
//! would not have to solve it. The historical Śaka era itself is older than
//! the national calendar and was reckoned differently; only the 1957 rules
//! are implemented, projected backwards.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::gregorian;

/// How far the Common Era runs ahead of the Śaka era.
pub const YEAR_OFFSET: i64 = 78;

/// The era code of the Śaka era.
pub const ERA: &str = "Saka";

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 9_999;

/// Whether `year` is a leap year.
///
/// It is exactly when the Gregorian year the Śaka year opens in is one, so
/// the extra day lands in Chaitra rather than in February.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    gregorian::is_leap_year(year + YEAR_OFFSET)
}

/// The number of days in `month` of `year`, or `None` when `month` is not in
/// `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    match month {
        1 => Some(if is_leap_year(year) { 31 } else { 30 }),
        2..=6 => Some(31),
        7..=12 => Some(30),
        _ => None,
    }
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// The fixed day of 1 Chaitra of `year`.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn new_year(year: i64) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    Ok(Rd(new_year_raw(year)))
}

/// The fixed day of 1 Chaitra of `year`, without the range check, so that
/// the range itself can be expressed in terms of it.
const fn new_year_raw(year: i64) -> i64 {
    let gregorian_year = year + YEAR_OFFSET;
    let day = if gregorian::is_leap_year(gregorian_year) {
        21
    } else {
        22
    };
    match gregorian::to_fixed(gregorian_year, 3, day) {
        Ok(rd) => rd.0,
        // Unreachable for any year inside the Gregorian range, which the
        // bounds on this calendar guarantee.
        Err(_) => 0,
    }
}

/// Days elapsed in the year before the first of `month`.
const fn days_before_month(month: u8, leap: bool) -> i64 {
    if month == 1 {
        return 0;
    }
    let chaitra = if leap { 31 } else { 30 };
    let long_months = if month <= 7 { month as i64 - 2 } else { 5 };
    let short_months = if month > 7 { month as i64 - 7 } else { 0 };
    chaitra + 31 * long_months + 30 * short_months
}

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = Rd(new_year_raw(MIN_YEAR));

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(new_year_raw(MAX_YEAR + 1) - 1);

/// The fixed day of an Indian national calendar date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    match crate::common::check_day(day, days_in_month(year, month)) {
        Err(error) => Err(error),
        Ok(()) => match new_year(year) {
            Err(error) => Err(error),
            Ok(start) => Ok(Rd(start.0
                + days_before_month(month, is_leap_year(year))
                + day as i64
                - 1)),
        },
    }
}

/// The Indian national calendar year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EARLIEST`]..=[`LATEST`].
pub fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd < EARLIEST {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd > LATEST {
        return Err(CalendarError::AfterSupportedRange);
    }
    // The Śaka year that a day belongs to opens either in the Gregorian year
    // of that day or in the one before, never any earlier.
    let (gregorian_year, _, _) = gregorian::from_fixed(rd)?;
    // The clamp matters only at the very last supported day, which falls in
    // the Gregorian year after the last supported Śaka year opens.
    let mut year = (gregorian_year - YEAR_OFFSET).min(MAX_YEAR);
    let mut start = new_year(year)?;
    if rd < start {
        year -= 1;
        start = new_year(year)?;
    }
    let leap = is_leap_year(year);
    let day_of_year = rd.0 - start.0 + 1;
    let chaitra = if leap { 31 } else { 30 };
    let (month, day) = if day_of_year <= chaitra {
        (1u8, day_of_year)
    } else {
        let after_chaitra = day_of_year - chaitra;
        if after_chaitra <= 5 * 31 {
            (
                (2 + (after_chaitra - 1) / 31) as u8,
                (after_chaitra - 1) % 31 + 1,
            )
        } else {
            let after_long = after_chaitra - 5 * 31;
            ((7 + (after_long - 1) / 30) as u8, (after_long - 1) % 30 + 1)
        }
    };
    Ok((year, month, day as u8))
}

/// A date in the Indian national calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndianDate {
    /// The year of the Śaka era, counting from 1.
    pub year: i64,
    /// The month, 1 for Chaitra through 12 for Phalguna.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl IndianDate {
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

    /// The Common Era year this Śaka year opens in.
    #[must_use]
    pub const fn common_era_year(self) -> i64 {
        self.year + YEAR_OFFSET
    }
}

/// The Indian national civil calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IndianCalendar;

impl Calendar for IndianCalendar {
    type Date = IndianDate;

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("indian"),
            english_name: "Indian national (Śaka)",
            year_kind: YearKind::EpochForward,
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
        Ok(IndianDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields::ymd(date.year, date.month, date.day).with_era(ERA))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        IndianDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_calendar_was_adopted_on_the_twenty_second_of_march_1957() {
        // 1 Chaitra 1879 Śaka is 22 March 1957, the day the reform took
        // effect in the Gazette of India.
        assert_eq!(
            to_fixed(1879, 1, 1),
            Ok(gregorian::to_fixed(1957, 3, 22).unwrap())
        );
        assert_eq!(IndianDate::new(1879, 1, 1).unwrap().common_era_year(), 1957);
    }

    #[test]
    fn the_new_year_moves_to_the_twenty_first_in_leap_years() {
        assert_eq!(new_year(1946), gregorian::to_fixed(2024, 3, 21)); // 2024 leap
        assert_eq!(new_year(1947), gregorian::to_fixed(2025, 3, 22));
        assert_eq!(new_year(1948), gregorian::to_fixed(2026, 3, 22));
        assert_eq!(new_year(1922), gregorian::to_fixed(2000, 3, 21)); // 2000 leap
        assert_eq!(new_year(1822), gregorian::to_fixed(1900, 3, 22)); // 1900 not
    }

    #[test]
    fn the_year_always_ends_the_day_before_the_next_one_starts() {
        for year in MIN_YEAR..MAX_YEAR {
            let start = new_year(year).unwrap();
            let next = new_year(year + 1).unwrap();
            assert_eq!(
                next.0 - start.0,
                i64::from(days_in_year(year)),
                "Śaka year {year}"
            );
            let last = to_fixed(year, 12, 30).unwrap();
            assert_eq!(last.0 + 1, next.0, "Śaka year {year}");
        }
    }

    #[test]
    fn chaitra_gains_a_day_in_leap_years() {
        assert!(is_leap_year(1946)); // opens in 2024
        assert_eq!(days_in_month(1946, 1), Some(31));
        assert!(!is_leap_year(1947));
        assert_eq!(days_in_month(1947, 1), Some(30));
        assert_eq!(days_in_month(1947, 6), Some(31));
        assert_eq!(days_in_month(1947, 7), Some(30));
        assert_eq!(days_in_month(1947, 13), None);
        assert_eq!(to_fixed(1947, 1, 31), Err(CalendarError::DayOutOfRange));
        assert!(to_fixed(1946, 1, 31).is_ok());
    }

    #[test]
    fn independence_day_and_republic_day_carry_the_published_saka_dates() {
        // The Gazette prints both: 15 August is 24 Sravana and 26 January is
        // 6 Magha in an ordinary year.
        assert_eq!(
            from_fixed(gregorian::to_fixed(2025, 8, 15).unwrap()),
            Ok((1947, 5, 24))
        );
        assert_eq!(
            from_fixed(gregorian::to_fixed(2025, 1, 26).unwrap()),
            Ok((1946, 11, 6))
        );
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (EARLIEST.0..=LATEST.0).step_by(103) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn every_single_day_of_a_leap_cycle_round_trips() {
        let start = new_year(1900).unwrap().0;
        let end = new_year(1950).unwrap().0;
        for rd in start..end {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = IndianCalendar;
        for rd in (EARLIEST.0..=LATEST.0).step_by(887) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId("indian"));
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
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            IndianCalendar.from_fields(&DateFields::ymd(1947, 1, 1).with_era("AD")),
            Err(CalendarError::UnknownEra)
        );
    }
}
