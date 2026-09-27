//! The Unified Hijri Calendars and the FCNA table against the bodies'
//! published month starts.
//!
//! `data/khgt_month_starts.txt` holds Muhammadiyah's calendar page for
//! 1447–1492 AH, `data/diyanet_month_starts.txt` Diyanet's lists of
//! religious days for 2022–2035 and `data/fcna_month_starts.txt` the Fiqh
//! Council of North America's calendar for 1440–1467 AH; each file's header
//! says where and when it was read. `docs/systems/unified-hijri.md` gives
//! what the figures below mean.

use hc_calendar::Rd;
use hc_calendar::gregorian::to_fixed;
use hc_calendars_lunar::islamic_fcna;
use hc_calendars_lunar::{IslamicFcnaCalendar, IslamicGlobalCalendar};

const KHGT: &str = include_str!("data/khgt_month_starts.txt");
const DIYANET: &str = include_str!("data/diyanet_month_starts.txt");
const FCNA: &str = include_str!("data/fcna_month_starts.txt");

/// Every row of a table as `(Hijri year, month, first day)`.
fn starts(table: &str) -> Vec<(i64, u8, Rd)> {
    table
        .lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .map(|line| {
            let fields: Vec<i64> = line
                .split_whitespace()
                .map(|field| {
                    field
                        .parse()
                        .unwrap_or_else(|_| panic!("{field:?} in {line:?} is not a number"))
                })
                .collect();
            let [year, month, g_year, g_month, g_day] = fields[..] else {
                panic!("malformed line {line:?}");
            };
            let first = to_fixed(g_year, g_month as u8, g_day as u8)
                .unwrap_or_else(|_| panic!("{line:?} names no Gregorian day"));
            (year, month as u8, first)
        })
        .collect()
}

/// The rows whose first day the calendar puts elsewhere, as `(year,
/// month, the calendar's day less the published one)`.
fn differences(calendar: &IslamicGlobalCalendar, rows: &[(i64, u8, Rd)]) -> Vec<(i64, u8, i64)> {
    rows.iter()
        .filter_map(|&(year, month, first)| {
            let ours = calendar
                .compose(year, month, 1)
                .unwrap_or_else(|error| panic!("{year}-{month}: {error:?}"));
            (ours != first).then_some((year, month, ours.0 - first.0))
        })
        .collect()
}

/// Diyanet's lists against `islamic-istanbul-2016`: 171 of the 174 month
/// starts of 1443–1457 AH. Dhū al-Qaʿda 1444 and Dhū al-Ḥijja 1453 begin a
/// day earlier here; Rajab 1453 a day later, the evening of 16 October 2031
/// meeting the parameters, by hundredths of a degree of elongation, only a
/// little west of the line of places the calendar judges, among the
/// Patagonian fjords.
#[test]
fn diyanet_s_published_months_are_reproduced_but_three() {
    let rows = starts(DIYANET);
    assert_eq!(rows.len(), 174);
    hc_core::memo::scope(|| {
        assert_eq!(
            differences(&IslamicGlobalCalendar::ISTANBUL_2016, &rows),
            [(1_444, 11, -1), (1_453, 7, 1), (1_453, 12, -1)]
        );
    });
}

/// Muhammadiyah's calendar against `islamic-khgt`: every month of 1447 to
/// 1449, the years in force and just ahead, and, in a release build, all
/// 551 months the page gives for 1447–1492: 493 on the same day, 52 a day
/// later here and 6 a day earlier, all from 1450 on. The page's later years
/// follow a reading of the rules this library could not find in the
/// texts.
#[test]
fn khgt_s_published_months_are_reproduced_for_1447_to_1449() {
    let rows = starts(KHGT);
    assert_eq!(rows.len(), 551);
    hc_core::memo::scope(|| {
        let calendar = IslamicGlobalCalendar::KHGT;
        let early: Vec<_> = rows.iter().copied().filter(|row| row.0 <= 1_449).collect();
        assert_eq!(early.len(), 36);
        assert_eq!(differences(&calendar, &early), []);
        if cfg!(debug_assertions) {
            return;
        }
        let all = differences(&calendar, &rows);
        let later = all.iter().filter(|(_, _, by)| *by == 1).count();
        let earlier = all.iter().filter(|(_, _, by)| *by == -1).count();
        assert_eq!((all.len(), later, earlier), (58, 52, 6), "{all:?}");
        assert!(all.iter().all(|(year, _, _)| *year >= 1_450));
    });
}

/// The FCNA table is the Council's page, row for row, as far as it runs.
#[test]
fn the_fcna_table_is_the_councils_page() {
    use hc_calendar::Calendar;
    let rows = starts(FCNA);
    assert_eq!(rows.len(), 335);
    let mut carried = 0;
    for (year, month, first) in rows {
        match islamic_fcna::to_fixed(year, month, 1) {
            Ok(rd) => {
                assert_eq!(rd, first, "{year}-{month}");
                carried += 1;
            }
            Err(_) => assert!(first > islamic_fcna::LATEST, "{year}-{month}"),
        }
    }
    assert_eq!(carried, 306);
    assert_eq!(
        IslamicFcnaCalendar.meta().latest,
        Some(islamic_fcna::LATEST)
    );
}

/// No computed rule stands in for the FCNA table: in a release build, the
/// Unified Hijri rules date 30 (KHGT's, geocentric altitude) and 36
/// (Diyanet's, topocentric) of its 335 months otherwise.
#[test]
fn neither_unified_rule_reproduces_the_fcna_table() {
    if cfg!(debug_assertions) {
        return;
    }
    let rows = starts(FCNA);
    hc_core::memo::scope(|| {
        let khgt = differences(&IslamicGlobalCalendar::KHGT, &rows).len();
        let istanbul = differences(&IslamicGlobalCalendar::ISTANBUL_2016, &rows).len();
        assert_eq!((khgt, istanbul), (30, 36));
    });
}
