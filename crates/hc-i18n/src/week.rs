//! Week rules by locale: which day a week starts on and which week is a
//! year's or a month's first, read from CLDR's `weekData`.
//!
//! A week number depends on two facts about the place that writes it, and
//! UTS #35 Part 4, "Week of Year" and "Week Elements", gives both: the
//! `firstDay` of the week and `minDays`, "the minimal days required in the
//! first week of a month or year". The arithmetic that turns the pair into
//! a week of the year, a week-numbering year and a week of the month is
//! `hc-calendar`'s [`WeekRule`], written once for every week number the
//! workspace gives; this module reads a locale's pair, [`for_locale`]. The
//! United States' Sunday and 1 and Germany's Monday and 4 are two places on
//! one scale, and ISO 8601's Monday and 4 is one of its points,
//! [`WeekRule::ISO`].
//!
//! `docs/systems/week-rules.md` works a date through the rule and says how it
//! was measured.
//!
//! # Sources
//!
//! UTS #35 Part 4, version 48.2, "Week of Year" and "Week Elements" (read
//! 2026-10-03); CLDR 48 `common/supplemental/supplementalData.xml`,
//! `weekData/minDays` and `weekData/firstDay`, as `scripts/locales-cldr.py`
//! reads them. Node 22's `Intl.Locale.getWeekInfo` (ICU 76.1, CLDR 46) gives
//! the same `firstDay` and `minimalDays` for the locales the tests name.

pub use hc_calendar::week::WeekRule;

use crate::locale::{Locale, region_min_days};
use crate::names;

/// The rule of a locale: its first day of the week, which a `-u-fw-`
/// extension overrides ([`names::first_day_of_week`]), and the `minDays`
/// of its region, or of the region its language is likeliest in where
/// the tag has none.
#[must_use]
pub fn for_locale(locale: &Locale) -> WeekRule {
    let min_days = match locale.region() {
        Some(region) => region_min_days(region),
        None => names::locale_data(locale).min_days,
    };
    WeekRule::new(names::first_day_of_week(locale), min_days)
}

#[cfg(test)]
mod tests {
    use hc_calendar::{Rd, Weekday, gregorian};

    use super::*;

    fn day(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    fn locale(tag: &str) -> Locale {
        Locale::parse(tag).unwrap()
    }

    /// Node 22's `Intl.Locale.getWeekInfo` (ICU 76.1, CLDR 46), read
    /// 2026-10-03: `firstDay` (1 Monday … 7 Sunday) and `minimalDays`.
    #[test]
    fn a_locales_rule_is_cldrs_week_data() {
        for (tag, first, min) in [
            ("en-US", Weekday::Sunday, 1),
            ("en", Weekday::Sunday, 1),
            ("en-GB", Weekday::Monday, 4),
            ("de", Weekday::Monday, 4),
            ("fr", Weekday::Monday, 4),
            ("es", Weekday::Monday, 4),
            ("it", Weekday::Monday, 4),
            ("ru", Weekday::Monday, 4),
            ("ja", Weekday::Sunday, 1),
            ("ko", Weekday::Sunday, 1),
            ("th", Weekday::Sunday, 1),
            ("hi", Weekday::Sunday, 1),
            ("he", Weekday::Sunday, 1),
            ("pt", Weekday::Sunday, 1),
            ("pt-PT", Weekday::Sunday, 4),
            ("ar", Weekday::Saturday, 1),
            ("ar-EG", Weekday::Saturday, 1),
            ("fa", Weekday::Saturday, 1),
            ("zh-Hant", Weekday::Sunday, 1),
        ] {
            let rule = for_locale(&locale(tag));
            assert_eq!((rule.first_day(), rule.min_days()), (first, min), "{tag}");
        }
    }

    /// A week-of-year number follows the locale: 2021-01-01, a Friday, is
    /// week 1 of 2021 where Sunday starts the week and one day will do, and
    /// week 53 of 2020 under ISO 8601's Monday and four.
    #[test]
    fn a_locale_numbers_its_weeks() {
        let new_year = day(2021, 1, 1);
        assert_eq!(
            for_locale(&locale("en-US")).week_of_year(new_year),
            (2021, 1)
        );
        assert_eq!(for_locale(&locale("de")).week_of_year(new_year), (2020, 53));
        // `-u-fw-` moves the first day and keeps the region's minimum days.
        let monday_us = for_locale(&locale("en-US-u-fw-mon"));
        assert_eq!(
            (monday_us.first_day(), monday_us.min_days()),
            (Weekday::Monday, 1)
        );
    }
}
