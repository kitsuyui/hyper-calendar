//! DCF77, the time code of PTB's long-wave station: German legal time, CET
//! or CEST, for the minute after the frame.
//!
//! PTB, "DCF77 time code" and its figure, read 2026-09-27
//! (`ptb-dcf77-timecode`). "A second mark with a duration of 0.1 s
//! correspond to a binary zero, and a second mark with a duration of
//! 0.2 s to a binary one". Second 59 has no mark — the figure shows none,
//! and the text calls the mark sent there in a leap-second minute
//! "different than before" — so a frame is the 59 marks of seconds 0 to
//! 58, least significant bit of each number first. "Each code emitted contains the information for the following
//! minute."
//!
//! | Seconds | Field |
//! | --- | --- |
//! | 0 | M, "transmitted as binary zero" |
//! | 1–14 | data "provided by a third party": civil warnings and Meteo Time's weather; "The PTB explicitly denies any responsibility for the data content" |
//! | 15 | R, the call bit, "used to signalize irregularities in the control facilities" |
//! | 16 | A1, a change between CET and CEST at the end of the hour |
//! | 17, 18 | Z1 Z2: 01 for CET, 10 for CEST |
//! | 19 | A2, a leap second at the end of the hour |
//! | 20 | S, always 1 |
//! | 21–27, 28 | minute, 1 2 4 8 10 20 40; P1 |
//! | 29–34, 35 | hour, 1 2 4 8 10 20; P2 |
//! | 36–41 | day of the month, 1 2 4 8 10 20 |
//! | 42–44 | weekday, "Monday being day one" |
//! | 45–49 | month, 1 2 4 8 10 |
//! | 50–57, 58 | the year's last two digits, 1 2 4 8 10 20 40 80; P3 |
//!
//! "The three test bits P1, P2 and P3 complement the preceding information
//! words (7 bits for the minute, 6 bits for the hour and 22 bits for the
//! date including the number of the weekday) to an even number of ones."
//! A1 is set for the hour before a change and A2 for the hour before a
//! leap second, whose minute has one mark more: "The 59th second mark …
//! is emitted … with a duration of 0.1 s. After that, the inserted 60th
//! second mark is emitted without carrier reduction", so that frame is 60
//! marks, the last a 0, and names 01:00 CET or 02:00 CEST, midnight UTC.
//!
//! Bits 1–14 are passed through as they came: PTB publishes no layout for
//! them, and says it does not answer for their content.
//!
//! [`summer_time`] reads the zone and A1 from a zone's rules instead of
//! the caller: Z1 Z2 from the offset and flag at the minute announced,
//! and A1 from the zone's transitions, PTB's "Before the transition from
//! CET to CEST or back takes place, A1 is emitted for one hour in state
//! one", from 01:00:16 CET (02:00:16 CEST) until 01:59:16 CET (02:59:16
//! CEST) — the frames sent during the hour before the change, which
//! announce the minutes after its first through the change itself.
//!
//! An omitted leap second is not carried. PTB calls one negligible and
//! says "the technical facilities on the transmitter allow it", but gives
//! no frame for it, so 58 marks are refused as a length.

use hc_calendar::{CivilDateTime, Weekday, gregorian};
use hc_core::UnixTime;
use hc_tz::TimeZone;

use super::{
    Frame, FrameError, FrameResult, minute_reading, odd_ones, read_bcd, split_year, unix_of,
    write_bcd, year_in_century, ymd,
};

const MINUTE: &[&[usize]] = &[&[27, 26, 25], &[24, 23, 22, 21]];
const HOUR: &[&[usize]] = &[&[34, 33], &[32, 31, 30, 29]];
const DAY: &[&[usize]] = &[&[41, 40], &[39, 38, 37, 36]];
const WEEKDAY: &[&[usize]] = &[&[44, 43, 42]];
const MONTH: &[&[usize]] = &[&[49], &[48, 47, 46, 45]];
const YEAR: &[&[usize]] = &[&[57, 56, 55, 54], &[53, 52, 51, 50]];
const P1: usize = 28;
const P2: usize = 35;
const P3: usize = 58;

/// The legal time a frame is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Zone {
    /// Central European Time, UTC+1: Z1 Z2 = 01.
    Cet,
    /// Central European Summer Time, UTC+2: Z1 Z2 = 10.
    Cest,
}

impl Zone {
    /// Hours ahead of UTC.
    #[must_use]
    pub const fn offset_hours(self) -> i64 {
        match self {
            Self::Cet => 1,
            Self::Cest => 2,
        }
    }

    /// The zone a time zone's rules keep at an instant: CET where they
    /// give UTC+1 and standard time, CEST where they give UTC+2 and summer
    /// time, and `None` for anything else, which Z1 Z2 cannot say.
    #[must_use]
    pub fn of(zone: &dyn TimeZone, instant: UnixTime) -> Option<Self> {
        match (zone.offset_at(instant).seconds(), zone.is_dst_at(instant)) {
            (3_600, false) => Some(Self::Cet),
            (7_200, true) => Some(Self::Cest),
            _ => None,
        }
    }
}

/// Z1 Z2 and A1 of the frame that announces the minute beginning at
/// `minute`, from a zone's rules: the zone [`Zone::of`] gives at that
/// minute, and A1 set when the rules change between CET and CEST at an
/// instant from that minute to an hour after it, exclusive — the frames
/// PTB sends during the hour before a change.
///
/// Europe/Berlin is the zone of German legal time, which the station
/// sends. `None` when the zone keeps, at the minute or either side of a
/// change within the hour, a time Z1 Z2 cannot say.
#[must_use]
pub fn summer_time(zone: &dyn TimeZone, minute: UnixTime) -> Option<(Zone, bool)> {
    let current = Zone::of(zone, minute)?;
    let end = minute.seconds().saturating_add(3_600);
    let mut at = UnixTime::from_seconds(minute.seconds().saturating_sub(1));
    while let Some(next) = zone.next_transition(at) {
        if next.seconds() >= end {
            break;
        }
        let before = Zone::of(zone, UnixTime::from_seconds(next.seconds() - 1))?;
        if Zone::of(zone, next)? != before {
            return Some((current, true));
        }
        at = next;
    }
    Some((current, false))
}

/// A decoded DCF77 frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Dcf77Frame {
    /// Bits 1–14, bit 1 as the least significant: the third party's data.
    pub third_party: u16,
    /// R, the call bit.
    pub call_bit: bool,
    /// A1: CET and CEST change at the end of this hour.
    pub zone_change: bool,
    /// Z1 Z2.
    pub zone: Zone,
    /// A2: a leap second at the end of this hour.
    pub leap_second: bool,
    /// The minute the frame announces.
    pub minute: u8,
    /// Its hour.
    pub hour: u8,
    /// Its day of the month.
    pub day: u8,
    /// Its weekday.
    pub weekday: Weekday,
    /// Its month.
    pub month: u8,
    /// The last two digits of its year.
    pub year: u8,
    /// The frame's marks: 59, or 60 in the minute of a leap second.
    pub marks: u8,
}

impl Dcf77Frame {
    /// The frame that announces a minute of legal time, sent during the
    /// minute before it, with the notices given; 60 marks when `leap_second`
    /// is set and the minute is 01:00 CET or 02:00 CEST on the first of a
    /// month.
    ///
    /// # Errors
    ///
    /// [`FrameError::Field`] for a reading that is not the start of a
    /// minute.
    pub fn for_minute(
        reading: CivilDateTime,
        zone: Zone,
        zone_change: bool,
        leap_second: bool,
    ) -> FrameResult<Self> {
        let time = reading.time;
        if time.second() != 0 || time.subsec_attos() != 0 {
            return Err(FrameError::Field("the start of a minute"));
        }
        let (year, month, day) = ymd(reading)?;
        let at_the_leap =
            day == 1 && (time.hour(), time.minute()) == (zone.offset_hours() as u8, 0);
        Ok(Self {
            third_party: 0,
            call_bit: false,
            zone_change,
            zone,
            leap_second,
            minute: time.minute(),
            hour: time.hour(),
            day,
            weekday: Weekday::from_rd(reading.day),
            month,
            year: split_year(year).1,
            marks: if leap_second && at_the_leap { 60 } else { 59 },
        })
    }

    /// Decode the 59 marks of a minute, or 60 in the minute of a leap
    /// second.
    ///
    /// # Errors
    ///
    /// [`FrameError::Length`]; [`FrameError::Symbol`] for M or the extra
    /// mark not 0, or S not 1; [`FrameError::Digit`]; [`FrameError::Parity`];
    /// and [`FrameError::Field`] for a zone of 00 or 11, a weekday of 0, or
    /// a month, day, hour or minute out of range.
    pub fn decode(marks: &[bool]) -> FrameResult<Self> {
        let length = marks.len();
        if !(59..=60).contains(&length) {
            return Err(FrameError::Length(length));
        }
        let bit = |second: usize| marks.get(second).copied().ok_or(FrameError::Length(length));
        if marks[0] {
            return Err(FrameError::Symbol(0));
        }
        if !marks[20] {
            return Err(FrameError::Symbol(20));
        }
        if length == 60 && marks[59] {
            return Err(FrameError::Symbol(59));
        }
        let minute = read_bcd(&bit, MINUTE)? as u8;
        if marks[P1] != odd_ones(&bit, 21..P1)? {
            return Err(FrameError::Parity(P1));
        }
        let hour = read_bcd(&bit, HOUR)? as u8;
        if marks[P2] != odd_ones(&bit, 29..P2)? {
            return Err(FrameError::Parity(P2));
        }
        let day = read_bcd(&bit, DAY)? as u8;
        let weekday = Weekday::from_iso_number(read_bcd(&bit, WEEKDAY)? as u8)
            .ok_or(FrameError::Field("weekday"))?;
        let month = read_bcd(&bit, MONTH)? as u8;
        let year = read_bcd(&bit, YEAR)? as u8;
        if marks[P3] != odd_ones(&bit, 36..P3)? {
            return Err(FrameError::Parity(P3));
        }
        if minute > 59 || hour > 23 || !(1..=31).contains(&day) || !(1..=12).contains(&month) {
            return Err(FrameError::Field("date and time"));
        }
        let zone = match (marks[17], marks[18]) {
            (false, true) => Zone::Cet,
            (true, false) => Zone::Cest,
            _ => return Err(FrameError::Field("zone")),
        };
        let third_party = (1..=14).fold(0u16, |value, second| {
            value | u16::from(marks[second]) << (second - 1)
        });
        Ok(Self {
            third_party,
            call_bit: marks[15],
            zone_change: marks[16],
            zone,
            leap_second: marks[19],
            minute,
            hour,
            day,
            weekday,
            month,
            year,
            marks: length as u8,
        })
    }

    /// The frame's marks.
    ///
    /// # Errors
    ///
    /// [`FrameError::Field`] for a field out of its range, third-party data
    /// wider than 14 bits, or 60 marks without A2; [`FrameError::Length`]
    /// for other than 59 or 60 marks.
    pub fn encode(&self) -> FrameResult<Frame<bool>> {
        let length = usize::from(self.marks);
        if !(59..=60).contains(&length) {
            return Err(FrameError::Length(length));
        }
        if self.minute > 59
            || self.hour > 23
            || !(1..=31).contains(&self.day)
            || !(1..=12).contains(&self.month)
            || self.year > 99
            || self.third_party >> 14 != 0
        {
            return Err(FrameError::Field("date and time"));
        }
        if length == 60 && !self.leap_second {
            return Err(FrameError::Field("leap second"));
        }
        let mut frame = Frame::<bool>::filled(length);
        for second in 1..=14 {
            frame.set(second, self.third_party >> (second - 1) & 1 == 1);
        }
        frame.set(15, self.call_bit);
        frame.set(16, self.zone_change);
        frame.set(17, self.zone == Zone::Cest);
        frame.set(18, self.zone == Zone::Cet);
        frame.set(19, self.leap_second);
        frame.set(20, true);
        let mut set = |second: usize, value: bool| frame.set(second, value);
        write_bcd(&mut set, MINUTE, u32::from(self.minute));
        write_bcd(&mut set, HOUR, u32::from(self.hour));
        write_bcd(&mut set, DAY, u32::from(self.day));
        write_bcd(&mut set, WEEKDAY, u32::from(self.weekday.iso_number()));
        write_bcd(&mut set, MONTH, u32::from(self.month));
        write_bcd(&mut set, YEAR, u32::from(self.year));
        let ones = |range: core::ops::Range<usize>| {
            frame.symbols[range].iter().filter(|&&one| one).count()
        };
        let (p1, p2, p3) = (
            ones(21..P1) % 2 == 1,
            ones(29..P2) % 2 == 1,
            ones(36..P3) % 2 == 1,
        );
        frame.set(P1, p1);
        frame.set(P2, p2);
        frame.set(P3, p3);
        Ok(frame)
    }

    /// The minute of legal time the frame announces, its two-digit year read
    /// in the century beginning `century`.
    ///
    /// # Errors
    ///
    /// [`FrameError::Field`] for a century that is not a multiple of 100, a
    /// date that does not exist, a weekday that is not the date's, or 60
    /// marks anywhere but before 01:00 CET or 02:00 CEST on the first of a
    /// month.
    pub fn reading(&self, century: i64) -> FrameResult<CivilDateTime> {
        let year = year_in_century(century, self.year)?;
        let day = gregorian::to_fixed(year, self.month, self.day)
            .map_err(|_| FrameError::Field("date"))?;
        if Weekday::from_rd(day) != self.weekday {
            return Err(FrameError::Field("weekday"));
        }
        let at_the_leap =
            self.day == 1 && (self.hour, self.minute) == (self.zone.offset_hours() as u8, 0);
        if self.marks == 60 && !at_the_leap {
            return Err(FrameError::Field("leap second"));
        }
        minute_reading(day, self.hour, self.minute)
    }

    /// The POSIX time of the minute the frame announces.
    ///
    /// # Errors
    ///
    /// As [`Dcf77Frame::reading`].
    pub fn to_unix(&self, century: i64) -> FrameResult<UnixTime> {
        unix_of(self.reading(century)?, self.zone.offset_hours())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::CivilTime;

    fn minute(year: i64, month: u8, day: u8, hour: u8, minute: u8) -> CivilDateTime {
        CivilDateTime::new(
            gregorian::to_fixed(year, month, day).expect("exists"),
            CivilTime::hms(hour, minute, 0).expect("valid"),
        )
    }

    /// 2026's changes of Europe/Berlin, on 29 March and 25 October at
    /// 01:00 UTC. The frame announcing 00:01 UTC, sent from 01:00:00 CET
    /// (02:00:00 CEST in October), is the first with A1; the one announcing
    /// the change itself, sent from 01:59:00 CET (02:59:00 CEST), the last.
    #[test]
    fn a1_is_set_for_the_hour_before_a_change() {
        let berlin = hc_tz::builtin::zone("Europe/Berlin").expect("built in");
        let at = |seconds: i64| summer_time(&berlin, UnixTime::from_seconds(seconds));
        for (change, before, after) in [
            (1_774_746_000, Zone::Cet, Zone::Cest),
            (1_792_890_000, Zone::Cest, Zone::Cet),
        ] {
            assert_eq!(at(change - 3_600), Some((before, false)));
            assert_eq!(at(change - 3_540), Some((before, true)));
            assert_eq!(at(change - 60), Some((before, true)));
            assert_eq!(at(change), Some((after, true)));
            assert_eq!(at(change + 60), Some((after, false)));
        }
        // A zone Z1 Z2 cannot say.
        let london = hc_tz::builtin::zone("Europe/London").expect("built in");
        assert_eq!(
            summer_time(&london, UnixTime::from_seconds(1_774_746_000)),
            None
        );
        assert_eq!(Zone::of(&london, UnixTime::from_seconds(0)), None);
    }

    fn marks(text: &str) -> [bool; 59] {
        let mut out = [false; 59];
        for (index, byte) in text.bytes().filter(|byte| *byte != b' ').enumerate() {
            out[index] = byte == b'1';
        }
        out
    }

    /// The frame sent from 14:29:00 CEST on Sunday 27 September 2026, which
    /// names 14:30 CEST: worked from PTB's layout, which comes with no
    /// dated example.
    #[test]
    fn a_dcf77_frame_for_the_following_minute() {
        // M, 1–14, R A1 Z1 Z2 A2 S, minute 30 and P1, hour 14 and P2,
        // day 27, weekday 7, month 9, year 26 and P3.
        let expected =
            marks("0 00000000000000 0 0 1 0 0 1 0000110 0 001010 0 111001 111 10010 01100100 0");
        let reading = minute(2026, 9, 27, 14, 30);
        let frame = Dcf77Frame::for_minute(reading, Zone::Cest, false, false).expect("valid");
        assert_eq!(frame.encode().expect("valid").as_slice(), expected);
        let decoded = Dcf77Frame::decode(&expected).expect("valid");
        assert_eq!(decoded, frame);
        assert_eq!(decoded.weekday, Weekday::Sunday);
        assert_eq!(decoded.reading(2000), Ok(reading));
        // 14:30 CEST is 12:30 UTC.
        assert_eq!(
            decoded.to_unix(2000).expect("valid").seconds(),
            20_723 * 86_400 + 12 * 3_600 + 30 * 60
        );
    }

    /// The leap second of 1 January 2017: the frame sent during 00:59 CET
    /// names 01:00 CET, carries A2, and has a 60th mark, a 0.
    #[test]
    fn the_dcf77_leap_second_frame() {
        let reading = minute(2017, 1, 1, 1, 0);
        let frame = Dcf77Frame::for_minute(reading, Zone::Cet, false, true).expect("valid");
        assert_eq!(frame.marks, 60);
        let marks = frame.encode().expect("valid");
        assert_eq!(marks.len(), 60);
        assert!(marks.as_slice()[19]);
        assert!(!marks.as_slice()[59]);
        let decoded = Dcf77Frame::decode(marks.as_slice()).expect("valid");
        assert_eq!(decoded, frame);
        assert_eq!(decoded.reading(2000), Ok(reading));
        assert_eq!(
            decoded.to_unix(2000),
            Ok(UnixTime::from_seconds(1_483_228_800))
        );
        // A2 an hour ahead does not lengthen an earlier frame; 60 marks
        // elsewhere are refused.
        let earlier = Dcf77Frame::for_minute(minute(2017, 1, 1, 0, 30), Zone::Cet, false, true)
            .expect("valid");
        assert_eq!(earlier.marks, 59);
        let wrong = Dcf77Frame {
            marks: 60,
            ..earlier
        };
        assert_eq!(wrong.reading(2000), Err(FrameError::Field("leap second")));
    }

    /// The third party's bits come back as they were.
    #[test]
    fn bits_1_to_14_are_passed_through() {
        let reading = minute(2026, 9, 27, 14, 30);
        let mut frame = Dcf77Frame::for_minute(reading, Zone::Cest, true, false).expect("valid");
        frame.third_party = 0b10_1100_1110_0101;
        frame.call_bit = true;
        let marks = frame.encode().expect("valid");
        assert!(marks.as_slice()[1] && !marks.as_slice()[2] && marks.as_slice()[14]);
        assert_eq!(Dcf77Frame::decode(marks.as_slice()), Ok(frame));
        frame.third_party = 1 << 14;
        assert!(frame.encode().is_err());
    }

    #[test]
    fn broken_frames_are_refused() {
        let good =
            marks("0 00000000000000 0 0 1 0 0 1 0000110 0 001010 0 111001 111 10010 01100100 0");
        let mut bad = good;
        bad[20] = false;
        assert_eq!(Dcf77Frame::decode(&bad), Err(FrameError::Symbol(20)));
        let mut bad = good;
        bad[28] = true;
        assert_eq!(Dcf77Frame::decode(&bad), Err(FrameError::Parity(28)));
        let mut bad = good;
        bad[58] = true;
        assert_eq!(Dcf77Frame::decode(&bad), Err(FrameError::Parity(58)));
        let mut bad = good;
        bad[17] = false;
        assert_eq!(Dcf77Frame::decode(&bad), Err(FrameError::Field("zone")));
        let mut bad = good;
        // Minute units 1 + 2 + 8 = 11, P1 kept even.
        bad[21] = true;
        bad[22] = true;
        bad[24] = true;
        bad[28] = true;
        assert_eq!(Dcf77Frame::decode(&bad), Err(FrameError::Digit(24)));
        assert_eq!(Dcf77Frame::decode(&good[..58]), Err(FrameError::Length(58)));
        let frame = Dcf77Frame::decode(&good).expect("valid");
        let wrong = Dcf77Frame {
            weekday: Weekday::Monday,
            ..frame
        };
        assert_eq!(wrong.reading(2000), Err(FrameError::Field("weekday")));
    }

    /// The field refusals: a reading that is not the start of a minute, an
    /// hour or month out of range, a weekday of 0, and 60 marks without A2.
    #[test]
    fn field_refusals() {
        let mut reading = minute(2026, 9, 27, 14, 30);
        reading.time = CivilTime::hms(14, 30, 30).expect("valid");
        assert_eq!(
            Dcf77Frame::for_minute(reading, Zone::Cest, false, false),
            Err(FrameError::Field("the start of a minute"))
        );
        let good =
            marks("0 00000000000000 0 0 1 0 0 1 0000110 0 001010 0 111001 111 10010 01100100 0");
        // Hour 14 as 24: 20 for 10, P2 still even.
        let mut bad = good;
        bad[33] = false;
        bad[34] = true;
        assert_eq!(
            Dcf77Frame::decode(&bad),
            Err(FrameError::Field("date and time"))
        );
        // Weekday 0, with P3 kept even.
        let mut bad = good;
        bad[42..=44].fill(false);
        bad[P3] = !bad[P3];
        assert_eq!(Dcf77Frame::decode(&bad), Err(FrameError::Field("weekday")));
        let frame = Dcf77Frame::decode(&good).expect("valid");
        for wrong in [
            Dcf77Frame {
                minute: 60,
                ..frame
            },
            Dcf77Frame { hour: 24, ..frame },
            Dcf77Frame { day: 0, ..frame },
            Dcf77Frame { day: 32, ..frame },
            Dcf77Frame { month: 0, ..frame },
            Dcf77Frame { month: 13, ..frame },
            Dcf77Frame { year: 100, ..frame },
        ] {
            assert_eq!(
                wrong.encode(),
                Err(FrameError::Field("date and time")),
                "{wrong:?}"
            );
        }
        let without_a2 = Dcf77Frame { marks: 60, ..frame };
        assert_eq!(without_a2.encode(), Err(FrameError::Field("leap second")));
        // No length but 59 or 60 is written or read: the 58 marks an omitted
        // second would leave are not carried.
        for marks in [58, 61] {
            assert_eq!(
                Dcf77Frame { marks, ..frame }.encode(),
                Err(FrameError::Length(usize::from(marks)))
            );
        }
        assert_eq!(
            Dcf77Frame::decode(&[false; 58]),
            Err(FrameError::Length(58))
        );
    }

    /// Five minutes of every day of 2000–2099, both zones, both ways:
    /// every day in a release build; every 89th and every month's first
    /// and last in a debug one.
    #[test]
    fn every_code_round_trips() {
        for day in crate::radio::sweep_days_2000_to_2099(89) {
            for (hour, minute_of_hour) in [(0, 0), (1, 0), (2, 0), (13, 37), (23, 59)] {
                for zone in [Zone::Cet, Zone::Cest] {
                    let reading = CivilDateTime::new(
                        hc_calendar::Rd(day),
                        CivilTime::hms(hour, minute_of_hour, 0).expect("valid"),
                    );
                    let frame = Dcf77Frame::for_minute(reading, zone, false, false).expect("valid");
                    let marks = frame.encode().expect("valid");
                    let back = Dcf77Frame::decode(marks.as_slice()).expect("valid");
                    assert_eq!(back, frame);
                    assert_eq!(back.reading(2000), Ok(reading));
                }
            }
        }
    }
}
