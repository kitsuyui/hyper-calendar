//! China — the days its statute gives to some citizens only, and the
//! minority festivals its autonomous regions set.
//!
//! The regime is written up in `docs/systems/china-holiday-arrangements.md`
//! in the repository, in its section on the scoped days. The table itself,
//! [`CHINA`](super::CHINA), is in `asia.rs`; this file holds the rules of
//! it that are scoped, and the table takes them in through
//! [`CN_ALL_RULES`].
//!
//! # Article 3: the days for some citizens
//!
//! 全国年节及纪念日放假办法 Article 3 lists 部分公民放假的节日及纪念日:
//!
//! | Day | Who | What |
//! | --- | --- | --- |
//! | 妇女节, 8 March | 妇女 | 放假半天, half a day |
//! | 青年节, 4 May | 14周岁以上的青年 | 放假半天, half a day |
//! | 儿童节, 1 June | 不满14周岁的少年儿童 | 放假1天, the whole day |
//! | 中国人民解放军建军纪念日, 1 August | 现役军人 | 放假半天, half a day |
//!
//! Each is a rule of [`CHINA`](super::CHINA) given to its [`Group`] alone,
//! so that the nationwide calendar for everyone does not have it: the half
//! days are [`Kind::HalfDay`] and the day for children
//! [`Kind::Public`](crate::rule::Kind::Public). Article 6 makes up a day
//! for everyone that falls on a weekend, and says a day for some citizens
//! is not made up (部分公民放假的假日，如果适逢周六、周日，则不补假); the
//! table has no substitution rule in any case. The years are the
//! statutory days' own, from 1999, and every year before is a gap: the text of Article 3 is the one the
//! 2007 decision reprints and the 2024 consolidated text keeps, no
//! decision read changed it, and the 1949 and 1999 texts were not read.
//!
//! # Article 4: the minority festivals of the autonomous regions
//!
//! Article 4 leaves 少数民族习惯的节日 to the local governments of the
//! areas where each minority lives, 按照各该民族习惯. What the regions'
//! instruments read give, each a rule of [`CHINA`](super::CHINA) scoped to
//! the region's ISO 3166-2 code:
//!
//! | Region | Instrument | Days | Carried |
//! | --- | --- | --- | --- |
//! | 广西, `CN-GX` | 广西壮族自治区少数民族习惯节日放假办法, 令第98号, in force 1 March 2014: 壮族三月三, two days for every citizen of the region, the dates announced each year | the notices' days | 2024 and 2026; 2014–2023 and 2025 gaps |
//! | 新疆, `CN-XJ` | 新疆维吾尔自治区少数民族习惯节日放假办法 as amended by 令第174号, in force 1 January 2012: 肉孜节 one day and 古尔邦节 three for the cadres and workers of every nationality | the 自治区人民政府办公厅's notices | 2023–2026; every year before a gap, the 办法 of 1999 not read |
//! | 宁夏, `CN-NX` | no standing instrument found; the 自治区人民政府办公厅's notices, 宁政办发 | 开斋节 and 古尔邦节 as each notice gives them | 2023–2026; every year before a gap |
//!
//! Every year after the last notice read is a gap. The days are the
//! notices' own, the weekend days inside a span of days off among them;
//! the Hijri dates they fall on were each year's announcement, and no rule
//! predicts them. Tibet's 藏历新年 and 雪顿节 are not carried: the notices
//! read are Lhasa's and Nyingchi's, prefecture-level cities, for 驻市各单位,
//! ISO 3166-2 codes no prefecture, and no instrument of the region was
//! reached. Inner Mongolia's notice for 2024 lists the national days alone,
//! and no festival of its own was found.
//!
//! # Article 5: the commemorations without a day off
//!
//! Article 5 names days that 均不放假, give no one a day off:
//! "二七纪念日、五卅纪念日、七七抗战纪念日、九三抗战胜利纪念日、九一八纪念日、
//! 教师节、护士节、记者节、植树节等其他节日、纪念日". Each is a
//! [`Kind::Observance`] of [`CHINA`](super::CHINA), which business-day
//! arithmetic ignores:
//!
//! | Day | Date | From |
//! | --- | --- | --- |
//! | 二七纪念日, 五卅纪念日, 七七抗战纪念日, 九三抗战胜利纪念日, 九一八纪念日 | 7 February, 30 May, 7 July, 3 September, 18 September, the dates their names are | 2008, the first year of the text read, the 2007 decision's, which the 2013 and 2024 texts keep word for word; a gap before |
//! | 植树节 | 12 March, by the 全国人大常委会关于植树节的决议 of 23 February 1979 | 1979 |
//! | 教师节 | 10 September, by the 全国人大常委会关于教师节的决定 of 21 January 1985 | 1985 |
//! | 记者节 | 8 November, the day the State Council approved in 2000 at the 中国记协's request | 2000 |
//! | 护士节 | 12 May, "5月12日是国际护士节", as the National Health Commission's notices put it | 2008, as the commemorations; a gap before, no founding instrument read |
//!
//! The "其他" days the article leaves unnamed are not carried.
//!
//! [`Group`]: crate::group::Group

use crate::group::{CHILDREN, MILITARY, WOMEN, YOUTH};
use crate::rule::{HolidayRule, Kind, Listing, Rule, joined};

use super::asia::{CN_RULES, CN_STATUTE_FIRST};

/// A day Article 3 gives to one group of citizens.
const fn cn_group_day(
    name: &'static str,
    local_name: &'static str,
    month: u8,
    day: u8,
    kind: Kind,
    groups: &'static [crate::group::Group],
    item: &'static str,
) -> HolidayRule {
    HolidayRule::fixed_public(name, local_name, Rule::gregorian(month, day))
        .of_kind(kind)
        .for_groups(groups)
        .years(Some(CN_STATUTE_FIRST), None)
        .cited(item)
}

/// A day of Article 3 before 1999: a gap.
const fn cn_group_unread(
    name: &'static str,
    local_name: &'static str,
    groups: &'static [crate::group::Group],
) -> HolidayRule {
    HolidayRule::fixed_public(name, local_name, Rule::UNREAD)
        .for_groups(groups)
        .years(None, Some(CN_STATUTE_FIRST - 1))
        .cited("全国年节及纪念日放假办法, 第三条: the texts before 1999 not read")
}

/// The days of Article 3, in its order.
pub(super) static CN_SCOPED_RULES: &[HolidayRule] = &[
    cn_group_day(
        "Women's Day",
        "妇女节",
        3,
        8,
        Kind::HalfDay,
        &[WOMEN],
        "全国年节及纪念日放假办法, 第三条 (一): 妇女放假半天",
    ),
    cn_group_day(
        "Youth Day",
        "青年节",
        5,
        4,
        Kind::HalfDay,
        &[YOUTH],
        "全国年节及纪念日放假办法, 第三条 (二): 14周岁以上的青年放假半天",
    ),
    cn_group_day(
        "Children's Day",
        "儿童节",
        6,
        1,
        Kind::Public,
        &[CHILDREN],
        "全国年节及纪念日放假办法, 第三条 (三): 不满14周岁的少年儿童放假1天",
    ),
    cn_group_day(
        "Army Day",
        "中国人民解放军建军纪念日",
        8,
        1,
        Kind::HalfDay,
        &[MILITARY],
        "全国年节及纪念日放假办法, 第三条 (四): 现役军人放假半天",
    ),
    // The 1949 text and the 1999 revision were not read: whether the four
    // days were given before 1999, and how, is a gap.
    cn_group_unread("Women's Day", "妇女节", &[WOMEN]),
    cn_group_unread("Youth Day", "青年节", &[YOUTH]),
    cn_group_unread("Children's Day", "儿童节", &[CHILDREN]),
    cn_group_unread("Army Day", "中国人民解放军建军纪念日", &[MILITARY]),
];

/// The first year of Article 5's text read: the 2007 decision's, in force
/// 1 January 2008.
const CN_ARTICLE_5_FIRST: i32 = 2008;

/// A commemoration of Article 5 on a fixed date, from `first`.
const fn cn_commemoration(
    name: &'static str,
    local_name: &'static str,
    month: u8,
    day: u8,
    first: i32,
    source: &'static str,
) -> HolidayRule {
    HolidayRule::observance(name, local_name, Rule::gregorian(month, day))
        .years(Some(first), None)
        .cited(source)
}

/// A commemoration of Article 5 before 2008: a gap.
const fn cn_commemoration_unread(name: &'static str, local_name: &'static str) -> HolidayRule {
    HolidayRule::observance(name, local_name, Rule::UNREAD)
        .years(None, Some(CN_ARTICLE_5_FIRST - 1))
        .cited("全国年节及纪念日放假办法, 第五条: the texts before 2008 not read")
}

/// Article 5's words, cited by each of its days.
const CN_ARTICLE_5: &str = "全国年节及纪念日放假办法, 第五条: 均不放假";

/// The commemorations of Article 5, in its order.
pub(super) static CN_COMMEMORATIONS: &[HolidayRule] = &[
    cn_commemoration(
        "February 7th Memorial Day",
        "二七纪念日",
        2,
        7,
        CN_ARTICLE_5_FIRST,
        CN_ARTICLE_5,
    ),
    cn_commemoration(
        "May 30th Memorial Day",
        "五卅纪念日",
        5,
        30,
        CN_ARTICLE_5_FIRST,
        CN_ARTICLE_5,
    ),
    cn_commemoration(
        "July 7th Memorial Day of the War of Resistance",
        "七七抗战纪念日",
        7,
        7,
        CN_ARTICLE_5_FIRST,
        CN_ARTICLE_5,
    ),
    cn_commemoration(
        "Victory Day of the War of Resistance",
        "九三抗战胜利纪念日",
        9,
        3,
        CN_ARTICLE_5_FIRST,
        CN_ARTICLE_5,
    ),
    cn_commemoration(
        "September 18th Memorial Day",
        "九一八纪念日",
        9,
        18,
        CN_ARTICLE_5_FIRST,
        CN_ARTICLE_5,
    ),
    cn_commemoration(
        "Teachers' Day",
        "教师节",
        9,
        10,
        1985,
        "全国人民代表大会常务委员会关于教师节的决定, 21 January 1985: 每年9月10日为教师节",
    ),
    cn_commemoration(
        "Nurses' Day",
        "护士节",
        5,
        12,
        CN_ARTICLE_5_FIRST,
        CN_ARTICLE_5,
    ),
    cn_commemoration(
        "Journalists' Day",
        "记者节",
        11,
        8,
        2000,
        "国务院, 批复中国记协《关于确定\"记者节\"具体日期的请示》, 2000: 11月8日",
    ),
    cn_commemoration(
        "Arbor Day",
        "植树节",
        3,
        12,
        1979,
        "全国人民代表大会常务委员会关于植树节的决议, 23 February 1979: 3月12日",
    ),
    cn_commemoration_unread("February 7th Memorial Day", "二七纪念日"),
    cn_commemoration_unread("May 30th Memorial Day", "五卅纪念日"),
    cn_commemoration_unread(
        "July 7th Memorial Day of the War of Resistance",
        "七七抗战纪念日",
    ),
    cn_commemoration_unread("Victory Day of the War of Resistance", "九三抗战胜利纪念日"),
    cn_commemoration_unread("September 18th Memorial Day", "九一八纪念日"),
    cn_commemoration_unread("Nurses' Day", "护士节"),
];

/// The days off the autonomous regions' notices give for their minority
/// festivals, by region and festival.
static CN_REGION_DAYS: Listing = Listing::Named(&[
    // 广西: the 自治区人民政府办公厅's notices, as reposted (2024) and as the
    // press reported the notice (2026, with its 补休 on the Friday and the
    // Monday around a festival on the weekend).
    (2024, 4, 11, "gx-sanyuesan"),
    (2024, 4, 12, "gx-sanyuesan"),
    (2026, 4, 17, "gx-sanyuesan"),
    (2026, 4, 18, "gx-sanyuesan"),
    (2026, 4, 19, "gx-sanyuesan"),
    (2026, 4, 20, "gx-sanyuesan"),
    // 新疆: 肉孜节. In 2023 the festival fell on Saturday 22 April and the
    // day off was moved to the 21st (放假调休1天).
    (2023, 4, 21, "xj-rozi"),
    (2024, 4, 10, "xj-rozi"),
    (2025, 3, 29, "xj-rozi"),
    (2025, 3, 30, "xj-rozi"),
    (2025, 3, 31, "xj-rozi"),
    (2026, 3, 20, "xj-rozi"),
    (2026, 3, 21, "xj-rozi"),
    (2026, 3, 22, "xj-rozi"),
    // 新疆: 古尔邦节.
    (2023, 6, 28, "xj-kurban"),
    (2023, 6, 29, "xj-kurban"),
    (2023, 6, 30, "xj-kurban"),
    (2024, 6, 17, "xj-kurban"),
    (2024, 6, 18, "xj-kurban"),
    (2024, 6, 19, "xj-kurban"),
    (2025, 6, 6, "xj-kurban"),
    (2025, 6, 7, "xj-kurban"),
    (2025, 6, 8, "xj-kurban"),
    (2025, 6, 9, "xj-kurban"),
    (2025, 6, 10, "xj-kurban"),
    (2026, 5, 27, "xj-kurban"),
    (2026, 5, 28, "xj-kurban"),
    (2026, 5, 29, "xj-kurban"),
    (2026, 5, 30, "xj-kurban"),
    (2026, 5, 31, "xj-kurban"),
    // 宁夏: 开斋节. On a Saturday in 2023 and 2026 the notice gave no day
    // (周末正常休息).
    (2024, 4, 10, "nx-eid-al-fitr"),
    (2024, 4, 11, "nx-eid-al-fitr"),
    (2025, 3, 31, "nx-eid-al-fitr"),
    (2025, 4, 1, "nx-eid-al-fitr"),
    // 宁夏: 古尔邦节.
    (2023, 6, 29, "nx-eid-al-adha"),
    (2023, 6, 30, "nx-eid-al-adha"),
    (2024, 6, 17, "nx-eid-al-adha"),
    (2024, 6, 18, "nx-eid-al-adha"),
    (2025, 6, 6, "nx-eid-al-adha"),
    (2026, 5, 27, "nx-eid-al-adha"),
    (2026, 5, 28, "nx-eid-al-adha"),
]);

/// A festival of one region, as its notices from `first` to `last` give
/// it: a gap in any other year from `established`, the year the instrument
/// that set the day came into force, and absent before it. `None` where no
/// instrument read gives that year, so that every earlier year is a gap.
const fn cn_region_day(
    name: &'static str,
    local_name: &'static str,
    key: &'static str,
    (first, last): (i64, i64),
    established: Option<i32>,
    regions: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::fixed_public(
        name,
        local_name,
        Rule::listed(CN_REGION_DAYS.named(key), first, last),
    )
    .years(established, None)
    .in_regions(regions)
    .cited(source)
}

/// A year whose notice was not read, of a region whose standing instrument
/// was: a gap.
const fn cn_region_unread(
    name: &'static str,
    local_name: &'static str,
    first: i32,
    last: i32,
    regions: &'static [&'static str],
    source: &'static str,
) -> HolidayRule {
    HolidayRule::fixed_public(name, local_name, Rule::UNREAD)
        .years(Some(first), Some(last))
        .in_regions(regions)
        .cited(source)
}

/// 广西's instrument.
const GX_MEASURES: &str = "广西壮族自治区少数民族习惯节日放假办法 (广西壮族自治区人民政府令第98号, 2014), 第二条: \
                           壮族三月三, 本自治区内全体公民放假2天";
/// 新疆's instrument.
const XJ_MEASURES: &str = "新疆维吾尔自治区少数民族习惯节日放假办法, as amended by 新疆维吾尔自治区人民政府令第174号 \
                           (2011): 肉孜节放假一天, 古尔邦节放假三天, for 全区各族干部职工";
/// 宁夏's notices.
const NX_NOTICES: &str = "宁夏回族自治区人民政府办公厅, the notices of the year's holidays, 宁政办发〔2022〕73号, \
                          〔2023〕51号, 〔2024〕54号 and 〔2025〕36号";

/// The minority festivals of the autonomous regions.
pub(super) static CN_REGION_RULES: &[HolidayRule] = &[
    // 令第98号 set the two days from 2014; the notices of 2014 to 2023 and
    // 2025 were not read.
    cn_region_day(
        "Sanyuesan",
        "壮族三月三",
        "gx-sanyuesan",
        (2024, 2026),
        Some(2014),
        &["CN-GX"],
        GX_MEASURES,
    ),
    cn_region_unread(
        "Sanyuesan",
        "壮族三月三",
        2025,
        2025,
        &["CN-GX"],
        GX_MEASURES,
    ),
    // The 办法 of 1999 and the years before it were not read: every year
    // before 2023 is a gap.
    cn_region_day(
        "Eid al-Fitr",
        "肉孜节",
        "xj-rozi",
        (2023, 2026),
        None,
        &["CN-XJ"],
        XJ_MEASURES,
    ),
    cn_region_day(
        "Eid al-Adha",
        "古尔邦节",
        "xj-kurban",
        (2023, 2026),
        None,
        &["CN-XJ"],
        XJ_MEASURES,
    ),
    // No standing instrument was found, and the notices before 2023 were
    // not read.
    cn_region_day(
        "Eid al-Fitr",
        "开斋节",
        "nx-eid-al-fitr",
        (2023, 2026),
        None,
        &["CN-NX"],
        NX_NOTICES,
    ),
    cn_region_day(
        "Eid al-Adha",
        "古尔邦节",
        "nx-eid-al-adha",
        (2023, 2026),
        None,
        &["CN-NX"],
        NX_NOTICES,
    ),
];

/// How many rules [`CHINA`](super::CHINA) has in all.
const CN_ALL_LEN: usize =
    CN_RULES.len() + CN_SCOPED_RULES.len() + CN_REGION_RULES.len() + CN_COMMEMORATIONS.len();

/// Every rule of [`CHINA`](super::CHINA): the nationwide ones of `asia.rs`,
/// then the scoped ones here.
pub(super) static CN_ALL_RULES: [HolidayRule; CN_ALL_LEN] = joined(&[
    CN_RULES,
    CN_SCOPED_RULES,
    CN_REGION_RULES,
    CN_COMMEMORATIONS,
]);
