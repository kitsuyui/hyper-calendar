//! Time on bodies other than Earth.
//!
//! A calendar is a way of naming a day, and a day is one turn of a planet. The
//! rest of this workspace takes the Earth's turn for granted; this crate is
//! what happens when you stop doing that.
//!
//! * [`mars`] — the one non-terrestrial timekeeping system that is genuinely
//!   standardised and in daily operational use: the sol, the Mars Sol Date,
//!   Coordinated Mars Time, local mean and true solar time with the Martian
//!   equation of time, the areocentric solar longitude `Ls`, Mars years, the
//!   sol counts of every surface mission, the Darian calendar and its
//!   Martiana variant.
//! * [`titan`] and [`galilean`] — Gangale's calendars for the moons that
//!   have no livable day: Titan's Darian calendar and the Gregorian-based
//!   calendars of Io, Europa, Ganymede and Callisto, counted in *circads*,
//!   fractions of each moon's solar day, through the one engine in
//!   [`circad`].
//! * [`bodies`] — a data table of the major bodies of the solar system, and
//!   the solar day, the year in local days and the clock rate derived from it.
//! * [`moon`] — lunation numbers, the age of the Moon, and the selenographic
//!   colongitude. **Not** Coordinated Lunar Time, which does not yet exist.
//! * [`clock`] — one interface, [`clock::BodyClock`], that turns an
//!   [`Instant<Tai>`](hc_core::Instant) into a local time anywhere in the
//!   table.
//!
//! # Everything starts from TAI
//!
//! Every entry point in this crate takes an [`hc_core::Instant<Tai>`]. TAI is
//! the only scale with no rotational content at all, so it is the honest place
//! to stand when the rotation you care about is not the Earth's. The step to
//! TT is the fixed 32.184 s, and every series here is stated in TT days from
//! J2000.0.
//!
//! ```
//! use hc_planetary::mars;
//!
//! // Curiosity's landing, 2012-08-06T05:17:57Z.
//! let landing = mars::missions::mission("Curiosity").unwrap();
//! let moment = landing.landing_moment().unwrap();
//!
//! assert_eq!(landing.clock().unwrap().sol(moment), 0);
//! assert_eq!(moment.mars_year(), 31);
//! ```
//!
//! # What this crate is not
//!
//! It is not an ephemeris and carries no planetary position model beyond
//! Mars's own orbit. It does not do relativity: a clock on the Moon ticks at a
//! different rate from one on Earth, and that is `hc-relativity`'s subject,
//! not this one's. It does not invent standards — where a timekeeping system
//! has not been agreed, this crate says so and stops, which is the whole
//! content of [`moon`]'s position on Coordinated Lunar Time.
//!
//! # Accuracy
//!
//! Mars is good to about 0.008° of `Ls`, roughly three seconds of true solar
//! time, over ±100 years of J2000; that is Allison and McEwen's own figure for
//! the model, and this crate reproduces their published worked examples to the
//! last published digit. Everything derived from [`bodies`] is only as good as
//! a fact sheet quoted to four to seven significant figures, which is parts in
//! 10⁵ on a rotation period — fine for "how long is a day on Titan", useless
//! for propagating a sol count over a century. See the README for the
//! per-source detail.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod bodies;
pub mod circad;
pub mod clock;
pub mod galilean;
pub mod mars;
pub mod moon;
pub mod titan;

mod util;

pub use bodies::{Body, BodyKind, ClockEpoch, EpochBasis};
pub use circad::{CircadCalendar, CircadDate, CircadRule};
pub use clock::{BodyClock, LocalTime};
pub use mars::{
    DarianCalendar, DarianDate, MARS_SOL_SECONDS, MarsMoment, MarsSeason, MarsTime,
    MartianaCalendar, MartianaDate, Mission, MissionClock, SolConvention,
};

pub use hc_astro;
pub use hc_calendar;
pub use hc_core;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_crate_answers_the_question_it_exists_for() {
        // What time is it at Olympus Mons, and what is the date there?
        let when = util::tai_from_utc_fields(2035, 7, 4, 12, 0, 0).unwrap();
        let mars = BodyClock::for_name("Mars").unwrap();
        let olympus = mars::site("Olympus Mons").unwrap();
        let local = mars.at_east_longitude(when, olympus.east_longitude_degrees);
        assert!(local.hour() < 24);

        let moment = MarsMoment::from_tai(when);
        let date = DarianCalendar.date_at(moment).unwrap();
        assert_eq!(date.mars_year(), moment.mars_year());

        // And on Titan, through the same interface.
        let titan = BodyClock::for_name("Titan").unwrap();
        assert!(!titan.is_standardised());
        assert!(titan.solar_day_seconds() > 1.3e6);
    }

    #[test]
    fn the_re_exports_point_at_the_same_types_as_the_modules() {
        let by_alias: MarsTime = MarsTime::from_sol_fraction(0.5);
        let by_path: mars::MarsTime = mars::MarsTime::from_sol_fraction(0.5);
        assert_eq!(by_alias, by_path);
        assert_eq!(MARS_SOL_SECONDS, mars::MARS_SOL_SECONDS);
        assert_eq!(bodies::count(), 22);
    }
}
