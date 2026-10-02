//! A church or a tradition answers for the years its sources reach and
//! reports the rest (ADR 0013, audit 10 a2).
//!
//! `roman-general` gave 215 entries for the year 1, 1500 and 1582, and 219
//! for 1700 to 2013, with `exact` confidence and no gap, though its calendar
//! is that of the Roman Missal of 2002 and the General Roman Calendar began
//! in 1970. `common-worship` gave 25 entries in the year 1, and the Sankranti
//! rules of `hindu` answered in the year -500 and in 3000. Each table below
//! is absent before the year its instrument was established and a gap
//! before the first year its sources were read for.

use hc_holiday::engine::HolidayCalendar;
use hc_holiday::hindu::{MAKAR_SANKRANTI, MESHA_SANKRANTI};
use hc_holiday::traditions;

/// A table, the year before which its subject did not exist (`None` where
/// no source read says), and the first year its sources were read for.
///
/// - `roman-general`: *Mysterii Paschalis*, 14 February 1969, "in effect
///   from 1 January 1970" (`mysterii-paschalis-1969`), and the *Missale
///   Romanum* of 2002, whose calendar the Liturgy Office's page copies.
/// - `common-worship`, `ember-common-worship`: Common Worship "launched on
///   the first Sunday of Advent in 2000", 3 December (Wikipedia, "Common
///   Worship"), so 2001 is the first whole year.
/// - `ember-bcp1662`: the Book of Common Prayer of 1662; 1753 the first
///   year wholly on the Gregorian calendar, England having changed in
///   September 1752 (`book_of_common_prayer`).
/// - `wheel-of-the-year`: Wikipedia, "Wheel of the Year": the phrase in use
///   "by the mid-1960s", the summer solstice and the equinoxes named by
///   Aidan Kelly in 1974.
/// - `tenrikyo`: Tenrikyo Online's report of the Oyasama Birth Celebration
///   Service of 18 April 2017, the earliest of the services it reports.
/// - `chaldean`: the bulletins of 2025; `syro-malabar`: the Church's
///   calendar of 2020-21 through Wikipedia.
/// - `mandaean`: Drower's dates of 1932 to 1936, which agree with the
///   calendar; the 1854 record she quotes parts from it by a day.
const TABLES: [(&str, Option<i32>, i32); 11] = [
    ("roman-general", Some(1970), 2002),
    ("common-worship", Some(2000), 2001),
    ("ember-common-worship", Some(2000), 2001),
    ("ember-bcp1662", Some(1662), 1753),
    ("wheel-of-the-year", None, 1974),
    ("wheel-of-the-year-south", None, 1974),
    ("tenrikyo", None, 2017),
    ("chaldean", None, 2025),
    ("syro-malabar", None, 2020),
    ("mandaean", None, 1932),
    ("church-of-the-east", Some(1965), 1965),
];

#[test]
fn a_table_is_a_gap_before_its_sources_and_absent_before_its_instrument() {
    for (code, established, read_from) in TABLES {
        let Some(table) = traditions::by_code(code) else {
            panic!("no table {code}");
        };
        let read_from = i64::from(read_from);
        // From the first year read it answers.
        let first = HolidayCalendar::for_year(table, None, read_from);
        assert!(!first.all().is_empty(), "{code} {read_from}");
        // The year before, and a year long before, answer nothing as a day.
        for year in [read_from - 1, read_from - 30, 1500] {
            let calendar = HolidayCalendar::for_year(table, None, year);
            assert!(
                calendar.all().is_empty(),
                "{code} {year}: {} days",
                calendar.all().len()
            );
            // A year the table's subject existed in is a gap; one before
            // it is not a year of any day.
            match established {
                Some(first) if year < i64::from(first) => assert!(
                    calendar.gaps().is_empty(),
                    "{code} {year}: a gap before the instrument"
                ),
                _ => assert!(!calendar.gaps().is_empty(), "{code} {year}: no gap"),
            }
        }
    }
}

#[test]
fn the_general_roman_calendar_began_in_1970_and_is_read_from_2002() {
    use hc_holiday::roman_calendar::GENERAL_ROMAN_CALENDAR;
    for (year, entries, gaps) in [(1, false, false), (1969, false, false), (1970, false, true)] {
        let calendar = HolidayCalendar::for_year(&GENERAL_ROMAN_CALENDAR, None, year);
        assert_eq!(!calendar.all().is_empty(), entries, "{year}");
        assert_eq!(!calendar.gaps().is_empty(), gaps, "{year}");
    }
    let calendar = HolidayCalendar::for_year(&GENERAL_ROMAN_CALENDAR, None, 2001);
    assert!(calendar.all().is_empty());
    // One gap for each celebration the calendar had in 2002.
    assert_eq!(calendar.gaps().len(), 219, "1970-2001 are 219 gaps a year");
    // The module's 230 celebrations of today (14 solemnities, the
    // Commemoration, 26 feasts, 69 memorials and 120 optional memorials)
    // less the 13 a decree has inscribed or re-ranked since 2002, plus the
    // two (Mary Magdalene's memorial, Martha's) that they replaced.
    let calendar = HolidayCalendar::for_year(&GENERAL_ROMAN_CALENDAR, None, 2002);
    assert_eq!(calendar.all().len(), 219);
    assert!(calendar.gaps().is_empty());
}

#[test]
fn common_worship_began_with_advent_2000() {
    use hc_holiday::common_worship::COMMON_WORSHIP;
    let before = HolidayCalendar::for_year(&COMMON_WORSHIP, None, 1999);
    assert!(before.all().is_empty() && before.gaps().is_empty());
    // 2000 holds the Advent that began it, and no whole year.
    let launched = HolidayCalendar::for_year(&COMMON_WORSHIP, None, 2000);
    assert!(launched.all().is_empty());
    assert_eq!(launched.gaps().len(), 40);
    let first = HolidayCalendar::for_year(&COMMON_WORSHIP, None, 2001);
    assert_eq!(first.all().len() + first.gaps().len(), 40);
}

#[test]
fn the_sankranti_rules_answer_inside_the_calendars_years_only() {
    for rule in [MAKAR_SANKRANTI, MESHA_SANKRANTI] {
        assert!(!rule.is_resolvable_in(-500));
        assert!(!rule.is_resolvable_in(1700));
        assert!(rule.is_resolvable_in(1701));
        assert!(rule.is_resolvable_in(2024));
        assert!(rule.is_resolvable_in(2298));
        assert!(!rule.is_resolvable_in(2299));
        assert!(!rule.is_resolvable_in(3000));
    }
    let calendar = HolidayCalendar::for_year(&traditions::HINDU, None, 3000);
    let gaps: Vec<&str> = calendar.gaps().iter().map(|gap| gap.name).collect();
    assert!(gaps.contains(&"Makar Sankranti"), "{gaps:?}");
    assert!(
        calendar
            .all()
            .iter()
            .all(|holiday| !holiday.name.contains("Sankranti")),
        "a Sankranti in 3000"
    );
}

#[test]
fn the_qumran_festivals_are_approximate_because_their_dates_are_a_convention() {
    // The weekday and the day of the festival are the scrolls', the
    // Gregorian date the library's epoch's (docs/systems/qumran.md).
    let calendar = HolidayCalendar::for_year(&traditions::QUMRAN, None, 1);
    assert_eq!(calendar.all().len(), 8);
    for holiday in calendar.all() {
        assert_eq!(holiday.confidence, hc_holiday::Confidence::Approximate);
    }
}
