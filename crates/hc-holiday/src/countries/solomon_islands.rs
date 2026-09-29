//! Solomon Islands' provincial days, each scoped to its province's ISO
//! 3166-2 code.
//!
//! Section 6 of the Public Holidays Act (Cap. 151) lets the Minister
//! appoint a public holiday for a province, and the Minister for Home
//! Affairs appoints the provinces' days each year by notice in the
//! Gazette. The notice of 3 November 2025 for 2026, in the Extra-Ordinary
//! Gazette of 5 November 2025, is carried as the Island Sun of 13 January
//! 2026 reports it — the Gazette itself, a PDF, was not read: Choiseul on
//! 25 February, Isabel on 2 June, Temotu on 8 June, Central on 29 June,
//! Rennell and Bellona on 20 July, Guadalcanal's 1 August "with July 31 to
//! be observed as the public holiday", Makira-Ulawa on 3 August, Malaita's
//! 15 August with 14 August observed, and Western on 7 December. The day
//! kept is the one observed.
//!
//! The days are appointed each year, so 2026 is the only year carried:
//! every other year, whose notice was not read, reports them as a gap.
//! Honiara, the Capital Territory, is given no day, and the table lists it
//! among the subdivisions read. The names are the crate's.

use crate::rule::{HolidayRule, Listing, Rule};

use super::oceania::SB_RULES;
use crate::rule::joined;

/// The year of the notice read.
const YEAR: i64 = 2026;

/// The days the notice for 2026 appoints, by province, as observed.
static APPOINTED: Listing = Listing::Named(&[
    (2026, 2, 25, "SB-CH"),
    (2026, 6, 2, "SB-IS"),
    (2026, 6, 8, "SB-TE"),
    (2026, 6, 29, "SB-CE"),
    (2026, 7, 20, "SB-RB"),
    (2026, 7, 31, "SB-GU"),
    (2026, 8, 3, "SB-MK"),
    (2026, 8, 14, "SB-ML"),
    (2026, 12, 7, "SB-WE"),
]);

/// A province's appointed day, in `region`, which is also its row of
/// [`APPOINTED`].
const fn provincial(name: &'static str, region: &'static [&'static str]) -> HolidayRule {
    HolidayRule::fixed_public(
        name,
        "",
        Rule::listed(APPOINTED.named(region[0]), YEAR, YEAR),
    )
    .in_regions(region)
}

/// The nine provincial days, in date order for 2026.
pub static PROVINCIAL_DAYS: &[HolidayRule] = &[
    provincial("Choiseul Province Day", &["SB-CH"]),
    provincial("Isabel Province Day", &["SB-IS"]),
    provincial("Temotu Province Day", &["SB-TE"]),
    provincial("Central Province Day", &["SB-CE"]),
    provincial("Rennell and Bellona Province Day", &["SB-RB"]),
    provincial("Guadalcanal Province Day", &["SB-GU"]),
    provincial("Makira-Ulawa Province Day", &["SB-MK"]),
    provincial("Malaita Province Day", &["SB-ML"]),
    provincial("Western Province Day", &["SB-WE"]),
];

/// Solomon Islands' national rules and its provincial days, the table
/// [`super::SOLOMON_ISLANDS`] evaluates.
pub(super) static RULES: [HolidayRule; SB_RULES.len() + PROVINCIAL_DAYS.len()] =
    joined(&[SB_RULES, PROVINCIAL_DAYS]);
