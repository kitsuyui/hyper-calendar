//! The places a Hindu reckoning is judged at.
//!
//! A tithi is one instant for the whole Earth; which *day* carries it
//! depends on whose sunrise is asked. Every calendar in this crate takes a
//! [`Location`] for that, and these are the ones the sources name.

use hc_astro::riseset::Location;

/// The adopted Central Station of India, 23°11′ N 82°30′ E: the point the
/// Calendar Reform Committee of 1955 fixed for the national calendar, whose
/// local mean time is Indian Standard Time to the second, and whose
/// sunrise the *Rashtriya Panchang* reads each day's tithi at. At sea
/// level, so that sunrise is the almanac's sunrise.
pub const CENTRAL_STATION: Location = Location::new(23.183_333, 82.5, 0.0);

/// Ujjain, 23.1765° N, 75.7885° E — the *Ujjayinī* of the classical
/// astronomers, whose meridian was the prime meridian of Indian astronomy
/// and the reference Reingold and Dershowitz keep for every Hindu
/// computation (*Calendrical Calculations*, `ujjain`).
pub const UJJAIN: Location = Location::new(23.176_5, 75.788_5, 0.0);

/// New Delhi, 28.6139° N, 77.2090° E — the city the Government of India's
/// holiday lists are drawn up for, and the first of the four whose sunrise
/// the *Rashtriya Panchang* tabulates.
pub const NEW_DELHI: Location = Location::new(28.613_9, 77.209_0, 0.0);

/// Kathmandu, 27°42′36″ N 85°19′12″ E — the city whose sunrise the
/// Nepal Sambat is judged at here (Wikipedia, "Kathmandu", retrieved
/// 2026-09-23, for the coordinates). At sea level, as the other places
/// are, so that its sunrise is the one computed for the horizon rather
/// than for the valley floor at 1 400 m.
pub const KATHMANDU: Location = Location::new(27.71, 85.32, 0.0);
