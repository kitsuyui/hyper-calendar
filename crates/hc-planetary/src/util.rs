//! Internal helpers shared by the body modules.
//!
//! Nothing here is public. The proleptic Gregorian arithmetic is a private
//! copy of the formula that [`hc_calendar::gregorian::to_fixed`] implements,
//! used only to write down the ten mission landing times.

use hc_core::math::floor;
use hc_core::{Duration, Instant, Tai, TimeResult};

/// Seconds in a terrestrial day as the astronomical series count them: 86 400
/// exactly, with no leap second, because TAI and TT are uniform scales.
pub(crate) const SECONDS_PER_DAY: f64 = 86_400.0;

/// The Rata Die of `1970-01-01`, repeated here so the landing-time constants
/// can be evaluated in a `const` context.
const RD_OF_UNIX_EPOCH: i64 = 719_163;

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

/// Whether `year` is a leap year in the proleptic Gregorian calendar.
const fn is_gregorian_leap(year: i64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// The Rata Die of a proleptic Gregorian date.
///
/// Reingold and Dershowitz, *Calendrical Calculations*, `fixed-from-gregorian`.
/// No validation: every caller is a compile-time constant in this crate or a
/// test.
const fn fixed_from_gregorian(year: i64, month: u8, day: u8) -> i64 {
    let prior = year - 1;
    let correction = if month <= 2 {
        0
    } else if is_gregorian_leap(year) {
        -1
    } else {
        -2
    };
    365 * prior + prior.div_euclid(4) - prior.div_euclid(100)
        + prior.div_euclid(400)
        + (367 * month as i64 - 362) / 12
        + correction
        + day as i64
}

/// The POSIX timestamp of a UTC civil date and time.
///
/// POSIX time ignores leap seconds by construction, so this is exact for every
/// second that is not itself an inserted leap second; none of the instants
/// this crate names are.
pub(crate) const fn utc_unix_seconds(
    year: i64,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
) -> i64 {
    (fixed_from_gregorian(year, month, day) - RD_OF_UNIX_EPOCH) * 86_400
        + hour as i64 * 3_600
        + minute as i64 * 60
        + second as i64
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

    #[test]
    fn the_gregorian_helper_agrees_with_the_published_rata_die_anchors() {
        // Reingold and Dershowitz, table 1.2.
        assert_eq!(fixed_from_gregorian(1, 1, 1), 1);
        assert_eq!(fixed_from_gregorian(1970, 1, 1), 719_163);
        assert_eq!(fixed_from_gregorian(2000, 1, 1), 730_120);
        assert_eq!(fixed_from_gregorian(1582, 10, 15), 577_736);
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
