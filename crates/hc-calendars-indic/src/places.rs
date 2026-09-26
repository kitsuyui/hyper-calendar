//! The places a Hindu reckoning is judged at.
//!
//! A tithi is one instant for the whole Earth; which *day* carries it
//! depends on whose sunrise is asked. Every calendar in this crate takes a
//! [`Location`] for that, and these are the ones the sources name.

use hc_astro::riseset::Location;

/// The adopted Central Station of India, 23°11′ N 82°30′ E: the point the
/// Calendar Reform Committee of 1955 fixed for the national calendar
/// (`crc1955`, p. 4), whose local mean time is Indian Standard Time to the
/// second, and whose sunrise the *Rashtriya Panchang* reads each day's
/// tithi at (`rashtriya-panchang-1945`). At sea level, so that sunrise is
/// the almanac's sunrise.
pub const CENTRAL_STATION: Location = Location::new(23.183_333, 82.5, 0.0);

/// Ujjain, 23°9′ N, 75°46′6″ E — the *Ujjayinī* of the classical
/// astronomers, whose meridian was the prime meridian of Indian astronomy
/// and the reference Reingold and Dershowitz keep for every Hindu
/// computation, at the coordinates their published code gives
/// (`reingold2018code`, `ujjain`: `(angle 23 9 0) (angle 75 46 6)`), the
/// same longitude [`crate::surya_siddhanta::UJJAIN_LONGITUDE_DEGREES`]
/// uses. The city's modern coordinates, 23.1765° N, 75.7885° E, lie about
/// a minute of arc away, five seconds of sunrise; over 1700–2299 the two
/// put a different sunrise tithi at Ujjain on sixteen days, 7 January 1995
/// the only one in the twentieth century.
pub const UJJAIN: Location = Location::new(23.15, 75.0 + 46.0 / 60.0 + 6.0 / 3_600.0, 0.0);

/// Kathmandu, 27°42′36″ N 85°19′12″ E — the city whose sunrise the
/// Nepal Sambat is judged at here (Wikipedia, "Kathmandu", retrieved
/// 2026-09-23, for the coordinates). At sea level, as the other places
/// are, so that its sunrise is the one computed for the horizon rather
/// than for the valley floor at 1 400 m.
pub const KATHMANDU: Location = Location::new(27.71, 85.32, 0.0);
