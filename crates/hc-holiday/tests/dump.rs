//! Temporary scaffolding: prints evaluated calendars for eyeball checks.
use hc_calendar::Weekday;
use hc_calendars_solar::gregorian;
use hc_holiday::countries;
use hc_holiday::engine::HolidayCalendar;

fn show(code: &str, region: Option<&str>, year: i64) {
    let country = countries::by_code(code).expect("known country");
    let calendar = HolidayCalendar::for_year(country, region, year);
    println!("--- {code} {region:?} {year} ---");
    for holiday in calendar.all() {
        let (y, m, d) = gregorian::from_fixed(holiday.date).expect("in range");
        println!(
            "{y:04}-{m:02}-{d:02} {:<3} {:<45} {:?} {}{}",
            Weekday::from_rd(holiday.date).english_name().get(..3).unwrap_or(""),
            holiday.name,
            holiday.kind,
            if holiday.is_substitute() { "SUB " } else { "" },
            if holiday.bridged { "BRIDGE" } else { "" },
        );
    }
}

#[test]
fn dump() {
    for year in [1948, 1949, 1966, 1967, 1973, 1985, 1986, 1988, 1989, 1990] {
        show("JP", None, year);
    }
    for year in [1996, 1999, 2000, 2002, 2003, 2006, 2007, 2015, 2016, 2019, 2020, 2021, 2024, 2026] {
        show("JP", None, year);
    }
    for (code, region) in [
        ("US", None),
        ("GB", Some("GB-EAW")),
        ("GB", Some("GB-SCT")),
        ("DE", Some("DE-BY")),
        ("FR", None),
        ("CA", Some("CA-ON")),
        ("AU", Some("AU-VIC")),
        ("NZ", None),
        ("CN", None),
        ("TW", None),
        ("KR", None),
        ("IN", None),
        ("TH", None),
        ("VN", None),
        ("ID", None),
        ("SG", None),
        ("MY", None),
        ("PH", None),
        ("IL", None),
        ("SA", None),
        ("AE", None),
        ("TR", None),
        ("EG", None),
        ("NG", None),
        ("ZA", None),
        ("MX", None),
        ("BR", None),
        ("IT", None),
        ("ES", None),
        ("PT", None),
        ("NL", None),
        ("BE", None),
        ("CH", None),
        ("AT", None),
        ("SE", None),
        ("NO", None),
        ("DK", None),
        ("FI", None),
        ("PL", None),
        ("CZ", None),
        ("GR", None),
        ("IE", None),
        ("NP", None),
    ] {
        show(code, region, 2024);
        show(code, region, 2025);
    }
}
