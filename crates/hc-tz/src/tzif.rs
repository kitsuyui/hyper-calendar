//! A reader for the binary TZif format, RFC 8536.
//!
//! TZif is what `/usr/share/zoneinfo/Asia/Tokyo` and every other file in the
//! IANA database is. Unlike a POSIX `TZ` string it records *history*: every
//! transition the zone has ever made, each with its own offset, abbreviation
//! and daylight flag, plus — from version 2 on — a POSIX string in a footer
//! saying what happens after the last recorded transition.
//!
//! # Shape of the format
//!
//! ```text
//! header (44 bytes)           magic "TZif", version, six counts
//! data block                  transition times, their type indices,
//!                             local time type records, abbreviation
//!                             strings, leap-second records, and the
//!                             standard/wall and UT/local indicators
//! [version 2 and later]       the whole thing again with 64-bit times,
//!                             then "\n" TZ-string "\n"
//! ```
//!
//! A version 2 or 3 file carries the 32-bit block only so that a version 1
//! reader can still use it. [`TzifData::parse`] reads the 64-bit block when
//! there is one, which is what every implementation since 1994 does.
//!
//! # Borrowing, not allocating
//!
//! [`TzifData`] holds sub-slices of the bytes it was given and decodes fields
//! on demand, so parsing allocates nothing and works in `no_std`. Reading the
//! bytes off disk is a separate, `std`-only concern in [`crate::system`]: a
//! parser over a slice can be tested against a fixture, and a loader cannot.

use hc_calendar::CivilDateTime;
use hc_core::UnixTime;

use crate::error::{TzError, TzResult};
use crate::offset::{MAX_OFFSET_SECONDS, UtcOffset};
use crate::posix::PosixTz;
use crate::zone::{LocalResolution, TimeZone, resolve_local_by_probing};

/// The four bytes every TZif file starts with.
const MAGIC: [u8; 4] = *b"TZif";

/// The fixed size of a TZif header.
const HEADER_LEN: usize = 44;

/// The size of one local time type record: `utoff`, `isdst`, `desigidx`.
const TYPE_RECORD_LEN: usize = 6;

/// Which version of the format a file claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TzifVersion {
    /// Version 1: 32-bit transition times, no footer. Written by nothing
    /// modern, but still the first block of every later file.
    V1,
    /// Version 2: adds the 64-bit block and the POSIX footer.
    V2,
    /// Version 3: as version 2, but the footer may use transition times
    /// outside `00:00..=24:00`.
    V3,
}

impl TzifVersion {
    /// Read the version byte.
    ///
    /// # Errors
    ///
    /// Returns [`TzError::UnsupportedTzifVersion`] for anything but `\0`, `2`
    /// and `3`.
    pub const fn from_byte(byte: u8) -> TzResult<Self> {
        match byte {
            0 => Ok(Self::V1),
            b'2' => Ok(Self::V2),
            b'3' => Ok(Self::V3),
            _ => Err(TzError::UnsupportedTzifVersion),
        }
    }

    /// The width in bytes of a transition time in this version's data block.
    #[must_use]
    pub const fn time_size(self) -> usize {
        match self {
            Self::V1 => 4,
            _ => 8,
        }
    }
}

/// The six counts in a TZif header, and the version byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TzifHeader {
    /// The format version.
    pub version: TzifVersion,
    /// How many UT/local indicators follow the data: 0 or `type_count`.
    pub ut_indicator_count: usize,
    /// How many standard/wall indicators follow: 0 or `type_count`.
    pub standard_indicator_count: usize,
    /// How many leap-second records the file carries.
    pub leap_second_count: usize,
    /// How many transitions the file records.
    pub transition_count: usize,
    /// How many local time types it defines.
    pub type_count: usize,
    /// How many bytes of abbreviation strings it holds.
    pub designation_len: usize,
}

impl TzifHeader {
    /// Read a header from the start of a slice.
    ///
    /// # Errors
    ///
    /// Returns [`TzError::TruncatedTzifData`] for a short slice,
    /// [`TzError::NotTzifData`] for a wrong magic number,
    /// [`TzError::UnsupportedTzifVersion`] for an unknown version and
    /// [`TzError::MalformedTzifData`] when the counts contradict RFC 8536
    /// §3.1.
    pub fn parse(bytes: &[u8]) -> TzResult<Self> {
        let header = bytes.get(..HEADER_LEN).ok_or(TzError::TruncatedTzifData)?;
        if header[..4] != MAGIC {
            return Err(TzError::NotTzifData);
        }
        let version = TzifVersion::from_byte(header[4])?;
        let count = |index: usize| -> usize {
            let start = 20 + index * 4;
            u32::from_be_bytes([
                header[start],
                header[start + 1],
                header[start + 2],
                header[start + 3],
            ]) as usize
        };
        let parsed = Self {
            version,
            ut_indicator_count: count(0),
            standard_indicator_count: count(1),
            leap_second_count: count(2),
            transition_count: count(3),
            type_count: count(4),
            designation_len: count(5),
        };
        // RFC 8536 §3.1: there must be at least one local time type and at
        // least one byte of designations, and the two indicator arrays are
        // either absent or one entry per type.
        if parsed.type_count == 0 || parsed.designation_len == 0 {
            return Err(TzError::MalformedTzifData);
        }
        if parsed.ut_indicator_count != 0 && parsed.ut_indicator_count != parsed.type_count {
            return Err(TzError::MalformedTzifData);
        }
        if parsed.standard_indicator_count != 0
            && parsed.standard_indicator_count != parsed.type_count
        {
            return Err(TzError::MalformedTzifData);
        }
        Ok(parsed)
    }

    /// How many bytes of data block this header describes.
    #[must_use]
    pub const fn data_len(&self, time_size: usize) -> usize {
        self.transition_count * (time_size + 1)
            + self.type_count * TYPE_RECORD_LEN
            + self.designation_len
            + self.leap_second_count * (time_size + 4)
            + self.standard_indicator_count
            + self.ut_indicator_count
    }
}

/// One local time type: an offset, a daylight flag and an abbreviation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalTimeType<'a> {
    /// The offset from UTC.
    pub offset: UtcOffset,
    /// Whether the zone considers this its saving period.
    pub is_dst: bool,
    /// The abbreviation, such as `JST` or `-03`.
    pub abbreviation: &'a str,
}

/// One recorded transition: when it happened and which type took effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Transition {
    /// The POSIX timestamp at which the new type took effect.
    pub time: i64,
    /// Which local time type it switched to.
    pub type_index: usize,
}

/// One leap-second record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeapSecond {
    /// The POSIX timestamp at which the correction takes effect.
    pub occurrence: i64,
    /// The cumulative `TAI - UTC` correction from that instant on.
    ///
    /// Only files from the `right/` half of the database carry these; the
    /// ordinary `posix/` files have none, because POSIX time has no room for
    /// a leap second. [`hc_core::leap`] is the authoritative table either way.
    pub correction: i32,
}

/// A parsed TZif file, borrowing the bytes it was parsed from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TzifData<'a> {
    header: TzifHeader,
    time_size: usize,
    transition_times: &'a [u8],
    transition_types: &'a [u8],
    type_records: &'a [u8],
    designations: &'a [u8],
    leap_records: &'a [u8],
    standard_indicators: &'a [u8],
    ut_indicators: &'a [u8],
    footer: Option<&'a str>,
}

impl<'a> TzifData<'a> {
    /// Parse a TZif file, preferring its 64-bit block.
    ///
    /// # Errors
    ///
    /// Returns [`TzError::NotTzifData`], [`TzError::UnsupportedTzifVersion`],
    /// [`TzError::TruncatedTzifData`] or [`TzError::MalformedTzifData`]
    /// according to which invariant the bytes break.
    pub fn parse(bytes: &'a [u8]) -> TzResult<Self> {
        let header = TzifHeader::parse(bytes)?;
        if header.version == TzifVersion::V1 {
            let body = bytes
                .get(HEADER_LEN..HEADER_LEN + header.data_len(4))
                .ok_or(TzError::TruncatedTzifData)?;
            return Self::from_block(header, body, 4, None);
        }

        // Skip the 32-bit block: it exists only for readers that predate
        // version 2, and it cannot represent the whole record.
        let skip = HEADER_LEN
            .checked_add(header.data_len(4))
            .ok_or(TzError::TruncatedTzifData)?;
        let rest = bytes.get(skip..).ok_or(TzError::TruncatedTzifData)?;
        let second = TzifHeader::parse(rest)?;
        if second.version == TzifVersion::V1 {
            return Err(TzError::MalformedTzifData);
        }
        let body_and_footer = rest.get(HEADER_LEN..).ok_or(TzError::TruncatedTzifData)?;
        let body_len = second.data_len(8);
        let body = body_and_footer
            .get(..body_len)
            .ok_or(TzError::TruncatedTzifData)?;
        let footer = parse_footer(
            body_and_footer
                .get(body_len..)
                .ok_or(TzError::TruncatedTzifData)?,
        )?;
        Self::from_block(second, body, 8, footer)
    }

    /// Split a data block into its parts and check the invariants that the
    /// accessors below rely on.
    fn from_block(
        header: TzifHeader,
        body: &'a [u8],
        time_size: usize,
        footer: Option<&'a str>,
    ) -> TzResult<Self> {
        let mut rest = body;
        let mut take = |length: usize| -> TzResult<&'a [u8]> {
            let (taken, remainder) = rest
                .split_at_checked(length)
                .ok_or(TzError::TruncatedTzifData)?;
            rest = remainder;
            Ok(taken)
        };
        let transition_times = take(header.transition_count * time_size)?;
        let transition_types = take(header.transition_count)?;
        let type_records = take(header.type_count * TYPE_RECORD_LEN)?;
        let designations = take(header.designation_len)?;
        let leap_records = take(header.leap_second_count * (time_size + 4))?;
        let standard_indicators = take(header.standard_indicator_count)?;
        let ut_indicators = take(header.ut_indicator_count)?;

        let parsed = Self {
            header,
            time_size,
            transition_times,
            transition_types,
            type_records,
            designations,
            leap_records,
            standard_indicators,
            ut_indicators,
            footer,
        };
        parsed.validate()?;
        Ok(parsed)
    }

    /// Check everything the accessors then take for granted: sorted
    /// transitions, in-range type indices, in-range offsets and
    /// NUL-terminated ASCII designations.
    fn validate(&self) -> TzResult<()> {
        if self.designations.last() != Some(&0) || !self.designations.is_ascii() {
            return Err(TzError::MalformedTzifData);
        }
        for index in 0..self.header.type_count {
            let record = self
                .type_records
                .get(index * TYPE_RECORD_LEN..(index + 1) * TYPE_RECORD_LEN)
                .ok_or(TzError::TruncatedTzifData)?;
            let offset = i32::from_be_bytes([record[0], record[1], record[2], record[3]]);
            if !(-MAX_OFFSET_SECONDS..=MAX_OFFSET_SECONDS).contains(&offset) {
                return Err(TzError::MalformedTzifData);
            }
            if record[4] > 1 {
                return Err(TzError::MalformedTzifData);
            }
            if self.designation_at(record[5] as usize).is_none() {
                return Err(TzError::MalformedTzifData);
            }
        }
        let mut previous: Option<i64> = None;
        for index in 0..self.header.transition_count {
            let time = self
                .transition_time(index)
                .ok_or(TzError::TruncatedTzifData)?;
            if previous.is_some_and(|earlier| earlier >= time) {
                return Err(TzError::MalformedTzifData);
            }
            previous = Some(time);
            let type_index = *self
                .transition_types
                .get(index)
                .ok_or(TzError::TruncatedTzifData)?;
            if type_index as usize >= self.header.type_count {
                return Err(TzError::MalformedTzifData);
            }
        }
        Ok(())
    }

    /// The header this file declared.
    #[must_use]
    pub const fn header(&self) -> &TzifHeader {
        &self.header
    }

    /// The format version.
    #[must_use]
    pub const fn version(&self) -> TzifVersion {
        self.header.version
    }

    /// How many transitions the file records.
    #[must_use]
    pub const fn transition_count(&self) -> usize {
        self.header.transition_count
    }

    /// The time of one transition, as a POSIX timestamp.
    #[must_use]
    pub fn transition_time(&self, index: usize) -> Option<i64> {
        let start = index.checked_mul(self.time_size)?;
        let bytes = self.transition_times.get(start..start + self.time_size)?;
        Some(match bytes {
            [a, b, c, d] => i64::from(i32::from_be_bytes([*a, *b, *c, *d])),
            [a, b, c, d, e, f, g, h] => i64::from_be_bytes([*a, *b, *c, *d, *e, *f, *g, *h]),
            _ => return None,
        })
    }

    /// The local time type one transition switches to.
    #[must_use]
    pub fn transition_type_index(&self, index: usize) -> Option<usize> {
        self.transition_types
            .get(index)
            .map(|value| *value as usize)
    }

    /// One transition, time and type together.
    #[must_use]
    pub fn transition(&self, index: usize) -> Option<Transition> {
        Some(Transition {
            time: self.transition_time(index)?,
            type_index: self.transition_type_index(index)?,
        })
    }

    /// Every transition, in order.
    pub fn transitions(&self) -> impl Iterator<Item = Transition> + '_ {
        (0..self.transition_count()).filter_map(|index| self.transition(index))
    }

    /// How many local time types the file defines.
    #[must_use]
    pub const fn local_time_type_count(&self) -> usize {
        self.header.type_count
    }

    /// One local time type.
    #[must_use]
    pub fn local_time_type(&self, index: usize) -> Option<LocalTimeType<'a>> {
        let record = self
            .type_records
            .get(index * TYPE_RECORD_LEN..(index + 1) * TYPE_RECORD_LEN)?;
        let offset = UtcOffset::from_seconds(i32::from_be_bytes([
            record[0], record[1], record[2], record[3],
        ]))
        .ok()?;
        Some(LocalTimeType {
            offset,
            is_dst: record[4] != 0,
            abbreviation: self.designation_at(record[5] as usize)?,
        })
    }

    /// Every local time type, in file order.
    pub fn local_time_types(&self) -> impl Iterator<Item = LocalTimeType<'a>> + '_ {
        (0..self.local_time_type_count()).filter_map(|index| self.local_time_type(index))
    }

    /// The NUL-terminated abbreviation starting at a byte index.
    #[must_use]
    pub fn designation_at(&self, index: usize) -> Option<&'a str> {
        let tail = self.designations.get(index..)?;
        let end = tail.iter().position(|byte| *byte == 0)?;
        core::str::from_utf8(&tail[..end]).ok()
    }

    /// How many leap-second records the file carries.
    #[must_use]
    pub const fn leap_second_count(&self) -> usize {
        self.header.leap_second_count
    }

    /// One leap-second record.
    #[must_use]
    pub fn leap_second(&self, index: usize) -> Option<LeapSecond> {
        let width = self.time_size + 4;
        let record = self.leap_records.get(index * width..(index + 1) * width)?;
        let (time, correction) = record.split_at(self.time_size);
        let occurrence = match time {
            [a, b, c, d] => i64::from(i32::from_be_bytes([*a, *b, *c, *d])),
            [a, b, c, d, e, f, g, h] => i64::from_be_bytes([*a, *b, *c, *d, *e, *f, *g, *h]),
            _ => return None,
        };
        let correction = match correction {
            [a, b, c, d] => i32::from_be_bytes([*a, *b, *c, *d]),
            _ => return None,
        };
        Some(LeapSecond {
            occurrence,
            correction,
        })
    }

    /// Every leap-second record, in order.
    pub fn leap_seconds(&self) -> impl Iterator<Item = LeapSecond> + '_ {
        (0..self.leap_second_count()).filter_map(|index| self.leap_second(index))
    }

    /// Whether a type's transition times were recorded in standard time
    /// rather than wall-clock time. Absent indicators mean wall-clock.
    #[must_use]
    pub fn is_standard_indicator(&self, index: usize) -> Option<bool> {
        self.standard_indicators.get(index).map(|value| *value != 0)
    }

    /// Whether a type's transition times were recorded in UT rather than
    /// local time. Absent indicators mean local.
    #[must_use]
    pub fn is_ut_indicator(&self, index: usize) -> Option<bool> {
        self.ut_indicators.get(index).map(|value| *value != 0)
    }

    /// The POSIX `TZ` string in the footer, if the file has one.
    #[must_use]
    pub const fn footer(&self) -> Option<&'a str> {
        self.footer
    }

    /// The index of the transition in force at an instant, if any transition
    /// is.
    #[must_use]
    pub fn transition_index_at(&self, unix_seconds: i64) -> Option<usize> {
        let count = self.transition_count();
        if count == 0 || self.transition_time(0)? > unix_seconds {
            return None;
        }
        let (mut low, mut high) = (0usize, count - 1);
        while low < high {
            let middle = low + (high - low).div_ceil(2);
            if self.transition_time(middle)? <= unix_seconds {
                low = middle;
            } else {
                high = middle - 1;
            }
        }
        Some(low)
    }

    /// The type that applies before the first recorded transition.
    ///
    /// RFC 8536 §3.2: the first type that is not a daylight-saving one, or
    /// type 0 when every type is.
    #[must_use]
    pub fn first_standard_type_index(&self) -> usize {
        (0..self.local_time_type_count())
            .find(|index| {
                self.local_time_type(*index)
                    .is_some_and(|kind| !kind.is_dst)
            })
            .unwrap_or(0)
    }
}

/// Read the `"\n" TZ-string "\n"` footer.
fn parse_footer(bytes: &[u8]) -> TzResult<Option<&str>> {
    match bytes {
        [] => Ok(None),
        [b'\n', middle @ .., b'\n'] => {
            if middle.contains(&b'\n') {
                return Err(TzError::MalformedTzifData);
            }
            let text = core::str::from_utf8(middle).map_err(|_| TzError::MalformedTzifData)?;
            if text.is_empty() {
                Ok(None)
            } else {
                Ok(Some(text))
            }
        }
        _ => Err(TzError::MalformedTzifData),
    }
}

/// A TZif file presented as a [`TimeZone`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TzifTimeZone<'a> {
    name: &'a str,
    data: TzifData<'a>,
    footer: Option<PosixTz>,
}

/// What governs a given instant: a recorded type, or the footer's rules.
enum Governing<'a> {
    Recorded(LocalTimeType<'a>),
    Footer(&'a PosixTz),
}

impl<'a> TzifTimeZone<'a> {
    /// Parse TZif bytes and give the zone a name.
    ///
    /// # Errors
    ///
    /// See [`TzifData::parse`]; a footer that is not a valid POSIX `TZ`
    /// string is also rejected, with the corresponding [`TzError`].
    pub fn parse(name: &'a str, bytes: &'a [u8]) -> TzResult<Self> {
        let data = TzifData::parse(bytes)?;
        let footer = match data.footer() {
            Some(text) => Some(PosixTz::parse(text)?),
            None => None,
        };
        Ok(Self { name, data, footer })
    }

    /// Build a zone from already-parsed data.
    ///
    /// # Errors
    ///
    /// See [`PosixTz::parse`].
    pub fn from_data(name: &'a str, data: TzifData<'a>) -> TzResult<Self> {
        let footer = match data.footer() {
            Some(text) => Some(PosixTz::parse(text)?),
            None => None,
        };
        Ok(Self { name, data, footer })
    }

    /// The parsed file.
    #[must_use]
    pub const fn data(&self) -> &TzifData<'a> {
        &self.data
    }

    /// The parsed footer rules, if the file had a footer.
    #[must_use]
    pub const fn posix_footer(&self) -> Option<&PosixTz> {
        self.footer.as_ref()
    }

    /// Decide what governs an instant.
    fn governing(&self, utc: UnixTime) -> Governing<'_> {
        let seconds = utc.seconds();
        let count = self.data.transition_count();
        // RFC 8536 §3.3: the footer describes every time at or after the last
        // recorded transition. Without a footer the last type simply persists.
        if let Some(footer) = &self.footer {
            let after_last = match count {
                0 => true,
                _ => self
                    .data
                    .transition_time(count - 1)
                    .is_some_and(|last| seconds >= last),
            };
            if after_last {
                return Governing::Footer(footer);
            }
        }
        let index = match self.data.transition_index_at(seconds) {
            Some(index) => self
                .data
                .transition_type_index(index)
                .unwrap_or_else(|| self.data.first_standard_type_index()),
            None => self.data.first_standard_type_index(),
        };
        // `validate` proved every index resolves, so the fallback is
        // unreachable; it keeps the function total without an `unwrap`.
        Governing::Recorded(self.data.local_time_type(index).unwrap_or(LocalTimeType {
            offset: UtcOffset::UTC,
            is_dst: false,
            abbreviation: "UTC",
        }))
    }
}

impl TimeZone for TzifTimeZone<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn offset_at(&self, utc: UnixTime) -> UtcOffset {
        match self.governing(utc) {
            Governing::Recorded(kind) => kind.offset,
            Governing::Footer(rules) => rules.offset_at(utc),
        }
    }

    fn abbreviation_at(&self, utc: UnixTime) -> Option<&str> {
        Some(match self.governing(utc) {
            Governing::Recorded(kind) => kind.abbreviation,
            Governing::Footer(rules) => rules.abbreviation_at(utc),
        })
    }

    fn is_dst_at(&self, utc: UnixTime) -> bool {
        match self.governing(utc) {
            Governing::Recorded(kind) => kind.is_dst,
            Governing::Footer(rules) => rules.is_dst_at(utc),
        }
    }

    fn resolve_local(&self, local: CivilDateTime) -> LocalResolution {
        resolve_local_by_probing(local, |instant| self.offset_at(instant))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian::rd_from_ymd;
    use crate::zone::Disambiguation;
    use hc_calendar::{CivilTime, Rd};

    /// A handcrafted version 2 file: two local time types (`EST` and `EDT`),
    /// four transitions covering 2024 and 2025 on the United States rules,
    /// one leap-second record, both indicator arrays, and the footer
    /// `EST5EDT,M3.2.0,M11.1.0`.
    ///
    /// Having a fixture in the source means the parser is testable on a
    /// machine with no zone database at all.
    const TZIF_V2_EASTERN: &[u8] = &[
        0x54, 0x5a, 0x69, 0x66, 0x32, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00,
        0x00, 0x01, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x08, 0x65,
        0xed, 0x5a, 0x70, 0x67, 0x27, 0x11, 0x60, 0x67, 0xcd, 0x3c, 0x70, 0x69, 0x06, 0xf3, 0x60,
        0x01, 0x00, 0x01, 0x00, 0xff, 0xff, 0xb9, 0xb0, 0x00, 0x00, 0xff, 0xff, 0xc7, 0xc0, 0x01,
        0x04, 0x45, 0x53, 0x54, 0x00, 0x45, 0x44, 0x54, 0x00, 0x58, 0x68, 0x46, 0x80, 0x00, 0x00,
        0x00, 0x1b, 0x00, 0x00, 0x00, 0x00, 0x54, 0x5a, 0x69, 0x66, 0x32, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
        0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00,
        0x02, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x65, 0xed, 0x5a, 0x70, 0x00, 0x00,
        0x00, 0x00, 0x67, 0x27, 0x11, 0x60, 0x00, 0x00, 0x00, 0x00, 0x67, 0xcd, 0x3c, 0x70, 0x00,
        0x00, 0x00, 0x00, 0x69, 0x06, 0xf3, 0x60, 0x01, 0x00, 0x01, 0x00, 0xff, 0xff, 0xb9, 0xb0,
        0x00, 0x00, 0xff, 0xff, 0xc7, 0xc0, 0x01, 0x04, 0x45, 0x53, 0x54, 0x00, 0x45, 0x44, 0x54,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x58, 0x68, 0x46, 0x80, 0x00, 0x00, 0x00, 0x1b, 0x00, 0x00,
        0x00, 0x00, 0x0a, 0x45, 0x53, 0x54, 0x35, 0x45, 0x44, 0x54, 0x2c, 0x4d, 0x33, 0x2e, 0x32,
        0x2e, 0x30, 0x2c, 0x4d, 0x31, 0x31, 0x2e, 0x31, 0x2e, 0x30, 0x0a,
    ];

    /// The same zone written as a version 1 file: 32-bit times, no leap
    /// records, no indicators and no footer.
    const TZIF_V1_EASTERN: &[u8] = &[
        0x54, 0x5a, 0x69, 0x66, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x08, 0x65,
        0xed, 0x5a, 0x70, 0x67, 0x27, 0x11, 0x60, 0x67, 0xcd, 0x3c, 0x70, 0x69, 0x06, 0xf3, 0x60,
        0x01, 0x00, 0x01, 0x00, 0xff, 0xff, 0xb9, 0xb0, 0x00, 0x00, 0xff, 0xff, 0xc7, 0xc0, 0x01,
        0x04, 0x45, 0x53, 0x54, 0x00, 0x45, 0x44, 0x54, 0x00,
    ];

    /// A version 3 file whose footer uses a `/24` and a negative transition
    /// time — spellings only version 3 admits.
    const TZIF_V3_ODD_FOOTER: &[u8] = &[
        0x54, 0x5a, 0x69, 0x66, 0x33, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x0c, 0x65,
        0x18, 0xb6, 0x80, 0x01, 0xff, 0xff, 0xd5, 0xd0, 0x00, 0x00, 0xff, 0xff, 0xe3, 0xe0, 0x01,
        0x06, 0x3c, 0x2d, 0x30, 0x33, 0x3e, 0x00, 0x3c, 0x2d, 0x30, 0x32, 0x3e, 0x00, 0x54, 0x5a,
        0x69, 0x66, 0x33, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00,
        0x00, 0x65, 0x18, 0xb6, 0x80, 0x01, 0xff, 0xff, 0xd5, 0xd0, 0x00, 0x00, 0xff, 0xff, 0xe3,
        0xe0, 0x01, 0x06, 0x3c, 0x2d, 0x30, 0x33, 0x3e, 0x00, 0x3c, 0x2d, 0x30, 0x32, 0x3e, 0x00,
        0x0a, 0x3c, 0x2d, 0x30, 0x33, 0x3e, 0x33, 0x3c, 0x2d, 0x30, 0x32, 0x3e, 0x2c, 0x4d, 0x31,
        0x30, 0x2e, 0x31, 0x2e, 0x30, 0x2f, 0x32, 0x34, 0x2c, 0x4d, 0x33, 0x2e, 0x31, 0x2e, 0x30,
        0x2f, 0x2d, 0x32, 0x0a,
    ];

    fn civil(year: i64, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> CivilDateTime {
        CivilDateTime::new(
            Rd(rd_from_ymd(year, month, day)),
            CivilTime::hms(hour, minute, second).unwrap(),
        )
    }

    #[test]
    fn the_fixture_declares_version_two_and_six_counts() {
        let data = TzifData::parse(TZIF_V2_EASTERN).unwrap();
        assert_eq!(data.version(), TzifVersion::V2);
        assert_eq!(data.transition_count(), 4);
        assert_eq!(data.local_time_type_count(), 2);
        assert_eq!(data.leap_second_count(), 1);
        assert_eq!(data.header().designation_len, 8);
        assert_eq!(data.header().ut_indicator_count, 2);
        assert_eq!(data.header().standard_indicator_count, 2);
    }

    #[test]
    fn transition_times_and_their_types_are_read_in_order() {
        let data = TzifData::parse(TZIF_V2_EASTERN).unwrap();
        let transitions: Vec<_> = data.transitions().collect();
        assert_eq!(
            transitions.iter().map(|t| t.time).collect::<Vec<_>>(),
            [1_710_054_000, 1_730_613_600, 1_741_503_600, 1_762_063_200]
        );
        assert_eq!(
            transitions.iter().map(|t| t.type_index).collect::<Vec<_>>(),
            [1, 0, 1, 0]
        );
    }

    #[test]
    fn local_time_types_carry_offsets_flags_and_abbreviations() {
        let data = TzifData::parse(TZIF_V2_EASTERN).unwrap();
        let types: Vec<_> = data.local_time_types().collect();
        assert_eq!(types[0].offset.seconds(), -5 * 3_600);
        assert!(!types[0].is_dst);
        assert_eq!(types[0].abbreviation, "EST");
        assert_eq!(types[1].offset.seconds(), -4 * 3_600);
        assert!(types[1].is_dst);
        assert_eq!(types[1].abbreviation, "EDT");
        assert_eq!(data.designation_at(4), Some("EDT"));
        assert_eq!(data.designation_at(9), None);
    }

    #[test]
    fn leap_second_records_are_read() {
        let data = TzifData::parse(TZIF_V2_EASTERN).unwrap();
        let leap = data.leap_seconds().next().unwrap();
        // The leap second of 2016-12-31 took TAI - UTC to 37; the fixture
        // records the correction as the file format does, cumulatively.
        assert_eq!(leap.occurrence, 1_483_228_800);
        assert_eq!(leap.correction, 27);
        assert_eq!(data.leap_second(1), None);
    }

    #[test]
    fn the_indicator_arrays_are_readable() {
        let data = TzifData::parse(TZIF_V2_EASTERN).unwrap();
        assert_eq!(data.is_standard_indicator(0), Some(false));
        assert_eq!(data.is_ut_indicator(1), Some(false));
        assert_eq!(data.is_ut_indicator(2), None);
    }

    #[test]
    fn the_footer_is_a_posix_tz_string() {
        let data = TzifData::parse(TZIF_V2_EASTERN).unwrap();
        assert_eq!(data.footer(), Some("EST5EDT,M3.2.0,M11.1.0"));
        let zone = TzifTimeZone::parse("America/New_York", TZIF_V2_EASTERN).unwrap();
        assert_eq!(
            zone.posix_footer().unwrap().to_string(),
            "EST5EDT,M3.2.0,M11.1.0"
        );
        assert_eq!(zone.name(), "America/New_York");
    }

    #[test]
    fn recorded_transitions_govern_the_years_they_cover() {
        let zone = TzifTimeZone::parse("America/New_York", TZIF_V2_EASTERN).unwrap();
        let winter = UnixTime::from_seconds(1_710_054_000 - 1);
        let summer = UnixTime::from_seconds(1_710_054_000);
        assert_eq!(zone.offset_at(winter).seconds(), -5 * 3_600);
        assert_eq!(zone.abbreviation_at(winter), Some("EST"));
        assert!(!zone.is_dst_at(winter));
        assert_eq!(zone.offset_at(summer).seconds(), -4 * 3_600);
        assert_eq!(zone.abbreviation_at(summer), Some("EDT"));
        assert!(zone.is_dst_at(summer));
    }

    #[test]
    fn instants_before_the_first_transition_use_the_first_standard_type() {
        let data = TzifData::parse(TZIF_V2_EASTERN).unwrap();
        assert_eq!(data.first_standard_type_index(), 0);
        assert_eq!(data.transition_index_at(0), None);
        let zone = TzifTimeZone::parse("America/New_York", TZIF_V2_EASTERN).unwrap();
        let long_ago = UnixTime::from_seconds(0);
        assert_eq!(zone.offset_at(long_ago).seconds(), -5 * 3_600);
        assert_eq!(zone.abbreviation_at(long_ago), Some("EST"));
    }

    #[test]
    fn instants_after_the_last_transition_follow_the_footer() {
        let zone = TzifTimeZone::parse("America/New_York", TZIF_V2_EASTERN).unwrap();
        // 2026-07-01T12:00:00Z is past every recorded transition, and the
        // last one recorded was a switch to standard time; only the footer
        // can know that July is on daylight saving time.
        let summer_2026 = UnixTime::from_seconds(1_782_000_000);
        assert_eq!(zone.offset_at(summer_2026).seconds(), -4 * 3_600);
        assert_eq!(zone.abbreviation_at(summer_2026), Some("EDT"));
        assert!(zone.is_dst_at(summer_2026));
        let winter_2027 = UnixTime::from_seconds(1_800_000_000);
        assert_eq!(zone.offset_at(winter_2027).seconds(), -5 * 3_600);
        assert!(!zone.is_dst_at(winter_2027));
    }

    #[test]
    fn version_one_files_have_no_footer_and_keep_their_last_type() {
        let data = TzifData::parse(TZIF_V1_EASTERN).unwrap();
        assert_eq!(data.version(), TzifVersion::V1);
        assert_eq!(data.footer(), None);
        assert_eq!(data.transition_count(), 4);
        assert_eq!(data.leap_second_count(), 0);
        let zone = TzifTimeZone::parse("America/New_York", TZIF_V1_EASTERN).unwrap();
        assert!(zone.posix_footer().is_none());
        // Without a footer the last recorded type persists, so the summer of
        // 2026 is standard time. That is the format's limitation, faithfully
        // reported rather than papered over.
        let summer_2026 = UnixTime::from_seconds(1_782_000_000);
        assert_eq!(zone.offset_at(summer_2026).seconds(), -5 * 3_600);
    }

    #[test]
    fn the_thirty_two_and_sixty_four_bit_blocks_agree() {
        let wide = TzifData::parse(TZIF_V2_EASTERN).unwrap();
        let narrow = TzifData::parse(TZIF_V1_EASTERN).unwrap();
        for index in 0..4 {
            assert_eq!(wide.transition_time(index), narrow.transition_time(index));
            assert_eq!(
                wide.transition_type_index(index),
                narrow.transition_type_index(index)
            );
        }
    }

    #[test]
    fn version_three_footers_may_use_hour_twenty_four_and_negative_times() {
        let data = TzifData::parse(TZIF_V3_ODD_FOOTER).unwrap();
        assert_eq!(data.version(), TzifVersion::V3);
        assert_eq!(data.footer(), Some("<-03>3<-02>,M10.1.0/24,M3.1.0/-2"));
        let zone = TzifTimeZone::parse("Test/Odd", TZIF_V3_ODD_FOOTER).unwrap();
        let rules = zone.posix_footer().unwrap();
        assert_eq!(rules.dst().unwrap().start.time, 86_400);
        assert_eq!(rules.dst().unwrap().end.time, -7_200);
        assert_eq!(rules.standard_offset().seconds(), -3 * 3_600);
    }

    #[test]
    fn a_tzif_zone_reports_the_repeated_and_the_skipped_hour() {
        let zone = TzifTimeZone::parse("America/New_York", TZIF_V2_EASTERN).unwrap();
        let repeated = zone.resolve_local(civil(2024, 11, 3, 1, 30, 0));
        assert!(repeated.is_ambiguous());
        assert_eq!(repeated.earliest().seconds(), 1_730_611_800);
        assert_eq!(repeated.latest().seconds(), 1_730_615_400);

        let skipped = zone.resolve_local(civil(2024, 3, 10, 2, 30, 0));
        assert!(skipped.is_nonexistent());
        assert_eq!(
            skipped
                .resolve(Disambiguation::PushForward)
                .unwrap()
                .seconds(),
            1_710_055_800
        );

        let ordinary = zone.resolve_local(civil(2024, 6, 1, 12, 0, 0));
        assert!(ordinary.is_unambiguous());
        assert_eq!(
            zone.local_at(ordinary.earliest()).unwrap(),
            civil(2024, 6, 1, 12, 0, 0)
        );
    }

    #[test]
    fn every_hour_of_the_recorded_range_round_trips() {
        let zone = TzifTimeZone::parse("America/New_York", TZIF_V2_EASTERN).unwrap();
        for seconds in (1_700_000_000..1_770_000_000).step_by(3_600) {
            let when = UnixTime::from_seconds(seconds);
            let local = zone.local_at(when).unwrap();
            let resolution = zone.resolve_local(local);
            assert!(
                resolution.earliest() <= when && when <= resolution.latest(),
                "{seconds}"
            );
        }
    }

    #[test]
    fn the_binary_search_finds_the_right_transition_everywhere() {
        let data = TzifData::parse(TZIF_V2_EASTERN).unwrap();
        let times = [1_710_054_000, 1_730_613_600, 1_741_503_600, 1_762_063_200];
        for (index, time) in times.iter().enumerate() {
            assert_eq!(data.transition_index_at(*time), Some(index));
            assert_eq!(data.transition_index_at(time + 1), Some(index));
            assert_eq!(
                data.transition_index_at(time - 1),
                index.checked_sub(1),
                "{time}"
            );
        }
    }

    #[test]
    fn versions_report_the_width_of_their_transition_times() {
        assert_eq!(TzifVersion::from_byte(0).unwrap().time_size(), 4);
        assert_eq!(TzifVersion::from_byte(b'2').unwrap().time_size(), 8);
        assert_eq!(TzifVersion::from_byte(b'3').unwrap().time_size(), 8);
        assert_eq!(
            TzifVersion::from_byte(b'4'),
            Err(TzError::UnsupportedTzifVersion)
        );
        // Already-parsed data can be wrapped without re-reading the bytes.
        let data = TzifData::parse(TZIF_V2_EASTERN).unwrap();
        let zone = TzifTimeZone::from_data("America/New_York", data).unwrap();
        assert_eq!(zone.data().transition_count(), 4);
        assert_eq!(zone.data().header().type_count, 2);
    }

    #[test]
    fn bytes_that_are_not_tzif_data_are_refused() {
        assert_eq!(TzifData::parse(b""), Err(TzError::TruncatedTzifData));
        assert_eq!(TzifData::parse(&[0u8; 44]), Err(TzError::NotTzifData));
        let mut wrong_version = TZIF_V2_EASTERN.to_vec();
        wrong_version[4] = b'9';
        assert_eq!(
            TzifData::parse(&wrong_version),
            Err(TzError::UnsupportedTzifVersion)
        );
        assert_eq!(
            TzifData::parse(&TZIF_V2_EASTERN[..100]),
            Err(TzError::TruncatedTzifData)
        );
    }

    #[test]
    fn internally_inconsistent_files_are_refused() {
        // A type count of zero leaves nothing to point at.
        let mut no_types = TZIF_V2_EASTERN.to_vec();
        no_types[36..40].copy_from_slice(&0u32.to_be_bytes());
        assert_eq!(TzifData::parse(&no_types), Err(TzError::MalformedTzifData));

        // A designation index past the end of the string table.
        let mut bad_index = TZIF_V2_EASTERN.to_vec();
        let position = bad_index
            .windows(4)
            .rposition(|window| window == b"EST\0")
            .unwrap();
        bad_index[position - 1] = 200;
        assert_eq!(TzifData::parse(&bad_index), Err(TzError::MalformedTzifData));

        // A footer that is not newline-delimited.
        let mut bad_footer = TZIF_V2_EASTERN.to_vec();
        let last = bad_footer.len() - 1;
        bad_footer[last] = b'X';
        assert_eq!(
            TzifData::parse(&bad_footer),
            Err(TzError::MalformedTzifData)
        );
    }

    #[test]
    fn transitions_must_be_strictly_increasing() {
        let mut jumbled = TZIF_V2_EASTERN.to_vec();
        // Overwrite the second 64-bit transition time with the first one.
        let header = TzifHeader::parse(TZIF_V2_EASTERN).unwrap();
        let block_start = HEADER_LEN + header.data_len(4) + HEADER_LEN;
        let first = block_start;
        let second = block_start + 8;
        let copy: Vec<u8> = jumbled[first..first + 8].to_vec();
        jumbled[second..second + 8].copy_from_slice(&copy);
        assert_eq!(TzifData::parse(&jumbled), Err(TzError::MalformedTzifData));
    }
}
