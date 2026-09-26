//! NTP's 128-bit date format, its eras, and the 64-bit timestamp that
//! wraps in 2036.
//!
//! RFC 5905, *Network Time Protocol Version 4*, June 2010, §6, read
//! 2026-09-26 (`rfc5905`). An NTP date is a signed 64-bit count of seconds
//! from the prime epoch, "0 h 1 January 1900 UTC" ([`crate::epoch::NTP`]),
//! and a 64-bit fraction; for convenience the seconds are split into a
//! 32-bit *era number* and a 32-bit *era offset*:
//!
//! > era = s / 2^(32) and timestamp = s - era * 2^(32), which works for
//! > positive and negative dates.
//!
//! so the division floors, and dates before 1900 are in negative eras. The
//! 64-bit timestamp of the packet headers is the era offset and the top 32
//! bits of the fraction: it cannot say which era it is in. "Era 0 includes
//! dates from the prime epoch to some time in 2036, when the timestamp
//! field wraps around" — 2³² s after 1900, 06:28:16 on 7 February 2036 —
//! and "Eras cannot be produced by NTP directly … they can be derived from
//! external means". [`NtpTimestamp::resolve`] is that external means made
//! explicit: the caller supplies a reference time, and the era is the one
//! that puts the timestamp within 2³¹ s, about 68 years, of it, the window
//! §6 names: "if the client is set within 68 years of the server …
//! correct values are obtained even if the client and server are in
//! adjacent eras".
//!
//! # What the seconds count
//!
//! §6's own table puts 1 January 1970 at 2 208 988 800 and 1 January 1972
//! at 2 272 060 800, whole days of 86 400 s from 1900 with no leap second
//! among them, so an NTP date is a POSIX-style label and converts to and
//! from [`UnixTime`]. §6 notes that UTC "did not exist prior to
//! 1 January 1972, but it is convenient to assume it has existed for all
//! eternity"; this module does the same, and the conversions never need
//! the leap-second table.
//!
//! In either format a value of zero "is a special case representing
//! unknown or unsynchronized time"; [`NtpTimestamp::resolve`] refuses it
//! rather than read it as 1900 or 2036.
//!
//! `docs/systems/binary-timestamps.md` works an example through and lists
//! the errata to §6's table.

use crate::duration::ATTOS_PER_SEC;
use crate::error::{TimeError, TimeResult};
use crate::unix::UnixTime;

/// Seconds in one era, 2³².
pub const ERA_SECONDS: i64 = 1 << 32;

/// The POSIX second of the prime epoch, 1900-01-01T00:00:00Z.
pub const PRIME_EPOCH_UNIX: i64 = -2_208_988_800;

/// A 128-bit NTP date: era, era offset and a fraction of 2⁻⁶⁴ s.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NtpDate {
    /// The era number: era 0 began in 1900, era 1 begins in 2036.
    pub era: i32,
    /// Seconds since the start of the era.
    pub offset: u32,
    /// The fraction of the second, in units of 2⁻⁶⁴ s.
    pub fraction: u64,
}

/// A 64-bit NTP timestamp: the era offset and a fraction of 2⁻³² s,
/// without the era.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NtpTimestamp {
    /// Seconds since the start of an era that the timestamp does not name.
    pub seconds: u32,
    /// The fraction of the second, in units of 2⁻³² s.
    pub fraction: u32,
}

impl NtpDate {
    /// The date `seconds` from the prime epoch with this fraction, its
    /// era by §6's floor division.
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] when the era does not fit in 32 bits.
    pub fn from_seconds(seconds: i64, fraction: u64) -> TimeResult<Self> {
        let era =
            i32::try_from(seconds.div_euclid(ERA_SECONDS)).map_err(|_| TimeError::OutOfRange)?;
        // In [0, 2³²) by the Euclidean remainder.
        let offset =
            u32::try_from(seconds.rem_euclid(ERA_SECONDS)).map_err(|_| TimeError::OutOfRange)?;
        Ok(Self {
            era,
            offset,
            fraction,
        })
    }

    /// Whole seconds from the prime epoch, §6's `era * 2^(32) + timestamp`.
    #[must_use]
    pub const fn seconds(self) -> i64 {
        self.era as i64 * ERA_SECONDS + self.offset as i64
    }

    /// The NTP date of a POSIX time, the fraction floored to 2⁻⁶⁴ s.
    ///
    /// # Errors
    ///
    /// As [`Self::from_seconds`], which no `UnixTime` reaches.
    pub fn from_unix(unix: UnixTime) -> TimeResult<Self> {
        let seconds = unix
            .seconds()
            .checked_sub(PRIME_EPOCH_UNIX)
            .ok_or(TimeError::Overflow)?;
        // attos < 10¹⁸, so attos · 2⁶⁴ < 1.9 × 10³⁷, inside u128.
        let fraction = (u128::from(unix.subsec_attos()) << 64) / u128::from(ATTOS_PER_SEC);
        Self::from_seconds(
            seconds,
            u64::try_from(fraction).map_err(|_| TimeError::Overflow)?,
        )
    }

    /// The POSIX time of this date, the fraction floored to the
    /// attosecond.
    ///
    /// # Errors
    ///
    /// [`TimeError::Overflow`] when the second is outside `i64`.
    pub fn to_unix(self) -> TimeResult<UnixTime> {
        let seconds = self
            .seconds()
            .checked_add(PRIME_EPOCH_UNIX)
            .ok_or(TimeError::Overflow)?;
        // fraction < 2⁶⁴, so fraction · 10¹⁸ < 1.9 × 10³⁷ and the quotient
        // is below 10¹⁸.
        let attos = (u128::from(self.fraction) * u128::from(ATTOS_PER_SEC)) >> 64;
        UnixTime::new(
            seconds,
            u64::try_from(attos).map_err(|_| TimeError::Overflow)?,
        )
    }

    /// The 64-bit timestamp of this date: the era dropped and the fraction
    /// cut to its top 32 bits.
    #[must_use]
    pub const fn timestamp(self) -> NtpTimestamp {
        NtpTimestamp {
            seconds: self.offset,
            fraction: (self.fraction >> 32) as u32,
        }
    }

    /// The date in the big-endian layout of RFC 5905's Figure 3: era,
    /// era offset, fraction.
    #[must_use]
    pub fn to_bytes(self) -> [u8; 16] {
        let mut out = [0; 16];
        out[..4].copy_from_slice(&self.era.to_be_bytes());
        out[4..8].copy_from_slice(&self.offset.to_be_bytes());
        out[8..].copy_from_slice(&self.fraction.to_be_bytes());
        out
    }

    /// Read the layout of [`Self::to_bytes`].
    #[must_use]
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        let [a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] = bytes;
        Self {
            era: i32::from_be_bytes([a, b, c, d]),
            offset: u32::from_be_bytes([e, f, g, h]),
            fraction: u64::from_be_bytes([i, j, k, l, m, n, o, p]),
        }
    }
}

impl NtpTimestamp {
    /// Whether this is the zero timestamp, which §6 reserves for unknown
    /// or unsynchronized time.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        self.seconds == 0 && self.fraction == 0
    }

    /// The date of this timestamp in the era that puts it within 2³¹ s of
    /// the reference: from 2³¹ s before the reference, included, to 2³¹ s
    /// after it, excluded.
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] for the zero timestamp, which names no
    /// time, or when the era does not fit in 32 bits.
    pub fn resolve(self, reference: UnixTime) -> TimeResult<NtpDate> {
        if self.is_unknown() {
            return Err(TimeError::OutOfRange);
        }
        let reference = reference
            .seconds()
            .checked_sub(PRIME_EPOCH_UNIX)
            .ok_or(TimeError::Overflow)?;
        // The signed 32-bit difference is the step within the window.
        let step = self.seconds.wrapping_sub(reference as u32) as i32;
        let seconds = reference
            .checked_add(i64::from(step))
            .ok_or(TimeError::Overflow)?;
        NtpDate::from_seconds(seconds, u64::from(self.fraction) << 32)
    }

    /// The timestamp in its big-endian wire layout: seconds, then fraction.
    #[must_use]
    pub fn to_bytes(self) -> [u8; 8] {
        let mut out = [0; 8];
        out[..4].copy_from_slice(&self.seconds.to_be_bytes());
        out[4..].copy_from_slice(&self.fraction.to_be_bytes());
        out
    }

    /// Read the layout of [`Self::to_bytes`].
    #[must_use]
    pub fn from_bytes(bytes: [u8; 8]) -> Self {
        let [a, b, c, d, e, f, g, h] = bytes;
        Self {
            seconds: u32::from_be_bytes([a, b, c, d]),
            fraction: u32::from_be_bytes([e, f, g, h]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The NTP seconds of midnight at the start of a Modified Julian Day:
    /// MJD 15 020 is 1 January 1900.
    fn midnight(mjd: i64) -> i64 {
        (mjd - 15_020) * 86_400
    }

    /// RFC 5905, Figure 4, "Interesting Historic NTP Dates": each row's
    /// MJD gives its era and era offset. Two rows carry typos that errata
    /// held for the next revision correct, and the corrected values are the
    /// ones computed: erratum 4025, the MJD of 1 January 0, printed
    /// −678 491 for −678 941; and erratum 6524, the era offset of
    /// 1 January 1, printed 202 939 144 for 202 934 144.
    #[test]
    fn figure_4_of_rfc_5905() {
        for (date, mjd, era, offset) in [
            ("1 Jan -4712", -2_400_001, -49, 1_795_583_104),
            ("1 Jan -1", -679_306, -14, 139_775_744),
            ("1 Jan 0, erratum 4025", -678_941, -14, 171_311_744),
            ("1 Jan 1, erratum 6524", -678_575, -14, 202_934_144),
            ("4 Oct 1582", -100_851, -3, 2_873_647_488),
            ("15 Oct 1582", -100_840, -3, 2_874_597_888),
            ("31 Dec 1899", 15_019, -1, 4_294_880_896),
            ("1 Jan 1900", 15_020, 0, 0),
            ("1 Jan 1970", 40_587, 0, 2_208_988_800),
            ("1 Jan 1972", 41_317, 0, 2_272_060_800),
            ("31 Dec 1999", 51_543, 0, 3_155_587_200),
            ("8 Feb 2036", 64_731, 1, 63_104),
        ] {
            let date_value = NtpDate::from_seconds(midnight(mjd), 0).expect("fits");
            assert_eq!((date_value.era, date_value.offset), (era, offset), "{date}");
            assert_eq!(date_value.seconds(), midnight(mjd), "{date}");
        }
        // The printed values are a day and 5 000 s from the computed ones.
        assert_ne!(
            NtpDate::from_seconds(midnight(-678_491), 0).map(|d| d.offset),
            Ok(171_311_744)
        );
        assert_eq!(202_939_144 - 202_934_144, 5_000);
    }

    #[test]
    fn the_unix_epoch_is_2_208_988_800_seconds_into_era_0() {
        let date = NtpDate::from_unix(UnixTime::EPOCH).expect("fits");
        assert_eq!(
            date,
            NtpDate {
                era: 0,
                offset: 2_208_988_800,
                fraction: 0
            }
        );
        assert_eq!(date.to_unix(), Ok(UnixTime::EPOCH));
    }

    /// 2³² s after 1900 is 2036-02-07T06:28:16Z, POSIX 2 085 978 496, and
    /// era 1 starts there; the day after it begins 63 104 s in, Figure 4's
    /// "First day NTP Era 1".
    #[test]
    fn the_timestamp_wraps_on_7_february_2036() {
        let wrap = UnixTime::from_seconds(2_085_978_496);
        let last = NtpDate::from_unix(UnixTime::from_seconds(2_085_978_495)).expect("fits");
        assert_eq!((last.era, last.offset), (0, u32::MAX));
        let first = NtpDate::from_unix(wrap).expect("fits");
        assert_eq!((first.era, first.offset), (1, 0));
        assert_eq!(2_085_978_496 - PRIME_EPOCH_UNIX, ERA_SECONDS);
        // 8 February 2036 00:00 is MJD 64 731.
        assert_eq!((64_731_i64 - 40_587) * 86_400 - 2_085_978_496, 63_104);
    }

    /// The same 64-bit timestamp is 1900 or 2036 by the reference: from
    /// 2030 the nearer is 2036, and from 1920 it is 1900. A reference in
    /// 1990 is within 68 years of both 1922 and 2058, so it reads 2036
    /// too.
    #[test]
    fn a_timestamp_is_resolved_against_a_reference() {
        let stamp = NtpTimestamp {
            seconds: 63_104,
            fraction: 0,
        };
        let in_2030 = UnixTime::from_seconds(1_893_456_000);
        let in_1920 = UnixTime::from_seconds(-1_577_923_200);
        let in_1990 = UnixTime::from_seconds(631_152_000);
        let resolved = stamp.resolve(in_2030).expect("resolves");
        assert_eq!((resolved.era, resolved.offset), (1, 63_104));
        assert_eq!(
            resolved.to_unix(),
            Ok(UnixTime::from_seconds(2_086_041_600))
        );
        assert_eq!(stamp.resolve(in_1990), stamp.resolve(in_2030));
        let resolved = stamp.resolve(in_1920).expect("resolves");
        assert_eq!((resolved.era, resolved.offset), (0, 63_104));
        assert_eq!(
            resolved.to_unix(),
            Ok(UnixTime::from_seconds(PRIME_EPOCH_UNIX + 63_104))
        );
    }

    /// The window is 2³¹ s each side of the reference, the lower end
    /// included.
    #[test]
    fn the_window_is_half_an_era_each_side() {
        let reference = UnixTime::from_seconds(0);
        let reference_seconds = 2_208_988_800_i64;
        let at = |seconds: i64| {
            NtpDate::from_seconds(seconds, 0)
                .expect("fits")
                .timestamp()
                .resolve(reference)
                .expect("resolves")
                .seconds()
        };
        let half = ERA_SECONDS / 2;
        assert_eq!(at(reference_seconds - half), reference_seconds - half);
        assert_eq!(
            at(reference_seconds + half - 1),
            reference_seconds + half - 1
        );
        // One more and it lands a whole era back.
        assert_eq!(
            at(reference_seconds + half),
            reference_seconds + half - ERA_SECONDS
        );
    }

    #[test]
    fn the_zero_timestamp_is_unknown_and_refused() {
        let zero = NtpTimestamp::default();
        assert!(zero.is_unknown());
        assert_eq!(zero.resolve(UnixTime::EPOCH), Err(TimeError::OutOfRange));
    }

    #[test]
    fn fractions_convert_within_one_unit() {
        let half = UnixTime::new(0, ATTOS_PER_SEC / 2).expect("valid");
        let date = NtpDate::from_unix(half).expect("fits");
        assert_eq!(date.fraction, 1 << 63);
        assert_eq!(date.timestamp().fraction, 1 << 31);
        assert_eq!(date.to_unix(), Ok(half));
        let odd = UnixTime::new(1_700_000_000, 123_456_789_123_456_789).expect("valid");
        let back = NtpDate::from_unix(odd)
            .and_then(NtpDate::to_unix)
            .expect("round trip");
        assert_eq!(back.seconds(), odd.seconds());
        assert!(odd.subsec_attos() - back.subsec_attos() <= 1);
    }

    #[test]
    fn the_wire_layouts_round_trip() {
        let date = NtpDate {
            era: -3,
            offset: 2_874_597_888,
            fraction: 0x0123_4567_89AB_CDEF,
        };
        let bytes = date.to_bytes();
        assert_eq!(bytes[..4], [0xFF, 0xFF, 0xFF, 0xFD]);
        assert_eq!(NtpDate::from_bytes(bytes), date);
        let stamp = date.timestamp();
        assert_eq!(stamp.fraction, 0x0123_4567);
        assert_eq!(NtpTimestamp::from_bytes(stamp.to_bytes()), stamp);
    }
}
