//! The Chinese lunisolar calendars of 104 BCE to 597 CE on their own
//! arithmetic: the 太初曆, 四分曆, 乾象曆, 景初曆, 元嘉曆, 大明曆, 興和曆,
//! 天和曆, 開皇曆, 三紀甲子元曆 and 正光曆.
//!
//! Three kinds of evidence, and the system document
//! `docs/systems/chinese-historical-lunisolar.md` says which is which:
//!
//! * dates written in the sources — the Han shu's epoch, the day the
//!   四分曆 began, the Wei court's eclipse records of 221 and 222, the
//!   treatises' sexagenary names for their own epochs;
//! * the first days of whole years in the tables of Liu Yuk Tung's
//!   *Chinese-Western Calendar Conversion Table* (`liu-chinese-calendar-tables`)
//!   and of the Chinese Wikipedia's article on 太初 (`wikipedia-zh-taichu-era`),
//!   year by year, with the leap months;
//! * an oracle written here in integer arithmetic from the same ratios,
//!   which every month of every system is compared with, since the
//!   engine counts in floating point.
//!
//! A month of a table is a Julian date, the calendar of the sources, and
//! `to_fixed` is asked for it by the lunisolar date it names.

use hc_calendar::{Calendar, CalendarError, Month, Rd, gregorian};
use hc_calendars_lunar::chinese_historical::{
    daming, jingchu, kaihuang, qianxiang, sanji, sifen, taichu, tianhe, xinghe, yuanjia, zhengguang,
};
use hc_calendars_lunar::{LunisolarCalendar, LunisolarDate, LunisolarParameters};
use hc_calendars_solar::julian;

fn julian_day(year: i64, month: u8, day: u8) -> Rd {
    julian::to_fixed(year, month, day)
        .unwrap_or_else(|error| panic!("a Julian date {year}-{month}-{day}: {error:?}"))
}

/// A year of a table: `(Chinese year, month, is leap, Julian year, month,
/// day)`, one entry for the first day of each month.
type Month1 = (i64, u8, bool, i64, u8, u8);

fn check_table(engine: LunisolarCalendar, table: &[Month1]) {
    let parameters = engine.parameters();
    for &(year, ordinal, leap, jy, jm, jd) in table {
        let rd = julian_day(jy, jm, jd);
        let month = Month { ordinal, leap };
        assert_eq!(
            parameters.to_fixed(year, month, 1),
            Ok(rd),
            "{year} month {ordinal}{}",
            if leap { " (leap)" } else { "" }
        );
        assert_eq!(
            parameters.from_fixed(rd),
            Ok((year, month, 1)),
            "{jy}-{jm}-{jd}"
        );
        // The day before is the last of the month before, unless the month
        // is the first of the span.
        if let Ok((_, _, last)) = parameters.from_fixed(Rd(rd.0 - 1)) {
            assert!(last == 29 || last == 30, "{jy}-{jm}-{jd} follows {last}");
        }
    }
}

/// The sexagenary name of a day, in the characters the sources write.
fn day_name(parameters: &LunisolarParameters, rd: Rd) -> String {
    const STEMS: [char; 10] = ['甲', '乙', '丙', '丁', '戊', '己', '庚', '辛', '壬', '癸'];
    const BRANCHES: [char; 12] = [
        '子', '丑', '寅', '卯', '辰', '巳', '午', '未', '申', '酉', '戌', '亥',
    ];
    let index = usize::from(parameters.sexagenary_day(rd).index());
    format!("{}{}", STEMS[index % 10], BRANCHES[index % 12])
}

// ---------------------------------------------------------------------------
// Dates the sources write

/// 太初元年五月 begins on 辛酉, 20 June 104 BCE: the Chinese Wikipedia's
/// table of 太初, which gives the day names with the Julian dates and the
/// size of every month for the four years of the era. Each month is the day
/// of the table, by name, month and size.
#[test]
fn the_four_years_of_taichu_are_the_wikipedia_table() {
    // (Julian year, month, day, 干支, month number, leap, 大 is true)
    let table: &[(i64, u8, u8, &str, u8, bool, bool)] = &[
        (-103, 6, 20, "辛酉", 5, false, false),
        (-103, 7, 19, "庚寅", 6, false, true),
        (-103, 8, 18, "庚申", 7, false, false),
        (-103, 9, 16, "己丑", 8, false, true),
        (-103, 10, 16, "己未", 9, false, false),
        (-103, 11, 14, "戊子", 10, false, true),
        (-103, 12, 14, "戊午", 11, false, false),
        (-102, 1, 12, "丁亥", 12, false, true),
        (-102, 2, 11, "丁巳", 1, false, false),
        (-102, 3, 12, "丙戌", 2, false, true),
        (-102, 4, 11, "丙辰", 3, false, true),
        (-102, 5, 11, "丙戌", 4, false, false),
        (-102, 6, 9, "乙卯", 5, false, true),
        (-102, 7, 9, "乙酉", 6, false, false),
        (-102, 8, 7, "甲寅", 7, false, true),
        (-102, 9, 6, "甲申", 8, false, false),
        (-102, 10, 5, "癸丑", 9, false, true),
        (-102, 11, 4, "癸未", 10, false, false),
        (-102, 12, 3, "壬子", 11, false, true),
        (-101, 1, 2, "壬午", 12, false, false),
        (-101, 1, 31, "辛亥", 1, false, true),
        (-101, 3, 2, "辛巳", 2, false, false),
        (-101, 3, 31, "庚戌", 3, false, true),
        (-101, 4, 30, "庚辰", 4, false, false),
        (-101, 5, 29, "己酉", 5, false, true),
        (-101, 6, 28, "己卯", 6, false, false),
        (-101, 7, 27, "戊申", 6, true, true),
        (-101, 8, 26, "戊寅", 7, false, true),
        (-101, 9, 25, "戊申", 8, false, false),
        (-101, 10, 24, "丁丑", 9, false, true),
        (-101, 11, 23, "丁未", 10, false, false),
        (-101, 12, 22, "丙子", 11, false, true),
        (-100, 1, 21, "丙午", 12, false, false),
        (-100, 2, 19, "乙亥", 1, false, true),
        (-100, 3, 20, "乙巳", 2, false, false),
        (-100, 4, 18, "甲戌", 3, false, true),
        (-100, 5, 18, "甲辰", 4, false, false),
        (-100, 6, 16, "癸酉", 5, false, true),
        (-100, 7, 16, "癸卯", 6, false, false),
        (-100, 8, 14, "壬申", 7, false, true),
        (-100, 9, 13, "壬寅", 8, false, false),
        (-100, 10, 12, "辛未", 9, false, true),
        (-100, 11, 11, "辛丑", 10, false, true),
        (-100, 12, 11, "辛未", 11, false, false),
        (-99, 1, 9, "庚子", 12, false, true),
    ];
    let engine = taichu::ENGINE;
    let parameters = engine.parameters();
    // The months before the first month 1 belong to the year 104 BCE, the
    // astronomical -103, and each month 1 begins the next year.
    let mut expected_year = -103;
    for (index, &(jy, jm, jd, name, ordinal, leap, big)) in table.iter().enumerate() {
        let rd = julian_day(jy, jm, jd);
        assert_eq!(day_name(parameters, rd), name, "{jy}-{jm}-{jd}");
        if ordinal == 1 && !leap {
            expected_year += 1;
        }
        let (year, month, day) = parameters.from_fixed(rd).expect("in range");
        assert_eq!(
            (month.ordinal, month.leap, day),
            (ordinal, leap, 1),
            "{jy}-{jm}-{jd}"
        );
        assert_eq!(year, expected_year, "{jy}-{jm}-{jd}");
        // Big months are 30 days, small 29.
        let next = table.get(index + 1);
        if let Some(&(ny, nm, nd, ..)) = next {
            let length = julian_day(ny, nm, nd).0 - rd.0;
            assert_eq!(length == 30, big, "{jy}-{jm}-{jd} has {length} days");
        }
    }
}

/// 元和二年二月四日甲寅, 85: the day the 四分曆 began, as the Chinese
/// Wikipedia gives it ("元和二年二月四日甲寅（85年）施行"). The 四分曆's
/// first day is the 4th of the second month, and the 太初曆, which counts
/// the second month from the same day, gives the day before it.
#[test]
fn the_sifen_began_on_the_fourth_of_the_second_month_of_85() {
    let parameters = sifen::ENGINE.parameters();
    assert_eq!(day_name(parameters, sifen::EARLIEST), "甲寅");
    assert_eq!(
        parameters.from_fixed(sifen::EARLIEST),
        Ok((85, Month::regular(2), 4))
    );
    assert_eq!(sifen::EARLIEST, julian_day(85, 3, 18));
    assert_eq!(taichu::LATEST.0 + 1, sifen::EARLIEST.0);
    assert_eq!(
        taichu::ENGINE.parameters().from_fixed(taichu::LATEST),
        Ok((85, Month::regular(2), 3))
    );
    assert_eq!(
        taichu::ENGINE.parameters().from_fixed(sifen::EARLIEST),
        Err(CalendarError::AfterSupportedRange)
    );
}

/// The epochs are the days the treatises name. The Han shu has the 太初
/// reckoning begin with 甲子 朔旦冬至 at midnight, and the 乾象曆's 內紀
/// is the same instant; the 後漢書 puts the 四分曆's 蔀首 on 甲子 in the
/// winter of the 庚辰 year; the 晉書's 景初曆 counts from a 甲申紀 and the
/// 宋書's 元嘉曆 from a 甲午紀; the 大明曆's 上元 is a 甲子 51 939 years
/// before 大明七年, which is one 紀 of 14 423 804 days before the day the
/// model starts from.
#[test]
fn every_epoch_is_the_sexagenary_day_the_treatise_names() {
    let cases: [(&LunisolarParameters, Rd, &str); 11] = [
        (taichu::ENGINE.parameters(), taichu::EPOCH, "甲子"),
        (sifen::ENGINE.parameters(), sifen::EPOCH, "甲子"),
        (qianxiang::ENGINE.parameters(), qianxiang::EPOCH, "甲子"),
        (jingchu::ENGINE.parameters(), jingchu::EPOCH, "甲申"),
        (yuanjia::ENGINE.parameters(), yuanjia::EPOCH, "甲午"),
        (
            daming::ENGINE.parameters(),
            Rd(daming::EPOCH.0 - 14_423_804),
            "甲子",
        ),
        // The 蔀 is 6 158 017 days, 8 568 631 and 37 605 463; the epochs
        // are 7, 37 and 40 of them after the treatise's 甲戌 紀, 甲子 上元
        // and 甲子 上元.
        (
            xinghe::ENGINE.parameters(),
            Rd(xinghe::EPOCH.0 - 7 * 6_158_017),
            "甲戌",
        ),
        (
            tianhe::ENGINE.parameters(),
            Rd(tianhe::EPOCH.0 - 37 * 8_568_631),
            "甲子",
        ),
        (
            kaihuang::ENGINE.parameters(),
            Rd(kaihuang::EPOCH.0 - 40 * 37_605_463),
            "甲子",
        ),
        // The 三紀甲子元曆's epoch is its 甲申紀's first day, and the 正光曆's
        // is seven 蔀 of 2 213 377 days after it.
        (sanji::ENGINE.parameters(), sanji::EPOCH, "甲申"),
        (
            zhengguang::ENGINE.parameters(),
            Rd(zhengguang::EPOCH.0 - 7 * 2_213_377),
            "甲申",
        ),
    ];
    for (parameters, epoch, name) in cases {
        assert_eq!(
            day_name(parameters, epoch),
            name,
            "{}",
            parameters.english_name
        );
    }
    // The Han epochs are the winter solstice's 25 December (Julian) but the
    // 元嘉曆's, which is 雨水 on 20 February.
    for rd in [
        taichu::EPOCH,
        sifen::EPOCH,
        qianxiang::EPOCH,
        jingchu::EPOCH,
    ] {
        let (_, month, day) = julian::from_fixed(rd).unwrap();
        assert_eq!((month, day), (12, 25));
    }
    assert_eq!(
        julian::from_fixed(yuanjia::EPOCH).map(|(_, m, d)| (m, d)),
        Ok((2, 20))
    );
}

/// The 晉書's 律曆中 gives the Wei court's records of 黃初二年 and 三年,
/// 221 and 222, when the Wei kept the 四分曆: "二年六月二十九日戊辰",
/// "二年七月十五日癸未", "三年正月丙寅朔" and "三年十一月十五日乙巳". A day
/// of a month of a year is a day of the 四分曆 here with the name the
/// record gives it. A fifth, "三年十一月二十九日庚申", is not: 庚申 is the
/// first day of the twelfth month here and in the Liu table, and the
/// 二十九日 the record gives it is a day before, so the record is left out.
#[test]
fn the_wei_courts_records_of_221_and_222_fall_on_their_named_days() {
    let engine = sifen::ENGINE;
    let parameters = engine.parameters();
    let cases: [(i64, u8, u8, &str); 4] = [
        (221, 6, 29, "戊辰"),
        (221, 7, 15, "癸未"),
        (222, 1, 1, "丙寅"),
        (222, 11, 15, "乙巳"),
    ];
    for (year, month, day, name) in cases {
        let rd = parameters
            .to_fixed(year, Month::regular(month), day)
            .unwrap_or_else(|error| panic!("{year}-{month}-{day}: {error:?}"));
        assert_eq!(day_name(parameters, rd), name, "{year}-{month}-{day}");
    }
}

// ---------------------------------------------------------------------------
// Whole years of the tables

/// The 太初曆's last full year before the 四分曆, 84: a leap month after the
/// first, and the 85 months the Liu table gives the first two months of
/// 85 with.
#[test]
fn the_last_taichu_years_are_the_tables() {
    check_table(
        taichu::ENGINE,
        &[
            (84, 1, false, 84, 1, 27),
            (84, 1, true, 84, 2, 25),
            (84, 2, false, 84, 3, 26),
            (84, 3, false, 84, 4, 25),
            (84, 4, false, 84, 5, 24),
            (84, 5, false, 84, 6, 23),
            (84, 6, false, 84, 7, 22),
            (84, 7, false, 84, 8, 21),
            (84, 8, false, 84, 9, 19),
            (84, 9, false, 84, 10, 19),
            (84, 10, false, 84, 11, 17),
            (84, 11, false, 84, 12, 17),
            (84, 12, false, 85, 1, 15),
            (85, 2, false, 85, 3, 15),
        ],
    );
}

/// The 四分曆 from 85 to 86, 150 and 219: the table's years, with a leap
/// month in 86 and in 219. The first month of 85 is the one the table begins
/// a day before the 太初曆's; the 四分曆 begins on the 18th of March.
#[test]
fn sifen_years_are_the_tables() {
    check_table(
        sifen::ENGINE,
        &[
            (85, 3, false, 85, 4, 13),
            (85, 4, false, 85, 5, 13),
            (85, 5, false, 85, 6, 11),
            (85, 6, false, 85, 7, 11),
            (85, 7, false, 85, 8, 9),
            (85, 8, false, 85, 9, 8),
            (85, 9, false, 85, 10, 7),
            (85, 10, false, 85, 11, 6),
            (85, 11, false, 85, 12, 5),
            (85, 12, false, 86, 1, 4),
            (86, 1, false, 86, 2, 2),
            (86, 10, false, 86, 10, 26),
            (86, 10, true, 86, 11, 25),
            (86, 11, false, 86, 12, 24),
            (150, 1, false, 150, 2, 15),
            (150, 2, false, 150, 3, 16),
            (150, 3, false, 150, 4, 15),
            (150, 4, false, 150, 5, 14),
            (150, 5, false, 150, 6, 13),
            (150, 6, false, 150, 7, 12),
            (150, 7, false, 150, 8, 11),
            (150, 8, false, 150, 9, 10),
            (150, 9, false, 150, 10, 9),
            (150, 10, false, 150, 11, 8),
            (150, 11, false, 150, 12, 7),
            (150, 12, false, 151, 1, 6),
            (219, 1, false, 219, 2, 3),
            (219, 2, false, 219, 3, 4),
            (219, 3, false, 219, 4, 3),
            (219, 4, false, 219, 5, 2),
            (219, 5, false, 219, 6, 1),
            (219, 6, false, 219, 6, 30),
            (219, 7, false, 219, 7, 30),
            (219, 8, false, 219, 8, 28),
            (219, 9, false, 219, 9, 27),
            (219, 10, false, 219, 10, 26),
            (219, 10, true, 219, 11, 25),
            (219, 11, false, 219, 12, 25),
            (219, 12, false, 220, 1, 23),
            // The Shu's last year, 263, which the Shu kept the 四分曆 to.
            (263, 1, false, 263, 1, 27),
            (263, 2, false, 263, 2, 26),
            (263, 3, false, 263, 3, 28),
            (263, 4, false, 263, 4, 26),
            (263, 4, true, 263, 5, 26),
            (263, 5, false, 263, 6, 24),
            (263, 6, false, 263, 7, 24),
            (263, 7, false, 263, 8, 22),
            (263, 8, false, 263, 9, 21),
            (263, 9, false, 263, 10, 20),
            (263, 10, false, 263, 11, 19),
            (263, 11, false, 263, 12, 18),
            (263, 12, false, 264, 1, 17),
        ],
    );
}

/// The Wu's first and last years under the 乾象曆, 223 and 280.
#[test]
fn qianxiang_years_are_the_tables() {
    check_table(
        qianxiang::ENGINE,
        &[
            (223, 1, false, 223, 2, 18),
            (223, 2, false, 223, 3, 19),
            (223, 3, false, 223, 4, 18),
            (223, 4, false, 223, 5, 17),
            (223, 5, false, 223, 6, 16),
            (223, 6, false, 223, 7, 15),
            (223, 7, false, 223, 8, 14),
            (223, 8, false, 223, 9, 12),
            (223, 9, false, 223, 10, 12),
            (223, 10, false, 223, 11, 10),
            (223, 11, false, 223, 12, 10),
            (223, 12, false, 224, 1, 9),
            (280, 1, false, 280, 2, 18),
            (280, 2, false, 280, 3, 18),
            (280, 3, false, 280, 4, 17),
            (280, 4, false, 280, 5, 16),
            (280, 5, false, 280, 6, 15),
            (280, 6, false, 280, 7, 14),
            (280, 7, false, 280, 8, 13),
            (280, 8, false, 280, 9, 11),
            (280, 9, false, 280, 10, 11),
            (280, 10, false, 280, 11, 10),
            (280, 11, false, 280, 12, 9),
            (280, 12, false, 281, 1, 8),
        ],
    );
}

/// The 景初曆 in the Wei's 240 and 265, the year with the leap month the
/// Jin began under it.
#[test]
fn jingchu_years_are_the_tables() {
    check_table(
        jingchu::ENGINE,
        &[
            (240, 1, false, 240, 2, 10),
            (240, 2, false, 240, 3, 11),
            (240, 3, false, 240, 4, 9),
            (240, 4, false, 240, 5, 9),
            (240, 5, false, 240, 6, 7),
            (240, 6, false, 240, 7, 7),
            (240, 7, false, 240, 8, 5),
            (240, 8, false, 240, 9, 4),
            (240, 9, false, 240, 10, 3),
            (240, 10, false, 240, 11, 2),
            (240, 11, false, 240, 12, 1),
            (240, 12, false, 240, 12, 31),
            (265, 1, false, 265, 2, 3),
            (265, 2, false, 265, 3, 5),
            (265, 3, false, 265, 4, 3),
            (265, 4, false, 265, 5, 3),
            (265, 5, false, 265, 6, 1),
            (265, 6, false, 265, 7, 1),
            (265, 7, false, 265, 7, 30),
            (265, 8, false, 265, 8, 29),
            (265, 9, false, 265, 9, 27),
            (265, 10, false, 265, 10, 27),
            (265, 11, false, 265, 11, 25),
            (265, 11, true, 265, 12, 25),
            (265, 12, false, 266, 1, 23),
            (444, 1, false, 444, 2, 5),
            (444, 2, false, 444, 3, 5),
            (444, 3, false, 444, 4, 4),
            (444, 4, false, 444, 5, 3),
            (444, 5, false, 444, 6, 2),
            (444, 6, false, 444, 7, 1),
            (444, 7, false, 444, 7, 31),
            (444, 8, false, 444, 8, 29),
            (444, 9, false, 444, 9, 28),
            (444, 10, false, 444, 10, 28),
            (444, 11, false, 444, 11, 26),
            (444, 12, false, 444, 12, 26),
        ],
    );
}

/// The 元嘉曆's first and last years, 445, with its leap month after the
/// fifth, and 509.
#[test]
fn yuanjia_years_are_the_tables() {
    check_table(
        yuanjia::ENGINE,
        &[
            (445, 1, false, 445, 1, 24),
            (445, 2, false, 445, 2, 23),
            (445, 3, false, 445, 3, 24),
            (445, 4, false, 445, 4, 23),
            (445, 5, false, 445, 5, 22),
            (445, 5, true, 445, 6, 21),
            (445, 6, false, 445, 7, 20),
            (445, 7, false, 445, 8, 19),
            (445, 8, false, 445, 9, 17),
            (445, 9, false, 445, 10, 17),
            (445, 10, false, 445, 11, 15),
            (445, 11, false, 445, 12, 15),
            (445, 12, false, 446, 1, 13),
            (509, 1, false, 509, 2, 5),
            (509, 2, false, 509, 3, 7),
            (509, 3, false, 509, 4, 5),
            (509, 4, false, 509, 5, 5),
            (509, 5, false, 509, 6, 3),
            (509, 6, false, 509, 7, 3),
            (509, 7, false, 509, 8, 2),
            (509, 8, false, 509, 8, 31),
            (509, 9, false, 509, 9, 30),
            (509, 10, false, 509, 10, 29),
            (509, 11, false, 509, 11, 28),
            (509, 12, false, 509, 12, 27),
        ],
    );
}

/// The 大明曆's first and last years, 510, with its leap month after the
/// sixth, and 589, with a leap month after the third.
#[test]
fn daming_years_are_the_tables() {
    check_table(
        daming::ENGINE,
        &[
            (510, 1, false, 510, 1, 26),
            (510, 2, false, 510, 2, 24),
            (510, 3, false, 510, 3, 26),
            (510, 4, false, 510, 4, 24),
            (510, 5, false, 510, 5, 24),
            (510, 6, false, 510, 6, 22),
            (510, 6, true, 510, 7, 22),
            (510, 7, false, 510, 8, 20),
            (510, 8, false, 510, 9, 19),
            (510, 9, false, 510, 10, 18),
            (510, 10, false, 510, 11, 17),
            (510, 11, false, 510, 12, 17),
            (510, 12, false, 511, 1, 15),
            (589, 1, false, 589, 1, 22),
            (589, 2, false, 589, 2, 21),
            (589, 3, false, 589, 3, 22),
            (589, 3, true, 589, 4, 21),
            (589, 4, false, 589, 5, 20),
            (589, 5, false, 589, 6, 19),
            (589, 6, false, 589, 7, 18),
            (589, 7, false, 589, 8, 17),
            (589, 8, false, 589, 9, 15),
            (589, 9, false, 589, 10, 15),
            (589, 10, false, 589, 11, 13),
            (589, 11, false, 589, 12, 13),
            (589, 12, false, 590, 1, 11),
        ],
    );
}

/// The 興和曆's first and last years, 540, with its leap month after the
/// fifth, and 550.
#[test]
fn xinghe_years_are_the_tables() {
    check_table(
        xinghe::ENGINE,
        &[
            (540, 1, false, 540, 1, 25),
            (540, 2, false, 540, 2, 23),
            (540, 3, false, 540, 3, 24),
            (540, 4, false, 540, 4, 22),
            (540, 5, false, 540, 5, 22),
            (540, 5, true, 540, 6, 20),
            (540, 6, false, 540, 7, 20),
            (540, 7, false, 540, 8, 18),
            (540, 8, false, 540, 9, 17),
            (540, 9, false, 540, 10, 16),
            (540, 10, false, 540, 11, 15),
            (540, 11, false, 540, 12, 14),
            (540, 12, false, 541, 1, 13),
            (550, 1, false, 550, 2, 2),
            (550, 2, false, 550, 3, 4),
            (550, 3, false, 550, 4, 2),
            (550, 4, false, 550, 5, 2),
            (550, 5, false, 550, 5, 31),
            (550, 6, false, 550, 6, 30),
            (550, 7, false, 550, 7, 30),
            (550, 8, false, 550, 8, 28),
            (550, 9, false, 550, 9, 27),
            (550, 10, false, 550, 10, 26),
            (550, 11, false, 550, 11, 25),
            (550, 12, false, 550, 12, 24),
        ],
    );
}

/// The 天和曆's first and last years, 566 and 578, the last with a leap
/// month after the sixth.
#[test]
fn tianhe_years_are_the_tables() {
    check_table(
        tianhe::ENGINE,
        &[
            (566, 1, false, 566, 2, 6),
            (566, 2, false, 566, 3, 7),
            (566, 3, false, 566, 4, 6),
            (566, 4, false, 566, 5, 5),
            (566, 5, false, 566, 6, 4),
            (566, 6, false, 566, 7, 3),
            (566, 7, false, 566, 8, 2),
            (566, 8, false, 566, 8, 31),
            (566, 9, false, 566, 9, 30),
            (566, 10, false, 566, 10, 29),
            (566, 11, false, 566, 11, 28),
            (566, 12, false, 566, 12, 27),
            (578, 1, false, 578, 1, 24),
            (578, 2, false, 578, 2, 23),
            (578, 3, false, 578, 3, 24),
            (578, 4, false, 578, 4, 23),
            (578, 5, false, 578, 5, 22),
            (578, 6, false, 578, 6, 21),
            (578, 6, true, 578, 7, 20),
            (578, 7, false, 578, 8, 19),
            (578, 8, false, 578, 9, 17),
            (578, 9, false, 578, 10, 17),
            (578, 10, false, 578, 11, 16),
            (578, 11, false, 578, 12, 15),
            (578, 12, false, 579, 1, 14),
        ],
    );
}

/// The 開皇曆's first, middle and last years: 584, 590 and 596.
#[test]
fn kaihuang_years_are_the_tables() {
    check_table(
        kaihuang::ENGINE,
        &[
            (584, 1, false, 584, 2, 17),
            (584, 2, false, 584, 3, 17),
            (584, 3, false, 584, 4, 16),
            (584, 4, false, 584, 5, 16),
            (584, 5, false, 584, 6, 14),
            (584, 6, false, 584, 7, 14),
            (584, 7, false, 584, 8, 12),
            (584, 8, false, 584, 9, 11),
            (584, 9, false, 584, 10, 10),
            (584, 10, false, 584, 11, 9),
            (584, 11, false, 584, 12, 8),
            (584, 12, false, 585, 1, 7),
            (590, 1, false, 590, 2, 10),
            (590, 2, false, 590, 3, 12),
            (590, 3, false, 590, 4, 10),
            (590, 4, false, 590, 5, 10),
            (590, 5, false, 590, 6, 8),
            (590, 6, false, 590, 7, 8),
            (590, 7, false, 590, 8, 6),
            (590, 8, false, 590, 9, 5),
            (590, 9, false, 590, 10, 4),
            (590, 10, false, 590, 11, 3),
            (590, 11, false, 590, 12, 3),
            (590, 12, false, 591, 1, 1),
            (596, 1, false, 596, 2, 4),
            (596, 2, false, 596, 3, 5),
            (596, 3, false, 596, 4, 4),
            (596, 4, false, 596, 5, 3),
            (596, 5, false, 596, 6, 2),
            (596, 6, false, 596, 7, 1),
            (596, 7, false, 596, 7, 31),
            (596, 8, false, 596, 8, 29),
            (596, 9, false, 596, 9, 28),
            (596, 10, false, 596, 10, 27),
            (596, 11, false, 596, 11, 26),
            (596, 12, false, 596, 12, 25),
        ],
    );
}

/// The 三紀甲子元曆's first and last years, 384 and 417, the last with
/// a leap month after the twelfth, and 400.
#[test]
fn sanji_years_are_the_tables() {
    check_table(
        sanji::ENGINE,
        &[
            (384, 1, false, 384, 2, 8),
            (384, 2, false, 384, 3, 9),
            (384, 3, false, 384, 4, 7),
            (384, 4, false, 384, 5, 7),
            (384, 5, false, 384, 6, 5),
            (384, 6, false, 384, 7, 5),
            (384, 7, false, 384, 8, 3),
            (384, 8, false, 384, 9, 2),
            (384, 9, false, 384, 10, 1),
            (384, 10, false, 384, 10, 31),
            (384, 11, false, 384, 11, 29),
            (384, 12, false, 384, 12, 29),
            (400, 1, false, 400, 2, 11),
            (400, 2, false, 400, 3, 12),
            (400, 3, false, 400, 4, 10),
            (400, 4, false, 400, 5, 10),
            (400, 5, false, 400, 6, 8),
            (400, 6, false, 400, 7, 8),
            (400, 7, false, 400, 8, 6),
            (400, 8, false, 400, 9, 5),
            (400, 9, false, 400, 10, 4),
            (400, 10, false, 400, 11, 3),
            (400, 11, false, 400, 12, 2),
            (400, 12, false, 401, 1, 1),
            (417, 1, false, 417, 2, 3),
            (417, 2, false, 417, 3, 4),
            (417, 3, false, 417, 4, 3),
            (417, 4, false, 417, 5, 2),
            (417, 5, false, 417, 6, 1),
            (417, 6, false, 417, 6, 30),
            (417, 7, false, 417, 7, 30),
            (417, 8, false, 417, 8, 28),
            (417, 9, false, 417, 9, 27),
            (417, 10, false, 417, 10, 26),
            (417, 11, false, 417, 11, 25),
            (417, 12, false, 417, 12, 24),
            (417, 12, true, 418, 1, 23),
        ],
    );
}

/// The 正光曆's first year, 523, a year of the Eastern Wei, 537, with a
/// leap month after the ninth, and 558, the Western Wei's last.
#[test]
fn zhengguang_years_are_the_tables() {
    check_table(
        zhengguang::ENGINE,
        &[
            (523, 1, false, 523, 2, 1),
            (523, 2, false, 523, 3, 3),
            (523, 3, false, 523, 4, 1),
            (523, 4, false, 523, 5, 1),
            (523, 5, false, 523, 5, 30),
            (523, 6, false, 523, 6, 29),
            (523, 7, false, 523, 7, 28),
            (523, 8, false, 523, 8, 27),
            (523, 9, false, 523, 9, 25),
            (523, 10, false, 523, 10, 25),
            (523, 11, false, 523, 11, 23),
            (523, 12, false, 523, 12, 23),
            (537, 1, false, 537, 1, 27),
            (537, 2, false, 537, 2, 25),
            (537, 3, false, 537, 3, 27),
            (537, 4, false, 537, 4, 26),
            (537, 5, false, 537, 5, 25),
            (537, 6, false, 537, 6, 24),
            (537, 7, false, 537, 7, 23),
            (537, 8, false, 537, 8, 22),
            (537, 9, false, 537, 9, 20),
            (537, 9, true, 537, 10, 20),
            (537, 10, false, 537, 11, 18),
            (537, 11, false, 537, 12, 18),
            (537, 12, false, 538, 1, 16),
            (558, 1, false, 558, 2, 4),
            (558, 2, false, 558, 3, 5),
            (558, 3, false, 558, 4, 4),
            (558, 4, false, 558, 5, 3),
            (558, 5, false, 558, 6, 2),
            (558, 6, false, 558, 7, 2),
            (558, 7, false, 558, 7, 31),
            (558, 8, false, 558, 8, 30),
            (558, 9, false, 558, 9, 28),
            (558, 10, false, 558, 10, 28),
            (558, 11, false, 558, 11, 26),
            (558, 12, false, 558, 12, 26),
        ],
    );
}

// ---------------------------------------------------------------------------
// The spans abut, and refuse beyond

#[test]
fn the_spans_abut_and_refuse_outside() {
    assert_eq!(taichu::LATEST.0 + 1, sifen::EARLIEST.0);
    assert_eq!(jingchu::LATEST.0 + 1, yuanjia::EARLIEST.0);
    assert_eq!(yuanjia::LATEST.0 + 1, daming::EARLIEST.0);
    for (engine, first, last) in [
        (taichu::ENGINE, taichu::EARLIEST, taichu::LATEST),
        (sifen::ENGINE, sifen::EARLIEST, sifen::LATEST),
        (qianxiang::ENGINE, qianxiang::EARLIEST, qianxiang::LATEST),
        (jingchu::ENGINE, jingchu::EARLIEST, jingchu::LATEST),
        (yuanjia::ENGINE, yuanjia::EARLIEST, yuanjia::LATEST),
        (daming::ENGINE, daming::EARLIEST, daming::LATEST),
        (xinghe::ENGINE, xinghe::EARLIEST, xinghe::LATEST),
        (tianhe::ENGINE, tianhe::EARLIEST, tianhe::LATEST),
        (kaihuang::ENGINE, kaihuang::EARLIEST, kaihuang::LATEST),
        (sanji::ENGINE, sanji::EARLIEST, sanji::LATEST),
        (zhengguang::ENGINE, zhengguang::EARLIEST, zhengguang::LATEST),
    ] {
        let parameters = engine.parameters();
        assert!(parameters.from_fixed(first).is_ok());
        assert!(parameters.from_fixed(last).is_ok());
        assert_eq!(
            parameters.from_fixed(Rd(first.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            parameters.from_fixed(Rd(last.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }
}

// ---------------------------------------------------------------------------
// The oracle

/// A system's two ratios and epoch, in integers.
struct System {
    engine: LunisolarCalendar,
    earliest: Rd,
    latest: Rd,
    epoch: i64,
    /// 朔: numerator and denominator, in days.
    month: (i128, i128),
    /// 歲: numerator and denominator, in days.
    year: (i128, i128),
    /// The number of the month the zhōngqì at the epoch belongs to: 11 for
    /// 冬至, 1 for 雨水.
    term_at_epoch: i64,
}

fn systems() -> [System; 11] {
    [
        System {
            engine: taichu::ENGINE,
            earliest: taichu::EARLIEST,
            latest: taichu::LATEST,
            epoch: taichu::EPOCH.0,
            month: (2_392, 81),
            year: (562_120, 1_539),
            term_at_epoch: 11,
        },
        System {
            engine: sifen::ENGINE,
            earliest: sifen::EARLIEST,
            latest: sifen::LATEST,
            epoch: sifen::EPOCH.0,
            month: (27_759, 940),
            year: (1_461, 4),
            term_at_epoch: 11,
        },
        System {
            engine: qianxiang::ENGINE,
            earliest: qianxiang::EARLIEST,
            latest: qianxiang::LATEST,
            epoch: qianxiang::EPOCH.0,
            month: (43_026, 1_457),
            year: (215_130, 589),
            term_at_epoch: 11,
        },
        System {
            engine: jingchu::ENGINE,
            earliest: jingchu::EARLIEST,
            latest: jingchu::LATEST,
            epoch: jingchu::EPOCH.0,
            month: (134_630, 4_559),
            year: (673_150, 1_843),
            term_at_epoch: 11,
        },
        System {
            engine: yuanjia::ENGINE,
            earliest: yuanjia::EARLIEST,
            latest: yuanjia::LATEST,
            epoch: yuanjia::EPOCH.0,
            month: (22_207, 752),
            year: (111_035, 304),
            term_at_epoch: 1,
        },
        System {
            engine: daming::ENGINE,
            earliest: daming::EARLIEST,
            latest: daming::LATEST,
            epoch: daming::EPOCH.0,
            month: (116_321, 3_939),
            year: (14_423_804, 39_491),
            term_at_epoch: 11,
        },
        System {
            engine: xinghe::ENGINE,
            earliest: xinghe::EARLIEST,
            latest: xinghe::LATEST,
            epoch: xinghe::EPOCH.0,
            month: (6_158_017, 208_530),
            year: (6_158_017, 16_860),
            term_at_epoch: 11,
        },
        System {
            engine: tianhe::ENGINE,
            earliest: tianhe::EARLIEST,
            latest: tianhe::LATEST,
            epoch: tianhe::EPOCH.0,
            month: (8_568_631, 290_160),
            year: (8_568_631, 23_460),
            term_at_epoch: 11,
        },
        System {
            engine: kaihuang::ENGINE,
            earliest: kaihuang::EARLIEST,
            latest: kaihuang::LATEST,
            epoch: kaihuang::EPOCH.0,
            month: (5_372_209, 181_920),
            year: (37_605_463, 102_960),
            term_at_epoch: 11,
        },
        System {
            engine: sanji::ENGINE,
            earliest: sanji::EARLIEST,
            latest: sanji::LATEST,
            epoch: sanji::EPOCH.0,
            month: (895_220, 30_315),
            year: (895_220, 2_451),
            term_at_epoch: 11,
        },
        System {
            engine: zhengguang::ENGINE,
            earliest: zhengguang::EARLIEST,
            latest: zhengguang::LATEST,
            epoch: zhengguang::EPOCH.0,
            month: (2_213_377, 74_952),
            year: (2_213_377, 6_060),
            term_at_epoch: 11,
        },
    ]
}

/// One month of the oracle.
#[derive(Debug, Clone, Copy)]
struct OracleMonth {
    start: i64,
    ordinal: u8,
    leap: bool,
    year: i64,
}

fn floor_div(a: i128, b: i128) -> i128 {
    a.div_euclid(b)
}

fn oracle(system: &System) -> Vec<OracleMonth> {
    let epoch = i128::from(system.epoch);
    let (mn, md) = system.month;
    let (yn, yd) = system.year;
    let start_of = |k: i128| epoch + floor_div(k * mn, md);
    // The zhōngqì are the epoch plus whole twelfths of the year.
    let term_day = |i: i128| epoch + floor_div(i * yn, 12 * yd);
    let lo = i128::from(system.earliest.0) - 450;
    let hi = i128::from(system.latest.0) + 90;
    let mut k = floor_div((lo - epoch) * md, mn) - 2;
    let mut starts = vec![];
    loop {
        let start = start_of(k);
        if start > hi {
            break;
        }
        starts.push(start);
        k += 1;
    }
    let mut i = floor_div((lo - epoch) * 12 * yd, yn) - 3;
    let mut terms = vec![];
    loop {
        let day = term_day(i);
        if day > hi + 40 {
            break;
        }
        terms.push((i, day));
        i += 1;
    }
    let mut months: Vec<OracleMonth> = vec![];
    for window in starts.windows(2) {
        let (start, next) = (window[0], window[1]);
        let inside = terms.iter().find(|&&(_, day)| day >= start && day < next);
        let (ordinal, leap) = match inside {
            Some(&(index, _)) => (
                u8::try_from((i128::from(system.term_at_epoch - 1) + index).rem_euclid(12) + 1)
                    .unwrap_or_else(|_| panic!("a month number")),
                false,
            ),
            None => (months.last().map_or(0, |month| month.ordinal), true),
        };
        months.push(OracleMonth {
            start: i64::try_from(start).unwrap_or_else(|_| panic!("a day")),
            ordinal,
            leap,
            year: 0,
        });
    }
    // A month belongs to the year whose first month, the one numbered 1,
    // began at or before it; the year is the Gregorian year it began in.
    let mut year = None;
    for month in &mut months {
        if month.ordinal == 1 && !month.leap {
            year = Some(gregorian::year_from_fixed(Rd(month.start)));
        }
        month.year = year.unwrap_or(0);
    }
    months.retain(|month| month.year != 0 || month.start > system.earliest.0);
    months
}

/// Every month of every system, from the engine's floating point, is the
/// month the integers give: its first day, its length, its number, whether
/// it is the leap month and its year.
#[test]
fn the_engine_agrees_with_integer_arithmetic_on_every_month() {
    for system in systems() {
        let parameters = system.engine.parameters();
        let months = oracle(&system);
        let mut compared = 0;
        for (index, month) in months.iter().enumerate() {
            if month.start < system.earliest.0 || month.start > system.latest.0 {
                continue;
            }
            let rd = Rd(month.start);
            let (year, found, day) = parameters.from_fixed(rd).expect("in range");
            assert_eq!(
                (year, found.ordinal, found.leap, day),
                (month.year, month.ordinal, month.leap, 1),
                "{} {}",
                parameters.english_name,
                month.start
            );
            if let Some(next) = months.get(index + 1)
                && next.start <= system.latest.0
            {
                assert_eq!(
                    parameters.days_in_month(year, found).map(i64::from),
                    Some(next.start - month.start),
                    "{} {}",
                    parameters.english_name,
                    month.start
                );
            }
            compared += 1;
        }
        assert!(
            compared > 130,
            "{} compared {compared}",
            parameters.english_name
        );
    }
}

/// The oracle is two ratios and an epoch and nothing else, so every
/// month it makes is 29 or 30 days, a year has 12 or 13 months, the leap
/// month follows the month it repeats, and a long run holds the 章's seven
/// leap months in nineteen years, within one.
#[test]
fn months_years_and_leap_months_have_the_shape_of_the_rule() {
    for system in systems() {
        let months = oracle(&system);
        let name = system.engine.parameters().english_name;
        for pair in months.windows(2) {
            let length = pair[1].start - pair[0].start;
            assert!(length == 29 || length == 30, "{name} {length}");
            if pair[1].leap {
                assert!(!pair[0].leap, "{name}: two leap months in a row");
                assert_eq!(pair[1].ordinal, pair[0].ordinal, "{name}");
            }
        }
        let leaps = months.iter().filter(|month| month.leap).count();
        let years = months
            .iter()
            .filter(|month| month.ordinal == 1 && !month.leap)
            .count();
        // 7 in 19, so about 0.368 a year.
        let expected = years as f64 * 7.0 / 19.0;
        assert!(
            (leaps as f64 - expected).abs() < 6.0,
            "{name}: {leaps} leap months in {years} years"
        );
    }
}

/// Every day of every span converts to a date and back, in a release
/// build; a debug build takes every seventh day and the days around every
/// month boundary of the first and last years.
#[test]
fn every_day_round_trips() {
    let stride = if cfg!(debug_assertions) { 7 } else { 1 };
    for system in systems() {
        let parameters = system.engine.parameters();
        let mut rd = system.earliest.0;
        while rd <= system.latest.0 {
            let (year, month, day) = parameters.from_fixed(Rd(rd)).expect("in range");
            assert_eq!(
                parameters.to_fixed(year, month, day),
                Ok(Rd(rd)),
                "{} {rd}",
                parameters.english_name
            );
            rd += stride;
        }
        // The ends, whatever the stride.
        for rd in [system.earliest.0, system.latest.0] {
            let (year, month, day) = parameters.from_fixed(Rd(rd)).expect("in range");
            assert_eq!(parameters.to_fixed(year, month, day), Ok(Rd(rd)));
        }
    }
}

/// The calendars are registered under their own identifiers, with their
/// span as their usage and the 夏正's months.
#[test]
fn the_registered_calendars_carry_their_spans() {
    use hc_calendar::CalendarRegistry;
    let mut registry = CalendarRegistry::new();
    hc_calendars_lunar::register_all(&mut registry);
    for (id, first, last) in [
        ("chinese-taichu", taichu::EARLIEST, taichu::LATEST),
        ("chinese-sifen", sifen::EARLIEST, sifen::LATEST),
        ("chinese-qianxiang", qianxiang::EARLIEST, qianxiang::LATEST),
        ("chinese-jingchu", jingchu::EARLIEST, jingchu::LATEST),
        ("chinese-yuanjia", yuanjia::EARLIEST, yuanjia::LATEST),
        ("chinese-daming", daming::EARLIEST, daming::LATEST),
        ("chinese-xinghe", xinghe::EARLIEST, xinghe::LATEST),
        ("chinese-tianhe", tianhe::EARLIEST, tianhe::LATEST),
        ("chinese-kaihuang", kaihuang::EARLIEST, kaihuang::LATEST),
        ("chinese-sanji", sanji::EARLIEST, sanji::LATEST),
        (
            "chinese-zhengguang",
            zhengguang::EARLIEST,
            zhengguang::LATEST,
        ),
    ] {
        let calendar = registry.get_by_name(id).expect("registered");
        let meta = calendar.meta();
        assert_eq!(
            (meta.earliest, meta.latest),
            (Some(first), Some(last)),
            "{id}"
        );
        assert!(meta.has_leap_months && meta.is_astronomical, "{id}");
        assert_eq!(meta.native_locales, &["zh"], "{id}");
    }
}

/// A date of a calendar with this engine is a [`LunisolarDate`]: the
/// year, the month with its leap flag, the day.
#[test]
fn a_date_is_year_month_and_day() {
    let calendar = yuanjia::YuanjiaCalendar;
    let rd = julian_day(445, 6, 21);
    let date = calendar.from_fixed(rd).expect("in range");
    assert_eq!(date, LunisolarDate::new(445, Month::leap(5), 1));
    assert!(date.is_in_leap_month());
    assert_eq!(calendar.to_fixed(date), Ok(rd));
}
