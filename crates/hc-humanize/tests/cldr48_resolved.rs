//! Every carried locale's phrases against CLDR's own resolution of them.
//!
//! `data/cldr48_resolved.tsv` holds, for each locale [`LOCALES`] carries,
//! a sample of the values CLDR 48 resolves: the past, future and duration
//! patterns of the hour and the day in each plural category of the
//! language, and the words for yesterday, today and the like, in the long,
//! short and narrow styles. They were read from the `cldr-json` 48.0.0
//! packages (`cldr-dates-full` `main/<locale>/dateFields.json` and
//! `cldr-units-full` `main/<locale>/units.json`, `cldr-json-48`, read
//! 2026-09-28), which CLDR's
//! own tools resolve from the XML: every parent, alias and inheritance
//! marker (`↑↑↑`) is already followed there. A category the file has no key
//! for is the style's `other`, as a reader of those files takes it; a word
//! it has no key for is none.
//!
//! This is the independent check on the data this crate generates from the
//! XML: a value that follows CLDR's inheritance wrongly — a style left to the
//! long one where the file states its own, or a count filled from `other`
//! where a wider style states it — differs from CLDR's here, and every value
//! of the eleven generated locales must match.
//!
//! The 21 entries written by hand differ from CLDR 48 in places: English's
//! *the day before yesterday*, which CLDR's English does not state, German's
//! *{0} Std.* for the narrow hour, where CLDR writes *{0}h*, and the rest
//! `data/cldr48_hand_differences.tsv` lists, one sampled value a line. The
//! list is held exactly: a difference it does not name fails, and so does
//! one it names that no longer differs, so that the list says what the
//! hand-written data does.
//!
//! [`LOCALES`]: hc_humanize::data::LOCALES

use hc_humanize::data::LOCALES;
use hc_humanize::lookup;
use hc_humanize::{RelativeStyle, TimeUnit};
use hc_i18n::{Locale, PluralCategory};

const SAMPLE: &str = include_str!("data/cldr48_resolved.tsv");
const HAND_DIFFERENCES: &str = include_str!("data/cldr48_hand_differences.tsv");

/// The locales whose phrases are generated from the CLDR 48 files, which
/// must match everywhere.
const GENERATED: &[&str] = &[
    "fil", "ha", "mr", "pa-Guru", "pcm", "pt-PT", "sw", "te", "ur", "yue-Hans", "yue-Hant",
];

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

#[test]
fn every_locale_resolves_its_phrases_as_cldr_does() {
    let mut failures = Vec::new();
    let mut seen = Vec::new();
    let mut hand: Vec<&str> = HAND_DIFFERENCES.lines().collect();
    for row in &hand {
        let tag = row.split('\t').next().unwrap_or_default();
        assert!(
            !GENERATED.contains(&tag),
            "{row}: a generated locale may not differ"
        );
    }
    for line in SAMPLE.lines() {
        let mut cells = line.split('\t');
        let (Some(tag), Some(style), Some(unit), Some(kind)) =
            (cells.next(), cells.next(), cells.next(), cells.next())
        else {
            panic!("short line {line:?}");
        };
        if !seen.contains(&tag) {
            seen.push(tag);
        }
        let locale: Locale = tag.parse().expect("well-formed tag");
        let (style_name, unit_name) = (style, unit);
        let style = match style {
            "long" => RelativeStyle::Long,
            "short" => RelativeStyle::Short,
            "narrow" => RelativeStyle::Narrow,
            _ => panic!("unknown style {style}"),
        };
        let unit = match unit {
            "hour" => TimeUnit::Hour,
            "day" => TimeUnit::Day,
            _ => panic!("unknown unit {unit}"),
        };
        let patterns = lookup::unit_patterns(&locale, unit, style).expect("every unit is stated");
        for cell in cells {
            let (key, expected) = cell.split_once('=').expect("key=value");
            let actual = match kind {
                "past" => patterns.past.get(category(key)),
                "future" => patterns.future.get(category(key)),
                "count" => patterns.count.get(category(key)),
                "relative" => {
                    let offset: i64 = key.parse().expect("an offset");
                    lookup::special_word(&locale, unit, style, offset).unwrap_or("")
                }
                _ => panic!("unknown kind {kind}"),
            };
            let row = format!("{tag}\t{style_name}\t{unit_name}\t{kind}\t{key}");
            let listed = hand.iter().position(|listed| *listed == row);
            match (actual == expected, listed) {
                (true, None) => {}
                (false, Some(index)) => {
                    hand.swap_remove(index);
                }
                (true, Some(_)) => failures.push(format!("{row}: listed, but matches CLDR")),
                (false, None) => failures.push(format!(
                    "{tag} {style:?} {unit:?} {kind} {key}: {actual:?}, CLDR {expected:?}"
                )),
            }
        }
    }
    for row in hand {
        failures.push(format!("{row}: listed, but not in the sample"));
    }
    for data in LOCALES {
        assert!(
            seen.contains(&data.tag),
            "{} is not in the sample",
            data.tag
        );
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
