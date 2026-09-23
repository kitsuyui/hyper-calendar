//! European national tables.
//!
//! Most of Europe is the easy case for this crate's design: a handful of
//! fixed Gregorian dates plus the Easter cycle, which is why the Easter
//! offsets live in [`crate::computus::offsets`] and no country repeats them.
//! Greece is the one that pays for the second computus.

use hc_calendar::{Rd, Weekday};

use hc_calendars_solar::gregorian;

use crate::computus::offsets::{
    ASCENSION, ASH_WEDNESDAY, CORPUS_CHRISTI, EASTER_MONDAY, EASTER_SUNDAY, GOOD_FRIDAY,
    HOLY_SATURDAY, MAUNDY_THURSDAY, PENTECOST, SHROVE_MONDAY, SHROVE_TUESDAY, WHIT_MONDAY,
};
use crate::rule::{
    CalendarSystem, Days, HolidayRule, Kind, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate,
    SubstituteDirection, SubstitutionPolicy, WeekendPolicy,
};

/// The British and Irish shift: a bank holiday on a weekend is kept on the
/// next weekday that is not already one.
static BRITISH_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

// ─────────────────────────────────────────────────────────────────────────
// United Kingdom
// ─────────────────────────────────────────────────────────────────────────

/// England and Wales.
pub const GB_ENGLAND_AND_WALES: &str = "GB-EAW";
/// Scotland.
pub const GB_SCOTLAND: &str = "GB-SCT";
/// Northern Ireland.
pub const GB_NORTHERN_IRELAND: &str = "GB-NIR";

const SCT: &[&str] = &[GB_SCOTLAND];
const NIR: &[&str] = &[GB_NORTHERN_IRELAND];
const EAW_NIR: &[&str] = &[GB_ENGLAND_AND_WALES, GB_NORTHERN_IRELAND];

static UK_RULES: &[HolidayRule] = &[
    // New Year's Day became a bank holiday in England, Wales and Northern
    // Ireland only in 1974; Scotland has kept it since 1871.
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)).years(Some(1974), None),
    HolidayRule::public("2 January", "", Rule::gregorian(1, 2))
        .in_regions(SCT)
        .years(Some(1974), None),
    HolidayRule::public("St Patrick's Day", "", Rule::gregorian(3, 17)).in_regions(NIR),
    HolidayRule::public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    // Scotland has never had Easter Monday as a bank holiday.
    HolidayRule::public("Easter Monday", "", Rule::easter(EASTER_MONDAY)).in_regions(EAW_NIR),
    // The Early May Bank Holiday dates from 1978. It was moved to 8 May in
    // 1995 and again in 2020 for the fiftieth and seventy-fifth
    // anniversaries of VE Day.
    HolidayRule::public(
        "Early May Bank Holiday",
        "",
        Rule::nth(5, 1, Weekday::Monday),
    )
    .years(Some(1978), Some(1994)),
    HolidayRule::public("Early May Bank Holiday", "", Rule::gregorian(5, 8))
        .years(Some(1995), Some(1995)),
    HolidayRule::public(
        "Early May Bank Holiday",
        "",
        Rule::nth(5, 1, Weekday::Monday),
    )
    .years(Some(1996), Some(2019)),
    HolidayRule::public("Early May Bank Holiday", "", Rule::gregorian(5, 8))
        .years(Some(2020), Some(2020)),
    HolidayRule::public(
        "Early May Bank Holiday",
        "",
        Rule::nth(5, 1, Weekday::Monday),
    )
    .years(Some(2021), None),
    // The Spring Bank Holiday was moved into June three times, each for a
    // royal jubilee.
    HolidayRule::public("Spring Bank Holiday", "", Rule::last(5, Weekday::Monday))
        .years(Some(1971), Some(2001)),
    HolidayRule::public("Spring Bank Holiday", "", Rule::gregorian(6, 4))
        .years(Some(2002), Some(2002)),
    HolidayRule::public("Spring Bank Holiday", "", Rule::last(5, Weekday::Monday))
        .years(Some(2003), Some(2011)),
    HolidayRule::public("Spring Bank Holiday", "", Rule::gregorian(6, 4))
        .years(Some(2012), Some(2012)),
    HolidayRule::public("Spring Bank Holiday", "", Rule::last(5, Weekday::Monday))
        .years(Some(2013), Some(2021)),
    HolidayRule::public("Spring Bank Holiday", "", Rule::gregorian(6, 2))
        .years(Some(2022), Some(2022)),
    HolidayRule::public("Spring Bank Holiday", "", Rule::last(5, Weekday::Monday))
        .years(Some(2023), None),
    HolidayRule::public("Battle of the Boyne", "", Rule::gregorian(7, 12)).in_regions(NIR),
    // Scotland's summer bank holiday is the first Monday of August, not the
    // last: the 1971 Act left the Scottish dates where custom had them.
    HolidayRule::public("Summer Bank Holiday", "", Rule::nth(8, 1, Weekday::Monday))
        .in_regions(SCT),
    HolidayRule::public("Summer Bank Holiday", "", Rule::last(8, Weekday::Monday))
        .in_regions(EAW_NIR)
        .years(Some(1971), None),
    HolidayRule::public("St Andrew's Day", "", Rule::gregorian(11, 30))
        .in_regions(SCT)
        .years(Some(2007), None),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
    // Royal and national one-offs, each proclaimed under section 1(3) of the
    // Banking and Financial Dealings Act 1971.
    HolidayRule::fixed_public("Silver Jubilee of Elizabeth II", "", Rule::gregorian(6, 7))
        .years(Some(1977), Some(1977)),
    HolidayRule::fixed_public("Wedding of the Prince of Wales", "", Rule::gregorian(7, 29))
        .years(Some(1981), Some(1981)),
    HolidayRule::fixed_public("Millennium Eve", "", Rule::gregorian(12, 31))
        .years(Some(1999), Some(1999)),
    HolidayRule::fixed_public("Golden Jubilee of Elizabeth II", "", Rule::gregorian(6, 3))
        .years(Some(2002), Some(2002)),
    HolidayRule::fixed_public("Wedding of Prince William", "", Rule::gregorian(4, 29))
        .years(Some(2011), Some(2011)),
    HolidayRule::fixed_public("Diamond Jubilee of Elizabeth II", "", Rule::gregorian(6, 5))
        .years(Some(2012), Some(2012)),
    HolidayRule::fixed_public(
        "Platinum Jubilee of Elizabeth II",
        "",
        Rule::gregorian(6, 3),
    )
    .years(Some(2022), Some(2022)),
    HolidayRule::fixed_public("State Funeral of Elizabeth II", "", Rule::gregorian(9, 19))
        .years(Some(2022), Some(2022)),
    HolidayRule::fixed_public("Coronation of Charles III", "", Rule::gregorian(5, 8))
        .years(Some(2023), Some(2023)),
];

/// The United Kingdom, with its three bank-holiday jurisdictions as regions.
pub static UNITED_KINGDOM: RuleSet = RuleSet {
    code: "GB",
    english_name: "United Kingdom",
    rules: UK_RULES,
    substitution: BRITISH_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Banking and Financial Dealings Act 1971, schedule 1, together \
              with the royal proclamations made under section 1(3); GOV.UK \
              \"UK bank holidays\"",
};

// ─────────────────────────────────────────────────────────────────────────
// Ireland
// ─────────────────────────────────────────────────────────────────────────

/// St Brigid's Day, the one Irish rule that is a sentence and not a pattern.
///
/// The Organisation of Working Time (Amendment) Act 2022 puts it on the
/// first Monday of February, "except where St Brigid's Day, being the first
/// day of February, falls on a Friday, in which case that Friday". Nothing
/// in the vocabulary expresses a conditional between two shapes, so this is
/// a [`Rule::Computed`] rule.
fn st_brigids_day(year: i64) -> Days {
    let first_of_february = Rule::gregorian(2, 1).days_in_year(year);
    let Some(first) = first_of_february.as_slice().first().copied() else {
        return Days::new();
    };
    if Weekday::from_rd(first) == Weekday::Friday {
        return Days::one(first);
    }
    Rule::nth(2, 1, Weekday::Monday).days_in_year(year)
}

static IE_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Lá Caille", Rule::gregorian(1, 1)),
    HolidayRule::public(
        "St Brigid's Day",
        "Lá Fhéile Bríde",
        Rule::Computed(st_brigids_day),
    )
    .years(Some(2023), None),
    HolidayRule::public(
        "St Patrick's Day",
        "Lá Fhéile Pádraig",
        Rule::gregorian(3, 17),
    ),
    HolidayRule::public("Easter Monday", "Luan Cásca", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("May Day", "Lá Bealtaine", Rule::nth(5, 1, Weekday::Monday))
        .years(Some(1994), None),
    HolidayRule::public("June Bank Holiday", "", Rule::nth(6, 1, Weekday::Monday)),
    HolidayRule::public("August Bank Holiday", "", Rule::nth(8, 1, Weekday::Monday)),
    HolidayRule::public("October Bank Holiday", "", Rule::last(10, Weekday::Monday)),
    HolidayRule::public("Christmas Day", "Lá Nollag", Rule::gregorian(12, 25)),
    HolidayRule::public(
        "St Stephen's Day",
        "Lá Fhéile Stiofáin",
        Rule::gregorian(12, 26),
    ),
    // 2020 and 2021 each had a one-off day marking the pandemic response;
    // 2022-03-18 was proclaimed for the same reason.
    HolidayRule::fixed_public("National Day of Commemoration", "", Rule::gregorian(3, 18))
        .years(Some(2022), Some(2022)),
];

/// Ireland.
pub static IRELAND: RuleSet = RuleSet {
    code: "IE",
    english_name: "Ireland",
    rules: IE_RULES,
    substitution: BRITISH_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Organisation of Working Time Act 1997, second schedule, as \
              amended by the Organisation of Working Time (Amendment) Act 2022",
};

// ─────────────────────────────────────────────────────────────────────────
// France
// ─────────────────────────────────────────────────────────────────────────

/// The three départements where the Napoleonic concordat survived the
/// 1905 separation of church and state, and with it two extra holidays.
const ALSACE_MOSELLE: &[&str] = &["FR-57", "FR-67", "FR-68"];

static FR_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Jour de l'An", Rule::gregorian(1, 1)),
    HolidayRule::public("Good Friday", "Vendredi saint", Rule::easter(GOOD_FRIDAY))
        .in_regions(ALSACE_MOSELLE),
    HolidayRule::public(
        "Easter Monday",
        "Lundi de Pâques",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::public("Labour Day", "Fête du Travail", Rule::gregorian(5, 1))
        .years(Some(1948), None),
    // 8 May was a holiday from 1953, abolished by décret in 1959 and
    // restored by the loi du 2 octobre 1981, first kept again in 1982.
    HolidayRule::public(
        "Victory in Europe Day",
        "Victoire 1945",
        Rule::gregorian(5, 8),
    )
    .years(Some(1953), Some(1958)),
    HolidayRule::public(
        "Victory in Europe Day",
        "Victoire 1945",
        Rule::gregorian(5, 8),
    )
    .years(Some(1982), None),
    HolidayRule::public("Ascension", "Ascension", Rule::easter(ASCENSION)),
    HolidayRule::public(
        "Whit Monday",
        "Lundi de Pentecôte",
        Rule::easter(WHIT_MONDAY),
    ),
    HolidayRule::public("Bastille Day", "Fête nationale", Rule::gregorian(7, 14)),
    HolidayRule::public("Assumption", "Assomption", Rule::gregorian(8, 15)),
    HolidayRule::public("All Saints' Day", "Toussaint", Rule::gregorian(11, 1)),
    HolidayRule::public("Armistice Day", "Armistice 1918", Rule::gregorian(11, 11))
        .years(Some(1922), None),
    HolidayRule::public("Christmas Day", "Noël", Rule::gregorian(12, 25)),
    HolidayRule::public("St Stephen's Day", "Saint Étienne", Rule::gregorian(12, 26))
        .in_regions(ALSACE_MOSELLE),
];

/// France.
pub static FRANCE: RuleSet = RuleSet {
    code: "FR",
    english_name: "France",
    rules: FR_RULES,
    // France has no weekend-substitution rule: a holiday falling on a Sunday
    // simply disappears, which is why the "pont" is a matter of agreement
    // rather than law.
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Code du travail, article L3133-1; loi du 2 octobre 1981 for 8 \
              May; Code du travail local d'Alsace-Moselle, articles 105a-105i",
};

// ─────────────────────────────────────────────────────────────────────────
// Germany
// ─────────────────────────────────────────────────────────────────────────

const DE_EPIPHANY: &[&str] = &["DE-BW", "DE-BY", "DE-ST"];
const DE_CORPUS_CHRISTI: &[&str] = &["DE-BW", "DE-BY", "DE-HE", "DE-NW", "DE-RP", "DE-SL"];
const DE_ALL_SAINTS: &[&str] = &["DE-BW", "DE-BY", "DE-NW", "DE-RP", "DE-SL"];
const DE_REFORMATION_EAST: &[&str] = &["DE-BB", "DE-MV", "DE-SN", "DE-ST", "DE-TH"];
const DE_REFORMATION_NORTH: &[&str] = &["DE-HB", "DE-HH", "DE-NI", "DE-SH"];
const DE_SAARLAND: &[&str] = &["DE-SL"];
const DE_SAXONY: &[&str] = &["DE-SN"];
const DE_BERLIN: &[&str] = &["DE-BE"];
const DE_MECKLENBURG: &[&str] = &["DE-MV"];
const DE_THURINGIA: &[&str] = &["DE-TH"];

static DE_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Neujahrstag", Rule::gregorian(1, 1)),
    HolidayRule::public("Epiphany", "Heilige Drei Könige", Rule::gregorian(1, 6))
        .in_regions(DE_EPIPHANY),
    HolidayRule::public(
        "International Women's Day",
        "Internationaler Frauentag",
        Rule::gregorian(3, 8),
    )
    .in_regions(DE_BERLIN)
    .years(Some(2019), None),
    HolidayRule::public(
        "International Women's Day",
        "Internationaler Frauentag",
        Rule::gregorian(3, 8),
    )
    .in_regions(DE_MECKLENBURG)
    .years(Some(2023), None),
    HolidayRule::public("Good Friday", "Karfreitag", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public("Easter Monday", "Ostermontag", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("Labour Day", "Tag der Arbeit", Rule::gregorian(5, 1)),
    // Berlin gave a single extra day for the seventy-fifth and eightieth
    // anniversaries of the end of the war in Europe.
    HolidayRule::fixed_public("Liberation Day", "Tag der Befreiung", Rule::gregorian(5, 8))
        .in_regions(DE_BERLIN)
        .years(Some(2020), Some(2020)),
    HolidayRule::fixed_public("Liberation Day", "Tag der Befreiung", Rule::gregorian(5, 8))
        .in_regions(DE_BERLIN)
        .years(Some(2025), Some(2025)),
    HolidayRule::public("Ascension", "Christi Himmelfahrt", Rule::easter(ASCENSION)),
    HolidayRule::public("Whit Monday", "Pfingstmontag", Rule::easter(WHIT_MONDAY)),
    HolidayRule::public(
        "Corpus Christi",
        "Fronleichnam",
        Rule::easter(CORPUS_CHRISTI),
    )
    .in_regions(DE_CORPUS_CHRISTI),
    HolidayRule::public("Assumption", "Mariä Himmelfahrt", Rule::gregorian(8, 15))
        .in_regions(DE_SAARLAND),
    HolidayRule::public(
        "World Children's Day",
        "Weltkindertag",
        Rule::gregorian(9, 20),
    )
    .in_regions(DE_THURINGIA)
    .years(Some(2019), None),
    HolidayRule::public(
        "German Unity Day",
        "Tag der Deutschen Einheit",
        Rule::gregorian(10, 3),
    )
    .years(Some(1990), None),
    HolidayRule::public(
        "Reformation Day",
        "Reformationstag",
        Rule::gregorian(10, 31),
    )
    .in_regions(DE_REFORMATION_EAST)
    .years(Some(1990), None),
    HolidayRule::public(
        "Reformation Day",
        "Reformationstag",
        Rule::gregorian(10, 31),
    )
    .in_regions(DE_REFORMATION_NORTH)
    .years(Some(2018), None),
    HolidayRule::public("All Saints' Day", "Allerheiligen", Rule::gregorian(11, 1))
        .in_regions(DE_ALL_SAINTS),
    // Buß- und Bettag is the Wednesday before 23 November. It was federal
    // until 1994, when it was dropped everywhere but Saxony to pay for
    // long-term care insurance.
    HolidayRule::public(
        "Day of Prayer and Repentance",
        "Buß- und Bettag",
        Rule::WeekdayOnOrBefore {
            month: 11,
            day: 22,
            weekday: Weekday::Wednesday,
        },
    )
    .years(None, Some(1994)),
    HolidayRule::public(
        "Day of Prayer and Repentance",
        "Buß- und Bettag",
        Rule::WeekdayOnOrBefore {
            month: 11,
            day: 22,
            weekday: Weekday::Wednesday,
        },
    )
    .in_regions(DE_SAXONY)
    .years(Some(1995), None),
    HolidayRule::public(
        "Christmas Day",
        "Erster Weihnachtstag",
        Rule::gregorian(12, 25),
    ),
    HolidayRule::public(
        "St Stephen's Day",
        "Zweiter Weihnachtstag",
        Rule::gregorian(12, 26),
    ),
];

/// Germany, with all sixteen *Länder* as regions.
pub static GERMANY: RuleSet = RuleSet {
    code: "DE",
    english_name: "Germany",
    rules: DE_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Feiertagsgesetze of the sixteen Länder; \
              Einigungsvertrag Art. 2 for 3 October; \
              Pflege-Versicherungsgesetz 1994 for Buß- und Bettag. \
              Mariä Himmelfahrt is listed for Saarland only; in Bavaria it \
              applies in predominantly Catholic municipalities, which is a \
              parish-level distinction this crate does not model",
};

// ─────────────────────────────────────────────────────────────────────────
// Italy, Spain, Portugal
// ─────────────────────────────────────────────────────────────────────────

static IT_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Capodanno", Rule::gregorian(1, 1)),
    HolidayRule::public("Epiphany", "Epifania", Rule::gregorian(1, 6)),
    HolidayRule::public("Easter Sunday", "Pasqua", Rule::easter(EASTER_SUNDAY)),
    HolidayRule::public(
        "Easter Monday",
        "Lunedì dell'Angelo",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::public(
        "Liberation Day",
        "Festa della Liberazione",
        Rule::gregorian(4, 25),
    )
    .years(Some(1946), None),
    HolidayRule::public("Labour Day", "Festa del Lavoro", Rule::gregorian(5, 1)),
    // Republic Day was a working day from 1977 to 2000, kept on the first
    // Sunday of June instead; law 336/2000 put it back on 2 June.
    HolidayRule::public(
        "Republic Day",
        "Festa della Repubblica",
        Rule::gregorian(6, 2),
    )
    .years(Some(1949), Some(1976)),
    HolidayRule::public(
        "Republic Day",
        "Festa della Repubblica",
        Rule::gregorian(6, 2),
    )
    .years(Some(2001), None),
    HolidayRule::public("Assumption", "Ferragosto", Rule::gregorian(8, 15)),
    HolidayRule::public("All Saints' Day", "Ognissanti", Rule::gregorian(11, 1)),
    HolidayRule::public(
        "Immaculate Conception",
        "Immacolata Concezione",
        Rule::gregorian(12, 8),
    ),
    HolidayRule::public("Christmas Day", "Natale", Rule::gregorian(12, 25)),
    HolidayRule::public("St Stephen's Day", "Santo Stefano", Rule::gregorian(12, 26)),
];

/// Italy.
pub static ITALY: RuleSet = RuleSet {
    code: "IT",
    english_name: "Italy",
    rules: IT_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Legge 27 maggio 1949 n. 260 and its amendments; legge 20 \
              novembre 2000 n. 336 for Republic Day. Municipal patron-saint \
              days are real holidays but are not modelled",
};

static ES_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Año Nuevo", Rule::gregorian(1, 1)),
    HolidayRule::public("Epiphany", "Epifanía del Señor", Rule::gregorian(1, 6)),
    HolidayRule::public("Good Friday", "Viernes Santo", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public("Labour Day", "Fiesta del Trabajo", Rule::gregorian(5, 1)),
    HolidayRule::public(
        "Assumption",
        "Asunción de la Virgen",
        Rule::gregorian(8, 15),
    ),
    HolidayRule::public(
        "National Day",
        "Fiesta Nacional de España",
        Rule::gregorian(10, 12),
    ),
    HolidayRule::public(
        "All Saints' Day",
        "Todos los Santos",
        Rule::gregorian(11, 1),
    ),
    HolidayRule::public(
        "Constitution Day",
        "Día de la Constitución",
        Rule::gregorian(12, 6),
    )
    .years(Some(1983), None),
    HolidayRule::public(
        "Immaculate Conception",
        "Inmaculada Concepción",
        Rule::gregorian(12, 8),
    ),
    HolidayRule::public(
        "Christmas Day",
        "Natividad del Señor",
        Rule::gregorian(12, 25),
    ),
];

/// Spain, national calendar only.
pub static SPAIN: RuleSet = RuleSet {
    code: "ES",
    english_name: "Spain",
    rules: ES_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Estatuto de los Trabajadores art. 37.2 and the annual \
              Resolución de la Dirección General de Trabajo. Maundy \
              Thursday and the autonomous communities' own days are not \
              modelled: each community may move a Sunday holiday to the \
              following Monday and substitute two of its own, which is a \
              yearly administrative act rather than a rule",
};

static PT_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Ano Novo", Rule::gregorian(1, 1)),
    HolidayRule::observance("Carnival", "Carnaval", Rule::easter(SHROVE_TUESDAY)),
    HolidayRule::public(
        "Good Friday",
        "Sexta-feira Santa",
        Rule::easter(GOOD_FRIDAY),
    ),
    HolidayRule::public("Easter Sunday", "Páscoa", Rule::easter(EASTER_SUNDAY)),
    HolidayRule::public("Freedom Day", "Dia da Liberdade", Rule::gregorian(4, 25))
        .years(Some(1974), None),
    HolidayRule::public("Labour Day", "Dia do Trabalhador", Rule::gregorian(5, 1)),
    // Austerity suspended four holidays from 2013 to 2015; they came back in
    // 2016 under lei 8/2016.
    HolidayRule::public(
        "Corpus Christi",
        "Corpo de Deus",
        Rule::easter(CORPUS_CHRISTI),
    )
    .years(None, Some(2012)),
    HolidayRule::public(
        "Corpus Christi",
        "Corpo de Deus",
        Rule::easter(CORPUS_CHRISTI),
    )
    .years(Some(2016), None),
    HolidayRule::public("Portugal Day", "Dia de Portugal", Rule::gregorian(6, 10)),
    HolidayRule::public(
        "Assumption",
        "Assunção de Nossa Senhora",
        Rule::gregorian(8, 15),
    ),
    HolidayRule::public(
        "Republic Day",
        "Implantação da República",
        Rule::gregorian(10, 5),
    )
    .years(None, Some(2012)),
    HolidayRule::public(
        "Republic Day",
        "Implantação da República",
        Rule::gregorian(10, 5),
    )
    .years(Some(2016), None),
    HolidayRule::public("All Saints' Day", "Todos os Santos", Rule::gregorian(11, 1))
        .years(None, Some(2012)),
    HolidayRule::public("All Saints' Day", "Todos os Santos", Rule::gregorian(11, 1))
        .years(Some(2016), None),
    HolidayRule::public(
        "Restoration of Independence",
        "Restauração da Independência",
        Rule::gregorian(12, 1),
    )
    .years(None, Some(2012)),
    HolidayRule::public(
        "Restoration of Independence",
        "Restauração da Independência",
        Rule::gregorian(12, 1),
    )
    .years(Some(2016), None),
    HolidayRule::public(
        "Immaculate Conception",
        "Imaculada Conceição",
        Rule::gregorian(12, 8),
    ),
    HolidayRule::public("Christmas Day", "Natal", Rule::gregorian(12, 25)),
];

/// Portugal.
pub static PORTUGAL: RuleSet = RuleSet {
    code: "PT",
    english_name: "Portugal",
    rules: PT_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Código do Trabalho art. 234; lei 23/2012 for the suspension \
              and lei 8/2016 for the restoration of the four holidays",
};

// ─────────────────────────────────────────────────────────────────────────
// The Low Countries, Switzerland and Austria
// ─────────────────────────────────────────────────────────────────────────

/// King's Day, the other rule that is a sentence.
///
/// Since 2014 it is 27 April, "or, if that is a Sunday, 26 April" — the
/// Dutch royal birthday has always been moved *backwards*, unlike every
/// other substitution in this crate. Queen's Day, 30 April, worked the same
/// way from 1980, moving to 29 April on a Sunday.
fn dutch_royal_day(year: i64) -> Days {
    let (month, day) = if year >= 2014 { (4u8, 27u8) } else { (4, 30) };
    let occurrence = Rule::gregorian(month, day).days_in_year(year);
    let Some(date) = occurrence.as_slice().first().copied() else {
        return Days::new();
    };
    if Weekday::from_rd(date) == Weekday::Sunday && year >= 1980 {
        return Days::one(hc_calendar::Rd(date.0 - 1));
    }
    Days::one(date)
}

static NL_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Nieuwjaarsdag", Rule::gregorian(1, 1)),
    // Good Friday is an official holiday but not, for most employers, a paid
    // day off, so it is recorded as an observance rather than a day off.
    HolidayRule::observance("Good Friday", "Goede Vrijdag", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public(
        "Easter Sunday",
        "Eerste Paasdag",
        Rule::easter(EASTER_SUNDAY),
    ),
    HolidayRule::public(
        "Easter Monday",
        "Tweede Paasdag",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::fixed_public("King's Day", "Koningsdag", Rule::Computed(dutch_royal_day))
        .years(Some(2014), None),
    HolidayRule::fixed_public(
        "Queen's Day",
        "Koninginnedag",
        Rule::Computed(dutch_royal_day),
    )
    .years(Some(1949), Some(2013)),
    // Liberation Day is a day off for most only in years divisible by five.
    HolidayRule::observance("Liberation Day", "Bevrijdingsdag", Rule::gregorian(5, 5)),
    HolidayRule::public("Ascension", "Hemelvaartsdag", Rule::easter(ASCENSION)),
    HolidayRule::public("Pentecost", "Eerste Pinksterdag", Rule::easter(PENTECOST)),
    HolidayRule::public(
        "Whit Monday",
        "Tweede Pinksterdag",
        Rule::easter(WHIT_MONDAY),
    ),
    HolidayRule::public("Christmas Day", "Eerste Kerstdag", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "Tweede Kerstdag", Rule::gregorian(12, 26)),
];

/// The Netherlands.
pub static NETHERLANDS: RuleSet = RuleSet {
    code: "NL",
    english_name: "Netherlands",
    rules: NL_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Algemene termijnenwet art. 3; Wet van 2013 for Koningsdag. \
              Goede Vrijdag and Bevrijdingsdag are official but are a paid \
              day off only by collective agreement, so they are recorded as \
              observances",
};

static BE_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Nieuwjaar", Rule::gregorian(1, 1)),
    HolidayRule::public("Easter Sunday", "Pasen", Rule::easter(EASTER_SUNDAY)),
    HolidayRule::public("Easter Monday", "Paasmaandag", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("Labour Day", "Dag van de Arbeid", Rule::gregorian(5, 1)),
    HolidayRule::public(
        "Ascension",
        "Onze-Lieve-Heer-Hemelvaart",
        Rule::easter(ASCENSION),
    ),
    HolidayRule::public("Pentecost", "Pinksteren", Rule::easter(PENTECOST)),
    HolidayRule::public("Whit Monday", "Pinkstermaandag", Rule::easter(WHIT_MONDAY)),
    HolidayRule::public("National Day", "Nationale feestdag", Rule::gregorian(7, 21)),
    HolidayRule::public(
        "Assumption",
        "Onze-Lieve-Vrouw-Hemelvaart",
        Rule::gregorian(8, 15),
    ),
    HolidayRule::public("All Saints' Day", "Allerheiligen", Rule::gregorian(11, 1)),
    HolidayRule::public("Armistice Day", "Wapenstilstand", Rule::gregorian(11, 11)),
    HolidayRule::public("Christmas Day", "Kerstmis", Rule::gregorian(12, 25)),
];

/// Belgium.
pub static BELGIUM: RuleSet = RuleSet {
    code: "BE",
    english_name: "Belgium",
    rules: BE_RULES,
    // Belgian law gives a replacement day when a holiday falls on a Sunday,
    // but the employer fixes which day, so there is no calendar rule to
    // encode and this crate declines to invent one.
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Koninklijk besluit van 18 april 1974 / arrêté royal du 18 \
              avril 1974, article 1",
};

static CH_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Neujahrstag", Rule::gregorian(1, 1)),
    HolidayRule::public("Good Friday", "Karfreitag", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public("Easter Monday", "Ostermontag", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("Ascension", "Auffahrt", Rule::easter(ASCENSION)),
    HolidayRule::public("Whit Monday", "Pfingstmontag", Rule::easter(WHIT_MONDAY)),
    HolidayRule::public("Swiss National Day", "Bundesfeier", Rule::gregorian(8, 1))
        .years(Some(1994), None),
    HolidayRule::public("Christmas Day", "Weihnachtstag", Rule::gregorian(12, 25)),
    HolidayRule::public("St Stephen's Day", "Stephanstag", Rule::gregorian(12, 26)),
];

/// Switzerland.
pub static SWITZERLAND: RuleSet = RuleSet {
    code: "CH",
    english_name: "Switzerland",
    rules: CH_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Bundesverfassung Art. 110 Abs. 3 makes 1 August the only \
              federal holiday; the rest of this list is cantonal law and is \
              kept in all or nearly all 26 cantons. The cantons' own days — \
              Berchtoldstag, Fronleichnam, Jeûne genevois and the rest — are \
              not modelled",
};

static AT_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Neujahr", Rule::gregorian(1, 1)),
    HolidayRule::public("Epiphany", "Heilige Drei Könige", Rule::gregorian(1, 6)),
    HolidayRule::public("Easter Monday", "Ostermontag", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("Labour Day", "Staatsfeiertag", Rule::gregorian(5, 1)),
    HolidayRule::public("Ascension", "Christi Himmelfahrt", Rule::easter(ASCENSION)),
    HolidayRule::public("Whit Monday", "Pfingstmontag", Rule::easter(WHIT_MONDAY)),
    HolidayRule::public(
        "Corpus Christi",
        "Fronleichnam",
        Rule::easter(CORPUS_CHRISTI),
    ),
    HolidayRule::public("Assumption", "Mariä Himmelfahrt", Rule::gregorian(8, 15)),
    HolidayRule::public("National Day", "Nationalfeiertag", Rule::gregorian(10, 26))
        .years(Some(1965), None),
    HolidayRule::public("All Saints' Day", "Allerheiligen", Rule::gregorian(11, 1)),
    HolidayRule::public(
        "Immaculate Conception",
        "Mariä Empfängnis",
        Rule::gregorian(12, 8),
    ),
    HolidayRule::public("Christmas Day", "Christtag", Rule::gregorian(12, 25)),
    HolidayRule::public("St Stephen's Day", "Stefanitag", Rule::gregorian(12, 26)),
];

/// Austria.
pub static AUSTRIA: RuleSet = RuleSet {
    code: "AT",
    english_name: "Austria",
    rules: AT_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Arbeitsruhegesetz § 7 Abs. 2; Bundesgesetz BGBl. 263/1967 for \
              the National Day",
};

// ─────────────────────────────────────────────────────────────────────────
// The Nordic countries
// ─────────────────────────────────────────────────────────────────────────

static SE_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Nyårsdagen", Rule::gregorian(1, 1)),
    HolidayRule::public("Epiphany", "Trettondedag jul", Rule::gregorian(1, 6)),
    HolidayRule::public("Good Friday", "Långfredagen", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public("Easter Sunday", "Påskdagen", Rule::easter(EASTER_SUNDAY)),
    HolidayRule::public(
        "Easter Monday",
        "Annandag påsk",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::public("May Day", "Första maj", Rule::gregorian(5, 1)),
    HolidayRule::public(
        "Ascension",
        "Kristi himmelsfärdsdag",
        Rule::easter(ASCENSION),
    ),
    HolidayRule::public("Pentecost", "Pingstdagen", Rule::easter(PENTECOST)),
    // Whit Monday was traded for the National Day in 2005.
    HolidayRule::public("Whit Monday", "Annandag pingst", Rule::easter(WHIT_MONDAY))
        .years(None, Some(2004)),
    HolidayRule::public(
        "National Day",
        "Sveriges nationaldag",
        Rule::gregorian(6, 6),
    )
    .years(Some(2005), None),
    HolidayRule::public(
        "Midsummer Eve",
        "Midsommarafton",
        Rule::WeekdayOnOrAfter {
            month: 6,
            day: 19,
            weekday: Weekday::Friday,
        },
    )
    .of_kind(Kind::Bank),
    HolidayRule::public(
        "Midsummer Day",
        "Midsommardagen",
        Rule::WeekdayOnOrAfter {
            month: 6,
            day: 20,
            weekday: Weekday::Saturday,
        },
    ),
    HolidayRule::public(
        "All Saints' Day",
        "Alla helgons dag",
        Rule::WeekdayOnOrAfter {
            month: 10,
            day: 31,
            weekday: Weekday::Saturday,
        },
    ),
    HolidayRule::public("Christmas Eve", "Julafton", Rule::gregorian(12, 24)).of_kind(Kind::Bank),
    HolidayRule::public("Christmas Day", "Juldagen", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "Annandag jul", Rule::gregorian(12, 26)),
    HolidayRule::public("New Year's Eve", "Nyårsafton", Rule::gregorian(12, 31))
        .of_kind(Kind::Bank),
];

/// Sweden.
pub static SWEDEN: RuleSet = RuleSet {
    code: "SE",
    english_name: "Sweden",
    rules: SE_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Lag (1989:253) om allmänna helgdagar; lag 2004:1042 traded \
              Annandag pingst for the National Day. Midsummer Eve, Christmas \
              Eve and New Year's Eve are not allmänna helgdagar but are \
              de facto closed days, so they are recorded as bank holidays",
};

static NO_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Første nyttårsdag", Rule::gregorian(1, 1)),
    HolidayRule::public(
        "Maundy Thursday",
        "Skjærtorsdag",
        Rule::easter(MAUNDY_THURSDAY),
    ),
    HolidayRule::public("Good Friday", "Langfredag", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public(
        "Easter Sunday",
        "Første påskedag",
        Rule::easter(EASTER_SUNDAY),
    ),
    HolidayRule::public(
        "Easter Monday",
        "Andre påskedag",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::public("Labour Day", "Arbeidernes dag", Rule::gregorian(5, 1)),
    HolidayRule::public("Constitution Day", "Grunnlovsdag", Rule::gregorian(5, 17)),
    HolidayRule::public(
        "Ascension",
        "Kristi himmelfartsdag",
        Rule::easter(ASCENSION),
    ),
    HolidayRule::public("Pentecost", "Første pinsedag", Rule::easter(PENTECOST)),
    HolidayRule::public("Whit Monday", "Andre pinsedag", Rule::easter(WHIT_MONDAY)),
    HolidayRule::public("Christmas Day", "Første juledag", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "Andre juledag", Rule::gregorian(12, 26)),
];

/// Norway.
pub static NORWAY: RuleSet = RuleSet {
    code: "NO",
    english_name: "Norway",
    rules: NO_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Lov om helligdager og helligdagsfred (1995-02-24 nr. 12) § 2; \
              lov om 1. og 17. mai som høgtidsdager (1947)",
};

static DK_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Nytårsdag", Rule::gregorian(1, 1)),
    HolidayRule::public(
        "Maundy Thursday",
        "Skærtorsdag",
        Rule::easter(MAUNDY_THURSDAY),
    ),
    HolidayRule::public("Good Friday", "Langfredag", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public("Easter Sunday", "Påskedag", Rule::easter(EASTER_SUNDAY)),
    HolidayRule::public(
        "Easter Monday",
        "Anden påskedag",
        Rule::easter(EASTER_MONDAY),
    ),
    // Store bededag, the fourth Friday after Easter, was abolished with
    // effect from 2024 by lov nr. 214 af 28. februar 2023 — the first Danish
    // holiday abolished in three centuries.
    HolidayRule::public("Great Prayer Day", "Store bededag", Rule::easter(26))
        .years(None, Some(2023)),
    HolidayRule::public(
        "Ascension",
        "Kristi himmelfartsdag",
        Rule::easter(ASCENSION),
    ),
    HolidayRule::public("Pentecost", "Pinsedag", Rule::easter(PENTECOST)),
    HolidayRule::public("Whit Monday", "Anden pinsedag", Rule::easter(WHIT_MONDAY)),
    HolidayRule::observance("Constitution Day", "Grundlovsdag", Rule::gregorian(6, 5)),
    HolidayRule::public("Christmas Eve", "Juleaften", Rule::gregorian(12, 24)).of_kind(Kind::Bank),
    HolidayRule::public("Christmas Day", "Juledag", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "Anden juledag", Rule::gregorian(12, 26)),
];

/// Denmark.
pub static DENMARK: RuleSet = RuleSet {
    code: "DK",
    english_name: "Denmark",
    rules: DK_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Lov om helligdage; lov nr. 214 af 28. februar 2023 abolishing \
              Store bededag from 2024. Grundlovsdag is not a public holiday \
              and is recorded as an observance",
};

static FI_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Uudenvuodenpäivä", Rule::gregorian(1, 1)),
    HolidayRule::public("Epiphany", "Loppiainen", Rule::gregorian(1, 6)),
    HolidayRule::public("Good Friday", "Pitkäperjantai", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public(
        "Easter Sunday",
        "Pääsiäispäivä",
        Rule::easter(EASTER_SUNDAY),
    ),
    HolidayRule::public(
        "Easter Monday",
        "Toinen pääsiäispäivä",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::public("May Day", "Vappu", Rule::gregorian(5, 1)),
    HolidayRule::public("Ascension", "Helatorstai", Rule::easter(ASCENSION)),
    HolidayRule::public("Pentecost", "Helluntaipäivä", Rule::easter(PENTECOST)),
    HolidayRule::public(
        "Midsummer Eve",
        "Juhannusaatto",
        Rule::WeekdayOnOrAfter {
            month: 6,
            day: 19,
            weekday: Weekday::Friday,
        },
    )
    .of_kind(Kind::Bank),
    HolidayRule::public(
        "Midsummer Day",
        "Juhannuspäivä",
        Rule::WeekdayOnOrAfter {
            month: 6,
            day: 20,
            weekday: Weekday::Saturday,
        },
    ),
    HolidayRule::public(
        "All Saints' Day",
        "Pyhäinpäivä",
        Rule::WeekdayOnOrAfter {
            month: 10,
            day: 31,
            weekday: Weekday::Saturday,
        },
    ),
    HolidayRule::public(
        "Independence Day",
        "Itsenäisyyspäivä",
        Rule::gregorian(12, 6),
    )
    .years(Some(1917), None),
    HolidayRule::public("Christmas Eve", "Jouluaatto", Rule::gregorian(12, 24)).of_kind(Kind::Bank),
    HolidayRule::public("Christmas Day", "Joulupäivä", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "Tapaninpäivä", Rule::gregorian(12, 26)),
];

/// Finland.
pub static FINLAND: RuleSet = RuleSet {
    code: "FI",
    english_name: "Finland",
    rules: FI_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Laki juhlapäivistä (1937/205) and laki itsenäisyyspäivän \
              viettämisestä (1937/388). Midsummer Eve and Christmas Eve are \
              not statutory but are universally closed",
};

// ─────────────────────────────────────────────────────────────────────────
// Central and Eastern Europe
// ─────────────────────────────────────────────────────────────────────────

static PL_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Nowy Rok", Rule::gregorian(1, 1)),
    HolidayRule::public("Epiphany", "Trzech Króli", Rule::gregorian(1, 6)).years(Some(2011), None),
    HolidayRule::public("Easter Sunday", "Wielkanoc", Rule::easter(EASTER_SUNDAY)),
    HolidayRule::public(
        "Easter Monday",
        "Poniedziałek Wielkanocny",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::public("Labour Day", "Święto Pracy", Rule::gregorian(5, 1)),
    HolidayRule::public(
        "Constitution Day",
        "Święto Konstytucji 3 Maja",
        Rule::gregorian(5, 3),
    )
    .years(Some(1990), None),
    HolidayRule::public("Pentecost", "Zielone Świątki", Rule::easter(PENTECOST)),
    HolidayRule::public("Corpus Christi", "Boże Ciało", Rule::easter(CORPUS_CHRISTI)),
    HolidayRule::public("Assumption", "Wniebowzięcie NMP", Rule::gregorian(8, 15)),
    HolidayRule::public(
        "All Saints' Day",
        "Wszystkich Świętych",
        Rule::gregorian(11, 1),
    ),
    HolidayRule::public(
        "Independence Day",
        "Święto Niepodległości",
        Rule::gregorian(11, 11),
    )
    .years(Some(1989), None),
    // The centenary of independence got a single extra day.
    HolidayRule::fixed_public("Centenary of Independence", "", Rule::gregorian(11, 12))
        .years(Some(2018), Some(2018)),
    HolidayRule::public(
        "Christmas Eve",
        "Wigilia Bożego Narodzenia",
        Rule::gregorian(12, 24),
    )
    .years(Some(2025), None),
    HolidayRule::public("Christmas Day", "Boże Narodzenie", Rule::gregorian(12, 25)),
    HolidayRule::public(
        "Boxing Day",
        "Drugi dzień Bożego Narodzenia",
        Rule::gregorian(12, 26),
    ),
];

/// Poland.
pub static POLAND: RuleSet = RuleSet {
    code: "PL",
    english_name: "Poland",
    rules: PL_RULES,
    // A Polish holiday falling on a Saturday earns an extra day off, but the
    // employer fixes the date within the settlement period, so there is no
    // calendar rule.
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Ustawa z dnia 18 stycznia 1951 r. o dniach wolnych od pracy, \
              as amended — including the 2010 amendment adding Epiphany and \
              the 2024 amendment adding Christmas Eve from 2025",
};

static CZ_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Nový rok", Rule::gregorian(1, 1)),
    HolidayRule::public("Good Friday", "Velký pátek", Rule::easter(GOOD_FRIDAY))
        .years(Some(2016), None),
    HolidayRule::public(
        "Easter Monday",
        "Velikonoční pondělí",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::public("Labour Day", "Svátek práce", Rule::gregorian(5, 1)),
    HolidayRule::public("Victory Day", "Den vítězství", Rule::gregorian(5, 8)),
    HolidayRule::public(
        "Sts Cyril and Methodius",
        "Den slovanských věrozvěstů",
        Rule::gregorian(7, 5),
    ),
    HolidayRule::public(
        "Jan Hus Day",
        "Den upálení mistra Jana Husa",
        Rule::gregorian(7, 6),
    ),
    HolidayRule::public(
        "Statehood Day",
        "Den české státnosti",
        Rule::gregorian(9, 28),
    ),
    HolidayRule::public(
        "Independence Day",
        "Den vzniku samostatného státu",
        Rule::gregorian(10, 28),
    ),
    HolidayRule::public(
        "Freedom and Democracy Day",
        "Den boje za svobodu a demokracii",
        Rule::gregorian(11, 17),
    ),
    HolidayRule::public("Christmas Eve", "Štědrý den", Rule::gregorian(12, 24)),
    HolidayRule::public(
        "Christmas Day",
        "1. svátek vánoční",
        Rule::gregorian(12, 25),
    ),
    HolidayRule::public(
        "St Stephen's Day",
        "2. svátek vánoční",
        Rule::gregorian(12, 26),
    ),
];

/// Czechia.
pub static CZECHIA: RuleSet = RuleSet {
    code: "CZ",
    english_name: "Czechia",
    rules: CZ_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Zákon č. 245/2000 Sb. o státních svátcích, as amended by zákon \
              č. 359/2015 Sb. adding Good Friday",
};

static GR_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Πρωτοχρονιά", Rule::gregorian(1, 1)),
    HolidayRule::public("Epiphany", "Θεοφάνεια", Rule::gregorian(1, 6)),
    // Greece keeps the fixed feasts on the civil calendar but computes
    // Easter by the Julian computus, so Clean Monday is Orthodox Easter −48.
    HolidayRule::public(
        "Clean Monday",
        "Καθαρά Δευτέρα",
        Rule::paschal(ASH_WEDNESDAY - 2),
    ),
    HolidayRule::public(
        "Independence Day",
        "Εικοστή Πέμπτη Μαρτίου",
        Rule::gregorian(3, 25),
    ),
    HolidayRule::public(
        "Good Friday",
        "Μεγάλη Παρασκευή",
        Rule::paschal(GOOD_FRIDAY),
    ),
    HolidayRule::public(
        "Easter Sunday",
        "Κυριακή του Πάσχα",
        Rule::paschal(EASTER_SUNDAY),
    ),
    HolidayRule::public(
        "Easter Monday",
        "Δευτέρα του Πάσχα",
        Rule::paschal(EASTER_MONDAY),
    ),
    HolidayRule::public("Labour Day", "Εργατική Πρωτομαγιά", Rule::gregorian(5, 1)),
    HolidayRule::public("Whit Monday", "Αγίου Πνεύματος", Rule::paschal(WHIT_MONDAY)),
    HolidayRule::public(
        "Dormition of the Theotokos",
        "Κοίμηση της Θεοτόκου",
        Rule::gregorian(8, 15),
    ),
    HolidayRule::public("Ochi Day", "Επέτειος του Όχι", Rule::gregorian(10, 28)),
    HolidayRule::public("Christmas Day", "Χριστούγεννα", Rule::gregorian(12, 25)),
    HolidayRule::public(
        "Synaxis of the Theotokos",
        "Σύναξις Θεοτόκου",
        Rule::gregorian(12, 26),
    ),
];

/// Greece.
pub static GREECE: RuleSet = RuleSet {
    code: "GR",
    english_name: "Greece",
    rules: GR_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Νόμος 4808/2021 art. 60 and the ΥΑ setting the yearly list; \
              Easter follows the Julian computus, the fixed feasts the civil \
              calendar",
};

// ─────────────────────────────────────────────────────────────────────────
// Hungary
// ─────────────────────────────────────────────────────────────────────────

static HU_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Újév", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "1848 Revolution Memorial Day",
        "Az 1848-as forradalom ünnepe",
        Rule::gregorian(3, 15),
    ),
    HolidayRule::fixed_public("Good Friday", "Nagypéntek", Rule::easter(GOOD_FRIDAY))
        .years(Some(2017), None),
    HolidayRule::fixed_public("Easter Monday", "Húsvéthétfő", Rule::easter(EASTER_MONDAY)),
    HolidayRule::fixed_public("Labour Day", "A munka ünnepe", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Whit Monday", "Pünkösdhétfő", Rule::easter(WHIT_MONDAY)),
    HolidayRule::fixed_public(
        "Saint Stephen's Day",
        "Az államalapítás ünnepe",
        Rule::gregorian(8, 20),
    ),
    HolidayRule::fixed_public(
        "1956 Revolution Memorial Day",
        "Az 1956-os forradalom ünnepe",
        Rule::gregorian(10, 23),
    ),
    HolidayRule::fixed_public("All Saints' Day", "Mindenszentek", Rule::gregorian(11, 1)),
    HolidayRule::fixed_public("Christmas Day", "Karácsony", Rule::gregorian(12, 25)),
    HolidayRule::fixed_public(
        "Second Day of Christmas",
        "Karácsony másnapja",
        Rule::gregorian(12, 26),
    ),
];

/// Hungary.
///
/// No substitution: a holiday on a weekend stays there. What Hungary does
/// instead is rearrange working days by decree each year — a Thursday
/// holiday's Friday becomes a day off and a Saturday a working day — and
/// those *áthelyezett munkanapok* are set annually and are not carried.
pub static HUNGARY: RuleSet = RuleSet {
    code: "HU",
    english_name: "Hungary",
    rules: HU_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "2012. évi I. törvény a munka törvénykönyvéről, § 102, as amended \
              in 2016 to add Good Friday from 2017; Wikipedia, \"Public \
              holidays in Hungary\", retrieved 2026-09-22. The annual \
              rearrangement of working days around holidays is by decree and \
              is not carried",
};

// ─────────────────────────────────────────────────────────────────────────
// Romania
// ─────────────────────────────────────────────────────────────────────────

static RO_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Anul Nou", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Day after New Year's Day",
        "Anul Nou",
        Rule::gregorian(1, 2),
    ),
    HolidayRule::fixed_public("Epiphany", "Boboteaza", Rule::gregorian(1, 6))
        .years(Some(2024), None),
    HolidayRule::fixed_public(
        "Saint John the Baptist",
        "Sfântul Ioan Botezătorul",
        Rule::gregorian(1, 7),
    )
    .years(Some(2024), None),
    HolidayRule::fixed_public(
        "Union of the Romanian Principalities",
        "Ziua Unirii Principatelor Române",
        Rule::gregorian(1, 24),
    )
    .years(Some(2017), None),
    // The Romanian Orthodox Church keeps the fixed feasts on the civil
    // calendar and Easter by the Julian computus.
    HolidayRule::fixed_public("Good Friday", "Vinerea Mare", Rule::paschal(GOOD_FRIDAY))
        .years(Some(2018), None),
    HolidayRule::fixed_public("Easter Sunday", "Paștele", Rule::paschal(EASTER_SUNDAY)),
    HolidayRule::fixed_public(
        "Easter Monday",
        "A doua zi de Paște",
        Rule::paschal(EASTER_MONDAY),
    ),
    HolidayRule::fixed_public("Labour Day", "Ziua Muncii", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Children's Day", "Ziua Copilului", Rule::gregorian(6, 1))
        .years(Some(2017), None),
    HolidayRule::fixed_public("Pentecost", "Rusaliile", Rule::paschal(PENTECOST)),
    HolidayRule::fixed_public(
        "Whit Monday",
        "A doua zi de Rusalii",
        Rule::paschal(WHIT_MONDAY),
    ),
    HolidayRule::fixed_public(
        "Dormition of the Mother of God",
        "Adormirea Maicii Domnului",
        Rule::gregorian(8, 15),
    ),
    HolidayRule::fixed_public(
        "Saint Andrew's Day",
        "Sfântul Andrei",
        Rule::gregorian(11, 30),
    )
    .years(Some(2012), None),
    HolidayRule::fixed_public(
        "National Day",
        "Ziua Națională a României",
        Rule::gregorian(12, 1),
    ),
    HolidayRule::fixed_public("Christmas Day", "Crăciunul", Rule::gregorian(12, 25)),
    HolidayRule::fixed_public(
        "Second Day of Christmas",
        "A doua zi de Crăciun",
        Rule::gregorian(12, 26),
    ),
];

/// Romania.
///
/// The Orthodox movable feasts by the Julian computus, the fixed ones on
/// the civil calendar, and the additions of the last decade with their
/// years: Saint Andrew from 2012, 24 January and Children's Day from 2017,
/// Good Friday from 2018, Epiphany and Saint John from 2024. No
/// substitution.
pub static ROMANIA: RuleSet = RuleSet {
    code: "RO",
    english_name: "Romania",
    rules: RO_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Codul muncii (Legea 53/2003) art. 139, as amended by Legea \
              147/2012 (Saint Andrew), Legea 171/2016 (24 January), Legea \
              220/2016 (Children's Day), Legea 88/2018 (Good Friday) and the \
              2023 amendment adding Epiphany and Saint John from 2024; \
              Wikipedia, \"Public holidays in Romania\", retrieved 2026-09-22",
};

// ─────────────────────────────────────────────────────────────────────────
// Russia
// ─────────────────────────────────────────────────────────────────────────

/// The first year the Government's transfer decrees carried here cover.
const RU_TRANSFERS_FIRST: i64 = 2013;
/// The last.
const RU_TRANSFERS_LAST: i64 = 2027;

/// Each year's decree «О переносе выходных дней»: the days off it moves,
/// as the year, the month and day moved from, and the month and day moved
/// to.
#[rustfmt::skip]
static RU_TRANSFERS: &[(i64, u8, u8, u8, u8)] = &[
    // Постановление Правительства РФ № 1048 of 15 October 2012.
    (2013, 1, 5, 5, 2), (2013, 1, 6, 5, 3), (2013, 2, 25, 5, 10),
    // Постановление Правительства РФ № 444 of 28 May 2013.
    (2014, 1, 4, 5, 2), (2014, 1, 5, 6, 13), (2014, 2, 24, 11, 3),
    // Постановление Правительства РФ № 860 of 27 August 2014.
    (2015, 1, 3, 1, 9), (2015, 1, 4, 5, 4),
    // Постановление Правительства РФ № 1017 of 24 September 2015.
    (2016, 1, 2, 5, 3), (2016, 1, 3, 3, 7), (2016, 2, 20, 2, 22),
    // Постановление Правительства РФ № 756 of 4 August 2016.
    (2017, 1, 1, 2, 24), (2017, 1, 7, 5, 8),
    // Постановление Правительства РФ № 1250 of 14 October 2017.
    (2018, 1, 6, 3, 9), (2018, 1, 7, 5, 2), (2018, 4, 28, 4, 30), (2018, 6, 9, 6, 11), (2018, 12, 29, 12, 31),
    // Постановление Правительства РФ № 1163 of 1 October 2018.
    (2019, 1, 5, 5, 2), (2019, 1, 6, 5, 3), (2019, 2, 23, 5, 10),
    // Постановление Правительства РФ № 875 of 10 July 2019.
    (2020, 1, 4, 5, 4), (2020, 1, 5, 5, 5),
    // Постановление Правительства РФ № 1648 of 10 October 2020.
    (2021, 1, 2, 11, 5), (2021, 1, 3, 12, 31), (2021, 2, 20, 2, 22),
    // Постановление Правительства РФ № 1564 of 16 September 2021.
    (2022, 1, 1, 5, 3), (2022, 1, 2, 5, 10), (2022, 3, 5, 3, 7),
    // Постановление Правительства РФ № 1505 of 29 August 2022.
    (2023, 1, 1, 2, 24), (2023, 1, 8, 5, 8),
    // Постановление Правительства РФ № 1314 of 10 August 2023.
    (2024, 1, 6, 5, 10), (2024, 1, 7, 12, 31), (2024, 4, 27, 4, 29), (2024, 11, 2, 4, 30), (2024, 12, 28, 12, 30),
    // Постановление Правительства РФ № 1335 of 4 October 2024.
    (2025, 1, 4, 5, 2), (2025, 1, 5, 12, 31), (2025, 2, 23, 5, 8), (2025, 3, 8, 6, 13), (2025, 11, 1, 11, 3),
    // Постановление Правительства РФ № 1466 of 24 September 2025.
    (2026, 1, 3, 1, 9), (2026, 1, 4, 12, 31),
    // Постановление Правительства РФ № 1187 of 17 September 2026.
    (2027, 1, 2, 11, 5), (2027, 1, 3, 12, 31), (2027, 2, 20, 2, 22),
];

/// The non-working holidays outside January. Article 112 moves the day off
/// of one that falls on a weekend to the next working day; the January
/// days it leaves to the Government, which transfers two of them.
const RU_OUTSIDE_JANUARY: [(u8, u8); 6] = [(2, 23), (3, 8), (5, 1), (5, 9), (6, 12), (11, 4)];

fn ru_day(year: i64, month: u8, day: u8) -> Option<Rd> {
    gregorian::to_fixed(year, month, day).ok()
}

fn is_ru_weekend(day: Rd) -> bool {
    matches!(Weekday::from_rd(day), Weekday::Saturday | Weekday::Sunday)
}

/// The decree's transfers for `year`, as (from, to).
fn ru_transfers(year: i64) -> impl Iterator<Item = (Rd, Rd)> {
    RU_TRANSFERS
        .iter()
        .filter(move |row| row.0 == year)
        .filter_map(|&(y, from_month, from_day, to_month, to_day)| {
            Some((
                ru_day(y, from_month, from_day)?,
                ru_day(y, to_month, to_day)?,
            ))
        })
}

/// Whether `day` is one of the non-working holidays of article 112.
fn is_ru_holiday(year: i64, day: Rd) -> bool {
    (1..=8).any(|january| ru_day(year, 1, january) == Some(day))
        || RU_OUTSIDE_JANUARY
            .iter()
            .any(|&(month, date)| ru_day(year, month, date) == Some(day))
}

/// The days the decree makes days off.
fn ru_transferred(year: i64) -> Days {
    let mut out = Days::new();
    for (_, to) in ru_transfers(year) {
        out.push(to);
    }
    out
}

/// The weekend days the decree makes working days: those it moves the day
/// off from that are not holidays themselves. A January holiday on a
/// weekend stays a holiday; only its weekend is moved.
fn ru_worked(year: i64) -> Days {
    let mut out = Days::new();
    for (from, _) in ru_transfers(year) {
        if is_ru_weekend(from) && !is_ru_holiday(year, from) {
            out.push(from);
        }
    }
    out
}

/// The days off article 112 carries over from a holiday on the weekend,
/// to the next working day — except where the decree moves that weekend
/// day itself, or the day it would have been carried to, somewhere else.
/// In 2025 the decree moved Sunday 23 February to 8 May; in 2014 it moved
/// Monday 24 February, the day Sunday 23 February gave, to 3 November.
fn ru_carried_over(year: i64) -> Days {
    let mut sources = Days::new();
    for (from, _) in ru_transfers(year) {
        sources.push(from);
    }
    let sources = sources.as_slice();
    let worked = ru_worked(year);
    let transferred = ru_transferred(year);
    let mut out = Days::new();
    for &(month, date) in &RU_OUTSIDE_JANUARY {
        let Some(holiday) = ru_day(year, month, date) else {
            continue;
        };
        if !is_ru_weekend(holiday) || sources.contains(&holiday) {
            continue;
        }
        let mut day = Rd(holiday.0 + 1);
        while (is_ru_weekend(day) && !worked.as_slice().contains(&day))
            || is_ru_holiday(year, day)
            || transferred.as_slice().contains(&day)
            || out.as_slice().contains(&day)
        {
            day = Rd(day.0 + 1);
        }
        if !sources.contains(&day) {
            out.push(day);
        }
    }
    out
}

const fn ru_tabulated(function: fn(i64) -> Days) -> Rule {
    Rule::Tabulated {
        function,
        first_year: RU_TRANSFERS_FIRST,
        last_year: RU_TRANSFERS_LAST,
    }
}

static RU_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "New Year Holidays",
        "Новогодние каникулы",
        Rule::gregorian(1, 1),
    ),
    HolidayRule::fixed_public(
        "New Year Holidays",
        "Новогодние каникулы",
        Rule::gregorian(1, 2),
    )
    .years(Some(1993), None),
    HolidayRule::fixed_public(
        "New Year Holidays",
        "Новогодние каникулы",
        Rule::gregorian(1, 3),
    )
    .years(Some(2005), None),
    HolidayRule::fixed_public(
        "New Year Holidays",
        "Новогодние каникулы",
        Rule::gregorian(1, 4),
    )
    .years(Some(2005), None),
    HolidayRule::fixed_public(
        "New Year Holidays",
        "Новогодние каникулы",
        Rule::gregorian(1, 5),
    )
    .years(Some(2005), None),
    HolidayRule::fixed_public(
        "New Year Holidays",
        "Новогодние каникулы",
        Rule::gregorian(1, 6),
    )
    .years(Some(2013), None),
    HolidayRule::fixed_public(
        "Orthodox Christmas",
        "Рождество Христово",
        Rule::gregorian(1, 7),
    )
    .years(Some(1991), None),
    HolidayRule::fixed_public(
        "New Year Holidays",
        "Новогодние каникулы",
        Rule::gregorian(1, 8),
    )
    .years(Some(2013), None),
    HolidayRule::fixed_public(
        "Defender of the Fatherland Day",
        "День защитника Отечества",
        Rule::gregorian(2, 23),
    )
    .years(Some(2002), None),
    HolidayRule::fixed_public(
        "International Women's Day",
        "Международный женский день",
        Rule::gregorian(3, 8),
    ),
    HolidayRule::fixed_public(
        "Spring and Labour Day",
        "Праздник Весны и Труда",
        Rule::gregorian(5, 1),
    ),
    HolidayRule::fixed_public(
        "Spring and Labour Day",
        "Праздник Весны и Труда",
        Rule::gregorian(5, 2),
    )
    .years(None, Some(2004)),
    HolidayRule::fixed_public("Victory Day", "День Победы", Rule::gregorian(5, 9)),
    HolidayRule::fixed_public("Russia Day", "День России", Rule::gregorian(6, 12))
        .years(Some(1992), None),
    HolidayRule::fixed_public(
        "Day of Accord and Reconciliation",
        "День согласия и примирения",
        Rule::gregorian(11, 7),
    )
    .years(Some(1996), Some(2004)),
    HolidayRule::fixed_public(
        "Unity Day",
        "День народного единства",
        Rule::gregorian(11, 4),
    )
    .years(Some(2005), None),
    HolidayRule::fixed_public(
        "Day off carried over from a holiday on the weekend",
        "Перенесённый выходной день",
        ru_tabulated(ru_carried_over),
    ),
    HolidayRule::fixed_public(
        "Day off transferred by the Government",
        "Перенесённый выходной день",
        ru_tabulated(ru_transferred),
    ),
    HolidayRule::workday(
        "Working day, a day off transferred by the Government",
        "Рабочий день",
        ru_tabulated(ru_worked),
    ),
];

/// Russia.
///
/// The non-working holidays of article 112 of the Labour Code, with the
/// growth of the New Year holidays — 1 and 2 January, then 1 to 5 from
/// 2005, then 1 to 8 from 2013 — and the two replacements: Unity Day on
/// 4 November from 2005 in place of 7 November, and 2 May dropped after
/// 2004.
///
/// Article 112 carries the day off of a holiday outside January that falls
/// on a weekend over to the next working day, and lets the Government
/// transfer days off by decree, which it does every year: two of the
/// January weekend days go elsewhere, and a Saturday between a holiday and
/// a weekend is often swapped for the working day beside it. The decrees
/// for 2013 to 2027 are carried as data, and the carry-over is computed
/// from them, since a decree may move the weekend day or the day it would
/// have been carried to — in 2025, Sunday 23 February went to 8 May, not
/// to the Monday. Both were checked against ConsultantPlus's production
/// calendars for every one of those years. A year outside them is a gap.
///
/// The President's non-working days of 2020 and 2021, which kept pay but
/// were not days off under the Labour Code, are not carried: the
/// production calendar does not count them either.
pub static RUSSIA: RuleSet = RuleSet {
    code: "RU",
    english_name: "Russia",
    rules: RU_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Трудовой кодекс Российской Федерации, статья 112, as amended \
              (Federal Law 201-ФЗ of 2004 for the 2005 list, 35-ФЗ of 2012 \
              for 6 and 8 January); the Government's decrees «О переносе \
              выходных дней» for 2013 to 2027, as ConsultantPlus and Garant \
              publish them; ConsultantPlus's production calendars for those \
              years (consultant.ru/law/ref/calendar/proizvodstvennye/), \
              retrieved 2026-09-23; Wikipedia, \"Public holidays in Russia\", \
              retrieved 2026-09-22",
};

// ─────────────────────────────────────────────────────────────────────────
// Ukraine
// ─────────────────────────────────────────────────────────────────────────

static UA_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Новий рік", Rule::gregorian(1, 1)),
    HolidayRule::public(
        "Orthodox Christmas",
        "Різдво Христове",
        Rule::gregorian(1, 7),
    )
    .years(Some(1991), Some(2022)),
    HolidayRule::public(
        "International Women's Day",
        "Міжнародний жіночий день",
        Rule::gregorian(3, 8),
    ),
    // The Orthodox Church of Ukraine keeps the Julian computus for Easter
    // even after moving its fixed feasts to the Revised Julian calendar.
    HolidayRule::public("Easter", "Великдень", Rule::paschal(EASTER_SUNDAY)),
    HolidayRule::public("Labour Day", "День праці", Rule::gregorian(5, 1)),
    HolidayRule::public("Labour Day", "День праці", Rule::gregorian(5, 2)).years(None, Some(2017)),
    HolidayRule::public(
        "Day of Remembrance and Victory over Nazism",
        "День пам'яті та перемоги над нацизмом у Другій світовій війні",
        Rule::gregorian(5, 8),
    )
    .years(Some(2023), None),
    HolidayRule::public("Victory Day", "День перемоги", Rule::gregorian(5, 9))
        .years(None, Some(2022)),
    HolidayRule::public("Trinity", "Трійця", Rule::paschal(PENTECOST)),
    HolidayRule::public(
        "Constitution Day",
        "День Конституції",
        Rule::gregorian(6, 28),
    )
    .years(Some(1997), None),
    HolidayRule::public(
        "Statehood Day",
        "День Української Державності",
        Rule::gregorian(7, 28),
    )
    .years(Some(2022), Some(2023)),
    HolidayRule::public(
        "Statehood Day",
        "День Української Державності",
        Rule::gregorian(7, 15),
    )
    .years(Some(2024), None),
    HolidayRule::public(
        "Independence Day",
        "День Незалежності",
        Rule::gregorian(8, 24),
    )
    .years(Some(1992), None),
    HolidayRule::public(
        "Defenders of Ukraine Day",
        "День захисників і захисниць України",
        Rule::gregorian(10, 14),
    )
    .years(Some(2015), Some(2022)),
    HolidayRule::public(
        "Defenders of Ukraine Day",
        "День захисників і захисниць України",
        Rule::gregorian(10, 1),
    )
    .years(Some(2023), None),
    HolidayRule::public("Christmas", "Різдво Христове", Rule::gregorian(12, 25))
        .years(Some(2017), None),
];

/// Ukraine.
///
/// The holidays of article 73 of the Labour Code, with the changes of the
/// last decade by year: Defenders Day from 2015 on 14 October and from
/// 2023 on 1 October, Christmas on 25 December from 2017 beside 7 January
/// and alone from 2023, 2 May dropped after 2017, Statehood Day from 2022
/// on 28 July and from 2024 on 15 July, 8 May in place of 9 May from 2023.
/// Easter and Trinity follow the Julian computus.
///
/// Article 67 moves a holiday that falls on a weekend to the next working
/// day, and that is carried. **Under martial law, in force since
/// 24 February 2022, no holiday is a day off**; the table states the law
/// and not the suspension, which has no end date to state.
pub static UKRAINE: RuleSet = RuleSet {
    code: "UA",
    english_name: "Ukraine",
    rules: UA_RULES,
    // The next working day, as article 67 says: the same policy as the
    // British bank-holiday shift.
    substitution: BRITISH_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Кодекс законів про працю України, статті 67 and 73, as amended \
              by the laws of 2015 (Defenders Day), 2017 (25 December), 2021 \
              (Statehood Day) and 2023 (8 May, 15 July, 1 October, 7 January \
              removed); Wikipedia, \"Public holidays in Ukraine\", retrieved \
              2026-09-22. Under martial law since 2022 holidays are not days \
              off, which the table does not model",
};

// ─────────────────────────────────────────────────────────────────────────
// Croatia
// ─────────────────────────────────────────────────────────────────────────

static HR_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Nova godina", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("Epiphany", "Sveta tri kralja", Rule::gregorian(1, 6)),
    HolidayRule::fixed_public("Easter Sunday", "Uskrs", Rule::easter(EASTER_SUNDAY)),
    HolidayRule::fixed_public(
        "Easter Monday",
        "Uskrsni ponedjeljak",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::fixed_public("Labour Day", "Praznik rada", Rule::gregorian(5, 1)),
    // 30 May from 1991 to 2001 and again from 2020; 25 June in between.
    HolidayRule::fixed_public("Statehood Day", "Dan državnosti", Rule::gregorian(5, 30))
        .years(Some(1991), Some(2001)),
    HolidayRule::fixed_public("Statehood Day", "Dan državnosti", Rule::gregorian(5, 30))
        .years(Some(2020), None),
    HolidayRule::fixed_public("Statehood Day", "Dan državnosti", Rule::gregorian(6, 25))
        .years(Some(2002), Some(2019)),
    HolidayRule::fixed_public("Corpus Christi", "Tijelovo", Rule::easter(CORPUS_CHRISTI)),
    HolidayRule::fixed_public(
        "Anti-Fascist Struggle Day",
        "Dan antifašističke borbe",
        Rule::gregorian(6, 22),
    ),
    HolidayRule::fixed_public(
        "Victory and Homeland Thanksgiving Day",
        "Dan pobjede i domovinske zahvalnosti",
        Rule::gregorian(8, 5),
    ),
    HolidayRule::fixed_public("Assumption Day", "Velika Gospa", Rule::gregorian(8, 15)),
    // Independence Day, a holiday from 2002 to 2019, a memorial day since.
    HolidayRule::fixed_public(
        "Independence Day",
        "Dan neovisnosti",
        Rule::gregorian(10, 8),
    )
    .years(Some(2002), Some(2019)),
    HolidayRule::fixed_public("All Saints' Day", "Dan svih svetih", Rule::gregorian(11, 1)),
    HolidayRule::fixed_public(
        "Remembrance Day for the Victims of the Homeland War",
        "Dan sjećanja na žrtve Domovinskog rata",
        Rule::gregorian(11, 18),
    )
    .years(Some(2020), None),
    HolidayRule::fixed_public("Christmas Day", "Božić", Rule::gregorian(12, 25)),
    HolidayRule::fixed_public(
        "Saint Stephen's Day",
        "Sveti Stjepan",
        Rule::gregorian(12, 26),
    ),
];

/// Croatia.
///
/// The Holidays, Memorial Days and Non-Working Days Act, with the change
/// of 2020: Statehood Day back on 30 May, where it had been from 1991 to
/// 2001, and Remembrance Day on 18 November new, while 25 June and
/// 8 October, Statehood Day and Independence Day from 2002 to 2019, became
/// memorial days and working days. The source dates the 2020 change and
/// 30 May's earlier span; that 25 June and 8 October began in 2002 is what
/// the gap in 30 May's years implies, and is carried as such. The right of
/// those who keep other religious calendars not to work on their own
/// feasts is personal and not carried. No substitution.
pub static CROATIA: RuleSet = RuleSet {
    code: "HR",
    english_name: "Croatia",
    rules: HR_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Zakon o blagdanima, spomendanima i neradnim danima, as summarised by \
              Wikipedia, \"Public holidays in Croatia\", retrieved 2026-09-22, \
              with its note on the 2020 change",
};

// ─────────────────────────────────────────────────────────────────────────
// Slovakia
// ─────────────────────────────────────────────────────────────────────────

/// A state holiday that stopped being a day off: a holiday to `last`, an
/// observance after.
const fn sk_demoted(
    name: &'static str,
    local: &'static str,
    month: u8,
    day: u8,
    last: i32,
) -> [HolidayRule; 2] {
    [
        HolidayRule::fixed_public(name, local, Rule::gregorian(month, day)).years(None, Some(last)),
        HolidayRule::observance(name, local, Rule::gregorian(month, day))
            .years(Some(last + 1), None),
    ]
}

static SK_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "Day of the Establishment of the Slovak Republic",
        "Deň vzniku Slovenskej republiky",
        Rule::gregorian(1, 1),
    ),
    HolidayRule::fixed_public("Epiphany", "Zjavenie Pána", Rule::gregorian(1, 6)),
    HolidayRule::fixed_public(
        "Good Friday",
        "Veľkonočný piatok",
        Rule::easter(GOOD_FRIDAY),
    ),
    HolidayRule::fixed_public(
        "Easter Monday",
        "Veľkonočný pondelok",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::fixed_public("Labour Day", "Sviatok práce", Rule::gregorian(5, 1)),
    sk_demoted(
        "Day of Victory over Fascism",
        "Deň víťazstva nad fašizmom",
        5,
        8,
        2025,
    )[0],
    sk_demoted(
        "Day of Victory over Fascism",
        "Deň víťazstva nad fašizmom",
        5,
        8,
        2025,
    )[1],
    HolidayRule::fixed_public(
        "Saints Cyril and Methodius Day",
        "Sviatok svätého Cyrila a Metoda",
        Rule::gregorian(7, 5),
    ),
    HolidayRule::fixed_public(
        "Slovak National Uprising Anniversary",
        "Výročie Slovenského národného povstania",
        Rule::gregorian(8, 29),
    ),
    sk_demoted(
        "Constitution Day",
        "Deň Ústavy Slovenskej republiky",
        9,
        1,
        2023,
    )[0],
    sk_demoted(
        "Constitution Day",
        "Deň Ústavy Slovenskej republiky",
        9,
        1,
        2023,
    )[1],
    sk_demoted(
        "Our Lady of the Seven Sorrows",
        "Sviatok Panny Márie Sedembolestnej",
        9,
        15,
        2025,
    )[0],
    sk_demoted(
        "Our Lady of the Seven Sorrows",
        "Sviatok Panny Márie Sedembolestnej",
        9,
        15,
        2025,
    )[1],
    // A state holiday since 2021, and a working day.
    HolidayRule::observance(
        "Day of the Establishment of an Independent Czecho-Slovak State",
        "Deň vzniku samostatného česko-slovenského štátu",
        Rule::gregorian(10, 28),
    )
    .years(Some(2021), None),
    HolidayRule::fixed_public(
        "All Saints' Day",
        "Sviatok všetkých svätých",
        Rule::gregorian(11, 1),
    ),
    sk_demoted(
        "Struggle for Freedom and Democracy Day",
        "Deň boja za slobodu a demokraciu",
        11,
        17,
        2024,
    )[0],
    sk_demoted(
        "Struggle for Freedom and Democracy Day",
        "Deň boja za slobodu a demokraciu",
        11,
        17,
        2024,
    )[1],
    HolidayRule::fixed_public("Christmas Eve", "Štedrý deň", Rule::gregorian(12, 24)),
    HolidayRule::fixed_public(
        "Christmas Day",
        "Prvý sviatok vianočný",
        Rule::gregorian(12, 25),
    ),
    HolidayRule::fixed_public(
        "Second Day of Christmas",
        "Druhý sviatok vianočný",
        Rule::gregorian(12, 26),
    ),
];

/// Slovakia.
///
/// The state holidays and the days off, which have parted company: since
/// 2024 Constitution Day, since 2025 17 November, and since 2026 8 May and
/// 15 September are state holidays on which work goes on, and 28 October
/// has been one since 2021. Each is carried as a day off to its last year
/// as one and an observance after. No substitution.
pub static SLOVAKIA: RuleSet = RuleSet {
    code: "SK",
    english_name: "Slovakia",
    rules: SK_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Wikipedia, \"Public holidays in Slovakia\", retrieved 2026-09-22, for \
              the list, and for the years each state holiday became a working day",
};

// ─────────────────────────────────────────────────────────────────────────
// Slovenia
// ─────────────────────────────────────────────────────────────────────────

static SI_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "novo leto", Rule::gregorian(1, 1)),
    // Work-free to 2012, and again from 2017.
    HolidayRule::fixed_public("New Year's Day", "novo leto", Rule::gregorian(1, 2))
        .years(None, Some(2012)),
    HolidayRule::fixed_public("New Year's Day", "novo leto", Rule::gregorian(1, 2))
        .years(Some(2017), None),
    HolidayRule::fixed_public("Prešeren Day", "Prešernov dan", Rule::gregorian(2, 8))
        .years(Some(1991), None),
    HolidayRule::fixed_public(
        "Easter Sunday",
        "velikonočna nedelja",
        Rule::easter(EASTER_SUNDAY),
    ),
    HolidayRule::fixed_public(
        "Easter Monday",
        "velikonočni ponedeljek",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::fixed_public(
        "Day of Uprising Against Occupation",
        "dan upora proti okupatorju",
        Rule::gregorian(4, 27),
    ),
    HolidayRule::fixed_public("May Day", "praznik dela", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("May Day", "praznik dela", Rule::gregorian(5, 2)),
    HolidayRule::fixed_public("Whit Sunday", "binkoštna nedelja", Rule::easter(PENTECOST)),
    HolidayRule::fixed_public("Statehood Day", "dan državnosti", Rule::gregorian(6, 25)),
    HolidayRule::fixed_public(
        "Assumption Day",
        "Marijino vnebovzetje",
        Rule::gregorian(8, 15),
    )
    .years(Some(1992), None),
    HolidayRule::fixed_public(
        "Reformation Day",
        "dan reformacije",
        Rule::gregorian(10, 31),
    )
    .years(Some(1992), None),
    HolidayRule::fixed_public(
        "All Saints' Day",
        "dan spomina na mrtve",
        Rule::gregorian(11, 1),
    ),
    HolidayRule::fixed_public("Christmas Day", "božič", Rule::gregorian(12, 25))
        .years(Some(1991), None),
    HolidayRule::fixed_public(
        "Independence and Unity Day",
        "dan samostojnosti in enotnosti",
        Rule::gregorian(12, 26),
    ),
];

/// Slovenia.
///
/// The work-free days, state holidays and religious days alike, with the
/// years the source gives: 2 January work-free to 2012 and again from
/// 2017, Prešeren Day and Christmas from 1991, the Assumption and
/// Reformation Day from 1992. The five state holidays that are working
/// days are not carried, nor the pre-1991 names. No substitution.
pub static SLOVENIA: RuleSet = RuleSet {
    code: "SI",
    english_name: "Slovenia",
    rules: SI_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Wikipedia, \"Public holidays in Slovenia\", retrieved 2026-09-22, \
              summarising the Holidays and Days off in the Republic of Slovenia \
              Act, with the years each day became work-free and the 2012 and \
              2017 changes to 2 January",
};

// ─────────────────────────────────────────────────────────────────────────
// Iceland
// ─────────────────────────────────────────────────────────────────────────

static IS_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Nýársdagur", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Maundy Thursday",
        "Skírdagur",
        Rule::easter(MAUNDY_THURSDAY),
    ),
    HolidayRule::fixed_public(
        "Good Friday",
        "Föstudagurinn langi",
        Rule::easter(GOOD_FRIDAY),
    ),
    HolidayRule::fixed_public("Easter Sunday", "Páskadagur", Rule::easter(EASTER_SUNDAY)),
    HolidayRule::fixed_public(
        "Easter Monday",
        "Annar í páskum",
        Rule::easter(EASTER_MONDAY),
    ),
    // The first Thursday after 18 April, so 19 to 25 April.
    HolidayRule::fixed_public(
        "First Day of Summer",
        "Sumardagurinn fyrsti",
        Rule::WeekdayOnOrAfter {
            month: 4,
            day: 19,
            weekday: Weekday::Thursday,
        },
    ),
    HolidayRule::fixed_public("May Day", "Verkalýðsdagurinn", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public(
        "Ascension Day",
        "Uppstigningardagur",
        Rule::easter(ASCENSION),
    ),
    HolidayRule::fixed_public("Whit Sunday", "Hvítasunnudagur", Rule::easter(PENTECOST)),
    HolidayRule::fixed_public(
        "Whit Monday",
        "Annar í hvítasunnu",
        Rule::easter(WHIT_MONDAY),
    ),
    HolidayRule::fixed_public(
        "National Day",
        "Þjóðhátíðardagurinn",
        Rule::gregorian(6, 17),
    ),
    HolidayRule::fixed_public(
        "Commerce Day",
        "Frídagur verslunarmanna",
        Rule::NthWeekday {
            month: 8,
            n: 1,
            weekday: Weekday::Monday,
        },
    ),
    // Holidays from 13:00; half days, as in Sweden and Denmark.
    HolidayRule::fixed_public("Christmas Eve", "Aðfangadagur", Rule::gregorian(12, 24))
        .of_kind(Kind::Bank),
    HolidayRule::fixed_public("Christmas Day", "Jóladagur", Rule::gregorian(12, 25)),
    HolidayRule::fixed_public(
        "Second Day of Christmas",
        "Annar í jólum",
        Rule::gregorian(12, 26),
    ),
    HolidayRule::fixed_public("New Year's Eve", "Gamlársdagur", Rule::gregorian(12, 31))
        .of_kind(Kind::Bank),
];

/// Iceland.
///
/// The public holidays the parliament's act establishes: the Easter and
/// Whitsun cycle from Maundy Thursday, the First Day of Summer on the
/// first Thursday after 18 April, Commerce Day on the first Monday of
/// August, and Christmas Eve and New Year's Eve, holidays from 13:00 and
/// carried as `Kind::Bank` half days as Sweden's and Denmark's are. The
/// flag days are not carried. No substitution.
pub static ICELAND: RuleSet = RuleSet {
    code: "IS",
    english_name: "Iceland",
    rules: IS_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Wikipedia, \"Public holidays in Iceland\", retrieved 2026-09-22, for \
              the list and the half days, and \"First day of summer (Iceland)\", \
              retrieved the same day, for the Thursday rule",
};

// ─────────────────────────────────────────────────────────────────────────
// Bulgaria
// ─────────────────────────────────────────────────────────────────────────

/// Labour Code art. 154(2), in force from 1 January 2017: a holiday on a
/// Saturday or Sunday makes the first working day after it, or the first
/// two, days off. The Easter days are excepted by the article itself and
/// are `fixed_public` below.
static BG_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: Some(2017),
    valid_until: None,
}];

static BG_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Нова година", Rule::gregorian(1, 1)),
    HolidayRule::public(
        "Liberation Day",
        "Ден на Освобождението на България от османско владичество",
        Rule::gregorian(3, 3),
    ),
    HolidayRule::fixed_public("Good Friday", "Велики петък", Rule::paschal(GOOD_FRIDAY)),
    HolidayRule::fixed_public(
        "Holy Saturday",
        "Велика събота",
        Rule::paschal(HOLY_SATURDAY),
    ),
    HolidayRule::fixed_public("Easter Sunday", "Великден", Rule::paschal(EASTER_SUNDAY)),
    HolidayRule::fixed_public("Easter Monday", "Великден", Rule::paschal(EASTER_MONDAY)),
    HolidayRule::public(
        "Labour Day",
        "Ден на труда и на международната работническа солидарност",
        Rule::gregorian(5, 1),
    ),
    HolidayRule::public(
        "Saint George's Day, Day of Valour and of the Bulgarian Army",
        "Гергьовден, Ден на храбростта и празник на Българската армия",
        Rule::gregorian(5, 6),
    ),
    HolidayRule::public(
        "Day of the Saints Cyril and Methodius, of the Bulgarian Alphabet, Education and Culture and of Slavic Literature",
        "Ден на светите братя Кирил и Методий, на българската азбука, просвета и култура и на славянската книжовност",
        Rule::gregorian(5, 24),
    ),
    HolidayRule::public(
        "Unification Day",
        "Ден на Съединението",
        Rule::gregorian(9, 6),
    ),
    HolidayRule::public(
        "Independence Day",
        "Ден на Независимостта на България",
        Rule::gregorian(9, 22),
    ),
    // A day off for schools only.
    HolidayRule::observance(
        "Day of the National Awakeners",
        "Ден на народните будители",
        Rule::gregorian(11, 1),
    )
    .of_kind(Kind::School),
    HolidayRule::public("Christmas Eve", "Бъдни вечер", Rule::gregorian(12, 24)),
    HolidayRule::public(
        "Christmas Day",
        "Рождество Христово",
        Rule::gregorian(12, 25),
    ),
    HolidayRule::public(
        "Second Day of Christmas",
        "Рождество Христово",
        Rule::gregorian(12, 26),
    ),
];

/// Bulgaria.
///
/// Labour Code art. 154(1): the fixed days, the Orthodox Easter from Good
/// Friday to Easter Monday by the Julian computus, and 1 November, a day
/// off for schools alone and carried as [`Kind::School`]. Art. 154(2),
/// from 2017, moves a weekend holiday to the working day after, except the
/// Easter days, which it names. The one-off days off the Council of
/// Ministers may declare under art. 154(3), and the working Saturdays that
/// paid for long weekends before 2017, are not carried; nor are the years
/// the fixed days were introduced, which the sources do not give.
pub static BULGARIA: RuleSet = RuleSet {
    code: "BG",
    english_name: "Bulgaria",
    rules: BG_RULES,
    substitution: BG_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Council of Ministers of the Republic of Bulgaria, \"Bulgarian public \
              holidays\", gov.bg, retrieved 2026-09-22, for the list and the \
              weekend rule; the Bulgarian Wikipedia, \"Официални празници в \
              България\", retrieved the same day, for Labour Code art. 154(2) as \
              amended by SG 105/2016, in force 1 January 2017, and the names",
};

// ─────────────────────────────────────────────────────────────────────────
// Cyprus
// ─────────────────────────────────────────────────────────────────────────

static CY_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Πρωτοχρονιά", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("Epiphany", "Θεοφάνια", Rule::gregorian(1, 6)),
    HolidayRule::fixed_public(
        "Green Monday",
        "Καθαρά Δευτέρα",
        Rule::paschal(ASH_WEDNESDAY - 2),
    ),
    HolidayRule::fixed_public(
        "Greek Independence Day",
        "Ημέρα της Ελληνικής Ανεξαρτησίας",
        Rule::gregorian(3, 25),
    ),
    HolidayRule::fixed_public(
        "Cyprus National Day",
        "Εθνική Ημέρα Κύπρου",
        Rule::gregorian(4, 1),
    ),
    HolidayRule::fixed_public(
        "Good Friday",
        "Μεγάλη Παρασκευή",
        Rule::paschal(GOOD_FRIDAY),
    ),
    HolidayRule::fixed_public(
        "Holy Saturday",
        "Μεγάλο Σάββατο",
        Rule::paschal(HOLY_SATURDAY),
    ),
    HolidayRule::fixed_public(
        "Easter Sunday",
        "Κυριακή του Πάσχα",
        Rule::paschal(EASTER_SUNDAY),
    ),
    HolidayRule::fixed_public(
        "Easter Monday",
        "Δευτέρα του Πάσχα",
        Rule::paschal(EASTER_MONDAY),
    ),
    // Banks only.
    HolidayRule::fixed_public(
        "Easter Tuesday",
        "Τρίτη του Πάσχα",
        Rule::paschal(EASTER_MONDAY + 1),
    )
    .of_kind(Kind::Bank),
    HolidayRule::fixed_public("Labour Day", "Ημέρα Εργασίας", Rule::gregorian(5, 1)),
    // Kataklysmos, the Monday of the Holy Spirit.
    HolidayRule::fixed_public(
        "Pentecost Monday",
        "Πεντηκοστή Δευτέρα",
        Rule::paschal(WHIT_MONDAY),
    ),
    HolidayRule::fixed_public(
        "Dormition of the Theotokos",
        "Κοίμηση της Θεοτόκου",
        Rule::gregorian(8, 15),
    ),
    HolidayRule::fixed_public(
        "Cyprus Independence Day",
        "Ημέρα Ανεξαρτησίας της Κύπρου",
        Rule::gregorian(10, 1),
    ),
    HolidayRule::fixed_public(
        "Greek National Day",
        "Επέτειος του Όχι",
        Rule::gregorian(10, 28),
    ),
    HolidayRule::fixed_public(
        "Christmas Day",
        "Ημέρα των Χριστουγέννων",
        Rule::gregorian(12, 25),
    ),
    HolidayRule::fixed_public(
        "Boxing Day",
        "Δεύτερη μέρα των Χριστουγέννων",
        Rule::gregorian(12, 26),
    ),
];

/// Cyprus, the Republic.
///
/// The public holidays as the Greek Wikipedia lists them, with the
/// Orthodox Easter by the Julian computus from Green Monday to Pentecost
/// Monday, and Easter Tuesday, which the Central Bank's list of bank
/// holidays carries and the public list does not, as [`Kind::Bank`]. No
/// substitution: the bank's 2026 list notes that the Dormition and Boxing
/// Day fall on Saturdays and moves nothing. The north of the island is not
/// carried.
pub static CYPRUS: RuleSet = RuleSet {
    code: "CY",
    english_name: "Cyprus",
    rules: CY_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "The Greek Wikipedia, \"Δημόσιες αργίες στην Κύπρο\", retrieved \
              2026-09-22, for the list and the names; Central Bank of Cyprus, \
              \"Bank holidays to be observed in Cyprus during 2026\", dated \
              7 June 2024, for Easter Tuesday and the 2026 dates",
};

// ─────────────────────────────────────────────────────────────────────────
// Estonia
// ─────────────────────────────────────────────────────────────────────────

static EE_RULES: &[HolidayRule] = &[
    // § 1: the national holiday, a day off.
    HolidayRule::fixed_public("Independence Day", "iseseisvuspäev", Rule::gregorian(2, 24)),
    // § 2: the public holidays.
    HolidayRule::fixed_public("New Year's Day", "uusaasta", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("Good Friday", "suur reede", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public(
        "Easter Sunday",
        "ülestõusmispühade 1. püha",
        Rule::easter(EASTER_SUNDAY),
    ),
    HolidayRule::fixed_public("Spring Day", "kevadpüha", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Whit Sunday", "nelipühade 1. püha", Rule::easter(PENTECOST)),
    HolidayRule::fixed_public("Victory Day", "võidupüha", Rule::gregorian(6, 23)),
    HolidayRule::fixed_public("Midsummer Day", "jaanipäev", Rule::gregorian(6, 24)),
    // A day of national importance under the 1994 act, a public holiday
    // under the 1998 one.
    HolidayRule::observance(
        "Day of Restoration of Independence",
        "taasiseseisvumispäev",
        Rule::gregorian(8, 20),
    )
    .years(Some(1994), Some(1997)),
    HolidayRule::fixed_public(
        "Day of Restoration of Independence",
        "taasiseseisvumispäev",
        Rule::gregorian(8, 20),
    )
    .years(Some(1998), None),
    HolidayRule::fixed_public("Christmas Eve", "jõululaupäev", Rule::gregorian(12, 24))
        .years(Some(2005), None),
    HolidayRule::fixed_public(
        "Christmas Day",
        "esimene jõulupüha",
        Rule::gregorian(12, 25),
    ),
    HolidayRule::fixed_public("Boxing Day", "teine jõulupüha", Rule::gregorian(12, 26)),
    // § 3: the days of national importance, working days.
    HolidayRule::observance("Epiphany", "kolmekuningapäev", Rule::gregorian(1, 6)),
    HolidayRule::observance(
        "Anniversary of the Tartu Peace Treaty",
        "Tartu rahulepingu aastapäev",
        Rule::gregorian(2, 2),
    ),
    HolidayRule::observance("Mother Tongue Day", "emakeelepäev", Rule::gregorian(3, 14))
        .years(Some(1999), None),
    HolidayRule::observance(
        "Mother's Day",
        "emadepäev",
        Rule::nth(5, 2, Weekday::Sunday),
    ),
    HolidayRule::observance(
        "Estonian Flag Day",
        "Eesti lipu päev",
        Rule::gregorian(6, 4),
    )
    .years(Some(2004), None),
    HolidayRule::observance("Day of Mourning", "leinapäev", Rule::gregorian(6, 14)),
    HolidayRule::observance(
        "Day of Remembrance for the Victims of Communism and Nazism",
        "kommunismi ja natsismi ohvrite mälestuspäev",
        Rule::gregorian(8, 23),
    )
    .years(Some(2009), None),
    HolidayRule::observance(
        "Grandparents' Day",
        "vanavanemate päev",
        Rule::nth(9, 2, Weekday::Sunday),
    )
    .years(Some(2010), None),
    HolidayRule::observance(
        "Resistance Day",
        "vastupanuvõitluse päev",
        Rule::gregorian(9, 22),
    )
    .years(Some(2010), None),
    HolidayRule::observance(
        "Finno-Ugric Day",
        "hõimupäev",
        Rule::nth(10, 3, Weekday::Saturday),
    )
    .years(Some(2011), None),
    HolidayRule::observance("All Souls' Day", "hingedepäev", Rule::gregorian(11, 2)),
    HolidayRule::observance(
        "Father's Day",
        "isadepäev",
        Rule::nth(11, 2, Weekday::Sunday),
    ),
    HolidayRule::observance("Day of Rebirth", "taassünni päev", Rule::gregorian(11, 16)),
];

/// Estonia.
///
/// The Public Holidays and Days of National Importance Act of 27 January
/// 1998, in force from 23 February 1998: the national holiday of § 1 and
/// the public holidays of § 2, days off, and the days of national
/// importance of § 3, working days carried as observances. The years are
/// the act's own amendment marks: 20 August a public holiday from the 1998
/// act, having been a day of national importance under the act of 1994,
/// Christmas Eve from 2005, and the six § 3 days added between 1999 and
/// 2011. The days without marks were in the 1994 act and are not dated.
/// No substitution; the three-hour shortening of the working day before
/// some of these, which the Employment Contracts Act sets, is not carried.
pub static ESTONIA: RuleSet = RuleSet {
    code: "EE",
    english_name: "Estonia",
    rules: EE_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Pühade ja tähtpäevade seadus, adopted 27 January 1998, and the act of \
              8 February 1994 it replaced, as reproduced with their amendment \
              marks by the Estonian Wikipedia, \"Pühade ja tähtpäevade seadus\", \
              retrieved 2026-09-22; Wikipedia, \"Public holidays in Estonia\", \
              retrieved the same day, for the English names",
};

// ─────────────────────────────────────────────────────────────────────────
// Latvia
// ─────────────────────────────────────────────────────────────────────────

/// Section 1 of the law: 4 May, the closing day of the Song and Dance
/// Celebration and 18 November, falling on a Saturday or Sunday, make the
/// next working day a day off. Only those three are `public` below; the
/// rest are `fixed_public` and the policy never reaches them.
static LV_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// The closing day of the Nationwide Latvian Song and Dance Celebration,
/// held every five years on dates the organisers set: 8 July 2018 and
/// 9 July 2023. The table starts in 2018 because in 2013 the Monday after
/// the closing day was a day off for the participants alone, under the
/// Song and Dance Celebration Law rather than this one. The next
/// celebration is planned for 2028 and its date is not yet set.
fn lv_song_and_dance_celebration(year: i64) -> Days {
    let (month, day) = match year {
        2018 => (7u8, 8u8),
        2023 => (7, 9),
        _ => return Days::new(),
    };
    gregorian::to_fixed(year, month, day).map_or_else(|_| Days::new(), Days::one)
}

static LV_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Jaungada diena", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("Good Friday", "Lielā Piektdiena", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Sunday", "Pirmās Lieldienas", Rule::easter(EASTER_SUNDAY)),
    HolidayRule::fixed_public("Easter Monday", "Otrās Lieldienas", Rule::easter(EASTER_MONDAY)),
    HolidayRule::fixed_public(
        "Labour Day",
        "Darba svētki, Latvijas Republikas Satversmes sapulces sasaukšanas diena",
        Rule::gregorian(5, 1),
    ),
    HolidayRule::public(
        "Restoration of Independence Day",
        "Latvijas Republikas Neatkarības atjaunošanas diena",
        Rule::gregorian(5, 4),
    ),
    HolidayRule::fixed_public("Mother's Day", "Mātes diena", Rule::nth(5, 2, Weekday::Sunday)),
    HolidayRule::fixed_public("Pentecost", "Vasarsvētki", Rule::easter(PENTECOST))
        .years(Some(1995), None),
    HolidayRule::fixed_public("Līgo Day", "Līgo diena", Rule::gregorian(6, 23)),
    HolidayRule::fixed_public("Midsummer Day", "Jāņu diena", Rule::gregorian(6, 24)),
    HolidayRule::public(
        "Closing Day of the Nationwide Latvian Song and Dance Celebration",
        "Vispārējo latviešu Dziesmu un deju svētku noslēguma diena",
        Rule::Tabulated {
            function: lv_song_and_dance_celebration,
            first_year: 2018,
            last_year: 2023,
        },
    )
    .years(Some(2018), None),
    HolidayRule::public(
        "Proclamation Day of the Republic of Latvia",
        "Latvijas Republikas Proklamēšanas diena",
        Rule::gregorian(11, 18),
    ),
    HolidayRule::fixed_public("Christmas Eve", "Ziemassvētku vakars", Rule::gregorian(12, 24)),
    HolidayRule::fixed_public("Christmas Day", "Pirmie Ziemassvētki", Rule::gregorian(12, 25)),
    HolidayRule::fixed_public(
        "Second Day of Christmas",
        "Otrie Ziemassvētki",
        Rule::gregorian(12, 26),
    ),
    HolidayRule::fixed_public("New Year's Eve", "Vecgada diena", Rule::gregorian(12, 31)),
    // The two one-off holidays the law names by date.
    HolidayRule::fixed_public(
        "Pastoral Visit of Pope Francis to Latvia",
        "Viņa Svētības pāvesta Franciska pastorālās vizītes Latvijā diena",
        Rule::gregorian(9, 24),
    )
    .years(Some(2018), Some(2018)),
    HolidayRule::fixed_public(
        "Bronze Medal of the Latvian Ice Hockey Team at the 2023 World Championship",
        "diena, kad Latvijas hokeja komanda ieguva bronzas medaļu 2023. gada Pasaules hokeja čempionātā",
        Rule::gregorian(5, 29),
    )
    .years(Some(2023), Some(2023)),
    // Section 2: the remembrance and celebration days, working days.
    HolidayRule::observance(
        "Remembrance Day of the Defenders of the Barricades of 1991",
        "1991. gada barikāžu aizstāvju atceres diena",
        Rule::gregorian(1, 20),
    )
    .years(Some(1997), None),
    HolidayRule::observance(
        "International Recognition Day of the Republic of Latvia",
        "Latvijas Republikas starptautiskās (de jure) atzīšanas diena",
        Rule::gregorian(1, 26),
    )
    .years(Some(1995), None),
    HolidayRule::observance(
        "International Day of Non-Governmental Organisations",
        "Starptautiskā nevalstisko organizāciju diena",
        Rule::gregorian(2, 27),
    )
    .years(Some(2025), None),
    HolidayRule::observance(
        "Remembrance Day of the Armed Resistance of the National Partisans",
        "Nacionālo partizānu bruņotās pretošanās atceres diena",
        Rule::gregorian(3, 2),
    ),
    HolidayRule::observance(
        "International Women's Day",
        "Starptautiskā sieviešu diena",
        Rule::gregorian(3, 8),
    )
    .years(Some(2007), None),
    HolidayRule::observance(
        "Remembrance Day of the National Resistance Movement",
        "Nacionālās pretošanās kustības piemiņas diena",
        Rule::gregorian(3, 17),
    ),
    HolidayRule::observance(
        "Remembrance Day of the Victims of Communist Genocide",
        "Komunistiskā genocīda upuru piemiņas diena",
        Rule::gregorian(3, 25),
    ),
    HolidayRule::observance("Latgale Congress Day", "Latgales kongresa diena", Rule::gregorian(4, 27)),
    HolidayRule::observance(
        "Day of the Defeat of Nazism and Remembrance of the Victims of the Second World War",
        "Nacisma sagrāves diena un Otrā pasaules kara upuru piemiņas diena",
        Rule::gregorian(5, 8),
    )
    .years(Some(1995), None),
    HolidayRule::observance("Europe Day", "Eiropas diena", Rule::gregorian(5, 9))
        .years(Some(1997), None),
    HolidayRule::observance(
        "International Day of Families",
        "Starptautiskā ģimenes diena",
        Rule::gregorian(5, 15),
    )
    .years(Some(2007), None),
    HolidayRule::observance(
        "Firefighters' and Rescuers' Day",
        "Ugunsdzēsēju un glābēju diena",
        Rule::gregorian(5, 17),
    ),
    HolidayRule::observance(
        "International Children's Day",
        "Starptautiskā bērnu aizsardzības diena",
        Rule::gregorian(6, 1),
    )
    .years(Some(2007), None),
    HolidayRule::observance(
        "Remembrance Day of the Victims of Communist Genocide",
        "Komunistiskā genocīda upuru piemiņas diena",
        Rule::gregorian(6, 14),
    ),
    HolidayRule::observance(
        "Day of the Occupation of the Republic of Latvia",
        "Latvijas Republikas okupācijas diena",
        Rule::gregorian(6, 17),
    )
    .years(Some(2000), None),
    HolidayRule::observance(
        "Medical Workers' Day",
        "Medicīnas darbinieku diena",
        Rule::nth(6, 3, Weekday::Sunday),
    ),
    HolidayRule::observance(
        "Heroes' Remembrance Day",
        "Varoņu piemiņas diena (Cēsu kaujas atceres diena)",
        Rule::gregorian(6, 22),
    )
    .years(Some(1995), None),
    HolidayRule::observance(
        "Remembrance Day of the Victims of the Genocide of the Jewish People",
        "Ebreju tautas genocīda upuru piemiņas diena",
        Rule::gregorian(7, 4),
    ),
    HolidayRule::observance(
        "Sea Festival Day",
        "Jūras svētku diena",
        Rule::nth(7, 2, Weekday::Saturday),
    )
    .years(Some(2007), None),
    HolidayRule::observance(
        "Remembrance Day of the Latvian Freedom Fighters",
        "Latvijas brīvības cīnītāju piemiņas diena",
        Rule::gregorian(8, 11),
    )
    .years(Some(1995), None),
    HolidayRule::observance(
        "Day of the Adoption of the Constitutional Law on the Statehood of the Republic of Latvia",
        "Konstitucionālā likuma “Par Latvijas Republikas valstisko statusu” pieņemšanas diena",
        Rule::gregorian(8, 21),
    )
    .years(Some(2002), None),
    HolidayRule::observance(
        "Remembrance Day of the Victims of Stalinism and Nazism",
        "staļinisma un nacisma upuru atceres diena",
        Rule::gregorian(8, 23),
    )
    .years(Some(2009), None),
    HolidayRule::observance("Knowledge Day", "Zinību diena", Rule::gregorian(9, 1))
        .years(Some(2002), None),
    HolidayRule::observance("Father's Day", "Tēva diena", Rule::nth(9, 2, Weekday::Sunday))
        .years(Some(2008), None),
    HolidayRule::observance("Baltic Unity Day", "Baltu vienības diena", Rule::gregorian(9, 22))
        .years(Some(2000), None),
    HolidayRule::observance(
        "International Day of Older Persons",
        "Starptautiskā senioru diena",
        Rule::gregorian(10, 1),
    ),
    HolidayRule::observance("Teachers' Day", "Skolotāju diena", Rule::gregorian(10, 5)),
    HolidayRule::observance("State Language Day", "Valsts valodas diena", Rule::gregorian(10, 15)),
    HolidayRule::observance("Border Guards' Day", "Robežsargu diena", Rule::gregorian(11, 7)),
    HolidayRule::observance("Lāčplēsis Day", "Lāčplēša diena", Rule::gregorian(11, 11)),
    HolidayRule::observance(
        "Remembrance Day of the Tragedy of 21 November 2013",
        "2013. gada 21. novembra traģēdijas atceres diena",
        Rule::gregorian(11, 21),
    )
    .years(Some(2014), None),
    HolidayRule::observance("Police Officers' Day", "Policijas darbinieku diena", Rule::gregorian(12, 5)),
    HolidayRule::observance(
        "Remembrance Day of the Victims of the Genocide of the Totalitarian Communist Regime against the Latvian People",
        "pret latviešu tautu vērstā totalitārā komunistiskā režīma genocīda upuru piemiņas diena",
        Rule::nth(12, 1, Weekday::Sunday),
    )
    .years(Some(1998), None),
];

/// Latvia.
///
/// The law "On Holidays, Remembrance Days and Days to be Celebrated" of
/// 1990 as in force from 18 March 2025: the holidays of section 1, days
/// off, with the two one-off days it names by date, and the remembrance
/// and celebration days of section 2, working days carried as observances.
/// Section 1 moves three holidays, and only three, off a weekend: 4 May,
/// the closing day of the Song and Dance Celebration and 18 November; the
/// year that clause began is not carried. The closing day is a table of
/// the dates the organisers set, 2018 and 2023, and the doc on the
/// function says why it starts in 2018. The years are those the Latvian
/// Wikipedia gives from the amending laws, Pentecost from 1995 and the
/// remembrance days it dates; the rest are not dated. The right of the
/// Orthodox and Old Believers to keep Easter, Pentecost and Christmas by
/// their own calendars is personal and not carried.
pub static LATVIA: RuleSet = RuleSet {
    code: "LV",
    english_name: "Latvia",
    rules: LV_RULES,
    substitution: LV_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Likums \"Par svētku, atceres un atzīmējamām dienām\", consolidated text \
              in force from 18 March 2025, likumi.lv, retrieved 2026-09-22; the \
              Latvian Wikipedia, \"Latvijas svētku, atceres un atzīmējamās \
              dienas\", retrieved the same day, for the years, and \"Vispārējie \
              latviešu Dziesmu un Deju svētki\" for the closing dates of 2018 \
              and 2023; Wikipedia, \"Public holidays in Latvia\", for the \
              English names",
};

// ─────────────────────────────────────────────────────────────────────────
// Lithuania
// ─────────────────────────────────────────────────────────────────────────

static LT_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Naujieji metai", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Day of Restoration of the State of Lithuania",
        "Lietuvos valstybės atkūrimo diena",
        Rule::gregorian(2, 16),
    )
    .years(Some(1990), None),
    HolidayRule::fixed_public(
        "Day of Restoration of Independence of Lithuania",
        "Lietuvos nepriklausomybės atkūrimo diena",
        Rule::gregorian(3, 11),
    )
    .years(Some(1990), None),
    HolidayRule::fixed_public("Easter Sunday", "Velykos", Rule::easter(EASTER_SUNDAY)),
    HolidayRule::fixed_public(
        "Easter Monday",
        "antroji Velykų diena",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::fixed_public(
        "International Workers' Day",
        "Tarptautinė darbo diena",
        Rule::gregorian(5, 1),
    ),
    HolidayRule::fixed_public(
        "Mother's Day",
        "Motinos diena",
        Rule::nth(5, 1, Weekday::Sunday),
    ),
    HolidayRule::fixed_public(
        "Father's Day",
        "Tėvo diena",
        Rule::nth(6, 1, Weekday::Sunday),
    )
    .years(Some(2009), None),
    HolidayRule::fixed_public("Midsummer Day", "Rasos (Joninės)", Rule::gregorian(6, 24))
        .years(Some(2003), None),
    HolidayRule::fixed_public(
        "Statehood Day",
        "Valstybės (Lietuvos karaliaus Mindaugo karūnavimo) diena",
        Rule::gregorian(7, 6),
    )
    .years(Some(1991), None),
    HolidayRule::fixed_public(
        "Assumption Day",
        "Žolinė (Švč. Mergelės Marijos ėmimo į dangų diena)",
        Rule::gregorian(8, 15),
    )
    .years(Some(1991), None),
    HolidayRule::fixed_public(
        "All Saints' Day",
        "Visų šventųjų diena",
        Rule::gregorian(11, 1),
    ),
    HolidayRule::fixed_public("All Souls' Day", "Vėlinės", Rule::gregorian(11, 2))
        .years(Some(2020), None),
    HolidayRule::fixed_public("Christmas Eve", "Kūčios", Rule::gregorian(12, 24))
        .years(Some(2011), None),
    HolidayRule::fixed_public("Christmas Day", "Kalėdos", Rule::gregorian(12, 25)),
    HolidayRule::fixed_public(
        "Second Day of Christmas",
        "Kalėdos",
        Rule::gregorian(12, 26),
    ),
];

/// Lithuania.
///
/// Article 123 of the Labour Code, the public holidays on which work
/// stops, as the Lithuanian Wikipedia's list reproduces it. The years:
/// 16 February and 11 March from the Law on Holidays of 1990, Statehood
/// Day and the Assumption from 1991, Midsummer from 2003, Father's Day
/// from the Seimas decision of 2009, All Souls' Day from 2020, and
/// Christmas Eve from 2011, the year after the Seimas vote of 9 December
/// 2010 that the source reports as taking effect the next year. Mother's
/// Day, All Saints' Day and the rest are not dated. No substitution: a
/// holiday on a weekend stays there. The seventy-odd memorable days of the
/// separate Law on Memorable Days are not carried.
pub static LITHUANIA: RuleSet = RuleSet {
    code: "LT",
    english_name: "Lithuania",
    rules: LT_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Lietuvos Respublikos darbo kodeksas, 123 straipsnis, as listed by the \
              Lithuanian Wikipedia, \"Sąrašas:Lietuvos šventės\", and Wikipedia, \
              \"Public holidays in Lithuania\", both retrieved 2026-09-22; \
              sventinesdienos.lt, retrieved the same day, for 24 June (2003), \
              6 July and 15 August (1991) and 2 November (2020); \
              nedarbodienos.lt for Father's Day (2009); 15min.lt, 9 December \
              2010, for Christmas Eve",
};

// ─────────────────────────────────────────────────────────────────────────
// Albania
// ─────────────────────────────────────────────────────────────────────────

/// Law 7651 of 1992: a holiday on a Saturday or Sunday gives the first
/// working day after it, each holiday its own, so that a Saturday
/// 28 November and a Sunday 29 November give the Monday and the Tuesday.
static AL_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static AL_RULES: &[HolidayRule] = &[
    HolidayRule::public(
        "New Year's Day",
        "Festat e Vitit të Ri",
        Rule::gregorian(1, 1),
    ),
    HolidayRule::public(
        "New Year's Day",
        "Festat e Vitit të Ri",
        Rule::gregorian(1, 2),
    ),
    HolidayRule::public("Summer Day", "Dita e Verës", Rule::gregorian(3, 14))
        .years(Some(2004), None),
    HolidayRule::public("Nevruz Day", "Dita e Nevruzit", Rule::gregorian(3, 22)),
    // Both Easters are Sundays, and the weekend rule gives their Mondays.
    HolidayRule::public(
        "Catholic Easter",
        "Pashkët Katolike",
        Rule::easter(EASTER_SUNDAY),
    ),
    HolidayRule::public(
        "Orthodox Easter",
        "Pashkët Ortodokse",
        Rule::paschal(EASTER_SUNDAY),
    ),
    HolidayRule::public(
        "International Workers' Day",
        "Dita Ndërkombëtare e Punonjësve",
        Rule::gregorian(5, 1),
    ),
    HolidayRule::public(
        "Eid al-Fitr",
        "Dita e Fitër Bajramit",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 1),
    )
    .approximate(),
    HolidayRule::public(
        "Eid al-Adha",
        "Dita e Kurban Bajramit",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 10),
    )
    .approximate(),
    HolidayRule::public(
        "Mother Teresa Day",
        "Dita e Nënë Terezës",
        Rule::gregorian(9, 5),
    ),
    HolidayRule::public("Alphabet Day", "Dita e Alfabetit", Rule::gregorian(11, 22))
        .years(Some(2024), None),
    HolidayRule::public(
        "Independence Day",
        "Dita e Pavarësisë",
        Rule::gregorian(11, 28),
    ),
    HolidayRule::public("Liberation Day", "Dita e Çlirimit", Rule::gregorian(11, 29)),
    HolidayRule::public(
        "National Youth Day",
        "Dita Kombëtare e Rinisë",
        Rule::gregorian(12, 8),
    )
    .years(Some(2010), None),
    HolidayRule::public("Christmas Day", "Krishtlindjet", Rule::gregorian(12, 25)),
];

/// Albania.
///
/// Law 7651 of 21 December 1992 on official holidays as amended: the
/// fifteen holidays, with the two Easters on their Sundays, the two Eids
/// on dates the Muslim Community announces and the tabular calendar
/// approximates, and a holiday on a weekend giving the first working day
/// after it. The years the sources give are carried: Summer Day from 2004,
/// National Youth Day from 2010 and Alphabet Day from the Government
/// decision of 11 January 2024; the rest are not dated. The extra days
/// the Council of Ministers declares around a holiday are not carried.
pub static ALBANIA: RuleSet = RuleSet {
    code: "AL",
    english_name: "Albania",
    rules: AL_RULES,
    substitution: AL_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "The Albanian Wikipedia, \"Lista e festave në Shqipëri\", and Wikipedia, \
              \"Public holidays in Albania\", both retrieved 2026-09-22, for the list, \
              the weekend rule and the years of Summer Day and Youth Day; \
              festazyrtare.al, retrieved the same day, for Law 7651 of 21 December \
              1992 and the 2026 dates; Shqiptarja.com, 11 January 2024, for \
              Alphabet Day",
};

// ─────────────────────────────────────────────────────────────────────────
// Montenegro
// ─────────────────────────────────────────────────────────────────────────

/// The law's Sunday rule: a holiday's first day on a Sunday makes the two
/// days after it non-working, and its second day on a Sunday makes the
/// working day after non-working. A Sunday-only forward policy that skips
/// occupied days does both.
static ME_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// A religious holiday that is a day off for the members of its
/// community alone.
const fn me_religious(name: &'static str, local: &'static str, rule: Rule) -> HolidayRule {
    HolidayRule::observance(name, local, rule).of_kind(Kind::Religious)
}

static ME_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Nova godina", Rule::gregorian(1, 1)),
    HolidayRule::public("New Year's Day", "Nova godina", Rule::gregorian(1, 2)),
    HolidayRule::public("Labour Day", "Praznik rada", Rule::gregorian(5, 1)),
    HolidayRule::public("Labour Day", "Praznik rada", Rule::gregorian(5, 2)),
    HolidayRule::public(
        "Independence Day",
        "Dan nezavisnosti",
        Rule::gregorian(5, 21),
    )
    .years(Some(2007), None),
    HolidayRule::public(
        "Independence Day",
        "Dan nezavisnosti",
        Rule::gregorian(5, 22),
    )
    .years(Some(2007), None),
    HolidayRule::public("Statehood Day", "Dan državnosti", Rule::gregorian(7, 13)),
    HolidayRule::public("Statehood Day", "Dan državnosti", Rule::gregorian(7, 14)),
    HolidayRule::public("Njegoš Day", "Njegošev dan", Rule::gregorian(11, 13))
        .years(Some(2022), None),
    HolidayRule::public("Njegoš Day", "Njegošev dan", Rule::gregorian(11, 14))
        .years(Some(2022), None),
    // Orthodox believers.
    me_religious(
        "Orthodox Christmas Eve",
        "Badnji dan",
        Rule::gregorian(1, 6),
    ),
    me_religious("Orthodox Christmas", "Božić", Rule::gregorian(1, 7)),
    me_religious("Orthodox Christmas", "Božić", Rule::gregorian(1, 8)),
    me_religious(
        "Orthodox Good Friday",
        "Veliki petak",
        Rule::paschal(GOOD_FRIDAY),
    ),
    me_religious(
        "Orthodox Easter Monday",
        "Vaskršnji ponedjeljak",
        Rule::paschal(EASTER_MONDAY),
    ),
    // Catholics.
    me_religious("Christmas Eve", "Badnji dan", Rule::gregorian(12, 24)),
    me_religious("Christmas Day", "Božić", Rule::gregorian(12, 25)),
    me_religious("Christmas Day", "Božić", Rule::gregorian(12, 26)),
    me_religious("Good Friday", "Veliki petak", Rule::easter(GOOD_FRIDAY)),
    me_religious(
        "Easter Monday",
        "Uskršnji ponedjeljak",
        Rule::easter(EASTER_MONDAY),
    ),
    me_religious("All Saints' Day", "Svi Sveti", Rule::gregorian(11, 1)),
    // Muslims: three days of each Bajram.
    me_religious(
        "Eid al-Fitr",
        "Ramazanski bajram",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 1),
    )
    .approximate(),
    me_religious(
        "Eid al-Fitr",
        "Ramazanski bajram",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 2),
    )
    .approximate(),
    me_religious(
        "Eid al-Fitr",
        "Ramazanski bajram",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 3),
    )
    .approximate(),
    me_religious(
        "Eid al-Adha",
        "Kurbanski bajram",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 10),
    )
    .approximate(),
    me_religious(
        "Eid al-Adha",
        "Kurbanski bajram",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 11),
    )
    .approximate(),
    me_religious(
        "Eid al-Adha",
        "Kurbanski bajram",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 12),
    )
    .approximate(),
    // Jews: two days each.
    me_religious(
        "Passover",
        "Pasha",
        Rule::in_calendar(CalendarSystem::HEBREW, 7, 15),
    ),
    me_religious(
        "Passover",
        "Pasha",
        Rule::in_calendar(CalendarSystem::HEBREW, 7, 16),
    ),
    me_religious(
        "Yom Kippur",
        "Jom Kipur",
        Rule::in_calendar(CalendarSystem::HEBREW, 1, 10),
    ),
    me_religious(
        "Yom Kippur",
        "Jom Kipur",
        Rule::in_calendar(CalendarSystem::HEBREW, 1, 11),
    ),
];

/// Montenegro.
///
/// The Law on State and Other Holidays of 2007: the state holidays,
/// Independence Day and Statehood Day, and the other holidays, New Year
/// and Labour Day, each two days, and Njegoš Day from 2022, voted in
/// December 2021, with the
/// Sunday rule as a forward policy. The religious holidays the law gives
/// the believers of each community — the Orthodox, Catholic, Muslim and
/// Jewish days, three of each Bajram and two of Passover and Yom Kippur —
/// are [`Kind::Religious`]: days off for those believers, not for all.
/// The Orthodox Krsna slava, a family's own day, is not carried, nor are
/// the years the older holidays were set.
pub static MONTENEGRO: RuleSet = RuleSet {
    code: "ME",
    english_name: "Montenegro",
    rules: ME_RULES,
    substitution: ME_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Zakon o državnim i drugim praznicima, as reproduced by paragraf.me, \
              retrieved 2026-09-22, for the state and other holidays, the two-day \
              rule and the Sunday rule; Wikipedia, \"Public holidays in \
              Montenegro\", retrieved the same day, for the religious holidays and \
              the names; Danas, 29 December 2021, for Njegoš Day's vote, and CdM \
              for its two days",
};

// ─────────────────────────────────────────────────────────────────────────
// North Macedonia
// ─────────────────────────────────────────────────────────────────────────

/// Article 3 of the Law on Holidays: a holiday of article 2 on a Sunday
/// makes the next day non-working.
static MK_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// A day off for the members of a religious community alone.
const fn mk_religious(name: &'static str, local: &'static str, rule: Rule) -> HolidayRule {
    HolidayRule::observance(name, local, rule).of_kind(Kind::Religious)
}

static MK_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Нова Година", Rule::gregorian(1, 1)),
    HolidayRule::public("Orthodox Christmas", "Прв ден Божик", Rule::gregorian(1, 7)),
    HolidayRule::public(
        "Orthodox Easter Monday",
        "Втор ден Велигден",
        Rule::paschal(EASTER_MONDAY),
    ),
    HolidayRule::public("Labour Day", "Ден на трудот", Rule::gregorian(5, 1)),
    HolidayRule::public(
        "Saints Cyril and Methodius Day",
        "Св. Кирил и Методиј, Ден на сесловенските просветители",
        Rule::gregorian(5, 24),
    ),
    HolidayRule::public("Republic Day", "Ден на Републиката", Rule::gregorian(8, 2)),
    HolidayRule::public(
        "Independence Day",
        "Ден на независноста",
        Rule::gregorian(9, 8),
    ),
    HolidayRule::public(
        "Day of the People's Uprising",
        "Ден на народното востание",
        Rule::gregorian(10, 11),
    ),
    HolidayRule::public(
        "Day of the Macedonian Revolutionary Struggle",
        "Ден на македонската револуционерна борба",
        Rule::gregorian(10, 23),
    )
    .years(Some(2007), None),
    HolidayRule::public(
        "Saint Clement of Ohrid Day",
        "Св. Климент Охридски",
        Rule::gregorian(12, 8),
    )
    .years(Some(2007), None),
    HolidayRule::public(
        "Eid al-Fitr",
        "Рамазан Бајрам",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 1),
    )
    .approximate()
    .years(Some(2007), None),
    // Days off for the members of the religious communities.
    mk_religious("Orthodox Christmas Eve", "Бадник", Rule::gregorian(1, 6)),
    mk_religious("Orthodox Epiphany", "Водици", Rule::gregorian(1, 19)),
    mk_religious(
        "Orthodox Good Friday",
        "Велики Петок",
        Rule::paschal(GOOD_FRIDAY),
    ),
    // The Friday before Pentecost.
    mk_religious("Duhovden", "Духовден", Rule::paschal(PENTECOST - 2)),
    mk_religious(
        "Dormition of the Theotokos",
        "Успение на Пресвета Богородица",
        Rule::gregorian(8, 28),
    ),
    mk_religious(
        "Catholic Easter Monday",
        "Втор ден Велигден (католички)",
        Rule::easter(EASTER_MONDAY),
    ),
    mk_religious("All Saints' Day", "Сите Светци", Rule::gregorian(11, 1)),
    mk_religious("Catholic Christmas", "Божиќ", Rule::gregorian(12, 25)),
    mk_religious(
        "Eid al-Adha",
        "Курбан Бајрам",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 10),
    )
    .approximate(),
    mk_religious(
        "Yom Kippur",
        "Јом Кипур",
        Rule::in_calendar(CalendarSystem::HEBREW, 1, 10),
    ),
    // Days off for the members of the ethnic communities, and the working
    // holidays.
    HolidayRule::observance("Saint Sava Day", "Свети Сава", Rule::gregorian(1, 27)),
    HolidayRule::observance(
        "International Romani Day",
        "Меѓународен ден на Ромите",
        Rule::gregorian(4, 8),
    ),
    HolidayRule::observance(
        "Macedonian Language Day",
        "Ден на македонскиот јазик",
        Rule::gregorian(5, 5),
    )
    .years(Some(2019), None),
    HolidayRule::observance(
        "Aromanian National Day",
        "Национален ден на Власите",
        Rule::gregorian(5, 23),
    ),
    HolidayRule::observance("Army Day", "Ден на Армијата", Rule::gregorian(8, 18)),
    HolidayRule::observance(
        "International Bosniaks Day",
        "Меѓународен ден на Бошњаците",
        Rule::gregorian(9, 28),
    ),
    HolidayRule::observance(
        "Albanian Alphabet Day",
        "Ден на албанската азбука",
        Rule::gregorian(11, 22),
    ),
    HolidayRule::observance(
        "Turkish Language Education Day",
        "Ден на настава на турски јазик",
        Rule::gregorian(12, 21),
    ),
];

/// North Macedonia.
///
/// The Law on Holidays of 1998 as amended in 2007: the holidays of
/// article 2, non-working for all, with the Day of the Macedonian
/// Revolutionary Struggle, Saint Clement's Day and the first day of
/// Ramazan Bajram from the 2007 amendment, and article 3's Sunday rule as
/// a forward policy. The days the law gives the members of the religious
/// communities are [`Kind::Religious`] and those it gives the ethnic
/// communities are observances: both are days off for members alone, and
/// the crate has no kind that says so. Duhovden is the Friday before
/// Pentecost, as the law defines it. Macedonian Language Day and Army Day
/// are working holidays.
pub static NORTH_MACEDONIA: RuleSet = RuleSet {
    code: "MK",
    english_name: "North Macedonia",
    rules: MK_RULES,
    substitution: MK_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Закон за празниците на Република Македонија, as reproduced on the \
              Macedonian Wikisource, retrieved 2026-09-22, for articles 2 and 3 and \
              the definition of Духовден; Wikipedia, \"Public holidays in North \
              Macedonia\", retrieved the same day, for the community days, the \
              2007 additions and the names",
};

// ─────────────────────────────────────────────────────────────────────────
// Serbia
// ─────────────────────────────────────────────────────────────────────────

/// Article 3a: a state holiday on a Sunday makes the first working day
/// after it non-working. The religious holidays of article 2 are
/// `fixed_public` and the policy never reaches them.
static RS_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// Article 4: a day the employees of another confession have the right
/// not to work.
const fn rs_religious(name: &'static str, local: &'static str, rule: Rule) -> HolidayRule {
    HolidayRule::observance(name, local, rule).of_kind(Kind::Religious)
}

static RS_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Нова година", Rule::gregorian(1, 1)),
    HolidayRule::public("New Year's Day", "Нова година", Rule::gregorian(1, 2)),
    HolidayRule::fixed_public("Christmas Day", "Божић", Rule::gregorian(1, 7)),
    HolidayRule::public(
        "Statehood Day",
        "Дан државности Србије",
        Rule::gregorian(2, 15),
    )
    .years(Some(2002), None),
    HolidayRule::public(
        "Statehood Day",
        "Дан државности Србије",
        Rule::gregorian(2, 16),
    )
    .years(Some(2002), None),
    HolidayRule::fixed_public("Good Friday", "Велики петак", Rule::paschal(GOOD_FRIDAY)),
    HolidayRule::fixed_public(
        "Holy Saturday",
        "Велика субота",
        Rule::paschal(HOLY_SATURDAY),
    ),
    HolidayRule::fixed_public("Easter Sunday", "Васкрс", Rule::paschal(EASTER_SUNDAY)),
    HolidayRule::fixed_public(
        "Easter Monday",
        "Васкрсни понедељак",
        Rule::paschal(EASTER_MONDAY),
    ),
    HolidayRule::public("Labour Day", "Празник рада", Rule::gregorian(5, 1)),
    HolidayRule::public("Labour Day", "Празник рада", Rule::gregorian(5, 2)),
    HolidayRule::public(
        "Armistice Day",
        "Дан примирја у Првом светском рату",
        Rule::gregorian(11, 11),
    )
    .years(Some(2012), None),
    // The state holidays article 3 keeps as working days.
    HolidayRule::observance("Saint Sava Day", "Савиндан", Rule::gregorian(1, 27)),
    HolidayRule::observance(
        "Holocaust Remembrance Day",
        "Дан сећања на жртве холокауста, геноцида и других жртава фашизма у Другом светском рату",
        Rule::gregorian(4, 22),
    ),
    HolidayRule::observance("Victory Day", "Дан победе", Rule::gregorian(5, 9)),
    HolidayRule::observance(
        "Saints Cyril and Methodius Day",
        "Дан Ћирила и Методија",
        Rule::gregorian(5, 24),
    ),
    HolidayRule::observance("Vidovdan", "Видовдан", Rule::gregorian(6, 28)),
    HolidayRule::observance(
        "Day of Serbian Unity, Freedom and the National Flag",
        "Дан српског јединства, слободе и националне заставе",
        Rule::gregorian(9, 15),
    )
    .years(Some(2020), None),
    HolidayRule::observance(
        "Serbian Victims of the Second World War Remembrance Day",
        "Дан сећања на српске жртве у Другом светском рату",
        Rule::gregorian(10, 21),
    ),
    // Article 4: the other confessions.
    rs_religious(
        "Catholic Christmas",
        "Католички Божић",
        Rule::gregorian(12, 25),
    ),
    rs_religious(
        "Catholic Good Friday",
        "Велики петак (католички)",
        Rule::easter(GOOD_FRIDAY),
    ),
    rs_religious(
        "Catholic Easter Sunday",
        "Ускрс",
        Rule::easter(EASTER_SUNDAY),
    ),
    rs_religious(
        "Catholic Easter Monday",
        "Ускршњи понедељак",
        Rule::easter(EASTER_MONDAY),
    ),
    rs_religious(
        "Eid al-Fitr",
        "Рамазански Бајрам",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 1),
    )
    .approximate(),
    rs_religious(
        "Eid al-Adha",
        "Курбански Бајрам",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 10),
    )
    .approximate(),
    rs_religious(
        "Yom Kippur",
        "Јом Кипур",
        Rule::in_calendar(CalendarSystem::HEBREW, 1, 10),
    ),
];

/// Serbia.
///
/// The Law on State and Other Holidays (Official Gazette 43/2001, 101/2007
/// and 92/2011): the state holidays of article 1, non-working, with
/// Statehood Day from 2002 and Armistice Day non-working from 2012; the
/// religious holidays of article 2, Christmas and the Easter days from
/// Good Friday to Easter Monday by the Julian computus, non-working for
/// all; article 3's state holidays that are working days as observances;
/// and the days article 4 gives the employees of the other confessions
/// the right not to work as [`Kind::Religious`]. Article 3a moves a state
/// holiday on a Sunday to the first working day after, and it reaches the
/// religious days of article 2 not at all. The Orthodox Krsna slava, a
/// family's own day, is not carried.
pub static SERBIA: RuleSet = RuleSet {
    code: "RS",
    english_name: "Serbia",
    rules: RS_RULES,
    substitution: RS_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Zakon o državnim i drugim praznicima u Republici Srbiji, as reproduced \
              by paragraf.rs, retrieved 2026-09-22, for the articles and the \
              gazette numbers; the Serbian Wikipedia, \"Државни и верски празници \
              у Србији\", and Wikipedia, \"Public holidays in Serbia\", both \
              retrieved the same day, for the names, the 2012 start of Armistice \
              Day and the 2020 start of 15 September",
};

// ─────────────────────────────────────────────────────────────────────────
// Bosnia and Herzegovina
// ─────────────────────────────────────────────────────────────────────────

/// The Federation of Bosnia and Herzegovina.
const BA_FEDERATION: &[&str] = &["BA-BIH"];
/// Republika Srpska.
const BA_SRPSKA: &[&str] = &["BA-SRP"];
/// Brčko District.
const BA_BRCKO: &[&str] = &["BA-BRC"];

/// The three Sunday rules as one policy, each reaching only the days its law
/// names: in the Federation a day of New Year or of 1 May on a Sunday makes
/// the first day after the two a holiday; in Republika Srpska article 4
/// moves only the second day of a two-day holiday, to the Monday; in Brčko
/// article 5 makes the first working day after any District holiday on a
/// Sunday non-working. A Sunday-only forward policy that skips occupied days
/// does all three, and the days a law does not reach are `fixed_public`.
static BA_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// A day of a Brčko Assembly decision, read from `table`, or nothing.
fn ba_decided(table: &[(i64, u8, u8)], year: i64) -> Days {
    table
        .iter()
        .find(|(decided, _, _)| *decided == year)
        .and_then(|(_, month, day)| gregorian::to_fixed(year, *month, *day).ok())
        .map_or_else(Days::new, Days::one)
}

/// The Assembly's decisions on the non-working days of the religious
/// holidays, one a year from March or April to the January after, as read
/// for the decisions of 2017 to 2026. The Assembly picks the day: Easter
/// Monday in some years, Good Friday in others, and the working day next to
/// a Christmas that falls at the weekend.
static BA_BRCKO_CATHOLIC_EASTER: &[(i64, u8, u8)] = &[
    (2017, 4, 17),
    (2018, 4, 2),
    (2019, 4, 22),
    (2020, 4, 13),
    (2021, 4, 5),
    (2022, 4, 18),
    (2023, 4, 10),
    (2024, 4, 1),
    (2025, 4, 18),
    (2026, 4, 6),
];
static BA_BRCKO_ORTHODOX_EASTER: &[(i64, u8, u8)] = &[
    (2017, 4, 14),
    (2018, 4, 9),
    (2019, 4, 26),
    (2020, 4, 17),
    (2021, 4, 30),
    (2022, 4, 22),
    (2023, 4, 17),
    (2024, 5, 6),
    (2025, 4, 21),
    (2026, 4, 13),
];
static BA_BRCKO_EID_AL_FITR: &[(i64, u8, u8)] = &[
    (2017, 6, 26),
    (2018, 6, 15),
    (2019, 6, 4),
    (2020, 5, 25),
    (2021, 5, 13),
    (2022, 5, 4),
    (2023, 4, 21),
    (2024, 4, 10),
    (2025, 3, 31),
    (2026, 3, 20),
];
static BA_BRCKO_EID_AL_ADHA: &[(i64, u8, u8)] = &[
    (2017, 9, 1),
    (2018, 8, 21),
    (2019, 8, 12),
    (2020, 7, 31),
    (2021, 7, 20),
    (2022, 7, 11),
    (2023, 6, 28),
    (2024, 6, 17),
    (2025, 6, 6),
    (2026, 5, 27),
];
static BA_BRCKO_CATHOLIC_CHRISTMAS: &[(i64, u8, u8)] = &[
    (2017, 12, 25),
    (2018, 12, 25),
    (2019, 12, 25),
    (2020, 12, 25),
    (2021, 12, 24),
    (2022, 12, 26),
    (2023, 12, 25),
    (2024, 12, 25),
    (2025, 12, 25),
    (2026, 12, 25),
];
/// The January of each decision is the next year's: the 2016 decision,
/// which would give January 2017, was not read, and the 2026 decision
/// gives January 2027.
static BA_BRCKO_ORTHODOX_CHRISTMAS: &[(i64, u8, u8)] = &[
    (2018, 1, 8),
    (2019, 1, 7),
    (2020, 1, 7),
    (2021, 1, 7),
    (2022, 1, 7),
    (2023, 1, 9),
    (2024, 1, 8),
    (2025, 1, 7),
    (2026, 1, 7),
    (2027, 1, 7),
];

fn ba_brcko_catholic_easter(year: i64) -> Days {
    ba_decided(BA_BRCKO_CATHOLIC_EASTER, year)
}

fn ba_brcko_orthodox_easter(year: i64) -> Days {
    ba_decided(BA_BRCKO_ORTHODOX_EASTER, year)
}

fn ba_brcko_eid_al_fitr(year: i64) -> Days {
    ba_decided(BA_BRCKO_EID_AL_FITR, year)
}

fn ba_brcko_eid_al_adha(year: i64) -> Days {
    ba_decided(BA_BRCKO_EID_AL_ADHA, year)
}

fn ba_brcko_catholic_christmas(year: i64) -> Days {
    ba_decided(BA_BRCKO_CATHOLIC_CHRISTMAS, year)
}

fn ba_brcko_orthodox_christmas(year: i64) -> Days {
    ba_decided(BA_BRCKO_ORTHODOX_CHRISTMAS, year)
}

/// A religious holiday's day in Brčko, as the Assembly's decisions give
/// it for the years read and a gap for every other.
const fn ba_brcko_decided(
    name: &'static str,
    local_name: &'static str,
    function: fn(i64) -> Days,
    first_year: i64,
    last_year: i64,
) -> HolidayRule {
    HolidayRule::fixed_public(
        name,
        local_name,
        Rule::Tabulated {
            function,
            first_year,
            last_year,
        },
    )
    .in_regions(BA_BRCKO)
}

/// 9 January in the years whose announcements were read: the Ministry of
/// Administration and Local Self-Government's notice for each January from
/// 2017 to 2026 names it a non-working day.
fn ba_srpska_republic_day(year: i64) -> Days {
    gregorian::to_fixed(year, 1, 9).map_or_else(|_| Days::new(), Days::one)
}

/// A day that article 8 gives the Orthodox, Catholic and Muslim believers
/// the right to be absent from work on, paid: a day off for them alone.
const fn ba_srpska_religious(name: &'static str, local: &'static str, rule: Rule) -> HolidayRule {
    HolidayRule::observance(name, local, rule)
        .of_kind(Kind::Religious)
        .in_regions(BA_SRPSKA)
}

static BA_RULES: &[HolidayRule] = &[
    // The Federation: the Law on Holidays the Republic took over in 1992
    // and the two laws of 1995.
    HolidayRule::public("New Year's Day", "Nova godina", Rule::gregorian(1, 1))
        .in_regions(BA_FEDERATION),
    HolidayRule::public("New Year's Day", "Nova godina", Rule::gregorian(1, 2))
        .in_regions(BA_FEDERATION),
    HolidayRule::fixed_public(
        "Independence Day",
        "Dan nezavisnosti Bosne i Hercegovine",
        Rule::gregorian(3, 1),
    )
    .in_regions(BA_FEDERATION),
    HolidayRule::public("Labour Day", "Praznik rada", Rule::gregorian(5, 1))
        .in_regions(BA_FEDERATION),
    HolidayRule::public("Labour Day", "Praznik rada", Rule::gregorian(5, 2))
        .in_regions(BA_FEDERATION),
    HolidayRule::observance(
        "Victory over Fascism Day",
        "Dan pobjede nad fašizmom",
        Rule::gregorian(5, 9),
    )
    .in_regions(BA_FEDERATION),
    HolidayRule::fixed_public(
        "Statehood Day",
        "Dan državnosti Bosne i Hercegovine",
        Rule::gregorian(11, 25),
    )
    .in_regions(BA_FEDERATION),
    // Republika Srpska: the Law on Holidays of 2007. Article 4 reaches
    // the second day of a two-day holiday only.
    HolidayRule::fixed_public("New Year's Day", "Нова година", Rule::gregorian(1, 1))
        .in_regions(BA_SRPSKA),
    HolidayRule::public("New Year's Day", "Нова година", Rule::gregorian(1, 2))
        .in_regions(BA_SRPSKA),
    HolidayRule::fixed_public("Republic Day", "Дан Републике", Rule::gregorian(1, 9))
        .years(None, Some(2016))
        .in_regions(BA_SRPSKA),
    HolidayRule::fixed_public(
        "Republic Day",
        "Дан Републике",
        Rule::Tabulated {
            function: ba_srpska_republic_day,
            first_year: 2017,
            last_year: 2026,
        },
    )
    .years(Some(2017), None)
    .in_regions(BA_SRPSKA),
    HolidayRule::fixed_public(
        "International Labour Day",
        "Међународни празник рада",
        Rule::gregorian(5, 1),
    )
    .in_regions(BA_SRPSKA),
    HolidayRule::public(
        "International Labour Day",
        "Међународни празник рада",
        Rule::gregorian(5, 2),
    )
    .in_regions(BA_SRPSKA),
    HolidayRule::fixed_public(
        "Victory over Fascism Day",
        "Дан побједе над фашизмом",
        Rule::gregorian(5, 9),
    )
    .in_regions(BA_SRPSKA),
    HolidayRule::fixed_public(
        "Dayton Agreement Day",
        "Дан успостављања Општег оквирног споразума за мир у Босни и Херцеговини",
        Rule::gregorian(11, 21),
    )
    .in_regions(BA_SRPSKA),
    // Articles 7 and 8: two days of each, Christmas on its Eve and Day,
    // and Good Friday one day of each Easter.
    ba_srpska_religious("Orthodox Christmas Eve", "Бадњи дан", Rule::gregorian(1, 6)),
    ba_srpska_religious("Orthodox Christmas", "Божић", Rule::gregorian(1, 7)),
    ba_srpska_religious(
        "Orthodox Good Friday",
        "Велики петак",
        Rule::paschal(GOOD_FRIDAY),
    ),
    ba_srpska_religious("Orthodox Easter", "Васкрс", Rule::paschal(EASTER_SUNDAY)),
    ba_srpska_religious(
        "Orthodox Easter Monday",
        "Васкрсни понедјељак",
        Rule::paschal(EASTER_MONDAY),
    ),
    ba_srpska_religious(
        "Catholic Good Friday",
        "Велики петак (католички)",
        Rule::easter(GOOD_FRIDAY),
    ),
    ba_srpska_religious("Catholic Easter", "Ускрс", Rule::easter(EASTER_SUNDAY)),
    ba_srpska_religious(
        "Catholic Easter Monday",
        "Ускршњи понедјељак",
        Rule::easter(EASTER_MONDAY),
    ),
    ba_srpska_religious(
        "Catholic Christmas Eve",
        "Бадњи дан (католички)",
        Rule::gregorian(12, 24),
    ),
    ba_srpska_religious(
        "Catholic Christmas",
        "Божић (католички)",
        Rule::gregorian(12, 25),
    ),
    ba_srpska_religious(
        "Eid al-Fitr",
        "Рамазански бајрам",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 1),
    )
    .approximate(),
    ba_srpska_religious(
        "Eid al-Fitr",
        "Рамазански бајрам",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 2),
    )
    .approximate(),
    ba_srpska_religious(
        "Eid al-Adha",
        "Курбан-бајрам",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 10),
    )
    .approximate(),
    ba_srpska_religious(
        "Eid al-Adha",
        "Курбан-бајрам",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 11),
    )
    .approximate(),
    // Brčko District: the Law on Holidays of 2002, and the Assembly's
    // yearly decisions.
    HolidayRule::public("New Year's Day", "Nova godina", Rule::gregorian(1, 1))
        .in_regions(BA_BRCKO),
    HolidayRule::public("New Year's Day", "Nova godina", Rule::gregorian(1, 2))
        .in_regions(BA_BRCKO),
    HolidayRule::public(
        "Brčko District Day",
        "Dan uspostavljanja Brčko distrikta",
        Rule::gregorian(3, 8),
    )
    .in_regions(BA_BRCKO),
    HolidayRule::public("Labour Day", "Praznik rada", Rule::gregorian(5, 1)).in_regions(BA_BRCKO),
    HolidayRule::public("Labour Day", "Praznik rada", Rule::gregorian(5, 2)).in_regions(BA_BRCKO),
    ba_brcko_decided(
        "Orthodox Christmas",
        "Božić (pravoslavni)",
        ba_brcko_orthodox_christmas,
        2018,
        2027,
    ),
    ba_brcko_decided(
        "Catholic Easter",
        "Uskrs (katolički)",
        ba_brcko_catholic_easter,
        2017,
        2026,
    ),
    ba_brcko_decided(
        "Orthodox Easter",
        "Vaskrs (pravoslavni)",
        ba_brcko_orthodox_easter,
        2017,
        2026,
    ),
    ba_brcko_decided(
        "Eid al-Fitr",
        "Ramazanski bajram",
        ba_brcko_eid_al_fitr,
        2017,
        2026,
    ),
    ba_brcko_decided(
        "Eid al-Adha",
        "Kurban-bajram",
        ba_brcko_eid_al_adha,
        2017,
        2026,
    ),
    ba_brcko_decided(
        "Catholic Christmas",
        "Božić (katolički)",
        ba_brcko_catholic_christmas,
        2017,
        2026,
    ),
];

/// Bosnia and Herzegovina.
///
/// There is no state law on holidays: the bills put to the Parliamentary
/// Assembly have all failed, and the national set is empty. Each of the
/// three units has its own, and the table carries them as regions.
///
/// **The Federation** (`BA-BIH`) applies the Law on Holidays of the SFRY
/// (Official Gazette of the SFRY 6/73), taken over as a law of the
/// Republic in 1992 and confirmed in 1994: New Year and 1 May two days
/// each, with a day of either on a Sunday making the first day after the
/// two a holiday, and 9 May kept as a working day, an observance here. The
/// laws of 1995 add Independence Day on 1 March and Statehood Day on 25
/// November as non-working days, with no Sunday rule; the Monday the
/// Government has declared for a Sunday 1 March, as in 2026, is not
/// carried. The Labour Law's four days of religious leave are chosen by
/// the worker and have no dates, and the cantons' own days are not
/// carried.
///
/// **Republika Srpska** (`BA-SRP`) follows its Law on Holidays (Official
/// Gazette 43/07): New Year and 1 May two days each, with article 4
/// moving the second day to the Monday when it falls on a Sunday, 9 May
/// and 21 November. Its 9 January, Republic Day, was struck from the law
/// by the Constitutional Court of Bosnia and Herzegovina (decision U 3/13,
/// the provision ceasing on 26 November 2016), re-enacted by the Law on
/// the Day of Republika Srpska (113/16) and struck again (U 2/18, 2019);
/// the entity's Ministry of Administration and Local Self-Government has
/// named it a non-working day every January since, under that law and a
/// Government decision. It is carried as the statute's until 2016 and as
/// the notices give it for 2017 to 2026, a gap after. The religious days of
/// articles 7 and 8 — two of each Christmas, Easter and Bajram, and each
/// Good Friday — are the believers' right to paid absence, and are
/// [`Kind::Religious`]. The extra days the Government declares for public
/// bodies, the Julian New Year on 14 January among them, are not carried.
///
/// **Brčko District** (`BA-BRC`) follows its Law on Holidays (Official
/// Gazette of the District 19/02 and 19/22): New Year and 1 May two days
/// each and District Day on 8 March, with article 5 making the first
/// working day after one on a Sunday non-working. The Assembly decides each
/// year which day of each Christmas, Easter and Bajram is non-working for
/// the District's institutions, public enterprises and health insurance
/// fund; those days are tabulated from the decisions of 2017 to 2026 and
/// are a gap in every other year. Private employers are not bound by
/// them.
pub static BOSNIA_AND_HERZEGOVINA: RuleSet = RuleSet {
    code: "BA",
    english_name: "Bosnia and Herzegovina",
    rules: BA_RULES,
    substitution: BA_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Federation: the Federal Ministry of Labour and Social Policy's notices on \
              fbihvlada.gov.ba for 1 May 2016, 1 May 2021 and New Year 2023, citing the Law \
              on Holidays (Sl. list SFRJ 6/73; Sl. list RBiH 2/92 and 13/94) and its Sunday \
              rule, and the Ministry's summary of the 1995 laws on 1 March and 25 November \
              (Sl. list RBiH 9/95) as a school reproduces it (msts-travnik.net); Republika \
              Srpska: Zakon o praznicima Republike Srpske (Sl. glasnik RS 43/07, with the \
              Constitutional Court's ruling in Sl. glasnik BiH 77/16) from Paragraf Lex \
              (paragraf.ba) and the Serbian Wikisource, the Constitutional Court's decision \
              U 2/18 (Sl. glasnik BiH 30/19, sluzbenilist.ba), and the Ministry of \
              Administration and Local Self-Government's January notices for 2017 to 2026 \
              as Novosti, Paragraf Lex, BL Portal, Glas Srpske (by the Internet Archive), \
              Mondo, RTRS, Srna and Tanjug report them; Brčko: Zakon o praznicima Brčko \
              distrikta BiH (Sl. glasnik BD 19/02 and 19/22) from Paragraf Lex, and the \
              Assembly's decisions on non-working days for 2017 to 2026 from its register \
              (skupstinabd.ba); RFE/RL, January 2021, for the absence of a state law; all \
              retrieved 2026-09-23",
};

// ─────────────────────────────────────────────────────────────────────────
// Belarus
// ─────────────────────────────────────────────────────────────────────────

static BY_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Новы год", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("New Year's Day", "Новы год", Rule::gregorian(1, 2))
        .years(Some(2020), None),
    HolidayRule::fixed_public(
        "Orthodox Christmas",
        "Каляды праваслаўныя",
        Rule::gregorian(1, 7),
    ),
    HolidayRule::fixed_public(
        "International Women's Day",
        "Міжнародны жаночы дзень",
        Rule::gregorian(3, 8),
    ),
    // The ninth day after Orthodox Easter.
    HolidayRule::fixed_public("Radunitsa", "Радаўніца", Rule::paschal(EASTER_SUNDAY + 9))
        .years(Some(1992), None),
    HolidayRule::fixed_public("Labour Day", "Дзень працы", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Victory Day", "Дзень Перамогі", Rule::gregorian(5, 9)),
    HolidayRule::fixed_public(
        "Independence Day",
        "Дзень Незалежнасці",
        Rule::gregorian(7, 27),
    )
    .years(Some(1991), Some(1996)),
    HolidayRule::fixed_public(
        "Independence Day",
        "Дзень Незалежнасці",
        Rule::gregorian(7, 3),
    )
    .years(Some(1997), None),
    HolidayRule::fixed_public(
        "October Revolution Day",
        "Дзень Кастрычніцкай рэвалюцыі",
        Rule::gregorian(11, 7),
    ),
    HolidayRule::fixed_public(
        "Catholic Christmas",
        "Каляды каталіцкія",
        Rule::gregorian(12, 25),
    ),
    // The state holidays that are working days.
    HolidayRule::observance(
        "Defender of the Fatherland and Armed Forces Day",
        "Дзень абаронцы Айчыны і Дзень Узброеных Сіл",
        Rule::gregorian(2, 23),
    ),
    HolidayRule::observance(
        "Constitution Day",
        "Дзень Канстытуцыі",
        Rule::gregorian(3, 15),
    )
    .years(Some(1994), None),
    HolidayRule::observance(
        "Day of Unity of the Peoples of Belarus and Russia",
        "Дзень яднання народаў Беларусі і Расіі",
        Rule::gregorian(4, 2),
    ),
    HolidayRule::observance(
        "State Flag and State Emblem Day",
        "Дзень Дзяржаўнага Сцяга і Дзяржаўнага Герба",
        Rule::nth(5, 2, Weekday::Sunday),
    ),
    HolidayRule::observance(
        "National Unity Day",
        "Дзень народнага адзінства",
        Rule::gregorian(9, 17),
    )
    .years(Some(2021), None),
];

/// Belarus.
///
/// Presidential Decree 157 of 26 March 1998 as amended: the ten
/// non-working days — New Year's second day from 2020, Radunitsa on the
/// ninth day after the Orthodox Easter and non-working since 1992,
/// Independence Day on 3 July from 1997 after 27 July from 1991 to 1996 —
/// and the state holidays that are working days as observances. No fixed
/// rule moves a holiday off a weekend; the Government swaps working days
/// around holidays by yearly decree, which is not carried.
pub static BELARUS: RuleSet = RuleSet {
    code: "BY",
    english_name: "Belarus",
    rules: BY_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "The President of the Republic of Belarus, \"Государственные праздники \
              Беларуси\", president.gov.by, retrieved 2026-09-22, for Decree 157's \
              list; the Russian Wikipedia, \"Праздники Белоруссии\", retrieved the \
              same day, for the years of 2 January, Radunitsa and Independence Day \
              and the Radunitsa dates; Wikipedia, \"Public holidays in Belarus\", \
              for the Belarusian names",
};

// ─────────────────────────────────────────────────────────────────────────
// Luxembourg
// ─────────────────────────────────────────────────────────────────────────

static LU_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Neijoerschdag", Rule::gregorian(1, 1)),
    // A bank holiday alone.
    HolidayRule::fixed_public("Good Friday", "Karfreideg", Rule::easter(GOOD_FRIDAY))
        .of_kind(Kind::Bank),
    HolidayRule::fixed_public(
        "Easter Monday",
        "Ouschterméindeg",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::fixed_public("Labour Day", "Dag vun der Aarbecht", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Europe Day", "Europadag", Rule::gregorian(5, 9))
        .years(Some(2019), None),
    HolidayRule::fixed_public(
        "Ascension Day",
        "Christi Himmelfaart",
        Rule::easter(ASCENSION),
    ),
    HolidayRule::fixed_public("Whit Monday", "Péngschtméindeg", Rule::easter(WHIT_MONDAY)),
    HolidayRule::fixed_public("National Day", "Nationalfeierdag", Rule::gregorian(6, 23)),
    HolidayRule::fixed_public(
        "Assumption Day",
        "Mariä Himmelfaart",
        Rule::gregorian(8, 15),
    ),
    HolidayRule::fixed_public("All Saints' Day", "Allerhellgen", Rule::gregorian(11, 1)),
    HolidayRule::fixed_public("Christmas Day", "Chrëschtdag", Rule::gregorian(12, 25)),
    HolidayRule::fixed_public("Saint Stephen's Day", "Stiefesdag", Rule::gregorian(12, 26)),
];

/// Luxembourg.
///
/// The eleven legal holidays of article L. 232-2 of the Code du travail,
/// Europe Day among them from the law of 25 April 2019, and Good Friday as
/// the banks' holiday alone, [`Kind::Bank`]. A legal holiday on a Sunday
/// or a non-working day gives a compensatory day to be taken within three
/// months, at no fixed date, and is not carried; nor are the civil
/// service's half days on Whit Tuesday and Christmas Eve, nor the local
/// days of the capital.
pub static LUXEMBOURG: RuleSet = RuleSet {
    code: "LU",
    english_name: "Luxembourg",
    rules: LU_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Inspection du travail et des mines, \"Jours fériés légaux\", itm.public.lu, \
              retrieved 2026-09-22, for article L. 232-2 and the law of 25 April \
              2019; the French Wikipedia, \"Jours fériés au Luxembourg\", and \
              Wikipedia, \"Public holidays in Luxembourg\", both retrieved the same \
              day, for the names, Good Friday and the compensatory day",
};

// ─────────────────────────────────────────────────────────────────────────
// Malta
// ─────────────────────────────────────────────────────────────────────────

static MT_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "L-Ewwel tas-Sena", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Feast of Saint Paul's Shipwreck",
        "Nawfraġju ta' San Pawl",
        Rule::gregorian(2, 10),
    ),
    HolidayRule::fixed_public(
        "Feast of Saint Joseph",
        "San Ġużepp",
        Rule::gregorian(3, 19),
    ),
    HolidayRule::fixed_public("Freedom Day", "Jum il-Ħelsien", Rule::gregorian(3, 31)),
    HolidayRule::fixed_public(
        "Good Friday",
        "Il-Ġimgħa l-Kbira",
        Rule::easter(GOOD_FRIDAY),
    ),
    HolidayRule::fixed_public("Workers' Day", "Jum il-Ħaddiem", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Sette Giugno", "Sette Giugno", Rule::gregorian(6, 7)),
    HolidayRule::fixed_public(
        "Feast of Saint Peter and Saint Paul",
        "L-Imnarja",
        Rule::gregorian(6, 29),
    ),
    HolidayRule::fixed_public(
        "Feast of the Assumption",
        "Santa Marija",
        Rule::gregorian(8, 15),
    ),
    HolidayRule::fixed_public("Victory Day", "Jum il-Vitorja", Rule::gregorian(9, 8)),
    HolidayRule::fixed_public(
        "Independence Day",
        "Jum l-Indipendenza",
        Rule::gregorian(9, 21),
    ),
    HolidayRule::fixed_public(
        "Feast of the Immaculate Conception",
        "Il-Kunċizzjoni",
        Rule::gregorian(12, 8),
    ),
    HolidayRule::fixed_public("Republic Day", "Jum ir-Repubblika", Rule::gregorian(12, 13)),
    HolidayRule::fixed_public("Christmas Day", "Il-Milied", Rule::gregorian(12, 25)),
];

/// Malta.
///
/// The National Holidays and Other Public Holidays Act (Cap. 252) of
/// 1975 as amended: the five national holidays and the nine public
/// holidays, all days off and carried alike. A holiday on a weekend stays
/// there; what the Act does with it is a matter of leave, not of the
/// calendar. The years the Act added and restored feasts are not carried.
pub static MALTA: RuleSet = RuleSet {
    code: "MT",
    english_name: "Malta",
    rules: MT_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "National Holidays and Other Public Holidays Act, Cap. 252, legislation.mt, \
              retrieved 2026-09-22, for the Act and its amendments; Wikipedia, \
              \"Public holidays in Malta\", retrieved the same day, for the two lists \
              and the Maltese names",
};

// ─────────────────────────────────────────────────────────────────────────
// Moldova
// ─────────────────────────────────────────────────────────────────────────

static MD_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Anul Nou", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Orthodox Christmas",
        "Nașterea lui Isus Hristos (Crăciunul pe stil vechi)",
        Rule::gregorian(1, 7),
    ),
    HolidayRule::fixed_public(
        "Orthodox Christmas",
        "Nașterea lui Isus Hristos (Crăciunul pe stil vechi)",
        Rule::gregorian(1, 8),
    ),
    HolidayRule::fixed_public(
        "International Women's Day",
        "Ziua Internațională a Femeii",
        Rule::gregorian(3, 8),
    ),
    HolidayRule::fixed_public("Easter Sunday", "Paștele", Rule::paschal(EASTER_SUNDAY)),
    HolidayRule::fixed_public("Easter Monday", "Paștele", Rule::paschal(EASTER_MONDAY)),
    // The Monday a week after Easter.
    HolidayRule::fixed_public(
        "Easter of the Blajini",
        "Paștele Blajinilor",
        Rule::paschal(EASTER_SUNDAY + 8),
    ),
    HolidayRule::fixed_public(
        "Labour Day",
        "Ziua internațională a solidarității oamenilor muncii",
        Rule::gregorian(5, 1),
    ),
    HolidayRule::fixed_public(
        "Victory Day",
        "Ziua Victoriei și a comemorării eroilor căzuți pentru independența Patriei",
        Rule::gregorian(5, 9),
    ),
    HolidayRule::fixed_public("Europe Day", "Ziua Europei", Rule::gregorian(5, 9))
        .years(Some(2017), None),
    HolidayRule::fixed_public(
        "Children's Day",
        "Ziua Internațională a Copilului",
        Rule::gregorian(6, 1),
    )
    .years(Some(2024), None),
    HolidayRule::fixed_public(
        "Independence Day",
        "Ziua Independenței",
        Rule::gregorian(8, 27),
    ),
    HolidayRule::fixed_public(
        "Romanian Language Day",
        "Ziua Limbii Române",
        Rule::gregorian(8, 31),
    ),
    HolidayRule::fixed_public(
        "Feast of Chișinău",
        "Hramul Chișinăului",
        Rule::gregorian(10, 14),
    )
    .in_regions(&["MD-CU"]),
    HolidayRule::fixed_public(
        "Christmas Day",
        "Nașterea lui Isus Hristos (Crăciunul pe stil nou)",
        Rule::gregorian(12, 25),
    )
    .years(Some(2009), None),
];

/// Moldova.
///
/// Article 111 of the Codul muncii: the non-working holidays, with the
/// Orthodox Christmas on two days, the Easter by the Julian computus on
/// its Sunday and Monday and the Easter of the Blajini on the Monday a
/// week after, Europe Day sharing 9 May with Victory Day from 2017,
/// Children's Day from 2024, Christmas by the new style from 2009, and the
/// feast of Chișinău as the capital's alone, `MD-CU`. A holiday on a
/// weekend is not moved by law; the Government's yearly transfers of
/// working days, and the patron-saint day each locality may keep, are not
/// carried.
pub static MOLDOVA: RuleSet = RuleSet {
    code: "MD",
    english_name: "Moldova",
    rules: MD_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Codul muncii al Republicii Moldova, art. 111, as listed by zilelibere.md, \
              retrieved 2026-09-22; contabilitate.md for 1 June from 2024; Radio \
              Moldova and Timpul for Europe Day from 2017; Wikipedia, \"Public \
              holidays in Moldova\", retrieved the same day, for Christmas by the new \
              style from 2009 and the English names",
};

// ─────────────────────────────────────────────────────────────────────────
// Liechtenstein
// ─────────────────────────────────────────────────────────────────────────

/// A day the banks close on that the Labour Act does not name.
const fn li_bank(name: &'static str, local: &'static str, rule: Rule) -> HolidayRule {
    HolidayRule::fixed_public(name, local, rule).of_kind(Kind::Bank)
}

static LI_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Neujahr", Rule::gregorian(1, 1)),
    li_bank("Berchtold's Day", "Berchtoldstag", Rule::gregorian(1, 2)),
    HolidayRule::fixed_public("Epiphany", "Heilige Drei Könige", Rule::gregorian(1, 6)),
    HolidayRule::observance("Candlemas", "Lichtmess", Rule::gregorian(2, 2)),
    li_bank(
        "Shrove Tuesday",
        "Fasnachtsdienstag",
        Rule::easter(SHROVE_TUESDAY),
    ),
    HolidayRule::observance("Saint Joseph's Day", "Josefstag", Rule::gregorian(3, 19)),
    li_bank("Good Friday", "Karfreitag", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Monday", "Ostermontag", Rule::easter(EASTER_MONDAY)),
    HolidayRule::fixed_public("Labour Day", "Tag der Arbeit", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Ascension", "Auffahrt", Rule::easter(ASCENSION)),
    HolidayRule::fixed_public("Whit Monday", "Pfingstmontag", Rule::easter(WHIT_MONDAY)),
    HolidayRule::fixed_public(
        "Corpus Christi",
        "Fronleichnam",
        Rule::easter(CORPUS_CHRISTI),
    ),
    HolidayRule::fixed_public("National Day", "Staatsfeiertag", Rule::gregorian(8, 15)),
    HolidayRule::fixed_public("Nativity of Mary", "Mariä Geburt", Rule::gregorian(9, 8)),
    HolidayRule::fixed_public("All Saints' Day", "Allerheiligen", Rule::gregorian(11, 1)),
    HolidayRule::fixed_public(
        "Immaculate Conception",
        "Mariä Empfängnis",
        Rule::gregorian(12, 8),
    ),
    li_bank("Christmas Eve", "Heiligabend", Rule::gregorian(12, 24)),
    HolidayRule::fixed_public("Christmas Day", "Weihnachten", Rule::gregorian(12, 25)),
    HolidayRule::fixed_public(
        "Saint Stephen's Day",
        "Stephanstag",
        Rule::gregorian(12, 26),
    ),
    li_bank("New Year's Eve", "Silvester", Rule::gregorian(12, 31)),
];

/// Liechtenstein.
///
/// The Labour Act (Gesetz über die Arbeit in Industrie, Gewerbe und
/// Handel), article 18(2) as amended in 1986: thirteen "legal holidays
/// equal to Sundays" — New Year's Day, Epiphany, Easter Monday, 1 May,
/// Ascension, Whit Monday, Corpus Christi, the Assumption as the National
/// Day, the Nativity of Mary, All Saints, the Immaculate Conception,
/// Christmas and St. Stephen's Day. The five days on which the banks
/// also close, Berchtold's Day, Shrove Tuesday, Good Friday, Christmas
/// Eve and New Year's Eve, are bank days here, and Candlemas and Saint
/// Joseph's Day, which collective agreements make paid days in some
/// trades, are observances. A holiday on a Sunday is a Sunday; nothing
/// moves.
pub static LIECHTENSTEIN: RuleSet = RuleSet {
    code: "LI",
    english_name: "Liechtenstein",
    rules: LI_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Gesetz über die Arbeit in Industrie, Gewerbe und Handel (Arbeitsgesetz), \
              LGBl. 1967 Nr. 6, article 18(2) as amended by LGBl. 1986 Nr. 85, from \
              gesetze.li (konso/1967.006), retrieved 2026-09-22; Wikipedia (de), \
              \"Feiertage in Liechtenstein\", for the bank days and the two \
              collective-agreement days, and Wikipedia, \"Public holidays in \
              Liechtenstein\", for the English names",
};

// ─────────────────────────────────────────────────────────────────────────
// Monaco
// ─────────────────────────────────────────────────────────────────────────

/// Article 1 of law 798: when six of the twelve days "fall on a Sunday,
/// the Monday that follows is a legal holiday".
static MC_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: false,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static MC_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Jour de l'an", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Saint Devota's Day",
        "Sainte-Dévote",
        Rule::gregorian(1, 27),
    ),
    HolidayRule::fixed_public(
        "Easter Monday",
        "Lundi de Pâques",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::public("Labour Day", "Fête du Travail", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Ascension", "Ascension", Rule::easter(ASCENSION)),
    HolidayRule::fixed_public(
        "Whit Monday",
        "Lundi de Pentecôte",
        Rule::easter(WHIT_MONDAY),
    ),
    HolidayRule::fixed_public("Corpus Christi", "Fête-Dieu", Rule::easter(CORPUS_CHRISTI)),
    HolidayRule::public("Assumption", "Assomption", Rule::gregorian(8, 15)),
    HolidayRule::public("All Saints' Day", "Toussaint", Rule::gregorian(11, 1)),
    HolidayRule::public(
        "Sovereign Prince's Day",
        "Fête du Prince",
        Rule::gregorian(11, 19),
    )
    .years(Some(1952), None),
    HolidayRule::fixed_public(
        "Immaculate Conception",
        "Immaculée Conception",
        Rule::gregorian(12, 8),
    ),
    HolidayRule::public("Christmas Day", "Noël", Rule::gregorian(12, 25)),
];

/// Monaco.
///
/// Law 798 of 18 February 1966 fixing the legal holidays, article 1, from
/// Legimonaco: the Sovereign Prince's Day, New Year's Day, Saint Devota's
/// Day, Easter Monday, 1 May, Ascension, Whit Monday, Corpus Christi, the
/// Assumption, All Saints, the Immaculate Conception and Christmas; and
/// "when the Sovereign Prince's Day, New Year's Day, 1 May, the
/// Assumption, All Saints and Christmas fall on a Sunday, the Monday that
/// follows is a legal holiday", which those six rules do and the other
/// six do not. The Prince's Day is the day the reigning Prince chooses,
/// 19 November since 1952 under Rainier III and kept by Albert II. The
/// days the Prince declares for an occasion are not carried.
pub static MONACO: RuleSet = RuleSet {
    code: "MC",
    english_name: "Monaco",
    rules: MC_RULES,
    substitution: MC_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Loi n° 798 du 18 février 1966 portant fixation des jours fériés légaux, \
              article 1, as Legimonaco publishes it, retrieved 2026-09-22; the Prince's \
              Government (gouv.mc) and the Prince's Palace on the Fête du Prince of \
              19 November since 1952; Wikipedia, \"Public holidays in Monaco\", for the \
              English names",
};

// ─────────────────────────────────────────────────────────────────────────
// San Marino
// ─────────────────────────────────────────────────────────────────────────

static SM_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Capodanno", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("Epiphany", "Epifania", Rule::gregorian(1, 6)),
    HolidayRule::fixed_public("Feast of Saint Agatha", "Sant'Agata", Rule::gregorian(2, 5)),
    HolidayRule::fixed_public(
        "Anniversary of the Arengo",
        "Anniversario dell'Arengo",
        Rule::gregorian(3, 25),
    ),
    HolidayRule::fixed_public(
        "Investiture of the Captains Regent",
        "Ingresso dei Capitani Reggenti",
        Rule::gregorian(4, 1),
    ),
    HolidayRule::fixed_public("Easter Sunday", "Pasqua", Rule::easter(EASTER_SUNDAY)),
    HolidayRule::fixed_public(
        "Easter Monday",
        "Lunedì dell'Angelo",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::fixed_public("Labour Day", "Festa del Lavoro", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public(
        "Corpus Christi",
        "Corpus Domini",
        Rule::easter(CORPUS_CHRISTI),
    ),
    HolidayRule::fixed_public(
        "Anniversary of the Fall of Fascism",
        "Anniversario della Caduta del Fascismo e Festa della Libertà",
        Rule::gregorian(7, 28),
    ),
    HolidayRule::fixed_public("Assumption", "Assunzione", Rule::gregorian(8, 15)),
    HolidayRule::fixed_public(
        "Feast of Saint Marinus and the Republic",
        "Festa di San Marino e di Fondazione della Repubblica",
        Rule::gregorian(9, 3),
    ),
    HolidayRule::fixed_public(
        "Investiture of the Captains Regent",
        "Ingresso dei Capitani Reggenti",
        Rule::gregorian(10, 1),
    ),
    HolidayRule::fixed_public("All Saints' Day", "Ognissanti", Rule::gregorian(11, 1)),
    HolidayRule::fixed_public(
        "All Souls' Day",
        "Commemorazione dei Defunti",
        Rule::gregorian(11, 2),
    ),
    HolidayRule::fixed_public(
        "Immaculate Conception",
        "Immacolata Concezione",
        Rule::gregorian(12, 8),
    ),
    HolidayRule::fixed_public(
        "Christmas Eve",
        "Vigilia di Natale",
        Rule::gregorian(12, 24),
    )
    .of_kind(Kind::Bank),
    HolidayRule::fixed_public("Christmas Day", "Natale", Rule::gregorian(12, 25)),
    HolidayRule::fixed_public(
        "Saint Stephen's Day",
        "Santo Stefano",
        Rule::gregorian(12, 26),
    ),
    HolidayRule::fixed_public("New Year's Eve", "San Silvestro", Rule::gregorian(12, 31))
        .of_kind(Kind::Bank),
];

/// San Marino.
///
/// Law 152 of 18 December 1990, the Calendar of Festivities, as its
/// article 2 was rewritten by law 152 of 30 October 2013: the civil
/// holidays Easter Monday and 26 December, and the civil and national
/// ones of 25 March, 1 April and 1 October for the Captains Regent,
/// 1 May and 28 July. Article 1's religious days were not read; they are
/// carried as the Central Bank's calendars of the days the national
/// payments system closes give them, New Year's Day, Epiphany, Saint
/// Agatha, Easter, Corpus Christi, the Assumption, Saint Marinus, All
/// Saints, All Souls, the Immaculate Conception and Christmas. The same
/// calendars close 24 and 31 December, which the 1990 law dropped from
/// the holidays; those two are bank days here. Nothing moves off a
/// Sunday.
pub static SAN_MARINO: RuleSet = RuleSet {
    code: "SM",
    english_name: "San Marino",
    rules: SM_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Legge 30 ottobre 2013 n. 152 replacing article 2 of Legge 18 dicembre 1990 \
              n. 152, Calendario delle Festività, from the Consiglio Grande e Generale's \
              archive, retrieved 2026-09-22; the Central Bank of San Marino's bank-holiday \
              calendars for 2025 and 2026 (bcsm.sm); San Marino RTV (4 June 2026) on the \
              days the 1990 law dropped; Wikipedia, \"Public holidays in San Marino\", \
              for the English names",
};

// ─────────────────────────────────────────────────────────────────────────
// Vatican City
// ─────────────────────────────────────────────────────────────────────────

/// The reigning Pope's election anniversary: Francis, elected on 13 March
/// 2013, from 2014 to 2025; Leo XIV, elected on 8 May 2025, in 2026. There
/// was none in 2013.
fn va_election_anniversary(year: i64) -> Days {
    let (month, day) = match year {
        2014..=2025 => (3, 13),
        2026 => (5, 8),
        _ => return Days::new(),
    };
    gregorian::to_fixed(year, month, day).map_or_else(|_| Days::new(), Days::one)
}

/// The reigning Pope's name day: Francis's, Jorge's, on Saint George's day,
/// 23 April, from 2013 to 2024; Leo XIV's, Robert's, on Saint Robert
/// Bellarmine's, 17 September, in 2025 and 2026. On 23 April 2025 the See
/// was vacant.
fn va_name_day(year: i64) -> Days {
    let (month, day) = match year {
        2013..=2024 => (4, 23),
        2025..=2026 => (9, 17),
        _ => return Days::new(),
    };
    gregorian::to_fixed(year, month, day).map_or_else(|_| Days::new(), Days::one)
}

/// A day that belongs to a pontificate, known for the pontificates that
/// have begun and a gap for the years after 2026.
const fn va_pontifical(
    name: &'static str,
    local_name: &'static str,
    function: fn(i64) -> Days,
) -> HolidayRule {
    HolidayRule::fixed_public(
        name,
        local_name,
        Rule::Tabulated {
            function,
            first_year: 2013,
            last_year: 2026,
        },
    )
}

static VA_RULES: &[HolidayRule] = &[
    // The holy days of obligation of canon 1246, the first of the two
    // lists; each falls on its day in the General Roman Calendar.
    HolidayRule::fixed_public(
        "Solemnity of Mary, Mother of God",
        "Maria Santissima Madre di Dio",
        Rule::gregorian(1, 1),
    ),
    HolidayRule::fixed_public("Epiphany", "Epifania del Signore", Rule::gregorian(1, 6)),
    HolidayRule::fixed_public("Saint Joseph", "San Giuseppe", Rule::gregorian(3, 19)),
    HolidayRule::fixed_public(
        "Ascension",
        "Ascensione del Signore",
        Rule::easter(ASCENSION),
    ),
    HolidayRule::fixed_public(
        "Corpus Christi",
        "Santissimo Corpo e Sangue di Cristo",
        Rule::easter(CORPUS_CHRISTI),
    ),
    HolidayRule::fixed_public(
        "Saints Peter and Paul",
        "Santi Pietro e Paolo",
        Rule::gregorian(6, 29),
    ),
    HolidayRule::fixed_public(
        "Assumption",
        "Assunzione della Beata Vergine Maria",
        Rule::gregorian(8, 15),
    ),
    HolidayRule::fixed_public("All Saints' Day", "Tutti i Santi", Rule::gregorian(11, 1)),
    HolidayRule::fixed_public(
        "Immaculate Conception",
        "Immacolata Concezione",
        Rule::gregorian(12, 8),
    ),
    HolidayRule::fixed_public(
        "Christmas Day",
        "Natale del Signore",
        Rule::gregorian(12, 25),
    ),
    // The regulations' own list.
    va_pontifical(
        "Anniversary of the Pope's Election",
        "Anniversario dell'elezione del Sommo Pontefice",
        va_election_anniversary,
    ),
    va_pontifical(
        "Pope's Name Day",
        "Onomastico del Sommo Pontefice",
        va_name_day,
    ),
    HolidayRule::fixed_public(
        "Anniversary of the Establishment of Vatican City State",
        "Anniversario dell'istituzione dello Stato della Città del Vaticano",
        Rule::gregorian(2, 11),
    ),
    HolidayRule::fixed_public(
        "Saint Joseph the Worker",
        "San Giuseppe Artigiano",
        Rule::gregorian(5, 1),
    ),
    HolidayRule::fixed_public(
        "Maundy Thursday",
        "Giovedì Santo",
        Rule::easter(MAUNDY_THURSDAY),
    ),
    HolidayRule::fixed_public("Good Friday", "Venerdì Santo", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Holy Saturday", "Sabato Santo", Rule::easter(HOLY_SATURDAY)),
    HolidayRule::fixed_public("Easter Sunday", "Pasqua", Rule::easter(EASTER_SUNDAY)),
    HolidayRule::fixed_public(
        "Easter Monday",
        "Lunedì di Pasqua",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::fixed_public(
        "Easter Tuesday",
        "Martedì di Pasqua",
        Rule::easter(EASTER_MONDAY + 1),
    ),
    HolidayRule::fixed_public(
        "Eve of the Assumption",
        "Vigilia dell'Assunzione",
        Rule::gregorian(8, 14),
    ),
    HolidayRule::fixed_public(
        "Day after the Assumption",
        "Giorno successivo all'Assunzione",
        Rule::gregorian(8, 16),
    ),
    HolidayRule::fixed_public(
        "All Souls' Day",
        "Commemorazione di tutti i fedeli defunti",
        Rule::gregorian(11, 2),
    ),
    HolidayRule::fixed_public(
        "Christmas Eve",
        "Vigilia di Natale",
        Rule::gregorian(12, 24),
    ),
    HolidayRule::fixed_public(
        "Saint Stephen's Day",
        "Santo Stefano",
        Rule::gregorian(12, 26),
    ),
    HolidayRule::fixed_public(
        "Saint John the Apostle",
        "San Giovanni Apostolo ed Evangelista",
        Rule::gregorian(12, 27),
    ),
    HolidayRule::fixed_public(
        "Last Day of the Year",
        "Ultimo giorno dell'anno",
        Rule::gregorian(12, 31),
    ),
];

/// Vatican City.
///
/// The Governorate's General Regulation for its staff, in force from 2011,
/// article 27, and the Holy See's for the Roman Curia — article 50 of the
/// General Regulation of 1999 and article 52 of the Regulation for the
/// Personnel in force from 2026 — give the same list: besides Sundays and
/// the other holy days of obligation ("according to canon 1246 of the Code
/// of Canon Law", the Governorate's adds), the days off are the anniversary
/// of the Pope's election, his name day, the anniversary of the State's
/// establishment on 11 February, Saint Joseph the Worker, the last three
/// days of Holy Week, Easter Monday and Tuesday, the eve and the day after
/// the Assumption, All Souls, the eve and the two days after Christmas, and
/// the last day of the year. Canon 1246's ten days are carried on the days
/// the General Roman Calendar ([`crate::roman_calendar`]) keeps them, the
/// Ascension and Corpus Christi on their Thursdays, as the Fabbrica di San
/// Pietro's excavations office listed its closures for 2014, 2022 and 2023.
/// Easter Sunday is carried with its week.
///
/// The Pope's two days are tabulated for the pontificates of Francis and
/// Leo XIV, from 2013 to 2026, and are a gap after, when the reigning Pope
/// is not a fact yet. The weekly rest is Sunday, as both regulations state,
/// and Saturday is a working day. Nothing moves off a Sunday, and a
/// celebration the liturgy transfers — Saint Joseph in Holy Week, the
/// Immaculate Conception on a Sunday of Advent — is not moved here, for no
/// source says the day off follows it.
pub static VATICAN_CITY: RuleSet = RuleSet {
    code: "VA",
    english_name: "Vatican City",
    rules: VA_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: VA_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Regolamento generale per il personale del Governatorato dello Stato della Città \
              del Vaticano (21 November 2010), articles 26 and 27, from vatican.va and from the \
              ULSA's current copy by the Internet Archive (11 May 2026); Regolamento del \
              Personale della Curia Romana (23 November 2025), articles 49 and 52, and \
              Regolamento generale della Curia Romana (30 April 1999), article 50, from \
              vatican.va; the Code of Canon Law, canon 1246, from vatican.va; the Ufficio \
              Scavi's closure days for 2014, 2022 and 2023 (scavi.va, by the Internet \
              Archive); AgenSIR, 17 September 2025, for Leo XIV's name day; all retrieved \
              2026-09-23",
};

/// Sunday alone: the regulations' "day of weekly rest, which coincides
/// with Sunday".
static VA_WEEKEND: &[WeekendPolicy] = &[WeekendPolicy {
    days: &[Weekday::Sunday],
    valid_from: None,
    valid_until: None,
}];

// ─────────────────────────────────────────────────────────────────────────
// Andorra
// ─────────────────────────────────────────────────────────────────────────

static AD_RULES: &[HolidayRule] = &[
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
    ),
];

/// Andorra.
///
/// Law 31/2018 on labour relations, article 62, gives the right to the
/// holidays "of the work calendar", which the Government decrees each
/// year; the table carries the fourteen national days of the 2026
/// calendar (Decree 340/2025), the same the calendars of 2024 and 2025
/// carried: New Year's Day, Epiphany, Carnival on the Monday before
/// Lent, Constitution Day, Good Friday, Easter Monday, 1 May, Whit
/// Monday, the Assumption, Our Lady of Meritxell, All Saints, the
/// Immaculate Conception, Christmas and Saint Stephen. The up to four
/// days each parish adds, and the tourism sector's leave to move all but
/// four of the days by agreement, are not carried. Nothing moves off a
/// Sunday.
pub static ANDORRA: RuleSet = RuleSet {
    code: "AD",
    english_name: "Andorra",
    rules: AD_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Llei 31/2018, del 6 de desembre, de relacions laborals, article 62, from the \
              Cambra de Comerç's copy (ccis.ad), retrieved 2026-09-22; the Government's \
              notice of Decret 340/2025 approving the 2026 work calendar (govern.ad) and \
              La Vall Associats' reproduction of its list; Wikipedia, \"2024 in \
              Andorra\", \"2025 in Andorra\" and \"2026 in Andorra\", for the yearly \
              dates, and \"Public holidays in Andorra\" for the names",
};
