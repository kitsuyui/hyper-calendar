//! The historical Indian eras over the lunisolar months are written in
//! Hindi and Sanskrit, not in English: each era's months from Chaitra, or
//! from Kārttika for the eras whose year opens there, as the lunisolar
//! calendars' own are, and in Hindi each era under the name Hindi
//! Wikipedia's "अब्द" article gives it (`wikipedia-hi-abda`).

#![cfg(all(
    feature = "alloc",
    feature = "civil",
    feature = "indic",
    feature = "i18n"
))]

use hyper_calendar::hc_calendar::Rd;

/// The cells of a calendar's line of `describe_day`: the identifier, the
/// era's name, the month's name and the written date.
fn cells(
    registry: &hyper_calendar::hc_calendar::CalendarRegistry,
    id: &str,
    locale: &str,
) -> [String; 4] {
    // Kārttika śukla 1 of Śaka 1947, 22 October 2025 (fixed day 739546), the
    // Gujarati new year.
    let day = Rd(739546);
    let text = hyper_calendar::lines::describe_day(registry, day, locale);
    let line = text
        .lines()
        .find(|line| line.split('\t').next() == Some(id))
        .unwrap_or_else(|| panic!("{id} has no line"));
    let cells: Vec<&str> = line.split('\t').collect();
    let n = cells.len();
    [
        cells[0].to_owned(),
        cells[3].to_owned(),
        cells[7].to_owned(),
        // The written date and the locale it was written in.
        format!("{}|{}", cells[n - 3], cells[n - 2]),
    ]
}

#[test]
fn the_seven_eras_are_written_in_hindi() {
    let registry = hyper_calendar::registry();
    for (id, era) in [
        ("vikram-samvat-kartikadi", "विक्रम संवत"),
        ("rajyabhisheka-saka", "राज्याभिषेक संवत्"),
        ("saptarshi", "सप्तर्षि संवत्"),
        ("gupta", "गुप्त संवत्"),
        ("valabhi", "वलभी संवत्"),
        ("kalachuri", "कलचुरि संवत्"),
        ("lakshmana-sena", "लक्ष्मणसेन संवत्"),
    ] {
        let [_, era_name, month, written] = cells(&registry, id, "hi");
        assert_eq!(era_name, era, "{id}");
        assert_eq!(month, "कार्तिक", "{id}");
        assert!(
            written.starts_with("1 कार्तिक ") && written.ends_with(&format!("{era}|hi")),
            "{id}: {written}"
        );
    }
}

#[test]
fn the_seven_eras_are_written_in_sanskrit_months() {
    let registry = hyper_calendar::registry();
    for id in [
        "vikram-samvat-kartikadi",
        "rajyabhisheka-saka",
        "saptarshi",
        "gupta",
        "valabhi",
        "kalachuri",
        "lakshmana-sena",
    ] {
        let [_, _, month, written] = cells(&registry, id, "sa");
        assert_eq!(month, "कार्तिक", "{id}");
        // The numerals are Devanagari and the locale is the one asked.
        assert!(
            written.starts_with("१ कार्तिक ") && written.ends_with("|sa"),
            "{id}: {written}"
        );
    }
}
