//! The Pax calendar of James A. Colligan (1930).
//!
//! A leap-week calendar: thirteen months of 28 days — the Gregorian twelve
//! from January to November, then *Columbus*, then December — and, in a
//! leap year, a seven-day month *Pax* between Columbus and December, so
//! that every year has whole weeks, 52 or 53, and "the first day of every
//! week, month and year would be Sunday". The leap week is added "to 71 of
//! the 400 years in the cycle. The years with leap week are years whose
//! last two digits are a number that is divisible by six (including 00) or
//! 99: however, if a year number ending in 00 is divisible by 400, then
//! Pax is cancelled", which gives the Gregorian mean year. The year
//! carries the Gregorian number and begins on the Sunday within nine days
//! of the Gregorian 1 January, 18 December to 6 January; Pax 1928 began on
//! Sunday 1 January 1928, the first year of the published table.
//!
//! The months are numbered 1 to 14 with Pax as month 13 and December as
//! month 14 in every year, so that each month keeps one number and one
//! name; month 13 exists only in a leap year, and asking for it in a
//! common year is [`CalendarError::MonthOutOfRange`]. The year's weekday
//! is the unbroken week's, since every year begins on a real Sunday.
//!
//! # Sources
//!
//! * Wikipedia, "Pax Calendar", <https://en.wikipedia.org/wiki/Pax_Calendar>,
//!   retrieved 2026-10-04 (`wikipedia-pax-calendar`): the months, the leap
//!   rule as quoted above, the Sunday, and its two tables of the Gregorian
//!   date of Pax New Year's Day, 1928–2054 and the turns of the centuries
//!   2091–2112 and 2291–2312, which the tests check. A secondary source: Colligan's proposal of 1930 and his *An
//!   unchangeable calendar without blank days* (University of San
//!   Francisco, 1933), which it cites, were not read, and would replace it.
//!
//! # Exactness
//!
//! Exact: the quoted rule is the definition.

use hc_calendar::shape::{CycleShape, MONTH, WEEKDAY};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Weekday,
    YearKind,
};

use crate::{common, gregorian};

/// The calendar identifier.
pub const ID: &str = "pax";

/// The fourteen month positions: the Gregorian months to November,
/// Columbus, Pax — a leap year's only — and December.
pub const MONTHS: [&str; 14] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "Columbus",
    "Pax",
    "December",
];

/// The month number of Pax, the leap week.
pub const PAX: u8 = 13;

/// The month number of December, in every year.
pub const DECEMBER: u8 = 14;

/// The length of every month but Pax.
pub const DAYS_IN_MONTH: u8 = 28;

/// Leap weeks in a 400-year cycle.
pub const LEAP_WEEKS_IN_CYCLE: i64 = 71;

/// The earliest year this implementation converts: the rule speaks of a
/// year's last two digits, so it is read for the positive years only.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 99_999;

/// Whether `year` has the month of Pax: its last two digits are divisible
/// by six or are 99, unless it is divisible by 400.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    let last_two = year.rem_euclid(100);
    (last_two % 6 == 0 && year.rem_euclid(400) != 0) || last_two == 99
}

/// Leap years `y` with `0 <= y < n`, negative for `n < 0`: the rule
/// repeats every 400 years with 71 leap years a cycle.
const fn leap_years_before(n: i64) -> i64 {
    let cycles = n.div_euclid(400);
    let rest = n.rem_euclid(400);
    let mut count = cycles * LEAP_WEEKS_IN_CYCLE;
    let mut year = 0;
    while year < rest {
        if is_leap_year(year) {
            count += 1;
        }
        year += 1;
    }
    count
}

/// The number of days in `year`, 364 or 371.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) { 371 } else { 364 }
}

/// The number of days in `month` of `year`: 28, or 7 for Pax in a leap
/// year; `None` for Pax in a common year and outside `1..=14`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    match month {
        PAX => {
            if is_leap_year(year) {
                Some(7)
            } else {
                None
            }
        }
        1..=12 | DECEMBER => Some(DAYS_IN_MONTH),
        _ => None,
    }
}

/// Days in the year before the first of `month`.
const fn days_before_month(year: i64, month: u8) -> i64 {
    if month == DECEMBER {
        12 * DAYS_IN_MONTH as i64 + if is_leap_year(year) { 7 } else { 0 }
    } else {
        DAYS_IN_MONTH as i64 * (month as i64 - 1)
    }
}

/// The fixed day on which Pax 1928 began: Sunday 1 January 1928, the
/// first row of the published table.
const ANCHOR: Rd = match gregorian::to_fixed(1928, 1, 1) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// The fixed day on which year 1 begins, counted back from [`ANCHOR`].
const YEAR_ONE: i64 = ANCHOR.0 - 364 * 1927 - 7 * leap_years_before(1928);

/// The fixed day on which `year` begins, without validation: year 1 and
/// 364 days a year, plus a week for each leap year before it.
const fn new_year_raw(year: i64) -> i64 {
    YEAR_ONE + 364 * (year - 1) + 7 * leap_years_before(year)
}

/// The fixed day of 1 January of `year`, always a Sunday.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn new_year(year: i64) -> CalendarResult<Rd> {
    if year < MIN_YEAR || year > MAX_YEAR {
        return Err(CalendarError::YearOutOfRange);
    }
    Ok(Rd(new_year_raw(year)))
}

/// The earliest fixed day this implementation converts: 1 January 1.
pub const EARLIEST: Rd = Rd(new_year_raw(MIN_YEAR));

/// The latest fixed day this implementation converts: 28 December 99 999.
pub const LATEST: Rd = Rd(new_year_raw(MAX_YEAR + 1) - 1);

/// The fixed day of a Pax date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`],
/// [`CalendarError::MonthOutOfRange`] — also for Pax in a common year — or
/// [`CalendarError::DayOutOfRange`].
pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    let start = match new_year(year) {
        Ok(start) => start,
        Err(error) => return Err(error),
    };
    match common::check_day(day, days_in_month(year, month)) {
        Err(error) => Err(error),
        Ok(()) => Ok(Rd(start.0 + days_before_month(year, month) + day as i64 - 1)),
    }
}

/// The Pax year of a fixed day, without range checks: the Gregorian year,
/// or its neighbour when the day falls between the two New Year's Days.
const fn year_from_fixed_raw(rd: Rd) -> CalendarResult<i64> {
    let gregorian_year = match gregorian::year_from_fixed(rd) {
        Ok(year) => year,
        Err(error) => return Err(error),
    };
    if rd.0 >= new_year_raw(gregorian_year + 1) {
        Ok(gregorian_year + 1)
    } else if rd.0 < new_year_raw(gregorian_year) {
        Ok(gregorian_year - 1)
    } else {
        Ok(gregorian_year)
    }
}

/// The Pax year, month and day of a fixed day.
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
    let year = match year_from_fixed_raw(rd) {
        Ok(year) => year,
        Err(error) => return Err(error),
    };
    let day_of_year = rd.0 - new_year_raw(year);
    let before_pax = 12 * DAYS_IN_MONTH as i64;
    let (month, day) = if day_of_year < before_pax {
        (day_of_year / 28 + 1, day_of_year % 28)
    } else if is_leap_year(year) && day_of_year < before_pax + 7 {
        (PAX as i64, day_of_year - before_pax)
    } else {
        (
            DECEMBER as i64,
            day_of_year - days_before_month(year, DECEMBER),
        )
    };
    Ok((year, month as u8, (day + 1) as u8))
}

/// A Pax date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PaxDate {
    /// The year, which carries the Gregorian number.
    pub year: i64,
    /// The month, 1 for January through 12 for Columbus, 13 for Pax in a
    /// leap year and 14 for December.
    pub month: u8,
    /// The day of the month, 1 through 28, or through 7 in Pax.
    pub day: u8,
}

impl PaxDate {
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

    /// The name of the month.
    #[must_use]
    pub const fn month_name(self) -> &'static str {
        MONTHS[self.month as usize - 1]
    }

    /// Whether this date falls in the leap week.
    #[must_use]
    pub const fn is_in_pax(self) -> bool {
        self.month == PAX
    }
}

/// The Pax calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct PaxCalendar;

/// Fourteen month positions, the thirteenth a leap year's only, and the
/// seven-day week, whose first day is Sunday in every month.
const SHAPE: &[CycleShape] = &[
    CycleShape::named(MONTH, &MONTHS),
    CycleShape::fixed(WEEKDAY, 7),
];

impl Calendar for PaxCalendar {
    type Date = PaxDate;

    /// Unrecorded: a proposal, adopted by nobody.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// The fourteen month positions and the week.
    fn cycles(&self) -> &'static [CycleShape] {
        SHAPE
    }

    /// A year with the month of Pax.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(ID),
            english_name: "Pax",
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
        Ok(PaxDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let weekday = Weekday::from_rd(to_fixed(date.year, date.month, date.day)?);
        DateFields::ymd(date.year, date.month, date.day)
            .with_extra("day-of-week", i64::from(weekday.iso_number()))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        PaxDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gregorian(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    /// Wikipedia's two tables of Pax New Year's Day, 1928–2054 and the
    /// turns of the centuries, 2091–2112 and 2291–2312: (Pax year,
    /// Gregorian year, month, day).
    const NEW_YEARS: [(i64, i64, u8, u8); 66] = [
        (1928, 1928, 1, 1),
        (1929, 1928, 12, 30),
        (1930, 1929, 12, 29),
        (1931, 1931, 1, 4),
        (1932, 1932, 1, 3),
        (1935, 1934, 12, 30),
        (1936, 1935, 12, 29),
        (1998, 1997, 12, 28),
        (1999, 1998, 12, 27),
        (2000, 2000, 1, 2),
        (2001, 2000, 12, 31),
        (2002, 2001, 12, 30),
        (2003, 2002, 12, 29),
        (2004, 2003, 12, 28),
        (2005, 2004, 12, 26),
        (2006, 2005, 12, 25),
        (2007, 2006, 12, 31),
        (2026, 2025, 12, 28),
        (2027, 2026, 12, 27),
        (2028, 2027, 12, 26),
        (2029, 2028, 12, 24),
        (2030, 2029, 12, 23),
        (2091, 2090, 12, 24),
        (2092, 2091, 12, 23),
        (2093, 2092, 12, 21),
        (2094, 2093, 12, 20),
        (2095, 2094, 12, 19),
        (2096, 2095, 12, 18),
        (2097, 2096, 12, 23),
        (2098, 2097, 12, 22),
        (2099, 2098, 12, 21),
        (2100, 2099, 12, 27),
        (2101, 2101, 1, 2),
        (2102, 2102, 1, 1),
        (2103, 2102, 12, 31),
        (2104, 2103, 12, 30),
        (2105, 2104, 12, 28),
        (2106, 2105, 12, 27),
        (2107, 2107, 1, 2),
        (2108, 2108, 1, 1),
        (2109, 2108, 12, 30),
        (2110, 2109, 12, 29),
        (2111, 2110, 12, 28),
        (2112, 2111, 12, 27),
        (2291, 2290, 12, 28),
        (2292, 2291, 12, 27),
        (2293, 2292, 12, 25),
        (2294, 2293, 12, 24),
        (2295, 2294, 12, 23),
        (2296, 2295, 12, 22),
        (2297, 2296, 12, 27),
        (2298, 2297, 12, 26),
        (2299, 2298, 12, 25),
        (2300, 2299, 12, 31),
        (2301, 2301, 1, 6),
        (2302, 2302, 1, 5),
        (2303, 2303, 1, 4),
        (2304, 2304, 1, 3),
        (2305, 2305, 1, 1),
        (2306, 2305, 12, 31),
        (2307, 2307, 1, 6),
        (2308, 2308, 1, 5),
        (2309, 2309, 1, 3),
        (2310, 2310, 1, 2),
        (2311, 2311, 1, 1),
        (2312, 2311, 12, 31),
    ];

    #[test]
    fn the_published_new_years_days() {
        for (year, gregorian_year, month, day) in NEW_YEARS {
            let expected = gregorian(gregorian_year, month, day);
            assert_eq!(new_year(year), Ok(expected), "{year}");
            assert_eq!(from_fixed(expected), Ok((year, 1, 1)), "{year}");
            assert_eq!(Weekday::from_rd(expected), Weekday::Sunday, "{year}");
        }
        // Pax 2006 has the month of Pax, so Pax 2007 begins a week later
        // than 364 days would put it: Pax 1 is 26 November 2006.
        assert!(is_leap_year(2006));
        assert_eq!(to_fixed(2006, PAX, 1), Ok(gregorian(2006, 11, 26)));
        assert_eq!(to_fixed(2006, PAX, 7), Ok(gregorian(2006, 12, 2)));
        assert_eq!(to_fixed(2006, DECEMBER, 1), Ok(gregorian(2006, 12, 3)));
        assert_eq!(to_fixed(2006, DECEMBER, 28), Ok(gregorian(2006, 12, 30)));
    }

    #[test]
    fn the_leap_rule_is_colligans() {
        // Divisible by six, including 00; or 99; unless divisible by 400.
        for year in [6, 12, 30, 96, 99, 100, 1900, 2006, 2012, 2100, 2199, 2200] {
            assert!(is_leap_year(year), "{year}");
        }
        for year in [1, 5, 7, 98, 400, 1600, 2000, 2001, 2004, 2026, 2400] {
            assert!(!is_leap_year(year), "{year}");
        }
        // 71 in 400 years: 17 multiples of six and 99 in each century less
        // the one year divisible by 400, which is the Gregorian count.
        for start in [1, 1601, 2000] {
            assert_eq!(
                (start..start + 400)
                    .filter(|year| is_leap_year(*year))
                    .count(),
                71
            );
        }
        assert_eq!(leap_years_before(400), 71);
        assert_eq!(leap_years_before(-400), -71);
        assert_eq!(leap_years_before(1), 0, "year 0 is divisible by 400");
    }

    #[test]
    fn every_year_begins_on_a_sunday_near_the_gregorian_new_year() {
        for year in MIN_YEAR..=MAX_YEAR {
            let start = new_year(year).unwrap();
            assert_eq!(Weekday::from_rd(start), Weekday::Sunday, "{year}");
            // "the full 19-day range (Dec 18 to Jan 06)".
            let earliest = gregorian(year - 1, 12, 18);
            let latest = gregorian(year, 1, 6);
            assert!((earliest..=latest).contains(&start), "{year}");
            let length = new_year_raw(year + 1) - start.0;
            assert_eq!(length, i64::from(days_in_year(year)), "{year}");
        }
    }

    #[test]
    fn pax_exists_only_in_a_leap_year() {
        assert_eq!(days_in_month(2006, PAX), Some(7));
        assert_eq!(days_in_month(2026, PAX), None);
        assert_eq!(to_fixed(2026, PAX, 1), Err(CalendarError::MonthOutOfRange));
        assert_eq!(to_fixed(2006, PAX, 8), Err(CalendarError::DayOutOfRange));
        assert_eq!(to_fixed(2026, 12, 29), Err(CalendarError::DayOutOfRange));
        assert_eq!(days_in_month(2026, 15), None);
        assert_eq!(days_in_month(2026, 0), None);
        assert_eq!(
            PaxDate::new(2006, PAX, 3).map(PaxDate::month_name),
            Ok("Pax")
        );
        assert!(PaxDate::new(2006, PAX, 3).unwrap().is_in_pax());
        // December is month 14 in every year, and follows Columbus directly
        // in a common one.
        assert_eq!(
            to_fixed(2026, DECEMBER, 1).unwrap().0,
            to_fixed(2026, 12, 28).unwrap().0 + 1
        );
        assert_eq!(
            PaxDate::new(2026, DECEMBER, 1).unwrap().month_name(),
            "December"
        );
        assert_eq!(to_fixed(0, 1, 1), Err(CalendarError::YearOutOfRange));
        assert_eq!(
            PaxCalendar.is_leap_year(MAX_YEAR + 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            PaxCalendar.from_fields(&DateFields::ymd_leap_month(2006, 12, 1)),
            Err(CalendarError::MonthOutOfRange)
        );
    }

    #[test]
    fn every_day_round_trips() {
        for rd in ANCHOR.0..ANCHOR.0 + 40_000 {
            let date = PaxCalendar.from_fixed(Rd(rd)).unwrap();
            assert_eq!(PaxCalendar.to_fixed(date), Ok(Rd(rd)), "{rd}");
            let fields = PaxCalendar.to_fields(date).unwrap();
            assert_eq!(PaxCalendar.from_fields(&fields), Ok(date));
        }
        // Every day in a release build; every 9 973rd in a debug one, with
        // each year's first and last day.
        let year_starts = (MIN_YEAR..=MAX_YEAR).map(new_year_raw);
        for rd in crate::sweep_days(EARLIEST.0, LATEST.0, 9_973, year_starts) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)));
        }
        assert_eq!(from_fixed(EARLIEST), Ok((MIN_YEAR, 1, 1)));
        assert_eq!(from_fixed(LATEST), Ok((MAX_YEAR, DECEMBER, 28)));
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
