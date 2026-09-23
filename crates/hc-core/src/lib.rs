//! Core time primitives for `hyper-calendar`.
//!
//! This crate owns the parts of the model that every other crate needs and
//! that nothing else is allowed to redefine:
//!
//! * [`Duration`] — an exact, arbitrary-sign span of SI seconds with
//!   attosecond resolution.
//! * [`Instant`] — a reading on one of the uniform physical time scales,
//!   parameterised by a zero-sized [`scale::TimeScale`] marker so that a TAI
//!   value can never be silently used where a TT value is expected.
//! * [`leap`] — the UTC leap-second table, which is *data*, not algorithm.
//! * [`epoch`] — well-known epochs expressed in the one canonical scale.
//!
//! # Design rules
//!
//! * No calendar knowledge lives here. A calendar turns a day number into
//!   fields; that is [`hc-calendar`](https://docs.rs/hc-calendar)'s job.
//! * The crate is `no_std`-compatible: enable `std` (the default) or, for a
//!   `no_std` build, `libm` to get floating-point math.
//! * Everything that can be exact is exact. Floating point appears only where
//!   the underlying physics is itself a fitted model (TDB, UT1).

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod catalogue;
pub mod duration;
pub mod epoch;
pub mod error;
pub mod leap;
pub mod math;
pub mod scale;
pub mod unix;

pub use duration::{ATTOS_PER_SEC, Duration};
pub use error::{TimeError, TimeResult};
pub use scale::{Gps, Instant, Tai, Tcb, Tcg, Tdb, TimeScale, TimeScaleId, Tt, Ut1};
pub use unix::UnixTime;
