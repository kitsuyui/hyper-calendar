//! Relativistic time for `hyper-calendar`.
//!
//! Clocks do not agree. One in orbit runs fast, one on a fast ship runs slow,
//! and the difference is large enough to matter: a GPS satellite gains 38
//! microseconds a day, which is ten kilometres of positioning error. This
//! crate computes those differences well enough to build a timeline with —
//! a satellite constellation's, or a science-fiction novel's.
//!
//! * [`constants`] — `c`, `G`, and the standard gravitational parameters of
//!   the Sun, Earth, Moon, Mars, Jupiter and Sagittarius A\*, each with the
//!   authority it came from and an honest note on how well it is known.
//! * [`special`] — Lorentz factors, rapidity, velocity composition, proper
//!   time, Doppler and aberration.
//! * [`gravitational`] — the Schwarzschild static factor, gravitational
//!   redshift, the Schwarzschild radius, and the combined rate offset of a
//!   circular orbit, which is the GPS calculation.
//! * [`worldline`] — piecewise paths through spacetime, integrated in closed
//!   form, plus the relativistic-rocket relations.
//! * [`dilated`] — the two clocks attached to real [`hc_core::Instant`]s, so
//!   a ship's calendar and Earth's can be printed side by side.
//!
//! # Anchors
//!
//! Each of these is a published figure, and each is a test in this crate:
//!
//! | Quantity | Value |
//! | --- | --- |
//! | Lorentz factor at β = 0.6 | exactly 1.25 |
//! | GPS gravitational gain | +45.7 µs/day |
//! | GPS kinematic loss | −7.2 µs/day |
//! | GPS net gain | +38.4 µs/day |
//! | Schwarzschild radius of the Sun | 2.95 km |
//! | 1 g flip-and-burn to Andromeda (2.5 Mly) | 28.6 years aboard |
//!
//! # What this crate does not model
//!
//! The metric is Schwarzschild: non-rotating, uncharged, spherically
//! symmetric. The Earth's quadrupole moment, the Kerr frame-dragging term,
//! the rotation of a ground station, orbital eccentricity and the Sagnac
//! effect are all absent. Each of them moves the GPS figures by nanoseconds
//! per day, not microseconds, but a real time-transfer system needs all of
//! them and should not use this crate as though it had them.
//!
//! No `f32` appears anywhere, and no result is allowed to become a `NaN`:
//! `β ≥ 1` and a radius inside the horizon are [`RelativityError`]s, caught
//! where they occur rather than propagated silently into an arrival date.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod constants;
pub mod dilated;
pub mod error;
pub mod gravitational;
pub mod special;
pub mod worldline;

mod hyperbolic;

pub use constants::{
    ASTRONOMICAL_UNIT, GM_EARTH, GM_JUPITER, GM_MARS, GM_MOON, GM_SAGITTARIUS_A_STAR, GM_SUN,
    GRAVITATING_BODIES, GRAVITATIONAL_CONSTANT, GravitatingBody, JULIAN_YEAR_SECONDS, LIGHT_YEAR,
    SPEED_OF_LIGHT, STANDARD_GRAVITY,
};
pub use dilated::{ClockComparison, compare_clocks, dilated_instant};
pub use error::{RelativityError, RelativityResult};
pub use gravitational::{schwarzschild_radius, static_dilation_factor};
pub use special::{lorentz_factor, proper_time_of, rapidity};
pub use worldline::{GravitationalPotential, Segment, VelocityProfile, Worldline};

pub use hc_core;
pub use hc_uncertainty;
