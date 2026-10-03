//! The pairings of days and locales that the tests rendering every
//! calendar in every locale check, shared by `readable_dates.rs` and
//! `vocabulary.rs`.
//!
//! Such a test renders each calendar's days in every carried locale, and
//! with more than fifty locales it is the render, not the conversion, that
//! takes the time. A release build, which CI's release-mode job runs,
//! renders every day in every locale. A debug build renders one pairing in
//! [`STRIDE`], staggered, so that every locale still renders some of every
//! calendar's days and every day is still rendered in some of the locales;
//! the days a test must render in full, and the calendar's own language, it
//! renders in every locale. The coverage job's instrumented build, which
//! runs the same tests one at a time and several times slower, renders one
//! in [`COVERAGE_STRIDE`], still short of the number of locales, so that the
//! two properties above hold there too.

use hyper_calendar::hc_core::sweep::INSTRUMENTED;

/// How sparsely a debug build pairs a calendar's days with the locales.
///
/// More than sixty locales are carried, and a stride below that keeps every
/// day rendered in at least one of them.
pub const STRIDE: usize = if INSTRUMENTED {
    COVERAGE_STRIDE
} else if cfg!(debug_assertions) {
    6
} else {
    1
};

/// The stride of a build instrumented for coverage: a prime still below
/// the number of locales, so that each day is rendered in one or two of
/// them.
pub const COVERAGE_STRIDE: usize = 47;

/// Every how many locales a test that renders each of its days in every
/// locale takes one, when the build is instrumented for coverage.
pub const COVERAGE_LOCALE_STEP: usize = 3;

/// Whether the `locale`th of `locales` locales is rendered by a test that
/// renders each of its days in every locale: all of them unless the build
/// is instrumented for coverage, and then the first of every
/// [`COVERAGE_LOCALE_STEP`] and the last.
#[allow(dead_code)]
pub fn locale_rendered(locale: usize, locales: usize) -> bool {
    !INSTRUMENTED || locale.is_multiple_of(COVERAGE_LOCALE_STEP) || locale + 1 >= locales
}

/// Whether the `day`th of a calendar's `days` days is rendered in the
/// `locale`th of `locales` locales, the last of which is the calendar's own
/// language: always in a release build; in a debug build when `full`, in
/// the last locale, and on one pairing in [`STRIDE`] otherwise. With fewer
/// days than the stride, the stride is the number of days, so that each
/// locale still renders one of them.
pub fn paired(day: usize, days: usize, locale: usize, locales: usize, full: bool) -> bool {
    let stride = STRIDE.min(days).max(1);
    full || locale + 1 == locales || (day + locale).is_multiple_of(stride)
}
