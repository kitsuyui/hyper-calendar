//! The places the equinox calendars are judged at.
//!
//! An equinox is one instant for the whole Earth; which *day* it falls on
//! depends on whose clock is asked. Each calendar in this crate names its
//! clock, and these are the places behind them.

use hc_astro::riseset::Location;

/// Tehran, as Reingold and Dershowitz place it for both the Solar Hijri
/// and the Badíʿ calendars: 35.696111° N, 51.423056° E, 1 100 m above sea
/// level (*Calendrical Calculations*, `tehran`).
pub const TEHRAN: Location = Location::new(35.696_111, 51.423_056, 1_100.0);

/// The standard meridian of Iran Standard Time, UTC+03:30.
pub const IRAN_STANDARD_MERIDIAN_DEGREES: f64 = 52.5;

/// Iran Standard Time's offset from Universal Time, as a fraction of a day.
pub const IRAN_STANDARD_OFFSET_DAYS: f64 = 3.5 / 24.0;

/// The Paris Observatory: 48.836389° N, 2.336389° E, 27 m — the place the
/// Republican calendar's decree named for its equinox
/// (*Calendrical Calculations*, `paris`).
pub const PARIS_OBSERVATORY: Location = Location::new(48.836_389, 2.336_389, 27.0);
