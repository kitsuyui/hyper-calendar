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

/// The attoseconds of one second, as the `i128` the rate-era arithmetic uses.
const ATTOS: i128 = crate::duration::ATTOS_PER_SEC as i128;

/// The Modified Julian Date of the POSIX epoch, 1970-01-01.
const MJD_OF_UNIX_EPOCH: i64 = 40_587;

/// The divisor of a drift: a drift is published in units of 10⁻⁷ s per day,
/// and a day is 86 400 s.
const DRIFT_DIVISOR: i128 = 86_400 * 10_000_000;

/// A [`leap::RateEntry`] with every coefficient as an exact integer.
///
/// The coefficients of `tai-utc.dat` are decimals of at most seven places,
/// and the three drifts, 0.001 296, 0.001 123 2 and 0.002 592 s per day, are
/// exactly 1.5·10⁻⁸, 1.3·10⁻⁸ and 3·10⁻⁸ s per second. In attoseconds, with
/// the drift in 10⁻⁷ s per day, `TAI − UTC` is an integer for every whole
/// second, so the 1961-1971 relation is evaluated and inverted without a
/// floating-point operation.
#[derive(Clone, Copy)]
struct ExactRate {
    /// The UTC reading of the segment's origin, in attoseconds from the epoch.
    origin: i128,
    /// The constant term, in attoseconds.
    offset: i128,
    /// The drift, in units of 10⁻⁷ s per day.
    drift: i128,
}

impl ExactRate {
    fn of(entry: &leap::RateEntry) -> Self {
        let origin_days = entry.origin_mjd as i64 - MJD_OF_UNIX_EPOCH;
        Self {
            origin: i128::from(origin_days) * 86_400 * ATTOS,
            offset: crate::math::round(entry.offset * 1e7) as i128 * (ATTOS / 10_000_000),
            drift: crate::math::round(entry.drift * 1e7) as i128,
        }
    }

    /// `TAI − UTC` in attoseconds at a UTC reading in attoseconds from the
    /// epoch.
    fn offset_at(self, utc: i128) -> i128 {
        self.offset + div_round((utc - self.origin) * self.drift, DRIFT_DIVISOR)
    }

    /// The UTC reading, in attoseconds, whose TAI reading is `tai`.
    ///
    /// With `n = utc − origin` the relation is `tai = origin + offset + n +
    /// n·drift/D`, so `n = x·D/(D + drift)` for `x = tai − origin − offset`,
    /// which is `x − x·drift/(D + drift)`; the product is formed before
    /// the division and the division is the only rounding.
    fn utc_at(self, tai: i128) -> i128 {
        let x = tai - self.origin - self.offset;
        self.origin + x - div_round(x * self.drift, DRIFT_DIVISOR + self.drift)
    }
}

/// `a / b` for `b > 0`, rounded to the nearest integer, halves upwards.
const fn div_round(a: i128, b: i128) -> i128 {
    (2 * a + b).div_euclid(2 * b)
}

/// The rate-era segment in force at a UTC reading in whole Unix seconds.
fn rate_segment_at(unix_seconds: i64) -> &'static leap::RateEntry {
    let index = leap::RATE_ERA.partition_point(|entry| entry.start_unix <= unix_seconds);
    &leap::RATE_ERA[index - 1]
}

/// `TAI - UTC` at a UTC reading, whole seconds and attoseconds of a POSIX
/// timestamp.
///
/// From 1972 it is a whole number of seconds and the sub-second part makes
/// no difference. From 1961 to 1971 it is the published straight line of
/// the segment, evaluated exactly at the instant, since it changes by
/// 1.5·10⁻⁸ s in each second.
fn offset_at(unix_seconds: i64, subsec_attos: u64, policy: LeapPolicy) -> TimeResult<Duration> {
    if unix_seconds >= leap::integer_era_start_unix() {
        if unix_seconds > leap::table_valid_until_unix() && policy == LeapPolicy::Strict {
            return Err(TimeError::AfterModelEnd);
        }
        let index = leap::TABLE.partition_point(|entry| entry.start_unix <= unix_seconds);
        let entry = leap::TABLE[index - 1];
        return Ok(Duration::from_secs(entry.tai_minus_utc as i128));
    }

    if unix_seconds >= leap::utc_start_unix() {
        let utc = i128::from(unix_seconds) * ATTOS + i128::from(subsec_attos);
        let exact = ExactRate::of(rate_segment_at(unix_seconds));
        return Ok(Duration::from_attos(exact.offset_at(utc)));
    }

    match policy {
        LeapPolicy::Strict => Err(TimeError::BeforeModelStart),
        LeapPolicy::Extrapolate => Ok(Duration::ZERO),
    }
}

/// `TAI - UTC` at a POSIX timestamp.
///
/// From 1972 the answer is a whole number of seconds. From 1961 to 1971 it
/// is not: it is the value the USNO `tai-utc.dat` and the IERS give for the
/// UTC reading, to the attosecond (see [`leap::RATE_ERA`]), and the
/// whole-second part of it is not the number of leap seconds, of which there
/// were none.
///
/// # Errors
///
/// Returns [`TimeError::BeforeModelStart`] before 1961 and
/// [`TimeError::AfterModelEnd`] past the announced validity of the table,
/// unless `policy` is [`LeapPolicy::Extrapolate`].
pub fn tai_minus_utc_at(unix_seconds: i64, policy: LeapPolicy) -> TimeResult<Duration> {
    offset_at(unix_seconds, 0, policy)
}

/// `TAI - UTC` at a POSIX instant, with its sub-second part.
///
/// From 1972 this is [`tai_minus_utc_at`]. From 1961 to 1971 the offset
/// moves by 1.5·10⁻⁸ s in a second or less (see [`leap::RATE_ERA`]), so it
/// is a function of the attoseconds too, and the whole-second form reads it
/// at the start of the second.
///
/// # Errors
///
/// See [`tai_minus_utc_at`].
pub fn tai_minus_utc_at_instant(unix: UnixTime, policy: LeapPolicy) -> TimeResult<Duration> {
    offset_at(unix.seconds(), unix.subsec_attos(), policy)
}

/// Convert a UTC instant to its TAI reading.
///
/// # Errors
///
/// See [`tai_minus_utc_at`].
pub fn tai_from_utc(utc: UtcInstant, policy: LeapPolicy) -> TimeResult<Instant<Tai>> {
    let offset = offset_at(utc.unix_seconds, utc.subsec_attos, policy)?;
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

/// The TAI reading of the first UTC instant of a rate-era segment.
fn rate_segment_tai_start(entry: &leap::RateEntry) -> i128 {
    let start = i128::from(entry.start_unix) * ATTOS;
    start + ExactRate::of(entry).offset_at(start)
}

/// Convert a TAI reading to UTC, naming an inserted leap second when the
/// instant falls inside one.
///
/// From 1961 to 1971 the inversion is exact, with the fraction of
/// `TAI − UTC` kept: the UTC reading is a whole second and a fraction of
/// one. The relation is a step function of segments, and the steps of 0.05
/// to 0.1 s (and 0.107 758 s at 1972-01-01) are not leap seconds, so a TAI
/// instant near one of them needs a rule:
///
/// * a segment starts where its first UTC reading is, and the last segment
///   whose start is not after the instant is used, so where TAI − UTC
///   steps down (1961-08-01, 1968-02-01) the instants of the overlap are
///   read with the new segment; the UTC readings of the 0.05 or 0.1 s
///   before the step, which the old segment gave them, do not round-trip;
/// * where TAI − UTC steps up (every other step) the instants between the
///   old segment's last reading and the new segment's first have no UTC
///   reading at all, and are given the UTC reading of the step, so that UTC
///   never goes backwards as TAI goes forwards.
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

    // Before the first segment's TAI start no UTC reading exists.
    let first_rate = &leap::RATE_ERA[0];
    let first_rate_tai = rate_segment_tai_start(first_rate);
    if reading < Duration::from_attos(first_rate_tai) {
        if policy == LeapPolicy::Strict {
            return Err(TimeError::BeforeModelStart);
        }
        // The convention of `LeapPolicy::Extrapolate`: TAI − UTC = 0.
        let unix_seconds = i64::try_from(tai_secs).map_err(|_| TimeError::Overflow)?;
        return Ok(UtcInstant {
            unix_seconds,
            leap_second: false,
            subsec_attos: subsec,
        });
    }

    // The rate era: invert the straight line of the segment in force.
    let tai_attos = tai_secs * ATTOS + i128::from(subsec);
    let index = leap::RATE_ERA.partition_point(|entry| rate_segment_tai_start(entry) <= tai_attos);
    let entry = &leap::RATE_ERA[index - 1];
    let next_start = leap::RATE_ERA
        .get(index)
        .map_or(leap::integer_era_start_unix(), |next| next.start_unix);
    let utc = ExactRate::of(entry)
        .utc_at(tai_attos)
        .min(i128::from(next_start) * ATTOS);
    Ok(UtcInstant {
        unix_seconds: i64::try_from(utc.div_euclid(ATTOS)).map_err(|_| TimeError::Overflow)?,
        leap_second: false,
        subsec_attos: utc.rem_euclid(ATTOS) as u64,
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    extern crate alloc;

    const STRICT: LeapPolicy = LeapPolicy::Strict;

    #[test]
    fn the_offset_is_ten_seconds_at_the_start_of_the_integer_era() {
        let offset = tai_minus_utc_at(63_072_000, STRICT).unwrap();
        assert_eq!(offset, Duration::from_secs(10));
    }

    /// The USNO `tai-utc.dat` line in force from 1968-02-01 to 1971-12-31 is
    /// `4.2131700 s + (MJD − 39126) × 0.002592 s`, which the table of
    /// `leap::RATE_ERA` carries. 1970-01-01 is MJD 40587: 4.213 17 s +
    /// 1461 × 0.002 592 s = 8.000 082 s. The drift is 3·10⁻⁸ s in a second, so
    /// half a second later it is 15 ns more, and the last second of 1971,
    /// MJD 40587 + 63 071 999/86 400, has 4.213 17 s + 189 302 399 × 3·10⁻⁸ s
    /// = 9.892 241 97 s.
    #[test]
    fn the_offset_in_the_rate_era_depends_on_the_attoseconds() {
        let at = |seconds, attos| {
            tai_minus_utc_at_instant(UnixTime::new(seconds, attos).unwrap(), STRICT).unwrap()
        };
        assert_eq!(at(0, 0), Duration::from_attos(8_000_082_000_000_000_000));
        assert_eq!(
            at(0, 500_000_000_000_000_000),
            Duration::from_attos(8_000_082_015_000_000_000)
        );
        assert_eq!(
            at(63_071_999, 0),
            Duration::from_attos(9_892_241_970_000_000_000)
        );
        // The whole-second form is the offset at the start of the second.
        assert_eq!(tai_minus_utc_at(0, STRICT).unwrap(), at(0, 0));
        // From 1972 it is a whole number of seconds whatever the attoseconds.
        assert_eq!(
            at(1_700_000_000, 123_000_000_000_000_000),
            Duration::from_secs(37)
        );
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

    /// `TAI − UTC` in attoseconds at the first second of each rate-era
    /// segment, and at the last second before it, computed with exact
    /// rational arithmetic (`scripts/rate-era-pins.py`, Python `fractions`) from the
    /// coefficients of the USNO `tai-utc.dat`, read 2026-10-03 at
    /// <https://maia.usno.navy.mil/ser7/tai-utc.dat>. The steps show: 1961-08
    /// −0.05 s, 1963-11 +0.1 s, 1966-01 and 1962-01 and 1964-01 continuous,
    /// 1968-02 −0.1 s.
    const OFFSETS: [(i64, i128); 25] = [
        (-283_996_800, 1_422_818_000_000_000_000),
        (-265_680_001, 1_697_569_985_000_000_000),
        (-265_680_000, 1_647_570_000_000_000_000),
        (-252_460_801, 1_845_857_985_000_000_000),
        (-252_460_800, 1_845_858_000_000_000_000),
        (-194_659_201, 2_597_278_787_000_000_000),
        (-194_659_200, 2_697_278_800_000_000_000),
        (-189_388_801, 2_765_793_987_000_000_000),
        (-189_388_800, 2_765_794_000_000_000_000),
        (-181_526_401, 2_883_729_985_000_000_000),
        (-181_526_400, 2_983_730_000_000_000_000),
        (-168_307_201, 3_182_017_985_000_000_000),
        (-168_307_200, 3_282_018_000_000_000_000),
        (-157_766_401, 3_440_129_985_000_000_000),
        (-157_766_400, 3_540_130_000_000_000_000),
        (-152_668_801, 3_616_593_985_000_000_000),
        (-152_668_800, 3_716_594_000_000_000_000),
        (-142_128_001, 3_874_705_985_000_000_000),
        (-142_128_000, 3_974_706_000_000_000_000),
        (-136_771_201, 4_055_057_985_000_000_000),
        (-136_771_200, 4_155_058_000_000_000_000),
        (-126_230_401, 4_313_169_985_000_000_000),
        (-126_230_400, 4_313_170_000_000_000_000),
        (-60_480_001, 6_285_681_970_000_000_000),
        (63_071_999, 9_892_241_970_000_000_000),
    ];

    /// The offset at the edges of every segment is the one the
    /// published coefficients give, to the attosecond.
    #[test]
    fn the_rate_era_offsets_at_every_segment_edge_are_exact() {
        for (unix, attos) in OFFSETS {
            assert_eq!(
                tai_minus_utc_at(unix, STRICT).unwrap(),
                Duration::from_attos(attos),
                "unix {unix}"
            );
        }
        // The 1968 segment's last second is the last second before the
        // integer era, and the 1972 step is 10 − 9.89224197 s.
        assert_eq!(
            tai_minus_utc_at(63_072_000, STRICT).unwrap() - Duration::from_attos(OFFSETS[24].1),
            Duration::from_attos(107_758_030_000_000_000)
        );
    }

    /// Every one of the 4 017 days of 1961-1971 is read with
    /// the segment the published tables give for it. The expectation is
    /// written in the test from the table's own Julian Dates and
    /// `offset + (MJD − origin) × drift`, in floating point, so the day of a
    /// segment start cannot be taken from the code under test.
    #[test]
    fn every_day_of_1961_to_1971_uses_the_segment_of_its_published_date() {
        // (start MJD, offset, origin MJD, drift) of the USNO table.
        const TABLE: [(i64, f64, f64, f64); 13] = [
            (37_300, 1.422_818_0, 37_300.0, 0.001_296),
            (37_512, 1.372_818_0, 37_300.0, 0.001_296),
            (37_665, 1.845_858_0, 37_665.0, 0.001_123_2),
            (38_334, 1.945_858_0, 37_665.0, 0.001_123_2),
            (38_395, 3.240_130_0, 38_761.0, 0.001_296),
            (38_486, 3.340_130_0, 38_761.0, 0.001_296),
            (38_639, 3.440_130_0, 38_761.0, 0.001_296),
            (38_761, 3.540_130_0, 38_761.0, 0.001_296),
            (38_820, 3.640_130_0, 38_761.0, 0.001_296),
            (38_942, 3.740_130_0, 38_761.0, 0.001_296),
            (39_004, 3.840_130_0, 38_761.0, 0.001_296),
            (39_126, 4.313_170_0, 39_126.0, 0.002_592),
            (39_887, 4.213_170_0, 39_126.0, 0.002_592),
        ];
        let first_day = 37_300 - 40_587;
        let last_day = 41_317 - 40_587 - 1;
        assert_eq!(last_day - first_day + 1, 4_017);
        for day in first_day..=last_day {
            let mjd = day + 40_587;
            let (_, offset, origin, drift) = TABLE
                .iter()
                .rev()
                .find(|row| row.0 <= mjd)
                .copied()
                .unwrap();
            let expected = offset + (mjd as f64 - origin) * drift;
            for second in [0, 43_200, 86_399] {
                let actual = tai_minus_utc_at(day * 86_400 + second, STRICT).unwrap();
                let at_second = expected + second as f64 / 86_400.0 * drift;
                assert!(
                    (actual.as_secs_f64() - at_second).abs() < 1e-9,
                    "day {day} (MJD {mjd}) second {second}: {actual} against {at_second}"
                );
            }
        }
    }

    /// A TAI reading of 1961-1971 is converted to UTC with the fraction of
    /// TAI − UTC kept. The reading `63 072 008` is 1971-12-31 23:59:5x, where TAI − UTC is 9.892 242 s,
    /// so UTC is 63 071 998.107 758 06 (exact rational arithmetic).
    #[test]
    fn utc_from_tai_keeps_the_fraction_of_the_offset() {
        let tai = Instant::<Tai>::from_epoch(Duration::from_secs(63_072_008));
        let utc = utc_from_tai(tai, STRICT).unwrap();
        assert!(!utc.leap_second);
        assert_eq!(utc.unix_seconds, 63_071_998);
        // 0.107 758 06 s, to the 8 digits of the pin; the exact value is
        // 63071998.10775806 + 5·10⁻¹⁶.
        let fraction = utc.subsec_attos as f64 / 1e18;
        assert!((fraction - 0.107_758_06).abs() < 1e-8, "{fraction}");
        // The same instant of 1961-01-01: TAI of 1961-01-01T00:00:00 UTC.
        let first = tai_from_utc(
            UtcInstant::from_unix(UnixTime::from_seconds(-283_996_800)),
            STRICT,
        )
        .unwrap();
        assert_eq!(
            first.since_epoch(),
            Duration::from_attos(-283_996_800 * 10i128.pow(18) + 1_422_818_000_000_000_000)
        );
        assert_eq!(
            utc_from_tai(first, STRICT).unwrap(),
            UtcInstant::from_unix(UnixTime::from_seconds(-283_996_800))
        );
    }

    /// UTC to TAI and back is the identity over the whole
    /// rate era, on whole seconds, on milliseconds and on arbitrary
    /// attoseconds (to the attosecond the division rounds to), at every
    /// segment edge and between.
    #[test]
    fn utc_and_tai_round_trip_over_every_rate_segment() {
        let mut checked = 0u32;
        let mut instants: Vec<(i64, u64)> = Vec::new();
        for entry in leap::RATE_ERA {
            for around in [-86_400, -3, -2, -1, 0, 1, 2, 3, 86_399, 86_400 * 40 + 17] {
                instants.push((entry.start_unix + around, 0));
            }
        }
        for unix in (leap::utc_start_unix()..leap::integer_era_start_unix()).step_by(1_999) {
            instants.push((unix, 0));
            instants.push((unix, 123_000_000_000_000_000));
            instants.push((unix, 999_999_999_999_999_999));
            instants.push((unix, 1_234_567_890_123));
        }
        for (unix, attos) in instants {
            if unix >= leap::integer_era_start_unix() || unix < leap::utc_start_unix() {
                continue;
            }
            let utc = UtcInstant {
                unix_seconds: unix,
                leap_second: false,
                subsec_attos: attos,
            };
            let tai = tai_from_utc(utc, STRICT).unwrap();
            let back = utc_from_tai(tai, STRICT).unwrap();
            assert!(!back.leap_second, "{utc:?}");
            let lost = (i128::from(back.unix_seconds) - i128::from(unix)) * 10i128.pow(18)
                + i128::from(back.subsec_attos)
                - i128::from(attos);
            // The first and last 0.1 s of a segment that is followed by a
            // step down are claimed by the next segment: a whole-second
            // reading never is, and the sub-second ones are checked there.
            assert!(lost.abs() <= 2, "{utc:?} came back as {back:?}");
            checked += 1;
        }
        assert!(checked > 10_000, "{checked}");
    }

    /// Where TAI − UTC steps down (1961-08-01 by 0.05 s,
    /// 1968-02-01 by 0.1 s) the instants of the overlap are read with the new
    /// segment, and where it steps up (seven times by 0.1 s) the instants of
    /// the gap get the UTC reading of the step. UTC never runs backwards as
    /// TAI runs forwards, anywhere around any of the thirteen starts.
    #[test]
    fn utc_never_runs_backwards_across_a_step() {
        const MS: i128 = 1_000_000_000_000_000;
        let reading = |tai: i128| {
            let utc = utc_from_tai(
                Instant::<Tai>::from_epoch(Duration::from_attos(tai)),
                LeapPolicy::Extrapolate,
            )
            .unwrap();
            i128::from(utc.unix_seconds) * 10i128.pow(18) + i128::from(utc.subsec_attos)
        };
        let step_up = [
            "1963-11-01",
            "1964-04-01",
            "1964-09-01",
            "1965-01-01",
            "1965-03-01",
            "1965-07-01",
            "1965-09-01",
        ];
        for entry in leap::RATE_ERA.iter().skip(1) {
            let start = i128::from(entry.start_unix) * 10i128.pow(18);
            let tai_start = start
                + tai_minus_utc_at(entry.start_unix, STRICT)
                    .unwrap()
                    .total_attos()
                    .unwrap();
            // The first TAI instant of a segment reads as its own start.
            assert_eq!(reading(tai_start), start, "{}", entry.label);
            let mut last = reading(tai_start - 250 * MS);
            for step in -249..=250 {
                let now = reading(tai_start + step * MS);
                assert!(now >= last, "{}: UTC went back at {step} ms", entry.label);
                last = now;
            }
            if step_up.contains(&entry.label) {
                // The gap is 0.1 s wide, and all of it reads as the step.
                for before in [1, MS, 50 * MS, 99 * MS] {
                    assert_eq!(
                        reading(tai_start - before),
                        start,
                        "{} {before}",
                        entry.label
                    );
                }
            } else if entry.label == "1961-08-01" || entry.label == "1968-02-01" {
                // The overlap: the new segment's 0.05 s or 0.1 s is read
                // from its own line, and just before it the old line's last
                // readings lie below the start.
                assert!(reading(tai_start - MS) < start, "{}", entry.label);
                // 40 ms of TAI is 40 ms less 6·10⁻¹⁰ s of this era's UTC, whose
                // second is 1.5·10⁻⁸ longer.
                let later = reading(tai_start + 40 * MS) - start;
                assert!(
                    (later - 40 * MS).abs() < 10i128.pow(12),
                    "{} {later}",
                    entry.label
                );
            }
        }
        // The 1972 step: TAI 63072009.9 is 0.1 s short of the integer era, in
        // the gap between 9.892 242 s and 10 s, and reads as 1972-01-01.
        let before_1972 = Instant::<Tai>::from_epoch(Duration::from_attos(
            63_072_009_900_000_000_000_000_000_i128,
        ));
        let utc = utc_from_tai(before_1972, STRICT).unwrap();
        assert_eq!((utc.unix_seconds, utc.subsec_attos), (63_072_000, 0));
    }

    /// Before the first rate segment's TAI start there is no UTC reading:
    /// refused under `Strict`, and TAI = UTC by the convention of
    /// `Extrapolate`. The first UTC reading, 1961-01-01T00:00:00, is TAI
    /// 1.422 818 s later.
    #[test]
    fn utc_from_tai_before_the_first_rate_segment_follows_the_policy() {
        let first_tai = -283_996_800 * 10i128.pow(18) + 1_422_818_000_000_000_000;
        let just_before = Instant::<Tai>::from_epoch(Duration::from_attos(first_tai - 1));
        assert_eq!(
            utc_from_tai(just_before, STRICT),
            Err(TimeError::BeforeModelStart)
        );
        let extrapolated = utc_from_tai(just_before, LeapPolicy::Extrapolate).unwrap();
        // TAI −283 996 798.577 182 is the floor second −283 996 799.
        assert_eq!(extrapolated.unix_seconds, -283_996_799);
        let on_start = Instant::<Tai>::from_epoch(Duration::from_attos(first_tai));
        assert_eq!(
            utc_from_tai(on_start, STRICT).unwrap(),
            UtcInstant::from_unix(UnixTime::from_seconds(-283_996_800))
        );
    }

    #[test]
    fn milliseconds_round_trip() {
        let value = UnixTime::from_millis(-1_500);
        assert_eq!(value.seconds(), -2);
        assert_eq!(value.as_millis(), -1_500);
    }
}
