//! What this crate does not ship, and why.
//!
//! Name days are kept in some twenty countries and this crate vendors one
//! of them. A [`Gap`] is a country a caller might reasonably expect and
//! will not find, with the reason attached as a value rather than as prose
//! to grep — the same shape as `hc_attributes::gaps`, for the same reason:
//! a crate about who says so has to be able to say "nobody does", or "the
//! owner charges", as clearly as it says "here is the list".
//!
//! Three of the reasons are about licensing rather than about authority. A
//! list whose owner charges for it ([`GapReason::LicensedForAFee`]) or
//! reserves its rights ([`GapReason::RightsReserved`]) is a perfectly good
//! list; it is simply not this crate's to give away, and the loader in
//! [`crate::load`] is how a caller who holds a licence uses it. A list
//! nobody has stated terms for ([`GapReason::LicenceUnknown`]) is in the
//! same position until somebody does: the Nordic countries protect a
//! non-original list as a *catalogue* and the whole EU/EEA has the database
//! right of Directive 96/9/EC, so silence is not permission. The other
//! reasons say what the survey did not find: no keeper
//! ([`GapReason::NoKeeperFound`]), a keeper's list not read
//! ([`GapReason::NotYetRead`]), or a church calendar of saints
//! ([`GapReason::SaintsNotNames`]).
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
//! // Finland is a gap because of a fee, not because of doubt.
//! assert_eq!(for_reason(GapReason::LicensedForAFee).count(), 1);
//! assert!(ALL.iter().all(|gap| !gap.sources.is_empty()));
//! ```

/// The date the sources behind this table were read.
pub const SURVEYED: &str = "2026-10-04";

/// Why a country's list is absent.
///
/// Every reason is a statement about the sources read on [`SURVEYED`], and
/// each says whether it is a finding (a source says it) or the absence of
/// one (none was found); none is a choice about what the crate carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum GapReason {
    /// A named body owns the list and charges for its use.
    LicensedForAFee,
    /// A list with a named compiler exists and no statement of terms was
    /// found; nobody has said it may be copied.
    LicenceUnknown,
    /// A named owner states copyright in the list, or in the data it
    /// contains, and requires written permission; no free licence is
    /// stated and no fee schedule was found.
    RightsReserved,
    /// No body keeps a list that a page read names. Where a source says
    /// that none does, the row quotes it; where none was found, the row says
    /// that nothing was found and where it looked.
    NoKeeperFound,
    /// A keeper's list exists and the survey did not read it: a file in a
    /// format that was not opened, a page that refused to be read, or a copy
    /// that could not be fetched.
    NotYetRead,
    /// The authority behind the custom is a church calendar that names
    /// saints by day; which given names belong to which saint is custom
    /// with no published list, so there is no name list to carry.
    SaintsNotNames,
}

impl GapReason {
    /// A stable identifier, lower-case and hyphenated, which a boundary
    /// writes the reason as.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::LicensedForAFee => "licensed-for-a-fee",
            Self::LicenceUnknown => "licence-unknown",
            Self::RightsReserved => "rights-reserved",
            Self::NoKeeperFound => "no-keeper-found",
            Self::NotYetRead => "not-yet-read",
            Self::SaintsNotNames => "saints-not-names",
        }
    }

    /// A one-line description.
    #[must_use]
    pub const fn english_description(self) -> &'static str {
        match self {
            Self::LicensedForAFee => "the owner charges for the list",
            Self::LicenceUnknown => "no statement of terms was found",
            Self::RightsReserved => "the owner reserves its rights and requires permission",
            Self::NoKeeperFound => "no body that keeps the list is named by a page read",
            Self::NotYetRead => "the keeper's list exists and was not read",
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
    /// The first year the crate carries a list for, when the gap is the
    /// years before it and the country has a list shipped for later years;
    /// `None` when the gap is the country's every year.
    pub before: Option<i32>,
}

impl Gap {
    /// Whether the gap answers for `year`: every year of a country with no
    /// list, and the years before the first edition of one with a list.
    #[must_use]
    pub const fn covers(&self, year: i32) -> bool {
        match self.before {
            Some(first) => year < first,
            None => true,
        }
    }
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
            explanation: "The Bulgarian Orthodox Church's Holy Synod publishes a calendar of \
                 saints by day, and its footer permits use and citation with exact attribution; \
                 no church page read lists given names by day. The only full list read, Bulgarian \
                 Wikipedia's \"Имен ден\" (CC BY-SA 4.0), is a volunteer compilation that names \
                 the Patriarchate as an authority the Patriarchate does not state. The movable \
                 name days (Цветница on Palm Sunday, Великден, Тодоровден on the Saturday of the \
                 first week of Lent, Спасовден on Ascension) follow the Julian computus and are \
                 hc-holiday's table name-days-bulgarian-movable.",
            sources: "bg-patriarshia-calendar; bg.wikipedia, \"Имен ден\", \
                      https://bg.wikipedia.org/wiki/Имен_ден, read 2026-10-04",
            before: None,
        };
        /// Czechia's jmeniny.
        pub const CZECHIA = Gap {
            id: "cz",
            country: "Czechia",
            subject: "the Czech name-day calendar",
            reason: GapReason::NoKeeperFound,
            explanation: "No body keeps a Czech name-day list, and a source says so: the \
                 National Library's reference service states that no official calendar exists in \
                 the Czech Republic, that the names differ by publisher and are not assessed or \
                 approved, and that the calendarium most publishers use is the one in Miloslava \
                 Knappová's Jak se bude vaše dítě jmenovat? (2017), which the author calls not \
                 binding. Czech Wikipedia says the commission that coordinated the civil calendar \
                 ended after 1989, its last meeting with the Ministry of the Interior being in \
                 1996, and prints a full list (\"Jmeniny v Česku\") that names no compiler. Until a \
                 keeper exists there is no edition to name.",
            sources: "ptejteseknihovny-kalendarium; cs.wikipedia, \"Jmeniny v Česku\", \
                      https://cs.wikipedia.org/wiki/Jmeniny_v_Česku, read 2026-10-04",
            before: None,
        };
        /// Germany's and Austria's Namenstag.
        pub const GERMANY_AUSTRIA = Gap {
            id: "de-at",
            country: "Germany and Austria",
            subject: "a German-language name-day list",
            reason: GapReason::RightsReserved,
            explanation: "Neither country keeps a civil list. The Catholic Regionalkalender für \
                 das deutsche Sprachgebiet and the Martyrologium Romanum are liturgical \
                 calendars of saints and belong beside hc_holiday::roman_calendar. A readable \
                 database of saints and the names they carry is namenstage.katholisch.de, \
                 operated by PubliKath GmbH, whose legal notice says that any use of its content \
                 needs written consent; German Wikipedia's \"Liste der Namenstage\" (CC BY-SA \
                 4.0) is a secondary compilation from the Regionalkalender and the Martyrologium. \
                 The Orthodox and Evangelical calendars differ from them. A caller who holds \
                 written permission loads a list with hc_name_days::load.",
            sources: "dewiki-namenstage; https://namenstage.katholisch.de, read 2026-10-04",
            before: None,
        };
        /// Denmark's navnedag.
        pub const DENMARK = Gap {
            id: "dk",
            country: "Denmark",
            subject: "the Danish name-day list",
            reason: GapReason::NoKeeperFound,
            explanation: "No body that keeps a Danish name-day list was found. The University of \
                 Copenhagen's Almanak page does not mention name days, and Danish Wikipedia's \
                 \"Danske navnedage\", the only full list read, has no sources and names no \
                 compiler: it carries the old almanac sanctorale, mostly one saint's name per \
                 day, with 24 February empty and the names of 25-29 February shifting one day in \
                 a leap year. A claim of an official register under Copenhagen University is \
                 supported by no page read. Searched: Danish Wikipedia, science.ku.dk, \
                 opendata.dk (no dataset for \"navnedag\").",
            sources: "dawiki-navnedag; https://science.ku.dk/fakultetet/almanak, read 2026-10-04",
            before: None,
        };
        /// Estonia's nimepäev.
        pub const ESTONIA = Gap {
            id: "ee",
            country: "Estonia",
            subject: "the Estonian name-day list",
            reason: GapReason::LicenceUnknown,
            explanation: "Statistics Estonia's \"Nimepäevad\" prints a full name-day list by date \
                 and credits it to a commercial book, Piret Mäeniit's Eesti Nimed (Mytho \
                 Kirjastus, 2011); the page states no terms, and the rights in the book are \
                 unknown (publishing a commercial book's list does not make it an official work). \
                 No Estonian keeper was found: commercial sites claim that the University of \
                 Tartu's institute keeps an official calendar revised every five years, and no \
                 university page confirms it.",
            sources: "stat-ee-nimepaevad; https://www.stat.ee/nimed/NIMEPAEVAD; \
                      et.wikipedia, \"Nimepäev\", https://et.wikipedia.org/wiki/Nimepäev, \
                      read 2026-10-04",
            before: None,
        };
        /// Spain's santo.
        pub const SPAIN = Gap {
            id: "es",
            country: "Spain",
            subject: "a Spanish name-day list",
            reason: GapReason::SaintsNotNames,
            explanation: "The Conferencia Episcopal Española publishes the liturgical calendar \
                 (Calendario Litúrgico-Pastoral, a PDF that was not opened, with no terms \
                 stated), which names saints per the Calendarium Romanum and the Calendario \
                 Propio de España. No page read shows a Spanish list of given names by day or a \
                 body that keeps one.",
            sources: "Conferencia Episcopal Española, Calendario Litúrgico-Pastoral; \
                      es.wikipedia, \"Santoral\", read 2026-10-04",
            before: None,
        };
        /// Finland's nimipäivä, in its four lists.
        pub const FINLAND = Gap {
            id: "fi",
            country: "Finland",
            subject: "the Finnish, Finland-Swedish, Orthodox and Sámi name-day lists",
            reason: GapReason::LicensedForAFee,
            explanation: "The University of Helsinki's Almanac Office holds the copyright to the \
                 Finnish and Finland-Swedish lists, confirmed by the Supreme Court in 2000 (KKO \
                 2000:56, catalogue protection under Tekijänoikeuslaki 49 §), and licenses the \
                 lists of 2025 to 2029 for a fee. Name-day information may be published freely \
                 only if no more than two weeks of it, or no more than 15 names from the \
                 alphabetical list, are published at a time; publishing the whole year, as a \
                 library would, is subject to a royalty per copy. The Office also supplies the \
                 Orthodox and Sámi lists and its price list says no copyright fee is charged for \
                 them; no page read says whether a third party may redistribute them, and the \
                 Finnish Orthodox Church, the Orthodox list's keeper, was not asked. A caller \
                 who holds a licence loads a list with hc_name_days::load.",
            sources: "helsinki-copyright; helsinki-pricing; kko-2000-56; read 2026-10-04",
            before: None,
        };
        /// France's fête.
        pub const FRANCE = Gap {
            id: "fr",
            country: "France",
            subject: "the French calendrier des postes and Nominis's prénoms",
            reason: GapReason::RightsReserved,
            explanation: "The Church's reference, Nominis, a site affiliated with the \
                 Conférence des évêques de France, states in its legal notice that all of its \
                 contents, including its data on first names and its database, are protected by \
                 copyright and may not be reproduced, even in part, without prior written \
                 authorisation. The postal calendar is a publishers' compilation with a \
                 selection of saints supplied by the Archdiocese of Paris; French Wikipedia \
                 shows its edition of 2011, without sources, and no list with a licence was \
                 found. The General Roman Calendar behind both is already in \
                 hc_holiday::roman_calendar.",
            sources: "frwiki-fleuristes; https://nominis.cef.fr, legal notice, read 2026-10-04",
            before: None,
        };
        /// Greece's ονομαστική εορτή.
        pub const GREECE = Gap {
            id: "gr",
            country: "Greece",
            subject: "the Greek name days, fixed and movable",
            reason: GapReason::RightsReserved,
            explanation: "The Church of Greece publishes no list of given names; its calendar \
                 names saints. A full list for 2026 is on eortologio.gr, compiled by lay \
                 Orthodox Christians from the synaxaria, a hagiography and regional Church \
                 publications and not an official church document, and its terms say that no \
                 part may be reproduced or transmitted without written permission. The movable \
                 rules are citable — Thomas Sunday (Pascha + 7) for Θωμάς and Θωμαΐς, All Saints \
                 (Pascha + 56) for names with no saint of their own, and St George moved to \
                 Easter Monday when 23 April falls before Pascha — and are hc-holiday's table \
                 name-days-greek-movable.",
            sources: "elwiki-eortologio; https://eortologio.gr/assist/about_gr.php, read 2026-10-04",
            before: None,
        };
        /// Croatia's imendan.
        pub const CROATIA = Gap {
            id: "hr",
            country: "Croatia",
            subject: "a Croatian name-day calendar",
            reason: GapReason::NoKeeperFound,
            explanation: "No body that keeps a Croatian name-day list is named by a page read. \
                 The Croatian Bishops' Conference publishes the liturgical calendar (approved in \
                 2008), which names saints and not given names; Croatian Wikipedia's \"Imendan\" \
                 names no body and prints no list. The one compiled imendanski kalendar found is \
                 the Bishops' Conference of Bosnia and Herzegovina's, a PDF for its own territory \
                 that was not opened, with no terms stated. Searched: hbk.hr, Croatian Wikipedia, \
                 data.gov.hr (no dataset for \"imendan\").",
            sources: "https://hbk.hr/nacionalni-liturgijski-kalendar; ktabkbih-imendanski; \
                      read 2026-10-04",
            before: None,
        };
        /// Hungary's névnap.
        pub const HUNGARY = Gap {
            id: "hu",
            country: "Hungary",
            subject: "the Hungarian name-day calendar",
            reason: GapReason::NoKeeperFound,
            explanation: "No body assigns Hungarian name days, and a source says so: Hungarian \
                 Wikipedia states that there is no central body and that the name days in \
                 calendars are not official. The registrable-name list of the HUN-REN \
                 Nyelvtudományi Kutatóközpont is official and carries no name days. The one full \
                 list read, Hungarian Wikipedia's \"Magyar névnapok listája dátum szerint\" \
                 (CC BY-SA 4.0), rests partly on a 2008 wall calendar that the page itself marks \
                 as unreliable; the earlier claim that the printed tradition rests on Ladó and \
                 Bíró's Magyar utónévkönyv (1998) was not re-read. The convention leaves 24 \
                 February empty in a leap year and moves the names of 24-28 February one day \
                 later.",
            sources: "huwiki-nevnap; https://archive.nytud.hu, utónevek; https://hun-ren.hu, 2023 \
                      news; read 2026-10-04",
            before: None,
        };
        /// Lithuania's vardadienis.
        pub const LITHUANIA = Gap {
            id: "lt",
            country: "Lithuania",
            subject: "the Lithuanian name-day list",
            reason: GapReason::NotYetRead,
            explanation: "The State Commission of the Lithuanian Language, the one possible \
                 keeper, refused its pages (HTTP 403, twice), so whether it keeps a list was not \
                 read. Lithuanian Wikipedia's \"Vardadienis\" gives no list and says only that \
                 name days come from church calendars, the Catholic and the Orthodox differing. \
                 No Lithuanian list with a stated keeper or licence was found; data.lt has no \
                 dataset for \"vardadienis\".",
            sources: "ltwiki-vardadienis; https://vlkk.lt (HTTP 403 when read 2026-10-04)",
            before: None,
        };
        /// Latvia's earlier editions.
        pub const LATVIA_EARLIER = Gap {
            id: "lv",
            country: "Latvia",
            subject: "the Latvian lists before the first edition carried",
            reason: GapReason::NotYetRead,
            explanation: "The Kalendārvārdu ekspertu komisija has decided additions in 1997, on \
                 26 March 2003, in 2011, about 22 May 2014, on 12 April 2018 and on 16 March 2022 \
                 (the gazette dates the decisions of 2022 to 17 March 2022, in force that day), \
                 and the lists in force before 2023 are not carried. The decisions list the names \
                 they add and not the whole list day by day; data.gov.lv's files were replaced in \
                 place and show no version history; the files of 26 April 2023 survive in \
                 Wayback Machine captures that the survey's tools could not fetch. The first \
                 whole year of the 2022 edition is 2023, and 2022 itself is a gap, since two \
                 editions overlap in it.",
            sources: "vvc-2022; https://likumi.lv/ta/id/330954; https://www.vestnesis.lv/ta/id/73773; \
                      https://data.gov.lv/dati/eng/dataset/latviesu-tradicionalais-un-paplasinatais-kalendarvardu-saraksts; \
                      read 2026-10-04",
            before: Some(2023),
        };
        /// Norway's navnedag.
        pub const NORWAY = Gap {
            id: "no",
            country: "Norway",
            subject: "the Norwegian name-day list",
            reason: GapReason::RightsReserved,
            explanation: "Almanakkforlaget, a publisher, states that its name-day calendar is \
                 protected under the copyright act; its page, read 2026-10-04, prints no list and \
                 no fee or terms, and no list with a stated licence was found. A caller who holds \
                 written permission loads the list with hc_name_days::load.",
            sources: "almanakkforlaget-navnedager; no.wikipedia, \"Navnedag\", \
                      https://no.wikipedia.org/wiki/Navnedag, read 2026-10-04",
            before: None,
        };
        /// Poland's imieniny.
        pub const POLAND = Gap {
            id: "pl",
            country: "Poland",
            subject: "the Polish name-day calendar",
            reason: GapReason::NoKeeperFound,
            explanation: "No Polish body that keeps a name-day calendar was found: the Rada \
                 Języka Polskiego's page and Polish Wikipedia's \"Imieniny\" name none and cite \
                 no source, and Wikipedia's month tables (CC BY-SA) carry no source. Article 4 \
                 of the Prawo autorskie, on official documents, was not read. The Catholic \
                 liturgical calendar behind the custom is already in hc_holiday::roman_calendar.",
            sources: "plwiki-imieniny; https://rjp.pan.pl; https://dane.gov.pl (no dataset), \
                      read 2026-10-04",
            before: None,
        };
        /// Russia's именины.
        pub const RUSSIA = Gap {
            id: "ru",
            country: "Russia",
            subject: "the Russian name days",
            reason: GapReason::SaintsNotNames,
            explanation: "A name day is the commemoration of the saint of the same name nearest \
                 after one's birthday, read from the Church's Месяцеслов on the Julian calendar. \
                 A name maps to many dates and the rule needs the caller's birthday rather than \
                 a list. The Месяцеслов's terms were not read: azbyka.ru refused the page (HTTP \
                 403) and patriarchia.ru showed no calendar page.",
            sources: "ruwiki-imeniny; https://azbyka.ru/imeniny (HTTP 403 when read 2026-10-04)",
            before: None,
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
                 list of their own. The list, the namnlängd of 2001 as amended (six names from \
                 2027), is readable as HTML on the Academy's and the Institute's sites, but not \
                 in date order. The Academy's pages carry \"© Svenska Akademien\" site-wide and \
                 the Institute's allow texts on its site to be copied with the source credited; \
                 neither states terms for the list itself, and Swedish catalogue protection can \
                 apply to a compiled list. The list is not carried until the committee is asked.",
            sources: "svenska-akademien-namnlangden; isof-namnsdagar; riksdagen-kru4 (1993/94:KrU4); \
                      data.riksdagen.se, 2006/07:Kr238; read 2026-10-04",
            before: None,
        };
        /// Slovakia's meniny.
        pub const SLOVAKIA = Gap {
            id: "sk",
            country: "Slovakia",
            subject: "the Oficiálne kalendárium of the Ministry of Culture",
            reason: GapReason::NotYetRead,
            explanation: "The Kalendárová komisia of the Ministry of Culture keeps an official \
                 calendarium, recommendatory for publishers (the list approved in 2017, the 2025 \
                 file), and publishes it as a PDF, which was not opened; the ministry states no \
                 terms. Section 5 of Zákon č. 185/2015 Z. z., read only through a secondary page, \
                 excludes legal texts, official and court decisions and technical norms, none of \
                 which is a recommendation; the section itself was not read.",
            sources: "culture-sk-kalendarium; https://www.culture.gov.sk/sk/kalendarova-komisia, \
                      read 2026-10-04",
            before: None,
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
        // Seventeen rows for the countries with no list, which cover eighteen
        // countries (Germany and Austria share one), and one for the years
        // before the first Latvian edition.
        assert_eq!(ALL.len(), 18);
        assert_eq!(ALL.iter().filter(|gap| gap.before.is_none()).count(), 17);
        let countries: usize = ALL
            .iter()
            .filter(|gap| gap.before.is_none())
            .map(|gap| gap.id.split('-').count())
            .sum();
        assert_eq!(countries, 18);
    }

    #[test]
    fn the_rows_that_ask_for_a_licence_point_the_caller_at_the_loader() {
        for id in ["fi", "no", "de-at"] {
            let gap = by_id(id).unwrap();
            assert!(gap.explanation.contains("hc_name_days::load"), "{id}");
        }
        assert_eq!(for_reason(GapReason::LicensedForAFee).count(), 1);
        assert_eq!(FINLAND.reason, GapReason::LicensedForAFee);
        assert!(FINLAND.explanation.contains("two weeks"));
        assert!(FINLAND.explanation.contains("15 names"));
        assert_eq!(NORWAY.reason, GapReason::RightsReserved);
    }

    #[test]
    fn the_reasons_say_what_was_found_and_what_was_not() {
        let ids = |reason| {
            let mut ids = [""; 18];
            let mut count = 0;
            for gap in for_reason(reason) {
                ids[count] = gap.id;
                count += 1;
            }
            (ids, count)
        };
        let (found, count) = ids(GapReason::NoKeeperFound);
        assert_eq!(&found[..count], ["cz", "dk", "hr", "hu", "pl"]);
        let (unread, count) = ids(GapReason::NotYetRead);
        assert_eq!(&unread[..count], ["lt", "lv", "sk"]);
        let (reserved, count) = ids(GapReason::RightsReserved);
        assert_eq!(&reserved[..count], ["de-at", "fr", "gr", "no"]);
        let (unknown, count) = ids(GapReason::LicenceUnknown);
        assert_eq!(&unknown[..count], ["ee", "se"]);
        // No reason is written as a design choice: each row says it is the
        // sources read that are silent or closed, and names none as excluded.
        for gap in ALL {
            assert!(!gap.explanation.contains("design"), "{}", gap.id);
            assert!(gap.sources.contains("2026-10-04") || gap.reason != GapReason::NoKeeperFound);
        }
    }

    #[test]
    fn a_gap_of_the_years_before_an_edition_covers_exactly_those_years() {
        assert_eq!(LATVIA_EARLIER.before, Some(2023));
        assert!(LATVIA_EARLIER.covers(2022) && LATVIA_EARLIER.covers(1997));
        assert!(!LATVIA_EARLIER.covers(2023));
        assert!(FINLAND.covers(1) && FINLAND.covers(2026));
    }

    #[test]
    fn the_orthodox_countries_record_the_movable_rules_they_carry() {
        assert!(GREECE.explanation.contains("Pascha + 56"));
        assert!(GREECE.explanation.contains("name-days-greek-movable"));
        assert!(BULGARIA.explanation.contains("name-days-bulgarian-movable"));
        assert_eq!(for_reason(GapReason::SaintsNotNames).count(), 3);
    }

    /// Nothing in `gaps` may quietly become a shipped list without the gap
    /// being removed.
    #[test]
    fn no_gap_names_a_country_that_has_a_shipped_list_for_the_same_years() {
        for gap in ALL {
            for list in crate::ALL {
                if gap.id != list.country {
                    continue;
                }
                // Only a gap of the years before the first edition may share
                // the country, and no edition may reach into it.
                let first = gap.before.expect("a partial gap");
                assert!(
                    list.validity.from.is_some_and(|from| from >= first),
                    "{} is both a gap and a list for {}",
                    gap.id,
                    list.id
                );
            }
        }
    }

    #[test]
    fn every_reason_has_a_distinct_kebab_identifier() {
        let reasons = [
            GapReason::LicensedForAFee,
            GapReason::LicenceUnknown,
            GapReason::RightsReserved,
            GapReason::NoKeeperFound,
            GapReason::NotYetRead,
            GapReason::SaintsNotNames,
        ];
        for (index, reason) in reasons.iter().enumerate() {
            let id = reason.id();
            assert!(
                id.bytes().all(|b| b.is_ascii_lowercase() || b == b'-'),
                "{id}"
            );
            for other in &reasons[index + 1..] {
                assert_ne!(id, other.id());
            }
        }
        assert_eq!(GapReason::LicensedForAFee.id(), "licensed-for-a-fee");
    }
}
