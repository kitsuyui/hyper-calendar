//! The exchange calendars against what the exchanges publish.

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;
use hc_holiday::engine::HolidayCalendar;
use hc_holiday::exchanges::{
    self, AUSTRALIAN_SECURITIES_EXCHANGE, B3, EURONEXT_AMSTERDAM, EURONEXT_BRUSSELS,
    EURONEXT_DUBLIN, EURONEXT_LISBON, EURONEXT_MILAN, EURONEXT_OSLO, EURONEXT_PARIS,
    FRANKFURT_STOCK_EXCHANGE, HONG_KONG_EXCHANGES, KOREA_EXCHANGE, LONDON_STOCK_EXCHANGE, NASDAQ,
    NASDAQ_COPENHAGEN, NASDAQ_HELSINKI, NASDAQ_ICELAND, NASDAQ_STOCKHOLM, NEW_YORK_STOCK_EXCHANGE,
    SHANGHAI_STOCK_EXCHANGE, SIX_SWISS_EXCHANGE, TOKYO_STOCK_EXCHANGE, TORONTO_STOCK_EXCHANGE,
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

#[test]
fn the_catalogue_is_keyed_by_market_identifier_code() {
    assert_eq!(
        exchanges::by_code("xnys").map(|e| e.english_name),
        Some("New York Stock Exchange")
    );
    assert!(exchanges::ALL.iter().all(|e| e.code.len() == 4));
    assert_eq!(exchanges::ALL.len(), 23);
}
