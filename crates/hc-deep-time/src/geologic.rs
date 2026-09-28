//! The geological time scale, as data.
//!
//! Boundary ages come from the **International Chronostratigraphic Chart**
//! published by the International Commission on Stratigraphy, in two named
//! editions:
//!
//! | Edition | Identifier | Source |
//! |---|---|---|
//! | v2026/06 | [`ICS_CHART_2026_06`], `ics-chart-2026-06` | the chart at <http://www.stratigraphy.org/ICSchart/ChronostratChart2026-06.pdf> (`ics-chart-2026-06`), which asks to be cited as Cohen, Harper, Gibbard & Car, *The ICS international chronostratigraphic chart this decade*, Episodes 48, 105–115 (2025) (`cohen2025`) |
//! | v2024/12 | [`ICS_CHART_2024_12`], `ics-chart-2024-12` | the chart at <https://stratigraphy.org/ICSchart/ChronostratChart2024-12.pdf> (`ics-chart-2024-12`), which asks to be cited as Cohen, Finney, Gibbard & Fan, *The ICS International Chronostratigraphic Chart*, Episodes 36, 199–204 (2013; updated) (`cohen2013`) |
//!
//! The chart in turn takes most of its numerical ages from Gradstein et
//! al., *A Geologic Time Scale 2020* (`gradstein2020`, not read here), with
//! revisions from the relevant ICS subcommissions. The ICS's own
//! machine-readable form of the chart, `chart.ttl` in
//! <https://github.com/i-c-stratigraphy/chart> (`ics-chart-ttl`), departs
//! from the printed v2026/06 chart in a few places: `chart.ttl` has ±1.0
//! where the printed chart has ±0.9 for the uncertainty at 443.1 Ma, 23.03
//! where it has 23.04 Ma for the base of the Aquitanian, and 419.62 where
//! it has 422.7 Ma for the top of the Ludlow. The arrays follow the printed
//! chart.
//!
//! An edition is an authority's revision, and §10 of the project policy
//! keeps each under its own name rather than replacing the older. The
//! arrays below ([`EONS`] to [`AGES`], and the free functions over them)
//! are v2026/06; [`ICS_CHART_2024_12`] is the same intervals with the three
//! boundaries that the 2026-06 chart moved put back where v2024/12 prints
//! them — the Anisian base at 246.7 Ma (now 247.0), the Olenekian base at
//! 249.9 Ma (now 250.8) and the Wuchiapingian base at 259.51 ± 0.21 Ma (now
//! 259.857 ± 0.084). Those three are every age on which the two printed
//! charts differ, compared number by number on 2026-09-26; `chart.ttl` at
//! commit `00cb4f7`, whose `owl:versionInfo` is `2024-12`, differs from the
//! 2026-06 file at the same three and no others, and the 2026-06 file lists
//! them in its `skos:changeNote`s. A chronology quoted without a chart version is
//! a chronology that cannot be checked; [`CHART_VERSION`] and
//! [`ChartEdition::version`] are part of the public API for that reason.
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

/// The chart edition the arrays and free functions of this module carry.
pub const CHART_VERSION: &str = "v2026/06";

/// The full citation for [`CHART_VERSION`].
pub const CHART_CITATION: &str = "Cohen, K.M., Harper, D.A.T., Gibbard, P.L. & Car, N. (2025, updated), The ICS \
     international chronostratigraphic chart this decade, Episodes 48: 105-115; chart \
     v2026/06 at stratigraphy.org";

/// One boundary an edition prints differently from [`CHART_VERSION`].
///
/// A boundary is shared by the interval below it and the one above, at every
/// rank that has it, so it is keyed by the age v2026/06 gives it and applies
/// wherever that age appears as a top or a base.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundaryAmendment {
    /// The boundary's age in v2026/06, in Ma.
    pub current_ma: f64,
    /// The age this edition prints, in Ma.
    pub ma: f64,
    /// The uncertainty this edition prints, or zero where it gives none.
    pub std_dev_ma: f64,
    /// How many digits of `ma` this edition prints.
    pub figures: u8,
}

/// A named edition of the International Chronostratigraphic Chart.
#[derive(Debug, Clone, Copy)]
pub struct ChartEdition {
    /// A stable identifier, lowercase and hyphenated.
    pub id: &'static str,
    /// The version as the ICS writes it: `v2026/06`.
    pub version: &'static str,
    /// Where the edition's ages come from.
    pub source: &'static str,
    /// The boundaries this edition prints differently from
    /// [`CHART_VERSION`]; empty for that edition itself.
    pub amendments: &'static [BoundaryAmendment],
}

impl PartialEq for ChartEdition {
    /// Two editions are the same when they carry the same identifier.
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ChartEdition {}

impl ChartEdition {
    /// A boundary of [`CHART_VERSION`] as this edition prints it: age,
    /// uncertainty and printed figures.
    fn boundary(&self, ma: f64, std_dev_ma: f64, figures: u8) -> (f64, f64, u8) {
        self.amendments
            .iter()
            .find(|amendment| amendment.current_ma.to_bits() == ma.to_bits())
            .map_or((ma, std_dev_ma, figures), |amendment| {
                (amendment.ma, amendment.std_dev_ma, amendment.figures)
            })
    }

    /// An interval of [`CHART_VERSION`] as this edition bounds it.
    #[must_use]
    pub fn amend(&self, interval: &GeologicInterval) -> GeologicInterval {
        let (top_ma, top_std_dev_ma, top_figures) = self.boundary(
            interval.top_ma,
            interval.top_std_dev_ma,
            interval.top_figures,
        );
        let (base_ma, base_std_dev_ma, base_figures) = self.boundary(
            interval.base_ma,
            interval.base_std_dev_ma,
            interval.base_figures,
        );
        GeologicInterval {
            top_ma,
            top_std_dev_ma,
            top_figures,
            base_ma,
            base_std_dev_ma,
            base_figures,
            ..*interval
        }
    }

    /// The intervals of one rank in this edition, youngest first.
    pub fn intervals(&self, rank: GeologicRank) -> impl Iterator<Item = GeologicInterval> + '_ {
        intervals(rank).iter().map(|interval| self.amend(interval))
    }

    /// Which interval of `rank` this edition puts `ma` megayears before
    /// present in; see [`interval_at`].
    #[must_use]
    pub fn interval_at(&self, ma: f64, rank: GeologicRank) -> Option<GeologicInterval> {
        self.intervals(rank)
            .find(|interval| interval.contains_ma(ma))
    }

    /// An interval of this edition by identifier, at any rank; see
    /// [`by_id`].
    #[must_use]
    pub fn by_id(&self, id: &str) -> Option<GeologicInterval> {
        by_id(id).map(|interval| self.amend(interval))
    }
}

hc_core::catalogue! {
    type: ChartEdition,
    id: |edition| edition.id,
    provenance: |edition| edition.source,
    tests: chart_edition_catalogue,

    /// Every edition of the chart this crate carries, newest first.
    pub const CHART_EDITIONS;

    /// The edition with this identifier.
    pub fn edition_by_id;

    entries: {
        /// Chart v2026/06, the edition the module's arrays carry.
        pub const ICS_CHART_2026_06 = ChartEdition {
            id: "ics-chart-2026-06",
            version: "v2026/06",
            source: "ICS, International Chronostratigraphic Chart v2026/06, \
                     stratigraphy.org/ICSchart/ChronostratChart2026-06.pdf, retrieved 2026-09-26",
            amendments: &[],
        };

        /// Chart v2024/12: v2026/06 with three boundaries where v2024/12
        /// prints them.
        pub const ICS_CHART_2024_12 = ChartEdition {
            id: "ics-chart-2024-12",
            version: "v2024/12",
            source: "ICS, International Chronostratigraphic Chart v2024/12, \
                     stratigraphy.org/ICSchart/ChronostratChart2024-12.pdf, retrieved 2026-09-26",
            amendments: &[
                BoundaryAmendment {
                    current_ma: 247.0,
                    ma: 246.7,
                    std_dev_ma: 0.0,
                    figures: 4,
                },
                BoundaryAmendment {
                    current_ma: 250.8,
                    ma: 249.9,
                    std_dev_ma: 0.0,
                    figures: 4,
                },
                BoundaryAmendment {
                    current_ma: 259.857,
                    ma: 259.51,
                    std_dev_ma: 0.21,
                    figures: 5,
                },
            ],
        };
    }
}

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
    /// A stable identifier, lower-case and hyphenated, for callers to match
    /// on instead of the English name.
    pub id: &'static str,
    /// The interval's name as the chart spells it.
    pub name: &'static str,
    /// Where it sits in the hierarchy.
    pub rank: GeologicRank,
    /// The identifier of the containing interval one rank up, if there is
    /// one.
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

/// Look an interval up by its identifier, by
/// [`hc_core::catalogue::matches`], at any rank.
#[must_use]
pub fn by_id(id: &str) -> Option<&'static GeologicInterval> {
    GeologicRank::ALL.iter().find_map(|rank| {
        intervals(*rank)
            .iter()
            .find(|entry| hc_core::catalogue::matches(id, entry.id))
    })
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
    let id = parent.id;
    finer
        .map(intervals)
        .unwrap_or(&[])
        .iter()
        .filter(move |child| child.parent == Some(id))
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
        id: "phanerozoic",
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
        id: "proterozoic",
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
        id: "archean",
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
        id: "hadean",
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
        id: "cenozoic",
        name: "Cenozoic",
        rank: GeologicRank::Era,
        parent: Some("phanerozoic"),
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
        id: "mesozoic",
        name: "Mesozoic",
        rank: GeologicRank::Era,
        parent: Some("phanerozoic"),
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
        id: "paleozoic",
        name: "Paleozoic",
        rank: GeologicRank::Era,
        parent: Some("phanerozoic"),
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
        id: "neoproterozoic",
        name: "Neoproterozoic",
        rank: GeologicRank::Era,
        parent: Some("proterozoic"),
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
        id: "mesoproterozoic",
        name: "Mesoproterozoic",
        rank: GeologicRank::Era,
        parent: Some("proterozoic"),
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
        id: "paleoproterozoic",
        name: "Paleoproterozoic",
        rank: GeologicRank::Era,
        parent: Some("proterozoic"),
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
        id: "neoarchean",
        name: "Neoarchean",
        rank: GeologicRank::Era,
        parent: Some("archean"),
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
        id: "mesoarchean",
        name: "Mesoarchean",
        rank: GeologicRank::Era,
        parent: Some("archean"),
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
        id: "paleoarchean",
        name: "Paleoarchean",
        rank: GeologicRank::Era,
        parent: Some("archean"),
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
        id: "eoarchean",
        name: "Eoarchean",
        rank: GeologicRank::Era,
        parent: Some("archean"),
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
        id: "quaternary",
        name: "Quaternary",
        rank: GeologicRank::Period,
        parent: Some("cenozoic"),
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
        id: "neogene",
        name: "Neogene",
        rank: GeologicRank::Period,
        parent: Some("cenozoic"),
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
        id: "paleogene",
        name: "Paleogene",
        rank: GeologicRank::Period,
        parent: Some("cenozoic"),
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
        id: "cretaceous",
        name: "Cretaceous",
        rank: GeologicRank::Period,
        parent: Some("mesozoic"),
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
        id: "jurassic",
        name: "Jurassic",
        rank: GeologicRank::Period,
        parent: Some("mesozoic"),
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
        id: "triassic",
        name: "Triassic",
        rank: GeologicRank::Period,
        parent: Some("mesozoic"),
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
        id: "permian",
        name: "Permian",
        rank: GeologicRank::Period,
        parent: Some("paleozoic"),
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
        id: "carboniferous",
        name: "Carboniferous",
        rank: GeologicRank::Period,
        parent: Some("paleozoic"),
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
        id: "devonian",
        name: "Devonian",
        rank: GeologicRank::Period,
        parent: Some("paleozoic"),
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
        id: "silurian",
        name: "Silurian",
        rank: GeologicRank::Period,
        parent: Some("paleozoic"),
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
        id: "ordovician",
        name: "Ordovician",
        rank: GeologicRank::Period,
        parent: Some("paleozoic"),
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
        id: "cambrian",
        name: "Cambrian",
        rank: GeologicRank::Period,
        parent: Some("paleozoic"),
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
        id: "ediacaran",
        name: "Ediacaran",
        rank: GeologicRank::Period,
        parent: Some("neoproterozoic"),
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
        id: "cryogenian",
        name: "Cryogenian",
        rank: GeologicRank::Period,
        parent: Some("neoproterozoic"),
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
        id: "tonian",
        name: "Tonian",
        rank: GeologicRank::Period,
        parent: Some("neoproterozoic"),
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
        id: "stenian",
        name: "Stenian",
        rank: GeologicRank::Period,
        parent: Some("mesoproterozoic"),
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
        id: "ectasian",
        name: "Ectasian",
        rank: GeologicRank::Period,
        parent: Some("mesoproterozoic"),
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
        id: "calymmian",
        name: "Calymmian",
        rank: GeologicRank::Period,
        parent: Some("mesoproterozoic"),
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
        id: "statherian",
        name: "Statherian",
        rank: GeologicRank::Period,
        parent: Some("paleoproterozoic"),
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
        id: "orosirian",
        name: "Orosirian",
        rank: GeologicRank::Period,
        parent: Some("paleoproterozoic"),
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
        id: "rhyacian",
        name: "Rhyacian",
        rank: GeologicRank::Period,
        parent: Some("paleoproterozoic"),
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
        id: "siderian",
        name: "Siderian",
        rank: GeologicRank::Period,
        parent: Some("paleoproterozoic"),
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
        id: "holocene",
        name: "Holocene",
        rank: GeologicRank::Epoch,
        parent: Some("quaternary"),
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
        id: "pleistocene",
        name: "Pleistocene",
        rank: GeologicRank::Epoch,
        parent: Some("quaternary"),
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
        id: "pliocene",
        name: "Pliocene",
        rank: GeologicRank::Epoch,
        parent: Some("neogene"),
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
        id: "miocene",
        name: "Miocene",
        rank: GeologicRank::Epoch,
        parent: Some("neogene"),
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
        id: "oligocene",
        name: "Oligocene",
        rank: GeologicRank::Epoch,
        parent: Some("paleogene"),
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
        id: "eocene",
        name: "Eocene",
        rank: GeologicRank::Epoch,
        parent: Some("paleogene"),
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
        id: "paleocene",
        name: "Paleocene",
        rank: GeologicRank::Epoch,
        parent: Some("paleogene"),
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
        id: "upper-cretaceous",
        name: "Upper Cretaceous",
        rank: GeologicRank::Epoch,
        parent: Some("cretaceous"),
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
        id: "lower-cretaceous",
        name: "Lower Cretaceous",
        rank: GeologicRank::Epoch,
        parent: Some("cretaceous"),
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
        id: "upper-jurassic",
        name: "Upper Jurassic",
        rank: GeologicRank::Epoch,
        parent: Some("jurassic"),
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
        id: "middle-jurassic",
        name: "Middle Jurassic",
        rank: GeologicRank::Epoch,
        parent: Some("jurassic"),
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
        id: "lower-jurassic",
        name: "Lower Jurassic",
        rank: GeologicRank::Epoch,
        parent: Some("jurassic"),
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
        id: "upper-triassic",
        name: "Upper Triassic",
        rank: GeologicRank::Epoch,
        parent: Some("triassic"),
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
        id: "middle-triassic",
        name: "Middle Triassic",
        rank: GeologicRank::Epoch,
        parent: Some("triassic"),
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
        id: "lower-triassic",
        name: "Lower Triassic",
        rank: GeologicRank::Epoch,
        parent: Some("triassic"),
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
        id: "lopingian",
        name: "Lopingian",
        rank: GeologicRank::Epoch,
        parent: Some("permian"),
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
        id: "guadalupian",
        name: "Guadalupian",
        rank: GeologicRank::Epoch,
        parent: Some("permian"),
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
        id: "cisuralian",
        name: "Cisuralian",
        rank: GeologicRank::Epoch,
        parent: Some("permian"),
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
        id: "upper-pennsylvanian",
        name: "Upper Pennsylvanian",
        rank: GeologicRank::Epoch,
        parent: Some("carboniferous"),
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
        id: "middle-pennsylvanian",
        name: "Middle Pennsylvanian",
        rank: GeologicRank::Epoch,
        parent: Some("carboniferous"),
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
        id: "lower-pennsylvanian",
        name: "Lower Pennsylvanian",
        rank: GeologicRank::Epoch,
        parent: Some("carboniferous"),
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
        id: "upper-mississippian",
        name: "Upper Mississippian",
        rank: GeologicRank::Epoch,
        parent: Some("carboniferous"),
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
        id: "middle-mississippian",
        name: "Middle Mississippian",
        rank: GeologicRank::Epoch,
        parent: Some("carboniferous"),
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
        id: "lower-mississippian",
        name: "Lower Mississippian",
        rank: GeologicRank::Epoch,
        parent: Some("carboniferous"),
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
        id: "upper-devonian",
        name: "Upper Devonian",
        rank: GeologicRank::Epoch,
        parent: Some("devonian"),
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
        id: "middle-devonian",
        name: "Middle Devonian",
        rank: GeologicRank::Epoch,
        parent: Some("devonian"),
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
        id: "lower-devonian",
        name: "Lower Devonian",
        rank: GeologicRank::Epoch,
        parent: Some("devonian"),
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
        id: "pridoli",
        name: "Pridoli",
        rank: GeologicRank::Epoch,
        parent: Some("silurian"),
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
        id: "ludlow",
        name: "Ludlow",
        rank: GeologicRank::Epoch,
        parent: Some("silurian"),
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
        id: "wenlock",
        name: "Wenlock",
        rank: GeologicRank::Epoch,
        parent: Some("silurian"),
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
        id: "llandovery",
        name: "Llandovery",
        rank: GeologicRank::Epoch,
        parent: Some("silurian"),
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
        id: "upper-ordovician",
        name: "Upper Ordovician",
        rank: GeologicRank::Epoch,
        parent: Some("ordovician"),
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
        id: "middle-ordovician",
        name: "Middle Ordovician",
        rank: GeologicRank::Epoch,
        parent: Some("ordovician"),
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
        id: "lower-ordovician",
        name: "Lower Ordovician",
        rank: GeologicRank::Epoch,
        parent: Some("ordovician"),
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
        id: "furongian",
        name: "Furongian",
        rank: GeologicRank::Epoch,
        parent: Some("cambrian"),
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
        id: "miaolingian",
        name: "Miaolingian",
        rank: GeologicRank::Epoch,
        parent: Some("cambrian"),
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
        id: "cambrian-series-2",
        name: "Cambrian Series 2",
        rank: GeologicRank::Epoch,
        parent: Some("cambrian"),
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
        id: "terreneuvian",
        name: "Terreneuvian",
        rank: GeologicRank::Epoch,
        parent: Some("cambrian"),
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
        id: "meghalayan",
        name: "Meghalayan",
        rank: GeologicRank::Age,
        parent: Some("holocene"),
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
        id: "northgrippian",
        name: "Northgrippian",
        rank: GeologicRank::Age,
        parent: Some("holocene"),
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
        id: "greenlandian",
        name: "Greenlandian",
        rank: GeologicRank::Age,
        parent: Some("holocene"),
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
        id: "upper-pleistocene",
        name: "Upper Pleistocene",
        rank: GeologicRank::Age,
        parent: Some("pleistocene"),
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
        id: "chibanian",
        name: "Chibanian",
        rank: GeologicRank::Age,
        parent: Some("pleistocene"),
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
        id: "calabrian",
        name: "Calabrian",
        rank: GeologicRank::Age,
        parent: Some("pleistocene"),
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
        id: "gelasian",
        name: "Gelasian",
        rank: GeologicRank::Age,
        parent: Some("pleistocene"),
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
        id: "piacenzian",
        name: "Piacenzian",
        rank: GeologicRank::Age,
        parent: Some("pliocene"),
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
        id: "zanclean",
        name: "Zanclean",
        rank: GeologicRank::Age,
        parent: Some("pliocene"),
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
        id: "messinian",
        name: "Messinian",
        rank: GeologicRank::Age,
        parent: Some("miocene"),
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
        id: "tortonian",
        name: "Tortonian",
        rank: GeologicRank::Age,
        parent: Some("miocene"),
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
        id: "serravallian",
        name: "Serravallian",
        rank: GeologicRank::Age,
        parent: Some("miocene"),
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
        id: "langhian",
        name: "Langhian",
        rank: GeologicRank::Age,
        parent: Some("miocene"),
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
        id: "burdigalian",
        name: "Burdigalian",
        rank: GeologicRank::Age,
        parent: Some("miocene"),
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
        id: "aquitanian",
        name: "Aquitanian",
        rank: GeologicRank::Age,
        parent: Some("miocene"),
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
        id: "chattian",
        name: "Chattian",
        rank: GeologicRank::Age,
        parent: Some("oligocene"),
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
        id: "rupelian",
        name: "Rupelian",
        rank: GeologicRank::Age,
        parent: Some("oligocene"),
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
        id: "priabonian",
        name: "Priabonian",
        rank: GeologicRank::Age,
        parent: Some("eocene"),
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
        id: "bartonian",
        name: "Bartonian",
        rank: GeologicRank::Age,
        parent: Some("eocene"),
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
        id: "lutetian",
        name: "Lutetian",
        rank: GeologicRank::Age,
        parent: Some("eocene"),
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
        id: "ypresian",
        name: "Ypresian",
        rank: GeologicRank::Age,
        parent: Some("eocene"),
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
        id: "thanetian",
        name: "Thanetian",
        rank: GeologicRank::Age,
        parent: Some("paleocene"),
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
        id: "selandian",
        name: "Selandian",
        rank: GeologicRank::Age,
        parent: Some("paleocene"),
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
        id: "danian",
        name: "Danian",
        rank: GeologicRank::Age,
        parent: Some("paleocene"),
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
        id: "maastrichtian",
        name: "Maastrichtian",
        rank: GeologicRank::Age,
        parent: Some("upper-cretaceous"),
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
        id: "campanian",
        name: "Campanian",
        rank: GeologicRank::Age,
        parent: Some("upper-cretaceous"),
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
        id: "santonian",
        name: "Santonian",
        rank: GeologicRank::Age,
        parent: Some("upper-cretaceous"),
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
        id: "coniacian",
        name: "Coniacian",
        rank: GeologicRank::Age,
        parent: Some("upper-cretaceous"),
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
        id: "turonian",
        name: "Turonian",
        rank: GeologicRank::Age,
        parent: Some("upper-cretaceous"),
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
        id: "cenomanian",
        name: "Cenomanian",
        rank: GeologicRank::Age,
        parent: Some("upper-cretaceous"),
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
        id: "albian",
        name: "Albian",
        rank: GeologicRank::Age,
        parent: Some("lower-cretaceous"),
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
        id: "aptian",
        name: "Aptian",
        rank: GeologicRank::Age,
        parent: Some("lower-cretaceous"),
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
        id: "barremian",
        name: "Barremian",
        rank: GeologicRank::Age,
        parent: Some("lower-cretaceous"),
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
        id: "hauterivian",
        name: "Hauterivian",
        rank: GeologicRank::Age,
        parent: Some("lower-cretaceous"),
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
        id: "valanginian",
        name: "Valanginian",
        rank: GeologicRank::Age,
        parent: Some("lower-cretaceous"),
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
        id: "berriasian",
        name: "Berriasian",
        rank: GeologicRank::Age,
        parent: Some("lower-cretaceous"),
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
        id: "tithonian",
        name: "Tithonian",
        rank: GeologicRank::Age,
        parent: Some("upper-jurassic"),
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
        id: "kimmeridgian",
        name: "Kimmeridgian",
        rank: GeologicRank::Age,
        parent: Some("upper-jurassic"),
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
        id: "oxfordian",
        name: "Oxfordian",
        rank: GeologicRank::Age,
        parent: Some("upper-jurassic"),
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
        id: "callovian",
        name: "Callovian",
        rank: GeologicRank::Age,
        parent: Some("middle-jurassic"),
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
        id: "bathonian",
        name: "Bathonian",
        rank: GeologicRank::Age,
        parent: Some("middle-jurassic"),
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
        id: "bajocian",
        name: "Bajocian",
        rank: GeologicRank::Age,
        parent: Some("middle-jurassic"),
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
        id: "aalenian",
        name: "Aalenian",
        rank: GeologicRank::Age,
        parent: Some("middle-jurassic"),
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
        id: "toarcian",
        name: "Toarcian",
        rank: GeologicRank::Age,
        parent: Some("lower-jurassic"),
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
        id: "pliensbachian",
        name: "Pliensbachian",
        rank: GeologicRank::Age,
        parent: Some("lower-jurassic"),
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
        id: "sinemurian",
        name: "Sinemurian",
        rank: GeologicRank::Age,
        parent: Some("lower-jurassic"),
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
        id: "hettangian",
        name: "Hettangian",
        rank: GeologicRank::Age,
        parent: Some("lower-jurassic"),
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
        id: "rhaetian",
        name: "Rhaetian",
        rank: GeologicRank::Age,
        parent: Some("upper-triassic"),
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
        id: "norian",
        name: "Norian",
        rank: GeologicRank::Age,
        parent: Some("upper-triassic"),
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
        id: "carnian",
        name: "Carnian",
        rank: GeologicRank::Age,
        parent: Some("upper-triassic"),
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
        id: "ladinian",
        name: "Ladinian",
        rank: GeologicRank::Age,
        parent: Some("middle-triassic"),
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
        id: "anisian",
        name: "Anisian",
        rank: GeologicRank::Age,
        parent: Some("middle-triassic"),
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
        id: "olenekian",
        name: "Olenekian",
        rank: GeologicRank::Age,
        parent: Some("lower-triassic"),
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
        id: "induan",
        name: "Induan",
        rank: GeologicRank::Age,
        parent: Some("lower-triassic"),
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
        id: "changhsingian",
        name: "Changhsingian",
        rank: GeologicRank::Age,
        parent: Some("lopingian"),
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
        id: "wuchiapingian",
        name: "Wuchiapingian",
        rank: GeologicRank::Age,
        parent: Some("lopingian"),
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
        id: "capitanian",
        name: "Capitanian",
        rank: GeologicRank::Age,
        parent: Some("guadalupian"),
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
        id: "wordian",
        name: "Wordian",
        rank: GeologicRank::Age,
        parent: Some("guadalupian"),
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
        id: "roadian",
        name: "Roadian",
        rank: GeologicRank::Age,
        parent: Some("guadalupian"),
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
        id: "kungurian",
        name: "Kungurian",
        rank: GeologicRank::Age,
        parent: Some("cisuralian"),
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
        id: "artinskian",
        name: "Artinskian",
        rank: GeologicRank::Age,
        parent: Some("cisuralian"),
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
        id: "sakmarian",
        name: "Sakmarian",
        rank: GeologicRank::Age,
        parent: Some("cisuralian"),
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
        id: "asselian",
        name: "Asselian",
        rank: GeologicRank::Age,
        parent: Some("cisuralian"),
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
        id: "gzhelian",
        name: "Gzhelian",
        rank: GeologicRank::Age,
        parent: Some("upper-pennsylvanian"),
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
        id: "kasimovian",
        name: "Kasimovian",
        rank: GeologicRank::Age,
        parent: Some("upper-pennsylvanian"),
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
        id: "moscovian",
        name: "Moscovian",
        rank: GeologicRank::Age,
        parent: Some("middle-pennsylvanian"),
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
        id: "bashkirian",
        name: "Bashkirian",
        rank: GeologicRank::Age,
        parent: Some("lower-pennsylvanian"),
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
        id: "serpukhovian",
        name: "Serpukhovian",
        rank: GeologicRank::Age,
        parent: Some("upper-mississippian"),
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
        id: "visean",
        name: "Visean",
        rank: GeologicRank::Age,
        parent: Some("middle-mississippian"),
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
        id: "tournaisian",
        name: "Tournaisian",
        rank: GeologicRank::Age,
        parent: Some("lower-mississippian"),
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
        id: "famennian",
        name: "Famennian",
        rank: GeologicRank::Age,
        parent: Some("upper-devonian"),
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
        id: "frasnian",
        name: "Frasnian",
        rank: GeologicRank::Age,
        parent: Some("upper-devonian"),
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
        id: "givetian",
        name: "Givetian",
        rank: GeologicRank::Age,
        parent: Some("middle-devonian"),
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
        id: "eifelian",
        name: "Eifelian",
        rank: GeologicRank::Age,
        parent: Some("middle-devonian"),
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
        id: "emsian",
        name: "Emsian",
        rank: GeologicRank::Age,
        parent: Some("lower-devonian"),
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
        id: "pragian",
        name: "Pragian",
        rank: GeologicRank::Age,
        parent: Some("lower-devonian"),
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
        id: "lochkovian",
        name: "Lochkovian",
        rank: GeologicRank::Age,
        parent: Some("lower-devonian"),
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
        id: "ludfordian",
        name: "Ludfordian",
        rank: GeologicRank::Age,
        parent: Some("ludlow"),
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
        id: "gorstian",
        name: "Gorstian",
        rank: GeologicRank::Age,
        parent: Some("ludlow"),
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
        id: "homerian",
        name: "Homerian",
        rank: GeologicRank::Age,
        parent: Some("wenlock"),
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
        id: "sheinwoodian",
        name: "Sheinwoodian",
        rank: GeologicRank::Age,
        parent: Some("wenlock"),
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
        id: "telychian",
        name: "Telychian",
        rank: GeologicRank::Age,
        parent: Some("llandovery"),
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
        id: "aeronian",
        name: "Aeronian",
        rank: GeologicRank::Age,
        parent: Some("llandovery"),
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
        id: "rhuddanian",
        name: "Rhuddanian",
        rank: GeologicRank::Age,
        parent: Some("llandovery"),
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
        id: "hirnantian",
        name: "Hirnantian",
        rank: GeologicRank::Age,
        parent: Some("upper-ordovician"),
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
        id: "katian",
        name: "Katian",
        rank: GeologicRank::Age,
        parent: Some("upper-ordovician"),
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
        id: "sandbian",
        name: "Sandbian",
        rank: GeologicRank::Age,
        parent: Some("upper-ordovician"),
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
        id: "darriwilian",
        name: "Darriwilian",
        rank: GeologicRank::Age,
        parent: Some("middle-ordovician"),
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
        id: "dapingian",
        name: "Dapingian",
        rank: GeologicRank::Age,
        parent: Some("middle-ordovician"),
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
        id: "floian",
        name: "Floian",
        rank: GeologicRank::Age,
        parent: Some("lower-ordovician"),
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
        id: "tremadocian",
        name: "Tremadocian",
        rank: GeologicRank::Age,
        parent: Some("lower-ordovician"),
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
        id: "cambrian-stage-10",
        name: "Cambrian Stage 10",
        rank: GeologicRank::Age,
        parent: Some("furongian"),
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
        id: "jiangshanian",
        name: "Jiangshanian",
        rank: GeologicRank::Age,
        parent: Some("furongian"),
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
        id: "paibian",
        name: "Paibian",
        rank: GeologicRank::Age,
        parent: Some("furongian"),
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
        id: "guzhangian",
        name: "Guzhangian",
        rank: GeologicRank::Age,
        parent: Some("miaolingian"),
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
        id: "drumian",
        name: "Drumian",
        rank: GeologicRank::Age,
        parent: Some("miaolingian"),
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
        id: "wuliuan",
        name: "Wuliuan",
        rank: GeologicRank::Age,
        parent: Some("miaolingian"),
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
        id: "cambrian-stage-4",
        name: "Cambrian Stage 4",
        rank: GeologicRank::Age,
        parent: Some("cambrian-series-2"),
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
        id: "cambrian-stage-3",
        name: "Cambrian Stage 3",
        rank: GeologicRank::Age,
        parent: Some("cambrian-series-2"),
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
        id: "cambrian-stage-2",
        name: "Cambrian Stage 2",
        rank: GeologicRank::Age,
        parent: Some("terreneuvian"),
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
        id: "fortunian",
        name: "Fortunian",
        rank: GeologicRank::Age,
        parent: Some("terreneuvian"),
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
                        let found = by_id(parent)
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
                let parent = by_id(parent_name).unwrap();
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
    fn intervals_are_reachable_by_id_at_every_rank() {
        assert_eq!(
            by_id("phanerozoic").map(|i| i.rank),
            Some(GeologicRank::Eon)
        );
        assert_eq!(by_id("mesozoic").map(|i| i.rank), Some(GeologicRank::Era));
        assert_eq!(
            by_id("jurassic").map(|i| i.rank),
            Some(GeologicRank::Period)
        );
        assert_eq!(by_id("holocene").map(|i| i.rank), Some(GeologicRank::Epoch));
        assert_eq!(by_id("chibanian").map(|i| i.rank), Some(GeologicRank::Age));
        assert_eq!(by_id(" JURASSIC ").map(|i| i.name), Some("Jurassic"));
        assert!(
            by_id("Cambrian Stage 10").is_none(),
            "a name is not an identifier"
        );
        assert!(by_id("vendian").is_none(), "a superseded name is not here");
    }

    #[test]
    fn the_cretaceous_has_exactly_two_epochs() {
        let cretaceous = by_id("cretaceous").unwrap();
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
        let chibanian = by_id("chibanian").unwrap();
        assert_eq!(children(chibanian).count(), 0);
    }

    #[test]
    fn the_carboniferous_keeps_its_six_series_despite_the_missing_subsystem_rank() {
        let carboniferous = by_id("carboniferous").unwrap();
        assert_eq!(children(carboniferous).count(), 6);
    }

    #[test]
    fn the_cretaceous_lasted_about_seventy_seven_megayears() {
        let cretaceous = by_id("cretaceous").unwrap();
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
        assert!(by_id("ediacaran").unwrap().is_approximate());
        assert!(by_id("cryogenian").unwrap().is_approximate());
        assert!(by_id("carnian").unwrap().is_approximate());
        assert!(!by_id("jurassic").unwrap().is_approximate());
    }

    #[test]
    fn the_revised_triassic_and_permian_ages_are_the_current_ones() {
        // The three boundaries that moved between chart v2024/12 and v2026/06.
        assert!((by_id("anisian").unwrap().base_ma - 247.0).abs() < 1e-9);
        assert!((by_id("olenekian").unwrap().base_ma - 250.8).abs() < 1e-9);
        let wuchiapingian = by_id("wuchiapingian").unwrap();
        assert!((wuchiapingian.base_ma - 259.857).abs() < 1e-9);
        assert!((wuchiapingian.base_std_dev_ma - 0.084).abs() < 1e-9);
    }

    #[test]
    fn the_base_of_the_phanerozoic_is_five_hundred_and_thirty_eight_point_eight() {
        let phanerozoic = by_id("phanerozoic").unwrap();
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
    fn chart_v2024_12_keeps_its_own_three_boundaries() {
        let old = edition_by_id("ics-chart-2024-12").unwrap();
        let anisian = old.by_id("anisian").unwrap();
        assert!((anisian.base_ma - 246.7).abs() < 1e-9);
        let olenekian = old.by_id("olenekian").unwrap();
        assert!((olenekian.top_ma - 246.7).abs() < 1e-9);
        assert!((olenekian.base_ma - 249.9).abs() < 1e-9);
        assert!((old.by_id("induan").unwrap().top_ma - 249.9).abs() < 1e-9);
        let wuchiapingian = old.by_id("wuchiapingian").unwrap();
        assert!((wuchiapingian.base_ma - 259.51).abs() < 1e-9);
        assert!((wuchiapingian.base_std_dev_ma - 0.21).abs() < 1e-9);
        assert_eq!(wuchiapingian.base_figures, 5);
        // 250 Ma is Olenekian in v2026/06 and Induan in v2024/12.
        assert_eq!(
            interval_at(250.0, GeologicRank::Age).map(|i| i.name),
            Some("Olenekian")
        );
        assert_eq!(
            old.interval_at(250.0, GeologicRank::Age).map(|i| i.name),
            Some("Induan")
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn the_two_editions_differ_in_exactly_the_intervals_the_2026_06_file_notes() {
        use alloc::vec::Vec;

        // The nine intervals bounded by the three moved ages, which are also
        // the nine carrying a 2026-06 `skos:changeNote` in the ICS's
        // chart.ttl.
        let mut changed: Vec<&str> = Vec::new();
        for rank in GeologicRank::ALL {
            for (new, old) in intervals(*rank)
                .iter()
                .zip(ICS_CHART_2024_12.intervals(*rank))
            {
                if *new != old {
                    changed.push(new.name);
                }
            }
        }
        changed.sort_unstable();
        assert_eq!(
            changed,
            [
                "Anisian",
                "Capitanian",
                "Guadalupian",
                "Induan",
                "Lopingian",
                "Lower Triassic",
                "Middle Triassic",
                "Olenekian",
                "Wuchiapingian"
            ]
        );
        // v2026/06 amends nothing.
        for rank in GeologicRank::ALL {
            assert!(
                intervals(*rank)
                    .iter()
                    .zip(ICS_CHART_2026_06.intervals(*rank))
                    .all(|(a, b)| *a == b)
            );
        }
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
