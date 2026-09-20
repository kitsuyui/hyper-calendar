//! Reading TZif files from the operating system's zone database.
//!
//! This is the only part of the crate that touches the filesystem, and it is
//! deliberately thin: it hands back bytes for [`crate::tzif`] to parse. The
//! separation is what makes the parser testable — a fixture in a test file
//! proves the reader works, on a machine with no `/usr/share/zoneinfo` and in
//! a `no_std` build where this module does not exist at all.

use std::path::{Path, PathBuf};

use crate::error::{TzError, TzResult};

/// Where the IANA database lives on a Unix system.
pub const DEFAULT_ZONEINFO_DIR: &str = "/usr/share/zoneinfo";

/// The directory to read zones from: `TZDIR` if it is set, else
/// [`DEFAULT_ZONEINFO_DIR`].
#[must_use]
pub fn zoneinfo_dir() -> PathBuf {
    std::env::var_os("TZDIR").map_or_else(|| PathBuf::from(DEFAULT_ZONEINFO_DIR), PathBuf::from)
}

/// Whether a string is safe and plausible as a zone identifier.
///
/// Zone names become path components, so this refuses anything that could
/// walk out of the database directory: absolute paths, `..`, backslashes and
/// anything outside the character set IANA actually uses. A name is data from
/// outside the program — an environment variable, a configuration file, a web
/// request — and joining it onto a path unchecked is how a zone lookup turns
/// into an arbitrary file read.
#[must_use]
pub fn is_valid_zone_name(name: &str) -> bool {
    if name.is_empty() || name.len() > 256 || name.starts_with('/') {
        return false;
    }
    name.split('/').all(|component| {
        !component.is_empty()
            && component != ".."
            && component != "."
            && component.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'+' | b'.')
            })
    })
}

/// Read a zone's TZif bytes from the system database.
///
/// # Errors
///
/// Returns [`TzError::UnknownZone`] when the name is not a valid identifier,
/// and [`TzError::Io`] when the file cannot be read.
pub fn read_zone(name: &str) -> TzResult<Vec<u8>> {
    read_zone_from(zoneinfo_dir(), name)
}

/// Read a zone's TZif bytes from a named directory.
///
/// # Errors
///
/// See [`read_zone`].
pub fn read_zone_from(directory: impl AsRef<Path>, name: &str) -> TzResult<Vec<u8>> {
    if !is_valid_zone_name(name) {
        return Err(TzError::UnknownZone);
    }
    let path = directory.as_ref().join(name);
    std::fs::read(path).map_err(TzError::from)
}

/// Whether the system database appears to hold a zone by that name.
#[must_use]
pub fn zone_available(name: &str) -> bool {
    is_valid_zone_name(name) && zoneinfo_dir().join(name).is_file()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::posix::PosixTz;
    use crate::tzif::{TzifData, TzifTimeZone, TzifVersion};
    use crate::zone::{Disambiguation, TimeZone};
    use hc_calendar::{CivilDateTime, CivilTime, Rd};
    use hc_core::UnixTime;

    fn civil(year: i64, month: u8, day: u8, hour: u8, minute: u8) -> CivilDateTime {
        CivilDateTime::new(
            Rd(crate::gregorian::rd_from_ymd(year, month, day)),
            CivilTime::hms(hour, minute, 0).unwrap(),
        )
    }

    #[test]
    fn zone_names_that_could_escape_the_database_are_refused() {
        assert!(is_valid_zone_name("Asia/Tokyo"));
        assert!(is_valid_zone_name("America/Argentina/Ushuaia"));
        assert!(is_valid_zone_name("Etc/GMT+5"));
        assert!(!is_valid_zone_name(""));
        assert!(!is_valid_zone_name("/etc/passwd"));
        assert!(!is_valid_zone_name("../../etc/passwd"));
        assert!(!is_valid_zone_name("Asia/../../etc/passwd"));
        assert!(!is_valid_zone_name("Asia//Tokyo"));
        assert!(!is_valid_zone_name("Asia\\Tokyo"));
        assert!(!is_valid_zone_name("Asia/Tokyo\0"));
        assert_eq!(read_zone("../../etc/passwd"), Err(TzError::UnknownZone));
    }

    #[test]
    fn a_real_tzif_file_parses_when_the_system_database_is_present() {
        // Skips cleanly where there is no database — a container, Windows, a
        // build sandbox — because the fixtures in `tzif` already cover the
        // parser itself.
        let Ok(bytes) = read_zone("Asia/Tokyo") else {
            return;
        };
        let data = TzifData::parse(&bytes).unwrap();
        assert!(data.version() >= TzifVersion::V2);
        assert!(data.local_time_type_count() >= 1);
        let zone = TzifTimeZone::parse("Asia/Tokyo", &bytes).unwrap();
        // 2024-01-01T00:00:00Z is 09:00 in Tokyo, and Japan has not observed
        // daylight saving since 1951.
        let when = UnixTime::from_seconds(1_704_067_200);
        assert_eq!(zone.offset_at(when).seconds(), 9 * 3_600);
        assert_eq!(zone.abbreviation_at(when), Some("JST"));
        assert!(!zone.is_dst_at(when));
        assert_eq!(zone.local_at(when).unwrap(), civil(2024, 1, 1, 9, 0));
    }

    #[test]
    fn the_system_database_remembers_the_2007_rule_change() {
        let Ok(bytes) = read_zone("America/New_York") else {
            return;
        };
        let zone = TzifTimeZone::parse("America/New_York", &bytes).unwrap();
        // 2006 ran on the old rules: 15 March was still standard time and
        // 30 October was too. 2007 ran on the new ones: both dates are on
        // daylight saving time.
        assert!(!zone.is_dst_at(UnixTime::from_seconds(1_142_424_000))); // 2006-03-15
        assert!(!zone.is_dst_at(UnixTime::from_seconds(1_162_209_600))); // 2006-10-30
        assert!(zone.is_dst_at(UnixTime::from_seconds(1_173_960_000))); // 2007-03-15
        assert!(zone.is_dst_at(UnixTime::from_seconds(1_193_745_600))); // 2007-10-30
        // And the transition instants themselves, from the published record:
        // 2006-04-02T07:00:00Z and 2007-03-11T07:00:00Z.
        assert!(!zone.is_dst_at(UnixTime::from_seconds(1_143_961_199)));
        assert!(zone.is_dst_at(UnixTime::from_seconds(1_143_961_200)));
        assert!(!zone.is_dst_at(UnixTime::from_seconds(1_173_596_399)));
        assert!(zone.is_dst_at(UnixTime::from_seconds(1_173_596_400)));
    }

    #[test]
    fn the_system_database_agrees_with_the_builtin_table_on_current_rules() {
        // Where a real file exists, its footer is the string this crate
        // compiled in. A mismatch means the built-in table has gone stale.
        for entry in crate::builtin::ZONES {
            let Ok(bytes) = read_zone(entry.id) else {
                continue;
            };
            let data = TzifData::parse(&bytes).unwrap();
            let Some(footer) = data.footer() else {
                continue;
            };
            assert_eq!(
                PosixTz::parse(footer).unwrap(),
                PosixTz::parse(entry.posix).unwrap(),
                "{} footer {footer}",
                entry.id
            );
        }
    }

    #[test]
    fn a_real_ambiguous_hour_resolves_the_same_way_as_the_rules_predict() {
        let Ok(bytes) = read_zone("America/New_York") else {
            return;
        };
        let zone = TzifTimeZone::parse("America/New_York", &bytes).unwrap();
        let repeated = zone.resolve_local(civil(2024, 11, 3, 1, 30));
        assert!(repeated.is_ambiguous());
        assert_eq!(repeated.earliest().seconds(), 1_730_611_800);
        assert_eq!(repeated.latest().seconds(), 1_730_615_400);

        let skipped = zone.resolve_local(civil(2024, 3, 10, 2, 30));
        assert!(skipped.is_nonexistent());
        assert_eq!(
            skipped.resolve(Disambiguation::Earliest).unwrap().seconds(),
            1_710_052_200
        );
    }

    #[test]
    fn a_missing_zone_reports_an_io_error_rather_than_panicking() {
        let result = read_zone("Mars/Olympus_Mons");
        assert!(matches!(result, Err(TzError::Io(_))) || result.is_ok());
        assert!(!zone_available("Mars/Olympus_Mons"));
    }
}
