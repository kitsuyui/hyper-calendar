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

use core::fmt;

use hc_calendar::{CalendarError, CalendarResult, CivilTime, Rd, Weekday};
use hc_calendars_solar::gregorian;
use hc_core::Duration;

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

impl DateTime {
    /// Combine a date and a time.
    #[must_use]
    pub const fn new(date: Date, time: Time) -> Self {
        Self { date, time }
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

impl TimeDelta {
    /// The zero span.
    pub const ZERO: Self = Self(Duration::ZERO);

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
