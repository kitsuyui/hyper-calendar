//! Easter, computed both ways, against the published tables.
//!
//! ```sh
//! cargo run -p hyper-calendar --example easter --features holiday
//! ```

use hyper_calendar::hc_calendars_solar::gregorian;
use hyper_calendar::hc_holiday::computus;

fn main() {
    println!("year  Gregorian     Orthodox (civil)   gap");
    for year in 2020..=2035 {
        let Some((west_m, west_d)) = computus::gregorian_easter_date(year) else {
            continue;
        };
        let Some((east_y, east_m, east_d)) = computus::orthodox_easter_gregorian_date(year) else {
            continue;
        };
        let gap = match (
            computus::gregorian_easter(year),
            computus::orthodox_easter(year),
        ) {
            (Some(west), Some(east)) => east.get() - west.get(),
            _ => 0,
        };
        let note = if gap == 0 { "  same day" } else { "" };
        println!(
            "{year}  {west_y:04}-{west_m:02}-{west_d:02}    {east_y:04}-{east_m:02}-{east_d:02}      {gap:>2} d{note}",
            west_y = year
        );
    }

    println!("\nThe ecclesiastical moon is a table, not the sky:");
    for year in [2019, 2038, 2049] {
        if let Some((month, day)) = computus::gregorian_easter_date(year) {
            let rd = gregorian::to_fixed(year, month, day);
            println!(
                "  {year}-{month:02}-{day:02}  (RD {:?})",
                rd.map(|r| r.get())
            );
        }
    }
}
