//! Japan, amendment by amendment.
//!
//! Every assertion below is a date from a published Japanese calendar or from
//! the text of the 祝日法 and its amending acts, not a date this crate
//! produced. The point of the file is that the table in
//! `countries/japan.rs` is data: nothing here exercises a Japan-specific
//! code path, because there is not one.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::countries::JAPAN;
use hc_holiday::engine::{Holiday, HolidayCalendar};

/// Panics rather than returning a `Result`, because every date in this file
/// is a literal the author typed and a bad one is a bug in the test.
///
/// # Panics
///
/// When the year, month and day are not a Gregorian date.
fn ymd(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(fixed) => fixed,
        Err(error) => panic!("{year}-{month:02}-{day:02} is not a date: {error:?}"),
    }
}

fn year(year: i64) -> HolidayCalendar<'static> {
    HolidayCalendar::for_year(&JAPAN, None, year)
}

/// The name of the first day-off holiday on a date, or `""`.
fn name_on(y: i64, m: u8, d: u8) -> &'static str {
    year(y).name_on(ymd(y, m, d)).unwrap_or("")
}

fn is_holiday(y: i64, m: u8, d: u8) -> bool {
    year(y).is_holiday(ymd(y, m, d))
}

fn entries_on(y: i64, m: u8, d: u8) -> Vec<Holiday> {
    year(y).on(ymd(y, m, d))
}

// ── 1948: the law came into force on 20 July ─────────────────────────────

#[test]
fn the_1948_half_year_has_only_the_three_holidays_that_follow_20_july() {
    let calendar = year(1948);
    assert_eq!(calendar.all().len(), 3);
    assert!(is_holiday(1948, 9, 23));
    assert!(is_holiday(1948, 11, 3));
    assert!(is_holiday(1948, 11, 23));
    // 元日 and 天皇誕生日 fall before the commencement date.
    assert!(!is_holiday(1948, 1, 1));
    assert!(!is_holiday(1948, 4, 29));
}

#[test]
fn nineteen_forty_nine_is_the_first_full_year_of_the_original_nine() {
    let calendar = year(1949);
    assert_eq!(calendar.all().len(), 9);
    assert_eq!(name_on(1949, 1, 1), "New Year's Day");
    assert_eq!(name_on(1949, 1, 15), "Coming of Age Day");
    assert_eq!(name_on(1949, 4, 29), "The Emperor's Birthday");
    assert_eq!(name_on(1949, 5, 3), "Constitution Memorial Day");
    assert_eq!(name_on(1949, 5, 5), "Children's Day");
}

// ── 1966: 昭和41年法律第86号 ──────────────────────────────────────────────

#[test]
fn the_1966_amendment_added_respect_for_the_aged_and_sports_day() {
    assert!(!is_holiday(1965, 9, 15));
    assert!(!is_holiday(1965, 10, 10));
    assert_eq!(name_on(1966, 9, 15), "Respect for the Aged Day");
    assert_eq!(name_on(1966, 10, 10), "Health and Sports Day");
}

#[test]
fn national_foundation_day_waited_for_its_cabinet_order_and_began_in_1967() {
    // The amending act left the date to a 政令, which came on 1966-12-09.
    assert!(!is_holiday(1966, 2, 11));
    assert_eq!(name_on(1967, 2, 11), "National Foundation Day");
}

// ── 1973: 振替休日 ────────────────────────────────────────────────────────

#[test]
fn the_first_substitute_holiday_in_japanese_history_was_30_april_1973() {
    // 天皇誕生日 1973 fell on a Sunday; the amending act was promulgated on
    // 12 April 1973, eighteen days before.
    let substitute = entries_on(1973, 4, 30)
        .into_iter()
        .find(Holiday::is_substitute)
        .expect("a 振替休日");
    assert_eq!(substitute.observed_for, Some(ymd(1973, 4, 29)));
}

#[test]
fn the_substitution_law_did_not_reach_back_before_its_own_commencement() {
    // 11 February 1973 was a Sunday, but the act only came into force on
    // 12 April, so the Monday was an ordinary working day.
    assert!(is_holiday(1973, 2, 11));
    assert!(!is_holiday(1973, 2, 12));
}

#[test]
fn a_sunday_holiday_before_1973_produced_nothing() {
    // 文化の日 1968 fell on a Sunday; there was no substitute.
    assert!(is_holiday(1968, 11, 3));
    assert!(!is_holiday(1968, 11, 4));
}

// ── 1985: 国民の休日 ──────────────────────────────────────────────────────

#[test]
fn the_bridge_rule_was_not_in_force_in_1985() {
    // 3 May 1985 was a Friday and 4 May a Saturday; the amending act was
    // promulgated in December 1985 and applied from 1986.
    assert!(!is_holiday(1985, 5, 4));
}

#[test]
fn the_first_citizens_holiday_was_4_may_1988() {
    let bridge = entries_on(1988, 5, 4)
        .into_iter()
        .find(|holiday| holiday.bridged)
        .expect("a 国民の休日");
    assert_eq!(bridge.name, "Citizens' Holiday");
    assert_eq!(bridge.local_name, "国民の休日");
    // 1986 and 1987 both missed: 4 May was a Sunday, then a 振替休日.
    assert!(!is_holiday(1986, 5, 4));
}

#[test]
fn the_bridge_rule_skips_a_sunday_but_not_a_saturday() {
    // 4 May 1996 was a Saturday and was a 国民の休日; 4 May 1986 was a
    // Sunday and was not.
    assert_eq!(name_on(1996, 5, 4), "Citizens' Holiday");
    assert!(!is_holiday(1986, 5, 4));
}

#[test]
fn silver_week_appears_when_the_equinox_falls_two_days_after_the_third_monday() {
    // 2015 and 2026 both put 敬老の日 on the 21st and 秋分の日 on the 23rd.
    assert_eq!(name_on(2015, 9, 22), "Citizens' Holiday");
    assert_eq!(name_on(2026, 9, 22), "Citizens' Holiday");
    // 2016 does not: the equinox was the 22nd and 敬老の日 the 19th.
    assert!(!is_holiday(2016, 9, 20));
}

// ── 1989: the accession of Emperor Akihito ───────────────────────────────

#[test]
fn the_emperors_birthday_moved_from_april_to_december_in_1989() {
    assert_eq!(name_on(1988, 4, 29), "The Emperor's Birthday");
    assert!(!is_holiday(1988, 12, 23));
    assert_eq!(name_on(1989, 4, 29), "Greenery Day");
    assert_eq!(name_on(1989, 12, 23), "The Emperor's Birthday");
}

#[test]
fn the_state_funeral_of_emperor_showa_was_a_one_off_holiday() {
    assert_eq!(name_on(1989, 2, 24), "State Funeral of Emperor Shōwa");
    assert!(!is_holiday(1990, 2, 24));
}

#[test]
fn the_imperial_one_offs_each_occupy_exactly_one_year() {
    assert_eq!(name_on(1959, 4, 10), "Wedding of Crown Prince Akihito");
    assert!(!is_holiday(1960, 4, 10));
    assert_eq!(
        name_on(1990, 11, 12),
        "Enthronement Ceremony of Emperor Akihito"
    );
    assert!(!is_holiday(1991, 11, 12));
    assert_eq!(name_on(1993, 6, 9), "Wedding of Crown Prince Naruhito");
    assert!(!is_holiday(1994, 6, 9));
}

// ── 1996: 海の日 ─────────────────────────────────────────────────────────

#[test]
fn marine_day_began_in_1996_on_the_twentieth_of_july() {
    assert!(!is_holiday(1995, 7, 20));
    assert_eq!(name_on(1996, 7, 20), "Marine Day");
    assert_eq!(name_on(2002, 7, 20), "Marine Day");
}

// ── 2000 and 2003: ハッピーマンデー ───────────────────────────────────────

#[test]
fn coming_of_age_day_moved_to_the_second_monday_of_january_in_2000() {
    assert_eq!(name_on(1999, 1, 15), "Coming of Age Day");
    assert!(!is_holiday(2000, 1, 15));
    assert_eq!(name_on(2000, 1, 10), "Coming of Age Day");
}

#[test]
fn sports_day_moved_to_the_second_monday_of_october_in_2000() {
    assert_eq!(name_on(1999, 10, 10), "Health and Sports Day");
    assert_eq!(name_on(2000, 10, 9), "Health and Sports Day");
    assert!(!is_holiday(2000, 10, 10));
}

#[test]
fn marine_day_and_respect_for_the_aged_moved_in_2003() {
    assert_eq!(name_on(2002, 7, 20), "Marine Day");
    assert_eq!(name_on(2003, 7, 21), "Marine Day");
    assert!(!is_holiday(2003, 7, 20));
    assert_eq!(name_on(2002, 9, 15), "Respect for the Aged Day");
    // In 2003 the third Monday happened to be the 15th again.
    assert_eq!(name_on(2003, 9, 15), "Respect for the Aged Day");
    assert_eq!(name_on(2004, 9, 20), "Respect for the Aged Day");
    assert!(!is_holiday(2004, 9, 15));
}

// ── 2007: 昭和の日, みどりの日 and the new 振替休日 ──────────────────────

#[test]
fn greenery_day_moved_to_4_may_and_29_april_became_showa_day_in_2007() {
    assert_eq!(name_on(2006, 4, 29), "Greenery Day");
    // 4 May 2006 was a Thursday and therefore a 国民の休日, not みどりの日.
    assert_eq!(name_on(2006, 5, 4), "Citizens' Holiday");
    assert_eq!(name_on(2007, 4, 29), "Shōwa Day");
    assert_eq!(name_on(2007, 5, 4), "Greenery Day");
}

#[test]
fn the_2007_substitution_rule_walks_past_a_day_that_is_already_a_holiday() {
    // 3 May 2015 was a Sunday. 4 May is みどりの日 and 5 May こどもの日, so
    // the 振替休日 lands on the 6th — which the pre-2007 wording could not
    // have produced.
    let substitute = entries_on(2015, 5, 6)
        .into_iter()
        .find(Holiday::is_substitute)
        .expect("a 振替休日");
    assert_eq!(substitute.observed_for, Some(ymd(2015, 5, 3)));
    // The same shape in 2009, the first year it happened.
    assert!(is_holiday(2009, 5, 6));
}

// ── 2016: 山の日 ─────────────────────────────────────────────────────────

#[test]
fn mountain_day_began_in_2016() {
    assert!(!is_holiday(2015, 8, 11));
    assert_eq!(name_on(2016, 8, 11), "Mountain Day");
    assert_eq!(name_on(2024, 8, 11), "Mountain Day");
}

// ── 2019: the abdication and the accession ───────────────────────────────

#[test]
fn twenty_nineteen_had_no_emperors_birthday_at_all() {
    assert_eq!(name_on(2018, 12, 23), "The Emperor's Birthday");
    assert!(!is_holiday(2019, 12, 23));
    assert!(!is_holiday(2019, 2, 23));
    assert_eq!(name_on(2020, 2, 23), "The Emperor's Birthday");
}

#[test]
fn the_2019_accession_produced_a_ten_day_golden_week() {
    assert_eq!(name_on(2019, 4, 29), "Shōwa Day");
    assert_eq!(name_on(2019, 4, 30), "Citizens' Holiday");
    assert_eq!(name_on(2019, 5, 1), "Accession Day");
    assert_eq!(name_on(2019, 5, 2), "Citizens' Holiday");
    assert_eq!(name_on(2019, 5, 3), "Constitution Memorial Day");
    assert_eq!(name_on(2019, 5, 4), "Greenery Day");
    assert_eq!(name_on(2019, 5, 5), "Children's Day");
    assert!(is_holiday(2019, 5, 6));
    // Twenty-seven April to six May inclusive, weekends included.
    for day in 27..=30 {
        assert!(
            year(2019).is_holiday(ymd(2019, 4, day))
                || !year(2019).is_business_day(ymd(2019, 4, day)),
            "2019-04-{day} should not be a working day"
        );
    }
}

#[test]
fn the_enthronement_ceremony_of_2019_was_a_single_holiday() {
    assert_eq!(
        name_on(2019, 10, 22),
        "Enthronement Ceremony of Emperor Naruhito"
    );
    assert!(!is_holiday(2020, 10, 22));
}

// ── 2020 and 2021: the Tokyo Olympics ────────────────────────────────────

#[test]
fn the_2020_games_moved_three_holidays_and_renamed_a_fourth() {
    assert_eq!(name_on(2020, 7, 23), "Marine Day");
    assert_eq!(name_on(2020, 7, 24), "Sports Day");
    assert_eq!(name_on(2020, 8, 10), "Mountain Day");
    // The days they would otherwise have fallen on were ordinary.
    assert!(!is_holiday(2020, 7, 20));
    assert!(!is_holiday(2020, 8, 11));
    assert!(!is_holiday(2020, 10, 12));
}

#[test]
fn the_postponed_2021_games_moved_them_again() {
    assert_eq!(name_on(2021, 7, 22), "Marine Day");
    assert_eq!(name_on(2021, 7, 23), "Sports Day");
    assert_eq!(name_on(2021, 8, 8), "Mountain Day");
    // 8 August 2021 was a Sunday, so the Monday was a 振替休日.
    let substitute = entries_on(2021, 8, 9)
        .into_iter()
        .find(Holiday::is_substitute)
        .expect("a 振替休日");
    assert_eq!(substitute.observed_for, Some(ymd(2021, 8, 8)));
    assert!(!is_holiday(2021, 7, 19));
    assert!(!is_holiday(2021, 10, 11));
}

#[test]
fn sports_day_returned_to_the_second_monday_of_october_in_2022() {
    assert_eq!(name_on(2022, 10, 10), "Sports Day");
    assert_eq!(name_on(2024, 10, 14), "Sports Day");
    assert_eq!(name_on(2022, 7, 18), "Marine Day");
    assert_eq!(name_on(2022, 8, 11), "Mountain Day");
}

// ── The equinoxes, computed rather than tabulated ────────────────────────

#[test]
fn the_equinox_holidays_match_the_dates_japan_published() {
    // 国立天文台's 暦要項 as printed in the 官報, one year ahead, for each of
    // these years. None of these numbers came from this crate.
    let published_spring = [
        (1980, 20),
        (1988, 20),
        (1992, 20),
        (1999, 21),
        (2000, 20),
        (2012, 20),
        (2016, 20),
        (2020, 20),
        (2021, 20),
        (2024, 20),
        (2025, 20),
        (2026, 20),
    ];
    for (y, day) in published_spring {
        assert_eq!(
            name_on(y, 3, day),
            "Vernal Equinox Day",
            "春分の日 {y} should be 3/{day}"
        );
    }
    let published_autumn = [
        (1979, 24),
        (1996, 23),
        (2012, 22),
        (2016, 22),
        (2020, 22),
        (2023, 23),
        (2024, 22),
        (2025, 23),
        (2026, 23),
    ];
    for (y, day) in published_autumn {
        assert_eq!(
            name_on(y, 9, day),
            "Autumnal Equinox Day",
            "秋分の日 {y} should be 9/{day}"
        );
    }
}

#[test]
fn the_autumn_equinox_of_2012_was_the_first_on_22_september_since_1896() {
    assert!(is_holiday(2012, 9, 22));
    assert!(!is_holiday(2012, 9, 23));
    assert!(is_holiday(2011, 9, 23));
}

// ── Whole-year shapes ────────────────────────────────────────────────────

#[test]
fn a_recent_year_matches_the_cabinet_offices_published_list() {
    // 内閣府「国民の祝日について」for 2024, plus the two 振替休日 and the
    // dates they substituted for.
    let calendar = year(2024);
    let expected = [
        (1, 1, "New Year's Day"),
        (1, 8, "Coming of Age Day"),
        (2, 11, "National Foundation Day"),
        (2, 12, "National Foundation Day"),
        (2, 23, "The Emperor's Birthday"),
        (3, 20, "Vernal Equinox Day"),
        (4, 29, "Shōwa Day"),
        (5, 3, "Constitution Memorial Day"),
        (5, 4, "Greenery Day"),
        (5, 5, "Children's Day"),
        (5, 6, "Children's Day"),
        (7, 15, "Marine Day"),
        (8, 11, "Mountain Day"),
        (8, 12, "Mountain Day"),
        (9, 16, "Respect for the Aged Day"),
        (9, 22, "Autumnal Equinox Day"),
        (9, 23, "Autumnal Equinox Day"),
        (10, 14, "Sports Day"),
        (11, 3, "Culture Day"),
        (11, 4, "Culture Day"),
        (11, 23, "Labour Thanksgiving Day"),
    ];
    assert_eq!(calendar.all().len(), expected.len());
    for (month, day, name) in expected {
        assert_eq!(name_on(2024, month, day), name, "2024-{month}-{day}");
    }
}

#[test]
fn the_number_of_holidays_grew_with_each_amendment() {
    let counts = [(1950, 9), (1965, 9), (1967, 12), (1997, 14), (2017, 16)];
    for (y, expected) in counts {
        let base = year(y)
            .all()
            .iter()
            .filter(|holiday| !holiday.is_substitute() && !holiday.bridged)
            .count();
        assert_eq!(base, expected, "base holidays in {y}");
    }
}

#[test]
fn next_and_previous_holiday_walk_the_japanese_year() {
    let calendar = HolidayCalendar::new(&JAPAN, None, 2024, 2024);
    let next = calendar
        .next_holiday(ymd(2024, 6, 1))
        .expect("a holiday after June");
    assert_eq!(next.date, ymd(2024, 7, 15));
    let previous = calendar
        .previous_holiday(ymd(2024, 6, 1))
        .expect("a holiday before June");
    assert_eq!(previous.date, ymd(2024, 5, 6));
}

#[test]
fn golden_week_2024_is_four_business_days_shorter_than_the_span() {
    let calendar = HolidayCalendar::new(&JAPAN, None, 2024, 2024);
    // 26 April (Friday) to 7 May (Tuesday): eleven days, of which only
    // 30 April, 1 May and 2 May are working days.
    assert_eq!(
        calendar.business_days_between(ymd(2024, 4, 26), ymd(2024, 5, 7)),
        Some(4)
    );
    assert_eq!(
        calendar.add_business_days(ymd(2024, 4, 26), 1),
        Some(ymd(2024, 4, 30))
    );
}

#[test]
fn every_japanese_holiday_is_exact_because_nothing_in_the_table_is_a_guess() {
    use hc_holiday::rule::Confidence;
    for y in 1948..=2050 {
        for holiday in year(y).all() {
            assert_eq!(
                holiday.confidence,
                Confidence::Exact,
                "{y} {}",
                holiday.name
            );
        }
    }
}

#[test]
fn no_two_japanese_holidays_share_a_day_under_a_single_name() {
    for y in 1948..=2050 {
        let calendar = year(y);
        let mut seen: Vec<(i64, &str)> = calendar
            .all()
            .iter()
            .map(|holiday| (holiday.date.0, holiday.name))
            .collect();
        let before = seen.len();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(seen.len(), before, "duplicate entries in {y}");
    }
}

#[test]
fn the_japanese_table_cites_its_statute_and_its_check_date() {
    assert!(JAPAN.sources.contains("昭和23年法律第178号"));
    assert_eq!(JAPAN.sources_checked.year, 2026);
}
