//! European national tables.
//!
//! Most of Europe is the easy case for this crate's design: a handful of
//! fixed Gregorian dates plus the Easter cycle, which is why the Easter
//! offsets live in [`crate::computus::offsets`] and no country repeats them.
//! Greece is the one that pays for the second computus.

use hc_calendar::Weekday;

use crate::computus::offsets::{
    ASCENSION, ASH_WEDNESDAY, CORPUS_CHRISTI, EASTER_MONDAY, EASTER_SUNDAY, GOOD_FRIDAY,
    MAUNDY_THURSDAY, PENTECOST, SHROVE_TUESDAY, WHIT_MONDAY,
};
use crate::rule::{
    Days, HolidayRule, Kind, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate, SubstituteDirection,
    SubstitutionPolicy,
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
    HolidayRule::public("Early May Bank Holiday", "", Rule::nth(5, 1, Weekday::Monday))
        .years(Some(1978), Some(1994)),
    HolidayRule::public("Early May Bank Holiday", "", Rule::gregorian(5, 8))
        .years(Some(1995), Some(1995)),
    HolidayRule::public("Early May Bank Holiday", "", Rule::nth(5, 1, Weekday::Monday))
        .years(Some(1996), Some(2019)),
    HolidayRule::public("Early May Bank Holiday", "", Rule::gregorian(5, 8))
        .years(Some(2020), Some(2020)),
    HolidayRule::public("Early May Bank Holiday", "", Rule::nth(5, 1, Weekday::Monday))
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
    HolidayRule::fixed_public(
        "Wedding of the Prince of Wales",
        "",
        Rule::gregorian(7, 29),
    )
    .years(Some(1981), Some(1981)),
    HolidayRule::fixed_public("Millennium Eve", "", Rule::gregorian(12, 31))
        .years(Some(1999), Some(1999)),
    HolidayRule::fixed_public("Golden Jubilee of Elizabeth II", "", Rule::gregorian(6, 3))
        .years(Some(2002), Some(2002)),
    HolidayRule::fixed_public("Wedding of Prince William", "", Rule::gregorian(4, 29))
        .years(Some(2011), Some(2011)),
    HolidayRule::fixed_public("Diamond Jubilee of Elizabeth II", "", Rule::gregorian(6, 5))
        .years(Some(2012), Some(2012)),
    HolidayRule::fixed_public("Platinum Jubilee of Elizabeth II", "", Rule::gregorian(6, 3))
        .years(Some(2022), Some(2022)),
    HolidayRule::fixed_public(
        "State Funeral of Elizabeth II",
        "",
        Rule::gregorian(9, 19),
    )
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
/// one of the genuine handful that uses [`Rule::Computed`].
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
    HolidayRule::public("St Brigid's Day", "Lá Fhéile Bríde", Rule::Computed(st_brigids_day))
        .years(Some(2023), None),
    HolidayRule::public("St Patrick's Day", "Lá Fhéile Pádraig", Rule::gregorian(3, 17)),
    HolidayRule::public("Easter Monday", "Luan Cásca", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("May Day", "Lá Bealtaine", Rule::nth(5, 1, Weekday::Monday))
        .years(Some(1994), None),
    HolidayRule::public("June Bank Holiday", "", Rule::nth(6, 1, Weekday::Monday)),
    HolidayRule::public("August Bank Holiday", "", Rule::nth(8, 1, Weekday::Monday)),
    HolidayRule::public("October Bank Holiday", "", Rule::last(10, Weekday::Monday)),
    HolidayRule::public("Christmas Day", "Lá Nollag", Rule::gregorian(12, 25)),
    HolidayRule::public("St Stephen's Day", "Lá Fhéile Stiofáin", Rule::gregorian(12, 26)),
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
    HolidayRule::public("Easter Monday", "Lundi de Pâques", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("Labour Day", "Fête du Travail", Rule::gregorian(5, 1))
        .years(Some(1948), None),
    // 8 May was a holiday from 1953, abolished by décret in 1959 and
    // restored by the loi du 2 octobre 1981, first kept again in 1982.
    HolidayRule::public("Victory in Europe Day", "Victoire 1945", Rule::gregorian(5, 8))
        .years(Some(1953), Some(1958)),
    HolidayRule::public("Victory in Europe Day", "Victoire 1945", Rule::gregorian(5, 8))
        .years(Some(1982), None),
    HolidayRule::public("Ascension", "Ascension", Rule::easter(ASCENSION)),
    HolidayRule::public("Whit Monday", "Lundi de Pentecôte", Rule::easter(WHIT_MONDAY)),
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
    HolidayRule::public("International Women's Day", "Internationaler Frauentag", Rule::gregorian(3, 8))
        .in_regions(DE_BERLIN)
        .years(Some(2019), None),
    HolidayRule::public("International Women's Day", "Internationaler Frauentag", Rule::gregorian(3, 8))
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
    HolidayRule::public("Corpus Christi", "Fronleichnam", Rule::easter(CORPUS_CHRISTI))
        .in_regions(DE_CORPUS_CHRISTI),
    HolidayRule::public("Assumption", "Mariä Himmelfahrt", Rule::gregorian(8, 15))
        .in_regions(DE_SAARLAND),
    HolidayRule::public("World Children's Day", "Weltkindertag", Rule::gregorian(9, 20))
        .in_regions(DE_THURINGIA)
        .years(Some(2019), None),
    HolidayRule::public("German Unity Day", "Tag der Deutschen Einheit", Rule::gregorian(10, 3))
        .years(Some(1990), None),
    HolidayRule::public("Reformation Day", "Reformationstag", Rule::gregorian(10, 31))
        .in_regions(DE_REFORMATION_EAST)
        .years(Some(1990), None),
    HolidayRule::public("Reformation Day", "Reformationstag", Rule::gregorian(10, 31))
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
    HolidayRule::public("Christmas Day", "Erster Weihnachtstag", Rule::gregorian(12, 25)),
    HolidayRule::public("St Stephen's Day", "Zweiter Weihnachtstag", Rule::gregorian(12, 26)),
];

/// Germany, with all sixteen *Länder* as regions.
pub static GERMANY: RuleSet = RuleSet {
    code: "DE",
    english_name: "Germany",
    rules: DE_RULES,
    substitution: &[],
    bridges: &[],
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
    HolidayRule::public("Easter Monday", "Lunedì dell'Angelo", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("Liberation Day", "Festa della Liberazione", Rule::gregorian(4, 25))
        .years(Some(1946), None),
    HolidayRule::public("Labour Day", "Festa del Lavoro", Rule::gregorian(5, 1)),
    // Republic Day was a working day from 1977 to 2000, kept on the first
    // Sunday of June instead; law 336/2000 put it back on 2 June.
    HolidayRule::public("Republic Day", "Festa della Repubblica", Rule::gregorian(6, 2))
        .years(Some(1949), Some(1976)),
    HolidayRule::public("Republic Day", "Festa della Repubblica", Rule::gregorian(6, 2))
        .years(Some(2001), None),
    HolidayRule::public("Assumption", "Ferragosto", Rule::gregorian(8, 15)),
    HolidayRule::public("All Saints' Day", "Ognissanti", Rule::gregorian(11, 1)),
    HolidayRule::public("Immaculate Conception", "Immacolata Concezione", Rule::gregorian(12, 8)),
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
    HolidayRule::public("Assumption", "Asunción de la Virgen", Rule::gregorian(8, 15)),
    HolidayRule::public("National Day", "Fiesta Nacional de España", Rule::gregorian(10, 12)),
    HolidayRule::public("All Saints' Day", "Todos los Santos", Rule::gregorian(11, 1)),
    HolidayRule::public("Constitution Day", "Día de la Constitución", Rule::gregorian(12, 6))
        .years(Some(1983), None),
    HolidayRule::public("Immaculate Conception", "Inmaculada Concepción", Rule::gregorian(12, 8)),
    HolidayRule::public("Christmas Day", "Natividad del Señor", Rule::gregorian(12, 25)),
];

/// Spain, national calendar only.
pub static SPAIN: RuleSet = RuleSet {
    code: "ES",
    english_name: "Spain",
    rules: ES_RULES,
    substitution: &[],
    bridges: &[],
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
    HolidayRule::public("Good Friday", "Sexta-feira Santa", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public("Easter Sunday", "Páscoa", Rule::easter(EASTER_SUNDAY)),
    HolidayRule::public("Freedom Day", "Dia da Liberdade", Rule::gregorian(4, 25))
        .years(Some(1974), None),
    HolidayRule::public("Labour Day", "Dia do Trabalhador", Rule::gregorian(5, 1)),
    // Austerity suspended four holidays from 2013 to 2015; they came back in
    // 2016 under lei 8/2016.
    HolidayRule::public("Corpus Christi", "Corpo de Deus", Rule::easter(CORPUS_CHRISTI))
        .years(None, Some(2012)),
    HolidayRule::public("Corpus Christi", "Corpo de Deus", Rule::easter(CORPUS_CHRISTI))
        .years(Some(2016), None),
    HolidayRule::public("Portugal Day", "Dia de Portugal", Rule::gregorian(6, 10)),
    HolidayRule::public("Assumption", "Assunção de Nossa Senhora", Rule::gregorian(8, 15)),
    HolidayRule::public("Republic Day", "Implantação da República", Rule::gregorian(10, 5))
        .years(None, Some(2012)),
    HolidayRule::public("Republic Day", "Implantação da República", Rule::gregorian(10, 5))
        .years(Some(2016), None),
    HolidayRule::public("All Saints' Day", "Todos os Santos", Rule::gregorian(11, 1))
        .years(None, Some(2012)),
    HolidayRule::public("All Saints' Day", "Todos os Santos", Rule::gregorian(11, 1))
        .years(Some(2016), None),
    HolidayRule::public("Restoration of Independence", "Restauração da Independência", Rule::gregorian(12, 1))
        .years(None, Some(2012)),
    HolidayRule::public("Restoration of Independence", "Restauração da Independência", Rule::gregorian(12, 1))
        .years(Some(2016), None),
    HolidayRule::public("Immaculate Conception", "Imaculada Conceição", Rule::gregorian(12, 8)),
    HolidayRule::public("Christmas Day", "Natal", Rule::gregorian(12, 25)),
];

/// Portugal.
pub static PORTUGAL: RuleSet = RuleSet {
    code: "PT",
    english_name: "Portugal",
    rules: PT_RULES,
    substitution: &[],
    bridges: &[],
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
    HolidayRule::public("Easter Sunday", "Eerste Paasdag", Rule::easter(EASTER_SUNDAY)),
    HolidayRule::public("Easter Monday", "Tweede Paasdag", Rule::easter(EASTER_MONDAY)),
    HolidayRule::fixed_public("King's Day", "Koningsdag", Rule::Computed(dutch_royal_day))
        .years(Some(2014), None),
    HolidayRule::fixed_public("Queen's Day", "Koninginnedag", Rule::Computed(dutch_royal_day))
        .years(Some(1949), Some(2013)),
    // Liberation Day is a day off for most only in years divisible by five.
    HolidayRule::observance("Liberation Day", "Bevrijdingsdag", Rule::gregorian(5, 5)),
    HolidayRule::public("Ascension", "Hemelvaartsdag", Rule::easter(ASCENSION)),
    HolidayRule::public("Pentecost", "Eerste Pinksterdag", Rule::easter(PENTECOST)),
    HolidayRule::public("Whit Monday", "Tweede Pinksterdag", Rule::easter(WHIT_MONDAY)),
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
    HolidayRule::public("Ascension", "Onze-Lieve-Heer-Hemelvaart", Rule::easter(ASCENSION)),
    HolidayRule::public("Pentecost", "Pinksteren", Rule::easter(PENTECOST)),
    HolidayRule::public("Whit Monday", "Pinkstermaandag", Rule::easter(WHIT_MONDAY)),
    HolidayRule::public("National Day", "Nationale feestdag", Rule::gregorian(7, 21)),
    HolidayRule::public("Assumption", "Onze-Lieve-Vrouw-Hemelvaart", Rule::gregorian(8, 15)),
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
    HolidayRule::public("Corpus Christi", "Fronleichnam", Rule::easter(CORPUS_CHRISTI)),
    HolidayRule::public("Assumption", "Mariä Himmelfahrt", Rule::gregorian(8, 15)),
    HolidayRule::public("National Day", "Nationalfeiertag", Rule::gregorian(10, 26))
        .years(Some(1965), None),
    HolidayRule::public("All Saints' Day", "Allerheiligen", Rule::gregorian(11, 1)),
    HolidayRule::public("Immaculate Conception", "Mariä Empfängnis", Rule::gregorian(12, 8)),
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
    HolidayRule::public("Easter Monday", "Annandag påsk", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("May Day", "Första maj", Rule::gregorian(5, 1)),
    HolidayRule::public("Ascension", "Kristi himmelsfärdsdag", Rule::easter(ASCENSION)),
    HolidayRule::public("Pentecost", "Pingstdagen", Rule::easter(PENTECOST)),
    // Whit Monday was traded for the National Day in 2005.
    HolidayRule::public("Whit Monday", "Annandag pingst", Rule::easter(WHIT_MONDAY))
        .years(None, Some(2004)),
    HolidayRule::public("National Day", "Sveriges nationaldag", Rule::gregorian(6, 6))
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
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Lag (1989:253) om allmänna helgdagar; lag 2004:1042 traded \
              Annandag pingst for the National Day. Midsummer Eve, Christmas \
              Eve and New Year's Eve are not allmänna helgdagar but are \
              de facto closed days, so they are recorded as bank holidays",
};

static NO_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Første nyttårsdag", Rule::gregorian(1, 1)),
    HolidayRule::public("Maundy Thursday", "Skjærtorsdag", Rule::easter(MAUNDY_THURSDAY)),
    HolidayRule::public("Good Friday", "Langfredag", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public("Easter Sunday", "Første påskedag", Rule::easter(EASTER_SUNDAY)),
    HolidayRule::public("Easter Monday", "Andre påskedag", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("Labour Day", "Arbeidernes dag", Rule::gregorian(5, 1)),
    HolidayRule::public("Constitution Day", "Grunnlovsdag", Rule::gregorian(5, 17)),
    HolidayRule::public("Ascension", "Kristi himmelfartsdag", Rule::easter(ASCENSION)),
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
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Lov om helligdager og helligdagsfred (1995-02-24 nr. 12) § 2; \
              lov om 1. og 17. mai som høgtidsdager (1947)",
};

static DK_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Nytårsdag", Rule::gregorian(1, 1)),
    HolidayRule::public("Maundy Thursday", "Skærtorsdag", Rule::easter(MAUNDY_THURSDAY)),
    HolidayRule::public("Good Friday", "Langfredag", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public("Easter Sunday", "Påskedag", Rule::easter(EASTER_SUNDAY)),
    HolidayRule::public("Easter Monday", "Anden påskedag", Rule::easter(EASTER_MONDAY)),
    // Store bededag, the fourth Friday after Easter, was abolished with
    // effect from 2024 by lov nr. 214 af 28. februar 2023 — the first Danish
    // holiday abolished in three centuries.
    HolidayRule::public("Great Prayer Day", "Store bededag", Rule::easter(26))
        .years(None, Some(2023)),
    HolidayRule::public("Ascension", "Kristi himmelfartsdag", Rule::easter(ASCENSION)),
    HolidayRule::public("Pentecost", "Pinsedag", Rule::easter(PENTECOST)),
    HolidayRule::public("Whit Monday", "Anden pinsedag", Rule::easter(WHIT_MONDAY)),
    HolidayRule::observance("Constitution Day", "Grundlovsdag", Rule::gregorian(6, 5)),
    HolidayRule::public("Christmas Eve", "Juleaften", Rule::gregorian(12, 24))
        .of_kind(Kind::Bank),
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
    HolidayRule::public("Easter Sunday", "Pääsiäispäivä", Rule::easter(EASTER_SUNDAY)),
    HolidayRule::public("Easter Monday", "Toinen pääsiäispäivä", Rule::easter(EASTER_MONDAY)),
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
    HolidayRule::public("Independence Day", "Itsenäisyyspäivä", Rule::gregorian(12, 6))
        .years(Some(1917), None),
    HolidayRule::public("Christmas Eve", "Jouluaatto", Rule::gregorian(12, 24))
        .of_kind(Kind::Bank),
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
    HolidayRule::public("Epiphany", "Trzech Króli", Rule::gregorian(1, 6))
        .years(Some(2011), None),
    HolidayRule::public("Easter Sunday", "Wielkanoc", Rule::easter(EASTER_SUNDAY)),
    HolidayRule::public("Easter Monday", "Poniedziałek Wielkanocny", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("Labour Day", "Święto Pracy", Rule::gregorian(5, 1)),
    HolidayRule::public("Constitution Day", "Święto Konstytucji 3 Maja", Rule::gregorian(5, 3))
        .years(Some(1990), None),
    HolidayRule::public("Pentecost", "Zielone Świątki", Rule::easter(PENTECOST)),
    HolidayRule::public("Corpus Christi", "Boże Ciało", Rule::easter(CORPUS_CHRISTI)),
    HolidayRule::public("Assumption", "Wniebowzięcie NMP", Rule::gregorian(8, 15)),
    HolidayRule::public("All Saints' Day", "Wszystkich Świętych", Rule::gregorian(11, 1)),
    HolidayRule::public("Independence Day", "Święto Niepodległości", Rule::gregorian(11, 11))
        .years(Some(1989), None),
    // The centenary of independence got a single extra day.
    HolidayRule::fixed_public("Centenary of Independence", "", Rule::gregorian(11, 12))
        .years(Some(2018), Some(2018)),
    HolidayRule::public("Christmas Eve", "Wigilia Bożego Narodzenia", Rule::gregorian(12, 24))
        .years(Some(2025), None),
    HolidayRule::public("Christmas Day", "Boże Narodzenie", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "Drugi dzień Bożego Narodzenia", Rule::gregorian(12, 26)),
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
    HolidayRule::public("Easter Monday", "Velikonoční pondělí", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("Labour Day", "Svátek práce", Rule::gregorian(5, 1)),
    HolidayRule::public("Victory Day", "Den vítězství", Rule::gregorian(5, 8)),
    HolidayRule::public("Sts Cyril and Methodius", "Den slovanských věrozvěstů", Rule::gregorian(7, 5)),
    HolidayRule::public("Jan Hus Day", "Den upálení mistra Jana Husa", Rule::gregorian(7, 6)),
    HolidayRule::public("Statehood Day", "Den české státnosti", Rule::gregorian(9, 28)),
    HolidayRule::public("Independence Day", "Den vzniku samostatného státu", Rule::gregorian(10, 28)),
    HolidayRule::public("Freedom and Democracy Day", "Den boje za svobodu a demokracii", Rule::gregorian(11, 17)),
    HolidayRule::public("Christmas Eve", "Štědrý den", Rule::gregorian(12, 24)),
    HolidayRule::public("Christmas Day", "1. svátek vánoční", Rule::gregorian(12, 25)),
    HolidayRule::public("St Stephen's Day", "2. svátek vánoční", Rule::gregorian(12, 26)),
];

/// Czechia.
pub static CZECHIA: RuleSet = RuleSet {
    code: "CZ",
    english_name: "Czechia",
    rules: CZ_RULES,
    substitution: &[],
    bridges: &[],
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
    HolidayRule::public("Clean Monday", "Καθαρά Δευτέρα", Rule::paschal(ASH_WEDNESDAY - 2)),
    HolidayRule::public("Independence Day", "Εικοστή Πέμπτη Μαρτίου", Rule::gregorian(3, 25)),
    HolidayRule::public("Good Friday", "Μεγάλη Παρασκευή", Rule::paschal(GOOD_FRIDAY)),
    HolidayRule::public("Easter Sunday", "Κυριακή του Πάσχα", Rule::paschal(EASTER_SUNDAY)),
    HolidayRule::public("Easter Monday", "Δευτέρα του Πάσχα", Rule::paschal(EASTER_MONDAY)),
    HolidayRule::public("Labour Day", "Εργατική Πρωτομαγιά", Rule::gregorian(5, 1)),
    HolidayRule::public("Whit Monday", "Αγίου Πνεύματος", Rule::paschal(WHIT_MONDAY)),
    HolidayRule::public("Dormition of the Theotokos", "Κοίμηση της Θεοτόκου", Rule::gregorian(8, 15)),
    HolidayRule::public("Ochi Day", "Επέτειος του Όχι", Rule::gregorian(10, 28)),
    HolidayRule::public("Christmas Day", "Χριστούγεννα", Rule::gregorian(12, 25)),
    HolidayRule::public("Synaxis of the Theotokos", "Σύναξις Θεοτόκου", Rule::gregorian(12, 26)),
];

/// Greece.
pub static GREECE: RuleSet = RuleSet {
    code: "GR",
    english_name: "Greece",
    rules: GR_RULES,
    substitution: &[],
    bridges: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Νόμος 4808/2021 art. 60 and the ΥΑ setting the yearly list; \
              Easter follows the Julian computus, the fixed feasts the civil \
              calendar",
};
