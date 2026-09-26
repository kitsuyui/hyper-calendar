//! The 60-bit timestamps of version 1 and version 6 UUIDs.
//!
//! RFC 9562, *Universally Unique IDentifiers (UUIDs)*, May 2024, read
//! 2026-09-26 (`rfc9562`), §5.1: a version 1 UUID carries "a 60-bit
//! timestamp represented by Coordinated Universal Time (UTC) as a count of
//! 100-nanosecond intervals since 00:00:00.00, 15 October 1582", the
//! [`crate::epoch::UUID_GREGORIAN`] epoch. §5.6: version 6 carries the same
//! timestamp with its bits in the opposite order, most significant first,
//! so that the UUIDs sort by time.
//!
//! # The layouts
//!
//! In both, octet 6's high nibble is the version and octet 8's two high
//! bits the variant, `0b10`; the clock sequence and the node fill octets 8
//! to 15 and are the caller's.
//!
//! | Octets | Version 1 | Version 6 |
//! | --- | --- | --- |
//! | 0–3 | the timestamp's low 32 bits | its high 32 bits |
//! | 4–5 | its middle 16 bits | its middle 16 bits |
//! | 6–7 | the version, then its high 12 bits | the version, then its low 12 bits |
//!
//! §5.1's prose says version 1's `time_high` holds "the least significant
//! 12 bits"; its own test vector (Appendix A.1) puts the most significant
//! 12 there, and verified erratum 7955 corrects the prose to say so. This
//! module follows the vector, and the test below holds it to it.
//!
//! # What the count counts
//!
//! The RFC's own conversion (Appendix A, Figure 15) is POSIX time divided
//! by 100 ns plus 122 192 928 000 000 000, so the count is a POSIX-style
//! label that does not count leap seconds, and this module converts it to
//! and from [`UnixTime`]. §6.1 lets an implementation "alter" the
//! timestamp, to smear leap seconds among other things, so a UUID's time is
//! what its generator wrote, not a measurement. The 60 bits run out on
//! 31 March 5236, [`MAX_TIMESTAMP`]: §6.1 printed "5623 AD", a year
//! counted from 1970 instead of 1582, and verified erratum 8288 corrects
//! it to 5236.
//!
//! `docs/systems/binary-timestamps.md` works the RFC's example through.

use crate::error::{TimeError, TimeResult};
use crate::unix::UnixTime;

/// The number of 100-nanosecond intervals from the UUID epoch to the
/// POSIX epoch, RFC 9562's `Greg_Unix_offset`.
pub const GREGORIAN_UNIX_OFFSET: u64 = 0x01B2_1DD2_1381_4000;

/// The largest timestamp the 60-bit field holds.
pub const MAX_TIMESTAMP: u64 = (1 << 60) - 1;

/// 100-nanosecond intervals in a second.
const TICKS_PER_SECOND: i128 = 10_000_000;

/// Attoseconds in 100 ns.
const ATTOS_PER_TICK: u64 = 100_000_000_000;

/// The two UUID versions whose timestamp is this count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeVersion {
    /// Version 1, the timestamp's low bits first.
    V1,
    /// Version 6, the timestamp's high bits first.
    V6,
}

impl TimeVersion {
    /// The version number written in octet 6.
    #[must_use]
    pub const fn number(self) -> u8 {
        match self {
            Self::V1 => 1,
            Self::V6 => 6,
        }
    }
}

/// The timestamp of a POSIX time: the 100-nanosecond interval that
/// contains it, counted from 1582-10-15.
///
/// # Errors
///
/// [`TimeError::OutOfRange`] before 1582-10-15 or after the 60-bit field's
/// last interval.
pub fn timestamp_from_unix(unix: UnixTime) -> TimeResult<u64> {
    let ticks = i128::from(unix.seconds()) * TICKS_PER_SECOND
        + i128::from(unix.subsec_attos() / ATTOS_PER_TICK)
        + i128::from(GREGORIAN_UNIX_OFFSET);
    match u64::try_from(ticks) {
        Ok(ticks) if ticks <= MAX_TIMESTAMP => Ok(ticks),
        _ => Err(TimeError::OutOfRange),
    }
}

/// The POSIX time at the start of a timestamp's interval.
///
/// # Errors
///
/// [`TimeError::OutOfRange`] for a value that does not fit in 60 bits.
pub fn unix_from_timestamp(timestamp: u64) -> TimeResult<UnixTime> {
    if timestamp > MAX_TIMESTAMP {
        return Err(TimeError::OutOfRange);
    }
    let since_unix = i128::from(timestamp) - i128::from(GREGORIAN_UNIX_OFFSET);
    let seconds = since_unix.div_euclid(TICKS_PER_SECOND);
    // Below 10⁷, so the product is below 10¹⁸.
    let ticks = since_unix.rem_euclid(TICKS_PER_SECOND) as u64;
    let seconds = i64::try_from(seconds).map_err(|_| TimeError::OutOfRange)?;
    UnixTime::new(seconds, ticks * ATTOS_PER_TICK)
}

/// A UUID with this timestamp and version written into octets 0 to 7 and
/// the variant into octet 8, keeping the clock sequence and node of
/// `rest`, octets 8 to 15, below the variant bits.
///
/// # Errors
///
/// [`TimeError::OutOfRange`] for a timestamp wider than 60 bits.
pub fn encode(version: TimeVersion, timestamp: u64, rest: [u8; 8]) -> TimeResult<[u8; 16]> {
    if timestamp > MAX_TIMESTAMP {
        return Err(TimeError::OutOfRange);
    }
    let (first, middle, last12) = match version {
        TimeVersion::V1 => (
            (timestamp & 0xFFFF_FFFF) as u32,
            ((timestamp >> 32) & 0xFFFF) as u16,
            ((timestamp >> 48) & 0x0FFF) as u16,
        ),
        TimeVersion::V6 => (
            (timestamp >> 28) as u32,
            ((timestamp >> 12) & 0xFFFF) as u16,
            (timestamp & 0x0FFF) as u16,
        ),
    };
    let versioned = (u16::from(version.number()) << 12) | last12;
    let mut out = [0; 16];
    out[..4].copy_from_slice(&first.to_be_bytes());
    out[4..6].copy_from_slice(&middle.to_be_bytes());
    out[6..8].copy_from_slice(&versioned.to_be_bytes());
    out[8..].copy_from_slice(&rest);
    out[8] = (out[8] & 0x3F) | 0x80;
    Ok(out)
}

/// The version and the timestamp of a version 1 or version 6 UUID.
///
/// # Errors
///
/// [`TimeError::OutOfRange`] for a UUID whose variant is not RFC 9562's
/// `0b10` or whose version is neither 1 nor 6, since only those two carry
/// this count.
pub fn decode(uuid: [u8; 16]) -> TimeResult<(TimeVersion, u64)> {
    if uuid[8] >> 6 != 0b10 {
        return Err(TimeError::OutOfRange);
    }
    let [a, b, c, d, e, f, g, h, ..] = uuid;
    let first = u64::from(u32::from_be_bytes([a, b, c, d]));
    let middle = u64::from(u16::from_be_bytes([e, f]));
    let versioned = u16::from_be_bytes([g, h]);
    let last12 = u64::from(versioned & 0x0FFF);
    match versioned >> 12 {
        1 => Ok((TimeVersion::V1, (last12 << 48) | (middle << 32) | first)),
        6 => Ok((TimeVersion::V6, (first << 28) | (middle << 12) | last12)),
        _ => Err(TimeError::OutOfRange),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// RFC 9562, Appendix A: both test vectors carry 0x1EC9414C232AB00,
    /// 138 648 505 420 000 000 intervals, "Tuesday, February 22, 2022
    /// 2:22:22.000000 PM GMT-05:00", which is 19:22:22 UTC, POSIX
    /// 1 645 557 742.
    const VECTOR_TIMESTAMP: u64 = 0x1EC_9414_C232_AB00;
    const VECTOR_UNIX: i64 = 1_645_557_742;
    /// Octets 8 to 15 of both vectors, `B3C8-9F6BDECED846`.
    const VECTOR_REST: [u8; 8] = [0xB3, 0xC8, 0x9F, 0x6B, 0xDE, 0xCE, 0xD8, 0x46];

    fn hex(text: &str) -> [u8; 16] {
        let mut out = [0; 16];
        let digits = text
            .bytes()
            .filter(|byte| *byte != b'-')
            .map(|byte| match byte {
                b'0'..=b'9' => byte - b'0',
                b'A'..=b'F' => byte - b'A' + 10,
                _ => panic!("upper-case hex"),
            });
        for (index, digit) in digits.enumerate() {
            out[index / 2] |= digit << (4 * (1 - index % 2));
        }
        out
    }

    #[test]
    fn the_rfc_vector_timestamp_is_its_posix_second() {
        assert_eq!(VECTOR_TIMESTAMP, 138_648_505_420_000_000);
        let unix = UnixTime::from_seconds(VECTOR_UNIX);
        assert_eq!(timestamp_from_unix(unix), Ok(VECTOR_TIMESTAMP));
        assert_eq!(unix_from_timestamp(VECTOR_TIMESTAMP), Ok(unix));
        assert_eq!(GREGORIAN_UNIX_OFFSET, 122_192_928_000_000_000);
    }

    /// Appendix A.1, `C232AB00-9414-11EC-B3C8-9F6BDECED846`.
    #[test]
    fn the_version_1_vector() {
        let uuid = hex("C232AB00-9414-11EC-B3C8-9F6BDECED846");
        assert_eq!(
            encode(TimeVersion::V1, VECTOR_TIMESTAMP, VECTOR_REST),
            Ok(uuid)
        );
        assert_eq!(decode(uuid), Ok((TimeVersion::V1, VECTOR_TIMESTAMP)));
    }

    /// Appendix A.5, `1EC9414C-232A-6B00-B3C8-9F6BDECED846`.
    #[test]
    fn the_version_6_vector() {
        let uuid = hex("1EC9414C-232A-6B00-B3C8-9F6BDECED846");
        assert_eq!(
            encode(TimeVersion::V6, VECTOR_TIMESTAMP, VECTOR_REST),
            Ok(uuid)
        );
        assert_eq!(decode(uuid), Ok((TimeVersion::V6, VECTOR_TIMESTAMP)));
    }

    #[test]
    fn the_count_starts_on_15_october_1582_and_ends_in_60_bits() {
        let epoch = crate::epoch::UUID_GREGORIAN.tai_reading.whole_seconds();
        let epoch = UnixTime::from_seconds(i64::try_from(epoch).expect("fits"));
        assert_eq!(timestamp_from_unix(epoch), Ok(0));
        let before = UnixTime::new(epoch.seconds() - 1, 999_999_999_999_999_999).expect("valid");
        assert_eq!(timestamp_from_unix(before), Err(TimeError::OutOfRange));
        let last = unix_from_timestamp(MAX_TIMESTAMP).expect("in range");
        assert_eq!(timestamp_from_unix(last), Ok(MAX_TIMESTAMP));
        // 2⁶⁰ intervals of 100 ns is about 3 653 years from 1582: erratum
        // 8288 gives the end as 5236-03-31 21:21:00.6846976 UTC, which is
        // POSIX 103 072 857 660, 1 192 973 days after 1970 less 9 540 s.
        assert_eq!(
            last,
            UnixTime::new(103_072_857_660, 684_697_500_000_000_000).expect("valid")
        );
        assert_eq!(1_192_973 * 86_400 - 9_540, 103_072_857_660_i64);
        assert_eq!(
            unix_from_timestamp(MAX_TIMESTAMP + 1),
            Err(TimeError::OutOfRange)
        );
        assert_eq!(
            encode(TimeVersion::V1, MAX_TIMESTAMP + 1, [0; 8]),
            Err(TimeError::OutOfRange)
        );
    }

    /// A time between two intervals is written as the interval containing
    /// it, and the sub-interval part is gone.
    #[test]
    fn a_sub_interval_time_is_floored() {
        let unix = UnixTime::new(VECTOR_UNIX, 123_456_789_000_000_000).expect("valid");
        let timestamp = timestamp_from_unix(unix).expect("in range");
        assert_eq!(timestamp, VECTOR_TIMESTAMP + 1_234_567);
        assert_eq!(
            unix_from_timestamp(timestamp),
            UnixTime::new(VECTOR_UNIX, 123_456_700_000_000_000)
        );
    }

    #[test]
    fn only_rfc_variant_versions_1_and_6_decode() {
        let mut uuid = hex("C232AB00-9414-11EC-B3C8-9F6BDECED846");
        uuid[6] = 0x41; // version 4
        assert_eq!(decode(uuid), Err(TimeError::OutOfRange));
        let mut uuid = hex("C232AB00-9414-11EC-B3C8-9F6BDECED846");
        uuid[8] = 0x33; // variant 0b00
        assert_eq!(decode(uuid), Err(TimeError::OutOfRange));
    }

    #[test]
    fn both_layouts_round_trip() {
        for timestamp in [0, 1, 0xFFF, 0x1000, VECTOR_TIMESTAMP, MAX_TIMESTAMP] {
            for version in [TimeVersion::V1, TimeVersion::V6] {
                let uuid = encode(version, timestamp, [0xFF; 8]).expect("fits");
                assert_eq!(uuid[8] >> 6, 0b10);
                assert_eq!(decode(uuid), Ok((version, timestamp)));
            }
        }
    }
}
