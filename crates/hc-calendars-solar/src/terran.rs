//! The Terran Computational Calendar, an instant system over TAI.
//!
//! A date of this calendar, `42.13.1,0.0.0 TC`, is the years, months, days,
//! hours, minutes and seconds that have elapsed since its epoch 0TC,
//! 221 788 790 SI seconds before 1977-01-01 00:00:00 TAI, every field
//! counted from zero. A year is thirteen months of 28 days and a
//! *minimonth*, month 13, holding the year's leap days — one, or two in a
//! year divisible by 4 and not by 128 — and the IERS leap seconds that fall
//! within the year. A year base, `TCn`, counts only the leap seconds of the
//! years before nTC, and `TC0` none.
//!
//! Because a TC day begins where the leap seconds so far put it, and not at
//! a civil midnight, the calendar is a pair of functions between
//! [`Instant<Tai>`] and [`TerranDate`], not a [`hc_calendar::Calendar`] over
//! fixed days, and it is not registered; `docs/systems/terran-computational.md`
//! gives the reasons and a worked example. The leap seconds are
//! [`hc_core::leap::TABLE`], and an instant past its announced validity is
//! refused under `TC`, where an unannounced leap second would move it.
//!
//! # Sources
//!
//! * "Terran Computational Calendar", <https://terrancalendar.com/>,
//!   retrieved 2026-09-26 (`terran-calendar`), under the Creative Commons
//!   Public Domain Mark: the epoch and its derivation, the months and the
//!   minimonth, the leap-day rule, year bases, the notation, the tables of
//!   leap seconds and of the seasons of 2010–2020, and the datemod
//!   examples.
//! * The leap seconds: the IANA `leap-seconds.list`, through
//!   [`hc_core::leap`] (`iana-leap-seconds-list`).
//!
//! # Exactness
//!
//! Exact to the attosecond, given the leap-second table. The site's caveat
//! that dates "before 1977 TAI may not be exact" concerns the realisation
//! of TAI, which [`Instant<Tai>`] does not model.

use core::fmt;

use hc_core::leap;
use hc_core::{Duration, Instant, Tai, TimeError, TimeResult};

/// The TAI reading of 0TC, from 1970-01-01 00:00:00 TAI: 221 788 790
/// seconds before 1977-01-01 00:00:00 TAI, which is 220 924 800 seconds
/// after that origin — 1969-12-22 00:00:10 TAI.
pub const EPOCH_TAI_SECONDS: i128 = 220_924_800 - 221_788_790;

/// 0TC as a TAI instant.
pub const EPOCH: Instant<Tai> = Instant::from_epoch(Duration::from_secs(EPOCH_TAI_SECONDS));

/// Seconds in a day.
const DAY: i128 = 86_400;

/// Days in the thirteen full months of a year.
pub const MONTHS_DAYS: i64 = 13 * 28;

/// The month number of the minimonth.
pub const MINIMONTH: u8 = 13;

/// The earliest year converted.
pub const MIN_YEAR: i64 = -1_000_000_000;

/// The latest year converted.
pub const MAX_YEAR: i64 = 1_000_000_000;

/// Which leap seconds a date counts: its designator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum YearBase {
    /// `TC`: every leap second.
    #[default]
    Full,
    /// `TCn`: only the leap seconds of the years before nTC; `TC0` none.
    Base(u32),
}

impl YearBase {
    /// The first year whose leap seconds are left out.
    const fn limit(self) -> i64 {
        match self {
            Self::Full => i64::MAX,
            Self::Base(year) => year as i64,
        }
    }
}

/// The leap days of `year`: one, or two when it is divisible by 4 and not
/// by 128.
#[must_use]
pub const fn leap_days(year: i64) -> u8 {
    if year.rem_euclid(4) == 0 && year.rem_euclid(128) != 0 {
        2
    } else {
        1
    }
}

/// Days from 0TC to the start of `year`, before any leap second.
const fn days_before(year: i64) -> i64 {
    // The years 0 .. year − 1 each have one leap day, and the multiples of
    // 4 among them a second unless they are multiples of 128.
    (MONTHS_DAYS + 1) * year + (year + 3).div_euclid(4) - (year + 127).div_euclid(128)
}

/// The number of steps in the leap-second table.
const STEPS: usize = leap::TABLE.len() - 1;

/// The TC timestamp at which the `k`th step's leap second begins, and the
/// seconds it adds: the new `TAI − UTC` applies from the step's Unix
/// second, so the inserted second began at that second under the old one.
const fn step(k: usize) -> (i128, i128) {
    let before = leap::TABLE[k];
    let after = leap::TABLE[k + 1];
    (
        after.start_unix as i128 + before.tai_minus_utc as i128 - EPOCH_TAI_SECONDS,
        (after.tai_minus_utc - before.tai_minus_utc) as i128,
    )
}

/// The TC year each leap second falls in, under full accounting: the year
/// whose start precedes it and whose end, its own minimonth included,
/// follows it.
const fn assign_years() -> [i64; STEPS] {
    let mut years = [0; STEPS];
    let mut year = 0;
    let mut start = 0;
    let mut in_year = 0;
    let mut k = 0;
    while k < STEPS {
        let (at, seconds) = step(k);
        let end = start + DAY * (MONTHS_DAYS + leap_days(year) as i64) as i128 + in_year;
        if at < end {
            years[k] = year;
            in_year += seconds;
            k += 1;
        } else {
            start = end;
            year += 1;
            in_year = 0;
        }
    }
    years
}

/// The TC year of each leap second in [`hc_core::leap::TABLE`], in order.
pub const LEAP_SECOND_YEARS: [i64; STEPS] = assign_years();

/// Leap seconds counted before `year` under `base`.
const fn leap_seconds_before(year: i64, base: YearBase) -> i128 {
    let limit = if year < base.limit() {
        year
    } else {
        base.limit()
    };
    let mut total = 0;
    let mut k = 0;
    while k < STEPS {
        if LEAP_SECOND_YEARS[k] < limit {
            total += step(k).1;
        }
        k += 1;
    }
    total
}

/// The leap seconds of `year` itself under `base`: those the IERS issued
/// within its bounds, or none when the base leaves the year out.
#[must_use]
pub const fn leap_seconds(year: i64, base: YearBase) -> i64 {
    if year >= base.limit() {
        return 0;
    }
    let mut total = 0;
    let mut k = 0;
    while k < STEPS {
        if LEAP_SECOND_YEARS[k] == year {
            total += step(k).1 as i64;
        }
        k += 1;
    }
    total
}

/// Seconds from 0TC to the start of `year` under `base`.
const fn year_start_seconds(year: i64, base: YearBase) -> i128 {
    DAY * days_before(year) as i128 + leap_seconds_before(year, base)
}

/// Seconds in the minimonth of `year` under `base`.
const fn minimonth_seconds(year: i64, base: YearBase) -> i128 {
    DAY * leap_days(year) as i128 + leap_seconds(year, base) as i128
}

/// The last TC second the leap-second table speaks for.
const VALID_UNTIL: i128 = leap::table_valid_until_unix() as i128
    + leap::TABLE[leap::TABLE.len() - 1].tai_minus_utc as i128
    - EPOCH_TAI_SECONDS;

/// Whether `base` asks only for leap seconds the table has announced.
const fn check_base(base: YearBase) -> TimeResult<()> {
    match base {
        YearBase::Full => Ok(()),
        YearBase::Base(year) => {
            if year_start_seconds(year as i64, YearBase::Full) > VALID_UNTIL {
                Err(TimeError::AfterModelEnd)
            } else {
                Ok(())
            }
        }
    }
}

/// The instant `year` begins under `base`: `year.0.0,0.0.0 TC`.
///
/// # Errors
///
/// [`TimeError::OutOfRange`] outside [`MIN_YEAR`]..=[`MAX_YEAR`], and
/// [`TimeError::AfterModelEnd`] when the base, or under `TC` the year
/// itself, lies past the leap-second table.
pub fn year_start(year: i64, base: YearBase) -> TimeResult<Instant<Tai>> {
    to_tai(TerranDate::new(year, 0, 0, 0, 0, 0, base)?)
}

/// The seconds since 0TC of an instant: the `TC+` timestamp.
///
/// # Errors
///
/// [`TimeError::Overflow`] when the difference is not representable.
pub fn timestamp(instant: Instant<Tai>) -> TimeResult<Duration> {
    instant.duration_since(EPOCH)
}

/// A Terran Computational date: the time elapsed since 0TC, field by field,
/// and the designator that says which leap seconds it counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerranDate {
    /// The year, any integer.
    pub year: i64,
    /// The month, 0 to 12, or 13 for the minimonth.
    pub month: u8,
    /// The day of the month, 0 to 27; in the minimonth, the leap days and
    /// then one more day holding the leap seconds.
    pub day: u8,
    /// The hour, 0 to 23.
    pub hour: u8,
    /// The minute, 0 to 59.
    pub minute: u8,
    /// The second, 0 to 59.
    pub second: u8,
    /// The fraction of the second, in attoseconds.
    pub attos: u64,
    /// `TC` or `TCn`.
    pub base: YearBase,
}

impl TerranDate {
    /// A date on a whole second, with its fields in their ranges; whether a
    /// minimonth date exists depends on the year's leap duration and is
    /// checked by [`to_tai`].
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] for a field outside its range or a year
    /// outside [`MIN_YEAR`]..=[`MAX_YEAR`].
    pub const fn new(
        year: i64,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        base: YearBase,
    ) -> TimeResult<Self> {
        if year < MIN_YEAR
            || year > MAX_YEAR
            || month > MINIMONTH
            || (month < MINIMONTH && day > 27)
            || hour > 23
            || minute > 59
            || second > 59
        {
            return Err(TimeError::OutOfRange);
        }
        Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            attos: 0,
            base,
        })
    }

    /// Seconds from the start of the year, before the fraction.
    const fn seconds_into_year(self) -> i128 {
        DAY * (28 * self.month as i128 + self.day as i128)
            + 3_600 * self.hour as i128
            + 60 * self.minute as i128
            + self.second as i128
    }
}

/// The TAI instant of a Terran Computational date.
///
/// # Errors
///
/// [`TimeError::OutOfRange`] for a field out of range, including a
/// minimonth date past the year's leap duration; and
/// [`TimeError::AfterModelEnd`] when the date's base lies past the
/// leap-second table, or under `TC` when the date itself does.
pub fn to_tai(date: TerranDate) -> TimeResult<Instant<Tai>> {
    let checked = TerranDate::new(
        date.year,
        date.month,
        date.day,
        date.hour,
        date.minute,
        date.second,
        date.base,
    )?;
    if date.attos >= hc_core::ATTOS_PER_SEC {
        return Err(TimeError::OutOfRange);
    }
    check_base(date.base)?;
    let into = checked.seconds_into_year();
    let months = DAY * MONTHS_DAYS as i128;
    if into >= months + minimonth_seconds(date.year, date.base) {
        return Err(TimeError::OutOfRange);
    }
    let seconds = year_start_seconds(date.year, date.base) + into;
    if date.base == YearBase::Full && seconds > VALID_UNTIL {
        return Err(TimeError::AfterModelEnd);
    }
    let since = Duration::new(EPOCH_TAI_SECONDS + seconds, date.attos)?;
    Ok(Instant::from_epoch(since))
}

/// The Terran Computational date of a TAI instant under a year base.
///
/// # Errors
///
/// [`TimeError::AfterModelEnd`] when the base lies past the leap-second
/// table, or under `TC` when the instant does; [`TimeError::OutOfRange`]
/// outside [`MIN_YEAR`]..=[`MAX_YEAR`].
pub fn from_tai(instant: Instant<Tai>, base: YearBase) -> TimeResult<TerranDate> {
    check_base(base)?;
    let elapsed = timestamp(instant)?;
    let seconds = elapsed.whole_seconds();
    if base == YearBase::Full && seconds > VALID_UNTIL {
        return Err(TimeError::AfterModelEnd);
    }
    // 128 years are 46 751 days before the leap seconds, and the estimate
    // is within a year; the loops close the gap.
    let estimate = (seconds * 128).div_euclid(DAY * 46_751);
    if estimate < i128::from(MIN_YEAR) || estimate > i128::from(MAX_YEAR) {
        return Err(TimeError::OutOfRange);
    }
    let mut year = estimate as i64;
    while year_start_seconds(year, base) > seconds {
        year -= 1;
    }
    while year_start_seconds(year + 1, base) <= seconds {
        year += 1;
    }
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(TimeError::OutOfRange);
    }
    let into = seconds - year_start_seconds(year, base);
    let (month, within) = if into < DAY * MONTHS_DAYS as i128 {
        (into / (28 * DAY), into % (28 * DAY))
    } else {
        (i128::from(MINIMONTH), into - DAY * MONTHS_DAYS as i128)
    };
    let (day, within) = (within / DAY, within % DAY);
    Ok(TerranDate {
        year,
        month: month as u8,
        day: day as u8,
        hour: (within / 3_600) as u8,
        minute: (within % 3_600 / 60) as u8,
        second: (within % 60) as u8,
        attos: elapsed.subsec_attos(),
        base,
    })
}

/// The dotted form the site writes, `42.13.1,0.0.0 TC`, with the fraction
/// of the second where there is one and the year base in the designator.
impl fmt::Display for TerranDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{},{}.{}.{}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )?;
        if self.attos != 0 {
            let mut digits = self.attos;
            let mut width = 18;
            while digits.is_multiple_of(10) {
                digits /= 10;
                width -= 1;
            }
            write!(f, ".{digits:0width$}")?;
        }
        match self.base {
            YearBase::Full => f.write_str(" TC"),
            YearBase::Base(year) => write!(f, " TC{year}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian;
    use hc_core::unix::{LeapPolicy, UtcInstant, tai_from_utc, utc_from_tai};

    /// The Unix second of a UTC date and time.
    fn unix(year: i64, month: u8, day: u8, hour: i64, minute: i64, second: i64) -> i64 {
        let days = gregorian::to_fixed(year, month, day).unwrap().0
            - gregorian::to_fixed(1970, 1, 1).unwrap().0;
        86_400 * days + 3_600 * hour + 60 * minute + second
    }

    fn tai(year: i64, month: u8, day: u8, hour: i64, minute: i64, second: i64) -> Instant<Tai> {
        let utc = UtcInstant {
            unix_seconds: unix(year, month, day, hour, minute, second),
            leap_second: false,
            subsec_attos: 0,
        };
        tai_from_utc(utc, LeapPolicy::Strict).unwrap()
    }

    fn date(year: i64, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> TerranDate {
        TerranDate::new(year, month, day, hour, minute, second, YearBase::Full).unwrap()
    }

    #[test]
    fn the_epoch_is_the_sites() {
        // "221788790 ((365*5 + 366*2 + 10)*86400 - 10) SI seconds ... before
        // 1977-01-01 00:00:00 TAI".
        assert_eq!((365 * 5 + 366 * 2 + 10) * 86_400 - 10, 221_788_790);
        let tai_1977 = Duration::from_secs(i128::from(
            86_400
                * (gregorian::to_fixed(1977, 1, 1).unwrap().0
                    - gregorian::to_fixed(1970, 1, 1).unwrap().0),
        ));
        assert_eq!(
            EPOCH.since_epoch(),
            tai_1977
                .checked_sub(Duration::from_secs(221_788_790))
                .unwrap()
        );
        // "TAI64 always ahead of TC by 2^62 - 863990 seconds".
        assert_eq!(hc_core::tai64::label(EPOCH), Ok((1 << 62) - 863_990));
        assert_eq!(from_tai(EPOCH, YearBase::Full), Ok(date(0, 0, 0, 0, 0, 0)));
    }

    /// The site's two lists: the 25 UTC leap seconds and the TC dates it
    /// gives for them, in the TC years it lists.
    #[test]
    fn the_utc_leap_seconds_fall_where_the_site_puts_them() {
        type Ymd = (i64, u8, u8);
        const SITE: [(Ymd, Ymd); 25] = [
            ((1972, 6, 30), (2, 6, 24)),
            ((1972, 12, 31), (3, 0, 11)),
            ((1973, 12, 31), (4, 0, 11)),
            ((1974, 12, 31), (5, 0, 10)),
            ((1975, 12, 31), (6, 0, 10)),
            ((1976, 12, 31), (7, 0, 11)),
            ((1977, 12, 31), (8, 0, 11)),
            ((1978, 12, 31), (9, 0, 10)),
            ((1979, 12, 31), (10, 0, 10)),
            ((1981, 6, 30), (11, 6, 24)),
            ((1982, 6, 30), (12, 6, 24)),
            ((1983, 6, 30), (13, 6, 23)),
            ((1985, 6, 30), (15, 6, 24)),
            ((1987, 12, 31), (18, 0, 10)),
            ((1989, 12, 31), (20, 0, 11)),
            ((1990, 12, 31), (21, 0, 10)),
            ((1992, 6, 30), (22, 6, 24)),
            ((1993, 6, 30), (23, 6, 24)),
            ((1994, 6, 30), (24, 6, 24)),
            ((1995, 12, 31), (26, 0, 10)),
            ((1997, 6, 30), (27, 6, 24)),
            ((1998, 12, 31), (29, 0, 10)),
            ((2005, 12, 31), (36, 0, 11)),
            ((2008, 12, 31), (39, 0, 11)),
            ((2012, 6, 30), (42, 6, 24)),
        ];
        for (k, ((year, month, day), (tc_year, tc_month, tc_day))) in SITE.iter().enumerate() {
            // 23:59:60 UTC on that day: the leap second before the next
            // day's first Unix second.
            let leap = UtcInstant {
                unix_seconds: unix(*year, *month, *day, 24, 0, 0),
                leap_second: true,
                subsec_attos: 0,
            };
            let instant = tai_from_utc(leap, LeapPolicy::Strict).unwrap();
            assert_eq!(
                from_tai(instant, YearBase::Full),
                Ok(date(*tc_year, *tc_month, *tc_day, 0, 0, 0)),
                "{year}-{month}-{day}"
            );
            assert_eq!(LEAP_SECOND_YEARS[k], *tc_year, "{year}-{month}-{day}");
        }
        // The two since, which the site's tables predate.
        assert_eq!(LEAP_SECOND_YEARS[25..], [45, 47]);
    }

    /// The site's "Terran Computational Leap Seconds": each year's leap
    /// seconds at the end of its minimonth, after the leap days, and the
    /// UTC instant it gives for each.
    #[test]
    fn the_minimonth_leap_seconds_are_the_sites() {
        const SITE: [(i64, u8, (i64, u8, u8)); 25] = [
            (2, 1, (1972, 12, 20)),
            (3, 1, (1973, 12, 20)),
            (4, 2, (1974, 12, 21)),
            (5, 1, (1975, 12, 21)),
            (6, 1, (1976, 12, 20)),
            (7, 1, (1977, 12, 20)),
            (8, 2, (1978, 12, 21)),
            (9, 1, (1979, 12, 21)),
            (10, 1, (1980, 12, 20)),
            (11, 1, (1981, 12, 20)),
            (12, 2, (1982, 12, 21)),
            (13, 1, (1983, 12, 21)),
            (15, 1, (1985, 12, 20)),
            (18, 1, (1988, 12, 20)),
            (20, 2, (1990, 12, 21)),
            (21, 1, (1991, 12, 21)),
            (22, 1, (1992, 12, 20)),
            (23, 1, (1993, 12, 20)),
            (24, 2, (1994, 12, 21)),
            (26, 1, (1996, 12, 20)),
            (27, 1, (1997, 12, 20)),
            (29, 1, (1999, 12, 21)),
            (36, 2, (2006, 12, 21)),
            (39, 1, (2009, 12, 20)),
            (42, 1, (2012, 12, 20)),
        ];
        for (year, day, (utc_year, utc_month, utc_day)) in SITE {
            assert_eq!(leap_days(year), day, "{year}");
            assert_eq!(leap_seconds(year, YearBase::Full), 1, "{year}");
            let instant = to_tai(date(year, MINIMONTH, day, 0, 0, 0)).unwrap();
            let utc = utc_from_tai(instant, LeapPolicy::Strict).unwrap();
            assert_eq!(
                utc.unix_seconds,
                unix(utc_year, utc_month, utc_day, 23, 59, 59),
                "{year}"
            );
            // It is the minimonth's last second.
            assert_eq!(
                to_tai(date(year, MINIMONTH, day, 0, 0, 1)),
                Err(TimeError::OutOfRange)
            );
            assert_eq!(
                from_tai(
                    instant.checked_add(Duration::SECOND).unwrap(),
                    YearBase::Full
                ),
                Ok(date(year + 1, 0, 0, 0, 0, 0))
            );
        }
    }

    /// The site's table of solstices and equinoxes, 2010 to 2020.
    ///
    /// Forty of the 44 agree to the second. The other four are the ones in
    /// TC year 45 after the leap second of 30 June 2015 and in TC year 47
    /// after that of 31 December 2016, the two the site's leap-second
    /// tables stop short of: its conversions count neither, and the rule
    /// puts each in its year, so here those instants are a second later
    /// into the year, as `42.9.23,14.49.1` is after the leap second of
    /// 2012, which the site does count.
    #[test]
    fn the_seasons_table() {
        type Utc = (i64, u8, u8, i64, i64);
        type Tc = (i64, u8, u8, u8, u8, u8);
        const SITE: [(Utc, Tc); 44] = [
            ((2010, 12, 21, 23, 38), (40, 13, 1, 23, 38, 0)),
            ((2011, 12, 22, 5, 30), (42, 0, 0, 5, 30, 0)),
            ((2012, 12, 21, 11, 12), (43, 0, 0, 11, 12, 0)),
            ((2013, 12, 21, 17, 11), (44, 0, 0, 17, 11, 0)),
            ((2014, 12, 21, 23, 3), (44, 13, 1, 23, 3, 0)),
            ((2015, 12, 22, 4, 48), (46, 0, 0, 4, 48, 0)),
            ((2016, 12, 21, 10, 44), (47, 0, 0, 10, 44, 0)),
            ((2017, 12, 21, 16, 28), (48, 0, 0, 16, 28, 0)),
            ((2018, 12, 21, 22, 23), (48, 13, 1, 22, 23, 0)),
            ((2019, 12, 22, 4, 19), (50, 0, 0, 4, 19, 0)),
            ((2020, 12, 21, 10, 2), (51, 0, 0, 10, 2, 0)),
            ((2010, 3, 20, 17, 32), (40, 3, 5, 17, 32, 0)),
            ((2011, 3, 20, 23, 21), (41, 3, 4, 23, 21, 0)),
            ((2012, 3, 20, 5, 14), (42, 3, 5, 5, 14, 0)),
            ((2013, 3, 20, 11, 2), (43, 3, 5, 11, 2, 0)),
            ((2014, 3, 20, 16, 57), (44, 3, 5, 16, 57, 0)),
            ((2015, 3, 20, 22, 45), (45, 3, 4, 22, 45, 0)),
            ((2016, 3, 20, 4, 30), (46, 3, 5, 4, 30, 0)),
            ((2017, 3, 20, 10, 29), (47, 3, 5, 10, 29, 0)),
            ((2018, 3, 20, 16, 15), (48, 3, 5, 16, 15, 0)),
            ((2019, 3, 20, 21, 58), (49, 3, 4, 21, 58, 0)),
            ((2020, 3, 20, 3, 50), (50, 3, 5, 3, 50, 0)),
            ((2010, 6, 21, 11, 28), (40, 6, 14, 11, 28, 0)),
            ((2011, 6, 21, 17, 16), (41, 6, 13, 17, 16, 0)),
            ((2012, 6, 20, 23, 9), (42, 6, 13, 23, 9, 0)),
            ((2013, 6, 21, 5, 4), (43, 6, 14, 5, 4, 0)),
            ((2014, 6, 21, 10, 51), (44, 6, 14, 10, 51, 0)),
            ((2015, 6, 21, 16, 38), (45, 6, 13, 16, 38, 0)),
            ((2016, 6, 20, 22, 34), (46, 6, 13, 22, 34, 0)),
            ((2017, 6, 21, 4, 24), (47, 6, 14, 4, 24, 0)),
            ((2018, 6, 21, 10, 7), (48, 6, 14, 10, 7, 0)),
            ((2019, 6, 21, 15, 54), (49, 6, 13, 15, 54, 0)),
            ((2020, 6, 20, 21, 44), (50, 6, 13, 21, 44, 0)),
            ((2010, 9, 23, 3, 9), (40, 9, 24, 3, 9, 0)),
            ((2011, 9, 23, 9, 5), (41, 9, 23, 9, 5, 0)),
            ((2012, 9, 22, 14, 49), (42, 9, 23, 14, 49, 1)),
            ((2013, 9, 22, 20, 44), (43, 9, 23, 20, 44, 0)),
            ((2014, 9, 23, 2, 29), (44, 9, 24, 2, 29, 0)),
            ((2015, 9, 23, 8, 21), (45, 9, 23, 8, 21, 0)),
            ((2016, 9, 22, 14, 21), (46, 9, 23, 14, 21, 0)),
            ((2017, 9, 22, 20, 2), (47, 9, 23, 20, 2, 0)),
            ((2018, 9, 23, 1, 54), (48, 9, 24, 1, 54, 0)),
            ((2019, 9, 23, 7, 50), (49, 9, 23, 7, 50, 0)),
            ((2020, 9, 22, 13, 31), (50, 9, 23, 13, 31, 0)),
        ];
        const AFTER_UNLISTED_LEAP_SECOND: [(i64, u8, u8); 4] =
            [(2015, 9, 23), (2017, 3, 20), (2017, 6, 21), (2017, 9, 22)];
        let mut exact = 0;
        for ((year, month, day, hour, minute), expected) in SITE {
            let found = from_tai(tai(year, month, day, hour, minute, 0), YearBase::Full).unwrap();
            let fields = (
                found.year,
                found.month,
                found.day,
                found.hour,
                found.minute,
                found.second,
            );
            if AFTER_UNLISTED_LEAP_SECOND.contains(&(year, month, day)) {
                let (y, m, d, h, mi, second) = expected;
                assert_eq!(fields, (y, m, d, h, mi, second + 1), "{year}-{month}-{day}");
            } else {
                assert_eq!(fields, expected, "{year}-{month}-{day} {hour}:{minute}");
                exact += 1;
            }
        }
        assert_eq!(exact, 40);
    }

    /// The worked example of the system document, under `TC` and `TC0`.
    #[test]
    fn the_worked_example() {
        let solstice = tai(2011, 12, 22, 5, 30, 0);
        assert_eq!(timestamp(solstice), Ok(Duration::from_secs(1_325_395_824)));
        let full = from_tai(solstice, YearBase::Full).unwrap();
        assert_eq!(full.to_string(), "42.0.0,5.30.0 TC");
        let zero = from_tai(solstice, YearBase::Base(0)).unwrap();
        assert_eq!(zero.to_string(), "42.0.0,5.30.24 TC0");
        assert_eq!(to_tai(zero), Ok(solstice));
    }

    /// "44.6.14TC = 44TC+2Q = 44TC+26W = 44TC+182D = ... = TC+1404172825",
    /// and "44TC+39W is the equivalent of 44.9.21TC".
    #[test]
    fn the_datemod_examples() {
        let instant = to_tai(date(44, 6, 14, 0, 0, 0)).unwrap();
        assert_eq!(timestamp(instant), Ok(Duration::from_secs(1_404_172_825)));
        let start = year_start(44, YearBase::Full).unwrap();
        assert_eq!(start.checked_add(Duration::from_days(182)), Ok(instant));
        assert_eq!(
            start.checked_add(Duration::from_weeks(39)),
            to_tai(date(44, 9, 21, 0, 0, 0))
        );
    }

    #[test]
    fn the_leap_days() {
        for (year, days) in [
            (0, 1),
            (1, 1),
            (4, 2),
            (40, 2),
            (128, 1),
            (256, 1),
            (-4, 2),
            (-128, 1),
            (-1, 1),
        ] {
            assert_eq!(leap_days(year), days, "{year}");
        }
        for year in -1_000..1_000 {
            assert_eq!(
                days_before(year + 1) - days_before(year),
                MONTHS_DAYS + i64::from(leap_days(year)),
                "{year}"
            );
        }
        // 128 years are 46 751 days.
        assert_eq!(days_before(128), 46_751);
    }

    #[test]
    fn every_instant_round_trips() {
        let start = EPOCH_TAI_SECONDS - 5 * 365 * 86_400;
        let end = VALID_UNTIL + EPOCH_TAI_SECONDS;
        for base in [YearBase::Full, YearBase::Base(0), YearBase::Base(44)] {
            let mut seconds = start;
            while seconds <= end {
                let instant =
                    Instant::from_epoch(Duration::new(seconds, 250_000_000_000_000_000).unwrap());
                let found = from_tai(instant, base).unwrap();
                assert_eq!(to_tai(found), Ok(instant), "{base:?} {found}");
                seconds += 86_399 * 7 + 13;
            }
        }
        // Across each leap second, second by second.
        for k in 0..STEPS {
            let at = step(k).0 + EPOCH_TAI_SECONDS;
            for seconds in at - 3..at + 3 {
                let instant = Instant::from_epoch(Duration::from_secs(seconds));
                for base in [YearBase::Full, YearBase::Base(0)] {
                    assert_eq!(to_tai(from_tai(instant, base).unwrap()), Ok(instant));
                }
            }
        }
        // Far from the table, under a year base.
        let far =
            to_tai(TerranDate::new(100_000, 5, 5, 5, 5, 5, YearBase::Base(44)).unwrap()).unwrap();
        assert_eq!(from_tai(far, YearBase::Base(44)).unwrap().year, 100_000);
        assert_eq!(
            from_tai(far, YearBase::Base(0)).map(|date| date.year),
            Ok(100_000)
        );
    }

    #[test]
    fn what_cannot_be_known_is_refused() {
        // Under TC, past the announced validity of the table.
        let future =
            to_tai(TerranDate::new(100, 0, 0, 0, 0, 0, YearBase::Base(50)).unwrap()).unwrap();
        assert_eq!(
            from_tai(future, YearBase::Full),
            Err(TimeError::AfterModelEnd)
        );
        assert_eq!(
            to_tai(date(100, 0, 0, 0, 0, 0)),
            Err(TimeError::AfterModelEnd)
        );
        // A base whose year the table does not reach.
        assert_eq!(
            from_tai(EPOCH, YearBase::Base(1_000)),
            Err(TimeError::AfterModelEnd)
        );
        // Fields out of range, and a minimonth past its leap duration.
        assert_eq!(
            TerranDate::new(40, 14, 0, 0, 0, 0, YearBase::Full),
            Err(TimeError::OutOfRange)
        );
        assert_eq!(
            TerranDate::new(40, 12, 28, 0, 0, 0, YearBase::Full),
            Err(TimeError::OutOfRange)
        );
        assert_eq!(
            to_tai(date(41, MINIMONTH, 1, 0, 0, 0)),
            Err(TimeError::OutOfRange)
        );
        assert!(to_tai(date(40, MINIMONTH, 1, 23, 59, 59)).is_ok());
        assert_eq!(
            to_tai(date(40, MINIMONTH, 2, 0, 0, 0)),
            Err(TimeError::OutOfRange)
        );
        let mut fraction = date(40, 0, 0, 0, 0, 0);
        fraction.attos = hc_core::ATTOS_PER_SEC;
        assert_eq!(to_tai(fraction), Err(TimeError::OutOfRange));
        fraction.attos = 500_000_000_000_000_000;
        assert_eq!(fraction.to_string(), "40.0.0,0.0.0.5 TC");
    }
}
