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
    BAHRAIN, EGYPT, ETHIOPIA, GHANA, IRAN, ISRAEL, JORDAN, KENYA, KUWAIT, LEBANON, MOROCCO,
    NIGERIA, SAUDI_ARABIA, SOUTH_AFRICA, TANZANIA, TURKEY, UGANDA, UNITED_ARAB_EMIRATES, ZAMBIA,
    ZIMBABWE,
};
pub use americas::{
    ARGENTINA, BOLIVIA, BRAZIL, CANADA, CHILE, COLOMBIA, COSTA_RICA, DOMINICAN_REPUBLIC, ECUADOR,
    GUATEMALA, MEXICO, PANAMA, PERU, UNITED_STATES, URUGUAY,
};
pub use asia::{
    ARMENIA, AZERBAIJAN, CHINA, GEORGIA, HONG_KONG, INDIA, INDONESIA, KAZAKHSTAN, MACAU, MALAYSIA,
    MYANMAR, NEPAL, PAKISTAN, PHILIPPINES, SINGAPORE, SOUTH_KOREA, TAIWAN, THAILAND, VIETNAM,
};
pub use europe::{
    ALBANIA, AUSTRIA, BELARUS, BELGIUM, BULGARIA, CROATIA, CYPRUS, CZECHIA, DENMARK, ESTONIA,
    FINLAND, FRANCE, GERMANY, GREECE, HUNGARY, ICELAND, IRELAND, ITALY, LATVIA, LITHUANIA,
    LUXEMBOURG, MALTA, MOLDOVA, MONTENEGRO, NETHERLANDS, NORTH_MACEDONIA, NORWAY, POLAND, PORTUGAL,
    ROMANIA, RUSSIA, SERBIA, SLOVAKIA, SLOVENIA, SPAIN, SWEDEN, SWITZERLAND, UKRAINE,
    UNITED_KINGDOM,
};
pub use japan::JAPAN;
pub use oceania::{AUSTRALIA, NEW_ZEALAND};

/// A country's holiday table. The same type as any other rule set: a country
/// is not special, it is just the rule set people ask for most.
pub type CountryRules = RuleSet;

/// Every country table in the crate, in ISO 3166-1 alpha-2 order.
pub static ALL: &[&CountryRules] = &[
    &UNITED_ARAB_EMIRATES,
    &ALBANIA,
    &ARMENIA,
    &ARGENTINA,
    &AUSTRIA,
    &AUSTRALIA,
    &AZERBAIJAN,
    &BELGIUM,
    &BULGARIA,
    &BAHRAIN,
    &BOLIVIA,
    &BRAZIL,
    &BELARUS,
    &CANADA,
    &SWITZERLAND,
    &CHILE,
    &CHINA,
    &COLOMBIA,
    &COSTA_RICA,
    &CYPRUS,
    &CZECHIA,
    &GERMANY,
    &DENMARK,
    &DOMINICAN_REPUBLIC,
    &ECUADOR,
    &ESTONIA,
    &EGYPT,
    &SPAIN,
    &ETHIOPIA,
    &FINLAND,
    &FRANCE,
    &UNITED_KINGDOM,
    &GEORGIA,
    &GHANA,
    &GREECE,
    &GUATEMALA,
    &HONG_KONG,
    &CROATIA,
    &HUNGARY,
    &INDONESIA,
    &IRELAND,
    &ISRAEL,
    &INDIA,
    &IRAN,
    &ICELAND,
    &ITALY,
    &JORDAN,
    &JAPAN,
    &KENYA,
    &SOUTH_KOREA,
    &KUWAIT,
    &KAZAKHSTAN,
    &LEBANON,
    &LITHUANIA,
    &LUXEMBOURG,
    &LATVIA,
    &MOROCCO,
    &MOLDOVA,
    &MONTENEGRO,
    &NORTH_MACEDONIA,
    &MYANMAR,
    &MACAU,
    &MALTA,
    &MEXICO,
    &MALAYSIA,
    &NIGERIA,
    &NETHERLANDS,
    &NORWAY,
    &NEPAL,
    &NEW_ZEALAND,
    &PANAMA,
    &PERU,
    &PHILIPPINES,
    &PAKISTAN,
    &POLAND,
    &PORTUGAL,
    &ROMANIA,
    &SERBIA,
    &RUSSIA,
    &SAUDI_ARABIA,
    &SWEDEN,
    &SINGAPORE,
    &SLOVENIA,
    &SLOVAKIA,
    &THAILAND,
    &TURKEY,
    &TAIWAN,
    &TANZANIA,
    &UKRAINE,
    &UGANDA,
    &UNITED_STATES,
    &URUGUAY,
    &VIETNAM,
    &SOUTH_AFRICA,
    &ZAMBIA,
    &ZIMBABWE,
];

/// The table for an ISO 3166-1 alpha-2 country code, case-insensitively.
#[must_use]
pub fn by_code(code: &str) -> Option<&'static CountryRules> {
    ALL.iter()
        .copied()
        .find(|country| code.len() == country.code.len() && code.eq_ignore_ascii_case(country.code))
}

hc_core::catalogue_tests! {
    type: &'static CountryRules,
    id: |country| country.code,
    sorted_by: |country| country.code,
    provenance: |country| country.sources,
    tests: country_table_tests,
    all: ALL,
    lookup: by_code,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule::Confidence;

    #[test]
    fn a_code_is_found_whatever_its_case() {
        assert_eq!(by_code("jp").map(|found| found.code), Some("JP"));
        assert!(by_code("ZZ").is_none());
    }

    #[test]
    fn the_registry_holds_the_number_of_countries_the_readme_claims() {
        // `README.md` and `docs/observances.md` both state this figure, and a
        // documented count that drifts is a documented lie.
        assert_eq!(ALL.len(), 96);
    }

    #[test]
    fn every_table_says_when_its_sources_were_checked() {
        for country in ALL {
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

        /// Whether a rule bottoms out in a Hijri calendar.
        ///
        /// Compared by identifier rather than matched: a `CalendarSystem`
        /// carries function pointers now, and those cannot appear in a
        /// pattern. That is the cost of the set being open, and it is a
        /// small one.
        fn hijri_dated(rule: &Rule) -> bool {
            match rule {
                Rule::FixedInCalendar { system, .. } => {
                    *system == CalendarSystem::ISLAMIC_CIVIL
                        || *system == CalendarSystem::ISLAMIC_UMM_AL_QURA
                }
                Rule::Offset { base, .. } => hijri_dated(base),
                _ => false,
            }
        }

        for country in ALL {
            for rule in country.rules {
                if hijri_dated(&rule.rule) {
                    assert_eq!(
                        rule.confidence,
                        Confidence::Approximate,
                        "{}: {} is Hijri-dated and must be a prediction",
                        country.code,
                        rule.name
                    );
                }
            }
        }
    }
}
