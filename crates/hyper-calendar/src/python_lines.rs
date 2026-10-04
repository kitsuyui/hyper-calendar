//! The tab-separated lines and the numbers the WebAssembly module and the
//! C library write for the functions of Python's `time` and `calendar`
//! modules, written once, over [`crate::civil`]'s [`StructTime`] and
//! [`calendar`], and for the week rules of [`hc_calendar::week`].
//!
//! * `time.gmtime` and `time.localtime` as the nine fields of a
//!   `struct_time`, one a cell: [`gmtime_line`], [`localtime_line`].
//! * `calendar.timegm` and `time.mktime` as a POSIX second: [`timegm`],
//!   [`mktime`].
//! * `calendar.isleap`, `leapdays`, `weekday`, `monthrange` and
//!   `monthcalendar`: [`isleap`], [`leapdays`], [`calendar_weekday`],
//!   [`monthrange_line`], [`monthcalendar_lines`].
//! * The week of the year under a week rule — the first day of the week and
//!   the fewest days a first week holds, which is ISO 8601's Monday and 4,
//!   `%U`'s Sunday and 7, `%W`'s Monday and 7, and every locale's pair:
//!   [`week_of_year_line`].
//!
//! The years are the Gregorian module's, [`gregorian::MIN_YEAR`] to
//! [`gregorian::MAX_YEAR`]; a year outside them is [`Refusal::OutOfRange`],
//! and a date that does not exist [`Refusal::InvalidDate`], as
//! [`hc_calendar::CalendarError`] maps to a refusal.

use alloc::string::String;

use hc_calendar::week::WeekRule;
use hc_calendar::{Rd, Weekday, gregorian};

use crate::boundary::{Answer, Line, Refusal};
use crate::civil::{StructTime, calendar};

/// How many columns a `struct_time` line has: Python's nine fields.
pub const STRUCT_TIME_COLUMNS: usize = 9;

/// How many columns [`monthrange_line`] writes.
pub const MONTHRANGE_COLUMNS: usize = 2;

/// How many columns each line of [`monthcalendar_lines`] writes: the seven
/// days of a week.
pub const MONTHCALENDAR_COLUMNS: usize = 7;

/// How many columns [`week_of_year_line`] writes.
pub const WEEK_OF_YEAR_COLUMNS: usize = 4;

/// A year the Gregorian module converts, or [`Refusal::OutOfRange`].
fn year_in_range(year: i64) -> Answer<i64> {
    if gregorian::year_in_range(year) {
        Ok(year)
    } else {
        Err(Refusal::OutOfRange)
    }
}

/// A month or a day of the month as a `u8`, or [`Refusal::InvalidDate`]
/// for one no month or day is.
fn field(value: u32) -> Answer<u8> {
    u8::try_from(value).map_err(|_| Refusal::InvalidDate)
}

/// A `struct_time` as one line: `tm_year`, `tm_mon`, `tm_mday`, `tm_hour`,
/// `tm_min`, `tm_sec`, `tm_wday` (Monday 0), `tm_yday` (from 1) and
/// `tm_isdst` (0, 1, or −1 for unknown), in Python's order.
fn struct_time_line(fields: StructTime) -> String {
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(fields.tm_year)
        .value(fields.tm_mon)
        .value(fields.tm_mday)
        .value(fields.tm_hour)
        .value(fields.tm_min)
        .value(fields.tm_sec)
        .value(fields.tm_wday)
        .value(fields.tm_yday)
        .value(fields.tm_isdst);
    line.end();
    out
}

/// The line of `hc_gmtime`: Python's `time.gmtime(seconds)`, the UTC
/// reading of a POSIX second as the nine fields of a `struct_time`, the
/// daylight flag 0.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a second outside the Gregorian years.
pub fn gmtime_line(unix_seconds: i64) -> Answer<String> {
    StructTime::gmtime(unix_seconds)
        .map(struct_time_line)
        .map_err(Refusal::from)
}

/// The fields of a reading as a `struct_time`, the weekday and the day of
/// the year left for [`StructTime::timegm`] to ignore, as Python's
/// `calendar.timegm` does.
const fn struct_time_of(fields: [i64; 6]) -> StructTime {
    StructTime {
        tm_year: fields[0],
        tm_mon: fields[1],
        tm_mday: fields[2],
        tm_hour: fields[3],
        tm_min: fields[4],
        tm_sec: fields[5],
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: -1,
    }
}

/// The value of `hc_timegm`: Python's `calendar.timegm`, the POSIX second
/// of a UTC reading given as a year, a month and a day, an hour, a minute
/// and a second. The day, hour, minute and second are not checked and add
/// up, as Python's do, so a day 32 is the first of the next month; the
/// month must be 1 to 12.
///
/// # Errors
///
/// [`Refusal::InvalidDate`] for a month outside 1 to 12,
/// [`Refusal::OutOfRange`] for a year outside the Gregorian years, and
/// [`Refusal::Overflow`] where the sum leaves an `i64`.
pub fn timegm(fields: [i64; 6]) -> Answer<i64> {
    year_in_range(fields[0])?;
    struct_time_of(fields).timegm().map_err(Refusal::from)
}

/// The value of `hc_isleap`: Python's `calendar.isleap(year)`, whether a
/// proleptic Gregorian year is a leap year, for any year.
///
/// # Errors
///
/// None; the signature is the shared one.
pub fn isleap(year: i64) -> Answer<bool> {
    Ok(calendar::is_leap_year(year))
}

/// The value of `hc_leapdays`: Python's `calendar.leapdays(y1, y2)`, the
/// number of leap years from `y1` up to but not including `y2`, counted
/// backwards when `y2` is before `y1`.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a year outside the Gregorian years.
pub fn leapdays(y1: i64, y2: i64) -> Answer<i64> {
    year_in_range(y1)?;
    year_in_range(y2)?;
    Ok(calendar::leapdays(y1, y2))
}

/// The value of `hc_calendar_weekday`: Python's `calendar.weekday(year,
/// month, day)`, the weekday of a date with Monday 0 and Sunday 6.
///
/// # Errors
///
/// [`Refusal::InvalidDate`] for a date that does not exist and
/// [`Refusal::OutOfRange`] for a year outside the Gregorian years.
pub fn calendar_weekday(year: i64, month: u32, day: u32) -> Answer<u8> {
    year_in_range(year)?;
    calendar::weekday(year, field(month)?, field(day)?).map_err(Refusal::from)
}

/// The line of `hc_monthrange`: Python's `calendar.monthrange(year,
/// month)`, the weekday of the first day of the month, Monday 0, and the
/// number of days in the month.
///
/// # Errors
///
/// [`Refusal::InvalidDate`] for a month outside 1 to 12 and
/// [`Refusal::OutOfRange`] for a year outside the Gregorian years.
pub fn monthrange_line(year: i64, month: u32) -> Answer<String> {
    year_in_range(year)?;
    let (first, days) = calendar::monthrange(year, field(month)?).map_err(Refusal::from)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(first).value(days);
    line.end();
    Ok(out)
}

/// The lines of `hc_monthcalendar`: Python's `calendar.monthcalendar(year,
/// month)` with the first weekday of the week as an argument, Monday 0 to
/// Sunday 6, as `calendar.setfirstweekday` sets it: one line a week, seven
/// cells each, the day of the month or `0` for a day outside the month.
///
/// # Errors
///
/// [`Refusal::InvalidDate`] for a month outside 1 to 12 or a first weekday
/// above 6, and [`Refusal::OutOfRange`] for a year outside the Gregorian
/// years.
pub fn monthcalendar_lines(year: i64, month: u32, first_weekday: u32) -> Answer<String> {
    year_in_range(year)?;
    let weeks = calendar::monthcalendar(year, field(month)?, field(first_weekday)?)
        .map_err(Refusal::from)?;
    let mut out = String::new();
    for week in weeks {
        let mut line = Line::new(&mut out);
        for day in week {
            line.value(day);
        }
        line.end();
    }
    Ok(out)
}

/// The week rule a first weekday and a count of minimal days name: the
/// first day as an ISO 8601 weekday number, Monday 1 to Sunday 7, and the
/// fewest days of a year or month a week needs to be its first, 1 to 7.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a weekday or a count outside 1 to 7.
pub fn week_rule(first_weekday: u32, min_days: u32) -> Answer<WeekRule> {
    let first = u8::try_from(first_weekday)
        .ok()
        .and_then(Weekday::from_iso_number)
        .ok_or(Refusal::OutOfRange)?;
    let min_days = u8::try_from(min_days).map_err(|_| Refusal::OutOfRange)?;
    if !(1..=7).contains(&min_days) {
        return Err(Refusal::OutOfRange);
    }
    Ok(WeekRule::new(first, min_days))
}

/// The line of `hc_week_of_year`: a fixed day's place in the weeks of the
/// Gregorian year under a week rule — the week-numbering year, the week of
/// that year from 1, the number of weeks that year has, 52 or 53, and the
/// week of the month, 1 for the month's first week and 0 for the days
/// before it. ISO 8601's rule is Monday and 4, `strftime`'s `%U` is Sunday
/// and 7 and its `%W` Monday and 7 (both counting the days before week 1
/// as the year's week 0, where this counts them as the last week of the
/// year before), and a locale's pair is `hc_locale_info`'s columns 14 and
/// 15.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a rule [`week_rule`] does not read or a day
/// outside the Gregorian years.
pub fn week_of_year_line(fixed: i64, first_weekday: u32, min_days: u32) -> Answer<String> {
    let rule = week_rule(first_weekday, min_days)?;
    let day = Rd(fixed);
    let year = hc_calendars_solar::gregorian::year_from_fixed(day).map_err(Refusal::from)?;
    // The weeks either side of the year's ends belong to the year before or
    // after, whose week 1 the rule finds with plain arithmetic; the range
    // keeps a year on each side so that it never overflows.
    if !(gregorian::MIN_YEAR + 1..=gregorian::MAX_YEAR - 1).contains(&year) {
        return Err(Refusal::OutOfRange);
    }
    let (week_year, week) = rule.week_of_year(day);
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(week_year)
        .value(week)
        .value(rule.weeks_in_year(week_year))
        .value(rule.week_of_month(day));
    line.end();
    Ok(out)
}

/// The refusal a zone error is: a reading no instant names, or two name,
/// is [`Refusal::InvalidDate`], as a date that does not exist is; a value
/// the rules do not reach, [`Refusal::OutOfRange`].
#[cfg(all(feature = "tz", feature = "std"))]
fn zone_refusal(error: hc_tz::TzError) -> Refusal {
    match error {
        hc_tz::TzError::NonexistentLocalTime | hc_tz::TzError::AmbiguousLocalTime => {
            Refusal::InvalidDate
        }
        _ => Refusal::OutOfRange,
    }
}

/// The disambiguation policy a name selects: `earliest`, `latest`,
/// `reject` or `push-forward`, in any case, as [`hc_tz::Disambiguation`]
/// has them.
///
/// # Errors
///
/// [`Refusal::Unknown`] for another name.
#[cfg(all(feature = "tz", feature = "std"))]
pub fn disambiguation(name: &str) -> Answer<hc_tz::Disambiguation> {
    use hc_tz::Disambiguation;
    [
        ("earliest", Disambiguation::Earliest),
        ("latest", Disambiguation::Latest),
        ("reject", Disambiguation::Reject),
        ("push-forward", Disambiguation::PushForward),
    ]
    .into_iter()
    .find(|(id, _)| hc_core::catalogue::matches(name, id))
    .map(|(_, policy)| policy)
    .ok_or(Refusal::Unknown)
}

/// The line of `hc_localtime`: Python's `time.localtime(seconds)` in a
/// zone, the wall-clock reading of a POSIX second as the nine fields of a
/// `struct_time`, the daylight flag 1 where the zone's rules call the time
/// daylight saving and 0 where not. The zone is read as `hc_zone_offset`
/// reads it ([`crate::zone_lines::with_zone`]).
///
/// # Errors
///
/// [`Refusal::Unknown`] for a zone neither loaded nor built in, and
/// [`Refusal::OutOfRange`] for an instant the zone's rules do not answer
/// for.
#[cfg(all(feature = "tz", feature = "std"))]
pub fn localtime_line(unix_seconds: i64, zone: &str) -> Answer<String> {
    crate::zone_lines::with_zone(zone, |rules, _| {
        let instant = crate::zone_lines::instant_in_zone_range(unix_seconds)?;
        StructTime::localtime(instant, rules)
            .map(struct_time_line)
            .map_err(zone_refusal)
    })?
}

/// The value of `hc_mktime`: Python's `time.mktime(tuple)` in a zone, the
/// POSIX second of a wall-clock reading given as a year, a month and a
/// day, an hour, a minute and a second, each checked as
/// `datetime(*tuple[:6])` checks them. A reading two instants name, on
/// the morning the clocks go back, is the one `policy` chooses, and one
/// none names, when the clocks go forward, is what `policy` makes of it:
/// `earliest`, `latest`, `reject` or `push-forward`, where Python reads
/// `tm_isdst`.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a zone neither loaded nor built in or a policy
/// not named; [`Refusal::InvalidDate`] for a field out of range, and for a
/// reading no instant or two instants name under `reject`;
/// [`Refusal::OutOfRange`] for a reading the zone's rules do not reach.
#[cfg(all(feature = "tz", feature = "std"))]
pub fn mktime(fields: [i64; 6], zone: &str, policy: &str) -> Answer<i64> {
    let policy = disambiguation(policy)?;
    crate::zone_lines::with_zone(zone, |rules, _| {
        let reading = struct_time_of(fields)
            .to_date_time()
            .map_err(Refusal::from)?;
        crate::zone_lines::day_in_zone_range(reading.date.fixed().0)?;
        reading
            .timestamp(rules, policy)
            .map(|instant| instant.seconds())
            .map_err(zone_refusal)
    })?
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixed(year: i64, month: u8, day: u8) -> i64 {
        hc_calendars_solar::gregorian::to_fixed(year, month, day)
            .unwrap()
            .0
    }

    /// `time.gmtime(0)` and `calendar.timegm` from the `time` module's
    /// documentation: the epoch is a Thursday, day 1 of 1970.
    #[test]
    fn gmtime_and_timegm_are_each_others_inverse() {
        assert_eq!(gmtime_line(0).unwrap(), "1970\t1\t1\t0\t0\t0\t3\t1\t0\n");
        assert_eq!(timegm([1970, 1, 1, 0, 0, 0]).unwrap(), 0);
        // A day 32 is the first of the next month, as the module's "each
        // other's inverse" needs.
        assert_eq!(
            timegm([2026, 1, 32, 0, 0, 0]).unwrap(),
            timegm([2026, 2, 1, 0, 0, 0]).unwrap()
        );
        assert_eq!(timegm([2026, 13, 1, 0, 0, 0]), Err(Refusal::InvalidDate));
        assert_eq!(timegm([i64::MAX, 1, 1, 0, 0, 0]), Err(Refusal::OutOfRange));
        assert_eq!(
            timegm([9_999_999, 12, 31, i64::MAX, 0, 0]),
            Err(Refusal::Overflow)
        );
        assert_eq!(gmtime_line(i64::MAX), Err(Refusal::OutOfRange));
    }

    /// The `calendar` module's documentation: `monthrange(2026, 2)` is
    /// `(6, 28)`, a Sunday and twenty-eight days.
    #[test]
    fn the_calendar_functions_answer_as_pythons() {
        assert!(isleap(2024).unwrap() && !isleap(2100).unwrap());
        assert_eq!(leapdays(2000, 2026).unwrap(), 7);
        assert_eq!(leapdays(2026, 2000).unwrap(), -7);
        assert_eq!(leapdays(2000, i64::MIN), Err(Refusal::OutOfRange));
        assert_eq!(calendar_weekday(2026, 9, 21).unwrap(), 0);
        assert_eq!(calendar_weekday(2026, 2, 30), Err(Refusal::InvalidDate));
        assert_eq!(calendar_weekday(2026, 300, 1), Err(Refusal::InvalidDate));
        assert_eq!(monthrange_line(2026, 2).unwrap(), "6\t28\n");
        assert_eq!(monthrange_line(2026, 0), Err(Refusal::InvalidDate));
        // February 2026 from Monday: the 1st is a Sunday, so the first week
        // is six zeros and a 1, and the month fills five rows.
        let february = monthcalendar_lines(2026, 2, 0).unwrap();
        let rows: alloc::vec::Vec<&str> = february.lines().collect();
        assert_eq!(rows.len(), 5);
        assert_eq!(rows[0], "0\t0\t0\t0\t0\t0\t1");
        assert_eq!(rows[4], "23\t24\t25\t26\t27\t28\t0");
        // From Sunday the 1st opens the first row.
        assert!(
            monthcalendar_lines(2026, 2, 6)
                .unwrap()
                .starts_with("1\t2\t3\t4\t5\t6\t7\n")
        );
        assert_eq!(monthcalendar_lines(2026, 2, 7), Err(Refusal::InvalidDate));
    }

    /// UTS #35's example and ISO 8601's: 1 January 2021, a Friday, is week
    /// 1 of 2021 under Sunday and 1 and week 53 of 2020 under Monday and 4;
    /// `%U` and `%W` count it as week 0, which this reads as 2020's last.
    #[test]
    fn the_week_of_the_year_follows_the_rule() {
        let new_year = fixed(2021, 1, 1);
        assert_eq!(
            week_of_year_line(new_year, 7, 1).unwrap(),
            "2021\t1\t52\t1\n"
        );
        assert_eq!(
            week_of_year_line(new_year, 1, 4).unwrap(),
            "2020\t53\t53\t0\n"
        );
        // Sunday and 7 opens week 1 of 2020 on 5 January, so 1 January 2021 is
        // 51 whole weeks on: week 52 of a 52-week year.
        assert_eq!(
            week_of_year_line(new_year, 7, 7).unwrap(),
            "2020\t52\t52\t0\n"
        );
        assert_eq!(
            week_of_year_line(fixed(2026, 9, 21), 1, 4).unwrap(),
            "2026\t39\t53\t4\n"
        );
        assert_eq!(week_of_year_line(new_year, 0, 4), Err(Refusal::OutOfRange));
        assert_eq!(week_of_year_line(new_year, 8, 4), Err(Refusal::OutOfRange));
        assert_eq!(week_of_year_line(new_year, 1, 0), Err(Refusal::OutOfRange));
        assert_eq!(week_of_year_line(new_year, 1, 8), Err(Refusal::OutOfRange));
        assert_eq!(week_of_year_line(1 << 62, 1, 4), Err(Refusal::OutOfRange));
    }

    #[cfg(all(feature = "tz", feature = "std"))]
    #[test]
    fn localtime_and_mktime_read_a_zone() {
        // 2026-03-29 02:30 does not exist in Berlin; 2026-10-25 02:30 is
        // read twice.
        assert_eq!(
            localtime_line(0, "Asia/Tokyo").unwrap(),
            "1970\t1\t1\t9\t0\t0\t3\t1\t0\n"
        );
        assert_eq!(
            mktime([1970, 1, 1, 9, 0, 0], "Asia/Tokyo", "reject").unwrap(),
            0
        );
        assert_eq!(
            mktime([2026, 3, 29, 2, 30, 0], "Europe/Berlin", "reject"),
            Err(Refusal::InvalidDate)
        );
        let earliest = mktime([2026, 10, 25, 2, 30, 0], "Europe/Berlin", "earliest").unwrap();
        let latest = mktime([2026, 10, 25, 2, 30, 0], "Europe/Berlin", "latest").unwrap();
        assert_eq!(latest - earliest, 3600);
        assert_eq!(
            mktime([2026, 10, 25, 2, 30, 0], "Europe/Berlin", "reject"),
            Err(Refusal::InvalidDate)
        );
        assert_eq!(
            mktime([2026, 1, 1, 0, 0, 0], "Mars/Olympus", "reject"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            mktime([2026, 1, 1, 0, 0, 0], "Asia/Tokyo", "guess"),
            Err(Refusal::Unknown)
        );
        assert_eq!(localtime_line(0, "Mars/Olympus"), Err(Refusal::Unknown));
    }
}
