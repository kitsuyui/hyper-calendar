//! The chronology of the universe, as data.
//!
//! Every time in this module is measured from the Big Bang, in SI seconds,
//! and every entry names where its number came from.
//!
//! # The cosmology used
//!
//! All ages are on the **Planck 2018 base-ΛCDM parameters**, specifically the
//! `TT,TE,EE+lowE+lensing+BAO` column of Table 2 of Planck Collaboration,
//! *Planck 2018 results. VI. Cosmological parameters*, A&A 641, A6 (2020),
//! arXiv:1807.06209:
//!
//! | Parameter | Value |
//! | --- | --- |
//! | Age of the universe | 13.787 ± 0.020 Gyr |
//! | `H0` | 67.66 ± 0.42 km s⁻¹ Mpc⁻¹ |
//! | `Ωm` | 0.3111 ± 0.0056 |
//! | `z*` (last scattering) | 1089.80 ± 0.21 |
//! | `z_eq` (matter–radiation equality) | 3387 ± 21 |
//!
//! Using one column throughout matters: mixing the `+BAO` age with a
//! `lensing`-only redshift would produce a chronology that is not internally
//! consistent with any single likelihood.
//!
//! # How the early ages were obtained
//!
//! Planck publishes redshifts to four digits and ages in years only as round
//! numbers. The ages here for matter–radiation equality, recombination,
//! reionisation and the first galaxies were obtained by integrating the flat
//! ΛCDM Friedmann equation
//!
//! ```text
//! t(a) = (1/H0) ∫₀^a a' da' / sqrt(Ωr + Ωm a' + ΩΛ a'⁴)
//! ```
//!
//! with the parameters above and `Ωr h² = 4.1834×10⁻⁵` (photons at
//! `T = 2.7255 K` plus `Neff = 3.046` relativistic neutrino species). Run at
//! `a = 1` the same integral returns 13.786 Gyr against Planck's published
//! 13.787 ± 0.020, which is the check that the integration is right. The
//! quoted `σ` on each early age is the spread obtained by moving `H0` and
//! `Ωm` over their own 1σ ranges, rounded up.
//!
//! # Order-of-magnitude boundaries
//!
//! The epochs before nucleosynthesis have no measured boundaries at all. They
//! are the conventional powers of ten used in every textbook chronology, and
//! this module writes their uncertainty as **equal to the value itself** —
//! a 100 % relative error bar, which is this crate's way of saying "this is a
//! decade, not a measurement". Any narrower `σ` would be a fabrication, and
//! omitting it would be worse.

use crate::error::DeepTimeResult;
use crate::magnitude::DeepTime;

/// The cosmological parameter set every age here is consistent with.
pub const PARAMETER_SET: &str =
    "Planck 2018 results VI, Table 2, TT,TE,EE+lowE+lensing+BAO (A&A 641, A6)";

/// A contiguous epoch of cosmic history.
///
/// Epochs tile the whole span from the Big Bang to the present without gaps
/// or overlaps, so that [`epoch_at`] always has exactly one answer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CosmicEpoch {
    /// The epoch's conventional name.
    pub name: &'static str,
    /// What physically distinguishes it.
    pub description: &'static str,
    /// Where the boundary times came from.
    pub source: &'static str,
    start_seconds: f64,
    start_std_dev: f64,
    end_seconds: f64,
    end_std_dev: f64,
    figures: u8,
}

impl CosmicEpoch {
    /// When the epoch opened, measured from the Big Bang.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::new`]; cannot fail for the tabulated entries.
    pub fn start(&self) -> DeepTimeResult<DeepTime> {
        DeepTime::from_seconds(self.start_seconds, self.start_std_dev)?.with_figures(self.figures)
    }

    /// When the epoch closed, measured from the Big Bang.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::new`]; cannot fail for the tabulated entries.
    pub fn end(&self) -> DeepTimeResult<DeepTime> {
        DeepTime::from_seconds(self.end_seconds, self.end_std_dev)?.with_figures(self.figures)
    }

    /// The opening time's central value, in seconds since the Big Bang.
    #[must_use]
    pub const fn start_seconds(&self) -> f64 {
        self.start_seconds
    }

    /// The closing time's central value, in seconds since the Big Bang.
    #[must_use]
    pub const fn end_seconds(&self) -> f64 {
        self.end_seconds
    }

    /// The standard uncertainty on the opening time, in seconds.
    #[must_use]
    pub const fn start_std_dev(&self) -> f64 {
        self.start_std_dev
    }

    /// The standard uncertainty on the closing time, in seconds.
    #[must_use]
    pub const fn end_std_dev(&self) -> f64 {
        self.end_std_dev
    }

    /// Whether `seconds` after the Big Bang falls in `[start, end)`.
    ///
    /// The comparison is on central values only. When the query time's own
    /// error bar straddles a boundary the answer is still a single epoch, so
    /// callers who care should compare
    /// [`CosmicEpoch::start`] against their value with
    /// [`DeepTime::overlaps`].
    #[must_use]
    pub fn contains_seconds(&self, seconds: f64) -> bool {
        seconds >= self.start_seconds && seconds < self.end_seconds
    }
}

/// A dated event rather than an interval.
///
/// Events are point milestones — a decoupling, a first light, a formation —
/// and unlike [`CosmicEpoch`] they are allowed to fall inside an epoch.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CosmicEvent {
    /// The event's conventional name.
    pub name: &'static str,
    /// What happened, and what is actually measured about it.
    pub description: &'static str,
    /// Where the number came from.
    pub source: &'static str,
    seconds: f64,
    std_dev: f64,
    figures: u8,
}

impl CosmicEvent {
    /// When it happened, measured from the Big Bang.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::new`]; cannot fail for the tabulated entries.
    pub fn deep_time(&self) -> DeepTimeResult<DeepTime> {
        DeepTime::from_seconds(self.seconds, self.std_dev)?.with_figures(self.figures)
    }

    /// The central value, in seconds since the Big Bang.
    #[must_use]
    pub const fn seconds(&self) -> f64 {
        self.seconds
    }

    /// The standard uncertainty, in seconds.
    #[must_use]
    pub const fn std_dev(&self) -> f64 {
        self.std_dev
    }
}

// --- Epochs -------------------------------------------------------------
//
// Boundaries before nucleosynthesis are the conventional decades of the
// standard chronology (Kolb & Turner, "The Early Universe", 1990, ch. 3;
// Peebles, "Principles of Physical Cosmology", 1993). They carry a 100 %
// relative uncertainty, which is the honest reading of "about 10^-36 s".

/// The epochs of cosmic history, oldest first, tiling the whole span.
pub const EPOCHS: &[CosmicEpoch] = &[
    CosmicEpoch {
        name: "Planck epoch",
        description: "Before about 10^-43 s the four interactions are expected to be unified and \
                      no tested theory applies: general relativity and quantum field theory give \
                      contradictory answers, so the epoch is bounded by the Planck time rather \
                      than described by it. t = 0 here is the limit of the classical model, not \
                      an observed instant.",
        source: "Conventional; bounded by the CODATA 2022 Planck time, 5.391247(60)e-44 s",
        start_seconds: 0.0,
        start_std_dev: 0.0,
        end_seconds: 1e-43,
        end_std_dev: 1e-43,
        figures: 1,
    },
    CosmicEpoch {
        name: "Grand unification epoch",
        description: "Gravity has separated; the strong, weak and electromagnetic interactions \
                      are still described by a single grand unified group. No accelerator has \
                      ever reached these energies, so the epoch is a theoretical expectation.",
        source: "Conventional order of magnitude; Kolb & Turner, The Early Universe (1990)",
        start_seconds: 1e-43,
        start_std_dev: 1e-43,
        end_seconds: 1e-36,
        end_std_dev: 1e-36,
        figures: 1,
    },
    CosmicEpoch {
        name: "Inflationary epoch",
        description: "An interval of accelerated expansion that flattens the geometry, dilutes \
                      relics and seeds the density perturbations later seen in the CMB. Planck \
                      measures the spectral index of those perturbations, ns = 0.9665 +/- 0.0038, \
                      but not the epoch's duration or its energy scale.",
        source: "Planck 2018 results X, Constraints on inflation (A&A 641, A10); timing \
                 conventional",
        start_seconds: 1e-36,
        start_std_dev: 1e-36,
        end_seconds: 1e-32,
        end_std_dev: 1e-32,
        figures: 1,
    },
    CosmicEpoch {
        name: "Electroweak epoch",
        description: "After reheating, the strong interaction has separated but the electroweak \
                      one has not. It ends at the electroweak phase transition near 100 GeV, the \
                      earliest epoch whose energy scale a laboratory has actually reached.",
        source: "Conventional; the 100 GeV scale is measured, the 10^-12 s timing derived from it",
        start_seconds: 1e-32,
        start_std_dev: 1e-32,
        end_seconds: 1e-12,
        end_std_dev: 1e-12,
        figures: 1,
    },
    CosmicEpoch {
        name: "Quark epoch",
        description: "Quarks, leptons and gauge bosons are free; the universe is a quark-gluon \
                      plasma of the kind heavy-ion colliders now recreate for about 10^-23 s at a \
                      time. It ends at the QCD confinement transition near 150 MeV.",
        source: "Conventional; confinement temperature from lattice QCD, ~156 MeV",
        start_seconds: 1e-12,
        start_std_dev: 1e-12,
        end_seconds: 1e-6,
        end_std_dev: 1e-6,
        figures: 1,
    },
    CosmicEpoch {
        name: "Hadron epoch",
        description: "Quarks are confined into hadrons. Most baryons and antibaryons annihilate; \
                      the roughly one-in-a-billion baryon excess that survives is everything \
                      ordinary matter is now made of.",
        source: "Conventional; the baryon-to-photon ratio is measured, Planck 2018 VI",
        start_seconds: 1e-6,
        start_std_dev: 1e-6,
        end_seconds: 1.0,
        end_std_dev: 1.0,
        figures: 1,
    },
    CosmicEpoch {
        name: "Lepton epoch",
        description: "Hadron-antihadron annihilation is complete and leptons dominate the mass \
                      of the universe. Neutrinos decouple at its opening and electron-positron \
                      annihilation ends it, reheating the photons relative to the neutrinos by \
                      the factor (11/4)^(1/3).",
        source: "Conventional; the neutrino temperature ratio is a standard model prediction",
        start_seconds: 1.0,
        start_std_dev: 1.0,
        end_seconds: 10.0,
        end_std_dev: 5.0,
        figures: 1,
    },
    CosmicEpoch {
        name: "Photon epoch",
        description: "Radiation dominates the energy density, then matter does. Primordial \
                      nucleosynthesis happens in its first twenty minutes and nothing much else \
                      happens for the next 370 000 years, until the plasma recombines and the \
                      photons go free as the cosmic microwave background.",
        source: "End = age at z* = 1089.80 +/- 0.21, Planck 2018 VI Table 2, integrated",
        start_seconds: 10.0,
        start_std_dev: 5.0,
        end_seconds: 1.17333e13,
        end_std_dev: 1.2e11,
        figures: 5,
    },
    CosmicEpoch {
        name: "Dark ages",
        description: "The CMB has redshifted out of the visible, no star has yet formed, and the \
                      universe contains no source of visible light at all. Only the 21 cm line of \
                      neutral hydrogen can in principle see into this interval.",
        source: "Start from z*; end when the first stars light up, z ~ 20, model-dependent",
        start_seconds: 1.17333e13,
        start_std_dev: 1.2e11,
        end_seconds: 5.68e15,
        end_std_dev: 2.0e15,
        figures: 2,
    },
    CosmicEpoch {
        name: "Reionisation",
        description: "Radiation from the first stars and quasars reionises the intergalactic \
                      medium. Planck measures its midpoint through the optical depth to \
                      scattering, z_re = 7.82 +/- 0.71; quasar absorption spectra show it \
                      complete by about z = 5.3.",
        source: "z_re from Planck 2018 VI Table 2; completion redshift from Lyman-alpha forest \
                 studies; ages integrated",
        start_seconds: 5.68e15,
        start_std_dev: 2.0e15,
        end_seconds: 3.4334e16,
        end_std_dev: 5.2e14,
        figures: 2,
    },
    CosmicEpoch {
        name: "Era of galaxies",
        description: "The present epoch: galaxies assemble, stars form and die, and after about \
                      9.8 Gyr the expansion begins to accelerate as the cosmological constant \
                      comes to dominate. It runs from the end of reionisation to now.",
        source: "End = the age of the universe, Planck 2018 VI Table 2",
        start_seconds: 3.4334e16,
        start_std_dev: 5.2e14,
        end_seconds: 4.350846e17,
        end_std_dev: 6.312e14,
        figures: 5,
    },
];

// --- Events -------------------------------------------------------------

/// Neutrino decoupling, about one second after the Big Bang.
pub const NEUTRINO_DECOUPLING: CosmicEvent = CosmicEvent {
    name: "Neutrino decoupling",
    description: "The weak interaction rate drops below the expansion rate and neutrinos stop \
         scattering, leaving a relic neutrino background at about 1.95 K today. It has never \
         been detected directly; its existence is inferred from Neff = 2.99 +/- 0.17 in the CMB.",
    source: "Conventional ~1 s; Neff from Planck 2018 results VI",
    seconds: 1.0,
    std_dev: 0.5,
    figures: 1,
};

/// Big Bang nucleosynthesis, beginning about ten seconds in.
pub const BIG_BANG_NUCLEOSYNTHESIS: CosmicEvent = CosmicEvent {
    name: "Big Bang nucleosynthesis",
    description: "Protons and neutrons fuse into deuterium, helium-4, helium-3 and lithium-7 \
                  over roughly twenty minutes, freezing out at a helium mass fraction near 0.247. \
                  This is the earliest epoch with a directly testable prediction, and the \
                  measured primordial abundances match it.",
    source: "Cyburt, Fields, Olive & Yeh, Rev. Mod. Phys. 88, 015004 (2016); timing conventional",
    seconds: 10.0,
    std_dev: 5.0,
    figures: 1,
};

/// Matter–radiation equality, about 51 000 years in.
pub const MATTER_RADIATION_EQUALITY: CosmicEvent = CosmicEvent {
    name: "Matter-radiation equality",
    description: "The matter and radiation energy densities cross at z_eq = 3387 +/- 21. Before \
                  it, perturbations inside the horizon cannot grow; after it, they can, so this \
                  is where the structure of the present universe starts to assemble.",
    source: "z_eq from Planck 2018 VI Table 2; age by Friedmann integration, 5.14e4 yr",
    seconds: 1.62161e12,
    std_dev: 2.4e10,
    figures: 3,
};

/// Recombination and photon decoupling — the cosmic microwave background.
pub const RECOMBINATION: CosmicEvent = CosmicEvent {
    name: "Recombination and photon decoupling",
    description: "Free electrons combine with protons into neutral hydrogen, the Thomson \
                  scattering rate collapses, and the photons stream freely from a last-scattering \
                  surface at z* = 1089.80 +/- 0.21. Those photons are the cosmic microwave \
                  background, and they are the oldest thing anyone has ever seen.",
    source: "z* from Planck 2018 VI Table 2; age by Friedmann integration, 3.72e5 yr",
    seconds: 1.17333e13,
    std_dev: 1.2e11,
    figures: 3,
};

/// The first stars, around 180 million years in.
pub const FIRST_STARS: CosmicEvent = CosmicEvent {
    name: "First stars",
    description: "Population III stars form in minihaloes from metal-free gas. No individual \
                  Population III star has been observed; the epoch is bracketed by simulations \
                  (z ~ 15-30) and by the reionisation optical depth, so the error bar here is \
                  wide on purpose.",
    source: "Bromm & Larson, ARA&A 42, 79 (2004); redshift range model-dependent",
    seconds: 5.68e15,
    std_dev: 2.0e15,
    figures: 2,
};

/// The earliest spectroscopically confirmed galaxy, about 286 million years in.
pub const FIRST_GALAXIES: CosmicEvent = CosmicEvent {
    name: "First galaxies",
    description: "JADES-GS-z14-0, confirmed by JWST at z = 14.32, is the earliest galaxy with a \
                  spectroscopic redshift. It is an observational record rather than a physical \
                  boundary: galaxies certainly formed earlier, and this bound has moved every \
                  year since JWST began observing.",
    source: "Carniani et al., Nature 633, 318 (2024); age by Friedmann integration, 2.86e8 yr",
    seconds: 9.0338e15,
    std_dev: 1.4e14,
    figures: 3,
};

/// The Milky Way's oldest disc, about 800 million years in.
pub const MILKY_WAY_FORMATION: CosmicEvent = CosmicEvent {
    name: "Formation of the Milky Way",
    description: "Asteroseismic and Gaia ages for a quarter of a million subgiants place the \
                  start of the Galaxy's old (thick) disc about 13 Gyr ago, roughly 0.8 Gyr after \
                  the Big Bang. The halo and the thin disc bracket it on either side, so \
                  \"the formation of the Milky Way\" is an interval, not an instant.",
    source: "Xiang & Rix, Nature 603, 599 (2022)",
    seconds: 2.5246e16,
    std_dev: 3.2e15,
    figures: 2,
};

/// The Sun and Solar System, about 9.22 billion years in.
pub const SOLAR_SYSTEM_FORMATION: CosmicEvent = CosmicEvent {
    name: "Formation of the Sun and Solar System",
    description: "Calcium-aluminium-rich inclusions in primitive meteorites, the oldest dated \
                  solids in the Solar System, give a U-corrected Pb-Pb age of 4567.30 +/- 0.16 \
                  Ma. That is the conventional t = 0 of Solar System chronology; the Earth \
                  finished accreting some 30-100 Myr later.",
    source: "Connelly et al., Science 338, 651 (2012); offset from the Planck 2018 age",
    seconds: 2.909516e17,
    std_dev: 6.4e14,
    figures: 5,
};

/// The present day, 13.787 ± 0.020 Gyr after the Big Bang.
pub const PRESENT_DAY: CosmicEvent = CosmicEvent {
    name: "The present",
    description: "13.787 +/- 0.020 Gyr after the Big Bang. The 0.14 % uncertainty is why this \
                  crate exists: it is five significant figures, not eleven, and every derived \
                  age inherits it.",
    source: "Planck 2018 results VI, Table 2, TT,TE,EE+lowE+lensing+BAO",
    seconds: 4.350846e17,
    std_dev: 6.312e14,
    figures: 5,
};

/// The age of the universe — the anchor every "years ago" here is measured
/// back from.
pub const AGE_OF_UNIVERSE: CosmicEvent = PRESENT_DAY;

/// Dated events of cosmic history, oldest first.
pub const EVENTS: &[CosmicEvent] = &[
    NEUTRINO_DECOUPLING,
    BIG_BANG_NUCLEOSYNTHESIS,
    MATTER_RADIATION_EQUALITY,
    RECOMBINATION,
    FIRST_STARS,
    FIRST_GALAXIES,
    MILKY_WAY_FORMATION,
    SOLAR_SYSTEM_FORMATION,
    PRESENT_DAY,
];

/// Which epoch `seconds` after the Big Bang falls in.
///
/// Returns `None` before the Big Bang and after the present day; the future
/// lives in [`crate::future`]. Epochs are half-open `[start, end)`, with one
/// exception: the closing boundary of the last epoch *is* the present day,
/// and the present day is in it.
#[must_use]
pub fn epoch_at(seconds: f64) -> Option<&'static CosmicEpoch> {
    EPOCHS
        .iter()
        .find(|epoch| epoch.contains_seconds(seconds))
        .or_else(|| EPOCHS.last().filter(|last| seconds == last.end_seconds))
}

/// The most recent dated event at or before `seconds` after the Big Bang.
#[must_use]
pub fn event_before(seconds: f64) -> Option<&'static CosmicEvent> {
    EVENTS.iter().rev().find(|event| event.seconds <= seconds)
}

/// Look an epoch up by name, case-sensitively.
#[must_use]
pub fn epoch_by_name(name: &str) -> Option<&'static CosmicEpoch> {
    EPOCHS.iter().find(|epoch| epoch.name == name)
}

/// Convert "years before the present day" into "seconds after the Big Bang".
///
/// This is the bridge between the cosmological convention (count forward from
/// the Big Bang) and every other convention on this planet (count backward
/// from now). The result inherits the age of the universe's own 0.020 Gyr
/// uncertainty, which swamps everything younger than about a gigayear.
///
/// # Errors
///
/// Returns [`crate::DeepTimeError::NotFinite`] on overflow.
pub fn since_big_bang_from_years_ago(
    years_ago: f64,
    std_dev_years: f64,
) -> DeepTimeResult<DeepTime> {
    let age = AGE_OF_UNIVERSE.deep_time()?;
    let ago = DeepTime::from_julian_years(years_ago, std_dev_years)?;
    age.checked_sub(ago)
}

#[cfg(test)]
mod tests {
    use super::*;

    use hc_core::math;

    use crate::constants;

    #[test]
    fn the_epochs_tile_cosmic_history_without_gaps_or_overlaps() {
        for pair in EPOCHS.windows(2) {
            assert!(
                (pair[0].end_seconds - pair[1].start_seconds).abs() <= 0.0,
                "{} ends at {} but {} starts at {}",
                pair[0].name,
                pair[0].end_seconds,
                pair[1].name,
                pair[1].start_seconds
            );
        }
    }

    #[test]
    fn every_epoch_runs_strictly_forwards() {
        for epoch in EPOCHS {
            assert!(
                epoch.end_seconds > epoch.start_seconds,
                "{} runs backwards",
                epoch.name
            );
        }
    }

    #[test]
    fn the_epochs_start_at_the_big_bang_and_end_now() {
        let first = EPOCHS.first().unwrap();
        let last = EPOCHS.last().unwrap();
        assert_eq!(first.start_seconds, 0.0);
        assert!((last.end_seconds - AGE_OF_UNIVERSE.seconds).abs() < 1e3);
    }

    #[test]
    fn the_events_are_in_strictly_increasing_order() {
        for pair in EVENTS.windows(2) {
            assert!(
                pair[1].seconds > pair[0].seconds,
                "{} is not after {}",
                pair[1].name,
                pair[0].name
            );
        }
    }

    #[test]
    fn no_entry_carries_a_negative_uncertainty() {
        for epoch in EPOCHS {
            assert!(epoch.start_std_dev >= 0.0, "{}", epoch.name);
            assert!(epoch.end_std_dev >= 0.0, "{}", epoch.name);
        }
        for event in EVENTS {
            assert!(event.std_dev >= 0.0, "{}", event.name);
        }
    }

    #[test]
    fn every_entry_names_a_source_and_describes_itself() {
        for epoch in EPOCHS {
            assert!(!epoch.source.is_empty(), "{} has no source", epoch.name);
            assert!(
                epoch.description.len() > 40,
                "{} is undescribed",
                epoch.name
            );
        }
        for event in EVENTS {
            assert!(!event.source.is_empty(), "{} has no source", event.name);
            assert!(
                event.description.len() > 40,
                "{} is undescribed",
                event.name
            );
        }
    }

    #[test]
    fn every_event_falls_inside_an_epoch() {
        for event in EVENTS {
            // The present day sits exactly on the closing boundary, which is
            // half-open, so it is the one legitimate exception.
            if event.seconds >= AGE_OF_UNIVERSE.seconds {
                continue;
            }
            assert!(
                epoch_at(event.seconds).is_some(),
                "{} at {} s falls outside every epoch",
                event.name,
                event.seconds
            );
        }
    }

    #[test]
    fn the_planck_epoch_ends_a_few_planck_times_in() {
        let planck_epoch = EPOCHS.first().unwrap();
        let ratio = planck_epoch.end_seconds / constants::PLANCK_TIME.value;
        assert!((1.0..10.0).contains(&ratio), "ratio was {ratio}");
    }

    #[test]
    fn the_age_of_the_universe_is_thirteen_point_seven_eight_seven_gigayears() {
        let age = AGE_OF_UNIVERSE.deep_time().unwrap();
        let gigayears = age.in_unit(crate::DeepUnit::Gigayear).unwrap();
        assert!(
            (gigayears.value - 13.787).abs() < 1e-4,
            "{}",
            gigayears.value
        );
        assert!(
            (gigayears.std_dev - 0.020).abs() < 1e-4,
            "{}",
            gigayears.std_dev
        );
    }

    #[test]
    fn the_universe_is_about_eight_times_ten_to_the_sixty_planck_times_old() {
        let age = AGE_OF_UNIVERSE.deep_time().unwrap();
        let planck = DeepTime::from_planck_times(1.0, 0.0).unwrap();
        let ratio = age.ratio(planck).unwrap();
        assert!(
            (ratio.value / 8.0e60 - 1.0).abs() < 0.02,
            "the age is {} Planck times",
            ratio.value
        );
    }

    #[test]
    fn recombination_is_about_three_hundred_and_seventy_thousand_years_in() {
        let time = RECOMBINATION.deep_time().unwrap();
        let years = time.in_unit(crate::DeepUnit::JulianYear).unwrap();
        assert!((years.value - 3.72e5).abs() < 1e4, "{}", years.value);
    }

    #[test]
    fn matter_radiation_equality_precedes_recombination() {
        let order = [MATTER_RADIATION_EQUALITY, RECOMBINATION];
        assert!(order[1].seconds > order[0].seconds);
        let decades = RECOMBINATION
            .deep_time()
            .unwrap()
            .orders_of_magnitude_between(MATTER_RADIATION_EQUALITY.deep_time().unwrap())
            .unwrap();
        assert!((decades - 0.86).abs() < 0.05, "{decades} decades apart");
    }

    #[test]
    fn the_solar_system_formed_four_and_a_half_gigayears_before_the_present() {
        let before_now = AGE_OF_UNIVERSE.seconds - SOLAR_SYSTEM_FORMATION.seconds;
        let gigayears = before_now / constants::JULIAN_YEAR_SECONDS / 1e9;
        assert!((gigayears - 4.5673).abs() < 0.01, "{gigayears} Ga");
    }

    #[test]
    fn a_query_lands_in_the_right_epoch() {
        assert_eq!(
            epoch_at(1e-40).map(|e| e.name),
            Some("Grand unification epoch")
        );
        assert_eq!(epoch_at(1e-20).map(|e| e.name), Some("Electroweak epoch"));
        assert_eq!(epoch_at(1e-3).map(|e| e.name), Some("Hadron epoch"));
        assert_eq!(epoch_at(5.0).map(|e| e.name), Some("Lepton epoch"));
        assert_eq!(epoch_at(1e10).map(|e| e.name), Some("Photon epoch"));
        assert_eq!(epoch_at(1e17).map(|e| e.name), Some("Era of galaxies"));
    }

    #[test]
    fn a_query_outside_cosmic_history_finds_nothing() {
        assert!(epoch_at(-1.0).is_none());
        assert!(epoch_at(1e30).is_none());
    }

    #[test]
    fn epochs_are_reachable_by_name() {
        assert!(epoch_by_name("Dark ages").is_some());
        assert!(
            epoch_by_name("dark ages").is_none(),
            "lookup is case-sensitive"
        );
    }

    #[test]
    fn the_preceding_event_is_found_for_any_time() {
        assert_eq!(event_before(0.5).map(|e| e.name), None);
        assert_eq!(
            event_before(2.0).map(|e| e.name),
            Some("Neutrino decoupling")
        );
        assert_eq!(
            event_before(1e14).map(|e| e.name),
            Some("Recombination and photon decoupling")
        );
        assert_eq!(event_before(1e18).map(|e| e.name), Some("The present"));
    }

    #[test]
    fn years_ago_converts_back_to_seconds_since_the_big_bang() {
        // The Solar System, 4.5673 Ga ago.
        let then = since_big_bang_from_years_ago(4.5673e9, 0.0).unwrap();
        assert!(
            math::abs(then.central_seconds() - SOLAR_SYSTEM_FORMATION.seconds) < 1e15,
            "{} vs {}",
            then.central_seconds(),
            SOLAR_SYSTEM_FORMATION.seconds
        );
        // And it inherits the age of the universe's error bar.
        assert!(then.std_dev() > 6e14);
    }

    #[test]
    fn the_parameter_set_is_named_in_the_crate_itself() {
        assert!(PARAMETER_SET.contains("Planck 2018"));
        assert!(PARAMETER_SET.contains("BAO"));
    }

    #[test]
    fn the_early_epochs_wear_their_ignorance_openly() {
        // A 100 % relative error bar is how this module writes "a decade".
        for epoch in &EPOCHS[1..6] {
            assert!(
                (epoch.start_std_dev - epoch.start_seconds).abs() < epoch.start_seconds * 1e-9,
                "{} pretends to know its start",
                epoch.name
            );
        }
    }

    #[test]
    fn every_epoch_and_event_yields_a_usable_deep_time() {
        for epoch in EPOCHS {
            let start = epoch.start().unwrap();
            let end = epoch.end().unwrap();
            assert!(end.central_cmp(start).is_gt(), "{}", epoch.name);
        }
        for event in EVENTS {
            let time = event.deep_time().unwrap();
            assert!(time.central_seconds() > 0.0, "{}", event.name);
            assert!(time.figures() >= 1);
        }
    }
}
