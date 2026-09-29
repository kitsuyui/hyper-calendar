//! The Chinese lunisolar calendar — CLDR `chinese`.
//!
//! The rules are in [`crate::lunisolar`]; this module is the parameters.
//! The calendar as the Purple Mountain Observatory promulgates it under
//! GB/T 33661-2017, the Shíxiàn reform of 1645 that bounds it, the 1929
//! change of meridian, the year counts in circulation and the published new
//! years it was checked against are in
//! `docs/systems/east-asian-lunisolar.md`.
//!
//! # The meridian
//!
//! | From | Offset | |
//! |---|---|---|
//! | — | UT+7:45:40 | Beijing local mean time, 116°25′E |
//! | 1929 | UT+8 | the 120°E standard zone |
//!
//! Both rows are Reingold and Dershowitz's (`reingold2018code`,
//! `chinese-location`). The Chinese Wikipedia's 农历 (`wikipedia-zh-nongli`)
//! dates the 120°E standard to 民國十七年, 1928, and quotes 116°23′E as the
//! Beijing local time of the Hong Kong Space Museum's almanac; the first
//! moves no date, the second one month start, in 1687, and the tests below
//! measure both.
//!
//! A conjunction or a solstice falling in the fourteen minutes between the
//! two local midnights lands on different days under the two conventions,
//! and that moves a month boundary or, through the zhōngqì test, a leap
//! month.
//!
//! # The almanac, where it was read
//!
//! Before 1912 the calendar was the Qing 時憲書, computed by the Bureau of
//! Astronomy with its own solar and lunar theory — Tycho's until the 1730s,
//! the *Lìxiàng kǎochéng hòubiān* of 1742 after — in Beijing apparent time
//! (`liu-chinese-calendar-computation`), not by modern astronomy. Where one
//! of its conjunctions lies minutes from midnight the rules here can put
//! the month on the other day, and where one of its *zhōngqì* does, the
//! leap month in another lunation. [`ALMANAC_CORRECTIONS`] carries the
//! first days and [`ALMANAC_TERM_CORRECTIONS`] the term days that a record
//! of the promulgated calendar says so of:
//!
//! - **1645–1899**: 28 first days and the terms behind 5 leap months,
//!   where Liu Yuk Tung's reconstruction of the Qing calendar
//!   (`liu-chinese-calendar-computation`) differs from the rules, each
//!   checked against the month's opening line in the Veritable Records,
//!   《清實錄》 (`qing-shilu`), which give its first day by its sexagenary
//!   name: every one of the 33 months stands there as Liu has it.
//! - **1900–1911**: one first day, the fourth month of 1906, which the
//!   rules begin on 23 April and the almanac on 24 April; the table is the
//!   Purple Mountain Observatory's 1900–2025 calendar
//!   (`pmo-calendar-1900-2025`), which follows the 時憲書, and the
//!   Veritable Records have the same day.
//!
//! Beyond those, every month of 1645–1911 whose first day the Records'
//! transcription gives was read against them — 3 260 of the 3 303, once
//! two lines in error are set aside — and each begins on the Records' day
//! (`tests/chinese_qing.rs`).
//!
//! The almanac's twenty-four term days of 1645–1733, most of which move no
//! month, are [`almanac_solar_term_days`], from Liu's table of the
//! bureau's calendrical terms.
//!
//! Each entry names its source. The term days of the leap months are the
//! weaker part: the leap months themselves are the Veritable Records', the
//! term days behind four of them are Liu's, and the fifth, 處暑 of 1805, is
//! inferred from its leap month, as each entry says.
//!
//! # Year numbering
//!
//! Years are counted continuously from the 2637 BCE epoch of Reingold and
//! Dershowitz's published code ([`crate::lunisolar::CHINESE_EPOCH`]), so the
//! year that began on 2024-02-10 is 4661: the count for which
//! [`hc_calendar::cycle::sexagenary_year`] is directly right, 4661 being
//! *jiǎ-chén*, the Wood Dragon. The number is this library's choice, not a
//! count anyone prints: the book writes a year as cycle and position, and
//! the almanacs' 黃帝紀元 is 4722. The document says how the counts 4721 and
//! 4722 relate to it; none is official, because the calendar has no
//! official continuous era. The cycle and position are available through
//! [`Calendar::to_fields`], the reign eras through `hc-calendars-regional`.
//!
//! # Range
//!
//! 1645-01-01 to 2150-12-31 Gregorian: from the Shíxiàn calendar, which
//! introduced the true-solar-term rule implemented here, to the end of
//! `hc-astro`'s ΔT fit. Earlier years are refused rather than answered with
//! a rule that was not in force.

use hc_calendar::gregorian;
use hc_calendar::{Calendar, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd};

use crate::lunisolar::{
    CHINESE_EPOCH, LunisolarCalendar, LunisolarDate, LunisolarParameters, MajorTermCorrection,
    MeridianEra, MonthStartCorrection, SolarTermMode,
};

/// The machine identifier CLDR uses for this calendar.
pub const ID: CalendarId = CalendarId("chinese");

/// The last day the calendar was China's civil calendar, 31 December 1911:
/// the Republic adopted the Gregorian calendar at its founding the next day.
pub const LAST_CIVIL: Rd = gregorian::to_fixed_saturating(1911, 12, 31);

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "The Shíxiàn calendar promulgated by the Shunzhi Emperor for 1645 [wikipedia-en-chongzhen-calendar]; \
    civil until the Republic adopted the Gregorian calendar at its founding on 1 January 1912 \
    [wikipedia-adoption-gregorian]; kept since for \
    the festivals, under GB/T 33661-2017 today, as docs/systems/east-asian-lunisolar.md states";

/// The earliest fixed day this calendar converts.
pub const EARLIEST: Rd = gregorian::to_fixed_saturating(1645, 1, 1);

/// The latest fixed day this calendar converts.
pub const LATEST: Rd = gregorian::to_fixed_saturating(2150, 12, 31);

/// Where [`MERIDIANS`] comes from.
pub const MERIDIAN_SOURCES: &str = "Beijing at 116°25′E, 1397/180 hours, before 1929 and the 120°E zone from \
    1929, as chinese-location in the published code of Calendrical Calculations \
    [reingold2018code]; the reference recorded as moving to UT+8 in 1928-1929 \
    [wikipedia-en-time-in-china], and 1928 with 116°23′E in [wikipedia-zh-nongli]";

/// The months of 1645–1911 that the promulgated calendar began on another
/// day than the rules do, sorted by the rules' day; each entry's `source`
/// names the record it was read in.
///
/// Twenty-eight are from before 1900: the months where Liu's
/// reconstruction of the Qing calendar (`liu-chinese-calendar-computation`)
/// differs from the rules, each confirmed by the first day the Veritable
/// Records (`qing-shilu`) give the month, and all but three by KASI's data
/// for the Korean calendar (`kasi-lunisolar-conversion`), which followed
/// the Qing almanac. Every one of them has a conjunction within 23 minutes
/// of Beijing mean midnight. The last is 光緒三十二年四月, whose
/// conjunction the rules place at 23:52 Beijing mean time on 23 April 1906
/// and whose first day the 時憲書 gives as 戊戌, 24 April
/// (`pmo-calendar-1900-2025`); the third month runs thirty days with it and
/// the fourth twenty-nine.
pub static ALMANAC_CORRECTIONS: [MonthStartCorrection; 29] = [
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1652, 10, 2),
        gregorian::to_fixed_saturating(1652, 10, 3),
        "《清世祖實錄》, 順治九年九月庚午朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1653, 9, 21),
        gregorian::to_fixed_saturating(1653, 9, 22),
        "《清世祖實錄》, 順治十年八月甲子朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1673, 11, 8),
        gregorian::to_fixed_saturating(1673, 11, 9),
        "《清聖祖實錄》, 康熙十二年十月丁酉朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1686, 4, 22),
        gregorian::to_fixed_saturating(1686, 4, 23),
        "《清聖祖實錄》, 康熙二十五年四月乙酉朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1687, 3, 14),
        gregorian::to_fixed_saturating(1687, 3, 13),
        "《清聖祖實錄》, 康熙二十六年二月己酉朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1692, 6, 14),
        gregorian::to_fixed_saturating(1692, 6, 15),
        "《清聖祖實錄》, 康熙三十一年五月庚戌朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1693, 4, 5),
        gregorian::to_fixed_saturating(1693, 4, 6),
        "《清聖祖實錄》, 康熙三十二年三月乙巳朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1704, 10, 28),
        gregorian::to_fixed_saturating(1704, 10, 29),
        "《清聖祖實錄》, 康熙四十三年十月戊辰朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1708, 2, 22),
        gregorian::to_fixed_saturating(1708, 2, 21),
        "《清聖祖實錄》, 康熙四十七年二月戊寅朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1713, 12, 17),
        gregorian::to_fixed_saturating(1713, 12, 18),
        "《清聖祖實錄》, 康熙五十二年十一月乙巳朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1715, 3, 5),
        gregorian::to_fixed_saturating(1715, 3, 6),
        "《清聖祖實錄》, 康熙五十四年二月戊辰朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1728, 8, 5),
        gregorian::to_fixed_saturating(1728, 8, 6),
        "《清世宗實錄》, 雍正六年七月庚戌朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1731, 6, 4),
        gregorian::to_fixed_saturating(1731, 6, 5),
        "《清世宗實錄》, 雍正九年五月癸亥朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1754, 9, 16),
        gregorian::to_fixed_saturating(1754, 9, 17),
        "《清高宗實錄》, 乾隆十九年八月戊申朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1789, 10, 18),
        gregorian::to_fixed_saturating(1789, 10, 19),
        "《清高宗實錄》, 乾隆五十四年九月甲申朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1794, 11, 22),
        gregorian::to_fixed_saturating(1794, 11, 23),
        "《清高宗實錄》, 乾隆五十九年十一月乙酉朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1813, 4, 30),
        gregorian::to_fixed_saturating(1813, 5, 1),
        "《清仁宗實錄》, 嘉慶十八年四月戊戌朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1817, 10, 10),
        gregorian::to_fixed_saturating(1817, 10, 11),
        "《清仁宗實錄》, 嘉慶二十二年九月壬寅朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1820, 12, 5),
        gregorian::to_fixed_saturating(1820, 12, 6),
        "《清宣宗實錄》, 嘉慶二十五年十一月甲寅朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1823, 5, 10),
        gregorian::to_fixed_saturating(1823, 5, 11),
        "《清宣宗實錄》, 道光三年四月庚子朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1842, 1, 12),
        gregorian::to_fixed_saturating(1842, 1, 11),
        "《清宣宗實錄》, 道光二十一年十二月庚辰朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1842, 11, 2),
        gregorian::to_fixed_saturating(1842, 11, 3),
        "《清宣宗實錄》, 道光二十二年十月丙子朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1849, 9, 16),
        gregorian::to_fixed_saturating(1849, 9, 17),
        "《清宣宗實錄》, 道光二十九年八月丙寅朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1856, 11, 27),
        gregorian::to_fixed_saturating(1856, 11, 28),
        "《清文宗實錄》, 咸豐六年十一月乙卯朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1861, 11, 2),
        gregorian::to_fixed_saturating(1861, 11, 3),
        "《清穆宗實錄》, 咸豐十一年十月丙辰朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1869, 5, 11),
        gregorian::to_fixed_saturating(1869, 5, 12),
        "《清穆宗實錄》, 同治八年四月癸卯朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1880, 11, 2),
        gregorian::to_fixed_saturating(1880, 11, 3),
        "《清德宗實錄》, 光緒六年十月丙申朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1887, 3, 24),
        gregorian::to_fixed_saturating(1887, 3, 25),
        "《清德宗實錄》, 光緒十三年三月己丑朔 [qing-shilu]; Liu's table [liu-chinese-calendar-computation]; KASI [kasi-lunisolar-conversion]",
    ),
    MonthStartCorrection::new(
        gregorian::to_fixed_saturating(1906, 4, 23),
        gregorian::to_fixed_saturating(1906, 4, 24),
        "The 《时宪书》 of 光绪三十二年 as the Purple Mountain Observatory's 1900-2025 calendar gives it, \
        四月初一 on 戊戌 [pmo-calendar-1900-2025]; 《清德宗實錄》, 光緒三十二年四月戊戌朔 [qing-shilu]; \
        the Hong Kong Observatory's table for 1906 [hko-conversion-tables]; KASI \
        [kasi-lunisolar-conversion]",
    ),
];

/// The *zhōngqì* of 1645–1911 that the promulgated calendar reckoned to
/// another day than the rules do, and that move a leap month, sorted by the
/// rules' day; each entry's `source` says where the reckoning was read or
/// what it is inferred from.
///
/// Each entry is one of the five years in which the Veritable Records
/// (`qing-shilu`) place the leap month a lunation from where the rules put
/// it, and Liu's reconstruction (`liu-chinese-calendar-computation`) with
/// them. The terms that differ without moving a month are not carried:
/// they change no date.
///
/// - **1645**, 大暑: the 時憲書 printed it on the first day of 閏六月, but
///   before that day's conjunction, and counted it to the month before —
///   Lǐ Tiānjīng's rule, used that one year, as Wāng Yuēzhēn's
///   《歷代長術輯要》 explains it in Liu's account (cited by Liu, not read)
///   — so the term is reckoned here to 22 July, the last day of 六月.
/// - **1651**, 春分 on 20 March, **1661**, 秋分 on 23 September, and
///   **1727**, 穀雨 on 20 April: the bureau's own term days, from the
///   Tychonic theory in use before 1733, as Liu's table of calendrical
///   solar terms gives them.
/// - **1805**, 處暑 on 24 August: no table of the almanac's terms after
///   1733 was read. The rules put 處暑 at 23:52 Beijing mean time on
///   23 August, eight minutes before midnight, and the Veritable Records'
///   閏六月 from 26 July is possible only if the almanac put it after, so
///   the day is inferred from the leap month. Aslaksen (`aslaksen2010`,
///   §4.6) takes this month as his example of the meridian and puts the
///   term at 0h07m on 24 August at 120°E and about seven minutes before
///   midnight at Beijing, which supports a term within minutes of
///   midnight; his conclusion that Beijing's meridian made the leap month
///   the one after the sixth does not follow under the rule here, where a
///   term on 23 August puts the leap month after the seventh.
pub static ALMANAC_TERM_CORRECTIONS: [MajorTermCorrection; 5] = [
    MajorTermCorrection::new(
        6,
        gregorian::to_fixed_saturating(1645, 7, 23),
        gregorian::to_fixed_saturating(1645, 7, 22),
        "閏六月辛巳朔, 順治二年, in 《清世祖實錄》 [qing-shilu]; 大暑 on its first day counted to the \
        month before, by Wāng Yuēzhēn's account in [liu-chinese-calendar-computation] \
        (《歷代長術輯要》, cited by Liu, not read)",
    ),
    MajorTermCorrection::new(
        2,
        gregorian::to_fixed_saturating(1651, 3, 21),
        gregorian::to_fixed_saturating(1651, 3, 20),
        "閏二月戊申朔, 順治八年, in 《清世祖實錄》 [qing-shilu]; 春分 on 20 March in Liu's calendrical \
        solar terms only [liu-chinese-calendar-computation]",
    ),
    MajorTermCorrection::new(
        8,
        gregorian::to_fixed_saturating(1661, 9, 22),
        gregorian::to_fixed_saturating(1661, 9, 23),
        "閏七月戊寅朔, 順治十八年, in 《清聖祖實錄》 [qing-shilu]; 秋分 on 23 September in Liu's \
        calendrical solar terms only [liu-chinese-calendar-computation]",
    ),
    MajorTermCorrection::new(
        3,
        gregorian::to_fixed_saturating(1727, 4, 21),
        gregorian::to_fixed_saturating(1727, 4, 20),
        "閏三月丁巳朔, 雍正五年, in 《清世宗實錄》 [qing-shilu]; 穀雨 on 20 April in Liu's calendrical \
        solar terms only [liu-chinese-calendar-computation]",
    ),
    MajorTermCorrection::new(
        7,
        gregorian::to_fixed_saturating(1805, 8, 23),
        gregorian::to_fixed_saturating(1805, 8, 24),
        "閏六月壬午朔, 嘉慶十年, in 《清仁宗實錄》 [qing-shilu], and Liu's table \
        [liu-chinese-calendar-computation]; 處暑 on 24 August inferred from that leap month, no \
        record of the term day read; the term within minutes of midnight also in [aslaksen2010]",
    ),
];

/// The first Gregorian year of [`almanac_solar_term_days`].
pub const ALMANAC_SOLAR_TERMS_FIRST_YEAR: i64 = 1645;

/// The last Gregorian year of [`almanac_solar_term_days`].
pub const ALMANAC_SOLAR_TERMS_LAST_YEAR: i64 = 1733;

/// The years of the Calendar Case, 1667–1669, whose almanacs computed the
/// terms by the Dàtǒng system's *píngqì* and which
/// [`almanac_solar_term_days`] refuses.
pub const DATONG_TERM_YEARS: core::ops::RangeInclusive<i64> = 1667..=1669;

/// The Qing almanac's twenty-four solar terms of 1645–1733, as Liu Yuk
/// Tung tabulates the "calendrical solar terms", the terms the bureau's
/// Tychonic system gave before the change to Kepler's laws in the 1730s
/// (`liu-chinese-calendar-computation`, § The Qing Period, and the
/// `calendricalSolarTerms` table of his conversion data, `table_c.js`).
///
/// One row per Gregorian year in Liu's own encoding: the day of January of
/// 小寒, then for each of the next twenty-three terms, 大寒 to 冬至, its
/// interval from the one before less fourteen days. Read that way the
/// table gives the three term days the leap months of 1651, 1661 and 1727
/// turn on as [`ALMANAC_TERM_CORRECTIONS`] carries them — 春分 on 20 March
/// 1651, 秋分 on 23 September 1661, 穀雨 on 20 April 1727 — which is how the
/// encoding was confirmed. The rows for 1667–1669 are the Western system's
/// recomputation, which Liu says differs from the almanacs of those years;
/// [`almanac_solar_term_days`] refuses them.
#[rustfmt::skip]
static ALMANAC_SOLAR_TERMS: [[u8; 24]; 89] = [
    [5, 1, 0, 1, 1, 1, 1, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 1, 1, 1, 0, 1], // 1645
    [5, 1, 0, 1, 1, 1, 1, 2, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 0], // 1646
    [5, 1, 1, 0, 1, 1, 2, 1, 2, 1, 2, 2, 1, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1], // 1647
    [5, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 1, 2, 1, 1, 0, 1, 1], // 1648
    [5, 0, 1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 1, 1, 1, 0, 1], // 1649
    [5, 1, 0, 1, 1, 1, 1, 2, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 0], // 1650
    [5, 1, 1, 0, 1, 1, 2, 1, 1, 2, 2, 2, 1, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1], // 1651
    [5, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 1, 2, 2, 0, 2, 2, 1, 1, 0, 1, 1], // 1652
    [5, 0, 1, 1, 1, 1, 1, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 1, 1, 1, 0, 1], // 1653
    [5, 1, 0, 1, 1, 0, 2, 2, 1, 2, 2, 1, 2, 2, 1, 2, 2, 1, 1, 1, 1, 1, 0, 1], // 1654
    [5, 1, 1, 0, 1, 1, 2, 1, 1, 2, 2, 2, 1, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1], // 1655
    [5, 2, 0, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 1, 2, 1, 1, 0, 1, 1], // 1656
    [5, 0, 1, 1, 1, 1, 1, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 1, 1, 1, 0, 1], // 1657
    [5, 0, 1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 1, 1, 1, 1, 1, 0], // 1658
    [5, 1, 0, 1, 1, 1, 2, 1, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1], // 1659
    [5, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 1, 2, 1, 1, 0, 1, 1], // 1660
    [5, 0, 1, 1, 1, 1, 1, 1, 2, 2, 1, 2, 2, 1, 2, 2, 1, 2, 1, 1, 1, 1, 0, 1], // 1661
    [5, 0, 1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 1, 1, 1, 1, 1, 0], // 1662
    [5, 1, 0, 1, 1, 1, 2, 1, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1], // 1663
    [5, 1, 1, 1, 0, 2, 1, 1, 2, 1, 2, 2, 1, 2, 2, 2, 1, 1, 2, 1, 1, 0, 1, 1], // 1664
    [4, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 2, 1, 1, 1, 1, 0, 1], // 1665
    [5, 0, 1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 1, 1, 1, 1, 0], // 1666
    [5, 1, 0, 1, 1, 1, 2, 1, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1], // 1667
    [5, 1, 1, 0, 1, 2, 1, 1, 2, 1, 2, 2, 1, 2, 2, 1, 2, 1, 1, 2, 0, 1, 1, 1], // 1668
    [4, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 2, 1, 1, 1, 1, 0, 1], // 1669
    [5, 0, 1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 1, 1, 1, 1, 0], // 1670
    [5, 1, 0, 1, 1, 1, 1, 2, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 0], // 1671
    [5, 1, 1, 0, 1, 1, 2, 1, 2, 1, 2, 2, 1, 2, 2, 1, 2, 1, 1, 2, 0, 1, 1, 1], // 1672
    [4, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 2, 1, 1, 1, 1, 0, 1], // 1673
    [5, 0, 1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 1, 1, 1, 1, 0], // 1674
    [5, 1, 0, 1, 1, 1, 1, 2, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 0], // 1675
    [5, 1, 1, 0, 1, 1, 2, 1, 2, 1, 2, 2, 1, 2, 2, 1, 2, 1, 1, 2, 0, 1, 1, 1], // 1676
    [4, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 1, 2, 1, 1, 1, 0, 1], // 1677
    [5, 0, 1, 1, 1, 1, 1, 2, 1, 1, 2, 2, 2, 2, 1, 2, 1, 2, 1, 1, 1, 1, 0, 1], // 1678
    [5, 1, 0, 1, 1, 1, 1, 2, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 0], // 1679
    [5, 1, 1, 0, 1, 1, 2, 1, 1, 2, 2, 2, 1, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1], // 1680
    [4, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 1, 2, 1, 1, 0, 1, 1], // 1681
    [5, 0, 1, 1, 1, 1, 1, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 1, 1, 1, 0, 1], // 1682
    [5, 1, 0, 1, 1, 1, 1, 2, 1, 2, 2, 1, 2, 2, 1, 2, 2, 1, 1, 1, 1, 1, 1, 0], // 1683
    [5, 1, 1, 0, 1, 1, 2, 1, 1, 2, 2, 2, 1, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1], // 1684
    [4, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 1, 2, 1, 1, 0, 1, 1], // 1685
    [5, 0, 1, 1, 1, 1, 1, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 1, 1, 1, 0, 1], // 1686
    [5, 1, 0, 1, 1, 1, 1, 2, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 1, 1, 1, 1, 1, 0], // 1687
    [5, 1, 1, 0, 1, 1, 2, 1, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1], // 1688
    [4, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 1, 2, 2, 2, 1, 1, 2, 1, 1, 0, 1, 1], // 1689
    [5, 0, 1, 1, 1, 1, 1, 1, 2, 2, 1, 2, 2, 1, 2, 2, 1, 2, 1, 1, 1, 1, 0, 1], // 1690
    [5, 0, 1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 1, 1, 1, 1, 1, 0], // 1691
    [5, 1, 0, 1, 1, 1, 2, 1, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1], // 1692
    [4, 1, 2, 0, 1, 1, 1, 1, 2, 1, 2, 2, 1, 2, 2, 2, 1, 1, 2, 1, 1, 0, 1, 1], // 1693
    [5, 0, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 2, 1, 1, 1, 1, 0, 1], // 1694
    [5, 0, 1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 1, 1, 1, 1, 0], // 1695
    [5, 1, 0, 1, 1, 1, 2, 1, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1], // 1696
    [4, 1, 1, 1, 0, 2, 1, 1, 2, 1, 2, 2, 1, 2, 2, 1, 2, 1, 2, 1, 1, 0, 1, 1], // 1697
    [4, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 2, 1, 1, 1, 1, 0, 1], // 1698
    [5, 0, 1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 1, 1, 1, 1, 0], // 1699
    [5, 1, 0, 1, 1, 1, 1, 2, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1], // 1700
    [5, 1, 1, 1, 0, 2, 1, 1, 2, 1, 2, 2, 1, 2, 2, 1, 2, 1, 1, 1, 2, 0, 1, 1], // 1701
    [5, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 2, 1, 1, 1, 1, 0, 1], // 1702
    [6, 0, 1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 1, 1, 1, 1, 0], // 1703
    [6, 1, 0, 1, 1, 1, 1, 2, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1], // 1704
    [5, 1, 1, 0, 1, 1, 2, 1, 2, 1, 2, 2, 1, 2, 2, 1, 2, 1, 1, 2, 0, 1, 1, 1], // 1705
    [5, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 1, 2, 1, 1, 1, 0, 1], // 1706
    [6, 0, 1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 1, 1, 1, 1, 0], // 1707
    [6, 1, 0, 1, 1, 1, 1, 2, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 0], // 1708
    [5, 1, 1, 0, 1, 1, 2, 1, 2, 1, 2, 2, 1, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1], // 1709
    [5, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 1, 2, 1, 1, 1, 0, 1], // 1710
    [6, 0, 1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 1, 1, 1, 1, 0], // 1711
    [6, 1, 0, 1, 1, 1, 1, 2, 1, 2, 2, 1, 2, 2, 1, 2, 2, 1, 1, 1, 1, 1, 1, 0], // 1712
    [5, 1, 1, 0, 1, 1, 2, 1, 1, 2, 2, 2, 1, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1], // 1713
    [5, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 1, 2, 1, 1, 0, 1, 1], // 1714
    [6, 0, 1, 1, 1, 1, 1, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 1, 1, 1, 0, 1], // 1715
    [6, 1, 0, 1, 1, 1, 1, 2, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 1, 1, 1, 1, 1, 0], // 1716
    [5, 1, 1, 0, 1, 1, 2, 1, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1], // 1717
    [5, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 1, 2, 1, 1, 0, 1, 1], // 1718
    [6, 0, 1, 1, 1, 1, 1, 1, 2, 2, 1, 2, 2, 1, 2, 2, 1, 2, 1, 1, 1, 1, 0, 1], // 1719
    [6, 1, 0, 1, 1, 1, 1, 2, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 1, 1, 1, 1, 1, 0], // 1720
    [5, 1, 1, 0, 1, 1, 2, 1, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1], // 1721
    [5, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 1, 2, 2, 2, 1, 1, 2, 1, 1, 0, 1, 1], // 1722
    [6, 0, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 2, 1, 1, 1, 1, 0, 1], // 1723
    [6, 1, 0, 1, 1, 1, 1, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 1, 1, 1, 1, 0], // 1724
    [5, 1, 0, 1, 1, 1, 2, 1, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1], // 1725
    [5, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 1, 2, 2, 1, 2, 1, 2, 1, 1, 0, 1, 1], // 1726
    [6, 0, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 2, 1, 1, 1, 1, 0, 1], // 1727
    [6, 0, 1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 1, 1, 1, 1, 0], // 1728
    [5, 1, 0, 1, 1, 1, 2, 1, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1], // 1729
    [5, 1, 1, 1, 0, 2, 1, 1, 2, 1, 2, 2, 1, 2, 2, 1, 2, 1, 2, 1, 1, 0, 1, 1], // 1730
    [6, 0, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 1, 2, 2, 1, 2, 1, 1, 1, 1, 0, 1], // 1731
    [6, 0, 1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 1, 1, 1, 1, 0], // 1732
    [5, 1, 0, 1, 1, 1, 1, 2, 1, 2, 2, 1, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1], // 1733
];

/// The days of the twenty-four solar terms the Qing almanac gave in
/// Gregorian year `year`, 小寒 first and 冬至 last, or `None` outside
/// 1645–1733 and in the Dàtǒng years 1667–1669 (`DATONG_TERM_YEARS`).
///
/// Before the bureau went over to Kepler's laws its term days often stood a
/// day from the modern ones; most of those move no month, so the calendar
/// here does not need them, and only the five that move a leap month are
/// [`ALMANAC_TERM_CORRECTIONS`]. This is the almanac's own list, for a
/// reader who wants the term day the almanac printed. From 1734 Liu lists
/// the almanac's terms only where they differ from the modern ones, and
/// that list was not transcribed.
#[must_use]
pub fn almanac_solar_term_days(year: i64) -> Option<[Rd; 24]> {
    if DATONG_TERM_YEARS.contains(&year) {
        return None;
    }
    let row =
        ALMANAC_SOLAR_TERMS.get(usize::try_from(year - ALMANAC_SOLAR_TERMS_FIRST_YEAR).ok()?)?;
    let mut day = gregorian::to_fixed_saturating(year, 1, row[0]);
    let mut days = [day; 24];
    for (slot, &interval) in days.iter_mut().zip(row.iter()).skip(1) {
        day = Rd(day.0 + 14 + i64::from(interval));
        *slot = day;
    }
    Some(days)
}

/// The meridian history of the Chinese calendar. Sources:
/// [`MERIDIAN_SOURCES`].
pub static MERIDIANS: [MeridianEra; 2] = [
    MeridianEra::from_longitude(
        i64::MIN / 4,
        116.416_666_666_666_67,
        "Beijing local mean time",
    ),
    MeridianEra::from_zone(1929, 8.0, "the 120°E standard zone"),
];

/// The parameters of the Chinese calendar.
pub static PARAMETERS: LunisolarParameters = LunisolarParameters {
    id: ID,
    english_name: "Chinese",
    native_locales: &["zh-Hans", "zh-Hant"],
    meridians: &MERIDIANS,
    epoch: CHINESE_EPOCH,
    year_offset: 0,
    solar_term_mode: SolarTermMode::Apparent,
    month_start_corrections: &ALMANAC_CORRECTIONS,
    major_term_corrections: &ALMANAC_TERM_CORRECTIONS,
    mean_motion: None,
    earliest: Some(EARLIEST),
    latest: Some(LATEST),
};

/// The engine configured as the Chinese calendar.
pub const ENGINE: LunisolarCalendar = LunisolarCalendar::new(&PARAMETERS);

/// A Chinese date.
///
/// All four lunisolar calendars in this crate share one representation on
/// purpose: a year, a possibly-intercalary month and a day is the whole of
/// what any of them records, and the calendar the date came from is what says
/// which days those are. Four identical structs would be the copying this
/// crate exists to avoid.
pub type ChineseDate = LunisolarDate;

/// The Chinese lunisolar calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ChineseCalendar;

impl Calendar for ChineseCalendar {
    type Date = ChineseDate;

    /// In use since the Shíxiàn calendar of 1645, which is also where the
    /// range begins; civil until the end of 1911, and the calendar of the
    /// Spring Festival and every other traditional date since, so it has an
    /// end of civil use and no end.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::since(EARLIEST, USAGE_SOURCE).civil_until(LAST_CIVIL)
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::LUNISOLAR_TWELVE
    }

    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        PARAMETERS.is_leap_year(year)
    }

    fn meta(&self) -> CalendarMeta {
        ENGINE.meta()
    }

    /// The engine's one-new-moon rule, not the trait's day-by-day walk.
    fn days_in_month(&self, fields: &DateFields) -> CalendarResult<u16> {
        ENGINE.days_in_month(fields)
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        ENGINE.to_fixed(date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        ENGINE.from_fixed(rd)
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        ENGINE.to_fields(date)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        ENGINE.from_fields(fields)
    }
}

/// The fixed day of Chinese New Year — 1 Zhēngyuè — of `year`.
///
/// # Errors
///
/// Returns [`hc_calendar::CalendarError::YearOutOfRange`] outside the
/// supported range.
pub fn new_year(year: i64) -> CalendarResult<Rd> {
    PARAMETERS.new_year(year)
}

/// A person's age as the Chinese count reckons it, on a fixed day: one at
/// birth, and one more at each Chinese New Year after, whatever the day
/// of birth (`chinese-age` in the published code of *Calendrical
/// Calculations*, `reingold2018code`). `Ok(None)` for a day before the
/// birth, which has no age.
///
/// This is the count Wikipedia's "East Asian age reckoning" gives as the
/// pre-modern reckoning of *suì* in China: one at birth and one more at
/// each lunar new year (`wikipedia-en-east-asian-age-reckoning`). Nothing
/// here says how any other country counts.
///
/// The year turns at 正月初一 because that is the rule of `chinese-age`.
/// The counts that turn elsewhere are their own functions:
/// [`reckoned_age_at_lichun`] at 立春, and [`reckoned_age_at_new_year_day`]
/// and [`year_age`] at 1 January.
///
/// # Errors
///
/// Returns a [`hc_calendar::CalendarError`] when the birth date does not
/// exist or either day is outside the supported range.
pub fn reckoned_age(birth: ChineseDate, on: Rd) -> CalendarResult<Option<u32>> {
    let born = ChineseCalendar.to_fixed(birth)?;
    if on < born {
        return Ok(None);
    }
    let today = ChineseCalendar.from_fixed(on)?;
    u32::try_from(today.year - birth.year + 1)
        .map(Some)
        .map_err(|_| hc_calendar::CalendarError::YearOutOfRange)
}

/// The day 立春 falls on in Gregorian year `year`, at the calendar's
/// meridian: the day at whose end the last minor term passed is 立春 and
/// at whose beginning it was 小寒.
///
/// # Errors
///
/// Returns [`hc_calendar::CalendarError::YearOutOfRange`] outside the
/// calendar's range.
pub fn lichun_day(year: i64) -> CalendarResult<Rd> {
    let start = gregorian::to_fixed_saturating(year, 2, 1);
    if start < EARLIEST || start > LATEST {
        return Err(hc_calendar::CalendarError::YearOutOfRange);
    }
    // 立春 is on 3, 4 or 5 February in every year of the range.
    (start.0..start.0 + 7)
        .map(Rd)
        .find(|&day| minor_solar_term(day) != 1 && minor_solar_term(Rd(day.0 + 1)) == 1)
        .ok_or(hc_calendar::CalendarError::YearOutOfRange)
}

/// A person's age counted from 立春 rather than from the New Year, on a
/// fixed day: one at birth and one more on each day 立春 falls on after
/// the day of birth. `Ok(None)` for a day before the birth.
///
/// 果壳's account of 虚岁 gives the count as one more at 正月初一 and adds
/// that in some places the age turns at 立春 instead:
/// 「在有的地方，长虚岁的节点是立春」 (`guokr-xusui`, a popular secondary
/// source; no primary account of the local custom was read). A birth on
/// the day of 立春 is counted after it, since the source does not say.
///
/// A child born on 1 June 2009 is one until 3 February 2010 and two from
/// 4 February 2010, the 立春 the *South China Morning Post* gives for that
/// year (`scmp-double-spring-2009`); under [`reckoned_age`] the same child
/// turns two at the New Year of 14 February 2010.
///
/// # Errors
///
/// Returns [`hc_calendar::CalendarError::YearOutOfRange`] when a year
/// between the two days is outside the calendar's range.
pub fn reckoned_age_at_lichun(birth: Rd, on: Rd) -> CalendarResult<Option<u32>> {
    if on < birth {
        return Ok(None);
    }
    let first = gregorian::year_from_fixed(birth);
    let last = gregorian::year_from_fixed(on);
    let mut age = 1u32;
    for year in first..=last {
        let day = lichun_day(year)?;
        if birth < day && day <= on {
            age += 1;
        }
    }
    Ok(Some(age))
}

/// A person's age counted in Gregorian years, one at birth and one more
/// each 1 January: the Korean 세는 나이, and the
/// modern 虚岁 of Korea (`wikipedia-zh-xusui`). `None` for a day before
/// the birth.
///
/// Wikipedia's "East Asian age reckoning" gives it as "Age = (Current Year
/// − Birth Year) + 1", and a child born on 31 December as two the next day
/// (`wikipedia-en-east-asian-age-reckoning`); 虚岁 on the Chinese
/// Wikipedia has the same, a person born on 除夕 two the day after and one
/// born on 1 January not two that day (`wikipedia-zh-xusui`).
#[must_use]
pub fn reckoned_age_at_new_year_day(birth: Rd, on: Rd) -> Option<u32> {
    year_age(birth, on).and_then(|age| age.checked_add(1))
}

/// A person's age counted in Gregorian years from nothing at birth, one
/// more each 1 January: the Korean 연 나이 some South Korean
/// laws use, "the difference between one's birth year and the current
/// year" (`wikipedia-en-east-asian-age-reckoning`), and the modern 虚岁
/// of Vietnam and parts of China, which starts at nothing
/// (`wikipedia-zh-xusui`). `None` for a day before the birth.
#[must_use]
pub fn year_age(birth: Rd, on: Rd) -> Option<u32> {
    if on < birth {
        return None;
    }
    u32::try_from(gregorian::year_from_fixed(on) - gregorian::year_from_fixed(birth)).ok()
}

/// Where 立春 (*lìchūn*, the Beginning of Spring) falls in a Chinese year,
/// the ground of the marriage auguries of the almanacs: none in the year,
/// once near its end, once near its start, or at both
/// (`chinese-year-marriage-augury` and the constants `widow`, `blind`,
/// `bright` and `double-bright` in the published code of *Calendrical
/// Calculations*, `reingold2018code`, whose names these are).
///
/// Wikipedia's "Lichun" calls a year without the term 無春年, 寡婦年
/// ("widow year") in the north and 盲年 ("blind year") in the south, and
/// says marriage in it is thought unlucky (`wikipedia-en-lichun`). So "blind"
/// names a year without 立春 there and a year with it only at the end in
/// the published code; the names here are the code's, and the Chinese
/// names the sources give are [`MarriageAugury::chinese_names`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarriageAugury {
    /// No 立春 in the year (the code's "double-blind year").
    Widow,
    /// 立春 once, near the end of the year.
    Blind,
    /// 立春 once, near the start of the year.
    Bright,
    /// 立春 twice, at the start and at the end ("double happiness").
    DoubleBright,
}

/// A Chinese name of a [`MarriageAugury`], as a source writes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuguryName {
    /// The name as written, in the script of `locale`.
    pub name: &'static str,
    /// The BCP 47 tag of the script it is written in, `zh-Hant` or
    /// `zh-Hans`.
    pub locale: &'static str,
    /// Where the name is used, when the source says: `"north"` or
    /// `"south"`.
    pub region: Option<&'static str>,
    /// Where the name was read, with the bibliography key.
    pub source: &'static str,
}

/// The names of a year without 立春.
static WIDOW_NAMES: [AuguryName; 6] = [
    AuguryName {
        name: "無春年",
        locale: "zh-Hant",
        region: None,
        source: "Wikipedia, \"Lichun\" [wikipedia-en-lichun]",
    },
    AuguryName {
        name: "寡婦年",
        locale: "zh-Hant",
        region: Some("north"),
        source: "Wikipedia, \"Lichun\" [wikipedia-en-lichun]",
    },
    AuguryName {
        name: "盲年",
        locale: "zh-Hant",
        region: Some("south"),
        source: "Wikipedia, \"Lichun\" [wikipedia-en-lichun]",
    },
    AuguryName {
        name: "无春年",
        locale: "zh-Hans",
        region: None,
        source: "维基百科「立春」 [wikipedia-zh-lichun]",
    },
    AuguryName {
        name: "寡妇年",
        locale: "zh-Hans",
        region: Some("north"),
        source: "维基百科「立春」 [wikipedia-zh-lichun]",
    },
    AuguryName {
        name: "盲年",
        locale: "zh-Hans",
        region: Some("south"),
        source: "维基百科「立春」 [wikipedia-zh-lichun]",
    },
];

/// The names of a year with two 立春.
static DOUBLE_BRIGHT_NAMES: [AuguryName; 2] = [
    AuguryName {
        name: "雙春兼閏月",
        locale: "zh-Hant",
        region: None,
        source: "Hong Kong Observatory, 『氣象冷知識』：雙春兼閏月, 2020 [hko-double-spring]",
    },
    AuguryName {
        name: "双春年",
        locale: "zh-Hans",
        region: None,
        source: "维基百科「立春」 [wikipedia-zh-lichun]",
    },
];

impl MarriageAugury {
    /// The Chinese names the sources read give this kind of year.
    ///
    /// A year without 立春 is 無春年, and 寡婦年 in the north and 盲年 in the
    /// south (`wikipedia-en-lichun`; 无春年, 寡妇年 and 盲年 in
    /// `wikipedia-zh-lichun`); a year with two is 雙春兼閏月
    /// (`hko-double-spring`) or 双春年 (`wikipedia-zh-lichun`). No source
    /// read names a year with one 立春, at its start or at its end, so
    /// [`MarriageAugury::Bright`] and [`MarriageAugury::Blind`] have none;
    /// the Chinese 盲年 is this crate's [`MarriageAugury::Widow`], not its
    /// `Blind`.
    #[must_use]
    pub const fn chinese_names(self) -> &'static [AuguryName] {
        match self {
            Self::Widow => &WIDOW_NAMES,
            Self::DoubleBright => &DOUBLE_BRIGHT_NAMES,
            Self::Blind | Self::Bright => &[],
        }
    }

    /// Whether the year's first 立春 comes after its New Year.
    #[must_use]
    pub const fn lichun_at_start(self) -> bool {
        matches!(self, Self::Bright | Self::DoubleBright)
    }

    /// Whether a 立春 comes before the next New Year after the first.
    #[must_use]
    pub const fn lichun_at_end(self) -> bool {
        matches!(self, Self::Blind | Self::DoubleBright)
    }
}

/// The last minor solar term (節氣) to begin before the local midnight
/// that starts `rd`, numbered 1 for 立春 at 315° to 12 for 小寒 at 285°
/// (`current-minor-solar-term` in the published code of *Calendrical
/// Calculations*).
fn minor_solar_term(rd: Rd) -> i64 {
    let longitude = hc_astro::solar_longitude(PARAMETERS.midnight(rd));
    (2 + hc_core::math::floor((longitude - 15.0) / 30.0) as i64).rem_euclid(12) + 1
}

/// The marriage augury of Chinese year `year`
/// (`chinese-year-marriage-augury`): whether 立春 falls after its New Year
/// and whether another falls before the next.
///
/// # Errors
///
/// Returns [`hc_calendar::CalendarError::YearOutOfRange`] when the year or
/// the next one begins outside the supported range.
pub fn marriage_augury(year: i64) -> CalendarResult<MarriageAugury> {
    // At New Year the last minor term is 小寒 (12) when 立春 is still to
    // come, and 立春 (1) itself when it has passed.
    let at_start = minor_solar_term(new_year(year)?) != 1;
    let at_end = minor_solar_term(new_year(year + 1)?) != 12;
    Ok(match (at_start, at_end) {
        (false, false) => MarriageAugury::Widow,
        (false, true) => MarriageAugury::Blind,
        (true, false) => MarriageAugury::Bright,
        (true, true) => MarriageAugury::DoubleBright,
    })
}

#[cfg(test)]
mod tests {
    use hc_calendar::{CalendarError, Month};

    use super::*;
    use crate::lunisolar::LunisolarDate;

    #[test]
    fn the_almanac_term_days_give_the_three_days_the_leap_months_turn_on() {
        let day = |year, month, day| gregorian::to_fixed_saturating(year, month, day);
        // 春分, the sixth term from 小寒; 秋分, the eighteenth; 穀雨, the eighth.
        assert_eq!(
            almanac_solar_term_days(1651).map(|days| days[5]),
            Some(day(1651, 3, 20))
        );
        assert_eq!(
            almanac_solar_term_days(1661).map(|days| days[17]),
            Some(day(1661, 9, 23))
        );
        assert_eq!(
            almanac_solar_term_days(1727).map(|days| days[7]),
            Some(day(1727, 4, 20))
        );
        for year in ALMANAC_SOLAR_TERMS_FIRST_YEAR..=ALMANAC_SOLAR_TERMS_LAST_YEAR {
            let Some(days) = almanac_solar_term_days(year) else {
                assert!(DATONG_TERM_YEARS.contains(&year));
                continue;
            };
            // 小寒 early in January, 冬至 in the third week of December.
            let (_, month, first) = gregorian::ymd(days[0]);
            let (_, last_month, last) = gregorian::ymd(days[23]);
            assert_eq!(month, 1, "{year}");
            assert!((4..=7).contains(&first), "{year}");
            assert_eq!(last_month, 12, "{year}");
            assert!((20..=23).contains(&last), "{year}");
        }
        for year in [1644, 1667, 1668, 1669, 1734] {
            assert_eq!(almanac_solar_term_days(year), None, "{year}");
        }
    }

    #[test]
    fn the_almanac_term_days_move_no_month_but_the_five_carried() {
        // Every major term of the almanac's list that the rules put on
        // another day, as a correction; applied, they must leave every
        // month where the calendar with its five term corrections has it,
        // which is what "they change no date" means.
        let mut corrections = Vec::new();
        let mut differing = 0;
        for year in ALMANAC_SOLAR_TERMS_FIRST_YEAR..=ALMANAC_SOLAR_TERMS_LAST_YEAR {
            let Some(days) = almanac_solar_term_days(year) else {
                continue;
            };
            for position in (1..24).step_by(2) {
                let term = hc_core::math::amod((position as i64 + 1) / 2 - 1, 12);
                let almanac = days[position];
                let computed = (almanac.0 - 3..=almanac.0 + 3)
                    .map(Rd)
                    .find(|&rd| {
                        PARAMETERS.computed_major_solar_term(Rd(rd.0 + 1)) == term
                            && PARAMETERS.computed_major_solar_term(rd) != term
                    })
                    .expect("the rules' term within three days");
                if computed == almanac {
                    continue;
                }
                differing += 1;
                assert!((computed.0 - almanac.0).abs() <= MajorTermCorrection::MAX_SHIFT);
                // 1645's 大暑 is carried by Lǐ Tiānjīng's rule, not by the
                // day the almanac printed.
                if !ALMANAC_TERM_CORRECTIONS
                    .iter()
                    .any(|carried| carried.term as i64 == term && carried.computed == computed)
                {
                    corrections.push(MajorTermCorrection::new(
                        term as u8, computed, almanac, "test",
                    ));
                }
            }
        }
        corrections.extend(ALMANAC_TERM_CORRECTIONS);
        corrections.sort_by_key(|correction| correction.computed);
        let every_term: &'static [MajorTermCorrection] = Box::leak(corrections.into_boxed_slice());
        let with_every_term: &'static LunisolarParameters =
            Box::leak(Box::new(LunisolarParameters {
                major_term_corrections: every_term,
                ..PARAMETERS
            }));
        assert_eq!(differing, 88);
        for year in ALMANAC_SOLAR_TERMS_FIRST_YEAR..=ALMANAC_SOLAR_TERMS_LAST_YEAR {
            let chinese_year = year + 2_637;
            assert_eq!(
                with_every_term.new_year(chinese_year),
                PARAMETERS.new_year(chinese_year),
                "{year}"
            );
            assert_eq!(
                with_every_term.leap_month(chinese_year),
                PARAMETERS.leap_month(chinese_year),
                "{year}"
            );
        }
    }

    #[test]
    fn the_year_of_the_rooster_2017_had_two_lichun_and_the_dog_year_after_it_one_at_its_end() {
        // 雙春 of 丁酉: 立春 on 正月初七 and 臘月十九, and 立春 on the 除夕 of
        // 戊戌 (`wikipedia-zh-shuangchun`).
        let day = |year, month, day| gregorian::to_fixed_saturating(year, month, day);
        assert_eq!(marriage_augury(4_654), Ok(MarriageAugury::DoubleBright));
        assert_eq!(
            ChineseCalendar.from_fixed(lichun_day(2017).expect("in range")),
            Ok(LunisolarDate::new(4_654, Month::regular(1), 7))
        );
        assert_eq!(
            ChineseCalendar.from_fixed(lichun_day(2018).expect("in range")),
            Ok(LunisolarDate::new(4_654, Month::regular(12), 19))
        );
        assert_eq!(marriage_augury(4_655), Ok(MarriageAugury::Blind));
        assert_eq!(
            lichun_day(2019),
            Ok(Rd(new_year(4_656).expect("in range").0 - 1))
        );
        assert_eq!(lichun_day(2019), Ok(day(2019, 2, 4)));
        let names = |augury: MarriageAugury| -> Vec<&str> {
            augury
                .chinese_names()
                .iter()
                .map(|name| name.name)
                .collect()
        };
        assert_eq!(
            names(MarriageAugury::DoubleBright),
            ["雙春兼閏月", "双春年"]
        );
        assert!(names(MarriageAugury::Widow).contains(&"寡婦年"));
        assert!(names(MarriageAugury::Widow).contains(&"盲年"));
        assert!(MarriageAugury::Blind.chinese_names().is_empty());
        assert!(MarriageAugury::Bright.chinese_names().is_empty());
    }

    #[test]
    fn the_age_counts_turn_at_their_own_boundaries() {
        let day = |year, month, day| gregorian::to_fixed_saturating(year, month, day);
        // 立春 on 4 February 2009 and 2010 (`scmp-double-spring-2009`).
        assert_eq!(lichun_day(2009), Ok(day(2009, 2, 4)));
        assert_eq!(lichun_day(2010), Ok(day(2010, 2, 4)));
        let born = day(2009, 6, 1);
        assert_eq!(reckoned_age_at_lichun(born, day(2010, 2, 3)), Ok(Some(1)));
        assert_eq!(reckoned_age_at_lichun(born, day(2010, 2, 4)), Ok(Some(2)));
        assert_eq!(reckoned_age_at_lichun(born, day(2009, 5, 31)), Ok(None));
        // The New Year count turns ten days later, at 14 February 2010.
        let birth = ChineseCalendar.from_fixed(born).expect("in range");
        assert_eq!(reckoned_age(birth, day(2010, 2, 13)), Ok(Some(1)));
        assert_eq!(reckoned_age(birth, day(2010, 2, 14)), Ok(Some(2)));
        // Born on 31 December, two the next day; born on 1 January, still
        // one that day (`wikipedia-en-east-asian-age-reckoning`,
        // `wikipedia-zh-xusui`).
        let eve = day(2000, 12, 31);
        assert_eq!(reckoned_age_at_new_year_day(eve, eve), Some(1));
        assert_eq!(reckoned_age_at_new_year_day(eve, day(2001, 1, 1)), Some(2));
        let new_year = day(2001, 1, 1);
        assert_eq!(reckoned_age_at_new_year_day(new_year, new_year), Some(1));
        assert_eq!(year_age(new_year, new_year), Some(0));
        assert_eq!(year_age(eve, day(2001, 1, 1)), Some(1));
        assert_eq!(year_age(eve, day(2000, 12, 30)), None);
        // (Current Year − Birth Year) + 1.
        assert_eq!(
            reckoned_age_at_new_year_day(day(1990, 7, 1), day(2026, 9, 29)),
            Some(37)
        );
    }

    #[test]
    fn chinese_new_year_2024_was_the_tenth_of_february() {
        // A published anchor: the Year of the Wood Dragon began on
        // 2024-02-10.
        let rd = gregorian::to_fixed_saturating(2024, 2, 10);
        assert_eq!(new_year(4_661), Ok(rd));
        assert_eq!(
            ChineseCalendar.from_fixed(rd),
            Ok(LunisolarDate::new(4_661, Month::regular(1), 1))
        );
    }

    #[test]
    fn new_year_refuses_the_ends_of_i64_instead_of_overflowing() {
        for year in [i64::MIN, i64::MAX] {
            assert_eq!(new_year(year), Err(CalendarError::YearOutOfRange));
            assert_eq!(
                ChineseCalendar.is_leap_year(year),
                Err(CalendarError::YearOutOfRange)
            );
            assert_eq!(
                ChineseCalendar.to_fixed(LunisolarDate::new(year, Month::regular(1), 1)),
                Err(CalendarError::YearOutOfRange)
            );
        }
    }

    #[test]
    fn the_year_that_began_in_2024_is_jia_chen_the_wood_dragon() {
        let cycle = PARAMETERS.sexagenary_year(4_661);
        assert_eq!(cycle.stem_name(), "jia");
        assert_eq!(cycle.branch_name(), "chen");
        assert_eq!(cycle.zodiac_animal(), "dragon");
        assert_eq!(cycle.five_phase(), hc_calendar::cycle::FivePhase::Wood);
        // 1984 was the last jiǎ-zǐ year, the start of a sexagenary cycle.
        assert_eq!(PARAMETERS.sexagenary_year(4_621).index(), 0);
    }

    #[test]
    fn twenty_twenty_three_had_a_leap_second_month() {
        // A published anchor: 閏二月 of 2023 began on 2023-03-22.
        assert_eq!(PARAMETERS.leap_month(4_660), Ok(Some(2)));
        assert_eq!(
            ChineseCalendar.to_fixed(LunisolarDate::new(4_660, Month::leap(2), 1)),
            Ok(gregorian::to_fixed_saturating(2023, 3, 22))
        );
        assert_eq!(PARAMETERS.months_in_year(4_660), Ok(13));
        assert_eq!(PARAMETERS.is_leap_year(4_660), Ok(true));
    }

    #[test]
    fn the_fourth_month_of_1906_began_on_the_day_the_almanac_gave() {
        // 光緒三十二年: 三月 of thirty days ending on 丁酉, 23 April, and
        // 四月初一 on 戊戌, 24 April, in the Purple Mountain Observatory's
        // table from the 時憲書 (`pmo-calendar-1900-2025`); 閏四月 from
        // 23 May, 五月 from 22 June.
        let year = 4_543;
        assert_eq!(
            new_year(year),
            Ok(gregorian::to_fixed_saturating(1906, 1, 25))
        );
        assert_eq!(
            ChineseCalendar.to_fixed(LunisolarDate::new(year, Month::regular(4), 1)),
            Ok(gregorian::to_fixed_saturating(1906, 4, 24))
        );
        assert_eq!(
            ChineseCalendar.from_fixed(gregorian::to_fixed_saturating(1906, 4, 23)),
            Ok(LunisolarDate::new(year, Month::regular(3), 30))
        );
        assert_eq!(PARAMETERS.days_in_month(year, Month::regular(3)), Some(30));
        assert_eq!(PARAMETERS.days_in_month(year, Month::regular(4)), Some(29));
        assert_eq!(PARAMETERS.leap_month(year), Ok(Some(4)));
        assert_eq!(
            ChineseCalendar.to_fixed(LunisolarDate::new(year, Month::leap(4), 1)),
            Ok(gregorian::to_fixed_saturating(1906, 5, 23))
        );
        assert_eq!(
            PARAMETERS
                .sexagenary_day(gregorian::to_fixed_saturating(1906, 4, 24))
                .index(),
            34,
            "戊戌 is the 35th day of the cycle"
        );
    }

    /// The rules alone, at the same meridians.
    static RULES: LunisolarParameters = LunisolarParameters {
        month_start_corrections: &[],
        major_term_corrections: &[],
        ..PARAMETERS
    };

    /// The sexagenary name of a day, as the Veritable Records write it.
    fn cjk_day(rd: Rd) -> String {
        const STEMS: [char; 10] = ['甲', '乙', '丙', '丁', '戊', '己', '庚', '辛', '壬', '癸'];
        const BRANCHES: [char; 12] = [
            '子', '丑', '寅', '卯', '辰', '巳', '午', '未', '申', '酉', '戌', '亥',
        ];
        let day = PARAMETERS.sexagenary_day(rd);
        [
            STEMS[day.stem_index() as usize],
            BRANCHES[day.branch_index() as usize],
        ]
        .iter()
        .collect()
    }

    #[test]
    fn the_almanac_corrections_are_live_and_move_a_day_at_most() {
        // Each correction names a first day the rules really give, so an
        // entry cannot outlive a change to the astronomy unnoticed, and none
        // moves further than the engine's search allows.
        for pair in ALMANAC_CORRECTIONS.windows(2) {
            assert!(pair[0].computed < pair[1].computed, "{pair:?} out of order");
        }
        for correction in &ALMANAC_CORRECTIONS {
            assert_eq!(
                RULES.new_moon_on_or_after(correction.computed),
                correction.computed
            );
            assert_ne!(correction.computed, correction.promulgated);
            assert!(
                (correction.promulgated.0 - correction.computed.0).abs()
                    <= MonthStartCorrection::MAX_SHIFT
            );
            // The corrected calendar has a month on the almanac's day and
            // none on the rules'.
            assert_eq!(
                PARAMETERS.new_moon_on_or_after(correction.computed.min(correction.promulgated)),
                correction.promulgated
            );
            assert_eq!(
                PARAMETERS.new_moon_before(Rd(correction.promulgated.0 + 1)),
                correction.promulgated
            );
            assert!(!correction.source.is_empty());
        }
        // Without the table the rules give 23 April 1906.
        assert_eq!(
            RULES.new_moon_on_or_after(gregorian::to_fixed_saturating(1906, 4, 20)),
            gregorian::to_fixed_saturating(1906, 4, 23)
        );
    }

    #[test]
    fn the_almanac_term_corrections_are_live_and_move_a_day_at_most() {
        for pair in ALMANAC_TERM_CORRECTIONS.windows(2) {
            assert!(pair[0].computed < pair[1].computed, "{pair:?} out of order");
        }
        for correction in &ALMANAC_TERM_CORRECTIONS {
            let term = correction.term as i64;
            let before = (term + 10).rem_euclid(12) + 1;
            // The rules put the term on the computed day: the index turns
            // at the midnight that ends it.
            assert_eq!(RULES.major_solar_term(correction.computed), before);
            assert_eq!(RULES.major_solar_term(Rd(correction.computed.0 + 1)), term);
            assert!(
                (correction.promulgated.0 - correction.computed.0).abs()
                    == MajorTermCorrection::MAX_SHIFT
            );
            // The corrected calendar turns at the midnight that ends the
            // almanac's day instead.
            assert_eq!(PARAMETERS.major_solar_term(correction.promulgated), before);
            assert_eq!(
                PARAMETERS.major_solar_term(Rd(correction.promulgated.0 + 1)),
                term
            );
            assert!(!correction.source.is_empty());
        }
    }

    /// A Gregorian year, month and day.
    type Ymd = (i64, u8, u8);

    /// A Chinese year, a month, its sexagenary first day and that day.
    type VeritableFirstDay = (i64, u8, &'static str, Ymd);

    /// A Chinese year, its leap month, the sexagenary first days of the
    /// month before and of the leap month, the leap month's first day, and
    /// the month the rules make leap instead.
    type VeritableLeapMonth = (i64, u8, &'static str, &'static str, Ymd, u8);

    /// The first day of each month that the rules put elsewhere, 1645–1899,
    /// as the month's opening line in the Veritable Records gives it
    /// (`qing-shilu`): the Chinese year, the month, its sexagenary first
    /// day, and that day in the Gregorian calendar, which is Liu's.
    const VERITABLE_FIRST_DAYS: [VeritableFirstDay; 28] = [
        (4_289, 9, "庚午", (1652, 10, 3)),
        (4_290, 8, "甲子", (1653, 9, 22)),
        (4_310, 10, "丁酉", (1673, 11, 9)),
        (4_323, 4, "乙酉", (1686, 4, 23)),
        (4_324, 2, "己酉", (1687, 3, 13)),
        (4_329, 5, "庚戌", (1692, 6, 15)),
        (4_330, 3, "乙巳", (1693, 4, 6)),
        (4_341, 10, "戊辰", (1704, 10, 29)),
        (4_345, 2, "戊寅", (1708, 2, 21)),
        (4_350, 11, "乙巳", (1713, 12, 18)),
        (4_352, 2, "戊辰", (1715, 3, 6)),
        (4_365, 7, "庚戌", (1728, 8, 6)),
        (4_368, 5, "癸亥", (1731, 6, 5)),
        (4_391, 8, "戊申", (1754, 9, 17)),
        (4_426, 9, "甲申", (1789, 10, 19)),
        (4_431, 11, "乙酉", (1794, 11, 23)),
        (4_450, 4, "戊戌", (1813, 5, 1)),
        (4_454, 9, "壬寅", (1817, 10, 11)),
        (4_457, 11, "甲寅", (1820, 12, 6)),
        (4_460, 4, "庚子", (1823, 5, 11)),
        (4_478, 12, "庚辰", (1842, 1, 11)),
        (4_479, 10, "丙子", (1842, 11, 3)),
        (4_486, 8, "丙寅", (1849, 9, 17)),
        (4_493, 11, "乙卯", (1856, 11, 28)),
        (4_498, 10, "丙辰", (1861, 11, 3)),
        (4_506, 4, "癸卯", (1869, 5, 12)),
        (4_517, 10, "丙申", (1880, 11, 3)),
        (4_524, 3, "己丑", (1887, 3, 25)),
    ];

    #[test]
    fn each_month_the_almanac_moved_begins_on_the_veritable_records_day() {
        for (year, ordinal, name, (y, m, d)) in VERITABLE_FIRST_DAYS {
            let first = gregorian::to_fixed_saturating(y, m, d);
            assert_eq!(cjk_day(first), name, "{y}-{m}-{d}");
            assert_eq!(
                ChineseCalendar.from_fixed(first),
                Ok(LunisolarDate::new(year, Month::regular(ordinal), 1)),
                "{y}-{m}-{d}"
            );
            // The rules begin the month a day away, and the correction that
            // moves it names the record.
            assert_ne!(RULES.new_moon_before(Rd(first.0 + 1)), first);
            let correction = ALMANAC_CORRECTIONS
                .iter()
                .find(|correction| correction.promulgated == first)
                .expect("a correction for the month");
            assert!(correction.source.contains(&format!("{name}朔")));
        }
        // With the one month of 1906, that is the whole table.
        assert_eq!(VERITABLE_FIRST_DAYS.len() + 1, ALMANAC_CORRECTIONS.len());
    }

    /// The five leap months that the rules put in another lunation: the
    /// Chinese year, the leap month, the sexagenary first day of the month
    /// before it and of the leap month in the Veritable Records
    /// (`qing-shilu`), and the leap month's first day, which is Liu's; and
    /// where the rules put the leap month instead.
    const VERITABLE_LEAP_MONTHS: [VeritableLeapMonth; 5] = [
        (4_282, 6, "壬子", "辛巳", (1645, 7, 23), 5),
        (4_288, 2, "己卯", "戊申", (1651, 3, 21), 1),
        (4_298, 7, "戊申", "戊寅", (1661, 8, 25), 8),
        (4_364, 3, "戊子", "丁巳", (1727, 4, 21), 2),
        (4_442, 6, "癸丑", "壬午", (1805, 7, 26), 7),
    ];

    #[test]
    fn the_leap_months_the_almanac_moved_are_where_the_veritable_records_have_them() {
        for (year, leap, before, name, (y, m, d), by_the_rules) in VERITABLE_LEAP_MONTHS {
            let first = gregorian::to_fixed_saturating(y, m, d);
            assert_eq!(PARAMETERS.leap_month(year), Ok(Some(leap)), "{year}");
            assert_eq!(RULES.leap_month(year), Ok(Some(by_the_rules)), "{year}");
            assert_eq!(
                ChineseCalendar.to_fixed(LunisolarDate::new(year, Month::leap(leap), 1)),
                Ok(first),
                "{year}"
            );
            assert_eq!(cjk_day(first), name, "{year}");
            let regular = ChineseCalendar
                .to_fixed(LunisolarDate::new(year, Month::regular(leap), 1))
                .expect("the month before the leap month");
            assert_eq!(cjk_day(regular), before, "{year}");
            let correction = ALMANAC_TERM_CORRECTIONS
                .iter()
                .find(|correction| gregorian::year_from_fixed(correction.computed) == y)
                .expect("a term correction for the year");
            assert!(correction.source.contains(&format!("{name}朔")), "{year}");
        }
    }

    #[test]
    fn other_published_new_years_are_reproduced() {
        // Dates in general circulation for the start of the Chinese year.
        for (year, gregorian) in [
            (4_537, (1900, 1, 31)),
            (4_637, (2000, 2, 5)),
            (4_657, (2020, 1, 25)),
            (4_658, (2021, 2, 12)),
            (4_659, (2022, 2, 1)),
            (4_660, (2023, 1, 22)),
            (4_661, (2024, 2, 10)),
            (4_662, (2025, 1, 29)),
            (4_663, (2026, 2, 17)),
        ] {
            assert_eq!(
                new_year(year),
                Ok(gregorian::to_fixed_saturating(
                    gregorian.0,
                    gregorian.1,
                    gregorian.2
                )),
                "year {year}"
            );
        }
    }

    #[test]
    fn chinese_new_year_always_falls_between_january_twenty_first_and_february_twenty_first() {
        for year in 4_570..4_780i64 {
            let rd = new_year(year).expect("in range");
            let (_, month, day) = gregorian::ymd(rd);
            let within = (month == 1 && day >= 21) || (month == 2 && day <= 21);
            assert!(within, "year {year} began on {month}-{day}");
        }
    }

    #[test]
    fn the_calendar_round_trips_over_six_thousand_modern_days() {
        let calendar = ChineseCalendar;
        let start = gregorian::to_fixed_saturating(2000, 1, 1);
        // Every day in a release build, every eleventh in a debug one.
        for offset in (0..6_000i64).step_by(crate::sweep_stride(11)) {
            let rd = Rd(start.0 + offset);
            let date = calendar.from_fixed(rd).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(rd), "RD {rd}");
        }
    }

    /// The Qing years, where the engine's corrections live: every day of
    /// 1645–1911 round-trips, in a release build; a debug build takes every
    /// sixty-first day, and every new year, every day a correction moves a
    /// first day or a term from or to, and the day before each.
    #[test]
    fn the_calendar_round_trips_over_the_qing_years() {
        let calendar = ChineseCalendar;
        let years: Vec<Rd> = (4_281..=4_549)
            .filter_map(|year| new_year(year).ok())
            .collect();
        let corrections = ALMANAC_CORRECTIONS
            .iter()
            .flat_map(|c| [c.computed, c.promulgated])
            .chain(
                ALMANAC_TERM_CORRECTIONS
                    .iter()
                    .flat_map(|c| [c.computed, c.promulgated]),
            );
        let boundaries: Vec<i64> = years
            .iter()
            .copied()
            .chain(corrections)
            .map(|rd| rd.0)
            .collect();
        for rd in crate::sweep_days(EARLIEST.0, LAST_CIVIL.0, 61, boundaries) {
            let rd = Rd(rd);
            let date = calendar.from_fixed(rd).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(rd), "RD {rd}");
        }
        for rd in years {
            let date = calendar.from_fixed(rd).expect("in range");
            assert_eq!((date.month, date.day), (Month::regular(1), 1), "RD {rd}");
        }
    }

    /// The Chinese Wikipedia dates the change to 120°E to 1928 (民國十七年;
    /// `wikipedia-zh-nongli`) and Reingold and Dershowitz to 1929; switching in either year gives
    /// the same day for every date of 1926–1930, so the disagreement moves
    /// nothing. See `docs/systems/solar-terms-and-pentads.md`.
    #[test]
    fn the_1928_and_1929_readings_of_the_meridian_change_agree() {
        static FROM_1928: [MeridianEra; 2] = [
            MERIDIANS[0],
            MeridianEra::from_zone(1928, 8.0, "the 120°E standard zone"),
        ];
        static PARAMETERS_1928: LunisolarParameters = LunisolarParameters {
            meridians: &FROM_1928,
            ..PARAMETERS
        };
        let from_1928 = LunisolarCalendar::new(&PARAMETERS_1928);
        // Every day in a release build, every fifth in a debug one: a month
        // that began a day apart would differ on all of its days.
        for rd in (gregorian::to_fixed_saturating(1926, 1, 1).0
            ..gregorian::to_fixed_saturating(1931, 1, 1).0)
            .step_by(crate::sweep_stride(5))
        {
            assert_eq!(
                ENGINE.from_fixed(Rd(rd)),
                from_1928.from_fixed(Rd(rd)),
                "RD {rd}"
            );
        }
    }

    /// The old Beijing meridian: 116°25′E, Reingold and Dershowitz's 1397⁄180
    /// hours, which this calendar uses, or 116°23′E, the Hong Kong Space
    /// Museum's figure as the Chinese Wikipedia quotes it
    /// (`wikipedia-zh-nongli`). Eight seconds of time apart, the rules at
    /// the two begin one month of 1645–1929 on different days: the second
    /// month of 4324, on 14 March 1687 at 116°25′ and 13 March at 116°23′.
    /// The almanac had 13 March — 康熙二十六年二月己酉朔 in the Veritable
    /// Records (`qing-shilu`) — and so does this calendar, by its
    /// correction; a release build checks that the month is the only one.
    #[test]
    fn the_two_readings_of_the_beijing_meridian_differ_once() {
        static AT_116_23: [MeridianEra; 2] = [
            MeridianEra::from_longitude(i64::MIN / 4, 116.383_333_333_333_33, "116°23′E"),
            MERIDIANS[1],
        ];
        static RULES_116_23: LunisolarParameters = LunisolarParameters {
            meridians: &AT_116_23,
            ..RULES
        };
        let rules = LunisolarCalendar::new(&RULES);
        let other = LunisolarCalendar::new(&RULES_116_23);
        let march_13 = gregorian::to_fixed_saturating(1687, 3, 13);
        let march_14 = Rd(march_13.0 + 1);
        let first = |calendar: LunisolarCalendar, rd: Rd| {
            let date = calendar.from_fixed(rd).expect("in range");
            (date.year, date.month.ordinal, date.day)
        };
        assert_eq!(first(rules, march_14), (4324, 2, 1));
        assert_eq!(first(other, march_13), (4324, 2, 1));
        assert_eq!(first(ENGINE, march_13), (4324, 2, 1));
        if cfg!(debug_assertions) {
            return;
        }
        let differing = (EARLIEST.0..gregorian::to_fixed_saturating(1930, 1, 1).0)
            .filter(|rd| rules.from_fixed(Rd(*rd)) != other.from_fixed(Rd(*rd)))
            .count();
        // The thirty days of that one month, 13 March to 11 April 1687.
        assert_eq!(differing, 30);
    }

    #[test]
    fn the_calendar_round_trips_across_the_1929_meridian_change() {
        let calendar = ChineseCalendar;
        let start = gregorian::to_fixed_saturating(1925, 1, 1);
        // Every day in a release build, every fifth in a debug one.
        for offset in (0..3_000i64).step_by(crate::sweep_stride(5)) {
            let rd = Rd(start.0 + offset);
            let date = calendar.from_fixed(rd).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(rd), "RD {rd}");
        }
    }

    #[test]
    fn the_calendar_round_trips_at_both_ends_of_its_range() {
        let calendar = ChineseCalendar;
        for start in [EARLIEST.0, LATEST.0 - 2_000] {
            // Every day in a release build, every eleventh in a debug one.
            for offset in (0..2_000i64).step_by(crate::sweep_stride(11)) {
                let rd = Rd(start + offset);
                let date = calendar.from_fixed(rd).expect("in range");
                assert_eq!(calendar.to_fixed(date), Ok(rd), "RD {rd}");
            }
        }
    }

    #[test]
    fn the_range_is_refused_rather_than_extrapolated() {
        let calendar = ChineseCalendar;
        assert_eq!(
            calendar.from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            calendar.from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(new_year(4_000), Err(CalendarError::YearOutOfRange));
    }

    #[test]
    fn months_are_twenty_nine_or_thirty_days_and_years_twelve_or_thirteen_months() {
        for year in 4_630..4_700i64 {
            let months = PARAMETERS.months_in_year(year).expect("in range");
            assert!(months == 12 || months == 13, "year {year}");
            let start = new_year(year).expect("in range");
            let end = new_year(year + 1).expect("in range");
            let mut cursor = start;
            let mut counted = 0u8;
            let mut ordinals = [0u8; 13];
            while cursor < end {
                let date = ChineseCalendar.from_fixed(cursor).expect("in range");
                assert_eq!(date.day, 1);
                let length = PARAMETERS
                    .days_in_month(date.year, date.month)
                    .expect("the month exists") as i64;
                assert!((29..=30).contains(&length), "year {year} month {length}");
                ordinals[counted as usize] = date.month.ordinal;
                counted += 1;
                cursor = Rd(cursor.0 + length);
            }
            assert_eq!(cursor, end);
            assert_eq!(counted, months, "year {year}");
            // The months run 1..=12 in order, with the leap month repeating
            // the ordinal before it.
            assert_eq!(ordinals[0], 1);
        }
    }

    #[test]
    fn every_leap_month_immediately_follows_the_month_it_repeats() {
        for year in 4_600..4_700i64 {
            let Some(ordinal) = PARAMETERS.leap_month(year).expect("in range") else {
                continue;
            };
            let regular = ChineseCalendar
                .to_fixed(LunisolarDate::new(year, Month::regular(ordinal), 1))
                .expect("exists");
            let leap = ChineseCalendar
                .to_fixed(LunisolarDate::new(year, Month::leap(ordinal), 1))
                .expect("exists");
            let length = PARAMETERS
                .days_in_month(year, Month::regular(ordinal))
                .expect("exists") as i64;
            assert_eq!(leap.0 - regular.0, length, "year {year}");
        }
    }

    #[test]
    fn leap_months_are_rare_and_never_the_first_month() {
        let mut leaps = 0;
        for year in 4_600..4_700i64 {
            if let Some(ordinal) = PARAMETERS.leap_month(year).expect("in range") {
                leaps += 1;
                assert_ne!(ordinal, 1, "year {year} had a leap first month");
            }
        }
        // Seven leap years in nineteen, so about 37 in a hundred.
        assert!(
            (30..=45).contains(&leaps),
            "{leaps} leap years in a century"
        );
    }

    #[test]
    fn the_generic_interface_carries_the_cycle_and_its_position() {
        let calendar = ChineseCalendar;
        let date = calendar
            .from_fixed(gregorian::to_fixed_saturating(2024, 2, 10))
            .expect("in range");
        let fields = calendar.to_fields(date).expect("describable");
        assert_eq!(fields.year, 4_661);
        // 4661 = 60 * 77 + 41, so it is position 41 of cycle 78.
        assert_eq!(fields.extra.get("cycle"), Some(78));
        assert_eq!(fields.extra.get("year_of_cycle"), Some(41));
        assert_eq!(calendar.from_fields(&fields), Ok(date));
    }

    #[test]
    fn the_metadata_admits_the_calendar_is_astronomical() {
        let meta = ChineseCalendar.meta();
        assert_eq!(meta.id, CalendarId("chinese"));
        assert!(meta.has_leap_months);
        assert!(meta.is_astronomical);
        assert_eq!(meta.earliest, Some(EARLIEST));
        assert_eq!(meta.latest, Some(LATEST));
    }

    #[test]
    fn a_child_born_in_june_2000_turns_thirteen_at_the_new_year_of_2012() {
        // Wikipedia, "East Asian age reckoning", § People's Republic of
        // China: one suì at birth, one more at each lunar new year, and a
        // child born in June 2000, a dragon year, 13 suì from the lunar new
        // year of 2012.
        let birth = ChineseCalendar
            .from_fixed(gregorian::to_fixed_saturating(2000, 6, 15))
            .expect("in range");
        assert_eq!(birth.year, 4_637);
        assert_eq!(PARAMETERS.sexagenary_year(4_637).zodiac_animal(), "dragon");
        let new_year_2012 = new_year(4_649).expect("in range");
        assert_eq!(new_year_2012, gregorian::to_fixed_saturating(2012, 1, 23));
        assert_eq!(reckoned_age(birth, new_year_2012), Ok(Some(13)));
        assert_eq!(reckoned_age(birth, Rd(new_year_2012.0 - 1)), Ok(Some(12)));
        assert_eq!(
            reckoned_age(birth, gregorian::to_fixed_saturating(2000, 6, 15)),
            Ok(Some(1))
        );
        assert_eq!(
            reckoned_age(birth, gregorian::to_fixed_saturating(2000, 6, 14)),
            Ok(None)
        );
    }

    #[test]
    fn a_child_born_on_new_years_eve_is_two_the_next_day() {
        let eve = Rd(new_year(4_661).expect("in range").0 - 1);
        let birth = ChineseCalendar.from_fixed(eve).expect("in range");
        assert_eq!(reckoned_age(birth, eve), Ok(Some(1)));
        assert_eq!(reckoned_age(birth, Rd(eve.0 + 1)), Ok(Some(2)));
        assert!(reckoned_age(LunisolarDate::new(4_661, Month::regular(1), 31), eve).is_err());
    }

    #[test]
    fn the_published_widow_and_double_spring_years_are_reproduced() {
        // The Year of the Dragon that began on 10 February 2024 is a Widow
        // Year, "lacking Spring Commences" (South China Morning Post,
        // 3 February 2024).
        assert_eq!(marriage_augury(4_661), Ok(MarriageAugury::Widow));
        // The lunar year that began on 26 January 2009 holds two 立春, on
        // 4 February 2009 and 4 February 2010 (South China Morning Post,
        // 25 January 2009).
        assert_eq!(
            new_year(4_646),
            Ok(gregorian::to_fixed_saturating(2009, 1, 26))
        );
        assert_eq!(marriage_augury(4_646), Ok(MarriageAugury::DoubleBright));
    }

    #[test]
    fn a_year_with_two_lichun_has_thirteen_months_and_one_with_none_twelve() {
        // Two 立春 are a tropical year apart and none leaves a gap of one,
        // so the first can only happen in a year longer than 365 days and
        // the second only in a shorter one. The augury and the year length
        // are computed independently, from the Sun and from the Moon.
        let mut seen = [0u32; 4];
        for year in 4_290..4_780i64 {
            let augury = marriage_augury(year).expect("in range");
            let length =
                new_year(year + 1).expect("in range").0 - new_year(year).expect("in range").0;
            match augury {
                MarriageAugury::DoubleBright => assert!(length > 366, "{year}"),
                MarriageAugury::Widow => assert!(length < 365, "{year}"),
                _ => {}
            }
            seen[augury as usize] += 1;
            // What ends one year is what the next does not start with.
            let next = marriage_augury(year + 1).expect("in range");
            assert_eq!(augury.lichun_at_end(), !next.lichun_at_start(), "{year}");
        }
        assert!(seen.iter().all(|count| *count > 0), "{seen:?}");
    }
}
