//! Choosing which unit to say a span in, and how to round into it.
//!
//! # Why this is a separate module
//!
//! "400 days ago" is arithmetically perfect and conversationally useless;
//! "just over a year ago" is what a person says. Getting from one to the
//! other takes two decisions that libraries routinely bake in as one
//! unchangeable opinion:
//!
//! * **Which unit.** Ninety minutes is `90 min`, `1.5 h` or `2 h`; four
//!   hundred days is `400 d`, `13 mo` or `1 y`. The choice is a threshold
//!   table, and a chat timestamp, a build log and a geological caption want
//!   different tables.
//! * **Which direction to round.** A cache that expires in 90 seconds has
//!   *2 minutes* left if you round up and *1 minute* if you round down, and
//!   only one of those is safe to show.
//!
//! Both are parameters here: [`Thresholds`] and [`RoundingPolicy`]. The
//! defaults are stated, not hidden, and a caller who disagrees passes their
//! own table rather than forking the crate.
//!
//! # Rounding of negative spans
//!
//! [`RoundingPolicy::Floor`] and [`RoundingPolicy::Truncate`] differ only
//! for spans in the past, and they differ every time: floor goes toward
//! minus infinity, so −1.5 days becomes −2, while truncate goes toward
//! zero, so it becomes −1. A humaniser usually wants truncate — "1 day ago"
//! rather than "2 days ago" for something 36 hours old — and a deadline
//! usually wants floor.

use hc_core::Duration;
use hc_core::math::{abs, ceil, floor, round, trunc};

use crate::error::{HumanizeError, HumanizeResult};
use crate::unit::{TimeUnit, UnitAmount};

/// How a fractional count is turned into a whole one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RoundingPolicy {
    /// Toward plus infinity: 1.2 → 2, −1.2 → −1.
    Ceil,
    /// Toward minus infinity: 1.8 → 1, −1.2 → −2.
    Floor,
    /// To the nearest, halves away from zero: 1.5 → 2, −1.5 → −2.
    #[default]
    Nearest,
    /// Toward zero: 1.8 → 1, −1.8 → −1.
    Truncate,
    /// To the nearest half, halves away from zero: 1.5 stays 1.5.
    ///
    /// This is the policy that turns ninety minutes into *an hour and a
    /// half* rather than *2 hours*.
    NearestHalf,
}

impl RoundingPolicy {
    /// Apply the policy to a signed count of units.
    ///
    /// # Errors
    ///
    /// Returns [`HumanizeError::Overflow`] if the rounded count is not
    /// finite or does not fit an `i64`.
    pub fn apply(self, count: f64, unit: TimeUnit) -> HumanizeResult<UnitAmount> {
        if self == Self::NearestHalf {
            let halves = to_i64(round(count * 2.0))?;
            return Ok(if halves.rem_euclid(2) == 0 {
                UnitAmount::whole(halves / 2, unit)
            } else {
                // Integer division truncates toward zero, which is what a
                // signed half wants: -3 halves is -1 whole plus a half, and
                // the sign already lives on the whole part.
                UnitAmount::half_past(halves / 2, unit)
            });
        }
        let rounded = match self {
            Self::Ceil => ceil(count),
            Self::Floor => floor(count),
            Self::Nearest => round(count),
            Self::Truncate | Self::NearestHalf => trunc(count),
        };
        Ok(UnitAmount::whole(to_i64(rounded)?, unit))
    }
}

/// Convert a rounded `f64` to `i64`, refusing anything unrepresentable.
fn to_i64(value: f64) -> HumanizeResult<i64> {
    // 2^63 exactly; an f64 at or above it cannot be an i64.
    const LIMIT: f64 = 9_223_372_036_854_775_808.0;
    if !(value.is_finite() && abs(value) < LIMIT) {
        return Err(HumanizeError::Overflow);
    }
    Ok(value as i64)
}

/// One row of a threshold table: *stay in this unit while the count is below
/// this many of it*.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Threshold {
    /// The unit this row is about.
    pub unit: TimeUnit,
    /// The exclusive upper bound, counted in that unit.
    ///
    /// `i64::MAX` means "and everything above", which the last row of a
    /// table needs.
    pub limit: i64,
}

impl Threshold {
    /// One row.
    #[must_use]
    pub const fn new(unit: TimeUnit, limit: i64) -> Self {
        Self { unit, limit }
    }
}

/// An ordered table of thresholds, shortest unit first.
///
/// The table is data the caller supplies, not a constant this crate hides.
/// [`Thresholds::DEFAULT`] is the conversational table — the one that says
/// *a minute ago* for 50 seconds — and the others are alternatives, not
/// refinements.
#[derive(Debug, Clone, Copy)]
pub struct Thresholds {
    rows: &'static [Threshold],
}

/// The conversational table.
///
/// The bounds are the ones a chat client or a feed wants: 45 seconds is
/// already "a minute", 22 hours is already "a day", and quarters are left
/// out because almost nobody says "two quarters ago" outside a financial
/// report.
static DEFAULT_ROWS: &[Threshold] = &[
    Threshold::new(TimeUnit::Second, 45),
    Threshold::new(TimeUnit::Minute, 45),
    Threshold::new(TimeUnit::Hour, 22),
    Threshold::new(TimeUnit::Day, 6),
    Threshold::new(TimeUnit::Week, 4),
    Threshold::new(TimeUnit::Month, 11),
    Threshold::new(TimeUnit::Year, i64::MAX),
];

/// The table that never rounds a unit up early: a span moves to the next
/// unit only once a whole one of it fits.
static EXACT_ROWS: &[Threshold] = &[
    Threshold::new(TimeUnit::Second, 60),
    Threshold::new(TimeUnit::Minute, 60),
    Threshold::new(TimeUnit::Hour, 24),
    Threshold::new(TimeUnit::Day, 7),
    Threshold::new(TimeUnit::Week, 4),
    Threshold::new(TimeUnit::Month, 12),
    Threshold::new(TimeUnit::Year, i64::MAX),
];

/// The conversational table with quarters restored, for reporting.
static QUARTER_ROWS: &[Threshold] = &[
    Threshold::new(TimeUnit::Second, 45),
    Threshold::new(TimeUnit::Minute, 45),
    Threshold::new(TimeUnit::Hour, 22),
    Threshold::new(TimeUnit::Day, 6),
    Threshold::new(TimeUnit::Week, 4),
    Threshold::new(TimeUnit::Month, 3),
    Threshold::new(TimeUnit::Quarter, 4),
    Threshold::new(TimeUnit::Year, i64::MAX),
];

impl Thresholds {
    /// The conversational table.
    pub const DEFAULT: Self = Self { rows: DEFAULT_ROWS };

    /// The table that promotes a unit only when a whole one fits.
    pub const EXACT: Self = Self { rows: EXACT_ROWS };

    /// The conversational table with quarters.
    pub const WITH_QUARTERS: Self = Self { rows: QUARTER_ROWS };

    /// A table of the caller's own.
    ///
    /// Rows are read in order and the last one answers for everything left,
    /// whatever its limit says, so a table is never empty-handed.
    #[must_use]
    pub const fn new(rows: &'static [Threshold]) -> Self {
        Self { rows }
    }

    /// The rows, in order.
    #[must_use]
    pub const fn rows(&self) -> &'static [Threshold] {
        self.rows
    }

    /// The unit a span of this magnitude belongs in.
    ///
    /// The sign is ignored: *3 days ago* and *in 3 days* pick the same unit.
    #[must_use]
    pub fn unit_for(&self, span: Duration) -> TimeUnit {
        let magnitude = abs(span.as_secs_f64());
        let mut last = TimeUnit::Year;
        for row in self.rows {
            last = row.unit;
            let bound = if row.limit == i64::MAX {
                f64::INFINITY
            } else {
                row.limit as f64 * row.unit.mean_seconds() as f64
            };
            if magnitude < bound {
                return row.unit;
            }
        }
        last
    }
}

impl Default for Thresholds {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// The span as a signed fractional count of one unit.
///
/// Mean unit lengths, so this is the place a month stops being a calendar
/// month; see [`crate::unit`].
#[must_use]
pub fn count_of(span: Duration, unit: TimeUnit) -> f64 {
    span.as_secs_f64() / unit.mean_seconds() as f64
}

/// The part of the span left over after the whole units, as a fraction of
/// one unit, always in `0.0..1.0`.
///
/// Sign is dropped first, so a span in the past and the matching span in the
/// future have the same remainder.
/// [`crate::approximate`](fn@crate::approximate) keys its hedges on this.
#[must_use]
pub fn fraction_of(span: Duration, unit: TimeUnit) -> f64 {
    let magnitude = abs(count_of(span, unit));
    magnitude - floor(magnitude)
}

/// Express a span in one named unit under a rounding policy.
///
/// # Errors
///
/// Returns [`HumanizeError::Overflow`] if the count does not fit an `i64`.
pub fn count_in(
    span: Duration,
    unit: TimeUnit,
    policy: RoundingPolicy,
) -> HumanizeResult<UnitAmount> {
    policy.apply(count_of(span, unit), unit)
}

/// Choose a unit for a span and express it in that unit.
///
/// This is the whole module in one call: pick the unit with the threshold
/// table, then round into it with the policy.
///
/// # Promotion implies at least one
///
/// A table that sends fifty seconds to minutes is asserting that a minute is
/// the right unit to say it in. Truncating to zero would throw that
/// assertion away and produce *in 0 minutes*, which is both wrong and worse
/// than the unrounded answer, so a span promoted past the table's shortest
/// unit never comes back as zero. A span that stays in the shortest unit is
/// left alone: *0 seconds* is a real answer.
///
/// # Errors
///
/// Returns [`HumanizeError::Overflow`] if the count does not fit an `i64`.
pub fn choose(
    span: Duration,
    thresholds: &Thresholds,
    policy: RoundingPolicy,
) -> HumanizeResult<UnitAmount> {
    let unit = thresholds.unit_for(span);
    let amount = count_in(span, unit, policy)?;
    let promoted = thresholds
        .rows()
        .first()
        .is_some_and(|shortest| shortest.unit != unit);
    if promoted && amount.count() == 0 && !amount.has_half() && !span.is_zero() {
        let sign = if span.is_negative() { -1 } else { 1 };
        return Ok(UnitAmount::whole(sign, unit));
    }
    Ok(amount)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINUTE: i128 = 60;
    const HOUR: i128 = 3_600;
    const DAY: i128 = 86_400;

    #[test]
    fn ninety_minutes_is_two_hours_or_an_hour_and_a_half() {
        let ninety = Duration::from_secs(90 * MINUTE);
        let rounded = choose(ninety, &Thresholds::DEFAULT, RoundingPolicy::Nearest).expect("fits");
        assert_eq!(rounded, UnitAmount::whole(2, TimeUnit::Hour));
        let halved =
            choose(ninety, &Thresholds::DEFAULT, RoundingPolicy::NearestHalf).expect("fits");
        assert_eq!(halved, UnitAmount::half_past(1, TimeUnit::Hour));
        assert!((halved.as_f64() - 1.5).abs() < 1e-12);
    }

    #[test]
    fn four_hundred_days_is_a_year_and_not_four_hundred_days() {
        let span = Duration::from_days(400);
        assert_eq!(Thresholds::DEFAULT.unit_for(span), TimeUnit::Year);
        assert_eq!(
            choose(span, &Thresholds::DEFAULT, RoundingPolicy::Truncate).expect("fits"),
            UnitAmount::whole(1, TimeUnit::Year)
        );
    }

    #[test]
    fn the_four_policies_disagree_only_where_it_matters() {
        // Thirty-six hours is one and a half days, and the disagreement is
        // the whole reason the policy is a parameter.
        let span = Duration::from_secs(36 * HOUR);
        let count = |policy| count_in(span, TimeUnit::Day, policy).expect("fits").count();
        assert_eq!(count(RoundingPolicy::Ceil), 2);
        assert_eq!(count(RoundingPolicy::Floor), 1);
        assert_eq!(count(RoundingPolicy::Nearest), 2);
        assert_eq!(count(RoundingPolicy::Truncate), 1);
    }

    #[test]
    fn floor_and_truncate_differ_on_every_span_in_the_past() {
        let span = Duration::from_secs(-36 * HOUR);
        let count = |policy| count_in(span, TimeUnit::Day, policy).expect("fits").count();
        assert_eq!(count(RoundingPolicy::Floor), -2);
        assert_eq!(count(RoundingPolicy::Truncate), -1);
        assert_eq!(count(RoundingPolicy::Ceil), -1);
        assert_eq!(count(RoundingPolicy::Nearest), -2);
    }

    #[test]
    fn the_half_policy_keeps_the_sign_on_the_whole_part() {
        let span = Duration::from_secs(-90 * MINUTE);
        let amount = count_in(span, TimeUnit::Hour, RoundingPolicy::NearestHalf).expect("fits");
        assert_eq!(amount, UnitAmount::half_past(-1, TimeUnit::Hour));
        assert!(amount.is_negative());
    }

    #[test]
    fn the_half_policy_leaves_a_whole_count_whole() {
        let span = Duration::from_secs(2 * HOUR);
        let amount = count_in(span, TimeUnit::Hour, RoundingPolicy::NearestHalf).expect("fits");
        assert_eq!(amount, UnitAmount::whole(2, TimeUnit::Hour));
        assert!(!amount.has_half());
    }

    #[test]
    fn a_promoted_span_never_rounds_away_to_zero() {
        // Fifty seconds is past the 45-second threshold, so the table has
        // already said "minutes"; truncating to zero would unsay it.
        let span = Duration::from_secs(50);
        assert_eq!(Thresholds::DEFAULT.unit_for(span), TimeUnit::Minute);
        assert_eq!(
            choose(span, &Thresholds::DEFAULT, RoundingPolicy::Truncate).expect("fits"),
            UnitAmount::whole(1, TimeUnit::Minute)
        );
        assert_eq!(
            choose(
                Duration::from_secs(-50),
                &Thresholds::DEFAULT,
                RoundingPolicy::Truncate
            )
            .expect("fits"),
            UnitAmount::whole(-1, TimeUnit::Minute)
        );
    }

    #[test]
    fn a_span_that_stays_in_the_shortest_unit_may_be_zero() {
        let span = Duration::from_millis(400);
        assert_eq!(Thresholds::DEFAULT.unit_for(span), TimeUnit::Second);
        assert_eq!(
            choose(span, &Thresholds::DEFAULT, RoundingPolicy::Truncate).expect("fits"),
            UnitAmount::whole(0, TimeUnit::Second)
        );
    }

    #[test]
    fn the_conversational_table_promotes_early_and_the_exact_one_does_not() {
        let fifty_seconds = Duration::from_secs(50);
        assert_eq!(
            Thresholds::DEFAULT.unit_for(fifty_seconds),
            TimeUnit::Minute
        );
        assert_eq!(Thresholds::EXACT.unit_for(fifty_seconds), TimeUnit::Second);
        let twenty_three_hours = Duration::from_secs(23 * HOUR);
        assert_eq!(
            Thresholds::DEFAULT.unit_for(twenty_three_hours),
            TimeUnit::Day
        );
        assert_eq!(
            Thresholds::EXACT.unit_for(twenty_three_hours),
            TimeUnit::Hour
        );
    }

    #[test]
    fn quarters_appear_only_in_the_table_that_asks_for_them() {
        let five_months = Duration::from_days(150);
        assert_eq!(Thresholds::DEFAULT.unit_for(five_months), TimeUnit::Month);
        assert_eq!(
            Thresholds::WITH_QUARTERS.unit_for(five_months),
            TimeUnit::Quarter
        );
    }

    #[test]
    fn a_caller_can_supply_a_table_of_their_own() {
        // A build log that never wants to hear about weeks or months.
        static ROWS: &[Threshold] = &[
            Threshold::new(TimeUnit::Second, 120),
            Threshold::new(TimeUnit::Minute, 120),
            Threshold::new(TimeUnit::Hour, i64::MAX),
        ];
        let table = Thresholds::new(ROWS);
        assert_eq!(table.rows().len(), 3);
        assert_eq!(table.unit_for(Duration::from_secs(90)), TimeUnit::Second);
        assert_eq!(
            table.unit_for(Duration::from_secs(90 * MINUTE)),
            TimeUnit::Minute
        );
        assert_eq!(table.unit_for(Duration::from_days(30)), TimeUnit::Hour);
    }

    #[test]
    fn the_sign_never_changes_the_unit() {
        for seconds in [50i128, 90 * MINUTE, 5 * DAY, 400 * DAY] {
            assert_eq!(
                Thresholds::DEFAULT.unit_for(Duration::from_secs(seconds)),
                Thresholds::DEFAULT.unit_for(Duration::from_secs(-seconds))
            );
        }
    }

    #[test]
    fn the_leftover_fraction_drops_the_sign() {
        let forward = fraction_of(Duration::from_secs(36 * HOUR), TimeUnit::Day);
        let back = fraction_of(Duration::from_secs(-36 * HOUR), TimeUnit::Day);
        assert!((forward - 0.5).abs() < 1e-12);
        assert!((back - 0.5).abs() < 1e-12);
        assert!((0.0..1.0).contains(&forward));
    }

    #[test]
    fn the_signed_count_keeps_its_sign() {
        assert!((count_of(Duration::from_secs(-36 * HOUR), TimeUnit::Day) + 1.5).abs() < 1e-12);
    }

    #[test]
    fn a_count_too_large_for_an_integer_is_reported_not_wrapped() {
        assert_eq!(
            RoundingPolicy::Nearest.apply(f64::INFINITY, TimeUnit::Day),
            Err(HumanizeError::Overflow)
        );
        assert_eq!(
            RoundingPolicy::Nearest.apply(f64::NAN, TimeUnit::Day),
            Err(HumanizeError::Overflow)
        );
        assert_eq!(
            RoundingPolicy::Nearest.apply(1e30, TimeUnit::Day),
            Err(HumanizeError::Overflow)
        );
    }

    #[test]
    fn the_last_row_of_a_table_answers_for_everything_left() {
        static ROWS: &[Threshold] = &[Threshold::new(TimeUnit::Second, 10)];
        let table = Thresholds::new(ROWS);
        assert_eq!(
            table.unit_for(Duration::from_days(10_000)),
            TimeUnit::Second
        );
    }
}
