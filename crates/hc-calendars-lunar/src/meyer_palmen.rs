//! The Meyer–Palmen Solilunar Calendar.
//!
//! Peter Meyer's proposal of March 1999, built on a rule Karl Palmen posted
//! to the CALNDR-L list on 24 February 1999: a lunisolar calendar with no
//! astronomy in it at all. Odd months have 29 days and even months 30, a
//! *long* year adds a thirteenth month *Meton* of 30 or 31 days, and two
//! remainders decide which years are long and which Metons are 31 days:
//! with Y = 6840, L = 2519 and M = 1328, the year `n` (counted as
//! `60 · cycle + year`) is long when `n · L mod Y < L`, and the `k`th long
//! year, `k = ⌊n · L / Y⌋`, has a 31-day Meton when `k · M mod L < M`. An
//! *era* of 114 sixty-year cycles, 6840 years, then holds 2519 long years
//! and 1328 long Metons, 84 599 months and 2 498 258 days, a whole number of
//! weeks, and every era begins on a Sunday. Cycle 000, year 01, month 01,
//! day 01 is Julian Day Number 207 227, Sunday 8 April 4146 BC (proleptic
//! Gregorian), and New Year's Day falls between 6 March and 7 April across
//! the Common Era.
//!
//! It lives in this crate, not beside the arithmetic solar calendars,
//! because it is lunisolar: its months are lunations and its thirteenth
//! month is intercalated to hold the year to the equinox, which is the
//! structure of the Hebrew calendar here and not of any solar one.
//!
//! The date's year is the continuous count `60 · cycle + year`, the number
//! both leap rules take, so 102-25 is year 6145 and 000-01 is year 1;
//! `cycle` and `year-of-cycle` are fields beside it, as the Badíʿ calendar
//! carries its *Váḥid*. The system document is
//! `docs/systems/meyer-palmen.md`.
//!
//! # Sources
//!
//! * Peter Meyer, "The Meyer-Palmen Solilunar Calendar", Hermetic Systems,
//!   <https://www.hermetic.ch/cal_stud/nlsc/nlsc.htm>, retrieved
//!   2026-09-26: the four-number date, the month lengths and names, the two
//!   rules and their constants, the era, the base Julian Day Number
//!   207 227, the properties listed (era length, weeks, months, the eras
//!   beginning 2695-04-07 and 9535-04-07, 1795-03-20 as 099-01-01-01,
//!   1999-03-17 and 1999-08-11), the three correspondence tables, and the
//!   New Year's Day frequency table for 0–4000 CE.
//! * Karl Palmen, "Some Properties of the Meyer-Palmen Solilunar Calendar",
//!   Hermetic Systems, <https://www.hermetic.ch/cal_stud/nlsc/mpslci.htm>,
//!   retrieved 2026-09-26: the class of "YLM" calendars, the table of
//!   remainders, year lengths and New Year's Days for 102-25 to 102-44.
//! * Meyer, "MPSLC Year 102-25",
//!   <https://www.hermetic.ch/cal_stud/nlsc/mp102_25.htm>, retrieved
//!   2026-09-26: every day of the year 102-25, the output of Meyer's own
//!   converter.
//!
//! # Exactness
//!
//! Exact: the rules are the definition. The day begins at midnight; the
//! sources give Julian Day Numbers and civil dates and no other boundary.

use hc_calendar::shape::{CycleLength, CycleShape, EraName, MONTH, WEEKDAY};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

/// The calendar identifier.
pub const ID: &str = "meyer-palmen";

/// The era code: dates are written "102-25-01-01 MP".
pub const ERA: &str = "mp";

/// The month names, after "distinguished persons in the history of
/// calendars, mathematics, cosmology and astronomy", as Meyer gives them.
pub const MONTHS: [&str; 13] = [
    "Aristarchus",
    "Bruno",
    "Copernicus",
    "Dee",
    "Eratosthenes",
    "Flamsteed",
    "Galileo",
    "Hypatia",
    "Ibrahim",
    "Julius",
    "Khayyam",
    "Lilius",
    "Meton",
];

/// The month number of *Meton*, the thirteenth month of a long year.
pub const METON: u8 = 13;

/// Years in a cycle, the unit dates are written in.
pub const CYCLE_YEARS: i64 = 60;

/// Years in an era, Meyer's Y, after which the structure repeats.
pub const ERA_YEARS: i64 = 6_840;

/// Long years in an era, Meyer's L.
pub const ERA_LONG_YEARS: i64 = 2_519;

/// Long years in an era whose *Meton* has 31 days, Meyer's M.
pub const ERA_LONG_METONS: i64 = 1_328;

/// Cycles in an era.
pub const ERA_CYCLES: i64 = ERA_YEARS / CYCLE_YEARS;

/// Days in an era: 6840 years of 354 days, 2519 Metons of 30 and 1328
/// thirty-first days.
pub const ERA_DAYS: i64 = 354 * ERA_YEARS + 30 * ERA_LONG_YEARS + ERA_LONG_METONS;

/// Months in an era.
pub const ERA_MONTHS: i64 = 12 * ERA_YEARS + ERA_LONG_YEARS;

/// The fixed day of 000-01-01-01 MP, Julian Day Number 207 227, Sunday
/// 8 April 4146 BC in the proleptic Gregorian calendar.
pub const EPOCH: Rd = Rd::from_julian_day_number(207_227);

/// The earliest year this implementation converts: year 01 of cycle −114,
/// the first year of the era before era 0.
pub const MIN_YEAR: i64 = 1 - ERA_YEARS;

/// The latest year this implementation converts: year 60 of cycle 341, the
/// last year of era 2, which Meyer's page opens on 9535-04-07.
pub const MAX_YEAR: i64 = 3 * ERA_YEARS;

/// The continuous year of `year` in `cycle`: `60 · cycle + year`, the
/// number both rules take.
#[must_use]
pub const fn absolute_year(cycle: i64, year: u8) -> i64 {
    CYCLE_YEARS * cycle + year as i64
}

/// The cycle and the year within it, 1 to 60, of a continuous year.
#[must_use]
pub const fn cycle_and_year(year: i64) -> (i64, u8) {
    let elapsed = year - 1;
    (
        elapsed.div_euclid(CYCLE_YEARS),
        (elapsed.rem_euclid(CYCLE_YEARS) + 1) as u8,
    )
}

/// Whether `year` is long, with the thirteenth month *Meton*: rule (i),
/// `year · L mod Y < L`.
#[must_use]
pub const fn is_long_year(year: i64) -> bool {
    (year * ERA_LONG_YEARS).rem_euclid(ERA_YEARS) < ERA_LONG_YEARS
}

/// Long years before `year`, counted from year 1, negative before it.
///
/// `⌊n · L / Y⌋` rises by one exactly at a long year, so it counts them.
const fn long_years_before(year: i64) -> i64 {
    ((year - 1) * ERA_LONG_YEARS).div_euclid(ERA_YEARS)
}

/// Whether `year` is long and its *Meton* has 31 days: rule (ii), with
/// `k = ⌊year · L / Y⌋`, `k · M mod L < M`.
#[must_use]
pub const fn has_long_meton(year: i64) -> bool {
    is_long_year(year)
        && ((year * ERA_LONG_YEARS).div_euclid(ERA_YEARS) * ERA_LONG_METONS)
            .rem_euclid(ERA_LONG_YEARS)
            < ERA_LONG_METONS
}

/// The number of days in `month` of `year`, or `None` when the month does
/// not exist — month 13 exists only in a long year.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    match month {
        1..=12 => Some(if month % 2 == 1 { 29 } else { 30 }),
        METON if is_long_year(year) => Some(if has_long_meton(year) { 31 } else { 30 }),
        _ => None,
    }
}

/// The number of days in `year`: 354, or 384 or 385 in a long year.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if !is_long_year(year) {
        354
    } else if has_long_meton(year) {
        385
    } else {
        384
    }
}

/// Days from the epoch to the first day of `year`, unchecked.
///
/// 354 a year, 30 for each Meton before it and one for each 31st of Meton:
/// the long years before the `k`th are `⌊k · M / L⌋` by the same counting
/// argument as the long years themselves.
const fn days_before_year(year: i64) -> i64 {
    let long = long_years_before(year);
    354 * (year - 1) + 30 * long + (long * ERA_LONG_METONS).div_euclid(ERA_LONG_YEARS)
}

/// Days elapsed in the year before the first of `month`: pairs of 29 and 30.
const fn days_before_month(month: u8) -> i64 {
    let elapsed = month as i64 - 1;
    59 * (elapsed / 2) + 29 * (elapsed % 2)
}

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = Rd(EPOCH.0 + days_before_year(MIN_YEAR));

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(EPOCH.0 + days_before_year(MAX_YEAR + 1) - 1);

/// The fixed day of New Year's Day, 1 Aristarchus, of `year`.
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

/// The fixed day of a Meyer–Palmen date, its year the continuous count.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] — also for *Meton* in a short year —
/// or [`CalendarError::DayOutOfRange`].
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
    // The mean year is within a day of the estimate for every year, so
    // one step either way corrects it.
    let mut year = 1 + (elapsed * ERA_YEARS).div_euclid(ERA_DAYS);
    while days_before_year(year) > elapsed {
        year -= 1;
    }
    while days_before_year(year + 1) <= elapsed {
        year += 1;
    }
    let within = elapsed - days_before_year(year);
    // Two months are 59 days, so this is the month for every day of the
    // twelve; only Meton, from day 354, is past them.
    let month = if within >= 354 {
        METON
    } else {
        (2 * (within / 59) + if within % 59 >= 29 { 2 } else { 1 }) as u8
    };
    Ok((year, month, (within - days_before_month(month) + 1) as u8))
}

/// A Meyer–Palmen date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MeyerPalmenDate {
    /// The continuous year, `60 · cycle + year`: 6145 for 102-25.
    pub year: i64,
    /// The month, 1 for Aristarchus through 12 for Lilius, 13 for *Meton*.
    pub month: u8,
    /// The day of the month, 1 through 29, 30 or 31.
    pub day: u8,
}

impl MeyerPalmenDate {
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

    /// A validated date as the calendar writes it: cycle, year of the
    /// cycle, month and day, so 102-25-01-01 is
    /// `from_cycle(102, 25, 1, 1)`.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] for a year of the cycle
    /// outside 1 to 60, and otherwise as [`MeyerPalmenDate::new`].
    pub const fn from_cycle(cycle: i64, year: u8, month: u8, day: u8) -> CalendarResult<Self> {
        if year == 0 || year as i64 > CYCLE_YEARS {
            return Err(CalendarError::YearOutOfRange);
        }
        Self::new(absolute_year(cycle, year), month, day)
    }

    /// The sixty-year cycle.
    #[must_use]
    pub const fn cycle(self) -> i64 {
        cycle_and_year(self.year).0
    }

    /// The year within the cycle, 1 to 60.
    #[must_use]
    pub const fn year_of_cycle(self) -> u8 {
        cycle_and_year(self.year).1
    }
}

/// The Meyer–Palmen Solilunar Calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MeyerPalmenCalendar;

/// Twelve months and *Meton* in a long year, named as Meyer names them, and
/// the seven-day week.
const SHAPE: &[CycleShape] = &[
    CycleShape {
        kind: MONTH,
        length: CycleLength::Intercalary {
            ordinary: 12,
            extended: 13,
        },
        names: &MONTHS,
    },
    CycleShape::fixed(WEEKDAY, 7),
];

impl Calendar for MeyerPalmenCalendar {
    type Date = MeyerPalmenDate;

    /// Unrecorded: a proposal, adopted by nobody.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// Twelve named months and *Meton*, and the week.
    fn cycles(&self) -> &'static [CycleShape] {
        SHAPE
    }

    /// A long year, the one with *Meton*.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(is_long_year(year))
    }

    /// "MP", as Meyer suffixes the calendar's dates.
    fn era_name(&self, code: &str) -> Option<EraName> {
        (code == ERA).then_some(EraName::new("MP", ""))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "Meyer–Palmen Solilunar",
            year_kind: YearKind::Astronomical,
            has_leap_months: true,
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
        Ok(MeyerPalmenDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.month, date.day)
            .with_era(ERA)
            .with_extra("cycle", date.cycle())?
            .with_extra("year-of-cycle", i64::from(date.year_of_cycle()))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        MeyerPalmenDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::{Weekday, gregorian};

    /// The date as Meyer writes it, from a fixed day.
    fn written(rd: Rd) -> (i64, u8, u8, u8) {
        let (year, month, day) = from_fixed(rd).unwrap();
        let (cycle, of_cycle) = cycle_and_year(year);
        (cycle, of_cycle, month, day)
    }

    fn jdn(number: i64) -> Rd {
        Rd::from_julian_day_number(number)
    }

    #[test]
    fn the_epoch_is_julian_day_207_227_a_sunday_in_april_4146_bc() {
        assert_eq!(EPOCH.to_julian_day_number(), 207_227);
        assert_eq!(gregorian::from_fixed(EPOCH), Ok((-4_145, 4, 8)));
        assert_eq!(Weekday::from_rd(EPOCH), Weekday::Sunday);
        assert_eq!(to_fixed(absolute_year(0, 1), 1, 1), Ok(EPOCH));
    }

    /// The three correspondence tables on Meyer's page, every row.
    #[test]
    fn meyers_correspondence_tables_agree_row_by_row() {
        type Table = (i64, (i64, u8, u8), (i64, u8, u8, u8), u8);
        let tables: [Table; 3] = [
            // First JDN, its CE date, its MP date, and how many rows.
            (0, (-4_713, 11, 24), (-10, 33, 9, 21), 12),
            (2_415_016, (1_899, 12, 27), (100, 45, 10, 26), 10),
            (2_488_341, (2_100, 9, 29), (104, 6, 7, 26), 9),
        ];
        for (first, gregorian_date, (cycle, year, month, day), rows) in tables {
            let start = jdn(first);
            assert_eq!(gregorian::from_fixed(start), Ok(gregorian_date));
            assert_eq!(written(start), (cycle, year, month, day), "JDN {first}");
            for offset in 0..i64::from(rows) {
                let rd = Rd(start.0 + offset);
                let (_, _, m, d) = written(rd);
                let expected_day = i64::from(day) + offset;
                let length = i64::from(days_in_month(absolute_year(cycle, year), month).unwrap());
                if expected_day <= length {
                    assert_eq!(
                        (m, i64::from(d)),
                        (month, expected_day),
                        "JDN {}",
                        first + offset
                    );
                } else {
                    assert_eq!((m, i64::from(d)), (month + 1, expected_day - length));
                }
            }
        }
        // The rows at the month changes, spelled out: Ibrahim 29 to
        // Julius 1, Julius 30 to Khayyam 1, Galileo 29 to Hypatia 1.
        assert_eq!(written(jdn(8)), (-10, 33, 9, 29));
        assert_eq!(written(jdn(9)), (-10, 33, 10, 1));
        assert_eq!(written(jdn(2_415_020)), (100, 45, 10, 30));
        assert_eq!(written(jdn(2_415_021)), (100, 45, 11, 1));
        assert_eq!(written(jdn(2_488_344)), (104, 6, 7, 29));
        assert_eq!(written(jdn(2_488_345)), (104, 6, 8, 1));
    }

    /// The dated properties Meyer lists.
    #[test]
    fn the_dates_meyer_lists_among_the_properties() {
        let cases = [
            // A vernal equinox and a new moon together: the first day of
            // cycle 099.
            ((1_795, 3, 20), (99, 1, 1, 1)),
            // The page's own date of publication.
            ((1_999, 3, 17), (102, 25, 1, 1)),
            // The total solar eclipse.
            ((1_999, 8, 11), (102, 25, 6, 1)),
            // Eras 1 and 2 begin, each on a Sunday.
            ((2_695, 4, 7), (114, 1, 1, 1)),
            ((9_535, 4, 7), (228, 1, 1, 1)),
            // Palmen's page is dated 1999-04-29 CE / 102-25-02-15 MP.
            ((1_999, 4, 29), (102, 25, 2, 15)),
        ];
        for ((y, m, d), expected) in cases {
            let rd = gregorian::to_fixed(y, m, d).unwrap();
            assert_eq!(written(rd), expected, "{y}-{m}-{d}");
        }
        for cycle in [0, 114, 228] {
            let start = new_year(absolute_year(cycle, 1)).unwrap();
            assert_eq!(Weekday::from_rd(start), Weekday::Sunday, "cycle {cycle}");
        }
    }

    /// Meyer's listing of the year 102-25, from his converter: 385 days,
    /// Meton from 5 March 2000 to its 31st on 4 April, and 102-26 on
    /// 5 April 2000.
    #[test]
    fn the_year_102_25_is_the_one_meyers_converter_lists() {
        let year = absolute_year(102, 25);
        assert_eq!(year, 6_145);
        assert_eq!(days_in_year(year), 385);
        let first = gregorian::to_fixed(1_999, 3, 17).unwrap();
        assert_eq!(first.to_julian_day_number(), 2_451_255);
        assert_eq!(Weekday::from_rd(first), Weekday::Wednesday);
        assert_eq!(
            written(gregorian::to_fixed(1_999, 4, 14).unwrap()),
            (102, 25, 1, 29)
        );
        assert_eq!(
            written(gregorian::to_fixed(1_999, 4, 15).unwrap()),
            (102, 25, 2, 1)
        );
        assert_eq!(
            written(gregorian::to_fixed(2_000, 3, 4).unwrap()),
            (102, 25, 12, 30)
        );
        assert_eq!(
            written(gregorian::to_fixed(2_000, 3, 5).unwrap()),
            (102, 25, 13, 1)
        );
        assert_eq!(
            written(gregorian::to_fixed(2_000, 4, 4).unwrap()).3,
            31,
            "Meton 31"
        );
        assert_eq!(
            written(gregorian::to_fixed(2_000, 4, 5).unwrap()),
            (102, 26, 1, 1)
        );
    }

    /// Palmen's table of the two remainders, the year lengths and the New
    /// Year's Days for 102-25 to 102-44.
    #[test]
    fn palmens_table_of_remainders_lengths_and_new_years() {
        // (year of cycle 102, first remainder, second remainder, length,
        // New Year's Day)
        type Row = (u8, i64, Option<i64>, u16, (i64, u8, u8));
        let table: [Row; 20] = [
            (25, 335, Some(97), 385, (1_999, 3, 17)),
            (26, 2_854, None, 354, (2_000, 4, 5)),
            (27, 5_373, None, 354, (2_001, 3, 25)),
            (28, 1_052, Some(1_425), 384, (2_002, 3, 14)),
            (29, 3_571, None, 354, (2_003, 4, 2)),
            (30, 6_090, None, 354, (2_004, 3, 21)),
            (31, 1_769, Some(234), 385, (2_005, 3, 10)),
            (32, 4_288, None, 354, (2_006, 3, 30)),
            (33, 6_807, None, 354, (2_007, 3, 19)),
            (34, 2_486, Some(1_562), 384, (2_008, 3, 7)),
            (35, 5_005, None, 354, (2_009, 3, 26)),
            (36, 684, Some(371), 385, (2_010, 3, 15)),
            (37, 3_203, None, 354, (2_011, 4, 4)),
            (38, 5_722, None, 354, (2_012, 3, 23)),
            (39, 1_401, Some(1_699), 384, (2_013, 3, 12)),
            (40, 3_920, None, 354, (2_014, 3, 31)),
            (41, 6_439, None, 354, (2_015, 3, 20)),
            (42, 2_118, Some(508), 385, (2_016, 3, 8)),
            (43, 4_637, None, 354, (2_017, 3, 28)),
            (44, 316, Some(1_836), 384, (2_018, 3, 17)),
        ];
        for (of_cycle, first, second, length, (y, m, d)) in table {
            let year = absolute_year(102, of_cycle);
            assert_eq!((year * ERA_LONG_YEARS) % ERA_YEARS, first, "102-{of_cycle}");
            assert_eq!(is_long_year(year), second.is_some(), "102-{of_cycle}");
            if let Some(second) = second {
                let k = year * ERA_LONG_YEARS / ERA_YEARS;
                assert_eq!((k * ERA_LONG_METONS) % ERA_LONG_YEARS, second);
            }
            assert_eq!(days_in_year(year), length, "102-{of_cycle}");
            assert_eq!(
                new_year(year),
                gregorian::to_fixed(y, m, d),
                "102-{of_cycle}"
            );
        }
        // "the quotient = 2263" for 102-25: it is the 2263rd long year.
        assert_eq!(absolute_year(102, 25) * ERA_LONG_YEARS / ERA_YEARS, 2_263);
        // "the first long year of cycle 000 is year 3".
        assert!(!is_long_year(1) && !is_long_year(2) && is_long_year(3));
    }

    /// The counts of an era, as Meyer states them.
    #[test]
    fn an_era_has_the_years_months_days_and_weeks_meyer_states() {
        let years = MIN_YEAR..MIN_YEAR + ERA_YEARS;
        let long = years.clone().filter(|year| is_long_year(*year)).count();
        let metons = years.clone().filter(|year| has_long_meton(*year)).count();
        assert_eq!((long, metons), (2_519, 1_328));
        let days: i64 = years
            .clone()
            .map(|year| i64::from(days_in_year(year)))
            .sum();
        assert_eq!(days, 2_498_258);
        assert_eq!(days, ERA_DAYS);
        assert_eq!(ERA_DAYS / 7, 356_894);
        assert_eq!(ERA_DAYS % 7, 0);
        assert_eq!(ERA_MONTHS, 84_599);
        assert_eq!(ERA_CYCLES, 114);
        let months: i64 = years.map(|year| 12 + i64::from(is_long_year(year))).sum();
        assert_eq!(months, ERA_MONTHS);
        // Every era is the same: the structure repeats after 6840 years.
        for year in (MIN_YEAR..=MAX_YEAR - ERA_YEARS).step_by(37) {
            assert_eq!(days_in_year(year), days_in_year(year + ERA_YEARS), "{year}");
            assert_eq!(
                new_year(year + ERA_YEARS).unwrap().0 - new_year(year).unwrap().0,
                ERA_DAYS
            );
        }
        // The mean year and month Meyer quotes.
        let mean_year = ERA_DAYS as f64 / ERA_YEARS as f64;
        let mean_month = ERA_DAYS as f64 / ERA_MONTHS as f64;
        assert!((mean_year - 365.242_397_66).abs() < 1e-8, "{mean_year}");
        assert!((mean_month - 29.530_585_468).abs() < 1e-9, "{mean_month}");
    }

    /// "The expression [(60.c + y) . L / Y] evaluates to n" for the nth
    /// long year: the second rule needs only the count of long years.
    #[test]
    fn the_quotient_of_the_first_rule_counts_the_long_years() {
        let mut count = 0;
        for year in 1..=ERA_YEARS {
            if is_long_year(year) {
                count += 1;
                assert_eq!(year * ERA_LONG_YEARS / ERA_YEARS, count, "{year}");
            }
            assert_eq!(long_years_before(year + 1), count, "{year}");
        }
    }

    /// Meyer's frequency table of New Year's Days over 0–4000 CE: 4001 of
    /// them, from 6 March to 7 April, with these counts.
    #[test]
    fn new_years_days_are_distributed_as_meyers_table_counts_them() {
        let expected: [((u8, u8), u32); 33] = [
            ((3, 6), 7),
            ((3, 7), 74),
            ((3, 8), 132),
            ((3, 9), 128),
            ((3, 10), 132),
            ((3, 11), 129),
            ((3, 12), 136),
            ((3, 13), 128),
            ((3, 14), 131),
            ((3, 15), 133),
            ((3, 16), 127),
            ((3, 17), 133),
            ((3, 18), 130),
            ((3, 19), 134),
            ((3, 20), 132),
            ((3, 21), 127),
            ((3, 22), 131),
            ((3, 23), 136),
            ((3, 24), 129),
            ((3, 25), 132),
            ((3, 26), 128),
            ((3, 27), 131),
            ((3, 28), 131),
            ((3, 29), 134),
            ((3, 30), 128),
            ((3, 31), 137),
            ((4, 1), 125),
            ((4, 2), 132),
            ((4, 3), 134),
            ((4, 4), 128),
            ((4, 5), 134),
            ((4, 6), 95),
            ((4, 7), 23),
        ];
        let mut counts = [0u32; 33];
        let first = gregorian::to_fixed(0, 1, 1).unwrap();
        let last = gregorian::to_fixed(4_000, 12, 31).unwrap();
        let (mut year, _, _) = from_fixed(first).unwrap();
        let mut seen = 0;
        loop {
            let start = new_year(year).unwrap();
            if start > last {
                break;
            }
            if start >= first {
                let (_, month, day) = gregorian::from_fixed(start).unwrap();
                let index = expected
                    .iter()
                    .position(|(date, _)| *date == (month, day))
                    .unwrap_or_else(|| panic!("New Year's Day on {month}-{day}"));
                counts[index] += 1;
                seen += 1;
            }
            year += 1;
        }
        assert_eq!(seen, 4_001);
        for (index, (date, count)) in expected.iter().enumerate() {
            assert_eq!(counts[index], *count, "{date:?}");
        }
    }

    #[test]
    fn odd_months_have_29_days_even_30_and_meton_30_or_31() {
        for year in (MIN_YEAR..=MAX_YEAR).step_by(97) {
            for month in 1..=12u8 {
                let expected = if month % 2 == 1 { 29 } else { 30 };
                assert_eq!(days_in_month(year, month), Some(expected));
            }
            let meton = days_in_month(year, METON);
            assert_eq!(meton.is_some(), is_long_year(year));
            if let Some(length) = meton {
                assert_eq!(length == 31, has_long_meton(year));
            }
            assert_eq!(days_in_month(year, 14), None);
            assert_eq!(days_in_month(year, 0), None);
        }
    }

    #[test]
    fn every_day_of_a_thousand_years_round_trips() {
        let start = new_year(absolute_year(100, 1)).unwrap().0;
        let end = new_year(absolute_year(100, 1) + 1_000).unwrap().0;
        let mut previous = from_fixed(Rd(start - 1)).unwrap();
        for rd in start..end {
            let date = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(date.0, date.1, date.2), Ok(Rd(rd)), "rd {rd}");
            // Each day follows the one before.
            if date.2 == 1 {
                assert!(date.1 == 1 || date.1 == previous.1 + 1);
            } else {
                assert_eq!((date.0, date.1, date.2 - 1), previous);
            }
            previous = date;
        }
    }

    #[test]
    fn the_whole_range_round_trips() {
        for rd in (EARLIEST.0..=LATEST.0).step_by(331) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)), "rd {rd}");
        }
        assert_eq!(from_fixed(EARLIEST), Ok((MIN_YEAR, 1, 1)));
        let (year, month, _) = from_fixed(LATEST).unwrap();
        assert_eq!(year, MAX_YEAR);
        assert!(month >= 12);
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }

    #[test]
    fn meton_is_refused_in_a_short_year_and_bad_fields_name_themselves() {
        let short = absolute_year(102, 26);
        assert!(!is_long_year(short));
        assert_eq!(
            to_fixed(short, METON, 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(to_fixed(short, 1, 30), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(short, 2, 0), Err(CalendarError::DayOutOfRange));
        let thirty = absolute_year(102, 28);
        assert_eq!(days_in_year(thirty), 384);
        assert_eq!(
            to_fixed(thirty, METON, 31),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            to_fixed(MAX_YEAR + 1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            MeyerPalmenDate::from_cycle(102, 61, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn the_calendar_speaks_cycle_and_year_of_cycle_through_its_fields() {
        let calendar = MeyerPalmenCalendar;
        let date = calendar
            .from_fixed(gregorian::to_fixed(1_999, 8, 11).unwrap())
            .unwrap();
        assert_eq!(date, MeyerPalmenDate::from_cycle(102, 25, 6, 1).unwrap());
        let fields = calendar.to_fields(date).unwrap();
        assert_eq!(fields.year, 6_145);
        assert_eq!(fields.era, Some(ERA));
        assert_eq!(fields.extra.get("cycle"), Some(102));
        assert_eq!(fields.extra.get("year-of-cycle"), Some(25));
        assert_eq!(calendar.from_fields(&fields), Ok(date));
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(6_145, 1, 1).with_era("ah")),
            Err(CalendarError::UnknownEra)
        );
        assert_eq!(
            calendar.from_fields(&DateFields::ymd_leap_month(6_145, 12, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(calendar.is_leap_year(6_145), Ok(true));
        assert_eq!(calendar.is_leap_year(6_146), Ok(false));
        assert_eq!(
            calendar.is_leap_year(MAX_YEAR + 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(calendar.era_name(ERA).map(|name| name.latin()), Some("MP"));
        assert_eq!(calendar.cycles()[0].name(12), Some("Meton"));
        for rd in (EARLIEST.0..=LATEST.0).step_by(4_999) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(calendar.from_fields(&fields), Ok(date));
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
    }
}
