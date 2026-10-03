//! The time codes of the long-wave time stations: one frame of one symbol
//! a second every minute, decoded to the minute it names and encoded from
//! one.
//!
//! | Module | Station | Time | Source |
//! | --- | --- | --- | --- |
//! | [`jjy`] | JJY, NICT, Japan | JST, UTC+9 | NICT, "標準電波の出し方", read 2026-09-27 (`nict-jjy-timecode`) |
//! | [`dcf77`] | DCF77, PTB, Germany | CET or CEST, as the frame says | PTB, "DCF77 time code", read 2026-09-27 (`ptb-dcf77-timecode`) |
//! | [`wwvb`] | WWVB, NIST, United States, both codes | UTC | NIST SP 432 (2002), SP 250-67 (2005) and "Enhanced WWVB Broadcast Format", revision 1.01 (2013), read 2026-09-27 |
//!
//! A frame is a slice of symbols: [`Symbol`] for the codes that have a
//! marker, `bool` for DCF77's marks and WWVB's phase bits. Its length is
//! the minute's: 60 seconds, 61 with an inserted leap second and 59 with
//! an omitted one, and each code puts the extra or missing second in its
//! own place. Decoding checks every fixed symbol, BCD digit and parity bit
//! and reports the second where the frame is wrong; reading the frame as a
//! date then needs the century, which a two-digit year does not carry, and
//! checks the rest: a day the year does not have, a weekday that is not the
//! date's, or a leap-second frame anywhere but at the end of a month.
//!
//! The pulse widths and carrier phases that make the symbols, and the
//! notices a station decides to send, are outside this module: `encode`
//! writes what it is given. `docs/systems/radio-time-codes.md` works a
//! frame of each code through.

use core::fmt;

use hc_calendar::{CivilDateTime, CivilTime, Rd, gregorian};
use hc_core::UnixTime;
use hc_core::duration::SECONDS_PER_DAY;

pub mod dcf77;
pub mod jjy;
pub mod wwvb;

/// The days of the codes' round-trip sweeps, 2000 to 2099: every day in a
/// release build; in a debug one every `sampled`th day and the first and
/// last day of every month, where a leap second's frames fall
/// (docs/policy.md §7).
#[cfg(test)]
pub(crate) fn sweep_days_2000_to_2099(sampled: usize) -> alloc::vec::Vec<i64> {
    let day = |year: i64, month: u8| gregorian::to_fixed(year, month, 1).map(|rd| rd.0);
    let (Ok(first), Ok(end)) = (day(2000, 1), day(2100, 1)) else {
        return alloc::vec::Vec::new();
    };
    let debug = cfg!(debug_assertions);
    let mut days: alloc::vec::Vec<i64> = (first..end)
        .step_by(if debug { sampled } else { 1 })
        .collect();
    if debug {
        for year in 2000..=2099 {
            for month in 1..=12 {
                if let Ok(start) = day(year, month) {
                    days.extend([start - 1, start]);
                }
            }
        }
        days.push(end - 1);
        days.retain(|&d| (first..end).contains(&d));
        days.sort_unstable();
        days.dedup();
    }
    days
}

/// A symbol of a pulse-width code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Symbol {
    /// A binary 0.
    #[default]
    Zero,
    /// A binary 1.
    One,
    /// A marker: the start of the minute, or a position marker.
    Marker,
}

impl Symbol {
    /// The binary symbol for a bit.
    #[must_use]
    pub const fn bit(value: bool) -> Self {
        if value { Self::One } else { Self::Zero }
    }
}

/// A leap second announced for the end of the month.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LeapNotice {
    /// None.
    #[default]
    None,
    /// A second inserted: the last minute has 61 seconds.
    Positive,
    /// A second omitted: the last minute has 59 seconds.
    Negative,
}

hc_core::catalogue! {
    type: LeapNotice,
    tests: leap_notice_catalogue_tests,
    variants;

    /// The three notices, none first.
    pub const ALL;
    /// The notice's identifier: `none`, `positive` or `negative`.
    pub fn id;
    /// The notice with this identifier.
    pub fn by_id;

    entries: {
        None => "none",
        Positive => "positive",
        Negative => "negative",
    }
}

/// A time code of this module, under the identifier a caller selects it
/// by: JJY, DCF77, and WWVB's amplitude and phase codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Code {
    /// JJY's code, [`jjy`].
    Jjy,
    /// DCF77's code, [`dcf77`].
    Dcf77,
    /// WWVB's amplitude code, [`wwvb::AmFrame`].
    WwvbAm,
    /// WWVB's phase code, [`wwvb::PmFrame`].
    WwvbPm,
}

hc_core::catalogue! {
    type: Code,
    tests: code_catalogue_tests,
    variants;

    /// The four codes.
    pub const ALL;
    /// The code's identifier: `jjy`, `dcf77`, `wwvb-am` or `wwvb-pm`.
    pub fn id;
    /// The code with this identifier.
    pub fn by_id;

    entries: {
        Jjy => "jjy",
        Dcf77 => "dcf77",
        WwvbAm => "wwvb-am",
        WwvbPm => "wwvb-pm",
    }
}

/// Why a frame is not a time code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum FrameError {
    /// A frame of this many seconds, where the code has 59, 60 or 61.
    Length(usize),
    /// The wrong kind of symbol at this second: a marker where a bit
    /// belongs, a bit where a marker belongs, or a 1 where the code fixes
    /// a 0.
    Symbol(usize),
    /// A BCD digit above 9, starting at this second.
    Digit(usize),
    /// The parity bit at this second does not match its word.
    Parity(usize),
    /// A field that is well formed but not a value of it: a day the year
    /// does not have, a weekday that is not the date's, a word of a table
    /// that the table does not list.
    Field(&'static str),
}

impl fmt::Display for FrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Length(length) => write!(f, "a frame of {length} seconds"),
            Self::Symbol(second) => write!(f, "the wrong symbol at second {second}"),
            Self::Digit(second) => write!(f, "a BCD digit above 9 at second {second}"),
            Self::Parity(second) => write!(f, "the parity bit at second {second} does not match"),
            Self::Field(field) => write!(f, "{field} is not a value the code allows"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FrameError {}

/// The result of reading a frame.
pub type FrameResult<T> = Result<T, FrameError>;

/// A frame being written or read: up to `N` symbols, 61 for a minute
/// with a leap second, and 100 for the longest IRIG frame
/// ([`crate::irig`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Frame<T, const N: usize = 61> {
    pub(crate) symbols: [T; N],
    len: u8,
}

impl<T: Copy + Default, const N: usize> Frame<T, N> {
    /// A frame of `len` default symbols, `len` at most `N`.
    pub(crate) fn filled(len: usize) -> Self {
        Self {
            symbols: [T::default(); N],
            len: len.min(N) as u8,
        }
    }

    /// The symbols, second 0 first.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.symbols[..usize::from(self.len)]
    }

    /// The number of seconds.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len as usize
    }

    /// Whether the frame has no seconds, which no decoded frame has.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub(crate) fn set(&mut self, second: usize, symbol: T) {
        self.symbols[second] = symbol;
    }
}

/// The length of a frame, checked to be a minute's.
fn check_length(length: usize) -> FrameResult<u8> {
    match length {
        59..=61 => Ok(length as u8),
        _ => Err(FrameError::Length(length)),
    }
}

/// Read the bits at `positions`, most significant first, as a binary
/// number.
pub(crate) fn read_bits(
    bit: impl Fn(usize) -> FrameResult<bool>,
    positions: &[usize],
) -> FrameResult<u32> {
    positions
        .iter()
        .try_fold(0, |value, &second| Ok(value << 1 | u32::from(bit(second)?)))
}

/// Read a BCD number whose digits are groups of `positions`, the most
/// significant digit first and each digit's bits most significant first.
pub(crate) fn read_bcd(
    bit: &impl Fn(usize) -> FrameResult<bool>,
    digits: &[&[usize]],
) -> FrameResult<u32> {
    digits.iter().try_fold(0, |value, positions| {
        let digit = read_bits(bit, positions)?;
        if digit > 9 {
            return Err(FrameError::Digit(positions[0]));
        }
        Ok(value * 10 + digit)
    })
}

/// Write `value` as BCD into `digits`, the inverse of [`read_bcd`].
pub(crate) fn write_bcd(mut set: impl FnMut(usize, bool), digits: &[&[usize]], value: u32) {
    let mut rest = value;
    for positions in digits.iter().rev() {
        let digit = rest % 10;
        rest /= 10;
        for (index, &second) in positions.iter().rev().enumerate() {
            set(second, digit >> index & 1 == 1);
        }
    }
}

/// The number of ones among `positions`, modulo 2.
fn odd_ones(
    bit: &impl Fn(usize) -> FrameResult<bool>,
    positions: impl IntoIterator<Item = usize>,
) -> FrameResult<bool> {
    positions
        .into_iter()
        .try_fold(false, |odd, second| Ok(odd ^ bit(second)?))
}

/// The year a two-digit year names in the century beginning `century`.
pub(crate) fn year_in_century(century: i64, two_digits: u8) -> FrameResult<i64> {
    if century.rem_euclid(100) != 0 {
        return Err(FrameError::Field("century"));
    }
    Ok(century + i64::from(two_digits))
}

/// The day a day of the year names.
pub(crate) fn day_of_year(year: i64, day: u16) -> FrameResult<Rd> {
    if !(1..=gregorian::days_in_year(year)).contains(&day) {
        return Err(FrameError::Field("day of year"));
    }
    Ok(Rd(gregorian::new_year(year).0 + i64::from(day) - 1))
}

/// The minute that begins at `hour`:`minute` on `day`.
fn minute_reading(day: Rd, hour: u8, minute: u8) -> FrameResult<CivilDateTime> {
    let time = CivilTime::hms(hour, minute, 0).map_err(|_| FrameError::Field("time of day"))?;
    Ok(CivilDateTime::new(day, time))
}

/// Whether a minute is the last of a month on the clock it is read on.
pub(crate) fn is_last_minute_of_month(reading: CivilDateTime, hour: u8, minute: u8) -> bool {
    let tomorrow = gregorian::from_fixed(Rd(reading.day.0 + 1));
    (reading.time.hour(), reading.time.minute()) == (hour, minute)
        && matches!(tomorrow, Ok((_, _, 1)))
}

/// The POSIX time of a minute read on a clock `offset_hours` ahead of UTC.
fn unix_of(reading: CivilDateTime, offset_hours: i64) -> FrameResult<UnixTime> {
    let seconds = reading
        .day
        .to_unix_days()
        .checked_mul(SECONDS_PER_DAY)
        .and_then(|start| {
            start.checked_add(
                i64::from(reading.time.hour()) * 3_600 + i64::from(reading.time.minute()) * 60
                    - offset_hours * 3_600,
            )
        })
        .ok_or(FrameError::Field("date"))?;
    Ok(UnixTime::from_seconds(seconds))
}

/// The year, month and day of a reading, for writing a frame.
pub(crate) fn ymd(reading: CivilDateTime) -> FrameResult<(i64, u8, u8)> {
    gregorian::from_fixed(reading.day).map_err(|_| FrameError::Field("date"))
}

/// The two digits of a year, and its century.
pub(crate) const fn split_year(year: i64) -> (i64, u8) {
    (year - year.rem_euclid(100), year.rem_euclid(100) as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bcd_digits_round_trip_and_refuse_above_nine() {
        let digits: &[&[usize]] = &[&[0, 1], &[2, 3, 4, 5]];
        let mut bits = [false; 6];
        write_bcd(|second, value| bits[second] = value, digits, 29);
        assert_eq!(bits, [true, false, true, false, false, true]);
        let bit = |second: usize| Ok(bits[second]);
        assert_eq!(read_bcd(&bit, digits), Ok(29));
        let bits = [false, false, true, true, true, true];
        let bit = |second: usize| Ok(bits[second]);
        assert_eq!(read_bcd(&bit, digits), Err(FrameError::Digit(2)));
    }

    #[test]
    fn a_century_is_a_multiple_of_a_hundred() {
        assert_eq!(year_in_century(2000, 26), Ok(2026));
        assert_eq!(year_in_century(1900, 99), Ok(1999));
        assert_eq!(year_in_century(2001, 0), Err(FrameError::Field("century")));
        assert_eq!(split_year(2026), (2000, 26));
    }

    #[test]
    fn frame_lengths_are_a_minutes() {
        assert_eq!(check_length(60), Ok(60));
        assert_eq!(check_length(58), Err(FrameError::Length(58)));
        assert_eq!(check_length(62), Err(FrameError::Length(62)));
    }
}
