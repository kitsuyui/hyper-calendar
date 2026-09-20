//! Instants that are not exactly known, and Allen's algebra over them.
//!
//! Most of history is not timestamped. A source says "in 1066", "in the third
//! century BC", "between 1180 and 1185", "3200 ± 50 BP", "before 1500", or
//! nothing at all. [`FuzzyInstant`] holds each of those as what it is,
//! instead of collapsing it to a point and pretending the rest of the digits
//! were merely not written down.
//!
//! # The support, and what an unknown bound means
//!
//! Every variant reduces to a **support**: the set of instants the value
//! could take. A support has a lower and an upper bound, each of which is
//! either known or *not known*. "Not known" is not the same as infinite: an
//! event described as "before 1500" happened at some definite finite moment
//! that this record does not give. That distinction is what makes the Allen
//! relations below return genuine sets — an unknown bound can fall on either
//! side of a known one, so several relations survive.
//!
//! # Allen's interval algebra
//!
//! James F. Allen, *Maintaining knowledge about temporal intervals*, CACM
//! 26(11), 1983, defines thirteen jointly exhaustive and mutually exclusive
//! relations between two proper intervals. This module computes, for two
//! supports, the set of relations that **could** hold given what is known.
//! A fully determined pair yields one relation; a vague pair yields several;
//! two completely unknown instants yield all thirteen.
//!
//! Two caveats, both deliberate:
//!
//! * Allen's exclusivity assumes *proper* intervals, with a start strictly
//!   before an end. An exact instant has a degenerate support, and several
//!   predicates can then hold at once — an instant at the first moment of
//!   1066 both `starts` and `meets` the year. The set is reported as it is
//!   rather than arbitrarily picking a winner.
//! * A Gaussian has no hard edge. Its support is cut at a stated multiple of
//!   σ ([`DEFAULT_SIGMA_ENVELOPE`] by default), so relations involving one
//!   are statements about that envelope, not about the tails.

use core::fmt;

use hc_core::{Duration, Instant, Tai};

use crate::error::{UncertaintyError, UncertaintyResult};
use crate::interval::DurationInterval;
use crate::quantity::Uncertain;

/// How many standard deviations of a Gaussian count as its support.
///
/// Three σ covers 99.73 % of the distribution, the conventional cut for "this
/// is the range the value is in" in archaeology and metrology alike.
pub const DEFAULT_SIGMA_ENVELOPE: f64 = 3.0;

/// The square root of 12, which converts the half-width of a uniform
/// distribution to its standard deviation.
///
/// A uniform distribution over a span of width `w` has variance `w²/12`
/// (JCGM 100:2008, the *GUM*, §4.3.7). This is the factor used when a bounded
/// but otherwise unconstrained instant has to be quoted as a `±`.
const SQRT_12: f64 = 3.464_101_615_137_754_6;

/// An instant known to some stated degree, including not at all.
///
/// ```
/// use hc_core::{Duration, Instant, Tai};
/// use hc_uncertainty::FuzzyInstant;
///
/// let battle = FuzzyInstant::resolved(
///     Instant::<Tai>::from_epoch(Duration::from_secs(0)),
///     Duration::from_days(365),
/// )
/// .unwrap();
/// assert!(!battle.is_exact());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FuzzyInstant {
    /// The instant is known exactly.
    Exact(Instant<Tai>),
    /// The instant is known only to fall within one unit of a coarser scale:
    /// a year, a century, a dynasty.
    Resolved {
        /// The first instant of the unit.
        start: Instant<Tai>,
        /// The length of the unit.
        resolution: Duration,
    },
    /// The instant is bounded but otherwise unconstrained — "between 1180 and
    /// 1185" — with no reason to prefer the middle.
    Bounded {
        /// The earliest possible instant.
        earliest: Instant<Tai>,
        /// The latest possible instant.
        latest: Instant<Tai>,
    },
    /// The instant is a measurement with a Gaussian error, as a radiocarbon
    /// or dendrochronological date is.
    Gaussian {
        /// The central value.
        centre: Instant<Tai>,
        /// The 1σ standard deviation.
        std_dev: Duration,
    },
    /// The instant is known only to be no later than this one.
    Before(Instant<Tai>),
    /// The instant is known only to be no earlier than this one.
    After(Instant<Tai>),
    /// Nothing at all is known about when this happened.
    Unknown,
}

/// The set of instants a [`FuzzyInstant`] could take.
///
/// `None` on a bound means the bound is **unknown**, not infinite: the value
/// is some definite instant that the record does not pin down on that side.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Support {
    /// The earliest possible instant, if it is known.
    pub earliest: Option<Instant<Tai>>,
    /// The latest possible instant, if it is known.
    pub latest: Option<Instant<Tai>>,
}

impl FuzzyInstant {
    /// An instant known exactly.
    #[must_use]
    pub const fn exact(instant: Instant<Tai>) -> Self {
        Self::Exact(instant)
    }

    /// An instant known only to the resolution of the unit starting at
    /// `start`.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NonPositive`] for a resolution that is
    /// zero or negative; an exact instant should be spelled
    /// [`FuzzyInstant::Exact`].
    pub fn resolved(start: Instant<Tai>, resolution: Duration) -> UncertaintyResult<Self> {
        if resolution <= Duration::ZERO {
            return Err(UncertaintyError::NonPositive);
        }
        Ok(Self::Resolved { start, resolution })
    }

    /// An instant known only to lie between two bounds, inclusive.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::EmptyInterval`] when `earliest` is after
    /// `latest`.
    pub fn bounded(earliest: Instant<Tai>, latest: Instant<Tai>) -> UncertaintyResult<Self> {
        if earliest > latest {
            return Err(UncertaintyError::EmptyInterval);
        }
        Ok(Self::Bounded { earliest, latest })
    }

    /// A measured instant with a Gaussian error.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NegativeUncertainty`] for a negative
    /// standard deviation.
    pub fn gaussian(centre: Instant<Tai>, std_dev: Duration) -> UncertaintyResult<Self> {
        if std_dev.is_negative() {
            return Err(UncertaintyError::NegativeUncertainty);
        }
        Ok(Self::Gaussian { centre, std_dev })
    }

    /// Whether the instant is pinned down completely.
    #[must_use]
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact(_))
    }

    /// Whether nothing whatsoever is known.
    #[must_use]
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// The support, cutting a Gaussian at [`DEFAULT_SIGMA_ENVELOPE`].
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::Overflow`] when a bound leaves the
    /// representable range.
    pub fn support(self) -> UncertaintyResult<Support> {
        self.support_within(DEFAULT_SIGMA_ENVELOPE)
    }

    /// The support, cutting a Gaussian at `n_sigma` standard deviations.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::NegativeUncertainty`] for a negative
    /// `n_sigma`, [`UncertaintyError::NotFinite`] for a non-finite one, and
    /// [`UncertaintyError::Overflow`] when a bound leaves the representable
    /// range.
    pub fn support_within(self, n_sigma: f64) -> UncertaintyResult<Support> {
        if n_sigma < 0.0 {
            return Err(UncertaintyError::NegativeUncertainty);
        }
        if !n_sigma.is_finite() {
            return Err(UncertaintyError::NotFinite);
        }
        Ok(match self {
            Self::Exact(instant) => Support {
                earliest: Some(instant),
                latest: Some(instant),
            },
            Self::Resolved { start, resolution } => Support {
                earliest: Some(start),
                latest: Some(start.checked_add(resolution)?),
            },
            Self::Bounded { earliest, latest } => Support {
                earliest: Some(earliest),
                latest: Some(latest),
            },
            Self::Gaussian { centre, std_dev } => {
                let envelope = std_dev.scale_f64(n_sigma)?;
                Support {
                    earliest: Some(centre.checked_sub(envelope)?),
                    latest: Some(centre.checked_add(envelope)?),
                }
            }
            Self::Before(latest) => Support {
                earliest: None,
                latest: Some(latest),
            },
            Self::After(earliest) => Support {
                earliest: Some(earliest),
                latest: None,
            },
            Self::Unknown => Support {
                earliest: None,
                latest: None,
            },
        })
    }

    /// The support as a closed interval of offsets from the 1970 epoch, when
    /// both of its bounds are known.
    ///
    /// # Errors
    ///
    /// See [`FuzzyInstant::support_within`]; also returns
    /// [`UncertaintyError::EmptyInterval`] if the bounds are crossed, which
    /// the constructors make unreachable.
    pub fn support_interval(self, n_sigma: f64) -> UncertaintyResult<Option<DurationInterval>> {
        let support = self.support_within(n_sigma)?;
        match (support.earliest, support.latest) {
            (Some(low), Some(high)) => Ok(Some(DurationInterval::new(
                low.since_epoch(),
                high.since_epoch(),
            )?)),
            _ => Ok(None),
        }
    }

    /// The single best point estimate, when there is one.
    ///
    /// Exact and Gaussian values have an obvious centre; bounded and resolved
    /// ones are reported at the midpoint of their support, which is the
    /// expectation of a uniform distribution and nothing more. Open-ended and
    /// unknown values have no estimate and return `None` rather than an
    /// invented one.
    ///
    /// # Errors
    ///
    /// Returns [`UncertaintyError::Overflow`] when the midpoint leaves the
    /// representable range.
    pub fn best_estimate(self) -> UncertaintyResult<Option<Instant<Tai>>> {
        Ok(match self {
            Self::Exact(instant) => Some(instant),
            Self::Gaussian { centre, .. } => Some(centre),
            Self::Resolved { .. } | Self::Bounded { .. } => {
                match self.support_interval(DEFAULT_SIGMA_ENVELOPE)? {
                    Some(interval) => Some(Instant::from_epoch(interval.midpoint()?)),
                    None => None,
                }
            }
            Self::Before(_) | Self::After(_) | Self::Unknown => None,
        })
    }

    /// The width of the support, when both bounds are known.
    ///
    /// # Errors
    ///
    /// See [`FuzzyInstant::support_within`].
    pub fn span(self) -> UncertaintyResult<Option<Duration>> {
        match self.support_interval(DEFAULT_SIGMA_ENVELOPE)? {
            Some(interval) => Ok(Some(interval.width()?)),
            None => Ok(None),
        }
    }

    /// The instant as a Gaussian number of seconds since the 1970 epoch.
    ///
    /// A bounded or resolved value is converted with the uniform-distribution
    /// factor `w/√12` from the *GUM*, which is the honest way to quote a flat
    /// range as a `±`: it is the actual standard deviation of "somewhere in
    /// here, no preference". Open-ended and unknown values have no such
    /// summary and return `None`.
    ///
    /// # Errors
    ///
    /// See [`FuzzyInstant::support_within`], plus the errors of
    /// [`Uncertain::new`].
    pub fn as_uncertain_seconds(self) -> UncertaintyResult<Option<Uncertain>> {
        Ok(match self {
            Self::Exact(instant) => Some(Uncertain::exact(instant.since_epoch().as_secs_f64())?),
            Self::Gaussian { centre, std_dev } => Some(Uncertain::new(
                centre.since_epoch().as_secs_f64(),
                std_dev.as_secs_f64(),
            )?),
            Self::Resolved { .. } | Self::Bounded { .. } => {
                match self.support_interval(DEFAULT_SIGMA_ENVELOPE)? {
                    Some(interval) => {
                        let centre = interval.midpoint()?.as_secs_f64();
                        let width = interval.width()?.as_secs_f64();
                        Some(Uncertain::new(centre, width / SQRT_12)?)
                    }
                    None => None,
                }
            }
            Self::Before(_) | Self::After(_) | Self::Unknown => None,
        })
    }

    /// The Allen relations that could hold between the two supports.
    ///
    /// # Errors
    ///
    /// See [`FuzzyInstant::support_within`].
    pub fn relations(self, other: Self) -> UncertaintyResult<RelationSet> {
        self.relations_within(other, DEFAULT_SIGMA_ENVELOPE)
    }

    /// The Allen relations that could hold, cutting Gaussians at `n_sigma`.
    ///
    /// # Errors
    ///
    /// See [`FuzzyInstant::support_within`].
    pub fn relations_within(self, other: Self, n_sigma: f64) -> UncertaintyResult<RelationSet> {
        Ok(possible_relations(
            self.support_within(n_sigma)?,
            other.support_within(n_sigma)?,
        ))
    }

    /// Whether `self` is certainly wholly earlier than `other`.
    ///
    /// # Errors
    ///
    /// See [`FuzzyInstant::support_within`].
    pub fn definitely_before(self, other: Self) -> UncertaintyResult<bool> {
        Ok(self.relations(other)? == RelationSet::only(AllenRelation::Before))
    }

    /// Whether `self` could be wholly earlier than `other`.
    ///
    /// # Errors
    ///
    /// See [`FuzzyInstant::support_within`].
    pub fn possibly_before(self, other: Self) -> UncertaintyResult<bool> {
        Ok(self.relations(other)?.contains(AllenRelation::Before))
    }

    /// Whether `self` is certainly wholly later than `other`.
    ///
    /// # Errors
    ///
    /// See [`FuzzyInstant::support_within`].
    pub fn definitely_after(self, other: Self) -> UncertaintyResult<bool> {
        Ok(self.relations(other)? == RelationSet::only(AllenRelation::After))
    }

    /// Whether `self` could be wholly later than `other`.
    ///
    /// # Errors
    ///
    /// See [`FuzzyInstant::support_within`].
    pub fn possibly_after(self, other: Self) -> UncertaintyResult<bool> {
        Ok(self.relations(other)?.contains(AllenRelation::After))
    }

    /// Whether the two supports could share an instant.
    ///
    /// # Errors
    ///
    /// See [`FuzzyInstant::support_within`].
    pub fn possibly_concurrent(self, other: Self) -> UncertaintyResult<bool> {
        let relations = self.relations(other)?;
        let separated =
            RelationSet::only(AllenRelation::Before).union(RelationSet::only(AllenRelation::After));
        Ok(!relations.difference(separated).is_empty())
    }
}

impl fmt::Display for FuzzyInstant {
    /// A terse human summary in seconds since the 1970 TAI epoch. Calendar
    /// rendering belongs to `hc-format`, which knows about years.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact(instant) => write!(f, "{}", instant.since_epoch()),
            Self::Resolved { start, resolution } => {
                write!(f, "{} +{}", start.since_epoch(), resolution)
            }
            Self::Bounded { earliest, latest } => {
                write!(f, "{}..{}", earliest.since_epoch(), latest.since_epoch())
            }
            Self::Gaussian { centre, std_dev } => {
                write!(f, "{} ± {}", centre.since_epoch(), std_dev)
            }
            Self::Before(latest) => write!(f, "..{}", latest.since_epoch()),
            Self::After(earliest) => write!(f, "{}..", earliest.since_epoch()),
            Self::Unknown => f.write_str("unknown"),
        }
    }
}

/// One of Allen's thirteen relations between two intervals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AllenRelation {
    /// `A` ends strictly before `B` starts.
    Before,
    /// `A` ends exactly where `B` starts.
    Meets,
    /// `A` starts first, they share a stretch, and `A` ends first.
    Overlaps,
    /// They start together and `A` ends first.
    Starts,
    /// `A` lies strictly inside `B`.
    During,
    /// `A` starts later and they end together.
    Finishes,
    /// The two intervals are identical.
    Equals,
    /// The converse of [`AllenRelation::Finishes`].
    FinishedBy,
    /// The converse of [`AllenRelation::During`]: `A` strictly contains `B`.
    Contains,
    /// The converse of [`AllenRelation::Starts`].
    StartedBy,
    /// The converse of [`AllenRelation::Overlaps`].
    OverlappedBy,
    /// The converse of [`AllenRelation::Meets`].
    MetBy,
    /// The converse of [`AllenRelation::Before`].
    After,
}

impl AllenRelation {
    /// All thirteen relations, in Allen's own order.
    pub const ALL: [Self; 13] = [
        Self::Before,
        Self::Meets,
        Self::Overlaps,
        Self::Starts,
        Self::During,
        Self::Finishes,
        Self::Equals,
        Self::FinishedBy,
        Self::Contains,
        Self::StartedBy,
        Self::OverlappedBy,
        Self::MetBy,
        Self::After,
    ];

    /// The bit index this relation occupies in a [`RelationSet`].
    #[must_use]
    pub const fn index(self) -> u8 {
        match self {
            Self::Before => 0,
            Self::Meets => 1,
            Self::Overlaps => 2,
            Self::Starts => 3,
            Self::During => 4,
            Self::Finishes => 5,
            Self::Equals => 6,
            Self::FinishedBy => 7,
            Self::Contains => 8,
            Self::StartedBy => 9,
            Self::OverlappedBy => 10,
            Self::MetBy => 11,
            Self::After => 12,
        }
    }

    /// The relation with its arguments swapped.
    #[must_use]
    pub const fn converse(self) -> Self {
        match self {
            Self::Before => Self::After,
            Self::Meets => Self::MetBy,
            Self::Overlaps => Self::OverlappedBy,
            Self::Starts => Self::StartedBy,
            Self::During => Self::Contains,
            Self::Finishes => Self::FinishedBy,
            Self::Equals => Self::Equals,
            Self::FinishedBy => Self::Finishes,
            Self::Contains => Self::During,
            Self::StartedBy => Self::Starts,
            Self::OverlappedBy => Self::Overlaps,
            Self::MetBy => Self::Meets,
            Self::After => Self::Before,
        }
    }

    /// Allen's shorthand symbol, as used in the 1983 paper's tables.
    #[must_use]
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Before => "<",
            Self::Meets => "m",
            Self::Overlaps => "o",
            Self::Starts => "s",
            Self::During => "d",
            Self::Finishes => "f",
            Self::Equals => "=",
            Self::FinishedBy => "fi",
            Self::Contains => "di",
            Self::StartedBy => "si",
            Self::OverlappedBy => "oi",
            Self::MetBy => "mi",
            Self::After => ">",
        }
    }

    /// The English name.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match self {
            Self::Before => "before",
            Self::Meets => "meets",
            Self::Overlaps => "overlaps",
            Self::Starts => "starts",
            Self::During => "during",
            Self::Finishes => "finishes",
            Self::Equals => "equals",
            Self::FinishedBy => "finished by",
            Self::Contains => "contains",
            Self::StartedBy => "started by",
            Self::OverlappedBy => "overlapped by",
            Self::MetBy => "met by",
            Self::After => "after",
        }
    }

    /// Whether this relation holds for the four endpoint positions
    /// `[a_start, a_end, b_start, b_end]`.
    const fn holds(self, points: [i32; 4]) -> bool {
        let (a1, a2, b1, b2) = (points[0], points[1], points[2], points[3]);
        match self {
            Self::Before => a2 < b1,
            Self::Meets => a2 == b1,
            Self::Overlaps => a1 < b1 && b1 < a2 && a2 < b2,
            Self::Starts => a1 == b1 && a2 < b2,
            Self::During => b1 < a1 && a2 < b2,
            Self::Finishes => b1 < a1 && a2 == b2,
            Self::Equals => a1 == b1 && a2 == b2,
            Self::FinishedBy => a1 < b1 && a2 == b2,
            Self::Contains => a1 < b1 && b2 < a2,
            Self::StartedBy => a1 == b1 && b2 < a2,
            Self::OverlappedBy => b1 < a1 && a1 < b2 && b2 < a2,
            Self::MetBy => a1 == b2,
            Self::After => a1 > b2,
        }
    }
}

impl fmt::Display for AllenRelation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.english_name())
    }
}

/// A set of Allen relations, held as a thirteen-bit mask.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct RelationSet(u16);

impl RelationSet {
    /// The set with nothing in it, which means "no relation is possible" —
    /// a contradiction, never produced by [`FuzzyInstant::relations`].
    pub const EMPTY: Self = Self(0);

    /// All thirteen relations: what two completely unknown instants yield.
    pub const ALL: Self = Self(0b1_1111_1111_1111);

    /// The set holding exactly one relation.
    #[must_use]
    pub const fn only(relation: AllenRelation) -> Self {
        Self(1 << relation.index())
    }

    /// Whether the relation is in the set.
    #[must_use]
    pub const fn contains(self, relation: AllenRelation) -> bool {
        self.0 & (1 << relation.index()) != 0
    }

    /// The set with `relation` added.
    #[must_use]
    pub const fn with(self, relation: AllenRelation) -> Self {
        Self(self.0 | (1 << relation.index()))
    }

    /// Everything in either set.
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Everything in both sets.
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// Everything in `self` that is not in `other`.
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    /// Whether the set holds nothing.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// How many relations are in the set.
    #[must_use]
    pub const fn len(self) -> u32 {
        self.0.count_ones()
    }

    /// Whether every relation of `self` is also in `other`.
    #[must_use]
    pub const fn is_subset_of(self, other: Self) -> bool {
        self.0 & !other.0 == 0
    }

    /// Whether the answer is unambiguous, and if so what it is.
    #[must_use]
    pub fn single(self) -> Option<AllenRelation> {
        if self.len() != 1 {
            return None;
        }
        AllenRelation::ALL.into_iter().find(|r| self.contains(*r))
    }

    /// The set of converses, which is the relation set of the swapped pair.
    #[must_use]
    pub fn converse(self) -> Self {
        let mut result = Self::EMPTY;
        for relation in AllenRelation::ALL {
            if self.contains(relation) {
                result = result.with(relation.converse());
            }
        }
        result
    }

    /// The relations in the set, in Allen's order.
    #[must_use]
    pub const fn iter(self) -> RelationSetIter {
        RelationSetIter { set: self, next: 0 }
    }
}

impl IntoIterator for RelationSet {
    type Item = AllenRelation;
    type IntoIter = RelationSetIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator over the members of a [`RelationSet`].
#[derive(Debug, Clone, Copy)]
pub struct RelationSetIter {
    set: RelationSet,
    next: usize,
}

impl Iterator for RelationSetIter {
    type Item = AllenRelation;

    fn next(&mut self) -> Option<Self::Item> {
        while self.next < AllenRelation::ALL.len() {
            let relation = AllenRelation::ALL[self.next];
            self.next += 1;
            if self.set.contains(relation) {
                return Some(relation);
            }
        }
        None
    }
}

impl fmt::Display for RelationSet {
    /// Renders as Allen's symbols in braces, `{< m o}`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("{")?;
        let mut first = true;
        for relation in self.iter() {
            if !first {
                f.write_str(" ")?;
            }
            first = false;
            f.write_str(relation.symbol())?;
        }
        f.write_str("}")
    }
}

/// Positions are spaced ten apart so that four free endpoints can be placed
/// strictly between two known ones without colliding.
const GAP: i32 = 10;

/// The relations that survive every placement of the unknown bounds.
///
/// Both supports are reduced to four endpoint *positions* on an ordinal
/// scale. Known bounds take fixed positions; unknown bounds range over every
/// position that is order-theoretically distinct — below everything, equal to
/// some known bound, or in one of the gaps. Because the relations depend only
/// on the order of the endpoints and not on their magnitudes, enumerating
/// that finite ordinal space is exactly equivalent to quantifying over the
/// real line, and the result is both sound and minimal.
fn possible_relations(a: Support, b: Support) -> RelationSet {
    let mut values = [Duration::ZERO; 4];
    let mut count = 0usize;
    for instant in [a.earliest, a.latest, b.earliest, b.latest]
        .into_iter()
        .flatten()
    {
        insert_sorted(&mut values, &mut count, instant.since_epoch());
    }
    if count == 0 {
        return RelationSet::ALL;
    }

    let mut candidates = [0i32; 24];
    let candidate_count = fill_candidates(&mut candidates, count);

    let bounds = [a.earliest, a.latest, b.earliest, b.latest];
    let mut fixed = [None; 4];
    let mut free_slot = [0usize; 4];
    let mut free_count = 0usize;
    for (index, bound) in bounds.iter().enumerate() {
        let position =
            bound.and_then(|instant| position_of(&values[..count], instant.since_epoch()));
        match position {
            Some(value) => fixed[index] = Some(value),
            None => {
                free_slot[index] = free_count;
                free_count += 1;
            }
        }
    }

    let mut digits = [0usize; 4];
    let mut found = RelationSet::EMPTY;
    loop {
        let mut points = [0i32; 4];
        for ((point, position), slot) in points.iter_mut().zip(fixed.iter()).zip(free_slot.iter()) {
            *point = match position {
                Some(value) => *value,
                None => candidates[digits[*slot]],
            };
        }
        if points[0] <= points[1] && points[2] <= points[3] {
            for relation in AllenRelation::ALL {
                if relation.holds(points) {
                    found = found.with(relation);
                }
            }
        }
        if !advance(&mut digits, free_count, candidate_count) {
            break;
        }
    }
    found
}

/// Insert into a sorted fixed-size array, skipping duplicates.
fn insert_sorted(values: &mut [Duration; 4], count: &mut usize, value: Duration) {
    let mut index = 0;
    while index < *count {
        if values[index] == value {
            return;
        }
        if values[index] > value {
            break;
        }
        index += 1;
    }
    let mut shift = *count;
    while shift > index {
        values[shift] = values[shift - 1];
        shift -= 1;
    }
    values[index] = value;
    *count += 1;
}

/// The ordinal position of a known bound.
fn position_of(values: &[Duration], value: Duration) -> Option<i32> {
    values
        .iter()
        .position(|candidate| *candidate == value)
        .map(|index| index as i32 * GAP)
}

/// Every position an unknown bound could occupy, given `count` known values.
///
/// Four slots below the first known value and four in each gap above one are
/// enough for the at most four unknown bounds to be placed in any distinct
/// order, which is all the relations can see.
fn fill_candidates(candidates: &mut [i32; 24], count: usize) -> usize {
    let mut length = 0;
    for offset in 1..=4 {
        candidates[length] = -offset;
        length += 1;
    }
    for index in 0..count {
        let base = index as i32 * GAP;
        candidates[length] = base;
        length += 1;
        for offset in 1..=4 {
            candidates[length] = base + offset;
            length += 1;
        }
    }
    length
}

/// Step the odometer over the free bounds; `false` once it has wrapped.
fn advance(digits: &mut [usize; 4], free_count: usize, radix: usize) -> bool {
    let mut index = 0;
    while index < free_count {
        digits[index] += 1;
        if digits[index] < radix {
            return true;
        }
        digits[index] = 0;
        index += 1;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A TAI instant at a whole number of days after the 1970 epoch. The
    /// tests only need a consistent timeline, not calendar accuracy.
    fn day(number: i64) -> Instant<Tai> {
        Instant::from_epoch(Duration::from_days(number))
    }

    fn year(number: i64) -> FuzzyInstant {
        FuzzyInstant::resolved(day(number * 365), Duration::from_days(365))
            .expect("a year is a positive resolution")
    }

    #[test]
    fn an_exact_instant_has_a_degenerate_support() {
        let value = FuzzyInstant::exact(day(10));
        let support = value.support().unwrap();
        assert_eq!(support.earliest, Some(day(10)));
        assert_eq!(support.latest, Some(day(10)));
        assert!(value.is_exact());
    }

    #[test]
    fn a_resolved_instant_spans_its_unit() {
        let in_1066 = year(1066);
        let support = in_1066.support().unwrap();
        assert_eq!(support.earliest, Some(day(1066 * 365)));
        assert_eq!(support.latest, Some(day(1066 * 365 + 365)));
    }

    #[test]
    fn a_zero_resolution_is_rejected() {
        assert_eq!(
            FuzzyInstant::resolved(day(0), Duration::ZERO),
            Err(UncertaintyError::NonPositive)
        );
    }

    #[test]
    fn crossed_bounds_are_rejected() {
        assert_eq!(
            FuzzyInstant::bounded(day(10), day(1)),
            Err(UncertaintyError::EmptyInterval)
        );
    }

    #[test]
    fn a_negative_standard_deviation_is_rejected() {
        assert_eq!(
            FuzzyInstant::gaussian(day(0), Duration::from_secs(-1)),
            Err(UncertaintyError::NegativeUncertainty)
        );
    }

    #[test]
    fn a_gaussian_support_is_cut_at_three_sigma_by_default() {
        let value = FuzzyInstant::gaussian(day(1_000), Duration::from_days(10)).unwrap();
        let support = value.support().unwrap();
        assert_eq!(support.earliest, Some(day(970)));
        assert_eq!(support.latest, Some(day(1_030)));
    }

    #[test]
    fn a_wider_envelope_widens_the_gaussian_support() {
        let value = FuzzyInstant::gaussian(day(1_000), Duration::from_days(10)).unwrap();
        let support = value.support_within(1.0).unwrap();
        assert_eq!(support.earliest, Some(day(990)));
        assert_eq!(support.latest, Some(day(1_010)));
    }

    #[test]
    fn an_open_ended_instant_has_one_unknown_bound() {
        let before = FuzzyInstant::Before(day(500));
        let after = FuzzyInstant::After(day(500));
        assert_eq!(before.support().unwrap().earliest, None);
        assert_eq!(before.support().unwrap().latest, Some(day(500)));
        assert_eq!(after.support().unwrap().earliest, Some(day(500)));
        assert_eq!(after.support().unwrap().latest, None);
    }

    #[test]
    fn an_unknown_instant_has_no_bounds_at_all() {
        let support = FuzzyInstant::Unknown.support().unwrap();
        assert_eq!(support.earliest, None);
        assert_eq!(support.latest, None);
        assert!(FuzzyInstant::Unknown.is_unknown());
    }

    #[test]
    fn a_negative_sigma_envelope_is_rejected() {
        assert_eq!(
            FuzzyInstant::exact(day(0)).support_within(-1.0),
            Err(UncertaintyError::NegativeUncertainty)
        );
    }

    #[test]
    fn two_exact_instants_relate_by_before_or_after_or_equals_only() {
        let early = FuzzyInstant::exact(day(1));
        let late = FuzzyInstant::exact(day(2));
        assert_eq!(
            early.relations(late).unwrap(),
            RelationSet::only(AllenRelation::Before)
        );
        assert_eq!(
            late.relations(early).unwrap(),
            RelationSet::only(AllenRelation::After)
        );
        assert!(
            early
                .relations(early)
                .unwrap()
                .contains(AllenRelation::Equals)
        );
    }

    #[test]
    fn consecutive_years_meet() {
        let relations = year(1066).relations(year(1067)).unwrap();
        assert_eq!(relations, RelationSet::only(AllenRelation::Meets));
        assert_eq!(relations.single(), Some(AllenRelation::Meets));
    }

    #[test]
    fn a_year_lies_during_a_bounded_span_that_encloses_it() {
        let span = FuzzyInstant::bounded(day(1000 * 365), day(1100 * 365)).unwrap();
        let relations = year(1066).relations(span).unwrap();
        assert_eq!(relations, RelationSet::only(AllenRelation::During));
        assert_eq!(
            span.relations(year(1066)).unwrap(),
            RelationSet::only(AllenRelation::Contains)
        );
    }

    #[test]
    fn the_relation_set_of_the_swapped_pair_is_the_converse_set() {
        let a = FuzzyInstant::Before(day(600));
        let b = year(1);
        let forward = a.relations(b).unwrap();
        let backward = b.relations(a).unwrap();
        assert_eq!(forward.converse(), backward);
        assert_eq!(backward.converse(), forward);
    }

    #[test]
    fn two_unknown_instants_admit_every_relation() {
        let relations = FuzzyInstant::Unknown
            .relations(FuzzyInstant::Unknown)
            .unwrap();
        assert_eq!(relations, RelationSet::ALL);
        assert_eq!(relations.len(), 13);
    }

    #[test]
    fn an_open_ended_bound_leaves_several_relations_possible() {
        // "Some time before day 600" against "day 300 to day 400": the
        // unknown start can sit before, inside or after the known span.
        let open = FuzzyInstant::Before(day(600));
        let span = FuzzyInstant::bounded(day(300), day(400)).unwrap();
        let relations = open.relations(span).unwrap();
        assert!(relations.len() > 1, "got {relations}");
        assert!(relations.contains(AllenRelation::After));
        assert!(relations.contains(AllenRelation::Contains));
        assert!(relations.contains(AllenRelation::OverlappedBy));
        assert!(relations.contains(AllenRelation::MetBy));
        assert!(relations.contains(AllenRelation::StartedBy));
        // It cannot possibly end before the span starts: its end is day 600.
        assert!(!relations.contains(AllenRelation::Before));
    }

    #[test]
    fn a_bounded_span_is_never_definitely_before_an_overlapping_one() {
        let a = FuzzyInstant::bounded(day(1180), day(1185)).unwrap();
        let b = FuzzyInstant::bounded(day(1183), day(1190)).unwrap();
        assert!(!a.definitely_before(b).unwrap());
        assert!(a.possibly_concurrent(b).unwrap());
    }

    #[test]
    fn a_clearly_separated_pair_is_definitely_ordered() {
        let a = FuzzyInstant::bounded(day(1180), day(1185)).unwrap();
        let b = FuzzyInstant::bounded(day(1200), day(1210)).unwrap();
        assert!(a.definitely_before(b).unwrap());
        assert!(b.definitely_after(a).unwrap());
        assert!(!a.possibly_after(b).unwrap());
        assert!(!a.possibly_concurrent(b).unwrap());
    }

    #[test]
    fn an_exact_instant_inside_a_year_lies_during_it() {
        let moment = FuzzyInstant::exact(day(1066 * 365 + 100));
        let relations = moment.relations(year(1066)).unwrap();
        assert_eq!(relations, RelationSet::only(AllenRelation::During));
    }

    #[test]
    fn a_degenerate_support_at_a_shared_endpoint_satisfies_several_relations() {
        // Allen's exclusivity assumes proper intervals; a point that sits on
        // the first instant of a year both starts it and meets it.
        let moment = FuzzyInstant::exact(day(1066 * 365));
        let relations = moment.relations(year(1066)).unwrap();
        assert!(relations.contains(AllenRelation::Starts));
        assert!(relations.contains(AllenRelation::Meets));
        assert!(relations.len() >= 2);
    }

    #[test]
    fn every_relation_is_its_own_converse_twice_over() {
        for relation in AllenRelation::ALL {
            assert_eq!(relation.converse().converse(), relation);
        }
    }

    #[test]
    fn equals_is_the_only_self_converse_relation() {
        let self_converse = AllenRelation::ALL
            .into_iter()
            .filter(|relation| relation.converse() == *relation)
            .count();
        assert_eq!(self_converse, 1);
        assert_eq!(AllenRelation::Equals.converse(), AllenRelation::Equals);
    }

    #[test]
    fn indices_and_symbols_are_unique_across_the_thirteen() {
        let mut seen = 0u16;
        for relation in AllenRelation::ALL {
            let bit = 1u16 << relation.index();
            assert_eq!(seen & bit, 0, "duplicate index for {relation}");
            seen |= bit;
            assert!(!relation.symbol().is_empty());
            assert!(!relation.english_name().is_empty());
        }
        assert_eq!(seen, RelationSet::ALL.0);
    }

    #[test]
    fn set_operations_behave_like_sets() {
        let before = RelationSet::only(AllenRelation::Before);
        let after = RelationSet::only(AllenRelation::After);
        let both = before.union(after);
        assert_eq!(both.len(), 2);
        assert_eq!(both.intersection(before), before);
        assert_eq!(both.difference(before), after);
        assert!(before.is_subset_of(both));
        assert!(!both.is_subset_of(before));
        assert!(RelationSet::EMPTY.is_empty());
        assert_eq!(both.single(), None);
        assert_eq!(before.single(), Some(AllenRelation::Before));
    }

    #[test]
    fn the_iterator_walks_the_set_in_allens_order() {
        let set = RelationSet::EMPTY
            .with(AllenRelation::After)
            .with(AllenRelation::Before)
            .with(AllenRelation::Equals);
        let mut walked = [AllenRelation::Before; 3];
        let mut length = 0;
        for relation in set {
            walked[length] = relation;
            length += 1;
        }
        assert_eq!(length, 3);
        assert_eq!(walked[0], AllenRelation::Before);
        assert_eq!(walked[1], AllenRelation::Equals);
        assert_eq!(walked[2], AllenRelation::After);
    }

    #[test]
    fn the_relation_set_renders_as_allen_symbols() {
        #[cfg(feature = "alloc")]
        {
            use alloc::string::ToString as _;
            let set = RelationSet::only(AllenRelation::Before).with(AllenRelation::Meets);
            assert_eq!(set.to_string(), "{< m}");
            assert_eq!(RelationSet::EMPTY.to_string(), "{}");
        }
    }

    #[test]
    fn the_best_estimate_of_a_span_is_its_midpoint() {
        let span = FuzzyInstant::bounded(day(1180), day(1190)).unwrap();
        assert_eq!(span.best_estimate().unwrap(), Some(day(1185)));
    }

    #[test]
    fn an_open_ended_instant_has_no_best_estimate() {
        assert_eq!(FuzzyInstant::Before(day(1)).best_estimate().unwrap(), None);
        assert_eq!(FuzzyInstant::After(day(1)).best_estimate().unwrap(), None);
        assert_eq!(FuzzyInstant::Unknown.best_estimate().unwrap(), None);
    }

    #[test]
    fn the_span_of_a_year_is_a_year() {
        assert_eq!(year(1066).span().unwrap(), Some(Duration::from_days(365)));
        assert_eq!(
            FuzzyInstant::exact(day(0)).span().unwrap(),
            Some(Duration::ZERO)
        );
        assert_eq!(FuzzyInstant::Unknown.span().unwrap(), None);
    }

    #[test]
    fn a_flat_range_becomes_a_gaussian_by_the_root_twelve_rule() {
        let span = FuzzyInstant::bounded(day(0), day(12)).unwrap();
        let quoted = span.as_uncertain_seconds().unwrap().unwrap();
        let width = Duration::from_days(12).as_secs_f64();
        assert!((quoted.std_dev - width / SQRT_12).abs() < 1e-3);
        assert!((quoted.value - Duration::from_days(6).as_secs_f64()).abs() < 1e-3);
    }

    #[test]
    fn a_gaussian_instant_keeps_its_sigma_when_quoted() {
        let value = FuzzyInstant::gaussian(day(100), Duration::from_days(5)).unwrap();
        let quoted = value.as_uncertain_seconds().unwrap().unwrap();
        assert!((quoted.std_dev - Duration::from_days(5).as_secs_f64()).abs() < 1e-6);
    }

    #[test]
    fn an_open_ended_instant_cannot_be_quoted_as_a_gaussian() {
        assert_eq!(FuzzyInstant::Unknown.as_uncertain_seconds().unwrap(), None);
        assert_eq!(
            FuzzyInstant::After(day(0)).as_uncertain_seconds().unwrap(),
            None
        );
    }

    #[test]
    fn the_support_interval_is_absent_when_a_bound_is_unknown() {
        assert!(
            FuzzyInstant::Before(day(1))
                .support_interval(3.0)
                .unwrap()
                .is_none()
        );
        assert!(year(1066).support_interval(3.0).unwrap().is_some());
    }

    #[test]
    fn relations_are_exhaustive_for_every_pair_of_definite_spans() {
        // With four known endpoints there is always at least one relation,
        // and for proper, distinct spans there is exactly one.
        for start in [0i64, 5, 10, 15] {
            for length in [1i64, 5, 20] {
                let a = FuzzyInstant::bounded(day(0), day(10)).unwrap();
                let b = FuzzyInstant::bounded(day(start), day(start + length)).unwrap();
                let relations = a.relations(b).unwrap();
                assert!(!relations.is_empty(), "no relation for {start}/{length}");
                assert_eq!(relations.len(), 1, "ambiguous for {start}/{length}");
            }
        }
    }

    #[test]
    fn every_allen_relation_is_reachable_from_some_pair_of_spans() {
        let reference = FuzzyInstant::bounded(day(10), day(20)).unwrap();
        let probes = [
            (0i64, 5i64, AllenRelation::Before),
            (0, 10, AllenRelation::Meets),
            (5, 15, AllenRelation::Overlaps),
            (10, 15, AllenRelation::Starts),
            (12, 18, AllenRelation::During),
            (15, 20, AllenRelation::Finishes),
            (10, 20, AllenRelation::Equals),
            (5, 20, AllenRelation::FinishedBy),
            (12, 30, AllenRelation::OverlappedBy),
            (20, 30, AllenRelation::MetBy),
            (25, 30, AllenRelation::After),
        ];
        for (low, high, expected) in probes {
            let probe = FuzzyInstant::bounded(day(low), day(high)).unwrap();
            let relations = probe.relations(reference).unwrap();
            assert!(
                relations.contains(expected),
                "{low}..{high} should be {expected}, got {relations}"
            );
        }
        // The two remaining relations are the converses of During and Starts.
        let container = FuzzyInstant::bounded(day(5), day(25)).unwrap();
        assert!(
            container
                .relations(reference)
                .unwrap()
                .contains(AllenRelation::Contains)
        );
        let started_by = FuzzyInstant::bounded(day(10), day(25)).unwrap();
        assert!(
            started_by
                .relations(reference)
                .unwrap()
                .contains(AllenRelation::StartedBy)
        );
    }

    #[test]
    fn display_summarises_each_variant() {
        #[cfg(feature = "alloc")]
        {
            use alloc::string::ToString as _;
            assert_eq!(FuzzyInstant::Unknown.to_string(), "unknown");
            assert_eq!(FuzzyInstant::exact(day(0)).to_string(), "0");
            assert!(FuzzyInstant::Before(day(1)).to_string().starts_with(".."));
            assert!(FuzzyInstant::After(day(1)).to_string().ends_with(".."));
        }
    }
}
