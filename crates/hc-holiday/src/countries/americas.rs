//! Tables for the Americas.

use hc_calendar::Weekday;
use hc_calendars_solar::gregorian;

use crate::computus::offsets::{
    ASCENSION, CORPUS_CHRISTI, EASTER_SUNDAY, GOOD_FRIDAY, MAUNDY_THURSDAY, SACRED_HEART,
    SHROVE_MONDAY, SHROVE_TUESDAY,
};
use crate::rule::{
    CalendarSystem, Days, HolidayRule, Kind, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate,
    SubstituteDirection, SubstitutionPolicy, TO_ADJACENT_MONDAY, TO_FOLLOWING_MONDAY,
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
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "5 U.S.C. § 6103; Uniform Monday Holiday Act (Pub. L. 90-363); \
              Pub. L. 94-97 restoring Veterans Day; Pub. L. 117-17 for \
              Juneteenth; Executive Orders 10358 and 11582 for the in-lieu-of \
              rule. State holidays are not modelled: there are no national \
              public holidays in the United States, only federal ones",
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
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Canada Labour Code s. 166 and the Holidays Act (R.S.C. 1985, \
              c. H-5); provincial employment-standards acts for the \
              subdivision entries. Remembrance Day is federal and is not \
              statutory in every province; the provincial variation is not \
              modelled",
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
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Ley Federal del Trabajo, artículo 74, as reformed in 2006 and \
              2024. Religious days — Semana Santa, 12 December — are not \
              días de descanso obligatorio and are not listed",
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
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Lei 662/1949 and lei 10.607/2002; lei 6.802/1980 for Nossa \
              Senhora Aparecida; lei 14.759/2023 making Consciência Negra a \
              national holiday from 2024. Carnival and Corpus Christi are \
              pontos facultativos, recorded here as bank holidays",
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
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Ley 51 de 1983, article 1 (funcionpublica.gov.co, gestor \
              normativo, retrieved 2026-09-22), for the list and the Monday \
              rule; Wikipedia (es), \"Anexo:Días festivos en Colombia\", \
              retrieved 2026-09-22, for the 2026 dates the tests check",
};
