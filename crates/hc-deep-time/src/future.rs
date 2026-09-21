//! The far future, as data.
//!
//! This is the half of deep time that makes a logarithmic type necessary. The
//! Sun leaves the main sequence in 5.4 billion years; a galaxy-mass black hole
//! evaporates in 10¹⁰⁰ years. Those two numbers differ by ninety-one decades,
//! and no linear representation can hold both usefully.
//!
//! # Two clocks, deliberately separate
//!
//! [`FutureEvent`] counts **years from the present day**, because that is how
//! stellar evolution and experimental limits are published. [`FutureEra`]
//! counts **cosmological decades** `η = log₁₀(t/yr)` measured from the Big
//! Bang, because that is Adams & Laughlin's own unit and re-basing it would
//! make their numbers unrecognisable. Above about 10¹¹ years the two agree to
//! better than a part in ten, since the universe's present age is a rounding
//! error at that scale, and [`FutureEvent::cosmological_decade`] does the
//! conversion where it matters.
//!
//! # Sources
//!
//! * The Sun: Schröder & Connon Smith, *Distant future of the Sun and Earth
//!   revisited*, MNRAS 386, 155 (2008), Table 1. Their model puts the Sun's
//!   present age at 4.58 Gyr, so every "from now" figure here is their model
//!   age minus 4.58 Gyr.
//! * The eras: Adams & Laughlin, *A dying universe: the long-term fate and
//!   evolution of astrophysical objects*, Rev. Mod. Phys. 69, 337 (1997),
//!   §VI. The gaps between their eras at `η = 14–15` and `η = 37–38` are
//!   theirs, not this crate's: the transitions are not sharp and they declined
//!   to pretend otherwise.
//! * Proton decay: Super-Kamiokande Collaboration, *Search for proton decay
//!   via p → e⁺π⁰ and p → μ⁺π⁰ with an enlarged fiducial volume*, Phys. Rev.
//!   D 102, 112011 (2020).
//! * Black hole evaporation: Hawking, Nature 248, 30 (1974) and Page, Phys.
//!   Rev. D 13, 198 (1976), through [`black_hole_lifetime`].
//!
//! # What a "prediction" means out here
//!
//! Every entry carries a [`Prediction`] saying what kind of claim it is. A
//! modelled stellar age and an experimental lower bound on the proton lifetime
//! are not the same kind of number, and printing them in the same column
//! without saying so would be misleading.

use core::f64::consts::PI;

use hc_uncertainty::Uncertain;

use crate::constants;
use crate::error::{DeepTimeError, DeepTimeResult};
use crate::magnitude::DeepTime;

/// What kind of claim a future entry is making.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Prediction {
    /// A number from a stellar-evolution or cosmological model, with the
    /// model's own published uncertainty.
    Modelled,
    /// An experimental limit. The true value lies on one side of it and the
    /// `±` is not a measurement error but the limit's own precision.
    ExperimentalBound,
    /// A cosmological decade: the order of magnitude, and nothing finer. The
    /// uncertainty is written as equal to the value, which is how this crate
    /// says "one decade".
    OrderOfMagnitude,
}

/// A dated event in the future of the universe.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FutureEvent {
    /// The event's name.
    pub name: &'static str,
    /// What happens, and what the number actually claims.
    pub description: &'static str,
    /// Where the number came from.
    pub source: &'static str,
    /// What kind of claim this is.
    pub prediction: Prediction,
    years_from_now: f64,
    std_dev_years: f64,
    figures: u8,
}

impl FutureEvent {
    /// How long from now, as a span.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::from_julian_years`]; cannot fail for the tabulated
    /// entries.
    pub fn from_now(&self) -> DeepTimeResult<DeepTime> {
        DeepTime::from_julian_years(self.years_from_now, self.std_dev_years)?
            .with_figures(self.figures)
    }

    /// The central value, in Julian years from now.
    #[must_use]
    pub const fn years_from_now(&self) -> f64 {
        self.years_from_now
    }

    /// The standard uncertainty, in Julian years.
    #[must_use]
    pub const fn std_dev_years(&self) -> f64 {
        self.std_dev_years
    }

    /// The cosmological decade `η = log₁₀(t/yr)` the event falls in, measured
    /// from the Big Bang so that it can be compared with [`ERAS`].
    ///
    /// # Errors
    ///
    /// See [`DeepTime::log10_seconds`].
    pub fn cosmological_decade(&self) -> DeepTimeResult<Uncertain> {
        let since_big_bang = crate::universe::AGE_OF_UNIVERSE
            .deep_time()?
            .checked_add(self.from_now()?)?;
        Ok(since_big_bang
            .log10_seconds()?
            .shifted(-log10_of_a_year())?)
    }
}

/// `log₁₀` of a Julian year in seconds, 7.499, used to turn a logarithm of
/// seconds into a logarithm of years.
fn log10_of_a_year() -> f64 {
    hc_core::math::log10(constants::JULIAN_YEAR_SECONDS)
}

/// One of Adams & Laughlin's eras, in cosmological decades from the Big Bang.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FutureEra {
    /// The era's name.
    pub name: &'static str,
    /// What characterises it.
    pub description: &'static str,
    /// Where the boundaries came from.
    pub source: &'static str,
    /// `log₁₀` of the era's opening time in years after the Big Bang.
    pub start_decade: f64,
    /// `log₁₀` of its closing time, or `None` for the Dark Era, which has no
    /// end anyone has proposed.
    pub end_decade: Option<f64>,
}

impl FutureEra {
    /// The era's opening time, as a span since the Big Bang.
    ///
    /// # Errors
    ///
    /// Returns [`DeepTimeError::NotFinite`] if the decade overflows `f64`,
    /// which happens above about `η = 300`.
    pub fn start(&self) -> DeepTimeResult<DeepTime> {
        years_from_decade(self.start_decade)
    }

    /// The era's closing time, or `None` where it has none.
    ///
    /// # Errors
    ///
    /// See [`FutureEra::start`].
    pub fn end(&self) -> DeepTimeResult<Option<DeepTime>> {
        match self.end_decade {
            Some(decade) => Ok(Some(years_from_decade(decade)?)),
            None => Ok(None),
        }
    }

    /// Whether the cosmological decade `decade` falls in `[start, end)`.
    ///
    /// Adams & Laughlin leave unassigned gaps between their eras, so a decade
    /// can legitimately belong to none of them, and the Black Hole Era and the
    /// Dark Era share the boundary at `η = 100`, which the half-open
    /// convention assigns to the later one.
    #[must_use]
    pub fn contains_decade(&self, decade: f64) -> bool {
        decade >= self.start_decade && self.end_decade.is_none_or(|end| decade < end)
    }
}

/// A span of `10^decade` years, as an order-of-magnitude magnitude.
///
/// The uncertainty is the value itself, which is the crate's convention for
/// "one decade, and no finer claim than that".
///
/// # Errors
///
/// Returns [`DeepTimeError::NotFinite`] when `10^decade` years overflows
/// `f64`, which happens above about `η = 300`.
pub fn years_from_decade(decade: f64) -> DeepTimeResult<DeepTime> {
    let years = hc_core::math::powf(10.0, decade);
    if !years.is_finite() {
        return Err(DeepTimeError::NotFinite);
    }
    DeepTime::from_julian_years(years, years)?.with_figures(1)
}

// --- Events -------------------------------------------------------------

/// The Sun exhausts core hydrogen, about 5.4 billion years from now.
pub const SUN_LEAVES_MAIN_SEQUENCE: FutureEvent = FutureEvent {
    name: "The Sun leaves the main sequence",
    description: "Core hydrogen runs out at a model age of 10.0 Gyr and the Sun begins burning \
                  hydrogen in a shell. By then its luminosity is 1.84 times today's, which has \
                  made the Earth uninhabitable long beforehand.",
    source: "Schroeder & Connon Smith, MNRAS 386, 155 (2008), Table 1, 'MS:final'",
    prediction: Prediction::Modelled,
    years_from_now: 5.42e9,
    std_dev_years: 1.0e8,
    figures: 2,
};

/// The tip of the red giant branch, 7.59 ± 0.05 Gyr from now.
pub const SUN_RED_GIANT_TIP: FutureEvent = FutureEvent {
    name: "Tip of the red giant branch",
    description: "The Sun reaches 2730 solar luminosities and a radius of 256 solar radii, \
                  having lost a third of its mass to a cool wind. The Earth is engulfed about \
                  half a million years before the tip, after tidal drag overcomes the orbital \
                  expansion the mass loss would otherwise have given it.",
    source: "Schroeder & Connon Smith, MNRAS 386, 155 (2008), 7.59 +/- 0.05 Gyr",
    prediction: Prediction::Modelled,
    years_from_now: 7.59e9,
    std_dev_years: 5.0e7,
    figures: 3,
};

/// The Sun becomes a white dwarf, about 7.72 Gyr from now.
pub const SUN_BECOMES_WHITE_DWARF: FutureEvent = FutureEvent {
    name: "The Sun becomes a white dwarf",
    description: "After the helium flash, the horizontal branch and a brief asymptotic giant \
                  phase, the Sun ejects its envelope and leaves a 0.54 solar mass carbon-oxygen \
                  white dwarf, which then cools for the rest of the Stelliferous Era.",
    source: "Schroeder & Connon Smith, MNRAS 386, 155 (2008), Table 1, 'AGB:tip'",
    prediction: Prediction::Modelled,
    years_from_now: 7.72e9,
    std_dev_years: 5.0e7,
    figures: 3,
};

/// Star formation in galaxies ends, around 10¹⁴ years from now.
pub const END_OF_STAR_FORMATION: FutureEvent = FutureEvent {
    name: "End of star formation",
    description: "Galaxies exhaust the gas they can turn into stars. The last and longest-lived \
                  stars are 0.1 solar mass red dwarfs, which burn for of order 10^13 years, so \
                  by the cosmological decade 14 the universe contains no hydrogen-burning stars \
                  at all.",
    source: "Adams & Laughlin, Rev. Mod. Phys. 69, 337 (1997), eta ~ 14",
    prediction: Prediction::OrderOfMagnitude,
    years_from_now: 1e14,
    std_dev_years: 1e14,
    figures: 1,
};

/// The experimental lower bound on the proton lifetime.
pub const PROTON_DECAY_LOWER_BOUND: FutureEvent = FutureEvent {
    name: "Proton decay lower bound",
    description: "Super-Kamiokande saw no candidate for p -> e+ pi0 in 450 kiloton-years of \
                  exposure, giving a partial-lifetime limit of 2.4e34 years at 90 % confidence. \
                  This is a bound, not a prediction: the proton may be stable. Grand unified \
                  models that predicted 10^31 years are already excluded, and the models still \
                  standing put it nearer 10^36.",
    source: "Super-Kamiokande Collaboration, Phys. Rev. D 102, 112011 (2020)",
    prediction: Prediction::ExperimentalBound,
    years_from_now: 2.4e34,
    std_dev_years: 0.0,
    figures: 2,
};

/// Hawking evaporation of a one-solar-mass black hole.
pub const SOLAR_MASS_BLACK_HOLE_EVAPORATES: FutureEvent = FutureEvent {
    name: "Evaporation of a solar-mass black hole",
    description: "A one-solar-mass Schwarzschild hole has a Hawking temperature of 61 nK, far \
                  below the present cosmic microwave background, so it absorbs more than it \
                  radiates and will not start shrinking until the background has cooled past it \
                  in some 10^19 years. The lifetime quoted here is the pure evaporation time \
                  once it does.",
    source: "Hawking (1974) and Page, Phys. Rev. D 13, 198 (1976); see black_hole_lifetime",
    prediction: Prediction::Modelled,
    years_from_now: 2.095e67,
    std_dev_years: 4.7e62,
    figures: 4,
};

/// Hawking evaporation of a million-solar-mass black hole.
pub const SUPERMASSIVE_BLACK_HOLE_EVAPORATES: FutureEvent = FutureEvent {
    name: "Evaporation of a supermassive black hole",
    description: "Evaporation time goes as the cube of the mass, so the 4.3 million solar mass \
                  hole at the centre of the Milky Way outlasts a stellar one by eighteen \
                  decades. A million solar masses is the round figure Adams & Laughlin scale to.",
    source: "Hawking/Page evaporation time at 1e6 solar masses; see black_hole_lifetime",
    prediction: Prediction::Modelled,
    years_from_now: 2.095e85,
    std_dev_years: 4.7e80,
    figures: 4,
};

/// Hawking evaporation of a galaxy-mass black hole.
pub const GALAXY_MASS_BLACK_HOLE_EVAPORATES: FutureEvent = FutureEvent {
    name: "Evaporation of a galaxy-mass black hole",
    description: "At 10^11 solar masses — the mass of a large galaxy, the largest hole the \
                  Degenerate Era can plausibly build — evaporation takes 10^100 years. This is \
                  the number that fixes the end of the Black Hole Era, and it is why the \
                  logarithmic scale in this crate has to reach a hundred and eight decades of \
                  seconds.",
    source: "Hawking/Page evaporation time at 1e11 solar masses; see black_hole_lifetime",
    prediction: Prediction::Modelled,
    years_from_now: 2.095e100,
    std_dev_years: 4.7e95,
    figures: 4,
};

/// The dated events of the far future, soonest first.
pub const EVENTS: &[FutureEvent] = &[
    SUN_LEAVES_MAIN_SEQUENCE,
    SUN_RED_GIANT_TIP,
    SUN_BECOMES_WHITE_DWARF,
    END_OF_STAR_FORMATION,
    PROTON_DECAY_LOWER_BOUND,
    SOLAR_MASS_BLACK_HOLE_EVAPORATES,
    SUPERMASSIVE_BLACK_HOLE_EVAPORATES,
    GALAXY_MASS_BLACK_HOLE_EVAPORATES,
];

// --- Eras ---------------------------------------------------------------

/// Adams & Laughlin's eras, in cosmological decades from the Big Bang.
///
/// The gaps at `η = 14–15` and `η = 37–38` are in the source: the transitions
/// are gradual, and the paper assigns no era to the decades in between rather
/// than drawing a line it cannot defend.
pub const ERAS: &[FutureEra] = &[
    FutureEra {
        name: "Stelliferous Era",
        description: "Most of the energy generated in the universe comes from nuclear fusion in \
                      stars. We are near its beginning on a logarithmic scale and very near its \
                      beginning on a linear one.",
        source: "Adams & Laughlin, Rev. Mod. Phys. 69, 337 (1997), 6 < eta < 14",
        start_decade: 6.0,
        end_decade: Some(14.0),
    },
    FutureEra {
        name: "Degenerate Era",
        description: "Fusion has stopped. Most baryonic mass is locked in white dwarfs, brown \
                      dwarfs and neutron stars, and the little energy released comes from dark \
                      matter annihilation in stellar remnants and, at the end, from proton decay \
                      — if the proton decays at all.",
        source: "Adams & Laughlin, Rev. Mod. Phys. 69, 337 (1997), 15 < eta < 37",
        start_decade: 15.0,
        end_decade: Some(37.0),
    },
    FutureEra {
        name: "Black Hole Era",
        description: "Protons have decayed and black holes are the only macroscopic objects \
                      left. They evaporate by Hawking radiation in order of increasing mass, the \
                      largest lasting until the cosmological decade 100.",
        source: "Adams & Laughlin, Rev. Mod. Phys. 69, 337 (1997), 38 < eta < 100",
        start_decade: 38.0,
        end_decade: Some(100.0),
    },
    FutureEra {
        name: "Dark Era",
        description: "Protons have decayed and black holes have evaporated. What remains is a \
                      thinning gas of photons, neutrinos, electrons and positrons, expanding and \
                      cooling with no further structure to form. No one has proposed an end to \
                      it.",
        source: "Adams & Laughlin, Rev. Mod. Phys. 69, 337 (1997), eta > 100",
        start_decade: 100.0,
        end_decade: None,
    },
];

/// The nominal solar mass parameter `GM☉`, exact by IAU 2015 Resolution B3.
///
/// The IAU defines `GM☉` rather than `M☉` because the product is what orbits
/// measure and it is known to nine digits, while the mass itself inherits the
/// 2.2×10⁻⁵ uncertainty of `G`. Deriving the evaporation time from `GM☉`
/// therefore makes that dependence explicit instead of hiding it in a
/// hard-coded kilogram figure.
pub const SOLAR_MASS_PARAMETER: f64 = 1.327_124_4e20;

/// The Hawking evaporation time of a Schwarzschild black hole.
///
/// `τ = 5120 π G² M³ / (ħ c⁴)`, which for `M = GM☉ / G` reduces to
/// `τ = 5120 π (GM☉)³ / (G ħ c⁴)` — so the only uncertain constant left is
/// `G`, and the answer carries its 2.2×10⁻⁵ relative uncertainty and nothing
/// else. One solar mass gives 2.095×10⁶⁷ years.
///
/// # What the formula assumes
///
/// A non-rotating, uncharged hole radiating massless photons and gravitons
/// only (Page 1976). It ignores accretion and, crucially, it ignores
/// absorption of the cosmic microwave background: a hole colder than its
/// surroundings *grows*, and every hole above about 10²² kg is colder than
/// today's CMB. The number is therefore the time an isolated hole takes once
/// the universe is colder than it, not the time from now. Adams & Laughlin
/// quote a coefficient about two decades smaller because they count more
/// radiated degrees of freedom at the high temperatures reached near the end.
///
/// # Errors
///
/// Returns [`DeepTimeError::OutOfDomain`] for a non-positive mass and
/// [`DeepTimeError::NotFinite`] when the cube of the mass overflows `f64`,
/// which happens above about 10⁸⁵ solar masses.
pub fn black_hole_lifetime(solar_masses: f64) -> DeepTimeResult<DeepTime> {
    if !solar_masses.is_finite() || solar_masses <= 0.0 {
        return Err(DeepTimeError::OutOfDomain);
    }
    let mass_parameter = Uncertain::exact(SOLAR_MASS_PARAMETER * solar_masses)?;
    let gravitation = constants::NEWTONIAN_CONSTANT_OF_GRAVITATION.uncertain()?;
    let hbar = constants::REDUCED_PLANCK_CONSTANT.uncertain()?;
    let light = constants::SPEED_OF_LIGHT.uncertain()?;
    let numerator = mass_parameter.powf(3.0)?.scaled(5120.0 * PI)?;
    let denominator = gravitation
        .checked_mul(hbar)?
        .checked_mul(light.powf(4.0)?)?;
    let seconds = numerator.checked_div(denominator)?;
    if !seconds.value.is_finite() {
        return Err(DeepTimeError::NotFinite);
    }
    DeepTime::from_uncertain_seconds(seconds)?.with_figures(4)
}

/// Which era the cosmological decade `decade` belongs to, if any.
#[must_use]
pub fn era_at_decade(decade: f64) -> Option<&'static FutureEra> {
    ERAS.iter().find(|era| era.contains_decade(decade))
}

/// The next dated event at or after `years` from now.
#[must_use]
pub fn event_after(years: f64) -> Option<&'static FutureEvent> {
    EVENTS.iter().find(|event| event.years_from_now >= years)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::magnitude::DeepUnit;
    use crate::universe;

    fn close(a: f64, b: f64, relative: f64) -> bool {
        hc_core::math::abs(a - b) <= hc_core::math::abs(b) * relative
    }

    #[test]
    fn the_events_are_in_strictly_increasing_order() {
        for pair in EVENTS.windows(2) {
            assert!(
                pair[1].years_from_now > pair[0].years_from_now,
                "{} is not after {}",
                pair[1].name,
                pair[0].name
            );
        }
    }

    #[test]
    fn no_future_entry_carries_a_negative_uncertainty() {
        for event in EVENTS {
            assert!(event.std_dev_years >= 0.0, "{}", event.name);
            assert!(event.years_from_now > 0.0, "{}", event.name);
        }
    }

    #[test]
    fn every_future_entry_names_a_source_and_describes_itself() {
        for event in EVENTS {
            assert!(!event.source.is_empty(), "{} has no source", event.name);
            assert!(
                event.description.len() > 40,
                "{} is undescribed",
                event.name
            );
        }
        for era in ERAS {
            assert!(!era.source.is_empty(), "{} has no source", era.name);
            assert!(era.description.len() > 40, "{} is undescribed", era.name);
        }
    }

    #[test]
    fn the_eras_are_ordered_and_never_overlap() {
        for pair in ERAS.windows(2) {
            let end = pair[0].end_decade.unwrap();
            assert!(
                pair[1].start_decade >= end,
                "{} overlaps {}",
                pair[1].name,
                pair[0].name
            );
        }
        assert!(ERAS.last().unwrap().end_decade.is_none());
    }

    #[test]
    fn the_adams_and_laughlin_gaps_are_preserved() {
        // eta 14 to 15 and 37 to 38 belong to no era in the source paper.
        assert!(era_at_decade(14.5).is_none());
        assert!(era_at_decade(37.5).is_none());
        assert_eq!(
            era_at_decade(10.0).map(|e| e.name),
            Some("Stelliferous Era")
        );
        assert_eq!(era_at_decade(20.0).map(|e| e.name), Some("Degenerate Era"));
        assert_eq!(era_at_decade(50.0).map(|e| e.name), Some("Black Hole Era"));
        assert_eq!(era_at_decade(500.0).map(|e| e.name), Some("Dark Era"));
    }

    #[test]
    fn the_present_falls_in_the_stelliferous_era() {
        let now = universe::AGE_OF_UNIVERSE.deep_time().unwrap();
        let decade = now.log10_seconds().unwrap().value - log10_of_a_year();
        assert!(close(decade, 10.14, 0.01), "eta = {decade}");
        assert_eq!(
            era_at_decade(decade).map(|e| e.name),
            Some("Stelliferous Era")
        );
    }

    #[test]
    fn the_sun_leaves_the_main_sequence_in_about_five_billion_years() {
        let event = SUN_LEAVES_MAIN_SEQUENCE.from_now().unwrap();
        let gigayears = event.in_unit(DeepUnit::Gigayear).unwrap();
        assert!(close(gigayears.value, 5.42, 1e-9), "{}", gigayears.value);
        // Schroeder & Smith's model age 10.00 Gyr minus their present 4.58.
        assert!(close(
            SUN_LEAVES_MAIN_SEQUENCE.years_from_now + 4.58e9,
            10.0e9,
            1e-9
        ));
    }

    #[test]
    fn the_sun_becomes_a_red_giant_before_it_becomes_a_white_dwarf() {
        // Held in a slice so that the comparison is a run-time one; the
        // point of the test is the table's content, not constant folding.
        let solar = [
            SUN_LEAVES_MAIN_SEQUENCE,
            SUN_RED_GIANT_TIP,
            SUN_BECOMES_WHITE_DWARF,
        ];
        for pair in solar.windows(2) {
            assert!(
                pair[1].years_from_now > pair[0].years_from_now,
                "{} is not after {}",
                pair[1].name,
                pair[0].name
            );
        }
    }

    #[test]
    fn a_solar_mass_black_hole_evaporates_in_two_times_ten_to_the_sixty_seven_years() {
        let lifetime = black_hole_lifetime(1.0).unwrap();
        let years = lifetime.in_unit(DeepUnit::JulianYear).unwrap();
        assert!(close(years.value, 2.095e67, 1e-3), "{}", years.value);
    }

    #[test]
    fn evaporation_time_goes_as_the_cube_of_the_mass() {
        let one = black_hole_lifetime(1.0).unwrap();
        let thousand = black_hole_lifetime(1000.0).unwrap();
        let decades = thousand.orders_of_magnitude_between(one).unwrap();
        assert!(close(decades, 9.0, 1e-9), "{decades} decades");
    }

    #[test]
    fn the_tabulated_evaporation_times_match_the_formula() {
        for (event, masses) in [
            (SOLAR_MASS_BLACK_HOLE_EVAPORATES, 1.0),
            (SUPERMASSIVE_BLACK_HOLE_EVAPORATES, 1e6),
            (GALAXY_MASS_BLACK_HOLE_EVAPORATES, 1e11),
        ] {
            let computed = black_hole_lifetime(masses).unwrap();
            let years = computed.in_unit(DeepUnit::JulianYear).unwrap();
            assert!(
                close(years.value, event.years_from_now, 1e-3),
                "{}: table {} vs formula {}",
                event.name,
                event.years_from_now,
                years.value
            );
            assert!(
                close(years.std_dev, event.std_dev_years, 0.05),
                "{}: sigma {} vs {}",
                event.name,
                event.std_dev_years,
                years.std_dev
            );
        }
    }

    #[test]
    fn the_evaporation_time_inherits_only_the_uncertainty_of_gravitation() {
        let lifetime = black_hole_lifetime(1.0).unwrap();
        let relative = lifetime.seconds().relative().unwrap();
        let gravitation = constants::NEWTONIAN_CONSTANT_OF_GRAVITATION
            .relative_uncertainty()
            .unwrap();
        assert!(close(relative, gravitation, 1e-6), "{relative}");
    }

    #[test]
    fn a_massless_or_negative_black_hole_is_refused() {
        assert_eq!(black_hole_lifetime(0.0), Err(DeepTimeError::OutOfDomain));
        assert_eq!(black_hole_lifetime(-1.0), Err(DeepTimeError::OutOfDomain));
        assert_eq!(
            black_hole_lifetime(f64::NAN),
            Err(DeepTimeError::OutOfDomain)
        );
    }

    #[test]
    fn an_absurdly_massive_black_hole_overflows_rather_than_lying() {
        assert!(black_hole_lifetime(1e120).is_err());
    }

    #[test]
    fn the_proton_decay_entry_is_labelled_a_bound_and_not_a_prediction() {
        assert_eq!(
            PROTON_DECAY_LOWER_BOUND.prediction,
            Prediction::ExperimentalBound
        );
        assert!(close(PROTON_DECAY_LOWER_BOUND.years_from_now, 2.4e34, 1e-9));
        assert_eq!(SUN_RED_GIANT_TIP.prediction, Prediction::Modelled);
        assert_eq!(
            END_OF_STAR_FORMATION.prediction,
            Prediction::OrderOfMagnitude
        );
    }

    #[test]
    fn an_order_of_magnitude_entry_wears_a_full_width_error_bar() {
        assert!(close(
            END_OF_STAR_FORMATION.std_dev_years,
            END_OF_STAR_FORMATION.years_from_now,
            1e-12
        ));
    }

    #[test]
    fn the_black_hole_era_ends_when_the_biggest_holes_evaporate() {
        let era = era_at_decade(99.0).unwrap();
        assert_eq!(era.name, "Black Hole Era");
        let decade = GALAXY_MASS_BLACK_HOLE_EVAPORATES
            .cosmological_decade()
            .unwrap();
        assert!(close(decade.value, 100.32, 1e-3), "eta = {}", decade.value);
    }

    #[test]
    fn a_decade_converts_to_a_span_and_back() {
        for decade in [6.0, 14.0, 37.0, 100.0] {
            let span = years_from_decade(decade).unwrap();
            let years = span.in_unit(DeepUnit::JulianYear).unwrap();
            let recovered = hc_core::math::log10(years.value);
            assert!(close(recovered, decade, 1e-12), "{recovered} vs {decade}");
        }
    }

    #[test]
    fn a_decade_beyond_the_reach_of_f64_is_refused() {
        assert_eq!(years_from_decade(400.0), Err(DeepTimeError::NotFinite));
    }

    #[test]
    fn the_eras_yield_usable_spans() {
        for era in ERAS {
            let start = era.start().unwrap();
            assert!(start.central_seconds() > 0.0, "{}", era.name);
            match era.end().unwrap() {
                Some(end) => assert!(end.central_cmp(start).is_gt(), "{}", era.name),
                None => assert_eq!(era.name, "Dark Era"),
            }
        }
    }

    #[test]
    fn the_next_event_is_found_for_any_horizon() {
        assert_eq!(
            event_after(0.0).map(|e| e.name),
            Some("The Sun leaves the main sequence")
        );
        assert_eq!(
            event_after(1e20).map(|e| e.name),
            Some("Proton decay lower bound")
        );
        assert!(event_after(1e200).is_none());
    }

    #[test]
    fn the_future_spans_more_than_ninety_decades() {
        let soonest = EVENTS.first().unwrap().from_now().unwrap();
        let latest = EVENTS.last().unwrap().from_now().unwrap();
        let decades = latest.orders_of_magnitude_between(soonest).unwrap();
        assert!(decades > 90.0, "only {decades} decades");
    }
}
