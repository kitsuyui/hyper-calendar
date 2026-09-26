//! The Gregorian adoption table against the calendars it names, which only
//! the facade has in one place.
//!
//! `hc-calendars-solar` holds the table and checks it against its own
//! reform, Swedish and Rumi calendars; what it cannot see is the Japanese,
//! Korean and Chinese lunisolar calendars and the Umm al-Qura calendar,
//! which live in `hc-calendars-lunar`. These tests hold each row's old
//! calendar to the registry, and the last lunisolar day of Japan, Korea and
//! China to the day those calendars' own modules say they stopped being
//! civil.

#![cfg(all(
    feature = "alloc",
    feature = "civil",
    feature = "lunar",
    feature = "regional",
    feature = "i18n",
    feature = "format",
))]
#![expect(
    clippy::expect_used,
    reason = "a fixture that does not convert is a failed test, and the \
              message says which"
)]

use hyper_calendar::hc_calendar::Rd;
use hyper_calendar::hc_calendars_lunar::{chinese, dangi, japanese_tenpo};
use hyper_calendar::hc_calendars_solar::adoption::{REGIONAL_ADOPTIONS, Scope};
use hyper_calendar::hc_calendars_solar::gregorian;
use hyper_calendar::{gregorian_adoption, lines};

fn day(year: i64, month: u8, day: u8) -> Rd {
    gregorian::to_fixed(year, month, day).expect("a Gregorian date")
}

#[test]
fn every_calendar_a_row_names_is_registered() {
    let registry = hyper_calendar::registry();
    for row in REGIONAL_ADOPTIONS {
        for id in [row.old_calendar, row.new_calendar] {
            assert!(registry.get_by_name(id).is_some(), "{id} in {row:?}");
        }
    }
}

#[test]
fn the_east_asian_rows_end_where_the_lunisolar_calendars_stopped_being_civil() {
    let japan: Vec<_> = gregorian_adoption("JP").collect();
    assert_eq!(japan.len(), 1);
    assert_eq!(japan[0].last_old_day().ok(), Some(japanese_tenpo::LATEST));
    assert_eq!(japan[0].first_day().ok(), Some(day(1873, 1, 1)));

    for region in ["KR", "KP"] {
        let korea: Vec<_> = gregorian_adoption(region).collect();
        assert_eq!(korea.len(), 1);
        assert_eq!(korea[0].last_old_day().ok(), Some(dangi::LAST_CIVIL));
        assert_eq!(korea[0].old_calendar, "dangi");
    }

    let china: Vec<_> = gregorian_adoption("CN").collect();
    assert_eq!(china.len(), 2);
    assert_eq!(china[0].last_old_day().ok(), Some(chinese::LAST_CIVIL));
    assert_eq!(china[0].scope, Scope::Partial);
    assert_eq!(china[1].first_day().ok(), Some(day(1929, 1, 1)));
    assert_eq!(china[1].scope, Scope::Civil);
}

#[test]
fn the_lines_carry_every_column() {
    let text = lines::gregorian_adoption("sa");
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].len(), lines::GREGORIAN_ADOPTION_COLUMNS);
    assert_eq!(rows[0][0], day(2016, 9, 30).0.to_string());
    assert_eq!(rows[0][1], day(2016, 10, 1).0.to_string());
    assert_eq!(rows[0][2..4], ["islamic-umalqura", "partial"]);
    assert!(rows[0][4].contains("Council of Ministers"));
    assert_eq!(rows[0][5], "gregory");
    assert!(lines::gregorian_adoption("XX").is_empty());
}
