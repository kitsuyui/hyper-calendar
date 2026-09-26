//! The French Republican calendar under Richards's arithmetic rule.
//!
//! E. G. Richards's conversion algorithms in the *Explanatory Supplement to
//! the Astronomical Almanac* (3rd ed., 2013, chapter 15, `richards2013`,
//! read in the U.S. Naval Observatory's online copy on 2026-09-26) carry the
//! Republican calendar as calendar 4 of Table 15.14, with the parameters
//! *y* = 6504, *j* = 111, *m* = 0, *n* = 13, *r* = 4, *p* = 1461, *q* = 0,
//! *v* = 3, *u* = 1, *s* = 30, *t* = 0, *w* = 0 and, for a calendar that
//! intercalates "with the same frequency as the Gregorian", *A* = 396,
//! *B* = 578 797, *C* = −51, in Algorithms 3 and 4 of §15.11.3. Worked
//! through, those parameters make a year sextile when it is 3 modulo 4,
//! except when it is 99 modulo 100 unless it is 399 modulo 400, from the
//! epoch of §15.9, 22 September 1792, Julian Day Number 2 375 840. Richards
//! states no rule in words: the rule is what his algorithms compute, and
//! the tests here run his two algorithms as printed against this module
//! over its whole range. His own §15.9 says the first leap day was "in
//! year 4 E.R.", which his parameters contradict and history does not bear
//! out; the parameters are what this module follows.
//!
//! The rule puts the sextile day at the end of An III, VII and XI, as the
//! equinox did in the years the calendar was kept, so it gives the fourteen
//! new years of An I to XIV that France kept, where Romme's rule of
//! [`crate::french_republican`] gives An IV, VIII and XII a day early. The
//! two are competing arithmetic rules for one calendar and are two
//! calendars under policy §5. The system document is
//! `docs/systems/equinox-calendars.md` in the repository.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, Rd, YearKind,
};

use crate::common;
use crate::french_republican::{self, EPOCH, FrenchRepublicanDate, date_fields, fields_date};

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts, as for Romme's rule.
pub const MAX_YEAR: i64 = french_republican::MAX_YEAR;

/// Whether `year` is sextile under Richards's parameters.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    year.rem_euclid(4) == 3 && (year.rem_euclid(100) != 99 || year.rem_euclid(400) == 399)
}

/// The number of days in `month` of `year`, or `None` when `month` is not in
/// `1..=13`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    match month {
        1..=12 => Some(30),
        13 => Some(if is_leap_year(year) { 6 } else { 5 }),
        _ => None,
    }
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// The fixed day of a date, without validation.
const fn to_fixed_raw(year: i64, month: u8, day: u8) -> i64 {
    // The sextile years before `year`: those 3 modulo 4, less those 99
    // modulo 100, plus those 399 modulo 400, among 1..year.
    EPOCH.0 - 1 + 365 * (year - 1) + year.div_euclid(4) - year.div_euclid(100)
        + year.div_euclid(400)
        + 30 * (month as i64 - 1)
        + day as i64
}

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = Rd(to_fixed_raw(MIN_YEAR, 1, 1));

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(to_fixed_raw(MAX_YEAR + 1, 1, 1) - 1);

/// The fixed day of a Republican date under Richards's rule.
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

/// The Republican year, month and day of a fixed day under Richards's rule.
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
    // 400 years are 146 097 days, as in the Gregorian calendar; the
    // estimate is within a year, and the comparisons settle it.
    let mut year = ((rd.0 - EPOCH.0) * 400).div_euclid(146_097) + 1;
    if rd.0 < to_fixed_raw(year, 1, 1) {
        year -= 1;
    } else if rd.0 >= to_fixed_raw(year + 1, 1, 1) {
        year += 1;
    }
    let day_of_year = rd.0 - to_fixed_raw(year, 1, 1);
    let month = (day_of_year.div_euclid(30) + 1) as u8;
    let day = (day_of_year.rem_euclid(30) + 1) as u8;
    Ok((year, month, day))
}

/// The French Republican calendar under Richards's rule.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RichardsFrenchRepublicanCalendar;

impl Calendar for RichardsFrenchRepublicanCalendar {
    type Date = FrenchRepublicanDate;

    /// The same twelve years in force as Romme's rule records, 6 October
    /// 1793 to 31 December 1805, in which this rule and the kept calendar
    /// agree day for day.
    fn usage(&self) -> hc_calendar::Usage {
        french_republican::KEPT
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        french_republican::SHAPE
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("french-republican-arithmetic-richards"),
            english_name: "French Republican (Richards's arithmetic)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &["fr"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(FrenchRepublicanDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<hc_calendar::DateFields> {
        date_fields(date)
    }

    fn from_fields(&self, fields: &hc_calendar::DateFields) -> CalendarResult<Self::Date> {
        let (year, month, day) = fields_date(fields)?;
        to_fixed(year, month, day)?;
        Ok(FrenchRepublicanDate { year, month, day })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian;

    /// Richards's Algorithm 3 as printed, with Table 15.14's row 4: a date
    /// to a Julian Day Number. Every quantity is positive in range, so the
    /// book's truncating division is Rust's.
    fn richards_algorithm_3(year: i64, month: i64, day: i64) -> i64 {
        let (y, j, m, n, r, p, q, u, s, t) = (6_504, 111, 0, 13, 4, 1_461, 0, 1, 30, 0);
        let (a, c) = (396, -51);
        let h = month - m;
        let g = year + y - (n - h) / n;
        let f = (h - 1 + n) % n;
        let e = (p * g + q) / r + day - 1 - j;
        let jdn = e + (s * f + t) / u;
        jdn - (3 * ((g + a) / 100)) / 4 - c
    }

    /// Richards's Algorithm 4 as printed, with the Gregorian-type step 1a.
    fn richards_algorithm_4(jdn: i64) -> (i64, i64, i64) {
        let (y, j, m, n, r, p, v, u, s, w) = (6_504, 111, 0, 13, 4, 1_461, 3, 1, 30, 0);
        let (b, c) = (578_797, -51);
        let mut f = jdn + j;
        f = f + (((4 * jdn + b) / 146_097) * 3) / 4 + c;
        let e = r * f + v;
        let g = (e % p) / r;
        let h = u * g + w;
        let day = (h % s) / u + 1;
        let month = (h / s + m) % n + 1;
        let year = e / p - y + (n + m - month) / n;
        (year, month, day)
    }

    #[test]
    fn the_epoch_is_julian_day_2375840() {
        assert_eq!(EPOCH.to_julian_day_number(), 2_375_840);
        assert_eq!(richards_algorithm_3(1, 1, 1), 2_375_840);
        assert_eq!(to_fixed(1, 1, 1), Ok(EPOCH));
    }

    #[test]
    fn the_twelfth_year_begins_on_the_twenty_fourth_of_september_1803() {
        assert_eq!(to_fixed(12, 1, 1), gregorian::to_fixed(1803, 9, 24));
    }

    #[test]
    fn the_fourteen_years_france_kept_begin_where_the_record_says() {
        // 1 Vendémiaire An I–XIV: Wikipedia, "French Republican calendar",
        // retrieved 2026-09-22, as `french-republican-equinox` tests them.
        let record = [
            (1, (1792, 9, 22)),
            (2, (1793, 9, 22)),
            (3, (1794, 9, 22)),
            (4, (1795, 9, 23)),
            (5, (1796, 9, 22)),
            (6, (1797, 9, 22)),
            (7, (1798, 9, 22)),
            (8, (1799, 9, 23)),
            (9, (1800, 9, 23)),
            (10, (1801, 9, 23)),
            (11, (1802, 9, 23)),
            (12, (1803, 9, 24)),
            (13, (1804, 9, 23)),
            (14, (1805, 9, 23)),
        ];
        for (year, (y, m, d)) in record {
            assert_eq!(
                to_fixed(year, 1, 1),
                gregorian::to_fixed(y, m, d),
                "An {year}"
            );
            assert_eq!(is_leap_year(year), matches!(year, 3 | 7 | 11), "An {year}");
        }
    }

    #[test]
    fn the_century_rule_is_ninety_nine_and_three_hundred_ninety_nine() {
        assert!(is_leap_year(3) && is_leap_year(95) && is_leap_year(103));
        assert!(!is_leap_year(99) && !is_leap_year(199) && !is_leap_year(299));
        assert!(is_leap_year(399) && is_leap_year(799));
        assert!(!is_leap_year(4) && !is_leap_year(400));
        assert_eq!(days_in_month(399, 13), Some(6));
        assert_eq!(to_fixed(99, 13, 6), Err(CalendarError::DayOutOfRange));
    }

    #[test]
    fn this_module_is_richards_algorithms_over_its_whole_range() {
        let step = if cfg!(debug_assertions) { 7 } else { 1 };
        for rd in (EARLIEST.0..=LATEST.0).step_by(step) {
            let (year, month, day) = from_fixed(Rd(rd)).unwrap();
            assert_eq!(to_fixed(year, month, day), Ok(Rd(rd)));
            let jdn = Rd(rd).to_julian_day_number();
            assert_eq!(
                richards_algorithm_4(jdn),
                (year, i64::from(month), i64::from(day)),
                "JDN {jdn}"
            );
            assert_eq!(
                richards_algorithm_3(year, i64::from(month), i64::from(day)),
                jdn
            );
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields_and_refuses_outside() {
        let calendar = RichardsFrenchRepublicanCalendar;
        for rd in (EARLIEST.0..=LATEST.0).step_by(881) {
            let date = calendar.from_fixed(Rd(rd)).unwrap();
            let fields = calendar.to_fields(date).unwrap();
            assert_eq!(calendar.from_fields(&fields), Ok(date));
        }
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        // An III's sixth complementary day exists here and not under Romme.
        let sextile = hc_calendar::DateFields::ymd(3, 13, 6);
        assert!(calendar.from_fields(&sextile).is_ok());
        assert!(
            crate::ArithmeticFrenchRepublicanCalendar
                .from_fields(&sextile)
                .is_err()
        );
        assert_eq!(
            calendar.from_fields(&hc_calendar::DateFields::ymd(1, 1, 1).with_era("ad")),
            Err(CalendarError::UnknownEra)
        );
    }
}
