//! A universal calendar and time library.
//!
//! `hyper-calendar` aims to cover the whole of what "a time" can mean: the
//! international standards, the everyday conveniences, as many of the world's
//! calendars as can be modelled at all, the scales that ordinary calendars
//! cannot reach in either direction, clocks on other planets, and the
//! relativistic corrections that make a science-fiction timeline computable.
//!
//! This crate is a facade. It contains almost no logic of its own; it
//! re-exports the workspace's crates behind features and adds the ergonomic
//! layer in [`civil`].
//!
//! # Picking what to compile
//!
//! Requirement 8 of this project's brief is that only what is used should be
//! compiled in, so the default is deliberately modest and everything else is
//! opt-in:
//!
//! | Feature | Brings in | For |
//! | --- | --- | --- |
//! | `civil` *(default)* | [`hc_calendar`], [`hc_calendars_solar`] | Gregorian-family dates |
//! | `units` | [`hc_units`] | Flicks, helakim, decimal time, BPM |
//! | `format` *(default)* | [`hc_format`] | ISO 8601, RFC 3339, patterns |
//! | `i18n` *(default)* | [`hc_i18n`] | Locales, plural rules, names |
//! | `lunar` | [`hc_astro`], [`hc_calendars_lunar`] | Hijri, Hebrew, Chinese, Tenpō |
//! | `regional` | [`hc_calendars_regional`] | Japanese eras, Maya, Pawukon |
//! | `astro` | [`hc_astro`] | Solar longitude, phases, rise and set |
//! | `seasons` | [`hc_seasons`] | 24 solar terms, 72 pentads, zassetsu |
//! | `almanac` | [`hc_almanac`] | 六曜, 二十八宿, 九星, 暦注下段, 選日 |
//! | `fiscal` | [`hc_fiscal`] | 年度, fiscal, tax and academic years |
//! | `attributes` | [`hc_attributes`] | Birthstones, birth flowers, moon names |
//! | `tz` | [`hc_tz`] | Time zones |
//! | `humanize` | [`hc_humanize`] | "3 days ago" |
//! | `holiday` | [`hc_holiday`] | Holidays and observances |
//! | `uncertainty` | [`hc_uncertainty`] | Significant figures, fuzzy dates, EDTF |
//! | `deep-time` | [`hc_deep_time`] | Planck time to cosmology |
//! | `planetary` | [`hc_planetary`] | Mars sols, other bodies |
//! | `relativity` | [`hc_relativity`] | Time dilation, worldlines |
//! | `full` | all of the above | |
//!
//! `std` is on by default. Turning it off leaves a `no_std` build; add
//! `alloc` for the parts that need an allocator and `libm` for floating-point
//! math on targets that lack it.
//!
//! # Where to start
//!
//! * [`civil`] for dates and times — the `datetime`-shaped layer.
//! * [`hc_core`] for exact durations and physical time scales.
//! * [`hc_calendar`] for the calendar abstraction and the registry.
//! * The project's `docs/` directory for the design and the coverage tables.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub use hc_core;

pub use hc_core::{
    Duration, Gps, Instant, Tai, Tcb, Tcg, Tdb, TimeError, TimeResult, TimeScale, TimeScaleId, Tt,
    UnixTime, Ut1,
};

#[cfg(feature = "civil")]
pub use hc_calendar;
#[cfg(feature = "civil")]
pub use hc_calendars_solar;

#[cfg(feature = "civil")]
pub use hc_calendar::{
    CalendarError, CalendarResult, CivilDateTime, CivilTime, DateFields, Month, Rd, Weekday,
};

#[cfg(feature = "civil")]
pub mod civil;

#[cfg(feature = "almanac")]
pub use hc_almanac;
#[cfg(feature = "astro")]
pub use hc_astro;
#[cfg(feature = "attributes")]
pub use hc_attributes;
#[cfg(feature = "lunar")]
pub use hc_calendars_lunar;
#[cfg(feature = "regional")]
pub use hc_calendars_regional;
#[cfg(feature = "deep-time")]
pub use hc_deep_time;
#[cfg(feature = "fiscal")]
pub use hc_fiscal;
#[cfg(feature = "format")]
pub use hc_format;
#[cfg(feature = "holiday")]
pub use hc_holiday;
#[cfg(feature = "humanize")]
pub use hc_humanize;
#[cfg(feature = "i18n")]
pub use hc_i18n;
#[cfg(feature = "planetary")]
pub use hc_planetary;
#[cfg(feature = "relativity")]
pub use hc_relativity;
#[cfg(feature = "seasons")]
pub use hc_seasons;
#[cfg(feature = "tz")]
pub use hc_tz;
#[cfg(feature = "uncertainty")]
pub use hc_uncertainty;
#[cfg(feature = "units")]
pub use hc_units;

/// The version of this crate, for FFI callers and bug reports.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Everything most callers want, in one import.
///
/// The prelude follows the enabled features, so `use hyper_calendar::prelude::*`
/// brings in exactly what was compiled and nothing that was not.
pub mod prelude {
    pub use hc_core::unix::{LeapPolicy, UtcInstant};
    pub use hc_core::{Duration, Instant, Tai, TimeScale, Tt, UnixTime};

    #[cfg(feature = "civil")]
    pub use crate::civil::{Date, DateTime, Time, TimeDelta};
    #[cfg(all(feature = "civil", feature = "alloc"))]
    pub use hc_calendar::CalendarRegistry;
    #[cfg(feature = "civil")]
    pub use hc_calendar::{Calendar, CalendarError, DynCalendar, Rd, Weekday};
    #[cfg(feature = "civil")]
    pub use hc_calendars_solar::{GregorianCalendar, GregorianDate};

    #[cfg(feature = "tz")]
    pub use hc_tz::{LocalResolution, TimeZone, UtcOffset};

    #[cfg(feature = "i18n")]
    pub use hc_i18n::Locale;

    #[cfg(feature = "uncertainty")]
    pub use hc_uncertainty::{FuzzyInstant, Uncertain};

    #[cfg(feature = "units")]
    pub use hc_units::{Quantity, Ratio, Tempo, Unit};
}

/// A registry populated with every calendar the enabled features provide.
///
/// This is the answer to requirement 4 of the project brief — several
/// calendars in use at once — in one call. Which calendars appear depends on
/// which features are on, so a build that only wants Gregorian dates does not
/// carry a lunar ephemeris to get them.
#[cfg(all(feature = "civil", feature = "alloc"))]
#[must_use]
pub fn registry() -> hc_calendar::CalendarRegistry {
    let mut registry = hc_calendar::CalendarRegistry::new();
    hc_calendars_solar::register_all(&mut registry);
    #[cfg(feature = "lunar")]
    hc_calendars_lunar::register_all(&mut registry);
    #[cfg(feature = "regional")]
    hc_calendars_regional::register_all(&mut registry);
    registry
}
