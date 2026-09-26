//! The Chinese calendar against the Veritable Records of the Qing.
//!
//! `data/qing_veritable_records_month_starts.txt` holds the first day of
//! each month of 1645–1911, by sexagenary name, as the Veritable Records
//! (《清實錄》, `qing-shilu`) open the month; its header says how it was
//! read and what was left out. The Records are the court's own account of
//! each reign, dated by the calendar the court kept, so they test the
//! almanac corrections of `chinese` against a record independent of the
//! reconstruction they came from.

use hc_calendar::{Calendar, Month};
use hc_calendars_lunar::ChineseCalendar;
use hc_calendars_lunar::chinese;
use hc_calendars_lunar::lunisolar::{LunisolarDate, LunisolarParameters};

/// The Records' first days.
const TABLE: &str = include_str!("data/qing_veritable_records_month_starts.txt");

/// The reigns, as the transcription names them, and the Gregorian year in
/// which each one's first year began.
const REIGNS: [(&str, i64); 9] = [
    ("顺治", 1644),
    ("康熙", 1662),
    ("雍正", 1723),
    ("乾隆", 1736),
    ("嘉庆", 1796),
    ("道光", 1821),
    ("咸丰", 1851),
    ("同治", 1862),
    ("光绪", 1875),
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
    let missed = months()
        .into_iter()
        .filter(|(year, month, name)| {
            first_day_name(&RULES, *year, *month).as_deref() != Some(name.as_str())
        })
        .count();
    // The 28 first days the corrections move before 1900, and 9 months of
    // the five years whose leap month the rules put a lunation away, which
    // the rules number differently.
    assert_eq!(missed, 37);
}
