//! Internal helpers shared by the body modules.
//!
//! Nothing here is public. The Gregorian date of a mission landing goes
//! through [`hc_calendar::gregorian::to_fixed`], which owns that arithmetic;
//! [`utc_unix_seconds`] only changes its shape into the bare POSIX timestamp
//! a `const` table can hold.

use hc_calendar::fixed::RD_OF_UNIX_EPOCH;
use hc_calendar::gregorian;
use hc_core::math::floor;
use hc_core::{Duration, Instant, Tai, TimeResult};

/// Seconds in a terrestrial day as the astronomical series count them: 86 400
/// exactly, with no leap second, because TAI and TT are uniform scales.
pub(crate) const SECONDS_PER_DAY: f64 = 86_400.0;

/// The TAI reading of J2000.0, measured from `1970-01-01T00:00:00 TAI`.
///
/// `2000-01-01T12:00:00 TT` is `11:59:27.816 TAI`, and `TAI − UTC` was 32 s in
/// 2000, so the instant is `2000-01-01T11:58:55.816 UTC`. The POSIX timestamp
/// of `2000-01-01T12:00:00Z` is 946 728 000, so the TAI reading is
/// `946 728 000 − 64.184 + 32 = 946 727 967.816`.
///
/// This is [`hc_core::epoch::J2000`]. It is bound to a local name because
/// every series in this crate is stated in `Δt_J2000`, and a test below
/// re-derives it from the leap-second table so that the two can never drift
/// apart unnoticed.
const J2000_TAI_READING: Duration = hc_core::epoch::J2000.tai_reading;

/// TT days elapsed from J2000.0 (`2000-01-01T12:00:00 TT`) to `instant`.
///
/// TAI and TT differ by a constant 32.184 s, so a difference of two TAI
/// readings is also a difference of two TT readings; the offset cancels and
/// the result is the `Δt_J2000` that every series in this crate is stated in.
///
/// The subtraction is done on the exact integer seconds before it reaches
/// `f64`, so the answer keeps microsecond resolution across the whole
/// space-age range rather than losing it to the 1.7 × 10⁹ magnitude of a raw
/// POSIX-style reading.
pub(crate) fn j2000_offset_days(instant: Instant<Tai>) -> f64 {
    let reading = instant.since_epoch();
    let epoch = J2000_TAI_READING;
    let seconds = (reading.whole_seconds() - epoch.whole_seconds()) as f64;
    let attos = (reading.subsec_attos() as f64 - epoch.subsec_attos() as f64) * 1e-18;
    (seconds + attos) / SECONDS_PER_DAY
}

/// The TAI instant a number of TT days after J2000.0.
///
/// # Errors
///
/// Returns [`hc_core::TimeError::NotFinite`] for a non-finite argument and
/// [`hc_core::TimeError::Overflow`] when the result leaves the representable
/// range.
pub(crate) fn instant_from_j2000_offset(days: f64) -> TimeResult<Instant<Tai>> {
    let span = Duration::from_secs_f64(days * SECONDS_PER_DAY)?;
    Instant::from_epoch(J2000_TAI_READING).checked_add(span)
}

/// The fractional part of `x`, always in `[0, 1)` even when `x` is negative.
pub(crate) fn fract(x: f64) -> f64 {
    x - floor(x)
}

/// `x` reduced modulo `modulus` into `[0, modulus)`.
pub(crate) fn modulo(x: f64, modulus: f64) -> f64 {
    x - modulus * floor(x / modulus)
}

/// `degrees` folded into `(-180, 180]`.
///
/// Differences of two angles are always small in this crate; folding removes
/// the spurious 360° that appears whenever the pair straddles zero.
pub(crate) fn signed_degrees(degrees: f64) -> f64 {
    let wrapped = modulo(degrees, 360.0);
    if wrapped > 180.0 {
        wrapped - 360.0
    } else {
        wrapped
    }
}

/// The POSIX timestamp of a UTC civil date and time.
///
/// POSIX time ignores leap seconds by construction, so this is exact for every
/// second that is not itself an inserted leap second; none of the instants
/// this crate names are.
///
/// # Panics
///
/// For a date that does not exist. Every caller is a `const` initialiser of
/// a published landing time, or a test, so the panic happens at compile time
/// and an impossible date is a build error rather than a wrong constant.
pub(crate) const fn utc_unix_seconds(
    year: i64,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
) -> i64 {
    let day = match gregorian::to_fixed(year, month, day) {
        Ok(rd) => rd.0,
        Err(_) => panic!("a landing date that does not exist"),
    };
    (day - RD_OF_UNIX_EPOCH) * 86_400 + hour as i64 * 3_600 + minute as i64 * 60 + second as i64
}

/// The TAI instant of a UTC civil date and time.
///
/// Only the tests need this: the crate's own data carries POSIX timestamps
/// already, and callers hand in an `Instant<Tai>` they built themselves.
///
/// # Errors
///
/// Propagates [`hc_core::unix::tai_from_unix`]: instants before 1961 or beyond
/// the published validity of the leap-second table are refused.
#[cfg(test)]
pub(crate) fn tai_from_utc_fields(
    year: i64,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
) -> TimeResult<Instant<Tai>> {
    let unix =
        hc_core::UnixTime::from_seconds(utc_unix_seconds(year, month, day, hour, minute, second));
    hc_core::unix::tai_from_unix(unix, hc_core::unix::LeapPolicy::Extrapolate)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The timestamp is the owner's fixed day, reshaped: whole days since
    /// 1970 times 86 400, plus the time of day.
    #[test]
    fn the_unix_helper_changes_the_shape_of_the_gregorian_conversion_and_nothing_else() {
        for (year, month, day) in [(1, 1, 1), (1582, 10, 15), (1970, 1, 1), (2024, 2, 29)] {
            let rd = gregorian::to_fixed(year, month, day).unwrap();
            assert_eq!(
                utc_unix_seconds(year, month, day, 1, 2, 3),
                rd.to_unix_days() * 86_400 + 3_723
            );
        }
    }

    #[test]
    #[should_panic(expected = "does not exist")]
    fn the_unix_helper_refuses_a_date_that_does_not_exist() {
        let _ = utc_unix_seconds(2023, 2, 29, 0, 0, 0);
    }

    #[test]
    fn the_unix_helper_reproduces_known_timestamps() {
        assert_eq!(utc_unix_seconds(1970, 1, 1, 0, 0, 0), 0);
        assert_eq!(utc_unix_seconds(2000, 1, 1, 12, 0, 0), 946_728_000);
        assert_eq!(utc_unix_seconds(2012, 8, 6, 5, 17, 57), 1_344_230_277);
    }

    #[test]
    fn the_j2000_offset_is_zero_at_the_j2000_epoch() {
        let offset = j2000_offset_days(Instant::from_epoch(J2000_TAI_READING));
        assert!(offset.abs() < 1e-12, "offset {offset}");
    }

    #[test]
    fn the_j2000_epoch_re_derives_from_the_leap_second_table() {
        // An independent route to the same instant: 2000-01-01T12:00:00 TT is
        // 11:58:55.816 UTC, because TT - TAI is 32.184 s and TAI - UTC was
        // 32 s in 2000. Every Martian answer in this crate is measured from
        // this epoch, so an error of even a second here would move all of
        // them; checking it against the leap-second path rather than against
        // a restated literal is what makes that impossible to miss.
        let unix = hc_core::UnixTime::new(946_727_935, 816_000_000_000_000_000).unwrap();
        let from_utc =
            hc_core::unix::tai_from_unix(unix, hc_core::unix::LeapPolicy::Strict).unwrap();
        assert_eq!(from_utc.since_epoch(), J2000_TAI_READING);
    }

    #[test]
    fn the_j2000_offset_round_trips_through_an_instant() {
        for days in [-40_000.0, -1.5, 0.0, 0.25, 9_131.0] {
            let instant = instant_from_j2000_offset(days).unwrap();
            let back = j2000_offset_days(instant);
            assert!((back - days).abs() < 1e-9, "{days} -> {back}");
        }
    }

    #[test]
    fn the_j2000_epoch_is_noon_on_the_first_of_january_2000_in_tt() {
        // 2000-01-01T12:00:00 TT is 2000-01-01T11:58:55.816 UTC, because
        // TT - UTC was 32.184 + 32 = 64.184 s in 2000.
        let instant = tai_from_utc_fields(2000, 1, 1, 11, 58, 55).unwrap();
        let offset = j2000_offset_days(instant) * SECONDS_PER_DAY;
        assert!((offset + 0.816).abs() < 1e-6, "offset {offset} s");
    }

    #[test]
    fn angles_fold_into_the_expected_intervals() {
        assert!((modulo(-10.0, 360.0) - 350.0).abs() < 1e-12);
        assert!((signed_degrees(350.0) + 10.0).abs() < 1e-12);
        assert!((signed_degrees(180.0) - 180.0).abs() < 1e-12);
        assert!((fract(-0.25) - 0.75).abs() < 1e-12);
    }
}
