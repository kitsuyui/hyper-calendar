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
//! is [`Kind::Public`]; a public rest day the law does not make equal to
//! Sunday, as Lucerne's 8 December or Ticino's giorni festivi "non
//! parificati", is [`Kind::Observance`]. Only days the law keeps in the
//! whole canton are carried: the days Aargau keeps in some districts,
//! Fribourg in its Catholic or Reformed communes, Solothurn outside the
//! Bucheggberg and Appenzell Innerrhoden in its inner part need a scope
//! finer than a canton. Jura, Schwyz and Zurich publish their current laws
//! as PDF only; their texts were read in LexFind's copies, and Zurich's of
//! 2000 to 2004 also in ZH-Lex's HTML.

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;

use super::weekends;
use crate::computus::offsets::{
    ASCENSION, CORPUS_CHRISTI, EASTER_MONDAY, GOOD_FRIDAY, WHIT_MONDAY,
};
use crate::rule::{Days, HolidayRule, Kind, Rule, RuleSet, SourceDate, Subdivisions, joined};

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

/// A day a canton's law makes equal to Sunday. Its years are the call's:
/// `.years` from the year a law read established it, or `.read_from` the
/// commencement of the law read, where the day is older than it and an
/// earlier law, not read, kept it or did not.
const fn canton(
    name: &'static str,
    local_name: &'static str,
    rule: Rule,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::fixed_public(name, local_name, rule)
        .in_regions(region)
        .cited(source)
}

/// A public rest day of a canton's law that the law does not make equal
/// to Sunday, with its years as [`canton`]'s.
const fn canton_rest_day(
    name: &'static str,
    local_name: &'static str,
    rule: Rule,
    region: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::observance(name, local_name, rule)
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
const CH_JU_LAW: &str = "Loi sur les jours fériés officiels et le repos dominical du 31 août 2022, RSJU 555.1, arts. 3 and 4, in force from 1 January 2023, LexFind's text of the official publication (secondary; RSJU publishes the law as PDF only) (https://www.lexfind.ch/api/fe/de/texts-of-law/compare?id1=224661&id2=224661), retrieved 2026-09-29";
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
const CH_SZ_LAW: &str = "Ruhetagsgesetz vom 21. November 2001, SRSZ 545.110, § 2, in the version in force from 1 July 2018, its lists of days unchanged since 1 January 2002, LexFind's text of the official publication (secondary; the Canton publishes the law as PDF only) (https://www.lexfind.ch/api/fe/de/texts-of-law/compare?id1=85104&id2=85104), retrieved 2026-09-29";
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
const CH_ZH_LAW: &str = "Ruhetags- und Ladenöffnungsgesetz vom 26. Juni 2000, LS 822.4, § 1, in force from 1 December 2000: the version to 30 April 2004 in ZH-Lex's HTML text (https://www.notes.zh.ch/appl/zhlex_r.nsf/WebRT/C1256C610039641BC1256036003AF0BE), and the version of 1 May 2004, § 1 unchanged, in LexFind's text (secondary), retrieved 2026-09-29";

/// Every canton, by ISO 3166-2 code.
const CANTONS: &[&str] = &[
    "CH-AG", "CH-AI", "CH-AR", "CH-BE", "CH-BL", "CH-BS", "CH-FR", "CH-GE", "CH-GL", "CH-GR",
    "CH-JU", "CH-LU", "CH-NE", "CH-NW", "CH-OW", "CH-SG", "CH-SH", "CH-SO", "CH-SZ", "CH-TG",
    "CH-TI", "CH-UR", "CH-VD", "CH-VS", "CH-ZG", "CH-ZH",
];

/// The first year every canton's text read answers for: Jura's law of
/// 2022, in force from 1 January 2023, is the latest.
const EVERY_CANTON_READ: i32 = 2023;

/// The names of New Year's Day, Ascension and Christmas Day that a canton's
/// law uses, in the language the law was read in. Where the law was read in
/// German the names are German; where it was read in French or Italian, the
/// words of its list of holidays; and where the French text was not read,
/// the English names stand (empty `local_name`), not the German.
type KeptNames = (&'static str, &'static str, &'static str);
const DE_NAMES: KeptNames = ("Neujahrstag", "Auffahrt", "Weihnachtstag");
/// Loi sur les jours fériés (Geneva), art. 1: "1er Janvier", "Ascension",
/// "Noël".
const GE_NAMES: KeptNames = ("1er Janvier", "Ascension", "Noël");
/// Loi sur les jours fériés officiels et le repos dominical (Jura), arts. 3
/// and 4: "Nouvel-An", "l'Ascension", "Noël".
const JU_NAMES: KeptNames = ("Nouvel-An", "Ascension", "Noël");
/// Loi sur le dimanche et les jours fériés (Neuchâtel), art. 3: "le
/// 1er janvier", "l'Ascension", "le jour de Noël".
const NE_NAMES: KeptNames = ("1er janvier", "Ascension", "Noël");
/// LALL (Ticino), art. 6: "Capodanno", "Ascensione", "Natale".
const TI_NAMES: KeptNames = ("Capodanno", "Ascensione", "Natale");
/// Fribourg's, Vaud's and Valais's texts of these three days were not read in
/// French in the form that gives their names.
const EN_NAMES: KeptNames = ("", "", "");

/// The three days every canton's law read keeps — New Year's Day,
/// Ascension and Christmas Day — in each canton, from the latest first
/// year its text read gives for any of its days, the years before a gap;
/// and nationwide, where the days are every canton's, from
/// [`EVERY_CANTON_READ`].
macro_rules! every_canton_keeps {
    ($(($region:expr, $law:expr, $first:expr, $names:expr)),* $(,)?) => {
        [
            HolidayRule::public("New Year's Day", "Neujahrstag", Rule::gregorian(1, 1))
                .except_in(CANTONS)
                .read_from(EVERY_CANTON_READ),
            HolidayRule::public("Ascension", "Auffahrt", Rule::easter(ASCENSION))
                .except_in(CANTONS)
                .read_from(EVERY_CANTON_READ),
            HolidayRule::public("Christmas Day", "Weihnachtstag", Rule::gregorian(12, 25))
                .except_in(CANTONS)
                .read_from(EVERY_CANTON_READ),
            $(
                canton("New Year's Day", $names.0, Rule::gregorian(1, 1), $region, $law)
                    .read_from($first),
                canton("Ascension", $names.1, Rule::easter(ASCENSION), $region, $law)
                    .read_from($first),
                canton("Christmas Day", $names.2, Rule::gregorian(12, 25), $region, $law)
                    .read_from($first),
            )*
        ]
    };
}

/// The days of [`every_canton_keeps`].
static KEPT_EVERYWHERE: [HolidayRule; 3 + 3 * 26] = every_canton_keeps![
    (CH_AG, CH_AG_LAW, 2013, DE_NAMES),
    (CH_AI, CH_AI_LAW, 2011, DE_NAMES),
    (CH_AR, CH_AR_LAW, 1967, DE_NAMES),
    (CH_BE, CH_BE_LAW, 1998, DE_NAMES),
    (CH_BL, CH_BL_LAW, 2011, DE_NAMES),
    (CH_BS, CH_BS_LAW, 1994, DE_NAMES),
    (CH_FR, CH_FR_LAW, 2011, EN_NAMES),
    (CH_GE, CH_GE_LAW, 1991, GE_NAMES),
    (CH_GL, CH_GL_LAW, 2013, DE_NAMES),
    (CH_GR, CH_GR_LAW, 2006, DE_NAMES),
    (CH_JU, CH_JU_LAW, 2023, JU_NAMES),
    (CH_LU, CH_LU_LAW, 1998, DE_NAMES),
    (CH_NE, CH_NE_LAW, 2010, NE_NAMES),
    (CH_NW, CH_NW_LAW, 2006, DE_NAMES),
    (CH_OW, CH_OW_LAW, 2008, DE_NAMES),
    (CH_SG, CH_SG_LAW, 2005, DE_NAMES),
    (CH_SH, CH_SH_LAW, 2011, DE_NAMES),
    (CH_SO, CH_SO_LAW, 2016, DE_NAMES),
    (CH_SZ, CH_SZ_LAW, 2002, DE_NAMES),
    (CH_TG, CH_TG_LAW, 2003, DE_NAMES),
    (CH_TI, CH_TI_LAW, 2012, TI_NAMES),
    (CH_UR, CH_UR_LAW, 2003, DE_NAMES),
    (CH_VD, CH_VD_LAW, 2006, EN_NAMES),
    (CH_VS, CH_VS_LAW, 2017, EN_NAMES),
    (CH_ZG, CH_ZG_LAW, 2004, DE_NAMES),
    (CH_ZH, CH_ZH_LAW, 2001, DE_NAMES),
];

/// The Arbeitsgesetz's Art. 20a, in force from 1 August 2000.
const ARG_20A: &str = "Arbeitsgesetz (SR 822.11), Art. 20a Abs. 1, \"Der Bundesfeiertag ist den \
     Sonntagen gleichgestellt\", inserted by Ziff. I des BG vom 20. März 1998, in Kraft seit \
     1. Aug. 2000 (AS 2000 1569), Fedlex, consolidation of 1 September 2023, retrieved 2026-09-29; \
     the Verordnung vom 30. Mai 1994 über den Bundesfeiertag, not read, for the years before";

/// Every rule but [`KEPT_EVERYWHERE`].
const CH_OWN_RULES: &[HolidayRule] = &[
    // ── The federal day ──────────────────────────────────────────────────
    // A holiday from the Verordnung of 1994, not read; equal to Sunday by
    // the Arbeitsgesetz from 2000, read.
    HolidayRule::public("Swiss National Day", "Bundesfeier", Rule::gregorian(8, 1))
        .read_from(2000)
        .cited(ARG_20A),
    // ── Each canton's other days ────────────────────────────────────────
    // Aargau.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        CH_AG,
        CH_AG_LAW,
    )
    .read_from(2013),
    // Appenzell Innerrhoden.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        CH_AI,
        CH_AI_LAW,
    )
    .read_from(2011),
    canton(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        CH_AI,
        CH_AI_LAW,
    )
    .read_from(2011),
    canton(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        CH_AI,
        CH_AI_LAW,
    )
    .read_from(2011),
    canton(
        "Corpus Christi",
        "Fronleichnam",
        Rule::easter(CORPUS_CHRISTI),
        CH_AI,
        CH_AI_LAW,
    )
    .read_from(2011),
    canton(
        "St Stephen's Day",
        "Stephanstag",
        Rule::Unsettled(appenzell_innerrhoden_st_stephen),
        CH_AI,
        CH_AI_LAW,
    )
    .read_from(2011),
    canton_rest_day(
        "Assumption",
        "Maria Himmelfahrt",
        Rule::gregorian(8, 15),
        CH_AI,
        CH_AI_LAW,
    )
    .read_from(1982),
    canton_rest_day(
        "All Saints' Day",
        "Allerheiligen",
        Rule::gregorian(11, 1),
        CH_AI,
        CH_AI_LAW,
    )
    .read_from(1982),
    canton_rest_day(
        "Immaculate Conception",
        "Maria Empfängnis",
        Rule::gregorian(12, 8),
        CH_AI,
        CH_AI_LAW,
    )
    .read_from(1982),
    // Appenzell Ausserrhoden.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        CH_AR,
        CH_AR_LAW,
    )
    .read_from(1966),
    canton(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        CH_AR,
        CH_AR_LAW,
    )
    .read_from(1966),
    canton(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        CH_AR,
        CH_AR_LAW,
    )
    .read_from(1966),
    canton(
        "St Stephen's Day",
        "zweiter Weihnachtstag",
        Rule::Computed(appenzell_ausserrhoden_st_stephen),
        CH_AR,
        CH_AR_LAW,
    )
    .read_from(1967),
    // Bern.
    canton(
        "Berchtold's Day",
        "der 2. Januar",
        Rule::gregorian(1, 2),
        CH_BE,
        CH_BE_LAW,
    )
    .read_from(1998),
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        CH_BE,
        CH_BE_LAW,
    )
    .read_from(1998),
    canton(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        CH_BE,
        CH_BE_LAW,
    )
    .read_from(1998),
    canton(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        CH_BE,
        CH_BE_LAW,
    )
    .read_from(1997),
    canton(
        "St Stephen's Day",
        "der 26. Dezember",
        Rule::gregorian(12, 26),
        CH_BE,
        CH_BE_LAW,
    )
    .read_from(1997),
    // Basel-Landschaft.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        CH_BL,
        CH_BL_LAW,
    )
    .read_from(2011),
    canton(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        CH_BL,
        CH_BL_LAW,
    )
    .read_from(2011),
    canton(
        "Labour Day",
        "1. Mai",
        Rule::gregorian(5, 1),
        CH_BL,
        CH_BL_LAW,
    )
    .read_from(2011),
    canton(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        CH_BL,
        CH_BL_LAW,
    )
    .read_from(2011),
    canton(
        "St Stephen's Day",
        "Stephanstag",
        Rule::gregorian(12, 26),
        CH_BL,
        CH_BL_LAW,
    )
    .read_from(2011),
    // Basel-Stadt.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        CH_BS,
        CH_BS_LAW,
    )
    .read_from(1994),
    canton(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        CH_BS,
        CH_BS_LAW,
    )
    .read_from(1994),
    canton(
        "Labour Day",
        "1. Mai",
        Rule::gregorian(5, 1),
        CH_BS,
        CH_BS_LAW,
    )
    .read_from(1994),
    canton(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        CH_BS,
        CH_BS_LAW,
    )
    .read_from(1994),
    canton(
        "St Stephen's Day",
        "Stephanstag",
        Rule::gregorian(12, 26),
        CH_BS,
        CH_BS_LAW,
    )
    .read_from(1994),
    // Fribourg.
    canton(
        "Good Friday",
        "Vendredi-Saint",
        Rule::easter(GOOD_FRIDAY),
        CH_FR,
        CH_FR_LAW,
    )
    .read_from(2011),
    // Geneva.
    canton(
        "Good Friday",
        "Vendredi saint",
        Rule::easter(GOOD_FRIDAY),
        CH_GE,
        CH_GE_LAW,
    )
    .read_from(1991),
    canton(
        "Easter Monday",
        "Lundi de Pâques",
        Rule::easter(EASTER_MONDAY),
        CH_GE,
        CH_GE_LAW,
    )
    .read_from(1991),
    canton(
        "Whit Monday",
        "Lundi de Pentecôte",
        Rule::easter(WHIT_MONDAY),
        CH_GE,
        CH_GE_LAW,
    )
    .read_from(1991),
    canton(
        "Geneva Fast",
        "Jeûne genevois",
        GENEVA_FAST,
        CH_GE,
        CH_GE_LAW,
    )
    .read_from(1991),
    canton(
        "Restoration of the Republic",
        "31 Décembre, anniversaire de la restauration de la République",
        Rule::gregorian(12, 31),
        CH_GE,
        CH_GE_LAW,
    )
    .read_from(1991),
    // Glarus.
    canton(
        "Näfels Pilgrimage",
        "Fahrtsfest",
        Rule::Computed(nafels_pilgrimage),
        CH_GL,
        CH_GL_LAW,
    )
    .read_from(2013),
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        CH_GL,
        CH_GL_LAW,
    )
    .read_from(2013),
    canton(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        CH_GL,
        CH_GL_LAW,
    )
    .read_from(2013),
    canton(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        CH_GL,
        CH_GL_LAW,
    )
    .read_from(2012),
    canton(
        "All Saints' Day",
        "Allerheiligen",
        Rule::gregorian(11, 1),
        CH_GL,
        CH_GL_LAW,
    )
    .read_from(2012),
    canton(
        "St Stephen's Day",
        "Stephanstag",
        Rule::gregorian(12, 26),
        CH_GL,
        CH_GL_LAW,
    )
    .read_from(2012),
    // Graubünden.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        CH_GR,
        CH_GR_LAW,
    )
    .read_from(2006),
    canton(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        CH_GR,
        CH_GR_LAW,
    )
    .read_from(2006),
    canton(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        CH_GR,
        CH_GR_LAW,
    )
    .read_from(2006),
    canton(
        "St Stephen's Day",
        "Stefanstag",
        Rule::gregorian(12, 26),
        CH_GR,
        CH_GR_LAW,
    )
    .read_from(2006),
    // Jura.
    canton(
        "Good Friday",
        "Vendredi-Saint",
        Rule::easter(GOOD_FRIDAY),
        CH_JU,
        CH_JU_LAW,
    )
    .read_from(2023),
    canton(
        "Easter Monday",
        "Lundi de Pâques",
        Rule::easter(EASTER_MONDAY),
        CH_JU,
        CH_JU_LAW,
    )
    .read_from(2023),
    canton(
        "Labour Day",
        "1er mai",
        Rule::gregorian(5, 1),
        CH_JU,
        CH_JU_LAW,
    )
    .read_from(2023),
    canton(
        "Whit Monday",
        "Lundi de Pentecôte",
        Rule::easter(WHIT_MONDAY),
        CH_JU,
        CH_JU_LAW,
    )
    .read_from(2023),
    canton(
        "Corpus Christi",
        "Fête-Dieu",
        Rule::easter(CORPUS_CHRISTI),
        CH_JU,
        CH_JU_LAW,
    )
    .read_from(2023),
    canton_rest_day(
        "Berchtold's Day",
        "2 janvier",
        Rule::gregorian(1, 2),
        CH_JU,
        CH_JU_LAW,
    )
    .read_from(2023),
    canton_rest_day(
        "Assumption",
        "Assomption",
        Rule::gregorian(8, 15),
        CH_JU,
        CH_JU_LAW,
    )
    .read_from(2023),
    canton_rest_day(
        "All Saints' Day",
        "Toussaint",
        Rule::gregorian(11, 1),
        CH_JU,
        CH_JU_LAW,
    )
    .read_from(2023),
    // Art. 3 lit. b names the day only as "le 23 juin", and Art. 4 does not
    // equate it with Sunday.
    canton_rest_day(
        "23 June",
        "le 23 juin",
        Rule::gregorian(6, 23),
        CH_JU,
        CH_JU_LAW,
    )
    .read_from(2023),
    // Lucerne.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        CH_LU,
        CH_LU_LAW,
    )
    .read_from(1998),
    canton(
        "Corpus Christi",
        "Fronleichnam",
        Rule::easter(CORPUS_CHRISTI),
        CH_LU,
        CH_LU_LAW,
    )
    .read_from(1998),
    canton(
        "Assumption",
        "Mariä Himmelfahrt",
        Rule::gregorian(8, 15),
        CH_LU,
        CH_LU_LAW,
    )
    .read_from(1997),
    canton(
        "All Saints' Day",
        "Allerheiligen",
        Rule::gregorian(11, 1),
        CH_LU,
        CH_LU_LAW,
    )
    .read_from(1997),
    canton(
        "St Stephen's Day",
        "Stefanstag",
        Rule::gregorian(12, 26),
        CH_LU,
        CH_LU_LAW,
    )
    .read_from(1997),
    canton_rest_day(
        "Immaculate Conception",
        "Mariä Empfängnis",
        Rule::gregorian(12, 8),
        CH_LU,
        CH_LU_LAW,
    )
    .read_from(1997),
    // Neuchâtel.
    canton(
        "2 January",
        "le 2 janvier",
        Rule::Computed(neuchatel_second_january),
        CH_NE,
        CH_NE_LAW,
    )
    .read_from(2010),
    canton(
        "1 March",
        "le 1er mars",
        Rule::gregorian(3, 1),
        CH_NE,
        CH_NE_LAW,
    )
    .read_from(2010),
    canton(
        "Labour Day",
        "le 1er mai",
        Rule::gregorian(5, 1),
        CH_NE,
        CH_NE_LAW,
    )
    .read_from(2010),
    canton(
        "Good Friday",
        "Vendredi Saint",
        Rule::easter(GOOD_FRIDAY),
        CH_NE,
        CH_NE_LAW,
    )
    .read_from(2010),
    canton(
        "St Stephen's Day",
        "le 26 décembre",
        Rule::Computed(neuchatel_st_stephen),
        CH_NE,
        CH_NE_LAW,
    )
    .read_from(2010),
    // Nidwalden.
    canton_rest_day(
        "St Joseph's Day",
        "Josefstag",
        Rule::gregorian(3, 19),
        CH_NW,
        CH_NW_LAW,
    )
    .read_from(2006),
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        CH_NW,
        CH_NW_LAW,
    )
    .read_from(2006),
    canton(
        "Corpus Christi",
        "Fronleichnam",
        Rule::easter(CORPUS_CHRISTI),
        CH_NW,
        CH_NW_LAW,
    )
    .read_from(2006),
    canton(
        "Assumption",
        "Maria Himmelfahrt",
        Rule::gregorian(8, 15),
        CH_NW,
        CH_NW_LAW,
    )
    .read_from(2006),
    canton(
        "All Saints' Day",
        "Allerheiligen",
        Rule::gregorian(11, 1),
        CH_NW,
        CH_NW_LAW,
    )
    .read_from(2005),
    canton(
        "Immaculate Conception",
        "Maria Empfängnis",
        Rule::gregorian(12, 8),
        CH_NW,
        CH_NW_LAW,
    )
    .read_from(2005),
    // Obwalden.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        CH_OW,
        CH_OW_LAW,
    )
    .read_from(2008),
    canton(
        "Corpus Christi",
        "Fronleichnam",
        Rule::easter(CORPUS_CHRISTI),
        CH_OW,
        CH_OW_LAW,
    )
    .read_from(2008),
    canton(
        "Assumption",
        "Mariä Himmelfahrt",
        Rule::gregorian(8, 15),
        CH_OW,
        CH_OW_LAW,
    )
    .read_from(2007),
    canton(
        "All Saints' Day",
        "Allerheiligen",
        Rule::gregorian(11, 1),
        CH_OW,
        CH_OW_LAW,
    )
    .read_from(2007),
    canton(
        "Immaculate Conception",
        "Mariä Empfängnis",
        Rule::gregorian(12, 8),
        CH_OW,
        CH_OW_LAW,
    )
    .read_from(2007),
    canton_rest_day(
        "St Nicholas of Flüe",
        "Bruderklausenfest",
        Rule::gregorian(9, 25),
        CH_OW,
        CH_OW_LAW,
    )
    .read_from(2007),
    // St. Gallen.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        CH_SG,
        CH_SG_LAW,
    )
    .read_from(2005),
    canton(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        CH_SG,
        CH_SG_LAW,
    )
    .read_from(2005),
    canton(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        CH_SG,
        CH_SG_LAW,
    )
    .read_from(2005),
    canton(
        "All Saints' Day",
        "Allerheiligen",
        Rule::gregorian(11, 1),
        CH_SG,
        CH_SG_LAW,
    )
    .read_from(2004),
    canton(
        "St Stephen's Day",
        "Stefanstag",
        Rule::gregorian(12, 26),
        CH_SG,
        CH_SG_LAW,
    )
    .read_from(2004),
    // Schaffhausen.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        CH_SH,
        CH_SH_LAW,
    )
    .read_from(2011),
    canton(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        CH_SH,
        CH_SH_LAW,
    )
    .read_from(2011),
    canton(
        "Labour Day",
        "1. Mai",
        Rule::gregorian(5, 1),
        CH_SH,
        CH_SH_LAW,
    )
    .read_from(2011),
    canton(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        CH_SH,
        CH_SH_LAW,
    )
    .read_from(2011),
    canton(
        "St Stephen's Day",
        "Stephanstag",
        Rule::gregorian(12, 26),
        CH_SH,
        CH_SH_LAW,
    )
    .read_from(2011),
    // Solothurn.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        CH_SO,
        CH_SO_LAW,
    )
    .read_from(2016),
    // § 46 Abs. 1 a): "der 1. Mai (ab 12 Uhr)", equal to Sunday from noon.
    canton(
        "Labour Day, from noon",
        "1. Mai (ab 12 Uhr)",
        Rule::gregorian(5, 1),
        CH_SO,
        CH_SO_LAW,
    )
    .of_kind(Kind::HalfDay)
    .read_from(2016),
    // Schwyz.
    canton(
        "St Joseph's Day",
        "Josefstag",
        Rule::gregorian(3, 19),
        CH_SZ,
        CH_SZ_LAW,
    )
    .read_from(2002),
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        CH_SZ,
        CH_SZ_LAW,
    )
    .read_from(2002),
    canton(
        "Corpus Christi",
        "Fronleichnam",
        Rule::easter(CORPUS_CHRISTI),
        CH_SZ,
        CH_SZ_LAW,
    )
    .read_from(2002),
    canton(
        "Assumption",
        "Mariä Himmelfahrt",
        Rule::gregorian(8, 15),
        CH_SZ,
        CH_SZ_LAW,
    )
    .read_from(2002),
    canton(
        "All Saints' Day",
        "Allerheiligen",
        Rule::gregorian(11, 1),
        CH_SZ,
        CH_SZ_LAW,
    )
    .read_from(2002),
    canton_rest_day(
        "Epiphany",
        "Heilige Drei Könige",
        Rule::gregorian(1, 6),
        CH_SZ,
        CH_SZ_LAW,
    )
    .read_from(2002),
    canton_rest_day(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        CH_SZ,
        CH_SZ_LAW,
    )
    .read_from(2002),
    canton_rest_day(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        CH_SZ,
        CH_SZ_LAW,
    )
    .read_from(2002),
    canton_rest_day(
        "Immaculate Conception",
        "Mariä Empfängnis",
        Rule::gregorian(12, 8),
        CH_SZ,
        CH_SZ_LAW,
    )
    .read_from(2002),
    canton_rest_day(
        "St Stephen's Day",
        "Stephanstag",
        Rule::gregorian(12, 26),
        CH_SZ,
        CH_SZ_LAW,
    )
    .read_from(2002),
    // Thurgau.
    canton(
        "Berchtold's Day",
        "2. Januar",
        Rule::gregorian(1, 2),
        CH_TG,
        CH_TG_LAW,
    )
    .read_from(2003),
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        CH_TG,
        CH_TG_LAW,
    )
    .read_from(2003),
    canton(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        CH_TG,
        CH_TG_LAW,
    )
    .read_from(2003),
    canton(
        "Labour Day",
        "1. Mai",
        Rule::gregorian(5, 1),
        CH_TG,
        CH_TG_LAW,
    )
    .read_from(2003),
    canton(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        CH_TG,
        CH_TG_LAW,
    )
    .read_from(2003),
    canton(
        "St Stephen's Day",
        "26. Dezember",
        Rule::gregorian(12, 26),
        CH_TG,
        CH_TG_LAW,
    )
    .read_from(2003),
    // Ticino.
    canton(
        "Epiphany",
        "Epifania",
        Rule::gregorian(1, 6),
        CH_TI,
        CH_TI_LAW,
    )
    .read_from(2012),
    canton(
        "Easter Monday",
        "Lunedì di Pasqua",
        Rule::easter(EASTER_MONDAY),
        CH_TI,
        CH_TI_LAW,
    )
    .read_from(2012),
    canton(
        "Assumption",
        "Assunzione",
        Rule::gregorian(8, 15),
        CH_TI,
        CH_TI_LAW,
    )
    .read_from(2011),
    canton(
        "All Saints' Day",
        "Ognissanti",
        Rule::gregorian(11, 1),
        CH_TI,
        CH_TI_LAW,
    )
    .read_from(2011),
    canton(
        "St Stephen's Day",
        "Santo Stefano",
        Rule::gregorian(12, 26),
        CH_TI,
        CH_TI_LAW,
    )
    .read_from(2011),
    canton_rest_day(
        "St Joseph's Day",
        "San Giuseppe",
        Rule::gregorian(3, 19),
        CH_TI,
        CH_TI_LAW,
    )
    .read_from(2010),
    canton_rest_day(
        "Labour Day",
        "1° Maggio",
        Rule::gregorian(5, 1),
        CH_TI,
        CH_TI_LAW,
    )
    .read_from(2010),
    canton_rest_day(
        "Whit Monday",
        "Lunedì di Pentecoste",
        Rule::easter(WHIT_MONDAY),
        CH_TI,
        CH_TI_LAW,
    )
    .read_from(2010),
    canton_rest_day(
        "Corpus Christi",
        "Corpus Domini",
        Rule::easter(CORPUS_CHRISTI),
        CH_TI,
        CH_TI_LAW,
    )
    .read_from(2010),
    canton_rest_day(
        "Saints Peter and Paul",
        "SS. Pietro e Paolo",
        Rule::gregorian(6, 29),
        CH_TI,
        CH_TI_LAW,
    )
    .read_from(2010),
    canton_rest_day(
        "Immaculate Conception",
        "Immacolata",
        Rule::gregorian(12, 8),
        CH_TI,
        CH_TI_LAW,
    )
    .read_from(2010),
    // Uri.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        CH_UR,
        CH_UR_LAW,
    )
    .read_from(2002),
    canton(
        "Corpus Christi",
        "Fronleichnam",
        Rule::easter(CORPUS_CHRISTI),
        CH_UR,
        CH_UR_LAW,
    )
    .read_from(2002),
    canton(
        "Assumption",
        "Mariä Himmelfahrt",
        Rule::gregorian(8, 15),
        CH_UR,
        CH_UR_LAW,
    )
    .read_from(2002),
    canton(
        "All Saints' Day",
        "Allerheiligen",
        Rule::gregorian(11, 1),
        CH_UR,
        CH_UR_LAW,
    )
    .read_from(2002),
    canton(
        "Immaculate Conception",
        "Mariä Empfängnis",
        Rule::gregorian(12, 8),
        CH_UR,
        CH_UR_LAW,
    )
    .read_from(2002),
    canton_rest_day(
        "Epiphany",
        "Dreikönigen",
        Rule::gregorian(1, 6),
        CH_UR,
        CH_UR_LAW,
    )
    .read_from(2003),
    canton_rest_day(
        "St Joseph's Day",
        "Sankt-Josefs-Tag",
        Rule::gregorian(3, 19),
        CH_UR,
        CH_UR_LAW,
    )
    .read_from(2003),
    canton_rest_day(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        CH_UR,
        CH_UR_LAW,
    )
    .read_from(2003),
    canton_rest_day(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        CH_UR,
        CH_UR_LAW,
    )
    .read_from(2003),
    canton_rest_day(
        "St Stephen's Day",
        "Sankt-Stefans-Tag",
        Rule::gregorian(12, 26),
        CH_UR,
        CH_UR_LAW,
    )
    .read_from(2003),
    // Vaud.
    canton(
        "Good Friday",
        "le Vendredi-Saint",
        Rule::easter(GOOD_FRIDAY),
        CH_VD,
        CH_VD_LAW,
    )
    .read_from(2006),
    canton(
        "Easter Monday",
        "le lundi de Pâques",
        Rule::easter(EASTER_MONDAY),
        CH_VD,
        CH_VD_LAW,
    )
    .read_from(2006),
    canton(
        "Federal Fast Monday",
        "le lundi du Jeûne fédéral",
        FEDERAL_FAST_MONDAY,
        CH_VD,
        CH_VD_LAW,
    )
    .read_from(2006),
    canton(
        "Berchtold's Day",
        "le 2 janvier",
        Rule::gregorian(1, 2),
        CH_VD,
        CH_VD_LAW,
    )
    .years(Some(2008), None),
    // The law read in its text of 2006, before the amendment of 2007, keeps
    // neither this day nor Whit Monday; the law before it was not read.
    canton(
        "Berchtold's Day",
        "le 2 janvier",
        Rule::NO_DAY,
        CH_VD,
        CH_VD_LAW,
    )
    .years(None, Some(2007))
    .read_from(2006),
    canton(
        "Whit Monday",
        "le lundi de Pentecôte",
        Rule::easter(WHIT_MONDAY),
        CH_VD,
        CH_VD_LAW,
    )
    .years(Some(2008), None),
    canton(
        "Whit Monday",
        "le lundi de Pentecôte",
        Rule::NO_DAY,
        CH_VD,
        CH_VD_LAW,
    )
    .years(None, Some(2007))
    .read_from(2006),
    // Valais.
    canton(
        "St Joseph's Day",
        "Saint-Joseph",
        Rule::gregorian(3, 19),
        CH_VS,
        CH_VS_LAW,
    )
    .read_from(2017),
    canton(
        "Corpus Christi",
        "Fête-Dieu",
        Rule::easter(CORPUS_CHRISTI),
        CH_VS,
        CH_VS_LAW,
    )
    .read_from(2017),
    canton(
        "Assumption",
        "Assomption",
        Rule::gregorian(8, 15),
        CH_VS,
        CH_VS_LAW,
    )
    .read_from(2017),
    canton(
        "All Saints' Day",
        "Toussaint",
        Rule::gregorian(11, 1),
        CH_VS,
        CH_VS_LAW,
    )
    .read_from(2016),
    canton(
        "Immaculate Conception",
        "Immaculée Conception",
        Rule::gregorian(12, 8),
        CH_VS,
        CH_VS_LAW,
    )
    .read_from(2016),
    // Zug.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        CH_ZG,
        CH_ZG_LAW,
    )
    .read_from(2004),
    canton(
        "Corpus Christi",
        "Fronleichnam",
        Rule::easter(CORPUS_CHRISTI),
        CH_ZG,
        CH_ZG_LAW,
    )
    .read_from(2004),
    canton(
        "Assumption",
        "Maria Himmelfahrt",
        Rule::gregorian(8, 15),
        CH_ZG,
        CH_ZG_LAW,
    )
    .read_from(2004),
    canton(
        "All Saints' Day",
        "Allerheiligen",
        Rule::gregorian(11, 1),
        CH_ZG,
        CH_ZG_LAW,
    )
    .read_from(2004),
    canton(
        "Immaculate Conception",
        "Maria Empfängnis",
        Rule::gregorian(12, 8),
        CH_ZG,
        CH_ZG_LAW,
    )
    .read_from(2004),
    // Zurich.
    canton(
        "Good Friday",
        "Karfreitag",
        Rule::easter(GOOD_FRIDAY),
        CH_ZH,
        CH_ZH_LAW,
    )
    .read_from(2001),
    canton(
        "Easter Monday",
        "Ostermontag",
        Rule::easter(EASTER_MONDAY),
        CH_ZH,
        CH_ZH_LAW,
    )
    .read_from(2001),
    canton(
        "Labour Day",
        "1. Mai",
        Rule::gregorian(5, 1),
        CH_ZH,
        CH_ZH_LAW,
    )
    .read_from(2001),
    canton(
        "Whit Monday",
        "Pfingstmontag",
        Rule::easter(WHIT_MONDAY),
        CH_ZH,
        CH_ZH_LAW,
    )
    .read_from(2001),
    canton(
        "St Stephen's Day",
        "Stephanstag",
        Rule::gregorian(12, 26),
        CH_ZH,
        CH_ZH_LAW,
    )
    .read_from(2000),
];

/// The table's rules: [`CH_OWN_RULES`] and [`KEPT_EVERYWHERE`].
static CH_RULES: [HolidayRule; CH_OWN_RULES.len() + KEPT_EVERYWHERE.len()] =
    joined(&[CH_OWN_RULES, &KEPT_EVERYWHERE]);

/// Switzerland.
pub static SWITZERLAND: RuleSet = RuleSet {
    code: "CH",
    english_name: "Switzerland",
    rules: &CH_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: weekends::CH,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "Arbeitsgesetz (SR 822.11), Art. 20a Abs. 1, in the consolidation of 1 September 2023 \
              on Fedlex, retrieved 2026-09-29; Bundesverfassung (SR 101), Art. 110 Abs. 3, and \
              the Verordnung vom 30. Mai 1994 über den Bundesfeiertag, for 1 August, not read; \
              the cantons' laws, each cited on its entries, read 2026-09-29 in the cantons' \
              collections, and for Jura, Schwyz and Zurich, whose laws are PDF only, the \
              German Wikipedia's \"Feiertage in der Schweiz\" and the Canton of Zurich's page \
              (secondary), as docs/systems/switzerland-holidays.md lists them",
    subdivisions: Subdivisions::Read(&[]),
};
