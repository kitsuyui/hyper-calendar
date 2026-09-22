//! The Solar Hijri calendar, astronomical — CLDR `persian`.
//!
//! The Iranian year begins at Nowruz, the day of the March equinox: if the
//! equinox falls before noon, Iran Standard Time, that day is 1 Farvardin;
//! if it falls after noon, that day is 30 Esfand and the next is Nowruz.
//! Everything else follows from where consecutive Nowruzes land — a year is
//! 365 or 366 days, and the thirtieth of Esfand exists in the long ones.
//!
//! Sources for the rule: Wikipedia, "Solar Hijri calendar", retrieved
//! 2026-09-22 ("If the exact moment of astronomical equinox occurs before
//! noon (Tehran time), that day is deemed to be 1 Farvardin"); and
//! M. Heydari-Malayeri, *A concise review of the Iranian calendar*,
//! arXiv:astro-ph/0409620 (2004), §5, which also shows why the result is
//! nearly — and only nearly — a 33-year cycle.
//!
//! # Which noon
//!
//! Iran Standard Time is UTC+03:30, the mean time of the 52.5° E meridian,
//! so the criterion is 08:30 Universal Time on the day of the equinox.
//! Reingold and Dershowitz judge it instead by *apparent* noon at Tehran
//! (51.42° E), which differs from the standard one by the equation of time
//! and four minutes of longitude — up to about a quarter of an hour either
//! way. The two rules disagree only when the equinox falls inside that
//! window, which in any given century is a year or two; this module follows
//! the standard-time rule the country states.
//!
//! # What it is not
//!
//! Not a formula. The arithmetic sibling `persian-arithmetic` in
//! `hc-calendars-solar` is Birashk's 2 820-year cycle, and it is wrong
//! about the present: it puts 1 Farvardin 1404 on 20 March 2025, and Iran
//! kept it on 21 March, because 1403 was a leap year. The tests here state
//! that divergence rather than hide it.
//!
//! Not exact beyond the astronomy either. The equinox instant comes from
//! `hc-astro` to within seconds, so a year whose equinox falls within
//! [`TOLERANCE_MINUTES`] of noon is a year this calendar decides by a model
//! where the country decided by an ephemeris. [`new_year_margin`] says how
//! close the call was.
//!
//! The month names in Persian script are the ones the arithmetic module
//! declares, [`hc_calendars_solar::persian::MONTHS`], and the date type is
//! shared, so a date converts between the two variants without ceremony.

use hc_astro::solar::{Equinox, equinox};
use hc_calendar::fixed::Moment;
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};
use hc_calendars_solar::gregorian;
use hc_calendars_solar::persian::{ERA, MONTHS, PersianDate};

use crate::places::IRAN_STANDARD_OFFSET_DAYS;

/// The identifier CLDR uses for the Solar Hijri calendar.
pub const ID: CalendarId = CalendarId("persian");

/// The Gregorian year in which Solar Hijri year zero would begin, so that
/// 1 Farvardin 1 falls in 622.
pub const GREGORIAN_YEAR_OFFSET: i64 = 621;

/// The earliest year this calendar converts: the epoch.
pub const MIN_YEAR: i64 = 1;

/// The latest year this calendar converts, whose Nowruz falls in Gregorian
/// 3000 — as far as the astronomy behind it is worth asking.
pub const MAX_YEAR: i64 = 3_000 - GREGORIAN_YEAR_OFFSET;

/// How close, in minutes, an equinox may fall to noon before this calendar
/// is deciding by a model rather than by the sky. The noon is a clock time
/// and exact; the equinox is placed to seconds; what is left is ΔT and the
/// truncation of the solar series, and a minute covers them many times over.
pub const TOLERANCE_MINUTES: f64 = 1.0;

/// The fraction of a Universal Time day at which noon, Iran Standard Time,
/// falls: 08:30 UT.
const NOON_IRST_DAY_FRACTION: f64 = 0.5 - IRAN_STANDARD_OFFSET_DAYS;

/// The instant of the March equinox that begins `year`, in Universal Time.
fn equinox_of(year: i64) -> Moment {
    equinox(year + GREGORIAN_YEAR_OFFSET, Equinox::March)
}

/// The day of Nowruz, without validation: the first day whose noon, Iran
/// Standard Time, follows the equinox.
fn nowruz_raw(year: i64) -> Rd {
    let instant = equinox_of(year);
    Rd(Moment(instant.0 - NOON_IRST_DAY_FRACTION).day().0 + 1)
}

/// The fixed day of Nowruz, the first day of `year`.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub fn new_year(year: i64) -> CalendarResult<Rd> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(CalendarError::YearOutOfRange);
    }
    Ok(nowruz_raw(year))
}

/// How far the equinox that begins `year` fell from the noon that decides
/// Nowruz, in minutes: positive when it fell before noon (Nowruz is the
/// equinox day), negative when after (Nowruz is the day after).
///
/// The astronomy is good to a few minutes, so a year whose margin is
/// smaller than that is a year this calendar decides by a model.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub fn new_year_margin(year: i64) -> CalendarResult<f64> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(CalendarError::YearOutOfRange);
    }
    let instant = equinox_of(year);
    let noon = instant.day().0 as f64 + NOON_IRST_DAY_FRACTION;
    Ok((noon - instant.0) * 24.0 * 60.0)
}

/// The earliest fixed day this calendar converts.
#[must_use]
pub fn earliest() -> Rd {
    nowruz_raw(MIN_YEAR)
}

/// The latest fixed day this calendar converts: the day before the Nowruz
/// after [`MAX_YEAR`].
#[must_use]
pub fn latest() -> Rd {
    Rd(nowruz_raw(MAX_YEAR + 1).0 - 1)
}

/// The number of days in `year`, or `None` outside [`MIN_YEAR`]..=[`MAX_YEAR`].
#[must_use]
pub fn days_in_year(year: i64) -> Option<u16> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return None;
    }
    Some((nowruz_raw(year + 1).0 - nowruz_raw(year).0) as u16)
}

/// Whether Esfand has thirty days in `year`, or `None` outside the range.
#[must_use]
pub fn is_leap_year(year: i64) -> Option<bool> {
    days_in_year(year).map(|days| days == 366)
}

/// The number of days in `month` of `year`: thirty-one for the first six,
/// thirty for the next five, twenty-nine or thirty for Esfand. `None` when
/// the year is out of range or `month` is not in `1..=12`.
#[must_use]
pub fn days_in_month(year: i64, month: u8) -> Option<u8> {
    match month {
        1..=6 => days_in_year(year).map(|_| 31),
        7..=11 => days_in_year(year).map(|_| 30),
        12 => is_leap_year(year).map(|leap| if leap { 30 } else { 29 }),
        _ => None,
    }
}

/// Days elapsed in the year before the first of `month`.
const fn days_before_month(month: u8) -> i64 {
    if month <= 7 {
        (month as i64 - 1) * 31
    } else {
        6 * 31 + (month as i64 - 7) * 30
    }
}

/// The fixed day of a Solar Hijri date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(CalendarError::YearOutOfRange);
    }
    let Some(length) = days_in_month(year, month) else {
        return Err(CalendarError::MonthOutOfRange);
    };
    if day == 0 || day > length {
        return Err(CalendarError::DayOutOfRange);
    }
    Ok(Rd(nowruz_raw(year).0
        + days_before_month(month)
        + i64::from(day)
        - 1))
}

/// The Solar Hijri year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside
/// [`earliest`]..=[`latest`].
pub fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd < earliest() {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd > latest() {
        return Err(CalendarError::AfterSupportedRange);
    }
    let (gregorian_year, _, _) = gregorian::from_fixed(rd)?;
    let mut year = gregorian_year - GREGORIAN_YEAR_OFFSET;
    if year > MAX_YEAR {
        year = MAX_YEAR;
    }
    if rd < nowruz_raw(year) {
        year -= 1;
    }
    let day_of_year = rd.0 - nowruz_raw(year).0;
    let month = if day_of_year < 6 * 31 {
        (day_of_year / 31 + 1) as u8
    } else {
        ((day_of_year - 6 * 31) / 30 + 7) as u8
    };
    let day = (day_of_year - days_before_month(month) + 1) as u8;
    Ok((year, month, day))
}

/// The astronomical Solar Hijri calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PersianCalendar;

/// Twelve named months and the seven-day week — the arithmetic variant's
/// own names, since the calendar is the same calendar.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for PersianCalendar {
    type Date = PersianDate;

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Solar Hijri",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: true,
            earliest: Some(earliest()),
            latest: Some(latest()),
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(PersianDate { year, month, day })
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
        let day = fields.require_day()?;
        to_fixed(fields.year, month.ordinal, day)?;
        Ok(PersianDate {
            year: fields.year,
            month: month.ordinal,
            day,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendars_solar::persian as arithmetic;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn nowruz_lands_where_iran_put_it() {
        // Published dates of 1 Farvardin: Wikipedia, "Solar Hijri calendar",
        // correspondence table, retrieved 2026-09-22.
        for (year, expected) in [
            (1390, (2011, 3, 21)),
            (1391, (2012, 3, 20)),
            (1395, (2016, 3, 20)),
            (1399, (2020, 3, 20)),
            (1400, (2021, 3, 21)),
            (1403, (2024, 3, 20)),
            (1404, (2025, 3, 21)),
            (1405, (2026, 3, 21)),
            (1408, (2029, 3, 20)),
            (1409, (2030, 3, 21)),
        ] {
            let (y, m, d) = expected;
            assert_eq!(new_year(year), Ok(ymd(y, m, d)), "Nowruz {year}");
        }
    }

    #[test]
    fn the_leap_years_are_the_ones_the_correspondence_table_marks() {
        // Wikipedia, "Solar Hijri calendar", correspondence table 1354–1419,
        // leap years starred; retrieved 2026-09-22.
        let leap = [
            1354, 1358, 1362, 1366, 1370, 1375, 1379, 1383, 1387, 1391, 1395, 1399, 1403, 1408,
            1412, 1416,
        ];
        for year in 1354..=1419 {
            assert_eq!(
                is_leap_year(year),
                Some(leap.contains(&year)),
                "{year}: {} days",
                days_in_year(year).unwrap_or(0)
            );
            let margin = new_year_margin(year).unwrap();
            assert!(
                margin.abs() >= TOLERANCE_MINUTES,
                "{year} is decided within the model's tolerance: {margin} minutes"
            );
        }
    }

    #[test]
    fn birashks_cycle_and_the_equinox_part_company_in_1404() {
        // The arithmetic sibling makes 1404 the leap year; the equinox made
        // 1403 one, so Nowruz 1404 fell a day later than the cycle says.
        assert_eq!(arithmetic::to_fixed(1404, 1, 1), Ok(ymd(2025, 3, 20)));
        assert_eq!(new_year(1404), Ok(ymd(2025, 3, 21)));
        assert_eq!(is_leap_year(1403), Some(true));
        assert!(!arithmetic::is_leap_year(1403));
    }

    #[test]
    fn the_margin_says_which_side_of_noon_the_equinox_fell() {
        // 2025-03-20 09:01 UT is after 08:30 UT: after noon in Tehran.
        let margin = new_year_margin(1404).unwrap();
        assert!(margin < 0.0, "1404 margin {margin}");
        assert!(margin > -60.0, "1404 margin {margin}");
        // 2024-03-20 03:06 UT is well before noon.
        let margin = new_year_margin(1403).unwrap();
        assert!(margin > 240.0, "1403 margin {margin}");
    }

    #[test]
    fn the_month_layout_is_six_thirty_ones_five_thirties_and_esfand() {
        assert_eq!(days_in_month(1403, 1), Some(31));
        assert_eq!(days_in_month(1403, 6), Some(31));
        assert_eq!(days_in_month(1403, 7), Some(30));
        assert_eq!(days_in_month(1403, 11), Some(30));
        assert_eq!(days_in_month(1403, 12), Some(30));
        assert_eq!(days_in_month(1404, 12), Some(29));
        assert_eq!(days_in_month(1404, 13), None);
        assert_eq!(days_in_month(1404, 0), None);
        // 1 Mehr is the day after 31 Shahrivar; 30 Esfand 1403 is 20 March 2025.
        assert_eq!(to_fixed(1403, 7, 1), Ok(ymd(2024, 9, 22)));
        assert_eq!(to_fixed(1403, 12, 30), Ok(ymd(2025, 3, 20)));
        assert_eq!(to_fixed(1404, 12, 30), Err(CalendarError::DayOutOfRange));
    }

    #[test]
    fn a_sample_of_days_round_trips() {
        let start = new_year(1300).unwrap().0;
        let end = new_year(1500).unwrap().0;
        for rd in (start..end).step_by(97) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = PersianCalendar;
        for rd in (new_year(1380).unwrap().0..new_year(1420).unwrap().0).step_by(367) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, ID);
        assert!(calendar.meta().is_astronomical);
    }

    #[test]
    fn the_range_is_stated_and_refused_outside() {
        assert_eq!(new_year(1), Ok(arithmetic::EPOCH));
        assert_eq!(new_year(0), Err(CalendarError::YearOutOfRange));
        assert_eq!(new_year(MAX_YEAR + 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            from_fixed(Rd(earliest().0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(latest().0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(from_fixed(latest()).map(|(year, _, _)| year), Ok(MAX_YEAR));
    }
}
