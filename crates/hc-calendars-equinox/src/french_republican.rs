//! The French Republican calendar as decreed — `french-republican-equinox`.
//!
//! The system is written up in `docs/systems/equinox-calendars.md` in the
//! repository, with the Badíʿ calendar and the arithmetic siblings: the
//! decree's rule and Romme's proposed replacement, 1 vendémiaire An IV
//! worked from the equinox and the Paris apparent midnight, the years too
//! close to call, what is carried and not, and how the model was checked
//! against the fourteen years France kept. This page summarises it and
//! states the code's own facts.
//!
//! "Chaque année commence à minuit, avec le jour où tombe l'équinoxe vrai
//! d'automne, pour l'observatoire de Paris": the decree of 4 frimaire an II
//! (24 November 1793), article III (`decret-4-frimaire-an-ii` in
//! `docs/references.bib`). *True* time is apparent solar time, so the day
//! runs from the Sun's lower transit at the Observatory to the next, and
//! 1 vendémiaire is the one such day in which the September equinox falls.
//! The year is 365 or 366 days accordingly; the sixth complementary day,
//! the *jour de la Révolution*, exists in the long ones.
//!
//! This is the calendar France kept until the end of An XIV (31 December
//! 1805), which is why the tests are the fourteen historical new years.
//! The arithmetic sibling `french-republican-arithmetic` in
//! `hc-calendars-solar` is the fixed rule Romme proposed in 1795 and the
//! Convention never adopted; the two disagree at once, An IV, VIII and XII
//! beginning a day apart. Past An XIV this is the decree's rule continued;
//! a year whose equinox falls within [`TOLERANCE_MINUTES`] of Paris's
//! apparent midnight is decided here by a model, and [`new_year_margin`]
//! says how close the call was.
//!
//! The month and *décade*-day names are the arithmetic module's own,
//! [`hc_calendars_solar::french_republican::MONTHS`] and
//! [`hc_calendars_solar::french_republican::DECADE_DAYS`], and the date
//! type is shared.

use hc_astro::riseset::solar_midnight;
use hc_astro::solar::{Equinox, equinox};
use hc_calendar::fixed::Moment;
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};
use hc_calendars_solar::french_republican::{DECADE_DAYS, ERA, FrenchRepublicanDate, MONTHS};
use hc_calendars_solar::gregorian;

use crate::places::PARIS_OBSERVATORY;

/// The identifier of the equinox-ruled French Republican calendar.
pub const ID: CalendarId = CalendarId("french-republican-equinox");

/// The Gregorian year in which Republican year zero would begin, so that
/// An I begins in 1792.
pub const GREGORIAN_YEAR_OFFSET: i64 = 1_791;

/// The earliest year this calendar converts: An I.
pub const MIN_YEAR: i64 = 1;

/// The latest year this calendar converts, whose 1 Vendémiaire falls in
/// Gregorian 3000 — as far as the astronomy behind it is worth asking.
pub const MAX_YEAR: i64 = 3_000 - GREGORIAN_YEAR_OFFSET;

/// How close, in minutes, an equinox may fall to Paris's apparent midnight
/// before this calendar is deciding by a model rather than by the sky. Both
/// instants are placed to seconds; a minute covers ΔT and the truncation of
/// the solar series many times over.
pub const TOLERANCE_MINUTES: f64 = 1.0;

/// The instant of the September equinox that begins `year`, in Universal
/// Time.
fn equinox_of(year: i64) -> Moment {
    equinox(year + GREGORIAN_YEAR_OFFSET, Equinox::September)
}

/// The Paris apparent-solar day containing `moment`: the day whose lower
/// transit of the Sun is at or before it, and whose next is after.
fn paris_day_containing(moment: Moment) -> Rd {
    let mut day = moment.day();
    while moment.0 >= solar_midnight(Rd(day.0 + 1), PARIS_OBSERVATORY).0 {
        day = Rd(day.0 + 1);
    }
    while moment.0 < solar_midnight(day, PARIS_OBSERVATORY).0 {
        day = Rd(day.0 - 1);
    }
    day
}

/// The fixed day of 1 Vendémiaire, without validation.
fn new_year_raw(year: i64) -> Rd {
    paris_day_containing(equinox_of(year))
}

/// The fixed day of 1 Vendémiaire, the first day of `year`.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub fn new_year(year: i64) -> CalendarResult<Rd> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(CalendarError::YearOutOfRange);
    }
    Ok(new_year_raw(year))
}

/// How far the equinox that begins `year` fell from the nearer Paris
/// apparent midnight, in minutes: positive when it fell after the midnight
/// that begins 1 Vendémiaire by at least as much as before the next, so
/// the sign is always positive and the size is what matters. A margin
/// smaller than the few minutes the astronomy is good to marks a year this
/// calendar decides by a model.
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
    let day = paris_day_containing(instant);
    let after_start = instant.0 - solar_midnight(day, PARIS_OBSERVATORY).0;
    let before_end = solar_midnight(Rd(day.0 + 1), PARIS_OBSERVATORY).0 - instant.0;
    Ok(if after_start < before_end {
        after_start
    } else {
        before_end
    } * 24.0
        * 60.0)
}

/// The earliest fixed day this calendar converts: 1 Vendémiaire An I.
#[must_use]
pub fn earliest() -> Rd {
    new_year_raw(MIN_YEAR)
}

/// The latest fixed day this calendar converts: the day before the new
/// year after [`MAX_YEAR`].
#[must_use]
pub fn latest() -> Rd {
    Rd(new_year_raw(MAX_YEAR + 1).0 - 1)
}

/// The number of days in `year`, or `None` outside [`MIN_YEAR`]..=[`MAX_YEAR`].
#[must_use]
pub fn days_in_year(year: i64) -> Option<u16> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return None;
    }
    Some((new_year_raw(year + 1).0 - new_year_raw(year).0) as u16)
}

/// Whether `year` has a sixth complementary day, or `None` outside the
/// range.
#[must_use]
pub fn is_leap_year(year: i64) -> Option<bool> {
    days_in_year(year).map(|days| days == 366)
}

/// The number of days in `month` of `year`: thirty for the twelve months,
/// five or six for the complementary days as month 13. `None` when the year
/// is out of range or `month` is not in `1..=13`.
#[must_use]
pub fn days_in_month(year: i64, month: u8) -> Option<u8> {
    match month {
        1..=12 => days_in_year(year).map(|_| 30),
        13 => is_leap_year(year).map(|leap| if leap { 6 } else { 5 }),
        _ => None,
    }
}

/// The fixed day of a Republican date.
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
    Ok(Rd(new_year_raw(year).0
        + (i64::from(month) - 1) * 30
        + i64::from(day)
        - 1))
}

/// The Republican year, month and day of a fixed day.
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
    if rd < new_year_raw(year) {
        year -= 1;
    }
    let day_of_year = rd.0 - new_year_raw(year).0;
    let month = (day_of_year / 30 + 1) as u8;
    let day = (day_of_year % 30 + 1) as u8;
    Ok((year, month, day))
}

/// The French Republican calendar as decreed.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EquinoxFrenchRepublicanCalendar;

/// Twelve named months with the complementary days as a thirteenth
/// position, and the ten-day *décade* — the arithmetic variant's own
/// names, since the calendar is the same calendar.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTHS),
    hc_calendar::shape::CycleShape::named("decade-day", &DECADE_DAYS),
];

impl Calendar for EquinoxFrenchRepublicanCalendar {
    type Date = FrenchRepublicanDate;

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    /// A year with a sixth complementary day.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        is_leap_year(year).ok_or(CalendarError::YearOutOfRange)
    }

    /// In force from the decree of 1793 until Napoleon abolished it at the
    /// end of An XIV, 31 December 1805 — the same span as the arithmetic
    /// variant, since the two agree throughout it.
    fn usage(&self) -> hc_calendar::Usage {
        match (
            gregorian::to_fixed(1793, 10, 24),
            gregorian::to_fixed(1805, 12, 31),
        ) {
            (Ok(from), Ok(until)) => hc_calendar::Usage::between(
                from,
                until,
                hc_calendars_solar::french_republican::USAGE_SOURCE,
            ),
            _ => hc_calendar::Usage::UNRECORDED,
        }
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "French Republican (equinox)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: true,
            earliest: Some(earliest()),
            latest: Some(latest()),
            native_locales: &["fr"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(FrenchRepublicanDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let fields = DateFields::ymd(date.year, date.month, date.day).with_era(ERA);
        match date.decade() {
            None => Ok(fields),
            Some((decade, day)) => fields
                .with_extra("decade", i64::from(decade))?
                .with_extra("day-of-decade", i64::from(day)),
        }
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
        Ok(FrenchRepublicanDate {
            year: fields.year,
            month: month.ordinal,
            day,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendars_solar::french_republican as arithmetic;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn the_fourteen_years_france_kept_begin_where_the_record_says() {
        // 1 Vendémiaire An I–XIV: Wikipedia, "French Republican calendar",
        // retrieved 2026-09-22.
        let record = [
            (1, (1792, 9, 22)),
            (2, (1793, 9, 22)),
            (3, (1794, 9, 22)),
            (4, (1795, 9, 23)),
            (5, (1796, 9, 22)),
            (6, (1797, 9, 22)),
            (7, (1798, 9, 22)),
            (8, (1799, 9, 23)),
            (9, (1800, 9, 23)),
            (10, (1801, 9, 23)),
            (11, (1802, 9, 23)),
            (12, (1803, 9, 24)),
            (13, (1804, 9, 23)),
            (14, (1805, 9, 23)),
        ];
        for (year, (y, m, d)) in record {
            assert_eq!(
                new_year(year),
                Ok(ymd(y, m, d)),
                "An {year}: margin {:?} minutes",
                new_year_margin(year)
            );
            let margin = new_year_margin(year).unwrap();
            assert!(
                margin >= TOLERANCE_MINUTES,
                "An {year} is decided within the model's tolerance: {margin} minutes"
            );
        }
    }

    #[test]
    fn the_sextile_years_are_the_third_seventh_and_eleventh() {
        for year in 1..=14 {
            assert_eq!(
                is_leap_year(year),
                Some(matches!(year, 3 | 7 | 11)),
                "An {year}: {} days",
                days_in_year(year).unwrap_or(0)
            );
        }
        assert_eq!(days_in_month(3, 13), Some(6));
        assert_eq!(days_in_month(4, 13), Some(5));
        assert_eq!(days_in_month(4, 14), None);
    }

    #[test]
    fn romme_and_the_equinox_part_company_in_the_fourth_eighth_and_twelfth_years() {
        // Romme's sextile day ends An IV, VIII and XII; the equinox's ended
        // An III, VII and XI. So those three years begin a day later by the
        // equinox than by the rule, and the two agree again a year later.
        for year in 1..=14 {
            let romme = arithmetic::to_fixed(year, 1, 1).unwrap();
            let expected = if matches!(year, 4 | 8 | 12) {
                Rd(romme.0 + 1)
            } else {
                romme
            };
            assert_eq!(new_year(year), Ok(expected), "An {year}");
        }
    }

    #[test]
    fn a_sample_of_days_round_trips() {
        let start = new_year(1).unwrap().0;
        let end = new_year(120).unwrap().0;
        for rd in (start..end).step_by(41) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = EquinoxFrenchRepublicanCalendar;
        for rd in (new_year(1).unwrap().0..new_year(20).unwrap().0).step_by(97) {
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
        assert_eq!(to_fixed(4, 13, 6), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(4, 14, 1), Err(CalendarError::MonthOutOfRange));
    }
}
