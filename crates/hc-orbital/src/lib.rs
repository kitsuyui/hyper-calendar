//! Milankovitch orbital elements: the slow cycles that paced the ice ages.
//!
//! Over tens of thousands of years the shape of Earth's orbit, the tilt of
//! its axis and the season in which it is nearest the Sun all change, and
//! with them the sunlight each latitude receives in each season. This crate
//! evaluates Berger's 1978 trigonometric solution for those elements — the
//! one the PMIP palaeoclimate experiments prescribe and the NCAR and GISS
//! climate models carry — and the daily insolation that follows from them.
//!
//! | Module | What it holds |
//! | --- | --- |
//! | [`elements`] | [`elements_at`]: eccentricity, obliquity, the longitude of perihelion from the moving equinox and the climatic precession *e* sin ϖ at an epoch, each with a measured spread; the epoch, the span and the year conversion |
//! | [`insolation`] | [`daily_insolation`] and [`insolation_65n_june`], with the solar constant as a named parameter |
//! | [`series`] | The three coefficient tables, 19 + 47 + 78 terms, as the author deposited them |
//!
//! # Epoch, span, refusal
//!
//! "Present" is the series' own: **1950 CE**, [`EPOCH_YEAR`]. Every function
//! takes years before 1950, negative for the future, and
//! [`years_before_present_from_year`] converts from a calendar year. The
//! series is answered over [`VALID_SPAN`], a million years either side, and
//! refused beyond it with [`OrbitalError::OutsideValidSpan`]: a
//! trigonometric fit does not fade past its span, it lies (ADR 0006).
//!
//! # How this relates to `hc-astro`
//!
//! `hc-astro` carries the obliquity too, as Laskar's 1986 polynomial (Meeus
//! 22.3), good to 0.01″ over a thousand years and a few arcseconds over ten
//! thousand, and useless beyond. This series is the opposite: coarse now —
//! its 1950 obliquity is within 0.001° of the polynomial's, its eccentricity
//! within 2 × 10⁻⁵ of the J2000 value — and still meaningful a hundred
//! thousand years out. For a date, use `hc-astro`; for an ice age, this.
//!
//! # What it does not carry
//!
//! Not Berger & Loutre's 1991 solution, nor Laskar et al.'s 2004 numerical
//! one (La2004), which is the modern reference and is distributed as tables
//! of megabytes. Either would be a second named solution beside this one
//! (policy §5), not a replacement, and the system document says what
//! adding it would take.
//!
//! ```
//! use hc_orbital::{SOLAR_CONSTANT_BERGER_LOUTRE_1991, elements_at, insolation_65n_june};
//!
//! let lgm = elements_at(21_000.0).unwrap();
//! assert!((lgm.obliquity_degrees.value - 22.949).abs() < 0.001);
//! let june = insolation_65n_june(21_000.0, SOLAR_CONSTANT_BERGER_LOUTRE_1991).unwrap();
//! assert!(june < insolation_65n_june(0.0, SOLAR_CONSTANT_BERGER_LOUTRE_1991).unwrap());
//! ```
//!
//! The explanation, the worked example and the accuracy measurements are in
//! `docs/systems/orbital-elements.md`; the module documentation summarises
//! and does not repeat them.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod elements;
pub mod error;
pub mod insolation;
pub mod series;

pub use elements::{
    EPOCH_YEAR, OrbitalElements, SOURCE, SPREAD_TIERS, Spread, VALID_SPAN, elements_at, spread_at,
    year_from_years_before_present, years_before_present_from_year,
};
pub use error::{OrbitalError, OrbitalResult};
pub use insolation::{
    LATITUDE_65N, MID_JUNE_SOLAR_LONGITUDE, SOLAR_CONSTANT_BERGER_1978,
    SOLAR_CONSTANT_BERGER_LOUTRE_1991, SOLAR_CONSTANT_INSOL14, daily_insolation,
    insolation_65n_june,
};
pub use series::Term;

pub use hc_core;
pub use hc_uncertainty;
