//! The values parsing produces and formatting consumes.
//!
//! # A parsed value is not an instant
//!
//! `2026-09-21T14:30:05` names a reading on somebody's wall clock. It does
//! not name a moment in time, and no amount of goodwill will make it name
//! one: the same text is 14 hours apart in Auckland and Honolulu. Treating an
//! unqualified local time as UTC is the single most common date bug in the
//! wild, so this crate refuses to do it. [`OffsetDateTime`] carries what the
//! text actually said — a local [`CivilDateTime`] plus a [`ZoneInfo`] that may
//! be [`ZoneInfo::Unspecified`] — and the conversions to an instant are
//! fallible for exactly that reason.
//!
//! # Round-tripping the written form
//!
//! The ISO 8601 value types ([`IsoDate`], [`IsoTime`], [`IsoDateTime`]) keep
//! the *shape* of the text as well as its meaning: basic or extended, how
//! many year digits, `.` or `,` for the decimal mark, `Z` or `+00:00`,
//! `24:00` or `00:00` of the next day. That is what lets formatting a parsed
//! value reproduce the input byte for byte.

use core::fmt;

use hc_calendar::{CivilDateTime, CivilTime, Rd};
use hc_calendars_solar::{gregorian, iso_week, ordinal};
use hc_core::unix::UtcInstant;
use hc_core::{ATTOS_PER_SEC, Duration, UnixTime};
use hc_tz::{Disambiguation, OffsetStyle, TimeZone, UtcOffset};

use crate::error::{ValueError, ValueResult};

/// The largest fraction this crate can hold, one attosecond short of a whole
/// unit.
const MAX_FRACTION_DIGITS: u8 = 18;

/// A decimal fraction of a unit, in `[0, 1)`.
///
/// The value is held as attos — eighteenths of a decimal place — because that
/// is [`hc_core::Duration`]'s resolution and converting through a float would
/// lose the low digits of a nanosecond timestamp. The digit count is kept
/// alongside so that `.500` and `.5` are distinguishable and both round-trip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fraction {
    attos: u64,
    digits: u8,
}

impl Fraction {
    /// Build a fraction from a scaled value and the digit count it was
    /// written with.
    ///
    /// Returns `None` when `attos` is not less than one whole unit or
    /// `digits` is outside `1..=18`.
    #[must_use]
    pub const fn new(attos: u64, digits: u8) -> Option<Self> {
        if attos >= ATTOS_PER_SEC || digits == 0 || digits > MAX_FRACTION_DIGITS {
            return None;
        }
        Some(Self { attos, digits })
    }

    /// The shortest exact spelling of a sub-unit remainder, or `None` for
    /// zero.
    ///
    /// Trailing zeros carry no information in a value this library produced
    /// itself, so the canonical rendering drops them: `.5`, not
    /// `.500000000000000000`.
    #[must_use]
    pub const fn minimal(attos: u64) -> Option<Self> {
        if attos == 0 || attos >= ATTOS_PER_SEC {
            return None;
        }
        let mut digits = MAX_FRACTION_DIGITS;
        let mut scale = 10u64;
        while digits > 1 && attos.is_multiple_of(scale) {
            digits -= 1;
            scale *= 10;
        }
        Some(Self { attos, digits })
    }

    /// The fraction scaled to attos of one unit.
    #[must_use]
    pub const fn attos(self) -> u64 {
        self.attos
    }

    /// How many digits the fraction was written with.
    #[must_use]
    pub const fn digits(self) -> u8 {
        self.digits
    }

    /// The fraction of a unit `unit_seconds` long, in attoseconds.
    ///
    /// Used to turn a fractional hour, minute or week into a span: `14.5`
    /// hours is this fraction of 3 600 seconds. The unit is given in seconds
    /// rather than attoseconds because a week in attoseconds multiplied by an
    /// attosecond-scaled fraction would overflow even a `u128`.
    #[must_use]
    pub const fn of_seconds(self, unit_seconds: u64) -> u128 {
        self.attos as u128 * unit_seconds as u128
    }

    /// Write the digits, without the leading decimal mark.
    ///
    /// # Errors
    ///
    /// Propagates the sink's failure.
    pub fn write_digits<W: fmt::Write>(self, out: &mut W) -> fmt::Result {
        let mut buffer = [b'0'; MAX_FRACTION_DIGITS as usize];
        let mut remainder = self.attos;
        for slot in buffer.iter_mut().rev() {
            *slot = b'0' + (remainder % 10) as u8;
            remainder /= 10;
        }
        for byte in buffer.iter().take(self.digits as usize) {
            out.write_char(char::from(*byte))?;
        }
        Ok(())
    }
}

/// Which decimal mark a fraction was written with.
///
/// ISO 8601 prefers the comma and permits the point; the internet profiles
/// permit only the point. Keeping which one appeared is what lets a
/// round trip reproduce it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DecimalMark {
    /// `14:30:05.5` — the point, universal in computing.
    #[default]
    Point,
    /// `14:30:05,5` — the comma, ISO 8601's stated preference.
    Comma,
}

impl DecimalMark {
    /// The character itself.
    #[must_use]
    pub const fn as_char(self) -> char {
        match self {
            Self::Point => '.',
            Self::Comma => ',',
        }
    }
}

/// Whether separators are written between components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Style {
    /// `20260921`, `143005`, `+0900` — no separators.
    Basic,
    /// `2026-09-21`, `14:30:05`, `+09:00` — hyphens and colons.
    #[default]
    Extended,
}

impl Style {
    /// Whether this style writes separators.
    #[must_use]
    pub const fn has_separators(self) -> bool {
        matches!(self, Self::Extended)
    }
}

/// How the year field was written.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum YearStyle {
    /// Four digits and no sign: `2026`. Only years `0000..=9999` fit.
    #[default]
    Plain,
    /// A sign and an agreed number of digits: `+002026`, `-000500`.
    ///
    /// ISO 8601 leaves the digit count to agreement between the parties, so
    /// it is carried here rather than fixed.
    Expanded(u8),
}

/// A date, in whichever of ISO 8601's three namings the text used.
///
/// Reduced accuracy is a first-class case: `2026` and `2026-09` are complete,
/// valid ISO 8601 dates that happen not to name a day, and squashing them to
/// 1 January or the 1st of the month would be inventing data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DateParts {
    /// `2026`, `2026-09`, `2026-09-21`.
    Calendar {
        /// The astronomical year: 1 BC is year 0.
        year: i64,
        /// The month, 1 through 12, when the text named one.
        month: Option<u8>,
        /// The day of the month, when the text named one.
        day: Option<u8>,
    },
    /// `2026-264`: the day counted from 1 January.
    Ordinal {
        /// The astronomical year.
        year: i64,
        /// The day of the year, 1 through 365 or 366.
        day_of_year: u16,
    },
    /// `2026-W38`, `2026-W38-1`.
    Week {
        /// The ISO week-numbering year, which may differ from the calendar
        /// year by one at either end.
        year: i64,
        /// The week, 1 through 52 or 53.
        week: u8,
        /// The ISO weekday, 1 for Monday through 7 for Sunday.
        weekday: Option<u8>,
    },
}

impl DateParts {
    /// The year field, whichever naming this is.
    ///
    /// For [`DateParts::Week`] this is the week-numbering year, which is not
    /// the calendar year of every day in it.
    #[must_use]
    pub const fn year(self) -> i64 {
        match self {
            Self::Calendar { year, .. } | Self::Ordinal { year, .. } | Self::Week { year, .. } => {
                year
            }
        }
    }

    /// Whether the value names one particular day.
    #[must_use]
    pub const fn names_a_day(self) -> bool {
        match self {
            Self::Calendar { day, .. } => day.is_some(),
            Self::Ordinal { .. } => true,
            Self::Week { weekday, .. } => weekday.is_some(),
        }
    }

    /// The fixed day this names.
    ///
    /// # Errors
    ///
    /// [`ValueError::ReducedAccuracy`] when the value names no single day,
    /// and [`ValueError::Calendar`] when the fields are out of range.
    pub fn to_fixed(self) -> ValueResult<Rd> {
        match self {
            Self::Calendar {
                year,
                month: Some(month),
                day: Some(day),
            } => Ok(gregorian::to_fixed(year, month, day)?),
            Self::Ordinal { year, day_of_year } => Ok(ordinal::to_fixed(year, day_of_year)?),
            Self::Week {
                year,
                week,
                weekday: Some(weekday),
            } => Ok(iso_week::to_fixed(year, week, weekday)?),
            _ => Err(ValueError::ReducedAccuracy),
        }
    }
}

/// A date together with the shape it was written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IsoDate {
    /// Which naming, and its fields.
    pub parts: DateParts,
    /// Basic or extended.
    pub style: Style,
    /// How the year was spelled.
    pub year_style: YearStyle,
}

impl IsoDate {
    /// A calendar date in the extended format with a plain four-digit year.
    #[must_use]
    pub const fn calendar(year: i64, month: u8, day: u8) -> Self {
        Self {
            parts: DateParts::Calendar {
                year,
                month: Some(month),
                day: Some(day),
            },
            style: Style::Extended,
            year_style: YearStyle::Plain,
        }
    }

    /// The canonical extended-format calendar date for a fixed day.
    ///
    /// Years outside `0000..=9999` are given the expanded form with as many
    /// digits as they need, because no other spelling of them exists.
    ///
    /// # Errors
    ///
    /// [`ValueError::Calendar`] outside the Gregorian module's range.
    pub fn from_fixed(rd: Rd) -> ValueResult<Self> {
        let (year, month, day) = gregorian::from_fixed(rd)?;
        let year_style = if (0..=9_999).contains(&year) {
            YearStyle::Plain
        } else {
            YearStyle::Expanded(expanded_digits(year))
        };
        Ok(Self {
            parts: DateParts::Calendar {
                year,
                month: Some(month),
                day: Some(day),
            },
            style: Style::Extended,
            year_style,
        })
    }

    /// The fixed day this names.
    ///
    /// # Errors
    ///
    /// See [`DateParts::to_fixed`].
    pub fn to_fixed(self) -> ValueResult<Rd> {
        self.parts.to_fixed()
    }

    /// Write the date.
    ///
    /// # Errors
    ///
    /// [`crate::FormatError::Unrepresentable`] when a plain four-digit year
    /// is asked to hold a year outside `0000..=9999`, and
    /// [`crate::FormatError::Sink`] when the sink refuses.
    pub fn write<W: fmt::Write>(self, out: &mut W) -> crate::FormatResult<()> {
        write_year(out, self.parts.year(), self.year_style)?;
        let separator = self.style.has_separators();
        match self.parts {
            DateParts::Calendar { month, day, .. } => {
                if let Some(month) = month {
                    if separator {
                        out.write_char('-')?;
                    }
                    write_fixed(out, u64::from(month), 2)?;
                    if let Some(day) = day {
                        if separator {
                            out.write_char('-')?;
                        }
                        write_fixed(out, u64::from(day), 2)?;
                    }
                }
            }
            DateParts::Ordinal { day_of_year, .. } => {
                if separator {
                    out.write_char('-')?;
                }
                write_fixed(out, u64::from(day_of_year), 3)?;
            }
            DateParts::Week { week, weekday, .. } => {
                if separator {
                    out.write_char('-')?;
                }
                out.write_char('W')?;
                write_fixed(out, u64::from(week), 2)?;
                if let Some(weekday) = weekday {
                    if separator {
                        out.write_char('-')?;
                    }
                    write_fixed(out, u64::from(weekday), 1)?;
                }
            }
        }
        Ok(())
    }
}

impl fmt::Display for IsoDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write(f).map_err(|_| fmt::Error)
    }
}

/// A time of day, together with the shape it was written in.
///
/// `hour` may be 24, which ISO 8601 allows only as `24:00:00` meaning the
/// end of the day. Every other component must then be zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IsoTime {
    /// The hour, 0 through 24.
    pub hour: u8,
    /// The minute, when the text named one.
    pub minute: Option<u8>,
    /// The second, when the text named one. May be 60 for a leap second.
    pub second: Option<u8>,
    /// The fraction of the *smallest component present*: of the second when
    /// seconds were written, of the minute when they were not, of the hour
    /// when neither was.
    pub fraction: Option<Fraction>,
    /// Basic or extended.
    pub style: Style,
    /// Which decimal mark the fraction used.
    pub mark: DecimalMark,
}

impl IsoTime {
    /// Midnight at the start of the day.
    pub const MIDNIGHT: Self = Self {
        hour: 0,
        minute: Some(0),
        second: Some(0),
        fraction: None,
        style: Style::Extended,
        mark: DecimalMark::Point,
    };

    /// `24:00:00`, the end of the day.
    pub const END_OF_DAY: Self = Self {
        hour: 24,
        minute: Some(0),
        second: Some(0),
        fraction: None,
        style: Style::Extended,
        mark: DecimalMark::Point,
    };

    /// The extended-format spelling of a civil time.
    ///
    /// The fraction is written only when there is one, with the fewest digits
    /// that state it exactly.
    #[must_use]
    pub fn from_civil(time: CivilTime) -> Self {
        Self {
            hour: time.hour(),
            minute: Some(time.minute()),
            second: Some(time.second()),
            fraction: Fraction::minimal(time.subsec_attos()),
            style: Style::Extended,
            mark: DecimalMark::Point,
        }
    }

    /// Whether this is the `24:00` end-of-day reading.
    #[must_use]
    pub const fn is_end_of_day(self) -> bool {
        self.hour == 24
    }

    /// Whether this names an inserted leap second.
    #[must_use]
    pub const fn is_leap_second(self) -> bool {
        matches!(self.second, Some(60))
    }

    /// The span from midnight, counting `24:00` as a full day and `23:59:60`
    /// as the 86 400th second.
    #[must_use]
    pub fn since_midnight(self) -> Duration {
        let unit = ATTOS_PER_SEC as u128;
        let mut attos = u128::from(self.hour) * 3_600 * unit;
        if let Some(minute) = self.minute {
            attos += u128::from(minute) * 60 * unit;
        }
        if let Some(second) = self.second {
            attos += u128::from(second) * unit;
        }
        if let Some(fraction) = self.fraction {
            let unit_seconds = match (self.minute, self.second) {
                (_, Some(_)) => 1,
                (Some(_), None) => 60,
                (None, None) => 3_600,
            };
            attos += fraction.of_seconds(unit_seconds);
        }
        Duration::from_attos(attos as i128)
    }

    /// The civil reading this names.
    ///
    /// # Errors
    ///
    /// [`ValueError::Calendar`] when the derived fields are not a civil time,
    /// which a parsed value never is.
    pub fn to_time_of_day(self) -> ValueResult<TimeOfDay> {
        if self.is_end_of_day() {
            return Ok(TimeOfDay::EndOfDay);
        }
        let span = self.since_midnight();
        let seconds = span.whole_seconds();
        let subsec = span.subsec_attos();
        if self.is_leap_second() {
            return Ok(TimeOfDay::Clock(CivilTime::new(23, 59, 60, subsec)?));
        }
        let hour = (seconds / 3_600) as u8;
        let minute = ((seconds % 3_600) / 60) as u8;
        let second = (seconds % 60) as u8;
        Ok(TimeOfDay::Clock(CivilTime::new(
            hour, minute, second, subsec,
        )?))
    }

    /// Write the time.
    ///
    /// # Errors
    ///
    /// [`crate::FormatError::Sink`] when the sink refuses.
    pub fn write<W: fmt::Write>(self, out: &mut W) -> crate::FormatResult<()> {
        let separator = self.style.has_separators();
        write_fixed(out, u64::from(self.hour), 2)?;
        if let Some(minute) = self.minute {
            if separator {
                out.write_char(':')?;
            }
            write_fixed(out, u64::from(minute), 2)?;
            if let Some(second) = self.second {
                if separator {
                    out.write_char(':')?;
                }
                write_fixed(out, u64::from(second), 2)?;
            }
        }
        if let Some(fraction) = self.fraction {
            out.write_char(self.mark.as_char())?;
            fraction.write_digits(out)?;
        }
        Ok(())
    }
}

impl fmt::Display for IsoTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write(f).map_err(|_| fmt::Error)
    }
}

/// A reading on a 24-hour clock, or the moment the clock runs out.
///
/// `24:00` of one day and `00:00` of the next are the same instant but not
/// the same statement — one is the end of a span, the other the start of one
/// — so the two are kept apart rather than normalised.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeOfDay {
    /// An ordinary reading, possibly the leap second `23:59:60`.
    Clock(CivilTime),
    /// `24:00`: midnight ending the day it was written on.
    EndOfDay,
}

impl TimeOfDay {
    /// The reading as a civil time, and whether it belongs to the next day.
    #[must_use]
    pub const fn normalise(self) -> (CivilTime, bool) {
        match self {
            Self::Clock(time) => (time, false),
            Self::EndOfDay => (CivilTime::MIDNIGHT, true),
        }
    }
}

/// What the text said about the zone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ZoneInfo {
    /// No designator at all: a local reading whose offset the text does not
    /// state. This is *not* UTC, and this crate will not pretend it is.
    #[default]
    Unspecified,
    /// The `Z` designator: UTC, stated as such.
    Zulu,
    /// A numeric offset such as `+09:00`.
    Offset(UtcOffset),
    /// RFC 3339 §4.3's `-00:00`: the instant is known and is UTC, but the
    /// offset of the local zone it was generated in is not known.
    ///
    /// This is deliberately distinct from `+00:00`, which asserts that the
    /// local zone really is at UTC. RFC 3339 spells out the difference and
    /// email headers rely on it; collapsing the two loses a fact.
    UnknownLocalOffset,
}

impl ZoneInfo {
    /// The numeric offset, or `None` when the text stated no zone.
    #[must_use]
    pub const fn offset(self) -> Option<UtcOffset> {
        match self {
            Self::Unspecified => None,
            Self::Zulu | Self::UnknownLocalOffset => Some(UtcOffset::UTC),
            Self::Offset(offset) => Some(offset),
        }
    }

    /// Whether the text stated a zone at all.
    #[must_use]
    pub const fn is_qualified(self) -> bool {
        !matches!(self, Self::Unspecified)
    }

    /// Write the designator in the style asked for.
    ///
    /// # Errors
    ///
    /// [`crate::FormatError::Sink`] when the sink refuses.
    pub fn write<W: fmt::Write>(self, out: &mut W, style: OffsetStyle) -> crate::FormatResult<()> {
        match self {
            Self::Unspecified => {}
            Self::Zulu => out.write_char('Z')?,
            Self::UnknownLocalOffset => {
                out.write_char('-')?;
                out.write_str("00")?;
                if !matches!(style, OffsetStyle::Hours) {
                    if matches!(style, OffsetStyle::Extended | OffsetStyle::ExtendedSeconds) {
                        out.write_char(':')?;
                    }
                    out.write_str("00")?;
                }
            }
            Self::Offset(offset) => out.write_str(offset.format(style).as_str())?,
        }
        Ok(())
    }
}

/// A date-time together with the shape it was written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IsoDateTime {
    /// The date, which may be of reduced accuracy.
    pub date: IsoDate,
    /// The time, when the text carried one.
    pub time: Option<IsoTime>,
    /// What the text said about the zone.
    pub zone: ZoneInfo,
    /// Which spelling the numeric offset used.
    pub zone_style: OffsetStyle,
}

impl IsoDateTime {
    /// A date on its own, with no time and no zone.
    #[must_use]
    pub const fn from_date(date: IsoDate) -> Self {
        Self {
            date,
            time: None,
            zone: ZoneInfo::Unspecified,
            zone_style: OffsetStyle::Extended,
        }
    }

    /// The extended-format spelling of a civil date-time in a stated zone.
    ///
    /// # Errors
    ///
    /// [`ValueError::Calendar`] outside the Gregorian range.
    pub fn from_civil(local: CivilDateTime, zone: ZoneInfo) -> ValueResult<Self> {
        Ok(Self {
            date: IsoDate::from_fixed(local.day)?,
            time: Some(IsoTime::from_civil(local.time)),
            zone,
            zone_style: OffsetStyle::Extended,
        })
    }

    /// Resolve to a local civil reading plus whatever zone the text stated.
    ///
    /// # Errors
    ///
    /// [`ValueError::ReducedAccuracy`] when no day is named and
    /// [`ValueError::MissingTime`] when no time is.
    pub fn to_offset_date_time(self) -> ValueResult<OffsetDateTime> {
        let day = self.date.to_fixed()?;
        let time = self.time.ok_or(ValueError::MissingTime)?;
        let (clock, next_day) = time.to_time_of_day()?.normalise();
        let day = if next_day {
            day.checked_add_days(1)?
        } else {
            day
        };
        Ok(OffsetDateTime {
            local: CivilDateTime::new(day, clock),
            zone: self.zone,
            written_as_end_of_day: next_day,
        })
    }

    /// Write the date-time.
    ///
    /// # Errors
    ///
    /// See [`IsoDate::write`].
    pub fn write<W: fmt::Write>(self, out: &mut W) -> crate::FormatResult<()> {
        self.date.write(out)?;
        if let Some(time) = self.time {
            out.write_char('T')?;
            time.write(out)?;
        }
        self.zone.write(out, self.zone_style)
    }
}

impl fmt::Display for IsoDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write(f).map_err(|_| fmt::Error)
    }
}

/// A local civil date-time plus what the text said about its zone.
///
/// This is the type a caller who "just wants the date" should reach for. It
/// is honest about the one thing that matters: without a zone it is a
/// reading, not an instant, and every method that produces an instant says so
/// in its signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OffsetDateTime {
    /// The local reading. A `24:00` in the source has already been moved to
    /// `00:00` of the following day.
    pub local: CivilDateTime,
    /// What the text said about the zone.
    pub zone: ZoneInfo,
    /// Whether that midnight was written as `24:00` of the previous day.
    pub written_as_end_of_day: bool,
}

impl OffsetDateTime {
    /// A reading with no zone.
    #[must_use]
    pub const fn local(local: CivilDateTime) -> Self {
        Self {
            local,
            zone: ZoneInfo::Unspecified,
            written_as_end_of_day: false,
        }
    }

    /// A reading with a numeric offset.
    #[must_use]
    pub const fn with_offset(local: CivilDateTime, offset: UtcOffset) -> Self {
        Self {
            local,
            zone: ZoneInfo::Offset(offset),
            written_as_end_of_day: false,
        }
    }

    /// The reading a POSIX timestamp gives under a stated zone designator.
    ///
    /// # Errors
    ///
    /// [`ValueError::MissingZone`] when `zone` is
    /// [`ZoneInfo::Unspecified`] — there would be nothing to read the
    /// timestamp against — and [`ValueError::Zone`] on overflow.
    pub fn from_unix(unix: UnixTime, zone: ZoneInfo) -> ValueResult<Self> {
        let offset = zone.offset().ok_or(ValueError::MissingZone)?;
        Ok(Self {
            local: hc_tz::zone::local_from_unix(unix, offset)?,
            zone,
            written_as_end_of_day: false,
        })
    }

    /// The reading a POSIX timestamp gives in a time zone.
    ///
    /// # Errors
    ///
    /// [`ValueError::Zone`] when the local day leaves the representable
    /// range.
    pub fn from_unix_in<Z: TimeZone>(unix: UnixTime, zone: &Z) -> ValueResult<Self> {
        let offset = zone.offset_at(unix);
        Ok(Self {
            local: hc_tz::zone::local_from_unix(unix, offset)?,
            zone: ZoneInfo::Offset(offset),
            written_as_end_of_day: false,
        })
    }

    /// The same moment read in UTC.
    ///
    /// # Errors
    ///
    /// [`ValueError::MissingZone`] when the text stated no zone.
    pub fn to_utc_civil(self) -> ValueResult<CivilDateTime> {
        let offset = self.zone.offset().ok_or(ValueError::MissingZone)?;
        Ok(offset.utc_from_local(self.local)?)
    }

    /// The POSIX timestamp of this reading.
    ///
    /// A `23:59:60` maps to the timestamp of the following second, which is
    /// what POSIX time does with a leap second: the mapping is not injective
    /// and cannot be made so. Use [`OffsetDateTime::to_utc_instant`] when the
    /// leap flag matters.
    ///
    /// # Errors
    ///
    /// [`ValueError::MissingZone`] when the text stated no zone, and
    /// [`ValueError::Zone`] on overflow.
    pub fn to_unix(self) -> ValueResult<UnixTime> {
        let offset = self.zone.offset().ok_or(ValueError::MissingZone)?;
        Ok(hc_tz::zone::unix_from_local(self.local, offset)?)
    }

    /// The UTC instant, leap-second flag included.
    ///
    /// # Errors
    ///
    /// See [`OffsetDateTime::to_unix`].
    pub fn to_utc_instant(self) -> ValueResult<UtcInstant> {
        let unix = self.to_unix()?;
        Ok(UtcInstant {
            unix_seconds: unix.seconds(),
            leap_second: self.local.time.is_leap_second(),
            subsec_attos: unix.subsec_attos(),
        })
    }

    /// The instant this reading names in a zone, under a disambiguation
    /// policy.
    ///
    /// This is the method for an unqualified local time: the text did not say
    /// what zone it meant, so the caller must.
    ///
    /// # Errors
    ///
    /// [`ValueError::UnresolvableLocalTime`] when the reading is ambiguous or
    /// nonexistent in the zone and the policy is
    /// [`Disambiguation::Reject`].
    pub fn to_unix_in<Z: TimeZone>(
        self,
        zone: &Z,
        policy: Disambiguation,
    ) -> ValueResult<UnixTime> {
        zone.resolve_local(self.local).resolve(policy).map_err(|_| {
            // The only two failures `resolve` reports are the two the policy
            // asked to be told about; everything else is infallible here.
            ValueError::UnresolvableLocalTime
        })
    }
}

/// The number of digits an expanded year needs, at least five.
///
/// ISO 8601 leaves the count to agreement; six is the customary choice and is
/// what this uses for anything that fits, which covers every year the
/// Gregorian module supports below a million.
pub(crate) const fn expanded_digits(year: i64) -> u8 {
    let magnitude = year.unsigned_abs();
    let mut digits = 6u8;
    let mut limit = 1_000_000u64;
    while magnitude >= limit && digits < 18 {
        digits += 1;
        limit *= 10;
    }
    digits
}

/// Write a year in the requested spelling.
pub(crate) fn write_year<W: fmt::Write>(
    out: &mut W,
    year: i64,
    style: YearStyle,
) -> crate::FormatResult<()> {
    match style {
        YearStyle::Plain => {
            if !(0..=9_999).contains(&year) {
                return Err(crate::FormatError::Unrepresentable(
                    "a year outside 0000..=9999 in a four-digit field",
                ));
            }
            write_fixed(out, year as u64, 4)?;
        }
        YearStyle::Expanded(digits) => {
            out.write_char(if year < 0 { '-' } else { '+' })?;
            write_fixed(out, year.unsigned_abs(), usize::from(digits))?;
        }
    }
    Ok(())
}

/// Write a zero-padded decimal of fixed width.
pub(crate) fn write_fixed<W: fmt::Write>(out: &mut W, value: u64, width: usize) -> fmt::Result {
    write!(out, "{value:0width$}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::{String, ToString as _};

    fn render(time: IsoTime) -> String {
        let mut out = String::new();
        time.write(&mut out).unwrap();
        out
    }

    #[test]
    fn the_shortest_exact_fraction_drops_trailing_zeros() {
        let half = Fraction::minimal(ATTOS_PER_SEC / 2).unwrap();
        assert_eq!(half.digits(), 1);
        let mut out = String::new();
        half.write_digits(&mut out).unwrap();
        assert_eq!(out, "5");
    }

    #[test]
    fn a_zero_remainder_has_no_fraction_at_all() {
        assert!(Fraction::minimal(0).is_none());
    }

    #[test]
    fn a_fraction_of_an_hour_becomes_half_past() {
        let time = IsoTime {
            hour: 14,
            minute: None,
            second: None,
            fraction: Fraction::new(ATTOS_PER_SEC / 2, 1),
            style: Style::Extended,
            mark: DecimalMark::Point,
        };
        assert_eq!(time.since_midnight(), Duration::from_secs(14 * 3600 + 1800));
        assert_eq!(
            time.to_time_of_day().unwrap(),
            TimeOfDay::Clock(CivilTime::hms(14, 30, 0).unwrap())
        );
    }

    #[test]
    fn end_of_day_is_not_the_same_statement_as_midnight() {
        assert_eq!(
            IsoTime::END_OF_DAY.to_time_of_day().unwrap(),
            TimeOfDay::EndOfDay
        );
        assert_eq!(render(IsoTime::END_OF_DAY), "24:00:00");
        assert_ne!(IsoTime::END_OF_DAY, IsoTime::MIDNIGHT);
    }

    #[test]
    fn the_leap_second_survives_the_round_trip_through_a_civil_time() {
        let time = IsoTime {
            hour: 23,
            minute: Some(59),
            second: Some(60),
            fraction: None,
            style: Style::Extended,
            mark: DecimalMark::Point,
        };
        let TimeOfDay::Clock(clock) = time.to_time_of_day().unwrap() else {
            panic!("a leap second is a clock reading");
        };
        assert!(clock.is_leap_second());
        assert_eq!(render(IsoTime::from_civil(clock)), "23:59:60");
    }

    #[test]
    fn an_unqualified_reading_names_no_instant() {
        let value = OffsetDateTime::local(CivilDateTime::midnight(Rd(739_000)));
        assert_eq!(value.to_unix().unwrap_err(), ValueError::MissingZone);
    }

    #[test]
    fn minus_zero_is_utc_but_not_plus_zero() {
        assert_eq!(ZoneInfo::UnknownLocalOffset.offset(), Some(UtcOffset::UTC));
        assert_ne!(ZoneInfo::UnknownLocalOffset, ZoneInfo::Zulu);
        let mut out = String::new();
        ZoneInfo::UnknownLocalOffset
            .write(&mut out, OffsetStyle::Extended)
            .unwrap();
        assert_eq!(out, "-00:00");
    }

    #[test]
    fn a_reduced_date_refuses_to_name_a_day() {
        let date = IsoDate {
            parts: DateParts::Calendar {
                year: 2026,
                month: Some(9),
                day: None,
            },
            style: Style::Extended,
            year_style: YearStyle::Plain,
        };
        assert_eq!(date.to_fixed().unwrap_err(), ValueError::ReducedAccuracy);
        assert_eq!(date.to_string(), "2026-09");
    }

    #[test]
    fn expanded_years_grow_only_as_far_as_they_must() {
        assert_eq!(expanded_digits(2026), 6);
        assert_eq!(expanded_digits(-1_000_000), 7);
    }

    #[test]
    fn a_four_digit_field_refuses_a_negative_year() {
        let mut out = String::new();
        let error = write_year(&mut out, -1, YearStyle::Plain).unwrap_err();
        assert!(matches!(error, crate::FormatError::Unrepresentable(_)));
    }
}
