//! The exchange calendars against what the exchanges publish.

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;
use hc_holiday::engine::HolidayCalendar;
use hc_holiday::exchanges::{
    self, AUSTRALIAN_SECURITIES_EXCHANGE, FRANKFURT_STOCK_EXCHANGE, NEW_YORK_STOCK_EXCHANGE,
    TORONTO_STOCK_EXCHANGE,
};
use hc_holiday::rule::RuleSet;
use hc_holiday::rule::{Confidence, Kind};

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02}: {error:?}"),
    }
}

/// Closures as (month, day, name).
type Closures = Vec<(u8, u8, &'static str)>;
/// Early closes as (month, day).
type EarlyCloses = Vec<(u8, u8)>;

/// The weekday closures of a year, in order — the exchange's calendar
/// lists the observed days, and a holiday that stays on its Saturday is
/// no closure the weekend did not already make — and the early closes.
fn year(year: i64) -> (Closures, EarlyCloses) {
    year_of(&NEW_YORK_STOCK_EXCHANGE, year)
}

fn year_of(exchange: &RuleSet, year: i64) -> (Closures, EarlyCloses) {
    let calendar = HolidayCalendar::for_year(exchange, None, year);
    assert!(calendar.is_complete(), "{year}: {:?}", calendar.gaps());
    let mut closed = Vec::new();
    let mut early = Vec::new();
    for holiday in calendar.all() {
        let Ok((_, month, day)) = gregorian::from_fixed(holiday.date) else {
            panic!("{}", holiday.name);
        };
        assert_eq!(holiday.confidence, Confidence::Exact, "{}", holiday.name);
        if matches!(
            Weekday::from_rd(holiday.date),
            Weekday::Saturday | Weekday::Sunday
        ) {
            continue;
        }
        if holiday.is_day_off() {
            closed.push((month, day, holiday.name));
        } else {
            assert_eq!(holiday.kind, Kind::Observance, "{}", holiday.name);
            assert!(holiday.name.starts_with("Early close"), "{}", holiday.name);
            early.push((month, day));
        }
    }
    (closed, early)
}

#[test]
fn the_nyse_closes_on_the_days_its_calendar_lists_for_2026() {
    let (closed, early) = year(2026);
    let days: Vec<(u8, u8)> = closed.iter().map(|(m, d, _)| (*m, *d)).collect();
    assert_eq!(
        days,
        [
            (1, 1),
            (1, 19),
            (2, 16),
            (4, 3),
            (5, 25),
            (6, 19),
            (7, 3),
            (9, 7),
            (11, 26),
            (12, 25)
        ]
    );
    // Independence Day, a Saturday, closes the Friday.
    assert_eq!(closed[6].2, "Independence Day");
    assert_eq!(early, [(11, 27), (12, 24)]);
}

#[test]
fn the_nyse_closes_on_the_days_its_calendar_lists_for_2027() {
    let (closed, early) = year(2027);
    let days: Vec<(u8, u8)> = closed.iter().map(|(m, d, _)| (*m, *d)).collect();
    assert_eq!(
        days,
        [
            (1, 1),
            (1, 18),
            (2, 15),
            (3, 26),
            (5, 31),
            (6, 18),
            (7, 5),
            (9, 6),
            (11, 25),
            (12, 24)
        ]
    );
    // Juneteenth and Christmas, Saturdays, close the Fridays; Independence
    // Day, a Sunday, the Monday. 3 July is a Saturday and Christmas Eve is
    // the observed holiday, so the only early close is after Thanksgiving.
    assert_eq!(closed[5].2, "Juneteenth National Independence Day");
    assert_eq!(closed[9].2, "Christmas Day");
    assert_eq!(early, [(11, 26)]);
}

#[test]
fn the_nyse_does_not_observe_a_saturday_new_years_day() {
    // "Because the holiday falls on Saturday, January 1, 2028, no New
    // Year's Day holiday is observed."
    let (closed, early) = year(2028);
    let days: Vec<(u8, u8)> = closed.iter().map(|(m, d, _)| (*m, *d)).collect();
    assert_eq!(
        days,
        [
            (1, 17),
            (2, 21),
            (4, 14),
            (5, 29),
            (6, 19),
            (7, 4),
            (9, 4),
            (11, 23),
            (12, 25)
        ]
    );
    assert_eq!(early, [(7, 3), (11, 24)]);
    let calendar = HolidayCalendar::for_year(&NEW_YORK_STOCK_EXCHANGE, None, 2027);
    assert!(!calendar.is_holiday(ymd(2027, 12, 31)));
    // A Sunday New Year's Day still closes the Monday.
    let calendar = HolidayCalendar::for_year(&NEW_YORK_STOCK_EXCHANGE, None, 2023);
    assert!(calendar.is_holiday(ymd(2023, 1, 2)));
}

#[test]
fn an_early_close_is_a_trading_day() {
    let calendar = HolidayCalendar::for_year(&NEW_YORK_STOCK_EXCHANGE, None, 2026);
    assert!(!calendar.is_holiday(ymd(2026, 11, 27)));
    assert!(!calendar.is_holiday(ymd(2026, 12, 24)));
    assert!(calendar.is_holiday(ymd(2026, 11, 26)));
    let names: Vec<&str> = calendar
        .on(ymd(2026, 11, 27))
        .iter()
        .map(|holiday| holiday.name)
        .collect();
    assert_eq!(names, ["Early close, the day after Thanksgiving"]);
}

#[test]
fn the_unscheduled_closures_are_days_off_in_their_years_only() {
    for (year, month, day) in [
        (2001, 9, 11),
        (2001, 9, 12),
        (2001, 9, 13),
        (2001, 9, 14),
        (2012, 10, 29),
        (2012, 10, 30),
        (2018, 12, 5),
    ] {
        let calendar = HolidayCalendar::for_year(&NEW_YORK_STOCK_EXCHANGE, None, year);
        assert!(
            calendar.is_holiday(ymd(year, month, day)),
            "{year}-{month}-{day}"
        );
        let calendar = HolidayCalendar::for_year(&NEW_YORK_STOCK_EXCHANGE, None, year + 1);
        assert!(
            !calendar.is_holiday(ymd(year + 1, month, day)),
            "{}-{month}-{day}",
            year + 1
        );
    }
}

#[test]
fn the_days_the_exchange_added_later_start_when_they_started() {
    let calendar = HolidayCalendar::for_year(&NEW_YORK_STOCK_EXCHANGE, None, 2021);
    assert!(!calendar.is_holiday(ymd(2021, 6, 18)));
    assert!(!calendar.is_holiday(ymd(2021, 6, 21)));
    let calendar = HolidayCalendar::for_year(&NEW_YORK_STOCK_EXCHANGE, None, 1997);
    assert!(!calendar.is_holiday(ymd(1997, 1, 20)));
    let calendar = HolidayCalendar::for_year(&NEW_YORK_STOCK_EXCHANGE, None, 1998);
    assert!(calendar.is_holiday(ymd(1998, 1, 19)));
    // The exchange trades on Columbus Day and Veterans Day.
    let calendar = HolidayCalendar::for_year(&NEW_YORK_STOCK_EXCHANGE, None, 2026);
    assert!(!calendar.is_holiday(ymd(2026, 10, 12)));
    assert!(!calendar.is_holiday(ymd(2026, 11, 11)));
}

fn days(closed: &Closures) -> Vec<(u8, u8)> {
    closed.iter().map(|(m, d, _)| (*m, *d)).collect()
}

#[test]
fn the_asx_closes_on_the_days_its_calendar_lists() {
    // 2026: Anzac Day is a Saturday and gives no Monday; Boxing Day, a
    // Saturday after a Friday Christmas, gives Monday the 28th.
    let (closed, early) = year_of(&AUSTRALIAN_SECURITIES_EXCHANGE, 2026);
    assert_eq!(
        days(&closed),
        [(1, 1), (1, 26), (4, 3), (4, 6), (6, 8), (12, 25), (12, 28)]
    );
    assert_eq!(closed[4].2, "King's Birthday");
    assert_eq!(early, [(12, 24), (12, 31)]);
    // 2027: Christmas on a Saturday gives the Monday and Boxing Day the
    // Tuesday; Anzac Day on a Sunday gives nothing, and the market trades
    // on Monday 26 April.
    let (closed, early) = year_of(&AUSTRALIAN_SECURITIES_EXCHANGE, 2027);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (1, 26),
            (3, 26),
            (3, 29),
            (6, 14),
            (12, 27),
            (12, 28)
        ]
    );
    assert_eq!(early, [(12, 24), (12, 31)]);
    let calendar = HolidayCalendar::for_year(&AUSTRALIAN_SECURITIES_EXCHANGE, None, 2027);
    assert!(!calendar.is_holiday(ymd(2027, 4, 26)));
    let calendar = HolidayCalendar::for_year(&AUSTRALIAN_SECURITIES_EXCHANGE, None, 2022);
    assert_eq!(calendar.name_on(ymd(2022, 6, 13)), Some("Queen's Birthday"));
}

#[test]
fn xetra_closes_on_its_eight_days_and_moves_none() {
    let (closed, early) = year_of(&FRANKFURT_STOCK_EXCHANGE, 2026);
    assert_eq!(
        days(&closed),
        [(1, 1), (4, 3), (4, 6), (5, 1), (12, 24), (12, 25), (12, 31)]
    );
    assert!(early.is_empty());
    // 2027: 1 May and Christmas fall on the weekend and no day is moved;
    // Ascension Day and Corpus Christi are traded on.
    let (closed, early) = year_of(&FRANKFURT_STOCK_EXCHANGE, 2027);
    assert_eq!(
        days(&closed),
        [(1, 1), (3, 26), (3, 29), (12, 24), (12, 31)]
    );
    assert!(early.is_empty());
    let calendar = HolidayCalendar::for_year(&FRANKFURT_STOCK_EXCHANGE, None, 2027);
    assert!(!calendar.is_holiday(ymd(2027, 5, 3)));
    assert!(!calendar.is_holiday(ymd(2027, 12, 27)));
    let calendar = HolidayCalendar::for_year(&FRANKFURT_STOCK_EXCHANGE, None, 2026);
    assert!(!calendar.is_holiday(ymd(2026, 5, 14)));
    assert!(!calendar.is_holiday(ymd(2026, 6, 4)));
}

#[test]
fn the_tsx_closes_on_the_days_its_calendar_lists() {
    let (closed, early) = year_of(&TORONTO_STOCK_EXCHANGE, 2025);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (2, 17),
            (4, 18),
            (5, 19),
            (7, 1),
            (8, 4),
            (9, 1),
            (10, 13),
            (12, 25),
            (12, 26)
        ]
    );
    assert_eq!(early, [(12, 24)]);
    // 2026: Boxing Day, a Saturday, closes Monday the 28th in lieu.
    let (closed, early) = year_of(&TORONTO_STOCK_EXCHANGE, 2026);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (2, 16),
            (4, 3),
            (5, 18),
            (7, 1),
            (8, 3),
            (9, 7),
            (10, 12),
            (12, 25),
            (12, 28)
        ]
    );
    assert_eq!(closed[9].2, "Boxing Day");
    assert_eq!(early, [(12, 24)]);
    // The United States days the calendar lists for settlement are trading days.
    let calendar = HolidayCalendar::for_year(&TORONTO_STOCK_EXCHANGE, None, 2026);
    assert!(!calendar.is_holiday(ymd(2026, 1, 19)));
    assert!(!calendar.is_holiday(ymd(2026, 11, 26)));
}

#[test]
fn the_catalogue_is_keyed_by_market_identifier_code() {
    assert_eq!(
        exchanges::by_code("xnys").map(|e| e.english_name),
        Some("New York Stock Exchange")
    );
    assert!(exchanges::ALL.iter().all(|e| e.code.len() == 4));
    assert_eq!(exchanges::ALL.len(), 4);
}
