//! The ISO calendar under CLDR's calendar identifier `iso8601`.
//!
//! Unicode CLDR 48 registers `iso8601` as a calendar of its own beside
//! `gregory`: "ISO calendar (Gregorian calendar using the ISO 8601 calendar
//! week rules)", since CLDR 2.0 (`common/bcp47/calendar.xml`,
//! `cldr-bcp47-calendar`). ICU4C's `ISO8601Calendar` is "a subclass of
//! GregorianCalendar" in which "the first day of a week is Monday and the
//! minimal days in the first week of a year or month is four days"
//! (`iso8601cal.h`, `icu4c-iso8601cal`), and ICU4X's `Iso`, which
//! "corresponds to the `"iso8601"` CLDR calendar", "is identical to the
//! Gregorian calendar, except that it uses a single `default` era instead
//! of `bce` and `ce`" (`icu` 2.3.1, `icu4x-iso`). All three were
//! read on 2026-09-26.
//!
//! So the days are `gregory`'s, and what the identifier adds is two rules:
//! the week of the year is the ISO one, Monday first and four days in the
//! first week, and the year is written astronomically without an era. This
//! library's `gregory` already numbers years astronomically, so the
//! identifier is registered for the rule it carries and for interchange: a
//! locale tagged `-u-ca-iso8601` names this calendar, and it answers with
//! the ISO week-numbering year, week and weekday beside the date, as
//! [`crate::iso_week`] computes them. `iso8601-week` and `iso8601-ordinal`
//! are this library's names for two ISO 8601 notations of the same day and
//! are not CLDR identifiers; this module is the one CLDR names.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Weekday,
    YearKind,
};

use crate::gregorian::{self, GregorianDate};
use crate::iso_week;

/// The first day of the week under the ISO week rules.
pub const FIRST_WEEKDAY: Weekday = Weekday::Monday;

/// The fewest days of a year the first week of the year holds.
pub const MINIMAL_DAYS_IN_FIRST_WEEK: u8 = 4;

/// The earliest fixed day this implementation converts: the week date's.
pub const EARLIEST: Rd = Rd(gregorian::EARLIEST.0 + 400);

/// The latest fixed day this implementation converts: the week date's.
pub const LATEST: Rd = Rd(gregorian::LATEST.0 - 400);

/// The ISO 8601 calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct IsoCalendar;

impl Calendar for IsoCalendar {
    type Date = GregorianDate;

    /// From the first edition of ISO 8601, June 1988, as
    /// [`crate::iso_week`] records it.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::since(iso_week::FIRST_PUBLISHED, iso_week::USAGE_SOURCE)
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(gregorian::is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("iso8601"),
            english_name: "ISO 8601",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &[],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        let rd = gregorian::to_fixed(date.year, date.month, date.day)?;
        self.meta().check_range(rd)?;
        Ok(rd)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        self.meta().check_range(rd)?;
        let (year, month, day) = gregorian::from_fixed(rd)?;
        GregorianDate::new(year, month, day)
    }

    /// The date, and the ISO week-numbering year, week and weekday of the
    /// same day.
    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let (week_year, week, weekday) = iso_week::from_fixed(self.to_fixed(date)?)?;
        DateFields::ymd(date.year, date.month, date.day)
            .with_extra("week-year", week_year)?
            .with_extra("week", i64::from(week))?
            .with_extra("day-of-week", i64::from(weekday))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some() {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let date = GregorianDate::new(fields.year, month.ordinal, fields.require_day()?)?;
        self.to_fixed(date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GregorianCalendar;

    #[test]
    fn the_days_are_the_gregorian_calendars() {
        for rd in (EARLIEST.0..=LATEST.0).step_by(100_003) {
            assert_eq!(
                IsoCalendar.from_fixed(Rd(rd)),
                GregorianCalendar.from_fixed(Rd(rd))
            );
        }
    }

    #[test]
    fn the_week_fields_follow_the_iso_rule() {
        // Monday first, four days in the first week: 1 January 2021, a
        // Friday, is in week 53 of 2020, and 30 December 2019, a Monday,
        // opens week 1 of 2020.
        assert_eq!(FIRST_WEEKDAY, Weekday::Monday);
        assert_eq!(MINIMAL_DAYS_IN_FIRST_WEEK, 4);
        for ((year, month, day), (week_year, week, weekday)) in [
            ((2021, 1, 1), (2020, 53, 5)),
            ((2019, 12, 30), (2020, 1, 1)),
            ((2026, 9, 26), (2026, 39, 6)),
        ] {
            let fields = IsoCalendar
                .to_fields(GregorianDate::new(year, month, day).unwrap())
                .unwrap();
            assert_eq!(fields.era, None);
            assert_eq!(fields.extra.get("week-year"), Some(week_year));
            assert_eq!(fields.extra.get("week"), Some(week));
            assert_eq!(fields.extra.get("day-of-week"), Some(weekday));
        }
    }

    #[test]
    fn the_calendar_round_trips_through_fields_and_refuses_an_era() {
        for rd in (EARLIEST.0..=LATEST.0).step_by(12_347) {
            let date = IsoCalendar.from_fixed(Rd(rd)).unwrap();
            let fields = IsoCalendar.to_fields(date).unwrap();
            assert_eq!(IsoCalendar.from_fields(&fields), Ok(date));
        }
        assert_eq!(
            IsoCalendar.from_fields(&DateFields::ymd(2026, 1, 1).with_era("ce")),
            Err(CalendarError::UnknownEra)
        );
        assert_eq!(
            IsoCalendar.from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        // Year 0 is 1 BC and has a 29 February.
        assert!(IsoCalendar.is_leap_year(0).unwrap());
        assert_eq!(IsoCalendar.meta().id, CalendarId("iso8601"));
    }
}
