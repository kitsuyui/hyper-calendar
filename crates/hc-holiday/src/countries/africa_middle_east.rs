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

use crate::computus::offsets::{EASTER_MONDAY, EASTER_SUNDAY, GOOD_FRIDAY, PALM_SUNDAY};
use crate::hindu::DIWALI;
use crate::rule::{
    CalendarSystem, Days, HolidayRule, Kind, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate,
    SubstituteDirection, SubstitutionPolicy, WeekendPolicy,
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
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Presidency of the Council of Ministers, \"الأعياد والعطل الرسمية\", \
              pcm.gov.lb, retrieved 2026-09-22, reproducing Decree 15215 of \
              27 September 2005 and its amendments; the Embassy of Lebanon in \
              Poland's 2026 list, retrieved the same day; Wikipedia, \"Public \
              holidays in Lebanon\", for the English names and Armenian \
              Christmas's 2003 start",
};
