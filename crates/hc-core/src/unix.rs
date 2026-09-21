//! UTC, POSIX time and their relationship to TAI.
//!
//! POSIX time (`time_t`) counts *nominal* 86 400-second days since
//! `1970-01-01T00:00:00Z`. It therefore has no way to name an inserted leap
//! second: `23:59:60` and the `00:00:00` after it share a timestamp. This
//! module keeps the two ideas apart.
//!
//! * [`UnixTime`] is POSIX time, ambiguous during a leap second by design.
//! * [`UtcInstant`] is UTC with the ambiguity resolved by an explicit flag.
//!
//! Conversions to TAI go through [`crate::leap::TABLE`], so replacing the
//! table replaces the behaviour without touching this code.

use crate::duration::Duration;
use crate::error::{TimeError, TimeResult};
use crate::leap;
use crate::scale::{Instant, Tai};

/// POSIX time: seconds since `1970-01-01T00:00:00Z`, ignoring leap seconds.
///
/// This is what `time_t`, `Date.now()` and every database `TIMESTAMP` mean.
/// It is a *label*, not an elapsed-time count: two POSIX timestamps one
/// second apart are not necessarily one SI second apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct UnixTime {
    seconds: i64,
    attos: u64,
}

impl UnixTime {
    /// The POSIX epoch itself.
    pub const EPOCH: Self = Self {
        seconds: 0,
        attos: 0,
    };

    /// Build a POSIX timestamp from whole seconds.
    #[must_use]
    pub const fn from_seconds(seconds: i64) -> Self {
        Self { seconds, attos: 0 }
    }

    /// Build a POSIX timestamp from seconds and a sub-second remainder.
    ///
    /// # Errors
    ///
    /// Returns [`TimeError::OutOfRange`] when `attos` is not below 10¹⁸.
    pub const fn new(seconds: i64, attos: u64) -> TimeResult<Self> {
        if attos >= crate::duration::ATTOS_PER_SEC {
            return Err(TimeError::OutOfRange);
        }
        Ok(Self { seconds, attos })
    }

    /// Build a POSIX timestamp from milliseconds, as JavaScript reports it.
    #[must_use]
    pub const fn from_millis(millis: i64) -> Self {
        let seconds = millis.div_euclid(1_000);
        let remainder = millis.rem_euclid(1_000) as u64;
        Self {
            seconds,
            attos: remainder * 1_000_000_000_000_000,
        }
    }

    /// Build a POSIX timestamp from nanoseconds.
    #[must_use]
    pub const fn from_nanos(nanos: i128) -> Self {
        let seconds = nanos.div_euclid(1_000_000_000);
        let remainder = nanos.rem_euclid(1_000_000_000) as u64;
        Self {
            seconds: seconds as i64,
            attos: remainder * 1_000_000_000,
        }
    }

    /// The whole-second part.
    #[must_use]
    pub const fn seconds(self) -> i64 {
        self.seconds
    }

    /// The sub-second remainder in attoseconds.
    #[must_use]
    pub const fn subsec_attos(self) -> u64 {
        self.attos
    }

    /// The timestamp in milliseconds, truncated towards negative infinity.
    #[must_use]
    pub const fn as_millis(self) -> i64 {
        self.seconds * 1_000 + (self.attos / 1_000_000_000_000_000) as i64
    }

    /// The timestamp as a span from the POSIX epoch.
    #[must_use]
    pub const fn as_duration(self) -> Duration {
        Duration::from_attos(
            self.seconds as i128 * crate::duration::ATTOS_PER_SEC as i128 + self.attos as i128,
        )
    }
}

/// UTC with leap seconds representable.
///
/// `leap_second` marks the inserted `23:59:60`. When it is set, the value
/// denotes the extra second immediately *before* [`UtcInstant::unix_seconds`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UtcInstant {
    /// The POSIX timestamp of this second, or of the second that follows it
    /// when `leap_second` is set.
    pub unix_seconds: i64,
    /// Whether this is an inserted leap second.
    pub leap_second: bool,
    /// The sub-second remainder in attoseconds.
    pub subsec_attos: u64,
}

impl UtcInstant {
    /// An ordinary, non-leap UTC second.
    #[must_use]
    pub const fn from_unix(unix: UnixTime) -> Self {
        Self {
            unix_seconds: unix.seconds(),
            leap_second: false,
            subsec_attos: unix.subsec_attos(),
        }
    }

    /// The POSIX timestamp this instant collapses to.
    ///
    /// A leap second and the second after it give the same answer; that loss
    /// is what POSIX time is.
    #[must_use]
    pub const fn to_unix_lossy(self) -> UnixTime {
        UnixTime {
            seconds: self.unix_seconds,
            attos: self.subsec_attos,
        }
    }
}

/// How to treat instants outside the published leap-second data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LeapPolicy {
    /// Refuse to answer outside the authoritative range. The default, because
    /// a forecast leap second is a guess and callers should know when they
    /// are getting one.
    #[default]
    Strict,
    /// Hold the last published `TAI - UTC` constant into the future and use
    /// the 1961-1971 rate formulas in the past.
    ///
    /// Before 1961 this extends `TAI - UTC = 0`, which is a convention, not a
    /// fact: UTC did not exist.
    Extrapolate,
}

/// `TAI - UTC` at a POSIX timestamp.
///
/// # Errors
///
/// Returns [`TimeError::BeforeModelStart`] before 1961 and
/// [`TimeError::AfterModelEnd`] past the announced validity of the table,
/// unless `policy` is [`LeapPolicy::Extrapolate`].
pub fn tai_minus_utc_at(unix_seconds: i64, policy: LeapPolicy) -> TimeResult<Duration> {
    if unix_seconds >= leap::integer_era_start_unix() {
        if unix_seconds > leap::table_valid_until_unix() && policy == LeapPolicy::Strict {
            return Err(TimeError::AfterModelEnd);
        }
        let index = leap::TABLE.partition_point(|entry| entry.start_unix <= unix_seconds);
        let entry = leap::TABLE[index - 1];
        return Ok(Duration::from_secs(entry.tai_minus_utc as i128));
    }

    if unix_seconds >= leap::utc_start_unix() {
        let index = leap::RATE_ERA.partition_point(|entry| entry.start_unix <= unix_seconds);
        let entry = leap::RATE_ERA[index - 1];
        // MJD of the POSIX epoch is 40587.
        let mjd = 40_587.0 + unix_seconds as f64 / 86_400.0;
        let offset = entry.offset + (mjd - entry.origin_mjd) * entry.drift;
        return Duration::from_secs_f64(offset);
    }

    match policy {
        LeapPolicy::Strict => Err(TimeError::BeforeModelStart),
        LeapPolicy::Extrapolate => Ok(Duration::ZERO),
    }
}

/// Convert a UTC instant to its TAI reading.
///
/// # Errors
///
/// See [`tai_minus_utc_at`].
pub fn tai_from_utc(utc: UtcInstant, policy: LeapPolicy) -> TimeResult<Instant<Tai>> {
    let offset = tai_minus_utc_at(utc.unix_seconds, policy)?;
    let base = Duration::from_attos(
        utc.unix_seconds as i128 * crate::duration::ATTOS_PER_SEC as i128
            + utc.subsec_attos as i128,
    );
    let mut reading = base.checked_add(offset)?;
    if utc.leap_second {
        reading = reading.checked_sub(Duration::SECOND)?;
    }
    Ok(Instant::from_epoch(reading))
}

/// Convert a TAI reading to UTC, naming an inserted leap second when the
/// instant falls inside one.
///
/// # Errors
///
/// See [`tai_minus_utc_at`].
pub fn utc_from_tai(tai: Instant<Tai>, policy: LeapPolicy) -> TimeResult<UtcInstant> {
    let reading = tai.since_epoch();
    let tai_secs = reading.whole_seconds();
    let subsec = reading.subsec_attos();

    // Locate the leap-table segment by comparing against each segment's TAI
    // start, which is `start_unix + tai_minus_utc`.
    let first_integer_tai =
        leap::TABLE[0].start_unix as i128 + leap::TABLE[0].tai_minus_utc as i128;
    if tai_secs >= first_integer_tai {
        let index = leap::TABLE.partition_point(|entry| {
            (entry.start_unix as i128 + entry.tai_minus_utc as i128) <= tai_secs
        });
        let entry = leap::TABLE[index - 1];
        let unix_seconds = tai_secs - entry.tai_minus_utc as i128;
        if unix_seconds > leap::table_valid_until_unix() as i128 && policy == LeapPolicy::Strict {
            return Err(TimeError::AfterModelEnd);
        }
        // An inserted second occupies the TAI range
        // `[next.tai_start - 1, next.tai_start)`.
        if let Some(next) = leap::TABLE.get(index) {
            let next_tai_start = next.start_unix as i128 + next.tai_minus_utc as i128;
            let gap = next.tai_minus_utc - entry.tai_minus_utc;
            if gap > 0 && tai_secs >= next_tai_start - gap as i128 {
                return Ok(UtcInstant {
                    unix_seconds: next.start_unix,
                    leap_second: true,
                    subsec_attos: subsec,
                });
            }
        }
        let unix_seconds = i64::try_from(unix_seconds).map_err(|_| TimeError::Overflow)?;
        return Ok(UtcInstant {
            unix_seconds,
            leap_second: false,
            subsec_attos: subsec,
        });
    }

    // Rate era and earlier: the offset is a smooth function, so solve by
    // fixed-point iteration. Two rounds are ample for a sub-second drift.
    let mut unix_guess = tai_secs;
    for _ in 0..3 {
        let offset = tai_minus_utc_at(
            i64::try_from(unix_guess).map_err(|_| TimeError::Overflow)?,
            policy,
        )?;
        unix_guess = tai_secs - offset.whole_seconds();
    }
    let unix_seconds = i64::try_from(unix_guess).map_err(|_| TimeError::Overflow)?;
    Ok(UtcInstant {
        unix_seconds,
        leap_second: false,
        subsec_attos: subsec,
    })
}

/// Convert POSIX time to TAI, treating the timestamp as a non-leap second.
///
/// # Errors
///
/// See [`tai_minus_utc_at`].
pub fn tai_from_unix(unix: UnixTime, policy: LeapPolicy) -> TimeResult<Instant<Tai>> {
    tai_from_utc(UtcInstant::from_unix(unix), policy)
}

/// Convert TAI to POSIX time, collapsing any leap second onto the following
/// timestamp.
///
/// # Errors
///
/// See [`tai_minus_utc_at`].
pub fn unix_from_tai(tai: Instant<Tai>, policy: LeapPolicy) -> TimeResult<UnixTime> {
    Ok(utc_from_tai(tai, policy)?.to_unix_lossy())
}

#[cfg(test)]
mod tests {
    use super::*;

    const STRICT: LeapPolicy = LeapPolicy::Strict;

    #[test]
    fn the_offset_is_ten_seconds_at_the_start_of_the_integer_era() {
        let offset = tai_minus_utc_at(63_072_000, STRICT).unwrap();
        assert_eq!(offset, Duration::from_secs(10));
    }

    #[test]
    fn the_offset_is_thirty_seven_seconds_today() {
        let offset = tai_minus_utc_at(1_700_000_000, STRICT).unwrap();
        assert_eq!(offset, Duration::from_secs(37));
    }

    #[test]
    fn the_offset_steps_exactly_at_midnight() {
        assert_eq!(
            tai_minus_utc_at(1_483_228_799, STRICT).unwrap(),
            Duration::from_secs(36)
        );
        assert_eq!(
            tai_minus_utc_at(1_483_228_800, STRICT).unwrap(),
            Duration::from_secs(37)
        );
    }

    #[test]
    fn utc_before_nineteen_sixty_one_is_refused_by_default() {
        assert_eq!(
            tai_minus_utc_at(-400_000_000, STRICT),
            Err(TimeError::BeforeModelStart)
        );
        assert_eq!(
            tai_minus_utc_at(-400_000_000, LeapPolicy::Extrapolate),
            Ok(Duration::ZERO)
        );
    }

    #[test]
    fn the_rate_era_produces_a_fractional_offset() {
        // 1970-01-01T00:00:00Z sits in the final rate segment and the
        // published TAI - UTC there is 8.000082 s.
        let offset = tai_minus_utc_at(0, STRICT).unwrap();
        assert!(
            (offset.as_secs_f64() - 8.000_082).abs() < 1e-6,
            "offset was {offset}"
        );
    }

    #[test]
    fn utc_and_tai_round_trip_across_the_integer_era() {
        for unix in [
            63_072_000i64,
            100_000_000,
            915_148_800,
            1_483_228_800,
            1_700_000_000,
        ] {
            let utc = UtcInstant::from_unix(UnixTime::from_seconds(unix));
            let tai = tai_from_utc(utc, STRICT).unwrap();
            assert_eq!(utc_from_tai(tai, STRICT).unwrap(), utc, "unix {unix}");
        }
    }

    #[test]
    fn the_inserted_second_is_nameable_and_round_trips() {
        // 2016-12-31T23:59:60Z precedes the 2017-01-01 step.
        let leap_instant = UtcInstant {
            unix_seconds: 1_483_228_800,
            leap_second: true,
            subsec_attos: 0,
        };
        let tai = tai_from_utc(leap_instant, STRICT).unwrap();
        let back = utc_from_tai(tai, STRICT).unwrap();
        assert_eq!(back, leap_instant);
        // It is exactly one second before the first second of 2017.
        let after = tai_from_utc(
            UtcInstant::from_unix(UnixTime::from_seconds(1_483_228_800)),
            STRICT,
        )
        .unwrap();
        assert_eq!(after.duration_since(tai).unwrap(), Duration::SECOND);
    }

    #[test]
    fn posix_time_collapses_the_leap_second() {
        let leap_instant = UtcInstant {
            unix_seconds: 1_483_228_800,
            leap_second: true,
            subsec_attos: 0,
        };
        assert_eq!(
            leap_instant.to_unix_lossy(),
            UnixTime::from_seconds(1_483_228_800)
        );
    }

    #[test]
    fn a_leap_second_makes_the_utc_day_last_86401_seconds() {
        let day_start = tai_from_unix(UnixTime::from_seconds(1_483_142_400), STRICT).unwrap();
        let next_day = tai_from_unix(UnixTime::from_seconds(1_483_228_800), STRICT).unwrap();
        assert_eq!(
            next_day.duration_since(day_start).unwrap(),
            Duration::from_secs(86_401)
        );
    }

    #[test]
    fn future_instants_are_refused_under_the_strict_policy() {
        // Far past any announced Bulletin C.
        assert_eq!(
            tai_minus_utc_at(4_000_000_000, STRICT),
            Err(TimeError::AfterModelEnd)
        );
        assert!(tai_minus_utc_at(4_000_000_000, LeapPolicy::Extrapolate).is_ok());
    }

    #[test]
    fn milliseconds_round_trip() {
        let value = UnixTime::from_millis(-1_500);
        assert_eq!(value.seconds(), -2);
        assert_eq!(value.as_millis(), -1_500);
    }
}
