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
//! rule reaches. Where the only instruments read that date a department's
//! efeméride are the one-year declarations made before 1985 — Chuquisaca's,
//! Cochabamba's, Potosí's, Santa Cruz's, Pando's and Beni's — the day is
//! carried from 1986, the first whole year of Decreto Supremo 21060, which
//! made the efeméride a holiday every year, on the date those declarations
//! give, the years before a gap. Cochabamba's 14 August and Beni's
//! 10 November, which departmental laws not read add, are gaps.

use hc_calendar::Weekday;

use super::read_all;
use super::weekends;
use crate::computus::offsets::{CORPUS_CHRISTI, GOOD_FRIDAY, SHROVE_MONDAY, SHROVE_TUESDAY};
use crate::rule::{
    HolidayRule, Rule, RuleSet, SourceDate, Subdivisions, SubstituteDirection, SubstitutionPolicy,
    joined,
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

/// The first whole year of Decreto Supremo 21060 of 29 August 1985, whose
/// article 67 makes each department's efeméride a holiday.
const DS_21060_FIRST_YEAR: i32 = 1986;

/// The instruments that date the efemérides before 1985, each for its own
/// year, quoted as read in the raw HTML of lexivox's and derechoteca's copies
/// (both secondary).
const CHUQUISACA_DECREES: &str = "Decreto Supremo 1186 of 24 May 1948: \"Declárase feriado el día 25 \
     de los corrientes en el Departamento de Chuquisaca\" (derechoteca.com), and Decreto Supremo \
     8772 of 1969, which keeps the day on Monday 26 May 1969 because \"el aniversario de la \
     efemérides del citado departamento coincide con un día domingo\" \
     (lexivox.org/norms/BO-DS-8772.xhtml); with Decreto Supremo 21060, art. 67, as \
     the preambles of Decreto Presidencial 205 and Decreto Supremo 4219 quote it; retrieved \
     2026-09-29";
const COCHABAMBA_LAWS: &str = "Ley de 14 de septiembre de 1950: \"declárase civil feriado ... los \
     días 14 y 15 del presente mes, en el Departamento de Cochabamba\" \
     (lexivox.org/norms/BO-L-19500914-1.xhtml); Decreto Supremo 27723 of 13 September 2004: \
     \"el Departamento de Cochabamba celebra sus efemérides el 14 de septiembre\" \
     (lexivox.org/norms/BO-DS-27723.xhtml); with Decreto Supremo 21060, art. 67; retrieved \
     2026-09-29";
const POTOSI_DECREES: &str = "Ley de 20 de octubre de 1909: \"decláranse cívicos feriados los días \
     14 y 24 de Septiembre y 10 de Noviembre de 1910\" (lexivox.org/norms/BO-L-19091020-2.xhtml), \
     and Decretos Supremos 1797 of 1949 and 3870 of 1954: \"el día 10 de noviembre\" in the \
     Department of Potosí (derechoteca.com); with Decreto Supremo 21060, art. 67; retrieved \
     2026-09-29";
const SANTA_CRUZ_DECREES: &str = "Decreto Supremo 8933 of 23 September 1969: \"Declaráse feriado con \
     suspensión de actividades públicas y privadas el día miércoles 24 de septiembre de 1969 para \
     el departamento de Santa Cruz\" (lexivox.org/norms/BO-DS-8933.xhtml), and the Ley de 21 de \
     septiembre de 1950 (lexivox.org/norms/BO-L-19500921-1.xhtml); with Decreto Supremo 21060, \
     art. 67; retrieved 2026-09-29";
const PANDO_DECREES: &str = "Decreto Supremo 8934 of 23 September 1969: \"el día miércoles 24 de \
     septiembre de 1969, para el departamento de Pando\" (lexivox.org/norms/BO-DS-8934.xhtml), \
     and the Ley de 21 de septiembre de 1950 (lexivox.org/norms/BO-L-19500921-1.xhtml); with \
     Decreto Supremo 21060, art. 67; retrieved 2026-09-29";
const BENI_DECREES: &str = "Decreto Supremo 4774 of 17 November 1957: \"En homenaje a la efemérides \
     beniana, se declara civil feriado el día 18 del presente mes\" (derechoteca.com), and the Ley \
     de 17 de noviembre de 1942 for the centenary, 18 November 1942 \
     (lexivox.org/norms/BO-L-19421117-1.xhtml); with Decreto Supremo 21060, art. 67; retrieved \
     2026-09-29";

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

/// A department's day whose instrument was not read: every year of its
/// `.years` is a gap.
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
const UNDATED: &str = "A departmental law not read: Cochabamba's Ley Departamental 946 of 2019 \
     or 2020, found in no form, and Beni's Ley Departamental 003 of 2010, published as PDF, as \
     the press quotes them; the Ministry of Labour's yearly notices, PDF, not read";

/// The first whole year of Decreto Supremo 2750 of 1 May 2016, whose
/// calendar the nationwide days are read from; the Sunday rule it carries
/// begins in 2016, and the days of the earlier calendars were not read.
const NATIONAL_FIRST: i32 = 2017;

/// Decreto Supremo 5521 of 13 January 2026, arts. 3, 4 and 5, read in the
/// text of the decree reproduced by pixilegal.com
/// (www.pixilegal.com/normativa/decreto-5521-2026-01-13) and
/// Infoleyes (bolivia.infoleyes.com/norma/8569/decreto-supremo-5521),
/// retrieved 2026-10-03; the Gaceta Oficial publishes PDF only.
const DS_5521: &str = "Decreto Supremo 5521 of 13 January 2026, arts. 3 and 4: Friday 5 June and \
     Friday 7 August are additional national holidays for 2026, and the holidays of Thursday 22 \
     January and Sunday 21 June are moved to Friday 23 January and Monday 22 June (pixilegal.com, \
     Infoleyes; the Gaceta Oficial's PDF not read), retrieved 2026-10-03";

/// The years whose decrees moving or adding holidays were not read.
const YEARLY_DECREES: &str = "The Government's decrees that move or add national holidays for a \
     year, after Decreto Supremo 2750 (2016), art. 2: only the decree for 2026 was read; the \
     Ministry of Labour's yearly notices are PDF, not read";

/// Decreto Supremo 2750 (2016), art. 3: the Monday after a national
/// holiday that falls on a Sunday is a holiday, except for the Carnival
/// days, Good Friday, Corpus Christi and All Souls' Day, which the article
/// names and which are `fixed_public` below.
static BO_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    regions: &[],
    avoid: &[],
    valid_from: Some(2016),
    valid_until: None,
}];

static BO_NATIONAL: &[HolidayRule] = &read_all(
    NATIONAL_FIRST,
    [
        HolidayRule::public("New Year's Day", "Año Nuevo", Rule::gregorian(1, 1)),
        HolidayRule::public(
            "Plurinational State Foundation Day",
            "Día de la Creación del Estado Plurinacional de Bolivia",
            Rule::gregorian(1, 22),
        )
        .years(Some(2010), Some(2025)),
        // Decreto Supremo 5521 of 13 January 2026, art. 4(1): the holiday of
        // Thursday 22 January is moved to Friday 23 January.
        HolidayRule::public(
            "Plurinational State Foundation Day",
            "Día de la Creación del Estado Plurinacional de Bolivia",
            Rule::gregorian(1, 23),
        )
        .years(Some(2026), Some(2026))
        .cited(DS_5521)
        .read_from(2026),
        HolidayRule::public(
            "Plurinational State Foundation Day",
            "Día de la Creación del Estado Plurinacional de Bolivia",
            Rule::gregorian(1, 22),
        )
        .years(Some(2027), None),
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
        .years(Some(2009), Some(2025)),
        // Decreto Supremo 5521, art. 4(2): the holiday of Sunday 21 June is
        // moved to Monday 22 June.
        HolidayRule::public(
            "Aymara Amazonian New Year",
            "Año Nuevo Aymara Amazónico",
            Rule::gregorian(6, 22),
        )
        .years(Some(2026), Some(2026))
        .cited(DS_5521)
        .read_from(2026),
        HolidayRule::public(
            "Aymara Amazonian New Year",
            "Año Nuevo Aymara Amazónico",
            Rule::gregorian(6, 21),
        )
        .years(Some(2027), None),
        // Decreto Supremo 5521, art. 3: Friday 5 June, after Corpus Christi of
        // Thursday 4 June, and Friday 7 August, after Independence Day of
        // Thursday 6 August, are national holidays for 2026.
        HolidayRule::fixed_public(
            "Additional holiday after Corpus Christi",
            "Feriado nacional adicional al de Corpus Christi",
            Rule::gregorian(6, 5),
        )
        .years(Some(2026), Some(2026))
        .cited(DS_5521)
        .read_from(2026),
        HolidayRule::fixed_public(
            "Additional holiday after Independence Day",
            "Feriado nacional adicional al Día de la Independencia",
            Rule::gregorian(8, 7),
        )
        .years(Some(2026), Some(2026))
        .cited(DS_5521)
        .read_from(2026),
        // The Government decrees bridges and additional days year by year,
        // after Decreto Supremo 2750 and apart from it. Only 2026's decree was
        // read: every other year is a gap, as the table cannot say that none
        // was made.
        HolidayRule::public(
            "The year's decree of moved and additional holidays",
            "",
            Rule::UNREAD,
        )
        .years(Some(2017), Some(2025))
        .cited(YEARLY_DECREES)
        .read_from(NATIONAL_FIRST),
        HolidayRule::public(
            "The year's decree of moved and additional holidays",
            "",
            Rule::UNREAD,
        )
        .years(Some(2027), None)
        .cited(YEARLY_DECREES)
        .read_from(NATIONAL_FIRST),
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
    ],
);

static BO_DEPARTMENTAL: &[HolidayRule] = &[
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
    // The efemérides the one-year declarations before 1985 date, from the
    // first whole year of Decreto Supremo 21060, the years before a gap.
    departmental(
        "Chuquisaca Departmental Day",
        "Efeméride del departamento de Chuquisaca",
        Rule::gregorian(5, 25),
        BO_CHUQUISACA,
        CHUQUISACA_DECREES,
    )
    .read_from(DS_21060_FIRST_YEAR),
    departmental(
        "Cochabamba Departmental Day",
        "Efeméride del departamento de Cochabamba",
        Rule::gregorian(9, 14),
        BO_COCHABAMBA,
        COCHABAMBA_LAWS,
    )
    .read_from(DS_21060_FIRST_YEAR),
    departmental(
        "Potosí Departmental Day",
        "Efeméride del departamento de Potosí",
        Rule::gregorian(11, 10),
        BO_POTOSI,
        POTOSI_DECREES,
    )
    .read_from(DS_21060_FIRST_YEAR),
    departmental(
        "Santa Cruz Departmental Day",
        "Efeméride del departamento de Santa Cruz",
        Rule::gregorian(9, 24),
        BO_SANTA_CRUZ,
        SANTA_CRUZ_DECREES,
    )
    .read_from(DS_21060_FIRST_YEAR),
    departmental(
        "Pando Departmental Day",
        "Efeméride del departamento de Pando",
        Rule::gregorian(9, 24),
        BO_PANDO,
        PANDO_DECREES,
    )
    .read_from(DS_21060_FIRST_YEAR),
    departmental(
        "Beni Departmental Day",
        "Efeméride del departamento del Beni",
        Rule::gregorian(11, 18),
        BO_BENI,
        BENI_DECREES,
    )
    .read_from(DS_21060_FIRST_YEAR),
    // The days departmental laws not read add, every year of them a gap:
    // Cochabamba's 14 August, by a law of 2019 or 2020, and Beni's
    // 10 November, by a law of 2010, as the press quotes them.
    undated(
        "Cochabamba's 14 August",
        "14 de agosto, feriado departamental de Cochabamba",
        BO_COCHABAMBA,
    )
    .years(Some(2019), None),
    undated(
        "Beni's 10 November",
        "10 de noviembre, feriado departamental del Beni",
        BO_BENI,
    )
    .years(Some(2010), None),
];

/// How many rules [`BOLIVIA`] has in all.
const BO_LEN: usize = BO_NATIONAL.len() + BO_DEPARTMENTAL.len();

/// Every rule of [`BOLIVIA`]: the nationwide days, then the departments'.
static BO_RULES: [HolidayRule; BO_LEN] = joined(&[BO_NATIONAL, BO_DEPARTMENTAL]);

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
///
/// Read from 2017: Decreto Supremo 2750 of 1 May 2016, from its first whole
/// year; the departments' days are read from their own instruments, 2026 from
/// Decreto Supremo 5521, and every other year's decree that moves or adds
/// days is a gap. Every earlier year is a gap (ADR 0013); the reasons for
/// every table's first year are in docs/systems/holiday-first-years.md.
pub static BOLIVIA: RuleSet = RuleSet {
    code: "BO",
    english_name: "Bolivia",
    rules: &BO_RULES,
    substitution: BO_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: weekends::BO,
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
