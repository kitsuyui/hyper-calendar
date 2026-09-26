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
//! [`GravitatingBody::gm`] is the way to get one, and
//! [`GravitatingBody::source`] says where it
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
/// CODATA 2022 recommended value (`codata2022`), `6.674 30(15)·10⁻¹¹`,
/// unchanged from CODATA 2018: a relative standard uncertainty of 2.2·10⁻⁵,
/// which makes it the least well known constant in this module by five
/// orders of magnitude. Prefer a `GM` wherever one
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
/// Exact by IAU 2012 Resolution B2 (`iau-2012-b2`), which redefined it as a
/// conventional length of 149 597 870 700 m rather than a measured one.
pub const ASTRONOMICAL_UNIT: f64 = 1.495_978_707e11;

/// Standard gravitational parameter of the Sun, in m³ s⁻².
///
/// `1.327 124 400 412 794 2·10²⁰`, the value of the JPL DE440 ephemeris
/// (Park et al. 2021, `park2021`), read as `BODY10_GM` in NAIF's
/// `gm_de440.tpc` (`naif-gm-de440`), which gives it in km³ s⁻² in the
/// ephemeris's TDB-compatible units. It is the same ephemeris the Mars and
/// Jupiter values come from.
///
/// It is not the IAU's nominal value: that is
/// [`GM_SUN_NOMINAL_IAU_2015`], a conversion constant that agrees with it to
/// eight figures.
pub const GM_SUN: f64 = 1.327_124_400_412_794_2e20;

/// The nominal solar mass parameter `(𝒢ℳ)☉ᴺ`, in m³ s⁻².
///
/// Exactly `1.327 124 4·10²⁰` by IAU 2015 Resolution B3 (Prša et al. 2016,
/// `prsa2016`, Table 1): a defined conversion constant for quoting masses
/// in solar units, given to the precision within which its TCB and TDB
/// values agree, and not a measurement.
pub const GM_SUN_NOMINAL_IAU_2015: f64 = 1.327_124_4e20;

/// Standard gravitational parameter of the Earth, in m³ s⁻².
///
/// `3.986 004 418·10¹⁴`, the value of the IERS Conventions (2010) and of
/// WGS 84. It is the value Ashby's review of relativity in GPS uses for the
/// satellite-clock figures (Ashby 2003, `ashby2003`, the text after
/// Eq. 13), which is why those figures come out right with it.
pub const GM_EARTH: f64 = 3.986_004_418e14;

/// Standard gravitational parameter of the Moon, in m³ s⁻².
///
/// `4.902 800 118 457 55·10¹²`, the value of the JPL DE440 ephemeris
/// (`park2021`), read as `BODY301_GM` in NAIF's `gm_de440.tpc`
/// (`naif-gm-de440`).
pub const GM_MOON: f64 = 4.902_800_118_457_55e12;

/// Standard gravitational parameter of the Mars system, in m³ s⁻².
///
/// `4.282 837·10¹³`, the Mars *system* value (planet plus Phobos and Deimos)
/// of JPL DE440 (`park2021`; `BODY4_GM` in `naif-gm-de440`, 4.282 837 58·10⁴
/// km³ s⁻², given here to seven figures). The two moons contribute far below
/// the last digit given.
pub const GM_MARS: f64 = 4.282_837e13;

/// Standard gravitational parameter of the Jupiter system, in m³ s⁻².
///
/// `1.267 127 64·10¹⁷`, the Jupiter *system* value of JPL DE440 (`park2021`;
/// `BODY5_GM` in `naif-gm-de440`). Jupiter alone is about 0.02 % smaller;
/// the difference is the Galilean moons.
pub const GM_JUPITER: f64 = 1.267_127_64e17;

/// Mass of Sagittarius A\* in solar masses.
///
/// `(4.297 ± 0.012)·10⁶ M☉`, GRAVITY Collaboration, "Mass distribution in
/// the Galactic Center based on interferometric astrometry of multiple
/// stellar orbits", A&A 657, L12 (2022) (`gravity2022`), with the distance
/// fitted at 8 277 pc. The ±0.012 is the statistical error only; the paper
/// puts the systematic error, after GRAVITY Collaboration 2021, at about
/// 40 000 M☉, or ±0.04·10⁶. Taken together that is about 1 %, so
/// [`GM_SAGITTARIUS_A_STAR`] is good to two figures and no more.
pub const SAGITTARIUS_A_STAR_SOLAR_MASSES: f64 = 4.297e6;

/// Standard gravitational parameter of Sagittarius A\*, in m³ s⁻².
///
/// Derived as [`SAGITTARIUS_A_STAR_SOLAR_MASSES`] × [`GM_SUN`], which keeps
/// the product free of the 2·10⁻⁵ uncertainty in `G` — only the mass ratio's
/// own error, about 1 % with the systematic part, remains.
pub const GM_SAGITTARIUS_A_STAR: f64 = SAGITTARIUS_A_STAR_SOLAR_MASSES * GM_SUN;

/// Equatorial radius of the Earth, in metres.
///
/// `6 378 137` exactly: the defining semi-major axis of the WGS 84 ellipsoid.
/// Used here as the radius of a ground clock, which reproduces the published
/// GPS numbers to better than 0.1 µs per day.
pub const EARTH_EQUATORIAL_RADIUS: f64 = 6_378_137.0;

/// Nominal orbital radius of a GPS satellite, in metres.
///
/// `26 561 750`, the semi-major axis of a Keplerian orbit about
/// [`GM_EARTH`] whose period is half a sidereal day, which the test below
/// derives; Ashby (`ashby2003`) describes the GPS clocks as sitting at
/// about 4.2 Earth radii. Two orbits per sidereal day is what makes the
/// ground track repeat once each sidereal day.
pub const GPS_ORBIT_RADIUS: f64 = 26_561_750.0;

/// A body whose standard gravitational parameter this crate carries.
///
/// # Why this is a struct and not an enum
///
/// An enum's variant list would be a claim that these are the bodies there
/// are, and each new body would need a match arm in every accessor. A table
/// of values is the shape `hc_planetary::bodies::Body` has for the same
/// concept, and one shape for one idea is easier to read across crates.
///
/// It is named `GravitatingBody` rather than `Body` because it carries a
/// different fact from the planetary one — a mass parameter, not a rotation
/// — and two types called `Body` in one workspace helped nobody.
#[derive(Debug, Clone, Copy)]
pub struct GravitatingBody {
    /// A stable identifier, lowercase and hyphenated.
    pub id: &'static str,
    /// The English name.
    pub english_name: &'static str,
    /// The standard gravitational parameter `GM`, in m³ s⁻².
    pub gm: f64,
    /// Where the value came from, for a footnote or a provenance record.
    pub source: &'static str,
}

impl PartialEq for GravitatingBody {
    /// Two entries are the same body when they carry the same identifier.
    ///
    /// Compared by identifier because `gm` is an `f64`, which cannot derive
    /// `Eq` and does not compare usefully anyway — the identifier is what
    /// distinguishes one body from another.
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for GravitatingBody {}

impl core::hash::Hash for GravitatingBody {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

hc_core::catalogue! {
    type: GravitatingBody,
    id: |body| body.id,
    provenance: |body| body.source,
    tests: gravitating_body_catalogue,

    /// Every body this crate carries a gravitational parameter for.
    ///
    /// Not a claim about which bodies exist. A caller with a mass parameter
    /// this table has never heard of can write one down and use it, which is
    /// the whole reason this is a table.
    pub const GRAVITATING_BODIES;

    /// The body with this identifier.
    pub fn by_id;

    entries: {
        /// The Sun.
        pub const SUN = GravitatingBody {
            id: "sun",
            english_name: "Sun",
            gm: GM_SUN,
            source: "JPL DE440, Park et al., AJ 161:105 (2021), via NAIF gm_de440.tpc BODY10_GM",
        };

        /// The Earth.
        pub const EARTH = GravitatingBody {
            id: "earth",
            english_name: "Earth",
            gm: GM_EARTH,
            source: "IERS Conventions (2010) and WGS 84",
        };

        /// The Moon.
        pub const MOON = GravitatingBody {
            id: "moon",
            english_name: "Moon",
            gm: GM_MOON,
            source: "JPL DE440, Park et al., AJ 161:105 (2021), via NAIF gm_de440.tpc BODY301_GM",
        };

        /// The Mars system.
        pub const MARS = GravitatingBody {
            id: "mars",
            english_name: "Mars system",
            gm: GM_MARS,
            source: "JPL DE440, Mars system",
        };

        /// The Jupiter system.
        pub const JUPITER = GravitatingBody {
            id: "jupiter",
            english_name: "Jupiter system",
            gm: GM_JUPITER,
            source: "JPL DE440, Jupiter system",
        };

        /// Sagittarius A\*, the Galactic centre black hole.
        pub const SAGITTARIUS_A_STAR = GravitatingBody {
            id: "sagittarius-a-star",
            english_name: "Sagittarius A*",
            gm: GM_SAGITTARIUS_A_STAR,
            source: "GRAVITY Collaboration, A&A 657, L12 (2022)",
        };
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

    /// The mass parameter of a body, looked up the way a caller would.
    ///
    /// Going through the table rather than naming the constants keeps these
    /// assertions about the *data*: comparing two `const` values folds to a
    /// constant that proves nothing, which is what clippy objects to and
    /// clippy is right.
    fn gm(id: &str) -> f64 {
        by_id(id)
            .unwrap_or_else(|| panic!("{id} should be in the table"))
            .gm
    }

    #[test]
    fn the_bodies_are_ordered_by_decreasing_mass_apart_from_the_black_hole() {
        assert!(gm("sagittarius-a-star") > gm("sun"));
        assert!(gm("sun") > gm("jupiter"));
        assert!(gm("jupiter") > gm("earth"));
        assert!(gm("earth") > gm("mars"));
        assert!(gm("mars") > gm("moon"));
    }

    #[test]
    fn sagittarius_a_star_is_four_million_solar_masses() {
        let ratio = gm("sagittarius-a-star") / gm("sun");
        assert!((ratio - 4.297e6).abs() < 1.0, "got {ratio}");
    }

    #[test]
    fn every_body_carries_a_positive_parameter_and_a_source() {
        for body in GRAVITATING_BODIES {
            assert!(body.gm > 0.0, "{} has no GM", body.english_name);
            assert!(!body.source.is_empty());
            assert!(!body.english_name.is_empty());
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
