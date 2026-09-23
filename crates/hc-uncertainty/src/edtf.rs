//! ISO 8601-2 Extended Date/Time Format, levels 0 to 2.
//!
//! EDTF is the interchange format for exactly the problem this crate exists
//! for: it is how libraries, archives and museums write down a date they do
//! not fully know. `1984?` is doubted, `1984~` is approximate, `1984%` is
//! both, `1984-01-XX` names a month but not a day, `Y-170000002` is a year
//! far outside the four-digit range, `1984/1985` is an interval,
//! `[1667,1668,1670..1672]` is "one of these", `..1760-12-03` is "no later
//! than", and `1760-12..` is "no earlier than".
//!
//! Parsing produces an [`EdtfValue`]; [`EdtfValue::to_fuzzy_instant`] turns
//! it into a [`crate::FuzzyInstant`] so that it can be reasoned about, and
//! [`core::fmt::Display`] turns it back into the same string it came from.
//!
//! # Scope
//!
//! Supported: level 0 dates and intervals, level 1 qualifiers, unspecified
//! digits, the `Y` long-year form and the open/unknown interval endpoints,
//! and the level 2 set and list forms (which need `alloc`).
//!
//! Deliberately not supported, and rejected rather than half-parsed:
//!
//! * **Times of day.** `1985-04-12T23:20:30Z` is legal EDTF; this module is
//!   about *which day*, and a crate that already has
//!   [`hc_core::Instant`] should not grow a second, weaker time parser.
//! * **Seasons and sub-year divisions** (`2001-21` for spring, `2001-34` for
//!   a quarter). Their boundaries are conventions that differ by hemisphere
//!   and by publisher, and guessing one would be inventing data.
//! * **Component-level qualification** (`2004-06~-11`, "June is approximate
//!   but the year and day are not"). The support it implies is not an
//!   interval, so it cannot be represented faithfully here.
//! * **Exponential years and significant digits** (`Y17E7S3`).
//!
//! # Why there is calendar arithmetic in an uncertainty crate
//!
//! Placing `1984-01-01` on a timeline needs proleptic Gregorian day
//! arithmetic, which properly belongs to `hc-calendar`. This crate depends
//! only on `hc-core`, so it carries a private, round-trip-tested copy of the
//! two Rata Die formulas from Reingold and Dershowitz, *Calendrical
//! Calculations*, 4th ed., §2.3. It is not re-exported, and nothing else
//! should use it.
//!
//! # Accuracy of the placement
//!
//! Instants are produced by counting 86 400-second days from the 1970 epoch
//! on the TAI scale. That ignores leap seconds, so a converted EDTF date is
//! displaced from true TAI by `TAI - UTC` — under 40 seconds since UTC
//! began in 1961, and undefined before that. At EDTF's
//! coarsest useful resolution of one day this is irrelevant, and making it
//! exact would require a UTC table that only covers 1 % of the range EDTF
//! can express.

use core::fmt;
use core::str::FromStr;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use hc_core::{Duration, Instant, Tai};

use crate::error::{UncertaintyError, UncertaintyResult};
use crate::fuzzy::FuzzyInstant;

/// Rata Die of `1970-01-01`, the day the `hc-core` epoch falls on.
const RD_OF_UNIX_EPOCH: i64 = 719_163;

/// How sure the writer of an EDTF value was.
///
/// The distinction is the standard's: `?` doubts the assertion while naming a
/// definite date, `~` names a date that is only approximately right, and `%`
/// does both. Only the approximate forms widen the support, because only they
/// say the value itself may be off.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EdtfQualifier {
    /// No qualifier: the date is asserted as written.
    #[default]
    Certain,
    /// `?` — the date is doubted but not blurred.
    Uncertain,
    /// `~` — the date is approximate.
    Approximate,
    /// `%` — both doubted and approximate.
    UncertainAndApproximate,
}

impl EdtfQualifier {
    /// The EDTF character, or the empty string for [`EdtfQualifier::Certain`].
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Certain => "",
            Self::Uncertain => "?",
            Self::Approximate => "~",
            Self::UncertainAndApproximate => "%",
        }
    }

    /// Whether the qualifier blurs the value rather than merely doubting it.
    #[must_use]
    pub const fn is_approximate(self) -> bool {
        matches!(self, Self::Approximate | Self::UncertainAndApproximate)
    }
}

/// One month or day component of an EDTF date.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdtfComponent {
    /// The component is not written at all — `1984` has no month.
    Absent,
    /// The component is written as `XX`: it exists but is not known.
    Unspecified,
    /// The component is written out.
    Known(u8),
}

impl EdtfComponent {
    /// The value, when there is one.
    #[must_use]
    pub const fn value(self) -> Option<u8> {
        match self {
            Self::Known(value) => Some(value),
            Self::Absent | Self::Unspecified => None,
        }
    }

    /// Whether the component appears in the string at all.
    #[must_use]
    pub const fn is_present(self) -> bool {
        !matches!(self, Self::Absent)
    }
}

/// How coarse an EDTF date is.
///
/// The ordering runs from coarsest to finest, so `Year < Day`, and a
/// comparison reads as "is at least as precise as".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EdtfPrecision {
    /// `XXXX` — not even the millennium.
    Unknown,
    /// `1XXX` — the millennium is known and nothing finer.
    Millennium,
    /// `19XX` — the century.
    Century,
    /// `198X` — the decade.
    Decade,
    /// `1984`, or `1984-XX`, or `1984-XX-XX`.
    Year,
    /// `1984-01`, or `1984-01-XX`.
    Month,
    /// `1984-01-01`.
    Day,
}

/// A single EDTF date: a year, optionally a month, optionally a day, with
/// unspecified digits and a qualifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EdtfDate {
    year: i64,
    unspecified_year_digits: u8,
    month: EdtfComponent,
    day: EdtfComponent,
    qualifier: EdtfQualifier,
    long_form: bool,
}

impl EdtfDate {
    /// A year-precision date such as `1984`.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::InvalidSyntax`] when the year needs more
    /// than four digits; use [`EdtfDate::long_year`] for those.
    pub fn year(year: i64) -> UncertaintyResult<Self> {
        if !(-9999..=9999).contains(&year) {
            return Err(UncertaintyError::InvalidSyntax(
                "a four-digit year; use the Y form for longer ones",
            ));
        }
        Ok(Self {
            year,
            unspecified_year_digits: 0,
            month: EdtfComponent::Absent,
            day: EdtfComponent::Absent,
            qualifier: EdtfQualifier::Certain,
            long_form: false,
        })
    }

    /// A year outside the four-digit range, rendered with the `Y` prefix.
    ///
    /// # Errors
    ///
    /// Never fails today; the signature is reserved for the range checks a
    /// future exponential-year form would need.
    pub const fn long_year(year: i64) -> UncertaintyResult<Self> {
        Ok(Self {
            year,
            unspecified_year_digits: 0,
            month: EdtfComponent::Absent,
            day: EdtfComponent::Absent,
            qualifier: EdtfQualifier::Certain,
            long_form: true,
        })
    }

    /// A day-precision date such as `1984-01-01`.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::InvalidSyntax`] for an out-of-range year,
    /// month or day. The day is checked against the actual length of the
    /// month, so `1984-02-30` is rejected and `1984-02-29` is not.
    pub fn ymd(year: i64, month: u8, day: u8) -> UncertaintyResult<Self> {
        let mut date = Self::year(year)?;
        date.month = EdtfComponent::Known(month);
        date.day = EdtfComponent::Known(day);
        date.validate()?;
        Ok(date)
    }

    /// A month-precision date such as `1984-01`.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::InvalidSyntax`] for an out-of-range year
    /// or month.
    pub fn year_month(year: i64, month: u8) -> UncertaintyResult<Self> {
        let mut date = Self::year(year)?;
        date.month = EdtfComponent::Known(month);
        date.validate()?;
        Ok(date)
    }

    /// The same date with a qualifier attached.
    #[must_use]
    pub const fn with_qualifier(mut self, qualifier: EdtfQualifier) -> Self {
        self.qualifier = qualifier;
        self
    }

    /// The year as written, with any unspecified digits read as zero.
    #[must_use]
    pub const fn year_value(self) -> i64 {
        self.year
    }

    /// How many of the four year digits are `X`.
    #[must_use]
    pub const fn unspecified_year_digits(self) -> u8 {
        self.unspecified_year_digits
    }

    /// The month component.
    #[must_use]
    pub const fn month(self) -> EdtfComponent {
        self.month
    }

    /// The day component.
    #[must_use]
    pub const fn day(self) -> EdtfComponent {
        self.day
    }

    /// The qualifier.
    #[must_use]
    pub const fn qualifier(self) -> EdtfQualifier {
        self.qualifier
    }

    /// Whether the year is written in the `Y` long form.
    #[must_use]
    pub const fn is_long_form(self) -> bool {
        self.long_form
    }

    /// How coarse the date is.
    #[must_use]
    pub const fn precision(self) -> EdtfPrecision {
        match self.unspecified_year_digits {
            4 => EdtfPrecision::Unknown,
            3 => EdtfPrecision::Millennium,
            2 => EdtfPrecision::Century,
            1 => EdtfPrecision::Decade,
            _ => match (self.month, self.day) {
                (EdtfComponent::Known(_), EdtfComponent::Known(_)) => EdtfPrecision::Day,
                (EdtfComponent::Known(_), _) => EdtfPrecision::Month,
                _ => EdtfPrecision::Year,
            },
        }
    }

    /// Check that every component is in range for the others.
    fn validate(self) -> UncertaintyResult<()> {
        if self.unspecified_year_digits > 4 {
            return Err(UncertaintyError::InvalidSyntax("at most four X digits"));
        }
        if let Some(month) = self.month.value()
            && !(1..=12).contains(&month)
        {
            return Err(UncertaintyError::InvalidSyntax("a month in 01..12"));
        }
        if let Some(day) = self.day.value() {
            if !(1..=31).contains(&day) {
                return Err(UncertaintyError::InvalidSyntax("a day in 01..31"));
            }
            if self.unspecified_year_digits == 0
                && let Some(month) = self.month.value()
                && day > days_in_month(self.year, month)
            {
                return Err(UncertaintyError::InvalidSyntax("a day that month has"));
            }
        }
        if self.day.is_present() && !self.month.is_present() {
            return Err(UncertaintyError::InvalidSyntax("a month before a day"));
        }
        Ok(())
    }

    /// The first and last fixed days the date could denote, inclusive.
    ///
    /// Unspecified digits widen the range; an unspecified month with a
    /// specified day yields the *hull* of the twelve possibilities, which is
    /// wider than the true support and is documented as such.
    fn day_range(self) -> (i64, i64) {
        let span = pow10(self.unspecified_year_digits);
        let low_year = self.year;
        let high_year = self.year + span - 1;
        let low_month = self.month.value().unwrap_or(1);
        let high_month = self.month.value().unwrap_or(12);
        let low_day = self.day.value().unwrap_or(1);
        let high_day = self
            .day
            .value()
            .unwrap_or_else(|| days_in_month(high_year, high_month));
        (
            fixed_from_gregorian(low_year, low_month, low_day),
            fixed_from_gregorian(high_year, high_month, high_day),
        )
    }

    /// The first instant the date could denote.
    fn start_instant(self) -> Instant<Tai> {
        instant_of_day(self.day_range().0)
    }

    /// The instant one day after the last day the date could denote, which
    /// is the exclusive end of the range and the closed upper bound of the
    /// support.
    fn end_instant(self) -> Instant<Tai> {
        instant_of_day(self.day_range().1 + 1)
    }

    /// The date as a fuzzy instant.
    ///
    /// An approximate qualifier widens the support by its own span on each
    /// side, so `1984~` covers 1983 to 1985. That factor is a convention of
    /// this crate: EDTF says a value is approximate but not by how much, and
    /// one unit of the stated precision is the least surprising reading.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::Overflow`] when the span leaves the
    /// representable range.
    pub fn to_fuzzy_instant(self) -> UncertaintyResult<FuzzyInstant> {
        let start = self.start_instant();
        let end = self.end_instant();
        if !self.qualifier.is_approximate() {
            let resolution = end.since_epoch().checked_sub(start.since_epoch())?;
            return FuzzyInstant::resolved(start, resolution);
        }
        let span = end.since_epoch().checked_sub(start.since_epoch())?;
        FuzzyInstant::bounded(start.checked_sub(span)?, end.checked_add(span)?)
    }

    /// Parse one date, with no interval or set syntax around it.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::InvalidSyntax`] describing what was
    /// expected.
    pub fn parse(text: &str) -> UncertaintyResult<Self> {
        if !text.is_ascii() {
            return Err(UncertaintyError::InvalidSyntax("ASCII digits"));
        }
        let (body, qualifier) = split_qualifier(text);
        if body.is_empty() {
            return Err(UncertaintyError::InvalidSyntax("a date"));
        }
        if let Some(rest) = body.strip_prefix('Y') {
            let year = parse_signed_integer(rest)?;
            let mut date = Self::long_year(year)?;
            date.qualifier = qualifier;
            return Ok(date);
        }
        let (negative, digits) = match body.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, body),
        };
        let mut parts = digits.split('-');
        let year_part = parts
            .next()
            .ok_or(UncertaintyError::InvalidSyntax("a year"))?;
        let (magnitude, unspecified) = parse_year_digits(year_part)?;
        let year = if negative { -magnitude } else { magnitude };
        let month = match parts.next() {
            Some(part) => parse_two_digit_component(part)?,
            None => EdtfComponent::Absent,
        };
        let day = match parts.next() {
            Some(part) => parse_two_digit_component(part)?,
            None => EdtfComponent::Absent,
        };
        if parts.next().is_some() {
            return Err(UncertaintyError::InvalidSyntax("at most three components"));
        }
        let date = Self {
            year,
            unspecified_year_digits: unspecified,
            month,
            day,
            qualifier,
            long_form: false,
        };
        date.validate()?;
        Ok(date)
    }
}

impl fmt::Display for EdtfDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.long_form {
            write!(f, "Y{}", self.year)?;
        } else {
            if self.year < 0 {
                f.write_str("-")?;
            }
            let mut digits = [b'0'; 4];
            let mut magnitude = self.year.unsigned_abs();
            for slot in digits.iter_mut().rev() {
                *slot = b'0' + (magnitude % 10) as u8;
                magnitude /= 10;
            }
            for index in 0..usize::from(self.unspecified_year_digits.min(4)) {
                digits[3 - index] = b'X';
            }
            f.write_str(core::str::from_utf8(&digits).unwrap_or("????"))?;
        }
        write_component(f, self.month)?;
        write_component(f, self.day)?;
        f.write_str(self.qualifier.symbol())
    }
}

impl FromStr for EdtfDate {
    type Err = UncertaintyError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::parse(text)
    }
}

/// One endpoint of an EDTF interval written with `/`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdtfEndpoint {
    /// A date.
    Date(EdtfDate),
    /// `..` — the interval is open on this side: it genuinely continues.
    Open,
    /// An empty component — the endpoint exists but is not recorded.
    Unknown,
}

impl fmt::Display for EdtfEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Date(date) => write!(f, "{date}"),
            Self::Open => f.write_str(".."),
            Self::Unknown => Ok(()),
        }
    }
}

/// One member of an EDTF level 2 set or list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdtfSetMember {
    /// A single date.
    Date(EdtfDate),
    /// An inclusive run, `1670..1672`.
    Range {
        /// The first date of the run.
        start: EdtfDate,
        /// The last date of the run.
        end: EdtfDate,
    },
    /// `..1672` — every date up to and including this one.
    EarlierThan(EdtfDate),
    /// `1670..` — every date from this one onwards.
    LaterThan(EdtfDate),
}

impl EdtfSetMember {
    /// The first and last fixed days this member could denote.
    ///
    /// Only reachable through a set, which needs `alloc`.
    #[cfg(feature = "alloc")]
    fn day_range(self) -> (Option<i64>, Option<i64>) {
        match self {
            Self::Date(date) => {
                let (low, high) = date.day_range();
                (Some(low), Some(high))
            }
            Self::Range { start, end } => (Some(start.day_range().0), Some(end.day_range().1)),
            Self::EarlierThan(date) => (None, Some(date.day_range().1)),
            Self::LaterThan(date) => (Some(date.day_range().0), None),
        }
    }

    /// Parse one member of a set.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::InvalidSyntax`] describing what was
    /// expected.
    pub fn parse(text: &str) -> UncertaintyResult<Self> {
        let trimmed = text.trim();
        if let Some(rest) = trimmed.strip_prefix("..") {
            return Ok(Self::EarlierThan(EdtfDate::parse(rest)?));
        }
        if let Some(rest) = trimmed.strip_suffix("..") {
            return Ok(Self::LaterThan(EdtfDate::parse(rest)?));
        }
        if let Some((start, end)) = trimmed.split_once("..") {
            return Ok(Self::Range {
                start: EdtfDate::parse(start)?,
                end: EdtfDate::parse(end)?,
            });
        }
        Ok(Self::Date(EdtfDate::parse(trimmed)?))
    }
}

impl fmt::Display for EdtfSetMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Date(date) => write!(f, "{date}"),
            Self::Range { start, end } => write!(f, "{start}..{end}"),
            Self::EarlierThan(date) => write!(f, "..{date}"),
            Self::LaterThan(date) => write!(f, "{date}.."),
        }
    }
}

/// A parsed EDTF string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EdtfValue {
    /// A single date.
    Date(EdtfDate),
    /// An interval written with `/`.
    Interval {
        /// The earlier endpoint.
        start: EdtfEndpoint,
        /// The later endpoint.
        end: EdtfEndpoint,
    },
    /// `..1760-12-03` — no later than this date.
    EarlierThan(EdtfDate),
    /// `1760-12..` — no earlier than this date.
    LaterThan(EdtfDate),
    /// `[...]` — exactly one of these is the date.
    #[cfg(feature = "alloc")]
    OneOf(Vec<EdtfSetMember>),
    /// `{...}` — all of these apply to the thing being dated.
    #[cfg(feature = "alloc")]
    AllOf(Vec<EdtfSetMember>),
}

impl EdtfValue {
    /// Parse an EDTF string.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::InvalidSyntax`] describing what was
    /// expected, or [`UncertaintyError::Unsupported`] for a set or list form
    /// in a build without `alloc`.
    pub fn parse(text: &str) -> UncertaintyResult<Self> {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return Err(UncertaintyError::InvalidSyntax("an EDTF value"));
        }
        if let Some(body) = trimmed.strip_prefix('[') {
            let body = body
                .strip_suffix(']')
                .ok_or(UncertaintyError::InvalidSyntax("a closing ]"))?;
            return parse_set(body, true);
        }
        if let Some(body) = trimmed.strip_prefix('{') {
            let body = body
                .strip_suffix('}')
                .ok_or(UncertaintyError::InvalidSyntax("a closing }"))?;
            return parse_set(body, false);
        }
        if let Some((start, end)) = trimmed.split_once('/') {
            return Ok(Self::Interval {
                start: parse_endpoint(start)?,
                end: parse_endpoint(end)?,
            });
        }
        if let Some(rest) = trimmed.strip_prefix("..") {
            return Ok(Self::EarlierThan(EdtfDate::parse(rest)?));
        }
        if let Some(rest) = trimmed.strip_suffix("..") {
            return Ok(Self::LaterThan(EdtfDate::parse(rest)?));
        }
        Ok(Self::Date(EdtfDate::parse(trimmed)?))
    }

    /// The value as a fuzzy instant.
    ///
    /// A set collapses to the hull of its members, which is an
    /// over-approximation: `[1667,1670]` becomes "somewhere from 1667 to the
    /// end of 1670", losing the gap. [`FuzzyInstant`] has no disjunctive
    /// form, and inventing one here would be a worse answer than a stated
    /// widening.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::InvalidSyntax`] for an empty set and
    /// [`UncertaintyError::Overflow`] when a bound leaves the representable
    /// range.
    pub fn to_fuzzy_instant(&self) -> UncertaintyResult<FuzzyInstant> {
        match self {
            Self::Date(date) => date.to_fuzzy_instant(),
            Self::Interval { start, end } => {
                let earliest = match start {
                    EdtfEndpoint::Date(date) => Some(date.start_instant()),
                    EdtfEndpoint::Open | EdtfEndpoint::Unknown => None,
                };
                let latest = match end {
                    EdtfEndpoint::Date(date) => Some(date.end_instant()),
                    EdtfEndpoint::Open | EdtfEndpoint::Unknown => None,
                };
                Ok(match (earliest, latest) {
                    (Some(low), Some(high)) => FuzzyInstant::bounded(low, high)?,
                    (Some(low), None) => FuzzyInstant::After(low),
                    (None, Some(high)) => FuzzyInstant::Before(high),
                    (None, None) => FuzzyInstant::Unknown,
                })
            }
            Self::EarlierThan(date) => Ok(FuzzyInstant::Before(date.end_instant())),
            Self::LaterThan(date) => Ok(FuzzyInstant::After(date.start_instant())),
            #[cfg(feature = "alloc")]
            Self::OneOf(members) | Self::AllOf(members) => hull_of(members),
        }
    }
}

impl FromStr for EdtfValue {
    type Err = UncertaintyError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::parse(text)
    }
}

impl fmt::Display for EdtfValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Date(date) => write!(f, "{date}"),
            Self::Interval { start, end } => write!(f, "{start}/{end}"),
            Self::EarlierThan(date) => write!(f, "..{date}"),
            Self::LaterThan(date) => write!(f, "{date}.."),
            #[cfg(feature = "alloc")]
            Self::OneOf(members) => write_set(f, members, '[', ']'),
            #[cfg(feature = "alloc")]
            Self::AllOf(members) => write_set(f, members, '{', '}'),
        }
    }
}

/// Render a set or list between its brackets.
#[cfg(feature = "alloc")]
fn write_set(
    f: &mut fmt::Formatter<'_>,
    members: &[EdtfSetMember],
    open: char,
    close: char,
) -> fmt::Result {
    write!(f, "{open}")?;
    for (index, member) in members.iter().enumerate() {
        if index > 0 {
            f.write_str(",")?;
        }
        write!(f, "{member}")?;
    }
    write!(f, "{close}")
}

/// Parse the comma-separated body of a set or list.
#[cfg(feature = "alloc")]
fn parse_set(body: &str, one_of: bool) -> UncertaintyResult<EdtfValue> {
    let mut members = Vec::new();
    for part in body.split(',') {
        members.push(EdtfSetMember::parse(part)?);
    }
    if members.is_empty() {
        return Err(UncertaintyError::InvalidSyntax("at least one member"));
    }
    Ok(if one_of {
        EdtfValue::OneOf(members)
    } else {
        EdtfValue::AllOf(members)
    })
}

/// Sets need a growable collection, so a build without `alloc` says so
/// instead of silently parsing the first member.
#[cfg(not(feature = "alloc"))]
fn parse_set(_body: &str, _one_of: bool) -> UncertaintyResult<EdtfValue> {
    Err(UncertaintyError::Unsupported(
        "EDTF set and list forms need the alloc feature",
    ))
}

/// The smallest fuzzy instant covering every member of a set.
#[cfg(feature = "alloc")]
fn hull_of(members: &[EdtfSetMember]) -> UncertaintyResult<FuzzyInstant> {
    let mut low: Option<i64> = None;
    let mut high: Option<i64> = None;
    let mut open_low = false;
    let mut open_high = false;
    if members.is_empty() {
        return Err(UncertaintyError::InvalidSyntax("at least one member"));
    }
    for member in members {
        match member.day_range() {
            (Some(start), Some(end)) => {
                low = Some(low.map_or(start, |current: i64| current.min(start)));
                high = Some(high.map_or(end, |current: i64| current.max(end)));
            }
            (None, Some(end)) => {
                open_low = true;
                high = Some(high.map_or(end, |current: i64| current.max(end)));
            }
            (Some(start), None) => {
                open_high = true;
                low = Some(low.map_or(start, |current: i64| current.min(start)));
            }
            (None, None) => {
                open_low = true;
                open_high = true;
            }
        }
    }
    Ok(match (open_low, open_high) {
        (true, true) => FuzzyInstant::Unknown,
        (true, false) => match high {
            Some(end) => FuzzyInstant::Before(instant_of_day(end + 1)),
            None => FuzzyInstant::Unknown,
        },
        (false, true) => match low {
            Some(start) => FuzzyInstant::After(instant_of_day(start)),
            None => FuzzyInstant::Unknown,
        },
        (false, false) => match (low, high) {
            (Some(start), Some(end)) => {
                FuzzyInstant::bounded(instant_of_day(start), instant_of_day(end + 1))?
            }
            _ => FuzzyInstant::Unknown,
        },
    })
}

/// Parse one side of a `/` interval.
fn parse_endpoint(text: &str) -> UncertaintyResult<EdtfEndpoint> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Ok(EdtfEndpoint::Unknown);
    }
    if trimmed == ".." {
        return Ok(EdtfEndpoint::Open);
    }
    Ok(EdtfEndpoint::Date(EdtfDate::parse(trimmed)?))
}

/// Split a trailing `?`, `~` or `%` off a date body.
fn split_qualifier(text: &str) -> (&str, EdtfQualifier) {
    for (symbol, qualifier) in [
        ('?', EdtfQualifier::Uncertain),
        ('~', EdtfQualifier::Approximate),
        ('%', EdtfQualifier::UncertainAndApproximate),
    ] {
        if let Some(body) = text.strip_suffix(symbol) {
            return (body, qualifier);
        }
    }
    (text, EdtfQualifier::Certain)
}

/// Parse the four-character year field, counting trailing `X` digits.
fn parse_year_digits(text: &str) -> UncertaintyResult<(i64, u8)> {
    if text.len() != 4 {
        return Err(UncertaintyError::InvalidSyntax("a four-character year"));
    }
    let mut value = 0i64;
    let mut unspecified = 0u8;
    for byte in text.bytes() {
        match byte {
            b'0'..=b'9' => {
                if unspecified > 0 {
                    return Err(UncertaintyError::InvalidSyntax(
                        "X digits only at the end of the year",
                    ));
                }
                value = value * 10 + i64::from(byte - b'0');
            }
            b'X' => {
                value *= 10;
                unspecified += 1;
            }
            _ => return Err(UncertaintyError::InvalidSyntax("digits or X")),
        }
    }
    Ok((value, unspecified))
}

/// Parse a two-character month or day field.
fn parse_two_digit_component(text: &str) -> UncertaintyResult<EdtfComponent> {
    if text.len() != 2 {
        return Err(UncertaintyError::InvalidSyntax("a two-digit component"));
    }
    if text == "XX" {
        return Ok(EdtfComponent::Unspecified);
    }
    let mut value = 0u8;
    for byte in text.bytes() {
        if !byte.is_ascii_digit() {
            return Err(UncertaintyError::InvalidSyntax("two digits or XX"));
        }
        value = value * 10 + (byte - b'0');
    }
    Ok(EdtfComponent::Known(value))
}

/// Parse a signed decimal integer with no width constraint.
fn parse_signed_integer(text: &str) -> UncertaintyResult<i64> {
    let (negative, digits) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    if digits.is_empty() {
        return Err(UncertaintyError::InvalidSyntax("at least one digit"));
    }
    let mut value = 0i64;
    for byte in digits.bytes() {
        if !byte.is_ascii_digit() {
            return Err(UncertaintyError::InvalidSyntax("decimal digits"));
        }
        value = value
            .checked_mul(10)
            .and_then(|scaled| scaled.checked_add(i64::from(byte - b'0')))
            .ok_or(UncertaintyError::Overflow)?;
    }
    Ok(if negative { -value } else { value })
}

/// Write a month or day component, including its leading hyphen.
fn write_component(f: &mut fmt::Formatter<'_>, component: EdtfComponent) -> fmt::Result {
    match component {
        EdtfComponent::Absent => Ok(()),
        EdtfComponent::Unspecified => f.write_str("-XX"),
        EdtfComponent::Known(value) => write!(f, "-{value:02}"),
    }
}

/// Ten to the power of a small exponent, as the width of an unspecified year.
const fn pow10(exponent: u8) -> i64 {
    match exponent {
        0 => 1,
        1 => 10,
        2 => 100,
        3 => 1_000,
        _ => 10_000,
    }
}

/// Whether a proleptic Gregorian year is a leap year.
const fn is_leap_year(year: i64) -> bool {
    year.rem_euclid(4) == 0 && (year.rem_euclid(100) != 0 || year.rem_euclid(400) == 0)
}

/// The length of a proleptic Gregorian month.
const fn days_in_month(year: i64, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 31,
    }
}

/// Rata Die of a proleptic Gregorian date.
///
/// Reingold and Dershowitz, *Calendrical Calculations*, 4th ed., equation
/// (2.17). `Rd(1)` is `0001-01-01`.
fn fixed_from_gregorian(year: i64, month: u8, day: u8) -> i64 {
    let prior = year - 1;
    let mut fixed = 365 * prior + prior.div_euclid(4) - prior.div_euclid(100)
        + prior.div_euclid(400)
        + (367 * i64::from(month) - 362).div_euclid(12)
        + i64::from(day);
    if month > 2 {
        fixed += if is_leap_year(year) { -1 } else { -2 };
    }
    fixed
}

/// The proleptic Gregorian date of a Rata Die, the inverse of
/// [`fixed_from_gregorian`].
///
/// Reingold and Dershowitz, 4th ed., equations (2.18) to (2.23). Present so
/// that the forward formula can be round-trip tested over its whole range.
#[cfg(test)]
fn gregorian_from_fixed(fixed: i64) -> (i64, u8, u8) {
    let offset = fixed - 1;
    let cycles_400 = offset.div_euclid(146_097);
    let remainder_400 = offset.rem_euclid(146_097);
    let cycles_100 = remainder_400.div_euclid(36_524);
    let remainder_100 = remainder_400.rem_euclid(36_524);
    let cycles_4 = remainder_100.div_euclid(1_461);
    let remainder_4 = remainder_100.rem_euclid(1_461);
    let years = remainder_4.div_euclid(365);
    let candidate = 400 * cycles_400 + 100 * cycles_100 + 4 * cycles_4 + years;
    let year = if cycles_100 == 4 || years == 4 {
        candidate
    } else {
        candidate + 1
    };
    let prior_days = fixed - fixed_from_gregorian(year, 1, 1);
    let correction = if fixed < fixed_from_gregorian(year, 3, 1) {
        0
    } else if is_leap_year(year) {
        1
    } else {
        2
    };
    let month = ((12 * (prior_days + correction) + 373).div_euclid(367)) as u8;
    let day = (fixed - fixed_from_gregorian(year, month, 1) + 1) as u8;
    (year, month, day)
}

/// The TAI instant at the start of a fixed day, counting 86 400-second days
/// from the 1970 epoch.
fn instant_of_day(fixed: i64) -> Instant<Tai> {
    Instant::from_epoch(Duration::from_days(fixed - RD_OF_UNIX_EPOCH))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    use alloc::string::ToString as _;

    #[test]
    fn the_unix_epoch_sits_at_the_published_rata_die() {
        // Reingold and Dershowitz give RD 719163 for 1970-01-01.
        assert_eq!(fixed_from_gregorian(1970, 1, 1), RD_OF_UNIX_EPOCH);
        assert_eq!(
            instant_of_day(RD_OF_UNIX_EPOCH).since_epoch(),
            Duration::ZERO
        );
    }

    #[test]
    fn published_reference_dates_land_on_their_fixed_days() {
        // Reingold and Dershowitz, Appendix C sample dates.
        assert_eq!(fixed_from_gregorian(1, 1, 1), 1);
        assert_eq!(fixed_from_gregorian(1945, 11, 12), 710_347);
        assert_eq!(fixed_from_gregorian(2000, 1, 1), 730_120);
        // The Gregorian reform: 1582-10-15 was the first Gregorian day.
        assert_eq!(fixed_from_gregorian(1582, 10, 15), 577_736);
    }

    #[test]
    fn the_gregorian_conversion_round_trips_over_four_centuries() {
        // A full 400-year cycle, day by day, is the only way to be sure the
        // leap rules and the month table agree with each other.
        let start = fixed_from_gregorian(1600, 1, 1);
        let end = fixed_from_gregorian(2000, 1, 1);
        for fixed in start..end {
            let (year, month, day) = gregorian_from_fixed(fixed);
            assert_eq!(
                fixed_from_gregorian(year, month, day),
                fixed,
                "round trip failed at {fixed}"
            );
        }
    }

    #[test]
    fn the_gregorian_conversion_round_trips_before_the_common_era() {
        for fixed in -2_000..2_000 {
            let (year, month, day) = gregorian_from_fixed(fixed);
            assert_eq!(fixed_from_gregorian(year, month, day), fixed);
        }
    }

    #[test]
    fn the_leap_rule_matches_the_century_exceptions() {
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(1800));
        assert!(is_leap_year(1600));
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2023));
    }

    #[test]
    fn february_has_twenty_nine_days_only_in_leap_years() {
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2023, 2), 28);
        assert_eq!(days_in_month(1900, 2), 28);
        assert_eq!(days_in_month(2000, 2), 29);
    }

    #[test]
    fn a_plain_year_parses_and_renders_unchanged() {
        let value = EdtfValue::parse("1984").unwrap();
        assert_eq!(value, EdtfValue::Date(EdtfDate::year(1984).unwrap()));
        #[cfg(feature = "alloc")]
        assert_eq!(value.to_string(), "1984");
    }

    #[test]
    fn the_three_qualifiers_parse() {
        for (text, expected) in [
            ("1984?", EdtfQualifier::Uncertain),
            ("1984~", EdtfQualifier::Approximate),
            ("1984%", EdtfQualifier::UncertainAndApproximate),
        ] {
            let date = EdtfDate::parse(text).unwrap();
            assert_eq!(date.qualifier(), expected, "for {text}");
            #[cfg(feature = "alloc")]
            assert_eq!(date.to_string(), text);
        }
    }

    #[test]
    fn an_uncertain_year_is_not_widened_but_an_approximate_one_is() {
        let doubted = EdtfDate::parse("1984?")
            .unwrap()
            .to_fuzzy_instant()
            .unwrap();
        let blurred = EdtfDate::parse("1984~")
            .unwrap()
            .to_fuzzy_instant()
            .unwrap();
        let plain = EdtfDate::parse("1984").unwrap().to_fuzzy_instant().unwrap();
        assert_eq!(doubted.span().unwrap(), plain.span().unwrap());
        let plain_span = plain.span().unwrap().unwrap();
        let blurred_span = blurred.span().unwrap().unwrap();
        assert_eq!(blurred_span, plain_span.checked_mul_int(3).unwrap());
    }

    #[test]
    fn unspecified_day_digits_widen_the_support_to_the_month() {
        let masked = EdtfValue::parse("1984-01-XX").unwrap();
        let month = EdtfValue::parse("1984-01").unwrap();
        assert_eq!(
            masked.to_fuzzy_instant().unwrap().span().unwrap(),
            month.to_fuzzy_instant().unwrap().span().unwrap()
        );
        #[cfg(feature = "alloc")]
        assert_eq!(masked.to_string(), "1984-01-XX");
    }

    #[test]
    fn unspecified_year_digits_name_a_decade_a_century_and_a_millennium() {
        assert_eq!(
            EdtfDate::parse("198X").unwrap().precision(),
            EdtfPrecision::Decade
        );
        assert_eq!(
            EdtfDate::parse("19XX").unwrap().precision(),
            EdtfPrecision::Century
        );
        assert_eq!(
            EdtfDate::parse("1XXX").unwrap().precision(),
            EdtfPrecision::Millennium
        );
        assert_eq!(
            EdtfDate::parse("XXXX").unwrap().precision(),
            EdtfPrecision::Unknown
        );
    }

    #[test]
    fn a_century_spans_a_hundred_years_of_days() {
        let century = EdtfDate::parse("19XX").unwrap();
        let (low, high) = century.day_range();
        assert_eq!(low, fixed_from_gregorian(1900, 1, 1));
        assert_eq!(high, fixed_from_gregorian(1999, 12, 31));
    }

    #[test]
    fn x_digits_must_be_at_the_end_of_the_year() {
        assert!(EdtfDate::parse("1X84").is_err());
        assert!(EdtfDate::parse("X984").is_err());
    }

    #[test]
    fn the_long_year_form_carries_a_geological_year() {
        let value = EdtfValue::parse("Y-170000002").unwrap();
        let EdtfValue::Date(date) = value.clone() else {
            panic!("expected a date");
        };
        assert!(date.is_long_form());
        assert_eq!(date.year_value(), -170_000_002);
        #[cfg(feature = "alloc")]
        assert_eq!(value.to_string(), "Y-170000002");
    }

    #[test]
    fn a_long_year_lands_far_before_the_epoch() {
        let date = EdtfDate::parse("Y-170000002").unwrap();
        let instant = date.start_instant();
        assert!(instant.since_epoch() < Duration::from_days(-62_000_000_000));
    }

    #[test]
    fn a_five_digit_year_without_the_y_prefix_is_rejected() {
        assert!(EdtfDate::parse("12345").is_err());
        assert!(EdtfDate::year(12_345).is_err());
    }

    #[test]
    fn an_interval_parses_both_endpoints() {
        let value = EdtfValue::parse("1984/1985").unwrap();
        match &value {
            EdtfValue::Interval { start, end } => {
                assert_eq!(*start, EdtfEndpoint::Date(EdtfDate::year(1984).unwrap()));
                assert_eq!(*end, EdtfEndpoint::Date(EdtfDate::year(1985).unwrap()));
            }
            other => panic!("expected an interval, got {other:?}"),
        }
        #[cfg(feature = "alloc")]
        assert_eq!(value.to_string(), "1984/1985");
    }

    #[test]
    fn an_interval_becomes_a_bounded_instant_spanning_both_years() {
        let fuzzy = EdtfValue::parse("1984/1985")
            .unwrap()
            .to_fuzzy_instant()
            .unwrap();
        let support = fuzzy.support().unwrap();
        assert_eq!(
            support.earliest,
            Some(instant_of_day(fixed_from_gregorian(1984, 1, 1)))
        );
        assert_eq!(
            support.latest,
            Some(instant_of_day(fixed_from_gregorian(1986, 1, 1)))
        );
    }

    #[test]
    fn an_open_interval_endpoint_becomes_an_open_ended_instant() {
        let later = EdtfValue::parse("1984/..").unwrap();
        assert!(matches!(
            later.to_fuzzy_instant().unwrap(),
            FuzzyInstant::After(_)
        ));
        let earlier = EdtfValue::parse("../1984").unwrap();
        assert!(matches!(
            earlier.to_fuzzy_instant().unwrap(),
            FuzzyInstant::Before(_)
        ));
        #[cfg(feature = "alloc")]
        {
            assert_eq!(later.to_string(), "1984/..");
            assert_eq!(earlier.to_string(), "../1984");
        }
    }

    #[test]
    fn an_empty_interval_endpoint_is_unknown_rather_than_open() {
        let value = EdtfValue::parse("1984/").unwrap();
        match &value {
            EdtfValue::Interval { end, .. } => assert_eq!(*end, EdtfEndpoint::Unknown),
            other => panic!("expected an interval, got {other:?}"),
        }
        #[cfg(feature = "alloc")]
        assert_eq!(value.to_string(), "1984/");
    }

    #[test]
    fn an_interval_with_two_unknown_endpoints_is_wholly_unknown() {
        let value = EdtfValue::parse("/").unwrap();
        assert_eq!(value.to_fuzzy_instant().unwrap(), FuzzyInstant::Unknown);
    }

    #[test]
    fn the_bare_earlier_than_form_parses_and_round_trips() {
        let value = EdtfValue::parse("..1760-12-03").unwrap();
        assert!(matches!(value, EdtfValue::EarlierThan(_)));
        assert!(matches!(
            value.to_fuzzy_instant().unwrap(),
            FuzzyInstant::Before(_)
        ));
        #[cfg(feature = "alloc")]
        assert_eq!(value.to_string(), "..1760-12-03");
    }

    #[test]
    fn the_bare_later_than_form_parses_and_round_trips() {
        let value = EdtfValue::parse("1760-12..").unwrap();
        assert!(matches!(value, EdtfValue::LaterThan(_)));
        assert!(matches!(
            value.to_fuzzy_instant().unwrap(),
            FuzzyInstant::After(_)
        ));
        #[cfg(feature = "alloc")]
        assert_eq!(value.to_string(), "1760-12..");
    }

    #[test]
    fn an_earlier_than_bound_includes_the_whole_named_day() {
        let value = EdtfValue::parse("..1760-12-03").unwrap();
        let FuzzyInstant::Before(latest) = value.to_fuzzy_instant().unwrap() else {
            panic!("expected an open start");
        };
        assert_eq!(latest, instant_of_day(fixed_from_gregorian(1760, 12, 4)));
    }

    #[test]
    fn an_invalid_month_or_day_is_rejected() {
        assert!(EdtfDate::parse("1984-13").is_err());
        assert!(EdtfDate::parse("1984-00").is_err());
        assert!(EdtfDate::parse("1984-02-30").is_err());
        assert!(EdtfDate::parse("1984-01-32").is_err());
        assert!(EdtfDate::ymd(1983, 2, 29).is_err());
        assert!(EdtfDate::ymd(1984, 2, 29).is_ok());
    }

    #[test]
    fn an_unspecified_month_may_still_carry_a_day() {
        // EDTF level 2 allows `1984-XX-01`; the support is then the hull of
        // the twelve first-of-the-month possibilities.
        assert!(EdtfDate::parse("1984-XX-01").is_ok());
        let date = EdtfDate::parse("1984-XX-01").unwrap();
        assert_eq!(date.month(), EdtfComponent::Unspecified);
        assert_eq!(date.day(), EdtfComponent::Known(1));
    }

    #[test]
    fn malformed_input_is_reported_rather_than_guessed() {
        assert!(EdtfValue::parse("").is_err());
        assert!(EdtfValue::parse("   ").is_err());
        assert!(EdtfDate::parse("198").is_err());
        assert!(EdtfDate::parse("1984-1").is_err());
        assert!(EdtfDate::parse("1984-01-01-01").is_err());
        assert!(EdtfDate::parse("abcd").is_err());
        assert!(EdtfDate::parse("1984\u{2013}01").is_err());
    }

    #[test]
    fn a_negative_four_digit_year_parses() {
        let date = EdtfDate::parse("-0999").unwrap();
        assert_eq!(date.year_value(), -999);
        #[cfg(feature = "alloc")]
        assert_eq!(date.to_string(), "-0999");
    }

    #[test]
    fn every_supported_form_round_trips_through_parse_and_render() {
        #[cfg(feature = "alloc")]
        for text in [
            "1984",
            "1984?",
            "1984~",
            "1984%",
            "1984-01",
            "1984-01-01",
            "1984-01-XX",
            "1984-XX-XX",
            "198X",
            "19XX",
            "Y-170000002",
            "Y170000002",
            "1984/1985",
            "1984-01-01/1984-12-31",
            "1984/..",
            "../1984",
            "1984/",
            "/1984",
            "..1760-12-03",
            "1760-12..",
            "-0999",
        ] {
            let parsed = EdtfValue::parse(text).unwrap();
            assert_eq!(parsed.to_string(), text, "round trip failed for {text}");
        }
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn a_one_of_set_parses_dates_and_ranges() {
        let value = EdtfValue::parse("[1667,1668,1670..1672]").unwrap();
        let EdtfValue::OneOf(members) = &value else {
            panic!("expected a one-of set");
        };
        assert_eq!(members.len(), 3);
        assert_eq!(
            members[0],
            EdtfSetMember::Date(EdtfDate::year(1667).unwrap())
        );
        assert_eq!(
            members[2],
            EdtfSetMember::Range {
                start: EdtfDate::year(1670).unwrap(),
                end: EdtfDate::year(1672).unwrap(),
            }
        );
        assert_eq!(value.to_string(), "[1667,1668,1670..1672]");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn an_all_of_list_parses_and_round_trips() {
        let value = EdtfValue::parse("{1960,1961-12}").unwrap();
        assert!(matches!(value, EdtfValue::AllOf(_)));
        assert_eq!(value.to_string(), "{1960,1961-12}");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn a_set_collapses_to_the_hull_of_its_members() {
        let value = EdtfValue::parse("[1667,1668,1670..1672]").unwrap();
        let support = value.to_fuzzy_instant().unwrap().support().unwrap();
        assert_eq!(
            support.earliest,
            Some(instant_of_day(fixed_from_gregorian(1667, 1, 1)))
        );
        assert_eq!(
            support.latest,
            Some(instant_of_day(fixed_from_gregorian(1673, 1, 1)))
        );
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn an_open_ended_set_member_leaves_the_hull_open() {
        let value = EdtfValue::parse("[..1760-12-03]").unwrap();
        assert!(matches!(
            value.to_fuzzy_instant().unwrap(),
            FuzzyInstant::Before(_)
        ));
        let value = EdtfValue::parse("[1760-12..]").unwrap();
        assert!(matches!(
            value.to_fuzzy_instant().unwrap(),
            FuzzyInstant::After(_)
        ));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn an_unclosed_set_is_rejected() {
        assert!(EdtfValue::parse("[1667,1668").is_err());
        assert!(EdtfValue::parse("{1667").is_err());
    }

    #[test]
    fn precision_reflects_the_finest_stated_component() {
        assert_eq!(
            EdtfDate::parse("1984").unwrap().precision(),
            EdtfPrecision::Year
        );
        assert_eq!(
            EdtfDate::parse("1984-01").unwrap().precision(),
            EdtfPrecision::Month
        );
        assert_eq!(
            EdtfDate::parse("1984-01-01").unwrap().precision(),
            EdtfPrecision::Day
        );
        assert_eq!(
            EdtfDate::parse("1984-XX").unwrap().precision(),
            EdtfPrecision::Year
        );
    }

    #[test]
    fn a_parsed_date_can_be_compared_with_another_through_allens_algebra() {
        let earlier = EdtfValue::parse("1667")
            .unwrap()
            .to_fuzzy_instant()
            .unwrap();
        let later = EdtfValue::parse("1670..")
            .unwrap()
            .to_fuzzy_instant()
            .unwrap();
        assert!(earlier.definitely_before(later).unwrap());
    }

    #[test]
    fn the_from_str_impls_agree_with_parse() {
        let by_trait: EdtfValue = "1984-01".parse().unwrap();
        assert_eq!(by_trait, EdtfValue::parse("1984-01").unwrap());
        let date: EdtfDate = "1984-01".parse().unwrap();
        assert_eq!(date, EdtfDate::parse("1984-01").unwrap());
    }

    #[test]
    fn qualifier_helpers_describe_the_symbols() {
        assert_eq!(EdtfQualifier::Certain.symbol(), "");
        assert_eq!(EdtfQualifier::Uncertain.symbol(), "?");
        assert!(!EdtfQualifier::Uncertain.is_approximate());
        assert!(EdtfQualifier::Approximate.is_approximate());
        assert!(EdtfQualifier::UncertainAndApproximate.is_approximate());
        assert_eq!(EdtfQualifier::default(), EdtfQualifier::Certain);
    }

    #[test]
    fn components_report_their_values() {
        assert_eq!(EdtfComponent::Known(7).value(), Some(7));
        assert_eq!(EdtfComponent::Unspecified.value(), None);
        assert_eq!(EdtfComponent::Absent.value(), None);
        assert!(EdtfComponent::Unspecified.is_present());
        assert!(!EdtfComponent::Absent.is_present());
    }
}
