//! Well-known epochs, all expressed as TAI readings from the 1970 TAI epoch.
//!
//! Systems disagree about where time starts, and most of those choices are
//! historical accidents. Collecting them here means a conversion never has to
//! rediscover a magic number.
//!
//! Every epoch names the document that defines it in [`Epoch::source`].
//! Three of them — FILETIME's 1601, the Modified Julian Date's 1858 and
//! NTP's 1900 — are written with a `Z` although they predate UTC, which
//! began in 1961: the label names the day on today's proleptic UTC
//! calendar, and the TAI reading is the arithmetic extension of it, not
//! what any clock read.

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
    /// The document that defines the epoch, keyed to `docs/references.bib`
    /// where it has an entry.
    pub source: &'static str,
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
    source: "IEEE Std 1003.1-2024 (POSIX), Base Definitions, 4.19 Seconds Since the Epoch \
        [posix-2024]; TAI - UTC of 8.000082 s from USNO tai-utc.dat, 1968 FEB 1 segment \
        [usno-tai-utc]",
};

/// `1980-01-06T00:00:00Z`, the GPS epoch.
pub const GPS: Epoch = Epoch {
    id: "gps",
    description: "GPS week zero, 1980-01-06T00:00:00Z",
    tai_reading: Duration::from_secs(315_964_800 + 19),
    source: "IS-GPS-200, 3.3.4 GPS Time and SV Z-Count: zero time-point at midnight of \
        5/6 January 1980, UTC(USNO), as quoted by the ARL memorandum of 15 August 2019 \
        [arl-gps-time-2019]; 19 s behind TAI [rots2015]",
};

/// `1999-08-21T23:59:47Z`, week zero of Galileo System Time.
///
/// The ICD states it as 1999-08-22 00:00:00 UTC less 13 s; `TAI − UTC` was
/// 32 s, so the TAI reading is the midnight's POSIX second plus 19 s, and
/// GST reads its epoch as 1999-08-22 00:00:00.
pub const GALILEO: Epoch = Epoch {
    id: "galileo-system-time",
    description: "Galileo System Time week zero, 1999-08-22T00:00:00 GST, 1999-08-21T23:59:47Z",
    tai_reading: Duration::from_secs(935_280_000 + 19),
    source: "Galileo OS SIS ICD, Issue 2.1 (2023), 5.1.2: GST epoch 1999-08-22 00:00:00 UTC less \
        13 s [galileo-os-sis-icd-2-1]; TAI - UTC of 32 s from the \
        leap-second table [iana-leap-seconds-list]",
};

/// `2006-01-01T00:00:00Z`, week zero of BeiDou Time.
pub const BEIDOU: Epoch = Epoch {
    id: "beidou-time",
    description: "BeiDou Time week zero, 2006-01-01T00:00:00 UTC",
    tai_reading: Duration::from_secs(1_136_073_600 + 33),
    source: "BDS-SIS-ICD-B1I-1.0 (2012), 3.3: BDT from 00:00:00 on 1 January 2006 UTC, \
        with no leap seconds [bds-sis-icd-b1i-1-0]; TAI - UTC of 33 s from the \
        leap-second table [iana-leap-seconds-list]",
};

/// `1999-08-21T23:59:47Z`, week zero of NavIC system time: the same
/// instant as [`GALILEO`], reached from a different document.
pub const NAVIC: Epoch = Epoch {
    id: "navic-time",
    description: "NavIC (IRNSS) system time week zero, 1999-08-22T00:00:00 NavIC, \
        1999-08-21T23:59:47Z",
    tai_reading: Duration::from_secs(935_280_000 + 19),
    source: "ISRO, IRNSS SIS ICD for SPS, version 1.1 (2017): 00:00 on 22 August 1999 of \
        its own reckoning, 23:59:47 UTC on 21 August 1999 [irnss-sps-icd-1-1]",
};

/// `1995-12-31T21:00:00Z`, 00:00 on 1 January 1996 in GLONASS time,
/// the start of the four-year interval *N*4 = 1.
///
/// GLONASS time is UTC(SU) + 3 h and takes leap seconds, so this is an
/// origin for its date fields, not the zero of a uniform count. The
/// leap second of 1996 came at the following UTC midnight, so `TAI − UTC`
/// was still 29 s.
pub const GLONASS: Epoch = Epoch {
    id: "glonass-time",
    description: "GLONASS four-year interval N4 = 1, 1996-01-01T00:00:00 UTC(SU) + 3 h, \
        1995-12-31T21:00:00Z",
    tai_reading: Duration::from_secs(820_443_600 + 29),
    source: "GLONASS ICD, Edition 5.1 (2008), 3.3.3 and 4: GLONASS time is UTC(SU) + 3 h; \
        N4 counts four-year intervals from 1996 [glonass-icd-5-1]; TAI - UTC of 29 s from \
        the leap-second table [iana-leap-seconds-list]",
};

/// `2000-01-01T12:00:00 TT`, the J2000.0 fundamental epoch of modern
/// astronomy.
pub const J2000: Epoch = Epoch {
    id: "j2000",
    description: "J2000.0, 2000-01-01T12:00:00 TT",
    // 2000-01-01T12:00:00 TT is exactly 946_728_000 s after
    // 1970-01-01T00:00:00 TT: 10_957 days of 86_400 s plus twelve hours. The
    // same event's TAI label is 11:59:27.816, because TT - TAI is 32.184 s
    // exactly, so the TAI reading is 32.184 s less.
    tai_reading: Duration::from_attos(946_727_967_816_000_000_000_000_000),
    source: "J2000.0 = 2000-01-01T12:00:00, JD 2451545.0 [rots2015, Table 1, which gives it \
        in TDB]; read here in TT, as hc-astro counts Julian centuries of TT from it; \
        TT = TAI + 32.184 s [rots2015, Table 2]",
};

/// `1977-01-01T00:00:00 TAI`, the origin of TCG and TCB.
pub const TCG_TCB_ORIGIN: Epoch = Epoch {
    id: "tcg-tcb-origin",
    description: "1977-01-01T00:00:00 TAI, the defining origin of TCG and TCB",
    tai_reading: Duration::from_secs(220_924_800),
    source: "JD0 = 2443144.5003725 TT, 1977-01-01T00:00:00 TAI, in the TCG and TDB relations \
        of IAU 1991 Resolution A4 and IAU 2006 Resolution B3 as Rots et al. state them \
        [rots2015]; the resolutions themselves not read",
};

/// `1858-11-17T00:00:00 UT`, the origin of the Modified Julian Date.
pub const MJD: Epoch = Epoch {
    id: "mjd",
    description: "Modified Julian Date zero, 1858-11-17T00:00:00 UT",
    tai_reading: Duration::from_secs(-3_506_716_800),
    source: "MJD = JD - 2400000.5, citing IAU 1997 [rots2015]",
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
    source: "Julian Dates counted from noon, 1 January 4713 BCE, proleptic Julian \
        [rots2015, usno-julian-date]",
};

/// `0001-01-01T00:00:00`, the origin of .NET ticks and Rata Die day 1.
pub const RATA_DIE: Epoch = Epoch {
    id: "rata-die",
    description: "Rata Die day 1, 0001-01-01 (proleptic Gregorian)",
    tai_reading: Duration::from_secs(-62_135_596_800),
    source: "Rata Die day 1 is 1 January 1 of the proleptic Gregorian calendar, \
        gregorian-epoch in reingold2018code",
};

/// `1601-01-01T00:00:00Z`, the origin of the Windows `FILETIME`.
pub const WINDOWS_FILETIME: Epoch = Epoch {
    id: "windows-filetime",
    description: "Windows FILETIME origin, 1601-01-01T00:00:00Z",
    tai_reading: Duration::from_secs(-11_644_473_600),
    source: "Microsoft Learn, FILETIME structure (minwinbase.h): 100-nanosecond intervals \
        since January 1, 1601 (UTC), retrieved 2026-09-26 [ms-filetime]",
};

/// `1900-01-01T00:00:00Z`, the origin of NTP time.
pub const NTP: Epoch = Epoch {
    id: "ntp",
    description: "NTP era zero, 1900-01-01T00:00:00Z",
    tai_reading: Duration::from_secs(-2_208_988_800),
    source: "RFC 5905, section 6: the prime epoch, or base date of era 0, is 0 h 1 January \
        1900 UTC [rfc5905]",
};

/// `2001-01-01T00:00:00Z`, the origin of Apple's Core Foundation absolute
/// time.
pub const CORE_FOUNDATION: Epoch = Epoch {
    id: "core-foundation",
    description: "Core Foundation absolute time origin, 2001-01-01T00:00:00Z",
    tai_reading: Duration::from_secs(978_307_200 + 32),
    source: "Apple Developer Documentation, CFAbsoluteTime: the absolute reference date of \
        1 Jan 2001 00:00:00 GMT, retrieved 2026-09-26 [apple-cfabsolutetime]",
};

/// Every epoch this crate knows about.
pub const ALL: &[Epoch] = &[
    UNIX,
    GPS,
    GALILEO,
    BEIDOU,
    NAVIC,
    GLONASS,
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
        // 10_957 days from 1970-01-01 to 2000-01-01, plus twelve hours. The
        // round number is the point: if the epoch is right, the TT reading is
        // exact, and any slip shows up immediately.
        let tt: Instant<Tt> = J2000.instant().convert();
        assert_eq!(tt.since_epoch(), Duration::from_secs(946_728_000));
    }

    #[test]
    fn j2000_agrees_with_the_utc_path() {
        // The same instant reached the other way: 2000-01-01T12:00:00 TT is
        // 11:59:27.816 TAI, and TAI - UTC was 32 s in 2000, so it is
        // 11:58:55.816 UTC. Deriving the epoch through the leap-second table
        // instead of through the TT offset is what catches a constant that
        // was built by applying the wrong correction in the wrong direction.
        let unix = crate::unix::UnixTime::new(946_727_935, 816_000_000_000_000_000).unwrap();
        let from_utc = crate::unix::tai_from_unix(unix, crate::unix::LeapPolicy::Strict).unwrap();
        assert_eq!(from_utc, J2000.instant());
    }

    #[test]
    fn the_gps_epoch_is_nineteen_seconds_of_tai_after_its_utc_label() {
        let gps_utc_seconds = 315_964_800i128;
        assert_eq!(GPS.tai_reading.whole_seconds() - gps_utc_seconds, 19);
    }

    /// Each GNSS epoch reached from its UTC label through the leap-second
    /// table, the second statement of the constants above.
    #[test]
    fn every_gnss_epoch_is_its_utc_label_read_through_the_leap_table() {
        use crate::scale::{BeidouTime, GalileoTime, Gps, NavicTime};
        use crate::unix::{LeapPolicy, UnixTime, tai_from_unix};
        let tai = |unix: i64| {
            tai_from_unix(UnixTime::from_seconds(unix), LeapPolicy::Strict)
                .expect("inside the table")
        };
        // (epoch, POSIX second of its UTC label)
        for (epoch, unix) in [
            (GPS, 315_964_800),
            (GALILEO, 935_280_000 - 13),
            (BEIDOU, 1_136_073_600),
            (NAVIC, 935_280_000 - 13),
            (GLONASS, 820_454_400 - 3 * 3_600),
        ] {
            assert_eq!(epoch.instant(), tai(unix), "{}", epoch.id);
        }
        // Each scale reads its own week zero as the label it was named by.
        let gps: Instant<Gps> = GPS.instant().convert();
        assert_eq!(gps.since_epoch(), Duration::from_secs(315_964_800));
        let galileo: Instant<GalileoTime> = GALILEO.instant().convert();
        assert_eq!(galileo.since_epoch(), Duration::from_secs(935_280_000));
        let navic: Instant<NavicTime> = NAVIC.instant().convert();
        assert_eq!(navic.since_epoch(), Duration::from_secs(935_280_000));
        let beidou: Instant<BeidouTime> = BEIDOU.instant().convert();
        assert_eq!(beidou.since_epoch(), Duration::from_secs(1_136_073_600));
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

crate::catalogue_tests! {
    type: Epoch,
    id: |epoch| epoch.id,
    provenance: |epoch| epoch.source,
    tests: epoch_table_tests,
    all: ALL,
    lookup: by_id,
}
