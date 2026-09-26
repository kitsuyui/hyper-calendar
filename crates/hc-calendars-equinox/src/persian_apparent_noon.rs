//! The Solar Hijri calendar judged by apparent noon at Tehran —
//! `persian-apparent-noon`.
//!
//! The same calendar as [`crate::persian`], with the other reading of its
//! rule. Nowruz is the day of the March equinox if the equinox falls
//! before noon, and the day after otherwise; `persian` takes the noon of
//! Iran Standard Time, 12:00 at UTC+03:30, and this calendar takes the
//! Sun's own noon, its transit of the meridian of Tehran. That is how
//! M. Heydari-Malayeri, *A concise review of the Iranian calendar*,
//! arXiv:astro-ph/0409620 (2004), §2, states it — the year "should begin
//! at midnight (Tehran true time)" (`heydari-malayeri2004`) — and how
//! Reingold and Dershowitz compute it, as `midday-in-tehran`, "true noon
//! … in Tehran", at 35.68° N, 51.42° E ([`crate::places::TEHRAN_PERSIAN`];
//! `reingold2018code`, `persian-new-year-on-or-before`; the book's
//! chapter on the Persian calendar, `reingold2018`, not read here).
//!
//! The two noons are about twelve minutes apart in March: Tehran lies
//! 1.08° west of the 52.5° meridian, four minutes of longitude, and the
//! equation of time holds the Sun some seven or eight minutes behind the
//! clock. They name different days only when the equinox falls between
//! them, which in the three thousand years this crate converts happens in
//! twenty years, 1470 (2091) the first after 1177 and 1503, 1536, 1602,
//! 1701, 2027, 2093 and 2159 the rest; from 1178 to 1469 the two calendars
//! are the same day for day. `docs/systems/solar-hijri.md` sets out the readings, the years
//! and the sources.
//!
//! Like `persian`, this is a model of a rule: [`new_year_margin`] says how
//! close each year's call was, and a year within [`TOLERANCE_MINUTES`] is
//! decided by the model. 1470 is such a year: its equinox falls about ten
//! seconds before the Sun's noon at Tehran.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};
use hc_calendars_solar::gregorian;
use hc_calendars_solar::persian::{ERA, MONTHS, PersianDate};

use crate::persian::{
    MAX_YEAR, MIN_YEAR, Noon, days_in_month_by, days_in_year_by, earliest_by, from_fixed_by,
    latest_by, margin_by, nowruz_by, to_fixed_by,
};
use crate::places::TEHRAN_PERSIAN;

/// The calendar's identifier.
pub const ID: CalendarId = CalendarId("persian-apparent-noon");

/// How close, in minutes, an equinox may fall to Tehran's apparent noon
/// before this calendar is deciding by a model. Both instants are placed
/// to seconds; a minute covers ΔT and the truncation of the solar series
/// many times over, as for [`crate::persian::TOLERANCE_MINUTES`].
pub const TOLERANCE_MINUTES: f64 = 1.0;

/// The rule this module's calendar follows.
const RULE: Noon = Noon::Apparent(TEHRAN_PERSIAN);

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "The calendar Iran adopted on 31 March 1925, as `persian` dates it, read with \
    Nowruz decided by the Sun's noon at Tehran as Heydari-Malayeri 2004, section 2 \
    [heydari-malayeri2004], and Reingold and Dershowitz's `midday-in-tehran` \
    [reingold2018code] state the rule; no source read says which noon Iran's calendar \
    authority applies, and the two readings give the same days from 1178 to 1469";

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
    Ok(nowruz_by(RULE, year))
}

/// How far the equinox that begins `year` fell from the nearer apparent
/// noon at Tehran, in minutes: positive before it (Nowruz is the equinox
/// day), negative after (Nowruz is the day after).
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub fn new_year_margin(year: i64) -> CalendarResult<f64> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(CalendarError::YearOutOfRange);
    }
    Ok(margin_by(RULE, year))
}

/// The earliest fixed day this calendar converts.
#[must_use]
pub fn earliest() -> Rd {
    earliest_by(RULE)
}

/// The latest fixed day this calendar converts.
#[must_use]
pub fn latest() -> Rd {
    latest_by(RULE)
}

/// The number of days in `year`, or `None` outside [`MIN_YEAR`]..=[`MAX_YEAR`].
#[must_use]
pub fn days_in_year(year: i64) -> Option<u16> {
    days_in_year_by(RULE, year)
}

/// Whether Esfand has thirty days in `year`, or `None` outside the range.
#[must_use]
pub fn is_leap_year(year: i64) -> Option<bool> {
    days_in_year(year).map(|days| days == 366)
}

/// The number of days in `month` of `year`, or `None` when the year is out
/// of range or `month` is not in `1..=12`.
#[must_use]
pub fn days_in_month(year: i64, month: u8) -> Option<u8> {
    days_in_month_by(RULE, year, month)
}

/// The fixed day of a date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    to_fixed_by(RULE, year, month, day)
}

/// The year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside
/// [`earliest`]..=[`latest`].
pub fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    from_fixed_by(RULE, rd)
}

/// The Solar Hijri calendar judged by apparent noon at Tehran.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ApparentNoonPersianCalendar;

/// Twelve named months and the seven-day week, as `persian` declares them.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for ApparentNoonPersianCalendar {
    type Date = PersianDate;

    /// The calendar of the law of 1925 under this reading of its rule,
    /// from the same day as `persian`.
    fn usage(&self) -> hc_calendar::Usage {
        match gregorian::to_fixed(1925, 3, 31) {
            Ok(rd) => hc_calendar::Usage::since(rd, USAGE_SOURCE),
            Err(_) => hc_calendar::Usage::UNRECORDED,
        }
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    /// A year whose Esfand has thirty days.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        is_leap_year(year).ok_or(CalendarError::YearOutOfRange)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Solar Hijri (apparent noon at Tehran)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: true,
            earliest: Some(earliest()),
            latest: Some(latest()),
            native_locales: &["fa"],
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
    use crate::persian;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn nowruz_lands_where_iran_put_it() {
        // Published dates of 1 Farvardin: Wikipedia, "Solar Hijri
        // calendar", correspondence table, retrieved 2026-09-22, as
        // `persian` tests them; and 1375 on 20 March 1996
        // (heydari-malayeri2004, §8).
        for (year, expected) in [
            (1375, (1996, 3, 20)),
            (1390, (2011, 3, 21)),
            (1391, (2012, 3, 20)),
            (1399, (2020, 3, 20)),
            (1400, (2021, 3, 21)),
            (1403, (2024, 3, 20)),
            (1404, (2025, 3, 21)),
            (1405, (2026, 3, 21)),
            (1409, (2030, 3, 21)),
        ] {
            let (y, m, d) = expected;
            assert_eq!(new_year(year), Ok(ymd(y, m, d)), "Nowruz {year}");
        }
    }

    #[test]
    fn the_two_noons_give_the_same_days_from_1178_to_1469() {
        for year in 1_178..=1_469 {
            assert_eq!(new_year(year), persian::new_year(year), "{year}");
        }
    }

    #[test]
    fn the_twenty_years_the_two_noons_part_company() {
        // Every such year's equinox falls between 08:30 UT and the Sun's
        // noon at Tehran, so the apparent reading keeps the equinox day
        // and the standard-time reading takes the next.
        let expected = [
            166, 426, 492, 525, 686, 719, 752, 785, 1_078, 1_111, 1_144, 1_177, 1_470, 1_503,
            1_536, 1_602, 1_701, 2_027, 2_093, 2_159,
        ];
        let mut found = 0;
        for year in MIN_YEAR..=MAX_YEAR {
            if new_year(year) != persian::new_year(year) {
                assert_eq!(expected.get(found), Some(&year), "{year}");
                found += 1;
            }
        }
        assert_eq!(found, expected.len());
        for year in expected {
            assert_eq!(
                persian::new_year(year).map(|rd| rd.0),
                new_year(year).map(|rd| rd.0 + 1),
                "{year}"
            );
            assert!(new_year_margin(year).unwrap() > 0.0, "{year}");
            assert!(persian::new_year_margin(year).unwrap() < 0.0, "{year}");
        }
    }

    #[test]
    fn the_year_1470_is_the_models_to_decide() {
        let margin = new_year_margin(1_470).unwrap();
        assert!(margin.abs() < TOLERANCE_MINUTES, "1470 margin {margin}");
        assert_eq!(new_year(1_470), Ok(ymd(2091, 3, 20)));
        assert_eq!(persian::new_year(1_470), Ok(ymd(2091, 3, 21)));
    }

    #[test]
    fn a_sample_of_days_round_trips() {
        let start = new_year(1_300).unwrap().0;
        let end = new_year(1_500).unwrap().0;
        for rd in (start..end).step_by(97) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
        // Across the first year the readings differ.
        for rd in new_year(1_469).unwrap().0..new_year(1_471).unwrap().0 {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
        assert_eq!(days_in_month(1_469, 12), Some(29));
        assert_eq!(persian::days_in_month(1_469, 12), Some(30));
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = ApparentNoonPersianCalendar;
        for rd in (new_year(1_380).unwrap().0..new_year(1_480).unwrap().0).step_by(367) {
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
        assert_eq!(new_year(0), Err(CalendarError::YearOutOfRange));
        assert_eq!(new_year(MAX_YEAR + 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(new_year_margin(0), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            from_fixed(Rd(earliest().0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(latest().0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(to_fixed(1_404, 12, 30), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(1_404, 13, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(is_leap_year(0), None);
    }
}
