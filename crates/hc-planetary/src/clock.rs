//! One interface for asking the time anywhere in the solar system.
//!
//! [`BodyClock`] turns an [`Instant<Tai>`] into a local mean solar time on any
//! body in [`crate::bodies`], so that "what time is it at Olympus Mons" and
//! "what time is it on Titan" are the same question asked twice.
//!
//! # What a generic clock can and cannot promise
//!
//! * **The rate is physical.** A local hour is 1/24 of that body's solar day,
//!   derived in [`crate::bodies`] from measured rotation and orbit. On Mars a
//!   local hour is 61.6 minutes; on Titan it is 15.97 hours; on Venus it is
//!   4.86 Earth days.
//! * **The zero point is mostly not.** Only Earth and Mars have a
//!   standardised one — Universal Time and Coordinated Mars Time. For every
//!   other body this crate *declares* J2000.0 to be local mean midnight at the
//!   prime meridian, which is reproducible and documented and is nobody else's
//!   convention. [`BodyClock::is_standardised`] says which you are holding, and
//!   [`crate::bodies::ClockEpoch::note`] says what it is.
//! * **Mean, not true.** This is mean solar time throughout: the fictitious
//!   Sun that moves uniformly. The equation of time — the difference from what
//!   a sundial reads — is modelled only for Mars, in
//!   [`crate::mars::MarsMoment::equation_of_time_degrees`], and only for Mars
//!   is there a series in this crate good enough to compute it.
//! * **Uniform, not observed.** The clock ticks at a constant rate from the
//!   epoch. For Earth that means it is UT-like but does not follow the planet's
//!   actual rotation, which wanders; ΔT is `hc-astro`'s subject.
//!
//! # Which way round the longitude goes
//!
//! Local time runs *ahead* in the direction the body's rotation carries a
//! place towards the Sun. For a prograde rotator that is eastward, so local
//! time is the prime meridian's time minus the west longitude, exactly as on
//! Earth. For a **retrograde** rotator — Venus, Uranus, Pluto, Triton, Charon
//! — the Sun rises in the west and the sign flips. [`BodyClock`] handles this
//! from [`crate::bodies::Body::is_retrograde_rotator`]; a caller passing a
//! longitude does not have to think about it.

use core::fmt;

use hc_core::math::floor;
use hc_core::{Duration, Instant, Tai, TimeResult};

use crate::bodies::{Body, EpochBasis, by_name, whole_and_fraction};
use crate::util::{j2000_offset_days, modulo};

/// A local time on some body: which local day, and how far through it.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LocalTime {
    day_number: i64,
    day_fraction: f64,
    solar_day_seconds: f64,
}

impl LocalTime {
    /// The local day number, counted from the body's clock epoch.
    ///
    /// On Earth this is the Rata Die fixed day; on Mars it is the Mars Sol
    /// Date's integer part; on the Moon it is the Meeus lunation number; on
    /// everything else it counts from a day this crate declared.
    #[must_use]
    pub const fn day_number(self) -> i64 {
        self.day_number
    }

    /// How far through the local day, in `[0, 1)`.
    #[must_use]
    pub const fn day_fraction(self) -> f64 {
        self.day_fraction
    }

    /// The reading in decimal local hours, in `[0, 24)`.
    #[must_use]
    pub fn decimal_hours(self) -> f64 {
        self.day_fraction * 24.0
    }

    /// The local hour, in `0..=23`.
    #[must_use]
    pub fn hour(self) -> u8 {
        floor(self.decimal_hours()) as u8
    }

    /// The local minute, in `0..=59`.
    #[must_use]
    pub fn minute(self) -> u8 {
        floor(modulo(self.decimal_hours(), 1.0) * 60.0) as u8
    }

    /// The local second, in `0..=59`.
    #[must_use]
    pub fn second(self) -> u8 {
        floor(modulo(self.decimal_hours() * 60.0, 1.0) * 60.0) as u8
    }

    /// The length of this body's solar day, in SI seconds.
    #[must_use]
    pub const fn solar_day_seconds(self) -> f64 {
        self.solar_day_seconds
    }

    /// How long a local hour is in SI seconds.
    ///
    /// On Earth 3600; on Mars 3699; on Titan 57 490.
    #[must_use]
    pub fn local_hour_seconds(self) -> f64 {
        self.solar_day_seconds / 24.0
    }

    /// SI time elapsed since local midnight.
    ///
    /// # Errors
    ///
    /// Returns [`hc_core::TimeError::NotFinite`] for a non-finite reading.
    pub fn since_local_midnight(self) -> TimeResult<Duration> {
        Duration::from_secs_f64(self.day_fraction * self.solar_day_seconds)
    }
}

impl fmt::Display for LocalTime {
    /// Formats as `HH:MM:SS` on the local 24-hour face, truncated so that the
    /// last moment of a day never prints as `24:00:00`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}",
            self.hour(),
            self.minute(),
            self.second()
        )
    }
}

/// A clock for one body.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BodyClock {
    body: &'static Body,
    solar_day_days: f64,
}

impl BodyClock {
    /// A clock for a body, or `None` when the body has no solar day. The Sun
    /// is the only such row in the table: it has no heliocentric orbit, so
    /// there is no Sun in its sky to keep time by.
    #[must_use]
    pub fn new(body: &'static Body) -> Option<Self> {
        body.solar_day_days().map(|solar_day_days| Self {
            body,
            solar_day_days,
        })
    }

    /// A clock for the named body, or `None` when the name is unknown or the
    /// body has no solar day. The comparison is case-sensitive.
    #[must_use]
    pub fn for_name(name: &str) -> Option<Self> {
        Self::new(by_name(name)?)
    }

    /// The body this clock belongs to.
    #[must_use]
    pub const fn body(&self) -> &'static Body {
        self.body
    }

    /// The length of the solar day in SI seconds.
    #[must_use]
    pub fn solar_day_seconds(&self) -> f64 {
        self.solar_day_days * 86_400.0
    }

    /// Whether this clock's zero point is an international standard rather
    /// than a convention this crate declares.
    #[must_use]
    pub fn is_standardised(&self) -> bool {
        self.body.clock_epoch.basis == EpochBasis::Standard
    }

    /// The continuous local day count at an instant, at the prime meridian,
    /// including the fraction of the current day.
    #[must_use]
    pub fn prime_meridian_index(&self, instant: Instant<Tai>) -> f64 {
        (j2000_offset_days(instant) - self.body.clock_epoch.j2000_offset_days) / self.solar_day_days
            + self.body.clock_epoch.day_number as f64
    }

    /// Local mean solar time at the prime meridian.
    #[must_use]
    pub fn at_prime_meridian(&self, instant: Instant<Tai>) -> LocalTime {
        self.at_west_longitude(instant, 0.0)
    }

    /// Local mean solar time at a west longitude.
    ///
    /// The sign of the longitude term follows the body's rotation sense; see
    /// the module documentation.
    #[must_use]
    pub fn at_west_longitude(&self, instant: Instant<Tai>, degrees: f64) -> LocalTime {
        let shift = if self.body.is_retrograde_rotator() {
            degrees / 360.0
        } else {
            -degrees / 360.0
        };
        let (day_number, day_fraction) =
            whole_and_fraction(self.prime_meridian_index(instant) + shift);
        LocalTime {
            day_number,
            day_fraction,
            solar_day_seconds: self.solar_day_seconds(),
        }
    }

    /// Local mean solar time at an east longitude.
    #[must_use]
    pub fn at_east_longitude(&self, instant: Instant<Tai>, degrees: f64) -> LocalTime {
        self.at_west_longitude(instant, -degrees)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mars;
    use crate::util::tai_from_utc_fields;

    fn at(year: i64, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Instant<Tai> {
        tai_from_utc_fields(year, month, day, hour, minute, second).unwrap()
    }

    #[test]
    fn every_body_but_the_sun_has_a_clock() {
        for entry in crate::bodies::ALL {
            let clock = BodyClock::new(entry);
            assert_eq!(clock.is_some(), entry.name != "Sun", "{}", entry.name);
        }
        assert!(BodyClock::for_name("Sun").is_none());
        assert!(BodyClock::for_name("Arrakis").is_none());
        assert!(BodyClock::for_name("Titan").is_some());
    }

    #[test]
    fn only_earth_and_mars_report_a_standardised_zero_point() {
        for entry in crate::bodies::ALL {
            let Some(clock) = BodyClock::new(entry) else {
                continue;
            };
            assert_eq!(
                clock.is_standardised(),
                entry.name == "Earth" || entry.name == "Mars",
                "{}",
                entry.name
            );
        }
    }

    #[test]
    fn the_generic_mars_clock_reproduces_coordinated_mars_time() {
        // The strongest cross-check in the crate: the table-driven clock and
        // the Allison-McEwen implementation are wholly separate code paths
        // that must agree on Mars.
        let clock = BodyClock::for_name("Mars").unwrap();
        for when in [
            at(2000, 1, 6, 0, 0, 0),
            at(2012, 8, 6, 5, 17, 57),
            at(1976, 7, 20, 11, 53, 6),
            at(2035, 11, 2, 18, 41, 9),
        ] {
            let moment = mars::MarsMoment::from_tai(when);
            let local = clock.at_prime_meridian(when);
            assert_eq!(local.day_number(), moment.sol_number());
            assert_eq!(
                local.to_string(),
                moment.coordinated_mars_time().to_string()
            );
            assert!(
                (local.day_fraction() - moment.coordinated_mars_time().sol_fraction()).abs() < 1e-9
            );
        }
    }

    #[test]
    fn the_generic_mars_clock_reproduces_local_mean_solar_time() {
        let clock = BodyClock::for_name("Mars").unwrap();
        let when = at(2021, 2, 18, 20, 43, 48);
        let moment = mars::MarsMoment::from_tai(when);
        for west in [0.0, 5.5266, 133.8, 222.56, 359.9] {
            let generic = clock.at_west_longitude(when, west);
            let special = moment.local_mean_solar_time(west);
            assert!(
                (generic.day_fraction() - special.sol_fraction()).abs() < 1e-9,
                "{west}"
            );
        }
    }

    #[test]
    fn what_time_is_it_at_olympus_mons() {
        // The question the module exists to answer, asked through the generic
        // interface and checked against the Mars-specific one.
        let clock = BodyClock::for_name("Mars").unwrap();
        let olympus = mars::site("Olympus Mons").unwrap();
        let when = at(2030, 6, 1, 12, 0, 0);
        let generic = clock.at_east_longitude(when, olympus.east_longitude_degrees);
        let special = olympus.local_mean_solar_time(mars::MarsMoment::from_tai(when));
        assert_eq!(generic.to_string(), special.to_string());
        // A Martian hour is a little over 61 minutes.
        assert!((generic.local_hour_seconds() - 3_699.0).abs() < 1.0);
    }

    #[test]
    fn the_earth_clock_numbers_its_days_by_rata_die() {
        let clock = BodyClock::for_name("Earth").unwrap();
        // 2000-01-01 is Rata Die 730120; the epoch is its midnight in TT, and
        // TT ran 64.184 s ahead of UTC in 2000, so noon UTC is inside it.
        let local = clock.at_prime_meridian(at(2000, 1, 1, 12, 0, 0));
        assert_eq!(local.day_number(), 730_120);
        assert_eq!(local.hour(), 12);
        assert_eq!(local.local_hour_seconds(), 3_600.0);
        // 2024-02-29 is Rata Die 738945.
        let later = clock.at_prime_meridian(at(2024, 2, 29, 0, 0, 30));
        assert_eq!(later.day_number(), 738_945);
        assert_eq!(later.hour(), 0);
    }

    #[test]
    fn the_earth_clock_is_uniform_and_so_parts_company_with_ut1() {
        // Documented limitation, pinned so it cannot be forgotten: the clock
        // ticks 86400 s per day and the planet does not.
        let clock = BodyClock::for_name("Earth").unwrap();
        let local = clock.at_prime_meridian(at(2020, 1, 1, 0, 0, 0));
        // TT - UTC was 69.184 s in 2020, and the epoch is a TT midnight, so
        // the clock reads about 69 s past midnight at UTC midnight.
        let seconds = local.day_fraction() * 86_400.0;
        assert!((seconds - 69.184).abs() < 0.01, "{seconds}");
    }

    #[test]
    fn longitude_runs_the_right_way_on_a_prograde_body() {
        let clock = BodyClock::for_name("Earth").unwrap();
        let when = at(2024, 6, 21, 12, 0, 0);
        let greenwich = clock.at_prime_meridian(when).day_fraction();
        let tokyo = clock.at_east_longitude(when, 135.0).day_fraction();
        // 135 degrees east is nine hours ahead.
        assert!((modulo(tokyo - greenwich, 1.0) - 0.375).abs() < 1e-9);
    }

    #[test]
    fn longitude_runs_the_other_way_on_a_retrograde_body() {
        // On Venus the Sun rises in the west, so a place to the west of the
        // prime meridian reaches noon first and its clock reads later.
        let clock = BodyClock::for_name("Venus").unwrap();
        let when = at(2024, 6, 21, 12, 0, 0);
        let prime = clock.at_prime_meridian(when).day_fraction();
        let west = clock.at_west_longitude(when, 90.0).day_fraction();
        assert!((modulo(west - prime, 1.0) - 0.25).abs() < 1e-9);
        // And the opposite for a prograde body, to prove the branch matters.
        let mars = BodyClock::for_name("Mars").unwrap();
        let mars_prime = mars.at_prime_meridian(when).day_fraction();
        let mars_west = mars.at_west_longitude(when, 90.0).day_fraction();
        assert!((modulo(mars_prime - mars_west, 1.0) - 0.25).abs() < 1e-9);
    }

    #[test]
    fn a_local_day_takes_a_solar_day_of_si_time() {
        for name in ["Earth", "Mars", "Titan", "Venus", "Io", "Charon"] {
            let clock = BodyClock::for_name(name).unwrap();
            let start = at(2030, 1, 1, 0, 0, 0);
            let index = clock.prime_meridian_index(start);
            let day = clock.solar_day_seconds();
            let later = start
                .checked_add(Duration::from_secs_f64(day).unwrap())
                .unwrap();
            let moved = clock.prime_meridian_index(later) - index;
            assert!((moved - 1.0).abs() < 1e-9, "{name}: {moved}");
        }
    }

    #[test]
    fn a_local_hour_is_a_twenty_fourth_of_the_bodys_own_day() {
        let expectations: [(&str, f64); 5] = [
            ("Earth", 3_600.0),
            ("Mars", 3_699.0),
            ("Titan", 57_490.0),
            ("Moon", 106_310.0),
            ("Venus", 420_302.0),
        ];
        for (name, seconds) in expectations {
            let clock = BodyClock::for_name(name).unwrap();
            let hour = clock
                .at_prime_meridian(at(2020, 1, 1, 0, 0, 0))
                .local_hour_seconds();
            assert!(
                (hour - seconds) / seconds < 1e-3,
                "{name}: {hour} vs {seconds}"
            );
        }
    }

    #[test]
    fn the_clock_face_is_always_a_valid_twenty_four_hour_reading() {
        for entry in crate::bodies::ALL {
            let Some(clock) = BodyClock::new(entry) else {
                continue;
            };
            for hours in [0, 3, 7, 13, 19, 23] {
                let local = clock.at_west_longitude(at(2027, 4, 9, hours, 31, 7), 217.3);
                assert!(local.hour() < 24, "{}", entry.name);
                assert!(local.minute() < 60, "{}", entry.name);
                assert!(local.second() < 60, "{}", entry.name);
                assert!((0.0..1.0).contains(&local.day_fraction()), "{}", entry.name);
            }
        }
    }

    #[test]
    fn time_since_local_midnight_matches_the_day_fraction() {
        let clock = BodyClock::for_name("Titan").unwrap();
        let local = clock.at_prime_meridian(at(2025, 9, 1, 6, 0, 0));
        let elapsed = local.since_local_midnight().unwrap().as_secs_f64();
        assert!((elapsed - local.day_fraction() * local.solar_day_seconds()).abs() < 1e-6);
        assert!(elapsed >= 0.0 && elapsed < local.solar_day_seconds());
    }

    #[test]
    fn the_day_number_advances_by_exactly_one_per_local_day() {
        let clock = BodyClock::for_name("Io").unwrap();
        let start = at(2026, 1, 1, 0, 0, 0);
        let first = clock.at_prime_meridian(start).day_number();
        let day = Duration::from_secs_f64(clock.solar_day_seconds()).unwrap();
        let mut when = start;
        for step in 1..=5 {
            when = when.checked_add(day).unwrap();
            assert_eq!(clock.at_prime_meridian(when).day_number(), first + step);
        }
    }

    #[test]
    fn a_longitude_of_three_hundred_and_sixty_degrees_is_the_prime_meridian() {
        let clock = BodyClock::for_name("Ganymede").unwrap();
        let when = at(2031, 7, 7, 7, 7, 7);
        let prime = clock.at_prime_meridian(when);
        let wrapped = clock.at_west_longitude(when, 360.0);
        assert_eq!(prime.day_number(), wrapped.day_number() + 1);
        assert!((prime.day_fraction() - wrapped.day_fraction()).abs() < 1e-9);
    }
}
