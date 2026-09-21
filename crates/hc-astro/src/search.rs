//! Root finding on angles that run round a circle.
//!
//! Solar longitude and lunar phase are both monotonic-but-wrapping: they
//! climb from 0° to 360° and then start again. A plain sign-change bisection
//! cannot see that, so the predicate used here is the one from Reingold &
//! Dershowitz, *Calendrical Calculations*, 4th ed., §14.4: the angle has
//! reached its target when `(f(t) − target) mod 360 < 180`.

use hc_calendar::fixed::Moment;

use crate::util::modulo;

/// How close a bisection has to get before it stops, in days.
///
/// 10⁻⁷ days is 8.6 milliseconds, far below the accuracy of any series in
/// this crate. The point of going this fine is not precision but
/// repeatability: two callers asking the same question get bit-identical
/// answers.
pub(crate) const BISECTION_TOLERANCE_DAYS: f64 = 1e-7;

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
pub(crate) fn invert_angular<F>(f: F, target: f64, low: Moment, high: Moment) -> Moment
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
pub(crate) fn bisect_rising<F>(f: F, low: Moment, high: Moment) -> Option<Moment>
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
pub(crate) fn bisect_falling<F>(f: F, low: Moment, high: Moment) -> Option<Moment>
where
    F: Fn(Moment) -> f64,
{
    bisect_rising(|moment| -f(moment), low, high)
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
    fn a_reversed_bracket_terminates_instead_of_spinning() {
        let angle = |moment: Moment| modulo(moment.0, 360.0);
        let found = invert_angular(angle, 100.0, Moment(200.0), Moment(0.0));
        assert!(found.0.is_finite());
    }
}
