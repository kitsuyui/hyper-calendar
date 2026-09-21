//! Deep time: the spans an ordinary calendar cannot reach, in both
//! directions.
//!
//! [`hc_core::Duration`] is exact — `i128` seconds plus attoseconds — and it
//! covers everything a clock can measure. This crate covers what lies outside
//! that: below an attosecond, where the Planck time sits twenty-six decades
//! further down, and above the span where "a count of seconds" is a sensible
//! answer at all. Values out there are not exact. They are *published
//! magnitudes with error bars*, so that is what this crate stores.
//!
//! | Module | What it holds |
//! | --- | --- |
//! | [`magnitude`] | [`DeepTime`], a span in seconds as an uncertain magnitude, with logarithmic comparison and rendering |
//! | [`constants`] | The Planck units from CODATA 2022, each with its stated relative uncertainty |
//! | [`universe`] | The chronology of the universe as data, on Planck 2018 parameters |
//! | [`future`] | The far future as data, out to 10¹⁰⁰ years and past it |
//! | [`geologic`] | The ICS International Chronostratigraphic Chart as a queryable tree |
//! | [`archaeology`] | The `BP` convention, and the calibrated/uncalibrated distinction |
//! | [`timeline`] | All of the above, queried together |
//!
//! # The one rule
//!
//! Every numeric entry in every table carries an uncertainty and names a
//! source. A timeline that says the Hadean began 4.567 Ga with no error bar is
//! not a timeline, it is a decoration.
//!
//! ```
//! use hc_deep_time::{DeepTime, universe};
//!
//! let now = universe::AGE_OF_UNIVERSE.deep_time().unwrap();
//! let planck = DeepTime::from_planck_times(1.0, 0.0).unwrap();
//! // Sixty-one decades from the Planck time to the present.
//! assert!((now.orders_of_magnitude_between(planck).unwrap() - 60.9).abs() < 0.1);
//! ```
//!
//! # What this crate deliberately does not do
//!
//! It is not a cosmology solver and not a physics engine. It carries published
//! values and does arithmetic on them with the error bars intact; computing a
//! value from a cosmological model is the caller's job.
//!
//! It does not calibrate radiocarbon dates. Turning an uncalibrated
//! radiocarbon age into a calendar year needs IntCal20 and its marine and
//! southern-hemisphere companions, which are large datasets with their own
//! release cadence. [`archaeology`] models the *distinction* and refuses the
//! conversion rather than pretending the two are the same thing.
//!
//! It does not know about calendars. Everything here is a span in SI seconds,
//! or a count of years before a stated epoch.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod archaeology;
pub mod constants;
pub mod error;
pub mod future;
pub mod geologic;
pub mod magnitude;
pub mod periods;
pub mod timeline;
pub mod universe;

pub use error::{DeepTimeError, DeepTimeResult};
pub use magnitude::{DeepTime, DeepUnit, LogMagnitude};

pub use archaeology::{ArchaeologicalPeriod, Bp, Calibration};
pub use constants::PhysicalConstant;
pub use future::{FutureEra, FutureEvent, Prediction};
pub use geologic::{GeologicInterval, GeologicRank};
pub use periods::{AstronomicalPeriod, GALACTIC_YEAR, Stability};
pub use timeline::{Placement, place, place_megayears_ago, place_years_ago, span_between};
pub use universe::{CosmicEpoch, CosmicEvent};

pub use hc_core;
pub use hc_uncertainty;
