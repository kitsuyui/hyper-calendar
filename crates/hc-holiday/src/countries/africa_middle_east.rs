//! Tables for the Middle East and Africa.
//!
//! Most of the crate's **Friday–Saturday weekends** are in this file, and so
//! are most of its weekend changes: Saudi Arabia moved from Thursday–Friday
//! in June 2013, and the United Arab Emirates to Saturday–Sunday on
//! 1 January 2022. So is **Israel's Independence Day**, whose statute is a
//! sentence rather than a pattern and which is therefore a
//! [`Rule::Computed`] rule.

use hc_calendar::{Month, Rd, Weekday};
use hc_calendars_indic::nakshatra::PUSHYA;
use hc_calendars_solar::gregorian;
use hc_seasons::Meridian;
use hc_seasons::zodiac::{Ayanamsa, SiderealSign};

use crate::computus::offsets::{
    ASCENSION, CORPUS_CHRISTI, EASTER_MONDAY, EASTER_SUNDAY, GOOD_FRIDAY, HOLY_SATURDAY,
    MAUNDY_THURSDAY, PALM_SUNDAY, PENTECOST, SHROVE_TUESDAY, WHIT_MONDAY,
};
use crate::hindu::{DIWALI, GANESH_CHATURTHI, MAHA_SHIVARATRI, UGADI};
use crate::rule::{
    BridgePolicy, CalendarSystem, Days, HolidayRule, Kind, Rule, RuleSet, SATURDAY_SUNDAY,
    SourceDate, SubstituteDirection, SubstitutionPolicy, WeekendPolicy,
};

/// The Friday–Saturday weekend, as most of the Arab world keeps it.
static FRIDAY_SATURDAY: &[WeekendPolicy] = &[WeekendPolicy {
    days: &[Weekday::Friday, Weekday::Saturday],
    valid_from: None,
    valid_until: None,
}];

const EID_AL_FITR: Rule = Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 1);
const EID_AL_ADHA: Rule = Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 10);
const HIJRI_NEW_YEAR: Rule = Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 1, 1);
const MAWLID: Rule = Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 3, 12);

// ─────────────────────────────────────────────────────────────────────────
// Israel
// ─────────────────────────────────────────────────────────────────────────

/// Yom HaAtzmaut, Israel's Independence Day.
///
/// The Independence Day Law puts it on 5 Iyar, and then moves it so that the
/// preceding Yom HaZikaron never abuts the Sabbath: 5 Iyar on a Friday or a
/// Saturday is kept the Thursday before, and — since the 2004 amendment —
/// 5 Iyar on a Monday is kept the Tuesday after. Nothing in the rule
/// vocabulary expresses "move in one of two directions depending on the
/// weekday", so this is one of the genuine handful.
fn yom_haatzmaut(year: i64) -> Days {
    let occurrences = Rule::FixedInCalendar {
        system: CalendarSystem::HEBREW,
        month: Month::regular(8),
        day: 5,
    }
    .days_in_year(year);
    let mut out = Days::new();
    for date in occurrences.as_slice() {
        let adjusted = match Weekday::from_rd(*date) {
            Weekday::Friday => Rd(date.0 - 1),
            Weekday::Saturday => Rd(date.0 - 2),
            Weekday::Monday if year >= 2004 => Rd(date.0 + 1),
            _ => *date,
        };
        out.push(adjusted);
    }
    out
}

static IL_INDEPENDENCE: Rule = Rule::Computed(yom_haatzmaut);

static IL_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "Rosh Hashanah",
        "ראש השנה",
        Rule::in_calendar(CalendarSystem::HEBREW, 1, 1),
    ),
    HolidayRule::fixed_public(
        "Rosh Hashanah",
        "ראש השנה",
        Rule::in_calendar(CalendarSystem::HEBREW, 1, 2),
    ),
    HolidayRule::fixed_public(
        "Yom Kippur",
        "יום כיפור",
        Rule::in_calendar(CalendarSystem::HEBREW, 1, 10),
    ),
    HolidayRule::fixed_public(
        "Sukkot",
        "סוכות",
        Rule::in_calendar(CalendarSystem::HEBREW, 1, 15),
    ),
    HolidayRule::fixed_public(
        "Simchat Torah",
        "שמחת תורה",
        Rule::in_calendar(CalendarSystem::HEBREW, 1, 22),
    ),
    HolidayRule::observance(
        "Hanukkah",
        "חנוכה",
        Rule::in_calendar(CalendarSystem::HEBREW, 3, 25),
    ),
    HolidayRule::observance(
        "Purim",
        "פורים",
        Rule::in_calendar(CalendarSystem::HEBREW, 6, 14),
    ),
    HolidayRule::fixed_public(
        "Passover",
        "פסח",
        Rule::in_calendar(CalendarSystem::HEBREW, 7, 15),
    ),
    HolidayRule::fixed_public(
        "Seventh Day of Passover",
        "שביעי של פסח",
        Rule::in_calendar(CalendarSystem::HEBREW, 7, 21),
    ),
    HolidayRule::observance(
        "Yom HaZikaron",
        "יום הזיכרון",
        Rule::Offset {
            base: &IL_INDEPENDENCE,
            days: -1,
        },
    ),
    HolidayRule::fixed_public("Yom HaAtzmaut", "יום העצמאות", IL_INDEPENDENCE)
        .years(Some(1949), None),
    HolidayRule::fixed_public(
        "Shavuot",
        "שבועות",
        Rule::in_calendar(CalendarSystem::HEBREW, 9, 6),
    ),
    HolidayRule::observance(
        "Tisha B'Av",
        "תשעה באב",
        Rule::in_calendar(CalendarSystem::HEBREW, 11, 9),
    ),
];

/// Israel's weekly day of rest: the Sabbath, and in law nothing else.
///
/// The Hours of Work and Rest Law, 5711-1951, section 7(b)(1), puts the
/// Sabbath in every Jewish employee's weekly rest, and the Law and
/// Administration Ordinance, section 18A, makes the Sabbath and the festivals
/// the State's prescribed days of rest. Friday is a working day: section
/// 2(b) shortens the day before the weekly rest to seven hours, and the
/// Sunday-to-Thursday week many employers keep is agreement and custom, not
/// statute.
static IL_WEEKEND: &[WeekendPolicy] = &[WeekendPolicy {
    days: &[Weekday::Saturday],
    valid_from: None,
    valid_until: None,
}];

/// Israel.
pub static ISRAEL: RuleSet = RuleSet {
    code: "IL",
    english_name: "Israel",
    rules: IL_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: IL_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "חוק יום העצמאות, התש\"ט-1949 and its 2004 amendment; \
              פקודת סדרי השלטון והמשפט for the festival days and, in \
              section 18A, the Sabbath as the day of rest; חוק שעות עבודה \
              ומנוחה, התשי\"א-1951, sections 2(b) and 7, in the ILO NATLEX \
              English translation, retrieved 2026-09-23. Every date is \
              exact, because the Hebrew calendar is arithmetic. Holidays \
              begin at sunset on the preceding evening, which this crate \
              does not model: it names days, not evenings. Friday is a \
              working day in law, shortened to seven hours, so the weekend \
              here is Saturday alone",
};

// ─────────────────────────────────────────────────────────────────────────
// Saudi Arabia
// ─────────────────────────────────────────────────────────────────────────

/// Iran's weekly holiday is Friday, under article 17 of the Constitution;
/// Thursday is a half-day in most offices and is not a weekend day.
static IR_WEEKEND: &[WeekendPolicy] = &[WeekendPolicy {
    days: &[Weekday::Friday],
    valid_from: None,
    valid_until: None,
}];

/// A civil holiday dated in the Solar Hijri calendar as Iran keeps it —
/// exact, because the calendar is the astronomical one.
const fn ir_solar(name: &'static str, local: &'static str, month: u8, day: u8) -> HolidayRule {
    HolidayRule::fixed_public(
        name,
        local,
        Rule::in_calendar(CalendarSystem::SOLAR_HIJRI, month, day),
    )
}

/// A religious holiday dated in the lunar Hijri calendar: a prediction,
/// because Iran declares each month on its own sighting and the tabular
/// calendar here is an approximation of that.
const fn ir_hijri(name: &'static str, local: &'static str, month: u8, day: u8) -> HolidayRule {
    HolidayRule::fixed_public(
        name,
        local,
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, month, day),
    )
    .approximate()
}

/// 1 Rabíʿ al-awwal, the base of the last day of Safar.
static IR_RABI_AL_AWWAL: Rule = Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 3, 1);

static IR_RULES: &[HolidayRule] = &[
    ir_solar("Nowruz", "نوروز", 1, 1),
    ir_solar("Nowruz", "نوروز", 1, 2),
    ir_solar("Nowruz", "نوروز", 1, 3),
    ir_solar("Nowruz", "نوروز", 1, 4),
    ir_solar("Islamic Republic Day", "روز جمهوری اسلامی", 1, 12),
    ir_solar("Nature Day (Sizdah Bedar)", "روز طبیعت", 1, 13),
    ir_solar("Demise of Imam Khomeini", "رحلت امام خمینی", 3, 14),
    ir_solar("15 Khordad Uprising", "قیام ۱۵ خرداد", 3, 15),
    ir_solar(
        "Victory of the Islamic Revolution",
        "پیروزی انقلاب اسلامی",
        11,
        22,
    ),
    ir_solar(
        "Nationalisation of the Oil Industry",
        "ملی‌شدن صنعت نفت",
        12,
        29,
    ),
    ir_hijri("Tasu'a", "تاسوعای حسینی", 1, 9),
    ir_hijri("Ashura", "عاشورای حسینی", 1, 10),
    ir_hijri("Arba'een", "اربعین حسینی", 2, 20),
    ir_hijri(
        "Demise of the Prophet and Martyrdom of Imam Hasan",
        "رحلت رسول اکرم و شهادت امام حسن مجتبی",
        2,
        28,
    ),
    // The last day of Safar, which the tabular calendar gives 29 days and
    // a sighted one sometimes 30.
    HolidayRule::fixed_public(
        "Martyrdom of Imam Reza",
        "شهادت امام رضا",
        Rule::Offset {
            base: &IR_RABI_AL_AWWAL,
            days: -1,
        },
    )
    .approximate(),
    ir_hijri(
        "Martyrdom of Imam Hasan al-Askari",
        "شهادت امام حسن عسکری",
        3,
        8,
    ),
    ir_hijri(
        "Birth of the Prophet and Imam Ja'far al-Sadiq",
        "ولادت رسول اکرم و امام جعفر صادق",
        3,
        17,
    ),
    ir_hijri("Martyrdom of Fatima", "شهادت حضرت فاطمه زهرا", 6, 3),
    ir_hijri("Birth of Imam Ali", "ولادت امام علی", 7, 13),
    ir_hijri("Mab'ath", "مبعث رسول اکرم", 7, 27),
    ir_hijri("Birth of Imam Mahdi", "ولادت حضرت قائم", 8, 15),
    ir_hijri("Martyrdom of Imam Ali", "شهادت امام علی", 9, 21),
    ir_hijri("Eid al-Fitr", "عید سعید فطر", 10, 1),
    ir_hijri("Eid al-Fitr", "عید سعید فطر", 10, 2),
    ir_hijri(
        "Martyrdom of Imam Ja'far al-Sadiq",
        "شهادت امام جعفر صادق",
        10,
        25,
    ),
    ir_hijri("Eid al-Adha", "عید سعید قربان", 12, 10),
    ir_hijri("Eid al-Ghadir", "عید سعید غدیر خم", 12, 18),
];

/// Iran.
///
/// Two calendars, and the table is honest about both. The civil
/// holidays are dated in the Solar Hijri calendar as Iran keeps it — the
/// astronomical `persian`, Nowruz on the equinox day or the day after
/// depending on noon in Tehran — so they are exact; the arithmetic cycle
/// would have put Nowruz 1404 a day early. The religious holidays are
/// dated in the lunar Hijri calendar, which Iran declares month by month on
/// its own sighting, so every one of them is flagged approximate.
///
/// No substitution: a holiday that falls on a Friday is simply a Friday.
pub static IRAN: RuleSet = RuleSet {
    code: "IR",
    english_name: "Iran",
    rules: IR_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: IR_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Wikipedia, \"Public holidays in Iran\" and \"تعطیلات رسمی \
              ایران\", retrieved 2026-09-22, for the list; the Constitution \
              of the Islamic Republic, article 17, for Friday; the Persian \
              names as the official calendar prints them. APPROXIMATE BY \
              NATURE for the lunar dates: Iran declares them on its own \
              sighting, and the tabular civil calendar here is a prediction",
};

/// Saudi Arabia moved its weekend from Thursday–Friday to Friday–Saturday
/// by royal decree in June 2013, to lose one fewer working day of overlap
/// with the rest of the world.
static SA_WEEKEND: &[WeekendPolicy] = &[
    WeekendPolicy {
        days: &[Weekday::Thursday, Weekday::Friday],
        valid_from: None,
        valid_until: Some(2012),
    },
    WeekendPolicy {
        days: &[Weekday::Friday, Weekday::Saturday],
        valid_from: Some(2013),
        valid_until: None,
    },
];

static SA_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("Founding Day", "يوم التأسيس", Rule::gregorian(2, 22))
        .years(Some(2022), None),
    // The Umm al-Qurā calendar is Saudi Arabia's own civil calendar, so its
    // holidays are dated in it. It remains a computation: the Eid dates are
    // proclaimed on a sighting.
    HolidayRule::fixed_public(
        "Eid al-Fitr",
        "عيد الفطر",
        Rule::in_calendar(CalendarSystem::ISLAMIC_UMM_AL_QURA, 9, 30),
    )
    .approximate(),
    HolidayRule::fixed_public(
        "Eid al-Fitr",
        "عيد الفطر",
        Rule::in_calendar(CalendarSystem::ISLAMIC_UMM_AL_QURA, 10, 1),
    )
    .approximate(),
    HolidayRule::fixed_public(
        "Eid al-Fitr",
        "عيد الفطر",
        Rule::in_calendar(CalendarSystem::ISLAMIC_UMM_AL_QURA, 10, 2),
    )
    .approximate(),
    HolidayRule::fixed_public(
        "Eid al-Fitr",
        "عيد الفطر",
        Rule::in_calendar(CalendarSystem::ISLAMIC_UMM_AL_QURA, 10, 3),
    )
    .approximate(),
    HolidayRule::fixed_public(
        "Day of Arafah",
        "يوم عرفة",
        Rule::in_calendar(CalendarSystem::ISLAMIC_UMM_AL_QURA, 12, 9),
    )
    .approximate(),
    HolidayRule::fixed_public(
        "Eid al-Adha",
        "عيد الأضحى",
        Rule::in_calendar(CalendarSystem::ISLAMIC_UMM_AL_QURA, 12, 10),
    )
    .approximate(),
    HolidayRule::fixed_public(
        "Eid al-Adha",
        "عيد الأضحى",
        Rule::in_calendar(CalendarSystem::ISLAMIC_UMM_AL_QURA, 12, 11),
    )
    .approximate(),
    HolidayRule::fixed_public(
        "Eid al-Adha",
        "عيد الأضحى",
        Rule::in_calendar(CalendarSystem::ISLAMIC_UMM_AL_QURA, 12, 12),
    )
    .approximate(),
    HolidayRule::fixed_public("National Day", "اليوم الوطني", Rule::gregorian(9, 23))
        .years(Some(2005), None),
];

/// Saudi Arabia.
pub static SAUDI_ARABIA: RuleSet = RuleSet {
    code: "SA",
    english_name: "Saudi Arabia",
    rules: SA_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SA_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Royal decrees; Umm al-Qura gazette; the June 2013 royal order \
              moving the weekend. APPROXIMATE BY NATURE: the Eid holidays \
              are announced by royal decree each year, usually longer than \
              the statutory span given here, and their start depends on a \
              crescent sighting. The Umm al-Qurā table covers 1300–1600 AH \
              only, so this calendar answers for roughly 1882–2174 CE",
};

// ─────────────────────────────────────────────────────────────────────────
// United Arab Emirates
// ─────────────────────────────────────────────────────────────────────────

/// The Emirates moved to a Saturday–Sunday weekend, with a half-day Friday,
/// on 1 January 2022 — the first Gulf state to do so.
static AE_WEEKEND: &[WeekendPolicy] = &[
    WeekendPolicy {
        days: &[Weekday::Friday, Weekday::Saturday],
        valid_from: None,
        valid_until: Some(2021),
    },
    WeekendPolicy {
        days: &[Weekday::Saturday, Weekday::Sunday],
        valid_from: Some(2022),
        valid_until: None,
    },
];

static AE_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "New Year's Day",
        "رأس السنة الميلادية",
        Rule::gregorian(1, 1),
    ),
    // The Cabinet circular states Eid al-Fitr as "29 Ramadan to 3 Shawwal",
    // so both possible last days of Ramadan are listed; in a 29-day Ramadan
    // the second simply does not exist.
    HolidayRule::fixed_public(
        "Eid al-Fitr",
        "عيد الفطر",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 9, 29),
    )
    .approximate(),
    HolidayRule::fixed_public(
        "Eid al-Fitr",
        "عيد الفطر",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 9, 30),
    )
    .approximate(),
    HolidayRule::fixed_public("Eid al-Fitr", "عيد الفطر", EID_AL_FITR).approximate(),
    HolidayRule::fixed_public(
        "Eid al-Fitr",
        "عيد الفطر",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 2),
    )
    .approximate(),
    HolidayRule::fixed_public(
        "Eid al-Fitr",
        "عيد الفطر",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 3),
    )
    .approximate(),
    HolidayRule::fixed_public(
        "Day of Arafah",
        "يوم عرفة",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 9),
    )
    .approximate(),
    HolidayRule::fixed_public("Eid al-Adha", "عيد الأضحى", EID_AL_ADHA).approximate(),
    HolidayRule::fixed_public(
        "Eid al-Adha",
        "عيد الأضحى",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 11),
    )
    .approximate(),
    HolidayRule::fixed_public(
        "Eid al-Adha",
        "عيد الأضحى",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 12),
    )
    .approximate(),
    HolidayRule::fixed_public("Islamic New Year", "رأس السنة الهجرية", HIJRI_NEW_YEAR)
        .approximate(),
    HolidayRule::fixed_public("Prophet's Birthday", "المولد النبوي", MAWLID).approximate(),
    HolidayRule::fixed_public("Commemoration Day", "يوم الشهيد", Rule::gregorian(11, 30))
        .years(Some(2015), Some(2018)),
    HolidayRule::fixed_public("Commemoration Day", "يوم الشهيد", Rule::gregorian(12, 1))
        .years(Some(2019), None),
    HolidayRule::fixed_public("National Day", "اليوم الوطني", Rule::gregorian(12, 2)),
    HolidayRule::fixed_public("National Day", "اليوم الوطني", Rule::gregorian(12, 3)),
];

/// The United Arab Emirates.
pub static UNITED_ARAB_EMIRATES: RuleSet = RuleSet {
    code: "AE",
    english_name: "United Arab Emirates",
    rules: AE_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: AE_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Cabinet Resolution 4-3 of 2019 and the yearly Cabinet circular \
              announcing the holiday table; the December 2021 Cabinet \
              decision on the working week. APPROXIMATE BY NATURE: every \
              Islamic date is announced by the Emirates Astronomy Society \
              and confirmed by the Cabinet, per year",
};

// ─────────────────────────────────────────────────────────────────────────
// Turkey
// ─────────────────────────────────────────────────────────────────────────

static TR_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Yılbaşı", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "National Sovereignty and Children's Day",
        "Ulusal Egemenlik ve Çocuk Bayramı",
        Rule::gregorian(4, 23),
    )
    .years(Some(1921), None),
    HolidayRule::fixed_public(
        "Labour and Solidarity Day",
        "Emek ve Dayanışma Günü",
        Rule::gregorian(5, 1),
    )
    .years(Some(2009), None),
    HolidayRule::fixed_public(
        "Commemoration of Atatürk, Youth and Sports Day",
        "Atatürk'ü Anma, Gençlik ve Spor Bayramı",
        Rule::gregorian(5, 19),
    ),
    HolidayRule::fixed_public("Ramazan Bayramı", "Ramazan Bayramı", EID_AL_FITR).approximate(),
    HolidayRule::fixed_public(
        "Ramazan Bayramı",
        "Ramazan Bayramı",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 2),
    )
    .approximate(),
    HolidayRule::fixed_public(
        "Ramazan Bayramı",
        "Ramazan Bayramı",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 3),
    )
    .approximate(),
    HolidayRule::fixed_public(
        "Democracy and National Unity Day",
        "Demokrasi ve Millî Birlik Günü",
        Rule::gregorian(7, 15),
    )
    .years(Some(2017), None),
    HolidayRule::fixed_public("Kurban Bayramı", "Kurban Bayramı", EID_AL_ADHA).approximate(),
    HolidayRule::fixed_public(
        "Kurban Bayramı",
        "Kurban Bayramı",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 11),
    )
    .approximate(),
    HolidayRule::fixed_public(
        "Kurban Bayramı",
        "Kurban Bayramı",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 12),
    )
    .approximate(),
    HolidayRule::fixed_public(
        "Kurban Bayramı",
        "Kurban Bayramı",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 13),
    )
    .approximate(),
    HolidayRule::fixed_public("Victory Day", "Zafer Bayramı", Rule::gregorian(8, 30)),
    HolidayRule::fixed_public(
        "Republic Day",
        "Cumhuriyet Bayramı",
        Rule::gregorian(10, 29),
    )
    .years(Some(1923), None),
];

/// Türkiye.
pub static TURKEY: RuleSet = RuleSet {
    code: "TR",
    english_name: "Türkiye",
    rules: TR_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "2429 sayılı Ulusal Bayram ve Genel Tatiller Hakkında Kanun. \
              Türkiye's Islamic dates come from the Diyanet's precomputed \
              calendar rather than from sighting, so they are firmer than \
              most; the tabular computation here can still differ by a day, \
              which is why they stay flagged approximate",
};

// ─────────────────────────────────────────────────────────────────────────
// Egypt
// ─────────────────────────────────────────────────────────────────────────

static EG_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "Coptic Christmas",
        "عيد الميلاد المجيد",
        Rule::gregorian(1, 7),
    ),
    HolidayRule::fixed_public(
        "Revolution Day",
        "عيد ثورة 25 يناير",
        Rule::gregorian(1, 25),
    )
    .years(Some(2012), None),
    HolidayRule::fixed_public(
        "Sinai Liberation Day",
        "عيد تحرير سيناء",
        Rule::gregorian(4, 25),
    )
    .years(Some(1982), None),
    // Sham El-Nessim is the day after Coptic Easter, which follows the
    // Julian computus.
    HolidayRule::fixed_public("Sham El-Nessim", "شم النسيم", Rule::paschal(EASTER_MONDAY)),
    HolidayRule::fixed_public("Labour Day", "عيد العمال", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public(
        "30 June Revolution",
        "عيد ثورة 30 يونيو",
        Rule::gregorian(6, 30),
    )
    .years(Some(2014), None),
    HolidayRule::fixed_public(
        "Revolution Day",
        "عيد ثورة 23 يوليو",
        Rule::gregorian(7, 23),
    ),
    HolidayRule::fixed_public(
        "Armed Forces Day",
        "عيد القوات المسلحة",
        Rule::gregorian(10, 6),
    ),
    HolidayRule::fixed_public("Eid al-Fitr", "عيد الفطر", EID_AL_FITR).approximate(),
    HolidayRule::fixed_public(
        "Eid al-Fitr",
        "عيد الفطر",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 2),
    )
    .approximate(),
    HolidayRule::fixed_public(
        "Eid al-Fitr",
        "عيد الفطر",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 3),
    )
    .approximate(),
    HolidayRule::fixed_public("Eid al-Adha", "عيد الأضحى", EID_AL_ADHA).approximate(),
    HolidayRule::fixed_public(
        "Eid al-Adha",
        "عيد الأضحى",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 11),
    )
    .approximate(),
    HolidayRule::fixed_public(
        "Eid al-Adha",
        "عيد الأضحى",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 12),
    )
    .approximate(),
    HolidayRule::fixed_public("Islamic New Year", "رأس السنة الهجرية", HIJRI_NEW_YEAR)
        .approximate(),
    HolidayRule::fixed_public("Prophet's Birthday", "المولد النبوي", MAWLID).approximate(),
];

/// Egypt.
pub static EGYPT: RuleSet = RuleSet {
    code: "EG",
    english_name: "Egypt",
    rules: EG_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: FRIDAY_SATURDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Labour Law 12 of 2003 and the Cabinet's annual holiday \
              decision. Egypt routinely moves a mid-week holiday to the \
              nearest Thursday by Cabinet decision, which is an annual act \
              and is not modelled",
};

// ─────────────────────────────────────────────────────────────────────────
// Nigeria and South Africa
// ─────────────────────────────────────────────────────────────────────────

static NG_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("Workers' Day", "", Rule::gregorian(5, 1)),
    HolidayRule::public("Democracy Day", "", Rule::gregorian(5, 29)).years(Some(2000), Some(2018)),
    HolidayRule::public("Democracy Day", "", Rule::gregorian(6, 12)).years(Some(2019), None),
    HolidayRule::public("Eid al-Fitr", "Eid-el-Fitr", EID_AL_FITR).approximate(),
    HolidayRule::public(
        "Eid al-Fitr",
        "Eid-el-Fitr",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 2),
    )
    .approximate(),
    HolidayRule::public("Eid al-Adha", "Eid-el-Kabir", EID_AL_ADHA).approximate(),
    HolidayRule::public(
        "Eid al-Adha",
        "Eid-el-Kabir",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 11),
    )
    .approximate(),
    HolidayRule::public("Independence Day", "", Rule::gregorian(10, 1)).years(Some(1960), None),
    HolidayRule::public("Mawlid", "Eid-el-Mawlid", MAWLID).approximate(),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
];

static SUNDAY_FORWARD: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// Nigeria.
pub static NIGERIA: RuleSet = RuleSet {
    code: "NG",
    english_name: "Nigeria",
    rules: NG_RULES,
    substitution: SUNDAY_FORWARD,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Public Holidays Act, Cap. P40, Laws of the Federation of \
              Nigeria 2004, and the Minister of Interior's annual \
              declaration. The Islamic dates are declared on sighting and \
              the declaration frequently adds a day",
};

static ZA_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public("Human Rights Day", "", Rule::gregorian(3, 21)).years(Some(1995), None),
    HolidayRule::public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public("Family Day", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("Freedom Day", "", Rule::gregorian(4, 27)).years(Some(1995), None),
    HolidayRule::public("Workers' Day", "", Rule::gregorian(5, 1)),
    HolidayRule::public("Youth Day", "", Rule::gregorian(6, 16)).years(Some(1995), None),
    HolidayRule::public("National Women's Day", "", Rule::gregorian(8, 9)).years(Some(1995), None),
    HolidayRule::public("Heritage Day", "", Rule::gregorian(9, 24)).years(Some(1995), None),
    HolidayRule::public("Day of Reconciliation", "", Rule::gregorian(12, 16))
        .years(Some(1995), None),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Day of Goodwill", "", Rule::gregorian(12, 26)),
];

/// South Africa.
pub static SOUTH_AFRICA: RuleSet = RuleSet {
    code: "ZA",
    english_name: "South Africa",
    rules: ZA_RULES,
    substitution: SUNDAY_FORWARD,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Public Holidays Act 36 of 1994, section 2(1) for the Sunday \
              rule. Days declared under section 2A — election days and the \
              occasional national day of mourning — are one-offs by \
              proclamation and are not modelled",
};

// ─────────────────────────────────────────────────────────────────────────
// Kenya
// ─────────────────────────────────────────────────────────────────────────

/// Section 4 of the Public Holidays Act: a Part I holiday on a Sunday
/// moves to "the first succeeding day, not being a public holiday", and the
/// Sunday ceases to be one.
static KE_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static KE_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("Labour Day", "", Rule::gregorian(5, 1)),
    HolidayRule::public("Madaraka Day", "", Rule::gregorian(6, 1)),
    HolidayRule::public("Idd-ul-Fitr", "", EID_AL_FITR).approximate(),
    // Utamaduni Day in the Act's 2022 text, Huduma Day and Moi Day before
    // that, and Mazingira Day in the source's current table; the renamings
    // are not dated by the sources and are not carried.
    HolidayRule::public("Mazingira Day", "", Rule::gregorian(10, 10)),
    HolidayRule::public("Mashujaa Day", "", Rule::gregorian(10, 20)),
    HolidayRule::public("Jamhuri Day", "", Rule::gregorian(12, 12)),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
    // Parts II and III of the Schedule: public holidays for all persons of
    // the Islamic and Hindu faiths respectively, and not for others.
    HolidayRule::observance("Idd-ul-Azha", "", EID_AL_ADHA)
        .of_kind(Kind::Religious)
        .approximate(),
    HolidayRule::observance("Diwali", "", DIWALI)
        .of_kind(Kind::Religious)
        .approximate(),
];

/// Kenya.
///
/// The Public Holidays Act (Cap. 110): Part I of its Schedule for everyone,
/// with section 4 moving a Sunday holiday to the next day that is not one;
/// Part II, Idd-ul-Azha, for all persons of the Islamic faith and Part III,
/// Diwali, for all of the Hindu faith, which are carried as religious
/// observances rather than days off for all. The two Idd days are "dates
/// depending upon the appearance of the moon" and are approximate; Diwali
/// is computed on the Hindu lunisolar calendar and flagged likewise, since
/// the Act says the moon decides. The election day and the swearing-in of
/// a President-elect, which section 2 makes holidays, and the Cabinet
/// Secretary's gazetted additions under section 3 are not carried.
pub static KENYA: RuleSet = RuleSet {
    code: "KE",
    english_name: "Kenya",
    rules: KE_RULES,
    substitution: KE_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "The Public Holidays Act (Cap. 110), Revised Edition 2022, National \
              Council for Law Reporting (new.kenyalaw.org, retrieved 2026-09-22), \
              sections 2 to 4 and the Schedule; Wikipedia, \"Public holidays in \
              Kenya\", retrieved 2026-09-22, for the present name of the \
              10 October holiday",
};

// ─────────────────────────────────────────────────────────────────────────
// Morocco
// ─────────────────────────────────────────────────────────────────────────

/// A religious feast of two days, as the decrees give the three great
/// feasts: the day and the day after.
const fn ma_two_days(
    name: &'static str,
    local: &'static str,
    base: &'static Rule,
) -> [HolidayRule; 2] {
    [
        HolidayRule::fixed_public(name, local, *base).approximate(),
        HolidayRule::fixed_public(name, local, Rule::Offset { base, days: 1 }).approximate(),
    ]
}

static MA_EID_AL_FITR: Rule = EID_AL_FITR;
static MA_EID_AL_ADHA: Rule = EID_AL_ADHA;
static MA_MAWLID: Rule = MAWLID;

static MA_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Nouvel An", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Proclamation of Independence Day",
        "Manifeste de l'indépendance",
        Rule::gregorian(1, 11),
    ),
    // Declared a national holiday on 3 May 2023, so kept from 2024.
    HolidayRule::fixed_public(
        "Amazigh New Year",
        "Nouvel An Amazigh",
        Rule::gregorian(1, 14),
    )
    .years(Some(2024), None),
    HolidayRule::fixed_public("Labour Day", "Fête du Travail", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Throne Day", "Fête du Trône", Rule::gregorian(7, 30)),
    HolidayRule::fixed_public(
        "Oued Ed-Dahab Allegiance Day",
        "Allégeance Oued Eddahab",
        Rule::gregorian(8, 14),
    ),
    HolidayRule::fixed_public(
        "Revolution of the King and the People",
        "Révolution du Roi et du Peuple",
        Rule::gregorian(8, 20),
    ),
    HolidayRule::fixed_public("Youth Day", "Fête de la Jeunesse", Rule::gregorian(8, 21)),
    // Established on 4 November 2025, so first kept in 2026.
    HolidayRule::fixed_public("Unity Day", "Fête de l'Unité", Rule::gregorian(10, 31))
        .years(Some(2026), None),
    HolidayRule::fixed_public("Green March Day", "Marche verte", Rule::gregorian(11, 6)),
    HolidayRule::fixed_public(
        "Independence Day",
        "Fête de l'indépendance",
        Rule::gregorian(11, 18),
    ),
    HolidayRule::fixed_public("Islamic New Year", "1er Moharram", HIJRI_NEW_YEAR).approximate(),
    ma_two_days("Mawlid", "Aïd al-Mawlid", &MA_MAWLID)[0],
    ma_two_days("Mawlid", "Aïd al-Mawlid", &MA_MAWLID)[1],
    ma_two_days("Eid al-Fitr", "Aïd al-Fitr", &MA_EID_AL_FITR)[0],
    ma_two_days("Eid al-Fitr", "Aïd al-Fitr", &MA_EID_AL_FITR)[1],
    ma_two_days("Eid al-Adha", "Aïd al-Adha", &MA_EID_AL_ADHA)[0],
    ma_two_days("Eid al-Adha", "Aïd al-Adha", &MA_EID_AL_ADHA)[1],
];

/// Morocco.
///
/// The eleven fixed days of the decrees on paid holidays and the four
/// religious feasts, three of them of two days — "les fêtes religieuses
/// donnent lieu à 2 jours fériés" — on the tabular Hijri calendar and
/// therefore approximate, since Morocco announces each on the sighting of
/// the moon. The Amazigh New Year is kept from 2024, having been declared
/// on 3 May 2023, and Unity Day from 2026, having been established on
/// 4 November 2025. The source says nothing of a holiday on the weekend,
/// and nothing is done with one.
pub static MOROCCO: RuleSet = RuleSet {
    code: "MA",
    english_name: "Morocco",
    rules: MA_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Décret n° 2-04-426 and décret n° 2-00-166 of 10 May 2000 amending \
              décret n° 2-77-169 of 28 February 1977, as cited by Wikipedia (fr), \
              \"Fêtes et jours fériés au Maroc\", retrieved 2026-09-22, which also \
              gives the royal decisions of 3 May 2023 (Yennayer) and 4 November \
              2025 (Fête de l'Unité)",
};

// ─────────────────────────────────────────────────────────────────────────
// Ethiopia
// ─────────────────────────────────────────────────────────────────────────

/// A holiday on a day of the Ethiopian calendar, which is where Ethiopia
/// keeps it: the Gregorian date the source prints moves by a day around
/// the six-day Pagumen, and the Ethiopian date does not.
const fn et_ethiopic(name: &'static str, local: &'static str, month: u8, day: u8) -> HolidayRule {
    HolidayRule::fixed_public(
        name,
        local,
        Rule::in_calendar(CalendarSystem::ETHIOPIC, month, day),
    )
}

static ET_RULES: &[HolidayRule] = &[
    // Tahsas 29: 7 January, and 8 January in a Gregorian leap year.
    et_ethiopic("Genna", "ገና", 4, 29),
    // Tirr 11: 19 January, and 20 January in a Gregorian leap year.
    et_ethiopic("Timkat", "ጥምቀት", 5, 11),
    // Yekatit 23: 2 March.
    et_ethiopic("Adwa Victory Day", "የዓድዋ ድል በዓል", 6, 23),
    HolidayRule::fixed_public("Good Friday", "ስቅለት", Rule::paschal(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Fasika", "ፋሲካ", Rule::paschal(EASTER_SUNDAY)),
    HolidayRule::fixed_public(
        "International Workers' Day",
        "ዓለም አቀፍ የሠራተኞች ቀን",
        Rule::gregorian(5, 1),
    ),
    // Miyazya 27: 5 May.
    et_ethiopic("Patriots' Victory Day", "የአርበኞች ቀን", 8, 27),
    // Ginbot 20: 28 May.
    et_ethiopic("Downfall of the Derg", "ደርግ የወደቀበት ቀን", 9, 20),
    // Mäskäräm 1: 11 September, and 12 September before a Gregorian leap
    // year.
    et_ethiopic("Enkutatash", "እንቁጣጣሽ", 1, 1),
    // Mäskäräm 17: 27 September, and 28 September before a Gregorian leap
    // year.
    et_ethiopic("Meskel", "መስቀል", 1, 17),
    HolidayRule::fixed_public("Mawlid", "", MAWLID).approximate(),
    HolidayRule::fixed_public("Eid al-Fitr", "", EID_AL_FITR).approximate(),
    HolidayRule::fixed_public("Eid al-Adha", "", EID_AL_ADHA).approximate(),
];

/// Ethiopia.
///
/// The national and Orthodox holidays on the Ethiopian calendar, which is
/// where they are kept: the source prints their Gregorian dates with the
/// leap-year alternatives, and the Ethiopian date is the one thing that
/// does not move. Good Friday and Fasika follow the Julian computus, as
/// the Ethiopian Orthodox Tewahedo Church does; the three Islamic holidays
/// are on the tabular Hijri calendar and approximate, Mawlid on 12 Rabi'
/// al-Awwal, the Sunni date the source gives first. The source says
/// nothing of a holiday on the weekend, and nothing is done with one.
pub static ETHIOPIA: RuleSet = RuleSet {
    code: "ET",
    english_name: "Ethiopia",
    rules: ET_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Wikipedia, \"Public holidays in Ethiopia\", retrieved 2026-09-22, for the \
              list, the Amharic names and the Gregorian dates with their leap-year \
              alternatives, which the Ethiopian dates here reproduce",
};

// ─────────────────────────────────────────────────────────────────────────
// Ghana
// ─────────────────────────────────────────────────────────────────────────

/// A holiday of the Public Holidays and Commemorative Days Act as it has
/// stood since the 2019 amendment, from which the table begins.
const fn gh(name: &'static str, rule: Rule) -> HolidayRule {
    HolidayRule::fixed_public(name, "", rule).years(Some(2019), None)
}

static GH_EID_AL_FITR: Rule = EID_AL_FITR;

static GH_RULES: &[HolidayRule] = &[
    gh("New Year's Day", Rule::gregorian(1, 1)),
    // First observed on 7 January 2019.
    gh("Constitution Day", Rule::gregorian(1, 7)),
    gh("Independence Day", Rule::gregorian(3, 6)),
    gh("Good Friday", Rule::easter(GOOD_FRIDAY)),
    gh("Easter Monday", Rule::easter(EASTER_MONDAY)),
    gh("May Day", Rule::gregorian(5, 1)),
    gh("Eid al-Fitr", EID_AL_FITR).approximate(),
    // The day after Eid al-Fitr, added by the 2025 amendment after that
    // year's Eid had passed, so first kept in 2026.
    gh(
        "Shaqq Day",
        Rule::Offset {
            base: &GH_EID_AL_FITR,
            days: 1,
        },
    )
    .approximate()
    .years(Some(2026), None),
    gh("Eid al-Adha", EID_AL_ADHA).approximate(),
    // A commemorative day, not a holiday, from 2019 to 2024; restored as a
    // holiday by the 2025 amendment, passed on 25 June 2025.
    gh("Republic Day", Rule::gregorian(7, 1)).years(Some(2025), None),
    // The 2019 amendment's Founders' Day, repealed in 2025.
    gh("Founders' Day", Rule::gregorian(8, 4)).years(Some(2019), Some(2024)),
    // 21 September: Kwame Nkrumah Memorial Day under the 2019 amendment,
    // Founders' Day again from 2025.
    gh("Kwame Nkrumah Memorial Day", Rule::gregorian(9, 21)).years(Some(2019), Some(2024)),
    gh("Founders' Day", Rule::gregorian(9, 21)).years(Some(2025), None),
    // The first Friday of December, since 1988.
    gh("Farmers' Day", Rule::nth(12, 1, Weekday::Friday)),
    gh("Christmas Day", Rule::gregorian(12, 25)),
    gh("Boxing Day", Rule::gregorian(12, 26)),
];

/// Ghana.
///
/// The Public Holidays and Commemorative Days Act as amended in 2019 and
/// 2025, from 2019, the first year the sources state: Constitution Day
/// from that year; Founders' Day on 4 August and Kwame Nkrumah Memorial
/// Day on 21 September from 2019 to 2024, with Republic Day a
/// commemorative day only; and from the amendment Parliament passed on
/// 25 June 2025, Republic Day a holiday again, 21 September Founders' Day
/// again, 4 August gone, and Shaqq Day, the day after Eid al-Fitr, new.
/// The Hijri days are on the tabular calendar and approximate. Nothing
/// before 2019 is stated.
///
/// A holiday on the weekend is not moved by the table: the source says
/// the following Monday "tends to be declared" one, and the 2025 amendment
/// lets the President move a Tuesday, Wednesday or Thursday holiday to the
/// Friday or the Monday — both declarations, made year by year, and not
/// carried.
pub static GHANA: RuleSet = RuleSet {
    code: "GH",
    english_name: "Ghana",
    rules: GH_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Wikipedia, \"Public holidays in Ghana\", retrieved 2026-09-22, for the \
              list, Constitution Day's first observance on 7 January 2019 and \
              Farmers' Day on the first Friday of December since 1988; Ghana News \
              Agency, \"Government proposes changes to public holidays\", June \
              2025 (gna.org.gh, retrieved 2026-09-22), for the 2019 list and the \
              2025 changes; Ghanaian Times, on Parliament's passage of the \
              amendment on 25 June 2025",
};

// ─────────────────────────────────────────────────────────────────────────
// Bahrain
// ─────────────────────────────────────────────────────────────────────────

/// A Hijri-dated holiday: the tabular calendar approximates the date the
/// sighting fixes.
const fn hijri(name: &'static str, local: &'static str, month: u8, day: u8) -> HolidayRule {
    HolidayRule::fixed_public(
        name,
        local,
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, month, day),
    )
    .approximate()
}

static BH_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "New Year's Day",
        "رأس السنة الميلادية",
        Rule::gregorian(1, 1),
    ),
    HolidayRule::fixed_public("Labour Day", "عيد العمال", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("National Day", "اليوم الوطني", Rule::gregorian(12, 16)),
    HolidayRule::fixed_public("National Day", "اليوم الوطني", Rule::gregorian(12, 17)),
    hijri("Hijri New Year", "رأس السنة الهجرية", 1, 1),
    hijri("Ashura", "عاشوراء", 1, 9),
    hijri("Ashura", "عاشوراء", 1, 10),
    hijri("Prophet's Birthday", "المولد النبوي", 3, 12),
    hijri("Eid al-Fitr", "عيد الفطر", 10, 1),
    hijri("Eid al-Fitr", "عيد الفطر", 10, 2),
    hijri("Eid al-Fitr", "عيد الفطر", 10, 3),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 10),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 11),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 12),
];

/// Bahrain.
///
/// The official holidays the Council of Ministers determines under
/// article 64 of the Labour Law for the Private Sector (Law 36 of 2012):
/// four national days and ten Hijri-dated ones, three of each Eid, the
/// two days of Ashura, the Hijri New Year and the Prophet's Birthday, the
/// Hijri dates on the tabular calendar as an approximation of the
/// sighting. Wikipedia's table adds Arafat Day; the 2026 list does not,
/// and it is not carried. The weekend is Friday and Saturday, and what a
/// holiday on it gives is the year's circular, not a rule, so nothing
/// moves; the Cabinet's Sports Day is not carried.
pub static BAHRAIN: RuleSet = RuleSet {
    code: "BH",
    english_name: "Bahrain",
    rules: BH_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: FRIDAY_SATURDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Law 36 of 2012, Labour Law for the Private Sector, art. 64, per the Labour \
              Market Regulatory Authority; HONO's 2026 list of Bahrain's fourteen public \
              holidays, retrieved 2026-09-22; Wikipedia and the Arabic Wikipedia, \
              \"Public holidays in Bahrain\" and \"قائمة العطل الرسمية في البحرين\", \
              retrieved the same day, for the names",
};

// ─────────────────────────────────────────────────────────────────────────
// Jordan
// ─────────────────────────────────────────────────────────────────────────

/// A day the decision of the committee on unifying the Christian feasts
/// gives Christian employees off.
const fn jo_christian(name: &'static str, local: &'static str, rule: Rule) -> HolidayRule {
    HolidayRule::observance(name, local, rule).of_kind(Kind::Religious)
}

static JO_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "New Year's Day",
        "رأس السنة الميلادية",
        Rule::gregorian(1, 1),
    ),
    HolidayRule::fixed_public("Labour Day", "عيد العمال العالمي", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public(
        "Independence Day",
        "عيد استقلال المملكة",
        Rule::gregorian(5, 25),
    ),
    HolidayRule::fixed_public(
        "Christmas Day",
        "عيد الميلاد المجيد",
        Rule::gregorian(12, 25),
    ),
    hijri("Hijri New Year", "رأس السنة الهجرية", 1, 1),
    hijri("Prophet's Birthday", "المولد النبوي الشريف", 3, 12),
    // Four days from 1 Shawwal, and five from the Day of Arafat.
    hijri("Eid al-Fitr", "عيد الفطر المبارك", 10, 1),
    hijri("Eid al-Fitr", "عيد الفطر المبارك", 10, 2),
    hijri("Eid al-Fitr", "عيد الفطر المبارك", 10, 3),
    hijri("Eid al-Fitr", "عيد الفطر المبارك", 10, 4),
    hijri("Eid al-Adha", "عيد الأضحى المبارك", 12, 9),
    hijri("Eid al-Adha", "عيد الأضحى المبارك", 12, 10),
    hijri("Eid al-Adha", "عيد الأضحى المبارك", 12, 11),
    hijri("Eid al-Adha", "عيد الأضحى المبارك", 12, 12),
    hijri("Eid al-Adha", "عيد الأضحى المبارك", 12, 13),
    // Christian employees, by the Eastern computus.
    jo_christian(
        "Christmas Day",
        "عيد الميلاد المجيد",
        Rule::gregorian(12, 26),
    ),
    jo_christian("Palm Sunday", "أحد الشعانين", Rule::paschal(PALM_SUNDAY)),
    jo_christian("Easter Sunday", "عيد الفصح", Rule::paschal(EASTER_SUNDAY)),
    jo_christian("Easter Monday", "عيد الفصح", Rule::paschal(EASTER_MONDAY)),
    // Working commemorations.
    HolidayRule::observance(
        "Isra and Mi'raj",
        "ذكرى الإسراء والمعراج",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 7, 27),
    )
    .approximate(),
    HolidayRule::observance(
        "King Abdullah II's Birthday",
        "ذكرى ميلاد الملك عبد الله الثاني",
        Rule::gregorian(1, 30),
    ),
    HolidayRule::observance(
        "Accession Day",
        "ذكرى جلوس الملك عبد الله الثاني",
        Rule::gregorian(6, 9),
    ),
    HolidayRule::observance(
        "Great Arab Revolt Day",
        "يوم الثورة العربية الكبرى",
        Rule::gregorian(6, 10),
    ),
    HolidayRule::observance("Army Day", "يوم الجيش", Rule::gregorian(6, 10)),
    HolidayRule::observance(
        "King Hussein's Birthday",
        "ذكرى ميلاد الملك الحسين بن طلال",
        Rule::gregorian(11, 14),
    ),
];

/// Jordan.
///
/// The official holidays as the Arabic Wikipedia lists them and the
/// Securities Depository Center's 2026 calendar confirms: four fixed
/// days, Christmas among them for all, and the Hijri-dated days on the
/// tabular calendar as an approximation of the sighting — Eid al-Fitr
/// four days from 1 Shawwal and Eid al-Adha five from the Day of Arafat,
/// as 2026's 20–23 March and 26–30 May show. The days the committee on
/// unifying the Christian feasts gives Christian employees, 26 December
/// and Palm Sunday, Easter Sunday and Easter Monday by the Eastern
/// computus, are [`Kind::Religious`]; the commemorations kept at work are
/// observances. The weekend is Friday and Saturday. The Government's
/// habit of moving a holiday to lengthen a weekend — 2026's Labour Day on
/// Thursday 30 April — is not carried.
pub static JORDAN: RuleSet = RuleSet {
    code: "JO",
    english_name: "Jordan",
    rules: JO_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: FRIDAY_SATURDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "The Arabic Wikipedia, \"العطل الرسمية في الأردن\", retrieved 2026-09-22, for \
              the list, the day counts, the working commemorations and the \
              Christian employees' days; the Securities Depository Center, \"أيام \
              العطل 2026\", retrieved the same day, for the 2026 dates; Wikipedia, \
              \"Public holidays in Jordan\", for the English names",
};

// ─────────────────────────────────────────────────────────────────────────
// Kuwait
// ─────────────────────────────────────────────────────────────────────────

static KW_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "New Year's Day",
        "رأس السنة الميلادية",
        Rule::gregorian(1, 1),
    ),
    HolidayRule::fixed_public("National Day", "العيد الوطني", Rule::gregorian(2, 25)),
    HolidayRule::fixed_public("Liberation Day", "عيد التحرير", Rule::gregorian(2, 26)),
    hijri("Hijri New Year", "رأس السنة الهجرية", 1, 1),
    hijri("Prophet's Birthday", "المولد النبوي", 3, 12),
    hijri("Isra and Mi'raj", "الإسراء والمعراج", 7, 27),
    hijri("Eid al-Fitr", "عيد الفطر", 10, 1),
    hijri("Eid al-Fitr", "عيد الفطر", 10, 2),
    hijri("Eid al-Fitr", "عيد الفطر", 10, 3),
    hijri("Day of Arafat", "يوم الوقوف بعرفة", 12, 9),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 10),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 11),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 12),
];

/// Kuwait.
///
/// Article 68 of the Private Sector Labour Law (Law 6 of 2010): the
/// thirteen paid official holidays, three of each Eid, the Day of Arafat,
/// Isra and Mi'raj, the Hijri New Year and the Prophet's Birthday on the
/// tabular calendar as an approximation of the sighting, and three fixed
/// days. The weekend is Friday and Saturday; the article compensates work
/// on a holiday, and the Civil Service Commission's extra days for the
/// public sector are not carried.
pub static KUWAIT: RuleSet = RuleSet {
    code: "KW",
    english_name: "Kuwait",
    rules: KW_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: FRIDAY_SATURDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Law 6 of 2010, the Private Sector Labour Law, art. 68, as published in \
              English by the Public Authority of Manpower and summarised by Kuwait \
              Up To Date, retrieved 2026-09-22; the Arabic Wikipedia, \"قائمة العطل \
              الرسمية في الكويت\", retrieved the same day, for the names",
};

// ─────────────────────────────────────────────────────────────────────────
// Lebanon
// ─────────────────────────────────────────────────────────────────────────

/// Decree 15215's rule for Labour Day alone: on a Sunday or another
/// holiday, the private sector closes the day after. Every other holiday
/// on a Sunday is not replaced, and is `fixed_public`.
static LB_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: true,
    valid_from: None,
    valid_until: None,
}];

static LB_GOOD_FRIDAY: Rule = Rule::easter(GOOD_FRIDAY);
static LB_ORTHODOX_GOOD_FRIDAY: Rule = Rule::paschal(GOOD_FRIDAY);

/// When the Catholic and the Orthodox Good Fridays fall on the same day,
/// the decree closes that Friday and the Saturday after it.
fn lb_holy_saturday(year: i64) -> Days {
    let catholic = LB_GOOD_FRIDAY.days_in_year(year);
    let orthodox = LB_ORTHODOX_GOOD_FRIDAY.days_in_year(year);
    match (catholic.as_slice().first(), orthodox.as_slice().first()) {
        (Some(a), Some(b)) if a == b => Days::one(Rd(a.0 + 1)),
        _ => Days::new(),
    }
}

static LB_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "New Year's Day",
        "رأس السنة الميلادية",
        Rule::gregorian(1, 1),
    ),
    HolidayRule::fixed_public(
        "Armenian Orthodox Christmas",
        "عيد الميلاد عند الطوائف الأرمنية الأرثوذكسية",
        Rule::gregorian(1, 6),
    )
    .years(Some(2003), None),
    HolidayRule::fixed_public("Saint Maron's Day", "عيد مار مارون", Rule::gregorian(2, 9)),
    HolidayRule::fixed_public(
        "Annunciation",
        "عيد بشارة السيدة مريم العذراء",
        Rule::gregorian(3, 25),
    ),
    HolidayRule::fixed_public(
        "Good Friday",
        "الجمعة العظيمة عند الطوائف الكاثوليكية",
        Rule::easter(GOOD_FRIDAY),
    ),
    HolidayRule::fixed_public(
        "Orthodox Good Friday",
        "الجمعة العظيمة عند الطوائف الأرثوذكسية",
        Rule::paschal(GOOD_FRIDAY),
    ),
    HolidayRule::fixed_public(
        "Holy Saturday",
        "سبت النور",
        Rule::Computed(lb_holy_saturday),
    ),
    HolidayRule::public("Labour Day", "عيد العمل", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public(
        "Assumption",
        "عيد انتقال السيدة العذراء",
        Rule::gregorian(8, 15),
    ),
    HolidayRule::fixed_public("Independence Day", "عيد الاستقلال", Rule::gregorian(11, 22)),
    HolidayRule::fixed_public("Christmas Day", "عيد الميلاد", Rule::gregorian(12, 25)),
    hijri("Hijri New Year", "رأس السنة الهجرية", 1, 1),
    hijri("Ashura", "ذكرى عاشوراء", 1, 10),
    hijri("Prophet's Birthday", "ذكرى المولد النبوي", 3, 12),
    hijri("Eid al-Fitr", "عيد الفطر", 10, 1),
    hijri("Eid al-Fitr", "عيد الفطر", 10, 2),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 10),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 11),
    // Commemorated on Sundays by the decree, and so no days off; and the
    // day the Council of Ministers declares year by year.
    HolidayRule::observance(
        "Martyrs' Day",
        "ذكرى الشهداء",
        Rule::nth(5, 1, Weekday::Sunday),
    ),
    HolidayRule::observance(
        "Resistance and Liberation Day",
        "عيد المقاومة والتحرير",
        Rule::nth(5, 2, Weekday::Sunday),
    ),
    HolidayRule::observance(
        "Rafic Hariri Memorial Day",
        "ذكرى استشهاد الرئيس رفيق الحريري",
        Rule::gregorian(2, 14),
    ),
];

/// Lebanon.
///
/// Decree 15215 of 27 September 2005 as amended, read from the Presidency
/// of the Council of Ministers' own table: the days on which the public
/// administration, the municipalities and the private sector close —
/// Good Friday by both the Gregorian and the Julian computus, Armenian
/// Christmas from 2003, and the Hijri-dated days on the tabular calendar
/// as an approximation of the sighting, two days of each Eid. The decree
/// replaces no holiday that falls on a Sunday, with two exceptions it
/// names and this table carries: Labour Day on a Sunday or another holiday
/// closes the private sector the day after, a forward policy that reaches
/// that rule alone, and the two Good Fridays on one day close the Saturday
/// after, a computed rule. Martyrs' Day and the Resistance and Liberation
/// Day are commemorated on the first and second Sundays of May and give no
/// day off; Rafic Hariri's day is declared year by year; all three are
/// observances. Easter Sundays are Sundays and the decree names the
/// Fridays.
pub static LEBANON: RuleSet = RuleSet {
    code: "LB",
    english_name: "Lebanon",
    rules: LB_RULES,
    substitution: LB_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Presidency of the Council of Ministers, \"الأعياد والعطل الرسمية\", \
              pcm.gov.lb, retrieved 2026-09-22, reproducing Decree 15215 of \
              27 September 2005 and its amendments; the Embassy of Lebanon in \
              Poland's 2026 list, retrieved the same day; Wikipedia, \"Public \
              holidays in Lebanon\", for the English names and Armenian \
              Christmas's 2003 start",
};

// ─────────────────────────────────────────────────────────────────────────
// Tanzania
// ─────────────────────────────────────────────────────────────────────────

/// Section 4 of the Public Holidays Act: a holiday on a Saturday or a
/// Sunday makes the next following day that is not itself a holiday a
/// holiday in its stead.
static TZ_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// A Hijri-dated holiday under a substitution policy.
const fn hijri_public(name: &'static str, local: &'static str, month: u8, day: u8) -> HolidayRule {
    HolidayRule::public(
        name,
        local,
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, month, day),
    )
    .approximate()
}

static TZ_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Mwaka Mpya", Rule::gregorian(1, 1)),
    HolidayRule::public(
        "Zanzibar Revolution Day",
        "Sikukuu ya Mapinduzi ya Zanzibar",
        Rule::gregorian(1, 12),
    ),
    hijri_public("Eid al-Fitr", "Idd el Fitri", 10, 1),
    hijri_public("Eid al-Fitr", "Idd el Fitri", 10, 2),
    HolidayRule::public("Good Friday", "Ijumaa Kuu", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public(
        "Easter Monday",
        "Jumatatu ya Pasaka",
        Rule::easter(EASTER_MONDAY),
    ),
    // Declared by the President under section 3 each year, and kept every year.
    HolidayRule::public("Karume Day", "Siku ya Karume", Rule::gregorian(4, 7)),
    HolidayRule::public("Union Day", "Sikukuu ya Muungano", Rule::gregorian(4, 26)),
    HolidayRule::public(
        "International Workers' Day",
        "Sikukuu ya Wafanyakazi",
        Rule::gregorian(5, 1),
    ),
    hijri_public("Eid al-Adha", "Idd el Haji", 12, 10),
    HolidayRule::public("Saba Saba Day", "Saba Saba", Rule::gregorian(7, 7)),
    HolidayRule::public("Nane Nane Day", "Nane Nane", Rule::gregorian(8, 8)),
    hijri_public("Maulid", "Maulidi", 3, 12),
    HolidayRule::public("Nyerere Day", "Siku ya Nyerere", Rule::gregorian(10, 14)),
    HolidayRule::public(
        "Independence and Republic Day",
        "Siku ya Uhuru",
        Rule::gregorian(12, 9),
    ),
    HolidayRule::public("Christmas Day", "Krismasi", Rule::gregorian(12, 25)),
    HolidayRule::public(
        "Boxing Day",
        "Siku ya Kufungua Zawadi",
        Rule::gregorian(12, 26),
    ),
];

/// Tanzania.
///
/// The Public Holidays Act (Cap. 35) as revised: the Schedule's days,
/// two of Eid al-Fitr and one each of Eid al-Adha and Maulid on the
/// tabular calendar as an approximation of the sighting, and section 4,
/// which makes the next free day a holiday when one falls on a Saturday
/// or a Sunday, as a forward policy. Karume Day and Saba Saba are not in
/// the Schedule; the President declares them under section 3 every year,
/// and they are carried on that footing. The Peasants' Day of the
/// Schedule goes by its everyday name, Nane Nane.
pub static TANZANIA: RuleSet = RuleSet {
    code: "TZ",
    english_name: "Tanzania",
    rules: TZ_RULES,
    substitution: TZ_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Public Holidays Act, Cap. 35, sections 2 to 4 and the Schedule, as \
              reproduced by tanzanialaws.com, retrieved 2026-09-22; Wikipedia, \
              \"Public holidays in Tanzania\", retrieved the same day, for the Swahili \
              names and the presidential days; sikukuu.co.tz for the 2026 list",
};

// ─────────────────────────────────────────────────────────────────────────
// Uganda
// ─────────────────────────────────────────────────────────────────────────

static UG_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("NRM Liberation Day", "", Rule::gregorian(1, 26)),
    HolidayRule::fixed_public("Archbishop Janani Luwum Day", "", Rule::gregorian(2, 16))
        .years(Some(2016), None),
    HolidayRule::fixed_public("International Women's Day", "", Rule::gregorian(3, 8)),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::fixed_public("Labour Day", "", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Uganda Martyrs' Day", "", Rule::gregorian(6, 3)),
    HolidayRule::fixed_public("National Heroes' Day", "", Rule::gregorian(6, 9))
        .years(Some(2001), None),
    HolidayRule::fixed_public("Independence Day", "", Rule::gregorian(10, 9)),
    HolidayRule::fixed_public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::fixed_public("Boxing Day", "", Rule::gregorian(12, 26)),
    hijri("Eid al-Fitr", "Idd el Fitr", 10, 1),
    hijri("Eid al-Adha", "Idd Adhuha", 12, 10),
];

/// Uganda.
///
/// The Public Holidays Act (Cap. 255) as the consulate's 2026 list and
/// Wikipedia reproduce it: twelve fixed and Easter days, Archbishop
/// Janani Luwum Day from 2016 and National Heroes' Day from 2001, and one
/// day of each Eid on the tabular calendar as an approximation of the
/// Uganda Muslim Supreme Council's sighting. A holiday on a weekend gets
/// whatever substitute the Office of the President designates, which is
/// no rule and is not carried; nor are the election days it declares.
pub static UGANDA: RuleSet = RuleSet {
    code: "UG",
    english_name: "Uganda",
    rules: UG_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Consulate of the Republic of Uganda in Arusha, \"Public Holidays\", \
              retrieved 2026-09-22, for the 2026 list and the Eids' single days; \
              Wikipedia, \"Public holidays in Uganda\" and \"Archbishop Janani Luwum \
              Day\", retrieved the same day, for the list and the years",
};

// ─────────────────────────────────────────────────────────────────────────
// Zambia
// ─────────────────────────────────────────────────────────────────────────

/// The Public Holidays Act (Cap. 272): a holiday on a Sunday is observed
/// on the following Monday.
static ZM_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static ZM_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public("International Women's Day", "", Rule::gregorian(3, 8))
        .years(Some(2008), None),
    HolidayRule::public("Youth Day", "", Rule::gregorian(3, 12)),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Holy Saturday", "", Rule::easter(HOLY_SATURDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("Kenneth Kaunda Day", "", Rule::gregorian(4, 28)).years(Some(2022), None),
    HolidayRule::public("Labour Day", "", Rule::gregorian(5, 1)),
    HolidayRule::public("African Freedom Day", "", Rule::gregorian(5, 25)),
    HolidayRule::fixed_public("Heroes' Day", "", Rule::nth(7, 1, Weekday::Monday)),
    HolidayRule::fixed_public("Unity Day", "", Rule::nth(7, 1, Weekday::Tuesday)),
    HolidayRule::fixed_public("Farmers' Day", "", Rule::nth(8, 1, Weekday::Monday)),
    HolidayRule::public(
        "National Day of Prayer, Fasting, Repentance and Reconciliation",
        "",
        Rule::gregorian(10, 18),
    )
    .years(Some(2015), None),
    HolidayRule::public("Independence Day", "", Rule::gregorian(10, 24)),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
];

/// Zambia.
///
/// The Public Holidays Act (Cap. 272) of 1964 with the days declared
/// under it: International Women's Day from 2008 (Statutory Instrument 33
/// of 2007), the National Day of Prayer from 2015 (Statutory Instrument
/// 78 of 2015) and Kenneth Kaunda Day from 2022. Heroes' Day is the first
/// Monday of July and Unity Day the Tuesday after it, Farmers' Day the
/// first Monday of August. The Act moves a Sunday holiday to the Monday
/// after, a forward policy that the weekday-fixed days and the Easter days
/// never need. Easter Monday is on every official list and is carried;
/// Easter Sunday, being a Sunday, is not. A Saturday holiday stays put.
pub static ZAMBIA: RuleSet = RuleSet {
    code: "ZM",
    english_name: "Zambia",
    rules: ZM_RULES,
    substitution: ZM_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Public Holidays Act, Cap. 272, and its subsidiary legislation as \
              summarised by zambialaws.com and ZambiaLII, retrieved 2026-09-22; \
              Wikipedia, \"Public holidays in Zambia\" and \"National Day of Prayer, \
              Fasting, Repentance and Reconciliation (Zambia)\", retrieved the same \
              day; Lusaka Times, 28 April 2022, for Kenneth Kaunda Day's first \
              observance; HONO's 2026 list for the in-lieu Mondays",
};

// ─────────────────────────────────────────────────────────────────────────
// Zimbabwe
// ─────────────────────────────────────────────────────────────────────────

/// Section 2 of the Public Holidays and Prohibition of Business Act
/// (Chapter 10:21): a public holiday on a Sunday makes the Monday
/// following a public holiday, past one already taken — Christmas 2022's
/// Tuesday.
static ZW_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static ZW_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public(
        "Robert Gabriel Mugabe National Youth Day",
        "",
        Rule::gregorian(2, 21),
    )
    .years(Some(2018), None),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Saturday", "", Rule::easter(HOLY_SATURDAY)),
    HolidayRule::fixed_public("Easter Sunday", "", Rule::easter(EASTER_SUNDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("Independence Day", "", Rule::gregorian(4, 18)),
    HolidayRule::public("Workers' Day", "", Rule::gregorian(5, 1)),
    HolidayRule::public("Africa Day", "", Rule::gregorian(5, 25)),
    HolidayRule::fixed_public("Heroes' Day", "", Rule::nth(8, 2, Weekday::Monday)),
    HolidayRule::fixed_public(
        "Defence Forces National Day",
        "",
        Rule::nth(8, 2, Weekday::Tuesday),
    ),
    HolidayRule::public("National Unity Day", "", Rule::gregorian(12, 22)),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
];

/// Zimbabwe.
///
/// The Public Holidays and Prohibition of Business Act (Chapter 10:21) as
/// General Notice 1361 of 2025 lists it for 2026: fourteen days, the four
/// Easter days as a block, Heroes' Day on the second Monday of August and
/// the Defence Forces on the Tuesday after, and Robert Gabriel Mugabe
/// National Youth Day from 2018 by the statutory instrument of November
/// 2017. Section 2's proviso moves a Sunday holiday to the Monday after,
/// a forward policy that skips a Monday already taken; the Easter days
/// are `fixed_public` so that Easter Sunday claims no Tuesday. Wikipedia's
/// Munhumutapa Day is not in the 2026 notice and is not carried; the
/// President's extra days under section 2(2) are not either.
pub static ZIMBABWE: RuleSet = RuleSet {
    code: "ZW",
    english_name: "Zimbabwe",
    rules: ZW_RULES,
    substitution: ZW_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "General Notice 1361 of 2025 under the Public Holidays and Prohibition of \
              Business Act, Chapter 10:21, as reproduced by SmartHR Solutions \
              Zimbabwe, retrieved 2026-09-22, for the 2026 list; ZimLII and Veritas \
              Zimbabwe for section 2's Sunday proviso; Wikipedia, \"Robert Gabriel \
              Mugabe National Youth Day\", retrieved the same day, for 2018",
};

// ─────────────────────────────────────────────────────────────────────────
// Algeria
// ─────────────────────────────────────────────────────────────────────────

/// Saturday–Sunday until the ordinances of 1976 made it Thursday–Friday,
/// and Friday–Saturday from 14 August 2009.
static DZ_WEEKEND: &[WeekendPolicy] = &[
    WeekendPolicy {
        days: &[Weekday::Saturday, Weekday::Sunday],
        valid_from: None,
        valid_until: Some(1975),
    },
    WeekendPolicy {
        days: &[Weekday::Thursday, Weekday::Friday],
        valid_from: Some(1976),
        valid_until: Some(2008),
    },
    WeekendPolicy {
        days: &[Weekday::Friday, Weekday::Saturday],
        valid_from: Some(2009),
        valid_until: None,
    },
];

/// A day article 3 or 4 of the law grants to the Christian or the Jewish
/// community: a religious day, not a day off for everyone.
const fn dz_community(name: &'static str, local: &'static str, rule: Rule) -> HolidayRule {
    HolidayRule::observance(name, local, rule).of_kind(Kind::Religious)
}

static DZ_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Jour de l'an", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("Yennayer", "Yennayer", Rule::gregorian(1, 12))
        .years(Some(2018), None),
    HolidayRule::fixed_public("Labour Day", "Fête des travailleurs", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public(
        "Independence Day",
        "Fête de l'indépendance",
        Rule::gregorian(7, 5),
    )
    .years(Some(1962), None),
    HolidayRule::fixed_public(
        "Revolution Day",
        "Fête de la Révolution",
        Rule::gregorian(11, 1),
    ),
    hijri("Islamic New Year", "Awal Mouharram", 1, 1),
    hijri("Ashura", "Achoura", 1, 10),
    hijri("Mawlid", "Mouloud", 3, 12),
    hijri("Eid al-Fitr", "Aïd el-Fitr", 10, 1),
    hijri("Eid al-Fitr", "Aïd el-Fitr", 10, 2),
    hijri("Eid al-Fitr", "Aïd el-Fitr", 10, 3).years(Some(2023), None),
    hijri("Eid al-Adha", "Aïd el-Adha", 12, 10),
    hijri("Eid al-Adha", "Aïd el-Adha", 12, 11),
    hijri("Eid al-Adha", "Aïd el-Adha", 12, 12).years(Some(2023), None),
    dz_community(
        "Easter Monday",
        "Lundi de Pâques",
        Rule::easter(EASTER_MONDAY),
    ),
    dz_community("Ascension", "Ascension", Rule::easter(ASCENSION)),
    dz_community(
        "Whit Monday",
        "Lundi de Pentecôte",
        Rule::easter(WHIT_MONDAY),
    ),
    dz_community("Assumption", "Assomption", Rule::gregorian(8, 15)),
    dz_community("Christmas Day", "Noël", Rule::gregorian(12, 25)),
    dz_community(
        "Rosh Hashanah",
        "Roch Hachana",
        Rule::in_calendar(CalendarSystem::HEBREW, 1, 1),
    ),
    dz_community(
        "Yom Kippur",
        "Yom Kippour",
        Rule::in_calendar(CalendarSystem::HEBREW, 1, 10),
    ),
    dz_community(
        "Passover",
        "Pessah",
        Rule::in_calendar(CalendarSystem::HEBREW, 7, 15),
    ),
];

/// Algeria.
///
/// Law 63-278 of 26 July 1963 fixing the list of legal holidays, as
/// amended: five civil days and, on the tabular Hijri calendar as
/// approximations of the sighted dates, Awal Mouharram, Achoura, Mouloud
/// and the two Eids — two days each until 2022, as the Government's
/// notices for 2019 and 2022 gave them, and "trois (3) jours" each by
/// law 23-10 of 26 June 2023, first kept for the Aïd el-Adha of that
/// month. Yennayer, the Amazigh new year, was added by law 18-12 of
/// 2 July 2018 and first kept on 12 January 2018. Articles 3 and 4 grant
/// the Christian community Easter Monday, Ascension, Whit Monday, the
/// Assumption and Christmas, and the Jewish community Rosh Hashanah, Yom
/// Kippur and Passover; those are religious days here, not days off for
/// all. The weekend was Saturday–Sunday until 1976, Thursday–Friday to
/// 2009 and Friday–Saturday from 14 August 2009. The law has no rule for
/// a holiday on the weekend, and nothing moves.
pub static ALGERIA: RuleSet = RuleSet {
    code: "DZ",
    english_name: "Algeria",
    rules: DZ_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: DZ_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Law 23-10 of 26 June 2023 amending law 63-278 of 26 July 1963 fixing the list \
              of legal holidays, Journal officiel no. 43 of 27 June 2023; legal-doctrine.com, \
              \"Les jours fériés en Algérie\", for articles 1, 3 and 4 of law 63-278 and \
              law 18-12 of 2 July 2018; APS and algerie-eco.com on the two Aïd el-Adha \
              days of 2019 and 2022; Wikipedia (fr), \"Fêtes et jours fériés en Algérie\" \
              and \"Yennayer\", and France 24 (22 July 2009) for the weekend",
};

// ─────────────────────────────────────────────────────────────────────────
// Tunisia
// ─────────────────────────────────────────────────────────────────────────

static TN_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "New Year's Day",
        "رأس السنة الميلادية",
        Rule::gregorian(1, 1),
    )
    .years(Some(1961), None),
    HolidayRule::fixed_public(
        "Revolution and Youth Day",
        "عيد الثورة والشباب",
        Rule::gregorian(1, 14),
    )
    .years(Some(2012), Some(2021)),
    HolidayRule::fixed_public("Revolution Day", "عيد الثورة", Rule::gregorian(1, 18))
        .years(Some(1961), Some(1987)),
    HolidayRule::fixed_public("Independence Day", "عيد الإستقلال", Rule::gregorian(3, 20))
        .years(Some(1961), None),
    HolidayRule::fixed_public("Youth Day", "عيد الشباب", Rule::gregorian(3, 21))
        .years(Some(1988), Some(2010)),
    HolidayRule::fixed_public("Martyrs' Day", "عيد الشهداء", Rule::gregorian(4, 9))
        .years(Some(1961), None),
    HolidayRule::fixed_public("Labour Day", "عيد الشغل", Rule::gregorian(5, 1))
        .years(Some(1961), None),
    HolidayRule::fixed_public("Victory Day", "عيد النصر", Rule::gregorian(6, 1))
        .years(Some(1961), Some(1987)),
    HolidayRule::fixed_public("Republic Day", "عيد الجمهورية", Rule::gregorian(7, 25))
        .years(Some(1961), None),
    HolidayRule::fixed_public(
        "President Bourguiba's Birthday",
        "عيد الزعيم",
        Rule::gregorian(8, 3),
    )
    .years(Some(1961), Some(1987)),
    HolidayRule::fixed_public("Women's Day", "عيد المرأة", Rule::gregorian(8, 13))
        .years(Some(1966), None),
    HolidayRule::fixed_public(
        "Commemoration of 3 September 1934",
        "",
        Rule::gregorian(9, 3),
    )
    .years(Some(1965), Some(1987)),
    HolidayRule::fixed_public("Evacuation Day", "عيد الجلاء", Rule::gregorian(10, 15))
        .years(Some(1964), None),
    HolidayRule::fixed_public(
        "Commemoration of 7 November 1987",
        "عيد التحول المبارك",
        Rule::gregorian(11, 7),
    )
    .years(Some(1990), Some(2010)),
    HolidayRule::fixed_public("Revolution Day", "عيد الثورة", Rule::gregorian(12, 17))
        .years(Some(2021), None),
    hijri("Islamic New Year", "رأس العام الهجري", 1, 1).years(Some(1961), None),
    hijri("Ashura", "عاشوراء", 1, 10).years(Some(1961), Some(1965)),
    hijri("Mouled", "المولد", 3, 12).years(Some(1961), None),
    hijri("Eid al-Fitr", "العيد الصغير", 10, 1).years(Some(1961), None),
    hijri("Eid al-Fitr", "العيد الصغير", 10, 2).years(Some(1961), None),
    hijri("Eid al-Fitr", "العيد الصغير", 10, 3).years(Some(1961), None),
    hijri("Eid al-Adha", "العيد الكبير", 12, 10).years(Some(1961), None),
    hijri("Eid al-Adha", "العيد الكبير", 12, 11).years(Some(1961), None),
];

/// Tunisia.
///
/// Presidential decree 2021-223 of 7 December 2021 fixing the holidays
/// that give leave to the personnel of the State, local authorities and
/// administrative public establishments, article 1: eight civil days of
/// one day each, the Hijri new year and the Mouled of one day, "Aïd el
/// fitr : trois jours" and "Aïd el idha : deux jours", the Hijri ones on
/// the tabular calendar as approximations. The private sector's holidays
/// are the Labour Code's and the collective agreements', not read here.
/// The chronology is Wikipedia's from the decrees it cites: the list of
/// 30 March 1961, with Achoura until 1965 and 18 January, 1 June and
/// 3 August until 1987; Evacuation Day from 1964; Women's Day from 1966
/// and 3 September over 1965–1987 by the decree of 30 August 1965; Youth
/// Day on 21 March from 1988 and 7 November from 1990, both last kept in
/// 2010; 14 January from 2012 to 2021; and 17 December from 2021. The
/// three Aïd el-Fitr days are the 2021 decree's and carry no first year.
/// No decree read says anything of a holiday on the weekend.
pub static TUNISIA: RuleSet = RuleSet {
    code: "TN",
    english_name: "Tunisia",
    rules: TN_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Décret présidentiel n° 2021-223 du 7 décembre 2021 (JORT 2021-113), article 1, \
              as legislation-securite.tn and jurisitetunisie.com publish it, retrieved \
              2026-09-22; Wikipedia (fr), \"Fêtes et jours fériés en Tunisie\", for the \
              Arabic names and the chronology of decrees 61-144, 64-13, 65-410, 87-1447, \
              90-1826 and 2011-317; Kapitalis on the 2021 decree",
};

// ─────────────────────────────────────────────────────────────────────────
// Senegal
// ─────────────────────────────────────────────────────────────────────────

/// Article 2 of law 74-52: "when Korité and Tabaski fall on a Sunday, the
/// following Monday is a holiday". Only those two rules substitute.
static SN_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: false,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static SN_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Jour de l'an", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Independence Day",
        "Fête de l'Indépendance",
        Rule::gregorian(4, 4),
    ),
    HolidayRule::fixed_public("Labour Day", "Fête du Travail", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Easter Sunday", "Pâques", Rule::easter(EASTER_SUNDAY)),
    HolidayRule::fixed_public(
        "Easter Monday",
        "Lundi de Pâques",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::fixed_public("Ascension", "Jeudi de l'Ascension", Rule::easter(ASCENSION)),
    HolidayRule::fixed_public("Pentecost", "Pentecôte", Rule::easter(PENTECOST)),
    HolidayRule::fixed_public(
        "Whit Monday",
        "Lundi de Pentecôte",
        Rule::easter(WHIT_MONDAY),
    ),
    HolidayRule::fixed_public("Assumption", "Assomption", Rule::gregorian(8, 15)),
    HolidayRule::fixed_public("All Saints' Day", "Toussaint", Rule::gregorian(11, 1)),
    HolidayRule::fixed_public("Christmas Day", "Noël", Rule::gregorian(12, 25)),
    hijri("Tamkharit", "Tamxarit", 1, 10),
    hijri("Grand Magal of Touba", "Grand Magal de Touba", 2, 18).years(Some(2012), None),
    hijri("Maouloud", "Maouloud", 3, 12),
    hijri_public("Korité", "Korité", 10, 1),
    hijri_public("Tabaski", "Tabaski", 12, 10),
];

/// Senegal.
///
/// Law 74-52 of 4 November 1974 on the national holiday and the legal
/// holidays, as amended in 1983 and 1989, from the Ministry of Labour's
/// collection: 4 April as the national holiday; "besides the feasts of
/// Easter and Pentecost falling on a Sunday", which are carried on their
/// Sundays, twelve legal holidays; and "when Korité and Tabaski fall on a
/// Sunday, the following Monday is a holiday", which only those two rules
/// do. The Grand Magal of Touba, on 18 Safar, was made a paid day off by
/// President Wade's decree and first kept on 12 January 2012, and written
/// into the law by law 2013-06 of 11 December 2013; the Monday after a
/// Sunday Magal is a decree each time, as in 2021, and is not carried.
/// The Hijri days are on the tabular calendar as approximations of the
/// sighted dates. Article 4's distinction of the days that are paid as
/// well as off — the national holiday, Tamkharit and 1 May — is not
/// carried.
pub static SENEGAL: RuleSet = RuleSet {
    code: "SN",
    english_name: "Senegal",
    rules: SN_RULES,
    substitution: SN_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Loi n° 74-52 du 4 novembre 1974 relative à la fête nationale et aux fêtes \
              légales, as amended by laws 83-54 and 89-41, from \"Le manuel du \
              travailleur\" on NATLEX (SEN-97260), retrieved 2026-09-22; NATLEX on loi \
              n° 2013-06 du 11 décembre 2013; mourides.com (18 December 2011) and \
              Seneweb on the Magal decree and its first application; Wikipedia, \
              \"Public holidays in Senegal\", for the English names",
};

// ─────────────────────────────────────────────────────────────────────────
// Côte d'Ivoire
// ─────────────────────────────────────────────────────────────────────────

/// Decree 2011-371, article 2 (new), items 13 to 16: "the day after" the
/// national holiday, Labour Day, Aïd el-Fitr, Christmas and Tabaski
/// "whenever the said feast falls on a Sunday".
static CI_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: false,
    on_collision: false,
    valid_from: Some(2011),
    valid_until: None,
}];

static CI_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Jour de l'an", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Easter Monday",
        "Lundi de Pâques",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::public("Labour Day", "Fête du travail", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Ascension", "Ascension", Rule::easter(ASCENSION)),
    HolidayRule::fixed_public(
        "Whit Monday",
        "Lundi de Pentecôte",
        Rule::easter(WHIT_MONDAY),
    ),
    HolidayRule::public(
        "Independence Day",
        "Fête de l'Indépendance",
        Rule::gregorian(8, 7),
    ),
    HolidayRule::fixed_public("Assumption", "Assomption", Rule::gregorian(8, 15)),
    HolidayRule::fixed_public("All Saints' Day", "Toussaint", Rule::gregorian(11, 1)),
    HolidayRule::fixed_public(
        "National Peace Day",
        "Journée nationale de la Paix",
        Rule::gregorian(11, 15),
    )
    .years(Some(1996), None),
    HolidayRule::public("Christmas Day", "Noël", Rule::gregorian(12, 25)),
    hijri(
        "Day after the Prophet's Birthday",
        "Lendemain du Maouloud",
        3,
        12,
    ),
    hijri(
        "Day after the Night of Destiny",
        "Lendemain de la Nuit du Destin",
        9,
        27,
    ),
    hijri_public("Eid al-Fitr", "Aïd el-Fitr", 10, 1),
    hijri_public("Tabaski", "Tabaski", 12, 10),
];

/// Côte d'Ivoire.
///
/// Decree 96-205 of 7 March 1996 determining the list and regime of
/// holidays, article 2 as rewritten by decree 2011-371 of 4 November
/// 2011: twelve days off, and "the day after" the national holiday,
/// Labour Day, Aïd el-Fitr, Christmas and Tabaski "whenever the said
/// feast falls on a Sunday", which those five rules do from 2011, the
/// 1996 wording not having been read. The national holiday of 7 August
/// and Labour Day are the decree's by reference. The Maouloud day is
/// "the day after the anniversary of the Prophet's birth" and the Night
/// of Destiny's "the day after" the night of 26 to 27 Ramadan, carried
/// as 12 Rabi al-awwal and 27 Ramadan on the tabular Hijri calendar, as
/// approximations of the dates the imams' councils announce and the
/// Ministry then decrees. National Peace Day is the 1996 decree's.
pub static COTE_D_IVOIRE: RuleSet = RuleSet {
    code: "CI",
    english_name: "Côte d'Ivoire",
    rules: CI_RULES,
    substitution: CI_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Décret n° 2011-371 du 4 novembre 2011 modifiant et complétant l'article 2 du \
              décret n° 96-205 du 7 mars 1996 déterminant la liste et le régime des jours \
              fériés, as loidici.biz reproduces it, retrieved 2026-09-22; Wikipedia (fr), \
              \"Fêtes et jours fériés en Côte d'Ivoire\", for the two lendemain days and \
              the sighting practice, and Wikipedia, \"Public holidays in Ivory Coast\", \
              for 1996",
};

// ─────────────────────────────────────────────────────────────────────────
// Benin
// ─────────────────────────────────────────────────────────────────────────

/// The day of the traditional religions from 2025: the second Friday of
/// January.
static BJ_TRADITIONAL_RELIGIONS: Rule = Rule::nth(1, 2, Weekday::Friday);

static BJ_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Fête du Nouvel An", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Traditional Religions Day",
        "Fête annuelle des religions traditionnelles",
        Rule::gregorian(1, 10),
    )
    .years(Some(1998), Some(2024)),
    HolidayRule::fixed_public(
        "Eve of Traditional Religions Day",
        "Jeudi précédant la fête des religions traditionnelles",
        Rule::Offset {
            base: &BJ_TRADITIONAL_RELIGIONS,
            days: -1,
        },
    )
    .years(Some(2025), None),
    HolidayRule::fixed_public(
        "Traditional Religions Day",
        "Fête annuelle des religions traditionnelles",
        BJ_TRADITIONAL_RELIGIONS,
    )
    .years(Some(2025), None),
    HolidayRule::observance(
        "Remembrance Day",
        "Journée de Souvenir",
        Rule::gregorian(1, 16),
    ),
    HolidayRule::observance(
        "People's Sovereignty Day",
        "Journée de la Souveraineté du Peuple",
        Rule::gregorian(2, 28),
    ),
    HolidayRule::observance("Women's Day", "Journée de la Femme", Rule::gregorian(3, 8)),
    HolidayRule::fixed_public(
        "Easter Monday",
        "Lundi de Pâques",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::fixed_public("Labour Day", "Fête du Travail", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Ascension", "Jour de l'Ascension", Rule::easter(ASCENSION)),
    HolidayRule::fixed_public(
        "Whit Monday",
        "Lundi de Pentecôte",
        Rule::easter(WHIT_MONDAY),
    ),
    HolidayRule::fixed_public("National Day", "Fête Nationale", Rule::gregorian(8, 1)),
    HolidayRule::fixed_public("Assumption", "Jour de l'Assomption", Rule::gregorian(8, 15)),
    HolidayRule::fixed_public(
        "All Saints' Day",
        "Jour de la Toussaint",
        Rule::gregorian(11, 1),
    ),
    HolidayRule::fixed_public("Christmas Day", "Jour de la Noël", Rule::gregorian(12, 25)),
    hijri("Maouloud", "Journée Maouloud", 3, 12),
    hijri("Eid al-Fitr", "Jour du Ramadan", 10, 1),
    hijri("Tabaski", "Jour de la Tabaski", 12, 10),
];

/// Benin.
///
/// Law 90-019 of 27 July 1990 fixing the legal holidays, from the scan the
/// Secrétariat général du Gouvernement publishes: article 1's twelve
/// legal holidays, which article 2 makes days off with pay, and article
/// 3's three national days — 16 January, 28 February and 8 March — which
/// are carried as observances; the scan stops at article 3, and nothing
/// after it was read. Law 97-031 of 20 August 1997 instituted the annual
/// feast of the traditional religions on 10 January, a paid day off, first
/// kept in 1998; law 2024-32 of 2 September 2024 repealed it and put the
/// feast on the second Friday of January, with that Friday "and the
/// Thursday before it" off with pay, from 2025. The Islamic days are on
/// the tabular calendar as approximations of the dates announced each
/// year. The law states no rule for a holiday on a Sunday and none is
/// carried; the days the Council of Ministers declares off now and then
/// are not carried either.
pub static BENIN: RuleSet = RuleSet {
    code: "BJ",
    english_name: "Benin",
    rules: BJ_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Loi n° 90-019 du 27 juillet 1990 fixant les fêtes légales en République du \
              Bénin (first page, articles 1 to 3), loi n° 97-031 du 20 août 1997 portant \
              institution d'une fête annuelle des religions traditionnelles, and loi n° \
              2024-32 du 02 septembre 2024 fixant la fête annuelle des religions \
              traditionnelles, each from the Secrétariat général du Gouvernement's \
              documenthèque (sgg.gouv.bj/doc/loi-90-019, loi-97-031, loi-2024-32), \
              retrieved 2026-09-23; Wikipedia (fr), \"Fêtes et jours fériés au Bénin\", \
              as a cross-check",
};

// ─────────────────────────────────────────────────────────────────────────
// Burkina Faso
// ─────────────────────────────────────────────────────────────────────────

/// Article 2 of law 079-2015/CNT: "when a legal holiday falls on a Sunday,
/// the day after is off with pay", until the 2026 law ended it.
static BF_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: false,
    on_collision: false,
    valid_from: Some(2016),
    valid_until: Some(2025),
}];

fn bf_unread(_year: i64) -> Days {
    Days::new()
}

/// The Monday after a Sunday holiday in 2026, the year the new law was
/// adopted: its promulgation date was not read, so whether the old
/// article 2 still gave a Sunday's Monday in the first months of 2026 is
/// not known.
const BF_2026_MONDAYS: Rule = Rule::Tabulated {
    function: bf_unread,
    first_year: 1,
    last_year: 0,
};

static BF_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Jour de l'An", Rule::gregorian(1, 1)),
    // 3 January 2026 came six days before the new law was adopted.
    HolidayRule::public(
        "Popular Uprising Day",
        "Soulèvement populaire",
        Rule::gregorian(1, 3),
    )
    .years(None, Some(2026)),
    HolidayRule::observance(
        "Popular Uprising Day",
        "Soulèvement populaire",
        Rule::gregorian(1, 3),
    )
    .years(Some(2027), None),
    HolidayRule::public(
        "International Women's Day",
        "Journée internationale de la femme",
        Rule::gregorian(3, 8),
    ),
    HolidayRule::public(
        "Easter Sunday",
        "Jour de Pâques",
        Rule::easter(EASTER_SUNDAY),
    )
    .years(None, Some(2025)),
    HolidayRule::public("Labour Day", "Fête du travail", Rule::gregorian(5, 1)),
    HolidayRule::public(
        "Customs and Traditions Day",
        "Journée des coutumes et traditions",
        Rule::gregorian(5, 15),
    )
    .years(Some(2024), None),
    HolidayRule::public("Ascension", "Ascension", Rule::easter(ASCENSION)),
    HolidayRule::observance(
        "Advent of the Democratic and Popular Revolution",
        "Avènement de la révolution démocratique et populaire",
        Rule::gregorian(8, 4),
    ),
    HolidayRule::public(
        "Independence Day",
        "Proclamation de l'indépendance",
        Rule::gregorian(8, 5),
    )
    .years(None, Some(2025)),
    HolidayRule::observance(
        "Independence Day",
        "Proclamation de l'indépendance",
        Rule::gregorian(8, 5),
    )
    .years(Some(2026), None),
    HolidayRule::public("Assumption", "Assomption", Rule::gregorian(8, 15)),
    HolidayRule::observance(
        "Day of Thanksgiving",
        "Journée d'action de grâce",
        Rule::gregorian(9, 29),
    )
    .years(None, Some(2025)),
    HolidayRule::observance(
        "Commemoration of the Assassination of Thomas Sankara",
        "Commémoration de l'assassinat du Président Thomas Sankara",
        Rule::gregorian(10, 15),
    )
    .years(Some(2026), None),
    HolidayRule::observance(
        "Popular Insurrection",
        "Insurrection populaire",
        Rule::gregorian(10, 30),
    )
    .years(Some(2016), Some(2025)),
    HolidayRule::public(
        "National Martyrs' Day",
        "Journée nationale des martyrs",
        Rule::gregorian(10, 31),
    )
    .years(Some(2016), Some(2025)),
    HolidayRule::observance(
        "National Martyrs' Day",
        "Journée nationale des martyrs",
        Rule::gregorian(10, 31),
    )
    .years(Some(2026), None),
    HolidayRule::public("All Saints' Day", "Toussaint", Rule::gregorian(11, 1))
        .years(None, Some(2025)),
    HolidayRule::observance("All Saints' Day", "Toussaint", Rule::gregorian(11, 1))
        .years(Some(2026), None),
    HolidayRule::public("National Day", "Fête nationale", Rule::gregorian(12, 11)),
    HolidayRule::public("Christmas Day", "Noël", Rule::gregorian(12, 25)),
    hijri_public("Mouloud", "Mouloud", 3, 12),
    hijri_public("Eid al-Fitr", "Ramadan", 10, 1),
    hijri_public("Tabaski", "Tabaski", 12, 10),
    HolidayRule::fixed_public(
        "Day after a Sunday holiday",
        "Lendemain d'une fête légale tombant un dimanche",
        BF_2026_MONDAYS,
    )
    .years(Some(2026), Some(2026)),
];

/// Burkina Faso.
///
/// Law 079-2015/CNT of 23 November 2015 instituting the legal holidays and
/// the events of a historical character, as the Académie de police
/// publishes it, from 2016: fifteen legal holidays, off with pay under
/// article 2, which adds that "when a legal holiday falls on a Sunday, the
/// day after is off with pay" — the policy, which is also how Easter
/// Sunday, the law's "jour de Pâques", gave the Monday; and article 4's
/// three commemorations with the services open, carried as observances.
/// The law repealed law 19-2000/AN, which was not read, so only the days
/// it names for the first time, the martyrs of 31 October and the
/// insurrection of 30 October, start in 2016. The Customs and Traditions
/// Day of 15 May was declared a holiday by the decree adopted in the
/// Council of Ministers of 6 March 2024. The law adopted by the
/// Transitional Legislative Assembly on 9 January 2026 repealed the 2015
/// law: eleven days off with pay — 1 January, 8 March, 1 May, 15 May,
/// 15 August, 11 December, Christmas, Ascension, Mouloud, Ramadan and
/// Tabaski — and 3 January, 4 and 5 August, 15 October, 31 October and
/// 1 November as days of commemoration, carried as observances; Easter
/// and the Sunday rule are gone. Its text and promulgation date were not
/// read, only the Assembly's and the press's accounts, so the table
/// switches at 2026, keeps 3 January 2026 — six days before the vote — on
/// the old law, and reports a Sunday holiday's Monday in 2026 as a gap.
/// The Islamic days are on the tabular calendar as approximations of the
/// dates the Government announces.
pub static BURKINA_FASO: RuleSet = RuleSet {
    code: "BF",
    english_name: "Burkina Faso",
    rules: BF_RULES,
    substitution: BF_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Loi n° 079-2015/CNT portant institution de fêtes légales et évènements à \
              caractère historique au Burkina Faso, from academiedepolice.bf, retrieved \
              2026-09-23; Présidence du Faso, \"Conseil des ministres : le 15 mai institué \
              Journée des coutumes et traditions\" (6 March 2024); Assemblée législative de \
              Transition, \"Nouvelle loi sur les jours fériés : ce qui va changer\" \
              (an.bf/545), leFaso.net and Sidwaya on the law adopted on 9 January 2026, \
              whose own text was not read",
};

// ─────────────────────────────────────────────────────────────────────────
// Cabo Verde
// ─────────────────────────────────────────────────────────────────────────

fn cv_unread(_year: i64) -> Days {
    Days::new()
}

/// 13 January in the years between law 16/IV/91, which does not have it,
/// and 2020, the first year a source read calls it a national holiday: the
/// law that added it was not read.
const CV_DEMOCRACY_DAY_UNREAD: Rule = Rule::Tabulated {
    function: cv_unread,
    first_year: 1,
    last_year: 0,
};

static CV_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Ano Novo", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Freedom and Democracy Day",
        "Dia da Liberdade e da Democracia",
        CV_DEMOCRACY_DAY_UNREAD,
    )
    .years(Some(1992), Some(2019)),
    HolidayRule::fixed_public(
        "Freedom and Democracy Day",
        "Dia da Liberdade e da Democracia",
        Rule::gregorian(1, 13),
    )
    .years(Some(2020), None),
    HolidayRule::fixed_public(
        "Nationality and National Heroes' Day",
        "Dia da Nacionalidade e dos Heróis Nacionais",
        Rule::gregorian(1, 20),
    ),
    HolidayRule::fixed_public(
        "Good Friday",
        "Sexta-Feira Santa",
        Rule::easter(GOOD_FRIDAY),
    ),
    HolidayRule::fixed_public("Workers' Day", "Dia do Trabalhador", Rule::gregorian(5, 1)),
    HolidayRule::observance("Children's Day", "Dia da Criança", Rule::gregorian(6, 1))
        .of_kind(Kind::School),
    HolidayRule::fixed_public(
        "Independence Day",
        "Dia da Independência Nacional",
        Rule::gregorian(7, 5),
    ),
    HolidayRule::fixed_public("Assumption", "Dia da Assunção", Rule::gregorian(8, 15)),
    HolidayRule::fixed_public(
        "All Saints' Day",
        "Dia de Todos os Santos",
        Rule::gregorian(11, 1),
    ),
    HolidayRule::fixed_public("Christmas Day", "Dia do Natal", Rule::gregorian(12, 25)),
];

/// Cabo Verde.
///
/// Law 16/IV/91 of 30 December 1991, from the supplement to the Boletim
/// Oficial no. 52 that the Government publishes: article 1's seven
/// national holidays "with total cessation of all activities not
/// permitted by law on Sundays", and Good Friday; article 3's Children's
/// Day on 1 June, on which schools may stop their normal activity, is a
/// school day off; article 2's two municipal holidays a year, chosen by
/// each Municipal Assembly, are not carried. The Freedom and Democracy
/// Day of 13 January is not in that law; it is carried from 2020, the
/// earliest year a source read names it a national holiday, and the
/// years 1992 to 2019 are a gap, the law that added it not having been
/// read. Carnival and Ash Wednesday, which the Government gives as
/// *tolerância de ponto* each year, are not holidays and not carried.
/// The law moves nothing off a Sunday.
pub static CABO_VERDE: RuleSet = RuleSet {
    code: "CV",
    english_name: "Cabo Verde",
    rules: CV_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Lei n.º 16/IV/91, de 30 de Dezembro, Suplemento ao Boletim Oficial de Cabo \
              Verde n.º 52 de 30 de Dezembro de 1991, pp. 10–11, from governo.cv, \
              retrieved 2026-09-23; for 13 January, Vatican News, \"Cabo Verde comemora 13 \
              de Janeiro\" (January 2020), and the list of national holidays of the \
              Consulate-General in the Netherlands (conscv.nl)",
};

// ─────────────────────────────────────────────────────────────────────────
// Guinea
// ─────────────────────────────────────────────────────────────────────────

/// Decree D/2022/0526: "if Independence Day, New Year's Day or Aïd el-Fitr
/// falls on a non-working day, the next working day is declared a
/// holiday". Only those three rules substitute.
static GN_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: Some(2023),
    valid_until: None,
}];

static GN_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Nouvel an", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Easter Monday",
        "Lundi de Pâques",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::fixed_public(
        "Labour Day",
        "Fête internationale du travail",
        Rule::gregorian(5, 1),
    ),
    HolidayRule::fixed_public(
        "Africa Day",
        "Anniversaire de l'Union africaine",
        Rule::gregorian(5, 25),
    ),
    HolidayRule::fixed_public("Assumption", "Assomption", Rule::gregorian(8, 15)),
    HolidayRule::public(
        "Independence Day",
        "Anniversaire de l'indépendance",
        Rule::gregorian(10, 2),
    ),
    HolidayRule::fixed_public("Christmas Day", "Noël", Rule::gregorian(12, 25)),
    hijri(
        "Day after the Prophet's Birthday",
        "Lendemain de la nuit du Maouloud",
        3,
        12,
    ),
    hijri(
        "Day after the Night of Destiny",
        "Lendemain de la nuit de Laylatoul Qadr",
        9,
        27,
    ),
    hijri_public("Eid al-Fitr", "Aïd el-Fitr", 10, 1),
    hijri("Tabaski", "Jour de la Tabaski", 12, 10),
    hijri("Day after Tabaski", "Lendemain de la Tabaski", 12, 11),
];

/// Guinea.
///
/// Decree D/2022/0526/PRG/CNRD/SGG of 2 November 2022 on the holidays,
/// which the Labour Code leaves to presidential decree: twelve days off
/// with pay for the public, private and mixed sectors, and "if
/// Independence Day, New Year's Day or Aïd el-Fitr falls on a non-working
/// day, the next working day is declared a holiday", which those three
/// rules do from 2023. The decree's own text was not read — only its
/// reading on the state media as the press reproduced it, which agree —
/// so the table does not say what it replaced or claim anything earlier.
/// The Maouloud day is "the day after the night of Maouloud" and the
/// Night of Destiny's "the day after the night of Laylatoul Qadr",
/// carried as 12 Rabi al-awwal and 27 Ramadan on the tabular Hijri
/// calendar as Côte d'Ivoire's are; all the Islamic days approximate the
/// dates the Ministry announces after the sighting.
pub static GUINEA: RuleSet = RuleSet {
    code: "GN",
    english_name: "Guinea",
    rules: GN_RULES,
    substitution: GN_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Décret D/2022/0526/PRG/CNRD/SGG du 2 novembre 2022 relatif aux jours fériés, \
              as Guinée Nondi (\"Guinée : les jours de fête et férié sont officiellement \
              connus\", 3 November 2022) and Africa Guinée reproduce it, the decree's own \
              text not read, retrieved 2026-09-23; Kalenews on the decree's number and \
              Labour Code article 222.6",
};

// ─────────────────────────────────────────────────────────────────────────
// Mali
// ─────────────────────────────────────────────────────────────────────────

static ML_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Jour de l'An", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Day of Recovered Sovereignty",
        "Journée nationale de la souveraineté retrouvée",
        Rule::gregorian(1, 14),
    )
    .years(Some(2023), None),
    HolidayRule::fixed_public(
        "Armed Forces Day",
        "Journée de l'Armée",
        Rule::gregorian(1, 20),
    ),
    HolidayRule::fixed_public("Martyrs' Day", "Journée du 26 mars", Rule::gregorian(3, 26)),
    HolidayRule::fixed_public(
        "Easter Monday",
        "Lundi de Pâques",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::fixed_public("Labour Day", "Fête du Travail", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Africa Day", "Journée de l'Afrique", Rule::gregorian(5, 25)),
    HolidayRule::fixed_public(
        "Independence Day",
        "Fête Nationale de la République du Mali",
        Rule::gregorian(9, 22),
    ),
    HolidayRule::fixed_public("Christmas Day", "Fête de Noël", Rule::gregorian(12, 25)),
    hijri("Prophet's Birthday", "Maouloud (Naissance)", 3, 12),
    hijri("Prophet's Baptism", "Maouloud (Baptême)", 3, 18),
    hijri("Eid al-Fitr", "Fête du Ramadan", 10, 1),
    hijri("Tabaski", "Tabaski", 12, 10),
];

/// Mali.
///
/// Law 05-040 of 22 July 2005 on the legal holidays, from the Journal
/// officiel of 10 September 2005: article 1's eleven legal holidays,
/// "the days of Maouloud (Birth and Baptism)" among them, all off with
/// pay under article 3. The Baptism is carried on 18 Rabi al-awwal, six
/// days after the Birth, as the Ministry of Labour's days of 25 and 31
/// August 2026 were, and like the other Islamic days it approximates the
/// announced date on the tabular calendar. Article 2
/// lets the Government declare the working day after "certain legal
/// holidays" a holiday as well, as it did for 2 January 2026; those days
/// are a decision each time and are not carried, nor is Achoura, which
/// the Ministry has declared off in some years on no footing the author
/// read. The Day of Recovered Sovereignty on 14 January was made a paid
/// day off by a decree of January 2023, which was not read, only Studio
/// Tamani's account of it. No Sunday rule.
pub static MALI: RuleSet = RuleSet {
    code: "ML",
    english_name: "Mali",
    rules: ML_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Loi n° 05-040 du 22 juillet 2005 relative aux fêtes légales en République \
              du Mali, Journal officiel de la République du Mali no. 25 of 10 September \
              2005 (sgg-mali.ml/JO/2005/mali-jo-2005-25.pdf), retrieved 2026-09-23; the \
              Ministry of Labour's communiqués on the Maouloud of 2025 and 2026 as Bamada \
              reports them; Studio Tamani, \"Mali : le 14 janvier désormais dédié à la \
              souveraineté\", on the decree of 2023",
};

// ─────────────────────────────────────────────────────────────────────────
// Oman
// ─────────────────────────────────────────────────────────────────────────

/// Thursday–Friday until 30 April 2013, Friday–Saturday from 1 May 2013.
static OM_WEEKEND: &[WeekendPolicy] = &[
    WeekendPolicy {
        days: &[Weekday::Thursday, Weekday::Friday],
        valid_from: None,
        valid_until: Some(2012),
    },
    WeekendPolicy {
        days: &[Weekday::Friday, Weekday::Saturday],
        valid_from: Some(2013),
        valid_until: None,
    },
];

/// Royal Decree 88/2022, article I, first: a weekend day inside one of the
/// single-day holidays "is compensated by one day", carried as the next
/// free weekday; the National Day pair and the Eids have their own rules.
static OM_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Friday, Weekday::Saturday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: Some(2020),
    valid_until: None,
}];

/// Whether `day` is on Oman's weekend in `year`.
fn om_is_weekend(year: i64, day: Rd) -> bool {
    let weekday = Weekday::from_rd(day);
    if year >= 2013 {
        matches!(weekday, Weekday::Friday | Weekday::Saturday)
    } else {
        matches!(weekday, Weekday::Thursday | Weekday::Friday)
    }
}

/// The first day after `last` that is not on the weekend.
fn om_next_weekday(year: i64, last: Rd) -> Rd {
    let mut cursor = Rd(last.0 + 1);
    while om_is_weekend(year, cursor) {
        cursor = Rd(cursor.0 + 1);
    }
    cursor
}

/// "If one or both days of the weekend fall within" the two National Day
/// days, "they are compensated by one day": the next weekday after the
/// pair, from the 2020 decree that first said so.
fn om_national_day_compensation(year: i64) -> Days {
    let mut out = Days::new();
    if year < 2020 {
        return out;
    }
    let first = if year >= 2025 { 20 } else { 18 };
    let Ok(a) = gregorian::to_fixed(year, 11, first) else {
        return out;
    };
    let b = Rd(a.0 + 1);
    if om_is_weekend(year, a) || om_is_weekend(year, b) {
        out.push(om_next_weekday(year, b));
    }
    out
}

/// "Friday is compensated if it falls on the first day of either Eid":
/// the next weekday after the holiday, read as the first day of each
/// holiday period, 29 Ramadan and 9 Dhu al-Hijja, on the tabular
/// calendar.
fn om_eid_compensation(year: i64) -> Days {
    let mut out = Days::new();
    if year < 2020 {
        return out;
    }
    for (month, day, span) in [(9, 29, 4), (12, 9, 3)] {
        let starts =
            Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, month, day).days_in_year(year);
        for start in starts.as_slice() {
            if Weekday::from_rd(*start) == Weekday::Friday {
                out.push(om_next_weekday(year, Rd(start.0 + span)));
            }
        }
    }
    out
}

static OM_RULES: &[HolidayRule] = &[
    HolidayRule::public(
        "Accession Day",
        "يوم تولي جلالة السلطان مقاليد الحكم",
        Rule::gregorian(1, 11),
    )
    .years(Some(2021), None),
    HolidayRule::fixed_public("Renaissance Day", "يوم النهضة", Rule::gregorian(7, 23))
        .years(None, Some(2019)),
    HolidayRule::fixed_public("National Day", "العيد الوطني", Rule::gregorian(11, 18))
        .years(None, Some(2024)),
    HolidayRule::fixed_public("National Day", "العيد الوطني", Rule::gregorian(11, 19))
        .years(Some(2020), Some(2024)),
    HolidayRule::fixed_public("National Day", "العيد الوطني", Rule::gregorian(11, 20))
        .years(Some(2025), None),
    HolidayRule::fixed_public("National Day", "العيد الوطني", Rule::gregorian(11, 21))
        .years(Some(2025), None),
    HolidayRule::fixed_public(
        "National Day",
        "العيد الوطني",
        Rule::Computed(om_national_day_compensation),
    ),
    hijri_public("Islamic New Year", "رأس السنة الهجرية", 1, 1),
    hijri_public("Prophet's Birthday", "المولد النبوي الشريف", 3, 12),
    hijri_public("Isra and Mi'raj", "الإسراء والمعراج", 7, 27),
    hijri("Eid al-Fitr", "عيد الفطر", 9, 29),
    hijri("Eid al-Fitr", "عيد الفطر", 9, 30),
    hijri("Eid al-Fitr", "عيد الفطر", 10, 1),
    hijri("Eid al-Fitr", "عيد الفطر", 10, 2),
    hijri("Eid al-Fitr", "عيد الفطر", 10, 3),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 9),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 10),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 11),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 12),
    HolidayRule::fixed_public(
        "Eid Compensation Day",
        "يوم تعويض",
        Rule::Computed(om_eid_compensation),
    )
    .approximate(),
];

/// Oman.
///
/// Royal Decree 88/2022 determining the official holidays, in Decree's
/// translation, as amended by Royal Decree 15/2025: the Hijri New Year,
/// the Prophet's Birthday, Isra and Mi'raj, Accession Day on 11 January
/// and National Day, on 18 and 19 November until Royal Decree 15/2025
/// made it 20 and 21 from 2025; "if one or both days of the weekend fall
/// within the aforementioned holidays, they are compensated by one day",
/// which is the Friday–Saturday policy for the single days and a
/// computed day after the National Day pair; Eid al-Fitr "starting from
/// 29 Ramadan until 3 Shawwal" and Eid al-Adha "from 9 Dhu Al-Hijja
/// until 12 Dhu Al-Hijja", on the tabular calendar, which always gives
/// 30 Ramadan and so five days, and "Friday is compensated if it falls on
/// the first day of either Eid", read as the first day of the holiday
/// and computed. Royal Decree 56/2020, which the 2022 decree replaced,
/// carried the same terms, and the compensations run from 2020; its
/// predecessor 76/96 was not read. Renaissance Day on 23 July ended with
/// the 2020 decree and Accession Day was first kept in 2021. The weekend
/// was Thursday–Friday until 1 May 2013.
pub static OMAN: RuleSet = RuleSet {
    code: "OM",
    english_name: "Oman",
    rules: OM_RULES,
    substitution: OM_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: OM_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Royal Decree 88/2022 Determining the Official Holidays and Royal Decree \
              15/2025 amending it, in Decree's translations (decree.om), retrieved \
              2026-09-22; Royal Decree 56/2020 on the same site; the Arabian Stories \
              (20 April 2020) on Renaissance Day's end; Arab News and Gulf News \
              (April 2013) on the weekend; Wikipedia, \"Public holidays in Oman\"",
};

// ─────────────────────────────────────────────────────────────────────────
// Qatar
// ─────────────────────────────────────────────────────────────────────────

/// Emiri Decision 57/2025, article 2: "if the interval between two
/// holidays is a single working day, it is a holiday within the two".
static QA_BRIDGES: &[BridgePolicy] = &[BridgePolicy {
    name: "Bridge Day",
    local_name: "يوم فاصل",
    max_gap: 1,
    exclude_weekdays: &[Weekday::Friday, Weekday::Saturday],
    valid_from: Some(2025),
    valid_until: None,
}];

static QA_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("National Day", "اليوم الوطني", Rule::gregorian(12, 18))
        .years(Some(2007), None),
    HolidayRule::fixed_public(
        "National Sport Day",
        "اليوم الرياضي",
        Rule::nth(2, 2, Weekday::Tuesday),
    )
    .years(Some(2012), None),
    hijri("Eid al-Fitr", "عيد الفطر", 9, 28),
    hijri("Eid al-Fitr", "عيد الفطر", 9, 29),
    hijri("Eid al-Fitr", "عيد الفطر", 9, 30),
    hijri("Eid al-Fitr", "عيد الفطر", 10, 1),
    hijri("Eid al-Fitr", "عيد الفطر", 10, 2),
    hijri("Eid al-Fitr", "عيد الفطر", 10, 3),
    hijri("Eid al-Fitr", "عيد الفطر", 10, 4),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 9),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 10),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 11),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 12),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 13),
    HolidayRule::fixed_public(
        "Bank Holiday",
        "عطلة اليوم الأول من شهر يناير",
        Rule::gregorian(1, 1),
    )
    .of_kind(Kind::Bank),
    HolidayRule::fixed_public("Bank Day", "يوم البنوك", Rule::nth(3, 1, Weekday::Sunday))
        .of_kind(Kind::Bank)
        .years(Some(2009), None),
];

/// Qatar.
///
/// Emiri Decision 57/2025 determining the working days, occasions and
/// official holidays of the State, from Al Meezan: Sunday to Thursday
/// with Friday and Saturday the weekend; National Day on 18 December;
/// National Sport Day on the second Tuesday of February; Eid al-Fitr
/// "from the twenty-eighth of Ramadan to the end of the fourth of
/// Shawwal" and Eid al-Adha "from the ninth to the end of the thirteenth
/// of Dhu al-Hijja", on the tabular calendar as approximations of the
/// sighted dates, which the Amiri Diwan announces; and "if the interval
/// between two holidays is a single working day, it is a holiday",
/// carried as a bridge from 2025, the year the Cabinet's decision
/// 18/2025 first put the same words into the decision of 2008, whose
/// original text was not read. Article 4's days for the Central Bank and
/// the financial sector, 1 January and the first Sunday of March, are
/// bank days. National Day dates from 2007 and Sport Day from 2012,
/// under Emiri Resolution 80/2011. The private sector's holidays are the
/// Labour Law's, which gives three days for each Eid, and are not carried.
pub static QATAR: RuleSet = RuleSet {
    code: "QA",
    english_name: "Qatar",
    rules: QA_RULES,
    substitution: &[],
    bridges: QA_BRIDGES,
    includes: &[],
    weekend: FRIDAY_SATURDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Emiri Decision No. 57 of 2025 determining the working days, occasions and \
              official holidays in the State, and Cabinet Decision No. 18 of 2025 amending \
              Cabinet Decision No. 6 of 2008, as Al Meezan publishes them (almeezan.qa), \
              retrieved 2026-09-22; Al Meezan on Emiri Resolution No. 80 of 2011 on Sports \
              Day; Wikipedia, \"National Day (Qatar)\" and \"Public holidays in Qatar\", \
              for 2007 and 2009",
};

// ─────────────────────────────────────────────────────────────────────────
// Iraq
// ─────────────────────────────────────────────────────────────────────────

/// A day article 2 of the 2024 law gives to one community: a religious
/// day, not a day off for everyone.
const fn iq_community(name: &'static str, local: &'static str, rule: Rule) -> HolidayRule {
    HolidayRule::observance(name, local, rule).of_kind(Kind::Religious)
}

/// The Yazidi Feast of the Assembly, 23 to 30 September in the Julian
/// calendar the law calls "eastern".
const fn iq_jama(day: u8) -> HolidayRule {
    iq_community(
        "Feast of the Assembly",
        "عيد الجما",
        Rule::in_calendar(CalendarSystem::JULIAN, 9, day),
    )
    .years(Some(2024), None)
}

static IQ_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "New Year's Day",
        "رأس السنة الميلادية",
        Rule::gregorian(1, 1),
    ),
    HolidayRule::fixed_public("Army Day", "عيد الجيش العراقي", Rule::gregorian(1, 6)),
    HolidayRule::fixed_public(
        "Remembrance of the Ba'ath Crimes",
        "ذكرى جرائم البعث الصدامي بحق الشعب العراقي",
        Rule::gregorian(3, 16),
    )
    .years(Some(2024), None),
    HolidayRule::fixed_public("Nowruz", "عيد نوروز", Rule::gregorian(3, 21)),
    HolidayRule::fixed_public("Labour Day", "عيد العمال العالمي", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Christmas Day", "عيد الميلاد", Rule::gregorian(12, 25))
        .years(Some(2020), Some(2023)),
    hijri("Islamic New Year", "رأس السنة الهجرية", 1, 1),
    hijri("Ashura", "عاشوراء", 1, 10),
    hijri("Prophet's Birthday", "المولد النبوي الشريف", 3, 12),
    hijri("Eid al-Fitr", "عيد الفطر", 10, 1),
    hijri("Eid al-Fitr", "عيد الفطر", 10, 2),
    hijri("Eid al-Fitr", "عيد الفطر", 10, 3),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 10),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 11),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 12),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 13),
    hijri("Eid al-Ghadir", "عيد الغدير", 12, 18).years(Some(2024), None),
    iq_community(
        "Christmas Day",
        "ميلاد السيد المسيح",
        Rule::gregorian(12, 25),
    )
    .years(Some(2024), None),
    iq_community("Easter Sunday", "العيد الكبير", Rule::easter(EASTER_SUNDAY))
        .years(Some(2024), None),
    iq_community("Easter Monday", "العيد الكبير", Rule::easter(EASTER_MONDAY))
        .years(Some(2024), None),
    iq_community(
        "Yazidi New Year",
        "عيد رأس السنة الإيزيدية",
        Rule::WeekdayOnOrAfter {
            month: 4,
            day: 14,
            weekday: Weekday::Wednesday,
        },
    )
    .years(Some(2024), None),
    iq_community(
        "Feast of the Forty Days of Summer",
        "عيد أربعينية الصيف",
        Rule::gregorian(7, 20),
    )
    .years(Some(2024), None),
    iq_community(
        "Feast of the Forty Days of Summer",
        "عيد أربعينية الصيف",
        Rule::gregorian(7, 21),
    )
    .years(Some(2024), None),
    iq_jama(23),
    iq_jama(24),
    iq_jama(25),
    iq_jama(26),
    iq_jama(27),
    iq_jama(28),
    iq_jama(29),
    iq_jama(30),
    iq_community(
        "Yazidi Feast of the Fast",
        "عيد الصيام",
        Rule::WeekdayOnOrAfter {
            month: 12,
            day: 14,
            weekday: Weekday::Friday,
        },
    )
    .years(Some(2024), None),
];

/// Iraq.
///
/// The Official Holidays Law No. 12 of 2024, from the Official Gazette
/// of 27 May 2024, article 1: Friday and Saturday each week; 1 and
/// 10 Muharram, 12 Rabi al-Awwal, 1 to 3 Shawwal, 10 to 13 Dhu al-Hijja
/// and, new in the law, 18 Dhu al-Hijja for Ghadir, on the tabular
/// calendar as approximations of the dates the Shia and Sunni endowments
/// announce; 1 and 6 January, 21 March, 1 May, and, new, 16 March for the
/// crimes of the Ba'ath. The 3 October and 10 December the Government
/// had kept by decision are not in the law and not carried, nor are the
/// temporary holidays of article 3 or the days the holy-city governorates
/// may add. Christmas was a holiday for all by the 2020 amendment of the
/// 1972 law, carried over 2020–2023; the 2024 law's article 2 gives it,
/// with two days of Easter, to Christians, and gives the Yazidis the
/// first Wednesday of "eastern" April, the Feast of the Assembly on 23 to
/// 30 eastern September, 20 and 21 July, and the first Friday of eastern
/// December — the Julian calendar, its first days carried as the
/// Gregorian 14th within this century — all religious days here; the
/// Mandaean feasts are in a calendar the crate lacks and are not
/// carried. The 1972 law was not read, so nothing before 2024 is dated
/// beyond Christmas. The Kurdistan Region's own holidays are not carried.
pub static IRAQ: RuleSet = RuleSet {
    code: "IQ",
    english_name: "Iraq",
    rules: IQ_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: FRIDAY_SATURDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Official Holidays Law No. 12 of 2024, Al-Waqa'i' al-Iraqiya no. 4777 of \
              27 May 2024, from the Ministry of Justice's copy (moj.gov.iq), retrieved \
              2026-09-22; Shafaq News (May 2024) on the vote; Vatican News (December 2020) \
              and Channel 8 (December 2024) on Christmas; Wikipedia, \"Public holidays in \
              Iraq\", for the English names",
};

// ─────────────────────────────────────────────────────────────────────────
// Botswana
// ─────────────────────────────────────────────────────────────────────────

/// Section 2(1)(i): a Sunday gives the following Monday, and nothing
/// when that Monday is already a holiday.
static BW_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: false,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// Section 2(1)(ii): "if 2nd January, 1st October or Boxing Day falls on
/// a Monday, the following Tuesday shall be observed".
const BW_MONDAY_TO_TUESDAY: &[(Weekday, i16)] = &[(Weekday::Monday, 1)];

static BW_JANUARY_2: Rule = Rule::gregorian(1, 2);
static BW_OCTOBER_1: Rule = Rule::gregorian(10, 1);
static BW_DECEMBER_26: Rule = Rule::gregorian(12, 26);
static BW_PRESIDENTS_DAY: Rule = Rule::nth(7, 3, Weekday::Monday);

static BW_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public(
        "New Year Holiday",
        "",
        Rule::moved_by_weekday(&BW_JANUARY_2, BW_MONDAY_TO_TUESDAY),
    ),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Holy Saturday", "", Rule::easter(HOLY_SATURDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::fixed_public("Ascension Day", "", Rule::easter(ASCENSION)),
    HolidayRule::public("Labour Day", "", Rule::gregorian(5, 1)),
    HolidayRule::public("Sir Seretse Khama Day", "", Rule::gregorian(7, 1)),
    HolidayRule::fixed_public("President's Day", "", Rule::nth(7, 3, Weekday::Monday)),
    HolidayRule::fixed_public(
        "President's Day Holiday",
        "",
        Rule::Offset {
            base: &BW_PRESIDENTS_DAY,
            days: 1,
        },
    ),
    HolidayRule::public("Botswana Day", "", Rule::gregorian(9, 30))
        .substitute_on(&[Weekday::Saturday, Weekday::Sunday]),
    HolidayRule::public(
        "Botswana Day Holiday",
        "",
        Rule::moved_by_weekday(&BW_OCTOBER_1, BW_MONDAY_TO_TUESDAY),
    ),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public(
        "Boxing Day",
        "",
        Rule::moved_by_weekday(&BW_DECEMBER_26, BW_MONDAY_TO_TUESDAY),
    ),
];

/// Botswana.
///
/// The Public Holidays Act (Cap. 03:07, re-enacted by Act 17 of 2006),
/// from the NATLEX copy: the Schedule's fourteen days, and section 2(1)'s
/// three provisos — a Sunday gives the following Monday; "if 2nd
/// January, 1st October or Boxing Day falls on a Monday, the following
/// Tuesday shall be observed", which those three rules do by themselves,
/// so that a Sunday New Year's Day, Botswana Day or Christmas takes the
/// Monday and its second day the Tuesday; and a Saturday Botswana Day
/// gives the Monday. When that Saturday puts Botswana Day and a Sunday
/// 1 October on the same Monday, as in 2023, the Act gives no Tuesday,
/// and the published calendar for 2023 kept none. The second days are
/// named here for the days they follow, the Schedule giving them only
/// their dates. Section 2(2)'s three-day mining calendar and the days the
/// President appoints under section 3 are not carried.
pub static BOTSWANA: RuleSet = RuleSet {
    code: "BW",
    english_name: "Botswana",
    rules: BW_RULES,
    substitution: BW_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Public Holidays Act, Cap. 03:07 (Act 17 of 2006), sections 2 and 3 and the \
              Schedule, from the NATLEX copy (BWA76156), retrieved 2026-09-22; Wikipedia, \
              \"Public holidays in Botswana\", for the names; Office Holidays, \"National \
              Holidays in Botswana in 2023\", for the days kept in 2023",
};

// ─────────────────────────────────────────────────────────────────────────
// Namibia
// ─────────────────────────────────────────────────────────────────────────

/// Section 1(2): "when a public holiday falls on a Sunday the following
/// Monday shall also be a public holiday, unless that Monday is a public
/// holiday" — so the search does not go past a taken Monday.
static NA_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: false,
    on_collision: false,
    valid_from: Some(1991),
    valid_until: None,
}];

static NA_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public("Independence Day", "", Rule::gregorian(3, 21)).years(Some(1990), None),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("Workers' Day", "", Rule::gregorian(5, 1)),
    HolidayRule::public("Cassinga Day", "", Rule::gregorian(5, 4)),
    HolidayRule::fixed_public("Ascension Day", "", Rule::easter(ASCENSION)),
    HolidayRule::public("Africa Day", "", Rule::gregorian(5, 25)),
    HolidayRule::public("Genocide Remembrance Day", "", Rule::gregorian(5, 28))
        .years(Some(2025), None),
    HolidayRule::public("Heroes' Day", "", Rule::gregorian(8, 26)),
    HolidayRule::public(
        "International Human Rights Day",
        "",
        Rule::gregorian(12, 10),
    )
    .years(None, Some(2004)),
    HolidayRule::public(
        "Day of the Namibian Women and International Human Rights Day",
        "",
        Rule::gregorian(12, 10),
    )
    .years(Some(2005), None),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Family Day", "", Rule::gregorian(12, 26)),
];

/// Namibia.
///
/// The Public Holidays Act 26 of 1990, in force from 1 February 1991,
/// from the Legal Assistance Centre's annotated text: the Schedule's
/// twelve days, 10 December renamed by the amendment of 2004, and
/// section 1(2), "when a public holiday falls on a Sunday the following
/// Monday shall also be a public holiday, unless that Monday is a public
/// holiday" — a Sunday Christmas therefore adds nothing to Family Day.
/// Genocide Remembrance Day on 28 May is Proclamation 19 of 2024 under
/// section 1(3), "with effect from 28 May 2025". Independence Day is
/// carried from 1990 itself; the other days the President proclaims for
/// a year are not carried.
pub static NAMIBIA: RuleSet = RuleSet {
    code: "NA",
    english_name: "Namibia",
    rules: NA_RULES,
    substitution: NA_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Public Holidays Act 26 of 1990, as amended by Act 16 of 2004, in the Legal \
              Assistance Centre's annotated statutes (lac.org.na), retrieved 2026-09-22; \
              Government Gazette No. 8373 of 28 May 2024, Proclamation No. 19; Wikipedia, \
              \"Public holidays in Namibia\"",
};

// ─────────────────────────────────────────────────────────────────────────
// Mauritius
// ─────────────────────────────────────────────────────────────────────────

/// The Second Schedule's alternation from 2016, the Assumption first: the
/// Assumption in even years.
fn mu_assumption(year: i64) -> Days {
    let mut out = Days::new();
    if year < 2016 || year % 2 != 0 {
        return out;
    }
    if let Ok(day) = gregorian::to_fixed(year, 8, 15) {
        out.push(day);
    }
    out
}

/// All Saints' Day every year until 2015 and in odd years since.
fn mu_all_saints(year: i64) -> Days {
    let mut out = Days::new();
    if year > 2015 && year % 2 == 0 {
        return out;
    }
    if let Ok(day) = gregorian::to_fixed(year, 11, 1) {
        out.push(day);
    }
    out
}

/// Thaipoosam Cavadee: the Pusam rule of [`crate::hindu::THAIPUSAM`] at
/// the island's own meridian, which in 2023 cut the Moon's stay in Pusam
/// a day earlier than Malaysia's did.
const MU_CAVADEE: Rule = Rule::Nakshatra {
    nakshatra: PUSHYA,
    sign: SiderealSign::MAKARA,
    with_tithi: Some(15),
    ayanamsa: Ayanamsa::LAHIRI,
    meridian: Meridian::from_seconds(4 * 3_600),
};

static MU_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("New Year Holiday", "", Rule::gregorian(1, 2)),
    HolidayRule::fixed_public("Abolition of Slavery", "", Rule::gregorian(2, 1))
        .years(Some(2001), None),
    HolidayRule::fixed_public("Thaipoosam Cavadee", "", MU_CAVADEE).approximate(),
    HolidayRule::fixed_public("Maha Shivaratree", "", MAHA_SHIVARATRI).approximate(),
    HolidayRule::fixed_public(
        "Chinese Spring Festival",
        "",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 1),
    ),
    HolidayRule::fixed_public(
        "Independence Day and Republic Day",
        "",
        Rule::gregorian(3, 12),
    ),
    HolidayRule::fixed_public("Ougadi", "", UGADI).approximate(),
    hijri("Eid-Ul-Fitr", "", 10, 1),
    HolidayRule::fixed_public("Labour Day", "", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public(
        "Assumption of the Blessed Virgin Mary",
        "",
        Rule::Computed(mu_assumption),
    ),
    HolidayRule::fixed_public("Ganesh Chaturthi", "", GANESH_CHATURTHI).approximate(),
    HolidayRule::fixed_public("All Saints' Day", "", Rule::Computed(mu_all_saints)),
    HolidayRule::fixed_public(
        "Arrival of Indentured Labourers",
        "",
        Rule::gregorian(11, 2),
    )
    .years(Some(2001), None),
    HolidayRule::fixed_public("Divali", "", DIWALI).approximate(),
    HolidayRule::fixed_public("Christmas Day", "", Rule::gregorian(12, 25)),
];

/// Mauritius.
///
/// The Public Holidays Act (Act 22 of 1968) as amended by Act 28 of 2015
/// from 1 January 2016 and Act 22 of 2019, from the Government's laws
/// portal: the First Schedule's days "observed as public holidays every
/// year" — 1 and 2 January, 1 February, 12 March, 1 May, 2 November,
/// Christmas, and the Chinese Spring Festival, Divali, Eid-Ul-Fitr,
/// Ganesh Chaturthi, Maha Shivaratree, Ougadi and Thaipoosam Cavadee on
/// the days the Prime Minister's Office notifies — and the Second
/// Schedule's Assumption or All Saints "on an alternate basis", the
/// Assumption in 2016 and so in even years. The notified feasts are on
/// the crate's Chinese, tabular Hijri and Hindu rules as approximations,
/// Cavadee on the Pusam rule at the island's meridian, which gives the
/// days kept from 2020 to 2026. Abolition of Slavery and the Arrival
/// of Indentured Labourers were declared for each year from 2001 until
/// the 2015 Act put them in the Schedule, and run from 2001 here; the
/// alternation of Cavadee with Tamizh Puttaandu that the 2015 Bill
/// proposed was not read in the Act and is not carried. Sundays are in
/// the Schedule, and nothing moves off one.
pub static MAURITIUS: RuleSet = RuleSet {
    code: "MU",
    english_name: "Mauritius",
    rules: MU_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Public Holidays Act (Act 22 of 1968) as amended to Act 22 of 2019, from \
              lawsofmauritius.govmu.org, retrieved 2026-09-22, for section 3 and the two \
              Schedules; the Public Holidays (Amendment) Bill (No. XIV of 2015) and its \
              explanatory memorandum (mauritiusassembly.govmu.org); the Prime Minister's \
              Office's General Notices No. 989 of 2024 and No. 1195 of 2025 for the 2025 \
              and 2026 dates; L'Express (lexpress.mu), \"Le 1er février 2001: un jour férié \
              pour commémorer l'abolition de l'esclavage\" and \"2-novembre: se souvenir de \
              l'arrivée des travailleurs engagés\", for the two days kept from 2001; Office \
              Holidays, Mauritius 2020 to 2023, for the Cavadee days kept; Wikipedia, \
              \"Culture of Mauritius\"",
};

// ─────────────────────────────────────────────────────────────────────────
// Malawi
// ─────────────────────────────────────────────────────────────────────────

/// Section 4: a Schedule day "other than the Saturday following Good
/// Friday" on a Saturday or Sunday gives "the next succeeding day, not
/// being itself a Sunday or public holiday", and itself ceases to be one.
static MW_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: Some(1983),
    valid_until: None,
}];

static MW_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public("John Chilembwe Day", "", Rule::gregorian(1, 15)),
    HolidayRule::public("Martyrs' Day", "", Rule::gregorian(3, 3)),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Holy Saturday", "", Rule::easter(HOLY_SATURDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY)),
    HolidayRule::public("Labour Day", "", Rule::gregorian(5, 1)),
    HolidayRule::public("Kamuzu Day", "", Rule::gregorian(5, 14)),
    HolidayRule::public("Independence Day", "", Rule::gregorian(7, 6)).years(Some(1964), None),
    HolidayRule::public("Mothers' Day", "", Rule::gregorian(10, 15)),
    hijri_public("Eid al-Fitr", "Eid al Fitri", 10, 1),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::public("Boxing Day", "", Rule::gregorian(12, 26)),
];

/// Malawi.
///
/// The Public Holidays Act (Cap. 18:05), from the NATLEX copy: the
/// Schedule as the Government Notices to 41 of 2007 left it — New Year's
/// Day, John Chilembwe Day, Martyrs' Day, Good Friday, the Saturday
/// following, Easter Monday, Labour Day, Kamuzu Day, Independence Day,
/// Mothers' Day, Eid al Fitri and Christmas — and section 4, from the
/// 1983 amendment: a Schedule day "other than the Saturday following
/// Good Friday" that falls on a Saturday or Sunday gives "the next
/// succeeding day, not being itself a Sunday or public holiday" and
/// ceases to be a holiday itself; the table keeps it as the day the
/// substitute is for. Boxing Day is not in the copy read and is kept in
/// the Government's yearly lists; it is carried without a first year.
/// Eid on the tabular calendar approximates the sighting. The days the
/// Minister adds or substitutes by order are not carried.
pub static MALAWI: RuleSet = RuleSet {
    code: "MW",
    english_name: "Malawi",
    rules: MW_RULES,
    substitution: MW_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Public Holidays Act, Cap. 18:05, sections 2 to 4 and the Schedule, from the \
              NATLEX copy (MWI90377), retrieved 2026-09-22; Nyasa Times on the Christmas, \
              Boxing and New Year's holidays; Wikipedia, \"Public holidays in Malawi\"",
};

// ─────────────────────────────────────────────────────────────────────────
// Syria
// ─────────────────────────────────────────────────────────────────────────

/// Friday alone, and Friday and Saturday from February 2004, when the
/// Council of Ministers added Saturday for the State's offices.
static SY_WEEKEND: &[WeekendPolicy] = &[
    WeekendPolicy {
        days: &[Weekday::Friday],
        valid_from: None,
        valid_until: Some(2003),
    },
    WeekendPolicy {
        days: &[Weekday::Friday, Weekday::Saturday],
        valid_from: Some(2004),
        valid_until: None,
    },
];

static SY_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "New Year's Day",
        "عيد رأس السنة الميلادية",
        Rule::gregorian(1, 1),
    ),
    HolidayRule::fixed_public(
        "Syrian Revolution Day",
        "عيد الثورة السورية",
        Rule::gregorian(3, 18),
    )
    .years(Some(2026), None),
    HolidayRule::fixed_public("Mother's Day", "عيد الأم", Rule::gregorian(3, 21)),
    HolidayRule::fixed_public("Nowruz", "عيد النوروز", Rule::gregorian(3, 21))
        .years(Some(2026), None),
    HolidayRule::fixed_public("Evacuation Day", "عيد الجلاء", Rule::gregorian(4, 17)),
    HolidayRule::fixed_public(
        "Western Easter",
        "عيد الفصح لدى الطوائف المسيحية الغربية",
        Rule::easter(EASTER_SUNDAY),
    ),
    HolidayRule::fixed_public(
        "Eastern Easter",
        "عيد الفصح لدى الطوائف المسيحية الشرقية",
        Rule::paschal(EASTER_SUNDAY),
    ),
    HolidayRule::fixed_public("Labour Day", "عيد العمال", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Liberation Day", "عيد التحرير", Rule::gregorian(12, 8))
        .years(Some(2025), None),
    HolidayRule::fixed_public(
        "Christmas Day",
        "عيد الميلاد لدى جميع الطوائف المسيحية",
        Rule::gregorian(12, 25),
    ),
    hijri("Islamic New Year", "عيد رأس السنة الهجرية", 1, 1),
    hijri("Prophet's Birthday", "عيد المولد النبوي الشريف", 3, 12),
    hijri("Eid al-Fitr", "عيد الفطر السعيد", 10, 1),
    hijri("Eid al-Fitr", "عيد الفطر السعيد", 10, 2),
    hijri("Eid al-Fitr", "عيد الفطر السعيد", 10, 3),
    hijri("Eid al-Adha", "عيد الأضحى المبارك", 12, 10),
    hijri("Eid al-Adha", "عيد الأضحى المبارك", 12, 11),
    hijri("Eid al-Adha", "عيد الأضحى المبارك", 12, 12),
    hijri("Eid al-Adha", "عيد الأضحى المبارك", 12, 13),
];

/// Syria.
///
/// Decree 188 of 5 October 2025, issued by President Ahmed al-Sharaa and
/// published by SANA, which repealed Decree 474 of 30 December 2004 and
/// fixes the days on which those under the Basic Law for State Workers
/// have a paid holiday: Eid al-Fitr three days and Eid al-Adha four, the
/// Hijri New Year and the Prophet's Birthday one each, on the tabular
/// calendar as approximations of the dates the Presidency's notices
/// announce under article 3; 1 January, Mother's Day on 21 March,
/// Evacuation Day on 17 April, 1 May and Christmas on 25 December "for
/// all Christian denominations"; Easter "for the Eastern Christian
/// denominations" and Easter "for the Western", one day each, which the
/// notice for 2026 closed on their Sundays, 5 and 12 April; and the two
/// new days, Liberation Day on 8 December, first kept in 2025, and the
/// Revolution on 18 March, first kept in 2026. Article 5 of Decree 13 of
/// 16 January 2026 adds Nowruz on 21 March, the day Mother's Day already
/// holds. The four days the decree dropped — 8 March, Teachers' Day, the
/// October War and Martyrs' Day — and the 2004 decree's list are not
/// carried, so nothing before 2025 is claimed beyond the days that stayed.
/// Eid al-Fitr is carried as 1 to 3 Shawwal and Eid al-Adha as 10 to
/// 13 Dhu al-Hijja; the notices lengthen them, as 2026's 26 to 30 May
/// from the Day of Arafah did, and those spans are not carried. The table
/// is the Damascus Government's; days kept by local authorities outside
/// its administration are not. The decree says nothing of a holiday on the
/// weekend, and nothing moves. The weekend is Friday and Saturday from
/// February 2004.
pub static SYRIA: RuleSet = RuleSet {
    code: "SY",
    english_name: "Syria",
    rules: SY_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SY_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Decree No. 188 of 2025 determining the official holidays, as SANA published it \
              (sana.sy/presidency/2299819), retrieved 2026-09-23; Decree No. 13 of 2026, \
              article 5, on Nowruz (sana.sy/presidency/2376054); the General Secretariat \
              of the Presidency's notices for 7 and 8 December 2025, 18 to 23 March 2026 and \
              5 and 12 April 2026 (SANA) and for 26 to 30 May 2026 (Al-Ain); Al-Dustour \
              (25 December 2003) on the Council of Ministers' decision adding Saturday to \
              the weekly holiday from February 2004; Wikipedia, \"Public holidays in \
              Syria\", for the English names",
};

// ─────────────────────────────────────────────────────────────────────────
// Palestine
// ─────────────────────────────────────────────────────────────────────────

/// A day the Council of Ministers' tables give only the Eastern or only the
/// Western Christian employees: a religious day, not a day off for all.
const fn ps_christian(name: &'static str, local: &'static str, rule: Rule) -> HolidayRule {
    HolidayRule::observance(name, local, rule).of_kind(Kind::Religious)
}

/// 1 Shawwal, the base of the eve of Eid al-Fitr.
static PS_SHAWWAL_1: Rule = EID_AL_FITR;

static PS_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "New Year's Day",
        "رأس السنة الميلادية",
        Rule::gregorian(1, 1),
    ),
    HolidayRule::fixed_public(
        "Eastern Christmas",
        "عيد الميلاد المجيد الشرقي",
        Rule::gregorian(1, 7),
    ),
    HolidayRule::fixed_public(
        "International Women's Day",
        "يوم المرأة العالمي",
        Rule::gregorian(3, 8),
    ),
    // The table's one Easter for all is the Eastern one: 12 April 2026.
    HolidayRule::fixed_public(
        "Easter Sunday",
        "عيد الفصح المجيد",
        Rule::paschal(EASTER_SUNDAY),
    ),
    HolidayRule::fixed_public("Labour Day", "عيد العمال", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Independence Day", "عيد الاستقلال", Rule::gregorian(11, 15)),
    HolidayRule::fixed_public(
        "Western Christmas",
        "عيد الميلاد المجيد الغربي",
        Rule::gregorian(12, 25),
    ),
    hijri("Islamic New Year", "رأس السنة الهجرية", 1, 1),
    hijri("Prophet's Birthday", "ذكرى المولد النبوي الشريف", 3, 12),
    hijri("Isra and Mi'raj", "ذكرى الإسراء والمعراج", 7, 27),
    // "The eve and three days", and "the eve and four days".
    HolidayRule::fixed_public(
        "Eve of Eid al-Fitr",
        "وقفة عيد الفطر",
        Rule::Offset {
            base: &PS_SHAWWAL_1,
            days: -1,
        },
    )
    .approximate(),
    hijri("Eid al-Fitr", "عيد الفطر السعيد", 10, 1),
    hijri("Eid al-Fitr", "عيد الفطر السعيد", 10, 2),
    hijri("Eid al-Fitr", "عيد الفطر السعيد", 10, 3),
    hijri("Eve of Eid al-Adha", "وقفة عيد الأضحى", 12, 9),
    hijri("Eid al-Adha", "عيد الأضحى المبارك", 12, 10),
    hijri("Eid al-Adha", "عيد الأضحى المبارك", 12, 11),
    hijri("Eid al-Adha", "عيد الأضحى المبارك", 12, 12),
    hijri("Eid al-Adha", "عيد الأضحى المبارك", 12, 13),
    // The Eastern Christian employees, by the Julian computus.
    ps_christian(
        "Eastern Christmas",
        "عيد الميلاد المجيد",
        Rule::gregorian(1, 8),
    ),
    ps_christian(
        "Eastern New Year",
        "عيد رأس السنة الشرقي",
        Rule::gregorian(1, 14),
    ),
    ps_christian("Eastern Epiphany", "عيد الغطاس", Rule::gregorian(1, 19)),
    ps_christian(
        "Eastern Palm Sunday",
        "أحد الشعانين",
        Rule::paschal(PALM_SUNDAY),
    ),
    ps_christian(
        "Eastern Maundy Thursday",
        "خميس الغسل",
        Rule::paschal(MAUNDY_THURSDAY),
    ),
    ps_christian(
        "Eastern Good Friday",
        "الجمعة العظيمة",
        Rule::paschal(GOOD_FRIDAY),
    ),
    ps_christian(
        "Eastern Holy Saturday",
        "سبت النور",
        Rule::paschal(HOLY_SATURDAY),
    ),
    ps_christian(
        "Eastern Easter Monday",
        "أحد الفصح المجيد",
        Rule::paschal(EASTER_MONDAY),
    ),
    ps_christian("Eastern Ascension", "خميس الصعود", Rule::paschal(ASCENSION)),
    ps_christian("Eastern Pentecost", "أحد العنصرة", Rule::paschal(PENTECOST)),
    // The Western Christian employees, by the Gregorian computus.
    ps_christian("Western Epiphany", "عيد الغطاس", Rule::gregorian(1, 6)),
    ps_christian(
        "Western Palm Sunday",
        "أحد الشعانين",
        Rule::easter(PALM_SUNDAY),
    ),
    ps_christian(
        "Western Maundy Thursday",
        "خميس الغسل",
        Rule::easter(MAUNDY_THURSDAY),
    ),
    ps_christian(
        "Western Good Friday",
        "الجمعة العظيمة",
        Rule::easter(GOOD_FRIDAY),
    ),
    ps_christian(
        "Western Holy Saturday",
        "سبت النور",
        Rule::easter(HOLY_SATURDAY),
    ),
    ps_christian(
        "Western Easter Sunday",
        "أحد الفصح المجيد",
        Rule::easter(EASTER_SUNDAY),
    ),
    ps_christian(
        "Western Easter Monday",
        "أحد الفصح المجيد",
        Rule::easter(EASTER_MONDAY),
    ),
    ps_christian("Western Ascension", "خميس الصعود", Rule::easter(ASCENSION)),
    ps_christian("Western Pentecost", "أحد العنصرة", Rule::easter(PENTECOST)),
    ps_christian(
        "Western Christmas",
        "عيد الميلاد المجيد",
        Rule::gregorian(12, 26),
    ),
];

/// Palestine.
///
/// The Palestinian Authority's Council of Ministers, which approves each
/// year's official holidays for the Government's employees in four tables;
/// the first three are carried as the National Information Centre (WAFA)
/// publishes them for 2025. The first, for everyone: Eid al-Fitr "the eve
/// and three days" and Eid al-Adha "the eve and four days", the Hijri New
/// Year, the Prophet's Birthday and Isra and Mi'raj, on the tabular
/// calendar as approximations of the dates the Council announces — 2026's
/// Fitr ran from Thursday 19 to Sunday 22 March and its Hijri New Year was
/// 16 June, a day before the tabular one; 1 January, 8 March, 1 May,
/// 15 November, both Christmases, and one Easter, which the Council's
/// announcement for Sunday 12 April 2026 shows to be the Eastern one. The
/// second and third tables give the Eastern and the Western Christian
/// employees their own days — Christmas and Easter of two days each, read
/// as the day after and Easter Monday, the Epiphany, Palm Sunday, Holy
/// Week from its Thursday, Ascension and Pentecost, and the Eastern New
/// Year on 14 January — which are [`Kind::Religious`] here; their days that
/// the first table already gives everyone — 1 and 7 January, the Eastern
/// Easter Sunday, 25 December — are not repeated. The fourth,
/// the Samaritans' feasts, is in a calendar the crate lacks and is not
/// carried. Wikipedia's older copy of the list has neither the Easter
/// for all nor the eve of Eid al-Fitr, and the year either was added is
/// not known, so both are carried without a first year, as is every
/// other day. The weekend is Friday and Saturday, as the Centre states
/// for the Government sector; the tables say nothing of a holiday on it,
/// and nothing moves. Days the de facto authorities in Gaza announce are
/// not carried.
pub static PALESTINE: RuleSet = RuleSet {
    code: "PS",
    english_name: "Palestine",
    rules: PS_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: FRIDAY_SATURDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Palestinian National Information Centre (WAFA), \"العطل الرسمية في فلسطين\" \
              (info.wafa.ps/pages/details/29601), retrieved 2026-09-23, for the Council of \
              Ministers' 2025 tables and the working week; the Council's announcements for \
              2026 reported by Al-Dahriyeh Municipality (Easter, 12 April), the National \
              Press Agency (Eid al-Fitr, 19 to 22 March) and An-Najah News (the Hijri New \
              Year, 16 June); Cabinet Decision No. 16 of 2003 on the paid religious and \
              official holidays (maqam.najah.edu); the Arabic Wikipedia, \"قائمة العطل \
              الرسمية في فلسطين\", for the older list. The yearly decisions in the Official \
              Gazette (mjr.ogb.gov.ps) could not be retrieved",
};

// ─────────────────────────────────────────────────────────────────────────
// Libya
// ─────────────────────────────────────────────────────────────────────────

/// Friday alone, and Friday and Saturday for the Government's offices from
/// January 2006; schools and hospitals kept Friday alone.
static LY_WEEKEND: &[WeekendPolicy] = &[
    WeekendPolicy {
        days: &[Weekday::Friday],
        valid_from: None,
        valid_until: Some(2005),
    },
    WeekendPolicy {
        days: &[Weekday::Friday, Weekday::Saturday],
        valid_from: Some(2006),
        valid_until: None,
    },
];

static LY_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "Revolution Day",
        "عيد الثورة الليبية",
        Rule::gregorian(2, 17),
    )
    .years(Some(2012), None),
    HolidayRule::fixed_public("Labour Day", "عيد العمل", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Martyrs' Day", "يوم الشهيد", Rule::gregorian(9, 16)),
    HolidayRule::fixed_public("Liberation Day", "عيد التحرير", Rule::gregorian(10, 23))
        .years(Some(2012), None),
    HolidayRule::fixed_public("Independence Day", "عيد الاستقلال", Rule::gregorian(12, 24)),
    hijri("Islamic New Year", "عيد رأس السنة الهجرية", 1, 1),
    hijri("Prophet's Birthday", "ذكرى المولد النبوي الشريف", 3, 12),
    hijri("Eid al-Fitr", "عيد الفطر المبارك", 10, 1),
    hijri("Eid al-Fitr", "عيد الفطر المبارك", 10, 2),
    hijri("Eid al-Fitr", "عيد الفطر المبارك", 10, 3),
    hijri("Day of Arafah", "يوم الوقوف بعرفة", 12, 9),
    hijri("Eid al-Adha", "عيد الأضحى المبارك", 12, 10),
    hijri("Eid al-Adha", "عيد الأضحى المبارك", 12, 11),
    hijri("Eid al-Adha", "عيد الأضحى المبارك", 12, 12),
];

/// Libya.
///
/// Law 5 of 2012 on the official holidays, issued by the National
/// Transitional Council on 8 January 2012, which repealed law 4 of 1987:
/// its table's Prophet's Birthday, Hijri New Year, three days of Eid
/// al-Fitr, the Day of Arafah and three days of Eid al-Adha, on the
/// tabular calendar as approximations of the dates each year's decision
/// of the Prime Minister fixes; and the Revolution of 17 February, 1 May,
/// Liberation on 23 October, Martyrs' Day on 16 September and
/// Independence on 24 December. The two days of 2011 run from 2012, the
/// law's first year; law 4 of 1987 was not read, so no other day is
/// dated. The law predates the division between the governments in
/// Tripoli and the east, and is the one both issue their holiday
/// decisions under; those yearly decisions, which fix the dates and add
/// days, are not carried. The law says nothing of a holiday on the
/// weekend, and nothing moves. The weekend is Friday and Saturday for the
/// Government's offices from January 2006.
pub static LIBYA: RuleSet = RuleSet {
    code: "LY",
    english_name: "Libya",
    rules: LY_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: LY_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Law No. 5 of 2012 on the official holidays and its table, Official Gazette \
              2012 no. 1, as the Libyan Legal Society's archive (lawsociety.ly) reproduces \
              it, retrieved 2026-09-23, with the Prime Minister's decisions it lists for \
              2021 to 2026; Al-Dustour (3 January 2006) on the Government's Friday and \
              Saturday weekend; Wikipedia, \"Public holidays in Libya\", for the English \
              names",
};

// ─────────────────────────────────────────────────────────────────────────
// Yemen
// ─────────────────────────────────────────────────────────────────────────

/// Thursday and Friday until Council of Ministers resolution 179 of 2013
/// made Saturday the second day in Thursday's place from 15 August 2013.
static YE_WEEKEND: &[WeekendPolicy] = &[
    WeekendPolicy {
        days: &[Weekday::Thursday, Weekday::Friday],
        valid_from: None,
        valid_until: Some(2012),
    },
    WeekendPolicy {
        days: &[Weekday::Friday, Weekday::Saturday],
        valid_from: Some(2013),
        valid_until: None,
    },
];

static YE_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "Labour Day",
        "ذكرى يوم العمال العالمي",
        Rule::gregorian(5, 1),
    ),
    HolidayRule::fixed_public(
        "National Day",
        "اليوم الوطني للجمهورية",
        Rule::gregorian(5, 22),
    ),
    HolidayRule::fixed_public(
        "26 September Revolution Day",
        "ذكرى ثورة 26 سبتمبر",
        Rule::gregorian(9, 26),
    ),
    HolidayRule::fixed_public(
        "14 October Revolution Day",
        "ذكرى ثورة 14 أكتوبر",
        Rule::gregorian(10, 14),
    ),
    HolidayRule::fixed_public(
        "Independence Day",
        "ذكرى يوم الاستقلال",
        Rule::gregorian(11, 30),
    ),
    hijri("Islamic New Year", "ذكرى الهجرة النبوية الشريفة", 1, 1),
    // "From 29 Ramadan to 3 Shawwal", and "from 9 Dhu al-Hijja to the
    // fourth day of the feast".
    hijri("Eid al-Fitr", "عيد الفطر المبارك", 9, 29),
    hijri("Eid al-Fitr", "عيد الفطر المبارك", 9, 30),
    hijri("Eid al-Fitr", "عيد الفطر المبارك", 10, 1),
    hijri("Eid al-Fitr", "عيد الفطر المبارك", 10, 2),
    hijri("Eid al-Fitr", "عيد الفطر المبارك", 10, 3),
    hijri("Eid al-Adha", "عيد الأضحى المبارك", 12, 9),
    hijri("Eid al-Adha", "عيد الأضحى المبارك", 12, 10),
    hijri("Eid al-Adha", "عيد الأضحى المبارك", 12, 11),
    hijri("Eid al-Adha", "عيد الأضحى المبارك", 12, 12),
    hijri("Eid al-Adha", "عيد الأضحى المبارك", 12, 13),
    // Article 3(b): "celebrated without an official holiday".
    HolidayRule::observance("Prophet's Birthday", "ذكرى المولد النبوي الشريف", MAWLID)
        .approximate(),
    HolidayRule::observance(
        "Isra and Mi'raj",
        "ذكرى الإسراء والمعراج",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 7, 27),
    )
    .approximate(),
    HolidayRule::observance("7 July", "ذكرى 7 يوليو", Rule::gregorian(7, 7)),
];

/// Yemen.
///
/// Law 2 of 2000 determining the official leave and holidays, issued at
/// Sana'a on 25 January 2000, which repealed law 42 of 1997, from the
/// Public Prosecution's copy. Article 3(a): Eid al-Fitr "from 29 Ramadan
/// to 3 Shawwal", Eid al-Adha "from 9 Dhu al-Hijja to the fourth day of
/// the feast" and the Hijra on 1 Muharram, on the tabular calendar as
/// approximations of the sighting, which gives five days of each Eid;
/// 22 May, 26 September, 14 October, 30 November and 1 May. Article 3(b)'s
/// days "celebrated without an official holiday" — the Prophet's
/// Birthday, Isra and Mi'raj and 7 July — are observances. Article 4
/// replaces a holiday that "coincides with a Friday or an official
/// holiday" with "another of the working days following the holiday", and
/// is not carried: a policy triggered by Friday alone would, in this
/// engine, land a Friday's replacement on the Saturday that resolution 179
/// of 2013 made a weekend day, and one triggered by Saturday too would
/// replace a Saturday holiday, which the article does not; whether the
/// yearly announcements the Minister of Civil Service makes under
/// article 5 still apply it was not checked either. So nothing moves, and
/// the table says less than the law rather than something else. The law
/// is the unified Republic's, from before the war; the announcements of
/// the internationally recognised Government and of the authorities in
/// Sana'a since then were not read, and nothing either has added is
/// carried. Amendments to the law, if any, were not read. The weekend was
/// Thursday and Friday until 15 August 2013 and is Friday and Saturday
/// since.
pub static YEMEN: RuleSet = RuleSet {
    code: "YE",
    english_name: "Yemen",
    rules: YE_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: YE_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Law No. 2 of 2000 determining the official leave and holidays, articles 3 to \
              7, from the Public Prosecution's legislation library (agoyemen.net), \
              retrieved 2026-09-23; Yemen Post (16 August 2013) on Council of Ministers \
              resolution No. 179 of 2013 and the Friday and Saturday weekend; Wikipedia, \
              \"Public holidays in Yemen\", for the English names",
};

// ─────────────────────────────────────────────────────────────────────────
// Cameroon
// ─────────────────────────────────────────────────────────────────────────

/// Article 2 of law 73/5: "when a civil legal holiday falls on a Sunday or
/// on a holiday, the following day is treated as that holiday" for work
/// and pay. Only the four civil days substitute.
static CM_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: false,
    on_collision: true,
    valid_from: Some(1974),
    valid_until: None,
}];

static CM_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Jour de l'an", Rule::gregorian(1, 1)),
    HolidayRule::public("Youth Day", "Fête de la Jeunesse", Rule::gregorian(2, 11)),
    HolidayRule::fixed_public("Good Friday", "Vendredi saint", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public("Labour Day", "Fête du Travail", Rule::gregorian(5, 1)),
    HolidayRule::public("National Day", "Fête nationale", Rule::gregorian(5, 20)),
    HolidayRule::fixed_public("Ascension", "Ascension", Rule::easter(ASCENSION)),
    HolidayRule::fixed_public("Assumption", "Assomption", Rule::gregorian(8, 15)),
    HolidayRule::fixed_public("Christmas Day", "Noël", Rule::gregorian(12, 25)),
    hijri(
        "Eid al-Fitr",
        "Fête de fin de Ramadan (Djouldé Soumaé)",
        10,
        1,
    ),
    hijri("Eid al-Adha", "Fête du Mouton (Djouldé Laihadji)", 12, 10),
];

/// Cameroon.
///
/// Law 73/5 of 7 December 1973 fixing the regime of the legal holidays,
/// from the Ministry of Public Service's collection, with the changes of
/// law 76/8 of 8 July 1976 as Camerlex summarises the regime: four civil
/// holidays — New Year, Youth Day on 11 February, 1 May and the National
/// Day on 20 May — and six religious ones — Ascension, Good Friday,
/// 15 August, Christmas, "la fête de fin de Ramadan (Djouldé Soumaé)" and
/// "la fête du Mouton (Djouldé Laihadji)", the two Eids on the tabular
/// Hijri calendar as approximations of the sighted dates. Article 2:
/// "when a civil legal holiday falls on a Sunday or on a holiday, the
/// following day is treated as that holiday" for work and pay, which the
/// four civil days do from 1974, the 1972 ordinance the law repealed not
/// having been read. The President's power to declare, by decree each
/// time, the day after a Sunday religious holiday, the eve or the morrow
/// of a holiday on a Friday or a Tuesday, and since 1976 a day for an
/// event of national importance is not carried. Nor is article 5's
/// distinction between the civil days, when stopping work is compulsory,
/// and the religious ones, when in the 1973 text it is not for workers
/// over eighteen: all ten are days off here.
pub static CAMEROON: RuleSet = RuleSet {
    code: "CM",
    english_name: "Cameroon",
    rules: CM_RULES,
    substitution: CM_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Loi n° 73/5 du 7 décembre 1973 fixant le régime des fêtes légales en \
              République Unie du Cameroun, from the Ministry of Public Service's \
              collection (minfopra.gov.cm) as the Internet Archive holds it, retrieved \
              2026-09-23; Camerlex, \"Les jours fériés\", on the regime as loi n° 76/8 \
              du 8 juillet 1976 amended it; NATLEX for the amending law's title",
};

// ─────────────────────────────────────────────────────────────────────────
// Republic of the Congo
// ─────────────────────────────────────────────────────────────────────────

static CG_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Jour de l'an", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Easter Monday",
        "Lundi de Pâques",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::fixed_public("Labour Day", "Fête du travail", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Ascension", "Jeudi de l'Ascension", Rule::easter(ASCENSION)),
    HolidayRule::fixed_public(
        "Whit Monday",
        "Lundi de Pentecôte",
        Rule::easter(WHIT_MONDAY),
    ),
    HolidayRule::fixed_public(
        "Sovereign National Conference Day",
        "Fête de la commémoration de la Conférence nationale souveraine",
        Rule::gregorian(6, 10),
    ),
    HolidayRule::fixed_public("National Day", "Fête nationale", Rule::gregorian(8, 15)),
    HolidayRule::fixed_public("All Saints' Day", "La Toussaint", Rule::gregorian(11, 1)),
    HolidayRule::fixed_public("Christmas Day", "Noël", Rule::gregorian(12, 25)),
];

/// Republic of the Congo.
///
/// Law 2-94 of 1 March 1994 fixing the holidays that are off and paid,
/// from the signed text the Secretariat-General of the Government
/// publishes, which repealed law 43-79 of 1979: nine days, with Easter
/// Monday, Ascension and Whit Monday, and 10 June for the Sovereign
/// National Conference, which closed on 10 June 1991. Article 2 lets the
/// Minister of Labour declare other days off by order "on the occasion of
/// important events", as order 976 of 30 April 2025 did for Friday 2 May
/// 2025, citing law 2-94 as the law in force; those orders are not
/// carried, and neither is 28 November, the Republic's anniversary, which
/// is not in the law's list — whether an order made it a day off, and in
/// which years, was not checked. The law says nothing of a holiday on a
/// Sunday, so nothing moves.
pub static CONGO: RuleSet = RuleSet {
    code: "CG",
    english_name: "Republic of the Congo",
    rules: CG_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Loi n° 2-94 du 1er mars 1994 fixant les jours fériés, chômés et payés, the \
              signed text and Unicongo's note of 2 April 1994, from the Secrétariat \
              général du Gouvernement (sgg.cg), retrieved 2026-09-23; arrêté n° 976 du \
              30 avril 2025, Journal officiel 2025 no. 19, on the same site; Wikipedia, \
              \"Public holidays in the Republic of the Congo\", for the English names",
};

// ─────────────────────────────────────────────────────────────────────────
// Democratic Republic of the Congo
// ─────────────────────────────────────────────────────────────────────────

/// Article 2 of ordinances 14-010 and 23-042: a holiday that "coincides
/// with a Sunday" is taken "the day before", as the Minister's
/// communiqués applied it until 2025.
static CD_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Backward,
    skip_occupied: false,
    on_collision: false,
    valid_from: Some(2014),
    valid_until: Some(2025),
}];

/// Nothing: no communiqué for 2027 or later was read, so a weekend holiday
/// moved by one is a gap in those years.
fn cd_unread(_: i64) -> Days {
    Days::new()
}

static CD_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Nouvel an", Rule::gregorian(1, 1)),
    HolidayRule::public(
        "Martyrs of Independence Day",
        "Journée des Martyrs de l'indépendance",
        Rule::gregorian(1, 4),
    ),
    HolidayRule::public(
        "Laurent-Désiré Kabila Day",
        "Journée du héros national Laurent Désiré Kabila",
        Rule::gregorian(1, 16),
    ),
    HolidayRule::public(
        "Patrice Lumumba Day",
        "Journée du héros national Patrice Emery Lumumba",
        Rule::gregorian(1, 17),
    ),
    HolidayRule::public(
        "Simon Kimbangu Day",
        "Journée du combat de Simon Kimbangu et de la conscience africaine",
        Rule::gregorian(4, 6),
    )
    .years(Some(2023), None),
    HolidayRule::public("Labour Day", "Fête du travail", Rule::gregorian(5, 1)),
    HolidayRule::public(
        "Armed Forces Day",
        "Journée des Forces armées",
        Rule::gregorian(5, 17),
    ),
    HolidayRule::public(
        "Independence Day",
        "Journée de l'indépendance",
        Rule::gregorian(6, 30),
    ),
    HolidayRule::public("Parents' Day", "Fête des parents", Rule::gregorian(8, 1)),
    HolidayRule::public("Christmas Day", "Noël", Rule::gregorian(12, 25)),
    // The Minister's communiqués: a Saturday holiday to the Friday in 2025,
    // and every weekend holiday to the Monday in 2026.
    HolidayRule::fixed_public(
        "Martyrs of Independence Day",
        "Journée des Martyrs de l'indépendance",
        Rule::gregorian(1, 3),
    )
    .years(Some(2025), Some(2025)),
    HolidayRule::fixed_public(
        "Armed Forces Day",
        "Journée des Forces armées",
        Rule::gregorian(5, 16),
    )
    .years(Some(2025), Some(2025)),
    HolidayRule::fixed_public(
        "Martyrs of Independence Day",
        "Journée des Martyrs de l'indépendance",
        Rule::gregorian(1, 5),
    )
    .years(Some(2026), Some(2026)),
    HolidayRule::fixed_public(
        "Patrice Lumumba Day",
        "Journée du héros national Patrice Emery Lumumba",
        Rule::gregorian(1, 19),
    )
    .years(Some(2026), Some(2026)),
    HolidayRule::fixed_public(
        "Armed Forces Day",
        "Journée des Forces armées",
        Rule::gregorian(5, 18),
    )
    .years(Some(2026), Some(2026)),
    HolidayRule::fixed_public("Parents' Day", "Fête des parents", Rule::gregorian(8, 3))
        .years(Some(2026), Some(2026)),
    HolidayRule::fixed_public(
        "Weekend holiday moved by communiqué",
        "Jour férié reporté par communiqué",
        Rule::Tabulated {
            function: cd_unread,
            first_year: 1,
            last_year: 0,
        },
    )
    .years(Some(2027), None),
];

/// Democratic Republic of the Congo.
///
/// Ordinance 23-042 of 30 March 2023 fixing the list of legal holidays,
/// which replaced ordinance 14-010 of 14 May 2014: the 2014 list of nine
/// days, and from 2023 6 April, the day of Simon Kimbangu's struggle and
/// of African consciousness. Article 2 of both: where a holiday "coincides
/// with a Sunday, the leave for that day is taken the day before", the
/// Saturday, carried from 2014 to 2025; the 1979 ordinance the 2014 one
/// replaced was not read. Since decree 24/09 of 17 February 2024 made
/// Saturday a day off in the public service, the Minister of Employment
/// and Labour's communiqués have departed from the ordinance: in 2025 a
/// Saturday holiday was brought forward to the Friday, 3 January and
/// 16 May, while Sunday 6 April still gave Saturday 5 April (and Friday
/// 4 April to the public services, which is not carried); in 2026 the
/// Minister moved Sunday 4 January to Monday 5 January, withdrawing an
/// earlier communiqué that gave the Saturday, and Saturday 17 January,
/// Sunday 17 May and Saturday 1 August to 19 January, 18 May and
/// 3 August. Those days are carried as the communiqués give them, and
/// from 2027, which no communiqué read covers, a weekend holiday's move is
/// reported as a gap. GENOCOST day on 2 August, commemorated since 2024,
/// is not in the ordinance's list, and reports disagree on whether it was
/// a day off in 2024; it is not carried, and neither are the other days
/// the Minister declares off for an occasion.
pub static DR_CONGO: RuleSet = RuleSet {
    code: "CD",
    english_name: "Democratic Republic of the Congo",
    rules: CD_RULES,
    substitution: CD_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Ordonnance n° 23-042 du 30 mars 2023 fixant la liste des jours fériés légaux \
              (J.O. RDC, 15 May 2023) and ordonnance n° 14/010 du 14 mai 2014 (J.O. RDC \
              no. 11, 1 June 2014), from droitcongolais.info, retrieved 2026-09-23; the \
              Ministry of Employment and Labour's communiqués as ACP (2 April and 14 May \
              2025, 12 January 2026), Congo Quotidien (2 January 2025), Opinion Info \
              (31 December 2025, 13 May 2026) and Netic News (27 July 2026) report them; \
              Actualite.cd (5 January 2026) on communiqués 010 and 011 of December 2025 \
              and (13 January 2026) on ordinance 23-042 being in force; Radio Okapi (15 July \
              2024) on GENOCOST",
};

// ─────────────────────────────────────────────────────────────────────────
// Angola
// ─────────────────────────────────────────────────────────────────────────

/// Article 6 of law 10/11 as enacted: a holiday on a Sunday is "transferred
/// to the working day immediately after", except New Year, Carnival, All
/// Souls and Christmas. Law 11/18 replaced it in September 2018; Sunday
/// 4 February 2018, the one case that year before the change, is a rule of
/// its own.
static AO_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: Some(2011),
    valid_until: Some(2017),
}];

/// The national holidays of article 2, each with the first year it is a
/// holiday, for the bridges of article 6 as law 11/18 rewrote it.
static AO_BRIDGED: &[(Rule, i64)] = &[
    (Rule::gregorian(1, 1), i64::MIN),
    (Rule::gregorian(2, 4), i64::MIN),
    (Rule::gregorian(3, 8), i64::MIN),
    (Rule::gregorian(3, 23), 2019),
    (Rule::easter(SHROVE_TUESDAY), i64::MIN),
    (Rule::gregorian(4, 4), i64::MIN),
    (Rule::easter(GOOD_FRIDAY), i64::MIN),
    (Rule::gregorian(5, 1), i64::MIN),
    (Rule::gregorian(9, 17), i64::MIN),
    (Rule::gregorian(11, 2), i64::MIN),
    (Rule::gregorian(11, 11), i64::MIN),
    (Rule::gregorian(12, 25), i64::MIN),
];

/// "When a national holiday falls on a Tuesday or a Thursday, work stops on
/// the working day before or the day immediately after, Monday or Friday
/// respectively": from the law's publication on 28 September 2018.
fn ao_bridges(year: i64) -> Days {
    let mut out = Days::new();
    let Ok(in_force) = gregorian::to_fixed(2018, 9, 28) else {
        return out;
    };
    for (rule, from) in AO_BRIDGED {
        if year < *from {
            continue;
        }
        for date in rule.days_in_year(year).as_slice() {
            if *date < in_force {
                continue;
            }
            match Weekday::from_rd(*date) {
                Weekday::Tuesday => out.push(Rd(date.0 - 1)),
                Weekday::Thursday => out.push(Rd(date.0 + 1)),
                _ => {}
            }
        }
    }
    out
}

static AO_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Dia do Ano Novo", Rule::gregorian(1, 1)),
    HolidayRule::public(
        "Liberation War Day",
        "Dia do Início da Luta Armada de Libertação Nacional",
        Rule::gregorian(2, 4),
    ),
    // The Monday after Sunday 4 February 2018, under article 6 as it stood
    // until 28 September 2018.
    HolidayRule::fixed_public(
        "Liberation War Day",
        "Dia do Início da Luta Armada de Libertação Nacional",
        Rule::gregorian(2, 5),
    )
    .years(Some(2018), Some(2018)),
    HolidayRule::public(
        "International Women's Day",
        "Dia Internacional da Mulher",
        Rule::gregorian(3, 8),
    ),
    HolidayRule::public(
        "Southern Africa Liberation Day",
        "Dia da Libertação da África Austral",
        Rule::gregorian(3, 23),
    )
    .years(Some(2019), None),
    HolidayRule::fixed_public("Carnival", "Dia do Carnaval", Rule::easter(SHROVE_TUESDAY)),
    HolidayRule::public(
        "Peace and National Reconciliation Day",
        "Dia da Paz e da Reconciliação Nacional",
        Rule::gregorian(4, 4),
    ),
    HolidayRule::fixed_public(
        "Good Friday",
        "Sexta-Feira Santa",
        Rule::easter(GOOD_FRIDAY),
    ),
    HolidayRule::public(
        "Labour Day",
        "Dia Internacional do Trabalhador",
        Rule::gregorian(5, 1),
    ),
    HolidayRule::public(
        "National Heroes' Day",
        "Dia do Fundador da Nação e do Herói Nacional",
        Rule::gregorian(9, 17),
    ),
    HolidayRule::fixed_public("All Souls' Day", "Dia dos Finados", Rule::gregorian(11, 2)),
    HolidayRule::public(
        "Independence Day",
        "Dia da Independência",
        Rule::gregorian(11, 11),
    ),
    HolidayRule::fixed_public(
        "Christmas and Family Day",
        "Dia de Natal e da Família",
        Rule::gregorian(12, 25),
    ),
    HolidayRule::fixed_public("Bridge day", "Ponte", Rule::Computed(ao_bridges)),
    // Article 3's national celebration dates, on which "there is no
    // suspension of work".
    HolidayRule::observance(
        "Martyrs of Colonial Repression Day",
        "Dia dos Mártires da Repressão Colonial",
        Rule::gregorian(1, 4),
    ),
    HolidayRule::observance(
        "Veterans' Day",
        "Dia do Antigo Combatente e Veterano da Pátria",
        Rule::gregorian(1, 15),
    )
    .years(Some(2019), None),
    HolidayRule::observance(
        "Angolan Women's Day",
        "Dia da Mulher Angolana",
        Rule::gregorian(3, 2),
    ),
    HolidayRule::observance(
        "Day of the Expansion of the Armed Struggle",
        "Dia da Expansão da Luta Armada de Libertação Nacional",
        Rule::gregorian(3, 15),
    ),
    HolidayRule::observance(
        "Angolan Youth Day",
        "Dia da Juventude Angolana",
        Rule::gregorian(4, 14),
    ),
    HolidayRule::observance("Africa Day", "Dia de África", Rule::gregorian(5, 25)),
    HolidayRule::observance(
        "International Children's Day",
        "Dia Internacional da Criança",
        Rule::gregorian(6, 1),
    ),
    HolidayRule::observance(
        "Human Rights Day",
        "Dia Internacional dos Direitos Humanos",
        Rule::gregorian(12, 10),
    ),
];

/// Angola.
///
/// Law 10/11 of 16 February 2011 on the national and local holidays and
/// the national celebration dates, as law 11/18 of 28 September 2018
/// rewrote its articles 2, 3 and 6, both from AngoLEX: eleven national
/// holidays, with Carnival on Shrove Tuesday, and Southern Africa
/// Liberation Day on 23 March the twelfth from 2019. As enacted, article 6
/// moved a holiday on a Sunday "to the working day immediately after",
/// New Year, Carnival, All Souls and Christmas excepted; that is carried
/// from 2011 to 2017, and for Sunday 4 February 2018 as the Monday. The
/// rewritten article 6 replaced it on publication with a bridge — work
/// stops on the Monday before a Tuesday holiday and on the Friday after a
/// Thursday one — computed from 28 September 2018, which gives every
/// Carnival its Monday from 2019; since then nothing moves off a Sunday.
/// Article 3's national celebration dates, 15 January among them from
/// 2019, are observances, "there being no suspension of work". Article
/// 7's afternoons of 24 and 31 December, the municipal holidays and the
/// Executive's tolerâncias de ponto are not carried. Law 7/03, which law
/// 10/11 repealed, was not read.
pub static ANGOLA: RuleSet = RuleSet {
    code: "AO",
    english_name: "Angola",
    rules: AO_RULES,
    substitution: AO_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Lei n.º 10/11 de 16 de Fevereiro, Lei dos Feriados Nacionais e Locais e Datas \
              de Celebração Nacional, and Lei n.º 11/18 de 28 de Setembro amending its \
              articles 2, 3 and 6, from AngoLEX (angolex.com), retrieved 2026-09-23; \
              lex.ao for the latter's publication in the Diário da República, I Série \
              n.º 147; Novo Jornal on the 2018 vote; the Ministério da Administração do \
              Território's \"Efemérides\" page on the laws in force",
};

// ─────────────────────────────────────────────────────────────────────────
// Rwanda
// ─────────────────────────────────────────────────────────────────────────

/// Article 4 of Presidential Order 54/01 of 2017: a holiday on the weekend
/// makes "the following working day" a holiday; "two consecutive official
/// public holidays" on the weekend "are compensated in one working day that
/// follows"; and two holidays on one day give "the following working day".
/// The search does not go past a day already taken, which is how the 2022
/// holidays were handled: a Sunday Christmas or New Year's Day is followed
/// by a holiday, and the compensation "will not apply", as The New Times
/// reported the Ministry's statement.
static RW_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: false,
    on_collision: true,
    valid_from: Some(2017),
    valid_until: None,
}];

/// A holiday of the 2017 Order, which the table carries from that year.
const fn rw(name: &'static str, rule: Rule) -> HolidayRule {
    HolidayRule::public(name, "", rule).years(Some(2017), None)
}

static RW_RULES: &[HolidayRule] = &[
    rw("New Year's Day", Rule::gregorian(1, 1)),
    rw("Day after New Year's Day", Rule::gregorian(1, 2)),
    rw("National Heroes' Day", Rule::gregorian(2, 1)),
    rw("Good Friday", Rule::easter(GOOD_FRIDAY)),
    rw("Easter Monday", Rule::easter(EASTER_MONDAY)),
    // "Except 07 April": never moved off the weekend.
    HolidayRule::fixed_public(
        "Genocide against the Tutsi Memorial Day",
        "",
        Rule::gregorian(4, 7),
    )
    .years(Some(2017), None),
    rw("Labour Day", Rule::gregorian(5, 1)),
    rw("Independence Day", Rule::gregorian(7, 1)),
    rw("Liberation Day", Rule::gregorian(7, 4)),
    // "Friday of the first week of August": the first Friday, as the
    // Government's list for 2025 gives it, 1 August.
    rw("Umuganura Day", Rule::nth(8, 1, Weekday::Friday)),
    rw("Assumption Day", Rule::gregorian(8, 15)),
    rw("Christmas Day", Rule::gregorian(12, 25)),
    rw("Boxing Day", Rule::gregorian(12, 26)),
    hijri_public("Eid al-Fitr", "", 10, 1).years(Some(2017), None),
    hijri_public("Eid al-Adha", "", 12, 10).years(Some(2017), None),
    // The two days the Ministry "exceptionally designated" for the festive
    // season, when the Sunday Christmas and New Year's Day gave none.
    HolidayRule::fixed_public("Additional public holiday", "", Rule::gregorian(12, 27))
        .years(Some(2022), Some(2022)),
    HolidayRule::fixed_public("Additional public holiday", "", Rule::gregorian(1, 3))
        .years(Some(2023), Some(2023)),
];

/// Rwanda.
///
/// Presidential Order 54/01 of 24 February 2017 determining official
/// public holidays, in force from its publication on 13 March 2017, which
/// repealed Presidential Order 42/03 of 2015: article 3's fifteen days for
/// the public and the private sector alike, Umuganura on the first Friday
/// of August, and Eid al-Fitr and Eid al-Adha, "announced each year by
/// Rwanda Moslems' Association", on the tabular Hijri calendar as
/// approximations. Article 4 makes the following working day a holiday
/// for one that falls on the weekend, 7 April excepted, compensates two
/// consecutive weekend holidays with one day, and two holidays on the same
/// day with one: a forward policy that does not go past a day already
/// taken, so that a Saturday Christmas and Sunday Boxing Day give the one
/// Monday, and a Sunday Christmas, followed by Boxing Day, gives nothing —
/// which is what The New Times reported of 2022 from the statement in
/// which the Ministry of Public Service and Labour "exceptionally
/// designated" 27 December 2022 and 3 January 2023 as additional
/// holidays, carried for those years. RwandaLII records
/// no amendment of the Order. The 2015 Order was not read, so nothing
/// before 2017 is stated. Other days the President or the Ministry
/// declares, such as the one after an inauguration, are not carried; nor
/// is the week of mourning after 7 April, which is not a holiday.
pub static RWANDA: RuleSet = RuleSet {
    code: "RW",
    english_name: "Rwanda",
    rules: RW_RULES,
    substitution: RW_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Presidential Order n° 54/01 of 24/02/2017 determining official public holidays, \
              Official Gazette no. 11 of 13 March 2017, articles 3 and 4, from the Laws.Africa \
              copy on RwandaLII (rwandalii.org/akn/rw/act/po/2017/54), retrieved 2026-09-23; \
              The New Times, \"Rwanda Announces Two Extra Public Holidays as Festive Season \
              Kicks in\" (23 December 2022), as allAfrica published it and the Internet Archive \
              holds it, allAfrica refusing this session's requests, for the Ministry's statement \
              on 27 December 2022 and 3 January 2023; the High Commission in Tanzania's list for \
              2025 (rwandaintanzania.gov.rw) for Umuganura on 1 August",
};

// ─────────────────────────────────────────────────────────────────────────
// Burundi
// ─────────────────────────────────────────────────────────────────────────

static BI_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "New Year's Day",
        "Premier Jour du nouvel an",
        Rule::gregorian(1, 1),
    ),
    HolidayRule::fixed_public(
        "Unity Day",
        "Fête de l'Unité et de la Réconciliation Nationales",
        Rule::gregorian(2, 5),
    ),
    HolidayRule::fixed_public(
        "Commemoration of the Assassination of President Cyprien Ntaryamira",
        "Commémoration de l'Assassinat du Président Cyprien Ntaryamira",
        Rule::gregorian(4, 6),
    ),
    HolidayRule::fixed_public("Ascension Day", "Ascension", Rule::easter(ASCENSION)),
    HolidayRule::fixed_public(
        "Labour Day",
        "Fête Internationale du Travail",
        Rule::gregorian(5, 1),
    ),
    // Pierre Nkurunziza died on 8 June 2020; the decree that made the day a
    // holiday came into force on 7 June 2021.
    HolidayRule::fixed_public(
        "National Patriotism Day",
        "Journée Nationale du Patriotisme et Commémoration de la Mort du Président Pierre Nkurunziza",
        Rule::gregorian(6, 8),
    )
    .years(Some(2021), None),
    HolidayRule::fixed_public(
        "Independence Day",
        "Anniversaire de l'Indépendance",
        Rule::gregorian(7, 1),
    ),
    HolidayRule::fixed_public("Assumption", "Assomption", Rule::gregorian(8, 15)),
    HolidayRule::fixed_public(
        "Rwagasore Day",
        "Commémoration de l'Assassinat du Héros National, le Prince Louis Rwagasore",
        Rule::gregorian(10, 13),
    ),
    HolidayRule::fixed_public(
        "Ndadaye Day",
        "Commémoration de l'Assassinat du Président Melchior Ndadaye",
        Rule::gregorian(10, 21),
    ),
    HolidayRule::fixed_public("All Saints' Day", "Toussaint", Rule::gregorian(11, 1)),
    HolidayRule::fixed_public("Christmas Day", "Noël", Rule::gregorian(12, 25)),
    hijri("Eid al-Fitr", "Aïd-El-Fithr", 10, 1),
    hijri("Eid al-Adha", "Aïd-El-Hadj", 12, 10),
];

/// Burundi.
///
/// Decree 100/150 of 7 June 2021 amending decree 100/182 of 17 July 2006
/// fixing the list and regime of public holidays, from the scan the
/// Presidency published, article 1: the fourteen days "fériés, chômés et
/// payés", Aïd-El-Fithr and Aïd-El-Hadj one day each on the tabular Hijri
/// calendar as approximations. The Ministry of Foreign Affairs' embassy
/// in Algiers lists the same fourteen for the present. The day of 8 June,
/// for the death of Pierre Nkurunziza, runs from 2021, the decree coming
/// into force on its signature the day before; the 2006 decree was not
/// read, so no other day is dated. Article 4 moves a holiday that
/// coincides with a Sunday to the next working day only "lorsqu'il
/// paraîtra inopportun" that it be observed there — a decision each time,
/// not a rule — and nothing is moved. The days declared by decree under
/// article 3 are not carried.
pub static BURUNDI: RuleSet = RuleSet {
    code: "BI",
    english_name: "Burundi",
    rules: BI_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Décret n° 100/150 du 07 juin 2021 portant modification du décret n° 100/182 du \
              17 juillet 2006 fixant la liste et le régime des jours fériés, the Presidency's \
              scan (presidence.gov.bi, Liste-et-regime-des-jours-feries.pdf) as the Internet \
              Archive holds it, the page now answering 404, retrieved 2026-09-23; the Embassy \
              of Burundi in Algiers, \"Jours fériés\" (ambabualgerie.mae.gov.bi), retrieved the \
              same day",
};

// ─────────────────────────────────────────────────────────────────────────
// Madagascar
// ─────────────────────────────────────────────────────────────────────────

/// A day on every one of the yearly decrees read, those for 2023 to 2026,
/// carried from 2023.
const fn mg(name: &'static str, local: &'static str, rule: Rule) -> HolidayRule {
    HolidayRule::fixed_public(name, local, rule).years(Some(2023), None)
}

/// Nothing: the calendar of the Concertation Nationale, the Ministry's
/// date for the day of culture and the electoral calendar were not read.
fn mg_unread(_: i64) -> Days {
    Days::new()
}

/// A day a decree names without its date, which another act fixes.
const MG_UNDATED: Rule = Rule::Tabulated {
    function: mg_unread,
    first_year: 1,
    last_year: 0,
};

static MG_RULES: &[HolidayRule] = &[
    mg("New Year's Day", "Jour de l'An", Rule::gregorian(1, 1)),
    // "Fériée et chômée uniquement pour les femmes".
    HolidayRule::observance(
        "International Women's Day",
        "Journée internationale de la Femme",
        Rule::gregorian(3, 8),
    )
    .years(Some(2023), None),
    mg(
        "Martyrs' Day",
        "Journée commémorative des morts des évènements de 1947",
        Rule::gregorian(3, 29),
    ),
    mg("Easter Sunday", "Pâques", Rule::easter(EASTER_SUNDAY)),
    mg(
        "Easter Monday",
        "Lundi de Pâques",
        Rule::easter(EASTER_MONDAY),
    ),
    mg("Labour Day", "Fête du Travail", Rule::gregorian(5, 1)),
    mg("Ascension Day", "Ascension", Rule::easter(ASCENSION)),
    mg("Pentecost", "Pentecôte", Rule::easter(PENTECOST)),
    mg(
        "Whit Monday",
        "Lundi de Pentecôte",
        Rule::easter(WHIT_MONDAY),
    ),
    mg(
        "Independence Day",
        "Fête nationale de l'Indépendance",
        Rule::gregorian(6, 26),
    ),
    mg("Assumption", "Assomption", Rule::gregorian(8, 15)),
    mg("All Saints' Day", "Toussaint", Rule::gregorian(11, 1)),
    mg("Christmas Day", "Noël", Rule::gregorian(12, 25)),
    hijri("Eid al-Fitr", "Eid Al-Fitr", 10, 1).years(Some(2023), None),
    hijri("Eid al-Adha", "Eid Al-Adha", 12, 10).years(Some(2023), None),
    // Article 2 of the 2024 decree: the communal and legislative election
    // days of the electoral calendar.
    HolidayRule::fixed_public("Election day", "Journée d'élections", MG_UNDATED)
        .years(Some(2024), Some(2024)),
    HolidayRule::fixed_public(
        "Additional public holiday",
        "Journée chômée et payée",
        Rule::gregorian(4, 23),
    )
    .years(Some(2025), Some(2025))
    .cited("Décret n° 2025-415 du 15 avril 2025"),
    // Articles 2 and 3 of the 2026 decree.
    HolidayRule::fixed_public("Malagasy New Year", "Taombaovao Malagasy", MG_UNDATED)
        .years(Some(2026), None),
    HolidayRule::fixed_public(
        "National Culture Day",
        "Journée nationale de la culture",
        MG_UNDATED,
    )
    .years(Some(2026), None),
];

/// Madagascar.
///
/// The decree that fixes each year's list of "jours fériés, chômés et
/// payés" — under article 81 of the 2004 Labour Code for 2023 and 2024
/// and article 115 of Law 2024-014 from 2025 — read for four years:
/// 2023-007, 2024-108, 2025-005 and 2026-006. All four give the same
/// thirteen days, Easter and Pentecost on their Sundays and their Mondays,
/// and 8 March "fériée et chômée uniquement pour les femmes", which the
/// crate cannot scope to part of the workforce and carries as an
/// observance; and Eid al-Fitr and Eid al-Adha "suivant le calendrier fixé
/// par la Communauté musulmane", on the tabular Hijri calendar as
/// approximations. They run from 2023 and are carried forward as the
/// yearly decrees have repeated them; the decrees before 2023 were not
/// read. The days named without a date are gaps: 2024's election days,
/// fixed by the electoral calendar's decree, and from 2026 the Malagasy
/// New Year, on "the calendar fixed at the Concertation Nationale", and
/// the national day of culture, whose date the Ministry of Communication
/// and Culture sets. Decree 2025-415 made 23 April 2025 a day off; other
/// days declared by decree in these years were not searched for. The
/// decrees list Sunday holidays as they fall, and nothing moves.
pub static MADAGASCAR: RuleSet = RuleSet {
    code: "MG",
    english_name: "Madagascar",
    rules: MG_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Décrets n° 2023-007 du 04 janvier 2023, n° 2024-108 du 31 janvier 2024, \
              n° 2025-005 du 07 janvier 2025 and n° 2026-006 du 08 janvier 2026 fixant la liste \
              des jours fériés, chômés et payés au titre de l'année, and décret n° 2025-415 du \
              15 avril 2025, from the Centre National de Législation (cnlegis.gov.mg), retrieved \
              2026-09-23",
};

// ─────────────────────────────────────────────────────────────────────────
// Seychelles
// ─────────────────────────────────────────────────────────────────────────

/// Section 4 of the Public Holidays Act: "where any public holiday, except
/// Sunday, falls on a Sunday the next following day, not being itself a
/// public holiday, shall be a public holiday".
static SC_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static SC_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public("New Year Holiday", "", Rule::gregorian(1, 2)),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Easter Saturday", "", Rule::easter(HOLY_SATURDAY)),
    HolidayRule::fixed_public("Easter Monday", "", Rule::easter(EASTER_MONDAY))
        .years(Some(2017), None),
    HolidayRule::public("Labour Day", "", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Corpus Christi", "Fête Dieu", Rule::easter(CORPUS_CHRISTI)),
    HolidayRule::public("Liberation Day", "", Rule::gregorian(6, 5)).years(None, Some(2016)),
    HolidayRule::public("National Day", "", Rule::gregorian(6, 18)).years(Some(1994), Some(2014)),
    HolidayRule::public("Constitution Day", "", Rule::gregorian(6, 18)).years(Some(2015), None),
    HolidayRule::public("Independence Day", "", Rule::gregorian(6, 29)).years(None, Some(2014)),
    HolidayRule::public("Independence (National) Day", "", Rule::gregorian(6, 29))
        .years(Some(2015), None),
    HolidayRule::public("Assumption Day", "", Rule::gregorian(8, 15)),
    HolidayRule::public("All Saints' Day", "", Rule::gregorian(11, 1)),
    HolidayRule::public("Immaculate Conception", "", Rule::gregorian(12, 8)),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
];

/// Seychelles.
///
/// The Public Holidays Act (Cap. 190) and its Schedule: the 1991 edition,
/// with National Day on 18 June added by Act 5 of 1994, carried from that
/// year; Act 11 of 2014, assented to on 23 July 2014, which renamed
/// National Day Constitution Day and Independence Day "Independence
/// (National) Day", carried from 2015, the first 18 June after it; and Act
/// 3 of 2017, assented to and in force on 11 April 2017, which removed
/// Liberation Day on 5 June, last kept in 2016, and made Easter Monday a
/// holiday "from 2017" — that Act's text was not read, and its effect is
/// the State House's and the Seychelles Nation's report of it, which the
/// Central Bank's list for 2026 bears out. Section 4 moves a Sunday
/// holiday to the next day that is not itself one, a forward policy that
/// makes a Sunday New Year's Day the 3rd; a Saturday holiday stays. The
/// Statute Law Revision Act of 2022 that also amends the Act was not read.
/// The days the President proclaims by order under section 5 — 1 February
/// 2026 by S.I. 2 of 2026, and the election and other days of earlier
/// orders — are not carried.
pub static SEYCHELLES: RuleSet = RuleSet {
    code: "SC",
    english_name: "Seychelles",
    rules: SC_RULES,
    substitution: SC_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Public Holidays Act (Cap. 190, 1991 edition) with the Public Holidays (Amendment) \
              Act, 2014 (Act 11 of 2014), the Public Service Bureau's scan (psb.gov.sc), \
              retrieved 2026-09-23; SeyLII's consolidation at 30 June 2012 as the Internet \
              Archive holds it, SeyLII refusing this session's requests; State House, \
              \"President Assents to Public Holiday (Amendment) Act\", and Seychelles Nation, \
              \"Assembly approves repeal of June 5 as a public holiday, supports Easter \
              Monday\", both 13 April 2017, for Act 3 of 2017; the Central Bank of Seychelles, \
              \"Public Holidays\" for 2026 (cbs.sc); S.I. 2 of 2026 (gazette.sc)",
};

// ─────────────────────────────────────────────────────────────────────────
// Mozambique
// ─────────────────────────────────────────────────────────────────────────

static MZ_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Ano Novo", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Heroes' Day",
        "Dia dos Heróis Moçambicanos",
        Rule::gregorian(2, 3),
    ),
    HolidayRule::fixed_public(
        "Women's Day",
        "Dia da Mulher Moçambicana",
        Rule::gregorian(4, 7),
    ),
    HolidayRule::fixed_public(
        "Workers' Day",
        "Dia Internacional do Trabalhador",
        Rule::gregorian(5, 1),
    ),
    HolidayRule::fixed_public(
        "Independence Day",
        "Dia da Independência Nacional",
        Rule::gregorian(6, 25),
    ),
    HolidayRule::fixed_public(
        "Lusaka Accord Day",
        "Dia dos Acordos de Lusaka",
        Rule::gregorian(9, 7),
    ),
    HolidayRule::fixed_public(
        "Armed Forces Day",
        "Dia das Forças Armadas",
        Rule::gregorian(9, 25),
    ),
    HolidayRule::fixed_public(
        "Peace and Reconciliation Day",
        "Dia da Paz e Reconciliação Nacional",
        Rule::gregorian(10, 4),
    ),
    HolidayRule::fixed_public("Family Day", "Dia da Família", Rule::gregorian(12, 25)),
];

/// Mozambique.
///
/// Article 105 of the Labour Law, Lei n.º 13/2023 of 25 August 2023, in
/// force 180 days after its publication, from the Boletim da República:
/// the nine "feriados obrigatórios" of paragraph 2, with their names in
/// the law. The same nine dates are in the Confederation of Business
/// Associations' table from Lei n.º 23/2007, which the 2023 law repealed,
/// so they carry no first year; the older names are not carried.
/// Paragraph 6 reads, as published, "Sempre que o dia feriado coincida
/// com o domingo, salvo nos casos de actividades laborais previstas no
/// número 4" and stops, stating no consequence; commentaries read into it
/// a move to the next working day, but the law does not say one, and
/// nothing is moved. The "tolerâncias de ponto" the Minister grants under
/// article 106, Good Friday and the Eids among them in practice, and the
/// municipal holidays are not carried.
pub static MOZAMBIQUE: RuleSet = RuleSet {
    code: "MZ",
    english_name: "Mozambique",
    rules: MZ_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Lei n.º 13/2023, de 25 de Agosto, Lei do Trabalho, Boletim da República I série \
              n.º 165, articles 105, 106 and 274, from the Tribunal Supremo's copy (ts.gov.mz), \
              retrieved 2026-09-23; CTA, \"Feriados em Moçambique: Entre a Oportunidade \
              Económica e a Desorganização Produtiva\" (cta.org.mz, May 2025), for the table \
              under Lei n.º 23/2007",
};

// ─────────────────────────────────────────────────────────────────────────
// Lesotho
// ─────────────────────────────────────────────────────────────────────────

/// Nothing: the notices that changed the Schedule between 1996 and 2025
/// were not read, so those years are a gap for the days they touched.
fn ls_unread(_: i64) -> Days {
    Days::new()
}

/// A day whose date in 1996–2025 depends on a notice not read.
const LS_UNREAD: Rule = Rule::Tabulated {
    function: ls_unread,
    first_year: 1,
    last_year: 0,
};

/// A day of the 1995 Act's Schedule that the 2026 list still gives on the
/// same date, carried from the Act's first year.
const fn ls(name: &'static str, rule: Rule) -> HolidayRule {
    HolidayRule::fixed_public(name, "", rule).years(Some(1996), None)
}

static LS_RULES: &[HolidayRule] = &[
    ls("New Year's Day", Rule::gregorian(1, 1)),
    ls("Moshoeshoe's Day", Rule::gregorian(3, 11)),
    ls("Good Friday", Rule::easter(GOOD_FRIDAY)),
    ls("Easter Monday", Rule::easter(EASTER_MONDAY)),
    ls("Workers' Day", Rule::gregorian(5, 1)),
    ls("Ascension Day", Rule::easter(ASCENSION)),
    ls("National Independence Day", Rule::gregorian(10, 4)),
    ls("Christmas Day", Rule::gregorian(12, 25)),
    // The Schedule's Heroes Day on 4 April and King's Birthday on 2 May
    // are not on the 2026 list; its Africa's Heroes' Day, King's Birthday
    // and Boxing Day are not in the Schedule. When each changed is not
    // known, so 1996 to 2025 are a gap for them.
    HolidayRule::fixed_public("Heroes' Day", "", LS_UNREAD).years(Some(1996), Some(2025)),
    HolidayRule::fixed_public("King's Birthday", "", LS_UNREAD).years(Some(1996), Some(2025)),
    HolidayRule::fixed_public("Boxing Day", "", LS_UNREAD).years(Some(1996), Some(2025)),
    HolidayRule::fixed_public("Africa's Heroes' Day", "", Rule::gregorian(5, 25))
        .years(Some(2026), None),
    HolidayRule::fixed_public("King Letsie III's Birthday", "", Rule::gregorian(7, 17))
        .years(Some(2026), None),
    HolidayRule::fixed_public("Boxing Day", "", Rule::gregorian(12, 26)).years(Some(2026), None),
];

/// Lesotho.
///
/// The Public Holidays Act 1995 (Act 7 of 1995), in operation from
/// 1 January 1996, which repealed the Act of 1967: the Schedule's days,
/// and section 3's power of the King, on the Minister of Home Affairs'
/// advice, to appoint others by notice in the Gazette. The Act has no rule
/// for a holiday on the weekend, and nothing moves; the Embassy's list
/// for 2026 leaves Independence Day on its Sunday. Eight of the
/// Schedule's days are on that list on the same dates and are carried from
/// 1996. The other two, Heroes Day on 4 April and the King's Birthday on
/// 2 May, are not, and the list has Africa's Heroes' Day on 25 May, King
/// Letsie III's Birthday on 17 July and Boxing Day instead. The notices
/// that made those changes were not read, so the list's three days are
/// carried from 2026, and 1996 to 2025 are reported as a gap for Heroes'
/// Day, the King's Birthday and Boxing Day rather than given a date. The
/// list's Ascension Day of 29 May is 2025's; the day is carried by
/// its rule, 14 May in 2026. Other days appointed under section 3 are not
/// carried.
pub static LESOTHO: RuleSet = RuleSet {
    code: "LS",
    english_name: "Lesotho",
    rules: LS_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Public Holidays Act 1995 (Act No. 7 of 1995), sections 1 to 6 and the Schedule, \
              CommonLII's copy (pha1995163.pdf) as the Internet Archive holds it, CommonLII and \
              LesothoLII refusing this session's requests; the Embassy of the Kingdom of Lesotho \
              in Washington, \"Public Holidays\" for 2026 (lesothoemb-usa.gov.ls), retrieved \
              2026-09-23",
};

// ─────────────────────────────────────────────────────────────────────────
// Chad
// ─────────────────────────────────────────────────────────────────────────

/// Article 2 of decree 97-413: "when these feasts fall on a Sunday, the
/// following Monday is a holiday, off and paid". Only article 2's days
/// substitute.
static TD_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: false,
    on_collision: false,
    valid_from: Some(1997),
    valid_until: None,
}];

/// Nothing: decree 10-636 of 10 August 2010, which replaced the day of
/// 11 August that year, was not read.
fn td_unread(_: i64) -> Days {
    Days::new()
}

/// A day whose date depends on a decree not read.
const TD_UNREAD: Rule = Rule::Tabulated {
    function: td_unread,
    first_year: 1,
    last_year: 0,
};

static TD_RULES: &[HolidayRule] = &[
    // Article 2: off and paid, and moved off a Sunday.
    HolidayRule::public("New Year's Day", "Jour de l'An", Rule::gregorian(1, 1)),
    HolidayRule::public(
        "International Women's Day",
        "Journée internationale de la femme",
        Rule::gregorian(3, 8),
    )
    .years(Some(2019), None),
    HolidayRule::public("Labour Day", "Fête du Travail", Rule::gregorian(5, 1)),
    HolidayRule::public(
        "Independence Day",
        "Anniversaire de la Proclamation de l'Indépendance, Fête nationale",
        Rule::gregorian(8, 11),
    )
    .years(None, Some(2009)),
    HolidayRule::fixed_public(
        "Independence Day",
        "Anniversaire de la Proclamation de l'Indépendance, Fête nationale",
        TD_UNREAD,
    )
    .years(Some(2010), Some(2010)),
    HolidayRule::public(
        "Independence Day",
        "Anniversaire de la Proclamation de l'Indépendance, Fête nationale",
        Rule::gregorian(8, 11),
    )
    .years(Some(2011), None),
    HolidayRule::public(
        "Freedom and Democracy Day",
        "Journée de la Liberté et de la Démocratie",
        Rule::gregorian(12, 1),
    ),
    // Article 1: off, not paid, and not moved.
    HolidayRule::fixed_public(
        "Easter Monday",
        "Lundi de Pâques",
        Rule::easter(EASTER_MONDAY),
    ),
    HolidayRule::fixed_public(
        "All Saints' Day",
        "Fête de la Toussaint",
        Rule::gregorian(11, 1),
    ),
    HolidayRule::fixed_public(
        "Republic Day",
        "Anniversaire de la Proclamation de la République",
        Rule::gregorian(11, 28),
    ),
    HolidayRule::fixed_public("Christmas Day", "Fête de Noël", Rule::gregorian(12, 25)),
    hijri("Prophet's Birthday", "Maouloud El Nebi", 3, 12),
    hijri("Eid al-Fitr", "Aïd El Fitir", 10, 1),
    hijri("Eid al-Adha", "Aïd El Adha", 12, 10),
];

/// Chad.
///
/// Decree 97-413/PR/MFPT of 30 September 1997 revising the list and regime
/// of the holidays, in force from its signature, from Légitchad's text:
/// article 1's seven days "fériés et chômés", which are not paid and may be
/// made up, and article 2's four days "fériés, chômés et payés", with "when
/// these feasts fall on a Sunday, the following Monday is a holiday, off
/// and paid", which those rules do from 1997; the decree before it, 063 of
/// 1991, was not read, so the Sunday rule is not claimed before it. Decree
/// 273/PR/MFPTDS of 7 March 2019 rewrote article 2 to add 8 March, carried
/// from 2019; its own text was not read, only the press's account of it,
/// which gives the same five days and the same Sunday rule. Légitchad lists
/// decree 10-636 of 10 August 2010 "replacing the day of 11 August" as
/// amending the 1997 decree; it was not read, so 11 August 2010 is a gap
/// rather than a date; the Minister's communiqué of 2015 declared 11 August
/// that year a paid day off. The Islamic days are on the tabular Hijri
/// calendar as approximations of the dates the Minister announces after the
/// sighting. Article 4's exceptional days the President declares are not
/// carried. The weekly rest is on Sunday under decree 56 of 1969; the
/// Saturday of the Saturday–Sunday weekend carried here is not from a
/// source read.
pub static CHAD: RuleSet = RuleSet {
    code: "TD",
    english_name: "Chad",
    rules: TD_RULES,
    substitution: TD_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Décret n° 97-413/PR/MFPT du 30 septembre 1997 portant révision de la liste et du \
              régime des jours fériés et chômés, Légitchad's text (legitchad.cefod-tchad.org/texte/895) \
              as NATLEX holds it (TCD-97323.pdf) through the Internet Archive, retrieved \
              2026-09-23; Tchadinfos, \"Tchad : désormais le 8 mars est déclaré férié, chômé et \
              payé\" (7 March 2019), for décret n° 273/PR/MFPTDS/2019, the decree's own text not \
              read; Le Pays (7 March 2025) on its application; NATLEX's abstract of décret n° 56 of \
              1969 on the weekly rest; Alwihda Info on the Minister's communiqué for 11 August 2015",
};

// ─────────────────────────────────────────────────────────────────────────
// Mauritania
// ─────────────────────────────────────────────────────────────────────────

/// Friday–Saturday until the decree the Council of Ministers adopted on
/// 11 September 2014, and Saturday–Sunday from 1 October 2014. The crate
/// keeps a weekend by whole years, so 2014 is given the new one, as
/// Algeria's 2009 is.
static MR_WEEKEND: &[WeekendPolicy] = &[
    WeekendPolicy {
        days: &[Weekday::Friday, Weekday::Saturday],
        valid_from: None,
        valid_until: Some(2013),
    },
    WeekendPolicy {
        days: &[Weekday::Saturday, Weekday::Sunday],
        valid_from: Some(2014),
        valid_until: None,
    },
];

static MR_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public(
        "New Year's Day",
        "رأس السنة الميلادية",
        Rule::gregorian(1, 1),
    ),
    HolidayRule::fixed_public("Labour Day", "عيد العمال", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public(
        "Africa Liberation Day",
        "يوم تحرير أفريقيا",
        Rule::gregorian(5, 25),
    ),
    HolidayRule::fixed_public("National Day", "العيد الوطني", Rule::gregorian(11, 28)),
    hijri("Islamic New Year", "رأس السنة الهجرية", 1, 1),
    hijri("Prophet's Birthday", "المولد النبوي الشريف", 3, 12),
    hijri("Eid al-Fitr", "عيد الفطر", 10, 1),
    hijri("Eid al-Adha", "عيد الأضحى", 12, 10),
];

/// Mauritania.
///
/// Law 92-018 of 7 December 1992 fixing the legal holidays, from the
/// Ministry of Public Service and Labour's own copy and its 2021
/// collection of the texts in force: the national day on 28 November and
/// seven legal holidays, all "chômées et payées" under article 2. The law
/// names the four Islamic days by the feast alone — "El Mawlid", "El
/// Fitre", "El Adha", "Mouharram" — so each is carried as one day, and
/// "Mouharram" is read as its first day, the New Year, which the law does
/// not say in so many words; all four are on the tabular Hijri calendar
/// as approximations of the sighted dates. The further days the President
/// declares by decree under article 3, as the second day of an Eid often
/// is, are not carried. The law has no weekend rule, and nothing moves.
/// The weekend is Friday–Saturday until 2013 and Saturday–Sunday from
/// 2014, from the press accounts of the decree of 2014, which was not
/// read; when the Friday–Saturday weekend began was not searched for.
pub static MAURITANIA: RuleSet = RuleSet {
    code: "MR",
    english_name: "Mauritania",
    rules: MR_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: MR_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Loi n° 92-018 du 7 décembre 1992 fixant les fêtes légales en Mauritanie, from the \
              Ministère de la Fonction Publique et du Travail (fonctionpublique.gov.mr) and its \
              \"Textes législatifs\" collection of 23 June 2021, retrieved 2026-09-23; Cridem, \
              \"Mauritanie : Le repos hebdomadaire s'aligne sur l'international\" (15 September \
              2014), and Le360 (6 October 2014) on the Council of Ministers' decree of \
              11 September 2014 changing the weekly rest from 1 October 2014",
};

// ─────────────────────────────────────────────────────────────────────────
// Djibouti
// ─────────────────────────────────────────────────────────────────────────

/// Article 97 of the Labour Code: the weekly rest "takes place in
/// principle on Friday"; article 2 of arrêté 2019-193 gives it to all
/// employees at once on the Friday.
static DJ_WEEKEND: &[WeekendPolicy] = &[WeekendPolicy {
    days: &[Weekday::Friday],
    valid_from: None,
    valid_until: None,
}];

/// Nothing: whether arrêté 80-0931, published in 1981, already governed
/// 28 June 1980 was not found out.
fn dj_unread(_: i64) -> Days {
    Days::new()
}

/// A day whose date in one year depends on when an arrêté took effect.
const DJ_UNREAD: Rule = Rule::Tabulated {
    function: dj_unread,
    first_year: 1,
    last_year: 0,
};

/// A day of arrêté 77-347, carried from 1978, its first full year.
const fn dj(name: &'static str, local: &'static str, rule: Rule) -> HolidayRule {
    HolidayRule::fixed_public(name, local, rule).years(Some(1978), None)
}

/// A Hijri day of arrêté 77-347, carried from 1978, on the tabular
/// calendar.
const fn dj_hijri(name: &'static str, local: &'static str, month: u8, day: u8) -> HolidayRule {
    dj(
        name,
        local,
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, month, day),
    )
    .approximate()
}

static DJ_RULES: &[HolidayRule] = &[
    dj("New Year's Day", "Nouvel An", Rule::gregorian(1, 1)),
    dj("Labour Day", "Fête du Travail", Rule::gregorian(5, 1)),
    dj(
        "Independence Day",
        "Fête de l'Indépendance",
        Rule::gregorian(6, 27),
    ),
    HolidayRule::fixed_public(
        "Independence Day (second day)",
        "Fête de l'Indépendance",
        DJ_UNREAD,
    )
    .years(Some(1980), Some(1980)),
    HolidayRule::fixed_public(
        "Independence Day (second day)",
        "Fête de l'Indépendance",
        Rule::gregorian(6, 28),
    )
    .years(Some(1981), None),
    dj("Christmas Day", "Fête de Noël", Rule::gregorian(12, 25)),
    dj_hijri("Islamic New Year", "Awal Mouharam", 1, 1),
    dj_hijri("Prophet's Birthday", "Mouloud", 3, 12),
    dj_hijri("Isra and Mi'raj", "Al Isra et Al Mirague", 7, 27),
    dj_hijri("Eid al-Fitr", "Aïd el-Fitre", 10, 1),
    dj_hijri("Eid al-Fitr (second day)", "Aïd el-Fitre", 10, 2),
    dj_hijri("Eid al-Adha", "Aïd el-Addha", 12, 10),
    dj_hijri("Eid al-Adha (second day)", "Aïd el-Addha", 12, 11),
];

/// Djibouti.
///
/// Arrêté 77-347/INT/AA of 4 October 1977 on the days "fériés, chômés et
/// payés", whose article 1 the rectifying arrêté 80-0931/PR, published on
/// 23 June 1981, quotes in full before replacing its Independence Day line:
/// two days of each Eid, the first of Muharram, New Year's Day, the
/// Mouloud, 1 May, Independence Day on 27 June and Al Isra wal Mi'raj, with
/// Independence Day made "two consecutive days, the 27th and 28th of June
/// of each year". Christmas was added by arrêté 77-609/PR/CAB, and arrêté
/// 78-0226 added it too, worded for 25 December 1977. The list is carried
/// from 1978, the arrêté's first full year, the French-era arrêté
/// 59/90/SPCG before it not having been read. The second day of
/// Independence is carried from 1981; whether the rectifying arrêté,
/// numbered in 1980, already reached 28 June 1980 was not found out, so
/// that day of 1980 is a gap. The Journal officiel's own search finds no
/// later text changing the list. The Islamic days are on the tabular Hijri
/// calendar, the second day of each Eid the day after, as approximations of
/// the dates the sighting fixes. No text read moves a holiday off the
/// weekly rest, and nothing moves. The weekend is the Friday alone: article
/// 97 of the Labour Code puts the weekly rest "in principle on Friday", and
/// arrêté 2019-193 gives it to all employees at once on that day, a two-day
/// rest being left to collective agreements.
pub static DJIBOUTI: RuleSet = RuleSet {
    code: "DJ",
    english_name: "Djibouti",
    rules: DJ_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: DJ_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Arrêté n° 80-0931/PR portant rectificatif de l'arrêté n° 77-347/PR/MI du \
              4.10.1977 règlementant les jours fériés, chômés et payés, arrêté n° 77-609/PR/CAB \
              and arrêté n° 78-0226/MI/AA, from the Journal officiel's eJO \
              (journalofficiel.dj), retrieved 2026-09-23; loi n° 133/AN/05/5ème L portant Code du \
              Travail, articles 97 and 98, and arrêté n° 2019-193/PR/MTRA on the weekly rest, \
              from the same, retrieved the same day",
};
