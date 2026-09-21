//! The geological time scale, as data.
//!
//! Boundary ages come from the **International Chronostratigraphic Chart
//! v2026/06**, published by the International Commission on Stratigraphy at
//! <http://www.stratigraphy.org/ICSchart/ChronostratChart2026-06.pdf> and
//! cited as Cohen, K.M., Harper, D.A.T., Gibbard, P.L. & Car, N. (2025,
//! updated), *The ICS international chronostratigraphic chart this decade*,
//! Episodes 48, 105–115. The chart in turn takes most of its numerical ages
//! from Gradstein et al., *A Geologic Time Scale 2020*, with revisions from
//! the relevant ICS subcommissions.
//!
//! Version matters. Between v2024/12 and v2026/06 three boundaries moved —
//! the Anisian from 246.7 to 247.0 Ma, the Olenekian from 249.9 to 250.8 Ma
//! and the Wuchiapingian from 259.51 ± 0.21 to 259.857 ± 0.084 Ma — so a
//! chronology quoted without a chart version is a chronology that cannot be
//! checked. [`CHART_VERSION`] is part of the public API for that reason.
//!
//! # What a zero uncertainty means here
//!
//! The chart prints an uncertainty for some boundaries and not for others,
//! and the silence has two quite different causes:
//!
//! * A **GSSA** — a Global Standard Stratigraphic Age — is a round number
//!   *defined by decree*. The Proterozoic and Archean boundaries at 2500,
//!   1600 and 1000 Ma are of this kind. They have no uncertainty because they
//!   are not measurements.
//! * A **GSSP** — a Global Boundary Stratotype Section and Point — is defined
//!   by a golden spike in a rock face. Its numerical age is an estimate, and
//!   the chart sometimes prints one without an error bar (the Danian at 66.00
//!   Ma, the Barremian at 125.77 Ma, astronomically tuned boundaries in the
//!   Neogene).
//!
//! This module stores `0.0` for both and does not distinguish them, because
//! the chart does not carry the distinction in a machine-readable form. The
//! `approximate` flags record the chart's own `~` marker, which is its way of
//! saying "no ratified GSSP and no well-constrained age".
//!
//! # The shape of the tree
//!
//! Five ranks, each a flat array ordered youngest first, each entry naming its
//! parent. The arrays cover different spans, because the chart subdivides
//! different amounts of Earth history at different ranks:
//!
//! | Rank | Entries | Covers |
//! | --- | --- | --- |
//! | [`EONS`] | 4 | 0 – 4567 Ma |
//! | [`ERAS`] | 10 | 0 – 4031 Ma (the Hadean has no eras) |
//! | [`PERIODS`] | 22 | 0 – 2500 Ma |
//! | [`EPOCHS`] | 38 | 0 – 538.8 Ma |
//! | [`AGES`] | 101 | 0 – 538.8 Ma, except the Pridoli, which has no stages |
//!
//! Two deliberate simplifications. The Mississippian and Pennsylvanian are
//! formally *subsystems* of the Carboniferous, a rank this five-level model
//! does not have; their series appear here as epochs named "Lower
//! Mississippian" and so on with the Carboniferous as their parent. And the
//! ratified subseries (Upper/Middle/Lower Pleistocene and their kin) are
//! folded into the age-rank names the same way.

use crate::error::DeepTimeResult;
use crate::magnitude::{DeepTime, DeepUnit};

/// The chart edition every age in this module comes from.
pub const CHART_VERSION: &str = "v2026/06";

/// The full citation for [`CHART_VERSION`].
pub const CHART_CITATION: &str = "Cohen, K.M., Harper, D.A.T., Gibbard, P.L. & Car, N. (2025, updated), The ICS \
     international chronostratigraphic chart this decade, Episodes 48: 105-115; chart \
     v2026/06 at stratigraphy.org";

/// Where an interval sits in the chronostratigraphic hierarchy.
///
/// These are the *geochronologic* names — eon, era, period, epoch, age — which
/// are the units of time. Their chronostratigraphic counterparts, the units of
/// rock, are eonothem, erathem, system, series and stage, and the chart prints
/// both. This crate deals in time, so it uses the time names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GeologicRank {
    /// The coarsest division: Hadean, Archean, Proterozoic, Phanerozoic.
    Eon,
    /// Palaeozoic, Mesozoic, Cenozoic and their Precambrian counterparts.
    Era,
    /// Cambrian through Quaternary, plus the Proterozoic periods.
    Period,
    /// Series-level: Holocene, Pleistocene, Upper Cretaceous and the rest.
    Epoch,
    /// The finest ratified division, a stage: Maastrichtian, Chibanian.
    Age,
}

impl GeologicRank {
    /// Every rank, coarsest first.
    pub const ALL: &'static [Self] = &[Self::Eon, Self::Era, Self::Period, Self::Epoch, Self::Age];

    /// The English name of the rank.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::Eon => "eon",
            Self::Era => "era",
            Self::Period => "period",
            Self::Epoch => "epoch",
            Self::Age => "age",
        }
    }

    /// The chronostratigraphic name for the same rank — the rock, not the
    /// time.
    #[must_use]
    pub const fn chronostratigraphic_name(self) -> &'static str {
        match self {
            Self::Eon => "eonothem",
            Self::Era => "erathem",
            Self::Period => "system",
            Self::Epoch => "series",
            Self::Age => "stage",
        }
    }
}

/// One interval of the geological time scale.
///
/// Boundary ages are in megayears before present, the unit the chart uses.
/// "Present" here is the chart's present, which for a scale whose finest
/// division is 4200 years wide is not worth pinning to a particular year;
/// [`crate::archaeology`] is where the 1950 convention starts to matter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeologicInterval {
    /// The interval's name as the chart spells it.
    pub name: &'static str,
    /// Where it sits in the hierarchy.
    pub rank: GeologicRank,
    /// The name of the containing interval one rank up, if there is one.
    pub parent: Option<&'static str>,
    /// The younger boundary, in Ma before present.
    pub top_ma: f64,
    /// Its published standard uncertainty, or zero where the chart gives none.
    pub top_std_dev_ma: f64,
    /// Whether the chart marks the younger boundary with `~`.
    pub top_approximate: bool,
    /// How many digits of `top_ma` the chart prints.
    pub top_figures: u8,
    /// The older boundary, in Ma before present.
    pub base_ma: f64,
    /// Its published standard uncertainty, or zero where the chart gives none.
    pub base_std_dev_ma: f64,
    /// Whether the chart marks the older boundary with `~`.
    pub base_approximate: bool,
    /// How many digits of `base_ma` the chart prints.
    pub base_figures: u8,
}

impl GeologicInterval {
    /// The older boundary as a span before present.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::new`]; cannot fail for the tabulated entries.
    pub fn base(&self) -> DeepTimeResult<DeepTime> {
        DeepTime::from_megayears(self.base_ma, self.base_std_dev_ma)?
            .with_figures(self.base_figures)
    }

    /// The younger boundary as a span before present.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::new`]; cannot fail for the tabulated entries.
    pub fn top(&self) -> DeepTimeResult<DeepTime> {
        DeepTime::from_megayears(self.top_ma, self.top_std_dev_ma)?.with_figures(self.top_figures)
    }

    /// How long the interval lasted, with the two boundary errors combined in
    /// quadrature.
    ///
    /// The two boundaries are treated as independent measurements, which they
    /// are: they come from different sections dated by different laboratories.
    ///
    /// # Errors
    ///
    /// See [`DeepTime::checked_sub`].
    pub fn duration(&self) -> DeepTimeResult<DeepTime> {
        self.base()?.checked_sub(self.top()?)
    }

    /// The duration in megayears, central value only.
    #[must_use]
    pub fn duration_ma(&self) -> f64 {
        self.base_ma - self.top_ma
    }

    /// Whether `ma` megayears before present falls in `[top, base)`.
    ///
    /// The half-open convention puts a boundary age in the *older* interval,
    /// so 66.00 Ma is Maastrichtian and Cretaceous rather than Danian and
    /// Paleogene. That matches how the boundary is read — "the Cretaceous
    /// ended at 66 Ma" — and it makes the ranks partition cleanly.
    #[must_use]
    pub fn contains_ma(&self, ma: f64) -> bool {
        ma >= self.top_ma && ma < self.base_ma
    }

    /// Whether either boundary is one the chart marks `~`.
    #[must_use]
    pub const fn is_approximate(&self) -> bool {
        self.top_approximate || self.base_approximate
    }
}

/// The intervals of one rank, ordered youngest first.
#[must_use]
pub fn intervals(rank: GeologicRank) -> &'static [GeologicInterval] {
    match rank {
        GeologicRank::Eon => EONS,
        GeologicRank::Era => ERAS,
        GeologicRank::Period => PERIODS,
        GeologicRank::Epoch => EPOCHS,
        GeologicRank::Age => AGES,
    }
}

/// Which interval of `rank` was current `ma` megayears before present.
///
/// Returns `None` outside the span that rank covers — before the Hadean at
/// any rank, and anywhere in the Precambrian for epochs and ages. That is the
/// right answer: the chart does not subdivide the Archean into periods, and
/// inventing one would be worse than admitting it.
#[must_use]
pub fn interval_at(ma: f64, rank: GeologicRank) -> Option<&'static GeologicInterval> {
    intervals(rank)
        .iter()
        .find(|interval| interval.contains_ma(ma))
}

/// The whole chain of intervals containing `ma`, from eon down to age.
///
/// Entries are `None` where that rank does not reach. Asking for 250 Ma gives
/// Phanerozoic, Mesozoic, Triassic, Lower Triassic, Olenekian; asking for
/// 3000 Ma gives Archean, Mesoarchean and three `None`s.
#[must_use]
pub fn chain_at(ma: f64) -> [Option<&'static GeologicInterval>; 5] {
    [
        interval_at(ma, GeologicRank::Eon),
        interval_at(ma, GeologicRank::Era),
        interval_at(ma, GeologicRank::Period),
        interval_at(ma, GeologicRank::Epoch),
        interval_at(ma, GeologicRank::Age),
    ]
}

/// Look an interval up by name, case-sensitively, at any rank.
#[must_use]
pub fn by_name(name: &str) -> Option<&'static GeologicInterval> {
    GeologicRank::ALL
        .iter()
        .find_map(|rank| intervals(*rank).iter().find(|entry| entry.name == name))
}

/// The children of an interval, one rank down.
///
/// Returns an empty slice iterator for the finest rank and for intervals the
/// chart does not subdivide.
pub fn children(
    parent: &GeologicInterval,
) -> impl Iterator<Item = &'static GeologicInterval> + use<> {
    let finer = match parent.rank {
        GeologicRank::Eon => Some(GeologicRank::Era),
        GeologicRank::Era => Some(GeologicRank::Period),
        GeologicRank::Period => Some(GeologicRank::Epoch),
        GeologicRank::Epoch => Some(GeologicRank::Age),
        GeologicRank::Age => None,
    };
    let name = parent.name;
    finer
        .map(intervals)
        .unwrap_or(&[])
        .iter()
        .filter(move |child| child.parent == Some(name))
}

/// The age of the oldest boundary on the chart, in megayears.
///
/// 4567 Ma, the base of the Hadean and the conventional start of the Solar
/// System. The chart prints it without an uncertainty; the best modern
/// determination is 4567.30 ± 0.16 Ma from calcium-aluminium-rich inclusions
/// (Connelly et al., Science 338, 651, 2012), which
/// [`crate::universe::SOLAR_SYSTEM_FORMATION`] carries.
pub const OLDEST_BOUNDARY_MA: f64 = 4567.0;

/// Convert an age in megayears before present into a span.
///
/// # Errors
///
/// See [`DeepTime::from_megayears`].
pub fn before_present(ma: f64, std_dev_ma: f64) -> DeepTimeResult<DeepTime> {
    DeepTime::from_megayears(ma, std_dev_ma)
}

/// Express a span before present in megayears.
///
/// # Errors
///
/// See [`DeepTime::in_unit`].
pub fn as_megayears(span: DeepTime) -> DeepTimeResult<f64> {
    Ok(span.in_unit(DeepUnit::Megayear)?.value)
}

/// The four eons, youngest first. They tile the whole of Earth history.
pub const EONS: &[GeologicInterval] = &[
    GeologicInterval {
        name: "Phanerozoic",
        rank: GeologicRank::Eon,
        parent: None,
        top_ma: 0.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 1,
        base_ma: 538.8,
        base_std_dev_ma: 0.6,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Proterozoic",
        rank: GeologicRank::Eon,
        parent: None,
        top_ma: 538.8,
        top_std_dev_ma: 0.6,
        top_approximate: false,
        top_figures: 4,
        base_ma: 2500.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Archean",
        rank: GeologicRank::Eon,
        parent: None,
        top_ma: 2500.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 4031.0,
        base_std_dev_ma: 3.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Hadean",
        rank: GeologicRank::Eon,
        parent: None,
        top_ma: 4031.0,
        top_std_dev_ma: 3.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 4567.0,
        base_std_dev_ma: 0.0,
        base_approximate: true,
        base_figures: 4,
    },
];

/// The ten eras, youngest first. They tile 0 to 4031 Ma; the Hadean has none.
pub const ERAS: &[GeologicInterval] = &[
    GeologicInterval {
        name: "Cenozoic",
        rank: GeologicRank::Era,
        parent: Some("Phanerozoic"),
        top_ma: 0.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 1,
        base_ma: 66.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Mesozoic",
        rank: GeologicRank::Era,
        parent: Some("Phanerozoic"),
        top_ma: 66.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 251.902,
        base_std_dev_ma: 0.024,
        base_approximate: false,
        base_figures: 6,
    },
    GeologicInterval {
        name: "Paleozoic",
        rank: GeologicRank::Era,
        parent: Some("Phanerozoic"),
        top_ma: 251.902,
        top_std_dev_ma: 0.024,
        top_approximate: false,
        top_figures: 6,
        base_ma: 538.8,
        base_std_dev_ma: 0.6,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Neoproterozoic",
        rank: GeologicRank::Era,
        parent: Some("Proterozoic"),
        top_ma: 538.8,
        top_std_dev_ma: 0.6,
        top_approximate: false,
        top_figures: 4,
        base_ma: 1000.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Mesoproterozoic",
        rank: GeologicRank::Era,
        parent: Some("Proterozoic"),
        top_ma: 1000.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 1600.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Paleoproterozoic",
        rank: GeologicRank::Era,
        parent: Some("Proterozoic"),
        top_ma: 1600.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 2500.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Neoarchean",
        rank: GeologicRank::Era,
        parent: Some("Archean"),
        top_ma: 2500.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 2800.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Mesoarchean",
        rank: GeologicRank::Era,
        parent: Some("Archean"),
        top_ma: 2800.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 3200.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Paleoarchean",
        rank: GeologicRank::Era,
        parent: Some("Archean"),
        top_ma: 3200.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 3600.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Eoarchean",
        rank: GeologicRank::Era,
        parent: Some("Archean"),
        top_ma: 3600.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 4031.0,
        base_std_dev_ma: 3.0,
        base_approximate: false,
        base_figures: 4,
    },
];

/// The twenty-two periods, youngest first. They tile 0 to 2500 Ma.
pub const PERIODS: &[GeologicInterval] = &[
    GeologicInterval {
        name: "Quaternary",
        rank: GeologicRank::Period,
        parent: Some("Cenozoic"),
        top_ma: 0.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 1,
        base_ma: 2.58,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 3,
    },
    GeologicInterval {
        name: "Neogene",
        rank: GeologicRank::Period,
        parent: Some("Cenozoic"),
        top_ma: 2.58,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 3,
        base_ma: 23.04,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Paleogene",
        rank: GeologicRank::Period,
        parent: Some("Cenozoic"),
        top_ma: 23.04,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 66.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Cretaceous",
        rank: GeologicRank::Period,
        parent: Some("Mesozoic"),
        top_ma: 66.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 143.1,
        base_std_dev_ma: 0.6,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Jurassic",
        rank: GeologicRank::Period,
        parent: Some("Mesozoic"),
        top_ma: 143.1,
        top_std_dev_ma: 0.6,
        top_approximate: false,
        top_figures: 4,
        base_ma: 201.4,
        base_std_dev_ma: 0.2,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Triassic",
        rank: GeologicRank::Period,
        parent: Some("Mesozoic"),
        top_ma: 201.4,
        top_std_dev_ma: 0.2,
        top_approximate: false,
        top_figures: 4,
        base_ma: 251.902,
        base_std_dev_ma: 0.024,
        base_approximate: false,
        base_figures: 6,
    },
    GeologicInterval {
        name: "Permian",
        rank: GeologicRank::Period,
        parent: Some("Paleozoic"),
        top_ma: 251.902,
        top_std_dev_ma: 0.024,
        top_approximate: false,
        top_figures: 6,
        base_ma: 298.9,
        base_std_dev_ma: 0.15,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Carboniferous",
        rank: GeologicRank::Period,
        parent: Some("Paleozoic"),
        top_ma: 298.9,
        top_std_dev_ma: 0.15,
        top_approximate: false,
        top_figures: 4,
        base_ma: 358.86,
        base_std_dev_ma: 0.19,
        base_approximate: false,
        base_figures: 5,
    },
    GeologicInterval {
        name: "Devonian",
        rank: GeologicRank::Period,
        parent: Some("Paleozoic"),
        top_ma: 358.86,
        top_std_dev_ma: 0.19,
        top_approximate: false,
        top_figures: 5,
        base_ma: 419.62,
        base_std_dev_ma: 1.36,
        base_approximate: false,
        base_figures: 5,
    },
    GeologicInterval {
        name: "Silurian",
        rank: GeologicRank::Period,
        parent: Some("Paleozoic"),
        top_ma: 419.62,
        top_std_dev_ma: 1.36,
        top_approximate: false,
        top_figures: 5,
        base_ma: 443.1,
        base_std_dev_ma: 0.9,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Ordovician",
        rank: GeologicRank::Period,
        parent: Some("Paleozoic"),
        top_ma: 443.1,
        top_std_dev_ma: 0.9,
        top_approximate: false,
        top_figures: 4,
        base_ma: 486.85,
        base_std_dev_ma: 1.5,
        base_approximate: false,
        base_figures: 5,
    },
    GeologicInterval {
        name: "Cambrian",
        rank: GeologicRank::Period,
        parent: Some("Paleozoic"),
        top_ma: 486.85,
        top_std_dev_ma: 1.5,
        top_approximate: false,
        top_figures: 5,
        base_ma: 538.8,
        base_std_dev_ma: 0.6,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Ediacaran",
        rank: GeologicRank::Period,
        parent: Some("Neoproterozoic"),
        top_ma: 538.8,
        top_std_dev_ma: 0.6,
        top_approximate: false,
        top_figures: 4,
        base_ma: 635.0,
        base_std_dev_ma: 0.0,
        base_approximate: true,
        base_figures: 3,
    },
    GeologicInterval {
        name: "Cryogenian",
        rank: GeologicRank::Period,
        parent: Some("Neoproterozoic"),
        top_ma: 635.0,
        top_std_dev_ma: 0.0,
        top_approximate: true,
        top_figures: 3,
        base_ma: 720.0,
        base_std_dev_ma: 0.0,
        base_approximate: true,
        base_figures: 3,
    },
    GeologicInterval {
        name: "Tonian",
        rank: GeologicRank::Period,
        parent: Some("Neoproterozoic"),
        top_ma: 720.0,
        top_std_dev_ma: 0.0,
        top_approximate: true,
        top_figures: 3,
        base_ma: 1000.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Stenian",
        rank: GeologicRank::Period,
        parent: Some("Mesoproterozoic"),
        top_ma: 1000.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 1200.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Ectasian",
        rank: GeologicRank::Period,
        parent: Some("Mesoproterozoic"),
        top_ma: 1200.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 1400.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Calymmian",
        rank: GeologicRank::Period,
        parent: Some("Mesoproterozoic"),
        top_ma: 1400.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 1600.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Statherian",
        rank: GeologicRank::Period,
        parent: Some("Paleoproterozoic"),
        top_ma: 1600.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 1800.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Orosirian",
        rank: GeologicRank::Period,
        parent: Some("Paleoproterozoic"),
        top_ma: 1800.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 2050.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Rhyacian",
        rank: GeologicRank::Period,
        parent: Some("Paleoproterozoic"),
        top_ma: 2050.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 2300.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Siderian",
        rank: GeologicRank::Period,
        parent: Some("Paleoproterozoic"),
        top_ma: 2300.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 2500.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
];

/// The thirty-eight epochs, youngest first. They tile the Phanerozoic.
pub const EPOCHS: &[GeologicInterval] = &[
    GeologicInterval {
        name: "Holocene",
        rank: GeologicRank::Epoch,
        parent: Some("Quaternary"),
        top_ma: 0.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 1,
        base_ma: 0.0117,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 3,
    },
    GeologicInterval {
        name: "Pleistocene",
        rank: GeologicRank::Epoch,
        parent: Some("Quaternary"),
        top_ma: 0.0117,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 3,
        base_ma: 2.58,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 3,
    },
    GeologicInterval {
        name: "Pliocene",
        rank: GeologicRank::Epoch,
        parent: Some("Neogene"),
        top_ma: 2.58,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 3,
        base_ma: 5.333,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Miocene",
        rank: GeologicRank::Epoch,
        parent: Some("Neogene"),
        top_ma: 5.333,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 23.04,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Oligocene",
        rank: GeologicRank::Epoch,
        parent: Some("Paleogene"),
        top_ma: 23.04,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 33.9,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 3,
    },
    GeologicInterval {
        name: "Eocene",
        rank: GeologicRank::Epoch,
        parent: Some("Paleogene"),
        top_ma: 33.9,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 3,
        base_ma: 56.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Paleocene",
        rank: GeologicRank::Epoch,
        parent: Some("Paleogene"),
        top_ma: 56.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 66.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Upper Cretaceous",
        rank: GeologicRank::Epoch,
        parent: Some("Cretaceous"),
        top_ma: 66.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 100.5,
        base_std_dev_ma: 0.1,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Lower Cretaceous",
        rank: GeologicRank::Epoch,
        parent: Some("Cretaceous"),
        top_ma: 100.5,
        top_std_dev_ma: 0.1,
        top_approximate: false,
        top_figures: 4,
        base_ma: 143.1,
        base_std_dev_ma: 0.6,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Upper Jurassic",
        rank: GeologicRank::Epoch,
        parent: Some("Jurassic"),
        top_ma: 143.1,
        top_std_dev_ma: 0.6,
        top_approximate: false,
        top_figures: 4,
        base_ma: 161.5,
        base_std_dev_ma: 1.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Middle Jurassic",
        rank: GeologicRank::Epoch,
        parent: Some("Jurassic"),
        top_ma: 161.5,
        top_std_dev_ma: 1.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 174.7,
        base_std_dev_ma: 0.8,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Lower Jurassic",
        rank: GeologicRank::Epoch,
        parent: Some("Jurassic"),
        top_ma: 174.7,
        top_std_dev_ma: 0.8,
        top_approximate: false,
        top_figures: 4,
        base_ma: 201.4,
        base_std_dev_ma: 0.2,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Upper Triassic",
        rank: GeologicRank::Epoch,
        parent: Some("Triassic"),
        top_ma: 201.4,
        top_std_dev_ma: 0.2,
        top_approximate: false,
        top_figures: 4,
        base_ma: 237.0,
        base_std_dev_ma: 0.0,
        base_approximate: true,
        base_figures: 3,
    },
    GeologicInterval {
        name: "Middle Triassic",
        rank: GeologicRank::Epoch,
        parent: Some("Triassic"),
        top_ma: 237.0,
        top_std_dev_ma: 0.0,
        top_approximate: true,
        top_figures: 3,
        base_ma: 247.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Lower Triassic",
        rank: GeologicRank::Epoch,
        parent: Some("Triassic"),
        top_ma: 247.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 251.902,
        base_std_dev_ma: 0.024,
        base_approximate: false,
        base_figures: 6,
    },
    GeologicInterval {
        name: "Lopingian",
        rank: GeologicRank::Epoch,
        parent: Some("Permian"),
        top_ma: 251.902,
        top_std_dev_ma: 0.024,
        top_approximate: false,
        top_figures: 6,
        base_ma: 259.857,
        base_std_dev_ma: 0.084,
        base_approximate: false,
        base_figures: 6,
    },
    GeologicInterval {
        name: "Guadalupian",
        rank: GeologicRank::Epoch,
        parent: Some("Permian"),
        top_ma: 259.857,
        top_std_dev_ma: 0.084,
        top_approximate: false,
        top_figures: 6,
        base_ma: 274.4,
        base_std_dev_ma: 0.4,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Cisuralian",
        rank: GeologicRank::Epoch,
        parent: Some("Permian"),
        top_ma: 274.4,
        top_std_dev_ma: 0.4,
        top_approximate: false,
        top_figures: 4,
        base_ma: 298.9,
        base_std_dev_ma: 0.15,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Upper Pennsylvanian",
        rank: GeologicRank::Epoch,
        parent: Some("Carboniferous"),
        top_ma: 298.9,
        top_std_dev_ma: 0.15,
        top_approximate: false,
        top_figures: 4,
        base_ma: 307.0,
        base_std_dev_ma: 0.1,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Middle Pennsylvanian",
        rank: GeologicRank::Epoch,
        parent: Some("Carboniferous"),
        top_ma: 307.0,
        top_std_dev_ma: 0.1,
        top_approximate: false,
        top_figures: 4,
        base_ma: 315.2,
        base_std_dev_ma: 0.2,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Lower Pennsylvanian",
        rank: GeologicRank::Epoch,
        parent: Some("Carboniferous"),
        top_ma: 315.2,
        top_std_dev_ma: 0.2,
        top_approximate: false,
        top_figures: 4,
        base_ma: 323.4,
        base_std_dev_ma: 0.4,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Upper Mississippian",
        rank: GeologicRank::Epoch,
        parent: Some("Carboniferous"),
        top_ma: 323.4,
        top_std_dev_ma: 0.4,
        top_approximate: false,
        top_figures: 4,
        base_ma: 330.3,
        base_std_dev_ma: 0.4,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Middle Mississippian",
        rank: GeologicRank::Epoch,
        parent: Some("Carboniferous"),
        top_ma: 330.3,
        top_std_dev_ma: 0.4,
        top_approximate: false,
        top_figures: 4,
        base_ma: 346.7,
        base_std_dev_ma: 0.4,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Lower Mississippian",
        rank: GeologicRank::Epoch,
        parent: Some("Carboniferous"),
        top_ma: 346.7,
        top_std_dev_ma: 0.4,
        top_approximate: false,
        top_figures: 4,
        base_ma: 358.86,
        base_std_dev_ma: 0.19,
        base_approximate: false,
        base_figures: 5,
    },
    GeologicInterval {
        name: "Upper Devonian",
        rank: GeologicRank::Epoch,
        parent: Some("Devonian"),
        top_ma: 358.86,
        top_std_dev_ma: 0.19,
        top_approximate: false,
        top_figures: 5,
        base_ma: 382.31,
        base_std_dev_ma: 1.36,
        base_approximate: false,
        base_figures: 5,
    },
    GeologicInterval {
        name: "Middle Devonian",
        rank: GeologicRank::Epoch,
        parent: Some("Devonian"),
        top_ma: 382.31,
        top_std_dev_ma: 1.36,
        top_approximate: false,
        top_figures: 5,
        base_ma: 393.47,
        base_std_dev_ma: 0.99,
        base_approximate: false,
        base_figures: 5,
    },
    GeologicInterval {
        name: "Lower Devonian",
        rank: GeologicRank::Epoch,
        parent: Some("Devonian"),
        top_ma: 393.47,
        top_std_dev_ma: 0.99,
        top_approximate: false,
        top_figures: 5,
        base_ma: 419.62,
        base_std_dev_ma: 1.36,
        base_approximate: false,
        base_figures: 5,
    },
    GeologicInterval {
        name: "Pridoli",
        rank: GeologicRank::Epoch,
        parent: Some("Silurian"),
        top_ma: 419.62,
        top_std_dev_ma: 1.36,
        top_approximate: false,
        top_figures: 5,
        base_ma: 422.7,
        base_std_dev_ma: 1.6,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Ludlow",
        rank: GeologicRank::Epoch,
        parent: Some("Silurian"),
        top_ma: 422.7,
        top_std_dev_ma: 1.6,
        top_approximate: false,
        top_figures: 4,
        base_ma: 426.7,
        base_std_dev_ma: 1.5,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Wenlock",
        rank: GeologicRank::Epoch,
        parent: Some("Silurian"),
        top_ma: 426.7,
        top_std_dev_ma: 1.5,
        top_approximate: false,
        top_figures: 4,
        base_ma: 432.9,
        base_std_dev_ma: 1.2,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Llandovery",
        rank: GeologicRank::Epoch,
        parent: Some("Silurian"),
        top_ma: 432.9,
        top_std_dev_ma: 1.2,
        top_approximate: false,
        top_figures: 4,
        base_ma: 443.1,
        base_std_dev_ma: 0.9,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Upper Ordovician",
        rank: GeologicRank::Epoch,
        parent: Some("Ordovician"),
        top_ma: 443.1,
        top_std_dev_ma: 0.9,
        top_approximate: false,
        top_figures: 4,
        base_ma: 458.2,
        base_std_dev_ma: 0.7,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Middle Ordovician",
        rank: GeologicRank::Epoch,
        parent: Some("Ordovician"),
        top_ma: 458.2,
        top_std_dev_ma: 0.7,
        top_approximate: false,
        top_figures: 4,
        base_ma: 471.3,
        base_std_dev_ma: 1.4,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Lower Ordovician",
        rank: GeologicRank::Epoch,
        parent: Some("Ordovician"),
        top_ma: 471.3,
        top_std_dev_ma: 1.4,
        top_approximate: false,
        top_figures: 4,
        base_ma: 486.85,
        base_std_dev_ma: 1.5,
        base_approximate: false,
        base_figures: 5,
    },
    GeologicInterval {
        name: "Furongian",
        rank: GeologicRank::Epoch,
        parent: Some("Cambrian"),
        top_ma: 486.85,
        top_std_dev_ma: 1.5,
        top_approximate: false,
        top_figures: 5,
        base_ma: 497.0,
        base_std_dev_ma: 0.0,
        base_approximate: true,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Miaolingian",
        rank: GeologicRank::Epoch,
        parent: Some("Cambrian"),
        top_ma: 497.0,
        top_std_dev_ma: 0.0,
        top_approximate: true,
        top_figures: 4,
        base_ma: 506.5,
        base_std_dev_ma: 0.0,
        base_approximate: true,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Cambrian Series 2",
        rank: GeologicRank::Epoch,
        parent: Some("Cambrian"),
        top_ma: 506.5,
        top_std_dev_ma: 0.0,
        top_approximate: true,
        top_figures: 4,
        base_ma: 521.0,
        base_std_dev_ma: 0.0,
        base_approximate: true,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Terreneuvian",
        rank: GeologicRank::Epoch,
        parent: Some("Cambrian"),
        top_ma: 521.0,
        top_std_dev_ma: 0.0,
        top_approximate: true,
        top_figures: 4,
        base_ma: 538.8,
        base_std_dev_ma: 0.6,
        base_approximate: false,
        base_figures: 4,
    },
];

/// The hundred and one ages, youngest first. They tile the Phanerozoic apart from the Pridoli, which has no stages.
pub const AGES: &[GeologicInterval] = &[
    GeologicInterval {
        name: "Meghalayan",
        rank: GeologicRank::Age,
        parent: Some("Holocene"),
        top_ma: 0.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 1,
        base_ma: 0.0042,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 2,
    },
    GeologicInterval {
        name: "Northgrippian",
        rank: GeologicRank::Age,
        parent: Some("Holocene"),
        top_ma: 0.0042,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 2,
        base_ma: 0.0082,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 2,
    },
    GeologicInterval {
        name: "Greenlandian",
        rank: GeologicRank::Age,
        parent: Some("Holocene"),
        top_ma: 0.0082,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 2,
        base_ma: 0.0117,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 3,
    },
    GeologicInterval {
        name: "Upper Pleistocene",
        rank: GeologicRank::Age,
        parent: Some("Pleistocene"),
        top_ma: 0.0117,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 3,
        base_ma: 0.129,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 3,
    },
    GeologicInterval {
        name: "Chibanian",
        rank: GeologicRank::Age,
        parent: Some("Pleistocene"),
        top_ma: 0.129,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 3,
        base_ma: 0.774,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 3,
    },
    GeologicInterval {
        name: "Calabrian",
        rank: GeologicRank::Age,
        parent: Some("Pleistocene"),
        top_ma: 0.774,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 3,
        base_ma: 1.8,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 3,
    },
    GeologicInterval {
        name: "Gelasian",
        rank: GeologicRank::Age,
        parent: Some("Pleistocene"),
        top_ma: 1.8,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 3,
        base_ma: 2.58,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 3,
    },
    GeologicInterval {
        name: "Piacenzian",
        rank: GeologicRank::Age,
        parent: Some("Pliocene"),
        top_ma: 2.58,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 3,
        base_ma: 3.6,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Zanclean",
        rank: GeologicRank::Age,
        parent: Some("Pliocene"),
        top_ma: 3.6,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 5.333,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Messinian",
        rank: GeologicRank::Age,
        parent: Some("Miocene"),
        top_ma: 5.333,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 7.246,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Tortonian",
        rank: GeologicRank::Age,
        parent: Some("Miocene"),
        top_ma: 7.246,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 11.63,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Serravallian",
        rank: GeologicRank::Age,
        parent: Some("Miocene"),
        top_ma: 11.63,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 13.82,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Langhian",
        rank: GeologicRank::Age,
        parent: Some("Miocene"),
        top_ma: 13.82,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 15.98,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Burdigalian",
        rank: GeologicRank::Age,
        parent: Some("Miocene"),
        top_ma: 15.98,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 20.45,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Aquitanian",
        rank: GeologicRank::Age,
        parent: Some("Miocene"),
        top_ma: 20.45,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 23.04,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Chattian",
        rank: GeologicRank::Age,
        parent: Some("Oligocene"),
        top_ma: 23.04,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 27.3,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Rupelian",
        rank: GeologicRank::Age,
        parent: Some("Oligocene"),
        top_ma: 27.3,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 33.9,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 3,
    },
    GeologicInterval {
        name: "Priabonian",
        rank: GeologicRank::Age,
        parent: Some("Eocene"),
        top_ma: 33.9,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 3,
        base_ma: 37.71,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Bartonian",
        rank: GeologicRank::Age,
        parent: Some("Eocene"),
        top_ma: 37.71,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 41.03,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Lutetian",
        rank: GeologicRank::Age,
        parent: Some("Eocene"),
        top_ma: 41.03,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 48.07,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Ypresian",
        rank: GeologicRank::Age,
        parent: Some("Eocene"),
        top_ma: 48.07,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 56.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Thanetian",
        rank: GeologicRank::Age,
        parent: Some("Paleocene"),
        top_ma: 56.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 59.24,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Selandian",
        rank: GeologicRank::Age,
        parent: Some("Paleocene"),
        top_ma: 59.24,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 61.66,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Danian",
        rank: GeologicRank::Age,
        parent: Some("Paleocene"),
        top_ma: 61.66,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 66.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Maastrichtian",
        rank: GeologicRank::Age,
        parent: Some("Upper Cretaceous"),
        top_ma: 66.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 72.2,
        base_std_dev_ma: 0.2,
        base_approximate: false,
        base_figures: 3,
    },
    GeologicInterval {
        name: "Campanian",
        rank: GeologicRank::Age,
        parent: Some("Upper Cretaceous"),
        top_ma: 72.2,
        top_std_dev_ma: 0.2,
        top_approximate: false,
        top_figures: 3,
        base_ma: 83.6,
        base_std_dev_ma: 0.2,
        base_approximate: false,
        base_figures: 3,
    },
    GeologicInterval {
        name: "Santonian",
        rank: GeologicRank::Age,
        parent: Some("Upper Cretaceous"),
        top_ma: 83.6,
        top_std_dev_ma: 0.2,
        top_approximate: false,
        top_figures: 3,
        base_ma: 85.7,
        base_std_dev_ma: 0.2,
        base_approximate: false,
        base_figures: 3,
    },
    GeologicInterval {
        name: "Coniacian",
        rank: GeologicRank::Age,
        parent: Some("Upper Cretaceous"),
        top_ma: 85.7,
        top_std_dev_ma: 0.2,
        top_approximate: false,
        top_figures: 3,
        base_ma: 89.8,
        base_std_dev_ma: 0.3,
        base_approximate: false,
        base_figures: 3,
    },
    GeologicInterval {
        name: "Turonian",
        rank: GeologicRank::Age,
        parent: Some("Upper Cretaceous"),
        top_ma: 89.8,
        top_std_dev_ma: 0.3,
        top_approximate: false,
        top_figures: 3,
        base_ma: 93.9,
        base_std_dev_ma: 0.2,
        base_approximate: false,
        base_figures: 3,
    },
    GeologicInterval {
        name: "Cenomanian",
        rank: GeologicRank::Age,
        parent: Some("Upper Cretaceous"),
        top_ma: 93.9,
        top_std_dev_ma: 0.2,
        top_approximate: false,
        top_figures: 3,
        base_ma: 100.5,
        base_std_dev_ma: 0.1,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Albian",
        rank: GeologicRank::Age,
        parent: Some("Lower Cretaceous"),
        top_ma: 100.5,
        top_std_dev_ma: 0.1,
        top_approximate: false,
        top_figures: 4,
        base_ma: 113.2,
        base_std_dev_ma: 0.3,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Aptian",
        rank: GeologicRank::Age,
        parent: Some("Lower Cretaceous"),
        top_ma: 113.2,
        top_std_dev_ma: 0.3,
        top_approximate: false,
        top_figures: 4,
        base_ma: 121.4,
        base_std_dev_ma: 0.6,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Barremian",
        rank: GeologicRank::Age,
        parent: Some("Lower Cretaceous"),
        top_ma: 121.4,
        top_std_dev_ma: 0.6,
        top_approximate: false,
        top_figures: 4,
        base_ma: 125.77,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 5,
    },
    GeologicInterval {
        name: "Hauterivian",
        rank: GeologicRank::Age,
        parent: Some("Lower Cretaceous"),
        top_ma: 125.77,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 5,
        base_ma: 132.6,
        base_std_dev_ma: 0.6,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Valanginian",
        rank: GeologicRank::Age,
        parent: Some("Lower Cretaceous"),
        top_ma: 132.6,
        top_std_dev_ma: 0.6,
        top_approximate: false,
        top_figures: 4,
        base_ma: 137.05,
        base_std_dev_ma: 0.2,
        base_approximate: false,
        base_figures: 5,
    },
    GeologicInterval {
        name: "Berriasian",
        rank: GeologicRank::Age,
        parent: Some("Lower Cretaceous"),
        top_ma: 137.05,
        top_std_dev_ma: 0.2,
        top_approximate: false,
        top_figures: 5,
        base_ma: 143.1,
        base_std_dev_ma: 0.6,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Tithonian",
        rank: GeologicRank::Age,
        parent: Some("Upper Jurassic"),
        top_ma: 143.1,
        top_std_dev_ma: 0.6,
        top_approximate: false,
        top_figures: 4,
        base_ma: 149.2,
        base_std_dev_ma: 0.7,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Kimmeridgian",
        rank: GeologicRank::Age,
        parent: Some("Upper Jurassic"),
        top_ma: 149.2,
        top_std_dev_ma: 0.7,
        top_approximate: false,
        top_figures: 4,
        base_ma: 154.8,
        base_std_dev_ma: 0.8,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Oxfordian",
        rank: GeologicRank::Age,
        parent: Some("Upper Jurassic"),
        top_ma: 154.8,
        top_std_dev_ma: 0.8,
        top_approximate: false,
        top_figures: 4,
        base_ma: 161.5,
        base_std_dev_ma: 1.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Callovian",
        rank: GeologicRank::Age,
        parent: Some("Middle Jurassic"),
        top_ma: 161.5,
        top_std_dev_ma: 1.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 165.3,
        base_std_dev_ma: 1.1,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Bathonian",
        rank: GeologicRank::Age,
        parent: Some("Middle Jurassic"),
        top_ma: 165.3,
        top_std_dev_ma: 1.1,
        top_approximate: false,
        top_figures: 4,
        base_ma: 168.2,
        base_std_dev_ma: 1.2,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Bajocian",
        rank: GeologicRank::Age,
        parent: Some("Middle Jurassic"),
        top_ma: 168.2,
        top_std_dev_ma: 1.2,
        top_approximate: false,
        top_figures: 4,
        base_ma: 170.9,
        base_std_dev_ma: 0.8,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Aalenian",
        rank: GeologicRank::Age,
        parent: Some("Middle Jurassic"),
        top_ma: 170.9,
        top_std_dev_ma: 0.8,
        top_approximate: false,
        top_figures: 4,
        base_ma: 174.7,
        base_std_dev_ma: 0.8,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Toarcian",
        rank: GeologicRank::Age,
        parent: Some("Lower Jurassic"),
        top_ma: 174.7,
        top_std_dev_ma: 0.8,
        top_approximate: false,
        top_figures: 4,
        base_ma: 184.2,
        base_std_dev_ma: 0.3,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Pliensbachian",
        rank: GeologicRank::Age,
        parent: Some("Lower Jurassic"),
        top_ma: 184.2,
        top_std_dev_ma: 0.3,
        top_approximate: false,
        top_figures: 4,
        base_ma: 192.9,
        base_std_dev_ma: 0.3,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Sinemurian",
        rank: GeologicRank::Age,
        parent: Some("Lower Jurassic"),
        top_ma: 192.9,
        top_std_dev_ma: 0.3,
        top_approximate: false,
        top_figures: 4,
        base_ma: 199.5,
        base_std_dev_ma: 0.3,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Hettangian",
        rank: GeologicRank::Age,
        parent: Some("Lower Jurassic"),
        top_ma: 199.5,
        top_std_dev_ma: 0.3,
        top_approximate: false,
        top_figures: 4,
        base_ma: 201.4,
        base_std_dev_ma: 0.2,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Rhaetian",
        rank: GeologicRank::Age,
        parent: Some("Upper Triassic"),
        top_ma: 201.4,
        top_std_dev_ma: 0.2,
        top_approximate: false,
        top_figures: 4,
        base_ma: 205.7,
        base_std_dev_ma: 0.0,
        base_approximate: true,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Norian",
        rank: GeologicRank::Age,
        parent: Some("Upper Triassic"),
        top_ma: 205.7,
        top_std_dev_ma: 0.0,
        top_approximate: true,
        top_figures: 4,
        base_ma: 227.3,
        base_std_dev_ma: 0.0,
        base_approximate: true,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Carnian",
        rank: GeologicRank::Age,
        parent: Some("Upper Triassic"),
        top_ma: 227.3,
        top_std_dev_ma: 0.0,
        top_approximate: true,
        top_figures: 4,
        base_ma: 237.0,
        base_std_dev_ma: 0.0,
        base_approximate: true,
        base_figures: 3,
    },
    GeologicInterval {
        name: "Ladinian",
        rank: GeologicRank::Age,
        parent: Some("Middle Triassic"),
        top_ma: 237.0,
        top_std_dev_ma: 0.0,
        top_approximate: true,
        top_figures: 3,
        base_ma: 241.464,
        base_std_dev_ma: 0.28,
        base_approximate: false,
        base_figures: 6,
    },
    GeologicInterval {
        name: "Anisian",
        rank: GeologicRank::Age,
        parent: Some("Middle Triassic"),
        top_ma: 241.464,
        top_std_dev_ma: 0.28,
        top_approximate: false,
        top_figures: 6,
        base_ma: 247.0,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Olenekian",
        rank: GeologicRank::Age,
        parent: Some("Lower Triassic"),
        top_ma: 247.0,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 250.8,
        base_std_dev_ma: 0.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Induan",
        rank: GeologicRank::Age,
        parent: Some("Lower Triassic"),
        top_ma: 250.8,
        top_std_dev_ma: 0.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 251.902,
        base_std_dev_ma: 0.024,
        base_approximate: false,
        base_figures: 6,
    },
    GeologicInterval {
        name: "Changhsingian",
        rank: GeologicRank::Age,
        parent: Some("Lopingian"),
        top_ma: 251.902,
        top_std_dev_ma: 0.024,
        top_approximate: false,
        top_figures: 6,
        base_ma: 254.14,
        base_std_dev_ma: 0.07,
        base_approximate: false,
        base_figures: 5,
    },
    GeologicInterval {
        name: "Wuchiapingian",
        rank: GeologicRank::Age,
        parent: Some("Lopingian"),
        top_ma: 254.14,
        top_std_dev_ma: 0.07,
        top_approximate: false,
        top_figures: 5,
        base_ma: 259.857,
        base_std_dev_ma: 0.084,
        base_approximate: false,
        base_figures: 6,
    },
    GeologicInterval {
        name: "Capitanian",
        rank: GeologicRank::Age,
        parent: Some("Guadalupian"),
        top_ma: 259.857,
        top_std_dev_ma: 0.084,
        top_approximate: false,
        top_figures: 6,
        base_ma: 264.28,
        base_std_dev_ma: 0.16,
        base_approximate: false,
        base_figures: 5,
    },
    GeologicInterval {
        name: "Wordian",
        rank: GeologicRank::Age,
        parent: Some("Guadalupian"),
        top_ma: 264.28,
        top_std_dev_ma: 0.16,
        top_approximate: false,
        top_figures: 5,
        base_ma: 266.9,
        base_std_dev_ma: 0.4,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Roadian",
        rank: GeologicRank::Age,
        parent: Some("Guadalupian"),
        top_ma: 266.9,
        top_std_dev_ma: 0.4,
        top_approximate: false,
        top_figures: 4,
        base_ma: 274.4,
        base_std_dev_ma: 0.4,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Kungurian",
        rank: GeologicRank::Age,
        parent: Some("Cisuralian"),
        top_ma: 274.4,
        top_std_dev_ma: 0.4,
        top_approximate: false,
        top_figures: 4,
        base_ma: 283.3,
        base_std_dev_ma: 0.4,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Artinskian",
        rank: GeologicRank::Age,
        parent: Some("Cisuralian"),
        top_ma: 283.3,
        top_std_dev_ma: 0.4,
        top_approximate: false,
        top_figures: 4,
        base_ma: 290.1,
        base_std_dev_ma: 0.26,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Sakmarian",
        rank: GeologicRank::Age,
        parent: Some("Cisuralian"),
        top_ma: 290.1,
        top_std_dev_ma: 0.26,
        top_approximate: false,
        top_figures: 4,
        base_ma: 293.52,
        base_std_dev_ma: 0.17,
        base_approximate: false,
        base_figures: 5,
    },
    GeologicInterval {
        name: "Asselian",
        rank: GeologicRank::Age,
        parent: Some("Cisuralian"),
        top_ma: 293.52,
        top_std_dev_ma: 0.17,
        top_approximate: false,
        top_figures: 5,
        base_ma: 298.9,
        base_std_dev_ma: 0.15,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Gzhelian",
        rank: GeologicRank::Age,
        parent: Some("Upper Pennsylvanian"),
        top_ma: 298.9,
        top_std_dev_ma: 0.15,
        top_approximate: false,
        top_figures: 4,
        base_ma: 303.7,
        base_std_dev_ma: 0.1,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Kasimovian",
        rank: GeologicRank::Age,
        parent: Some("Upper Pennsylvanian"),
        top_ma: 303.7,
        top_std_dev_ma: 0.1,
        top_approximate: false,
        top_figures: 4,
        base_ma: 307.0,
        base_std_dev_ma: 0.1,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Moscovian",
        rank: GeologicRank::Age,
        parent: Some("Middle Pennsylvanian"),
        top_ma: 307.0,
        top_std_dev_ma: 0.1,
        top_approximate: false,
        top_figures: 4,
        base_ma: 315.2,
        base_std_dev_ma: 0.2,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Bashkirian",
        rank: GeologicRank::Age,
        parent: Some("Lower Pennsylvanian"),
        top_ma: 315.2,
        top_std_dev_ma: 0.2,
        top_approximate: false,
        top_figures: 4,
        base_ma: 323.4,
        base_std_dev_ma: 0.4,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Serpukhovian",
        rank: GeologicRank::Age,
        parent: Some("Upper Mississippian"),
        top_ma: 323.4,
        top_std_dev_ma: 0.4,
        top_approximate: false,
        top_figures: 4,
        base_ma: 330.3,
        base_std_dev_ma: 0.4,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Visean",
        rank: GeologicRank::Age,
        parent: Some("Middle Mississippian"),
        top_ma: 330.3,
        top_std_dev_ma: 0.4,
        top_approximate: false,
        top_figures: 4,
        base_ma: 346.7,
        base_std_dev_ma: 0.4,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Tournaisian",
        rank: GeologicRank::Age,
        parent: Some("Lower Mississippian"),
        top_ma: 346.7,
        top_std_dev_ma: 0.4,
        top_approximate: false,
        top_figures: 4,
        base_ma: 358.86,
        base_std_dev_ma: 0.19,
        base_approximate: false,
        base_figures: 5,
    },
    GeologicInterval {
        name: "Famennian",
        rank: GeologicRank::Age,
        parent: Some("Upper Devonian"),
        top_ma: 358.86,
        top_std_dev_ma: 0.19,
        top_approximate: false,
        top_figures: 5,
        base_ma: 372.15,
        base_std_dev_ma: 0.46,
        base_approximate: false,
        base_figures: 5,
    },
    GeologicInterval {
        name: "Frasnian",
        rank: GeologicRank::Age,
        parent: Some("Upper Devonian"),
        top_ma: 372.15,
        top_std_dev_ma: 0.46,
        top_approximate: false,
        top_figures: 5,
        base_ma: 382.31,
        base_std_dev_ma: 1.36,
        base_approximate: false,
        base_figures: 5,
    },
    GeologicInterval {
        name: "Givetian",
        rank: GeologicRank::Age,
        parent: Some("Middle Devonian"),
        top_ma: 382.31,
        top_std_dev_ma: 1.36,
        top_approximate: false,
        top_figures: 5,
        base_ma: 387.95,
        base_std_dev_ma: 1.04,
        base_approximate: false,
        base_figures: 5,
    },
    GeologicInterval {
        name: "Eifelian",
        rank: GeologicRank::Age,
        parent: Some("Middle Devonian"),
        top_ma: 387.95,
        top_std_dev_ma: 1.04,
        top_approximate: false,
        top_figures: 5,
        base_ma: 393.47,
        base_std_dev_ma: 0.99,
        base_approximate: false,
        base_figures: 5,
    },
    GeologicInterval {
        name: "Emsian",
        rank: GeologicRank::Age,
        parent: Some("Lower Devonian"),
        top_ma: 393.47,
        top_std_dev_ma: 0.99,
        top_approximate: false,
        top_figures: 5,
        base_ma: 410.62,
        base_std_dev_ma: 1.95,
        base_approximate: false,
        base_figures: 5,
    },
    GeologicInterval {
        name: "Pragian",
        rank: GeologicRank::Age,
        parent: Some("Lower Devonian"),
        top_ma: 410.62,
        top_std_dev_ma: 1.95,
        top_approximate: false,
        top_figures: 5,
        base_ma: 413.02,
        base_std_dev_ma: 1.91,
        base_approximate: false,
        base_figures: 5,
    },
    GeologicInterval {
        name: "Lochkovian",
        rank: GeologicRank::Age,
        parent: Some("Lower Devonian"),
        top_ma: 413.02,
        top_std_dev_ma: 1.91,
        top_approximate: false,
        top_figures: 5,
        base_ma: 419.62,
        base_std_dev_ma: 1.36,
        base_approximate: false,
        base_figures: 5,
    },
    GeologicInterval {
        name: "Ludfordian",
        rank: GeologicRank::Age,
        parent: Some("Ludlow"),
        top_ma: 422.7,
        top_std_dev_ma: 1.6,
        top_approximate: false,
        top_figures: 4,
        base_ma: 425.0,
        base_std_dev_ma: 1.5,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Gorstian",
        rank: GeologicRank::Age,
        parent: Some("Ludlow"),
        top_ma: 425.0,
        top_std_dev_ma: 1.5,
        top_approximate: false,
        top_figures: 4,
        base_ma: 426.7,
        base_std_dev_ma: 1.5,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Homerian",
        rank: GeologicRank::Age,
        parent: Some("Wenlock"),
        top_ma: 426.7,
        top_std_dev_ma: 1.5,
        top_approximate: false,
        top_figures: 4,
        base_ma: 430.6,
        base_std_dev_ma: 1.3,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Sheinwoodian",
        rank: GeologicRank::Age,
        parent: Some("Wenlock"),
        top_ma: 430.6,
        top_std_dev_ma: 1.3,
        top_approximate: false,
        top_figures: 4,
        base_ma: 432.9,
        base_std_dev_ma: 1.2,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Telychian",
        rank: GeologicRank::Age,
        parent: Some("Llandovery"),
        top_ma: 432.9,
        top_std_dev_ma: 1.2,
        top_approximate: false,
        top_figures: 4,
        base_ma: 438.6,
        base_std_dev_ma: 1.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Aeronian",
        rank: GeologicRank::Age,
        parent: Some("Llandovery"),
        top_ma: 438.6,
        top_std_dev_ma: 1.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 440.5,
        base_std_dev_ma: 1.0,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Rhuddanian",
        rank: GeologicRank::Age,
        parent: Some("Llandovery"),
        top_ma: 440.5,
        top_std_dev_ma: 1.0,
        top_approximate: false,
        top_figures: 4,
        base_ma: 443.1,
        base_std_dev_ma: 0.9,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Hirnantian",
        rank: GeologicRank::Age,
        parent: Some("Upper Ordovician"),
        top_ma: 443.1,
        top_std_dev_ma: 0.9,
        top_approximate: false,
        top_figures: 4,
        base_ma: 445.2,
        base_std_dev_ma: 0.9,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Katian",
        rank: GeologicRank::Age,
        parent: Some("Upper Ordovician"),
        top_ma: 445.2,
        top_std_dev_ma: 0.9,
        top_approximate: false,
        top_figures: 4,
        base_ma: 452.8,
        base_std_dev_ma: 0.7,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Sandbian",
        rank: GeologicRank::Age,
        parent: Some("Upper Ordovician"),
        top_ma: 452.8,
        top_std_dev_ma: 0.7,
        top_approximate: false,
        top_figures: 4,
        base_ma: 458.2,
        base_std_dev_ma: 0.7,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Darriwilian",
        rank: GeologicRank::Age,
        parent: Some("Middle Ordovician"),
        top_ma: 458.2,
        top_std_dev_ma: 0.7,
        top_approximate: false,
        top_figures: 4,
        base_ma: 469.4,
        base_std_dev_ma: 0.9,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Dapingian",
        rank: GeologicRank::Age,
        parent: Some("Middle Ordovician"),
        top_ma: 469.4,
        top_std_dev_ma: 0.9,
        top_approximate: false,
        top_figures: 4,
        base_ma: 471.3,
        base_std_dev_ma: 1.4,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Floian",
        rank: GeologicRank::Age,
        parent: Some("Lower Ordovician"),
        top_ma: 471.3,
        top_std_dev_ma: 1.4,
        top_approximate: false,
        top_figures: 4,
        base_ma: 477.1,
        base_std_dev_ma: 1.2,
        base_approximate: false,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Tremadocian",
        rank: GeologicRank::Age,
        parent: Some("Lower Ordovician"),
        top_ma: 477.1,
        top_std_dev_ma: 1.2,
        top_approximate: false,
        top_figures: 4,
        base_ma: 486.85,
        base_std_dev_ma: 1.5,
        base_approximate: false,
        base_figures: 5,
    },
    GeologicInterval {
        name: "Cambrian Stage 10",
        rank: GeologicRank::Age,
        parent: Some("Furongian"),
        top_ma: 486.85,
        top_std_dev_ma: 1.5,
        top_approximate: false,
        top_figures: 5,
        base_ma: 491.0,
        base_std_dev_ma: 0.0,
        base_approximate: true,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Jiangshanian",
        rank: GeologicRank::Age,
        parent: Some("Furongian"),
        top_ma: 491.0,
        top_std_dev_ma: 0.0,
        top_approximate: true,
        top_figures: 4,
        base_ma: 494.2,
        base_std_dev_ma: 0.0,
        base_approximate: true,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Paibian",
        rank: GeologicRank::Age,
        parent: Some("Furongian"),
        top_ma: 494.2,
        top_std_dev_ma: 0.0,
        top_approximate: true,
        top_figures: 4,
        base_ma: 497.0,
        base_std_dev_ma: 0.0,
        base_approximate: true,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Guzhangian",
        rank: GeologicRank::Age,
        parent: Some("Miaolingian"),
        top_ma: 497.0,
        top_std_dev_ma: 0.0,
        top_approximate: true,
        top_figures: 4,
        base_ma: 500.5,
        base_std_dev_ma: 0.0,
        base_approximate: true,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Drumian",
        rank: GeologicRank::Age,
        parent: Some("Miaolingian"),
        top_ma: 500.5,
        top_std_dev_ma: 0.0,
        top_approximate: true,
        top_figures: 4,
        base_ma: 504.5,
        base_std_dev_ma: 0.0,
        base_approximate: true,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Wuliuan",
        rank: GeologicRank::Age,
        parent: Some("Miaolingian"),
        top_ma: 504.5,
        top_std_dev_ma: 0.0,
        top_approximate: true,
        top_figures: 4,
        base_ma: 506.5,
        base_std_dev_ma: 0.0,
        base_approximate: true,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Cambrian Stage 4",
        rank: GeologicRank::Age,
        parent: Some("Cambrian Series 2"),
        top_ma: 506.5,
        top_std_dev_ma: 0.0,
        top_approximate: true,
        top_figures: 4,
        base_ma: 514.5,
        base_std_dev_ma: 0.0,
        base_approximate: true,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Cambrian Stage 3",
        rank: GeologicRank::Age,
        parent: Some("Cambrian Series 2"),
        top_ma: 514.5,
        top_std_dev_ma: 0.0,
        top_approximate: true,
        top_figures: 4,
        base_ma: 521.0,
        base_std_dev_ma: 0.0,
        base_approximate: true,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Cambrian Stage 2",
        rank: GeologicRank::Age,
        parent: Some("Terreneuvian"),
        top_ma: 521.0,
        top_std_dev_ma: 0.0,
        top_approximate: true,
        top_figures: 4,
        base_ma: 529.0,
        base_std_dev_ma: 0.0,
        base_approximate: true,
        base_figures: 4,
    },
    GeologicInterval {
        name: "Fortunian",
        rank: GeologicRank::Age,
        parent: Some("Terreneuvian"),
        top_ma: 529.0,
        top_std_dev_ma: 0.0,
        top_approximate: true,
        top_figures: 4,
        base_ma: 538.8,
        base_std_dev_ma: 0.6,
        base_approximate: false,
        base_figures: 4,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_rank_is_ordered_youngest_to_oldest_and_never_overlaps() {
        for rank in GeologicRank::ALL {
            for pair in intervals(*rank).windows(2) {
                assert!(
                    pair[1].top_ma >= pair[0].base_ma,
                    "{} overlaps {}",
                    pair[1].name,
                    pair[0].name
                );
                assert!(
                    pair[1].base_ma > pair[0].base_ma,
                    "{} is not older than {}",
                    pair[1].name,
                    pair[0].name
                );
            }
        }
    }

    #[test]
    fn every_interval_has_a_positive_duration() {
        for rank in GeologicRank::ALL {
            for interval in intervals(*rank) {
                assert!(
                    interval.base_ma > interval.top_ma,
                    "{} runs backwards",
                    interval.name
                );
                assert!(interval.duration_ma() > 0.0, "{}", interval.name);
            }
        }
    }

    #[test]
    fn no_boundary_carries_a_negative_uncertainty() {
        for rank in GeologicRank::ALL {
            for interval in intervals(*rank) {
                assert!(interval.top_std_dev_ma >= 0.0, "{}", interval.name);
                assert!(interval.base_std_dev_ma >= 0.0, "{}", interval.name);
                assert!(interval.top_ma.is_finite() && interval.base_ma.is_finite());
            }
        }
    }

    #[test]
    fn every_rank_tiles_its_span_without_gaps_except_the_pridoli() {
        for rank in GeologicRank::ALL {
            for pair in intervals(*rank).windows(2) {
                if pair[0].base_ma == pair[1].top_ma {
                    continue;
                }
                // The Pridoli is the one series the chart leaves without
                // stages, so the age rank has exactly one gap.
                assert_eq!(*rank, GeologicRank::Age);
                assert_eq!(pair[0].name, "Lochkovian");
                assert_eq!(pair[1].name, "Ludfordian");
            }
        }
    }

    #[test]
    fn every_rank_starts_at_the_present() {
        for rank in GeologicRank::ALL {
            let youngest = intervals(*rank).first().unwrap();
            assert_eq!(youngest.top_ma, 0.0, "{} does not reach now", youngest.name);
        }
    }

    #[test]
    fn the_chart_has_the_expected_number_of_intervals() {
        assert_eq!(EONS.len(), 4);
        assert_eq!(ERAS.len(), 10);
        assert_eq!(PERIODS.len(), 22);
        assert_eq!(EPOCHS.len(), 38);
        assert_eq!(AGES.len(), 101);
    }

    #[test]
    fn every_interval_names_a_parent_that_exists_one_rank_up() {
        for rank in GeologicRank::ALL {
            for interval in intervals(*rank) {
                match (rank, interval.parent) {
                    (GeologicRank::Eon, None) => {}
                    (GeologicRank::Eon, Some(parent)) => {
                        panic!("{} claims a parent {parent}", interval.name)
                    }
                    (_, None) => panic!("{} has no parent", interval.name),
                    (_, Some(parent)) => {
                        let found = by_name(parent)
                            .unwrap_or_else(|| panic!("{} names a missing parent", interval.name));
                        assert!(
                            found.rank < *rank,
                            "{}'s parent {parent} is not coarser",
                            interval.name
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn every_interval_lies_inside_its_parent() {
        // The Carboniferous subsystems are folded into the epoch rank, so an
        // epoch's parent is the period and the containment still has to hold.
        for rank in GeologicRank::ALL {
            for interval in intervals(*rank) {
                let Some(parent_name) = interval.parent else {
                    continue;
                };
                let parent = by_name(parent_name).unwrap();
                assert!(
                    interval.top_ma >= parent.top_ma && interval.base_ma <= parent.base_ma,
                    "{} ({}-{} Ma) escapes {} ({}-{} Ma)",
                    interval.name,
                    interval.base_ma,
                    interval.top_ma,
                    parent.name,
                    parent.base_ma,
                    parent.top_ma
                );
            }
        }
    }

    #[test]
    fn two_hundred_and_fifty_megayears_ago_was_the_triassic() {
        let chain = chain_at(250.0);
        assert_eq!(chain[0].map(|i| i.name), Some("Phanerozoic"));
        assert_eq!(chain[1].map(|i| i.name), Some("Mesozoic"));
        assert_eq!(chain[2].map(|i| i.name), Some("Triassic"));
        assert_eq!(chain[3].map(|i| i.name), Some("Lower Triassic"));
        assert_eq!(chain[4].map(|i| i.name), Some("Olenekian"));
    }

    #[test]
    fn the_cretaceous_paleogene_boundary_belongs_to_the_older_interval() {
        // 66.00 Ma is where the Cretaceous ends, so the convention is that the
        // boundary age itself is still Maastrichtian.
        assert_eq!(
            interval_at(66.00, GeologicRank::Age).map(|i| i.name),
            Some("Maastrichtian")
        );
        assert_eq!(
            interval_at(66.00, GeologicRank::Period).map(|i| i.name),
            Some("Cretaceous")
        );
        // A hair younger and it is Paleogene.
        assert_eq!(
            interval_at(65.99, GeologicRank::Period).map(|i| i.name),
            Some("Paleogene")
        );
    }

    #[test]
    fn the_present_day_is_the_meghalayan() {
        let chain = chain_at(0.0);
        assert_eq!(chain[2].map(|i| i.name), Some("Quaternary"));
        assert_eq!(chain[3].map(|i| i.name), Some("Holocene"));
        assert_eq!(chain[4].map(|i| i.name), Some("Meghalayan"));
    }

    #[test]
    fn the_archean_has_eras_but_no_periods_or_finer() {
        let chain = chain_at(3000.0);
        assert_eq!(chain[0].map(|i| i.name), Some("Archean"));
        assert_eq!(chain[1].map(|i| i.name), Some("Mesoarchean"));
        assert!(chain[2].is_none(), "the Archean has no periods");
        assert!(chain[3].is_none());
        assert!(chain[4].is_none());
    }

    #[test]
    fn the_hadean_has_no_subdivisions_at_all() {
        let chain = chain_at(4500.0);
        assert_eq!(chain[0].map(|i| i.name), Some("Hadean"));
        for finer in &chain[1..] {
            assert!(finer.is_none());
        }
    }

    #[test]
    fn a_query_before_the_solar_system_finds_nothing() {
        for rank in GeologicRank::ALL {
            assert!(interval_at(5000.0, *rank).is_none());
            assert!(interval_at(-1.0, *rank).is_none());
        }
    }

    #[test]
    fn the_pridoli_gap_leaves_the_age_rank_with_no_answer() {
        // 421 Ma is inside the Pridoli, which the chart does not subdivide.
        assert_eq!(
            interval_at(421.0, GeologicRank::Epoch).map(|i| i.name),
            Some("Pridoli")
        );
        assert!(interval_at(421.0, GeologicRank::Age).is_none());
    }

    #[test]
    fn intervals_are_reachable_by_name_at_every_rank() {
        assert_eq!(
            by_name("Phanerozoic").map(|i| i.rank),
            Some(GeologicRank::Eon)
        );
        assert_eq!(by_name("Mesozoic").map(|i| i.rank), Some(GeologicRank::Era));
        assert_eq!(
            by_name("Jurassic").map(|i| i.rank),
            Some(GeologicRank::Period)
        );
        assert_eq!(
            by_name("Holocene").map(|i| i.rank),
            Some(GeologicRank::Epoch)
        );
        assert_eq!(
            by_name("Chibanian").map(|i| i.rank),
            Some(GeologicRank::Age)
        );
        assert!(by_name("jurassic").is_none(), "lookup is case-sensitive");
        assert!(
            by_name("Vendian").is_none(),
            "a superseded name is not here"
        );
    }

    #[test]
    fn the_cretaceous_has_exactly_two_epochs() {
        let cretaceous = by_name("Cretaceous").unwrap();
        let mut names = [""; 4];
        let mut count = 0;
        for child in children(cretaceous) {
            names[count] = child.name;
            count += 1;
        }
        assert_eq!(count, 2);
        assert!(names.contains(&"Upper Cretaceous"));
        assert!(names.contains(&"Lower Cretaceous"));
    }

    #[test]
    fn the_finest_rank_has_no_children() {
        let chibanian = by_name("Chibanian").unwrap();
        assert_eq!(children(chibanian).count(), 0);
    }

    #[test]
    fn the_carboniferous_keeps_its_six_series_despite_the_missing_subsystem_rank() {
        let carboniferous = by_name("Carboniferous").unwrap();
        assert_eq!(children(carboniferous).count(), 6);
    }

    #[test]
    fn the_cretaceous_lasted_about_seventy_seven_megayears() {
        let cretaceous = by_name("Cretaceous").unwrap();
        assert!(
            (cretaceous.duration_ma() - 77.1).abs() < 0.1,
            "{} Ma",
            cretaceous.duration_ma()
        );
        let duration = cretaceous.duration().unwrap();
        let megayears = as_megayears(duration).unwrap();
        assert!((megayears - 77.1).abs() < 0.1);
        // Both boundaries carry an error bar, so the duration must too.
        assert!(duration.std_dev() > 0.0);
    }

    #[test]
    fn a_boundary_converts_to_a_span_and_back() {
        for rank in GeologicRank::ALL {
            for interval in intervals(*rank) {
                let base = interval.base().unwrap();
                let back = as_megayears(base).unwrap();
                assert!(
                    (back - interval.base_ma).abs() <= interval.base_ma.abs() * 1e-12,
                    "{} came back as {back} Ma",
                    interval.name
                );
            }
        }
    }

    #[test]
    fn approximate_boundaries_are_flagged_as_such() {
        assert!(by_name("Ediacaran").unwrap().is_approximate());
        assert!(by_name("Cryogenian").unwrap().is_approximate());
        assert!(by_name("Carnian").unwrap().is_approximate());
        assert!(!by_name("Jurassic").unwrap().is_approximate());
    }

    #[test]
    fn the_revised_triassic_and_permian_ages_are_the_current_ones() {
        // The three boundaries that moved between chart v2024/12 and v2026/06.
        assert!((by_name("Anisian").unwrap().base_ma - 247.0).abs() < 1e-9);
        assert!((by_name("Olenekian").unwrap().base_ma - 250.8).abs() < 1e-9);
        let wuchiapingian = by_name("Wuchiapingian").unwrap();
        assert!((wuchiapingian.base_ma - 259.857).abs() < 1e-9);
        assert!((wuchiapingian.base_std_dev_ma - 0.084).abs() < 1e-9);
    }

    #[test]
    fn the_base_of_the_phanerozoic_is_five_hundred_and_thirty_eight_point_eight() {
        let phanerozoic = by_name("Phanerozoic").unwrap();
        assert!((phanerozoic.base_ma - 538.8).abs() < 1e-9);
        assert!((phanerozoic.base_std_dev_ma - 0.6).abs() < 1e-9);
        assert_eq!(phanerozoic.base_figures, 4);
    }

    #[test]
    fn the_oldest_boundary_is_the_base_of_the_hadean() {
        let hadean = EONS.last().unwrap();
        assert_eq!(hadean.name, "Hadean");
        assert!((hadean.base_ma - OLDEST_BOUNDARY_MA).abs() < 1e-9);
    }

    #[test]
    fn the_chart_version_is_part_of_the_public_api() {
        assert_eq!(CHART_VERSION, "v2026/06");
        assert!(CHART_CITATION.contains("Cohen"));
        assert!(CHART_CITATION.contains("v2026/06"));
    }

    #[test]
    fn rank_names_come_in_both_vocabularies() {
        assert_eq!(GeologicRank::Period.english_name(), "period");
        assert_eq!(GeologicRank::Period.chronostratigraphic_name(), "system");
        assert_eq!(GeologicRank::Age.chronostratigraphic_name(), "stage");
        for rank in GeologicRank::ALL {
            assert!(!rank.english_name().is_empty());
            assert!(!rank.chronostratigraphic_name().is_empty());
        }
    }

    #[test]
    fn spans_before_present_round_trip_through_megayears() {
        for ma in [0.0117, 2.58, 66.0, 251.902, 538.8, 2500.0, 4567.0] {
            let span = before_present(ma, 0.0).unwrap();
            assert!((as_megayears(span).unwrap() - ma).abs() <= ma * 1e-12);
        }
    }
}
