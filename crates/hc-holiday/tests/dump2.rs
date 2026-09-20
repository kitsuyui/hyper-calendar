use hc_calendars_solar::gregorian;
use hc_holiday::engine::HolidayCalendar;
use hc_holiday::traditions;

#[test]
fn dump_traditions() {
    for set in traditions::ALL {
        for year in [2024i64, 2025] {
            println!("=== {} {year} ===", set.code);
            for h in HolidayCalendar::for_year(set, None, year).all() {
                let (y, m, d) = gregorian::from_fixed(h.date).expect("ok");
                println!("{y:04}-{m:02}-{d:02} {} {:?}", h.name, h.confidence);
            }
        }
    }
}
