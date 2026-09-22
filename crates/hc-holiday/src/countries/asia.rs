//! Asian national tables, other than Japan.
//!
//! This is where the calendars stop being Gregorian. China, Taiwan, Korea
//! and Vietnam key their great festivals to four *different* lunisolar
//! calendars — the same algorithm at four different meridians — and Chinese
//! New Year, Seollal and Tết are not always the same day. Each table names
//! its own calendar rather than borrowing China's.

use hc_calendar::{Rd, Weekday};
use hc_seasons::SolarTerm;

use crate::computus::offsets::{
    ASCENSION, EASTER_MONDAY, GOOD_FRIDAY, HOLY_SATURDAY, MAUNDY_THURSDAY,
};
use crate::hindu::{
    BUDDHA_PURNIMA, DIWALI, GURU_NANAK_JAYANTI, HOLI, JANMASHTAMI, MAHAVIR_JAYANTI, RAMA_NAVAMI,
    VIJAYA_DASHAMI,
};
use crate::rule::{
    CalendarSystem, Days, HolidayRule, Kind, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate,
    SubstituteDirection, SubstitutionPolicy, WeekendPolicy,
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
];

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
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "관공서의 공휴일에 관한 규정 (대통령령), including the 2013 \
              amendment introducing 대체공휴일 for Seollal, Chuseok and \
              Children's Day, the July 2021 extension to the four national \
              days, and the 2023 extension to Buddha's Birthday and \
              Christmas. Seollal and Chuseok are dated in the `dangi` \
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
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Holidays Act 1998, schedule; Ministry of Manpower's annual \
              list. Deepavali is a public holiday and is absent because it \
              needs a Hindu calendar; the two Islamic days are the tabular \
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
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Holidays Act 1951, schedule; the federal gazette's annual \
              list. State holidays are not modelled, and neither is the \
              Friday–Saturday weekend of Johor, Kedah, Kelantan and \
              Terengganu. Deepavali and Thaipusam need a Hindu calendar and \
              are absent",
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

static NP_RULES: &[HolidayRule] = &[
    HolidayRule::fixed_public("Prithvi Jayanti", "पृथ्वी जयन्ती", Rule::gregorian(1, 11))
        .approximate(),
    HolidayRule::fixed_public("Martyrs' Day", "शहीद दिवस", Rule::gregorian(1, 30)).approximate(),
    HolidayRule::fixed_public(
        "National Democracy Day",
        "प्रजातन्त्र दिवस",
        Rule::gregorian(2, 19),
    )
    .approximate(),
    HolidayRule::fixed_public(
        "International Women's Day",
        "नारी दिवस",
        Rule::gregorian(3, 8),
    ),
    HolidayRule::fixed_public("Labour Day", "श्रमिक दिवस", Rule::gregorian(5, 1)),
    HolidayRule::fixed_public("Republic Day", "गणतन्त्र दिवस", Rule::gregorian(5, 29)).approximate(),
    HolidayRule::fixed_public("Constitution Day", "संविधान दिवस", Rule::gregorian(9, 19))
        .approximate(),
];

/// Nepal — the weekend rule, and very little else.
pub static NEPAL: RuleSet = RuleSet {
    code: "NP",
    english_name: "Nepal",
    rules: NP_RULES,
    substitution: &[],
    bridges: &[],
    weekend: NP_WEEKEND,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Government of Nepal, Ministry of Home Affairs, annual public \
              holiday notice. DELIBERATELY THIN: Nepal's holidays are dated \
              in Bikram Sambat and dominated by Hindu and Buddhist festivals \
              that need calendars this crate does not have. Only the days \
              whose Bikram Sambat date maps to a near-fixed Gregorian one \
              are listed, and each is flagged approximate because that \
              mapping moves by a day. The table exists chiefly for the \
              one-day weekend",
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
