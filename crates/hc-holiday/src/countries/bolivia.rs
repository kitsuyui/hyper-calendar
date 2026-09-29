//! Bolivia — the national holidays of Decreto Supremo 2750, and the
//! departments' own.
//!
//! The departments' days are written up in `docs/systems/bolivia-holidays.md`
//! in the repository. Decreto Supremo 21060 of 1985, article 67, makes each
//! department's efeméride a holiday with the suspension of public and
//! private activity in the department, without dating it; the table carries
//! a department's day from the first instrument read that dates it — the
//! years before a gap where that instrument presupposes the day, and absent
//! where it sets it, as Pando's law of 2024 does — scoped
//! to the department's ISO 3166-2 code, and moves a Sunday one to the Monday
//! from 2024, the first year Decreto Supremo 5019's extension of the Sunday
//! rule reaches. Only four departments' days are dated by an instrument
//! read; the others, and Santa Cruz's 24 September, rest on the Ministry of
//! Labour's yearly notices, published as PDF and not read, and are not yet
//! carried.

use hc_calendar::Weekday;

use crate::computus::offsets::{CORPUS_CHRISTI, GOOD_FRIDAY, SHROVE_MONDAY, SHROVE_TUESDAY};
use crate::rule::{
    HolidayRule, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate, Subdivisions, SubstituteDirection,
    SubstitutionPolicy,
};

/// The departments, by ISO 3166-2 code.
const BO_BENI: &[&str] = &["BO-B"];
const BO_COCHABAMBA: &[&str] = &["BO-C"];
const BO_CHUQUISACA: &[&str] = &["BO-H"];
const BO_LA_PAZ: &[&str] = &["BO-L"];
const BO_ORURO: &[&str] = &["BO-O"];
const BO_PANDO: &[&str] = &["BO-N"];
const BO_POTOSI: &[&str] = &["BO-P"];
const BO_SANTA_CRUZ: &[&str] = &["BO-S"];
const BO_TARIJA: &[&str] = &["BO-T"];

/// The instruments read for the departments, on lexivox.org (secondary: the
/// Gaceta Oficial publishes PDF only).
const LA_PAZ_DECREE: &str = "Decreto Presidencial 205 of 13 July 2009, artículo único: \"queda \
     vigente el feriado departamental de La Paz del día 16 de julio\", and 15 July 2009 \"por \
     única vez\"; Decreto Supremo 21060 of 29 August 1985, art. 67, as its preamble quotes it \
     (lexivox.org/norms/BO-DP-N205.xhtml), retrieved 2026-09-29";
const ORURO_DECREE: &str = "Decreto Supremo 1484 of 6 February 2013, artículo único: \"el feriado \
     departamental del día 10 de febrero\", moved to 6 February \"por única vez\" \
     (lexivox.org/norms/BO-DS-N1484.xhtml), retrieved 2026-09-29";
const PANDO_LAW: &str = "Ley 1606 of 13 November 2024, art. 1: \"feriado departamental\" on 11 \
     October \"de cada año\", with the suspension of work in the public and private sectors in \
     Pando (lexivox.org/norms/BO-L-N1606.xhtml), retrieved 2026-09-29";
const SANTA_CRUZ_LAW: &str = "Ley Departamental 21 of 23 September 2010 of the Asamblea Legislativa \
     Departamental de Santa Cruz, arts. 1 and 4: \"feriado departamental, sin suspensión de \
     actividades\" (lexivox.org/norms/BO_SCZ-LD-21.xhtml), retrieved 2026-09-29";
const TARIJA_DECREE: &str = "Decreto Supremo 4219 of 14 April 2020, artículo único: the \"feriado \
     departamental de Tarija\", the efeméride \"que se celebra el 15 de abril de cada año\" \
     (lexivox.org/norms/BO-DS-N4219.xhtml), retrieved 2026-09-29";

/// A department's day, with the suspension of public and private activity
/// in the department, moved off a Sunday from 2024. Its years are the
/// call's: `.years` where the instrument read sets the day, `.read_from`
/// where it presupposes it, the years before being a gap.
const fn departmental(
    name: &'static str,
    local_name: &'static str,
    rule: Rule,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::public(name, local_name, rule)
        .in_regions(region)
        .substituted_from(2024)
        .cited(source)
}

/// A department's efeméride that no instrument read dates: Decreto Supremo
/// 21060, art. 67, makes it a holiday, so every year is a gap.
const fn undated(
    name: &'static str,
    local_name: &'static str,
    region: &'static [&'static str],
) -> HolidayRule {
    HolidayRule::public(name, local_name, Rule::UNREAD)
        .in_regions(region)
        .cited(UNDATED)
}

/// Why an efeméride is [`undated`].
const UNDATED: &str = "Decreto Supremo 21060 of 29 August 1985, art. 67, as the preambles of \
     Decreto Presidencial 205 (2009) and Decreto Supremo 4219 (2020) quote it: the department's \
     efeméride is a holiday; no instrument read dates it, the Ministry of Labour's yearly \
     notices being PDF, not read";

/// Decreto Supremo 2750 (2016), art. 3: the Monday after a national
/// holiday that falls on a Sunday is a holiday, except for the Carnival
/// days, Good Friday, Corpus Christi and All Souls' Day, which the article
/// names and which are `fixed_public` below.
static BO_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: Some(2016),
    valid_until: None,
}];

static BO_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Año Nuevo", Rule::gregorian(1, 1)),
    HolidayRule::public(
        "Plurinational State Foundation Day",
        "Día de la Creación del Estado Plurinacional de Bolivia",
        Rule::gregorian(1, 22),
    )
    .years(Some(2010), None),
    HolidayRule::fixed_public(
        "Carnival Monday",
        "Lunes de Carnaval",
        Rule::easter(SHROVE_MONDAY),
    ),
    HolidayRule::fixed_public(
        "Carnival Tuesday",
        "Martes de Carnaval",
        Rule::easter(SHROVE_TUESDAY),
    ),
    HolidayRule::fixed_public("Good Friday", "Viernes Santo", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public("Labour Day", "Día del Trabajo", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public(
        "Corpus Christi",
        "Corpus Christi",
        Rule::easter(CORPUS_CHRISTI),
    ),
    HolidayRule::public(
        "Aymara Amazonian New Year",
        "Año Nuevo Aymara Amazónico",
        Rule::gregorian(6, 21),
    )
    .years(Some(2009), None),
    HolidayRule::public(
        "Independence Day",
        "Día de la Independencia de Bolivia",
        Rule::gregorian(8, 6),
    ),
    HolidayRule::fixed_public(
        "All Souls' Day",
        "Día de Todos los Difuntos",
        Rule::gregorian(11, 2),
    ),
    HolidayRule::public("Christmas Day", "Navidad", Rule::gregorian(12, 25)),
    // ── The departments' own days ───────────────────────────────────────
    departmental(
        "La Paz Departmental Day",
        "Feriado departamental de La Paz",
        Rule::gregorian(7, 16),
        BO_LA_PAZ,
        LA_PAZ_DECREE,
    )
    .read_from(2009),
    HolidayRule::fixed_public(
        "Bicentenary of the La Paz Revolution",
        "Feriado departamental por el Bicentenario de la Gesta Libertaria",
        Rule::gregorian(7, 15),
    )
    .years(Some(2009), Some(2009))
    .in_regions(BO_LA_PAZ)
    .cited(LA_PAZ_DECREE),
    HolidayRule::fixed_public(
        "Oruro Departmental Day",
        "Efeméride Departamental de Oruro",
        Rule::gregorian(2, 6),
    )
    .years(Some(2013), Some(2013))
    .in_regions(BO_ORURO)
    .cited(ORURO_DECREE),
    // Oruro's 10 February, moved to 6 February in 2013 alone: the 10 February
    // rule stops before 2013 as a gap, the decree of that year presupposing
    // the day.
    departmental(
        "Oruro Departmental Day",
        "Efeméride Departamental de Oruro",
        Rule::gregorian(2, 10),
        BO_ORURO,
        ORURO_DECREE,
    )
    .years(None, Some(2012))
    .read_from(2013),
    departmental(
        "Oruro Departmental Day",
        "Efeméride Departamental de Oruro",
        Rule::gregorian(2, 10),
        BO_ORURO,
        ORURO_DECREE,
    )
    .years(Some(2014), None),
    departmental(
        "Tarija Departmental Day",
        "Efeméride del departamento de Tarija",
        Rule::gregorian(4, 15),
        BO_TARIJA,
        TARIJA_DECREE,
    )
    .read_from(2020),
    departmental(
        "Battle of Bahía",
        "Batalla de Bahía",
        Rule::gregorian(10, 11),
        BO_PANDO,
        PANDO_LAW,
    )
    .years(Some(2025), None)
    .substituted_from(2025),
    HolidayRule::observance(
        "Departmental Autonomy Day",
        "Día Departamental de la Autonomía",
        Rule::gregorian(5, 4),
    )
    .in_regions(BO_SANTA_CRUZ)
    .cited(SANTA_CRUZ_LAW)
    // The law of 2010 repeals a departmental decree of 29 April 2009, not
    // read, which may have set the day before it: the years before are a
    // gap.
    .read_from(2011),
    // The efemérides no instrument read dates, every year a gap: Beni's
    // 18 November, Cochabamba's 14 September, Chuquisaca's 25 May, Potosí's
    // 10 November, and Santa Cruz's and Pando's 24 September, as the
    // Ministry's notices and the press give them.
    undated(
        "Beni Departmental Day",
        "Efeméride del departamento del Beni",
        BO_BENI,
    ),
    undated(
        "Cochabamba Departmental Day",
        "Efeméride del departamento de Cochabamba",
        BO_COCHABAMBA,
    ),
    undated(
        "Chuquisaca Departmental Day",
        "Efeméride del departamento de Chuquisaca",
        BO_CHUQUISACA,
    ),
    undated(
        "Potosí Departmental Day",
        "Efeméride del departamento de Potosí",
        BO_POTOSI,
    ),
    undated(
        "Santa Cruz Departmental Day",
        "Efeméride del departamento de Santa Cruz",
        BO_SANTA_CRUZ,
    ),
    undated(
        "Pando Departmental Day",
        "Efeméride del departamento de Pando",
        BO_PANDO,
    ),
];

/// Bolivia.
///
/// The national holidays of Decreto Supremo 2750 of 1 May 2016, with
/// 22 January from Decreto Supremo 405 of 2010 and 21 June from Decreto
/// Supremo 173 of 2009, and the decree's Sunday rule, which the four
/// holidays it names sit outside. The departmental holidays, to which
/// Decreto Supremo 5019 of 2023 extended the Sunday rule, are carried where
/// an instrument read dates them; the bridges and moves the Government
/// decrees year by year, such as 2026's Friday 23 January, are not. The
/// Sunday rule is carried from the 2016
/// decree, and whatever earlier decrees did is not.
pub static BOLIVIA: RuleSet = RuleSet {
    code: "BO",
    english_name: "Bolivia",
    rules: BO_RULES,
    substitution: BO_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "Decreto Supremo 2750 of 1 May 2016, arts. 2 and 3, and Decreto Supremo \
              5019 of 13 September 2023, lexivox.org, retrieved 2026-09-22; Decreto \
              Supremo 173 of 17 June 2009 for 21 June, lexivox.org; Decreto Supremo \
              405 of 20 January 2010 for 22 January, as reported by the Ministry of \
              Labour; Wikipedia, \"Public holidays in Bolivia\", retrieved the same \
              day, for the English names; the departments' days from the instruments cited on \
              their entries, retrieved 2026-09-29, as docs/systems/bolivia-holidays.md lists \
              them",
    subdivisions: Subdivisions::Read(&[]),
};
