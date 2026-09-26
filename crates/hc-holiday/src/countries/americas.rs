//! Tables for the Americas.

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;
use hc_seasons::{Meridian, SolarTerm};

use crate::computus::offsets::{
    ASCENSION, ASH_WEDNESDAY, CORPUS_CHRISTI, EASTER_MONDAY, EASTER_SUNDAY, GOOD_FRIDAY,
    HOLY_SATURDAY, MAUNDY_THURSDAY, SACRED_HEART, SHROVE_MONDAY, SHROVE_TUESDAY, WHIT_MONDAY,
};
use crate::hindu::{DIWALI, HOLI};
use crate::rule::{
    CalendarSystem, Days, HolidayRule, Kind, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate,
    SubstituteDirection, SubstitutionPolicy, TO_ADJACENT_MONDAY, TO_FOLLOWING_MONDAY,
    WeekendPolicy,
};

// ─────────────────────────────────────────────────────────────────────────
// United States
// ─────────────────────────────────────────────────────────────────────────

/// The District of Columbia and the counties around it, the only place
/// Inauguration Day is a holiday.
const US_CAPITAL_REGION: &[&str] = &["US-DC"];

/// Inauguration Day, 5 U.S.C. § 6103(c).
///
/// "January 20 of each fourth year after 1965" — and 21 January when the
/// 20th is a Sunday, because the oath is not administered publicly on a
/// Sunday. The section explicitly denies the in-lieu-of day that every other
/// federal holiday gets, so a Saturday inauguration simply is not a holiday
/// for anyone. Nothing in the rule vocabulary says "every fourth year", so
/// this is one of the genuine handful.
fn inauguration_day(year: i64) -> Days {
    if year < 1965 || (year - 1965).rem_euclid(4) != 0 {
        return Days::new();
    }
    let Ok(twentieth) = gregorian::to_fixed(year, 1, 20) else {
        return Days::new();
    };
    if Weekday::from_rd(twentieth) == Weekday::Sunday {
        return Days::one(hc_calendar::Rd(twentieth.0 + 1));
    }
    Days::one(twentieth)
}

static US_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)).years(Some(1871), None),
    HolidayRule::public(
        "Birthday of Martin Luther King, Jr.",
        "",
        Rule::nth(1, 3, Weekday::Monday),
    )
    .years(Some(1986), None),
    HolidayRule::fixed_public("Inauguration Day", "", Rule::Computed(inauguration_day))
        .in_regions(US_CAPITAL_REGION),
    // The Uniform Monday Holiday Act of 1968 took effect on 1 January 1971.
    HolidayRule::public("Washington's Birthday", "", Rule::gregorian(2, 22))
        .years(Some(1879), Some(1970)),
    HolidayRule::public(
        "Washington's Birthday",
        "",
        Rule::nth(2, 3, Weekday::Monday),
    )
    .years(Some(1971), None),
    HolidayRule::public("Memorial Day", "", Rule::gregorian(5, 30)).years(Some(1888), Some(1970)),
    HolidayRule::public("Memorial Day", "", Rule::last(5, Weekday::Monday)).years(Some(1971), None),
    HolidayRule::public(
        "Juneteenth National Independence Day",
        "",
        Rule::gregorian(6, 19),
    )
    .years(Some(2021), None),
    HolidayRule::public("Independence Day", "", Rule::gregorian(7, 4)).years(Some(1871), None),
    HolidayRule::public("Labor Day", "", Rule::nth(9, 1, Weekday::Monday)).years(Some(1894), None),
    HolidayRule::public("Columbus Day", "", Rule::nth(10, 2, Weekday::Monday))
        .years(Some(1971), None),
    // Veterans Day spent the Uniform Monday years on the fourth Monday of
    // October; Public Law 94-97 put it back on 11 November from 1978.
    HolidayRule::public("Veterans Day", "", Rule::gregorian(11, 11)).years(Some(1954), Some(1970)),
    HolidayRule::public("Veterans Day", "", Rule::nth(10, 4, Weekday::Monday))
        .years(Some(1971), Some(1977)),
    HolidayRule::public("Veterans Day", "", Rule::gregorian(11, 11)).years(Some(1978), None),
    HolidayRule::public("Thanksgiving Day", "", Rule::nth(11, 4, Weekday::Thursday))
        .years(Some(1942), None),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)).years(Some(1871), None),
    // State funerals closed the federal government for a day by executive
    // order.
    HolidayRule::fixed_public("National Day of Mourning", "", Rule::gregorian(12, 5))
        .years(Some(2018), Some(2018)),
    HolidayRule::fixed_public("National Day of Mourning", "", Rule::gregorian(1, 9))
        .years(Some(2025), Some(2025)),
];

/// The federal "in lieu of" rule: Executive Order 11582 of 1971 codified
/// what Executive Order 10358 had begun in 1959 — a Saturday holiday is kept
/// the preceding Friday, a Sunday holiday the following Monday.
static US_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Nearest,
    skip_occupied: false,
    on_collision: false,
    valid_from: Some(1959),
    valid_until: None,
}];

/// The United States: the federal holidays of 5 U.S.C. § 6103.
pub static UNITED_STATES: RuleSet = RuleSet {
    code: "US",
    english_name: "United States",
    rules: US_RULES,
    substitution: US_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "5 U.S.C. § 6103(a) to (c), on the Legal Information Institute \
              (law.cornell.edu/uscode/text/5/6103), retrieved 2026-09-26, whose subsection (b) \
              states the in-lieu-of rule; Pub. L. 90-363 (1968, the Uniform Monday Holiday \
              Act), Pub. L. 94-97 (1975) restoring Veterans Day, Pub. L. 98-144 (1983) for \
              Martin Luther King Jr. Day and Pub. L. 117-17 (2021) for Juneteenth, not read; \
              Executive Orders 10358 and 11582 on the in-lieu-of rule before it was in the \
              statute, not read. Closures by executive order for a single year are not \
              carried. State holidays are not modelled: there are no national public holidays \
              in the United States, only federal ones",
};

// ─────────────────────────────────────────────────────────────────────────
// Canada
// ─────────────────────────────────────────────────────────────────────────

const CA_FAMILY_DAY_THIRD_MONDAY: &[&str] = &["CA-AB", "CA-NB", "CA-ON", "CA-SK"];
const CA_BRITISH_COLUMBIA: &[&str] = &["CA-BC"];
const CA_MANITOBA: &[&str] = &["CA-MB"];
const CA_NOVA_SCOTIA: &[&str] = &["CA-NS"];
const CA_PRINCE_EDWARD_ISLAND: &[&str] = &["CA-PE"];
const CA_QUEBEC: &[&str] = &["CA-QC"];
const CA_YUKON: &[&str] = &["CA-YT"];
const CA_AUGUST_CIVIC: &[&str] = &[
    "CA-BC", "CA-MB", "CA-NB", "CA-NT", "CA-NU", "CA-ON", "CA-SK",
];

static CA_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Jour de l'An", Rule::gregorian(1, 1)),
    HolidayRule::public("Family Day", "", Rule::nth(2, 3, Weekday::Monday))
        .in_regions(CA_FAMILY_DAY_THIRD_MONDAY)
        .years(Some(2008), None),
    // British Columbia kept Family Day on the second Monday until 2019.
    HolidayRule::public("Family Day", "", Rule::nth(2, 2, Weekday::Monday))
        .in_regions(CA_BRITISH_COLUMBIA)
        .years(Some(2013), Some(2018)),
    HolidayRule::public("Family Day", "", Rule::nth(2, 3, Weekday::Monday))
        .in_regions(CA_BRITISH_COLUMBIA)
        .years(Some(2019), None),
    HolidayRule::public("Louis Riel Day", "", Rule::nth(2, 3, Weekday::Monday))
        .in_regions(CA_MANITOBA)
        .years(Some(2008), None),
    HolidayRule::public(
        "Nova Scotia Heritage Day",
        "",
        Rule::nth(2, 3, Weekday::Monday),
    )
    .in_regions(CA_NOVA_SCOTIA)
    .years(Some(2015), None),
    HolidayRule::public("Islander Day", "", Rule::nth(2, 3, Weekday::Monday))
        .in_regions(CA_PRINCE_EDWARD_ISLAND)
        .years(Some(2009), None),
    HolidayRule::public("Good Friday", "Vendredi saint", Rule::easter(GOOD_FRIDAY)),
    // Victoria Day is the Monday preceding 25 May.
    HolidayRule::public(
        "Victoria Day",
        "Fête de la Reine",
        Rule::WeekdayOnOrBefore {
            month: 5,
            day: 24,
            weekday: Weekday::Monday,
        },
    ),
    HolidayRule::public(
        "Saint-Jean-Baptiste Day",
        "Fête nationale du Québec",
        Rule::gregorian(6, 24),
    )
    .in_regions(CA_QUEBEC),
    HolidayRule::public("Canada Day", "Fête du Canada", Rule::gregorian(7, 1)),
    HolidayRule::public("Civic Holiday", "", Rule::nth(8, 1, Weekday::Monday))
        .in_regions(CA_AUGUST_CIVIC),
    HolidayRule::public("Discovery Day", "", Rule::nth(8, 3, Weekday::Monday)).in_regions(CA_YUKON),
    HolidayRule::public(
        "Labour Day",
        "Fête du Travail",
        Rule::nth(9, 1, Weekday::Monday),
    ),
    HolidayRule::public(
        "National Day for Truth and Reconciliation",
        "Journée nationale de la vérité et de la réconciliation",
        Rule::gregorian(9, 30),
    )
    .years(Some(2021), None),
    HolidayRule::public(
        "Thanksgiving",
        "Action de grâce",
        Rule::nth(10, 2, Weekday::Monday),
    ),
    HolidayRule::public(
        "Remembrance Day",
        "Jour du Souvenir",
        Rule::gregorian(11, 11),
    ),
    HolidayRule::public("Christmas Day", "Noël", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "Lendemain de Noël", Rule::gregorian(12, 26)),
];

static CA_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// Canada: the federally regulated holidays, plus the provincial days that
/// are fixed by statute rather than by proclamation.
pub static CANADA: RuleSet = RuleSet {
    code: "CA",
    english_name: "Canada",
    rules: CA_RULES,
    substitution: CA_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "Canada Labour Code (R.S.C. 1985, c. L-2), s. 166, and the Holidays Act (R.S.C. \
              1985, c. H-5), both current to 2026-09-03, on the Justice Laws Website \
              (laws-lois.justice.gc.ca), retrieved 2026-09-26; the Holidays Act moves only a \
              Sunday Canada Day, and the Code's s. 195 on a holiday falling on a non-working \
              day was not read; the provincial employment-standards acts for the subdivision \
              entries, not read. Remembrance Day is federal and is not statutory in every \
              province; the provincial variation is not modelled",
};

// ─────────────────────────────────────────────────────────────────────────
// Mexico
// ─────────────────────────────────────────────────────────────────────────

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
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "Ley Federal del Trabajo, artículo 74, as reformed in 2006 and by the decree \
              reforming fracción VII (DOF 30 September 2024); the Cámara de Diputados text \
              (diputados.gob.mx/LeyesBiblio/pdf/LFT.pdf) could not be reached on 2026-09-26, \
              and the article was read in a secondary copy (conceptosjuridicos.com), retrieved \
              2026-09-26. Election days under fracción IX are not modelled. Religious days — \
              Semana Santa, 12 December — are not días de descanso obligatorio and are not \
              listed",
};

// ─────────────────────────────────────────────────────────────────────────
// Brazil
// ─────────────────────────────────────────────────────────────────────────

static BR_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "New Year's Day",
        "Confraternização Universal",
        Rule::gregorian(1, 1),
    ),
    // Carnival is a ponto facultativo, not a feriado nacional, however
    // universally the country stops.
    HolidayRule::observance(
        "Carnival Monday",
        "Segunda-feira de Carnaval",
        Rule::easter(SHROVE_MONDAY),
    ),
    HolidayRule::public(
        "Carnival Tuesday",
        "Terça-feira de Carnaval",
        Rule::easter(SHROVE_TUESDAY),
    )
    .of_kind(Kind::Bank),
    HolidayRule::fixed_public(
        "Good Friday",
        "Sexta-feira da Paixão",
        Rule::easter(GOOD_FRIDAY),
    ),
    HolidayRule::observance("Easter Sunday", "Páscoa", Rule::easter(EASTER_SUNDAY)),
    HolidayRule::fixed_public("Tiradentes", "Tiradentes", Rule::gregorian(4, 21)),
    HolidayRule::fixed_public("Labour Day", "Dia do Trabalhador", Rule::gregorian(5, 1)),
    HolidayRule::public(
        "Corpus Christi",
        "Corpus Christi",
        Rule::easter(CORPUS_CHRISTI),
    )
    .of_kind(Kind::Bank),
    HolidayRule::fixed_public(
        "Independence Day",
        "Independência do Brasil",
        Rule::gregorian(9, 7),
    ),
    HolidayRule::fixed_public(
        "Our Lady of Aparecida",
        "Nossa Senhora Aparecida",
        Rule::gregorian(10, 12),
    )
    .years(Some(1980), None),
    HolidayRule::fixed_public("All Souls' Day", "Finados", Rule::gregorian(11, 2)),
    HolidayRule::fixed_public(
        "Republic Day",
        "Proclamação da República",
        Rule::gregorian(11, 15),
    ),
    HolidayRule::fixed_public(
        "Black Awareness Day",
        "Dia da Consciência Negra",
        Rule::gregorian(11, 20),
    )
    .years(Some(2024), None),
    HolidayRule::fixed_public("Christmas Day", "Natal", Rule::gregorian(12, 25)),
];

/// Brazil.
pub static BRAZIL: RuleSet = RuleSet {
    code: "BR",
    english_name: "Brazil",
    rules: BR_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "Lei nº 662, de 6 de abril de 1949, art. 1, in the wording of Lei nº 10.607, de \
              19 de dezembro de 2002; Lei nº 6.802, de 30 de junho de 1980, for Nossa Senhora \
              Aparecida; Lei nº 14.759, de 21 de dezembro de 2023 (DOU 22 December 2023), \
              making Consciência Negra a national holiday from 2024, its title and publication \
              only; Lei nº 9.093, de 12 de setembro de 1995, art. 2, under which Good Friday \
              is a municipal religious holiday; all on the Câmara dos Deputados' legislation \
              site (camara.leg.br/legin), retrieved 2026-09-26, the Planalto being \
              unreachable. Good Friday is carried nationwide although the federal law makes it \
              the municipalities' to declare. Carnival and Corpus Christi are pontos \
              facultativos, recorded here as bank holidays",
};

// ─────────────────────────────────────────────────────────────────────────
// Argentina
// ─────────────────────────────────────────────────────────────────────────

/// A *feriado trasladable* under article 6 of Ley 27.399: a Tuesday or
/// Wednesday pulled to the Monday before, a Thursday or Friday pushed to
/// the Monday after, a weekend left alone.
const fn ar_trasladable(
    name: &'static str,
    local: &'static str,
    base: &'static Rule,
) -> HolidayRule {
    HolidayRule::public(
        name,
        local,
        Rule::moved_by_weekday(base, TO_ADJACENT_MONDAY),
    )
}

/// A day the observant of a faith may take off, under article 4 of Ley
/// 27.399 and article 5 of Decreto 1584/2010: not a holiday, and stated as
/// an observance.
const fn ar_religious(name: &'static str, local: &'static str, rule: Rule) -> HolidayRule {
    HolidayRule::observance(name, local, rule).years(Some(2011), None)
}

static AR_GUEMES: Rule = Rule::gregorian(6, 17);
static AR_SAN_MARTIN: Rule = Rule::gregorian(8, 17);
static AR_DIVERSIDAD: Rule = Rule::gregorian(10, 12);
static AR_SOBERANIA: Rule = Rule::gregorian(11, 20);

static AR_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Año Nuevo", Rule::gregorian(1, 1))
        .years(Some(2011), None),
    HolidayRule::fixed_public(
        "Carnival Monday",
        "Lunes de Carnaval",
        Rule::easter(SHROVE_MONDAY),
    )
    .years(Some(2011), None),
    HolidayRule::fixed_public(
        "Carnival Tuesday",
        "Martes de Carnaval",
        Rule::easter(SHROVE_TUESDAY),
    )
    .years(Some(2011), None),
    HolidayRule::fixed_public(
        "Day of Remembrance for Truth and Justice",
        "Día Nacional de la Memoria por la Verdad y la Justicia",
        Rule::gregorian(3, 24),
    )
    .years(Some(2011), None),
    HolidayRule::fixed_public(
        "Day of the Veterans and Fallen of the Malvinas War",
        "Día del Veterano y de los Caídos en la Guerra de Malvinas",
        Rule::gregorian(4, 2),
    )
    .years(Some(2011), None),
    HolidayRule::fixed_public("Good Friday", "Viernes Santo", Rule::easter(GOOD_FRIDAY))
        .years(Some(2011), None),
    HolidayRule::observance(
        "Holy Thursday",
        "Jueves Santo",
        Rule::easter(MAUNDY_THURSDAY),
    )
    .years(Some(2011), None),
    HolidayRule::fixed_public("Labour Day", "Día del Trabajador", Rule::gregorian(5, 1))
        .years(Some(2011), None),
    HolidayRule::fixed_public(
        "May Revolution Day",
        "Día de la Revolución de Mayo",
        Rule::gregorian(5, 25),
    )
    .years(Some(2011), None),
    // Güemes: a holiday from 2016 by Ley 27.258, carried from 2018, the
    // first year its rule can be stated from a text the author read.
    ar_trasladable(
        "Anniversary of the Passing of General Güemes",
        "Paso a la Inmortalidad del General Martín Miguel de Güemes",
        &AR_GUEMES,
    )
    .years(Some(2018), None),
    HolidayRule::fixed_public(
        "Anniversary of the Passing of General Belgrano",
        "Paso a la Inmortalidad del General Manuel Belgrano",
        Rule::gregorian(6, 20),
    )
    .years(Some(2011), None),
    HolidayRule::fixed_public(
        "Independence Day",
        "Día de la Independencia",
        Rule::gregorian(7, 9),
    )
    .years(Some(2011), None),
    // The three of Decreto 1584/2010, article 2: the third Monday of
    // August, the second of October and the fourth of November, 2011–2016;
    // then the weekday rule of Ley 27.399 from 2018.
    HolidayRule::public(
        "Anniversary of the Passing of General San Martín",
        "Paso a la Inmortalidad del General José de San Martín",
        Rule::nth(8, 3, Weekday::Monday),
    )
    .years(Some(2011), Some(2016)),
    ar_trasladable(
        "Anniversary of the Passing of General San Martín",
        "Paso a la Inmortalidad del General José de San Martín",
        &AR_SAN_MARTIN,
    )
    .years(Some(2018), None),
    HolidayRule::public(
        "Day of Respect for Cultural Diversity",
        "Día del Respeto a la Diversidad Cultural",
        Rule::nth(10, 2, Weekday::Monday),
    )
    .years(Some(2011), Some(2016)),
    ar_trasladable(
        "Day of Respect for Cultural Diversity",
        "Día del Respeto a la Diversidad Cultural",
        &AR_DIVERSIDAD,
    )
    .years(Some(2018), None),
    HolidayRule::public(
        "National Sovereignty Day",
        "Día de la Soberanía Nacional",
        Rule::nth(11, 4, Weekday::Monday),
    )
    .years(Some(2011), Some(2016)),
    ar_trasladable(
        "National Sovereignty Day",
        "Día de la Soberanía Nacional",
        &AR_SOBERANIA,
    )
    .years(Some(2018), None),
    HolidayRule::fixed_public(
        "Immaculate Conception",
        "Inmaculada Concepción de María",
        Rule::gregorian(12, 8),
    )
    .years(Some(2011), None),
    HolidayRule::fixed_public("Christmas Day", "Navidad", Rule::gregorian(12, 25))
        .years(Some(2011), None),
    // Days the observant may take off: Rosh Hashanah's two days, Yom
    // Kippur, the first two and last two days of Pesach; the Islamic New
    // Year, Eid al-Fitr and Eid al-Adha, which Argentina keeps on the
    // community's announcement and are therefore approximate.
    ar_religious(
        "Rosh Hashanah",
        "Año Nuevo Judío",
        Rule::in_calendar(CalendarSystem::HEBREW, 1, 1),
    ),
    ar_religious(
        "Rosh Hashanah",
        "Año Nuevo Judío",
        Rule::in_calendar(CalendarSystem::HEBREW, 1, 2),
    ),
    ar_religious(
        "Yom Kippur",
        "Día del Perdón",
        Rule::in_calendar(CalendarSystem::HEBREW, 1, 10),
    ),
    ar_religious(
        "Passover",
        "Pascua Judía",
        Rule::in_calendar(CalendarSystem::HEBREW, 7, 15),
    ),
    ar_religious(
        "Passover",
        "Pascua Judía",
        Rule::in_calendar(CalendarSystem::HEBREW, 7, 16),
    ),
    ar_religious(
        "Passover",
        "Pascua Judía",
        Rule::in_calendar(CalendarSystem::HEBREW, 7, 21),
    ),
    ar_religious(
        "Passover",
        "Pascua Judía",
        Rule::in_calendar(CalendarSystem::HEBREW, 7, 22),
    ),
    ar_religious(
        "Islamic New Year",
        "Año Nuevo Musulmán",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 1, 1),
    )
    .approximate(),
    ar_religious(
        "Eid al-Fitr",
        "Culminación del Ayuno",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 1),
    )
    .approximate(),
    ar_religious(
        "Eid al-Adha",
        "Fiesta del Sacrificio",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 10),
    )
    .approximate(),
];

/// Argentina.
///
/// The table begins with Decreto 1584/2010, in force from 2011, and states
/// nothing before: the earlier arrangement, in which Ley 24.445 (1995) put
/// 20 June, 17 August and 12 October on Mondays, is not carried, nor are
/// the years the fixed days were created (24 March by Ley 26.085 of 2006,
/// 2 April by Ley 25.370 of 2000). From 2011 the *inamovibles* stay where
/// they fall, weekend included, and the *trasladables* move: under the
/// decree, to the third Monday of August, the second of October and the
/// fourth of November, and under Ley 27.399 (2017) by the weekday rule of
/// its article 6 — Tuesday and Wednesday to the Monday before, Thursday and
/// Friday to the Monday after — which the table carries from 2018. The
/// 2017 dates, set by Decreto 52/2017, and Güemes's day in 2016 and 2017
/// are left unstated, since the author read neither text.
///
/// The *feriados con fines turísticos* — up to three a year, set by the
/// Executive fifty days ahead on a Monday or Friday — are annual and not
/// carried. Holy Thursday is a *día no laborable*, an optional day off,
/// and is stated as an observance, as are the days the observant of the
/// Jewish and Islamic faiths may take.
pub static ARGENTINA: RuleSet = RuleSet {
    code: "AR",
    english_name: "Argentina",
    rules: AR_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Decreto 1584/2010 (servicios.infoleg.gob.ar, retrieved \
              2026-09-22), articles 1, 2 and 5, for the 2011 list, the Monday \
              rules to 2016 and the days of the faiths; Ley 27.399 (2017), \
              articles 1, 2, 4, 6 and 7, for the list, the weekday rule and \
              the tourist holidays; Wikipedia (es), \"Día del Veterano y de \
              los Caídos en la Guerra de Malvinas\" and \"Martín Miguel de \
              Güemes\", retrieved 2026-09-22, for Ley 25.370 and Ley 27.258",
};

// ─────────────────────────────────────────────────────────────────────────
// Colombia
// ─────────────────────────────────────────────────────────────────────────

/// A holiday under article 1, paragraph 2 of Ley 51 de 1983: moved to the
/// following Monday whenever it does not fall on one.
const fn emiliani(name: &'static str, local: &'static str, base: &'static Rule) -> HolidayRule {
    HolidayRule::fixed_public(
        name,
        local,
        Rule::moved_by_weekday(base, TO_FOLLOWING_MONDAY),
    )
    .years(Some(1984), None)
}

/// A holiday under article 1 that stays on its date.
const fn co_fixed(name: &'static str, local: &'static str, rule: Rule) -> HolidayRule {
    HolidayRule::fixed_public(name, local, rule).years(Some(1984), None)
}

static CO_EPIPHANY: Rule = Rule::gregorian(1, 6);
static CO_SAINT_JOSEPH: Rule = Rule::gregorian(3, 19);
static CO_ASCENSION: Rule = Rule::easter(ASCENSION);
static CO_CORPUS_CHRISTI: Rule = Rule::easter(CORPUS_CHRISTI);
static CO_SACRED_HEART: Rule = Rule::easter(SACRED_HEART);
static CO_PETER_AND_PAUL: Rule = Rule::gregorian(6, 29);
static CO_ASSUMPTION: Rule = Rule::gregorian(8, 15);
static CO_COLUMBUS: Rule = Rule::gregorian(10, 12);
static CO_ALL_SAINTS: Rule = Rule::gregorian(11, 1);
static CO_CARTAGENA: Rule = Rule::gregorian(11, 11);

static CO_RULES: &[HolidayRule] = &[
    co_fixed("New Year's Day", "Año Nuevo", Rule::gregorian(1, 1)),
    emiliani("Epiphany", "Día de los Reyes Magos", &CO_EPIPHANY),
    emiliani("Saint Joseph's Day", "Día de San José", &CO_SAINT_JOSEPH),
    co_fixed(
        "Maundy Thursday",
        "Jueves Santo",
        Rule::easter(MAUNDY_THURSDAY),
    ),
    co_fixed("Good Friday", "Viernes Santo", Rule::easter(GOOD_FRIDAY)),
    co_fixed("Labour Day", "Día del Trabajo", Rule::gregorian(5, 1)),
    emiliani("Ascension Day", "Ascensión del Señor", &CO_ASCENSION),
    emiliani("Corpus Christi", "Corpus Christi", &CO_CORPUS_CHRISTI),
    emiliani("Sacred Heart", "Sagrado Corazón de Jesús", &CO_SACRED_HEART),
    emiliani(
        "Saints Peter and Paul",
        "San Pedro y San Pablo",
        &CO_PETER_AND_PAUL,
    ),
    co_fixed(
        "Independence Day",
        "Día de la Independencia",
        Rule::gregorian(7, 20),
    ),
    co_fixed(
        "Battle of Boyacá",
        "Batalla de Boyacá",
        Rule::gregorian(8, 7),
    ),
    emiliani(
        "Assumption of Mary",
        "Asunción de la Virgen",
        &CO_ASSUMPTION,
    ),
    emiliani("Columbus Day", "Día de la Raza", &CO_COLUMBUS),
    emiliani("All Saints' Day", "Día de Todos los Santos", &CO_ALL_SAINTS),
    emiliani(
        "Independence of Cartagena",
        "Independencia de Cartagena",
        &CO_CARTAGENA,
    ),
    co_fixed(
        "Immaculate Conception",
        "Inmaculada Concepción",
        Rule::gregorian(12, 8),
    ),
    co_fixed("Christmas Day", "Navidad", Rule::gregorian(12, 25)),
];

/// Colombia.
///
/// Ley 51 de 1983, the *Ley Emiliani*: eighteen days of paid rest, ten of
/// which — Epiphany, Saint Joseph, Ascension, Corpus Christi, the Sacred
/// Heart, Saints Peter and Paul, the Assumption, 12 October, All Saints
/// and the Independence of Cartagena — "cuando no caigan en día lunes se
/// trasladarán al lunes siguiente", a Sunday included. The other eight stay
/// where they fall. The table begins with the law's first full year, 1984,
/// and states nothing before.
pub static COLOMBIA: RuleSet = RuleSet {
    code: "CO",
    english_name: "Colombia",
    rules: CO_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Ley 51 de 1983, article 1 (funcionpublica.gov.co, gestor \
              normativo, retrieved 2026-09-22), for the list and the Monday \
              rule; Wikipedia (es), \"Anexo:Días festivos en Colombia\", \
              retrieved 2026-09-22, for the 2026 dates the tests check",
};

// ─────────────────────────────────────────────────────────────────────────
// Peru
// ─────────────────────────────────────────────────────────────────────────

/// A national holiday as the source lists it for 2026, carried from 2024.
const fn pe(name: &'static str, local: &'static str, rule: Rule) -> HolidayRule {
    HolidayRule::fixed_public(name, local, rule).years(Some(2024), None)
}

static PE_RULES: &[HolidayRule] = &[
    pe("New Year's Day", "Año Nuevo", Rule::gregorian(1, 1)),
    pe(
        "Maundy Thursday",
        "Jueves Santo",
        Rule::easter(MAUNDY_THURSDAY),
    ),
    pe("Good Friday", "Viernes Santo", Rule::easter(GOOD_FRIDAY)),
    pe("Labour Day", "Día del Trabajo", Rule::gregorian(5, 1)),
    pe("Flag Day", "Día de la Bandera", Rule::gregorian(6, 7)),
    pe(
        "Saints Peter and Paul",
        "San Pedro y San Pablo",
        Rule::gregorian(6, 29),
    ),
    pe(
        "Air Force Day",
        "Día de la Fuerza Aérea",
        Rule::gregorian(7, 23),
    ),
    pe(
        "Independence Day",
        "Fiestas Patrias",
        Rule::gregorian(7, 28),
    ),
    pe(
        "Independence Day",
        "Fiestas Patrias",
        Rule::gregorian(7, 29),
    ),
    pe("Battle of Junín", "Batalla de Junín", Rule::gregorian(8, 6)),
    pe(
        "Saint Rose of Lima",
        "Santa Rosa de Lima",
        Rule::gregorian(8, 30),
    ),
    pe(
        "Battle of Angamos",
        "Combate de Angamos",
        Rule::gregorian(10, 8),
    ),
    pe(
        "All Saints' Day",
        "Día de Todos los Santos",
        Rule::gregorian(11, 1),
    ),
    pe(
        "Immaculate Conception",
        "Inmaculada Concepción",
        Rule::gregorian(12, 8),
    ),
    pe(
        "Battle of Ayacucho",
        "Batalla de Ayacucho",
        Rule::gregorian(12, 9),
    ),
    pe("Christmas Day", "Navidad", Rule::gregorian(12, 25)),
];

/// Peru.
///
/// The national holidays of Decreto Legislativo 713 as the source lists
/// them for 2026, sixteen days including the two of Fiestas Patrias. Four
/// of them are additions of the 2020s — Flag Day, Air Force Day, the
/// Battle of Junín and the Battle of Ayacucho — under laws the author could
/// not read, so the table begins in 2024, the first year all sixteen were
/// kept, and states nothing before. A holiday on a weekend stays there, and
/// the *días no laborables* the government declares each year by decree
/// are not carried.
pub static PERU: RuleSet = RuleSet {
    code: "PE",
    english_name: "Peru",
    rules: PE_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "Decreto Legislativo 713, art. 6, as amended by Leyes 31381 (2021), 31530 (2022), \
              31788 (2023) and 31822 (2023), not read, El Peruano and gob.pe refusing access; \
              the Spanish Wikipedia, \"Anexo:Días festivos en Perú\" (secondary), retrieved \
              2026-09-22, which tabulates the 2026 holidays under the decree and marks the \
              irrenunciable ones",
};

// ─────────────────────────────────────────────────────────────────────────
// Bolivia
// ─────────────────────────────────────────────────────────────────────────

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
];

/// Bolivia.
///
/// The national holidays of Decreto Supremo 2750 of 1 May 2016, with
/// 22 January from Decreto Supremo 405 of 2010 and 21 June from Decreto
/// Supremo 173 of 2009, and the decree's Sunday rule, which the four
/// holidays it names sit outside. The departmental holidays, to which
/// Decreto Supremo 5019 of 2023 extended the Sunday rule, are not carried;
/// nor are the bridges and moves the Government decrees year by year, such
/// as 2026's Friday 23 January. The Sunday rule is carried from the 2016
/// decree, and whatever earlier decrees did is not.
pub static BOLIVIA: RuleSet = RuleSet {
    code: "BO",
    english_name: "Bolivia",
    rules: BO_RULES,
    substitution: BO_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Decreto Supremo 2750 of 1 May 2016, arts. 2 and 3, and Decreto Supremo \
              5019 of 13 September 2023, lexivox.org, retrieved 2026-09-22; Decreto \
              Supremo 173 of 17 June 2009 for 21 June, lexivox.org; Decreto Supremo \
              405 of 20 January 2010 for 22 January, as reported by the Ministry of \
              Labour; Wikipedia, \"Public holidays in Bolivia\", retrieved the same \
              day, for the English names",
};

// ─────────────────────────────────────────────────────────────────────────
// Chile
// ─────────────────────────────────────────────────────────────────────────

/// Chile's standard time, UTC−4, at which the June solstice is dated.
const CHILE_STANDARD_TIME: Meridian = Meridian::from_seconds(-4 * 3_600);

/// Ley 19.668 (2000): a Tuesday, Wednesday or Thursday holiday goes to the
/// Monday before, a Friday one to the Monday after.
const CL_TO_MONDAY: &[(Weekday, i16)] = &[
    (Weekday::Tuesday, -1),
    (Weekday::Wednesday, -2),
    (Weekday::Thursday, -3),
    (Weekday::Friday, 3),
];

/// Ley 20.299 (2008): 31 October goes to the Friday before when a Tuesday
/// and to the Friday after when a Wednesday.
const CL_TO_FRIDAY: &[(Weekday, i16)] = &[(Weekday::Tuesday, -4), (Weekday::Wednesday, 2)];

static CL_JUNE_29: Rule = Rule::gregorian(6, 29);
static CL_OCTOBER_12: Rule = Rule::gregorian(10, 12);
static CL_OCTOBER_31: Rule = Rule::gregorian(10, 31);
static CL_CORPUS_CHRISTI: Rule = Rule::easter(CORPUS_CHRISTI);

/// A fixed date that is a holiday only in the years it falls on `weekday`.
fn cl_when(year: i64, month: u8, day: u8, weekday: Weekday) -> Days {
    match gregorian::to_fixed(year, month, day) {
        Ok(rd) if Weekday::from_rd(rd) == weekday => Days::one(rd),
        _ => Days::new(),
    }
}

fn cl_january_2_monday(year: i64) -> Days {
    cl_when(year, 1, 2, Weekday::Monday)
}

fn cl_september_17_monday(year: i64) -> Days {
    cl_when(year, 9, 17, Weekday::Monday)
}

fn cl_september_17_friday(year: i64) -> Days {
    cl_when(year, 9, 17, Weekday::Friday)
}

fn cl_september_20_friday(year: i64) -> Days {
    cl_when(year, 9, 20, Weekday::Friday)
}

static CL_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Año Nuevo", Rule::gregorian(1, 1)),
    // Ley 20.983: a Monday 2 January is a holiday.
    HolidayRule::fixed_public(
        "Monday after New Year's Day",
        "Lunes 2 de enero",
        Rule::Computed(cl_january_2_monday),
    )
    .years(Some(2017), None),
    HolidayRule::fixed_public("Good Friday", "Viernes Santo", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Holy Saturday", "Sábado Santo", Rule::easter(HOLY_SATURDAY)),
    HolidayRule::fixed_public(
        "Labour Day",
        "Día Nacional del Trabajo",
        Rule::gregorian(5, 1),
    ),
    HolidayRule::fixed_public(
        "Navy Day",
        "Día de las Glorias Navales",
        Rule::gregorian(5, 21),
    ),
    HolidayRule::fixed_public(
        "Battle of Arica Day",
        "Asalto y Toma del Morro de Arica",
        Rule::gregorian(6, 7),
    )
    .in_regions(&["CL-AP"])
    .years(Some(2013), None),
    // Ley 21.357: the day of the June solstice, and 21 June in 2021 by its
    // transitional article.
    HolidayRule::fixed_public(
        "National Indigenous Peoples' Day",
        "Día Nacional de los Pueblos Indígenas",
        Rule::gregorian(6, 21),
    )
    .years(Some(2021), Some(2021)),
    HolidayRule::fixed_public(
        "National Indigenous Peoples' Day",
        "Día Nacional de los Pueblos Indígenas",
        Rule::SolarTerm {
            term: SolarTerm::SUMMER_SOLSTICE,
            meridian: CHILE_STANDARD_TIME,
        },
    )
    .years(Some(2022), None),
    // Saints Peter and Paul: abolished in 1968, back from 1986, moved to a
    // Monday from 2000.
    HolidayRule::fixed_public(
        "Saints Peter and Paul",
        "San Pedro y San Pablo",
        Rule::gregorian(6, 29),
    )
    .years(Some(1986), Some(1999)),
    HolidayRule::fixed_public(
        "Saints Peter and Paul",
        "San Pedro y San Pablo",
        Rule::moved_by_weekday(&CL_JUNE_29, CL_TO_MONDAY),
    )
    .years(Some(2000), None),
    // Corpus Christi: back from 1987, moved to the Monday before from 2000,
    // replaced by Our Lady of Mount Carmel from 2007.
    HolidayRule::fixed_public(
        "Corpus Christi",
        "Corpus Christi",
        Rule::easter(CORPUS_CHRISTI),
    )
    .years(Some(1987), Some(1999)),
    HolidayRule::fixed_public(
        "Corpus Christi",
        "Corpus Christi",
        Rule::moved_by_weekday(&CL_CORPUS_CHRISTI, CL_TO_MONDAY),
    )
    .years(Some(2000), Some(2006)),
    HolidayRule::fixed_public(
        "Our Lady of Mount Carmel",
        "Virgen del Carmen",
        Rule::gregorian(7, 16),
    )
    .years(Some(2007), None),
    HolidayRule::fixed_public(
        "Assumption of Mary",
        "Asunción de la Virgen",
        Rule::gregorian(8, 15),
    ),
    HolidayRule::fixed_public(
        "Day of National Liberation",
        "Día de la Liberación Nacional",
        Rule::gregorian(9, 11),
    )
    .years(Some(1981), Some(1998)),
    HolidayRule::fixed_public(
        "National Unity Day",
        "Día de la Unidad Nacional",
        Rule::nth(9, 1, Weekday::Monday),
    )
    .years(Some(1999), Some(2001)),
    // Leyes 20.215 and 20.983: a Monday or Friday 17 September and a Friday
    // 20 September are holidays.
    HolidayRule::fixed_public(
        "Monday 17 September",
        "Lunes 17 de septiembre",
        Rule::Computed(cl_september_17_monday),
    )
    .years(Some(2007), None),
    HolidayRule::fixed_public(
        "Friday 17 September",
        "Viernes 17 de septiembre",
        Rule::Computed(cl_september_17_friday),
    )
    .years(Some(2017), None),
    HolidayRule::fixed_public(
        "Independence Day",
        "Día de la Independencia Nacional",
        Rule::gregorian(9, 18),
    ),
    HolidayRule::fixed_public(
        "Army Day",
        "Día de las Glorias del Ejército",
        Rule::gregorian(9, 19),
    ),
    HolidayRule::fixed_public(
        "Friday 20 September",
        "Viernes 20 de septiembre",
        Rule::Computed(cl_september_20_friday),
    )
    .years(Some(2007), None),
    // 12 October: a holiday from 1922, moved to a Monday and renamed from 2000.
    HolidayRule::fixed_public(
        "Discovery of America Anniversary",
        "Aniversario del Descubrimiento de América",
        Rule::gregorian(10, 12),
    )
    .years(Some(1922), Some(1999)),
    HolidayRule::fixed_public(
        "Meeting of Two Worlds Day",
        "Día del Encuentro de Dos Mundos",
        Rule::moved_by_weekday(&CL_OCTOBER_12, CL_TO_MONDAY),
    )
    .years(Some(2000), None),
    HolidayRule::fixed_public(
        "National Day of the Evangelical and Protestant Churches",
        "Día Nacional de las Iglesias Evangélicas y Protestantes",
        Rule::moved_by_weekday(&CL_OCTOBER_31, CL_TO_FRIDAY),
    )
    .years(Some(2008), None),
    HolidayRule::fixed_public(
        "All Saints' Day",
        "Día de Todos los Santos",
        Rule::gregorian(11, 1),
    ),
    HolidayRule::fixed_public(
        "Immaculate Conception",
        "Inmaculada Concepción",
        Rule::gregorian(12, 8),
    ),
    HolidayRule::fixed_public(
        "Christmas Day",
        "Natividad del Señor",
        Rule::gregorian(12, 25),
    ),
];

/// Chile.
///
/// The national holidays as the laws set them, each move a rule of its
/// own rather than a substitution policy: Ley 19.668 of 2000 sends Saints
/// Peter and Paul and 12 October, and until 2006 Corpus Christi, to the
/// Monday before from a Tuesday, Wednesday or Thursday and the Monday
/// after from a Friday; Ley 20.299 of 2008 sends 31 October to the Friday
/// before from a Tuesday and the Friday after from a Wednesday; and
/// Leyes 20.215 and 20.983 make a Monday or Friday 17 September, a Friday
/// 20 September and a Monday 2 January holidays in the years they occur,
/// which four computed rules carry. Ley 21.357 of 2021 puts the National
/// Indigenous Peoples' Day on the June solstice, dated at Chile's standard
/// time, and on 21 June in 2021 by its transitional article. The Battle of
/// Arica is the Arica and Parinacota Region's alone, `CL-AP`. The changes
/// the sources date are carried as years: the Day of National Liberation
/// from 1981 to 1998 and the National Unity Day from 1999 to 2001, Corpus
/// Christi from 1987 to 2006 and Our Lady of Mount Carmel from 2007. Not
/// carried: election and census days, which each law or decree sets, and
/// the commune-level holiday of Chillán and Chillán Viejo. New Year's Day,
/// Labour Day, 18 and 19 September and Christmas are irrenunciable under
/// Leyes 19.973 and 20.215, which the crate has no field for.
pub static CHILE: RuleSet = RuleSet {
    code: "CL",
    english_name: "Chile",
    rules: CL_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "Ley 2.977 (1915), Ley 19.668 (2000), Ley 20.215 (2007; Código del Trabajo art. \
              35 ter), Ley 20.299 (2008), Ley 20.983 (2016) and Ley 21.357 (2021), on \
              LeyChile, Biblioteca del Congreso Nacional \
              (leychile.cl/Consulta/obtxml?opt=7&idNorma=23639, 160270, 264651, 279294, \
              1098384 and 1161743), retrieved 2026-09-26; Leyes 3.810, 18.432, 20.148, 20.629 \
              and 20.663 as feriadoschilenos.cl lists them, not read; Wikipedia, \"Public \
              holidays in Chile\", and the Spanish Wikipedia, \"Anexo:Días feriados en \
              Chile\", retrieved 2026-09-22, for the years before 2000",
};

// ─────────────────────────────────────────────────────────────────────────
// Ecuador
// ─────────────────────────────────────────────────────────────────────────

/// The 2016 law's moves: a Tuesday holiday to the Monday before, a
/// Wednesday or Thursday one to the Friday of its week, a Saturday one to
/// the Friday before and a Sunday one to the Monday after.
const EC_MOVES: &[(Weekday, i16)] = &[
    (Weekday::Tuesday, -1),
    (Weekday::Wednesday, 2),
    (Weekday::Thursday, 1),
    (Weekday::Saturday, -1),
    (Weekday::Sunday, 1),
];

/// The same for the three days the law excepts from the weekday moves but
/// not from the weekend ones.
const EC_WEEKEND_MOVES: &[(Weekday, i16)] = &[(Weekday::Saturday, -1), (Weekday::Sunday, 1)];

static EC_JANUARY_1: Rule = Rule::gregorian(1, 1);
static EC_MAY_1: Rule = Rule::gregorian(5, 1);
static EC_MAY_24: Rule = Rule::gregorian(5, 24);
static EC_AUGUST_10: Rule = Rule::gregorian(8, 10);
static EC_OCTOBER_9: Rule = Rule::gregorian(10, 9);
static EC_DECEMBER_25: Rule = Rule::gregorian(12, 25);

/// Where a fixed date lands under `EC_MOVES`.
fn ec_moved(year: i64, month: u8, day: u8) -> Option<(Rd, Rd)> {
    let rd = gregorian::to_fixed(year, month, day).ok()?;
    let shift = EC_MOVES
        .iter()
        .find(|(trigger, _)| *trigger == Weekday::from_rd(rd))
        .map_or(0, |(_, days)| i64::from(*days));
    Some((rd, Rd(rd.0 + shift)))
}

/// 2 and 3 November, which the law moves like any other day and which
/// therefore sometimes land on each other, or one on the other's own day.
/// The law is silent; the Government's calendars have resolved it the
/// same way each time, and this does what they did. The unmoved one keeps
/// its day, and when both move 3 November keeps its target; the blocked
/// one takes the free day beside the target on the working side of the
/// weekend, the Thursday before a Friday or the Tuesday after a Monday:
/// Thursday 1 November 2018, Thursday 3 and Friday 4 November 2022,
/// Thursday 2 and Friday 3 November 2023, Monday 3 and Tuesday 4 November
/// 2025, and Monday 2 and Tuesday 3 November 2026.
fn ec_november_pair(year: i64) -> Option<(Rd, Rd)> {
    let (second, second_moved) = ec_moved(year, 11, 2)?;
    let (_, third_moved) = ec_moved(year, 11, 3)?;
    if second_moved != third_moved {
        return Some((second_moved, third_moved));
    }
    let beside = |target: Rd| {
        if Weekday::from_rd(target) == Weekday::Friday {
            Rd(target.0 - 1)
        } else {
            Rd(target.0 + 1)
        }
    };
    if second_moved == second {
        Some((second, beside(third_moved)))
    } else {
        Some((beside(second_moved), third_moved))
    }
}

fn ec_november_2(year: i64) -> Days {
    ec_november_pair(year).map_or_else(Days::new, |(day, _)| Days::one(day))
}

fn ec_november_3(year: i64) -> Days {
    ec_november_pair(year).map_or_else(Days::new, |(_, day)| Days::one(day))
}

static EC_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Año Nuevo", Rule::gregorian(1, 1))
        .years(None, Some(2016)),
    HolidayRule::fixed_public(
        "New Year's Day",
        "Año Nuevo",
        Rule::moved_by_weekday(&EC_JANUARY_1, EC_WEEKEND_MOVES),
    )
    .years(Some(2017), None),
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
    HolidayRule::fixed_public("Labour Day", "Día del Trabajo", Rule::gregorian(5, 1))
        .years(None, Some(2016)),
    HolidayRule::fixed_public(
        "Labour Day",
        "Día del Trabajo",
        Rule::moved_by_weekday(&EC_MAY_1, EC_MOVES),
    )
    .years(Some(2017), None),
    HolidayRule::fixed_public(
        "Battle of Pichincha",
        "Batalla de Pichincha",
        Rule::gregorian(5, 24),
    )
    .years(None, Some(2016)),
    HolidayRule::fixed_public(
        "Battle of Pichincha",
        "Batalla de Pichincha",
        Rule::moved_by_weekday(&EC_MAY_24, EC_MOVES),
    )
    .years(Some(2017), None),
    HolidayRule::fixed_public(
        "First Cry of Independence",
        "Primer Grito de Independencia",
        Rule::gregorian(8, 10),
    )
    .years(None, Some(2016)),
    HolidayRule::fixed_public(
        "First Cry of Independence",
        "Primer Grito de Independencia",
        Rule::moved_by_weekday(&EC_AUGUST_10, EC_MOVES),
    )
    .years(Some(2017), None),
    HolidayRule::fixed_public(
        "Independence of Guayaquil",
        "Independencia de Guayaquil",
        Rule::gregorian(10, 9),
    )
    .years(None, Some(2016)),
    HolidayRule::fixed_public(
        "Independence of Guayaquil",
        "Independencia de Guayaquil",
        Rule::moved_by_weekday(&EC_OCTOBER_9, EC_MOVES),
    )
    .years(Some(2017), None),
    HolidayRule::fixed_public(
        "Day of the Dead",
        "Día de los Difuntos",
        Rule::gregorian(11, 2),
    )
    .years(None, Some(2016)),
    HolidayRule::fixed_public(
        "Day of the Dead",
        "Día de los Difuntos",
        Rule::Computed(ec_november_2),
    )
    .years(Some(2017), None),
    HolidayRule::fixed_public(
        "Independence of Cuenca",
        "Independencia de Cuenca",
        Rule::gregorian(11, 3),
    )
    .years(None, Some(2016)),
    HolidayRule::fixed_public(
        "Independence of Cuenca",
        "Independencia de Cuenca",
        Rule::Computed(ec_november_3),
    )
    .years(Some(2017), None),
    HolidayRule::fixed_public("Christmas Day", "Navidad", Rule::gregorian(12, 25))
        .years(None, Some(2016)),
    HolidayRule::fixed_public(
        "Christmas Day",
        "Navidad",
        Rule::moved_by_weekday(&EC_DECEMBER_25, EC_WEEKEND_MOVES),
    )
    .years(Some(2017), None),
];

/// Ecuador.
///
/// Article 65 of the Código del Trabajo as reformed by the law of
/// Registro Oficial 906 of 20 December 2016: the days of obligatory rest,
/// and the moves the law makes to them, each a rule of its own. A Tuesday
/// holiday goes to the Monday before, a Wednesday or Thursday one to the
/// Friday of its week, a Saturday one to the Friday before and a Sunday
/// one to the Monday after; New Year's Day, Christmas and Carnival Tuesday
/// are excepted from the weekday moves but not from the weekend ones, so
/// that a Saturday 1 January is kept on the Friday before, across the New
/// Year. The law is silent on 2 and 3 November landing on each other, and
/// the pair's computed rules do what the Government's calendars of 2023,
/// 2025 and 2026 did. The table is complete from 2017; before, the dates
/// are carried fixed and the bridges the Government decreed are not. The
/// local holidays of cantons and provinces are not carried.
pub static ECUADOR: RuleSet = RuleSet {
    code: "EC",
    english_name: "Ecuador",
    rules: EC_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Ley Orgánica Reformatoria a la Ley Orgánica del Servicio Público y al \
              Código del Trabajo, Registro Oficial 906 of 20 December 2016, as \
              published by the Municipality of Santo Domingo, for the moves and \
              the exceptions; El Universo, 20 December 2016, for the list and the \
              2017 dates; the Spanish Wikipedia, \"Anexo:Días festivos en \
              Ecuador\", retrieved 2026-09-22, for the moved dates of 2023 to 2025; \
              El Universo's official 2026 calendar for 2 and 3 November 2026",
};

// ─────────────────────────────────────────────────────────────────────────
// Uruguay
// ─────────────────────────────────────────────────────────────────────────

static UY_APRIL_19: Rule = Rule::gregorian(4, 19);
static UY_MAY_18: Rule = Rule::gregorian(5, 18);
static UY_OCTOBER_12: Rule = Rule::gregorian(10, 12);

/// A feriado común: banks and public offices close, private employers may
/// require work.
const fn uy_common(name: &'static str, local: &'static str, rule: Rule) -> HolidayRule {
    HolidayRule::fixed_public(name, local, rule).of_kind(Kind::Bank)
}

static UY_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Año Nuevo", Rule::gregorian(1, 1)),
    uy_common("Children's Day", "Día de los Niños", Rule::gregorian(1, 6)),
    uy_common(
        "Carnival Monday",
        "Lunes de Carnaval",
        Rule::easter(SHROVE_MONDAY),
    ),
    uy_common(
        "Carnival Tuesday",
        "Martes de Carnaval",
        Rule::easter(SHROVE_TUESDAY),
    ),
    uy_common("Tourism Week", "Semana de Turismo", Rule::easter(-6)),
    uy_common("Tourism Week", "Semana de Turismo", Rule::easter(-5)),
    uy_common("Tourism Week", "Semana de Turismo", Rule::easter(-4)),
    uy_common("Tourism Week", "Semana de Turismo", Rule::easter(-3)),
    uy_common("Tourism Week", "Semana de Turismo", Rule::easter(-2)),
    uy_common("Tourism Week", "Semana de Turismo", Rule::easter(-1)),
    uy_common(
        "Landing of the Thirty-Three Orientals",
        "Desembarco de los Treinta y Tres Orientales",
        Rule::moved_by_weekday(&UY_APRIL_19, TO_ADJACENT_MONDAY),
    )
    .years(Some(2002), None),
    HolidayRule::fixed_public(
        "Workers' Day",
        "Día de los Trabajadores",
        Rule::gregorian(5, 1),
    ),
    uy_common(
        "Battle of Las Piedras",
        "Batalla de Las Piedras",
        Rule::moved_by_weekday(&UY_MAY_18, TO_ADJACENT_MONDAY),
    )
    .years(Some(2002), None),
    uy_common(
        "Birth of Artigas",
        "Natalicio de Artigas",
        Rule::gregorian(6, 19),
    ),
    HolidayRule::fixed_public(
        "Constitution Day",
        "Jura de la Constitución",
        Rule::gregorian(7, 18),
    ),
    HolidayRule::fixed_public(
        "Independence Day",
        "Declaratoria de la Independencia",
        Rule::gregorian(8, 25),
    ),
    uy_common(
        "Day of the Race",
        "Día de la Raza",
        Rule::moved_by_weekday(&UY_OCTOBER_12, TO_ADJACENT_MONDAY),
    )
    .years(Some(2002), None),
    uy_common(
        "All Souls' Day",
        "Día de los Difuntos",
        Rule::gregorian(11, 2),
    ),
    HolidayRule::fixed_public("Family Day", "Día de la Familia", Rule::gregorian(12, 25)),
];

/// Uruguay.
///
/// Ley 16.805 of 24 December 1996 as amended by Ley 17.414 of 8 November
/// 2001: the five paid holidays on which work stops, `Kind::Public`, and
/// the common holidays, `Kind::Bank`, on which banks and public offices
/// close and a private employer may require work — the six days of
/// Tourism Week among them. Three holidays move: 19 April, 18 May and
/// 12 October go to the Monday before from a Tuesday or Wednesday and the
/// Monday after from a Thursday or Friday, the same table Argentina uses;
/// the rest the amended law names as immovable. The three moved rules
/// start in 2002, the first year under the amendment; what the 1996 law
/// moved before it is not carried, and the sector holidays and the
/// one-off days are not carried either.
pub static URUGUAY: RuleSet = RuleSet {
    code: "UY",
    english_name: "Uruguay",
    rules: UY_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "Ley N° 16.805 de 24 de diciembre de 1996, arts. 1 and 2, art. 2 as worded by Ley \
              N° 17.414 de 8 de noviembre de 2001, and Ley N° 12.590 de 23 de diciembre de \
              1958, art. 18, on IMPO (impo.com.uy/bases/leyes/16805-1996, 17414-2001 and \
              12590-1958), retrieved 2026-09-26; the Spanish Wikipedia, \"Días feriados de \
              Uruguay\", retrieved 2026-09-22, for Carnival and Tourism Week; Wikipedia, \
              \"Public holidays in Uruguay\", for the English names",
};

// ─────────────────────────────────────────────────────────────────────────
// Costa Rica
// ─────────────────────────────────────────────────────────────────────────

/// A fixed holiday under the transitional provision Ley 9875 added to
/// article 148: on the Monday the provision names in the years it names,
/// and on its own date otherwise.
fn cr_scheduled(year: i64, month: u8, day: u8, schedule: &[(i64, u8, u8)]) -> Days {
    let (m, d) = schedule
        .iter()
        .find(|(scheduled_year, _, _)| *scheduled_year == year)
        .map_or((month, day), |(_, m, d)| (*m, *d));
    gregorian::to_fixed(year, m, d).map_or_else(|_| Days::new(), Days::one)
}

fn cr_juan_santamaria(year: i64) -> Days {
    cr_scheduled(year, 4, 11, &[(2023, 4, 10), (2024, 4, 15)])
}

fn cr_labour_day(year: i64) -> Days {
    cr_scheduled(year, 5, 1, &[(2021, 5, 3)])
}

fn cr_nicoya(year: i64) -> Days {
    cr_scheduled(
        year,
        7,
        25,
        &[(2020, 7, 27), (2021, 7, 26), (2023, 7, 24), (2024, 7, 29)],
    )
}

fn cr_mothers_day(year: i64) -> Days {
    cr_scheduled(year, 8, 15, &[(2020, 8, 17), (2023, 8, 14)])
}

fn cr_independence_day(year: i64) -> Days {
    cr_scheduled(year, 9, 15, &[(2020, 9, 14), (2021, 9, 13), (2022, 9, 19)])
}

fn cr_army_abolition_day(year: i64) -> Days {
    cr_scheduled(
        year,
        12,
        1,
        &[(2020, 11, 30), (2021, 11, 29), (2022, 12, 5)],
    )
}

static CR_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Año Nuevo", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Maundy Thursday",
        "Jueves Santo",
        Rule::easter(MAUNDY_THURSDAY),
    ),
    HolidayRule::fixed_public("Good Friday", "Viernes Santo", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public(
        "Juan Santamaría Day",
        "Día de Juan Santamaría",
        Rule::Computed(cr_juan_santamaria),
    ),
    HolidayRule::fixed_public(
        "Labour Day",
        "Día Internacional del Trabajo",
        Rule::Computed(cr_labour_day),
    ),
    HolidayRule::fixed_public(
        "Annexation of the Party of Nicoya",
        "Anexión del Partido de Nicoya a Costa Rica",
        Rule::Computed(cr_nicoya),
    ),
    // Unpaid: article 148 makes the day off obligatory and its pay not.
    HolidayRule::fixed_public(
        "Feast of Our Lady of the Angels",
        "Día de la Virgen de los Ángeles",
        Rule::gregorian(8, 2),
    ),
    HolidayRule::fixed_public(
        "Mother's Day",
        "Día de la Madre",
        Rule::Computed(cr_mothers_day),
    ),
    HolidayRule::fixed_public(
        "Day of the Black Person and Afro-Costa Rican Culture",
        "Día de la Persona Negra y la Cultura Afrocostarricense",
        Rule::gregorian(8, 31),
    )
    .years(Some(2022), None),
    HolidayRule::fixed_public(
        "Independence Day",
        "Día de la Independencia",
        Rule::Computed(cr_independence_day),
    ),
    HolidayRule::fixed_public(
        "Day of the Cultures",
        "Día de las Culturas",
        Rule::gregorian(10, 12),
    )
    .years(None, Some(2019)),
    HolidayRule::fixed_public(
        "Army Abolition Day",
        "Día de la Abolición del Ejército",
        Rule::Computed(cr_army_abolition_day),
    )
    .years(Some(2020), None),
    HolidayRule::fixed_public("Christmas Day", "Navidad", Rule::gregorian(12, 25)),
];

/// Costa Rica.
///
/// Article 148 of the Código de Trabajo: the paid holidays, and the three
/// whose pay the article does not oblige — 2 August, 31 August from 2022
/// under Ley 10050, and 1 December from 2020 under Ley 9803, which took
/// 12 October's place. The transitional provision of Ley 9875, amended by
/// Leyes 10050 and 10396, moved named holidays to named Mondays from 2020
/// to 2024 to help tourism, and the six holidays it touched are computed
/// rules that give the provision's Monday in those years and their own
/// date in every other: Mother's Day stayed on Thursday 15 August 2024 by
/// Ley 10396, and nothing moves from 2025. A holiday on a weekend stays
/// there.
pub static COSTA_RICA: RuleSet = RuleSet {
    code: "CR",
    english_name: "Costa Rica",
    rules: CR_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Código de Trabajo art. 148 as reformed by Ley 9803 (2020) and Ley 10050 \
              (2021), per the Ministry of Labour's communiqués of October 2020 and \
              August 2022; Ley 9875 (2020) with its reforms, as reported by La \
              Nación on the 2020 and 2021 Mondays and by Wikipedia, \"Public \
              holidays in Costa Rica\", retrieved 2026-09-22, for every year's \
              moved dates; Ley 10396 (2023) for 15 August 2024",
};

// ─────────────────────────────────────────────────────────────────────────
// Dominican Republic
// ─────────────────────────────────────────────────────────────────────────

/// Ley 139-97, article 4's paragraph: a Sunday 1 May gives the Monday, on
/// top of article 1's moves.
const DO_LABOUR_MOVES: &[(Weekday, i16)] = &[
    (Weekday::Tuesday, -1),
    (Weekday::Wednesday, -2),
    (Weekday::Thursday, 4),
    (Weekday::Friday, 3),
    (Weekday::Sunday, 1),
];

static DO_JANUARY_6: Rule = Rule::gregorian(1, 6);
static DO_JANUARY_26: Rule = Rule::gregorian(1, 26);
static DO_MAY_1: Rule = Rule::gregorian(5, 1);
static DO_AUGUST_16: Rule = Rule::gregorian(8, 16);
static DO_NOVEMBER_6: Rule = Rule::gregorian(11, 6);
static DO_AUGUST_16_MOVED: Rule = Rule::moved_by_weekday(&DO_AUGUST_16, TO_ADJACENT_MONDAY);

/// Restoration Day, moved like the others except when 16 August opens a
/// constitutional period, which it does every fourth year.
fn do_restoration_day(year: i64) -> Days {
    if year % 4 == 0 {
        DO_AUGUST_16.days_in_year(year)
    } else {
        DO_AUGUST_16_MOVED.days_in_year(year)
    }
}

static DO_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Año Nuevo", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("Epiphany", "Día de los Santos Reyes", Rule::gregorian(1, 6))
        .years(None, Some(1997)),
    HolidayRule::fixed_public(
        "Epiphany",
        "Día de los Santos Reyes",
        Rule::moved_by_weekday(&DO_JANUARY_6, TO_ADJACENT_MONDAY),
    )
    .years(Some(1998), None),
    HolidayRule::fixed_public(
        "Our Lady of Altagracia",
        "Día de Nuestra Señora de la Altagracia",
        Rule::gregorian(1, 21),
    ),
    HolidayRule::fixed_public("Duarte Day", "Día de Duarte", Rule::gregorian(1, 26))
        .years(None, Some(1997)),
    HolidayRule::fixed_public(
        "Duarte Day",
        "Día de Duarte",
        Rule::moved_by_weekday(&DO_JANUARY_26, TO_ADJACENT_MONDAY),
    )
    .years(Some(1998), None),
    HolidayRule::fixed_public(
        "Independence Day",
        "Día de la Independencia Nacional",
        Rule::gregorian(2, 27),
    ),
    HolidayRule::fixed_public("Good Friday", "Viernes Santo", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Labour Day", "Día del Trabajo", Rule::gregorian(5, 1))
        .years(None, Some(1997)),
    HolidayRule::fixed_public(
        "Labour Day",
        "Día del Trabajo",
        Rule::moved_by_weekday(&DO_MAY_1, DO_LABOUR_MOVES),
    )
    .years(Some(1998), None),
    HolidayRule::fixed_public(
        "Corpus Christi",
        "Corpus Christi",
        Rule::easter(CORPUS_CHRISTI),
    ),
    HolidayRule::fixed_public(
        "Restoration Day",
        "Día de la Restauración",
        Rule::gregorian(8, 16),
    )
    .years(None, Some(1996)),
    HolidayRule::fixed_public(
        "Restoration Day",
        "Día de la Restauración",
        Rule::Computed(do_restoration_day),
    )
    .years(Some(1997), None),
    HolidayRule::fixed_public(
        "Our Lady of Mercy",
        "Día de Nuestra Señora de las Mercedes",
        Rule::gregorian(9, 24),
    ),
    HolidayRule::fixed_public(
        "Constitution Day",
        "Día de la Constitución",
        Rule::gregorian(11, 6),
    )
    .years(None, Some(1996)),
    HolidayRule::fixed_public(
        "Constitution Day",
        "Día de la Constitución",
        Rule::moved_by_weekday(&DO_NOVEMBER_6, TO_ADJACENT_MONDAY),
    )
    .years(Some(1997), None),
    HolidayRule::fixed_public("Christmas Day", "Navidad", Rule::gregorian(12, 25)),
];

/// Dominican Republic.
///
/// Ley 139-97 of 19 June 1997, in force from the 27th, read from its own
/// text: a holiday on a Tuesday or Wednesday is kept on the Monday before
/// and one on a Thursday or Friday on the Monday after, except New Year's
/// Day, Our Lady of Altagracia, Independence Day, Our Lady of Mercy,
/// Christmas, the two feasts fixed by their weekday, and Restoration Day
/// in a year that opens a constitutional period, which every fourth year
/// does — 2024's Friday 16 August stayed, as the Ministry's list said. A
/// Sunday 1 May gives the Monday under article 4. The moved rules start
/// with the law, 16 August and 6 November in 1997 and the rest in 1998,
/// their dates fixed before. Holy Thursday, which article 3 names among
/// the weekday feasts, is not on the Ministry's lists and is not carried.
pub static DOMINICAN_REPUBLIC: RuleSet = RuleSet {
    code: "DO",
    english_name: "Dominican Republic",
    rules: DO_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Ley 139-97 of 19 June 1997, as published by the Suprema Corte de Justicia \
              (justia.com), for articles 1 to 4; the Ministry of Labour's lists for \
              2024 and 2026 on presidencia.gob.do, retrieved 2026-09-22, for the \
              observed dates and the 2024 exception; Wikipedia, \"Public holidays in \
              the Dominican Republic\", for the English names",
};

// ─────────────────────────────────────────────────────────────────────────
// Guatemala
// ─────────────────────────────────────────────────────────────────────────

/// The Law Promoting Internal Tourism (Decreto 42-2010 as reformed by
/// Decreto 19-2018): a holiday on a Tuesday or Wednesday to the Monday
/// before, on a Thursday, Friday, Saturday or Sunday to the Monday after.
const GT_TO_MONDAY: &[(Weekday, i16)] = &[
    (Weekday::Tuesday, -1),
    (Weekday::Wednesday, -2),
    (Weekday::Thursday, 4),
    (Weekday::Friday, 3),
    (Weekday::Saturday, 2),
    (Weekday::Sunday, 1),
];

static GT_MAY_1: Rule = Rule::gregorian(5, 1);
static GT_JUNE_30: Rule = Rule::gregorian(6, 30);
static GT_OCTOBER_20: Rule = Rule::gregorian(10, 20);

static GT_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Año Nuevo", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Maundy Thursday",
        "Jueves Santo",
        Rule::easter(MAUNDY_THURSDAY),
    ),
    HolidayRule::fixed_public("Good Friday", "Viernes Santo", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Holy Saturday", "Sábado Santo", Rule::easter(HOLY_SATURDAY)),
    // Labour Day and Revolution Day were moved under the 2018 reform until
    // the Constitutional Court struck them from it in 2020.
    HolidayRule::fixed_public("Labour Day", "Día del Trabajo", Rule::gregorian(5, 1))
        .years(None, Some(2018)),
    HolidayRule::fixed_public(
        "Labour Day",
        "Día del Trabajo",
        Rule::moved_by_weekday(&GT_MAY_1, GT_TO_MONDAY),
    )
    .years(Some(2019), Some(2019)),
    HolidayRule::fixed_public("Labour Day", "Día del Trabajo", Rule::gregorian(5, 1))
        .years(Some(2020), None),
    HolidayRule::fixed_public("Army Day", "Día del Ejército", Rule::gregorian(6, 30))
        .years(None, Some(2018)),
    HolidayRule::fixed_public(
        "Army Day",
        "Día del Ejército",
        Rule::moved_by_weekday(&GT_JUNE_30, GT_TO_MONDAY),
    )
    .years(Some(2019), None),
    // The festivity of the locality: Guatemala City's is the Assumption.
    HolidayRule::fixed_public(
        "Assumption Day",
        "Día de la Asunción",
        Rule::gregorian(8, 15),
    )
    .in_regions(&["GT-GU"]),
    HolidayRule::fixed_public(
        "Independence Day",
        "Día de la Independencia",
        Rule::gregorian(9, 15),
    ),
    HolidayRule::fixed_public(
        "Revolution Day",
        "Día de la Revolución",
        Rule::gregorian(10, 20),
    )
    .years(None, Some(2017)),
    HolidayRule::fixed_public(
        "Revolution Day",
        "Día de la Revolución",
        Rule::moved_by_weekday(&GT_OCTOBER_20, GT_TO_MONDAY),
    )
    .years(Some(2018), Some(2019)),
    HolidayRule::fixed_public(
        "Revolution Day",
        "Día de la Revolución",
        Rule::gregorian(10, 20),
    )
    .years(Some(2020), None),
    HolidayRule::fixed_public(
        "All Saints' Day",
        "Día de Todos los Santos",
        Rule::gregorian(11, 1),
    ),
    // From noon.
    HolidayRule::fixed_public("Christmas Eve", "Nochebuena", Rule::gregorian(12, 24))
        .of_kind(Kind::Bank),
    HolidayRule::fixed_public("Christmas Day", "Navidad", Rule::gregorian(12, 25)),
    HolidayRule::fixed_public("New Year's Eve", "Fin de Año", Rule::gregorian(12, 31))
        .of_kind(Kind::Bank),
];

/// Guatemala.
///
/// Article 127 of the Código de Trabajo: the days of paid rest, with
/// Christmas Eve and New Year's Eve from noon as [`Kind::Bank`] half days
/// and the Assumption as Guatemala City's own festivity, `GT-GU`. The Law
/// Promoting Internal Tourism as reformed by Decreto 19-2018, in force
/// from 18 October 2018, moves Army Day to the Monday before from a
/// Tuesday or Wednesday and the Monday after from any later day, and did
/// the same to Revolution Day in 2018 and 2019 and Labour Day in 2019
/// until the Constitutional Court's ruling of 17 March 2020 struck those
/// two from it; the festivity of the locality it excludes. The 2010 law's
/// own moves before the 2018 reform are not carried, and the dates before
/// 2018 are fixed. Nothing else moves.
pub static GUATEMALA: RuleSet = RuleSet {
    code: "GT",
    english_name: "Guatemala",
    rules: GT_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "Código de Trabajo (Decreto 1441), artículo 127, not read (mintrabajo.gob.gt and \
              congreso.gob.gt refusing access), as Wikipedia, \"Public holidays in Guatemala\" \
              (secondary), retrieved 2026-09-22, gives its list; Decreto 19-2018, emitido 25 \
              de septiembre de 2018, Diario de Centro América of 10 October 2018, as \
              transcribed at cuasiabogadosgt.blogspot.com, retrieved 2026-09-26, and Lexology \
              and Prensa Libre on the Constitutional Court's ruling of 17 March 2020, for the \
              moves and their years; Prensa Libre, 13 August 2026, for the Assumption as \
              Guatemala City's festivity outside the law",
};

// ─────────────────────────────────────────────────────────────────────────
// Panama
// ─────────────────────────────────────────────────────────────────────────

/// Article 47 of the Código de Trabajo: a national holiday or day of
/// mourning on a Sunday makes the Monday after the obligatory rest day.
static PA_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static PA_JANUARY_9: Rule = Rule::gregorian(1, 9);
static PA_NOVEMBER_28: Rule = Rule::gregorian(11, 28);

static PA_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Año Nuevo", Rule::gregorian(1, 1)),
    // Ley 70 of 2007: Martyrs' Day and Independence Day to the adjacent
    // Monday from a Tuesday to a Friday.
    HolidayRule::public("Martyrs' Day", "Día de los Mártires", Rule::gregorian(1, 9))
        .years(None, Some(2007)),
    HolidayRule::public(
        "Martyrs' Day",
        "Día de los Mártires",
        Rule::moved_by_weekday(&PA_JANUARY_9, TO_ADJACENT_MONDAY),
    )
    .years(Some(2008), None),
    HolidayRule::public(
        "Carnival Tuesday",
        "Martes de Carnaval",
        Rule::easter(SHROVE_TUESDAY),
    ),
    HolidayRule::public("Good Friday", "Viernes Santo", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public("Labour Day", "Día del Trabajo", Rule::gregorian(5, 1)),
    HolidayRule::public(
        "Separation Day",
        "Separación de Panamá de Colombia",
        Rule::gregorian(11, 3),
    ),
    // A day off for public offices and schools, not a fiesta nacional.
    HolidayRule::observance(
        "Flag Day",
        "Día de los Símbolos Patrios",
        Rule::gregorian(11, 4),
    ),
    HolidayRule::public("Colón Day", "Día de Colón", Rule::gregorian(11, 5)),
    HolidayRule::public(
        "First Cry of Independence",
        "Primer Grito de Independencia de la Villa de Los Santos",
        Rule::gregorian(11, 10),
    ),
    HolidayRule::public(
        "Independence Day",
        "Independencia de Panamá de España",
        Rule::gregorian(11, 28),
    )
    .years(None, Some(2007)),
    HolidayRule::public(
        "Independence Day",
        "Independencia de Panamá de España",
        Rule::moved_by_weekday(&PA_NOVEMBER_28, TO_ADJACENT_MONDAY),
    )
    .years(Some(2008), None),
    HolidayRule::public("Mother's Day", "Día de la Madre", Rule::gregorian(12, 8)),
    HolidayRule::public(
        "National Mourning Day",
        "Día de Duelo Nacional",
        Rule::gregorian(12, 20),
    )
    .years(Some(2022), None),
    HolidayRule::public("Christmas Day", "Navidad", Rule::gregorian(12, 25)),
];

/// Panama.
///
/// Article 46 of the Código de Trabajo: the national holidays and days of
/// mourning, with 20 December from Ley 291 of 2022, and article 47's
/// Sunday rule as a forward policy. Ley 70 of 28 December 2007 keeps
/// Martyrs' Day and Independence Day on the Monday before from a Tuesday
/// or Wednesday and the Monday after from a Thursday or Friday, the same
/// table Argentina and Uruguay use, from 2008. Flag Day is a day off for
/// public offices and schools and not a holiday of the article, and is an
/// observance; the Carnival Monday and the bridges the Government decrees
/// are not carried.
pub static PANAMA: RuleSet = RuleSet {
    code: "PA",
    english_name: "Panama",
    rules: PA_RULES,
    substitution: PA_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Código de Trabajo arts. 46 and 47 as amended by Ley 70 of 28 December \
              2007, per the Gaceta Oficial and the Ministry of Labour's consulta of \
              14 February 2000 on article 47; Ley 291 of 2022 for 20 December, per \
              the Ministry of Labour and TVN; Wikipedia, \"Public holidays in \
              Panama\", retrieved 2026-09-22, for the list and the names",
};

// ─────────────────────────────────────────────────────────────────────────
// Jamaica
// ─────────────────────────────────────────────────────────────────────────

/// The Schedule's Sunday moves: New Year's Day, Emancipation Day and
/// Independence Day to the Monday, a Sunday Christmas to the 26th and
/// 27th, which the forward search past Boxing Day produces.
static JM_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static JM_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("Ash Wednesday", "", Rule::easter(ASH_WEDNESDAY)),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("Labour Day", "", Rule::gregorian(5, 23))
        .substitute_on(&[Weekday::Saturday, Weekday::Sunday])
        .years(Some(1961), None),
    HolidayRule::public("Emancipation Day", "", Rule::gregorian(8, 1)).years(Some(1997), None),
    HolidayRule::fixed_public("Independence Day", "", Rule::nth(8, 1, Weekday::Monday))
        .years(Some(1962), Some(1996)),
    HolidayRule::public("Independence Day", "", Rule::gregorian(8, 6)).years(Some(1997), None),
    HolidayRule::fixed_public(
        "National Heroes' Day",
        "",
        Rule::nth(10, 3, Weekday::Monday),
    )
    .years(Some(1969), None),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)).substituted_from(2021),
];

/// Jamaica.
///
/// The Holidays (Public General) Act, read from the Ministry of Labour's
/// copy of its Schedule: New Year's Day, with the day after when it is a
/// Sunday; Ash Wednesday; Easter Monday; National Labour Day on 23 May,
/// or the Monday after a Saturday or Sunday; Emancipation Day and
/// Independence Day on 1 and 6 August, each to the Monday after a Sunday;
/// National Heroes' Day on the third Monday of October; and "the day
/// after Christmas Day, or when Christmas Day falls on a Sunday, then the
/// 26th and 27th of December". Good Friday and Christmas Day are not in
/// the Schedule — the Act descends from the English one that took them
/// as holidays already — and every yearly list carries them, so they are
/// here. The Sunday moves are one policy and the Saturday one is Labour
/// Day's own trigger; the Ministry's 2026 notice that a Saturday
/// Emancipation Day "will be observed on that date" is why nothing else
/// moves off a Saturday. A Sunday Boxing Day has no rule of its own; the
/// Minister appointed the Monday in 2021 under section 7, and the table
/// substitutes it from that year and not before. A Sunday Christmas gives
/// the 26th to Boxing Day and the 27th to Christmas, the same two days the
/// Ministry names the other way round. Labour Day replaced Empire Day in
/// 1961 and National Heroes' Day began in 1969; Emancipation Day was
/// restored in 1997, when Independence Day returned from the first Monday
/// of August, where it had sat since 1962, to 6 August. The special days
/// the Minister appoints under section 7 are not carried.
pub static JAMAICA: RuleSet = RuleSet {
    code: "JM",
    english_name: "Jamaica",
    rules: JM_RULES,
    substitution: JM_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "The Holidays (Public General) Act as the Ministry of Labour and Social \
              Security publishes it (mlss.gov.jm), retrieved 2026-09-22, for sections \
              2, 6 and 7 and the Schedule; JIS, \"Emancipation Day to be Observed on \
              Saturday, August 1, 2026\" and \"Christmas Day to be Celebrated on \
              Monday, December 26\" (2022), and the Ministry's 2021 Boxing Day notice \
              as the Jamaica Observer reported it; the Gleaner, \"The fight for \
              Emancipation Day holiday in Jamaica\" (2023) and \"Legal Scoop: When a \
              public holiday falls on a Saturday\" (2020); Wikipedia, \"Labour Day\", \
              \"Heroes' Day\" and \"Public holidays in Jamaica\"",
};

// ─────────────────────────────────────────────────────────────────────────
// Trinidad and Tobago
// ─────────────────────────────────────────────────────────────────────────

/// Section 3(2): a Sunday, or two holidays on one day, gives "the next
/// following day that (apart from this subsection) is not a public
/// holiday".
static TT_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: true,
    valid_from: None,
    valid_until: None,
}];

static TT_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::observance("Carnival Monday", "", Rule::easter(SHROVE_MONDAY)),
    HolidayRule::observance("Carnival Tuesday", "", Rule::easter(SHROVE_TUESDAY)),
    HolidayRule::public(
        "Spiritual Baptist Liberation Shouter Day",
        "",
        Rule::gregorian(3, 30),
    )
    .years(Some(1996), None),
    HolidayRule::public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("Indian Arrival Day", "", Rule::gregorian(5, 30)).years(Some(1995), None),
    HolidayRule::public("Corpus Christi", "", Rule::easter(CORPUS_CHRISTI)),
    HolidayRule::public("Labour Day", "", Rule::gregorian(6, 19)),
    HolidayRule::public("Emancipation Day", "", Rule::gregorian(8, 1))
        .years(Some(1985), Some(2023)),
    HolidayRule::public("African Emancipation Day", "", Rule::gregorian(8, 1))
        .years(Some(2024), None),
    HolidayRule::public("Independence Day", "", Rule::gregorian(8, 31)).years(Some(1962), None),
    HolidayRule::public("Republic Day", "", Rule::gregorian(9, 24)),
    HolidayRule::public(
        "Eid-ul-Fitr",
        "",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 1),
    )
    .approximate(),
    HolidayRule::public("Divali", "", DIWALI).approximate(),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
];

/// Trinidad and Tobago.
///
/// The Public Holidays and Festivals Act (Chap. 19:05, updated to
/// 31 December 2016) from its Schedule and section 3(2): fourteen days,
/// and "where a public holiday falls on a Sunday or where two public
/// holidays fall on the same day, the next following day that (apart from
/// this subsection) is not a public holiday shall be a public holiday" —
/// the Friday after Thursday 30 May 2024, which was Corpus Christi and
/// Indian Arrival Day at once. Only a Sunday or a coincidence moves; a
/// Saturday holiday stays. The Act names no holiday for the extra day,
/// and the table gives it to the later of the two in name order, with the
/// shared day as the one it is for. Eid-ul-Fitr and Divali are "date to
/// be appointed" by the President's Notification, carried on the tabular
/// Hijri calendar and the crate's Lakṣmī Pūjā rule as approximations.
/// Carnival Monday and Tuesday are festivals under section 5 and not
/// public holidays, though the Office of the President notes most
/// businesses close; they are observances. Emancipation Day came in 1985
/// and became African Emancipation Day by the 2024 Order; Indian Arrival
/// Day in 1995 and the Spiritual Baptist day, under the Schedule's own
/// order of its words, in 1996. Labour Day and Republic Day carry no
/// first year, and Whit Monday and Discovery Day, which the newer days
/// replaced, are not carried.
pub static TRINIDAD_AND_TOBAGO: RuleSet = RuleSet {
    code: "TT",
    english_name: "Trinidad and Tobago",
    rules: TT_RULES,
    substitution: TT_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Public Holidays and Festivals Act, Chap. 19:05, as laws.gov.tt publishes it \
              (updated to 31 December 2016), retrieved 2026-09-22, for sections 3 to 5 \
              and the Schedule; Legal Notice No. 68 of 2024, the Public Holidays and \
              Festivals (Amendment to Schedule) Order, 2024 (printery.gov.tt), for \
              African Emancipation Day; the Office of the President, \"National \
              Holidays and Festivals\" (otp.tt), for Carnival; the Trinidad Guardian on \
              Friday 31 May 2024 under section 3(2); Wikipedia, \"Emancipation Day\", \
              \"Indian Arrival Day\" and \"Spiritual Baptist\", for 1985, 1995 and 1996",
};

// ─────────────────────────────────────────────────────────────────────────
// Barbados
// ─────────────────────────────────────────────────────────────────────────

/// The paragraph after the First Schedule: a Sunday gives the Monday, or
/// the Tuesday when the Monday is already Boxing Day or the first-Monday
/// holiday; Emancipation Day's own trigger adds its Monday case.
static BB_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static BB_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public("Errol Barrow Day", "", Rule::gregorian(1, 21)).years(Some(1989), None),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("National Heroes Day", "", Rule::gregorian(4, 28)).years(Some(1998), None),
    HolidayRule::public("Labour Day", "", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Whit Monday", "", Rule::easter(WHIT_MONDAY)),
    HolidayRule::public("Emancipation Day", "", Rule::gregorian(8, 1))
        .substitute_on(&[Weekday::Sunday, Weekday::Monday]),
    HolidayRule::fixed_public("Kadooment Day", "", Rule::nth(8, 1, Weekday::Monday)),
    HolidayRule::public("Independence Day", "", Rule::gregorian(11, 30)),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
];

/// Barbados.
///
/// The Public Holidays Act (Cap. 352, L.R.O. 1998) from its First
/// Schedule and the paragraph after it: twelve days, of which New Year's
/// Day, Errol Barrow Day, National Heroes Day, Labour Day, Independence
/// Day and Boxing Day give the Monday after a Sunday; Emancipation Day
/// the Tuesday after a Sunday or a Monday, the Monday being the first
/// Monday of August, which the Government's calendar calls Kadooment Day;
/// and Christmas the Tuesday after a Sunday, the Monday being Boxing Day.
/// The Schedule's "if a week day" makes the Sunday itself no holiday; the
/// table keeps it as the day the substitute is for, as the Government's
/// calendar does. Errol Barrow Day began in 1989 and National Heroes Day
/// in 1998; the Act's notes cite amendments of 1974, 1978, 1983, 1997 and
/// 1998 without saying which entry each made, so Emancipation Day and
/// Kadooment Day carry no first year.
pub static BARBADOS: RuleSet = RuleSet {
    code: "BB",
    english_name: "Barbados",
    rules: BB_RULES,
    substitution: BB_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Public Holidays Act, Cap. 352 (L.R.O. 1998), as barbadoslawcourts.gov.bb \
              publishes it, retrieved 2026-09-22, for section 3 and the First Schedule; \
              the Government of Barbados bank-holiday calendar (alpha.gov.bb) for 2021, \
              2022 and 2026, retrieved 2026-09-22, for the observed days and Kadooment \
              Day's name; Wikipedia, \"Errol Barrow Day\" and \"Order of National \
              Heroes\" (Barbados), for 1989 and 1998",
};

// ─────────────────────────────────────────────────────────────────────────
// The Bahamas
// ─────────────────────────────────────────────────────────────────────────

/// Section 3's proviso for a Sunday, and the Government's practice for a
/// Saturday, both to the next free weekday.
static BS_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static BS_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public("Majority Rule Day", "", Rule::gregorian(1, 10)).years(Some(2014), None),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::fixed_public("Whit Monday", "", Rule::easter(WHIT_MONDAY)),
    HolidayRule::fixed_public("Labour Day", "", Rule::nth(6, 1, Weekday::Friday))
        .years(None, Some(2013)),
    HolidayRule::fixed_public(
        "Randol Fawkes Labour Day",
        "",
        Rule::nth(6, 1, Weekday::Friday),
    )
    .years(Some(2014), None),
    HolidayRule::public("Independence Day", "", Rule::gregorian(7, 10)).years(Some(1973), None),
    HolidayRule::fixed_public("Emancipation Day", "", Rule::nth(8, 1, Weekday::Monday)),
    HolidayRule::public("Discovery Day", "", Rule::gregorian(10, 12)).years(None, Some(2012)),
    HolidayRule::fixed_public("National Heroes Day", "", Rule::nth(10, 2, Weekday::Monday))
        .years(Some(2013), None),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
];

/// The Bahamas.
///
/// The Public Holidays Act (Ch. 36, L.R.O. 1/2017) from its First
/// Schedule and section 3's proviso that a holiday on a Sunday is observed
/// the day after. The Saturday move is the Government's practice, Majority
/// Rule Day 2026 kept on Monday 12 January; a Saturday Christmas gives the
/// Monday and its Sunday Boxing Day the Tuesday. Majority Rule Day came
/// with the 2013 Act and was first kept in 2014; the National Heroes Act,
/// assented to in October 2013, put National Heroes Day on the second
/// Monday of October in place of Discovery Day on the 12th, which the
/// revised Schedule still prints and the table ends in 2012; Labour Day
/// became Randol Fawkes Labour Day by the 2013 amendment, carried from
/// 2014. The revised Act has no rule for a holiday on a Tuesday,
/// Wednesday or Thursday, and the Governor-General's 2024 notice took
/// Wednesday 10 January as "the day celebrated as Majority Rule Day in
/// 2024", so none is carried, whatever some lists say of Fridays. The
/// special days of section 4 are not carried.
pub static BAHAMAS: RuleSet = RuleSet {
    code: "BS",
    english_name: "Bahamas",
    rules: BS_RULES,
    substitution: BS_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Public Holidays Act, Ch. 36 (Statute Law of The Bahamas, L.R.O. 1/2017), as \
              laws.bahamas.gov.bs publishes it, retrieved 2026-09-22, for sections 3 and \
              4 and the First Schedule; S.I. No. 2 of 2024, the Public Holidays (Majority \
              Rule Day, 2024) (Opening of Shops) Notice; the Securities Commission of The \
              Bahamas on Monday 12 January 2026; the Tribune, \"National Heroes Day \
              formally established\" (12 October 2013); Wikipedia, \"Public holidays in \
              the Bahamas\" and \"Discovery Day\"",
};

// ─────────────────────────────────────────────────────────────────────────
// Cuba
// ─────────────────────────────────────────────────────────────────────────

/// Article 82 of the Labour Code rests the week on Sunday, the "descanso
/// dominical" that article 97 moves.
static CU_WEEKEND: &[WeekendPolicy] = &[WeekendPolicy {
    days: &[Weekday::Sunday],
    valid_from: None,
    valid_from_day: None,
    valid_until: None,
    valid_until_day: None,
}];

/// Article 97: a Sunday 1 May or 10 October moves the Sunday rest to the
/// Monday; a Sunday 1 January or 26 July does not, "being preceded and
/// followed by holidays".
static CU_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: false,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static CU_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "Triumph of the Revolution",
        "Aniversario de la Revolución",
        Rule::gregorian(1, 1),
    ),
    HolidayRule::fixed_public("Victory Day", "Día de la Victoria", Rule::gregorian(1, 2)),
    HolidayRule::fixed_public("Good Friday", "Viernes Santo", Rule::easter(GOOD_FRIDAY))
        .years(Some(2012), None),
    HolidayRule::public(
        "International Workers' Day",
        "Día Internacional de los Trabajadores",
        Rule::gregorian(5, 1),
    ),
    HolidayRule::fixed_public(
        "Day before National Rebellion Day",
        "Víspera del Día de la Rebeldía Nacional",
        Rule::gregorian(7, 25),
    ),
    HolidayRule::fixed_public(
        "National Rebellion Day",
        "Día de la Rebeldía Nacional",
        Rule::gregorian(7, 26),
    ),
    HolidayRule::fixed_public(
        "Day after National Rebellion Day",
        "Día siguiente al Día de la Rebeldía Nacional",
        Rule::gregorian(7, 27),
    ),
    HolidayRule::public(
        "Beginning of the Wars of Independence",
        "Inicio de las Guerras de Independencia",
        Rule::gregorian(10, 10),
    ),
    HolidayRule::fixed_public("Christmas Day", "Navidad", Rule::gregorian(12, 25))
        .years(Some(1998), None),
    HolidayRule::fixed_public("New Year's Eve", "Fin de Año", Rule::gregorian(12, 31)),
];

/// Cuba.
///
/// Law 116 of 20 December 2013, the Labour Code, in force from 17 June
/// 2014, from the Ministry of Justice's edition: article 94's four days of
/// national commemoration — 1 January, 1 May, 26 July and 10 October —
/// and its five holidays, 2 January, 25 and 27 July, 25 and 31 December;
/// article 100's Good Friday as a day of rest with pay, which the
/// Government had already granted in 2012 at Pope Benedict XVI's request
/// and in 2013, so it runs from 2012; and article 97, by which a Sunday
/// 1 May or 10 October moves the Sunday rest to the Monday while a Sunday
/// 1 January or 26 July does not, "being preceded and followed by
/// holidays" — only those two rules substitute. Christmas was a working
/// day from 1969 and a holiday again from 1998 after Pope John Paul II's
/// visit. The Code's weekly rest is the Sunday, which is the weekend
/// here. Article 98's fifteen official commemorations are working days
/// and are not carried; nor are the days the Government declares under
/// article 100.
pub static CUBA: RuleSet = RuleSet {
    code: "CU",
    english_name: "Cuba",
    rules: CU_RULES,
    substitution: CU_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: CU_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Ley No. 116, Código de Trabajo, articles 94 to 100, in the Ministry of \
              Justice's 2014 edition (minjus.gob.cu), retrieved 2026-09-22; ACI Prensa on \
              the Good Fridays of 2012 and 2013; Wikipedia, \"Public holidays in Cuba\", \
              for the English names and 1998",
};

// ─────────────────────────────────────────────────────────────────────────
// Belize
// ─────────────────────────────────────────────────────────────────────────

/// The Second Schedule's days as the Government's notices place them: a
/// Tuesday, Wednesday or Thursday to the Monday before, a Friday,
/// Saturday or Sunday to the Monday after.
const BZ_TO_MONDAY: &[(Weekday, i16)] = &[
    (Weekday::Tuesday, -1),
    (Weekday::Wednesday, -2),
    (Weekday::Thursday, -3),
    (Weekday::Friday, 3),
    (Weekday::Saturday, 2),
    (Weekday::Sunday, 1),
];

/// A Tuesday to the Monday before, as the notices did for Emancipation
/// Day in 2023 and St. George's Caye Day in 2024.
const BZ_TUESDAY_BACK: &[(Weekday, i16)] = &[(Weekday::Tuesday, -1)];

/// A Tuesday or Wednesday to the Monday before, as the notice did for
/// Labour Day in 2024.
const BZ_TUESDAY_WEDNESDAY_BACK: &[(Weekday, i16)] =
    &[(Weekday::Tuesday, -1), (Weekday::Wednesday, -2)];

static BZ_MARCH_9: Rule = Rule::gregorian(3, 9);
static BZ_MAY_1: Rule = Rule::gregorian(5, 1);
static BZ_AUGUST_1: Rule = Rule::gregorian(8, 1);
static BZ_SEPTEMBER_10: Rule = Rule::gregorian(9, 10);
static BZ_OCTOBER_12: Rule = Rule::gregorian(10, 12);

/// A Sunday to the Monday after, for the First Schedule's days.
static BZ_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: false,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static BZ_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public("George Price Day", "", Rule::gregorian(1, 15)).years(Some(2021), None),
    HolidayRule::fixed_public(
        "National Heroes and Benefactors Day",
        "",
        Rule::moved_by_weekday(&BZ_MARCH_9, BZ_TO_MONDAY),
    ),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Holy Saturday", "", Rule::easter(HOLY_SATURDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public(
        "Labour Day",
        "",
        Rule::moved_by_weekday(&BZ_MAY_1, BZ_TUESDAY_WEDNESDAY_BACK),
    ),
    HolidayRule::public("Commonwealth Day", "", Rule::gregorian(5, 24)).years(None, Some(2021)),
    HolidayRule::public(
        "Emancipation Day",
        "",
        Rule::moved_by_weekday(&BZ_AUGUST_1, BZ_TUESDAY_BACK),
    )
    .years(Some(2021), None),
    HolidayRule::public(
        "St. George's Caye Day",
        "",
        Rule::moved_by_weekday(&BZ_SEPTEMBER_10, BZ_TUESDAY_BACK),
    ),
    HolidayRule::public("Independence Day", "", Rule::gregorian(9, 21)).years(Some(1981), None),
    HolidayRule::fixed_public(
        "Pan American Day",
        "",
        Rule::moved_by_weekday(&BZ_OCTOBER_12, BZ_TO_MONDAY),
    )
    .years(None, Some(2021)),
    HolidayRule::fixed_public(
        "Indigenous Peoples' Resistance Day",
        "",
        Rule::moved_by_weekday(&BZ_OCTOBER_12, BZ_TO_MONDAY),
    )
    .years(Some(2022), None),
    HolidayRule::public("Garifuna Settlement Day", "", Rule::gregorian(11, 19)),
    HolidayRule::fixed_public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
];

/// Belize.
///
/// The Holidays Act, Chapter 289, could not be read; what is carried is
/// the Government's yearly notices of the days its First and Second
/// Schedules give, from 2020 to 2026, and Wikipedia's account of the
/// Act. The Second Schedule's two days, National Heroes and Benefactors
/// Day on 9 March and the 12 October day, go to the Monday before from a
/// Tuesday, Wednesday or Thursday and the Monday after from a Saturday or
/// Sunday, as every notice from 2021 shows, and from a Friday by
/// Wikipedia's reading. The First Schedule's days go to the Monday after
/// a Sunday and stay on a Saturday; three of them also leave a Tuesday
/// for the Monday before — Emancipation Day in 2023, St. George's Caye
/// Day in 2024 — and Labour Day left a Wednesday too, in 2024, where
/// St. George's Caye Day kept its Wednesday in 2025; the table follows
/// the notices in each case. A Sunday Christmas got no Monday in 2022,
/// and Christmas does not move. George Price Day and Emancipation Day
/// began in 2021, when Commonwealth Day ended; Pan American Day became
/// Indigenous Peoples' Resistance Day in 2022.
pub static BELIZE: RuleSet = RuleSet {
    code: "BZ",
    english_name: "Belize",
    rules: BZ_RULES,
    substitution: BZ_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Government of Belize Press Office, \"Public and Bank Holidays\" for 2020, \
              2022, 2023, 2024, 2025 and 2026 (pressoffice.gov.bz), each citing section 3 \
              and the First and Second Schedules of the Holidays Act, Chapter 289, \
              retrieved 2026-09-22; the San Pedro Sun (4 January 2021) on the 2021 list; \
              Wikipedia, \"Public holidays in Belize\", for the Act's rules",
};

// ─────────────────────────────────────────────────────────────────────────
// Guyana
// ─────────────────────────────────────────────────────────────────────────

/// Section 3(1): each day "or, if that day is a Sunday, the following
/// day", and Boxing Day the Tuesday when Christmas Day is the Sunday.
static GY_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static GY_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public("Republic Day", "", Rule::gregorian(2, 23)).years(Some(1970), None),
    HolidayRule::public("Phagwah", "", HOLI).approximate(),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("Labour Day", "", Rule::gregorian(5, 1)),
    HolidayRule::public("Arrival Day", "", Rule::gregorian(5, 5)).years(Some(2004), None),
    HolidayRule::public("Independence Day", "", Rule::gregorian(5, 26)).years(Some(1966), None),
    HolidayRule::fixed_public("CARICOM Day", "", Rule::nth(7, 1, Weekday::Monday)),
    HolidayRule::public("Emancipation Day", "", Rule::gregorian(8, 1)),
    HolidayRule::public(
        "Youman Nabi",
        "",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 3, 12),
    )
    .approximate(),
    HolidayRule::public(
        "Eid-ul-Azha",
        "",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 10),
    )
    .approximate(),
    HolidayRule::public("Deepavali", "", DIWALI).approximate(),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
];

/// Guyana.
///
/// The Public Holidays Act, Chapter 19:07 (L.R.O. 1/2012), section 3(1):
/// "the first week-day of January", Republic Day as the Minister
/// gazettes it, Good Friday, Easter Monday, Labour Day, Christmas and
/// Boxing Day, and Phagwah, Deepavali, Eid-ul-Azha and Youman Nabi as
/// section 3(2) has them gazetted each year, every fixed one "or, if that
/// day is a Sunday, the following day", and Boxing Day the Tuesday when
/// Christmas is the Sunday — the same two days here, with the
/// substitute named for Christmas. Arrival Day, Independence Day,
/// CARICOM Day and Emancipation Day are the Minister's under section 6,
/// and the Sunday rule is applied to them as the yearly lists do — a
/// Sunday 5 May 2024 was kept on the Monday. Republic Day from 1970 and
/// Arrival Day from 2004; Commonwealth Day on the first Monday of
/// August, which the printed section still carries, is not. The four
/// gazetted feasts are on the tabular Hijri calendar and the crate's
/// Holi and Lakṣmī Pūjā rules as approximations.
pub static GUYANA: RuleSet = RuleSet {
    code: "GY",
    english_name: "Guyana",
    rules: GY_RULES,
    substitution: GY_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Public Holidays Act, Chapter 19:07, L.R.O. 1/2012, as the Ministry of Legal \
              Affairs publishes it (mola.gov.gy), retrieved 2026-09-22, for sections 3 \
              and 6; Wikipedia, \"2024 in Guyana\" and \"2025 in Guyana\", and the Ministry \
              of Home Affairs' notices for the yearly lists; Kaieteur News on Arrival Day's \
              2004 designation; Wikipedia, \"Public holidays in Guyana\", for the names",
};

// ─────────────────────────────────────────────────────────────────────────
// Haiti
// ─────────────────────────────────────────────────────────────────────────

/// Carnival Monday, a holiday "from noon".
const fn ht_lundi_gras() -> HolidayRule {
    HolidayRule::fixed_public("Carnival Monday", "Lundi Gras", Rule::easter(SHROVE_MONDAY))
        .of_kind(Kind::Bank)
}

static HT_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "Independence Day",
        "Fête de l'Indépendance",
        Rule::gregorian(1, 1),
    ),
    HolidayRule::fixed_public("Ancestors' Day", "Jour des Aïeux", Rule::gregorian(1, 2)),
    ht_lundi_gras().years(None, Some(1988)),
    ht_lundi_gras().years(Some(2025), None),
    HolidayRule::fixed_public(
        "Carnival Tuesday",
        "Mardi Gras",
        Rule::easter(SHROVE_TUESDAY),
    ),
    HolidayRule::fixed_public("Good Friday", "Vendredi Saint", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public(
        "Labour and Agriculture Day",
        "Fête du Travail et de l'Agriculture",
        Rule::gregorian(5, 1),
    ),
    HolidayRule::fixed_public(
        "Flag and University Day",
        "Fête du Drapeau et de l'Université",
        Rule::gregorian(5, 18),
    ),
    HolidayRule::fixed_public("Corpus Christi", "Fête-Dieu", Rule::easter(CORPUS_CHRISTI))
        .years(Some(1989), None),
    HolidayRule::fixed_public(
        "Bois-Caïman Day",
        "Jour du Bois-Caïman",
        Rule::gregorian(8, 14),
    )
    .years(Some(2025), None),
    HolidayRule::fixed_public("Assumption", "Fête de l'Assomption", Rule::gregorian(8, 15))
        .years(Some(1989), None),
    HolidayRule::fixed_public(
        "Dessalines Day",
        "Jour de Dessalines",
        Rule::gregorian(9, 20),
    )
    .years(Some(2025), None),
    HolidayRule::fixed_public(
        "Death of Dessalines",
        "Commémoration de la mort de Dessalines",
        Rule::gregorian(10, 17),
    )
    .years(Some(1989), None),
    HolidayRule::fixed_public(
        "All Saints' Day",
        "Fête de la Toussaint",
        Rule::gregorian(11, 1),
    )
    .years(Some(2025), None),
    HolidayRule::fixed_public("All Souls' Day", "Fête des Morts", Rule::gregorian(11, 2))
        .of_kind(Kind::Bank)
        .years(None, Some(1988)),
    HolidayRule::fixed_public("All Souls' Day", "Fête des Morts", Rule::gregorian(11, 2))
        .years(Some(1989), None),
    HolidayRule::fixed_public(
        "Battle of Vertières Day",
        "Commémoration de la Bataille de Vertières",
        Rule::gregorian(11, 18),
    ),
    HolidayRule::fixed_public("Christmas Day", "Noël", Rule::gregorian(12, 25)),
];

/// Haiti.
///
/// Two texts. Article 275-1 of the 1987 Constitution names the five
/// national holidays — 1 and 2 January, 1 and 18 May, 18 November — which
/// the 1984 Labour Code's article 110 already had as days off. The
/// decree of 11 December 2024, article 2, lists the legal holidays with
/// days off: Carnival Monday from noon, a bank day here, Carnival
/// Tuesday, Good Friday, Corpus Christi, 14 and 15 August, 20 September,
/// 17 October, 1 and 2 November and Christmas. Before it, the decree of
/// 23 May 1989 kept Carnival Tuesday, Good Friday, Corpus Christi, the
/// Assumption, 17 October, 2 November and Christmas, so those three
/// dates run from 1989 and 14 August, 20 September and 1 November from
/// 2025; the 1984 code's Carnival Monday and 2 November "from noon" end
/// in 1988, and the yearly decrees that gave 20 September and 1 November
/// before 2024 are not carried. No text read moves a holiday off a
/// Sunday.
pub static HAITI: RuleSet = RuleSet {
    code: "HT",
    english_name: "Haiti",
    rules: HT_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Code du travail, décret du 24 février 1984, articles 109 to 111, from the \
              CEPAL copy (oig.cepal.org), retrieved 2026-09-22; HDIT Cabinet Volmar, \
              \"Nouveaux jours fériés en Haïti\" (29 December 2024), for article 2 of the \
              décret du 11 décembre 2024 (Moniteur spécial no. 66-A) and the décret du \
              23 mai 1989, and \"Le premier novembre nécessite-t-il un arrêté \
              présidentiel ?\" for article 275-1 of the 1987 Constitution; Wikipedia, \
              \"Public holidays in Haiti\", for the English names",
};

// ─────────────────────────────────────────────────────────────────────────
// Venezuela
// ─────────────────────────────────────────────────────────────────────────

static VE_RULES: &[HolidayRule] = &[
    // Article 184(b) of the LOTTT.
    HolidayRule::fixed_public("New Year's Day", "Año Nuevo", Rule::gregorian(1, 1)),
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
    HolidayRule::fixed_public(
        "Maundy Thursday",
        "Jueves Santo",
        Rule::easter(MAUNDY_THURSDAY),
    ),
    HolidayRule::fixed_public("Good Friday", "Viernes Santo", Rule::easter(GOOD_FRIDAY)),
    // Article 184(c): the five days of the Ley de Fiestas Nacionales.
    HolidayRule::fixed_public(
        "Declaration of Independence",
        "Declaración de la Independencia",
        Rule::gregorian(4, 19),
    ),
    HolidayRule::fixed_public("Labour Day", "Día del Trabajador", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public(
        "Battle of Carabobo",
        "Batalla de Carabobo",
        Rule::gregorian(6, 24),
    ),
    HolidayRule::fixed_public(
        "Independence Day",
        "Día de la Independencia",
        Rule::gregorian(7, 5),
    ),
    HolidayRule::fixed_public(
        "Birthday of Simón Bolívar",
        "Natalicio del Libertador",
        Rule::gregorian(7, 24),
    ),
    // Decreto 2.028 of 10 October 2002 renamed the day; the date and the
    // day off are the Ley de Fiestas Nacionales'.
    HolidayRule::fixed_public("Day of the Race", "Día de la Raza", Rule::gregorian(10, 12))
        .years(None, Some(2001)),
    HolidayRule::fixed_public(
        "Day of Indigenous Resistance",
        "Día de la Resistencia Indígena",
        Rule::gregorian(10, 12),
    )
    .years(Some(2002), None),
    HolidayRule::fixed_public("Christmas Eve", "Nochebuena", Rule::gregorian(12, 24)),
    HolidayRule::fixed_public("Christmas Day", "Navidad", Rule::gregorian(12, 25)),
    HolidayRule::fixed_public("New Year's Eve", "Fin de Año", Rule::gregorian(12, 31)),
];

/// Venezuela.
///
/// Article 184 of the Ley Orgánica del Trabajo, los Trabajadores y las
/// Trabajadoras of 2012: Sundays; 1 January, Carnival Monday and Tuesday,
/// Maundy Thursday and Good Friday, 1 May and 24, 25 and 31 December,
/// all as whole days; and "los señalados en la Ley de Fiestas
/// Nacionales", whose 1971 text makes 19 April, 24 June, 5 July, 24 July
/// and 12 October the days of national festival. 12 October is the Day
/// of the Race until 2001 and the Day of Indigenous Resistance from
/// Decreto 2.028 of 10 October 2002, which renamed it. The Asamblea
/// Nacional lists a Ley de Reforma Parcial de la Ley de Fiestas
/// Nacionales among its sanctioned laws; its text could not be read, and
/// the 2022 bill as LabLabor reported it added four patriotic dates
/// without the standing of holidays under the LOTTT, so the table follows
/// the 1971 text. The up to three days a year the national, state or
/// municipal governments may declare under article 184(d) are not
/// carried, and nothing in either law moves a holiday off a weekend.
/// Article 173 gives two continuous rest days a week without naming them;
/// the weekend is taken as Saturday and Sunday.
pub static VENEZUELA: RuleSet = RuleSet {
    code: "VE",
    english_name: "Venezuela",
    rules: VE_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Ley Orgánica del Trabajo, los Trabajadores y las Trabajadoras, Gaceta \
              Oficial Extraordinaria 6.076 of 7 May 2012, articles 173 and 184, from \
              tugacetaoficial.com and Acceso a la Justicia, retrieved 2026-09-23; Ley de \
              Fiestas Nacionales, Gaceta Oficial 29.541 of 22 June 1971, article 1, from \
              the Justia copy of the gazette; Decreto 2.028 of 10 October 2002 as \
              NotiIndígena reproduces it; LabLabor (12 August 2022) and El Universal \
              (8 July 2022) on the reform bill; Wikipedia, \"Public holidays in \
              Venezuela\", for the names",
};

// ─────────────────────────────────────────────────────────────────────────
// Paraguay
// ─────────────────────────────────────────────────────────────────────────

/// The last year before Ley 1723 of 2001 let the Executive move holidays
/// by decree.
const PY_UNMOVED_UNTIL: i32 = 2000;
/// The years whose moving decrees the table has read.
const PY_DECREES_FIRST: i64 = 2026;
/// The last.
const PY_DECREES_LAST: i64 = 2026;

/// The four days Ley 7544/2025 lets an annual decree move, where each
/// read year's decrees put them: the day's month and day in the law, then
/// the month and day of that year.
static PY_DECREED: &[(i64, u8, u8, u8, u8)] = &[
    // 1 March, a Sunday, moved to Monday 2 March.
    (2026, 3, 1, 3, 2),
    // 12 June, a Friday, left where it was.
    (2026, 6, 12, 6, 12),
    // Decreto 6215: Saturday 20 June to Monday 22 June.
    (2026, 6, 20, 6, 22),
    // Decreto 6601: Tuesday 29 September to Monday 28 September.
    (2026, 9, 29, 9, 28),
];

fn py_decreed(year: i64, month: u8, day: u8) -> Days {
    for &(y, law_month, law_day, to_month, to_day) in PY_DECREED {
        if y == year && (law_month, law_day) == (month, day) {
            return gregorian::to_fixed(y, to_month, to_day)
                .map_or_else(|_| Days::new(), Days::one);
        }
    }
    Days::new()
}

fn py_heroes_day(year: i64) -> Days {
    py_decreed(year, 3, 1)
}

fn py_chaco_peace_day(year: i64) -> Days {
    py_decreed(year, 6, 12)
}

fn py_constitution_day(year: i64) -> Days {
    py_decreed(year, 6, 20)
}

fn py_boqueron_day(year: i64) -> Days {
    py_decreed(year, 9, 29)
}

/// Nothing: the decrees of 2001 to 2025 that could move this day were not
/// read, so a year among them is a gap.
fn py_unread(_: i64) -> Days {
    Days::new()
}

const fn py_decrees(function: fn(i64) -> Days) -> Rule {
    Rule::Tabulated {
        function,
        first_year: PY_DECREES_FIRST,
        last_year: PY_DECREES_LAST,
    }
}

/// A day a decree may move: on its date until 2000, and from 2001 where the
/// read decrees put it, a gap in every other year.
const fn py_movable(
    name: &'static str,
    local_name: &'static str,
    function: fn(i64) -> Days,
) -> HolidayRule {
    HolidayRule::fixed_public(name, local_name, py_decrees(function))
        .years(Some(PY_UNMOVED_UNTIL + 1), None)
}

/// A day Ley 1723 let a decree move and Ley 7544 fixed again: the years
/// 2001 to 2025 are a gap.
const fn py_moved_until_2025(name: &'static str, local_name: &'static str) -> HolidayRule {
    HolidayRule::fixed_public(name, local_name, py_decrees(py_unread))
        .years(Some(PY_UNMOVED_UNTIL + 1), Some(2025))
}

static PY_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Año Nuevo", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Heroes' Day",
        "Día de los Héroes de la Patria",
        Rule::gregorian(3, 1),
    )
    .years(None, Some(PY_UNMOVED_UNTIL)),
    py_movable(
        "Heroes' Day",
        "Día de los Héroes de la Patria",
        py_heroes_day,
    ),
    HolidayRule::fixed_public(
        "Maundy Thursday",
        "Jueves Santo",
        Rule::easter(MAUNDY_THURSDAY),
    ),
    HolidayRule::fixed_public("Good Friday", "Viernes Santo", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public(
        "Labour Day",
        "Día de los Trabajadores",
        Rule::gregorian(5, 1),
    )
    .years(None, Some(PY_UNMOVED_UNTIL)),
    py_moved_until_2025("Labour Day", "Día de los Trabajadores"),
    HolidayRule::fixed_public(
        "Labour Day",
        "Día de los Trabajadores",
        Rule::gregorian(5, 1),
    )
    .years(Some(2026), None),
    // Ley 4531 of November 2011 restored 14 May beside 15 May.
    HolidayRule::fixed_public(
        "Independence Day",
        "Día de la Independencia Nacional",
        Rule::Tabulated {
            function: py_unread,
            first_year: PY_DECREES_FIRST,
            last_year: PY_DECREES_LAST,
        },
    )
    .years(Some(2012), Some(2025)),
    HolidayRule::fixed_public(
        "Independence Day",
        "Día de la Independencia Nacional",
        Rule::gregorian(5, 14),
    )
    .years(Some(2026), None),
    HolidayRule::fixed_public(
        "Independence Day",
        "Día de la Independencia Nacional",
        Rule::gregorian(5, 15),
    ),
    HolidayRule::fixed_public(
        "Chaco Peace Day",
        "Día de la Paz del Chaco",
        Rule::gregorian(6, 12),
    )
    .years(None, Some(PY_UNMOVED_UNTIL)),
    py_movable(
        "Chaco Peace Day",
        "Día de la Paz del Chaco",
        py_chaco_peace_day,
    ),
    py_movable(
        "Constitution Day",
        "Día de la Jura de la Constitución Nacional",
        py_constitution_day,
    )
    .years(Some(2026), None),
    HolidayRule::fixed_public(
        "Founding of Asunción",
        "Día de la Fundación de Asunción",
        Rule::gregorian(8, 15),
    )
    .years(None, Some(PY_UNMOVED_UNTIL)),
    py_moved_until_2025("Founding of Asunción", "Día de la Fundación de Asunción"),
    HolidayRule::fixed_public(
        "Founding of Asunción",
        "Día de la Fundación de Asunción",
        Rule::gregorian(8, 15),
    )
    .years(Some(2026), None),
    HolidayRule::fixed_public(
        "Battle of Boquerón Day",
        "Día de la Batalla de Boquerón",
        Rule::gregorian(9, 29),
    )
    .years(Some(1995), Some(PY_UNMOVED_UNTIL)),
    py_movable(
        "Battle of Boquerón Day",
        "Día de la Batalla de Boquerón",
        py_boqueron_day,
    ),
    HolidayRule::fixed_public(
        "Virgin of Caacupé Day",
        "Día de la Virgen de Caacupé",
        Rule::gregorian(12, 8),
    ),
    HolidayRule::fixed_public(
        "Christmas Day",
        "Día de la Navidad",
        Rule::gregorian(12, 25),
    ),
    // The additional days of article 4 of Ley 7544/2025 that the table has
    // read of.
    HolidayRule::fixed_public(
        "Additional national holiday",
        "Feriado nacional adicional",
        Rule::gregorian(9, 5),
    )
    .years(Some(2025), Some(2025)),
    HolidayRule::fixed_public(
        "Additional national holiday",
        "Feriado nacional adicional",
        Rule::gregorian(12, 26),
    )
    .years(Some(2025), Some(2025)),
    HolidayRule::fixed_public(
        "Additional national holiday",
        "Feriado nacional adicional",
        Rule::gregorian(6, 30),
    )
    .years(Some(2026), Some(2026))
    .cited("Decreto 6280 of 29 June 2026"),
];

/// Paraguay.
///
/// Article 217 of the Código del Trabajo makes the holidays "established by
/// law" days of obligatory rest. The law is Ley 7544/2025, promulgated and
/// published on 3 September 2025, which repealed and replaced Ley 8/90 and
/// its amendments: 1 January, 1 March, Maundy Thursday and Good Friday, 1
/// May, 14 and 15 May, 12 June, 20 June, 15 August, 29 September, 8
/// December and 25 December. Its 20 June is new and runs from 2026; 14 May
/// is Ley 4531's, restored from 2012; 29 September is Ley 715's of 1995,
/// for five years and then for good by Ley 1601 of 2000; the rest are Ley
/// 8/90's of 1990, and nothing earlier is stated. Article 3 of the 2025 law
/// lets the Executive move 1 March, 12 June, 20 June and 29 September to
/// the Monday before or after by an annual decree, as Ley 1723 of 2001 had
/// let it move all but 1 January, 15 May, 8 and 25 December and Holy Week;
/// those moves are not a rule. The table carries the days on their dates
/// until 2000 and where 2026's decrees put them, and reports every day a
/// decree could move as a gap in the years whose decrees it has not read:
/// 2001 to 2025 under Ley 1723, and from 2027 the four days Ley 7544 lets
/// move. Ley 4316's one-off pair of 14 and 16 May 2011 is left out.
/// Article 4's up to three additional days a year are carried for the three the table read of, 5 September and
/// 26 December 2025 and 30 June 2026. No law read moves a holiday off a
/// weekend, and the Code's weekly rest is "normally Sunday"; the weekend is
/// Saturday and Sunday as for the region's other tables.
pub static PARAGUAY: RuleSet = RuleSet {
    code: "PY",
    english_name: "Paraguay",
    rules: PY_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Ley 7544/2025, Ley 8/90, Ley 715/95, Ley 1601/2000, Ley 1723/2001, Ley \
              4531/2011 and Ley 213/93 (Código del Trabajo) articles 213 and 217, from \
              the Biblioteca y Archivo Central del Congreso Nacional (bacn.gov.py), \
              retrieved 2026-09-23; Decreto 6215 of 9 June 2026 from its published scan; \
              ABC Color on Decreto 6601 (20 August 2026), Decreto 6280 (29 June 2026), \
              the 1 March 2026 move (16 February 2026), the 2025 additional days and the \
              2026 list (21 September 2026)",
};

// ─────────────────────────────────────────────────────────────────────────
// Honduras
// ─────────────────────────────────────────────────────────────────────────

/// The first Wednesday of the Semana Morazánica: of October from 2015 by
/// Decreto 78-2015, and of November in 2020 alone by Decreto 126-2020.
fn hn_morazanic_wednesday(year: i64) -> Days {
    if year < 2015 {
        return Days::new();
    }
    let month = if year == 2020 { 11 } else { 10 };
    gregorian::to_fixed(year, month, 1).map_or_else(
        |_| Days::new(),
        |first| Days::one(Weekday::Wednesday.on_or_after(first)),
    )
}

/// The Thursday and Friday after that Wednesday.
fn hn_morazanic_thursday_friday(year: i64) -> Days {
    let mut out = Days::new();
    for wednesday in hn_morazanic_wednesday(year).as_slice() {
        out.push(Rd(wednesday.0 + 1));
        out.push(Rd(wednesday.0 + 2));
    }
    out
}

/// Nothing: Decreto 75-2014, which placed the three October days in 2014,
/// was not read, so that year is a gap.
fn hn_unread(_: i64) -> Days {
    Days::new()
}

static HN_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Año Nuevo", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Pan American Day",
        "Día de las Américas",
        Rule::gregorian(4, 14),
    ),
    HolidayRule::fixed_public(
        "Maundy Thursday",
        "Jueves Santo",
        Rule::easter(MAUNDY_THURSDAY),
    ),
    HolidayRule::fixed_public("Good Friday", "Viernes Santo", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Holy Saturday", "Sábado Santo", Rule::easter(HOLY_SATURDAY)),
    HolidayRule::fixed_public("Labour Day", "Día del Trabajo", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public(
        "Independence Day",
        "Día de la Independencia",
        Rule::gregorian(9, 15),
    ),
    // Article 339's three October days, on their dates until Decreto
    // 75-2014 moved them.
    HolidayRule::fixed_public("Soldier's Day", "Día del Soldado", Rule::gregorian(10, 3))
        .years(None, Some(2013)),
    HolidayRule::fixed_public(
        "Discovery of America Day",
        "Día del Descubrimiento de América",
        Rule::gregorian(10, 12),
    )
    .years(None, Some(2013)),
    HolidayRule::fixed_public(
        "Armed Forces Day",
        "Día de las Fuerzas Armadas",
        Rule::gregorian(10, 21),
    )
    .years(None, Some(2013)),
    HolidayRule::fixed_public(
        "Morazanic Week",
        "Semana Morazánica",
        Rule::Tabulated {
            function: hn_unread,
            first_year: 2015,
            last_year: 2015,
        },
    )
    .years(Some(2014), Some(2014)),
    // The private sector's holiday begins at noon on the Wednesday.
    HolidayRule::fixed_public(
        "Morazanic Week",
        "Semana Morazánica",
        Rule::Computed(hn_morazanic_wednesday),
    )
    .of_kind(Kind::Bank),
    HolidayRule::fixed_public(
        "Morazanic Week",
        "Semana Morazánica",
        Rule::Computed(hn_morazanic_thursday_friday),
    ),
    HolidayRule::fixed_public("Christmas Day", "Navidad", Rule::gregorian(12, 25)),
];

/// Honduras.
///
/// Article 339 of the Código del Trabajo: 1 January, 14 April, 1 May, 15
/// September, 3, 12 and 21 October and 25 December "aunque caigan en
/// domingo", and the Thursday, Friday and Saturday of Holy Week; a
/// holiday on a Sunday is paid and not moved. Decreto 75-2014 put the
/// three October days together for tourism, and Decreto 78-2015, in
/// force from September 2015, made them the Semana Morazánica "a partir
/// del primer miércoles del mes de octubre, de cada año": for public
/// employees the Wednesday, Thursday and Friday, for the private sector
/// from noon on the Wednesday to noon on the Saturday. The Wednesday is
/// therefore [`Kind::Bank`] here, the Thursday and Friday days off, and
/// the Saturday morning falls on the weekend. Decreto 126-2020 moved the
/// week to the first Wednesday of November for 2020 alone. The two
/// decrees that describe 75-2014 put its week at the end and at the start
/// of October, and its own text was not read, so 2014 is reported as a
/// gap. The festivity days that article 339's second paragraph
/// leaves to the employer are not carried.
pub static HONDURAS: RuleSet = RuleSet {
    code: "HN",
    english_name: "Honduras",
    rules: HN_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Código del Trabajo, Decreto 189 of 1959, articles 338 and 339, from the \
              edu-honduras.info copy, retrieved 2026-09-23; Decreto 78-2015, La Gaceta \
              33,834 of 14 September 2015, and Decreto 126-2020, La Gaceta 35,387 of 30 \
              September 2020, from the Tribunal Superior de Cuentas (tsc.gob.hn); Infobae \
              (25 August 2026) and La Prensa on the COHEP's dates for 2025 and 2026; \
              Wikipedia, \"Public holidays in Honduras\", for the names",
};

// ─────────────────────────────────────────────────────────────────────────
// El Salvador
// ─────────────────────────────────────────────────────────────────────────

/// The department of San Salvador, whose capital is the city article 190
/// names.
const SV_SAN_SALVADOR: &[&str] = &["SV-SS"];

static SV_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Año Nuevo", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Maundy Thursday",
        "Jueves Santo",
        Rule::easter(MAUNDY_THURSDAY),
    ),
    HolidayRule::fixed_public("Good Friday", "Viernes Santo", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Holy Saturday", "Sábado Santo", Rule::easter(HOLY_SATURDAY)),
    HolidayRule::fixed_public("Labour Day", "Día del Trabajo", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Mother's Day", "Día de la Madre", Rule::gregorian(5, 10))
        .years(Some(2016), None)
        .cited("Decreto Legislativo 339 of 14 April 2016"),
    HolidayRule::fixed_public("Father's Day", "Día del Padre", Rule::gregorian(6, 17))
        .years(Some(2013), None)
        .cited("Decreto Legislativo 208 of 28 November 2012"),
    HolidayRule::fixed_public(
        "Central American and Caribbean Games holiday",
        "Asueto nacional por los XXIV Juegos Centroamericanos y del Caribe",
        Rule::gregorian(7, 7),
    )
    .years(Some(2023), Some(2023))
    .cited("Decreto Legislativo 776 of 20 June 2023"),
    HolidayRule::fixed_public(
        "August Festivities",
        "Fiestas Agostinas",
        Rule::gregorian(8, 3),
    )
    .in_regions(SV_SAN_SALVADOR),
    HolidayRule::fixed_public(
        "August Festivities",
        "Fiestas Agostinas",
        Rule::gregorian(8, 5),
    )
    .in_regions(SV_SAN_SALVADOR),
    HolidayRule::fixed_public(
        "Feast of the Divine Saviour of the World",
        "Fiesta del Divino Salvador del Mundo",
        Rule::gregorian(8, 6),
    ),
    HolidayRule::fixed_public(
        "Independence Day",
        "Día de la Independencia",
        Rule::gregorian(9, 15),
    ),
    HolidayRule::fixed_public(
        "All Souls' Day",
        "Día de los Difuntos",
        Rule::gregorian(11, 2),
    ),
    HolidayRule::fixed_public("Christmas Day", "Navidad", Rule::gregorian(12, 25)),
];

/// El Salvador.
///
/// Article 190 of the Código de Trabajo, in the text of Decreto Legislativo
/// 408 of 1995: 1 January, the Thursday, Friday and Saturday of Holy Week,
/// 1 May, 6 August, 15 September, 2 November and 25 December, and 3 and 5
/// August "en la ciudad de San Salvador". The city is not an ISO 3166-2
/// subdivision, so those two days are scoped to its department, `SV-SS`,
/// which is wider than the Code; the patron-saint day that the same
/// sentence gives every other place "según la costumbre" is not carried.
/// Two decrees outside the Code add days: Father's Day on 17 June "con
/// asueto remunerado", restricted to no sector, from Decreto Legislativo
/// 208 of 2012, in force before 17 June 2013; and Mother's Day on 10 May
/// for the public and private sectors from Decreto Legislativo 339 of 2016,
/// which extended to everyone the public sector's day of 1983 — so the day
/// runs from 2016 here, and the public sector's years before are not
/// carried. Decreto Legislativo 776 of 2023 made 7 July 2023 a national day
/// off for the Central American and Caribbean Games; the other one-off days
/// the Assembly decrees are not carried. Article 194 pays a holiday that
/// falls on the weekly rest day and does not move it.
pub static EL_SALVADOR: RuleSet = RuleSet {
    code: "SV",
    english_name: "El Salvador",
    rules: SV_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Código de Trabajo, Decreto Legislativo 15 of 23 June 1972, articles 190 and \
              194 with the reform list, and Decretos Legislativos 208/2012, 339/2016 and \
              776/2023, from the Asamblea Legislativa's Índice Legislativo \
              (asamblea.gob.sv), retrieved 2026-09-23; the Corte Suprema de Justicia's \
              note on the días de asueto (26 May 2021); Wikipedia, \"Public holidays in \
              El Salvador\", for the English names",
};

// ─────────────────────────────────────────────────────────────────────────
// Nicaragua
// ─────────────────────────────────────────────────────────────────────────

static NI_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Año Nuevo", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Maundy Thursday",
        "Jueves Santo",
        Rule::easter(MAUNDY_THURSDAY),
    ),
    HolidayRule::fixed_public("Good Friday", "Viernes Santo", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Labour Day", "Día del Trabajo", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public(
        "Revolution Day",
        "Día de la Revolución",
        Rule::gregorian(7, 19),
    ),
    // Article 67: Managua's two days of Santo Domingo.
    HolidayRule::fixed_public(
        "Santo Domingo de Guzmán",
        "Fiestas de Santo Domingo de Guzmán",
        Rule::gregorian(8, 1),
    )
    .in_regions(&["NI-MN"]),
    HolidayRule::fixed_public(
        "Santo Domingo de Guzmán",
        "Fiestas de Santo Domingo de Guzmán",
        Rule::gregorian(8, 10),
    )
    .in_regions(&["NI-MN"]),
    HolidayRule::fixed_public(
        "Battle of San Jacinto",
        "Batalla de San Jacinto",
        Rule::gregorian(9, 14),
    ),
    HolidayRule::fixed_public(
        "Independence Day",
        "Día de la Independencia",
        Rule::gregorian(9, 15),
    ),
    HolidayRule::fixed_public(
        "Immaculate Conception",
        "Día de la Inmaculada Concepción",
        Rule::gregorian(12, 8),
    ),
    HolidayRule::fixed_public("Christmas Day", "Navidad", Rule::gregorian(12, 25)),
];

/// Nicaragua.
///
/// Article 66 of the Código del Trabajo, Ley 185 of 1996: 1 January,
/// Maundy Thursday and Good Friday, 1 May, 19 July, 14 and 15 September,
/// 8 and 25 December. Article 67 adds 1 and 10 August "en la ciudad de
/// Managua", scoped here to the department, `NI-MN`, which is wider than
/// the Code, and gives every other place the main day of its festivity
/// "según la costumbre", which is not carried. Article 68 says a holiday
/// that coincides with the seventh day "será compensado" without naming
/// a day, so no substitution is carried. The days of asueto the Executive
/// may declare under article 66 are not carried either.
pub static NICARAGUA: RuleSet = RuleSet {
    code: "NI",
    english_name: "Nicaragua",
    rules: NI_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Ley 185, Código del Trabajo, La Gaceta 205 of 30 October 1996, articles 66 \
              to 68, from the Asamblea Nacional's Normas Jurídicas de Nicaragua \
              (legislacion.asamblea.gob.ni), retrieved 2026-09-23; Wikipedia, \"Public \
              holidays in Nicaragua\", for the English names",
};

// ─────────────────────────────────────────────────────────────────────────
// Antigua and Barbuda
// ─────────────────────────────────────────────────────────────────────────

/// The Schedule's Sunday moves, to the next day not already a holiday:
/// New Year's Day and Boxing Day to the Monday, a Sunday Christmas to the
/// Tuesday after Boxing Day. Independence Day, and from 2020 Christmas and
/// V. C. Bird Day, add their Saturday by their own triggers.
static AG_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// The first Monday in August, the first day of the Carnival holiday.
static AG_CARNIVAL_MONDAY: Rule = Rule::nth(8, 1, Weekday::Monday);

/// A holiday of the 2005 Schedule, carried from its first full year.
const fn ag(rule: HolidayRule) -> HolidayRule {
    rule.years(Some(2006), None)
}

static AG_RULES: &[HolidayRule] = &[
    ag(HolidayRule::public(
        "New Year's Day",
        "",
        Rule::gregorian(1, 1),
    )),
    ag(HolidayRule::fixed_public(
        "Good Friday",
        "",
        Rule::easter(GOOD_FRIDAY),
    )),
    ag(HolidayRule::fixed_public(
        "Easter Monday",
        "",
        Rule::easter(EASTER_MONDAY),
    )),
    ag(HolidayRule::fixed_public(
        "Labour Day",
        "",
        Rule::nth(5, 1, Weekday::Monday),
    )),
    ag(HolidayRule::fixed_public(
        "Whit Monday",
        "",
        Rule::easter(WHIT_MONDAY),
    )),
    ag(HolidayRule::fixed_public(
        "Carnival Monday",
        "",
        AG_CARNIVAL_MONDAY,
    )),
    ag(HolidayRule::fixed_public(
        "Carnival Tuesday",
        "",
        Rule::Offset {
            base: &AG_CARNIVAL_MONDAY,
            days: 1,
        },
    )),
    ag(
        HolidayRule::public("Independence Day", "", Rule::gregorian(11, 1))
            .substitute_on(&[Weekday::Saturday, Weekday::Sunday]),
    ),
    HolidayRule::fixed_public("National Heroes Day", "", Rule::gregorian(12, 9))
        .years(Some(2006), Some(2013)),
    HolidayRule::fixed_public(
        "Sir Vere Cornwall Bird Snr. Day",
        "",
        Rule::gregorian(12, 9),
    )
    .years(Some(2014), Some(2019)),
    HolidayRule::public(
        "Sir Vere Cornwall Bird Snr. Day",
        "",
        Rule::gregorian(12, 9),
    )
    .substitute_on(&[Weekday::Saturday, Weekday::Sunday])
    .years(Some(2020), None),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)).years(Some(2006), Some(2019)),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25))
        .substitute_on(&[Weekday::Saturday, Weekday::Sunday])
        .years(Some(2020), None),
    ag(HolidayRule::public(
        "Boxing Day",
        "",
        Rule::gregorian(12, 26),
    )),
];

/// Antigua and Barbuda.
///
/// The Public Holidays Act, Cap. 354, by the two Schedules that replaced
/// the revised one. The Public Holidays (Amendment) Act 2005, published
/// on 15 September 2005 and carried from 2006: New Year's Day, to the
/// Monday after a Sunday; Independence Day on 1 November, to the Monday
/// after a Saturday or a Sunday; Boxing Day, with the Monday when
/// Christmas is a Saturday and the Monday and Tuesday when it is a
/// Sunday; Easter Monday, Labour Day on the first Monday in May, Whit
/// Monday, the Carnival holiday on the first Monday in August and the
/// Tuesday after it, and National Heroes Day on 9 December, with
/// Christmas and Good Friday as common-law holidays. The Amendment Act
/// 2014, published on 9 October 2014, renamed 9 December Sir Vere
/// Cornwall Bird (Snr) Day. The Amendment Act 2019, published on
/// 31 December 2019 and carried from 2020, wrote Christmas and Good
/// Friday into the Schedule and gave Christmas the Monday and Tuesday
/// after a Saturday or a Sunday, Boxing Day the Monday and Tuesday after
/// a Sunday, and V. C. Bird Day the Monday after a Saturday or a Sunday.
/// The Sunday rules are one policy that takes the next free day, which
/// gives the Schedules' Monday and Tuesday exactly; the Saturday moves
/// are each day's own trigger. The day after a general election, a
/// holiday since the 2014 Act, has no date and is not carried, nor are
/// the special days of section 5 or the 2019 Schedule's Sundays. The
/// revised Schedule it replaced, with amendments to 1991, which had
/// CARICOM Day and neither Carnival Tuesday nor 9 December, is not
/// carried, and the table states nothing before 2006.
pub static ANTIGUA_AND_BARBUDA: RuleSet = RuleSet {
    code: "AG",
    english_name: "Antigua and Barbuda",
    rules: AG_RULES,
    substitution: AG_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Public Holidays Act, Cap. 354, as laws.gov.ag publishes it, and the Public \
              Holidays (Amendment) Acts No. 8 of 2005, No. 8 of 2014 and No. 23 of 2019 \
              (laws.gov.ag), retrieved 2026-09-23, for the Schedules; the Government of \
              Antigua and Barbuda, \"National Holidays\" (ab.gov.ag)",
};

// ─────────────────────────────────────────────────────────────────────────
// Dominica
// ─────────────────────────────────────────────────────────────────────────

/// Every Sunday is in the Schedule, so a holiday on a Sunday is two on
/// one day, and section 9 gives "the next succeeding day not being itself
/// a public holiday".
static DM_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: true,
    valid_from: None,
    valid_until: None,
}];

static DM_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("Carnival Monday", "", Rule::easter(SHROVE_MONDAY)),
    HolidayRule::fixed_public("Carnival Tuesday", "", Rule::easter(SHROVE_TUESDAY)),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::fixed_public("Labour Day", "", Rule::nth(5, 1, Weekday::Monday))
        .years(Some(2021), None),
    HolidayRule::fixed_public("Whit Monday", "", Rule::easter(WHIT_MONDAY)),
    HolidayRule::fixed_public("Emancipation Day", "", Rule::nth(8, 1, Weekday::Monday)),
    HolidayRule::public("Independence Day", "", Rule::gregorian(11, 3)),
    HolidayRule::public(
        "National Day of Community Service",
        "",
        Rule::gregorian(11, 4),
    ),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
];

/// Dominica.
///
/// The Public Holidays Act, Chap. 19:10 (L.R.O. 1/1991), whose Schedule
/// since Act 12 of 1990 lists every Sunday, New Year's Day, Carnival
/// Monday and Tuesday, "the two days immediately preceding Ash
/// Wednesday", Good Friday, Easter Monday, Labour Day, Whit Monday, the
/// first Monday of August, Independence Day and Community Day on 3 and
/// 4 November, Christmas and Boxing Day; section 9 makes "the next
/// succeeding day not being itself a public holiday" a holiday when two
/// fall on one day, and a holiday on a Sunday is one such. The
/// Government's lists for 2021 to 2026 show it: a Sunday New Year's Day
/// in 2023 kept on the 2nd, a Sunday Christmas in 2022 and a Sunday
/// Independence Day in 2024 giving the two days after, Saturdays left
/// alone. The lists name the two days the other way round — Christmas on
/// the Monday, Boxing Day on the Tuesday — and the table's substitute is
/// the Sunday holiday's, on the same days. The Schedule's Labour Day is
/// 1 May, but every list from 2021 keeps it on the first Monday of May,
/// under an order the author did not find; the table follows the lists
/// from 2021 and states no Labour Day before. The names are the lists'.
pub static DOMINICA: RuleSet = RuleSet {
    code: "DM",
    english_name: "Dominica",
    rules: DM_RULES,
    substitution: DM_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Public Holidays Act, Chap. 19:10, L.R.O. 1/1991, as dominica.gov.dm publishes \
              it, retrieved 2026-09-23, for sections 2, 5 and 9 and the Schedule; the \
              Government of Dominica's public holiday lists for 2021 to 2026 \
              (dominica.gov.dm, \"Public Holidays\"), retrieved 2026-09-23",
};

// ─────────────────────────────────────────────────────────────────────────
// Grenada
// ─────────────────────────────────────────────────────────────────────────

/// The Schedule: "when any of the above days falls on a Sunday, the
/// Monday immediately next following shall be a Bank Holiday" — that
/// Monday and no other, so a Sunday Christmas adds nothing to Boxing Day.
static GD_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: false,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// The second Monday in August, the Carnival August Celebrations.
static GD_CARNIVAL_MONDAY: Rule = Rule::nth(8, 2, Weekday::Monday);

/// The day after Carnival Monday.
static GD_CARNIVAL_TUESDAY: Rule = Rule::Offset {
    base: &GD_CARNIVAL_MONDAY,
    days: 1,
};

static GD_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public("Independence Day", "", Rule::gregorian(2, 7)),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("Labour Day", "", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Whit Monday", "", Rule::easter(WHIT_MONDAY)),
    HolidayRule::fixed_public("Corpus Christi", "", Rule::easter(CORPUS_CHRISTI)),
    HolidayRule::fixed_public("Emancipation Holiday", "", Rule::nth(8, 1, Weekday::Monday))
        .years(None, Some(2024)),
    HolidayRule::public("Emancipation Day", "", Rule::gregorian(8, 1)).years(Some(2025), None),
    HolidayRule::fixed_public("Carnival Monday", "", GD_CARNIVAL_MONDAY),
    // "From noon the day following" until the 2023 amendment.
    HolidayRule::fixed_public("Carnival Tuesday", "", GD_CARNIVAL_TUESDAY)
        .of_kind(Kind::Bank)
        .years(None, Some(2023)),
    HolidayRule::fixed_public("Carnival Tuesday", "", GD_CARNIVAL_TUESDAY).years(Some(2024), None),
    HolidayRule::public("National Heroes' Day", "", Rule::gregorian(10, 19))
        .years(Some(2025), None),
    HolidayRule::public("Thanksgiving Day", "", Rule::gregorian(10, 25)),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
];

/// Grenada.
///
/// The Bank Holidays Act, Chapter 25, as revised to Act 19 of 1999, whose
/// section 6 keeps its days at all public offices: New Year's Day,
/// Independence Day on 7 February, Good Friday, Easter Monday, Labour Day
/// on 1 May — "and Indian Arrival Day" by Act 2 of 2017 — Whit Monday,
/// Corpus Christi, the Emancipation Holiday on the first Monday in
/// August, "the second Monday in August and from noon the day following"
/// for Carnival, Thanksgiving Day on 25 October, Christmas and Boxing
/// Day, and "when any of the above days falls on a Sunday, the Monday
/// immediately next following", which the Cabinet Office's lists for
/// 2025 and 2026 repeat: National Heroes' Day on Sunday 19 October 2025
/// kept on the 20th. That Monday is the only day the rule gives, so a
/// Sunday Christmas adds nothing to Boxing Day, and a Saturday holiday
/// stays. Carnival Tuesday is a half day, a bank day here, until the
/// Bank Holidays (Amendment) Bill 2023, laid in August 2023, made it the
/// whole day; the November 2024 Bill quotes that wording as the law, and
/// the table makes the whole day a holiday from 2024. Emancipation Day
/// moved to 1 August by the Bank Holidays (Amendment) Bill 2025, which
/// the Senate passed on 1 July 2025, and is on that date in the 2025
/// list, so the first-Monday holiday ends in 2024.
/// 19 October was National Heroes' Day by proclamation in 2023 and 2024,
/// not carried, and by the 2024 Bill and the lists from 2025.
pub static GRENADA: RuleSet = RuleSet {
    code: "GD",
    english_name: "Grenada",
    rules: GD_RULES,
    substitution: GD_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Bank Holidays Act, Chapter 25, as the Parliament of Grenada publishes it \
              (grenadaparliament.gd), the Bank Holidays (Amendment) Act No. 2 of 2017 \
              (laws.gov.gd) and the Bank Holidays (Amendment) Bill 2023 \
              (grenadaparliament.gd), retrieved 2026-09-23; the Cabinet Office, \
              \"Grenada's Public Holidays\" for 2025 and 2026 (gov.gd); NOW Grenada, \
              \"19 October not among 2023 public holidays\" (13 December 2022), \"Bill for \
              19 October permanent holiday for debate and approval\" (18 November 2024) and \
              \"Education and awareness as Grenada legislates 1 August as Emancipation Day\" \
              (July 2025)",
};

// ─────────────────────────────────────────────────────────────────────────
// Saint Kitts and Nevis
// ─────────────────────────────────────────────────────────────────────────

/// The Schedule's Sunday rule, to the next day not already a holiday: the
/// Monday for New Year's Day, the two September days and Boxing Day, and
/// the Tuesday for a Sunday Christmas, whose Monday is Boxing Day.
static KN_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static KN_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::fixed_public("Labour Day", "", Rule::nth(5, 1, Weekday::Monday)),
    HolidayRule::fixed_public("Whit Monday", "", Rule::easter(WHIT_MONDAY)),
    HolidayRule::fixed_public("Emancipation Day", "", Rule::nth(8, 1, Weekday::Monday)),
    HolidayRule::public("National Heroes Day", "", Rule::gregorian(9, 16)),
    HolidayRule::public("Independence Day", "", Rule::gregorian(9, 19)),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
];

/// Saint Kitts and Nevis.
///
/// The Public Holidays Act, Cap. 23.23, in the revised edition showing
/// the law at 31 December 2002: New Year's Day, Independence Day on
/// 19 September and National Heroes Day on 16 September, "if any of the
/// above-mentioned days fall on a Sunday the next following Monday";
/// Boxing Day, with the Monday after a Saturday Christmas and "the next
/// following Monday and Tuesday" after a Sunday one, as the Government
/// announced for 26 and 27 December 2022; Easter Monday, Labour Day on
/// the first Monday in May, Whit Monday and Emancipation Day on the first
/// Monday in August, with Christmas and Good Friday as common-law
/// holidays. The Schedule's notes date Independence Day's entry to Act 17
/// of 1983 and National Heroes Day's to Act 14 of 1998, and the table
/// gives neither a first year, not knowing which year each was first
/// kept. The Sovereign's birthday "of which due public notice shall be
/// given" is not carried. Carnival Day on 2 January and Culturama Day in
/// August are kept by the Governor-General's proclamation each year and
/// are not carried either, since the author read no proclamation.
pub static SAINT_KITTS_AND_NEVIS: RuleSet = RuleSet {
    code: "KN",
    english_name: "Saint Kitts and Nevis",
    rules: KN_RULES,
    substitution: KN_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Public Holidays Act, Cap. 23.23, Revised Edition to 31 December 2002, as the \
              St. Kitts and Nevis Law Commission publishes it (lawcommission.gov.kn), \
              retrieved 2026-09-23, for sections 2 and 5 and the Schedule; SKNIS, \"St. \
              Kitts and Nevis Public Holidays Announcement\" (7 December 2022)",
};

// ─────────────────────────────────────────────────────────────────────────
// Saint Lucia
// ─────────────────────────────────────────────────────────────────────────

/// The Schedule's note: a day "on a Sunday, or bank holiday" gives "the
/// Monday, or other day, immediately next following not being a holiday".
static LC_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: true,
    valid_from: None,
    valid_until: None,
}];

static LC_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public("New Year's Holiday", "", Rule::gregorian(1, 2)),
    HolidayRule::public("Independence Day", "", Rule::gregorian(2, 22)),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("Labour Day", "", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Whit Monday", "", Rule::easter(WHIT_MONDAY)),
    HolidayRule::fixed_public("Corpus Christi", "", Rule::easter(CORPUS_CHRISTI)),
    HolidayRule::public("Emancipation Day", "", Rule::gregorian(8, 1)),
    HolidayRule::fixed_public("Thanksgiving Day", "", Rule::nth(10, 1, Weekday::Monday))
        .approximate(),
    HolidayRule::public("National Day", "", Rule::gregorian(12, 13)),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
];

/// Saint Lucia.
///
/// The Bank Holidays Act in the Revised Laws of Saint Lucia (2023), as
/// amended to Act 5 of 2004, whose section 8 keeps its days at the public
/// offices: 1 and 2 January, Independence Day on 22 February, Good
/// Friday, Easter Monday, Whit Monday, Labour Day on 1 May, Corpus
/// Christi, Emancipation Day on 1 August, Thanksgiving Day, Saint Lucia
/// Day on 13 December, Christmas and Boxing Day, and "if any of the above
/// days fall on a Sunday, or bank holiday, the Monday, or other day,
/// immediately next following not being a holiday is a bank holiday" —
/// so a Sunday New Year's Day gives the 3rd, the 2nd being a holiday
/// already. The Schedule gives Thanksgiving Day no date; the Cabinet
/// Secretary's list for 2025 puts it on 6 October, the first Monday, and
/// the table carries the first Monday of October as an approximation. The
/// names are that list's. Carnival Monday and Tuesday in July, and the
/// other days the list says "will be by proclamation", are not carried,
/// and the table says nothing of which amendment brought which day.
pub static SAINT_LUCIA: RuleSet = RuleSet {
    code: "LC",
    english_name: "Saint Lucia",
    rules: LC_RULES,
    substitution: LC_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Bank Holidays Act, Revised Laws of Saint Lucia (2023), sections 2, 6 to 8 and \
              the Schedule, as the Attorney General's Chambers published it \
              (attorneygeneralchambers.com, read through the Internet Archive's copy of \
              April 2026), retrieved 2026-09-23; \"Saint Lucia's List of Public Holidays for \
              the Year 2025\", Office of the Cabinet Secretary, 28 August 2024, as the St. \
              Lucia Chamber of Commerce publishes it",
};

// ─────────────────────────────────────────────────────────────────────────
// Saint Vincent and the Grenadines
// ─────────────────────────────────────────────────────────────────────────

/// The lists' footnote, "the HOLIDAY would be celebrated on the following
/// Monday", on every Sunday day they mark, and the Tuesday after a Sunday
/// Christmas.
static VC_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

const VC_CARNIVAL_FIRST: i64 = 2021;
const VC_CARNIVAL_LAST: i64 = 2026;

/// Carnival Monday and Tuesday as the Prime Minister's Office lists them;
/// in 2021 the list moved both to September.
const VC_CARNIVAL: &[(i64, u8, u8, u8)] = &[
    (2021, 9, 6, 7),
    (2022, 7, 4, 5),
    (2023, 7, 10, 11),
    (2024, 7, 8, 9),
    (2025, 7, 7, 8),
    (2026, 7, 6, 7),
];

fn vc_carnival(year: i64, tuesday: bool) -> Days {
    let mut out = Days::new();
    for &(y, month, monday, next) in VC_CARNIVAL {
        if y == year {
            let day = if tuesday { next } else { monday };
            if let Ok(fixed) = gregorian::to_fixed(y, month, day) {
                out.push(fixed);
            }
        }
    }
    out
}

fn vc_carnival_monday(year: i64) -> Days {
    vc_carnival(year, false)
}

fn vc_carnival_tuesday(year: i64) -> Days {
    vc_carnival(year, true)
}

static VC_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public("National Heroes' Day", "", Rule::gregorian(3, 14)),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("National Workers' Day", "", Rule::gregorian(5, 1)),
    HolidayRule::public(
        "Spiritual Baptist Liberation Day",
        "",
        Rule::gregorian(5, 21),
    )
    .years(Some(2025), None),
    HolidayRule::fixed_public("Whit Monday", "", Rule::easter(WHIT_MONDAY)),
    HolidayRule::fixed_public(
        "Carnival Monday",
        "",
        Rule::Tabulated {
            function: vc_carnival_monday,
            first_year: VC_CARNIVAL_FIRST,
            last_year: VC_CARNIVAL_LAST,
        },
    ),
    HolidayRule::fixed_public(
        "Carnival Tuesday",
        "",
        Rule::Tabulated {
            function: vc_carnival_tuesday,
            first_year: VC_CARNIVAL_FIRST,
            last_year: VC_CARNIVAL_LAST,
        },
    ),
    HolidayRule::public("Emancipation Day", "", Rule::gregorian(8, 1)),
    HolidayRule::public("Independence Day", "", Rule::gregorian(10, 27)),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
];

/// Saint Vincent and the Grenadines.
///
/// The Public Holidays Act could not be read; what is carried is the
/// Prime Minister's Office's yearly lists for 2019 and 2021 to 2026. Each
/// fixed day that falls on a Sunday is marked "celebrated on the
/// following Monday" — Independence Day in 2019 and 2024, National
/// Heroes' Day, Emancipation Day and Boxing Day in 2021, National
/// Workers' Day in 2022 — and a Sunday Christmas in 2022 gives Boxing Day
/// the Monday and "the following Tuesday" besides; a Saturday day is
/// never moved, and the 2026 list leaves three on Saturday. The 2023 list
/// has the footnote but marks no day, though New Year's Day was a Sunday;
/// the table gives it the Monday by the rule the other lists show.
/// Carnival Monday and Tuesday move from year to year and are taken from
/// the lists for 2021 to 2026, a year outside them reporting a gap; the
/// 2021 list moved them to 6 and 7 September. Spiritual Baptist
/// Liberation Day on 21 May is in the lists from 2025 and carried from
/// then. National Workers' Day is 1 May in every list.
pub static SAINT_VINCENT_AND_THE_GRENADINES: RuleSet = RuleSet {
    code: "VC",
    english_name: "Saint Vincent and the Grenadines",
    rules: VC_RULES,
    substitution: VC_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Office of the Prime Minister, \"Public Holidays\" (pmoffice.gov.vc), the 2026 \
              list retrieved 2026-09-23 and the lists for 2019 and 2021 to 2025 through the \
              Internet Archive's copies of that page",
};

// ─────────────────────────────────────────────────────────────────────────
// Suriname
// ─────────────────────────────────────────────────────────────────────────

/// A Hijri-dated day of the decree, on the tabular calendar.
const fn sr_hijri(name: &'static str, local: &'static str, month: u8, day: u8) -> HolidayRule {
    HolidayRule::fixed_public(
        name,
        local,
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, month, day),
    )
    .approximate()
}

static SR_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Nieuwjaarsdag", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Chinese New Year",
        "Chinees Nieuwjaar",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 1),
    )
    .approximate()
    .years(Some(2021), None),
    HolidayRule::fixed_public(
        "Day of Liberation and Renewal",
        "Dag van Bevrijding en Vernieuwing",
        Rule::gregorian(2, 25),
    )
    .years(Some(2012), Some(2020)),
    HolidayRule::fixed_public("Holi Phagwa", "Holi-dag", HOLI).approximate(),
    HolidayRule::fixed_public("Good Friday", "Goede Vrijdag", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public(
        "Easter Monday",
        "Tweede Paasdag",
        Rule::easter(EASTER_MONDAY),
    ),
    sr_hijri("Eid al-Fitr", "Ied Ul Fitre", 10, 1),
    HolidayRule::fixed_public("Labour Day", "Dag van de Arbeid", Rule::gregorian(5, 1)),
    sr_hijri("Eid al-Adha", "Ied Ul Adha", 12, 10).years(Some(2012), None),
    HolidayRule::fixed_public("Keti Koti", "Keti Koti Dey", Rule::gregorian(7, 1)),
    HolidayRule::fixed_public(
        "Indigenous People's Day",
        "Dag der Inheemsen",
        Rule::gregorian(8, 9),
    )
    .years(Some(2007), None),
    HolidayRule::fixed_public(
        "Day of the Maroons",
        "Dag der Marrons",
        Rule::gregorian(10, 10),
    )
    .years(Some(2012), None),
    HolidayRule::fixed_public("Divali", "Divali", DIWALI)
        .approximate()
        .years(Some(2012), None),
    HolidayRule::fixed_public(
        "Independence Day",
        "Onafhankelijkheidsdag",
        Rule::gregorian(11, 25),
    ),
    HolidayRule::fixed_public("Christmas Day", "Eerste Kerstdag", Rule::gregorian(12, 25)),
    HolidayRule::fixed_public("Boxing Day", "Tweede Kerstdag", Rule::gregorian(12, 26)),
];

/// Suriname.
///
/// The Besluit Vrije Dagen 1971 (G.B. 1971 no. 78) as S.B. 2021 no. 27
/// replaced its article 1, in force from 23 February 2021: fifteen
/// national free days "met de zondag gelijkgesteld" — Good Friday, Easter
/// Monday, Christmas and Boxing Day; Holi-dag and Divali; Ied Ul Fitre
/// and Ied Ul Adha; New Year's Day, Chinese New Year, Labour Day, Keti
/// Koti Dey on 1 July, the Day of the Indigenous People on 9 August, the
/// Day of the Maroons on 10 October and Independence Day. The earlier
/// amendments are carried as far as they were read: S.B. 2007 no. 98
/// added 9 August from the day after its publication, 9 August 2007;
/// S.B. 2012 no. 21, in force from 21 February 2012, added Divali, Ied
/// Ul Adha, 10 October and the Day of Liberation and Renewal on
/// 25 February, which the 2021 decree removed, so it runs 2012 to 2020.
/// Chinese New Year is in the 2021 decree and not the 2012 one, and a
/// Government notice of 2 February 2021 announced it as a national free
/// day that year; it is carried from 2021, and whatever gave it before
/// is not. Holi, Divali
/// and the two Ieds are dated by the Minister of Home Affairs each year.
/// The crate's Holi rule, the day after Holikā Dahana, gives the days
/// announced for 25 March 2024 and 3 March 2026, and its Lakṣmī Pūjā rule
/// those for 4 November 2021, 24 October 2022 and 12 November 2023; the
/// four are approximate, as the Ieds on the tabular Hijri calendar are,
/// and Chinese New Year, "wisselend" in the decree and announced, is on
/// the crate's Chinese calendar, which gives the days announced for
/// Sunday 22 January 2023 and Saturday 10 February 2024. No text moves a
/// day off a weekend, and those two and Divali on Sunday 12 November 2023
/// stayed where they fell. The one-off days the
/// Government declares — Javanese New Year in 2023, a hundred and seventy
/// years of Chinese immigration — are not carried.
pub static SURINAME: RuleSet = RuleSet {
    code: "SR",
    english_name: "Suriname",
    rules: SR_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Besluit Vrije Dagen 1971 (G.B. 1971 no. 78) and its amendments S.B. 2007 no. \
              98, S.B. 2012 no. 21 and S.B. 2021 no. 27, from the SRIS copies (sris.sr), and \
              S.B. 2021 no. 26 amending the Besluit Rustdagen 1971 (gov.sr), retrieved \
              2026-09-23; the Ministry of Home Affairs' announcements on gov.sr of Holi-dag \
              2024 and 2026, Divali 2021, 2022 and 2023, Ied-ul-Fitr 2023 and 2024, Ied-ul-\
              Adha 2023, Chinese New Year 2021, 2023 and 2024, and Javanese New Year 2023",
};
