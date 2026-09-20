//! Finding the data for a locale.
//!
//! Two fallbacks run here and they are independent of each other:
//!
//! 1. **Locale fallback** — [`hc_i18n::Locale::fallback`], the CLDR
//!    truncation chain `pt-BR` → `pt` → root. The first entry that actually
//!    carries the field wins, so a regional entry can state one thing and
//!    inherit the rest.
//! 2. **Style fallback** — narrow → short → long, within a single entry
//!    before moving up the locale chain. A language that abbreviates
//!    nothing states its long forms once.
//!
//! Doing the style fallback first is deliberate: `pt-BR`'s long form is a
//! better answer for a narrow request than `pt`'s narrow form would be,
//! because it is at least the right locale.

use hc_i18n::Locale;

use crate::data::{LOCALES, ROOT};
use crate::pattern::{
    ApproximatePatterns, ListForms, LocaleData, RelativeStyle, UnitPatterns, UnitStrings,
    WeekdayPatterns,
};
use crate::unit::TimeUnit;

/// Walk the locale fallback chain and return the first entry that yields a
/// value, with the root entry as the floor.
fn resolve<T, F>(locale: &Locale, pick: F) -> Option<T>
where
    F: Fn(&'static LocaleData) -> Option<T>,
{
    for candidate in locale.fallback() {
        for data in LOCALES {
            if candidate.matches_tag(data.tag)
                && let Some(value) = pick(data)
            {
                return Some(value);
            }
        }
    }
    pick(&ROOT)
}

/// The entry a locale resolves to, ignoring which fields it carries.
///
/// Never fails: the root entry answers for everything else.
#[must_use]
pub fn locale_data(locale: &Locale) -> &'static LocaleData {
    resolve(locale, Some).unwrap_or(&ROOT)
}

/// The patterns for one unit in one style.
///
/// Returns `None` only if nothing in the chain — root included — states the
/// unit, which for the data shipped here cannot happen.
#[must_use]
pub fn unit_patterns(
    locale: &Locale,
    unit: TimeUnit,
    style: RelativeStyle,
) -> Option<&'static UnitPatterns> {
    resolve(locale, |data| {
        let mut current = Some(style);
        while let Some(width) = current {
            let patterns = width.within(data).get(unit);
            if !patterns.is_empty() {
                return Some(patterns);
            }
            current = width.wider();
        }
        None
    })
}

/// The special word a language has for an integer offset in a unit, such as
/// *yesterday* for `-1` days.
///
/// Style fallback applies here too, but a style that states a numeric
/// pattern and no special word does **not** inherit one from a wider style:
/// if `narrow` says *{0}d ago* and nothing else, the narrow style genuinely
/// has no word for *yesterday* and the caller should get the numeric form.
#[must_use]
pub fn special_word(
    locale: &Locale,
    unit: TimeUnit,
    style: RelativeStyle,
    offset: i64,
) -> Option<&'static str> {
    let patterns = unit_patterns(locale, unit, style)?;
    let word = patterns.special(offset);
    if word.is_empty() { None } else { Some(word) }
}

/// The list patterns a style joins duration components with.
///
/// The mapping is fixed: the long style takes CLDR's `standard` list, which
/// is the one carrying the conjunction, the short style takes `unit`, which
/// is comma-joined, and the narrow style takes `unit-narrow`.
#[must_use]
pub fn list_forms(locale: &Locale, style: RelativeStyle) -> ListForms {
    let pick = |data: &'static LocaleData| -> Option<ListForms> {
        let mut current = Some(style);
        while let Some(width) = current {
            let forms = match width {
                RelativeStyle::Long => data.list.standard,
                RelativeStyle::Short => data.list.unit,
                RelativeStyle::Narrow => data.list.narrow,
            };
            if !forms.is_empty() {
                return Some(forms);
            }
            current = width.wider();
        }
        None
    };
    resolve(locale, pick).unwrap_or(ROOT.list.standard)
}

/// The approximation hedges of a locale.
#[must_use]
pub fn approximate_patterns(locale: &Locale) -> ApproximatePatterns {
    resolve(locale, |data| {
        if data.approximate.about.is_empty() {
            None
        } else {
            Some(data.approximate)
        }
    })
    .unwrap_or(ROOT.approximate)
}

/// How a locale points at a named weekday.
#[must_use]
pub fn weekday_patterns(locale: &Locale) -> WeekdayPatterns {
    resolve(locale, |data| {
        if data.weekday.previous.is_empty() {
            None
        } else {
            Some(data.weekday)
        }
    })
    .unwrap_or(ROOT.weekday)
}

/// The compact-form suffixes of a locale.
#[must_use]
pub fn compact_units(locale: &Locale) -> UnitStrings {
    resolve(locale, |data| {
        if data.compact.hour.is_empty() {
            None
        } else {
            Some(data.compact)
        }
    })
    .unwrap_or(ROOT.compact)
}

/// The indefinite singular of each unit, empty for a language with no
/// indefinite article.
#[must_use]
pub fn indefinite_units(locale: &Locale) -> UnitStrings {
    resolve(locale, |data| {
        if data.indefinite.hour.is_empty() {
            None
        } else {
            Some(data.indefinite)
        }
    })
    .unwrap_or(ROOT.indefinite)
}

/// The decimal separator a locale writes a half unit with.
#[must_use]
pub fn decimal_separator(locale: &Locale) -> &'static str {
    resolve(locale, |data| {
        if data.decimal_separator.is_empty() {
            None
        } else {
            Some(data.decimal_separator)
        }
    })
    .unwrap_or(ROOT.decimal_separator)
}

/// How a locale joins a day phrase to a time of day.
#[must_use]
pub fn at_pattern(locale: &Locale) -> &'static str {
    resolve(locale, |data| {
        if data.at_pattern.is_empty() {
            None
        } else {
            Some(data.at_pattern)
        }
    })
    .unwrap_or(ROOT.at_pattern)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::RelativeStyle;

    fn locale(tag: &str) -> Locale {
        tag.parse().expect("well-formed tag")
    }

    #[test]
    fn a_region_inherits_everything_its_language_states() {
        assert_eq!(locale_data(&locale("en-GB")).tag, "en");
        assert_eq!(locale_data(&locale("pt-BR")).tag, "pt");
        assert_eq!(locale_data(&locale("de-AT-u-ca-gregory")).tag, "de");
    }

    #[test]
    fn simplified_chinese_reaches_the_zh_entry_by_truncation() {
        assert_eq!(locale_data(&locale("zh")).tag, "zh");
        assert_eq!(locale_data(&locale("zh-Hans")).tag, "zh");
        assert_eq!(locale_data(&locale("zh-Hans-CN")).tag, "zh");
        assert_eq!(locale_data(&locale("zh-Hant")).tag, "zh-Hant");
        assert_eq!(locale_data(&locale("zh-Hant-TW")).tag, "zh-Hant");
    }

    #[test]
    fn an_unknown_language_lands_on_the_root() {
        assert_eq!(locale_data(&locale("xx")).tag, "und");
        assert_eq!(locale_data(&Locale::ROOT).tag, "und");
    }

    #[test]
    fn a_style_a_locale_does_not_distinguish_falls_back_to_a_wider_one() {
        // German abbreviates months but not days, and states no narrow
        // forms at all, so every style has to find an answer.
        let german = locale("de");
        let day_long = unit_patterns(&german, TimeUnit::Day, RelativeStyle::Long).expect("stated");
        let day_narrow =
            unit_patterns(&german, TimeUnit::Day, RelativeStyle::Narrow).expect("stated");
        assert_eq!(day_long.past.other, day_narrow.past.other);
        let month_short =
            unit_patterns(&german, TimeUnit::Month, RelativeStyle::Short).expect("stated");
        let month_narrow =
            unit_patterns(&german, TimeUnit::Month, RelativeStyle::Narrow).expect("stated");
        assert_eq!(month_short.past.other, "vor {0} Mon.");
        assert_eq!(month_narrow.past.other, "vor {0} Mon.");
    }

    #[test]
    fn every_shipped_locale_answers_for_every_unit_and_style() {
        for data in crate::data::LOCALES {
            let tag = locale(data.tag);
            for style in RelativeStyle::ALL {
                for unit in TimeUnit::ALL {
                    assert!(
                        unit_patterns(&tag, unit, style).is_some(),
                        "{} has no {style:?} {unit}",
                        data.tag
                    );
                }
            }
        }
    }

    #[test]
    fn a_special_word_is_reported_only_when_the_language_has_one() {
        let english = locale("en");
        assert_eq!(
            special_word(&english, TimeUnit::Day, RelativeStyle::Long, -1),
            Some("yesterday")
        );
        assert_eq!(
            special_word(&english, TimeUnit::Day, RelativeStyle::Long, -5),
            None
        );
        // The narrow style genuinely has no word for the day before
        // yesterday, and must not borrow the long one.
        assert_eq!(
            special_word(&english, TimeUnit::Day, RelativeStyle::Narrow, -2),
            None
        );
        assert_eq!(
            special_word(&english, TimeUnit::Day, RelativeStyle::Narrow, -1),
            Some("yesterday")
        );
    }

    #[test]
    fn the_auxiliary_tables_fall_back_without_borrowing_from_a_neighbour() {
        // Japanese has no indefinite article, so it must get nothing rather
        // than English's.
        assert_eq!(indefinite_units(&locale("ja")).year, "");
        assert_eq!(indefinite_units(&locale("en-AU")).year, "a year");
        assert_eq!(decimal_separator(&locale("de-CH")), ",");
        assert_eq!(decimal_separator(&locale("xx")), ".");
        assert_eq!(at_pattern(&locale("en-GB")), "{0} at {1}");
        assert_eq!(weekday_patterns(&locale("fr-CA")).previous, "{0} dernier");
        // Arabic states no compact suffixes, so it falls to the root's.
        assert_eq!(compact_units(&locale("ar")).hour, "h");
        assert_eq!(compact_units(&locale("ja")).hour, "時間");
    }

    #[test]
    fn the_list_style_mapping_is_fixed() {
        let english = locale("en");
        assert_eq!(list_forms(&english, RelativeStyle::Long).end, "{0} and {1}");
        assert_eq!(list_forms(&english, RelativeStyle::Short).end, "{0}, {1}");
        assert_eq!(list_forms(&english, RelativeStyle::Narrow).end, "{0} {1}");
    }
}
