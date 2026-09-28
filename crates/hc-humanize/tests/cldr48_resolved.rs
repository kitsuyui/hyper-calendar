//! Every carried locale's phrases against CLDR's own resolution of them.
//!
//! `data/cldr48_resolved.tsv` holds, for each locale [`LOCALES`] carries,
//! a sample of the values CLDR 48 resolves: the past, future and duration
//! patterns of the hour and the day in each plural category of the
//! language, and the words for yesterday, today and the like, in the long,
//! short and narrow styles. `scripts/humanize-cldr-sample.py` reads them
//! from the `cldr-json` 48.0.0 packages (`cldr-dates-full`
//! `main/<locale>/dateFields.json` and `cldr-units-full`
//! `main/<locale>/units.json`, `cldr-json-48`, read 2026-09-28), which
//! CLDR's own tools resolve from the XML: every parent, alias and
//! inheritance marker (`↑↑↑`) is already followed there. A category the
//! file has no key for is the style's `other`, as a reader of those files
//! takes it; a word it has no key for is none.
//!
//! This is the independent check on the data `scripts/humanize-cldr.py`
//! generates from the XML with a resolver of its own: a value that follows
//! CLDR's inheritance wrongly — a style left to the long one where the file
//! states its own, or a count filled from `other` where a wider style
//! states it — differs from CLDR's here. Every locale's data is generated,
//! so a sampled value may differ from CLDR's only where
//! `src/data/cldr48_overrides.tsv`, the one list of documented overrides,
//! says so and gives the value; an override the sample covers must differ
//! from CLDR's, and every override, sampled or not, must be the value the
//! crate carries.
//!
//! [`LOCALES`]: hc_humanize::data::LOCALES
#![expect(
    clippy::expect_used,
    reason = "a sample line or an override that does not parse is a failed test, \
              and the message says which"
)]

use hc_humanize::data::LOCALES;
use hc_humanize::lookup;
use hc_humanize::{RelativeStyle, TimeUnit};
use hc_i18n::{Locale, PluralCategory};

const SAMPLE: &str = include_str!("data/cldr48_resolved.tsv");
const OVERRIDES: &str = include_str!("../src/data/cldr48_overrides.tsv");

fn category(name: &str) -> PluralCategory {
    match name {
        "zero" => PluralCategory::Zero,
        "one" => PluralCategory::One,
        "two" => PluralCategory::Two,
        "few" => PluralCategory::Few,
        "many" => PluralCategory::Many,
        "other" => PluralCategory::Other,
        _ => panic!("unknown category {name}"),
    }
}

fn style(name: &str) -> RelativeStyle {
    match name {
        "long" => RelativeStyle::Long,
        "short" => RelativeStyle::Short,
        "narrow" => RelativeStyle::Narrow,
        _ => panic!("unknown style {name}"),
    }
}

fn unit(name: &str) -> TimeUnit {
    TimeUnit::ALL
        .into_iter()
        .find(|unit| unit.to_string() == name)
        .unwrap_or_else(|| panic!("unknown unit {name}"))
}

/// The override list: (tag, key, value).
fn overrides() -> Vec<(&'static str, &'static str, &'static str)> {
    OVERRIDES
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let cells: Vec<&str> = line.split('\t').collect();
            assert_eq!(cells.len(), 4, "{line:?}: four cells");
            assert!(!cells[3].is_empty(), "{line:?}: no reason");
            let value = if cells[2] == "-" { "" } else { cells[2] };
            (cells[0], cells[1], value)
        })
        .collect()
}

/// The value the crate carries for a key of the override list, as the
/// lookup a formatter makes finds it: style fallback included.
fn carried(tag: &str, key: &str) -> String {
    let locale: Locale = tag.parse().expect("well-formed tag");
    let parts: Vec<&str> = key.split('.').collect();
    match parts.as_slice() {
        ["decimal"] => lookup::decimal_separator(&locale).to_string(),
        ["at"] => lookup::at_pattern(&locale).to_string(),
        ["list", kind, part] => {
            let width = match *kind {
                "standard" => RelativeStyle::Long,
                "unit" => RelativeStyle::Short,
                "narrow" => RelativeStyle::Narrow,
                _ => panic!("unknown list {kind}"),
            };
            let forms = lookup::list_forms(&locale, width);
            match *part {
                "2" => forms.two,
                "start" => forms.start,
                "middle" => forms.middle,
                "end" => forms.end,
                _ => panic!("unknown list part {part}"),
            }
            .to_string()
        }
        [style_name, unit_name, "relative", offset] => {
            let offset: i64 = offset.parse().expect("an offset");
            lookup::special_word(&locale, unit(unit_name), style(style_name), offset)
                .unwrap_or("")
                .to_string()
        }
        [style_name, unit_name, kind, name] => {
            let patterns = lookup::unit_patterns(&locale, unit(unit_name), style(style_name))
                .expect("every unit is stated");
            let forms = match *kind {
                "past" => patterns.past,
                "future" => patterns.future,
                "count" => patterns.count,
                _ => panic!("unknown kind {kind}"),
            };
            forms.get(category(name)).to_string()
        }
        _ => panic!("unknown key {key}"),
    }
}

#[test]
fn every_locale_resolves_its_phrases_as_cldr_does_but_where_an_override_says() {
    let overrides = overrides();
    let mut failures = Vec::new();
    let mut seen = Vec::new();
    let mut sampled = Vec::new();
    for line in SAMPLE.lines() {
        let mut cells = line.split('\t');
        let (Some(tag), Some(style_name), Some(unit_name), Some(kind)) =
            (cells.next(), cells.next(), cells.next(), cells.next())
        else {
            panic!("short line {line:?}");
        };
        if !seen.contains(&tag) {
            seen.push(tag);
        }
        for cell in cells {
            let (key, expected) = cell.split_once('=').expect("key=value");
            let key = format!("{style_name}.{unit_name}.{kind}.{key}");
            let actual = carried(tag, &key);
            let listed = overrides.iter().find(|row| row.0 == tag && row.1 == key);
            match listed {
                None if actual != expected => {
                    failures.push(format!("{tag} {key}: {actual:?}, CLDR {expected:?}"));
                }
                Some(row) if row.2 == expected => {
                    failures.push(format!("{tag} {key}: overridden, but CLDR's own value"));
                }
                _ => {}
            }
            if listed.is_some() {
                sampled.push((tag, key));
            }
        }
    }
    for (tag, key, value) in &overrides {
        let actual = carried(tag, key);
        if actual != *value {
            failures.push(format!(
                "{tag} {key}: overridden with {value:?}, carries {actual:?}"
            ));
        }
    }
    for data in LOCALES {
        assert!(
            seen.contains(&data.tag),
            "{} is not in the sample",
            data.tag
        );
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    // The sample reaches the overrides of the hour and the day; the rest are
    // held by the loop over the list above.
    assert_eq!(sampled.len(), 13, "{sampled:?}");
}

/// The decimal separator is the one of the digits `hc-i18n` writes the
/// number in, which `scripts/humanize-cldr.py` names where it is not CLDR
/// 48's default system: Arabic's `arab`, ٫, for `ar`, whose CLDR default is
/// `latn`. A change of digits in `hc-i18n` fails here, and the script's
/// `NUMBERING` and its output then follow it.
#[test]
fn the_decimal_separator_belongs_to_the_digits_written() {
    for data in LOCALES {
        let locale: Locale = data.tag.parse().expect("well-formed tag");
        let digits = hc_i18n::NumberingSystem::for_locale(&locale).id();
        let expected = match data.tag {
            "ar" => "arab",
            "mr" => "deva",
            _ => "latn",
        };
        assert_eq!(digits, expected, "{}", data.tag);
    }
    let arabic: Locale = "ar".parse().expect("well-formed tag");
    assert_eq!(lookup::decimal_separator(&arabic), "\u{66b}");
}
