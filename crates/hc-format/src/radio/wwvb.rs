//! WWVB, NIST's long-wave station: the amplitude code and the phase code,
//! both UTC, both the minute at the frame's first second.
//!
//! **The amplitude code.** NIST Special Publication 432, *NIST Time and
//! Frequency Services*, 2002 edition (`nist-sp432-2002`), Table 2.3 and its
//! text, and SP 250-67, *NIST Time and Frequency Radio Stations*, 2005
//! (`nist-sp250-67`), chapter 2 §2, both read 2026-09-27. Each second is a
//! 0, a 1 or a marker; markers at 0, 9, 19, 29, 39, 49 and 59; BCD most
//! significant bit first. "The decoded UTC at the start of the frame is
//! 2001, 258 days, 18 hours, and 42 minutes" is SP 432's example.
//!
//! | Seconds | Field |
//! | --- | --- |
//! | 1–3, 5–8 | minute |
//! | 12–13, 15–18 | hour |
//! | 22–23, 25–28, 30–33 | day of the year |
//! | 36–38 | UT1 − UTC's sign: 1 0 1 positive, 0 1 0 negative |
//! | 40–43 | its magnitude, 0.8 0.4 0.2 0.1 s |
//! | 45–48, 50–53 | the year's last two digits |
//! | 55 | leap year indicator, set "usually sometime in January but before February 29" |
//! | 56 | leap second warning: "a leap second will be added to UTC at the end of the current month" |
//! | 57, 58 | summer time: 00 standard, 11 daylight saving time, 57 changing at 00:00 UTC on the day of a change and 58 a day later |
//!
//! **The phase code.** NIST, "Enhanced WWVB Broadcast Format", revision
//! 1.01, 6 November 2013 (`nist-wwvb-enhanced-2013`), read 2026-09-27: a
//! second phase-modulated frame of 60 bits in the same minute, "the minute
//! being encoded in the broadcast is the one that has already started". Its
//! time is a 26-bit count of "the number of minutes that have elapsed since
//! 00:00UTC on January 1st in the year 2000", "reset at the beginning of
//! year XX00", "considering 60 minutes per hour and 24 hours per day",
//! guarded by five Hamming parity bits (§4.3); a 5-bit word for summer
//! time and the leap second (Table 4); and a 6-bit word for the next change
//! of summer time (Table 8).
//!
//! **Leap seconds**, in both codes (§4.4): "the time frame representing the
//! extended 61-second minute, starting at 23:59:00UTC, will have bit 59
//! repeated (a marker in the legacy broadcast and a '0' in PM)", and for a
//! negative one "bit 59 removed". That minute is the last of a month.
//!
//! Not carried: the phase code's message frames (sync_M) and its six-minute
//! sequences at 10 and 40 minutes past the hour, which carry no time frame,
//! and the Hamming code's correction of one error, since a frame whose
//! parity does not match is refused.

use hc_calendar::{CalendarResult, CivilDateTime, Rd, Weekday, gregorian};
use hc_core::UnixTime;
use hc_tz::TimeZone;

use super::{
    Frame, FrameError, FrameResult, LeapNotice, Symbol, check_length, day_of_year,
    is_last_minute_of_month, minute_reading, read_bcd, read_bits, split_year, unix_of, write_bcd,
    year_in_century, ymd,
};

/// The summer-time state both codes carry: in the amplitude code bits 57
/// and 58, in the phase code `dst_on[1]` and `dst_on[0]` (Table 5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DstState {
    /// 00: standard time, "DST has not been in effect for over a day".
    Standard,
    /// 10: "DST starts today".
    BeginsToday,
    /// 11: "DST has been in effect for more than a day".
    InEffect,
    /// 01: "DST ends today".
    EndsToday,
}

impl DstState {
    /// The two bits, bit 57 (`dst_on[1]`) first.
    #[must_use]
    pub const fn bits(self) -> (bool, bool) {
        match self {
            Self::Standard => (false, false),
            Self::BeginsToday => (true, false),
            Self::InEffect => (true, true),
            Self::EndsToday => (false, true),
        }
    }

    /// The state of the UTC day a minute falls in, from a zone's rules.
    ///
    /// SP 250-67, chapter 2 §2 (`nist-sp250-67`): "At 0000 UTC on the day
    /// ST changes to DST, bit 57 is set to a one, and bit 58 is set to a
    /// one at 0000 UTC the following day. When DST ends, bit 57 is set to
    /// zero at 0000 UTC the day of the change, and bit 58 goes low 24
    /// hours later." So bit 57 is whether the zone keeps saving time at
    /// 24:00 UTC ending the day and bit 58 whether at 00:00 UTC beginning
    /// it — which reads the rules for the day of a change as the station
    /// does wherever the change falls within that UTC day, as the United
    /// States' 02:00 local change does in every zone of the contiguous
    /// states. America/Denver is the station's, near Fort Collins.
    #[must_use]
    pub fn of_day(zone: &dyn TimeZone, minute: UnixTime) -> Self {
        let start = minute.seconds().div_euclid(86_400).saturating_mul(86_400);
        Self::from_bits(
            zone.is_dst_at(UnixTime::from_seconds(start.saturating_add(86_400))),
            zone.is_dst_at(UnixTime::from_seconds(start)),
        )
    }

    /// The state of two bits, bit 57 first.
    #[must_use]
    pub const fn from_bits(first: bool, second: bool) -> Self {
        match (first, second) {
            (false, false) => Self::Standard,
            (true, false) => Self::BeginsToday,
            (true, true) => Self::InEffect,
            (false, true) => Self::EndsToday,
        }
    }
}

// ---------------------------------------------------------------------
// The amplitude code.
// ---------------------------------------------------------------------

const MINUTE: &[&[usize]] = &[&[1, 2, 3], &[5, 6, 7, 8]];
const HOUR: &[&[usize]] = &[&[12, 13], &[15, 16, 17, 18]];
const DAY: &[&[usize]] = &[&[22, 23], &[25, 26, 27, 28], &[30, 31, 32, 33]];
const DUT1: &[&[usize]] = &[&[40, 41, 42, 43]];
const YEAR: &[&[usize]] = &[&[45, 46, 47, 48], &[50, 51, 52, 53]];
const AM_ZEROS: [usize; 11] = [4, 10, 11, 14, 20, 21, 24, 34, 35, 44, 54];
const LEAP_YEAR: usize = 55;
const LEAP_SECOND_WARNING: usize = 56;

/// A decoded frame of the amplitude code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AmFrame {
    /// The minute of UTC at the frame's first marker.
    pub minute: u8,
    /// The hour.
    pub hour: u8,
    /// The day of the year, 1 January being 1.
    pub day_of_year: u16,
    /// The year's last two digits.
    pub year: u8,
    /// UT1 − UTC in tenths of a second, −9 to 9.
    pub dut1_tenths: i8,
    /// Bit 55.
    pub leap_year: bool,
    /// Bit 56: a second is added at the end of the month.
    pub leap_second_warning: bool,
    /// Bits 57 and 58.
    pub dst: DstState,
    /// The frame's seconds: 60, or 61 or 59 in the minute of a leap second.
    pub seconds: u8,
}

/// The markers of a frame of `seconds` seconds.
fn am_marker(second: usize, seconds: u8) -> bool {
    matches!(second, 0 | 9 | 19 | 29 | 39 | 49) || (second == 59 || second == 60) && seconds >= 60
}

impl AmFrame {
    /// The frame of a UTC minute, with the leap year bit set in every day
    /// of a leap year, and 61 seconds long when `leap_second_warning` is
    /// set and the minute is 23:59 on the last day of a month. The station
    /// may set the leap year bit only later in January; a frame decoded
    /// from the air keeps the bit it had.
    ///
    /// # Errors
    ///
    /// [`FrameError::Field`] for a reading that is not the start of a
    /// minute, or a UT1 − UTC outside ±0.9 s.
    pub fn for_minute(
        reading: CivilDateTime,
        dut1_tenths: i8,
        dst: DstState,
        leap_second_warning: bool,
    ) -> FrameResult<Self> {
        let time = reading.time;
        if time.second() != 0 || time.subsec_attos() != 0 {
            return Err(FrameError::Field("the start of a minute"));
        }
        if !(-9..=9).contains(&dut1_tenths) {
            return Err(FrameError::Field("UT1 correction"));
        }
        let (year, month, day) = ymd(reading)?;
        let day_of_year =
            gregorian::day_of_year(year, month, day).map_err(|_| FrameError::Field("date"))?;
        let at_the_leap = is_last_minute_of_month(reading, 23, 59);
        Ok(Self {
            minute: time.minute(),
            hour: time.hour(),
            day_of_year,
            year: split_year(year).1,
            dut1_tenths,
            leap_year: gregorian::is_leap_year(year),
            leap_second_warning,
            dst,
            seconds: if leap_second_warning && at_the_leap {
                61
            } else {
                60
            },
        })
    }

    /// Decode a frame of 59, 60 or 61 symbols.
    ///
    /// # Errors
    ///
    /// [`FrameError::Length`]; [`FrameError::Symbol`] for a marker out of
    /// place or a 1 in a second fixed at 0; [`FrameError::Digit`]; and
    /// [`FrameError::Field`] for a UT1 sign other than 1 0 1 or 0 1 0, or an
    /// hour or minute out of range.
    pub fn decode(symbols: &[Symbol]) -> FrameResult<Self> {
        let seconds = check_length(symbols.len())?;
        for (second, &symbol) in symbols.iter().enumerate() {
            if (symbol == Symbol::Marker) != am_marker(second, seconds) {
                return Err(FrameError::Symbol(second));
            }
        }
        let bit = |second: usize| match symbols.get(second) {
            Some(Symbol::One) => Ok(true),
            Some(Symbol::Zero) => Ok(false),
            _ => Err(FrameError::Symbol(second)),
        };
        for second in AM_ZEROS {
            if bit(second)? {
                return Err(FrameError::Symbol(second));
            }
        }
        let minute = read_bcd(&bit, MINUTE)? as u8;
        let hour = read_bcd(&bit, HOUR)? as u8;
        if minute > 59 || hour > 23 {
            return Err(FrameError::Field("time of day"));
        }
        let magnitude = read_bcd(&bit, DUT1)? as i8;
        let dut1_tenths = match (bit(36)?, bit(37)?, bit(38)?) {
            (true, false, true) => magnitude,
            (false, true, false) => -magnitude,
            _ => return Err(FrameError::Field("UT1 sign")),
        };
        Ok(Self {
            minute,
            hour,
            day_of_year: read_bcd(&bit, DAY)? as u16,
            year: read_bcd(&bit, YEAR)? as u8,
            dut1_tenths,
            leap_year: bit(LEAP_YEAR)?,
            leap_second_warning: bit(LEAP_SECOND_WARNING)?,
            dst: DstState::from_bits(bit(57)?, bit(58)?),
            seconds,
        })
    }

    /// The frame's symbols. A UT1 − UTC of zero is written with the
    /// positive sign.
    ///
    /// # Errors
    ///
    /// [`FrameError::Field`] for a field out of its range, or 61 seconds
    /// without the leap second warning; [`FrameError::Length`] for other
    /// than 59 to 61 seconds.
    pub fn encode(&self) -> FrameResult<Frame<Symbol>> {
        let seconds = check_length(usize::from(self.seconds))?;
        if self.minute > 59
            || self.hour > 23
            || !(1..=366).contains(&self.day_of_year)
            || self.year > 99
            || !(-9..=9).contains(&self.dut1_tenths)
        {
            return Err(FrameError::Field("date and time"));
        }
        if seconds == 61 && !self.leap_second_warning {
            return Err(FrameError::Field("leap second"));
        }
        let mut frame = Frame::filled(usize::from(seconds));
        for second in 0..usize::from(seconds) {
            if am_marker(second, seconds) {
                frame.set(second, Symbol::Marker);
            }
        }
        let mut set = |second: usize, value: bool| frame.set(second, Symbol::bit(value));
        write_bcd(&mut set, MINUTE, u32::from(self.minute));
        write_bcd(&mut set, HOUR, u32::from(self.hour));
        write_bcd(&mut set, DAY, u32::from(self.day_of_year));
        write_bcd(&mut set, DUT1, u32::from(self.dut1_tenths.unsigned_abs()));
        write_bcd(&mut set, YEAR, u32::from(self.year));
        let negative = self.dut1_tenths < 0;
        set(36, !negative);
        set(37, negative);
        set(38, !negative);
        set(LEAP_YEAR, self.leap_year);
        set(LEAP_SECOND_WARNING, self.leap_second_warning);
        let (first, second) = self.dst.bits();
        set(57, first);
        set(58, second);
        Ok(frame)
    }

    /// The UTC minute the frame names, its two-digit year read in the
    /// century beginning `century`.
    ///
    /// # Errors
    ///
    /// [`FrameError::Field`] for a century that is not a multiple of 100, a
    /// day the year does not have, the leap year bit in a common year, or a
    /// frame of 61 or 59 seconds anywhere but 23:59 on the last day of a
    /// month, or of 61 without the leap second warning.
    pub fn reading(&self, century: i64) -> FrameResult<CivilDateTime> {
        let year = year_in_century(century, self.year)?;
        if self.leap_year && !gregorian::is_leap_year(year) {
            return Err(FrameError::Field("leap year"));
        }
        let reading = minute_reading(day_of_year(year, self.day_of_year)?, self.hour, self.minute)?;
        let at_the_leap = is_last_minute_of_month(reading, 23, 59);
        if self.seconds != 60 && !at_the_leap || self.seconds == 61 && !self.leap_second_warning {
            return Err(FrameError::Field("leap second"));
        }
        Ok(reading)
    }

    /// The POSIX time of the frame's first marker.
    ///
    /// # Errors
    ///
    /// As [`AmFrame::reading`].
    pub fn to_unix(&self, century: i64) -> FrameResult<UnixTime> {
        unix_of(self.reading(century)?, 0)
    }
}

// ---------------------------------------------------------------------
// The phase code.
// ---------------------------------------------------------------------

/// Table 3's synchronisation word of a time frame, `sync_T`, bits 0–12.
const SYNC_T: u32 = 0b0_0111_0110_1000;
/// Table 3's synchronisation word of a message frame, `sync_M`.
const SYNC_M: u32 = 0b1_1010_0011_1010;
const SYNC: [usize; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
/// `time_par[4]` to `time_par[0]`.
const PARITY: [usize; 5] = [13, 14, 15, 16, 17];
/// `dst_ls[4]` to `dst_ls[0]`.
const DST_LS: [usize; 5] = [47, 48, 50, 51, 52];
/// `dst_next[5]` to `dst_next[0]`.
const DST_NEXT: [usize; 6] = [53, 54, 55, 56, 57, 58];
const NOTICE: usize = 49;
const RESERVED: [usize; 2] = [29, 39];
/// The repeat of `time[0]`, on the amplitude code's marker at 19.
const TIME_0_REPEAT: usize = 19;

/// The second that carries `time[bit]`: bit 25 at 18, 24–16 at 20–28,
/// 15–7 at 30–38 and 6–0 at 40–46 (§4.3 and Table 1).
const fn time_second(bit: usize) -> usize {
    match bit {
        25 => 18,
        16..=24 => 20 + (24 - bit),
        7..=15 => 30 + (15 - bit),
        _ => 40 + (6 - bit),
    }
}

/// §4.3: the bits of the time word each parity bit sums, `time_par[0]`
/// first.
const PARITY_SUMS: [[u8; 15]; 5] = [
    [23, 21, 20, 17, 16, 15, 14, 13, 9, 8, 6, 5, 4, 2, 0],
    [24, 22, 21, 18, 17, 16, 15, 14, 10, 9, 7, 6, 5, 3, 1],
    [25, 23, 22, 19, 18, 17, 16, 15, 11, 10, 8, 7, 6, 4, 2],
    [24, 21, 19, 18, 15, 14, 13, 12, 11, 7, 6, 4, 3, 2, 0],
    [25, 22, 20, 19, 16, 15, 14, 13, 12, 8, 7, 5, 4, 3, 1],
];

/// The five parity bits of a minute count, `time_par[4..0]` as a number.
const fn time_parity(count: u32) -> u32 {
    let mut parity = 0;
    let mut index = 0;
    while index < 5 {
        let mut sum = 0;
        let mut term = 0;
        while term < 15 {
            sum ^= count >> PARITY_SUMS[index][term] & 1;
            term += 1;
        }
        parity |= sum << index;
        index += 1;
    }
    parity
}

/// Table 4: the twelve words of `dst_ls`, and their summer-time state and
/// leap-second notice.
const TABLE_4: [(u8, DstState, LeapNotice); 12] = [
    (0b01000, DstState::Standard, LeapNotice::None),
    (0b10110, DstState::BeginsToday, LeapNotice::None),
    (0b00011, DstState::InEffect, LeapNotice::None),
    (0b10101, DstState::EndsToday, LeapNotice::None),
    (0b00100, DstState::Standard, LeapNotice::Negative),
    (0b10000, DstState::BeginsToday, LeapNotice::Negative),
    (0b01101, DstState::InEffect, LeapNotice::Negative),
    (0b01110, DstState::EndsToday, LeapNotice::Negative),
    (0b11001, DstState::Standard, LeapNotice::Positive),
    (0b11010, DstState::BeginsToday, LeapNotice::Positive),
    (0b11111, DstState::InEffect, LeapNotice::Positive),
    (0b11100, DstState::EndsToday, LeapNotice::Positive),
];

/// A scheduled change of summer time that Table 8 can name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DstTransition {
    /// Into summer time, in the spring; or out of it, in the autumn.
    pub into_dst: bool,
    /// The Sunday, in weeks from the first Sunday of March (into summer
    /// time, 0 to 7, "M" in the table) or of November (out of it, −4 to 3,
    /// "N").
    pub weeks: i8,
    /// The local hour of the change, 1, 2 or 3 AM: "skip from 2:00AM to
    /// 3:00AM" is 2, "instead of 2:00AM move back to 1:00AM" is 2.
    pub hour: u8,
}

impl DstTransition {
    /// The Sunday of the change in `year`.
    ///
    /// # Errors
    ///
    /// A [`hc_calendar::CalendarError`] for a year outside the calendar's
    /// range.
    pub fn day(self, year: i64) -> CalendarResult<Rd> {
        let month = if self.into_dst { 3 } else { 11 };
        let anchor = Weekday::Sunday.on_or_after(gregorian::to_fixed(year, month, 1)?);
        Ok(Rd(anchor.0 + 7 * i64::from(self.weeks)))
    }
}

/// What Table 8's word says of the next change of summer time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DstNext {
    /// A change on one of the table's 48 schedules.
    Transition(DstTransition),
    /// Word 49: "DST transition occurs at different time", no advance
    /// notice.
    DifferentTime,
    /// Word 50: "no DST period scheduled this year".
    NoDst,
    /// Word 51: "DST in effect for this whole year".
    AllYear,
    /// Words 52–56, reserved; the number is 1 to 5.
    Reserved(u8),
}

/// Table 8's schedule words into summer time, for the change at 1, 2 and
/// 3 AM, each on the first Sunday of March and the seven after it.
const INTO_DST: [[u8; 8]; 3] = [
    [
        0b110001, 0b100110, 0b100101, 0b010101, 0b111110, 0b010110, 0b110111, 0b111101,
    ],
    [
        0b101010, 0b011011, 0b001110, 0b000001, 0b000010, 0b001000, 0b001101, 0b101001,
    ],
    [
        0b000100, 0b100000, 0b110100, 0b101100, 0b111000, 0b010000, 0b110010, 0b011100,
    ],
];

/// Table 8's words out of summer time, for the change at 1, 2 and 3 AM,
/// on the fourth Sunday before the first Sunday of November to the fourth
/// Sunday of November.
const OUT_OF_DST: [[u8; 8]; 3] = [
    [
        0b110111, 0b010101, 0b110001, 0b010110, 0b100110, 0b111110, 0b100101, 0b111101,
    ],
    [
        0b001101, 0b000001, 0b101010, 0b001000, 0b011011, 0b000010, 0b001110, 0b101001,
    ],
    [
        0b110010, 0b101100, 0b000100, 0b010000, 0b100000, 0b111000, 0b110100, 0b011100,
    ],
];

/// Table 8's words 49 to 56, whatever the summer-time state.
const MESSAGES: [(u8, DstNext); 8] = [
    (0b100011, DstNext::DifferentTime),
    (0b000111, DstNext::NoDst),
    (0b101111, DstNext::AllYear),
    (0b110000, DstNext::Reserved(1)),
    (0b100100, DstNext::Reserved(2)),
    (0b010100, DstNext::Reserved(3)),
    (0b110110, DstNext::Reserved(4)),
    (0b110101, DstNext::Reserved(5)),
];

impl DstNext {
    /// The meaning of a word read with the summer-time state's first bit,
    /// `dst_on[1]`, which says whether the next change is out of summer
    /// time.
    #[must_use]
    pub fn from_word(word: u8, dst_on: bool) -> Option<Self> {
        if let Some(&(_, message)) = MESSAGES.iter().find(|(code, _)| *code == word) {
            return Some(message);
        }
        let (table, first_week) = if dst_on {
            (&OUT_OF_DST, -4)
        } else {
            (&INTO_DST, 0)
        };
        table.iter().zip(1u8..).find_map(|(row, hour)| {
            row.iter().position(|&code| code == word).map(|index| {
                Self::Transition(DstTransition {
                    into_dst: !dst_on,
                    weeks: first_week + index as i8,
                    hour,
                })
            })
        })
    }

    /// The word a zone's rules give the UTC day a minute falls in: the
    /// next change of summer time after that day's end, in the direction
    /// [`DstState::of_day`]'s bit 57 (`dst_on[1]`) says it goes.
    ///
    /// The Enhanced WWVB Broadcast Format, §4.6
    /// (`nist-wwvb-enhanced-2013`): "When DST is in effect (in the spring
    /// or summer), the DST_NEXT field provides advance notification for
    /// the end of the DST period in the fall, whereas when DST is not in
    /// effect, as is the case in the winter, this field provides advance
    /// notification for the beginning of the next DST period in the
    /// upcoming spring." Table 8's words are read with `dst_on[1]`, which
    /// turns at 00:00 UTC on the day of a change, so the word turns with
    /// it: from that instant it names the change after the one the day
    /// holds. The change is Table 8's schedule when it falls on one of its
    /// Sundays — the first of March and the seven after it into summer
    /// time, the fourth before the first of November to the third after it
    /// out of it — at 1, 2 or 3 AM on the clock before the change, as the
    /// table's "after 1:59AM, skip from 2:00AM to 3:00AM" and "after
    /// 1:59AM, instead of 2:00AM move back to 1:00AM" are both 2 AM; any
    /// other day or time is word 49, "DST transition occurs at different
    /// time", which "will serve to convey that no advance notification can
    /// be provided". A zone that makes no change in the year after the
    /// day's end is word 50, "no DST period scheduled this year", which the
    /// document reserves "for the possibility of DST being cancelled (i.e.
    /// standard time is maintained throughout the year)", or word 51, "DST
    /// in effect for this whole year", reserved "for the case of DST being
    /// permanently in effect", as the zone keeps summer time or not.
    #[must_use]
    pub fn of_day(zone: &dyn TimeZone, minute: UnixTime) -> Self {
        /// A year and a day: past it, a zone has scheduled no change.
        const HORIZON: i64 = 367 * 86_400;
        /// More changes than this in a year are not summer time.
        const MOST_CHANGES: usize = 64;
        let start = minute.seconds().div_euclid(86_400).saturating_mul(86_400);
        let end = start.saturating_add(86_400);
        let dst_on = zone.is_dst_at(UnixTime::from_seconds(end));
        let mut at = UnixTime::from_seconds(end);
        let mut change = None;
        for _ in 0..MOST_CHANGES {
            let Some(next) = zone.next_transition(at) else {
                break;
            };
            if next.seconds() > end.saturating_add(HORIZON) {
                break;
            }
            if zone.is_dst_at(next) != dst_on {
                change = Some(next);
                break;
            }
            at = next;
        }
        let Some(change) = change else {
            return if dst_on { Self::AllYear } else { Self::NoDst };
        };
        Self::schedule(zone, change, !dst_on).unwrap_or(Self::DifferentTime)
    }

    /// Table 8's schedule for a change at `change`, read on the clock the
    /// zone keeps just before it, or `None` for one the table does not
    /// list.
    fn schedule(zone: &dyn TimeZone, change: UnixTime, into_dst: bool) -> Option<Self> {
        let before = zone.offset_at(UnixTime::from_seconds(change.seconds().checked_sub(1)?));
        let local = change.seconds().checked_add(i64::from(before.seconds()))?;
        let seconds_of_day = local.rem_euclid(86_400);
        let day = Rd::from_unix_days(local.div_euclid(86_400));
        if seconds_of_day % 3_600 != 0 || Weekday::from_rd(day) != Weekday::Sunday {
            return None;
        }
        let hour = u8::try_from(seconds_of_day / 3_600).ok()?;
        let (month, first_week) = if into_dst { (3, 0) } else { (11, -4) };
        let anchor = Weekday::Sunday
            .on_or_after(gregorian::to_fixed(gregorian::year_from_fixed(day), month, 1).ok()?);
        let weeks = i8::try_from((day.0 - anchor.0).div_euclid(7)).ok()?;
        if !(1..=3).contains(&hour) || !(first_week..first_week + 8).contains(&weeks) {
            return None;
        }
        Some(Self::Transition(DstTransition {
            into_dst,
            weeks,
            hour,
        }))
    }

    /// The word, if the table has one for this meaning.
    #[must_use]
    pub fn word(self) -> Option<u8> {
        match self {
            Self::Transition(transition) => {
                let (table, first_week) = if transition.into_dst {
                    (&INTO_DST, 0)
                } else {
                    (&OUT_OF_DST, -4)
                };
                let row = table.get(usize::from(transition.hour.checked_sub(1)?))?;
                row.get(usize::try_from(transition.weeks - first_week).ok()?)
                    .copied()
            }
            message => MESSAGES
                .iter()
                .find(|(_, meaning)| *meaning == message)
                .map(|&(code, _)| code),
        }
    }
}

/// A decoded time frame of the phase code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PmFrame {
    /// The minutes since 00:00 UTC on 1 January of the century's first
    /// year, 26 bits.
    pub minute_of_century: u32,
    /// `dst_on`.
    pub dst: DstState,
    /// `leap_sec`: a leap second at the end of this month.
    pub leap: LeapNotice,
    /// Bit 49: NIST has posted a notice.
    pub notice: bool,
    /// `dst_next`.
    pub next: DstNext,
    /// Bits 29 and 39, reserved.
    pub reserved: [bool; 2],
    /// The frame's seconds: 60, or 61 or 59 in the minute of a leap second.
    pub seconds: u8,
}

impl PmFrame {
    /// The frame of a UTC minute in the century beginning `century`, 61 or
    /// 59 seconds long when `leap` is announced and the minute is 23:59 on
    /// the last day of a month.
    ///
    /// # Errors
    ///
    /// [`FrameError::Field`] for a reading that is not the start of a
    /// minute or not in the century, a century that is not a multiple of
    /// 100, or a schedule in `next` that does not go the way `dst` says
    /// the next change goes.
    pub fn for_minute(
        reading: CivilDateTime,
        century: i64,
        dst: DstState,
        leap: LeapNotice,
        next: DstNext,
    ) -> FrameResult<Self> {
        let time = reading.time;
        if time.second() != 0 || time.subsec_attos() != 0 {
            return Err(FrameError::Field("the start of a minute"));
        }
        let start = century_start(century)?;
        let (year, _, _) = ymd(reading)?;
        if !(century..century + 100).contains(&year) {
            return Err(FrameError::Field("minute count"));
        }
        let minute_of_century = (reading.day.0 - start.0) * 1_440
            + i64::from(time.hour()) * 60
            + i64::from(time.minute());
        let at_the_leap = is_last_minute_of_month(reading, 23, 59);
        let frame = Self {
            minute_of_century: minute_of_century as u32,
            dst,
            leap,
            notice: false,
            next,
            reserved: [false; 2],
            seconds: match leap {
                LeapNotice::Positive if at_the_leap => 61,
                LeapNotice::Negative if at_the_leap => 59,
                _ => 60,
            },
        };
        frame.dst_word()?;
        Ok(frame)
    }

    fn dst_word(&self) -> FrameResult<(u8, u8)> {
        let dst_ls = TABLE_4
            .iter()
            .find(|(_, dst, leap)| *dst == self.dst && *leap == self.leap)
            .map(|&(code, _, _)| code)
            .ok_or(FrameError::Field("dst_ls"))?;
        let dst_next = self.next.word().ok_or(FrameError::Field("dst_next"))?;
        let (dst_on, _) = self.dst.bits();
        if DstNext::from_word(dst_next, dst_on) != Some(self.next) {
            return Err(FrameError::Field("dst_next"));
        }
        Ok((dst_ls, dst_next))
    }

    /// Decode a frame of 59, 60 or 61 bits.
    ///
    /// # Errors
    ///
    /// [`FrameError::Length`]; [`FrameError::Symbol`] for a synchronisation
    /// word that is not sync_T, a 1 at bit 59 or at its repeat, or a
    /// repeat of `time[0]` that differs; [`FrameError::Field`] with
    /// `"message frame"` for sync_M, and for a `dst_ls` or `dst_next` word
    /// Tables 4 and 8 do not list; and [`FrameError::Parity`] at the first
    /// parity bit that does not match.
    pub fn decode(bits: &[bool]) -> FrameResult<Self> {
        let seconds = check_length(bits.len())?;
        let bit = |second: usize| {
            bits.get(second)
                .copied()
                .ok_or(FrameError::Length(bits.len()))
        };
        match read_bits(bit, &SYNC)? {
            SYNC_T => {}
            SYNC_M => return Err(FrameError::Field("message frame")),
            _ => return Err(FrameError::Symbol(0)),
        }
        if seconds >= 60 && bits[59] {
            return Err(FrameError::Symbol(59));
        }
        if seconds == 61 && bits[60] {
            return Err(FrameError::Symbol(60));
        }
        let count = (0..26).try_fold(0u32, |value, index| {
            Ok(value | u32::from(bit(time_second(index))?) << index)
        })?;
        if bits[TIME_0_REPEAT] != bits[time_second(0)] {
            return Err(FrameError::Symbol(TIME_0_REPEAT));
        }
        let parity = time_parity(count);
        for (index, &second) in PARITY.iter().enumerate() {
            if bits[second] != (parity >> (4 - index) & 1 == 1) {
                return Err(FrameError::Parity(second));
            }
        }
        let dst_ls = read_bits(bit, &DST_LS)? as u8;
        let &(_, dst, leap) = TABLE_4
            .iter()
            .find(|(code, _, _)| *code == dst_ls)
            .ok_or(FrameError::Field("dst_ls"))?;
        let (dst_on, _) = dst.bits();
        let next = DstNext::from_word(read_bits(bit, &DST_NEXT)? as u8, dst_on)
            .ok_or(FrameError::Field("dst_next"))?;
        Ok(Self {
            minute_of_century: count,
            dst,
            leap,
            notice: bits[NOTICE],
            next,
            reserved: [bits[RESERVED[0]], bits[RESERVED[1]]],
            seconds,
        })
    }

    /// The frame's bits.
    ///
    /// # Errors
    ///
    /// [`FrameError::Field`] for a count wider than 26 bits, a state and
    /// notice Tables 4 and 8 cannot write, or 61 or 59 seconds without the
    /// leap-second notice of that sign; [`FrameError::Length`] for other
    /// than 59 to 61 seconds.
    pub fn encode(&self) -> FrameResult<Frame<bool>> {
        let seconds = check_length(usize::from(self.seconds))?;
        if self.minute_of_century >> 26 != 0 {
            return Err(FrameError::Field("minute count"));
        }
        let expected = match seconds {
            61 => LeapNotice::Positive,
            59 => LeapNotice::Negative,
            _ => self.leap,
        };
        if self.leap != expected {
            return Err(FrameError::Field("leap second"));
        }
        let (dst_ls, dst_next) = self.dst_word()?;
        let mut frame = Frame::<bool>::filled(usize::from(seconds));
        for (index, &second) in SYNC.iter().enumerate() {
            frame.set(second, SYNC_T >> (12 - index) & 1 == 1);
        }
        for index in 0..26 {
            frame.set(time_second(index), self.minute_of_century >> index & 1 == 1);
        }
        frame.set(TIME_0_REPEAT, self.minute_of_century & 1 == 1);
        let parity = time_parity(self.minute_of_century);
        for (index, &second) in PARITY.iter().enumerate() {
            frame.set(second, parity >> (4 - index) & 1 == 1);
        }
        for (index, &second) in DST_LS.iter().enumerate() {
            frame.set(second, dst_ls >> (4 - index) & 1 == 1);
        }
        for (index, &second) in DST_NEXT.iter().enumerate() {
            frame.set(second, dst_next >> (5 - index) & 1 == 1);
        }
        frame.set(NOTICE, self.notice);
        frame.set(RESERVED[0], self.reserved[0]);
        frame.set(RESERVED[1], self.reserved[1]);
        Ok(frame)
    }

    /// The UTC minute the frame names, its count read from the start of the
    /// century beginning `century`.
    ///
    /// # Errors
    ///
    /// [`FrameError::Field`] for a century that is not a multiple of 100, a
    /// count past the century's end, or a frame of 61 or 59 seconds
    /// anywhere but 23:59 on the last day of a month.
    pub fn reading(&self, century: i64) -> FrameResult<CivilDateTime> {
        let start = century_start(century)?;
        let days = i64::from(self.minute_of_century / 1_440);
        let minute_of_day = self.minute_of_century % 1_440;
        let reading = minute_reading(
            Rd(start.0 + days),
            (minute_of_day / 60) as u8,
            (minute_of_day % 60) as u8,
        )?;
        let (year, _, _) = ymd(reading)?;
        if year >= century + 100 {
            return Err(FrameError::Field("minute count"));
        }
        if self.seconds != 60 && !is_last_minute_of_month(reading, 23, 59) {
            return Err(FrameError::Field("leap second"));
        }
        Ok(reading)
    }

    /// The POSIX time of the frame's first second.
    ///
    /// # Errors
    ///
    /// As [`PmFrame::reading`].
    pub fn to_unix(&self, century: i64) -> FrameResult<UnixTime> {
        unix_of(self.reading(century)?, 0)
    }
}

/// 1 January of the first year of the century beginning `century`.
fn century_start(century: i64) -> FrameResult<Rd> {
    year_in_century(century, 0)?;
    gregorian::to_fixed(century, 1, 1).map_err(|_| FrameError::Field("century"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::CivilTime;

    const M: Symbol = Symbol::Marker;
    const O: Symbol = Symbol::Zero;
    const I: Symbol = Symbol::One;

    fn minute(year: i64, month: u8, day: u8, hour: u8, minute: u8) -> CivilDateTime {
        CivilDateTime::new(
            gregorian::to_fixed(year, month, day).expect("exists"),
            CivilTime::hms(hour, minute, 0).expect("valid"),
        )
    }

    /// 2026's changes of the United States' rule in Denver: 8 March at
    /// 09:00 UTC and 1 November at 08:00 UTC. Bit 57 turns at 00:00 UTC on
    /// each day and bit 58 a day later; NIST's frame of 4 July 2012 is `11`.
    #[test]
    fn the_summer_time_bits_follow_the_day_of_a_change() {
        let denver =
            hc_tz::PosixTimeZone::parse("America/Denver", "MST7MDT,M3.2.0,M11.1.0").expect("rules");
        let at = |seconds: i64| DstState::of_day(&denver, UnixTime::from_seconds(seconds));
        let march_8 = 1_772_928_000;
        let november_1 = 1_793_491_200;
        assert_eq!(at(march_8 - 60), DstState::Standard);
        assert_eq!(at(march_8), DstState::BeginsToday);
        assert_eq!(at(march_8 + 9 * 3_600 - 1), DstState::BeginsToday);
        assert_eq!(at(march_8 + 9 * 3_600), DstState::BeginsToday);
        assert_eq!(at(march_8 + 86_340), DstState::BeginsToday);
        assert_eq!(at(march_8 + 86_400), DstState::InEffect);
        assert_eq!(at(november_1 - 60), DstState::InEffect);
        assert_eq!(at(november_1), DstState::EndsToday);
        assert_eq!(at(november_1 + 8 * 3_600), DstState::EndsToday);
        assert_eq!(at(november_1 + 86_400), DstState::Standard);
        assert_eq!(at(1_341_423_000), DstState::InEffect);
        let phoenix = hc_tz::PosixTimeZone::parse("America/Phoenix", "MST7").expect("rules");
        assert_eq!(
            DstState::of_day(&phoenix, UnixTime::from_seconds(march_8)),
            DstState::Standard
        );
    }

    /// `dst_next` from the United States' rule in Denver, 2026: before
    /// 00:00 UTC on 8 March the word names that day, the second Sunday of
    /// March at 2 AM; from it, the first Sunday of November at 2 AM, row 37
    /// of Table 8, as in the document's frame of 4 July 2012; from 00:00
    /// UTC on 1 November, the second Sunday of March 2027, the 14th. Both
    /// words are `011011`, read one way and the other by `dst_on[1]`. Every
    /// day's word is one Table 8 lists for the day's state.
    #[test]
    fn dst_next_follows_the_next_change() {
        let denver =
            hc_tz::PosixTimeZone::parse("America/Denver", "MST7MDT,M3.2.0,M11.1.0").expect("rules");
        let at = |seconds: i64| DstNext::of_day(&denver, UnixTime::from_seconds(seconds));
        let march_8 = 1_772_928_000;
        let november_1 = 1_793_491_200;
        let into = DstNext::Transition(DstTransition {
            into_dst: true,
            weeks: 1,
            hour: 2,
        });
        let out = DstNext::Transition(DstTransition {
            into_dst: false,
            weeks: 0,
            hour: 2,
        });
        let january_15 = 1_768_435_200;
        assert_eq!(at(january_15), into);
        assert_eq!(at(march_8 - 60), into);
        assert_eq!(at(march_8), out);
        assert_eq!(at(march_8 + 9 * 3_600), out);
        assert_eq!(at(1_341_423_000), out);
        assert_eq!(at(november_1 - 60), out);
        assert_eq!(at(november_1), into);
        assert_eq!(into.word(), Some(0b011011));
        assert_eq!(out.word(), Some(0b011011));
        let DstNext::Transition(spring) = into else {
            unreachable!()
        };
        let DstNext::Transition(autumn) = out else {
            unreachable!()
        };
        assert_eq!(spring.day(2026), gregorian::to_fixed(2026, 3, 8));
        assert_eq!(autumn.day(2026), gregorian::to_fixed(2026, 11, 1));
        assert_eq!(spring.day(2027), gregorian::to_fixed(2027, 3, 14));
        for day in 0..365 {
            let minute = UnixTime::from_seconds(1_767_225_600 + day * 86_400);
            let next = DstNext::of_day(&denver, minute);
            let (dst_on, _) = DstState::of_day(&denver, minute).bits();
            let word = next.word().expect("a word");
            assert_eq!(DstNext::from_word(word, dst_on), Some(next), "day {day}");
        }
    }

    /// Other rules: Berlin's last Sundays of March at 2 AM and of October
    /// at 3 AM, 29 March and 25 October 2026, are schedules of Table 8;
    /// Sydney's first Sunday of April is not, and is word 49; Phoenix keeps
    /// no summer time, word 50.
    #[test]
    fn dst_next_of_other_rules() {
        let berlin = hc_tz::PosixTimeZone::parse("Europe/Berlin", "CET-1CEST,M3.5.0,M10.5.0/3")
            .expect("rules");
        let january_15 = UnixTime::from_seconds(1_768_435_200);
        let july_1 = UnixTime::from_seconds(1_782_864_000);
        let spring = DstNext::of_day(&berlin, january_15);
        assert_eq!(
            spring,
            DstNext::Transition(DstTransition {
                into_dst: true,
                weeks: 4,
                hour: 2,
            })
        );
        assert_eq!(spring.word(), Some(0b000010));
        let autumn = DstNext::of_day(&berlin, july_1);
        assert_eq!(
            autumn,
            DstNext::Transition(DstTransition {
                into_dst: false,
                weeks: -1,
                hour: 3,
            })
        );
        assert_eq!(autumn.word(), Some(0b010000));
        let DstNext::Transition(autumn) = autumn else {
            unreachable!()
        };
        assert_eq!(autumn.day(2026), gregorian::to_fixed(2026, 10, 25));
        let sydney =
            hc_tz::PosixTimeZone::parse("Australia/Sydney", "AEST-10AEDT,M10.1.0,M4.1.0/3")
                .expect("rules");
        assert_eq!(DstNext::of_day(&sydney, january_15), DstNext::DifferentTime);
        let phoenix = hc_tz::PosixTimeZone::parse("America/Phoenix", "MST7").expect("rules");
        assert_eq!(DstNext::of_day(&phoenix, january_15), DstNext::NoDst);
        assert_eq!(DstNext::NoDst.word(), Some(0b000111));
    }

    fn bits(text: &str) -> Vec<bool> {
        text.bytes()
            .filter(|byte| *byte != b' ')
            .map(|byte| byte == b'1')
            .collect()
    }

    /// SP 432's figure: 2001, day 258, 18:42 UTC, UT1 − UTC = −0.7 s. The
    /// figure's DST and leap bits are not legible in the text; they are
    /// taken here as 0.
    #[test]
    fn sp_432s_frame() {
        let reading = minute(2001, 9, 15, 18, 42);
        let frame = AmFrame::for_minute(reading, -7, DstState::Standard, false).expect("valid");
        assert_eq!((frame.day_of_year, frame.year), (258, 1));
        let symbols = frame.encode().expect("valid");
        assert_eq!(symbols.as_slice()[36..44], [O, I, O, M, O, I, I, I]);
        let decoded = AmFrame::decode(symbols.as_slice()).expect("valid");
        assert_eq!(decoded, frame);
        assert_eq!(decoded.reading(2000), Ok(reading));
        assert_eq!(decoded.dut1_tenths, -7);
    }

    /// The phase-code document's Table 10: 17:30 UTC on 4 July 2012, the
    /// amplitude bits and the phase bits of the same minute.
    #[test]
    fn the_2012_example_in_both_codes() {
        let amplitude: [Symbol; 60] = [
            M, O, I, I, O, O, O, O, O, M, // minute 3|0
            O, O, O, I, O, O, I, I, I, M, // hour 1|7
            O, O, O, I, O, I, O, O, O, M, // day 1|8
            O, I, I, O, O, O, I, O, I, M, // day 6, UT1 +
            O, I, O, O, O, O, O, O, I, M, // 0.4 s, year 1
            O, O, I, O, O, I, O, I, I, M, // year 2, LYI, LSW, DST 11
        ];
        let reading = minute(2012, 7, 4, 17, 30);
        let am = AmFrame::decode(&amplitude).expect("Table 10's amplitude bits");
        assert_eq!(
            am,
            AmFrame {
                minute: 30,
                hour: 17,
                day_of_year: 186,
                year: 12,
                dut1_tenths: 4,
                leap_year: true,
                leap_second_warning: false,
                dst: DstState::InEffect,
                seconds: 60,
            }
        );
        assert_eq!(am.reading(2000), Ok(reading));
        assert_eq!(
            AmFrame::for_minute(reading, 4, DstState::InEffect, false),
            Ok(am)
        );
        assert_eq!(am.encode().expect("valid").as_slice(), amplitude);

        // Table 10's phase row, seconds 0 to 59. Bits 29 and 39 are
        // reserved and "arbitrarily set to 0 and 1", and the notice bit is 1.
        let phase = bits("0011101101 0001001000 0011001000 0110001101 0011010001 0110110110");
        let pm = PmFrame::decode(&phase).expect("Table 10's phase bits");
        assert_eq!(pm.minute_of_century, 6_578_970);
        assert_eq!(pm.dst, DstState::InEffect);
        assert_eq!(pm.leap, LeapNotice::None);
        assert!(pm.notice);
        assert_eq!(pm.reserved, [false, true]);
        assert_eq!(
            pm.next,
            DstNext::Transition(DstTransition {
                into_dst: false,
                weeks: 0,
                hour: 2,
            })
        );
        assert_eq!(time_parity(6_578_970), 0b10010);
        assert_eq!(pm.reading(2000), Ok(reading));
        assert_eq!(pm.encode().expect("valid").as_slice(), phase);
        let built =
            PmFrame::for_minute(reading, 2000, DstState::InEffect, LeapNotice::None, pm.next)
                .expect("valid");
        assert_eq!(
            PmFrame {
                notice: true,
                reserved: [false, true],
                ..built
            },
            pm
        );
        // The first Sunday of November 2012 was the 4th.
        let DstNext::Transition(transition) = pm.next else {
            panic!("a schedule");
        };
        assert_eq!(transition.day(2012), gregorian::to_fixed(2012, 11, 4));
    }

    /// §4.3: at 21:30:00 UTC on 28 July 2016 the count goes from 8 717 609
    /// to 8 717 610.
    #[test]
    fn the_minute_count_of_28_july_2016() {
        let frame = PmFrame::for_minute(
            minute(2016, 7, 28, 21, 30),
            2000,
            DstState::InEffect,
            LeapNotice::None,
            DstNext::NoDst,
        )
        .expect("valid");
        assert_eq!(frame.minute_of_century, 8_717_610);
        let before = PmFrame::for_minute(
            minute(2016, 7, 28, 21, 29),
            2000,
            DstState::InEffect,
            LeapNotice::None,
            DstNext::NoDst,
        )
        .expect("valid");
        assert_eq!(before.minute_of_century, 8_717_609);
    }

    #[test]
    fn table_4_round_trips() {
        for (code, dst, leap) in TABLE_4 {
            let frame = PmFrame {
                minute_of_century: 0,
                dst,
                leap,
                notice: false,
                next: DstNext::NoDst,
                reserved: [false; 2],
                seconds: 60,
            };
            assert_eq!(frame.dst_word().map(|(word, _)| word), Ok(code));
            let decoded = PmFrame::decode(frame.encode().expect("valid").as_slice());
            assert_eq!(decoded, Ok(frame));
        }
        let codes: Vec<u8> = TABLE_4.iter().map(|&(code, _, _)| code).collect();
        for (index, code) in codes.iter().enumerate() {
            assert!(!codes[index + 1..].contains(code));
        }
    }

    /// Every word of Table 8 reads back to itself; the 24 words of each
    /// direction are distinct, and no message word is a schedule word.
    #[test]
    fn table_8_round_trips() {
        let mut meanings = 0;
        for dst_on in [false, true] {
            let mut words = Vec::new();
            for word in 0..64u8 {
                if let Some(meaning) = DstNext::from_word(word, dst_on) {
                    assert_eq!(meaning.word(), Some(word));
                    words.push(word);
                    meanings += 1;
                }
            }
            assert_eq!(words.len(), 32);
        }
        assert_eq!(meanings, 64);
        // Row 10 and row 37, the schedules of 2013 as §4.6 explains them.
        assert_eq!(
            DstNext::from_word(0b011011, false),
            Some(DstNext::Transition(DstTransition {
                into_dst: true,
                weeks: 1,
                hour: 2,
            }))
        );
        let second_sunday_of_march = DstTransition {
            into_dst: true,
            weeks: 1,
            hour: 2,
        };
        assert_eq!(
            second_sunday_of_march.day(2013),
            gregorian::to_fixed(2013, 3, 10)
        );
        assert_eq!(DstNext::Reserved(6).word(), None);
    }

    /// The leap second of 30 June 2012: the minute from 23:59:00 UTC has 61
    /// seconds, bit 59 repeated in both codes.
    #[test]
    fn leap_second_frames() {
        let reading = minute(2012, 6, 30, 23, 59);
        let am = AmFrame::for_minute(reading, 0, DstState::InEffect, true).expect("valid");
        assert_eq!(am.seconds, 61);
        let symbols = am.encode().expect("valid");
        assert_eq!(symbols.as_slice()[58..], [I, M, M]);
        assert_eq!(AmFrame::decode(symbols.as_slice()), Ok(am));
        assert_eq!(am.reading(2000), Ok(reading));
        let pm = PmFrame::for_minute(
            reading,
            2000,
            DstState::InEffect,
            LeapNotice::Positive,
            DstNext::NoDst,
        )
        .expect("valid");
        assert_eq!(pm.seconds, 61);
        let phase = pm.encode().expect("valid");
        assert_eq!(phase.as_slice()[59..], [false, false]);
        assert_eq!(PmFrame::decode(phase.as_slice()), Ok(pm));
        assert_eq!(pm.reading(2000), Ok(reading));
        // A negative one removes bit 59.
        let negative = PmFrame {
            leap: LeapNotice::Negative,
            seconds: 59,
            ..pm
        };
        let phase = negative.encode().expect("valid");
        assert_eq!(phase.len(), 59);
        assert_eq!(PmFrame::decode(phase.as_slice()), Ok(negative));
        // Not at the end of a month, a long frame is refused.
        let early = AmFrame::for_minute(minute(2012, 6, 29, 23, 59), 0, DstState::InEffect, true)
            .expect("valid");
        assert_eq!(early.seconds, 60);
        let wrong = AmFrame {
            seconds: 61,
            ..early
        };
        assert_eq!(wrong.reading(2000), Err(FrameError::Field("leap second")));
    }

    #[test]
    fn broken_frames_are_refused() {
        let good = PmFrame::for_minute(
            minute(2012, 7, 4, 17, 30),
            2000,
            DstState::InEffect,
            LeapNotice::None,
            DstNext::NoDst,
        )
        .expect("valid")
        .encode()
        .expect("valid");
        let mut bad = good.as_slice().to_vec();
        bad[46] = !bad[46];
        assert_eq!(PmFrame::decode(&bad), Err(FrameError::Symbol(19)));
        let mut bad = good.as_slice().to_vec();
        bad[20] = !bad[20];
        assert!(matches!(PmFrame::decode(&bad), Err(FrameError::Parity(_))));
        let mut bad = good.as_slice().to_vec();
        for (index, second) in SYNC.into_iter().enumerate() {
            bad[second] = SYNC_M >> (12 - index) & 1 == 1;
        }
        assert_eq!(
            PmFrame::decode(&bad),
            Err(FrameError::Field("message frame"))
        );
        let mut bad = good.as_slice().to_vec();
        bad[47] = true;
        bad[48] = true;
        bad[50] = false;
        assert_eq!(PmFrame::decode(&bad), Err(FrameError::Field("dst_ls")));
        let am = AmFrame::for_minute(minute(2012, 7, 4, 17, 30), 4, DstState::InEffect, false)
            .expect("valid")
            .encode()
            .expect("valid");
        let mut bad = am.as_slice().to_vec();
        bad[37] = I;
        assert_eq!(AmFrame::decode(&bad), Err(FrameError::Field("UT1 sign")));
        let mut bad = am.as_slice().to_vec();
        bad[44] = I;
        assert_eq!(AmFrame::decode(&bad), Err(FrameError::Symbol(44)));
        let mut bad = am.as_slice().to_vec();
        bad[29] = O;
        assert_eq!(AmFrame::decode(&bad), Err(FrameError::Symbol(29)));
        // The leap year bit in 2013.
        let frame = AmFrame::decode(am.as_slice()).expect("valid");
        let wrong = AmFrame { year: 13, ..frame };
        assert_eq!(wrong.reading(2000), Err(FrameError::Field("leap year")));
    }

    /// The field refusals of both codes: a reading that is not the start of
    /// a minute, a UT1 − UTC past ±0.9 s, a minute count outside the
    /// century or its 26 bits, a `dst_next` word or schedule the state
    /// cannot carry, and a 1 at bit 59 or 60.
    #[test]
    fn field_refusals() {
        let reading = minute(2012, 7, 4, 17, 30);
        let mut late = reading;
        late.time = CivilTime::hms(17, 30, 1).expect("valid");
        assert_eq!(
            AmFrame::for_minute(late, 0, DstState::InEffect, false),
            Err(FrameError::Field("the start of a minute"))
        );
        assert_eq!(
            PmFrame::for_minute(
                late,
                2000,
                DstState::InEffect,
                LeapNotice::None,
                DstNext::NoDst
            ),
            Err(FrameError::Field("the start of a minute"))
        );
        for dut1_tenths in [-10, 10] {
            assert_eq!(
                AmFrame::for_minute(reading, dut1_tenths, DstState::InEffect, false),
                Err(FrameError::Field("UT1 correction"))
            );
        }
        let am = AmFrame::for_minute(reading, 4, DstState::InEffect, false).expect("valid");
        let wrong = AmFrame {
            dut1_tenths: 10,
            ..am
        };
        assert_eq!(wrong.encode(), Err(FrameError::Field("date and time")));

        let pm = |century: i64, dst: DstState, next: DstNext| {
            PmFrame::for_minute(reading, century, dst, LeapNotice::None, next)
        };
        assert_eq!(
            pm(1900, DstState::InEffect, DstNext::NoDst),
            Err(FrameError::Field("minute count"))
        );
        assert_eq!(
            pm(1950, DstState::InEffect, DstNext::NoDst),
            Err(FrameError::Field("century"))
        );
        // Into summer time while it is in effect, and a reserved word
        // Table 8 does not have.
        let into = DstNext::Transition(DstTransition {
            into_dst: true,
            weeks: 1,
            hour: 2,
        });
        assert_eq!(
            pm(2000, DstState::InEffect, into),
            Err(FrameError::Field("dst_next"))
        );
        assert_eq!(
            pm(2000, DstState::InEffect, DstNext::Reserved(6)),
            Err(FrameError::Field("dst_next"))
        );
        let frame = pm(2000, DstState::InEffect, DstNext::NoDst).expect("valid");
        let wide = PmFrame {
            minute_of_century: 1 << 26,
            ..frame
        };
        assert_eq!(wide.encode(), Err(FrameError::Field("minute count")));
        // 2100-01-01T00:00, one minute past the century's count.
        let past = PmFrame {
            minute_of_century: 36_525 * 1_440,
            ..frame
        };
        assert_eq!(past.reading(2000), Err(FrameError::Field("minute count")));
        let good = frame.encode().expect("valid");
        let unknown = (0..64u8)
            .find(|&word| DstNext::from_word(word, true).is_none())
            .expect("Table 8 leaves words unused");
        let mut bad = good.as_slice().to_vec();
        for (index, &second) in DST_NEXT.iter().enumerate() {
            bad[second] = unknown >> (5 - index) & 1 == 1;
        }
        assert_eq!(PmFrame::decode(&bad), Err(FrameError::Field("dst_next")));
        let mut bad = good.as_slice().to_vec();
        bad[59] = true;
        assert_eq!(PmFrame::decode(&bad), Err(FrameError::Symbol(59)));
        let mut bad = good.as_slice().to_vec();
        bad.push(true);
        assert_eq!(PmFrame::decode(&bad), Err(FrameError::Symbol(60)));
    }

    /// Three minutes of every day of 2000–2099, both codes, both ways:
    /// every day in a release build; every 83rd and every month's first
    /// and last in a debug one.
    #[test]
    fn every_code_round_trips() {
        for day in crate::radio::sweep_days_2000_to_2099(83) {
            for (hour, minute_of_hour) in [(0, 0), (6, 30), (23, 59)] {
                let reading = CivilDateTime::new(
                    Rd(day),
                    CivilTime::hms(hour, minute_of_hour, 0).expect("valid"),
                );
                let am =
                    AmFrame::for_minute(reading, -3, DstState::EndsToday, false).expect("valid");
                let back = AmFrame::decode(am.encode().expect("valid").as_slice()).expect("valid");
                assert_eq!(back, am);
                assert_eq!(back.reading(2000), Ok(reading));
                let pm = PmFrame::for_minute(
                    reading,
                    2000,
                    DstState::BeginsToday,
                    LeapNotice::None,
                    DstNext::Transition(DstTransition {
                        into_dst: false,
                        weeks: -2,
                        hour: 3,
                    }),
                )
                .expect("valid");
                let back = PmFrame::decode(pm.encode().expect("valid").as_slice()).expect("valid");
                assert_eq!(back, pm);
                assert_eq!(back.reading(2000), Ok(reading));
            }
        }
    }
}
