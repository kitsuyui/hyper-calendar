//! The Archetypes Calendar of Peter Meyer.
//!
//! An arithmetic lunisolar calendar meant to track the dark moon and to
//! keep its New Year's Day where the Chinese calendar keeps its own,
//! between 21 January and 21 February. Months alternate 30 and 29 days,
//! Apollo to Demeter, and a *long* year adds a thirteenth, Persephone, of
//! 30; in a *leap* year the tenth month, Sophia, has 30 days instead of 29.
//! A month is three *tweeks* of ten days, the last of nine in a 29-day
//! month, the days named Sun Day to Pluto Day. Years run in ARC periods of
//! 1 803: the year y has position `p = ((y + 1360) mod 1803) + 1`, and is
//! long when `(664 · p + 901) mod 1803 < 664` and leap when
//! `(350 · p + 901) mod 1803 < 350`. A period therefore holds 664 long and
//! 350 leap years, 22 300 months and 658 532 days, a mean year of
//! 365.24237 days and a mean month of 29.530583, and a whole number of
//! seven-day weeks. The years 443 to 2245 are an ARC period, and 443-1-1
//! is Julian Day Number 897 474, 5 February 2256 BC.
//!
//! Meyer's "long year" is the one with the intercalary month, so it is
//! what [`Calendar::is_leap_year`] answers; his "leap year", the one with
//! a 30-day Sophia, is [`has_long_sophia`]. The tweek and its day are the
//! `tweek` and `day-of-tweek` extra fields and the `tweek-day` cycle. The
//! system document is `docs/systems/archetypes.md`.
//!
//! # Sources
//!
//! * Peter Meyer, "The Archetypes Calendar: An accurate lunisolar calendar
//!   with connections to the Chinese Calendar", Hermetic Systems,
//!   <https://www.hermetic.ch/cal_stud/arch_cal/arch_cal.htm>, retrieved
//!   2026-09-26 (`meyer-archetypes`): the definition, the names, the
//!   correlation, the three runs of dated days of 2010 to 2012, the table
//!   of New Year's Days for 4699 to 4755, the properties of an ARC period,
//!   and the dates compared with the Chinese calendar.
//!
//! # Exactness
//!
//! Exact: the rules are the definition. Meyer's comparisons with the dark
//! moon and with the Chinese New Year are his and are not re-measured.

use hc_calendar::shape::{CycleLength, CycleShape, MONTH};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

/// The calendar identifier.
pub const ID: &str = "archetypes";

/// The month names, as Meyer gives them; Persephone only in a long year.
pub const MONTHS: [&str; 13] = [
    "Apollo",
    "Diana",
    "Hermes",
    "Aphrodite",
    "Ares",
    "Zeus",
    "Chronos",
    "Prometheus",
    "Orpheus",
    "Sophia",
    "Dionysus",
    "Demeter",
    "Persephone",
];

/// The days of the tweek, as Meyer names them.
pub const TWEEK_DAYS: [&str; 10] = [
    "Sun Day",
    "Mercury Day",
    "Venus Day",
    "Earth Day",
    "Mars Day",
    "Jupiter Day",
    "Saturn Day",
    "Uranus Day",
    "Neptune Day",
    "Pluto Day",
];

/// The cycle kind of the day of the tweek.
pub const TWEEK_DAY: &str = "tweek-day";

/// The month number of Persephone, the thirteenth month of a long year.
pub const PERSEPHONE: u8 = 13;

/// The month number of Sophia, 30 days in a leap year.
pub const SOPHIA: u8 = 10;

/// Years in an ARC period, Meyer's Y.
pub const PERIOD_YEARS: i64 = 1_803;

/// Long years in a period, Meyer's L1.
pub const PERIOD_LONG_YEARS: i64 = 664;

/// Leap years in a period, Meyer's L2.
pub const PERIOD_LEAP_YEARS: i64 = 350;

/// Days in a period.
pub const PERIOD_DAYS: i64 = 354 * PERIOD_YEARS + 30 * PERIOD_LONG_YEARS + PERIOD_LEAP_YEARS;

/// The first year of the ARC period Meyer anchors, whose position is 1.
pub const ANCHOR_YEAR: i64 = 443;

/// The fixed day of 443-1-1 ARC, Julian Day Number 897 474.
pub const EPOCH: Rd = Rd::from_julian_day_number(897_474);

/// The earliest year this implementation converts: the first of the ARC
/// period three before the anchored one.
pub const MIN_YEAR: i64 = ANCHOR_YEAR - 3 * PERIOD_YEARS;

/// The latest year this implementation converts: the last of the fourth
/// period after the anchored one, which holds the years Meyer compared, to
/// 5400.
pub const MAX_YEAR: i64 = ANCHOR_YEAR + 5 * PERIOD_YEARS - 1;

/// The position of `year` in its ARC period, 1 to 1803.
#[must_use]
pub const fn position(year: i64) -> i64 {
    (year + 1_360).rem_euclid(PERIOD_YEARS) + 1
}

/// Half the period, `(Y − 1) / 2`, the offset of both rules.
const OFFSET: i64 = (PERIOD_YEARS - 1) / 2;

/// Whether `year` is long, with Persephone: `(664 · p + 901) mod 1803 < 664`.
#[must_use]
pub const fn is_long_year(year: i64) -> bool {
    (PERIOD_LONG_YEARS * position(year) + OFFSET).rem_euclid(PERIOD_YEARS) < PERIOD_LONG_YEARS
}

/// Whether `year` is leap in Meyer's sense, Sophia of 30 days:
/// `(350 · p + 901) mod 1803 < 350`.
#[must_use]
pub const fn has_long_sophia(year: i64) -> bool {
    (PERIOD_LEAP_YEARS * position(year) + OFFSET).rem_euclid(PERIOD_YEARS) < PERIOD_LEAP_YEARS
}

/// Years of a rule from the anchor year up to but not including `year`,
/// negative before it: the rule's form makes them one division.
const fn counted_before(rate: i64, year: i64) -> i64 {
    // `year - ANCHOR_YEAR` is position − 1 modulo the period, and the
    // count steps up exactly at the years the rule selects.
    (rate * (year - ANCHOR_YEAR) + OFFSET).div_euclid(PERIOD_YEARS)
}

/// Days from 443-1-1 to the first day of `year`.
const fn days_before_year(year: i64) -> i64 {
    354 * (year - ANCHOR_YEAR)
        + 30 * counted_before(PERIOD_LONG_YEARS, year)
        + counted_before(PERIOD_LEAP_YEARS, year)
}

/// The number of days in `month` of `year`, or `None` when the month does
/// not exist — Persephone exists only in a long year.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    match month {
        SOPHIA => Some(if has_long_sophia(year) { 30 } else { 29 }),
        1..=12 => Some(if month % 2 == 1 { 30 } else { 29 }),
        PERSEPHONE if is_long_year(year) => Some(30),
        _ => None,
    }
}

/// The number of days in `year`: 354, 355, 384 or 385.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    354 + if is_long_year(year) { 30 } else { 0 } + if has_long_sophia(year) { 1 } else { 0 }
}

/// Days before the first of `month` in `year`: pairs of 30 and 29, and
/// Sophia's thirtieth before Dionysus.
const fn days_before_month(year: i64, month: u8) -> i64 {
    let elapsed = month as i64 - 1;
    let before = 59 * (elapsed / 2) + 30 * (elapsed % 2);
    if month > SOPHIA && has_long_sophia(year) {
        before + 1
    } else {
        before
    }
}

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = Rd(EPOCH.0 + days_before_year(MIN_YEAR));

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(EPOCH.0 + days_before_year(MAX_YEAR + 1) - 1);

/// The fixed day of New Year's Day, 1 Apollo, of `year`.
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

/// The fixed day of an Archetypes date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] — also for Persephone in a year
/// without one — or [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    let start = match new_year(year) {
        Ok(start) => start,
        Err(error) => return Err(error),
    };
    match days_in_month(year, month) {
        None => Err(CalendarError::MonthOutOfRange),
        Some(length) if day == 0 || day > length => Err(CalendarError::DayOutOfRange),
        Some(_) => Ok(Rd(start.0 + days_before_month(year, month) + day as i64 - 1)),
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
    let elapsed = rd.0 - EPOCH.0;
    // The period's own mean year lands within a year of the answer.
    let mut year = ANCHOR_YEAR + (elapsed * PERIOD_YEARS).div_euclid(PERIOD_DAYS);
    while days_before_year(year) > elapsed {
        year -= 1;
    }
    while days_before_year(year + 1) <= elapsed {
        year += 1;
    }
    let within = elapsed - days_before_year(year);
    let mut month = 1u8;
    while month < PERSEPHONE {
        let next = days_before_month(year, month + 1);
        if within < next {
            break;
        }
        month += 1;
    }
    Ok((
        year,
        month,
        (within - days_before_month(year, month) + 1) as u8,
    ))
}

/// An Archetypes date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArchetypesDate {
    /// The year, any integer.
    pub year: i64,
    /// The month, 1 for Apollo through 12 for Demeter, 13 for Persephone.
    pub month: u8,
    /// The day of the month, 1 through 29 or 30.
    pub day: u8,
}

impl ArchetypesDate {
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

    /// The tweek of the month, 1 to 3.
    #[must_use]
    pub const fn tweek(self) -> u8 {
        (self.day - 1) / 10 + 1
    }

    /// The day of the tweek, 1 for Sun Day to 10 for Pluto Day: the last
    /// digit of the day of the month, as Meyer notes.
    #[must_use]
    pub const fn day_of_tweek(self) -> u8 {
        (self.day - 1) % 10 + 1
    }

    /// The name of the day of the tweek.
    #[must_use]
    pub const fn day_name(self) -> &'static str {
        TWEEK_DAYS[self.day_of_tweek() as usize - 1]
    }
}

/// The Archetypes Calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ArchetypesCalendar;

/// Twelve months and Persephone in a long year, and the ten-day tweek.
const SHAPE: &[CycleShape] = &[
    CycleShape {
        kind: MONTH,
        length: CycleLength::Intercalary {
            ordinary: 12,
            extended: 13,
        },
        names: &MONTHS,
    },
    CycleShape::named(TWEEK_DAY, &TWEEK_DAYS),
];

impl Calendar for ArchetypesCalendar {
    type Date = ArchetypesDate;

    /// Unrecorded: a proposal, adopted by nobody.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// Twelve or thirteen named months and the tweek.
    fn cycles(&self) -> &'static [CycleShape] {
        SHAPE
    }

    /// A long year, the one with Persephone; Meyer's leap year, with a
    /// 30-day Sophia, is [`has_long_sophia`].
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(is_long_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "Archetypes",
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
        Ok(ArchetypesDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        DateFields::ymd(date.year, date.month, date.day)
            .with_extra("tweek", i64::from(date.tweek()))?
            .with_extra("day-of-tweek", i64::from(date.day_of_tweek()))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        ArchetypesDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::{Weekday, gregorian};

    fn gregorian(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    fn jdn(number: i64) -> Rd {
        Rd::from_julian_day_number(number)
    }

    /// A Gregorian year, month and day.
    type Ymd = (i64, u8, u8);

    #[test]
    fn the_correlation_is_meyers() {
        // "443-1-1 ARC denotes the day with Julian day number 897,474
        // (which is -2255-02-05 CE) ... 1-1-1 ARC denotes the day with
        // Julian day number 736,030 (which is -2697-01-30 CE)."
        assert_eq!(new_year(443), Ok(jdn(897_474)));
        assert_eq!(jdn(897_474), gregorian(-2_255, 2, 5));
        assert_eq!(new_year(1), Ok(jdn(736_030)));
        assert_eq!(jdn(736_030), gregorian(-2_697, 1, 30));
        assert_eq!(position(443), 1);
        assert_eq!(position(2_245), 1_803);
        assert_eq!(position(2_246), 1);
    }

    #[test]
    fn the_worked_example_of_4300() {
        // Position 252; 168229 mod 1803 = 550 < 664, long; 89101 mod 1803
        // = 754, not leap.
        assert_eq!(position(4_300), 252);
        assert!(is_long_year(4_300));
        assert!(!has_long_sophia(4_300));
        assert_eq!(days_in_year(4_300), 384);
    }

    /// Meyer's three runs of dates, with the day of the tweek and the JDN.
    #[test]
    fn the_three_runs_of_dated_days() {
        let runs = [
            ((2010, 3, 7), 2_455_263, (4_708, 1, 22), "Mercury Day"),
            ((2011, 1, 28), 2_455_590, (4_708, 12, 24), "Earth Day"),
            ((2012, 12, 16), 2_456_278, (4_710, 12, 3), "Venus Day"),
        ];
        for ((year, month, day), number, (arc_year, arc_month, arc_day), name) in runs {
            let start = gregorian(year, month, day);
            assert_eq!(start, jdn(number));
            let date = ArchetypesCalendar.from_fixed(start).unwrap();
            assert_eq!(
                (date.year, date.month, date.day),
                (arc_year, arc_month, arc_day)
            );
            assert_eq!(date.day_name(), name);
        }
        // Each run is twelve consecutive days; the turns of month and year
        // inside them are Meyer's.
        assert_eq!(from_fixed(jdn(2_455_271)), Ok((4_708, 1, 30)));
        assert_eq!(from_fixed(jdn(2_455_272)), Ok((4_708, 2, 1)));
        assert_eq!(from_fixed(jdn(2_455_595)), Ok((4_708, 12, 29)));
        assert_eq!(from_fixed(jdn(2_455_596)), Ok((4_709, 1, 1)));
        assert_eq!(from_fixed(jdn(2_455_601)), Ok((4_709, 1, 6)));
        assert_eq!(from_fixed(jdn(2_456_285)), Ok((4_710, 12, 10)));
        assert_eq!(
            ArchetypesDate::new(4_710, 12, 10).unwrap().day_name(),
            "Pluto Day"
        );
        assert_eq!(from_fixed(jdn(2_456_289)), Ok((4_710, 12, 14)));
    }

    /// Meyer's table of New Year's Days for 4699 to 4755, with which years
    /// are long and which leap.
    #[test]
    fn the_new_years_of_4699_to_4755() {
        const TABLE: [(i64, Ymd, bool, bool); 57] = [
            (4699, (2001, 1, 24), true, false),
            (4700, (2002, 2, 12), false, true),
            (4701, (2003, 2, 2), false, false),
            (4702, (2004, 1, 22), true, false),
            (4703, (2005, 2, 9), false, false),
            (4704, (2006, 1, 29), true, false),
            (4705, (2007, 2, 17), false, true),
            (4706, (2008, 2, 7), false, false),
            (4707, (2009, 1, 26), true, false),
            (4708, (2010, 2, 14), false, false),
            (4709, (2011, 2, 3), false, false),
            (4710, (2012, 1, 23), true, true),
            (4711, (2013, 2, 11), false, false),
            (4712, (2014, 1, 31), true, false),
            (4713, (2015, 2, 19), false, false),
            (4714, (2016, 2, 8), false, false),
            (4715, (2017, 1, 27), true, false),
            (4716, (2018, 2, 15), false, true),
            (4717, (2019, 2, 5), false, false),
            (4718, (2020, 1, 25), true, false),
            (4719, (2021, 2, 12), false, false),
            (4720, (2022, 2, 1), false, false),
            (4721, (2023, 1, 21), true, true),
            (4722, (2024, 2, 10), false, false),
            (4723, (2025, 1, 29), true, false),
            (4724, (2026, 2, 17), false, false),
            (4725, (2027, 2, 6), false, false),
            (4726, (2028, 1, 26), true, true),
            (4727, (2029, 2, 14), false, false),
            (4728, (2030, 2, 3), false, false),
            (4729, (2031, 1, 23), true, false),
            (4730, (2032, 2, 11), false, false),
            (4731, (2033, 1, 30), true, true),
            (4732, (2034, 2, 19), false, false),
            (4733, (2035, 2, 8), false, false),
            (4734, (2036, 1, 28), true, false),
            (4735, (2037, 2, 15), false, false),
            (4736, (2038, 2, 4), false, true),
            (4737, (2039, 1, 25), true, false),
            (4738, (2040, 2, 13), false, false),
            (4739, (2041, 2, 1), false, false),
            (4740, (2042, 1, 21), true, false),
            (4741, (2043, 2, 9), false, true),
            (4742, (2044, 1, 30), true, false),
            (4743, (2045, 2, 17), false, false),
            (4744, (2046, 2, 6), false, false),
            (4745, (2047, 1, 26), true, false),
            (4746, (2048, 2, 14), false, false),
            (4747, (2049, 2, 2), false, true),
            (4748, (2050, 1, 23), true, false),
            (4749, (2051, 2, 11), false, false),
            (4750, (2052, 1, 31), true, false),
            (4751, (2053, 2, 18), false, false),
            (4752, (2054, 2, 7), false, true),
            (4753, (2055, 1, 28), true, false),
            (4754, (2056, 2, 16), false, false),
            (4755, (2057, 2, 4), false, false),
        ];
        for (year, (gregorian_year, month, day), long, leap) in TABLE {
            assert_eq!(
                new_year(year),
                Ok(gregorian(gregorian_year, month, day)),
                "{year}"
            );
            assert_eq!(
                (is_long_year(year), has_long_sophia(year)),
                (long, leap),
                "{year}"
            );
        }
    }

    /// The dates Meyer gives in comparing the calendar with the Chinese.
    #[test]
    fn the_dates_of_the_chinese_comparison() {
        // "4400-01-01 ARC (= 1702-01-28 CE) through 5400-12-29 ARC (=
        // 2703-02-07 CE)", and "4709-09-24 ARC (= 2011-10-20 CE)".
        assert_eq!(to_fixed(4_400, 1, 1), Ok(gregorian(1_702, 1, 28)));
        assert_eq!(to_fixed(5_400, 12, 29), Ok(gregorian(2_703, 2, 7)));
        assert_eq!(to_fixed(4_709, 9, 24), Ok(gregorian(2_011, 10, 20)));
        // "In ARC year 3195 (497 CE) new year's day occurred on January 20."
        assert_eq!(new_year(3_195), Ok(gregorian(497, 1, 20)));
        // "All new year's days in ARC years 4300 through 5200 ... occur from
        // January 21 through February 21."
        let earliest = gregorian::to_fixed(2_000, 1, 21).unwrap().0 - gregorian(2_000, 1, 1).0;
        for year in 4_300..=5_200 {
            let start = new_year(year).unwrap();
            let (gregorian_year, month, day) = gregorian::from_fixed(start).unwrap();
            let offset = start.0 - gregorian(gregorian_year, 1, 1).0;
            assert!(
                offset >= earliest && (month, day) <= (2, 21),
                "{year}: {month}-{day}"
            );
        }
    }

    /// "During one ARC period there are exactly 664 occurrences of long
    /// years ... and 350 ... of leap years. The 1,803 years consist of
    /// 22,300 months and 658,532 days", 94 076 weeks, and the year types
    /// are symmetrical about the middle year.
    #[test]
    fn the_properties_of_an_arc_period() {
        let period = ANCHOR_YEAR..ANCHOR_YEAR + PERIOD_YEARS;
        assert_eq!(
            period.clone().filter(|year| is_long_year(*year)).count(),
            664
        );
        assert_eq!(
            period.clone().filter(|year| has_long_sophia(*year)).count(),
            350
        );
        let months: i64 = period
            .clone()
            .map(|year| if is_long_year(year) { 13 } else { 12 })
            .sum();
        assert_eq!(months, 22_300);
        assert_eq!(days_before_year(ANCHOR_YEAR + PERIOD_YEARS), 658_532);
        assert_eq!(PERIOD_DAYS, 658_532);
        assert_eq!(PERIOD_DAYS % 7, 0);
        assert_eq!(PERIOD_DAYS / 7, 94_076);
        for offset in 0..PERIOD_YEARS {
            let year = ANCHOR_YEAR + offset;
            let mirror = ANCHOR_YEAR + PERIOD_YEARS - 1 - offset;
            assert_eq!(
                (is_long_year(year), has_long_sophia(year)),
                (is_long_year(mirror), has_long_sophia(mirror)),
                "{year}"
            );
        }
        // Every period begins on the same weekday.
        assert_eq!(
            Weekday::from_rd(new_year(ANCHOR_YEAR + PERIOD_YEARS).unwrap()),
            Weekday::from_rd(EPOCH)
        );
    }

    #[test]
    fn the_month_lengths() {
        assert_eq!(days_in_month(4_300, PERSEPHONE), Some(30));
        assert_eq!(days_in_month(4_701, PERSEPHONE), None);
        assert_eq!(days_in_month(4_700, SOPHIA), Some(30));
        assert_eq!(days_in_month(4_701, SOPHIA), Some(29));
        assert_eq!(days_in_month(4_701, 14), None);
        assert_eq!(
            to_fixed(4_701, PERSEPHONE, 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(to_fixed(4_701, 2, 30), Err(CalendarError::DayOutOfRange));
        for year in MIN_YEAR..MAX_YEAR {
            let length = days_before_year(year + 1) - days_before_year(year);
            assert_eq!(length, i64::from(days_in_year(year)), "{year}");
        }
    }

    #[test]
    fn every_day_round_trips() {
        for rd in (EARLIEST.0..=LATEST.0).step_by(211) {
            let date = ArchetypesCalendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(ArchetypesCalendar.to_fixed(date), Ok(Rd(rd)));
            let fields = ArchetypesCalendar.to_fields(date).unwrap();
            assert_eq!(ArchetypesCalendar.from_fields(&fields), Ok(date));
        }
        let start = new_year(4_699).unwrap().0;
        for rd in start..start + 21_000 {
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
            ArchetypesCalendar.is_leap_year(MAX_YEAR + 1),
            Err(CalendarError::YearOutOfRange)
        );
    }
}
