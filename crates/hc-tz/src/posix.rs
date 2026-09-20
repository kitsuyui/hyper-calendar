//! POSIX `TZ` strings: parsing, evaluation and rendering.
//!
//! A POSIX `TZ` string states a zone's *current* rules in one line:
//!
//! ```text
//! std offset[dst[offset][,start[/time],end[/time]]]
//! ```
//!
//! `JST-9` is Japan; `EST5EDT,M3.2.0,M11.1.0` is the eastern United States
//! since 2007; `<-03>3<-02>,M10.3.0/0,M2.3.0/0` is Brazil as it was before it
//! abandoned daylight saving in 2019. The grammar is POSIX.1-2017 §8.3,
//! extended by RFC 8536 §3.3 — which is where the `<>`-quoted abbreviations
//! and the `/time` values outside `00:00..=24:00` come from, and why this
//! parser accepts hours up to 167 in a transition time.
//!
//! # What a POSIX string cannot say
//!
//! It has one pair of rules and applies them to every year, past and future.
//! It cannot express that the United States moved its spring transition from
//! April to March in 2007, that Brazil stopped changing its clocks, or that
//! Japan observed daylight saving from 1948 to 1951. For history, read TZif
//! data with [`crate::tzif`]; this module answers "what are the rules now".

use core::fmt;

use hc_calendar::CivilDateTime;
use hc_calendar::fixed::RD_OF_UNIX_EPOCH;
use hc_core::UnixTime;

use crate::error::{TzError, TzResult};
use crate::gregorian;
use crate::offset::UtcOffset;
use crate::zone::{LocalResolution, TimeZone, resolve_local_by_probing};

/// The local time a transition happens at when the rule omits `/time`:
/// 02:00:00, as POSIX.1-2017 §8.3 specifies.
pub const DEFAULT_TRANSITION_TIME: i32 = 2 * 3_600;

/// The largest magnitude a `/time` may have, in hours.
///
/// RFC 8536 §3.3.2 widened POSIX's `0..=24` to `-167..=167` so that a rule can
/// name a transition a week either side of the day it is anchored to.
const MAX_RULE_TIME_HOURS: i32 = 167;

/// The longest zone abbreviation this crate stores.
///
/// POSIX sets no upper bound; TZif designations are at most six characters in
/// practice. Sixteen leaves room for anything real without an allocator.
pub const MAX_ABBREVIATION_LEN: usize = 16;

/// A zone abbreviation, stored inline so that `no_std` builds need no
/// allocator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Abbreviation {
    bytes: [u8; MAX_ABBREVIATION_LEN],
    len: u8,
}

impl Abbreviation {
    /// Store an abbreviation.
    ///
    /// # Errors
    ///
    /// Returns [`TzError::MalformedAbbreviation`] when the text is empty,
    /// longer than [`MAX_ABBREVIATION_LEN`], or not ASCII alphanumeric with
    /// `+` and `-` allowed.
    pub const fn new(text: &str) -> TzResult<Self> {
        let source = text.as_bytes();
        if source.is_empty() || source.len() > MAX_ABBREVIATION_LEN {
            return Err(TzError::MalformedAbbreviation);
        }
        let mut bytes = [0u8; MAX_ABBREVIATION_LEN];
        let mut index = 0;
        while index < source.len() {
            let byte = source[index];
            if !byte.is_ascii_alphanumeric() && byte != b'+' && byte != b'-' {
                return Err(TzError::MalformedAbbreviation);
            }
            bytes[index] = byte;
            index += 1;
        }
        Ok(Self {
            bytes,
            len: source.len() as u8,
        })
    }

    /// The abbreviation as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        // `new` admits only ASCII, so this cannot fail; the fallback keeps the
        // accessor total without an `unwrap`.
        core::str::from_utf8(&self.bytes[..self.len as usize]).unwrap_or("")
    }

    /// Whether the abbreviation needs `<>` quoting in a `TZ` string.
    ///
    /// POSIX allows bare alphabetic names only; anything with a digit or sign
    /// — `+0545`, `-03` — must be quoted.
    #[must_use]
    pub fn needs_quoting(&self) -> bool {
        !self.as_str().bytes().all(|byte| byte.is_ascii_alphabetic())
    }
}

impl fmt::Display for Abbreviation {
    /// Renders as it would appear in a `TZ` string, quoted when necessary.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.needs_quoting() {
            write!(f, "<{}>", self.as_str())
        } else {
            f.write_str(self.as_str())
        }
    }
}

/// Which day of a year a POSIX transition falls on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PosixRule {
    /// `Jn`: day `n` of the year counting from 1, with 29 February never
    /// counted. `J60` is 1 March in every year, leap or not.
    JulianNoLeap(u16),
    /// `n`: day `n` of the year counting from 0, with 29 February counted.
    /// `n59` is 29 February in a leap year and 1 March otherwise.
    ZeroBasedDay(u16),
    /// `Mm.w.d`: weekday `d` (Sunday is 0) of week `w` of month `m`. Week 5
    /// means the last such weekday in the month, whether that is the fourth
    /// or the fifth.
    MonthWeekDay {
        /// The month, 1 through 12.
        month: u8,
        /// The week, 1 through 5, where 5 means "the last".
        week: u8,
        /// The weekday, 0 (Sunday) through 6 (Saturday).
        day: u8,
    },
}

impl PosixRule {
    /// The fixed day this rule picks out in a proleptic Gregorian year.
    #[must_use]
    pub fn fixed_day(self, year: i64) -> i64 {
        match self {
            Self::JulianNoLeap(day) => {
                let january_first = gregorian::rd_from_ymd(year, 1, 1);
                let ordinal = i64::from(day);
                // 29 February is not counted, so every day from 1 March on is
                // one further along the real calendar in a leap year.
                let skip = i64::from(ordinal >= 60 && gregorian::is_leap_year(year));
                january_first + ordinal - 1 + skip
            }
            Self::ZeroBasedDay(day) => gregorian::rd_from_ymd(year, 1, 1) + i64::from(day),
            Self::MonthWeekDay { month, week, day } => {
                let first = gregorian::rd_from_ymd(year, month, 1);
                let shift = (day + 7 - gregorian::weekday_from_rd(first)) % 7;
                let mut chosen = first + i64::from(shift) + 7 * (i64::from(week) - 1);
                let last = first + i64::from(gregorian::days_in_month(year, month)) - 1;
                while chosen > last {
                    chosen -= 7;
                }
                chosen
            }
        }
    }

    /// Read a rule from the text after a comma in a `TZ` string.
    ///
    /// # Errors
    ///
    /// Returns [`TzError::MalformedRule`] for an unrecognised form or an
    /// out-of-range field.
    pub fn parse(text: &str) -> TzResult<Self> {
        let mut cursor = Cursor::new(text.as_bytes());
        let rule = parse_rule(&mut cursor)?;
        if !cursor.is_empty() {
            return Err(TzError::TrailingInput);
        }
        Ok(rule)
    }
}

impl fmt::Display for PosixRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::JulianNoLeap(day) => write!(f, "J{day}"),
            Self::ZeroBasedDay(day) => write!(f, "{day}"),
            Self::MonthWeekDay { month, week, day } => write!(f, "M{month}.{week}.{day}"),
        }
    }
}

/// A rule together with the local time of day it takes effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PosixTransition {
    /// Which day the transition falls on.
    pub rule: PosixRule,
    /// Seconds after local midnight, from `-167:00:00` to `+167:00:00`.
    /// Values of 86 400 and above are ordinary: Egypt ends its saving period
    /// at `24:00`, the last instant of the day.
    pub time: i32,
}

impl PosixTransition {
    /// A transition at the POSIX default time of 02:00 local.
    #[must_use]
    pub const fn new(rule: PosixRule) -> Self {
        Self {
            rule,
            time: DEFAULT_TRANSITION_TIME,
        }
    }

    /// A transition at a stated local time.
    ///
    /// # Errors
    ///
    /// Returns [`TzError::MalformedRule`] when the time is outside
    /// `-167:00:00 ..= +167:00:00`.
    pub const fn at(rule: PosixRule, time: i32) -> TzResult<Self> {
        if time < -MAX_RULE_TIME_HOURS * 3_600 || time > MAX_RULE_TIME_HOURS * 3_600 {
            return Err(TzError::MalformedRule);
        }
        Ok(Self { rule, time })
    }
}

impl fmt::Display for PosixTransition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.rule)?;
        if self.time != DEFAULT_TRANSITION_TIME {
            f.write_str("/")?;
            write_signed_hms(f, self.time)?;
        }
        Ok(())
    }
}

/// The daylight-saving half of a `TZ` string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PosixDst {
    /// The abbreviation used while saving time is in force.
    pub abbreviation: Abbreviation,
    /// The offset used while saving time is in force.
    pub offset: UtcOffset,
    /// When saving time starts.
    pub start: PosixTransition,
    /// When it ends.
    pub end: PosixTransition,
}

/// A parsed POSIX `TZ` string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PosixTz {
    standard_abbreviation: Abbreviation,
    standard_offset: UtcOffset,
    dst: Option<PosixDst>,
}

impl PosixTz {
    /// Build a zone with no daylight saving.
    #[must_use]
    pub const fn standard(abbreviation: Abbreviation, offset: UtcOffset) -> Self {
        Self {
            standard_abbreviation: abbreviation,
            standard_offset: offset,
            dst: None,
        }
    }

    /// Build a zone with daylight saving.
    #[must_use]
    pub const fn with_dst(abbreviation: Abbreviation, offset: UtcOffset, dst: PosixDst) -> Self {
        Self {
            standard_abbreviation: abbreviation,
            standard_offset: offset,
            dst: Some(dst),
        }
    }

    /// The standard-time abbreviation.
    #[must_use]
    pub const fn standard_abbreviation(&self) -> &Abbreviation {
        &self.standard_abbreviation
    }

    /// The standard-time offset.
    #[must_use]
    pub const fn standard_offset(&self) -> UtcOffset {
        self.standard_offset
    }

    /// The daylight-saving rules, when the zone has any.
    #[must_use]
    pub const fn dst(&self) -> Option<&PosixDst> {
        self.dst.as_ref()
    }

    /// Read a POSIX `TZ` string.
    ///
    /// # Errors
    ///
    /// Returns the [`TzError`] variant describing which part of the grammar
    /// the input failed: [`TzError::MalformedAbbreviation`],
    /// [`TzError::MalformedOffset`], [`TzError::MalformedRule`],
    /// [`TzError::UnexpectedEnd`] or [`TzError::TrailingInput`].
    pub fn parse(text: &str) -> TzResult<Self> {
        let mut cursor = Cursor::new(text.as_bytes());
        if cursor.is_empty() {
            return Err(TzError::UnexpectedEnd);
        }
        let standard_abbreviation = parse_abbreviation(&mut cursor)?;
        let standard_offset = parse_posix_offset(&mut cursor)?;
        if cursor.is_empty() {
            return Ok(Self::standard(standard_abbreviation, standard_offset));
        }
        let abbreviation = parse_abbreviation(&mut cursor)?;
        let offset = if cursor.is_empty() || cursor.peek() == Some(b',') {
            // POSIX: a daylight abbreviation with no offset means one hour
            // ahead of standard time.
            standard_offset.checked_add_seconds(3_600)?
        } else {
            parse_posix_offset(&mut cursor)?
        };
        let (start, end) = if cursor.eat(b',') {
            let start = parse_transition(&mut cursor)?;
            if !cursor.eat(b',') {
                return Err(TzError::MalformedRule);
            }
            let end = parse_transition(&mut cursor)?;
            (start, end)
        } else {
            // tzcode's TZDEFRULESTRING: with a daylight name but no rules, the
            // United States rules in force since 2007 are assumed.
            (
                PosixTransition::new(PosixRule::MonthWeekDay {
                    month: 3,
                    week: 2,
                    day: 0,
                }),
                PosixTransition::new(PosixRule::MonthWeekDay {
                    month: 11,
                    week: 1,
                    day: 0,
                }),
            )
        };
        if !cursor.is_empty() {
            return Err(TzError::TrailingInput);
        }
        Ok(Self::with_dst(
            standard_abbreviation,
            standard_offset,
            PosixDst {
                abbreviation,
                offset,
                start,
                end,
            },
        ))
    }

    /// The instants at which saving time starts and ends in a Gregorian year.
    ///
    /// Both are POSIX timestamps. In the southern hemisphere the end instant
    /// precedes the start instant within the same year, because the saving
    /// period straddles the new year.
    #[must_use]
    pub fn transitions_in_year(&self, year: i64) -> Option<(i64, i64)> {
        let dst = self.dst.as_ref()?;
        Some((
            self.transition_instant(&dst.start, year, self.standard_offset),
            self.transition_instant(&dst.end, year, dst.offset),
        ))
    }

    /// The instant a transition happens, given the offset in force just
    /// before it — which is what a POSIX `/time` is measured in.
    fn transition_instant(
        &self,
        transition: &PosixTransition,
        year: i64,
        offset_in_force: UtcOffset,
    ) -> i64 {
        let day = transition.rule.fixed_day(year) - RD_OF_UNIX_EPOCH;
        day.saturating_mul(86_400)
            .saturating_add(i64::from(transition.time))
            .saturating_sub(i64::from(offset_in_force.seconds()))
    }

    /// The Gregorian year a POSIX timestamp falls in, measured in standard
    /// time. Only used to pick which years' rules to evaluate, so being a few
    /// hours out at a year boundary is harmless: the neighbouring years are
    /// checked too.
    fn approximate_local_year(&self, seconds: i64) -> i64 {
        let shifted = seconds.saturating_add(i64::from(self.standard_offset.seconds()));
        gregorian::year_from_rd(shifted.div_euclid(86_400) + RD_OF_UNIX_EPOCH)
    }

    /// Whether saving time is in force at an instant.
    #[must_use]
    pub fn is_dst_at(&self, utc: UnixTime) -> bool {
        let Some(dst) = self.dst.as_ref() else {
            return false;
        };
        let seconds = utc.seconds();
        let year = self.approximate_local_year(seconds);
        for candidate in [year - 1, year, year + 1] {
            let start = self.transition_instant(&dst.start, candidate, self.standard_offset);
            let end = self.transition_instant(&dst.end, candidate, dst.offset);
            if start <= end {
                if seconds >= start && seconds < end {
                    return true;
                }
            } else {
                // Southern hemisphere: the period runs from this year's start
                // to next year's end.
                let end_next = self.transition_instant(&dst.end, candidate + 1, dst.offset);
                if seconds >= start && seconds < end_next {
                    return true;
                }
            }
        }
        false
    }

    /// The offset in force at an instant.
    #[must_use]
    pub fn offset_at(&self, utc: UnixTime) -> UtcOffset {
        match self.dst.as_ref() {
            Some(dst) if self.is_dst_at(utc) => dst.offset,
            _ => self.standard_offset,
        }
    }

    /// The abbreviation in force at an instant.
    #[must_use]
    pub fn abbreviation_at(&self, utc: UnixTime) -> &str {
        match self.dst.as_ref() {
            Some(dst) if self.is_dst_at(utc) => dst.abbreviation.as_str(),
            _ => self.standard_abbreviation.as_str(),
        }
    }

    /// Resolve a local reading against these rules.
    #[must_use]
    pub fn resolve_local(&self, local: CivilDateTime) -> LocalResolution {
        resolve_local_by_probing(local, |instant| self.offset_at(instant))
    }
}

impl fmt::Display for PosixTz {
    /// Renders the canonical `TZ` string, omitting everything POSIX lets a
    /// reader infer: a daylight offset one hour ahead of standard, and a
    /// transition time of 02:00.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.standard_abbreviation)?;
        write_posix_offset(f, self.standard_offset)?;
        if let Some(dst) = &self.dst {
            write!(f, "{}", dst.abbreviation)?;
            if dst.offset.seconds() != self.standard_offset.seconds() + 3_600 {
                write_posix_offset(f, dst.offset)?;
            }
            write!(f, ",{},{}", dst.start, dst.end)?;
        }
        Ok(())
    }
}

/// A POSIX `TZ` string with a name attached, so that it can be a
/// [`TimeZone`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PosixTimeZone<'a> {
    name: &'a str,
    rules: PosixTz,
}

impl<'a> PosixTimeZone<'a> {
    /// Attach a name to a set of rules.
    #[must_use]
    pub const fn new(name: &'a str, rules: PosixTz) -> Self {
        Self { name, rules }
    }

    /// Parse a `TZ` string and attach a name to it.
    ///
    /// # Errors
    ///
    /// See [`PosixTz::parse`].
    pub fn parse(name: &'a str, text: &str) -> TzResult<Self> {
        Ok(Self::new(name, PosixTz::parse(text)?))
    }

    /// The rules themselves.
    #[must_use]
    pub const fn rules(&self) -> &PosixTz {
        &self.rules
    }
}

impl TimeZone for PosixTimeZone<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn offset_at(&self, utc: UnixTime) -> UtcOffset {
        self.rules.offset_at(utc)
    }

    fn abbreviation_at(&self, utc: UnixTime) -> Option<&str> {
        Some(self.rules.abbreviation_at(utc))
    }

    fn is_dst_at(&self, utc: UnixTime) -> bool {
        self.rules.is_dst_at(utc)
    }

    fn resolve_local(&self, local: CivilDateTime) -> LocalResolution {
        self.rules.resolve_local(local)
    }
}

/// A byte cursor over a `TZ` string.
struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.position).copied()
    }

    fn bump(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.position += 1;
        Some(byte)
    }

    fn eat(&mut self, byte: u8) -> bool {
        if self.peek() == Some(byte) {
            self.position += 1;
            true
        } else {
            false
        }
    }

    fn is_empty(&self) -> bool {
        self.position >= self.bytes.len()
    }
}

/// Read a zone abbreviation, bare or `<>`-quoted.
fn parse_abbreviation(cursor: &mut Cursor<'_>) -> TzResult<Abbreviation> {
    let mut bytes = [0u8; MAX_ABBREVIATION_LEN];
    let mut length = 0usize;
    if cursor.eat(b'<') {
        loop {
            let byte = cursor.bump().ok_or(TzError::UnexpectedEnd)?;
            if byte == b'>' {
                break;
            }
            if !byte.is_ascii_alphanumeric() && byte != b'+' && byte != b'-' {
                return Err(TzError::MalformedAbbreviation);
            }
            *bytes
                .get_mut(length)
                .ok_or(TzError::MalformedAbbreviation)? = byte;
            length += 1;
        }
    } else {
        while let Some(byte) = cursor.peek() {
            if !byte.is_ascii_alphabetic() {
                break;
            }
            *bytes
                .get_mut(length)
                .ok_or(TzError::MalformedAbbreviation)? = byte;
            length += 1;
            cursor.bump();
        }
    }
    // POSIX requires at least three characters, which keeps a lone sign or a
    // stray digit from being read as a name.
    if length < 3 {
        return Err(TzError::MalformedAbbreviation);
    }
    let text =
        core::str::from_utf8(&bytes[..length]).map_err(|_| TzError::MalformedAbbreviation)?;
    Abbreviation::new(text)
}

/// Read a run of ASCII digits as a number, at most `max_digits` of them.
fn parse_number(cursor: &mut Cursor<'_>, max_digits: usize) -> TzResult<i32> {
    let mut value: i32 = 0;
    let mut digits = 0usize;
    while let Some(byte) = cursor.peek() {
        if !byte.is_ascii_digit() || digits == max_digits {
            break;
        }
        value = value * 10 + i32::from(byte - b'0');
        digits += 1;
        cursor.bump();
    }
    if digits == 0 {
        return Err(TzError::MalformedOffset);
    }
    Ok(value)
}

/// Read `[+-]hh[:mm[:ss]]` and return it as seconds, sign as written.
fn parse_hms(cursor: &mut Cursor<'_>) -> TzResult<(bool, i32)> {
    let negative = match cursor.peek() {
        Some(b'+') => {
            cursor.bump();
            false
        }
        Some(b'-') => {
            cursor.bump();
            true
        }
        _ => false,
    };
    let hours = parse_number(cursor, 3)?;
    let mut minutes = 0;
    let mut seconds = 0;
    if cursor.eat(b':') {
        minutes = parse_number(cursor, 2)?;
        if minutes > 59 {
            return Err(TzError::MalformedOffset);
        }
        if cursor.eat(b':') {
            seconds = parse_number(cursor, 2)?;
            if seconds > 59 {
                return Err(TzError::MalformedOffset);
            }
        }
    }
    Ok((negative, hours * 3_600 + minutes * 60 + seconds))
}

/// Read a `TZ` offset, converting POSIX's west-positive sign to this crate's
/// east-positive one.
fn parse_posix_offset(cursor: &mut Cursor<'_>) -> TzResult<UtcOffset> {
    let (negative, magnitude) = parse_hms(cursor)?;
    UtcOffset::from_seconds(if negative { magnitude } else { -magnitude })
}

/// Read a transition rule and its optional `/time`.
fn parse_transition(cursor: &mut Cursor<'_>) -> TzResult<PosixTransition> {
    let rule = parse_rule(cursor)?;
    if cursor.eat(b'/') {
        let (negative, magnitude) = parse_hms(cursor).map_err(|_| TzError::MalformedRule)?;
        let time = if negative { -magnitude } else { magnitude };
        return PosixTransition::at(rule, time);
    }
    Ok(PosixTransition::new(rule))
}

/// Read one of the three rule forms.
fn parse_rule(cursor: &mut Cursor<'_>) -> TzResult<PosixRule> {
    match cursor.peek() {
        Some(b'J') => {
            cursor.bump();
            let day = parse_number(cursor, 3).map_err(|_| TzError::MalformedRule)?;
            if !(1..=365).contains(&day) {
                return Err(TzError::MalformedRule);
            }
            Ok(PosixRule::JulianNoLeap(day as u16))
        }
        Some(b'M') => {
            cursor.bump();
            let month = parse_number(cursor, 2).map_err(|_| TzError::MalformedRule)?;
            if !cursor.eat(b'.') {
                return Err(TzError::MalformedRule);
            }
            let week = parse_number(cursor, 1).map_err(|_| TzError::MalformedRule)?;
            if !cursor.eat(b'.') {
                return Err(TzError::MalformedRule);
            }
            let day = parse_number(cursor, 1).map_err(|_| TzError::MalformedRule)?;
            if !(1..=12).contains(&month) || !(1..=5).contains(&week) || !(0..=6).contains(&day) {
                return Err(TzError::MalformedRule);
            }
            Ok(PosixRule::MonthWeekDay {
                month: month as u8,
                week: week as u8,
                day: day as u8,
            })
        }
        Some(byte) if byte.is_ascii_digit() => {
            let day = parse_number(cursor, 3).map_err(|_| TzError::MalformedRule)?;
            if day > 365 {
                return Err(TzError::MalformedRule);
            }
            Ok(PosixRule::ZeroBasedDay(day as u16))
        }
        _ => Err(TzError::MalformedRule),
    }
}

/// Write an offset in POSIX's west-positive convention.
fn write_posix_offset(f: &mut fmt::Formatter<'_>, offset: UtcOffset) -> fmt::Result {
    write_signed_hms(f, -offset.seconds())
}

/// Write `[-]h[:mm[:ss]]`, omitting the parts POSIX lets a reader infer.
fn write_signed_hms(f: &mut fmt::Formatter<'_>, seconds: i32) -> fmt::Result {
    if seconds < 0 {
        f.write_str("-")?;
    }
    let magnitude = seconds.unsigned_abs();
    write!(f, "{}", magnitude / 3_600)?;
    let minutes = (magnitude % 3_600) / 60;
    let remainder = magnitude % 60;
    if minutes != 0 || remainder != 0 {
        write!(f, ":{minutes:02}")?;
    }
    if remainder != 0 {
        write!(f, ":{remainder:02}")?;
    }
    Ok(())
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

    fn instant(year: i64, month: u8, day: u8, hour: u8, minute: u8) -> UnixTime {
        let days = rd_from_ymd(year, month, day) - RD_OF_UNIX_EPOCH;
        UnixTime::from_seconds(days * 86_400 + i64::from(hour) * 3_600 + i64::from(minute) * 60)
    }

    #[test]
    fn japan_has_a_standard_offset_and_no_saving_period() {
        let tz = PosixTz::parse("JST-9").unwrap();
        assert_eq!(tz.standard_abbreviation().as_str(), "JST");
        assert_eq!(tz.standard_offset().seconds(), 9 * 3_600);
        assert!(tz.dst().is_none());
        assert_eq!(tz.transitions_in_year(2024), None);
        for month in 1..=12u8 {
            let when = instant(2024, month, 15, 12, 0);
            assert_eq!(tz.offset_at(when).seconds(), 9 * 3_600);
            assert!(!tz.is_dst_at(when));
            assert_eq!(tz.abbreviation_at(when), "JST");
        }
        assert_eq!(tz.to_string(), "JST-9");
    }

    #[test]
    fn the_eastern_united_states_switches_on_the_published_dates() {
        let tz = PosixTz::parse("EST5EDT,M3.2.0,M11.1.0").unwrap();
        assert_eq!(tz.standard_offset().seconds(), -5 * 3_600);
        let dst = tz.dst().unwrap();
        assert_eq!(dst.offset.seconds(), -4 * 3_600);
        assert_eq!(dst.abbreviation.as_str(), "EDT");
        // 2024: forward on 10 March at 02:00 EST, back on 3 November at
        // 02:00 EDT.
        let (start, end) = tz.transitions_in_year(2024).unwrap();
        assert_eq!(start, 1_710_054_000);
        assert_eq!(end, 1_730_613_600);
        assert_eq!(
            tz.offset_at(UnixTime::from_seconds(start - 1)).seconds(),
            -5 * 3_600
        );
        assert_eq!(
            tz.offset_at(UnixTime::from_seconds(start)).seconds(),
            -4 * 3_600
        );
        assert_eq!(
            tz.offset_at(UnixTime::from_seconds(end - 1)).seconds(),
            -4 * 3_600
        );
        assert_eq!(
            tz.offset_at(UnixTime::from_seconds(end)).seconds(),
            -5 * 3_600
        );
        assert_eq!(tz.abbreviation_at(UnixTime::from_seconds(start)), "EDT");
    }

    #[test]
    fn the_2007_rule_change_moved_the_spring_transition_from_april_to_march() {
        // The Energy Policy Act of 2005 moved the United States transitions,
        // effective 2007: spring from the first Sunday in April to the second
        // Sunday in March, autumn from the last Sunday in October to the
        // first Sunday in November.
        let before = PosixTz::parse("EST5EDT,M4.1.0,M10.5.0").unwrap();
        let after = PosixTz::parse("EST5EDT,M3.2.0,M11.1.0").unwrap();

        // 2007-03-15 12:00 UTC falls between the two spring dates.
        let march = instant(2007, 3, 15, 12, 0);
        assert!(!before.is_dst_at(march));
        assert!(after.is_dst_at(march));

        // 2007-10-30 12:00 UTC falls between the two autumn dates.
        let october = instant(2007, 10, 30, 12, 0);
        assert!(!before.is_dst_at(october));
        assert!(after.is_dst_at(october));

        // Both agree in the middle of the summer and of the winter.
        assert!(before.is_dst_at(instant(2007, 7, 1, 12, 0)));
        assert!(after.is_dst_at(instant(2007, 7, 1, 12, 0)));
        assert!(!before.is_dst_at(instant(2007, 1, 1, 12, 0)));
        assert!(!after.is_dst_at(instant(2007, 1, 1, 12, 0)));
    }

    #[test]
    fn european_rules_change_at_the_same_instant_across_the_union() {
        // Directive 2000/84/EC: the last Sunday in March and October at
        // 01:00 UTC everywhere in the Union.
        let paris = PosixTz::parse("CET-1CEST,M3.5.0,M10.5.0/3").unwrap();
        let london = PosixTz::parse("GMT0BST,M3.5.0/1,M10.5.0").unwrap();
        let (paris_start, paris_end) = paris.transitions_in_year(2024).unwrap();
        let (london_start, london_end) = london.transitions_in_year(2024).unwrap();
        assert_eq!(paris_start, london_start);
        assert_eq!(paris_end, london_end);
        // 2024-03-31T01:00:00Z and 2024-10-27T01:00:00Z.
        assert_eq!(paris_start, 1_711_846_800);
        assert_eq!(paris_end, 1_729_990_800);
    }

    #[test]
    fn southern_hemisphere_saving_time_straddles_the_new_year() {
        let sydney = PosixTz::parse("AEST-10AEDT,M10.1.0,M4.1.0/3").unwrap();
        assert!(sydney.is_dst_at(instant(2024, 1, 15, 0, 0)));
        assert!(!sydney.is_dst_at(instant(2024, 7, 15, 0, 0)));
        assert!(sydney.is_dst_at(instant(2024, 12, 15, 0, 0)));
        // Saving time starts on the first Sunday in October at 02:00 AEST and
        // ends on the first Sunday in April at 03:00 AEDT.
        let (start, end) = sydney.transitions_in_year(2024).unwrap();
        assert!(end < start, "the end of the period precedes its start");
        // 2024-04-06T16:00:00Z is 2024-04-07T03:00 AEDT.
        assert_eq!(end, 1_712_419_200);
        // 2024-10-05T16:00:00Z is 2024-10-06T02:00 AEST.
        assert_eq!(start, 1_728_144_000);
    }

    #[test]
    fn new_zealand_and_australia_disagree_by_two_hours_in_january() {
        let auckland = PosixTz::parse("NZST-12NZDT,M9.5.0,M4.1.0/3").unwrap();
        let sydney = PosixTz::parse("AEST-10AEDT,M10.1.0,M4.1.0/3").unwrap();
        let summer = instant(2024, 1, 15, 0, 0);
        assert_eq!(auckland.offset_at(summer).seconds(), 13 * 3_600);
        assert_eq!(sydney.offset_at(summer).seconds(), 11 * 3_600);
    }

    #[test]
    fn brazil_before_and_after_it_abandoned_daylight_saving() {
        let old = PosixTz::parse("<-03>3<-02>,M10.3.0/0,M2.3.0/0").unwrap();
        let now = PosixTz::parse("<-03>3").unwrap();
        assert_eq!(old.standard_offset().seconds(), -3 * 3_600);
        assert_eq!(old.dst().unwrap().offset.seconds(), -2 * 3_600);
        assert!(old.is_dst_at(instant(2018, 1, 15, 12, 0)));
        assert!(!now.is_dst_at(instant(2018, 1, 15, 12, 0)));
        assert_eq!(old.abbreviation_at(instant(2018, 1, 15, 12, 0)), "-02");
        assert_eq!(now.abbreviation_at(instant(2018, 1, 15, 12, 0)), "-03");
        assert_eq!(old.to_string(), "<-03>3<-02>,M10.3.0/0,M2.3.0/0");
        assert_eq!(now.to_string(), "<-03>3");
    }

    #[test]
    fn egypt_ends_its_saving_period_at_the_last_instant_of_a_thursday() {
        // Africa/Cairo: last Friday in April at 00:00, last Thursday in
        // October at 24:00.
        let cairo = PosixTz::parse("EET-2EEST,M4.5.5/0,M10.5.4/24").unwrap();
        let dst = cairo.dst().unwrap();
        assert_eq!(dst.start.time, 0);
        assert_eq!(dst.end.time, 86_400);
        let (start, end) = cairo.transitions_in_year(2024).unwrap();
        // 2024-04-25T22:00:00Z is 2024-04-26T00:00 EET.
        assert_eq!(start, 1_714_082_400);
        // 2024-10-31T21:00:00Z is 2024-10-31T24:00 EEST.
        assert_eq!(end, 1_730_408_400);
        assert_eq!(cairo.to_string(), "EET-2EEST,M4.5.5/0,M10.5.4/24");
    }

    #[test]
    fn kathmandu_keeps_a_quarter_hour_offset() {
        let tz = PosixTz::parse("<+0545>-5:45").unwrap();
        assert_eq!(tz.standard_offset().seconds(), 5 * 3_600 + 45 * 60);
        assert_eq!(tz.abbreviation_at(UnixTime::EPOCH), "+0545");
        assert_eq!(tz.to_string(), "<+0545>-5:45");
    }

    #[test]
    fn lord_howe_island_changes_its_clocks_by_half_an_hour() {
        let tz = PosixTz::parse("<+1030>-10:30<+11>-11,M10.1.0,M4.1.0").unwrap();
        assert_eq!(tz.standard_offset().seconds(), 10 * 3_600 + 30 * 60);
        let dst = tz.dst().unwrap();
        assert_eq!(dst.offset.seconds(), 11 * 3_600);
        assert_eq!(dst.offset.seconds() - tz.standard_offset().seconds(), 1_800);
        assert!(tz.is_dst_at(instant(2024, 1, 15, 0, 0)));
        assert!(!tz.is_dst_at(instant(2024, 7, 15, 0, 0)));
        // The half-hour shift must survive rendering, since it is not the
        // one-hour default POSIX would otherwise infer.
        assert_eq!(tz.to_string(), "<+1030>-10:30<+11>-11,M10.1.0,M4.1.0");
    }

    #[test]
    fn india_keeps_a_half_hour_offset_and_renders_it() {
        let tz = PosixTz::parse("IST-5:30").unwrap();
        assert_eq!(tz.standard_offset().seconds(), 5 * 3_600 + 30 * 60);
        assert_eq!(tz.to_string(), "IST-5:30");
    }

    #[test]
    fn julian_rules_never_count_the_twenty_ninth_of_february() {
        // J60 is 1 March in every year.
        let rule = PosixRule::JulianNoLeap(60);
        assert_eq!(rule.fixed_day(2023), rd_from_ymd(2023, 3, 1));
        assert_eq!(rule.fixed_day(2024), rd_from_ymd(2024, 3, 1));
        assert_eq!(
            PosixRule::JulianNoLeap(1).fixed_day(2024),
            rd_from_ymd(2024, 1, 1)
        );
        assert_eq!(
            PosixRule::JulianNoLeap(365).fixed_day(2024),
            rd_from_ymd(2024, 12, 31)
        );
        assert_eq!(
            PosixRule::JulianNoLeap(365).fixed_day(2023),
            rd_from_ymd(2023, 12, 31)
        );
    }

    #[test]
    fn zero_based_rules_do_count_the_twenty_ninth_of_february() {
        // n59 is 29 February in a leap year and 1 March otherwise.
        let rule = PosixRule::ZeroBasedDay(59);
        assert_eq!(rule.fixed_day(2024), rd_from_ymd(2024, 2, 29));
        assert_eq!(rule.fixed_day(2023), rd_from_ymd(2023, 3, 1));
        assert_eq!(
            PosixRule::ZeroBasedDay(0).fixed_day(2024),
            rd_from_ymd(2024, 1, 1)
        );
        assert_eq!(
            PosixRule::ZeroBasedDay(365).fixed_day(2024),
            rd_from_ymd(2024, 12, 31)
        );
    }

    #[test]
    fn week_five_means_the_last_one_whether_or_not_there_are_five() {
        let last_sunday_in_march = PosixRule::MonthWeekDay {
            month: 3,
            week: 5,
            day: 0,
        };
        // March 2024 has five Sundays; March 2021 has four.
        assert_eq!(
            last_sunday_in_march.fixed_day(2024),
            rd_from_ymd(2024, 3, 31)
        );
        assert_eq!(
            last_sunday_in_march.fixed_day(2021),
            rd_from_ymd(2021, 3, 28)
        );
        let last_thursday_in_october = PosixRule::MonthWeekDay {
            month: 10,
            week: 5,
            day: 4,
        };
        assert_eq!(
            last_thursday_in_october.fixed_day(2024),
            rd_from_ymd(2024, 10, 31)
        );
    }

    #[test]
    fn month_week_day_rules_always_land_inside_their_month() {
        for year in 2000..2100i64 {
            for month in 1..=12u8 {
                for week in 1..=5u8 {
                    for day in 0..=6u8 {
                        let rule = PosixRule::MonthWeekDay { month, week, day };
                        let chosen = rule.fixed_day(year);
                        let first = rd_from_ymd(year, month, 1);
                        let last = first + i64::from(gregorian::days_in_month(year, month)) - 1;
                        assert!(
                            chosen >= first && chosen <= last,
                            "{year}-{month} {week}.{day}"
                        );
                        assert_eq!(gregorian::weekday_from_rd(chosen), day);
                    }
                }
            }
        }
    }

    #[test]
    fn a_daylight_name_without_rules_falls_back_to_the_current_us_rules() {
        let implied = PosixTz::parse("EST5EDT").unwrap();
        let explicit = PosixTz::parse("EST5EDT,M3.2.0,M11.1.0").unwrap();
        assert_eq!(implied, explicit);
        assert_eq!(implied.to_string(), "EST5EDT,M3.2.0,M11.1.0");
    }

    #[test]
    fn an_omitted_daylight_offset_means_one_hour_ahead() {
        let tz = PosixTz::parse("CET-1CEST,M3.5.0,M10.5.0/3").unwrap();
        assert_eq!(tz.dst().unwrap().offset.seconds(), 2 * 3_600);
        assert_eq!(tz.to_string(), "CET-1CEST,M3.5.0,M10.5.0/3");
    }

    #[test]
    fn rule_times_may_be_negative_or_past_midnight() {
        let tz = PosixTz::parse("XST-3XDT,M10.1.0/24,M3.1.0/-2").unwrap();
        assert_eq!(tz.dst().unwrap().start.time, 86_400);
        assert_eq!(tz.dst().unwrap().end.time, -7_200);
        assert_eq!(tz.to_string(), "XST-3XDT,M10.1.0/24,M3.1.0/-2");
        assert_eq!(
            PosixTransition::at(PosixRule::ZeroBasedDay(0), 168 * 3_600),
            Err(TzError::MalformedRule)
        );
    }

    #[test]
    fn malformed_tz_strings_are_refused_with_a_reason() {
        assert_eq!(PosixTz::parse(""), Err(TzError::UnexpectedEnd));
        assert_eq!(PosixTz::parse("AB-9"), Err(TzError::MalformedAbbreviation));
        assert_eq!(PosixTz::parse("JST"), Err(TzError::MalformedOffset));
        assert_eq!(
            PosixTz::parse("EST5EDT,M3.2.0"),
            Err(TzError::MalformedRule)
        );
        assert_eq!(
            PosixTz::parse("EST5EDT,M13.2.0,M11.1.0"),
            Err(TzError::MalformedRule)
        );
        assert_eq!(
            PosixTz::parse("EST5EDT,M3.6.0,M11.1.0"),
            Err(TzError::MalformedRule)
        );
        assert_eq!(
            PosixTz::parse("EST5EDT,M3.2.7,M11.1.0"),
            Err(TzError::MalformedRule)
        );
        assert_eq!(
            PosixTz::parse("EST5EDT,J0,M11.1.0"),
            Err(TzError::MalformedRule)
        );
        assert_eq!(
            PosixTz::parse("EST5EDT,J366,M11.1.0"),
            Err(TzError::MalformedRule)
        );
        assert_eq!(
            PosixTz::parse("EST5EDT,366,M11.1.0"),
            Err(TzError::MalformedRule)
        );
        assert_eq!(PosixTz::parse("JST-26"), Err(TzError::OffsetOutOfRange));
        assert_eq!(PosixTz::parse("<JST-9"), Err(TzError::UnexpectedEnd));
        assert_eq!(
            PosixTz::parse("EST5EDT,M3.2.0,M11.1.0,J1"),
            Err(TzError::TrailingInput)
        );
    }

    #[test]
    fn a_rule_can_be_read_on_its_own() {
        assert_eq!(
            PosixRule::parse("M3.2.0").unwrap(),
            PosixRule::MonthWeekDay {
                month: 3,
                week: 2,
                day: 0
            }
        );
        assert_eq!(
            PosixRule::parse("J60").unwrap(),
            PosixRule::JulianNoLeap(60)
        );
        assert_eq!(PosixRule::parse("59").unwrap(), PosixRule::ZeroBasedDay(59));
        assert_eq!(PosixRule::parse("M3.2.0/2"), Err(TzError::TrailingInput));
        assert_eq!(PosixRule::parse("Q1"), Err(TzError::MalformedRule));
        assert_eq!(PosixRule::parse("M3.2.0").unwrap().to_string(), "M3.2.0");
        assert_eq!(
            PosixTransition::new(PosixRule::JulianNoLeap(60)).to_string(),
            "J60"
        );
    }

    #[test]
    fn abbreviations_are_quoted_only_when_they_have_to_be() {
        assert!(!Abbreviation::new("JST").unwrap().needs_quoting());
        assert!(Abbreviation::new("+0545").unwrap().needs_quoting());
        assert!(Abbreviation::new("-03").unwrap().needs_quoting());
        assert_eq!(Abbreviation::new("-03").unwrap().to_string(), "<-03>");
        assert_eq!(Abbreviation::new("JST").unwrap().to_string(), "JST");
        assert_eq!(Abbreviation::new(""), Err(TzError::MalformedAbbreviation));
        assert_eq!(
            Abbreviation::new("ABCDEFGHIJKLMNOPQ"),
            Err(TzError::MalformedAbbreviation)
        );
        assert_eq!(
            Abbreviation::new("A/B"),
            Err(TzError::MalformedAbbreviation)
        );
    }

    #[test]
    fn every_documented_tz_string_survives_a_render_and_reparse() {
        for text in [
            "JST-9",
            "UTC0",
            "EST5EDT,M3.2.0,M11.1.0",
            "PST8PDT,M3.2.0,M11.1.0",
            "GMT0BST,M3.5.0/1,M10.5.0",
            "CET-1CEST,M3.5.0,M10.5.0/3",
            "MSK-3",
            "IST-5:30",
            "AEST-10AEDT,M10.1.0,M4.1.0/3",
            "<-03>3",
            "EET-2EEST,M4.5.5/0,M10.5.4/24",
            "NZST-12NZDT,M9.5.0,M4.1.0/3",
            "<+0545>-5:45",
            "<+1030>-10:30<+11>-11,M10.1.0,M4.1.0",
            "EST5EDT,J60,J300",
            "XYZ-1:23:45",
        ] {
            let parsed = PosixTz::parse(text).unwrap();
            assert_eq!(parsed.to_string(), text, "rendering {text}");
            assert_eq!(PosixTz::parse(&parsed.to_string()).unwrap(), parsed);
        }
    }

    #[test]
    fn the_repeated_hour_at_the_end_of_american_saving_time_is_ambiguous() {
        let tz = PosixTimeZone::parse("America/New_York", "EST5EDT,M3.2.0,M11.1.0").unwrap();
        let resolution = tz.resolve_local(civil(2024, 11, 3, 1, 30, 0));
        assert!(resolution.is_ambiguous());
        assert_eq!(resolution.earliest().seconds(), 1_730_611_800);
        assert_eq!(resolution.latest().seconds(), 1_730_615_400);
        assert_eq!(
            tz.unix_at(civil(2024, 11, 3, 1, 30, 0), Disambiguation::Reject),
            Err(TzError::AmbiguousLocalTime)
        );
    }

    #[test]
    fn the_skipped_hour_at_the_start_of_american_saving_time_does_not_exist() {
        let tz = PosixTimeZone::parse("America/New_York", "EST5EDT,M3.2.0,M11.1.0").unwrap();
        let resolution = tz.resolve_local(civil(2024, 3, 10, 2, 30, 0));
        match resolution {
            LocalResolution::Nonexistent {
                gap_start,
                gap_end,
                transition,
                ..
            } => {
                assert_eq!(gap_start, civil(2024, 3, 10, 2, 0, 0));
                assert_eq!(gap_end, civil(2024, 3, 10, 3, 0, 0));
                assert_eq!(transition.seconds(), 1_710_054_000);
            }
            other => panic!("expected a gap, got {other:?}"),
        }
    }

    #[test]
    fn the_european_gap_and_overlap_are_an_hour_long_too() {
        let tz = PosixTimeZone::parse("Europe/Paris", "CET-1CEST,M3.5.0,M10.5.0/3").unwrap();
        // 2024-03-31 02:30 local does not exist in Paris.
        assert!(
            tz.resolve_local(civil(2024, 3, 31, 2, 30, 0))
                .is_nonexistent()
        );
        // 2024-10-27 02:30 local happens twice.
        let overlap = tz.resolve_local(civil(2024, 10, 27, 2, 30, 0));
        assert!(overlap.is_ambiguous());
        assert_eq!(
            overlap.latest().seconds() - overlap.earliest().seconds(),
            3_600
        );
    }

    #[test]
    fn the_southern_gap_and_overlap_fall_in_october_and_april() {
        let tz = PosixTimeZone::parse("Australia/Sydney", "AEST-10AEDT,M10.1.0,M4.1.0/3").unwrap();
        // Clocks go forward on the first Sunday in October at 02:00.
        assert!(
            tz.resolve_local(civil(2024, 10, 6, 2, 30, 0))
                .is_nonexistent()
        );
        // And back on the first Sunday in April at 03:00, repeating 02:00-03:00.
        assert!(tz.resolve_local(civil(2024, 4, 7, 2, 30, 0)).is_ambiguous());
        // The reverse pairing must not resolve: April has no gap, October no
        // overlap.
        assert!(
            tz.resolve_local(civil(2024, 4, 7, 4, 30, 0))
                .is_unambiguous()
        );
        assert!(
            tz.resolve_local(civil(2024, 10, 6, 4, 30, 0))
                .is_unambiguous()
        );
    }

    #[test]
    fn every_hour_of_a_year_in_new_york_resolves_back_to_its_own_instant() {
        let tz = PosixTimeZone::parse("America/New_York", "EST5EDT,M3.2.0,M11.1.0").unwrap();
        for seconds in (1_704_067_200..1_735_689_600).step_by(3_600) {
            let when = UnixTime::from_seconds(seconds);
            let local = tz.local_at(when).unwrap();
            let resolution = tz.resolve_local(local);
            assert!(
                resolution.earliest() <= when && when <= resolution.latest(),
                "{seconds}"
            );
        }
    }

    #[test]
    fn every_hour_of_a_year_in_sydney_resolves_back_to_its_own_instant() {
        let tz = PosixTimeZone::parse("Australia/Sydney", "AEST-10AEDT,M10.1.0,M4.1.0/3").unwrap();
        for seconds in (1_704_067_200..1_735_689_600).step_by(3_600) {
            let when = UnixTime::from_seconds(seconds);
            let local = tz.local_at(when).unwrap();
            let resolution = tz.resolve_local(local);
            assert!(
                resolution.earliest() <= when && when <= resolution.latest(),
                "{seconds}"
            );
        }
    }

    #[test]
    fn a_zone_without_saving_time_resolves_every_reading_exactly_once() {
        let tz = PosixTimeZone::parse("Asia/Tokyo", "JST-9").unwrap();
        for day in 719_000..719_400 {
            for hour in [0u8, 1, 2, 3, 23] {
                let local = CivilDateTime::new(Rd(day), CivilTime::hms(hour, 30, 0).unwrap());
                let resolution = tz.resolve_local(local);
                assert!(resolution.is_unambiguous(), "{day} {hour}");
                assert_eq!(tz.local_at(resolution.earliest()).unwrap(), local);
            }
        }
    }

    #[test]
    fn offsets_hold_steady_between_transitions_across_a_century() {
        // Evaluating the rules year by year must never produce a stray
        // transition: exactly two per year, in the expected months.
        let tz = PosixTz::parse("EST5EDT,M3.2.0,M11.1.0").unwrap();
        for year in 1970..2070i64 {
            let (start, end) = tz.transitions_in_year(year).unwrap();
            assert!(start < end, "{year}");
            assert!(!tz.is_dst_at(UnixTime::from_seconds(start - 1)), "{year}");
            assert!(tz.is_dst_at(UnixTime::from_seconds(start)), "{year}");
            assert!(tz.is_dst_at(UnixTime::from_seconds(end - 1)), "{year}");
            assert!(!tz.is_dst_at(UnixTime::from_seconds(end)), "{year}");
        }
    }
}
