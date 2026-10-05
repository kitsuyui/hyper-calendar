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
//! * `time.asctime` as text, in a build with the `format` feature:
//!   `asctime_line`.
//! * The week of the year under a week rule — the first day of the week and
//!   the fewest days a first week holds, which is ISO 8601's Monday and 4,
//!   `%U`'s Sunday and 7, `%W`'s Monday and 7, and every locale's pair:
//!   [`week_of_year_line`], and the day a week date names under it:
//!   [`fixed_from_week`].
//! * What a wall-clock reading means in a zone, before a policy reduces it
//!   to one instant: [`local_resolution_line`], and the policies
//!   [`mktime`] reads, [`mktime_policies_lines`].
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

/// How many columns `asctime_line` writes.
pub const ASCTIME_COLUMNS: usize = 1;

/// How many columns each line of [`mktime_policies_lines`] writes.
pub const MKTIME_POLICY_COLUMNS: usize = 2;

/// How many columns [`local_resolution_line`] writes.
pub const LOCAL_RESOLUTION_COLUMNS: usize = 5;

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

/// The value of `hc_fixed_from_week`: the fixed day a week date names under
/// a week rule, the inverse of [`week_of_year_line`] — the week-numbering
/// year, the week of it from 1, the weekday as an ISO 8601 number (Monday 1
/// to Sunday 7), and the rule's first weekday and minimal days as
/// [`week_rule`] reads them. ISO 8601's week date is Monday and 4.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a rule [`week_rule`] does not read or a year
/// whose weeks lie beyond the Gregorian years, and [`Refusal::InvalidDate`]
/// for a weekday outside 1 to 7 or a week the year does not have: week 53
/// of a year of 52 weeks names no day, as 31 February does not.
pub fn fixed_from_week(
    week_year: i64,
    week: u32,
    weekday: u32,
    first_weekday: u32,
    min_days: u32,
) -> Answer<i64> {
    let rule = week_rule(first_weekday, min_days)?;
    if !(gregorian::MIN_YEAR + 1..=gregorian::MAX_YEAR - 1).contains(&week_year) {
        return Err(Refusal::OutOfRange);
    }
    let weekday = u8::try_from(weekday)
        .ok()
        .and_then(Weekday::from_iso_number)
        .ok_or(Refusal::InvalidDate)?;
    let week = u8::try_from(week).map_err(|_| Refusal::InvalidDate)?;
    if week == 0 || week > rule.weeks_in_year(week_year) {
        return Err(Refusal::InvalidDate);
    }
    Ok(rule.to_fixed(week_year, week, weekday).0)
}

/// The line of `hc_asctime`: Python's `time.asctime` of a Gregorian
/// reading, `Sun Jun 20 23:21:05 1993`, the day padded with a space, one
/// cell. The weekday is the date's, as `datetime.ctime` computes it; the
/// tuple's `tm_wday` that `time.asctime` reads is not an argument.
///
/// # Errors
///
/// [`Refusal::InvalidDate`] for a field out of range, as
/// `datetime(*tuple[:6])` raises, a second of 60 included, and
/// [`Refusal::OutOfRange`] for a year outside 1 to 9999, which `datetime`
/// cannot hold.
#[cfg(feature = "format")]
pub fn asctime_line(fields: [i64; 6]) -> Answer<String> {
    if !(1..=9_999).contains(&fields[0]) {
        return Err(Refusal::OutOfRange);
    }
    let text = struct_time_of(fields)
        .asctime()
        .map_err(|error| match error {
            crate::civil::AsctimeError::Value(error) => Refusal::from(error),
            crate::civil::AsctimeError::Format(_) => Refusal::OutOfRange,
        })?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell(&text);
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

/// The disambiguation policy a name selects, from [`hc_tz::Disambiguation::ALL`]:
/// `earliest`, `latest`, `reject` or `push-forward`, in any case.
///
/// # Errors
///
/// [`Refusal::Unknown`] for another name.
#[cfg(all(feature = "tz", feature = "std"))]
pub fn disambiguation(name: &str) -> Answer<hc_tz::Disambiguation> {
    hc_tz::Disambiguation::by_id(name).ok_or(Refusal::Unknown)
}

/// The lines of `hc_mktime_policies`: the policies `hc_mktime` reads a
/// word for, one a line in the order of [`hc_tz::Disambiguation::ALL`] — the
/// identifier, and what the policy makes of a reading two instants name and
/// of one no instant names.
#[cfg(all(feature = "tz", feature = "std"))]
pub fn mktime_policies_lines() -> String {
    let mut out = String::new();
    for policy in hc_tz::Disambiguation::ALL {
        let mut line = Line::new(&mut out);
        line.cell(policy.id()).cell(policy.description());
        line.end();
    }
    out
}

/// The line of `hc_local_resolution`: what a wall-clock reading given as a
/// year, a month and a day, an hour, a minute and a second means in a zone
/// before any policy reduces it to one instant. The cells:
///
/// 1. `unique` where the zone's clock showed the reading once, `ambiguous`
///    where the clocks went back and showed it twice, `nonexistent` where
///    they went forward past it;
/// 2. the first instant, as a POSIX second: the reading itself, the first of
///    the two occurrences, or, for a skipped reading, the instant it would
///    be under the offset in force after the gap, before the gap opened;
/// 3. the offset of the first instant, in seconds east of UTC, which is the
///    offset the reading is subtracted by;
/// 4. the second instant: the reading again where it is unique, the second
///    occurrence, or, for a skipped reading, the instant it would be under
///    the offset in force before the gap, after the gap closed;
/// 5. the offset of the second instant.
///
/// The `earliest`, `latest`, `reject` and `push-forward` policies of
/// [`mktime`] choose between cells 2 and 4 or refuse. The reading's fields
/// are checked as [`mktime`]'s are.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a zone neither loaded nor built in,
/// [`Refusal::InvalidDate`] for a field out of range, and
/// [`Refusal::OutOfRange`] for a reading the zone's rules do not reach.
#[cfg(all(feature = "tz", feature = "std"))]
pub fn local_resolution_line(fields: [i64; 6], zone: &str) -> Answer<String> {
    use hc_tz::LocalResolution;
    crate::zone_lines::with_zone(zone, |rules, _| {
        let reading = struct_time_of(fields)
            .to_date_time()
            .map_err(Refusal::from)?;
        crate::zone_lines::day_in_zone_range(reading.date.fixed().0)?;
        let (kind, first, first_offset, second, second_offset) =
            match rules.resolve_local(reading.to_civil()) {
                LocalResolution::Unambiguous(instant) => {
                    let offset = rules.offset_at(instant);
                    ("unique", instant, offset, instant, offset)
                }
                LocalResolution::Ambiguous {
                    earlier,
                    later,
                    earlier_offset,
                    later_offset,
                } => ("ambiguous", earlier, earlier_offset, later, later_offset),
                LocalResolution::Nonexistent {
                    before_gap,
                    after_gap,
                    offset_before,
                    offset_after,
                    ..
                } => (
                    "nonexistent",
                    before_gap,
                    offset_after,
                    after_gap,
                    offset_before,
                ),
            };
        let mut out = String::new();
        let mut line = Line::new(&mut out);
        line.cell(kind)
            .value(first.seconds())
            .value(first_offset.seconds())
            .value(second.seconds())
            .value(second_offset.seconds());
        line.end();
        Ok(out)
    })?
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

    /// `datetime(2026, 6, 30, 23, 59, 60)` raises in CPython, and `time.mktime`
    /// here checks the reading as it does: a second 60 is refused on every
    /// day, in every zone, whether or not the day ended in a leap second.
    #[cfg(all(feature = "tz", feature = "std"))]
    #[test]
    fn mktime_refuses_a_second_60_on_every_day_in_every_zone() {
        for (fields, zone) in [
            ([2026, 6, 30, 23, 59, 60], "UTC"),
            ([2016, 12, 31, 23, 59, 60], "UTC"),
            ([2026, 1, 1, 23, 59, 60], "America/New_York"),
            ([2026, 9, 21, 12, 0, 60], "Asia/Tokyo"),
            ([2017, 1, 1, 8, 59, 60], "Asia/Tokyo"),
        ] {
            for policy in ["earliest", "reject"] {
                assert_eq!(
                    mktime(fields, zone, policy),
                    Err(Refusal::InvalidDate),
                    "{fields:?} {zone}"
                );
            }
            assert_eq!(
                local_resolution_line(fields, zone),
                Err(Refusal::InvalidDate)
            );
        }
        assert_eq!(
            mktime([2026, 6, 30, 23, 59, 59], "UTC", "reject").unwrap(),
            1_782_863_999
        );
    }

    /// CPython 3.9.6's `zoneinfo`: Berlin read at 02:30 on 2026-10-25 is
    /// 1 792 888 200 at +2 h with `fold=0` and 1 792 891 800 at +1 h with
    /// `fold=1`; at 02:30 on 2026-03-29 `fold=0` is 1 774 747 800 at +1 h and
    /// `fold=1` 1 774 744 200 at +2 h, the skipped reading under each side's
    /// offset.
    #[cfg(all(feature = "tz", feature = "std"))]
    #[test]
    fn a_local_reading_resolves_to_one_two_or_no_instants() {
        assert_eq!(
            local_resolution_line([2026, 7, 1, 12, 0, 0], "Europe/Berlin").unwrap(),
            "unique\t1782900000\t7200\t1782900000\t7200\n"
        );
        assert_eq!(
            local_resolution_line([2026, 10, 25, 2, 30, 0], "Europe/Berlin").unwrap(),
            "ambiguous\t1792888200\t7200\t1792891800\t3600\n"
        );
        assert_eq!(
            local_resolution_line([2026, 3, 29, 2, 30, 0], "Europe/Berlin").unwrap(),
            "nonexistent\t1774744200\t7200\t1774747800\t3600\n"
        );
        // The policies of `mktime` choose between the two instants.
        for (fields, policy, cell) in [
            ([2026, 10, 25, 2, 30, 0], "earliest", 1),
            ([2026, 10, 25, 2, 30, 0], "latest", 3),
            ([2026, 10, 25, 2, 30, 0], "push-forward", 1),
            ([2026, 3, 29, 2, 30, 0], "earliest", 1),
            ([2026, 3, 29, 2, 30, 0], "latest", 3),
            ([2026, 3, 29, 2, 30, 0], "push-forward", 3),
        ] {
            let line = local_resolution_line(fields, "Europe/Berlin").unwrap();
            let cells: Vec<&str> = line.trim_end().split('\t').collect();
            assert_eq!(cells.len(), LOCAL_RESOLUTION_COLUMNS);
            assert_eq!(
                mktime(fields, "Europe/Berlin", policy).unwrap().to_string(),
                cells[cell],
                "{fields:?} {policy}"
            );
        }
        assert_eq!(
            local_resolution_line([2026, 2, 30, 0, 0, 0], "Europe/Berlin"),
            Err(Refusal::InvalidDate)
        );
        assert_eq!(
            local_resolution_line([2026, 1, 1, 0, 0, 0], "Mars/Olympus"),
            Err(Refusal::Unknown)
        );
    }

    /// The four policies, each with a sentence, in the table's order.
    #[cfg(all(feature = "tz", feature = "std"))]
    #[test]
    fn the_mktime_policies_are_the_tables() {
        let lines = mktime_policies_lines();
        let ids: Vec<&str> = lines
            .lines()
            .map(|line| line.split('\t').next().unwrap())
            .collect();
        assert_eq!(ids, ["earliest", "latest", "reject", "push-forward"]);
        for line in lines.lines() {
            assert_eq!(line.split('\t').count(), MKTIME_POLICY_COLUMNS);
        }
        for id in ids {
            assert!(disambiguation(id).is_ok());
        }
    }

    /// `datetime.date.fromisocalendar` of CPython 3.9.6: week 53 of 2020, day
    /// 5, is 2021-01-01 and week 1 of 2026, day 1, is 2025-12-29. The inverse
    /// of `week_of_year_line` on every day of three years, under four rules.
    #[test]
    fn a_week_date_names_the_day_week_of_year_gave_it() {
        assert_eq!(
            fixed_from_week(2020, 53, 5, 1, 4).unwrap(),
            fixed(2021, 1, 1)
        );
        assert_eq!(
            fixed_from_week(2026, 1, 1, 1, 4).unwrap(),
            fixed(2025, 12, 29)
        );
        for (first, min) in [(1, 4), (7, 1), (7, 7), (1, 7)] {
            for day in fixed(2019, 12, 20)..=fixed(2022, 1, 10) {
                let line = week_of_year_line(day, first, min).unwrap();
                let cells: Vec<i64> = line
                    .trim_end()
                    .split('\t')
                    .map(|cell| cell.parse().unwrap())
                    .collect();
                let weekday = (day - 1).rem_euclid(7) + 1;
                assert_eq!(
                    fixed_from_week(
                        cells[0],
                        u32::try_from(cells[1]).unwrap(),
                        u32::try_from(weekday).unwrap(),
                        first,
                        min
                    ),
                    Ok(day),
                    "{day} under {first} and {min}"
                );
            }
        }
        // A week or a weekday the year does not have, and a rule that is none.
        assert_eq!(
            fixed_from_week(2025, 53, 1, 1, 4),
            Err(Refusal::InvalidDate)
        );
        assert_eq!(fixed_from_week(2026, 0, 1, 1, 4), Err(Refusal::InvalidDate));
        assert_eq!(fixed_from_week(2026, 1, 0, 1, 4), Err(Refusal::InvalidDate));
        assert_eq!(fixed_from_week(2026, 1, 8, 1, 4), Err(Refusal::InvalidDate));
        assert_eq!(fixed_from_week(2026, 1, 1, 0, 4), Err(Refusal::OutOfRange));
        assert_eq!(fixed_from_week(2026, 1, 1, 1, 8), Err(Refusal::OutOfRange));
        assert_eq!(
            fixed_from_week(gregorian::MAX_YEAR, 1, 1, 1, 4),
            Err(Refusal::OutOfRange)
        );
    }

    /// CPython 3.9.6: `datetime(1993, 6, 20, 23, 21, 5).ctime()` is `Sun Jun 20
    /// 23:21:05 1993`; the weekday is the date's, where `time.asctime` reads
    /// the tuple's.
    #[cfg(feature = "format")]
    #[test]
    fn asctime_is_ctime_of_the_reading() {
        assert_eq!(
            asctime_line([1993, 6, 20, 23, 21, 5]).unwrap(),
            "Sun Jun 20 23:21:05 1993\n"
        );
        assert_eq!(
            asctime_line([2026, 9, 21, 1, 2, 3]).unwrap(),
            "Mon Sep 21 01:02:03 2026\n"
        );
        assert_eq!(
            asctime_line([2026, 2, 30, 1, 2, 3]),
            Err(Refusal::InvalidDate)
        );
        assert_eq!(
            asctime_line([2026, 9, 21, 24, 0, 0]),
            Err(Refusal::InvalidDate)
        );
        // A second of 60 is no reading, on a day that ended in a leap second or not.
        for fields in [[2016, 12, 31, 23, 59, 60], [2026, 6, 30, 23, 59, 60]] {
            assert_eq!(
                asctime_line(fields),
                Err(Refusal::InvalidDate),
                "{fields:?}"
            );
        }
        // A year Python's `datetime` cannot hold, outside 1 to 9999.
        for year in [0, -5, 12_345, 10_000_000] {
            assert_eq!(
                asctime_line([year, 9, 21, 1, 2, 3]),
                Err(Refusal::OutOfRange),
                "{year}"
            );
        }
    }
}
