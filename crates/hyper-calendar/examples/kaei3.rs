//! One Japanese lunisolar year, read in every calendar the build provides.
//!
//! Kaei 3 (嘉永3年) is a good demonstration case because it is a *lunisolar*
//! year, so it does not line up with a Gregorian one: it began on
//! 1850-02-12 and ended on 1851-01-31. A converter that answers "Kaei 3 =
//! 1850" is rounding off six weeks at one end and a month at the other.
//!
//! Run with:
//!
//! ```sh
//! cargo run -p hyper-calendar --example kaei3 --features lunar
//! ```

use hyper_calendar::hc_calendar::{Calendar, CalendarRegistry, DateFields, Rd, Weekday};
use hyper_calendar::hc_calendars_lunar as lunar;
use hyper_calendar::hc_calendars_solar as solar;

fn registry() -> CalendarRegistry {
    let mut registry = CalendarRegistry::new();
    solar::register_all(&mut registry);
    lunar::register_all(&mut registry);
    registry
}

fn describe(label: &str, rd: Rd, registry: &CalendarRegistry) {
    let weekday = Weekday::from_rd(rd);
    println!("\n=== {label} — fixed day {} ({weekday}) ===", rd.get());
    let mut rows = registry.describe_day(rd);
    rows.sort_by_key(|(id, _)| id.as_str());
    for (id, fields) in rows {
        let month = match fields.month {
            Some(month) if month.leap => format!("leap {}", month.ordinal),
            Some(month) => month.ordinal.to_string(),
            None => "-".to_string(),
        };
        let day = fields
            .day
            .map_or_else(|| "-".to_string(), |day| day.to_string());
        let era = fields.era.unwrap_or("");
        let mut extra = String::new();
        for field in fields.extra.iter() {
            extra.push_str(&format!(" {}={}", field.name, field.value));
        }
        println!(
            "{:<28} {:>9} {:>8} {:>4} {}{}",
            id.as_str(),
            fields.year,
            month,
            day,
            era,
            extra
        );
    }
}

fn main() {
    let registry = registry();
    let tenpo = lunar::JapaneseTenpoCalendar;

    // The Tenpō calendar in this workspace numbers a lunisolar year by the
    // Gregorian year it starts in, so Kaei 3 is 1850.
    let Ok(first_day) = tenpo
        .from_fields(&DateFields::ymd(1850, 1, 1))
        .and_then(|date| tenpo.to_fixed(date))
    else {
        eprintln!("Kaei 3 is outside the Tenpō calendar's supported range");
        return;
    };
    let Ok(next_year) = tenpo
        .from_fields(&DateFields::ymd(1851, 1, 1))
        .and_then(|date| tenpo.to_fixed(date))
    else {
        eprintln!("Kaei 4 is outside the Tenpō calendar's supported range");
        return;
    };
    let last_day = Rd(next_year.get() - 1);

    println!(
        "Kaei 3 (嘉永3年) spans {} days.",
        last_day.get() - first_day.get() + 1
    );
    describe("Kaei 3, first day (嘉永3年正月1日)", first_day, &registry);
    describe("Kaei 3, last day (嘉永3年12月晦日)", last_day, &registry);
}
