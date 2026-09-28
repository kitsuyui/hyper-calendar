//! Root finding on angles that run round a circle, and on sign changes.
//!
//! Solar longitude and lunar phase are both monotonic-but-wrapping: they
//! climb from 0° to 360° and then start again. A plain sign-change bisection
//! cannot see that, so the predicate used here is the one from Reingold &
//! Dershowitz, *Calendrical Calculations*, 4th ed., §14.4: the angle has
//! reached its target when `(f(t) − target) mod 360 < 180`.
//!
//! These are public so that the calendars built on other models of the sky
//! — the Surya Siddhanta's, the pañcāṅga's sidereal Moon — search with the
//! same code rather than a copy of it. Each function states what it needs of
//! its function and its bracket, and what it answers when the bracket is
//! wrong; none of them panics.

use hc_calendar::fixed::Moment;

use hc_core::math::{modulo, normalize_degrees};

/// How close a bisection has to get before it stops, in days.
///
/// 10⁻⁷ days is 8.6 milliseconds, far below the accuracy of any series in
/// this crate. The point of going this fine is not precision but
/// repeatability: two callers asking the same question get bit-identical
/// answers.
pub const BISECTION_TOLERANCE_DAYS: f64 = 1e-7;

/// A hard cap on bisection steps, so that a caller who hands in a `NaN` or a
/// reversed interval gets a wrong answer rather than a hung thread.
const MAXIMUM_STEPS: u32 = 64;

/// The moment in `[low, high]` at which a circular angle `f` reaches
/// `target`, found by bisection on the wrap-aware predicate.
///
/// The caller is responsible for bracketing: the predicate must be false at
/// `low` and true at `high`. When it is not, the result is the endpoint the
/// search collapses onto, which is the behaviour a calendar wants — a
/// clamped answer rather than a panic.
pub fn invert_angular<F>(f: F, target: f64, low: Moment, high: Moment) -> Moment
where
    F: Fn(Moment) -> f64,
{
    let mut low = low.0;
    let mut high = high.0;
    let mut steps = 0;
    while high - low > BISECTION_TOLERANCE_DAYS && steps < MAXIMUM_STEPS {
        let middle = low + (high - low) / 2.0;
        if modulo(f(Moment(middle)) - target, 360.0) < 180.0 {
            high = middle;
        } else {
            low = middle;
        }
        steps += 1;
    }
    Moment(low + (high - low) / 2.0)
}

/// The moment in `[low, high]` at which `f` crosses zero from below,
/// found by ordinary bisection.
///
/// Returns `None` unless `f(low) <= 0 <= f(high)`, which is how the rise and
/// set code reports a polar day or a polar night rather than inventing one.
pub fn bisect_rising<F>(f: F, low: Moment, high: Moment) -> Option<Moment>
where
    F: Fn(Moment) -> f64,
{
    let mut low = low.0;
    let mut high = high.0;
    if !(f(Moment(low)) <= 0.0 && f(Moment(high)) >= 0.0) {
        return None;
    }
    let mut steps = 0;
    while high - low > BISECTION_TOLERANCE_DAYS && steps < MAXIMUM_STEPS {
        let middle = low + (high - low) / 2.0;
        if f(Moment(middle)) < 0.0 {
            low = middle;
        } else {
            high = middle;
        }
        steps += 1;
    }
    Some(Moment(low + (high - low) / 2.0))
}

/// The mirror image of [`bisect_rising`]: the moment at which `f` crosses
/// zero from above.
pub fn bisect_falling<F>(f: F, low: Moment, high: Moment) -> Option<Moment>
where
    F: Fn(Moment) -> f64,
{
    bisect_rising(|moment| -f(moment), low, high)
}

/// Halve `[low, high]` exactly `rounds` times towards the moment at which
/// `reached` turns true, and answer the upper end, the earliest moment
/// found at which it is true.
///
/// The caller brackets: `reached` false at `low` and true at `high`, and
/// true from its first true moment to `high`. A fixed number of rounds
/// rather than a tolerance is what a model stated to a fixed precision
/// wants, and it makes the answer the same bits on every platform; 40
/// rounds narrow a bracket of a day to 8 × 10⁻⁸ s. A bracket that breaks
/// the contract gets the end the halving collapses onto, not a panic.
pub fn bisect_until<F>(reached: F, low: Moment, high: Moment, rounds: u32) -> Moment
where
    F: Fn(Moment) -> bool,
{
    let mut low = low.0;
    let mut high = high.0;
    for _ in 0..rounds {
        let middle = (low + high) / 2.0;
        if reached(Moment(middle)) {
            high = middle;
        } else {
            low = middle;
        }
    }
    Moment(high)
}

/// The first moment at or after `from` at which a steadily growing angle,
/// in degrees, reaches `target`: stepping forward by `step` days until the
/// angle passes the target, then 40 rounds of [`bisect_until`].
///
/// "Steadily growing" is the contract: the angle never falls, and covers
/// less than half a revolution in `step` days, so that what is left to
/// travel, `(target − angle) mod 360`, falls with time and jumps by a
/// revolution at the crossing. The Moon's longitude, about 13° a day, and
/// its elongation from the Sun, 10° to 15°, meet it with a step of half a
/// day. An angle that does not keep it is searched until it next appears
/// to, which for one that never grows is for ever; the caller's angle is a
/// model of the sky, which always does.
pub fn next_angle_crossing<F>(angle: F, target: f64, from: Moment, step: f64) -> Moment
where
    F: Fn(Moment) -> f64,
{
    let to_go = |at: Moment| normalize_degrees(target - angle(at));
    let mut low = from;
    let mut left = to_go(low);
    let mut high = Moment(low.0 + step);
    let mut ahead = to_go(high);
    while ahead < left {
        low = high;
        left = ahead;
        high = Moment(high.0 + step);
        ahead = to_go(high);
    }
    // The crossing lies between `low` and `high`: before it less than a
    // step's travel is left, after it nearly a revolution.
    bisect_until(|at| to_go(at) >= 180.0, low, high, 40)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angular_inversion_finds_a_target_on_a_rising_ramp() {
        // A toy angle that gains one degree per day and wraps at 360.
        let angle = |moment: Moment| modulo(moment.0, 360.0);
        let found = invert_angular(angle, 100.0, Moment(0.0), Moment(200.0));
        assert!((found.0 - 100.0).abs() < 1e-6, "found {}", found.0);
    }

    #[test]
    fn angular_inversion_crosses_the_wrap_point_without_flinching() {
        let angle = |moment: Moment| modulo(moment.0, 360.0);
        let found = invert_angular(angle, 5.0, Moment(350.0), Moment(370.0));
        assert!((found.0 - 365.0).abs() < 1e-6, "found {}", found.0);
    }

    #[test]
    fn bisection_reports_no_crossing_when_the_bracket_has_none() {
        let always_positive = |_: Moment| 1.0;
        assert!(bisect_rising(always_positive, Moment(0.0), Moment(1.0)).is_none());
        let always_negative = |_: Moment| -1.0;
        assert!(bisect_rising(always_negative, Moment(0.0), Moment(1.0)).is_none());
    }

    #[test]
    fn bisection_finds_a_rising_and_a_falling_crossing() {
        let rising = |moment: Moment| moment.0 - 0.375;
        let found = bisect_rising(rising, Moment(0.0), Moment(1.0));
        assert!(found.is_some());
        assert!((found.unwrap().0 - 0.375).abs() < 1e-6);

        let falling = |moment: Moment| 0.625 - moment.0;
        let found = bisect_falling(falling, Moment(0.0), Moment(1.0));
        assert!((found.unwrap().0 - 0.625).abs() < 1e-6);
    }

    #[test]
    fn a_fixed_number_of_rounds_answers_the_upper_end() {
        let found = bisect_until(|moment| moment.0 >= 0.3, Moment(0.0), Moment(1.0), 40);
        assert!(found.0 >= 0.3 && found.0 - 0.3 < 1e-11, "found {}", found.0);
        // No rounds: the bracket's upper end.
        assert_eq!(bisect_until(|_| true, Moment(0.0), Moment(1.0), 0).0, 1.0);
    }

    #[test]
    fn a_growing_angle_is_found_where_it_crosses_the_target() {
        // Thirteen degrees a day, starting at 350°: 10° is reached after
        // twenty degrees, across the wrap, and 350° again a revolution on.
        let angle = |moment: Moment| modulo(350.0 + 13.0 * moment.0, 360.0);
        let found = next_angle_crossing(angle, 10.0, Moment(0.0), 0.5);
        assert!((found.0 - 20.0 / 13.0).abs() < 1e-9, "found {}", found.0);
        let found = next_angle_crossing(angle, 350.0, Moment(0.1), 0.5);
        assert!((found.0 - 360.0 / 13.0).abs() < 1e-9, "found {}", found.0);
    }

    #[test]
    fn a_reversed_bracket_terminates_instead_of_spinning() {
        let angle = |moment: Moment| modulo(moment.0, 360.0);
        let found = invert_angular(angle, 100.0, Moment(200.0), Moment(0.0));
        assert!(found.0.is_finite());
    }
}
