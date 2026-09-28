//! The pairings of days and locales that the tests rendering every
//! calendar in every locale check, shared by `readable_dates.rs` and
//! `vocabulary.rs`.
//!
//! Such a test renders each calendar's days in every carried locale, and
//! with more than fifty locales it is the render, not the conversion, that
//! takes the time. A release build, which CI's release-mode job runs,
//! renders every day in every locale. A debug build, which the coverage job
//! runs instrumented, renders one pairing in [`STRIDE`], staggered, so that
//! every locale still renders some of every calendar's days and every day
//! is still rendered in some of the locales; the days a test must render in
//! full, and the calendar's own language, it renders in every locale.

/// How sparsely a debug build pairs a calendar's days with the locales.
pub const STRIDE: usize = if cfg!(debug_assertions) { 6 } else { 1 };

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
