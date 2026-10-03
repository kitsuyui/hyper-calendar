//! A day a gap leaves open is refused, not answered (ADR 0013, ADR 0015,
//! audit 10 d1): whether it is a day off, whether it counts as a business
//! day, and the next and previous holiday, over a table invented for the
//! test whose gaps and weekends are the ones each case needs.

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;
use hc_holiday::engine::{HolidayCalendar, Unanswered};
use hc_holiday::rule::{HolidayRule, Kind, Rule, RuleSet, SourceDate, Subdivisions, WeekendPolicy};

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

static RULES: &[HolidayRule] = &[
    // Kept in every year: a day the table always knows.
    HolidayRule::public("Founding Day", "", Rule::gregorian(3, 11)),
    // Read from 2000: its day in an earlier year is not known.
    HolidayRule::public("Announced Day", "", Rule::gregorian(9, 9)).read_from(2000),
    // An observance that stops no work, read from 2010.
    HolidayRule::observance("Flag Day", "", Rule::gregorian(6, 14)).read_from(2010),
    // A weekend day made a working day, read from 2015.
    HolidayRule::workday("Make-up Day", "", Rule::gregorian(3, 14)).read_from(2015),
];

/// Saturday and Sunday for the country; Friday and Saturday in `XX-F`
/// from 2010, and a law not read there before.
static WEEKENDS: &[WeekendPolicy] = &[
    WeekendPolicy {
        days: &[Weekday::Saturday, Weekday::Sunday],
        regions: &[],
        valid_from: None,
        valid_from_day: None,
        valid_until: None,
        valid_until_day: None,
    },
    WeekendPolicy {
        days: &[],
        regions: &["XX-F"],
        valid_from: None,
        valid_from_day: None,
        valid_until: Some(2009),
        valid_until_day: None,
    },
    WeekendPolicy {
        days: &[Weekday::Friday, Weekday::Saturday],
        regions: &["XX-F"],
        valid_from: Some(2010),
        valid_from_day: None,
        valid_until: None,
        valid_until_day: None,
    },
];

static TABLE: RuleSet = RuleSet {
    code: "XX",
    english_name: "A test country",
    rules: RULES,
    substitution: &[],
    bridges: &[],
    includes: &[],
    weekend: WEEKENDS,
    sources_checked: SourceDate::new(2026, 10, 3),
    sources: "invented for the test",
    subdivisions: Subdivisions::Read(&["XX-F"]),
};

fn calendar(region: Option<&'static str>, first: i64, last: i64) -> HolidayCalendar<'static> {
    HolidayCalendar::new(&TABLE, region, first, last)
}

#[test]
fn a_day_without_an_entry_is_a_day_off_that_is_not_only_where_the_table_is_whole() {
    // 2026 has no gap: an entry is a day off and every other day is not.
    let whole = calendar(None, 2026, 2026);
    assert!(whole.gaps().is_empty());
    assert_eq!(whole.day_off(ymd(2026, 3, 11)), Ok(true));
    assert_eq!(whole.day_off(ymd(2026, 3, 12)), Ok(false));
    // A weekend day is not a holiday, and the answer is not open.
    assert_eq!(whole.day_off(ymd(2026, 3, 14)), Ok(false));
    // 1995 is before "Announced Day" was read: its day is open, so every
    // day that is not an entry is refused, and the entry itself is not.
    let early = calendar(None, 1995, 1995);
    assert_eq!(early.day_off(ymd(1995, 3, 11)), Ok(true));
    assert_eq!(early.day_off(ymd(1995, 3, 12)), Err(Unanswered::Gap));
    assert_eq!(early.day_off(ymd(1995, 9, 9)), Err(Unanswered::Gap));
    // The ordinary answer for a day outside the span is its own refusal.
    assert_eq!(
        early.day_off(ymd(1996, 3, 12)),
        Err(Unanswered::OutsideSpan)
    );
}

#[test]
fn only_a_gap_of_a_kind_that_stops_work_opens_a_day_off() {
    // 2005 is before the Flag Day observance was read (2010) and the
    // Make-up Day (2015), and after the public Announced Day was (2000): an
    // observance and a work day stop no work, so their gaps leave "is it a
    // day off" closed.
    let calendar_2005 = calendar(None, 2005, 2005);
    assert_eq!(calendar_2005.gaps().len(), 2);
    assert!(
        calendar_2005
            .gaps()
            .iter()
            .all(|gap| gap.kind == Kind::Observance || gap.kind == Kind::Workday)
    );
    assert_eq!(calendar_2005.day_off(ymd(2005, 6, 14)), Ok(false));
    assert_eq!(calendar_2005.business_day(ymd(2005, 6, 14)), Ok(true));
    // A weekend day is a working day only where a work-day entry says so,
    // and the gap of one leaves a weekend day open: 2012 is before
    // "Make-up Day" was read (2015).
    let calendar_2012 = calendar(None, 2012, 2012);
    assert_eq!(calendar_2012.gaps().len(), 1);
    assert_eq!(calendar_2012.gaps()[0].kind, Kind::Workday);
    let saturday = ymd(2012, 3, 17);
    assert_eq!(Weekday::from_rd(saturday), Weekday::Saturday);
    assert_eq!(calendar_2012.business_day(saturday), Err(Unanswered::Gap));
    // A weekday is not made open by a work-day gap, nor is a day off.
    assert_eq!(calendar_2012.business_day(ymd(2012, 3, 12)), Ok(true));
    assert_eq!(calendar_2012.day_off(ymd(2012, 3, 17)), Ok(false));
    // Once it is read the designated day counts, a Saturday's.
    let calendar_2026 = calendar(None, 2026, 2026);
    assert_eq!(Weekday::from_rd(ymd(2026, 3, 14)), Weekday::Saturday);
    assert_eq!(calendar_2026.business_day(ymd(2026, 3, 14)), Ok(true));
    assert_eq!(calendar_2026.business_day(ymd(2026, 3, 15)), Ok(false));
}

#[test]
fn a_weekend_law_not_read_refuses_the_day_and_the_walk() {
    // XX-F's weekend law before 2010 was not read; the country's was.
    let before = calendar(Some("XX-F"), 2009, 2009);
    let friday = ymd(2009, 5, 15);
    assert_eq!(Weekday::from_rd(friday), Weekday::Friday);
    assert!(!before.weekend_is_read(friday));
    assert_eq!(before.day_off(friday), Err(Unanswered::UnreadWeekend));
    assert_eq!(before.business_day(friday), Err(Unanswered::UnreadWeekend));
    assert_eq!(
        before.try_add_business_days(ymd(2009, 5, 13), 3),
        Err(Unanswered::UnreadWeekend)
    );
    assert_eq!(before.add_business_days(ymd(2009, 5, 13), 3), None);
    // An entry is a day off whatever the weekend: Founding Day is a Wednesday
    // of 2009, whose law was not read, and it stops work.
    assert_eq!(before.day_off(ymd(2009, 3, 11)), Ok(true));
    // The country is not refused by its state's unread law: a Friday is a
    // working day there.
    let country = calendar(None, 2009, 2009);
    assert!(country.weekend_is_read(friday));
    assert_eq!(country.business_day(friday), Ok(true));
}

#[test]
fn the_walk_over_a_whole_year_counts_a_states_own_weekend() {
    // 2026: Friday and Saturday off in XX-F, Saturday and Sunday elsewhere.
    // Thursday 5 March + 1 is Sunday in the state, Friday elsewhere.
    let thursday = ymd(2026, 3, 5);
    let state = calendar(Some("XX-F"), 2026, 2026);
    let country = calendar(None, 2026, 2026);
    assert!(state.gaps().is_empty() && country.gaps().is_empty());
    assert_eq!(
        state.try_add_business_days(thursday, 1),
        Ok(ymd(2026, 3, 8))
    );
    assert_eq!(
        country.try_add_business_days(thursday, 1),
        Ok(ymd(2026, 3, 6))
    );
    assert_eq!(
        state.try_business_days_between(ymd(2026, 3, 8), ymd(2026, 3, 15)),
        Ok(5)
    );
    // Founding Day, Wednesday 11 March, is skipped: Tuesday + 1 is Thursday.
    assert_eq!(
        country.try_add_business_days(ymd(2026, 3, 10), 1),
        Ok(ymd(2026, 3, 12))
    );
}

#[test]
fn a_walk_that_reaches_a_day_a_gap_leaves_open_is_refused_and_one_that_does_not_is_not() {
    // 1999 and 2000 straddle the year Announced Day was first read in.
    let span = calendar(None, 1999, 2001);
    assert!(!span.gaps().is_empty());
    // 1999 is open; a walk that stays in 2000 is not.
    assert_eq!(
        span.try_add_business_days(ymd(1999, 12, 28), 1),
        Err(Unanswered::Gap)
    );
    assert_eq!(
        span.try_add_business_days(ymd(2000, 4, 3), 1),
        Ok(ymd(2000, 4, 4))
    );
    // The first business day of 2000 is reached from the last of 1999 only
    // by crossing the gap: the count is refused, whichever way it runs.
    assert_eq!(
        span.try_business_days_between(ymd(1999, 12, 30), ymd(2000, 1, 5)),
        Err(Unanswered::Gap)
    );
    assert_eq!(
        span.try_business_days_between(ymd(2000, 1, 3), ymd(2000, 1, 7)),
        Ok(4)
    );
    // The Option methods are the same answers without the reason.
    assert_eq!(span.add_business_days(ymd(1999, 12, 28), 1), None);
    assert_eq!(
        span.business_days_between(ymd(2000, 1, 3), ymd(2000, 1, 7)),
        Some(4)
    );
    // A walk out of the span is its own refusal.
    assert_eq!(
        span.try_add_business_days(ymd(2001, 12, 31), 1),
        Err(Unanswered::OutsideSpan)
    );
}

#[test]
fn the_next_and_previous_holiday_are_refused_where_a_gap_could_hide_a_nearer_one() {
    let span = calendar(None, 1998, 2003);
    // From 1 January 2000 the next day off is Founding Day, 11 March: no
    // gap lies in 2000.
    let next = span.try_next_of(ymd(2000, 1, 1), &[]).expect("closed");
    assert_eq!(next.map(|holiday| holiday.date), Some(ymd(2000, 3, 11)));
    // From 1 January 1999 the next entry is Founding Day too, but the
    // Announced Day of 1999 is open and could be nearer: refused.
    assert_eq!(span.try_next_of(ymd(1999, 1, 1), &[]), Err(Unanswered::Gap));
    // Back from 1 January 2001: the last day off is Announced Day, 9 Sep
    // 2000, with no gap between.
    let previous = span.try_previous_of(ymd(2001, 1, 1), &[]).expect("closed");
    assert_eq!(previous.map(|holiday| holiday.date), Some(ymd(2000, 9, 9)));
    // Back from 1 January 2000 the open 1999 lies between the day and
    // anything before it.
    assert_eq!(
        span.try_previous_of(ymd(2000, 1, 1), &[]),
        Err(Unanswered::Gap)
    );
    // The kinds asked for decide both which entry and which gap counts: the
    // observance's gap in 2005 leaves the next day off closed, and the next
    // observance open.
    let later = calendar(None, 2005, 2008);
    assert_eq!(
        later
            .try_next_of(ymd(2005, 1, 1), &[])
            .expect("closed")
            .map(|holiday| holiday.date),
        Some(ymd(2005, 3, 11))
    );
    assert_eq!(
        later.try_next_of(ymd(2005, 1, 1), &[Kind::Observance]),
        Err(Unanswered::Gap)
    );
    // A span with no entry of the kind and no gap says there is none.
    let flagless = calendar(None, 2026, 2026);
    assert_eq!(
        flagless
            .try_next_of(ymd(2026, 12, 30), &[])
            .expect("closed"),
        None
    );
    assert_eq!(
        flagless.try_next_of(ymd(2027, 1, 1), &[]),
        Err(Unanswered::OutsideSpan)
    );
}

/// 2151 is past the lunisolar range China's festivals are dated in, so the
/// year has gaps and a day that is not a fixed holiday might be one of them
/// (the example of `hc-holiday`'s README).
#[test]
fn a_year_past_a_calendars_range_leaves_the_unlisted_days_open() {
    let calendar = HolidayCalendar::for_year(&hc_holiday::countries::CHINA, None, 2151);
    assert!(!calendar.is_complete());
    assert_eq!(calendar.day_off(ymd(2151, 3, 4)), Err(Unanswered::Gap));
    assert_eq!(calendar.day_off(ymd(2151, 1, 1)), Ok(true));
    assert!(!calendar.is_holiday(ymd(2151, 3, 4)));
}

/// A subdivision the table was not read for is a gap whatever kinds are
/// asked for, as the year's lines write it: its own days of any kind may be
/// missing, a work day on a weekend included.
#[test]
fn a_subdivision_not_read_is_open_whatever_the_kind() {
    let unread = calendar(Some("XX-Z"), 2026, 2027);
    assert_eq!(unread.gaps().len(), 2, "one for each year");
    // The next observance of the table is open: the region's own may be
    // nearer.
    assert_eq!(
        unread.try_next_of(ymd(2026, 1, 1), &[Kind::Observance]),
        Err(Unanswered::Gap)
    );
    // A Saturday is a weekend day, but a work day of the region's own may
    // make it a business day.
    let saturday = ymd(2026, 3, 21);
    assert_eq!(Weekday::from_rd(saturday), Weekday::Saturday);
    assert_eq!(unread.business_day(saturday), Err(Unanswered::Gap));
    // The same day in the table, whole in 2026, is not a business day.
    assert_eq!(
        calendar(None, 2026, 2026).business_day(ymd(2026, 3, 15)),
        Ok(false)
    );
}
