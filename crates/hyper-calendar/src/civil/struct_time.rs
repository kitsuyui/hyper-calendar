//! Python's `time.struct_time`, and the functions of the `time` and
//! `calendar` modules that work on it.
//!
//! [`StructTime`] is the nine-field tuple `time.gmtime()`,
//! `time.strptime()` and `date.timetuple()` return: year, month, day, hour,
//! minute, second, weekday with Monday 0, day of the year from 1, and the
//! daylight-saving flag, which is 0, 1 or -1 (unknown). Like Python's, it is
//! a bag of integers that is not checked when it is built —
//! `calendar.timegm()` is documented to take whatever the fields add up to —
//! and [`StructTime::to_date_time`] is where a reading that names no moment is
//! refused.
//!
//! The behaviour is that of the Python 3.13 documentation of
//! [`time`](https://docs.python.org/3.13/library/time.html) and
//! [`calendar`](https://docs.python.org/3.13/library/calendar.html), read
//! 2026-10-03.
//!
//! # What is not carried, and why
//!
//! * `tm_zone` and `tm_gmtoff` are the zone the machine was in. A reading has
//!   no zone of its own here (policy §13); the zone's offset and
//!   abbreviation at an instant are `TimeZone::offset_at` and
//!   `TimeZone::abbreviation_at`.
//! * `time.localtime()` with no argument, `time.timezone`, `time.altzone`,
//!   `time.daylight`, `time.tzname` and `time.time()` read the machine's clock
//!   or zone. [`StructTime::localtime`] takes the zone and the instant.
//! * A second of 61, which the documentation calls historical, is refused.

use hc_calendar::fixed::RD_OF_UNIX_EPOCH;
use hc_calendar::{CalendarError, CalendarResult, Rd};
use hc_calendars_solar::gregorian;
use hc_core::duration::days_and_seconds;

use super::{Date, DateTime, Time};

/// A broken-down time, as Python's `time.struct_time` holds it.
///
/// ```
/// use hyper_calendar::civil::StructTime;
///
/// // time.gmtime(0), from the `time` module's documentation.
/// let epoch = StructTime::gmtime(0)?;
/// assert_eq!(
///     (epoch.tm_year, epoch.tm_mon, epoch.tm_mday, epoch.tm_wday, epoch.tm_yday, epoch.tm_isdst),
///     (1970, 1, 1, 3, 1, 0)
/// );
/// assert_eq!(epoch.timegm()?, 0);
/// # Ok::<(), hyper_calendar::CalendarError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StructTime {
    /// The year, `1993`.
    pub tm_year: i64,
    /// The month, 1 to 12.
    pub tm_mon: i64,
    /// The day of the month, 1 to 31.
    pub tm_mday: i64,
    /// The hour, 0 to 23.
    pub tm_hour: i64,
    /// The minute, 0 to 59.
    pub tm_min: i64,
    /// The second, 0 to 60 (61 is refused).
    pub tm_sec: i64,
    /// The weekday, Monday 0 to Sunday 6.
    pub tm_wday: i64,
    /// The day of the year, 1 to 366.
    pub tm_yday: i64,
    /// Whether daylight saving time is in force: 0, 1, or -1 for unknown.
    pub tm_isdst: i64,
}

impl StructTime {
    /// A reading as `date.timetuple()` has it: midnight, with an unknown
    /// daylight-saving flag.
    #[must_use]
    pub fn from_date(date: Date) -> Self {
        Self::from_date_time(DateTime::midnight(date), -1)
    }

    /// A reading as `datetime.timetuple()` has it, with the flag the caller
    /// knows (Python's is -1 for a naive datetime).
    #[must_use]
    pub fn from_date_time(reading: DateTime, isdst: i64) -> Self {
        let (date, time) = (reading.date, reading.time);
        Self {
            tm_year: date.year(),
            tm_mon: i64::from(date.month()),
            tm_mday: i64::from(date.day()),
            tm_hour: i64::from(time.hour()),
            tm_min: i64::from(time.minute()),
            tm_sec: i64::from(time.second()),
            tm_wday: i64::from(date.weekday().monday_first_number()),
            tm_yday: i64::from(date.day_of_year()),
            tm_isdst: isdst,
        }
    }

    /// `time.gmtime(seconds)`: the UTC reading of a count of seconds since
    /// the epoch, with the flag 0.
    ///
    /// # Errors
    ///
    /// [`CalendarError`] when the day is outside the Gregorian range.
    pub fn gmtime(seconds: i64) -> CalendarResult<Self> {
        let (days, of_day) = days_and_seconds(seconds);
        let date = Date::from_fixed(Rd(RD_OF_UNIX_EPOCH
            .checked_add(days)
            .ok_or(CalendarError::Overflow)?))?;
        let time = Time::hms(
            (of_day / 3_600) as u8,
            (of_day % 3_600 / 60) as u8,
            (of_day % 60) as u8,
        )?;
        Ok(Self::from_date_time(DateTime::new(date, time), 0))
    }

    /// `calendar.timegm(tuple)`: the seconds since the epoch of a UTC
    /// reading. Only the first six fields count, and the day, hour, minute
    /// and second are not checked: they add up, as the documentation's
    /// "each other's inverse" for `gmtime` needs, so `tm_mday` 32 is the
    /// first of the next month. The month must be 1 to 12.
    ///
    /// # Errors
    ///
    /// [`CalendarError`] for a month outside 1 to 12, which Python's
    /// `datetime.date(year, month, 1)` refuses, and for a year outside the
    /// Gregorian range; [`CalendarError::Overflow`] where the sum leaves an
    /// `i64`, which Python's unbounded integers never do.
    pub fn timegm(&self) -> CalendarResult<i64> {
        let month = u8::try_from(self.tm_mon).map_err(|_| CalendarError::MonthOutOfRange)?;
        let first = gregorian::to_fixed(self.tm_year, month, 1)?.0;
        let step = |total: i64, scale: i64, field: i64| {
            total
                .checked_mul(scale)
                .and_then(|scaled| scaled.checked_add(field))
                .ok_or(CalendarError::Overflow)
        };
        let days = step(first - RD_OF_UNIX_EPOCH, 1, self.tm_mday)?
            .checked_sub(1)
            .ok_or(CalendarError::Overflow)?;
        let hours = step(days, 24, self.tm_hour)?;
        let minutes = step(hours, 60, self.tm_min)?;
        step(minutes, 60, self.tm_sec)
    }

    /// `datetime(*tt[:6])`: the reading the first six fields name.
    ///
    /// # Errors
    ///
    /// [`CalendarError`] when a field is out of range, as Python's
    /// `ValueError`.
    pub fn to_date_time(&self) -> CalendarResult<DateTime> {
        let field = |value: i64| u8::try_from(value).map_err(|_| CalendarError::MonthOutOfRange);
        let date = Date::new(self.tm_year, field(self.tm_mon)?, field(self.tm_mday)?)?;
        let time = Time::hms(
            field(self.tm_hour)?,
            field(self.tm_min)?,
            field(self.tm_sec)?,
        )?;
        Ok(DateTime::new(date, time))
    }
}

#[cfg(feature = "tz")]
impl StructTime {
    /// `time.localtime(seconds)` for a zone: the reading the zone's clocks
    /// showed, with the flag set to whether daylight saving was in force.
    ///
    /// # Errors
    ///
    /// A [`hc_tz::TzError`] outside the Gregorian range.
    pub fn localtime<Z: hc_tz::TimeZone + ?Sized>(
        unix: hc_core::UnixTime,
        zone: &Z,
    ) -> hc_tz::TzResult<Self> {
        let reading = DateTime::from_timestamp(unix, zone)?;
        Ok(Self::from_date_time(
            reading,
            i64::from(zone.is_dst_at(unix)),
        ))
    }

    /// `time.mktime(tuple)` for a zone, as seconds: the instant the first six
    /// fields name in `zone`. Python chooses between a repeated reading's two
    /// instants with `tm_isdst`; here the choice is the `policy` argument.
    ///
    /// # Errors
    ///
    /// A [`hc_tz::TzError`] for a reading no instant names, and under
    /// [`hc_tz::Disambiguation::Reject`] for one that two do.
    pub fn mktime<Z: hc_tz::TimeZone + ?Sized>(
        &self,
        zone: &Z,
        policy: hc_tz::Disambiguation,
    ) -> hc_tz::TzResult<hc_core::UnixTime> {
        self.to_date_time()?.timestamp(zone, policy)
    }
}

#[cfg(feature = "format")]
impl StructTime {
    /// `time.strptime(text, pattern)`: `datetime.strptime`'s reading as a
    /// tuple, with the flag -1.
    ///
    /// ```
    /// use hyper_calendar::civil::StructTime;
    ///
    /// // time.strptime("30 Nov 00", "%d %b %y"), from the documentation.
    /// let parsed = StructTime::strptime("30 Nov 00", "%d %b %y")?;
    /// assert_eq!(
    ///     (parsed.tm_year, parsed.tm_mon, parsed.tm_mday, parsed.tm_wday, parsed.tm_yday, parsed.tm_isdst),
    ///     (2000, 11, 30, 3, 335, -1)
    /// );
    /// # Ok::<(), hyper_calendar::civil::StrptimeError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// As [`DateTime::strptime`].
    pub fn strptime(text: &str, pattern: &str) -> Result<Self, super::StrptimeError> {
        let (reading, _) = DateTime::strptime(text, pattern)?;
        Ok(Self::from_date_time(reading, -1))
    }

    /// `time.asctime(tuple)`: `Sun Jun 20 23:21:05 1993`, the day padded
    /// with a space.
    ///
    /// # Errors
    ///
    /// A [`CalendarError`] when the fields name no reading, and
    /// [`hc_format::FormatError`] when the sink refuses.
    #[cfg(feature = "alloc")]
    pub fn asctime(&self) -> Result<alloc::string::String, AsctimeError> {
        self.to_date_time()
            .map_err(AsctimeError::Value)?
            .ctime()
            .map_err(AsctimeError::Format)
    }

    /// `time.strftime(pattern, tuple)`.
    ///
    /// # Errors
    ///
    /// As [`StructTime::asctime`].
    #[cfg(feature = "alloc")]
    pub fn strftime(&self, pattern: &str) -> Result<alloc::string::String, AsctimeError> {
        self.to_date_time()
            .map_err(AsctimeError::Value)?
            .strftime(pattern)
            .map_err(AsctimeError::Format)
    }
}

/// Why [`StructTime::asctime`] or [`StructTime::strftime`] failed.
#[cfg(feature = "format")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsctimeError {
    /// The fields name no reading.
    Value(CalendarError),
    /// The text could not be written.
    Format(hc_format::FormatError),
}

#[cfg(feature = "format")]
impl core::fmt::Display for AsctimeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Value(error) => write!(f, "{error}"),
            Self::Format(error) => write!(f, "{error}"),
        }
    }
}

#[cfg(all(feature = "format", feature = "std"))]
impl std::error::Error for AsctimeError {}

// --- the calendar module ----------------------------------------------------

/// The functions of Python's `calendar` module that work on a year, a month
/// or a day.
///
/// `calendar.isleap` is [`is_leap_year`](calendar::is_leap_year),
/// `calendar.leapdays` is [`leapdays`](calendar::leapdays),
/// `calendar.weekday` is [`weekday`](calendar::weekday),
/// `calendar.monthrange` is [`monthrange`](calendar::monthrange),
/// `calendar.monthcalendar` is [`monthcalendar`](calendar::monthcalendar)
/// and `calendar.timegm` is [`StructTime::timegm`]. The names of the days
/// and months are `Date::strftime("%A")` and its kin, in a locale through
/// `FormatContext`; the text and HTML calendar renderers are not carried (see
/// `docs/python-parity.md`).
pub mod calendar {
    use hc_calendar::{CalendarError, CalendarResult};

    use super::{Date, gregorian};

    /// `calendar.isleap(year)`.
    #[must_use]
    pub const fn is_leap_year(year: i64) -> bool {
        gregorian::is_leap_year(year)
    }

    /// `calendar.leapdays(y1, y2)`: the number of leap years from `y1` up
    /// to but not including `y2`. It counts backwards, as Python's does,
    /// when `y2` is before `y1`.
    #[must_use]
    pub const fn leapdays(y1: i64, y2: i64) -> i64 {
        let (y1, y2) = (y1 - 1, y2 - 1);
        (y2.div_euclid(4) - y1.div_euclid(4)) - (y2.div_euclid(100) - y1.div_euclid(100))
            + (y2.div_euclid(400) - y1.div_euclid(400))
    }

    /// `calendar.weekday(year, month, day)`: Monday is 0.
    ///
    /// # Errors
    ///
    /// [`CalendarError`] for a day that does not exist.
    pub fn weekday(year: i64, month: u8, day: u8) -> CalendarResult<u8> {
        Ok(Date::new(year, month, day)?.weekday().monday_first_number())
    }

    /// `calendar.monthrange(year, month)`: the weekday of the first and the
    /// number of days.
    ///
    /// # Errors
    ///
    /// [`CalendarError::MonthOutOfRange`] for a month outside 1 to 12.
    pub fn monthrange(year: i64, month: u8) -> CalendarResult<(u8, u8)> {
        let first = Date::new(year, month, 1)?;
        Ok((first.weekday().monday_first_number(), first.days_in_month()))
    }

    /// `calendar.monthcalendar(year, month)`, with the first weekday of the
    /// week as an argument (`calendar.setfirstweekday` is a global in Python:
    /// 0 for Monday): a row for each week, and 0 for a day outside the
    /// month.
    ///
    /// # Errors
    ///
    /// [`CalendarError::MonthOutOfRange`] for a month outside 1 to 12 and
    /// [`CalendarError::DayOutOfRange`] for a first weekday above 6.
    #[cfg(feature = "alloc")]
    pub fn monthcalendar(
        year: i64,
        month: u8,
        first_weekday: u8,
    ) -> CalendarResult<alloc::vec::Vec<[u8; 7]>> {
        if first_weekday > 6 {
            return Err(CalendarError::DayOutOfRange);
        }
        let (first, days) = monthrange(year, month)?;
        let mut weeks = alloc::vec::Vec::new();
        let mut week = [0u8; 7];
        let mut column = usize::from((first + 7 - first_weekday) % 7);
        for day in 1..=days {
            if let Some(cell) = week.get_mut(column) {
                *cell = day;
            }
            column += 1;
            if column == 7 {
                weeks.push(week);
                week = [0; 7];
                column = 0;
            }
        }
        if column > 0 {
            weeks.push(week);
        }
        Ok(weeks)
    }
}
