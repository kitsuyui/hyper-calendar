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
//! Not carried, and rejected rather than half-parsed (policy §4):
//!
//! * **Times of day.** `1985-04-12T23:20:30Z` is Level 0 EDTF; this module
//!   reads *which day* and not yet the time within it.
//! * **Seasons and sub-year divisions** (`2001-21` for spring, `2001-34` for
//!   a quarter). Their boundaries are conventions that differ by hemisphere
//!   and by publisher. Policy §5 would give each convention its own name, and
//!   none has been added, so the form is refused and no boundary is guessed.
//! * **Component-level qualification** (`2004-06~-11`, "June is approximate
//!   but the year and day are not"). The support it implies is not an
//!   interval, and [`crate::FuzzyInstant`] holds only intervals.
//! * **Exponential years and significant digits** (`Y17E7S3`). Not yet done.
//!
//! # Where the calendar arithmetic comes from
//!
//! Placing `1984-01-01` on a timeline needs proleptic Gregorian day
//! arithmetic, which belongs to `hc-calendar` (policy §2, one
//! implementation), and this module calls
//! [`hc_calendar::gregorian::to_fixed`] for it. The one adapter here widens
//! the range: `hc-calendar` converts years within ±9 999 999, and the `Y`
//! form writes years beyond that, so a year is reduced by whole 400-year
//! Gregorian cycles into that range, converted there, and moved back. A test
//! asserts that the adapter changes the range and nothing else.
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

use hc_calendar::fixed::RD_OF_UNIX_EPOCH;
use hc_calendar::gregorian;
use hc_core::{Duration, Instant, Tai};

use crate::error::{UncertaintyError, UncertaintyResult};
use crate::fuzzy::FuzzyInstant;

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
            // With unspecified year digits the year could be a leap year,
            // so the day is checked against the month's longest length.
            let year = if self.unspecified_year_digits == 0 {
                self.year
            } else {
                LEAP_YEAR
            };
            if let Some(month) = self.month.value()
                && gregorian::days_in_month(year, month).is_some_and(|length| day > length)
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
    /// wider than the true support and is documented as such. So does
    /// `19XX-02-29`: where the first or last year of the span is a common
    /// year, its end of the hull is that year's 28 February.
    fn day_range(self) -> UncertaintyResult<(i64, i64)> {
        let span = pow10(self.unspecified_year_digits);
        let low_year = self.year;
        let high_year = self
            .year
            .checked_add(span - 1)
            .ok_or(UncertaintyError::Overflow)?;
        let low_month = self.month.value().unwrap_or(1);
        let high_month = self.month.value().unwrap_or(12);
        let low_length = month_length(low_year, low_month)?;
        let high_length = month_length(high_year, high_month)?;
        let low_day = self.day.value().map_or(1, |day| day.min(low_length));
        let high_day = self
            .day
            .value()
            .map_or(high_length, |day| day.min(high_length));
        Ok((
            fixed_from_gregorian(low_year, low_month, low_day)?,
            fixed_from_gregorian(high_year, high_month, high_day)?,
        ))
    }

    /// The first instant the date could denote.
    fn start_instant(self) -> UncertaintyResult<Instant<Tai>> {
        instant_of_day(self.day_range()?.0)
    }

    /// The instant one day after the last day the date could denote, which
    /// is the exclusive end of the range and the closed upper bound of the
    /// support.
    fn end_instant(self) -> UncertaintyResult<Instant<Tai>> {
        let last = self.day_range()?.1;
        instant_of_day(last.checked_add(1).ok_or(UncertaintyError::Overflow)?)
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
        let start = self.start_instant()?;
        let end = self.end_instant()?;
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
    fn day_range(self) -> UncertaintyResult<(Option<i64>, Option<i64>)> {
        Ok(match self {
            Self::Date(date) => {
                let (low, high) = date.day_range()?;
                (Some(low), Some(high))
            }
            Self::Range { start, end } => (Some(start.day_range()?.0), Some(end.day_range()?.1)),
            Self::EarlierThan(date) => (None, Some(date.day_range()?.1)),
            Self::LaterThan(date) => (Some(date.day_range()?.0), None),
        })
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
                    EdtfEndpoint::Date(date) => Some(date.start_instant()?),
                    EdtfEndpoint::Open | EdtfEndpoint::Unknown => None,
                };
                let latest = match end {
                    EdtfEndpoint::Date(date) => Some(date.end_instant()?),
                    EdtfEndpoint::Open | EdtfEndpoint::Unknown => None,
                };
                Ok(match (earliest, latest) {
                    (Some(low), Some(high)) => FuzzyInstant::bounded(low, high)?,
                    (Some(low), None) => FuzzyInstant::After(low),
                    (None, Some(high)) => FuzzyInstant::Before(high),
                    (None, None) => FuzzyInstant::Unknown,
                })
            }
            Self::EarlierThan(date) => Ok(FuzzyInstant::Before(date.end_instant()?)),
            Self::LaterThan(date) => Ok(FuzzyInstant::After(date.start_instant()?)),
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
        match member.day_range()? {
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
            Some(end) => FuzzyInstant::Before(instant_of_day(
                end.checked_add(1).ok_or(UncertaintyError::Overflow)?,
            )?),
            None => FuzzyInstant::Unknown,
        },
        (false, true) => match low {
            Some(start) => FuzzyInstant::After(instant_of_day(start)?),
            None => FuzzyInstant::Unknown,
        },
        (false, false) => match (low, high) {
            (Some(start), Some(end)) => {
                let past = end.checked_add(1).ok_or(UncertaintyError::Overflow)?;
                FuzzyInstant::bounded(instant_of_day(start)?, instant_of_day(past)?)?
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

/// A leap year, for checking a day against a month's longest length.
const LEAP_YEAR: i64 = 2000;

/// Days in one 400-year Gregorian cycle, after which the calendar repeats.
const DAYS_PER_CYCLE: i64 = gregorian::new_year(400).0 - gregorian::new_year(0).0;

/// The length of a month of a year, for any year EDTF can write.
///
/// Only February depends on the year, and only through its position in the
/// 400-year cycle, so the year is reduced into that cycle first.
fn month_length(year: i64, month: u8) -> UncertaintyResult<u8> {
    gregorian::days_in_month(year.rem_euclid(400), month)
        .ok_or(UncertaintyError::InvalidSyntax("a month in 01..12"))
}

/// Rata Die of a proleptic Gregorian date, for any year EDTF can write.
///
/// [`gregorian::to_fixed`] is the implementation. It converts years within
/// ±9 999 999, and the `Y` form writes years beyond that (`Y-170000002`), so
/// the year is reduced into `0..400`, converted there, and moved back by
/// whole cycles. That widens the range and changes nothing else, which
/// `the_adapter_agrees_with_hc_calendar_wherever_both_convert` asserts.
fn fixed_from_gregorian(year: i64, month: u8, day: u8) -> UncertaintyResult<i64> {
    let cycles = year.div_euclid(400);
    let within = gregorian::to_fixed(year.rem_euclid(400), month, day)
        .map_err(|_| UncertaintyError::InvalidSyntax("a day that month has"))?;
    cycles
        .checked_mul(DAYS_PER_CYCLE)
        .and_then(|days| days.checked_add(within.0))
        .ok_or(UncertaintyError::Overflow)
}

/// The TAI instant at the start of a fixed day, counting 86 400-second days
/// from the 1970 epoch.
fn instant_of_day(fixed: i64) -> UncertaintyResult<Instant<Tai>> {
    let days = fixed
        .checked_sub(RD_OF_UNIX_EPOCH)
        .ok_or(UncertaintyError::Overflow)?;
    Ok(Instant::from_epoch(Duration::from_days(days)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    use alloc::string::ToString as _;

    /// `fixed_from_gregorian` with the published answer unwrapped.
    fn fixed(year: i64, month: u8, day: u8) -> i64 {
        fixed_from_gregorian(year, month, day).unwrap()
    }

    #[test]
    fn the_unix_epoch_sits_at_the_published_rata_die() {
        // Reingold and Dershowitz give RD 719163 for 1970-01-01.
        assert_eq!(fixed(1970, 1, 1), RD_OF_UNIX_EPOCH);
        assert_eq!(
            instant_of_day(RD_OF_UNIX_EPOCH).unwrap().since_epoch(),
            Duration::ZERO
        );
    }

    #[test]
    fn published_reference_dates_land_on_their_fixed_days() {
        // Reingold and Dershowitz, Appendix C sample dates.
        assert_eq!(fixed(1, 1, 1), 1);
        assert_eq!(fixed(1945, 11, 12), 710_347);
        assert_eq!(fixed(2000, 1, 1), 730_120);
        // The Gregorian reform: 1582-10-15 was the first Gregorian day.
        assert_eq!(fixed(1582, 10, 15), 577_736);
    }

    /// Policy §2: the adapter may widen the range and must change nothing
    /// else. Every 1 March and 28 February of four whole cycles either side
    /// of year zero, and the edges of `hc-calendar`'s range, land where
    /// `hc-calendar` puts them.
    #[test]
    fn the_adapter_agrees_with_hc_calendar_wherever_both_convert() {
        let years = (-1_600..1_600).chain([
            gregorian::MIN_YEAR,
            gregorian::MIN_YEAR + 1,
            gregorian::MAX_YEAR - 1,
            gregorian::MAX_YEAR,
        ]);
        for year in years {
            for (month, day) in [(1, 1), (2, 28), (3, 1), (12, 31)] {
                assert_eq!(
                    fixed(year, month, day),
                    gregorian::to_fixed(year, month, day).unwrap().get(),
                    "{year}-{month:02}-{day:02}"
                );
            }
            if gregorian::is_leap_year(year) {
                assert_eq!(
                    fixed(year, 2, 29),
                    gregorian::to_fixed(year, 2, 29).unwrap().get()
                );
            } else {
                assert!(fixed_from_gregorian(year, 2, 29).is_err());
            }
        }
    }

    #[test]
    fn the_adapter_converts_years_past_hc_calendars_range() {
        // One cycle past the edge is one cycle's days past the edge's day.
        let year = gregorian::MAX_YEAR - 399;
        assert!(gregorian::to_fixed(year + 400, 1, 1).is_err());
        assert_eq!(fixed(year + 400, 1, 1), fixed(year, 1, 1) + 146_097);
        assert_eq!(
            fixed_from_gregorian(i64::MAX, 1, 1),
            Err(UncertaintyError::Overflow)
        );
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
        let (low, high) = century.day_range().unwrap();
        assert_eq!(low, fixed(1900, 1, 1));
        assert_eq!(high, fixed(1999, 12, 31));
    }

    #[test]
    fn an_unspecified_year_still_bounds_the_day_by_the_month() {
        // April never has a 31st, whichever year of the century it is.
        assert!(EdtfDate::parse("19XX-04-31").is_err());
        assert!(EdtfDate::parse("19XX-02-30").is_err());
        // Some year of the century has a 29 February. 1900 and 1999 are
        // common years, so the hull ends on their 28 February, which
        // encloses 1904-02-29 and 1996-02-29.
        let leap_day = EdtfDate::parse("19XX-02-29").unwrap();
        let (low, high) = leap_day.day_range().unwrap();
        assert_eq!(low, fixed(1900, 2, 28));
        assert_eq!(high, fixed(1999, 2, 28));
        let (low, high) = EdtfDate::parse("199X-02-29").unwrap().day_range().unwrap();
        assert_eq!(low, fixed(1990, 2, 28));
        assert_eq!(high, fixed(1999, 2, 28));
        let (low, _) = EdtfDate::parse("200X-02-29").unwrap().day_range().unwrap();
        assert_eq!(low, fixed(2000, 2, 29));
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
        let instant = date.start_instant().unwrap();
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
            Some(instant_of_day(fixed(1984, 1, 1)).unwrap())
        );
        assert_eq!(
            support.latest,
            Some(instant_of_day(fixed(1986, 1, 1)).unwrap())
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
        assert_eq!(latest, instant_of_day(fixed(1760, 12, 4)).unwrap());
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
            Some(instant_of_day(fixed(1667, 1, 1)).unwrap())
        );
        assert_eq!(
            support.latest,
            Some(instant_of_day(fixed(1673, 1, 1)).unwrap())
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
