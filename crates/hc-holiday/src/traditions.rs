//! The cross-cutting religious and traditional cycles.
//!
//! National tables depend on these, so they come first: a country that keeps
//! Good Friday is keeping the Western computus, and a country that keeps
//! Eid al-Fiṭr is keeping 1 Shawwāl. Each tradition here is an ordinary
//! [`RuleSet`], evaluated by the same engine as any country, and a country
//! table that wanted the whole of one could simply concatenate it.
//!
//! Every entry is [`Kind::Religious`] or [`Kind::Observance`] — a tradition
//! does not give anyone a day off; a statute does — so evaluating one of
//! these sets and asking `is_holiday` will correctly say no. Ask
//! [`HolidayCalendar::on`](crate::engine::HolidayCalendar::on) instead.
//!
//! # What is firm and what is not
//!
//! | Tradition | Dating | Firmness |
//! | --- | --- | --- |
//! | Christian, Western | Gregorian computus, Gregorian fixed feasts | exact |
//! | Christian, Orthodox | Julian computus, Julian fixed feasts | exact |
//! | Christian, Orthodox, Revised Julian | Julian computus, fixed feasts in the Revised Julian calendar | exact |
//! | Ethiopian Orthodox | Ethiopic calendar; the Bahire Hasab cycle as offsets from the Julian-computus Pascha | exact as stated; a feast kept on a fixed Gregorian date by practice is not modelled |
//! | Coptic Orthodox | Coptic calendar; the paschal cycle as offsets from the Julian-computus Pascha | exact |
//! | Islamic, with the Shia days of Tasu'a, Arba'een and Eid al-Ghadir | tabular civil Hijri | **approximate** — the observed date is a sighting decision |
//! | Jewish | arithmetic Hebrew calendar | exact; the day begins at the preceding sunset, which this crate does not model |
//! | Bahá'í | the Badíʿ calendar as kept — arithmetic to 171 BE, the Bahá'í World Centre's table for 172–221 BE; the Twin Holy Birthdays from the same table | exact through 19 March 2065, and a reported gap after, where the table ends |
//! | Buddhist, Thai | the Thai lunar calendar, `thai-lunar`, as Thailand publishes its year types | exact for 1992–2027, and reported gaps outside |
//! | Buddhist, East Asian | the Gregorian dates Japan keeps, and the Chinese lunisolar calendar for the lunar Birthday | exact |
//! | Buddhist, Tibetan | the Phugpa calendar, `tibetan`, on the day that bears the number; three tables, one per convention for a skipped or repeated number | exact to the arithmetic; in `buddhist-tibetan` a year in which the number is skipped or repeated is a reported gap, in `buddhist-tibetan-berzin` it is Berzin's day, and `buddhist-tibetan-henning`, on `tibetan-lochen`, is Henning's almanacs' placement |
//! | Buddhist, Thai uposatha | `thai-lunar`, the month's length deciding แรม 14 or 15 ค่ำ | exact for 1992–2027, and reported gaps outside |
//! | Chinese folk | Chinese lunisolar calendar and the solar terms | exact to the astronomical model |
//! | 小年, five regional tables | Chinese lunisolar calendar | exact to the astronomical model; which region keeps which day is the source's |
//! | Taoist | Chinese lunisolar calendar | exact to the astronomical model; from secondary sources |
//! | Korean folk | the Korean lunisolar calendar, `dangi`, and 동지 at the Korean meridian | exact to the astronomical model |
//! | Vietnamese folk | the Vietnamese lunisolar calendar, `vietnamese` | exact to the astronomical model |
//! | Hindu | the amānta Hindu lunisolar calendar at the national almanac's sunrise; each festival on the part of the day its tithi must hold | exact to the astronomical model, and to the conventions [`crate::hindu`] states — a regional almanac may keep a day differently |
//! | Wheel of the Year | the solstices and equinoxes on their Universal Time day; the cross-quarter days on their fixed Gregorian dates | exact as stated; a group may keep a quarter day on its local date or the nearest weekend, and the eve convention for Samhain is not modelled |
//! | Jain | the amānta Hindu lunisolar calendar at the national almanac's sunrise; Paryuṣaṇa and Daśa Lakṣaṇa counted back from their last days | **approximate** — Jain almanacs differ from the national one by a day in some years, as the source says |
//! | Shinto | fixed Gregorian dates, and 節分 as the day before 立春 at the Japanese meridian | exact; a shrine's own festival dates are not carried |
//! | 五節句 | fixed Gregorian dates, from 1873 | exact |
//! | お盆, 酉の市, 初午, 亥の子, 十日夜 | fixed Gregorian dates and the 酉, 午 and 亥 days of a Gregorian month from 1873, or days of the Japanese 旧暦 1844–2146, one table per reckoning | exact to the astronomical model; the 旧暦's tenth month of 2033, and every year outside 1844–2146, is a reported gap |
//! | Imperial court rites (宮中祭祀) | fixed Gregorian dates and the two equinox days at the Japanese meridian | exact for the Reiwa-era schedule the source gives; the rites tied to a reign change with it |
//! | Sikh | the Nanakshahi calendar of 2003 for the gurpurabs; the amānta Hindu lunisolar calendar for the three the 2003 calendar left on the Bikrami | exact for the Nanakshahi dates, which are fixed Gregorian dates, and for two of the lunar three; Hola Mohalla's sunrise is fitted |
//! | Sikh, SGPC | the Vikrami solar calendar and the amānta Hindu lunisolar calendar, for the observances whose Bikrami rule was read, and the SGPC's published days of Bandi Chhor Divas, 2010–2026 | exact to the astronomical model, but Hola Mohalla, whose sunrise is fitted; Bandi Chhor Divas outside 2010–2026 is a reported gap, and the SGPC's other gurpurabs are not carried |
//! | Zoroastrian | the Parsi schedule of feasts on each of the three reckonings — Fasli, Shahanshahi, Qadimi — as three tables, and the name-day feasts of the Zoroastrians of Iran on their thirty-day months from the civil Nowruz | exact: every feast is a fixed day of a fixed month, and each reckoning is arithmetic |
//! | Iranian festivals | the Solar Hijri calendar, `persian`: Tirgan, Mehregan, Yalda and Sadeh on their civil dates | exact to the calendar |
//! | Armenian Apostolic | Gregorian calendar and computus (Etchmiadzin), or Julian (the Patriarchate of Jerusalem), as two tables; the feasts on the Sunday nearest a date as a moved date | exact; the saints' days are not carried |
//! | Ember and Rogation Days | the Gregorian computus and fixed Gregorian dates, one table per church: the 1662 Prayer Book, *Common Worship*'s traditional weeks, the Roman rubrics of 1960 | exact as stated; *Common Worship*'s week before an ordination is the bishop's and not computed |
//! | Samaritan | the `samaritan` calendar, a modern calculation of the priesthood's | exact to that calculation, which puts the Passovers of 2016, 2025 and 2026 a day late; a reported gap outside 1900–2100 |
//! | Mandaean | the `mandaean` calendar of 365 days | exact: arithmetic |
//! | Yazidi | the Eastern calendar, which is the Julian, and Serêsal by its weekday rule | exact |
//! | Balinese Pawukon holy days | the 210-day Pawukon's concurrent weeks | exact: the cycle is arithmetic |
//! | Movable name days, Greek and Bulgarian | the Julian computus and the Revised Julian fixed dates | exact as stated; which names belong to a feast is custom, and only the principal ones are named |
//! | Qumran festivals | the 364-day year, `qumran` | exact to the scrolls' weekdays; the Gregorian date is the calendar's conventional epoch's |
//! | Plough Monday, Plough Sunday, Distaff Day | fixed Gregorian dates and the weekday after one | exact as stated; the regional variants are not carried |
//! | Chaharshanbe Suri | the Solar Hijri calendar, `persian`: the Tuesday before the year's last Wednesday | exact to the calendar |
//! | Tenrikyo | fixed Gregorian dates | exact for the present schedule; when the services left the lunar dates was not read |
//! | Friday the 13th | the Gregorian calendar | exact |
//! | Wednesdays on the eighth lunar day | the amānta Hindu lunisolar calendar at the national almanac's sunrise | exact to the astronomical model; a reported gap outside the calendar's years |
//! | Assyrian Church of the East | the Gregorian computus and fixed Gregorian dates, from 1965 | exact as stated; the Sundays of Moses and the order of the Elijah and Cross Sundays are not carried |
//! | *Common Worship* | the Gregorian computus and fixed Gregorian dates, with the transfers the Rules require | exact as stated; the years the Rules leave open are reported gaps |

use hc_astro::MEAN_TROPICAL_YEAR;
use hc_calendar::Calendar as _;
use hc_calendar::cycle::{branch, branch_day_on_or_after};
use hc_calendar::fixed::Moment;
use hc_calendar::{Month, Rd, Weekday};
use hc_calendars_indic::SiddhantaLunarCalendar;
use hc_calendars_lunar::{japanese_tenpo, tibetan};
use hc_calendars_regional::balinese_pawukon::{BalinesePawukonCalendar, PawukonDate};
use hc_calendars_solar::{bahai, gregorian, yazidi};
use hc_seasons::solar_terms::term_moment;
use hc_seasons::{Meridian, SolarTerm};

use crate::computus::Computus;
use crate::computus::offsets::{
    ASCENSION, ASH_WEDNESDAY, CORPUS_CHRISTI, DIVINE_MERCY_SUNDAY, EASTER_MONDAY, EASTER_SUNDAY,
    GOOD_FRIDAY, HOLY_SATURDAY, MAUNDY_THURSDAY, PALM_SUNDAY, PENTECOST, SACRED_HEART,
    TRINITY_SUNDAY, WHIT_MONDAY,
};
use crate::countries::read_all;
use crate::hindu::{
    AKSHAYA_TRITIYA, ANANT_CHATURDASHI, BUDDHA_PURNIMA, DIWALI, DRIK_HOLI, DRIK_HOLI_FIRST,
    DRIK_HOLI_LAST, DURGA_ASHTAMI, GANESH_CHATURTHI, GURU_GOBIND_SINGH_PARKASH, GURU_NANAK_JAYANTI,
    GURU_PURNIMA, HOLA_MOHALLA, HOLI, HOLIKA_DAHAN, JANMASHTAMI, MAHA_SHIVARATRI, MAHAVIR_JAYANTI,
    MAKAR_SANKRANTI, MESHA_SANKRANTI, NARAKA_CHATURDASHI, NAVARATRI, RAKSHA_BANDHAN, RAMA_NAVAMI,
    SAMVATSARI, UGADI, VIJAYA_DASHAMI,
};
use crate::rule::{
    CalendarSystem, Confidence, Days, HolidayRule, Kind, Listing, NO_WEEKEND, Rule, RuleSet,
    SourceDate, Subdivisions, TibetanDayRule, TibetanMonth, dated, joined,
};

/// 清明, the fifth solar term.
const QINGMING: SolarTerm = match SolarTerm::from_degrees(15) {
    Some(term) => term,
    None => SolarTerm::SPRING_EQUINOX,
};

/// A religious day with no day off attached.
const fn feast(name: &'static str, local: &'static str, rule: Rule) -> HolidayRule {
    HolidayRule::observance(name, local, rule).of_kind(Kind::Religious)
}

// ─────────────────────────────────────────────────────────────────────────
// Christianity, Western
// ─────────────────────────────────────────────────────────────────────────

static CHRISTIAN_WESTERN_RULES: &[HolidayRule] = &read_all(
    1583,
    [
        feast(
            "Solemnity of Mary, Mother of God",
            "",
            Rule::gregorian(1, 1),
        ),
        feast("Epiphany", "", Rule::gregorian(1, 6)),
        feast(
            "Candlemas",
            "Presentation of the Lord",
            Rule::gregorian(2, 2),
        ),
        feast("Ash Wednesday", "", Rule::easter(ASH_WEDNESDAY)),
        feast("Annunciation", "", Rule::gregorian(3, 25)),
        feast("Palm Sunday", "", Rule::easter(PALM_SUNDAY)),
        feast("Maundy Thursday", "", Rule::easter(MAUNDY_THURSDAY)),
        feast("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
        feast("Holy Saturday", "", Rule::easter(HOLY_SATURDAY)),
        feast("Easter Sunday", "", Rule::easter(EASTER_SUNDAY)),
        feast("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
        feast("Divine Mercy Sunday", "", Rule::easter(DIVINE_MERCY_SUNDAY)),
        feast("Ascension of the Lord", "", Rule::easter(ASCENSION)),
        feast("Pentecost", "", Rule::easter(PENTECOST)),
        feast("Whit Monday", "", Rule::easter(WHIT_MONDAY)),
        feast("Trinity Sunday", "", Rule::easter(TRINITY_SUNDAY)),
        feast("Corpus Christi", "", Rule::easter(CORPUS_CHRISTI)),
        feast("Sacred Heart of Jesus", "", Rule::easter(SACRED_HEART)),
        feast("Nativity of John the Baptist", "", Rule::gregorian(6, 24)),
        feast("Saints Peter and Paul", "", Rule::gregorian(6, 29)),
        feast("Transfiguration", "", Rule::gregorian(8, 6)),
        feast("Assumption of Mary", "", Rule::gregorian(8, 15)),
        feast("All Saints' Day", "", Rule::gregorian(11, 1)),
        feast("All Souls' Day", "", Rule::gregorian(11, 2)),
        // The first Sunday of Advent is the Sunday falling 27 November to
        // 3 December, four Sundays before Christmas.
        feast(
            "First Sunday of Advent",
            "",
            Rule::WeekdayOnOrAfter {
                month: 11,
                day: 27,
                weekday: Weekday::Sunday,
            },
        ),
        feast("Immaculate Conception", "", Rule::gregorian(12, 8)),
        feast("Christmas Eve", "", Rule::gregorian(12, 24)),
        feast("Christmas Day", "", Rule::gregorian(12, 25)),
        feast("St Stephen's Day", "", Rule::gregorian(12, 26)),
        feast("Holy Innocents", "", Rule::gregorian(12, 28)),
    ],
);

/// Western Christianity: the Gregorian computus and the Gregorian-dated
/// fixed feasts.
pub static CHRISTIAN_WESTERN: RuleSet = RuleSet {
    code: "christian-western",
    english_name: "Christianity (Western computus)",
    rules: CHRISTIAN_WESTERN_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "The General Roman Calendar and the Universal Norms on the Liturgical \
              Year and the General Roman Calendar, as `roman_calendar` cites them \
              (`roman-calendar-norms`); the movable feasts are offsets from the \
              Gregorian computus and nothing else",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// Christianity, Orthodox
// ─────────────────────────────────────────────────────────────────────────

/// The Great Feasts fixed in the calendar, in the calendar a church keeps
/// them by, and the movable cycle from the Julian computus, which every
/// Orthodox church keeps whichever calendar it dates the fixed feasts in.
macro_rules! orthodox_feasts {
    ($first:expr, $system:expr) => {
        &read_all(
            $first,
            [
                feast("Nativity of Christ", "", Rule::in_calendar($system, 12, 25)),
                feast("Theophany", "", Rule::in_calendar($system, 1, 6)),
                feast("Meeting of the Lord", "", Rule::in_calendar($system, 2, 2)),
                feast("Annunciation", "", Rule::in_calendar($system, 3, 25)),
                feast("Transfiguration", "", Rule::in_calendar($system, 8, 6)),
                feast(
                    "Dormition of the Theotokos",
                    "",
                    Rule::in_calendar($system, 8, 15),
                ),
                feast(
                    "Nativity of the Theotokos",
                    "",
                    Rule::in_calendar($system, 9, 8),
                ),
                feast(
                    "Exaltation of the Cross",
                    "",
                    Rule::in_calendar($system, 9, 14),
                ),
                feast(
                    "Presentation of the Theotokos",
                    "",
                    Rule::in_calendar($system, 11, 21),
                ),
                feast("Clean Monday", "", Rule::paschal(ASH_WEDNESDAY - 2)),
                feast("Lazarus Saturday", "", Rule::paschal(-8)),
                feast("Palm Sunday", "", Rule::paschal(PALM_SUNDAY)),
                feast("Holy Friday", "", Rule::paschal(GOOD_FRIDAY)),
                feast("Pascha", "", Rule::paschal(EASTER_SUNDAY)),
                feast("Bright Monday", "", Rule::paschal(EASTER_MONDAY)),
                feast("Ascension", "", Rule::paschal(ASCENSION)),
                feast("Pentecost", "", Rule::paschal(PENTECOST)),
                feast("All Saints", "", Rule::paschal(TRINITY_SUNDAY)),
            ],
        )
    };
}

// The fixed feasts are stated in the Julian calendar, so they appear
// thirteen days later on a civil calendar for as long as the two calendars
// are thirteen days apart, which is until 2100.
static CHRISTIAN_ORTHODOX_RULES: &[HolidayRule] = orthodox_feasts!(326, CalendarSystem::JULIAN);

// The same feasts in the Revised Julian calendar, which gives them the
// Gregorian dates until 2800.
static CHRISTIAN_ORTHODOX_REVISED_JULIAN_RULES: &[HolidayRule] =
    orthodox_feasts!(1925, CalendarSystem::REVISED_JULIAN);

/// Where every Orthodox table here takes its rules from, and what is not
/// read.
const ORTHODOX_SOURCES: &str = "Wikipedia, \"Great Feasts\", retrieved 2026-09-26, for \
    the nine fixed Great Feasts with their Old Style dates (secondary); the primary that \
    states them, the Menaion — for instance The Menaion, translated from the Greek by the \
    Holy Transfiguration Monastery (Boston, 2005–), whose publisher's brochure \
    (bostonmonks.com/pdfs/b045.pdf) was read the same day — was not read. The movable \
    cycle is offsets from the Julian-computus Pascha, which `computus` computes by \
    Delambre's formula as Meeus gives it (`meeus1998`)";

/// Orthodox Christianity on the Julian calendar: the Julian computus, and
/// the fixed feasts dated in the Julian calendar, as the churches that did
/// not take up the Revised Julian calendar keep them.
///
/// The churches that date the fixed feasts in the Revised Julian calendar
/// have their own table, [`CHRISTIAN_ORTHODOX_REVISED_JULIAN`]; both keep
/// the same Pascha.
pub static CHRISTIAN_ORTHODOX: RuleSet = RuleSet {
    code: "christian-orthodox",
    english_name: "Christianity (Julian computus)",
    rules: CHRISTIAN_ORTHODOX_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: ORTHODOX_SOURCES,
    subdivisions: Subdivisions::Undivided,
};

/// Orthodox Christianity on the Revised Julian calendar: the fixed feasts
/// dated in it, which gives them the Gregorian dates from 1600 to 2800,
/// and Pascha still by the Julian computus.
///
/// The Pan-Orthodox Congress of Constantinople of May–June 1923 proposed
/// the calendar, to begin by calling the coming 1 October 14 October. The
/// churches took it up one by one and not on that day: Constantinople,
/// Greece and Cyprus on 10/23 March 1924, Romania and Poland on 1/14
/// October 1924, Alexandria in 1928, Albania in 1937, Bulgaria in
/// December 1968, the Orthodox Church in America on 1 September 1982, and
/// the Orthodox Church of Ukraine on 1 September 2023, the Ukrainian Greek
/// Catholic Church with it. Poland returned most of its parishes to the
/// Julian calendar in 2014, and Antioch's year of adoption is given as
/// 1928 and as 1948 by the sources read. So the table is the convention,
/// with no years: which church keeps it in a given year is not a thing a
/// rule set here says.
pub static CHRISTIAN_ORTHODOX_REVISED_JULIAN: RuleSet = RuleSet {
    code: "christian-orthodox-revised-julian",
    english_name: "Christianity (Julian computus, Revised Julian fixed feasts)",
    rules: CHRISTIAN_ORTHODOX_REVISED_JULIAN_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "As christian-orthodox for the feasts and the Pascha. For the calendar's \
              adoption: Wikipedia, \"Revised Julian calendar\" (secondary, citing Clogg \
              2002, not read), retrieved 2026-09-26, for the dates by church; Orthochristian \
              (1 April 2014) on the Polish Synod's decision of 18 March 2014; holy-trinity.org's \
              reprint of The Dawn (October 1982) for the Orthodox Church in America; Interfax \
              (27 July 2023) for the Orthodox Church of Ukraine; the Ukrainian Greek Catholic \
              Church's announcement of 6 February 2023 (ugcc.ua); all retrieved 2026-09-26. \
              Milanković's own account in Astronomische Nachrichten no. 5279 (1924) was not \
              read",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// Ethiopian Orthodox Tewahedo
// ─────────────────────────────────────────────────────────────────────────

/// A fixed feast of the Ethiopian Orthodox Tewahedo Church, dated in the
/// Ethiopic calendar it is actually kept by.
const fn ethiopic_feast(
    name: &'static str,
    local: &'static str,
    month: u8,
    day: u8,
) -> HolidayRule {
    feast(
        name,
        local,
        Rule::in_calendar(CalendarSystem::ETHIOPIC, month, day),
    )
}

/// The *tewsak* of Bahire Hasab that the shared offsets do not already
/// name: days from Tinsae (Easter Sunday) to the start of the Fast of
/// Nineveh, the start of the Great Fast, Mid-Lent Sunday and Mid-Pentecost.
const TSOME_NENEWE: i16 = -69;
const ABIY_TSOM: i16 = -55;
const DEBRE_ZEIT: i16 = -28;
const REKBE_KAHNAT: i16 = 24;

static ETHIOPIAN_ORTHODOX_RULES: &[HolidayRule] = &read_all(
    401,
    [
        ethiopic_feast("Enkutatash (New Year)", "እንቁጣጣሽ", 1, 1),
        ethiopic_feast("Meskel (Finding of the True Cross)", "መስቀል", 1, 17),
        // Genna on 25 December of the Julian calendar, as the Coptic Nativity:
        // 29 Tahsas, and 28 Tahsas after an Ethiopic leap year (eotc-ma-calendar).
        feast(
            "Genna (Christmas)",
            "ገና",
            Rule::in_calendar(CalendarSystem::JULIAN, 12, 25),
        ),
        ethiopic_feast("Timkat (Epiphany)", "ጥምቀት", 5, 11),
        ethiopic_feast("Debre Tabor (Transfiguration)", "ደብረ ታቦር", 12, 13),
        // The movable cycle of Bahire Hasab, as offsets from Tinsae.
        feast(
            "Tsome Nenewe (Fast of Nineveh) begins",
            "ጾመ ነነዌ",
            Rule::paschal(TSOME_NENEWE),
        ),
        feast(
            "Abiy Tsom (Great Lent) begins",
            "ዐቢይ ጾም",
            Rule::paschal(ABIY_TSOM),
        ),
        feast(
            "Debre Zeit (Mid-Lent)",
            "ደብረ ዘይት",
            Rule::paschal(DEBRE_ZEIT),
        ),
        feast("Hosanna (Palm Sunday)", "ሆሣዕና", Rule::paschal(PALM_SUNDAY)),
        feast("Siklet (Good Friday)", "ስቅለት", Rule::paschal(GOOD_FRIDAY)),
        feast("Fasika (Easter)", "ፋሲካ", Rule::paschal(EASTER_SUNDAY)),
        feast(
            "Rekbe Kahnat (Mid-Pentecost)",
            "ርክበ ካህናት",
            Rule::paschal(REKBE_KAHNAT),
        ),
        feast("Erget (Ascension)", "ዕርገት", Rule::paschal(ASCENSION)),
        feast("Paraclete (Pentecost)", "ጰራቅሊጦስ", Rule::paschal(PENTECOST)),
    ],
);

/// The feasts of the Ethiopian Orthodox Tewahedo Church.
///
/// The fixed ones are ordinary dates — 11 Tirr, 17 Meskerem — in the calendar
/// the church actually keeps, and they are written down there, because
/// [`CalendarSystem`] is open to any calendar the registry names. A closed
/// list of calendars would have left two poor choices: approximate them in
/// a calendar they do not belong to, or leave them out.
///
/// The movable ones follow *Bahire Hasab*, the Ethiopian computus. Its
/// arithmetic — the cycle of evangelists, *wengelawi*, *abektie* and
/// *metqi* — is its own, but it is the Alexandrian computus, the same rule
/// the Julian Paschalion states, and Tinsae falls on the Orthodox Pascha
/// every year. So the cycle is written as offsets from the Julian-computus
/// Easter: the *tewsak* of the Ethiopian tables are the same numbers of
/// days, and the test anchors check three years of them. What is not here
/// is a second implementation of the same Sunday under another name.
///
/// # Genna
///
/// The fixed feasts are dates in the Ethiopic calendar, all but Genna. After
/// an Ethiopic leap year the whole Ethiopic year sits a day later against
/// the Gregorian one, so 29 Tahsas falls on 8 January rather than the 7th,
/// and Enkutatash moves as the arithmetic says, to 12 September. Genna does
/// not: Keraneyo Medhane Alem's order of the calendar keeps it on "Tahisas
/// 29 E.C." and "on leap year on Tahisas 28 E.C", and the church's calendar
/// page says it "coincides with the date of this observance in the Eastern
/// Orthodox dispensation", 25 December of the Julian calendar, which is the
/// rule the table carries: 7 January from 1900 to 2099, as the Coptic
/// Nativity is. A tour operator's page says Genna is on 8 January in those
/// years "only in Lalibela, the rest of the country is always celebrating
/// Genna on January 7th"; Lalibela's day is not yet carried, as no church
/// source for it was read. Ethiopia's public holiday, the `ET` table's, is the
/// Proclamation's 29 Tahsas.
pub static ETHIOPIAN_ORTHODOX: RuleSet = RuleSet {
    code: "ethiopian-orthodox",
    english_name: "Ethiopian Orthodox Tewahedo",
    rules: ETHIOPIAN_ORTHODOX_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "The calendar page of ethiopianorthodox.org (ethiopianorthodox.org/english/\
              calendar.html, \"©2003 Ethiopian Orthodox Tewahedo Church\"), retrieved \
              2026-09-26, for Timkat on 11 Tirr and Meskel on 17 Meskerem, Genna \
              coinciding with the Eastern Orthodox Christmas, the Fast of Nineveh in \
              the third week before the Great Lent of 56 days, and Easter reckoned by \
              the Alexandrian rule; Keraneyo Medhane Alem Ethiopian Orthodox Tewahedo \
              Church, \"Beliefs and Origins - Order of Calendar\" (eotc-ma.com, \
              eotc-ma-calendar), retrieved 2026-09-29, for Genna on 28 Tahsas after a \
              leap year; Boundless Ethiopia, \"Genna (Christmas)\" \
              (boundless-ethiopia-genna), retrieved 2026-09-29, a tour operator's page, \
              for Lalibela's 8 January; Enkutatash on \
              1 Meskerem and Debre Tabor on 13 Nehasse, and the tewsak of Mid-Lent and \
              Mid-Pentecost, are not from a document read here. The scholarly account, \
              O. Neugebauer, Ethiopic Astronomy and Computus (Vienna: Österreichische \
              Akademie der Wissenschaften, 1979), and Aymro Wondmagegnehu and Joachim \
              Motovu (eds.), The Ethiopian Orthodox Church (Addis Ababa: The Ethiopian \
              Orthodox Mission, 1970), were not read",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// Coptic Orthodox
// ─────────────────────────────────────────────────────────────────────────

/// A fixed feast of the Coptic Orthodox Church, dated in the Coptic
/// calendar it is kept by.
const fn coptic_feast(name: &'static str, local: &'static str, month: u8, day: u8) -> HolidayRule {
    feast(
        name,
        local,
        Rule::in_calendar(CalendarSystem::COPTIC, month, day),
    )
}

/// The Sunday after Easter, which the Coptic Church keeps for Thomas.
const THOMAS_SUNDAY: i16 = 7;

static COPTIC_ORTHODOX_RULES: &[HolidayRule] = &read_all(
    326,
    [
        coptic_feast("Nayrouz (New Year)", "عيد النيروز", 1, 1),
        coptic_feast("Feast of the Cross", "عيد الصليب", 1, 17),
        // The Nativity on 25 December of the Julian calendar: 29 Koiak, and
        // 28 Koiak in the Coptic year after a Coptic leap year, 7 January from
        // 1900 to 2099 (st-takla-nativity-fast; ramzy-nativity-2004).
        feast(
            "Nativity (Christmas)",
            "عيد الميلاد المجيد",
            Rule::in_calendar(CalendarSystem::JULIAN, 12, 25),
        ),
        coptic_feast("Circumcision of the Lord", "عيد الختان", 5, 6),
        coptic_feast("Theophany (Epiphany)", "عيد الغطاس", 5, 11),
        coptic_feast("Wedding at Cana", "عرس قانا الجليل", 5, 13),
        coptic_feast("Dormition of St Mary", "نياحة السيدة العذراء", 5, 21),
        coptic_feast(
            "Entry of the Lord into the Temple",
            "دخول السيد المسيح الهيكل",
            6,
            8,
        ),
        coptic_feast("Feast of the Cross (second)", "عيد الصليب", 7, 10),
        coptic_feast("Annunciation", "عيد البشارة", 7, 29),
        coptic_feast(
            "Entry of the Holy Family into Egypt",
            "دخول السيد المسيح أرض مصر",
            9,
            24,
        ),
        coptic_feast("Feast of the Apostles", "عيد الرسل", 11, 5),
        coptic_feast("Transfiguration", "عيد التجلي", 12, 13),
        coptic_feast("Assumption of St Mary", "صعود جسد السيدة العذراء", 12, 16),
        // The movable cycle: the Coptic Church keeps the Alexandrian computus,
        // which is the Julian Paschalion's Sunday, and the same tewsak as the
        // Ethiopian table.
        feast(
            "Fast of Nineveh (Jonah) begins",
            "صوم يونان",
            Rule::paschal(TSOME_NENEWE),
        ),
        feast(
            "Great Lent begins",
            "الصوم الكبير",
            Rule::paschal(ABIY_TSOM),
        ),
        feast("Palm Sunday", "أحد الشعانين", Rule::paschal(PALM_SUNDAY)),
        feast(
            "Covenant Thursday",
            "خميس العهد",
            Rule::paschal(MAUNDY_THURSDAY),
        ),
        feast("Good Friday", "الجمعة العظيمة", Rule::paschal(GOOD_FRIDAY)),
        feast(
            "Easter (Resurrection)",
            "عيد القيامة المجيد",
            Rule::paschal(EASTER_SUNDAY),
        ),
        feast("Thomas Sunday", "أحد توما", Rule::paschal(THOMAS_SUNDAY)),
        feast("Ascension", "عيد الصعود", Rule::paschal(ASCENSION)),
        feast("Pentecost", "عيد العنصرة", Rule::paschal(PENTECOST)),
    ],
);

/// The feasts of the Coptic Orthodox Church of Alexandria.
///
/// The fourteen feasts of the Lord — seven major, seven minor — with Nayrouz,
/// both Feasts of the Cross, the Apostles and the two feasts of St Mary,
/// dated in the Coptic calendar the church keeps; and the paschal cycle from
/// the Fast of Nineveh to Pentecost as offsets from the Julian-computus
/// Easter, which is the Alexandrian one.
///
/// The Nativity is the one fixed feast not kept on its Coptic date every
/// year. St-Takla's rite of the Nativity Fast has the fast end "في 29 كيهك أي
/// 7 يناير", on 29 Koiak, that is 7 January, and on 28 Koiak "في السنوات
/// الميلادية الكبيسة", in the Gregorian leap years; the Metropolis of the Southern
/// United States's calendar keeps it on 7 January in every year of
/// 2000–2100 it lists; Fr. John Ramzy's article for the Metropolis gives
/// the reason, 25 December of the Julian calendar, which "will become
/// 8 January after the year 2100". So the feast is 25 December Julian, and
/// in 2024 and 2028, when 29 Koiak is 8 January, it is 7 January, 28 Koiak.
/// The Metropolis's calendar gives the feast two days in its Gregorian leap
/// years, 7 and 8 January 2024; the second day is not yet carried, as no
/// source read states the rule behind it.
///
/// The Arabic names are the ones the church's own publications use. The
/// Coptic-language names of seven feasts, where a source prints them, are
/// `hc-i18n`'s `holiday_names` table for `cop`; no source read prints the
/// others. The fasts are `orthodox_fasts`'s reckoning `coptic-fasts`.
pub static COPTIC_ORTHODOX: RuleSet = RuleSet {
    code: "coptic-orthodox",
    english_name: "Coptic Orthodox",
    rules: COPTIC_ORTHODOX_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "The Feasts of the Church, Coptic Orthodox Diocese of Los Angeles \
              (lacopts.org), retrieved 2026-09-22, for the fixed dates; St-Takla.org, \
              the rite of the Nativity Fast (st-takla-nativity-fast), the Coptic \
              Orthodox Metropolis of the Southern United States's fasts and feasts of \
              2000-2100 (suscopts-fasts) and Fr. John Ramzy, \"The Glorious Feast of \
              Nativity: 7 January? 29 Kiahk? 25 December?\" (suscopts.org, \
              ramzy-nativity-2004), retrieved 2026-09-29, for the Nativity on \
              25 December of the Julian calendar; the movable cycle as offsets from \
              the Julian-computus Pascha the church shares",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// Islam
// ─────────────────────────────────────────────────────────────────────────

/// An Islamic observance: tabular, and therefore a prediction.
const fn hijri(name: &'static str, local: &'static str, month: u8, day: u8) -> HolidayRule {
    feast(
        name,
        local,
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, month, day),
    )
    .approximate()
}

static ISLAMIC_RULES: &[HolidayRule] = &[
    hijri("Islamic New Year", "رأس السنة الهجرية", 1, 1),
    hijri("Tasu'a", "تاسوعاء", 1, 9),
    hijri("Ashura", "عاشوراء", 1, 10),
    hijri("Arba'een", "الأربعين", 2, 20),
    hijri("Mawlid an-Nabi", "المولد النبوي", 3, 12),
    hijri("Isra and Mi'raj", "الإسراء والمعراج", 7, 27),
    hijri("Mid-Sha'ban", "ليلة البراءة", 8, 15),
    hijri("First of Ramadan", "أول رمضان", 9, 1),
    hijri("Laylat al-Qadr", "ليلة القدر", 9, 27),
    hijri("Eid al-Fitr", "عيد الفطر", 10, 1),
    hijri("Day of Arafah", "يوم عرفة", 12, 9),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 10),
    hijri("Eid al-Ghadir", "عيد الغدير", 12, 18),
];

/// Islam.
///
/// **Every date here is a prediction.** The Hijri months of religious
/// practice begin on a crescent sighting decided per country, sometimes on
/// the evening before; the tabular civil calendar used here is a good
/// arithmetic approximation of that and nothing more. Countries that publish
/// their own table — Saudi Arabia's Umm al-Qurā, Türkiye's Diyanet — are
/// modelled in [`crate::countries`], and are still flagged approximate.
///
/// Three of the days are Shia: Tasu'a on 9 Muharram, the eve of Ashura;
/// Arba'een on 20 Safar, forty days after it; and Eid al-Ghadir on
/// 18 Dhu al-Hijjah, the day of Ghadir Khumm, a public holiday in Iran and,
/// from 2024, in Iraq, whose tables date it too. Wikipedia's Arba'in of
/// 14 August 2025, 3 August 2026 and 24 July 2027 are Umm al-Qurā's
/// 20 Safar; the tabular day is one, two and one days later, because the
/// tabular year opens a day after Umm al-Qurā's in 1447 and 1448, and its
/// Muharram has 30 days where Umm al-Qurā's of 1448 and 1449 has 29.
pub static ISLAMIC: RuleSet = RuleSet {
    code: "islamic",
    english_name: "Islam",
    rules: ISLAMIC_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 27),
    sources: "The tabular civil Hijri calendar, CLDR `islamic-civil`. These \
              are computations, not announcements. The Shia days from \
              Wikipedia, \"Tasu'a\", \"Arba'in\" and \"Eid al-Ghadir\" \
              (`wikipedia-tasua`, `wikipedia-arbaeen`, `wikipedia-eid-al-ghadir`, \
              secondary), retrieved 2026-09-27: 9 Muharram, 20 Safar with \
              Arba'in on 14 August 2025, 3 August 2026 and 24 July 2027, Umm \
              al-Qurā's 20 Safar (`islamic-umalqura`), as checks, and 18 Dhu \
              al-Hijjah",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// Judaism
// ─────────────────────────────────────────────────────────────────────────

/// A Hebrew-calendar observance. The month numbering is Tishrei-first, so
/// Nisan is 7 and Adar — Adar II in a leap year — is 6.
const fn hebrew_day(name: &'static str, local: &'static str, month: u8, day: u8) -> HolidayRule {
    feast(
        name,
        local,
        Rule::in_calendar(CalendarSystem::HEBREW, month, day),
    )
}

/// Purim, 14 Adar — Adar II in a leap year.
const PURIM: Rule = Rule::in_calendar(CalendarSystem::HEBREW, 6, 14);

/// The day before Purim.
const PURIM_EVE: Rule = Rule::Offset {
    base: &PURIM,
    days: -1,
};

/// Ta'anit Esther: the day before Purim, or the Thursday before when Purim
/// is a Sunday, so that the fast is not kept on the Sabbath: "if Purim
/// falls on Sunday, we start the fast on Thursday" (*Shulchan Arukh*,
/// Orach Chayim 686:2).
const TAANIT_ESTHER: Rule = Rule::moved_by_weekday(&PURIM_EVE, &[(Weekday::Saturday, -2)]);

/// 14 Nisan, the eve of Passover.
const NISAN_14: Rule = Rule::in_calendar(CalendarSystem::HEBREW, 7, 14);

/// Ta'anit Bechorot, the fast of the firstborn on the eve of Passover, or
/// the Thursday before when the eve is a Sabbath: "יש אומרי' שמתעני'
/// הבכורו' ביום ה'", some say the firstborn fast on the Thursday, the
/// opinion the Rema says to follow (*Shulchan Arukh*, Orach Chayim 470:1–2).
const TAANIT_BECHOROT: Rule = Rule::moved_by_weekday(&NISAN_14, &[(Weekday::Saturday, -2)]);

/// The move of the four fasts of the destruction off the Sabbath: "All four
/// fasts, if they fall on Shabbos, are pushed to after Shabbos" (*Shulchan
/// Arukh*, Orach Chayim 550:3), to the Sunday.
const AFTER_THE_SABBATH: &[(Weekday, i16)] = &[(Weekday::Saturday, 1)];

/// 3 Tishrei, the Fast of Gedaliah's date.
const TISHREI_3: Rule = Rule::in_calendar(CalendarSystem::HEBREW, 1, 3);
/// 17 Tammuz.
const TAMMUZ_17: Rule = Rule::in_calendar(CalendarSystem::HEBREW, 10, 17);
/// 9 Av.
const AV_9: Rule = Rule::in_calendar(CalendarSystem::HEBREW, 11, 9);

/// The Fast of Gedaliah: 3 Tishrei, or 4 Tishrei when the 3rd is a Sabbath.
const FAST_OF_GEDALIAH: Rule = Rule::moved_by_weekday(&TISHREI_3, AFTER_THE_SABBATH);
/// The Seventeenth of Tammuz, or the 18th when the 17th is a Sabbath.
const SEVENTEENTH_OF_TAMMUZ: Rule = Rule::moved_by_weekday(&TAMMUZ_17, AFTER_THE_SABBATH);
/// Tisha B'Av, or 10 Av when the 9th is a Sabbath, as Reingold and
/// Dershowitz's `tishah-be-av` gives it.
const TISHA_BAV: Rule = Rule::moved_by_weekday(&AV_9, AFTER_THE_SABBATH);

static JEWISH_RULES: &[HolidayRule] = &read_all(
    359,
    [
        hebrew_day("Rosh Hashanah", "ראש השנה", 1, 1),
        hebrew_day("Rosh Hashanah (second day)", "ראש השנה", 1, 2),
        feast("Fast of Gedaliah", "צום גדליה", FAST_OF_GEDALIAH),
        hebrew_day("Yom Kippur", "יום כיפור", 1, 10),
        hebrew_day("Sukkot", "סוכות", 1, 15),
        hebrew_day("Hoshana Rabbah", "הושענא רבה", 1, 21),
        hebrew_day("Shemini Atzeret", "שמיני עצרת", 1, 22),
        hebrew_day("Simchat Torah", "שמחת תורה", 1, 23),
        hebrew_day("Hanukkah", "חנוכה", 3, 25),
        hebrew_day("Tenth of Tevet", "עשרה בטבת", 4, 10),
        hebrew_day("Tu BiShvat", "ט\"ו בשבט", 5, 15),
        feast("Ta'anit Esther", "תענית אסתר", TAANIT_ESTHER),
        hebrew_day("Purim", "פורים", 6, 14),
        hebrew_day("Shushan Purim", "שושן פורים", 6, 15),
        feast("Ta'anit Bechorot", "תענית בכורות", TAANIT_BECHOROT),
        hebrew_day("Passover", "פסח", 7, 15),
        hebrew_day("Seventh Day of Passover", "שביעי של פסח", 7, 21),
        hebrew_day("Lag BaOmer", "ל\"ג בעומר", 8, 18),
        hebrew_day("Shavuot", "שבועות", 9, 6),
        feast(
            "Seventeenth of Tammuz",
            "שבעה עשר בתמוז",
            SEVENTEENTH_OF_TAMMUZ,
        ),
        feast("Tisha B'Av", "תשעה באב", TISHA_BAV),
        // Sh'ela, the first day of the prayer for rain outside the Land of
        // Israel: 26 Hatur, in the Coptic calendar's third month.
        feast(
            "Sh'ela (prayer for rain, outside the Land of Israel)",
            "שאלה",
            Rule::in_calendar(CalendarSystem::COPTIC, 3, 26),
        ),
    ],
);

/// Judaism.
///
/// The Hebrew calendar is arithmetic, so every date here is exact. A Jewish
/// day begins at sunset on the preceding evening, which is not modelled.
///
/// The minor fasts are kept off the Sabbath. The four fasts of the
/// destruction, "if they fall on Shabbos, are pushed to after Shabbos"
/// (*Shulchan Arukh*, Orach Chayim 550:3): the Fast of Gedaliah, the
/// Seventeenth of Tammuz and Tisha B'Av move to the Sunday — the first to
/// 6 October 2024, the others to 13 July and 3 August 2025 — and the Tenth
/// of Tevet never falls on a Sabbath in the fixed calendar, which a test
/// checks. Ta'anit Esther moves back instead:
/// "if Purim falls on Sunday, we start the fast on Thursday" (686:2), as
/// Reingold and Dershowitz's `ta-anit-esther` gives it. So does Ta'anit
/// Bechorot, the firstborn's fast on the eve of Passover (470:1): when the
/// eve is a Sabbath, "some say" it is kept on the Thursday and "some say"
/// not at all (470:2), and the Rema's gloss says to keep the first, which is
/// carried: 25 March 2021, 14 Nisan being Saturday the 27th. The second
/// opinion, no fast in those years, is not yet a table of its own.
///
/// *Sh'ela*, the day the prayer for rain begins outside the Land of Israel,
/// is "60 days after the onset of tekufat Tishrei", the autumn *tekufah* of
/// a year of 365¼ days, which Reingold and Dershowitz's `sh-ela` puts on
/// 26 Hatur of the Coptic calendar: 5 December, or 6 December in the year
/// before a Gregorian leap year, until 2100, when it moves a day later. The
/// prayer is first said on the evening before, which begins that day, as
/// Chabad.org puts it: "on the night of December 4, and in the year before
/// a (civil) leap year … on the night of December 5".
///
/// A yahrzeit or a Hebrew birthday is not a table entry but a function of
/// the date it keeps: `hc_calendars_lunar::hebrew::yahrzeit` and
/// `hebrew::birthday`.
pub static JEWISH: RuleSet = RuleSet {
    code: "jewish",
    english_name: "Judaism",
    rules: JEWISH_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "The arithmetic Hebrew calendar as `hc-calendars-lunar` \
              implements it, following Dershowitz and Reingold, \
              Calendrical Calculations, chapter 8; Ta'anit Esther, Tisha B'Av and \
              Sh'ela as `ta-anit-esther`, `tishah-be-av` and `sh-ela` in their \
              calendar.l (`reingold2018code`), read 2026-09-26 and, for Tisha B'Av, \
              2026-09-29; Shulchan Arukh, Orach Chayim 470:1-2, 550:3 and 686:2, for \
              Ta'anit Bechorot and the moves of the minor fasts off the Sabbath, read on Sefaria in the Hebrew \
              of the Lemberg 1893 edition and the Sefaria Community Translation \
              (`sefaria-shulchan-arukh-oc`), retrieved 2026-09-29; Yehuda Shurpin, \"Why Is the \
              Prayer for Rain Based on the Civil Calendar?\", Chabad.org, retrieved \
              2026-09-26, for the dates of Sh'ela to 2100; Hebcal, \"Ta'anit Esther\", \
              retrieved 2026-09-26, for the fast's dates of 2024–2031 as a check",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// The Bahá'í Faith
// ─────────────────────────────────────────────────────────────────────────

/// A holy day dated in the Badíʿ calendar as kept — see [`BAHAI`] for
/// what that is and where it ends.
const fn badi(name: &'static str, local: &'static str, month: u8, day: u8) -> HolidayRule {
    feast(
        name,
        local,
        Rule::in_calendar(CalendarSystem::BADI, month, day),
    )
}

/// The Birth of the Báb since 172 BE, from the Bahá'í World Centre's table
/// of Badíʿ dates for 172–221 BE (2015–2064).
///
/// The Universal House of Justice's letter of 10 July 2014 set the Twin
/// Holy Birthdays on "the first and the second day following the
/// occurrence of the eighth new moon after Naw-Rúz" — a lunar rule on an
/// otherwise solar calendar, decided against the Tehran meridian, which no
/// arithmetic in this crate reproduces. The table is the source, and past
/// its last year the rule reports a gap rather than a guess.
#[rustfmt::skip]
static BIRTH_OF_THE_BAB_DAYS: Listing = Listing::Dates(&[
    (2015, 11, 13), (2016, 11, 1), (2017, 10, 21), (2018, 11, 9), (2019, 10, 29),
    (2020, 10, 18), (2021, 11, 6), (2022, 10, 26), (2023, 10, 16), (2024, 11, 2),
    (2025, 10, 22), (2026, 11, 10), (2027, 10, 30), (2028, 10, 19), (2029, 11, 7),
    (2030, 10, 28), (2031, 10, 17), (2032, 11, 4), (2033, 10, 24), (2034, 11, 12),
    (2035, 11, 1), (2036, 10, 20), (2037, 11, 8), (2038, 10, 29), (2039, 10, 19),
    (2040, 11, 6), (2041, 10, 26), (2042, 10, 15), (2043, 11, 3), (2044, 10, 22),
    (2045, 11, 10), (2046, 10, 30), (2047, 10, 20), (2048, 11, 7), (2049, 10, 28),
    (2050, 10, 17), (2051, 11, 5), (2052, 10, 24), (2053, 11, 11), (2054, 11, 1),
    (2055, 10, 21), (2056, 11, 8), (2057, 10, 29), (2058, 10, 18), (2059, 11, 6),
    (2060, 10, 25), (2061, 10, 14), (2062, 11, 2), (2063, 10, 23), (2064, 11, 10),
]);

/// The tabulated Birth of the Báb; the Birth of Bahá'u'lláh is the day after.
const BIRTH_OF_THE_BAB: Rule = Rule::listed(BIRTH_OF_THE_BAB_DAYS.every(), 2015, 2064);

static BAHAI_RULES: &[HolidayRule] = &[
    badi("Naw-Rúz", "عید نوروز", 1, 1),
    badi("First day of Riḍván", "عید رضوان", 2, 13),
    badi("Ninth day of Riḍván", "عید رضوان", 3, 2),
    badi("Twelfth day of Riḍván", "عید رضوان", 3, 5),
    badi("Declaration of the Báb", "بعثت حضرت باب", 4, 8),
    badi("Ascension of Bahá'u'lláh", "صعود حضرت بهاءالله", 4, 13),
    badi("Martyrdom of the Báb", "شهادت حضرت باب", 6, 17),
    // The Twin Holy Birthdays: the fixed Badíʿ dates of Western practice
    // until 171 BE — 5 ʻIlm and 9 Qudrat, which the arithmetic calendar puts
    // on 20 October and 12 November — and the published lunar table from
    // 172 BE. In Iran and the Middle East they were kept on 1 and 2 Muḥarram
    // before then; that convention is not carried.
    badi("Birth of the Báb", "میلاد حضرت باب", 12, 5).years(None, Some(2014)),
    badi("Birth of Bahá'u'lláh", "میلاد حضرت بهاءالله", 13, 9).years(None, Some(2014)),
    feast("Birth of the Báb", "میلاد حضرت باب", BIRTH_OF_THE_BAB).years(Some(2015), None),
    feast(
        "Birth of Bahá'u'lláh",
        "میلاد حضرت بهاءالله",
        Rule::Offset {
            base: &BIRTH_OF_THE_BAB,
            days: 1,
        },
    )
    .years(Some(2015), None),
    badi("Day of the Covenant", "یوم میثاق", 14, 4),
    badi("Ascension of ʻAbdu'l-Bahá", "صعود حضرت عبدالبهاء", 14, 6),
    // The two periods, by their first day. Ayyám-i-Há is not a month: the
    // calendar numbers it zero.
    HolidayRule::observance(
        "First day of Ayyám-i-Há",
        "ایام هاء",
        Rule::in_calendar(CalendarSystem::BADI, bahai::AYYAM_I_HA, 1),
    ),
    badi("First day of the Fast", "صیام", 19, 1),
];

/// The Bahá'í Faith.
///
/// The nine holy days on which work is suspended — Naw-Rúz, the first,
/// ninth and twelfth days of Riḍván, the Declaration of the Báb, the
/// Ascension of Bahá'u'lláh, the Martyrdom of the Báb and the Twin Holy
/// Birthdays — with the two on which it is not, the Day of the Covenant
/// and the Ascension of ʻAbdu'l-Bahá; and the first days of Ayyám-i-Há and
/// of the Fast, the month of ʻAláʼ.
///
/// # What is firm and what is not
///
/// The days are dated in the Badíʿ calendar as kept
/// (`hc_calendars_solar::bahai_kept`): the arithmetic Western rule, Naw-Rúz
/// on 21 March, until 171 BE, and from Naw-Rúz 172 BE (2015) the unified
/// calendar that begins on the day of the Tehran equinox, as the Bahá'í
/// World Centre published it for 172–221 BE. Every Badíʿ-dated entry is
/// therefore exact through 19 March 2065 and a reported gap after, where
/// the table ends and this crate does no astronomy.
///
/// The Twin Holy Birthdays are not a Badíʿ date since 172 BE but a lunar
/// rule, and they come from the same table: exact for 2015–2064, and a
/// reported gap after. Before 2015 they are the fixed 5 ʻIlm and 9 Qudrat
/// of Western practice.
///
/// A Bahá'í day runs from sunset to sunset, so each observance begins at
/// sunset on the day before the date given. The hours of the Ascension of
/// Bahá'u'lláh (3 a.m.) and the Martyrdom of the Báb (noon) are not
/// modelled.
///
/// The names are Persian, as the Bahá'í Reference Library heads them; the
/// three days of Riḍván share one.
pub static BAHAI: RuleSet = RuleSet {
    code: "bahai",
    english_name: "Bahá'í Faith",
    rules: BAHAI_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Badíʿ dates 172 to 221 BE, prepared by an ad hoc committee at the \
              Bahá'í World Centre from data of HM Nautical Almanac Office, \
              2014 (bahai-library.com/pdf/uhj/uhj_bahai_dates_172-221.pdf, \
              retrieved 2026-09-22), for the Badíʿ date of every holy day and \
              the Twin Holy Birthdays; the Universal House of Justice, letter \
              of 10 July 2014, for the lunar rule; Days of Remembrance, Bahá'í \
              Reference Library, Persian edition, for the names, with the \
              community's usual terms for the days it does not head",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// Hinduism
// ─────────────────────────────────────────────────────────────────────────

/// Holikā Dahana and Holī as Drik Panchang's pages give them for 2015 to
/// 2036, exactly, and as the rules of [`crate::hindu`] have them before and
/// after, approximately: the rules part from the pages in three of the 22
/// years.
const HOLIKA_DAHAN_ROWS: [HolidayRule; 3] = feast("Holika Dahan", "होलिका दहन", HOLIKA_DAHAN)
    .read_in(
        DRIK_HOLI.named("dahan"),
        DRIK_HOLI_FIRST,
        DRIK_HOLI_LAST,
        Confidence::Exact,
    );
const HOLI_ROWS: [HolidayRule; 3] = feast("Holi", "होली", HOLI).read_in(
    DRIK_HOLI.named("holi"),
    DRIK_HOLI_FIRST,
    DRIK_HOLI_LAST,
    Confidence::Exact,
);

static HINDU_RULES: &[HolidayRule] = &[
    feast("Makar Sankranti", "मकर संक्रांति", MAKAR_SANKRANTI),
    feast("Maha Shivaratri", "महाशिवरात्रि", MAHA_SHIVARATRI),
    HOLIKA_DAHAN_ROWS[0],
    HOLIKA_DAHAN_ROWS[1],
    HOLIKA_DAHAN_ROWS[2],
    HOLI_ROWS[0],
    HOLI_ROWS[1],
    HOLI_ROWS[2],
    feast("Ugadi", "उगादि", UGADI),
    feast("Rama Navami", "राम नवमी", RAMA_NAVAMI),
    feast("Mahavir Jayanti", "महावीर जयंती", MAHAVIR_JAYANTI),
    feast("Mesha Sankranti", "मेष संक्रांति", MESHA_SANKRANTI),
    feast("Akshaya Tritiya", "अक्षय तृतीया", AKSHAYA_TRITIYA),
    feast("Buddha Purnima", "बुद्ध पूर्णिमा", BUDDHA_PURNIMA),
    feast("Guru Purnima", "गुरु पूर्णिमा", GURU_PURNIMA),
    feast("Raksha Bandhan", "रक्षा बंधन", RAKSHA_BANDHAN),
    feast("Krishna Janmashtami", "कृष्ण जन्माष्टमी", JANMASHTAMI),
    feast("Ganesh Chaturthi", "गणेश चतुर्थी", GANESH_CHATURTHI),
    feast("Navaratri", "शारदीय नवरात्रि", NAVARATRI),
    feast("Durga Ashtami", "दुर्गा अष्टमी", DURGA_ASHTAMI),
    feast("Vijaya Dashami", "विजयादशमी", VIJAYA_DASHAMI),
    feast("Naraka Chaturdashi", "नरक चतुर्दशी", NARAKA_CHATURDASHI),
    feast("Diwali", "दीपावली", DIWALI),
    feast("Guru Nanak Jayanti", "गुरु नानक जयंती", GURU_NANAK_JAYANTI),
];

/// Hinduism, with the Jain and Sikh days the national almanac lists beside
/// it.
///
/// Every entry is a [`Rule::Tithi`] or a [`Rule::Sankranti`] from
/// [`crate::hindu`], dated in the amānta lunisolar calendar at the sunrise
/// of the national almanac's Central Station and kept on the part of the
/// day its convention names. The dates are exact to the astronomical model
/// and to those conventions; a regional almanac that follows a local
/// sunrise, or the Vaiṣṇava rather than the Smārta Janmāṣṭamī, may keep a
/// day differently, and can build the same rules with its own calendar.
///
/// Makara Saṅkrānti is the day of the Sun's entry into Makara at the Indian
/// meridian; northern India keeps it the day before when the entry falls
/// after sunset, which is not modelled.
pub static HINDU: RuleSet = RuleSet {
    code: "hindu",
    english_name: "Hinduism",
    rules: HINDU_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "The Rashtriya Panchang, Positional Astronomy Centre, India \
              Meteorological Department, Śaka 1945 and 1946 (2023–2025), \
              English editions: the \"Principal Festivals and Anniversaries\" \
              list, which every rule here but Naraka Chaturdashi reproduces, and \
              the prevalence conventions as `hindu` states them; Naraka \
              Chaturdashi, the Tamil Deepavali, by the Calendar Reform \
              Committee's rule (`crc1955`), which gives the Naraka Chaturdasi \
              of the Department of Personnel and Training's restricted holidays \
              for 2026 and 2027, 8 November 2026 and 28 October 2027 \
              (`dopt-holidays-2025-2027`), and the Deepavali of the governments \
              `hindu::NARAKA_CHATURDASHI` names",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// The Wheel of the Year
// ─────────────────────────────────────────────────────────────────────────

/// A quarter day: a solstice or equinox on its Universal Time day.
const fn quarter_day(name: &'static str, term: SolarTerm) -> HolidayRule {
    feast(
        name,
        "",
        Rule::SolarTerm {
            term,
            meridian: Meridian::UNIVERSAL,
        },
    )
}

/// A cross-quarter day, on its fixed Gregorian date.
const fn cross_quarter_day(name: &'static str, month: u8, day: u8) -> HolidayRule {
    feast(name, "", Rule::gregorian(month, day))
}

static WHEEL_OF_THE_YEAR_RULES: &[HolidayRule] = &dated(
    [
        cross_quarter_day("Imbolc", 2, 1),
        quarter_day("Ostara", SolarTerm::SPRING_EQUINOX),
        cross_quarter_day("Beltane", 5, 1),
        quarter_day("Litha", SolarTerm::SUMMER_SOLSTICE),
        cross_quarter_day("Lughnasadh", 8, 1),
        quarter_day("Mabon", SolarTerm::AUTUMN_EQUINOX),
        cross_quarter_day("Samhain", 11, 1),
        quarter_day("Yule", SolarTerm::WINTER_SOLSTICE),
    ],
    None,
    Some(1974),
);

static WHEEL_OF_THE_YEAR_SOUTH_RULES: &[HolidayRule] = &dated(
    [
        cross_quarter_day("Lughnasadh", 2, 1),
        quarter_day("Mabon", SolarTerm::SPRING_EQUINOX),
        cross_quarter_day("Samhain", 5, 1),
        quarter_day("Yule", SolarTerm::SUMMER_SOLSTICE),
        cross_quarter_day("Imbolc", 8, 1),
        quarter_day("Ostara", SolarTerm::AUTUMN_EQUINOX),
        cross_quarter_day("Beltane", 11, 1),
        quarter_day("Litha", SolarTerm::WINTER_SOLSTICE),
    ],
    None,
    Some(1974),
);

/// The Wheel of the Year, as kept in the northern hemisphere.
///
/// The eight festivals of modern paganism: the four quarter days — the
/// solstices and equinoxes, Yule, Ostara, Litha and Mabon — and the four
/// cross-quarter days between them, Imbolc, Beltane, Lughnasadh and
/// Samhain, which British neopagans joined into one cycle in the middle of
/// the twentieth century from the solar festivals of many European peoples
/// and the four fire festivals of the Insular Celts.
///
/// A quarter day is the day of its solstice or equinox in Universal Time;
/// a cross-quarter day is its conventional Gregorian date, Samhain on
/// 1 November rather than the eve. Groups that keep a quarter day on its
/// local date, on the astronomical midpoint of a cross-quarter, or on the
/// nearest weekend do so by their own rule, which this table does not
/// carry.
///
/// Read from 1974, the year Aidan Kelly named the summer solstice and the
/// equinoxes' festivals, Litha, Ostara and Mabon, as Wikipedia dates it:
/// the same article says only that "Wheel of the Year" was in use "by the
/// mid-1960s" for a cycle of eight observances, with no year, and the
/// earlier years are gaps.
pub static WHEEL_OF_THE_YEAR: RuleSet = RuleSet {
    code: "wheel-of-the-year",
    english_name: "Wheel of the Year",
    rules: WHEEL_OF_THE_YEAR_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "Wikipedia, \"Wheel of the Year\", retrieved 2026-09-22, for the eight \
              festivals, their dates in each hemisphere and the cycle's \
              mid-twentieth-century origin. Secondary: no body defines the Wheel of the Year for all who keep it, so there is no primary to replace it",
    subdivisions: Subdivisions::Undivided,
};

/// The Wheel of the Year, as kept in the southern hemisphere: the same
/// eight festivals half a year round, so that Yule falls at the June
/// solstice and Samhain on 1 May.
pub static WHEEL_OF_THE_YEAR_SOUTH: RuleSet = RuleSet {
    code: "wheel-of-the-year-south",
    english_name: "Wheel of the Year (southern hemisphere)",
    rules: WHEEL_OF_THE_YEAR_SOUTH_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "Wikipedia, \"Wheel of the Year\", retrieved 2026-09-22, southern-hemisphere \
              column. Secondary: no body defines the Wheel of the Year for all who keep it, so there is no primary to replace it",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// Buddhism, as Thailand keeps it
// ─────────────────────────────────────────────────────────────────────────

/// Asalha Bucha, the full moon of month 8 of the Thai lunar calendar — in
/// an adhikamāsa year the second month 8, which `thai-lunar` writes as the
/// regular one — so a plain date in it.
pub(crate) static THAI_ASALHA_BUCHA: Rule = Rule::in_calendar(CalendarSystem::THAI_LUNAR, 8, 15);

/// Khao Phansa, แรม 1 ค่ำ เดือน 8, the day after Asalha Bucha.
pub(crate) static THAI_KHAO_PHANSA: Rule = Rule::in_calendar(CalendarSystem::THAI_LUNAR, 8, 16);

/// A day the Thai lunar calendar computes for a Buddhist Era year, in the
/// Gregorian year its Makha Bucha falls in.
fn thai_lunar_day(year: i64, day: fn(i64) -> hc_calendar::CalendarResult<Rd>) -> Days {
    match day(year + hc_calendars_regional::thai_lunar::BUDDHIST_ERA_OFFSET) {
        Ok(rd) => Days::one(rd),
        Err(_) => Days::new(),
    }
}

fn thai_makha_bucha(year: i64) -> Days {
    thai_lunar_day(year, hc_calendars_regional::thai_lunar::makha_bucha)
}

fn thai_visakha_bucha(year: i64) -> Days {
    thai_lunar_day(year, hc_calendars_regional::thai_lunar::visakha_bucha)
}

/// A Thai lunar day that moves a month in an adhikamāsa year, over the
/// years the calendar's table answers for.
const fn thai_moving(function: fn(i64) -> Days) -> Rule {
    Rule::Tabulated {
        function,
        first_year: hc_calendars_regional::thai_lunar::FIRST_GREGORIAN_YEAR,
        last_year: hc_calendars_regional::thai_lunar::LAST_GREGORIAN_YEAR,
    }
}

/// Makha Bucha, the full moon of month 3, or of month 4 in an adhikamāsa
/// year: the calendar's own function, since a date names one month.
pub(crate) const THAI_MAKHA_BUCHA: Rule = thai_moving(thai_makha_bucha);

/// Visakha Bucha, the full moon of month 6, or of month 7 in an
/// adhikamāsa year.
pub(crate) const THAI_VISAKHA_BUCHA: Rule = thai_moving(thai_visakha_bucha);

static BUDDHIST_THAI_RULES: &[HolidayRule] = &[
    feast("Makha Bucha", "วันมาฆบูชา", THAI_MAKHA_BUCHA),
    feast("Visakha Bucha", "วันวิสาขบูชา", THAI_VISAKHA_BUCHA),
    feast("Asalha Bucha", "วันอาสาฬหบูชา", THAI_ASALHA_BUCHA),
    feast("Khao Phansa", "วันเข้าพรรษา", THAI_KHAO_PHANSA),
];

/// Theravāda Buddhism's four holy days as Thailand dates them, on the Thai
/// lunar calendar, `thai-lunar`.
///
/// The calendar carries the year types Thailand published for 2535–2570 BE
/// and so answers exactly for 1992–2027 and not at all outside them, where
/// the four days are reported as gaps; the year types and how the days
/// move in an adhikamāsa year are in
/// `docs/systems/thai-lunar.md`.
/// These are the days the Bank of Thailand's lists date and the Thailand
/// table gives off.
///
/// Burma, Cambodia, Laos and Sri Lanka keep the same full moons on
/// calendars of their own, which do not always agree with the Thai one:
/// their days are in the Myanmar table on `burmese`, the Cambodian on
/// `khmer`, and the Lao and Sri Lankan tables, and no set here stands for
/// all of them. The end of the rains retreat, Ok Phansa, is not carried,
/// because no list read dates it.
pub static BUDDHIST_THAI: RuleSet = RuleSet {
    code: "buddhist-thai",
    english_name: "Buddhism (Thai Theravāda)",
    rules: BUDDHIST_THAI_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "The Makha, Visakha and Asalha Bucha (to 2006 Khao Phansa) dates of \
              the Bank of Thailand's financial-institution holiday lists for \
              1992-2022, as the Internet Archive keeps them, and of its \
              notifications FPG 3/2565, FPG 8/2566, FPG 5/2567, 31/2568 and \
              37/2569 for 2023-2027, from which `thai-lunar`'s year types are \
              read (docs/systems/thai-lunar.md; bot-fiholiday and the bot-* keys \
              in references.bib), retrieved 2026-09-23",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// Buddhism, East Asian
// ─────────────────────────────────────────────────────────────────────────

static BUDDHIST_EAST_ASIAN_RULES: &[HolidayRule] = &[
    // The Japanese days are on the Gregorian calendar from the Meiji
    // reform of 1873; the lunar reckoning follows its own calendar.
    feast("Nirvana Day", "涅槃会", Rule::gregorian(2, 15)).read_from(1873),
    feast("Buddha's Birthday", "灌仏会", Rule::gregorian(4, 8)).read_from(1873),
    feast(
        "Buddha's Birthday (lunar reckoning)",
        "佛誕",
        Rule::in_calendar(CalendarSystem::CHINESE, 4, 8),
    ),
    feast("Bodhi Day", "成道会", Rule::gregorian(12, 8)).read_from(1873),
];

/// East Asian Mahāyāna Buddhism: the Japanese days on the Gregorian
/// calendar, and Buddha's Birthday on the eighth of the fourth month of the
/// Chinese calendar, as China, Korea, Vietnam and the Chinese communities
/// keep it.
///
/// Japan keeps 灌仏会 on 8 April and 成道会 on 8 December, the lunar days
/// moved to the same Gregorian dates in the Meiji era, and 涅槃会 mostly
/// on 15 February. Some temples keep 涅槃会 on 15 March, a month late, and
/// some on 8 February; those are not carried. Hong Kong's and South
/// Korea's public holidays for the Birthday are the lunar reckoning here,
/// and their tables cite the ordinances.
pub static BUDDHIST_EAST_ASIAN: RuleSet = RuleSet {
    code: "buddhist-east-asian",
    english_name: "Buddhism (East Asian)",
    rules: BUDDHIST_EAST_ASIAN_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "Secondary sources only, all retrieved 2026-09-26: the Japanese \
              Wikipedia, \"灌仏会\", for 8 April in Japan and the Chinese \
              calendar's 4/8 elsewhere; Wikipedia, \"Bodhi Day\", for 8 December \
              in Japan since the Meiji era; Wikipedia, \"Parinirvana Day\", for \
              15 February as the date most keep; the Japanese Wikipedia, \
              \"涅槃会\", for 15 March in some temples. A temple's or a school's \
              own calendar would be the primary",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// Chinese folk religion
// ─────────────────────────────────────────────────────────────────────────

static CHINESE_NEW_YEAR: Rule = Rule::in_calendar(CalendarSystem::CHINESE, 1, 1);

/// 清明 at the Chinese meridian.
static QINGMING_IN_CHINA: Rule = Rule::SolarTerm {
    term: QINGMING,
    meridian: Meridian::CHINA,
};

/// 冬至 at the Chinese meridian.
static WINTER_SOLSTICE_IN_CHINA: Rule = Rule::SolarTerm {
    term: SolarTerm::WINTER_SOLSTICE,
    meridian: Meridian::CHINA,
};

/// 清明 by 平氣, the equal terms of the calendars before the 時憲曆 of
/// 1645: seven twenty-fourths of the mean tropical year after the winter
/// solstice, at the Chinese meridian.
///
/// The division is the one `hc-calendars-lunar` makes for its
/// [`SolarTermMode::Mean`](hc_calendars_lunar::lunisolar::SolarTermMode::Mean)
/// major terms — equal steps of the mean year from the solstice — taken at
/// the half step, since 清明 is a minor term. The solstice is the instant
/// this library computes, as for 寒食's count, and not the one the older
/// calendars' own constants gave, so the day is approximate; its order
/// against 寒食 is not, because both are counted from the same solstice.
fn qingming_by_mean_terms(year: i64) -> Days {
    let solstice = term_moment(year - 1, SolarTerm::WINTER_SOLSTICE);
    let qingming = Moment(solstice.0 + 7.0 * MEAN_TROPICAL_YEAR / 24.0);
    Days::one(Meridian::CHINA.day_of(qingming))
}

static CHINESE_FOLK_RULES: &[HolidayRule] = &[
    feast(
        "Chinese New Year's Eve",
        "除夕",
        Rule::Offset {
            base: &CHINESE_NEW_YEAR,
            days: -1,
        },
    ),
    feast(
        "Spring Festival",
        "春節",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 1),
    ),
    // "正月一日為雞……七日為人": the seventh day of the first month.
    feast(
        "Human Day",
        "人日",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 7),
    ),
    feast(
        "Lantern Festival",
        "元宵節",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 15),
    ),
    // Fixed at 三月初三 from the Wei–Jin; the earlier 三月上旬的巳日 is
    // not carried, see the set's documentation.
    feast(
        "Shangsi Festival",
        "上巳節",
        Rule::in_calendar(CalendarSystem::CHINESE, 3, 3),
    ),
    // 冬至後一百五日 until the 時憲曆 of 1645,
    // `hc_seasons::ColdFoodConvention::SolsticePlus105`; approximate,
    // because the solstice is today's astronomy, not the day the older
    // calendars' own reckoning of it gave. It falls one or two days before
    // the 清明 of the equal terms those calendars used.
    feast(
        "Cold Food Festival",
        "寒食節",
        Rule::Offset {
            base: &WINTER_SOLSTICE_IN_CHINA,
            days: hc_seasons::cold_food::DAYS_AFTER_SOLSTICE as i16,
        },
    )
    .years(None, Some(1644))
    .approximate(),
    // The day before 清明 from 1645, when the 時憲曆 shortened the
    // interval from the solstice, `ColdFoodConvention::EveOfQingming`.
    feast(
        "Cold Food Festival",
        "寒食節",
        Rule::Offset {
            base: &QINGMING_IN_CHINA,
            days: -1,
        },
    )
    .years(Some(1645), None),
    // 清明 by the equal terms until 1645, approximate as 寒食's count is,
    // and by the true ones, 定氣, from the 時憲曆.
    feast(
        "Qingming Festival",
        "清明節",
        Rule::Computed(qingming_by_mean_terms),
    )
    .years(None, Some(1644))
    .approximate(),
    feast("Qingming Festival", "清明節", QINGMING_IN_CHINA).years(Some(1645), None),
    feast(
        "Dragon Boat Festival",
        "端午節",
        Rule::in_calendar(CalendarSystem::CHINESE, 5, 5),
    ),
    feast(
        "Qixi Festival",
        "七夕",
        Rule::in_calendar(CalendarSystem::CHINESE, 7, 7),
    ),
    feast(
        "Ghost Festival",
        "中元節",
        Rule::in_calendar(CalendarSystem::CHINESE, 7, 15),
    ),
    feast(
        "Mid-Autumn Festival",
        "中秋節",
        Rule::in_calendar(CalendarSystem::CHINESE, 8, 15),
    ),
    feast(
        "Double Ninth Festival",
        "重陽節",
        Rule::in_calendar(CalendarSystem::CHINESE, 9, 9),
    ),
    feast(
        "Laba Festival",
        "臘八節",
        Rule::in_calendar(CalendarSystem::CHINESE, 12, 8),
    ),
    feast("Winter Solstice Festival", "冬至", WINTER_SOLSTICE_IN_CHINA),
];

/// Chinese folk religion and the festivals of the Chinese year.
///
/// 人日 on the seventh of the first month; 上巳 on the third of the third,
/// where it has been fixed since the Wei–Jin; and 寒食 on the day before
/// 清明 from 1645, when the 時憲曆 shortened the interval from the winter
/// solstice, and 105 days after the solstice before it, beside the
/// festivals of the year. The 寒食 of the years before 1645 is
/// [`Confidence::Approximate`]: its
/// solstice is the one this library computes, and the calendars of those
/// years reckoned their own, which is not modelled here.
///
/// Not carried: 上巳 in its older form, "三月上旬的巳日", the 巳 day of the
/// first ten days of the third month, which the source puts before the
/// Han–Wei change. It is not quite "the first 巳 day" — when that falls on
/// the eleventh or twelfth there is no 巳 day in the first ten — and it was
/// kept centuries before 1645, where `chinese` begins, so there is no year
/// in the calendar's range it would be true of. Both reckonings of 寒食
/// are named conventions of `hc-seasons` (`ColdFoodConvention`), which
/// gives either for any year; the count of 106 days that some texts give
/// is not carried. 小年 is in the regional tables beside this one, one per
/// region.
pub static CHINESE_FOLK: RuleSet = RuleSet {
    code: "chinese-folk",
    english_name: "Chinese folk tradition",
    rules: CHINESE_FOLK_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "The Chinese lunisolar calendar and the 24 solar terms, both \
              computed at the Beijing meridian by `hc-calendars-lunar` and \
              `hc-seasons`; Wikipedia (zh), \"人日\", for 正月初七 after 董勛's \
              答問禮俗說; Wikipedia (zh), \"上巳節\", for 三月初三 and the earlier \
              三月上旬的巳日; Wikipedia (zh), \"寒食节\", for 清明前一日 after the \
              時憲曆 of 1645 (`wikipedia-zh-hanshi`), all retrieved 2026-09-26 \
              (secondary)",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// 小年, one table per region
// ─────────────────────────────────────────────────────────────────────────

/// Where every 小年 table takes its day from.
const XIAONIAN_SOURCES: &str = "Wikipedia (zh), \"小年\", retrieved 2026-09-26 \
    (`wikipedia-zh-xiaonian`, secondary), for the regions and their days; 新华社, \
    \"今天明天，都是小年！\", republished by 共产党员网 on 10 February 2026, \
    https://m.12371.gov.cn/content/2026-02/10/content_506756.html \
    (`xinhua-xiaonian-2026`), \"今天，腊月二十三是北方小年，明天，腊月二十四是南方小年\", \
    retrieved 2026-09-26 and re-read 2026-09-27, for the northern and southern days \
    of 2026";

/// A 小年 table: one day of the Chinese calendar, kept as the Little New
/// Year in one region.
macro_rules! xiaonian {
    ($(#[$doc:meta])* $set:ident, $rules:ident, $code:literal, $region:literal, $rule:expr) => {
        static $rules: &[HolidayRule] = &[feast(
            concat!("Little New Year (", $region, ")"),
            "小年",
            $rule,
        )];

        $(#[$doc])*
        pub static $set: RuleSet = RuleSet {
            code: $code,
            english_name: concat!("Little New Year (", $region, ")"),
            rules: $rules,
            substitution: &[],
            bridges: &[],
            includes: &[],
            weekend: NO_WEEKEND,
            sources_checked: SourceDate::new(2026, 9, 27),
            sources: XIAONIAN_SOURCES,
            subdivisions: Subdivisions::Undivided,
        };
    };
}

xiaonian!(
    /// 小年 in the north: 腊月廿三, the Kitchen God's day as the court kept
    /// it, which "北方受官方祭祀影响，渐以廿三过小年为多".
    CHINESE_XIAONIAN_NORTH,
    CHINESE_XIAONIAN_NORTH_RULES,
    "chinese-xiaonian-north",
    "north",
    Rule::in_calendar(CalendarSystem::CHINESE, 12, 23)
);

xiaonian!(
    /// 小年 in much of the south: 腊月廿四, where "南方多地则延续廿四旧俗".
    CHINESE_XIAONIAN_SOUTH,
    CHINESE_XIAONIAN_SOUTH_RULES,
    "chinese-xiaonian-south",
    "south",
    Rule::in_calendar(CalendarSystem::CHINESE, 12, 24)
);

xiaonian!(
    /// 小年 in Jiangnan and the Wu-speaking area, where 除夕 is 大年夜 and
    /// the night before it 小年夜, and in most of Fujian and in Taiwan,
    /// which keep the same day: two days before the New Year.
    CHINESE_XIAONIAN_JIANGNAN,
    CHINESE_XIAONIAN_JIANGNAN_RULES,
    "chinese-xiaonian-jiangnan",
    "Jiangnan, Fujian and Taiwan",
    Rule::Offset {
        base: &CHINESE_NEW_YEAR,
        days: -2,
    }
);

xiaonian!(
    /// 小年 around Nanjing, which gives the name to 正月十五, the Lantern
    /// Festival.
    CHINESE_XIAONIAN_NANJING,
    CHINESE_XIAONIAN_NANJING_RULES,
    "chinese-xiaonian-nanjing",
    "Nanjing",
    Rule::in_calendar(CalendarSystem::CHINESE, 1, 15)
);

xiaonian!(
    /// 小年 in parts of Sichuan, Chongqing, Guizhou and Yunnan, where 除夕
    /// itself is "过小年".
    CHINESE_XIAONIAN_SOUTHWEST,
    CHINESE_XIAONIAN_SOUTHWEST_RULES,
    "chinese-xiaonian-southwest",
    "south-west",
    Rule::Offset {
        base: &CHINESE_NEW_YEAR,
        days: -1,
    }
);

// ─────────────────────────────────────────────────────────────────────────
// Taoism
// ─────────────────────────────────────────────────────────────────────────

static TAOIST_RULES: &[HolidayRule] = &[
    feast(
        "Upper Yuan Festival (birthday of the Official of Heaven)",
        "上元節",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 15),
    ),
    feast(
        "Birthday of Mazu",
        "媽祖生日",
        Rule::in_calendar(CalendarSystem::CHINESE, 3, 23),
    ),
    feast(
        "Middle Yuan Festival (birthday of the Official of Earth)",
        "中元節",
        Rule::in_calendar(CalendarSystem::CHINESE, 7, 15),
    ),
    feast(
        "Ascension of Mazu",
        "",
        Rule::in_calendar(CalendarSystem::CHINESE, 9, 9),
    ),
    feast(
        "Lower Yuan Festival (birthday of the Official of Water)",
        "下元節",
        Rule::in_calendar(CalendarSystem::CHINESE, 10, 15),
    ),
];

/// Taoist days on the Chinese calendar: the three Yuan festivals, the
/// birthdays of the Three Officials, 三官大帝 — 天官 on the fifteenth of the
/// first month, 地官 of the seventh and 水官 of the tenth — and Mazu's
/// birthday on the twenty-third of the third month and her death, her
/// ascension, on the Double Ninth, the ninth of the ninth.
///
/// 上元 and 中元 fall on the days of the Chinese folk table's 元宵 and
/// 中元; they are the same days under their Taoist names. The other
/// deities' days are not carried: they want a temple's or an almanac's
/// list, and none was read.
pub static TAOIST: RuleSet = RuleSet {
    code: "taoist",
    english_name: "Taoism",
    rules: TAOIST_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 27),
    sources: "Secondary sources only, all retrieved 2026-09-26: Wikipedia (zh), \
              \"下元節\", for the Three Officials' birthdays on 正月十五, 七月十五 and \
              十月十五; Wikipedia, \"Mazu\", for her birthday on the 23rd day of the 3rd \
              month and her death on the Double Ninth; TVBS News, 18 April 2025, \
              https://news.tvbs.com.tw/life/2836258 (`tvbs-mazu-2025`), for \
              \"媽祖生日日期落在農曆3月23日即是國曆4月20日\", re-read 2026-09-27. A \
              temple's own calendar would be the primary",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// Korean folk days
// ─────────────────────────────────────────────────────────────────────────

/// A day of the Korean lunar calendar, `dangi`.
const fn dangi(month: u8, day: u8) -> Rule {
    Rule::in_calendar(CalendarSystem::DANGI, month, day)
}

/// 설날, the first day of the Korean year.
static SEOLLAL: Rule = dangi(1, 1);

/// 동지, the winter solstice, at the Korean meridian.
static DONGJI: Rule = Rule::SolarTerm {
    term: SolarTerm::WINTER_SOLSTICE,
    meridian: Meridian::KOREA,
};

static KOREAN_FOLK_RULES: &[HolidayRule] = &[
    HolidayRule::observance("Jeongwol Daeboreum", "정월대보름", dangi(1, 15)),
    HolidayRule::observance("Yeongdeung Day", "영등날", dangi(2, 1)),
    HolidayRule::observance("Samjinnal", "삼짇날", dangi(3, 3)),
    // 105 days after 동지, the solstice not counted, as the Korea Astronomy
    // and Space Science Institute dates it; `hc_seasons::ColdFoodConvention::Hansik`.
    HolidayRule::observance(
        "Hansik",
        "한식",
        Rule::Offset {
            base: &DONGJI,
            days: hc_seasons::cold_food::DAYS_AFTER_SOLSTICE as i16,
        },
    ),
    HolidayRule::observance("Dano", "단오", dangi(5, 5)),
    HolidayRule::observance("Yudu", "유두", dangi(6, 15)),
    HolidayRule::observance("Chilseok", "칠석", dangi(7, 7)),
    HolidayRule::observance("Baekjung", "백중", dangi(7, 15)),
    HolidayRule::observance("Jungyangjeol", "중양절", dangi(9, 9)),
    HolidayRule::observance("Siwol Boreum", "시월보름", dangi(10, 15)),
    HolidayRule::observance(
        "Seotdal Geumeum",
        "섣달그믐",
        Rule::Offset {
            base: &SEOLLAL,
            days: -1,
        },
    ),
];

/// The Korean folk days, 명절 and 세시 days, on the Korean lunar calendar,
/// `dangi`: 정월대보름, the first full moon, on 1/15; 영등날 on 2/1;
/// 삼짇날 on 3/3; 단오 on 5/5; 유두 on 6/15; 칠석 on 7/7; 백중 on 7/15;
/// 중양절 on 9/9; 시월보름, the 下元, on 10/15; 섣달그믐, the last day of
/// the year, the eve of 설날; and 한식, 105 days after 동지.
///
/// None is a day off; South Korea's table carries 설날 and 추석, which are.
/// 한식 is counted from the solstice's day at the Korean meridian, the
/// solstice not counted: the Korea Astronomy and Space Science Institute's
/// 한식 of 5 April 2024, 5 April 2025 and 6 April 2026 require that, and
/// counting the solstice as the first day would put each a day early
/// (`docs/systems/solar-term-counts.md`). The lunar days are in the ordinary
/// month of their number when the year repeats it.
pub static KOREAN_FOLK: RuleSet = RuleSet {
    code: "korean-folk",
    english_name: "Korean folk days",
    rules: KOREAN_FOLK_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "Wikipedia (ko), \"한국의 명절\", for the lunar days, including 영등날 on \
              2/1 and 시월보름 on 10/15 (secondary); Encyclopedia of Korean Culture \
              (한국민족문화대백과사전, encykorea.aks.ac.kr), \"명절\" (임동권), \"세시풍속\" \
              (김명자) and \"한식\" (김선풍), for the days and for 한식 \"동지로부터 105일째 되는 \
              날\" on 청명 or the day after (`aks-hansik`); the Korea Astronomy and Space \
              Science Institute's 월력요항 notices for 2024, 2025 and 2026 (kasi.re.kr), \
              for 한식, 단오 and 칠석 in those years (`kasi-wollyeok`); all retrieved \
              2026-09-26",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// Vietnamese folk days
// ─────────────────────────────────────────────────────────────────────────

/// A day of the Vietnamese lunar calendar, `vietnamese`.
const fn am_lich(name: &'static str, local: &'static str, month: u8, day: u8) -> HolidayRule {
    HolidayRule::observance(
        name,
        local,
        Rule::in_calendar(CalendarSystem::VIETNAMESE, month, day),
    )
}

static VIETNAMESE_FOLK_RULES: &[HolidayRule] = &[
    am_lich("Khai Ha Festival", "Tết Khai Hạ", 1, 7),
    am_lich("Lantern Festival", "Tết Nguyên Tiêu", 1, 15),
    am_lich("Cold Food Festival", "Tết Hàn Thực", 3, 3),
    am_lich("Double Fifth Festival", "Tết Đoan Ngọ", 5, 5),
    am_lich("Vu Lan (Ghost Festival)", "Tết Trung Nguyên", 7, 15),
    am_lich("Mid-Autumn Festival", "Tết Trung Thu", 8, 15),
    am_lich("Double Ninth Festival", "Tết Trùng Cửu", 9, 9),
    am_lich("Double Tenth Festival", "Tết Trùng Thập", 10, 10),
    am_lich("Lower Yuan Festival (new rice)", "Tết Hạ Nguyên", 10, 15),
    am_lich("Kitchen Gods' Day", "Ông Công Ông Táo", 12, 23),
];

/// The Vietnamese folk days on the Vietnamese lunar calendar, `vietnamese`,
/// as the Vietnam News Agency lists "những ngày tết tính theo âm lịch":
/// Khai Hạ on 1/7, Nguyên Tiêu on 1/15, Hàn Thực on 3/3, Đoan Ngọ on 5/5,
/// Trung Nguyên, the Vu Lan, on 7/15, Trung Thu on 8/15, Trùng Cửu on 9/9,
/// Trùng Thập on 10/10, Hạ Nguyên on 10/15, and Ông Táo on 12/23.
///
/// None is a day off; Vietnam's table carries Tết, which is. Vietnam's Tết
/// Hàn Thực is the lunar third of the third, not a count from a solar term
/// as the Chinese and Korean Cold Food days are. The list's Tết Thanh Minh,
/// "trong tháng Ba", in the third month, is a solar term and not given a
/// date by it; it is the Chinese table's 清明 at the Chinese meridian, and
/// is not repeated here at Vietnam's.
pub static VIETNAMESE_FOLK: RuleSet = RuleSet {
    code: "vietnamese-folk",
    english_name: "Vietnamese folk days",
    rules: VIETNAMESE_FOLK_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 27),
    sources: "Ngô Trọng Bình, \"Ý nghĩa của những ngày tết tính theo âm lịch\", TTXVN/Vietnam+ \
              (vietnamplus.vn), 21 January 2012 (`vnplus-tet-am-lich`), for the days; \
              Vietnam+, 15 January 2025 (`vnplus-ong-tao-2025`), for Ông Táo on 22 January \
              2025, and VietNamNet, 28 September 2025 (`vietnamnet-trung-thu-2025`), for \
              Trung Thu on 6 October 2025, as checks; all retrieved 2026-09-26, the two \
              checks re-read 2026-09-27 at the URLs their entries give",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// The Japanese 五節句
// ─────────────────────────────────────────────────────────────────────────

/// A 節句 on its Gregorian date, from 1873, when Japan took up the
/// Gregorian calendar and the Council of State abolished the five as
/// official festivals.
const fn sekku(name: &'static str, local: &'static str, month: u8, day: u8) -> HolidayRule {
    HolidayRule::observance(name, local, Rule::gregorian(month, day)).years(Some(1873), None)
}

static GOSEKKU_RULES: &[HolidayRule] = &[
    sekku("Jinjitsu (Festival of Seven Herbs)", "人日", 1, 7),
    sekku("Joshi (Peach Festival)", "上巳", 3, 3),
    sekku("Tango (Iris Festival)", "端午", 5, 5),
    sekku("Tanabata (Bamboo Festival)", "七夕", 7, 7),
    sekku("Choyo (Chrysanthemum Festival)", "重陽", 9, 9),
];

/// The five seasonal festivals of Japan, the 五節句, on the Gregorian dates
/// they have been kept on since 1873: 人日, 七草の節句, on 7 January; 上巳,
/// 桃の節句, on 3 March; 端午, 菖蒲の節句, on 5 May; 七夕, 笹の節句, on 7
/// July; 重陽, 菊の節句, on 9 September.
///
/// The Council of State's notice No. 1 of 4 January 1873, 「五節ヲ廃シ祝日ヲ
/// 定ム」, abolished them as official festivals, and the year before they
/// were the lunar dates of the Tenpō calendar, so the table starts in 1873.
/// 端午 is also こどもの日, a public holiday in Japan's table. The National
/// Astronomical Observatory's 伝統的七夕, the old 七夕 restated as a count
/// from 処暑, is `hc_seasons::moon_calendar::traditional_tanabata`; the 月遅れ
/// reckoning a month later is in no source read and not carried.
pub static GOSEKKU: RuleSet = RuleSet {
    code: "gosekku",
    english_name: "The five seasonal festivals of Japan (五節句)",
    rules: GOSEKKU_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "Wikipedia (ja), \"節句\", retrieved 2026-09-26, for the five, their \
              names and dates, and their abolition by 太政官第1号布告 of 4 January 1873 \
              (the notice itself not read; secondary)",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// お盆 and the Japanese folk days of the sexagenary cycle
// ─────────────────────────────────────────────────────────────────────────

/// The earthly branch of 午 days.
const UMA: u8 = branch::WU;
/// The earthly branch of 酉 days.
const TORI: u8 = branch::YOU;
/// The earthly branch of 亥 days.
const INOSHISHI: u8 = branch::HAI;

/// The first year the Japanese 旧暦 is taken to answer for: 1844, when the
/// 天保暦 took effect, since the unbounded 天保暦 rules are what the almanacs
/// have continued, and the calendars before it reckoned differently.
const KYUREKI_FIRST_YEAR: i32 = 1844;

/// The last year the Japanese 旧暦 is taken to answer for. The 天保暦 rule
/// cannot number the months of 2033–34, which the rules below report as
/// gaps. The Observatory lists 2147–48 as the next years it leaves open,
/// without saying which months (`nao-rekiwiki-2033`), so nothing later is
/// answered.
const KYUREKI_LAST_YEAR: i32 = 2146;

/// The first year of the Gregorian calendar in Japan, 1873, from which the
/// Gregorian-dated folk days are answered.
const JAPANESE_GREGORIAN_FIRST_YEAR: i32 = 1873;

/// The `n`th day, counting from 1, of the days from `first` to `last` whose
/// earthly branch is `branch`; none when the span holds fewer.
fn nth_branch_day(first: Rd, last: Rd, branch: u8, n: i64) -> Days {
    let day = Rd(branch_day_on_or_after(first, branch).0 + 12 * (n - 1));
    if day.0 <= last.0 {
        Days::one(day)
    } else {
        Days::new()
    }
}

/// A day of the Japanese 旧暦, the 天保暦's rules continued at the Japanese
/// meridian (`hc_calendars_lunar::japanese_tenpo::UNBOUNDED_PARAMETERS`), in
/// the ordinary month of its number.
fn kyureki_day(year: i64, month: u8, day: u8) -> Option<Rd> {
    japanese_tenpo::UNBOUNDED_PARAMETERS
        .to_fixed(year, Month::regular(month), day)
        .ok()
}

/// The first and last day of an ordinary month of the Japanese 旧暦.
fn kyureki_month(year: i64, month: u8) -> Option<(Rd, Rd)> {
    let first = kyureki_day(year, month, 1)?;
    let length = japanese_tenpo::UNBOUNDED_PARAMETERS.days_in_month(year, Month::regular(month))?;
    Some((first, Rd(first.0 + i64::from(length) - 1)))
}

/// Whether a 旧暦 month is not answered: in a year outside
/// [`KYUREKI_FIRST_YEAR`] to [`KYUREKI_LAST_YEAR`], or one whose numbering
/// the 天保暦 rule leaves open, the eighth to the twelfth month of 2033,
/// which the Observatory's three resolutions number differently
/// (`nao-rekiwiki-2033`). The rules report either as a gap.
const fn kyureki_month_unsettled(year: i64, month: u8) -> bool {
    year < KYUREKI_FIRST_YEAR as i64
        || year > KYUREKI_LAST_YEAR as i64
        || (year == 2033 && month >= 8)
}

/// Rules over the days of a month whose earthly branch is given: the `n`th
/// such day of a Gregorian month, or of an ordinary 旧暦 month, a gap in a
/// year the 旧暦 does not answer and where the 2033 problem leaves that
/// month's numbering open.
macro_rules! branch_day_rules {
    ($(gregorian $name:ident = $month:literal, $branch:ident, $n:literal;)*) => {$(
        fn $name(year: i64) -> Days {
            crate::rule::gregorian_month_span(year, $month).map_or(Days::new(), |(first, last)| {
                nth_branch_day(first, last, $branch, $n)
            })
        }
    )*};
    ($(kyureki $name:ident = $month:literal, $branch:ident, $n:literal;)*) => {$(
        fn $name(year: i64) -> Option<Days> {
            if kyureki_month_unsettled(year, $month) {
                return None;
            }
            Some(kyureki_month(year, $month).map_or(Days::new(), |(first, last)| {
                nth_branch_day(first, last, $branch, $n)
            }))
        }
    )*};
}

branch_day_rules! {
    gregorian ichi_no_tori = 11, TORI, 1;
    gregorian ni_no_tori = 11, TORI, 2;
    gregorian san_no_tori = 11, TORI, 3;
    gregorian hatsuuma = 2, UMA, 1;
    gregorian ni_no_uma = 2, UMA, 2;
    gregorian san_no_uma = 2, UMA, 3;
    gregorian inoko_november = 11, INOSHISHI, 1;
}

branch_day_rules! {
    kyureki hatsuuma_kyureki = 2, UMA, 1;
    kyureki inoko_kyureki = 10, INOSHISHI, 1;
}

/// Rules for a numbered day of an ordinary 旧暦 month, a gap in a year the
/// 旧暦 does not answer and where the 2033 problem leaves the month open.
macro_rules! kyureki_day_rules {
    ($($name:ident = $month:literal, $day:literal;)*) => {$(
        fn $name(year: i64) -> Option<Days> {
            if kyureki_month_unsettled(year, $month) {
                return None;
            }
            Some(kyureki_day(year, $month, $day).map_or(Days::new(), Days::one))
        }
    )*};
}

kyureki_day_rules! {
    kyureki_7_13 = 7, 13;
    kyureki_7_14 = 7, 14;
    kyureki_7_15 = 7, 15;
    kyureki_10_10 = 10, 10;
}

/// 新暦 13 July, the first day of 7月盆.
static JULY_13: Rule = Rule::gregorian(7, 13);
/// 新暦 16 July, its last.
static JULY_16: Rule = Rule::gregorian(7, 16);
/// 13 August, the first day of 月遅れ盆.
static AUGUST_13: Rule = Rule::gregorian(8, 13);
/// 16 August, its last.
static AUGUST_16: Rule = Rule::gregorian(8, 16);
/// 旧暦 7/13, the first day of 旧盆.
static KYUREKI_7_13: Rule = Rule::Unsettled(kyureki_7_13);
/// 旧暦 7/15, its last in Okinawa.
static KYUREKI_7_15: Rule = Rule::Unsettled(kyureki_7_15);

/// お盆 on a Gregorian month: the four days from the 13th to the 16th, the
/// 迎え火 on the evening of the 13th and the 送り火 on the 16th, from 1873.
macro_rules! gregorian_obon {
    ($first:ident, $last:ident, $month:literal) => {
        &[
            feast("Obon", "お盆", Rule::span(&$first, &$last))
                .years(Some(JAPANESE_GREGORIAN_FIRST_YEAR), None),
            feast(
                "Mukaebi (welcoming fire)",
                "迎え火",
                Rule::gregorian($month, 13),
            )
            .years(Some(JAPANESE_GREGORIAN_FIRST_YEAR), None),
            feast(
                "Okuribi (sending-off fire)",
                "送り火",
                Rule::gregorian($month, 16),
            )
            .years(Some(JAPANESE_GREGORIAN_FIRST_YEAR), None),
        ]
    };
}

/// Where the お盆 tables come from.
const OBON_SOURCES: &str = "Wikipedia (ja), \"お盆\", retrieved 2026-09-27 \
    (`wikipedia-ja-obon`): 新暦7月15日 in Tokyo and parts of Tōhoku, Hokuriku and \
    Shizuoka, 新暦8月15日, 月遅れ, in most of Japan, the period 13 to 16, 迎え火 on the \
    evening of the 13th and 送り火 on the 16th; 旧暦7月15日 in Okinawa and Amami, with \
    Okinawa's ウンケー on 7/13, ナカビ on 7/14 and ウークイ on 7/15, and 閏7月15日 not \
    旧盆 (secondary; the Okinawa prefectural office's Q&A it cites not read)";

/// お盆 on 新暦 7 月, 7月盆: 13 to 16 July, the practice of Tokyo, where the
/// Meiji government's calendar reform put it, and of parts of Tōhoku and
/// Hokuriku, from 1873. `obon-august` is the month-late reckoning most of
/// Japan keeps and `obon-lunar` the 旧暦 one of Okinawa and Amami
/// (`docs/policy.md` §5; `docs/systems/east-asian-folk-days.md`).
///
/// The 迎え火 is on the evening of the 13th and the 送り火 on the 16th; the
/// places that light the 送り火 on the 15th, and those that keep お盆 on the
/// weekend nearest the 15th, are not carried.
pub static OBON_JULY: RuleSet = RuleSet {
    code: "obon-july",
    english_name: "Obon on the Gregorian July (7月盆)",
    rules: gregorian_obon!(JULY_13, JULY_16, 7),
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 27),
    sources: OBON_SOURCES,
    subdivisions: Subdivisions::Undivided,
};

/// お盆 a month late, 月遅れ盆: 13 to 16 August, as most of Japan keeps it,
/// from 1873, with the 迎え火 on the 13th and the 送り火 on the 16th. None
/// of the days is a public holiday; the private sector's summer break is
/// not carried. The places that keep it on 1 August are not carried.
pub static OBON_AUGUST: RuleSet = RuleSet {
    code: "obon-august",
    english_name: "Obon a month late (月遅れ盆)",
    rules: gregorian_obon!(AUGUST_13, AUGUST_16, 8),
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 27),
    sources: OBON_SOURCES,
    subdivisions: Subdivisions::Undivided,
};

static OBON_LUNAR_RULES: &[HolidayRule] = &[
    feast("Kyūbon", "旧盆", Rule::span(&KYUREKI_7_13, &KYUREKI_7_15)),
    feast(
        "Unkē (welcoming the ancestors)",
        "ウンケー",
        Rule::Unsettled(kyureki_7_13),
    ),
    feast("Nakabi", "ナカビ", Rule::Unsettled(kyureki_7_14)),
    feast(
        "Ukui (sending off the ancestors)",
        "ウークイ",
        Rule::Unsettled(kyureki_7_15),
    ),
];

/// 旧盆, お盆 on the 旧暦: 7/13 to 7/15, as Okinawa keeps it, with its
/// names, ウンケー on the 13th, ナカビ on the 14th and ウークイ on the 15th.
///
/// The 旧暦 is the one Japanese almanacs print, the 天保暦's rules
/// continued at the Japanese meridian, from 1844, when the 天保暦 took
/// effect, to 2146. The fifteenth falls from 8 August to 7 September, and a
/// leap seventh month does not repeat it: in 2006 旧盆 was 8 August, and
/// the 15th of that year's leap month, 7 September, was not 旧盆. The
/// places that keep ウークイ on the 16th, and the Miyako and Yaeyama names,
/// are not carried.
pub static OBON_LUNAR: RuleSet = RuleSet {
    code: "obon-lunar",
    english_name: "Obon on the Japanese lunar calendar (旧盆)",
    rules: OBON_LUNAR_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 27),
    sources: OBON_SOURCES,
    subdivisions: Subdivisions::Undivided,
};

static TORI_NO_ICHI_RULES: &[HolidayRule] = &[
    feast("Ichi no Tori", "一の酉", Rule::Computed(ichi_no_tori))
        .years(Some(JAPANESE_GREGORIAN_FIRST_YEAR), None),
    feast("Ni no Tori", "二の酉", Rule::Computed(ni_no_tori))
        .years(Some(JAPANESE_GREGORIAN_FIRST_YEAR), None),
    feast("San no Tori", "三の酉", Rule::Computed(san_no_tori))
        .years(Some(JAPANESE_GREGORIAN_FIRST_YEAR), None),
];

/// 酉の市, the fair of the 酉 days of November at the 鷲 and 大鳥 shrines of
/// the Kantō: 一の酉, 二の酉 and, when the first falls on 1 to 6 November,
/// 三の酉, from 1873. Asakusa's 鷲神社 holds it from midnight to midnight
/// of each 酉 day.
///
/// A shrine's own day is not carried: the 鷲宮神社's 大酉祭 on the first 酉
/// of December and 川越's 熊野神社 on 3 December.
pub static TORI_NO_ICHI: RuleSet = RuleSet {
    code: "tori-no-ichi",
    english_name: "Tori no Ichi, the fairs of the Rooster days (酉の市)",
    rules: TORI_NO_ICHI_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 27),
    sources: "Wikipedia (ja), \"酉の市\", retrieved 2026-09-27 (`wikipedia-ja-tori-no-ichi`), \
              for the 酉 days of November and the third 酉 when the first is on 1 to 6 \
              November; 鷲神社 (Asakusa), 「今年の酉の市」 and 「年間祭事」, retrieved \
              2026-09-27 (`otorisama-kotoshi`), for 7 and 19 November 2026, 二の酉まで",
    subdivisions: Subdivisions::Undivided,
};

static HATSUUMA_RULES: &[HolidayRule] = &[
    feast("Hatsuuma", "初午", Rule::Computed(hatsuuma))
        .years(Some(JAPANESE_GREGORIAN_FIRST_YEAR), None),
    feast("Ni no Uma", "二の午", Rule::Computed(ni_no_uma))
        .years(Some(JAPANESE_GREGORIAN_FIRST_YEAR), None),
    feast("San no Uma", "三の午", Rule::Computed(san_no_uma))
        .years(Some(JAPANESE_GREGORIAN_FIRST_YEAR), None),
];

/// Where the two 初午 tables come from.
const HATSUUMA_SOURCES: &str = "Wikipedia (ja), \"初午\", retrieved 2026-09-27 \
    (`wikipedia-ja-hatsuuma`), for the first 午 day of February on the Gregorian calendar \
    today and of the second month of the 旧暦 before the reform and in some places now, and \
    for 二の午 and 三の午; 伏見稲荷大社, 「祭礼と行事」, for 初午大祭 on 「2月初午の日」, and \
    京都観光Navi for its day, 1 February 2026 (`inari-hatsuuma`); HugKum, 「2026年の「初午」はいつ？」 \
    (`hugkum-hatsuuma-2026`), for 2025–2029 on the Gregorian calendar and 21 March 2026 on the \
    旧暦; all retrieved 2026-09-27";

/// 初午, the festival of the Inari shrines, on the Gregorian calendar: the
/// first 午 day of February, with 二の午 and 三の午, the second and third,
/// from 1873. `hatsuuma-lunar` is the first 午 day of the 旧暦's second
/// month, which some shrines keep.
///
/// Not carried, each a reckoning of its own that no dated source read
/// gives for more than one shrine: the first 午 of Gregorian March, as the
/// 箭弓稲荷神社 keeps it; the first 午 after 節分, as the 豊川稲荷東京別院
/// is reported to; the first 午 after the lunar New Year.
pub static HATSUUMA: RuleSet = RuleSet {
    code: "hatsuuma",
    english_name: "Hatsuuma, the first Horse day of February (初午)",
    rules: HATSUUMA_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 27),
    sources: HATSUUMA_SOURCES,
    subdivisions: Subdivisions::Undivided,
};

static HATSUUMA_LUNAR_RULES: &[HolidayRule] =
    &[feast("Hatsuuma", "初午", Rule::Unsettled(hatsuuma_kyureki))];

/// 初午 on the 旧暦: the first 午 day of the second month, the day before
/// the reform and in some places now, on the 旧暦 Japanese almanacs print,
/// 1844 to 2146. The first 午 of an ordinary month comes before any 午 of
/// a leap month of the same number, so a leap second month changes
/// nothing.
pub static HATSUUMA_LUNAR: RuleSet = RuleSet {
    code: "hatsuuma-lunar",
    english_name: "Hatsuuma on the Japanese lunar calendar (旧暦の初午)",
    rules: HATSUUMA_LUNAR_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 27),
    sources: HATSUUMA_SOURCES,
    subdivisions: Subdivisions::Undivided,
};

/// Where the two 亥の子 tables come from.
const INOKO_SOURCES: &str = "Wikipedia (ja), \"亥の子\", retrieved 2026-09-27 \
    (`wikipedia-ja-inoko`), for the first 亥 day of the 旧暦's tenth month, 亥の月, mainly in \
    western Japan; KOYOMI NOTE, 「亥の子（いのこ）」 (`koyominote-inoko`), for its days of \
    2020–2028 on the 旧暦; All About, 「2025年こたつや暖房器具を出す日はいつ？」 (三浦康子, \
    `allabout-inoko-2025`), for 「現在は一般的に新暦11月の最初の亥の日で考える」 and 2 November \
    2025; all retrieved 2026-09-27";

static INOKO_RULES: &[HolidayRule] = &[HolidayRule::observance(
    "Inoko",
    "亥の子",
    Rule::Unsettled(inoko_kyureki),
)];

/// 亥の子 on the 旧暦: the first 亥 day of the tenth month, 亥の月, kept
/// mainly in western Japan with 亥の子餅 and the 亥の子石, 1844 to 2146.
/// `inoko-november` is the first 亥 day of Gregorian November, the day most
/// now reckon it by. 2033 is a gap: the Observatory's three resolutions of
/// the 2033 problem put that year's tenth month on 23 October or on
/// 22 November.
pub static INOKO: RuleSet = RuleSet {
    code: "inoko",
    english_name: "Inoko, the first Boar day of the tenth lunar month (亥の子)",
    rules: INOKO_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 27),
    sources: INOKO_SOURCES,
    subdivisions: Subdivisions::Undivided,
};

static INOKO_NOVEMBER_RULES: &[HolidayRule] =
    &[
        HolidayRule::observance("Inoko", "亥の子", Rule::Computed(inoko_november))
            .years(Some(JAPANESE_GREGORIAN_FIRST_YEAR), None),
    ];

/// 亥の子 on the Gregorian calendar: the first 亥 day of November, from
/// 1873, which All About gives as the usual reckoning today.
pub static INOKO_NOVEMBER: RuleSet = RuleSet {
    code: "inoko-november",
    english_name: "Inoko, the first Boar day of November (亥の子)",
    rules: INOKO_NOVEMBER_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 27),
    sources: INOKO_SOURCES,
    subdivisions: Subdivisions::Undivided,
};

/// Where the two 十日夜 tables come from.
const TOKANYA_SOURCES: &str = "Wikipedia (ja), \"十日夜\", retrieved 2026-09-27 \
    (`wikipedia-ja-tokanya`), for 旧暦10月10日, mainly in northern Kantō, Kōshin'etsu and \
    southern Tōhoku; 暦生活 (新日本カレンダー), 「十日夜 とおかんや」 (`koyomiseikatsu-tokanya-2023`), \
    for 22 November 2023; 日本文化研究ブログ, 「「十日夜」2026年はいつ？」 (`jpnculture-tokanya`), \
    for 11月10日 in many places and 18 November 2026 on the 旧暦; all retrieved 2026-09-27";

static TOKANYA_RULES: &[HolidayRule] = &[HolidayRule::observance(
    "Tōkanya",
    "十日夜",
    Rule::Unsettled(kyureki_10_10),
)];

/// 十日夜 on the 旧暦: the tenth day of the tenth month, the harvest
/// festival of eastern Japan that answers the west's 亥の子, 1844 to 2146.
/// `tokanya-november` is 10 November, where many places keep it now. 2033
/// is a gap, as for [`INOKO`].
pub static TOKANYA: RuleSet = RuleSet {
    code: "tokanya",
    english_name: "Tōkanya, the tenth night of the tenth lunar month (十日夜)",
    rules: TOKANYA_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 27),
    sources: TOKANYA_SOURCES,
    subdivisions: Subdivisions::Undivided,
};

static TOKANYA_NOVEMBER_RULES: &[HolidayRule] =
    &[
        HolidayRule::observance("Tōkanya", "十日夜", Rule::gregorian(11, 10))
            .years(Some(JAPANESE_GREGORIAN_FIRST_YEAR), None),
    ];

/// 十日夜 on 10 November, the 旧暦 date moved a month onto the Gregorian
/// calendar, where many places keep it, from 1873.
pub static TOKANYA_NOVEMBER: RuleSet = RuleSet {
    code: "tokanya-november",
    english_name: "Tōkanya on 10 November (十日夜)",
    rules: TOKANYA_NOVEMBER_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 27),
    sources: TOKANYA_SOURCES,
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// Tibetan Buddhism
// ─────────────────────────────────────────────────────────────────────────

/// A numbered day of the Phugpa calendar, `tibetan`, kept by `when` where
/// the number is skipped or repeated; see [`Rule::TibetanDay`].
const fn duchen(
    name: &'static str,
    month: TibetanMonth,
    day: u8,
    when: TibetanDayRule,
) -> HolidayRule {
    feast(
        name,
        "",
        Rule::tibetan_by(&tibetan::TIBETAN, month, day, when),
    )
}

/// The Demonstration of Miracles, the first fifteen days of the first
/// month, Henning's "From 1st to 15th", on `$calendar` by `$when`.
macro_rules! demonstration_of_miracles {
    ($make:ident, $when:expr) => {
        [
            $make(MIRACLES, TibetanMonth::First, 1, $when),
            $make(MIRACLES, TibetanMonth::First, 2, $when),
            $make(MIRACLES, TibetanMonth::First, 3, $when),
            $make(MIRACLES, TibetanMonth::First, 4, $when),
            $make(MIRACLES, TibetanMonth::First, 5, $when),
            $make(MIRACLES, TibetanMonth::First, 6, $when),
            $make(MIRACLES, TibetanMonth::First, 7, $when),
            $make(MIRACLES, TibetanMonth::First, 8, $when),
            $make(MIRACLES, TibetanMonth::First, 9, $when),
            $make(MIRACLES, TibetanMonth::First, 10, $when),
            $make(MIRACLES, TibetanMonth::First, 11, $when),
            $make(MIRACLES, TibetanMonth::First, 12, $when),
            $make(MIRACLES, TibetanMonth::First, 13, $when),
            $make(MIRACLES, TibetanMonth::First, 14, $when),
            $make(MIRACLES, TibetanMonth::First, 15, $when),
        ]
    };
}

/// The fifteen days of the Demonstration of Miracles, Chötrul Düchen the
/// last of them.
const MIRACLES: &str = "Demonstration of Miracles";

/// The *düchen* and the New Year on the Phugpa calendar, each kept by
/// `$when` where its number is skipped or repeated, and a gap in a year
/// that doubles its month.
macro_rules! buddhist_tibetan_rules {
    ($when:expr) => {
        joined::<{ 1 + 15 + 7 }>(&[
            &[duchen("Losar", TibetanMonth::First, 1, $when)],
            &demonstration_of_miracles!(duchen, $when),
            &[
                duchen(
                    "Revelation of the Kalacakra Tantra",
                    TibetanMonth::Unrepeated(3),
                    15,
                    $when,
                ),
                duchen("Birth of the Buddha", TibetanMonth::Unrepeated(4), 7, $when),
                duchen("Saga Dawa Düchen", TibetanMonth::Unrepeated(4), 15, $when),
                duchen(
                    "Universal Prayer Day",
                    TibetanMonth::Unrepeated(5),
                    15,
                    $when,
                ),
                duchen("Chökhor Düchen", TibetanMonth::Unrepeated(6), 4, $when),
                duchen(
                    "The Buddha's entry into the womb of his mother",
                    TibetanMonth::Unrepeated(6),
                    15,
                    $when,
                ),
                duchen("Lhabab Düchen", TibetanMonth::Unrepeated(9), 22, $when),
            ],
        ])
    };
}

static BUDDHIST_TIBETAN_RULES: [HolidayRule; 23] =
    buddhist_tibetan_rules!(TibetanDayRule::Unsettled);
static BUDDHIST_TIBETAN_BERZIN_RULES: [HolidayRule; 23] =
    buddhist_tibetan_rules!(TibetanDayRule::Berzin);

/// The sources the Phugpa tables share.
const BUDDHIST_TIBETAN_SOURCES: &str = "Tibetan Nuns Project, \"Important Tibetan Buddhist \
     Holidays in 2024\" (tnp.org, `tnp-losar`), retrieved 2026-09-26, for Losar, Saga Dawa \
     Düchen, the Universal Prayer Day, Chökhor Düchen and Lhabab Düchen, their lunar dates and \
     their 2024 dates: Losar 10 February, Saga Dawa Düchen 23 May, the Universal Prayer Day \
     22 June, Chökhor Düchen 9 July, Lhabab Düchen 22 November; Edward Henning's computed \
     Phugpa almanacs (kalacakra.org, tdata/pl_*.txt, `kalacakra-org`), read 2026-09-29 for \
     1990 and 2024 to 2026, for the Demonstration of Miracles \"From 1st to 15th\" of the first \
     month, the Revelation of the Kalacakra Tantra on 3/15, the Birth of the Buddha on 4/7 and \
     the Buddha's entry into the womb of his mother on 6/15, with his English words; the lunar \
     days as `hc_calendars_lunar::tibetan::TIBETAN` computes them";

/// Tibetan Buddhism's *düchen* and the New Year on the Phugpa calendar,
/// `tibetan`: Losar on the first day of the year and the Demonstration of
/// Miracles on the first fifteen, Chötrul Düchen the last; the Revelation
/// of the Kalacakra Tantra on 3/15; the Birth of the Buddha on 4/7; Saga
/// Dawa Düchen on 4/15; the Universal Prayer Day on 5/15; Chökhor Düchen,
/// the first teaching, on 6/4; the Buddha's entry into the womb of his
/// mother on 6/15; and Lhabab Düchen, the descent from heaven, on 9/22.
///
/// A holiday is on the calendar day that bears its number. Where the
/// calendar skips that number, or repeats it, the day is reported as a gap
/// rather than guessed, as for Mongolia's and Bhutan's lunar holidays
/// ([`TibetanDayRule::Unsettled`]): which neighbouring day is kept is not
/// settled, and the two rules that answer it are the tables
/// [`BUDDHIST_TIBETAN_BERZIN`] and [`BUDDHIST_TIBETAN_HENNING`]. So is a
/// year that repeats the month: the list keeps Chökhor Düchen of 2024 on
/// 9 July, the first of two fourth days of the leap month 6, where Janson's
/// rule would keep it in the regular month, on 8 August, so a year in which
/// the month is doubled is a gap too ([`TibetanMonth::Unrepeated`];
/// `docs/systems/tibetan-calendar-holidays.md`). The days are those of the
/// Tibetan Nuns Project's 2024 list, which names no almanac, and the four
/// Henning's almanacs mark that the list does not.
pub static BUDDHIST_TIBETAN: RuleSet = RuleSet {
    code: "buddhist-tibetan",
    english_name: "Buddhism (Tibetan)",
    rules: &BUDDHIST_TIBETAN_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: BUDDHIST_TIBETAN_SOURCES,
    subdivisions: Subdivisions::Undivided,
};

/// The days of [`BUDDHIST_TIBETAN`], kept where a number is skipped or
/// repeated by the general rule Janson reports from Berzin
/// ([`TibetanDayRule::Berzin`]): a skipped date on the day before, a
/// repeated one on the first of its two days, which is where the Tibetan
/// Nuns Project kept Chökhor Düchen of 2024, on 9 July. A year that repeats
/// the month is still a gap, the rule saying nothing of months.
pub static BUDDHIST_TIBETAN_BERZIN: RuleSet = RuleSet {
    code: "buddhist-tibetan-berzin",
    english_name: "Buddhism (Tibetan, Berzin's rule)",
    rules: &BUDDHIST_TIBETAN_BERZIN_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "As `buddhist-tibetan`; Svante Janson, \"Tibetan calendar mathematics\" \
              (arXiv:1401.6285, `janson2014`), §11, for Berzin's rule, which Janson has \
              \"not checked against published calendars\"; the day as \
              `hc_calendars_regional::tibetan_almanac::berzin_day` gives it",
    subdivisions: Subdivisions::Undivided,
};

/// A festival Henning's almanacs mark, in the regular month of its number
/// and in the leap month of that number too, on `tibetan-lochen`.
const fn henning_festival(name: &'static str, month: u8, day: u8) -> [HolidayRule; 2] {
    [
        henning_day(name, TibetanMonth::Regular(month), day),
        henning_day(name, TibetanMonth::Leap(month), day),
    ]
}

/// A day of Henning's almanacs, as they mark it.
const fn henning_day(name: &'static str, month: TibetanMonth, day: u8) -> HolidayRule {
    feast(
        name,
        "",
        Rule::tibetan_by(
            &tibetan::TIBETAN_LOCHEN,
            month,
            day,
            TibetanDayRule::HenningAlmanac,
        ),
    )
}

/// [`henning_day`], with the unused parameter [`demonstration_of_miracles`]
/// passes.
const fn henning_first_month(
    name: &'static str,
    month: TibetanMonth,
    day: u8,
    _when: TibetanDayRule,
) -> HolidayRule {
    henning_day(name, month, day)
}

static BUDDHIST_TIBETAN_HENNING_RULES: [HolidayRule; 15 + 12] = joined(&[
    &demonstration_of_miracles!(henning_first_month, TibetanDayRule::HenningAlmanac),
    &henning_festival("Revelation of the Kalacakra Tantra", 3, 15),
    &henning_festival("Birth of the Buddha", 4, 7),
    &henning_festival("Enlightenment and Parinirvana of the Buddha", 4, 15),
    &henning_festival("Turning of the Wheel of the Dharma", 6, 4),
    &henning_festival("The Buddha's entry into the womb of his mother", 6, 15),
    &henning_festival("Descent of the Buddha from the realm of the gods", 9, 22),
]);

/// The seven festivals Edward Henning's computed Phugpa almanacs mark, with
/// his English words, on the calendar of those almanacs, `tibetan-lochen`
/// (the Phugpa under Minling Lochen's anomaly), as the almanacs place them
/// ([`TibetanDayRule::HenningAlmanac`]): on the second of two days of a
/// repeated number, on no day when the number is skipped — his Birth of the
/// Buddha is unmarked in 1966, 1975 and 1990 — and, in a year that repeats
/// the month, in both months: the almanac for 2024 marks the Turning of the
/// Wheel on 10 July, in the leap month 6, and on 8 August, in the regular
/// one. The first month's fifteen days of the Demonstration of Miracles
/// are his "From 1st to 15th" on its first day. The almanacs state no rule;
/// these are the library's reading of them.
pub static BUDDHIST_TIBETAN_HENNING: RuleSet = RuleSet {
    code: "buddhist-tibetan-henning",
    english_name: "Buddhism (Tibetan, as Henning's almanacs mark it)",
    rules: &BUDDHIST_TIBETAN_HENNING_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "Edward Henning's computed Phugpa almanacs (kalacakra.org, tdata/pl_*.txt, \
              `kalacakra-org`), read 2026-09-29 for 1990 and 2024 to 2026: every festival \
              and its English words, the two months of 2024 and the unmarked Birth of the \
              Buddha of 1990; the days as `tibetan-lochen` and \
              `hc_calendars_regional::tibetan_almanac::henning_almanac_day` give them",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// Uposatha, as Thailand keeps it
// ─────────────────────────────────────────────────────────────────────────

/// The days of `year` whose Thai lunar day `pick` accepts, given the day
/// and the length of its month.
fn thai_lunar_days(year: i64, pick: fn(u8, u8) -> bool) -> Days {
    let mut out = Days::new();
    let (Ok(first), Ok(last)) = (
        gregorian::to_fixed(year, 1, 1),
        gregorian::to_fixed(year, 12, 31),
    ) else {
        return out;
    };
    for day in first.0..=last.0 {
        let Ok(date) = hc_calendars_regional::thai_lunar::from_fixed(Rd(day)) else {
            continue;
        };
        let Ok(length) = hc_calendars_regional::thai_lunar::month_length(date.year, date.month)
        else {
            continue;
        };
        if pick(date.day, length) {
            out.push(Rd(day));
        }
    }
    out
}

/// ขึ้น 8 ค่ำ.
fn uposatha_waxing_eighth(year: i64) -> Days {
    thai_lunar_days(year, |day, _| day == 8)
}

/// ขึ้น 15 ค่ำ, the full moon.
fn uposatha_full_moon(year: i64) -> Days {
    thai_lunar_days(year, |day, _| day == 15)
}

/// แรม 8 ค่ำ, the twenty-third day of the month.
fn uposatha_waning_eighth(year: i64) -> Days {
    thai_lunar_days(year, |day, _| day == 23)
}

/// The last day of the month: แรม 15 ค่ำ, or แรม 14 ค่ำ in a month of 29
/// days, a เดือนขาด.
fn uposatha_new_moon(year: i64) -> Days {
    thai_lunar_days(year, |day, length| day == length)
}

/// A Thai uposatha day, over the years `thai-lunar` answers for.
const fn wan_phra(
    name: &'static str,
    local: &'static str,
    function: fn(i64) -> Days,
) -> HolidayRule {
    feast(
        name,
        local,
        Rule::Tabulated {
            function,
            first_year: hc_calendars_regional::thai_lunar::FIRST_GREGORIAN_YEAR,
            last_year: hc_calendars_regional::thai_lunar::LAST_GREGORIAN_YEAR,
        },
    )
}

static BUDDHIST_UPOSATHA_THAI_RULES: &[HolidayRule] = &[
    wan_phra(
        "Uposatha (waxing half moon)",
        "วันพระ ขึ้น 8 ค่ำ",
        uposatha_waxing_eighth,
    ),
    wan_phra("Uposatha (full moon)", "วันพระ ขึ้น 15 ค่ำ", uposatha_full_moon),
    wan_phra(
        "Uposatha (waning half moon)",
        "วันพระ แรม 8 ค่ำ",
        uposatha_waning_eighth,
    ),
    wan_phra(
        "Uposatha (new moon)",
        "วันพระ แรม 14 หรือ 15 ค่ำ",
        uposatha_new_moon,
    ),
];

/// The Theravāda *uposatha* days as Thailand keeps them, วันพระ, on the
/// Thai lunar calendar, `thai-lunar`: ขึ้น 8 ค่ำ, ขึ้น 15 ค่ำ, แรม 8 ค่ำ,
/// and the last day of the month, แรม 15 ค่ำ, or แรม 14 ค่ำ in a month of
/// 29 days, a เดือนขาด.
///
/// Which months are short is the calendar's: the odd months have 29 days
/// and the even 30, month 7 has 30 in an adhikavāra year, and an
/// adhikamāsa year repeats month 8, each copy with its own days
/// (`docs/systems/thai-lunar.md`). The days are exact for 1992–2027, the
/// years whose types Thailand published, and reported gaps outside. The
/// Dhammayuttika Nikāya may keep slightly different days, as Wikipedia's
/// "Uposatha" notes, and no reckoning of its own is carried; the Burmese,
/// Sri Lankan and Mahāyāna sets are other tables, not carried yet.
pub static BUDDHIST_UPOSATHA_THAI: RuleSet = RuleSet {
    code: "buddhist-uposatha-thai",
    english_name: "Buddhism (Thai uposatha days)",
    rules: BUDDHIST_UPOSATHA_THAI_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "Wikipedia (th), \"วันพระ\", for \"วันขึ้น 8 ค่ำ, วันขึ้น 15 ค่ำ (วันเพ็ญ), วันแรม \
              8 ค่ำ และวันแรม 15 ค่ำ (หากเดือนใดเป็นเดือนขาด ถือเอาวันแรม 14 ค่ำ)\"; Wikipedia, \
              \"Uposatha\", for the four days and the orders' differences (both \
              secondary); Thai PBS, \"ปฏิทินวันพระ 2568\" (thaipbs.or.th, \
              `thaipbs-wanphra-2568`), which credits กรมการศาสนา, for the days of 2025 \
              as checks; all retrieved 2026-09-26; the month lengths from the year \
              types of `thai-lunar` (docs/systems/thai-lunar.md)",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// Plough Monday, Plough Sunday and Distaff Day
// ─────────────────────────────────────────────────────────────────────────

static PLOUGH_DAYS_RULES: &[HolidayRule] = &read_all(
    1753,
    [
        HolidayRule::observance("Distaff Day", "St Distaff's Day", Rule::gregorian(1, 7)),
        // "The Sunday between 7 January and 13 January".
        feast(
            "Plough Sunday",
            "",
            Rule::WeekdayOnOrAfter {
                month: 1,
                day: 7,
                weekday: Weekday::Sunday,
            },
        ),
        // "The first Monday after Twelfth-day": after 6 January, so 13 January
        // when the 6th is itself a Monday, as in 2025.
        HolidayRule::observance(
            "Plough Monday",
            "",
            Rule::WeekdayOnOrAfter {
                month: 1,
                day: 7,
                weekday: Weekday::Monday,
            },
        ),
    ],
);

/// The English folk days that end Christmas: Distaff Day, St Distaff's
/// Day, on 7 January, "the day after Epiphany", when the women went back
/// to their spinning; Plough Monday, "the first Monday after Twelfth-day",
/// when the men went back to the plough; and Plough Sunday, "the Sunday
/// between 7 January and 13 January", on which a plough is blessed.
///
/// A Monday 6 January is not "after Twelfth-day", so Plough Monday is then
/// 13 January, the date it was kept on in 2025. When 6 January is a
/// Sunday, Plough Sunday is the 13th and Plough Monday the 7th, six days
/// before it: the Plough Monday article's "the day before Plough Monday",
/// said of the twentieth-century revival, is true only in the other six
/// years, and the Plough Sunday article's rule, which states a date, is
/// the one carried. The regional first- or second-Monday variants are not
/// carried. All three are Gregorian dates; the accounts are of England
/// after 1752.
pub static PLOUGH_DAYS: RuleSet = RuleSet {
    code: "plough-days",
    english_name: "Plough Monday, Plough Sunday and Distaff Day",
    rules: PLOUGH_DAYS_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 27),
    sources: "William Hone, The Every-Day Book, vol. 1 (London: Hunt and Clarke, 1826; \
              read in William Tegg and Co.'s reissue as Project Gutenberg's eBook 53275 \
              transcribes it, and in the transcription at hymnsandcarolsofchristmas.com; \
              `hone1826`), cols. 71-72, for \"The first Monday after Twelfth-day\", \
              and cols. 61-62, for \"St. Distaff's day, or the morrow after \
              Twelfth-day\"; Wikipedia, \"Plough Monday\", \"Plough Sunday\" and \
              \"Distaff Day\" (`wikipedia-plough-monday`, `wikipedia-plough-sunday`, \
              `wikipedia-distaff-day`, secondary), for the Sunday between 7 and 13 \
              January, Distaff Day on 7 January and the regional variants, retrieved \
              2026-09-26; Liz Gwedhan, \"13th January: Plough Monday\", 12 January \
              2025, https://lizgwedhan.substack.com/p/13th-january-plough-monday, \
              retrieved 2026-09-27, as the dated check of a Monday Twelfth-day",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// Chaharshanbe Suri
// ─────────────────────────────────────────────────────────────────────────

/// Nowruz, 1 Farvardin.
static NOWRUZ: Rule = Rule::in_calendar(CalendarSystem::SOLAR_HIJRI, 1, 1);

/// The last day of the Solar Hijri year, the day before Nowruz.
static LAST_DAY_OF_THE_YEAR: Rule = Rule::Offset {
    base: &NOWRUZ,
    days: -1,
};

/// From the last day of the year to the Tuesday before the last Wednesday
/// on or before it: a year that ends on a Tuesday has its last Wednesday a
/// week before its last day.
const TO_EVE_OF_LAST_WEDNESDAY: &[(Weekday, i16)] = &[
    (Weekday::Wednesday, -1),
    (Weekday::Thursday, -2),
    (Weekday::Friday, -3),
    (Weekday::Saturday, -4),
    (Weekday::Sunday, -5),
    (Weekday::Monday, -6),
    (Weekday::Tuesday, -7),
];

static CHAHARSHANBE_SURI_RULES: &[HolidayRule] = &[HolidayRule::observance(
    "Chaharshanbe Suri",
    "چهارشنبه‌سوری",
    Rule::moved_by_weekday(&LAST_DAY_OF_THE_YEAR, TO_EVE_OF_LAST_WEDNESDAY),
)];

/// Chaharshanbe Suri, the Iranian fire festival: "شبِ آخرین چهارشنبهٔ سال
/// (از غروب سه‌شنبه)", the night of the last Wednesday of the year, from
/// Tuesday's sunset, on the Solar Hijri calendar, `persian`.
///
/// The date given is the Tuesday whose evening it is. The Wednesday is the
/// last *of the year*, so when Nowruz is itself a Wednesday — the year
/// ending on a Tuesday — the festival is on the Tuesday a week before the
/// last day, as it was on 12 March 2024, and not on the eve of Nowruz.
pub static CHAHARSHANBE_SURI: RuleSet = RuleSet {
    code: "chaharshanbe-suri",
    english_name: "Chaharshanbe Suri",
    rules: CHAHARSHANBE_SURI_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 27),
    sources: "Wikipedia (fa), \"چهارشنبه‌سوری\", for the night of the last Wednesday of \
              the year from Tuesday's sunset and 28 Esfand 1404, 17 March 2026; Wikipedia, \
              \"Chaharshanbe Suri\", for 18 March 2025, 17 March 2026 and 16 March 2027 \
              (both secondary), retrieved 2026-09-26; an Eventbrite listing of a Chahar \
              Shanbe Suri on Tuesday 12 March 2024 at Richmond Hill, Ontario, the year \
              Nowruz was a Wednesday, \
              https://www.eventbrite.com/e/chahar-shanbe-suri-tickets-853218709127, \
              retrieved 2026-09-27, as a weak check",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// The Iranian festivals and the Zoroastrians of Iran
// ─────────────────────────────────────────────────────────────────────────

/// A festival on a day of a month of the Solar Hijri calendar, `persian`.
const fn iranian(name: &'static str, local: &'static str, month: u8, day: u8) -> HolidayRule {
    HolidayRule::observance(
        name,
        local,
        Rule::in_calendar(CalendarSystem::SOLAR_HIJRI, month, day),
    )
}

static IRANIAN_FESTIVALS_RULES: &[HolidayRule] = &[
    iranian("Tirgan", "جشن تیرگان", 4, 13),
    iranian("Mehregan", "جشن مهرگان", 7, 16),
    // The night between 30 Azar and 1 Dey, from the evening of the 30th.
    iranian("Yalda Night", "شب یلدا", 9, 30),
    // From the evening of 10 Bahman.
    iranian("Sadeh", "جشن سده", 11, 10),
];

/// The Iranian festivals on the civil calendar, the Solar Hijri, `persian`:
/// Tirgan on 13 Tir and Mehregan on 16 Mehr, the name days of their months
/// on the civil months; Yalda, "the night between the last day of the
/// ninth month (Azar) and the first day of the tenth month (Dey)", the
/// longest night; and Sadeh, "در شامگاه دهمین روز … از بهمن ماه", on the
/// evening of 10 Bahman, fifty days before Nowruz.
///
/// Yalda and Sadeh are nights, and the date given is the day whose evening
/// begins them — 30 Azar and 10 Bahman — as Chaharshanbe Suri's is.
/// Tirgan and Mehregan are also the Zoroastrians' name-day feasts, which
/// the Zoroastrians of Iran keep on their own reckoning of thirty-day
/// months, on 10 Tir and 10 Mehr of the civil calendar: that is the table
/// [`ZOROASTRIAN_IRANIAN`], a convention of its own (§5).
pub static IRANIAN_FESTIVALS: RuleSet = RuleSet {
    code: "iranian-festivals",
    english_name: "Iranian festivals",
    rules: IRANIAN_FESTIVALS_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "Wikipedia, \"Tirgan\" (Tir 13), \"Mehregan\" (8 October), \"Yaldā Night\" \
              (the night between the last day of Azar and 1 Dey, 21 December or 20 in a \
              leap year) and \"Sadeh\" (10 Bahman, 29 to 31 January, 50 days before \
              Nowruz); Wikipedia (fa), \"جشن سده\", for the evening of 10 Bahman, and \
              \"جشن‌های زرتشتی\", for Mehregan on 16 Mehr of the Iranian calendar; all \
              secondary, retrieved 2026-09-29",
    subdivisions: Subdivisions::Undivided,
};

/// A feast of the Zoroastrians of Iran on day `day` of month `month` of
/// their year of twelve thirty-day months, which begins with the civil
/// Nowruz: the `(month − 1) × 30 + day`-th day of the Solar Hijri year.
const fn bastani(name: &'static str, local: &'static str, month: u8, day: u8) -> HolidayRule {
    feast(
        name,
        local,
        Rule::Offset {
            base: &NOWRUZ,
            days: (month as i16 - 1) * 30 + day as i16 - 1,
        },
    )
}

static ZOROASTRIAN_IRANIAN_RULES: &[HolidayRule] = &[
    bastani("Farvardingan", "فروردینگان", 1, 19),
    bastani("Ardibeheshtgan", "اردیبهشتگان", 2, 3),
    bastani("Khordadgan", "خردادگان", 3, 6),
    bastani("Tirgan", "تیرگان", 4, 13),
    bastani("Amordadgan", "امردادگان", 5, 7),
    bastani("Shahrivargan", "شهریورگان", 6, 4),
    bastani("Mehregan", "مهرگان", 7, 16),
    bastani("Abangan", "آبانگان", 8, 10),
    bastani("Azargan", "آذرگان", 9, 9),
    bastani("Digan", "دیگان", 10, 1),
    bastani("Digan", "دیگان", 10, 8),
    bastani("Digan", "دیگان", 10, 15),
    bastani("Digan", "دیگان", 10, 23),
    bastani("Bahmangan", "بهمنگان", 11, 2),
    bastani("Esfandgan", "اسفندگان", 12, 5),
];

/// The twelve name-day feasts of the Zoroastrians of Iran, on the day whose
/// name is its month's, in their reckoning: twelve months of thirty days
/// from the civil Nowruz, with the five Gatha days at the end, so that
/// the feasts fall on other days of the civil months, whose first six have
/// 31 days. Mehregan, 16 Mehr of the Zoroastrian year, is 10 Mehr of the
/// civil calendar; Tirgan, 13 Tir, is 10 Tir; and the four days of the
/// Creator in Dey are 25 Azar and 2, 9 and 17 Dey — each as the Persian
/// Wikipedia's list gives the two dates side by side.
///
/// It is the Parsi Fasli schedule's name-day feasts on another year: the
/// Fasli begins on 21 March of every Gregorian year, this one on the civil
/// Nowruz, the day of the equinox at Tehran's noon. The Gahambars, Muktad
/// and the Gatha days, which the source read does not date on this
/// reckoning, are not carried.
pub static ZOROASTRIAN_IRANIAN: RuleSet = RuleSet {
    code: "zoroastrian-iranian",
    english_name: "Zoroastrian (Iran)",
    rules: ZOROASTRIAN_IRANIAN_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "Wikipedia (fa), \"جشن‌های زرتشتی\" (`wikipedia-fa-zoroastrian-festivals`), \
              the twelve monthly feasts with their Zoroastrian and their present Iranian \
              dates, secondary, citing Hayedeh Ramezan Rostamabadi, Zoroastrians in Iran \
              (1384 SH) and Mobed Ardeshir Azargoshasb, The Religious Ceremonies and \
              Customs of the Zoroastrians (1372 SH), not read; retrieved 2026-09-29; the \
              civil Nowruz as `persian` computes it",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// Tenrikyo
// ─────────────────────────────────────────────────────────────────────────

/// A monthly service on the 26th of a month that has no Grand Service.
const fn monthly_service(month: u8) -> HolidayRule {
    feast("Monthly Service", "月次祭", Rule::gregorian(month, 26))
}

static TENRIKYO_RULES: &[HolidayRule] = &dated(
    [
        feast("New Year's Day Service", "元旦祭", Rule::gregorian(1, 1)),
        feast("Spring Grand Service", "春季大祭", Rule::gregorian(1, 26)),
        monthly_service(2),
        monthly_service(3),
        feast(
            "Spring Memorial Service",
            "春季霊祭",
            Rule::gregorian(3, 27),
        ),
        feast(
            "Oyasama Birth Celebration Service",
            "教祖誕生祭",
            Rule::gregorian(4, 18),
        ),
        monthly_service(4),
        monthly_service(5),
        monthly_service(6),
        monthly_service(7),
        monthly_service(8),
        monthly_service(9),
        feast(
            "Autumn Memorial Service",
            "秋季霊祭",
            Rule::gregorian(9, 27),
        ),
        feast("Autumn Grand Service", "秋季大祭", Rule::gregorian(10, 26)),
        monthly_service(11),
        monthly_service(12),
    ],
    None,
    Some(2017),
);

/// The services of Tenrikyo at its Church Headquarters in Tenri, as the
/// headquarters' own page lists them: the New Year's Day Service on
/// 1 January; the Spring Grand Service on 26 January, for the day in 1887
/// on which Oyasama, the foundress, withdrew from physical life; the
/// Oyasama Birth Celebration Service on 18 April; the Autumn Grand Service
/// on 26 October, for the founding of 1838; the Monthly Service on the 26th
/// of each of the other ten months; and the Spring and Autumn Memorial
/// Services on 27 March and 27 September.
///
/// The anniversaries were lunar dates — the 26th of the first month of
/// 1887, the 18th of the fourth month of 1798, the 26th of the tenth month
/// of 1838 — and the services are now kept on the same numbers of the
/// Gregorian months. When that began was not read, so the table is read
/// from 2017, the earliest of the services Tenrikyo Online reports (18 April
/// 2017), and every earlier year is a gap. The
/// Service for Germination, performed in April on a night the weather
/// decides, the *obiya-zutome*, performed as needed, and a church's own
/// monthly service, on a day each church sets, are not carried.
pub static TENRIKYO: RuleSet = RuleSet {
    code: "tenrikyo",
    english_name: "Tenrikyo",
    rules: TENRIKYO_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 27),
    sources: "Tenrikyo Church Headquarters, \"祭典\" (tenrikyo.or.jp/jpn/service/, \
              `tenrikyo-saiten`), for 元旦祭 on 1 January, 春季大祭 on 26 January, \
              教祖誕生祭 on 18 April, 秋季大祭 on 26 October, 月次祭 \"1月と10月を除く毎月26日\" \
              and 春季霊祭 on 3月27日 and 秋季霊祭 on 9月27日; Yoshikazu Fukaya, \"Tenrikyo \
              Services\" (Tenrikyo Overseas Department, tenrikyo.or.jp, \
              `tenrikyo-services`), for the English names and the lunar anniversaries; \
              Tenrikyo Online, \"Spring Grand Service 2025\", \"Autumn Grand Service \
              2020\" and \"Oyasama's 219th Birthday Joyfully Celebrated\" (2017) \
              (`tenrikyo-online-services`), for the services of 26 January 2025, \
              26 October 2020 and 18 April 2017 as checks; all retrieved 2026-09-27",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// Reingold and Dershowitz's weekday coincidences
// ─────────────────────────────────────────────────────────────────────────

/// The days of `year` that fall on `weekday` and satisfy `keep`.
fn weekdays_where(year: i64, weekday: Weekday, keep: impl Fn(Rd) -> bool) -> Days {
    let (Ok(first), Ok(last)) = (
        gregorian::to_fixed(year, 1, 1),
        gregorian::to_fixed(year, 12, 31),
    ) else {
        return Days::new();
    };
    let mut day = first;
    while Weekday::from_rd(day) != weekday {
        day = Rd(day.0 + 1);
    }
    let mut days = Days::new();
    while day <= last {
        if keep(day) {
            days.push(day);
        }
        day = Rd(day.0 + 7);
    }
    days
}

/// Friday the 13th: Reingold and Dershowitz's `unlucky-fridays`, the
/// Fridays of a Gregorian year that are the 13th of their month.
fn unlucky_fridays(year: i64) -> Days {
    weekdays_where(year, Weekday::Friday, |day| {
        gregorian::from_fixed(day).is_ok_and(|(_, _, date)| date == 13)
    })
}

/// Reingold and Dershowitz's `sacred-wednesdays`: the Wednesdays of a
/// Gregorian year that are day 8 of a month of
/// `hindu-lunar-surya-siddhanta`, the day whose sunrise carries the eighth
/// tithi, as their `hindu-lunar-from-fixed` numbers it.
fn sacred_wednesdays(year: i64) -> Days {
    let calendar = SiddhantaLunarCalendar::UJJAIN;
    weekdays_where(year, Weekday::Wednesday, |day| {
        calendar.from_fixed(day).is_ok_and(|date| date.day == 8)
    })
}

/// The first and last Gregorian years `hindu-lunar-surya-siddhanta`
/// converts whole: it runs from 13 January 3101 BCE (−3100) to 15 June
/// 6900, and the test `the_sacred_wednesdays_are_the_books` checks the
/// years against the calendar's own range.
const SACRED_WEDNESDAYS_YEARS: (i64, i64) = (-3_099, 6_899);

static UNLUCKY_FRIDAYS_RULES: &[HolidayRule] = &read_all(
    2000,
    [HolidayRule::observance(
        "Friday the 13th",
        "",
        Rule::Computed(unlucky_fridays),
    )],
);

/// Friday the 13th, as Reingold and Dershowitz compute it in their
/// `unlucky-fridays`: every Friday of a Gregorian year that is the 13th of
/// its month, one to three a year. A folk day that no body defines, carried
/// as the book's rule and nothing more; how the day came to be thought
/// unlucky is not this table's.
pub static UNLUCKY_FRIDAYS: RuleSet = RuleSet {
    code: "unlucky-fridays",
    english_name: "Friday the 13th",
    rules: UNLUCKY_FRIDAYS_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 27),
    sources: "Edward M. Reingold and Nachum Dershowitz, calendar-code2, `calendar.l` \
              (`reingold2018code`), `unlucky-fridays` and `unlucky-fridays-in-range`, \
              read 2026-09-27 at commit 9afc1f3277b839db1a70c2350d6c708ac83df78f, and \
              run for 2000–2030 as the check",
    subdivisions: Subdivisions::Undivided,
};

static SACRED_WEDNESDAYS_RULES: &[HolidayRule] = &read_all(
    2000,
    [HolidayRule::observance(
        "Wednesday on the eighth lunar day",
        "",
        Rule::Tabulated {
            function: sacred_wednesdays,
            first_year: SACRED_WEDNESDAYS_YEARS.0,
            last_year: SACRED_WEDNESDAYS_YEARS.1,
        },
    )],
);

/// Reingold and Dershowitz's `sacred-wednesdays`: the Wednesdays that are
/// day 8 of a Hindu lunar month, which is the eighth tithi of the bright
/// fortnight, śukla aṣṭamī, on the day whose sunrise carries it.
///
/// The book computes the days on its own Hindu lunar calendar, the
/// *Sūrya Siddhānta*'s Sun and Moon at Ujjain, and so does this table:
/// that calendar is `hindu-lunar-surya-siddhanta`, and the table gives all
/// 56 of the days the book's code gives for 2000–2030 and no others. The
/// same Wednesdays read on `hindu-lunar`, the true Sun and Moon at the
/// Central Station, differ on ten of those 60 days; that reading is in no
/// source and is not carried. No traditional name was found for the set:
/// the almanacs' Budh Ashtami, seen only in search summaries, falls on an
/// eighth tithi of either fortnight and is not this rule. The years are
/// those `hindu-lunar-surya-siddhanta` converts whole, and a year outside
/// them is a reported gap.
pub static SACRED_WEDNESDAYS: RuleSet = RuleSet {
    code: "sacred-wednesdays",
    english_name: "Wednesdays on the eighth lunar day",
    rules: SACRED_WEDNESDAYS_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 27),
    sources: "Edward M. Reingold and Nachum Dershowitz, calendar-code2, `calendar.l` \
              (`reingold2018code`), `sacred-wednesdays` and \
              `sacred-wednesdays-in-range`, read 2026-09-27 at commit \
              9afc1f3277b839db1a70c2350d6c708ac83df78f, and run for 2000–2030 as the \
              check; the day of the month read on `hindu-lunar-surya-siddhanta`, the \
              book's own calendar (docs/systems/hindu-calendars.md)",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// The Assyrian Church of the East
// ─────────────────────────────────────────────────────────────────────────

/// The Feast of the Cross, which the Church of the East keeps on
/// 13 September.
const HOLY_CROSS_DAY: u8 = 13;

/// The first Sunday of Elijah: seven weeks after the first Sunday of
/// Summer, unless that is on or after the Feast of the Cross, when the
/// sixth and seventh Sundays of Summer merge and Elijah begins a week
/// earlier, so that its first Sunday comes before the Cross.
fn first_sunday_of_elijah(year: i64) -> Option<Days> {
    let easter = crate::computus::gregorian_easter(year)?;
    let cross = gregorian::to_fixed(year, 9, HOLY_CROSS_DAY).ok()?;
    let ideal = Rd(easter.0 + 147);
    Some(Days::one(if ideal < cross {
        ideal
    } else {
        Rd(ideal.0 - 7)
    }))
}

/// The Monday, Tuesday and Wednesday of the Rogation of the Ninevites,
/// in the third week before the Great Fast.
const fn rogation_of_the_ninevites(offset: i16) -> HolidayRule {
    east(
        "Rogation of the Ninevites",
        "ܒܵܥܘܼܬ̣ܵܐ ܕܢܝ݂ܢܘܵܝܹ̈ܐ",
        Rule::easter(offset),
    )
}

/// A day of the Church of the East, from the year after the one in which
/// it took up the Gregorian calendar.
const fn east(name: &'static str, local: &'static str, rule: Rule) -> HolidayRule {
    feast(name, local, rule).years(Some(1965), None)
}

pub(crate) static CHURCH_OF_THE_EAST_RULES: &[HolidayRule] = &[
    east(
        "Epiphany (the season of Denha begins)",
        "ܥܹܐܕܵܐ ܕܕܸܢܚܵܐ",
        Rule::gregorian(1, 6),
    ),
    rogation_of_the_ninevites(-69),
    rogation_of_the_ninevites(-68),
    rogation_of_the_ninevites(-67),
    east(
        "First Sunday of the Great Fast",
        "ܐ ܕܨܘܡܐ",
        Rule::easter(-49),
    ),
    east("Palm Sunday", "ܥܹܐܕܵܐ ܕܐܘܿܫܲܥܢܹ̈ܐ", Rule::easter(PALM_SUNDAY)),
    east(
        "Easter (the season of the Resurrection begins)",
        "ܥܹܐܕܵܐ ܕܲܩܝܵܡܬܹܗ ܕܡܵܪܲܢ ܝܑܼܫܘܿܥ ܡܫܝܼܚܵܐ",
        Rule::easter(EASTER_SUNDAY),
    ),
    east("Ascension", "ܥܹܐܕܵܐ ܕܣܘܼܠܵܩܵܐ", Rule::easter(ASCENSION)),
    east(
        "Pentecost (the season of the Apostles begins)",
        "ܥܹܐܕܵܐ ܕܦܲܢܛܹܩܘܼܣܛܹ̈ܐ",
        Rule::easter(PENTECOST),
    ),
    east(
        "First Sunday of Summer (Nusardel)",
        "ܐ ܩܝܛܐ، ܥܹܐܕܵܐ ܕܢܘܼܣܲܪܕܹܐܝܠ",
        Rule::easter(98),
    ),
    east("Transfiguration", "ܥܹܐܕܵܐ ܕܓܸܠܝܵܢܵܐ", Rule::gregorian(8, 6)),
    east(
        "First Sunday of Elijah",
        "ܐ ܕܐܠܝܐ",
        Rule::Unsettled(first_sunday_of_elijah),
    ),
    east(
        "Feast of the Cross",
        "ܥܹܐܕܵܐ ܕܨܠܝ݂ܒ݂ܵܐ",
        Rule::gregorian(9, HOLY_CROSS_DAY),
    ),
    east(
        "First Sunday of the Dedication of the Church",
        "ܐ ܕܩܘܕܫ ܥܕܬܐ",
        Rule::WeekdayOnOrAfter {
            month: 10,
            day: 30,
            weekday: Weekday::Sunday,
        },
    ),
    east(
        "First Sunday of the Annunciation (Subara)",
        "ܐ ܕܣܘܒܪܐ",
        Rule::WeekdayOnOrAfter {
            month: 11,
            day: 27,
            weekday: Weekday::Sunday,
        },
    ),
    east(
        "Nativity of Our Lord",
        "ܥܹܐܕܵܐ ܕܝܲܠܕܹܗ ܕܡܵܪܲܢ ܝܑܼܫܘܿܥ ܡܫܝܼܚܵܐ",
        Rule::gregorian(12, 25),
    ),
];

/// The liturgical year of the Assyrian Church of the East, divided into
/// seasons of seven weeks, *shawu'e*, anchored to Easter on the Gregorian
/// computus and to three fixed feasts: the first Sunday of each season
/// whose start the Church's own statement fixes, with the feasts that open
/// or divide them.
///
/// The Annunciation (Subara) has the four Sundays before Christmas; the
/// Nativity follows, then Epiphany (Denha) from 6 January to the Great
/// Fast, the Great Fast the seven weeks before Easter, the Resurrection the
/// seven weeks to Pentecost, the Apostles the seven weeks after it, and
/// Summer seven more. Elijah follows, and its first Sunday must come
/// before the Feast of the Cross on 13 September, so in a year of a late
/// Easter Summer loses a Sunday. The Dedication of the Church has the four
/// Sundays before the Annunciation. The Rogation of the Ninevites is kept
/// on the Monday, Tuesday and Wednesday of the third week before the Great
/// Fast. See `docs/systems/church-of-the-east-year.md`.
///
/// The Fridays of commemoration and the saints' days follow the seasons,
/// from [`crate::east_syriac::CHURCH_OF_THE_EAST_COMMEMORATIONS`]: the
/// Fridays whose place the statement fixes, and the saints' days that keep
/// one date or one distance from Easter in all four years of the Church's
/// calendar read.
///
/// Not carried: the Sundays of Moses, between Elijah and the Dedication,
/// whose first Sunday the statement read does not fix; the order in which
/// the Sundays of Elijah and of the Cross are kept when the Cross falls
/// early in Elijah, which the Church's calendar of 2026–2029 shows varying
/// in a way the statement does not describe; the numbering of the Sundays
/// of Epiphany after the first, which the calendar fits to the weeks by
/// keeping two on one day, differently in two years of five Sundays each;
/// the Fridays of Epiphany between the first and the penultimate, which it
/// merges in the same way; the saints' days that move by another rule; and
/// the Ancient Church of the East, whose calendar was not read. The
/// Chaldean and Syro-Malabar years are `chaldean` and `syro-malabar`. The table begins in 1965, the year
/// after the Church's decision of 1964 to take up the Gregorian calendar,
/// the day that decision took effect not having been read.
pub static CHURCH_OF_THE_EAST: RuleSet = RuleSet {
    code: "church-of-the-east",
    english_name: "Assyrian Church of the East",
    rules: &crate::east_syriac::CHURCH_OF_THE_EAST_ALL_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 27),
    sources: "Rev. Tower Andrious, \"The Ecclesiastical Liturgical Year for the Church \
              of the East\" (Assyrian Church of the East, Beth Kokheh, \
              bethkokheh.assyrianchurch.org, uploaded 2020-08; `andrious-liturgical-year`), \
              read in the Internet Archive's copy of 13 February 2026, the live file \
              returning an error, for the nine seasons and their lengths, the Rogation \
              of the Ninevites, the Feast of the Cross \"precisely on the 13th of \
              September\" and Elijah's first Sunday before it; the Assyrian Church of the \
              East Liturgical Calendar (calendar.assyrianchurch.org, its English and \
              Assyrian feeds, `ace-liturgical-calendar`), for every Sunday and feast of \
              2026–2029 as checks and the Syriac names; Wikipedia, \"Ancient Church of the \
              East\" (secondary), for the Gregorian calendar from 1964; all retrieved \
              2026-09-27",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────

// ─────────────────────────────────────────────────────────────────────────
// Jainism
// ─────────────────────────────────────────────────────────────────────────

/// A Jain day: religious, and approximate, since the sects' almanacs can
/// differ from the national one by a day.
const fn jain(name: &'static str, local: &'static str, rule: Rule) -> HolidayRule {
    feast(name, local, rule).approximate()
}

/// One of the days of a festival counted back from its last day.
const fn jain_day(
    name: &'static str,
    local: &'static str,
    last: &'static Rule,
    days_before: i16,
) -> HolidayRule {
    jain(
        name,
        local,
        Rule::Offset {
            base: last,
            days: -days_before,
        },
    )
}

static JAIN_RULES: &[HolidayRule] = &[
    jain("Mahavir Jayanti", "महावीर जयंती", MAHAVIR_JAYANTI),
    jain("Akshaya Tritiya", "अक्षय तृतीया", AKSHAYA_TRITIYA),
    // The Śvetāmbara Paryuṣaṇa: eight days ending with Saṃvatsarī.
    jain_day("Paryushana", "पर्युषण", &SAMVATSARI, 7),
    jain_day("Paryushana", "पर्युषण", &SAMVATSARI, 6),
    jain_day("Paryushana", "पर्युषण", &SAMVATSARI, 5),
    jain_day("Paryushana", "पर्युषण", &SAMVATSARI, 4),
    jain_day("Paryushana", "पर्युषण", &SAMVATSARI, 3),
    jain_day("Paryushana", "पर्युषण", &SAMVATSARI, 2),
    jain_day("Paryushana", "पर्युषण", &SAMVATSARI, 1),
    jain_day("Paryushana", "पर्युषण", &SAMVATSARI, 0),
    jain(
        "Samvatsari",
        "संवत्सरी",
        Rule::Offset {
            base: &SAMVATSARI,
            days: 0,
        },
    ),
    // The Digambara Daśa Lakṣaṇa: ten days from the day after, ending with
    // Ananta Caturdaśī.
    jain_day("Das Lakshana", "दशलक्षण", &ANANT_CHATURDASHI, 9),
    jain_day("Das Lakshana", "दशलक्षण", &ANANT_CHATURDASHI, 8),
    jain_day("Das Lakshana", "दशलक्षण", &ANANT_CHATURDASHI, 7),
    jain_day("Das Lakshana", "दशलक्षण", &ANANT_CHATURDASHI, 6),
    jain_day("Das Lakshana", "दशलक्षण", &ANANT_CHATURDASHI, 5),
    jain_day("Das Lakshana", "दशलक्षण", &ANANT_CHATURDASHI, 4),
    jain_day("Das Lakshana", "दशलक्षण", &ANANT_CHATURDASHI, 3),
    jain_day("Das Lakshana", "दशलक्षण", &ANANT_CHATURDASHI, 2),
    jain_day("Das Lakshana", "दशलक्षण", &ANANT_CHATURDASHI, 1),
    jain_day("Das Lakshana", "दशलक्षण", &ANANT_CHATURDASHI, 0),
    jain(
        "Anant Chaturdashi",
        "अनंत चतुर्दशी",
        Rule::Offset {
            base: &ANANT_CHATURDASHI,
            days: 0,
        },
    ),
    jain("Diwali", "दीपावली", DIWALI),
];

/// Jainism.
///
/// The two sects' great festival as one table: the Śvetāmbara Paryuṣaṇa,
/// eight days ending with Saṃvatsarī on Bhādrapada śukla 4, and the
/// Digambara Daśa Lakṣaṇa, ten days beginning the day after and ending
/// with Ananta Caturdaśī on śukla 14 — each counted back from its last
/// day, which is the day the sect fixes. Mahāvīra Jayantī and Akṣaya
/// Tṛtīyā on the rules the Hindu table shares, and Diwali, which Jains keep
/// as the day of Mahāvīra's nirvāṇa.
///
/// **Every entry is approximate.** The rules read the national almanac's
/// sunrise; a Jain almanac can put a tithi a day away in some years, and
/// the source says so. Kṣamāvaṇī, the Digambara day of forgiveness, is
/// not carried: its source states it on Āśvina kṛṣṇa 1 and dates it on
/// Ananta Caturdaśī in two of its three example years, and the table does
/// not choose.
pub static JAIN: RuleSet = RuleSet {
    code: "jain",
    english_name: "Jainism",
    rules: JAIN_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "Wikipedia, \"Paryushana\", retrieved 2026-09-22, for the eight \
              Śvetāmbara days ending with Saṃvatsarī on Bhadrapada śukla 4, the \
              ten Digambara days that start right after, and the sects' \
              computational differences; Wikipedia, \"Kshamavani\", retrieved \
              2026-09-22, for the day not carried; Wikipedia, \"Diwali\", \
              retrieved 2026-09-22, for the Jain observance; the Rashtriya \
              Panchang for Mahāvīra Jayantī and Akṣaya Tṛtīyā, as `hindu` \
              states. All secondary; a Śvetāmbara or Digambara pañcāṅga, which would be the primary, was not read",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// Shinto
// ─────────────────────────────────────────────────────────────────────────

/// 立春 at the Japanese meridian, the day 節分 precedes.
static RISSHUN: Rule = Rule::SolarTerm {
    term: SolarTerm::BEGINNING_OF_SPRING,
    meridian: Meridian::JAPAN,
};

static SHINTO_RULES: &[HolidayRule] = &read_all(
    1873,
    [
        HolidayRule::observance("Hatsumōde", "初詣", Rule::gregorian(1, 1)),
        HolidayRule::observance(
            "Setsubun",
            "節分",
            Rule::Offset {
                base: &RISSHUN,
                days: -1,
            },
        ),
        feast("Nagoshi no Ōharae", "夏越の大祓", Rule::gregorian(6, 30)),
        HolidayRule::observance("Shichi-Go-San", "七五三", Rule::gregorian(11, 15)),
        feast(
            "Toshikoshi no Ōharae",
            "年越の大祓",
            Rule::gregorian(12, 31),
        ),
    ],
);

/// Shinto, as the year is kept at a shrine and at home: the New Year
/// visit, 節分 on the eve of 立春, the two 大祓 purifications at the half
/// and the end of the year, and 七五三 on 15 November, the date it has had
/// since the Meiji calendar reform.
///
/// 節分 is the day before 立春 at the Japanese meridian, which is why it
/// was 4 February in the leap years to 1984, 3 February from 1985 to 2020,
/// and 2 February in the years after a leap year from 2021 — the source
/// states the pattern and the rule reproduces it. A shrine's own festival
/// days, and the many observances kept on a weekend near the date, are
/// not carried; the imperial rites have a table of their own,
/// [`KYUCHU_SAISHI`].
pub static SHINTO: RuleSet = RuleSet {
    code: "shinto",
    english_name: "Shinto",
    rules: SHINTO_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "Wikipedia (ja), \"節分\", retrieved 2026-09-22, for the rule — the \
              day before 立春, the Sun at longitude 315° — and its dates by \
              period; \"七五三\", for 15 November since the Meiji reform; \
              \"初詣\", for the New Year visit; \"宮中祭祀\" (`wikipedia-ja-kyuchu-saishi`), for 大祓 on \
              30 June and 31 December. All secondary: no shrine's or the Association of Shinto Shrines' calendar was read",
    subdivisions: Subdivisions::Undivided,
};

/// A rite of the imperial court on a fixed Gregorian date.
const fn rite(name: &'static str, local: &'static str, month: u8, day: u8) -> HolidayRule {
    feast(name, local, Rule::gregorian(month, day))
}

/// A rite on an equinox day at the Japanese meridian.
const fn equinox_rite(name: &'static str, local: &'static str, term: SolarTerm) -> HolidayRule {
    feast(
        name,
        local,
        Rule::SolarTerm {
            term,
            meridian: Meridian::JAPAN,
        },
    )
}

static KYUCHU_SAISHI_RULES: &[HolidayRule] = &read_all(
    2020,
    [
        rite("Shihōhai", "四方拝", 1, 1),
        rite("Saitansai", "歳旦祭", 1, 1),
        rite("Genshisai", "元始祭", 1, 3),
        rite("Sōjihajime", "奏事始", 1, 4),
        rite("Shōwa Tennō-sai", "昭和天皇祭", 1, 7),
        rite("Kōmei Tennō reisai", "孝明天皇例祭", 1, 30),
        // 紀元節祭 until 1948; the 三殿御拝 since.
        rite("Sanden gohai", "三殿御拝", 2, 11).years(Some(1949), None),
        rite("Kinensai", "祈年祭", 2, 17),
        // The present Emperor's birthday, a Tenchōsai from 2020.
        rite("Tenchōsai", "天長祭", 2, 23).years(Some(2020), None),
        equinox_rite("Shunki Kōreisai", "春季皇霊祭", SolarTerm::SPRING_EQUINOX),
        equinox_rite("Shunki Shindensai", "春季神殿祭", SolarTerm::SPRING_EQUINOX),
        rite("Jinmu Tennō-sai", "神武天皇祭", 4, 3),
        rite("Kōreiden Mikagura", "皇霊殿御神楽", 4, 3),
        rite("Kōjun Kōgō reisai", "香淳皇后例祭", 6, 16),
        rite("Yoori", "節折", 6, 30),
        rite("Ōharai", "大祓", 6, 30),
        rite("Meiji Tennō reisai", "明治天皇例祭", 7, 30),
        equinox_rite("Shūki Kōreisai", "秋季皇霊祭", SolarTerm::AUTUMN_EQUINOX),
        equinox_rite("Shūki Shindensai", "秋季神殿祭", SolarTerm::AUTUMN_EQUINOX),
        rite("Kannamesai", "神嘗祭", 10, 17),
        rite("Niinamesai", "新嘗祭", 11, 23),
        rite("Taishō Tennō reisai", "大正天皇例祭", 12, 25),
        rite("Yoori", "節折", 12, 31),
        rite("Ōharai", "大祓", 12, 31),
        // The 旬祭 on the first, eleventh and twenty-first of every month.
        rite("Shunsai", "旬祭", 1, 1),
        rite("Shunsai", "旬祭", 1, 11),
        rite("Shunsai", "旬祭", 1, 21),
        rite("Shunsai", "旬祭", 2, 1),
        rite("Shunsai", "旬祭", 2, 11),
        rite("Shunsai", "旬祭", 2, 21),
        rite("Shunsai", "旬祭", 3, 1),
        rite("Shunsai", "旬祭", 3, 11),
        rite("Shunsai", "旬祭", 3, 21),
        rite("Shunsai", "旬祭", 4, 1),
        rite("Shunsai", "旬祭", 4, 11),
        rite("Shunsai", "旬祭", 4, 21),
        rite("Shunsai", "旬祭", 5, 1),
        rite("Shunsai", "旬祭", 5, 11),
        rite("Shunsai", "旬祭", 5, 21),
        rite("Shunsai", "旬祭", 6, 1),
        rite("Shunsai", "旬祭", 6, 11),
        rite("Shunsai", "旬祭", 6, 21),
        rite("Shunsai", "旬祭", 7, 1),
        rite("Shunsai", "旬祭", 7, 11),
        rite("Shunsai", "旬祭", 7, 21),
        rite("Shunsai", "旬祭", 8, 1),
        rite("Shunsai", "旬祭", 8, 11),
        rite("Shunsai", "旬祭", 8, 21),
        rite("Shunsai", "旬祭", 9, 1),
        rite("Shunsai", "旬祭", 9, 11),
        rite("Shunsai", "旬祭", 9, 21),
        rite("Shunsai", "旬祭", 10, 1),
        rite("Shunsai", "旬祭", 10, 11),
        rite("Shunsai", "旬祭", 10, 21),
        rite("Shunsai", "旬祭", 11, 1),
        rite("Shunsai", "旬祭", 11, 11),
        rite("Shunsai", "旬祭", 11, 21),
        rite("Shunsai", "旬祭", 12, 1),
        rite("Shunsai", "旬祭", 12, 11),
        rite("Shunsai", "旬祭", 12, 21),
    ],
);

/// The rites of the imperial court, 宮中祭祀, on the schedule the source
/// gives for the Reiwa era: the 大祭 and 小祭 of the year, the 旬祭 three
/// times a month, and the two equinox rites on 春分の日 and 秋分の日 at
/// the Japanese meridian.
///
/// The rites tied to a reign change with it. 天長祭 is the reigning
/// Emperor's birthday, 23 February since 2020, and is bounded so; the
/// 先帝祭 — 昭和天皇祭 — and the 例祭 of the three emperors before him and
/// of the late Empress are the present reign's, stated without a start.
/// 賢所御神楽, "mid-December", has no fixed date and is not carried; the
/// 式年祭 of set anniversaries are not either.
pub static KYUCHU_SAISHI: RuleSet = RuleSet {
    code: "kyuchu-saishi",
    english_name: "Imperial court rites (宮中祭祀)",
    rules: KYUCHU_SAISHI_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "宮内庁, \"主要祭儀一覧\" (kunaicho.go.jp/about/gokomu/kyuchu/saishi/saishi01.html, \
              `kunaicho-saishi`), \
              retrieved 2026-09-26, for the 大祭 and 小祭 with their dates; Wikipedia (ja), \
              \"宮中祭祀\" (`wikipedia-ja-kyuchu-saishi`), retrieved 2026-09-22, for the 旬祭 and the \
              三殿御拝 of 11 February, which \
              the Agency's list of the principal rites does not include, and \"天皇誕生日\", \
              retrieved 2026-09-22, for 23 February from 2020",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// Sikhism
// ─────────────────────────────────────────────────────────────────────────

/// A gurpurab on a fixed day of the Nanakshahi calendar.
const fn nanakshahi(name: &'static str, month: u8, day: u8) -> HolidayRule {
    feast(
        name,
        "",
        Rule::in_calendar(CalendarSystem::NANAKSHAHI, month, day),
    )
}

/// The observances of the Nanakshahi calendar of 2003, as its table gives
/// them, under the Sikh terms: *Parkash* for a Guru's birth, *Gurgaddi* for
/// his accession, *Joti Jot* for his passing, *Shaheedi* for a martyrdom.
static SIKH_RULES: &[HolidayRule] = &[
    nanakshahi("Nanakshahi New Year", 1, 1),
    nanakshahi("Gurgaddi of Guru Har Rai", 1, 1),
    nanakshahi("Joti Jot of Guru Hargobind", 1, 6),
    // The Khalsa's ordination, 1 Vaisakh; the calendar's fixed 14 April,
    // where the SGPC has kept 1 Vaisakh of the Bikrami solar calendar, 13
    // or 14 April, since 2010 ([`SIKH_SGPC`]).
    nanakshahi("Vaisakhi", 2, 1),
    nanakshahi("Joti Jot of Guru Angad", 2, 3),
    nanakshahi("Gurgaddi of Guru Amar Das", 2, 3),
    nanakshahi("Joti Jot of Guru Harkrishan", 2, 3),
    nanakshahi("Gurgaddi of Guru Tegh Bahadur", 2, 3),
    nanakshahi("Parkash of Guru Angad", 2, 5),
    nanakshahi("Parkash of Guru Tegh Bahadur", 2, 5),
    nanakshahi("Parkash of Guru Arjan", 2, 19),
    nanakshahi("Parkash of Guru Amar Das", 3, 9),
    nanakshahi("Gurgaddi of Guru Hargobind", 3, 28),
    nanakshahi("Shaheedi of Guru Arjan", 4, 2),
    nanakshahi("Parkash of Guru Hargobind", 4, 21),
    nanakshahi("Miri Piri Divas", 5, 6),
    nanakshahi("Parkash of Guru Harkrishan", 5, 8),
    nanakshahi("Completion of the Guru Granth Sahib", 6, 15),
    nanakshahi("First Parkash of the Guru Granth Sahib", 6, 17),
    nanakshahi("Joti Jot of Guru Amar Das", 7, 2),
    nanakshahi("Gurgaddi of Guru Ram Das", 7, 2),
    nanakshahi("Joti Jot of Guru Ram Das", 7, 2),
    nanakshahi("Gurgaddi of Guru Arjan", 7, 2),
    nanakshahi("Gurgaddi of Guru Angad", 7, 4),
    nanakshahi("Joti Jot of Guru Nanak", 7, 8),
    nanakshahi("Parkash of Guru Ram Das", 7, 25),
    nanakshahi("Joti Jot of Guru Har Rai", 8, 6),
    nanakshahi("Gurgaddi of Guru Harkrishan", 8, 6),
    nanakshahi("Gurgaddi of the Guru Granth Sahib", 8, 6),
    nanakshahi("Joti Jot of Guru Gobind Singh", 8, 7),
    nanakshahi("Gurgaddi of Guru Gobind Singh", 9, 11),
    nanakshahi("Shaheedi of Guru Tegh Bahadur", 9, 11),
    nanakshahi("Shaheedi of the Elder Sahibzadas", 10, 8),
    nanakshahi("Shaheedi of the Younger Sahibzadas", 10, 13),
    nanakshahi("Parkash of Guru Gobind Singh", 10, 23),
    nanakshahi("Parkash of Guru Har Rai", 11, 19),
    // The three the 2003 calendar left on the Bikrami lunar calendar, "as
    // a compromise": Hola Mohalla on Chet vadi 1, the day after Holi,
    // Bandi Chhor Divas on Diwali, and Guru Nanak's Parkash on the full
    // moon of Kārttika.
    feast("Hola Mohalla", "", HOLA_MOHALLA).approximate(),
    feast("Bandi Chhor Divas", "", DIWALI),
    feast("Parkash of Guru Nanak", "", GURU_NANAK_JAYANTI),
];

/// Sikhism, on the Nanakshahi calendar of 2003 — the *Mool* Nanakshahi —
/// and named for it, because the Sikh bodies do not keep one calendar.
///
/// The gurpurabs on the fixed dates that calendar gave them — Guru Gobind
/// Singh's Parkash on 23 Poh, 5 January; Guru Arjan's Shaheedi on 2 Harh,
/// 16 June; Guru Tegh Bahadur's on 11 Maghar, 24 November — and the three
/// observances it kept lunar: Hola Mohalla on Chet vadi 1 at sunrise
/// ([`crate::hindu::HOLA_MOHALLA`], fitted and so approximate), and Bandi
/// Chhor Divas and the Parkash of Guru Nanak on the same rules as Diwali
/// and Kartik Purnima in the Hindu table. The SGPC's own list of the
/// movable dates of 2010 to 2020 has all three on those rules' days, and
/// has Hola Mohalla a day after Holi in 2012, 2013 and 2016. The same
/// table's Hola Mohalla of 2003 to 2008 is the rule's too; its 11 March
/// 2009 is Holi itself, a day before Chet vadi 1 at sunrise, and is the one
/// year of 2003–2020 the rule misses, which the table's test states.
///
/// **This is the 2003 calendar.** Pal Singh Purewal's calendar was adopted
/// by the SGPC's general house and the Akal Takht in 2003. The SGPC's 2010
/// amendments returned the sangrands and four gurpurabs to Bikrami dates,
/// and by 2015 it had gone back to the Bikrami calendar under the
/// Nanakshahi name; the Pakistan Sikh Gurdwara Parbandhak Committee, the
/// American Gurdwara Parbandhak Committee and many other gurdwara
/// committees keep the 2003 version. The SGPC's own dates are the Bikrami
/// calendar's as its yearly jantri prints them, and those whose rule was
/// read are the second set, [`SIKH_SGPC`]. The 2017 resolution of the Mool
/// calendar's supporters to fix the three lunar days as well is not
/// carried either.
/// The Akal Takht's foundation day is omitted: the source's row gives
/// 18 Harh against 16 June, which is 2 Harh, and the table does not guess.
pub static SIKH_NANAKSHAHI_2003: RuleSet = RuleSet {
    code: "sikh-nanakshahi-2003",
    english_name: "Sikhism (Nanakshahi calendar of 2003)",
    rules: SIKH_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "Wikipedia, \"Nanakshahi calendar\", retrieved 2026-09-22: the table \
              of festivals and events of the 2003 version for every fixed \
              date, and its table of the movable dates 2003–2020 for the three \
              lunar observances (secondary; the SGPC's 2003 calendar itself was \
              not read); The Tribune (15 April 2019; 14 March 2015, Perneet Singh; \
              24 May 2015) and Asia Samachar (2 October 2022) for the adoption, the \
              2010 amendments and who keeps the 2003 version, retrieved 2026-09-26",
    subdivisions: Subdivisions::Undivided,
};

/// Bandi Chhor Divas as the SGPC kept it, the days read, 2010 to 2026.
///
/// 2010 to 2020 are the table of movable dates on Wikipedia, "Nanakshahi
/// calendar" (secondary, from Purewal's list, not read); 2021 and 2026
/// SikhNet's lists "as per SGPC Calendar" (`sikhnet-gurpurab-2021`,
/// `sikhnet-gurpurab-2026`); 2022 and 2023 dekho-ji's "Nanakshahi Calendar
/// SGPC" (`dekhoji-nanakshahi-sgpc`, secondary); 2024 PTC News,
/// 1 November 2024 (`ptcnews-bandi-chhor-2024`); 2025 The Tribune,
/// 22 October 2025 (`tribune-bandi-chhor-2025`), and Babushahi, "as
/// confirmed by" the SGPC (`babushahi-bandi-chhor-2025`, secondary: read
/// in a search engine's summary only).
#[rustfmt::skip]
static SGPC_BANDI_CHHOR: Listing = Listing::Dates(&[
    (2010, 11, 5), (2011, 10, 26), (2012, 11, 13), (2013, 11, 3), (2014, 10, 23),
    (2015, 11, 11), (2016, 10, 30), (2017, 10, 19), (2018, 11, 7), (2019, 10, 27),
    (2020, 11, 14), (2021, 11, 4), (2022, 10, 24), (2023, 11, 12), (2024, 11, 1),
    (2025, 10, 21), (2026, 11, 8),
]);

/// The observances the SGPC keeps on the Bikrami calendar, by a rule a
/// source read states or fits, or as the days it published.
static SIKH_SGPC_RULES: &[HolidayRule] = &[
    // Khalsa Sajna Divas, 1 Vaisakh of the Vikrami solar calendar, whose
    // month begins on the sunrise-to-sunrise day of its saṅkrānti.
    feast(
        "Vaisakhi",
        "",
        Rule::in_calendar(CalendarSystem::VIKRAMI, 1, 1),
    ),
    feast(
        "Parkash of Guru Gobind Singh",
        "",
        GURU_GOBIND_SINGH_PARKASH,
    ),
    feast("Hola Mohalla", "", HOLA_MOHALLA).approximate(),
    feast(
        "Bandi Chhor Divas",
        "",
        Rule::listed(SGPC_BANDI_CHHOR.every(), 2010, 2026),
    ),
    feast("Parkash of Guru Nanak", "", GURU_NANAK_JAYANTI),
];

/// Sikhism as the Shiromani Gurdwara Parbandhak Committee keeps it since
/// 2010, on the Bikrami calendar: the observances whose day a source read
/// states or reproduces, and no others.
///
/// - **Vaisakhi**, Khalsa Sajna Divas, is 1 Vaisakh of the Vikrami solar
///   calendar of Punjab ([`hc_calendars_indic::hindu_solar::VIKRAMI`]),
///   whose month begins on the sunrise-to-sunrise day its saṅkrānti falls
///   in, as the *Rashtriya Panchang*'s Punjab column has it. A Meṣa
///   saṅkrānti between midnight and sunrise therefore makes Vaisakhi the
///   day before the saṅkrānti's civil day: 13 April in 2017, 2021 and
///   2025, the days the SGPC kept. The same calendar gives every sangrand
///   of SikhNet's SGPC lists of 2021–22 and 2026 but one, Maagh 2026,
///   which that list gives as 13 January and the rule as 14 January.
/// - **The Parkash of Guru Gobind Singh** is Poh sudi 7
///   ([`crate::hindu::GURU_GOBIND_SINGH_PARKASH`]).
/// - **Hola Mohalla** is Chet vadi 1 at sunrise
///   ([`crate::hindu::HOLA_MOHALLA`]). The day is the Encyclopaedia of
///   Sikhism's and the sunrise is fitted, to every day the SGPC kept from
///   2010 to 2026, so the rule is carried approximate.
/// - **Bandi Chhor Divas** is the days the SGPC kept, 2010 to 2026
///   ([`Rule::Listed`]), and a gap outside them. They are the days of
///   Diwali ([`crate::hindu::DIWALI`]) but in 2024 and 2025, when the SGPC
///   kept 1 November and 21 October, a day after it. No source read states
///   the SGPC's rule. The day whose sunset the new moon of Kārttika holds,
///   the later of two, gives all seventeen, but that reading is fitted
///   only, and is not carried as a rule.
/// - **The Parkash of Guru Nanak** is Kartik Puranmashi, as in the 2003
///   table ([`SIKH_NANAKSHAHI_2003`]).
///
/// **Partial.** The SGPC prints its dates each year in a *jantri*, and the
/// jantri of Nanakshahi 557 and 558 on sgpc.net is scanned pages, which
/// could not be read. The other gurpurabs are on Bikrami dates whose tithi
/// no source read gives, so they are not carried; SikhNet's lists of the
/// SGPC's days give Gregorian dates only.
pub static SIKH_SGPC: RuleSet = RuleSet {
    code: "sikh-sgpc",
    english_name: "Sikhism (SGPC, Bikrami calendar)",
    rules: SIKH_SGPC_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 28),
    sources: "SGPC, \"Gurpurbs\" (sgpc.net/gurpurbs, sgpc-gurpurbs), for Poh sudi 7, \
              Kartik Puranmashi and the observances it names; The Tribune, 16 January 2024 \
              (tribune-parkash-purb-2024), for the SGPC's Parkash of Guru Gobind Singh \
              on 9 January and 29 December 2022 and 17 January 2024; Wikipedia, \
              \"Nanakshahi calendar\", for the movable dates of the 2003 and 2010 versions \
              (secondary, from Purewal's list, not read); SikhNet, \"Sikh Gurpurab \
              Calendar\" of 2021-22 and 2026-27 (sikhnet-gurpurab-2021, \
              sikhnet-gurpurab-2026), \"as per SGPC Calendar\", for the sangrands and the \
              days of those years; the Encyclopaedia of Sikhism, \"Hola Mahalla\", as Asia \
              Samachar reprints it (asiasamachar-hola-mahalla-2015), for Chet wadi 1; The \
              Tribune's reports of Vaisakhi, one a year from 2010 to 2025 \
              (tribune-baisakhi-2010 to tribune-baisakhi-2025; those of 2020 and 2025 \
              by the weekday only), and of Hola Mohalla on 26 March 2024 \
              (tribune-hola-mohalla-2024), AIR (air-hola-mohalla-2025) and The Week \
              (theweek-khalsa-sajna-2025); dekho-ji (dekhoji-nanakshahi-sgpc, \
              secondary), PTC News (ptcnews-bandi-chhor-2024), The Tribune \
              (tribune-bandi-chhor-2025) and Babushahi (babushahi-bandi-chhor-2025, \
              secondary, a search summary) for Bandi Chhor Divas of 2022 to 2025; all \
              retrieved 2026-09-28",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// Zoroastrianism
// ─────────────────────────────────────────────────────────────────────────

/// A feast on a day of a month of the Zoroastrian year, in one reckoning.
const fn roz(system: CalendarSystem, name: &'static str, month: u8, day: u8) -> HolidayRule {
    feast(name, "", Rule::in_calendar(system, month, day))
}

/// The Parsi schedule of feasts, written once and dated in each of the
/// three reckonings — the same day of the same month in all three, which
/// is exactly why a Zoroastrian date needs its reckoning named.
///
/// The schedule is chapter 3 of the *Compendium of Fasli Zoroastrian
/// Calendars*: the name-day *jashan* of each month, the day whose name is
/// its month's (Meher 16, Mehregan); the six Gahambars of five days; Nowruz,
/// Rapithwin, Khordad Sal, Zartosht No-Diso and Avardad Sal Gah; the ten
/// days of Muktad at the year's end, the last five of them the Gatha days
/// and Hamaspathmaidyem. Where a feast has a name the wider literature
/// uses — Nowruz, Tiragan, Mehregan, Zartosht No-Diso — that name is
/// carried and the compendium's "Tir Month Jashan" is in the doc; the rest
/// are as the compendium heads them.
macro_rules! zoroastrian_schedule {
    ($system:expr) => {
        &[
            roz($system, "Nowruz", 1, 1),
            roz($system, "Rapithwin Jashan", 1, 3),
            roz($system, "Khordad Sal", 1, 6),
            roz($system, "Farvardin Month Jashan", 1, 19),
            roz($system, "Ardibehesht Month Jashan", 2, 3),
            roz($system, "Maidyozarem Gahambar", 2, 11),
            roz($system, "Maidyozarem Gahambar", 2, 12),
            roz($system, "Maidyozarem Gahambar", 2, 13),
            roz($system, "Maidyozarem Gahambar", 2, 14),
            roz($system, "Maidyozarem Gahambar", 2, 15),
            roz($system, "Khordad Month Jashan", 3, 6),
            roz($system, "Maidyoshahem Gahambar", 4, 11),
            roz($system, "Maidyoshahem Gahambar", 4, 12),
            // Tir 13 is both the third day of the Gahambar and Tiragan, the
            // name-day of Tir; the compendium prints both on the row.
            roz($system, "Maidyoshahem Gahambar", 4, 13),
            roz($system, "Tiragan", 4, 13),
            roz($system, "Maidyoshahem Gahambar", 4, 14),
            roz($system, "Maidyoshahem Gahambar", 4, 15),
            roz($system, "Amardad Month Jashan", 5, 7),
            roz($system, "Shahrewar Month Jashan", 6, 4),
            roz($system, "Paitishahem Gahambar", 6, 26),
            roz($system, "Paitishahem Gahambar", 6, 27),
            roz($system, "Paitishahem Gahambar", 6, 28),
            roz($system, "Paitishahem Gahambar", 6, 29),
            roz($system, "Paitishahem Gahambar", 6, 30),
            roz($system, "Mehregan", 7, 16),
            roz($system, "Ayathrem Gahambar", 7, 26),
            roz($system, "Ayathrem Gahambar", 7, 27),
            roz($system, "Ayathrem Gahambar", 7, 28),
            roz($system, "Ayathrem Gahambar", 7, 29),
            roz($system, "Ayathrem Gahambar", 7, 30),
            roz($system, "Avan Month Jashan", 8, 10),
            roz($system, "Adar Month Jashan", 9, 9),
            roz($system, "Fravardegan Jashan", 9, 19),
            // The Creator's four days in the Creator's month.
            roz($system, "Dae Month Jashan", 10, 1),
            roz($system, "Dae Month Jashan", 10, 8),
            roz($system, "Zartosht No-Diso", 10, 11),
            roz($system, "Dae Month Jashan", 10, 15),
            roz($system, "Maidyarem Gahambar", 10, 16),
            roz($system, "Maidyarem Gahambar", 10, 17),
            roz($system, "Maidyarem Gahambar", 10, 18),
            roz($system, "Maidyarem Gahambar", 10, 19),
            roz($system, "Maidyarem Gahambar", 10, 20),
            roz($system, "Dae Month Jashan", 10, 23),
            roz($system, "Bahman Month Jashan", 11, 2),
            roz($system, "Aspandard Month Jashan", 12, 5),
            // The jashan the wandering reckonings keep on the day the leap
            // day would have gone, and the Fasli keeps as well.
            roz($system, "Avardad Sal Gah Jashan", 12, 7),
            roz($system, "Muktad", 12, 26),
            roz($system, "Muktad", 12, 27),
            roz($system, "Muktad", 12, 28),
            roz($system, "Muktad", 12, 29),
            roz($system, "Mareshpand Jashan", 12, 29),
            roz($system, "Muktad", 12, 30),
            roz($system, "Muktad", 13, 1),
            roz($system, "Hamaspathmaidyem Gahambar", 13, 1),
            roz($system, "Muktad", 13, 2),
            roz($system, "Hamaspathmaidyem Gahambar", 13, 2),
            roz($system, "Muktad", 13, 3),
            roz($system, "Hamaspathmaidyem Gahambar", 13, 3),
            roz($system, "Muktad", 13, 4),
            roz($system, "Hamaspathmaidyem Gahambar", 13, 4),
            roz($system, "Muktad", 13, 5),
            roz($system, "Hamaspathmaidyem Gahambar", 13, 5),
        ]
    };
}

static ZOROASTRIAN_FASLI_RULES: &[HolidayRule] =
    zoroastrian_schedule!(CalendarSystem::ZOROASTRIAN_FASLI);
static ZOROASTRIAN_SHAHANSHAHI_RULES: &[HolidayRule] =
    zoroastrian_schedule!(CalendarSystem::ZOROASTRIAN_SHAHANSHAHI);
static ZOROASTRIAN_QADIMI_RULES: &[HolidayRule] =
    zoroastrian_schedule!(CalendarSystem::ZOROASTRIAN_QADIMI);

/// The sources every Zoroastrian table cites: the schedule is one document
/// and the reckonings are one article.
const ZOROASTRIAN_SOURCES: &str = "Rohinton Erach Kadva, Compendium of Fasli Zoroastrian Calendars 1379 AY \
     through 1400 AY, Bangalore, 2009, chapter 3, Schedule of Festivals \
     (zoroastrian.ru/files/eng/zoroastrian-calendars-1379-ay-1400-ay-fasli.pdf, \
     retrieved 2026-09-22), for every feast and its day of the month; \
     Wikipedia, \"Zoroastrian calendar\", retrieved 2026-09-22, for the \
     reckonings, which are hc-calendars-solar's zoroastrian module";

/// The Zoroastrian feasts as the Fasli Parsis keep them: the schedule on the
/// seasonal reckoning, so that Nowruz is 21 March, Mehregan 2 October,
/// Zartosht No-Diso 26 December and Muktad the ten days to 20 March — a day
/// earlier for the last twenty-one days of a Fasli leap year.
///
/// The Iranian community keeps the same name-day feasts on its own year of
/// thirty-day months from the civil Nowruz, which is
/// [`ZOROASTRIAN_IRANIAN`]; the civil calendar's dates of Tirgan and
/// Mehregan, and Sadeh and Yalda, which are Iranian festivals rather than
/// days of this schedule, are [`IRANIAN_FESTIVALS`].
pub static ZOROASTRIAN_FASLI: RuleSet = RuleSet {
    code: "zoroastrian-fasli",
    english_name: "Zoroastrian (Fasli)",
    rules: ZOROASTRIAN_FASLI_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: ZOROASTRIAN_SOURCES,
    subdivisions: Subdivisions::Undivided,
};

/// The Zoroastrian feasts as the Shahanshahi Parsis, the majority, keep
/// them: the same schedule on the wandering reckoning, so that in 1395 Y.Z.
/// Nowruz was 15 August 2025 and Zartosht No-Diso 22 May 2026, and every
/// feast comes a day earlier after each Gregorian leap day.
pub static ZOROASTRIAN_SHAHANSHAHI: RuleSet = RuleSet {
    code: "zoroastrian-shahanshahi",
    english_name: "Zoroastrian (Shahanshahi)",
    rules: ZOROASTRIAN_SHAHANSHAHI_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: ZOROASTRIAN_SOURCES,
    subdivisions: Subdivisions::Undivided,
};

/// The Zoroastrian feasts as the Kadmi Parsis and the Zoroastrians of Yazd
/// keep them: the same schedule on the Qadimi reckoning, thirty days ahead
/// of the Shahanshahi — Nowruz of 1395 Y.Z. on 16 July 2025.
pub static ZOROASTRIAN_QADIMI: RuleSet = RuleSet {
    code: "zoroastrian-qadimi",
    english_name: "Zoroastrian (Qadimi)",
    rules: ZOROASTRIAN_QADIMI_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: ZOROASTRIAN_SOURCES,
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// Ember and Rogation Days
// ─────────────────────────────────────────────────────────────────────────

/// Three Ember Days — a Wednesday, the Friday and the Saturday after it —
/// named for their season.
macro_rules! ember_days {
    ($season:literal, $wednesday:expr, $friday:expr, $saturday:expr) => {
        [
            feast(concat!("Ember Wednesday (", $season, ")"), "", $wednesday),
            feast(concat!("Ember Friday (", $season, ")"), "", $friday),
            feast(concat!("Ember Saturday (", $season, ")"), "", $saturday),
        ]
    };
}

/// The Monday, Tuesday and Wednesday before the Ascension.
macro_rules! rogation_days {
    ($monday:literal, $tuesday:literal, $wednesday:literal) => {
        [
            feast($monday, "", Rule::easter(ASCENSION - 3)),
            feast($tuesday, "", Rule::easter(ASCENSION - 2)),
            feast($wednesday, "", Rule::easter(ASCENSION - 1)),
        ]
    };
}

/// The Sunday after Ash Wednesday, the first Sunday in Lent.
const FIRST_SUNDAY_IN_LENT: i16 = ASH_WEDNESDAY + 4;

/// The Wednesday after 14 September, Holy Cross Day.
const WEDNESDAY_AFTER_HOLY_CROSS: Rule = Rule::WeekdayOnOrAfter {
    month: 9,
    day: 15,
    weekday: Weekday::Wednesday,
};

/// The Wednesday after 13 December, St Lucy's Day.
const WEDNESDAY_AFTER_ST_LUCY: Rule = Rule::WeekdayOnOrAfter {
    month: 12,
    day: 14,
    weekday: Weekday::Wednesday,
};

static EMBER_BCP1662_RULES: &[HolidayRule] = &dated(
    {
        let lent = ember_days!(
            "Lent",
            Rule::easter(FIRST_SUNDAY_IN_LENT + 3),
            Rule::easter(FIRST_SUNDAY_IN_LENT + 5),
            Rule::easter(FIRST_SUNDAY_IN_LENT + 6)
        );
        let whitsun = ember_days!(
            "Whitsun",
            Rule::easter(PENTECOST + 3),
            Rule::easter(PENTECOST + 5),
            Rule::easter(PENTECOST + 6)
        );
        let september = ember_days!(
            "September",
            WEDNESDAY_AFTER_HOLY_CROSS,
            Rule::Offset {
                base: &WEDNESDAY_AFTER_HOLY_CROSS,
                days: 2,
            },
            Rule::Offset {
                base: &WEDNESDAY_AFTER_HOLY_CROSS,
                days: 3,
            }
        );
        let december = ember_days!(
            "December",
            WEDNESDAY_AFTER_ST_LUCY,
            Rule::Offset {
                base: &WEDNESDAY_AFTER_ST_LUCY,
                days: 2,
            },
            Rule::Offset {
                base: &WEDNESDAY_AFTER_ST_LUCY,
                days: 3,
            }
        );
        let rogation = rogation_days!("Rogation Monday", "Rogation Tuesday", "Rogation Wednesday");
        [
            lent[0],
            lent[1],
            lent[2],
            rogation[0],
            rogation[1],
            rogation[2],
            whitsun[0],
            whitsun[1],
            whitsun[2],
            september[0],
            september[1],
            september[2],
            december[0],
            december[1],
            december[2],
        ]
    },
    Some(1662),
    Some(1753),
);

/// The Ember and Rogation Days of the *Book of Common Prayer* of 1662.
///
/// "The Ember Days at the Four Seasons, being the Wednesday, Friday and
/// Saturday after" the First Sunday in Lent, the Feast of Pentecost,
/// 14 September and 13 December, and "The three Rogation Days, being the
/// Monday, Tuesday, and Wednesday before Holy Thursday, or the Ascension
/// of our Lord", as the Prayer Book's table of vigils, fasts and days of
/// abstinence lists them. "The Wednesday, Friday and Saturday after
/// September 14" is read as one week: the first Wednesday after the day,
/// and the Friday and Saturday after that Wednesday, so that a Tuesday
/// 14 September gives the 15th, 17th and 18th, as Wikipedia's "Ember
/// days" states the older Western rule.
///
/// The same table's vigils, and its note moving a vigil off a Sunday, are
/// not carried: they are the eves of feasts, not Ember or Rogation Days.
/// Days of fasting are not days off, so every entry is religious.
///
/// The three churches' tables, the two readings of "after September 14"
/// and a worked example are in `docs/systems/ember-and-rogation-days.md`.
///
/// Absent before 1662, the year of the book; a gap from 1662 to 1752, when
/// England kept the Julian calendar, whose days this table does not
/// compute; answered from 1753, as `bcp-1662`'s calendar does.
pub static EMBER_BCP1662: RuleSet = RuleSet {
    code: "ember-bcp1662",
    english_name: "Ember and Rogation Days (Book of Common Prayer, 1662)",
    rules: EMBER_BCP1662_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "Church of England, Book of Common Prayer (1662), \"A Table of the Vigils, \
              Fasts, and Days of Abstinence, to be observed in the year\" \
              (churchofengland.org/sites/default/files/2017-10/5-table-vigils-fasts.pdf, \
              `bcp1662-vigils`), retrieved 2026-09-26; Wikipedia, \"Ember days\", \
              retrieved 2026-09-26, for the reading of the September and December weeks \
              (secondary)",
    subdivisions: Subdivisions::Undivided,
};

/// The Third Sunday of Advent, the Sunday between 11 and 17 December: two
/// weeks after the First, which falls 27 November to 3 December.
const THIRD_SUNDAY_OF_ADVENT: Rule = Rule::WeekdayOnOrAfter {
    month: 12,
    day: 11,
    weekday: Weekday::Sunday,
};

/// The Sunday nearest 29 June, which falls 26 June to 2 July.
const SUNDAY_NEAREST_29_JUNE: Rule = Rule::WeekdayOnOrAfter {
    month: 6,
    day: 26,
    weekday: Weekday::Sunday,
};

/// The Sunday nearest 29 September, which falls 26 September to 2 October.
const SUNDAY_NEAREST_29_SEPTEMBER: Rule = Rule::WeekdayOnOrAfter {
    month: 9,
    day: 26,
    weekday: Weekday::Sunday,
};

/// The Wednesday, Friday and Saturday of the week before a Sunday.
macro_rules! ember_week_before {
    ($season:literal, $sunday:expr) => {
        ember_days!(
            $season,
            Rule::Offset {
                base: &$sunday,
                days: -4,
            },
            Rule::Offset {
                base: &$sunday,
                days: -2,
            },
            Rule::Offset {
                base: &$sunday,
                days: -1,
            }
        )
    };
}

static EMBER_COMMON_WORSHIP_RULES: &[HolidayRule] = &dated(
    {
        // The Second Sunday of Lent is a week after the first.
        let lent = ember_days!(
            "Lent",
            Rule::easter(FIRST_SUNDAY_IN_LENT + 3),
            Rule::easter(FIRST_SUNDAY_IN_LENT + 5),
            Rule::easter(FIRST_SUNDAY_IN_LENT + 6)
        );
        let june = ember_week_before!("June", SUNDAY_NEAREST_29_JUNE);
        let september = ember_week_before!("September", SUNDAY_NEAREST_29_SEPTEMBER);
        let advent = ember_week_before!("Advent", THIRD_SUNDAY_OF_ADVENT);
        let rogation = rogation_days!("Rogation Monday", "Rogation Tuesday", "Rogation Wednesday");
        [
            lent[0],
            lent[1],
            lent[2],
            rogation[0],
            rogation[1],
            rogation[2],
            june[0],
            june[1],
            june[2],
            september[0],
            september[1],
            september[2],
            advent[0],
            advent[1],
            advent[2],
        ]
    },
    Some(2000),
    Some(2001),
);

/// The Ember and Rogation Days of the Church of England's *Common Worship*,
/// on the traditional weeks its rules name.
///
/// *Common Worship*'s "Rules to Order the Christian Year" say that "Ember
/// Days should be kept, under the bishop's directions, in the week before
/// an ordination", and that "Traditionally they have been observed on the
/// Wednesdays, Fridays and Saturdays within the weeks before the Third
/// Sunday of Advent, the Second Sunday of Lent and the Sundays nearest to
/// 29 June and 29 September"; and that "Rogation Days are the three days
/// before Ascension Day". **The first rule cannot be computed**: an
/// ordination's date is the bishop's, and no table of them is read. This
/// table is the second, the traditional weeks, and a diocese that keeps
/// its Ember Days before an ordination keeps them on other days.
///
/// It differs from [`EMBER_BCP1662`] in three seasons: the week before the
/// Sunday nearest 29 June rather than after Pentecost, before the Sunday
/// nearest 29 September rather than after 14 September, and before the
/// Third Sunday of Advent rather than after 13 December. The Lent week is
/// the same.
///
/// Absent before 2000, a gap in 2000, which holds only the Advent *Common
/// Worship* was launched on, and answered from 2001, as `common-worship`
/// is.
pub static EMBER_COMMON_WORSHIP: RuleSet = RuleSet {
    code: "ember-common-worship",
    english_name: "Ember and Rogation Days (Common Worship, traditional weeks)",
    rules: EMBER_COMMON_WORSHIP_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "Church of England, Common Worship, \"Rules to Order the Christian Year\", \
              \"Ember Days\" and \"Eastertide\" (churchofengland.org, prayer-and-worship/\
              worship-texts-and-resources/common-worship/churchs-year/rules, `cw-rules`), \
              retrieved 2026-09-26. The Third Sunday of Advent is two weeks after the First, \
              whose window is the Consultation on Common Texts' (`cct-rcl`)",
    subdivisions: Subdivisions::Undivided,
};

/// The Greater Litanies: 25 April, or the Tuesday after Easter when Easter
/// Sunday or Easter Monday falls on 25 April.
fn greater_litanies(year: i64) -> Option<Days> {
    let day = gregorian::to_fixed(year, 4, 25).ok()?;
    let easter = crate::computus::gregorian_easter(year)?;
    Some(if day == easter || day.0 == easter.0 + 1 {
        Days::one(Rd(easter.0 + 2))
    } else {
        Days::one(day)
    })
}

static ROGATION_ROMAN_1960_RULES: &[HolidayRule] = &{
    let lesser = rogation_days!(
        "Lesser Litanies (Rogation Monday)",
        "Lesser Litanies (Rogation Tuesday)",
        "Lesser Litanies (Rogation Wednesday)"
    );
    [
        feast(
            "Greater Litanies (Major Rogation)",
            "Litaniae maiores",
            Rule::Unsettled(greater_litanies),
        ),
        lesser[0],
        lesser[1],
        lesser[2],
    ]
};

/// The Rogation Days of the Roman Rite under the Code of Rubrics of 1960.
///
/// "The Greater Litanies are fixed on 25th April; but if Easter Sunday or
/// Monday after Easter falls on that day, they are transferred to the
/// following Tuesday" (no. 80); "The Lesser Litanies or Rogations are
/// normally (per se) fixed on the Monday, Tuesday and Wednesday before the
/// feast of our Lord's Ascension" (no. 87), which a local Ordinary may
/// move to three other days — a local decision this table does not know.
/// The Ember Days of the same rubrics are not carried: the text read names
/// them without the rule that dates the September week.
pub static ROGATION_ROMAN_1960: RuleSet = RuleSet {
    code: "rogation-roman-1960",
    english_name: "Rogation Days (Roman Rite, Code of Rubrics of 1960)",
    rules: ROGATION_ROMAN_1960_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "Code of Rubrics, approved by John XXIII's motu proprio Rubricarum instructum \
              of 25 July 1960, General Rubrics nos. 80 and 87, in the English translation \
              The New Rubrics of the Roman Breviary and Missal (1960), read in the copy at \
              cdn.restorethe54.com/media/pdf/the-new-rubrics-of-the-roman-missal-and-breviary-1960.pdf, \
              retrieved 2026-09-26 (`rubrics-1960`); Wikipedia, \"Rogation days\", \
              retrieved 2026-09-26, which pointed to no. 80",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// The Samaritans
// ─────────────────────────────────────────────────────────────────────────

/// A Samaritan festival, by the sources' month — the First Month is
/// Passover's — and day.
const fn samaritan(
    name: &'static str,
    local: &'static str,
    biblical_month: u8,
    day: u8,
) -> HolidayRule {
    // The calendar numbers its months from the Sixth, where the year
    // changes: the First to Fifth are public months 8 to 12, the Sixth to
    // Twelfth 1 to 7.
    let month = if biblical_month >= 6 {
        biblical_month - 5
    } else {
        biblical_month + 7
    };
    feast(
        name,
        local,
        Rule::in_calendar(CalendarSystem::SAMARITAN, month, day),
    )
    .approximate()
}

/// The first and last days of the Feast of Unleavened Bread, 15 and 21 of
/// the First Month.
const UNLEAVENED_BREAD_FIRST: Rule = Rule::in_calendar(CalendarSystem::SAMARITAN, 8, 15);
const UNLEAVENED_BREAD_LAST: Rule = Rule::in_calendar(CalendarSystem::SAMARITAN, 8, 21);

static SAMARITAN_RULES: &[HolidayRule] = &[
    samaritan("Passover sacrifice", "פסח", 1, 14),
    feast(
        "Feast of Unleavened Bread",
        "חג המצות",
        Rule::span(&UNLEAVENED_BREAD_FIRST, &UNLEAVENED_BREAD_LAST),
    )
    .approximate(),
    samaritan("Festival of the Seventh Month", "", 7, 1),
    samaritan("Day of Atonement", "יום הכפורים", 7, 10),
    samaritan("Festival of Sukkot (Tabernacles)", "סוכות", 7, 15),
    samaritan("Shemini Atseret (Day of Assembly)", "שמיני עצרת", 7, 22),
];

/// The festivals of the Israelite Samaritans, on the Samaritan calendar.
///
/// The Passover sacrifice on the fourteenth of the First Month and the
/// seven days of Unleavened Bread that follow it, the Festival of the
/// Seventh Month on its first day, the Day of Atonement on the tenth,
/// Sukkot on the fifteenth and Shemini Atseret on the twenty-second, as
/// the community's calendar and festival pages give them. They are dated
/// on `samaritan`, Reingold and Dershowitz's modern calculation, not the
/// priesthood's own, which is not published; its accuracy — the published
/// Passovers of 2010 and 2017–2020 and autumn 2026 agree, and 2016's,
/// 2025's and 2026's are a day late — is [`hc_calendars_lunar::samaritan`]'s,
/// so every festival is approximate, and outside its years, autumn 1900 to
/// autumn 2100, every festival is a reported gap. A Samaritan day
/// begins at the preceding sunset, so each festival begins on the evening
/// before the date given.
///
/// Shavuot is not yet carried. The community's page on it gives a counting
/// rule — fifty days from the day after the Sabbath that falls in the
/// seven days of Unleavened Bread, so always a Sunday — and the one dated
/// Shavuot read, the Palestinian Council of Ministers' Sunday 1 June 2025,
/// follows from it on the community's year, whose Unleavened Bread ran from
/// 12 April, and not on `samaritan`'s, which puts it on 8 June; on this
/// calendar the rule would miss the one date there is to check it against.
pub static SAMARITAN: RuleSet = RuleSet {
    code: "samaritan",
    english_name: "Samaritan festivals",
    rules: SAMARITAN_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "The Samaritans, \"The Samaritan Calendar\" and its upcoming festivals of \
              2026, and the festival pages \"The Festival of the Matzot\", \"The Festival \
              of the Seventh Month\", \"The Festival of Yom Kippur\", \"The Festival of \
              Succoth\" and \"The Festival of Shmini Atseret\" (the-samaritans.net, \
              `samaritans-net-calendar`), retrieved 2026-09-26; the Passover sacrifice on \
              the fourteenth of the First Month as `hc_calendars_lunar::samaritan` dates \
              it, checked against the Israelite Samaritan Information Institute's dates \
              (`samaritan-institute-calendar`)",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// The Mandaeans
// ─────────────────────────────────────────────────────────────────────────

/// A day of the Mandaean calendar, by its position — the Parwanaia are
/// position 9, so Qaina is 10 and Gadia 13 — and day.
const fn mandaean(month: u8, day: u8) -> Rule {
    Rule::in_calendar(CalendarSystem::MANDAEAN, month, day)
}

/// An inauspicious day, on which no enterprise or religious ceremony
/// should be undertaken.
const fn mbattal(rule: Rule) -> HolidayRule {
    HolidayRule::observance("Mbattal day", "", rule).of_kind(Kind::Religious)
}

const DEHWA_HNINA_FIRST: Rule = mandaean(4, 18);
const DEHWA_HNINA_LAST: Rule = mandaean(4, 20);
const PANJA_FIRST: Rule = mandaean(9, 1);
const PANJA_LAST: Rule = mandaean(9, 5);
const TAURA_FIRST: Rule = mandaean(4, 1);
const TAURA_FOURTH: Rule = mandaean(4, 4);
const SHUMBULTA_26: Rule = mandaean(8, 26);
const SHUMBULTA_30: Rule = mandaean(8, 30);
const GADIA_27: Rule = mandaean(13, 27);
const GADIA_29: Rule = mandaean(13, 29);

static MANDAEAN_RULES: &[HolidayRule] = &dated(
    [
        feast("Dehwa Rabba (New Year)", "", mandaean(1, 1)),
        feast("Dehwa d Shishlam Rba (Nauruz Zota)", "", mandaean(1, 6)),
        feast("Dehwa d Shishlam Rba (Nauruz Zota)", "", mandaean(1, 7)),
        feast(
            "Dehwa Hnina",
            "",
            Rule::span(&DEHWA_HNINA_FIRST, &DEHWA_HNINA_LAST),
        ),
        feast("Ashuriyah", "", mandaean(6, 1)),
        feast(
            "Panja (Parwanaia)",
            "",
            Rule::span(&PANJA_FIRST, &PANJA_LAST),
        ),
        feast("Dehwa Daimana", "", mandaean(12, 1)),
        feast("Kanshia uZahla (New Year's Eve)", "", mandaean(13, 30)),
        mbattal(mandaean(1, 6)),
        mbattal(mandaean(1, 7)),
        mbattal(mandaean(1, 22)),
        mbattal(mandaean(2, 25)),
        mbattal(Rule::span(&TAURA_FIRST, &TAURA_FOURTH)),
        mbattal(mandaean(6, 9)),
        mbattal(mandaean(6, 15)),
        mbattal(mandaean(6, 23)),
        mbattal(Rule::span(&SHUMBULTA_26, &SHUMBULTA_30)),
        mbattal(mandaean(10, 1)),
        mbattal(mandaean(12, 2)),
        mbattal(Rule::span(&GADIA_27, &GADIA_29)),
    ],
    None,
    Some(1932),
);

/// The feasts and *mbattal* days of the Mandaeans, on the Mandaean
/// calendar, as Drower recorded them.
///
/// The feasts: Dehwa Rabba, the New Year, on 1 Daula, and Kanshia uZahla,
/// its eve, on the last day of Gadia; the Little New Year, Nauruz Zota or
/// Dehwa d Shishlam Rba, on 6 and 7 Daula; Dehwa Hnina, the Little Feast,
/// on 18 Taura, which "lasts for three days"; Ashuriyah on 1 Sartana;
/// Panja, the five Parwanaia days; and Dehwa Daimana on 1 Hatia. The
/// mbattal days, "useless, inauspicious": 22 Daula, 25 Nuna, the first four
/// of Taura, the 9th, 15th and 23rd of Sartana, the last five of Shumbulta,
/// the three before Kanshia uZahla (pp. 85–92); the day after Panja (p. 60);
/// and the day after Dehwa Daimana and the sixth and seventh of the New
/// Year, among the "major mbattal days" (p. 211). "All Moslem festivals are
/// mbattal days" too (p. 92); they are the Islamic table's, and are not
/// repeated here.
///
/// Drower also says Dehwa Daimana "falls ninety days after Panja"; by her
/// own month table, 1 Hatia is 65 days after the first day of Panja, and
/// the day named is what is carried. The calendar has no leap day, so every
/// date drifts a day earlier against the Gregorian every four years. The
/// Mandaean day begins at dawn (p. 87); the date given is the civil day
/// that holds its daylight.
///
/// Read from 1932, the first year of the dates Drower records that the
/// calendar reproduces (Panja on 5 April 1932 to 1935, the new year of 1934
/// and 1935): the record of 1854 she quotes, Petermann's, parts from it by
/// a day, and no earlier year was read, so every year before 1932 is a gap.
pub static MANDAEAN: RuleSet = RuleSet {
    code: "mandaean",
    english_name: "Mandaean feasts",
    rules: MANDAEAN_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: "E. S. Drower, The Mandaeans of Iraq and Iran (Oxford: Clarendon Press, \
              1937), pp. 60, 84–92 and 211, read in the archive.org text \
              (`drower1937`), retrieved 2026-09-26, with the month names in her forms; \
              Wikipedia, \"Mandaean calendar\", retrieved 2026-09-26, for the 2024 dates \
              as a check (secondary)",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// The Qumran covenanters
// ─────────────────────────────────────────────────────────────────────────

/// A festival of the 364-day year, on its day and month.
const fn qumran(name: &'static str, month: u8, day: u8) -> HolidayRule {
    feast(
        name,
        "",
        Rule::in_calendar(CalendarSystem::QUMRAN, month, day),
    )
    // The weekday is the scrolls', the Gregorian date a convention of this
    // library's epoch, so no date is firmer than approximate.
    .approximate()
}

static QUMRAN_RULES: &[HolidayRule] = &read_all(
    -133,
    [
        qumran("Passover", 1, 14),
        qumran("Feast of Unleavened Bread", 1, 15),
        qumran("Waving of the Omer", 1, 26),
        qumran("Second Passover", 2, 14),
        qumran("Feast of Weeks", 3, 15),
        qumran("Day of Remembrance", 7, 1),
        qumran("Day of Atonement", 7, 10),
        qumran("Feast of Booths", 7, 15),
    ],
);

/// The festivals of the 364-day year of the Qumran scrolls, on `qumran`,
/// each on the same weekday every year: the Passover lamb on Tuesday 14/I,
/// Passover — the Feast of Unleavened Bread — on Wednesday 15/I, the Omer
/// on Sunday 26/I, the Second Passover on 14/II, the Feast of Weeks on
/// Sunday 15/III, the Day of Remembrance on 1/VII, the Day of Atonement on
/// Friday 10/VII and the Feast of Booths on Wednesday 15/VII, as Talmon
/// gives them and as 4Q320 4.ii sets them in the priestly courses.
///
/// The year has no intercalation and `qumran`'s epoch is a convention of
/// this library, chosen near an equinox at the turn of the era: the
/// weekday, the day of the year and the course of each festival are the
/// scrolls', and its Gregorian date is the convention's, not a
/// reconstruction (`docs/systems/qumran.md`). The festivals of the Temple
/// Scroll — the New Wine, the New Oil, the Wood Offering — are not carried:
/// no source read dates them.
///
/// Every entry is approximate: the weekday is the scrolls', but the
/// Gregorian date is the convention's, and none is more than a position in
/// the convention. No source read says in which years the covenanters kept
/// the calendar, so the table is read from 134 BCE, year −133, the earliest
/// year Wikipedia's "Qumran" dates the Hasmonean settlement to (secondary),
/// and every earlier year is a gap.
pub static QUMRAN: RuleSet = RuleSet {
    code: "qumran-festivals",
    english_name: "Qumran festivals",
    rules: QUMRAN_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "Shemaryahu Talmon, \"Calendars and Mishmarot\", Encyclopedia of the Dead \
              Sea Scrolls (2000), pp. 110–111 (`talmon2000`), read 2026-09-26 in a scanned \
              PDF, not re-read, as the test `the_first_years_festivals_fall_in_the_courses_of_4q320` \
              of `hc_calendars_solar::qumran` transcribes it; Wikipedia, \"Qumran calendrical \
              texts\" (`wikipedia-qumran-calendrical-texts`), retrieved 2026-09-29, for 4Q326's \
              Feast of Unleavened Bread on the fourth day of the week and the Barley Festival \
              on the 26th, secondary",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// Balinese Hinduism, on the Pawukon
// ─────────────────────────────────────────────────────────────────────────

/// The days of Gregorian `year`, from `first_month` to `last_month`, whose
/// Pawukon day `keep` accepts.
fn pawukon_days(year: i64, first_month: u8, last_month: u8, keep: fn(PawukonDate) -> bool) -> Days {
    let mut out = Days::new();
    let first = gregorian::to_fixed(year, first_month, 1);
    let last = if last_month == 12 {
        gregorian::to_fixed(year, 12, 31)
    } else {
        gregorian::to_fixed(year, last_month + 1, 1).map(|next| Rd(next.0 - 1))
    };
    let (Ok(first), Ok(last)) = (first, last) else {
        return out;
    };
    for day in first.0..=last.0 {
        if BalinesePawukonCalendar.from_fixed(Rd(day)).is_ok_and(keep) {
            out.push(Rd(day));
        }
    }
    out
}

/// The seven-day week's Buda, Wednesday.
const BUDA: u8 = 4;
/// The seven-day week's Saniscara, Saturday.
const SANISCARA: u8 = 7;
/// The five-day week's Kliwon.
const KLIWON: u8 = 5;
/// The three-day week's Kajeng.
const KAJENG: u8 = 3;
/// The *wuku* Dungulan, the eleventh.
const DUNGULAN: u8 = 11;
/// The *wuku* Kuningan, the twelfth.
const WUKU_KUNINGAN: u8 = 12;

/// Galungan, Buda Kliwon Dungulan.
fn galungan(year: i64) -> Days {
    pawukon_days(year, 1, 12, |date| {
        date.wuku() == DUNGULAN && date.saptawara() == BUDA && date.pancawara() == KLIWON
    })
}

/// Kuningan, Saniscara Kliwon Kuningan, ten days after Galungan.
fn kuningan(year: i64) -> Days {
    pawukon_days(year, 1, 12, |date| {
        date.wuku() == WUKU_KUNINGAN && date.saptawara() == SANISCARA && date.pancawara() == KLIWON
    })
}

/// Galungan and Kuningan as rules, the bases of the days around them.
static GALUNGAN: Rule = Rule::Computed(galungan);
static KUNINGAN: Rule = Rule::Computed(kuningan);

/// A day a number of days from Galungan or Kuningan, as English
/// Wikipedia's table of the days around Galungan gives it.
const fn around(name: &'static str, base: &'static Rule, days: i16) -> HolidayRule {
    HolidayRule::observance(name, name, Rule::Offset { base, days })
}

/// A Tumpek, Saniscara Kliwon, of the *wuku* numbered `W`: six to the
/// Pawukon, 35 days apart, from the second *wuku*, Landep.
fn tumpek<const W: u8>(year: i64) -> Days {
    pawukon_days(year, 1, 12, |date| {
        date.wuku() == W && date.saptawara() == SANISCARA && date.pancawara() == KLIWON
    })
}

/// Whether a day is Kajeng Kliwon, Kajeng in the three-day week and Kliwon
/// in the five.
const fn is_kajeng_kliwon(date: PawukonDate) -> bool {
    date.triwara() == KAJENG && date.pancawara() == KLIWON
}

/// Kajeng Kliwon from January to June: a rule yields at most
/// [`Days::CAPACITY`] days a year, and the day comes every fifteen, so the
/// year is taken in two halves.
fn kajeng_kliwon_first_half(year: i64) -> Days {
    pawukon_days(year, 1, 6, is_kajeng_kliwon)
}

/// Kajeng Kliwon from July to December.
fn kajeng_kliwon_second_half(year: i64) -> Days {
    pawukon_days(year, 7, 12, is_kajeng_kliwon)
}

/// A holy day of the Pawukon, an observance.
const fn pawukon_day(
    name: &'static str,
    local: &'static str,
    function: fn(i64) -> Days,
) -> HolidayRule {
    HolidayRule::observance(name, local, Rule::Computed(function))
}

static BALINESE_PAWUKON_DAYS_RULES: &[HolidayRule] = &read_all(
    2000,
    [
        around("Penyekeban", &GALUNGAN, -3),
        around("Penyajaan", &GALUNGAN, -2),
        around("Penampahan", &GALUNGAN, -1),
        feast("Galungan", "Galungan", Rule::Computed(galungan)),
        around("Manis Galungan", &GALUNGAN, 1),
        feast("Kuningan", "Kuningan", Rule::Computed(kuningan)),
        around("Manis Kuningan", &KUNINGAN, 1),
        pawukon_day("Tumpek Landep", "Tumpek Landep", tumpek::<2>),
        pawukon_day("Tumpek Wariga", "Tumpek Wariga", tumpek::<7>),
        pawukon_day("Tumpek Kuningan", "Tumpek Kuningan", tumpek::<12>),
        pawukon_day("Tumpek Krulut", "Tumpek Krulut", tumpek::<17>),
        pawukon_day("Tumpek Uye", "Tumpek Uye", tumpek::<22>),
        pawukon_day("Tumpek Wayang", "Tumpek Wayang", tumpek::<27>),
        pawukon_day("Kajeng Kliwon", "Kajeng Kliwon", kajeng_kliwon_first_half),
        pawukon_day("Kajeng Kliwon", "Kajeng Kliwon", kajeng_kliwon_second_half),
    ],
);

/// The holy days of Balinese Hinduism that the 210-day Pawukon,
/// `balinese-pawukon`, dates by its concurrent weeks: Galungan on Buda
/// Kliwon Dungulan, the Wednesday that is Kliwon in the *wuku* Dungulan;
/// Kuningan ten days later, Saniscara Kliwon Kuningan; the six Tumpek,
/// every Saniscara Kliwon, the Saturday that is Kliwon, named by their
/// *wuku* — Landep, Wariga, Kuningan, Krulut, Uye and Wayang; and Kajeng
/// Kliwon, every fifteen days, Kajeng in the three-day week and Kliwon in
/// the five.
///
/// The Pawukon repeats every 210 days and has no year, so each is exact:
/// Galungan and Kuningan once or twice a Gregorian year, each Tumpek once
/// or twice, and Kajeng Kliwon 24 or 25 times. Tumpek is Reingold and
/// Dershowitz's `tumpek`, "the 14th day of Pawukon and every 35th
/// subsequent day", and Kajeng Kliwon their `kajeng-keliwon`, "the 9th day
/// of each 15-day subcycle"; the Tumpek of the *wuku* Wariga is Tumpek
/// Uduh, "Saniscara Kliwon Wariga, 25 days before Galungan". Each Tumpek's
/// other names — Tumpek Kandang for Uye's, Tumpek Pengatag for Wariga's —
/// are not yet carried, as no source read tabulates them. The days around
/// Galungan are, as English Wikipedia's table gives them by their distance
/// from it: Penyekeban three days before, Penyajaan two, Penampahan one,
/// Manis Galungan the day after and Manis Kuningan the day after Kuningan,
/// observances. See `docs/systems/pawukon-and-pasaran.md`.
pub static BALINESE_PAWUKON_DAYS: RuleSet = RuleSet {
    code: "balinese-pawukon-days",
    english_name: "Balinese Hinduism (Pawukon holy days)",
    rules: BALINESE_PAWUKON_DAYS_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "Edward M. Reingold and Nachum Dershowitz, calendar-code2, `calendar.l` \
              (`reingold2018code`), `tumpek` and `kajeng-keliwon`, read 2026-09-29; \
              Wikipedia, \"Galungan\" (`wikipedia-galungan`), for Buda Keliwon Dunggulan, \
              the dates of Galungan and Kuningan, 2018–2028, and the days around them; Wikipedia, \"Kuningan \
              (Bali)\", for Saniscara Kliwon Wuku Kuningan; Wikipedia (id), \"Tumpek Uduh\" \
              and \"Kajeng Kliwon\" (`wikipedia-id-tumpek-uduh`, \
              `wikipedia-id-kajeng-kliwon`), for Tumpek Wariga and the fifteen-day round, \
              all secondary, retrieved 2026-09-29; the Pawukon as \
              `hc_calendars_regional::balinese_pawukon` computes it",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// The movable Orthodox name days
// ─────────────────────────────────────────────────────────────────────────

/// A name day on a movable feast, `offset` days from Pascha by the Julian
/// computus.
const fn movable_name_day(name: &'static str, local: &'static str, offset: i16) -> HolidayRule {
    HolidayRule::observance(name, local, Rule::paschal(offset))
}

/// A saint's day of April that the Church of Greece keeps after Pascha when
/// Pascha falls after 23 April: on `day` of April, or `after` days after
/// Pascha. `None` outside the computus's years.
fn after_pascha_if_late(year: i64, day: u8, after: i64) -> Option<Days> {
    let pascha = Computus::JULIAN.easter(year)?;
    let fixed = gregorian::to_fixed(year, 4, day).ok()?;
    let april_23 = gregorian::to_fixed(year, 4, 23).ok()?;
    Some(Days::one(if pascha > april_23 {
        Rd(pascha.0 + after)
    } else {
        fixed
    }))
}

/// St George, 23 April, or Easter Monday when Pascha falls after it.
fn st_george_greek(year: i64) -> Option<Days> {
    after_pascha_if_late(year, 23, 1)
}

/// St Mark, 25 April, or two days after Pascha when Pascha falls after
/// 23 April.
fn st_mark_greek(year: i64) -> Option<Days> {
    after_pascha_if_late(year, 25, 2)
}

static NAME_DAYS_GREEK_MOVABLE_RULES: &[HolidayRule] = &read_all(
    1925,
    [
        HolidayRule::observance(
            "Chloe",
            "Χλόη",
            Rule::WeekdayOnOrAfter {
                month: 2,
                day: 13,
                weekday: Weekday::Sunday,
            },
        ),
        movable_name_day(
            "Saturday of St Theodore (Theodore, Theodora)",
            "Αγίου Θεοδώρου",
            -43,
        ),
        movable_name_day(
            "Sunday of Orthodoxy (Orthodoxia)",
            "Κυριακή της Ορθοδοξίας",
            -42,
        ),
        movable_name_day("Gregory Palamas (Gregory)", "Γρηγορίου του Παλαμά", -35),
        movable_name_day("Lazarus Saturday (Lazarus)", "Σάββατο του Λαζάρου", -8),
        movable_name_day("Palm Sunday (Vaios, Vaia)", "Κυριακή των Βαΐων", -7),
        movable_name_day("Holy Monday (Pangalos)", "Μεγάλη Δευτέρα", -6),
        movable_name_day("Holy Thursday (Alitheia)", "Μεγάλη Πέμπτη", -3),
        movable_name_day("Pascha (Anastasios, Anastasia)", "Το Άγιο Πάσχα", 0),
        movable_name_day(
            "Bright Tuesday (Raphael, Nicholas and Irene of Mytilene)",
            "Τρίτη της Διακαινησίμου",
            2,
        ),
        movable_name_day(
            "Bright Wednesday (Theocharis)",
            "Τετάρτη της Διακαινησίμου",
            3,
        ),
        movable_name_day("Life-giving Spring (Zoe, Pigi)", "Ζωοδόχου Πηγής", 5),
        movable_name_day("Thomas Sunday (Thomas)", "Του Θωμά", 7),
        movable_name_day(
            "Sunday of the Myrrh-bearers (Myrofora)",
            "Των Μυροφόρων",
            14,
        ),
        movable_name_day("Sunday of the Paralytic (Vithesda)", "Του Παραλύτου", 21),
        movable_name_day(
            "Sunday of the Samaritan Woman (Photini)",
            "Της Σαμαρείτιδος",
            28,
        ),
        movable_name_day("Ascension (Nefeli)", "Ανάληψη του Χριστού", 39),
        movable_name_day("Monday of the Holy Spirit (Triada)", "Αγίου Πνεύματος", 50),
        movable_name_day(
            "All Saints (the names with no saint of their own)",
            "Αγίων Πάντων",
            56,
        ),
        HolidayRule::observance(
            "St George (George, Georgia)",
            "Αγίου Γεωργίου",
            Rule::Unsettled(st_george_greek),
        ),
        HolidayRule::observance(
            "St Mark (Mark)",
            "Μάρκου του Αποστόλου",
            Rule::Unsettled(st_mark_greek),
        ),
        HolidayRule::observance(
            "Sunday of the Forefathers",
            "Κυριακή των Προπατόρων",
            Rule::WeekdayOnOrAfter {
                month: 12,
                day: 11,
                weekday: Weekday::Sunday,
            },
        ),
    ],
);

/// The Greek name days that move, as the Greek Orthodox calendar keeps
/// them: the feasts counted from Pascha by the Julian computus on which
/// names are celebrated, with the principal names of each, from the
/// Saturday of St Theodore, 43 days before Pascha, to All Saints, 56 days
/// after, when "the names that have no known feast day" are celebrated;
/// St George on 23 April and St Mark on 25 April, each moved "1 day" or
/// "2 days after Pascha" when Pascha falls after 23 April; Chloe on the
/// first Sunday after 13 February, 13 February itself when it is a Sunday;
/// and the Forefathers on 11 December or the Sunday after.
///
/// The fixed dates are the Church of Greece's Revised Julian ones, which
/// are the Gregorian dates from 1600 to 2800. Which given names belong to
/// which feast is custom, and no church-published list exists
/// (`docs/systems/name-days.md`); the names given are the principal ones
/// the source prints, not its whole lists. The fixed name days are not
/// carried: they are a list, not rules, and no list may be shipped.
pub static NAME_DAYS_GREEK_MOVABLE: RuleSet = RuleSet {
    code: "name-days-greek-movable",
    english_name: "Greek movable name days",
    rules: NAME_DAYS_GREEK_MOVABLE_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "eortologio.gr, \"Υπολογισμός ημέρας του Πάσχα και κινητών γιορτών\" \
              (eortologio.gr/arthra/pasxa.php, `eortologio-pasxa`), the table of the \
              Orthodox movable feasts with their days from Pascha and their names, and \
              the special rules of Chloe, St George, St Mark and the Forefathers; its \
              \"about\" page for All Saints' names; Wikipedia (el), \"Άγιος Γεώργιος\", for \
              St George on the Monday of Bright Week when 23 April falls before \
              Pascha; all retrieved 2026-09-29, secondary",
    subdivisions: Subdivisions::Undivided,
};

static NAME_DAYS_BULGARIAN_MOVABLE_RULES: &[HolidayRule] = &read_all(
    2010,
    [
        movable_name_day("Todorovden (Todor, Teodora)", "Тодоровден", -43),
        movable_name_day("Lazarovden (Lazar)", "Лазаровден", -8),
        movable_name_day("Tsvetnitsa (names of flowers and plants)", "Цветница", -7),
        movable_name_day("Velikden (Veliko, Velika)", "Великден", 0),
        movable_name_day("Bright Friday (Zhivko, Zhivka)", "Светли петък", 5),
        movable_name_day("Thomas Sunday (Toma)", "Томина неделя", 7),
        movable_name_day("Spasovden (Spas)", "Спасовден", 39),
        movable_name_day("All Saints (Panayot)", "Всички светии", 56),
        movable_name_day(
            "All Bulgarian Saints (Rumen, Rumyana)",
            "Всички български светии",
            63,
        ),
    ],
);

/// The Bulgarian name days that move with Великден, Pascha by the Julian
/// computus: Тодоровден, the Saturday at the end of the first week of
/// Great Lent, 43 days before; Лазаровден, 8 days before; Цветница, the
/// Sunday before, for the names of flowers and plants; Великден itself;
/// Bright Friday, of the Life-giving Spring; Томина неделя, a week after;
/// Спасовден, the Ascension, 39 days after; Всички светии, 56; and All
/// Bulgarian Saints, 63, with the principal names the source prints. The
/// fixed name days, a list, are not carried (`docs/systems/name-days.md`).
pub static NAME_DAYS_BULGARIAN_MOVABLE: RuleSet = RuleSet {
    code: "name-days-bulgarian-movable",
    english_name: "Bulgarian movable name days",
    rules: NAME_DAYS_BULGARIAN_MOVABLE_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "Wikipedia (bg), \"Имен ден\" (`bgwiki-imen-den`), \"Подвижни имени дни в \
              България\": each feast's rule and names, and its table of the days of \
              2010–2023, secondary, retrieved 2026-09-29",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// The Yazidis
// ─────────────────────────────────────────────────────────────────────────

/// Serêsal, the Yazidi New Year: the first Wednesday of Nisan, the Eastern
/// April, in the Gregorian year's Julian year.
fn seresal(year: i64) -> Days {
    yazidi::new_year(year + yazidi::YEAR_OFFSET).map_or_else(|_| Days::new(), Days::one)
}

/// A Yazidi feast on its Eastern date, which is the Julian one.
const fn eastern(name: &'static str, local: &'static str, month: u8, day: u8) -> HolidayRule {
    feast(
        name,
        local,
        Rule::in_calendar(CalendarSystem::JULIAN, month, day),
    )
}

/// 1 Shawwāl, ʿAyd al-Fiṭr, on the tabular calendar.
const SHAWWAL_1: Rule = Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 1);

const WINTER_FAST_FIRST: Rule = Rule::in_calendar(CalendarSystem::JULIAN, 11, 28);
const WINTER_FAST_LAST: Rule = Rule::in_calendar(CalendarSystem::JULIAN, 11, 30);

static YAZIDI_RULES: &[HolidayRule] = &read_all(
    1901,
    [
        feast("Serêsal (New Year)", "Serêsal", Rule::Computed(seresal)),
        eastern(
            "Chilleyê Havînan (Forty Days of Summer) begins",
            "Chilleyê Havînan",
            6,
            10,
        ),
        eastern("Festival of the Assembly", "Jezhna Jema‘iyye", 9, 23),
        eastern("Festival of the Assembly", "Jezhna Jema‘iyye", 9, 24),
        eastern("Festival of the Assembly", "Jezhna Jema‘iyye", 9, 25),
        eastern("Festival of the Assembly", "Jezhna Jema‘iyye", 9, 26),
        eastern("Festival of the Assembly", "Jezhna Jema‘iyye", 9, 27),
        eastern("Festival of the Assembly", "Jezhna Jema‘iyye", 9, 28),
        eastern("Festival of the Assembly", "Jezhna Jema‘iyye", 9, 29),
        eastern("Festival of the Assembly", "Jezhna Jema‘iyye", 9, 30),
        feast(
            "Winter fast",
            "",
            Rule::span(&WINTER_FAST_FIRST, &WINTER_FAST_LAST),
        ),
        eastern("Bêlinde", "Bêlinde", 12, 1),
        // The mobile feasts, on the Islamic calendar (pp. 157–158).
        hijri("Sheva Berat", "Sheva Berat", 8, 15),
        feast(
            "Feast of Ramadan (Sheykh Khal Shemsan)",
            "Sheykh Khal Shemsan",
            Rule::Offset {
                base: &SHAWWAL_1,
                days: -2,
            },
        )
        .approximate(),
        hijri("Feast of ‘Erefat", "Jezhna ‘Erefat", 12, 9),
    ],
);

/// The Yazidi feasts that Kreyenbroek dates, on the Eastern calendar.
///
/// "In calculating the dates of their festivals Yezidis generally use the
/// Seleucid or 'Eastern' calendar, which in this century is thirteen days
/// behind the Gregorian" (p. 164 n. 53), which is the Julian calendar, so
/// the feasts are Julian dates: Serêsal, the New Year, "on the first
/// Wednesday of Nisan" (p. 151), as `yazidi` computes it; the Forty Days of
/// Summer, when "On the tenth day of Haziran (June)" the religious leaders
/// go to Sheykh Adi to fast (pp. 151–152); the Festival of the Assembly,
/// "the principal and central occasion of the Yezidi religious year",
/// "held from 23 to 30 September (Seleucid)" (p. 152); and the three-day
/// winter fast that "immediately precedes the Festival of Bêlinde on the
/// first of December" (p. 155).
///
/// The "mobile" feasts "follow the Islamic lunar calendar" (p. 150), and
/// are dated on the tabular one, approximate, as every Hijri day in this
/// crate is: Sheva Berat, the counterpart of the night of mid-Shaʿbān,
/// "celebrated on the same date, 15 Sha'ban"; the Feast of Ramadan, of
/// Sheykh Khal Shemsan, "two days before the Muslim festival of 'Ayd
/// al-Fitr"; and the Feast of ʿErefat, which "according to one source
/// falls on 9 Dhi 'l-Hijja" (pp. 157–158). That "the Yezidi festival
/// precedes the Islamic 'Ayd al-Adha by two days" is also reported, of the
/// custom of killing an animal, and the source cannot say how it relates
/// to the feast, so no day is carried for it.
///
/// Not carried: the Feast of the Dead "said to fall on 10 December" and
/// Khidr-Ilyas "said to fall on the first of February" (p. 156), which
/// the source reports with doubt, and which some Yazidis deny; and the
/// *tiwafs*, which are each village's.
pub static YAZIDI: RuleSet = RuleSet {
    code: "yazidi",
    english_name: "Yazidi feasts",
    rules: YAZIDI_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 29),
    sources: "Philip G. Kreyenbroek, Yezidism: Its Background, Observances and Textual \
              Tradition (Lewiston: Edwin Mellen Press, 1995), pp. 150–158 and 164 n. 53, \
              read in the archive.org text (`kreyenbroek1995`), retrieved 2026-09-26 and, \
              for the mobile feasts of pp. 157–158, 2026-09-29, with the names in his \
              forms; Serêsal as `hc_calendars_solar::yazidi` computes it; the mobile \
              feasts on the tabular Hijri calendar",
    subdivisions: Subdivisions::Undivided,
};

// ─────────────────────────────────────────────────────────────────────────
// The Armenian Apostolic Church
// ─────────────────────────────────────────────────────────────────────────

/// The moves that take a day to the Sunday on or after it.
const TO_SUNDAY_ON_OR_AFTER: &[(Weekday, i16)] = &[
    (Weekday::Monday, 6),
    (Weekday::Tuesday, 5),
    (Weekday::Wednesday, 4),
    (Weekday::Thursday, 3),
    (Weekday::Friday, 2),
    (Weekday::Saturday, 1),
];

/// The Armenian year in the calendar a see keeps and on its computus: the
/// six fixed feasts, the feasts on the Sunday nearest a date, and the
/// paschal cycle.
macro_rules! armenian_table {
    ($module:ident, $first:expr, $system:expr, $paschal:path) => {
        mod $module {
            use super::*;

            /// The Sunday nearest 15 August falls 12–18 August.
            const ASSUMPTION_EARLIEST: Rule = Rule::in_calendar($system, 8, 12);
            const ASSUMPTION: Rule =
                Rule::moved_by_weekday(&ASSUMPTION_EARLIEST, TO_SUNDAY_ON_OR_AFTER);
            /// The Sunday nearest 14 September falls 11–17 September.
            const EXALTATION_EARLIEST: Rule = Rule::in_calendar($system, 9, 11);
            const EXALTATION: Rule =
                Rule::moved_by_weekday(&EXALTATION_EARLIEST, TO_SUNDAY_ON_OR_AFTER);
            /// The Sunday nearest 7 May falls 4–10 May.
            const APPARITION_EARLIEST: Rule = Rule::in_calendar($system, 5, 4);
            const APPARITION: Rule =
                Rule::moved_by_weekday(&APPARITION_EARLIEST, TO_SUNDAY_ON_OR_AFTER);
            /// The Sunday nearest 18 November falls 15–21 November.
            const ADVENT_SUNDAY_EARLIEST: Rule = Rule::in_calendar($system, 11, 15);
            const ADVENT_SUNDAY: Rule =
                Rule::moved_by_weekday(&ADVENT_SUNDAY_EARLIEST, TO_SUNDAY_ON_OR_AFTER);

            pub(super) static RULES: &[HolidayRule] = &read_all(
                $first,
                [
                    feast(
                        "Theophany (Nativity and Baptism of Christ)",
                        "",
                        Rule::in_calendar($system, 1, 6),
                    ),
                    feast(
                        "Presentation of the Lord to the Temple",
                        "",
                        Rule::in_calendar($system, 2, 14),
                    ),
                    feast("Annunciation", "", Rule::in_calendar($system, 4, 7)),
                    feast(
                        "Nativity of the Mother of God",
                        "",
                        Rule::in_calendar($system, 9, 8),
                    ),
                    feast(
                        "Presentation of the Mother of God to the Temple",
                        "",
                        Rule::in_calendar($system, 11, 21),
                    ),
                    feast(
                        "Conception of the Mother of God",
                        "",
                        Rule::in_calendar($system, 12, 9),
                    ),
                    feast("Fast of the Catechumens begins", "", $paschal(-69)),
                    feast("Great Lent begins", "", $paschal(-48)),
                    feast("Easter", "", $paschal(EASTER_SUNDAY)),
                    feast("Apparition of the Cross", "", APPARITION),
                    feast("Transfiguration (Vardavar)", "Վարդավառ", $paschal(98)),
                    feast("Assumption of the Mother of God", "", ASSUMPTION),
                    feast("Exaltation of the Holy Cross", "", EXALTATION),
                    feast(
                        "Holy Cross of Varak",
                        "",
                        Rule::Offset {
                            base: &EXALTATION,
                            days: 21,
                        },
                    ),
                    feast(
                        "Discovery of the Holy Cross",
                        "",
                        Rule::Offset {
                            base: &EXALTATION,
                            days: 49,
                        },
                    ),
                    feast(
                        "Advent (Hisnag) begins",
                        "",
                        Rule::Offset {
                            base: &ADVENT_SUNDAY,
                            days: 1,
                        },
                    ),
                ],
            );
        }
    };
}

armenian_table!(
    armenian_gregorian,
    1583,
    CalendarSystem::GREGORIAN,
    Rule::easter
);
armenian_table!(armenian_julian, 326, CalendarSystem::JULIAN, Rule::paschal);

/// Where both Armenian tables take their rules from.
const ARMENIAN_SOURCES: &str = "Armenian Apostolic Church of Holy Resurrection, Sydney, \
    \"Liturgical Year of the Armenian Apostolic Church\" \
    (armenianchurchsydney.org.au/liturgical-year-of-the-armenian-apostolic-church/, \
    `armenian-church-sydney`), retrieved 2026-09-26, which credits Bishop Daniel \
    Findikyan by way of encyclopaedia.com (not read), for the fixed feasts, the Sundays nearest a \
    date, the feasts of the Cross, Lent, the Fast of the Catechumens, Advent and the \
    calendars; Wikipedia, \"Vardavar\", retrieved 2026-09-26, for the Transfiguration \
    98 days after Easter and its dates of 2010–2026 (secondary)";

/// The Armenian Apostolic Church as the Mother See of Holy Etchmiadzin
/// keeps its year: the Gregorian calendar and computus, which it adopted
/// "in 1923 for civil and liturgical use".
///
/// The six fixed feasts — Theophany, the Nativity and Baptism kept
/// together on 6 January, the Presentation on 14 February, the Annunciation
/// on 7 April, and the Nativity, Presentation and Conception of the Mother
/// of God on 8 September, 21 November and 9 December; the feasts kept on
/// the Sunday nearest a date — the Assumption to 15 August, the Exaltation
/// of the Cross to 14 September, the Apparition of the Cross to 7 May —
/// and those counted from the Exaltation, the Holy Cross of Varak on the
/// third Sunday after it and the Discovery of the Cross on the seventh;
/// Advent from the day after the Sunday nearest 18 November; and the
/// paschal cycle, the Fast of the Catechumens from "the third Monday before
/// Lent", Great Lent from "the seventh Monday before Easter", Easter, and
/// the Transfiguration, Vardavar, 98 days after it.
///
/// The saints' days, which fall "exclusively on Mondays, Tuesdays,
/// Thursdays, and Saturdays" and are "defined in relation to the closest
/// major feast and a day of the week", are not carried: no list of them was
/// read. The table has no years: which dioceses kept the Julian calendar
/// after 1923 is not a thing it says, and before 1923 the whole Church kept
/// the reckoning of [`CHRISTIAN_ARMENIAN_JERUSALEM`].
pub static CHRISTIAN_ARMENIAN: RuleSet = RuleSet {
    code: "christian-armenian",
    english_name: "Armenian Apostolic Church (Gregorian calendar)",
    rules: armenian_gregorian::RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: ARMENIAN_SOURCES,
    subdivisions: Subdivisions::Undivided,
};

/// The Armenian Apostolic Church as the Armenian Patriarchate of Jerusalem
/// keeps its year: the same feasts on the Julian calendar and the Julian
/// computus, since "Only the Armenian Patriarchate of Jerusalem follows the
/// Julian calendar because of the status quo of the Holy Places". Theophany
/// is therefore 19 January on a civil calendar until 2100, and a Sunday
/// nearest a Julian date is the Sunday nearest the civil date thirteen days
/// later.
pub static CHRISTIAN_ARMENIAN_JERUSALEM: RuleSet = RuleSet {
    code: "christian-armenian-jerusalem",
    english_name: "Armenian Apostolic Church (Patriarchate of Jerusalem, Julian calendar)",
    rules: armenian_julian::RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NO_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 26),
    sources: ARMENIAN_SOURCES,
    subdivisions: Subdivisions::Undivided,
};

/// Every tradition table in the crate.
pub static ALL: &[&RuleSet] = &[
    &CHRISTIAN_WESTERN,
    &crate::roman_calendar::GENERAL_ROMAN_CALENDAR,
    &crate::roman_calendar_1960::GENERAL_ROMAN_CALENDAR_1960,
    &CHRISTIAN_ORTHODOX,
    &CHRISTIAN_ORTHODOX_REVISED_JULIAN,
    &ETHIOPIAN_ORTHODOX,
    &COPTIC_ORTHODOX,
    &ISLAMIC,
    &JEWISH,
    &BAHAI,
    &HINDU,
    &BUDDHIST_THAI,
    &BUDDHIST_EAST_ASIAN,
    &BUDDHIST_TIBETAN,
    &BUDDHIST_TIBETAN_BERZIN,
    &BUDDHIST_TIBETAN_HENNING,
    &BUDDHIST_UPOSATHA_THAI,
    &CHINESE_FOLK,
    &CHINESE_XIAONIAN_NORTH,
    &CHINESE_XIAONIAN_SOUTH,
    &CHINESE_XIAONIAN_JIANGNAN,
    &CHINESE_XIAONIAN_NANJING,
    &CHINESE_XIAONIAN_SOUTHWEST,
    &TAOIST,
    &KOREAN_FOLK,
    &VIETNAMESE_FOLK,
    &WHEEL_OF_THE_YEAR,
    &WHEEL_OF_THE_YEAR_SOUTH,
    &JAIN,
    &SHINTO,
    &KYUCHU_SAISHI,
    &GOSEKKU,
    &OBON_JULY,
    &OBON_AUGUST,
    &OBON_LUNAR,
    &TORI_NO_ICHI,
    &HATSUUMA,
    &HATSUUMA_LUNAR,
    &INOKO,
    &INOKO_NOVEMBER,
    &TOKANYA,
    &TOKANYA_NOVEMBER,
    &SIKH_NANAKSHAHI_2003,
    &SIKH_SGPC,
    &ZOROASTRIAN_FASLI,
    &ZOROASTRIAN_SHAHANSHAHI,
    &ZOROASTRIAN_QADIMI,
    &ZOROASTRIAN_IRANIAN,
    &EMBER_BCP1662,
    &EMBER_COMMON_WORSHIP,
    &ROGATION_ROMAN_1960,
    &SAMARITAN,
    &MANDAEAN,
    &YAZIDI,
    &BALINESE_PAWUKON_DAYS,
    &NAME_DAYS_GREEK_MOVABLE,
    &NAME_DAYS_BULGARIAN_MOVABLE,
    &QUMRAN,
    &CHRISTIAN_ARMENIAN,
    &CHRISTIAN_ARMENIAN_JERUSALEM,
    &PLOUGH_DAYS,
    &CHAHARSHANBE_SURI,
    &IRANIAN_FESTIVALS,
    &TENRIKYO,
    &UNLUCKY_FRIDAYS,
    &SACRED_WEDNESDAYS,
    &CHURCH_OF_THE_EAST,
    &crate::east_syriac::CHALDEAN,
    &crate::east_syriac::SYRO_MALABAR,
    &crate::common_worship::COMMON_WORSHIP,
    &crate::book_of_common_prayer::BOOK_OF_COMMON_PRAYER_1662,
    &crate::book_of_common_prayer::BOOK_OF_COMMON_PRAYER_1871,
];

/// The table for a tradition's identifier, by
/// [`hc_core::catalogue::matches`].
#[must_use]
pub fn by_code(code: &str) -> Option<&'static RuleSet> {
    ALL.iter()
        .copied()
        .find(|set| hc_core::catalogue::matches(code, set.code))
}

hc_core::catalogue_tests! {
    type: &'static RuleSet,
    id: |set| set.code,
    provenance: |set| set.sources,
    tests: tradition_table_tests,
    all: ALL,
    lookup: by_code,
}
