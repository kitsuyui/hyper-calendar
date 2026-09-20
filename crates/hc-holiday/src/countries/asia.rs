//! Asian national tables, other than Japan.
//!
//! This is where the calendars stop being Gregorian. China, Taiwan, Korea
//! and Vietnam key their great festivals to four *different* lunisolar
//! calendars — the same algorithm at four different meridians — and Chinese
//! New Year, Seollal and Tết are not always the same day. Each table names
//! its own calendar rather than borrowing China's.

use hc_calendar::Weekday;
use hc_seasons::SolarTerm;

use crate::computus::offsets::{ASCENSION, GOOD_FRIDAY, MAUNDY_THURSDAY};
use crate::rule::{
    CalendarSystem, HolidayRule, Kind, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate,
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
const EID_AL_FITR: Rule = Rule::in_calendar(CalendarSystem::IslamicCivil, 10, 1);
const EID_AL_ADHA: Rule = Rule::in_calendar(CalendarSystem::IslamicCivil, 12, 10);
const HIJRI_NEW_YEAR: Rule = Rule::in_calendar(CalendarSystem::IslamicCivil, 1, 1);
const MAWLID: Rule = Rule::in_calendar(CalendarSystem::IslamicCivil, 3, 12);

// ─────────────────────────────────────────────────────────────────────────
// China
// ─────────────────────────────────────────────────────────────────────────

static CN_NEW_YEAR: Rule = Rule::in_calendar(CalendarSystem::Chinese, 1, 1);

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
        Rule::in_calendar(CalendarSystem::Chinese, 1, 1),
    ),
    HolidayRule::fixed_public(
        "Spring Festival",
        "春节",
        Rule::in_calendar(CalendarSystem::Chinese, 1, 2),
    ),
    HolidayRule::fixed_public(
        "Spring Festival",
        "春节",
        Rule::in_calendar(CalendarSystem::Chinese, 1, 3),
    )
    .years(None, Some(2007)),
    HolidayRule::fixed_public(
        "Spring Festival",
        "春节",
        Rule::in_calendar(CalendarSystem::Chinese, 1, 3),
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
        Rule::in_calendar(CalendarSystem::Chinese, 5, 5),
    )
    .years(Some(2008), None),
    HolidayRule::fixed_public(
        "Mid-Autumn Festival",
        "中秋节",
        Rule::in_calendar(CalendarSystem::Chinese, 8, 15),
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

static TW_NEW_YEAR: Rule = Rule::in_calendar(CalendarSystem::Chinese, 1, 1);

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
        Rule::in_calendar(CalendarSystem::Chinese, 1, 1),
    ),
    HolidayRule::fixed_public(
        "Spring Festival",
        "春節",
        Rule::in_calendar(CalendarSystem::Chinese, 1, 2),
    ),
    HolidayRule::fixed_public(
        "Spring Festival",
        "春節",
        Rule::in_calendar(CalendarSystem::Chinese, 1, 3),
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
        Rule::in_calendar(CalendarSystem::Chinese, 5, 5),
    ),
    HolidayRule::public(
        "Mid-Autumn Festival",
        "中秋節",
        Rule::in_calendar(CalendarSystem::Chinese, 8, 15),
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

static KR_SEOLLAL: Rule = Rule::in_calendar(CalendarSystem::Dangi, 1, 1);
static KR_CHUSEOK: Rule = Rule::in_calendar(CalendarSystem::Dangi, 8, 15);

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
        Rule::in_calendar(CalendarSystem::Dangi, 1, 1),
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
        Rule::in_calendar(CalendarSystem::Dangi, 4, 8),
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
        Rule::in_calendar(CalendarSystem::Dangi, 8, 15),
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
        Rule::in_calendar(CalendarSystem::IslamicCivil, 1, 10),
    )
    .approximate(),
    HolidayRule::fixed_public("Milad-un-Nabi", "ईद मिलाद उन-नबी", MAWLID).approximate(),
];

/// India: the three national holidays plus the gazetted days this crate can
/// actually compute.
pub static INDIA: RuleSet = RuleSet {
    code: "IN",
    english_name: "India",
    rules: IN_RULES,
    substitution: &[],
    bridges: &[],
    weekend: SATURDAY_SUNDAY,
    sources_checked: SourceDate::new(2026, 9, 21),
    sources: "Department of Personnel and Training, \"List of Holidays\", \
              issued annually. INCOMPLETE BY DESIGN: the Hindu, Sikh and \
              Jain gazetted holidays — Holi, Diwali, Dussehra, Janmashtami, \
              Mahavir Jayanti, Guru Nanak's Birthday and the rest — need a \
              `hindu-lunar` calendar that `hc-calendars-lunar` does not yet \
              have, and this crate will not tabulate what it cannot compute",
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
static TH_ASALHA: Rule = Rule::in_calendar(CalendarSystem::Chinese, 6, 15);

static TH_RULES: &[HolidayRule] = &[
    HolidayRule::public("New Year's Day", "วันขึ้นปีใหม่", Rule::gregorian(1, 1)),
    HolidayRule::public(
        "Makha Bucha",
        "วันมาฆบูชา",
        Rule::in_calendar(CalendarSystem::Chinese, 1, 15),
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
        Rule::in_calendar(CalendarSystem::Chinese, 4, 15),
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

static VN_TET: Rule = Rule::in_calendar(CalendarSystem::Vietnamese, 1, 1);

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
        Rule::in_calendar(CalendarSystem::Vietnamese, 1, 1),
    ),
    HolidayRule::fixed_public(
        "Tết",
        "Tết Nguyên Đán",
        Rule::in_calendar(CalendarSystem::Vietnamese, 1, 2),
    ),
    HolidayRule::fixed_public(
        "Tết",
        "Tết Nguyên Đán",
        Rule::in_calendar(CalendarSystem::Vietnamese, 1, 3),
    ),
    HolidayRule::fixed_public(
        "Tết",
        "Tết Nguyên Đán",
        Rule::in_calendar(CalendarSystem::Vietnamese, 1, 4),
    ),
    HolidayRule::public(
        "Hùng Kings' Festival",
        "Giỗ Tổ Hùng Vương",
        Rule::in_calendar(CalendarSystem::Vietnamese, 3, 10),
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
        Rule::in_calendar(CalendarSystem::Chinese, 1, 1),
    )
    .years(Some(2003), None),
    HolidayRule::fixed_public(
        "Isra and Mi'raj",
        "Isra Mikraj",
        Rule::in_calendar(CalendarSystem::IslamicCivil, 7, 27),
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
        Rule::in_calendar(CalendarSystem::Chinese, 4, 15),
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
        Rule::in_calendar(CalendarSystem::IslamicCivil, 10, 2),
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
        Rule::in_calendar(CalendarSystem::Chinese, 1, 1),
    ),
    HolidayRule::public(
        "Chinese New Year",
        "农历新年",
        Rule::in_calendar(CalendarSystem::Chinese, 1, 2),
    ),
    HolidayRule::public("Good Friday", "", Rule::easter(GOOD_FRIDAY)),
    HolidayRule::public("Hari Raya Puasa", "", EID_AL_FITR).approximate(),
    HolidayRule::public("Labour Day", "", Rule::gregorian(5, 1)),
    HolidayRule::public(
        "Vesak Day",
        "",
        Rule::in_calendar(CalendarSystem::Chinese, 4, 15),
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
        Rule::in_calendar(CalendarSystem::Chinese, 1, 1),
    ),
    HolidayRule::public(
        "Chinese New Year",
        "Tahun Baru Cina",
        Rule::in_calendar(CalendarSystem::Chinese, 1, 2),
    ),
    HolidayRule::public("Hari Raya Aidilfitri", "", EID_AL_FITR).approximate(),
    HolidayRule::public(
        "Hari Raya Aidilfitri",
        "",
        Rule::in_calendar(CalendarSystem::IslamicCivil, 10, 2),
    )
    .approximate(),
    HolidayRule::public("Labour Day", "Hari Pekerja", Rule::gregorian(5, 1)),
    HolidayRule::public(
        "Wesak Day",
        "Hari Wesak",
        Rule::in_calendar(CalendarSystem::Chinese, 4, 15),
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
        Rule::in_calendar(CalendarSystem::Chinese, 1, 1),
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
