//! The Revised Julian calendar of Milutin Milanković, 1923.
//!
//! The Pan-Orthodox Congress of Constantinople adopted Milanković's leap
//! rule in 1923 and proposed that the coming 1 October Julian be called
//! 14 October. The churches took the calendar up one at a time, for their
//! fixed feasts, and this library dates its use from the first adoption
//! that lasted — Constantinople and Greece on 10/23 March 1924, Cyprus
//! with them ([`ADOPTION`]) — not from the proposal, nor from the Russian
//! Church's acceptance of 15 October 1923, which Patriarch Tikhon reversed
//! twenty-four days later. Then Romania on 1/14 October 1924,
//! Alexandria and Antioch in 1928, Albania in 1937, Bulgaria on
//! 20 December 1968, and the Orthodox Church of Ukraine on 1 September
//! 2023; the Polish Orthodox Church, which adopted it on 1/14 October 1924
//! though few of its parishes changed, returned to the Julian calendar on
//! 15 June 2014
//! (`wikipedia-revised-julian-calendar`).
//!
//! # The rule
//!
//! Leap years are the ones divisible by four, except that a century year is
//! leap only when it leaves a remainder of 200 or 600 on division by 900.
//! So 2000 and 2400 are leap, and 1900, 2100, 2200, 2300, 2500 and 2800 are
//! not.
//!
//! That gives 218 leap years in 900, a mean year of 365.2422̄ days. Milanković
//! chose it because it was within two seconds of the mean tropical year as
//! then measured — against the Gregorian 365.2425, which is about
//! twenty-six seconds long.
//!
//! # Where it agrees with the Gregorian calendar
//!
//! From 1 March 1600 to 28 February 2800, exactly. The first disagreement
//! is 2800, a Gregorian leap year and a Revised Julian common one; the two
//! have then drifted a day apart and stay so.
//!
//! The disagreement does not last, though, and that is easy to get wrong:
//! the Gregorian calendar takes its leap day in 2800 and this one takes its
//! in 2900, so from 1 March 2900 the two agree again. They oscillate a day
//! apart and back, which is how a 365.2422-day year and a 365.2425-day one
//! stay together for millennia.
//!
//! Going backwards, 1600 is the boundary for the same reason in reverse:
//! 1600 mod 900 is 700, so this calendar has no 29 February 1600 and the
//! Gregorian calendar does. Ask for it and you get
//! [`CalendarError::DayOutOfRange`], which is the honest answer and not the
//! Gregorian one.
//!
//! # What this is not
//!
//! It is not the Orthodox liturgical calendar. The churches that adopted it
//! adopted it for *fixed* feasts only and kept computing Pascha on the
//! Julian paschalion, which is why Orthodox Easter still drifts from the
//! Western date. A calendar cannot express that split; `hc-holiday`'s
//! computus can, and does.
//!
//! **Sources:** Wikipedia, "Revised Julian calendar", retrieved 2026-09-26
//! (`wikipedia-revised-julian-calendar`), for the Congress's decision and
//! the dated list of adopting churches, which it cites to Clogg (2002) and
//! news reports, not read here; M. Milankovitch, *Astronomische
//! Nachrichten* 220 (1924) 379–384 (`milankovitch1924`), Milanković's own
//! account of the 900-year rule, not read here. The rule itself is stated
//! above and tested below.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Usage,
    YearKind,
};

use crate::{common, gregorian};

/// The earliest year this implementation converts.
pub use crate::gregorian::MIN_YEAR;

/// The latest year this implementation converts.
pub use crate::gregorian::MAX_YEAR;

/// Years in the leap cycle.
pub const CYCLE_YEARS: i64 = 900;

/// Leap years per cycle, giving a mean year of 365.2422̄ days.
pub const LEAPS_PER_CYCLE: i64 = 218;

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "In use from 10 March 1924 Julian, called 23 March, when the \
    Ecumenical Patriarchate and the Church of Greece adopted it, the first churches to do so; \
    the Pan-Orthodox Congress of Constantinople had proposed 1/14 October 1923 \
    [wikipedia-revised-julian-calendar]";

/// The first day any church kept the calendar: 10 March 1924 in the Julian
/// calendar, which the Ecumenical Patriarchate and the Church of Greece
/// called 23 March 1924, as this calendar and the Gregorian one both do.
///
/// The Pan-Orthodox Congress had proposed 1 October 1923 Julian, called
/// 14 October, which [`PROPOSED`] records; the Russian Church's acceptance
/// of the next day lasted twenty-four days and is not counted as the start
/// (`wikipedia-revised-julian-calendar`).
pub const ADOPTION: Rd = match gregorian::to_fixed(1924, 3, 23) {
    Ok(rd) => rd,
    // Unreachable: the date is a valid Gregorian one, and a `const` cannot
    // unwrap.
    Err(_) => Rd(0),
};

/// The day the Pan-Orthodox Congress of Constantinople proposed to put the
/// calendar into use: 1 October 1923 Julian, to be called 14 October
/// (`wikipedia-revised-julian-calendar`).
pub const PROPOSED: Rd = match gregorian::to_fixed(1923, 10, 14) {
    Ok(rd) => rd,
    // Unreachable: the date is a valid Gregorian one, and a `const` cannot
    // unwrap.
    Err(_) => Rd(0),
};

/// Whether `year` is a leap year.
///
/// Divisible by four, except that a century year must leave 200 or 600 on
/// division by 900.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    if year.rem_euclid(4) != 0 {
        return false;
    }
    if year.rem_euclid(100) != 0 {
        return true;
    }
    let remainder = year.rem_euclid(900);
    remainder == 200 || remainder == 600
}

/// The number of leap years from year 1 up to and including `year`.
///
/// Counting rather than looping: the quarters, less the centuries, plus the
/// centuries the 900-year rule gives back. A century year `100n` is leap
/// exactly when `n` leaves 2 or 6 on division by 9.
const fn leap_years_through(year: i64) -> i64 {
    let quarters = year.div_euclid(4);
    let centuries = year.div_euclid(100);
    quarters - centuries + restored_centuries(centuries)
}

/// How many of the first `centuries` century years the 900-rule restores.
const fn restored_centuries(centuries: i64) -> i64 {
    // n in 1..=centuries with n mod 9 == 2, plus those with n mod 9 == 6.
    count_congruent(centuries, 2) + count_congruent(centuries, 6)
}

/// How many `n` in `1..=limit` satisfy `n mod 9 == residue`.
const fn count_congruent(limit: i64, residue: i64) -> i64 {
    if limit < residue {
        return 0;
    }
    (limit - residue).div_euclid(9) + 1
}

/// The number of days in `month` of `year`, or `None` outside `1..=12`.
#[must_use]
pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 => Some(if is_leap_year(year) { 29 } else { 28 }),
        _ => None,
    }
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// The fixed day of 1 January of `year`, without validation.
const fn new_year_raw(year: i64) -> i64 {
    let previous = year - 1;
    365 * previous + leap_years_through(previous) + 1
}

/// The fixed day of 1 January of `year`.
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

/// Days elapsed in `year` before the first of `month`.
const fn days_before_month(year: i64, month: u8) -> i64 {
    let mut total = 0;
    let mut index = 1u8;
    while index < month {
        total += match days_in_month(year, index) {
            Some(length) => length as i64,
            None => 0,
        };
        index += 1;
    }
    total
}

/// The fixed day of a Revised Julian date.
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
        Ok(()) => Ok(Rd(new_year_raw(year)
            + days_before_month(year, month)
            + day as i64
            - 1)),
    }
}

/// The Revised Julian year, month and day of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] or
/// [`CalendarError::AfterSupportedRange`] outside the supported range.
pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    if rd.0 < EARLIEST.0 {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd.0 > LATEST.0 {
        return Err(CalendarError::AfterSupportedRange);
    }
    // The mean year as the cycle's own ratio lands within a year; the two
    // corrections close the gap.
    let mut year = rd.0.div_euclid(365) + 1;
    while new_year_raw(year) > rd.0 {
        year -= 1;
    }
    while new_year_raw(year + 1) <= rd.0 {
        year += 1;
    }
    let mut day_of_year = rd.0 - new_year_raw(year);
    let mut month = 1u8;
    while month < 12 {
        let length = match days_in_month(year, month) {
            Some(length) => length as i64,
            None => 0,
        };
        if day_of_year < length {
            break;
        }
        day_of_year -= length;
        month += 1;
    }
    Ok((year, month, (day_of_year + 1) as u8))
}

/// The earliest fixed day this implementation converts.
pub const EARLIEST: Rd = Rd(new_year_raw(MIN_YEAR));

/// The latest fixed day this implementation converts.
pub const LATEST: Rd = Rd(new_year_raw(MAX_YEAR + 1) - 1);

/// A Revised Julian date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RevisedJulianDate {
    /// The year.
    pub year: i64,
    /// The month, 1 through 12.
    pub month: u8,
    /// The day of the month.
    pub day: u8,
}

impl RevisedJulianDate {
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
}

/// The Revised Julian calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RevisedJulianCalendar;

impl Calendar for RevisedJulianCalendar {
    type Date = RevisedJulianDate;

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("revised-julian"),
            english_name: "Revised Julian (Milanković)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &[],
        }
    }

    /// In use since the first lasting adoption in March 1924, and
    /// proleptic before — which matters, because the proleptic form disagrees with the
    /// Gregorian calendar before 1 March 1600 and no church ever used it
    /// there.
    fn usage(&self) -> Usage {
        Usage::since(ADOPTION, USAGE_SOURCE)
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = from_fixed(rd)?;
        Ok(RevisedJulianDate { year, month, day })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields::ymd(date.year, date.month, date.day))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        RevisedJulianDate::new(fields.year, month.ordinal, fields.require_day()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_century_rule_is_the_one_milankovic_published() {
        // Leap: 2000 (200), 2400 (600). Common: every other century in the
        // cycle, including 1600 and 2800.
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2400));
        for year in [
            1600, 1700, 1800, 1900, 2100, 2200, 2300, 2500, 2600, 2700, 2800,
        ] {
            assert!(!is_leap_year(year), "{year} should be common");
        }
        // And the ordinary rule still holds between them.
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2023));
    }

    #[test]
    fn the_cycle_has_two_hundred_and_eighteen_leap_years() {
        let count = (1..=CYCLE_YEARS).filter(|year| is_leap_year(*year)).count();
        assert_eq!(count as i64, LEAPS_PER_CYCLE);
        // Which is a mean year of 365 + 218/900 = 365.24222… days.
        let mean = 365.0 + LEAPS_PER_CYCLE as f64 / CYCLE_YEARS as f64;
        assert!((mean - 365.242_222).abs() < 1e-6, "{mean}");
    }

    /// The claim the calendar was adopted on: it agrees with the Gregorian
    /// calendar from 1 March 1600 to 28 February 2800, and then it does not.
    #[test]
    fn it_agrees_with_the_gregorian_calendar_over_its_stated_span() {
        let first = gregorian::to_fixed(1600, 3, 1).expect("exists");
        let last = gregorian::to_fixed(2800, 2, 28).expect("exists");
        for rd in (first.0..=last.0).step_by(29) {
            let rd = Rd(rd);
            assert_eq!(
                from_fixed(rd),
                gregorian::from_fixed(rd),
                "{rd} should read the same in both"
            );
        }
        // The endpoints exactly.
        assert_eq!(from_fixed(first), Ok((1600, 3, 1)));
        assert_eq!(from_fixed(last), Ok((2800, 2, 28)));
    }

    /// The first disagreement, and — the part that is easy to get wrong —
    /// what happens after it.
    ///
    /// The two do *not* part company for good in 2800. They part for a
    /// century and then re-align: the Gregorian calendar takes its leap day
    /// in 2800 and this one takes its in 2900, so from 1 March 2900 they
    /// agree again. That oscillation is the whole reason a 365.2422-day
    /// mean year and a 365.2425-day one stay within a day of each other for
    /// millennia.
    #[test]
    fn the_first_disagreement_is_the_twenty_eight_hundredth_leap_day() {
        assert!(gregorian::is_leap_year(2800));
        assert!(!is_leap_year(2800));
        assert_eq!(to_fixed(2800, 2, 29), Err(CalendarError::DayOutOfRange));

        // The day the Gregorian calendar calls 29 February 2800 is already
        // 1 March here.
        let leap_day = gregorian::to_fixed(2800, 2, 29).expect("Gregorian has it");
        assert_eq!(from_fixed(leap_day), Ok((2800, 3, 1)));
        assert_eq!(gregorian::from_fixed(leap_day), Ok((2800, 2, 29)));

        // A day apart for the century that follows.
        let inside = gregorian::to_fixed(2850, 6, 15).expect("exists");
        assert_ne!(from_fixed(inside), gregorian::from_fixed(inside));

        // Then 2900 is leap here and common there, and they re-align.
        assert!(is_leap_year(2900));
        assert!(!gregorian::is_leap_year(2900));
        let after = gregorian::to_fixed(2900, 3, 1).expect("exists");
        assert_eq!(from_fixed(after), gregorian::from_fixed(after));
        let later = gregorian::to_fixed(3000, 1, 1).expect("exists");
        assert_eq!(from_fixed(later), gregorian::from_fixed(later));
    }

    #[test]
    fn it_has_no_leap_day_in_sixteen_hundred_either() {
        // 1600 mod 900 = 700, so this calendar skips a day the Gregorian
        // one keeps — the reason the agreement starts on 1 March and not on
        // 1 January.
        assert!(gregorian::is_leap_year(1600));
        assert!(!is_leap_year(1600));
        assert_eq!(to_fixed(1600, 2, 29), Err(CalendarError::DayOutOfRange));
        // The two agree from 1 March 1600, so counting back one day the
        // Gregorian 29 February is this calendar's 28th.
        let gregorian_leap_day = gregorian::to_fixed(1600, 2, 29).expect("Gregorian has it");
        assert_eq!(from_fixed(gregorian_leap_day), Ok((1600, 2, 28)));
        assert_eq!(
            to_fixed(1600, 3, 1),
            gregorian::to_fixed(1600, 3, 1),
            "the agreement starts on 1 March"
        );
    }

    #[test]
    fn the_first_church_changed_on_the_twenty_third_of_march_1924() {
        // "10/23 March 1924: Constantinople, Cyprus and Greece"
        // (wikipedia-revised-julian-calendar).
        assert_eq!(from_fixed(ADOPTION), Ok((1924, 3, 23)));
        assert_eq!(gregorian::from_fixed(ADOPTION), Ok((1924, 3, 23)));
        assert_eq!(crate::julian::from_fixed(ADOPTION), Ok((1924, 3, 10)));
        // The Congress's proposed day, which no church kept.
        assert_eq!(from_fixed(PROPOSED), Ok((1923, 10, 14)));
        assert_eq!(crate::julian::from_fixed(PROPOSED), Ok((1923, 10, 1)));
        assert!(PROPOSED < ADOPTION);
    }

    #[test]
    fn it_round_trips_every_sampled_day_across_three_cycles() {
        let start = new_year(1).expect("in range");
        let end = new_year(2_700).expect("in range");
        for rd in (start.0..end.0).step_by(11) {
            let rd = Rd(rd);
            let (year, month, day) = from_fixed(rd).expect("in range");
            assert_eq!(to_fixed(year, month, day), Ok(rd), "{rd}");
        }
    }

    #[test]
    fn the_leap_count_agrees_with_counting_them_one_by_one() {
        // `leap_years_through` is a closed form; this is the loop it replaces.
        for year in (0..3_000).step_by(7) {
            let counted = (1..=year)
                .filter(|candidate| is_leap_year(*candidate))
                .count();
            assert_eq!(leap_years_through(year), counted as i64, "through {year}");
        }
    }

    #[test]
    fn a_date_before_the_first_adoption_is_proleptic_and_says_so() {
        use hc_calendar::Standing;
        let calendar = RevisedJulianCalendar;
        assert_eq!(calendar.standing(ADOPTION), Standing::InUse);
        assert_eq!(
            calendar.standing(Rd(ADOPTION.0 - 1)),
            Standing::Proleptic,
            "the first churches adopted it in March 1924, not before"
        );
        assert_eq!(calendar.standing(PROPOSED), Standing::Proleptic);
    }
}
