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
//! | Christian, Orthodox | Julian computus, Julian fixed feasts | exact as stated; churches on the Revised Julian calendar keep the fixed feasts thirteen days earlier |
//! | Islamic | tabular civil Hijri | **approximate** — the observed date is a sighting decision |
//! | Jewish | arithmetic Hebrew calendar | exact; the day begins at the preceding sunset, which this crate does not model |
//! | Buddhist | approximated from the Chinese lunisolar calendar | **approximate** — see [`BUDDHIST`] |
//! | Chinese folk | Chinese lunisolar calendar and the solar terms | exact to the astronomical model |

use hc_calendar::Weekday;
use hc_seasons::{Meridian, SolarTerm};

use crate::computus::offsets::{
    ASCENSION, ASH_WEDNESDAY, CORPUS_CHRISTI, DIVINE_MERCY_SUNDAY, EASTER_MONDAY, EASTER_SUNDAY,
    GOOD_FRIDAY, HOLY_SATURDAY, MAUNDY_THURSDAY, PALM_SUNDAY, PENTECOST, SACRED_HEART,
    TRINITY_SUNDAY, WHIT_MONDAY,
};
use crate::rule::{CalendarSystem, HolidayRule, Kind, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate};

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

static CHRISTIAN_WESTERN_RULES: &[HolidayRule] = &[
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
];

/// Western Christianity: the Gregorian computus and the Gregorian-dated
/// fixed feasts.
pub static CHRISTIAN_WESTERN: RuleSet = RuleSet {
    code: "christian-western",
    english_name: "Christianity (Western computus)",
    rules: CHRISTIAN_WESTERN_RULES,
    substitution: &[],
    bridges: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "General Roman Calendar (1969, as revised); the movable feasts \
              are offsets from the Gregorian computus and nothing else",
};

// ─────────────────────────────────────────────────────────────────────────
// Christianity, Orthodox
// ─────────────────────────────────────────────────────────────────────────

static CHRISTIAN_ORTHODOX_RULES: &[HolidayRule] = &[
    // The fixed feasts are stated in the Julian calendar, so they appear
    // thirteen days later on a civil calendar for as long as the two
    // calendars are thirteen days apart, which is until 2100.
    feast(
        "Nativity of Christ",
        "",
        Rule::in_calendar(CalendarSystem::JULIAN, 12, 25),
    ),
    feast(
        "Theophany",
        "",
        Rule::in_calendar(CalendarSystem::JULIAN, 1, 6),
    ),
    feast(
        "Meeting of the Lord",
        "",
        Rule::in_calendar(CalendarSystem::JULIAN, 2, 2),
    ),
    feast(
        "Annunciation",
        "",
        Rule::in_calendar(CalendarSystem::JULIAN, 3, 25),
    ),
    feast(
        "Transfiguration",
        "",
        Rule::in_calendar(CalendarSystem::JULIAN, 8, 6),
    ),
    feast(
        "Dormition of the Theotokos",
        "",
        Rule::in_calendar(CalendarSystem::JULIAN, 8, 15),
    ),
    feast(
        "Nativity of the Theotokos",
        "",
        Rule::in_calendar(CalendarSystem::JULIAN, 9, 8),
    ),
    feast(
        "Exaltation of the Cross",
        "",
        Rule::in_calendar(CalendarSystem::JULIAN, 9, 14),
    ),
    feast(
        "Presentation of the Theotokos",
        "",
        Rule::in_calendar(CalendarSystem::JULIAN, 11, 21),
    ),
    // The movable cycle, from the Julian computus.
    feast("Clean Monday", "", Rule::paschal(ASH_WEDNESDAY - 2)),
    feast("Lazarus Saturday", "", Rule::paschal(-8)),
    feast("Palm Sunday", "", Rule::paschal(PALM_SUNDAY)),
    feast("Holy Friday", "", Rule::paschal(GOOD_FRIDAY)),
    feast("Pascha", "", Rule::paschal(EASTER_SUNDAY)),
    feast("Bright Monday", "", Rule::paschal(EASTER_MONDAY)),
    feast("Ascension", "", Rule::paschal(ASCENSION)),
    feast("Pentecost", "", Rule::paschal(PENTECOST)),
    feast("All Saints", "", Rule::paschal(TRINITY_SUNDAY)),
];

/// Orthodox Christianity: the Julian computus, and fixed feasts dated in the
/// Julian calendar.
///
/// Churches that adopted the Revised Julian calendar in 1923 — the
/// Ecumenical Patriarchate, Greece, Romania, Bulgaria and others — keep the
/// *fixed* feasts on dates that coincide with the Gregorian ones, while
/// still computing Pascha by the Julian computus. To model those, take the
/// movable entries here and the fixed entries from [`CHRISTIAN_WESTERN`].
pub static CHRISTIAN_ORTHODOX: RuleSet = RuleSet {
    code: "christian-orthodox",
    english_name: "Christianity (Julian computus)",
    rules: CHRISTIAN_ORTHODOX_RULES,
    substitution: &[],
    bridges: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "The Menaion and Pentecostarion as kept on the Julian calendar; \
              the Paschalion of the Council of Nicaea as Delambre's rule \
              computes it",
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

static ETHIOPIAN_ORTHODOX_RULES: &[HolidayRule] = &[
    ethiopic_feast("Enkutatash (New Year)", "እንቁጣጣሽ", 1, 1),
    ethiopic_feast("Meskel (Finding of the True Cross)", "መስቀል", 1, 17),
    ethiopic_feast("Genna (Christmas)", "ገና", 4, 29),
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
];

/// The feasts of the Ethiopian Orthodox Tewahedo Church.
///
/// The fixed ones are ordinary dates — 29 Tahsas, 11 Tirr — in the calendar
/// the church actually keeps, and until [`CalendarSystem`] stopped being a
/// closed enum **none of them could be written down at all**. The options
/// were to approximate them in a calendar they do not belong to, or to
/// leave them out.
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
/// # Calendrical date and kept date
///
/// These are the dates in the Ethiopic calendar. In a leap year the whole
/// Ethiopic year sits a day later against the Gregorian one, so 29 Tahsas
/// falls on 8 January rather than the 7th — and yet Genna is reported as
/// kept on 7 January across most of Ethiopia even then, with Lalibela the
/// exception. Enkutatash moves as the arithmetic says, to 12 September.
///
/// The table gives the calendrical date, because that is what "dated in the
/// Ethiopic calendar" means and it is the part that can be computed. Where a
/// feast is pinned to a Gregorian date by practice instead, that is a
/// different fact about a different thing, and one this crate would need a
/// source per country to state. Saying so is better than quietly choosing.
pub static ETHIOPIAN_ORTHODOX: RuleSet = RuleSet {
    code: "ethiopian-orthodox",
    english_name: "Ethiopian Orthodox Tewahedo",
    rules: ETHIOPIAN_ORTHODOX_RULES,
    substitution: &[],
    bridges: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "The fixed feasts of the Ethiopian Orthodox Tewahedo Church as \
              dated in the Ethiopic calendar; the movable cycle as the tewsak \
              of Bahire Hasab, offsets from Tinsae, which coincides with the \
              Julian-computus Pascha",
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
    hijri("Ashura", "عاشوراء", 1, 10),
    hijri("Mawlid an-Nabi", "المولد النبوي", 3, 12),
    hijri("Isra and Mi'raj", "الإسراء والمعراج", 7, 27),
    hijri("Mid-Sha'ban", "ليلة البراءة", 8, 15),
    hijri("First of Ramadan", "أول رمضان", 9, 1),
    hijri("Laylat al-Qadr", "ليلة القدر", 9, 27),
    hijri("Eid al-Fitr", "عيد الفطر", 10, 1),
    hijri("Day of Arafah", "يوم عرفة", 12, 9),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 10),
];

/// Islam.
///
/// **Every date here is a prediction.** The Hijri months of religious
/// practice begin on a crescent sighting decided per country, sometimes on
/// the evening before; the tabular civil calendar used here is a good
/// arithmetic approximation of that and nothing more. Countries that publish
/// their own table — Saudi Arabia's Umm al-Qurā, Türkiye's Diyanet — are
/// modelled in [`crate::countries`], and are still flagged approximate.
pub static ISLAMIC: RuleSet = RuleSet {
    code: "islamic",
    english_name: "Islam",
    rules: ISLAMIC_RULES,
    substitution: &[],
    bridges: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "The tabular civil Hijri calendar, CLDR `islamic-civil`. These \
              are computations, not announcements",
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

static JEWISH_RULES: &[HolidayRule] = &[
    hebrew_day("Rosh Hashanah", "ראש השנה", 1, 1),
    hebrew_day("Rosh Hashanah (second day)", "ראש השנה", 1, 2),
    hebrew_day("Fast of Gedaliah", "צום גדליה", 1, 3),
    hebrew_day("Yom Kippur", "יום כיפור", 1, 10),
    hebrew_day("Sukkot", "סוכות", 1, 15),
    hebrew_day("Hoshana Rabbah", "הושענא רבה", 1, 21),
    hebrew_day("Shemini Atzeret", "שמיני עצרת", 1, 22),
    hebrew_day("Simchat Torah", "שמחת תורה", 1, 23),
    hebrew_day("Hanukkah", "חנוכה", 3, 25),
    hebrew_day("Tenth of Tevet", "עשרה בטבת", 4, 10),
    hebrew_day("Tu BiShvat", "ט\"ו בשבט", 5, 15),
    hebrew_day("Purim", "פורים", 6, 14),
    hebrew_day("Shushan Purim", "שושן פורים", 6, 15),
    hebrew_day("Passover", "פסח", 7, 15),
    hebrew_day("Seventh Day of Passover", "שביעי של פסח", 7, 21),
    hebrew_day("Lag BaOmer", "ל\"ג בעומר", 8, 18),
    hebrew_day("Shavuot", "שבועות", 9, 6),
    hebrew_day("Seventeenth of Tammuz", "שבעה עשר בתמוז", 10, 17),
    hebrew_day("Tisha B'Av", "תשעה באב", 11, 9),
];

/// Judaism.
///
/// The Hebrew calendar is arithmetic, so every date here is exact. Two
/// things it does not model: a Jewish day begins at sunset on the preceding
/// evening, and several of these dates are *postponed* when they would fall
/// on the Sabbath — the Fast of Gedaliah and Tisha B'Av move to the Sunday,
/// the Tenth of Tevet never can. Those postponements are liturgical rules
/// this crate has not encoded.
pub static JEWISH: RuleSet = RuleSet {
    code: "jewish",
    english_name: "Judaism",
    rules: JEWISH_RULES,
    substitution: &[],
    bridges: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "The arithmetic Hebrew calendar as `hc-calendars-lunar` \
              implements it, following Dershowitz and Reingold, \
              Calendrical Calculations, chapter 8",
};

// ─────────────────────────────────────────────────────────────────────────
// Buddhism
// ─────────────────────────────────────────────────────────────────────────

static BUDDHIST_RULES: &[HolidayRule] = &[
    // Theravada, approximated: the Thai lunar month n is usually the
    // Chinese lunar month n − 2, and these are all full moons.
    feast(
        "Magha Puja",
        "วันมาฆบูชา",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 15),
    )
    .approximate(),
    feast(
        "Vesak",
        "วันวิสาขบูชา",
        Rule::in_calendar(CalendarSystem::CHINESE, 4, 15),
    )
    .approximate(),
    feast(
        "Asalha Puja",
        "วันอาสาฬหบูชา",
        Rule::in_calendar(CalendarSystem::CHINESE, 6, 15),
    )
    .approximate(),
    feast(
        "Vassa (Rains Retreat) begins",
        "วันเข้าพรรษา",
        Rule::in_calendar(CalendarSystem::CHINESE, 6, 16),
    )
    .approximate(),
    feast(
        "Pavarana (Rains Retreat) ends",
        "วันออกพรรษา",
        Rule::in_calendar(CalendarSystem::CHINESE, 9, 15),
    )
    .approximate(),
    // Mahayana, East Asian: fixed in the Gregorian calendar in Japan since
    // the 1873 adoption, and on the eighth of the fourth lunar month in
    // China, Korea and Vietnam.
    feast("Nirvana Day", "涅槃会", Rule::gregorian(2, 15)),
    feast("Buddha's Birthday", "灌仏会", Rule::gregorian(4, 8)),
    feast(
        "Buddha's Birthday (lunar reckoning)",
        "佛誕",
        Rule::in_calendar(CalendarSystem::CHINESE, 4, 8),
    ),
    feast("Bodhi Day", "成道会", Rule::gregorian(12, 8)),
];

/// Buddhism — **partial**, and honest about it.
///
/// Theravada observances are dated by the Thai, Burmese, Sinhalese or Lao
/// lunar calendars, none of which this crate implements. What it has is the
/// Chinese lunisolar calendar, whose month *n* is usually the Thai month
/// *n + 2*; that relation puts Vesak on the full moon of Chinese month 4,
/// which matched the Thai date in 2022, 2024 and 2025, missed by a day in
/// 2023 and would miss by a month in a Thai intercalary year. Every
/// Theravada entry is therefore [`Confidence::Approximate`](crate::rule::Confidence::Approximate).
///
/// The Mahayana entries are firmer: Japan fixed its Buddhist observances to
/// the Gregorian calendar when it adopted it in 1873, and the East Asian
/// lunar reckoning of Buddha's Birthday is an ordinary Chinese-calendar
/// date.
pub static BUDDHIST: RuleSet = RuleSet {
    code: "buddhist",
    english_name: "Buddhism",
    rules: BUDDHIST_RULES,
    substitution: &[],
    bridges: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Thai and Japanese Buddhist calendars. PARTIAL: the Theravada \
              dates are approximated from the Chinese lunisolar calendar \
              because no Thai, Burmese or Sinhalese lunar calendar exists in \
              `hc-calendars-lunar` yet",
};

// ─────────────────────────────────────────────────────────────────────────
// Chinese folk religion
// ─────────────────────────────────────────────────────────────────────────

static CHINESE_NEW_YEAR: Rule = Rule::in_calendar(CalendarSystem::CHINESE, 1, 1);

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
    feast(
        "Lantern Festival",
        "元宵節",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 15),
    ),
    feast(
        "Qingming Festival",
        "清明節",
        Rule::SolarTerm {
            term: QINGMING,
            meridian: Meridian::CHINA,
        },
    ),
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
    feast(
        "Winter Solstice Festival",
        "冬至",
        Rule::SolarTerm {
            term: SolarTerm::WINTER_SOLSTICE,
            meridian: Meridian::CHINA,
        },
    ),
];

/// Chinese folk religion and the festivals of the Chinese year.
pub static CHINESE_FOLK: RuleSet = RuleSet {
    code: "chinese-folk",
    english_name: "Chinese folk tradition",
    rules: CHINESE_FOLK_RULES,
    substitution: &[],
    bridges: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "The Chinese lunisolar calendar and the 24 solar terms, both \
              computed at the Beijing meridian by `hc-calendars-lunar` and \
              `hc-seasons`",
};

// ─────────────────────────────────────────────────────────────────────────

/// Every tradition table in the crate.
pub static ALL: &[&RuleSet] = &[
    &CHRISTIAN_WESTERN,
    &CHRISTIAN_ORTHODOX,
    &ETHIOPIAN_ORTHODOX,
    &ISLAMIC,
    &JEWISH,
    &BUDDHIST,
    &CHINESE_FOLK,
];

/// The table for a tradition's identifier.
#[must_use]
pub fn by_code(code: &str) -> Option<&'static RuleSet> {
    ALL.iter().copied().find(|set| set.code == code)
}

hc_core::catalogue_tests! {
    type: &'static RuleSet,
    id: |set| set.code,
    provenance: |set| set.sources,
    tests: tradition_table_tests,
    all: ALL,
    lookup: by_code,
}
