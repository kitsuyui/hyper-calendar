//! Counts the READMEs state, kept where they can be checked.
//!
//! `countries/mod.rs` already guards its own length with the comment "a
//! documented count that drifts is a documented lie". These are the other
//! numbers the prose commits to. One of them was wrong — the README said
//! Hijri dates appeared in nine countries and the tables held ten — and
//! another that an audit reported as wrong turned out to be right, which is
//! the better argument for computing them than for correcting them.

use hc_holiday::{CalendarSystem, Rule, countries, traditions};

/// Whether a rule is dated in a Hijri calendar, following `Offset` down to
/// the rule it shifts.
fn uses_hijri(rule: &Rule) -> bool {
    // Compared by identifier, because a `CalendarSystem` is now a struct
    // carrying function pointers and those cannot appear in a pattern.
    match rule {
        Rule::FixedInCalendar { system, .. } => {
            *system == CalendarSystem::ISLAMIC_CIVIL
                || *system == CalendarSystem::ISLAMIC_UMM_AL_QURA
        }
        Rule::Offset { base, .. } => uses_hijri(base),
        _ => false,
    }
}

#[test]
fn hijri_dates_appear_in_the_number_of_countries_the_readme_states() {
    let countries_with_hijri: Vec<&str> = countries::ALL
        .iter()
        .filter(|set| set.rules.iter().any(|rule| uses_hijri(&rule.rule)))
        .map(|set| set.code)
        .collect();
    assert_eq!(
        countries_with_hijri.len(),
        53,
        "got {countries_with_hijri:?}"
    );
    // Named, so that a country losing its Hijri holidays is visible rather
    // than merely a smaller number.
    for expected in [
        "AE", "AL", "AR", "AZ", "BA", "BD", "BF", "BH", "BI", "BJ", "BN", "CI", "CM", "DZ", "EG",
        "ET", "GH", "GN", "GY", "ID", "IN", "IQ", "IR", "JO", "KE", "KG", "KW", "KZ", "LB", "LY",
        "MA", "ME", "MG", "MK", "ML", "MM", "MU", "MW", "MY", "NG", "NP", "OM", "PH", "PK", "PS",
        "QA", "RS", "RW", "SA", "SG", "SR", "SY", "TL", "TR", "TT", "TZ", "UG", "YE",
    ] {
        assert!(
            countries_with_hijri.contains(&expected),
            "{expected} should have a Hijri-dated holiday"
        );
    }
}

#[test]
fn the_tradition_count_is_the_one_the_readme_states() {
    assert_eq!(traditions::ALL.len(), 20);
}
