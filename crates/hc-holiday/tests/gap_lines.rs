//! A gap is written once (audit 10 a37).
//!
//! Quebec's table has two rules for "Good Friday or Easter Monday, at the
//! employer's choice", one for each day the employer may pick, with one
//! identifier; the year the table could not settle wrote the same gap line
//! twice.

use hc_holiday::countries::CANADA;
use hc_holiday::engine::HolidayCalendar;

#[test]
fn a_gap_two_rules_share_is_written_once() {
    let calendar = HolidayCalendar::for_year(&CANADA, Some("CA-QC"), 2025);
    let named: Vec<_> = calendar
        .gaps()
        .iter()
        .filter(|gap| gap.name.contains("Good Friday or Easter Monday"))
        .collect();
    assert_eq!(named.len(), 1, "{named:?}");
    // No two gaps of a year share an identifier, in any region.
    for region in [None, Some("CA-QC"), Some("CA-ON")] {
        for year in 2024..=2027 {
            let calendar = HolidayCalendar::for_year(&CANADA, region, year);
            let gaps = calendar.gaps();
            for (index, gap) in gaps.iter().enumerate() {
                assert!(
                    gaps[index + 1..]
                        .iter()
                        .all(|other| !(other.year == gap.year && other.id == gap.id)),
                    "{region:?} {year}: {} twice",
                    gap.name
                );
            }
        }
    }
}
