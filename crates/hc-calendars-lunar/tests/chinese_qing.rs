//! The Chinese calendar against the Veritable Records of the Qing.
//!
//! `data/qing_veritable_records_month_starts.txt` holds the first day of
//! each month of 1645–1911 whose opening line the transcription's pattern
//! caught, by sexagenary name, as the Veritable Records (《清實錄》,
//! `qing-shilu`) open the month; its header says how it was read and what
//! was left out. The Records are the court's own account of each reign,
//! dated by the calendar the court kept. They are a record apart from Liu's
//! reconstruction and the Observatory's table, from which the almanac
//! corrections of `chinese` were taken, with one exception: the 處暑 of
//! 1805 behind that year's leap month was inferred from the Records' own
//! 閏六月, so for the two months it moves this test is circular.

use hc_calendar::{Calendar, Month};
use hc_calendars_lunar::ChineseCalendar;
use hc_calendars_lunar::chinese;
use hc_calendars_lunar::lunisolar::{LunisolarDate, LunisolarParameters};

/// The Records' first days.
const TABLE: &str = include_str!("data/qing_veritable_records_month_starts.txt");

/// The reigns, as the transcription names them, and the Gregorian year in
/// which each one's first year began.
const REIGNS: [(&str, i64); 10] = [
    ("顺治", 1644),
    ("康熙", 1662),
    ("雍正", 1723),
    ("乾隆", 1736),
    ("嘉庆", 1796),
    ("道光", 1821),
    ("咸丰", 1851),
    ("同治", 1862),
    ("光绪", 1875),
    ("宣统", 1909),
];

const STEMS: [char; 10] = ['甲', '乙', '丙', '丁', '戊', '己', '庚', '辛', '壬', '癸'];
const BRANCHES: [char; 12] = [
    '子', '丑', '寅', '卯', '辰', '巳', '午', '未', '申', '酉', '戌', '亥',
];

/// Every line of the table as `(Chinese year, month, sexagenary name)`.
fn months() -> Vec<(i64, Month, String)> {
    let mut out = Vec::new();
    for line in TABLE.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let fields: Vec<&str> = line.split_whitespace().collect();
        let [reign, year, month, name] = fields[..] else {
            panic!("malformed line {line:?}");
        };
        let first_year = REIGNS
            .iter()
            .find(|(known, _)| *known == reign)
            .unwrap_or_else(|| panic!("unknown reign in {line:?}"))
            .1;
        let year: i64 = year
            .parse()
            .unwrap_or_else(|_| panic!("{year:?} in {line:?} is not a year"));
        let (leap, ordinal) = month
            .strip_prefix('L')
            .map_or((false, month), |ordinal| (true, ordinal));
        let ordinal: u8 = ordinal
            .parse()
            .unwrap_or_else(|_| panic!("{month:?} in {line:?} is not a month"));
        let month = if leap {
            Month::leap(ordinal)
        } else {
            Month::regular(ordinal)
        };
        // The Chinese year that began in 2024 is 4661.
        out.push((first_year + year - 1 + 2_637, month, name.to_owned()));
    }
    out
}

/// The sexagenary name of the first day of a month of a calendar.
fn first_day_name(calendar: &LunisolarParameters, year: i64, month: Month) -> Option<String> {
    let first = calendar.to_fixed(year, month, 1).ok()?;
    let day = calendar.sexagenary_day(first);
    Some(
        [
            STEMS[day.stem_index() as usize],
            BRANCHES[day.branch_index() as usize],
        ]
        .iter()
        .collect(),
    )
}

#[test]
fn the_table_is_read_whole() {
    let months = months();
    let stated: usize = TABLE
        .lines()
        .find_map(|line| line.strip_prefix("# Lines: "))
        .and_then(|count| count.trim().parse().ok())
        .expect("the header states the number of lines");
    assert_eq!(months.len(), stated);
    // Each month once.
    let mut keys: Vec<(i64, Month)> = months.iter().map(|m| (m.0, m.1)).collect();
    keys.sort_by_key(|(year, month)| (*year, month.ordinal, month.leap));
    keys.dedup();
    assert_eq!(keys.len(), months.len());
}

#[test]
fn every_month_the_veritable_records_open_begins_on_their_day() {
    for (year, month, name) in months() {
        let date = LunisolarDate::new(year, month, 1);
        let first = ChineseCalendar
            .to_fixed(date)
            .unwrap_or_else(|error| panic!("{year} {month:?} is not a month: {error:?}"));
        assert_eq!(
            first_day_name(&chinese::PARAMETERS, year, month).as_deref(),
            Some(name.as_str()),
            "{year} {month:?}, which the calendar begins on RD {first:?}"
        );
    }
}

#[test]
fn without_the_corrections_the_rules_miss_the_months_the_corrections_carry() {
    static RULES: LunisolarParameters = LunisolarParameters {
        month_start_corrections: &[],
        major_term_corrections: &[],
        ..chinese::PARAMETERS
    };
    let calendar = &chinese::PARAMETERS;
    let first_day = |year: i64, month: Month| {
        calendar
            .to_fixed(year, month, 1)
            .unwrap_or_else(|error| panic!("{year} {month:?} is not a month: {error:?}"))
    };
    let months = months();
    // Every correction the engine applies has a month in the Records to
    // test it: the first day of a month for each of the 29 first days, and
    // the leap month of each of the five years whose term day it moves.
    for correction in &chinese::ALMANAC_CORRECTIONS {
        assert!(
            months
                .iter()
                .any(|(year, month, _)| first_day(*year, *month) == correction.promulgated),
            "no month of the Records begins on {:?}: {}",
            correction.promulgated,
            correction.source
        );
    }
    let term_years: Vec<i64> = chinese::ALMANAC_TERM_CORRECTIONS
        .iter()
        .map(|correction| {
            calendar
                .from_fixed(correction.promulgated)
                .expect("in range")
                .0
        })
        .collect();
    for (year, correction) in term_years.iter().zip(&chinese::ALMANAC_TERM_CORRECTIONS) {
        let leap = calendar
            .leap_month(*year)
            .expect("in range")
            .expect("the correction places a leap month");
        assert!(
            months
                .iter()
                .any(|(y, month, _)| y == year && *month == Month::leap(leap)),
            "the Records do not open the leap month of {year}: {}",
            correction.source
        );
    }
    // Where the rules miss: the 29 months whose first day a correction
    // moves, and in each of the five years, the leap month and the month
    // the rules number differently beside it.
    let missed: Vec<&(i64, Month, String)> = months
        .iter()
        .filter(|(year, month, name)| {
            first_day_name(&RULES, *year, *month).as_deref() != Some(name.as_str())
        })
        .collect();
    let moved_first_days = missed
        .iter()
        .filter(|(year, month, _)| {
            chinese::ALMANAC_CORRECTIONS
                .iter()
                .any(|correction| correction.promulgated == first_day(*year, *month))
        })
        .count();
    let in_term_years = missed
        .iter()
        .filter(|(year, _, _)| term_years.contains(year))
        .count();
    assert_eq!(moved_first_days, chinese::ALMANAC_CORRECTIONS.len());
    assert_eq!(in_term_years, 2 * chinese::ALMANAC_TERM_CORRECTIONS.len());
    assert_eq!(missed.len(), moved_first_days + in_term_years);
    assert_eq!(missed.len(), 39);
}
