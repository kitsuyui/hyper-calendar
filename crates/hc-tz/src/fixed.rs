//! Zones whose offset never changes.

use hc_calendar::CivilDateTime;
use hc_core::UnixTime;

use crate::offset::UtcOffset;
use crate::zone::{LocalResolution, TimeZone, unix_from_local_saturating};

/// A zone with one offset, for all time.
///
/// Useful in its own right — `Etc/GMT+5`, a log file that records `+09:00`
/// and nothing more — and useful as the degenerate case that proves the
/// [`TimeZone`] interface is not secretly assuming transitions exist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedTimeZone<'a> {
    name: &'a str,
    abbreviation: Option<&'a str>,
    offset: UtcOffset,
}

impl<'a> FixedTimeZone<'a> {
    /// A named fixed-offset zone with no abbreviation.
    #[must_use]
    pub const fn new(name: &'a str, offset: UtcOffset) -> Self {
        Self {
            name,
            abbreviation: None,
            offset,
        }
    }

    /// A named fixed-offset zone that also publishes an abbreviation.
    #[must_use]
    pub const fn with_abbreviation(
        name: &'a str,
        abbreviation: &'a str,
        offset: UtcOffset,
    ) -> Self {
        Self {
            name,
            abbreviation: Some(abbreviation),
            offset,
        }
    }

    /// The offset, which is the same at every instant.
    #[must_use]
    pub const fn offset(&self) -> UtcOffset {
        self.offset
    }
}

impl TimeZone for FixedTimeZone<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn offset_at(&self, _utc: UnixTime) -> UtcOffset {
        self.offset
    }

    fn abbreviation_at(&self, _utc: UnixTime) -> Option<&str> {
        self.abbreviation
    }

    fn is_dst_at(&self, _utc: UnixTime) -> bool {
        false
    }

    fn resolve_local(&self, local: CivilDateTime) -> LocalResolution {
        LocalResolution::Unambiguous(unix_from_local_saturating(local, self.offset))
    }
}

/// Coordinated Universal Time itself.
///
/// A separate type rather than a `FixedTimeZone` constant so that "this value
/// is UTC" can be stated in a signature and checked at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Utc;

impl Utc {
    /// UTC as a fixed-offset zone.
    #[must_use]
    pub const fn as_fixed() -> FixedTimeZone<'static> {
        FixedTimeZone::with_abbreviation("UTC", "UTC", UtcOffset::UTC)
    }
}

impl TimeZone for Utc {
    fn name(&self) -> &str {
        "UTC"
    }

    fn offset_at(&self, _utc: UnixTime) -> UtcOffset {
        UtcOffset::UTC
    }

    fn abbreviation_at(&self, _utc: UnixTime) -> Option<&str> {
        Some("UTC")
    }

    fn is_dst_at(&self, _utc: UnixTime) -> bool {
        false
    }

    fn resolve_local(&self, local: CivilDateTime) -> LocalResolution {
        LocalResolution::Unambiguous(unix_from_local_saturating(local, UtcOffset::UTC))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian::rd_from_ymd;
    use crate::zone::Disambiguation;
    use hc_calendar::{CivilTime, Rd};

    fn civil(year: i64, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> CivilDateTime {
        CivilDateTime::new(
            Rd(rd_from_ymd(year, month, day)),
            CivilTime::hms(hour, minute, second).unwrap(),
        )
    }

    #[test]
    fn utc_names_itself_and_never_moves() {
        let zone = Utc;
        assert_eq!(zone.name(), "UTC");
        assert_eq!(zone.abbreviation_at(UnixTime::EPOCH), Some("UTC"));
        for seconds in (-2_000_000_000..2_000_000_000).step_by(97_000_003) {
            let instant = UnixTime::from_seconds(seconds);
            assert_eq!(zone.offset_at(instant), UtcOffset::UTC);
            assert!(!zone.is_dst_at(instant));
        }
    }

    #[test]
    fn the_unix_epoch_is_midnight_on_the_first_of_january_1970_in_utc() {
        let zone = Utc;
        assert_eq!(
            zone.local_at(UnixTime::EPOCH).unwrap(),
            civil(1970, 1, 1, 0, 0, 0)
        );
        assert_eq!(
            zone.unix_at(civil(1970, 1, 1, 0, 0, 0), Disambiguation::Reject)
                .unwrap(),
            UnixTime::EPOCH
        );
    }

    #[test]
    fn a_fixed_zone_is_never_ambiguous_and_never_on_daylight_saving_time() {
        let zone = FixedTimeZone::with_abbreviation(
            "Asia/Kathmandu",
            "+0545",
            UtcOffset::from_hms(5, 45, 0).unwrap(),
        );
        for day in 719_000..720_000 {
            let local = CivilDateTime::new(Rd(day), CivilTime::hms(2, 30, 0).unwrap());
            assert!(zone.resolve_local(local).is_unambiguous(), "{day}");
        }
        assert!(!zone.is_dst_at(UnixTime::EPOCH));
        assert_eq!(zone.abbreviation_at(UnixTime::EPOCH), Some("+0545"));
        assert_eq!(zone.offset().seconds(), 20_700);
    }

    #[test]
    fn a_forty_five_minute_offset_reaches_the_right_wall_clock_reading() {
        // 2024-01-01T00:00:00Z is 05:45 in Kathmandu.
        let zone = FixedTimeZone::new("Asia/Kathmandu", UtcOffset::from_hms(5, 45, 0).unwrap());
        let instant = UnixTime::from_seconds(1_704_067_200);
        assert_eq!(zone.local_at(instant).unwrap(), civil(2024, 1, 1, 5, 45, 0));
        assert_eq!(
            zone.unix_at(civil(2024, 1, 1, 5, 45, 0), Disambiguation::Reject)
                .unwrap(),
            instant
        );
        assert_eq!(zone.abbreviation_at(instant), None);
    }

    #[test]
    fn utc_can_also_be_had_as_an_ordinary_fixed_zone() {
        let zone = Utc::as_fixed();
        assert_eq!(zone.name(), "UTC");
        assert_eq!(zone.offset(), UtcOffset::UTC);
        assert_eq!(zone.abbreviation_at(UnixTime::EPOCH), Some("UTC"));
        assert_eq!(
            zone.local_at(UnixTime::EPOCH).unwrap(),
            Utc.local_at(UnixTime::EPOCH).unwrap()
        );
    }

    #[test]
    fn fixed_zones_round_trip_every_hour_of_a_decade() {
        let zone = FixedTimeZone::new("Etc/GMT-12", UtcOffset::from_hms(12, 0, 0).unwrap());
        for seconds in (1_000_000_000..1_315_000_000).step_by(3_600) {
            let instant = UnixTime::from_seconds(seconds);
            let local = zone.local_at(instant).unwrap();
            assert_eq!(
                zone.unix_at(local, Disambiguation::Reject).unwrap(),
                instant,
                "{seconds}"
            );
        }
    }
}
