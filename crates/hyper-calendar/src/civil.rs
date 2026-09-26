//! The everyday date and time types.
//!
//! These are the ergonomic layer requirement 2 of the project brief asks for:
//! the shape of Python's `datetime`, `date`, `time` and `timedelta`, over the
//! machinery in [`hc_core`] and [`hc_calendar`].
//!
//! # Why these exist at all
//!
//! The lower crates are deliberately uncompromising. `Rd` is a bare day
//! number, `CivilTime` can be a leap second, `Instant<S>` refuses to be read
//! on the wrong scale, and almost everything returns `Result`. That is right
//! for correctness and wrong for the ninety per cent of calls that are "what
//! is the date three weeks from Tuesday".
//!
//! So [`Date`] validates once at construction and is infallible afterwards.
//! It stores both the fixed day and the broken-down fields, which costs
//! sixteen bytes over a bare [`Rd`] and buys accessors that cannot fail.
//!
//! # The one borrowed idea worth naming
//!
//! Python's `date.toordinal()` returns the proleptic Gregorian ordinal with
//! `0001-01-01` as day 1 — which is exactly [`Rd`]. The correspondence is not
//! a coincidence; both come from the same place in the calendrical literature.
//! [`Date::to_ordinal`] and [`Date::from_ordinal`] are therefore the same
//! function as [`Date::fixed`] and [`Date::from_fixed`], and are provided
//! under both names so that a port from Python reads naturally.
//!
//! # Where the rest of Python lives
//!
//! This module holds the shapes; the rules are in the crates that own them,
//! and each method here is a thin adapter over one of them:
//!
//! | Python | Owner |
//! | --- | --- |
//! | `timedelta` arithmetic and `str()` | [`hc_core::Duration`] |
//! | `isocalendar`, `fromisocalendar` | [`hc_calendars_solar::iso_week`] |
//! | `fromisoformat`, `isoformat`, `strftime`, `strptime` | `hc_format::python`, `hc_format::patterns::strftime` (feature `format`) |
//! | `fromtimestamp`, `timestamp`, `astimezone` | `hc_tz` (feature `tz`) |
//! | `humanize` | `hc_humanize::natural` (feature `humanize`) |
//!
//! The whole correspondence, with what is missing and why, is
//! `docs/python-parity.md`.
//!
//! # Naive, and exact
//!
//! [`DateTime`] is Python's *naive* datetime: a wall-clock reading. Python's
//! aware datetime is a reading plus a zone, and here the zone travels beside
//! the reading — an [`hc_format::ZoneInfo`] from a parser, an
//! [`hc_tz::TimeZone`] passed to a method — rather than inside it, so that
//! a reading can never silently be treated as UTC.
//!
//! Spans are exact to the attosecond where Python's stop at the
//! microsecond, and the ranges are wider: years −9 999 999 to 9 999 999
//! rather than 1 to 9999. Where Python rounds a span to the microsecond —
//! `timedelta / int`, `timedelta * float` — this library floors at the
//! attosecond instead.
//!
//! # Operators
//!
//! `TimeDelta`'s `+`, `-`, unary `-`, `* i64`, `/ i64` and `%`, and `Date`
//! and `DateTime` plus or minus a `TimeDelta`, panic on overflow (and `/` and
//! `%` on a zero divisor), in release builds too, so that arithmetic reads
//! as it does in Python. Each has a `checked_*` twin that returns an error
//! instead (policy §8).

use core::fmt;
use core::ops::{Add, AddAssign, Div, Mul, Neg, Rem, Sub, SubAssign};

use hc_calendar::{CalendarError, CalendarResult, CivilDateTime, CivilTime, Rd, Weekday};
use hc_calendars_solar::gregorian;
use hc_calendars_solar::iso_week::{self, IsoWeekDate};
use hc_core::{ATTOS_PER_SEC, Duration, TimeResult};

/// A Gregorian calendar date.
///
/// Proleptic in both directions, with astronomical year numbering: 1 BC is
/// year 0 and 2 BC is year -1, so arithmetic never has to step over a
/// non-existent year zero.
///
/// ```
/// use hyper_calendar::civil::Date;
///
/// let date = Date::new(2026, 9, 21)?;
/// assert_eq!(date.weekday().english_name(), "Monday");
/// assert_eq!(date.to_ordinal(), 739_880);
/// # Ok::<(), hyper_calendar::CalendarError>(())
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    // The fixed day is the authority; the fields are cached so that the
    // accessors below cannot fail. They are kept private precisely so the two
    // cannot drift apart.
    fixed: Rd,
    year: i64,
    month: u8,
    day: u8,
}

impl Date {
    /// The Gregorian date `1970-01-01`.
    pub const UNIX_EPOCH: Self = Self {
        fixed: Rd(719_163),
        year: 1970,
        month: 1,
        day: 1,
    };

    /// The earliest date: 1 January of year −9 999 999.
    ///
    /// Python's `date.min` is `0001-01-01`; this range is wider.
    pub const MIN: Self = Self {
        fixed: gregorian::EARLIEST,
        year: gregorian::MIN_YEAR,
        month: 1,
        day: 1,
    };

    /// The latest date: 31 December of year 9 999 999.
    pub const MAX: Self = Self {
        fixed: gregorian::LATEST,
        year: gregorian::MAX_YEAR,
        month: 12,
        day: 31,
    };

    /// The smallest difference between two dates: one day. Python's
    /// `date.resolution`.
    pub const RESOLUTION: TimeDelta = TimeDelta(Duration::DAY);

    /// Build a date, validating it once.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::MonthOutOfRange`] or
    /// [`CalendarError::DayOutOfRange`] when the date does not exist, such as
    /// 30 February or 31 April.
    pub const fn new(year: i64, month: u8, day: u8) -> CalendarResult<Self> {
        match gregorian::to_fixed(year, month, day) {
            Ok(fixed) => Ok(Self {
                fixed,
                year,
                month,
                day,
            }),
            Err(error) => Err(error),
        }
    }

    /// The date on a given fixed day.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the day is outside the supported
    /// range.
    pub const fn from_fixed(fixed: Rd) -> CalendarResult<Self> {
        match gregorian::from_fixed(fixed) {
            Ok((year, month, day)) => Ok(Self {
                fixed,
                year,
                month,
                day,
            }),
            Err(error) => Err(error),
        }
    }

    /// The date on a given proleptic Gregorian ordinal, where `0001-01-01` is
    /// day 1.
    ///
    /// This is Python's `date.fromordinal`, and the same function as
    /// [`Date::from_fixed`].
    ///
    /// # Errors
    ///
    /// See [`Date::from_fixed`].
    pub const fn from_ordinal(ordinal: i64) -> CalendarResult<Self> {
        Self::from_fixed(Rd(ordinal))
    }

    /// The year.
    #[must_use]
    pub const fn year(self) -> i64 {
        self.year
    }

    /// The month, 1 through 12.
    #[must_use]
    pub const fn month(self) -> u8 {
        self.month
    }

    /// The day of the month, counting from 1.
    #[must_use]
    pub const fn day(self) -> u8 {
        self.day
    }

    /// The fixed day.
    #[must_use]
    pub const fn fixed(self) -> Rd {
        self.fixed
    }

    /// The proleptic Gregorian ordinal, where `0001-01-01` is day 1.
    ///
    /// This is Python's `date.toordinal`.
    #[must_use]
    pub const fn to_ordinal(self) -> i64 {
        self.fixed.0
    }

    /// The weekday.
    #[must_use]
    pub const fn weekday(self) -> Weekday {
        Weekday::from_rd(self.fixed)
    }

    /// The ISO 8601 weekday number, Monday = 1 through Sunday = 7.
    ///
    /// This is Python's `date.isoweekday`. Note that Python's `date.weekday`
    /// is zero-based from Monday; use `weekday().iso_number() - 1` for that,
    /// or better, use the [`Weekday`] value and avoid the ambiguity.
    #[must_use]
    pub const fn iso_weekday(self) -> u8 {
        self.weekday().iso_number()
    }

    /// The 1-based day of the year.
    #[must_use]
    pub const fn day_of_year(self) -> u16 {
        match gregorian::day_of_year(self.year, self.month, self.day) {
            Ok(value) => value,
            // Unreachable: the fields were validated at construction. Falling
            // back to 1 rather than panicking keeps this callable from a
            // `const` context and from a WebAssembly runtime.
            Err(_) => 1,
        }
    }

    /// Whether this date's year is a Gregorian leap year.
    #[must_use]
    pub const fn is_leap_year(self) -> bool {
        gregorian::is_leap_year(self.year)
    }

    /// The number of days in this date's month.
    #[must_use]
    pub const fn days_in_month(self) -> u8 {
        match gregorian::days_in_month(self.year, self.month) {
            Some(value) => value,
            // Unreachable: the month was validated at construction.
            None => 31,
        }
    }

    /// The same date with some fields replaced.
    ///
    /// This is Python's `date.replace`.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the result does not exist, which is
    /// the usual trap: replacing the month of 31 January with February.
    pub const fn replace(
        self,
        year: Option<i64>,
        month: Option<u8>,
        day: Option<u8>,
    ) -> CalendarResult<Self> {
        let year = match year {
            Some(value) => value,
            None => self.year,
        };
        let month = match month {
            Some(value) => value,
            None => self.month,
        };
        let day = match day {
            Some(value) => value,
            None => self.day,
        };
        Self::new(year, month, day)
    }

    /// The ISO 8601 week date: Python's `date.isocalendar()`.
    ///
    /// `date(2003, 12, 29).isocalendar()` is `(2004, 1, 1)`: the ISO year is
    /// not always the calendar year.
    #[must_use]
    pub fn iso_calendar(self) -> IsoWeekDate {
        match iso_week::from_fixed(self.fixed) {
            Ok((year, week, weekday)) => IsoWeekDate {
                year,
                week,
                weekday,
            },
            // Unreachable: every day in the Gregorian range has a week date.
            Err(_) => IsoWeekDate {
                year: self.year,
                week: 1,
                weekday: self.iso_weekday(),
            },
        }
    }

    /// The date of an ISO 8601 week date: Python's `date.fromisocalendar`.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the week does not exist in that year
    /// or the weekday is not 1 through 7.
    pub fn from_iso_calendar(year: i64, week: u8, weekday: u8) -> CalendarResult<Self> {
        Self::from_fixed(iso_week::to_fixed(year, week, weekday)?)
    }

    /// Add a span as Python adds a `timedelta` to a `date`: by its whole
    /// days, rounded towards negative infinity, ignoring the rest.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the result leaves the supported
    /// range.
    pub fn checked_add_delta(self, delta: TimeDelta) -> CalendarResult<Self> {
        let days = i64::try_from(delta.days()).map_err(|_| CalendarError::Overflow)?;
        self.add_days(days)
    }

    /// Subtract a span as Python does, `date + timedelta(-delta.days)`: by
    /// its whole days, rounded towards negative infinity, ignoring the rest.
    ///
    /// # Errors
    ///
    /// As [`Date::checked_add_delta`].
    pub fn checked_sub_delta(self, delta: TimeDelta) -> CalendarResult<Self> {
        let days = i64::try_from(delta.days()).map_err(|_| CalendarError::Overflow)?;
        self.add_days(days.checked_neg().ok_or(CalendarError::Overflow)?)
    }

    /// Move forward or backward by a whole number of days.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the result leaves the supported
    /// range.
    pub const fn add_days(self, days: i64) -> CalendarResult<Self> {
        match self.fixed.checked_add_days(days) {
            Ok(fixed) => Self::from_fixed(fixed),
            Err(error) => Err(error),
        }
    }

    /// Move by a number of months, clamping the day to the target month's
    /// length.
    ///
    /// Calendar month arithmetic has no answer that is right for everyone:
    /// 31 January plus one month is either 28 February or an error, and both
    /// conventions are in use. This method clamps, which is what spreadsheets
    /// and most business rules do; [`Date::replace`] errors, which is what a
    /// validation path wants. Neither is hidden behind the other.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the result leaves the supported
    /// range.
    pub const fn add_months(self, months: i64) -> CalendarResult<Self> {
        let total = self.year * 12 + (self.month as i64 - 1) + months;
        let year = total.div_euclid(12);
        let month = (total.rem_euclid(12) + 1) as u8;
        let limit = match gregorian::days_in_month(year, month) {
            Some(value) => value,
            None => return Err(CalendarError::MonthOutOfRange),
        };
        let day = if self.day > limit { limit } else { self.day };
        Self::new(year, month, day)
    }

    /// Move by a number of years, clamping 29 February to 28 February in a
    /// common year.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the result leaves the supported
    /// range.
    pub const fn add_years(self, years: i64) -> CalendarResult<Self> {
        self.add_months(years * 12)
    }

    /// The number of days from `earlier` to this date.
    #[must_use]
    pub const fn days_since(self, earlier: Self) -> i64 {
        self.fixed.0 - earlier.fixed.0
    }

    /// The span from `earlier` to this date, counting every day as 86 400
    /// seconds.
    #[must_use]
    pub const fn duration_since(self, earlier: Self) -> TimeDelta {
        TimeDelta(Duration::from_days(self.days_since(earlier)))
    }
}

impl fmt::Display for Date {
    /// Renders as ISO 8601 `YYYY-MM-DD`, with an explicit sign and expanded
    /// year outside the four-digit range as ISO 8601 requires.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if (0..=9_999).contains(&self.year) {
            write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
        } else {
            write!(f, "{:+06}-{:02}-{:02}", self.year, self.month, self.day)
        }
    }
}

/// A time of day.
///
/// A thin wrapper over [`CivilTime`] that keeps the leap-second capability
/// while presenting the accessors people expect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Time(CivilTime);

impl Time {
    /// Midnight.
    pub const MIDNIGHT: Self = Self(CivilTime::MIDNIGHT);

    /// Noon.
    pub const NOON: Self = Self(CivilTime::NOON);

    /// The earliest time: midnight. Python's `time.min`.
    pub const MIN: Self = Self(CivilTime::MIDNIGHT);

    /// The latest time: the last attosecond of a leap second, `23:59:60.999…`.
    ///
    /// Python's `time.max` is `23:59:59.999999`, because Python has no leap
    /// second.
    pub const MAX: Self = match CivilTime::new(23, 59, 60, ATTOS_PER_SEC - 1) {
        Ok(time) => Self(time),
        Err(_) => Self(CivilTime::MIDNIGHT),
    };

    /// The smallest difference between two times: one attosecond. Python's
    /// `time.resolution` is a microsecond.
    pub const RESOLUTION: TimeDelta = TimeDelta(Duration::from_attos(1));

    /// Build a time of day from Python's fields, the last in microseconds:
    /// `time(12, 30, 59, 250)`.
    ///
    /// # Errors
    ///
    /// As [`Time::new`].
    pub const fn from_hms_micro(
        hour: u8,
        minute: u8,
        second: u8,
        microsecond: u32,
    ) -> CalendarResult<Self> {
        if microsecond >= 1_000_000 {
            return Err(CalendarError::DayOutOfRange);
        }
        match CivilTime::new(hour, minute, second, microsecond as u64 * 1_000_000_000_000) {
            Ok(time) => Ok(Self(time)),
            Err(error) => Err(error),
        }
    }

    /// The same time with some fields replaced: Python's `time.replace`.
    /// A replaced microsecond replaces the whole sub-second part.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the result is not a time of day.
    pub const fn replace(
        self,
        hour: Option<u8>,
        minute: Option<u8>,
        second: Option<u8>,
        microsecond: Option<u32>,
    ) -> CalendarResult<Self> {
        let hour = match hour {
            Some(value) => value,
            None => self.hour(),
        };
        let minute = match minute {
            Some(value) => value,
            None => self.minute(),
        };
        let second = match second {
            Some(value) => value,
            None => self.second(),
        };
        let attos = match microsecond {
            Some(value) if value >= 1_000_000 => return Err(CalendarError::DayOutOfRange),
            Some(value) => value as u64 * 1_000_000_000_000,
            None => self.0.subsec_attos(),
        };
        match CivilTime::new(hour, minute, second, attos) {
            Ok(time) => Ok(Self(time)),
            Err(error) => Err(error),
        }
    }

    /// Build a time of day.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::DayOutOfRange`] when a field is out of range.
    /// A `second` of 60 is accepted only at `23:59`, where UTC really does
    /// put one.
    pub const fn new(hour: u8, minute: u8, second: u8, nanosecond: u32) -> CalendarResult<Self> {
        match CivilTime::new(hour, minute, second, nanosecond as u64 * 1_000_000_000) {
            Ok(time) => Ok(Self(time)),
            Err(error) => Err(error),
        }
    }

    /// Build a time of day from whole seconds.
    ///
    /// # Errors
    ///
    /// See [`Time::new`].
    pub const fn hms(hour: u8, minute: u8, second: u8) -> CalendarResult<Self> {
        Self::new(hour, minute, second, 0)
    }

    /// The hour, 0 through 23.
    #[must_use]
    pub const fn hour(self) -> u8 {
        self.0.hour()
    }

    /// The minute, 0 through 59.
    #[must_use]
    pub const fn minute(self) -> u8 {
        self.0.minute()
    }

    /// The second, 0 through 60.
    #[must_use]
    pub const fn second(self) -> u8 {
        self.0.second()
    }

    /// The sub-second part in nanoseconds, truncated.
    #[must_use]
    pub const fn nanosecond(self) -> u32 {
        (self.0.subsec_attos() / 1_000_000_000) as u32
    }

    /// The sub-second part in microseconds, truncated.
    ///
    /// This is Python's `time.microsecond`, which is the finest resolution
    /// that library has. This one goes to attoseconds; see
    /// [`Time::inner`].
    #[must_use]
    pub const fn microsecond(self) -> u32 {
        (self.0.subsec_attos() / 1_000_000_000_000) as u32
    }

    /// Whether this names an inserted leap second.
    #[must_use]
    pub const fn is_leap_second(self) -> bool {
        self.0.is_leap_second()
    }

    /// The elapsed time since midnight.
    #[must_use]
    pub const fn since_midnight(self) -> TimeDelta {
        TimeDelta(self.0.since_midnight())
    }

    /// The underlying [`CivilTime`], for full attosecond access.
    #[must_use]
    pub const fn inner(self) -> CivilTime {
        self.0
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<CivilTime> for Time {
    fn from(value: CivilTime) -> Self {
        Self(value)
    }
}

impl From<Time> for CivilTime {
    fn from(value: Time) -> Self {
        value.0
    }
}

/// A date and a time of day, with no time zone and no time scale.
///
/// This is a *local* wall-clock reading, the equivalent of Python's "naive"
/// datetime. Turning it into a physical instant needs a time zone and a
/// leap-second policy, and both are explicit arguments elsewhere — nothing
/// here silently assumes UTC.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTime {
    /// The date.
    pub date: Date,
    /// The time of day.
    pub time: Time,
}

/// The fields [`DateTime::replace`] may replace, as Python's keyword
/// arguments: `Replace { day: Some(26), ..Replace::default() }`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Replace {
    /// A new year.
    pub year: Option<i64>,
    /// A new month.
    pub month: Option<u8>,
    /// A new day of the month.
    pub day: Option<u8>,
    /// A new hour.
    pub hour: Option<u8>,
    /// A new minute.
    pub minute: Option<u8>,
    /// A new second.
    pub second: Option<u8>,
    /// A new sub-second part, in microseconds.
    pub microsecond: Option<u32>,
}

impl DateTime {
    /// The earliest reading: midnight on [`Date::MIN`].
    pub const MIN: Self = Self {
        date: Date::MIN,
        time: Time::MIN,
    };

    /// The latest reading: [`Time::MAX`] on [`Date::MAX`].
    pub const MAX: Self = Self {
        date: Date::MAX,
        time: Time::MAX,
    };

    /// The smallest difference between two readings: one attosecond.
    pub const RESOLUTION: TimeDelta = Time::RESOLUTION;

    /// Combine a date and a time.
    #[must_use]
    pub const fn new(date: Date, time: Time) -> Self {
        Self { date, time }
    }

    /// Combine a date and a time: Python's `datetime.combine`, the same
    /// function as [`DateTime::new`].
    #[must_use]
    pub const fn combine(date: Date, time: Time) -> Self {
        Self::new(date, time)
    }

    /// Midnight on the day with a given proleptic Gregorian ordinal:
    /// Python's `datetime.fromordinal`.
    ///
    /// # Errors
    ///
    /// See [`Date::from_fixed`].
    pub const fn from_ordinal(ordinal: i64) -> CalendarResult<Self> {
        match Date::from_ordinal(ordinal) {
            Ok(date) => Ok(Self::midnight(date)),
            Err(error) => Err(error),
        }
    }

    /// Midnight on an ISO 8601 week date: Python's
    /// `datetime.fromisocalendar`.
    ///
    /// # Errors
    ///
    /// See [`Date::from_iso_calendar`].
    pub fn from_iso_calendar(year: i64, week: u8, weekday: u8) -> CalendarResult<Self> {
        Ok(Self::midnight(Date::from_iso_calendar(
            year, week, weekday,
        )?))
    }

    /// The reading as the lower layer's type.
    #[must_use]
    pub const fn to_civil(self) -> CivilDateTime {
        CivilDateTime::new(self.date.fixed, self.time.0)
    }

    /// A reading from the lower layer's type.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the day is outside the Gregorian
    /// range.
    pub const fn from_civil(value: CivilDateTime) -> CalendarResult<Self> {
        match Date::from_fixed(value.day) {
            Ok(date) => Ok(Self::new(date, Time(value.time))),
            Err(error) => Err(error),
        }
    }

    /// The same reading with some fields replaced: Python's
    /// `datetime.replace`.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the result does not exist.
    pub const fn replace(self, fields: Replace) -> CalendarResult<Self> {
        let date = match self.date.replace(fields.year, fields.month, fields.day) {
            Ok(date) => date,
            Err(error) => return Err(error),
        };
        match self.time.replace(
            fields.hour,
            fields.minute,
            fields.second,
            fields.microsecond,
        ) {
            Ok(time) => Ok(Self::new(date, time)),
            Err(error) => Err(error),
        }
    }

    /// Add a span, counting every day as 86 400 seconds: Python's
    /// `datetime + timedelta`.
    ///
    /// A leap-second reading, `23:59:60`, is the 86 400th second of its day
    /// in this nominal arithmetic, so a non-zero span added to it counts
    /// from midnight of the next day; a zero span returns it unchanged.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the result leaves the supported
    /// range.
    pub fn checked_add_delta(self, delta: TimeDelta) -> CalendarResult<Self> {
        if delta.is_zero() {
            return Ok(self);
        }
        let offset = self.time.0.since_midnight().checked_add(delta.0)?;
        let (days, seconds, attos) = offset.days_seconds_attos();
        let days = i64::try_from(days).map_err(|_| CalendarError::Overflow)?;
        let date = self.date.add_days(days)?;
        let time = CivilTime::new(
            (seconds / 3_600) as u8,
            (seconds % 3_600 / 60) as u8,
            (seconds % 60) as u8,
            attos,
        )?;
        Ok(Self::new(date, Time(time)))
    }

    /// Subtract a span: Python's `datetime - timedelta`.
    ///
    /// # Errors
    ///
    /// As [`DateTime::checked_add_delta`].
    pub fn checked_sub_delta(self, delta: TimeDelta) -> CalendarResult<Self> {
        self.checked_add_delta(delta.checked_neg().map_err(CalendarError::from)?)
    }

    /// Midnight at the start of a date.
    #[must_use]
    pub const fn midnight(date: Date) -> Self {
        Self {
            date,
            time: Time::MIDNIGHT,
        }
    }

    /// Build from all the components at once.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the date or the time does not exist.
    pub const fn from_parts(
        year: i64,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
    ) -> CalendarResult<Self> {
        let date = match Date::new(year, month, day) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        let time = match Time::new(hour, minute, second, nanosecond) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        Ok(Self { date, time })
    }

    /// The span from `earlier` to this moment, counting every day as 86 400
    /// seconds.
    ///
    /// This is *nominal* elapsed time: it ignores leap seconds and any time
    /// zone transition, because a local wall-clock reading carries neither.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] when the span is unrepresentable.
    pub fn nominal_duration_since(self, earlier: Self) -> CalendarResult<TimeDelta> {
        let days = Duration::from_days(self.date.days_since(earlier.date));
        let span = days
            .checked_add(self.time.since_midnight().0)?
            .checked_sub(earlier.time.since_midnight().0)?;
        Ok(TimeDelta(span))
    }
}

impl fmt::Display for DateTime {
    /// Renders as ISO 8601 `YYYY-MM-DDTHH:MM:SS`, with no zone designator,
    /// because there is no zone.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}T{}", self.date, self.time)
    }
}

/// A span of time.
///
/// The equivalent of Python's `timedelta`, over [`hc_core::Duration`], so it
/// is exact to the attosecond rather than to the microsecond.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TimeDelta(Duration);

/// The keyword arguments of Python's `timedelta`, which it adds up and
/// normalises: `TimeDeltaParts { days: 50, seconds: 27, ..Default::default() }`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TimeDeltaParts {
    /// Seven-day weeks.
    pub weeks: i64,
    /// 86 400-second days.
    pub days: i64,
    /// Hours.
    pub hours: i64,
    /// Minutes.
    pub minutes: i64,
    /// Seconds.
    pub seconds: i64,
    /// Milliseconds.
    pub milliseconds: i64,
    /// Microseconds.
    pub microseconds: i64,
}

impl TimeDelta {
    /// The zero span.
    pub const ZERO: Self = Self(Duration::ZERO);

    /// The most negative span, about −5.4 × 10³⁰ years.
    ///
    /// Python's `timedelta.min` is −999 999 999 days; this range is wider.
    pub const MIN: Self = Self(Duration::MIN);

    /// The most positive span.
    pub const MAX: Self = Self(Duration::MAX);

    /// The smallest difference between two spans: one attosecond. Python's
    /// `timedelta.resolution` is a microsecond.
    pub const RESOLUTION: Self = Self(Duration::from_attos(1));

    /// Add up Python's keyword arguments into one span.
    ///
    /// Python's documentation: `timedelta(days=50, seconds=27,
    /// microseconds=10, milliseconds=29000, minutes=5, hours=8, weeks=2)` is
    /// `timedelta(days=64, seconds=29156, microseconds=10)`. No combination
    /// of `i64` arguments can overflow, so this cannot fail.
    #[must_use]
    pub const fn from_parts(parts: TimeDeltaParts) -> Self {
        let seconds = parts.weeks as i128 * 604_800
            + parts.days as i128 * 86_400
            + parts.hours as i128 * 3_600
            + parts.minutes as i128 * 60
            + parts.seconds as i128;
        let micros = parts.milliseconds as i128 * 1_000 + parts.microseconds as i128;
        Self(Duration::from_attos(
            seconds * ATTOS_PER_SEC as i128 + micros * 1_000_000_000_000,
        ))
    }

    /// A span of whole seven-day weeks.
    #[must_use]
    pub const fn from_weeks(weeks: i64) -> Self {
        Self(Duration::from_weeks(weeks))
    }

    /// Build a span from the components Python's `timedelta` takes.
    #[must_use]
    pub const fn new(days: i64, hours: i64, minutes: i64, seconds: i64) -> Self {
        Self(Duration::from_secs(
            days as i128 * 86_400 + hours as i128 * 3_600 + minutes as i128 * 60 + seconds as i128,
        ))
    }

    /// A span of whole days, each counted as 86 400 seconds.
    #[must_use]
    pub const fn from_days(days: i64) -> Self {
        Self(Duration::from_days(days))
    }

    /// A span of whole hours.
    #[must_use]
    pub const fn from_hours(hours: i64) -> Self {
        Self(Duration::from_hours(hours))
    }

    /// A span of whole minutes.
    #[must_use]
    pub const fn from_minutes(minutes: i64) -> Self {
        Self(Duration::from_minutes(minutes))
    }

    /// A span of whole seconds.
    #[must_use]
    pub const fn from_seconds(seconds: i64) -> Self {
        Self(Duration::from_secs(seconds as i128))
    }

    /// A span of milliseconds.
    #[must_use]
    pub const fn from_millis(millis: i64) -> Self {
        Self(Duration::from_millis(millis as i128))
    }

    /// A span of microseconds.
    #[must_use]
    pub const fn from_micros(micros: i64) -> Self {
        Self(Duration::from_micros(micros as i128))
    }

    /// A span of nanoseconds.
    #[must_use]
    pub const fn from_nanos(nanos: i64) -> Self {
        Self(Duration::from_nanos(nanos as i128))
    }

    /// The span in seconds as `f64`.
    ///
    /// This is Python's `timedelta.total_seconds`.
    #[must_use]
    pub fn total_seconds(self) -> f64 {
        self.0.as_secs_f64()
    }

    /// The whole days in the span, rounding towards negative infinity.
    #[must_use]
    pub const fn whole_days(self) -> i64 {
        (self.0.whole_seconds().div_euclid(86_400)) as i64
    }

    /// The underlying exact [`Duration`].
    #[must_use]
    pub const fn inner(self) -> Duration {
        self.0
    }

    /// Python's normalised `days`: whole days, rounded towards negative
    /// infinity, so that `seconds` and `microseconds` are never negative.
    #[must_use]
    pub const fn days(self) -> i128 {
        self.0.days_seconds_attos().0
    }

    /// Python's normalised `seconds`: the seconds after [`TimeDelta::days`],
    /// 0 through 86 399.
    #[must_use]
    pub const fn seconds(self) -> u32 {
        self.0.days_seconds_attos().1
    }

    /// Python's normalised `microseconds`: 0 through 999 999, truncated.
    #[must_use]
    pub const fn microseconds(self) -> u32 {
        (self.0.subsec_attos() / 1_000_000_000_000) as u32
    }

    /// Whether the span is zero: Python's `bool(timedelta)`, inverted.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.0.is_zero()
    }

    /// Negation that reports overflow instead of panicking.
    ///
    /// # Errors
    ///
    /// Returns [`hc_core::TimeError::Overflow`] for [`TimeDelta::MIN`].
    pub const fn checked_neg(self) -> TimeResult<Self> {
        match self.0.checked_neg() {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }

    /// The absolute value, reporting overflow instead of panicking.
    ///
    /// # Errors
    ///
    /// Returns [`hc_core::TimeError::Overflow`] for [`TimeDelta::MIN`].
    pub const fn checked_abs(self) -> TimeResult<Self> {
        match self.0.checked_abs() {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }

    /// The absolute value: Python's `abs(timedelta)`.
    ///
    /// # Panics
    ///
    /// Panics for [`TimeDelta::MIN`], whose magnitude is one attosecond too
    /// large. Use [`TimeDelta::checked_abs`] to handle it.
    #[must_use]
    pub const fn abs(self) -> Self {
        match self.checked_abs() {
            Ok(value) => value,
            Err(_) => panic!("hyper-calendar: time delta absolute value overflowed"),
        }
    }

    /// Multiplication by an integer: Python's `timedelta * int`.
    ///
    /// # Errors
    ///
    /// Returns [`hc_core::TimeError::Overflow`] when the result is
    /// unrepresentable.
    pub const fn checked_mul(self, factor: i64) -> TimeResult<Self> {
        match self.0.checked_mul_int(factor) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }

    /// Division by an integer, rounding towards negative infinity at the
    /// attosecond: Python's `timedelta // int`, and `timedelta / int` to
    /// within an attosecond where Python rounds to the microsecond.
    ///
    /// # Errors
    ///
    /// Returns [`hc_core::TimeError::DivideByZero`] for a zero divisor.
    pub const fn checked_div(self, divisor: i64) -> TimeResult<Self> {
        match self.0.checked_div_int(divisor) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }

    /// How many whole `divisor`s fit: Python's `timedelta // timedelta`.
    ///
    /// # Errors
    ///
    /// Returns [`hc_core::TimeError::DivideByZero`] for a zero divisor.
    pub const fn checked_div_floor(self, divisor: Self) -> TimeResult<i128> {
        self.0.checked_div_floor(divisor.0)
    }

    /// The remainder with the divisor's sign: Python's
    /// `timedelta % timedelta`.
    ///
    /// # Errors
    ///
    /// Returns [`hc_core::TimeError::DivideByZero`] for a zero divisor.
    pub const fn checked_rem(self, divisor: Self) -> TimeResult<Self> {
        match self.0.checked_rem(divisor.0) {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }

    /// Python's `divmod(timedelta, timedelta)`.
    ///
    /// # Errors
    ///
    /// Returns [`hc_core::TimeError::DivideByZero`] for a zero divisor.
    pub const fn checked_div_rem(self, divisor: Self) -> TimeResult<(i128, Self)> {
        match self.0.checked_div_rem(divisor.0) {
            Ok((quotient, remainder)) => Ok((quotient, Self(remainder))),
            Err(error) => Err(error),
        }
    }

    /// The ratio of two spans as `f64`: Python's `timedelta / timedelta`.
    ///
    /// # Errors
    ///
    /// Returns [`hc_core::TimeError::DivideByZero`] for a zero divisor.
    pub fn ratio(self, divisor: Self) -> TimeResult<f64> {
        self.0.ratio(divisor.0)
    }

    /// Scale by a real factor: Python's `timedelta * float`, to within what
    /// `f64` holds.
    ///
    /// # Errors
    ///
    /// Returns [`hc_core::TimeError::NotFinite`] or
    /// [`hc_core::TimeError::Overflow`] for a factor that leaves the range.
    pub fn checked_scale(self, factor: f64) -> TimeResult<Self> {
        Ok(Self(self.0.scale_f64(factor)?))
    }

    /// Addition that reports overflow instead of panicking.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] on overflow.
    pub const fn checked_add(self, other: Self) -> CalendarResult<Self> {
        match self.0.checked_add(other.0) {
            Ok(value) => Ok(Self(value)),
            Err(_) => Err(CalendarError::Overflow),
        }
    }

    /// Subtraction that reports overflow instead of panicking.
    ///
    /// # Errors
    ///
    /// Returns a [`CalendarError`] on overflow.
    pub const fn checked_sub(self, other: Self) -> CalendarResult<Self> {
        match self.0.checked_sub(other.0) {
            Ok(value) => Ok(Self(value)),
            Err(_) => Err(CalendarError::Overflow),
        }
    }
}

impl From<Duration> for TimeDelta {
    fn from(value: Duration) -> Self {
        Self(value)
    }
}

impl From<TimeDelta> for Duration {
    fn from(value: TimeDelta) -> Self {
        value.0
    }
}

impl fmt::Display for TimeDelta {
    /// Renders as Python's `str(timedelta)`: `-1 day, 19:00:00`,
    /// `0:00:00.000010`. See [`Duration::days_and_clock`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.days_and_clock().fmt(f)
    }
}

// --- operators -------------------------------------------------------------

macro_rules! panic_on_error {
    ($expression:expr, $what:literal) => {
        match $expression {
            Ok(value) => value,
            Err(_) => panic!(concat!("hyper-calendar: ", $what)),
        }
    };
}

impl Add for TimeDelta {
    type Output = Self;

    /// # Panics
    ///
    /// On overflow. Use [`TimeDelta::checked_add`].
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Sub for TimeDelta {
    type Output = Self;

    /// # Panics
    ///
    /// On overflow. Use [`TimeDelta::checked_sub`].
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl Neg for TimeDelta {
    type Output = Self;

    /// # Panics
    ///
    /// For [`TimeDelta::MIN`]. Use [`TimeDelta::checked_neg`].
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl AddAssign for TimeDelta {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for TimeDelta {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul<i64> for TimeDelta {
    type Output = Self;

    /// # Panics
    ///
    /// On overflow. Use [`TimeDelta::checked_mul`].
    fn mul(self, factor: i64) -> Self {
        Self(self.0 * factor)
    }
}

impl Mul<TimeDelta> for i64 {
    type Output = TimeDelta;

    /// # Panics
    ///
    /// On overflow. Use [`TimeDelta::checked_mul`].
    fn mul(self, delta: TimeDelta) -> TimeDelta {
        delta * self
    }
}

impl Div<i64> for TimeDelta {
    type Output = Self;

    /// # Panics
    ///
    /// On a zero divisor. Use [`TimeDelta::checked_div`].
    fn div(self, divisor: i64) -> Self {
        Self(self.0 / divisor)
    }
}

impl Rem for TimeDelta {
    type Output = Self;

    /// # Panics
    ///
    /// On a zero divisor. Use [`TimeDelta::checked_rem`].
    fn rem(self, divisor: Self) -> Self {
        Self(self.0 % divisor.0)
    }
}

impl Add<TimeDelta> for Date {
    type Output = Self;

    /// # Panics
    ///
    /// When the result leaves the supported range. Use
    /// [`Date::checked_add_delta`].
    fn add(self, delta: TimeDelta) -> Self {
        panic_on_error!(
            self.checked_add_delta(delta),
            "date addition left the supported range"
        )
    }
}

impl Sub<TimeDelta> for Date {
    type Output = Self;

    /// # Panics
    ///
    /// When the result leaves the supported range. Use
    /// [`Date::checked_sub_delta`].
    fn sub(self, delta: TimeDelta) -> Self {
        panic_on_error!(
            self.checked_sub_delta(delta),
            "date subtraction left the supported range"
        )
    }
}

impl Sub for Date {
    type Output = TimeDelta;

    /// The span between two dates, in whole days: Python's `date - date`.
    /// It cannot overflow: the whole Gregorian range is some 7 × 10⁹ days.
    fn sub(self, earlier: Self) -> TimeDelta {
        self.duration_since(earlier)
    }
}

impl Add<TimeDelta> for DateTime {
    type Output = Self;

    /// # Panics
    ///
    /// When the result leaves the supported range. Use
    /// [`DateTime::checked_add_delta`].
    fn add(self, delta: TimeDelta) -> Self {
        panic_on_error!(
            self.checked_add_delta(delta),
            "date-time addition left the supported range"
        )
    }
}

impl Sub<TimeDelta> for DateTime {
    type Output = Self;

    /// # Panics
    ///
    /// When the result leaves the supported range. Use
    /// [`DateTime::checked_sub_delta`].
    fn sub(self, delta: TimeDelta) -> Self {
        panic_on_error!(
            self.checked_sub_delta(delta),
            "date-time subtraction left the supported range"
        )
    }
}

impl Sub for DateTime {
    type Output = TimeDelta;

    /// The nominal span between two readings: Python's `datetime - datetime`
    /// for naive values. It cannot overflow over the Gregorian range.
    fn sub(self, earlier: Self) -> TimeDelta {
        panic_on_error!(
            self.nominal_duration_since(earlier),
            "date-time difference overflowed"
        )
    }
}

impl From<DateTime> for CivilDateTime {
    fn from(value: DateTime) -> Self {
        value.to_civil()
    }
}

impl TryFrom<CivilDateTime> for DateTime {
    type Error = CalendarError;

    fn try_from(value: CivilDateTime) -> CalendarResult<Self> {
        Self::from_civil(value)
    }
}

// --- ISO 8601 and patterns, from hc-format ---------------------------------

#[cfg(feature = "format")]
mod with_format {
    use core::fmt;

    use hc_format::patterns::{FormatContext, strftime};
    use hc_format::python::{self, TimeSpec};
    use hc_format::{ErrorKind, FormatResult, ParseError, ParseResult, ValueError, ZoneInfo};

    use super::{Date, DateTime, Time};
    use hc_calendar::{CivilDateTime, Rd};

    /// Why [`DateTime::strptime`] refused: the text did not match the
    /// pattern, or it matched and named no real reading.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum StrptimeError {
        /// The text does not match the pattern.
        Parse(ParseError),
        /// The fields read do not name a reading.
        Value(ValueError),
    }

    impl fmt::Display for StrptimeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Parse(error) => error.fmt(f),
                Self::Value(error) => error.fmt(f),
            }
        }
    }

    #[cfg(feature = "std")]
    impl std::error::Error for StrptimeError {}

    impl From<ParseError> for StrptimeError {
        fn from(error: ParseError) -> Self {
            Self::Parse(error)
        }
    }

    impl From<ValueError> for StrptimeError {
        fn from(error: ValueError) -> Self {
            Self::Value(error)
        }
    }

    fn in_range(day: Rd) -> ParseResult<Date> {
        Date::from_fixed(day).map_err(|_| ParseError::new(ErrorKind::OutOfRange("date"), 0))
    }

    /// Python's `time.strftime` gives a bare time the date 1900-01-01.
    const PYTHON_TIME_DATE: Rd = Rd(693_596);

    impl Date {
        /// Python's `date.fromisoformat`: `YYYY-MM-DD`, `YYYYMMDD`,
        /// `YYYY-Www-D` and `YYYYWwwD`. See [`hc_format::python`].
        ///
        /// # Errors
        ///
        /// A [`ParseError`] naming what was expected and where.
        pub fn from_iso_format(text: &str) -> ParseResult<Self> {
            in_range(python::parse_date(text)?)
        }

        /// Python's `date.strftime`, into a sink, with the C locale's names.
        ///
        /// # Errors
        ///
        /// As [`strftime::format`].
        pub fn write_strftime<W: fmt::Write>(self, out: &mut W, pattern: &str) -> FormatResult<()> {
            DateTime::midnight(self).write_strftime(out, pattern)
        }

        /// Python's `date.strftime`.
        ///
        /// # Errors
        ///
        /// As [`strftime::format`].
        #[cfg(feature = "alloc")]
        pub fn strftime(self, pattern: &str) -> FormatResult<alloc::string::String> {
            DateTime::midnight(self).strftime(pattern)
        }

        /// Python's `date.ctime`: `Wed Dec  4 00:00:00 2002`.
        ///
        /// # Errors
        ///
        /// As [`strftime::format`].
        #[cfg(feature = "alloc")]
        pub fn ctime(self) -> FormatResult<alloc::string::String> {
            DateTime::midnight(self).ctime()
        }
    }

    impl Time {
        /// Python's `time.fromisoformat`: the time, and the zone when the text
        /// wrote one — Python's `tzinfo`. See [`hc_format::python`].
        ///
        /// # Errors
        ///
        /// A [`ParseError`] naming what was expected and where.
        pub fn from_iso_format(text: &str) -> ParseResult<(Self, ZoneInfo)> {
            let (time, zone) = python::parse_time(text)?;
            Ok((Self(time), zone))
        }

        /// Python's `time.isoformat(timespec)`, into a sink.
        ///
        /// # Errors
        ///
        /// [`hc_format::FormatError::Sink`] when the sink refuses.
        pub fn write_iso_format<W: fmt::Write>(
            self,
            out: &mut W,
            spec: TimeSpec,
        ) -> FormatResult<()> {
            python::write_time(out, self.0, spec)
        }

        /// Python's `time.isoformat(timespec)`.
        ///
        /// # Errors
        ///
        /// As [`Time::write_iso_format`].
        #[cfg(feature = "alloc")]
        pub fn iso_format(self, spec: TimeSpec) -> FormatResult<alloc::string::String> {
            let mut out = alloc::string::String::new();
            self.write_iso_format(&mut out, spec)?;
            Ok(out)
        }

        /// Python's `time.strftime`, which dates a bare time 1900-01-01.
        ///
        /// # Errors
        ///
        /// As [`strftime::format`].
        pub fn write_strftime<W: fmt::Write>(self, out: &mut W, pattern: &str) -> FormatResult<()> {
            let reading = CivilDateTime::new(PYTHON_TIME_DATE, self.0);
            strftime::format(out, pattern, &FormatContext::new(reading))
        }

        /// Python's `time.strftime`.
        ///
        /// # Errors
        ///
        /// As [`strftime::format`].
        #[cfg(feature = "alloc")]
        pub fn strftime(self, pattern: &str) -> FormatResult<alloc::string::String> {
            let mut out = alloc::string::String::new();
            self.write_strftime(&mut out, pattern)?;
            Ok(out)
        }
    }

    impl DateTime {
        /// Python's `datetime.fromisoformat`: the reading, and the zone when
        /// the text wrote one — Python's `tzinfo`. See [`hc_format::python`].
        ///
        /// # Errors
        ///
        /// A [`ParseError`] naming what was expected and where.
        pub fn from_iso_format(text: &str) -> ParseResult<(Self, ZoneInfo)> {
            let value = python::parse_date_time(text)?;
            let date = in_range(value.local.day)?;
            Ok((Self::new(date, Time(value.local.time)), value.zone))
        }

        /// Python's `datetime.strptime(text, pattern)`: the reading, and the
        /// zone when `%z` or `%Z` read one. Fields the text leaves out come
        /// from 1900-01-01, as in Python.
        ///
        /// # Errors
        ///
        /// [`StrptimeError::Parse`] when the text does not match, and
        /// [`StrptimeError::Value`] when it names no real reading.
        pub fn strptime(text: &str, pattern: &str) -> Result<(Self, ZoneInfo), StrptimeError> {
            let value = python::strptime(text, pattern)?.to_offset_date_time()?;
            let date = Date::from_fixed(value.local.day)
                .map_err(|error| StrptimeError::Value(ValueError::Calendar(error)))?;
            Ok((Self::new(date, Time(value.local.time)), value.zone))
        }

        /// Python's `datetime.isoformat(sep, timespec)` for a naive reading,
        /// into a sink. An aware one is
        /// [`hc_format::python::write_date_time`] with its zone.
        ///
        /// # Errors
        ///
        /// [`hc_format::FormatError::Sink`] when the sink refuses.
        pub fn write_iso_format<W: fmt::Write>(
            self,
            out: &mut W,
            separator: char,
            spec: TimeSpec,
        ) -> FormatResult<()> {
            python::write_date_time(out, self.to_civil(), ZoneInfo::Unspecified, separator, spec)
        }

        /// Python's `datetime.isoformat(sep, timespec)` for a naive reading.
        ///
        /// # Errors
        ///
        /// As [`DateTime::write_iso_format`].
        #[cfg(feature = "alloc")]
        pub fn iso_format(
            self,
            separator: char,
            spec: TimeSpec,
        ) -> FormatResult<alloc::string::String> {
            let mut out = alloc::string::String::new();
            self.write_iso_format(&mut out, separator, spec)?;
            Ok(out)
        }

        /// Python's `datetime.strftime` for a naive reading, into a sink, with
        /// the C locale's names. For a zone or a locale, build an
        /// [`hc_format::patterns::FormatContext`] and call
        /// [`strftime::format`].
        ///
        /// # Errors
        ///
        /// As [`strftime::format`].
        pub fn write_strftime<W: fmt::Write>(self, out: &mut W, pattern: &str) -> FormatResult<()> {
            strftime::format(out, pattern, &FormatContext::new(self.to_civil()))
        }

        /// Python's `datetime.strftime` for a naive reading.
        ///
        /// # Errors
        ///
        /// As [`strftime::format`].
        #[cfg(feature = "alloc")]
        pub fn strftime(self, pattern: &str) -> FormatResult<alloc::string::String> {
            let mut out = alloc::string::String::new();
            self.write_strftime(&mut out, pattern)?;
            Ok(out)
        }

        /// Python's `datetime.ctime`: `Wed Dec  4 00:00:00 2002`.
        ///
        /// # Errors
        ///
        /// As [`strftime::format`].
        #[cfg(feature = "alloc")]
        pub fn ctime(self) -> FormatResult<alloc::string::String> {
            let mut out = alloc::string::String::new();
            python::write_ctime(&mut out, self.to_civil())?;
            Ok(out)
        }
    }
}

#[cfg(feature = "format")]
pub use with_format::StrptimeError;

#[cfg(feature = "format")]
pub use hc_format::python::TimeSpec;

// --- time zones, from hc-tz ------------------------------------------------

#[cfg(feature = "tz")]
mod with_tz {
    use hc_core::UnixTime;
    use hc_tz::{Disambiguation, TimeZone, TzResult, Utc, UtcOffset};

    use super::{Date, DateTime};

    impl Date {
        /// Python's `date.fromtimestamp(timestamp)`, with the zone named: the
        /// date the zone's clocks showed at that POSIX timestamp.
        ///
        /// # Errors
        ///
        /// A [`hc_tz::TzError`] when the date is outside the Gregorian range.
        pub fn from_timestamp<Z: TimeZone + ?Sized>(unix: UnixTime, zone: &Z) -> TzResult<Self> {
            Ok(DateTime::from_timestamp(unix, zone)?.date)
        }
    }

    impl DateTime {
        /// Python's `datetime.fromtimestamp(timestamp, tz)`, with the zone
        /// named rather than read from the machine: the wall-clock reading in
        /// `zone` at that POSIX timestamp.
        ///
        /// # Errors
        ///
        /// A [`hc_tz::TzError`] when the reading is outside the Gregorian
        /// range.
        pub fn from_timestamp<Z: TimeZone + ?Sized>(unix: UnixTime, zone: &Z) -> TzResult<Self> {
            Ok(Self::from_civil(zone.local_at(unix)?)?)
        }

        /// The UTC reading at a POSIX timestamp: Python's
        /// `datetime.fromtimestamp(timestamp, timezone.utc)` without the
        /// `tzinfo`, and what the deprecated `utcfromtimestamp` returned.
        ///
        /// # Errors
        ///
        /// As [`DateTime::from_timestamp`].
        pub fn from_timestamp_utc(unix: UnixTime) -> TzResult<Self> {
            Self::from_timestamp(unix, &Utc)
        }

        /// Python's `datetime.timestamp()`, with the zone named: the POSIX
        /// timestamp of this reading in `zone`.
        ///
        /// Python assumes a naive reading is local time and resolves a
        /// repeated or skipped reading by `fold`; here the zone and the
        /// resolution are arguments, and [`Disambiguation::Reject`] refuses to
        /// choose.
        ///
        /// # Errors
        ///
        /// [`hc_tz::TzError::AmbiguousLocalTime`] or
        /// [`hc_tz::TzError::NonexistentLocalTime`] under
        /// [`Disambiguation::Reject`], and [`hc_tz::TzError::Overflow`]
        /// outside the range of a POSIX timestamp.
        pub fn timestamp<Z: TimeZone + ?Sized>(
            self,
            zone: &Z,
            policy: Disambiguation,
        ) -> TzResult<UnixTime> {
            zone.unix_at(self.to_civil(), policy)
        }

        /// The POSIX timestamp of this reading taken as UTC: Python's
        /// `calendar.timegm(dt.utctimetuple())`, and `dt.replace(tzinfo=
        /// timezone.utc).timestamp()`.
        ///
        /// # Errors
        ///
        /// [`hc_tz::TzError::Overflow`] outside the range of a POSIX
        /// timestamp.
        pub fn timestamp_utc(self) -> TzResult<UnixTime> {
            self.timestamp(&Utc, Disambiguation::Reject)
        }

        /// Python's `datetime.astimezone(tz)`: the reading in `to` at the
        /// instant this reading names in `from`.
        ///
        /// # Errors
        ///
        /// As [`DateTime::timestamp`] and [`DateTime::from_timestamp`].
        pub fn astimezone<A: TimeZone + ?Sized, B: TimeZone + ?Sized>(
            self,
            from: &A,
            to: &B,
            policy: Disambiguation,
        ) -> TzResult<Self> {
            Self::from_timestamp(self.timestamp(from, policy)?, to)
        }

        /// Python's `datetime.utcoffset()` for this reading in `zone`.
        ///
        /// # Errors
        ///
        /// As [`DateTime::timestamp`].
        pub fn utc_offset<Z: TimeZone + ?Sized>(
            self,
            zone: &Z,
            policy: Disambiguation,
        ) -> TzResult<UtcOffset> {
            Ok(zone.offset_at(self.timestamp(zone, policy)?))
        }
    }
}
