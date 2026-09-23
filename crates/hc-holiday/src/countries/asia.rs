//! Asian national tables, other than Japan.
//!
//! This is where the calendars stop being Gregorian. China, Taiwan, Korea
//! and Vietnam key their great festivals to four *different* lunisolar
//! calendars — the same algorithm at four different meridians — and Chinese
//! New Year, Seollal and Tết are not always the same day. Each table names
//! its own calendar rather than borrowing China's.

use hc_calendar::{Rd, Weekday};
use hc_calendars_indic::places::KATHMANDU;
use hc_calendars_indic::{HinduLunarCalendar, Prevalence};
use hc_calendars_solar::gregorian;
use hc_seasons::SolarTerm;
use hc_seasons::zodiac::Ayanamsa;

use crate::computus::offsets::{
    ASCENSION, EASTER_MONDAY, EASTER_SUNDAY, GOOD_FRIDAY, HOLY_SATURDAY, MAUNDY_THURSDAY,
};
use crate::hindu::{
    BUDDHA_PURNIMA, DIWALI, GURU_NANAK_JAYANTI, HOLI, JANMASHTAMI, MAHAVIR_JAYANTI, RAMA_NAVAMI,
    VIJAYA_DASHAMI,
};
use crate::rule::{
    CalendarSystem, Days, HolidayRule, Kind, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate,
    SubstituteDirection, SubstitutionPolicy, WeekendPolicy, WhenTwice,
};

/// 清明, the fifth solar term, at solar longitude 15°.
const QINGMING: SolarTerm = match SolarTerm::from_degrees(15) {
    Some(term) => term,
    // Unreachable: 15° is a term boundary by construction. The fallback
    // keeps the constant usable in a `const` context without a panic.
    None => SolarTerm::SPRING_EQUINOX,
};

/// The Islamic dates every Muslim-majority table in the crate shares.
///
/// All of them are approximate: the observed date is settled
/// by a crescent sighting, per country, sometimes on the night before.
const EID_AL_FITR: Rule = Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 1);
const EID_AL_ADHA: Rule = Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 10);
const HIJRI_NEW_YEAR: Rule = Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 1, 1);
const MAWLID: Rule = Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 3, 12);

// ─────────────────────────────────────────────────────────────────────────
// China
// ─────────────────────────────────────────────────────────────────────────

static CN_NEW_YEAR: Rule = Rule::in_calendar(CalendarSystem::CHINESE, 1, 1);

static CN_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "元旦", Rule::gregorian(1, 1)),
    // 除夕 was a statutory day 2008–2013, dropped in 2014 and restored by
    // the 2024 revision of the 放假办法 with effect from 2025.
    HolidayRule::fixed_public(
        "Chinese New Year's Eve",
        "除夕",
        Rule::Offset {
            base: &CN_NEW_YEAR,
            days: -1,
        },
    )
    .years(Some(2008), Some(2013)),
    HolidayRule::fixed_public(
        "Chinese New Year's Eve",
        "除夕",
        Rule::Offset {
            base: &CN_NEW_YEAR,
            days: -1,
        },
    )
    .years(Some(2025), None),
    HolidayRule::fixed_public(
        "Spring Festival",
        "春节",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 1),
    ),
    HolidayRule::fixed_public(
        "Spring Festival",
        "春节",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 2),
    ),
    HolidayRule::fixed_public(
        "Spring Festival",
        "春节",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 3),
    )
    .years(None, Some(2007)),
    HolidayRule::fixed_public(
        "Spring Festival",
        "春节",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 3),
    )
    .years(Some(2014), None),
    HolidayRule::fixed_public(
        "Qingming Festival",
        "清明节",
        Rule::SolarTerm {
            term: QINGMING,
            meridian: hc_seasons::Meridian::CHINA,
        },
    )
    .years(Some(2008), None),
    HolidayRule::fixed_public("Labour Day", "劳动节", Rule::gregorian(5, 1)),
    // The May golden week ran 1999–2007, and the 2024 revision gave the day
    // after May Day back from 2025.
    HolidayRule::fixed_public("Labour Day", "劳动节", Rule::gregorian(5, 2))
        .years(Some(1999), Some(2007)),
    HolidayRule::fixed_public("Labour Day", "劳动节", Rule::gregorian(5, 3))
        .years(Some(1999), Some(2007)),
    HolidayRule::fixed_public("Labour Day", "劳动节", Rule::gregorian(5, 2))
        .years(Some(2025), None),
    HolidayRule::fixed_public(
        "Dragon Boat Festival",
        "端午节",
        Rule::in_calendar(CalendarSystem::CHINESE, 5, 5),
    )
    .years(Some(2008), None),
    HolidayRule::fixed_public(
        "Mid-Autumn Festival",
        "中秋节",
        Rule::in_calendar(CalendarSystem::CHINESE, 8, 15),
    )
    .years(Some(2008), None),
    HolidayRule::fixed_public("National Day", "国庆节", Rule::gregorian(10, 1)),
    HolidayRule::fixed_public("National Day", "国庆节", Rule::gregorian(10, 2)),
    HolidayRule::fixed_public("National Day", "国庆节", Rule::gregorian(10, 3)),
];

/// China.
pub static CHINA: RuleSet = RuleSet {
    code: "CN",
    english_name: "China",
    rules: CN_RULES,
    // China has no substitution rule. It has 调休: the State Council
    // publishes a table each autumn that both extends the holidays and
    // designates ordinary weekends as working days. That is an annual
    // administrative act, not a rule, and this crate will not guess it.
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "《全国年节及纪念日放假办法》(国务院令第270号), as revised in \
              1999, 2007, 2013 and 2024. The statutory days are listed; the \
              annual 调休 bridging days are not",
};

// ─────────────────────────────────────────────────────────────────────────
// Taiwan
// ─────────────────────────────────────────────────────────────────────────

static TW_NEW_YEAR: Rule = Rule::in_calendar(CalendarSystem::CHINESE, 1, 1);

static TW_RULES: &[HolidayRule] = &[
    HolidayRule::public(
        "Founding Day of the Republic of China",
        "中華民國開國紀念日",
        Rule::gregorian(1, 1),
    ),
    // The Lunar New Year cluster is never adjusted by the Saturday/Sunday
    // rule: 人事行政總處 extends it instead, by a different number of days
    // each year, and this crate does not guess an administrative decision.
    HolidayRule::fixed_public(
        "Lunar New Year's Eve",
        "農曆除夕",
        Rule::Offset {
            base: &TW_NEW_YEAR,
            days: -1,
        },
    ),
    HolidayRule::fixed_public(
        "Spring Festival",
        "春節",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 1),
    ),
    HolidayRule::fixed_public(
        "Spring Festival",
        "春節",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 2),
    ),
    HolidayRule::fixed_public(
        "Spring Festival",
        "春節",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 3),
    ),
    HolidayRule::public("Peace Memorial Day", "和平紀念日", Rule::gregorian(2, 28))
        .years(Some(1997), None),
    HolidayRule::public("Children's Day", "兒童節", Rule::gregorian(4, 4)).years(Some(2011), None),
    HolidayRule::public(
        "Tomb Sweeping Day",
        "民族掃墓節",
        Rule::SolarTerm {
            term: QINGMING,
            meridian: hc_seasons::Meridian::CHINA,
        },
    ),
    HolidayRule::public("Labour Day", "勞動節", Rule::gregorian(5, 1)),
    HolidayRule::public(
        "Dragon Boat Festival",
        "端午節",
        Rule::in_calendar(CalendarSystem::CHINESE, 5, 5),
    ),
    HolidayRule::public(
        "Mid-Autumn Festival",
        "中秋節",
        Rule::in_calendar(CalendarSystem::CHINESE, 8, 15),
    ),
    HolidayRule::public("Teachers' Day", "孔子誕辰紀念日", Rule::gregorian(9, 28))
        .years(Some(2025), None),
    HolidayRule::public("National Day", "國慶日", Rule::gregorian(10, 10)),
    HolidayRule::public("Retrocession Day", "臺灣光復節", Rule::gregorian(10, 25))
        .years(Some(2025), None),
    HolidayRule::public("Constitution Day", "行憲紀念日", Rule::gregorian(12, 25))
        .years(Some(2025), None),
];

/// Taiwan's adjustment rule: a Saturday holiday is kept the Friday before,
/// a Sunday holiday the Monday after.
static TW_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Nearest,
    skip_occupied: false,
    on_collision: false,
    valid_from: Some(2012),
    valid_until: None,
}];

/// Taiwan.
pub static TAIWAN: RuleSet = RuleSet {
    code: "TW",
    english_name: "Taiwan",
    rules: TW_RULES,
    substitution: TW_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "紀念日及節日實施條例 (2024) and the 紀念日及節日實施辦法 it \
              replaced; 行政院人事行政總處 for the adjustment rule. The \
              annual 調整上班日 that turns a Saturday into a working day is \
              an administrative act and is not modelled",
};

// ─────────────────────────────────────────────────────────────────────────
// South Korea
// ─────────────────────────────────────────────────────────────────────────

static KR_SEOLLAL: Rule = Rule::in_calendar(CalendarSystem::DANGI, 1, 1);
static KR_CHUSEOK: Rule = Rule::in_calendar(CalendarSystem::DANGI, 8, 15);

/// Seollal and Chuseok move only for a Sunday, in every year.
///
/// The reason is that the 관공서 규정 lists Sunday itself as a public
/// holiday, so "overlapping another public holiday" already covers a Sunday
/// and never covered a Saturday for these two festivals. The 2021 extension
/// that added Saturdays applies to the four national days, not to these.
const SUNDAY_ONLY: &[Weekday] = &[Weekday::Sunday];

static KR_RULES: &[HolidayRule] = &[
    // New Year's Day and Memorial Day are the two the 대체공휴일 never
    // reaches.
    HolidayRule::fixed_public("New Year's Day", "신정", Rule::gregorian(1, 1)),
    // Seollal is three days: the eve, the day, and the day after. The
    // three-day form dates from 1989; 1985–88 kept the day alone, under the
    // name 민속의 날.
    HolidayRule::public(
        "Seollal",
        "설날",
        Rule::Offset {
            base: &KR_SEOLLAL,
            days: -1,
        },
    )
    .substituted_from(2014)
    .substitute_on(SUNDAY_ONLY)
    .years(Some(1989), None),
    HolidayRule::public(
        "Seollal",
        "설날",
        Rule::in_calendar(CalendarSystem::DANGI, 1, 1),
    )
    .substituted_from(2014)
    .substitute_on(SUNDAY_ONLY)
    .years(Some(1985), None),
    HolidayRule::public(
        "Seollal",
        "설날",
        Rule::Offset {
            base: &KR_SEOLLAL,
            days: 1,
        },
    )
    .substituted_from(2014)
    .substitute_on(SUNDAY_ONLY)
    .years(Some(1989), None),
    // The four national days came under the 대체공휴일 in July 2021; the
    // first day it produced was 16 August 2021.
    HolidayRule::public("Independence Movement Day", "삼일절", Rule::gregorian(3, 1))
        .substituted_from(2021),
    // 대통령령 제36290호 made 노동절, the renamed 근로자의 날, a public
    // holiday from 1 May 2026, under the 대체공휴일 from the start. Before
    // then it was a paid day off for employees under its own Act, and not a
    // public holiday.
    HolidayRule::public("Labour Day", "노동절", Rule::gregorian(5, 1))
        .substituted_from(2026)
        .years(Some(2026), None),
    HolidayRule::public("Children's Day", "어린이날", Rule::gregorian(5, 5))
        .substituted_from(2014)
        .years(Some(1975), None),
    HolidayRule::public(
        "Buddha's Birthday",
        "부처님 오신 날",
        Rule::in_calendar(CalendarSystem::DANGI, 4, 8),
    )
    .substituted_from(2023)
    .years(Some(1975), None),
    HolidayRule::fixed_public("Memorial Day", "현충일", Rule::gregorian(6, 6)),
    HolidayRule::public("Constitution Day", "제헌절", Rule::gregorian(7, 17))
        .years(Some(1949), Some(2007)),
    // Restored by the same decree, in force for it from 11 May 2026, and
    // substituted like the other national days.
    HolidayRule::public("Constitution Day", "제헌절", Rule::gregorian(7, 17))
        .substituted_from(2026)
        .years(Some(2026), None),
    HolidayRule::public("Liberation Day", "광복절", Rule::gregorian(8, 15)).substituted_from(2021),
    // Chuseok is the fourteenth, fifteenth and sixteenth of the eighth
    // month.
    HolidayRule::public(
        "Chuseok",
        "추석",
        Rule::Offset {
            base: &KR_CHUSEOK,
            days: -1,
        },
    )
    .substituted_from(2014)
    .substitute_on(SUNDAY_ONLY)
    .years(Some(1989), None),
    HolidayRule::public(
        "Chuseok",
        "추석",
        Rule::in_calendar(CalendarSystem::DANGI, 8, 15),
    )
    .substituted_from(2014)
    .substitute_on(SUNDAY_ONLY),
    HolidayRule::public(
        "Chuseok",
        "추석",
        Rule::Offset {
            base: &KR_CHUSEOK,
            days: 1,
        },
    )
    .substituted_from(2014)
    .substitute_on(SUNDAY_ONLY)
    .years(Some(1989), None),
    HolidayRule::public("National Foundation Day", "개천절", Rule::gregorian(10, 3))
        .substituted_from(2021),
    // Hangul Day was a public holiday until 1990, dropped to make room for
    // more working days, and restored in 2013.
    HolidayRule::public("Hangul Day", "한글날", Rule::gregorian(10, 9))
        .years(Some(1949), Some(1990)),
    HolidayRule::public("Hangul Day", "한글날", Rule::gregorian(10, 9))
        .substituted_from(2021)
        .years(Some(2013), None),
    HolidayRule::public("Christmas Day", "성탄절", Rule::gregorian(12, 25)).substituted_from(2023),
    // ── Election days, 제2조제10호의2 ────────────────────────────────────
    // The day of every election held because a term has run out has been a
    // public holiday since 대통령령 제19674호 of 6 September 2006; before,
    // each was designated in turn. None is substituted, and none triggers a
    // substitute by coinciding with another holiday.
    kr_one_off(
        "17th presidential election",
        "제17대 대통령 선거",
        2007,
        12,
        19,
    ),
    kr_one_off(
        "18th National Assembly election",
        "제18대 국회의원 선거",
        2008,
        4,
        9,
    ),
    kr_one_off("5th local elections", "제5회 전국동시지방선거", 2010, 6, 2),
    kr_one_off(
        "19th National Assembly election",
        "제19대 국회의원 선거",
        2012,
        4,
        11,
    ),
    kr_one_off(
        "18th presidential election",
        "제18대 대통령 선거",
        2012,
        12,
        19,
    ),
    kr_one_off("6th local elections", "제6회 전국동시지방선거", 2014, 6, 4),
    kr_one_off(
        "20th National Assembly election",
        "제20대 국회의원 선거",
        2016,
        4,
        13,
    ),
    kr_one_off("7th local elections", "제7회 전국동시지방선거", 2018, 6, 13),
    kr_one_off(
        "21st National Assembly election",
        "제21대 국회의원 선거",
        2020,
        4,
        15,
    ),
    kr_one_off(
        "20th presidential election",
        "제20대 대통령 선거",
        2022,
        3,
        9,
    ),
    kr_one_off("8th local elections", "제8회 전국동시지방선거", 2022, 6, 1),
    kr_one_off(
        "22nd National Assembly election",
        "제22대 국회의원 선거",
        2024,
        4,
        10,
    ),
    kr_one_off("9th local elections", "제9회 전국동시지방선거", 2026, 6, 3),
    // ── Days the government designated, 제2조제11호 ─────────────────────
    // Each by a Cabinet decision. Carried from 2009, the first year the
    // Korea Exchange's closure lists reach, which they were checked
    // against. The two presidential elections that followed a vacancy
    // rather than a term's end are here, not above.
    kr_one_off("Temporary holiday", "임시공휴일", 2015, 8, 14),
    kr_one_off("Temporary holiday", "임시공휴일", 2016, 5, 6),
    kr_one_off(
        "19th presidential election",
        "제19대 대통령 선거",
        2017,
        5,
        9,
    ),
    kr_one_off("Temporary holiday", "임시공휴일", 2017, 10, 2),
    kr_one_off("Temporary holiday", "임시공휴일", 2020, 8, 17),
    kr_one_off("Temporary holiday", "임시공휴일", 2023, 10, 2),
    kr_one_off("Armed Forces Day", "국군의 날", 2024, 10, 1),
    kr_one_off("Temporary holiday", "임시공휴일", 2025, 1, 27),
    kr_one_off(
        "21st presidential election",
        "제21대 대통령 선거",
        2025,
        6,
        3,
    ),
];

/// A day off for one year only.
const fn kr_one_off(
    name: &'static str,
    local_name: &'static str,
    year: i32,
    month: u8,
    day: u8,
) -> HolidayRule {
    HolidayRule::fixed_public(name, local_name, Rule::gregorian(month, day))
        .years(Some(year), Some(year))
}

static KR_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    // "또는 다른 공휴일과 겹칠 경우": two holidays on one day earn a third.
    // 5 May 2025 was Children's Day and Buddha's Birthday at once, and
    // 6 May was the 대체공휴일.
    on_collision: true,
    valid_from: Some(2014),
    valid_until: None,
}];

/// South Korea.
pub static SOUTH_KOREA: RuleSet = RuleSet {
    code: "KR",
    english_name: "South Korea",
    rules: KR_RULES,
    substitution: KR_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "관공서의 공휴일에 관한 규정 (대통령령), as in force from \
              11 May 2026 (대통령령 제36290호) and its earlier texts: the \
              2006 amendment making election days holidays, the 2013 \
              amendment introducing 대체공휴일 for Seollal, Chuseok and \
              Children's Day, the July 2021 extension to the national \
              days, the 2023 extension to Buddha's Birthday and \
              Christmas, and the 2026 addition of 노동절 and the \
              restoration of 제헌절. The designated days from 2009 were \
              checked against the Korea Exchange's closure lists; days \
              designated later than the table was read are not carried. \
              Seollal and Chuseok are dated in the `dangi` \
              calendar, computed at the Seoul meridian, which puts them a \
              day away from the Chinese dates a few times a century",
};

// ─────────────────────────────────────────────────────────────────────────
// India
// ─────────────────────────────────────────────────────────────────────────

static IN_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("Republic Day", "गणतंत्र दिवस", Rule::gregorian(1, 26))
        .years(Some(1950), None),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::fixed_public("Independence Day", "स्वतंत्रता दिवस", Rule::gregorian(8, 15))
        .years(Some(1947), None),
    HolidayRule::fixed_public("Gandhi Jayanti", "गांधी जयंती", Rule::gregorian(10, 2)),
    HolidayRule::fixed_public("Christmas Day", "", Rule::gregorian(12, 25)),
    HolidayRule::fixed_public("Id-ul-Fitr", "ईद उल-फ़ित्र", EID_AL_FITR).approximate(),
    HolidayRule::fixed_public("Id-ul-Zuha", "ईद उल-अज़हा", EID_AL_ADHA).approximate(),
    HolidayRule::fixed_public(
        "Muharram",
        "मुहर्रम",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 1, 10),
    )
    .approximate(),
    HolidayRule::fixed_public("Milad-un-Nabi", "ईद मिलाद उन-नबी", MAWLID).approximate(),
    // The gazetted Hindu, Jain, Buddhist and Sikh days, on the Hindu
    // lunisolar calendar as the Rashtriya Panchang keeps it.
    HolidayRule::fixed_public("Holi", "होली", HOLI),
    HolidayRule::fixed_public("Ram Navami", "राम नवमी", RAMA_NAVAMI),
    HolidayRule::fixed_public("Mahavir Jayanti", "महावीर जयंती", MAHAVIR_JAYANTI),
    HolidayRule::fixed_public("Buddha Purnima", "बुद्ध पूर्णिमा", BUDDHA_PURNIMA),
    HolidayRule::fixed_public("Janmashtami", "जन्माष्टमी", JANMASHTAMI),
    HolidayRule::fixed_public("Dussehra", "दशहरा", VIJAYA_DASHAMI),
    HolidayRule::fixed_public("Diwali", "दीपावली", DIWALI),
    HolidayRule::fixed_public("Guru Nanak's Birthday", "गुरु नानक जयंती", GURU_NANAK_JAYANTI),
];

/// India: the three national holidays and the gazetted days of the central
/// government's list — Christian, Muslim, Hindu, Jain, Buddhist and Sikh.
pub static INDIA: RuleSet = RuleSet {
    code: "IN",
    english_name: "India",
    rules: IN_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Department of Personnel and Training, \"List of Holidays\", \
              issued annually, for the list; the Rashtriya Panchang for the \
              Hindu, Jain, Buddhist and Sikh dates, computed on the \
              `hindu-lunar` calendar as it keeps them. The Hijri-dated days \
              are approximate, as everywhere",
};

// ─────────────────────────────────────────────────────────────────────────
// Thailand
// ─────────────────────────────────────────────────────────────────────────

/// Thai Buddhist observances are dated by the Thai lunar calendar, which
/// this crate does not implement. The nearest thing it has is the Chinese
/// lunisolar calendar, whose month *n* is usually the Thai month *n + 2*, so
/// Makha Bucha is the full moon of Chinese month 1, Visakha Bucha of month
/// 4 and Asalha Bucha of month 6. The approximation is right in most years
/// and a month out in a Thai intercalary year, so every one of them is
/// flagged approximate.
static TH_ASALHA: Rule = Rule::in_calendar(CalendarSystem::CHINESE, 6, 15);

static TH_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "วันขึ้นปีใหม่", Rule::gregorian(1, 1)),
    HolidayRule::public(
        "Makha Bucha",
        "วันมาฆบูชา",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 15),
    )
    .approximate(),
    HolidayRule::public("Chakri Memorial Day", "วันจักรี", Rule::gregorian(4, 6)),
    HolidayRule::fixed_public("Songkran", "วันสงกรานต์", Rule::gregorian(4, 13)),
    HolidayRule::fixed_public("Songkran", "วันสงกรานต์", Rule::gregorian(4, 14)),
    HolidayRule::fixed_public("Songkran", "วันสงกรานต์", Rule::gregorian(4, 15)),
    HolidayRule::public("Labour Day", "วันแรงงานแห่งชาติ", Rule::gregorian(5, 1)),
    HolidayRule::public("Coronation Day", "วันฉัตรมงคล", Rule::gregorian(5, 4))
        .years(Some(2019), None),
    HolidayRule::public(
        "Visakha Bucha",
        "วันวิสาขบูชา",
        Rule::in_calendar(CalendarSystem::CHINESE, 4, 15),
    )
    .approximate(),
    HolidayRule::public(
        "Queen Suthida's Birthday",
        "วันเฉลิมพระชนมพรรษาสมเด็จพระนางเจ้าฯ",
        Rule::gregorian(6, 3),
    )
    .years(Some(2019), None),
    HolidayRule::public("Asalha Bucha", "วันอาสาฬหบูชา", TH_ASALHA).approximate(),
    HolidayRule::fixed_public(
        "Khao Phansa",
        "วันเข้าพรรษา",
        Rule::Offset {
            base: &TH_ASALHA,
            days: 1,
        },
    )
    .approximate(),
    HolidayRule::public(
        "King Vajiralongkorn's Birthday",
        "วันเฉลิมพระชนมพรรษา",
        Rule::gregorian(7, 28),
    )
    .years(Some(2017), None),
    HolidayRule::public(
        "Queen Mother's Birthday",
        "วันแม่แห่งชาติ",
        Rule::gregorian(8, 12),
    ),
    HolidayRule::public(
        "Passing of King Bhumibol",
        "วันคล้ายวันสวรรคต",
        Rule::gregorian(10, 13),
    )
    .years(Some(2017), None),
    HolidayRule::public("Chulalongkorn Day", "วันปิยมหาราช", Rule::gregorian(10, 23)),
    HolidayRule::public(
        "King Bhumibol's Birthday",
        "วันพ่อแห่งชาติ",
        Rule::gregorian(12, 5),
    ),
    HolidayRule::public("Constitution Day", "วันรัฐธรรมนูญ", Rule::gregorian(12, 10)),
    HolidayRule::public("New Year's Eve", "วันสิ้นปี", Rule::gregorian(12, 31)),
];

static TH_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// Thailand.
pub static THAILAND: RuleSet = RuleSet {
    code: "TH",
    english_name: "Thailand",
    rules: TH_RULES,
    substitution: TH_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Bank of Thailand's annual list of financial-institution \
              holidays and the Royal Gazette announcements behind it. The \
              four Buddhist observances are approximated from the Chinese \
              lunisolar calendar and can be a month out in a Thai \
              intercalary year; the Royal Ploughing Ceremony, whose date the \
              palace fixes each year, is not modelled",
};

// ─────────────────────────────────────────────────────────────────────────
// Vietnam
// ─────────────────────────────────────────────────────────────────────────

static VN_TET: Rule = Rule::in_calendar(CalendarSystem::VIETNAMESE, 1, 1);

static VN_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Tết Dương lịch", Rule::gregorian(1, 1)),
    // Tết is five statutory days and the Prime Minister fixes which five,
    // so the weekend make-up rule is not applied to them here.
    HolidayRule::fixed_public(
        "Tết",
        "Tết Nguyên Đán",
        Rule::Offset {
            base: &VN_TET,
            days: -1,
        },
    ),
    HolidayRule::fixed_public(
        "Tết",
        "Tết Nguyên Đán",
        Rule::in_calendar(CalendarSystem::VIETNAMESE, 1, 1),
    ),
    HolidayRule::fixed_public(
        "Tết",
        "Tết Nguyên Đán",
        Rule::in_calendar(CalendarSystem::VIETNAMESE, 1, 2),
    ),
    HolidayRule::fixed_public(
        "Tết",
        "Tết Nguyên Đán",
        Rule::in_calendar(CalendarSystem::VIETNAMESE, 1, 3),
    ),
    HolidayRule::fixed_public(
        "Tết",
        "Tết Nguyên Đán",
        Rule::in_calendar(CalendarSystem::VIETNAMESE, 1, 4),
    ),
    HolidayRule::public(
        "Hùng Kings' Festival",
        "Giỗ Tổ Hùng Vương",
        Rule::in_calendar(CalendarSystem::VIETNAMESE, 3, 10),
    )
    .years(Some(2007), None),
    HolidayRule::public(
        "Reunification Day",
        "Ngày Giải phóng miền Nam",
        Rule::gregorian(4, 30),
    ),
    HolidayRule::public("Labour Day", "Ngày Quốc tế Lao động", Rule::gregorian(5, 1)),
    HolidayRule::public("National Day", "Quốc khánh", Rule::gregorian(9, 2)),
    HolidayRule::public("National Day", "Quốc khánh", Rule::gregorian(9, 1))
        .years(Some(2021), None),
];

static VN_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// Vietnam.
pub static VIETNAM: RuleSet = RuleSet {
    code: "VN",
    english_name: "Vietnam",
    rules: VN_RULES,
    substitution: VN_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Bộ luật Lao động 2019, điều 112. Tết is five statutory days; \
              which five is fixed by the Prime Minister each year, and the \
              eve-plus-four split used here is the usual one. The second \
              National Day holiday may be 1 or 3 September by the same \
              annual decision",
};

// ─────────────────────────────────────────────────────────────────────────
// Indonesia
// ─────────────────────────────────────────────────────────────────────────

static ID_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Tahun Baru Masehi", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public(
        "Chinese New Year",
        "Tahun Baru Imlek",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 1),
    )
    .years(Some(2003), None),
    HolidayRule::fixed_public(
        "Isra and Mi'raj",
        "Isra Mikraj",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 7, 27),
    )
    .approximate(),
    HolidayRule::fixed_public(
        "Good Friday",
        "Wafat Isa Almasih",
        Rule::easter(GOOD_FRIDAY),
    ),
    HolidayRule::fixed_public("Labour Day", "Hari Buruh", Rule::gregorian(5, 1))
        .years(Some(2014), None),
    HolidayRule::fixed_public("Ascension", "Kenaikan Isa Almasih", Rule::easter(ASCENSION)),
    HolidayRule::fixed_public(
        "Vesak",
        "Hari Raya Waisak",
        Rule::in_calendar(CalendarSystem::CHINESE, 4, 15),
    )
    .approximate(),
    HolidayRule::fixed_public(
        "Pancasila Day",
        "Hari Lahir Pancasila",
        Rule::gregorian(6, 1),
    )
    .years(Some(2017), None),
    HolidayRule::fixed_public("Eid al-Fitr", "Idul Fitri", EID_AL_FITR).approximate(),
    HolidayRule::fixed_public(
        "Eid al-Fitr",
        "Idul Fitri",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 2),
    )
    .approximate(),
    HolidayRule::fixed_public("Eid al-Adha", "Idul Adha", EID_AL_ADHA).approximate(),
    HolidayRule::fixed_public("Islamic New Year", "Tahun Baru Islam", HIJRI_NEW_YEAR).approximate(),
    HolidayRule::fixed_public(
        "Independence Day",
        "Hari Kemerdekaan",
        Rule::gregorian(8, 17),
    )
    .years(Some(1945), None),
    HolidayRule::fixed_public("Mawlid", "Maulid Nabi Muhammad", MAWLID).approximate(),
    HolidayRule::fixed_public("Christmas Day", "Hari Raya Natal", Rule::gregorian(12, 25)),
];

/// Indonesia.
pub static INDONESIA: RuleSet = RuleSet {
    code: "ID",
    english_name: "Indonesia",
    rules: ID_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Surat Keputusan Bersama of the three ministries, issued \
              annually. The Islamic dates are the civil tabular computation, \
              not the sighting Indonesia actually follows, and Nyepi — the \
              Balinese Saka new year — is absent because this crate has no \
              Balinese calendar. The cuti bersama days are an annual \
              decision and are not modelled",
};

// ─────────────────────────────────────────────────────────────────────────
// Singapore, Malaysia, the Philippines
// ─────────────────────────────────────────────────────────────────────────

static SG_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::public(
        "Chinese New Year",
        "农历新年",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 1),
    ),
    HolidayRule::public(
        "Chinese New Year",
        "农历新年",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 2),
    ),
    HolidayRule::public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public("Hari Raya Puasa", "", EID_AL_FITR).approximate(),
    HolidayRule::public("Labour Day", "", Rule::gregorian(5, 1)),
    HolidayRule::public(
        "Vesak Day",
        "",
        Rule::in_calendar(CalendarSystem::CHINESE, 4, 15),
    )
    .approximate(),
    HolidayRule::public("Hari Raya Haji", "", EID_AL_ADHA).approximate(),
    HolidayRule::public("National Day", "", Rule::gregorian(8, 9)).years(Some(1965), None),
    HolidayRule::public("Deepavali", "", DIWALI).approximate(),
    HolidayRule::public("Christmas Day", "", Rule::gregorian(12, 25)),
];

/// Singapore substitutes only for a Sunday: a Saturday public holiday is
/// simply lost.
static SG_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// Singapore.
pub static SINGAPORE: RuleSet = RuleSet {
    code: "SG",
    english_name: "Singapore",
    rules: SG_RULES,
    substitution: SG_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Holidays Act 1998, schedule; Ministry of Manpower's annual \
              list (mom.gov.sg), whose Deepavali for 2020 to 2026 the crate's \
              Dīpāvalī rule gives, carried approximate as the Ministry \
              announces the day; the two Islamic days are the tabular \
              computation and not the MUIS announcement",
};

static MY_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Tahun Baru", Rule::gregorian(1, 1)),
    HolidayRule::public(
        "Chinese New Year",
        "Tahun Baru Cina",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 1),
    ),
    HolidayRule::public(
        "Chinese New Year",
        "Tahun Baru Cina",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 2),
    ),
    HolidayRule::public("Hari Raya Aidilfitri", "", EID_AL_FITR).approximate(),
    HolidayRule::public(
        "Hari Raya Aidilfitri",
        "",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 2),
    )
    .approximate(),
    HolidayRule::public("Labour Day", "Hari Pekerja", Rule::gregorian(5, 1)),
    HolidayRule::public(
        "Wesak Day",
        "Hari Wesak",
        Rule::in_calendar(CalendarSystem::CHINESE, 4, 15),
    )
    .approximate(),
    HolidayRule::public(
        "Agong's Birthday",
        "Hari Keputeraan Agong",
        Rule::nth(6, 1, Weekday::Monday),
    )
    .years(Some(2017), None),
    HolidayRule::public("Hari Raya Haji", "", EID_AL_ADHA).approximate(),
    HolidayRule::public("Awal Muharram", "", HIJRI_NEW_YEAR).approximate(),
    HolidayRule::public("National Day", "Hari Kebangsaan", Rule::gregorian(8, 31))
        .years(Some(1957), None),
    HolidayRule::public("Malaysia Day", "Hari Malaysia", Rule::gregorian(9, 16))
        .years(Some(2010), None),
    HolidayRule::public("Mawlid", "Maulidur Rasul", MAWLID).approximate(),
    HolidayRule::public("Deepavali", "Hari Deepavali", DIWALI).approximate(),
    HolidayRule::public("Christmas Day", "Hari Krismas", Rule::gregorian(12, 25)),
];

static MY_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// Malaysia, federal holidays only.
pub static MALAYSIA: RuleSet = RuleSet {
    code: "MY",
    english_name: "Malaysia",
    rules: MY_RULES,
    substitution: MY_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Holidays Act 1951, schedule; the federal gazette's annual \
              list, with Office Holidays' copies of it for 2023 to 2025. \
              Deepavali, which the schedule keeps everywhere but Sarawak, is \
              the crate's Dīpāvalī rule, carried approximate as the gazette \
              announces the day. State holidays are not modelled — Thaipusam \
              among them, though the crate now has its rule — and neither is \
              the Friday–Saturday weekend of Johor, Kedah, Kelantan and \
              Terengganu",
};

static PH_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "Bagong Taon", Rule::gregorian(1, 1)),
    HolidayRule::public(
        "Chinese New Year",
        "",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 1),
    )
    .of_kind(Kind::Bank)
    .years(Some(2012), None),
    HolidayRule::fixed_public(
        "Maundy Thursday",
        "Huwebes Santo",
        Rule::easter(MAUNDY_THURSDAY),
    ),
    HolidayRule::fixed_public("Good Friday", "Biyernes Santo", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public("Black Saturday", "Sabado de Gloria", Rule::easter(-1)).of_kind(Kind::Bank),
    HolidayRule::fixed_public("Day of Valour", "Araw ng Kagitingan", Rule::gregorian(4, 9)),
    HolidayRule::fixed_public("Labor Day", "Araw ng mga Manggagawa", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public(
        "Independence Day",
        "Araw ng Kalayaan",
        Rule::gregorian(6, 12),
    ),
    HolidayRule::fixed_public("Eid'l Fitr", "", EID_AL_FITR).approximate(),
    HolidayRule::fixed_public("Eid'l Adha", "", EID_AL_ADHA).approximate(),
    HolidayRule::public("Ninoy Aquino Day", "", Rule::gregorian(8, 21)).of_kind(Kind::Bank),
    HolidayRule::fixed_public(
        "National Heroes Day",
        "Araw ng mga Bayani",
        Rule::last(8, Weekday::Monday),
    )
    .years(Some(2007), None),
    HolidayRule::public(
        "All Saints' Day",
        "Araw ng mga Patay",
        Rule::gregorian(11, 1),
    )
    .of_kind(Kind::Bank),
    HolidayRule::fixed_public(
        "Bonifacio Day",
        "Araw ni Bonifacio",
        Rule::gregorian(11, 30),
    ),
    HolidayRule::public("Immaculate Conception", "", Rule::gregorian(12, 8))
        .of_kind(Kind::Bank)
        .years(Some(2019), None),
    HolidayRule::fixed_public("Christmas Day", "Pasko", Rule::gregorian(12, 25)),
    HolidayRule::fixed_public("Rizal Day", "Araw ni Rizal", Rule::gregorian(12, 30)),
    HolidayRule::public("Last Day of the Year", "", Rule::gregorian(12, 31)).of_kind(Kind::Bank),
];

/// The Philippines.
pub static PHILIPPINES: RuleSet = RuleSet {
    code: "PH",
    english_name: "Philippines",
    rules: PH_RULES,
    // Republic Act 9492 provides for moving holidays to the nearest Monday,
    // but the President suspends or applies it by proclamation year by year,
    // so there is no rule to encode.
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Administrative Code of 1987 as amended by Republic Act 9492 \
              and Republic Act 9849; the annual Malacañang proclamation. \
              Special (non-working) days are recorded as bank holidays; the \
              two Islamic days are the tabular computation, while the \
              proclaimed dates follow the National Commission on Muslim \
              Filipinos",
};

// ─────────────────────────────────────────────────────────────────────────
// Nepal
// ─────────────────────────────────────────────────────────────────────────

/// Nepal kept a one-day weekend — Saturday alone — until the government
/// extended it to Saturday and Sunday in April 2026. It is the reason the
/// engine takes weekend days as data rather than assuming Saturday and
/// Sunday, and the reason a weekend rule carries years like everything else.
static NP_WEEKEND: &[WeekendPolicy] = &[
    WeekendPolicy {
        days: &[Weekday::Saturday],
        valid_from: None,
        valid_until: Some(2025),
    },
    // The government extended the weekly holiday to Saturday and Sunday for
    // government offices and schools in April 2026.
    WeekendPolicy {
        days: &[Weekday::Saturday, Weekday::Sunday],
        valid_from: Some(2026),
        valid_until: None,
    },
];

/// A day the notices date in the Bikram Sambat.
const fn np_bs(name: &'static str, local: &'static str, month: u8, day: u8) -> HolidayRule {
    HolidayRule::fixed_public(
        name,
        local,
        Rule::in_calendar(CalendarSystem::BIKRAM_SAMBAT, month, day),
    )
}

/// Nepal's festivals are read at Kathmandu's sunrise with the Lahiri
/// ayanamsa: the tithis of the modern Moon and Sun, as the Bikram Sambat's
/// months are not (`hc_calendars_indic::bikram_sambat`).
const NP_LUNAR: HinduLunarCalendar = HinduLunarCalendar::new(KATHMANDU, Ayanamsa::LAHIRI);

/// A tithi of an amānta month, read at Kathmandu.
///
/// The part of the day each festival's tithi must hold is fitted to the
/// notices of 2080 to 2083 BS, not quoted from the almanac, so every
/// festival dated this way is flagged approximate, as Thaipusam is. Where
/// the Indian rule for the same festival fits all four years, it is kept;
/// where it does not and several parts of the day do, sunrise is taken,
/// the tithi the day carries.
const fn np_tithi(month: u8, tithi: u8, prevails: Prevalence, when_twice: WhenTwice) -> Rule {
    Rule::Tithi {
        month,
        tithi,
        prevails,
        when_twice,
        calendar: NP_LUNAR,
    }
}

/// Phūlpātī, the seventh day of Dashain: Āśvina śukla 7, at midday — at
/// sunrise it gives 18 October 2026, the day after the notice's.
static NP_PHULPATI: Rule = np_tithi(7, 7, Prevalence::Midday, WhenTwice::Earlier);
/// The last day of the Dashain holiday, Āśvina śukla 12, "द्वादशी".
static NP_DASHAIN_DWADASHI: Rule = np_tithi(7, 12, Prevalence::Sunrise, WhenTwice::Earlier);
/// Lakṣmī Pūjā, the new moon of Āśvina in the evening, the first day of
/// the Tihar holiday. When the new moon holds two evenings Nepal keeps the
/// first — 31 October 2024, where India's Dīpāvalī was 1 November.
static NP_LAXMI_PUJA: Rule = np_tithi(7, 30, Prevalence::Evening, WhenTwice::Earlier);
/// Bhai Tika, Kārtika śukla 2.
static NP_BHAI_TIKA: Rule = np_tithi(8, 2, Prevalence::Sunrise, WhenTwice::Earlier);
/// The day after Bhai Tika, the last day of the Tihar holiday.
static NP_TIHAR_LAST: Rule = Rule::Offset {
    base: &NP_BHAI_TIKA,
    days: 1,
};

/// A festival day from the notice's section 2.1, 7.1 or both.
const fn np_festival(name: &'static str, local: &'static str, rule: Rule) -> HolidayRule {
    HolidayRule::fixed_public(name, local, rule).approximate()
}

/// The days the Ministry of Home Affairs' notice gives every office in the
/// country, and which fall on a fixed date: in the Bikram Sambat for the
/// national days and the two that open a month, in the Gregorian calendar
/// for the three the notice dates that way — it prints "(मे १)", "(मार्च
/// ८)" and "(डिसेम्बर २५)" beside them. Local names are the notice's.
static NP_RULES: &[HolidayRule] = &[
    np_bs("Nepali New Year", "नव वर्ष", 1, 1),
    HolidayRule::fixed_public("Labour Day", "विश्व मजदुर दिवस", Rule::gregorian(5, 1)),
    np_bs("Republic Day", "गणतन्त्र दिवस", 2, 15),
    np_bs("Constitution Day", "संविधान दिवस", 6, 3),
    HolidayRule::fixed_public("Christmas Day", "क्रिसमस डे", Rule::gregorian(12, 25)),
    np_bs("Prithvi Jayanti", "पृथ्वी जयन्ती", 9, 27),
    np_bs("Maghe Sankranti", "माघे सङ्क्रान्ति", 10, 1),
    np_bs("Martyrs' Day", "सहिद दिवस", 10, 16),
    np_bs("National Democracy Day", "राष्ट्रिय प्रजातन्त्र दिवस", 11, 7),
    HolidayRule::fixed_public(
        "International Women's Day",
        "अन्तर्राष्ट्रिय महिला दिवस",
        Rule::gregorian(3, 8),
    ),
    // Tamu Lhosar, the Gurung new year, is "पुस १५" in both notices.
    np_bs("Tamu Lhosar", "तमू ल्होछार", 9, 15),
    // The festivals, from the same sections: a tithi each, or the span
    // between two, or the Tibetan new year.
    np_festival(
        "Buddha Jayanti",
        "बुद्ध जयन्ती",
        np_tithi(2, 15, Prevalence::Midday, WhenTwice::Earlier),
    ),
    np_festival(
        "Janai Purnima",
        "रक्षाबन्धन",
        np_tithi(5, 15, Prevalence::Sunrise, WhenTwice::Earlier),
    ),
    // In the evening: the Indian rule, the eighth tithi at midnight, gives
    // 15 August 2025 where the notice has the 16th, and sunrise gives
    // 7 September 2023 where it has the 6th.
    np_festival(
        "Krishna Janmashtami",
        "श्रीकृष्ण जन्माष्टमी",
        np_tithi(5, 23, Prevalence::Evening, WhenTwice::Earlier),
    ),
    np_festival(
        "Ghatasthapana",
        "घटस्थापना",
        np_tithi(7, 1, Prevalence::Sunrise, WhenTwice::Earlier),
    ),
    np_festival(
        "Dashain",
        "दशैं",
        Rule::span(&NP_PHULPATI, &NP_DASHAIN_DWADASHI),
    ),
    np_festival("Tihar", "तिहार", Rule::span(&NP_LAXMI_PUJA, &NP_TIHAR_LAST)),
    // Chhath, Kārtika śukla 6, is left out: no one part of the day puts
    // it where all four notices do. Sunrise gives 28 October 2025 for the
    // notice's 27th, and every later part of the day 18 November 2023 for
    // its 19th.
    np_festival(
        "Dhanya Purnima",
        "धान्य पूर्णिमा",
        np_tithi(9, 15, Prevalence::Sunrise, WhenTwice::Earlier),
    ),
    np_festival(
        "Sonam Lhosar",
        "सोनम ल्होछार",
        np_tithi(11, 1, Prevalence::Sunrise, WhenTwice::Earlier),
    ),
    np_festival(
        "Maha Shivaratri",
        "महाशिवरात्री",
        np_tithi(11, 29, Prevalence::Midnight, WhenTwice::Earlier),
    ),
    // Gyalpo Lhosar is the Tibetan New Year, but not the Phugpa
    // reckoning's: its Losar is 18 February 2026, the notice's day, and
    // 7 February 2027, a month before the notice's 9 March, because it
    // intercalates a month that Nepal's reckoning does not. Phālguna śukla
    // 1 at Kathmandu gives both of the notices' days.
    np_festival(
        "Gyalpo Lhosar",
        "ग्याल्पो ल्होसार",
        np_tithi(12, 1, Prevalence::Sunrise, WhenTwice::Earlier),
    ),
    // The notices give these without a date — "ईद (ईद उल फित्र) का दिन",
    // the day of Eid — and the tabular Hijri calendar is a prediction of it.
    np_festival("Eid al-Fitr", "ईद", EID_AL_FITR),
    np_festival("Eid al-Adha", "बकर ईद", EID_AL_ADHA),
];

/// Nepal — the weekend, the public holidays on a fixed date, and the
/// festivals.
pub static NEPAL: RuleSet = RuleSet {
    code: "NP",
    english_name: "Nepal",
    rules: NP_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: NP_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "Government of Nepal, Ministry of Home Affairs, the annual \
              notices of public holidays in the Nepal Rajpatra, Part 5: for \
              2082 BS (Khanda 74, No. 59) and 2083 BS (Khanda 75, No. 67), \
              sections 2.1, 6.1 and 7.1, the holidays for every office in \
              the country, with their dates and local names. Which tithi \
              each festival is: Wikipedia, \"Dashain\" (Āśvina śukla, \
              Phulpati the seventh day), \"Tihar (festival)\" (Lakshmi Puja \
              on the new moon, Bhai Tika Kartika śukla 2), \"Raksha Bandhan\" (the full moon of \
              Shravana; Janai Purnima in Nepal), \"Krishna Janmashtami\" \
              (Shravana kṛṣṇa 8, amānta), \"Buddha's Birthday\" (the full \
              moon of Vaisakha in Nepal), \"Yomari Punhi\" (the full moon of \
              Thinlā, Mārgaśīrṣa), \"Maha Shivaratri\" (Māgha kṛṣṇa 14, \
              amānta), \"Sonam Lhosar\" (Magh śukla pratipadā), \"Tamu \
              Lhosar\" (15 Poush) and \"Gyalpo Losar\" (the first day of \
              the Tibetan year, whose Phugpa reckoning the 2083 notice does \
              not follow), all retrieved 2026-09-23. The part of the day each \
              tithi holds is fitted to the notices for 2080 to 2083 BS. \
              Chhath, which no single rule fits, and the holidays for one \
              community, region or group are not listed yet",
};

// ─────────────────────────────────────────────────────────────────────────
// Sri Lanka
// ─────────────────────────────────────────────────────────────────────────

/// The first year the gazettes carried here fix.
const LK_FIRST: i64 = 2023;
/// The last year they fix.
const LK_LAST: i64 = 2027;

/// The days the Holidays Act orders name, as the gazette for each year
/// lists them, in the gazette's order.
static LK_GAZETTED: &[(i64, u8, u8, &str)] = &[
    // 2023
    (2023, 1, 6, "Duruthu Full Moon Poya Day"),
    (2023, 1, 15, "Tamil Thai Pongal Day"),
    (2023, 1, 16, "Special Bank Holiday"),
    (2023, 2, 5, "Navam Full Moon Poya Day"),
    (2023, 2, 18, "Maha Shivarathri Day"),
    (2023, 3, 6, "Medin Full Moon Poya Day"),
    (2023, 4, 5, "Bak Full Moon Poya Day"),
    (2023, 4, 13, "Day Prior to Sinhala & Tamil New Year Day"),
    (2023, 4, 14, "Sinhala & Tamil New Year Day"),
    (2023, 4, 22, "Id-Ul-Fitr (Ramazan Festival Day)"),
    (2023, 5, 5, "Vesak Full Moon Poya Day"),
    (2023, 5, 6, "Day Following Vesak Full Moon Poya Day"),
    (2023, 6, 3, "Poson Full Moon Poya Day"),
    (2023, 6, 29, "Id-Ul-Adha (Hadji Festival Day)"),
    (2023, 7, 3, "Adhi Esala Full Moon Poya Day"),
    (2023, 8, 1, "Esala Full Moon Poya Day"),
    (2023, 8, 30, "Nikini Full Moon Poya Day"),
    (2023, 9, 28, "Milad-Un-Nabi (Holy Prophet's Birthday)"),
    (2023, 9, 29, "Binara Full Moon Poya Day"),
    (2023, 10, 28, "Vap Full Moon Poya Day"),
    (2023, 11, 12, "Deepavali Festival Day"),
    (2023, 11, 26, "Il Full Moon Poya Day"),
    (2023, 12, 26, "Unduvap Full Moon Poya Day"),
    // 2024
    (2024, 1, 15, "Tamil Thai Pongal Day"),
    (2024, 1, 25, "Duruthu Full Moon Poya Day"),
    (2024, 2, 23, "Navam Full Moon Poya Day"),
    (2024, 3, 8, "Maha Shivarathri Day"),
    (2024, 3, 24, "Medin Full Moon Poya Day"),
    (2024, 4, 11, "Id-Ul-Fitr (Ramazan Festival Day)"),
    (2024, 4, 12, "Day Prior to Sinhala & Tamil New Year Day"),
    (2024, 4, 13, "Sinhala & Tamil New Year Day"),
    (2024, 4, 23, "Bak Full Moon Poya Day"),
    (2024, 5, 23, "Vesak Full Moon Poya Day"),
    (2024, 5, 24, "Day Following Vesak Full Moon Poya Day"),
    (2024, 6, 17, "Id-Ul-Adha (Hadji Festival Day)"),
    (2024, 6, 21, "Poson Full Moon Poya Day"),
    (2024, 7, 20, "Esala Full Moon Poya Day"),
    (2024, 8, 19, "Nikini Full Moon Poya Day"),
    (2024, 9, 16, "Milad-Un-Nabi (Holy Prophet's Birthday)"),
    (2024, 9, 17, "Binara Full Moon Poya Day"),
    (2024, 10, 17, "Vap Full Moon Poya Day"),
    (2024, 10, 31, "Deepavali Festival Day"),
    (2024, 11, 15, "Il Full Moon Poya Day"),
    (2024, 12, 14, "Unduvap Full Moon Poya Day"),
    // 2025
    (2025, 1, 13, "Duruthu Full Moon Poya Day"),
    (2025, 1, 14, "Tamil Thai Pongal Day"),
    (2025, 2, 12, "Navam Full Moon Poya Day"),
    (2025, 2, 26, "Maha Shivarathri Day"),
    (2025, 3, 13, "Medin Full Moon Poya Day"),
    (2025, 3, 31, "Id-Ul-Fitr (Ramazan Festival Day)"),
    (2025, 4, 12, "Bak Full Moon Poya Day"),
    (2025, 4, 13, "Day Prior to Sinhala & Tamil New Year Day"),
    (2025, 4, 14, "Sinhala & Tamil New Year Day"),
    (2025, 4, 15, "Special Bank Holiday"),
    (2025, 5, 12, "Vesak Full Moon Poya Day"),
    (2025, 5, 13, "Day Following Vesak Full Moon Poya Day"),
    (2025, 6, 7, "Id-Ul-Adha (Hadji Festival Day)"),
    (2025, 6, 10, "Poson Full Moon Poya Day"),
    (2025, 7, 10, "Esala Full Moon Poya Day"),
    (2025, 8, 8, "Nikini Full Moon Poya Day"),
    (2025, 9, 5, "Milad-Un-Nabi (Holy Prophet's Birthday)"),
    (2025, 9, 7, "Binara Full Moon Poya Day"),
    (2025, 10, 6, "Vap Full Moon Poya Day"),
    (2025, 10, 20, "Deepavali Festival Day"),
    (2025, 11, 5, "Il Full Moon Poya Day"),
    (2025, 12, 4, "Unduvap Full Moon Poya Day"),
    // 2026
    (2026, 1, 3, "Duruthu Full Moon Poya Day"),
    (2026, 1, 15, "Tamil Thai Pongal Day"),
    (2026, 2, 1, "Navam Full Moon Poya Day"),
    (2026, 2, 15, "Maha Shivarathri Day"),
    (2026, 3, 2, "Medin Full Moon Poya Day"),
    (2026, 3, 21, "Id-Ul-Fitr (Ramazan Festival Day)"),
    (2026, 4, 1, "Bak Full Moon Poya Day"),
    (2026, 4, 13, "Day Prior to Sinhala & Tamil New Year Day"),
    (2026, 4, 14, "Sinhala & Tamil New Year Day"),
    (2026, 5, 1, "Vesak Full Moon Poya Day"),
    (2026, 5, 2, "Day Following Vesak Full Moon Poya Day"),
    (2026, 5, 28, "Id-Ul-Adha (Hadji Festival Day)"),
    (2026, 5, 30, "Adhi Poson Full Moon Poya Day"),
    (2026, 6, 29, "Poson Full Moon Poya Day"),
    (2026, 7, 29, "Esala Full Moon Poya Day"),
    (2026, 8, 26, "Milad-Un-Nabi (Holy Prophet's Birthday)"),
    (2026, 8, 27, "Nikini Full Moon Poya Day"),
    (2026, 9, 26, "Binara Full Moon Poya Day"),
    (2026, 10, 25, "Vap Full Moon Poya Day"),
    (2026, 11, 8, "Deepavali Festival Day"),
    (2026, 11, 24, "Il Full Moon Poya Day"),
    (2026, 12, 23, "Unduvap Full Moon Poya Day"),
    // 2027
    (2027, 1, 15, "Tamil Thai Pongal Day"),
    (2027, 1, 22, "Duruthu Full Moon Poya Day"),
    (2027, 2, 20, "Navam Full Moon Poya Day"),
    (2027, 3, 6, "Maha Shivarathri Day"),
    (2027, 3, 10, "Id-Ul-Fitr (Ramazan Festival Day)"),
    (2027, 3, 22, "Medin Full Moon Poya Day"),
    (2027, 4, 13, "Day Prior to Sinhala & Tamil New Year Day"),
    (2027, 4, 14, "Sinhala & Tamil New Year Day"),
    (2027, 4, 20, "Bak Full Moon Poya Day"),
    (2027, 5, 17, "Id-Ul-Adha (Hadji Festival Day)"),
    (2027, 5, 19, "Vesak Full Moon Poya Day"),
    (2027, 5, 20, "Day Following Vesak Full Moon Poya Day"),
    (2027, 6, 18, "Poson Full Moon Poya Day"),
    (2027, 7, 18, "Esala Full Moon Poya Day"),
    (2027, 8, 15, "Milad-Un-Nabi (Holy Prophet's Birthday)"),
    (2027, 8, 16, "Nikini Full Moon Poya Day"),
    (2027, 9, 15, "Binara Full Moon Poya Day"),
    (2027, 10, 15, "Vap Full Moon Poya Day"),
    (2027, 10, 28, "Deepavali Festival Day"),
    (2027, 11, 13, "Il Full Moon Poya Day"),
    (2027, 12, 13, "Unduvap Full Moon Poya Day"),
];

/// A lookup into [`LK_GAZETTED`] for each holiday it names: the days of
/// that name the gazette for `year` lists.
macro_rules! lk_gazetted {
    ($($function:ident => $name:literal),* $(,)?) => {
        $(
            fn $function(year: i64) -> Days {
                let mut out = Days::new();
                for &(y, month, day, name) in LK_GAZETTED {
                    if y == year && name == $name {
                        if let Ok(fixed) = gregorian::to_fixed(y, month, day) {
                            out.push(fixed);
                        }
                    }
                }
                out
            }
        )*
    };
}

lk_gazetted! {
    lk_duruthu => "Duruthu Full Moon Poya Day",
    lk_navam => "Navam Full Moon Poya Day",
    lk_medin => "Medin Full Moon Poya Day",
    lk_bak => "Bak Full Moon Poya Day",
    lk_vesak => "Vesak Full Moon Poya Day",
    lk_after_vesak => "Day Following Vesak Full Moon Poya Day",
    lk_adhi_poson => "Adhi Poson Full Moon Poya Day",
    lk_poson => "Poson Full Moon Poya Day",
    lk_adhi_esala => "Adhi Esala Full Moon Poya Day",
    lk_esala => "Esala Full Moon Poya Day",
    lk_nikini => "Nikini Full Moon Poya Day",
    lk_binara => "Binara Full Moon Poya Day",
    lk_vap => "Vap Full Moon Poya Day",
    lk_il => "Il Full Moon Poya Day",
    lk_unduvap => "Unduvap Full Moon Poya Day",
    lk_thai_pongal => "Tamil Thai Pongal Day",
    lk_shivarathri => "Maha Shivarathri Day",
    lk_new_year_eve => "Day Prior to Sinhala & Tamil New Year Day",
    lk_new_year => "Sinhala & Tamil New Year Day",
    lk_fitr => "Id-Ul-Fitr (Ramazan Festival Day)",
    lk_adha => "Id-Ul-Adha (Hadji Festival Day)",
    lk_milad => "Milad-Un-Nabi (Holy Prophet's Birthday)",
    lk_deepavali => "Deepavali Festival Day",
    lk_special_bank => "Special Bank Holiday",
}

/// A day the gazettes list, public and bank holiday alike, for the years
/// they cover.
const fn lk(name: &'static str, function: fn(i64) -> Days) -> HolidayRule {
    HolidayRule::fixed_public(
        name,
        "",
        Rule::Tabulated {
            function,
            first_year: LK_FIRST,
            last_year: LK_LAST,
        },
    )
}

static LK_RULES: &[HolidayRule] = &[
    lk("Duruthu Full Moon Poya Day", lk_duruthu),
    lk("Tamil Thai Pongal Day", lk_thai_pongal),
    HolidayRule::fixed_public("Independence Day", "", Rule::gregorian(2, 4)),
    lk("Navam Full Moon Poya Day", lk_navam),
    lk("Maha Shivarathri Day", lk_shivarathri),
    lk("Medin Full Moon Poya Day", lk_medin),
    lk("Id-Ul-Fitr (Ramazan Festival Day)", lk_fitr),
    lk("Bak Full Moon Poya Day", lk_bak),
    HolidayRule::fixed_public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    lk("Day Prior to Sinhala & Tamil New Year Day", lk_new_year_eve),
    lk("Sinhala & Tamil New Year Day", lk_new_year),
    HolidayRule::fixed_public(
        "May Day (International Workers' Day)",
        "",
        Rule::gregorian(5, 1),
    ),
    lk("Vesak Full Moon Poya Day", lk_vesak),
    lk("Day Following Vesak Full Moon Poya Day", lk_after_vesak),
    lk("Id-Ul-Adha (Hadji Festival Day)", lk_adha),
    lk("Adhi Poson Full Moon Poya Day", lk_adhi_poson),
    lk("Poson Full Moon Poya Day", lk_poson),
    lk("Adhi Esala Full Moon Poya Day", lk_adhi_esala),
    lk("Esala Full Moon Poya Day", lk_esala),
    lk("Nikini Full Moon Poya Day", lk_nikini),
    lk("Milad-Un-Nabi (Holy Prophet's Birthday)", lk_milad),
    lk("Binara Full Moon Poya Day", lk_binara),
    lk("Vap Full Moon Poya Day", lk_vap),
    lk("Deepavali Festival Day", lk_deepavali),
    lk("Il Full Moon Poya Day", lk_il),
    lk("Unduvap Full Moon Poya Day", lk_unduvap),
    HolidayRule::fixed_public("Christmas Day", "", Rule::gregorian(12, 25)),
    // Marked for the banks alone: 16 January 2023 and 15 April 2025.
    lk("Special Bank Holiday", lk_special_bank).of_kind(Kind::Bank),
];

/// Sri Lanka — the public and bank holidays the Minister of Public
/// Administration orders under section 4 of the Holidays Act, No. 29 of
/// 1971, one gazette a year.
///
/// Independence Day, May Day and Christmas are fixed Gregorian dates and
/// Good Friday is the Western Easter's; everything else — the full-moon
/// Poya days, Thai Pongal, Maha Shivarathri, the Sinhala and Tamil New
/// Year, the three Muslim days and Deepavali — is taken from the gazettes
/// for 2023 to 2027, and a year outside them reports those days as a gap.
///
/// # Why the Poya days are a table
///
/// Every Poya is the day of a full moon, and in an intercalary year one is
/// the *adhi* Poya of the month it doubles. But no rule found reproduces
/// which day: the day whose sunrise, midday, afternoon, evening or midnight
/// has the full-moon tithi at Colombo places 34 to 55 of the 62 gazetted
/// Poya days of these five years. The *Sūrya Siddhānta*'s full moon fits
/// at best 59, taking the day of the last sunset before it, and only with
/// its clock moved two and a half hours for no reason the sources give.
/// And the intercalary month is not
/// always India's: 2026's Adhi Poson is the adhika Jyeṣṭha of the
/// *Rashtriya Panchang*, but 2023's Adhi Esala falls a month before its
/// adhika Śrāvaṇa. So the days are the gazettes', and the crate does not
/// guess the next year's.
///
/// No day is moved when it falls on a weekend: the gazettes list, for
/// instance, the Poson Poya of Saturday 3 June 2023 and no substitute.
pub static SRI_LANKA: RuleSet = RuleSet {
    code: "LK",
    english_name: "Sri Lanka",
    rules: LK_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 23),
    sources: "The Holidays Act, No. 29 of 1971, section 4 orders in the Gazette Extraordinary:               Nos. 2287/4 (2023), 2341/46 (2024), 2395/33 (2025), 2438/22 (2026) and 2493/5               (2027), from the Department of Government Printing (documents.gov.lk),               retrieved 2026-09-23. Holiday names are the gazettes' English ones, their               spellings unified where they vary from year to year",
};

// ─────────────────────────────────────────────────────────────────────────
// Pakistan
// ─────────────────────────────────────────────────────────────────────────

/// A Hijri-dated holiday, which Pakistan keeps on the Ruet-e-Hilal
/// Committee's sighting and which is therefore approximate.
const fn pk_hijri(name: &'static str, local: &'static str, month: u8, day: u8) -> HolidayRule {
    HolidayRule::fixed_public(
        name,
        local,
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, month, day),
    )
    .approximate()
}

static PK_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("Kashmir Day", "یومِ یکجہتیِ کشمیر", Rule::gregorian(2, 5)),
    HolidayRule::fixed_public("Pakistan Day", "یومِ پاکستان", Rule::gregorian(3, 23)),
    HolidayRule::fixed_public("Labour Day", "یومِ مزدور", Rule::gregorian(5, 1)),
    // In the source's table of state holidays; first gazetted as a day
    // off for 2024, a year the author knows and the source does not state.
    HolidayRule::fixed_public("Youm-e-Takbeer", "یومِ تکبیر", Rule::gregorian(5, 28))
        .years(Some(2024), None),
    HolidayRule::fixed_public("Independence Day", "یومِ آزادی", Rule::gregorian(8, 14)),
    // Withdrawn in 2015 and restored in 2022.
    HolidayRule::fixed_public("Iqbal Day", "یومِ اقبال", Rule::gregorian(11, 9))
        .years(None, Some(2014)),
    HolidayRule::fixed_public("Iqbal Day", "یومِ اقبال", Rule::gregorian(11, 9))
        .years(Some(2022), None),
    HolidayRule::fixed_public("Quaid-e-Azam Day", "یومِ قائدِاعظم", Rule::gregorian(12, 25)),
    pk_hijri("Ashura", "عاشورہ", 1, 9),
    pk_hijri("Ashura", "عاشورہ", 1, 10),
    pk_hijri("Eid Milad-un-Nabi", "عید میلاد النبی", 3, 12),
    pk_hijri("Eid-ul-Fitr", "عيد الفطر", 10, 1),
    pk_hijri("Eid-ul-Adha", "عید الاضحٰی", 12, 10),
];

/// Pakistan.
///
/// The state holidays as the source tabulates them: the civil days on the
/// Gregorian calendar and the religious ones on the Hijri, the latter kept
/// on the Ruet-e-Hilal Committee's sighting and so approximate. Iqbal Day
/// is carried to 2014 and from 2022, the years its holiday status was
/// withdrawn and restored. The extra days the annual notification adds to
/// the two Eids, and the optional holidays of the religious minorities, are
/// not carried; the source says nothing of a holiday on the weekend, and
/// nothing is done with one.
pub static PAKISTAN: RuleSet = RuleSet {
    code: "PK",
    english_name: "Pakistan",
    rules: PK_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Wikipedia, \"Public holidays in Pakistan\", retrieved 2026-09-22, for \
              the state holidays and their Urdu names; Wikipedia, \"Iqbal Day\", \
              retrieved 2026-09-22, for the withdrawal of 2015 and the \
              restoration of 2022",
};

// ─────────────────────────────────────────────────────────────────────────
// Myanmar
// ─────────────────────────────────────────────────────────────────────────

/// A holiday on a day of the Burmese calendar.
const fn mm_burmese(name: &'static str, local: &'static str, month: u8, day: u8) -> HolidayRule {
    HolidayRule::fixed_public(
        name,
        local,
        Rule::in_calendar(CalendarSystem::BURMESE, month, day),
    )
}

/// The Myanmar year whose Thingyan falls in Gregorian `year`: the era's
/// year 0 began in 638.
const fn mm_year_of_thingyan(year: i64) -> i64 {
    year - 638
}

fn thingyan_akyo(year: i64) -> Days {
    Days::one(hc_calendars_regional::burmese::thingyan(mm_year_of_thingyan(year)).akyo)
}

fn thingyan_akya(year: i64) -> Days {
    Days::one(hc_calendars_regional::burmese::thingyan(mm_year_of_thingyan(year)).akya)
}

/// The one or two *akyat* days between *akya* and *atat*.
fn thingyan_akyat(year: i64) -> Days {
    let festival = hc_calendars_regional::burmese::thingyan(mm_year_of_thingyan(year));
    let mut out = Days::new();
    let mut day = festival.akya.0 + 1;
    while day <= festival.last_akyat.0 {
        out.push(hc_calendar::Rd(day));
        day += 1;
    }
    out
}

fn thingyan_atat(year: i64) -> Days {
    Days::one(hc_calendars_regional::burmese::thingyan(mm_year_of_thingyan(year)).atat)
}

fn myanmar_new_year(year: i64) -> Days {
    Days::one(hc_calendars_regional::burmese::thingyan(mm_year_of_thingyan(year)).new_year)
}

static MM_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("Independence Day", "လွတ်လပ်ရေးနေ့", Rule::gregorian(1, 4)),
    HolidayRule::fixed_public(
        "Chinese New Year",
        "",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 1),
    ),
    HolidayRule::fixed_public("Union Day", "ပြည်ထောင်စုနေ့", Rule::gregorian(2, 12)),
    HolidayRule::fixed_public("Peasants' Day", "တောင်သူလယ်သမားနေ့", Rule::gregorian(3, 2)),
    mm_burmese("Full Moon Day of Tabaung", "တပေါင်းလပြည့်နေ့", 12, 15),
    HolidayRule::fixed_public("Armed Forces Day", "တပ်မတော်နေ့", Rule::gregorian(3, 27)),
    // Thingyan: the eve, the first day, the one or two days between, the
    // day the old year ends, and the New Year's day, from the calendar's
    // own moments.
    HolidayRule::fixed_public("Thingyan Eve", "သင်္ကြန်အကြိုနေ့", Rule::Computed(thingyan_akyo)),
    HolidayRule::fixed_public(
        "Thingyan Akya Day",
        "သင်္ကြန်အကျနေ့",
        Rule::Computed(thingyan_akya),
    ),
    HolidayRule::fixed_public(
        "Thingyan Akyat Day",
        "သင်္ကြန်အကြတ်နေ့",
        Rule::Computed(thingyan_akyat),
    ),
    HolidayRule::fixed_public(
        "Thingyan Atat Day",
        "သင်္ကြန်အတက်နေ့",
        Rule::Computed(thingyan_atat),
    ),
    HolidayRule::fixed_public(
        "Myanmar New Year's Day",
        "နှစ်ဆန်းတစ်ရက်နေ့",
        Rule::Computed(myanmar_new_year),
    ),
    HolidayRule::fixed_public("Labour Day", "အလုပ်သမားနေ့", Rule::gregorian(5, 1)),
    mm_burmese("Full Moon Day of Kason", "ကဆုန်လပြည့်နေ့", 2, 15),
    HolidayRule::fixed_public("Martyrs' Day", "အာဇာနည်နေ့", Rule::gregorian(7, 19)),
    mm_burmese("Full Moon Day of Waso", "ဝါဆိုလပြည့်နေ့", 4, 15),
    mm_burmese("Thadingyut Holiday", "သီတင်းကျွတ်", 7, 14),
    mm_burmese("Full Moon Day of Thadingyut", "သီတင်းကျွတ်လပြည့်နေ့", 7, 15),
    mm_burmese("Thadingyut Holiday", "သီတင်းကျွတ်", 7, 16),
    mm_burmese("Tazaungdaing Holiday", "တန်ဆောင်တိုင်", 8, 14),
    mm_burmese("Full Moon Day of Tazaungmon", "တန်ဆောင်မုန်းလပြည့်နေ့", 8, 15),
    // The tenth waning day of Tazaungmon.
    mm_burmese("National Day", "အမျိုးသားနေ့", 8, 25),
    HolidayRule::fixed_public("Christmas Day", "ခရစ္စမတ်နေ့", Rule::gregorian(12, 25)),
    // The first waxing day of Pyatho.
    mm_burmese("Kayin New Year", "ကရင်နှစ်သစ်ကူး", 10, 1),
    HolidayRule::fixed_public("Eid al-Adha", "", EID_AL_ADHA).approximate(),
    // As the source states it: "based on the traditional Burmese calendar
    // (1st waxing day of Tazaungmon)".
    mm_burmese("Deepavali", "ဒီပါဝလီ", 8, 1),
];

/// Myanmar.
///
/// The public holidays as the source lists them, dated on the Burmese
/// calendar where the source dates them there: the full moons of Tabaung,
/// Kason, Waso, Thadingyut and Tazaungmon, National Day on the tenth waning
/// of Tazaungmon, the Kayin New Year on the first waxing of Pyatho, and
/// Deepavali on the first waxing of Tazaungmon, which is how the source
/// says Myanmar fixes it. Thingyan is five days from the calendar's own
/// *akya* and *atat* moments: the eve, the first day, the day or two
/// between, the last day of the old year, and the New Year's day. The
/// gazette extends several of these — nine days for Thadingyut in recent
/// years, a longer Thingyan block — and the extensions are annual and not
/// carried. Eid al-Adha is on the tabular Hijri calendar and approximate.
pub static MYANMAR: RuleSet = RuleSet {
    code: "MM",
    english_name: "Myanmar",
    rules: MM_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Wikipedia, \"Public holidays in Myanmar\", retrieved 2026-09-22, for \
              the list and the Burmese dates it gives; the Thingyan moments from \
              Yan Naing Aye's arithmetic as `hc-calendars-regional::burmese` \
              carries it",
};

// ─────────────────────────────────────────────────────────────────────────
// Hong Kong
// ─────────────────────────────────────────────────────────────────────────

/// The General Holidays Ordinance as gazetted year by year: a general
/// holiday on a Sunday, or on the same day as another general holiday, is
/// observed on the next day that is not itself one. A Saturday moves
/// nothing. The three Lunar New Year days and the day following the
/// Mid-Autumn Festival were the exception from 1983 to 2011 and are
/// `fixed_public` for those years, with their own eve-of rules below.
static HK_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: true,
    valid_from: None,
    valid_until: None,
}];

static HK_LUNAR_NEW_YEAR: Rule = Rule::in_calendar(CalendarSystem::CHINESE, 1, 1);
static HK_MID_AUTUMN_FOLLOWING: Rule = Rule::in_calendar(CalendarSystem::CHINESE, 8, 16);
static HK_DECEMBER_26: Rule = Rule::gregorian(12, 26);

/// From 1983 to 2011 a Lunar New Year day on a Sunday was made up on the
/// day before Lunar New Year's Day, so that the eve rather than the fourth
/// day was the day off — 17 February 2007 and 13 February 2010 among
/// them. The make-up ran forward before 1983 and again from the amendment
/// of 14 December 2011, which first bit on 13 February 2013.
fn hk_lunar_new_year_eve(year: i64) -> Days {
    let days = HK_LUNAR_NEW_YEAR.days_in_year(year);
    let Some(&first) = days.as_slice().first() else {
        return Days::new();
    };
    let on_sunday = (0..3).any(|offset| Weekday::from_rd(Rd(first.0 + offset)) == Weekday::Sunday);
    if on_sunday {
        Days::one(Rd(first.0 - 1))
    } else {
        Days::new()
    }
}

/// The same for the day following the Mid-Autumn Festival: on a Sunday,
/// the festival day itself, a Saturday, was the day off from 1983 to 2011,
/// 3 October 2009 the last time.
fn hk_mid_autumn_day(year: i64) -> Days {
    let days = HK_MID_AUTUMN_FOLLOWING.days_in_year(year);
    let Some(&following) = days.as_slice().first() else {
        return Days::new();
    };
    if Weekday::from_rd(following) == Weekday::Sunday {
        Days::one(Rd(following.0 - 1))
    } else {
        Days::new()
    }
}

static HK_RULES: &[HolidayRule] = &[
    HolidayRule::public("The first day of January", "一月一日", Rule::gregorian(1, 1)),
    // Lunar New Year: made up after the third day, except from 1983 to
    // 2011, when the eve stood in.
    HolidayRule::public("Lunar New Year's Day", "農曆年初一", Rule::in_calendar(CalendarSystem::CHINESE, 1, 1))
        .years(None, Some(1982)),
    HolidayRule::fixed_public("Lunar New Year's Day", "農曆年初一", Rule::in_calendar(CalendarSystem::CHINESE, 1, 1))
        .years(Some(1983), Some(2011)),
    HolidayRule::public("Lunar New Year's Day", "農曆年初一", Rule::in_calendar(CalendarSystem::CHINESE, 1, 1))
        .years(Some(2012), None),
    HolidayRule::public("The second day of Lunar New Year", "農曆年初二", Rule::in_calendar(CalendarSystem::CHINESE, 1, 2))
        .years(None, Some(1982)),
    HolidayRule::fixed_public("The second day of Lunar New Year", "農曆年初二", Rule::in_calendar(CalendarSystem::CHINESE, 1, 2))
        .years(Some(1983), Some(2011)),
    HolidayRule::public("The second day of Lunar New Year", "農曆年初二", Rule::in_calendar(CalendarSystem::CHINESE, 1, 2))
        .years(Some(2012), None),
    HolidayRule::public("The third day of Lunar New Year", "農曆年初三", Rule::in_calendar(CalendarSystem::CHINESE, 1, 3))
        .years(Some(1968), Some(1982)),
    HolidayRule::fixed_public("The third day of Lunar New Year", "農曆年初三", Rule::in_calendar(CalendarSystem::CHINESE, 1, 3))
        .years(Some(1983), Some(2011)),
    HolidayRule::public("The third day of Lunar New Year", "農曆年初三", Rule::in_calendar(CalendarSystem::CHINESE, 1, 3))
        .years(Some(2012), None),
    HolidayRule::fixed_public("Lunar New Year's Eve", "農曆年初一前一日", Rule::Computed(hk_lunar_new_year_eve))
        .years(Some(1983), Some(2011)),
    HolidayRule::public(
        "Ching Ming Festival",
        "清明節",
        Rule::SolarTerm {
            term: QINGMING,
            meridian: hc_seasons::Meridian::CHINA,
        },
    )
    .years(Some(1968), None),
    // The Easter days are general holidays that the Employment Ordinance
    // reaches only as its 2021 amendment phases them in.
    HolidayRule::public("Good Friday", "耶穌受難節", Rule::easter(GOOD_FRIDAY))
        .years(None, Some(2027))
        .of_kind(Kind::Bank),
    HolidayRule::public("Good Friday", "耶穌受難節", Rule::easter(GOOD_FRIDAY)).years(Some(2028), None),
    HolidayRule::public("The day following Good Friday", "耶穌受難節翌日", Rule::easter(HOLY_SATURDAY))
        .years(None, Some(2029))
        .of_kind(Kind::Bank),
    HolidayRule::public("The day following Good Friday", "耶穌受難節翌日", Rule::easter(HOLY_SATURDAY))
        .years(Some(2030), None),
    HolidayRule::public("Easter Monday", "復活節星期一", Rule::easter(EASTER_MONDAY))
        .years(None, Some(2025))
        .of_kind(Kind::Bank),
    HolidayRule::public("Easter Monday", "復活節星期一", Rule::easter(EASTER_MONDAY)).years(Some(2026), None),
    HolidayRule::public("Labour Day", "勞動節", Rule::gregorian(5, 1)).years(Some(1999), None),
    HolidayRule::public("The Birthday of the Buddha", "佛誕", Rule::in_calendar(CalendarSystem::CHINESE, 4, 8))
        .years(Some(1999), Some(2021))
        .of_kind(Kind::Bank),
    HolidayRule::public("The Birthday of the Buddha", "佛誕", Rule::in_calendar(CalendarSystem::CHINESE, 4, 8))
        .years(Some(2022), None),
    HolidayRule::public("Tuen Ng Festival", "端午節", Rule::in_calendar(CalendarSystem::CHINESE, 5, 5))
        .years(Some(1968), None),
    HolidayRule::public(
        "Hong Kong Special Administrative Region Establishment Day",
        "香港特別行政區成立紀念日",
        Rule::gregorian(7, 1),
    )
    .years(Some(1997), None),
    HolidayRule::fixed_public(
        "The day following Hong Kong Special Administrative Region Establishment Day",
        "香港特別行政區成立紀念日翌日",
        Rule::gregorian(7, 2),
    )
    .years(Some(1997), Some(1997)),
    HolidayRule::public(
        "Sino-Japanese War Victory Day",
        "抗日戰爭勝利紀念日",
        Rule::nth(8, 3, Weekday::Monday),
    )
    .years(Some(1997), Some(1998))
    .of_kind(Kind::Bank),
    HolidayRule::public(
        "The day following the Chinese Mid-Autumn Festival",
        "中秋節翌日",
        Rule::in_calendar(CalendarSystem::CHINESE, 8, 16),
    )
    .years(Some(1968), Some(1982)),
    HolidayRule::fixed_public(
        "The day following the Chinese Mid-Autumn Festival",
        "中秋節翌日",
        Rule::in_calendar(CalendarSystem::CHINESE, 8, 16),
    )
    .years(Some(1983), Some(2011)),
    HolidayRule::public(
        "The day following the Chinese Mid-Autumn Festival",
        "中秋節翌日",
        Rule::in_calendar(CalendarSystem::CHINESE, 8, 16),
    )
    .years(Some(2012), None),
    HolidayRule::fixed_public("Chinese Mid-Autumn Festival", "中秋節", Rule::Computed(hk_mid_autumn_day))
        .years(Some(1983), Some(2011)),
    HolidayRule::public("National Day", "國慶日", Rule::gregorian(10, 1)).years(Some(1997), None),
    HolidayRule::public("The day following National Day", "國慶日翌日", Rule::gregorian(10, 2))
        .years(Some(1997), Some(1998))
        .of_kind(Kind::Bank),
    HolidayRule::public("Chung Yeung Festival", "重陽節", Rule::in_calendar(CalendarSystem::CHINESE, 9, 9))
        .years(Some(1968), None),
    HolidayRule::public("Christmas Day", "聖誕節", Rule::gregorian(12, 25)),
    // 26 December, or 27 December when the 26th is a Sunday: the rule
    // moves the holiday itself, so it never needs a substitute.
    HolidayRule::fixed_public(
        "The first weekday after Christmas Day",
        "聖誕節後第一個周日",
        Rule::moved_by_weekday(&HK_DECEMBER_26, &[(Weekday::Sunday, 1)]),
    )
    .years(None, Some(2023))
    .of_kind(Kind::Bank),
    HolidayRule::fixed_public(
        "The first weekday after Christmas Day",
        "聖誕節後第一個周日",
        Rule::moved_by_weekday(&HK_DECEMBER_26, &[(Weekday::Sunday, 1)]),
    )
    .years(Some(2024), None),
    // The one-off of 2015, a general and a statutory holiday.
    HolidayRule::fixed_public(
        "The 70th anniversary day of the victory of the Chinese people's war of resistance against Japanese aggression",
        "中國人民抗日戰爭勝利70周年紀念日",
        Rule::gregorian(9, 3),
    )
    .years(Some(2015), Some(2015)),
    // The Employment Ordinance lets an employer give this instead of
    // Christmas Day; it is not a general holiday.
    HolidayRule::observance(
        "Chinese Winter Solstice Festival",
        "冬節",
        Rule::SolarTerm {
            term: SolarTerm::WINTER_SOLSTICE,
            meridian: hc_seasons::Meridian::CHINA,
        },
    ),
];

/// Hong Kong.
///
/// The seventeen general holidays of the General Holidays Ordinance
/// (Cap. 149), which banks, schools, public offices and the Government
/// keep, and within them the statutory holidays of the Employment
/// Ordinance (Cap. 57), which every employer must give. A general holiday
/// that is not statutory is [`Kind::Bank`], and the 2021 amendment's
/// phasing-in is carried as years: the Birthday of the Buddha statutory
/// from 2022, the first weekday after Christmas Day from 2024, Easter
/// Monday from 2026, Good Friday from 2028 and the day following it from
/// 2030, when the two lists meet. A Sunday, or a coincidence of two
/// general holidays, is made up on the next free day, which is how a
/// Sunday Lunar New Year day yields the fourth day and a Sunday Ching Ming
/// on Easter Monday yields the Tuesday; from 1983 to 2011 the Lunar New
/// Year days and the day following the Mid-Autumn Festival were made up on
/// their eves instead, and the two computed rules carry that. Saturdays
/// move nothing.
///
/// The table is complete from the Special Administrative Region's first
/// day, 1 July 1997, with that year's 2 July, the two years of the Victory
/// Day and the day following National Day, and the one-off 3 September
/// 2015. The holidays that predate 1997 carry their real years — the Ching
/// Ming, Tuen Ng and Chung Yeung Festivals and the third Lunar New Year
/// day from 1968 — but the colonial holidays they sat beside, the Queen's
/// Birthday and Liberation Day among them, are not carried, so a year
/// before 1997 is answered incompletely. The Chinese Winter Solstice
/// Festival is an observance because an employer may give it in place of
/// Christmas Day.
pub static HONG_KONG: RuleSet = RuleSet {
    code: "HK",
    english_name: "Hong Kong",
    rules: HK_RULES,
    substitution: HK_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "GovHK, \"General holidays for 2022\", \"2026\" and \"2027\", retrieved \
              2026-09-22, for the gazetted lists and the Government's stated \
              substitution reasoning; the Chinese Wikipedia, \"香港節日與公眾假期\", \
              retrieved the same day, for the 1968, 1983, 1997, 1999 and 2011 \
              changes and the 1983–2011 eve rule; Labour Department, \"Statutory \
              holidays\" FAQ and \"Increase of statutory holidays\", for the \
              Employment Ordinance's list, its phasing-in and the Winter \
              Solstice option",
};

// ─────────────────────────────────────────────────────────────────────────
// Macau
// ─────────────────────────────────────────────────────────────────────────

/// Article 79(4) of the Public Administration Staff Statute, as amended
/// by Law 18/2018 and applied from 2019: a public holiday on a Saturday or
/// Sunday, or on another holiday, gives the public administration the
/// next working day off. The Government's yearly list names those days
/// as compensatory rest days.
static MO_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: true,
    valid_from: Some(2019),
    valid_until: None,
}];

static MO_LUNAR_NEW_YEAR: Rule = Rule::in_calendar(CalendarSystem::CHINESE, 1, 1);

static MO_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "元旦", Rule::gregorian(1, 1)),
    HolidayRule::public(
        "Lunar New Year's Day",
        "農曆正月初一",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 1),
    ),
    HolidayRule::public(
        "The second day of Lunar New Year",
        "農曆正月初二",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 2),
    ),
    HolidayRule::public(
        "The third day of Lunar New Year",
        "農曆正月初三",
        Rule::in_calendar(CalendarSystem::CHINESE, 1, 3),
    ),
    HolidayRule::public(
        "Cheng Ming Festival",
        "清明節",
        Rule::SolarTerm {
            term: QINGMING,
            meridian: hc_seasons::Meridian::CHINA,
        },
    ),
    HolidayRule::public("Good Friday", "耶穌受難日", Rule::easter(GOOD_FRIDAY)).of_kind(Kind::Bank),
    HolidayRule::public(
        "The day before Easter",
        "復活節前日",
        Rule::easter(HOLY_SATURDAY),
    )
    .of_kind(Kind::Bank),
    HolidayRule::public("Labour Day", "勞動節", Rule::gregorian(5, 1)),
    HolidayRule::public(
        "The Buddha's Birthday",
        "佛誕節",
        Rule::in_calendar(CalendarSystem::CHINESE, 4, 8),
    )
    .years(Some(2000), None)
    .of_kind(Kind::Bank),
    HolidayRule::public(
        "Tung Ng Festival",
        "端午節",
        Rule::in_calendar(CalendarSystem::CHINESE, 5, 5),
    )
    .of_kind(Kind::Bank),
    HolidayRule::public(
        "The day following Mid-Autumn Festival",
        "中秋節翌日",
        Rule::in_calendar(CalendarSystem::CHINESE, 8, 16),
    ),
    HolidayRule::public(
        "National Day of the People's Republic of China",
        "中華人民共和國國慶日",
        Rule::gregorian(10, 1),
    ),
    HolidayRule::public(
        "The day following National Day",
        "中華人民共和國國慶日翌日",
        Rule::gregorian(10, 2),
    )
    .years(Some(2000), None)
    .of_kind(Kind::Bank),
    HolidayRule::public(
        "Chong Yeung Festival",
        "重陽節",
        Rule::in_calendar(CalendarSystem::CHINESE, 9, 9),
    ),
    HolidayRule::public("All Souls' Day", "追思節", Rule::gregorian(11, 2)).of_kind(Kind::Bank),
    HolidayRule::public(
        "Feast of the Immaculate Conception",
        "聖母無原罪瞻禮",
        Rule::gregorian(12, 8),
    )
    .of_kind(Kind::Bank),
    HolidayRule::public(
        "Macao Special Administrative Region Establishment Day",
        "澳門特別行政區成立紀念日",
        Rule::gregorian(12, 20),
    )
    .years(Some(1999), None),
    HolidayRule::public(
        "Winter Solstice",
        "冬至",
        Rule::SolarTerm {
            term: SolarTerm::WINTER_SOLSTICE,
            meridian: hc_seasons::Meridian::CHINA,
        },
    )
    .of_kind(Kind::Bank),
    HolidayRule::public("Christmas Eve", "聖誕節前日", Rule::gregorian(12, 24)).of_kind(Kind::Bank),
    HolidayRule::public("Christmas Day", "聖誕節", Rule::gregorian(12, 25)).of_kind(Kind::Bank),
    // The two eves: afternoons the public administration is usually
    // exempted from work by the Chief Executive's yearly dispatch, not
    // public holidays.
    HolidayRule::observance(
        "Lunar New Year's Eve",
        "農曆除夕",
        Rule::Offset {
            base: &MO_LUNAR_NEW_YEAR,
            days: -1,
        },
    ),
    HolidayRule::observance("New Year's Eve", "公曆除夕", Rule::gregorian(12, 31)),
];

/// Macau.
///
/// The public holidays of Executive Order 60/2000, which the public
/// administration keeps, and within them the ten obligatory holidays of
/// art. 44 of the Labour Relations Law (Law 7/2008), which every employer
/// must give: those ten are [`Kind::Public`] and the rest [`Kind::Bank`],
/// a split that follows the 2008 law and is not carried further back. The
/// Buddha's Birthday and the day following National Day date from the
/// 2000 order, Establishment Day from 1999; the rest predate the order,
/// and a year before 2000 is answered with the order's list alone. From
/// 2019 a holiday on a Saturday, a Sunday or another holiday gives the
/// public administration the next working day as a compensatory rest day,
/// which is how the day before Easter, always a Saturday, yields the
/// Monday after Easter every year; nothing was made up from 1927 to 2018.
/// The two eves are afternoons the public administration is usually
/// exempted from work by yearly dispatch and are observances.
pub static MACAU: RuleSet = RuleSet {
    code: "MO",
    english_name: "Macau",
    rules: MO_RULES,
    substitution: MO_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Government of the Macao SAR, \"Public holidays\", \"2026\" and \"2027\", \
              gov.mo, retrieved 2026-09-22, for Executive Order 60/2000, the \
              obligatory holidays of art. 44 of Law 7/2008, the compensatory \
              rest days of art. 79(4) of the Public Administration Staff Statute \
              and the dates; the Chinese Wikipedia, \"澳門政府假期\", retrieved the \
              same day, for the 1999 and 2000 additions, the 2019 start of the \
              compensatory days and the eves",
};

// ─────────────────────────────────────────────────────────────────────────
// Armenia
// ─────────────────────────────────────────────────────────────────────────

/// The Day of the Citizen: the last Saturday of April, or the last Sunday
/// when that Saturday is 24 April, the Genocide Remembrance Day.
fn am_citizens_day(year: i64) -> Days {
    let days = Rule::nth(4, -1, Weekday::Saturday).days_in_year(year);
    let Some(&saturday) = days.as_slice().first() else {
        return Days::new();
    };
    if gregorian::to_fixed(year, 4, 24).ok() == Some(saturday) {
        Days::one(Rd(saturday.0 + 1))
    } else {
        Days::one(saturday)
    }
}

static AM_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Eve", "Ամանոր", Rule::gregorian(12, 31)),
    HolidayRule::fixed_public("New Year's Day", "Ամանոր", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("New Year's Day", "Ամանոր", Rule::gregorian(1, 2)),
    HolidayRule::fixed_public(
        "Christmas and Epiphany",
        "Սուրբ Ծնունդ և Հայտնություն",
        Rule::gregorian(1, 6),
    ),
    HolidayRule::observance(
        "Memorial Day after Christmas",
        "Մեռելոց",
        Rule::gregorian(1, 7),
    ),
    HolidayRule::fixed_public(
        "Day of Remembrance and Reverence",
        "Հիշատակի և խոնարհումի օր",
        Rule::gregorian(1, 27),
    )
    .years(Some(2026), None),
    HolidayRule::fixed_public("Army Day", "Բանակի օր", Rule::gregorian(1, 28)),
    HolidayRule::fixed_public("Women's Day", "Կանանց միջազգային օր", Rule::gregorian(3, 8)),
    HolidayRule::fixed_public(
        "Armenian Genocide Remembrance Day",
        "Հայոց ցեղասպանության զոհերի հիշատակի օր",
        Rule::gregorian(4, 24),
    ),
    HolidayRule::fixed_public("Labour Day", "Աշխատանքի օր", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public(
        "Victory and Peace Day",
        "Հաղթանակի և խաղաղության տոն",
        Rule::gregorian(5, 9),
    ),
    HolidayRule::fixed_public("Republic Day", "Հանրապետության տոն", Rule::gregorian(5, 28)),
    HolidayRule::fixed_public(
        "Constitution Day",
        "Սահմանադրության օր, պետական խորհրդանիշների օր",
        Rule::gregorian(7, 5),
    ),
    HolidayRule::fixed_public(
        "Independence Day",
        "Անկախության տոն",
        Rule::gregorian(9, 21),
    ),
    // The holidays and remembrance days the law keeps as working days.
    HolidayRule::observance(
        "Book Giving Day",
        "Գիրք նվիրելու օր",
        Rule::gregorian(2, 19),
    ),
    HolidayRule::observance(
        "Mother Language Day",
        "Մայրենի լեզվի օր",
        Rule::gregorian(2, 21),
    ),
    HolidayRule::observance(
        "Day of Remembrance of the Victims of the Massacres in the Azerbaijan SSR",
        "Ադրբեջանական ԽՍՀ-ում կազմակերպված ջարդերի զոհերի հիշատակի օր",
        Rule::gregorian(2, 28),
    ),
    HolidayRule::observance(
        "Motherhood and Beauty Day",
        "Մայրության և գեղեցկության տոն",
        Rule::gregorian(4, 7),
    ),
    HolidayRule::observance(
        "Day of Armenian Cinema",
        "Հայ կինոյի օր",
        Rule::gregorian(4, 16),
    ),
    HolidayRule::observance(
        "Day of the Citizen",
        "Հայաստանի քաղաքացու օր",
        Rule::Computed(am_citizens_day),
    ),
    HolidayRule::observance("Yerkrapah Day", "Երկրապահի օր", Rule::gregorian(5, 8)),
    HolidayRule::observance("Family Day", "Ընտանիքի օր", Rule::gregorian(5, 15)),
    HolidayRule::observance(
        "Students' and Youth Day",
        "Ուսանողների և երիտասարդների օր",
        Rule::gregorian(5, 16),
    ),
    HolidayRule::observance(
        "Children's Rights Protection Day",
        "Երեխաների իրավունքների պաշտպանության օր",
        Rule::gregorian(6, 1),
    ),
    HolidayRule::observance(
        "Day of Remembrance of the Repressed",
        "Բռնադատվածների հիշատակի օր",
        Rule::gregorian(6, 14),
    ),
    HolidayRule::observance(
        "Knowledge and School Day",
        "Գիտելիքի և դպրության օր",
        Rule::gregorian(9, 1),
    ),
    HolidayRule::observance("Sparapet Day", "Սպարապետի օր", Rule::gregorian(9, 12)),
    HolidayRule::observance("Teachers' Day", "Ուսուցչի օր", Rule::gregorian(10, 5)),
    HolidayRule::observance(
        "Holy Translators' Day",
        "Թարգմանչաց տոն",
        Rule::nth(10, 2, Weekday::Saturday),
    ),
    HolidayRule::observance(
        "Local Self-Government Day",
        "Տեղական ինքնակառավարման օր",
        Rule::gregorian(11, 10),
    ),
    HolidayRule::observance(
        "Day of Remembrance of the Earthquake Victims",
        "Երկրաշարժի զոհերի հիշատակի և աղետների դիմակայունության օր",
        Rule::gregorian(12, 7),
    ),
    HolidayRule::observance(
        "Day of Condemnation and Prevention of Genocides",
        "Ցեղասպանությունների դատապարտման և կանխարգելման օր",
        Rule::gregorian(12, 9),
    ),
    // The Thursday eight weeks before Easter, and the Sunday nine weeks after.
    HolidayRule::observance(
        "Saint Vardanants Day",
        "Սուրբ Վարդանանց տոն",
        Rule::easter(-59),
    ),
    HolidayRule::observance(
        "Feast of Holy Etchmiadzin",
        "Սուրբ Էջմիածնի տոն",
        Rule::easter(63),
    ),
];

/// Armenia.
///
/// The Law on Holidays and Remembrance Days of 24 June 2001 as it stands:
/// the thirteen non-working days, and the holidays and remembrance days it
/// keeps as working days as observances, the Day of the Citizen with its
/// rule for a 24 April Saturday and the two church days by their Easter
/// offsets. The Day of Remembrance and Reverence on 27 January dates from
/// the amendment of January 2026. Not carried: the longer New Year break
/// of the years before 2022, whose start the sources do not give; the
/// Merelots after Easter and the other feasts, which the Government
/// declares non-working by decision rather than by law; and the
/// Government's swapping of working days around a holiday. No
/// substitution.
pub static ARMENIA: RuleSet = RuleSet {
    code: "AM",
    english_name: "Armenia",
    rules: AM_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "The Armenian Wikipedia, \"Հայաստանի տոների և հիշատակի օրերի ցանկ\", and \
              Wikipedia, \"Public holidays in Armenia\", both retrieved 2026-09-22, \
              reproducing the Law on Holidays and Remembrance Days; the Armenian \
              Weekly, 28 January 2026, and OC Media for the 27 January amendment; \
              yerevan.am, \"Holidays and memorial days\", for the law's title",
};

// ─────────────────────────────────────────────────────────────────────────
// Azerbaijan
// ─────────────────────────────────────────────────────────────────────────

/// Labour Code art. 105(5) and (6), as applied from 2006: a rest day that
/// coincides with a non-working holiday moves to the working day after
/// the holiday, and a Qurban or Ramazan day that coincides with another
/// non-working holiday gives the next working day off. Together they made
/// 25, 26, 27 and 30 March 2026 days off after a Novruz and a Ramazan that
/// shared a weekend.
static AZ_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: true,
    valid_from: Some(2006),
    valid_until: None,
}];

static AZ_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Yeni il bayramı", Rule::gregorian(1, 1)),
    HolidayRule::public("New Year's Day", "Yeni il bayramı", Rule::gregorian(1, 2)),
    HolidayRule::public(
        "National Mourning Day",
        "Ümumxalq Hüzn Günü",
        Rule::gregorian(1, 20),
    ),
    HolidayRule::public("Women's Day", "Qadınlar günü", Rule::gregorian(3, 8)),
    HolidayRule::public("Novruz", "Novruz bayramı", Rule::gregorian(3, 20)).years(Some(2007), None),
    HolidayRule::public("Novruz", "Novruz bayramı", Rule::gregorian(3, 21)).years(Some(2007), None),
    HolidayRule::public("Novruz", "Novruz bayramı", Rule::gregorian(3, 22)).years(Some(2007), None),
    HolidayRule::public("Novruz", "Novruz bayramı", Rule::gregorian(3, 23)).years(Some(2007), None),
    HolidayRule::public("Novruz", "Novruz bayramı", Rule::gregorian(3, 24)).years(Some(2007), None),
    HolidayRule::public(
        "Victory over Fascism Day",
        "Faşizm üzərində qələbə günü",
        Rule::gregorian(5, 9),
    ),
    HolidayRule::public(
        "Independence Day",
        "Müstəqillik Günü",
        Rule::gregorian(5, 28),
    ),
    HolidayRule::public(
        "National Salvation Day",
        "Azərbaycan xalqının milli qurtuluş günü",
        Rule::gregorian(6, 15),
    )
    .years(Some(1998), None),
    HolidayRule::public(
        "Armed Forces Day",
        "Azərbaycan Respublikasının Silahlı Qüvvələri günü",
        Rule::gregorian(6, 26),
    )
    .years(Some(1998), None),
    HolidayRule::public("Victory Day", "Zəfər Günü", Rule::gregorian(11, 8))
        .years(Some(2021), None),
    HolidayRule::public(
        "State Flag Day",
        "Dövlət Bayrağı Günü",
        Rule::gregorian(11, 9),
    )
    .years(Some(2010), None),
    HolidayRule::public(
        "Solidarity Day of World Azerbaijanis",
        "Dünya azərbaycanlılarının həmrəyliyi günü",
        Rule::gregorian(12, 31),
    ),
    // Two days each, on dates the Caucasus Muslim Board announces; the
    // tabular calendar approximates them.
    HolidayRule::public(
        "Ramazan Bayramı",
        "Ramazan bayramı",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 1),
    )
    .approximate()
    .years(Some(1993), None),
    HolidayRule::public(
        "Ramazan Bayramı",
        "Ramazan bayramı",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 2),
    )
    .approximate()
    .years(Some(1993), None),
    HolidayRule::public(
        "Qurban Bayramı",
        "Qurban bayramı",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 10),
    )
    .approximate()
    .years(Some(1993), None),
    HolidayRule::public(
        "Qurban Bayramı",
        "Qurban bayramı",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 11),
    )
    .approximate()
    .years(Some(1993), None),
    // The holidays art. 105 keeps as working days.
    HolidayRule::observance(
        "State Sovereignty Day",
        "Dövlət Suverenliyi Günü",
        Rule::gregorian(9, 20),
    )
    .years(Some(2024), None),
    HolidayRule::observance("Remembrance Day", "Anım Günü", Rule::gregorian(9, 27))
        .years(Some(2021), None),
    HolidayRule::observance(
        "Restoration of Independence Day",
        "Müstəqilliyin Bərpası Günü",
        Rule::gregorian(10, 18),
    ),
    HolidayRule::observance(
        "Constitution Day",
        "Konstitusiya günü",
        Rule::gregorian(11, 12),
    ),
    HolidayRule::observance(
        "National Revival Day",
        "Milli Dirçəliş günü",
        Rule::gregorian(11, 17),
    ),
];

/// Azerbaijan.
///
/// Article 105 of the Labour Code: the holidays that are non-working days
/// as [`Kind::Public`], and the four the article keeps as working days as
/// observances. Novruz is five days from the amendment of 8 December 2006,
/// and its earlier form is not carried; Ramazan and Qurban are two days
/// each from 1993 on dates the Caucasus Muslim Board announces, which the
/// tabular calendar approximates. National Salvation Day and Armed Forces
/// Day date from 1998, the State Flag Day from 2010 and Victory Day from
/// 2021. From 2006 a rest day that coincides with a holiday moves to the
/// working day after it, and a Qurban or Ramazan day that coincides with
/// another holiday gives the next working day off, which a forward policy
/// with collisions carries and 2026's March reproduces. The Ministry's
/// swapping of working and rest days around a holiday, art. 105(7), is
/// not carried.
pub static AZERBAIJAN: RuleSet = RuleSet {
    code: "AZ",
    english_name: "Azerbaijan",
    rules: AZ_RULES,
    substitution: AZ_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Cabinet of Ministers of the Republic of Azerbaijan, \"Holidays\", \
              nk.gov.az, retrieved 2026-09-22, for art. 105 and the rest-day rule; \
              the Azerbaijani Wikipedia, \"Azərbaycanın dövlət bayramları və xüsusi \
              günləri\", retrieved the same day, for the article's text, the 2006 \
              amendment and the years; Wikipedia, \"Public holidays in Azerbaijan\", \
              for the English names; APA and Modern.az, for the non-working days of \
              March 2026",
};

// ─────────────────────────────────────────────────────────────────────────
// Georgia
// ─────────────────────────────────────────────────────────────────────────

static GE_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("New Year's Day", "ახალი წელი", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("New Year's Day", "ახალი წელი", Rule::gregorian(1, 2)),
    HolidayRule::fixed_public("Orthodox Christmas", "ქრისტეშობა", Rule::gregorian(1, 7)),
    HolidayRule::fixed_public("Orthodox Epiphany", "ნათლისღება", Rule::gregorian(1, 19)),
    HolidayRule::fixed_public("Mother's Day", "დედის დღე", Rule::gregorian(3, 3)),
    HolidayRule::fixed_public(
        "International Women's Day",
        "ქალთა საერთაშორისო დღე",
        Rule::gregorian(3, 8),
    ),
    HolidayRule::fixed_public(
        "National Unity Day",
        "ეროვნული ერთიანობის დღე",
        Rule::gregorian(4, 9),
    ),
    HolidayRule::fixed_public(
        "Good Friday",
        "წითელი პარასკევი",
        Rule::paschal(GOOD_FRIDAY),
    ),
    HolidayRule::fixed_public("Holy Saturday", "დიდი შაბათი", Rule::paschal(HOLY_SATURDAY)),
    HolidayRule::fixed_public(
        "Easter Sunday",
        "ბრწყინვალე აღდგომა",
        Rule::paschal(EASTER_SUNDAY),
    ),
    HolidayRule::fixed_public(
        "Easter Monday",
        "აღდგომის ორშაბათი",
        Rule::paschal(EASTER_MONDAY),
    ),
    HolidayRule::fixed_public(
        "Day of Victory over Fascism",
        "ფაშიზმზე გამარჯვების დღე",
        Rule::gregorian(5, 9),
    ),
    HolidayRule::fixed_public(
        "Saint Andrew the First-Called Day",
        "წმინდა ანდრია პირველწოდებულის ხსენების დღე",
        Rule::gregorian(5, 12),
    ),
    HolidayRule::fixed_public(
        "Day of Family Purity and Respect for Parents",
        "ოჯახის სიწმინდისა და მშობლების პატივისცემის დღე",
        Rule::gregorian(5, 17),
    ),
    HolidayRule::fixed_public(
        "Independence Day",
        "დამოუკიდებლობის დღე",
        Rule::gregorian(5, 26),
    ),
    HolidayRule::fixed_public(
        "Dormition of the Mother of God",
        "მარიამობა",
        Rule::gregorian(8, 28),
    ),
    HolidayRule::fixed_public("Svetitskhovloba", "სვეტიცხოვლობა", Rule::gregorian(10, 14)),
    HolidayRule::fixed_public("Saint George's Day", "გიორგობა", Rule::gregorian(11, 23)),
];

/// Georgia.
///
/// Article 30 of the Organic Law "Labour Code of Georgia": thirteen fixed
/// days and the Orthodox Easter from Good Friday to Easter Monday by the
/// Julian computus. The article says nothing about a holiday on a weekend,
/// and nothing moves. The years the days were added are not carried, the
/// source giving none.
pub static GEORGIA: RuleSet = RuleSet {
    code: "GE",
    english_name: "Georgia",
    rules: GE_RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Organic Law of Georgia, Labour Code of Georgia, art. 30, as published in \
              English by the Legislative Herald of Georgia, matsne.gov.ge, retrieved \
              2026-09-22; Wikipedia, \"Public holidays in Georgia (country)\", \
              retrieved the same day, for the Georgian names",
};

// ─────────────────────────────────────────────────────────────────────────
// Kazakhstan
// ─────────────────────────────────────────────────────────────────────────

/// Article 5 of the Law on Holidays: when a rest day coincides with a
/// holiday, the working day after the holiday is the rest day.
static KZ_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static KZ_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Жаңа жыл", Rule::gregorian(1, 1)),
    HolidayRule::public("New Year's Day", "Жаңа жыл", Rule::gregorian(1, 2)),
    // A day off under the amendment of 30 December 2005, with Kurban Ait.
    HolidayRule::public(
        "Orthodox Christmas",
        "Православиелік Рождество",
        Rule::gregorian(1, 7),
    )
    .years(Some(2006), None),
    HolidayRule::public(
        "International Women's Day",
        "Халықаралық әйелдер күні",
        Rule::gregorian(3, 8),
    ),
    // The Constitution of 15 March 2026 moved its day from 30 August by
    // the law of 11 June 2026, in force from 1 July: 30 August 2026 was
    // not a day off and 15 March is one from 2027.
    HolidayRule::public(
        "Constitution Day",
        "Конституция күні",
        Rule::gregorian(3, 15),
    )
    .years(Some(2027), None),
    HolidayRule::public("Nauryz Meyramy", "Наурыз мейрамы", Rule::gregorian(3, 22))
        .years(None, Some(2008)),
    HolidayRule::public("Nauryz Meyramy", "Наурыз мейрамы", Rule::gregorian(3, 21))
        .years(Some(2009), None),
    HolidayRule::public("Nauryz Meyramy", "Наурыз мейрамы", Rule::gregorian(3, 22))
        .years(Some(2009), None),
    HolidayRule::public("Nauryz Meyramy", "Наурыз мейрамы", Rule::gregorian(3, 23))
        .years(Some(2009), None),
    HolidayRule::public(
        "Kazakhstan People's Unity Day",
        "Қазақстан халқының бірлігі мерекесі",
        Rule::gregorian(5, 1),
    ),
    HolidayRule::public(
        "Defender of the Fatherland Day",
        "Отан қорғаушы күні",
        Rule::gregorian(5, 7),
    )
    .years(Some(2013), None),
    HolidayRule::public("Victory Day", "Жеңіс күні", Rule::gregorian(5, 9)),
    HolidayRule::public("Capital City Day", "Астана күні", Rule::gregorian(7, 6))
        .years(Some(2008), None),
    HolidayRule::public(
        "Constitution Day",
        "Конституция күні",
        Rule::gregorian(8, 30),
    )
    .years(None, Some(2025)),
    HolidayRule::public("Republic Day", "Республика күні", Rule::gregorian(10, 25))
        .years(Some(1995), Some(2008)),
    HolidayRule::public("Republic Day", "Республика күні", Rule::gregorian(10, 25))
        .years(Some(2022), None),
    HolidayRule::public(
        "First President Day",
        "Қазақстан Республикасының Тұңғыш Президенті күні",
        Rule::gregorian(12, 1),
    )
    .years(Some(2012), Some(2021)),
    HolidayRule::public(
        "Independence Day",
        "Тәуелсіздік күні",
        Rule::gregorian(12, 16),
    ),
    HolidayRule::public(
        "Independence Day",
        "Тәуелсіздік күні",
        Rule::gregorian(12, 17),
    )
    .years(None, Some(2021)),
    HolidayRule::public(
        "Kurban Ait",
        "Құрбан айт",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 10),
    )
    .approximate()
    .years(Some(2006), None),
];

/// Kazakhstan.
///
/// The Law on Holidays of 13 December 2001 as amended: the national and
/// state holidays, and the two days off the amendment of 30 December 2005
/// added, Orthodox Christmas and the first day of Kurban Ait, the latter
/// on a date the Muslim Board announces and the tabular calendar
/// approximates. The changes the sources date are years: Nauryz one day
/// on 22 March until 2008 and three from 2009; Republic Day a holiday from
/// 1995 until the amendment of April 2009 and again, as the national
/// holiday, from 2022; Capital City Day from 2008; Defender of the
/// Fatherland Day from 2013; First President Day from 2012 to 2021 and
/// 17 December to 2021, both dropped by the amendment of 29 September
/// 2022; and Constitution Day on 30 August until 2025 and on 15 March from
/// 2027, moved by the law of 11 June 2026 after the Constitution of
/// 15 March 2026. Article 5 moves a holiday on a rest day to the working
/// day after. The table is complete from the 2001 law; the Government's
/// yearly bridges are not carried.
pub static KAZAKHSTAN: RuleSet = RuleSet {
    code: "KZ",
    english_name: "Kazakhstan",
    rules: KZ_RULES,
    substitution: KZ_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Закон Республики Казахстан от 13 декабря 2001 года № 267 «О праздниках в \
              Республике Казахстан», as consolidated by Параграф (prg.kz) to the law \
              of 11 June 2026, retrieved 2026-09-22; the Russian Wikipedia, \
              \"Праздники Казахстана\", retrieved the same day, for the dates of the \
              amendments; Wikipedia, \"Public holidays in Kazakhstan\", for the Kazakh \
              names; pro1c.kz and Tengrinews for the 2022 and 2026 changes; \
              Inform.kz for Republic Day's removal in April 2009",
};

// ─────────────────────────────────────────────────────────────────────────
// Uzbekistan
// ─────────────────────────────────────────────────────────────────────────

/// Article 208: "when a day off coincides with a holiday, the day off is
/// transferred to the working day following the holiday".
static UZ_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static UZ_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Yangi yil", Rule::gregorian(1, 1)),
    HolidayRule::public("Women's Day", "Xotin-qizlar kuni", Rule::gregorian(3, 8)),
    HolidayRule::public("Navruz", "Navroʻz bayrami", Rule::gregorian(3, 21)),
    HolidayRule::public("Victory Day", "Gʻalaba kuni", Rule::gregorian(5, 9))
        .years(None, Some(1998)),
    HolidayRule::public(
        "Day of Remembrance and Honour",
        "Xotira va qadrlash kuni",
        Rule::gregorian(5, 9),
    )
    .years(Some(1999), None),
    HolidayRule::public(
        "Independence Day",
        "Mustaqillik kuni",
        Rule::gregorian(9, 1),
    )
    .years(Some(1991), None),
    HolidayRule::public(
        "Teachers' and Mentors' Day",
        "Oʻqituvchi va murabbiylar kuni",
        Rule::gregorian(10, 1),
    )
    .years(Some(1997), None),
    HolidayRule::public(
        "Constitution Day",
        "Konstitutsiya kuni",
        Rule::gregorian(12, 8),
    ),
    HolidayRule::public(
        "Ruza Hayit",
        "Roʻza hayiti",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 1),
    )
    .approximate(),
    HolidayRule::public(
        "Kurban Hayit",
        "Qurbon hayiti",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 10),
    )
    .approximate(),
];

/// Uzbekistan.
///
/// The Labour Code of 28 October 2022 (ЗРУ-798), in force from 30 April
/// 2023, article 208: seven fixed days and the first day of each of Ruza
/// Hayit and Kurban Hayit, the two on the tabular Hijri calendar as an
/// approximation of the dates the Government announces; and "when a day
/// off coincides with a holiday, the day off is transferred to the working
/// day following the holiday", which is the Saturday-and-Sunday policy —
/// Saturday 21 March 2026 gave Monday the 23rd. The additional days off
/// and the transfers the President decrees each year under the same
/// article are not carried. 9 May was Victory Day until 1998 and the Day
/// of Remembrance and Honour from 1999; Teachers' and Mentors' Day began
/// in 1997.
pub static UZBEKISTAN: RuleSet = RuleSet {
    code: "UZ",
    english_name: "Uzbekistan",
    rules: UZ_RULES,
    substitution: UZ_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Labour Code of the Republic of Uzbekistan (Law ЗРУ-798 of 28 October 2022), \
              article 208, as lex.uz publishes it, retrieved 2026-09-22; gazeta.uz on the \
              2026 Navruz transfer under Presidential Decree No. 106 of 17 March 2026; \
              Wikipedia (ru), \"Праздники Узбекистана\", for the Uzbek names and 1997, and \
              Wikipedia, \"Victory Day (9 May)\", for 1999",
};

// ─────────────────────────────────────────────────────────────────────────
// Kyrgyzstan
// ─────────────────────────────────────────────────────────────────────────

/// Article 113 of the 2004 code: "when a day off coincides with a
/// non-working holiday, the day off is transferred to the working day
/// following the holiday". The 2025 code has no such rule.
static KG_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: Some(2024),
}];

/// A state holiday that the 2025 code keeps as a holiday but makes a
/// working day: public through 2024, an observance from 2025.
const fn kg_demoted(
    name: &'static str,
    local: &'static str,
    month: u8,
    day: u8,
) -> [HolidayRule; 2] {
    [
        HolidayRule::public(name, local, Rule::gregorian(month, day)).years(None, Some(2024)),
        HolidayRule::observance(name, local, Rule::gregorian(month, day)).years(Some(2025), None),
    ]
}

const KG_DEFENDER: [HolidayRule; 2] = kg_demoted(
    "Defender of the Fatherland Day",
    "Мекенди коргоочунун күнү",
    2,
    23,
);
const KG_APRIL_7: [HolidayRule; 2] = kg_demoted(
    "Day of the People's April Revolution",
    "Элдик Апрель революциясы күнү",
    4,
    7,
);
const KG_NOVEMBER_7: [HolidayRule; 2] =
    kg_demoted("Days of History and Commemoration of Ancestors", "", 11, 7);
const KG_NOVEMBER_8: [HolidayRule; 2] =
    kg_demoted("Days of History and Commemoration of Ancestors", "", 11, 8);

static KG_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Жаңы жыл", Rule::gregorian(1, 1)),
    HolidayRule::fixed_public("New Year Holidays", "", Rule::gregorian(1, 2))
        .years(Some(2026), None),
    HolidayRule::fixed_public("New Year Holidays", "", Rule::gregorian(1, 3))
        .years(Some(2026), None),
    HolidayRule::fixed_public("New Year Holidays", "", Rule::gregorian(1, 4))
        .years(Some(2026), None),
    HolidayRule::fixed_public("New Year Holidays", "", Rule::gregorian(1, 5))
        .years(Some(2026), None),
    HolidayRule::fixed_public("New Year Holidays", "", Rule::gregorian(1, 6))
        .years(Some(2026), None),
    HolidayRule::public(
        "Orthodox Christmas",
        "Төрөлүү майрамы",
        Rule::gregorian(1, 7),
    ),
    KG_DEFENDER[0],
    KG_DEFENDER[1],
    HolidayRule::public(
        "International Women's Day",
        "Эл аралык аялдар күнү",
        Rule::gregorian(3, 8),
    ),
    HolidayRule::public("Nooruz", "Нооруз", Rule::gregorian(3, 21)),
    HolidayRule::public(
        "Day of the People's April Revolution",
        "Элдик Апрель революциясы күнү",
        Rule::gregorian(4, 7),
    )
    .years(Some(2016), Some(2024)),
    KG_APRIL_7[1],
    HolidayRule::public("Labour Day", "Эмгек күнү", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("May Holidays", "", Rule::gregorian(5, 2)).years(Some(2025), None),
    HolidayRule::fixed_public("May Holidays", "", Rule::gregorian(5, 3)).years(Some(2025), None),
    HolidayRule::fixed_public("May Holidays", "", Rule::gregorian(5, 4)).years(Some(2025), None),
    HolidayRule::public(
        "Constitution Day",
        "Кыргыз Республикасынын Конституция күнү",
        Rule::gregorian(5, 5),
    ),
    HolidayRule::fixed_public("May Holidays", "", Rule::gregorian(5, 6)).years(Some(2025), None),
    HolidayRule::fixed_public("May Holidays", "", Rule::gregorian(5, 7)).years(Some(2025), None),
    HolidayRule::fixed_public("May Holidays", "", Rule::gregorian(5, 8)).years(Some(2025), None),
    HolidayRule::public("Victory Day", "Жеңиш күнү", Rule::gregorian(5, 9)),
    HolidayRule::public(
        "Independence Day",
        "Кыргыз Республикасынын Эгемендүүлүк күнү",
        Rule::gregorian(8, 31),
    ),
    HolidayRule::public(
        "Day of the Great October Socialist Revolution",
        "",
        Rule::gregorian(11, 7),
    )
    .years(None, Some(2017)),
    HolidayRule::public(
        "Days of History and Commemoration of Ancestors",
        "",
        Rule::gregorian(11, 7),
    )
    .years(Some(2018), Some(2024)),
    HolidayRule::public(
        "Days of History and Commemoration of Ancestors",
        "",
        Rule::gregorian(11, 8),
    )
    .years(Some(2018), Some(2024)),
    KG_NOVEMBER_7[1],
    KG_NOVEMBER_8[1],
    HolidayRule::public(
        "Orozo Ait",
        "Орозо айт",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 1),
    )
    .approximate(),
    HolidayRule::public(
        "Kurman Ait",
        "Курман айт",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 10),
    )
    .approximate(),
];

/// Kyrgyzstan.
///
/// Two codes. The Labour Code of 4 August 2004 (No. 106), article 113 as
/// last amended: twelve fixed days, Orozo Ait and Kurman Ait "by the
/// lunar calendar", and "when a day off coincides with a non-working
/// holiday, the day off is transferred to the working day following the
/// holiday", the Saturday-and-Sunday policy, which ends with the code in
/// 2024. The Day of the People's April Revolution joined the list by the
/// law of 6 April 2016; the Days of History and Commemoration of
/// Ancestors replaced the Day of the Great October Socialist Revolution
/// by the law of 22 November 2017, from 2018. The Labour Code of
/// 23 January 2025 (No. 23), article 66, keeps the state holidays but
/// makes only some of them days off: New Year holidays on 1 to 6 January,
/// carried from 2026 as the first January under it; 8 and 21 March; May
/// holidays on 1 to 4 and 6 to 8 May around Constitution Day and Victory
/// Day; 31 August; Orthodox Christmas; and the two Muslim holidays, on
/// the tabular Hijri calendar as approximations. Defender of the
/// Fatherland Day, 7 April and 7–8 November stay state holidays on
/// working days, and are observances from 2025; the code has no transfer
/// rule, which the Ministry of Labour's summary calls its removal. The
/// days the Cabinet moves each year are not carried.
pub static KYRGYZSTAN: RuleSet = RuleSet {
    code: "KG",
    english_name: "Kyrgyzstan",
    rules: KG_RULES,
    substitution: KG_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Labour Code of the Kyrgyz Republic (No. 23 of 23 January 2025), article 66, \
              from the copy on isito.kg, retrieved 2026-09-22; Labour Code of the Kyrgyz \
              Republic (No. 106 of 4 August 2004), article 113 as last amended, as \
              continent-online.com publishes it; the Ministry of Labour's summaries of \
              17 April 2024 and 8 April 2025 (mlsp.gov.kg); K-News on the law of \
              22 November 2017 and Kaktus on the law of 6 April 2016; Wikipedia, \"Public \
              holidays in Kyrgyzstan\", for the Kyrgyz names",
};

// ─────────────────────────────────────────────────────────────────────────
// Tajikistan
// ─────────────────────────────────────────────────────────────────────────

/// Labour Code article 89(5): "when a day off coincides with a
/// non-working holiday, the day off is transferred to the working day
/// following the non-working holiday".
static TJ_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

static TJ_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Соли нав", Rule::gregorian(1, 1)),
    HolidayRule::public("Mother's Day", "Рӯзи модар", Rule::gregorian(3, 8)),
    HolidayRule::public("Navruz", "Наврӯз", Rule::gregorian(3, 21)),
    HolidayRule::public("Navruz", "Наврӯз", Rule::gregorian(3, 22)),
    HolidayRule::public("Navruz", "Наврӯз", Rule::gregorian(3, 23)),
    HolidayRule::public("Navruz", "Наврӯз", Rule::gregorian(3, 24)),
    HolidayRule::public("Labour Day", "Рӯзи меҳнат", Rule::gregorian(5, 1)).years(None, Some(2016)),
    HolidayRule::public("Victory Day", "Рӯзи Ғалаба", Rule::gregorian(5, 9)),
    HolidayRule::public(
        "National Unity Day",
        "Рӯзи Ваҳдати миллӣ",
        Rule::gregorian(6, 27),
    ),
    HolidayRule::public(
        "Independence Day",
        "Рӯзи Истиқлолияти давлатии Ҷумҳурии Тоҷикистон",
        Rule::gregorian(9, 9),
    ),
    HolidayRule::public(
        "Constitution Day",
        "Рӯзи Конститутсияи Ҷумҳурии Тоҷикистон",
        Rule::gregorian(11, 6),
    ),
    HolidayRule::public(
        "Idi Ramazon",
        "Иди Рамазон",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 1),
    )
    .approximate(),
    HolidayRule::public(
        "Idi Kurbon",
        "Иди Қурбон",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, 10),
    )
    .approximate(),
];

/// Tajikistan.
///
/// The Law on Holidays of 2 August 2011 (No. 753) as amended, article 3,
/// from the National Centre of Legislation's text: New Year's Day,
/// Mother's Day, the four days of Navruz, Victory Day, National Unity
/// Day, Independence Day, Constitution Day, and one day each of Ramazon
/// and Kurbon "set annually by the authorised state body for religion",
/// carried on the tabular Hijri calendar as approximations. 1 May was
/// struck from the article by the law of 24 February 2017 and ends in
/// 2016. The Labour Code of 23 July 2016 (No. 1329), article 89(5): "when
/// a day off coincides with a non-working holiday, the day off is
/// transferred to the working day following the non-working holiday",
/// the Saturday-and-Sunday policy, which after a weekend Navruz runs two
/// days past the 24th; the transfers the Government makes under
/// article 89(4) beyond that are not carried, nor are the working
/// holidays of article 2, from Armed Forces Day to Flag Day.
pub static TAJIKISTAN: RuleSet = RuleSet {
    code: "TJ",
    english_name: "Tajikistan",
    rules: TJ_RULES,
    substitution: TJ_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Law of the Republic of Tajikistan on Holidays (No. 753 of 2 August 2011) as \
              amended, articles 2 and 3, as the National Centre of Legislation publishes it \
              (ncz.tj), retrieved 2026-09-22; Labour Code of the Republic of Tajikistan \
              (No. 1329 of 23 July 2016), article 89, from the Tax Committee's copy \
              (andoz.tj); Vecherka on the 2017 removal of 1 May; Wikipedia (ru), \
              \"Праздники Таджикистана\", for the Tajik names",
};

// ─────────────────────────────────────────────────────────────────────────
// Turkmenistan
// ─────────────────────────────────────────────────────────────────────────

/// Article 81(2): "when a non-working holiday or memorial day coincides
/// with the day off (Sunday), the day off is the working day following
/// it".
static TM_SUBSTITUTION: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: true,
    on_collision: false,
    valid_from: None,
    valid_until: None,
}];

/// Kurban Bayram on the three days the President's decrees have given it,
/// 6–8 June 2025 and 27–29 May 2026.
const fn tm_kurban(day: u8) -> HolidayRule {
    HolidayRule::public(
        "Kurban Bayram",
        "Gurban baýramy",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 12, day),
    )
    .approximate()
}

static TM_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "Täze ýyl", Rule::gregorian(1, 1)),
    HolidayRule::public(
        "State Flag Day",
        "Türkmenistanyň Döwlet baýdagynyň güni",
        Rule::gregorian(2, 19),
    )
    .years(Some(1995), Some(2017)),
    HolidayRule::public(
        "International Women's Day",
        "Halkara zenanlar güni",
        Rule::gregorian(3, 8),
    ),
    HolidayRule::public("Nowruz", "Milli bahar baýramy", Rule::gregorian(3, 21)),
    HolidayRule::public("Nowruz", "Milli bahar baýramy", Rule::gregorian(3, 22)),
    HolidayRule::public(
        "Constitution Day",
        "Türkmenistanyň Konstitusiýasynyň güni",
        Rule::gregorian(5, 18),
    )
    .years(None, Some(2017)),
    HolidayRule::public(
        "Constitution and State Flag Day",
        "Türkmenistanyň Konstitusiýasynyň we Döwlet baýdagynyň güni",
        Rule::gregorian(5, 18),
    )
    .years(Some(2018), None),
    HolidayRule::public(
        "Independence Day",
        "Türkmenistanyň Garaşsyzlyk güni",
        Rule::gregorian(10, 27),
    )
    .years(Some(1991), Some(2017)),
    HolidayRule::public(
        "Independence Day",
        "Türkmenistanyň Garaşsyzlyk güni",
        Rule::gregorian(9, 27),
    )
    .years(Some(2018), None),
    HolidayRule::public("Day of Remembrance", "Hatyra güni", Rule::gregorian(10, 6)),
    HolidayRule::public(
        "Neutrality Day",
        "Halkara Bitaraplyk güni",
        Rule::gregorian(12, 12),
    ),
    HolidayRule::public(
        "Oraza Bayram",
        "Oraza baýramy",
        Rule::in_calendar(CalendarSystem::ISLAMIC_CIVIL, 10, 1),
    )
    .approximate(),
    tm_kurban(10),
    tm_kurban(11),
    tm_kurban(12),
];

/// Turkmenistan.
///
/// The Labour Code of 18 April 2009, article 81, as the Ombudsman's copy
/// prints it: work stops on New Year's Day, International Women's Day, the
/// two days of the National Spring Holiday, Constitution and State Flag
/// Day, Independence Day, the Day of Remembrance, Neutrality Day, and
/// Kurban and Oraza Bayram on dates the President decrees — three days
/// and one in 2025 and 2026, carried on the tabular Hijri calendar as
/// approximations; and "when a non-working holiday or memorial day
/// coincides with the day off (Sunday), the day off is the working day
/// following it", the Sunday policy, which the decree of 16 March 2026
/// applied to Sunday 22 March. The memorial days without days off, and
/// the days the Cabinet moves under paragraph 3, are not carried. The law
/// of 9 October 2017, in force from 2018, moved Independence Day from
/// 27 October to 27 September and merged State Flag Day, a day off on
/// 19 February since 1995, into Constitution Day on 18 May.
pub static TURKMENISTAN: RuleSet = RuleSet {
    code: "TM",
    english_name: "Turkmenistan",
    rules: TM_RULES,
    substitution: TM_SUBSTITUTION,
    bridges: &[],
    includes: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 22),
    sources: "Labour Code of Turkmenistan (18 April 2009, as amended), article 81, from the \
              Ombudsman's copy (ombudsman.gov.tm), retrieved 2026-09-22; the Embassy of \
              Turkmenistan in Bishkek's list of holidays and memorial dates \
              (tmembassy.gov.tm) for the Turkmen names; the Presidential Decrees on Oraza \
              Bayram and the 22 March transfer (16 March 2026) and on Kurban Bayram \
              (22 May 2026) on turkmenistan.gov.tm, and oilgas.gov.tm's 2025 list; \
              Wikipedia, \"State Flag and Constitution Day (Turkmenistan)\", and Wikipedia \
              (ru), \"День независимости Туркменистана\", for 1995, 2017 and 2018",
};
