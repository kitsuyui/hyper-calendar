//! Time zones for `hyper-calendar`: offsets, POSIX rules and TZif data.
//!
//! A time zone is the rule that turns an instant into a wall-clock reading
//! and back. This crate holds three kinds of rule, behind one trait:
//!
//! * [`FixedTimeZone`] and [`Utc`] — one offset, for all time.
//! * [`PosixTimeZone`] — a POSIX `TZ` string such as `EST5EDT,M3.2.0,M11.1.0`,
//!   evaluated for any year.
//! * [`TzifTimeZone`] — the binary IANA format, RFC 8536, which records every
//!   transition a zone has actually made.
//!
//! # The part most libraries get wrong
//!
//! Going from an instant to a local reading is a function. Going back is not.
//! When clocks go back an hour, a local reading names *two* instants; when
//! they go forward, it names *none*. [`TimeZone::resolve_local`] returns a
//! [`LocalResolution`] that says which of the three happened, and
//! [`Disambiguation`] lets a caller name the policy — earliest, latest,
//! refuse, or push forward — instead of having one chosen for them.
//!
//! ```
//! use hc_tz::{Disambiguation, LocalResolution, TimeZone, builtin};
//! use hc_calendar::{CivilDateTime, CivilTime, Rd};
//!
//! let new_york = builtin::zone("America/New_York")?;
//! // 2024-11-03 01:30 happened twice in New York.
//! let local = CivilDateTime::new(Rd(739_193), CivilTime::hms(1, 30, 0)?);
//! let resolution = new_york.resolve_local(local);
//! assert!(matches!(resolution, LocalResolution::Ambiguous { .. }));
//! assert_eq!(
//!     resolution.resolve(Disambiguation::Latest)?.seconds()
//!         - resolution.resolve(Disambiguation::Earliest)?.seconds(),
//!     3_600,
//! );
//! # Ok::<(), Box<dyn core::error::Error>>(())
//! ```
//!
//! # Timeline
//!
//! Everything here works in POSIX time, where every day is 86 400 seconds and
//! leap seconds do not exist. That is the timeline civil zone rules are
//! published in. Converting to elapsed physical time is
//! [`hc_core::unix`]'s job and needs a leap-second policy, which is a separate
//! decision from a zone's.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod builtin;
pub mod error;
pub mod fixed;
mod gregorian;
pub mod offset;
pub mod posix;
pub mod tzif;
pub mod zone;

#[cfg(feature = "std")]
pub mod system;

pub use error::{TzError, TzResult};
pub use fixed::{FixedTimeZone, Utc};
pub use offset::{MAX_OFFSET_SECONDS, OffsetStyle, OffsetText, UtcOffset};
pub use posix::{Abbreviation, PosixDst, PosixRule, PosixTimeZone, PosixTransition, PosixTz};
pub use tzif::{
    LeapSecond, LocalTimeType, Transition, TzifData, TzifHeader, TzifTimeZone, TzifVersion,
};
pub use zone::{Disambiguation, LocalResolution, TimeZone};

pub use hc_calendar;
pub use hc_core;
