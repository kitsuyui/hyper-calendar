//! Well-known epochs, all expressed as TAI readings from the 1970 TAI epoch.
//!
//! Systems disagree about where time starts, and most of those choices are
//! historical accidents. Collecting them here means a conversion never has to
//! rediscover a magic number.

use crate::duration::Duration;
use crate::scale::{Instant, Tai};

/// A named origin of some timekeeping system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Epoch {
    /// Short identifier, for example `"unix"` or `"j2000"`.
    pub id: &'static str,
    /// Human-readable description of what the epoch is.
    pub description: &'static str,
    /// The TAI reading of the epoch, measured from `1970-01-01T00:00:00 TAI`.
    pub tai_reading: Duration,
}

impl Epoch {
    /// The epoch as a TAI instant.
    #[must_use]
    pub const fn instant(self) -> Instant<Tai> {
        Instant::from_epoch(self.tai_reading)
    }
}

/// `1970-01-01T00:00:00Z`, the POSIX epoch. `TAI - UTC` was 8.000082 s.
pub const UNIX: Epoch = Epoch {
    id: "unix",
    description: "POSIX time_t origin, 1970-01-01T00:00:00Z",
    tai_reading: Duration::from_attos(8_000_082_000_000_000_000),
};

/// `1980-01-06T00:00:00Z`, the GPS epoch.
pub const GPS: Epoch = Epoch {
    id: "gps",
    description: "GPS week zero, 1980-01-06T00:00:00Z",
    tai_reading: Duration::from_secs(315_964_800 + 19),
};

/// `2000-01-01T12:00:00 TT`, the J2000.0 fundamental epoch of modern
/// astronomy.
pub const J2000: Epoch = Epoch {
    id: "j2000",
    description: "J2000.0, 2000-01-01T12:00:00 TT",
    // 2000-01-01T12:00:00 TT is 946_727_967.816 s after 1970-01-01T00:00:00 TT,
    // and TT - TAI is 32.184 s exactly.
    tai_reading: Duration::from_attos(946_727_935_632_000_000_000_000_000),
};

/// `1977-01-01T00:00:00 TAI`, the origin of TCG and TCB.
pub const TCG_TCB_ORIGIN: Epoch = Epoch {
    id: "tcg-tcb-origin",
    description: "1977-01-01T00:00:00 TAI, the defining origin of TCG and TCB",
    tai_reading: Duration::from_secs(220_924_800),
};

/// `1858-11-17T00:00:00 UT`, the origin of the Modified Julian Date.
pub const MJD: Epoch = Epoch {
    id: "mjd",
    description: "Modified Julian Date zero, 1858-11-17T00:00:00 UT",
    tai_reading: Duration::from_secs(-3_506_716_800),
};

/// `-4712-01-01T12:00:00 UT` (Julian proleptic), the origin of the Julian Day.
///
/// This predates every atomic scale by millennia, so the TAI reading here is
/// a proleptic extension used only to make arithmetic uniform. It is not a
/// claim about what a clock would have read.
pub const JULIAN_DAY: Epoch = Epoch {
    id: "julian-day",
    description: "Julian Day zero, -4712-01-01T12:00:00 UT (proleptic Julian)",
    tai_reading: Duration::from_secs(-210_866_760_000),
};

/// `0001-01-01T00:00:00`, the origin of .NET ticks and Rata Die day 1.
pub const RATA_DIE: Epoch = Epoch {
    id: "rata-die",
    description: "Rata Die day 1, 0001-01-01 (proleptic Gregorian)",
    tai_reading: Duration::from_secs(-62_135_596_800),
};

/// `1601-01-01T00:00:00Z`, the origin of the Windows `FILETIME`.
pub const WINDOWS_FILETIME: Epoch = Epoch {
    id: "windows-filetime",
    description: "Windows FILETIME origin, 1601-01-01T00:00:00Z",
    tai_reading: Duration::from_secs(-11_644_473_600),
};

/// `1900-01-01T00:00:00Z`, the origin of NTP time.
pub const NTP: Epoch = Epoch {
    id: "ntp",
    description: "NTP era zero, 1900-01-01T00:00:00Z",
    tai_reading: Duration::from_secs(-2_208_988_800),
};

/// `2001-01-01T00:00:00Z`, the origin of Apple's Core Foundation absolute
/// time.
pub const CORE_FOUNDATION: Epoch = Epoch {
    id: "core-foundation",
    description: "Core Foundation absolute time origin, 2001-01-01T00:00:00Z",
    tai_reading: Duration::from_secs(978_307_200 + 32),
};

/// Every epoch this crate knows about.
pub const ALL: &[Epoch] = &[
    UNIX,
    GPS,
    J2000,
    TCG_TCB_ORIGIN,
    MJD,
    JULIAN_DAY,
    RATA_DIE,
    WINDOWS_FILETIME,
    NTP,
    CORE_FOUNDATION,
];

/// Look an epoch up by its identifier.
#[must_use]
pub fn by_id(id: &str) -> Option<Epoch> {
    let mut index = 0;
    while index < ALL.len() {
        if str_eq(ALL[index].id, id) {
            return Some(ALL[index]);
        }
        index += 1;
    }
    None
}

fn str_eq(a: &str, b: &str) -> bool {
    a.as_bytes() == b.as_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scale::Tt;

    #[test]
    fn j2000_reads_noon_on_the_first_of_january_in_tt() {
        let tt: Instant<Tt> = J2000.instant().convert();
        assert_eq!(
            tt.since_epoch(),
            Duration::from_secs(946_727_967) + Duration::from_millis(816)
        );
    }

    #[test]
    fn the_gps_epoch_is_nineteen_seconds_of_tai_after_its_utc_label() {
        let gps_utc_seconds = 315_964_800i128;
        assert_eq!(GPS.tai_reading.whole_seconds() - gps_utc_seconds, 19);
    }

    #[test]
    fn epochs_are_addressable_by_id() {
        assert_eq!(by_id("unix"), Some(UNIX));
        assert_eq!(by_id("j2000"), Some(J2000));
        assert_eq!(by_id("nope"), None);
    }

    #[test]
    fn epoch_identifiers_are_unique() {
        for (index, epoch) in ALL.iter().enumerate() {
            for other in &ALL[index + 1..] {
                assert_ne!(epoch.id, other.id);
            }
        }
    }
}
