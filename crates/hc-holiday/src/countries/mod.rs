//! The national tables.
//!
//! Every country here is a [`CountryRules`] value — a name, a list of
//! [`HolidayRule`](crate::rule::HolidayRule)s, the weekend and substitution
//! law that modifies them, the date the sources were last checked and the
//! statute or gazette they came from. None of them contributes a line of
//! logic.
//!
//! # What is and is not claimed
//!
//! * **Japan** ([`japan`]) is complete from 1948 and exact, amendment by
//!   amendment. It is the crate's proof that the data-not-code rule holds
//!   for a hard case.
//! * Every other table covers the **present-day national list**, with
//!   historical `valid_from` / `valid_until` years wherever a change is
//!   named in the source. They are not complete back to each country's
//!   founding, and they do not claim to be.
//! * Every **Hijri-dated** entry is flagged
//!   [`Confidence::Approximate`](crate::rule::Confidence::Approximate),
//!   because the observed date is a decision made on a crescent sighting,
//!   per country, sometimes on the night before.
//! * A country whose substitution law the author could not cite carries **no
//!   substitution policy**, and its holidays fall on the weekend and stay
//!   there. That is a deliberate refusal, not an oversight.
//!
//! Subdivisions are named with ISO 3166-2 codes. Asking for the national
//! set — `region = None` — returns only the rules with no subdivision
//! scoping, never the union of every subdivision's rules.

use crate::rule::RuleSet;

pub mod africa_middle_east;
pub mod americas;
pub mod asia;
pub mod europe;
pub mod japan;
pub mod oceania;

pub use africa_middle_east::{
    EGYPT, ISRAEL, NIGERIA, SAUDI_ARABIA, SOUTH_AFRICA, TURKEY, UNITED_ARAB_EMIRATES,
};
pub use americas::{BRAZIL, CANADA, MEXICO, UNITED_STATES};
pub use asia::{
    CHINA, INDIA, INDONESIA, MALAYSIA, NEPAL, PHILIPPINES, SINGAPORE, SOUTH_KOREA, TAIWAN,
    THAILAND, VIETNAM,
};
pub use europe::{
    AUSTRIA, BELGIUM, CZECHIA, DENMARK, FINLAND, FRANCE, GERMANY, GREECE, IRELAND, ITALY,
    NETHERLANDS, NORWAY, POLAND, PORTUGAL, SPAIN, SWEDEN, SWITZERLAND, UNITED_KINGDOM,
};
pub use japan::JAPAN;
pub use oceania::{AUSTRALIA, NEW_ZEALAND};

/// A country's holiday table. The same type as any other rule set: a country
/// is not special, it is just the rule set people ask for most.
pub type CountryRules = RuleSet;

/// Every country table in the crate, in ISO 3166-1 alpha-2 order.
pub static ALL: &[&CountryRules] = &[
    &AUSTRIA,
    &AUSTRALIA,
    &BELGIUM,
    &BRAZIL,
    &CANADA,
    &SWITZERLAND,
    &CHINA,
    &CZECHIA,
    &GERMANY,
    &DENMARK,
    &EGYPT,
    &SPAIN,
    &FINLAND,
    &FRANCE,
    &UNITED_KINGDOM,
    &GREECE,
    &INDONESIA,
    &IRELAND,
    &ISRAEL,
    &INDIA,
    &ITALY,
    &JAPAN,
    &SOUTH_KOREA,
    &MEXICO,
    &MALAYSIA,
    &NIGERIA,
    &NETHERLANDS,
    &NORWAY,
    &NEPAL,
    &NEW_ZEALAND,
    &PHILIPPINES,
    &POLAND,
    &PORTUGAL,
    &SAUDI_ARABIA,
    &SWEDEN,
    &SINGAPORE,
    &THAILAND,
    &TURKEY,
    &TAIWAN,
    &UNITED_ARAB_EMIRATES,
    &UNITED_STATES,
    &VIETNAM,
    &SOUTH_AFRICA,
];

/// The table for an ISO 3166-1 alpha-2 country code, case-insensitively.
#[must_use]
pub fn by_code(code: &str) -> Option<&'static CountryRules> {
    ALL.iter()
        .copied()
        .find(|country| code.len() == country.code.len() && code.eq_ignore_ascii_case(country.code))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule::Confidence;

    #[test]
    fn every_country_is_reachable_by_its_code() {
        for country in ALL {
            assert_eq!(
                by_code(country.code).map(|found| found.code),
                Some(country.code)
            );
        }
        assert_eq!(by_code("jp").map(|found| found.code), Some("JP"));
        assert!(by_code("ZZ").is_none());
    }

    #[test]
    fn the_registry_holds_the_number_of_countries_the_readme_claims() {
        // `README.md` and `docs/observances.md` both state this figure, and a
        // documented count that drifts is a documented lie.
        assert_eq!(ALL.len(), 43);
    }

    #[test]
    fn no_country_code_is_registered_twice() {
        for (index, country) in ALL.iter().enumerate() {
            for other in &ALL[index + 1..] {
                assert_ne!(country.code, other.code, "duplicate {}", country.code);
            }
        }
    }

    #[test]
    fn every_table_names_its_sources_and_when_they_were_checked() {
        for country in ALL {
            assert!(
                !country.sources.is_empty(),
                "{} has no cited source",
                country.code
            );
            assert!(
                country.sources_checked.year >= 2026,
                "{} was checked before this crate existed",
                country.code
            );
        }
    }

    #[test]
    fn every_hijri_dated_rule_is_flagged_as_a_prediction() {
        use crate::rule::{CalendarSystem, Rule};
        for country in ALL {
            for rule in country.rules {
                let hijri = matches!(
                    rule.rule,
                    Rule::FixedInCalendar {
                        system: CalendarSystem::IslamicCivil | CalendarSystem::IslamicUmmAlQura,
                        ..
                    }
                ) || matches!(
                    rule.rule,
                    Rule::Offset {
                        base: Rule::FixedInCalendar {
                            system: CalendarSystem::IslamicCivil | CalendarSystem::IslamicUmmAlQura,
                            ..
                        },
                        ..
                    }
                );
                if hijri {
                    assert_eq!(
                        rule.confidence,
                        Confidence::Approximate,
                        "{} / {} is Hijri-dated but claims to be exact",
                        country.code,
                        rule.name
                    );
                }
            }
        }
    }
}
