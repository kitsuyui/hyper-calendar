//! The IRIG serial time codes A, B, D, E, G and H: one frame decoded to
//! the time of year it names, and encoded from one.
//!
//! Range Commanders Council, *IRIG Serial Time Code Formats*, IRIG
//! Standard 200-16, August 2016, read 2026-09-28 (`rcc-200-16`). A frame
//! is a run of pulses, one per index count: a binary 0 and an index marker
//! last 0.2 of the index count, a binary 1 0.5, and a position identifier
//! and the reference bit Pr 0.8 (the bits §3.6, the index markers §3.9, Pr
//! §3.4 and the position identifiers §3.5). That is [`Symbol`]'s three
//! symbols, so a frame is a slice of them, Pr first.
//!
//! | Format | Index count | Frame | Time of year in BCD | Year | Control bits | SBS |
//! | --- | --- | --- | --- | --- | --- | --- |
//! | A | 1 ms | 0.1 s | days, hours, minutes, seconds, tenths | yes | 18 | yes |
//! | B | 10 ms | 1 s | days, hours, minutes, seconds | yes | 18 | yes |
//! | D | 1 min | 1 h | days, hours | no | 9 | no |
//! | E | 0.1 s | 10 s | days, hours, minutes, tens of seconds | yes | 18 | no |
//! | G | 0.1 ms | 10 ms | days, hours, minutes, seconds, tenths, hundredths | yes | 27 | no |
//! | H | 1 s | 1 min | days, hours, minutes | no | 9 | no |
//!
//! Pr is index count 0 and names the time of the frame; the position
//! identifiers P1, P2 … follow at counts 9, 19 …, and P0, the last count of
//! the frame, comes just before the next Pr (Tables 3-1 to 3-3). The BCD
//! digits are sent least significant bit first, the straight binary
//! seconds (SBS) of the day least significant bit first from count 80, and
//! every count not assigned to a field is an index marker. The year is its
//! last two digits and the control bits have no standard meaning: "the
//! assignment of control bits (CFs) to specific functions … is left to the
//! end user" (§4.1).
//!
//! Which fields a frame carries is its *coded expression*, the last digit
//! of a signal designation such as B122 (Figure 4-1, Table 4-1), and a
//! frame cannot be read without it: a field it leaves out is sent as index
//! markers, which look like zeros. [`IrigCode`] is the format and the
//! expression. Table 4-1 allows format E no SBS, although §3.7 and Table
//! 5-9 lay SBS out for it; this module follows Table 4-1, which says "no
//! other combinations are standard".
//!
//! The code carries no time scale. The standard says the ranges keep "UTC
//! referenced to the United States Naval Observatory (USNO) Master Clock"
//! (chapter 1), but a generator sends whatever clock it is set to, so a
//! reading is a date and a time of day and nothing more. Of a leap second
//! the standard says only that the SBS read 0 at 24:00 "excluding leap
//! second days when a second may be added or subtracted" (§3.6), and it
//! lays out no leap-second frame. The frame of 23:59:60 is this module's
//! choice: the SBS reach 86 400, the BCD seconds read 60, and a reading of
//! second 60 is accepted only at the end of a month.
//! `docs/systems/irig-time-codes.md` works a frame through.

use core::fmt;

use hc_calendar::{CivilDateTime, CivilTime};

use crate::radio::{
    Frame, FrameError, FrameResult, Symbol, day_of_year, is_last_minute_of_month, read_bcd,
    split_year, write_bcd, year_in_century, ymd,
};

/// The longest frame, 100 index counts.
pub const MAX_FRAME: usize = 100;

/// Attoseconds in a hundredth of a second.
const ATTOS_PER_HUNDREDTH: u64 = 10_000_000_000_000_000;

/// One of the six formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IrigFormat {
    /// 1 000 pulses a second, a frame every 0.1 s.
    A,
    /// 100 pulses a second, a frame every second.
    B,
    /// One pulse a minute, a frame every hour.
    D,
    /// 10 pulses a second, a frame every 10 s.
    E,
    /// 10 000 pulses a second, a frame every 10 ms.
    G,
    /// One pulse a second, a frame every minute.
    H,
}

/// A BCD field: its digits, most significant first and each digit's
/// positions most significant bit first, as [`read_bcd`] reads them, and
/// the unit of its last digit.
struct Field {
    digits: &'static [&'static [usize]],
    unit: u32,
}

const NONE: Field = Field {
    digits: &[],
    unit: 1,
};

/// Where a format puts its fields (Tables 5-1, 5-4, 5-7, 5-9, 5-12, 5-15).
struct Layout {
    seconds: Field,
    minutes: Field,
    hours: Field,
    days: Field,
    /// Hundredths of a second.
    fraction: Field,
    year: Field,
    /// Control bit 1 first.
    control: &'static [usize],
    /// 2⁰ first.
    sbs: &'static [usize],
}

const SECONDS: Field = Field {
    digits: &[&[8, 7, 6], &[4, 3, 2, 1]],
    unit: 1,
};
const MINUTES: Field = Field {
    digits: &[&[17, 16, 15], &[13, 12, 11, 10]],
    unit: 1,
};
const HOURS: Field = Field {
    digits: &[&[26, 25], &[23, 22, 21, 20]],
    unit: 1,
};
const DAYS: Field = Field {
    digits: &[&[41, 40], &[38, 37, 36, 35], &[33, 32, 31, 30]],
    unit: 1,
};
const YEAR_AT_50: Field = Field {
    digits: &[&[58, 57, 56, 55], &[53, 52, 51, 50]],
    unit: 1,
};
const CONTROL_AT_60: &[usize] = &[
    60, 61, 62, 63, 64, 65, 66, 67, 68, 70, 71, 72, 73, 74, 75, 76, 77, 78,
];
const CONTROL_AT_50: &[usize] = &[50, 51, 52, 53, 54, 55, 56, 57, 58];
const SBS: &[usize] = &[
    80, 81, 82, 83, 84, 85, 86, 87, 88, 90, 91, 92, 93, 94, 95, 96, 97,
];

const LAYOUT_A: Layout = Layout {
    seconds: SECONDS,
    minutes: MINUTES,
    hours: HOURS,
    days: DAYS,
    fraction: Field {
        digits: &[&[48, 47, 46, 45]],
        unit: 10,
    },
    year: YEAR_AT_50,
    control: CONTROL_AT_60,
    sbs: SBS,
};
const LAYOUT_B: Layout = Layout {
    fraction: NONE,
    ..LAYOUT_A
};
const LAYOUT_D: Layout = Layout {
    seconds: NONE,
    minutes: NONE,
    hours: HOURS,
    days: DAYS,
    fraction: NONE,
    year: NONE,
    control: CONTROL_AT_50,
    sbs: &[],
};
const LAYOUT_E: Layout = Layout {
    seconds: Field {
        digits: &[&[8, 7, 6]],
        unit: 10,
    },
    sbs: &[],
    ..LAYOUT_B
};
const LAYOUT_G: Layout = Layout {
    fraction: Field {
        digits: &[&[48, 47, 46, 45], &[53, 52, 51, 50]],
        unit: 1,
    },
    year: Field {
        digits: &[&[68, 67, 66, 65], &[63, 62, 61, 60]],
        unit: 1,
    },
    control: &[
        70, 71, 72, 73, 74, 75, 76, 77, 78, 80, 81, 82, 83, 84, 85, 86, 87, 88, 90, 91, 92, 93, 94,
        95, 96, 97, 98,
    ],
    sbs: &[],
    ..LAYOUT_A
};
const LAYOUT_H: Layout = Layout {
    minutes: MINUTES,
    ..LAYOUT_D
};

impl IrigFormat {
    /// The six formats.
    pub const ALL: [Self; 6] = [Self::A, Self::B, Self::D, Self::E, Self::G, Self::H];

    /// The format's letter.
    #[must_use]
    pub const fn letter(self) -> char {
        match self {
            Self::A => 'A',
            Self::B => 'B',
            Self::D => 'D',
            Self::E => 'E',
            Self::G => 'G',
            Self::H => 'H',
        }
    }

    /// The format a letter names, in either case.
    #[must_use]
    pub const fn from_letter(letter: char) -> Option<Self> {
        match letter.to_ascii_uppercase() {
            'A' => Some(Self::A),
            'B' => Some(Self::B),
            'D' => Some(Self::D),
            'E' => Some(Self::E),
            'G' => Some(Self::G),
            'H' => Some(Self::H),
            _ => None,
        }
    }

    /// The index counts in a frame: 100, or 60 for D and H (Table 3-2).
    #[must_use]
    pub const fn frame_len(self) -> usize {
        match self {
            Self::D | Self::H => 60,
            _ => 100,
        }
    }

    /// The index count interval in microseconds (Table 3-1).
    #[must_use]
    pub const fn index_count_micros(self) -> u64 {
        match self {
            Self::A => 1_000,
            Self::B => 10_000,
            Self::D => 60_000_000,
            Self::E => 100_000,
            Self::G => 100,
            Self::H => 1_000_000,
        }
    }

    /// The time frame in microseconds (Table 3-2).
    #[must_use]
    pub const fn frame_micros(self) -> u64 {
        self.frame_len() as u64 * self.index_count_micros()
    }

    /// The number of control bits (Table 3-4).
    #[must_use]
    pub const fn control_bits(self) -> usize {
        self.layout().control.len()
    }

    /// The fields of the BCD time of year, most significant first, as
    /// the module's table names them: `days`, `hours`, `minutes`,
    /// `seconds` or E's `tens-of-seconds`, and A's `tenths` or G's
    /// `tenths` and `hundredths`. The last is the frame's length: a frame
    /// begins at every multiple of it from midnight.
    #[must_use]
    pub const fn time_fields(self) -> &'static [&'static str] {
        match self {
            Self::A => &["days", "hours", "minutes", "seconds", "tenths"],
            Self::B => &["days", "hours", "minutes", "seconds"],
            Self::D => &["days", "hours"],
            Self::E => &["days", "hours", "minutes", "tens-of-seconds"],
            Self::G => &[
                "days",
                "hours",
                "minutes",
                "seconds",
                "tenths",
                "hundredths",
            ],
            Self::H => &["days", "hours", "minutes"],
        }
    }

    /// The modulation types Table 4-1 permits the format, the first digit
    /// of a signal designation.
    #[must_use]
    pub const fn modulations(self) -> &'static [u8] {
        self.permitted().0
    }

    /// The carrier frequencies Table 4-1 permits the format, the second
    /// digit.
    #[must_use]
    pub const fn carriers(self) -> &'static [u8] {
        self.permitted().1
    }

    /// The coded expressions Table 4-1 permits the format, the third
    /// digit.
    #[must_use]
    pub const fn expressions(self) -> &'static [u8] {
        self.permitted().2
    }

    const fn layout(self) -> &'static Layout {
        match self {
            Self::A => &LAYOUT_A,
            Self::B => &LAYOUT_B,
            Self::D => &LAYOUT_D,
            Self::E => &LAYOUT_E,
            Self::G => &LAYOUT_G,
            Self::H => &LAYOUT_H,
        }
    }

    /// Table 4-1: the modulation types, frequencies and coded expressions
    /// the format allows.
    const fn permitted(self) -> (&'static [u8], &'static [u8], &'static [u8]) {
        match self {
            Self::A => (&[0, 1, 2], &[0, 3, 4, 5], &[0, 1, 2, 3, 4, 5, 6, 7]),
            Self::B => (&[0, 1, 2], &[0, 2, 3, 4, 5], &[0, 1, 2, 3, 4, 5, 6, 7]),
            Self::D | Self::H => (&[0, 1], &[0, 1, 2], &[1, 2]),
            Self::E => (&[0, 1], &[0, 1, 2], &[1, 2, 5, 6]),
            Self::G => (&[0, 1, 2], &[0, 4, 5], &[1, 2, 5, 6]),
        }
    }
}

impl fmt::Display for IrigFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IRIG {}", self.letter())
    }
}

const fn contains(list: &[u8], value: u8) -> bool {
    let mut index = 0;
    while index < list.len() {
        if list[index] == value {
            return true;
        }
        index += 1;
    }
    false
}

/// A format and the coded expression that says which fields its frames
/// carry (Figure 4-1): 0 BCD time of year, control bits and SBS; 1 time
/// and control bits; 2 time alone; 3 time and SBS; and 4 to 7 the same
/// with the BCD year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IrigCode {
    format: IrigFormat,
    expression: u8,
}

impl IrigCode {
    /// A format with a coded expression Table 4-1 permits it.
    ///
    /// # Errors
    ///
    /// [`FrameError::Field`] for an expression Table 4-1 does not list
    /// for the format.
    pub const fn new(format: IrigFormat, expression: u8) -> FrameResult<Self> {
        if !contains(format.permitted().2, expression) {
            return Err(FrameError::Field("coded expression"));
        }
        Ok(Self { format, expression })
    }

    /// The code of a signal designation, a format letter and three digits
    /// — modulation, frequency and coded expression — as `B122`, with each
    /// digit checked against Table 4-1. The modulation and the carrier are
    /// how the pulses are sent and are not kept: the frame is the same.
    /// Table 4-1 lists the modulations and the frequencies apart, so every
    /// pairing it lists is accepted, B020 and B100 among them, although
    /// Figure 4-1 makes frequency 0 "no carrier".
    ///
    /// # Errors
    ///
    /// [`FrameError::Field`] for text of another shape, or a digit Table
    /// 4-1 does not permit the format.
    pub fn from_signal(signal: &str) -> FrameResult<Self> {
        let refused = FrameError::Field("signal designation");
        let mut characters = signal.trim().chars();
        let format = characters
            .next()
            .and_then(IrigFormat::from_letter)
            .ok_or(refused)?;
        let mut digits = [0u8; 3];
        for digit in &mut digits {
            *digit = characters
                .next()
                .and_then(|character| character.to_digit(10))
                .ok_or(refused)? as u8;
        }
        let (modulations, frequencies, _) = format.permitted();
        if characters.next().is_some()
            || !contains(modulations, digits[0])
            || !contains(frequencies, digits[1])
        {
            return Err(refused);
        }
        Self::new(format, digits[2]).map_err(|_| refused)
    }

    /// The format.
    #[must_use]
    pub const fn format(self) -> IrigFormat {
        self.format
    }

    /// The coded expression, 0 to 7.
    #[must_use]
    pub const fn expression(self) -> u8 {
        self.expression
    }

    /// Whether the frames carry the year.
    #[must_use]
    pub const fn has_year(self) -> bool {
        self.expression >= 4
    }

    /// Whether the frames carry control bits.
    #[must_use]
    pub const fn has_control(self) -> bool {
        matches!(self.expression % 4, 0 | 1)
    }

    /// Whether the frames carry the straight binary seconds.
    #[must_use]
    pub const fn has_sbs(self) -> bool {
        matches!(self.expression % 4, 0 | 3)
    }
}

/// A decoded IRIG frame: the time of year at its reference bit Pr.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IrigFrame {
    /// The format and the fields the frame carries.
    pub code: IrigCode,
    /// The day of the year, 1 January being 1.
    pub day_of_year: u16,
    /// The hour, 0 to 23.
    pub hour: u8,
    /// The minute, 0 in format D.
    pub minute: u8,
    /// The second, 60 for a leap second; 0 in formats D and H and a
    /// multiple of 10 in E.
    pub second: u8,
    /// Hundredths of a second: a multiple of 10 in format A, any value in
    /// G, 0 in the others.
    pub hundredths: u8,
    /// The year's last two digits, where the code carries them.
    pub year: Option<u8>,
    /// The control bits, control bit 1 as bit 0; 0 where the code carries
    /// none.
    pub control: u32,
}

/// A binary number sent least significant bit first at `positions`.
fn lsb_first(bit: &impl Fn(usize) -> FrameResult<bool>, positions: &[usize]) -> FrameResult<u32> {
    positions.iter().rev().try_fold(0, |value, &position| {
        Ok(value << 1 | u32::from(bit(position)?))
    })
}

/// Whether `position` is a position identifier or Pr in a frame of `len`.
const fn is_marker(position: usize, len: usize) -> bool {
    position % 10 == 9 || position == 0 || position + 1 == len
}

impl IrigFrame {
    /// The frame whose Pr falls at `reading`.
    ///
    /// # Errors
    ///
    /// [`FrameError::Field`] for a reading the format has no frame at —
    /// for format B one off the second, for D one off the hour — or
    /// control bits the code does not carry.
    pub fn for_reading(code: IrigCode, reading: CivilDateTime, control: u32) -> FrameResult<Self> {
        let (year, month, day) = ymd(reading)?;
        let day_of_year = hc_calendar::gregorian::day_of_year(year, month, day)
            .map_err(|_| FrameError::Field("date"))?;
        let time = reading.time;
        let attos = time.subsec_attos();
        if !attos.is_multiple_of(ATTOS_PER_HUNDREDTH) {
            return Err(FrameError::Field("a frame boundary"));
        }
        let frame = Self {
            code,
            day_of_year,
            hour: time.hour(),
            minute: time.minute(),
            second: time.second(),
            hundredths: (attos / ATTOS_PER_HUNDREDTH) as u8,
            year: code.has_year().then_some(split_year(year).1),
            control,
        };
        frame.check()?;
        Ok(frame)
    }

    /// Whether each field is one the code carries and has room for.
    fn check(&self) -> FrameResult<()> {
        let layout = self.code.format.layout();
        let fits = |field: &Field, value: u8| {
            value == 0 || (!field.digits.is_empty() && u32::from(value) % field.unit == 0)
        };
        if !(fits(&layout.seconds, self.second)
            && fits(&layout.minutes, self.minute)
            && fits(&layout.fraction, self.hundredths))
        {
            return Err(FrameError::Field("a frame boundary"));
        }
        CivilTime::new(self.hour, self.minute, self.second, 0)
            .map_err(|_| FrameError::Field("time of day"))?;
        if self.hundredths > 99 {
            return Err(FrameError::Field("time of day"));
        }
        if !(1..=366).contains(&self.day_of_year) {
            return Err(FrameError::Field("day of year"));
        }
        if self.year.is_some() != self.code.has_year() || self.year.is_some_and(|year| year > 99) {
            return Err(FrameError::Field("year"));
        }
        let room = if self.code.has_control() {
            layout.control.len()
        } else {
            0
        };
        if u64::from(self.control) >> room != 0 {
            return Err(FrameError::Field("control bits"));
        }
        Ok(())
    }

    /// The straight binary seconds of the day at Pr.
    #[must_use]
    pub const fn straight_binary_seconds(&self) -> u32 {
        self.hour as u32 * 3_600 + self.minute as u32 * 60 + self.second as u32
    }

    /// Decode a frame of the code's length, Pr first.
    ///
    /// # Errors
    ///
    /// [`FrameError::Length`]; [`FrameError::Symbol`] for a marker out of
    /// place, a missing one, or a 1 at a count the code leaves as an index
    /// marker; [`FrameError::Digit`]; and [`FrameError::Field`] for a time
    /// of day out of range, or SBS that are not the BCD time's.
    pub fn decode(code: IrigCode, symbols: &[Symbol]) -> FrameResult<Self> {
        let format = code.format;
        let len = format.frame_len();
        if symbols.len() != len {
            return Err(FrameError::Length(symbols.len()));
        }
        let layout = format.layout();
        let mut used = [false; MAX_FRAME];
        for positions in Self::fields(code) {
            for &position in positions {
                used[position] = true;
            }
        }
        for (position, &symbol) in symbols.iter().enumerate() {
            let marker = is_marker(position, len);
            if (symbol == Symbol::Marker) != marker || symbol == Symbol::One && !used[position] {
                return Err(FrameError::Symbol(position));
            }
        }
        let bit = |position: usize| Ok(symbols[position] == Symbol::One);
        let field = |field: &Field| {
            Ok::<u8, FrameError>((read_bcd(&bit, field.digits)? * field.unit) as u8)
        };
        let frame = Self {
            code,
            day_of_year: read_bcd(&bit, layout.days.digits)? as u16,
            hour: field(&layout.hours)?,
            minute: field(&layout.minutes)?,
            second: field(&layout.seconds)?,
            hundredths: field(&layout.fraction)?,
            year: if code.has_year() {
                Some(field(&layout.year)?)
            } else {
                None
            },
            control: if code.has_control() {
                lsb_first(&bit, layout.control)?
            } else {
                0
            },
        };
        frame.check()?;
        if code.has_sbs() && lsb_first(&bit, layout.sbs)? != frame.straight_binary_seconds() {
            return Err(FrameError::Field("straight binary seconds"));
        }
        Ok(frame)
    }

    /// The positions of the fields a code carries.
    fn fields(code: IrigCode) -> impl Iterator<Item = &'static [usize]> {
        let layout = code.format.layout();
        let time = [
            &layout.seconds,
            &layout.minutes,
            &layout.hours,
            &layout.days,
            &layout.fraction,
        ]
        .into_iter()
        .chain(code.has_year().then_some(&layout.year))
        .flat_map(|field| field.digits.iter().copied());
        time.chain(code.has_control().then_some(layout.control))
            .chain(code.has_sbs().then_some(layout.sbs))
    }

    /// The frame's symbols, Pr first.
    ///
    /// # Errors
    ///
    /// [`FrameError::Field`] as [`IrigFrame::for_reading`], for a field
    /// out of range or one the code does not carry.
    pub fn encode(&self) -> FrameResult<Frame<Symbol, MAX_FRAME>> {
        self.check()?;
        let format = self.code.format;
        let layout = format.layout();
        let len = format.frame_len();
        let mut frame = Frame::filled(len);
        for position in (0..len).filter(|&position| is_marker(position, len)) {
            frame.set(position, Symbol::Marker);
        }
        let mut set = |position: usize, value: bool| frame.set(position, Symbol::bit(value));
        let mut write = |field: &Field, value: u8| {
            write_bcd(&mut set, field.digits, u32::from(value) / field.unit);
        };
        write(&layout.seconds, self.second);
        write(&layout.minutes, self.minute);
        write(&layout.hours, self.hour);
        write(&layout.fraction, self.hundredths);
        if let Some(year) = self.year {
            write(&layout.year, year);
        }
        write_bcd(&mut set, layout.days.digits, u32::from(self.day_of_year));
        if self.code.has_control() {
            for (index, &position) in layout.control.iter().enumerate() {
                set(position, self.control >> index & 1 == 1);
            }
        }
        if self.code.has_sbs() {
            let seconds = self.straight_binary_seconds();
            for (index, &position) in layout.sbs.iter().enumerate() {
                set(position, seconds >> index & 1 == 1);
            }
        }
        Ok(frame)
    }

    /// The reading at Pr, its two-digit year read in the century beginning
    /// `century`, as 2000.
    ///
    /// # Errors
    ///
    /// [`FrameError::Field`] for a code without the year (use
    /// [`IrigFrame::reading_in_year`]), a century that is not a multiple
    /// of 100, a day the year does not have, or a second 60 anywhere but
    /// at the end of a month.
    pub fn reading(&self, century: i64) -> FrameResult<CivilDateTime> {
        let year = self.year.ok_or(FrameError::Field("year"))?;
        self.reading_in_year(year_in_century(century, year)?)
    }

    /// The reading at Pr in `year`, which a code without the year needs
    /// and a code with it must agree with.
    ///
    /// # Errors
    ///
    /// As [`IrigFrame::reading`], and [`FrameError::Field`] for a year
    /// whose last two digits are not the frame's.
    pub fn reading_in_year(&self, year: i64) -> FrameResult<CivilDateTime> {
        if self
            .year
            .is_some_and(|two_digits| two_digits != split_year(year).1)
        {
            return Err(FrameError::Field("year"));
        }
        let day = day_of_year(year, self.day_of_year)?;
        let time = CivilTime::new(
            self.hour,
            self.minute,
            self.second,
            u64::from(self.hundredths) * ATTOS_PER_HUNDREDTH,
        )
        .map_err(|_| FrameError::Field("time of day"))?;
        let reading = CivilDateTime::new(day, time);
        if self.second == 60 && !is_last_minute_of_month(reading, 23, 59) {
            return Err(FrameError::Field("leap second"));
        }
        Ok(reading)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::{Rd, gregorian};

    /// A frame written as the standard's figures draw it, `M`, `1` and
    /// `0`, Pr first, spaces ignored.
    fn frame(text: &str) -> alloc::vec::Vec<Symbol> {
        text.chars()
            .filter(|character| !character.is_whitespace())
            .map(|character| match character {
                'M' => Symbol::Marker,
                '1' => Symbol::One,
                _ => Symbol::Zero,
            })
            .collect()
    }

    fn code(format: IrigFormat, expression: u8) -> IrigCode {
        IrigCode::new(format, expression).expect("permitted")
    }

    /// Figure 5-2, the pulses of IRIG B read off the figure: day 173,
    /// 21:18:42, year 03, the control bits all 0, and the SBS 76 722.
    const FIGURE_5_2: &str = "M01000001M 000101000M 100000100M 110001110M 100000000M \
                              110000000M 000000000M 000000000M 010011011M 101010010M";

    /// Figure 5-1, IRIG A: the same, with 8 tenths at counts 45–48.
    const FIGURE_5_1: &str = "M01000001M 000101000M 100000100M 110001110M 100000001M \
                              110000000M 000000000M 000000000M 010011011M 101010010M";

    /// Figure 5-5, IRIG G: 21:18:42.80 on day 173, the hundredths 0 at
    /// counts 50–53 and the year 03 at 60–68.
    const FIGURE_5_5: &str = "M01000001M 000101000M 100000100M 110001110M 100000001M \
                              000000000M 110000000M 000000000M 000000000M 000000000M";

    /// Figure 5-3, IRIG D: day 173, hour 21.
    const FIGURE_5_3: &str = "M00000000M 000000000M 100000100M 110001110M 100000000M 000000000M";

    /// Figure 5-6, IRIG H: day 173, 21:24.
    const FIGURE_5_6: &str = "M00000000M 001000100M 100000100M 110001110M 100000000M 000000000M";

    fn reading(year: i64, month: u8, day: u8, hms: (u8, u8, u8), hundredths: u8) -> CivilDateTime {
        CivilDateTime::new(
            gregorian::to_fixed(year, month, day).expect("exists"),
            CivilTime::new(
                hms.0,
                hms.1,
                hms.2,
                u64::from(hundredths) * ATTOS_PER_HUNDREDTH,
            )
            .expect("valid"),
        )
    }

    /// Each figure's frame, both ways: 22 June 2003 is day 173.
    /// Each format's fields are its layout's, and its frame is its finest
    /// field: A 0.1 s and tenths, D an hour and hours.
    #[test]
    fn the_time_fields_are_the_layouts() {
        for format in IrigFormat::ALL {
            let layout = format.layout();
            let carried = [
                &layout.days,
                &layout.hours,
                &layout.minutes,
                &layout.seconds,
                &layout.fraction,
            ]
            .iter()
            .filter(|field| !field.digits.is_empty())
            .count()
                + usize::from(matches!(format, IrigFormat::G));
            assert_eq!(format.time_fields().len(), carried, "{format}");
        }
        assert_eq!(IrigFormat::A.frame_micros(), 100_000);
        assert_eq!(IrigFormat::D.frame_micros(), 3_600_000_000);
        assert_eq!(IrigFormat::G.time_fields().last(), Some(&"hundredths"));
        assert_eq!(IrigFormat::B.carriers(), &[0, 2, 3, 4, 5]);
    }

    #[test]
    fn the_standards_figures() {
        let cases = [
            (FIGURE_5_2, code(IrigFormat::B, 4), (21, 18, 42), 0),
            (FIGURE_5_1, code(IrigFormat::A, 4), (21, 18, 42), 80),
            (FIGURE_5_5, code(IrigFormat::G, 5), (21, 18, 42), 80),
            (FIGURE_5_3, code(IrigFormat::D, 1), (21, 0, 0), 0),
            (FIGURE_5_6, code(IrigFormat::H, 1), (21, 24, 0), 0),
        ];
        for (text, code, hms, hundredths) in cases {
            let symbols = frame(text);
            let decoded = IrigFrame::decode(code, &symbols).expect("the figure's frame");
            let expected = reading(2003, 6, 22, hms, hundredths);
            assert_eq!(decoded.day_of_year, 173);
            assert_eq!(decoded.control, 0);
            if code.has_year() {
                assert_eq!(decoded.year, Some(3));
                assert_eq!(decoded.reading(2000), Ok(expected));
            } else {
                assert_eq!(decoded.reading(2000), Err(FrameError::Field("year")));
                assert_eq!(decoded.reading_in_year(2003), Ok(expected));
            }
            assert_eq!(decoded.encode().expect("valid").as_slice(), symbols);
            assert_eq!(IrigFrame::for_reading(code, expected, 0), Ok(decoded));
        }
        let b = IrigFrame::decode(code(IrigFormat::B, 4), &frame(FIGURE_5_2)).expect("valid");
        assert_eq!(b.straight_binary_seconds(), 76_722);
    }

    /// Figure 5-4 draws IRIG E with Figure 5-2's pulses, units of seconds
    /// and SBS included, which Table 5-9 and Table 4-1 do not give E. Its
    /// note puts count 75 at "21 Hours, 18 Minutes, 47.5 Seconds", and 75
    /// counts of 0.1 s after Pr make Pr 21:18:40. The drawn frame is refused
    /// at its first units bit, and the frame of 21:18:40 is written without
    /// them.
    #[test]
    fn figure_5_4_is_not_an_e_frame() {
        let e = code(IrigFormat::E, 5);
        assert_eq!(
            IrigFrame::decode(e, &frame(FIGURE_5_2)),
            Err(FrameError::Symbol(2))
        );
        let at = reading(2003, 6, 22, (21, 18, 40), 0);
        let written = IrigFrame::for_reading(e, at, 0)
            .expect("a frame boundary")
            .encode()
            .expect("valid");
        assert_eq!(
            written.as_slice(),
            frame(
                "M00000001M 000101000M 100000100M 110001110M 100000000M \
                 110000000M 000000000M 000000000M 000000000M 000000000M"
            )
        );
        assert_eq!(
            IrigFrame::for_reading(e, reading(2003, 6, 22, (21, 18, 42), 0), 0),
            Err(FrameError::Field("a frame boundary"))
        );
    }

    #[test]
    fn signal_designations_follow_table_4_1() {
        let b122 = IrigCode::from_signal("B122").expect("permitted");
        assert_eq!((b122.format(), b122.expression()), (IrigFormat::B, 2));
        assert!(!b122.has_year() && !b122.has_control() && !b122.has_sbs());
        let a137 = IrigCode::from_signal("A137").expect("Figure 4-1's example");
        assert!(a137.has_year() && !a137.has_control() && a137.has_sbs());
        assert!(IrigCode::from_signal(" g145 ").is_ok());
        for refused in ["B112", "D130", "E120", "G141x", "C120", "H13", "B12Z"] {
            assert_eq!(
                IrigCode::from_signal(refused),
                Err(FrameError::Field("signal designation")),
                "{refused}"
            );
        }
        assert_eq!(
            IrigCode::new(IrigFormat::E, 0),
            Err(FrameError::Field("coded expression"))
        );
        assert_eq!(
            IrigCode::new(IrigFormat::D, 5),
            Err(FrameError::Field("coded expression"))
        );
        let counts = IrigFormat::ALL.map(IrigFormat::control_bits);
        assert_eq!(counts, [18, 18, 9, 18, 27, 9]);
        let frames = IrigFormat::ALL.map(IrigFormat::frame_micros);
        assert_eq!(
            frames,
            [
                100_000,
                1_000_000,
                3_600_000_000,
                10_000_000,
                10_000,
                60_000_000
            ]
        );
    }

    /// Control bits go at their counts, least significant first, and are
    /// refused where the code has none or too many are set.
    #[test]
    fn control_bits() {
        let at = reading(2003, 6, 22, (21, 18, 42), 0);
        let frame = IrigFrame::for_reading(code(IrigFormat::B, 0), at, 0b10_0000_0001)
            .expect("valid")
            .encode()
            .expect("valid");
        assert_eq!(frame.as_slice()[60], Symbol::One);
        assert_eq!(frame.as_slice()[70], Symbol::One);
        let decoded = IrigFrame::decode(code(IrigFormat::B, 0), frame.as_slice()).expect("valid");
        assert_eq!((decoded.control, decoded.year), (0b10_0000_0001, None));
        let g = IrigFrame::for_reading(code(IrigFormat::G, 1), at, 1 << 26).expect("27 bits");
        assert_eq!(g.encode().expect("valid").as_slice()[98], Symbol::One);
        let at_hour = reading(2003, 6, 22, (21, 0, 0), 0);
        for (code, control) in [
            (code(IrigFormat::B, 4), 1 << 18),
            (code(IrigFormat::B, 7), 1),
            (code(IrigFormat::D, 1), 1 << 9),
        ] {
            assert_eq!(
                IrigFrame::for_reading(code, at_hour, control),
                Err(FrameError::Field("control bits"))
            );
        }
    }

    /// The module's leap-second frame: the SBS reach 86 400 at 23:59:60,
    /// which §3.6 leaves open, and second 60 is read only at the end of a
    /// month.
    #[test]
    fn a_leap_second() {
        let b = code(IrigFormat::B, 7);
        let at = reading(2016, 12, 31, (23, 59, 60), 0);
        let frame = IrigFrame::for_reading(b, at, 0).expect("valid");
        assert_eq!(frame.straight_binary_seconds(), 86_400);
        let symbols = frame.encode().expect("valid");
        let decoded = IrigFrame::decode(b, symbols.as_slice()).expect("valid");
        assert_eq!(decoded.reading(2000), Ok(at));
        let earlier = IrigFrame {
            day_of_year: 365,
            ..decoded
        };
        assert_eq!(earlier.reading(2000), Err(FrameError::Field("leap second")));
    }

    #[test]
    fn broken_frames_are_refused() {
        let b = code(IrigFormat::B, 4);
        let good = frame(FIGURE_5_2);
        let broken = |position: usize, symbol: Symbol| {
            let mut symbols = good.clone();
            symbols[position] = symbol;
            IrigFrame::decode(b, &symbols)
        };
        assert_eq!(broken(9, Symbol::Zero), Err(FrameError::Symbol(9)));
        assert_eq!(broken(0, Symbol::One), Err(FrameError::Symbol(0)));
        assert_eq!(broken(5, Symbol::One), Err(FrameError::Symbol(5)));
        assert_eq!(broken(10, Symbol::Marker), Err(FrameError::Symbol(10)));
        // 42 s with one more SBS bit, and with 52 s in BCD.
        assert_eq!(
            broken(80, Symbol::One),
            Err(FrameError::Field("straight binary seconds"))
        );
        assert_eq!(
            broken(1, Symbol::One),
            Err(FrameError::Field("straight binary seconds"))
        );
        // Units of seconds of 15, and tens of seconds of 70.
        let mut symbols = good.clone();
        symbols[1] = Symbol::One;
        symbols[3] = Symbol::One;
        symbols[4] = Symbol::One;
        // A digit is reported at its most significant bit, as the radio
        // codes report theirs.
        assert_eq!(IrigFrame::decode(b, &symbols), Err(FrameError::Digit(4)));
        let mut symbols = good.clone();
        symbols[6] = Symbol::One;
        symbols[7] = Symbol::One;
        assert_eq!(
            IrigFrame::decode(b, &symbols),
            Err(FrameError::Field("time of day"))
        );
        assert_eq!(
            IrigFrame::decode(b, &good[..99]),
            Err(FrameError::Length(99))
        );
        // The year's counts are index markers in a code without it.
        assert_eq!(
            IrigFrame::decode(code(IrigFormat::B, 0), &good),
            Err(FrameError::Symbol(50))
        );
        let decoded = IrigFrame::decode(b, &good).expect("valid");
        assert_eq!(decoded.reading(2001), Err(FrameError::Field("century")));
        assert_eq!(
            decoded.reading_in_year(2004),
            Err(FrameError::Field("year"))
        );
        let day_366 = IrigFrame {
            day_of_year: 366,
            ..decoded
        };
        assert_eq!(day_366.reading(2000), Err(FrameError::Field("day of year")));
        let wrong_year = IrigFrame {
            year: None,
            ..decoded
        };
        assert_eq!(wrong_year.encode(), Err(FrameError::Field("year")));
        let wrong_day = IrigFrame {
            day_of_year: 0,
            ..decoded
        };
        assert_eq!(wrong_day.encode(), Err(FrameError::Field("day of year")));
    }

    /// Every permitted code at four times of a sample of the days of
    /// 2000–2099, both ways: every day in a release build; every 89th and
    /// every month's first and last in a debug one.
    #[test]
    fn every_code_round_trips() {
        let codes: alloc::vec::Vec<IrigCode> = IrigFormat::ALL
            .into_iter()
            .flat_map(|format| {
                format
                    .permitted()
                    .2
                    .iter()
                    .map(move |&expression| code(format, expression))
            })
            .collect();
        assert_eq!(codes.len(), 8 + 8 + 2 + 4 + 4 + 2);
        for day in crate::radio::sweep_days_2000_to_2099(89) {
            let (year, _, _) = gregorian::from_fixed(Rd(day)).expect("valid");
            for hms in [(0, 0, 0), (9, 0, 0), (12, 34, 0), (23, 59, 50)] {
                for &code in &codes {
                    let at = CivilDateTime::new(
                        Rd(day),
                        CivilTime::new(hms.0, hms.1, hms.2, 0).expect("valid"),
                    );
                    let Ok(frame) = IrigFrame::for_reading(code, at, 0) else {
                        // Off the format's frames: D off the hour, H off
                        // the minute.
                        assert!(matches!(code.format(), IrigFormat::D | IrigFormat::H));
                        continue;
                    };
                    let symbols = frame.encode().expect("valid");
                    let back = IrigFrame::decode(code, symbols.as_slice()).expect("valid");
                    assert_eq!(back, frame);
                    assert_eq!(back.reading_in_year(year), Ok(at));
                }
            }
        }
    }
}
