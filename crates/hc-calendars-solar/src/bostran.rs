//! The Bostran era, the era of the Roman province of Arabia.
//!
//! Trajan made Arabia a province in AD 106, and its capital Bostra counted
//! years from the event: year 1 began on 1 Xanthikos, 22 March 106 in the
//! Julian calendar. The year is twelve months of thirty days under the
//! Macedonian names, Xanthikos first, and five epagomenal days "at the end
//! of the year", with "a sixth epagomenal day" in "years 2, 6, 10 etc.",
//! so that 1 Xanthikos stays on 22 March: year *N* runs from 22 March
//! of AD 105 + *N* to 21 March of the next year and holds the Julian leap
//! day exactly when *N* is 2 modulo 4. The quotations are Wikipedia,
//! "Bostran era" (`wikipedia-bostran-era`), retrieved 2026-09-26, which
//! cites Mercier 2001, not read; Grumel, "Eras, Historical"
//! (`grumel-eras-historical`), gives the point of departure as 22 March
//! AD 106. The month names are the Macedonian months in their order from
//! Xanthikos, as Wikipedia, "Ancient Macedonian calendar"
//! (`wikipedia-ancient-macedonian-calendar`), retrieved 2026-09-26, lists
//! and spells them. The system document is `docs/systems/era-counts.md` in
//! the repository, which says what a primary source would settle.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::{common, julian};

/// The fixed day of 1 Xanthikos of year 1, 22 March AD 106 in the
/// proleptic Julian calendar.
pub const EPOCH: Rd = match julian::to_fixed(106, 3, 22) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The era code of the Bostran era.
pub const ERA: &str = "bostran";

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 9_999;

/// The twelve Macedonian months from Xanthikos, and the epagomenal days as
/// a thirteenth position.
pub const MONTHS: [&str; 13] = [
    "Xanthikos",
    "Artemisios",
    "Daisios",
    "Panemos",
    "Loios",
    "Gorpiaios",
    "Hyperberetaios",
    "Dios",
    "Apellaios",
    "Audnaios",
    "Peritios",
    "Dystros",
    "Epagomenai",
];

/// Whether `year` carries the sixth epagomenal day: years 2, 6, 10 and so
/// on.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    year.rem_euclid(4) == 2
}

/// The number of days in `month` of `year`, or `None` when `month` is not
/// in `1..=13`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    match month {
        1..=12 => Some(30),
        13 => Some(if is_leap_year(year) { 6 } else { 5 }),
        _ => None,
    }
}

/// The fixed day of a date, without validation.
const fn to_fixed_raw(year: i64, month: u8, day: u8) -> i64 {
    // The years before `year` that are 2 modulo 4.
    let leap_days = (year + 1).div_euclid(4);
    EPOCH.0 + 365 * (year - 1) + leap_days + 30 * (month as i64 - 1) + day as i64 - 1
}

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(to_fixed_raw(MAX_YEAR + 1, 1, 1) - 1);

/// The fixed day of a Bostran date.
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
        Ok(()) => Ok(Rd(to_fixed_raw(year, month, day))),
    }
}

/// The Bostran year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EPOCH`]..=[`LATEST`].
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd.0 < EPOCH.0 {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd.0 > LATEST.0 {
        return Err(CalendarError::AfterSupportedRange);
    }
    // Four years are 1 461 days; the estimate is exact or one too high.
    let estimate = (4 * (rd.0 - EPOCH.0) + 4).div_euclid(1_461) + 1;
    let year = if rd.0 < to_fixed_raw(estimate, 1, 1) {
        estimate - 1
    } else {
        estimate
    };
    let day_of_year = rd.0 - to_fixed_raw(year, 1, 1);
    let month = (day_of_year.div_euclid(30) + 1) as u8;
    let day = (day_of_year.rem_euclid(30) + 1) as u8;
    Ok((year, month, day))
}

/// A Bostran date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BostranDate {
    /// The year of the province, from 1.
    pub year: i64,
    /// The month, 1 (Xanthikos) through 12 (Dystros); 13 holds the
    /// epagomenal days.
    pub month: u8,
    /// The day of the month, from 1.
    pub day: u8,
}

impl BostranDate {
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

/// The Bostran era.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BostranCalendar;

/// Thirteen named months and the seven-day week.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for BostranCalendar {
    type Date = BostranDate;

    /// Unrecorded as a period of days: the source dates the era's use by
    /// the year only, from an inscription of AD 107 to AD 735, "almost
    /// never identified explicitly" in the later period.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("bostran-era"),
            english_name: "Bostran era (Provincia Arabia)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EPOCH),
            latest: Some(LATEST),
            native_locales: &["grc"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(BostranDate { year, month, day })
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
        BostranDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year_one_begins_on_the_twenty_second_of_march_106() {
        assert_eq!(julian::from_fixed(EPOCH), Ok((106, 3, 22)));
        assert_eq!(from_fixed(EPOCH), Ok((1, 1, 1)));
    }

    #[test]
    fn every_new_year_is_the_twenty_second_of_march() {
        // 1 Xanthikos "corresponded to 22 March in the Julian calendar",
        // which the sixth epagomenal day of years 2, 6, 10 keeps true.
        for year in MIN_YEAR..=MAX_YEAR {
            let new_year = to_fixed(year, 1, 1).unwrap();
            assert_eq!(
                julian::from_fixed(new_year),
                Ok((105 + year, 3, 22)),
                "{year}"
            );
        }
        assert!(is_leap_year(2) && is_leap_year(6) && is_leap_year(10));
        assert!(!is_leap_year(1) && !is_leap_year(3) && !is_leap_year(4));
    }

    #[test]
    fn the_sixth_epagomenal_day_is_the_day_before_the_new_year() {
        // Year 2 holds 29 February 108, and ends with its sixth day on
        // 21 March 108; the five days of a common year run 17-21 March.
        let sixth = to_fixed(2, 13, 6).unwrap();
        assert_eq!(julian::from_fixed(sixth), Ok((108, 3, 21)));
        assert_eq!(sixth.0 + 1, to_fixed(3, 1, 1).unwrap().0);
        assert_eq!(
            julian::from_fixed(to_fixed(3, 13, 1).unwrap()),
            Ok((109, 3, 17))
        );
        assert_eq!(to_fixed(3, 13, 6), Err(CalendarError::DayOutOfRange));
    }

    #[test]
    fn every_day_round_trips() {
        for rd in EPOCH.0..=LATEST.0 {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields_and_refuses_outside() {
        let calendar = BostranCalendar;
        for rd in (EPOCH.0..=LATEST.0).step_by(211) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(calendar.from_fields(&fields), Ok(date));
        }
        assert_eq!(from_fixed(Rd(EPOCH.0 - 1)), Err(CalendarError::BeforeEpoch));
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(1, 1, 1).with_era("ad")),
            Err(CalendarError::UnknownEra)
        );
    }
}
