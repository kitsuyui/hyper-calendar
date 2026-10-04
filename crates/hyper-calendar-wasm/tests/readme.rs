//! The README's table of exports and the source cannot drift apart.
//!
//! `crates/hyper-calendar/tests/abi.rs` renders the table and diffs it;
//! this test, which lives with the module, reads both sides back and holds
//! them to each other by name and by feature: every `extern "C"` function
//! of the module's sources and of the rows of the facade's table it
//! expands has a row, every row names a function that exists, the
//! feature the row gives is the feature the function is gated by, and
//! every such feature is one the manifest declares. A page that reads the
//! table to decide which build to load is reading the truth.

use std::collections::BTreeMap;

#[path = "../../hyper-calendar/tests/support/boundaries.rs"]
mod boundaries;

const README: &str = include_str!("../README.md");
const MANIFEST: &str = include_str!("../Cargo.toml");

/// Every export with the feature of the module it sits in, or that expands
/// it from the facade's table, or `always` outside any gated module.
fn exports_in_source() -> BTreeMap<String, String> {
    boundaries::exports(
        std::path::Path::new("."),
        std::path::Path::new("../hyper-calendar"),
        boundaries::Side::Wasm,
    )
    .into_iter()
    .map(|export| (export.name, export.feature))
    .collect()
}

/// Every row of the README's export table, name to feature.
fn exports_in_readme() -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for line in README.lines() {
        let Some(rest) = line.strip_prefix("| `hc_") else {
            continue;
        };
        let Some((name, after_name)) = rest.split_once('(') else {
            panic!("a row without a signature: {line}")
        };
        let Some(feature) = after_name.split(" | ").nth(1) else {
            panic!("a row without a feature cell: {line}")
        };
        out.insert(format!("hc_{name}"), feature.trim_matches('`').to_owned());
    }
    out
}

/// The features `Cargo.toml` declares.
fn declared_features() -> Vec<String> {
    let mut features = Vec::new();
    let mut in_features = false;
    for line in MANIFEST.lines() {
        if line.starts_with('[') {
            in_features = line.trim() == "[features]";
            continue;
        }
        if in_features
            && let Some((name, _)) = line.split_once('=')
            && !name.trim_start().starts_with('#')
        {
            features.push(name.trim().to_owned());
        }
    }
    features
}

#[test]
fn every_export_has_a_row_and_every_row_an_export() {
    let source = exports_in_source();
    let readme = exports_in_readme();
    assert!(source.len() >= 19, "{source:?}");
    let missing: Vec<&String> = source
        .keys()
        .filter(|name| !readme.contains_key(*name))
        .collect();
    assert!(
        missing.is_empty(),
        "exported but not in the README: {missing:?}"
    );
    let stale: Vec<&String> = readme
        .keys()
        .filter(|name| !source.contains_key(*name))
        .collect();
    assert!(
        stale.is_empty(),
        "in the README but not exported: {stale:?}"
    );
}

#[test]
fn every_row_names_the_feature_its_export_is_gated_by() {
    let source = exports_in_source();
    let readme = exports_in_readme();
    let declared = declared_features();
    for (name, feature) in &source {
        assert_eq!(
            readme.get(name),
            Some(feature),
            "{name} is behind `{feature}` in the source"
        );
        assert!(
            feature == "always" || declared.contains(feature),
            "{name} is behind `{feature}`, which Cargo.toml does not declare"
        );
    }
}

#[test]
fn the_layers_are_the_ones_the_readme_describes() {
    // The README's layer table names every feature; a feature the manifest
    // declares and the README does not describe is a layer nobody can find.
    for feature in declared_features() {
        if feature == "default" {
            continue;
        }
        assert!(
            README.contains(&format!("\n| `{feature}` ")),
            "the README's layer table has no row for `{feature}`"
        );
    }
}

/// The exports a layer row of the README's layer table lists: the names in
/// backticks that begin the row's second cell, up to the colon that starts
/// its description. `None` for a row whose cell lists none (`civil`, which
/// is described in words, and `full`, which is all of them).
fn layer_row_exports(feature: &str) -> Option<Vec<String>> {
    let row = README
        .lines()
        .find(|line| line.starts_with(&format!("| `{feature}` ")))
        .unwrap_or_else(|| panic!("the README's layer table has no row for `{feature}`"));
    let cell = row
        .split(" | ")
        .nth(1)
        .unwrap_or_else(|| panic!("no exports cell in {row}"));
    let list = cell.split_once(": ").map_or(cell, |(list, _)| list);
    let mut names = Vec::new();
    for part in list.split(", ") {
        let name = part.trim().trim_matches('`');
        if name.starts_with("hc_") {
            names.push(name.to_owned());
        }
    }
    (!names.is_empty()).then_some(names)
}

#[test]
fn every_layer_row_lists_the_exports_its_layer_holds() {
    let source = exports_in_source();
    let mut checked = 0;
    for feature in declared_features() {
        if matches!(feature.as_str(), "default" | "civil" | "full") {
            continue;
        }
        let listed = layer_row_exports(&feature)
            .unwrap_or_else(|| panic!("the layer row of `{feature}` lists no export"));
        let mut held: Vec<String> = source
            .iter()
            .filter(|(_, own)| **own == feature)
            .map(|(name, _)| name.clone())
            .collect();
        let mut listed_sorted = listed.clone();
        listed_sorted.sort();
        held.sort();
        let missing: Vec<&String> = held.iter().filter(|name| !listed.contains(name)).collect();
        let extra: Vec<&String> = listed.iter().filter(|name| !held.contains(name)).collect();
        assert!(
            missing.is_empty() && extra.is_empty(),
            "the layer row of `{feature}` omits {missing:?} and lists {extra:?} that are not in the layer"
        );
        let mut duplicates = listed.clone();
        duplicates.sort();
        duplicates.dedup();
        assert_eq!(
            duplicates.len(),
            listed.len(),
            "`{feature}` lists a name twice"
        );
        checked += 1;
    }
    assert!(checked >= 20, "{checked} layers checked");
}
