//! Tables for the Middle East and Africa.
//!
//! Two things in this file exist nowhere else in the crate. The first is the
//! **Friday–Saturday weekend**, and the fact that two of these countries
//! changed theirs inside the last fifteen years: Saudi Arabia moved from
//! Thursday–Friday in June 2013, and the United Arab Emirates to
//! Saturday–Sunday on 1 January 2022. The second is **Israel's Independence
//! Day**, whose statute is a sentence rather than a pattern and which is
//! therefore one of the crate's handful of [`Rule::Computed`] rules.

use hc_calendar::{Month, Rd, Weekday};
use hc_calendars_indic::nakshatra::PUSHYA;
use hc_calendars_solar::gregorian;
use hc_seasons::Meridian;
use hc_seasons::zodiac::{Ayanamsa, SiderealSign};

use crate::computus::offsets::{
    ASCENSION, EASTER_MONDAY, EASTER_SUNDAY, GOOD_FRIDAY, HOLY_SATURDAY, MAUNDY_THURSDAY,
    PALM_SUNDAY, PENTECOST, WHIT_MONDAY,
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

/// Israel.
pub static ISRAEL: RuleSet = RuleSet {
    code: "IL",
    english_name: "Israel",
    rules: IL_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: FRIDAY_SATURDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "חוק יום העצמאות, התש\"ט-1949 and its 2004 amendment; \
              פקודת סדרי השלטון והמשפט for the festival days. Every date is \
              exact, because the Hebrew calendar is arithmetic. Holidays \
              begin at sunset on the preceding evening, which this crate \
              does not model: it names days, not evenings. Friday is a \
              working half-day rather than a full weekend day",
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
/// Two calendars, and the crate can now be honest about both. The civil
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
