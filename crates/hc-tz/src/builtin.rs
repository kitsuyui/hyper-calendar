//! A small compiled-in table of well-known zones, as POSIX `TZ` strings.
//!
//! # What this table is for
//!
//! It answers "what time is it in Tokyo" on a device with no zone database:
//! an embedded target, a WebAssembly module, a static binary. Seventeen
//! zones and a few hundred bytes of text cover most of the world's
//! population.
//!
//! # What it is not for
//!
//! **A POSIX `TZ` string states one pair of rules and applies them to every
//! year.** It cannot say that the United States moved its transitions in
//! 2007, that Brazil stopped changing its clocks in 2019, that Russia
//! abolished daylight saving in 2011, or that Egypt reinstated it in 2023.
//! Every entry here describes the rules *as they stand today*, and is wrong
//! about the past — sometimes about the recent past. For history, read the
//! IANA database with [`crate::tzif`], which records every transition a zone
//! has actually made.
//!
//! The strings are the POSIX footers of the corresponding files in the IANA
//! time zone database, release 2026c, which is where the tzdata maintainers
//! put their own answer to "what are this zone's current rules". The test
//! `the_system_database_agrees_with_the_builtin_table_on_current_rules` in
//! [`crate::system`] re-checks them against the local database whenever one
//! is present, so a stale entry shows up as a failing test rather than as a
//! wrong answer.

use crate::error::{TzError, TzResult};
use crate::posix::{PosixTimeZone, PosixTz};

/// One entry in the built-in table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuiltinZone {
    /// The IANA identifier, such as `Asia/Tokyo`.
    pub id: &'static str,
    /// The zone's current rules as a POSIX `TZ` string.
    pub posix: &'static str,
}

/// The built-in zones.
///
/// The selection is deliberately lopsided towards awkward offsets: India and
/// Nepal for the half and quarter hours, Lord Howe Island for a half-hour
/// daylight shift, Sydney and Auckland for southern-hemisphere rules that
/// straddle the new year, Cairo for a rule anchored to the last Thursday of a
/// month at 24:00. Those are the entries that catch bugs.
pub const ZONES: &[BuiltinZone] = &[
    BuiltinZone {
        id: "UTC",
        posix: "UTC0",
    },
    BuiltinZone {
        id: "Africa/Cairo",
        posix: "EET-2EEST,M4.5.5/0,M10.5.4/24",
    },
    BuiltinZone {
        id: "America/Los_Angeles",
        posix: "PST8PDT,M3.2.0,M11.1.0",
    },
    BuiltinZone {
        id: "America/New_York",
        posix: "EST5EDT,M3.2.0,M11.1.0",
    },
    BuiltinZone {
        id: "America/Sao_Paulo",
        posix: "<-03>3",
    },
    BuiltinZone {
        id: "Asia/Kathmandu",
        posix: "<+0545>-5:45",
    },
    BuiltinZone {
        id: "Asia/Kolkata",
        posix: "IST-5:30",
    },
    BuiltinZone {
        id: "Asia/Seoul",
        posix: "KST-9",
    },
    BuiltinZone {
        id: "Asia/Shanghai",
        posix: "CST-8",
    },
    BuiltinZone {
        id: "Asia/Tokyo",
        posix: "JST-9",
    },
    BuiltinZone {
        id: "Australia/Lord_Howe",
        posix: "<+1030>-10:30<+11>-11,M10.1.0,M4.1.0",
    },
    BuiltinZone {
        id: "Australia/Sydney",
        posix: "AEST-10AEDT,M10.1.0,M4.1.0/3",
    },
    BuiltinZone {
        id: "Europe/Berlin",
        posix: "CET-1CEST,M3.5.0,M10.5.0/3",
    },
    BuiltinZone {
        id: "Europe/London",
        posix: "GMT0BST,M3.5.0/1,M10.5.0",
    },
    BuiltinZone {
        id: "Europe/Moscow",
        posix: "MSK-3",
    },
    BuiltinZone {
        id: "Europe/Paris",
        posix: "CET-1CEST,M3.5.0,M10.5.0/3",
    },
    BuiltinZone {
        id: "Pacific/Auckland",
        posix: "NZST-12NZDT,M9.5.0,M4.1.0/3",
    },
];

/// Look up a zone by IANA identifier, ignoring ASCII case.
///
/// Case folding matches what `TZ=asia/tokyo` does on most systems, and costs
/// nothing at this table size.
#[must_use]
pub fn find(id: &str) -> Option<&'static BuiltinZone> {
    ZONES.iter().find(|zone| zone.id.eq_ignore_ascii_case(id))
}

/// The POSIX `TZ` string for a zone.
#[must_use]
pub fn posix_string(id: &str) -> Option<&'static str> {
    find(id).map(|zone| zone.posix)
}

/// Every built-in identifier.
pub fn ids() -> impl Iterator<Item = &'static str> {
    ZONES.iter().map(|zone| zone.id)
}

/// Build a usable zone from the table.
///
/// # Errors
///
/// Returns [`TzError::UnknownZone`] when the identifier is not in the table.
/// A parse failure would mean the table itself is wrong, which the tests
/// rule out.
pub fn zone(id: &str) -> TzResult<PosixTimeZone<'static>> {
    let entry = find(id).ok_or(TzError::UnknownZone)?;
    Ok(PosixTimeZone::new(entry.id, PosixTz::parse(entry.posix)?))
}

#[cfg(test)]
mod tests {
    /// The README states this count twice, and a documented count that
    /// drifts is a documented lie. The count is easy to get wrong by eye, so
    /// it is checked here rather than trusted.
    #[test]
    fn the_builtin_zone_count_is_the_one_the_readme_states() {
        assert_eq!(ZONES.len(), 17);
    }

    use super::*;
    use crate::gregorian::rd_from_ymd;
    use crate::zone::TimeZone;
    use hc_calendar::fixed::RD_OF_UNIX_EPOCH;
    use hc_core::UnixTime;

    fn instant(year: i64, month: u8, day: u8, hour: u8) -> UnixTime {
        let days = rd_from_ymd(year, month, day) - RD_OF_UNIX_EPOCH;
        UnixTime::from_seconds(days * 86_400 + i64::from(hour) * 3_600)
    }

    #[test]
    fn every_builtin_string_parses_and_renders_back_to_itself() {
        for entry in ZONES {
            let rules =
                PosixTz::parse(entry.posix).unwrap_or_else(|error| panic!("{}: {error}", entry.id));
            assert_eq!(rules.to_string(), entry.posix, "{}", entry.id);
        }
    }

    #[test]
    fn the_table_is_sorted_and_free_of_duplicates() {
        for pair in ZONES.windows(2) {
            assert!(
                pair[0].id == "UTC" || pair[0].id < pair[1].id,
                "{} then {}",
                pair[0].id,
                pair[1].id
            );
        }
    }

    #[test]
    fn lookup_ignores_ascii_case_and_reports_unknown_zones() {
        assert_eq!(posix_string("Asia/Tokyo"), Some("JST-9"));
        assert_eq!(posix_string("asia/tokyo"), Some("JST-9"));
        assert_eq!(posix_string("Mars/Olympus_Mons"), None);
        assert_eq!(zone("Mars/Olympus_Mons").unwrap_err(), TzError::UnknownZone);
        assert_eq!(ids().count(), ZONES.len());
    }

    #[test]
    fn the_table_covers_the_offsets_that_catch_bugs() {
        let midsummer = instant(2024, 7, 1, 0);
        let midwinter = instant(2024, 1, 1, 0);
        // Quarter and half hours.
        assert_eq!(
            zone("Asia/Kathmandu")
                .unwrap()
                .offset_at(midsummer)
                .seconds(),
            5 * 3_600 + 45 * 60
        );
        assert_eq!(
            zone("Asia/Kolkata").unwrap().offset_at(midsummer).seconds(),
            5 * 3_600 + 30 * 60
        );
        // A half-hour daylight shift.
        let lord_howe = zone("Australia/Lord_Howe").unwrap();
        assert_eq!(
            lord_howe.offset_at(midwinter).seconds() - lord_howe.offset_at(midsummer).seconds(),
            1_800
        );
        // Southern hemisphere: daylight saving in January, not July.
        let sydney = zone("Australia/Sydney").unwrap();
        assert!(sydney.is_dst_at(midwinter));
        assert!(!sydney.is_dst_at(midsummer));
        // Northern hemisphere, the other way round.
        let new_york = zone("America/New_York").unwrap();
        assert!(!new_york.is_dst_at(midwinter));
        assert!(new_york.is_dst_at(midsummer));
    }

    #[test]
    fn zones_without_daylight_saving_stay_put_all_year() {
        for id in [
            "UTC",
            "Asia/Tokyo",
            "Asia/Seoul",
            "Asia/Shanghai",
            "Asia/Kolkata",
            "Asia/Kathmandu",
            "Europe/Moscow",
            "America/Sao_Paulo",
        ] {
            let zone = zone(id).unwrap();
            let reference = zone.offset_at(instant(2024, 1, 1, 0));
            for month in 1..=12u8 {
                let when = instant(2024, month, 15, 12);
                assert_eq!(zone.offset_at(when), reference, "{id} month {month}");
                assert!(!zone.is_dst_at(when), "{id} month {month}");
            }
        }
    }

    #[test]
    fn the_named_zones_report_their_expected_offsets_in_january() {
        let january = instant(2024, 1, 15, 0);
        for (id, expected_hours) in [
            ("UTC", 0.0_f64),
            ("Asia/Tokyo", 9.0),
            ("Asia/Seoul", 9.0),
            ("Asia/Shanghai", 8.0),
            ("Asia/Kolkata", 5.5),
            ("Asia/Kathmandu", 5.75),
            ("Europe/Moscow", 3.0),
            ("Europe/Paris", 1.0),
            ("Europe/London", 0.0),
            ("America/New_York", -5.0),
            ("America/Los_Angeles", -8.0),
            ("America/Sao_Paulo", -3.0),
            ("Africa/Cairo", 2.0),
            ("Australia/Sydney", 11.0),
            ("Australia/Lord_Howe", 11.0),
            ("Pacific/Auckland", 13.0),
        ] {
            let seconds = zone(id).unwrap().offset_at(january).seconds();
            assert!(
                (f64::from(seconds) - expected_hours * 3_600.0).abs() < 0.5,
                "{id}: {seconds}"
            );
        }
    }

    #[test]
    fn abbreviations_come_through_the_table() {
        assert_eq!(
            zone("Asia/Tokyo")
                .unwrap()
                .abbreviation_at(instant(2024, 7, 1, 0)),
            Some("JST")
        );
        assert_eq!(
            zone("America/New_York")
                .unwrap()
                .abbreviation_at(instant(2024, 7, 1, 0)),
            Some("EDT")
        );
        assert_eq!(
            zone("America/Sao_Paulo")
                .unwrap()
                .abbreviation_at(instant(2024, 7, 1, 0)),
            Some("-03")
        );
        assert_eq!(zone("Asia/Tokyo").unwrap().name(), "Asia/Tokyo");
    }
}
