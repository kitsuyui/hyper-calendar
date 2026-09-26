//! GNSS week numbers, their rollovers, and GLONASS time.
//!
//! The satellite navigation systems broadcast time as a week number and
//! the seconds of the week, counted from a week-zero epoch, and the week
//! field is short: ten bits in the GPS legacy message and NavIC, twelve in
//! Galileo, thirteen in GPS CNAV and BeiDou. A broadcast week therefore
//! names a family of weeks a power of two apart, and only a date the
//! receiver already knows can pick one. [`WeekNumbering`] carries each
//! field as an epoch and a bit width; the two resolution rules take that
//! date from the caller explicitly rather than assuming one.
//!
//! GLONASS keeps UTC(SU) + 3 h, leap seconds included, so it is not a
//! [`crate::TimeScale`]; [`GlonassTime`] is a [`UtcInstant`] read three hours
//! ahead. The uniform GNSS scales themselves are in [`crate::scale`].
//!
//! The systems, their epochs, the worked example of the April 2019
//! rollover and the sources are in `docs/systems/gnss-time.md`.

use crate::duration::Duration;
use crate::epoch::{self, Epoch};
use crate::error::{TimeError, TimeResult};
use crate::scale::{Instant, Tai};
use crate::unix::{self, LeapPolicy, UtcInstant};

/// The length of a GNSS week, 604 800 SI seconds.
pub const WEEK: Duration = Duration::from_secs(604_800);

/// A full week number and the time into that week.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WeekTime {
    /// Weeks since week zero, not truncated to any broadcast field.
    pub week: u32,
    /// The time since the start of the week, below [`WEEK`].
    pub time_of_week: Duration,
}

/// A broadcast week-number field: the week-zero epoch it counts from and
/// how many bits it has.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeekNumbering {
    id: &'static str,
    english_name: &'static str,
    week_zero: Epoch,
    bits: u8,
    source: &'static str,
}

impl WeekNumbering {
    /// This numbering's identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        self.id
    }

    /// This numbering's English name.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        self.english_name
    }

    /// The start of week zero.
    #[must_use]
    pub const fn week_zero(self) -> Epoch {
        self.week_zero
    }

    /// The width of the broadcast week field.
    #[must_use]
    pub const fn bits(self) -> u8 {
        self.bits
    }

    /// The number of weeks after which the broadcast field repeats.
    #[must_use]
    pub const fn modulus(self) -> u32 {
        1 << self.bits
    }

    /// The document that defines the field.
    #[must_use]
    pub const fn source(self) -> &'static str {
        self.source
    }

    /// The full week and the time of week of a TAI instant.
    ///
    /// # Errors
    ///
    /// [`TimeError::BeforeModelStart`] before week zero, and
    /// [`TimeError::Overflow`] past week `u32::MAX`.
    pub fn week_time(self, tai: Instant<Tai>) -> TimeResult<WeekTime> {
        let since = tai.since_epoch().checked_sub(self.week_zero.tai_reading)?;
        if since.is_negative() {
            return Err(TimeError::BeforeModelStart);
        }
        let (week, time_of_week) = since.checked_div_rem(WEEK)?;
        let week = u32::try_from(week).map_err(|_| TimeError::Overflow)?;
        Ok(WeekTime { week, time_of_week })
    }

    /// The TAI instant of a full week and time of week.
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] when the time of week is negative or not
    /// below [`WEEK`].
    pub fn to_tai(self, time: WeekTime) -> TimeResult<Instant<Tai>> {
        if time.time_of_week.is_negative() || time.time_of_week >= WEEK {
            return Err(TimeError::OutOfRange);
        }
        let start = Duration::from_weeks(i64::from(time.week));
        let reading = self
            .week_zero
            .tai_reading
            .checked_add(start)?
            .checked_add(time.time_of_week)?;
        Ok(Instant::from_epoch(reading))
    }

    /// The week number as this field broadcasts it: the full week modulo
    /// [`Self::modulus`].
    #[must_use]
    pub const fn broadcast(self, week: u32) -> u32 {
        week % self.modulus()
    }

    /// The first full week at or after the reference instant's week whose
    /// broadcast number is `broadcast`.
    ///
    /// The rule a receiver applies with a date it knows it is not before,
    /// such as its firmware's build date. A reference before week zero
    /// counts as week zero.
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] when `broadcast` does not fit the field,
    /// and [`TimeError::Overflow`] past week `u32::MAX`.
    pub fn resolve_not_before(self, broadcast: u32, reference: Instant<Tai>) -> TimeResult<u32> {
        let modulus = u64::from(self.modulus());
        let reference = u64::from(self.reference_week(broadcast, reference)?);
        let mut week = reference - reference % modulus + u64::from(broadcast);
        if week < reference {
            week += modulus;
        }
        u32::try_from(week).map_err(|_| TimeError::Overflow)
    }

    /// The full week within half a period of the reference instant's week
    /// whose broadcast number is `broadcast`: the one in the half-open
    /// window `[reference − m/2, reference + m/2)` for modulus `m`.
    ///
    /// The rule for a reference that may be early or late, such as a
    /// file's timestamp. Where the window reaches before week zero, the
    /// weeks that do not exist are skipped and the answer is the week one
    /// period later.
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] when `broadcast` does not fit the field,
    /// and [`TimeError::Overflow`] past week `u32::MAX`.
    pub fn resolve_nearest(self, broadcast: u32, reference: Instant<Tai>) -> TimeResult<u32> {
        let modulus = i64::from(self.modulus());
        let reference = i64::from(self.reference_week(broadcast, reference)?);
        let lower = reference - modulus / 2;
        let mut week = lower + (i64::from(broadcast) - lower).rem_euclid(modulus);
        if week < 0 {
            week += modulus;
        }
        u32::try_from(week).map_err(|_| TimeError::Overflow)
    }

    fn reference_week(self, broadcast: u32, reference: Instant<Tai>) -> TimeResult<u32> {
        if broadcast >= self.modulus() {
            return Err(TimeError::OutOfRange);
        }
        match self.week_time(reference) {
            Ok(time) => Ok(time.week),
            Err(TimeError::BeforeModelStart) => Ok(0),
            Err(error) => Err(error),
        }
    }
}

/// The GPS legacy navigation message's week, ten bits, modulo 1024.
pub const GPS_LNAV: WeekNumbering = WeekNumbering {
    id: "gps-lnav-week",
    english_name: "GPS week, legacy navigation message",
    week_zero: epoch::GPS,
    bits: 10,
    source: "IS-GPS-200G (2012), 20.3.3.3.1.1: the ten-bit transmitted week number, \
        modulo 1024 [is-gps-200g]",
};

/// The GPS CNAV message's week, thirteen bits, modulo 8192.
pub const GPS_CNAV: WeekNumbering = WeekNumbering {
    id: "gps-cnav-week",
    english_name: "GPS week, CNAV message",
    week_zero: epoch::GPS,
    bits: 13,
    source: "IS-GPS-200G (2012), 30.3.3.1.1.1: the thirteen-bit week number, modulo 8192 \
        [is-gps-200g]",
};

/// The Galileo week, twelve bits, from the GST epoch.
pub const GALILEO: WeekNumbering = WeekNumbering {
    id: "galileo-week",
    english_name: "Galileo System Time week",
    week_zero: epoch::GALILEO,
    bits: 12,
    source: "Galileo OS SIS ICD, Issue 2.1 (2023), 5.1.2 and Table 67: the twelve-bit \
        week number [galileo-os-sis-icd-2-1]",
};

/// The BeiDou week, thirteen bits, from the BDT epoch.
pub const BEIDOU: WeekNumbering = WeekNumbering {
    id: "beidou-week",
    english_name: "BeiDou Time week",
    week_zero: epoch::BEIDOU,
    bits: 13,
    source: "BDS-SIS-ICD-B1I-1.0 (2012), 3.3: the thirteen-bit week number \
        [bds-sis-icd-b1i-1-0]",
};

/// The NavIC week, ten bits, from the NavIC epoch.
pub const NAVIC: WeekNumbering = WeekNumbering {
    id: "navic-week",
    english_name: "NavIC system time week",
    week_zero: epoch::NAVIC,
    bits: 10,
    source: "ISRO, IRNSS SIS ICD for SPS, version 1.1 (2017): the ten-bit week number \
        [irnss-sps-icd-1-1]",
};

/// Every broadcast week-number field.
pub const ALL: &[WeekNumbering] = &[GPS_LNAV, GPS_CNAV, GALILEO, BEIDOU, NAVIC];

/// The week-number field with this identifier.
#[must_use]
pub fn by_id(id: &str) -> Option<WeekNumbering> {
    ALL.iter().copied().find(|numbering| numbering.id == id)
}

crate::catalogue_tests! {
    type: WeekNumbering,
    id: |numbering| numbering.id,
    provenance: |numbering| numbering.source,
    tests: week_numbering_catalogue,
    all: ALL,
    lookup: by_id,
}

/// `GLONASS − UTC`, three hours: GLONASS time is UTC(SU) + 3 h (GLONASS
/// ICD, Edition 5.1, 2008, §3.3.3, `glonass-icd-5-1`).
pub const GLONASS_MINUS_UTC: Duration = Duration::from_hours(3);

const GLONASS_MINUS_UTC_SECS: i64 = 3 * 3_600;

/// The GLONASS label of 1996-01-01 00:00, the start of *N*4 = 1, on the
/// POSIX-style count [`GlonassTime`] reads.
const GLONASS_1996_LABEL: i64 = 820_454_400;

/// The days from 1996-01-01 to 2100-01-01: twenty-six intervals of 1 461.
const GLONASS_INTERVAL_DAYS_END: i64 = 26 * 1_461;

/// A GLONASS date as the navigation message carries it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlonassDate {
    /// *N*4, the four-year interval, 1 for 1996–1999.
    pub four_year_interval: u32,
    /// *N*T, the day within the interval, 1 on 1 January of its leap year.
    pub day: u32,
}

/// GLONASS time: UTC(SU) + 3 h, taking UTC's leap seconds.
///
/// The reading is a [`UtcInstant`] shifted three hours ahead, so its
/// `leap_second` flag names the inserted second, which GLONASS labels
/// 02:59:60 rather than 23:59:60. That is why this is not a
/// [`crate::TimeScale`]: its offset from TAI steps with every leap second.
/// See `docs/systems/gnss-time.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlonassTime(pub UtcInstant);

impl GlonassTime {
    /// GLONASS time at a UTC instant.
    ///
    /// # Errors
    ///
    /// [`TimeError::Overflow`] when the shifted reading is unrepresentable.
    pub fn from_utc(utc: UtcInstant) -> TimeResult<Self> {
        let unix_seconds = utc
            .unix_seconds
            .checked_add(GLONASS_MINUS_UTC_SECS)
            .ok_or(TimeError::Overflow)?;
        Ok(Self(UtcInstant {
            unix_seconds,
            ..utc
        }))
    }

    /// The UTC instant of this reading.
    ///
    /// # Errors
    ///
    /// [`TimeError::Overflow`] when the shifted reading is unrepresentable.
    pub fn to_utc(self) -> TimeResult<UtcInstant> {
        let unix_seconds = self
            .0
            .unix_seconds
            .checked_sub(GLONASS_MINUS_UTC_SECS)
            .ok_or(TimeError::Overflow)?;
        Ok(UtcInstant {
            unix_seconds,
            ..self.0
        })
    }

    /// GLONASS time at a TAI instant, through the leap-second table.
    ///
    /// # Errors
    ///
    /// As [`unix::utc_from_tai`].
    pub fn from_tai(tai: Instant<Tai>, policy: LeapPolicy) -> TimeResult<Self> {
        Self::from_utc(unix::utc_from_tai(tai, policy)?)
    }

    /// The TAI instant of this reading, through the leap-second table.
    ///
    /// # Errors
    ///
    /// As [`unix::tai_from_utc`].
    pub fn to_tai(self, policy: LeapPolicy) -> TimeResult<Instant<Tai>> {
        unix::tai_from_utc(self.to_utc()?, policy)
    }

    /// The four-year interval *N*4 and the day *N*T within it (GLONASS ICD,
    /// Edition 5.1, 2008, §4, `glonass-icd-5-1`).
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] before 1996 and from 2100. The intervals
    /// are 1 461 days each until 2100, a common year, breaks the cycle; the
    /// day numbering there is not counted rather than counted wrong.
    pub fn date(self) -> TimeResult<GlonassDate> {
        // An inserted second, 02:59:60, belongs to the day of the second
        // before it, which is the same GLONASS day as the one after it.
        let label = self.0.unix_seconds - i64::from(self.0.leap_second);
        let day = (label - GLONASS_1996_LABEL).div_euclid(86_400);
        if !(0..GLONASS_INTERVAL_DAYS_END).contains(&day) {
            return Err(TimeError::OutOfRange);
        }
        let interval = u32::try_from(day / 1_461 + 1).map_err(|_| TimeError::Overflow)?;
        let day = u32::try_from(day % 1_461 + 1).map_err(|_| TimeError::Overflow)?;
        Ok(GlonassDate {
            four_year_interval: interval,
            day,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unix::{UnixTime, tai_from_unix};

    const STRICT: LeapPolicy = LeapPolicy::Strict;

    fn utc(unix: i64) -> Instant<Tai> {
        tai_from_unix(UnixTime::from_seconds(unix), STRICT).expect("inside the table")
    }

    /// GST and NavIC week 0 is GPS week 1024, the day GPS's legacy week
    /// first rolled over; BDT week 0 is GPS week 1356, 14 s in, because
    /// BDT was set equal to UTC when GPS − UTC was 14 s.
    #[test]
    fn the_week_zeros_line_up_with_gps_weeks() {
        for numbering in [GALILEO, NAVIC] {
            let start = numbering.week_zero().instant();
            assert_eq!(
                GPS_LNAV.week_time(start),
                Ok(WeekTime {
                    week: 1024,
                    time_of_week: Duration::ZERO
                }),
                "{}",
                numbering.id()
            );
        }
        assert_eq!(
            GPS_CNAV.week_time(BEIDOU.week_zero().instant()),
            Ok(WeekTime {
                week: 1356,
                time_of_week: Duration::from_secs(14)
            })
        );
    }

    /// GPS week 2048 began at 2019-04-06 23:59:42 UTC, when `GPS − UTC` was
    /// 18 s; the legacy field read 0 again and CNAV read 2048.
    #[test]
    fn the_april_2019_rollover() {
        // 2019-04-07 00:00:00 UTC is POSIX 1 554 595 200.
        let rollover = utc(1_554_595_200 - 18);
        let before = rollover.checked_sub(Duration::SECOND).expect("in range");
        assert_eq!(
            GPS_LNAV.week_time(rollover),
            Ok(WeekTime {
                week: 2048,
                time_of_week: Duration::ZERO
            })
        );
        assert_eq!(
            GPS_LNAV.week_time(before),
            Ok(WeekTime {
                week: 2047,
                time_of_week: Duration::from_secs(604_799)
            })
        );
        assert_eq!(GPS_LNAV.broadcast(2048), 0);
        assert_eq!(GPS_LNAV.broadcast(2047), 1023);
        assert_eq!(GPS_CNAV.broadcast(2048), 2048);

        // 2019-01-01 00:00:00 UTC, in GPS week 2034.
        let reference = utc(1_546_300_800);
        assert_eq!(GPS_LNAV.week_time(reference).map(|t| t.week), Ok(2034));
        assert_eq!(GPS_LNAV.resolve_not_before(0, reference), Ok(2048));
        assert_eq!(GPS_LNAV.resolve_nearest(0, reference), Ok(2048));
        assert_eq!(GPS_CNAV.resolve_not_before(2048, reference), Ok(2048));
        // The week before the rollover, against the same reference.
        assert_eq!(GPS_LNAV.resolve_not_before(1023, reference), Ok(2047));
        assert_eq!(GPS_LNAV.resolve_nearest(1023, reference), Ok(2047));
        // Without a reference, or with week zero as one, the field reads 1980.
        let epoch = GPS_LNAV.week_zero().instant();
        assert_eq!(GPS_LNAV.resolve_not_before(0, epoch), Ok(0));
    }

    /// The two rules differ when the reference is past the broadcast week:
    /// "not before" moves a whole period on, "nearest" stays behind.
    #[test]
    fn the_rollover_rules_differ_behind_the_reference() {
        let reference = GPS_LNAV
            .to_tai(WeekTime {
                week: 2100,
                time_of_week: Duration::ZERO,
            })
            .expect("in range");
        // Broadcast 6 is week 2054, 46 behind, or 3078.
        assert_eq!(GPS_LNAV.resolve_nearest(6, reference), Ok(2054));
        assert_eq!(GPS_LNAV.resolve_not_before(6, reference), Ok(3078));
        // The window is half-open: 512 behind is in, 512 ahead is out.
        // Weeks 1588 and 2612 share broadcast 564.
        assert_eq!(GPS_LNAV.broadcast(1588), 564);
        assert_eq!(GPS_LNAV.broadcast(2612), 564);
        assert_eq!(GPS_LNAV.resolve_nearest(564, reference), Ok(1588));
        // A reference near week zero cannot reach before it.
        let early = GPS_LNAV.week_zero().instant();
        assert_eq!(GPS_LNAV.resolve_nearest(1000, early), Ok(1000));
        assert_eq!(
            GPS_LNAV.resolve_nearest(1024, early),
            Err(TimeError::OutOfRange)
        );
    }

    #[test]
    fn week_time_round_trips_and_refuses_bad_fields() {
        for numbering in ALL {
            for offset in [0i128, 1, 604_799, 604_800, 1_000_000_000] {
                let instant = numbering
                    .week_zero()
                    .instant()
                    .checked_add(Duration::from_secs(offset))
                    .expect("in range");
                let time = numbering.week_time(instant).expect("after week zero");
                assert_eq!(numbering.to_tai(time), Ok(instant), "{}", numbering.id());
            }
            let before = numbering
                .week_zero()
                .instant()
                .checked_sub(Duration::from_nanos(1))
                .expect("in range");
            assert_eq!(
                numbering.week_time(before),
                Err(TimeError::BeforeModelStart)
            );
            assert_eq!(
                numbering.to_tai(WeekTime {
                    week: 1,
                    time_of_week: WEEK
                }),
                Err(TimeError::OutOfRange)
            );
            assert_eq!(numbering.modulus(), 1 << numbering.bits());
        }
        assert_eq!(GALILEO.modulus(), 4_096);
        assert_eq!(BEIDOU.modulus(), 8_192);
        assert_eq!(NAVIC.modulus(), 1_024);
    }

    /// GLONASS reads UTC three hours ahead and inserts the leap second when
    /// UTC does, at 02:59:60.
    #[test]
    fn glonass_is_utc_three_hours_ahead_with_its_leap_seconds() {
        // 2024-01-01 00:00:00 UTC reads 03:00:00 GLONASS.
        let midnight = UtcInstant::from_unix(UnixTime::from_seconds(1_704_067_200));
        let glonass = GlonassTime::from_utc(midnight).expect("in range");
        assert_eq!(glonass.0.unix_seconds, 1_704_067_200 + 10_800);
        assert_eq!(glonass.to_utc(), Ok(midnight));

        // 2016-12-31 23:59:60 UTC is 2017-01-01 02:59:60 GLONASS.
        let leap = UtcInstant {
            unix_seconds: 1_483_228_800,
            leap_second: true,
            subsec_attos: 0,
        };
        let tai = unix::tai_from_utc(leap, STRICT).expect("in the table");
        let glonass = GlonassTime::from_tai(tai, STRICT).expect("in the table");
        assert!(glonass.0.leap_second);
        assert_eq!(glonass.0.unix_seconds, 1_483_228_800 + 10_800);
        assert_eq!(glonass.to_tai(STRICT), Ok(tai));

        // Its offset from TAI steps with the leap second: GLONASS minus TAI
        // is 3 h − 36 s before it and 3 h − 37 s after.
        let offset = |unix: i64| {
            let tai = utc(unix);
            let glonass = GlonassTime::from_tai(tai, STRICT).expect("in the table");
            glonass.0.unix_seconds - i64::try_from(tai.since_epoch().whole_seconds()).unwrap()
        };
        assert_eq!(offset(1_483_228_799), 10_800 - 36);
        assert_eq!(offset(1_483_228_800), 10_800 - 37);
        assert_eq!(GLONASS_MINUS_UTC, Duration::from_secs(10_800));
    }

    #[test]
    fn glonass_four_year_intervals() {
        let at = |unix_label: i64| {
            GlonassTime(UtcInstant::from_unix(UnixTime::from_seconds(unix_label))).date()
        };
        // The labels are GLONASS's own clock, 1996-01-01 00:00 = 820 454 400.
        assert_eq!(
            at(820_454_400),
            Ok(GlonassDate {
                four_year_interval: 1,
                day: 1
            })
        );
        // 2024-01-01 00:00 GLONASS, 2023-12-31 21:00 UTC.
        assert_eq!(
            at(1_704_067_200),
            Ok(GlonassDate {
                four_year_interval: 8,
                day: 1
            })
        );
        // 2027-12-31 23:59:59, the interval's last day.
        assert_eq!(
            at(1_830_297_599),
            Ok(GlonassDate {
                four_year_interval: 8,
                day: 1_461
            })
        );
        // The epoch reached from UTC.
        let epoch = GlonassTime::from_tai(epoch::GLONASS.instant(), STRICT).expect("in range");
        assert_eq!(
            epoch.date().map(|d| (d.four_year_interval, d.day)),
            Ok((1, 1))
        );
        assert_eq!(at(820_454_399), Err(TimeError::OutOfRange));
        // 2100-01-01 00:00 is refused; 2099-12-31 is interval 26's last day.
        assert_eq!(at(4_102_444_800), Err(TimeError::OutOfRange));
        assert_eq!(
            at(4_102_444_799),
            Ok(GlonassDate {
                four_year_interval: 26,
                day: 1_461
            })
        );
    }
}
