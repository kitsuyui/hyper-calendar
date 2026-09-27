//! The 黃帝紀元, the years of the Yellow Emperor, on the Chinese lunisolar
//! calendar — `huangdi-era`, `huangdi-era-tongmenghui`,
//! `huangdi-era-liu-shipei` and `huangdi-era-jiangsu`.
//!
//! Chinese reformers and revolutionaries of the last Qing decade counted
//! years from the Yellow Emperor instead of by the reign, and did not agree
//! on the epoch. 張新斌, 「百余年来"黄帝纪年"倡行的两次高潮及意义」
//! (`zhang-xinbin-huangdi`, read in the Internet Archive's copy of
//! 20 January 2023) gives five: 嚴復's 1898 as 4386, 劉師培's 1903 as 4614,
//! the journal 《江蘇》's 1903 as 4394, the 同盟會's 1908 as 4605 in its
//! address at the Yellow Emperor's tomb, and 宋教仁's 1905 as 4603, which
//! the revolutionaries took up and Sun Yat-sen's telegram used. Each is its
//! own identifier under policy §5 but 嚴復's, whose "开国自黄帝至今
//! 四千三百八十六年" is a count of elapsed years in one article and not a
//! way of dating.
//!
//! The years are the Chinese lunisolar calendar's, turning at 正月初一, and
//! the months and days are its own: Sun Yat-sen's telegram of 2 January
//! 1912 made "黄帝纪年四千六百九年十一月十三日" the first day of the
//! Republic (`zhang-xinbin-huangdi`), which is 1 January 1912 and the
//! 十一月十三日 of the Chinese year that began in 1911; the Hubei Military
//! Government's gazette of 31 October 1911 is dated 4609年9月10日, and
//! 劉師培 signed his essay "黄帝降生四千六百一十四年闰五月十七日", in the
//! 閏五月 of 1903 (Wikipedia (zh), 「黃帝紀元」, `wikipedia-zh-huangdi-era`,
//! whose table gives each count for 1903–1911 with the year turning at the
//! lunar new year). So each calendar here is [`hc_calendars_lunar::chinese`]
//! with the year shifted by a constant, and converts what `chinese` converts,
//! 1645 to 2150.
//!
//! The Wikipedia table's own epochs in years BC disagree with its year
//! numbers for 劉師培's count and for 《江蘇》's; the year numbers agree with
//! 張新斌, and those are followed. Two further rows of the table, from 2997
//! BC and 2999 BC, cite nothing and are not carried. 孔子紀年 is not carried:
//! the sources read give its epoch, 551 BC, but not the day its year turns.
//!
//! None of the four records a period of use: the sources date each count by
//! the documents that used it, not by a first and a last day.
//! `docs/systems/era-counts.md` in the repository works an example through.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Usage,
    YearKind,
};
use hc_calendars_lunar::chinese::{self, ChineseCalendar, ChineseDate};

/// One count of the years of the Yellow Emperor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HuangdiCount {
    /// The calendar identifier.
    pub id: &'static str,
    /// The English name of the calendar.
    pub english_name: &'static str,
    /// The era code a date carries.
    pub era: &'static str,
    /// This count's year less the Chinese calendar's continuous year, the
    /// count from 2637 BC in which the year beginning in 2024 is 4661.
    pub offset: i64,
}

impl HuangdiCount {
    /// The Chinese calendar's year of year `year` of this count.
    #[must_use]
    pub const fn chinese_year(self, year: i64) -> i64 {
        year - self.offset
    }

    /// The year of this count that begins at 正月初一 of `gregorian_year`.
    #[must_use]
    pub const fn year_beginning_in(self, gregorian_year: i64) -> i64 {
        // The Chinese year beginning in 2024 is 4661.
        gregorian_year + 2_637 + self.offset
    }
}

/// 宋教仁's count, the one the revolution and the Republic's first day used:
/// the year beginning in 1905 is 4603, and in 1911 4609. Its epoch is 2698
/// BC.
pub const SONG_JIAOREN: HuangdiCount = HuangdiCount {
    id: "huangdi-era",
    english_name: "Huangdi era (Song Jiaoren)",
    era: "huangdi",
    offset: 61,
};

/// The 同盟會's count, in its address at the tomb of the Yellow Emperor:
/// the year beginning in 1908 is 4605. Its epoch is 2697 BC, the 甲子
/// year, and it is one less than [`SONG_JIAOREN`] throughout.
pub const TONGMENGHUI: HuangdiCount = HuangdiCount {
    id: "huangdi-era-tongmenghui",
    english_name: "Huangdi era (Tongmenghui)",
    era: "huangdi-tongmenghui",
    offset: 60,
};

/// 劉師培's count, of the Yellow Emperor's birth: the year beginning in
/// 1903 is 4614. Its epoch is 2711 BC.
pub const LIU_SHIPEI: HuangdiCount = HuangdiCount {
    id: "huangdi-era-liu-shipei",
    english_name: "Huangdi era (Liu Shipei)",
    era: "huangdi-liu-shipei",
    offset: 74,
};

/// The count of the journal 《江蘇》 and others: the year beginning in 1903
/// is 4394. Its epoch is 2491 BC.
pub const JIANGSU: HuangdiCount = HuangdiCount {
    id: "huangdi-era-jiangsu",
    english_name: "Huangdi era (Jiangsu)",
    era: "huangdi-jiangsu",
    offset: -146,
};

/// Every count, in registry order.
pub const ALL: [HuangdiCount; 4] = [SONG_JIAOREN, TONGMENGHUI, LIU_SHIPEI, JIANGSU];

/// A date of a Huangdi count: the Chinese date with its year renamed.
pub type HuangdiDate = ChineseDate;

/// A count of the years of the Yellow Emperor as a calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HuangdiCalendar(pub HuangdiCount);

impl HuangdiCalendar {
    /// The Chinese date of a date of this count.
    const fn chinese(self, date: HuangdiDate) -> ChineseDate {
        ChineseDate::new(self.0.chinese_year(date.year), date.month, date.day)
    }
}

impl Calendar for HuangdiCalendar {
    type Date = HuangdiDate;

    /// Unrecorded: the sources date each count by the documents that used
    /// it, from 1903 to the Republic's first day, and never by a first or
    /// a last day of use.
    fn usage(&self) -> Usage {
        Usage::UNRECORDED
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        ChineseCalendar.cycles()
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        ChineseCalendar.is_leap_year(self.0.chinese_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId(self.0.id),
            english_name: self.0.english_name,
            year_kind: YearKind::EpochForward,
            has_leap_months: true,
            is_astronomical: true,
            earliest: Some(chinese::EARLIEST),
            latest: Some(chinese::LATEST),
            native_locales: &["zh-Hant", "zh-Hans"],
        }
    }

    fn days_in_month(&self, fields: &DateFields) -> CalendarResult<u16> {
        let mut chinese = *fields;
        chinese.year = self.0.chinese_year(fields.year);
        chinese.era = None;
        ChineseCalendar.days_in_month(&chinese)
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        ChineseCalendar.to_fixed(self.chinese(date))
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let date = ChineseCalendar.from_fixed(rd)?;
        Ok(HuangdiDate::new(
            date.year + self.0.offset,
            date.month,
            date.day,
        ))
    }

    /// The Chinese calendar's fields, the year renamed and the era set.
    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let mut fields = ChineseCalendar.to_fields(self.chinese(date))?;
        fields.year = date.year;
        fields.era = Some(self.0.era);
        Ok(fields)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != self.0.era) {
            return Err(CalendarError::UnknownEra);
        }
        let date = HuangdiDate::new(fields.year, fields.require_month()?, fields.require_day()?);
        self.to_fixed(date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::Month;
    use hc_calendars_solar::gregorian;

    fn day(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn sun_yat_sen_made_4609_11_13_the_first_day_of_the_republic() {
        // 黄帝纪年四千六百九年十一月十三日 = 1 January 1912; the Hubei
        // Military Government's 4609年9月10日 = 31 October 1911.
        let calendar = HuangdiCalendar(SONG_JIAOREN);
        let republic = HuangdiDate::new(4_609, Month::regular(11), 13);
        assert_eq!(calendar.to_fixed(republic), Ok(day(1912, 1, 1)));
        assert_eq!(calendar.from_fixed(day(1912, 1, 1)), Ok(republic));
        let gazette = HuangdiDate::new(4_609, Month::regular(9), 10);
        assert_eq!(calendar.to_fixed(gazette), Ok(day(1911, 10, 31)));
        // The worked example of docs/systems/era-counts.md: the year began
        // on 30 January 1911 and its eleventh month on 20 December.
        assert_eq!(chinese::new_year(4_548), Ok(day(1911, 1, 30)));
        assert_eq!(
            calendar.to_fixed(HuangdiDate::new(4_609, Month::regular(11), 1)),
            Ok(day(1911, 12, 20))
        );
        // 宋教仁: 1905 is 4603.
        assert_eq!(SONG_JIAOREN.year_beginning_in(1905), 4_603);
        let new_year = chinese::new_year(SONG_JIAOREN.chinese_year(4_603)).unwrap();
        assert_eq!(gregorian::from_fixed(new_year).unwrap().0, 1905);
    }

    #[test]
    fn liu_shipei_signed_in_the_leap_fifth_month_of_4614() {
        // "黄帝降生四千六百一十四年闰五月十七日": 1903 is 4614 and has a
        // 閏五月.
        let calendar = HuangdiCalendar(LIU_SHIPEI);
        assert_eq!(LIU_SHIPEI.year_beginning_in(1903), 4_614);
        let signed = HuangdiDate::new(4_614, Month::leap(5), 17);
        let rd = calendar.to_fixed(signed).unwrap();
        assert_eq!(gregorian::from_fixed(rd).unwrap().0, 1903);
        assert_eq!(calendar.from_fixed(rd), Ok(signed));
        assert_eq!(calendar.is_leap_year(4_614), Ok(true));
    }

    #[test]
    fn the_other_counts_are_the_sources_years() {
        // 張新斌: 《江蘇》 1903 as 4394, the 同盟會 1908 as 4605; Wikipedia's
        // table: 1903 as 4600 in the 甲子 count, 4601 in 《民報》's.
        assert_eq!(JIANGSU.year_beginning_in(1903), 4_394);
        assert_eq!(TONGMENGHUI.year_beginning_in(1908), 4_605);
        assert_eq!(TONGMENGHUI.year_beginning_in(1903), 4_600);
        assert_eq!(SONG_JIAOREN.year_beginning_in(1903), 4_601);
        // Every count's year turns with the Chinese year, at 正月初一.
        let lunar_new_year = chinese::new_year(4_661).unwrap();
        for count in ALL {
            let calendar = HuangdiCalendar(count);
            let first = calendar.from_fixed(lunar_new_year).unwrap();
            let eve = calendar.from_fixed(Rd(lunar_new_year.0 - 1)).unwrap();
            assert_eq!(first.year, count.year_beginning_in(2024), "{}", count.id);
            assert_eq!((first.month, first.day), (Month::regular(1), 1));
            assert_eq!(eve.year, first.year - 1, "{}", count.id);
        }
    }

    #[test]
    fn every_count_is_the_chinese_calendar_renamed() {
        // The range is `chinese`'s, and on a sample of days, with every
        // new year and its eve from 1645 to 2150, the fields are
        // `chinese`'s with the year shifted and the era set.
        let (first, last) = (chinese::EARLIEST.0, chinese::LATEST.0);
        let mut days: alloc::vec::Vec<i64> = (first..=last)
            .step_by(89 * crate::sweep_stride(7))
            .chain([first, last])
            .collect();
        for year in 4_282..=4_787 {
            if let Ok(new_year) = chinese::new_year(year) {
                days.extend([new_year.0 - 1, new_year.0]);
            }
        }
        days.retain(|rd| (first..=last).contains(rd));
        crate::check_days(&days, |rd| {
            let base = ChineseCalendar.from_fixed(Rd(rd)).unwrap();
            let expected = ChineseCalendar.to_fields(base).unwrap();
            for count in ALL {
                let calendar = HuangdiCalendar(count);
                let date = calendar.from_fixed(Rd(rd)).unwrap();
                assert_eq!(date.year, base.year + count.offset, "RD {rd}");
                assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "RD {rd}");
                let fields = calendar.to_fields(date).unwrap();
                let mut renamed = expected;
                renamed.year += count.offset;
                renamed.era = Some(count.era);
                assert_eq!(fields, renamed, "RD {rd}");
                assert_eq!(calendar.from_fields(&fields), Ok(date), "RD {rd}");
            }
        });
        for count in ALL {
            let calendar = HuangdiCalendar(count);
            assert!(calendar.from_fixed(Rd(first - 1)).is_err());
            assert!(calendar.from_fixed(Rd(last + 1)).is_err());
            assert_eq!(
                calendar.from_fields(&DateFields::ymd(4_700, 1, 1).with_era("ad")),
                Err(CalendarError::UnknownEra)
            );
            let meta = calendar.meta();
            assert_eq!(meta.earliest, Some(chinese::EARLIEST));
            assert_eq!(meta.latest, Some(chinese::LATEST));
        }
    }

    #[test]
    fn the_identifiers_and_era_codes_are_distinct() {
        for (index, count) in ALL.iter().enumerate() {
            for other in &ALL[index + 1..] {
                assert_ne!(count.id, other.id);
                assert_ne!(count.era, other.era);
                assert_ne!(count.offset, other.offset);
            }
        }
    }
}
