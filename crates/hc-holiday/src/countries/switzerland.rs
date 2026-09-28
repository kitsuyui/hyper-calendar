//! Switzerland — the Bundesfeiertag, and the cantons' own days under
//! their laws.
//!
//! The regime is written up in `docs/systems/switzerland-holidays.md` in
//! the repository, with every canton and what was read for it; this
//! comment keeps the summary.
//!
//! The Arbeitsgesetz (SR 822.11), Art. 20a Abs. 1, makes 1 August equal to
//! Sunday and lets each canton make up to eight more days equal to Sunday;
//! every other public holiday is cantonal. So the nationwide rules are
//! 1 August and the three days every canton's law keeps — New Year's Day,
//! Ascension and Christmas Day — and every other day is a rule of the
//! canton whose law keeps it, scoped to its ISO 3166-2 code, from the
//! first year the text read was in force on the day, and the years before a
//! gap: the laws read replaced older ones, which were not read. Vaud's
//! 2 January and Whit Monday, which its amendment of 2007 added, are absent
//! in 2006 and 2007, whose text was read. A day equal to Sunday
//! is [`Kind::Public`](crate::rule::Kind::Public); a public rest day the law does not make equal to
//! Sunday, as Lucerne's 8 December or Ticino's giorni festivi "non
//! parificati", is [`Kind::Observance`](crate::rule::Kind::Observance). Only days the law keeps in the
//! whole canton are carried: the days Aargau keeps in some districts,
//! Fribourg in its Catholic or Reformed communes, Solothurn outside the
//! Bucheggberg and Appenzell Innerrhoden in its inner part need a scope
//! finer than a canton. Jura, Schwyz and Zurich publish their laws as PDF
//! only, and their days are carried from secondary sources from 2026, the
//! year they were read, the years before a gap.

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;

use crate::computus::offsets::{
    ASCENSION, CORPUS_CHRISTI, EASTER_MONDAY, GOOD_FRIDAY, WHIT_MONDAY,
};
use crate::rule::{Days, HolidayRule, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate};

/// The Jeûne genevois, "le jeudi qui suit le premier dimanche du mois de
/// septembre": the first Thursday on or after 5 September.
const GENEVA_FAST: Rule = Rule::WeekdayOnOrAfter {
    month: 9,
    day: 5,
    weekday: Weekday::Thursday,
};

/// Vaud's lundi du Jeûne fédéral, the Monday after the Federal Fast, the
/// third Sunday of September: the first Monday on or after 16 September.
const FEDERAL_FAST_MONDAY: Rule = Rule::WeekdayOnOrAfter {
    month: 9,
    day: 16,
    weekday: Weekday::Monday,
};

/// The weekday of `month`/`day` in `year`, if the date exists.
fn weekday_of(year: i64, month: u8, day: u8) -> Option<Weekday> {
    gregorian::to_fixed(year, month, day)
        .ok()
        .map(Weekday::from_rd)
}

/// A date of `year`, as one day.
fn day_of(year: i64, month: u8, day: u8) -> Days {
    gregorian::to_fixed(year, month, day).map_or_else(|_| Days::new(), Days::one)
}

/// Appenzell Ausserrhoden's zweiter Weihnachtstag, which "wird nicht
/// gefeiert, wenn der 1. Weihnachtstag auf einen Montag oder Freitag fällt".
fn appenzell_ausserrhoden_st_stephen(year: i64) -> Days {
    match weekday_of(year, 12, 25) {
        Some(Weekday::Monday | Weekday::Friday) | None => Days::new(),
        Some(_) => day_of(year, 12, 26),
    }
}

/// Appenzell Innerrhoden's Stephanstag, kept "sofern durch dessen Feier
/// nicht drei Ruhetage aufeinander folgen". Read literally that leaves out
/// the years whose Christmas is a Monday or a Friday, but the Canton's own
/// list keeps 26 December 2026, whose Christmas is a Friday; so those years
/// are a gap rather than a guess.
fn appenzell_innerrhoden_st_stephen(year: i64) -> Option<Days> {
    match weekday_of(year, 12, 25)? {
        Weekday::Monday | Weekday::Friday => None,
        _ => Some(day_of(year, 12, 26)),
    }
}

/// Neuchâtel's 2 January, kept "lorsque le 1er janvier … tombe un
/// dimanche".
fn neuchatel_second_january(year: i64) -> Days {
    if weekday_of(year, 1, 1) == Some(Weekday::Sunday) {
        return day_of(year, 1, 2);
    }
    Days::new()
}

/// Neuchâtel's 26 December, kept when Christmas Day falls on a Sunday.
fn neuchatel_st_stephen(year: i64) -> Days {
    if weekday_of(year, 12, 25) == Some(Weekday::Sunday) {
        return day_of(year, 12, 26);
    }
    Days::new()
}

/// Glarus's Fahrtsfest, the Näfelser Fahrt: the first Thursday of April,
/// or the Thursday after when that falls in Holy Week, as the Canton's
/// pages state it; the law names the day and not its date.
fn nafels_pilgrimage(year: i64) -> Days {
    static FIRST_THURSDAY_OF_APRIL: Rule = Rule::nth(4, 1, Weekday::Thursday);
    static PALM_SUNDAY: Rule = Rule::easter(-7);
    let (Some(&thursday), Some(&palm_sunday)) = (
        FIRST_THURSDAY_OF_APRIL
            .days_in_year(year)
            .as_slice()
            .first(),
        PALM_SUNDAY.days_in_year(year).as_slice().first(),
    ) else {
        return Days::new();
    };
    if thursday.0 >= palm_sunday.0 && thursday.0 < palm_sunday.0 + 7 {
        return Days::one(Rd(thursday.0 + 7));
    }
    Days::one(thursday)
}

/// A day a canton's law makes equal to Sunday, from `first`.
const fn canton(
    name: &'static str,
    local_name: &'static str,
    rule: Rule,
    first: i32,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::fixed_public(name, local_name, rule)
        .years(Some(first), None)
        .in_regions(region)
        .cited(source)
}

/// A public rest day of a canton's law that the law does not make equal
/// to Sunday, from `first`.
const fn canton_rest_day(
    name: &'static str,
    local_name: &'static str,
    rule: Rule,
    first: i32,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::observance(name, local_name, rule)
        .years(Some(first), None)
        .in_regions(region)
        .cited(source)
}

/// The years to `last` before a canton's day, as a gap: the law read is the
/// one in force from its commencement, and the day is older than it — an
/// earlier law, not read, kept it or did not.
const fn earlier_years_unread(
    name: &'static str,
    local_name: &'static str,
    last: i32,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::observance(name, local_name, Rule::UNREAD)
        .years(None, Some(last))
        .in_regions(region)
        .cited(source)
}

// The cantons, by ISO 3166-2 code, and the law each was read in.
const CH_AG: &[&str] = &["CH-AG"];
const CH_AG_LAW: &str = "Einführungsgesetz zum Arbeitsrecht (EG ArR) vom 8. November 2011, SAR 961.200, § 6 (https://gesetzessammlungen.ag.ch/app/de/texts_of_law/961.200), retrieved 2026-09-29";
const CH_AI: &[&str] = &["CH-AI"];
const CH_AI_LAW: &str = "Gesetz über die öffentlichen Ruhetage (Ruhetagsgesetz) vom 25. April 1982, GS 822.200, Art. 2, in the version in force from 1 January 2011 (https://ai.clex.ch/app/de/texts_of_law/822.200), retrieved 2026-09-29";
const CH_AR: &[&str] = &["CH-AR"];
const CH_AR_LAW: &str = "Verordnung zum Arbeitsgesetz, bGS 822.11, Art. 7, unchanged since 1 February 1966 (https://ar.clex.ch/app/de/texts_of_law/822.11), retrieved 2026-09-29";
const CH_BE: &[&str] = &["CH-BE"];
const CH_BE_LAW: &str = "Gesetz über die Ruhe an öffentlichen Feiertagen (FRG) vom 1. Dezember 1996, BSG 555.1, Art. 2, unchanged since 1 May 1997 (https://www.belex.sites.be.ch/app/de/texts_of_law/555.1), retrieved 2026-09-29";
const CH_BL: &[&str] = &["CH-BL"];
const CH_BL_LAW: &str = "Gesetz über öffentliche Ruhetage (RTG), SGS 547, §§ 2 and 10, unchanged since 1 January 2011 (https://bl.clex.ch/app/de/texts_of_law/547), retrieved 2026-09-29";
const CH_BS: &[&str] = &["CH-BS"];
const CH_BS_LAW: &str = "Einführungsgesetz zum Arbeitsgesetz, SG 812.100, § 9, in force from 1 January 1994 (https://www.gesetzessammlung.bs.ch/app/de/texts_of_law/812.100), retrieved 2026-09-29";
const CH_FR: &[&str] = &["CH-FR"];
const CH_FR_LAW: &str = "Loi sur l'emploi et le marché du travail (LEMT), RSF 866.1.1, art. 49, unchanged since 1 January 2011 (https://bdlf.fr.ch/app/fr/texts_of_law/866.1.1), retrieved 2026-09-29";
const CH_GE: &[&str] = &["CH-GE"];
const CH_GE_LAW: &str = "Loi sur les jours fériés (LJF) du 3 novembre 1951, rsGE J 1 45, art. 1, as last modified to 1 January 1991 (https://silgeneve.ch/legis/data/rsg_j1_45.htm), retrieved 2026-09-29";
const CH_GL: &[&str] = &["CH-GL"];
const CH_GL_LAW: &str = "Gesetz über die öffentlichen Ruhetage, GS IX B/21/1, Art. 2, unchanged since 6 May 2012; the date of the Fahrtsfest from the Canton's pages on the Näfelser Fahrt (secondary) (https://gesetze.gl.ch/app/de/texts_of_law/IX-B.21.1), retrieved 2026-09-29";
const CH_GR: &[&str] = &["CH-GR"];
const CH_GR_LAW: &str = "Einführungsgesetz zum Arbeitsgesetz, BR 530.100, Art. 7, unchanged since 1 February 2006 (https://www.gr-lex.gr.ch/app/de/texts_of_law/530.100), retrieved 2026-09-29";
const CH_JU: &[&str] = &["CH-JU"];
const CH_JU_LAW: &str = "Wikipedia, \"Feiertage in der Schweiz\" (secondary): the Loi sur les jours fériés officiels, RSJU 555.1, and its décret RSJU 555.10 are published as PDF only and were not read (https://de.wikipedia.org/wiki/Feiertage_in_der_Schweiz), retrieved 2026-09-29";
const CH_LU: &[&str] = &["CH-LU"];
const CH_LU_LAW: &str = "Gesetz über den Ruhetag und die Ladenöffnung, SRL Nr. 855, §§ 1 and 1a, § 1a in force from 1 June 1997 (https://srl.lu.ch/app/de/texts_of_law/855), retrieved 2026-09-29";
const CH_NE: &[&str] = &["CH-NE"];
const CH_NE_LAW: &str = "Loi sur le dimanche et les jours fériés du 30 septembre 1991, RSN 941.02, art. 3, in its wording in force from 1 January 2010 (https://rsn.ne.ch/DATA/program/books/RSN2022/20224/htm/94102.htm), retrieved 2026-09-29";
const CH_NW: &[&str] = &["CH-NW"];
const CH_NW_LAW: &str = "Gesetz über die öffentlichen Ruhetage (RTG), NG 921.1, Art. 2, unchanged since 1 September 2005 (https://gesetze.nw.ch/app/de/texts_of_law/921.1), retrieved 2026-09-29";
const CH_OW: &[&str] = &["CH-OW"];
const CH_OW_LAW: &str = "Gesetz über die öffentlichen Ruhetage, GDB 975.2, Art. 2, unchanged since 1 July 2007 (https://gdb.ow.ch/app/de/texts_of_law/975.2), retrieved 2026-09-29";
const CH_SG: &[&str] = &["CH-SG"];
const CH_SG_LAW: &str = "Einführungsgesetz zum eidgenössischen Arbeitsgesetz, sGS 511.1, Art. 1bis, in force from 1 July 2004 (https://www.gesetzessammlung.sg.ch/app/de/texts_of_law/511.1), retrieved 2026-09-29";
const CH_SH: &[&str] = &["CH-SH"];
const CH_SH_LAW: &str = "Verordnung zum Arbeitsgesetz und zum Bundesgesetz über die Unfallversicherung, SHR 822.101, § 7, unchanged since 1 April 2011 (https://rechtsbuch.sh.ch/app/de/texts_of_law/822.101), retrieved 2026-09-29";
const CH_SO: &[&str] = &["CH-SO"];
const CH_SO_LAW: &str = "Gesetz über Wirtschaft und Arbeit (WAG), BGS 940.11, § 46, unchanged since 1 January 2016 (https://bgs.so.ch/app/de/texts_of_law/940.11), retrieved 2026-09-29";
const CH_SZ: &[&str] = &["CH-SZ"];
const CH_SZ_LAW: &str = "Wikipedia, \"Feiertage in der Schweiz\" (secondary): the Ruhetagsgesetz, SRSZ 545.110, is published as PDF only and was not read (https://de.wikipedia.org/wiki/Feiertage_in_der_Schweiz), retrieved 2026-09-29";
const CH_TG: &[&str] = &["CH-TG"];
const CH_TG_LAW: &str = "Gesetz über die öffentlichen Ruhetage, RB 822.9, § 1, in its version in force from 1 January 2003 to 31 December 2025 and the Ruhetagsgesetz of 5 February 2025 in force from 1 January 2026 (https://www.rechtsbuch.tg.ch/app/de/texts_of_law/822.9), retrieved 2026-09-29";
const CH_TI: &[&str] = &["CH-TI"];
const CH_TI_LAW: &str = "Legge di applicazione della legge federale sul lavoro (LALL), RL 843.100, art. 6, in force from 1 June 2011, and the Legge concernente i giorni festivi ufficiali, RL 843.200, art. 1, in force from 9 February 2010 (https://m3.ti.ch/CAN/RLeggi/public/index.php/raccolta-leggi/legge/num/569), retrieved 2026-09-29";
const CH_UR: &[&str] = &["CH-UR"];
const CH_UR_LAW: &str = "Kantonale Arbeitsverordnung (KAV), RB 20.1111, unchanged since 1 February 2002, and the Gesetz über den Ladenschluss und die Sonntagsruhe (LSG), RB 70.1421 (https://rechtsbuch.ur.ch/app/de/texts_of_law/20.1111), retrieved 2026-09-29";
const CH_VD: &[&str] = &["CH-VD"];
const CH_VD_LAW: &str = "Loi sur l'emploi (LEmp), BLV 822.11, art. 47, in force from 1 January 2006, as amended by the modification of 20 February 2007, in force 1 September 2007 (https://prestations.vd.ch/pub/blv-publication/actes/consolide/822.11), retrieved 2026-09-29";
const CH_VS: &[&str] = &["CH-VS"];
const CH_VS_LAW: &str = "Ordonnance cantonale sur le travail (OcTr), RS 822.100, art. 7, in force from 1 October 2016 (https://lex.vs.ch/app/fr/texts_of_law/822.100), retrieved 2026-09-29";
const CH_ZG: &[&str] = &["CH-ZG"];
const CH_ZG_LAW: &str = "Gesetz über Ruhetage und Ladenöffnung, BGS 942.31, § 1, unchanged since 1 January 2004 (https://bgs.zg.ch/app/de/texts_of_law/942.31), retrieved 2026-09-29";
const CH_ZH: &[&str] = &["CH-ZH"];
const CH_ZH_LAW: &str = "the Canton's page \"Feiertage\" (secondary, official), for the Ruhetags- und Ladenöffnungsgesetz vom 26. Juni 2000, LS 822.4, § 1, published as PDF only and not read (https://www.zh.ch/de/wirtschaft-arbeit/arbeitsbedingungen/arbeitsssicherheit-gesundheitsschutz/arbeits-ruhezeiten/feiertage.html), retrieved 2026-09-29";

static CH_RULES: &[HolidayRule] = &[
    // ── Kept in every canton ────────────────────────────────────────────
    HolidayRule::public("New Year's Day", "Neujahrstag", Rule::gregorian(1, 1)),
    HolidayRule::public("Ascension", "Auffahrt", Rule::easter(ASCENSION)),
    HolidayRule::public("Swiss National Day", "Bundesfeier", Rule::gregorian(8, 1))
        .years(Some(1994), None),
    HolidayRule::public("Christmas Day", "Weihnachtstag", Rule::gregorian(12, 25)),
    // ── Each canton's other days ────────────────────────────────────────
    // Aargau.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        2013,
        CH_AG,
        CH_AG_LAW,
    ),
    earlier_years_unread("Good Friday", "Karfreitag", 2012, CH_AG, CH_AG_LAW),
    // Appenzell Innerrhoden.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        2011,
        CH_AI,
        CH_AI_LAW,
    ),
    earlier_years_unread("Good Friday", "Karfreitag", 2010, CH_AI, CH_AI_LAW),
    canton(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        2011,
        CH_AI,
        CH_AI_LAW,
    ),
    earlier_years_unread("Easter Monday", "Ostermontag", 2010, CH_AI, CH_AI_LAW),
    canton(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        2011,
        CH_AI,
        CH_AI_LAW,
    ),
    earlier_years_unread("Whit Monday", "Pfingstmontag", 2010, CH_AI, CH_AI_LAW),
    canton(
        "Corpus Christi",
        "Fronleichnam",
        Rule::easter(CORPUS_CHRISTI),
        2011,
        CH_AI,
        CH_AI_LAW,
    ),
    earlier_years_unread("Corpus Christi", "Fronleichnam", 2010, CH_AI, CH_AI_LAW),
    canton(
        "St Stephen's Day",
        "Stephanstag",
        Rule::Unsettled(appenzell_innerrhoden_st_stephen),
        2011,
        CH_AI,
        CH_AI_LAW,
    ),
    earlier_years_unread("St Stephen's Day", "Stephanstag", 2010, CH_AI, CH_AI_LAW),
    canton_rest_day(
        "Assumption",
        "Maria Himmelfahrt",
        Rule::gregorian(8, 15),
        1982,
        CH_AI,
        CH_AI_LAW,
    ),
    earlier_years_unread("Assumption", "Maria Himmelfahrt", 1981, CH_AI, CH_AI_LAW),
    canton_rest_day(
        "All Saints' Day",
        "Allerheiligen",
        Rule::gregorian(11, 1),
        1982,
        CH_AI,
        CH_AI_LAW,
    ),
    earlier_years_unread("All Saints' Day", "Allerheiligen", 1981, CH_AI, CH_AI_LAW),
    canton_rest_day(
        "Immaculate Conception",
        "Maria Empfängnis",
        Rule::gregorian(12, 8),
        1982,
        CH_AI,
        CH_AI_LAW,
    ),
    earlier_years_unread(
        "Immaculate Conception",
        "Maria Empfängnis",
        1981,
        CH_AI,
        CH_AI_LAW,
    ),
    // Appenzell Ausserrhoden.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        1966,
        CH_AR,
        CH_AR_LAW,
    ),
    earlier_years_unread("Good Friday", "Karfreitag", 1965, CH_AR, CH_AR_LAW),
    canton(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        1966,
        CH_AR,
        CH_AR_LAW,
    ),
    earlier_years_unread("Easter Monday", "Ostermontag", 1965, CH_AR, CH_AR_LAW),
    canton(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        1966,
        CH_AR,
        CH_AR_LAW,
    ),
    earlier_years_unread("Whit Monday", "Pfingstmontag", 1965, CH_AR, CH_AR_LAW),
    canton(
        "St Stephen's Day",
        "zweiter Weihnachtstag",
        Rule::Computed(appenzell_ausserrhoden_st_stephen),
        1967,
        CH_AR,
        CH_AR_LAW,
    ),
    earlier_years_unread(
        "St Stephen's Day",
        "zweiter Weihnachtstag",
        1966,
        CH_AR,
        CH_AR_LAW,
    ),
    // Bern.
    canton(
        "Berchtold's Day",
        "der 2. Januar",
        Rule::gregorian(1, 2),
        1998,
        CH_BE,
        CH_BE_LAW,
    ),
    earlier_years_unread("Berchtold's Day", "der 2. Januar", 1997, CH_BE, CH_BE_LAW),
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        1998,
        CH_BE,
        CH_BE_LAW,
    ),
    earlier_years_unread("Good Friday", "Karfreitag", 1997, CH_BE, CH_BE_LAW),
    canton(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        1998,
        CH_BE,
        CH_BE_LAW,
    ),
    earlier_years_unread("Easter Monday", "Ostermontag", 1997, CH_BE, CH_BE_LAW),
    canton(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        1997,
        CH_BE,
        CH_BE_LAW,
    ),
    earlier_years_unread("Whit Monday", "Pfingstmontag", 1996, CH_BE, CH_BE_LAW),
    canton(
        "St Stephen's Day",
        "der 26. Dezember",
        Rule::gregorian(12, 26),
        1997,
        CH_BE,
        CH_BE_LAW,
    ),
    earlier_years_unread(
        "St Stephen's Day",
        "der 26. Dezember",
        1996,
        CH_BE,
        CH_BE_LAW,
    ),
    // Basel-Landschaft.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        2011,
        CH_BL,
        CH_BL_LAW,
    ),
    earlier_years_unread("Good Friday", "Karfreitag", 2010, CH_BL, CH_BL_LAW),
    canton(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        2011,
        CH_BL,
        CH_BL_LAW,
    ),
    earlier_years_unread("Easter Monday", "Ostermontag", 2010, CH_BL, CH_BL_LAW),
    canton(
        "Labour Day",
        "1. Mai",
        Rule::gregorian(5, 1),
        2011,
        CH_BL,
        CH_BL_LAW,
    ),
    earlier_years_unread("Labour Day", "1. Mai", 2010, CH_BL, CH_BL_LAW),
    canton(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        2011,
        CH_BL,
        CH_BL_LAW,
    ),
    earlier_years_unread("Whit Monday", "Pfingstmontag", 2010, CH_BL, CH_BL_LAW),
    canton(
        "St Stephen's Day",
        "Stephanstag",
        Rule::gregorian(12, 26),
        2011,
        CH_BL,
        CH_BL_LAW,
    ),
    earlier_years_unread("St Stephen's Day", "Stephanstag", 2010, CH_BL, CH_BL_LAW),
    // Basel-Stadt.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        1994,
        CH_BS,
        CH_BS_LAW,
    ),
    earlier_years_unread("Good Friday", "Karfreitag", 1993, CH_BS, CH_BS_LAW),
    canton(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        1994,
        CH_BS,
        CH_BS_LAW,
    ),
    earlier_years_unread("Easter Monday", "Ostermontag", 1993, CH_BS, CH_BS_LAW),
    canton(
        "Labour Day",
        "1. Mai",
        Rule::gregorian(5, 1),
        1994,
        CH_BS,
        CH_BS_LAW,
    ),
    earlier_years_unread("Labour Day", "1. Mai", 1993, CH_BS, CH_BS_LAW),
    canton(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        1994,
        CH_BS,
        CH_BS_LAW,
    ),
    earlier_years_unread("Whit Monday", "Pfingstmontag", 1993, CH_BS, CH_BS_LAW),
    canton(
        "St Stephen's Day",
        "Stephanstag",
        Rule::gregorian(12, 26),
        1994,
        CH_BS,
        CH_BS_LAW,
    ),
    earlier_years_unread("St Stephen's Day", "Stephanstag", 1993, CH_BS, CH_BS_LAW),
    // Fribourg.
    canton(
        "Good Friday",
        "Vendredi-Saint",
        Rule::easter(GOOD_FRIDAY),
        2011,
        CH_FR,
        CH_FR_LAW,
    ),
    earlier_years_unread("Good Friday", "Vendredi-Saint", 2010, CH_FR, CH_FR_LAW),
    // Geneva.
    canton(
        "Good Friday",
        "Vendredi saint",
        Rule::easter(GOOD_FRIDAY),
        1991,
        CH_GE,
        CH_GE_LAW,
    ),
    earlier_years_unread("Good Friday", "Vendredi saint", 1990, CH_GE, CH_GE_LAW),
    canton(
        "Easter Monday",
        "Lundi de Pâques",
        Rule::easter(EASTER_MONDAY),
        1991,
        CH_GE,
        CH_GE_LAW,
    ),
    earlier_years_unread("Easter Monday", "Lundi de Pâques", 1990, CH_GE, CH_GE_LAW),
    canton(
        "Whit Monday",
        "Lundi de Pentecôte",
        Rule::easter(WHIT_MONDAY),
        1991,
        CH_GE,
        CH_GE_LAW,
    ),
    earlier_years_unread("Whit Monday", "Lundi de Pentecôte", 1990, CH_GE, CH_GE_LAW),
    canton(
        "Geneva Fast",
        "Jeûne genevois",
        GENEVA_FAST,
        1991,
        CH_GE,
        CH_GE_LAW,
    ),
    earlier_years_unread("Geneva Fast", "Jeûne genevois", 1990, CH_GE, CH_GE_LAW),
    canton(
        "Restoration of the Republic",
        "31 Décembre, anniversaire de la restauration de la République",
        Rule::gregorian(12, 31),
        1991,
        CH_GE,
        CH_GE_LAW,
    ),
    earlier_years_unread(
        "Restoration of the Republic",
        "31 Décembre, anniversaire de la restauration de la République",
        1990,
        CH_GE,
        CH_GE_LAW,
    ),
    // Glarus.
    canton(
        "Näfels Pilgrimage",
        "Fahrtsfest",
        Rule::Computed(nafels_pilgrimage),
        2013,
        CH_GL,
        CH_GL_LAW,
    ),
    earlier_years_unread("Näfels Pilgrimage", "Fahrtsfest", 2012, CH_GL, CH_GL_LAW),
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        2013,
        CH_GL,
        CH_GL_LAW,
    ),
    earlier_years_unread("Good Friday", "Karfreitag", 2012, CH_GL, CH_GL_LAW),
    canton(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        2013,
        CH_GL,
        CH_GL_LAW,
    ),
    earlier_years_unread("Easter Monday", "Ostermontag", 2012, CH_GL, CH_GL_LAW),
    canton(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        2012,
        CH_GL,
        CH_GL_LAW,
    ),
    earlier_years_unread("Whit Monday", "Pfingstmontag", 2011, CH_GL, CH_GL_LAW),
    canton(
        "All Saints' Day",
        "Allerheiligen",
        Rule::gregorian(11, 1),
        2012,
        CH_GL,
        CH_GL_LAW,
    ),
    earlier_years_unread("All Saints' Day", "Allerheiligen", 2011, CH_GL, CH_GL_LAW),
    canton(
        "St Stephen's Day",
        "Stephanstag",
        Rule::gregorian(12, 26),
        2012,
        CH_GL,
        CH_GL_LAW,
    ),
    earlier_years_unread("St Stephen's Day", "Stephanstag", 2011, CH_GL, CH_GL_LAW),
    // Graubünden.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        2006,
        CH_GR,
        CH_GR_LAW,
    ),
    earlier_years_unread("Good Friday", "Karfreitag", 2005, CH_GR, CH_GR_LAW),
    canton(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        2006,
        CH_GR,
        CH_GR_LAW,
    ),
    earlier_years_unread("Easter Monday", "Ostermontag", 2005, CH_GR, CH_GR_LAW),
    canton(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        2006,
        CH_GR,
        CH_GR_LAW,
    ),
    earlier_years_unread("Whit Monday", "Pfingstmontag", 2005, CH_GR, CH_GR_LAW),
    canton(
        "St Stephen's Day",
        "Stefanstag",
        Rule::gregorian(12, 26),
        2006,
        CH_GR,
        CH_GR_LAW,
    ),
    earlier_years_unread("St Stephen's Day", "Stefanstag", 2005, CH_GR, CH_GR_LAW),
    // Jura.
    canton(
        "Good Friday",
        "Vendredi-Saint",
        Rule::easter(GOOD_FRIDAY),
        2026,
        CH_JU,
        CH_JU_LAW,
    ),
    earlier_years_unread("Good Friday", "Vendredi-Saint", 2025, CH_JU, CH_JU_LAW),
    canton(
        "Easter Monday",
        "Lundi de Pâques",
        Rule::easter(EASTER_MONDAY),
        2026,
        CH_JU,
        CH_JU_LAW,
    ),
    earlier_years_unread("Easter Monday", "Lundi de Pâques", 2025, CH_JU, CH_JU_LAW),
    canton(
        "Labour Day",
        "1er mai",
        Rule::gregorian(5, 1),
        2026,
        CH_JU,
        CH_JU_LAW,
    ),
    earlier_years_unread("Labour Day", "1er mai", 2025, CH_JU, CH_JU_LAW),
    canton(
        "Whit Monday",
        "Lundi de Pentecôte",
        Rule::easter(WHIT_MONDAY),
        2026,
        CH_JU,
        CH_JU_LAW,
    ),
    earlier_years_unread("Whit Monday", "Lundi de Pentecôte", 2025, CH_JU, CH_JU_LAW),
    canton(
        "Corpus Christi",
        "Fête-Dieu",
        Rule::easter(CORPUS_CHRISTI),
        2026,
        CH_JU,
        CH_JU_LAW,
    ),
    earlier_years_unread("Corpus Christi", "Fête-Dieu", 2025, CH_JU, CH_JU_LAW),
    canton_rest_day(
        "Berchtold's Day",
        "2 janvier",
        Rule::gregorian(1, 2),
        2026,
        CH_JU,
        CH_JU_LAW,
    ),
    earlier_years_unread("Berchtold's Day", "2 janvier", 2025, CH_JU, CH_JU_LAW),
    canton_rest_day(
        "Assumption",
        "Assomption",
        Rule::gregorian(8, 15),
        2026,
        CH_JU,
        CH_JU_LAW,
    ),
    earlier_years_unread("Assumption", "Assomption", 2025, CH_JU, CH_JU_LAW),
    canton_rest_day(
        "All Saints' Day",
        "Toussaint",
        Rule::gregorian(11, 1),
        2026,
        CH_JU,
        CH_JU_LAW,
    ),
    earlier_years_unread("All Saints' Day", "Toussaint", 2025, CH_JU, CH_JU_LAW),
    // Lucerne.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        1998,
        CH_LU,
        CH_LU_LAW,
    ),
    earlier_years_unread("Good Friday", "Karfreitag", 1997, CH_LU, CH_LU_LAW),
    canton(
        "Corpus Christi",
        "Fronleichnam",
        Rule::easter(CORPUS_CHRISTI),
        1998,
        CH_LU,
        CH_LU_LAW,
    ),
    earlier_years_unread("Corpus Christi", "Fronleichnam", 1997, CH_LU, CH_LU_LAW),
    canton(
        "Assumption",
        "Mariä Himmelfahrt",
        Rule::gregorian(8, 15),
        1997,
        CH_LU,
        CH_LU_LAW,
    ),
    earlier_years_unread("Assumption", "Mariä Himmelfahrt", 1996, CH_LU, CH_LU_LAW),
    canton(
        "All Saints' Day",
        "Allerheiligen",
        Rule::gregorian(11, 1),
        1997,
        CH_LU,
        CH_LU_LAW,
    ),
    earlier_years_unread("All Saints' Day", "Allerheiligen", 1996, CH_LU, CH_LU_LAW),
    canton(
        "St Stephen's Day",
        "Stefanstag",
        Rule::gregorian(12, 26),
        1997,
        CH_LU,
        CH_LU_LAW,
    ),
    earlier_years_unread("St Stephen's Day", "Stefanstag", 1996, CH_LU, CH_LU_LAW),
    canton_rest_day(
        "Immaculate Conception",
        "Mariä Empfängnis",
        Rule::gregorian(12, 8),
        1997,
        CH_LU,
        CH_LU_LAW,
    ),
    earlier_years_unread(
        "Immaculate Conception",
        "Mariä Empfängnis",
        1996,
        CH_LU,
        CH_LU_LAW,
    ),
    // Neuchâtel.
    canton(
        "2 January",
        "le 2 janvier",
        Rule::Computed(neuchatel_second_january),
        2010,
        CH_NE,
        CH_NE_LAW,
    ),
    earlier_years_unread("2 January", "le 2 janvier", 2009, CH_NE, CH_NE_LAW),
    canton(
        "1 March",
        "le 1er mars",
        Rule::gregorian(3, 1),
        2010,
        CH_NE,
        CH_NE_LAW,
    ),
    earlier_years_unread("1 March", "le 1er mars", 2009, CH_NE, CH_NE_LAW),
    canton(
        "Labour Day",
        "le 1er mai",
        Rule::gregorian(5, 1),
        2010,
        CH_NE,
        CH_NE_LAW,
    ),
    earlier_years_unread("Labour Day", "le 1er mai", 2009, CH_NE, CH_NE_LAW),
    canton(
        "Good Friday",
        "Vendredi Saint",
        Rule::easter(GOOD_FRIDAY),
        2010,
        CH_NE,
        CH_NE_LAW,
    ),
    earlier_years_unread("Good Friday", "Vendredi Saint", 2009, CH_NE, CH_NE_LAW),
    canton(
        "St Stephen's Day",
        "le 26 décembre",
        Rule::Computed(neuchatel_st_stephen),
        2010,
        CH_NE,
        CH_NE_LAW,
    ),
    earlier_years_unread("St Stephen's Day", "le 26 décembre", 2009, CH_NE, CH_NE_LAW),
    // Nidwalden.
    canton_rest_day(
        "St Joseph's Day",
        "Josefstag",
        Rule::gregorian(3, 19),
        2006,
        CH_NW,
        CH_NW_LAW,
    ),
    earlier_years_unread("St Joseph's Day", "Josefstag", 2005, CH_NW, CH_NW_LAW),
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        2006,
        CH_NW,
        CH_NW_LAW,
    ),
    earlier_years_unread("Good Friday", "Karfreitag", 2005, CH_NW, CH_NW_LAW),
    canton(
        "Corpus Christi",
        "Fronleichnam",
        Rule::easter(CORPUS_CHRISTI),
        2006,
        CH_NW,
        CH_NW_LAW,
    ),
    earlier_years_unread("Corpus Christi", "Fronleichnam", 2005, CH_NW, CH_NW_LAW),
    canton(
        "Assumption",
        "Maria Himmelfahrt",
        Rule::gregorian(8, 15),
        2006,
        CH_NW,
        CH_NW_LAW,
    ),
    earlier_years_unread("Assumption", "Maria Himmelfahrt", 2005, CH_NW, CH_NW_LAW),
    canton(
        "All Saints' Day",
        "Allerheiligen",
        Rule::gregorian(11, 1),
        2005,
        CH_NW,
        CH_NW_LAW,
    ),
    earlier_years_unread("All Saints' Day", "Allerheiligen", 2004, CH_NW, CH_NW_LAW),
    canton(
        "Immaculate Conception",
        "Maria Empfängnis",
        Rule::gregorian(12, 8),
        2005,
        CH_NW,
        CH_NW_LAW,
    ),
    earlier_years_unread(
        "Immaculate Conception",
        "Maria Empfängnis",
        2004,
        CH_NW,
        CH_NW_LAW,
    ),
    // Obwalden.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        2008,
        CH_OW,
        CH_OW_LAW,
    ),
    earlier_years_unread("Good Friday", "Karfreitag", 2007, CH_OW, CH_OW_LAW),
    canton(
        "Corpus Christi",
        "Fronleichnam",
        Rule::easter(CORPUS_CHRISTI),
        2008,
        CH_OW,
        CH_OW_LAW,
    ),
    earlier_years_unread("Corpus Christi", "Fronleichnam", 2007, CH_OW, CH_OW_LAW),
    canton(
        "Assumption",
        "Mariä Himmelfahrt",
        Rule::gregorian(8, 15),
        2007,
        CH_OW,
        CH_OW_LAW,
    ),
    earlier_years_unread("Assumption", "Mariä Himmelfahrt", 2006, CH_OW, CH_OW_LAW),
    canton(
        "All Saints' Day",
        "Allerheiligen",
        Rule::gregorian(11, 1),
        2007,
        CH_OW,
        CH_OW_LAW,
    ),
    earlier_years_unread("All Saints' Day", "Allerheiligen", 2006, CH_OW, CH_OW_LAW),
    canton(
        "Immaculate Conception",
        "Mariä Empfängnis",
        Rule::gregorian(12, 8),
        2007,
        CH_OW,
        CH_OW_LAW,
    ),
    earlier_years_unread(
        "Immaculate Conception",
        "Mariä Empfängnis",
        2006,
        CH_OW,
        CH_OW_LAW,
    ),
    canton_rest_day(
        "St Nicholas of Flüe",
        "Bruderklausenfest",
        Rule::gregorian(9, 25),
        2007,
        CH_OW,
        CH_OW_LAW,
    ),
    earlier_years_unread(
        "St Nicholas of Flüe",
        "Bruderklausenfest",
        2006,
        CH_OW,
        CH_OW_LAW,
    ),
    // St. Gallen.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        2005,
        CH_SG,
        CH_SG_LAW,
    ),
    earlier_years_unread("Good Friday", "Karfreitag", 2004, CH_SG, CH_SG_LAW),
    canton(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        2005,
        CH_SG,
        CH_SG_LAW,
    ),
    earlier_years_unread("Easter Monday", "Ostermontag", 2004, CH_SG, CH_SG_LAW),
    canton(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        2005,
        CH_SG,
        CH_SG_LAW,
    ),
    earlier_years_unread("Whit Monday", "Pfingstmontag", 2004, CH_SG, CH_SG_LAW),
    canton(
        "All Saints' Day",
        "Allerheiligen",
        Rule::gregorian(11, 1),
        2004,
        CH_SG,
        CH_SG_LAW,
    ),
    earlier_years_unread("All Saints' Day", "Allerheiligen", 2003, CH_SG, CH_SG_LAW),
    canton(
        "St Stephen's Day",
        "Stefanstag",
        Rule::gregorian(12, 26),
        2004,
        CH_SG,
        CH_SG_LAW,
    ),
    earlier_years_unread("St Stephen's Day", "Stefanstag", 2003, CH_SG, CH_SG_LAW),
    // Schaffhausen.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        2011,
        CH_SH,
        CH_SH_LAW,
    ),
    earlier_years_unread("Good Friday", "Karfreitag", 2010, CH_SH, CH_SH_LAW),
    canton(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        2011,
        CH_SH,
        CH_SH_LAW,
    ),
    earlier_years_unread("Easter Monday", "Ostermontag", 2010, CH_SH, CH_SH_LAW),
    canton(
        "Labour Day",
        "1. Mai",
        Rule::gregorian(5, 1),
        2011,
        CH_SH,
        CH_SH_LAW,
    ),
    earlier_years_unread("Labour Day", "1. Mai", 2010, CH_SH, CH_SH_LAW),
    canton(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        2011,
        CH_SH,
        CH_SH_LAW,
    ),
    earlier_years_unread("Whit Monday", "Pfingstmontag", 2010, CH_SH, CH_SH_LAW),
    canton(
        "St Stephen's Day",
        "Stephanstag",
        Rule::gregorian(12, 26),
        2011,
        CH_SH,
        CH_SH_LAW,
    ),
    earlier_years_unread("St Stephen's Day", "Stephanstag", 2010, CH_SH, CH_SH_LAW),
    // Solothurn.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        2016,
        CH_SO,
        CH_SO_LAW,
    ),
    earlier_years_unread("Good Friday", "Karfreitag", 2015, CH_SO, CH_SO_LAW),
    // Schwyz.
    canton(
        "St Joseph's Day",
        "Josefstag",
        Rule::gregorian(3, 19),
        2026,
        CH_SZ,
        CH_SZ_LAW,
    ),
    earlier_years_unread("St Joseph's Day", "Josefstag", 2025, CH_SZ, CH_SZ_LAW),
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        2026,
        CH_SZ,
        CH_SZ_LAW,
    ),
    earlier_years_unread("Good Friday", "Karfreitag", 2025, CH_SZ, CH_SZ_LAW),
    canton(
        "Corpus Christi",
        "Fronleichnam",
        Rule::easter(CORPUS_CHRISTI),
        2026,
        CH_SZ,
        CH_SZ_LAW,
    ),
    earlier_years_unread("Corpus Christi", "Fronleichnam", 2025, CH_SZ, CH_SZ_LAW),
    canton(
        "Assumption",
        "Mariä Himmelfahrt",
        Rule::gregorian(8, 15),
        2026,
        CH_SZ,
        CH_SZ_LAW,
    ),
    earlier_years_unread("Assumption", "Mariä Himmelfahrt", 2025, CH_SZ, CH_SZ_LAW),
    canton(
        "All Saints' Day",
        "Allerheiligen",
        Rule::gregorian(11, 1),
        2026,
        CH_SZ,
        CH_SZ_LAW,
    ),
    earlier_years_unread("All Saints' Day", "Allerheiligen", 2025, CH_SZ, CH_SZ_LAW),
    canton_rest_day(
        "Epiphany",
        "Heilige Drei Könige",
        Rule::gregorian(1, 6),
        2026,
        CH_SZ,
        CH_SZ_LAW,
    ),
    earlier_years_unread("Epiphany", "Heilige Drei Könige", 2025, CH_SZ, CH_SZ_LAW),
    canton_rest_day(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        2026,
        CH_SZ,
        CH_SZ_LAW,
    ),
    earlier_years_unread("Easter Monday", "Ostermontag", 2025, CH_SZ, CH_SZ_LAW),
    canton_rest_day(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        2026,
        CH_SZ,
        CH_SZ_LAW,
    ),
    earlier_years_unread("Whit Monday", "Pfingstmontag", 2025, CH_SZ, CH_SZ_LAW),
    canton_rest_day(
        "Immaculate Conception",
        "Mariä Empfängnis",
        Rule::gregorian(12, 8),
        2026,
        CH_SZ,
        CH_SZ_LAW,
    ),
    earlier_years_unread(
        "Immaculate Conception",
        "Mariä Empfängnis",
        2025,
        CH_SZ,
        CH_SZ_LAW,
    ),
    canton_rest_day(
        "St Stephen's Day",
        "Stephanstag",
        Rule::gregorian(12, 26),
        2026,
        CH_SZ,
        CH_SZ_LAW,
    ),
    earlier_years_unread("St Stephen's Day", "Stephanstag", 2025, CH_SZ, CH_SZ_LAW),
    // Thurgau.
    canton(
        "Berchtold's Day",
        "2. Januar",
        Rule::gregorian(1, 2),
        2003,
        CH_TG,
        CH_TG_LAW,
    ),
    earlier_years_unread("Berchtold's Day", "2. Januar", 2002, CH_TG, CH_TG_LAW),
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        2003,
        CH_TG,
        CH_TG_LAW,
    ),
    earlier_years_unread("Good Friday", "Karfreitag", 2002, CH_TG, CH_TG_LAW),
    canton(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        2003,
        CH_TG,
        CH_TG_LAW,
    ),
    earlier_years_unread("Easter Monday", "Ostermontag", 2002, CH_TG, CH_TG_LAW),
    canton(
        "Labour Day",
        "1. Mai",
        Rule::gregorian(5, 1),
        2003,
        CH_TG,
        CH_TG_LAW,
    ),
    earlier_years_unread("Labour Day", "1. Mai", 2002, CH_TG, CH_TG_LAW),
    canton(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        2003,
        CH_TG,
        CH_TG_LAW,
    ),
    earlier_years_unread("Whit Monday", "Pfingstmontag", 2002, CH_TG, CH_TG_LAW),
    canton(
        "St Stephen's Day",
        "26. Dezember",
        Rule::gregorian(12, 26),
        2003,
        CH_TG,
        CH_TG_LAW,
    ),
    earlier_years_unread("St Stephen's Day", "26. Dezember", 2002, CH_TG, CH_TG_LAW),
    // Ticino.
    canton(
        "Epiphany",
        "Epifania",
        Rule::gregorian(1, 6),
        2012,
        CH_TI,
        CH_TI_LAW,
    ),
    earlier_years_unread("Epiphany", "Epifania", 2011, CH_TI, CH_TI_LAW),
    canton(
        "Easter Monday",
        "Lunedì di Pasqua",
        Rule::easter(EASTER_MONDAY),
        2012,
        CH_TI,
        CH_TI_LAW,
    ),
    earlier_years_unread("Easter Monday", "Lunedì di Pasqua", 2011, CH_TI, CH_TI_LAW),
    canton(
        "Assumption",
        "Assunzione",
        Rule::gregorian(8, 15),
        2011,
        CH_TI,
        CH_TI_LAW,
    ),
    earlier_years_unread("Assumption", "Assunzione", 2010, CH_TI, CH_TI_LAW),
    canton(
        "All Saints' Day",
        "Ognissanti",
        Rule::gregorian(11, 1),
        2011,
        CH_TI,
        CH_TI_LAW,
    ),
    earlier_years_unread("All Saints' Day", "Ognissanti", 2010, CH_TI, CH_TI_LAW),
    canton(
        "St Stephen's Day",
        "Santo Stefano",
        Rule::gregorian(12, 26),
        2011,
        CH_TI,
        CH_TI_LAW,
    ),
    earlier_years_unread("St Stephen's Day", "Santo Stefano", 2010, CH_TI, CH_TI_LAW),
    canton_rest_day(
        "St Joseph's Day",
        "San Giuseppe",
        Rule::gregorian(3, 19),
        2010,
        CH_TI,
        CH_TI_LAW,
    ),
    earlier_years_unread("St Joseph's Day", "San Giuseppe", 2009, CH_TI, CH_TI_LAW),
    canton_rest_day(
        "Labour Day",
        "1° Maggio",
        Rule::gregorian(5, 1),
        2010,
        CH_TI,
        CH_TI_LAW,
    ),
    earlier_years_unread("Labour Day", "1° Maggio", 2009, CH_TI, CH_TI_LAW),
    canton_rest_day(
        "Whit Monday",
        "Lunedì di Pentecoste",
        Rule::easter(WHIT_MONDAY),
        2010,
        CH_TI,
        CH_TI_LAW,
    ),
    earlier_years_unread(
        "Whit Monday",
        "Lunedì di Pentecoste",
        2009,
        CH_TI,
        CH_TI_LAW,
    ),
    canton_rest_day(
        "Corpus Christi",
        "Corpus Domini",
        Rule::easter(CORPUS_CHRISTI),
        2010,
        CH_TI,
        CH_TI_LAW,
    ),
    earlier_years_unread("Corpus Christi", "Corpus Domini", 2009, CH_TI, CH_TI_LAW),
    canton_rest_day(
        "Saints Peter and Paul",
        "SS. Pietro e Paolo",
        Rule::gregorian(6, 29),
        2010,
        CH_TI,
        CH_TI_LAW,
    ),
    earlier_years_unread(
        "Saints Peter and Paul",
        "SS. Pietro e Paolo",
        2009,
        CH_TI,
        CH_TI_LAW,
    ),
    canton_rest_day(
        "Immaculate Conception",
        "Immacolata",
        Rule::gregorian(12, 8),
        2010,
        CH_TI,
        CH_TI_LAW,
    ),
    earlier_years_unread(
        "Immaculate Conception",
        "Immacolata",
        2009,
        CH_TI,
        CH_TI_LAW,
    ),
    // Uri.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        2002,
        CH_UR,
        CH_UR_LAW,
    ),
    earlier_years_unread("Good Friday", "Karfreitag", 2001, CH_UR, CH_UR_LAW),
    canton(
        "Corpus Christi",
        "Fronleichnam",
        Rule::easter(CORPUS_CHRISTI),
        2002,
        CH_UR,
        CH_UR_LAW,
    ),
    earlier_years_unread("Corpus Christi", "Fronleichnam", 2001, CH_UR, CH_UR_LAW),
    canton(
        "Assumption",
        "Mariä Himmelfahrt",
        Rule::gregorian(8, 15),
        2002,
        CH_UR,
        CH_UR_LAW,
    ),
    earlier_years_unread("Assumption", "Mariä Himmelfahrt", 2001, CH_UR, CH_UR_LAW),
    canton(
        "All Saints' Day",
        "Allerheiligen",
        Rule::gregorian(11, 1),
        2002,
        CH_UR,
        CH_UR_LAW,
    ),
    earlier_years_unread("All Saints' Day", "Allerheiligen", 2001, CH_UR, CH_UR_LAW),
    canton(
        "Immaculate Conception",
        "Mariä Empfängnis",
        Rule::gregorian(12, 8),
        2002,
        CH_UR,
        CH_UR_LAW,
    ),
    earlier_years_unread(
        "Immaculate Conception",
        "Mariä Empfängnis",
        2001,
        CH_UR,
        CH_UR_LAW,
    ),
    canton_rest_day(
        "Epiphany",
        "Dreikönigen",
        Rule::gregorian(1, 6),
        2003,
        CH_UR,
        CH_UR_LAW,
    ),
    earlier_years_unread("Epiphany", "Dreikönigen", 2002, CH_UR, CH_UR_LAW),
    canton_rest_day(
        "St Joseph's Day",
        "Sankt-Josefs-Tag",
        Rule::gregorian(3, 19),
        2003,
        CH_UR,
        CH_UR_LAW,
    ),
    earlier_years_unread(
        "St Joseph's Day",
        "Sankt-Josefs-Tag",
        2002,
        CH_UR,
        CH_UR_LAW,
    ),
    canton_rest_day(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        2003,
        CH_UR,
        CH_UR_LAW,
    ),
    earlier_years_unread("Easter Monday", "Ostermontag", 2002, CH_UR, CH_UR_LAW),
    canton_rest_day(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        2003,
        CH_UR,
        CH_UR_LAW,
    ),
    earlier_years_unread("Whit Monday", "Pfingstmontag", 2002, CH_UR, CH_UR_LAW),
    canton_rest_day(
        "St Stephen's Day",
        "Sankt-Stefans-Tag",
        Rule::gregorian(12, 26),
        2003,
        CH_UR,
        CH_UR_LAW,
    ),
    earlier_years_unread(
        "St Stephen's Day",
        "Sankt-Stefans-Tag",
        2002,
        CH_UR,
        CH_UR_LAW,
    ),
    // Vaud.
    canton(
        "Good Friday",
        "le Vendredi-Saint",
        Rule::easter(GOOD_FRIDAY),
        2006,
        CH_VD,
        CH_VD_LAW,
    ),
    earlier_years_unread("Good Friday", "le Vendredi-Saint", 2005, CH_VD, CH_VD_LAW),
    canton(
        "Easter Monday",
        "le lundi de Pâques",
        Rule::easter(EASTER_MONDAY),
        2006,
        CH_VD,
        CH_VD_LAW,
    ),
    earlier_years_unread(
        "Easter Monday",
        "le lundi de Pâques",
        2005,
        CH_VD,
        CH_VD_LAW,
    ),
    canton(
        "Federal Fast Monday",
        "le lundi du Jeûne fédéral",
        FEDERAL_FAST_MONDAY,
        2006,
        CH_VD,
        CH_VD_LAW,
    ),
    earlier_years_unread(
        "Federal Fast Monday",
        "le lundi du Jeûne fédéral",
        2005,
        CH_VD,
        CH_VD_LAW,
    ),
    canton(
        "Berchtold's Day",
        "le 2 janvier",
        Rule::gregorian(1, 2),
        2008,
        CH_VD,
        CH_VD_LAW,
    ),
    earlier_years_unread("Berchtold's Day", "le 2 janvier", 2005, CH_VD, CH_VD_LAW),
    canton(
        "Whit Monday",
        "le lundi de Pentecôte",
        Rule::easter(WHIT_MONDAY),
        2008,
        CH_VD,
        CH_VD_LAW,
    ),
    earlier_years_unread(
        "Whit Monday",
        "le lundi de Pentecôte",
        2005,
        CH_VD,
        CH_VD_LAW,
    ),
    // Valais.
    canton(
        "St Joseph's Day",
        "Saint-Joseph",
        Rule::gregorian(3, 19),
        2017,
        CH_VS,
        CH_VS_LAW,
    ),
    earlier_years_unread("St Joseph's Day", "Saint-Joseph", 2016, CH_VS, CH_VS_LAW),
    canton(
        "Corpus Christi",
        "Fête-Dieu",
        Rule::easter(CORPUS_CHRISTI),
        2017,
        CH_VS,
        CH_VS_LAW,
    ),
    earlier_years_unread("Corpus Christi", "Fête-Dieu", 2016, CH_VS, CH_VS_LAW),
    canton(
        "Assumption",
        "Assomption",
        Rule::gregorian(8, 15),
        2017,
        CH_VS,
        CH_VS_LAW,
    ),
    earlier_years_unread("Assumption", "Assomption", 2016, CH_VS, CH_VS_LAW),
    canton(
        "All Saints' Day",
        "Toussaint",
        Rule::gregorian(11, 1),
        2016,
        CH_VS,
        CH_VS_LAW,
    ),
    earlier_years_unread("All Saints' Day", "Toussaint", 2015, CH_VS, CH_VS_LAW),
    canton(
        "Immaculate Conception",
        "Immaculée Conception",
        Rule::gregorian(12, 8),
        2016,
        CH_VS,
        CH_VS_LAW,
    ),
    earlier_years_unread(
        "Immaculate Conception",
        "Immaculée Conception",
        2015,
        CH_VS,
        CH_VS_LAW,
    ),
    // Zug.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        2004,
        CH_ZG,
        CH_ZG_LAW,
    ),
    earlier_years_unread("Good Friday", "Karfreitag", 2003, CH_ZG, CH_ZG_LAW),
    canton(
        "Corpus Christi",
        "Fronleichnam",
        Rule::easter(CORPUS_CHRISTI),
        2004,
        CH_ZG,
        CH_ZG_LAW,
    ),
    earlier_years_unread("Corpus Christi", "Fronleichnam", 2003, CH_ZG, CH_ZG_LAW),
    canton(
        "Assumption",
        "Maria Himmelfahrt",
        Rule::gregorian(8, 15),
        2004,
        CH_ZG,
        CH_ZG_LAW,
    ),
    earlier_years_unread("Assumption", "Maria Himmelfahrt", 2003, CH_ZG, CH_ZG_LAW),
    canton(
        "All Saints' Day",
        "Allerheiligen",
        Rule::gregorian(11, 1),
        2004,
        CH_ZG,
        CH_ZG_LAW,
    ),
    earlier_years_unread("All Saints' Day", "Allerheiligen", 2003, CH_ZG, CH_ZG_LAW),
    canton(
        "Immaculate Conception",
        "Maria Empfängnis",
        Rule::gregorian(12, 8),
        2004,
        CH_ZG,
        CH_ZG_LAW,
    ),
    earlier_years_unread(
        "Immaculate Conception",
        "Maria Empfängnis",
        2003,
        CH_ZG,
        CH_ZG_LAW,
    ),
    // Zurich.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        2026,
        CH_ZH,
        CH_ZH_LAW,
    ),
    earlier_years_unread("Good Friday", "Karfreitag", 2025, CH_ZH, CH_ZH_LAW),
    canton(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        2026,
        CH_ZH,
        CH_ZH_LAW,
    ),
    earlier_years_unread("Easter Monday", "Ostermontag", 2025, CH_ZH, CH_ZH_LAW),
    canton(
        "Labour Day",
        "1. Mai",
        Rule::gregorian(5, 1),
        2026,
        CH_ZH,
        CH_ZH_LAW,
    ),
    earlier_years_unread("Labour Day", "1. Mai", 2025, CH_ZH, CH_ZH_LAW),
    canton(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        2026,
        CH_ZH,
        CH_ZH_LAW,
    ),
    earlier_years_unread("Whit Monday", "Pfingstmontag", 2025, CH_ZH, CH_ZH_LAW),
    canton(
        "St Stephen's Day",
        "Stephanstag",
        Rule::gregorian(12, 26),
        2026,
        CH_ZH,
        CH_ZH_LAW,
    ),
    earlier_years_unread("St Stephen's Day", "Stephanstag", 2025, CH_ZH, CH_ZH_LAW),
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
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "Arbeitsgesetz (SR 822.11), Art. 20a Abs. 1, in the consolidation of 1 September 2023 \
              on Fedlex, retrieved 2026-09-29; Bundesverfassung (SR 101), Art. 110 Abs. 3, and \
              the Verordnung vom 30. Mai 1994 über den Bundesfeiertag, for 1 August, not read; \
              the cantons' laws, each cited on its entries, read 2026-09-29 in the cantons' \
              collections, and for Jura, Schwyz and Zurich, whose laws are PDF only, the \
              German Wikipedia's \"Feiertage in der Schweiz\" and the Canton of Zurich's page \
              (secondary), as docs/systems/switzerland-holidays.md lists them",
};
