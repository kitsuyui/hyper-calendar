//! JJY, the time code of NICT's long-wave stations: JST, the minute at the
//! frame's first marker.
//!
//! NICT, "標準電波の出し方" and its two figures, read 2026-09-27
//! (`nict-jjy-timecode`). Symbols of 0.2 s are markers, of 0.5 s ones and
//! of 0.8 s zeros. "１周期の先頭マーカー（Ｍ）の時刻（年、通算日、時、分）を符号化して
//! 送信します": the frame carries the year, day of the year, hour and minute of
//! the time at its first marker, M, at second 0; the position markers P1–P5
//! and P0 are at seconds 9, 19, 29, 39, 49 and 59. The hour is JST, "協定世界時
//! (ＵＴＣ)を９時間進めたもの", UTC nine hours ahead.
//!
//! | Seconds | Field |
//! | --- | --- |
//! | 1–3, 5–8 | minute, BCD 40 20 10, 8 4 2 1 |
//! | 12–13, 15–18 | hour, 20 10, 8 4 2 1 |
//! | 22–23, 25–28, 30–33 | day of the year, 1 January = 1 |
//! | 36, 37 | PA1, PA2: "偶数パリティ", even parity of the hour and of the minute |
//! | 38, 40 | SU1, SU2, spare, "ともに\"0\"を割り当てます" until defined |
//! | 41–48 | the year's last two digits |
//! | 50–52 | weekday, Sunday 0 to Saturday 6 |
//! | 53, 54 | LS1 LS2: 00 no leap second within a month, 11 a positive one, 10 a negative one |
//!
//! At 15 and 45 minutes past the hour the call sign fills 40–48, 50–52 are
//! ST1–ST3, when a planned stop begins, and 53–55 ST4–ST6, whether it is by
//! day only and how long it lasts; there is no year, weekday or leap-second
//! information in that frame. Every other second of the frame is 0.
//!
//! A leap second falls just before 09:00 JST on the first of a month,
//! "月始め（１日）の９時０分（日本標準時）の直前". In a positive one P0 moves to second
//! 60 and second 59 is a 0; in a negative one P0 moves to second 58. LS1 LS2
//! are shown from 09:00 on the 2nd of the month before until 08:59 on the
//! 1st.

use hc_calendar::{CivilDateTime, Weekday};
use hc_core::UnixTime;

use super::{
    Frame, FrameError, FrameResult, LeapNotice, Symbol, check_length, day_of_year, minute_reading,
    odd_ones, read_bcd, read_bits, split_year, unix_of, write_bcd, year_in_century, ymd,
};

const MINUTE: &[&[usize]] = &[&[1, 2, 3], &[5, 6, 7, 8]];
const HOUR: &[&[usize]] = &[&[12, 13], &[15, 16, 17, 18]];
const DAY: &[&[usize]] = &[&[22, 23], &[25, 26, 27, 28], &[30, 31, 32, 33]];
const YEAR: &[&[usize]] = &[&[41, 42, 43, 44], &[45, 46, 47, 48]];
const WEEKDAY: &[usize] = &[50, 51, 52];
const STOP_START: &[usize] = &[50, 51, 52];
const STOP_SPAN: &[usize] = &[54, 55];
const MINUTE_BITS: [usize; 7] = [1, 2, 3, 5, 6, 7, 8];
const HOUR_BITS: [usize; 6] = [12, 13, 15, 16, 17, 18];
const PA1: usize = 36;
const PA2: usize = 37;
const SU1: usize = 38;
const SU2: usize = 40;
const LS1: usize = 53;
const LS2: usize = 54;
const ST4: usize = 53;
/// Seconds that are 0 in every frame.
const ZEROS: [usize; 9] = [4, 10, 11, 14, 20, 21, 24, 34, 35];
/// Seconds that are 0 in an ordinary frame, where the frame has them.
const STANDARD_ZEROS: [usize; 4] = [55, 56, 57, 58];
/// Seconds that are 0 in a call-sign frame.
const CALL_SIGN_ZEROS: [usize; 4] = [38, 56, 57, 58];
/// The call sign, a Morse signal and not bits.
const CALL_SIGN: core::ops::RangeInclusive<usize> = 40..=48;

/// What the second half of a frame carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JjyContent {
    /// Every minute but 15 and 45.
    Standard {
        /// The year's last two digits.
        year: u8,
        /// The weekday.
        weekday: Weekday,
        /// LS1 and LS2.
        leap: LeapNotice,
        /// SU1 and SU2, 0 until NICT gives them a use.
        spare: [bool; 2],
    },
    /// Minutes 15 and 45, with the call sign.
    CallSign {
        /// ST1–ST3: 0 no stop planned, 1 within 7 days, 2 within 3–6 days,
        /// 3 within 2 days, 4 within 24 hours, 5 within 12 hours, 6 within
        /// 2 hours.
        stop_start: u8,
        /// ST4: the stop is by day only.
        daytime_only: bool,
        /// ST5–ST6: 0 no stop planned, 1 seven days or more, or not known,
        /// 2 two to six days, 3 less than two days.
        stop_span: u8,
    },
}

/// A decoded JJY frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JjyFrame {
    /// The minute of JST at the frame's first marker.
    pub minute: u8,
    /// The hour of JST.
    pub hour: u8,
    /// The day of the year, 1 January being 1.
    pub day_of_year: u16,
    /// The year, weekday and leap second, or the call sign's stop notice.
    pub content: JjyContent,
    /// The frame's seconds: 60, or 61 or 59 for the minute of a leap
    /// second.
    pub seconds: u8,
}

/// The second of P0 in a frame of `seconds` seconds.
const fn p0(seconds: u8) -> usize {
    match seconds {
        59 => 58,
        61 => 60,
        _ => 59,
    }
}

impl JjyFrame {
    /// The frame of a JST minute: the call-sign frame with no stop planned
    /// at 15 and 45 minutes past the hour, and an ordinary one otherwise,
    /// 61 or 59 seconds long when `leap` is announced and the minute is
    /// 08:59 on the first of a month.
    ///
    /// # Errors
    ///
    /// [`FrameError::Field`] for a reading that is not the start of a
    /// minute.
    pub fn for_minute(reading: CivilDateTime, leap: LeapNotice) -> FrameResult<Self> {
        let time = reading.time;
        if time.second() != 0 || time.subsec_attos() != 0 {
            return Err(FrameError::Field("the start of a minute"));
        }
        let (year, month, day) = ymd(reading)?;
        let day_of_year = gregorian_day_of_year(year, month, day)?;
        let content = if time.minute() % 30 == 15 {
            JjyContent::CallSign {
                stop_start: 0,
                daytime_only: false,
                stop_span: 0,
            }
        } else {
            JjyContent::Standard {
                year: split_year(year).1,
                weekday: Weekday::from_rd(reading.day),
                leap,
                spare: [false; 2],
            }
        };
        let at_the_leap = day == 1 && (time.hour(), time.minute()) == (8, 59);
        let seconds = match leap {
            LeapNotice::Positive if at_the_leap => 61,
            LeapNotice::Negative if at_the_leap => 59,
            _ => 60,
        };
        Ok(Self {
            minute: time.minute(),
            hour: time.hour(),
            day_of_year,
            content,
            seconds,
        })
    }

    /// Decode a frame of 59, 60 or 61 symbols.
    ///
    /// # Errors
    ///
    /// [`FrameError::Length`], [`FrameError::Symbol`] for a marker out of
    /// place or a 1 where the code fixes a 0, [`FrameError::Digit`],
    /// [`FrameError::Parity`], and [`FrameError::Field`] for a weekday of 7,
    /// an LS1 LS2 of `01`, an ST1–ST3 of `111`, or an hour or minute out of
    /// range.
    pub fn decode(symbols: &[Symbol]) -> FrameResult<Self> {
        let seconds = check_length(symbols.len())?;
        let p0 = p0(seconds);
        // The call sign's seconds are checked below, once the minute says
        // whether they are bits.
        for (second, &symbol) in symbols.iter().enumerate() {
            let marker = matches!(second, 0 | 9 | 19 | 29 | 39 | 49) || second == p0;
            if (symbol == Symbol::Marker) != marker && !CALL_SIGN.contains(&second) {
                return Err(FrameError::Symbol(second));
            }
        }
        let bit = |second: usize| match symbols.get(second) {
            Some(Symbol::One) => Ok(true),
            Some(Symbol::Zero) => Ok(false),
            _ => Err(FrameError::Symbol(second)),
        };
        let minute = read_bcd(&bit, MINUTE)? as u8;
        let hour = read_bcd(&bit, HOUR)? as u8;
        if minute > 59 || hour > 23 {
            return Err(FrameError::Field("time of day"));
        }
        let day_of_year = read_bcd(&bit, DAY)? as u16;
        if bit(PA1)? != odd_ones(&bit, HOUR_BITS)? {
            return Err(FrameError::Parity(PA1));
        }
        if bit(PA2)? != odd_ones(&bit, MINUTE_BITS)? {
            return Err(FrameError::Parity(PA2));
        }
        let call_sign = minute % 30 == 15;
        let zeros = if call_sign {
            CALL_SIGN_ZEROS
        } else {
            STANDARD_ZEROS
        };
        for second in ZEROS.into_iter().chain(zeros) {
            if second < p0 && bit(second)? {
                return Err(FrameError::Symbol(second));
            }
        }
        if seconds == 61 && bit(59)? {
            return Err(FrameError::Symbol(59));
        }
        // An ordinary frame's 40–48 are SU2 and the year, read as bits
        // below; a call-sign frame's are Morse, and not read.
        let content = if call_sign {
            let stop_start = read_bits(bit, STOP_START)? as u8;
            if stop_start == 7 {
                return Err(FrameError::Field("stop notice"));
            }
            JjyContent::CallSign {
                stop_start,
                daytime_only: bit(ST4)?,
                stop_span: read_bits(bit, STOP_SPAN)? as u8,
            }
        } else {
            let weekday = match read_bits(bit, WEEKDAY)? {
                0 => Weekday::Sunday,
                number @ 1..=6 => {
                    Weekday::from_iso_number(number as u8).ok_or(FrameError::Field("weekday"))?
                }
                _ => return Err(FrameError::Field("weekday")),
            };
            let leap = match (bit(LS1)?, bit(LS2)?) {
                (false, false) => LeapNotice::None,
                (true, true) => LeapNotice::Positive,
                (true, false) => LeapNotice::Negative,
                (false, true) => return Err(FrameError::Field("leap second notice")),
            };
            JjyContent::Standard {
                year: read_bcd(&bit, YEAR)? as u8,
                weekday,
                leap,
                spare: [bit(SU1)?, bit(SU2)?],
            }
        };
        Ok(Self {
            minute,
            hour,
            day_of_year,
            content,
            seconds,
        })
    }

    /// The frame's symbols.
    ///
    /// # Errors
    ///
    /// [`FrameError::Field`] for a field out of its range, or a length of
    /// 61 or 59 without the leap-second notice of its sign or away from
    /// 08:59; [`FrameError::Length`] for any other length.
    pub fn encode(&self) -> FrameResult<Frame<Symbol>> {
        let seconds = check_length(usize::from(self.seconds))?;
        if self.minute > 59 || self.hour > 23 || self.day_of_year == 0 || self.day_of_year > 366 {
            return Err(FrameError::Field("time of day"));
        }
        let call_sign = self.minute % 30 == 15;
        let mut frame = Frame::filled(usize::from(seconds));
        let p0 = p0(seconds);
        for second in [0, 9, 19, 29, 39, 49, p0] {
            frame.set(second, Symbol::Marker);
        }
        let mut set = |second: usize, value: bool| frame.set(second, Symbol::bit(value));
        write_bcd(&mut set, MINUTE, u32::from(self.minute));
        write_bcd(&mut set, HOUR, u32::from(self.hour));
        write_bcd(&mut set, DAY, u32::from(self.day_of_year));
        match (self.content, call_sign) {
            (
                JjyContent::Standard {
                    year,
                    weekday,
                    leap,
                    spare,
                },
                false,
            ) => {
                let expected = match seconds {
                    61 => LeapNotice::Positive,
                    59 => LeapNotice::Negative,
                    _ => leap,
                };
                if year > 99
                    || leap != expected
                    || seconds != 60 && (self.hour, self.minute) != (8, 59)
                {
                    return Err(FrameError::Field("leap second"));
                }
                write_bcd(&mut set, YEAR, u32::from(year));
                let number = u32::from(weekday.sunday_first_number());
                for (index, &second) in WEEKDAY.iter().rev().enumerate() {
                    set(second, number >> index & 1 == 1);
                }
                let (ls1, ls2) = match leap {
                    LeapNotice::None => (false, false),
                    LeapNotice::Positive => (true, true),
                    LeapNotice::Negative => (true, false),
                };
                set(LS1, ls1);
                set(LS2, ls2);
                set(SU1, spare[0]);
                set(SU2, spare[1]);
            }
            (
                JjyContent::CallSign {
                    stop_start,
                    daytime_only,
                    stop_span,
                },
                true,
            ) => {
                if stop_start > 6 || stop_span > 3 || seconds != 60 {
                    return Err(FrameError::Field("stop notice"));
                }
                for (index, &second) in STOP_START.iter().rev().enumerate() {
                    set(second, stop_start >> index & 1 == 1);
                }
                set(ST4, daytime_only);
                for (index, &second) in STOP_SPAN.iter().rev().enumerate() {
                    set(second, stop_span >> index & 1 == 1);
                }
            }
            _ => return Err(FrameError::Field("call sign")),
        }
        let pa1 = HOUR_BITS
            .iter()
            .filter(|&&second| frame.symbols[second] == Symbol::One)
            .count();
        let pa2 = MINUTE_BITS
            .iter()
            .filter(|&&second| frame.symbols[second] == Symbol::One)
            .count();
        frame.set(PA1, Symbol::bit(pa1 % 2 == 1));
        frame.set(PA2, Symbol::bit(pa2 % 2 == 1));
        Ok(frame)
    }

    /// The JST minute the frame names, its two-digit year read in the
    /// century beginning `century`, as 2000.
    ///
    /// # Errors
    ///
    /// [`FrameError::Field`] for a call-sign frame, which has no year (use
    /// [`JjyFrame::reading_in_year`]), a century that is not a multiple of
    /// 100, a day the year does not have, a weekday that is not the
    /// date's, or a frame of 61 or 59 seconds anywhere but 08:59 on the
    /// first of a month.
    pub fn reading(&self, century: i64) -> FrameResult<CivilDateTime> {
        match self.content {
            JjyContent::Standard { year, .. } => {
                self.reading_in_year(year_in_century(century, year)?)
            }
            JjyContent::CallSign { .. } => Err(FrameError::Field("year")),
        }
    }

    /// The JST minute the frame names in `year`, which a call-sign frame
    /// needs and an ordinary frame's two digits must agree with.
    ///
    /// # Errors
    ///
    /// As [`JjyFrame::reading`], and [`FrameError::Field`] for a year whose
    /// last two digits are not the frame's.
    pub fn reading_in_year(&self, year: i64) -> FrameResult<CivilDateTime> {
        let day = day_of_year(year, self.day_of_year)?;
        let reading = minute_reading(day, self.hour, self.minute)?;
        if let JjyContent::Standard {
            year: two_digits,
            weekday,
            ..
        } = self.content
        {
            if split_year(year).1 != two_digits {
                return Err(FrameError::Field("year"));
            }
            if Weekday::from_rd(day) != weekday {
                return Err(FrameError::Field("weekday"));
            }
        }
        let first_of_month = matches!(ymd(reading)?, (_, _, 1));
        if self.seconds != 60 && !(first_of_month && (self.hour, self.minute) == (8, 59)) {
            return Err(FrameError::Field("leap second"));
        }
        Ok(reading)
    }

    /// The POSIX time of the frame's first marker: JST less nine hours.
    ///
    /// # Errors
    ///
    /// As [`JjyFrame::reading`].
    pub fn to_unix(&self, century: i64) -> FrameResult<UnixTime> {
        unix_of(self.reading(century)?, 9)
    }
}

fn gregorian_day_of_year(year: i64, month: u8, day: u8) -> FrameResult<u16> {
    hc_calendar::gregorian::day_of_year(year, month, day).map_err(|_| FrameError::Field("date"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::{CivilTime, gregorian};

    const M: Symbol = Symbol::Marker;
    const O: Symbol = Symbol::Zero;
    const I: Symbol = Symbol::One;

    fn minute(year: i64, month: u8, day: u8, hour: u8, minute: u8) -> CivilDateTime {
        CivilDateTime::new(
            gregorian::to_fixed(year, month, day).expect("exists"),
            CivilTime::hms(hour, minute, 0).expect("valid"),
        )
    }

    /// NICT's first figure, symbol by symbol: 2004, day 92 (1 April),
    /// 17:25 JST, Thursday, no leap second within a month.
    const NICT_EXAMPLE: [Symbol; 60] = [
        M, O, I, O, O, O, I, O, I, M, // 0–9: minute 2|5
        O, O, O, I, O, O, I, I, I, M, // 10–19: hour 1|7
        O, O, O, O, O, I, O, O, I, M, // 20–29: day 0|9
        O, O, I, O, O, O, O, I, O, M, // 30–39: day 2, PA1 0, PA2 1, SU1 0
        O, O, O, O, O, O, I, O, O, M, // 40–49: SU2 0, year 0|4
        I, O, O, O, O, O, O, O, O, M, // 50–59: weekday 4, LS 00
    ];

    #[test]
    fn nicts_example_frame() {
        let frame = JjyFrame::decode(&NICT_EXAMPLE).expect("NICT's frame");
        assert_eq!((frame.hour, frame.minute, frame.day_of_year), (17, 25, 92));
        assert_eq!(
            frame.content,
            JjyContent::Standard {
                year: 4,
                weekday: Weekday::Thursday,
                leap: LeapNotice::None,
                spare: [false; 2],
            }
        );
        let reading = minute(2004, 4, 1, 17, 25);
        assert_eq!(frame.reading(2000), Ok(reading));
        // 17:25 JST is 08:25 UTC.
        let unix = frame.to_unix(2000).expect("valid");
        assert_eq!(unix.seconds(), 12_509 * 86_400 + 8 * 3_600 + 25 * 60);
        assert_eq!(frame.encode().expect("valid").as_slice(), NICT_EXAMPLE);
        assert_eq!(JjyFrame::for_minute(reading, LeapNotice::None), Ok(frame));
    }

    /// NICT's second figure is 17:15 on the same day, the call-sign frame,
    /// with no stop planned. It has no year, so it is read in one.
    #[test]
    fn a_call_sign_frame_needs_the_year() {
        let reading = minute(2004, 4, 1, 17, 15);
        let mut frame = JjyFrame::for_minute(reading, LeapNotice::None).expect("valid");
        assert!(matches!(frame.content, JjyContent::CallSign { .. }));
        let mut symbols = frame.encode().expect("valid");
        // The call sign's Morse, whatever it looks like as bits, is not read.
        for second in 40..=48 {
            symbols.set(second, if second % 2 == 0 { I } else { O });
        }
        let decoded = JjyFrame::decode(symbols.as_slice()).expect("valid");
        assert_eq!(decoded, frame);
        assert_eq!(decoded.reading(2000), Err(FrameError::Field("year")));
        assert_eq!(decoded.reading_in_year(2004), Ok(reading));
        // A stop within 24 hours, by day, for two to six days.
        frame.content = JjyContent::CallSign {
            stop_start: 4,
            daytime_only: true,
            stop_span: 2,
        };
        let symbols = frame.encode().expect("valid");
        assert_eq!(symbols.as_slice()[50..56], [I, O, O, I, I, O]);
        assert_eq!(JjyFrame::decode(symbols.as_slice()), Ok(frame));
    }

    /// The leap second of 1 January 2017 came before 09:00 JST: the frame of
    /// 08:59 has 61 seconds, a 0 at 59 and P0 at 60. A negative one would
    /// put P0 at 58.
    #[test]
    fn jjy_leap_second_frames() {
        let reading = minute(2017, 1, 1, 8, 59);
        let frame = JjyFrame::for_minute(reading, LeapNotice::Positive).expect("valid");
        assert_eq!(frame.seconds, 61);
        let symbols = frame.encode().expect("valid");
        assert_eq!(symbols.as_slice()[58..], [O, O, M]);
        assert_eq!(JjyFrame::decode(symbols.as_slice()), Ok(frame));
        assert_eq!(frame.reading(2000), Ok(reading));
        let negative = JjyFrame::for_minute(reading, LeapNotice::Negative).expect("valid");
        let symbols = negative.encode().expect("valid");
        assert_eq!(symbols.len(), 59);
        assert_eq!(symbols.as_slice()[53..], [I, O, O, O, O, M]);
        assert_eq!(JjyFrame::decode(symbols.as_slice()), Ok(negative));
        // The notice alone does not lengthen another minute, and a long
        // frame elsewhere is refused.
        let before =
            JjyFrame::for_minute(minute(2016, 12, 31, 8, 59), LeapNotice::Positive).expect("valid");
        assert_eq!(before.seconds, 60);
        let wrong = JjyFrame {
            seconds: 61,
            ..before
        };
        // The frame alone cannot tell the 31st from the 1st; its date can.
        assert!(wrong.encode().is_ok());
        assert_eq!(wrong.reading(2000), Err(FrameError::Field("leap second")));
        let wrong = JjyFrame {
            day_of_year: 2,
            ..frame
        };
        assert_eq!(wrong.reading(2000), Err(FrameError::Field("weekday")));
    }

    #[test]
    fn broken_frames_are_refused() {
        let mut symbols = NICT_EXAMPLE;
        symbols[9] = O;
        assert_eq!(JjyFrame::decode(&symbols), Err(FrameError::Symbol(9)));
        let mut symbols = NICT_EXAMPLE;
        symbols[37] = O;
        assert_eq!(JjyFrame::decode(&symbols), Err(FrameError::Parity(37)));
        let mut symbols = NICT_EXAMPLE;
        symbols[4] = I;
        // Bit 4 is 0 in every frame; as part of no digit it is caught
        // after the parity, as a symbol.
        assert_eq!(JjyFrame::decode(&symbols), Err(FrameError::Symbol(4)));
        let mut symbols = NICT_EXAMPLE;
        symbols[25] = I;
        symbols[26] = I;
        assert_eq!(JjyFrame::decode(&symbols), Err(FrameError::Digit(25)));
        let mut symbols = NICT_EXAMPLE;
        symbols[54] = I;
        assert_eq!(
            JjyFrame::decode(&symbols),
            Err(FrameError::Field("leap second notice"))
        );
        assert_eq!(
            JjyFrame::decode(&NICT_EXAMPLE[..58]),
            Err(FrameError::Length(58))
        );
        let frame = JjyFrame::decode(&NICT_EXAMPLE).expect("valid");
        assert_eq!(frame.reading(2001), Err(FrameError::Field("century")));
        // Day 92 of 2104 is not a Thursday.
        assert_eq!(frame.reading(2100), Err(FrameError::Field("weekday")));
    }

    /// A reading that is not the start of a minute, a stop notice the code
    /// does not define, and content that does not match the minute are
    /// refused.
    #[test]
    fn field_refusals() {
        let mut reading = minute(2004, 4, 1, 17, 25);
        reading.time = CivilTime::hms(17, 25, 1).expect("valid");
        assert_eq!(
            JjyFrame::for_minute(reading, LeapNotice::None),
            Err(FrameError::Field("the start of a minute"))
        );
        reading.time = CivilTime::new(17, 25, 0, 1).expect("valid");
        assert_eq!(
            JjyFrame::for_minute(reading, LeapNotice::None),
            Err(FrameError::Field("the start of a minute"))
        );
        let call_sign =
            JjyFrame::for_minute(minute(2004, 4, 1, 17, 15), LeapNotice::None).expect("valid");
        // ST1–ST3 of 111 is no notice NICT defines; ST1–ST3 above 6 and
        // ST5–ST6 above 3 cannot be written.
        let mut symbols = call_sign.encode().expect("valid");
        for &second in STOP_START {
            symbols.set(second, I);
        }
        assert_eq!(
            JjyFrame::decode(symbols.as_slice()),
            Err(FrameError::Field("stop notice"))
        );
        for (stop_start, stop_span) in [(7, 0), (0, 4)] {
            let frame = JjyFrame {
                content: JjyContent::CallSign {
                    stop_start,
                    daytime_only: false,
                    stop_span,
                },
                ..call_sign
            };
            assert_eq!(frame.encode(), Err(FrameError::Field("stop notice")));
        }
        // The call sign at 17:25, and a year and weekday at 17:15.
        let standard = JjyFrame::decode(&NICT_EXAMPLE).expect("valid");
        let misplaced = JjyFrame {
            content: call_sign.content,
            ..standard
        };
        assert_eq!(misplaced.encode(), Err(FrameError::Field("call sign")));
        let misplaced = JjyFrame {
            content: standard.content,
            ..call_sign
        };
        assert_eq!(misplaced.encode(), Err(FrameError::Field("call sign")));
    }

    /// Five minutes of every day of 2000–2099, both ways: every day in a
    /// release build; every 97th and every month's first and last in a
    /// debug one.
    #[test]
    fn every_code_round_trips() {
        for day in crate::radio::sweep_days_2000_to_2099(97) {
            for (hour, minute_of_hour) in [(0, 0), (8, 59), (9, 15), (12, 45), (23, 59)] {
                let reading = CivilDateTime::new(
                    hc_calendar::Rd(day),
                    CivilTime::hms(hour, minute_of_hour, 0).expect("valid"),
                );
                let frame = JjyFrame::for_minute(reading, LeapNotice::None).expect("valid");
                let symbols = frame.encode().expect("valid");
                let back = JjyFrame::decode(symbols.as_slice()).expect("valid");
                assert_eq!(back, frame);
                let (year, _, _) = gregorian::from_fixed(reading.day).expect("valid");
                assert_eq!(back.reading_in_year(year), Ok(reading));
            }
        }
    }
}
