//! Mars: the sol, the Mars Sol Date, MTC, local mean and true solar time, the
//! areocentric solar longitude, Mars years, the Darian calendar and its
//! Martiana variant.
//!
//! # Where the numbers come from
//!
//! Everything in this module is the Allison–McEwen formulation as published by
//! NASA GISS in *Mars24 Sunclock — Algorithm and Worked Examples*
//! (<https://www.giss.nasa.gov/tools/mars24/help/algorithm.html>, updated
//! 2025-01-07, retrieved 2026-09-26, `mars24-algorithm`) and the *Technical
//! Notes on Mars Solar Time*
//! (<https://www.giss.nasa.gov/tools/mars24/help/notes.html>, retrieved
//! 2026-09-26, `mars24-notes`), which restate, with later revisions by
//! Allison (the note's equations B-1, B-2, C-2 and D-2 were changed in
//! 2015),
//!
//! * Allison, M. (1997), "Accurate analytic representations of solar time and
//!   seasons on Mars with applications to the Pathfinder/Surveyor missions",
//!   *Geophysical Research Letters* **24**, 1967–1970 (`allison1997`, not
//!   read here); and
//! * Allison, M., and M. McEwen (2000), "A post-Pathfinder evaluation of
//!   areocentric solar coordinates with improved timing recipes for Mars
//!   seasonal/diurnal climate studies", *Planetary and Space Science* **48**,
//!   215–235, doi:10.1016/S0032-0633(99)00092-6 (`allison2000`, not read
//!   here).
//!
//! The whole system — MSD, MTC, local mean and true solar time, the Mars
//! year and the mission sol counts — is written up in
//! `docs/systems/mars-timekeeping.md`.
//!
//! Allison and McEwen state a maximum error in the areocentric solar longitude
//! of about **0.008°** over ±100 years of J2000, which is about **three
//! seconds** of true solar time. This module claims that and nothing more; it
//! is not an ephemeris, and outside roughly 1900–2100 it is an extrapolation
//! whose error grows because the model carries no secular change of Mars's
//! orbital elements.
//!
//! # The chain
//!
//! ```text
//! Instant<Tai> --+32.184 s--> TT --Δt from J2000--> MSD --fraction--> MTC
//!                                        |
//!                                        +--> M, ν−M --> Ls --> EOT --> LTST
//! ```
//!
//! Every quantity hangs off `Δt_J2000`, the count of TT days since
//! 2000-01-01T12:00:00 TT, so [`MarsMoment`] carries that one number and
//! derives the rest.

pub mod darian;
pub mod martiana;
pub mod missions;

use core::fmt;

use hc_core::math::{cos_deg, floor, sin_deg};
use hc_core::{Duration, Instant, Tai, TimeResult};

use crate::util::{fract, instant_from_j2000_offset, j2000_offset_days, modulo, signed_degrees};

pub use darian::{DarianCalendar, DarianDate};
pub use martiana::{MartianaCalendar, MartianaDate};
pub use missions::{Mission, MissionClock, SolConvention};

/// The mean Martian solar day, in SI seconds: 24 h 39 m 35.244 s.
///
/// Allison and McEwen (2000); quoted in the Mars24 technical notes as the
/// defining length of the sol.
pub const MARS_SOL_SECONDS: f64 = 88_775.244;

/// The mean Martian *sidereal* day, in SI seconds: 24 h 37 m 22.663 s.
///
/// Mars24 technical notes. The sol is longer because Mars moves along its
/// orbit while it turns, exactly as a terrestrial solar day exceeds the
/// sidereal day by about four minutes.
pub const MARS_SIDEREAL_DAY_SECONDS: f64 = 88_642.663;

/// The length of a sol in terrestrial days, as Mars24 equation C-2 states it.
///
/// Note the hairline inconsistency in the published constants, which this
/// crate reproduces rather than silently fixes: `88775.244 / 86400` is
/// 1.027 491 250 0 exactly, while Mars24 uses 1.027 491 251 7. The difference
/// is 1.5 × 10⁻⁷ s per sol — about five seconds over the whole span from the
/// MSD epoch to today — and using Mars24's value is what makes this module
/// agree with the published worked examples digit for digit.
pub const SOL_IN_DAYS: f64 = 1.027_491_251_7;

/// The Julian Date in TT at which the Mars Sol Date is [`MSD_AT_EPOCH`], less
/// [`MSD_MIDNIGHT_ADJUSTMENT`]: `2000-01-06T00:00:00 TT`.
pub const MSD_EPOCH_JULIAN_DATE_TT: f64 = 2_451_549.5;

/// The `Δt_J2000` of [`MSD_EPOCH_JULIAN_DATE_TT`], in TT days.
pub const MSD_EPOCH_J2000_OFFSET: f64 = MSD_EPOCH_JULIAN_DATE_TT - 2_451_545.0;

/// The Mars Sol Date at the MSD epoch, before the midnight adjustment.
pub const MSD_AT_EPOCH: f64 = 44_796.0;

/// The adjustment that aligns MSD sol boundaries with mean midnight at the
/// Martian prime meridian, as carried by Mars24 (equation C-2).
///
/// Allison and McEwen (2000) published `0.00072`, which makes MSD exactly
/// 44 796.0 at 2000-01-06T00:00 UTC; Mars24 carries the revised `0.0009626`.
/// The two differ by 0.000 242 6 sol, **21.5 Martian seconds** — the "21
/// Mars-seconds away from also being mean midnight" of the GISS worked
/// example. Using the revised value is this library's choice, made so that
/// [`mars_sol_date`] and MTC agree with Mars24 and its worked examples;
/// [`MSD_MIDNIGHT_ADJUSTMENT_2000`] is provided for callers who need to
/// reproduce the 2000 paper. The 2000 value is stated here as the Mars24
/// history gives it; the paper itself was not read.
pub const MSD_MIDNIGHT_ADJUSTMENT: f64 = 0.000_962_6;

/// The midnight adjustment as first published in Allison and McEwen (2000).
pub const MSD_MIDNIGHT_ADJUSTMENT_2000: f64 = 0.000_72;

/// Mars's tropical year in sols: the interval between successive `Ls = 0`
/// crossings. Mars24 technical notes.
pub const MARS_TROPICAL_YEAR_SOLS: f64 = 668.592_1;

/// Mars's tropical year in terrestrial days. Mars24 technical notes.
pub const MARS_TROPICAL_YEAR_DAYS: f64 = 686.972_5;

/// Mars's sidereal year in sols. Mars24 technical notes.
pub const MARS_SIDEREAL_YEAR_SOLS: f64 = 668.599_1;

/// Mars's sidereal year in terrestrial days. Mars24 technical notes.
pub const MARS_SIDEREAL_YEAR_DAYS: f64 = 686.979_7;

/// The mean rate at which the areocentric solar longitude advances, in degrees
/// per sol.
pub const MEAN_SOLAR_LONGITUDE_RATE: f64 = 360.0 / MARS_TROPICAL_YEAR_SOLS;

/// The Mars Sol Date of the `Ls = 0` crossing that begins Mars Year 1 under
/// the Clancy convention.
///
/// Clancy et al. (2000), *J. Geophys. Res.* **105**(E4), 9553–9571
/// (`clancy2000`, not read here), number
/// Mars years from the northern spring equinox of **1955 April 11**, chosen so
/// that the 1956 planet-encircling dust storm falls in Mars Year 1. The paper
/// gives the date but no time of day, and published times of day disagree: the
/// figure 08:31 UTC circulates widely but does not reproduce, while this
/// model, an independent fit to DE430 and an extrapolation of the tabulation
/// in Piqueux et al. (2015), *Icarus* **251**, 332–338, all cluster near
/// 11:00 UTC.
///
/// This constant is therefore a **seed**, not a citation: it is this module's
/// own solution of `Ls = 0` nearest 1955 April 11, and [`mars_year_start`]
/// re-solves from it so that the year boundaries stay self-consistent with
/// [`solar_longitude`].
pub const MARS_YEAR_1_START_MSD: f64 = 28_892.659_3;

/// Amplitude, period in terrestrial days, and phase in degrees of the seven
/// perturbation terms of Mars24 equation B-3.
///
/// They model the planetary perturbations that the five-term equation of the
/// centre does not carry. Their total amplitude is about 0.026°, which is
/// small but not negligible next to the model's 0.008° claim.
const PERTURBERS: [(f64, f64, f64); 7] = [
    (0.0071, 2.2353, 49.409),
    (0.0057, 2.7543, 168.173),
    (0.0039, 1.1177, 191.837),
    (0.0037, 15.7866, 21.736),
    (0.0021, 2.1354, 15.704),
    (0.0020, 2.4694, 95.528),
    (0.0018, 32.8493, 49.095),
];

/// A reading on a 24-hour Martian clock.
///
/// A Martian hour is 1/24 of a sol, a Martian minute 1/60 of that and a
/// Martian second 1/60 of *that*, so a Martian second is about 1.0275 SI
/// seconds. This is the convention every Mars mission has used: the clock has
/// the familiar face and the seconds are stretched, rather than the seconds
/// staying SI and the sol ending at 24:39:35.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MarsTime {
    sol_fraction: f64,
}

impl MarsTime {
    /// A clock reading from a fraction of a sol, wrapped into `[0, 1)`.
    #[must_use]
    pub fn from_sol_fraction(fraction: f64) -> Self {
        Self {
            sol_fraction: fract(fraction),
        }
    }

    /// The fraction of the sol elapsed since local midnight, in `[0, 1)`.
    #[must_use]
    pub fn sol_fraction(self) -> f64 {
        self.sol_fraction
    }

    /// The reading in decimal Martian hours, in `[0, 24)`.
    #[must_use]
    pub fn decimal_hours(self) -> f64 {
        self.sol_fraction * 24.0
    }

    /// The Martian hour, in `0..=23`.
    #[must_use]
    pub fn hour(self) -> u8 {
        floor(self.decimal_hours()) as u8
    }

    /// The Martian minute, in `0..=59`.
    #[must_use]
    pub fn minute(self) -> u8 {
        floor(fract(self.decimal_hours()) * 60.0) as u8
    }

    /// The Martian second, in `0..=59`.
    #[must_use]
    pub fn second(self) -> u8 {
        floor(fract(self.decimal_hours() * 60.0) * 60.0) as u8
    }

    /// SI seconds elapsed since local midnight.
    #[must_use]
    pub fn si_seconds_since_midnight(self) -> f64 {
        self.sol_fraction * MARS_SOL_SECONDS
    }

    /// The elapsed time since local midnight as an exact [`Duration`].
    ///
    /// # Errors
    ///
    /// Returns [`hc_core::TimeError::NotFinite`] if the reading is not finite.
    pub fn duration_since_midnight(self) -> TimeResult<Duration> {
        Duration::from_secs_f64(self.si_seconds_since_midnight())
    }
}

impl fmt::Display for MarsTime {
    /// Formats as `HH:MM:SS`, truncated rather than rounded so that the last
    /// moment of a sol never prints as `24:00:00`.
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

/// Which hemisphere's season a solar longitude names.
///
/// Mars science almost always speaks in `Ls` rather than in seasons, because
/// Mars's orbital eccentricity of 0.093 makes the four seasons markedly
/// unequal in length; the names are here for presentation, not for analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarsSeason {
    /// `Ls` in `[0, 90)`: northern spring, southern autumn.
    NorthernSpring,
    /// `Ls` in `[90, 180)`: northern summer, southern winter.
    NorthernSummer,
    /// `Ls` in `[180, 270)`: northern autumn, southern spring.
    NorthernAutumn,
    /// `Ls` in `[270, 360)`: northern winter, southern summer.
    NorthernWinter,
}

impl MarsSeason {
    /// The season containing an areocentric solar longitude.
    #[must_use]
    pub fn from_solar_longitude(degrees: f64) -> Self {
        match (modulo(degrees, 360.0) / 90.0) as u8 {
            0 => Self::NorthernSpring,
            1 => Self::NorthernSummer,
            2 => Self::NorthernAutumn,
            _ => Self::NorthernWinter,
        }
    }

    /// The English name of the season in the northern hemisphere.
    #[must_use]
    pub const fn northern_name(self) -> &'static str {
        match self {
            Self::NorthernSpring => "northern spring",
            Self::NorthernSummer => "northern summer",
            Self::NorthernAutumn => "northern autumn",
            Self::NorthernWinter => "northern winter",
        }
    }
}

/// A terrestrial instant seen as a position in Martian time.
///
/// The value it carries is `Δt_J2000`, TT days since J2000.0, because every
/// series in the Allison–McEwen recipe is a function of exactly that.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MarsMoment {
    j2000_offset_days: f64,
}

impl MarsMoment {
    /// The Martian moment of a TAI instant.
    #[must_use]
    pub fn from_tai(instant: Instant<Tai>) -> Self {
        Self::from_j2000_offset(j2000_offset_days(instant))
    }

    /// A Martian moment from TT days since J2000.0.
    #[must_use]
    pub const fn from_j2000_offset(days: f64) -> Self {
        Self {
            j2000_offset_days: days,
        }
    }

    /// A Martian moment from a Mars Sol Date.
    #[must_use]
    pub fn from_mars_sol_date(msd: f64) -> Self {
        Self::from_j2000_offset(
            (msd - MSD_AT_EPOCH + MSD_MIDNIGHT_ADJUSTMENT) * SOL_IN_DAYS + MSD_EPOCH_J2000_OFFSET,
        )
    }

    /// TT days since J2000.0.
    #[must_use]
    pub const fn j2000_offset(self) -> f64 {
        self.j2000_offset_days
    }

    /// The Julian Date in TT.
    #[must_use]
    pub fn julian_date_tt(self) -> f64 {
        self.j2000_offset_days + 2_451_545.0
    }

    /// The TAI instant this moment stands for.
    ///
    /// # Errors
    ///
    /// Returns [`hc_core::TimeError::NotFinite`] or
    /// [`hc_core::TimeError::Overflow`] for a moment outside the representable
    /// range.
    pub fn to_tai(self) -> TimeResult<Instant<Tai>> {
        instant_from_j2000_offset(self.j2000_offset_days)
    }

    /// The **Mars Sol Date**: a continuous count of sols whose zero falls on
    /// 1873 December 29, chosen by Allison so that the count has been positive
    /// throughout the era of telescopic Mars observation.
    ///
    /// Mars24 equation C-2.
    #[must_use]
    pub fn mars_sol_date(self) -> f64 {
        (self.j2000_offset_days - MSD_EPOCH_J2000_OFFSET) / SOL_IN_DAYS + MSD_AT_EPOCH
            - MSD_MIDNIGHT_ADJUSTMENT
    }

    /// The whole sol number of the Mars Sol Date.
    #[must_use]
    pub fn sol_number(self) -> i64 {
        floor(self.mars_sol_date()) as i64
    }

    /// **Coordinated Mars Time**: the mean solar time at Mars's prime
    /// meridian, which passes through the crater Airy-0.
    ///
    /// MTC is the Martian analogue of UT1 and is simply the fractional part of
    /// the Mars Sol Date on a 24-hour Martian clock. Unlike UT1 it is a purely
    /// computed quantity: there is no Martian equivalent of the IERS measuring
    /// the planet's rotation, so MTC is as uniform as the model that defines
    /// it.
    #[must_use]
    pub fn coordinated_mars_time(self) -> MarsTime {
        MarsTime::from_sol_fraction(self.mars_sol_date())
    }

    /// Mars's mean anomaly in degrees. Mars24 equation B-1.
    #[must_use]
    pub fn mean_anomaly(self) -> f64 {
        modulo(19.3871 + 0.524_020_73 * self.j2000_offset_days, 360.0)
    }

    /// The angle of the fictitious mean Sun in degrees. Mars24 equation B-2.
    ///
    /// This is the areocentric solar longitude a Mars on a circular orbit
    /// would have; the equation of the centre is the correction from it to the
    /// real one.
    #[must_use]
    pub fn angle_of_fictitious_mean_sun(self) -> f64 {
        modulo(270.3871 + 0.524_038_496 * self.j2000_offset_days, 360.0)
    }

    /// The sum of the seven perturbation terms, in degrees. Mars24 equation
    /// B-3.
    #[must_use]
    pub fn perturbations(self) -> f64 {
        let mut total = 0.0;
        for (amplitude, period, phase) in PERTURBERS {
            total += amplitude * cos_deg(0.985_626 * self.j2000_offset_days / period + phase);
        }
        total
    }

    /// The equation of the centre, `ν − M`, in degrees. Mars24 equation B-4.
    ///
    /// Mars's orbital eccentricity of 0.093 is five times the Earth's, so this
    /// reaches nearly ±11° where the terrestrial equivalent stays inside ±2°.
    /// That is the root cause of the outsized Martian equation of time.
    #[must_use]
    pub fn equation_of_centre(self) -> f64 {
        let m = self.mean_anomaly();
        (10.691 + 3.0e-7 * self.j2000_offset_days) * sin_deg(m)
            + 0.623 * sin_deg(2.0 * m)
            + 0.050 * sin_deg(3.0 * m)
            + 0.005 * sin_deg(4.0 * m)
            + 0.0005 * sin_deg(5.0 * m)
            + self.perturbations()
    }

    /// The **areocentric solar longitude** `Ls`, in degrees.
    ///
    /// `Ls` is the angle of Mars along its orbit measured from the northern
    /// spring equinox, and it is the coordinate Mars science actually uses to
    /// say when something happened: `Ls = 0` is northern spring, 90 northern
    /// summer, 180 northern autumn, 270 northern winter. Because the orbit is
    /// eccentric, equal steps in `Ls` are not equal steps in time.
    #[must_use]
    pub fn solar_longitude(self) -> f64 {
        modulo(
            self.angle_of_fictitious_mean_sun() + self.equation_of_centre(),
            360.0,
        )
    }

    /// The season this moment falls in.
    #[must_use]
    pub fn season(self) -> MarsSeason {
        MarsSeason::from_solar_longitude(self.solar_longitude())
    }

    /// The Martian **equation of time** in degrees: true solar time minus mean
    /// solar time. Mars24 equation C-1.
    ///
    /// The first three terms are the obliquity effect, the last is the
    /// eccentricity effect. Sweeping the series over a Mars year gives a range
    /// of **−51.1 min at `Ls ≈ 329°` to +39.9 min at `Ls ≈ 188°`** — strongly
    /// asymmetric, and three times the terrestrial swing of −14.2 to
    /// +16.3 min. A Martian sundial and a Martian clock disagree by nearly an
    /// hour and a half between the extremes, which is why this crate refuses
    /// to approximate the series away.
    #[must_use]
    pub fn equation_of_time_degrees(self) -> f64 {
        let ls = self.solar_longitude();
        2.861 * sin_deg(2.0 * ls) - 0.071 * sin_deg(4.0 * ls) + 0.002 * sin_deg(6.0 * ls)
            - self.equation_of_centre()
    }

    /// The equation of time in Martian minutes, which are also 1/1440 of a
    /// sol.
    #[must_use]
    pub fn equation_of_time_minutes(self) -> f64 {
        self.equation_of_time_degrees() * 1_440.0 / 360.0
    }

    /// **Local mean solar time** at a west longitude, in Martian hours on a
    /// 24-hour clock.
    ///
    /// West longitude is the historical areographic convention and the one the
    /// Mars24 recipe is written in; [`Self::local_mean_solar_time_east`] takes
    /// the modern east-positive coordinate instead.
    #[must_use]
    pub fn local_mean_solar_time(self, west_longitude_degrees: f64) -> MarsTime {
        MarsTime::from_sol_fraction(self.local_mean_sol_index(west_longitude_degrees))
    }

    /// Local mean solar time at an east longitude.
    #[must_use]
    pub fn local_mean_solar_time_east(self, east_longitude_degrees: f64) -> MarsTime {
        self.local_mean_solar_time(-east_longitude_degrees)
    }

    /// **Local true solar time** at a west longitude: what a Martian sundial
    /// reads.
    #[must_use]
    pub fn local_true_solar_time(self, west_longitude_degrees: f64) -> MarsTime {
        MarsTime::from_sol_fraction(self.local_true_sol_index(west_longitude_degrees))
    }

    /// Local true solar time at an east longitude.
    #[must_use]
    pub fn local_true_solar_time_east(self, east_longitude_degrees: f64) -> MarsTime {
        self.local_true_solar_time(-east_longitude_degrees)
    }

    /// The continuous local-mean-solar-day count at a west longitude: the Mars
    /// Sol Date shifted so that its integer part changes at local midnight
    /// rather than at midnight on the prime meridian.
    ///
    /// Mission sol counts are floors of this, which is why they are exposed
    /// rather than kept private.
    #[must_use]
    pub fn local_mean_sol_index(self, west_longitude_degrees: f64) -> f64 {
        self.mars_sol_date() - west_longitude_degrees / 360.0
    }

    /// The continuous local-*true*-solar-day count at a west longitude.
    #[must_use]
    pub fn local_true_sol_index(self, west_longitude_degrees: f64) -> f64 {
        self.local_mean_sol_index(west_longitude_degrees) + self.equation_of_time_degrees() / 360.0
    }

    /// The **Mars year** under the Clancy convention.
    ///
    /// See [`MARS_YEAR_1_START_MSD`] for the epoch and for why its time of day
    /// is a seed rather than a citation. Years before 1955 are negative, and
    /// there is no year zero problem: the count is a plain integer, so the year
    /// before 1 is 0.
    #[must_use]
    pub fn mars_year(self) -> i64 {
        mars_year_at(self.mars_sol_date())
    }
}

/// The Mars Sol Date of the `Ls = 0` crossing that begins `year`.
///
/// The seed is the mean-rate estimate from [`MARS_YEAR_1_START_MSD`], which is
/// never more than about a tenth of a sol out over the space age, and the
/// refinement is Newton's method with the mean rate of `Ls` as the derivative.
/// Because the true rate varies by a factor of 1.2/0.83 around the mean, each
/// step contracts the error by at least a factor of five, so thirty-two
/// iterations reach the floating-point limit with room to spare.
#[must_use]
pub fn mars_year_start(year: i64) -> f64 {
    let mut msd = MARS_YEAR_1_START_MSD + (year - 1) as f64 * MARS_TROPICAL_YEAR_SOLS;
    for _ in 0..32 {
        let error = signed_degrees(MarsMoment::from_mars_sol_date(msd).solar_longitude());
        let step = error / MEAN_SOLAR_LONGITUDE_RATE;
        // Clamping keeps a pathological seed from being thrown across a whole
        // year, which would make the search land in the wrong one.
        msd -= step.clamp(-100.0, 100.0);
    }
    msd
}

/// The Mars year containing a Mars Sol Date.
///
/// The linear estimate is corrected by walking to the neighbouring year
/// boundaries, so the answer is right even for an instant within seconds of a
/// crossing.
#[must_use]
pub fn mars_year_at(msd: f64) -> i64 {
    let estimate = (msd - MARS_YEAR_1_START_MSD) / MARS_TROPICAL_YEAR_SOLS;
    let mut year = 1 + floor(estimate) as i64;
    // Two guarded walks rather than one loop: the estimate is never more than
    // a fraction of a sol out, so at most one step is ever taken.
    for _ in 0..4 {
        if mars_year_start(year) > msd {
            year -= 1;
        } else {
            break;
        }
    }
    for _ in 0..4 {
        if mars_year_start(year + 1) <= msd {
            year += 1;
        } else {
            break;
        }
    }
    year
}

/// The Mars Sol Date of a TAI instant.
#[must_use]
pub fn mars_sol_date(instant: Instant<Tai>) -> f64 {
    MarsMoment::from_tai(instant).mars_sol_date()
}

/// Coordinated Mars Time at a TAI instant.
#[must_use]
pub fn coordinated_mars_time(instant: Instant<Tai>) -> MarsTime {
    MarsMoment::from_tai(instant).coordinated_mars_time()
}

/// The areocentric solar longitude at a TAI instant, in degrees.
#[must_use]
pub fn solar_longitude(instant: Instant<Tai>) -> f64 {
    MarsMoment::from_tai(instant).solar_longitude()
}

/// The Mars year of a TAI instant under the Clancy convention.
#[must_use]
pub fn mars_year(instant: Instant<Tai>) -> i64 {
    MarsMoment::from_tai(instant).mars_year()
}

/// A named place on Mars, with the longitude its clock runs on.
///
/// Longitudes are IAU east-positive; Mars is close enough to a sphere that
/// planetocentric and planetographic *longitude* agree (only latitude
/// differs), so no distinction is drawn here.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Site {
    /// The English name of the place.
    pub name: &'static str,
    /// East longitude in degrees.
    pub east_longitude_degrees: f64,
    /// Where the coordinate comes from.
    pub source: &'static str,
}

impl Site {
    /// West longitude in degrees, in `[0, 360)`.
    #[must_use]
    pub fn west_longitude_degrees(&self) -> f64 {
        modulo(-self.east_longitude_degrees, 360.0)
    }

    /// Local mean solar time here at a Martian moment.
    #[must_use]
    pub fn local_mean_solar_time(&self, moment: MarsMoment) -> MarsTime {
        moment.local_mean_solar_time(self.west_longitude_degrees())
    }

    /// Local true solar time here at a Martian moment.
    #[must_use]
    pub fn local_true_solar_time(&self, moment: MarsMoment) -> MarsTime {
        moment.local_true_solar_time(self.west_longitude_degrees())
    }
}

/// The Martian prime meridian, through the crater Airy-0 in Sinus Meridiani.
///
/// Kuchynka et al. (2014) fix it by defining the Viking Lander 1 site to be
/// exactly 47.951 37° east of it, which is why the Viking 1 longitude in
/// [`missions`] is so precise a number.
pub const AIRY_0: Site = Site {
    name: "Airy-0",
    east_longitude_degrees: 0.0,
    source: "IAU prime meridian; Kuchynka et al. (2014)",
};

/// A handful of Martian landmarks, so that "what time is it at Olympus Mons"
/// can be asked without looking up a coordinate.
pub const SITES: &[Site] = &[
    AIRY_0,
    Site {
        name: "Olympus Mons",
        east_longitude_degrees: 226.2,
        source: "IAU Gazetteer of Planetary Nomenclature (centre of the caldera)",
    },
    Site {
        name: "Valles Marineris",
        east_longitude_degrees: 301.4,
        source: "IAU Gazetteer of Planetary Nomenclature (centre)",
    },
    Site {
        name: "Hellas Planitia",
        east_longitude_degrees: 70.5,
        source: "IAU Gazetteer of Planetary Nomenclature (centre)",
    },
    Site {
        name: "Gale Crater",
        east_longitude_degrees: 137.44,
        source: "NASA GISS Mars24 lander table",
    },
    Site {
        name: "Jezero Crater",
        east_longitude_degrees: 77.45,
        source: "NASA GISS Mars24 lander table",
    },
];

/// Look a landmark up by name. The comparison is case-sensitive, because these
/// are proper nouns.
#[must_use]
pub fn site(name: &str) -> Option<&'static Site> {
    SITES.iter().find(|entry| entry.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::tai_from_utc_fields;

    fn at(year: i64, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> MarsMoment {
        MarsMoment::from_tai(tai_from_utc_fields(year, month, day, hour, minute, second).unwrap())
    }

    #[test]
    fn the_sol_and_the_ratio_agree_with_each_other_to_the_published_precision() {
        // 88775.244 / 86400 is 1.02749125 exactly; Mars24's ratio differs in
        // the ninth decimal, and the crate documents rather than hides it.
        let implied = MARS_SOL_SECONDS / 86_400.0;
        assert!((implied - SOL_IN_DAYS).abs() < 2e-9);
        assert!((implied - SOL_IN_DAYS).abs() > 1e-10);
    }

    #[test]
    fn the_sol_is_longer_than_the_martian_sidereal_day_by_about_two_minutes() {
        let difference = MARS_SOL_SECONDS - MARS_SIDEREAL_DAY_SECONDS;
        assert!((difference - 132.581).abs() < 1e-3, "{difference}");
    }

    /// NASA GISS, *Mars24 Algorithm and Worked Examples*, example 1:
    /// 2000-01-06T00:00:00 UTC gives Ls 277.187 58°, EOT −5.187 74° and Mars
    /// Coordinated Time 23:59:39.
    #[test]
    fn the_first_published_worked_example_reproduces() {
        let moment = at(2000, 1, 6, 0, 0, 0);
        assert!(
            (moment.julian_date_tt() - 2_451_549.500_74).abs() < 1e-5,
            "{}",
            moment.julian_date_tt()
        );
        assert!(
            (moment.solar_longitude() - 277.187_58).abs() < 1e-4,
            "{}",
            moment.solar_longitude()
        );
        assert!(
            (moment.equation_of_time_degrees() + 5.187_74).abs() < 1e-4,
            "{}",
            moment.equation_of_time_degrees()
        );
        assert_eq!(moment.coordinated_mars_time().to_string(), "23:59:39");
    }

    /// NASA GISS, *Mars24 Algorithm and Worked Examples*, example 2: at
    /// 2004-01-03T13:46:31 UTC the local *true* solar time at 184.702°W — the
    /// Spirit landing site — is 00:00:00, with Ls 327.324 16° and
    /// EOT −12.775 53°.
    #[test]
    fn the_second_published_worked_example_reproduces() {
        let moment = at(2004, 1, 3, 13, 46, 31);
        assert!(
            (moment.solar_longitude() - 327.324_16).abs() < 1e-4,
            "{}",
            moment.solar_longitude()
        );
        assert!(
            (moment.equation_of_time_degrees() + 12.775_53).abs() < 1e-4,
            "{}",
            moment.equation_of_time_degrees()
        );
        assert_eq!(
            moment.local_true_solar_time(184.702).to_string(),
            "00:00:00"
        );
    }

    #[test]
    fn the_mars_sol_date_is_44796_at_the_epoch_the_2000_paper_chose() {
        // Allison and McEwen (2000) picked the epoch so that MSD is exactly
        // 44796.0 at 2000-01-06T00:00 UTC under their 0.00072 adjustment; the
        // revised Mars24 adjustment leaves it 21.5 Martian seconds short.
        let moment = at(2000, 1, 6, 0, 0, 0);
        let mars24 = moment.mars_sol_date();
        let paper = mars24 + MSD_MIDNIGHT_ADJUSTMENT - MSD_MIDNIGHT_ADJUSTMENT_2000;
        assert!((paper - 44_796.0).abs() < 1e-5, "{paper}");
        assert!((mars24 - 44_796.0).abs() < 3e-4, "{mars24}");
        let gap_seconds = (paper - mars24) * MARS_SOL_SECONDS;
        assert!((gap_seconds - 21.5).abs() < 0.2, "{gap_seconds}");
    }

    #[test]
    fn the_mars_sol_date_at_the_j2000_epoch_is_not_the_round_number() {
        // A common misremembering: 44796.0 belongs to 2000 January 6, not to
        // the J2000 epoch, which falls four and a half days earlier and so
        // about 4.38 sols earlier.
        let moment = MarsMoment::from_j2000_offset(0.0);
        assert!((moment.mars_sol_date() - 44_791.619_4).abs() < 1e-3);
    }

    #[test]
    fn the_mars_sol_date_epoch_falls_at_the_end_of_1873() {
        // Allison chose the zero of the count to precede telescopic Mars
        // observation: MSD 0 is 1873-12-29, Julian Date 2405522.0028779.
        let moment = MarsMoment::from_mars_sol_date(0.0);
        assert!(
            (moment.julian_date_tt() - 2_405_522.002_877_9).abs() < 1e-4,
            "{}",
            moment.julian_date_tt()
        );
    }

    #[test]
    fn the_single_constant_form_of_the_msd_formula_agrees() {
        // Wikipedia and several implementations state MSD as
        // (JD_TT - 2405522.0028779) / 1.0274912517; it is algebraically the
        // same as the two-constant form.
        for offset in [-40_000.0, -1_000.0, 0.0, 4.5, 9_000.0] {
            let moment = MarsMoment::from_j2000_offset(offset);
            let other = (moment.julian_date_tt() - 2_405_522.002_877_9) / SOL_IN_DAYS;
            assert!(
                (moment.mars_sol_date() - other).abs() < 1e-5,
                "{offset}: {} vs {other}",
                moment.mars_sol_date()
            );
        }
    }

    #[test]
    fn the_mars_sol_date_round_trips_over_two_centuries() {
        let mut msd = 20_000.0;
        while msd < 80_000.0 {
            let moment = MarsMoment::from_mars_sol_date(msd);
            assert!((moment.mars_sol_date() - msd).abs() < 1e-8, "{msd}");
            msd += 0.37;
        }
    }

    #[test]
    fn a_sol_of_coordinated_mars_time_takes_a_sol_of_si_time() {
        let start = MarsMoment::from_mars_sol_date(50_000.0);
        let end = MarsMoment::from_mars_sol_date(50_001.0);
        let elapsed = (end.j2000_offset() - start.j2000_offset()) * 86_400.0;
        assert!((elapsed - MARS_SOL_SECONDS).abs() < 2e-4, "{elapsed}");
    }

    #[test]
    fn the_martian_clock_divides_the_sol_the_way_the_missions_do() {
        let time = MarsTime::from_sol_fraction(0.5);
        assert_eq!(time.hour(), 12);
        assert_eq!(time.minute(), 0);
        assert_eq!(time.second(), 0);
        assert!((time.si_seconds_since_midnight() - MARS_SOL_SECONDS / 2.0).abs() < 1e-9);
        // A Martian second is longer than an SI second by the sol ratio.
        let one_second = MarsTime::from_sol_fraction(1.0 / 86_400.0);
        assert!((one_second.si_seconds_since_midnight() - SOL_IN_DAYS).abs() < 1e-6);
    }

    #[test]
    fn the_clock_truncates_so_that_a_sol_never_ends_at_twenty_four_hundred() {
        let last = MarsTime::from_sol_fraction(1.0 - 1e-12);
        assert_eq!(last.to_string(), "23:59:59");
        assert_eq!(MarsTime::from_sol_fraction(-0.25).to_string(), "18:00:00");
    }

    #[test]
    fn local_mean_time_runs_ahead_to_the_east() {
        let moment = at(2020, 6, 1, 0, 0, 0);
        let prime = moment.coordinated_mars_time().sol_fraction();
        // 90 degrees east is six Martian hours ahead of the prime meridian.
        let east = moment.local_mean_solar_time_east(90.0).sol_fraction();
        assert!((fract(east - prime) - 0.25).abs() < 1e-9, "{east} {prime}");
        let west = moment.local_mean_solar_time(90.0).sol_fraction();
        assert!((fract(prime - west) - 0.25).abs() < 1e-9);
    }

    #[test]
    fn west_and_east_longitude_name_the_same_meridian() {
        let moment = at(2015, 3, 14, 9, 26, 53);
        let west = moment.local_mean_solar_time(222.56);
        let east = moment.local_mean_solar_time_east(137.44);
        assert!((west.sol_fraction() - east.sol_fraction()).abs() < 1e-12);
    }

    #[test]
    fn true_solar_time_differs_from_mean_by_the_equation_of_time() {
        let moment = at(2004, 1, 25, 5, 5, 0);
        let mean = moment.local_mean_solar_time(5.53).sol_fraction();
        let truth = moment.local_true_solar_time(5.53).sol_fraction();
        let expected = moment.equation_of_time_degrees() / 360.0;
        assert!((signed_degrees((truth - mean) * 360.0) / 360.0 - expected).abs() < 1e-9);
    }

    #[test]
    fn the_equation_of_time_swings_between_the_published_extremes() {
        // GISS technical notes: the Martian EOT runs from -51.1 to +39.9
        // minutes, against -14.2 to +16.3 on Earth.
        let mut lowest = f64::INFINITY;
        let mut highest = f64::NEG_INFINITY;
        let mut lowest_ls = 0.0;
        let mut highest_ls = 0.0;
        let mut msd = 50_000.0;
        while msd < 50_000.0 + MARS_TROPICAL_YEAR_SOLS {
            let moment = MarsMoment::from_mars_sol_date(msd);
            let minutes = moment.equation_of_time_minutes();
            if minutes < lowest {
                lowest = minutes;
                lowest_ls = moment.solar_longitude();
            }
            if minutes > highest {
                highest = minutes;
                highest_ls = moment.solar_longitude();
            }
            msd += 0.05;
        }
        assert!((lowest + 51.1).abs() < 0.2, "minimum {lowest}");
        assert!((highest - 39.9).abs() < 0.2, "maximum {highest}");
        assert!((lowest_ls - 329.0).abs() < 2.0, "minimum at Ls {lowest_ls}");
        assert!(
            (highest_ls - 188.0).abs() < 2.0,
            "maximum at Ls {highest_ls}"
        );
        // Far more than the terrestrial swing, which is the point.
        assert!(highest - lowest > 85.0);
    }

    #[test]
    fn solar_longitude_advances_through_a_full_turn_in_one_mars_year() {
        let start = MarsMoment::from_mars_sol_date(50_000.0).solar_longitude();
        let end =
            MarsMoment::from_mars_sol_date(50_000.0 + MARS_TROPICAL_YEAR_SOLS).solar_longitude();
        assert!(signed_degrees(end - start).abs() < 0.05, "{start} {end}");
    }

    #[test]
    fn solar_longitude_is_monotonic_across_a_mars_year() {
        let mut previous = MarsMoment::from_mars_sol_date(60_000.0).solar_longitude();
        let mut msd = 60_000.5;
        while msd < 60_000.0 + MARS_TROPICAL_YEAR_SOLS {
            let current = MarsMoment::from_mars_sol_date(msd).solar_longitude();
            let step = modulo(current - previous, 360.0);
            assert!(step > 0.0 && step < 1.0, "step {step} at {msd}");
            previous = current;
            msd += 0.5;
        }
    }

    #[test]
    fn the_seasons_are_unequal_because_the_orbit_is_eccentric() {
        // Mars is near perihelion around Ls 251, so southern summer is the
        // short season. Northern spring (Ls 0-90) is the longest.
        let boundaries: [f64; 5] = [0.0, 90.0, 180.0, 270.0, 360.0];
        let mut lengths = [0.0f64; 4];
        let base = mars_year_start(35);
        for index in 0..4 {
            let start = solve_solar_longitude(base, boundaries[index]);
            let end = solve_solar_longitude(base, boundaries[index + 1]);
            lengths[index] = end - start;
        }
        let total: f64 = lengths.iter().sum();
        assert!((total - MARS_TROPICAL_YEAR_SOLS).abs() < 0.5, "{total}");
        assert!(lengths[0] > 190.0, "northern spring {}", lengths[0]);
        assert!(lengths[3] < 160.0, "northern winter {}", lengths[3]);
    }

    /// Newton search for a target `Ls` at or after `from`, used only by tests.
    fn solve_solar_longitude(from: f64, target: f64) -> f64 {
        if target >= 360.0 {
            return from + MARS_TROPICAL_YEAR_SOLS;
        }
        let mut msd = from + target / MEAN_SOLAR_LONGITUDE_RATE;
        for _ in 0..40 {
            let error =
                signed_degrees(MarsMoment::from_mars_sol_date(msd).solar_longitude() - target);
            msd -= error / MEAN_SOLAR_LONGITUDE_RATE;
        }
        msd
    }

    #[test]
    fn mars_year_one_begins_at_the_equinox_of_april_1955() {
        let start = mars_year_start(1);
        let moment = MarsMoment::from_mars_sol_date(start);
        assert!(signed_degrees(moment.solar_longitude()).abs() < 1e-6);
        // 1955-04-11, as Clancy et al. (2000) state it.
        let day_start = at(1955, 4, 11, 0, 0, 0).mars_sol_date();
        let day_end = at(1955, 4, 12, 0, 0, 0).mars_sol_date();
        assert!(start > day_start && start < day_end, "{start}");
    }

    #[test]
    fn mars_years_are_a_tropical_year_apart() {
        for year in [-100, 1, 24, 37, 100] {
            let span = mars_year_start(year + 1) - mars_year_start(year);
            assert!(
                (span - MARS_TROPICAL_YEAR_SOLS).abs() < 0.05,
                "year {year} span {span}"
            );
        }
    }

    #[test]
    fn mars_year_starts_land_on_the_published_dates() {
        // The Mars-year table everybody uses; each entry is the Gregorian day
        // containing the Ls = 0 crossing.
        let expected: [(i64, (i64, u8, u8)); 8] = [
            (24, (1998, 7, 14)),
            (25, (2000, 5, 31)),
            (28, (2006, 1, 21)),
            (31, (2011, 9, 13)),
            (33, (2015, 6, 18)),
            (35, (2019, 3, 23)),
            (36, (2021, 2, 7)),
            (38, (2024, 11, 12)),
        ];
        for (year, (gy, gm, gd)) in expected {
            let start = mars_year_start(year);
            let day_start = at(gy, gm, gd, 0, 0, 0).mars_sol_date();
            let day_end = day_start + 86_400.0 / MARS_SOL_SECONDS;
            assert!(
                start >= day_start && start < day_end,
                "MY{year}: {start} not within {gy}-{gm}-{gd}"
            );
        }
    }

    #[test]
    fn the_mars_year_of_an_instant_matches_the_boundaries_that_bracket_it() {
        let mut msd = 30_000.0;
        while msd < 55_000.0 {
            let year = mars_year_at(msd);
            assert!(mars_year_start(year) <= msd, "{msd} in MY{year}");
            assert!(mars_year_start(year + 1) > msd, "{msd} in MY{year}");
            msd += 97.3;
        }
    }

    #[test]
    fn the_landings_fall_in_the_mars_years_the_literature_gives_them() {
        // Every one of these is the Mars year quoted in the mission
        // literature for the landing.
        let expected: [(&str, i64); 8] = [
            ("Viking 1", 12),
            ("Mars Pathfinder", 23),
            ("Spirit", 26),
            ("Opportunity", 26),
            ("Phoenix", 29),
            ("Curiosity", 31),
            ("InSight", 34),
            ("Perseverance", 36),
        ];
        for (name, year) in expected {
            let mission = missions::mission(name).unwrap();
            let moment = mission.landing_moment().unwrap();
            assert_eq!(moment.mars_year(), year, "{name}");
        }
    }

    #[test]
    fn seasons_are_named_from_the_solar_longitude() {
        assert_eq!(
            MarsSeason::from_solar_longitude(10.0),
            MarsSeason::NorthernSpring
        );
        assert_eq!(
            MarsSeason::from_solar_longitude(270.0),
            MarsSeason::NorthernWinter
        );
        assert_eq!(
            MarsSeason::from_solar_longitude(-10.0),
            MarsSeason::NorthernWinter
        );
        assert_eq!(
            MarsSeason::NorthernSummer.northern_name(),
            "northern summer"
        );
        // Curiosity landed at Ls 150.7, in northern summer.
        let moment = at(2012, 8, 6, 5, 17, 57);
        assert_eq!(moment.season(), MarsSeason::NorthernSummer);
    }

    #[test]
    fn the_free_functions_agree_with_the_moment_methods() {
        let instant = tai_from_utc_fields(2021, 2, 18, 20, 43, 48).unwrap();
        let moment = MarsMoment::from_tai(instant);
        assert!((mars_sol_date(instant) - moment.mars_sol_date()).abs() < 1e-12);
        assert_eq!(
            coordinated_mars_time(instant).to_string(),
            moment.coordinated_mars_time().to_string()
        );
        assert!((solar_longitude(instant) - moment.solar_longitude()).abs() < 1e-12);
        assert_eq!(mars_year(instant), moment.mars_year());
    }

    #[test]
    fn a_martian_moment_round_trips_through_a_tai_instant() {
        let instant = tai_from_utc_fields(2012, 8, 6, 5, 17, 57).unwrap();
        let moment = MarsMoment::from_tai(instant);
        let back = MarsMoment::from_tai(moment.to_tai().unwrap());
        assert!((back.j2000_offset() - moment.j2000_offset()).abs() < 1e-9);
    }

    #[test]
    fn the_landmark_table_is_self_consistent() {
        assert_eq!(site("Airy-0").unwrap().west_longitude_degrees(), 0.0);
        let olympus = site("Olympus Mons").unwrap();
        assert!((olympus.west_longitude_degrees() - 133.8).abs() < 1e-9);
        assert!(site("Tharsis").is_none());
        // Every landmark's true solar time differs from its mean by the
        // equation of time, whichever way it is reached.
        let moment = at(2030, 1, 1, 0, 0, 0);
        for entry in SITES {
            let mean = entry.local_mean_solar_time(moment).sol_fraction();
            let truth = entry.local_true_solar_time(moment).sol_fraction();
            let gap = signed_degrees((truth - mean) * 360.0);
            assert!(
                (gap - moment.equation_of_time_degrees()).abs() < 1e-9,
                "{}",
                entry.name
            );
        }
    }

    #[test]
    fn the_duration_since_midnight_is_exact_enough_to_be_useful() {
        let time = MarsTime::from_sol_fraction(0.25);
        let span = time.duration_since_midnight().unwrap();
        assert!((span.as_secs_f64() - MARS_SOL_SECONDS / 4.0).abs() < 1e-9);
    }
}
