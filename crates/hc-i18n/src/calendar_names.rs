//! What a locale calls a calendar CLDR has no name for, where a source in
//! that language names it.
//!
//! CLDR names the calendars of its own `-u-ca-` keys, and [`crate::names`]
//! reads those from each locale's data. The registry carries many more,
//! and a name for one of them is only here when a source written in the
//! language names that calendar; every other one keeps its English name,
//! the calendar's own. The Tibetan almanac's calendars are the first:
//!
//! - `tibetan`, the Phugpa reckoning, "浦派为藏地官方历书所采用", the one the
//!   official almanacs use, bears the name of the Tibetan calendar itself:
//!   藏历 and 藏曆 (Chinese Wikipedia, 藏曆, revision 91955698, read
//!   2026-09-29, `wikipedia-zh-tibetan-calendar`, in its zh-Hans and zh-Hant
//!   variants), チベット暦 (Japanese Wikipedia, revision 90277485,
//!   `wikipedia-ja-tibetan-calendar`) and བོད་ཀྱི་ལོ་ཐོ (Tibetan Wikipedia's
//!   title, revision 133632, `wikipedia-bo-tibetan-calendar`).
//! - `tibetan-tsurphu` is the reckoning of the Tsurphu school, which the
//!   Chinese article names 楚尔派 (楚爾派), "མཚུར་ལུགས་, mtshur lugs".
//! - `mongolian`, the Tögs buyant reckoning of 1747, is the traditional
//!   Mongolian calendar: モンゴル暦 (Japanese Wikipedia, revision 87251925,
//!   `wikipedia-ja-mongolian-calendar`), 蒙古历 and 蒙古曆 (Chinese
//!   Wikipedia, 蒙古历, revision 87459686, `wikipedia-zh-mongolian-calendar`),
//!   and Билгийн тоолол, the "traditional lunar reckoning" beside the
//!   Gregorian Аргын тоолол (Mongolian Wikipedia, Монгол цаг тоолол,
//!   revision 817498, `wikipedia-mn-mongolian-calendar`, and the Japanese
//!   article).
//!
//! Not named, for want of a source: the Bhutanese reckonings, both Lochen
//! variants and the Tsurphu karaṇa Sun in any of these languages; the
//! Mongolian calendar in Tibetan; and the Tibetan ones in Mongolian.

use hc_calendar::CalendarId;

use crate::names::CalendarDisplayName;

/// The names, each with the tag of the locale data it belongs to.
pub const SOURCED_CALENDAR_NAMES: &[(&str, CalendarDisplayName)] = &[
    ("bo", CalendarDisplayName::new("tibetan", "བོད་ཀྱི་ལོ་ཐོ")),
    ("bo", CalendarDisplayName::new("tibetan-tsurphu", "མཚུར་ལུགས")),
    ("ja", CalendarDisplayName::new("tibetan", "チベット暦")),
    ("ja", CalendarDisplayName::new("mongolian", "モンゴル暦")),
    (
        "mn",
        CalendarDisplayName::new("mongolian", "Билгийн тоолол"),
    ),
    ("zh-Hans", CalendarDisplayName::new("tibetan", "藏历")),
    (
        "zh-Hans",
        CalendarDisplayName::new("tibetan-tsurphu", "楚尔派"),
    ),
    ("zh-Hans", CalendarDisplayName::new("mongolian", "蒙古历")),
    ("zh-Hant", CalendarDisplayName::new("tibetan", "藏曆")),
    (
        "zh-Hant",
        CalendarDisplayName::new("tibetan-tsurphu", "楚爾派"),
    ),
    ("zh-Hant", CalendarDisplayName::new("mongolian", "蒙古曆")),
];

/// The name the locale data tagged `tag` has for `calendar` here, if any.
#[must_use]
pub fn sourced_calendar_name(tag: &str, calendar: CalendarId) -> Option<&'static str> {
    SOURCED_CALENDAR_NAMES
        .iter()
        .find(|(entry_tag, entry)| *entry_tag == tag && entry.calendar == calendar)
        .map(|(_, entry)| entry.name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Locale;
    use crate::names::calendar_display_name_with_tag;

    fn named(tag: &str, calendar: &'static str) -> Option<(&'static str, &'static str)> {
        calendar_display_name_with_tag(&Locale::parse(tag).expect("a tag"), CalendarId(calendar))
    }

    /// The names are found through each locale's fallback, a region's
    /// tag answering with its language's data, and a calendar CLDR names
    /// keeps CLDR's name.
    #[test]
    fn the_tibetan_almanacs_calendars_are_named_where_a_source_names_them() {
        assert_eq!(named("zh-Hans", "tibetan"), Some(("藏历", "zh-Hans")));
        assert_eq!(
            named("zh-TW", "tibetan-tsurphu"),
            Some(("楚爾派", "zh-Hant"))
        );
        assert_eq!(named("ja-JP", "mongolian"), Some(("モンゴル暦", "ja")));
        assert_eq!(named("mn", "mongolian"), Some(("Билгийн тоолол", "mn")));
        assert_eq!(named("bo", "tibetan"), Some(("བོད་ཀྱི་ལོ་ཐོ", "bo")));
        assert_eq!(named("ja", "japanese"), Some(("和暦", "ja")));
        // No source names the Bhutanese reckoning in Japanese.
        assert_ne!(
            named("ja", "tibetan-bhutan").map(|(_, tag)| tag),
            Some("ja")
        );
        let tags: Vec<&str> = crate::data::LOCALES.iter().map(|data| data.tag).collect();
        for (tag, entry) in SOURCED_CALENDAR_NAMES {
            assert!(tags.contains(tag), "{tag}");
            let data = crate::data::LOCALES
                .iter()
                .find(|data| data.tag == *tag)
                .expect("carried");
            // CLDR's own name would come first, so none is shadowed.
            assert!(
                data.calendar_names
                    .iter()
                    .all(|cldr| cldr.calendar != entry.calendar),
                "{tag} {}",
                entry.calendar.0
            );
        }
    }
}
