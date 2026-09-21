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

use crate::computus::offsets::{EASTER_MONDAY, GOOD_FRIDAY};
use crate::rule::{
    CalendarSystem, Days, HolidayRule, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate,
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
