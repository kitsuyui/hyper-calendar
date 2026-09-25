//! The README's table of exports and the source cannot drift apart.
//!
//! `crates/hyper-calendar/tests/abi.rs` renders the table and diffs it;
//! this test, which lives with the module, reads both sides back and holds
//! them to each other by name and by feature: every `extern "C"` function
//! in `src/lib.rs` has a row, every row names a function that exists, the
//! feature the row gives is the feature the function is gated by, and
//! every such feature is one the manifest declares. A page that reads the
//! table to decide which build to load is reading the truth.

use std::collections::BTreeMap;

const SOURCE: &str = include_str!("../src/lib.rs");
const README: &str = include_str!("../README.md");
const MANIFEST: &str = include_str!("../Cargo.toml");

/// Every export in the source with the feature of the module it sits in,
/// or `always` outside any gated module.
fn exports_in_source() -> BTreeMap<String, String> {
    let lines: Vec<&str> = SOURCE.lines().collect();
    let mut out = BTreeMap::new();
    let mut feature: Option<&str> = None;
    for (index, line) in lines.iter().enumerate() {
        if let Some(gate) = line
            .strip_prefix("#[cfg(feature = \"")
            .and_then(|rest| rest.strip_suffix("\")]"))
            && lines
                .get(index + 1)
                .is_some_and(|next| next.starts_with("mod "))
        {
            feature = Some(gate);
        }
        if *line == "}" {
            feature = None;
        }
        if line.trim() != "#[unsafe(no_mangle)]" {
            continue;
        }
        let signature = lines[index + 1];
        let name = signature
            .split(" fn ")
            .nth(1)
            .and_then(|rest| rest.split('(').next())
            .unwrap_or_else(|| panic!("no function after line {}", index + 1));
        out.insert(name.to_owned(), feature.unwrap_or("always").to_owned());
    }
    out
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
