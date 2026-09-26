//! The arithmetic shared by the Symmetry calendars.
//!
//! Irv Bromberg's two proposals differ in exactly one thing: how the 364
//! days of an ordinary year are cut into months. Symmetry454 uses weeks —
//! 4, 5, 4 per quarter — so every month begins on a Monday. Symmetry010
//! uses days — 30, 31, 30 — which reads more like the Gregorian calendar
//! and gives up the fixed weekday within a month.
//!
//! Everything else is identical, including the leap rule that is the actual
//! proposal: 52 leap weeks in 293 years, chosen so the mean year is
//! 365.24232 days, closer to the mean northward-equinoctial year than the
//! Gregorian 365.2425. So the rule lives here once and the two layouts are
//! thin layers over it, per policy §2; the arithmetic under the rule is
//! the crate-private `leap_week`'s, which the Hermetic Leap Week Calendar shares.
//!
//! # Which leap cycle
//!
//! Bromberg's survey of leap cycles gives the 52/293 cycle as the one
//! "preferred for the Symmetry454 and Symmetry010 calendars", and says
//! why: the shortest cycle that tracks the mean northward equinox for the
//! past five millennia and the next four or five, 294 × 364 days long, its
//! leap weeks spread symmetrically. The same page tabulates 93/524, which
//! aligns the equinox slightly more tightly for about 500 years fewer, and
//! names 327 and 389 years as the best short cycles for the north
//! solstitial year, which is not these calendars' target. Only the
//! preferred cycle is carried: the others are the author's own comparison,
//! not rival conventions anyone keeps, so policy §5 does not ask for them.
//!
//! **Sources:** Irv Bromberg, *The Symmetry454 Calendar*, University of
//! Toronto (`bromberg-symmetry454`), which specifies both variants; and his
//! "Calendar Leap Cycles", `individual.utoronto.ca/kalendis/leap/`, read
//! 2026-09-26 in the Wayback Machine's copy of 30 November 2020
//! (`bromberg-leap-cycles`), for the choice of cycle. The live pages, now
//! at `kalendis.free.nf`, sit behind a script check and were not read.

use hc_calendar::{CalendarResult, Rd};

use crate::leap_week::{self, LeapWeekRule};

/// Days in an ordinary year: 52 weeks.
pub const ORDINARY_YEAR_DAYS: i64 = leap_week::ORDINARY_YEAR_DAYS;

/// Days in a leap year: 53 weeks.
pub const LEAP_YEAR_DAYS: i64 = leap_week::LEAP_YEAR_DAYS;

/// Years in the leap cycle.
pub const CYCLE_YEARS: i64 = 293;

/// Leap years per cycle.
pub const LEAPS_PER_CYCLE: i64 = 52;

/// Days in the whole cycle.
pub const CYCLE_DAYS: i64 = RULE.cycle_days();

/// The earliest year these implementations convert.
pub const MIN_YEAR: i64 = 1;

/// The latest year these implementations convert.
pub const MAX_YEAR: i64 = 99_999;

/// Bromberg's rule, `(52 · year + 146) mod 293 < 52`, with year 1 on
/// Monday 1 January 1, [`Rd(1)`][Rd].
const RULE: LeapWeekRule = LeapWeekRule {
    leaps: LEAPS_PER_CYCLE,
    cycle: CYCLE_YEARS,
    offset: 146,
    epoch: 1,
    min_year: MIN_YEAR,
    max_year: MAX_YEAR,
};

/// Whether `year` is a leap year, with a fifty-third week in December.
#[must_use]
pub const fn is_leap_year(year: i64) -> bool {
    RULE.is_leap_year(year)
}

/// The number of days in `year`.
#[must_use]
pub const fn days_in_year(year: i64) -> u16 {
    RULE.days_in_year(year)
}

/// The fixed day on which `year` begins, without validation.
#[must_use]
pub const fn new_year_raw(year: i64) -> i64 {
    RULE.new_year_raw(year)
}

/// The fixed day on which `year` begins, always a Monday.
///
/// # Errors
///
/// Returns [`hc_calendar::CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub const fn new_year(year: i64) -> CalendarResult<Rd> {
    RULE.new_year(year)
}

/// The earliest fixed day these implementations convert.
pub const EARLIEST: Rd = RULE.earliest();

/// The latest fixed day these implementations convert.
pub const LATEST: Rd = RULE.latest();

/// The days in `month` of `year` under a given ordinary-year layout.
///
/// The leap week is always added to December, in both variants.
#[must_use]
pub const fn days_in_month(layout: &[u8; 12], year: i64, month: u8) -> Option<u8> {
    RULE.days_in_month(layout, year, month)
}

/// The fixed day of a date under a given layout.
///
/// # Errors
///
/// Returns [`hc_calendar::CalendarError::YearOutOfRange`],
/// [`hc_calendar::CalendarError::MonthOutOfRange`] or
/// [`hc_calendar::CalendarError::DayOutOfRange`].
pub const fn to_fixed(layout: &[u8; 12], year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
    RULE.fixed_day(layout, year, month, day)
}

/// The year, month and day of a fixed day under a given layout.
///
/// # Errors
///
/// Returns [`hc_calendar::CalendarError::BeforeEpoch`] or
/// [`hc_calendar::CalendarError::AfterSupportedRange`] outside
/// [`EARLIEST`]..=[`LATEST`].
pub const fn from_fixed(layout: &[u8; 12], rd: Rd) -> CalendarResult<(i64, u8, u8)> {
    RULE.date_of(layout, rd)
}
