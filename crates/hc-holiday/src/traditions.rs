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
//! | Ethiopian Orthodox | Ethiopic calendar; the Bahire Hasab cycle as offsets from the Julian-computus Pascha | exact as stated; a feast kept on a fixed Gregorian date by practice is not modelled |
//! | Coptic Orthodox | Coptic calendar; the paschal cycle as offsets from the Julian-computus Pascha | exact |
//! | Islamic | tabular civil Hijri | **approximate** — the observed date is a sighting decision |
//! | Jewish | arithmetic Hebrew calendar | exact; the day begins at the preceding sunset, which this crate does not model |
//! | Bahá'í | the Badíʿ calendar as kept — arithmetic to 171 BE, the Bahá'í World Centre's table for 172–221 BE; the Twin Holy Birthdays from the same table | exact through 19 March 2065, and a reported gap after, where the table ends |
//! | Buddhist | approximated from the Chinese lunisolar calendar | **approximate** — see [`BUDDHIST`] |
//! | Chinese folk | Chinese lunisolar calendar and the solar terms | exact to the astronomical model |
//! | Hindu | the amānta Hindu lunisolar calendar at the national almanac's sunrise; each festival on the part of the day its tithi must hold | exact to the astronomical model, and to the conventions [`crate::hindu`] states — a regional almanac may keep a day differently |
//! | Wheel of the Year | the solstices and equinoxes on their Universal Time day; the cross-quarter days on their fixed Gregorian dates | exact as stated; a group may keep a quarter day on its local date or the nearest weekend, and the eve convention for Samhain is not modelled |
//! | Jain | the amānta Hindu lunisolar calendar at the national almanac's sunrise; Paryuṣaṇa and Daśa Lakṣaṇa counted back from their last days | **approximate** — Jain almanacs differ from the national one by a day in some years, as the source says |
//! | Shinto | fixed Gregorian dates, and 節分 as the day before 立春 at the Japanese meridian | exact; a shrine's own festival dates are not carried |
//! | Imperial court rites (宮中祭祀) | fixed Gregorian dates and the two equinox days at the Japanese meridian | exact for the Reiwa-era schedule the source gives; the rites tied to a reign change with it |
//! | Sikh | the Nanakshahi calendar of 2003 for the gurpurabs; the amānta Hindu lunisolar calendar for the three the 2003 calendar left on the Bikrami | exact: the Nanakshahi dates are fixed Gregorian dates, and the lunar three follow the same model as the Hindu table; the SGPC's post-2010 dates are not carried |
//! | Zoroastrian | the Parsi schedule of feasts on each of the three reckonings — Fasli, Shahanshahi, Qadimi — as three tables | exact: every feast is a fixed day of a fixed month, and each reckoning is arithmetic; the Iranian community's dates on the civil calendar are not carried |

use hc_calendar::Weekday;
use hc_calendars_solar::{bahai, gregorian};
use hc_seasons::{Meridian, SolarTerm};

use crate::computus::offsets::{
    ASCENSION, ASH_WEDNESDAY, CORPUS_CHRISTI, DIVINE_MERCY_SUNDAY, EASTER_MONDAY, EASTER_SUNDAY,
    GOOD_FRIDAY, HOLY_SATURDAY, MAUNDY_THURSDAY, PALM_SUNDAY, PENTECOST, SACRED_HEART,
    TRINITY_SUNDAY, WHIT_MONDAY,
};
use crate::hindu::{
    AKSHAYA_TRITIYA, ANANT_CHATURDASHI, BUDDHA_PURNIMA, DIWALI, DURGA_ASHTAMI, GANESH_CHATURTHI,
    GURU_NANAK_JAYANTI, GURU_PURNIMA, HOLI, HOLIKA_DAHAN, JANMASHTAMI, MAHA_SHIVARATRI,
    MAHAVIR_JAYANTI, MAKAR_SANKRANTI, MESHA_SANKRANTI, NAVARATRI, RAKSHA_BANDHAN, RAMA_NAVAMI,
    SAMVATSARI, UGADI, VIJAYA_DASHAMI,
};
use crate::rule::{
    CalendarSystem, Days, HolidayRule, Kind, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate,
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

static COPTIC_ORTHODOX_RULES: &[HolidayRule] = &[
    coptic_feast("Nayrouz (New Year)", "عيد النيروز", 1, 1),
    coptic_feast("Feast of the Cross", "عيد الصليب", 1, 17),
    coptic_feast("Nativity (Christmas)", "عيد الميلاد المجيد", 4, 29),
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
];

/// The feasts of the Coptic Orthodox Church of Alexandria.
///
/// The fourteen feasts of the Lord — seven major, seven minor — with Nayrouz,
/// both Feasts of the Cross, the Apostles and the two feasts of St Mary,
/// dated in the Coptic calendar the church keeps; and the paschal cycle from
/// the Fast of Nineveh to Pentecost as offsets from the Julian-computus
/// Easter, which is the Alexandrian one. This is the second of the calendars
/// the closed `CalendarSystem` could not name, after the Ethiopian.
///
/// The Arabic names are the ones the church's own publications use; the
/// Coptic-language names of the feasts are not carried.
pub static COPTIC_ORTHODOX: RuleSet = RuleSet {
    code: "coptic-orthodox",
    english_name: "Coptic Orthodox",
    rules: COPTIC_ORTHODOX_RULES,
    substitution: &[],
    bridges: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "The Feasts of the Church, Coptic Orthodox Diocese of Los Angeles \
              (lacopts.org), retrieved 2026-09-22, for the fixed dates; the \
              movable cycle as offsets from the Julian-computus Pascha the \
              church shares",
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
fn birth_of_the_bab(year: i64) -> Days {
    let (month, day) = match year {
        2015 => (11, 13),
        2016 => (11, 1),
        2017 => (10, 21),
        2018 => (11, 9),
        2019 => (10, 29),
        2020 => (10, 18),
        2021 => (11, 6),
        2022 => (10, 26),
        2023 => (10, 16),
        2024 => (11, 2),
        2025 => (10, 22),
        2026 => (11, 10),
        2027 => (10, 30),
        2028 => (10, 19),
        2029 => (11, 7),
        2030 => (10, 28),
        2031 => (10, 17),
        2032 => (11, 4),
        2033 => (10, 24),
        2034 => (11, 12),
        2035 => (11, 1),
        2036 => (10, 20),
        2037 => (11, 8),
        2038 => (10, 29),
        2039 => (10, 19),
        2040 => (11, 6),
        2041 => (10, 26),
        2042 => (10, 15),
        2043 => (11, 3),
        2044 => (10, 22),
        2045 => (11, 10),
        2046 => (10, 30),
        2047 => (10, 20),
        2048 => (11, 7),
        2049 => (10, 28),
        2050 => (10, 17),
        2051 => (11, 5),
        2052 => (10, 24),
        2053 => (11, 11),
        2054 => (11, 1),
        2055 => (10, 21),
        2056 => (11, 8),
        2057 => (10, 29),
        2058 => (10, 18),
        2059 => (11, 6),
        2060 => (10, 25),
        2061 => (10, 14),
        2062 => (11, 2),
        2063 => (10, 23),
        2064 => (11, 10),
        _ => return Days::new(),
    };
    gregorian::to_fixed(year, month, day).map_or_else(|_| Days::new(), Days::one)
}

/// The tabulated Birth of the Báb; the Birth of Bahá'u'lláh is the day after.
const BIRTH_OF_THE_BAB: Rule = Rule::Tabulated {
    function: birth_of_the_bab,
    first_year: 2015,
    last_year: 2064,
};

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
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Badíʿ dates 172 to 221 BE, prepared by an ad hoc committee at the \
              Bahá'í World Centre from data of HM Nautical Almanac Office, \
              2014 (bahai-library.com/pdf/uhj/uhj_bahai_dates_172-221.pdf, \
              retrieved 2026-09-22), for the Badíʿ date of every holy day and \
              the Twin Holy Birthdays; the Universal House of Justice, letter \
              of 10 July 2014, for the lunar rule; Days of Remembrance, Bahá'í \
              Reference Library, Persian edition, for the names, with the \
              community's usual terms for the days it does not head",
};

// ─────────────────────────────────────────────────────────────────────────
// Hinduism
// ─────────────────────────────────────────────────────────────────────────

static HINDU_RULES: &[HolidayRule] = &[
    feast("Makar Sankranti", "मकर संक्रांति", MAKAR_SANKRANTI),
    feast("Maha Shivaratri", "महाशिवरात्रि", MAHA_SHIVARATRI),
    feast("Holika Dahan", "होलिका दहन", HOLIKA_DAHAN),
    feast("Holi", "होली", HOLI),
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
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "The Rashtriya Panchang, Positional Astronomy Centre, India \
              Meteorological Department, Śaka 1945 and 1946 (2023–2025), \
              English editions: the \"Principal Festivals and Anniversaries\" \
              list, which every rule here reproduces, and the prevalence \
              conventions as `hindu` states them",
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

static WHEEL_OF_THE_YEAR_RULES: &[HolidayRule] = &[
    cross_quarter_day("Imbolc", 2, 1),
    quarter_day("Ostara", SolarTerm::SPRING_EQUINOX),
    cross_quarter_day("Beltane", 5, 1),
    quarter_day("Litha", SolarTerm::SUMMER_SOLSTICE),
    cross_quarter_day("Lughnasadh", 8, 1),
    quarter_day("Mabon", SolarTerm::AUTUMN_EQUINOX),
    cross_quarter_day("Samhain", 11, 1),
    quarter_day("Yule", SolarTerm::WINTER_SOLSTICE),
];

static WHEEL_OF_THE_YEAR_SOUTH_RULES: &[HolidayRule] = &[
    cross_quarter_day("Lughnasadh", 2, 1),
    quarter_day("Mabon", SolarTerm::SPRING_EQUINOX),
    cross_quarter_day("Samhain", 5, 1),
    quarter_day("Yule", SolarTerm::SUMMER_SOLSTICE),
    cross_quarter_day("Imbolc", 8, 1),
    quarter_day("Ostara", SolarTerm::AUTUMN_EQUINOX),
    cross_quarter_day("Beltane", 11, 1),
    quarter_day("Litha", SolarTerm::WINTER_SOLSTICE),
];

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
pub static WHEEL_OF_THE_YEAR: RuleSet = RuleSet {
    code: "wheel-of-the-year",
    english_name: "Wheel of the Year",
    rules: WHEEL_OF_THE_YEAR_RULES,
    substitution: &[],
    bridges: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Wikipedia, \"Wheel of the Year\", retrieved 2026-09-22, for the eight \
              festivals, their dates in each hemisphere and the cycle's \
              mid-twentieth-century origin",
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
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Wikipedia, \"Wheel of the Year\", retrieved 2026-09-22, southern-hemisphere \
              column",
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
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Wikipedia, \"Paryushana\", retrieved 2026-09-22, for the eight \
              Śvetāmbara days ending with Saṃvatsarī on Bhadrapada śukla 4, the \
              ten Digambara days that start right after, and the sects' \
              computational differences; Wikipedia, \"Kshamavani\", retrieved \
              2026-09-22, for the day not carried; Wikipedia, \"Diwali\", \
              retrieved 2026-09-22, for the Jain observance; the Rashtriya \
              Panchang for Mahāvīra Jayantī and Akṣaya Tṛtīyā, as `hindu` \
              states",
};

// ─────────────────────────────────────────────────────────────────────────
// Shinto
// ─────────────────────────────────────────────────────────────────────────

/// 立春 at the Japanese meridian, the day 節分 precedes.
static RISSHUN: Rule = Rule::SolarTerm {
    term: SolarTerm::BEGINNING_OF_SPRING,
    meridian: Meridian::JAPAN,
};

static SHINTO_RULES: &[HolidayRule] = &[
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
];

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
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Wikipedia (ja), \"節分\", retrieved 2026-09-22, for the rule — the \
              day before 立春, the Sun at longitude 315° — and its dates by \
              period; \"七五三\", for 15 November since the Meiji reform; \
              \"初詣\", for the New Year visit; \"宮中祭祀\", for 大祓 on \
              30 June and 31 December",
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

static KYUCHU_SAISHI_RULES: &[HolidayRule] = &[
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
];

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
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Wikipedia (ja), \"宮中祭祀\", retrieved 2026-09-22, for the table \
              of 恒例祭 and the 旬祭; \"天皇誕生日\", retrieved 2026-09-22, for \
              23 February from 2020",
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
    // where the SGPC has kept the Bikrami saṅkrānti of 13 or 14 April
    // since 2010.
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
    // a compromise": Hola Mohalla on the day of Holi, Bandi Chhor Divas on
    // Diwali, and Guru Nanak's Parkash on the full moon of Kārttika.
    feast("Hola Mohalla", "", HOLI),
    feast("Bandi Chhor Divas", "", DIWALI),
    feast("Parkash of Guru Nanak", "", GURU_NANAK_JAYANTI),
];

/// Sikhism, on the Nanakshahi calendar of 2003.
///
/// The gurpurabs on the fixed dates that calendar gave them — Guru Gobind
/// Singh's Parkash on 23 Poh, 5 January; Guru Arjan's Shaheedi on 2 Harh,
/// 16 June; Guru Tegh Bahadur's on 11 Maghar, 24 November — and the three
/// observances it kept lunar: Hola Mohalla, Bandi Chhor Divas and the
/// Parkash of Guru Nanak, on the same rules as Holi, Diwali and Kartik
/// Purnima in the Hindu table, whose dates the SGPC's own 2003–2020 list
/// matches.
///
/// **This is the 2003 calendar, the *Mool* Nanakshahi.** The SGPC's 2010
/// amendments returned several gurpurabs to lunar dates and its 2014
/// calendar is the Bikrami calendar under the Nanakshahi name; the dates it
/// publishes since are not carried, and the 2017 resolution of the Mool
/// calendar's supporters to fix the three lunar days as well is not either.
/// The Akal Takht's foundation day is omitted: the source's row gives
/// 18 Harh against 16 June, which is 2 Harh, and the table does not guess.
pub static SIKH: RuleSet = RuleSet {
    code: "sikh",
    english_name: "Sikhism",
    rules: SIKH_RULES,
    substitution: &[],
    bridges: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Wikipedia, \"Nanakshahi calendar\", retrieved 2026-09-22: the table \
              of festivals and events of the 2003 version for every fixed \
              date, and its table of the movable dates 2003–2020 for the three \
              lunar observances",
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
/// The Iranian community keeps the same feasts on the civil Solar Hijri
/// calendar under the name *Bastani*, and where that calendar's 31-day
/// months put a feast — Tiragan on 10 or 13 Tir, Mehregan on 10 or 16 Mehr —
/// its sources disagree; those dates, and Sadeh and Yalda, which are
/// Iranian festivals rather than days of this schedule, are not carried.
pub static ZOROASTRIAN_FASLI: RuleSet = RuleSet {
    code: "zoroastrian-fasli",
    english_name: "Zoroastrian (Fasli)",
    rules: ZOROASTRIAN_FASLI_RULES,
    substitution: &[],
    bridges: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: ZOROASTRIAN_SOURCES,
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
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: ZOROASTRIAN_SOURCES,
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
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: ZOROASTRIAN_SOURCES,
};

/// Every tradition table in the crate.
pub static ALL: &[&RuleSet] = &[
    &CHRISTIAN_WESTERN,
    &CHRISTIAN_ORTHODOX,
    &ETHIOPIAN_ORTHODOX,
    &COPTIC_ORTHODOX,
    &ISLAMIC,
    &JEWISH,
    &BAHAI,
    &HINDU,
    &BUDDHIST,
    &CHINESE_FOLK,
    &WHEEL_OF_THE_YEAR,
    &WHEEL_OF_THE_YEAR_SOUTH,
    &JAIN,
    &SHINTO,
    &KYUCHU_SAISHI,
    &SIKH,
    &ZOROASTRIAN_FASLI,
    &ZOROASTRIAN_SHAHANSHAHI,
    &ZOROASTRIAN_QADIMI,
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
