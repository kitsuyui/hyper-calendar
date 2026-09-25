//! The exchange calendars against what the exchanges publish.

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;
use hc_holiday::engine::HolidayCalendar;
use hc_holiday::exchanges::{
    self, AUSTRALIAN_SECURITIES_EXCHANGE, B3, EURONEXT_AMSTERDAM, EURONEXT_BRUSSELS,
    EURONEXT_DUBLIN, EURONEXT_LISBON, EURONEXT_MILAN, EURONEXT_OSLO, EURONEXT_PARIS,
    FRANKFURT_STOCK_EXCHANGE, HONG_KONG_EXCHANGES, KOREA_EXCHANGE, LONDON_STOCK_EXCHANGE, NASDAQ,
    NASDAQ_COPENHAGEN, NASDAQ_HELSINKI, NASDAQ_ICELAND, NASDAQ_STOCKHOLM, NEW_YORK_STOCK_EXCHANGE,
    SHANGHAI_STOCK_EXCHANGE, SIX_SWISS_EXCHANGE, TAIWAN_STOCK_EXCHANGE, TOKYO_STOCK_EXCHANGE,
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
            // Two names on one day — Dublin's 1 May 2023 — are one closure.
            if closed
                .last()
                .is_some_and(|(m, d, _)| (*m, *d) == (month, day))
            {
                continue;
            }
            closed.push((month, day, holiday.name));
        } else if holiday.name.starts_with("Early close")
            || holiday.name.starts_with("Half trading day")
            || holiday.name.starts_with("Late open")
        {
            assert_eq!(holiday.kind, Kind::Observance, "{}", holiday.name);
            early.push((month, day));
        } else {
            // Every entry is a closure or a partial day: an included
            // country's observances do not come along.
            panic!("{}: neither a closure nor a partial day", holiday.name);
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
    // The early closes are the "Last Business day before Christmas Day" and
    // the "Last Business day of the Year", as the calendar names them, not
    // fixed dates: in 2022, when 24 and 31 December were Saturdays, they
    // fall on the Fridays before.
    let (_, early) = year_of(&AUSTRALIAN_SECURITIES_EXCHANGE, 2022);
    assert_eq!(early, [(12, 23), (12, 30)]);
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
fn the_four_euronext_markets_with_one_calendar_close_on_its_days() {
    for market in [
        &EURONEXT_AMSTERDAM,
        &EURONEXT_BRUSSELS,
        &EURONEXT_LISBON,
        &EURONEXT_PARIS,
    ] {
        // 2022: New Year's Day, 1 May and Christmas on the weekend, none
        // moved, and no half day for a Saturday Christmas Eve.
        let (closed, early) = year_of(market, 2022);
        assert_eq!(
            days(&closed),
            [(4, 15), (4, 18), (12, 26)],
            "{}",
            market.code
        );
        assert!(early.is_empty(), "{}", market.code);
        let (closed, early) = year_of(market, 2023);
        assert_eq!(
            days(&closed),
            [(4, 7), (4, 10), (5, 1), (12, 25), (12, 26)],
            "{}",
            market.code
        );
        assert!(early.is_empty(), "{}", market.code);
        let (closed, early) = year_of(market, 2024);
        assert_eq!(
            days(&closed),
            [(1, 1), (3, 29), (4, 1), (5, 1), (12, 25), (12, 26)],
            "{}",
            market.code
        );
        assert_eq!(early, [(12, 24), (12, 31)], "{}", market.code);
        let (closed, early) = year_of(market, 2025);
        assert_eq!(
            days(&closed),
            [(1, 1), (4, 18), (4, 21), (5, 1), (12, 25), (12, 26)],
            "{}",
            market.code
        );
        assert_eq!(early, [(12, 24), (12, 31)], "{}", market.code);
        let (closed, early) = year_of(market, 2026);
        assert_eq!(
            days(&closed),
            [(1, 1), (4, 3), (4, 6), (5, 1), (12, 25)],
            "{}",
            market.code
        );
        assert_eq!(early, [(12, 24), (12, 31)], "{}", market.code);
        let calendar = HolidayCalendar::for_year(market, None, 2026);
        assert!(!calendar.is_holiday(ymd(2026, 12, 28)), "{}", market.code);
    }
}

#[test]
fn dublin_moves_its_weekend_holidays_and_halves_the_last_day_before() {
    let (closed, early) = year_of(&EURONEXT_DUBLIN, 2021);
    assert_eq!(
        days(&closed),
        [(1, 1), (4, 2), (4, 5), (5, 3), (12, 27), (12, 28)]
    );
    assert_eq!(early, [(12, 24), (12, 31)]);
    let (closed, early) = year_of(&EURONEXT_DUBLIN, 2022);
    assert_eq!(
        days(&closed),
        [(1, 3), (4, 15), (4, 18), (5, 2), (12, 26), (12, 27)]
    );
    assert_eq!(early, [(12, 23), (12, 30)]);
    let (closed, early) = year_of(&EURONEXT_DUBLIN, 2023);
    assert_eq!(
        days(&closed),
        [(1, 2), (4, 7), (4, 10), (5, 1), (12, 25), (12, 26)]
    );
    assert_eq!(early, [(12, 22), (12, 29)]);
    let (closed, early) = year_of(&EURONEXT_DUBLIN, 2026);
    assert_eq!(
        days(&closed),
        [(1, 1), (4, 3), (4, 6), (5, 1), (5, 4), (12, 25), (12, 28)]
    );
    assert_eq!(early, [(12, 24), (12, 31)]);
}

#[test]
fn milan_closes_on_ferragosto_and_both_eves() {
    let (closed, early) = year_of(&EURONEXT_MILAN, 2024);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (3, 29),
            (4, 1),
            (5, 1),
            (8, 15),
            (12, 24),
            (12, 25),
            (12, 26),
            (12, 31)
        ]
    );
    assert!(early.is_empty());
    let (closed, early) = year_of(&EURONEXT_MILAN, 2025);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (4, 18),
            (4, 21),
            (5, 1),
            (8, 15),
            (12, 24),
            (12, 25),
            (12, 26),
            (12, 31)
        ]
    );
    assert!(early.is_empty());
    let (closed, early) = year_of(&EURONEXT_MILAN, 2026);
    assert_eq!(
        days(&closed),
        [(1, 1), (4, 3), (4, 6), (5, 1), (12, 24), (12, 25), (12, 31)]
    );
    assert!(early.is_empty());
}

#[test]
fn oslo_keeps_the_norwegian_days_and_halves_the_wednesday_before_easter() {
    let (closed, early) = year_of(&EURONEXT_OSLO, 2021);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (4, 1),
            (4, 2),
            (4, 5),
            (5, 13),
            (5, 17),
            (5, 24),
            (12, 24),
            (12, 31)
        ]
    );
    assert_eq!(early, [(3, 31)]);
    let (closed, early) = year_of(&EURONEXT_OSLO, 2024);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (3, 28),
            (3, 29),
            (4, 1),
            (5, 1),
            (5, 9),
            (5, 17),
            (5, 20),
            (12, 24),
            (12, 25),
            (12, 26),
            (12, 31)
        ]
    );
    assert_eq!(early, [(3, 27)]);
    let (closed, early) = year_of(&EURONEXT_OSLO, 2026);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (4, 2),
            (4, 3),
            (4, 6),
            (5, 1),
            (5, 14),
            (5, 25),
            (12, 24),
            (12, 25),
            (12, 31)
        ]
    );
    assert_eq!(early, [(4, 1)]);
}

#[test]
fn b3_closes_on_the_days_its_calendar_lists() {
    // 2022: New Year's Day a Saturday and Christmas a Sunday, neither
    // moved; the last weekday of the year is Friday the 30th.
    let (closed, early) = year_of(&B3, 2022);
    assert_eq!(
        days(&closed),
        [
            (2, 28),
            (3, 1),
            (4, 15),
            (4, 21),
            (6, 16),
            (9, 7),
            (10, 12),
            (11, 2),
            (11, 15),
            (12, 30)
        ]
    );
    assert_eq!(early, [(3, 2)]);
    // 2023: no Black Consciousness Day yet; the last weekday is the 29th.
    let (closed, early) = year_of(&B3, 2023);
    assert_eq!(
        days(&closed),
        [
            (2, 20),
            (2, 21),
            (4, 7),
            (4, 21),
            (5, 1),
            (6, 8),
            (9, 7),
            (10, 12),
            (11, 2),
            (11, 15),
            (12, 25),
            (12, 29)
        ]
    );
    assert_eq!(early, [(2, 22)]);
    let (closed, early) = year_of(&B3, 2024);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (2, 12),
            (2, 13),
            (3, 29),
            (5, 1),
            (5, 30),
            (11, 15),
            (11, 20),
            (12, 24),
            (12, 25),
            (12, 31)
        ]
    );
    assert_eq!(early, [(2, 14)]);
    let (closed, early) = year_of(&B3, 2026);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (2, 16),
            (2, 17),
            (4, 3),
            (4, 21),
            (5, 1),
            (6, 4),
            (9, 7),
            (10, 12),
            (11, 2),
            (11, 20),
            (12, 24),
            (12, 25),
            (12, 31)
        ]
    );
    assert_eq!(early, [(2, 18)]);
    // Ash Wednesday is a trading day; São Paulo's own days are too.
    let calendar = HolidayCalendar::for_year(&B3, None, 2026);
    assert!(!calendar.is_holiday(ymd(2026, 2, 18)));
    assert!(!calendar.is_holiday(ymd(2026, 7, 9)));
    let calendar = HolidayCalendar::for_year(&B3, None, 2024);
    assert!(!calendar.is_holiday(ymd(2024, 1, 25)));
}

#[test]
fn tokyo_closes_on_japans_holidays_and_its_three_market_holidays() {
    // JPX's 2026 and 2027 calendars: Japan's days, with the substitute of
    // Wednesday 6 May 2026 and Monday 22 March 2027 and the bridge of
    // Tuesday 22 September 2026 under Japan's own law, and 2 and 3 January
    // and 31 December as the exchange's own.
    let (closed, early) = year_of(&TOKYO_STOCK_EXCHANGE, 2026);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (1, 2),
            (1, 12),
            (2, 11),
            (2, 23),
            (3, 20),
            (4, 29),
            (5, 4),
            (5, 5),
            (5, 6),
            (7, 20),
            (8, 11),
            (9, 21),
            (9, 22),
            (9, 23),
            (10, 12),
            (11, 3),
            (11, 23),
            (12, 31)
        ]
    );
    assert!(early.is_empty());
    let (closed, early) = year_of(&TOKYO_STOCK_EXCHANGE, 2027);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (1, 11),
            (2, 11),
            (2, 23),
            (3, 22),
            (4, 29),
            (5, 3),
            (5, 4),
            (5, 5),
            (7, 19),
            (8, 11),
            (9, 20),
            (9, 23),
            (10, 11),
            (11, 3),
            (11, 23),
            (12, 31)
        ]
    );
    assert!(early.is_empty());
    // The included table's days are not repeated: one entry a day.
    let calendar = HolidayCalendar::for_year(&TOKYO_STOCK_EXCHANGE, None, 2026);
    assert_eq!(calendar.on(ymd(2026, 5, 6)).len(), 1);
    assert_eq!(calendar.on(ymd(2026, 1, 2)).len(), 1);
    assert!(calendar.is_complete());
}

/// The weekday closures the Korea Exchange lists, year by year.
///
/// Its list for 2030 stops at October and is left out.
#[rustfmt::skip]
const KRX_CLOSURES: &[(i64, &[(u8, u8)])] = &[
    (2009, &[(1, 1), (1, 26), (1, 27), (5, 1), (5, 5), (10, 2), (12, 25), (12, 31)]),
    (2010, &[(1, 1), (2, 15), (3, 1), (5, 5), (5, 21), (6, 2), (9, 21), (9, 22), (9, 23), (12, 31)]),
    (2011, &[(2, 2), (2, 3), (2, 4), (3, 1), (5, 5), (5, 10), (6, 6), (8, 15), (9, 12), (9, 13), (10, 3), (12, 30)]),
    (2012, &[(1, 23), (1, 24), (3, 1), (4, 11), (5, 1), (5, 28), (6, 6), (8, 15), (10, 1), (10, 3), (12, 19), (12, 25), (12, 31)]),
    (2013, &[(1, 1), (2, 11), (3, 1), (5, 1), (5, 17), (6, 6), (8, 15), (9, 18), (9, 19), (9, 20), (10, 3), (10, 9), (12, 25), (12, 31)]),
    (2014, &[(1, 1), (1, 30), (1, 31), (5, 1), (5, 5), (5, 6), (6, 4), (6, 6), (8, 15), (9, 8), (9, 9), (9, 10), (10, 3), (10, 9), (12, 25), (12, 31)]),
    (2015, &[(1, 1), (2, 18), (2, 19), (2, 20), (5, 1), (5, 5), (5, 25), (8, 14), (9, 28), (9, 29), (10, 9), (12, 25), (12, 31)]),
    (2016, &[(1, 1), (2, 8), (2, 9), (2, 10), (3, 1), (4, 13), (5, 5), (5, 6), (6, 6), (8, 15), (9, 14), (9, 15), (9, 16), (10, 3), (12, 30)]),
    (2017, &[(1, 27), (1, 30), (3, 1), (5, 1), (5, 3), (5, 5), (5, 9), (6, 6), (8, 15), (10, 2), (10, 3), (10, 4), (10, 5), (10, 6), (10, 9), (12, 25), (12, 29)]),
    (2018, &[(1, 1), (2, 15), (2, 16), (3, 1), (5, 1), (5, 7), (5, 22), (6, 6), (6, 13), (8, 15), (9, 24), (9, 25), (9, 26), (10, 3), (10, 9), (12, 25), (12, 31)]),
    (2019, &[(1, 1), (2, 4), (2, 5), (2, 6), (3, 1), (5, 1), (5, 6), (6, 6), (8, 15), (9, 12), (9, 13), (10, 3), (10, 9), (12, 25), (12, 31)]),
    (2020, &[(1, 1), (1, 24), (1, 27), (4, 15), (4, 30), (5, 1), (5, 5), (8, 17), (9, 30), (10, 1), (10, 2), (10, 9), (12, 25), (12, 31)]),
    (2021, &[(1, 1), (2, 11), (2, 12), (3, 1), (5, 5), (5, 19), (8, 16), (9, 20), (9, 21), (9, 22), (10, 4), (10, 11), (12, 31)]),
    (2022, &[(1, 31), (2, 1), (2, 2), (3, 1), (3, 9), (5, 5), (6, 1), (6, 6), (8, 15), (9, 9), (9, 12), (10, 3), (10, 10), (12, 30)]),
    (2023, &[(1, 23), (1, 24), (3, 1), (5, 1), (5, 5), (5, 29), (6, 6), (8, 15), (9, 28), (9, 29), (10, 2), (10, 3), (10, 9), (12, 25), (12, 29)]),
    (2024, &[(1, 1), (2, 9), (2, 12), (3, 1), (4, 10), (5, 1), (5, 6), (5, 15), (6, 6), (8, 15), (9, 16), (9, 17), (9, 18), (10, 1), (10, 3), (10, 9), (12, 25), (12, 31)]),
    (2025, &[(1, 1), (1, 27), (1, 28), (1, 29), (1, 30), (3, 3), (5, 1), (5, 5), (5, 6), (6, 3), (6, 6), (8, 15), (10, 3), (10, 6), (10, 7), (10, 8), (10, 9), (12, 25), (12, 31)]),
    (2026, &[(1, 1), (2, 16), (2, 17), (2, 18), (3, 2), (5, 1), (5, 5), (5, 25), (6, 3), (7, 17), (8, 17), (9, 24), (9, 25), (10, 5), (10, 9), (12, 25), (12, 31)]),
    (2027, &[(1, 1), (2, 8), (2, 9), (3, 1), (5, 3), (5, 5), (5, 13), (7, 19), (8, 16), (9, 14), (9, 15), (9, 16), (10, 4), (10, 11), (12, 27), (12, 31)]),
    (2028, &[(1, 26), (1, 27), (1, 28), (3, 1), (5, 1), (5, 2), (5, 5), (6, 6), (7, 17), (8, 15), (10, 2), (10, 3), (10, 4), (10, 5), (10, 9), (12, 25), (12, 29)]),
    (2029, &[(1, 1), (2, 12), (2, 13), (2, 14), (3, 1), (5, 1), (5, 7), (5, 21), (6, 6), (7, 17), (8, 15), (9, 21), (9, 24), (10, 3), (10, 9), (12, 25), (12, 31)]),
];

#[test]
fn the_krx_closes_on_the_days_it_lists_from_2009_to_2029() {
    for &(y, listed) in KRX_CLOSURES {
        let (closed, early) = year_of(&KOREA_EXCHANGE, y);
        assert_eq!(days(&closed), listed, "{y}");
        assert!(early.is_empty(), "{y}");
    }
}

#[test]
fn the_krx_keeps_labour_day_and_the_last_weekday_of_the_year() {
    // Labour Day on a Monday before it was a public holiday; the End of
    // Year Holiday on the Friday when the 30th and 31st are a weekend.
    let calendar = HolidayCalendar::for_year(&KOREA_EXCHANGE, None, 2023);
    assert_eq!(calendar.on(ymd(2023, 5, 1))[0].name, "Labour Day");
    assert_eq!(
        calendar.on(ymd(2023, 12, 29))[0].name,
        "End of Year Holiday"
    );
    // From 2026 the day comes from the country's table, once, and is
    // substituted like it: Saturday 1 May 2027 gives Monday the 3rd.
    let calendar = HolidayCalendar::for_year(&KOREA_EXCHANGE, None, 2027);
    assert_eq!(calendar.on(ymd(2027, 5, 1)).len(), 1);
    assert!(calendar.is_holiday(ymd(2027, 5, 3)));
    // The national table does not close for either before 2026.
    let calendar = HolidayCalendar::for_year(&hc_holiday::countries::SOUTH_KOREA, None, 2023);
    assert!(!calendar.is_holiday(ymd(2023, 5, 1)));
    assert!(!calendar.is_holiday(ymd(2023, 12, 29)));
}

/// The weekday closures in the Shanghai Stock Exchange's annual notices,
/// with the three later changes: 3 and 4 September 2015 from the
/// exchanges' announcement of July 2015, as the press reported it; 2 and 3
/// May 2019 from 上证公告〔2019〕20号; 31 January 2020 from 上证公告〔2020〕6号.
/// The notice for 2019 closes 31 December 2018, which is here under 2018.
#[rustfmt::skip]
const SSE_CLOSURES: &[(i64, &[(u8, u8)])] = &[
    (2014, &[(1, 1), (1, 31), (2, 3), (2, 4), (2, 5), (2, 6), (4, 7), (5, 1), (5, 2), (6, 2), (9, 8), (10, 1), (10, 2), (10, 3), (10, 6), (10, 7)]),
    (2015, &[(1, 1), (1, 2), (2, 18), (2, 19), (2, 20), (2, 23), (2, 24), (4, 6), (5, 1), (6, 22), (9, 3), (9, 4), (10, 1), (10, 2), (10, 5), (10, 6), (10, 7)]),
    (2016, &[(1, 1), (2, 8), (2, 9), (2, 10), (2, 11), (2, 12), (4, 4), (5, 2), (6, 9), (6, 10), (9, 15), (9, 16), (10, 3), (10, 4), (10, 5), (10, 6), (10, 7)]),
    (2017, &[(1, 2), (1, 27), (1, 30), (1, 31), (2, 1), (2, 2), (4, 3), (4, 4), (5, 1), (5, 29), (5, 30), (10, 2), (10, 3), (10, 4), (10, 5), (10, 6)]),
    (2018, &[(1, 1), (2, 15), (2, 16), (2, 19), (2, 20), (2, 21), (4, 5), (4, 6), (4, 30), (5, 1), (6, 18), (9, 24), (10, 1), (10, 2), (10, 3), (10, 4), (10, 5), (12, 31)]),
    (2019, &[(1, 1), (2, 4), (2, 5), (2, 6), (2, 7), (2, 8), (4, 5), (5, 1), (5, 2), (5, 3), (6, 7), (9, 13), (10, 1), (10, 2), (10, 3), (10, 4), (10, 7)]),
    (2020, &[(1, 1), (1, 24), (1, 27), (1, 28), (1, 29), (1, 30), (1, 31), (4, 6), (5, 1), (5, 4), (5, 5), (6, 25), (6, 26), (10, 1), (10, 2), (10, 5), (10, 6), (10, 7), (10, 8)]),
    (2021, &[(1, 1), (2, 11), (2, 12), (2, 15), (2, 16), (2, 17), (4, 5), (5, 3), (5, 4), (5, 5), (6, 14), (9, 20), (9, 21), (10, 1), (10, 4), (10, 5), (10, 6), (10, 7)]),
    (2022, &[(1, 3), (1, 31), (2, 1), (2, 2), (2, 3), (2, 4), (4, 4), (4, 5), (5, 2), (5, 3), (5, 4), (6, 3), (9, 12), (10, 3), (10, 4), (10, 5), (10, 6), (10, 7)]),
    (2023, &[(1, 2), (1, 23), (1, 24), (1, 25), (1, 26), (1, 27), (4, 5), (5, 1), (5, 2), (5, 3), (6, 22), (6, 23), (9, 29), (10, 2), (10, 3), (10, 4), (10, 5), (10, 6)]),
    (2024, &[(1, 1), (2, 9), (2, 12), (2, 13), (2, 14), (2, 15), (2, 16), (4, 4), (4, 5), (5, 1), (5, 2), (5, 3), (6, 10), (9, 16), (9, 17), (10, 1), (10, 2), (10, 3), (10, 4), (10, 7)]),
    (2025, &[(1, 1), (1, 28), (1, 29), (1, 30), (1, 31), (2, 3), (2, 4), (4, 4), (5, 1), (5, 2), (5, 5), (6, 2), (10, 1), (10, 2), (10, 3), (10, 6), (10, 7), (10, 8)]),
    (2026, &[(1, 1), (1, 2), (2, 16), (2, 17), (2, 18), (2, 19), (2, 20), (2, 23), (4, 6), (5, 1), (5, 4), (5, 5), (6, 19), (9, 25), (10, 1), (10, 2), (10, 5), (10, 6), (10, 7)]),
];

#[test]
fn shanghai_closes_on_the_days_its_notices_list_from_2014_to_2026() {
    for &(y, listed) in SSE_CLOSURES {
        let (closed, early) = year_of(&SHANGHAI_STOCK_EXCHANGE, y);
        assert_eq!(days(&closed), listed, "{y}");
        assert!(early.is_empty(), "{y}");
    }
    // The Sunday China works is a weekend day for the exchange.
    let calendar = HolidayCalendar::for_year(&SHANGHAI_STOCK_EXCHANGE, None, 2024);
    assert!(!calendar.is_business_day(ymd(2024, 2, 4)));
    assert!(!calendar.is_business_day(ymd(2024, 2, 9)));
    assert!(calendar.is_business_day(ymd(2024, 2, 19)));
    // And a year without an arrangement is not guessed.
    let calendar = HolidayCalendar::for_year(&SHANGHAI_STOCK_EXCHANGE, None, 2027);
    assert!(!calendar.is_complete());
}

/// The weekday closures in the Taiwan Stock Exchange's schedules.
#[rustfmt::skip]
const TWSE_CLOSURES: &[(i64, &[(u8, u8)])] = &[
    (2023, &[(1, 2), (1, 18), (1, 19), (1, 20), (1, 23), (1, 24), (1, 25), (1, 26), (1, 27), (2, 27), (2, 28), (4, 3), (4, 4), (4, 5), (5, 1), (6, 22), (6, 23), (9, 29), (10, 9), (10, 10)]),
    (2024, &[(1, 1), (2, 6), (2, 7), (2, 8), (2, 9), (2, 12), (2, 13), (2, 14), (2, 28), (4, 4), (4, 5), (5, 1), (6, 10), (9, 17), (10, 10)]),
    (2025, &[(1, 1), (1, 23), (1, 24), (1, 27), (1, 28), (1, 29), (1, 30), (1, 31), (2, 28), (4, 3), (4, 4), (5, 1), (5, 30), (9, 29), (10, 6), (10, 10), (10, 24), (12, 25)]),
    (2026, &[(1, 1), (2, 12), (2, 13), (2, 16), (2, 17), (2, 18), (2, 19), (2, 20), (2, 27), (4, 3), (4, 6), (5, 1), (6, 19), (9, 25), (9, 28), (10, 9), (10, 26), (12, 25)]),
];

#[test]
fn taipei_closes_on_the_days_its_schedules_list_from_2023_to_2026() {
    for &(y, listed) in TWSE_CLOSURES {
        let (closed, early) = year_of(&TAIWAN_STOCK_EXCHANGE, y);
        assert_eq!(days(&closed), listed, "{y}");
        assert!(early.is_empty(), "{y}");
    }
    // The Saturdays the government worked, the exchange did not.
    let calendar = HolidayCalendar::for_year(&TAIWAN_STOCK_EXCHANGE, None, 2023);
    assert!(!calendar.is_business_day(ymd(2023, 1, 7)));
    assert!(!HolidayCalendar::for_year(&TAIWAN_STOCK_EXCHANGE, None, 2027).is_complete());
}

#[test]
fn hong_kong_closes_on_the_general_holidays_and_halves_three_eves() {
    // HKEX's calendar feed for 2026: every "Hong Kong Market is closed"
    // entry, and the three half days.
    let (closed, early) = year_of(&HONG_KONG_EXCHANGES, 2026);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (2, 17),
            (2, 18),
            (2, 19),
            (4, 3),
            (4, 6),
            (4, 7),
            (5, 1),
            (5, 25),
            (6, 19),
            (7, 1),
            (10, 1),
            (10, 19),
            (12, 25)
        ]
    );
    assert_eq!(early, [(2, 16), (12, 24), (12, 31)]);
    // The feed's 2027 entries run to 3 October: a Saturday Lunar New Year's
    // Day gives the third and fourth days, and the eve is Friday 5 February.
    let (closed, early) = year_of(&HONG_KONG_EXCHANGES, 2027);
    let to_october: Vec<(u8, u8)> = days(&closed)
        .into_iter()
        .filter(|(m, d)| (*m, *d) <= (10, 3))
        .collect();
    assert_eq!(
        to_october,
        [
            (1, 1),
            (2, 8),
            (2, 9),
            (3, 26),
            (3, 29),
            (4, 5),
            (5, 13),
            (6, 9),
            (7, 1),
            (9, 16),
            (10, 1)
        ]
    );
    assert!(early.contains(&(2, 5)));
}

#[test]
fn nasdaq_keeps_the_nyse_calendar_day_for_day() {
    // Nasdaq's own 2026 calendar: the same ten closed days and two early
    // closes; and the same three unscheduled closures.
    let (closed, early) = year_of(&NASDAQ, 2026);
    assert_eq!(
        days(&closed),
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
    assert_eq!(early, [(11, 27), (12, 24)]);
    for year in [2001, 2012, 2018, 2027, 2028] {
        let nyse = HolidayCalendar::for_year(&NEW_YORK_STOCK_EXCHANGE, None, year);
        let nasdaq = HolidayCalendar::for_year(&NASDAQ, None, year);
        let dates = |calendar: &HolidayCalendar| -> Vec<(Rd, &str)> {
            calendar
                .all()
                .iter()
                .map(|holiday| (holiday.date, holiday.name))
                .collect()
        };
        assert_eq!(dates(&nyse), dates(&nasdaq), "{year}");
    }
}

#[test]
fn the_nordic_exchanges_close_on_the_days_nasdaqs_calendar_lists() {
    // Copenhagen: the Friday after Ascension, Constitution Day, no Great
    // Prayer Day.
    let (closed, early) = year_of(&NASDAQ_COPENHAGEN, 2025);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (4, 17),
            (4, 18),
            (4, 21),
            (5, 29),
            (5, 30),
            (6, 5),
            (6, 9),
            (12, 24),
            (12, 25),
            (12, 26),
            (12, 31)
        ]
    );
    assert!(early.is_empty());
    let (closed, _) = year_of(&NASDAQ_COPENHAGEN, 2027);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (3, 25),
            (3, 26),
            (3, 29),
            (5, 6),
            (5, 7),
            (5, 17),
            (12, 24),
            (12, 31)
        ]
    );
    // Stockholm: Epiphany, National Day, Midsummer Eve, and five half days.
    let (closed, early) = year_of(&NASDAQ_STOCKHOLM, 2025);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (1, 6),
            (4, 18),
            (4, 21),
            (5, 1),
            (5, 29),
            (6, 6),
            (6, 20),
            (12, 24),
            (12, 25),
            (12, 26),
            (12, 31)
        ]
    );
    assert_eq!(early, [(4, 17), (4, 30), (5, 28), (10, 31)]);
    let (closed, early) = year_of(&NASDAQ_STOCKHOLM, 2026);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (1, 6),
            (4, 3),
            (4, 6),
            (5, 1),
            (5, 14),
            (6, 19),
            (12, 24),
            (12, 25),
            (12, 31)
        ]
    );
    assert_eq!(early, [(1, 5), (4, 2), (4, 30), (5, 13), (10, 30)]);
    let (closed, early) = year_of(&NASDAQ_STOCKHOLM, 2027);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (1, 6),
            (3, 26),
            (3, 29),
            (5, 6),
            (6, 25),
            (12, 24),
            (12, 31)
        ]
    );
    assert_eq!(early, [(1, 5), (3, 25), (4, 30), (5, 5), (11, 5)]);
    // Helsinki: Independence Day on 6 December, a weekday only in 2027.
    let (closed, early) = year_of(&NASDAQ_HELSINKI, 2025);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (1, 6),
            (4, 18),
            (4, 21),
            (5, 1),
            (5, 29),
            (6, 20),
            (12, 24),
            (12, 25),
            (12, 26),
            (12, 31)
        ]
    );
    assert!(early.is_empty());
    let (closed, _) = year_of(&NASDAQ_HELSINKI, 2027);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (1, 6),
            (3, 26),
            (3, 29),
            (5, 6),
            (6, 25),
            (12, 6),
            (12, 24),
            (12, 31)
        ]
    );
    // Iceland: the First Day of Summer, National Day, Commerce Day.
    let (closed, early) = year_of(&NASDAQ_ICELAND, 2025);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (4, 17),
            (4, 18),
            (4, 21),
            (4, 24),
            (5, 1),
            (5, 29),
            (6, 9),
            (6, 17),
            (8, 4),
            (12, 24),
            (12, 25),
            (12, 26),
            (12, 31)
        ]
    );
    assert!(early.is_empty());
    let (closed, _) = year_of(&NASDAQ_ICELAND, 2026);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (4, 2),
            (4, 3),
            (4, 6),
            (4, 23),
            (5, 1),
            (5, 14),
            (5, 25),
            (6, 17),
            (8, 3),
            (12, 24),
            (12, 25),
            (12, 31)
        ]
    );
    let (closed, _) = year_of(&NASDAQ_ICELAND, 2027);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (3, 25),
            (3, 26),
            (3, 29),
            (4, 22),
            (5, 6),
            (5, 17),
            (6, 17),
            (8, 2),
            (12, 24),
            (12, 31)
        ]
    );
}

#[test]
fn an_included_country_lends_its_days_off_and_not_its_observances() {
    // Hong Kong keeps the Winter Solstice, 22 December 2026, as an
    // observance; HKEX's calendar feed has no entry for the day, and the
    // exchange's table does not either.
    let country = HolidayCalendar::for_year(&hc_holiday::countries::HONG_KONG, None, 2026);
    assert!(
        country
            .on(ymd(2026, 12, 22))
            .iter()
            .any(|holiday| !holiday.is_day_off()),
        "the country table keeps the observance"
    );
    let exchange = HolidayCalendar::for_year(&HONG_KONG_EXCHANGES, None, 2026);
    assert!(exchange.on(ymd(2026, 12, 22)).is_empty());
    // Its days off do come along, substitutes with them.
    assert!(exchange.is_holiday(ymd(2026, 12, 25)));
    assert!(exchange.is_holiday(ymd(2026, 4, 7)));
}

#[test]
fn london_closes_on_the_bank_holidays_of_england_and_wales_and_halves_two_days() {
    // The Exchange's business-days table, August 2026 to January 2029.
    let (closed, early) = year_of(&LONDON_STOCK_EXCHANGE, 2026);
    let from_august: Vec<(u8, u8)> = days(&closed)
        .into_iter()
        .filter(|(month, _)| *month >= 8)
        .collect();
    assert_eq!(from_august, [(8, 31), (12, 25), (12, 28)]);
    assert_eq!(early, [(12, 24), (12, 31)]);
    let (closed, early) = year_of(&LONDON_STOCK_EXCHANGE, 2027);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (3, 26),
            (3, 29),
            (5, 3),
            (5, 31),
            (8, 30),
            (12, 27),
            (12, 28)
        ]
    );
    assert_eq!(early, [(12, 24), (12, 31)]);
    let (closed, early) = year_of(&LONDON_STOCK_EXCHANGE, 2028);
    assert_eq!(
        days(&closed),
        [
            (1, 3),
            (4, 14),
            (4, 17),
            (5, 1),
            (5, 29),
            (8, 28),
            (12, 25),
            (12, 26)
        ]
    );
    // The 24th and the 31st are Sundays: the half days are the Fridays.
    assert_eq!(early, [(12, 22), (12, 29)]);
    let (closed, _) = year_of(&LONDON_STOCK_EXCHANGE, 2029);
    assert_eq!(days(&closed).first(), Some(&(1, 1)));
}

#[test]
fn an_included_region_is_the_inclusions_and_not_the_callers() {
    // Scotland keeps 2 January and the first Monday of August, and the
    // Exchange, on England and Wales's days, trades through both.
    let calendar = HolidayCalendar::for_year(&LONDON_STOCK_EXCHANGE, None, 2027);
    assert!(calendar.is_business_day(ymd(2027, 8, 2)));
    assert!(!calendar.is_business_day(ymd(2027, 8, 30)));
    assert!(!calendar.is_business_day(ymd(2027, 3, 29)), "Easter Monday");
}

#[test]
fn six_closes_on_the_days_its_calendar_lists() {
    // SIX's market holidays of the Swiss Stock Exchange, 2026 and 2027: the
    // weekday ones, nothing moved off a weekend, no early close.
    let (closed, early) = year_of(&SIX_SWISS_EXCHANGE, 2026);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (1, 2),
            (4, 3),
            (4, 6),
            (5, 1),
            (5, 14),
            (5, 25),
            (12, 24),
            (12, 25),
            (12, 31)
        ]
    );
    assert!(early.is_empty());
    let (closed, early) = year_of(&SIX_SWISS_EXCHANGE, 2027);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (3, 26),
            (3, 29),
            (5, 6),
            (5, 17),
            (12, 24),
            (12, 31)
        ]
    );
    assert!(early.is_empty());
}

// ─────────────────────────────────────────────────────────────────────────
// Moscow, Johannesburg, Mexico City, Tel Aviv, Riyadh, Istanbul, Warsaw,
// Vienna, Madrid, NZX
// ─────────────────────────────────────────────────────────────────────────

/// The weekday closures in the Moscow Exchange's announcements, which for
/// 2025 and 2026 are also its trading calendar's weekdays without trading
/// or with only the weekend session.
#[rustfmt::skip]
const MISX_CLOSURES: &[(i64, &[(u8, u8)])] = &[
    (2023, &[(1, 2), (2, 23), (3, 8), (5, 1), (5, 9), (6, 12)]),
    (2024, &[(1, 1), (1, 2), (2, 23), (3, 8), (5, 1), (5, 9), (6, 12), (11, 4), (12, 31)]),
    (2025, &[(1, 1), (1, 2), (1, 7), (5, 1), (5, 9), (6, 12), (11, 4), (12, 31)]),
    (2026, &[(1, 1), (1, 2), (1, 7), (2, 23), (5, 1), (6, 12), (11, 4), (12, 31)]),
];

#[test]
fn moscow_closes_on_the_days_its_announcements_list_from_2023_to_2026() {
    for &(y, listed) in MISX_CLOSURES {
        let (closed, early) = year_of(&exchanges::MOSCOW_EXCHANGE, y);
        assert_eq!(days(&closed), listed, "{y}");
        assert!(early.is_empty(), "{y}");
    }
}

#[test]
fn moscow_trades_on_russias_other_days_off_and_its_working_saturdays() {
    let russia = |y| HolidayCalendar::for_year(&hc_holiday::countries::RUSSIA, None, y);
    let moex = |y| HolidayCalendar::for_year(&exchanges::MOSCOW_EXCHANGE, None, y);
    // Days off in Russia on which the exchange trades.
    for (y, m, d) in [
        (2023, 1, 3),
        (2023, 1, 6),
        (2023, 2, 24),
        (2023, 11, 6),
        (2024, 1, 8),
        (2024, 4, 29),
        (2024, 12, 30),
        (2025, 1, 3),
        (2025, 5, 8),
        (2025, 6, 13),
        (2025, 11, 3),
        (2026, 1, 9),
        (2026, 3, 9),
        (2026, 5, 11),
    ] {
        assert!(!russia(y).is_business_day(ymd(y, m, d)), "{y}-{m}-{d}");
        assert!(moex(y).is_business_day(ymd(y, m, d)), "{y}-{m}-{d}");
    }
    // The working Saturdays, traded "в обычном режиме".
    for (y, m, d) in [(2024, 4, 27), (2024, 11, 2), (2024, 12, 28), (2025, 11, 1)] {
        assert!(russia(y).is_business_day(ymd(y, m, d)), "{y}-{m}-{d}");
        assert!(moex(y).is_business_day(ymd(y, m, d)), "{y}-{m}-{d}");
    }
    // A weekend with a weekend-day session is not a trading day of its
    // own, and a 2026 holiday with only that session is named for it.
    assert!(!moex(2026).is_business_day(ymd(2026, 3, 14)));
    assert_eq!(
        moex(2026).on(ymd(2026, 6, 12))[0].name,
        "Russia Day, weekend session only"
    );
    // A year the exchange's days are not read for is a gap.
    assert!(!moex(2022).is_complete());
    assert!(!moex(2027).is_complete());
}

/// Days as (month, day).
type Days = Vec<(u8, u8)>;

/// The closures on the exchange's own trading days, in order, and its
/// partial days, for an exchange whose weekend need not be Saturday and
/// Sunday; the gaps are left to the caller.
fn trading_week_closures(exchange: &RuleSet, year: i64) -> (Days, Days) {
    let calendar = HolidayCalendar::for_year(exchange, None, year);
    let mut closed = Vec::new();
    let mut early = Vec::new();
    for holiday in calendar.all() {
        let Ok((_, month, day)) = gregorian::from_fixed(holiday.date) else {
            panic!("{}", holiday.name);
        };
        assert_eq!(holiday.confidence, Confidence::Exact, "{}", holiday.name);
        if calendar.is_weekend(holiday.date) {
            continue;
        }
        if holiday.is_day_off() {
            if closed.last() != Some(&(month, day)) {
                closed.push((month, day));
            }
        } else {
            assert!(
                holiday.name.starts_with("Early close")
                    || holiday.name.starts_with("Half trading day"),
                "{}",
                holiday.name
            );
            early.push((month, day));
        }
    }
    (closed, early)
}

#[test]
fn johannesburg_closes_on_the_national_holidays_and_the_declared_days() {
    let jse = &exchanges::JOHANNESBURG_STOCK_EXCHANGE;
    // The markets calendars: a Sunday holiday closes the Monday (Youth Day
    // 2024, Freedom Day 2025, Women's Day 2026), a Saturday one nothing
    // (Freedom Day 2024, Women's Day 2025, Human Rights Day 2026), and the
    // declared days close for their year.
    let (closed, early) = year_of(jse, 2024);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (3, 21),
            (3, 29),
            (4, 1),
            (5, 1),
            (5, 29),
            (6, 17),
            (8, 9),
            (9, 24),
            (12, 16),
            (12, 25),
            (12, 26)
        ]
    );
    assert_eq!(early, [(12, 24), (12, 31)]);
    let (closed, early) = year_of(jse, 2025);
    assert_eq!(
        days(&closed),
        [
            (1, 1),
            (3, 21),
            (4, 18),
            (4, 21),
            (4, 28),
            (5, 1),
            (6, 16),
            (9, 24),
            (12, 16),
            (12, 25),
            (12, 26)
        ]
    );
    assert_eq!(early, [(12, 24), (12, 31)]);
    // 2026's early closes are not yet announced: a gap, and no guess.
    let (closed, early) = trading_week_closures(jse, 2026);
    assert_eq!(
        closed,
        [
            (1, 1),
            (4, 3),
            (4, 6),
            (4, 27),
            (5, 1),
            (6, 16),
            (8, 10),
            (9, 24),
            (11, 4),
            (12, 16),
            (12, 25)
        ]
    );
    assert!(early.is_empty());
    assert!(!HolidayCalendar::for_year(jse, None, 2026).is_complete());
    // December 2023: the declared 15th, and the Fridays before the Sunday
    // 24th and 31st closing at noon.
    let calendar = HolidayCalendar::for_year(jse, None, 2023);
    assert!(calendar.is_holiday(ymd(2023, 12, 15)));
    assert!(calendar.is_business_day(ymd(2023, 12, 22)));
    assert_eq!(calendar.on(ymd(2023, 12, 29))[0].name, "Early close, 12:00");
    // The declared days are for their year only.
    let calendar = HolidayCalendar::for_year(jse, None, 2025);
    assert!(calendar.is_business_day(ymd(2025, 5, 29)));
}

/// The weekday closures on the Bolsa Mexicana de Valores' lists.
#[rustfmt::skip]
const XMEX_CLOSURES: &[(i64, &[(u8, u8)])] = &[
    (2019, &[(1, 1), (2, 4), (3, 18), (4, 18), (4, 19), (5, 1), (9, 16), (11, 18), (12, 12), (12, 25)]),
    (2020, &[(1, 1), (2, 3), (3, 16), (4, 9), (4, 10), (5, 1), (9, 16), (11, 2), (11, 16), (12, 25)]),
    (2021, &[(1, 1), (2, 1), (3, 15), (4, 1), (4, 2), (9, 16), (11, 2), (11, 15)]),
    (2022, &[(2, 7), (3, 21), (4, 14), (4, 15), (9, 16), (11, 2), (11, 21), (12, 12)]),
    (2023, &[(2, 6), (3, 20), (4, 6), (4, 7), (5, 1), (11, 2), (11, 20), (12, 12), (12, 25)]),
    // The BMV's page leaves out 1 January; the CNBV's list closes it.
    (2024, &[(1, 1), (2, 5), (3, 18), (3, 28), (3, 29), (5, 1), (9, 16), (10, 1), (11, 18), (12, 12), (12, 25)]),
    (2025, &[(1, 1), (2, 3), (3, 17), (4, 17), (4, 18), (5, 1), (9, 16), (11, 17), (12, 12), (12, 25)]),
    (2026, &[(1, 1), (2, 2), (3, 16), (4, 2), (4, 3), (5, 1), (9, 16), (11, 2), (11, 16), (12, 25)]),
];

#[test]
fn mexico_city_closes_on_the_days_its_lists_give_from_2019_to_2026() {
    for &(y, listed) in XMEX_CLOSURES {
        let (closed, early) = year_of(&exchanges::BOLSA_MEXICANA_DE_VALORES, y);
        assert_eq!(days(&closed), listed, "{y}");
        assert!(early.is_empty(), "{y}");
    }
    // Holy Thursday, 2 November and 12 December are the exchange's, not
    // the country's.
    let country = HolidayCalendar::for_year(&hc_holiday::countries::MEXICO, None, 2026);
    assert!(country.is_business_day(ymd(2026, 4, 2)));
    assert!(country.is_business_day(ymd(2026, 11, 2)));
}

/// The closures on the Tel Aviv Stock Exchange's trading days, from its
/// vacation schedules.
#[rustfmt::skip]
const XTAE_CLOSURES: &[(i64, &[(u8, u8)])] = &[
    (2024, &[(3, 24), (4, 22), (4, 23), (4, 28), (4, 29), (5, 13), (5, 14), (6, 11), (6, 12), (8, 13), (10, 2), (10, 3), (10, 16), (10, 17), (10, 23), (10, 24)]),
    (2025, &[(4, 13), (4, 30), (5, 1), (6, 1), (6, 2), (8, 3), (9, 22), (9, 23), (9, 24), (10, 1), (10, 2), (10, 6), (10, 7), (10, 13), (10, 14)]),
    (2026, &[(1, 2), (3, 3), (4, 1), (4, 2), (4, 7), (4, 8), (4, 21), (4, 22), (5, 21), (5, 22), (7, 23), (9, 11), (9, 18), (9, 21), (9, 25), (10, 2), (10, 27)]),
    (2027, &[(3, 23), (4, 21), (4, 22), (4, 27), (4, 28), (5, 11), (5, 12), (6, 10), (6, 11), (8, 12), (10, 1), (10, 8), (10, 11), (10, 15), (10, 22)]),
];

#[test]
fn tel_aviv_closes_on_the_days_its_schedules_list_from_2024_to_2027() {
    let tase = &exchanges::TEL_AVIV_STOCK_EXCHANGE;
    for &(y, listed) in XTAE_CLOSURES {
        let calendar = HolidayCalendar::for_year(tase, None, y);
        assert!(calendar.is_complete(), "{y}: {:?}", calendar.gaps());
        let (closed, _) = trading_week_closures(tase, y);
        assert_eq!(closed, listed, "{y}");
    }
    let (_, early) = trading_week_closures(tase, 2025);
    assert_eq!(
        early,
        [
            (4, 14),
            (4, 15),
            (4, 16),
            (4, 17),
            (10, 8),
            (10, 9),
            (10, 12)
        ]
    );
    // Sunday to Thursday in 2025, Monday to Friday from January 2026.
    let calendar = HolidayCalendar::for_year(tase, None, 2025);
    assert!(calendar.is_business_day(ymd(2025, 12, 28)));
    assert!(!calendar.is_business_day(ymd(2025, 12, 26)));
    let calendar = HolidayCalendar::for_year(tase, None, 2026);
    assert!(!calendar.is_business_day(ymd(2026, 1, 2)));
    assert!(!calendar.is_business_day(ymd(2026, 1, 4)));
    assert!(calendar.is_business_day(ymd(2026, 1, 9)));
    assert!(!HolidayCalendar::for_year(tase, None, 2023).is_complete());
    assert!(!HolidayCalendar::for_year(tase, None, 2028).is_complete());
}

/// The closures on the Saudi Exchange's trading days, from its holiday
/// announcements.
#[rustfmt::skip]
const XSAU_CLOSURES: &[(i64, &[(u8, u8)])] = &[
    (2023, &[(2, 22), (4, 18), (4, 19), (4, 20), (4, 23), (4, 24), (6, 25), (6, 26), (6, 27), (6, 28), (6, 29), (9, 24)]),
    (2024, &[(2, 22), (4, 7), (4, 8), (4, 9), (4, 10), (4, 11), (6, 16), (6, 17), (6, 18), (6, 19), (6, 20), (9, 23)]),
    (2025, &[(2, 23), (3, 30), (3, 31), (4, 1), (4, 2), (6, 5), (6, 8), (6, 9), (6, 10), (9, 23)]),
    (2026, &[(2, 22), (3, 17), (3, 18), (3, 19), (3, 22), (3, 23), (5, 24), (5, 25), (5, 26), (5, 27), (5, 28), (9, 23)]),
];

#[test]
fn riyadh_closes_on_the_days_its_announcements_give_from_2023_to_2026() {
    let tadawul = &exchanges::SAUDI_EXCHANGE;
    for &(y, listed) in XSAU_CLOSURES {
        let calendar = HolidayCalendar::for_year(tadawul, None, y);
        assert!(calendar.is_complete(), "{y}: {:?}", calendar.gaps());
        let (closed, early) = trading_week_closures(tadawul, y);
        assert_eq!(closed, listed, "{y}");
        assert!(early.is_empty(), "{y}");
    }
    // A Sunday-to-Thursday week: Sunday trades, Friday does not.
    let calendar = HolidayCalendar::for_year(tadawul, None, 2026);
    assert!(calendar.is_business_day(ymd(2026, 3, 15)));
    assert!(!calendar.is_business_day(ymd(2026, 3, 13)));
    assert!(
        calendar.is_business_day(ymd(2026, 3, 24)),
        "trading resumes"
    );
    // The Eid days are announced, not predicted.
    assert!(!HolidayCalendar::for_year(tadawul, None, 2027).is_complete());
}

/// A year's weekday closures and half days, as (month, day).
type ClosuresAndHalves = (i64, &'static [(u8, u8)], &'static [(u8, u8)]);

/// The weekday closures and half days in Borsa İstanbul's tables.
#[rustfmt::skip]
const XIST_DAYS: &[ClosuresAndHalves] = &[
    (2019, &[(1, 1), (4, 23), (5, 1), (6, 4), (6, 5), (6, 6), (7, 15), (8, 12), (8, 13), (8, 14), (8, 30), (10, 29)], &[(6, 3), (10, 28)]),
    (2020, &[(1, 1), (4, 23), (5, 1), (5, 19), (5, 25), (5, 26), (7, 15), (7, 31), (8, 3), (10, 29)], &[(7, 30), (10, 28)]),
    (2021, &[(1, 1), (4, 23), (5, 13), (5, 14), (5, 19), (7, 15), (7, 20), (7, 21), (7, 22), (7, 23), (8, 30), (10, 29)], &[(5, 12), (7, 19), (10, 28)]),
    (2022, &[(5, 2), (5, 3), (5, 4), (5, 19), (7, 11), (7, 12), (7, 15), (8, 30)], &[(7, 8), (10, 28)]),
    (2023, &[(4, 21), (5, 1), (5, 19), (6, 28), (6, 29), (6, 30), (8, 30)], &[(4, 20), (6, 27)]),
    (2024, &[(1, 1), (4, 10), (4, 11), (4, 12), (4, 23), (5, 1), (6, 17), (6, 18), (6, 19), (7, 15), (8, 30), (10, 29)], &[(4, 9), (10, 28)]),
    (2025, &[(1, 1), (3, 31), (4, 1), (4, 23), (5, 1), (5, 19), (6, 6), (6, 9), (7, 15), (10, 29)], &[(6, 5), (10, 28)]),
    (2026, &[(1, 1), (3, 20), (4, 23), (5, 1), (5, 19), (5, 27), (5, 28), (5, 29), (7, 15), (10, 29)], &[(3, 19), (5, 26), (10, 28)]),
];

#[test]
fn istanbul_closes_on_the_days_its_tables_give_from_2019_to_2026() {
    for &(y, listed, halves) in XIST_DAYS {
        let (closed, early) = year_of(&exchanges::BORSA_ISTANBUL, y);
        assert_eq!(days(&closed), listed, "{y}");
        assert_eq!(early, halves, "{y}");
    }
    // An administrative-leave day beside a Bayram is traded.
    let calendar = HolidayCalendar::for_year(&exchanges::BORSA_ISTANBUL, None, 2026);
    assert!(calendar.is_business_day(ymd(2026, 5, 25)));
    assert!(!HolidayCalendar::for_year(&exchanges::BORSA_ISTANBUL, None, 2027).is_complete());
}

/// The weekday "Dni bez sesji" on GPW's pages.
#[rustfmt::skip]
const XWAR_CLOSURES: &[(i64, &[(u8, u8)])] = &[
    (2019, &[(1, 1), (4, 19), (4, 22), (5, 1), (5, 3), (6, 20), (8, 15), (11, 1), (11, 11), (12, 24), (12, 25), (12, 26), (12, 31)]),
    (2020, &[(1, 1), (1, 6), (4, 10), (4, 13), (5, 1), (6, 11), (11, 11), (12, 24), (12, 25), (12, 31)]),
    (2021, &[(1, 1), (1, 6), (4, 2), (4, 5), (5, 3), (6, 3), (11, 1), (11, 11), (12, 24), (12, 31)]),
    (2022, &[(1, 6), (4, 15), (4, 18), (5, 3), (6, 16), (8, 15), (11, 1), (11, 11), (12, 26)]),
    (2023, &[(1, 6), (4, 7), (4, 10), (5, 1), (5, 3), (6, 8), (8, 15), (11, 1), (12, 25), (12, 26)]),
    (2024, &[(1, 1), (3, 29), (4, 1), (5, 1), (5, 3), (5, 30), (8, 15), (11, 1), (11, 11), (12, 24), (12, 25), (12, 26), (12, 31)]),
    (2025, &[(1, 1), (1, 6), (4, 18), (4, 21), (5, 1), (6, 19), (8, 15), (11, 11), (12, 24), (12, 25), (12, 26), (12, 31)]),
    (2026, &[(1, 1), (1, 6), (4, 3), (4, 6), (5, 1), (6, 4), (11, 11), (12, 24), (12, 25), (12, 31)]),
    (2027, &[(1, 1), (1, 6), (3, 26), (3, 29), (5, 3), (5, 27), (11, 1), (11, 11), (12, 24), (12, 31)]),
];

#[test]
fn warsaw_closes_on_the_days_its_pages_list_from_2019_to_2027() {
    for &(y, listed) in XWAR_CLOSURES {
        let (closed, early) = year_of(&exchanges::WARSAW_STOCK_EXCHANGE, y);
        assert_eq!(days(&closed), listed, "{y}");
        assert!(early.is_empty(), "{y}");
    }
    // Christmas Eve 2025 comes from the country's table, once.
    let calendar = HolidayCalendar::for_year(&exchanges::WARSAW_STOCK_EXCHANGE, None, 2025);
    assert_eq!(calendar.on(ymd(2025, 12, 24)).len(), 1);
}

/// The weekday "Handelsfreie Feiertage" of the Wiener Börse.
#[rustfmt::skip]
const XWBO_CLOSURES: &[(i64, &[(u8, u8)])] = &[
    (2019, &[(1, 1), (4, 19), (4, 22), (5, 1), (6, 10), (12, 24), (12, 25), (12, 26), (12, 31)]),
    (2020, &[(1, 1), (4, 10), (4, 13), (5, 1), (6, 1), (10, 26), (12, 24), (12, 25), (12, 31)]),
    (2021, &[(1, 1), (4, 2), (4, 5), (5, 24), (10, 26), (12, 24), (12, 31)]),
    (2022, &[(4, 15), (4, 18), (6, 6), (10, 26), (12, 26)]),
    (2023, &[(4, 7), (4, 10), (5, 1), (10, 26), (12, 25), (12, 26)]),
    (2024, &[(1, 1), (3, 29), (4, 1), (5, 1), (12, 24), (12, 25), (12, 26), (12, 31)]),
    (2025, &[(1, 1), (4, 18), (4, 21), (5, 1), (12, 24), (12, 25), (12, 26), (12, 31)]),
    (2026, &[(1, 1), (4, 3), (4, 6), (5, 1), (10, 26), (12, 24), (12, 25), (12, 31)]),
    (2027, &[(1, 1), (3, 26), (3, 29), (10, 26), (12, 24), (12, 31)]),
];

#[test]
fn vienna_closes_on_the_days_its_lists_give_from_2019_to_2027() {
    for &(y, listed) in XWBO_CLOSURES {
        let (closed, early) = year_of(&exchanges::WIENER_BOERSE, y);
        assert_eq!(days(&closed), listed, "{y}");
        assert!(early.is_empty(), "{y}");
    }
    // Whit Monday traded from 2023; 8 December and 1 November traded.
    let calendar = HolidayCalendar::for_year(&exchanges::WIENER_BOERSE, None, 2026);
    assert!(calendar.is_business_day(ymd(2026, 5, 25)));
    assert!(calendar.is_business_day(ymd(2026, 12, 8)));
}

#[test]
fn madrid_closes_on_the_days_bmes_calendars_list_from_2023_to_2026() {
    let bme = &exchanges::BOLSA_DE_MADRID;
    let (closed, early) = year_of(bme, 2023);
    assert_eq!(days(&closed), [(4, 7), (4, 10), (5, 1), (12, 25), (12, 26)]);
    assert!(early.is_empty());
    for (y, listed) in [
        (2024, [(1, 1), (3, 29), (4, 1), (5, 1), (12, 25), (12, 26)]),
        (2025, [(1, 1), (4, 18), (4, 21), (5, 1), (12, 25), (12, 26)]),
    ] {
        let (closed, early) = year_of(bme, y);
        assert_eq!(days(&closed), listed, "{y}");
        assert_eq!(early, [(12, 24), (12, 31)], "{y}");
    }
    let (closed, early) = year_of(bme, 2026);
    assert_eq!(days(&closed), [(1, 1), (4, 3), (4, 6), (5, 1), (12, 25)]);
    assert_eq!(early, [(12, 24), (12, 31)]);
    // Epiphany and the Immaculate Conception are trading days.
    let calendar = HolidayCalendar::for_year(bme, None, 2026);
    assert!(calendar.is_business_day(ymd(2026, 1, 6)));
    assert!(calendar.is_business_day(ymd(2026, 12, 8)));
}

/// The weekday closures in NZX's market-holiday memos.
#[rustfmt::skip]
const XNZE_CLOSURES: &[(i64, &[(u8, u8)])] = &[
    (2022, &[(1, 3), (1, 4), (2, 7), (4, 15), (4, 18), (4, 25), (6, 6), (6, 24), (9, 26), (10, 24), (12, 26), (12, 27)]),
    (2023, &[(1, 2), (1, 3), (2, 6), (4, 7), (4, 10), (4, 25), (6, 5), (7, 14), (10, 23), (12, 25), (12, 26)]),
    (2024, &[(1, 1), (1, 2), (2, 6), (3, 29), (4, 1), (4, 25), (6, 3), (6, 28), (10, 28), (12, 25), (12, 26)]),
    (2025, &[(1, 1), (1, 2), (2, 6), (4, 18), (4, 21), (4, 25), (6, 2), (6, 20), (10, 27), (12, 25), (12, 26)]),
    (2026, &[(1, 1), (1, 2), (2, 6), (4, 3), (4, 6), (4, 27), (6, 1), (7, 10), (10, 26), (12, 25), (12, 28)]),
];

#[test]
fn nzx_closes_on_the_days_its_memos_list_from_2022_to_2026() {
    for &(y, listed) in XNZE_CLOSURES {
        let (closed, _) = year_of(&exchanges::NZX, y);
        assert_eq!(days(&closed), listed, "{y}");
    }
    // The abbreviated days: the business days before Christmas and the New
    // Year, the Fridays in 2022.
    let (_, early) = year_of(&exchanges::NZX, 2022);
    assert_eq!(early, [(12, 23), (12, 30)]);
    let (_, early) = year_of(&exchanges::NZX, 2026);
    assert_eq!(early, [(12, 24), (12, 31)]);
    // 2027 as far as the memo reaches: 2 January, a Saturday, closes the
    // Monday. Wellington's anniversary day is traded.
    let (closed, _) = year_of(&exchanges::NZX, 2027);
    assert_eq!(days(&closed)[..2], [(1, 1), (1, 4)]);
    let calendar = HolidayCalendar::for_year(&exchanges::NZX, None, 2026);
    assert!(calendar.is_business_day(ymd(2026, 1, 19)));
}

// ─────────────────────────────────────────────────────────────────────────
// Shenzhen, Bangkok, NSE, BSE, Singapore, Kuala Lumpur, Jakarta, Manila
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn shenzhen_closes_on_the_days_its_notices_list_from_2015_to_2026() {
    // The Shenzhen notices for 2015 to 2026 close the same weekdays as the
    // Shanghai ones, 31 December 2018 and 9 February 2024 among them.
    for &(y, listed) in SSE_CLOSURES {
        if y < 2015 {
            continue;
        }
        let (closed, early) = year_of(&exchanges::SHENZHEN_STOCK_EXCHANGE, y);
        assert_eq!(days(&closed), listed, "{y}");
        assert!(early.is_empty(), "{y}");
    }
    let calendar = HolidayCalendar::for_year(&exchanges::SHENZHEN_STOCK_EXCHANGE, None, 2026);
    // 20 September 2026, a Sunday China works, is 周末休市.
    assert!(!calendar.is_business_day(ymd(2026, 9, 20)));
    assert!(!calendar.is_business_day(ymd(2026, 9, 25)));
    assert!(calendar.is_business_day(ymd(2026, 9, 28)));
    assert!(
        !HolidayCalendar::for_year(&exchanges::SHENZHEN_STOCK_EXCHANGE, None, 2027).is_complete()
    );
}

/// The weekday closures on the Stock Exchange of Thailand's holiday pages.
#[rustfmt::skip]
const XBKK_CLOSURES: &[(i64, &[(u8, u8)])] = &[
    (2022, &[(1, 3), (2, 16), (4, 6), (4, 13), (4, 14), (4, 15), (5, 2), (5, 4), (5, 16), (6, 3), (7, 13), (7, 28), (7, 29), (8, 12), (10, 13), (10, 14), (10, 24), (12, 5), (12, 12)]),
    (2023, &[(1, 2), (3, 6), (4, 6), (4, 13), (4, 14), (5, 1), (5, 4), (5, 5), (6, 5), (7, 28), (8, 1), (8, 14), (10, 13), (10, 23), (12, 5), (12, 11), (12, 29)]),
    (2024, &[(1, 1), (2, 26), (4, 8), (4, 12), (4, 15), (4, 16), (5, 1), (5, 6), (5, 22), (6, 3), (7, 22), (7, 29), (8, 12), (10, 14), (10, 23), (12, 5), (12, 10), (12, 31)]),
    (2025, &[(1, 1), (2, 12), (4, 7), (4, 14), (4, 15), (5, 1), (5, 5), (5, 12), (6, 2), (6, 3), (7, 10), (7, 28), (8, 11), (8, 12), (10, 13), (10, 23), (12, 5), (12, 10), (12, 31)]),
    (2026, &[(1, 1), (1, 2), (3, 3), (4, 6), (4, 13), (4, 14), (4, 15), (5, 1), (5, 4), (6, 1), (6, 3), (7, 28), (7, 29), (8, 12), (10, 13), (10, 16), (10, 23), (12, 7), (12, 10), (12, 31)]),
    (2027, &[(1, 1), (2, 22), (4, 6), (4, 13), (4, 14), (4, 15), (5, 3), (5, 4), (5, 20), (6, 3), (7, 19), (7, 28), (8, 12), (10, 13), (10, 25), (12, 6), (12, 10), (12, 31)]),
];

#[test]
fn bangkok_closes_on_the_days_its_pages_list_from_2022_to_2027() {
    let set = &exchanges::STOCK_EXCHANGE_OF_THAILAND;
    for &(y, listed) in XBKK_CLOSURES {
        let (closed, early) = year_of(set, y);
        assert_eq!(days(&closed), listed, "{y}");
        assert!(early.is_empty(), "{y}");
    }
    // A Saturday holiday is substituted too, and Songkran's weekend days
    // by one Tuesday; the Sunday New Year's Eve of 2023 by the Friday.
    let calendar = HolidayCalendar::for_year(set, None, 2024);
    assert_eq!(
        calendar.on(ymd(2024, 4, 16))[0].name,
        "Substitution for Songkran Festival"
    );
    assert_eq!(
        calendar.on(ymd(2024, 2, 26))[0].name,
        "Substitution for Makha Bucha Day"
    );
    let calendar = HolidayCalendar::for_year(set, None, 2023);
    assert_eq!(
        calendar.on(ymd(2023, 12, 29))[0].name,
        "Substitution for New Year's Eve"
    );
    assert_eq!(calendar.on(ymd(2023, 6, 5)).len(), 2);
    // The day the Cabinet added in June 2026.
    let calendar = HolidayCalendar::for_year(set, None, 2026);
    assert_eq!(
        calendar.on(ymd(2026, 10, 16))[0].name,
        "Additional special holiday"
    );
    assert!(!HolidayCalendar::for_year(set, None, 2021).is_complete());
    assert!(!HolidayCalendar::for_year(set, None, 2028).is_complete());
}

/// The weekday trading holidays of the NSE circulars and BSE notices,
/// after the changes made during the year.
#[rustfmt::skip]
const XNSE_CLOSURES: &[(i64, &[(u8, u8)])] = &[
    (2020, &[(2, 21), (3, 10), (4, 2), (4, 6), (4, 10), (4, 14), (5, 1), (5, 25), (10, 2), (11, 16), (11, 30), (12, 25)]),
    (2021, &[(1, 26), (3, 11), (3, 29), (4, 2), (4, 14), (4, 21), (5, 13), (7, 21), (8, 19), (9, 10), (10, 15), (11, 4), (11, 5), (11, 19)]),
    (2022, &[(1, 26), (3, 1), (3, 18), (4, 14), (4, 15), (5, 3), (8, 9), (8, 15), (8, 31), (10, 5), (10, 24), (10, 26), (11, 8)]),
    (2023, &[(1, 26), (3, 7), (3, 30), (4, 4), (4, 7), (4, 14), (5, 1), (6, 29), (8, 15), (9, 19), (10, 2), (10, 24), (11, 14), (11, 27), (12, 25)]),
    (2024, &[(1, 22), (1, 26), (3, 8), (3, 25), (3, 29), (4, 11), (4, 17), (5, 1), (5, 20), (6, 17), (7, 17), (8, 15), (10, 2), (11, 1), (11, 15), (11, 20), (12, 25)]),
    (2025, &[(2, 26), (3, 14), (3, 31), (4, 10), (4, 14), (4, 18), (5, 1), (8, 15), (8, 27), (10, 2), (10, 21), (10, 22), (11, 5), (12, 25)]),
    (2026, &[(1, 15), (1, 26), (3, 3), (3, 26), (3, 31), (4, 3), (4, 14), (5, 1), (5, 28), (6, 26), (9, 14), (10, 2), (10, 20), (11, 10), (11, 24), (12, 25)]),
];

#[test]
fn the_indian_exchanges_close_on_the_days_their_circulars_list_from_2020_to_2026() {
    for set in [
        &exchanges::NATIONAL_STOCK_EXCHANGE_OF_INDIA,
        &exchanges::BSE,
    ] {
        for &(y, listed) in XNSE_CLOSURES {
            let (closed, early) = year_of(set, y);
            assert_eq!(days(&closed), listed, "{} {y}", set.code);
            assert!(early.is_empty(), "{} {y}", set.code);
        }
        // Bakri Id 2023 moved from the Wednesday to the Thursday.
        let calendar = HolidayCalendar::for_year(set, None, 2023);
        assert!(calendar.is_business_day(ymd(2023, 6, 28)));
        assert!(!calendar.is_business_day(ymd(2023, 6, 29)));
        // A weekday Muhurat day is a closure that says the session is
        // held; a Sunday one is not a trading day of its own.
        let calendar = HolidayCalendar::for_year(set, None, 2025);
        assert_eq!(
            calendar.on(ymd(2025, 10, 21))[0].name,
            "Diwali Laxmi Pujan (Muhurat trading session held)"
        );
        let calendar = HolidayCalendar::for_year(set, None, 2026);
        assert!(!calendar.is_business_day(ymd(2026, 11, 8)));
        assert_eq!(calendar.on(ymd(2026, 11, 8)).len(), 1);
        // The Union Budget sessions on a Saturday and a Sunday, and the
        // three Saturdays of 2024, are trading days.
        assert!(calendar.is_business_day(ymd(2026, 2, 1)));
        let calendar = HolidayCalendar::for_year(set, None, 2024);
        for (m, d) in [(1, 20), (3, 2), (5, 18)] {
            assert!(calendar.is_business_day(ymd(2024, m, d)), "{m}-{d}");
        }
        assert!(!calendar.is_business_day(ymd(2024, 1, 27)));
        assert!(!HolidayCalendar::for_year(set, None, 2019).is_complete());
        assert!(!HolidayCalendar::for_year(set, None, 2027).is_complete());
    }
}

/// The weekday closures and half days of the Singapore Exchange: the
/// Ministry of Manpower's holidays, and the half days SGX prints.
#[rustfmt::skip]
const XSES_DAYS: &[ClosuresAndHalves] = &[
    (2020, &[(1, 1), (1, 27), (4, 10), (5, 1), (5, 7), (5, 25), (7, 10), (7, 31), (8, 10), (12, 25)], &[(1, 24), (12, 24), (12, 31)]),
    (2021, &[(1, 1), (2, 12), (4, 2), (5, 13), (5, 26), (7, 20), (8, 9), (11, 4)], &[(2, 11), (12, 24), (12, 31)]),
    (2022, &[(2, 1), (2, 2), (4, 15), (5, 2), (5, 3), (5, 16), (7, 11), (8, 9), (10, 24), (12, 26)], &[(1, 31)]),
    (2023, &[(1, 2), (1, 23), (1, 24), (4, 7), (5, 1), (6, 2), (6, 29), (8, 9), (9, 1), (11, 13), (12, 25)], &[]),
    (2024, &[(1, 1), (2, 12), (3, 29), (4, 10), (5, 1), (5, 22), (6, 17), (8, 9), (10, 31), (12, 25)], &[(2, 9), (12, 24), (12, 31)]),
    (2025, &[(1, 1), (1, 29), (1, 30), (3, 31), (4, 18), (5, 1), (5, 12), (10, 20), (12, 25)], &[(1, 28), (12, 24), (12, 31)]),
    (2026, &[(1, 1), (2, 17), (2, 18), (4, 3), (5, 1), (5, 27), (6, 1), (8, 10), (11, 9), (12, 25)], &[(2, 16), (12, 24), (12, 31)]),
];

#[test]
fn singapore_closes_on_the_ministrys_days_from_2020_to_2026_and_halves_three_eves() {
    let sgx = &exchanges::SINGAPORE_EXCHANGE;
    for &(y, listed, halves) in XSES_DAYS {
        let (closed, early) = year_of(sgx, y);
        assert_eq!(days(&closed), listed, "{y}");
        assert_eq!(early, halves, "{y}");
    }
    // A Sunday holiday closes the Monday — the Tuesday when the Monday is
    // Chinese New Year — and a Saturday one nothing.
    let calendar = HolidayCalendar::for_year(sgx, None, 2023);
    assert_eq!(
        calendar.on(ymd(2023, 1, 24))[0].name,
        "Chinese New Year holiday, in lieu of the Sunday"
    );
    assert!(calendar.is_business_day(ymd(2023, 4, 24)));
    // Polling Day, and the country table's day off the exchange does not
    // add.
    assert_eq!(calendar.on(ymd(2023, 9, 1))[0].name, "Polling Day");
    assert!(!HolidayCalendar::for_year(sgx, None, 2019).is_complete());
    assert!(!HolidayCalendar::for_year(sgx, None, 2027).is_complete());
}

/// The weekday closures and half days on Bursa Malaysia's calendar pages.
#[rustfmt::skip]
const XKLS_DAYS: &[ClosuresAndHalves] = &[
    (2020, &[(1, 1), (1, 27), (5, 1), (5, 7), (5, 11), (5, 25), (5, 26), (6, 8), (7, 31), (8, 20), (8, 31), (9, 16), (10, 29), (12, 25)], &[(1, 24)]),
    (2021, &[(1, 1), (1, 28), (2, 1), (2, 12), (4, 29), (5, 13), (5, 14), (5, 26), (6, 7), (7, 20), (8, 10), (8, 31), (9, 16), (10, 19), (11, 4), (12, 3)], &[(2, 11), (5, 12)]),
    (2022, &[(1, 18), (2, 1), (2, 2), (4, 19), (5, 2), (5, 3), (5, 4), (5, 16), (6, 6), (7, 11), (8, 31), (9, 16), (10, 10), (10, 24), (11, 28), (12, 26)], &[(1, 31)]),
    (2023, &[(1, 2), (1, 23), (1, 24), (2, 1), (2, 6), (4, 21), (4, 24), (5, 1), (5, 4), (6, 5), (6, 29), (7, 19), (8, 31), (9, 28), (11, 13), (12, 25)], &[]),
    (2024, &[(1, 1), (1, 25), (2, 1), (2, 12), (3, 28), (4, 10), (4, 11), (5, 1), (5, 22), (6, 3), (6, 17), (7, 8), (9, 16), (9, 17), (10, 31), (12, 25)], &[]),
    (2025, &[(1, 1), (1, 29), (1, 30), (2, 11), (3, 18), (3, 31), (4, 1), (5, 1), (5, 12), (6, 2), (6, 27), (9, 1), (9, 5), (9, 15), (9, 16), (10, 20), (12, 25)], &[]),
    (2026, &[(1, 1), (2, 2), (2, 17), (2, 18), (3, 20), (3, 23), (5, 1), (5, 27), (6, 1), (6, 17), (8, 25), (8, 31), (9, 16), (11, 9), (12, 25)], &[]),
];

#[test]
fn kuala_lumpur_closes_on_the_days_its_pages_give_from_2020_to_2026() {
    let bursa = &exchanges::BURSA_MALAYSIA;
    for &(y, listed, halves) in XKLS_DAYS {
        let (closed, early) = year_of(bursa, y);
        assert_eq!(days(&closed), listed, "{y}");
        assert_eq!(early, halves, "{y}");
    }
    // The Federal Territory's days, and the Labuan-only festival traded.
    let calendar = HolidayCalendar::for_year(bursa, None, 2024);
    assert_eq!(
        calendar.on(ymd(2024, 2, 1))[0].name,
        "Federal Territory Day"
    );
    assert!(calendar.is_business_day(ymd(2024, 5, 30)));
    // Two holidays on one Monday close the Tuesday after.
    assert_eq!(calendar.on(ymd(2024, 9, 16)).len(), 2);
    assert_eq!(
        calendar.on(ymd(2024, 9, 17))[0].name,
        "Birthday of Prophet Muhammad holiday, in lieu"
    );
    // Hari Raya Puasa 2026 as the gazette set it, and the Tuesday after
    // the Agong's Birthday not carried.
    let calendar = HolidayCalendar::for_year(bursa, None, 2026);
    assert!(!calendar.is_business_day(ymd(2026, 3, 20)));
    assert!(calendar.is_business_day(ymd(2026, 3, 19)));
    assert!(calendar.is_business_day(ymd(2026, 6, 2)));
    assert!(!HolidayCalendar::for_year(bursa, None, 2019).is_complete());
    assert!(!HolidayCalendar::for_year(bursa, None, 2027).is_complete());
}

/// The weekday closures of the Indonesia Stock Exchange's calendars, each
/// in its last version read; 2023 was not read.
#[rustfmt::skip]
const XIDX_CLOSURES: &[(i64, &[(u8, u8)])] = &[
    (2020, &[(1, 1), (3, 25), (4, 10), (5, 1), (5, 7), (5, 21), (5, 22), (5, 25), (6, 1), (7, 31), (8, 17), (8, 20), (8, 21), (10, 28), (10, 29), (10, 30), (12, 9), (12, 24), (12, 25), (12, 31)]),
    (2021, &[(1, 1), (2, 12), (3, 11), (4, 2), (5, 12), (5, 13), (5, 14), (5, 26), (6, 1), (7, 20), (8, 11), (8, 17), (10, 20), (12, 31)]),
    (2022, &[(2, 1), (2, 28), (3, 3), (4, 15), (4, 29), (5, 2), (5, 3), (5, 4), (5, 5), (5, 6), (5, 16), (5, 26), (6, 1), (8, 17)]),
    (2024, &[(1, 1), (2, 8), (2, 9), (2, 14), (3, 11), (3, 12), (3, 29), (4, 8), (4, 9), (4, 10), (4, 11), (4, 12), (4, 15), (5, 1), (5, 9), (5, 10), (5, 23), (5, 24), (6, 17), (6, 18), (9, 16), (11, 27), (12, 25), (12, 26), (12, 31)]),
    (2025, &[(1, 1), (1, 27), (1, 28), (1, 29), (3, 28), (3, 31), (4, 1), (4, 2), (4, 3), (4, 4), (4, 7), (4, 18), (5, 1), (5, 12), (5, 13), (5, 29), (5, 30), (6, 6), (6, 9), (6, 27), (8, 18), (9, 5), (12, 25), (12, 26), (12, 31)]),
    (2026, &[(1, 1), (1, 16), (2, 16), (2, 17), (3, 18), (3, 19), (3, 20), (3, 23), (3, 24), (4, 3), (5, 1), (5, 14), (5, 15), (5, 27), (5, 28), (6, 1), (6, 16), (8, 17), (8, 25), (12, 24), (12, 25), (12, 31)]),
];

/// The trading days each calendar totals.
const XIDX_TRADING_DAYS: &[(i64, usize)] = &[
    (2020, 242),
    (2021, 247),
    (2022, 246),
    (2024, 237),
    (2025, 236),
    (2026, 239),
];

#[test]
fn jakarta_closes_on_the_days_its_calendars_list_and_2023_is_a_gap() {
    let idx = &exchanges::INDONESIA_STOCK_EXCHANGE;
    for &(y, listed) in XIDX_CLOSURES {
        let (closed, early) = year_of(idx, y);
        assert_eq!(days(&closed), listed, "{y}");
        assert!(early.is_empty(), "{y}");
    }
    // The calendars' own totals of trading days.
    for &(y, total) in XIDX_TRADING_DAYS {
        let calendar = HolidayCalendar::for_year(idx, None, y);
        let mut count = 0;
        let mut day = ymd(y, 1, 1);
        while day < ymd(y + 1, 1, 1) {
            if calendar.is_business_day(day) {
                count += 1;
            }
            day = Rd(day.0 + 1);
        }
        assert_eq!(count, total, "{y}");
    }
    // The joint-leave day the government cancelled in 2021 was traded.
    let calendar = HolidayCalendar::for_year(idx, None, 2021);
    assert!(calendar.is_business_day(ymd(2021, 12, 24)));
    assert_eq!(
        calendar.on(ymd(2021, 12, 31))[0].name,
        "Exchange holiday, the last day of the year"
    );
    for y in [2019, 2023, 2027] {
        assert!(
            !HolidayCalendar::for_year(idx, None, y).is_complete(),
            "{y}"
        );
    }
}

/// The weekday non-trading days and half days in the Philippine Stock
/// Exchange's memoranda.
#[rustfmt::skip]
const XPHS_DAYS: &[ClosuresAndHalves] = &[
    (2020, &[(1, 1), (1, 13), (2, 25), (3, 17), (3, 18), (4, 9), (4, 10), (5, 1), (5, 25), (6, 12), (7, 31), (8, 21), (8, 31), (11, 2), (11, 12), (11, 30), (12, 8), (12, 24), (12, 25), (12, 30), (12, 31)], &[]),
    (2021, &[(1, 1), (2, 12), (2, 25), (4, 1), (4, 2), (4, 9), (5, 13), (7, 20), (8, 30), (11, 1), (11, 30), (12, 8), (12, 30)], &[(12, 24), (12, 31)]),
    (2022, &[(1, 4), (2, 1), (2, 25), (4, 14), (4, 15), (5, 3), (5, 9), (8, 29), (9, 26), (10, 31), (11, 1), (11, 30), (12, 8), (12, 26), (12, 30)], &[]),
    (2023, &[(1, 2), (2, 24), (4, 6), (4, 7), (4, 10), (4, 21), (5, 1), (6, 12), (6, 28), (8, 21), (8, 28), (10, 30), (11, 1), (11, 2), (11, 27), (12, 8), (12, 25), (12, 26)], &[]),
    (2024, &[(1, 1), (2, 9), (3, 28), (3, 29), (4, 9), (4, 10), (5, 1), (6, 12), (6, 17), (7, 24), (8, 23), (8, 26), (11, 1), (12, 24), (12, 25), (12, 30), (12, 31)], &[]),
    (2025, &[(1, 1), (1, 29), (4, 1), (4, 9), (4, 17), (4, 18), (5, 1), (5, 12), (6, 6), (6, 12), (8, 21), (8, 25), (10, 31), (12, 8), (12, 24), (12, 25), (12, 30), (12, 31)], &[]),
    (2026, &[(1, 1), (2, 17), (3, 20), (4, 2), (4, 3), (4, 9), (5, 1), (5, 27), (6, 12), (8, 21), (8, 31), (11, 2), (11, 30), (12, 8), (12, 24), (12, 25), (12, 30), (12, 31)], &[]),
];

#[test]
fn manila_closes_on_the_days_its_memoranda_give_from_2020_to_2026() {
    let pse = &exchanges::PHILIPPINE_STOCK_EXCHANGE;
    for &(y, listed, halves) in XPHS_DAYS {
        let (closed, early) = year_of(pse, y);
        assert_eq!(days(&closed), listed, "{y}");
        assert_eq!(early, halves, "{y}");
    }
    // A day moved by proclamation, an unscheduled closure, and the day
    // the exchange traded when the Wednesday was first announced.
    let calendar = HolidayCalendar::for_year(pse, None, 2024);
    assert_eq!(calendar.on(ymd(2024, 8, 23))[0].name, "Ninoy Aquino Day");
    assert!(calendar.is_business_day(ymd(2024, 8, 21)));
    assert_eq!(
        calendar.on(ymd(2024, 7, 24))[0].name,
        "Trading suspension, inclement weather and floods"
    );
    // EDSA Day 2025, in no memorandum, is a trading day.
    let calendar = HolidayCalendar::for_year(pse, None, 2025);
    assert!(calendar.is_business_day(ymd(2025, 2, 25)));
    assert!(!HolidayCalendar::for_year(pse, None, 2019).is_complete());
    assert!(!HolidayCalendar::for_year(pse, None, 2027).is_complete());
}

#[test]
fn the_catalogue_is_keyed_by_market_identifier_code() {
    assert_eq!(
        exchanges::by_code("xnys").map(|e| e.english_name),
        Some("New York Stock Exchange")
    );
    assert!(exchanges::ALL.iter().all(|e| e.code.len() == 4));
    assert_eq!(exchanges::ALL.len(), 42);
}
