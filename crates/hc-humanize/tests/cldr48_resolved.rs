//! Every carried locale's phrases against CLDR's own resolution of them.
//!
//! `data/cldr48_resolved.tsv` holds, for each locale [`LOCALES`] carries,
//! every value it takes from CLDR 48, as CLDR resolves it: the past, future
//! and duration patterns of all eight units in each plural category of the
//! language and their words for two units before to two after (*the day
//! before yesterday*, *last year*), in the long, short and narrow styles;
//! the standard, unit and narrow list patterns; the decimal separator of
//! the digits `hc-i18n` writes; and the long `relative` date-time pattern
//! (UTS #35 Part 4, `uts35-dates-48`), in this crate's order.
//! `scripts/humanize-cldr-sample.py` reads them from the `cldr-json` 48.0.0
//! packages (`cldr-dates-full`, `cldr-units-full`, `cldr-misc-full`,
//! `cldr-numbers-full` and `cldr-core`, `cldr-json-48`, read 2026-09-28),
//! which CLDR's own tools resolve from the XML: every parent, alias and
//! inheritance marker (`↑↑↑`) is already followed there. A category the
//! file has no key for is the style's `other`, as a reader of those files
//! takes it; a word it has no key for is none.
//!
//! This is the independent check on the data `scripts/humanize-cldr.py`
//! generates from the XML with a resolver of its own: a value that follows
//! CLDR's inheritance wrongly — a style left to the long one where the file
//! states its own, or a count filled from `other` where a wider style
//! states it — differs from CLDR's here. Every locale's data is generated,
//! so a value may differ from CLDR's only where
//! `src/data/cldr48_overrides.tsv`, the one list of documented overrides,
//! says so and gives the value; every override must differ from CLDR's, be
//! the value the crate carries, and have its reason written above it in
//! the generated `src/data/cldr48.rs`. The sample and the list are both in
//! the repository, so the check needs no network: only regenerating the
//! sample or the data does.
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
const GENERATED: &str = include_str!("../src/data/cldr48.rs");

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

/// The override list: (tag, key, value, reason).
fn overrides() -> Vec<(&'static str, &'static str, &'static str, &'static str)> {
    OVERRIDES
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let cells: Vec<&str> = line.split('\t').collect();
            assert_eq!(cells.len(), 4, "{line:?}: four cells");
            assert!(!cells[3].is_empty(), "{line:?}: no reason");
            let value = if cells[2] == "-" { "" } else { cells[2] };
            (cells[0], cells[1], value, cells[3])
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
        let Some(tag) = cells.next() else {
            panic!("empty line");
        };
        if !seen.contains(&tag) {
            seen.push(tag);
        }
        for cell in cells {
            let (key, expected) = cell.split_once('=').expect("key=value");
            let actual = carried(tag, key);
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
    for (tag, key, value, _) in &overrides {
        let actual = carried(tag, key);
        if actual != *value {
            failures.push(format!(
                "{tag} {key}: overridden with {value:?}, carries {actual:?}"
            ));
        }
        if !sampled.contains(&(*tag, *key)) {
            failures.push(format!("{tag} {key}: overridden, but not in the sample"));
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
    // The sample holds every key, so it reaches every override once.
    assert_eq!(sampled.len(), overrides.len(), "{sampled:?}");
}

/// Every override's reason is written in the generated file, so that the
/// file was generated from the list as it stands: a reason edited in the
/// list and not regenerated fails here, with no network.
#[test]
fn the_generated_file_carries_every_override_reason() {
    let comments = GENERATED
        .lines()
        .filter_map(|line| line.trim_start().strip_prefix("// "))
        .collect::<Vec<_>>()
        .join(" ");
    let squeeze = |text: &str| text.split_whitespace().collect::<Vec<_>>().join(" ");
    let comments = squeeze(&comments);
    for (tag, key, _, reason) in overrides() {
        assert!(
            comments.contains(&squeeze(reason)),
            "{tag} {key}: the reason is not in src/data/cldr48.rs; run scripts/humanize-cldr.py"
        );
        assert!(
            comments.contains(&format!("`{key}`")),
            "{tag} {key}: the key is not in src/data/cldr48.rs; run scripts/humanize-cldr.py"
        );
    }
}

/// The decimal separator is the one of the digits `hc-i18n` writes the
/// number in, CLDR 48's default system, which `scripts/humanize-cldr.py`
/// reads too: `latn` for `ar`, whose `ar.xml` inherits root's, `arab`, ٫,
/// for `ar-EG`, and `arabext` for `ur-IN`. A change of digits in `hc-i18n`
/// fails here, and the script's `NUMBERING` and its output then follow it.
#[test]
fn the_decimal_separator_belongs_to_the_digits_written() {
    for data in LOCALES {
        let locale: Locale = data.tag.parse().expect("well-formed tag");
        let digits = hc_i18n::NumberingSystem::for_locale(&locale).id();
        let expected = match data.tag {
            "ar-EG" => "arab",
            "ur-IN" => "arabext",
            "mr" => "deva",
            _ => "latn",
        };
        assert_eq!(digits, expected, "{}", data.tag);
    }
    let egyptian: Locale = "ar-EG".parse().expect("well-formed tag");
    assert_eq!(lookup::decimal_separator(&egyptian), "\u{66b}");
    let arabic: Locale = "ar".parse().expect("well-formed tag");
    assert_eq!(lookup::decimal_separator(&arabic), ".");
}
