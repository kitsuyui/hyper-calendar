//! The era names of the Ming and Qing dynasties, and the Qing eras as a
//! calendar.
//!
//! A Chinese date was written by reign era and year — 康熙二十三年 — over
//! the lunisolar calendar's month and day, and from the Ming on a reign had,
//! with few exceptions, one era, which began with the New Year after the
//! accession: *踰年改元*. That convention is what makes an era calendar
//! simple here where [`crate::japanese`] needs two readings: every Qing era
//! begins on 1 Zhēngyuè of its first year, so the era of a day is the era
//! of its lunisolar year, and the year within the era is a subtraction.
//!
//! This module is two things:
//!
//! * **Data**: [`ALL`], the eras of the Ming (1368–1644), the Southern
//!   Ming, the Shun, and the Later Jin and Qing (1616–1911), each with the
//!   years its year 1 and its last year fell in, and [`era_of_year`], the
//!   backdated reading of a year — 1402 is 洪武 35, the restoration having
//!   been backdated over 建文 4, and 1620 is 泰昌 元年 over 萬曆 48. The
//!   Qing era that was proclaimed and never kept, 祺祥, is in the table and
//!   marked not in use.
//! * **A calendar**, `chinese-regnal`: the Qing eras day by day, from
//!   1 January 1645 — where the Shíxiàn calendar of `chinese` begins — to
//!   宣統 3年 12月 25日, 12 February 1912, the abdication. The Ming eras are
//!   not a calendar here, because the Datong calendar the Ming kept is not
//!   in this workspace and a day-level conversion under the wrong calendar
//!   would be a lie; they are year data.
//!
//! # What is not carried
//!
//! The eras before the Ming, in their hundreds, with 改元 in mid-year and
//! several regimes at once; the continued use of 宣統 inside the Forbidden
//! City after 1912 and its restoration for twelve days in 1917; the 洪憲
//! of 1916 and the eras of Manchukuo, 大同 and 康德, which ran on the
//! Gregorian calendar and belong to no dynasty in [`Dynasty`]; and 保慶,
//! which rumour in 1899–1900 gave as the era of a planned successor and
//! which was never proclaimed (`zhwiki-baoqing`). Each is a table waiting
//! on a source that dates it, not a gap in the shape.
//!
//! The era system — 踰年改元 and the mid-year exceptions, restored and
//! withdrawn eras, the concurrent regimes of 1644–1683 — is written up with
//! a worked example in
//! [`docs/systems/east-asian-eras.md`](https://github.com/kitsuyui/hyper-calendar/blob/main/docs/systems/east-asian-eras.md).
//!
//! Sources, as keyed in `docs/references.bib`: `wikipedia-ja-chinese-era-list`
//! (元号一覧 (中国), retrieved 2026-09-22) for the years of every era and the
//! months of the mid-year changes; `wikipedia-ja-xuantong` (宣統, retrieved
//! 2026-09-22) for 宣統 3年 12月 25日 as 12 February 1912 and 11月 13日 as
//! 1 January 1912; `wikipedia-en-chinese-era-names` ("List of Chinese era
//! names", retrieved 2026-09-22) for the Hanzi and pinyin;
//! `zhwiki-hongguang` and `zhwiki-chongde` (read 2026-09-26) for the
//! beginnings of 弘光 and 崇德. The primary chronologies these pages rest on,
//! the 實錄 of each reign and 方詩銘『中國歷史紀年表』, were not read.

use core::fmt;

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};
use hc_calendars_lunar::chinese;
use hc_calendars_solar::gregorian;

/// How far the Chinese calendar's continuous year count runs ahead of the
/// Common Era year a lunisolar year begins in: the year that began on
/// 10 February 2024 is 4661.
pub const YEAR_OFFSET: i64 = 2_637;

/// The dynasty an era belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dynasty {
    /// The Ming, 1368–1644.
    Ming,
    /// The Southern Ming, 1644–1683.
    SouthernMing,
    /// The Shun of Li Zicheng, 1644–1645.
    Shun,
    /// The Later Jin and the Qing, 1616–1911.
    Qing,
}

/// An era.
#[derive(Debug, Clone, Copy)]
pub struct ChineseEra {
    /// The machine identifier: the pinyin in lower case, with a restored
    /// era suffixed by its year.
    pub id: &'static str,
    /// The name in traditional characters.
    pub hanzi: &'static str,
    /// The name in pinyin with an initial capital.
    pub pinyin: &'static str,
    /// The dynasty.
    pub dynasty: Dynasty,
    /// The Common Era year in which the lunisolar year that was this era's
    /// year 1 began.
    pub start_year: i64,
    /// The Common Era year in which the era's last lunisolar year began.
    pub end_year: i64,
    /// The month of the year in which the era was proclaimed, where the
    /// source gives one because the change fell in mid-year.
    pub proclaimed_month: Option<u8>,
    /// Whether the era was ever kept: false for one proclaimed and
    /// withdrawn.
    pub in_use: bool,
    /// What the source says beyond the years.
    pub note: &'static str,
}

impl PartialEq for ChineseEra {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ChineseEra {}

const fn era(
    id: &'static str,
    hanzi: &'static str,
    pinyin: &'static str,
    dynasty: Dynasty,
    start_year: i64,
    end_year: i64,
) -> ChineseEra {
    ChineseEra {
        id,
        hanzi,
        pinyin,
        dynasty,
        start_year,
        end_year,
        proclaimed_month: None,
        in_use: true,
        note: "",
    }
}

const fn noted(
    mut era: ChineseEra,
    proclaimed_month: Option<u8>,
    in_use: bool,
    note: &'static str,
) -> ChineseEra {
    era.proclaimed_month = proclaimed_month;
    era.in_use = in_use;
    era.note = note;
    era
}

/// Every era, dynasty by dynasty, in the order of the source.
pub static ALL: [ChineseEra; 37] = [
    era("hongwu", "洪武", "Hongwu", Dynasty::Ming, 1368, 1398),
    era("jianwen", "建文", "Jianwen", Dynasty::Ming, 1399, 1402),
    noted(
        era("hongwu-1402", "洪武", "Hongwu", Dynasty::Ming, 1402, 1402),
        None,
        true,
        "restored by the Yongle Emperor for the rest of 1402, which is then 洪武 35",
    ),
    era("yongle", "永樂", "Yongle", Dynasty::Ming, 1403, 1424),
    era("hongxi", "洪熙", "Hongxi", Dynasty::Ming, 1425, 1425),
    era("xuande", "宣德", "Xuande", Dynasty::Ming, 1426, 1435),
    era("zhengtong", "正統", "Zhengtong", Dynasty::Ming, 1436, 1449),
    noted(
        era("jingtai", "景泰", "Jingtai", Dynasty::Ming, 1450, 1457),
        None,
        true,
        "to the first month of 1457",
    ),
    noted(
        era("tianshun", "天順", "Tianshun", Dynasty::Ming, 1457, 1464),
        Some(1),
        true,
        "from the first month of 1457",
    ),
    era("chenghua", "成化", "Chenghua", Dynasty::Ming, 1465, 1487),
    era("hongzhi", "弘治", "Hongzhi", Dynasty::Ming, 1488, 1505),
    era("zhengde", "正德", "Zhengde", Dynasty::Ming, 1506, 1521),
    era("jiajing", "嘉靖", "Jiajing", Dynasty::Ming, 1522, 1566),
    era("longqing", "隆慶", "Longqing", Dynasty::Ming, 1567, 1572),
    noted(
        era("wanli", "萬曆", "Wanli", Dynasty::Ming, 1573, 1620),
        None,
        true,
        "to the seventh month of 1620",
    ),
    noted(
        era("taichang", "泰昌", "Taichang", Dynasty::Ming, 1620, 1620),
        Some(8),
        true,
        "the eighth to the twelfth month of 1620",
    ),
    era("tianqi", "天啟", "Tianqi", Dynasty::Ming, 1621, 1627),
    era("chongzhen", "崇禎", "Chongzhen", Dynasty::Ming, 1628, 1644),
    noted(
        era(
            "hongguang",
            "弘光",
            "Hongguang",
            Dynasty::SouthernMing,
            1645,
            1645,
        ),
        None,
        true,
        "fixed in the fifth month of 1644, when the Hongguang Emperor took the throne in Nanjing, for the next year, 踰年改元; to the sixth month of 1645",
    ),
    noted(
        era(
            "longwu",
            "隆武",
            "Longwu",
            Dynasty::SouthernMing,
            1645,
            1646,
        ),
        Some(8),
        true,
        "22 August 1645 to the twelfth month of 1646",
    ),
    era(
        "shaowu",
        "紹武",
        "Shaowu",
        Dynasty::SouthernMing,
        1647,
        1647,
    ),
    era(
        "yongli",
        "永曆",
        "Yongli",
        Dynasty::SouthernMing,
        1647,
        1683,
    ),
    era("yongchang", "永昌", "Yongchang", Dynasty::Shun, 1644, 1645),
    era("tianming", "天命", "Tianming", Dynasty::Qing, 1616, 1626),
    noted(
        era("tiancong", "天聰", "Tiancong", Dynasty::Qing, 1627, 1636),
        None,
        true,
        "to the fourth month of 1636",
    ),
    noted(
        era("chongde", "崇德", "Chongde", Dynasty::Qing, 1636, 1643),
        Some(4),
        true,
        "from 天聰 10年 4月 11日, 15 May 1636, when the state was renamed Qing",
    ),
    era("shunzhi", "順治", "Shunzhi", Dynasty::Qing, 1644, 1661),
    era("kangxi", "康熙", "Kangxi", Dynasty::Qing, 1662, 1722),
    era("yongzheng", "雍正", "Yongzheng", Dynasty::Qing, 1723, 1735),
    era("qianlong", "乾隆", "Qianlong", Dynasty::Qing, 1736, 1795),
    era("jiaqing", "嘉慶", "Jiaqing", Dynasty::Qing, 1796, 1820),
    era("daoguang", "道光", "Daoguang", Dynasty::Qing, 1821, 1850),
    era("xianfeng", "咸豐", "Xianfeng", Dynasty::Qing, 1851, 1861),
    noted(
        era("qixiang", "祺祥", "Qixiang", Dynasty::Qing, 1861, 1861),
        None,
        false,
        "proclaimed in 1861 and never kept",
    ),
    era("tongzhi", "同治", "Tongzhi", Dynasty::Qing, 1862, 1874),
    era("guangxu", "光緒", "Guangxu", Dynasty::Qing, 1875, 1908),
    noted(
        era("xuantong", "宣統", "Xuantong", Dynasty::Qing, 1909, 1911),
        None,
        true,
        "to the abdication on 宣統 3年 12月 25日, 12 February 1912; kept on inside the Forbidden City and restored for twelve days in 1917, neither carried",
    ),
];

/// The eras of one dynasty, in order.
pub fn of_dynasty(dynasty: Dynasty) -> impl Iterator<Item = &'static ChineseEra> {
    ALL.iter().filter(move |era| era.dynasty == dynasty)
}

/// The era with this identifier or name.
#[must_use]
pub fn find(name: &str) -> Option<&'static ChineseEra> {
    ALL.iter()
        .find(|era| era.id == name || era.hanzi == name || era.pinyin == name)
}

/// The era a lunisolar year belongs to in the backdated reading, the one
/// the dynasty's own records use: the last era in the table, among those
/// kept, whose span contains the year. 1402 is 洪武, 1457 天順, 1620 泰昌,
/// 1636 崇德 and 1647 永曆.
#[must_use]
pub fn era_of_year(dynasty: Dynasty, year: i64) -> Option<&'static ChineseEra> {
    of_dynasty(dynasty)
        .filter(|era| era.in_use && era.start_year <= year && year <= era.end_year)
        .last()
}

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "The Qing eras over the Shíxiàn calendar from 1 January 1645 to 宣統 3年 12月 25日, 12 February \
    1912, the abdication [wikipedia-ja-xuantong, wikipedia-ja-chinese-era-list]; the Qing eras \
    from 1616 are year data only";

/// The earliest day the calendar converts: 1 January 1645, where the
/// Shíxiàn calendar begins.
pub const EARLIEST: Rd = chinese::EARLIEST;

/// The latest day the calendar converts: 宣統 3年 12月 25日, 12 February
/// 1912, the abdication.
pub const LATEST: Rd = match gregorian::to_fixed(1912, 2, 12) {
    Ok(rd) => rd,
    Err(_) => Rd(0),
};

/// A date in a Qing era.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChineseRegnalDate {
    /// The era.
    pub era: &'static ChineseEra,
    /// The year of the era, counting from 1.
    pub year: i64,
    /// The lunisolar month, with its intercalary flag.
    pub month: Month,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl ChineseRegnalDate {
    /// The Common Era year in which this date's lunisolar year began.
    #[must_use]
    pub const fn common_era_year(&self) -> i64 {
        self.era.start_year + self.year - 1
    }
}

impl fmt::Display for ChineseRegnalDate {
    /// Writes the date as it was written: `康熙元年正月初一` is rendered
    /// `康熙元年1月1日`, and a leap month with 閏.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.era.hanzi)?;
        if self.year == 1 {
            write!(f, "元年")?;
        } else {
            write!(f, "{}年", self.year)?;
        }
        if self.month.leap {
            write!(f, "閏")?;
        }
        write!(f, "{}月{}日", self.month.ordinal, self.day)
    }
}

/// The Qing eras over the Chinese lunisolar calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ChineseRegnalCalendar;

/// The date of a fixed day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] before 1645 and
/// [`CalendarError::AfterSupportedRange`] after the abdication.
pub fn from_fixed(rd: Rd) -> CalendarResult<ChineseRegnalDate> {
    if rd > LATEST {
        return Err(CalendarError::AfterSupportedRange);
    }
    let (number, month, day) = chinese::PARAMETERS.from_fixed(rd)?;
    let common_era_year = number - YEAR_OFFSET;
    let era = era_of_year(Dynasty::Qing, common_era_year).ok_or(CalendarError::UnknownEra)?;
    Ok(ChineseRegnalDate {
        era,
        year: common_era_year - era.start_year + 1,
        month,
        day,
    })
}

/// The fixed day of a date.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] when the era had no such
/// year, the lunisolar errors for a month or day the year did not have,
/// and [`CalendarError::AfterSupportedRange`] for a day after the
/// abdication.
pub fn to_fixed(date: ChineseRegnalDate) -> CalendarResult<Rd> {
    if date.year < 1 || date.common_era_year() > date.era.end_year {
        return Err(CalendarError::YearOutOfRange);
    }
    let rd =
        chinese::PARAMETERS.to_fixed(date.common_era_year() + YEAR_OFFSET, date.month, date.day)?;
    if rd > LATEST {
        return Err(CalendarError::AfterSupportedRange);
    }
    Ok(rd)
}

impl Calendar for ChineseRegnalCalendar {
    type Date = ChineseRegnalDate;

    /// Day by day from 1645 to the abdication of 12 February 1912, which is
    /// also the whole of the range it converts.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::between(EARLIEST, LATEST, USAGE_SOURCE)
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::LUNISOLAR_TWELVE
    }

    /// For the Common Era year a lunisolar year began in, as
    /// [`Self::from_fields`] reads one with no era: whether that year had
    /// a leap month, within the years the Qing eras span.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if era_of_year(Dynasty::Qing, year).is_none() {
            return Err(CalendarError::YearOutOfRange);
        }
        chinese::PARAMETERS.is_leap_year(year + YEAR_OFFSET)
    }

    /// Resolves the era first: the fields count years within it, and the
    /// leap rule counts Common Era years.
    fn is_leap_year_of(&self, fields: &DateFields) -> CalendarResult<bool> {
        match fields.era {
            Some(name) => {
                let era = find(name).ok_or(CalendarError::UnknownEra)?;
                self.is_leap_year(era.start_year + fields.year - 1)
            }
            None => self.is_leap_year(fields.year),
        }
    }

    /// The era in traditional characters with its pinyin.
    fn era_name(&self, code: &str) -> Option<hc_calendar::EraName> {
        find(code).map(|era| hc_calendar::EraName::new(era.hanzi, era.pinyin))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("chinese-regnal"),
            english_name: "Qing dynasty eras",
            year_kind: YearKind::EraRelative,
            has_leap_months: true,
            is_astronomical: true,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &["zh-Hant", "zh-Hans"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        to_fixed(date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        from_fixed(rd)
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let mut fields =
            DateFields::ymd(date.year, date.month.ordinal, date.day).with_era(date.era.id);
        fields.month = Some(date.month);
        Ok(fields)
    }

    /// Reads era-tagged fields, or fields with no era as the Common Era
    /// year the lunisolar year began in.
    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        let day = fields.require_day()?;
        match fields.era {
            Some(name) => {
                let era = find(name).ok_or(CalendarError::UnknownEra)?;
                if era.dynasty != Dynasty::Qing || !era.in_use {
                    return Err(CalendarError::UnknownEra);
                }
                let date = ChineseRegnalDate {
                    era,
                    year: fields.year,
                    month,
                    day,
                };
                to_fixed(date)?;
                Ok(date)
            }
            None => {
                from_fixed(chinese::PARAMETERS.to_fixed(fields.year + YEAR_OFFSET, month, day)?)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("valid Gregorian date")
    }

    #[test]
    fn the_year_offset_is_the_one_the_lunar_crate_documents() {
        let (number, _, _) = chinese::PARAMETERS
            .from_fixed(greg(2024, 2, 10))
            .expect("in range");
        assert_eq!(number, 2024 + YEAR_OFFSET);
    }

    #[test]
    fn the_qing_eras_begin_with_the_new_year_and_end_with_the_abdication() {
        // 康熙元年 opened with the New Year of 1662, 18 February; 光緒元年
        // with that of 1875, 6 February; 1 January 1900 was 光緒 25年 12月
        // 1日; and the source's two 宣統 dates.
        let cases = [
            ((1662, 2, 18), "kangxi", 1, 1, 1),
            ((1875, 2, 6), "guangxu", 1, 1, 1),
            ((1900, 1, 1), "guangxu", 25, 12, 1),
            ((1912, 1, 1), "xuantong", 3, 11, 13),
            ((1912, 2, 12), "xuantong", 3, 12, 25),
        ];
        for ((y, m, d), id, year, month, day) in cases {
            let date = from_fixed(greg(y, m, d)).expect("in range");
            assert_eq!(
                (
                    date.era.id,
                    date.year,
                    date.month.ordinal,
                    date.day,
                    date.month.leap
                ),
                (id, year, month, day, false),
                "{y}-{m}-{d}"
            );
            assert_eq!(to_fixed(date), Ok(greg(y, m, d)));
        }
        assert_eq!(
            from_fixed(greg(1912, 2, 12)).unwrap().to_string(),
            "宣統3年12月25日"
        );
        assert_eq!(
            from_fixed(greg(1662, 2, 18)).unwrap().to_string(),
            "康熙元年1月1日"
        );
        assert_eq!(
            from_fixed(greg(1912, 2, 13)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert!(from_fixed(Rd(EARLIEST.0 - 1)).is_err());
        // The day before 康熙 was 順治 18年.
        let last_shunzhi = from_fixed(greg(1662, 2, 17)).expect("in range");
        assert_eq!((last_shunzhi.era.id, last_shunzhi.year), ("shunzhi", 18));
        // A year an era never had.
        let too_far = ChineseRegnalDate {
            era: find("kangxi").unwrap(),
            year: 62,
            month: Month::regular(1),
            day: 1,
        };
        assert_eq!(to_fixed(too_far), Err(CalendarError::YearOutOfRange));
    }

    #[test]
    fn the_backdated_reading_gives_shared_years_to_the_later_era() {
        let check = |dynasty, year, id, era_year| {
            let era = era_of_year(dynasty, year).expect("in a span");
            assert_eq!(era.id, id, "{year}");
            assert_eq!(year - era.start_year + 1, era_year, "{year}");
        };
        check(Dynasty::Ming, 1368, "hongwu", 1);
        check(Dynasty::Ming, 1402, "hongwu-1402", 1);
        check(Dynasty::Ming, 1457, "tianshun", 1);
        check(Dynasty::Ming, 1620, "taichang", 1);
        check(Dynasty::Ming, 1644, "chongzhen", 17);
        check(Dynasty::SouthernMing, 1645, "longwu", 1);
        check(Dynasty::Ming, 1644, "chongzhen", 17);
        assert_eq!(era_of_year(Dynasty::SouthernMing, 1644), None);
        check(Dynasty::SouthernMing, 1683, "yongli", 37);
        check(Dynasty::Qing, 1636, "chongde", 1);
        check(Dynasty::Qing, 1861, "xianfeng", 11);
        check(Dynasty::Qing, 1899, "guangxu", 25);
        check(Dynasty::Qing, 1795, "qianlong", 60);
        assert_eq!(era_of_year(Dynasty::Ming, 1367), None);
        assert!(!find("祺祥").unwrap().in_use);
        assert_eq!(
            ALL.iter()
                .filter(|era| era.dynasty == Dynasty::Qing && era.in_use)
                .count(),
            13
        );
    }

    #[test]
    fn every_nineteenth_day_of_the_qing_round_trips() {
        let calendar = ChineseRegnalCalendar;
        // Every nineteenth day in a release build, every 95th in a debug one.
        for rd in (EARLIEST.0..=LATEST.0).step_by(19 * crate::sweep_stride(5)) {
            let date = calendar.from_fixed(Rd(rd)).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "rd {rd}");
            let fields = calendar.to_fields(date).expect("describable");
            assert_eq!(fields.era, Some(date.era.id));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
        }
        let plain = DateFields::ymd(1700, 3, 3);
        assert_eq!(
            calendar.from_fields(&plain),
            from_fixed(
                chinese::PARAMETERS
                    .to_fixed(1700 + YEAR_OFFSET, Month::regular(3), 3)
                    .unwrap()
            )
        );
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(1, 1, 1).with_era("wanli")),
            Err(CalendarError::UnknownEra)
        );
    }
}
