//! Astronomical algorithms for `hyper-calendar`.
//!
//! This crate is the engine underneath every calendar that is defined by
//! where the Sun and Moon actually are rather than by a counting rule: the
//! Chinese and Dangi lunisolar calendars, the observational Hijri variants,
//! the Hebrew calendar's molad cross-checks, the 24 solar terms, and the
//! holiday rules that pin a date to an equinox or a solstice.
//!
//! # What it is
//!
//! * [`time`] — Universal Time to Terrestrial Time, and ΔT.
//! * [`earth`] — obliquity, nutation, sidereal time.
//! * [`vsop87`] — the Earth's heliocentric position from VSOP87, truncated
//!   to a measured quarter of a second of arc.
//! * [`solar`] — the Sun's apparent longitude, the search that solar terms
//!   are built on, and the equinoxes and solstices.
//! * [`lunar`] — the Moon's longitude, its phase, and the conjunction search.
//! * [`riseset`] — sunrise, sunset, twilight, moonrise and moonset for a
//!   [`riseset::Location`].
//!
//! # What it is not
//!
//! It is not an ephemeris. Every series here is a truncation chosen for the
//! question a calendar asks, which is always "on which *day* did this
//! happen": the Sun's longitude to about 0.01°, the Moon's to about 10″, a
//! conjunction to under a minute, a sunrise to under a minute of the model's
//! own geometry. If you need arcsecond positions, planetary positions or
//! eclipse circumstances, use a real ephemeris and convert.
//!
//! It also contains no calendar. Nothing here knows what a month is.
//!
//! # Time scales
//!
//! Every public function that takes a [`hc_calendar::fixed::Moment`]
//! takes it in **Universal Time**, and every one that returns a `Moment`
//! returns Universal Time. The conversion to Terrestrial Time, which is what
//! the series are actually stated in, happens inside. Published worked
//! examples are usually quoted in TT; [`time::universal_time`] bridges them.
//!
//! ```
//! use hc_astro::solar::{Equinox, equinox};
//!
//! // The March equinox of 2000 was 2000-03-20 07:35 UT.
//! let moment = equinox(2000, Equinox::March);
//! assert_eq!(moment.day(), hc_calendar::Rd(730_199));
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod earth;
pub mod lunar;
pub mod riseset;
pub mod solar;
pub mod time;
pub mod vsop87;

mod search;
mod util;

pub use earth::{Equatorial, Nutation, Obliquity, nutation, obliquity};
pub use lunar::{
    MEAN_SYNODIC_MONTH, MoonPhase, lunar_illuminated_fraction, lunar_longitude, lunar_phase,
    moon_phase_at_or_after, new_moon_at_or_after, new_moon_before, nth_new_moon,
};
pub use riseset::{Location, Twilight, dawn, dusk, moonrise, moonset, solar_noon, sunrise, sunset};
pub use solar::{
    Equinox, MEAN_TROPICAL_YEAR, Solstice, equinox, solar_longitude, solar_longitude_after,
    solstice,
};
pub use time::{delta_t, dynamical_time, julian_centuries, universal_time};

pub use hc_calendar;
pub use hc_calendar::fixed::Moment;
