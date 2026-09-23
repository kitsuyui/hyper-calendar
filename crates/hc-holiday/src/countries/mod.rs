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
    ALGERIA, BAHRAIN, BOTSWANA, COTE_D_IVOIRE, EGYPT, ETHIOPIA, GHANA, IRAN, IRAQ, ISRAEL, JORDAN,
    KENYA, KUWAIT, LEBANON, LIBYA, MALAWI, MAURITIUS, MOROCCO, NAMIBIA, NIGERIA, OMAN, PALESTINE,
    QATAR, SAUDI_ARABIA, SENEGAL, SOUTH_AFRICA, SYRIA, TANZANIA, TUNISIA, TURKEY, UGANDA,
    UNITED_ARAB_EMIRATES, YEMEN, ZAMBIA, ZIMBABWE,
};
pub use americas::{
    ANTIGUA_AND_BARBUDA, ARGENTINA, BAHAMAS, BARBADOS, BELIZE, BOLIVIA, BRAZIL, CANADA, CHILE,
    COLOMBIA, COSTA_RICA, CUBA, DOMINICA, DOMINICAN_REPUBLIC, ECUADOR, EL_SALVADOR, GRENADA,
    GUATEMALA, GUYANA, HAITI, HONDURAS, JAMAICA, MEXICO, NICARAGUA, PANAMA, PARAGUAY, PERU,
    SAINT_KITTS_AND_NEVIS, SAINT_LUCIA, SAINT_VINCENT_AND_THE_GRENADINES, SURINAME,
    TRINIDAD_AND_TOBAGO, UNITED_STATES, URUGUAY, VENEZUELA,
};
pub use asia::{
    ARMENIA, AZERBAIJAN, BANGLADESH, BRUNEI, CAMBODIA, CHINA, GEORGIA, HONG_KONG, INDIA, INDONESIA,
    KAZAKHSTAN, KYRGYZSTAN, LAOS, MACAU, MALAYSIA, MONGOLIA, MYANMAR, NEPAL, PAKISTAN, PHILIPPINES,
    SINGAPORE, SOUTH_KOREA, SRI_LANKA, TAIWAN, TAJIKISTAN, THAILAND, TIMOR_LESTE, TURKMENISTAN,
    UZBEKISTAN, VIETNAM,
};
pub use europe::{
    ALBANIA, ANDORRA, AUSTRIA, BELARUS, BELGIUM, BOSNIA_AND_HERZEGOVINA, BULGARIA, CROATIA, CYPRUS,
    CZECHIA, DENMARK, ESTONIA, FINLAND, FRANCE, GERMANY, GREECE, HUNGARY, ICELAND, IRELAND, ITALY,
    LATVIA, LIECHTENSTEIN, LITHUANIA, LUXEMBOURG, MALTA, MOLDOVA, MONACO, MONTENEGRO, NETHERLANDS,
    NORTH_MACEDONIA, NORWAY, POLAND, PORTUGAL, ROMANIA, RUSSIA, SAN_MARINO, SERBIA, SLOVAKIA,
    SLOVENIA, SPAIN, SWEDEN, SWITZERLAND, UKRAINE, UNITED_KINGDOM, VATICAN_CITY,
};
pub use japan::JAPAN;
pub use oceania::{
    AUSTRALIA, MARSHALL_ISLANDS, MICRONESIA, NAURU, NEW_ZEALAND, PALAU, PAPUA_NEW_GUINEA, SAMOA,
    SOLOMON_ISLANDS, TONGA, TUVALU, VANUATU,
};

/// A country's holiday table. The same type as any other rule set: a country
/// is not special, it is just the rule set people ask for most.
pub type CountryRules = RuleSet;

/// Every country table in the crate, in ISO 3166-1 alpha-2 order.
pub static ALL: &[&CountryRules] = &[
    &ANDORRA,
    &UNITED_ARAB_EMIRATES,
    &ANTIGUA_AND_BARBUDA,
    &ALBANIA,
    &ARMENIA,
    &ARGENTINA,
    &AUSTRIA,
    &AUSTRALIA,
    &AZERBAIJAN,
    &BOSNIA_AND_HERZEGOVINA,
    &BARBADOS,
    &BANGLADESH,
    &BELGIUM,
    &BULGARIA,
    &BAHRAIN,
    &BRUNEI,
    &BOLIVIA,
    &BRAZIL,
    &BAHAMAS,
    &BOTSWANA,
    &BELARUS,
    &BELIZE,
    &CANADA,
    &SWITZERLAND,
    &COTE_D_IVOIRE,
    &CHILE,
    &CHINA,
    &COLOMBIA,
    &COSTA_RICA,
    &CUBA,
    &CYPRUS,
    &CZECHIA,
    &GERMANY,
    &DENMARK,
    &DOMINICA,
    &DOMINICAN_REPUBLIC,
    &ALGERIA,
    &ECUADOR,
    &ESTONIA,
    &EGYPT,
    &SPAIN,
    &ETHIOPIA,
    &FINLAND,
    &MICRONESIA,
    &FRANCE,
    &UNITED_KINGDOM,
    &GRENADA,
    &GEORGIA,
    &GHANA,
    &GREECE,
    &GUATEMALA,
    &GUYANA,
    &HONG_KONG,
    &HONDURAS,
    &CROATIA,
    &HAITI,
    &HUNGARY,
    &INDONESIA,
    &IRELAND,
    &ISRAEL,
    &INDIA,
    &IRAQ,
    &IRAN,
    &ICELAND,
    &ITALY,
    &JAMAICA,
    &JORDAN,
    &JAPAN,
    &KENYA,
    &KYRGYZSTAN,
    &CAMBODIA,
    &SAINT_KITTS_AND_NEVIS,
    &SOUTH_KOREA,
    &KUWAIT,
    &KAZAKHSTAN,
    &LAOS,
    &LEBANON,
    &SAINT_LUCIA,
    &LIECHTENSTEIN,
    &SRI_LANKA,
    &LITHUANIA,
    &LUXEMBOURG,
    &LATVIA,
    &LIBYA,
    &MOROCCO,
    &MONACO,
    &MOLDOVA,
    &MONTENEGRO,
    &MARSHALL_ISLANDS,
    &NORTH_MACEDONIA,
    &MYANMAR,
    &MONGOLIA,
    &MACAU,
    &MALTA,
    &MAURITIUS,
    &MALAWI,
    &MEXICO,
    &MALAYSIA,
    &NAMIBIA,
    &NIGERIA,
    &NICARAGUA,
    &NETHERLANDS,
    &NORWAY,
    &NEPAL,
    &NAURU,
    &NEW_ZEALAND,
    &OMAN,
    &PANAMA,
    &PERU,
    &PAPUA_NEW_GUINEA,
    &PHILIPPINES,
    &PAKISTAN,
    &POLAND,
    &PALESTINE,
    &PORTUGAL,
    &PALAU,
    &PARAGUAY,
    &QATAR,
    &ROMANIA,
    &SERBIA,
    &RUSSIA,
    &SAUDI_ARABIA,
    &SOLOMON_ISLANDS,
    &SWEDEN,
    &SINGAPORE,
    &SLOVENIA,
    &SLOVAKIA,
    &SAN_MARINO,
    &SENEGAL,
    &SURINAME,
    &EL_SALVADOR,
    &SYRIA,
    &THAILAND,
    &TAJIKISTAN,
    &TIMOR_LESTE,
    &TURKMENISTAN,
    &TUNISIA,
    &TONGA,
    &TURKEY,
    &TRINIDAD_AND_TOBAGO,
    &TUVALU,
    &TAIWAN,
    &TANZANIA,
    &UKRAINE,
    &UGANDA,
    &UNITED_STATES,
    &URUGUAY,
    &UZBEKISTAN,
    &VATICAN_CITY,
    &SAINT_VINCENT_AND_THE_GRENADINES,
    &VENEZUELA,
    &VIETNAM,
    &VANUATU,
    &SAMOA,
    &YEMEN,
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
        assert_eq!(ALL.len(), 154);
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
