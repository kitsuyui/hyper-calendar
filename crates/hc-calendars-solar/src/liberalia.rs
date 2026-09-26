//! The Liberalia Triday Calendar of Peter Meyer (1999), solar form.
//!
//! Meyer's proposal of November 1999 replaces the week with the *triday*,
//! three days named Sophiesday, Zoesday and Norasday, and builds two
//! independent calendars of whole tridays on one epoch: a solar one of
//! four quarters and a lunar one of twelve months, the second in
//! `hc-calendars-lunar`. In the solar calendar the first and third
//! quarters, Kamaliel and Samlo, have 30 tridays, the second, Gabriel, 31,
//! and the fourth, Abrasax, 31 — or 30 when the year number plus one is
//! divisible by 4 or by 198. A year is therefore 366 days or 363, and 396
//! years are 144 636 days, a mean year of 365.2424̄ days. Solar year 0,
//! quarter 1, triday 1, day 1 (`0-1-01-1 SLT`) is Julian Day Number
//! 2 416 557, 17 March 1904, the day of an annular eclipse four days before
//! the equinox, and the feast of the Liberalia. Years are any integer.
//!
//! A date here is the year, the quarter as its month and the day of the
//! quarter, 1 to 93, as its day; the triday of the quarter and the day of
//! the triday, which are what Meyer writes, are the `triday` and
//! `day-of-triday` extra fields, and the day's name is the `triday-day`
//! cycle. The system document is `docs/systems/liberalia-triday.md`.
//!
//! # Sources
//!
//! * Peter Meyer, "The Liberalia Triday Calendar", Hermetic Systems,
//!   <https://www.hermetic.ch/cal_stud/ltc/ltc.htm>, final version of
//!   14 November 1999, retrieved 2026-09-26 (`meyer-liberalia-triday`): the
//!   definition, the names, the tables of the solar and lunar calendars,
//!   the correlation with JDN 2 416 557 and its two tables of dates, and
//!   the dated examples.
//!
//! # Exactness
//!
//! Exact: the rules are the definition. The day runs from midnight to
//! midnight, as Meyer defines it.

use hc_calendar::shape::{CycleShape, MONTH};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

/// The calendar identifier.
pub const ID: &str = "liberalia-triday-solar";

/// The day both Liberalia Triday calendars begin on: `0-1-01-1 SLT` and
/// `0-000-01-01-1 LLT`, Julian Day Number 2 416 557, 17 March 1904.
pub const EPOCH: Rd = Rd::from_julian_day_number(2_416_557);

/// The days of the triday, as Meyer names them.
pub const DAYS: [&str; 3] = ["Sophiesday", "Zoesday", "Norasday"];

/// The cycle kind of the day of the triday, which both calendars declare.
pub const TRIDAY_DAY: &str = "triday-day";

/// The quarters of the solar year, as Meyer names them.
pub const QUARTERS: [&str; 4] = ["Kamaliel", "Gabriel", "Samlo", "Abrasax"];

/// Years in the solar cycle, 2 · 198.
pub const CYCLE_YEARS: i64 = 396;

/// Days in the solar cycle.
pub const CYCLE_DAYS: i64 = 144_636;

/// The earliest solar year this implementation converts.
pub const MIN_YEAR: i64 = -99_999;

/// The latest solar year this implementation converts.
pub const MAX_YEAR: i64 = 99_999;

/// The triday of a period and the day within it, both from 1, of a day
/// numbered from 1 in a period that begins on a Sophiesday.
#[must_use]
pub const fn triday_of(day: u8) -> (u8, u8) {
    ((day - 1) / 3 + 1, (day - 1) % 3 + 1)
}

/// The cycle of the triday's days, named as Meyer names them.
pub const TRIDAY_CYCLE: CycleShape = CycleShape::named(TRIDAY_DAY, &DAYS);

/// Whether the fourth quarter of `year` is short, 30 tridays: when
/// `year + 1` is divisible by 4 or by 198.
#[must_use]
pub const fn is_short_year(year: i64) -> bool {
    (year + 1).rem_euclid(4) == 0 || (year + 1).rem_euclid(198) == 0
}

/// Short years from year 0 up to but not including `year`: the multiples
/// of 4 or of 198 among `1..=year`, negative before year 0.
const fn short_years_before(year: i64) -> i64 {
    year.div_euclid(4) + year.div_euclid(198) - year.div_euclid(CYCLE_YEARS)
}

/// Days from the epoch to the first day of `year`.
const fn days_before_year(year: i64) -> i64 {
    366 * year - 3 * short_years_before(year)
}

/// The number of tridays in `quarter` of `year`, or `None` outside `1..=4`.
#[must_use]
pub const fn tridays_in_quarter(year: i64, quarter: u8) -> Option<u8> {
    match quarter {
        1 | 3 => Some(30),
        2 => Some(31),
        4 => Some(if is_short_year(year) { 30 } else { 31 }),
        _ => None,
    }
}

/// The number of days in `quarter` of `year`, or `None` outside `1..=4`.
#[must_use]
pub const fn days_in_quarter(year: i64, quarter: u8) -> Option<u8> {
    match tridays_in_quarter(year, quarter) {
        Some(tridays) => Some(3 * tridays),
        None => None,
    }
}

/// The number of days in `year`, 363 or 366.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_short_year(year) { 363 } else { 366 }
}

/// Days before the first day of `quarter`: 90, 93 and 90 days.
const fn days_before_quarter(quarter: u8) -> i64 {
    match quarter {
        1 => 0,
        2 => 90,
        3 => 183,
        _ => 273,
    }
}

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = Rd(EPOCH.0 + days_before_year(MIN_YEAR));

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(EPOCH.0 + days_before_year(MAX_YEAR + 1) - 1);

/// The fixed day of the first day of `year`, always a Sophiesday.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn new_year(year: i64) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    Ok(Rd(EPOCH.0 + days_before_year(year)))
}

/// The fixed day of a solar date given as year, quarter and day of the
/// quarter.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] for a quarter outside `1..=4`, or
/// [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, quarter: u8, day: u8) -> CalendarResult<Rd> {
    let start = match new_year(year) {
        Ok(start) => start,
        Err(error) => return Err(error),
    };
    match crate::common::check_day(day, days_in_quarter(year, quarter)) {
        Err(error) => Err(error),
        Ok(()) => Ok(Rd(start.0 + days_before_quarter(quarter) + day as i64 - 1)),
    }
}

/// The year, quarter and day of the quarter of a fixed day.
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
    let elapsed = rd.0 - EPOCH.0;
    let mut year = (elapsed * CYCLE_YEARS).div_euclid(CYCLE_DAYS);
    while days_before_year(year) > elapsed {
        year -= 1;
    }
    while days_before_year(year + 1) <= elapsed {
        year += 1;
    }
    let day_of_year = elapsed - days_before_year(year);
    let quarter = if day_of_year < 90 {
        1
    } else if day_of_year < 183 {
        2
    } else if day_of_year < 273 {
        3
    } else {
        4
    };
    Ok((
        year,
        quarter,
        (day_of_year - days_before_quarter(quarter) + 1) as u8,
    ))
}

/// A solar Liberalia Triday date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiberaliaSolarDate {
    /// The solar year, any integer.
    pub year: i64,
    /// The quarter, 1 for Kamaliel through 4 for Abrasax.
    pub quarter: u8,
    /// The day of the quarter, 1 through 90 or 93.
    pub day: u8,
}

impl LiberaliaSolarDate {
    /// A validated date from its day of the quarter.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    pub const fn new(year: i64, quarter: u8, day: u8) -> CalendarResult<Self> {
        match to_fixed(year, quarter, day) {
            Err(error) => Err(error),
            Ok(_) => Ok(Self { year, quarter, day }),
        }
    }

    /// A validated date as Meyer writes it, `year-quarter-triday-day`, so
    /// `95-3-15-2 SLT` is `from_triday(95, 3, 15, 2)`.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::DayOutOfRange`] for a day of the triday
    /// outside `1..=3` or a triday the quarter does not have, and otherwise
    /// as [`LiberaliaSolarDate::new`].
    pub const fn from_triday(year: i64, quarter: u8, triday: u8, day: u8) -> CalendarResult<Self> {
        if day == 0 || day > 3 || triday == 0 || triday > 31 {
            return Err(CalendarError::DayOutOfRange);
        }
        Self::new(year, quarter, 3 * (triday - 1) + day)
    }

    /// The triday of the quarter, 1 to 30 or 31.
    #[must_use]
    pub const fn triday(self) -> u8 {
        triday_of(self.day).0
    }

    /// The day of the triday, 1 for Sophiesday to 3 for Norasday.
    #[must_use]
    pub const fn day_of_triday(self) -> u8 {
        triday_of(self.day).1
    }
}

/// The Liberalia Triday Calendar, solar form.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LiberaliaSolarCalendar;

/// Four named quarters in the month slot, and the named days of the triday.
const SHAPE: &[CycleShape] = &[CycleShape::named(MONTH, &QUARTERS), TRIDAY_CYCLE];

impl Calendar for LiberaliaSolarCalendar {
    type Date = LiberaliaSolarDate;

    /// Unrecorded: a proposal, adopted by nobody.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// The quarters and the triday.
    fn cycles(&self) -> &'static [CycleShape] {
        SHAPE
    }

    /// The calendar's intercalary unit is a triday, and the year that
    /// carries it is the long one, of 366 days; a short year of 363 drops
    /// it. So this answers whether the year is long.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(!is_short_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "Liberalia Triday, solar",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &[],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.quarter, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, quarter, day) = from_fixed(rd)?;
        Ok(LiberaliaSolarDate { year, quarter, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.quarter, date.day)
            .with_extra("triday", i64::from(date.triday()))?
            .with_extra("day-of-triday", i64::from(date.day_of_triday()))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let quarter = fields.require_month()?;
        if quarter.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        LiberaliaSolarDate::new(fields.year, quarter.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian;

    /// The solar date as Meyer writes it.
    fn written(rd: Rd) -> (i64, u8, u8, u8) {
        let (year, quarter, day) = from_fixed(rd).unwrap();
        let (triday, day) = triday_of(day);
        (year, quarter, triday, day)
    }

    fn jdn(number: i64) -> Rd {
        Rd::from_julian_day_number(number)
    }

    fn gregorian(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    /// Meyer's correlation table, JDN 2 416 554 to 2 416 565.
    #[test]
    fn the_correlation_table_around_the_epoch() {
        assert_eq!(EPOCH, gregorian(1904, 3, 17));
        for (number, expected) in [
            (2_416_554, (-1, 4, 30, 1)),
            (2_416_555, (-1, 4, 30, 2)),
            (2_416_556, (-1, 4, 30, 3)),
            (2_416_557, (0, 1, 1, 1)),
            (2_416_558, (0, 1, 1, 2)),
            (2_416_559, (0, 1, 1, 3)),
            (2_416_560, (0, 1, 2, 1)),
            (2_416_561, (0, 1, 2, 2)),
            (2_416_562, (0, 1, 2, 3)),
            (2_416_563, (0, 1, 3, 1)),
            (2_416_564, (0, 1, 3, 2)),
            (2_416_565, (0, 1, 3, 3)),
        ] {
            assert_eq!(written(jdn(number)), expected, "JDN {number}");
        }
    }

    /// Meyer's table of the solar New Year's Days of 2000 to 2015.
    #[test]
    fn the_new_years_of_2000_to_2015() {
        for (number, (year, month, day), solar) in [
            (2_451_621, (2000, 3, 17), 96),
            (2_451_987, (2001, 3, 18), 97),
            (2_452_353, (2002, 3, 19), 98),
            (2_452_719, (2003, 3, 20), 99),
            (2_453_082, (2004, 3, 17), 100),
            (2_453_448, (2005, 3, 18), 101),
            (2_453_814, (2006, 3, 19), 102),
            (2_454_180, (2007, 3, 20), 103),
            (2_454_543, (2008, 3, 17), 104),
            (2_454_909, (2009, 3, 18), 105),
            (2_455_275, (2010, 3, 19), 106),
            (2_455_641, (2011, 3, 20), 107),
            (2_456_004, (2012, 3, 17), 108),
            (2_456_370, (2013, 3, 18), 109),
            (2_456_736, (2014, 3, 19), 110),
            (2_457_102, (2015, 3, 20), 111),
        ] {
            assert_eq!(jdn(number), gregorian(year, month, day));
            assert_eq!(new_year(solar), Ok(jdn(number)), "{solar}");
        }
    }

    /// The dates Meyer gives in the text.
    #[test]
    fn the_dated_examples() {
        for ((year, month, day), expected) in [
            // First and final publication, 1 and 14 November 1999.
            ((1999, 11, 1), (95, 3, 15, 2)),
            ((1999, 11, 14), (95, 3, 19, 3)),
            // Albert Hofmann's birth, 1906-01-11, 1-4-09-3 SLT; his 94th
            // solar birthday, 95-4-09-3 SLT, 2000-01-13.
            ((1906, 1, 11), (1, 4, 9, 3)),
            ((2000, 1, 13), (95, 4, 9, 3)),
            // Bicycle Day, 1943-04-19: Sophiesday, 11 Kamaliel 39.
            ((1943, 4, 19), (39, 1, 11, 1)),
        ] {
            assert_eq!(
                written(gregorian(year, month, day)),
                expected,
                "{year}-{month}-{day}"
            );
        }
        // "Zoesday, 5 Loios 98, 21 Samlo 95" and "Norasday, 9 Davithe 98,
        // 5 Abrasax 95" are in the lunar module's tests with the lunar
        // halves; here, that they are solar dates that exist.
        assert!(LiberaliaSolarDate::from_triday(95, 3, 21, 2).is_ok());
        assert!(LiberaliaSolarDate::from_triday(95, 4, 5, 3).is_ok());
        // "110-3-28-2 SLT is Zoesday".
        let date = LiberaliaSolarDate::from_triday(110, 3, 28, 2).unwrap();
        assert_eq!(DAYS[usize::from(date.day_of_triday()) - 1], "Zoesday");
    }

    /// The tables in section 4: quarters of 30, 31, 30 and 31 tridays, the
    /// 31st triday of Abrasax omitted "if year number + 1 is divisible by 4
    /// or by 198".
    #[test]
    fn the_quarter_table() {
        assert_eq!(QUARTERS, ["Kamaliel", "Gabriel", "Samlo", "Abrasax"]);
        for year in [0, 1, 2, 4, 196, 198] {
            assert_eq!(
                [1, 2, 3, 4].map(|quarter| tridays_in_quarter(year, quarter)),
                [Some(30), Some(31), Some(30), Some(31)],
                "{year}"
            );
        }
        for year in [-1, 3, 7, 99, 197, 395] {
            assert_eq!(tridays_in_quarter(year, 4), Some(30), "{year}");
        }
        assert_eq!(tridays_in_quarter(0, 5), None);
        assert_eq!(to_fixed(3, 4, 91), Err(CalendarError::DayOutOfRange));
        assert!(to_fixed(4, 4, 93).is_ok());
        assert_eq!(to_fixed(4, 5, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(
            LiberaliaSolarDate::from_triday(4, 1, 1, 4),
            Err(CalendarError::DayOutOfRange)
        );
    }

    /// "the total number of days in 396 solar years is 144,636".
    #[test]
    fn the_cycle_is_144_636_days() {
        assert_eq!(days_before_year(CYCLE_YEARS), CYCLE_DAYS);
        for start in [-792, -396, 0, 396, 1_188] {
            assert_eq!(
                days_before_year(start + CYCLE_YEARS) - days_before_year(start),
                CYCLE_DAYS
            );
        }
        for year in -1_000..1_000 {
            let length = days_before_year(year + 1) - days_before_year(year);
            assert_eq!(length, i64::from(days_in_year(year)), "{year}");
        }
    }

    #[test]
    fn every_day_round_trips() {
        for rd in (EARLIEST.0..=LATEST.0).step_by(1_009) {
            let date = LiberaliaSolarCalendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(LiberaliaSolarCalendar.to_fixed(date), Ok(Rd(rd)));
            let fields = LiberaliaSolarCalendar.to_fields(date).unwrap();
            assert_eq!(LiberaliaSolarCalendar.from_fields(&fields), Ok(date));
        }
        for rd in EPOCH.0 - 2_000..EPOCH.0 + 2_000 {
            let (year, quarter, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, quarter, day), Ok(Rd(rd)));
            // Every year and quarter begins on a Sophiesday.
            assert_eq!(triday_of(day).1, ((rd - EPOCH.0).rem_euclid(3) + 1) as u8);
        }
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }
}
