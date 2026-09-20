//! Tables for the Americas.

use hc_calendar::Weekday;
use hc_calendars_solar::gregorian;

use crate::computus::offsets::{
    CORPUS_CHRISTI, EASTER_SUNDAY, GOOD_FRIDAY, SHROVE_MONDAY, SHROVE_TUESDAY,
};
use crate::rule::{
    Days, HolidayRule, Kind, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate, SubstituteDirection,
    SubstitutionPolicy,
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
    HolidayRule::public("Washington's Birthday", "", Rule::nth(2, 3, Weekday::Monday))
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
    HolidayRule::public("Nova Scotia Heritage Day", "", Rule::nth(2, 3, Weekday::Monday))
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
    HolidayRule::public("Saint-Jean-Baptiste Day", "Fête nationale du Québec", Rule::gregorian(6, 24))
        .in_regions(CA_QUEBEC),
    HolidayRule::public("Canada Day", "Fête du Canada", Rule::gregorian(7, 1)),
    HolidayRule::public("Civic Holiday", "", Rule::nth(8, 1, Weekday::Monday))
        .in_regions(CA_AUGUST_CIVIC),
    HolidayRule::public("Discovery Day", "", Rule::nth(8, 3, Weekday::Monday))
        .in_regions(CA_YUKON),
    HolidayRule::public("Labour Day", "Fête du Travail", Rule::nth(9, 1, Weekday::Monday)),
    HolidayRule::public(
        "National Day for Truth and Reconciliation",
        "Journée nationale de la vérité et de la réconciliation",
        Rule::gregorian(9, 30),
    )
    .years(Some(2021), None),
    HolidayRule::public("Thanksgiving", "Action de grâce", Rule::nth(10, 2, Weekday::Monday)),
    HolidayRule::public("Remembrance Day", "Jour du Souvenir", Rule::gregorian(11, 11)),
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
    let (first_year, month) = if year >= 2024 { (2024i64, 10u8) } else { (1934, 12) };
    if year < first_year || (year - first_year).rem_euclid(6) != 0 {
        return Days::new();
    }
    gregorian::to_fixed(year, month, 1).map_or_else(|_| Days::new(), Days::one)
}

static MX_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Año Nuevo", Rule::gregorian(1, 1)),
    // The 2006 reform moved three fixed dates onto Mondays.
    HolidayRule::fixed_public("Constitution Day", "Día de la Constitución", Rule::gregorian(2, 5))
        .years(None, Some(2005)),
    HolidayRule::fixed_public("Constitution Day", "Día de la Constitución", Rule::nth(2, 1, Weekday::Monday))
        .years(Some(2006), None),
    HolidayRule::fixed_public("Benito Juárez's Birthday", "Natalicio de Benito Juárez", Rule::gregorian(3, 21))
        .years(None, Some(2005)),
    HolidayRule::fixed_public("Benito Juárez's Birthday", "Natalicio de Benito Juárez", Rule::nth(3, 3, Weekday::Monday))
        .years(Some(2006), None),
    HolidayRule::fixed_public("Labour Day", "Día del Trabajo", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Independence Day", "Día de la Independencia", Rule::gregorian(9, 16)),
    HolidayRule::fixed_public("Revolution Day", "Día de la Revolución", Rule::gregorian(11, 20))
        .years(None, Some(2005)),
    HolidayRule::fixed_public("Revolution Day", "Día de la Revolución", Rule::nth(11, 3, Weekday::Monday))
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
    HolidayRule::fixed_public("New Year's Day", "Confraternização Universal", Rule::gregorian(1, 1)),
    // Carnival is a ponto facultativo, not a feriado nacional, however
    // universally the country stops.
    HolidayRule::observance("Carnival Monday", "Segunda-feira de Carnaval", Rule::easter(SHROVE_MONDAY)),
    HolidayRule::public("Carnival Tuesday", "Terça-feira de Carnaval", Rule::easter(SHROVE_TUESDAY))
        .of_kind(Kind::Bank),
    HolidayRule::fixed_public("Good Friday", "Sexta-feira da Paixão", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::observance("Easter Sunday", "Páscoa", Rule::easter(EASTER_SUNDAY)),
    HolidayRule::fixed_public("Tiradentes", "Tiradentes", Rule::gregorian(4, 21)),
    HolidayRule::fixed_public("Labour Day", "Dia do Trabalhador", Rule::gregorian(5, 1)),
    HolidayRule::public("Corpus Christi", "Corpus Christi", Rule::easter(CORPUS_CHRISTI))
        .of_kind(Kind::Bank),
    HolidayRule::fixed_public("Independence Day", "Independência do Brasil", Rule::gregorian(9, 7)),
    HolidayRule::fixed_public("Our Lady of Aparecida", "Nossa Senhora Aparecida", Rule::gregorian(10, 12))
        .years(Some(1980), None),
    HolidayRule::fixed_public("All Souls' Day", "Finados", Rule::gregorian(11, 2)),
    HolidayRule::fixed_public("Republic Day", "Proclamação da República", Rule::gregorian(11, 15)),
    HolidayRule::fixed_public("Black Awareness Day", "Dia da Consciência Negra", Rule::gregorian(11, 20))
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
