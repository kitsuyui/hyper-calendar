//! The 33-year calendars of John Dee: the Dee–Cecil and Dee calendars.
//!
//! The Julian and Gregorian months with another leap rule: a year is leap
//! when its number, divided by 33, leaves a remainder that is not zero and
//! is a multiple of 4 — the 4th, 8th, …, 32nd years of each 33, eight leap
//! years in 33 and a mean year of 365.2424̄ days, closer to the mean vernal
//! equinox year than the Gregorian 365.2425. Peter Meyer attributes it to
//! John Dee, who advised Elizabeth I on the reform of 1582, and gives two
//! correlations, which are two calendars under policy §5: the **Dee–Cecil**
//! calendar, 1-1-1 on Julian Day Number 1 721 426, the Gregorian 1 January
//! 1, named for William Cecil, who preferred dropping ten days to Dee's
//! eleven; and the **Dee** calendar, 1-1-1 on JDN 1 721 425, one day
//! earlier, so that every day carries the Dee–Cecil date of the day after.
//! Years are numbered astronomically.
//!
//! The Dee–Cecil calendar and the Gregorian agree from 1 March 1980 to
//! 28 February 2016 — their leap years are the same from 1981 to 2015 —
//! and part at 29 February 2016, which the Gregorian calendar has and this
//! one does not. Simon Cassidy's "Anni-Domini" rule of 1996, "February
//! will have 29 days whenever the A.D. year-number, reduced modulo 33, is
//! non-zero and divisible by 4", is the same rule on the same days, which
//! he describes as "in effect since March 1st. 1980": it is this calendar,
//! not another, and has no identifier of its own. His proposal to drop
//! 29 February 2000 as well is Dee's eleventh day, and from 1 March 2000
//! reads as the Dee calendar. The system document is
//! `docs/systems/hermetic-reforms.md`.
//!
//! # Sources
//!
//! * Peter Meyer, "The Dee-Cecil Calendar and its Date Conversion
//!   Algorithms", Hermetic Systems,
//!   <https://www.hermetic.ch/cal_stud/dee-cecil-calendar.htm>, retrieved
//!   2026-09-26 (`meyer-dee-cecil`): the rule and its examples, the two
//!   correlations and their names, the agreement from 1 March 1980 to
//!   28 February 2016, and the conversion functions `IsDeeLeapYear`,
//!   `Dee2JDN` and `JDN2Dee`, which the tests run as the reference.
//! * Simon Cassidy, "Implementing a correct 33-year calendar reform",
//!   message to the East Carolina University calendar list, 9 October
//!   1996, Hermetic Systems,
//!   <https://www.hermetic.ch/cal_stud/cassidy/33yr-cal.htm>, retrieved
//!   2026-09-26 (`cassidy-33-year`): the rule in his words, the reduction
//!   procedure and its examples, the agreement over 1981–2015 and
//!   1585–1619, and the proposal for 2000.
//! * Duncan Steel, "The Non-implemented 33-Year English Protestant
//!   Calendar", Hermetic Systems, <https://www.hermetic.ch/cal_stud/dst01.htm>,
//!   retrieved 2026-09-26 (`steel-33-year`), for the history only.
//!
//! # Exactness
//!
//! Exact: the rule is the definition, and Meyer's functions agree with it
//! on every day tested.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};

use crate::common;

/// The earliest year this implementation converts.
pub use crate::gregorian::MIN_YEAR;

/// The latest year this implementation converts.
pub use crate::gregorian::MAX_YEAR;

/// Years in the leap cycle.
pub const CYCLE_YEARS: i64 = 33;

/// Leap years in a cycle.
pub const LEAPS_PER_CYCLE: i64 = 8;

/// Days in a cycle: 25 years of 365 days and 8 of 366.
pub const CYCLE_DAYS: i64 = 365 * CYCLE_YEARS + LEAPS_PER_CYCLE;

/// Whether `year` is leap: `year mod 33` is not zero and is divisible by 4.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    let remainder = year.rem_euclid(CYCLE_YEARS);
    remainder != 0 && remainder % 4 == 0
}

/// Leap years from year 1 through `year`, negative before year 1.
///
/// Eight a cycle, and in the part cycle one for each whole four years of
/// the remainder — which also counts nothing for remainder zero.
const fn leap_years_through(year: i64) -> i64 {
    LEAPS_PER_CYCLE * year.div_euclid(CYCLE_YEARS) + year.rem_euclid(CYCLE_YEARS) / 4
}

/// Days from 1-1-1 to 1 January of `year`.
const fn days_before_year(year: i64) -> i64 {
    365 * (year - 1) + leap_years_through(year - 1)
}

/// The number of days in `month` of `year`, or `None` outside `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    common::julian_style_days_in_month(month, is_leap_year(year))
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// A 33-year calendar: the Dee–Cecil or the Dee correlation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeeCalendar {
    /// The calendar's identifier.
    pub id: &'static str,
    /// Its English name.
    pub english_name: &'static str,
    /// The fixed day of 1-1-1.
    pub epoch: Rd,
}

/// The Dee–Cecil calendar: 1-1-1 on JDN 1 721 426, the Gregorian 1 January 1.
pub const DEE_CECIL: DeeCalendar = DeeCalendar {
    id: "dee-cecil",
    english_name: "Dee–Cecil",
    epoch: Rd::from_julian_day_number(1_721_426),
};

/// The Dee calendar: 1-1-1 on JDN 1 721 425, a day before the Dee–Cecil.
pub const DEE: DeeCalendar = DeeCalendar {
    id: "dee",
    english_name: "Dee (eleven-day correction)",
    epoch: Rd::from_julian_day_number(1_721_425),
};

/// Both correlations, in registration order.
pub const ALL: [DeeCalendar; 2] = [DEE_CECIL, DEE];

impl Default for DeeCalendar {
    fn default() -> Self {
        DEE_CECIL
    }
}

impl DeeCalendar {
    /// The fixed day of 1 January of `year`.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] outside
    /// [`MIN_YEAR`]..=[`MAX_YEAR`].
    pub const fn new_year(self, year: i64) -> CalendarResult<Rd> {
        if year < MIN_YEAR || year > MAX_YEAR {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(Rd(self.epoch.0 + days_before_year(year)))
    }

    /// The earliest fixed day converted.
    #[must_use]
    pub const fn earliest(self) -> Rd {
        Rd(self.epoch.0 + days_before_year(MIN_YEAR))
    }

    /// The latest fixed day converted.
    #[must_use]
    pub const fn latest(self) -> Rd {
        Rd(self.epoch.0 + days_before_year(MAX_YEAR + 1) - 1)
    }

    /// The fixed day of a date.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`],
    /// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
    pub const fn ymd_to_fixed(self, year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
        let start = match self.new_year(year) {
            Ok(start) => start,
            Err(error) => return Err(error),
        };
        let leap = is_leap_year(year);
        match common::check_day(day, common::julian_style_days_in_month(month, leap)) {
            Err(error) => Err(error),
            Ok(()) => Ok(Rd(start.0
                + common::julian_style_day_of_year(month, day, leap) as i64
                - 1)),
        }
    }

    /// The year, month and day of a fixed day.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::BeforeEpoch`] or
    /// [`CalendarError::AfterSupportedRange`] outside the supported range.
    pub const fn fixed_to_ymd(self, rd: Rd) -> CalendarResult<(i64, u8, u8)> {
        if rd.0 < self.earliest().0 {
            return Err(CalendarError::BeforeEpoch);
        }
        if rd.0 > self.latest().0 {
            return Err(CalendarError::AfterSupportedRange);
        }
        let elapsed = rd.0 - self.epoch.0;
        // The cycle's own mean year lands within a year of the answer.
        let mut year = (elapsed * CYCLE_YEARS).div_euclid(CYCLE_DAYS) + 1;
        while days_before_year(year) > elapsed {
            year -= 1;
        }
        while days_before_year(year + 1) <= elapsed {
            year += 1;
        }
        let day_of_year = (elapsed - days_before_year(year) + 1) as u16;
        let (month, day) = common::julian_style_month_day(day_of_year, is_leap_year(year));
        Ok((year, month, day))
    }
}

/// A date in either 33-year calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeeDate {
    /// The year, numbered astronomically.
    pub year: i64,
    /// The month, 1 through 12.
    pub month: u8,
    /// The day of the month.
    pub day: u8,
}

impl DeeDate {
    /// A validated date. The months are the same in both correlations.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date does not exist.
    pub const fn new(year: i64, month: u8, day: u8) -> CalendarResult<Self> {
        if year < MIN_YEAR || year > MAX_YEAR {
            return Err(CalendarError::YearOutOfRange);
        }
        match common::check_day(day, days_in_month(year, month)) {
            Err(error) => Err(error),
            Ok(()) => Ok(Self { year, month, day }),
        }
    }
}

impl Calendar for DeeCalendar {
    type Date = DeeDate;

    /// Unrecorded: a proposal, adopted by nobody.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(self.id),
            english_name: self.english_name,
            year_kind: YearKind::Astronomical,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(self.earliest()),
            latest: Some(self.latest()),
            native_locales: &[],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        self.ymd_to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = self.fixed_to_ymd(rd)?;
        Ok(DeeDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields::ymd(date.year, date.month, date.day))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        DeeDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian;

    /// Meyer's `IsDeeLeapYear`, as published.
    fn meyer_is_leap(year: i64) -> bool {
        if year >= 1 {
            ((year - 1) % 33) % 4 == 3
        } else {
            ((-year) % 33) % 4 == 1
        }
    }

    /// Meyer's `NumDaysInDeeMonth`.
    fn meyer_days_in_month(year: i64, month: i64) -> i64 {
        match month {
            2 if meyer_is_leap(year) => 29,
            2 => 28,
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }

    /// Meyer's `Dee2JDN`, as published, with `\` and `Mod` on non-negative
    /// operands only, as his page requires.
    fn meyer_dee_to_jdn(jdn111: i64, year: i64, month: i64, day: i64) -> i64 {
        let (cycle, y) = if year >= 1 {
            ((year - 1) / 33, (year - 1) % 33)
        } else {
            (-(-year / 33) - 1, 32 - ((-year) % 33))
        };
        let mut x = cycle * 12_053 + y * 365 + y / 4;
        for m in 1..month {
            x += meyer_days_in_month(year, m);
        }
        x + day + jdn111 - 1
    }

    /// Meyer's `JDN2Dee`, as published.
    fn meyer_jdn_to_dee(jdn111: i64, jdn: i64) -> (i64, i64, i64) {
        let x = jdn - jdn111;
        let (mut year, mut d) = if x >= 0 {
            (1 + (x / 12_053) * 33, x % 12_053)
        } else {
            (
                -(((-1 - x) / 12_053) * 33) - 32,
                12_052 - ((-x - 1) % 12_053),
            )
        };
        for i in 1..=33 {
            let length = if meyer_is_leap(i) { 366 } else { 365 };
            if d < length {
                break;
            }
            year += 1;
            d -= length;
        }
        let mut month = 1;
        for i in 1..=12 {
            let length = meyer_days_in_month(year, i);
            if d < length {
                break;
            }
            month += 1;
            d -= length;
        }
        (year, month, d + 1)
    }

    fn gregorian(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn the_leap_rule_is_meyers_and_his_examples_hold() {
        // "2009 is not a leap year ... 2012 is a leap year ... 2013 is not".
        assert!(!is_leap_year(2009));
        assert!(is_leap_year(2012));
        assert!(!is_leap_year(2013));
        // 2017 is leap here and not in the Gregorian calendar.
        assert!(is_leap_year(2017) && !gregorian::is_leap_year(2017));
        for year in -3_000..=3_000 {
            assert_eq!(is_leap_year(year), meyer_is_leap(year), "{year}");
        }
        assert!(
            (1..=33)
                .filter(|year| is_leap_year(*year))
                .eq([4, 8, 12, 16, 20, 24, 28, 32])
        );
    }

    #[test]
    fn meyers_conversion_functions_are_the_reference() {
        for calendar in ALL {
            let jdn111 = calendar.epoch.to_julian_day_number();
            for jdn in (1_000_000..=3_000_000).step_by(97) {
                let (year, month, day) = calendar
                    .fixed_to_ymd(Rd::from_julian_day_number(jdn))
                    .unwrap();
                assert_eq!(
                    meyer_jdn_to_dee(jdn111, jdn),
                    (year, i64::from(month), i64::from(day)),
                    "{} JDN {jdn}",
                    calendar.id
                );
                assert_eq!(
                    meyer_dee_to_jdn(jdn111, year, i64::from(month), i64::from(day)),
                    jdn,
                    "{} {year}-{month}-{day}",
                    calendar.id
                );
            }
        }
    }

    #[test]
    fn one_one_one_is_the_day_meyer_gives() {
        // "1-1-1 DC ... the day with Julian day number 1,721,426. This is
        // the same day as 1-1-1 CE."
        assert_eq!(DEE_CECIL.ymd_to_fixed(1, 1, 1), Ok(gregorian(1, 1, 1)));
        assert_eq!(
            DEE_CECIL
                .ymd_to_fixed(1, 1, 1)
                .unwrap()
                .to_julian_day_number(),
            1_721_426
        );
        assert_eq!(
            DEE.ymd_to_fixed(1, 1, 1).unwrap().to_julian_day_number(),
            1_721_425
        );
    }

    /// "days back to March 1, 1980, and days forward to February 28, 2016,
    /// have the same dates in the Dee-Cecil Calendar as in the Gregorian",
    /// and the day after is 29 February in one and 1 March in the other.
    #[test]
    fn dee_cecil_agrees_with_the_gregorian_calendar_from_1980_to_2016() {
        let first = gregorian(1980, 3, 1);
        let last = gregorian(2016, 2, 28);
        for rd in first.0..=last.0 {
            assert_eq!(
                DEE_CECIL.fixed_to_ymd(Rd(rd)),
                gregorian::from_fixed(Rd(rd))
            );
        }
        assert_ne!(
            DEE_CECIL.fixed_to_ymd(Rd(first.0 - 1)),
            gregorian::from_fixed(Rd(first.0 - 1))
        );
        let next = Rd(last.0 + 1);
        assert_eq!(gregorian::from_fixed(next), Ok((2016, 2, 29)));
        assert_eq!(DEE_CECIL.fixed_to_ymd(next), Ok((2016, 3, 1)));
        assert_eq!(
            DEE_CECIL.ymd_to_fixed(2016, 2, 29),
            Err(CalendarError::DayOutOfRange)
        );
        // 2017 is leap here, so the two agree again from 1 March 2017, and
        // part at 2020, 7 mod 33, the system document's worked example.
        assert_eq!(
            DEE_CECIL.fixed_to_ymd(gregorian(2017, 2, 28)),
            Ok((2017, 2, 29))
        );
        assert_eq!(
            DEE_CECIL.fixed_to_ymd(gregorian(2017, 3, 1)),
            Ok((2017, 3, 1))
        );
        assert_eq!(
            DEE_CECIL.fixed_to_ymd(gregorian(2020, 2, 29)),
            Ok((2020, 3, 1))
        );
    }

    /// Cassidy's claim: the Gregorian and Anni-Domini leap years are the
    /// same from 1981 to 2015, as they were over 1585–1619, and his
    /// reductions of 2012, 1996 and 2016.
    #[test]
    fn cassidys_rule_is_this_rule_and_his_spans_hold() {
        for span in [1_981..=2_015, 1_585..=1_619] {
            for year in span {
                assert_eq!(is_leap_year(year), gregorian::is_leap_year(year), "{year}");
            }
        }
        for year in [1_980, 2_016, 1_584, 1_620] {
            assert_ne!(is_leap_year(year), gregorian::is_leap_year(year), "{year}");
        }
        // "1583 should be a leap year not 1584".
        assert!(is_leap_year(1_583) && !is_leap_year(1_584));
        // 2012 reduces to 32, 1996 to 16 (leap), 2016 to 3 (not).
        assert!(is_leap_year(2_012) && is_leap_year(1_996) && !is_leap_year(2_016));
    }

    /// Cassidy's proposal to make 2000 a common year is the Dee calendar:
    /// the Gregorian 29 February 2000 is its 1 March.
    #[test]
    fn dropping_29_february_2000_is_the_dee_calendar() {
        let leap_day = gregorian(2000, 2, 29);
        assert_eq!(DEE_CECIL.fixed_to_ymd(leap_day), Ok((2000, 2, 29)));
        assert_eq!(DEE.fixed_to_ymd(leap_day), Ok((2000, 3, 1)));
        for rd in (gregorian(1900, 1, 1).0..gregorian(2100, 1, 1).0).step_by(13) {
            assert_eq!(DEE.fixed_to_ymd(Rd(rd)), DEE_CECIL.fixed_to_ymd(Rd(rd + 1)));
        }
    }

    #[test]
    fn thirteen_thousand_two_hundred_years_are_a_day_short_of_the_gregorian() {
        // 400 cycles are 4 821 200 days, the Gregorian 13 200 years 4 821 201.
        let dee = DEE_CECIL.ymd_to_fixed(13_201, 1, 1).unwrap().0
            - DEE_CECIL.ymd_to_fixed(1, 1, 1).unwrap().0;
        assert_eq!(dee, 4_821_200);
        assert_eq!(gregorian(13_201, 1, 1).0 - gregorian(1, 1, 1).0, 4_821_201);
    }

    #[test]
    fn the_calendar_round_trips_and_refuses_what_does_not_exist() {
        for calendar in ALL {
            for rd in (-2_000_000..=2_000_000).step_by(211) {
                let date = Calendar::from_fixed(&calendar, Rd(rd)).unwrap();
                assert_eq!(Calendar::to_fixed(&calendar, date), Ok(Rd(rd)));
                let fields = calendar.to_fields(date).unwrap();
                assert_eq!(calendar.from_fields(&fields), Ok(date));
            }
            assert!(calendar.meta().supports(calendar.earliest()));
            assert_eq!(
                calendar.fixed_to_ymd(Rd(calendar.earliest().0 - 1)),
                Err(CalendarError::BeforeEpoch)
            );
            assert_eq!(
                calendar.fixed_to_ymd(Rd(calendar.latest().0 + 1)),
                Err(CalendarError::AfterSupportedRange)
            );
        }
        assert_eq!(
            DeeDate::new(2013, 13, 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            DeeDate::new(MAX_YEAR + 1, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(DeeCalendar::default(), DEE_CECIL);
    }
}
