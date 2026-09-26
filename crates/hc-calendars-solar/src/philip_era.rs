//! The Era of Philip: the Egyptian wandering year of Ptolemy's *Handy
//! Tables*.
//!
//! The *Almagest* counts Egyptian years from the era of Nabonassar, noon on
//! 26 February 747 BC, which is [`crate::egyptian`]'s epoch. The *Handy
//! Tables* count the same years from the Era of Philip, noon on 12 November
//! 324 BC (−323), the first of Thoth after the death of Alexander, so a day
//! has the same month and day in both and a year number 424 less in this
//! one: 1 Thoth of Philip 1 is 1 Thoth of Nabonassar 425. The two epochs,
//! "noon, −323 November 12" and "noon, −746 February 26", are Chabás's
//! review of Tihon and Mercier's edition, *Aestimatio* 10 (2013) 106–109
//! (`chabas2013`), read 2026-09-26; the offset of 424 years is derived from
//! them and tested, not read. The system document is
//! `docs/systems/era-counts.md` in the repository.
//!
//! The days are [`crate::egyptian`]'s arithmetic, the crate's shared
//! wandering year, under the Egyptian module's Greek month names. Like that
//! module, this one keeps the civil midnight: the astronomers' noon epoch
//! names the day, and a caller who needs Ptolemy's day from noon applies
//! it. Days before the era are refused rather than numbered backwards; they
//! are Nabonassar's.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::{common, egyptian};

/// The Nabonassar year that is year 1 of the Era of Philip, less one.
pub const NABONASSAR_OFFSET: i64 = 424;

/// The fixed day of 1 Thoth of year 1, 12 November 324 BC (−323) in the
/// proleptic Julian calendar.
pub const EPOCH: Rd = Rd(common::wandering_to_fixed(
    egyptian::EPOCH.0,
    NABONASSAR_OFFSET + 1,
    1,
    1,
));

/// The era code of the Era of Philip.
pub const ERA: &str = "philip";

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts, where
/// [`crate::egyptian`]'s range ends.
pub const MAX_YEAR: i64 = egyptian::MAX_YEAR - NABONASSAR_OFFSET;

/// The fixed day of an Era of Philip date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    egyptian::to_fixed(year + NABONASSAR_OFFSET, month, day)
}

/// The Era of Philip year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] before [`EPOCH`], and
/// [`CalendarError::AfterSupportedRange`] after [`egyptian::LATEST`].
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd.0 < EPOCH.0 {
        return Err(CalendarError::BeforeEpoch);
    }
    match egyptian::from_fixed(rd) {
        Ok((year, month, day)) => Ok((year - NABONASSAR_OFFSET, month, day)),
        Err(error) => Err(error),
    }
}

/// An Era of Philip date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhilipEraDate {
    /// The year of the Era of Philip, from 1.
    pub year: i64,
    /// The month, 1 (Thoth) through 13, the five epagomenal days.
    pub month: u8,
    /// The day of the month, from 1.
    pub day: u8,
}

impl PhilipEraDate {
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

    /// The same day's year of the era of Nabonassar.
    #[must_use]
    pub const fn nabonassar_year(self) -> i64 {
        self.year + NABONASSAR_OFFSET
    }
}

/// The Era of Philip.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PhilipEraCalendar;

/// The Egyptian months and the seven-day week.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &egyptian::MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for PhilipEraCalendar {
    type Date = PhilipEraDate;

    /// Unrecorded: the era is the *Handy Tables*' epoch, and no source read
    /// bounds the centuries astronomers counted from it.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    /// Never: the wandering year intercalates nothing.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(false)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("philip-era"),
            english_name: "Era of Philip (Egyptian)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EPOCH),
            latest: Some(egyptian::LATEST),
            native_locales: &["grc"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(PhilipEraDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.month, date.day)
            .with_era(ERA)
            .with_extra("nabonassar-year", date.nabonassar_year())
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        PhilipEraDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::julian;

    #[test]
    fn the_epoch_is_the_twelfth_of_november_324_bc() {
        // Chabás: the Era of Philip from noon, -323 November 12, and the
        // era of Nabonassar from noon, -746 February 26.
        assert_eq!(julian::from_fixed(EPOCH), Ok((-323, 11, 12)));
        assert_eq!(julian::from_fixed(egyptian::EPOCH), Ok((-746, 2, 26)));
        assert_eq!(from_fixed(EPOCH), Ok((1, 1, 1)));
        assert_eq!(egyptian::from_fixed(EPOCH), Ok((425, 1, 1)));
    }

    #[test]
    fn a_day_keeps_its_egyptian_month_and_day_and_loses_424_years() {
        for rd in (EPOCH.0..EPOCH.0 + 800_000).step_by(89) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(
                egyptian::from_fixed(Rd(rd)),
                Ok((year + 424, month, day)),
                "rd {rd}"
            );
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)));
        }
        // Censorinus's 1 Thoth of Nabonassar 887, 20 July AD 139, as
        // Richards gives it (`richards2013`, section 15.2.1), is 1 Thoth 463.
        let censorinus = julian::to_fixed(139, 7, 20).unwrap();
        assert_eq!(censorinus.to_julian_day_number(), 1_772_028);
        assert_eq!(from_fixed(censorinus), Ok((463, 1, 1)));
    }

    #[test]
    fn the_days_before_the_era_are_refused() {
        assert_eq!(from_fixed(Rd(EPOCH.0 - 1)), Err(CalendarError::BeforeEpoch));
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(to_fixed(1, 13, 6), Err(CalendarError::DayOutOfRange));
        assert_eq!(
            from_fixed(Rd(egyptian::LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(
            PhilipEraCalendar.from_fields(&DateFields::ymd(1, 1, 1).with_era("nabonassar")),
            Err(CalendarError::UnknownEra)
        );
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = PhilipEraCalendar;
        for rd in (EPOCH.0..=900_000).step_by(523) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(fields.extra.get("nabonassar-year"), Some(date.year + 424));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
        }
        assert_eq!(calendar.is_leap_year(1), Ok(false));
    }
}
