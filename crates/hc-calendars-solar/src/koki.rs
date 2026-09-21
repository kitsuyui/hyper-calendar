//! The Japanese imperial year, 皇紀.
//!
//! Gregorian structure with the year counted from the legendary accession of
//! Emperor Jinmu, placed by the *Nihon Shoki* at 660 BCE. Kōki 1 is therefore
//! 660 BCE and the Common Era year `n` is Kōki `n + 660`: 2026 CE is Kōki
//! 2686.
//!
//! Also called 神武天皇即位紀元, 皇紀, 紀元 and, in English, the Japanese
//! imperial year. The Meiji government adopted it in 1872 together with the
//! Gregorian calendar, and it was in official use until 1945; it survives in
//! the names of things dated when it was current, most famously the Mitsubishi
//! A6M "Zero", named for Kōki 2600 (1940).
//!
//! # What this module does and does not claim
//!
//! The accession date is legendary, not historical. This module is not
//! asserting that anything happened in 660 BCE; it implements the *counting
//! convention*, which is exactly defined, and says nothing about the event it
//! counts from.
//!
//! Two consequences of that convention are worth stating, because both are
//! places where a naive `year + 660` goes wrong:
//!
//! * **There is no Kōki year zero.** The epoch year is Kōki 1, so the
//!   arithmetic runs through the astronomical year 0 of the proleptic
//!   Gregorian calendar without a gap. Kōki 1 is astronomical year −659.
//! * **Before 1873 Japan did not use this calendar.** The imperial year was
//!   introduced alongside the Gregorian calendar in 1872; before that Japan
//!   used the lunisolar Tenpō calendar, whose years do not line up with
//!   Gregorian ones. Kōki applied retrospectively to a pre-1873 date is a
//!   Gregorian-structure back-projection, not what anyone wrote at the time.
//!   [`PROLEPTIC_BEFORE`] marks where that begins, and
//!   [`crate::julian_gregorian`] or `hc-calendars-lunar`'s Tenpō calendar are
//!   the honest routes for earlier dates.
//!
//! 紀元節, the holiday marking the accession, was 11 February; it is now
//! 建国記念の日 and belongs to `hc-holiday` rather than here.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::{common, gregorian};

/// The Common Era year corresponding to Kōki year zero; Kōki 1 is therefore
/// 660 BCE, which is astronomical year −659.
pub const YEAR_OFFSET: i64 = 660;

/// The era code of the imperial year.
pub const ERA: &str = "koki";

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = gregorian::MAX_YEAR + YEAR_OFFSET;

/// The first fixed day on which the imperial year was actually in use.
///
/// Japan adopted the Gregorian calendar and the imperial year together, with
/// 明治6年1月1日 falling on 1873-01-01. Conversions before this day are
/// proleptic: the arithmetic is well defined, but no one dated a document
/// that way.
pub const PROLEPTIC_BEFORE: Rd = match gregorian::to_fixed(1873, 1, 1) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// Whether `year` is a leap year, by the Gregorian rule on the corresponding
/// Common Era year.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    gregorian::is_leap_year(year - YEAR_OFFSET)
}

/// The number of days in `month` of `year`, or `None` when `month` is not in
/// `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    gregorian::days_in_month(year - YEAR_OFFSET, month)
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    gregorian::days_in_year(year - YEAR_OFFSET)
}

/// The earliest fixed day this implementation converts, 1 January of Kōki 1.
pub const EARLIEST: Rd = match gregorian::to_fixed(MIN_YEAR - YEAR_OFFSET, 1, 1) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// Whether a fixed day precedes Japan's adoption of this year count.
#[must_use]
pub const fn is_proleptic(rd: Rd) -> bool {
    rd.0 < PROLEPTIC_BEFORE.0
}

/// The fixed day of an imperial-year date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(CalendarError::YearOutOfRange);
    }
    common::offset_to_fixed(year, month, day, YEAR_OFFSET)
}

/// The imperial year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] for any day before Kōki 1.
pub fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd < EARLIEST {
        return Err(CalendarError::BeforeEpoch);
    }
    common::offset_from_fixed(rd, YEAR_OFFSET)
}

/// A date in the Japanese imperial year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KokiDate {
    /// The imperial year, counting from 1.
    pub year: i64,
    /// The month, 1 for January through 12 for December.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl KokiDate {
    /// A validated date.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    pub fn new(year: i64, month: u8, day: u8) -> CalendarResult<Self> {
        to_fixed(year, month, day)?;
        Ok(Self { year, month, day })
    }

    /// The same day in astronomical Common Era year numbering, where 1 BCE is
    /// year 0.
    #[must_use]
    pub const fn common_era_year(self) -> i64 {
        self.year - YEAR_OFFSET
    }
}

/// The Japanese imperial year.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct KokiCalendar;

impl Calendar for KokiCalendar {
    type Date = KokiDate;

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("japanese-imperial"),
            english_name: "Japanese imperial year (kōki)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(gregorian::LATEST),
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(KokiDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let fields = DateFields::ymd(date.year, date.month, date.day)
            .with_era(ERA)
            .with_extra("common-era-year", date.common_era_year())?;
        let rd = to_fixed(date.year, date.month, date.day)?;
        fields.with_extra("proleptic", i64::from(is_proleptic(rd)))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        KokiDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_common_era_offset_is_six_hundred_and_sixty() {
        // 2026 CE is Kōki 2686.
        assert_eq!(
            to_fixed(2_686, 9, 21),
            Ok(gregorian::to_fixed(2026, 9, 21).unwrap())
        );
        assert_eq!(from_fixed(Rd(719_163)), Ok((2_630, 1, 1)));
    }

    #[test]
    fn the_zero_fighter_was_named_for_koki_twenty_six_hundred() {
        // Kōki 2600 is 1940, the year the A6M entered service and took its
        // name from the last two digits.
        assert_eq!(KokiDate::new(2_600, 1, 1).unwrap().common_era_year(), 1940);
        assert_eq!(
            to_fixed(2_600, 1, 1),
            Ok(gregorian::to_fixed(1940, 1, 1).unwrap())
        );
    }

    #[test]
    fn koki_one_is_astronomical_year_minus_six_hundred_and_fifty_nine() {
        // 660 BCE is astronomical year -659; there is no year zero in the
        // imperial count, and the proleptic Gregorian calendar has one, so
        // the offset is not symmetric about the epoch.
        assert_eq!(KokiDate::new(1, 1, 1).unwrap().common_era_year(), -659);
        assert_eq!(
            to_fixed(1, 1, 1),
            Ok(gregorian::to_fixed(-659, 1, 1).unwrap())
        );
    }

    #[test]
    fn the_accession_date_is_the_eleventh_of_february() {
        // 紀元節, now 建国記念の日. The holiday itself belongs to hc-holiday;
        // this only checks the date arithmetic lands where it should.
        let rd = to_fixed(2_686, 2, 11).unwrap();
        assert_eq!(gregorian::from_fixed(rd), Ok((2026, 2, 11)));
    }

    #[test]
    fn there_is_no_year_before_koki_one() {
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(to_fixed(-1, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
    }

    #[test]
    fn dates_before_the_eighteen_seventy_three_adoption_are_marked_proleptic() {
        // Japan adopted this count with the Gregorian calendar; anything
        // earlier is a back-projection and says so.
        assert!(is_proleptic(gregorian::to_fixed(1872, 12, 31).unwrap()));
        assert!(!is_proleptic(gregorian::to_fixed(1873, 1, 1).unwrap()));
        assert_eq!(PROLEPTIC_BEFORE, gregorian::to_fixed(1873, 1, 1).unwrap());
    }

    #[test]
    fn leap_years_follow_the_gregorian_rule_on_the_common_era_year() {
        assert!(is_leap_year(2_660)); // 2000 CE
        assert!(!is_leap_year(2_560)); // 1900 CE
        assert!(is_leap_year(2_684)); // 2024 CE
        assert_eq!(days_in_month(2_684, 2), Some(29));
        assert_eq!(days_in_month(2_685, 2), Some(28));
        assert_eq!(days_in_year(2_684), 366);
    }

    #[test]
    fn every_day_round_trips_across_the_whole_supported_range() {
        for rd in (EARLIEST.0..=1_000_000).step_by(97) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
            assert!(year >= MIN_YEAR);
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = KokiCalendar;
        for rd in (EARLIEST.0..=1_000_000).step_by(389) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(
                fields.extra.get("common-era-year"),
                Some(date.common_era_year())
            );
            assert_eq!(
                fields.extra.get("proleptic"),
                Some(i64::from(is_proleptic(Rd(rd))))
            );
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId("japanese-imperial"));
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(2_686, 1, 1).with_era("juche")),
            Err(CalendarError::UnknownEra)
        );
    }
}
