//! Bhutan's days for Thimphu alone, scoped to `BT-15`, the Thimphu
//! district.
//!
//! The Ministry of Home Affairs' government holiday lists for 2025 and 2026
//! give two festivals for Thimphu only, each with its Bhutanese date as
//! the national days have theirs: Thimphu Drubchoe on the 6th day of the
//! 8th month, and Thimphu Tshechu on the 10th to the 12th. They are taken
//! from the lists in 2025 and 2026 — 28 September and 2–4 October 2025,
//! 17 September and 21–23 September 2026 — and predicted in the other
//! years on `tibetan-bhutan`, marked approximate, as the lists' national
//! days are ([`super::asia`]'s Bhutan table): the Ministry settles each
//! year, and its notification of 7 September 2021, "Change of dates for
//! Thimphu Dromche and Tshechu", moved these two, the new dates not being
//! on its page. A year in which one of the days is skipped or repeated is
//! a gap.
//!
//! The other districts' tshechus are not carried: the lists say only that
//! their days are "confirmed by the respective Dzongkhag Administration",
//! and no district's confirmation was read. The names are the lists'.

use crate::rule::{HolidayRule, Listing, TibetanMonth};

use super::asia::{BT_RULES, bt_predicted, bt_read};
use super::joined;

/// The Thimphu district.
const THIMPHU: &[&str] = &["BT-15"];

/// The eighth Bhutanese month, the two festivals'.
const EIGHTH: TibetanMonth = TibetanMonth::Regular(8);

/// Thimphu Drubchoe.
const DRUBCHOE: &str = "Thimphu Drubchoe";
/// Thimphu Tshechu.
const TSHECHU: &str = "Thimphu Tshechu";

/// The two festivals' days, as the lists for 2025 and 2026 give them.
static LISTED: Listing = Listing::Named(&[
    (2025, 9, 28, DRUBCHOE),
    (2025, 10, 2, TSHECHU),
    (2025, 10, 3, TSHECHU),
    (2025, 10, 4, TSHECHU),
    (2026, 9, 17, DRUBCHOE),
    (2026, 9, 21, TSHECHU),
    (2026, 9, 22, TSHECHU),
    (2026, 9, 23, TSHECHU),
]);

/// The days for Thimphu alone: the lists' days in their years, the
/// predictions on `tibetan-bhutan` before and after.
pub static THIMPHU_DAYS: &[HolidayRule] = &[
    bt_read(DRUBCHOE, LISTED.named(DRUBCHOE)).in_regions(THIMPHU),
    bt_predicted(DRUBCHOE, EIGHTH, 6, true).in_regions(THIMPHU),
    bt_predicted(DRUBCHOE, EIGHTH, 6, false).in_regions(THIMPHU),
    bt_read(TSHECHU, LISTED.named(TSHECHU)).in_regions(THIMPHU),
    bt_predicted(TSHECHU, EIGHTH, 10, true).in_regions(THIMPHU),
    bt_predicted(TSHECHU, EIGHTH, 11, true).in_regions(THIMPHU),
    bt_predicted(TSHECHU, EIGHTH, 12, true).in_regions(THIMPHU),
    bt_predicted(TSHECHU, EIGHTH, 10, false).in_regions(THIMPHU),
    bt_predicted(TSHECHU, EIGHTH, 11, false).in_regions(THIMPHU),
    bt_predicted(TSHECHU, EIGHTH, 12, false).in_regions(THIMPHU),
];

/// Bhutan's nationwide rules and Thimphu's days, the table
/// [`super::BHUTAN`] evaluates.
pub(super) static RULES: [HolidayRule; BT_RULES.len() + THIMPHU_DAYS.len()] =
    joined(BT_RULES, THIMPHU_DAYS);
