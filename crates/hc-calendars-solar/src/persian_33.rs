//! The Solar Hijri calendar under the 33-year rule — `persian-arithmetic-33`.
//!
//! The second arithmetic approximation of the Iranian calendar, beside
//! Birashk's 2 820-year cycle in [`crate::persian`]. Eight leap years in
//! every thirty-three: a year is leap when it leaves a remainder of 1, 5,
//! 9, 13, 17, 22, 26 or 30 on division by 33, so the cycle opens with a
//! leap year after four common ones and then takes one every fourth year.
//! Its mean year is 365 + 8⁄33 = 365.2424… days, the length of the year
//! from one March equinox to the next rather than the mean tropical year
//! the 2 820-year cycle aims at.
//!
//! The rule, the remainders and the reading of the cycle are M.
//! Heydari-Malayeri's, *A concise review of the Iranian calendar*,
//! arXiv:astro-ph/0409620 (2004), §§5 and 8 (`heydari-malayeri2004`),
//! which gives the remainders as the ones H. Bagher-Zadeh's conversion
//! program uses, cites K. M. Borkowski, "The Persian calendar for 3000
//! years", *Earth, Moon, and Planets* 74 (1996) 223–230 (`borkowski1996`,
//! not read here), for the rule's agreement with the equinox from A.P.
//! 1178 to 1634, and calls the 2 820-year cycle erroneous (§7). The
//! system document is `docs/systems/solar-hijri.md`.
//!
//! # Why a second arithmetic calendar
//!
//! The two cycles are both published approximations of the same
//! astronomical calendar, and they disagree about the present:
//! Birashk's puts Nowruz 1404 on 20 March 2025 and this one, like the
//! equinox, on 21 March. Policy §5 gives each convention a name. The
//! identifier `persian-arithmetic` stays with the 2 820-year cycle because
//! it is the scheme Reingold and Dershowitz call `arithmetic-persian`
//! (`reingold2018code`), and this one says which rule it is.
//!
//! # The epoch the rule implies
//!
//! Counting the rule back from its modern years puts 1 Farvardin 1 on
//! 18 March 622 in the Julian calendar, a day before the epoch of the
//! 2 820-year cycle ([`EPOCH`]). Neither is an observation: the rule is
//! stated for the modern calendar, and the years before 1178 are its
//! arithmetic carried back.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::common;
use crate::persian::{ERA, MONTHS, PersianDate, days_before_month};

/// The calendar's identifier.
pub const ID: CalendarId = CalendarId("persian-arithmetic-33");

/// The fixed day of 1 Farvardin 1 under the 33-year rule: 18 March 622 in
/// the Julian calendar, the day before [`crate::persian::EPOCH`].
pub const EPOCH: Rd = Rd(crate::persian::EPOCH.0 - 1);

/// The length of the leap cycle in years.
pub const CYCLE_YEARS: i64 = 33;

/// The number of leap years in a cycle.
pub const LEAPS_PER_CYCLE: i64 = 8;

/// The remainders on division by 33 of the leap years
/// (`heydari-malayeri2004`, §8).
pub const LEAP_REMAINDERS: [i64; 8] = [1, 5, 9, 13, 17, 22, 26, 30];

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 9_999;

/// Whether `year` is a leap year: whether it leaves one of
/// [`LEAP_REMAINDERS`] on division by 33.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    // (25y + 11) mod 33 < 8 picks out exactly the eight remainders.
    (25 * year + 11).rem_euclid(CYCLE_YEARS) < LEAPS_PER_CYCLE
}

/// The fixed day of 1 Farvardin of `year`, without validation: the leap
/// years before it are ⌊(8y + 21) / 33⌋.
const fn new_year_raw(year: i64) -> i64 {
    EPOCH.0 + 365 * (year - 1) + (LEAPS_PER_CYCLE * year + 21).div_euclid(CYCLE_YEARS)
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// The number of days in `month` of `year`, or `None` when `month` is not in
/// `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    match month {
        1..=6 => Some(31),
        7..=11 => Some(30),
        12 => Some(if is_leap_year(year) { 30 } else { 29 }),
        _ => None,
    }
}

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = Rd(new_year_raw(MIN_YEAR));

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(new_year_raw(MAX_YEAR + 1) - 1);

/// The fixed day of a date.
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
        Ok(()) => Ok(Rd(new_year_raw(year)
            + days_before_month(month)
            + day as i64
            - 1)),
    }
}

/// The year, month and day of a fixed day.
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
    // 12 053 days make 33 years; the estimate is never more than a year
    // out, and the two loops settle it.
    let mut year = 1 + (CYCLE_YEARS * (rd.0 - EPOCH.0)).div_euclid(12_053);
    while year > MIN_YEAR && new_year_raw(year) > rd.0 {
        year -= 1;
    }
    while new_year_raw(year + 1) <= rd.0 {
        year += 1;
    }
    let day_of_year = rd.0 - new_year_raw(year) + 1;
    let ordinal = if day_of_year <= 186 {
        (day_of_year + 30) / 31
    } else {
        (day_of_year + 23) / 30
    };
    let month = ordinal as u8;
    let day = (day_of_year - days_before_month(month)) as u8;
    Ok((year, month, day))
}

/// The Solar Hijri calendar under the 33-year rule.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ThirtyThreeYearPersianCalendar;

/// Twelve named months and the seven-day week, as [`crate::persian`]
/// declares them.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for ThirtyThreeYearPersianCalendar {
    type Date = PersianDate;

    /// Unrecorded: the rule approximates the calendar in force, which is
    /// `persian`, and no authority read promulgates the rule itself.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Solar Hijri (33-year rule)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
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
    use crate::{gregorian, julian, persian};

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn the_leap_years_are_the_eight_remainders() {
        for year in MIN_YEAR..=MAX_YEAR {
            assert_eq!(
                is_leap_year(year),
                LEAP_REMAINDERS.contains(&year.rem_euclid(CYCLE_YEARS)),
                "{year}"
            );
        }
        let leaps = (1..=CYCLE_YEARS).filter(|year| is_leap_year(*year)).count();
        assert_eq!(leaps as i64, LEAPS_PER_CYCLE);
    }

    #[test]
    fn the_year_lengths_and_the_leap_rule_agree_everywhere() {
        for year in MIN_YEAR..MAX_YEAR {
            let length = new_year_raw(year + 1) - new_year_raw(year);
            assert_eq!(length, i64::from(days_in_year(year)), "{year}");
        }
    }

    #[test]
    fn the_epoch_is_a_day_before_the_2820_year_cycles() {
        assert_eq!(julian::to_fixed(622, 3, 18), Ok(EPOCH));
        assert_eq!(to_fixed(1, 1, 1), Ok(EPOCH));
        assert_eq!(Rd(EPOCH.0 + 1), persian::EPOCH);
    }

    #[test]
    fn heydari_malayeris_example_year_is_leap() {
        // "the year A.P. 1375 that begun on March 20, 1996 has the
        // remainder of 22 and thus is the leap year" (heydari-malayeri2004,
        // §8).
        assert_eq!(1375 % 33, 22);
        assert!(is_leap_year(1375));
        assert_eq!(to_fixed(1375, 1, 1), Ok(ymd(1996, 3, 20)));
    }

    #[test]
    fn the_leap_years_are_the_ones_the_correspondence_table_marks() {
        // Wikipedia, "Solar Hijri calendar", correspondence table 1354–1419,
        // leap years starred, as `hc-calendars-equinox`'s `persian` tests
        // it; retrieved 2026-09-22.
        let leap = [
            1354, 1358, 1362, 1366, 1370, 1375, 1379, 1383, 1387, 1391, 1395, 1399, 1403, 1408,
            1412, 1416,
        ];
        for year in 1354..=1419 {
            assert_eq!(is_leap_year(year), leap.contains(&year), "{year}");
        }
        // Nowruz as published in the same table.
        for (year, expected) in [
            (1390, (2011, 3, 21)),
            (1399, (2020, 3, 20)),
            (1400, (2021, 3, 21)),
            (1403, (2024, 3, 20)),
            (1404, (2025, 3, 21)),
            (1405, (2026, 3, 21)),
            (1409, (2030, 3, 21)),
        ] {
            let (y, m, d) = expected;
            assert_eq!(to_fixed(year, 1, 1), Ok(ymd(y, m, d)), "Nowruz {year}");
        }
    }

    #[test]
    fn the_two_arithmetic_cycles_part_company_in_1403() {
        // Birashk's cycle makes 1404 the leap year, this rule 1403.
        assert!(is_leap_year(1403));
        assert!(!persian::is_leap_year(1403));
        assert_eq!(persian::to_fixed(1404, 1, 1), Ok(ymd(2025, 3, 20)));
        assert_eq!(to_fixed(1404, 1, 1), Ok(ymd(2025, 3, 21)));
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (EARLIEST.0..=LATEST.0).step_by(97) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
        let start = to_fixed(1350, 1, 1).unwrap().0;
        let end = to_fixed(1450, 1, 1).unwrap().0;
        for rd in start..end {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = ThirtyThreeYearPersianCalendar;
        for rd in (EARLIEST.0..=LATEST.0).step_by(919) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, ID);
        assert!(!calendar.meta().is_astronomical);
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
        assert_eq!(from_fixed(LATEST).map(|(year, ..)| year), Ok(MAX_YEAR));
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert!(to_fixed(1403, 12, 30).is_ok());
        assert_eq!(to_fixed(1404, 12, 30), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(1404, 13, 1), Err(CalendarError::MonthOutOfRange));
    }
}
