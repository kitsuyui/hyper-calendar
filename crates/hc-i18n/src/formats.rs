//! The standard date, time and date-time formats of a locale, as data:
//! Unicode CLDR 48's `dateFormats`, `timeFormats` and `dateTimeFormats` in
//! their four lengths, and a few `availableFormats` items, for every carried
//! locale and every calendar CLDR gives them.
//!
//! UTS #35 Part 4, *Dates*, version 48.2, "Element dateFormats": the
//! standard patterns "are each normally provided in four types: full
//! (usually with weekday name), long (with wide month name), medium, and
//! short (usually with numeric month)". A date-time format joins one of
//! each: "{1} {0}", where "{0} is replaced by the time format, and {1} is
//! replaced by the date format", and the date's length chooses which
//! date-time format to use. The non-Gregorian calendars' date formats are
//! the `generic` calendar's unless a file states their own: "The 'generic'
//! calendar formats are intended to provide a consistent set of default
//! formats for non-Gregorian calendars in the locale" [uts35-dates-48].
//!
//! `scripts/formats-cldr.py` generates the table from each carried locale's
//! file, resolved through `root.xml`'s aliases [cldr48-date-formats];
//! `hc-format`'s `strftime` engine writes `%c`, `%x`, `%X` and `%r` from
//! it, and `docs/systems/date-patterns.md` says which length each takes.

use hc_calendar::CalendarId;

use crate::locale::Locale;

mod cldr48;

/// The four lengths CLDR gives a standard date or time format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormatLength {
    /// *Monday, September 21, 2026*; *2:30:05 PM Japan Standard Time*.
    Full,
    /// *September 21, 2026*; *2:30:05 PM JST*.
    Long,
    /// *Sep 21, 2026*; *2:30:05 PM*.
    Medium,
    /// *9/21/26*; *2:30 PM*.
    Short,
}

impl FormatLength {
    /// Every length, longest first, as CLDR orders them.
    pub const ALL: [Self; 4] = [Self::Full, Self::Long, Self::Medium, Self::Short];

    /// CLDR's `type`: `full`, `long`, `medium` or `short`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Long => "long",
            Self::Medium => "medium",
            Self::Short => "short",
        }
    }

    /// Parse CLDR's `type`, in either case.
    #[must_use]
    pub fn from_keyword(keyword: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|length| length.as_str().eq_ignore_ascii_case(keyword))
    }

    const fn index(self) -> usize {
        match self {
            Self::Full => 0,
            Self::Long => 1,
            Self::Medium => 2,
            Self::Short => 3,
        }
    }
}

/// The `availableFormats` items the table carries, by their skeleton.
pub const AVAILABLE_FORMATS: [&str; 6] = ["hms", "Hms", "Gy", "d", "yMMMMd", "yMMMd"];

/// The CLDR calendar types the table has rows for.
pub const CALENDARS: [&str; 14] = [
    "gregorian",
    "generic",
    "buddhist",
    "chinese",
    "coptic",
    "dangi",
    "ethiopic",
    "hebrew",
    "indian",
    "islamic",
    "iso8601",
    "japanese",
    "persian",
    "roc",
];

/// The CLDR calendar type a registry identifier's formats are read under:
/// the families CLDR names, and `generic` for every other calendar, as
/// `root.xml` gives the calendars it has no formats for the `generic` ones.
#[must_use]
pub fn calendar_key(id: CalendarId) -> &'static str {
    match id.0 {
        "gregory" | "julian" | "revised-julian" | "iso8601-week" | "iso8601-ordinal" => "gregorian",
        "iso8601" => "iso8601",
        "buddhist" => "buddhist",
        "roc" => "roc",
        "coptic" => "coptic",
        "ethiopic" => "ethiopic",
        "indian" => "indian",
        "chinese" | "vietnamese" => "chinese",
        _ if crate::data::JAPANESE_CALENDARS.contains(&id) => "japanese",
        _ if crate::data::HEBREW_CALENDARS.contains(&id) => "hebrew",
        _ if crate::data::ISLAMIC_CALENDARS.contains(&id) => "islamic",
        _ if crate::data::SOLAR_HIJRI_CALENDARS.contains(&id)
            || crate::data::PERSIAN_CALENDARS.contains(&id) =>
        {
            "persian"
        }
        _ if crate::data::DANGI_CALENDARS.contains(&id) => "dangi",
        _ if crate::data::CHINESE_FAMILY_CALENDARS.contains(&id) => "chinese",
        _ => "generic",
    }
}

/// The CLDR calendar type a locale's `-u-ca-` key asks for, `gregorian`
/// with none: the key's own type where the table has it, else `generic`.
#[must_use]
pub fn calendar_key_for_locale(locale: &Locale) -> &'static str {
    match locale.calendar() {
        None => "gregorian",
        Some(key) => CALENDARS
            .iter()
            .copied()
            .find(|candidate| *candidate == key)
            .or_else(|| calendar_key_of_cldr_type(key))
            .unwrap_or("generic"),
    }
}

/// The table's type for a CLDR key it has no row of its own for: the Hijri
/// variants' `islamic`, `gregory`'s `gregorian`, `ethioaa`'s `ethiopic`.
fn calendar_key_of_cldr_type(key: &str) -> Option<&'static str> {
    match key {
        "gregory" => Some("gregorian"),
        "ethioaa" | "ethiopic-amete-alem" => Some("ethiopic"),
        other if other.starts_with("islamic") => Some("islamic"),
        _ => None,
    }
}

/// The base a calendar's row leaves its unchanged fields to: `gregorian`
/// for `iso8601` and `generic`, `generic` for every other calendar.
const fn base_of(calendar: &str) -> Option<&'static str> {
    match calendar.as_bytes() {
        b"gregorian" => None,
        b"iso8601" | b"generic" => Some("gregorian"),
        _ => Some("generic"),
    }
}

/// The field at `index` for a locale and a calendar type: the calendar's
/// rows along the locale's chain and root, then its base's, then the
/// Gregorian's.
fn field(locale: &Locale, calendar: &str, index: usize) -> Option<&'static str> {
    let mut step = Some(calendar);
    while let Some(current) = step {
        for candidate in locale.fallback() {
            let Some(rendered) = candidate.rendered() else {
                continue;
            };
            if let Some(value) = row_field(rendered.as_str(), current, index) {
                return Some(value);
            }
        }
        if let Some(value) = row_field("und", current, index) {
            return Some(value);
        }
        step = base_of(current);
    }
    None
}

fn row_field(tag: &str, calendar: &str, index: usize) -> Option<&'static str> {
    let position = cldr48::FORMATS
        .binary_search_by(|(row_tag, row_calendar, _)| {
            row_tag.cmp(&tag).then_with(|| row_calendar.cmp(&calendar))
        })
        .ok()?;
    let (_, _, fields) = cldr48::FORMATS[position];
    fields
        .split('|')
        .nth(index)
        .filter(|value| !value.is_empty())
}

/// The locale's standard date format of a length, for a calendar type
/// ([`calendar_key`]): `dd.MM.y` for `de`, `y/MM/dd` for `ja`, both medium.
#[must_use]
pub fn date_pattern(locale: &Locale, calendar: &str, length: FormatLength) -> Option<&'static str> {
    field(locale, calendar, length.index())
}

/// The locale's standard time format of a length: `HH:mm:ss` for `de`,
/// `h:mm:ss a` for `en`, both medium.
#[must_use]
pub fn time_pattern(locale: &Locale, calendar: &str, length: FormatLength) -> Option<&'static str> {
    field(locale, calendar, 4 + length.index())
}

/// The locale's standard date-time format of a length, with `{1}` for the
/// date and `{0}` for the time: `{1}, {0}` for `de`, `{1} {0}` for `ja`.
#[must_use]
pub fn date_time_pattern(
    locale: &Locale,
    calendar: &str,
    length: FormatLength,
) -> Option<&'static str> {
    field(locale, calendar, 8 + length.index())
}

/// One of the `availableFormats` items of [`AVAILABLE_FORMATS`], by its
/// skeleton: `hms` is `h:mm:ss a` in `en` and `aK:mm:ss` in `ja`. `None`
/// for a skeleton the table does not carry.
#[must_use]
pub fn available_format(locale: &Locale, calendar: &str, skeleton: &str) -> Option<&'static str> {
    let index = AVAILABLE_FORMATS
        .iter()
        .position(|candidate| *candidate == skeleton)?;
    field(locale, calendar, 12 + index)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn locale(tag: &str) -> Locale {
        Locale::parse(tag).unwrap()
    }

    /// CLDR 48 `de.xml`, `ja.xml`, `fr.xml`, `en.xml`, `calendar
    /// type="gregorian"`: the four date lengths and the medium time.
    #[test]
    fn the_standard_formats_are_the_files() {
        let de = locale("de");
        assert_eq!(
            FormatLength::ALL.map(|length| date_pattern(&de, "gregorian", length).unwrap()),
            ["EEEE, d. MMMM y", "d. MMMM y", "dd.MM.y", "dd.MM.yy"]
        );
        assert_eq!(
            time_pattern(&de, "gregorian", FormatLength::Medium),
            Some("HH:mm:ss")
        );
        assert_eq!(
            date_time_pattern(&de, "gregorian", FormatLength::Medium),
            Some("{1}, {0}")
        );
        let ja = locale("ja-JP");
        assert_eq!(
            date_pattern(&ja, "gregorian", FormatLength::Medium),
            Some("y/MM/dd")
        );
        assert_eq!(
            time_pattern(&ja, "gregorian", FormatLength::Full),
            Some("H時mm分ss秒 zzzz")
        );
        assert_eq!(
            date_time_pattern(&ja, "gregorian", FormatLength::Medium),
            Some("{1} {0}")
        );
        let fr = locale("fr");
        assert_eq!(
            date_pattern(&fr, "gregorian", FormatLength::Medium),
            Some("d MMM y")
        );
        let en = locale("en");
        assert_eq!(
            date_pattern(&en, "gregorian", FormatLength::Short),
            Some("M/d/yy")
        );
        assert_eq!(
            time_pattern(&en, "gregorian", FormatLength::Medium),
            Some("h:mm:ss\u{202f}a")
        );
        assert_eq!(
            date_time_pattern(&en, "gregorian", FormatLength::Long),
            Some("{1}, {0}")
        );
    }

    /// `root.xml` aliases the Persian calendar's `dateFormats` to the
    /// `generic` calendar's and its `timeFormats` to the Gregorian's; the
    /// Japanese calendar in `ja.xml` has formats of its own.
    #[test]
    fn a_non_gregorian_calendar_takes_the_generic_formats_unless_the_file_states_its_own() {
        let de = locale("de");
        assert_eq!(
            date_pattern(&de, "persian", FormatLength::Medium),
            Some("dd.MM.y G")
        );
        assert_eq!(
            date_pattern(&de, "generic", FormatLength::Long),
            Some("d. MMMM y G")
        );
        assert_eq!(
            time_pattern(&de, "persian", FormatLength::Short),
            Some("HH:mm")
        );
        let ja = locale("ja");
        assert_eq!(
            date_pattern(&ja, "japanese", FormatLength::Medium),
            Some("Gy年M月d日")
        );
        assert_eq!(
            date_pattern(&ja, "persian", FormatLength::Medium),
            Some("GGGGGy/MM/dd")
        );
        assert_eq!(
            date_pattern(&ja, "chinese", FormatLength::Long),
            Some("U年MMMd日")
        );
        let en = locale("en");
        assert_eq!(
            date_pattern(&en, "chinese", FormatLength::Medium),
            Some("MMM d, r")
        );
    }

    /// `hms` and `Gy` from `en.xml`, `ja.xml` and `de.xml`; a skeleton the
    /// table does not carry is `None`.
    #[test]
    fn the_available_formats_are_carried_by_skeleton() {
        assert_eq!(
            available_format(&locale("en"), "gregorian", "hms"),
            Some("h:mm:ss\u{202f}a")
        );
        assert_eq!(
            available_format(&locale("ja"), "gregorian", "hms"),
            Some("aK:mm:ss")
        );
        assert_eq!(
            available_format(&locale("ja"), "gregorian", "Gy"),
            Some("Gy年")
        );
        assert_eq!(
            available_format(&locale("de"), "gregorian", "Hms"),
            Some("HH:mm:ss")
        );
        assert_eq!(available_format(&locale("de"), "gregorian", "yMMMEd"), None);
    }

    /// A regional entry inherits what its files do not change: `en-GB`'s
    /// chain reaches `en-001` and `en`; a tag with no entry reaches root,
    /// whose medium date is `y MMM d`.
    #[test]
    fn the_lookup_walks_the_chain_and_ends_at_root() {
        assert_eq!(
            date_pattern(&locale("en-GB"), "gregorian", FormatLength::Medium),
            Some("d MMM y")
        );
        assert_eq!(
            date_pattern(&locale("en-AU"), "gregorian", FormatLength::Short),
            Some("dd/MM/y")
        );
        assert_eq!(
            time_pattern(&locale("en-GB"), "gregorian", FormatLength::Medium),
            Some("HH:mm:ss")
        );
        assert_eq!(
            date_pattern(&locale("xx-YY"), "gregorian", FormatLength::Medium),
            Some("y MMM d")
        );
        assert_eq!(
            date_time_pattern(&locale("xx"), "persian", FormatLength::Short),
            Some("{1} {0}")
        );
    }

    #[test]
    fn registry_identifiers_map_to_cldrs_calendar_types() {
        assert_eq!(calendar_key(CalendarId("gregory")), "gregorian");
        assert_eq!(calendar_key(CalendarId("julian")), "gregorian");
        assert_eq!(calendar_key(CalendarId("iso8601")), "iso8601");
        assert_eq!(calendar_key(CalendarId("islamic-umalqura")), "islamic");
        assert_eq!(calendar_key(CalendarId("persian-afghan")), "persian");
        assert_eq!(calendar_key(CalendarId("hebrew")), "hebrew");
        assert_eq!(calendar_key(CalendarId("japanese")), "japanese");
        assert_eq!(calendar_key(CalendarId("dangi")), "dangi");
        assert_eq!(calendar_key(CalendarId("maya-haab")), "generic");
        assert_eq!(calendar_key_for_locale(&locale("en")), "gregorian");
        assert_eq!(
            calendar_key_for_locale(&locale("th-u-ca-buddhist")),
            "buddhist"
        );
        assert_eq!(
            calendar_key_for_locale(&locale("ar-u-ca-islamic-umalqura")),
            "islamic"
        );
        assert_eq!(
            calendar_key_for_locale(&locale("en-u-ca-gregory")),
            "gregorian"
        );
        assert_eq!(calendar_key_for_locale(&locale("en-u-ca-coptic")), "coptic");
    }

    #[test]
    fn the_table_is_sorted_for_its_search() {
        for pair in cldr48::FORMATS.windows(2) {
            assert!(
                (pair[0].0, pair[0].1) < (pair[1].0, pair[1].1),
                "{:?} !< {:?}",
                (pair[0].0, pair[0].1),
                (pair[1].0, pair[1].1)
            );
        }
        for (_, calendar, fields) in cldr48::FORMATS {
            assert!(CALENDARS.contains(calendar), "{calendar}");
            assert!(fields.split('|').count() <= 18, "{fields}");
        }
        for length in FormatLength::ALL {
            assert_eq!(FormatLength::from_keyword(length.as_str()), Some(length));
        }
    }
}
