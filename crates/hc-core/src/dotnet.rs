//! .NET's `DateTime.Ticks`: 100-nanosecond intervals from 0001-01-01, and
//! the `DateTimeKind` that says in which zone they count.
//!
//! Microsoft Learn, "DateTime.Ticks Property", read 2026-09-27
//! (`ms-datetime-ticks`): the value "represents the number of
//! 100-nanosecond intervals that have elapsed since 12:00:00 midnight,
//! January 1, 0001 in the Gregorian calendar", the
//! [`crate::epoch::DOTNET_TICKS`] epoch, and "does not include the number
//! of ticks that are attributable to leap seconds". "GregorianCalendar
//! Class" (`ms-gregoriancalendar`) gives the calendar's leap-year rule and
//! no reform date, so the count runs on the Gregorian rule applied to every
//! year from 1. "DateTime.MaxValue Field" (`ms-datetime-maxvalue`): the
//! largest value is 23:59:59.9999999 on 31 December 9999, whose ticks, as
//! its example prints, are 3 155 378 975 999 999 999, [`MAX_TICKS`].
//!
//! # What the Kind means
//!
//! "DateTimeKind Enum" (`ms-datetimekind`) has three values: `Unspecified`
//! (0), "not specified as either local time or Coordinated Universal Time",
//! `Utc` (1) and `Local` (2). The ticks property page: "the ticks represent
//! the time according to the time zone specified by the `Kind` property" —
//! a `Local` value counts the wall clock of "the current time zone setting"
//! of the machine that made it, and an `Unspecified` one "the unknown time
//! zone". So only a `Utc` value names an instant: [`DotnetDateTime::to_unix`]
//! answers for it and returns `None` for the other two, whose reading
//! [`DotnetDateTime::wall_clock`] gives in the POSIX shape for a caller that
//! knows the zone. The Kind is carried beside the ticks, not inside them:
//! `ToBinary` packs both into one `Int64`, and Microsoft's page for it says
//! the `Kind` is "concatenated to" the ticks but gives no bit positions, so
//! that form is not read or written here.
//!
//! Like POSIX time, the count cannot name 23:59:60: a leap second is
//! refused rather than folded into the next second.

use crate::error::{TimeError, TimeResult};
use crate::unix::{UnixTime, UtcInstant};

/// 100-nanosecond ticks in a second.
pub const TICKS_PER_SECOND: i64 = 10_000_000;

/// `DateTime.MaxValue.Ticks`, 23:59:59.9999999 on 9999-12-31.
pub const MAX_TICKS: i64 = 3_155_378_975_999_999_999;

/// The ticks of 1970-01-01T00:00:00: 719 162 days of 864 × 10⁹ ticks.
pub const UNIX_EPOCH_TICKS: i64 = 621_355_968_000_000_000;

/// Attoseconds in a tick.
const ATTOS_PER_TICK: u64 = 100_000_000_000;

/// What the ticks of a `DateTime` count, `System.DateTimeKind`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DateTimeKind {
    /// "Not specified as either local time or Coordinated Universal Time".
    Unspecified,
    /// "The time represented is UTC."
    Utc,
    /// "The time represented is local time", in the zone of the machine
    /// that made it.
    Local,
}

impl DateTimeKind {
    /// The enumeration's value: 0, 1 or 2.
    #[must_use]
    pub const fn value(self) -> u8 {
        match self {
            Self::Unspecified => 0,
            Self::Utc => 1,
            Self::Local => 2,
        }
    }

    /// The Kind with this value.
    #[must_use]
    pub const fn from_value(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Unspecified),
            1 => Some(Self::Utc),
            2 => Some(Self::Local),
            _ => None,
        }
    }
}

/// A .NET `DateTime`: its ticks and its Kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DotnetDateTime {
    ticks: i64,
    kind: DateTimeKind,
}

impl DotnetDateTime {
    /// A value from its ticks and Kind.
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] outside 0 to [`MAX_TICKS`], the range of
    /// `DateTime`.
    pub const fn new(ticks: i64, kind: DateTimeKind) -> TimeResult<Self> {
        if ticks < 0 || ticks > MAX_TICKS {
            return Err(TimeError::OutOfRange);
        }
        Ok(Self { ticks, kind })
    }

    /// The ticks.
    #[must_use]
    pub const fn ticks(self) -> i64 {
        self.ticks
    }

    /// The Kind.
    #[must_use]
    pub const fn kind(self) -> DateTimeKind {
        self.kind
    }

    /// The `Utc` value of a POSIX time, floored to the tick.
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] before 0001-01-01 or after 9999-12-31.
    pub fn from_unix(unix: UnixTime) -> TimeResult<Self> {
        Self::from_wall_clock(unix, DateTimeKind::Utc)
    }

    /// The `Utc` value of a UTC instant.
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] for a leap second, which the ticks do not
    /// count, and as [`DotnetDateTime::from_unix`].
    pub fn from_utc_instant(utc: UtcInstant) -> TimeResult<Self> {
        if utc.leap_second {
            return Err(TimeError::OutOfRange);
        }
        Self::from_unix(utc.to_unix_lossy())
    }

    /// A value of any Kind from a wall-clock reading written in the POSIX
    /// shape: seconds from 1970-01-01T00:00:00 of the zone the Kind names.
    ///
    /// # Errors
    ///
    /// As [`DotnetDateTime::from_unix`].
    pub fn from_wall_clock(reading: UnixTime, kind: DateTimeKind) -> TimeResult<Self> {
        let ticks = i128::from(reading.seconds()) * i128::from(TICKS_PER_SECOND)
            + i128::from(reading.subsec_attos() / ATTOS_PER_TICK)
            + i128::from(UNIX_EPOCH_TICKS);
        let ticks = i64::try_from(ticks).map_err(|_| TimeError::OutOfRange)?;
        Self::new(ticks, kind)
    }

    /// The reading the ticks count, in the POSIX shape, whatever the Kind:
    /// POSIX time for `Utc`, and for `Local` and `Unspecified` the wall
    /// clock of a zone the value does not name.
    #[must_use]
    pub fn wall_clock(self) -> UnixTime {
        UnixTime::from_nanos(i128::from(self.ticks - UNIX_EPOCH_TICKS) * 100)
    }

    /// The POSIX time of a `Utc` value, and `None` for `Local` and
    /// `Unspecified`, which do not say what zone they count in.
    #[must_use]
    pub fn to_unix(self) -> Option<UnixTime> {
        match self.kind {
            DateTimeKind::Utc => Some(self.wall_clock()),
            DateTimeKind::Local | DateTimeKind::Unspecified => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The ticks of a date at midnight: its days from 0001-01-01 times
    /// 864 × 10⁹.
    fn midnight(days_from_0001: i64) -> i64 {
        days_from_0001 * 86_400 * TICKS_PER_SECOND
    }

    /// 1970-01-01 is 719 162 days after 0001-01-01 on the Gregorian rule.
    #[test]
    fn the_posix_epoch_is_621_355_968_000_000_000_ticks() {
        assert_eq!(UNIX_EPOCH_TICKS, midnight(719_162));
        let utc = DotnetDateTime::from_unix(UnixTime::from_seconds(0)).expect("in range");
        assert_eq!(utc.ticks(), UNIX_EPOCH_TICKS);
        assert_eq!(utc.to_unix(), Some(UnixTime::from_seconds(0)));
        assert_eq!(
            DotnetDateTime::new(0, DateTimeKind::Utc)
                .expect("MinValue")
                .to_unix(),
            Some(UnixTime::from_seconds(
                crate::epoch::DOTNET_TICKS.tai_reading.whole_seconds() as i64
            ))
        );
    }

    /// `DateTime.MaxValue`, 23:59:59.9999999 UTC on 31 December 9999.
    #[test]
    fn max_value_is_the_last_tick_of_9999() {
        // 9999-12-31 is 2 932 896 days after 1970-01-01.
        let last =
            UnixTime::new(2_932_896 * 86_400 + 86_399, 999_999_900_000_000_000).expect("valid");
        let value = DotnetDateTime::from_unix(last).expect("in range");
        assert_eq!(value.ticks(), MAX_TICKS);
        assert_eq!(value.to_unix(), Some(last));
        let next = UnixTime::from_seconds(2_932_897 * 86_400);
        assert_eq!(DotnetDateTime::from_unix(next), Err(TimeError::OutOfRange));
        assert_eq!(
            DotnetDateTime::new(MAX_TICKS + 1, DateTimeKind::Local),
            Err(TimeError::OutOfRange)
        );
        assert_eq!(
            DotnetDateTime::new(-1, DateTimeKind::Utc),
            Err(TimeError::OutOfRange)
        );
    }

    /// The two examples on the Ticks page count from 2001-01-01: to
    /// 15:23 on 14 December 2007, 2 193 385 800 000 000 ticks, "2,538
    /// days, 15 hours, 23 minutes"; and to 18:21:38.171 on 14 November
    /// 2019, 5 954 484 981 710 000 ticks. Both are `DateTime.Now`, a local
    /// reading, so the difference is the same whatever the zone.
    #[test]
    fn the_examples_on_microsofts_page() {
        // 2001-01-01 is day 11 323 of the POSIX count, 2007-12-14 day
        // 13 861 and 2019-11-14 day 18 214.
        let reading = |day: i64, seconds: i64, attos: u64| {
            let unix = UnixTime::new(day * 86_400 + seconds, attos).expect("valid");
            DotnetDateTime::from_wall_clock(unix, DateTimeKind::Local).expect("in range")
        };
        let century = reading(11_323, 0, 0);
        let first = reading(13_861, 15 * 3_600 + 23 * 60, 0);
        assert_eq!(first.ticks() - century.ticks(), 2_193_385_800_000_000);
        assert_eq!(13_861 - 11_323, 2_538);
        let second = reading(18_214, 18 * 3_600 + 21 * 60 + 38, 171_000_000_000_000_000);
        assert_eq!(second.ticks() - century.ticks(), 5_954_484_981_710_000);
        // A local reading is not an instant.
        assert_eq!(first.to_unix(), None);
        assert_eq!(first.wall_clock().seconds(), 13_861 * 86_400 + 55_380);
    }

    /// A tick is 100 ns: anything finer is floored, and a leap second has
    /// no tick.
    #[test]
    fn a_tick_is_100_nanoseconds_and_a_leap_second_has_none() {
        let unix = UnixTime::new(1_483_228_799, 123_456_789_000_000_000).expect("valid");
        let value = DotnetDateTime::from_unix(unix).expect("in range");
        assert_eq!(value.ticks(), UNIX_EPOCH_TICKS + 14_832_287_991_234_567);
        assert_eq!(
            value.to_unix(),
            Some(UnixTime::new(1_483_228_799, 123_456_700_000_000_000).expect("valid"))
        );
        let leap = UtcInstant {
            unix_seconds: 1_483_228_800,
            leap_second: true,
            subsec_attos: 0,
        };
        assert_eq!(
            DotnetDateTime::from_utc_instant(leap),
            Err(TimeError::OutOfRange)
        );
        let after = UtcInstant::from_unix(UnixTime::from_seconds(1_483_228_800));
        assert!(DotnetDateTime::from_utc_instant(after).is_ok());
    }

    #[test]
    fn the_kinds_have_their_enumeration_values() {
        for kind in [
            DateTimeKind::Unspecified,
            DateTimeKind::Utc,
            DateTimeKind::Local,
        ] {
            assert_eq!(DateTimeKind::from_value(kind.value()), Some(kind));
        }
        assert_eq!(DateTimeKind::Utc.value(), 1);
        assert_eq!(DateTimeKind::from_value(3), None);
    }

    /// Every day's midnight from 0001 to 9999 converts and returns, on a
    /// sample of days in a debug build.
    #[test]
    fn midnights_round_trip() {
        let step = if cfg!(debug_assertions) { 997 } else { 1 };
        let first = -719_162_i64;
        let last = 2_932_896_i64;
        let mut day = first;
        while day <= last {
            let unix = UnixTime::from_seconds(day * 86_400);
            let value = DotnetDateTime::from_unix(unix).expect("in range");
            assert_eq!(value.ticks(), midnight(day - first));
            assert_eq!(value.to_unix(), Some(unix));
            day += step;
        }
    }
}
