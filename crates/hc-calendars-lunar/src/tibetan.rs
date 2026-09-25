//! The Tibetan calendar in the Phugpa tradition.
//!
//! A lunisolar calendar of twelve or thirteen months, each of thirty lunar
//! days, computed entirely by arithmetic: a mean motion for the date, the
//! sun and the moon's anomaly, two small tables in place of a sine, and a
//! leap-month rule on a 65-month cycle. A calendar day is named by the
//! lunar day that is current at its dawn, so a lunar day short enough to
//! end within one calendar day gives its number to none — the number is
//! *skipped* — and one long enough to contain a whole calendar day gives
//! it to two, the first of which is the *extra* (leap) day. The year is
//! numbered by the Western year it begins in, as Tibetans commonly number
//! it, and named in the sixty-year cycle: 2007 is the Fire–Pig year.
//!
//! # Whose arithmetic
//!
//! Svante Janson's *Tibetan Calendar Mathematics* (2014), which states the
//! Phugpa calculations in modern notation with exact rational constants
//! and the epoch of 806 from the Kālacakra Tantra. The true month count
//! is his (5.10), the leap-month rule his (5.8), the inverse his
//! (5.19)–(5.22), the mean date, mean sun and moon's anomaly his (7.1),
//! (7.5) and (7.11) with the almanacs' `a2 = 1/28`, the tables his (7.18)
//! and (7.21), the true date his (7.22), and the calendar day his (8.1)
//! with the rule of Section 6 for skipped and repeated days. Every constant
//! is carried as the rational it is, and every date is computed exactly.
//!
//! # What is not carried
//!
//! The Tsurphu and other traditions, which differ in their constants
//! (his Appendix A); Henning's alternative anomaly increment (7.24), which
//! moves about one day in four thousand; the almanac's further columns —
//! the true day of week's fraction, the lunar mansion, the yoga and the
//! karaṇa — of his Section 10; and the Mongolian and Bhutanese variants.
//!
//! Source: Svante Janson, "Tibetan calendar mathematics", arXiv:1401.6285,
//! revised 8 January 2014, retrieved 2026-09-22.

use core::fmt;

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, CivilTime, DateFields,
    DayBoundary, Month, Rd, YearKind,
};

/// Mean daybreak, 05:00 local mean solar time, when the Tibetan calendar day
/// begins.
///
/// The calendar day runs "from dawn to dawn" and is a constant 24 hours, so
/// no sunrise is computed; Janson's Remark 6 gives Henning's mean daybreak,
/// 5 a.m. local mean solar time, as the start (Janson, "Tibetan calendar
/// mathematics", Section 2 and Remark 6; Edward Henning, *Kālacakra and the
/// Tibetan Calendar*, 2007, pp. 10–11).
pub const DAWN: CivilTime = match CivilTime::hms(5, 0, 0) {
    Ok(time) => time,
    Err(_) => panic!("05:00 is a time of day"),
};

/// The epoch year of the Kālacakra Tantra reckoning, 806.
pub const EPOCH_YEAR: i64 = 806;

/// The epoch month, the third.
pub const EPOCH_MONTH: i64 = 3;

/// The initial intercalation index at the epoch, `β*` of the source.
pub const EPOCH_INDEX: i64 = 61;

/// `β = 184 − β*`, the constant of the inverse formulas.
pub const INVERSE_CONSTANT: i64 = 184 - EPOCH_INDEX;

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1000;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 3000;

/// A rational number, kept reduced, for the calendar's exact arithmetic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Ratio {
    num: i128,
    den: i128,
}

const fn gcd(mut a: i128, mut b: i128) -> i128 {
    if a < 0 {
        a = -a;
    }
    if b < 0 {
        b = -b;
    }
    while b != 0 {
        let rest = a % b;
        a = b;
        b = rest;
    }
    a
}

impl Ratio {
    const fn new(num: i128, den: i128) -> Self {
        let g = gcd(num, den);
        let (num, den) = if g == 0 {
            (num, den)
        } else {
            (num / g, den / g)
        };
        if den < 0 {
            Self {
                num: -num,
                den: -den,
            }
        } else {
            Self { num, den }
        }
    }

    const fn int(value: i128) -> Self {
        Self { num: value, den: 1 }
    }

    const fn add(self, other: Self) -> Self {
        Self::new(
            self.num * other.den + other.num * self.den,
            self.den * other.den,
        )
    }

    const fn sub(self, other: Self) -> Self {
        Self::new(
            self.num * other.den - other.num * self.den,
            self.den * other.den,
        )
    }

    const fn mul(self, other: Self) -> Self {
        Self::new(self.num * other.num, self.den * other.den)
    }

    const fn scale(self, factor: i128) -> Self {
        Self::new(self.num * factor, self.den)
    }

    const fn floor(self) -> i128 {
        self.num.div_euclid(self.den)
    }

    /// The fractional part, in `[0, 1)`.
    const fn frac(self) -> Self {
        Self::new(self.num.rem_euclid(self.den), self.den)
    }
}

/// The mean lunar month, `m1`, in days.
const M1: Ratio = Ratio::new(167_025, 5_656);
/// The mean lunar day, `m2 = m1 / 30`.
const M2: Ratio = Ratio::new(11_135, 11_312);
/// The mean date at the epoch, `m0`, as a Julian Date.
const M0: Ratio = Ratio::new(2_015_501 * 5_656 + 4_783, 5_656);
/// The sun's mean motion per month, `s1`, in revolutions.
const S1: Ratio = Ratio::new(65, 804);
/// The sun's mean motion per lunar day, `s2`.
const S2: Ratio = Ratio::new(13, 4_824);
/// The sun's mean longitude at the epoch, `s0`.
const S0: Ratio = Ratio::new(743, 804);
/// The moon's anomaly per month, `a1`.
const A1: Ratio = Ratio::new(253, 3_528);
/// The moon's anomaly per lunar day, `a2`, the almanacs' rounded value.
const A2: Ratio = Ratio::new(1, 28);
/// The moon's anomaly at the epoch, `a0`.
const A0: Ratio = Ratio::new(475, 3_528);

/// The moon's equation table, `moon_tab(i)` for `i = 0..=7`, extended by
/// symmetry to a period of 28.
const MOON_TABLE: [i128; 8] = [0, 5, 10, 15, 19, 22, 24, 25];
/// The sun's equation table, `sun_tab(i)` for `i = 0..=3`, extended by
/// symmetry to a period of 12.
const SUN_TABLE: [i128; 4] = [0, 6, 10, 11];

/// A table with the symmetries `tab(2h − i) = tab(i)` and
/// `tab(2h + i) = −tab(i)`, linearly interpolated.
fn table(values: &[i128], half: i128, x: Ratio) -> Ratio {
    let period = 4 * half;
    let mut x = Ratio::new(x.num.rem_euclid(x.den * period), x.den);
    let mut sign = 1;
    // Beyond the half period the table is negated, and within it mirrored.
    if x.num >= 2 * half * x.den {
        x = x.sub(Ratio::int(2 * half));
        sign = -1;
    }
    if x.num > half * x.den {
        x = Ratio::int(2 * half).sub(x);
    }
    let index = x.floor();
    let fraction = x.frac();
    let value = if index >= half {
        Ratio::int(values[half as usize])
    } else {
        let low = values[index as usize];
        let high = values[index as usize + 1];
        Ratio::int(low).add(fraction.scale(high - low))
    };
    value.scale(sign)
}

/// The true month count of month `month` (1–12) of `year`, or `None` when
/// a leap month is asked for in a month that has none.
#[must_use]
pub fn true_month_count(year: i64, month: u8, leap: bool) -> Option<i64> {
    let solar_months = 12 * (year - EPOCH_YEAR) + i64::from(month) - EPOCH_MONTH;
    let index = (2 * solar_months + EPOCH_INDEX).rem_euclid(65);
    if leap && index != 48 && index != 49 {
        return None;
    }
    Some((67 * solar_months + EPOCH_INDEX + 17).div_euclid(65) - i64::from(leap))
}

/// The year, month and leap flag of true month count `n`.
#[must_use]
pub fn month_of_count(n: i64) -> (i64, u8, bool) {
    let x = (65 * n + INVERSE_CONSTANT + 66).div_euclid(67);
    let month = (x - 1).rem_euclid(12) + 1;
    let year = (x + 11).div_euclid(12) - 1 + EPOCH_YEAR;
    let remainder = (65 * n + INVERSE_CONSTANT).rem_euclid(67);
    (year, month as u8, remainder == 1 || remainder == 2)
}

/// Whether `year` has a leap month, by the source's (5.41).
#[must_use]
pub fn is_leap_year(year: i64) -> bool {
    (24 * year + 33).rem_euclid(65) >= 41
}

/// The number of the leap month of `year`, or `None`, by the source's
/// (5.34).
#[must_use]
pub fn leap_month_of(year: i64) -> Option<u8> {
    if !is_leap_year(year) {
        return None;
    }
    let x = (24 * (year - EPOCH_YEAR) - INVERSE_CONSTANT).rem_euclid(65);
    Some((1 + (64 - x).div_euclid(2)) as u8)
}

/// The true date at the end of lunar day `day` of true month `n`, as a
/// Julian Date whose integer part is the calendar day.
fn true_date(day: i64, n: i64) -> Ratio {
    let n = Ratio::int(n as i128);
    let day = Ratio::int(day as i128);
    let mean_date = n.mul(M1).add(day.mul(M2)).add(M0);
    let mean_sun = n.mul(S1).add(day.mul(S2)).add(S0).frac();
    let anomaly_moon = n.mul(A1).add(day.mul(A2)).add(A0).frac();
    let moon_equation = table(&MOON_TABLE, 7, anomaly_moon.scale(28));
    let sun_equation = table(&SUN_TABLE, 3, mean_sun.sub(Ratio::new(1, 4)).scale(12));
    mean_date
        .add(moon_equation.mul(Ratio::new(1, 60)))
        .sub(sun_equation.mul(Ratio::new(1, 60)))
}

/// The Julian Day Number of the calendar day in which lunar day `day` of
/// true month `n` ends.
fn end_day(day: i64, n: i64) -> i64 {
    true_date(day, n).floor() as i64
}

/// The end day of the lunar day before `day` of month `n`: day 30 of the
/// previous month when `day` is 1.
fn previous_end_day(day: i64, n: i64) -> i64 {
    if day == 1 {
        end_day(30, n - 1)
    } else {
        end_day(day - 1, n)
    }
}

const JDN_OFFSET: i64 = 1_721_425;

/// The Julian Day Number of Losar, the first day of `year`: the day after
/// day 30 of the regular twelfth month of the year before.
#[must_use]
pub fn new_year_jdn(year: i64) -> i64 {
    let n = true_month_count(year - 1, 12, false).unwrap_or(0);
    end_day(30, n) + 1
}

/// The fixed day of Losar of `year`.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub fn new_year(year: i64) -> CalendarResult<Rd> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(CalendarError::YearOutOfRange);
    }
    Ok(Rd(new_year_jdn(year) - JDN_OFFSET))
}

/// The earliest fixed day this implementation converts, Losar of
/// [`MIN_YEAR`].
pub fn earliest() -> Rd {
    Rd(new_year_jdn(MIN_YEAR) - JDN_OFFSET)
}

/// The latest fixed day this implementation converts, the day before Losar
/// of the year after [`MAX_YEAR`].
pub fn latest() -> Rd {
    Rd(new_year_jdn(MAX_YEAR + 1) - 1 - JDN_OFFSET)
}

/// The five elements of the sixty-year cycle, two years each.
pub const ELEMENTS: [&str; 5] = ["Wood", "Fire", "Earth", "Iron", "Water"];

/// The twelve animals of the cycle, from the Mouse.
pub const ANIMALS: [&str; 12] = [
    "Mouse", "Ox", "Tiger", "Rabbit", "Dragon", "Snake", "Horse", "Sheep", "Monkey", "Bird", "Dog",
    "Pig",
];

/// The name of a year in the sixty-year cycle: its element, whether it is
/// male or female, and its animal. 2007 is Fire, female, Pig.
#[must_use]
pub fn year_name(year: i64) -> (&'static str, bool, &'static str) {
    let position = (year - 4).rem_euclid(60);
    (
        ELEMENTS[(position % 10 / 2) as usize],
        position % 2 == 0,
        ANIMALS[(position % 12) as usize],
    )
}

/// The year's place in the Prabhava (*rab byung*) cycles of sixty years
/// numbered from 1027: the cycle number and the year within it. 2007 is
/// the 21st year of the 17th cycle.
#[must_use]
pub fn prabhava(year: i64) -> (i64, i64) {
    (
        (year - 1026 + 59).div_euclid(60),
        (year - 1027).rem_euclid(60) + 1,
    )
}

/// A Tibetan date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TibetanDate {
    /// The year, numbered by the Western year it begins in.
    pub year: i64,
    /// The month, 1 to 12, with the leap month flagged; the leap month
    /// precedes the regular month of the same number.
    pub month: Month,
    /// The lunar day, 1 to 30.
    pub day: u8,
    /// Whether this is the first of two calendar days with the same
    /// number, the *extra* day of the almanacs.
    pub leap_day: bool,
}

impl fmt::Display for TibetanDate {
    /// Writes the date as year, month and day, marking a leap month and an
    /// extra day: `2000-1L-1`, `2024-6-15x`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.year, self.month.ordinal)?;
        if self.month.leap {
            write!(f, "L")?;
        }
        write!(f, "-{}", self.day)?;
        if self.leap_day {
            write!(f, "x")?;
        }
        Ok(())
    }
}

/// The fixed day of a Tibetan date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside the years converted,
/// [`CalendarError::MonthOutOfRange`] for a month outside `1..=12` or a
/// leap month the year does not have, and [`CalendarError::DayOutOfRange`]
/// for a day outside `1..=30`, a day the calendar skips, or an extra day
/// where the day is not repeated.
pub fn to_fixed(date: TibetanDate) -> CalendarResult<Rd> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&date.year) {
        return Err(CalendarError::YearOutOfRange);
    }
    if date.month.ordinal == 0 || date.month.ordinal > 12 {
        return Err(CalendarError::MonthOutOfRange);
    }
    if date.day == 0 || date.day > 30 {
        return Err(CalendarError::DayOutOfRange);
    }
    let n = true_month_count(date.year, date.month.ordinal, date.month.leap)
        .ok_or(CalendarError::MonthOutOfRange)?;
    let day = i64::from(date.day);
    let end = end_day(day, n);
    let before = previous_end_day(day, n);
    match end - before {
        0 => Err(CalendarError::DayOutOfRange),
        1 if date.leap_day => Err(CalendarError::DayOutOfRange),
        1 => Ok(Rd(end - JDN_OFFSET)),
        _ if date.leap_day => Ok(Rd(before + 1 - JDN_OFFSET)),
        _ => Ok(Rd(end - JDN_OFFSET)),
    }
}

/// The Tibetan date of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside the years converted.
pub fn from_fixed(rd: Rd) -> CalendarResult<TibetanDate> {
    if rd < earliest() {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd > latest() {
        return Err(CalendarError::AfterSupportedRange);
    }
    let jdn = rd.0 + JDN_OFFSET;
    let estimate = Ratio::int(jdn as i128)
        .sub(M0)
        .mul(Ratio::new(M1.den, M1.num))
        .floor() as i64;
    for n in [estimate - 1, estimate, estimate + 1] {
        let mut before = previous_end_day(1, n);
        if before >= jdn {
            continue;
        }
        for day in 1..=30 {
            let end = end_day(day, n);
            if before < jdn && jdn <= end {
                let (year, month, leap) = month_of_count(n);
                return Ok(TibetanDate {
                    year,
                    month: if leap {
                        Month::leap(month)
                    } else {
                        Month::regular(month)
                    },
                    day: day as u8,
                    leap_day: end - before == 2 && jdn == before + 1,
                });
            }
            before = end;
        }
    }
    Err(CalendarError::DayOutOfRange)
}

/// The Tibetan calendar of the Phugpa tradition.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TibetanCalendar;

impl Calendar for TibetanCalendar {
    type Date = TibetanDate;

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::LUNISOLAR_TWELVE
    }

    /// A year with a doubled month.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(is_leap_year(year))
    }

    /// Mean daybreak, [`DAWN`]: the Tibetan day runs from dawn to dawn.
    fn day_boundary(&self) -> DayBoundary {
        DayBoundary::LocalTime(DAWN)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("tibetan"),
            english_name: "Tibetan (Phugpa)",
            year_kind: YearKind::EpochForward,
            has_leap_months: true,
            is_astronomical: false,
            earliest: Some(earliest()),
            latest: Some(latest()),
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        from_fixed(rd)
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let mut fields = DateFields::ymd(date.year, date.month.ordinal, date.day);
        fields.month = Some(date.month);
        fields.leap_day = date.leap_day;
        Ok(fields)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let date = TibetanDate {
            year: fields.year,
            month: fields.require_month()?,
            day: fields.require_day()?,
            leap_day: fields.leap_day,
        };
        to_fixed(date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::civil;

    #[test]
    fn the_day_begins_at_mean_daybreak() {
        assert_eq!(
            TibetanCalendar.day_boundary(),
            DayBoundary::LocalTime(CivilTime::hms(5, 0, 0).unwrap())
        );
    }

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        civil::to_rd(year, month, day)
    }

    #[test]
    fn the_sources_own_dates_and_losar_of_2000_are_reproduced() {
        // The paper is dated 31 December 2007, day 23 of month 11 of the
        // Fire–Pig year, and revised 8 January 2014, day 8 of month 11 of the
        // Water–Snake year.
        let first = from_fixed(greg(2007, 12, 31)).expect("in range");
        assert_eq!(
            (first.year, first.month, first.day, first.leap_day),
            (2007, Month::regular(11), 23, false)
        );
        assert_eq!(year_name(2007), ("Fire", false, "Pig"));
        let second = from_fixed(greg(2014, 1, 8)).expect("in range");
        assert_eq!(
            (second.year, second.month, second.day),
            (2013, Month::regular(11), 8)
        );
        assert_eq!(year_name(2013), ("Water", false, "Snake"));
        assert_eq!(prabhava(2007), (17, 21));
        // Losar 2000 fell on Sunday 6 February, the first day of a leap
        // month 1 (the source's footnote 28).
        assert_eq!(new_year(2000), Ok(greg(2000, 2, 6)));
        let losar = from_fixed(greg(2000, 2, 6)).expect("in range");
        assert_eq!(
            (losar.year, losar.month, losar.day),
            (2000, Month::leap(1), 1)
        );
        assert_eq!(leap_month_of(2000), Some(1));
        assert_eq!(losar.to_string(), "2000-1L-1");
    }

    #[test]
    fn losar_falls_on_the_published_days_of_recent_years() {
        for (year, month, day) in [(2023, 2, 21), (2024, 2, 10), (2025, 2, 28), (2026, 2, 18)] {
            assert_eq!(new_year(year), Ok(greg(year, month, day)), "{year}");
            let date = from_fixed(greg(year, month, day)).expect("in range");
            assert_eq!(
                (date.year, date.month.ordinal, date.day),
                (year, 1, 1),
                "{year}"
            );
        }
        // Saga Dawa Düchen, the fifteenth of the fourth month, 23 May 2024.
        let saga_dawa = from_fixed(greg(2024, 5, 23)).expect("in range");
        assert_eq!(
            (saga_dawa.year, saga_dawa.month, saga_dawa.day),
            (2024, Month::regular(4), 15)
        );
        assert_eq!(year_name(2024), ("Wood", true, "Dragon"));
    }

    #[test]
    fn leap_years_follow_the_sixty_five_year_rule() {
        let leap: Vec<i64> = (2000..=2030).filter(|year| is_leap_year(*year)).collect();
        assert_eq!(
            leap,
            [
                2000, 2002, 2005, 2008, 2010, 2013, 2016, 2019, 2021, 2024, 2027, 2029
            ]
        );
        assert_eq!(leap_month_of(2024), Some(6));
        assert_eq!(leap_month_of(2023), None);
        assert!(true_month_count(2024, 6, true).is_some());
        assert_eq!(true_month_count(2024, 5, true), None);
        // The inverse of the true month count is exact across a cycle.
        for n in 14_000..14_900 {
            let (year, month, leap) = month_of_count(n);
            assert_eq!(true_month_count(year, month, leap), Some(n), "n {n}");
        }
    }

    #[test]
    fn every_day_of_four_decades_round_trips_and_years_have_the_five_lengths() {
        let calendar = TibetanCalendar;
        let start = greg(1990, 1, 1).0;
        let end = greg(2031, 1, 1).0;
        let mut previous: Option<TibetanDate> = None;
        let mut skipped = 0;
        let mut repeated = 0;
        for rd in start..end {
            let date = calendar.from_fixed(Rd(rd)).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "rd {rd} {date}");
            let fields = calendar.to_fields(date).expect("describable");
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            if let Some(before) = previous
                && before.month == date.month
                && before.year == date.year
            {
                if before.day == date.day {
                    repeated += 1;
                    assert!(before.leap_day && !date.leap_day, "rd {rd}");
                } else {
                    assert!(date.day > before.day, "rd {rd}");
                    if date.day - before.day == 2 {
                        skipped += 1;
                    }
                }
            }
            previous = Some(date);
        }
        assert!(skipped > 0 && repeated > 0);
        for year in 1990..2030 {
            let length = new_year(year + 1).unwrap().0 - new_year(year).unwrap().0;
            assert!(
                [354, 355, 383, 384, 385].contains(&length),
                "{year}: {length}"
            );
            assert_eq!(length >= 383, is_leap_year(year), "{year}");
        }
    }

    #[test]
    fn impossible_dates_are_refused() {
        assert_eq!(
            to_fixed(TibetanDate {
                year: 2023,
                month: Month::leap(3),
                day: 1,
                leap_day: false
            }),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(TibetanDate {
                year: 2023,
                month: Month::regular(13),
                day: 1,
                leap_day: false
            }),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            to_fixed(TibetanDate {
                year: 2023,
                month: Month::regular(1),
                day: 31,
                leap_day: false
            }),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            to_fixed(TibetanDate {
                year: 999,
                month: Month::regular(1),
                day: 1,
                leap_day: false
            }),
            Err(CalendarError::YearOutOfRange)
        );
        // Losar of 2024 is not an extra day.
        assert_eq!(
            to_fixed(TibetanDate {
                year: 2024,
                month: Month::regular(1),
                day: 1,
                leap_day: true
            }),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            from_fixed(Rd(earliest().0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(latest().0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }
}
