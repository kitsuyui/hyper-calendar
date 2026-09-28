//! `hc-i18n`'s names of the days of holiday tables name days the tables
//! have.

#![cfg(all(feature = "holiday", feature = "i18n"))]

use hyper_calendar::hc_holiday::traditions;
use hyper_calendar::hc_i18n::holiday_names::TABLES;

#[test]
fn every_named_day_is_a_day_of_its_table() {
    for names in TABLES {
        let table = traditions::ALL
            .iter()
            .find(|set| set.code == names.table)
            .unwrap_or_else(|| panic!("no table {}", names.table));
        for (english, _) in names.names {
            assert!(
                table.rules.iter().any(|rule| rule.name == *english),
                "{}: no day {english}",
                names.table
            );
        }
    }
}
