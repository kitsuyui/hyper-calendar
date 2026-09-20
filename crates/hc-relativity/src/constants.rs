//! Physical constants, each with the authority it comes from.
//!
//! Two kinds of number live here and they should not be confused.
//!
//! * **Defined** values — the speed of light, standard gravity, the Julian
//!   year, the astronomical unit, the WGS 84 equatorial radius — are exact by
//!   convention. Every digit is real and they will not change.
//! * **Measured** values — `G`, and the standard gravitational parameters —
//!   carry an uncertainty. `G` is by far the worst of them, known to only
//!   about 2·10⁻⁵ relative; the `GM` products are known some six orders of
//!   magnitude better than `G` and the masses separately, which is exactly
//!   why celestial mechanics quotes `GM` and not `M`.
//!
//! Every dilation formula in this crate therefore takes a `GM`, never a mass.
//! [`Body::gm`] is the way to get one, and [`Body::source`] says where it
//! came from.

/// Speed of light in vacuum, in metres per second.
///
/// Exact: the 2019 SI redefinition fixes it, and the metre is derived from
/// it.
pub const SPEED_OF_LIGHT: f64 = 299_792_458.0;

/// `c²`, in m² s⁻².
pub const SPEED_OF_LIGHT_SQUARED: f64 = SPEED_OF_LIGHT * SPEED_OF_LIGHT;

/// Newtonian constant of gravitation, in m³ kg⁻¹ s⁻².
///
/// CODATA 2018 recommended value, `6.674 30(15)·10⁻¹¹`: a relative standard
/// uncertainty of 2.2·10⁻⁵, which makes it the least well known constant in
/// this module by five orders of magnitude. Prefer a `GM` wherever one
/// exists.
pub const GRAVITATIONAL_CONSTANT: f64 = 6.674_30e-11;

/// Standard acceleration of gravity, in m s⁻².
///
/// Exact by convention: the 3rd CGPM (1901) fixed it at 9.806 65 m s⁻². It is
/// the conventional "1 g" of a relativistic-rocket calculation, not the local
/// gravity anywhere in particular.
pub const STANDARD_GRAVITY: f64 = 9.806_65;

/// The Julian year, in seconds.
///
/// Exactly 365.25 × 86 400 by IAU convention. This is the year the
/// light-year is defined against and the one astronomical ages are quoted in.
pub const JULIAN_YEAR_SECONDS: f64 = 31_557_600.0;

/// The light-year, in metres.
///
/// Exact, being [`SPEED_OF_LIGHT`] × [`JULIAN_YEAR_SECONDS`] (IAU 2012).
pub const LIGHT_YEAR: f64 = 9_460_730_472_580_800.0;

/// The astronomical unit, in metres.
///
/// Exact by IAU 2012 Resolution B2, which redefined it as a conventional
/// length rather than a measured one.
pub const ASTRONOMICAL_UNIT: f64 = 1.495_978_707e11;

/// Standard gravitational parameter of the Sun, in m³ s⁻².
///
/// The IAU 2015 Resolution B3 nominal value, `1.327 124 400 18·10²⁰`, which
/// is the heliocentric gravitational constant of the JPL DE430/DE440
/// ephemerides expressed in SI units.
pub const GM_SUN: f64 = 1.327_124_400_18e20;

/// Standard gravitational parameter of the Earth, in m³ s⁻².
///
/// `3.986 004 418·10¹⁴`, the value of the IERS Conventions (2010) and of
/// WGS 84. This is the number the GPS control segment uses, which is why the
/// canonical satellite-clock figures come out right with it.
pub const GM_EARTH: f64 = 3.986_004_418e14;

/// Standard gravitational parameter of the Moon, in m³ s⁻².
///
/// `4.902 800 66·10¹²`, from the JPL DE430 lunar ephemeris.
pub const GM_MOON: f64 = 4.902_800_66e12;

/// Standard gravitational parameter of the Mars system, in m³ s⁻².
///
/// `4.282 837·10¹³`, the Mars *system* value (planet plus Phobos and Deimos)
/// of JPL DE440. The two moons contribute far below the last digit given.
pub const GM_MARS: f64 = 4.282_837e13;

/// Standard gravitational parameter of the Jupiter system, in m³ s⁻².
///
/// `1.267 127 64·10¹⁷`, the Jupiter *system* value of JPL DE440. Jupiter
/// alone is about 0.02 % smaller; the difference is the Galilean moons.
pub const GM_JUPITER: f64 = 1.267_127_64e17;

/// Mass of Sagittarius A\* in solar masses.
///
/// `(4.297 ± 0.013)·10⁶ M☉`, GRAVITY Collaboration, *A geometric distance
/// measurement to the Galactic centre black hole*, A&A 625, L10 (2019). The
/// uncertainty is 0.3 %, so [`GM_SAGITTARIUS_A_STAR`] is good to three
/// figures and no more.
pub const SAGITTARIUS_A_STAR_SOLAR_MASSES: f64 = 4.297e6;

/// Standard gravitational parameter of Sagittarius A\*, in m³ s⁻².
///
/// Derived as [`SAGITTARIUS_A_STAR_SOLAR_MASSES`] × [`GM_SUN`], which keeps
/// the product free of the 2·10⁻⁵ uncertainty in `G` — only the mass ratio's
/// own 0.3 % remains.
pub const GM_SAGITTARIUS_A_STAR: f64 = SAGITTARIUS_A_STAR_SOLAR_MASSES * GM_SUN;

/// Equatorial radius of the Earth, in metres.
///
/// `6 378 137` exactly: the defining semi-major axis of the WGS 84 ellipsoid.
/// Used here as the radius of a ground clock, which reproduces the published
/// GPS numbers to better than 0.1 µs per day.
pub const EARTH_EQUATORIAL_RADIUS: f64 = 6_378_137.0;

/// Nominal orbital radius of a GPS satellite, in metres.
///
/// `26 561 750`, the semi-major axis of the nominal GPS constellation: a
/// half-sidereal-day orbit, twelve hours of ground track repeat.
pub const GPS_ORBIT_RADIUS: f64 = 26_561_750.0;

/// A body whose standard gravitational parameter this crate carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Body {
    /// The Sun.
    Sun,
    /// The Earth.
    Earth,
    /// The Moon.
    Moon,
    /// The Mars system.
    Mars,
    /// The Jupiter system.
    Jupiter,
    /// Sagittarius A\*, the Galactic centre black hole.
    SagittariusAStar,
}

impl Body {
    /// Every body in the table.
    pub const ALL: [Self; 6] = [
        Self::Sun,
        Self::Earth,
        Self::Moon,
        Self::Mars,
        Self::Jupiter,
        Self::SagittariusAStar,
    ];

    /// The standard gravitational parameter `GM`, in m³ s⁻².
    #[must_use]
    pub const fn gm(self) -> f64 {
        match self {
            Self::Sun => GM_SUN,
            Self::Earth => GM_EARTH,
            Self::Moon => GM_MOON,
            Self::Mars => GM_MARS,
            Self::Jupiter => GM_JUPITER,
            Self::SagittariusAStar => GM_SAGITTARIUS_A_STAR,
        }
    }

    /// The English name.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::Sun => "Sun",
            Self::Earth => "Earth",
            Self::Moon => "Moon",
            Self::Mars => "Mars system",
            Self::Jupiter => "Jupiter system",
            Self::SagittariusAStar => "Sagittarius A*",
        }
    }

    /// Where the value came from, for a footnote or a provenance record.
    #[must_use]
    pub const fn source(self) -> &'static str {
        match self {
            Self::Sun => "IAU 2015 Resolution B3 nominal value (JPL DE430/DE440)",
            Self::Earth => "IERS Conventions (2010) and WGS 84",
            Self::Moon => "JPL DE430 lunar ephemeris",
            Self::Mars => "JPL DE440, Mars system",
            Self::Jupiter => "JPL DE440, Jupiter system",
            Self::SagittariusAStar => "GRAVITY Collaboration, A&A 625, L10 (2019)",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_light_year_is_the_speed_of_light_times_the_julian_year() {
        let derived = SPEED_OF_LIGHT * JULIAN_YEAR_SECONDS;
        assert!((derived - LIGHT_YEAR).abs() < 1.0, "got {derived}");
    }

    #[test]
    fn the_julian_year_is_exactly_three_hundred_and_sixty_five_and_a_quarter_days() {
        assert!((JULIAN_YEAR_SECONDS - 365.25 * 86_400.0).abs() < 1e-9);
    }

    #[test]
    fn c_squared_matches_the_published_value() {
        // 8.987 551 787 368 176 4 ·10^16 m^2/s^2.
        assert!((SPEED_OF_LIGHT_SQUARED - 8.987_551_787_368_176e16).abs() < 1e3);
    }

    #[test]
    fn the_solar_gm_matches_the_product_of_g_and_the_solar_mass() {
        // The Sun is about 1.989e30 kg; the product must agree to the 0.01 %
        // that the mass itself is known to, which is the whole reason GM is
        // tabulated instead of M.
        let from_mass = GRAVITATIONAL_CONSTANT * 1.988_41e30;
        let relative = (from_mass - GM_SUN).abs() / GM_SUN;
        assert!(relative < 1e-3, "relative difference {relative}");
    }

    #[test]
    fn the_earth_gm_matches_the_product_of_g_and_the_earth_mass() {
        let from_mass = GRAVITATIONAL_CONSTANT * 5.972_2e24;
        let relative = (from_mass - GM_EARTH).abs() / GM_EARTH;
        assert!(relative < 1e-3, "relative difference {relative}");
    }

    #[test]
    fn the_bodies_are_ordered_by_decreasing_mass_apart_from_the_black_hole() {
        assert!(Body::SagittariusAStar.gm() > Body::Sun.gm());
        assert!(Body::Sun.gm() > Body::Jupiter.gm());
        assert!(Body::Jupiter.gm() > Body::Earth.gm());
        assert!(Body::Earth.gm() > Body::Mars.gm());
        assert!(Body::Mars.gm() > Body::Moon.gm());
    }

    #[test]
    fn sagittarius_a_star_is_four_million_solar_masses() {
        let ratio = Body::SagittariusAStar.gm() / Body::Sun.gm();
        assert!((ratio - 4.297e6).abs() < 1.0, "got {ratio}");
    }

    #[test]
    fn every_body_carries_a_positive_parameter_and_a_source() {
        for body in Body::ALL {
            assert!(body.gm() > 0.0, "{} has no GM", body.english_name());
            assert!(!body.source().is_empty());
            assert!(!body.english_name().is_empty());
        }
    }

    #[test]
    fn the_gps_orbit_is_a_half_sidereal_day() {
        // A 12-hour-sidereal orbit: T = 2 pi sqrt(a^3 / GM) must be half of
        // the 86 164.1 s sidereal day, which is what the nominal semi-major
        // axis encodes.
        let period = 2.0
            * core::f64::consts::PI
            * hc_core::math::sqrt(
                GPS_ORBIT_RADIUS * GPS_ORBIT_RADIUS * GPS_ORBIT_RADIUS / GM_EARTH,
            );
        assert!((period - 86_164.1 / 2.0).abs() < 2.0, "period {period}");
    }
}
