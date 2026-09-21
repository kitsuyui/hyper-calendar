//! The units a span can be expressed in, and the mean length of each.
//!
//! # Why a month has a length here at all
//!
//! A month is not a fixed number of seconds; nor is a year, and nor — once
//! leap seconds and daylight saving exist — is a day. But a humaniser has to
//! answer "is this span better said in months or in days?", and that
//! question only has an answer if the units can be compared. So this module
//! gives every unit a *mean* length, and says so.
//!
//! The means are the Gregorian ones used by CLDR and ICU: the Gregorian
//! calendar repeats after 400 years containing 146 097 days, which is
//! 365.2425 days per year, 31 556 952 seconds exactly. A month is that
//! divided by twelve (2 629 746 s) and a quarter divided by four
//! (7 889 238 s); all three divide exactly, which is the reason this
//! particular year length is convenient. A week is seven 86 400-second days
//! and a day is 86 400 seconds — the nominal day, not the day a leap second
//! lands in.
//!
//! Anything that needs a *calendar* answer rather than a mean one must not
//! come through here. That is what [`crate::calendar_relative`] is for: it
//! counts days with [`hc_calendar::Rd`] and never multiplies a mean.

use hc_core::Duration;
use hc_units::Unit;
use hc_units::unit::{
    DAY, HOUR, MEAN_GREGORIAN_MONTH, MEAN_GREGORIAN_QUARTER, MEAN_GREGORIAN_YEAR, MINUTE, SECOND,
    WEEK,
};

/// A unit a span of time can be expressed in.
///
/// These are exactly the units of the CLDR `relativeTime` fields that carry
/// a numeric pattern, in increasing order of length. CLDR's `weekday`,
/// `dayperiod`, `zone` and `dayOfYear` fields are not spans and are not
/// here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TimeUnit {
    /// A second.
    Second,
    /// Sixty seconds.
    Minute,
    /// Sixty minutes.
    Hour,
    /// A nominal day of 86 400 seconds.
    Day,
    /// Seven nominal days.
    Week,
    /// A twelfth of a mean Gregorian year.
    Month,
    /// A quarter of a mean Gregorian year.
    Quarter,
    /// A mean Gregorian year of 365.2425 days.
    Year,
}

/// The mean length of a Gregorian year in seconds: 365.2425 × 86 400.
///
/// Exact, and divisible by both 12 and 4, so the month and quarter means
/// below are exact too.
///
/// Taken from [`hc_units`] rather than written here. `hc-units` is the crate
/// that owns "a named length of time", and this crate's question — which
/// unit to phrase a span in — is a different one that happens to need the
/// same numbers. Policy §2: one implementation, in the crate that owns the
/// idea.
pub const MEAN_YEAR_SECONDS: i64 = whole_seconds(MEAN_GREGORIAN_YEAR);

/// A unit's length in whole seconds, for the table below.
///
/// # Panics
///
/// If the unit is not a whole number of seconds, or does not fit in `i64`.
/// Every unit this module names is both, and these are `const` contexts, so
/// a violation is a compile error rather than a runtime one.
const fn whole_seconds(unit: Unit) -> i64 {
    assert!(
        unit.seconds.denominator() == 1,
        "a humanised unit must be a whole number of seconds"
    );
    let value = unit.seconds.numerator();
    assert!(
        value >= i64::MIN as i128 && value <= i64::MAX as i128,
        "a humanised unit must fit in i64 seconds"
    );
    #[expect(
        clippy::cast_possible_truncation,
        reason = "bounded to the i64 range on the line above"
    )]
    let seconds = value as i64;
    seconds
}

impl TimeUnit {
    /// Every unit, shortest first.
    pub const ALL: [Self; 8] = [
        Self::Second,
        Self::Minute,
        Self::Hour,
        Self::Day,
        Self::Week,
        Self::Month,
        Self::Quarter,
        Self::Year,
    ];

    /// The units a wall-clock duration decomposes into without ambiguity.
    ///
    /// Weeks, months and quarters are left out: a bare span has no calendar
    /// to anchor them to, and "1 month 2 days" would mean different amounts
    /// of time in February and in July.
    pub const CLOCK: [Self; 4] = [Self::Day, Self::Hour, Self::Minute, Self::Second];

    /// The CLDR field name of this unit.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Second => "second",
            Self::Minute => "minute",
            Self::Hour => "hour",
            Self::Day => "day",
            Self::Week => "week",
            Self::Month => "month",
            Self::Quarter => "quarter",
            Self::Year => "year",
        }
    }

    /// Parse a CLDR field name.
    #[must_use]
    pub fn from_field_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|unit| unit.as_str() == name)
    }

    /// The mean length of the unit in whole seconds.
    ///
    /// See the module documentation for where each constant comes from.
    #[must_use]
    pub const fn mean_seconds(self) -> i64 {
        match self {
            Self::Second => whole_seconds(SECOND),
            Self::Minute => whole_seconds(MINUTE),
            Self::Hour => whole_seconds(HOUR),
            Self::Day => whole_seconds(DAY),
            Self::Week => whole_seconds(WEEK),
            Self::Month => whole_seconds(MEAN_GREGORIAN_MONTH),
            Self::Quarter => whole_seconds(MEAN_GREGORIAN_QUARTER),
            Self::Year => MEAN_YEAR_SECONDS,
        }
    }

    /// The mean length of the unit as a [`Duration`].
    #[must_use]
    pub const fn mean_duration(self) -> Duration {
        Duration::from_secs(self.mean_seconds() as i128)
    }

    /// The next longer unit, if there is one.
    #[must_use]
    pub const fn larger(self) -> Option<Self> {
        match self {
            Self::Second => Some(Self::Minute),
            Self::Minute => Some(Self::Hour),
            Self::Hour => Some(Self::Day),
            Self::Day => Some(Self::Week),
            Self::Week => Some(Self::Month),
            Self::Month => Some(Self::Quarter),
            Self::Quarter => Some(Self::Year),
            Self::Year => None,
        }
    }

    /// The next shorter unit, if there is one.
    #[must_use]
    pub const fn smaller(self) -> Option<Self> {
        match self {
            Self::Second => None,
            Self::Minute => Some(Self::Second),
            Self::Hour => Some(Self::Minute),
            Self::Day => Some(Self::Hour),
            Self::Week => Some(Self::Day),
            Self::Month => Some(Self::Week),
            Self::Quarter => Some(Self::Month),
            Self::Year => Some(Self::Quarter),
        }
    }
}

impl core::fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A count of a unit, with an optional half.
///
/// The half is a separate flag rather than a fraction because that is the
/// only fraction a humaniser ever wants: languages have idioms for "half an
/// hour" and none for "0.37 of an hour". A formatter renders the half either
/// through the locale's own idiom, where the data states one, or as the
/// decimal `1.5`, which is what makes the plural rules see `v = 1` and pick
/// the form a written decimal takes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnitAmount {
    unit: TimeUnit,
    whole: i64,
    half: bool,
}

impl UnitAmount {
    /// A whole number of units.
    #[must_use]
    pub const fn whole(whole: i64, unit: TimeUnit) -> Self {
        Self {
            unit,
            whole,
            half: false,
        }
    }

    /// A number of units plus a half.
    ///
    /// The half is added in the direction of the sign, so
    /// `half_past(-1, Hour)` is an hour and a half into the past.
    #[must_use]
    pub const fn half_past(whole: i64, unit: TimeUnit) -> Self {
        Self {
            unit,
            whole,
            half: true,
        }
    }

    /// The unit.
    #[must_use]
    pub const fn unit(self) -> TimeUnit {
        self.unit
    }

    /// The whole part of the count, signed.
    #[must_use]
    pub const fn count(self) -> i64 {
        self.whole
    }

    /// Whether a half unit is added on top of [`Self::count`].
    #[must_use]
    pub const fn has_half(self) -> bool {
        self.half
    }

    /// Whether the amount is negative, that is in the past.
    ///
    /// A bare half with a zero whole part is positive: `0.5` has no sign of
    /// its own, so callers that need "half an hour ago" build it from a
    /// signed whole part or set the direction themselves.
    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.whole < 0
    }

    /// The count as a floating point number of units.
    #[must_use]
    pub fn as_f64(self) -> f64 {
        let magnitude = self.whole.unsigned_abs() as f64 + if self.half { 0.5 } else { 0.0 };
        if self.whole < 0 {
            -magnitude
        } else {
            magnitude
        }
    }

    /// The same amount with the sign removed.
    #[must_use]
    pub const fn abs(self) -> Self {
        Self {
            unit: self.unit,
            whole: self.whole.saturating_abs(),
            half: self.half,
        }
    }

    /// The mean length of the amount.
    ///
    /// # Errors
    ///
    /// Returns [`crate::HumanizeError::Overflow`] if the product does not
    /// fit a [`Duration`].
    pub fn mean_duration(self) -> crate::HumanizeResult<Duration> {
        let whole = self.unit.mean_duration().checked_mul_int(self.whole)?;
        if !self.half {
            return Ok(whole);
        }
        let mut half = self.unit.mean_duration().checked_div_int(2)?;
        if self.whole < 0 {
            half = half.checked_neg()?;
        }
        Ok(whole.checked_add(half)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_mean_gregorian_year_divides_exactly_into_months_and_quarters() {
        // 400 Gregorian years are 146 097 days, so a mean year is 365.2425
        // days. The point of that number here is that it divides by both 12
        // and 4 without a remainder, so no unit length is a rounded one.
        assert_eq!(MEAN_YEAR_SECONDS, 146_097 * 86_400 / 400);
        // The divisibility is a property of the number, not of where it is
        // stored, so it is asserted against the arithmetic rather than
        // against the constant it now comes from.
        assert_eq!(TimeUnit::Month.mean_seconds() * 12, MEAN_YEAR_SECONDS);
        assert_eq!(TimeUnit::Quarter.mean_seconds() * 4, MEAN_YEAR_SECONDS);
        assert_eq!(TimeUnit::Month.mean_seconds(), 2_629_746);
        assert_eq!(TimeUnit::Year.mean_seconds(), 31_556_952);
    }

    #[test]
    fn the_short_units_are_the_definitional_ones() {
        assert_eq!(TimeUnit::Minute.mean_seconds(), 60);
        assert_eq!(TimeUnit::Hour.mean_seconds(), 60 * 60);
        assert_eq!(TimeUnit::Day.mean_seconds(), 24 * 60 * 60);
        assert_eq!(TimeUnit::Week.mean_seconds(), 7 * 24 * 60 * 60);
    }

    #[test]
    fn the_unit_ladder_is_ordered_and_doubly_linked() {
        for pair in TimeUnit::ALL.windows(2) {
            assert!(pair[0] < pair[1]);
            assert!(pair[0].mean_seconds() < pair[1].mean_seconds());
            assert_eq!(pair[0].larger(), Some(pair[1]));
            assert_eq!(pair[1].smaller(), Some(pair[0]));
        }
        assert_eq!(TimeUnit::Second.smaller(), None);
        assert_eq!(TimeUnit::Year.larger(), None);
    }

    #[test]
    fn field_names_round_trip_through_their_cldr_spelling() {
        for unit in TimeUnit::ALL {
            assert_eq!(TimeUnit::from_field_name(unit.as_str()), Some(unit));
        }
        assert_eq!(TimeUnit::from_field_name("fortnight"), None);
    }

    #[test]
    fn a_half_takes_the_sign_of_its_whole_part() {
        let future = UnitAmount::half_past(1, TimeUnit::Hour);
        let past = UnitAmount::half_past(-1, TimeUnit::Hour);
        assert!(!future.is_negative());
        assert!(past.is_negative());
        assert!((future.as_f64() - 1.5).abs() < 1e-12);
        assert!((past.as_f64() + 1.5).abs() < 1e-12);
        assert_eq!(past.abs().count(), 1);
    }

    #[test]
    fn a_bare_half_is_positive_because_zero_has_no_sign() {
        let half = UnitAmount::half_past(0, TimeUnit::Hour);
        assert!(!half.is_negative());
        assert!((half.as_f64() - 0.5).abs() < 1e-12);
    }

    #[test]
    fn an_amount_converts_to_the_mean_duration_it_stands_for() {
        let two_and_a_half = UnitAmount::half_past(2, TimeUnit::Hour);
        assert_eq!(
            two_and_a_half.mean_duration().expect("in range"),
            Duration::from_secs(9_000)
        );
        let back = UnitAmount::half_past(-2, TimeUnit::Hour);
        assert_eq!(
            back.mean_duration().expect("in range"),
            Duration::from_secs(-9_000)
        );
    }

    #[test]
    fn an_amount_too_large_for_a_duration_is_an_error_not_a_wrap() {
        let absurd = UnitAmount::whole(i64::MAX, TimeUnit::Year);
        assert!(absurd.mean_duration().is_ok());
        assert_eq!(
            UnitAmount::whole(3, TimeUnit::Day).mean_duration(),
            Ok(Duration::from_days(3))
        );
    }
}
