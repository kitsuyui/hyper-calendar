//! Vanuatu's provincial days, each scoped to its province's ISO 3166-2
//! code.
//!
//! The Public Holidays Act [Cap. 114] does not name them. The Government's
//! list of holidays (gov.vu, "Holidays") gives six days of type
//! "Provincial Holiday" beside its national ones, one for each province,
//! on a fixed date: Shefa Day on 18 June, Penama Day on 16 September,
//! Sanma Day on 24 September, Torba Day on 2 October, Tafea Day on
//! 8 October and Malampa Day on 10 October. The list as read dates Good
//! Friday on 10 April, Easter Monday on 13 April and Ascension on 21 May,
//! which is 2020, so the days are carried from 2020, the years before a
//! gap: no earlier list, and no provincial council's resolution, was read. They are days off in
//! their province, as the list gives them, and none moves off a Sunday:
//! section 3's Sunday rule is the Act's, for the Act's own holidays. The
//! names are the list's.

use crate::rule::{HolidayRule, Rule};

use super::oceania::VU_RULES;
use crate::rule::joined;

/// The year of the Government's list read.
const FIRST: i32 = 2020;

/// A province's day on a fixed date, in `region`, answered from the
/// list's year, the years before a gap.
const fn provincial(
    name: &'static str,
    month: u8,
    day: u8,
    region: &'static [&'static str],
) -> HolidayRule {
    HolidayRule::fixed_public(name, "", Rule::gregorian(month, day))
        .read_from(FIRST)
        .in_regions(region)
}

/// The six provincial days, in date order.
pub static PROVINCIAL_DAYS: &[HolidayRule] = &[
    provincial("Shefa Day", 6, 18, &["VU-SEE"]),
    provincial("Penama Day", 9, 16, &["VU-PAM"]),
    provincial("Sanma Day", 9, 24, &["VU-SAM"]),
    provincial("Torba Day", 10, 2, &["VU-TOB"]),
    provincial("Tafea Day", 10, 8, &["VU-TAE"]),
    provincial("Malampa Day", 10, 10, &["VU-MAP"]),
];

/// Vanuatu's national rules and its provincial days, the table
/// [`super::VANUATU`] evaluates.
pub(super) static RULES: [HolidayRule; VU_RULES.len() + PROVINCIAL_DAYS.len()] =
    joined(&[VU_RULES, PROVINCIAL_DAYS]);
