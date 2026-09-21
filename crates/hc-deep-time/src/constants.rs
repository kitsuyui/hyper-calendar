//! The Planck units, and the measured constants they are built from.
//!
//! Every number in this module is the CODATA 2022 recommended value, taken
//! from the NIST *Fundamental Physical Constants — Complete Listing* table at
//! <https://physics.nist.gov/cuu/Constants/Table/allascii.txt>, which is the
//! machine-readable form of Mohr, Newell, Taylor & Tiesinga, *CODATA
//! Recommended Values of the Fundamental Physical Constants: 2022*
//! (arXiv:2409.03787).
//!
//! # Why the Planck units are not exact
//!
//! Since the 2019 SI redefinition, `c`, `h` and `k` are *defined*: they have
//! no uncertainty at all. The Newtonian constant of gravitation `G` is not,
//! and it remains the worst-measured constant in the table at a relative
//! standard uncertainty of 2.2×10⁻⁵. Every Planck unit is built from `G`, so
//! every Planck unit inherits that uncertainty, halved or thirded by the
//! square and cube roots in its definition — which is why the Planck time,
//! length and mass all come out at 1.1×10⁻⁵ relative.
//!
//! That is the whole reason this crate exists. A library that stored the
//! Planck time as a bare `5.391247e-44` would be claiming seven digits of a
//! quantity known to six, and would be silently dropping the error bar that
//! every published use of the number carries.

use hc_uncertainty::{Significant, Uncertain};

use crate::error::{DeepTimeError, DeepTimeResult};

/// The CODATA adjustment these values come from.
pub const CODATA_YEAR: u16 = 2022;

/// A published physical constant with its stated uncertainty and its source.
///
/// `std_dev` is the *standard uncertainty* in CODATA's sense — one standard
/// deviation, the `u(x)` column of the NIST table — not a confidence
/// interval. A defined constant carries `std_dev == 0.0`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhysicalConstant {
    /// The constant's name as CODATA spells it.
    pub name: &'static str,
    /// The conventional symbol, in ASCII where the symbol has one.
    pub symbol: &'static str,
    /// The SI unit the value is expressed in.
    pub unit: &'static str,
    /// The recommended value.
    pub value: f64,
    /// The standard uncertainty, zero for a defined constant.
    pub std_dev: f64,
    /// How many digits of `value` the source actually prints.
    pub figures: u8,
    /// Where the value came from, named well enough to look up.
    pub source: &'static str,
}

impl PhysicalConstant {
    /// The value and its standard uncertainty as a Gaussian quantity.
    ///
    /// # Errors
    ///
    /// Returns [`DeepTimeError::NotFinite`] if the stored pair is not finite.
    pub fn uncertain(&self) -> DeepTimeResult<Uncertain> {
        Ok(Uncertain::new(self.value, self.std_dev)?)
    }

    /// The value with the figure count the source prints.
    ///
    /// # Errors
    ///
    /// Returns [`DeepTimeError::NotFinite`] for a non-finite value and
    /// propagates [`hc_uncertainty::UncertaintyError::InvalidSignificantFigures`]
    /// for a nonsensical figure count.
    pub fn significant(&self) -> DeepTimeResult<Significant> {
        Ok(Significant::new(self.value, self.figures)?)
    }

    /// The relative standard uncertainty `u(x)/|x|`, as CODATA tabulates it.
    ///
    /// # Errors
    ///
    /// Returns [`DeepTimeError::Uncertainty`] wrapping
    /// [`hc_uncertainty::UncertaintyError::DivideByZero`] for a constant whose
    /// value is zero, which none of these are.
    pub fn relative_uncertainty(&self) -> DeepTimeResult<f64> {
        Ok(self.uncertain()?.relative()?)
    }

    /// Whether the constant is exact by definition rather than measured.
    #[must_use]
    pub const fn is_defined(&self) -> bool {
        self.std_dev == 0.0
    }
}

/// The speed of light in vacuum, exact by the 2019 SI definition.
pub const SPEED_OF_LIGHT: PhysicalConstant = PhysicalConstant {
    name: "speed of light in vacuum",
    symbol: "c",
    unit: "m s^-1",
    value: 299_792_458.0,
    std_dev: 0.0,
    figures: 9,
    source: "SI (2019), exact by definition of the metre",
};

/// The reduced Planck constant, exact since `h` became a defining constant.
pub const REDUCED_PLANCK_CONSTANT: PhysicalConstant = PhysicalConstant {
    name: "reduced Planck constant",
    symbol: "hbar",
    unit: "J s",
    value: 1.054_571_817e-34,
    std_dev: 0.0,
    figures: 10,
    source: "CODATA 2022 (exact; h = 6.62607015e-34 J Hz^-1 by SI definition)",
};

/// The Newtonian constant of gravitation — the reason the Planck units are
/// uncertain at all.
///
/// At 2.2×10⁻⁵ relative this is the least precisely known of the constants
/// that enter everyday physics, and published determinations still disagree
/// by more than their quoted errors.
pub const NEWTONIAN_CONSTANT_OF_GRAVITATION: PhysicalConstant = PhysicalConstant {
    name: "Newtonian constant of gravitation",
    symbol: "G",
    unit: "m^3 kg^-1 s^-2",
    value: 6.674_30e-11,
    std_dev: 0.000_15e-11,
    figures: 6,
    source: "CODATA 2022, NIST allascii table",
};

/// The Planck time, `sqrt(hbar G / c^5)`.
///
/// 5.391247(60)×10⁻⁴⁴ s, a relative standard uncertainty of 1.1×10⁻⁵. Twenty
/// six decades below an attosecond, which is why `hc_core::Duration` cannot
/// hold it and this crate can.
pub const PLANCK_TIME: PhysicalConstant = PhysicalConstant {
    name: "Planck time",
    symbol: "t_P",
    unit: "s",
    value: 5.391_247e-44,
    std_dev: 0.000_060e-44,
    figures: 7,
    source: "CODATA 2022, NIST allascii table",
};

/// The Planck length, `sqrt(hbar G / c^3)` — 1.616255(18)×10⁻³⁵ m.
pub const PLANCK_LENGTH: PhysicalConstant = PhysicalConstant {
    name: "Planck length",
    symbol: "l_P",
    unit: "m",
    value: 1.616_255e-35,
    std_dev: 0.000_018e-35,
    figures: 7,
    source: "CODATA 2022, NIST allascii table",
};

/// The Planck mass, `sqrt(hbar c / G)` — 2.176434(24)×10⁻⁸ kg.
///
/// Unlike the other Planck units this one is macroscopic: about 22 micrograms,
/// roughly a flea's egg.
pub const PLANCK_MASS: PhysicalConstant = PhysicalConstant {
    name: "Planck mass",
    symbol: "m_P",
    unit: "kg",
    value: 2.176_434e-8,
    std_dev: 0.000_024e-8,
    figures: 7,
    source: "CODATA 2022, NIST allascii table",
};

/// The Planck energy, `m_P c^2`, in joules.
///
/// NIST tabulates this as the "Planck mass energy equivalent in GeV",
/// 1.220890(14)×10¹⁹ GeV. The joule value here is that same quantity
/// re-expressed with the defined elementary charge, and the two agree to the
/// digits printed. The relative uncertainty is the Planck mass's, because
/// `c` contributes none.
pub const PLANCK_ENERGY: PhysicalConstant = PhysicalConstant {
    name: "Planck energy",
    symbol: "E_P",
    unit: "J",
    value: 1.956_081e9,
    std_dev: 0.000_022e9,
    figures: 7,
    source: "CODATA 2022, derived as m_P c^2; cf. NIST 1.220890(14)e19 GeV",
};

/// The Planck temperature, `m_P c^2 / k` — 1.416784(16)×10³² K.
pub const PLANCK_TEMPERATURE: PhysicalConstant = PhysicalConstant {
    name: "Planck temperature",
    symbol: "T_P",
    unit: "K",
    value: 1.416_784e32,
    std_dev: 0.000_016e32,
    figures: 7,
    source: "CODATA 2022, NIST allascii table",
};

/// Every constant this module publishes, in no particular order.
pub const ALL: &[PhysicalConstant] = &[
    SPEED_OF_LIGHT,
    REDUCED_PLANCK_CONSTANT,
    NEWTONIAN_CONSTANT_OF_GRAVITATION,
    PLANCK_TIME,
    PLANCK_LENGTH,
    PLANCK_MASS,
    PLANCK_ENERGY,
    PLANCK_TEMPERATURE,
];

/// Look a constant up by its symbol, case-sensitively.
#[must_use]
pub fn by_symbol(symbol: &str) -> Option<&'static PhysicalConstant> {
    ALL.iter().find(|constant| constant.symbol == symbol)
}

/// The Julian year, 365.25 days of 86 400 SI seconds.
///
/// This is the IAU's definition (IAU 1976 System of Astronomical Constants)
/// and the one every "Gyr" in the astronomical literature means. It is a
/// defined conversion factor, not a measurement, so it is exact — and it is
/// deliberately not the tropical or the Gregorian mean year, both of which
/// drift.
pub const JULIAN_YEAR_SECONDS: f64 = 31_557_600.0;

/// Reconstruct the Planck time from `hbar`, `G` and `c` and check it.
///
/// `t_P = sqrt(hbar G / c^5)`. This exists so that the tabulated value above
/// can be tested against its own definition rather than only against itself,
/// and so that a caller who wants the covariance-free propagation can see
/// exactly which constant contributes the error.
///
/// # Errors
///
/// Propagates the uncertainty layer's errors; none of them can occur for the
/// stored constants.
pub fn planck_time_from_definition() -> DeepTimeResult<Uncertain> {
    let hbar = REDUCED_PLANCK_CONSTANT.uncertain()?;
    let gravitation = NEWTONIAN_CONSTANT_OF_GRAVITATION.uncertain()?;
    let light = SPEED_OF_LIGHT.uncertain()?;
    let numerator = hbar.checked_mul(gravitation)?;
    let denominator = light.powf(5.0)?;
    let ratio = numerator.checked_div(denominator)?;
    if ratio.value <= 0.0 {
        return Err(DeepTimeError::OutOfDomain);
    }
    Ok(ratio.powf(0.5)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    use hc_core::math;

    #[test]
    fn the_planck_time_matches_the_current_codata_value() {
        assert!((PLANCK_TIME.value - 5.391_247e-44).abs() < 1e-55);
        // 5.391247(60)e-44 means u = 0.000060e-44, which is 6.0e-49 s.
        assert!((PLANCK_TIME.std_dev / 6.0e-49 - 1.0).abs() < 1e-12);
    }

    #[test]
    fn every_planck_unit_is_known_to_about_one_part_in_ninety_thousand() {
        // G enters t_P, l_P and m_P under a square root, so each inherits half
        // of G's 2.2e-5 relative uncertainty.
        for constant in [PLANCK_TIME, PLANCK_LENGTH, PLANCK_MASS, PLANCK_TEMPERATURE] {
            let relative = constant.relative_uncertainty().unwrap();
            assert!(
                (relative - 1.1e-5).abs() < 0.1e-5,
                "{} has relative uncertainty {relative}",
                constant.name
            );
        }
    }

    #[test]
    fn gravitation_carries_twice_the_relative_uncertainty_of_the_planck_units() {
        let g_relative = NEWTONIAN_CONSTANT_OF_GRAVITATION
            .relative_uncertainty()
            .unwrap();
        let t_relative = PLANCK_TIME.relative_uncertainty().unwrap();
        assert!((g_relative / t_relative - 2.0).abs() < 0.05);
    }

    #[test]
    fn the_defined_constants_carry_no_uncertainty() {
        assert!(SPEED_OF_LIGHT.is_defined());
        assert!(REDUCED_PLANCK_CONSTANT.is_defined());
        assert!(!NEWTONIAN_CONSTANT_OF_GRAVITATION.is_defined());
        assert_eq!(
            SPEED_OF_LIGHT.relative_uncertainty().unwrap(),
            0.0,
            "a defined constant has zero relative uncertainty"
        );
    }

    #[test]
    fn the_tabulated_planck_time_agrees_with_its_own_definition() {
        let derived = planck_time_from_definition().unwrap();
        let tabulated = PLANCK_TIME.uncertain().unwrap();
        // Agreement to a fraction of the quoted uncertainty; the residual is
        // the rounding CODATA applies when it prints seven digits.
        let separation = math::abs(derived.value - tabulated.value);
        assert!(
            separation < tabulated.std_dev,
            "derived {} vs tabulated {}",
            derived.value,
            tabulated.value
        );
        assert!(derived.overlaps(tabulated));
    }

    #[test]
    fn the_planck_energy_agrees_with_the_mass_times_c_squared() {
        let mass = PLANCK_MASS.uncertain().unwrap();
        let light = SPEED_OF_LIGHT.uncertain().unwrap();
        let derived = mass.checked_mul(light.powf(2.0).unwrap()).unwrap();
        let tabulated = PLANCK_ENERGY.uncertain().unwrap();
        assert!(
            math::abs(derived.value - tabulated.value) < tabulated.std_dev,
            "derived {} vs tabulated {}",
            derived.value,
            tabulated.value
        );
    }

    #[test]
    fn every_constant_renders_at_the_precision_its_source_prints() {
        for constant in ALL {
            let significant = constant.significant().unwrap();
            assert_eq!(significant.figures(), constant.figures);
            assert!(
                !constant.source.is_empty(),
                "{} has no source",
                constant.name
            );
        }
    }

    #[test]
    fn no_constant_carries_a_negative_uncertainty() {
        for constant in ALL {
            assert!(constant.std_dev >= 0.0, "{} is negative", constant.name);
            assert!(constant.value.is_finite());
        }
    }

    #[test]
    fn constants_are_reachable_by_symbol() {
        assert_eq!(by_symbol("t_P"), Some(&PLANCK_TIME));
        assert_eq!(by_symbol("c"), Some(&SPEED_OF_LIGHT));
        assert_eq!(by_symbol("not a symbol"), None);
    }

    #[test]
    fn the_julian_year_is_exactly_three_hundred_and_sixty_five_and_a_quarter_days() {
        assert!((JULIAN_YEAR_SECONDS - 365.25 * 86_400.0).abs() < 1e-9);
    }

    #[test]
    fn the_codata_adjustment_is_named() {
        assert_eq!(CODATA_YEAR, 2022);
    }
}
