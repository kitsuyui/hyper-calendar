//! No data taken from Unicode CLDR carries one of CLDR's two reserved
//! values as though it were a name.
//!
//! UTS #35 version 48.2, Part 1, reserves two values [uts35-v48]: the
//! inheritance marker `↑↑↑`, which a conformant implementation reads as the
//! element being absent, and the empty override `∅∅∅`, which says that a
//! locale "is to have no value for a path, even if the parent locale has a
//! value for that path". Neither is text to show. The generators resolve
//! both (`scripts/cldr_xml.py`, and the others refuse the empty override
//! at the paths they read), and the zone names write the override as `~`,
//! which `hc_i18n::zone_names` reads as "no name". This test reads every
//! file the CLDR generators write and the tables kept by hand from CLDR's
//! files, and fails on a marker anywhere outside a comment, so that a
//! regenerated file cannot bring one back.

use std::fs;
use std::path::{Path, PathBuf};

/// The files `scripts/*-cldr.py` write.
const GENERATED: [&str; 8] = [
    "crates/hc-i18n/src/zone_names/cldr48.rs",
    "crates/hc-i18n/src/day_periods/cldr48.rs",
    "crates/hc-i18n/src/data/cldr48_locales.rs",
    "crates/hc-i18n/src/data/japanese_eras.rs",
    "crates/hc-i18n/src/place_names/cldr48.rs",
    "crates/hc-humanize/src/data/cldr48.rs",
    "crates/hc-humanize/tests/data/cldr48_resolved.tsv",
    "crates/hc-calendars-regional/src/nengo/table.rs",
];

/// Tables kept by hand from CLDR's files: their data, before the tests.
const KEPT: [&str; 2] = [
    "crates/hc-i18n/src/data.rs",
    "crates/hc-i18n/src/exemplar_cities.rs",
];

const MARKERS: [&str; 2] = ["∅∅∅", "↑↑↑"];

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The lines of a text, numbered from 1, that hold a marker outside a
/// comment, up to its test module where `stop_at_tests` asks.
fn marked_lines(text: &str, stop_at_tests: bool) -> Vec<(usize, &str)> {
    let mut found = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        if stop_at_tests && trimmed.starts_with("#[cfg(test)]") {
            break;
        }
        if !trimmed.starts_with("//") && MARKERS.iter().any(|marker| line.contains(marker)) {
            found.push((index + 1, trimmed));
        }
    }
    found
}

fn problems(path: &str, stop_at_tests: bool) -> Vec<String> {
    let text = fs::read_to_string(repository_root().join(path))
        .unwrap_or_else(|error| panic!("{path}: {error}"));
    marked_lines(&text, stop_at_tests)
        .into_iter()
        .map(|(number, line)| format!("{path}:{number}: {line}"))
        .collect()
}

#[test]
fn no_cldr_data_writes_a_reserved_marker_as_a_value() {
    let mut found = Vec::new();
    for path in GENERATED {
        found.extend(problems(path, false));
    }
    for path in KEPT {
        found.extend(problems(path, true));
    }
    assert!(found.is_empty(), "{}", found.join("\n"));
}

/// The scan itself: a literal holding either marker is caught, a comment
/// that names one is not, and a kept table's tests are left out.
#[test]
fn the_scan_finds_a_marker_in_a_literal_and_not_in_a_comment() {
    let text = "// `∅∅∅` is CLDR's empty override.\n    \"|||∅∅∅\",\n(\"GB\", \"↑↑↑\"),\n\
                #[cfg(test)]\nassert_eq!(x, \"↑↑↑\");\n";
    let lines = |stop| {
        marked_lines(text, stop)
            .into_iter()
            .map(|(number, _)| number)
            .collect::<Vec<_>>()
    };
    assert_eq!(lines(false), [2, 3, 5]);
    assert_eq!(lines(true), [2, 3]);
}
