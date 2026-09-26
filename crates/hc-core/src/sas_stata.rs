//! SAS datetimes and Stata's two datetime encodings, all counted from
//! 1960-01-01T00:00:00 ([`crate::epoch::SAS_STATA`]).
//!
//! Three conventions share the origin and differ in what they count, so
//! each has its own functions (policy.md §5):
//!
//! - **`sas-datetime`**, [`sas_datetime`]: "the number of seconds between
//!   January 1, 1960, and an hour/minute/second within a specified date"
//!   (SAS 9.3 Language Reference: Concepts, "About SAS Date, Time, and
//!   Datetime Values", `sas-lrcon-dates`). The SAS page does not mention
//!   leap seconds; Stata's documentation says "SAS ignores leap seconds"
//!   (`stata-help-datetime-conversion`), and the count is read so: a
//!   POSIX-style label, 86 400 s a day. SAS calculates on "dates ranging
//!   from A.D. 1582 to A.D. 19,900"; the range is the whole of those
//!   years, [`SAS_FIRST_DAY`] to [`SAS_LAST_DAY`].
//! - **`stata-tc`**, [`stata_tc`]: Stata's `%tc`, datetime/c,
//!   "milliseconds since 01jan1960 00:00:00.000, assuming 86,400 s/day"
//!   (Stata, `help datetime`, `stata-help-datetime`), so a label like
//!   POSIX time in milliseconds, which cannot name 23:59:60.
//! - **`stata-tc-utc`**, [`stata_tc_utc`]: Stata's `%tC`, datetime/C,
//!   "milliseconds since 01jan1960 00:00:00.000, adjusted for leap
//!   seconds", which Stata calls equivalent to UTC: the leap seconds
//!   inserted since 1972 are counted, 24 of them by 2010, and 23:59:60 of a
//!   leap second is a valid time. Before 1972 no second is added, and the
//!   fractional `TAI − UTC` of 1961–1971 is not part of the count, so
//!   `%tC` is `%tc` plus 1 000 ms for each leap second before the instant.
//!   Stata looks the leap seconds up in its own file; this module uses
//!   [`crate::leap::TABLE`], and a time past the table is refused under
//!   [`LeapPolicy::Strict`], as Stata's manual says a `%tC` value more than
//!   six months ahead cannot be known.
//!
//! Stata's range for both is 01jan0100 00:00:00.000 to 31dec9999
//! 23:59:59.999 (`help datetime_functions`, `stata-help-datetime-functions`).
//! Stata's `%td`, and SAS date values, are day counts:
//! `stata-date` and `sas-date` in `hc-calendars-solar`.
//! `docs/systems/statistical-software-dates.md` describes all five.

use crate::duration::{ATTOS_PER_SEC, Duration};
use crate::error::{TimeError, TimeResult};
use crate::scale::{Instant, Tai};
use crate::unix::{LeapPolicy, UnixTime, UtcInstant, tai_from_utc, utc_from_tai};

/// The POSIX second of 1960-01-01T00:00:00, the origin of all three.
pub const EPOCH_UNIX: i64 = -315_619_200;

/// The first SAS date, 1 January 1582, −138 061 days from 1960.
pub const SAS_FIRST_DAY: i64 = -138_061;

/// The last SAS date, 31 December 19 900.
pub const SAS_LAST_DAY: i64 = 6_552_815;

/// The first Stata day, 01jan0100, −679 350 in `%td`.
pub const STATA_FIRST_DAY: i64 = -679_350;

/// The last Stata day, 31dec9999, 2 936 549 in `%td`.
pub const STATA_LAST_DAY: i64 = 2_936_549;

const SECONDS_PER_DAY: i64 = 86_400;
const MILLIS_PER_DAY: i64 = 86_400_000;
const ATTOS_PER_MILLI: u64 = 1_000_000_000_000_000;

/// 1972-01-01T00:00:00Z, from when `%tC` counts leap seconds.
const LEAP_ERA_UNIX: i64 = 63_072_000;

/// `TAI − UTC` on 1 January 1972, which `%tC` does not count.
const TAI_MINUS_UTC_1972: i64 = 10;

/// The SAS datetime of a POSIX time: seconds from 1960, exactly.
///
/// # Errors
///
/// [`TimeError::OutOfRange`] outside 1 January 1582 to the end of
/// 31 December 19 900.
pub fn sas_datetime(unix: UnixTime) -> TimeResult<Duration> {
    let since = unix
        .as_duration()
        .checked_sub(Duration::from_secs(i128::from(EPOCH_UNIX)))?;
    check_days(
        since.floor_seconds(),
        SAS_FIRST_DAY,
        SAS_LAST_DAY,
        SECONDS_PER_DAY,
    )?;
    Ok(since)
}

/// The POSIX time of a SAS datetime.
///
/// # Errors
///
/// As [`sas_datetime`].
pub fn unix_from_sas_datetime(seconds: Duration) -> TimeResult<UnixTime> {
    check_days(
        seconds.floor_seconds(),
        SAS_FIRST_DAY,
        SAS_LAST_DAY,
        SECONDS_PER_DAY,
    )?;
    let unix = seconds.checked_add(Duration::from_secs(i128::from(EPOCH_UNIX)))?;
    let whole = i64::try_from(unix.whole_seconds()).map_err(|_| TimeError::OutOfRange)?;
    UnixTime::new(whole, unix.subsec_attos())
}

/// Stata's `%tc` of a POSIX time: milliseconds from 1960, the
/// sub-millisecond part floored.
///
/// # Errors
///
/// [`TimeError::OutOfRange`] outside 01jan0100 to 31dec9999.
pub fn stata_tc(unix: UnixTime) -> TimeResult<i64> {
    let millis = (unix.seconds() - EPOCH_UNIX)
        .checked_mul(1_000)
        .and_then(|ms| ms.checked_add((unix.subsec_attos() / ATTOS_PER_MILLI) as i64))
        .ok_or(TimeError::OutOfRange)?;
    check_days(
        i128::from(millis),
        STATA_FIRST_DAY,
        STATA_LAST_DAY,
        MILLIS_PER_DAY,
    )?;
    Ok(millis)
}

/// The POSIX time of a `%tc` value.
///
/// # Errors
///
/// As [`stata_tc`].
pub fn unix_from_stata_tc(millis: i64) -> TimeResult<UnixTime> {
    check_days(
        i128::from(millis),
        STATA_FIRST_DAY,
        STATA_LAST_DAY,
        MILLIS_PER_DAY,
    )?;
    let seconds = millis.div_euclid(1_000) + EPOCH_UNIX;
    // In [0, 1 000), so the product is below 10¹⁸.
    UnixTime::new(seconds, millis.rem_euclid(1_000) as u64 * ATTOS_PER_MILLI)
}

/// Stata's `%tC` of a UTC instant: `%tc` plus a second for each leap
/// second inserted before it, the sub-millisecond part floored. A leap
/// second, 23:59:60, has a value of its own.
///
/// # Errors
///
/// [`TimeError::OutOfRange`] outside 01jan0100 to 31dec9999, or for an
/// instant marked as a leap second where the table has none;
/// [`TimeError::AfterModelEnd`] past the leap-second table under
/// [`LeapPolicy::Strict`].
pub fn stata_tc_utc(utc: UtcInstant, policy: LeapPolicy) -> TimeResult<i64> {
    let label = UnixTime::new(utc.unix_seconds, utc.subsec_attos)?;
    let posix_millis = stata_tc(label)?;
    if utc.unix_seconds < LEAP_ERA_UNIX {
        return if utc.leap_second {
            Err(TimeError::OutOfRange)
        } else {
            Ok(posix_millis)
        };
    }
    if utc.leap_second && !is_inserted_leap_second(utc.unix_seconds, policy)? {
        return Err(TimeError::OutOfRange);
    }
    // From 1972 `TAI − UTC` is whole seconds, so this is exact.
    let tai = tai_from_utc(utc, policy)?.since_epoch();
    let leap_era_seconds =
        tai.floor_seconds() - i128::from(EPOCH_UNIX) - i128::from(TAI_MINUS_UTC_1972);
    let millis = leap_era_seconds * 1_000 + i128::from(tai.subsec_attos() / ATTOS_PER_MILLI);
    i64::try_from(millis).map_err(|_| TimeError::OutOfRange)
}

/// The UTC instant of a `%tC` value, naming the leap second when the
/// value falls in one.
///
/// # Errors
///
/// As [`stata_tc_utc`].
pub fn utc_from_stata_tc_utc(millis: i64, policy: LeapPolicy) -> TimeResult<UtcInstant> {
    let leap_era_start = (LEAP_ERA_UNIX - EPOCH_UNIX) * 1_000;
    if millis < leap_era_start {
        return Ok(UtcInstant::from_unix(unix_from_stata_tc(millis)?));
    }
    let seconds = i128::from(millis.div_euclid(1_000))
        + i128::from(EPOCH_UNIX)
        + i128::from(TAI_MINUS_UTC_1972);
    let subsec = millis.rem_euclid(1_000) as u64 * ATTOS_PER_MILLI;
    let tai: Instant<Tai> = Instant::from_epoch(Duration::new(seconds, subsec)?);
    let utc = utc_from_tai(tai, policy)?;
    // Refuse what `%tc` would refuse at the same label.
    stata_tc(utc.to_unix_lossy())?;
    Ok(utc)
}

/// Whether a leap second was inserted just before this POSIX second.
fn is_inserted_leap_second(unix_seconds: i64, policy: LeapPolicy) -> TimeResult<bool> {
    let after = crate::unix::tai_minus_utc_at(unix_seconds, policy)?;
    let before = crate::unix::tai_minus_utc_at(unix_seconds - 1, policy)?;
    Ok(after.checked_sub(before)? == Duration::SECOND)
}

/// Refuse a count whose day is outside `first..=last`, `per_day` units a
/// day.
fn check_days(count: i128, first: i64, last: i64, per_day: i64) -> TimeResult<()> {
    let day = count.div_euclid(i128::from(per_day));
    if day < i128::from(first) || day > i128::from(last) {
        return Err(TimeError::OutOfRange);
    }
    Ok(())
}

// `ATTOS_PER_SEC` is the unit the constants above divide; keep them tied.
const _: () = assert!(ATTOS_PER_MILLI * 1_000 == ATTOS_PER_SEC);

#[cfg(test)]
mod tests {
    use super::*;

    const STRICT: LeapPolicy = LeapPolicy::Strict;

    /// POSIX seconds of a day's midnight, from its day number in 1960.
    fn unix_of_day(day: i64) -> i64 {
        day * SECONDS_PER_DAY + EPOCH_UNIX
    }

    fn utc(unix: i64, millis: u64) -> UtcInstant {
        UtcInstant {
            unix_seconds: unix,
            leap_second: false,
            subsec_attos: millis * ATTOS_PER_MILLI,
        }
    }

    /// SAS's own example: `'26oct02'd` is 15 639, so midnight on
    /// 26 October 2002 is 15 639 × 86 400 s.
    #[test]
    fn sas_counts_seconds_from_1960() {
        let unix = UnixTime::from_seconds(unix_of_day(15_639));
        assert_eq!(sas_datetime(unix), Ok(Duration::from_secs(15_639 * 86_400)));
        assert_eq!(
            unix_from_sas_datetime(Duration::from_secs(15_639 * 86_400)),
            Ok(unix)
        );
        assert_eq!(
            sas_datetime(UnixTime::from_seconds(EPOCH_UNIX)),
            Ok(Duration::ZERO)
        );
    }

    #[test]
    fn sas_refuses_what_it_does_not_calculate() {
        let first = UnixTime::from_seconds(unix_of_day(SAS_FIRST_DAY));
        assert!(sas_datetime(first).is_ok());
        let before = UnixTime::from_seconds(unix_of_day(SAS_FIRST_DAY) - 1);
        assert_eq!(sas_datetime(before), Err(TimeError::OutOfRange));
        let last = Duration::from_secs(i128::from(SAS_LAST_DAY + 1) * 86_400 - 1);
        assert!(unix_from_sas_datetime(last).is_ok());
        assert_eq!(
            unix_from_sas_datetime(last.checked_add(Duration::SECOND).expect("fits")),
            Err(TimeError::OutOfRange)
        );
    }

    /// Stata, `help datetime`: 20jan2010 09:15:22.120 is 1 579 598 122 120
    /// in `%tc` and 1 579 598 146 120 in `%tC`, 24 leap seconds apart; its
    /// day is 18 282 in `%td`.
    #[test]
    fn stata_20_january_2010() {
        let unix = unix_of_day(18_282) + 9 * 3_600 + 15 * 60 + 22;
        let reading = utc(unix, 120);
        let tc = stata_tc(reading.to_unix_lossy()).expect("in range");
        assert_eq!(tc, 1_579_598_122_120);
        let tc_utc = stata_tc_utc(reading, STRICT).expect("in the table");
        assert_eq!(tc_utc, 1_579_598_146_120);
        assert_eq!(tc_utc - tc, 24_000);
        assert_eq!(unix_from_stata_tc(tc), Ok(reading.to_unix_lossy()));
        assert_eq!(utc_from_stata_tc_utc(tc_utc, STRICT), Ok(reading));
    }

    /// Stata, `help datetime_conversion`: noon of 23nov2010 is
    /// 1 606 132 800 000 in `%tc` and 1 606 132 824 000 in `%tC`,
    /// "because 24 seconds have been inserted into datetime/C between
    /// 01jan1960 and 23nov2010".
    #[test]
    fn stata_noon_23_november_2010() {
        let day = 1_606_132_800_000 / MILLIS_PER_DAY;
        let noon = utc(unix_of_day(day) + 12 * 3_600, 0);
        assert_eq!(stata_tc(noon.to_unix_lossy()), Ok(1_606_132_800_000));
        assert_eq!(stata_tc_utc(noon, STRICT), Ok(1_606_132_824_000));
    }

    /// `help datetime`: `tc(15jun2004 12:00:00)` is 1 402 920 000 000, and
    /// `td(15jun2004)` is 16 237; 22jul2010 00:00:00.000 is
    /// 1 595 376 000 000 and 18 465.
    #[test]
    fn stata_typed_examples() {
        let noon = UnixTime::from_seconds(unix_of_day(16_237) + 12 * 3_600);
        assert_eq!(stata_tc(noon), Ok(1_402_920_000_000));
        let midnight = UnixTime::from_seconds(unix_of_day(18_465));
        assert_eq!(stata_tc(midnight), Ok(1_595_376_000_000));
    }

    /// `help datetime_conversion`: 31dec2005 23:59:60 "is an invalid
    /// datetime/c but a valid datetime/C", and 30dec2005 23:59:60 is
    /// invalid in both, as is 30jun1997 23:59:60 in `%tc`.
    #[test]
    fn only_real_leap_seconds_have_a_tc_utc_value() {
        // 2006-01-01T00:00:00Z is POSIX 1 136 073 600.
        let leap = UtcInstant {
            unix_seconds: 1_136_073_600,
            leap_second: true,
            subsec_attos: 0,
        };
        let value = stata_tc_utc(leap, STRICT).expect("a real leap second");
        let next = stata_tc_utc(utc(1_136_073_600, 0), STRICT).expect("in range");
        assert_eq!(next - value, 1_000);
        assert_eq!(utc_from_stata_tc_utc(value, STRICT), Ok(leap));
        let not_leap = UtcInstant {
            unix_seconds: 1_136_073_600 - 86_400,
            ..leap
        };
        assert_eq!(stata_tc_utc(not_leap, STRICT), Err(TimeError::OutOfRange));
        // 1997-07-01T00:00:00Z, the leap second of 30 June 1997.
        let june_1997 = UtcInstant {
            unix_seconds: 867_715_200,
            ..leap
        };
        assert!(stata_tc_utc(june_1997, STRICT).is_ok());
    }

    /// Before 1972 the two encodings agree, and on 1 January 1972 they
    /// still do: no leap second had been inserted yet.
    #[test]
    fn tc_and_tc_utc_agree_until_the_first_leap_second() {
        for unix in [EPOCH_UNIX, 0, LEAP_ERA_UNIX - 1, LEAP_ERA_UNIX, 78_796_799] {
            let reading = utc(unix, 0);
            assert_eq!(
                stata_tc_utc(reading, STRICT),
                stata_tc(reading.to_unix_lossy()),
                "{unix}"
            );
            let value = stata_tc_utc(reading, STRICT).expect("in range");
            assert_eq!(utc_from_stata_tc_utc(value, STRICT), Ok(reading));
        }
        // 1972-07-01, after the first one.
        let july = utc(78_796_800, 0);
        assert_eq!(
            stata_tc_utc(july, STRICT).expect("in range")
                - stata_tc(july.to_unix_lossy()).expect("in range"),
            1_000
        );
    }

    #[test]
    fn stata_refuses_outside_0100_to_9999_and_past_the_table() {
        let first = unix_of_day(STATA_FIRST_DAY);
        assert!(stata_tc(UnixTime::from_seconds(first)).is_ok());
        assert_eq!(
            stata_tc(UnixTime::from_seconds(first - 1)),
            Err(TimeError::OutOfRange)
        );
        let end = (STATA_LAST_DAY + 1) * MILLIS_PER_DAY;
        assert!(unix_from_stata_tc(end - 1).is_ok());
        assert_eq!(unix_from_stata_tc(end), Err(TimeError::OutOfRange));
        // 2100 is past any announced leap second.
        let later = utc(4_102_444_800, 0);
        assert_eq!(stata_tc_utc(later, STRICT), Err(TimeError::AfterModelEnd));
        assert!(stata_tc_utc(later, LeapPolicy::Extrapolate).is_ok());
    }
}
