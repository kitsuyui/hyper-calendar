//! China's lunisolar calendars from 104 BCE to 597 CE, each on its own
//! arithmetic.
//!
//! The system is written up in `docs/systems/chinese-historical-lunisolar.md`
//! in the repository: what each system was, its constants and where each is
//! printed, how the epoch is read from the text, a month worked by hand,
//! what each was measured against and what is not carried. This page
//! states the code's facts.
//!
//! Between the Han reform of 104 BCE and the 麟德曆 of 665 every Chinese
//! calendar kept its months at the *mean* conjunction and its twelve
//! zhōngqì at equal twelfths of its own year (平朔 and 平氣), and made a
//! month with no zhōngqì the leap month. The whole calendar is therefore
//! two linear functions of the day count, a month length and a year
//! length, each a ratio of two integers that the system's treatise prints,
//! phased at an epoch at which a new moon and a winter solstice (or, for
//! one system, a *yǔshuǐ*) fall together at midnight. Each system here is
//! those two ratios, its epoch and the span in which it ran, on the
//! [`LunisolarParameters`] engine the Japanese systems run on, with
//! [`ConjunctionMode::Mean`] and [`SolarTermMode::Mean`] and no 進朔.
//!
//! | Module | System | Ran | 朔 (days) | 歲 (days) |
//! |---|---|---|---|---|
//! | [`taichu`] | 太初曆, 三統曆 | −103–85 | 29 43/81 | 365 385/1539 |
//! | [`sifen`] | 四分曆 | 85–264 | 29 499/940 | 365 1/4 |
//! | [`qianxiang`] | 乾象曆 | 223–281 | 29 773/1457 | 365 145/589 |
//! | [`jingchu`] | 景初曆 | 240–445 | 29 2419/4559 | 365 455/1843 |
//! | [`yuanjia`] | 元嘉曆 | 445–510 | 29 399/752 | 365 75/304 |
//! | [`daming`] | 大明曆 | 510–590 | 29 2090/3939 | 365 9589/39491 |
//! | [`xinghe`] | 興和曆 | 540–551 | 29 110647/208530 | 365 4117/16860 |
//! | [`tianhe`] | 天和曆 | 566–579 | 29 153991/290160 | 365 5731/23460 |
//! | [`kaihuang`] | 開皇曆 | 584–597 | 29 96529/181920 | 365 25063/102960 |
//! | [`sanji`] | 三紀甲子元曆 | 384–418 | 29 3247/6063 | 365 605/2451 |
//! | [`zhengguang`] | 正光曆 | 523–559 | 29 39769/74952 | 365 1477/6060 |
//!
//! # The epoch
//!
//! Each treatise counts years from an *epoch* (上元) at which the new moon
//! and the winter solstice coincide at midnight on a 甲子 day, and
//! numbers the cycles (紀 or 蔀) after it by their first day. The code
//! reads an epoch from the treatise's own statement of where it stands: the
//! 後漢書 puts the 蔀首 of the 四分曆 on a 甲子 day, in the winter of the
//! 庚辰 year, the forty-fifth of the Han; the 宋書 says of the 元嘉曆 that
//! 元嘉二十年 is the 231st year of its 甲午紀. The epoch's day is the one
//! whose sexagenary name and distance from the winter solstice of that
//! year agree with the treatise, near 25 December. Nothing is fitted to a
//! table of months. The day each epoch falls on is given in each module's
//! documentation and checked in its tests.
//!
//! # What is carried
//!
//! The days of the months, their lengths and which of them is the leap
//! month, over the span each system ran. The month numbering is the
//! 夏正's, the first month the 寅 month in which 雨水 falls, throughout: the
//! numbering of 9–23 under the Xin and of 237–239 under the Wei, which
//! began the year a month earlier, is not carried, and the Wei span of
//! [`jingchu`] begins after it.

use hc_calendar::{Calendar, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd};
use hc_calendars_solar::julian;

use crate::lunisolar::{
    CHINESE_EPOCH, ConjunctionMode, LunisolarCalendar, LunisolarDate, LunisolarParameters,
    MeanMotionModel, MeridianEra, SolarTermMode,
};

/// How far the year number falls below the continuous Chinese count, as in
/// [`crate::japanese_historical::YEAR_OFFSET`]: the number is the
/// proleptic Gregorian year in which the lunisolar year begins.
pub const YEAR_OFFSET: i64 = -2_637;

/// The arithmetic has no meridian: months and terms are counted in the
/// system's own days from an epoch, and a day is the day by that count.
/// The engine wants a meridian era and reads it only for the modern
/// astronomy these systems do not use, so the table states that.
pub static MERIDIANS: [MeridianEra; 1] = [MeridianEra::from_zone(
    i64::MIN / 4,
    0.0,
    "none: the arithmetic counts the system's own days from its epoch",
)];

/// How much later than its exact value the epoch is put, in days.
///
/// The conjunctions and the zhōngqì are exact ratios, and some of them
/// fall on the instant of midnight, which the system puts on the day it
/// begins: a month whose conjunction is at midnight begins that
/// day, and a zhōngqì at midnight is that day's. A binary floating-point
/// product cannot be trusted to land on the right side of such a
/// boundary, so the epoch carries a millionth of a day (86 ms), which
/// moves no instant across a boundary except those that lie on one: over
/// the span of every system here the nearest instant that does not is more
/// than 9 × 10⁻⁵ of a day from midnight, and a test compares every month
/// with an integer computation of the same ratios.
pub const BOUNDARY_NUDGE: f64 = 1.0e-6;

/// A Julian date as a fixed day.
const fn julian_day(year: i64, month: u8, day: u8) -> Rd {
    match julian::to_fixed(year, month, day) {
        Ok(rd) => rd,
        Err(_) => Rd(0),
    }
}

/// The delegating calendar and the parameter set every system exposes.
macro_rules! system {
    (
        $(#[$doc:meta])*
        calendar $calendar:ident,
        id $id:literal,
        english $english:literal,
        earliest $earliest:expr,
        latest $latest:expr,
        usage $usage:literal,
        model $model:expr $(,)?
    ) => {
        /// The identifier.
        pub const ID: CalendarId = CalendarId($id);

        /// The first day of the span the system ran.
        pub const EARLIEST: Rd = $earliest;

        /// The last day of the span the system ran.
        pub const LATEST: Rd = $latest;

        /// Where the span comes from.
        pub const USAGE_SOURCE: &str = $usage;

        /// The period constants and the epoch.
        pub const MODEL: MeanMotionModel = $model;

        /// The engine's parameters.
        pub static PARAMETERS: LunisolarParameters = LunisolarParameters {
            id: ID,
            english_name: $english,
            native_locales: &["zh"],
            meridians: &MERIDIANS,
            epoch: CHINESE_EPOCH,
            year_offset: YEAR_OFFSET,
            solar_term_mode: SolarTermMode::Mean,
            month_start_corrections: &[],
            major_term_corrections: &[],
            mean_motion: Some(MODEL),
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
        };

        /// The engine configured as this calendar.
        pub const ENGINE: LunisolarCalendar = LunisolarCalendar::new(&PARAMETERS);

        $(#[$doc])*
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
        pub struct $calendar;

        impl Calendar for $calendar {
            type Date = LunisolarDate;

            fn usage(&self) -> hc_calendar::Usage {
                hc_calendar::Usage::between(EARLIEST, LATEST, USAGE_SOURCE)
            }

            fn meta(&self) -> CalendarMeta {
                ENGINE.meta()
            }

            fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
                ENGINE.cycles()
            }

            fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
                PARAMETERS.is_leap_year(year)
            }

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

        /// The fixed day of the lunisolar new year of `year`.
        ///
        /// # Errors
        ///
        /// Returns [`hc_calendar::CalendarError::YearOutOfRange`] outside
        /// the span the system ran.
        pub fn new_year(year: i64) -> CalendarResult<Rd> {
            PARAMETERS.new_year(year)
        }
    };
}

/// A model of mean motions: the two period ratios, the epoch of the new
/// moon and of the winter solstice, both at midnight, and nothing else.
macro_rules! mean_model {
    (
        month $month_num:literal / $month_den:literal,
        year $year_num:literal / $year_den:literal,
        epoch $epoch:expr,
        solstice_from_epoch $solstice_shift:expr $(,)?
    ) => {
        MeanMotionModel {
            tropical_year: $year_num as f64 / $year_den as f64,
            synodic_month: $month_num as f64 / $month_den as f64,
            // The mean conjunction needs no anomaly, so none is carried.
            anomalistic_month: 0.0,
            solstice_epoch: $epoch.0 as f64 + $solstice_shift + BOUNDARY_NUDGE,
            conjunction_epoch: $epoch.0 as f64 + BOUNDARY_NUDGE,
            perigee_epoch: 0.0,
            conjunction_mode: ConjunctionMode::Mean,
            solar_equation_days: 0.0,
            lunar_equation_days: 0.0,
            advance_limit: None,
            seasonal_advance: None,
            eclipse_exception: None,
        }
    };
}

/// The 太初曆 of the Han and its revision, the 三統曆.
///
/// 太初元年 is the year 104 BCE: the reform of 五月 that year put the
/// calendar of Dèng Píng and Luòxià Hóng in place of the 顓頊曆 and began
/// the year with the 寅 month. 三統曆 is Liú Xīn's revision of the text
/// for the 漢書, with the same numbers.
///
/// | | |
/// |---|---|
/// | 日法 | 81 |
/// | 朔 | 2 392/81 = 29 43/81 days |
/// | 歲 | 365 385/1 539 days (一統 = 1 539 years = 81 章) |
/// | Epoch | 25 December 105 BCE (Julian), 甲子, 朔旦冬至 at midnight |
pub mod taichu {
    use super::*;

    system! {
        /// 太初曆: the Han calendar from 104 BCE to 85 CE.
        calendar TaichuCalendar,
        id "chinese-taichu",
        english "Chinese Taichu (Han, 104 BCE – 85 CE)",
        earliest julian_day(-103, 6, 20),
        latest julian_day(85, 3, 17),
        usage "the 五月 of 太初元年 (104 BCE), 辛酉 on 20 June (Julian), when the Han put the 太初曆 in the place of the 顓頊曆 \
            [wikipedia-zh-taichu-era, wikipedia-zh-taichuli, hanshu-lulizhi], to the day before 元和二年二月甲寅, 18 March 85, the day the 四分曆 \
            began [wikipedia-zh-sifen, liu-chinese-calendar-tables]",
        model mean_model! {
            month 2_392 / 81,
            year 562_120 / 1_539,
            epoch EPOCH,
            solstice_from_epoch 0.0,
        },
    }

    /// 25 December 105 BCE (Julian): the 甲子 day on which the Han shu
    /// puts 朔旦冬至 at midnight, in the year before 太初元年.
    pub const EPOCH: Rd = julian_day(-104, 12, 25);
}

/// The 四分曆, the Eastern Han's, and the Shu's after it.
///
/// | | |
/// |---|---|
/// | 日法 | 4 |
/// | 朔 | 27 759/940 = 29 499/940 days (蔀月 940) |
/// | 歲 | 1 461/4 = 365 1/4 days |
/// | Epoch | 25 December 162 BCE (Julian), 甲子, the first 蔀首 of the 元 |
pub mod sifen {
    use super::*;

    system! {
        /// 四分曆: the Eastern Han's from 85, the Wei's to 237 and the Shu's to 263.
        calendar SifenCalendar,
        id "chinese-sifen",
        english "Chinese Sifen (Eastern Han, 85 – 264)",
        earliest julian_day(85, 3, 18),
        latest julian_day(264, 2, 14),
        usage "元和二年二月甲寅, 18 March 85, when the 四分曆 replaced the 太初曆 [wikipedia-zh-sifen, hhs-lulizhi-xia], to the end of the \
            Chinese year 263, the last the Shu kept it [liu-chinese-calendar-tables]; the Wei used it to 237 and the Wu in 222",
        model mean_model! {
            month 27_759 / 940,
            year 1_461 / 4,
            epoch EPOCH,
            solstice_from_epoch 0.0,
        },
    }

    /// 25 December 162 BCE (Julian): the 甲子 蔀首 at midnight, the 冬十有
    /// 一月 of the 庚辰 year the 後漢書 names.
    pub const EPOCH: Rd = julian_day(-161, 12, 25);
}

/// The 乾象曆 of Liú Hóng, the Wu's.
///
/// | | |
/// |---|---|
/// | 日法 | 1 457 |
/// | 朔 | 43 026/1 457 = 29 773/1 457 days |
/// | 歲 | 215 130/589 = 365 145/589 days (紀法 589) |
/// | Epoch | the 內紀's 甲子 at 25 December 105 BCE |
pub mod qianxiang {
    use super::*;

    system! {
        /// 乾象曆: the Wu's from 黃武二年正月 (223) to its end.
        calendar QianxiangCalendar,
        id "chinese-qianxiang",
        english "Chinese Qianxiang (Wu, 223 – 281)",
        earliest julian_day(223, 2, 18),
        latest julian_day(281, 2, 5),
        usage "黃武二年正月, 18 February 223, when the Wu put the 乾象曆 in place of the 四分曆 [wikipedia-zh-qianxiang, jinshu-lulizhi, liu-chinese-calendar-tables], \
            to the end of the Chinese year 280, the Wu's last [liu-chinese-calendar-tables]",
        model mean_model! {
            month 43_026 / 1_457,
            year 215_130 / 589,
            epoch EPOCH,
            solstice_from_epoch 0.0,
        },
    }

    /// 25 December 105 BCE (Julian): the first day of the 內紀, reached
    /// from the 上元己丑 and the 歲積 7 378 to 建安十一年 as the 晉書 gives
    /// them. It is the day of the 太初曆's epoch, as the 晉書's own
    /// account of Liú Hóng's 加《太初》元十二紀 (徐岳) has it.
    pub const EPOCH: Rd = julian_day(-104, 12, 25);
}

/// The 景初曆 of Yáng Wěi, the Wei's, the Jin's and the Liu Song's.
///
/// | | |
/// |---|---|
/// | 日法 | 4 559 |
/// | 朔 | 134 630/4 559 = 29 2 419/4 559 days |
/// | 歲 | 673 150/1 843 = 365 455/1 843 days (紀法 1 843) |
/// | Epoch | the 甲申紀's, 25 December 124 BCE (Julian) |
pub mod jingchu {
    use super::*;

    system! {
        /// 景初曆: from 正始元年 (240) to 元嘉二十一年 (444).
        calendar JingchuCalendar,
        id "chinese-jingchu",
        english "Chinese Jingchu (Wei, Jin and Song, 240 – 445)",
        earliest julian_day(240, 2, 10),
        latest julian_day(445, 1, 23),
        usage "the first month of the Chinese year 240, 10 February, after the Wei's three years with the first month a month earlier \
            [liu-chinese-calendar-tables], to the day before the 元嘉曆's first day [songshu-lulizhi]; the Jin kept it as the 泰始曆 \
            [jinshu-lulizhi]",
        model mean_model! {
            month 134_630 / 4_559,
            year 673_150 / 1_843,
            epoch EPOCH,
            solstice_from_epoch 0.0,
        },
    }

    /// 25 December 124 BCE (Julian): the 甲申 first day of the third 紀,
    /// reached from the 壬辰 上元 and the 積 4 046 to 景初元年 as the 晉書
    /// gives them.
    pub const EPOCH: Rd = julian_day(-123, 12, 25);
}

/// The 元嘉曆 of Hé Chéngtiān, the Southern Dynasties' from 445.
///
/// Its epoch is a new moon at which 雨水, not 冬至, falls at midnight: the
/// treatise reckons the year from 雨水, so the winter solstice stands two
/// twelfths of the year before it.
///
/// | | |
/// |---|---|
/// | 日法 | 752 |
/// | 朔 | 22 207/752 = 29 399/752 days |
/// | 歲 | 111 035/304 = 365 75/304 days (度法 304) |
/// | Epoch | the 甲午紀's, 20 February 212 (Julian), 朔旦雨水 at midnight |
pub mod yuanjia {
    use super::*;

    system! {
        /// 元嘉曆: from 元嘉二十二年 (445) to 天監八年 (509).
        calendar YuanjiaCalendar,
        id "chinese-yuanjia",
        english "Chinese Yuanjia (Southern Dynasties, 445 – 510)",
        earliest julian_day(445, 1, 24),
        latest julian_day(510, 1, 25),
        usage "the first month of the Chinese year 445, 24 January, when the 元嘉曆 replaced the 景初曆 [songshu-lulizhi, liu-chinese-calendar-tables], \
            to the day before the 大明曆's [liu-chinese-calendar-tables]",
        model mean_model! {
            month 22_207 / 752,
            year 111_035 / 304,
            epoch EPOCH,
            // 雨水 is two zhōngqì after 冬至, so the solstice is two
            // twelfths of the year before the epoch.
            solstice_from_epoch -2.0 * (111_035.0 / 304.0) / 12.0,
        },
    }

    /// 20 February 212 (Julian): the 甲午 first day of the fourth 紀, from
    /// which 元嘉二十年 is the 231st year, as the 宋書 says.
    pub const EPOCH: Rd = julian_day(212, 2, 20);
}

/// The 大明曆 of Zǔ Chōngzhī, the Southern Dynasties' from 510.
///
/// The treatise's epoch is 51 939 years before 大明七年, 天正 甲子 朔旦
/// 冬至 at midnight; the model takes the same instant one 紀 of 39 491
/// years later, 14 423 804 days, the nearest at which the new moon and the
/// solstice recur together to the day.
///
/// | | |
/// |---|---|
/// | 日法 | 3 939 |
/// | 朔 | 116 321/3 939 = 29 2 090/3 939 days (月法 116 321) |
/// | 歲 | 14 423 804/39 491 = 365 9 589/39 491 days (紀法 39 491) |
/// | Epoch | 20 March 11 986 BCE (Julian) |
pub mod daming {
    use super::*;

    system! {
        /// 大明曆: from 天監九年 (510) to the end of the Chen (589).
        calendar DamingCalendar,
        id "chinese-daming",
        english "Chinese Daming (Southern Dynasties, 510 – 590)",
        earliest julian_day(510, 1, 26),
        latest julian_day(590, 2, 9),
        usage "the first month of the Chinese year 510, 26 January, when the Liang put the 大明曆 in place of the 元嘉曆 [songshu-lulizhi-daming, liu-chinese-calendar-tables], \
            to the end of the Chinese year 589, the Chen's last [liu-chinese-calendar-tables]",
        model mean_model! {
            month 116_321 / 3_939,
            year 14_423_804 / 39_491,
            epoch EPOCH,
            solstice_from_epoch 0.0,
        },
    }

    /// 20 March 11 986 BCE (Julian): 2 656 385 days before JDN 0, one 紀
    /// after the treatise's 上元, whose 甲子 朔旦冬至 the 宋書 puts 51 939
    /// years before 大明七年.
    pub const EPOCH: Rd = julian_day(-11_985, 3, 20);
}

/// The 興和曆 of Lǐ Yèxīng, the Eastern Wei's.
///
/// Its 蔀 of 16 860 years is the interval after which the new moon, the
/// winter solstice and the day all return together, so the epoch of the
/// model is the 蔀's beginning, and the treatise's own 紀首, a 甲戌 day, lies
/// seven 蔀 of 6 158 017 days earlier.
///
/// | | |
/// |---|---|
/// | 日法 | 208 530 |
/// | 朔 | 6 158 017/208 530 = 29 110 647/208 530 days (通數) |
/// | 歲 | 6 158 017/16 860 = 365 4 117/16 860 days (蔀法, 斗分) |
/// | Epoch | 1 February 6837 BCE (Julian), the 蔀 beginning seven after the 甲戌紀's |
pub mod xinghe {
    use super::*;

    system! {
        /// 興和曆: the Eastern Wei's from 540 to 550.
        calendar XingheCalendar,
        id "chinese-xinghe",
        english "Chinese Xinghe (Eastern Wei, 540 – 551)",
        earliest julian_day(540, 1, 25),
        latest julian_day(551, 1, 22),
        usage "the first month of the Chinese year 540, 25 January, the 興和二年 the 魏書 counts from [weishu-lulizhi-xia, liu-chinese-calendar-tables], \
            to the end of the Chinese year 550, the last of the Eastern Wei [liu-chinese-calendar-tables]; Li Yèxīng compiled it in 興和元年 (539) \
            [wikipedia-zh-xinghe]",
        model mean_model! {
            month 6_158_017 / 208_530,
            year 6_158_017 / 16_860,
            epoch EPOCH,
            solstice_from_epoch 0.0,
        },
    }

    /// 1 February 6837 BCE (Julian): the 蔀 beginning seven 蔀 after the
    /// 甲戌 紀's, which the 魏書 gives for 興和二年 as year 125 397 of the
    /// 紀 (算上).
    pub const EPOCH: Rd = julian_day(-6_836, 2, 1);
}

/// The 天和曆 of Zhēn Luán, the Northern Zhou's.
///
/// | | |
/// |---|---|
/// | 日法 | 290 160 (the months in a 蔀) |
/// | 朔 | 8 568 631/290 160 = 29 153 991/290 160 days (朔餘 153 991) |
/// | 歲 | 8 568 631/23 460 = 365 5 731/23 460 days (蔀法, 斗分) |
/// | Epoch | 1 February 7207 BCE (Julian), the 蔀 beginning 37 蔀 after the 上元's |
pub mod tianhe {
    use super::*;

    system! {
        /// 天和曆: the Northern Zhou's from 天和元年 (566) to 宣政元年 (578).
        calendar TianheCalendar,
        id "chinese-tianhe",
        english "Chinese Tianhe (Northern Zhou, 566 – 579)",
        earliest julian_day(566, 2, 6),
        latest julian_day(579, 2, 11),
        usage "the first month of the Chinese year 566, 6 February, 天和元年 [suishu-lulizhi-zhong, wikipedia-zh-tianhe, liu-chinese-calendar-tables], \
            to the end of the Chinese year 578, 宣政元年 [wikipedia-zh-tianhe, liu-chinese-calendar-tables]",
        model mean_model! {
            month 8_568_631 / 290_160,
            year 8_568_631 / 23_460,
            epoch EPOCH,
            solstice_from_epoch 0.0,
        },
    }

    /// 1 February 7207 BCE (Julian): the 蔀 beginning 37 蔀 after the
    /// 甲子 of the 上元, to which the 隋書 counts 875 792 years (算外) to
    /// 天和元年, the 蔀首 taking the name 辛未 on the way.
    pub const EPOCH: Rd = julian_day(-7_206, 2, 1);
}

/// The 開皇曆 of Zhāng Bīn, the Sui's from 開皇四年 (584).
///
/// | | |
/// |---|---|
/// | 日法 | 181 920 |
/// | 朔 | 5 372 209/181 920 = 29 96 529/181 920 days (通月) |
/// | 歲 | 37 605 463/102 960 = 365 25 063/102 960 days (蔀法, 斗分) |
/// | Epoch | 27 February 10 017 BCE (Julian), the 甲辰 蔀 beginning, the 40th after the 上元's |
pub mod kaihuang {
    use super::*;

    system! {
        /// 開皇曆: the Sui's from 開皇四年 (584) to 開皇十六年 (596).
        calendar KaihuangCalendar,
        id "chinese-kaihuang",
        english "Chinese Kaihuang (Sui, 584 – 597)",
        earliest julian_day(584, 2, 17),
        latest julian_day(597, 1, 23),
        usage "the first month of the Chinese year 584, 17 February, 開皇四年, when Zhāng Bīn's calendar was put in use [suishu-lulizhi-zhong, wikipedia-zh-kaihuang, liu-chinese-calendar-tables], \
            to the end of the Chinese year 596, the last before 張胄玄's [wikipedia-zh-kaihuang, wikipedia-zh-daye, liu-chinese-calendar-tables]",
        model mean_model! {
            month 5_372_209 / 181_920,
            year 37_605_463 / 102_960,
            epoch EPOCH,
            solstice_from_epoch 0.0,
        },
    }

    /// 27 February 10 017 BCE (Julian): the 蔀 beginning 40 蔀 after the
    /// 甲子 of the 上元, whose 甲辰 name follows from the 蔀's 37 605 463
    /// days, 43 more than a multiple of sixty. The 隋書 puts 開皇四年 4 129 001
    /// years (算上) from the 上元.
    pub const EPOCH: Rd = julian_day(-10_016, 2, 27);
}

/// The 三紀甲子元曆 of Jiāng Jí, the Later Qin's.
///
/// | | |
/// |---|---|
/// | 日法 | 6 063 (the 隋 transcription prints 6 062) |
/// | 朔 | 895 220/30 315 = 179 044/6 063 = 29 3 247/6 063 days (紀日 and 紀月) |
/// | 歲 | 895 220/2 451 = 365 605/2 451 days (紀法, 斗分) |
/// | Epoch | the 甲申紀's, 25 December 124 BCE (Julian) |
pub mod sanji {
    use super::*;

    system! {
        /// 三紀甲子元曆: the Later Qin's, 384 to 417.
        calendar SanjiCalendar,
        id "chinese-sanji",
        english "Chinese Sanji (Later Qin, 384 – 418)",
        earliest julian_day(384, 2, 8),
        latest julian_day(418, 2, 20),
        usage "the first month of the Chinese year 384, 8 February, the 太元九年 Jiāng Jí made it for [jinshu-lulizhi, liu-chinese-calendar-tables], \
            to the end of the Chinese year 417, the last of the Later Qin [liu-chinese-calendar-tables]; the Northern Liang's months of 412–439 are the same but for one",
        model mean_model! {
            month 895_220 / 30_315,
            year 895_220 / 2_451,
            epoch EPOCH,
            solstice_from_epoch 0.0,
        },
    }

    /// 25 December 124 BCE (Julian): the 甲申 紀's first day, the second
    /// of the 元's three, reached from the 甲子 上元 and the 積 83 841 to
    /// 太元九年 (算上) as the 晉書 gives them.
    pub const EPOCH: Rd = julian_day(-123, 12, 25);
}

/// The 正光曆 of Lǐ Yèxīng and Zhāng Lóngxiáng, the Northern Wei's from
/// 523 and the Western Wei's.
///
/// | | |
/// |---|---|
/// | 日法 | 74 952 (the months in a 蔀) |
/// | 朔 | 2 213 377/74 952 = 29 39 769/74 952 days (經月) |
/// | 歲 | 2 213 377/6 060 = 365 1 477/6 060 days (蔀法, 斗分) |
/// | Epoch | 15 January 3609 BCE (Julian), the 蔀 beginning seven after the 甲申紀's |
pub mod zhengguang {
    use super::*;

    system! {
        /// 正光曆: from 523 to 558.
        calendar ZhengguangCalendar,
        id "chinese-zhengguang",
        english "Chinese Zhengguang (Northern and Western Wei, 523 – 559)",
        earliest julian_day(523, 2, 1),
        latest julian_day(559, 1, 23),
        usage "the first month of the Chinese year 523, 1 February, the 正光 Liu's list gives it from [weishu-lulizhi-shang, wikipedia-zh-zhengguang, liu-chinese-calendar-tables], \
            to the end of the Chinese year 558, the last of the Western Wei's [liu-chinese-calendar-tables]; it is also the months of the Eastern Wei's 534–539 and, \
            by the table, of the Northern Zhou's 557–565",
        model mean_model! {
            month 2_213_377 / 74_952,
            year 2_213_377 / 6_060,
            epoch EPOCH,
            solstice_from_epoch 0.0,
        },
    }

    /// 15 January 3609 BCE (Julian): the 蔀 beginning seven after the 甲申
    /// 紀's, the third 紀 of the 元, to which the 魏書 counts 167 745 years
    /// (算外) from the 壬子 上元 to 熙平二年.
    pub const EPOCH: Rd = julian_day(-3_608, 1, 15);
}
