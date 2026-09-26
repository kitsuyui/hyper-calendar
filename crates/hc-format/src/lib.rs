//! Parsing and formatting for `hyper-calendar`: ISO 8601, RFC 3339,
//! RFC 5322, and the two pattern vocabularies.
//!
//! # The three things this crate refuses to do
//!
//! **It will not treat an unqualified local time as UTC.** `2026-09-21T14:30`
//! names a reading on somebody's wall clock and nothing more. Parsing gives
//! back an [`OffsetDateTime`] whose [`ZoneInfo`] is
//! [`ZoneInfo::Unspecified`], and turning that into an instant needs a zone
//! the caller supplies. This is the single most common date bug in the wild
//! and the type system is the right place to stop it.
//!
//! **It will not say "invalid".** Every [`ParseError`] carries what was
//! expected and the byte offset where the parser stopped, because the caller
//! is usually about to explain the problem to a human.
//!
//! **It will not round `24:00` to `00:00` or drop the `60` from
//! `23:59:60`.** Both are real readings that mean something the alternative
//! does not, and both survive a parse-format round trip unchanged.
//!
//! # The modules
//!
//! | Module | Format |
//! | --- | --- |
//! | [`iso8601`] | ISO 8601-1:2019 in full: calendar, ordinal and week dates, basic and extended, reduced accuracy, expanded years, durations, intervals |
//! | [`rfc3339`] | The internet profile, including `-00:00` |
//! | [`rfc2822`] | Email and HTTP dates, obsolete syntax included |
//! | [`patterns`] | `strftime`/`strptime` and CLDR field patterns, both directions |
//! | [`python`] | The ISO 8601 profile and `strptime` defaults of Python's `datetime` |
//! | [`parse`] | A sniffing front door for "a date string" |
//! | [`label`] | A calendar's eras, years, months, days and dates, written as a locale writes them |
//!
//! ```
//! use hc_format::{ZoneInfo, iso8601};
//!
//! let value = iso8601::parse("2026-09-21T14:30:05+09:00")?;
//! let reading = value.to_offset_date_time()?;
//! assert_eq!(reading.to_unix()?.seconds(), 1_789_968_605);
//!
//! let local = iso8601::parse("2026-09-21T14:30:05")?.to_offset_date_time()?;
//! assert_eq!(local.zone, ZoneInfo::Unspecified);
//! assert!(local.to_unix().is_err());
//! # Ok::<(), Box<dyn core::error::Error>>(())
//! ```
//!
//! # `no_std`
//!
//! Everything formats into a [`core::fmt::Write`] sink, so a `no_std` caller
//! writes into its own buffer with no allocator in sight. The `alloc`
//! feature adds [`format_to_string`] and the [`core::fmt::Display`] impls'
//! `to_string`, and nothing else depends on it.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::fmt;

pub mod error;
pub mod iso8601;
pub mod label;
pub mod parse;
pub mod patterns;
pub mod python;
pub mod rfc2822;
pub mod rfc3339;
pub mod roman;
pub mod value;

mod scan;

pub use error::{
    ErrorKind, FormatError, FormatResult, ParseError, ParseResult, ValueError, ValueResult,
};
pub use iso8601::{Interval, IsoDuration, RepeatingInterval, Strictness};
pub use roman::{Anchor, BissextileStyle, RomanDayName};
pub use value::{
    DateParts, DecimalMark, Fraction, IsoDate, IsoDateTime, IsoTime, OffsetDateTime, Style,
    TimeOfDay, YearStyle, ZoneInfo,
};

pub use hc_calendar;
pub use hc_calendars_solar;
pub use hc_core;
pub use hc_i18n;
pub use hc_tz;

/// Anything this crate can render into a [`core::fmt::Write`] sink.
///
/// The trait exists so that [`format_to_string`] can be written once. It is
/// generic over the sink rather than taking `&mut dyn Write`, because a
/// `no_std` caller writing into a fixed buffer should not pay for a vtable.
pub trait WriteTo {
    /// Write the value.
    ///
    /// # Errors
    ///
    /// [`FormatError::Sink`] when the sink refuses, and
    /// [`FormatError::Unrepresentable`] when the value has no spelling in the
    /// form it was asked for.
    fn write_to<W: fmt::Write>(&self, out: &mut W) -> FormatResult<()>;
}

macro_rules! impl_write_to {
    ($($type:ty),+ $(,)?) => {
        $(impl WriteTo for $type {
            fn write_to<W: fmt::Write>(&self, out: &mut W) -> FormatResult<()> {
                (*self).write(out)
            }
        })+
    };
}

impl_write_to!(
    IsoDate,
    IsoTime,
    IsoDateTime,
    IsoDuration,
    Interval,
    RepeatingInterval,
);

/// Render a value into a fresh [`alloc::string::String`].
///
/// # Errors
///
/// See [`WriteTo::write_to`].
#[cfg(feature = "alloc")]
pub fn format_to_string<T: WriteTo + ?Sized>(value: &T) -> FormatResult<alloc::string::String> {
    let mut out = alloc::string::String::new();
    value.write_to(&mut out)?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_renderable_value_reaches_a_string_through_one_function() {
        let date_time = iso8601::parse("2026-09-21T14:30:05Z").unwrap();
        assert_eq!(
            format_to_string(&date_time).unwrap(),
            "2026-09-21T14:30:05Z"
        );
        assert_eq!(format_to_string(&date_time.date).unwrap(), "2026-09-21");
        assert_eq!(
            format_to_string(&iso8601::duration::parse("P1W").unwrap()).unwrap(),
            "P1W"
        );
        assert_eq!(
            format_to_string(&iso8601::interval::parse("P1Y/2026-12-31T23:59:59Z").unwrap())
                .unwrap(),
            "P1Y/2026-12-31T23:59:59Z"
        );
    }

    #[test]
    fn an_unrepresentable_value_fails_instead_of_panicking() {
        let date = IsoDate {
            parts: DateParts::Calendar {
                year: -1,
                month: Some(1),
                day: Some(1),
            },
            style: Style::Extended,
            year_style: YearStyle::Plain,
        };
        assert!(format_to_string(&date).is_err());
    }
}
