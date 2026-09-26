//! Karl Palmen's Yerm lunar calendar.
//!
//! A purely lunar calendar that "abandons any pretence to follow the
//! seasons". Its unit above the month is the *yerm* (YEaR Moon), an odd
//! number of months alternating 30 and 29 nights, beginning and ending
//! with 30, so that a new yerm begins wherever two 30-night months meet.
//! The yerms of a cycle have 17 months, except those whose number is
//! divisible by 3, which have 15: 17, 17, 15 seventeen times, 1447 days,
//! and a fifty-second yerm of 17 months, 502 days, which Palmen describes
//! as "an additional 17 month yerm" inserted after every seventeen of the
//! three-yerm groups. A cycle is 52 yerms, 850 months and 25 101 nights, a
//! mean month of 29.530 588 2 days.
//!
//! The day, or "night", begins at noon, "so that the night is not
//! interrupted by a date change". This library maps a yerm date to the
//! civil day on whose noon it begins, which is how Palmen's tables give it
//! ("begins noon 2016-09-02") and how the Julian Day Number maps its own
//! noon-to-noon days: the morning of the next civil day belongs to the
//! same yerm date. The day boundary says noon.
//!
//! Palmen numbers the cycles from cycle 1 on Julian Day 1 948 379, noon of
//! 16 May 622 Julian, so that the cycle of the present day, which began at
//! noon on 11 November 1996, is cycle 21. The date's year is the yerm
//! counted continuously from that epoch, `52 · (cycle − 1) + yerm`, and
//! `cycle` and `yerm-of-cycle` are fields beside it: 21-05(03(30 is yerm
//! 1045, month 3, night 30. The calendar claims nothing about the seasons
//! and nothing here tests it against them. The system document is
//! `docs/systems/yerm.md`.
//!
//! # Sources
//!
//! * Karl Palmen, "Yerm Lunar Calendar", Hermetic Systems,
//!   <https://www.hermetic.ch/cal_stud/palmen/yerm1.htm>, retrieved
//!   2026-09-26: the three rules, the noon day boundary, the months of
//!   yerms 21-16 to 21-18 with the Gregorian noons they begin on, the new
//!   yerms of cycles 20 and 21 with their weekdays, the new cycles 17 to
//!   22, the conversion algorithm and its constants, the cycle numbering
//!   from Julian Day 1 948 379, and the example "2002-06-10 pm =
//!   21-05(03(30".
//! * Karl Palmen, "Some Properties of the Meyer-Palmen Solilunar
//!   Calendar", <https://www.hermetic.ch/cal_stud/nlsc/mpslci.htm>,
//!   retrieved 2026-09-26: the month of the cycle, counted from the first
//!   month of the last yerm, is long when `m · 451 mod 850 < 451`.
//!
//! # Exactness
//!
//! Exact: the rules are the definition.

use hc_calendar::shape::{CycleShape, MONTH, WEEKDAY};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, DayBoundary, Rd,
    YearKind,
};

/// The calendar identifier.
pub const ID: &str = "yerm";

/// Yerms in a cycle.
pub const CYCLE_YERMS: i64 = 52;

/// Months in a cycle.
pub const CYCLE_MONTHS: i64 = 850;

/// Nights in a cycle.
pub const CYCLE_DAYS: i64 = 25_101;

/// Nights in three consecutive yerms of 17, 17 and 15 months: "exactly 2
/// weeks less than 4 Julian years".
pub const TRIAD_DAYS: i64 = 1_447;

/// Nights in a yerm of 17 months.
pub const LONG_YERM_DAYS: i64 = 502;

/// Nights in a yerm of 15 months.
pub const SHORT_YERM_DAYS: i64 = 443;

/// Nights in two consecutive months, 30 and 29.
const MONTH_PAIR_DAYS: i64 = 59;

/// The fixed day on whose noon night 1 of month 1 of yerm 1 of cycle 1
/// begins: Julian Day 1 948 379, 16 May 622 in the Julian calendar.
pub const EPOCH: Rd = Rd::from_julian_day_number(1_948_379);

/// The latest cycle this implementation converts, which ends in the
/// eleventh millennium.
pub const MAX_CYCLE: i64 = 150;

/// The earliest continuous yerm this implementation converts: yerm 1 of
/// cycle 1.
pub const MIN_YEAR: i64 = 1;

/// The latest continuous yerm this implementation converts: yerm 52 of
/// [`MAX_CYCLE`].
pub const MAX_YEAR: i64 = CYCLE_YERMS * MAX_CYCLE;

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = EPOCH;

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(EPOCH.0 + MAX_CYCLE * CYCLE_DAYS - 1);

/// The continuous yerm of `yerm` in `cycle`.
#[must_use]
pub const fn absolute_yerm(cycle: i64, yerm: u8) -> i64 {
    CYCLE_YERMS * (cycle - 1) + yerm as i64
}

/// The cycle and the yerm within it, 1 to 52, of a continuous yerm.
#[must_use]
pub const fn cycle_and_yerm(year: i64) -> (i64, u8) {
    let elapsed = year - 1;
    (
        elapsed.div_euclid(CYCLE_YERMS) + 1,
        (elapsed.rem_euclid(CYCLE_YERMS) + 1) as u8,
    )
}

/// The number of months in a continuous yerm: 15 when its number within
/// the cycle is divisible by 3, otherwise 17.
///
/// Fifty-two is not divisible by 3, so the last yerm of the cycle, the one
/// inserted after seventeen groups of three, has 17 without a rule of its
/// own.
#[must_use]
pub const fn months_in_yerm(year: i64) -> u8 {
    if cycle_and_yerm(year).1.is_multiple_of(3) {
        15
    } else {
        17
    }
}

/// Whether a continuous yerm is the fifty-second of its cycle, the "additional
/// 17 month yerm" inserted after every seventeen groups of 17, 17 and 15
/// months, which is the calendar's intercalation.
#[must_use]
pub const fn is_inserted_yerm(year: i64) -> bool {
    cycle_and_yerm(year).1 as i64 == CYCLE_YERMS
}

/// The number of nights in `month` of a continuous yerm, or `None` when the
/// yerm has no such month: odd months 30, even months 29.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    if month == 0 || month > months_in_yerm(year) {
        None
    } else if month % 2 == 1 {
        Some(30)
    } else {
        Some(29)
    }
}

/// The number of nights in a continuous yerm: 502 or 443.
#[must_use]
pub const fn days_in_yerm(year: i64) -> u16 {
    if months_in_yerm(year) == 17 {
        LONG_YERM_DAYS as u16
    } else {
        SHORT_YERM_DAYS as u16
    }
}

/// Nights from the epoch to the first night of a continuous yerm,
/// unchecked: Palmen's conversion to a Julian Day, less the epoch.
const fn days_before_yerm(year: i64) -> i64 {
    let (cycle, yerm) = cycle_and_yerm(year);
    let within = yerm as i64 - 1;
    (cycle - 1) * CYCLE_DAYS + (within / 3) * TRIAD_DAYS + (within % 3) * LONG_YERM_DAYS
}

/// Nights elapsed in the yerm before the first of `month`.
const fn days_before_month(month: u8) -> i64 {
    let within = month as i64 - 1;
    (within / 2) * MONTH_PAIR_DAYS + (within % 2) * 30
}

/// The fixed day of the first night of a continuous yerm.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn new_yerm(year: i64) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    Ok(Rd(EPOCH.0 + days_before_yerm(year)))
}

/// The fixed day of a yerm date, the civil day on whose noon its night
/// begins.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] — also for month 16 or 17 of a
/// 15-month yerm — or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    let start = match new_yerm(year) {
        Ok(start) => start,
        Err(error) => return Err(error),
    };
    match days_in_month(year, month) {
        None => Err(CalendarError::MonthOutOfRange),
        Some(length) if day == 0 || day > length => Err(CalendarError::DayOutOfRange),
        Some(_) => Ok(Rd(start.0 + days_before_month(month) + day as i64 - 1)),
    }
}

/// The continuous yerm, month and night of a fixed day, by Palmen's
/// conversion from a Julian Day.
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
    // Each division takes the quotient and leaves the remainder, as
    // Palmen's `divide(&day, b)` does. The fifty-second yerm is the
    // eighteenth "group of three", of which only its first 502 nights
    // exist, so the division by 502 that follows never reaches 1 there.
    let day = rd.0 - EPOCH.0;
    let cycle = 1 + day / CYCLE_DAYS;
    let day = day % CYCLE_DAYS;
    let yerm = 1 + 3 * (day / TRIAD_DAYS);
    let day = day % TRIAD_DAYS;
    let yerm = yerm + day / LONG_YERM_DAYS;
    let day = day % LONG_YERM_DAYS;
    let month = 1 + 2 * (day / MONTH_PAIR_DAYS);
    let day = day % MONTH_PAIR_DAYS;
    let month = month + day / 30;
    let day = day % 30;
    Ok((
        absolute_yerm(cycle, yerm as u8),
        month as u8,
        (day + 1) as u8,
    ))
}

/// A yerm date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct YermDate {
    /// The yerm counted continuously from the epoch,
    /// `52 · (cycle − 1) + yerm`.
    pub year: i64,
    /// The month of the yerm, 1 to 15 or 17.
    pub month: u8,
    /// The night of the month, 1 to 29 or 30.
    pub day: u8,
}

impl YermDate {
    /// A validated date from its continuous yerm.
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

    /// A validated date as Palmen writes it, cycle-yerm(month(night, so
    /// 21-05(03(30 is `from_cycle(21, 5, 3, 30)`.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] for a yerm of the cycle
    /// outside 1 to 52, and otherwise as [`YermDate::new`].
    pub const fn from_cycle(cycle: i64, yerm: u8, month: u8, day: u8) -> CalendarResult<Self> {
        if yerm == 0 || yerm as i64 > CYCLE_YERMS {
            return Err(CalendarError::YearOutOfRange);
        }
        Self::new(absolute_yerm(cycle, yerm), month, day)
    }

    /// The 52-yerm cycle, counted from 1.
    #[must_use]
    pub const fn cycle(self) -> i64 {
        cycle_and_yerm(self.year).0
    }

    /// The yerm within the cycle, 1 to 52.
    #[must_use]
    pub const fn yerm_of_cycle(self) -> u8 {
        cycle_and_yerm(self.year).1
    }
}

/// The Yerm lunar calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct YermCalendar;

/// Fifteen or seventeen numbered months, and the week the nights run in.
const SHAPE: &[CycleShape] = &[
    CycleShape::intercalary(MONTH, 15, 17),
    CycleShape::fixed(WEEKDAY, 7),
];

impl Calendar for YermCalendar {
    type Date = YermDate;

    /// Unrecorded: a proposal, adopted by nobody.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// Fifteen or seventeen months, numbered, and the week.
    fn cycles(&self) -> &'static [CycleShape] {
        SHAPE
    }

    /// The fifty-second yerm of a cycle, the one Palmen inserts after
    /// seventeen groups of three to correct the mean month. A 17-month
    /// yerm is not leap: two in every three have 17 months.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(is_inserted_yerm(year))
    }

    /// Noon, so that a night carries one date.
    fn day_boundary(&self) -> DayBoundary {
        DayBoundary::Noon
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "Yerm Lunar",
            year_kind: YearKind::EpochForward,
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
        Ok(YermDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.month, date.day)
            .with_extra("cycle", date.cycle())?
            .with_extra("yerm-of-cycle", i64::from(date.yerm_of_cycle()))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some() {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        YermDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::{Weekday, gregorian};

    /// The date as Palmen writes it, from a fixed day.
    fn written(rd: Rd) -> (i64, u8, u8, u8) {
        let (year, month, day) = from_fixed(rd).unwrap();
        let (cycle, yerm) = cycle_and_yerm(year);
        (cycle, yerm, month, day)
    }

    fn day(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn cycle_one_begins_on_julian_day_1_948_379() {
        assert_eq!(EPOCH.to_julian_day_number(), 1_948_379);
        // 16 May 622 Julian is 19 May proleptic Gregorian.
        assert_eq!(gregorian::from_fixed(EPOCH), Ok((622, 5, 19)));
        assert_eq!(to_fixed(1, 1, 1), Ok(EPOCH));
        // "just two months before the start of the Islamic AH era": 61
        // days before the civil epoch, JDN 1 948 440.
        assert_eq!(
            crate::tabular::CIVIL_EPOCH.to_julian_day_number() - EPOCH.to_julian_day_number(),
            61
        );
    }

    /// "The present cycle began at noon on 11 November 1996", and
    /// "2002-06-10 pm = 21-05(03(30".
    #[test]
    fn the_present_cycle_and_palmens_worked_date() {
        assert_eq!(written(day(1_996, 11, 11)), (21, 1, 1, 1));
        assert_eq!(written(day(1_996, 11, 10)), (20, 52, 17, 30));
        assert_eq!(written(day(2_002, 6, 10)), (21, 5, 3, 30));
        assert_eq!(day(1_996, 11, 11).to_julian_day_number(), 2_450_399);
        assert_eq!(
            YermDate::from_cycle(21, 5, 3, 30).unwrap(),
            YermDate::new(1_045, 3, 30).unwrap()
        );
    }

    /// Palmen's "New Yerm Cycles" and the numbered cycles 17 to 22.
    #[test]
    fn the_cycles_begin_where_palmen_lists_them() {
        let starts = [
            (17, (1_721, 12, 19), Weekday::Friday),
            (18, (1_790, 9, 9), Weekday::Thursday),
            (19, (1_859, 6, 1), Weekday::Wednesday),
            (20, (1_928, 2, 21), Weekday::Tuesday),
            (21, (1_996, 11, 11), Weekday::Monday),
            (22, (2_065, 8, 2), Weekday::Sunday),
        ];
        for (cycle, (y, m, d), weekday) in starts {
            let rd = new_yerm(absolute_yerm(cycle, 1)).unwrap();
            assert_eq!(gregorian::from_fixed(rd), Ok((y, m, d)), "cycle {cycle}");
            assert_eq!(Weekday::from_rd(rd), weekday, "cycle {cycle}");
        }
    }

    /// Palmen's table of the dates of new yerms, 25 to 52 of the last
    /// cycle and 1 to 24 of this one, with their weekdays.
    #[test]
    fn the_new_yerms_are_the_ones_palmen_tabulates() {
        use Weekday::{Friday, Monday, Saturday, Sunday, Thursday, Tuesday, Wednesday};
        // (cycle, yerm, Gregorian date, weekday)
        type Row = (i64, u8, (i64, u8, u8), Weekday);
        let table: [Row; 52] = [
            (20, 25, (1_959, 11, 1), Sunday),
            (20, 26, (1_961, 3, 17), Friday),
            (20, 27, (1_962, 8, 1), Wednesday),
            (20, 28, (1_963, 10, 18), Friday),
            (20, 29, (1_965, 3, 3), Wednesday),
            (20, 30, (1_966, 7, 18), Monday),
            (20, 31, (1_967, 10, 4), Wednesday),
            (20, 32, (1_969, 2, 17), Monday),
            (20, 33, (1_970, 7, 4), Saturday),
            (20, 34, (1_971, 9, 20), Monday),
            (20, 35, (1_973, 2, 3), Saturday),
            (20, 36, (1_974, 6, 20), Thursday),
            (20, 37, (1_975, 9, 6), Saturday),
            (20, 38, (1_977, 1, 20), Thursday),
            (20, 39, (1_978, 6, 6), Tuesday),
            (20, 40, (1_979, 8, 23), Thursday),
            (20, 41, (1_981, 1, 6), Tuesday),
            (20, 42, (1_982, 5, 23), Sunday),
            (20, 43, (1_983, 8, 9), Tuesday),
            (20, 44, (1_984, 12, 23), Sunday),
            (20, 45, (1_986, 5, 9), Friday),
            (20, 46, (1_987, 7, 26), Sunday),
            (20, 47, (1_988, 12, 9), Friday),
            (20, 48, (1_990, 4, 25), Wednesday),
            (20, 49, (1_991, 7, 12), Friday),
            (20, 50, (1_992, 11, 25), Wednesday),
            (20, 51, (1_994, 4, 11), Monday),
            (20, 52, (1_995, 6, 28), Wednesday),
            (21, 1, (1_996, 11, 11), Monday),
            (21, 2, (1_998, 3, 28), Saturday),
            (21, 3, (1_999, 8, 12), Thursday),
            (21, 4, (2_000, 10, 28), Saturday),
            (21, 5, (2_002, 3, 14), Thursday),
            (21, 6, (2_003, 7, 29), Tuesday),
            (21, 7, (2_004, 10, 14), Thursday),
            (21, 8, (2_006, 2, 28), Tuesday),
            (21, 9, (2_007, 7, 15), Sunday),
            (21, 10, (2_008, 9, 30), Tuesday),
            (21, 11, (2_010, 2, 14), Sunday),
            (21, 12, (2_011, 7, 1), Friday),
            (21, 13, (2_012, 9, 16), Sunday),
            (21, 14, (2_014, 1, 31), Friday),
            (21, 15, (2_015, 6, 17), Wednesday),
            (21, 16, (2_016, 9, 2), Friday),
            (21, 17, (2_018, 1, 17), Wednesday),
            (21, 18, (2_019, 6, 3), Monday),
            (21, 19, (2_020, 8, 19), Wednesday),
            (21, 20, (2_022, 1, 3), Monday),
            (21, 21, (2_023, 5, 20), Saturday),
            (21, 22, (2_024, 8, 5), Monday),
            (21, 23, (2_025, 12, 20), Saturday),
            (21, 24, (2_027, 5, 6), Thursday),
        ];
        for (cycle, yerm, (y, m, d), weekday) in table {
            let rd = day(y, m, d);
            assert_eq!(written(rd), (cycle, yerm, 1, 1), "{cycle}-{yerm}");
            assert_eq!(Weekday::from_rd(rd), weekday, "{cycle}-{yerm}");
            assert_eq!(new_yerm(absolute_yerm(cycle, yerm)), Ok(rd));
        }
    }

    /// Palmen's "Correlation with the Moon" table: the Gregorian noon on
    /// which each month of yerms 21-16 to 21-18 begins.
    #[test]
    fn the_months_of_yerms_16_to_18_begin_where_palmen_lists_them() {
        let table: [(u8, u8, (i64, u8, u8)); 49] = [
            (16, 1, (2_016, 9, 2)),
            (16, 2, (2_016, 10, 2)),
            (16, 3, (2_016, 10, 31)),
            (16, 4, (2_016, 11, 30)),
            (16, 5, (2_016, 12, 29)),
            (16, 6, (2_017, 1, 28)),
            (16, 7, (2_017, 2, 26)),
            (16, 8, (2_017, 3, 28)),
            (16, 9, (2_017, 4, 26)),
            (16, 10, (2_017, 5, 26)),
            (16, 11, (2_017, 6, 24)),
            (16, 12, (2_017, 7, 24)),
            (16, 13, (2_017, 8, 22)),
            (16, 14, (2_017, 9, 21)),
            (16, 15, (2_017, 10, 20)),
            (16, 16, (2_017, 11, 19)),
            (16, 17, (2_017, 12, 18)),
            (17, 1, (2_018, 1, 17)),
            (17, 2, (2_018, 2, 16)),
            (17, 3, (2_018, 3, 17)),
            (17, 4, (2_018, 4, 16)),
            (17, 5, (2_018, 5, 15)),
            (17, 6, (2_018, 6, 14)),
            (17, 7, (2_018, 7, 13)),
            (17, 8, (2_018, 8, 12)),
            (17, 9, (2_018, 9, 10)),
            (17, 10, (2_018, 10, 10)),
            (17, 11, (2_018, 11, 8)),
            (17, 12, (2_018, 12, 8)),
            (17, 13, (2_019, 1, 6)),
            (17, 14, (2_019, 2, 5)),
            (17, 15, (2_019, 3, 6)),
            (17, 16, (2_019, 4, 5)),
            (17, 17, (2_019, 5, 4)),
            (18, 1, (2_019, 6, 3)),
            (18, 2, (2_019, 7, 3)),
            (18, 3, (2_019, 8, 1)),
            (18, 4, (2_019, 8, 31)),
            (18, 5, (2_019, 9, 29)),
            (18, 6, (2_019, 10, 29)),
            (18, 7, (2_019, 11, 27)),
            (18, 8, (2_019, 12, 27)),
            (18, 9, (2_020, 1, 25)),
            (18, 10, (2_020, 2, 24)),
            (18, 11, (2_020, 3, 24)),
            (18, 12, (2_020, 4, 23)),
            (18, 13, (2_020, 5, 22)),
            (18, 14, (2_020, 6, 21)),
            (18, 15, (2_020, 7, 20)),
        ];
        for (yerm, month, (y, m, d)) in table {
            assert_eq!(
                to_fixed(absolute_yerm(21, yerm), month, 1),
                Ok(day(y, m, d)),
                "21-{yerm}({month}"
            );
        }
        // Yerm 18 is divisible by three: fifteen months, then yerm 19.
        assert_eq!(months_in_yerm(absolute_yerm(21, 18)), 15);
        assert_eq!(written(day(2_020, 8, 19)), (21, 19, 1, 1));
    }

    #[test]
    fn a_cycle_is_52_yerms_850_months_and_25_101_nights() {
        let yerms = 1..=CYCLE_YERMS;
        let months: i64 = yerms.clone().map(|y| i64::from(months_in_yerm(y))).sum();
        let days: i64 = yerms.clone().map(|y| i64::from(days_in_yerm(y))).sum();
        assert_eq!((months, days), (CYCLE_MONTHS, CYCLE_DAYS));
        assert_eq!(yerms.filter(|y| months_in_yerm(*y) == 15).count(), 17);
        // The cycle is the constant Palmen's conversion divides by.
        for cycle in [1, 2, 21, MAX_CYCLE] {
            assert_eq!(
                new_yerm(absolute_yerm(cycle, 1)),
                Ok(Rd(EPOCH.0 + (cycle - 1) * CYCLE_DAYS))
            );
        }
        let mean = CYCLE_DAYS as f64 / CYCLE_MONTHS as f64;
        assert!((mean - 29.530_588_2).abs() < 1e-7, "{mean}");
    }

    /// The arithmetic Palmen remarks on: three yerms are two weeks short of
    /// four Julian years, a 17- and a 15-month yerm together are exactly
    /// 135 weeks, and a yerm begins and ends with a 30-night month.
    #[test]
    fn the_yerm_arithmetic_palmen_remarks_on() {
        assert_eq!(TRIAD_DAYS, 4 * 365 + 1 - 14);
        assert_eq!(LONG_YERM_DAYS + SHORT_YERM_DAYS, 135 * 7);
        assert_eq!(2 * LONG_YERM_DAYS + SHORT_YERM_DAYS, TRIAD_DAYS);
        for year in 1..=CYCLE_YERMS * 3 {
            let last = months_in_yerm(year);
            assert_eq!(days_in_month(year, 1), Some(30));
            assert_eq!(days_in_month(year, last), Some(30), "{year}");
            assert_eq!(days_in_month(year, last + 1), None);
            let total: u16 = (1..=last)
                .map(|m| u16::from(days_in_month(year, m).unwrap()))
                .sum();
            assert_eq!(total, days_in_yerm(year));
        }
    }

    /// Palmen's second statement of the rule: counting months from the
    /// first of the last yerm of a cycle as 0, month m has 30 nights when
    /// `m · 451 mod 850 < 451`. It is independent of the first, so it
    /// checks it.
    #[test]
    fn the_long_months_follow_palmens_451_in_850() {
        let mut rd = new_yerm(absolute_yerm(20, 52)).unwrap();
        for m in 0..3 * CYCLE_MONTHS {
            let (year, month, night) = from_fixed(rd).unwrap();
            assert_eq!(night, 1);
            let length = days_in_month(year, month).unwrap();
            assert_eq!(length == 30, (m * 451) % 850 < 451, "month {m}");
            rd = Rd(rd.0 + i64::from(length));
        }
    }

    #[test]
    fn every_night_of_two_cycles_round_trips() {
        let start = new_yerm(absolute_yerm(20, 1)).unwrap().0;
        let mut previous = from_fixed(Rd(start - 1)).unwrap();
        for rd in start..start + 2 * CYCLE_DAYS {
            let date = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(date.0, date.1, date.2), Ok(Rd(rd)), "rd {rd}");
            if date.2 != 1 {
                assert_eq!((date.0, date.1, date.2 - 1), previous);
            } else if date.1 != 1 {
                assert_eq!((date.0, date.1 - 1), (previous.0, previous.1));
            } else {
                assert_eq!(date.0, previous.0 + 1);
            }
            previous = date;
        }
    }

    #[test]
    fn the_whole_range_round_trips_and_its_edges_are_refused() {
        for rd in (EARLIEST.0..=LATEST.0).step_by(113) {
            let (year, month, night) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, night), Ok(Rd(rd)), "rd {rd}");
        }
        assert_eq!(from_fixed(EARLIEST), Ok((1, 1, 1)));
        assert_eq!(from_fixed(LATEST), Ok((MAX_YEAR, 17, 30)));
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        let short = absolute_yerm(21, 3);
        assert_eq!(to_fixed(short, 16, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(to_fixed(short, 2, 30), Err(CalendarError::DayOutOfRange));
        assert_eq!(
            YermDate::from_cycle(21, 53, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn the_night_begins_at_noon_and_the_calendar_speaks_its_cycle() {
        let calendar = YermCalendar;
        assert_eq!(calendar.day_boundary(), DayBoundary::Noon);
        let date = calendar.from_fixed(day(2_002, 6, 10)).unwrap();
        let fields = calendar.to_fields(date).unwrap();
        assert_eq!(fields.year, 1_045);
        assert_eq!(fields.extra.get("cycle"), Some(21));
        assert_eq!(fields.extra.get("yerm-of-cycle"), Some(5));
        assert_eq!(calendar.from_fields(&fields), Ok(date));
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(1_045, 1, 1).with_era("ah")),
            Err(CalendarError::UnknownEra)
        );
        assert_eq!(calendar.is_leap_year(absolute_yerm(21, 52)), Ok(true));
        assert_eq!(calendar.is_leap_year(absolute_yerm(21, 51)), Ok(false));
        assert_eq!(calendar.is_leap_year(absolute_yerm(21, 50)), Ok(false));
        assert_eq!(calendar.is_leap_year(0), Err(CalendarError::YearOutOfRange));
        for rd in (EARLIEST.0..=LATEST.0).step_by(1_999) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(calendar.from_fields(&fields), Ok(date));
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
        }
    }
}
