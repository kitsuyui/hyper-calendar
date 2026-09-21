//! The Ethiopian calendar.
//!
//! Structurally the same calendar as [`crate::coptic`] — twelve months of
//! thirty days, then Pagumen of five or six — differing only in where the
//! count starts. The Ethiopian era of the Incarnation, *Amätä Məḥrät*, puts
//! the birth of Christ seven to eight years later than the Dionysian
//! reckoning the Gregorian calendar uses, so 1 Mäskäräm 1 is 29 August AD 8
//! in the Julian calendar, [`EPOCH`], and the Ethiopian year number today is
//! seven or eight less than the Gregorian one depending on the month.
//!
//! That one-number difference from the Coptic calendar is the whole point of
//! the data/algorithm split: the arithmetic lives in
//! [`crate::common`]-shaped helpers, and each calendar contributes an epoch.
//!
//! The older *Amätä Aläm* (Year of the World) era, still used in liturgical
//! contexts, runs 5 500 years ahead of the era of the Incarnation and is
//! reachable through [`ERA_WORLD`].
//!
//! Month names belong to `hc-i18n`, not here.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::common;

/// The fixed day of 1 Mäskäräm 1, which is 8-08-29 in the Julian calendar.
pub const EPOCH: Rd = Rd(2_796);

/// The era code of the era of the Incarnation, *Amätä Məḥrät*.
pub const ERA_INCARNATION: &str = "AM";

/// The era code of the era of the World, *Amätä Aläm*.
pub const ERA_WORLD: &str = "AA";

/// How far the era of the World runs ahead of the era of the Incarnation.
pub const WORLD_ERA_OFFSET: i64 = 5_500;

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 999_999;

/// Whether `year` is an Ethiopian leap year, so that Pagumen has six days.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    common::coptic_style_is_leap(year)
}

/// The number of days in `month` of `year`, or `None` when `month` is not in
/// `1..=13`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    common::coptic_style_days_in_month(year, month)
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = Rd(common::coptic_style_to_fixed(EPOCH.0, MIN_YEAR, 1, 1));

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(common::coptic_style_to_fixed(EPOCH.0, MAX_YEAR + 1, 1, 1) - 1);

/// The fixed day of an Ethiopian date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    match common::check_day(day, days_in_month(year, month)) {
        Err(error) => Err(error),
        Ok(()) => Ok(Rd(common::coptic_style_to_fixed(EPOCH.0, year, month, day))),
    }
}

/// The Ethiopian year, month and day of a fixed day.
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
    Ok(common::coptic_style_from_fixed(EPOCH.0, rd.0))
}

/// An Ethiopian date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EthiopicDate {
    /// The year of the era of the Incarnation, counting from 1.
    pub year: i64,
    /// The month, 1 through 13; month 13 is Pagumen.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl EthiopicDate {
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

    /// The same year counted in the era of the World.
    #[must_use]
    pub const fn world_era_year(self) -> i64 {
        self.year + WORLD_ERA_OFFSET
    }

    /// Whether this date falls in Pagumen.
    #[must_use]
    pub const fn is_pagumen(self) -> bool {
        self.month == 13
    }
}

/// The Ethiopian calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EthiopicCalendar;

impl Calendar for EthiopicCalendar {
    type Date = EthiopicDate;

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("ethiopic"),
            english_name: "Ethiopian",
            year_kind: YearKind::EraRelative,
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
        Ok(EthiopicDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.month, date.day)
            .with_era(ERA_INCARNATION)
            .with_extra("amete-alem-year", date.world_era_year())
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let day = fields.require_day()?;
        let year = match fields.era {
            None | Some(ERA_INCARNATION) => fields.year,
            Some(ERA_WORLD) => fields.year - WORLD_ERA_OFFSET,
            Some(_) => return Err(CalendarError::UnknownEra),
        };
        EthiopicDate::new(year, month.ordinal, day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{coptic, gregorian, julian};

    #[test]
    fn the_epoch_is_the_ethiopian_incarnation_year() {
        // 1 Mäskäräm 1 is 29 August AD 8 in the Julian calendar.
        assert_eq!(julian::to_fixed(8, 8, 29), Ok(EPOCH));
        assert_eq!(to_fixed(1, 1, 1), Ok(EPOCH));
        assert_eq!(from_fixed(EPOCH), Ok((1, 1, 1)));
    }

    #[test]
    fn enkutatash_falls_on_the_eleventh_or_twelfth_of_september() {
        // The new year sits on 11 September, and on the 12th in the year
        // before a Gregorian leap year, because the Ethiopian leap day comes
        // six months before the Gregorian one. Recent years, as published:
        // 2015 E.C. opened on 11 September 2022, 2016 on 12 September 2023,
        // 2017 on 11 September 2024 and 2018 on 11 September 2025.
        for (ethiopian_year, expected) in [
            (2015, (2022, 9, 11)),
            (2016, (2023, 9, 12)),
            (2017, (2024, 9, 11)),
            (2018, (2025, 9, 11)),
        ] {
            let new_year = to_fixed(ethiopian_year, 1, 1).unwrap();
            assert_eq!(gregorian::from_fixed(new_year), Ok(expected));
        }
        // The Ethiopian millennium therefore fell on 12 September 2007,
        // the celebrations having begun the evening before.
        assert_eq!(
            gregorian::from_fixed(to_fixed(2000, 1, 1).unwrap()),
            Ok((2007, 9, 12))
        );
    }

    #[test]
    fn the_ethiopian_year_runs_seven_or_eight_behind_the_gregorian_one() {
        let new_year = to_fixed(2018, 1, 1).unwrap();
        let (gregorian_year, month, _) = gregorian::from_fixed(new_year).unwrap();
        assert_eq!((gregorian_year, month), (2025, 9));
    }

    #[test]
    fn only_the_epoch_separates_this_calendar_from_the_coptic_one() {
        // Same structure, different starting point: the difference between
        // the two fixed days is exactly the difference between the epochs.
        let offset = coptic::EPOCH.0 - EPOCH.0;
        for year in (1..3_000).step_by(7) {
            for month in [1u8, 6, 12, 13] {
                let ethiopic = to_fixed(year, month, 1).unwrap();
                let coptic = coptic::to_fixed(year, month, 1).unwrap();
                assert_eq!(coptic.0 - ethiopic.0, offset, "{year}-{month}");
            }
        }
        assert_eq!(offset, 100_809);
    }

    #[test]
    fn pagumen_gains_a_sixth_day_every_fourth_year() {
        assert!(is_leap_year(3));
        assert!(!is_leap_year(4));
        assert_eq!(days_in_month(3, 13), Some(6));
        assert_eq!(days_in_month(4, 13), Some(5));
        assert_eq!(days_in_year(3), 366);
        assert!(EthiopicDate::new(3, 13, 6).unwrap().is_pagumen());
        assert_eq!(to_fixed(4, 13, 6), Err(CalendarError::DayOutOfRange));
    }

    #[test]
    fn the_world_era_runs_five_thousand_five_hundred_years_ahead() {
        let date = EthiopicDate::new(2018, 1, 1).unwrap();
        assert_eq!(date.world_era_year(), 7518);
        let calendar = EthiopicCalendar;
        let fields = DateFields::ymd(7518, 1, 1).with_era(ERA_WORLD);
        assert_eq!(calendar.from_fields(&fields), Ok(date));
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(1, 1, 1).with_era("AD")),
            Err(CalendarError::UnknownEra)
        );
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (EARLIEST.0..=1_200_000).step_by(31) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn every_single_day_of_a_leap_cycle_round_trips() {
        let start = to_fixed(1, 1, 1).unwrap().0;
        let end = to_fixed(21, 1, 1).unwrap().0;
        for rd in start..end {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = EthiopicCalendar;
        for rd in (EARLIEST.0..=900_000).step_by(677) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(ERA_INCARNATION));
            assert_eq!(
                fields.extra.get("amete-alem-year"),
                Some(date.world_era_year())
            );
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId("ethiopic"));
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
    }
}
