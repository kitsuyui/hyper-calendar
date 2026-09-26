//! Human-readable time: *3 days ago*, *in 2 hours*, *2 hours 30 minutes*,
//! *yesterday at 15:30*, *just over a week*.
//!
//! # What this crate is for
//!
//! `hc-calendar` can tell you that two instants are 34 200 seconds apart.
//! Nobody says that. This crate turns a [`hc_core::Duration`] or a pair of
//! [`hc_calendar::Rd`] days into the phrase a person would use, in the
//! language they read, with the grammar that language actually has.
//!
//! The grammar is the hard part and it is why this crate sits on
//! [`hc_i18n`] rather than on a table of English strings. "3 days" is easy.
//! *3 дня*, *5 дней*, *21 день*; *2 dni*, *5 dni*, *22 dni*; *يومين*,
//! *3 أيام*, *11 يومًا*; *2 ddiwrnod*, *3 diwrnod*, *8 o ddiwrnodau* — those
//! are four different plural systems, and every one of them goes through
//! [`hc_i18n::PluralRules`]. Nothing in this crate decides a plural form by
//! comparing a number to one.
//!
//! # The four questions
//!
//! | Module | Question | Example |
//! |---|---|---|
//! | [`relative`] | When was it, relative to now? | *3 days ago*, *yesterday* |
//! | [`duration`] | How long is it? | *2 hours 30 minutes*, *2h30m* |
//! | [`calendar_relative`] | Which calendar day was it? | *yesterday at 15:30*, *last Tuesday* |
//! | [`approximate`](mod@approximate) | Roughly how long? | *just over a week* |
//!
//! and one shared decision underneath them:
//!
//! | [`unit_choice`] | Which unit, and rounded how? | 90 min → *2 hours* or *an hour and a half* |
//!
//! [`natural`] answers the same questions in a second convention: the
//! thresholds and English strings of the Python `humanize` package —
//! *a moment*, *1 year, 3 months*, *1.2 billion* — for code ported from it.
//!
//! # Data is not code
//!
//! Every locale is one [`pattern::LocaleData`] value of `&'static` strings
//! in [`data::LOCALES`], and lookup walks the CLDR fallback chain taking the
//! first entry that carries the field asked for. Adding a language is one
//! `const` and one line in an array; no function in this crate learns a new
//! branch. This is the same split `hc-i18n` uses, for the same reason.
//!
//! # No allocator needed
//!
//! Every formatter writes into a [`core::fmt::Write`] sink a piece at a
//! time, and nothing is assembled into an intermediate buffer. The crate
//! builds without `std`, with or without `alloc`, given `hc-core`'s `libm`
//! feature for floating-point math, and the `alloc` feature adds only the
//! `String`-returning conveniences — `format`, `format_amount`,
//! `format_elapsed` — beside the `write` ones.
//!
//! ```
//! use core::fmt::Write as _;
//! use hc_humanize::{RelativeTimeFormatter, TimeUnit};
//! use hc_i18n::Locale;
//!
//! let formatter = RelativeTimeFormatter::new("ru".parse::<Locale>()?);
//! let mut text = String::new();
//! formatter.write(-5, TimeUnit::Day, &mut text)?;
//! assert_eq!(text, "5 дней назад");
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Scope
//!
//! This crate phrases *spans*. It does not format dates or times — that is
//! `hc-format`, which [`natural`]'s `naturalday` hands a date to, as Python's
//! `humanize` hands it to `strftime` — and it does not know what "now" is: every entry point takes
//! both ends, or a span, from the caller. A humaniser that read a clock
//! could not be tested.
//!
//! Its month and year lengths are the Gregorian means, which is what makes
//! "about 3 months" a meaningful sentence about a bare duration; see
//! [`unit`](mod@unit) for the exact constants and for what that deliberately
//! refuses to answer.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod approximate;
pub mod calendar_relative;
pub mod data;
pub mod duration;
pub mod error;
pub mod lookup;
pub mod natural;
pub mod pattern;
pub mod relative;
pub mod unit;
pub mod unit_choice;

mod render;

pub use approximate::{
    ApproximateFormatter, ApproximatePolicy, ApproximateSpan, Approximation, approximate,
};
pub use calendar_relative::{CalendarRelativeFormatter, day_offset, week_offset};
pub use duration::{Components, DurationFormatter, DurationStyle, decompose};
pub use error::{HumanizeError, HumanizeResult};
pub use pattern::RelativeStyle;
pub use relative::{Numeric, RelativeTimeFormatter};
pub use unit::{TimeUnit, UnitAmount};
pub use unit_choice::{RoundingPolicy, Threshold, Thresholds};

pub use hc_calendar;
pub use hc_i18n;
