//! What this crate deliberately does not ship, and why.
//!
//! Name days are kept in some twenty countries and this crate vendors one
//! of them. A [`Gap`] is a country a caller might reasonably expect and
//! will not find, with the reason attached as a value rather than as prose
//! to grep — the same shape as `hc_attributes::gaps`, for the same reason:
//! a crate about who says so has to be able to say "nobody does", or "the
//! owner charges", as clearly as it says "here is the list".
//!
//! Two of the reasons are about licensing rather than about authority. A
//! list whose owner charges for it ([`GapReason::LicensedForAFee`]) is a
//! perfectly good list; it is simply not this crate's to give away, and
//! the loader in [`crate::load`] is how a caller who holds a licence uses
//! it. A list nobody has stated terms for ([`GapReason::LicenceUnknown`])
//! is in the same position until somebody does: the Nordic countries
//! protect a non-original list as a *catalogue* and the whole EU/EEA has
//! the database right of Directive 96/9/EC, so silence is not permission.
//!
//! Every gap is present tense: it describes the sources as they were read
//! on the date in [`SURVEYED`], and a country leaves the table when its
//! entry in the table of lists arrives. A `sources` string names each
//! source by its `docs/references.bib` key where it has one and by URL
//! where it does not; the survey and its sources are described in
//! `docs/systems/name-days.md`.
//!
//! ```
//! use hc_name_days::gaps::{ALL, GapReason, for_reason};
//!
//! // Finland and Norway are gaps because of a fee, not because of doubt.
//! assert_eq!(for_reason(GapReason::LicensedForAFee).count(), 2);
//! assert!(ALL.iter().all(|gap| !gap.sources.is_empty()));
//! ```

/// The date the sources behind this table were read.
pub const SURVEYED: &str = "2026-09-25";

/// Why a country's list is absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum GapReason {
    /// A named body owns the list and charges for its use.
    LicensedForAFee,
    /// A list with a named compiler exists and no statement of terms was
    /// found; nobody has said it may be copied.
    LicenceUnknown,
    /// Several published lists exist, they disagree, and no body chooses
    /// between them.
    SourcesDisagreeWithNoAuthority,
    /// The widely printed list rests on a publication whose compiler has
    /// not published the method, or on a claim of an authority that could
    /// not be confirmed.
    MethodUnpublished,
    /// The authority behind the custom is a church calendar that names
    /// saints by day; which given names belong to which saint is custom
    /// with no published list, so there is no name list to carry.
    SaintsNotNames,
}

impl GapReason {
    /// A one-line description.
    #[must_use]
    pub const fn english_description(self) -> &'static str {
        match self {
            Self::LicensedForAFee => "the owner charges for the list",
            Self::LicenceUnknown => "no statement of terms was found",
            Self::SourcesDisagreeWithNoAuthority => {
                "several published lists disagree and none is authoritative"
            }
            Self::MethodUnpublished => "the compiler has not published the method",
            Self::SaintsNotNames => {
                "the church calendar names saints, not given names; the mapping has no authority"
            }
        }
    }
}

/// A country the crate declined to ship, and the reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gap {
    /// A stable identifier: the country code, or two joined by a hyphen.
    pub id: &'static str,
    /// The country, in English.
    pub country: &'static str,
    /// What is missing.
    pub subject: &'static str,
    /// Why.
    pub reason: GapReason,
    /// The reasoning in full, present tense.
    pub explanation: &'static str,
    /// What was consulted before declining.
    pub sources: &'static str,
}

hc_core::catalogue! {
    type: Gap,
    id: |gap| gap.id,
    sorted_by: |gap| gap.id,
    provenance: |gap| gap.sources,
    tests: gap_catalogue_tests,

    /// Every gap this crate records, by identifier.
    pub const ALL;
    /// The gap with this identifier.
    pub fn by_id;

    entries: {
        /// Bulgaria's имен ден.
        pub const BULGARIA = Gap {
            id: "bg",
            country: "Bulgaria",
            subject: "the Bulgarian name days, fixed and movable",
            reason: GapReason::SaintsNotNames,
            explanation: "The Bulgarian Orthodox Church's calendar names saints by date, and its \
                 footer permits use and citation with exact attribution; which given names belong \
                 to which saint is custom, and no church-published name list exists. The movable \
                 name days (Цветница on Palm Sunday, Великден, Тодоровден on the Saturday of the \
                 first week of Lent, Спасовден on Ascension) follow the Julian computus and are \
                 not yet carried.",
            sources: "bg-patriarshia-calendar; bg.wikipedia, \"Имен ден\", \
                      https://bg.wikipedia.org/wiki/Имен_ден",
        };
        /// Czechia's jmeniny.
        pub const CZECHIA = Gap {
            id: "cz",
            country: "Czechia",
            subject: "the Czech name-day calendar",
            reason: GapReason::SourcesDisagreeWithNoAuthority,
            explanation: "The National Library's reference service states that no official \
                 calendar exists in the Czech Republic, that the content differs by publisher and \
                 is not codified, and that the widely used calendarium is the one in Miloslava \
                 Knappová's Jak se bude vaše dítě jmenovat? (2017), which is not binding. A \
                 copyrighted book with no binding force is not an authority to name.",
            sources: "ptejteseknihovny-kalendarium; cs.wikipedia, \"Jmeniny v Česku\", \
                      https://cs.wikipedia.org/wiki/Jmeniny_v_Česku",
        };
        /// Germany's and Austria's Namenstag.
        pub const GERMANY_AUSTRIA = Gap {
            id: "de-at",
            country: "Germany and Austria",
            subject: "a German-language name-day list",
            reason: GapReason::SaintsNotNames,
            explanation: "No civil list exists. The lists in circulation are drawn from the \
                 Regionalkalender für das deutsche Sprachgebiet and the Martyrologium Romanum, \
                 and the Orthodox and Evangelical calendars differ from them. The Regionalkalender \
                 is a liturgical calendar and belongs beside hc_holiday::roman_calendar, not here.",
            sources: "dewiki-namenstage; https://namenstage.katholisch.de",
        };
        /// Denmark's navnedag.
        pub const DENMARK = Gap {
            id: "dk",
            country: "Denmark",
            subject: "the Danish name-day list",
            reason: GapReason::SourcesDisagreeWithNoAuthority,
            explanation: "No body keeps a list. The list in circulation is the old almanac \
                 sanctorale, mostly one saint's name per day, with 24 February empty and the names \
                 of 25-29 February shifting one day in a leap year; the Danish Wikipedia article \
                 carrying it is marked as having no sources, and the Copenhagen University Almanac \
                 does not mention name days on its contents page. A claim of an official register \
                 under Copenhagen University is unsupported by any page read.",
            sources: "dawiki-navnedag; https://science.ku.dk/fakultetet/almanak",
        };
        /// Estonia's nimepäev.
        pub const ESTONIA = Gap {
            id: "ee",
            country: "Estonia",
            subject: "the Estonian name-day list",
            reason: GapReason::MethodUnpublished,
            explanation: "Statistics Estonia's name-day page cites a commercial book, Piret \
                 Mäeniit's Eesti Nimed (2011), as its source. Commercial sites claim the University \
                 of Tartu's institute of Estonian and general linguistics keeps an official calendar \
                 revised every five years; no university page confirming this was found.",
            sources: "stat-ee-nimepaevad; et.wikipedia, \"Nimepäev\", \
                      https://et.wikipedia.org/wiki/Nimepäev",
        };
        /// Spain's santo.
        pub const SPAIN = Gap {
            id: "es",
            country: "Spain",
            subject: "a Spanish name-day list",
            reason: GapReason::SaintsNotNames,
            explanation: "The santoral is the liturgical calendar of the Conferencia Episcopal \
                 Española, per the Calendarium Romanum and the Calendario Propio de España. There \
                 is no name-day list distinct from it.",
            sources: "Conferencia Episcopal Española, Calendario Litúrgico-Pastoral",
        };
        /// Finland's nimipäivä, in its four lists.
        pub const FINLAND = Gap {
            id: "fi",
            country: "Finland",
            subject: "the Finnish, Finland-Swedish, Orthodox and Sámi name-day lists",
            reason: GapReason::LicensedForAFee,
            explanation: "The University of Helsinki holds the copyright to its name-day lists, \
                 confirmed by the Supreme Court in 2000 (KKO 2000:56, catalogue protection under \
                 Tekijänoikeuslaki 49 §). Name-day information may be published freely only if no \
                 more than two weeks of it, or no more than 15 names from the alphabetical list, \
                 are published at a time; publishing the whole year, as a library would, is always \
                 subject to a royalty per copy. A caller who holds a licence loads the list with \
                 hc_name_days::load.",
            sources: "helsinki-copyright; helsinki-pricing; kko-2000-56",
        };
        /// France's fête.
        pub const FRANCE = Gap {
            id: "fr",
            country: "France",
            subject: "the French calendrier des postes",
            reason: GapReason::LicenceUnknown,
            explanation: "The postal calendar is a publishers' compilation with a selection of \
                 saints supplied by the Archdiocese of Paris, and its terms are not stated. The \
                 Church's own reference, Nominis of the Conférence des évêques de France, publishes \
                 calendars for download and its terms were not retrieved. The General Roman \
                 Calendar behind both is already in hc_holiday::roman_calendar.",
            sources: "frwiki-fleuristes; https://nominis.cef.fr",
        };
        /// Greece's ονομαστική εορτή.
        pub const GREECE = Gap {
            id: "gr",
            country: "Greece",
            subject: "the Greek name days, fixed and movable",
            reason: GapReason::SaintsNotNames,
            explanation: "No list published by the Church of Greece was found; the compilations in \
                 use are private sites that assert copyright. The movable rules are citable — Thomas \
                 Sunday (Pascha + 7) for Θωμάς and Θωμαΐς, All Saints (Pascha + 56) for names with no \
                 saint of their own, and St George moved to Easter Monday when 23 April falls before \
                 Pascha — and are not yet carried.",
            sources: "elwiki-eortologio; https://eortologio.gr/assist/about_gr.php",
        };
        /// Croatia's imendan.
        pub const CROATIA = Gap {
            id: "hr",
            country: "Croatia",
            subject: "a Croatian name-day calendar",
            reason: GapReason::LicenceUnknown,
            explanation: "The Croatian bishops' conference publishes a liturgical calendar, not a \
                 name-day list. The only compiled imendanski kalendar found is the Bishops' \
                 Conference of Bosnia and Herzegovina's, issued for its own territory, with no terms \
                 stated and a publisher's notice of all rights reserved.",
            sources: "https://hbk.hr/nacionalni-liturgijski-kalendar; ktabkbih-imendanski",
        };
        /// Hungary's névnap.
        pub const HUNGARY = Gap {
            id: "hu",
            country: "Hungary",
            subject: "the Hungarian name-day calendar",
            reason: GapReason::MethodUnpublished,
            explanation: "No central body assigns name days and the name days printed in calendars \
                 are not official. The registrable-name list of the HUN-REN Nyelvtudományi \
                 Kutatóközpont is official and carries no name days; the printed tradition rests on \
                 Ladó and Bíró's Magyar utónévkönyv (1998), a copyrighted book. The convention \
                 leaves 24 February empty in a leap year and moves the names of 24-28 February one \
                 day later.",
            sources: "huwiki-nevnap; https://archive.nytud.hu, utónevek; https://hun-ren.hu, 2023 news",
        };
        /// Lithuania's vardadienis.
        pub const LITHUANIA = Gap {
            id: "lt",
            country: "Lithuania",
            subject: "the Lithuanian name-day list",
            reason: GapReason::SourcesDisagreeWithNoAuthority,
            explanation: "No authority was found: the State Commission of the Lithuanian Language's \
                 consultation pages could not be retrieved, and the encyclopaedic account is that \
                 name days come from church calendars, with the Catholic and Orthodox saints' \
                 calendars differing.",
            sources: "ltwiki-vardadienis; https://vlkk.lt (HTTP 403 when read)",
        };
        /// Norway's navnedag.
        pub const NORWAY = Gap {
            id: "no",
            country: "Norway",
            subject: "the Norwegian name-day list",
            reason: GapReason::LicensedForAFee,
            explanation: "Almanakkforlaget states that it is the producer, publisher and owner of \
                 the name-day sequence, that editorial use in the press is free with the publisher \
                 credited as source, that commercial use requires contacting it for terms, and that \
                 the calendar is protected under the copyright act. A caller who holds a licence \
                 loads the list with hc_name_days::load.",
            sources: "almanakkforlaget-navnedager; no.wikipedia, \"Navnedag\", \
                      https://no.wikipedia.org/wiki/Navnedag",
        };
        /// Poland's imieniny.
        pub const POLAND = Gap {
            id: "pl",
            country: "Poland",
            subject: "the Polish name-day calendar",
            reason: GapReason::SourcesDisagreeWithNoAuthority,
            explanation: "No body keeps a list; publishers differ and state that the dates are not \
                 regulated. The Catholic liturgical calendar behind the custom is already in \
                 hc_holiday::roman_calendar.",
            sources: "plwiki-imieniny",
        };
        /// Russia's именины.
        pub const RUSSIA = Gap {
            id: "ru",
            country: "Russia",
            subject: "the Russian name days",
            reason: GapReason::SaintsNotNames,
            explanation: "A name day is the commemoration of the saint of the same name nearest \
                 after one's birthday, read from the Church's Месяцеслов on the Julian calendar. A \
                 name maps to many dates, the rule needs the caller's birthday rather than a list, \
                 and a name-to-dates table would transcribe a copyrighted Patriarchate calendar.",
            sources: "ruwiki-imeniny; https://azbyka.ru/imeniny",
        };
        /// Sweden's namnsdag.
        pub const SWEDEN = Gap {
            id: "se",
            country: "Sweden",
            subject: "the Swedish namnsdagslängd",
            reason: GapReason::LicenceUnknown,
            explanation: "The Namnlängdskommittén of the Swedish Academy, the Royal Academies and \
                 the Institute for Language and Folklore keeps the list, which has had no official \
                 status since 1972, when the Riksdag left anyone free to publish an almanac with a \
                 list of their own. Neither the Academy's page nor the Institute's asserts or waives \
                 copyright, and Swedish catalogue protection can apply to a compiled list; nobody \
                 has said whether it does. The list is not carried until the committee is asked.",
            sources: "svenska-akademien-namnlangden; isof-namnsdagar; riksdagen-kru4 (1993/94:KrU4); \
                      data.riksdagen.se, 2006/07:Kr238",
        };
        /// Slovakia's meniny.
        pub const SLOVAKIA = Gap {
            id: "sk",
            country: "Slovakia",
            subject: "the Oficiálne kalendárium of the Ministry of Culture",
            reason: GapReason::LicenceUnknown,
            explanation: "The Kalendárová komisia of the Ministry of Culture publishes an official \
                 calendarium, recommendatory for publishers, as a PDF. The ministry states no terms. \
                 Slovak copyright law excludes official works from protection, and whether the \
                 calendarium is one under Zákon č. 185/2015 Z. z. § 5 has not been checked; the \
                 list is not carried until it is.",
            sources: "culture-sk-kalendarium; https://www.culture.gov.sk/sk/kalendarova-komisia",
        };
    }
}

/// Every gap recorded for a given reason.
pub fn for_reason(reason: GapReason) -> impl Iterator<Item = &'static Gap> {
    ALL.iter().filter(move |gap| gap.reason == reason)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_gap_names_a_country_a_subject_an_explanation_and_its_sources() {
        for gap in ALL {
            assert!(!gap.id.is_empty());
            assert!(!gap.country.is_empty(), "{}", gap.id);
            assert!(!gap.subject.is_empty(), "{}", gap.id);
            assert!(gap.explanation.len() > 80, "{}", gap.id);
            assert!(!gap.sources.is_empty(), "{}", gap.id);
            assert!(!gap.reason.english_description().is_empty());
        }
        assert_eq!(ALL.len(), 17);
    }

    #[test]
    fn the_two_fee_gaps_point_the_caller_at_the_loader() {
        assert_eq!(for_reason(GapReason::LicensedForAFee).count(), 2);
        for gap in for_reason(GapReason::LicensedForAFee) {
            assert!(gap.explanation.contains("hc_name_days::load"), "{}", gap.id);
        }
        assert_eq!(FINLAND.reason, GapReason::LicensedForAFee);
        assert!(FINLAND.explanation.contains("two weeks"));
        assert!(FINLAND.explanation.contains("15 names"));
        assert_eq!(NORWAY.reason, GapReason::LicensedForAFee);
    }

    #[test]
    fn the_unknown_licence_gaps_are_sweden_slovakia_croatia_and_france() {
        let ids: [&str; 4] = ["fr", "hr", "se", "sk"];
        assert!(
            for_reason(GapReason::LicenceUnknown)
                .map(|gap| gap.id)
                .eq(ids)
        );
    }

    #[test]
    fn the_orthodox_countries_record_the_movable_rules_they_do_not_yet_carry() {
        assert!(GREECE.explanation.contains("Pascha + 56"));
        assert!(GREECE.explanation.contains("not yet carried"));
        assert!(BULGARIA.explanation.contains("not yet carried"));
        assert_eq!(for_reason(GapReason::SaintsNotNames).count(), 5);
    }

    /// Nothing in `gaps` may quietly become a shipped list without the gap
    /// being removed.
    #[test]
    fn no_gap_names_a_country_that_has_a_shipped_list() {
        for gap in ALL {
            for list in crate::ALL {
                assert_ne!(gap.id, list.country, "{} is both a gap and a list", gap.id);
            }
        }
    }
}
