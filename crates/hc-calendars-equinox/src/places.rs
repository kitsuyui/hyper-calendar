//! The places the equinox calendars are judged at.
//!
//! An equinox is one instant for the whole Earth; which *day* it falls on
//! depends on whose clock is asked. Each calendar in this crate names its
//! clock, and these are the places behind them.

use hc_astro::riseset::Location;

/// Tehran, 35.696111° N, 51.423056° E, with its sunset taken as an almanac
/// tabulates one, against a sea-level horizon — the location Reingold and
/// Dershowitz use for the astronomical Badíʿ calendar (*Calendrical
/// Calculations*, `bahai-location`, elevation 0 m).
///
/// Their `tehran`, used for the Solar Hijri calendar, gives the city its
/// 1 100 m of elevation, which dips the horizon and puts sunset about five
/// minutes later. The Bahá'í World Centre's table agrees with the sea-level
/// horizon on the one row that can tell: on 20 March 2026 the equinox and
/// the sea-level sunset fall within seconds of each other, and the table
/// puts Naw-Rúz on the 21st — the equinox after sunset — where a dipped
/// horizon would put it five minutes before. See
/// `docs/systems/equinox-calendars.md` in the repository.
pub const TEHRAN: Location = Location::new(35.696_111, 51.423_056, 0.0);

/// The standard meridian of Iran Standard Time, UTC+03:30.
pub const IRAN_STANDARD_MERIDIAN_DEGREES: f64 = 52.5;

/// Iran Standard Time's offset from Universal Time, as a fraction of a day.
pub const IRAN_STANDARD_OFFSET_DAYS: f64 = 3.5 / 24.0;

/// The Paris Observatory: 48°50′11″ N, 2°20′15″ E, 27 m — the place the
/// Republican calendar's decree named for its equinox, as Reingold and
/// Dershowitz give it (*Calendrical Calculations*, `paris`), the longitude
/// tied to the 9 minutes 21 seconds between Paris time and Universal Time.
pub const PARIS_OBSERVATORY: Location = Location::new(48.836_389, 2.337_5, 27.0);
