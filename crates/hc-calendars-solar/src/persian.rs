//! The Solar Hijri calendar, arithmetic (Birashk) variant.
//!
//! The Iranian year begins at Nowruz, the day on which the March equinox
//! falls before noon at the 52.5°E meridian. That is an astronomical
//! definition: it cannot be reduced to arithmetic without approximating,
//! and this module is the approximation.
//!
//! # Which variant this is, and which it is not
//!
//! Implemented here is the 2 820-year cyclic rule associated with Ahmad
//! Birashk, in the form given by Reingold and Dershowitz, *Calendrical
//! Calculations*, as `fixed-from-arithmetic-persian`. The cycle contains
//! 683 leap years, giving a mean year of 365.24219858 days — within a
//! second of the mean tropical year, and far better than the Gregorian
//! 365.2425.
//!
//! It is nonetheless **not** the calendar of the Iranian civil code. The
//! official calendar follows the equinox, and the two disagree for a handful
//! of years even inside the range where the cycle is at its best (Birashk's
//! own claim is agreement from 1178 to 1633 A.P. with a small number of
//! exceptions). Nowruz is an observation, not a formula.
//!
//! For that reason this calendar takes the identifier `persian-arithmetic`
//! and leaves the CLDR identifier `persian` to the astronomical
//! implementation, which is `hc-calendars-equinox`'s and has the published
//! Nowruz 1404 this cycle misses.
//!
//! # Structure
//!
//! Six months of 31 days, five of 30, and a last month of 29 days — 30 in a
//! leap year. The month names in Persian script are [`MONTHS`], declared
//! with the calendar's shape; romanisations belong to `hc-i18n`.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::common;

/// The fixed day of 1 Farvardin 1 A.P., which is 622-03-19 in the Julian
/// calendar and 622-03-22 in the proleptic Gregorian one.
pub const EPOCH: Rd = Rd(226_896);

/// The era code of the Solar Hijri era.
pub const ERA: &str = "AP";

/// The length of the leap cycle in years.
pub const CYCLE_YEARS: i64 = 2_820;

/// The length of the leap cycle in days: 2 820 years containing 683 leap
/// years, so a mean year of 365.24219858 days.
pub const CYCLE_DAYS: i64 = 1_029_983;

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 9_999;

/// The year's position within the 2 820-year cycle, shifted so that the
/// cycle arithmetic starts at year 474.
const fn cycle_position(year: i64) -> (i64, i64) {
    let offset = if year > 0 { year - 474 } else { year - 473 };
    (offset, offset.rem_euclid(CYCLE_YEARS) + 474)
}

/// Whether `year` is a leap year under the arithmetic rule.
///
/// The test distributes 683 leap years over 2 820 as evenly as a single
/// modulo can; it is not the equinox.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    let (_, within) = cycle_position(year);
    ((within + 38) * 31).rem_euclid(128) <= 30
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

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// Days elapsed in the year before the first of `month`.
const fn days_before_month(month: u8) -> i64 {
    if month <= 7 {
        31 * (month as i64 - 1)
    } else {
        30 * (month as i64 - 1) + 6
    }
}

/// The fixed day of a date, without validation.
const fn to_fixed_raw(year: i64, month: u8, day: u8) -> i64 {
    let (offset, within) = cycle_position(year);
    EPOCH.0 - 1
        + CYCLE_DAYS * offset.div_euclid(CYCLE_YEARS)
        + 365 * (within - 1)
        + (31 * within - 5).div_euclid(128)
        + days_before_month(month)
        + day as i64
}

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = Rd(to_fixed_raw(MIN_YEAR, 1, 1));

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(to_fixed_raw(MAX_YEAR + 1, 1, 1) - 1);

/// The fixed day of a Solar Hijri date.
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

/// The Solar Hijri year containing a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EARLIEST`]..=[`LATEST`].
pub const fn year_from_fixed(rd: Rd) -> CalendarResult<i64> {
    if rd.0 < EARLIEST.0 {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd.0 > LATEST.0 {
        return Err(CalendarError::AfterSupportedRange);
    }
    let elapsed = rd.0 - to_fixed_raw(475, 1, 1);
    let cycles = elapsed.div_euclid(CYCLE_DAYS);
    let within_cycle = elapsed.rem_euclid(CYCLE_DAYS);
    // The last day of a cycle would otherwise round up into the next one.
    let year_in_cycle = if within_cycle == CYCLE_DAYS - 1 {
        CYCLE_YEARS
    } else {
        (128 * within_cycle + 46_878).div_euclid(46_751)
    };
    Ok(474 + CYCLE_YEARS * cycles + year_in_cycle)
}

/// The Solar Hijri year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside [`EARLIEST`]..=[`LATEST`].
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    match year_from_fixed(rd) {
        Err(error) => Err(error),
        Ok(year) => {
            let day_of_year = rd.0 - to_fixed_raw(year, 1, 1) + 1;
            // The first six months are 31 days and the rest 30, so the month
            // falls out of one division on each side of day 186.
            let ordinal = if day_of_year <= 186 {
                (day_of_year + 30) / 31
            } else {
                (day_of_year + 23) / 30
            };
            let month = ordinal as u8;
            let day = (day_of_year - days_before_month(month)) as u8;
            Ok((year, month, day))
        }
    }
}

/// A Solar Hijri date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PersianDate {
    /// The year of the Solar Hijri era, counting from 1.
    pub year: i64,
    /// The month, 1 for Farvardin through 12 for Esfand.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl PersianDate {
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

    /// Whether this date is Nowruz, the first day of the year.
    #[must_use]
    pub const fn is_nowruz(self) -> bool {
        self.month == 1 && self.day == 1
    }
}

/// The arithmetic Solar Hijri calendar.
///
/// The name carries the variant because the difference matters: see the
/// module documentation for what this is not.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ArithmeticPersianCalendar;

/// The twelve months in Persian script, Farvardin first.
///
/// Source: the months table of Wikipedia, "Solar Hijri calendar",
/// retrieved 2026-09-22. Mordad and Esfand have the older variants Amordad
/// and Espand, which are not listed. The romanisations (Farvardin,
/// Ordibehesht …) are a locale's and live in `hc-i18n`.
pub const MONTHS: [&str; 12] = [
    "فروردین",
    "اردیبهشت",
    "خرداد",
    "تیر",
    "مرداد",
    "شهریور",
    "مهر",
    "آبان",
    "آذر",
    "دی",
    "بهمن",
    "اسفند",
];

/// Twelve named months and the seven-day week.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for ArithmeticPersianCalendar {
    type Date = PersianDate;

    /// Unrecorded: Birashk's cycle is an arithmetic approximation that no
    /// authority promulgated; the calendar in force is `persian`.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// Twelve months, named in Persian script, and the seven-day week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("persian-arithmetic"),
            english_name: "Solar Hijri (arithmetic)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            // The rule implemented here is arithmetic; the calendar it
            // approximates is not, which is what the module documentation is
            // about. This flag describes the implementation.
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
        PersianDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{gregorian, julian};

    #[test]
    fn the_epoch_is_the_hijra_year_in_the_julian_calendar() {
        // 1 Farvardin 1 A.P. is 19 March AD 622 Julian, 22 March Gregorian.
        assert_eq!(julian::to_fixed(622, 3, 19), Ok(EPOCH));
        assert_eq!(gregorian::from_fixed(EPOCH), Ok((622, 3, 22)));
        assert_eq!(to_fixed(1, 1, 1), Ok(EPOCH));
        assert_eq!(from_fixed(EPOCH), Ok((1, 1, 1)));
    }

    #[test]
    fn nowruz_falls_on_the_twentieth_or_twenty_first_of_march() {
        // 1 Farvardin 1399 was 20 March 2020 and 1400 was 21 March 2021, as
        // published. 1404 is where the cycle parts from the record: Iran kept
        // it on 21 March 2025, and the cycle says the 20th — the astronomical
        // calendar in `hc-calendars-equinox` has the published date.
        for (persian_year, expected) in [
            (1399, (2020, 3, 20)),
            (1400, (2021, 3, 21)),
            (1404, (2025, 3, 20)),
        ] {
            let nowruz = to_fixed(persian_year, 1, 1).unwrap();
            assert_eq!(
                gregorian::from_fixed(nowruz),
                Ok(expected),
                "Nowruz {persian_year}"
            );
        }
    }

    #[test]
    fn the_islamic_revolution_is_dated_as_published() {
        // 22 Bahman 1357, the anniversary of the revolution, is
        // 11 February 1979.
        assert_eq!(
            to_fixed(1357, 11, 22),
            Ok(gregorian::to_fixed(1979, 2, 11).unwrap())
        );
    }

    #[test]
    fn the_leap_rule_and_the_year_length_agree_everywhere() {
        // The strongest available check on the cycle: the leap predicate and
        // the difference between consecutive new years must never disagree,
        // across the whole supported range and both cycle boundaries.
        for year in MIN_YEAR..MAX_YEAR {
            let start = to_fixed(year, 1, 1).unwrap();
            let next = to_fixed(year + 1, 1, 1).unwrap();
            let length = next.0 - start.0;
            assert_eq!(
                length,
                i64::from(days_in_year(year)),
                "year {year} measured {length}"
            );
        }
    }

    #[test]
    fn the_cycle_contains_six_hundred_and_eighty_three_leap_years() {
        let leaps = (475..475 + CYCLE_YEARS)
            .filter(|year| is_leap_year(*year))
            .count();
        assert_eq!(leaps, 683);
        // Which gives the mean year the cycle is famous for.
        let mean = CYCLE_DAYS as f64 / CYCLE_YEARS as f64;
        assert!((mean - 365.242_198_58).abs() < 1e-8, "mean year {mean}");
    }

    #[test]
    fn the_months_are_six_long_then_five_short_then_one_shorter() {
        for month in 1..=6u8 {
            assert_eq!(days_in_month(1400, month), Some(31));
        }
        for month in 7..=11u8 {
            assert_eq!(days_in_month(1400, month), Some(30));
        }
        assert_eq!(days_in_month(1400, 12), Some(29));
        // 1399 ran from 20 March 2020 to 20 March 2021 inclusive, 366 days,
        // so Esfand 1399 had thirty.
        assert!(is_leap_year(1399));
        assert!(!is_leap_year(1403));
        assert_eq!(days_in_month(1399, 12), Some(30));
        assert_eq!(days_in_month(1400, 13), None);
        assert_eq!(to_fixed(1400, 12, 30), Err(CalendarError::DayOutOfRange));
        assert!(to_fixed(1399, 12, 30).is_ok());
    }

    #[test]
    fn every_day_of_a_wide_range_round_trips() {
        for rd in (EARLIEST.0..=LATEST.0).step_by(101) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn every_single_day_of_a_century_round_trips() {
        let start = to_fixed(1350, 1, 1).unwrap().0;
        let end = to_fixed(1450, 1, 1).unwrap().0;
        for rd in start..end {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn every_single_day_across_a_cycle_boundary_round_trips() {
        // Year 3294 = 474 + 2820 closes the first cycle counted from 474.
        let start = to_fixed(3_290, 1, 1).unwrap().0;
        let end = to_fixed(3_300, 1, 1).unwrap().0;
        for rd in start..end {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = ArithmeticPersianCalendar;
        for rd in (EARLIEST.0..=LATEST.0).step_by(919) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(fields.era, Some(ERA));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
        assert_eq!(calendar.meta().id, CalendarId("persian-arithmetic"));
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
            to_fixed(MAX_YEAR + 1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert!(PersianDate::new(1400, 1, 1).unwrap().is_nowruz());
        assert!(!PersianDate::new(1400, 1, 2).unwrap().is_nowruz());
    }
}
