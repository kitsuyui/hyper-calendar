//! The national tables.
//!
//! Every country here is a [`RuleSet`] value — a name, a list of
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
//! * Every other table is the national list **from the first year its
//!   sources support**, with historical `valid_from` / `valid_until` years
//!   wherever a change is named in the source. Before that year the engine
//!   reports a gap, never an answer (ADR 0013): `read_from`, which the
//!   tables of the Americas, Europe, Africa, the Middle East, Oceania and Asia
//!   set for the whole table through `read_all`, and the reasons for each
//!   table's first year are in `docs/systems/holiday-first-years.md`.
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

/// Reads every rule of a table from `first`, the first year its sources
/// answer for (ADR 0013): a rule with no `read_from` of its own gets it, so
/// that each year before `first` that the rule's establishment does not rule
/// out is a gap, which the engine reports with the rule's name and the
/// table's `sources`. A rule that already has a `read_from` keeps it, and a
/// rule whose `valid_from` is later than `first` stays absent before then,
/// which is an answer.
///
/// `first` is the first whole year the earliest instrument or list read
/// states: the year of a law's commencement, or the first year of a list
/// that was read for a year and not a law. A table says in its doc comment
/// which that is.
#[must_use]
pub(crate) const fn read_all<const N: usize>(
    first: i32,
    mut rules: [crate::rule::HolidayRule; N],
) -> [crate::rule::HolidayRule; N] {
    let mut index = 0;
    while index < N {
        if rules[index].read_from.is_none() {
            rules[index].read_from = Some(first);
        }
        index += 1;
    }
    rules
}

pub mod africa_middle_east;
pub mod americas;
pub mod andorra;
pub mod asia;
mod bangladesh_optional;
pub mod bhutan;
pub mod bolivia;
pub mod canada;
mod china_scoped;
pub mod europe;
pub mod india;
pub mod japan;
pub mod mexico;
mod nepal_sections;
pub mod new_zealand;
pub mod oceania;
mod russia_republics;
pub mod solomon_islands;
pub mod spain;
pub mod switzerland;
mod taiwan_scoped;
pub mod united_states;
pub mod vanuatu;

pub use africa_middle_east::{
    ALGERIA, ANGOLA, BAHRAIN, BENIN, BOTSWANA, BURKINA_FASO, BURUNDI, CABO_VERDE, CAMEROON, CHAD,
    COMOROS, CONGO, COTE_D_IVOIRE, DJIBOUTI, DR_CONGO, EGYPT, EQUATORIAL_GUINEA, ESWATINI,
    ETHIOPIA, GABON, GAMBIA, GHANA, GUINEA, GUINEA_BISSAU, IRAN, IRAQ, ISRAEL, JORDAN, KENYA,
    KUWAIT, LEBANON, LESOTHO, LIBERIA, LIBYA, MADAGASCAR, MALAWI, MALI, MAURITANIA, MAURITIUS,
    MOROCCO, MOZAMBIQUE, NAMIBIA, NIGER, NIGERIA, OMAN, PALESTINE, QATAR, RWANDA, SAUDI_ARABIA,
    SENEGAL, SEYCHELLES, SIERRA_LEONE, SOMALIA, SOUTH_AFRICA, SOUTH_SUDAN, SUDAN, SYRIA, TANZANIA,
    TOGO, TUNISIA, TURKEY, UGANDA, UNITED_ARAB_EMIRATES, YEMEN, ZAMBIA, ZIMBABWE,
};
pub use americas::{
    ANTIGUA_AND_BARBUDA, ARGENTINA, BAHAMAS, BARBADOS, BELIZE, BRAZIL, CHILE, COLOMBIA, COSTA_RICA,
    CUBA, DOMINICA, DOMINICAN_REPUBLIC, ECUADOR, EL_SALVADOR, GRENADA, GUATEMALA, GUYANA, HAITI,
    HONDURAS, JAMAICA, NICARAGUA, PANAMA, PARAGUAY, PERU, SAINT_KITTS_AND_NEVIS, SAINT_LUCIA,
    SAINT_VINCENT_AND_THE_GRENADINES, SURINAME, TRINIDAD_AND_TOBAGO, URUGUAY, VENEZUELA,
};
pub use andorra::ANDORRA;
pub use asia::{
    AFGHANISTAN, ARMENIA, AZERBAIJAN, BANGLADESH, BHUTAN, BRUNEI, CAMBODIA, CHINA, GEORGIA,
    HONG_KONG, INDIA, INDONESIA, KAZAKHSTAN, KYRGYZSTAN, LAOS, MACAU, MALAYSIA, MALDIVES, MONGOLIA,
    MYANMAR, NEPAL, NORTH_KOREA, PAKISTAN, PHILIPPINES, SINGAPORE, SOUTH_KOREA, SRI_LANKA, TAIWAN,
    TAJIKISTAN, THAILAND, TIMOR_LESTE, TURKMENISTAN, UZBEKISTAN, VIETNAM,
};
pub use bolivia::BOLIVIA;
pub use canada::CANADA;
pub use europe::{
    ALBANIA, AUSTRIA, BELARUS, BELGIUM, BOSNIA_AND_HERZEGOVINA, BULGARIA, CROATIA, CYPRUS, CZECHIA,
    DENMARK, ESTONIA, FINLAND, FRANCE, GERMANY, GREECE, HUNGARY, ICELAND, IRELAND, ITALY, LATVIA,
    LIECHTENSTEIN, LITHUANIA, LUXEMBOURG, MALTA, MOLDOVA, MONACO, MONTENEGRO, NETHERLANDS,
    NORTH_MACEDONIA, NORWAY, POLAND, PORTUGAL, ROMANIA, RUSSIA, SAN_MARINO, SERBIA, SLOVAKIA,
    SLOVENIA, SWEDEN, UKRAINE, UNITED_KINGDOM, VATICAN_CITY,
};
pub use japan::JAPAN;
pub use mexico::MEXICO;
pub use oceania::{
    AUSTRALIA, FIJI, KIRIBATI, MARSHALL_ISLANDS, MICRONESIA, NAURU, NEW_ZEALAND, PALAU,
    PAPUA_NEW_GUINEA, SAMOA, SOLOMON_ISLANDS, TONGA, TUVALU, VANUATU,
};
pub use spain::SPAIN;
pub use switzerland::SWITZERLAND;
pub use united_states::UNITED_STATES;

/// Every country table in the crate, in ISO 3166-1 alpha-2 order. A
/// country's table is a [`RuleSet`] like any other: a country is not
/// special, it is just the rule set people ask for most.
pub static ALL: &[&RuleSet] = &[
    &ANDORRA,
    &UNITED_ARAB_EMIRATES,
    &AFGHANISTAN,
    &ANTIGUA_AND_BARBUDA,
    &ALBANIA,
    &ARMENIA,
    &ANGOLA,
    &ARGENTINA,
    &AUSTRIA,
    &AUSTRALIA,
    &AZERBAIJAN,
    &BOSNIA_AND_HERZEGOVINA,
    &BARBADOS,
    &BANGLADESH,
    &BELGIUM,
    &BURKINA_FASO,
    &BULGARIA,
    &BAHRAIN,
    &BURUNDI,
    &BENIN,
    &BRUNEI,
    &BOLIVIA,
    &BRAZIL,
    &BAHAMAS,
    &BHUTAN,
    &BOTSWANA,
    &BELARUS,
    &BELIZE,
    &CANADA,
    &DR_CONGO,
    &CONGO,
    &SWITZERLAND,
    &COTE_D_IVOIRE,
    &CHILE,
    &CAMEROON,
    &CHINA,
    &COLOMBIA,
    &COSTA_RICA,
    &CUBA,
    &CABO_VERDE,
    &CYPRUS,
    &CZECHIA,
    &GERMANY,
    &DJIBOUTI,
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
    &FIJI,
    &MICRONESIA,
    &FRANCE,
    &GABON,
    &UNITED_KINGDOM,
    &GRENADA,
    &GEORGIA,
    &GHANA,
    &GAMBIA,
    &GUINEA,
    &EQUATORIAL_GUINEA,
    &GREECE,
    &GUATEMALA,
    &GUINEA_BISSAU,
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
    &KIRIBATI,
    &COMOROS,
    &SAINT_KITTS_AND_NEVIS,
    &NORTH_KOREA,
    &SOUTH_KOREA,
    &KUWAIT,
    &KAZAKHSTAN,
    &LAOS,
    &LEBANON,
    &SAINT_LUCIA,
    &LIECHTENSTEIN,
    &SRI_LANKA,
    &LIBERIA,
    &LESOTHO,
    &LITHUANIA,
    &LUXEMBOURG,
    &LATVIA,
    &LIBYA,
    &MOROCCO,
    &MONACO,
    &MOLDOVA,
    &MONTENEGRO,
    &MADAGASCAR,
    &MARSHALL_ISLANDS,
    &NORTH_MACEDONIA,
    &MALI,
    &MYANMAR,
    &MONGOLIA,
    &MACAU,
    &MAURITANIA,
    &MALTA,
    &MAURITIUS,
    &MALDIVES,
    &MALAWI,
    &MEXICO,
    &MALAYSIA,
    &MOZAMBIQUE,
    &NAMIBIA,
    &NIGER,
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
    &RWANDA,
    &SAUDI_ARABIA,
    &SOLOMON_ISLANDS,
    &SEYCHELLES,
    &SUDAN,
    &SWEDEN,
    &SINGAPORE,
    &SLOVENIA,
    &SLOVAKIA,
    &SIERRA_LEONE,
    &SAN_MARINO,
    &SENEGAL,
    &SOMALIA,
    &SURINAME,
    &SOUTH_SUDAN,
    &EL_SALVADOR,
    &SYRIA,
    &ESWATINI,
    &CHAD,
    &TOGO,
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

/// The table for an ISO 3166-1 alpha-2 country code, by
/// [`hc_core::catalogue::matches`].
#[must_use]
pub fn by_code(code: &str) -> Option<&'static RuleSet> {
    ALL.iter()
        .copied()
        .find(|country| hc_core::catalogue::matches(code, country.code))
}

hc_core::catalogue_tests! {
    type: &'static RuleSet,
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
        // `README.md` states this figure, and a documented count that drifts
        // is a documented lie.
        assert_eq!(ALL.len(), 195);
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
        /// carries function pointers, and those cannot appear in a
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
