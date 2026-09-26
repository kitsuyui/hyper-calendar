//! Swatch Internet Time: the day in a thousand *.beats*, @000 to @999, the
//! same everywhere.
//!
//! Swatch, "Internet Time", swatch.com, read 2026-09-26
//! (`swatch-internet-time`): "We have divided up the day into 1000
//! '.beats'. So, one Swatch '.Beat' is equivalent to 1 Minute 26.4
//! Seconds", and "A day in internet time begins at midnight BMT (@000
//! Swatch .Beats) (Central European Wintertime)". Biel Mean Time is not the
//! mean solar time of Biel's meridian but UTC+1, and it keeps no summer
//! time; the reading is ⌊(3600 h + 60 m + s) / 86.4⌋ from the hours,
//! minutes and seconds of UTC+1 (Wikipedia, "Swatch Internet Time",
//! read 2026-09-26, `wikipedia-swatch-internet-time`, which also gives the
//! launch on 23 October 1998).
//!
//! The reading is taken from POSIX time, as a clock showing it would, so a
//! leap second is read as the second after it: the day has 86 400 s and
//! each beat exactly 86.4 of them. Swatch defines no unit below the beat;
//! the "centibeats" some programs add are not carried. The `beat` unit
//! itself, 86.4 s, is in `hc-units`.

use core::fmt;

use crate::duration::{ATTOS_PER_SEC, Duration};
use crate::error::{TimeError, TimeResult};
use crate::unix::UnixTime;

/// Biel Mean Time's offset from UTC, one hour, all year.
pub const BMT_OFFSET_SECONDS: i64 = 3_600;

/// Beats in a day.
pub const BEATS_PER_DAY: u16 = 1_000;

/// One beat, 86.4 s, in milliseconds.
const MILLIS_PER_BEAT: u64 = 86_400;

/// A reading of Swatch Internet Time, @000 to @999.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Beat(u16);

impl Beat {
    /// A reading, refused from 1 000.
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] for a value above 999.
    pub const fn new(beat: u16) -> TimeResult<Self> {
        if beat >= BEATS_PER_DAY {
            return Err(TimeError::OutOfRange);
        }
        Ok(Self(beat))
    }

    /// The reading at a POSIX time.
    #[must_use]
    pub const fn at(unix: UnixTime) -> Self {
        let second_of_day = (unix.seconds().rem_euclid(86_400) + BMT_OFFSET_SECONDS) % 86_400;
        // Below 86 400 000, so the quotient is below 1 000.
        let millis = second_of_day as u64 * 1_000 + unix.subsec_attos() / (ATTOS_PER_SEC / 1_000);
        Self((millis / MILLIS_PER_BEAT) as u16)
    }

    /// The number, 0 to 999.
    #[must_use]
    pub const fn value(self) -> u16 {
        self.0
    }

    /// When this beat begins, after midnight BMT: `beat` × 86.4 s, exactly.
    #[must_use]
    pub const fn start_after_bmt_midnight(self) -> Duration {
        Duration::from_millis(self.0 as i128 * MILLIS_PER_BEAT as i128)
    }

    /// When this beat begins, after midnight UTC: an hour earlier than
    /// after midnight BMT, so @000 is 23:00 UTC of the day before and
    /// this is [`Duration::DAY`] less an hour.
    #[must_use]
    pub const fn start_after_utc_midnight(self) -> Duration {
        let millis = (self.0 as i128 * MILLIS_PER_BEAT as i128
            - BMT_OFFSET_SECONDS as i128 * 1_000)
            .rem_euclid(86_400_000);
        Duration::from_millis(millis)
    }
}

impl fmt::Display for Beat {
    /// Swatch's notation, `@` and three digits: `@000`, `@248`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{:03}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString as _;

    extern crate alloc;

    /// Wikipedia's example: "@248" is "04:57:07.2 UTC", 248 × 86.4 s after
    /// midnight BMT.
    #[test]
    fn at_248_is_04_57_07_2_utc() {
        let beat = Beat::new(248).expect("in range");
        assert_eq!(
            beat.start_after_utc_midnight(),
            Duration::from_millis(((4 * 60 + 57) * 60 + 7) * 1_000 + 200)
        );
        let start = UnixTime::new(4 * 3_600 + 57 * 60 + 7, 200_000_000_000_000_000).expect("valid");
        assert_eq!(Beat::at(start), beat);
        let before =
            UnixTime::new(4 * 3_600 + 57 * 60 + 7, 199_999_999_999_999_999).expect("valid");
        assert_eq!(Beat::at(before).value(), 247);
        assert_eq!(beat.to_string(), "@248");
    }

    /// Midnight BMT is 23:00 UTC, @000, as Wikipedia's table has London
    /// at 23:00 GMT; the POSIX epoch, 00:00 UTC, is 01:00 BMT, @041.
    #[test]
    fn midnight_bmt_is_23_00_utc() {
        assert_eq!(
            Beat::at(UnixTime::from_seconds(-3_600)),
            Beat::new(0).expect("in range")
        );
        assert_eq!(
            Beat::at(UnixTime::from_seconds(23 * 3_600)).to_string(),
            "@000"
        );
        assert_eq!(
            Beat::at(UnixTime::from_seconds(23 * 3_600 - 1)).to_string(),
            "@999"
        );
        assert_eq!(Beat::at(UnixTime::EPOCH).value(), 41);
        assert_eq!(
            Beat::new(0).expect("in range").start_after_utc_midnight(),
            Duration::from_hours(23)
        );
    }

    /// No summer time: the reading follows UTC all year, in January and in
    /// July alike.
    #[test]
    fn the_reading_ignores_summer_time() {
        // 2026-01-15 and 2026-07-15, both 12:00 UTC.
        for noon in [1_768_478_400, 1_784_116_800] {
            assert_eq!(Beat::at(UnixTime::from_seconds(noon)).value(), 541);
        }
    }

    #[test]
    fn a_thousand_is_refused() {
        assert_eq!(Beat::new(1_000), Err(TimeError::OutOfRange));
        assert_eq!(Beat::new(999).map(Beat::value), Ok(999));
        assert_eq!(
            Beat::new(999).expect("in range").start_after_bmt_midnight(),
            Duration::from_millis(86_313_600)
        );
    }
}
