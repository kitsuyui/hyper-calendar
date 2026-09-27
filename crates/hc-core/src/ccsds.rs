//! The CCSDS time code formats with binary counts: the preamble field
//! (P-field) of every binary code, the Unsegmented Code (CUC) and the Day
//! Segmented Code (CDS).
//!
//! CCSDS 301.0-B-4, *Time Code Formats*, Blue Book, Issue 4, November 2010,
//! read 2026-09-27 (`ccsds-301-0-b-4`). A code is an optional P-field that
//! names the format and a T-field that holds the time; bit 0 of a field is
//! the first transmitted and the most significant (§1.5).
//!
//! - **CUC** (§3.2) is a binary count of seconds, 1 to 7 octets, and of
//!   binary fractions of a second, 0 to 10 octets. At Level 1 it counts TAI
//!   from 1958 January 1, [`crate::epoch::CCSDS_CUC`]: "This time code is
//!   not UTC-based and leap-second corrections do not apply". At Level 2 the
//!   epoch is the agency's, and this module gives the span since it.
//! - **CDS** (§3.3) is a count of days, 16 or 24 bits, the millisecond of
//!   the day, 32 bits, and optionally the microsecond (16 bits) or the
//!   picosecond (32 bits) of the millisecond. It is UTC: the millisecond of
//!   the day reaches 86 400 999 on a day that ends in an inserted leap
//!   second, which is checked against [`crate::leap`].
//!
//! The Calendar Segmented Code (CCS) and the ASCII codes are calendar
//! readings and live in `hc-format::ccsds`, which uses the [`Preamble`]
//! and [`Octets`] of this module. `docs/systems/ccsds-time-codes.md` works
//! an example of each code through.
//!
//! Where the standard reserves a value, or allows an extension octet
//! without defining its content, this module refuses it with
//! [`TimeError::OutOfRange`] rather than guessing a meaning.

use crate::duration::{ATTOS_PER_SEC, Duration};
use crate::epoch::CCSDS_CUC;
use crate::error::{TimeError, TimeResult};
use crate::leap;
use crate::scale::{Instant, Tai, TimeScale};
use crate::unix::UtcInstant;

/// The CDS Level 1 epoch, 1958 January 1, as a day of the POSIX day count:
/// 4 383 days before 1970-01-01.
pub const CDS_EPOCH_UNIX_DAY: i64 = -4_383;

/// Whether a CUC or CDS code counts from the recommended epoch or from one
/// its agency defines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EpochLevel {
    /// 1958 January 1, a Level 1 code: TAI for CUC, UTC days for CDS.
    Recommended,
    /// An epoch "necessary to obtain … from an external source", a Level 2
    /// code.
    AgencyDefined,
}

/// The shape of a CUC T-field, as its P-field states it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CucFormat {
    /// The epoch the count starts from.
    pub epoch: EpochLevel,
    /// Octets of whole seconds, 1 to 7.
    pub coarse_octets: u8,
    /// Octets of binary fraction of a second, 0 to 10.
    pub fine_octets: u8,
    /// Bits 6–7 of the P-field's second octet, "reserved for mission
    /// definition", 0 to 3. A non-zero value makes the P-field two octets.
    pub mission_bits: u8,
}

impl CucFormat {
    /// A format with no mission bits.
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] for a width outside 1–7 or 0–10 octets.
    pub const fn new(epoch: EpochLevel, coarse_octets: u8, fine_octets: u8) -> TimeResult<Self> {
        let format = Self {
            epoch,
            coarse_octets,
            fine_octets,
            mission_bits: 0,
        };
        if format.is_valid() {
            Ok(format)
        } else {
            Err(TimeError::OutOfRange)
        }
    }

    const fn is_valid(self) -> bool {
        self.coarse_octets >= 1
            && self.coarse_octets <= 7
            && self.fine_octets <= 10
            && self.mission_bits <= 3
    }

    /// The length of the T-field in octets.
    #[must_use]
    pub const fn t_field_octets(self) -> usize {
        self.coarse_octets as usize + self.fine_octets as usize
    }
}

/// The width of a CDS day segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DaySegment {
    /// 16 bits, days 0 to 65 535.
    Bits16,
    /// 24 bits, "for special applications such as Astronomy".
    Bits24,
}

impl DaySegment {
    const fn octets(self) -> usize {
        match self {
            Self::Bits16 => 2,
            Self::Bits24 => 3,
        }
    }
}

/// The optional submillisecond segment of a CDS code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Submillisecond {
    /// Absent: millisecond resolution.
    None,
    /// 16 bits of microseconds of the millisecond, 0 to 999.
    Microseconds,
    /// 32 bits of picoseconds of the millisecond, 0 to 999 999 999.
    Picoseconds,
}

impl Submillisecond {
    const fn octets(self) -> usize {
        match self {
            Self::None => 0,
            Self::Microseconds => 2,
            Self::Picoseconds => 4,
        }
    }

    /// The largest value, and the attoseconds in one unit.
    const fn limit_and_unit(self) -> (u32, u64) {
        match self {
            Self::None => (0, 0),
            Self::Microseconds => (999, 1_000_000_000_000),
            Self::Picoseconds => (999_999_999, 1_000_000),
        }
    }
}

/// The shape of a CDS T-field, as its P-field states it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CdsFormat {
    /// The epoch the day count starts from.
    pub epoch: EpochLevel,
    /// The width of the day segment.
    pub day: DaySegment,
    /// The submillisecond segment, if any.
    pub submillisecond: Submillisecond,
}

impl CdsFormat {
    /// The length of the T-field in octets.
    #[must_use]
    pub const fn t_field_octets(self) -> usize {
        self.day.octets() + 4 + self.submillisecond.octets()
    }
}

/// Which calendar fields a CCS T-field carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CcsVariation {
    /// Year, month and day of the month (§3.4.1.1).
    MonthOfYear,
    /// Year and day of the year (§3.4.1.2).
    DayOfYear,
}

/// The shape of a CCS T-field, as its P-field states it.
///
/// The T-field itself is decoded by `hc-format::ccsds`, which has the
/// calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CcsFormat {
    /// The calendar fields.
    pub variation: CcsVariation,
    /// Subsecond octets of two decimal digits each, 0 to 6: a resolution of
    /// 1 s down to 10⁻¹² s.
    pub subsecond_octets: u8,
}

impl CcsFormat {
    /// The length of the T-field in octets: seven, and the subsecond octets.
    #[must_use]
    pub const fn t_field_octets(self) -> usize {
        7 + self.subsecond_octets as usize
    }
}

/// A decoded P-field: which code follows, and its shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Preamble {
    /// Identification `001` (Level 1) or `010` (Level 2).
    Cuc(CucFormat),
    /// Identification `100`.
    Cds(CdsFormat),
    /// Identification `101`.
    Ccs(CcsFormat),
    /// Identification `110`, a Level 3 or Level 4 code whose octets only the
    /// agency can read: 1 to 16 of them (§3.6).
    AgencyDefined {
        /// The length of the T-field in octets.
        octets: u8,
    },
}

impl Preamble {
    /// The P-field at the start of `bytes`, and the number of octets it
    /// took.
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] for an empty input, the reserved
    /// identifications `000`, `011` and `111`, the reserved CDS resolution
    /// `11` and CCS resolution `111`, a second octet that is announced and
    /// missing, and an extension octet the standard does not define: a third
    /// CUC octet, or any second octet of a CDS, CCS or agency-defined
    /// P-field.
    pub fn decode(bytes: &[u8]) -> TimeResult<(Self, usize)> {
        let first = *bytes.first().ok_or(TimeError::OutOfRange)?;
        let extended = first & 0x80 != 0;
        match (first >> 4) & 0b111 {
            identification @ (0b001 | 0b010) => {
                let epoch = if identification == 0b001 {
                    EpochLevel::Recommended
                } else {
                    EpochLevel::AgencyDefined
                };
                let mut format = CucFormat {
                    epoch,
                    coarse_octets: ((first >> 2) & 0b11) + 1,
                    fine_octets: first & 0b11,
                    mission_bits: 0,
                };
                if !extended {
                    return Ok((Self::Cuc(format), 1));
                }
                let second = *bytes.get(1).ok_or(TimeError::OutOfRange)?;
                if second & 0x80 != 0 {
                    return Err(TimeError::OutOfRange);
                }
                format.coarse_octets += (second >> 5) & 0b11;
                format.fine_octets += (second >> 2) & 0b111;
                format.mission_bits = second & 0b11;
                Ok((Self::Cuc(format), 2))
            }
            _ if extended => Err(TimeError::OutOfRange),
            0b100 => {
                let submillisecond = match first & 0b11 {
                    0b00 => Submillisecond::None,
                    0b01 => Submillisecond::Microseconds,
                    0b10 => Submillisecond::Picoseconds,
                    _ => return Err(TimeError::OutOfRange),
                };
                let format = CdsFormat {
                    epoch: if first & 0x08 == 0 {
                        EpochLevel::Recommended
                    } else {
                        EpochLevel::AgencyDefined
                    },
                    day: if first & 0x04 == 0 {
                        DaySegment::Bits16
                    } else {
                        DaySegment::Bits24
                    },
                    submillisecond,
                };
                Ok((Self::Cds(format), 1))
            }
            0b101 => {
                let subsecond_octets = first & 0b111;
                if subsecond_octets == 0b111 {
                    return Err(TimeError::OutOfRange);
                }
                let variation = if first & 0x08 == 0 {
                    CcsVariation::MonthOfYear
                } else {
                    CcsVariation::DayOfYear
                };
                Ok((
                    Self::Ccs(CcsFormat {
                        variation,
                        subsecond_octets,
                    }),
                    1,
                ))
            }
            0b110 => Ok((
                Self::AgencyDefined {
                    octets: (first & 0x0F) + 1,
                },
                1,
            )),
            _ => Err(TimeError::OutOfRange),
        }
    }

    /// The P-field's octets.
    ///
    /// A CUC field is written in one octet when its widths fit, 1 to 4
    /// octets of seconds and 0 to 3 of fraction, and its mission bits are
    /// zero; otherwise in two, the first octet carrying 4 and 3 and the
    /// second the rest. A two-octet field that reads the same as a
    /// one-octet one is therefore written in its short form.
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] for a width outside the ranges the fields
    /// can express.
    pub fn encode(self) -> TimeResult<Octets> {
        let mut out = Octets::new();
        match self {
            Self::Cuc(format) => {
                if !format.is_valid() {
                    return Err(TimeError::OutOfRange);
                }
                let identification = match format.epoch {
                    EpochLevel::Recommended => 0b001,
                    EpochLevel::AgencyDefined => 0b010,
                };
                let coarse = if format.coarse_octets < 4 {
                    format.coarse_octets
                } else {
                    4
                };
                let fine = if format.fine_octets < 3 {
                    format.fine_octets
                } else {
                    3
                };
                let extra_coarse = format.coarse_octets - coarse;
                let extra_fine = format.fine_octets - fine;
                let extended = extra_coarse != 0 || extra_fine != 0 || format.mission_bits != 0;
                out.push(u8::from(extended) << 7 | identification << 4 | (coarse - 1) << 2 | fine)?;
                if extended {
                    out.push(extra_coarse << 5 | extra_fine << 2 | format.mission_bits)?;
                }
            }
            Self::Cds(format) => {
                let epoch = match format.epoch {
                    EpochLevel::Recommended => 0,
                    EpochLevel::AgencyDefined => 0x08,
                };
                let day = match format.day {
                    DaySegment::Bits16 => 0,
                    DaySegment::Bits24 => 0x04,
                };
                let submillisecond = match format.submillisecond {
                    Submillisecond::None => 0b00,
                    Submillisecond::Microseconds => 0b01,
                    Submillisecond::Picoseconds => 0b10,
                };
                out.push(0b100 << 4 | epoch | day | submillisecond)?;
            }
            Self::Ccs(format) => {
                if format.subsecond_octets > 6 {
                    return Err(TimeError::OutOfRange);
                }
                let variation = match format.variation {
                    CcsVariation::MonthOfYear => 0,
                    CcsVariation::DayOfYear => 0x08,
                };
                out.push(0b101 << 4 | variation | format.subsecond_octets)?;
            }
            Self::AgencyDefined { octets } => {
                if !(1..=16).contains(&octets) {
                    return Err(TimeError::OutOfRange);
                }
                out.push(0b110 << 4 | (octets - 1))?;
            }
        }
        Ok(out)
    }

    /// The length of the T-field the P-field announces.
    #[must_use]
    pub const fn t_field_octets(self) -> usize {
        match self {
            Self::Cuc(format) => format.t_field_octets(),
            Self::Cds(format) => format.t_field_octets(),
            Self::Ccs(format) => format.t_field_octets(),
            Self::AgencyDefined { octets } => octets as usize,
        }
    }
}

/// The octets of a code, P-field and T-field, without an allocator.
///
/// The longest binary code is a two-octet CUC P-field and a T-field of
/// 7 + 10 octets, [`Octets::CAPACITY`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Octets {
    bytes: [u8; Self::CAPACITY],
    len: u8,
}

impl Octets {
    /// The most octets a code can take.
    pub const CAPACITY: usize = 19;

    /// No octets.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bytes: [0; Self::CAPACITY],
            len: 0,
        }
    }

    /// The octets written so far.
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[..usize::from(self.len)]
    }

    /// The number of octets.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len as usize
    }

    /// Whether nothing has been written.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Append one octet.
    ///
    /// # Errors
    ///
    /// [`TimeError::Overflow`] past [`Octets::CAPACITY`].
    pub fn push(&mut self, byte: u8) -> TimeResult<()> {
        let slot = self
            .bytes
            .get_mut(usize::from(self.len))
            .ok_or(TimeError::Overflow)?;
        *slot = byte;
        self.len += 1;
        Ok(())
    }

    /// Append octets.
    ///
    /// # Errors
    ///
    /// [`TimeError::Overflow`] past [`Octets::CAPACITY`].
    pub fn extend_from_slice(&mut self, bytes: &[u8]) -> TimeResult<()> {
        for &byte in bytes {
            self.push(byte)?;
        }
        Ok(())
    }

    /// Append the low `octets` octets of `value`, most significant first.
    ///
    /// # Errors
    ///
    /// [`TimeError::Overflow`] past [`Octets::CAPACITY`].
    pub fn push_be(&mut self, value: u128, octets: usize) -> TimeResult<()> {
        for index in (0..octets).rev() {
            // A shift of 128 or more would be out of range; those octets
            // are zero.
            let byte = value.checked_shr(8 * index as u32).unwrap_or(0) as u8;
            self.push(byte)?;
        }
        Ok(())
    }
}

impl Default for Octets {
    fn default() -> Self {
        Self::new()
    }
}

/// A big-endian unsigned integer of up to 16 octets.
fn read_be(bytes: &[u8]) -> u128 {
    bytes
        .iter()
        .fold(0, |value, &byte| value << 8 | u128::from(byte))
}

/// Split `bytes` into a P-field and its T-field, the P-field naming the
/// code `expect` accepts.
fn split_preamble<T>(
    bytes: &[u8],
    expect: impl FnOnce(Preamble) -> Option<T>,
) -> TimeResult<(T, &[u8])> {
    let (preamble, used) = Preamble::decode(bytes)?;
    let format = expect(preamble).ok_or(TimeError::OutOfRange)?;
    Ok((format, &bytes[used..]))
}

/// A CUC code: a count of seconds and binary fractions of a second from its
/// epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CucTime {
    format: CucFormat,
    coarse: u64,
    fine: u128,
}

/// 5¹⁸: 10¹⁸ = 2¹⁸ · 5¹⁸, which lets a binary fraction convert to
/// attoseconds without a product wider than 128 bits.
const FIVE_POW_18: u128 = 3_814_697_265_625;

impl CucTime {
    /// A count in a format.
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] for an invalid format, or a count wider
    /// than its octets.
    pub fn new(format: CucFormat, coarse: u64, fine: u128) -> TimeResult<Self> {
        if !format.is_valid()
            || coarse >> (8 * u32::from(format.coarse_octets)) != 0
            || fine >> (8 * u32::from(format.fine_octets)) != 0
        {
            return Err(TimeError::OutOfRange);
        }
        Ok(Self {
            format,
            coarse,
            fine,
        })
    }

    /// The format.
    #[must_use]
    pub const fn format(self) -> CucFormat {
        self.format
    }

    /// The count of whole seconds.
    #[must_use]
    pub const fn coarse(self) -> u64 {
        self.coarse
    }

    /// The fraction, in units of 2⁻⁸ⁿ s for *n* fine octets.
    #[must_use]
    pub const fn fine(self) -> u128 {
        self.fine
    }

    /// A T-field read in a format the caller knows, the P-field being
    /// implicit.
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] for an invalid format or a T-field of the
    /// wrong length.
    pub fn decode(format: CucFormat, t_field: &[u8]) -> TimeResult<Self> {
        if !format.is_valid() || t_field.len() != format.t_field_octets() {
            return Err(TimeError::OutOfRange);
        }
        let (coarse, fine) = t_field.split_at(usize::from(format.coarse_octets));
        Self::new(format, read_be(coarse) as u64, read_be(fine))
    }

    /// A P-field naming a CUC code, and its T-field.
    ///
    /// # Errors
    ///
    /// As [`Preamble::decode`] and [`CucTime::decode`], and
    /// [`TimeError::OutOfRange`] for a P-field of another code.
    pub fn decode_with_preamble(bytes: &[u8]) -> TimeResult<Self> {
        let (format, t_field) = split_preamble(bytes, |preamble| match preamble {
            Preamble::Cuc(format) => Some(format),
            _ => None,
        })?;
        Self::decode(format, t_field)
    }

    /// The T-field, and the P-field before it when `with_preamble` is set.
    ///
    /// # Errors
    ///
    /// None in practice: the format was checked when the value was built.
    pub fn encode(self, with_preamble: bool) -> TimeResult<Octets> {
        let mut out = if with_preamble {
            Preamble::Cuc(self.format).encode()?
        } else {
            Octets::new()
        };
        out.push_be(
            u128::from(self.coarse),
            usize::from(self.format.coarse_octets),
        )?;
        out.push_be(self.fine, usize::from(self.format.fine_octets))?;
        Ok(out)
    }

    /// The span since the epoch, the fraction rounded up to the attosecond.
    ///
    /// With up to two fine octets the unit, 2⁻¹⁶ s, is a whole number of
    /// attoseconds and the span is exact. With three to seven the unit is
    /// finer than that but coarser than the attosecond, and rounding up
    /// keeps the round trip: [`CucTime::from_since_epoch`] of the span is
    /// this count again. With eight or more the unit is finer than the
    /// attosecond, and several counts share a span.
    #[must_use]
    pub fn since_epoch(self) -> Duration {
        let bits = 8 * u32::from(self.format.fine_octets);
        let scaled = self.fine * FIVE_POW_18;
        let attos = if bits >= 18 {
            let shift = bits - 18;
            let floor = scaled >> shift;
            floor + u128::from(floor << shift != scaled)
        } else {
            scaled << (18 - bits)
        };
        // At most 10¹⁸, when the rounding carries into the next second.
        Duration::from_attos(i128::from(self.coarse) * ATTOS_PER_SEC as i128 + attos as i128)
    }

    /// The count for a span since the epoch, floored to the format's unit.
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] for a negative span, which the count
    /// cannot express, or one past the last count of the format.
    pub fn from_since_epoch(format: CucFormat, span: Duration) -> TimeResult<Self> {
        if !format.is_valid() || span.is_negative() {
            return Err(TimeError::OutOfRange);
        }
        let coarse = u64::try_from(span.whole_seconds()).map_err(|_| TimeError::OutOfRange)?;
        let attos = u128::from(span.subsec_attos());
        let bits = 8 * u32::from(format.fine_octets);
        let fine = if bits >= 18 {
            (attos << (bits - 18)) / FIVE_POW_18
        } else {
            attos / (FIVE_POW_18 << (18 - bits))
        };
        Self::new(format, coarse, fine)
    }

    /// The TAI instant of a Level 1 code.
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] for a Level 2 code, whose epoch is not
    /// known here; use [`CucTime::since_epoch`] or [`CucTime::after`].
    pub fn to_tai(self) -> TimeResult<Instant<Tai>> {
        if self.format.epoch != EpochLevel::Recommended {
            return Err(TimeError::OutOfRange);
        }
        CCSDS_CUC.instant().checked_add(self.since_epoch())
    }

    /// The Level 1 code of a TAI instant.
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] for a Level 2 format, an instant before
    /// 1958 or one past the last count.
    pub fn from_tai(format: CucFormat, instant: Instant<Tai>) -> TimeResult<Self> {
        if format.epoch != EpochLevel::Recommended {
            return Err(TimeError::OutOfRange);
        }
        Self::from_since_epoch(format, instant.duration_since(CCSDS_CUC.instant())?)
    }

    /// The instant of a code on the scale and from the epoch the caller
    /// names, as a Level 2 code needs.
    ///
    /// # Errors
    ///
    /// [`TimeError::Overflow`] when the sum leaves the range of an instant.
    pub fn after<S: TimeScale>(self, epoch: Instant<S>) -> TimeResult<Instant<S>> {
        epoch.checked_add(self.since_epoch())
    }
}

/// A CDS code: a day count, the millisecond of the day and the
/// submillisecond.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CdsTime {
    format: CdsFormat,
    day: u32,
    millisecond: u32,
    submillisecond: u32,
}

/// The largest millisecond of the day, on a day that ends in an inserted
/// leap second (annex A).
const LAST_MILLISECOND: u32 = 86_400_999;

impl CdsTime {
    /// The code with these segments.
    ///
    /// The millisecond of the day may reach 86 400 999, as on a day with an
    /// inserted leap second; whether the day has one is checked when the
    /// code is read as UTC.
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] for a day wider than its segment, a
    /// millisecond past 86 400 999, or a submillisecond past 999 µs or
    /// 999 999 999 ps, or non-zero when the format has no such segment.
    pub fn new(
        format: CdsFormat,
        day: u32,
        millisecond: u32,
        submillisecond: u32,
    ) -> TimeResult<Self> {
        let day_limit = match format.day {
            DaySegment::Bits16 => u32::from(u16::MAX),
            DaySegment::Bits24 => (1 << 24) - 1,
        };
        let (submillisecond_limit, _) = format.submillisecond.limit_and_unit();
        if day > day_limit
            || millisecond > LAST_MILLISECOND
            || submillisecond > submillisecond_limit
        {
            return Err(TimeError::OutOfRange);
        }
        Ok(Self {
            format,
            day,
            millisecond,
            submillisecond,
        })
    }

    /// The format.
    #[must_use]
    pub const fn format(self) -> CdsFormat {
        self.format
    }

    /// The day count from the epoch.
    #[must_use]
    pub const fn day(self) -> u32 {
        self.day
    }

    /// The millisecond of the day.
    #[must_use]
    pub const fn millisecond(self) -> u32 {
        self.millisecond
    }

    /// The microseconds or picoseconds of the millisecond, 0 when the
    /// format has no such segment.
    #[must_use]
    pub const fn submillisecond(self) -> u32 {
        self.submillisecond
    }

    /// A T-field read in a format the caller knows.
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] for a T-field of the wrong length or a
    /// segment out of its range.
    pub fn decode(format: CdsFormat, t_field: &[u8]) -> TimeResult<Self> {
        if t_field.len() != format.t_field_octets() {
            return Err(TimeError::OutOfRange);
        }
        let (day, rest) = t_field.split_at(format.day.octets());
        let (millisecond, submillisecond) = rest.split_at(4);
        Self::new(
            format,
            read_be(day) as u32,
            read_be(millisecond) as u32,
            read_be(submillisecond) as u32,
        )
    }

    /// A P-field naming a CDS code, and its T-field.
    ///
    /// # Errors
    ///
    /// As [`Preamble::decode`] and [`CdsTime::decode`], and
    /// [`TimeError::OutOfRange`] for a P-field of another code.
    pub fn decode_with_preamble(bytes: &[u8]) -> TimeResult<Self> {
        let (format, t_field) = split_preamble(bytes, |preamble| match preamble {
            Preamble::Cds(format) => Some(format),
            _ => None,
        })?;
        Self::decode(format, t_field)
    }

    /// The T-field, and the P-field before it when `with_preamble` is set.
    ///
    /// # Errors
    ///
    /// None in practice: the segments were checked when the value was
    /// built.
    pub fn encode(self, with_preamble: bool) -> TimeResult<Octets> {
        let mut out = if with_preamble {
            Preamble::Cds(self.format).encode()?
        } else {
            Octets::new()
        };
        out.push_be(u128::from(self.day), self.format.day.octets())?;
        out.push_be(u128::from(self.millisecond), 4)?;
        out.push_be(
            u128::from(self.submillisecond),
            self.format.submillisecond.octets(),
        )?;
        Ok(out)
    }

    /// The UTC instant of a Level 1 code.
    ///
    /// # Errors
    ///
    /// As [`CdsTime::to_utc_from_epoch`], and [`TimeError::OutOfRange`] for
    /// a Level 2 code.
    pub fn to_utc(self) -> TimeResult<UtcInstant> {
        if self.format.epoch != EpochLevel::Recommended {
            return Err(TimeError::OutOfRange);
        }
        self.to_utc_from_epoch(CDS_EPOCH_UNIX_DAY)
    }

    /// The UTC instant of a code whose day 0 is `epoch_unix_day`, counted
    /// in days from 1970-01-01: 1950 January 1 is −7 305.
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] for a millisecond of the day in a second
    /// that the day does not have: 23:59:60 on a day without an inserted
    /// leap second, or 23:59:59 on one with an omitted second.
    /// [`TimeError::AfterModelEnd`] for 23:59:60 on a day past the
    /// leap-second table, which may or may not end in one. A negative leap
    /// second has never been scheduled, and past the table 23:59:59 is read
    /// as written.
    pub fn to_utc_from_epoch(self, epoch_unix_day: i64) -> TimeResult<UtcInstant> {
        let unix_day = epoch_unix_day
            .checked_add(i64::from(self.day))
            .ok_or(TimeError::Overflow)?;
        let second_of_day = i64::from(self.millisecond / 1_000);
        check_second_of_day(unix_day, second_of_day)?;
        let (_, unit) = self.format.submillisecond.limit_and_unit();
        let subsec_attos = u64::from(self.millisecond % 1_000) * 1_000_000_000_000_000
            + u64::from(self.submillisecond) * unit;
        let day_start = unix_day.checked_mul(86_400).ok_or(TimeError::Overflow)?;
        if second_of_day == 86_400 {
            return Ok(UtcInstant {
                unix_seconds: day_start + 86_400,
                leap_second: true,
                subsec_attos,
            });
        }
        Ok(UtcInstant {
            unix_seconds: day_start + second_of_day,
            leap_second: false,
            subsec_attos,
        })
    }

    /// The Level 1 code of a UTC instant, the submillisecond floored to the
    /// format's resolution.
    ///
    /// # Errors
    ///
    /// As [`CdsTime::from_utc_with_epoch`], and [`TimeError::OutOfRange`]
    /// for a Level 2 format.
    pub fn from_utc(format: CdsFormat, utc: UtcInstant) -> TimeResult<Self> {
        if format.epoch != EpochLevel::Recommended {
            return Err(TimeError::OutOfRange);
        }
        Self::from_utc_with_epoch(format, utc, CDS_EPOCH_UNIX_DAY)
    }

    /// The code of a UTC instant counted from `epoch_unix_day`.
    ///
    /// # Errors
    ///
    /// [`TimeError::OutOfRange`] before the epoch, past the day segment, or
    /// for a leap second on a day that does not end in one;
    /// [`TimeError::AfterModelEnd`] for a leap second past the table.
    pub fn from_utc_with_epoch(
        format: CdsFormat,
        utc: UtcInstant,
        epoch_unix_day: i64,
    ) -> TimeResult<Self> {
        let (unix_day, second_of_day) = if utc.leap_second {
            if utc.unix_seconds.rem_euclid(86_400) != 0 {
                return Err(TimeError::OutOfRange);
            }
            (utc.unix_seconds.div_euclid(86_400) - 1, 86_400)
        } else {
            (
                utc.unix_seconds.div_euclid(86_400),
                utc.unix_seconds.rem_euclid(86_400),
            )
        };
        check_second_of_day(unix_day, second_of_day)?;
        let day = u32::try_from(unix_day - epoch_unix_day).map_err(|_| TimeError::OutOfRange)?;
        let millisecond =
            second_of_day as u32 * 1_000 + (utc.subsec_attos / 1_000_000_000_000_000) as u32;
        let (_, unit) = format.submillisecond.limit_and_unit();
        // No submillisecond segment has a unit of zero, and writes 0.
        let submillisecond = (utc.subsec_attos % 1_000_000_000_000_000)
            .checked_div(unit)
            .unwrap_or(0) as u32;
        Self::new(format, day, millisecond, submillisecond)
    }
}

/// Whether second `second_of_day` of UTC day `unix_day` exists: 86 400,
/// 23:59:60, only on a day that ends in an inserted leap second, and
/// 86 399, 23:59:59, not on one that ends in an omitted second.
fn check_second_of_day(unix_day: i64, second_of_day: i64) -> TimeResult<()> {
    match second_of_day {
        86_400 => match leap::end_of_day_step(unix_day)? {
            1 => Ok(()),
            _ => Err(TimeError::OutOfRange),
        },
        86_399 => match leap::end_of_day_step(unix_day) {
            Ok(-1) => Err(TimeError::OutOfRange),
            _ => Ok(()),
        },
        0..=86_398 => Ok(()),
        _ => Err(TimeError::OutOfRange),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unix::{LeapPolicy, UnixTime, tai_from_utc, utc_from_tai};

    const CDS_MICROSECONDS: CdsFormat = CdsFormat {
        epoch: EpochLevel::Recommended,
        day: DaySegment::Bits16,
        submillisecond: Submillisecond::Microseconds,
    };

    fn cuc(coarse: u8, fine: u8) -> CucFormat {
        CucFormat::new(EpochLevel::Recommended, coarse, fine).expect("valid")
    }

    /// §3.2.1: the Level 1 count is TAI seconds from 1958 January 1 TAI,
    /// so count 0 is the epoch.
    #[test]
    fn the_cuc_epoch_is_1958_tai() {
        let zero = CucTime::new(cuc(4, 0), 0, 0).expect("fits");
        assert_eq!(zero.to_tai(), Ok(CCSDS_CUC.instant()));
        assert_eq!(
            CCSDS_CUC.instant().since_epoch(),
            Duration::from_days(CDS_EPOCH_UNIX_DAY)
        );
    }

    /// 2000-01-01T00:00:00 UTC, when TAI − UTC was 32 s: 15 340 days and
    /// 32 s after 1958-01-01 TAI, 0x4EFFA220, `1C 4E FF A2 20`; half a
    /// second later with one fine octet, `1D 4E FF A2 20 80`.
    #[test]
    fn the_first_second_of_2000() {
        let utc = UtcInstant::from_unix(UnixTime::from_seconds(946_684_800));
        let tai = tai_from_utc(utc, LeapPolicy::Strict).expect("in the table");
        let code = CucTime::from_tai(cuc(4, 0), tai).expect("fits");
        assert_eq!(code.coarse(), 15_340 * 86_400 + 32);
        assert_eq!(code.coarse(), 0x4EFF_A220);
        assert_eq!(
            code.encode(true).expect("fits").as_slice(),
            [0x1C, 0x4E, 0xFF, 0xA2, 0x20]
        );
        let half = tai.checked_add(Duration::from_millis(500)).expect("fits");
        let code = CucTime::from_tai(cuc(4, 1), half).expect("fits");
        let bytes = code.encode(true).expect("fits");
        assert_eq!(bytes.as_slice(), [0x1D, 0x4E, 0xFF, 0xA2, 0x20, 0x80]);
        let back = CucTime::decode_with_preamble(bytes.as_slice()).expect("valid");
        assert_eq!(back.to_tai(), Ok(half));
    }

    /// Annex B3.2: "The difference between the epochs 1958 January 1 and
    /// 1950 January 1 is exactly 2922.0 days." A Level 2 CDS code from 1950
    /// reads the same instant as the Level 1 code 2 922 days smaller.
    #[test]
    fn the_1950_agency_epoch() {
        let from_1950 = CdsTime::new(
            CdsFormat {
                epoch: EpochLevel::AgencyDefined,
                ..CDS_MICROSECONDS
            },
            2_922 + 10_974,
            62_443_123,
            456,
        )
        .expect("valid");
        let from_1958 = CdsTime::new(CDS_MICROSECONDS, 10_974, 62_443_123, 456).expect("valid");
        assert_eq!(from_1950.to_utc_from_epoch(-7_305), from_1958.to_utc());
        assert_eq!(from_1950.to_utc(), Err(TimeError::OutOfRange));
        assert_eq!(CDS_EPOCH_UNIX_DAY - (-7_305), 2_922);
    }

    /// The standard's example instant, 1988-01-18T17:20:43.123456 UTC
    /// (§3.5.1.1), in CDS: day 10 974, millisecond 62 443 123, 456 µs,
    /// `41 2A DE 03 B8 CE 73 01 C8`.
    #[test]
    fn the_ascii_example_in_cds() {
        let bytes = [0x41, 0x2A, 0xDE, 0x03, 0xB8, 0xCE, 0x73, 0x01, 0xC8];
        let code = CdsTime::decode_with_preamble(&bytes).expect("valid");
        assert_eq!(
            (code.day(), code.millisecond(), code.submillisecond()),
            (10_974, 62_443_123, 456)
        );
        // 1988-01-18 is day 6 591 of the POSIX count.
        let utc = code.to_utc().expect("an ordinary second");
        assert_eq!(
            utc,
            UtcInstant {
                unix_seconds: 6_591 * 86_400 + 62_443,
                leap_second: false,
                subsec_attos: 123_456_000_000_000_000,
            }
        );
        assert_eq!(CdsTime::from_utc(CDS_MICROSECONDS, utc), Ok(code));
        assert_eq!(code.encode(true).expect("fits").as_slice(), bytes);
    }

    /// Every day of the 16-bit day segment, 0 to 65 535 (1958 to 2137),
    /// round-trips through CDS and UTC at its first, a middle and its last
    /// millisecond: every day in a release build; in a debug one every 97th,
    /// the last, and every day that ends in an inserted leap second of the
    /// table, whose 23:59:60.5 round-trips too, while the day before it has
    /// none (docs/policy.md §7).
    #[test]
    fn every_cds_day_round_trips() {
        let format = CdsFormat {
            submillisecond: Submillisecond::None,
            ..CDS_MICROSECONDS
        };
        let leap_days: alloc::vec::Vec<u32> = leap::steps()
            .filter(|(_, delta)| *delta > 0)
            .map(|(at, _)| (at.div_euclid(86_400) - 1 - CDS_EPOCH_UNIX_DAY) as u32)
            .collect();
        assert_eq!(leap_days.len(), 27);
        assert_eq!(leap_days.last(), Some(&21_549));
        let step = if cfg!(debug_assertions) { 97 } else { 1 };
        let mut days: alloc::vec::Vec<u32> = (0..=u32::from(u16::MAX))
            .step_by(step)
            .chain([u32::from(u16::MAX)])
            .chain(leap_days.iter().copied())
            .collect();
        days.sort_unstable();
        days.dedup();
        for day in days {
            for millisecond in [0, 43_200_001, 86_399_999] {
                let code = CdsTime::new(format, day, millisecond, 0).expect("valid");
                let bytes = code.encode(true).expect("fits");
                let back = CdsTime::decode_with_preamble(bytes.as_slice()).expect("valid");
                assert_eq!(back, code);
                let utc = code.to_utc().expect("a second the day has");
                assert_eq!(CdsTime::from_utc(format, utc), Ok(code), "day {day}");
            }
            if leap_days.contains(&day) {
                let leap = CdsTime::new(format, day, 86_400_500, 0).expect("in range");
                let utc = leap.to_utc().expect("a leap second of the table");
                assert!(utc.leap_second, "day {day}");
                assert_eq!(CdsTime::from_utc(format, utc), Ok(leap));
                let before = CdsTime::new(format, day - 1, 86_400_500, 0).expect("in range");
                assert_eq!(before.to_utc(), Err(TimeError::OutOfRange), "day {day}");
            }
        }
    }

    /// 2016-12-31 ended in an inserted second: it is day 21 549 from 1958,
    /// and millisecond 86 400 500 is 23:59:60.5. The day before has no such
    /// millisecond.
    #[test]
    fn the_2016_leap_second_in_cds() {
        let format = CdsFormat {
            submillisecond: Submillisecond::None,
            ..CDS_MICROSECONDS
        };
        let leap = CdsTime::new(format, 21_549, 86_400_500, 0).expect("in range");
        let utc = leap.to_utc().expect("a real leap second");
        assert_eq!(
            utc,
            UtcInstant {
                unix_seconds: 1_483_228_800,
                leap_second: true,
                subsec_attos: 500_000_000_000_000_000,
            }
        );
        // TAI knows the same second: 2017-01-01 00:00:36.5 TAI.
        let tai = tai_from_utc(utc, LeapPolicy::Strict).expect("in the table");
        assert_eq!(utc_from_tai(tai, LeapPolicy::Strict), Ok(utc));
        assert_eq!(CdsTime::from_utc(format, utc), Ok(leap));
        let day_before = CdsTime::new(format, 21_548, 86_400_500, 0).expect("in range");
        assert_eq!(day_before.to_utc(), Err(TimeError::OutOfRange));
        // Past the table the second cannot be known.
        let later = CdsTime::new(format, 30_000, 86_400_000, 0).expect("in range");
        assert_eq!(later.to_utc(), Err(TimeError::AfterModelEnd));
        let later_ordinary = CdsTime::new(format, 30_000, 86_399_999, 0).expect("in range");
        assert!(later_ordinary.to_utc().is_ok());
    }

    /// Every first octet either decodes or is refused, and one that
    /// decodes, with no extension, re-encodes to itself.
    #[test]
    fn every_first_octet() {
        let mut decoded = 0;
        for first in 0..=u8::MAX {
            match Preamble::decode(&[first, 0x00]) {
                Ok((preamble, used)) => {
                    decoded += 1;
                    let again = preamble.encode().expect("valid");
                    if used == 1 {
                        assert_eq!(again.as_slice(), [first], "{first:#04x}");
                    } else {
                        // A second octet of zeros adds nothing; the short
                        // form reads the same.
                        assert_eq!(again.as_slice(), [first & 0x7F], "{first:#04x}");
                        assert_eq!(Preamble::decode(again.as_slice()), Ok((preamble, 1)));
                    }
                }
                Err(error) => assert_eq!(error, TimeError::OutOfRange, "{first:#04x}"),
            }
        }
        // Without the extension bit: 32 CUC octets, 12 CDS (16 less the four
        // with the reserved resolution), 14 CCS (16 less the two with
        // resolution 111) and 16 agency-defined; with it, the 32 CUC octets
        // again, each followed by a zero second octet.
        assert_eq!(decoded, 32 + 12 + 14 + 16 + 32);
    }

    #[test]
    fn reserved_and_undefined_preambles_are_refused() {
        for bytes in [
            &[0x00][..],   // identification 000
            &[0x30],       // 011
            &[0x70],       // 111
            &[0x43],       // CDS, submillisecond 11
            &[0x57],       // CCS, resolution 111
            &[0xC0, 0x00], // CDS with an extension octet
            &[0x9C],       // CUC extension announced and missing
            &[0x9C, 0x80], // CUC with a third octet
            &[],
        ] {
            assert_eq!(
                Preamble::decode(bytes),
                Err(TimeError::OutOfRange),
                "{bytes:02x?}"
            );
        }
        assert_eq!(
            CucFormat::new(EpochLevel::Recommended, 8, 0),
            Err(TimeError::OutOfRange)
        );
        assert_eq!(
            Preamble::AgencyDefined { octets: 17 }.encode(),
            Err(TimeError::OutOfRange)
        );
    }

    /// The refusals each documented error names: a Level 2 format where
    /// the Level 1 epoch is needed, a leap second that is not the last of a
    /// UTC day, a CCS P-field with more than six subsecond octets, and a
    /// T-field of the wrong length.
    #[test]
    fn the_documented_refusals() {
        let tai = CCSDS_CUC
            .instant()
            .checked_add(Duration::from_secs(1_000))
            .expect("fits");
        let level_2 = CucFormat::new(EpochLevel::AgencyDefined, 4, 0).expect("valid");
        assert_eq!(CucTime::from_tai(level_2, tai), Err(TimeError::OutOfRange));
        assert_eq!(
            CucTime::new(level_2, 1_000, 0).and_then(CucTime::to_tai),
            Err(TimeError::OutOfRange)
        );
        let ordinary = UtcInstant::from_unix(UnixTime::from_seconds(946_684_800));
        let cds_level_2 = CdsFormat {
            epoch: EpochLevel::AgencyDefined,
            ..CDS_MICROSECONDS
        };
        assert_eq!(
            CdsTime::from_utc(cds_level_2, ordinary),
            Err(TimeError::OutOfRange)
        );
        // The leap flag on an instant that is not the midnight after a
        // day's last second: 2016-12-31T12:00 UTC.
        let not_midnight = UtcInstant {
            unix_seconds: 1_483_228_800 - 43_200,
            leap_second: true,
            subsec_attos: 0,
        };
        assert_eq!(
            CdsTime::from_utc(CDS_MICROSECONDS, not_midnight),
            Err(TimeError::OutOfRange)
        );
        for subsecond_octets in [7, 8, u8::MAX] {
            let format = CcsFormat {
                variation: CcsVariation::DayOfYear,
                subsecond_octets,
            };
            assert_eq!(
                Preamble::Ccs(format).encode(),
                Err(TimeError::OutOfRange),
                "{subsecond_octets}"
            );
        }
        let format = cuc(4, 1);
        for length in [0, 4, 6] {
            assert_eq!(
                CucTime::decode(format, &[0; 6][..length]),
                Err(TimeError::OutOfRange),
                "CUC, {length} octets"
            );
        }
        assert_eq!(
            CucTime::decode_with_preamble(&[0x1D, 0x4E, 0xFF, 0xA2, 0x20]),
            Err(TimeError::OutOfRange)
        );
        for length in [7, 9] {
            assert_eq!(
                CdsTime::decode(CDS_MICROSECONDS, &[0; 9][..length]),
                Err(TimeError::OutOfRange),
                "CDS, {length} octets"
            );
        }
        assert_eq!(CDS_MICROSECONDS.t_field_octets(), 8);
    }

    /// The widest CUC code: seven octets of seconds and ten of fraction, in
    /// a two-octet P-field, with mission bits.
    #[test]
    fn the_widest_cuc_code() {
        let format = CucFormat {
            mission_bits: 0b10,
            ..CucFormat::new(EpochLevel::AgencyDefined, 7, 10).expect("valid")
        };
        let preamble = Preamble::Cuc(format).encode().expect("valid");
        assert_eq!(preamble.as_slice(), [0xAF, 0b0111_1110]);
        assert_eq!(
            Preamble::decode(preamble.as_slice()),
            Ok((Preamble::Cuc(format), 2))
        );
        let code = CucTime::new(format, (1 << 56) - 1, (1 << 80) - 1).expect("fits");
        let bytes = code.encode(true).expect("fits");
        assert_eq!(bytes.len(), Octets::CAPACITY);
        assert_eq!(CucTime::decode_with_preamble(bytes.as_slice()), Ok(code));
        assert_eq!(code.to_tai(), Err(TimeError::OutOfRange));
        assert_eq!(CucTime::new(format, 1 << 56, 0), Err(TimeError::OutOfRange));
        // 2⁸⁰ − 1 units of 2⁻⁸⁰ s round up to the next second.
        assert_eq!(code.since_epoch().subsec_attos(), 0);
        assert_eq!(code.since_epoch().whole_seconds(), 1 << 56);
    }

    /// A span converts to a count and back exactly while the unit is
    /// coarser than the attosecond, up to seven fine octets.
    #[test]
    fn round_trips() {
        let spans = [
            Duration::ZERO,
            Duration::from_attos(1),
            Duration::from_millis(1),
            Duration::new(1_325_376_032, 123_456_789_012_345_678).expect("valid"),
            Duration::new(4_294_967_295, 999_999_999_999_999_999).expect("valid"),
        ];
        for fine in 0..=10 {
            let format = cuc(5, fine);
            for span in spans {
                let code = CucTime::from_since_epoch(format, span).expect("fits");
                let bytes = code.encode(false).expect("fits");
                let back = CucTime::decode(format, bytes.as_slice()).expect("valid");
                assert_eq!(back, code);
                // The count is the unit at or before the span, and reads
                // back no later than the span's attosecond.
                assert!(back.since_epoch() <= span || fine > 7);
                let again = CucTime::from_since_epoch(format, back.since_epoch()).expect("fits");
                if fine <= 7 {
                    assert_eq!(again, code, "{fine} octets, {span:?}");
                }
            }
        }
        assert_eq!(
            CucTime::from_since_epoch(cuc(1, 0), Duration::from_secs(256)),
            Err(TimeError::OutOfRange)
        );
        assert_eq!(
            CucTime::from_since_epoch(cuc(1, 0), Duration::from_secs(-1)),
            Err(TimeError::OutOfRange)
        );
        // CDS in every format, over a sample of days from 1958 to 2027.
        for day in [0u32, 1, 5_113, 10_974, 21_549, 25_000] {
            for submillisecond in [
                Submillisecond::None,
                Submillisecond::Microseconds,
                Submillisecond::Picoseconds,
            ] {
                for width in [DaySegment::Bits16, DaySegment::Bits24] {
                    let format = CdsFormat {
                        epoch: EpochLevel::Recommended,
                        day: width,
                        submillisecond,
                    };
                    let (limit, _) = submillisecond.limit_and_unit();
                    let code = CdsTime::new(format, day, 43_200_001, limit).expect("valid");
                    let bytes = code.encode(true).expect("fits");
                    let back = CdsTime::decode_with_preamble(bytes.as_slice()).expect("valid");
                    assert_eq!(back, code);
                    let utc = code.to_utc().expect("an ordinary second");
                    assert_eq!(CdsTime::from_utc(format, utc), Ok(code));
                }
            }
        }
        let format = CdsFormat {
            submillisecond: Submillisecond::None,
            ..CDS_MICROSECONDS
        };
        assert_eq!(
            CdsTime::new(format, 65_536, 0, 0),
            Err(TimeError::OutOfRange)
        );
        assert_eq!(
            CdsTime::new(format, 0, 86_401_000, 0),
            Err(TimeError::OutOfRange)
        );
        assert_eq!(CdsTime::new(format, 0, 0, 1), Err(TimeError::OutOfRange));
        let before_1958 = UtcInstant::from_unix(UnixTime::from_seconds(-4_384 * 86_400));
        assert_eq!(
            CdsTime::from_utc(format, before_1958),
            Err(TimeError::OutOfRange)
        );
    }
}
