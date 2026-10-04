//! Well-known epochs, all expressed as TAI readings from the 1970 TAI epoch.
//!
//! Systems disagree about where time starts, and most of those choices are
//! historical accidents. Collecting them here means a conversion never has to
//! rediscover a magic number.
//!
//! Every epoch names the document that defines it in [`Epoch::source`].
//! Four of them — the UUID's 1582, FILETIME's 1601, NTP's 1900 and SAS's
//! 1960 — are written with a `Z` although they predate UTC, which began in
//! 1961: the label names the day on today's proleptic UTC calendar, and the
//! TAI reading is the arithmetic extension of it, not what any clock read.
//! The Modified Julian Date's 1858 and the Julian Day's 4713 BCE are
//! written in UT and extended the same way.

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
    source: "IS-GPS-200G (2012), 3.3.4 GPS Time and SV Z-Count: zero time-point at midnight \
        of 5/6 January 1980, UTC(USNO) [is-gps-200g], as the ARL memorandum of 15 August \
        2019 also quotes it [arl-gps-time-2019]; 19 s behind TAI [rots2015]",
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
    source: "ISRO, IRNSS SIS ICD for SPS, version 1.1 (2017), 5.7: 00:00 on 22 August 1999 \
        of its own reckoning, 23:59:47 UTC on 21 August 1999 [irnss-sps-icd-1-1]",
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
    source: "GLONASS ICD, Edition 5.1 (2008), 3.3.3 and 4.5: GLONASS time is UTC(SU) + 3 h; \
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

/// The TT reading of [`J2000`] in seconds from `1970-01-01T00:00:00 TT`:
/// 10 957 days of 86 400 s and twelve hours, exactly, which is also the
/// POSIX timestamp of `2000-01-01T12:00:00` read on the TT scale.
pub const J2000_TT_SECONDS: i64 = 946_728_000;

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

/// The Modified Julian Date of 1970-01-01, the day the POSIX count
/// begins: [`MJD`] is 40 587 days before [`UNIX`]'s calendar day.
pub const MJD_OF_UNIX_EPOCH: i64 = 40_587;

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

/// `0001-01-01T00:00:00`, Rata Die day 1. [`DOTNET_TICKS`] is the same
/// instant, reached from Microsoft's documentation.
pub const RATA_DIE: Epoch = Epoch {
    id: "rata-die",
    description: "Rata Die day 1, 0001-01-01 (proleptic Gregorian)",
    tai_reading: Duration::from_secs(-62_135_596_800),
    source: "Rata Die day 1 is 1 January 1 of the proleptic Gregorian calendar, \
        gregorian-epoch in reingold2018code",
};

/// `0001-01-01T00:00:00`, the origin of .NET's `DateTime.Ticks`, in the
/// Gregorian calendar with its leap-year rule applied to every year.
///
/// The ticks count the time of the zone a `DateTime`'s `Kind` names, and
/// not leap seconds; [`crate::dotnet`] reads and writes them. The TAI
/// reading is that of the day's label on today's proleptic UTC calendar,
/// as for the other epochs before 1961.
pub const DOTNET_TICKS: Epoch = Epoch {
    id: "dotnet-ticks",
    description: ".NET DateTime.Ticks origin, 0001-01-01T00:00:00 (Gregorian)",
    tai_reading: Duration::from_secs(-62_135_596_800),
    source: "Microsoft Learn, DateTime.Ticks Property: 100-nanosecond intervals since \
        12:00:00 midnight, January 1, 0001 in the Gregorian calendar, excluding leap \
        seconds, retrieved 2026-09-27 [ms-datetime-ticks]",
};

/// `1958-01-01T00:00:00 TAI`, the Level 1 epoch of the CCSDS Unsegmented
/// Code, whose count is TAI seconds with no leap seconds.
///
/// The Day Segmented Code counts UTC days from the same date; that count is
/// `ccsds-day` in `hc-calendars-solar`, and [`crate::ccsds`] reads both.
pub const CCSDS_CUC: Epoch = Epoch {
    id: "ccsds-cuc",
    description: "CCSDS Unsegmented Code Level 1 epoch, 1958-01-01T00:00:00 TAI",
    tai_reading: Duration::from_secs(-378_691_200),
    source: "CCSDS 301.0-B-4, Time Code Formats (2010), 3.2.1: the CCSDS-Recommended \
        epoch is 1958 January 1 (TAI) [ccsds-301-0-b-4]",
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

/// `1582-10-15T00:00:00Z`, the origin of the timestamps in version 1 and
/// version 6 UUIDs, the first day of the Gregorian calendar.
///
/// RFC 9562 counts 100-nanosecond intervals from it and gives the offset
/// to the POSIX epoch as 122 192 928 000 000 000 intervals, which is
/// 12 219 292 800 s; [`crate::uuid`] reads and writes the count.
pub const UUID_GREGORIAN: Epoch = Epoch {
    id: "uuid-gregorian",
    description: "UUID version 1 and 6 timestamp origin, 1582-10-15T00:00:00Z",
    tai_reading: Duration::from_secs(-12_219_292_800),
    source: "RFC 9562, 5.1: 100-nanosecond intervals since 00:00:00.00, 15 October 1582; \
        Appendix A: Greg_Unix_offset = 0x01b21dd213814000 [rfc9562]",
};

/// `1960-01-01T00:00:00Z`, the origin of SAS date and datetime values and
/// of Stata's `%td`, `%tc` and `%tC`.
///
/// [`crate::sas_stata`] reads and writes the datetimes; the day counts are
/// `sas-date` and `stata-date` in `hc-calendars-solar`.
pub const SAS_STATA: Epoch = Epoch {
    id: "sas-stata",
    description: "SAS and Stata origin, 1960-01-01T00:00:00Z",
    tai_reading: Duration::from_secs(-315_619_200),
    source: "SAS 9.3 Language Reference: Concepts, About SAS Date, Time, and Datetime \
        Values: days and seconds from January 1, 1960 [sas-lrcon-dates]; Stata, help \
        datetime: durations from 01jan1960 [stata-help-datetime]",
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

/// `1582-10-14T00:00:00`, the day before the first Gregorian day, from
/// which IBM SPSS Statistics counts the seconds of a date variable.
///
/// The documentation names no zone; the TAI reading is that of the day's
/// label on today's proleptic UTC calendar, as for the other epochs
/// before 1961.
pub const SPSS: Epoch = Epoch {
    id: "spss",
    description: "IBM SPSS Statistics date origin, 1582-10-14T00:00:00",
    tai_reading: Duration::from_secs(-12_219_379_200),
    source: "IBM SPSS Statistics documentation, Date Variables versus Date Format Variables: \
        most date format variables are stored as the number of seconds from October 14, \
        1582, retrieved 2026-10-04 [ibm-spss-date-variables]",
};

/// `1840-12-31T00:00:00`, day 0 of the MUMPS `$HOROLOG` count, whose
/// first piece is days since that date and whose second is seconds since
/// midnight, in the zone of the process.
///
/// The YottaDB guide's example, `58883,55555` at 15:25:55 on 20 March
/// 2002, is the day count's anchor.
pub const MUMPS_HOROLOG: Epoch = Epoch {
    id: "mumps-horolog",
    description: "MUMPS $HOROLOG day 0, 1840-12-31T00:00:00",
    tai_reading: Duration::from_secs(-4_070_908_800),
    source: "YottaDB M Programmer's Guide, chapter 8, Intrinsic Special Variables, $HOROLOG: \
        the number of days since December 31, 1840, and the number of seconds since midnight \
        of that date in the time zone of the process, retrieved 2026-10-04 \
        [yottadb-isv-horolog]",
};

/// `1904-01-01T00:00:00`, the origin of the classic Mac OS date-time: an
/// unsigned 32-bit count of seconds in the machine's local time, which
/// runs out at 06:28:15 on 6 February 2040.
pub const CLASSIC_MAC_OS: Epoch = Epoch {
    id: "classic-mac-os",
    description: "Classic Mac OS date-time origin, 1904-01-01T00:00:00",
    tai_reading: Duration::from_secs(-2_082_844_800),
    source: "Inside Macintosh: Operating System Utilities, chapter 4, Date, Time, and \
        Measurement Utilities: the number of seconds elapsed since midnight, January 1, \
        1904, in 4 bytes, retrieved 2026-10-04 [apple-inside-macintosh-date-time]",
};

/// `1904-01-01T00:00:00Z`, the origin of the LabVIEW time stamp, a 64-bit
/// signed count of seconds and a 64-bit fraction: the same instant as
/// [`CLASSIC_MAC_OS`], but stated in UTC.
pub const LABVIEW: Epoch = Epoch {
    id: "labview",
    description: "LabVIEW time stamp origin, 1904-01-01T00:00:00Z",
    tai_reading: Duration::from_secs(-2_082_844_800),
    source: "NI, LabVIEW Timestamp Overview: a 128-bit type of (i64) seconds since the epoch \
        01/01/1904 00:00:00.00 UTC and (u64) positive fractions of a second, retrieved \
        2026-10-04 [ni-labview-timestamp]",
};

/// `1978-01-01T00:00:00Z`, the origin of the AmigaOS system time, a count
/// of seconds and microseconds. `TAI - UTC` was 17 s.
pub const AMIGAOS: Epoch = Epoch {
    id: "amigaos",
    description: "AmigaOS system time origin, 1978-01-01T00:00:00Z",
    tai_reading: Duration::from_secs(252_460_800 + 17),
    source: "AmigaOS Documentation Wiki, Timer Device: by convention, how many seconds have \
        passed since midnight, January 1, 1978, retrieved 2026-10-04 [amigaos-timer-device]",
};

/// `1989-12-31T00:00:00Z`, the origin of the FIT `date_time`, a count of
/// seconds 631 065 600 s after the POSIX epoch. `TAI - UTC` was 24 s.
pub const GARMIN_FIT: Epoch = Epoch {
    id: "garmin-fit",
    description: "Garmin FIT date_time origin, 1989-12-31T00:00:00Z",
    tai_reading: Duration::from_secs(631_065_600 + 24),
    source: "Garmin, fit-javascript-sdk, src/utils.js: FIT_EPOCH_MS = 631065600000, a \
        date_time being seconds added to it, retrieved 2026-10-04 [garmin-fit-javascript-sdk]",
};

/// `2000-01-01T00:00:00Z`, the origin of PostgreSQL's `timestamp` types,
/// stored as microseconds before or after it in an eight-byte integer.
/// `TAI - UTC` was 32 s.
pub const POSTGRESQL: Epoch = Epoch {
    id: "postgresql",
    description: "PostgreSQL timestamp origin, 2000-01-01T00:00:00Z",
    tai_reading: Duration::from_secs(946_684_800 + 32),
    source: "PostgreSQL 9.1 Documentation, 8.5 Date/Time Types: timestamp values are stored \
        as seconds before or after midnight 2000-01-01, with microsecond precision as \
        eight-byte integers, retrieved 2026-10-04 [postgresql-9-1-datetime]",
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
    DOTNET_TICKS,
    CCSDS_CUC,
    WINDOWS_FILETIME,
    NTP,
    CORE_FOUNDATION,
    UUID_GREGORIAN,
    SAS_STATA,
    SPSS,
    MUMPS_HOROLOG,
    CLASSIC_MAC_OS,
    LABVIEW,
    AMIGAOS,
    GARMIN_FIT,
    POSTGRESQL,
];

/// Look an epoch up by its identifier, by [`crate::catalogue::matches`].
#[must_use]
pub fn by_id(id: &str) -> Option<Epoch> {
    ALL.iter()
        .copied()
        .find(|epoch| crate::catalogue::matches(id, epoch.id))
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
        assert_eq!(
            tt.since_epoch(),
            Duration::from_secs(i128::from(J2000_TT_SECONDS))
        );
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

    /// The proleptic labels are whole days before the POSIX epoch: the
    /// UUID's 141 427 days, which RFC 9562's offset of
    /// 122 192 928 000 000 000 intervals of 100 ns states again, and the
    /// 3 653 days from 1960 to 1970.
    #[test]
    fn the_uuid_and_sas_epochs_are_whole_days_before_1970() {
        assert_eq!(
            UUID_GREGORIAN.tai_reading,
            Duration::from_days(-141_427),
            "1582-10-15 is 141 427 days before 1970-01-01"
        );
        assert_eq!(
            UUID_GREGORIAN.tai_reading.whole_seconds() * 10_000_000,
            -122_192_928_000_000_000
        );
        assert_eq!(SAS_STATA.tai_reading, Duration::from_days(-3_653));
    }

    /// The .NET origin is Rata Die day 1, 719 162 days before 1970, which
    /// Microsoft's page states as ticks: `DateTime.MaxValue`, 23:59:59.9999999
    /// on 31 December 9999, is 3 155 378 975 999 999 999 of them. The CCSDS
    /// epoch is 4 383 days before 1970, twelve years with three leap days.
    #[test]
    fn the_dotnet_and_ccsds_epochs_are_whole_days_before_1970() {
        assert_eq!(DOTNET_TICKS.tai_reading, RATA_DIE.tai_reading);
        assert_eq!(DOTNET_TICKS.tai_reading, Duration::from_days(-719_162));
        // 9999-12-31 is 2 932 896 days after 1970-01-01.
        let max_seconds = 2_932_896 * 86_400 + 86_399 - DOTNET_TICKS.tai_reading.whole_seconds();
        assert_eq!(
            max_seconds * 10_000_000 + 9_999_999,
            3_155_378_975_999_999_999
        );
        assert_eq!(CCSDS_CUC.tai_reading, Duration::from_days(-4_383));
    }

    /// The software epochs after 1972, reached from their UTC labels
    /// through the leap-second table.
    #[test]
    fn the_later_software_epochs_are_their_utc_labels_read_through_the_leap_table() {
        use crate::unix::{LeapPolicy, UnixTime, tai_from_unix};
        let tai = |unix: i64| {
            tai_from_unix(UnixTime::from_seconds(unix), LeapPolicy::Strict)
                .expect("inside the table")
        };
        for (epoch, unix) in [
            (AMIGAOS, 252_460_800),
            (GARMIN_FIT, 631_065_600),
            (POSTGRESQL, 946_684_800),
            (CORE_FOUNDATION, 978_307_200),
        ] {
            assert_eq!(epoch.instant(), tai(unix), "{}", epoch.id);
        }
        // Garmin's own constant, in milliseconds after the POSIX epoch.
        assert_eq!(631_065_600 * 1_000, 631_065_600_000i64);
    }

    /// The proleptic software epochs are whole days before 1970: SPSS's
    /// the day before the UUID's, the Macintosh's and LabVIEW's 24 107
    /// days, MUMPS's 47 117, which YottaDB's example states again — the
    /// 20 March 2002 of `$HOROLOG` 58883 is 11 766 days after 1970.
    #[test]
    fn the_earlier_software_epochs_are_whole_days_before_1970() {
        assert_eq!(
            SPSS.tai_reading,
            Duration::from_days(-141_428),
            "1582-10-14 is the day before 1582-10-15"
        );
        assert_eq!(
            SPSS.tai_reading.whole_seconds(),
            UUID_GREGORIAN.tai_reading.whole_seconds() - 86_400
        );
        assert_eq!(CLASSIC_MAC_OS.tai_reading, Duration::from_days(-24_107));
        assert_eq!(LABVIEW.tai_reading, CLASSIC_MAC_OS.tai_reading);
        assert_eq!(MUMPS_HOROLOG.tai_reading, Duration::from_days(-47_117));
        assert_eq!(47_117 + 11_766, 58_883);
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
