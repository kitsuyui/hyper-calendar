//! The Liberalia Triday Calendar of Peter Meyer (1999), lunar form.
//!
//! The lunar half of Meyer's triday calendar, on the solar half's epoch
//! and tridays ([`hc_calendars_solar::liberalia`]): a purely lunar year of
//! twelve months, Armedon to Eleleth, each of ten tridays, 30 days, except
//! the sixth, Oraiel, of nine, and the twelfth of nine — or ten when the
//! year of the cycle less 2 is divisible by 8, except year 2 itself. A
//! cycle of 384 years therefore holds 47 long years, 136 077 days and
//! 4 608 months, a mean month of 29.530599 days; the calendar claims to
//! follow the dark moon to within four days, and nothing tests it against
//! the sky. `0-000-01-01-1 LLT`, cycle 0, year 0, Armedon, triday 1,
//! Sophiesday, is Julian Day Number 2 416 557, 17 March 1904, the day of a
//! central annular eclipse.
//!
//! It lives in this crate because it is lunar; it takes its epoch, the
//! triday's day names and the triday arithmetic from the solar module,
//! which defines them. The date's year is the continuous count
//! `384 · cycle + year`, so `98-12-05-1 LLT` is year 98 and
//! `-1-383-12-09-1 LLT` year −1; `cycle` and `year-of-cycle` are fields
//! beside it, as the Meyer–Palmen calendar carries its sixty-year cycles.
//! The triday of the month and the day of the triday are the `triday` and
//! `day-of-triday` fields. The system document is
//! `docs/systems/liberalia-triday.md`.
//!
//! # Sources
//!
//! * Peter Meyer, "The Liberalia Triday Calendar", Hermetic Systems,
//!   <https://www.hermetic.ch/cal_stud/ltc/ltc.htm>, final version of
//!   14 November 1999, retrieved 2026-09-26 (`meyer-liberalia-triday`): the
//!   lunar rule and its table, the month names, the correlation and its
//!   tables, and the dated examples.
//!
//! # Exactness
//!
//! Exact: the rules are the definition.

use hc_calendar::shape::{CycleShape, MONTH};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};
use hc_calendars_solar::liberalia::{EPOCH, TRIDAY_CYCLE, triday_of};

/// The calendar identifier.
pub const ID: &str = "liberalia-triday-lunar";

/// The month names, as Meyer gives them.
pub const MONTHS: [&str; 12] = [
    "Armedon",
    "Nousanios",
    "Harmozel",
    "Phaionios",
    "Ainios",
    "Oraiel",
    "Mellephaneus",
    "Loios",
    "Davithe",
    "Mousanios",
    "Amethes",
    "Eleleth",
];

/// Years in a cycle.
pub const CYCLE_YEARS: i64 = 384;

/// Days in a cycle: 384 years of 354 days and 47 long Eleleths.
pub const CYCLE_DAYS: i64 = 354 * CYCLE_YEARS + 3 * 47;

/// The earliest cycle this implementation converts.
pub const MIN_CYCLE: i64 = -260;

/// The latest cycle this implementation converts.
pub const MAX_CYCLE: i64 = 260;

/// The earliest continuous year converted: year 0 of [`MIN_CYCLE`].
pub const MIN_YEAR: i64 = CYCLE_YEARS * MIN_CYCLE;

/// The latest continuous year converted: year 383 of [`MAX_CYCLE`].
pub const MAX_YEAR: i64 = CYCLE_YEARS * (MAX_CYCLE + 1) - 1;

/// The cycle and the year of the cycle, 0 to 383, of a continuous year.
#[must_use]
pub const fn cycle_and_year(year: i64) -> (i64, u16) {
    (
        year.div_euclid(CYCLE_YEARS),
        year.rem_euclid(CYCLE_YEARS) as u16,
    )
}

/// Whether `year` is long, its Eleleth of ten tridays: the year of the
/// cycle less 2 divisible by 8, and not year 2.
#[must_use]
pub const fn is_long_year(year: i64) -> bool {
    let of_cycle = year.rem_euclid(CYCLE_YEARS);
    of_cycle != 2 && (of_cycle - 2).rem_euclid(8) == 0
}

/// Days from the epoch to the first day of `year`.
const fn days_before_year(year: i64) -> i64 {
    let (cycle, of_cycle) = cycle_and_year(year);
    let of_cycle = of_cycle as i64;
    // The long years of a cycle are 10, 18, …, 378.
    let long = if of_cycle < 3 { 0 } else { (of_cycle - 3) / 8 };
    CYCLE_DAYS * cycle + 354 * of_cycle + 3 * long
}

/// The number of tridays in `month` of `year`, or `None` outside `1..=12`.
#[must_use]
pub const fn tridays_in_month(year: i64, month: u8) -> Option<u8> {
    match month {
        6 => Some(9),
        12 => Some(if is_long_year(year) { 10 } else { 9 }),
        1..=11 => Some(10),
        _ => None,
    }
}

/// The number of days in `month` of `year`, or `None` outside `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    match tridays_in_month(year, month) {
        Some(tridays) => Some(3 * tridays),
        None => None,
    }
}

/// The number of days in `year`, 354 or 357.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_long_year(year) { 357 } else { 354 }
}

/// Days before the first of `month`: thirty a month, three fewer after
/// Oraiel.
const fn days_before_month(month: u8) -> i64 {
    let before = 30 * (month as i64 - 1);
    if month > 6 { before - 3 } else { before }
}

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = Rd(EPOCH.0 + days_before_year(MIN_YEAR));

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(EPOCH.0 + days_before_year(MAX_YEAR + 1) - 1);

/// The fixed day of the first day of `year`, a Sophiesday.
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

/// The fixed day of a lunar date, its year the continuous count.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    let start = match new_year(year) {
        Ok(start) => start,
        Err(error) => return Err(error),
    };
    match days_in_month(year, month) {
        None => Err(CalendarError::MonthOutOfRange),
        Some(length) if day == 0 || day > length => Err(CalendarError::DayOutOfRange),
        Some(_) => Ok(Rd(start.0 + days_before_month(month) + day as i64 - 1)),
    }
}

/// The continuous year, month and day of a fixed day.
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
    let within = elapsed - days_before_year(year);
    // Five months of 30 days, Oraiel of 27 from day 150, then six more of
    // 30 from day 177, the last of 27 or 30.
    let month = if within < 150 {
        within / 30 + 1
    } else if within < 177 {
        6
    } else {
        (within - 177) / 30 + 7
    } as u8;
    Ok((year, month, (within - days_before_month(month) + 1) as u8))
}

/// A lunar Liberalia Triday date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiberaliaLunarDate {
    /// The continuous year, `384 · cycle + year`: 98 for `98-12-05-1 LLT`.
    pub year: i64,
    /// The month, 1 for Armedon through 12 for Eleleth.
    pub month: u8,
    /// The day of the month, 1 through 27 or 30.
    pub day: u8,
}

impl LiberaliaLunarDate {
    /// A validated date from its continuous year.
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

    /// A validated date as Meyer writes it, cycle-year-month-triday-day,
    /// so `0-098-12-05-1 LLT` is `from_written(0, 98, 12, 5, 1)`.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] for a year of the cycle
    /// past 383, [`CalendarError::DayOutOfRange`] for a triday or day of
    /// the triday the month does not have, and otherwise as
    /// [`LiberaliaLunarDate::new`].
    pub const fn from_written(
        cycle: i64,
        year: u16,
        month: u8,
        triday: u8,
        day: u8,
    ) -> CalendarResult<Self> {
        if year as i64 >= CYCLE_YEARS {
            return Err(CalendarError::YearOutOfRange);
        }
        if day == 0 || day > 3 || triday == 0 || triday > 10 {
            return Err(CalendarError::DayOutOfRange);
        }
        Self::new(
            CYCLE_YEARS * cycle + year as i64,
            month,
            3 * (triday - 1) + day,
        )
    }

    /// The 384-year cycle.
    #[must_use]
    pub const fn cycle(self) -> i64 {
        cycle_and_year(self.year).0
    }

    /// The year of the cycle, 0 to 383.
    #[must_use]
    pub const fn year_of_cycle(self) -> u16 {
        cycle_and_year(self.year).1
    }

    /// The triday of the month, 1 to 9 or 10.
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

/// The Liberalia Triday Calendar, lunar form.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LiberaliaLunarCalendar;

/// Twelve named months and the named days of the triday.
const SHAPE: &[CycleShape] = &[CycleShape::named(MONTH, &MONTHS), TRIDAY_CYCLE];

impl Calendar for LiberaliaLunarCalendar {
    type Date = LiberaliaLunarDate;

    /// Unrecorded: a proposal, adopted by nobody.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// The months and the triday.
    fn cycles(&self) -> &'static [CycleShape] {
        SHAPE
    }

    /// A long year, whose Eleleth has ten tridays.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(is_long_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "Liberalia Triday, lunar",
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &[],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(LiberaliaLunarDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.month, date.day)
            .with_extra("cycle", date.cycle())?
            .with_extra("year-of-cycle", i64::from(date.year_of_cycle()))?
            .with_extra("triday", i64::from(date.triday()))?
            .with_extra("day-of-triday", i64::from(date.day_of_triday()))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        LiberaliaLunarDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::gregorian;
    use hc_calendars_solar::liberalia;

    /// The lunar date as Meyer writes it.
    fn written(rd: Rd) -> (i64, u16, u8, u8, u8) {
        let date = LiberaliaLunarCalendar.from_fixed(rd).unwrap();
        (
            date.cycle(),
            date.year_of_cycle(),
            date.month,
            date.triday(),
            date.day_of_triday(),
        )
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
        for (number, expected) in [
            (2_416_554, (-1, 383, 12, 9, 1)),
            (2_416_555, (-1, 383, 12, 9, 2)),
            (2_416_556, (-1, 383, 12, 9, 3)),
            (2_416_557, (0, 0, 1, 1, 1)),
            (2_416_558, (0, 0, 1, 1, 2)),
            (2_416_559, (0, 0, 1, 1, 3)),
            (2_416_560, (0, 0, 1, 2, 1)),
            (2_416_561, (0, 0, 1, 2, 2)),
            (2_416_562, (0, 0, 1, 2, 3)),
            (2_416_563, (0, 0, 1, 3, 1)),
            (2_416_564, (0, 0, 1, 3, 2)),
            (2_416_565, (0, 0, 1, 3, 3)),
        ] {
            assert_eq!(written(jdn(number)), expected, "JDN {number}");
        }
    }

    /// The lunar dates of Meyer's table of solar New Year's Days.
    #[test]
    fn the_lunar_dates_of_the_solar_new_years_of_2000_to_2015() {
        for (number, lunar) in [
            (2_451_621, (0, 98, 12, 5, 1)),
            (2_451_987, (0, 99, 12, 8, 1)),
            (2_452_353, (0, 101, 1, 3, 1)),
            (2_452_719, (0, 102, 1, 7, 1)),
            (2_453_082, (0, 103, 1, 10, 1)),
            (2_453_448, (0, 104, 2, 4, 1)),
            (2_453_814, (0, 105, 2, 8, 1)),
            (2_454_180, (0, 106, 3, 2, 1)),
            (2_454_543, (0, 107, 3, 4, 1)),
            (2_454_909, (0, 108, 3, 8, 1)),
            (2_455_275, (0, 109, 4, 2, 1)),
            (2_455_641, (0, 110, 4, 6, 1)),
            (2_456_004, (0, 111, 4, 9, 1)),
            (2_456_370, (0, 112, 5, 3, 1)),
            (2_456_736, (0, 113, 5, 7, 1)),
            (2_457_102, (0, 114, 6, 1, 1)),
        ] {
            assert_eq!(written(jdn(number)), lunar, "JDN {number}");
        }
    }

    /// The combined dates in the text, solar and lunar halves together.
    #[test]
    fn the_dated_examples() {
        for ((year, month, day), lunar, solar) in [
            // "Zoesday, 9 Mellephaneus 98, 15 Samlo 95", 1 November 1999.
            ((1999, 11, 1), (0, 98, 7, 9, 2), Some((95, 3, 15))),
            // "Norasday, 3 Loios 98, 19 Samlo 95", 14 November 1999.
            ((1999, 11, 14), (0, 98, 8, 3, 3), Some((95, 3, 19))),
            // Hofmann's birth, 1-11-05-3 LLT and 1-4-09-3 SLT, and his lunar
            // 100th birthday, 101-11-05-3 LLT on 2003-01-18, for which Meyer
            // gives no solar date.
            ((1906, 1, 11), (0, 1, 11, 5, 3), Some((1, 4, 9))),
            ((2003, 1, 18), (0, 101, 11, 5, 3), None),
            // Bicycle Day: "Sophiesday, 6 Phaionios 40, 11 Kamaliel 39".
            ((1943, 4, 19), (0, 40, 4, 6, 1), Some((39, 1, 11))),
        ] {
            let rd = gregorian(year, month, day);
            assert_eq!(written(rd), lunar, "{year}-{month}-{day}");
            if let Some(solar) = solar {
                let (solar_year, quarter, day_of_quarter) = liberalia::from_fixed(rd).unwrap();
                assert_eq!(
                    (solar_year, quarter, triday_of(day_of_quarter).0),
                    solar,
                    "{year}-{month}-{day}"
                );
            }
        }
        // "Zoesday, 5 Loios 98, 21 Samlo 95" and "Norasday, 9 Davithe 98,
        // 5 Abrasax 95" name the same days in both.
        for (lunar, solar) in [
            ((98, 8, 5, 2), (95, 3, 21, 2)),
            ((98, 9, 9, 3), (95, 4, 5, 3)),
        ] {
            let lunar =
                LiberaliaLunarDate::from_written(0, lunar.0, lunar.1, lunar.2, lunar.3).unwrap();
            let solar =
                liberalia::LiberaliaSolarDate::from_triday(solar.0, solar.1, solar.2, solar.3)
                    .unwrap();
            assert_eq!(
                LiberaliaLunarCalendar.to_fixed(lunar),
                liberalia::to_fixed(solar.year, solar.quarter, solar.day)
            );
        }
        // "111-04-07-3 LLT is Norasday".
        let date = LiberaliaLunarDate::from_written(0, 111, 4, 7, 3).unwrap();
        assert_eq!(
            liberalia::DAYS[usize::from(date.day_of_triday()) - 1],
            "Norasday"
        );
    }

    /// The table in section 4: ten tridays a month, nine in Oraiel, and
    /// "10th triday of Eleleth is included only if year is not 2 and
    /// year-2 is divisible by 8".
    #[test]
    fn the_month_table() {
        for year in [0, 1, 2, 3, 9, 11, 383] {
            let tridays =
                core::array::from_fn::<_, 12, _>(|index| tridays_in_month(year, index as u8 + 1));
            let mut expected = [Some(10); 12];
            expected[5] = Some(9);
            expected[11] = Some(9);
            assert_eq!(tridays, expected, "{year}");
        }
        for year in [10, 18, 378, 384 + 10, -374] {
            assert_eq!(tridays_in_month(year, 12), Some(10), "{year}");
        }
        assert_eq!(tridays_in_month(0, 13), None);
        assert_eq!(
            (0..CYCLE_YEARS).filter(|year| is_long_year(*year)).count(),
            47
        );
        assert_eq!(to_fixed(2, 12, 28), Err(CalendarError::DayOutOfRange));
        assert!(to_fixed(10, 12, 30).is_ok());
        assert_eq!(to_fixed(10, 13, 1), Err(CalendarError::MonthOutOfRange));
    }

    /// "the total number of days in one cycle is 136,077", 4 608 months.
    #[test]
    fn the_cycle_is_136_077_days() {
        assert_eq!(CYCLE_DAYS, 136_077);
        assert_eq!(days_before_year(CYCLE_YEARS), CYCLE_DAYS);
        for year in -2_000..2_000 {
            let length = days_before_year(year + 1) - days_before_year(year);
            assert_eq!(length, i64::from(days_in_year(year)), "{year}");
        }
    }

    #[test]
    fn every_day_round_trips() {
        for rd in (EARLIEST.0..=LATEST.0).step_by(1_013) {
            let date = LiberaliaLunarCalendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(LiberaliaLunarCalendar.to_fixed(date), Ok(Rd(rd)));
            let fields = LiberaliaLunarCalendar.to_fields(date).unwrap();
            assert_eq!(LiberaliaLunarCalendar.from_fields(&fields), Ok(date));
        }
        for rd in EPOCH.0 - 3_000..EPOCH.0 + 3_000 {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)));
        }
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(
            LiberaliaLunarDate::from_written(0, 384, 1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
    }
}
