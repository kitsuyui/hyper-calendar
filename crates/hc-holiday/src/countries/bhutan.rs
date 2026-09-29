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
//! year. Its notification of 7 September 2021, "Change of dates for
//! Thimphu Dromche and Tshechu" (`moha-bt-notification-2021`), moved these
//! two, and its page gives the new dates only in
//! an image, which was not read; so 2021 is a gap for both, not a
//! prediction. A year in which one of the days is skipped or repeated is
//! a gap too.
//!
//! The other districts' tshechus are not carried: the lists say only that
//! their days are "confirmed by the respective Dzongkhag Administration",
//! and no district's confirmation was read. The names are the lists'.

use hc_calendar::Rd;
use hc_calendars_lunar::tibetan;
use hc_calendars_regional::tibetan_almanac::bhutanese_winter_solstice;

use crate::rule::{Days, HolidayRule, Listing, Rule, TibetanMonth};

use super::asia::{BT_RULES, bt_predicted, bt_read};
use crate::rule::joined;

/// The Bhutanese Winter Solstice of `year`: the day the mean Sun of the
/// Bhutanese calendar reaches 250°, 18;45 in mansions, as Henning's
/// Bhutanese program computes it (Janson, "Tibetan calendar mathematics",
/// Appendix A.4; Henning, "Bhutan calendars"), which gives 2 January in
/// both the Ministry's lists read, 2025 and 2026, and every year from 2011
/// to 2019 of Henning's Bhutanese almanacs; see
/// `hc_calendars_regional::tibetan_almanac::bhutanese_winter_solstice`.
fn winter_solstice(year: i64) -> Days {
    bhutanese_winter_solstice(&tibetan::TIBETAN_BHUTAN, year).map_or_else(
        |_| Days::new(),
        |instant| Days::one(Rd::from_julian_day_number(instant.floor() as i64)),
    )
}

/// The Winter Solstice predicted by [`winter_solstice`] before the lists'
/// years (`before`) or after them, over the years the Bhutanese calendar
/// converts. The Ministry prints the day each year, so a prediction is
/// approximate, as the lists' lunar days are.
pub(super) const fn winter_solstice_predicted(before: bool) -> HolidayRule {
    let rule = HolidayRule::fixed_public(
        "Winter Solstice",
        "",
        Rule::Tabulated {
            function: winter_solstice,
            first_year: tibetan::MIN_YEAR,
            last_year: tibetan::MAX_YEAR,
        },
    )
    .approximate();
    if before {
        rule.years(None, Some(super::asia::BT_FIRST as i32 - 1))
    } else {
        rule.years(Some(super::asia::BT_LAST as i32 + 1), None)
    }
}

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

/// The year the Ministry's notification of 7 September 2021 moved both
/// festivals to dates not read.
const MOVED: i32 = 2021;

/// A prediction before the lists, in Thimphu, the years before [`MOVED`]
/// (`early`) or those between it and the lists.
const fn thimphu_before(name: &'static str, day: u8, early: bool) -> HolidayRule {
    let rule = bt_predicted(name, EIGHTH, day, true).in_regions(THIMPHU);
    if early {
        rule.years(None, Some(MOVED - 1))
    } else {
        rule.years(Some(MOVED + 1), Some(super::asia::BT_FIRST as i32 - 1))
    }
}

/// A festival in [`MOVED`], a gap.
const fn thimphu_moved(name: &'static str) -> HolidayRule {
    HolidayRule::fixed_public(name, "", Rule::UNREAD)
        .years(Some(MOVED), Some(MOVED))
        .in_regions(THIMPHU)
}

/// The days for Thimphu alone: the lists' days in their years, the
/// predictions on `tibetan-bhutan` before and after, and a gap in the
/// year the Ministry moved them.
pub static THIMPHU_DAYS: &[HolidayRule] = &[
    bt_read(DRUBCHOE, LISTED.named(DRUBCHOE)).in_regions(THIMPHU),
    thimphu_before(DRUBCHOE, 6, true),
    thimphu_before(DRUBCHOE, 6, false),
    thimphu_moved(DRUBCHOE),
    bt_predicted(DRUBCHOE, EIGHTH, 6, false).in_regions(THIMPHU),
    bt_read(TSHECHU, LISTED.named(TSHECHU)).in_regions(THIMPHU),
    thimphu_before(TSHECHU, 10, true),
    thimphu_before(TSHECHU, 11, true),
    thimphu_before(TSHECHU, 12, true),
    thimphu_before(TSHECHU, 10, false),
    thimphu_before(TSHECHU, 11, false),
    thimphu_before(TSHECHU, 12, false),
    thimphu_moved(TSHECHU),
    bt_predicted(TSHECHU, EIGHTH, 10, false).in_regions(THIMPHU),
    bt_predicted(TSHECHU, EIGHTH, 11, false).in_regions(THIMPHU),
    bt_predicted(TSHECHU, EIGHTH, 12, false).in_regions(THIMPHU),
];

/// Bhutan's nationwide rules and Thimphu's days, the table
/// [`super::BHUTAN`] evaluates.
pub(super) static RULES: [HolidayRule; BT_RULES.len() + THIMPHU_DAYS.len()] =
    joined(&[BT_RULES, THIMPHU_DAYS]);
