//! The calendar abstraction shared by every calendar in `hyper-calendar`.
//!
//! # The pivot
//!
//! There is no useful "common calendar". What every calendar *does* share is
//! a way to name a day, and days can be counted. This crate makes that count
//! the pivot: [`Rd`], the Rata Die fixed day number, with day 1 being
//! `0001-01-01` in the proleptic Gregorian calendar. Every calendar
//! implements exactly two operations against it:
//!
//! ```text
//! fields  --to_fixed-->  Rd  --from_fixed-->  fields
//! ```
//!
//! An `n`-calendar library then needs `2n` conversions instead of `n²`, and
//! two calendars can be displayed side by side without either knowing the
//! other exists. This is the design from Reingold and Dershowitz's
//! *Calendrical Calculations*, and it is what makes requirement 4 —
//! separating data from algorithm — mechanical rather than aspirational.
//!
//! # Two levels of interface
//!
//! * [`Calendar`] is the static one: each calendar has its own `Date` type,
//!   so a Hebrew date cannot be passed where a Gregorian one is expected.
//! * [`DynCalendar`] is the object-safe one: dates become [`DateFields`], so
//!   a registry can hold calendars it was not compiled against and an FFI
//!   caller can name one by string.
//!
//! Every calendar implements `Calendar`; [`DynAdapter`] derives the dynamic
//! one from it, so no calendar author writes the bridge twice.
//!
//! # Cycles, which are not calendars
//!
//! Some ways of naming a day are not calendars at all: they repeat without
//! counting. [`weekday`] holds the seven-day week and the four-, five- and
//! ten-day weeks that are not seven days long; [`cycle`] holds the East Asian
//! sexagenary cycle (干支) — the stems and branches, the twelve double-hours
//! (十二時辰), and the four pillars (四柱 / 八字) of year, month, day and
//! hour, with each pillar's boundary documented because they all differ. Both
//! modules are pure functions of [`Rd`], which is why they live here rather
//! than in a calendar crate, and both stop at the point where astronomy would
//! be needed: the solar term that fixes a month pillar is an *argument*,
//! supplied by `hc-seasons`.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod cycle;
pub mod error;
pub mod fields;
pub mod fixed;
pub mod time;
pub mod traits;
pub mod weekday;

#[cfg(feature = "alloc")]
pub mod registry;

pub use error::{CalendarError, CalendarResult};
pub use fields::{DateFields, Month, YearKind};
pub use fixed::{Rd, moment_to_rd, rd_to_moment};
pub use time::{CivilDateTime, CivilTime};
pub use traits::{Calendar, CalendarId, CalendarMeta, DynAdapter, DynCalendar};
pub use weekday::Weekday;

#[cfg(feature = "alloc")]
pub use registry::CalendarRegistry;

pub use hc_core;
