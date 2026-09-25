//! The ancient Armenian calendar.
//!
//! The same wandering year as [`crate::egyptian`] — twelve months of thirty
//! days, five *aweleacʿ* days, and no intercalation — moved to a different
//! epoch. The Armenian era begins on 11 July AD 552 in the Julian calendar
//! ([`EPOCH`]), the year the Armenian church broke with the Byzantine
//! computus and started its own reckoning.
//!
//! Because the year is 365 days flat, the calendar wanders against the
//! seasons exactly as the Egyptian one does, and an Armenian date is a fixed
//! offset from the Egyptian date with the same year and month number. This
//! module implements the original wandering form; Yovhannēs Sarkawag's
//! fixed year of 1084 is [`crate::armenian_fixed`], and modern Armenia uses
//! the Gregorian calendar.
//!
//! The month names in Armenian script are [`MONTHS`], declared with the
//! calendar's shape; transliterations are a locale's and belong to `hc-i18n`.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::common;

/// The fixed day of 1 Nawasardi 1, which is 552-07-11 in the Julian
/// calendar.
pub const EPOCH: Rd = Rd(201_443);

/// The era code of the Armenian era.
pub const ERA: &str = "Armenian";

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 999_999;

/// The length of every Armenian year, without exception.
pub const DAYS_IN_YEAR: u16 = 365;

/// The number of days in `month`, or `None` when `month` is not in `1..=13`.
#[must_use]
pub const fn days_in_month(month: u8) -> Option<u8> {
    common::wandering_days_in_month(month)
}

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "The era's first day, 1 Nawasardi 1 = 11 July 552 Julian, when the Armenian church \
    began its own reckoning, as this module states it; superseded by Sarkawag's fixed \
    year from 11 August 1084, as `armenian_fixed` states it";

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = Rd(common::wandering_to_fixed(EPOCH.0, MIN_YEAR, 1, 1));

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(common::wandering_to_fixed(EPOCH.0, MAX_YEAR + 1, 1, 1) - 1);

/// The fixed day of an Armenian date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    match common::check_day(day, days_in_month(month)) {
        Err(error) => Err(error),
        Ok(()) => Ok(Rd(common::wandering_to_fixed(EPOCH.0, year, month, day))),
    }
}

/// The Armenian year, month and day of a fixed day.
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
    Ok(common::wandering_from_fixed(EPOCH.0, rd.0))
}

/// An ancient Armenian date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArmenianDate {
    /// The year of the Armenian era, counting from 1.
    pub year: i64,
    /// The month, 1 through 13; month 13 holds the five *aweleacʿ* days.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl ArmenianDate {
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

    /// Whether this date is one of the five intercalary days.
    #[must_use]
    pub const fn is_epagomenal(self) -> bool {
        self.month == 13
    }
}

/// The ancient Armenian calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ArmenianCalendar;

/// The thirteen months in Armenian script and classical orthography,
/// Nawasard first. The thirteenth is Aweleacʿ, the five epagomenal days.
///
/// Source: the months table of Wikipedia, "Armenian calendar", retrieved
/// 2026-09-22, which prints them in lowercase as here. The
/// Hübschmann-Meillet-Benveniste transliterations are a locale's and live
/// in `hc-i18n`. [`crate::armenian_fixed`] shares this table.
pub const MONTHS: [&str; 13] = [
    "նաւասարդ",
    "հոռի",
    "սահմի",
    "տրէ",
    "քաղոց",
    "արաց",
    "մեհեկան",
    "արեգ",
    "ահեկան",
    "մարերի",
    "մարգաց",
    "հրոտից",
    "աւելեաց",
];

/// Thirteen named months and the seven-day week.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for ArmenianCalendar {
    type Date = ArmenianDate;

    /// From the era's first day, 11 July 552, until the day before Sarkawag's
    /// fixed year took effect on 11 August 1084. The wandering year lingered
    /// in use beside the fixed one afterwards; no source read dates that, so
    /// it is not carried.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::between(
            EPOCH,
            hc_calendar::Rd(crate::armenian_fixed::REFORM.0 - 1),
            USAGE_SOURCE,
        )
    }

    /// Thirteen months, named in Armenian script, and the seven-day week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    /// Never: a wandering year of 365 days intercalates nothing.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(false)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("armenian"),
            english_name: "Armenian",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &["hy"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(ArmenianDate { year, month, day })
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
        ArmenianDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{egyptian, julian};

    #[test]
    fn the_epoch_is_the_eleventh_of_july_552() {
        assert_eq!(julian::to_fixed(552, 7, 11), Ok(EPOCH));
        assert_eq!(to_fixed(1, 1, 1), Ok(EPOCH));
        assert_eq!(from_fixed(EPOCH), Ok((1, 1, 1)));
    }

    #[test]
    fn only_the_epoch_separates_this_calendar_from_the_egyptian_one() {
        let offset = EPOCH.0 - egyptian::EPOCH.0;
        for year in (1..5_000).step_by(11) {
            for month in [1u8, 7, 13] {
                let armenian = to_fixed(year, month, 1).unwrap();
                let egyptian = egyptian::to_fixed(year, month, 1).unwrap();
                assert_eq!(armenian.0 - egyptian.0, offset, "{year}-{month}");
            }
        }
        assert_eq!(offset, 474_230);
    }

    #[test]
    fn every_year_is_exactly_three_hundred_and_sixty_five_days() {
        for year in (1..10_000).step_by(17) {
            let start = to_fixed(year, 1, 1).unwrap();
            let next = to_fixed(year + 1, 1, 1).unwrap();
            assert_eq!(next.0 - start.0, i64::from(DAYS_IN_YEAR), "year {year}");
        }
    }

    #[test]
    fn the_new_year_wanders_backwards_through_the_julian_year() {
        // Four Armenian years are 1 460 days, one short of four Julian
        // years, so 1 Nawasardi moves back one Julian date every four years.
        let (_, first_month, first_day) = julian::from_fixed(to_fixed(1, 1, 1).unwrap()).unwrap();
        let (_, later_month, later_day) = julian::from_fixed(to_fixed(5, 1, 1).unwrap()).unwrap();
        assert_eq!((first_month, first_day), (7, 11));
        assert_eq!((later_month, later_day), (7, 10));
    }

    #[test]
    fn the_thirteenth_month_holds_exactly_five_days() {
        assert_eq!(days_in_month(13), Some(5));
        assert_eq!(days_in_month(14), None);
        assert!(to_fixed(1, 13, 5).is_ok());
        assert_eq!(to_fixed(1, 13, 6), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(1, 0, 1), Err(CalendarError::MonthOutOfRange));
        assert!(ArmenianDate::new(1, 13, 1).unwrap().is_epagomenal());
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (EARLIEST.0..=1_200_000).step_by(19) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = ArmenianCalendar;
        for rd in (EARLIEST.0..=900_000).step_by(499) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId("armenian"));
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
            ArmenianCalendar.from_fields(&DateFields::ymd(1, 1, 1).with_era("AD")),
            Err(CalendarError::UnknownEra)
        );
    }
}
