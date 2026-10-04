//! A gap carries the days its holiday could fall on, and only those days
//! are refused (ADR 0013).
//!
//! A rule whose shape the year can compute gives the days it would place,
//! widened by the days a substitution can move one; a table of announced
//! dates that has run out gives the span of dates the day took in the years
//! it lists; a rule with nothing to narrow it leaves the whole year open.

use hc_calendar::{Rd, Weekday};
use hc_calendars_solar::gregorian;
use hc_holiday::engine::{HolidayCalendar, Unanswered};
use hc_holiday::rule::{
    HolidayRule, Listing, NO_WEEKEND, Rule, RuleSet, SATURDAY_SUNDAY, SourceDate, Subdivisions,
    SubstituteDirection, SubstitutionPolicy,
};

fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

/// The days an announcement lists, read for 2010 to 2012 only.
static ANNOUNCED: Listing = Listing::Dates(&[(2010, 3, 5), (2011, 3, 9), (2012, 3, 7)]);

static RULES: &[HolidayRule] = &[
    // Read from 2000: a day of the year it falls on, 9 September.
    HolidayRule::public("Founding Day", "", Rule::gregorian(9, 9)).read_from(2000),
    // Announced for 2010 to 2012 and for no other year.
    HolidayRule::public(
        "Spring Day",
        "",
        Rule::listed(ANNOUNCED.every(), 2010, 2012),
    ),
    // Read in no year at all.
    HolidayRule::public("Lost Day", "", Rule::UNREAD).years(Some(2030), Some(2030)),
];

static MOVED: &[SubstitutionPolicy] = &[SubstitutionPolicy {
    trigger: &[Weekday::Saturday, Weekday::Sunday],
    direction: SubstituteDirection::Forward,
    skip_occupied: false,
    on_collision: false,
    regions: &[],
    avoid: &[],
    valid_from: None,
    valid_until: None,
}];

const fn table(substitution: &'static [SubstitutionPolicy]) -> RuleSet {
    RuleSet {
        code: "XX",
        english_name: "A test country",
        rules: RULES,
        substitution,
        bridges: &[],
        includes: &[],
        weekend: SATURDAY_SUNDAY,
        sources_checked: SourceDate::new(2026, 10, 4),
        sources: "invented for the test",
        subdivisions: Subdivisions::Undivided,
    }
}

static PLAIN: RuleSet = table(&[]);
static SUBSTITUTING: RuleSet = table(MOVED);

fn gap_of<'a>(calendar: &'a HolidayCalendar<'_>, name: &str) -> &'a hc_holiday::engine::Gap {
    match calendar.gaps().iter().find(|gap| gap.name == name) {
        Some(gap) => gap,
        None => panic!("no gap named {name}: {:?}", calendar.gaps()),
    }
}

#[test]
fn a_rule_before_its_first_year_is_open_on_the_day_it_would_fall_on() {
    let calendar = HolidayCalendar::new(&PLAIN, None, 1995, 1995);
    let gap = gap_of(&calendar, "Founding Day");
    assert_eq!(gap.window, Some((ymd(1995, 9, 9), ymd(1995, 9, 9))));
    assert!(gap.could_fall_on(ymd(1995, 9, 9)));
    assert!(!gap.could_fall_on(ymd(1995, 9, 8)));
    assert_eq!(calendar.day_off(ymd(1995, 9, 9)), Err(Unanswered::Gap));
    assert_eq!(calendar.day_off(ymd(1995, 9, 8)), Ok(false));
    assert_eq!(calendar.day_off(ymd(1995, 9, 11)), Ok(false));
}

#[test]
fn a_table_with_a_substitution_widens_the_window_by_the_days_it_can_move_a_day() {
    let calendar = HolidayCalendar::new(&SUBSTITUTING, None, 1995, 1995);
    let gap = gap_of(&calendar, "Founding Day");
    assert_eq!(gap.window, Some((ymd(1995, 9, 2), ymd(1995, 9, 16))));
    assert_eq!(calendar.day_off(ymd(1995, 9, 12)), Err(Unanswered::Gap));
    assert_eq!(calendar.day_off(ymd(1995, 9, 18)), Ok(false));
}

#[test]
fn an_announcement_that_has_run_out_is_open_on_the_span_of_dates_it_took() {
    // The dates read fell between 5 and 9 March: in 2015 nothing says which
    // day the announcement would have given, and nothing opens a day outside
    // them.
    let calendar = HolidayCalendar::new(&PLAIN, None, 2015, 2015);
    let gap = gap_of(&calendar, "Spring Day");
    assert_eq!(gap.window, Some((ymd(2015, 3, 5), ymd(2015, 3, 9))));
    assert_eq!(calendar.day_off(ymd(2015, 3, 7)), Err(Unanswered::Gap));
    assert_eq!(calendar.day_off(ymd(2015, 3, 10)), Ok(false));
    assert_eq!(calendar.day_off(ymd(2015, 6, 1)), Ok(false));
    // The years listed answer.
    let listed = HolidayCalendar::new(&PLAIN, None, 2011, 2011);
    assert!(listed.gaps().iter().all(|gap| gap.name != "Spring Day"));
    assert_eq!(listed.day_off(ymd(2011, 3, 9)), Ok(true));
}

#[test]
fn a_rule_read_in_no_year_leaves_the_whole_year_open() {
    let calendar = HolidayCalendar::new(&PLAIN, None, 2030, 2030);
    let gap = gap_of(&calendar, "Lost Day");
    assert_eq!(gap.window, None);
    assert!(gap.could_fall_on(ymd(2030, 1, 2)));
    assert!(gap.could_fall_on(ymd(2030, 12, 30)));
    assert!(!gap.could_fall_on(ymd(2031, 1, 2)));
    assert_eq!(calendar.day_off(ymd(2030, 7, 2)), Err(Unanswered::Gap));
}

#[test]
fn the_next_holiday_is_refused_only_where_a_gap_lies_before_it() {
    let calendar = HolidayCalendar::new(&PLAIN, None, 1995, 1996);
    // Nothing is listed in the span, and the open days of both years lie in
    // it: refused from the start of 1995, and from 10 September, because
    // 9 September 1996 lies ahead.
    assert_eq!(
        calendar.try_next_of(ymd(1995, 1, 1), &[]),
        Err(Unanswered::Gap)
    );
    assert_eq!(
        calendar.try_next_of(ymd(1995, 9, 10), &[]),
        Err(Unanswered::Gap)
    );
    // A span that ends before the next open day says there is none: the
    // open days of 1995 lie behind 10 September.
    let short = HolidayCalendar::new(&PLAIN, None, 1995, 1995);
    assert_eq!(short.try_next_of(ymd(1995, 9, 10), &[]), Ok(None));
    // And back from 1 March 1995 the open 5 to 9 March lies ahead of nothing
    // that was found: the days before 5 March are asked.
    assert_eq!(short.try_previous_of(ymd(1995, 3, 4), &[]), Ok(None));
}

#[test]
fn a_table_that_states_no_weekend_has_no_weekend_gap_and_a_stated_one_has() {
    let none = RuleSet {
        weekend: NO_WEEKEND,
        ..PLAIN
    };
    assert!(!none.weekend_unread_in(None, 1500));
    assert_eq!(
        none.weekend_on(ymd(1500, 1, 1)),
        Some(&[Weekday::Saturday, Weekday::Sunday][..])
    );
}
