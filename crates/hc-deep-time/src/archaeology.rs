//! Before Present, and the distinction the convention exists to make.
//!
//! "BP" means *before present*, and the present is **1950 CE** by convention,
//! fixed there because atmospheric nuclear testing from the early 1950s
//! flooded the atmosphere with bomb carbon and made later samples useless as a
//! baseline. The datum has nothing to do with the year anyone is reading in.
//!
//! # The distinction that gets botched
//!
//! A radiocarbon laboratory reports a **conventional radiocarbon age**, in
//! years BP. That number is not a count of calendar years. It is computed
//! from the measured ¹⁴C activity on three fixed conventions (Stuiver &
//! Polach, *Radiocarbon* 19, 355, 1977): the Libby half-life of 5568 years
//! rather than the measured 5730, a constant atmospheric ¹⁴C concentration,
//! and a δ¹³C normalisation. Atmospheric ¹⁴C has *not* been constant — it
//! varies with solar activity, the geomagnetic field and ocean circulation —
//! so a radiocarbon year is not a year.
//!
//! Turning a radiocarbon age into a calendar age is **calibration**, and it
//! needs a calibration curve: IntCal20 for the northern hemisphere
//! atmosphere, SHCal20 for the southern, Marine20 for marine reservoirs
//! (Reimer et al., *Radiocarbon* 62, 725, 2020). The result is written "cal
//! BP" and is usually a multi-modal probability distribution, not a Gaussian,
//! because the curve wiggles: several stretches of the first millennium BCE
//! map a single radiocarbon age onto three or four calendar ranges at once.
//!
//! **This crate does not calibrate.** The curves are large datasets with their
//! own release cadence, and approximating them would produce dates that look
//! authoritative and are wrong. What the crate does instead is make the
//! distinction unignorable: [`Bp`] carries a [`Calibration`] tag, and asking
//! an uncalibrated age for a calendar year returns
//! [`crate::DeepTimeError::CalibrationRequired`] rather than a plausible
//! number.
//!
//! # Three different "presents"
//!
//! | Datum | Used by | Offset from BP |
//! | --- | --- | --- |
//! | 1950 CE ("BP") | radiocarbon, archaeology | 0 |
//! | 2000 CE ("b2k") | Greenland ice cores, the Quaternary GSSPs | −50 years |
//! | "now" | casual writing | drifts one year per year |
//!
//! The Holocene's base is 11 700 b2k, which is 11 650 BP. Mixing the two
//! silently misplaces every Late Glacial date by fifty years, so
//! [`b2k_to_bp`] exists and is used by the table in this module.

use core::fmt;

use hc_uncertainty::Uncertain;

use crate::error::{DeepTimeError, DeepTimeResult};
use crate::magnitude::{DeepTime, DeepUnit};

/// The calendar year the BP scale counts back from: 1950 CE.
pub const BP_DATUM_YEAR: i32 = 1950;

/// The calendar year the ice-core "b2k" scale counts back from: 2000 CE.
pub const B2K_DATUM_YEAR: i32 = 2000;

/// The Libby half-life of ¹⁴C, 5568 years, used *by convention* in every
/// reported radiocarbon age.
///
/// It is not the best value — that is [`CAMBRIDGE_HALF_LIFE_YEARS`] — and it
/// has not been since 1962. It is kept because changing it would invalidate
/// every published age, and because calibration absorbs the difference
/// anyway.
pub const LIBBY_HALF_LIFE_YEARS: f64 = 5568.0;

/// The measured half-life of ¹⁴C, 5730 ± 40 years, agreed at Cambridge in
/// 1962 and never adopted for reporting.
pub const CAMBRIDGE_HALF_LIFE_YEARS: f64 = 5730.0;

/// Whether an age has been through a calibration curve.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Calibration {
    /// A conventional radiocarbon age, in radiocarbon years BP. Not a count
    /// of calendar years and not convertible to one without a curve.
    Uncalibrated,
    /// A calendar age in years before 1950 CE, whether it came from a
    /// calibration curve, dendrochronology, an ice core or a historical
    /// record.
    Calibrated,
}

impl Calibration {
    /// The suffix the literature writes after the number.
    #[must_use]
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Uncalibrated => "BP",
            Self::Calibrated => "cal BP",
        }
    }
}

/// An age before present, tagged with whether it is calendar years.
///
/// ```
/// use hc_deep_time::{Bp, DeepTimeError};
///
/// // A laboratory report: 3200 +/- 50 radiocarbon years BP.
/// let raw = Bp::uncalibrated(3200.0, 50.0).unwrap();
/// assert_eq!(raw.to_calendar_year(), Err(DeepTimeError::CalibrationRequired));
///
/// // What IntCal20 makes of it is roughly 3430 cal BP, and only then is it
/// // a calendar year.
/// let calibrated = Bp::calibrated(3430.0, 60.0).unwrap();
/// assert!((calibrated.to_calendar_year().unwrap().value - (-1480.0)).abs() < 1.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bp {
    years: Uncertain,
    calibration: Calibration,
}

impl Bp {
    /// A conventional radiocarbon age as the laboratory reported it.
    ///
    /// # Errors
    ///
    /// Returns [`DeepTimeError::NotFinite`] for non-finite inputs and
    /// [`DeepTimeError::Uncertainty`] for a negative standard deviation.
    pub fn uncalibrated(years: f64, std_dev: f64) -> DeepTimeResult<Self> {
        Ok(Self {
            years: Uncertain::new(years, std_dev)?,
            calibration: Calibration::Uncalibrated,
        })
    }

    /// A calendar age before 1950 CE.
    ///
    /// # Errors
    ///
    /// See [`Bp::uncalibrated`].
    pub fn calibrated(years: f64, std_dev: f64) -> DeepTimeResult<Self> {
        Ok(Self {
            years: Uncertain::new(years, std_dev)?,
            calibration: Calibration::Calibrated,
        })
    }

    /// The number itself, in whichever kind of year this age is measured in.
    #[must_use]
    pub const fn years(self) -> Uncertain {
        self.years
    }

    /// Whether the age has been through a calibration curve.
    #[must_use]
    pub const fn calibration(self) -> Calibration {
        self.calibration
    }

    /// Whether the age is in calendar years.
    #[must_use]
    pub fn is_calibrated(self) -> bool {
        self.calibration == Calibration::Calibrated
    }

    /// The calendar year, in astronomical numbering.
    ///
    /// Astronomical numbering has a year zero, which is 1 BCE, so 1950 BP is
    /// year 0 and 2000 BP is year −50, meaning 51 BCE. This is the numbering
    /// every astronomical and chronological computation uses and the one
    /// [`hc_calendar`](https://docs.rs/hc-calendar) expects.
    ///
    /// # Errors
    ///
    /// Returns [`DeepTimeError::CalibrationRequired`] for an uncalibrated
    /// radiocarbon age. A radiocarbon year is not a calendar year, and the
    /// conversion is a calibration curve, not a subtraction.
    pub fn to_calendar_year(self) -> DeepTimeResult<Uncertain> {
        if !self.is_calibrated() {
            return Err(DeepTimeError::CalibrationRequired);
        }
        Ok(self.years.negated()?.shifted(f64::from(BP_DATUM_YEAR))?)
    }

    /// An age from a calendar year in astronomical numbering.
    ///
    /// The result is [`Calibration::Calibrated`] by construction: a calendar
    /// year already is one.
    ///
    /// # Errors
    ///
    /// See [`Bp::uncalibrated`].
    pub fn from_calendar_year(year: f64, std_dev: f64) -> DeepTimeResult<Self> {
        Self::calibrated(f64::from(BP_DATUM_YEAR) - year, std_dev)
    }

    /// The age as a span of real time before 1950 CE.
    ///
    /// # Errors
    ///
    /// Returns [`DeepTimeError::CalibrationRequired`] for an uncalibrated
    /// radiocarbon age, for the same reason as [`Bp::to_calendar_year`]: a
    /// radiocarbon year is not an SI year and multiplying it by 31 557 600
    /// seconds would be inventing a duration.
    pub fn to_deep_time(self) -> DeepTimeResult<DeepTime> {
        if !self.is_calibrated() {
            return Err(DeepTimeError::CalibrationRequired);
        }
        DeepTime::from_unit(self.years, DeepUnit::JulianYear)
    }

    /// A calendar age from a span before present.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::in_unit`].
    pub fn from_deep_time(span: DeepTime) -> DeepTimeResult<Self> {
        let years = span.in_unit(DeepUnit::JulianYear)?;
        Self::calibrated(years.value, years.std_dev)
    }

    /// Restate a conventional radiocarbon age on the measured half-life.
    ///
    /// Multiplies by 5730/5568 ≈ 1.0291. **This is not calibration.** It only
    /// undoes one of the three conventions in a reported age, leaves the
    /// atmospheric variation untouched, and produces a number that is still
    /// not a calendar year — which is why the result stays
    /// [`Calibration::Uncalibrated`]. It is here because the two half-lives
    /// are a standing source of confusion, not because anyone should use it
    /// for dating.
    ///
    /// # Errors
    ///
    /// Returns [`DeepTimeError::CalibrationRequired`] if the age has already
    /// been calibrated, where the rescaling would be meaningless, and
    /// [`DeepTimeError::NotFinite`] on overflow.
    pub fn on_measured_half_life(self) -> DeepTimeResult<Self> {
        if self.is_calibrated() {
            return Err(DeepTimeError::CalibrationRequired);
        }
        let factor = CAMBRIDGE_HALF_LIFE_YEARS / LIBBY_HALF_LIFE_YEARS;
        Ok(Self {
            years: self.years.scaled(factor)?,
            calibration: Calibration::Uncalibrated,
        })
    }
}

impl fmt::Display for Bp {
    /// `3200 ± 50 BP`, or `3430 ± 60 cal BP` once calibrated.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.years.is_exact() {
            write!(f, "{} {}", self.years.value, self.calibration.suffix())
        } else {
            write!(
                f,
                "{} ± {} {}",
                self.years.value,
                self.years.std_dev,
                self.calibration.suffix()
            )
        }
    }
}

/// Convert an ice-core "b2k" age to years BP.
///
/// The Greenland ice-core chronologies and the Quaternary GSSPs that depend on
/// them count from 2000 CE, so they run fifty years ahead of the radiocarbon
/// datum. The Holocene's base is 11 700 b2k, which is 11 650 BP.
#[must_use]
pub fn b2k_to_bp(years_b2k: f64) -> f64 {
    years_b2k - f64::from(B2K_DATUM_YEAR - BP_DATUM_YEAR)
}

/// Convert years BP to the ice-core "b2k" scale.
#[must_use]
pub fn bp_to_b2k(years_bp: f64) -> f64 {
    years_bp + f64::from(B2K_DATUM_YEAR - BP_DATUM_YEAR)
}

/// A conventional archaeological period.
///
/// **Read the `region` field before using one of these.** Archaeological
/// periods are named after technologies, and a technology arrives at
/// different times in different places: the Bronze Age begins around 3300 BCE
/// in the Near East, 2200 BCE in Britain and never in most of the Americas.
/// The sequence in [`PERIODS`] is the Southwest Asian and European one,
/// because that is the sequence the Three-Age System was built on, and it is
/// wrong everywhere else.
///
/// The boundary uncertainties here are wider than any single site's, because
/// they are not measurements at all: they are the spread between the
/// conventional dates different handbooks use.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArchaeologicalPeriod {
    /// The period's conventional name.
    pub name: &'static str,
    /// What defines it, and what the boundary dates are really worth.
    pub description: &'static str,
    /// The region the sequence applies to.
    pub region: &'static str,
    /// Where the boundaries came from.
    pub source: &'static str,
    begins_bp: f64,
    begins_std_dev_bp: f64,
    ends_bp: f64,
    ends_std_dev_bp: f64,
}

impl ArchaeologicalPeriod {
    /// When the period began, in years before 1950 CE.
    ///
    /// # Errors
    ///
    /// See [`Bp::calibrated`]; cannot fail for the tabulated entries.
    pub fn begins(&self) -> DeepTimeResult<Bp> {
        Bp::calibrated(self.begins_bp, self.begins_std_dev_bp)
    }

    /// When it ended, in years before 1950 CE.
    ///
    /// # Errors
    ///
    /// See [`Bp::calibrated`]; cannot fail for the tabulated entries.
    pub fn ends(&self) -> DeepTimeResult<Bp> {
        Bp::calibrated(self.ends_bp, self.ends_std_dev_bp)
    }

    /// The older boundary's central value, in years BP.
    #[must_use]
    pub const fn begins_bp(&self) -> f64 {
        self.begins_bp
    }

    /// The younger boundary's central value, in years BP.
    #[must_use]
    pub const fn ends_bp(&self) -> f64 {
        self.ends_bp
    }

    /// Whether `bp` years before present falls in `[ends, begins)`.
    #[must_use]
    pub fn contains_bp(&self, bp: f64) -> bool {
        bp >= self.ends_bp && bp < self.begins_bp
    }
}

/// The conventional Southwest Asian and European sequence, youngest first.
///
/// Boundaries are round numbers on purpose. The Younger Dryas termination at
/// 11 700 b2k (11 650 BP) is the one hard date in the list, ratified as the
/// base of the Holocene from the NGRIP ice core. The other boundaries are
/// this library's choice of round numbers for divisions in common use; no
/// handbook was read for them, so no handbook is cited, and their error
/// bars are wide to say so. Where a named work fixes one side of a period,
/// its `source` names it.
pub const PERIODS: &[ArchaeologicalPeriod] = &[
    ArchaeologicalPeriod {
        name: "Modern period",
        description: "From about 1500 CE. A historian's division rather than an archaeological \
                      one; included so that the sequence reaches the present and a query for a \
                      recent date has an answer.",
        region: "Europe and Southwest Asia",
        source: "This library's round-number boundaries; no handbook read",
        begins_bp: 450.0,
        begins_std_dev_bp: 50.0,
        ends_bp: 0.0,
        ends_std_dev_bp: 0.0,
    },
    ArchaeologicalPeriod {
        name: "Middle Ages",
        description: "Roughly 550 to 1500 CE in Europe. Both boundaries are arguments rather \
                      than dates, and the fifty-year error bars understate how much they are \
                      argued about.",
        region: "Europe",
        source: "This library's round-number boundaries; no handbook read",
        begins_bp: 1400.0,
        begins_std_dev_bp: 100.0,
        ends_bp: 450.0,
        ends_std_dev_bp: 50.0,
    },
    ArchaeologicalPeriod {
        name: "Classical antiquity",
        description: "Roughly 550 BCE to 550 CE: the Greek and Roman Mediterranean. Dated by \
                      documents rather than by radiocarbon, which is why it is the only stretch \
                      of this table whose dates are better than its error bars suggest.",
        region: "Mediterranean",
        source: "This library's round-number boundaries; no handbook read",
        begins_bp: 2500.0,
        begins_std_dev_bp: 100.0,
        ends_bp: 1400.0,
        ends_std_dev_bp: 100.0,
    },
    ArchaeologicalPeriod {
        name: "Iron Age",
        description: "From the Late Bronze Age collapse around 1200 BCE. Iron working spread \
                      over centuries and the transition is a gradient, not a line.",
        region: "Southwest Asia and the eastern Mediterranean",
        source: "Thomsen's Three-Age System (1836); boundaries this library's round numbers",
        begins_bp: 3150.0,
        begins_std_dev_bp: 100.0,
        ends_bp: 2500.0,
        ends_std_dev_bp: 100.0,
    },
    ArchaeologicalPeriod {
        name: "Bronze Age",
        description: "From about 3300 BCE in the Near East, where tin bronze and the first \
                      writing appear together. In Britain the same period begins eleven \
                      centuries later, which is the clearest illustration of why this table \
                      names a region.",
        region: "Southwest Asia",
        source: "Thomsen's Three-Age System (1836); boundaries this library's round numbers",
        begins_bp: 5250.0,
        begins_std_dev_bp: 100.0,
        ends_bp: 3150.0,
        ends_std_dev_bp: 100.0,
    },
    ArchaeologicalPeriod {
        name: "Chalcolithic",
        description: "The Copper Age, about 4500 to 3300 BCE: copper worked but not yet alloyed. \
                      Not recognised as a separate period by every tradition, which is part of \
                      why its boundaries are the softest in the table.",
        region: "Southwest Asia",
        source: "The Levantine and Mesopotamian division in common use; boundaries this \
                 library's round numbers, no handbook read",
        begins_bp: 6450.0,
        begins_std_dev_bp: 200.0,
        ends_bp: 5250.0,
        ends_std_dev_bp: 100.0,
    },
    ArchaeologicalPeriod {
        name: "Neolithic",
        description: "From the Pre-Pottery Neolithic A, which begins with the Younger Dryas \
                      termination: cultivation, then herding, then pottery. Its opening is tied \
                      to the base of the Holocene at 11 700 b2k, the one boundary in this table \
                      fixed by a ratified GSSP rather than by convention.",
        region: "Southwest Asia",
        source: "Walker et al., J. Quaternary Sci. 24, 3 (2009), Holocene GSSP at 11 700 b2k",
        begins_bp: 11_650.0,
        begins_std_dev_bp: 100.0,
        ends_bp: 6450.0,
        ends_std_dev_bp: 200.0,
    },
    ArchaeologicalPeriod {
        name: "Epipalaeolithic",
        description: "The Last Glacial Maximum and the Late Glacial, including the Natufian. \
                      The European literature calls the later part of this span the Mesolithic \
                      and draws it differently; the two terms are not interchangeable, and this \
                      table uses the Southwest Asian one throughout.",
        region: "Southwest Asia",
        source: "Bar-Yosef, Evolutionary Anthropology 6, 159 (1998); boundaries this library's round numbers",
        begins_bp: 23_000.0,
        begins_std_dev_bp: 1000.0,
        ends_bp: 11_650.0,
        ends_std_dev_bp: 100.0,
    },
    ArchaeologicalPeriod {
        name: "Upper Palaeolithic",
        description: "Blade technology, worked bone and figurative art, from about 45 000 BP. \
                      This is also the outer limit of radiocarbon dating: beyond roughly 50 000 \
                      years the remaining 14C is below what accelerator mass spectrometry can \
                      separate from contamination.",
        region: "Europe and Southwest Asia",
        source: "This library's round numbers; the radiocarbon limit is instrumental, not a convention",
        begins_bp: 45_000.0,
        begins_std_dev_bp: 3000.0,
        ends_bp: 23_000.0,
        ends_std_dev_bp: 1000.0,
    },
    ArchaeologicalPeriod {
        name: "Middle Palaeolithic",
        description: "Prepared-core (Levallois) technology, about 300 000 to 45 000 BP. Every \
                      date in this period comes from luminescence, uranium series or \
                      argon-argon, never from radiocarbon, so \"BP\" here means calendar years \
                      and nothing else.",
        region: "Europe, Southwest Asia and Africa",
        source: "This library's round numbers; the 300 ka boundary follows the Middle Stone Age transition",
        begins_bp: 300_000.0,
        begins_std_dev_bp: 50_000.0,
        ends_bp: 45_000.0,
        ends_std_dev_bp: 3000.0,
    },
    ArchaeologicalPeriod {
        name: "Lower Palaeolithic",
        description: "From the oldest known stone tools, the Lomekwi 3 assemblage in Kenya at \
                      3.3 Ma — which predates the genus Homo. On this scale a fifty-year \
                      argument about the BP datum is six orders of magnitude below the noise.",
        region: "Africa, then Eurasia",
        source: "Harmand et al., Nature 521, 310 (2015), Lomekwi 3 at 3.3 Ma",
        begins_bp: 3_300_000.0,
        begins_std_dev_bp: 100_000.0,
        ends_bp: 300_000.0,
        ends_std_dev_bp: 50_000.0,
    },
];

/// Which conventional period `bp` years before present falls in.
///
/// Returns `None` for a future date and for anything before the Lomekwi tools.
#[must_use]
pub fn period_at_bp(bp: f64) -> Option<&'static ArchaeologicalPeriod> {
    PERIODS.iter().find(|period| period.contains_bp(bp))
}

/// Look a period up by name, case-sensitively.
#[must_use]
pub fn period_by_name(name: &str) -> Option<&'static ArchaeologicalPeriod> {
    PERIODS.iter().find(|period| period.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    use alloc::string::ToString as _;

    fn close(a: f64, b: f64, tolerance: f64) -> bool {
        hc_core::math::abs(a - b) <= tolerance
    }

    #[test]
    fn the_present_of_before_present_is_nineteen_fifty() {
        assert_eq!(BP_DATUM_YEAR, 1950);
        let age = Bp::calibrated(0.0, 0.0).unwrap();
        assert!(close(age.to_calendar_year().unwrap().value, 1950.0, 1e-9));
    }

    #[test]
    fn a_calendar_year_round_trips_through_bp() {
        for year in [-3300.0, -44.0, 0.0, 1066.0, 1492.0, 1950.0, 2026.0] {
            let age = Bp::from_calendar_year(year, 0.0).unwrap();
            let back = age.to_calendar_year().unwrap();
            assert!(
                close(back.value, year, 1e-9),
                "{year} came back as {}",
                back.value
            );
        }
    }

    #[test]
    fn astronomical_numbering_puts_year_zero_at_one_bce() {
        // 1950 BP is astronomical year 0, which historians call 1 BCE.
        let age = Bp::calibrated(1950.0, 0.0).unwrap();
        assert!(close(age.to_calendar_year().unwrap().value, 0.0, 1e-9));
        // 2000 BP is astronomical -50, which is 51 BCE.
        let older = Bp::calibrated(2000.0, 0.0).unwrap();
        assert!(close(older.to_calendar_year().unwrap().value, -50.0, 1e-9));
    }

    #[test]
    fn an_uncalibrated_radiocarbon_age_refuses_to_become_a_calendar_year() {
        let raw = Bp::uncalibrated(3200.0, 50.0).unwrap();
        assert_eq!(
            raw.to_calendar_year(),
            Err(DeepTimeError::CalibrationRequired)
        );
        assert_eq!(raw.to_deep_time(), Err(DeepTimeError::CalibrationRequired));
    }

    #[test]
    fn a_calibrated_age_converts_freely() {
        let calibrated = Bp::calibrated(3430.0, 60.0).unwrap();
        assert!(calibrated.to_calendar_year().is_ok());
        let span = calibrated.to_deep_time().unwrap();
        assert!(close(span.central_seconds(), 3430.0 * 31_557_600.0, 1.0));
    }

    #[test]
    fn the_error_bar_survives_every_conversion() {
        let calibrated = Bp::calibrated(3430.0, 60.0).unwrap();
        assert!(close(
            calibrated.to_calendar_year().unwrap().std_dev,
            60.0,
            1e-9
        ));
        let span = calibrated.to_deep_time().unwrap();
        assert!(close(span.std_dev(), 60.0 * 31_557_600.0, 1.0));
        let back = Bp::from_deep_time(span).unwrap();
        assert!(close(back.years().std_dev, 60.0, 1e-6));
        assert!(back.is_calibrated());
    }

    #[test]
    fn a_span_round_trips_through_a_calibrated_age() {
        for years in [0.0, 100.0, 11_650.0, 3_300_000.0] {
            let age = Bp::calibrated(years, 0.0).unwrap();
            let back = Bp::from_deep_time(age.to_deep_time().unwrap()).unwrap();
            assert!(close(back.years().value, years, years.abs() * 1e-12 + 1e-9));
        }
    }

    #[test]
    fn the_two_half_lives_differ_by_about_three_per_cent() {
        let ratio = CAMBRIDGE_HALF_LIFE_YEARS / LIBBY_HALF_LIFE_YEARS;
        assert!(close(ratio, 1.0291, 1e-4), "{ratio}");
    }

    #[test]
    fn rescaling_to_the_measured_half_life_is_not_calibration() {
        let raw = Bp::uncalibrated(5568.0, 100.0).unwrap();
        let rescaled = raw.on_measured_half_life().unwrap();
        assert!(close(rescaled.years().value, 5730.0, 1e-6));
        assert!(!rescaled.is_calibrated(), "it is still not a calendar year");
        assert_eq!(
            rescaled.to_calendar_year(),
            Err(DeepTimeError::CalibrationRequired)
        );
    }

    #[test]
    fn rescaling_an_already_calibrated_age_is_refused() {
        let calibrated = Bp::calibrated(3430.0, 60.0).unwrap();
        assert_eq!(
            calibrated.on_measured_half_life(),
            Err(DeepTimeError::CalibrationRequired)
        );
    }

    #[test]
    fn the_ice_core_datum_runs_fifty_years_ahead_of_the_radiocarbon_one() {
        assert!(close(b2k_to_bp(11_700.0), 11_650.0, 1e-9));
        assert!(close(bp_to_b2k(11_650.0), 11_700.0, 1e-9));
        for years in [0.0, 1000.0, 11_700.0, 129_000.0] {
            assert!(close(bp_to_b2k(b2k_to_bp(years)), years, 1e-9));
        }
    }

    #[test]
    fn the_neolithic_opens_at_the_holocene_gssp() {
        let neolithic = period_by_name("Neolithic").unwrap();
        assert!(close(neolithic.begins_bp(), b2k_to_bp(11_700.0), 1e-9));
    }

    #[test]
    fn the_periods_are_in_strictly_increasing_order_and_do_not_overlap() {
        for pair in PERIODS.windows(2) {
            assert!(
                pair[1].ends_bp > pair[0].ends_bp,
                "{} is not older than {}",
                pair[1].name,
                pair[0].name
            );
            assert!(
                (pair[0].begins_bp - pair[1].ends_bp).abs() < 1e-9,
                "{} and {} leave a gap",
                pair[0].name,
                pair[1].name
            );
        }
    }

    #[test]
    fn every_period_runs_from_older_to_younger() {
        for period in PERIODS {
            assert!(
                period.begins_bp > period.ends_bp,
                "{} runs backwards",
                period.name
            );
        }
    }

    #[test]
    fn no_period_boundary_carries_a_negative_uncertainty() {
        for period in PERIODS {
            assert!(period.begins_std_dev_bp >= 0.0, "{}", period.name);
            assert!(period.ends_std_dev_bp >= 0.0, "{}", period.name);
        }
    }

    #[test]
    fn every_period_names_a_region_and_a_source() {
        for period in PERIODS {
            assert!(!period.region.is_empty(), "{} has no region", period.name);
            assert!(!period.source.is_empty(), "{} has no source", period.name);
            assert!(period.description.len() > 40, "{}", period.name);
        }
    }

    #[test]
    fn the_sequence_reaches_the_present_and_the_first_stone_tools() {
        assert!(close(PERIODS.first().unwrap().ends_bp(), 0.0, 1e-9));
        assert!(close(PERIODS.last().unwrap().begins_bp(), 3.3e6, 1e-9));
    }

    #[test]
    fn a_query_lands_in_the_right_period() {
        assert_eq!(period_at_bp(0.0).map(|p| p.name), Some("Modern period"));
        assert_eq!(
            period_at_bp(2000.0).map(|p| p.name),
            Some("Classical antiquity")
        );
        assert_eq!(period_at_bp(4000.0).map(|p| p.name), Some("Bronze Age"));
        assert_eq!(period_at_bp(10_000.0).map(|p| p.name), Some("Neolithic"));
        assert_eq!(
            period_at_bp(30_000.0).map(|p| p.name),
            Some("Upper Palaeolithic")
        );
        assert_eq!(
            period_at_bp(1e6).map(|p| p.name),
            Some("Lower Palaeolithic")
        );
    }

    #[test]
    fn a_query_outside_the_sequence_finds_nothing() {
        assert!(
            period_at_bp(-100.0).is_none(),
            "the future is not archaeology"
        );
        assert!(period_at_bp(1e7).is_none(), "nor is the Miocene");
    }

    #[test]
    fn periods_are_reachable_by_name() {
        assert!(period_by_name("Bronze Age").is_some());
        assert!(period_by_name("bronze age").is_none(), "case-sensitive");
        assert!(
            period_by_name("Mesolithic").is_none(),
            "not in this sequence"
        );
    }

    #[test]
    fn every_period_boundary_is_a_calibrated_age() {
        for period in PERIODS {
            assert!(period.begins().unwrap().is_calibrated(), "{}", period.name);
            assert!(period.ends().unwrap().is_calibrated(), "{}", period.name);
        }
    }

    #[test]
    fn an_age_renders_with_the_suffix_its_calibration_earns() {
        #[cfg(feature = "alloc")]
        {
            assert_eq!(
                Bp::uncalibrated(3200.0, 50.0).unwrap().to_string(),
                "3200 ± 50 BP"
            );
            assert_eq!(
                Bp::calibrated(3430.0, 60.0).unwrap().to_string(),
                "3430 ± 60 cal BP"
            );
            assert_eq!(Bp::calibrated(0.0, 0.0).unwrap().to_string(), "0 cal BP");
        }
        assert_eq!(Calibration::Uncalibrated.suffix(), "BP");
        assert_eq!(Calibration::Calibrated.suffix(), "cal BP");
    }

    #[test]
    fn a_negative_standard_deviation_is_refused() {
        assert!(Bp::uncalibrated(3200.0, -1.0).is_err());
        assert!(Bp::calibrated(3200.0, -1.0).is_err());
    }
}
