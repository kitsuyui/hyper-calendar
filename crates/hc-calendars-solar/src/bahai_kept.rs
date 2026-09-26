//! The Badíʿ (Bahá'í) calendar as kept: `bahai`.
//!
//! The rules and the table are written up, with the arithmetic and the
//! astronomical calendars, in `docs/systems/equinox-calendars.md` in the
//! repository.
//!
//! Two rules, one calendar. Until 171 BE the Bahá'í World Centre and the
//! communities of the West kept the arithmetic calendar of [`crate::bahai`]:
//! Naw-Rúz on 21 March, the year as long as the Gregorian year. From Naw-Rúz
//! 172 BE (21 March 2015) the calendar is unified worldwide on astronomical
//! rules — the year begins on the day whose sunset follows the March equinox
//! at Tehran, and Ayyám-i-Há is as long as it takes to reach the next one —
//! and the Bahá'í World Centre published the resulting dates for 172 to
//! 221 BE. This calendar is those two joined at 172 BE and nothing else:
//! before 172 BE it *is* the arithmetic calendar, from 172 BE it *is* the
//! published table, and after 221 BE it refuses, because no table exists
//! and this crate does no astronomy.
//!
//! # The table, and where it came from
//!
//! *Badíʿ dates 172 to 221 BE*, prepared by an ad hoc committee at the
//! Bahá'í World Centre using data provided by Her Majesty's Nautical Almanac
//! Office in the United Kingdom, with Tehran taken from WGS 84 (2014;
//! `bahai-library.com/pdf/uhj/uhj_bahai_dates_172-221.pdf`, retrieved
//! 2026-09-22). Two of its columns are carried: the day in March on which
//! Naw-Rúz falls, and the length of Ayyám-i-Há. They are redundant — a year
//! is 361 days plus Ayyám-i-Há — and the tests check that the two columns
//! agree row by row, which is the check that the transcription is right.
//! The document sets its digits in a font that extracts as combining marks;
//! they were decoded programmatically and the result cross-checked the same
//! way.
//!
//! The last row's Ayyám-i-Há (25–28 February 2065) fixes the end of 221 BE
//! at 19 March 2065, which is [`LATEST`].
//!
//! # What this is not
//!
//! Not the practice of Iran and the Middle East before 172 BE, where
//! Naw-Rúz followed the Iranian equinox day rather than 21 March. Not a
//! computation past 221 BE. Not the arithmetic rule continued past 171 BE,
//! which [`crate::bahai`] remains for anyone who wants it. The equinox rule
//! applied to any year at all is `bahai-astronomical` in
//! `hc-calendars-equinox`, whose tests reproduce this table.
//!
//! A Badíʿ day runs from sunset to sunset; a date here names the fixed day
//! the Badíʿ day *ends* in, as the table itself does.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::bahai::{self, AYYAM_I_HA, BahaiDate, EPOCH, GREGORIAN_YEAR_OFFSET, NINETEEN};
use crate::gregorian;

/// The identifier of the calendar as kept.
pub const ID: CalendarId = CalendarId("bahai");

/// The first year the Bahá'í World Centre's table covers, and the first year
/// of the unified calendar.
pub const FIRST_TABULATED_YEAR: i64 = 172;

/// The last year the table covers.
pub const LAST_TABULATED_YEAR: i64 = 221;

/// The number of tabulated years.
pub const TABULATED_YEARS: usize = 50;

/// The day in March on which Naw-Rúz falls, for 172 BE through 221 BE.
///
/// Column "Naw-Rúz, Gregorian equivalent" of the table.
pub const NAW_RUZ_MARCH_DAY: [u8; TABULATED_YEARS] = [
    21, 20, 20, 21, 21, 20, 20, 21, 21, 20, 20, 21, 21, 20, 20, 20, 21, 20, 20, 20, 21, 20, 20, 20,
    21, 20, 20, 20, 21, 20, 20, 20, 21, 20, 20, 20, 21, 20, 20, 20, 21, 20, 20, 20, 20, 20, 20, 20,
    20, 20,
];

/// The length of Ayyám-i-Há, four or five days, for 172 BE through 221 BE.
///
/// Column "Ayyám-i-Há" of the table. Redundant with [`NAW_RUZ_MARCH_DAY`]
/// by construction, and kept because the table gives both and their
/// agreement is the transcription check.
pub const AYYAM_I_HA_DAYS: [u8; TABULATED_YEARS] = [
    4, 4, 5, 4, 4, 4, 5, 4, 4, 4, 5, 4, 4, 4, 4, 5, 4, 4, 4, 5, 4, 4, 4, 5, 4, 4, 4, 5, 4, 4, 4, 5,
    4, 4, 4, 5, 4, 4, 4, 5, 4, 4, 4, 4, 5, 4, 4, 4, 5, 4,
];

/// The earliest year this calendar converts: the first year of the era.
pub const MIN_YEAR: i64 = bahai::MIN_YEAR;

/// The latest year this calendar converts: the last year of the table.
pub const MAX_YEAR: i64 = LAST_TABULATED_YEAR;

/// Whether `year` is one the table covers.
#[must_use]
pub const fn is_tabulated(year: i64) -> bool {
    year >= FIRST_TABULATED_YEAR && year <= LAST_TABULATED_YEAR
}

/// The length of Ayyám-i-Há in `year`, without validation.
const fn intercalary_raw(year: i64) -> i64 {
    if year < FIRST_TABULATED_YEAR {
        if bahai::is_leap_year(year) { 5 } else { 4 }
    } else if year <= LAST_TABULATED_YEAR {
        AYYAM_I_HA_DAYS[(year - FIRST_TABULATED_YEAR) as usize] as i64
    } else {
        // Unreachable inside this calendar's year bounds.
        0
    }
}

/// The fixed day of Naw-Rúz of `year`, without validation; also answers for
/// the year after the table, from the last row's Ayyám-i-Há.
const fn new_year_raw(year: i64) -> i64 {
    if year < FIRST_TABULATED_YEAR {
        bahai::new_year_raw(year)
    } else if year <= LAST_TABULATED_YEAR {
        let day = NAW_RUZ_MARCH_DAY[(year - FIRST_TABULATED_YEAR) as usize];
        match gregorian::to_fixed(year + GREGORIAN_YEAR_OFFSET, 3, day) {
            Ok(rd) => rd.0,
            // Unreachable: every entry is a day in March.
            Err(_) => 0,
        }
    } else {
        let last = LAST_TABULATED_YEAR;
        let day = NAW_RUZ_MARCH_DAY[(last - FIRST_TABULATED_YEAR) as usize];
        let start = match gregorian::to_fixed(last + GREGORIAN_YEAR_OFFSET, 3, day) {
            Ok(rd) => rd.0,
            Err(_) => 0,
        };
        start + 18 * NINETEEN + intercalary_raw(last) + NINETEEN
    }
}

/// The fixed day of Naw-Rúz, the first day of `year`.
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

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "The calendar as kept from the first Naw-Rúz, 21 March 1844: by the arithmetic rule to \
    171 BE and by the Bahá'í World Centre's *Badíʿ dates 172 to 221 BE* (2014) since";

/// The earliest fixed day this calendar converts: 21 March 1844.
pub const EARLIEST: Rd = EPOCH;

/// The latest fixed day this calendar converts: the last day of 221 BE,
/// 19 March 2065.
pub const LATEST: Rd = Rd(new_year_raw(LAST_TABULATED_YEAR + 1) - 1);

/// The number of days in `year`, or `None` outside [`MIN_YEAR`]..=[`MAX_YEAR`].
#[must_use]
pub const fn days_in_year(year: i64) -> Option<u16> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return None;
    }
    Some((18 * NINETEEN + intercalary_raw(year) + NINETEEN) as u16)
}

/// The number of days in `month` of `year`, or `None` when the year is out of
/// range or `month` is not in `0..=19`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return None;
    }
    match month {
        AYYAM_I_HA => Some(intercalary_raw(year) as u8),
        1..=19 => Some(19),
        _ => None,
    }
}

/// The fixed day of a Badíʿ date.
///
/// `month` is `1..=19`, or [`AYYAM_I_HA`] for the intercalary days.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`] — including every year after the table —
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    match crate::common::check_day(day, days_in_month(year, month)) {
        Err(error) => Err(error),
        Ok(()) => Ok(Rd(new_year_raw(year)
            + bahai::days_before_month_with(intercalary_raw(year), month)
            + day as i64
            - 1)),
    }
}

/// The Badíʿ year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EARLIEST`]..=[`LATEST`].
/// The table is the calendar from 172 BE, so there is nothing to return
/// after it.
pub fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd < EARLIEST {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd > LATEST {
        return Err(CalendarError::AfterSupportedRange);
    }
    let (gregorian_year, _, _) = gregorian::from_fixed(rd)?;
    let mut year = gregorian_year - GREGORIAN_YEAR_OFFSET;
    if year > MAX_YEAR {
        year = MAX_YEAR;
    }
    if rd.0 < new_year_raw(year) {
        year -= 1;
    }
    let day_of_year = rd.0 - new_year_raw(year);
    let (month, day) = bahai::split_day_of_year(day_of_year, intercalary_raw(year));
    Ok((year, month, day))
}

/// The Badíʿ calendar as kept.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BahaiCalendar;

impl Calendar for BahaiCalendar {
    type Date = BahaiDate;

    /// Kept since the first Naw-Rúz, 21 March 1844, under one rule to 171 BE
    /// and the published table since; the range ends where the table does,
    /// which is a limit of the data and not of the calendar.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::since(EARLIEST, USAGE_SOURCE)
    }

    /// Nineteen months of nineteen days, and a seven-day week; Ayyám-i-Há is
    /// not a position in the month cycle.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        bahai::SHAPE
    }

    /// A year whose Ayyám-i-Há has five days, as the table records it.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        days_in_month(year, AYYAM_I_HA)
            .map(|days| days == 5)
            .ok_or(CalendarError::YearOutOfRange)
    }

    /// The Bahá'í day begins at sunset, and a date names the Gregorian day
    /// the Badíʿ day ends in, as the Bahá'í World Centre's table of dates
    /// does (`docs/systems/equinox-calendars.md`, `bwc-badi-dates`).
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunset(hc_calendar::DayNaming::ByEnd)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Badíʿ",
            year_kind: YearKind::EpochForward,
            // Ayyám-i-Há is carried as an intercalary repetition of month
            // 18, which is what makes this true.
            has_leap_months: true,
            // The table was computed from the Tehran equinox, but this
            // implementation performs no astronomy: it reads a published
            // table, which is what makes it exact.
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &["fa", "ar"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(BahaiDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        bahai::fields_of(date)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let (year, month, day) = bahai::ordinal_from_fields(fields)?;
        match to_fixed(year, month, day) {
            Ok(_) => Ok(BahaiDate { year, month, day }),
            Err(error) => Err(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn the_two_columns_of_the_table_agree_row_by_row() {
        // Naw-Rúz to Naw-Rúz is 361 days plus Ayyám-i-Há: the transcription
        // check the module doc promises.
        for index in 0..TABULATED_YEARS {
            let year = FIRST_TABULATED_YEAR + index as i64;
            let length = new_year_raw(year + 1) - new_year_raw(year);
            assert_eq!(
                length,
                18 * NINETEEN + i64::from(AYYAM_I_HA_DAYS[index]) + NINETEEN,
                "{year} BE"
            );
            assert!(matches!(NAW_RUZ_MARCH_DAY[index], 20 | 21), "{year} BE");
            assert!(matches!(AYYAM_I_HA_DAYS[index], 4 | 5), "{year} BE");
        }
    }

    #[test]
    fn the_table_is_the_calendar_from_172_be() {
        // Rows of the World Centre's table.
        assert_eq!(new_year(172), Ok(ymd(2015, 3, 21)));
        assert_eq!(new_year(173), Ok(ymd(2016, 3, 20)));
        assert_eq!(new_year(181), Ok(ymd(2024, 3, 20)));
        assert_eq!(new_year(182), Ok(ymd(2025, 3, 20)));
        assert_eq!(new_year(183), Ok(ymd(2026, 3, 21)));
        assert_eq!(new_year(221), Ok(ymd(2064, 3, 20)));
        // 182 BE: Ayyám-i-Há 25 February – 1 March 2026, five days, and the
        // Fast from 2 March.
        assert_eq!(to_fixed(182, AYYAM_I_HA, 1), Ok(ymd(2026, 2, 25)));
        assert_eq!(to_fixed(182, AYYAM_I_HA, 5), Ok(ymd(2026, 3, 1)));
        assert_eq!(to_fixed(182, 19, 1), Ok(ymd(2026, 3, 2)));
        assert_eq!(days_in_month(182, AYYAM_I_HA), Some(5));
        // 183 BE: four days, 26 February – 1 March 2027.
        assert_eq!(to_fixed(183, AYYAM_I_HA, 1), Ok(ymd(2027, 2, 26)));
        assert_eq!(to_fixed(183, 19, 1), Ok(ymd(2027, 3, 2)));
        assert_eq!(days_in_month(183, AYYAM_I_HA), Some(4));
        // The last row fixes the end of the calendar.
        assert_eq!(to_fixed(221, AYYAM_I_HA, 1), Ok(ymd(2065, 2, 25)));
        assert_eq!(to_fixed(221, 19, 19), Ok(ymd(2065, 3, 19)));
        assert_eq!(LATEST, ymd(2065, 3, 19));
    }

    #[test]
    fn before_172_be_it_is_the_arithmetic_calendar_and_the_join_is_seamless() {
        let join = bahai::new_year(FIRST_TABULATED_YEAR).unwrap();
        assert_eq!(new_year(FIRST_TABULATED_YEAR), Ok(join));
        for rd in (EARLIEST.0..join.0).step_by(7) {
            assert_eq!(from_fixed(Rd(rd)), bahai::from_fixed(Rd(rd)), "rd {rd}");
        }
        assert_eq!(from_fixed(Rd(join.0 - 1)), Ok((171, 19, 19)));
        assert_eq!(from_fixed(join), Ok((172, 1, 1)));
        // Where the two part: 173 BE begins on 20 March 2016 as kept, and on
        // the 21st by the old arithmetic.
        assert_eq!(new_year(173), Ok(ymd(2016, 3, 20)));
        assert_eq!(bahai::new_year(173), Ok(ymd(2016, 3, 21)));
    }

    #[test]
    fn every_single_day_round_trips() {
        for rd in EARLIEST.0..=LATEST.0 {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = BahaiCalendar;
        for rd in (EARLIEST.0..=LATEST.0).step_by(367) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(bahai::ERA));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, ID);
        assert!(calendar.meta().has_leap_months);
        assert!(!calendar.meta().is_astronomical);
    }

    #[test]
    fn nothing_is_answered_past_the_table() {
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(to_fixed(222, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(new_year(222), Err(CalendarError::YearOutOfRange));
        assert_eq!(days_in_year(222), None);
        assert_eq!(days_in_month(222, 1), None);
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            to_fixed(182, AYYAM_I_HA, 6),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            to_fixed(183, AYYAM_I_HA, 5),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(to_fixed(183, 20, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(
            BahaiCalendar.from_fields(&DateFields::ymd_leap_month(183, 5, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert!(!is_tabulated(171));
        assert!(is_tabulated(172));
        assert!(is_tabulated(221));
        assert!(!is_tabulated(222));
    }
}
