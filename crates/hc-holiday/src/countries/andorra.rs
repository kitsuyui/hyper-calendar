//! Andorra — the national days of the Government's yearly calendar, and
//! the parishes' own.
//!
//! The parishes' days are written up in `docs/systems/andorra-holidays.md`
//! in the repository. The Government's decree on each year's work calendar
//! lets each comú fix up to four parish days, which it publishes in the
//! BOPA for that year alone; the table carries the days each comú's
//! instrument keeps in the whole parish, as a [`Listing`] per parish for the
//! years read, 2024 to 2026, and a gap in any other year. The days a comú keeps in one village or quarter
//! are not carried: they need a scope finer than a parish.

use super::read_all;
use super::weekends;
use crate::computus::offsets::{EASTER_MONDAY, GOOD_FRIDAY, SHROVE_MONDAY, WHIT_MONDAY};
use crate::rule::{HolidayRule, Listing, Rule, RuleSet, SourceDate, Subdivisions};

/// The comuns' instruments read, 2024 to 2026, one for each parish.
const CANILLO: &str = "Comú de Canillo, avisos pel qual es fa públic el calendari de dies festius \
     de la parròquia de Canillo, of 30 November 2023, 10 October 2024 and 4 December 2025, BOPA";
const LA_MASSANA: &str = "Comú de la Massana, decrets pel qual s'aprova el calendari de dies \
     festius, of 30 November 2023, 13 November 2024 and 13 November 2025, BOPA";
const ORDINO: &str = "Comú d'Ordino, decrets pel qual es publiquen les festes obligatòries a la \
     parròquia d'Ordino, of 22 December 2023, 28 November 2024 and 27 November 2025, BOPA";
const SANT_JULIA: &str = "Comú de Sant Julià de Lòria, decrets pel qual es fixa el calendari de dies \
     festius de la parròquia, of 6 December 2023, 12 December 2024 and 11 December 2025, BOPA";
const ANDORRA_LA_VELLA: &str = "Comú d'Andorra la Vella, edictes pel qual es fa públic el calendari \
     de dies festius de la parròquia, of 15 November 2023, 19 November 2024 and 25 November \
     2025, BOPA";
const ENCAMP: &str = "Comú d'Encamp, avisos of 15 December 2023, 25 November 2024 and 21 November \
     2025, BOPA: no day in the whole parish, the festa del poble and Sant Roc being Encamp \
     village's and Sant Pere Pas de la Casa's";
const ESCALDES: &str = "Comú d'Escaldes-Engordany, decrets pel qual es fixen els dies festius \
     parroquials, of 5 January 2024, 30 December 2024 and 22 December 2025, BOPA";

/// The parishes, by ISO 3166-2 code.
const AD_CANILLO: &[&str] = &["AD-02"];
const AD_ENCAMP: &[&str] = &["AD-03"];
const AD_LA_MASSANA: &[&str] = &["AD-04"];
const AD_ORDINO: &[&str] = &["AD-05"];
const AD_SANT_JULIA: &[&str] = &["AD-06"];
const AD_ANDORRA_LA_VELLA: &[&str] = &["AD-07"];
const AD_ESCALDES: &[&str] = &["AD-08"];

/// The parish days each comú's instruments for 2024, 2025 and 2026 keep in
/// the whole parish.
static PARISH_DAYS: Listing = Listing::Named(&[
    (2024, 8, 16, "Sant Roc (Canillo)"),
    (2025, 8, 16, "Sant Roc (Canillo)"),
    (2026, 8, 16, "Sant Roc (Canillo)"),
    (2024, 1, 17, "Sant Antoni"),
    (2025, 1, 17, "Sant Antoni"),
    (2026, 1, 17, "Sant Antoni"),
    (2024, 6, 29, "St. Pere"),
    (2025, 6, 29, "St. Pere"),
    (2026, 6, 29, "St. Pere"),
    (2024, 1, 7, "Sant Julià, Patró de la Parròquia"),
    (2025, 1, 7, "Sant Julià, Patró de la Parròquia"),
    (2026, 1, 7, "Sant Julià, Patró de la Parròquia"),
    (2024, 5, 25, "Diada de Canòlich, Patrona de la Parròquia"),
    (2025, 5, 31, "Diada de Canòlich, Patrona de la Parròquia"),
    (2026, 5, 30, "Diada de Canòlich, Patrona de la Parròquia"),
    (2024, 7, 29, "Dilluns de Festa Major"),
    (2025, 7, 28, "Dilluns de Festa Major"),
    (2026, 7, 27, "Dilluns de Festa Major"),
    (2024, 7, 30, "Dimarts de Festa Major"),
    (2025, 7, 29, "Dimarts de Festa Major"),
    (2026, 7, 28, "Dimarts de Festa Major"),
    (2024, 6, 24, "Festa del Poble (Sant Joan)"),
    (2025, 6, 24, "Festa del Poble (Sant Joan)"),
    (2026, 6, 24, "Festa del Poble (Sant Joan)"),
    (2024, 8, 3, "Festa Major"),
    (2024, 8, 4, "Festa Major"),
    (2024, 8, 5, "Festa Major"),
    (2025, 8, 2, "Festa Major"),
    (2025, 8, 3, "Festa Major"),
    (2025, 8, 4, "Festa Major"),
    (2026, 8, 1, "Festa Major"),
    (2026, 8, 2, "Festa Major"),
    (2026, 8, 3, "Festa Major"),
    (2025, 5, 8, "Diada de Sant Miquel d'Engolasters"),
    (2026, 5, 8, "Diada de Sant Miquel d'Engolasters"),
    (
        2024,
        6,
        16,
        "Diada commemorativa de la creació de la parròquia",
    ),
    (
        2025,
        6,
        15,
        "Diada commemorativa de la creació de la parròquia",
    ),
    (
        2026,
        6,
        14,
        "Diada commemorativa de la creació de la parròquia",
    ),
    (2024, 7, 25, "Sant Jaume (Festa Major)"),
    (2025, 7, 25, "Sant Jaume (Festa Major)"),
    (2026, 7, 25, "Sant Jaume (Festa Major)"),
    (2024, 7, 26, "Santa Anna (Festa Major)"),
    (2025, 7, 26, "Santa Anna (Festa Major)"),
    (2026, 7, 26, "Santa Anna (Festa Major)"),
]);

/// A parish day, as the comú's instruments for 2024 to 2026 date it, and a
/// gap before 2024 and after 2026, whose instruments were not read.
const fn parish(
    name: &'static str,
    local_name: &'static str,
    key: &'static str,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::fixed_public(
        name,
        local_name,
        Rule::listed(PARISH_DAYS.named(key), 2024, 2026),
    )
    .in_regions(region)
    .cited(source)
}

static AD_RULES: &[HolidayRule] = &read_all(
    2024,
    [
        HolidayRule::fixed_public("New Year's Day", "Cap d'Any", Rule::gregorian(1, 1)),
        HolidayRule::fixed_public("Epiphany", "Reis", Rule::gregorian(1, 6)),
        HolidayRule::fixed_public("Carnival", "Carnaval", Rule::easter(SHROVE_MONDAY)),
        HolidayRule::fixed_public(
            "Constitution Day",
            "Dia de la Constitució",
            Rule::gregorian(3, 14),
        ),
        HolidayRule::fixed_public("Good Friday", "Divendres Sant", Rule::easter(GOOD_FRIDAY)),
        HolidayRule::fixed_public(
            "Easter Monday",
            "Dilluns de Pasqua",
            Rule::easter(EASTER_MONDAY),
        ),
        HolidayRule::fixed_public("Labour Day", "Festa del Treball", Rule::gregorian(5, 1)),
        HolidayRule::fixed_public(
            "Whit Monday",
            "Dilluns de Pentecosta",
            Rule::easter(WHIT_MONDAY),
        ),
        HolidayRule::fixed_public("Assumption", "Assumpció", Rule::gregorian(8, 15)),
        HolidayRule::fixed_public(
            "Our Lady of Meritxell",
            "Mare de Déu de Meritxell",
            Rule::gregorian(9, 8),
        ),
        HolidayRule::fixed_public("All Saints' Day", "Tots Sants", Rule::gregorian(11, 1)),
        HolidayRule::fixed_public(
            "Immaculate Conception",
            "Immaculada Concepció",
            Rule::gregorian(12, 8),
        ),
        HolidayRule::fixed_public("Christmas Day", "Nadal", Rule::gregorian(12, 25)),
        HolidayRule::fixed_public(
            "Saint Stephen's Day",
            "Sant Esteve",
            Rule::gregorian(12, 26),
        ), // ── The parishes' own days, as each comú fixes them ─────────────────
        parish(
            "Saint Roch",
            "Sant Roc",
            "Sant Roc (Canillo)",
            AD_CANILLO,
            CANILLO,
        ),
        parish(
            "Saint Anthony",
            "Sant Antoni",
            "Sant Antoni",
            AD_LA_MASSANA,
            LA_MASSANA,
        ),
        parish("Saint Peter", "St. Pere", "St. Pere", AD_ORDINO, ORDINO),
        parish(
            "Saint Julian",
            "Sant Julià, Patró de la Parròquia",
            "Sant Julià, Patró de la Parròquia",
            AD_SANT_JULIA,
            SANT_JULIA,
        ),
        parish(
            "Our Lady of Canòlich",
            "Diada de Canòlich, Patrona de la Parròquia",
            "Diada de Canòlich, Patrona de la Parròquia",
            AD_SANT_JULIA,
            SANT_JULIA,
        ),
        parish(
            "Festa Major Monday",
            "Dilluns de Festa Major",
            "Dilluns de Festa Major",
            AD_SANT_JULIA,
            SANT_JULIA,
        ),
        parish(
            "Festa Major Tuesday",
            "Dimarts de Festa Major",
            "Dimarts de Festa Major",
            AD_SANT_JULIA,
            SANT_JULIA,
        ),
        parish(
            "Village Festival",
            "Festa del Poble (Sant Joan)",
            "Festa del Poble (Sant Joan)",
            AD_ANDORRA_LA_VELLA,
            ANDORRA_LA_VELLA,
        ),
        parish(
            "Festa Major",
            "Festa Major",
            "Festa Major",
            AD_ANDORRA_LA_VELLA,
            ANDORRA_LA_VELLA,
        ),
        parish(
            "Saint Michael of Engolasters",
            "Diada de Sant Miquel d'Engolasters",
            "Diada de Sant Miquel d'Engolasters",
            AD_ESCALDES,
            ESCALDES,
        ),
        parish(
            "Parish Foundation Day",
            "Diada commemorativa de la creació de la parròquia",
            "Diada commemorativa de la creació de la parròquia",
            AD_ESCALDES,
            ESCALDES,
        ),
        parish(
            "Saint James",
            "Sant Jaume (Festa Major)",
            "Sant Jaume (Festa Major)",
            AD_ESCALDES,
            ESCALDES,
        ),
        parish(
            "Saint Anne",
            "Santa Anna (Festa Major)",
            "Santa Anna (Festa Major)",
            AD_ESCALDES,
            ESCALDES,
        ),
        // Encamp's instruments for 2024 to 2026 keep no day in the whole
        // parish; the other years' were not read.
        HolidayRule::fixed_public(
            "Parish days",
            "Festes parroquials",
            Rule::unlisted(2024, 2026),
        )
        .in_regions(AD_ENCAMP)
        .cited(ENCAMP),
    ],
);

/// Andorra.
///
/// Law 31/2018 on labour relations, article 62, gives the right to the
/// holidays "of the work calendar", which the Government decrees each
/// year; the table carries the fourteen national days of the 2026
/// calendar (Decree 340/2025), the same the calendars of 2024 and 2025
/// carried, and so reads them from 2024; every earlier year is a gap
/// (ADR 0013), since Law 31/2018 leaves the days to the yearly decree and
/// no earlier decree was read: New Year's Day, Epiphany, Carnival on the Monday before
/// Lent, Constitution Day, Good Friday, Easter Monday, 1 May, Whit
/// Monday, the Assumption, Our Lady of Meritxell, All Saints, the
/// Immaculate Conception, Christmas and Saint Stephen. The up to four
/// days each parish adds are carried where a comú keeps them in the whole
/// parish, for the years read; the tourism sector's leave to move all but
/// four of the days by agreement is not carried. Nothing moves off a
/// Sunday.
///
/// Read from 2024; every earlier year is a gap (ADR 0013). The reason
/// and the source are in docs/systems/holiday-first-years.md.
pub static ANDORRA: RuleSet = RuleSet {
    code: "AD",
    english_name: "Andorra",
    rules: AD_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: weekends::AD,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "Llei 31/2018, del 6 de desembre, de relacions laborals, article 62, from the \
              Cambra de Comerç's copy (ccis.ad), retrieved 2026-09-22; the Government's \
              notice of Decret 340/2025 approving the 2026 work calendar (govern.ad) and \
              La Vall Associats' reproduction of its list; Wikipedia, \"2024 in \
              Andorra\", \"2025 in Andorra\" and \"2026 in Andorra\", for the yearly \
              dates, and \"Public holidays in Andorra\" for the names; the Government's \
              decrets 487/2023, 409/2024 and 340/2025 approving the calendars of 2024 to 2026, \
              article 3, and each comú's instruments for those years, in the BOPA's HTML \
              (bopa.ad), retrieved 2026-09-29, each cited on its entries",
    subdivisions: Subdivisions::Read(&[]),
};
