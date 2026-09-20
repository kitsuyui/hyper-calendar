//! The [`TimeZone`] trait and the three-way answer a local time deserves.
//!
//! # Why local times need three answers
//!
//! A zone maps an instant to an offset. Going the other way — from a local
//! wall-clock reading to an instant — is not a function. When clocks go back,
//! the hour before the change repeats and a local time names *two* instants.
//! When they go forward, an hour is skipped and a local time names *none*.
//! Libraries that return a single value have to pick one silently, and the
//! choice surfaces later as a meeting scheduled an hour out or an alarm that
//! never fires. [`LocalResolution`] makes the caller look at the answer, and
//! [`Disambiguation`] makes the choice explicit and named when they would
//! rather not.

use hc_calendar::fixed::RD_OF_UNIX_EPOCH;
use hc_calendar::{CivilDateTime, CivilTime, Rd};
use hc_core::{ATTOS_PER_SEC, Duration, UnixTime};

use crate::error::{TzError, TzResult};
use crate::offset::UtcOffset;

/// A rule for turning instants into local time and back.
///
/// The trait is object-safe, so a program can hold `&dyn TimeZone` values that
/// mix fixed offsets, POSIX rules and TZif data without caring which is which.
///
/// Every method works on POSIX time: days are 86 400 seconds long and leap
/// seconds do not exist. Zone rules are published in civil time and civil time
/// is what POSIX time counts, so this is the right timeline for the question.
/// Callers who need the physical elapsed time between two instants convert to
/// TAI with [`hc_core::unix`] afterwards.
pub trait TimeZone {
    /// The zone's name — an IANA identifier such as `Asia/Tokyo` where one
    /// exists, otherwise whatever the caller supplied.
    fn name(&self) -> &str;

    /// The offset in force at an instant.
    fn offset_at(&self, utc: UnixTime) -> UtcOffset;

    /// The abbreviation in force at an instant, such as `JST` or `-03`.
    ///
    /// `None` means the zone does not publish one. Abbreviations are not
    /// unique across the world (`CST` is used by three different zones) and
    /// are not stable over time, so they are for display only.
    fn abbreviation_at(&self, utc: UnixTime) -> Option<&str>;

    /// Whether daylight saving time is in force at an instant.
    ///
    /// This is the zone's own flag, not a comparison of offsets: Ireland
    /// records winter as its saving period, and some zones have carried a
    /// permanent "DST" offset for decades.
    fn is_dst_at(&self, utc: UnixTime) -> bool;

    /// Map a local wall-clock reading back to the instant or instants it names.
    fn resolve_local(&self, local: CivilDateTime) -> LocalResolution;

    /// The local civil date-time at an instant.
    ///
    /// # Errors
    ///
    /// Returns [`TzError::Overflow`] when the local day leaves the range of
    /// [`Rd`].
    fn local_at(&self, utc: UnixTime) -> TzResult<CivilDateTime> {
        local_from_unix(utc, self.offset_at(utc))
    }

    /// Map a local wall-clock reading to a single instant under a stated
    /// policy.
    ///
    /// # Errors
    ///
    /// Returns [`TzError::AmbiguousLocalTime`] or
    /// [`TzError::NonexistentLocalTime`] when the policy is
    /// [`Disambiguation::Reject`] and the local time is not unique.
    fn unix_at(&self, local: CivilDateTime, policy: Disambiguation) -> TzResult<UnixTime> {
        self.resolve_local(local).resolve(policy)
    }
}

impl<T: TimeZone + ?Sized> TimeZone for &T {
    fn name(&self) -> &str {
        (**self).name()
    }

    fn offset_at(&self, utc: UnixTime) -> UtcOffset {
        (**self).offset_at(utc)
    }

    fn abbreviation_at(&self, utc: UnixTime) -> Option<&str> {
        (**self).abbreviation_at(utc)
    }

    fn is_dst_at(&self, utc: UnixTime) -> bool {
        (**self).is_dst_at(utc)
    }

    fn resolve_local(&self, local: CivilDateTime) -> LocalResolution {
        (**self).resolve_local(local)
    }
}

/// What a local wall-clock reading turned out to mean.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalResolution {
    /// The local time names exactly one instant.
    Unambiguous(UnixTime),
    /// The local time names two instants: the clock was put back and the hour
    /// was lived through twice.
    Ambiguous {
        /// The first occurrence, before the clocks changed.
        earlier: UnixTime,
        /// The second occurrence, after the clocks changed.
        later: UnixTime,
        /// The offset in force during the first occurrence — the larger of
        /// the two, since the clock moved back.
        earlier_offset: UtcOffset,
        /// The offset in force during the second occurrence.
        later_offset: UtcOffset,
    },
    /// The local time names no instant: the clock was put forward across it.
    Nonexistent {
        /// The first local time that does not exist — the reading the clock
        /// jumped away from, inclusive.
        gap_start: CivilDateTime,
        /// The first local time after the gap that does exist — the reading
        /// the clock jumped to, exclusive as a gap bound.
        gap_end: CivilDateTime,
        /// The instant at which the clocks jumped.
        transition: UnixTime,
        /// The offset in force before the jump.
        offset_before: UtcOffset,
        /// The offset in force after it.
        offset_after: UtcOffset,
        /// What the requested reading would mean under `offset_after`: an
        /// instant strictly before the gap opened.
        before_gap: UnixTime,
        /// What it would mean under `offset_before`: an instant at or after
        /// the gap closed. `after_gap - before_gap` is the length of the gap.
        after_gap: UnixTime,
    },
}

impl LocalResolution {
    /// Whether the local time names exactly one instant.
    #[must_use]
    pub const fn is_unambiguous(&self) -> bool {
        matches!(self, Self::Unambiguous(_))
    }

    /// Whether the local time names two instants.
    #[must_use]
    pub const fn is_ambiguous(&self) -> bool {
        matches!(self, Self::Ambiguous { .. })
    }

    /// Whether the local time names none.
    #[must_use]
    pub const fn is_nonexistent(&self) -> bool {
        matches!(self, Self::Nonexistent { .. })
    }

    /// The earliest instant the reading could mean.
    #[must_use]
    pub const fn earliest(&self) -> UnixTime {
        match self {
            Self::Unambiguous(instant) => *instant,
            Self::Ambiguous { earlier, .. } => *earlier,
            Self::Nonexistent { before_gap, .. } => *before_gap,
        }
    }

    /// The latest instant the reading could mean.
    #[must_use]
    pub const fn latest(&self) -> UnixTime {
        match self {
            Self::Unambiguous(instant) => *instant,
            Self::Ambiguous { later, .. } => *later,
            Self::Nonexistent { after_gap, .. } => *after_gap,
        }
    }

    /// Reduce the resolution to one instant under a stated policy.
    ///
    /// # Errors
    ///
    /// Returns [`TzError::AmbiguousLocalTime`] or
    /// [`TzError::NonexistentLocalTime`] under [`Disambiguation::Reject`].
    pub const fn resolve(self, policy: Disambiguation) -> TzResult<UnixTime> {
        match self {
            Self::Unambiguous(instant) => Ok(instant),
            Self::Ambiguous { earlier, later, .. } => match policy {
                // The repeated reading does exist, so the "compatible" policy
                // has nothing to push forward and takes the first occurrence.
                Disambiguation::Earliest | Disambiguation::PushForward => Ok(earlier),
                Disambiguation::Latest => Ok(later),
                Disambiguation::Reject => Err(TzError::AmbiguousLocalTime),
            },
            Self::Nonexistent {
                before_gap,
                after_gap,
                ..
            } => match policy {
                Disambiguation::Earliest => Ok(before_gap),
                // Shifting the reading forward by the length of the gap and
                // then applying the new offset gives exactly `after_gap`, so
                // the two policies agree here; they differ on repeated times.
                Disambiguation::Latest | Disambiguation::PushForward => Ok(after_gap),
                Disambiguation::Reject => Err(TzError::NonexistentLocalTime),
            },
        }
    }
}

/// How to reduce an awkward local time to a single instant.
///
/// There is deliberately no `Default`. Which policy is right depends on what
/// the local time meant — a recurring alarm, a past log entry, a booking — and
/// a default would be a silent choice of the kind this crate exists to avoid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Disambiguation {
    /// Take the earliest instant the reading could name.
    Earliest,
    /// Take the latest instant the reading could name.
    Latest,
    /// Refuse to choose, and report the ambiguity as an error.
    Reject,
    /// The `compatible` policy of ECMAScript Temporal and the default
    /// resolver of `java.time`: a repeated reading takes its first
    /// occurrence, and a skipped reading is pushed forward by the length of
    /// the gap. This is what most calendar applications do to a recurring
    /// 02:30 appointment on the morning the clocks go forward.
    PushForward,
}

/// The local civil date-time an instant shows under a fixed offset.
///
/// # Errors
///
/// Returns [`TzError::Overflow`] when the local day leaves the range of
/// [`Rd`], which needs an instant some 292 billion years from now.
pub fn local_from_unix(utc: UnixTime, offset: UtcOffset) -> TzResult<CivilDateTime> {
    let total = i128::from(utc.seconds())
        + i128::from(offset.seconds())
        + i128::from(RD_OF_UNIX_EPOCH) * 86_400;
    let day = i64::try_from(total.div_euclid(86_400)).map_err(|_| TzError::Overflow)?;
    let remainder = total.rem_euclid(86_400);
    let time = CivilTime::from_midnight_offset(Duration::from_attos(
        remainder * i128::from(ATTOS_PER_SEC) + i128::from(utc.subsec_attos()),
    ))?;
    Ok(CivilDateTime::new(Rd(day), time))
}

/// The instant a local civil date-time names under a fixed offset.
///
/// # Errors
///
/// Returns [`TzError::Overflow`] when the instant is outside the range of a
/// POSIX timestamp.
pub fn unix_from_local(local: CivilDateTime, offset: UtcOffset) -> TzResult<UnixTime> {
    let seconds = nominal_seconds(local)
        - i128::from(RD_OF_UNIX_EPOCH) * 86_400
        - i128::from(offset.seconds());
    let seconds = i64::try_from(seconds).map_err(|_| TzError::Overflow)?;
    UnixTime::new(seconds, local.time.subsec_attos()).map_err(TzError::from)
}

/// Seconds from the Rata Die epoch to a local reading, every day counted as
/// 86 400 seconds.
fn nominal_seconds(local: CivilDateTime) -> i128 {
    i128::from(local.day.get()) * 86_400 + local.time.since_midnight().whole_seconds()
}

/// Like [`unix_from_local`], but saturating instead of failing.
///
/// [`TimeZone::resolve_local`] returns a resolution rather than a `Result`,
/// because "which instants does this reading name" is a question about the
/// zone, not about arithmetic. Readings far enough outside the POSIX range for
/// this to matter are billions of years away.
pub(crate) fn unix_from_local_saturating(local: CivilDateTime, offset: UtcOffset) -> UnixTime {
    let seconds = nominal_seconds(local)
        - i128::from(RD_OF_UNIX_EPOCH) * 86_400
        - i128::from(offset.seconds());
    let clamped = seconds.clamp(i128::from(i64::MIN), i128::from(i64::MAX)) as i64;
    UnixTime::new(clamped, local.time.subsec_attos()).unwrap_or(UnixTime::from_seconds(clamped))
}

/// How far either side of a reading to look for the offsets that could apply.
///
/// Two days covers any offset (at most 26 hours) plus a day's slack. The
/// probing below assumes a zone's offset holds steady for at least a day
/// around any transition, which is true of every zone in the IANA database:
/// the shortest-lived offset on record, Lord Howe Island's 1981 experiment
/// aside, is measured in months.
const PROBE_RADIUS: i64 = 2 * 86_400;

/// Step used when scanning for the exact instant of a transition.
const SCAN_STEP: i64 = 3_600;

/// Work out what a local reading means, given only a zone's instant-to-offset
/// function.
///
/// The method is the one that does not need a transition list, so it serves
/// fixed offsets, POSIX rules and TZif data alike: collect the offsets the
/// zone uses around the reading, and keep the candidate instants `local -
/// offset` for which the zone really does use `offset`. None surviving means
/// the reading was skipped; two surviving means it was repeated.
pub fn resolve_local_by_probing<F>(local: CivilDateTime, offset_at: F) -> LocalResolution
where
    F: Fn(UnixTime) -> UtcOffset,
{
    let nominal = unix_from_local_saturating(local, UtcOffset::UTC).seconds();

    let mut offsets = [UtcOffset::UTC; 5];
    let mut offset_count = 0usize;
    for step in -2i64..=2 {
        let probe = nominal.saturating_add(step.saturating_mul(86_400));
        let offset = offset_at(UnixTime::from_seconds(probe));
        if !offsets[..offset_count].contains(&offset) {
            offsets[offset_count] = offset;
            offset_count += 1;
        }
    }

    let mut candidates = [(UnixTime::EPOCH, UtcOffset::UTC); 5];
    let mut candidate_count = 0usize;
    for &offset in &offsets[..offset_count] {
        let candidate = unix_from_local_saturating(local, offset);
        if offset_at(candidate) == offset {
            candidates[candidate_count] = (candidate, offset);
            candidate_count += 1;
        }
    }

    match candidate_count {
        1 => LocalResolution::Unambiguous(candidates[0].0),
        0 => nonexistent(local, nominal, &offset_at),
        _ => {
            let mut earliest = candidates[0];
            let mut latest = candidates[0];
            for &candidate in &candidates[1..candidate_count] {
                if candidate.0 < earliest.0 {
                    earliest = candidate;
                }
                if candidate.0 > latest.0 {
                    latest = candidate;
                }
            }
            LocalResolution::Ambiguous {
                earlier: earliest.0,
                later: latest.0,
                earlier_offset: earliest.1,
                later_offset: latest.1,
            }
        }
    }
}

/// Describe the gap a skipped reading fell into.
fn nonexistent<F>(local: CivilDateTime, nominal: i64, offset_at: &F) -> LocalResolution
where
    F: Fn(UnixTime) -> UtcOffset,
{
    let Some(transition) = nearest_transition(nominal, offset_at) else {
        // Unreachable for a zone whose `offset_at` is consistent: a reading
        // with no valid interpretation requires a change of offset nearby.
        let offset = offset_at(UnixTime::from_seconds(nominal));
        return LocalResolution::Unambiguous(unix_from_local_saturating(local, offset));
    };
    let offset_before = offset_at(UnixTime::from_seconds(transition - 1));
    let offset_after = offset_at(UnixTime::from_seconds(transition));
    let instant = UnixTime::from_seconds(transition);
    LocalResolution::Nonexistent {
        gap_start: local_from_unix(instant, offset_before).unwrap_or(local),
        gap_end: local_from_unix(instant, offset_after).unwrap_or(local),
        transition: instant,
        offset_before,
        offset_after,
        before_gap: unix_from_local_saturating(local, offset_after),
        after_gap: unix_from_local_saturating(local, offset_before),
    }
}

/// The instant nearest `around` at which the offset changes, searched within
/// [`PROBE_RADIUS`].
fn nearest_transition<F>(around: i64, offset_at: &F) -> Option<i64>
where
    F: Fn(UnixTime) -> UtcOffset,
{
    let start = around.saturating_sub(PROBE_RADIUS);
    let end = around.saturating_add(PROBE_RADIUS);
    let mut best: Option<i64> = None;
    let mut low = start;
    let mut low_offset = offset_at(UnixTime::from_seconds(low));
    while low < end {
        let high = (low + SCAN_STEP).min(end);
        let high_offset = offset_at(UnixTime::from_seconds(high));
        if high_offset != low_offset {
            let transition = first_instant_with_new_offset(low, high, low_offset, offset_at);
            if best.is_none_or(|current| (transition - around).abs() < (current - around).abs()) {
                best = Some(transition);
            }
        }
        low = high;
        low_offset = high_offset;
    }
    best
}

/// Binary-search `(low, high]` for the first second whose offset differs from
/// `low_offset`.
fn first_instant_with_new_offset<F>(
    mut low: i64,
    mut high: i64,
    low_offset: UtcOffset,
    offset_at: &F,
) -> i64
where
    F: Fn(UnixTime) -> UtcOffset,
{
    while high - low > 1 {
        let middle = low + (high - low) / 2;
        if offset_at(UnixTime::from_seconds(middle)) == low_offset {
            low = middle;
        } else {
            high = middle;
        }
    }
    high
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gregorian::rd_from_ymd;

    fn civil(year: i64, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> CivilDateTime {
        CivilDateTime::new(
            Rd(rd_from_ymd(year, month, day)),
            CivilTime::hms(hour, minute, second).unwrap(),
        )
    }

    /// A stand-in zone: `-05:00` until the transition instant, `-04:00` after.
    fn spring_forward(utc: UnixTime) -> UtcOffset {
        let offset = if utc.seconds() >= 1_710_054_000 {
            -4 * 3_600
        } else {
            -5 * 3_600
        };
        UtcOffset::from_seconds(offset).unwrap()
    }

    /// The mirror image: `-04:00` until the transition, `-05:00` after.
    fn fall_back(utc: UnixTime) -> UtcOffset {
        let offset = if utc.seconds() >= 1_730_613_600 {
            -5 * 3_600
        } else {
            -4 * 3_600
        };
        UtcOffset::from_seconds(offset).unwrap()
    }

    #[test]
    fn instants_and_local_readings_round_trip_under_a_fixed_offset() {
        let offset = UtcOffset::from_hms(5, 45, 0).unwrap();
        for seconds in (0..40_000_000).step_by(9_973) {
            let instant = UnixTime::from_seconds(seconds);
            let local = local_from_unix(instant, offset).unwrap();
            assert_eq!(
                unix_from_local(local, offset).unwrap(),
                instant,
                "{seconds}"
            );
        }
    }

    #[test]
    fn a_constant_offset_is_never_ambiguous() {
        let offset = UtcOffset::from_hms(9, 0, 0).unwrap();
        for day in 719_000..719_500 {
            let local = CivilDateTime::new(Rd(day), CivilTime::hms(2, 30, 0).unwrap());
            let resolution = resolve_local_by_probing(local, |_| offset);
            assert!(resolution.is_unambiguous(), "{day}");
            assert_eq!(
                resolution.earliest(),
                unix_from_local(local, offset).unwrap()
            );
        }
    }

    #[test]
    fn the_repeated_hour_resolves_to_two_instants() {
        let local = civil(2024, 11, 3, 1, 30, 0);
        let resolution = resolve_local_by_probing(local, fall_back);
        match resolution {
            LocalResolution::Ambiguous {
                earlier,
                later,
                earlier_offset,
                later_offset,
            } => {
                assert_eq!(earlier.seconds(), 1_730_611_800);
                assert_eq!(later.seconds(), 1_730_615_400);
                assert_eq!(later.seconds() - earlier.seconds(), 3_600);
                assert_eq!(earlier_offset.seconds(), -4 * 3_600);
                assert_eq!(later_offset.seconds(), -5 * 3_600);
            }
            other => panic!("expected an ambiguous reading, got {other:?}"),
        }
    }

    #[test]
    fn the_skipped_hour_resolves_to_no_instant_and_names_the_gap() {
        let local = civil(2024, 3, 10, 2, 30, 0);
        let resolution = resolve_local_by_probing(local, spring_forward);
        match resolution {
            LocalResolution::Nonexistent {
                gap_start,
                gap_end,
                transition,
                offset_before,
                offset_after,
                before_gap,
                after_gap,
            } => {
                assert_eq!(gap_start, civil(2024, 3, 10, 2, 0, 0));
                assert_eq!(gap_end, civil(2024, 3, 10, 3, 0, 0));
                assert_eq!(transition.seconds(), 1_710_054_000);
                assert_eq!(offset_before.seconds(), -5 * 3_600);
                assert_eq!(offset_after.seconds(), -4 * 3_600);
                assert_eq!(before_gap.seconds(), 1_710_052_200);
                assert_eq!(after_gap.seconds(), 1_710_055_800);
                assert_eq!(after_gap.seconds() - before_gap.seconds(), 3_600);
            }
            other => panic!("expected a skipped reading, got {other:?}"),
        }
    }

    #[test]
    fn disambiguation_policies_pick_the_documented_instant() {
        let repeated = resolve_local_by_probing(civil(2024, 11, 3, 1, 30, 0), fall_back);
        assert_eq!(
            repeated
                .resolve(Disambiguation::Earliest)
                .unwrap()
                .seconds(),
            1_730_611_800
        );
        assert_eq!(
            repeated.resolve(Disambiguation::Latest).unwrap().seconds(),
            1_730_615_400
        );
        assert_eq!(
            repeated
                .resolve(Disambiguation::PushForward)
                .unwrap()
                .seconds(),
            1_730_611_800
        );
        assert_eq!(
            repeated.resolve(Disambiguation::Reject),
            Err(TzError::AmbiguousLocalTime)
        );

        let skipped = resolve_local_by_probing(civil(2024, 3, 10, 2, 30, 0), spring_forward);
        assert_eq!(
            skipped.resolve(Disambiguation::Earliest).unwrap().seconds(),
            1_710_052_200
        );
        assert_eq!(
            skipped.resolve(Disambiguation::Latest).unwrap().seconds(),
            1_710_055_800
        );
        assert_eq!(
            skipped
                .resolve(Disambiguation::PushForward)
                .unwrap()
                .seconds(),
            1_710_055_800
        );
        assert_eq!(
            skipped.resolve(Disambiguation::Reject),
            Err(TzError::NonexistentLocalTime)
        );
    }

    #[test]
    fn pushing_a_skipped_reading_forward_lands_on_the_same_wall_clock_offset() {
        // Pushing 02:30 forward by the gap gives 03:30, which exists; the
        // instant it names must be the one `PushForward` reported.
        let skipped = resolve_local_by_probing(civil(2024, 3, 10, 2, 30, 0), spring_forward);
        let pushed = resolve_local_by_probing(civil(2024, 3, 10, 3, 30, 0), spring_forward);
        assert_eq!(
            skipped.resolve(Disambiguation::PushForward).unwrap(),
            pushed.resolve(Disambiguation::PushForward).unwrap()
        );
    }

    #[test]
    fn readings_just_outside_the_gap_are_unambiguous() {
        for (hour, minute) in [(1u8, 59u8), (3, 0), (3, 1)] {
            let local = civil(2024, 3, 10, hour, minute, 0);
            assert!(
                resolve_local_by_probing(local, spring_forward).is_unambiguous(),
                "{hour}:{minute}"
            );
        }
        for (hour, minute) in [(0u8, 59u8), (2, 0), (2, 30)] {
            let local = civil(2024, 11, 3, hour, minute, 0);
            assert!(
                resolve_local_by_probing(local, fall_back).is_unambiguous(),
                "{hour}:{minute}"
            );
        }
    }

    #[test]
    fn every_instant_of_a_transition_week_survives_the_round_trip() {
        // Local readings taken from real instants must always resolve back to
        // an interval containing the instant they came from.
        for seconds in (1_709_800_000..1_710_400_000).step_by(60) {
            let instant = UnixTime::from_seconds(seconds);
            let offset = spring_forward(instant);
            let local = local_from_unix(instant, offset).unwrap();
            let resolution = resolve_local_by_probing(local, spring_forward);
            assert!(
                resolution.earliest() <= instant && instant <= resolution.latest(),
                "{seconds}"
            );
        }
    }

    #[test]
    fn a_reference_to_a_zone_is_itself_a_zone() {
        // Both forms matter: the trait object proves object safety, and the
        // blanket impl lets `&zone` be passed where a zone is wanted.
        fn summarise<Z: TimeZone>(zone: Z, instant: UnixTime) -> (i32, bool, bool) {
            (
                zone.offset_at(instant).seconds(),
                zone.is_dst_at(instant),
                zone.resolve_local(zone.local_at(instant).unwrap_or_default())
                    .is_unambiguous(),
            )
        }

        use crate::fixed::Utc;
        let utc = Utc;
        let instant = UnixTime::from_seconds(1_704_067_200);
        let borrowed: &dyn TimeZone = &utc;
        assert_eq!(borrowed.name(), "UTC");
        assert_eq!(borrowed.abbreviation_at(instant), Some("UTC"));
        assert_eq!(borrowed.offset_at(instant), UtcOffset::UTC);
        let by_reference: &Utc = &utc;
        assert_eq!(summarise(by_reference, instant), (0, false, true));
        assert_eq!(TimeZone::name(&&utc), "UTC");
    }

    #[test]
    fn overflowing_conversions_are_reported_rather_than_wrapped() {
        let far_future = CivilDateTime::midnight(Rd(i64::MAX));
        assert_eq!(
            unix_from_local(far_future, UtcOffset::UTC),
            Err(TzError::Overflow)
        );
        // The saturating path used by `resolve_local` must still answer.
        let resolution = resolve_local_by_probing(far_future, |_| UtcOffset::UTC);
        assert!(resolution.is_unambiguous());
    }
}
