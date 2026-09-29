//! The day periods beyond am and pm, as data: Unicode CLDR 48's day period
//! rules and names, for the CLDR pattern fields `b` and `B`.
//!
//! UTS #35 Part 4, *Dates*, version 48.2, "Day Period Rule Sets": besides
//! am and pm, which every locale has, a language may name *midnight* at
//! 00:00 and *noon* at 12:00 — English does, German does not, having no
//! word for exactly 12:00 — and may divide the day into *flexible*
//! periods, `morning1` to `night2`, that "completely cover the 24 hours":
//! English's morning from midnight to noon, afternoon, evening from 18:00
//! and night from 21:00, Japanese's 夜中 from 23:00 to 04:00 across
//! midnight. `b` writes midnight and noon where the language has them, and
//! am or pm otherwise; `B` writes the flexible period, and midnight and noon
//! at those instants. Where a language has no rules, "the computation of
//! dayPeriods falls back to AM/PM".
//!
//! `scripts/day-periods-cldr.py` generates the data from the format rule
//! set of `supplemental/dayPeriods.xml` and each carried locale's format
//! names in `common/main/<file>.xml` [cldr48-day-periods]; am and pm are
//! the entries' own, [`crate::names::day_period_name`]. A language's rules
//! are found by truncating the locale's tag, as the file keys them by
//! language. `docs/systems/zone-names.md` works the rules and names
//! through with examples.

use crate::locale::Locale;
use crate::names::NameWidth;

mod cldr48;

/// A day period other than am and pm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlexibleDayPeriod {
    /// 00:00 exactly, where the language has a word for it.
    Midnight,
    /// 12:00 exactly, where the language has a word for it.
    Noon,
    /// The first morning period.
    Morning1,
    /// The second morning period.
    Morning2,
    /// The first afternoon period.
    Afternoon1,
    /// The second afternoon period.
    Afternoon2,
    /// The first evening period.
    Evening1,
    /// The second evening period.
    Evening2,
    /// The first night period.
    Night1,
    /// The second night period.
    Night2,
}

impl FlexibleDayPeriod {
    /// Every period, in the order of the name tables.
    pub const ALL: [Self; 10] = [
        Self::Midnight,
        Self::Noon,
        Self::Morning1,
        Self::Morning2,
        Self::Afternoon1,
        Self::Afternoon2,
        Self::Evening1,
        Self::Evening2,
        Self::Night1,
        Self::Night2,
    ];

    /// CLDR's identifier: `midnight`, `morning1`.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Midnight => "midnight",
            Self::Noon => "noon",
            Self::Morning1 => "morning1",
            Self::Morning2 => "morning2",
            Self::Afternoon1 => "afternoon1",
            Self::Afternoon2 => "afternoon2",
            Self::Evening1 => "evening1",
            Self::Evening2 => "evening2",
            Self::Night1 => "night1",
            Self::Night2 => "night2",
        }
    }

    const fn index(self) -> usize {
        self as usize
    }

    const fn is_fixed(self) -> bool {
        matches!(self, Self::Midnight | Self::Noon)
    }
}

/// A language's rules: (period, from, before), in minutes after midnight.
type Rules = &'static [(FlexibleDayPeriod, u16, u16)];

/// The format rule set of `locale`: the one `dayPeriods.xml` lists under
/// its tag, else under the tag truncated from the right, a subtag at a
/// time, to its language.
///
/// The rule sets are keyed by language, as the plural rules are (`zh`,
/// `yue`, `pa`), with a few by language and script or region (`hi_Latn`,
/// `es_CO`), so they are looked up by truncation, UTS #35 Part 1's rule,
/// and not along the names' fallback chain: `parentLocales` send
/// `zh-Hant` and `yue-Hans` straight to root, which would leave them with
/// am and pm alone, while their files name 午夜, 清晨 and 晚上 for the
/// periods of `zh`'s rules. ICU4C's `DayPeriodRules::getInstance` walks
/// the same truncation (`icu-zone-format-sources`).
fn rules(locale: &Locale) -> Option<Rules> {
    let rendered = locale.without_extensions().rendered()?;
    let mut tag = rendered.as_str();
    loop {
        if let Some((_, rows)) = cldr48::RULES.iter().find(|(key, _)| *key == tag) {
            return Some(*rows);
        }
        tag = &tag[..tag.rfind('-')?];
    }
}

/// Whether `locale`'s language has a word for this fixed period, midnight
/// or noon, in its rules.
#[must_use]
pub fn has_fixed_period(locale: &Locale, period: FlexibleDayPeriod) -> bool {
    period.is_fixed()
        && rules(locale).is_some_and(|rows| rows.iter().any(|(kind, _, _)| *kind == period))
}

/// The flexible period of `locale`'s rules that covers the minute of the
/// day, `0..1440`; `None` where the language has no rules. The fixed
/// periods are not answered here: a caller asks [`has_fixed_period`] at
/// 00:00 and 12:00.
#[must_use]
pub fn flexible_period(locale: &Locale, minute_of_day: u16) -> Option<FlexibleDayPeriod> {
    rules(locale)?
        .iter()
        .find(|(kind, from, before)| {
            !kind.is_fixed()
                && if from < before {
                    (*from..*before).contains(&minute_of_day)
                } else {
                    minute_of_day >= *from || minute_of_day < *before
                }
        })
        .map(|(kind, _, _)| *kind)
}

/// Whether a period of `locale`'s rules lies wholly before noon or wholly
/// after it, and so says am or pm on its own: English's *in the evening*
/// is pm, Japanese's 夜中, from 23:00 to 04:00, neither.
#[must_use]
pub fn period_half(locale: &Locale, period: FlexibleDayPeriod) -> Option<crate::DayPeriod> {
    let (_, from, before) = rules(locale)?.iter().find(|(kind, _, _)| *kind == period)?;
    match (*from, *before) {
        (0, 0) => Some(crate::DayPeriod::Am),
        (720, 720) => Some(crate::DayPeriod::Pm),
        (from, before) if from < before && before <= 720 => Some(crate::DayPeriod::Am),
        (from, before) if from < before && from >= 720 => Some(crate::DayPeriod::Pm),
        _ => None,
    }
}

/// What `locale` calls a period in a date's format context, from the first
/// locale in its chain that names it. `Abbreviated`, `Wide` and `Narrow`
/// are CLDR's widths; `Short` asks for the abbreviated name.
#[must_use]
pub fn period_name(
    locale: &Locale,
    period: FlexibleDayPeriod,
    width: NameWidth,
) -> Option<&'static str> {
    let field = match width {
        NameWidth::Wide => 1,
        NameWidth::Narrow => 2,
        NameWidth::Abbreviated | NameWidth::Short => 0,
    };
    locale.fallback().find_map(|candidate| {
        let rendered = candidate.rendered()?;
        let (_, lines) = cldr48::NAMES
            .iter()
            .find(|(tag, _)| *tag == rendered.as_str())?;
        lines
            .split('\n')
            .nth(period.index())?
            .split('|')
            .nth(field)
            .filter(|name| !name.is_empty())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn locale(tag: &str) -> Locale {
        Locale::parse(tag).unwrap()
    }

    /// `dayPeriods.xml`'s English rules: morning from 00:00 before 12:00,
    /// afternoon before 18:00, evening before 21:00, night before 24:00; and
    /// Japanese's 夜中 from 23:00 across midnight before 04:00.
    #[test]
    fn a_minute_falls_in_the_period_its_rules_give() {
        let english = locale("en-GB");
        assert_eq!(
            flexible_period(&english, 3 * 60),
            Some(FlexibleDayPeriod::Morning1)
        );
        assert_eq!(
            flexible_period(&english, 15 * 60),
            Some(FlexibleDayPeriod::Afternoon1)
        );
        assert_eq!(
            flexible_period(&english, 20 * 60 + 59),
            Some(FlexibleDayPeriod::Evening1)
        );
        assert_eq!(
            flexible_period(&english, 21 * 60),
            Some(FlexibleDayPeriod::Night1)
        );
        let japanese = locale("ja");
        assert_eq!(
            flexible_period(&japanese, 23 * 60 + 30),
            Some(FlexibleDayPeriod::Night2)
        );
        assert_eq!(
            flexible_period(&japanese, 2 * 60),
            Some(FlexibleDayPeriod::Night2)
        );
        assert_eq!(
            flexible_period(&japanese, 4 * 60),
            Some(FlexibleDayPeriod::Morning1)
        );
        assert!(has_fixed_period(&english, FlexibleDayPeriod::Noon));
        assert!(!has_fixed_period(&locale("de"), FlexibleDayPeriod::Noon));
        assert!(has_fixed_period(&locale("de"), FlexibleDayPeriod::Midnight));
    }

    /// `dayPeriods.xml` keys its rules by language: Traditional Chinese
    /// takes `zh`'s, midnight at 00:00, `night1` (凌晨) from 00:00,
    /// `morning1` (清晨) from 05:00 and `evening1` (晚上) from 19:00, and `yue-Hans` takes `yue`'s, although neither
    /// reaches `zh` or `yue` through `parentLocales`. `hi-Latn` has rules
    /// of its own, `hi_Latn` in the file: `morning1` from 04:00 and
    /// `night1` from 20:00.
    #[test]
    fn the_rules_are_found_by_truncating_the_tag() {
        for tag in ["zh-Hant", "zh-TW", "zh-Hant-HK", "zh-Hant-MO"] {
            let chinese = locale(tag);
            assert!(
                has_fixed_period(&chinese, FlexibleDayPeriod::Midnight),
                "{tag}"
            );
            assert_eq!(
                flexible_period(&chinese, 6 * 60),
                Some(FlexibleDayPeriod::Morning1),
                "{tag}"
            );
            assert_eq!(
                flexible_period(&chinese, 20 * 60),
                Some(FlexibleDayPeriod::Evening1),
                "{tag}"
            );
            assert_eq!(
                flexible_period(&chinese, 3 * 60),
                Some(FlexibleDayPeriod::Night1),
                "{tag}"
            );
        }
        assert_eq!(
            period_name(
                &locale("zh-Hant"),
                FlexibleDayPeriod::Morning1,
                NameWidth::Wide
            ),
            Some("清晨")
        );
        assert_eq!(
            period_name(
                &locale("zh-TW"),
                FlexibleDayPeriod::Midnight,
                NameWidth::Wide
            ),
            Some("午夜")
        );
        assert_eq!(
            flexible_period(&locale("yue-Hans"), 20 * 60),
            flexible_period(&locale("yue"), 20 * 60)
        );
        assert!(flexible_period(&locale("yue-Hans"), 20 * 60).is_some());
        let hinglish = locale("hi-Latn");
        assert_eq!(
            flexible_period(&hinglish, 3 * 60 + 59),
            Some(FlexibleDayPeriod::Night1)
        );
        assert_eq!(
            flexible_period(&hinglish, 4 * 60),
            Some(FlexibleDayPeriod::Morning1)
        );
        assert_eq!(flexible_period(&locale("und"), 600), None);
    }

    #[test]
    fn a_period_has_the_names_the_locales_file_gives() {
        let english = locale("en");
        assert_eq!(
            period_name(&english, FlexibleDayPeriod::Night1, NameWidth::Wide),
            Some("at night")
        );
        assert_eq!(
            period_name(&english, FlexibleDayPeriod::Noon, NameWidth::Abbreviated),
            Some("noon")
        );
        assert_eq!(
            period_name(&locale("ja"), FlexibleDayPeriod::Night2, NameWidth::Wide),
            Some("夜中")
        );
    }
}
