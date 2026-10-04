//! The era names (*niên hiệu*) of the Nguyễn dynasty, 1802–1945, as a
//! calendar over the Vietnamese lunisolar calendar: `vietnamese-regnal-nguyen`.
//!
//! The Nguyễn court dated by reign era and year over the lunisolar month
//! and day — 嗣德三十六年六月十六日, Tự Đức 36, the 16th of the 6th month,
//! the day the emperor died — and a reign had one era. Where an emperor
//! came to the throne late in a year the era began with the next new year,
//! as the Chinese *踰年改元* has it: Tự Đức, enthroned in the tenth month
//! of 1847, "đặt niên hiệu là Tự Đức, bắt đầu từ năm sau là 1848"
//! (`wikipedia-vi-tu-duc`), and Minh Mạng, Kiến Phúc, Hàm Nghi and Bảo Đại
//! are counted the same way by every table read (`wikipedia-en-vietnamese-era-name`,
//! `wikipedia-zh-vietnamese-era-list`, `wikipedia-vi-nien-hieu`). Six eras
//! began part way through a year, on the day the emperor took the name, and
//! the Chinese list gives the month of each: 嘉隆 in the fifth month of 1802,
//! 紹治 and 成泰 in the first month of 1841 and 1889, 同慶 from the tenth
//! month of 1885, 維新 in the seventh month of 1907 and 啟定 in the fourth of
//! 1916 (`wikipedia-zh-vietnamese-era-list`). The day of each is the
//! emperor's article in the Vietnamese Wikipedia, and the engine's lunisolar
//! date of that day falls in the month the list names, every time.
//!
//! # The eras, with the day each began
//!
//! | Era | Chữ Hán | Year 1 | First day | Source for the day |
//! |---|---|---|---|---|
//! | Gia Long | 嘉隆 | 1802 | 1 June 1802, 5/2 of Nhâm Tuất | `wikipedia-vi-gia-long` ("Đăng quang: 1 tháng 6 năm Nhâm Tuất (1802)") |
//! | Minh Mạng | 明命 | 1820 | 14 February 1820, Tết | "Tháng giêng năm Canh Thìn (1820) … đặt niên hiệu là Minh Mạng" (`wikipedia-vi-minh-mang`); Gia Long died on 3 February 1820, before Tết |
//! | Thiệu Trị | 紹治 | 1841 | 11 February 1841, 1/20 of Tân Sửu | `wikipedia-vi-thieu-tri`; Minh Mạng died on 20 January 1841 |
//! | Tự Đức | 嗣德 | 1848 | 5 February 1848, Tết | enthroned in the tenth month of 1847, the era "bắt đầu từ năm sau là 1848" (`wikipedia-vi-tu-duc`) |
//! | Kiến Phúc | 建福 | 1884 | 28 January 1884, Tết | enthroned 2 December 1883 (`wikipedia-vi-kien-phuc`); year 1 is 1884 in every list |
//! | Hàm Nghi | 咸宜 | 1885 | 15 February 1885, Tết | enthroned 2 August 1884, 6/12 of Giáp Thân (`wikipedia-vi-ham-nghi`); year 1 is 1885 in every list |
//! | Đồng Khánh | 同慶 | 1885 | 7 November 1885, 10/1 of Ất Dậu | "Từ tháng 10 âm lịch trở đi gọi là năm Đồng Khánh Ất Dậu" (`wikipedia-vi-dong-khanh`); enthroned 19 September 1885 |
//! | Thành Thái | 成泰 | 1889 | 2 February 1889, 1/3 of Kỷ Sửu | `wikipedia-vi-thanh-thai`, `wikipedia-en-thanh-thai`; Đồng Khánh died on 28 January 1889, 12/27 of Mậu Tý |
//! | Duy Tân | 維新 | 1907 | 5 September 1907, 7/28 of Đinh Mùi | `wikipedia-vi-duy-tan` |
//! | Khải Định | 啟定 | 1916 | 18 May 1916, 4/17 of Bính Thìn | `wikipedia-vi-khai-dinh` |
//! | Bảo Đại | 保大 | 1926 | 13 February 1926, Tết | the name taken at the enthronement of 8 January 1926, 11/24 of Ất Sửu (`wikipedia-vi-bao-dai`, `wikipedia-en-bao-dai`); year 1 is 1926 in every list, so the remaining days of Ất Sửu stay Khải Định 10 here |
//!
//! The calendar ends on 30 August 1945, 7/23 of Ất Dậu, the afternoon Bảo
//! Đại read the abdication at the Meridian Gate (`wikipedia-en-bao-dai`,
//! `wikipedia-vi-bao-dai`). Two emperors have no era: Dục Đức reigned
//! three days in July 1883 "chưa kịp đặt niên hiệu" (`wikipedia-vi-duc-duc`),
//! and Hiệp Hòa, who took the name Hiệp Hòa on 30 July 1883 and was deposed
//! on 29 November 1883, 10/30 of Quý Mùi (`wikipedia-vi-hiep-hoa`), before
//! the year the name would have counted; the English list has the name
//! "planned" and never "put into effective use". 1883 is Tự Đức 36
//! throughout, as the lists have it, and Hiệp Hòa is in [`ALL`] marked not
//! in use.
//!
//! # Readings not carried
//!
//! * The Cần Vương movement kept dating by Hàm Nghi after the court took
//!   Đồng Khánh, to the eighth month of 1888 by the Chinese list; no
//!   document of theirs was read, so the stream is not a calendar here.
//! * Bảo Đại from 8 January 1926 rather than from Tết: both Wikipedias say
//!   the name was taken that day, and both count 1926 as year 1 and 1945 as
//!   year 20, which the day-level reading here follows; a reading that puts
//!   8 January to 12 February 1926 under Bảo Đại would need a document
//!   dated that way, and none was read.
//! * The abdication edict's own date, 25 August 1945, 7/18: the pages read
//!   give the ceremony of the 30th, and the calendar runs to it.
//! * The earlier dynasties, 544–1789: year ranges only, in the roadmap.
//!
//! # The lunisolar calendar underneath
//!
//! `hc_calendars_lunar::vietnamese`, which computes on UT+8 before 1968 —
//! the Chinese rules at the meridian the court's almanac shared — and is
//! proleptic before 1968 (no source read dates the court's adoption of the
//! Shíxiàn rules). Seven lunisolar dates the emperors' articles give beside
//! their Gregorian days all fall out of the engine as written: Thiệu Trị's
//! enthronement 1/20 of Tân Sửu (11 February 1841) and death 9/27 of Đinh
//! Mùi (4 November 1847), Tự Đức's death 6/16 of Quý Mùi (19 July 1883),
//! Hiệp Hòa's death 10/30 of Quý Mùi (29 November 1883), Kiến Phúc's death
//! 6/10 of Giáp Thân (31 July 1884), Hàm Nghi's enthronement 6/12 of Giáp
//! Thân (2 August 1884), Đồng Khánh's death 12/27 of Mậu Tý (28 January
//! 1889) and Khải Định's death 9/20 of Ất Sửu (6 November 1925); the test
//! `the_emperors_lunisolar_dates_fall_out_of_the_engine` holds them.

use core::fmt;

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};
use hc_calendars_lunar::vietnamese;
use hc_calendars_solar::gregorian;

/// The machine identifier.
pub const ID: CalendarId = CalendarId("vietnamese-regnal-nguyen");

/// A Gregorian day as a fixed day, for the table.
const fn day(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(rd) => rd,
        Err(_) => Rd(0),
    }
}

/// An era of the Nguyễn dynasty.
#[derive(Debug, Clone, Copy)]
pub struct VietnameseEra {
    /// The machine identifier: the Vietnamese name in lower-case kebab
    /// case without diacritics, `gia-long`.
    pub id: &'static str,
    /// The name in chữ Hán, as the court wrote it.
    pub han: &'static str,
    /// The name in Vietnamese, with its diacritics.
    pub name: &'static str,
    /// The Common Era year in which the lunisolar year that was this era's
    /// year 1 began.
    pub start_year: i64,
    /// The first day the era was in force.
    pub start: Rd,
    /// Whether the era was ever kept: false for Hiệp Hòa, taken and never
    /// counted.
    pub in_use: bool,
    /// What the sources say beyond the day.
    pub note: &'static str,
}

impl PartialEq for VietnameseEra {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for VietnameseEra {}

const fn era(
    id: &'static str,
    han: &'static str,
    name: &'static str,
    start_year: i64,
    start: Rd,
    note: &'static str,
) -> VietnameseEra {
    VietnameseEra {
        id,
        han,
        name,
        start_year,
        start,
        in_use: true,
        note,
    }
}

/// Every era of the dynasty in order, Hiệp Hòa included and marked not in
/// use; the sources are in the module documentation.
pub static ALL: [VietnameseEra; 12] = [
    era(
        "gia-long",
        "嘉隆",
        "Gia Long",
        1802,
        day(1802, 6, 1),
        "from 1 June 1802, 5/2 of Nhâm Tuất, the enthronement at Huế",
    ),
    era(
        "minh-mang",
        "明命",
        "Minh Mạng",
        1820,
        day(1820, 2, 14),
        "from Tết 1820, the first month of Canh Thìn; Gia Long died on 3 February 1820, which is Gia Long 18, 12/19",
    ),
    era(
        "thieu-tri",
        "紹治",
        "Thiệu Trị",
        1841,
        day(1841, 2, 11),
        "from 11 February 1841, 1/20 of Tân Sửu, the enthronement; the first nineteen days of the year are Minh Mạng 22",
    ),
    era(
        "tu-duc",
        "嗣德",
        "Tự Đức",
        1848,
        day(1848, 2, 5),
        "from Tết 1848; enthroned in the tenth month of 1847, the era from the next year",
    ),
    VietnameseEra {
        id: "hiep-hoa",
        han: "協和",
        name: "Hiệp Hòa",
        start_year: 1884,
        start: day(1883, 7, 30),
        in_use: false,
        note: "taken on 30 July 1883 and never counted: the emperor was deposed on 29 November 1883, 10/30 of Quý Mùi, before the year the name would have numbered; 1883 is Tự Đức 36 throughout",
    },
    era(
        "kien-phuc",
        "建福",
        "Kiến Phúc",
        1884,
        day(1884, 1, 28),
        "from Tết 1884; enthroned 2 December 1883, 11/3 of Quý Mùi",
    ),
    era(
        "ham-nghi",
        "咸宜",
        "Hàm Nghi",
        1885,
        day(1885, 2, 15),
        "from Tết 1885; enthroned 2 August 1884, 6/12 of Giáp Thân; kept by the Cần Vương to the eighth month of 1888, which is not carried",
    ),
    era(
        "dong-khanh",
        "同慶",
        "Đồng Khánh",
        1885,
        day(1885, 11, 7),
        "from the tenth month of Ất Dậu, 7 November 1885, so that year is Hàm Nghi 1 to its ninth month and Đồng Khánh 1 from its tenth; enthroned 19 September 1885",
    ),
    era(
        "thanh-thai",
        "成泰",
        "Thành Thái",
        1889,
        day(1889, 2, 2),
        "from 2 February 1889, 1/3 of Kỷ Sửu, the enthronement; Đồng Khánh died on 28 January 1889, 12/27 of Mậu Tý",
    ),
    era(
        "duy-tan",
        "維新",
        "Duy Tân",
        1907,
        day(1907, 9, 5),
        "from 5 September 1907, 7/28 of Đinh Mùi, the enthronement",
    ),
    era(
        "khai-dinh",
        "啟定",
        "Khải Định",
        1916,
        day(1916, 5, 18),
        "from 18 May 1916, 4/17 of Bính Thìn, the enthronement",
    ),
    era(
        "bao-dai",
        "保大",
        "Bảo Đại",
        1926,
        day(1926, 2, 13),
        "from Tết 1926; the name taken at the enthronement of 8 January 1926, 11/24 of Ất Sửu, and year 1 the lunisolar year 1926 in every list, so the rest of Ất Sửu is Khải Định 10; to the abdication, 30 August 1945, 7/23",
    ),
];

/// The era with this identifier, by [`hc_core::catalogue::matches`].
#[must_use]
pub fn by_id(id: &str) -> Option<&'static VietnameseEra> {
    ALL.iter()
        .find(|era| hc_core::catalogue::matches(id, era.id))
}

/// The era in force on a day: the last kept era whose first day is on or
/// before it, within the calendar's range.
///
/// # Errors
///
/// [`CalendarError::BeforeEpoch`] before 1 June 1802 and
/// [`CalendarError::AfterSupportedRange`] after 30 August 1945.
pub fn era_at(rd: Rd) -> CalendarResult<&'static VietnameseEra> {
    if rd > LATEST {
        return Err(CalendarError::AfterSupportedRange);
    }
    ALL.iter()
        .rfind(|era| era.in_use && era.start <= rd)
        .ok_or(CalendarError::BeforeEpoch)
}

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "The Nguyễn eras over the Vietnamese lunisolar calendar from 1 June 1802, when Nguyễn Ánh \
    took the name Gia Long at Huế [wikipedia-vi-gia-long], to 30 August 1945, when Bảo Đại read \
    the abdication [wikipedia-en-bao-dai, wikipedia-vi-bao-dai]; the years of each era from \
    the lists [wikipedia-en-vietnamese-era-name, wikipedia-zh-vietnamese-era-list], the day \
    each began from the emperor's article in the Vietnamese Wikipedia";

/// The first day: 1 June 1802.
pub const EARLIEST: Rd = day(1802, 6, 1);

/// The last day: 30 August 1945, 7/23 of Ất Dậu, the abdication.
pub const LATEST: Rd = day(1945, 8, 30);

/// A date in a Nguyễn era.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VietnameseRegnalDate {
    /// The era.
    pub era: &'static VietnameseEra,
    /// The year of the era, counting from 1.
    pub year: i64,
    /// The lunisolar month, with its intercalary flag.
    pub month: Month,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl VietnameseRegnalDate {
    /// The Common Era year in which this date's lunisolar year began,
    /// which is `vietnamese`'s year number.
    #[must_use]
    pub const fn common_era_year(&self) -> i64 {
        self.era.start_year + self.year - 1
    }
}

impl fmt::Display for VietnameseRegnalDate {
    /// Writes the date as the court wrote it, in chữ Hán: 嗣德36年6月16日,
    /// the first year as 元年 and a leap month with 閏.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.era.han)?;
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

/// The date of a fixed day.
///
/// # Errors
///
/// As [`era_at`], and the lunisolar engine's errors.
pub fn from_fixed(rd: Rd) -> CalendarResult<VietnameseRegnalDate> {
    let era = era_at(rd)?;
    let (number, month, day) = vietnamese::PARAMETERS.from_fixed(rd)?;
    Ok(VietnameseRegnalDate {
        era,
        year: number - era.start_year + 1,
        month,
        day,
    })
}

/// The fixed day of a date.
///
/// # Errors
///
/// [`CalendarError::YearOutOfRange`] when the era was not in force on that
/// day — a year below 1, a year after the era ended, or a day of the era's
/// first or last year that belongs to its neighbour — the lunisolar errors
/// for a month or day the year did not have, and
/// [`CalendarError::AfterSupportedRange`] after the abdication.
pub fn to_fixed(date: VietnameseRegnalDate) -> CalendarResult<Rd> {
    if date.year < 1 || !date.era.in_use {
        return Err(CalendarError::YearOutOfRange);
    }
    let rd = vietnamese::PARAMETERS.to_fixed(date.common_era_year(), date.month, date.day)?;
    if rd > LATEST {
        return Err(CalendarError::AfterSupportedRange);
    }
    if era_at(rd)? != date.era {
        return Err(CalendarError::YearOutOfRange);
    }
    Ok(rd)
}

/// The Nguyễn eras over the Vietnamese lunisolar calendar, 1802–1945:
/// `vietnamese-regnal-nguyen`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct VietnameseRegnalCalendar;

impl Calendar for VietnameseRegnalCalendar {
    type Date = VietnameseRegnalDate;

    /// Day by day from 1 June 1802 to 30 August 1945, which is also the
    /// whole of the range it converts.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::between(EARLIEST, LATEST, USAGE_SOURCE)
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::LUNISOLAR_TWELVE
    }

    /// For the Common Era year a lunisolar year began in, as
    /// [`Self::from_fields`] reads one with no era: whether that year had a
    /// leap month, within the years the eras span.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(ALL[0].start_year..=1945).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        vietnamese::PARAMETERS.is_leap_year(year)
    }

    /// Resolves the era first: the fields count years within it, and the
    /// leap rule counts Common Era years.
    fn is_leap_year_of(&self, fields: &DateFields) -> CalendarResult<bool> {
        match fields.era {
            Some(name) => {
                let era = by_id(name).ok_or(CalendarError::UnknownEra)?;
                self.is_leap_year(era.start_year + fields.year - 1)
            }
            None => self.is_leap_year(fields.year),
        }
    }

    /// The era in chữ Hán with its Vietnamese name as the romanisation.
    fn era_name(&self, code: &str) -> Option<hc_calendar::EraName> {
        by_id(code).map(|era| hc_calendar::EraName::new(era.han, era.name))
    }

    /// The eras of the table, in its order.
    fn era_code(&self, index: usize) -> Option<&'static str> {
        ALL.get(index).map(|era| era.id)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Nguyễn dynasty eras",
            year_kind: YearKind::EraRelative,
            has_leap_months: true,
            is_astronomical: true,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
            native_locales: &["vi"],
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
                let era = by_id(name).ok_or(CalendarError::UnknownEra)?;
                let date = VietnameseRegnalDate {
                    era,
                    year: fields.year,
                    month,
                    day,
                };
                to_fixed(date)?;
                Ok(date)
            }
            None => from_fixed(vietnamese::PARAMETERS.to_fixed(fields.year, month, day)?),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("valid Gregorian date")
    }

    fn lunisolar(rd: Rd) -> (i64, u8, bool, u8) {
        let (year, month, day) = vietnamese::PARAMETERS.from_fixed(rd).expect("in range");
        (year, month.ordinal, month.leap, day)
    }

    #[test]
    fn the_emperors_lunisolar_dates_fall_out_of_the_engine() {
        // The Vietnamese Wikipedia gives these days in both calendars; the
        // engine at UT+8 reproduces every one.
        for (gregorian_day, year, month, day) in [
            (greg(1802, 6, 1), 1802, 5, 2),
            (greg(1841, 2, 11), 1841, 1, 20),
            (greg(1847, 11, 4), 1847, 9, 27),
            (greg(1883, 7, 19), 1883, 6, 16),
            (greg(1883, 11, 29), 1883, 10, 30),
            (greg(1884, 7, 31), 1884, 6, 10),
            (greg(1884, 8, 2), 1884, 6, 12),
            (greg(1889, 1, 28), 1888, 12, 27),
            (greg(1925, 11, 6), 1925, 9, 20),
            (greg(1926, 1, 8), 1925, 11, 24),
            (greg(1945, 8, 30), 1945, 7, 23),
        ] {
            assert_eq!(lunisolar(gregorian_day), (year, month, false, day));
        }
    }

    #[test]
    fn every_mid_year_change_falls_in_the_month_the_chinese_list_names() {
        for (id, month) in [
            ("gia-long", 5),
            ("thieu-tri", 1),
            ("dong-khanh", 10),
            ("thanh-thai", 1),
            ("duy-tan", 7),
            ("khai-dinh", 4),
        ] {
            let era = by_id(id).expect("in the table");
            assert_eq!(lunisolar(era.start).1, month, "{id}");
            assert_eq!(lunisolar(era.start).0, era.start_year, "{id}");
        }
        // The year-start eras begin on Tết.
        for id in ["minh-mang", "tu-duc", "kien-phuc", "ham-nghi", "bao-dai"] {
            let era = by_id(id).expect("in the table");
            assert_eq!(
                vietnamese::PARAMETERS.new_year(era.start_year),
                Ok(era.start),
                "{id}"
            );
        }
    }

    #[test]
    fn the_lists_years_are_reproduced() {
        // The English list's spans: Gia Long 1802–1819, Minh Mạng 1820–1841,
        // Thiệu Trị 1841–1847, Tự Đức 1848–1883, Kiến Phúc 1884, Hàm Nghi
        // 1885, Đồng Khánh 1885–1889, Thành Thái 1889–1907, Duy Tân
        // 1907–1916, Khải Định 1916–1925, Bảo Đại 1926–1945.
        let date = |rd| from_fixed(rd).expect("in range");
        assert_eq!(date(greg(1819, 12, 31)).era.id, "gia-long");
        assert_eq!(date(greg(1819, 12, 31)).year, 18);
        // Gia Long died on 3 February 1820, before Tết: still Gia Long 18.
        assert_eq!(date(greg(1820, 2, 3)).era.id, "gia-long");
        assert_eq!(date(greg(1820, 2, 3)).year, 18);
        assert_eq!(date(greg(1820, 2, 14)).era.id, "minh-mang");
        assert_eq!(date(greg(1820, 2, 14)).year, 1);
        // Minh Mạng 22 for the first nineteen days of 1841.
        assert_eq!(date(greg(1841, 2, 10)).era.id, "minh-mang");
        assert_eq!(date(greg(1841, 2, 10)).year, 22);
        assert_eq!(date(greg(1841, 2, 11)).to_string(), "紹治元年1月20日");
        // Tự Đức 36 on the day he died and through Hiệp Hòa's reign.
        assert_eq!(date(greg(1883, 7, 19)).to_string(), "嗣德36年6月16日");
        assert_eq!(date(greg(1883, 11, 29)).to_string(), "嗣德36年10月30日");
        assert_eq!(date(greg(1884, 1, 27)).to_string(), "嗣德36年12月30日");
        assert_eq!(date(greg(1884, 1, 28)).to_string(), "建福元年1月1日");
        // Hàm Nghi 1 to the ninth month of 1885, Đồng Khánh 1 from the tenth.
        assert_eq!(date(greg(1885, 11, 6)).to_string(), "咸宜元年9月30日");
        assert_eq!(date(greg(1885, 11, 7)).to_string(), "同慶元年10月1日");
        assert_eq!(date(greg(1889, 2, 1)).to_string(), "同慶5年1月2日");
        assert_eq!(date(greg(1889, 2, 2)).to_string(), "成泰元年1月3日");
        assert_eq!(date(greg(1907, 9, 4)).year, 19);
        assert_eq!(date(greg(1907, 9, 5)).to_string(), "維新元年7月28日");
        assert_eq!(date(greg(1916, 5, 17)).year, 10);
        assert_eq!(date(greg(1916, 5, 18)).to_string(), "啟定元年4月17日");
        // Khải Định 10 to the end of Ất Sửu, Bảo Đại 1 from Tết 1926.
        assert_eq!(date(greg(1926, 1, 8)).to_string(), "啟定10年11月24日");
        assert_eq!(date(greg(1926, 2, 13)).to_string(), "保大元年1月1日");
        assert_eq!(date(greg(1945, 8, 30)).to_string(), "保大20年7月23日");
    }

    #[test]
    fn the_range_is_the_dynasty_s() {
        assert_eq!(
            from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(
            from_fixed(EARLIEST).map(|d| d.to_string()),
            Ok("嘉隆元年5月2日".into())
        );
        // A year the era did not reach, and a day of a shared year that
        // belongs to the neighbour.
        let khai_dinh = by_id("khai-dinh").expect("in the table");
        assert_eq!(
            to_fixed(VietnameseRegnalDate {
                era: khai_dinh,
                year: 11,
                month: Month::regular(1),
                day: 1
            }),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            to_fixed(VietnameseRegnalDate {
                era: khai_dinh,
                year: 1,
                month: Month::regular(1),
                day: 1
            }),
            Err(CalendarError::YearOutOfRange)
        );
        let hiep_hoa = by_id("hiep-hoa").expect("in the table");
        assert!(!hiep_hoa.in_use);
        assert_eq!(
            to_fixed(VietnameseRegnalDate {
                era: hiep_hoa,
                year: 1,
                month: Month::regular(1),
                day: 1
            }),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn every_day_of_the_dynasty_round_trips() {
        let calendar = VietnameseRegnalCalendar;
        let boundaries = ALL.iter().map(|era| era.start.0);
        for rd in crate::sweep_days(EARLIEST.0, LATEST.0, 13, boundaries) {
            let rd = Rd(rd);
            let date = calendar.from_fixed(rd).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(rd), "{rd}");
            let fields = calendar.to_fields(date).expect("fields");
            assert_eq!(calendar.from_fields(&fields), Ok(date), "{rd}");
            let plain = DateFields::ymd(date.common_era_year(), date.month.ordinal, date.day);
            let mut plain = plain;
            plain.month = Some(date.month);
            assert_eq!(calendar.from_fields(&plain), Ok(date), "{rd}");
        }
    }

    #[test]
    fn the_era_names_and_codes_are_served() {
        let calendar = VietnameseRegnalCalendar;
        assert_eq!(calendar.era_code(0), Some("gia-long"));
        assert_eq!(calendar.era_code(11), Some("bao-dai"));
        assert_eq!(calendar.era_code(12), None);
        let name = calendar.era_name("tu-duc").expect("named");
        assert_eq!((name.native, name.romanised), ("嗣德", "Tự Đức"));
        assert_eq!(calendar.meta().id, ID);
        assert_eq!(calendar.usage().from, Some(EARLIEST));
        assert_eq!(calendar.usage().until, Some(LATEST));
    }
}
