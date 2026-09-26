//! Counts the READMEs state, kept where they can be checked.
//!
//! `countries/mod.rs` already guards its own length with the comment "a
//! documented count that drifts is a documented lie". These are the other
//! numbers the prose commits to. A count like these is easy to get wrong by
//! hand in either direction, which is the argument for computing it rather
//! than correcting it.

use hc_holiday::{CalendarSystem, Rule, countries, traditions};

/// The crate's README, whose prose states the counts checked here.
const README: &str = include_str!("../README.md");

/// A number under a hundred in English words, as the README spells its
/// counts: "sixty-nine", "twenty-two".
fn in_words(number: usize) -> String {
    const UNITS: [&str; 20] = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    const TENS: [&str; 10] = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    assert!(number < 100, "{number} is not spelled here");
    match (number / 10, number % 10) {
        (0 | 1, _) => UNITS[number].to_owned(),
        (tens, 0) => TENS[tens].to_owned(),
        (tens, units) => format!("{}-{}", TENS[tens], UNITS[units]),
    }
}

/// Whether the README states `count` in words followed by `noun`, in lower
/// case or at the start of a sentence.
fn readme_states(count: usize, noun: &str) -> bool {
    let words = in_words(count);
    let mut capitalised = words.clone();
    capitalised[..1].make_ascii_uppercase();
    README.contains(&format!("{words} {noun}")) || README.contains(&format!("{capitalised} {noun}"))
}

/// Whether a rule is dated in a Hijri calendar, following `Offset` down to
/// the rule it shifts.
fn uses_hijri(rule: &Rule) -> bool {
    // Compared by identifier, because a `CalendarSystem` is a struct
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
        71,
        "got {countries_with_hijri:?}"
    );
    // And the README's prose says the same number, so the two cannot drift.
    assert!(
        readme_states(countries_with_hijri.len(), "countries"),
        "the README should say \"{} countries\"",
        in_words(countries_with_hijri.len())
    );
    // Named, so that a country losing its Hijri holidays is visible rather
    // than merely a smaller number.
    for expected in [
        "AE", "AF", "AL", "AR", "AZ", "BA", "BD", "BF", "BH", "BI", "BJ", "BN", "CI", "CM", "DJ",
        "DZ", "EG", "ET", "GH", "GN", "GY", "ID", "IN", "IQ", "IR", "JO", "KE", "KG", "KM", "KW",
        "KZ", "LB", "LY", "MA", "ME", "MG", "MK", "ML", "MM", "MR", "MU", "MV", "MW", "MY", "NE",
        "NG", "NP", "OM", "PH", "PK", "PS", "QA", "RS", "RW", "SA", "SG", "SN", "SO", "SR", "SY",
        "TD", "TJ", "TL", "TM", "TN", "TR", "TT", "TZ", "UG", "UZ", "YE",
    ] {
        assert!(
            countries_with_hijri.contains(&expected),
            "{expected} should have a Hijri-dated holiday"
        );
    }
}

#[test]
fn the_tradition_count_is_the_one_the_readme_states() {
    assert_eq!(traditions::ALL.len(), 43);
    assert!(
        readme_states(traditions::ALL.len(), "traditions"),
        "the README should say \"{} traditions\"",
        in_words(traditions::ALL.len())
    );
}
