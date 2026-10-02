//! Every holiday of every table has a well-formed identifier, and within a
//! table an identifier names one holiday.
//!
//! An identifier is the kebab-case of the English name unless the rule
//! sets one (`hc_holiday::id`). Two rules of one table with the same
//! English name, or with names that differ only in case, punctuation or
//! diacritics, share it and are the same holiday; a rule that sets its
//! own identifier shares it with none.

use std::collections::BTreeMap;

use hc_holiday::id::Kebab;
use hc_holiday::rule::RuleSet;
use hc_holiday::{countries, exchanges, international, traditions};

fn tables() -> impl Iterator<Item = &'static RuleSet> {
    countries::ALL
        .iter()
        .chain(exchanges::ALL)
        .chain(traditions::ALL)
        .chain(international::ALL)
        .copied()
}

/// Lower-case ASCII letters and digits, in runs joined by single hyphens.
fn well_formed(id: &str) -> bool {
    !id.is_empty()
        && !id.starts_with('-')
        && !id.ends_with('-')
        && !id.contains("--")
        && id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

#[test]
fn the_kebab_case_of_a_name_is_as_documented() {
    let kebab = |name: &str| Kebab::new(name).collect::<String>();
    assert_eq!(kebab("New Year's Day"), "new-years-day");
    assert_eq!(kebab("Washington's Birthday"), "washingtons-birthday");
    assert_eq!(kebab("Tōkanya"), "tokanya");
    assert_eq!(kebab("Kurban Bayramı"), "kurban-bayrami");
    assert_eq!(kebab("Prešeren Day"), "preseren-day");
    assert_eq!(kebab("Nayrouz (New Year)"), "nayrouz-new-year");
    assert_eq!(kebab("  A  --  B "), "a-b");
    assert_eq!(kebab("Straße"), "strasse");
    assert_eq!(kebab("'Eid'"), "eid");
    assert_eq!(kebab("St George's"), "st-georges");
    assert_eq!(kebab("元日"), "");
}

#[test]
fn every_rule_has_a_well_formed_identifier() {
    let mut faults = Vec::new();
    for table in tables() {
        for rule in table.rules {
            let id = rule.id().to_string();
            if !well_formed(&id) {
                faults.push(format!("{}: {:?} -> {id:?}", table.code, rule.name));
            }
            if !rule.id.is_empty() && !well_formed(rule.id) {
                faults.push(format!("{}: explicit {:?}", table.code, rule.id));
            }
        }
    }
    assert!(faults.is_empty(), "{}", faults.join("\n"));
}

/// Names that share an identifier are spelling variants of one holiday:
/// the identifier folds case, punctuation, apostrophes and diacritics, so
/// `Mothers' Day` and `Mother's Day`, which US states spell both ways of
/// one observance, are one holiday. A rule that sets an identifier of its
/// own is the exception made for two days a fold would merge, and its
/// identifier must be no other name's.
#[test]
fn within_a_table_an_identifier_names_one_holiday() {
    let mut collisions = Vec::new();
    for table in tables() {
        let mut names: BTreeMap<String, Vec<(&str, bool)>> = BTreeMap::new();
        for rule in table.rules {
            let seen = names.entry(rule.id().to_string()).or_default();
            let explicit = !rule.id.is_empty();
            if !seen.iter().any(|(name, _)| *name == rule.name) {
                seen.push((rule.name, explicit));
            }
        }
        for (id, seen) in names {
            if seen.len() > 1 && seen.iter().any(|(_, explicit)| *explicit) {
                collisions.push(format!("{}: {id} is set for {seen:?}", table.code));
            }
        }
    }
    assert!(collisions.is_empty(), "{}", collisions.join("\n"));
}

#[test]
fn an_identifier_compares_by_what_it_renders_to() {
    use hc_holiday::HolidayId;
    assert_eq!(
        HolidayId::of_name("New Year's Day"),
        HolidayId::explicit("new-years-day")
    );
    assert!(HolidayId::of_name("New Year's Day").matches(" New-Years-Day "));
    assert!(!HolidayId::of_name("New Year's Day").matches("new-year"));
    assert_eq!(HolidayId::of_name("Tōkanya").to_string(), "tokanya");
    assert!(HolidayId::explicit("x").is_explicit());
    assert!(!HolidayId::of_name("x").is_explicit());
}
