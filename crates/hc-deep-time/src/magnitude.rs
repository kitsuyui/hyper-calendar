//! [`DeepTime`]: a span in seconds that is known to a stated precision.
//!
//! [`hc_core::Duration`] is exact and covers everything a clock can measure.
//! Below an attosecond and above the range where "a count of seconds" is a
//! sensible answer, values are not exact: they are published magnitudes with
//! error bars. Those live here.
//!
//! A `DeepTime` is a [`hc_uncertainty::Uncertain`] number of SI seconds plus
//! the figure count the source actually printed. Both halves matter, and they
//! answer different questions: the error bar says how wrong the number might
//! be, and the figure count says how many digits may be shown.
//!
//! # Why logarithms
//!
//! The span from the Planck time to the age of the universe is sixty-one
//! decades. At that range the difference between two values carries no
//! information — 10¹⁷ s minus 10⁻⁴³ s is 10¹⁷ s — and the ratio carries all
//! of it. So [`DeepTime::log10_ratio`] and
//! [`DeepTime::orders_of_magnitude_between`] are the comparison operators
//! that mean anything here, and [`DeepTime::logarithmic`] is the rendering
//! that shows it.

use core::cmp::Ordering;
use core::fmt;

use hc_core::{Duration, math};
use hc_uncertainty::{MAX_FIGURES, Significant, Uncertain};

use crate::constants;
use crate::error::{DeepTimeError, DeepTimeResult};

/// `ln(10)`, the conversion between natural and decimal logarithms.
const LN_10: f64 = core::f64::consts::LN_10;

/// A unit the deep-time literature actually quotes spans in.
///
/// The SI prefixes run down to yocto because that is where particle lifetimes
/// stop and where the SI prefix table stopped until 2022; below it the only
/// unit anyone uses is the Planck time itself. Upwards the sequence stops at
/// the gigayear, because above that the literature switches to powers of ten
/// of years and this crate follows it — see [`crate::future`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DeepUnit {
    /// The Planck time, `sqrt(hbar G / c^5)`. The only *measured* unit here.
    PlanckTime,
    /// 10⁻²⁴ s.
    Yoctosecond,
    /// 10⁻²¹ s.
    Zeptosecond,
    /// 10⁻¹⁸ s, the resolution floor of the exact `hc_core::Duration`.
    Attosecond,
    /// 10⁻¹⁵ s.
    Femtosecond,
    /// 10⁻¹² s.
    Picosecond,
    /// 10⁻⁹ s.
    Nanosecond,
    /// 10⁻⁶ s.
    Microsecond,
    /// 10⁻³ s.
    Millisecond,
    /// The SI second.
    Second,
    /// 60 s.
    Minute,
    /// 3600 s.
    Hour,
    /// 86 400 s. A nominal day, not a rotation of the Earth.
    Day,
    /// 365.25 days of 86 400 s — the IAU Julian year, exact by definition.
    JulianYear,
    /// 10³ Julian years, written `ka` in the Quaternary literature.
    Kiloyear,
    /// 10⁶ Julian years, written `Ma` on every geological chart.
    Megayear,
    /// 10⁹ Julian years, written `Ga` or `Gyr`.
    Gigayear,
}

impl DeepUnit {
    /// Every unit, ordered from smallest to largest.
    pub const ALL: &'static [Self] = &[
        Self::PlanckTime,
        Self::Yoctosecond,
        Self::Zeptosecond,
        Self::Attosecond,
        Self::Femtosecond,
        Self::Picosecond,
        Self::Nanosecond,
        Self::Microsecond,
        Self::Millisecond,
        Self::Second,
        Self::Minute,
        Self::Hour,
        Self::Day,
        Self::JulianYear,
        Self::Kiloyear,
        Self::Megayear,
        Self::Gigayear,
    ];

    /// How many seconds one of these is, as a central value.
    ///
    /// For every unit but [`DeepUnit::PlanckTime`] this factor is exact by
    /// definition. For the Planck time it is the CODATA 2022 central value,
    /// and [`DeepUnit::seconds_uncertain`] is the one that carries its error
    /// bar.
    #[must_use]
    pub fn seconds(self) -> f64 {
        match self {
            Self::PlanckTime => constants::PLANCK_TIME.value,
            Self::Yoctosecond => 1e-24,
            Self::Zeptosecond => 1e-21,
            Self::Attosecond => 1e-18,
            Self::Femtosecond => 1e-15,
            Self::Picosecond => 1e-12,
            Self::Nanosecond => 1e-9,
            Self::Microsecond => 1e-6,
            Self::Millisecond => 1e-3,
            Self::Second => 1.0,
            Self::Minute => 60.0,
            Self::Hour => 3600.0,
            Self::Day => 86_400.0,
            Self::JulianYear => constants::JULIAN_YEAR_SECONDS,
            Self::Kiloyear => constants::JULIAN_YEAR_SECONDS * 1e3,
            Self::Megayear => constants::JULIAN_YEAR_SECONDS * 1e6,
            Self::Gigayear => constants::JULIAN_YEAR_SECONDS * 1e9,
        }
    }

    /// The same scale factor with its uncertainty, which is zero for every
    /// unit except the Planck time.
    ///
    /// # Errors
    ///
    /// Returns [`DeepTimeError::NotFinite`] if the stored factor is not
    /// finite, which it always is.
    pub fn seconds_uncertain(self) -> DeepTimeResult<Uncertain> {
        match self {
            Self::PlanckTime => constants::PLANCK_TIME.uncertain(),
            other => Ok(Uncertain::exact(other.seconds())?),
        }
    }

    /// Whether the conversion to seconds is exact by definition.
    #[must_use]
    pub fn is_defined(self) -> bool {
        self != Self::PlanckTime
    }

    /// The symbol the literature writes this unit with.
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::PlanckTime => "t_P",
            Self::Yoctosecond => "ys",
            Self::Zeptosecond => "zs",
            Self::Attosecond => "as",
            Self::Femtosecond => "fs",
            Self::Picosecond => "ps",
            Self::Nanosecond => "ns",
            Self::Microsecond => "us",
            Self::Millisecond => "ms",
            Self::Second => "s",
            Self::Minute => "min",
            Self::Hour => "h",
            Self::Day => "d",
            Self::JulianYear => "a",
            Self::Kiloyear => "ka",
            Self::Megayear => "Ma",
            Self::Gigayear => "Ga",
        }
    }
}

/// A span of time carried as a magnitude in seconds, with its uncertainty and
/// its figure count.
///
/// ```
/// use hc_deep_time::DeepTime;
///
/// // Planck 2018: the universe is 13.787 +/- 0.020 Gyr old.
/// let age = DeepTime::from_gigayears(13.787, 0.020).unwrap();
/// let planck = DeepTime::from_planck_times(1.0, 0.0).unwrap();
/// let decades = age.orders_of_magnitude_between(planck).unwrap();
/// assert!((decades - 60.9).abs() < 0.1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeepTime {
    seconds: Uncertain,
    figures: u8,
}

impl DeepTime {
    /// Build a magnitude from seconds and an explicit figure count.
    ///
    /// # Errors
    ///
    /// Returns [`DeepTimeError::NotFinite`] for a non-finite value and
    /// [`DeepTimeError::Uncertainty`] wrapping
    /// [`hc_uncertainty::UncertaintyError::InvalidSignificantFigures`] when
    /// `figures` is zero or above [`hc_uncertainty::MAX_FIGURES`].
    pub fn new(seconds: Uncertain, figures: u8) -> DeepTimeResult<Self> {
        // Constructing a Significant validates the figure count once, here,
        // so that every later accessor can assume it is in range.
        let _ = Significant::new(seconds.value, figures)?;
        Ok(Self { seconds, figures })
    }

    /// Build a magnitude from seconds, letting the error bar fix the figure
    /// count.
    ///
    /// The rule is the Particle Data Group's: the uncertainty is quoted to two
    /// digits and fixes the last significant place of the value. An exact
    /// input claims every digit `f64` can carry.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::new`].
    pub fn from_uncertain_seconds(seconds: Uncertain) -> DeepTimeResult<Self> {
        let figures = seconds.to_significant()?.figures();
        Self::new(seconds, figures)
    }

    /// Build an exactly known magnitude in seconds.
    ///
    /// Use this for definitions and conventions, never for measurements.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::new`].
    pub fn from_exact_seconds(seconds: f64) -> DeepTimeResult<Self> {
        Self::new(Uncertain::exact(seconds)?, MAX_FIGURES)
    }

    /// Build a magnitude from a value that already knows its own precision.
    ///
    /// The result is exact — a [`Significant`] carries a digit count, not an
    /// error bar, and inventing a `σ` from the digit count would be exactly
    /// the fabrication this crate exists to prevent.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::new`].
    pub fn from_significant_seconds(seconds: Significant) -> DeepTimeResult<Self> {
        Self::new(Uncertain::exact(seconds.value())?, seconds.figures())
    }

    /// The magnitude in seconds, with its standard uncertainty.
    #[must_use]
    pub const fn seconds(self) -> Uncertain {
        self.seconds
    }

    /// The central value in seconds, with no error bar attached.
    #[must_use]
    pub const fn central_seconds(self) -> f64 {
        self.seconds.value
    }

    /// The standard uncertainty in seconds.
    #[must_use]
    pub const fn std_dev(self) -> f64 {
        self.seconds.std_dev
    }

    /// How many digits of the central value are claimed.
    #[must_use]
    pub const fn figures(self) -> u8 {
        self.figures
    }

    /// Whether the magnitude carries no uncertainty.
    #[must_use]
    pub fn is_exact(self) -> bool {
        self.seconds.is_exact()
    }

    /// The central value at the precision it actually claims.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::new`]; cannot fail for a value built through the public
    /// constructors.
    pub fn significant(self) -> DeepTimeResult<Significant> {
        Ok(Significant::new(self.seconds.value, self.figures)?)
    }

    /// The same magnitude with a different, explicitly stated figure count.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::new`].
    pub fn with_figures(self, figures: u8) -> DeepTimeResult<Self> {
        Self::new(self.seconds, figures)
    }

    /// Build a magnitude from a value expressed in `unit`.
    ///
    /// For every unit but the Planck time the scale factor is exact, so the
    /// conversion rescales `σ` and loses nothing. For the Planck time the
    /// factor is itself a measurement, and its 1.1×10⁻⁵ relative uncertainty
    /// is propagated in — which means a round trip through
    /// [`DeepTime::in_unit`] returns the same central value but a wider error
    /// bar, because the two occurrences of the constant are treated as
    /// independent. That is the honest answer, not a defect.
    ///
    /// # Errors
    ///
    /// Returns [`DeepTimeError::NotFinite`] when the product overflows `f64`.
    pub fn from_unit(value: Uncertain, unit: DeepUnit) -> DeepTimeResult<Self> {
        let seconds = if unit.is_defined() {
            value.scaled(unit.seconds())?
        } else {
            value.checked_mul(unit.seconds_uncertain()?)?
        };
        let figures = value.to_significant()?.figures();
        Self::new(seconds, figures)
    }

    /// The magnitude expressed in `unit`.
    ///
    /// # Errors
    ///
    /// Returns [`DeepTimeError::NotFinite`] when the quotient overflows or
    /// underflows `f64`.
    pub fn in_unit(self, unit: DeepUnit) -> DeepTimeResult<Uncertain> {
        if unit.is_defined() {
            Ok(self.seconds.scaled(1.0 / unit.seconds())?)
        } else {
            Ok(self.seconds.checked_div(unit.seconds_uncertain()?)?)
        }
    }

    /// An exactly known quantity of `unit`.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::from_unit`].
    pub fn exact_in(value: f64, unit: DeepUnit) -> DeepTimeResult<Self> {
        Self::from_unit(Uncertain::exact(value)?, unit)
    }

    /// Convert to the exact [`hc_core::Duration`] representation.
    ///
    /// **This conversion is lossy by construction.** It keeps the central
    /// value and drops the error bar and the figure count entirely, because
    /// `Duration` has nowhere to put them: a `Duration` asserts that the span
    /// is exactly what it says, and a `DeepTime` almost never is. It also
    /// truncates below one attosecond, which silently zeroes anything shorter.
    /// Use it at the boundary where a deep-time value has to enter the exact
    /// civil arithmetic, and not before.
    ///
    /// # Errors
    ///
    /// Returns [`DeepTimeError::OutOfDurationRange`] when the value does not
    /// fit the `i128` second count, and [`DeepTimeError::NotFinite`] for a
    /// non-finite central value.
    pub fn to_duration(self) -> DeepTimeResult<Duration> {
        Ok(Duration::from_secs_f64(self.seconds.value)?)
    }

    /// Read an exact duration as a deep-time magnitude.
    ///
    /// The result is exact in the uncertainty sense — a `Duration` really is
    /// the span it says — but the value passes through `f64` on the way, so a
    /// span longer than about 2⁵³ seconds (285 million years) keeps only the
    /// leading 15 to 17 digits. The figure count is set to
    /// [`hc_uncertainty::MAX_FIGURES`] to reflect that, and no further.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::new`].
    pub fn from_duration(duration: Duration) -> DeepTimeResult<Self> {
        Self::from_exact_seconds(duration.as_secs_f64())
    }

    /// `log10` of the magnitude in seconds, with the uncertainty propagated.
    ///
    /// `σ_log = σ / (x ln 10)`, the delta method applied to `log10`. A
    /// relative uncertainty of 1% becomes ±0.0043 in the exponent, which is
    /// why logarithmic error bars look so much smaller than linear ones and
    /// why they must not be compared with linear ones.
    ///
    /// # Errors
    ///
    /// Returns [`DeepTimeError::OutOfDomain`] for a non-positive central
    /// value, where the logarithm does not exist.
    pub fn log10_seconds(self) -> DeepTimeResult<Uncertain> {
        if self.seconds.value <= 0.0 {
            return Err(DeepTimeError::OutOfDomain);
        }
        Ok(self.seconds.ln()?.scaled(1.0 / LN_10)?)
    }

    /// `log10(self / other)`, with both uncertainties propagated.
    ///
    /// This is the comparison that means something across sixty decades. The
    /// two operands are treated as independent, which they are whenever they
    /// come from different measurements; comparing a value with itself will
    /// therefore report a non-zero `σ` around a central zero.
    ///
    /// # Errors
    ///
    /// Returns [`DeepTimeError::OutOfDomain`] when either central value is
    /// non-positive.
    pub fn log10_ratio(self, other: Self) -> DeepTimeResult<Uncertain> {
        Ok(self.log10_seconds()?.checked_sub(other.log10_seconds()?)?)
    }

    /// How many decades separate the two magnitudes, as a plain number.
    ///
    /// The absolute value of the central `log10` ratio. Use
    /// [`DeepTime::log10_ratio`] when the error bar on the comparison matters.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::log10_ratio`].
    pub fn orders_of_magnitude_between(self, other: Self) -> DeepTimeResult<f64> {
        Ok(math::abs(self.log10_ratio(other)?.value))
    }

    /// The dimensionless ratio `self / other`, with error propagation.
    ///
    /// # Errors
    ///
    /// Returns [`DeepTimeError::Uncertainty`] wrapping
    /// [`hc_uncertainty::UncertaintyError::DivideByZero`] for a zero divisor.
    pub fn ratio(self, other: Self) -> DeepTimeResult<Uncertain> {
        Ok(self.seconds.checked_div(other.seconds)?)
    }

    /// Sum of two independent magnitudes.
    ///
    /// The figure count follows the laboratory addition rule: the result is
    /// significant only down to the coarser of the two last significant
    /// places, so adding a second to a gigayear returns a gigayear.
    ///
    /// # Errors
    ///
    /// Returns [`DeepTimeError::NotFinite`] on overflow.
    pub fn checked_add(self, other: Self) -> DeepTimeResult<Self> {
        let seconds = self.seconds.checked_add(other.seconds)?;
        let figures = self
            .significant()?
            .checked_add(other.significant()?)?
            .figures();
        Self::new(seconds, figures)
    }

    /// Difference of two independent magnitudes.
    ///
    /// # Errors
    ///
    /// Returns [`DeepTimeError::NotFinite`] on overflow.
    pub fn checked_sub(self, other: Self) -> DeepTimeResult<Self> {
        let seconds = self.seconds.checked_sub(other.seconds)?;
        let figures = self
            .significant()?
            .checked_sub(other.significant()?)?
            .figures();
        Self::new(seconds, figures)
    }

    /// Multiply by an exactly known factor, rescaling `σ` with the value.
    ///
    /// # Errors
    ///
    /// Returns [`DeepTimeError::NotFinite`] on overflow.
    pub fn scaled(self, factor: f64) -> DeepTimeResult<Self> {
        Self::new(self.seconds.scaled(factor)?, self.figures)
    }

    /// Whether the two magnitudes' 1σ bars meet.
    #[must_use]
    pub fn overlaps(self, other: Self) -> bool {
        self.seconds.overlaps(other.seconds)
    }

    /// Order two magnitudes by central value alone.
    ///
    /// Deliberately not a [`PartialOrd`] implementation: two magnitudes with
    /// overlapping error bars are not really ordered, and an operator that
    /// looked total would hide that. Callers who need a sort use this and say
    /// in their own code that they are sorting on central values.
    #[must_use]
    pub fn central_cmp(self, other: Self) -> Ordering {
        self.seconds.value.total_cmp(&other.seconds.value)
    }

    /// Wrap the magnitude for logarithmic rendering.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::log10_seconds`].
    pub fn logarithmic(self) -> DeepTimeResult<LogMagnitude> {
        Ok(LogMagnitude {
            exponent: self.log10_seconds()?,
        })
    }
}

/// Unit-named constructors, one per unit the literature quotes.
///
/// Each takes a central value and a 1σ standard uncertainty *in that unit*.
/// Pass `0.0` for the uncertainty only when the value really is a definition.
impl DeepTime {
    /// A count of Planck times. The CODATA uncertainty of the unit itself is
    /// propagated in; see [`DeepTime::from_unit`].
    ///
    /// # Errors
    ///
    /// See [`DeepTime::from_unit`].
    pub fn from_planck_times(value: f64, std_dev: f64) -> DeepTimeResult<Self> {
        Self::from_unit(Uncertain::new(value, std_dev)?, DeepUnit::PlanckTime)
    }

    /// Yoctoseconds, 10⁻²⁴ s.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::from_unit`].
    pub fn from_yoctoseconds(value: f64, std_dev: f64) -> DeepTimeResult<Self> {
        Self::from_unit(Uncertain::new(value, std_dev)?, DeepUnit::Yoctosecond)
    }

    /// Zeptoseconds, 10⁻²¹ s.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::from_unit`].
    pub fn from_zeptoseconds(value: f64, std_dev: f64) -> DeepTimeResult<Self> {
        Self::from_unit(Uncertain::new(value, std_dev)?, DeepUnit::Zeptosecond)
    }

    /// Attoseconds, 10⁻¹⁸ s.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::from_unit`].
    pub fn from_attoseconds(value: f64, std_dev: f64) -> DeepTimeResult<Self> {
        Self::from_unit(Uncertain::new(value, std_dev)?, DeepUnit::Attosecond)
    }

    /// Femtoseconds, 10⁻¹⁵ s.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::from_unit`].
    pub fn from_femtoseconds(value: f64, std_dev: f64) -> DeepTimeResult<Self> {
        Self::from_unit(Uncertain::new(value, std_dev)?, DeepUnit::Femtosecond)
    }

    /// Picoseconds, 10⁻¹² s.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::from_unit`].
    pub fn from_picoseconds(value: f64, std_dev: f64) -> DeepTimeResult<Self> {
        Self::from_unit(Uncertain::new(value, std_dev)?, DeepUnit::Picosecond)
    }

    /// Nanoseconds, 10⁻⁹ s.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::from_unit`].
    pub fn from_nanoseconds(value: f64, std_dev: f64) -> DeepTimeResult<Self> {
        Self::from_unit(Uncertain::new(value, std_dev)?, DeepUnit::Nanosecond)
    }

    /// Microseconds, 10⁻⁶ s.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::from_unit`].
    pub fn from_microseconds(value: f64, std_dev: f64) -> DeepTimeResult<Self> {
        Self::from_unit(Uncertain::new(value, std_dev)?, DeepUnit::Microsecond)
    }

    /// Milliseconds, 10⁻³ s.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::from_unit`].
    pub fn from_milliseconds(value: f64, std_dev: f64) -> DeepTimeResult<Self> {
        Self::from_unit(Uncertain::new(value, std_dev)?, DeepUnit::Millisecond)
    }

    /// SI seconds.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::from_unit`].
    pub fn from_seconds(value: f64, std_dev: f64) -> DeepTimeResult<Self> {
        Self::from_unit(Uncertain::new(value, std_dev)?, DeepUnit::Second)
    }

    /// Minutes of 60 s.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::from_unit`].
    pub fn from_minutes(value: f64, std_dev: f64) -> DeepTimeResult<Self> {
        Self::from_unit(Uncertain::new(value, std_dev)?, DeepUnit::Minute)
    }

    /// Hours of 3600 s.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::from_unit`].
    pub fn from_hours(value: f64, std_dev: f64) -> DeepTimeResult<Self> {
        Self::from_unit(Uncertain::new(value, std_dev)?, DeepUnit::Hour)
    }

    /// Nominal days of 86 400 s.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::from_unit`].
    pub fn from_days(value: f64, std_dev: f64) -> DeepTimeResult<Self> {
        Self::from_unit(Uncertain::new(value, std_dev)?, DeepUnit::Day)
    }

    /// Julian years of 365.25 days.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::from_unit`].
    pub fn from_julian_years(value: f64, std_dev: f64) -> DeepTimeResult<Self> {
        Self::from_unit(Uncertain::new(value, std_dev)?, DeepUnit::JulianYear)
    }

    /// Kiloyears (`ka`).
    ///
    /// # Errors
    ///
    /// See [`DeepTime::from_unit`].
    pub fn from_kiloyears(value: f64, std_dev: f64) -> DeepTimeResult<Self> {
        Self::from_unit(Uncertain::new(value, std_dev)?, DeepUnit::Kiloyear)
    }

    /// Megayears (`Ma`), the unit of every geological chart.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::from_unit`].
    pub fn from_megayears(value: f64, std_dev: f64) -> DeepTimeResult<Self> {
        Self::from_unit(Uncertain::new(value, std_dev)?, DeepUnit::Megayear)
    }

    /// Gigayears (`Ga`), the unit of cosmology.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::from_unit`].
    pub fn from_gigayears(value: f64, std_dev: f64) -> DeepTimeResult<Self> {
        Self::from_unit(Uncertain::new(value, std_dev)?, DeepUnit::Gigayear)
    }
}

impl fmt::Display for DeepTime {
    /// Render at the true precision: `1.38e10 s` or `1.3787e10 s ± 2.0e7`.
    ///
    /// The central value is printed through [`Significant`], so it never shows
    /// a digit the source did not claim, and the uncertainty is printed to two
    /// digits, which is the Particle Data Group's table convention.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match Significant::new(self.seconds.value, self.figures) {
            Ok(value) => write!(f, "{value} s")?,
            Err(_) => write!(f, "{} s", self.seconds.value)?,
        }
        if !self.seconds.is_exact() {
            match Significant::new(self.seconds.std_dev, 2) {
                Ok(sigma) => write!(f, " ± {sigma}")?,
                Err(_) => write!(f, " ± {}", self.seconds.std_dev)?,
            }
        }
        Ok(())
    }
}

/// A [`DeepTime`] rendered as a power of ten of seconds.
///
/// Produced by [`DeepTime::logarithmic`]. Displays as `10^17.638 s`, or
/// `10^(17.638 ± 0.002) s` when the magnitude is not exact. Three decimal
/// places in the exponent is a millidex, the resolution at which two
/// astronomical magnitudes are conventionally distinguished; more would be
/// meaningless for values whose linear error bars run to per cent.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LogMagnitude {
    exponent: Uncertain,
}

impl LogMagnitude {
    /// The base-ten exponent, with its propagated uncertainty.
    #[must_use]
    pub const fn exponent(self) -> Uncertain {
        self.exponent
    }
}

impl fmt::Display for LogMagnitude {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.exponent.is_exact() {
            write!(f, "10^{:.3} s", self.exponent.value)
        } else {
            write!(
                f,
                "10^({:.3} ± {:.3}) s",
                self.exponent.value, self.exponent.std_dev
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    use alloc::string::ToString as _;

    /// Relative closeness, the only comparison that means anything here.
    fn close(a: f64, b: f64, relative: f64) -> bool {
        math::abs(a - b) <= math::abs(b) * relative
    }

    #[test]
    fn a_gigayear_is_the_iau_julian_year_times_a_billion() {
        assert!(close(DeepUnit::Gigayear.seconds(), 3.155_76e16, 1e-12));
    }

    #[test]
    fn every_unit_is_larger_than_the_one_below_it() {
        for pair in DeepUnit::ALL.windows(2) {
            assert!(
                pair[0].seconds() < pair[1].seconds(),
                "{:?} is not smaller than {:?}",
                pair[0],
                pair[1]
            );
        }
    }

    #[test]
    fn only_the_planck_time_is_a_measured_unit() {
        for unit in DeepUnit::ALL {
            let factor = unit.seconds_uncertain().unwrap();
            assert_eq!(
                factor.is_exact(),
                unit.is_defined(),
                "{unit:?} disagrees with itself about being defined"
            );
        }
    }

    #[test]
    fn every_unit_has_a_symbol() {
        for unit in DeepUnit::ALL {
            assert!(!unit.symbol().is_empty(), "{unit:?} has no symbol");
        }
    }

    #[test]
    fn round_trips_through_every_defined_unit_are_exact() {
        for unit in DeepUnit::ALL.iter().filter(|unit| unit.is_defined()) {
            for value in [1.0, 3.5, 1234.0, 1e-3, 9.87654e6] {
                let deep = DeepTime::exact_in(value, *unit).unwrap();
                let back = deep.in_unit(*unit).unwrap();
                assert!(
                    close(back.value, value, 1e-12),
                    "{unit:?}: {value} came back as {}",
                    back.value
                );
                assert_eq!(back.std_dev, 0.0);
            }
        }
    }

    #[test]
    fn round_trips_through_every_unit_preserve_the_central_value() {
        // The Planck time is included here: its factor is uncertain, so the
        // error bar widens, but the central value must survive untouched.
        for unit in DeepUnit::ALL {
            let deep = DeepTime::from_unit(Uncertain::new(2.5, 0.1).unwrap(), *unit).unwrap();
            let back = deep.in_unit(*unit).unwrap();
            assert!(close(back.value, 2.5, 1e-12), "{unit:?} lost its value");
        }
    }

    #[test]
    fn a_round_trip_through_the_planck_time_widens_the_error_bar() {
        // Documented behaviour, not an accident: the two occurrences of the
        // CODATA constant are treated as independent measurements.
        let deep = DeepTime::from_planck_times(1.0, 0.0).unwrap();
        let back = deep.in_unit(DeepUnit::PlanckTime).unwrap();
        assert!(close(back.value, 1.0, 1e-12));
        assert!(back.std_dev > 0.0, "the constant's error bar vanished");
        assert!(back.std_dev < 1e-4, "and it should still be tiny");
    }

    #[test]
    fn unit_named_constructors_agree_with_the_general_one() {
        let pairs: [(DeepTime, DeepUnit); 17] = [
            (
                DeepTime::from_planck_times(2.0, 0.0).unwrap(),
                DeepUnit::PlanckTime,
            ),
            (
                DeepTime::from_yoctoseconds(2.0, 0.0).unwrap(),
                DeepUnit::Yoctosecond,
            ),
            (
                DeepTime::from_zeptoseconds(2.0, 0.0).unwrap(),
                DeepUnit::Zeptosecond,
            ),
            (
                DeepTime::from_attoseconds(2.0, 0.0).unwrap(),
                DeepUnit::Attosecond,
            ),
            (
                DeepTime::from_femtoseconds(2.0, 0.0).unwrap(),
                DeepUnit::Femtosecond,
            ),
            (
                DeepTime::from_picoseconds(2.0, 0.0).unwrap(),
                DeepUnit::Picosecond,
            ),
            (
                DeepTime::from_nanoseconds(2.0, 0.0).unwrap(),
                DeepUnit::Nanosecond,
            ),
            (
                DeepTime::from_microseconds(2.0, 0.0).unwrap(),
                DeepUnit::Microsecond,
            ),
            (
                DeepTime::from_milliseconds(2.0, 0.0).unwrap(),
                DeepUnit::Millisecond,
            ),
            (DeepTime::from_seconds(2.0, 0.0).unwrap(), DeepUnit::Second),
            (DeepTime::from_minutes(2.0, 0.0).unwrap(), DeepUnit::Minute),
            (DeepTime::from_hours(2.0, 0.0).unwrap(), DeepUnit::Hour),
            (DeepTime::from_days(2.0, 0.0).unwrap(), DeepUnit::Day),
            (
                DeepTime::from_julian_years(2.0, 0.0).unwrap(),
                DeepUnit::JulianYear,
            ),
            (
                DeepTime::from_kiloyears(2.0, 0.0).unwrap(),
                DeepUnit::Kiloyear,
            ),
            (
                DeepTime::from_megayears(2.0, 0.0).unwrap(),
                DeepUnit::Megayear,
            ),
            (
                DeepTime::from_gigayears(2.0, 0.0).unwrap(),
                DeepUnit::Gigayear,
            ),
        ];
        assert_eq!(pairs.len(), DeepUnit::ALL.len());
        for (built, unit) in pairs {
            assert!(close(built.central_seconds(), 2.0 * unit.seconds(), 1e-12));
        }
    }

    #[test]
    fn the_age_of_the_universe_is_about_eight_times_ten_to_the_sixty_planck_times() {
        // Planck 2018 VI, Table 2: 13.787 +/- 0.020 Gyr. CODATA 2022 t_P.
        let age = DeepTime::from_gigayears(13.787, 0.020).unwrap();
        let planck = DeepTime::from_planck_times(1.0, 0.0).unwrap();
        let ratio = age.ratio(planck).unwrap();
        assert!(
            close(ratio.value, 8.07e60, 0.01),
            "ratio came out as {}",
            ratio.value
        );
        assert!((ratio.value / 1e60 - 8.0).abs() < 0.5);
    }

    #[test]
    fn sixty_one_decades_separate_the_planck_time_from_the_present() {
        let age = DeepTime::from_gigayears(13.787, 0.020).unwrap();
        let planck = DeepTime::from_planck_times(1.0, 0.0).unwrap();
        let decades = age.orders_of_magnitude_between(planck).unwrap();
        assert!((decades - 60.907).abs() < 0.01, "decades = {decades}");
    }

    #[test]
    fn the_logarithmic_ratio_is_antisymmetric() {
        let a = DeepTime::from_seconds(1e10, 0.0).unwrap();
        let b = DeepTime::from_seconds(1e4, 0.0).unwrap();
        assert!(close(a.log10_ratio(b).unwrap().value, 6.0, 1e-12));
        assert!(close(b.log10_ratio(a).unwrap().value, -6.0, 1e-12));
        assert!(close(a.orders_of_magnitude_between(b).unwrap(), 6.0, 1e-12));
        assert!(close(b.orders_of_magnitude_between(a).unwrap(), 6.0, 1e-12));
    }

    #[test]
    fn a_one_per_cent_relative_error_is_four_millidex_in_the_exponent() {
        // sigma_log = sigma / (x ln 10) = 0.01 / 2.302585
        let value = DeepTime::from_seconds(100.0, 1.0).unwrap();
        let log = value.log10_seconds().unwrap();
        assert!(close(log.value, 2.0, 1e-12));
        assert!(close(log.std_dev, 0.004_343, 1e-3), "{}", log.std_dev);
    }

    #[test]
    fn the_logarithm_of_a_non_positive_span_is_refused() {
        let zero = DeepTime::from_seconds(0.0, 0.0).unwrap();
        assert_eq!(zero.log10_seconds(), Err(DeepTimeError::OutOfDomain));
        let negative = DeepTime::from_seconds(-1.0, 0.0).unwrap();
        assert_eq!(negative.log10_seconds(), Err(DeepTimeError::OutOfDomain));
    }

    #[test]
    fn a_duration_round_trips_through_deep_time() {
        for seconds in [0i128, 1, 86_400, 1_000_000_000, -3600] {
            let duration = Duration::from_secs(seconds);
            let deep = DeepTime::from_duration(duration).unwrap();
            let back = deep.to_duration().unwrap();
            assert_eq!(back.whole_seconds(), seconds);
        }
    }

    #[test]
    fn the_planck_time_has_no_exact_duration_but_does_not_error() {
        // Duration truncates below an attosecond, so the conversion succeeds
        // and returns zero. That is the loss the doc comment warns about.
        let planck = DeepTime::from_planck_times(1.0, 0.0).unwrap();
        assert_eq!(planck.to_duration().unwrap(), Duration::ZERO);
    }

    #[test]
    fn a_span_beyond_the_i128_second_count_is_refused() {
        let absurd = DeepTime::from_gigayears(1e30, 0.0).unwrap();
        assert_eq!(absurd.to_duration(), Err(DeepTimeError::OutOfDurationRange));
    }

    #[test]
    fn the_age_of_the_universe_does_fit_an_exact_duration() {
        let age = DeepTime::from_gigayears(13.787, 0.020).unwrap();
        let duration = age.to_duration().unwrap();
        assert!(close(duration.as_secs_f64(), 4.3508e17, 1e-4));
    }

    #[test]
    fn addition_follows_the_laboratory_figure_rule() {
        let gigayear = DeepTime::from_gigayears(1.0, 0.0)
            .unwrap()
            .with_figures(3)
            .unwrap();
        let second = DeepTime::from_seconds(1.0, 0.0)
            .unwrap()
            .with_figures(1)
            .unwrap();
        let sum = gigayear.checked_add(second).unwrap();
        assert_eq!(sum.figures(), 3, "a second cannot refine a gigayear");
    }

    #[test]
    fn subtraction_propagates_the_error_bars_in_quadrature() {
        let a = DeepTime::from_seconds(100.0, 3.0).unwrap();
        let b = DeepTime::from_seconds(40.0, 4.0).unwrap();
        let difference = a.checked_sub(b).unwrap();
        assert!(close(difference.central_seconds(), 60.0, 1e-12));
        assert!(
            close(difference.std_dev(), 5.0, 1e-9),
            "3-4-5 in quadrature"
        );
    }

    #[test]
    fn scaling_rescales_the_error_bar_with_the_value() {
        let value = DeepTime::from_seconds(10.0, 2.0).unwrap();
        let doubled = value.scaled(2.0).unwrap();
        assert!(close(doubled.central_seconds(), 20.0, 1e-12));
        assert!(close(doubled.std_dev(), 4.0, 1e-12));
    }

    #[test]
    fn overlapping_error_bars_are_reported_as_such() {
        let a = DeepTime::from_gigayears(13.787, 0.020).unwrap();
        let b = DeepTime::from_gigayears(13.797, 0.023).unwrap();
        assert!(a.overlaps(b), "the two Planck likelihoods agree");
        let c = DeepTime::from_gigayears(11.0, 0.1).unwrap();
        assert!(!a.overlaps(c));
    }

    #[test]
    fn central_comparison_orders_magnitudes() {
        let small = DeepTime::from_planck_times(1.0, 0.0).unwrap();
        let large = DeepTime::from_gigayears(13.787, 0.020).unwrap();
        assert_eq!(small.central_cmp(large), Ordering::Less);
        assert_eq!(large.central_cmp(small), Ordering::Greater);
        assert_eq!(small.central_cmp(small), Ordering::Equal);
    }

    #[test]
    fn an_uncertain_value_takes_its_figure_count_from_its_error_bar() {
        // 13.787 +/- 0.020: sigma's leading digit is at 1e-2, so the value is
        // significant to 1e-3 and claims five figures.
        let age = DeepTime::from_gigayears(13.787, 0.020).unwrap();
        assert_eq!(age.figures(), 5);
    }

    #[test]
    fn a_three_figure_age_never_renders_as_eleven_digits() {
        #[cfg(feature = "alloc")]
        {
            let age = DeepTime::from_gigayears(13.8, 0.0)
                .unwrap()
                .with_figures(3)
                .unwrap();
            let rendered = age.to_string();
            assert_eq!(rendered, "4.35e17 s");
            assert!(!rendered.contains("4354948"));
        }
    }

    #[test]
    fn an_uncertain_magnitude_renders_its_error_bar() {
        #[cfg(feature = "alloc")]
        {
            let age = DeepTime::from_gigayears(13.787, 0.020).unwrap();
            let rendered = age.to_string();
            assert!(rendered.contains('±'), "{rendered}");
            assert!(rendered.starts_with("4.3508e17 s"), "{rendered}");
        }
    }

    #[test]
    fn logarithmic_rendering_shows_the_exponent() {
        #[cfg(feature = "alloc")]
        {
            let age = DeepTime::from_gigayears(13.787, 0.020).unwrap();
            let rendered = age.logarithmic().unwrap().to_string();
            assert!(rendered.starts_with("10^(17.639"), "{rendered}");
            let exact = DeepTime::from_seconds(1000.0, 0.0).unwrap();
            assert_eq!(exact.logarithmic().unwrap().to_string(), "10^3.000 s");
        }
        let age = DeepTime::from_gigayears(13.787, 0.020).unwrap();
        assert!(close(
            age.logarithmic().unwrap().exponent().value,
            17.6385,
            1e-4
        ));
    }

    #[test]
    fn a_figure_count_outside_the_representable_range_is_refused() {
        let seconds = Uncertain::exact(1.0).unwrap();
        assert!(DeepTime::new(seconds, 0).is_err());
        assert!(DeepTime::new(seconds, MAX_FIGURES + 1).is_err());
        assert!(DeepTime::new(seconds, MAX_FIGURES).is_ok());
    }

    #[test]
    fn a_significant_value_becomes_an_exact_magnitude_with_its_digits() {
        let significant = Significant::new(13.8e9, 3).unwrap();
        let deep = DeepTime::from_significant_seconds(significant).unwrap();
        assert_eq!(deep.figures(), 3);
        assert!(deep.is_exact(), "digits are not an error bar");
    }

    #[test]
    fn an_uncertain_seconds_value_keeps_its_own_figure_count() {
        let value = Uncertain::new(1234.5, 12.0).unwrap();
        let deep = DeepTime::from_uncertain_seconds(value).unwrap();
        assert_eq!(deep.figures(), 4, "the error bar fixes the last place");
        assert_eq!(deep.seconds(), value);
    }

    #[test]
    fn a_negative_standard_deviation_is_refused_at_every_constructor() {
        assert!(DeepTime::from_seconds(1.0, -1.0).is_err());
        assert!(DeepTime::from_gigayears(1.0, -1e-9).is_err());
        assert!(DeepTime::from_planck_times(1.0, -1.0).is_err());
    }

    #[test]
    fn a_non_finite_magnitude_is_refused() {
        assert_eq!(
            DeepTime::from_seconds(f64::NAN, 0.0),
            Err(DeepTimeError::NotFinite)
        );
        assert_eq!(
            DeepTime::from_seconds(f64::INFINITY, 0.0),
            Err(DeepTimeError::NotFinite)
        );
    }

    #[test]
    fn the_ratio_of_a_span_to_itself_is_one() {
        let value = DeepTime::from_megayears(66.0, 0.05).unwrap();
        let ratio = value.ratio(value).unwrap();
        assert!(close(ratio.value, 1.0, 1e-12));
    }

    #[test]
    fn dividing_by_a_zero_length_span_is_refused() {
        let value = DeepTime::from_seconds(1.0, 0.0).unwrap();
        let zero = DeepTime::from_seconds(0.0, 0.0).unwrap();
        assert!(value.ratio(zero).is_err());
    }
}
