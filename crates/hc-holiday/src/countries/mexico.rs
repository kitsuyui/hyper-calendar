//! Mexico — the federal days of rest of the Ley Federal del Trabajo, and
//! the states' own.
//!
//! The states' days are written up in `docs/systems/mexico-holidays.md` in
//! the repository, with every state and what was read for it. A state's
//! own days are those its law for its public servants adds to the federal
//! list; they bind the state's and its municipalities' offices, not a
//! private employer, whom the federal list alone binds, so they are
//! [`Kind::Government`]. Only Jalisco's law could be read in HTML, and it
//! is the only state carried; the others publish their laws as PDF or
//! Word files, which were not read, or leave the days to a yearly official
//! calendar.

use hc_calendar::Weekday;
use hc_calendars_solar::gregorian;

use crate::rule::{
    Days, HolidayRule, Kind, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate, Subdivisions,
};

/// The presidential handover day, Ley Federal del Trabajo art. 74 VI.
///
/// A holiday once every six years and on no other day. It was 1 December
/// through the 2018 handover; the 2024 constitutional reform moved the term
/// change to 1 October, so 2024 was the first handover kept on that date.
fn presidential_handover(year: i64) -> Days {
    let (first_year, month) = if year >= 2024 {
        (2024i64, 10u8)
    } else {
        (1934, 12)
    };
    if year < first_year || (year - first_year).rem_euclid(6) != 0 {
        return Days::new();
    }
    gregorian::to_fixed(year, month, 1).map_or_else(|_| Days::new(), Days::one)
}

/// Jalisco.
const JALISCO: &[&str] = &["MX-JAL"];

/// The instrument for Jalisco's days.
const JALISCO_LAW: &str = "Ley para los Servidores Públicos del Estado de Jalisco y sus \
     Municipios (Decreto 11559, in force 16 June 1984), artículo 38 as reformed by Decreto \
     21593/LVII/06 (Periódico Oficial El Estado de Jalisco, 2 December 2006): días de descanso \
     obligatorio";

/// A day of rest Jalisco's law adds for its public servants, in the text
/// read, in force from its reform of December 2006.
const fn jalisco(name: &'static str, local_name: &'static str, month: u8, day: u8) -> HolidayRule {
    HolidayRule::fixed_public(name, local_name, Rule::gregorian(month, day))
        .of_kind(Kind::Government)
        .read_from(2007)
        .in_regions(JALISCO)
        .cited(JALISCO_LAW)
}

static MX_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Año Nuevo", Rule::gregorian(1, 1)),
    // The 2006 reform moved three fixed dates onto Mondays.
    HolidayRule::fixed_public(
        "Constitution Day",
        "Día de la Constitución",
        Rule::gregorian(2, 5),
    )
    .years(None, Some(2005)),
    HolidayRule::fixed_public(
        "Constitution Day",
        "Día de la Constitución",
        Rule::nth(2, 1, Weekday::Monday),
    )
    .years(Some(2006), None),
    HolidayRule::fixed_public(
        "Benito Juárez's Birthday",
        "Natalicio de Benito Juárez",
        Rule::gregorian(3, 21),
    )
    .years(None, Some(2005)),
    HolidayRule::fixed_public(
        "Benito Juárez's Birthday",
        "Natalicio de Benito Juárez",
        Rule::nth(3, 3, Weekday::Monday),
    )
    .years(Some(2006), None),
    HolidayRule::fixed_public("Labour Day", "Día del Trabajo", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public(
        "Independence Day",
        "Día de la Independencia",
        Rule::gregorian(9, 16),
    ),
    HolidayRule::fixed_public(
        "Revolution Day",
        "Día de la Revolución",
        Rule::gregorian(11, 20),
    )
    .years(None, Some(2005)),
    HolidayRule::fixed_public(
        "Revolution Day",
        "Día de la Revolución",
        Rule::nth(11, 3, Weekday::Monday),
    )
    .years(Some(2006), None),
    HolidayRule::fixed_public(
        "Presidential Inauguration",
        "Transmisión del Poder Ejecutivo Federal",
        Rule::Computed(presidential_handover),
    ),
    HolidayRule::fixed_public("Christmas Day", "Navidad", Rule::gregorian(12, 25)),
    // ── The states' own days ────────────────────────────────────────────
    // Article 38 names these by their dates alone.
    jalisco("5 May", "5 de mayo", 5, 5),
    jalisco("28 September", "28 de septiembre", 9, 28),
    jalisco("12 October", "12 de octubre", 10, 12),
    jalisco("2 November", "2 de noviembre", 11, 2),
];

/// Mexico.
pub static MEXICO: RuleSet = RuleSet {
    code: "MX",
    english_name: "Mexico",
    rules: MX_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "Ley Federal del Trabajo, artículo 74, as reformed in 2006 and by the decree \
              reforming fracción VII (DOF 30 September 2024); the Cámara de Diputados text \
              (diputados.gob.mx/LeyesBiblio/pdf/LFT.pdf) could not be reached on 2026-09-26, \
              and the article was read in a secondary copy (conceptosjuridicos.com), retrieved \
              2026-09-26. Election days under fracción IX are not modelled. Religious days — \
              Semana Santa, 12 December — are not días de descanso obligatorio and are not \
              listed. Jalisco's days: the Ley para los Servidores Públicos del Estado de Jalisco \
              y sus Municipios, artículo 38, in the Government of Jalisco's HTML copy \
              (siga.jalisco.gob.mx/Assets/documentos/normatividad/ley_servidores.htm), whose \
              list of reforms ends in December 2009, retrieved 2026-09-29; the other states' \
              laws, published as PDF or Word files, not read, as \
              docs/systems/mexico-holidays.md lists them",
    subdivisions: Subdivisions::Read(&[]),
};
