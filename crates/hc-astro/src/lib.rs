//! Astronomical algorithms for `hyper-calendar`.
//!
//! This crate is the engine underneath every calendar that is defined by
//! where the Sun and Moon actually are rather than by a counting rule: the
//! Chinese and Dangi lunisolar calendars, the observational Hijri variants,
//! the Hindu calendars, the 24 solar terms, and the holiday rules that pin a date to an equinox or a solstice.
//!
//! # What it is
//!
//! * [`time`] — Universal Time to Terrestrial Time, and ΔT.
//! * [`delta_t_table`] — the observed ΔT, 1974–2026, and the USNO's
//!   predictions to 2033, that [`time`] reads before falling back to its
//!   polynomials.
//! * [`ut1`] — the UT1 time scale, from ΔT or from a published DUT1 series.
//! * [`ut_variants`] — UT2, UT1R and UT1S, the smoothed readings of UT1.
//! * [`gmat`] — Greenwich Mean Astronomical Time, the noon-based day of
//!   the *Nautical Almanac* before 1925.
//! * [`earth`] — obliquity, nutation, the Earth Rotation Angle, and
//!   sidereal time by the IAU 1982 and the IAU 2006 conventions.
//! * [`vsop87`] — the Earth's heliocentric position from VSOP87, truncated
//!   to a measured quarter of a second of arc.
//! * `vsop87_jupiter` and `jupiter`, behind the `jupiter` feature — Jupiter's
//!   position from the complete VSOP87B series (3 625 terms, 55 kB of
//!   tables): heliocentric, and apparent as seen from the Earth, to a
//!   measured half an arcsecond from 1500 to 2500.
//! * [`solar`] — the Sun's apparent longitude, the search that solar terms
//!   are built on, and the equinoxes and solstices.
//! * [`lunar`] — the Moon's longitude, its phase, and the conjunction search.
//! * [`riseset`] — sunrise, sunset, twilight, moonrise and moonset for a
//!   [`riseset::Location`].
//! * [`horizon`] — the named horizons a rising or a setting is measured
//!   against: the default, the USNO's and *Calendrical Calculations*'.
//! * [`solar_time`] — local mean and local apparent (sundial) time, the
//!   unequal hours, and religious times of day, the Islamic prayer times by
//!   named method among them.
//! * [`hjd`] — the Heliocentric Julian Date, HJD_TT and HJD_UTC.
//! * [`search`] — the bisections every search above is built on, public so
//!   that the calendars with their own models of the sky use them too.
//!
//! # What it does not carry
//!
//! Every series here is a truncation chosen for the question a calendar
//! asks, which is always "on which *day* did this happen": the Sun's
//! longitude to about 1″, the Moon's to about 10″, a conjunction to under a
//! minute, a sunrise to under a minute of the model's own geometry.
//!
//! **Not carried: a general ephemeris.** The planets other than Jupiter,
//! and eclipse circumstances, are not computed, and no position is carried
//! to better than the truncations above. The one planet carried is Jupiter,
//! with the `jupiter` feature, for the festivals its sign sets; no series
//! file for another planet was read, which `docs/systems/jupiter-ephemeris.md`
//! records, and no eclipse model has been written yet.
//!
//! A calendar is not carried here either: nothing in this crate knows what
//! a month is. The calendars that use these searches are in the crates that
//! own them.
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

pub mod delta_t_model;
pub mod delta_t_table;
pub mod earth;
pub mod gmat;
pub mod hjd;
pub mod horizon;
#[cfg(feature = "jupiter")]
pub mod jupiter;
pub mod lunar;
pub mod riseset;
pub mod search;
pub mod solar;
pub mod solar_time;
pub mod time;
pub mod ut1;
pub mod ut_variants;
pub mod vsop87;
#[cfg(feature = "jupiter")]
pub mod vsop87_jupiter;

// `check_days`: run a check on each day of a sweep, spread over the
// machine's threads (docs/policy.md §7).
hc_core::check_days_in_parallel!();

pub use delta_t_model::{
    DELTA_T_MODELS, DeltaTModel, ESPENAK_MEEUS_2006, MORRISON_STEPHENSON_2021,
};
pub use earth::{Equatorial, Nutation, Obliquity, nutation, obliquity};
pub use horizon::{HORIZONS, Horizon};
pub use lunar::{
    MEAN_SYNODIC_MONTH, MoonPhase, lunar_illuminated_fraction, lunar_longitude, lunar_phase,
    moon_phase_at_or_after, new_moon_at_or_after, new_moon_before, nth_new_moon,
};
pub use riseset::{
    Location, Twilight, dawn, dusk, moonrise, moonrise_with, moonset, moonset_with, solar_noon,
    sunrise, sunrise_with, sunset, sunset_with,
};
pub use solar::{
    Equinox, MEAN_TROPICAL_YEAR, Solstice, equinox, solar_longitude, solar_longitude_after,
    solstice,
};
pub use time::{delta_t, delta_t_with, dynamical_time, julian_centuries, universal_time};
pub use ut1::{Ut1, Ut1Offsets};

pub use hc_calendar::fixed::Moment;
