//! Every country table, five dates across at least two years, including the
//! case that exercises its weekend rule.
//!
//! Where a country has no substitution law the test asserts the *absence* of
//! a shift, because "this holiday stays on the Saturday" is as much a claim
//! as "this holiday moves to the Monday", and the crate makes it deliberately.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::countries::{self, CountryRules};
use hc_holiday::engine::{Holiday, HolidayCalendar};
use hc_holiday::rule::{Confidence, Kind};

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

/// # Panics
///
/// When `code` is not a country this crate carries.
fn table(code: &str) -> &'static CountryRules {
    match countries::by_code(code) {
        Some(country) => country,
        None => panic!("{code} is not a registered country"),
    }
}

/// Assert that each `(year, month, day, name)` is a day off with that name.
fn expect(code: &str, region: Option<&str>, days: &[(i64, u8, u8, &str)]) {
    let country = table(code);
    for (year, month, day, name) in days {
        let calendar = HolidayCalendar::for_year(country, region, *year);
        let date = ymd(*year, *month, *day);
        let found: Vec<&'static str> = calendar
            .on(date)
            .iter()
            .filter(|holiday| holiday.is_day_off())
            .map(|holiday| holiday.name)
            .collect();
        assert!(
            found.contains(name),
            "{code} {year}-{month:02}-{day:02}: expected {name}, found {found:?}"
        );
    }
}

/// Assert that no day-off holiday falls on each date.
fn expect_working(code: &str, region: Option<&str>, days: &[(i64, u8, u8)]) {
    let country = table(code);
    for (year, month, day) in days {
        let calendar = HolidayCalendar::for_year(country, region, *year);
        assert!(
            !calendar.is_holiday(ymd(*year, *month, *day)),
            "{code} {year}-{month:02}-{day:02} should not be a holiday"
        );
    }
}

/// Assert that `observed` is a substitute standing in for `original`.
fn expect_substitute(
    code: &str,
    region: Option<&str>,
    year: i64,
    original: (u8, u8),
    observed: (u8, u8),
) {
    let calendar = HolidayCalendar::for_year(table(code), region, year);
    let target = ymd(year, observed.0, observed.1);
    let substitute = calendar
        .on(target)
        .into_iter()
        .find(Holiday::is_substitute)
        .unwrap_or_else(|| panic!("{code} {year}: no substitute on {observed:?}"));
    assert_eq!(
        substitute.observed_for,
        Some(ymd(year, original.0, original.1)),
        "{code} {year}: substitute on {observed:?} stands in for the wrong day"
    );
}

// ─────────────────────────────────────────────────────────────────────────
// The Americas
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn united_states_federal_holidays() {
    expect(
        "US",
        None,
        &[
            (2024, 1, 15, "Birthday of Martin Luther King, Jr."),
            (2024, 5, 27, "Memorial Day"),
            (2024, 6, 19, "Juneteenth National Independence Day"),
            (2024, 11, 28, "Thanksgiving Day"),
            (2025, 2, 17, "Washington's Birthday"),
            (2025, 10, 13, "Columbus Day"),
            (2025, 11, 11, "Veterans Day"),
        ],
    );
    // Juneteenth did not exist before 2021, and Columbus Day before 1971.
    expect_working("US", None, &[(2020, 6, 19), (1970, 10, 12)]);
}

#[test]
fn the_united_states_observes_a_saturday_holiday_on_the_friday_before() {
    // 4 July 2020 was a Saturday, observed Friday 3 July; 25 December 2022
    // was a Sunday, observed Monday 26 December.
    expect_substitute("US", None, 2020, (7, 4), (7, 3));
    expect_substitute("US", None, 2022, (12, 25), (12, 26));
    expect_substitute("US", None, 2021, (12, 25), (12, 24));
}

#[test]
fn the_uniform_monday_holiday_act_moved_three_holidays_from_1971() {
    expect(
        "US",
        None,
        &[
            (1970, 2, 22, "Washington's Birthday"),
            (1970, 5, 30, "Memorial Day"),
            (1971, 2, 15, "Washington's Birthday"),
            (1971, 5, 31, "Memorial Day"),
            (1975, 10, 27, "Veterans Day"),
            (1978, 11, 11, "Veterans Day"),
        ],
    );
}

#[test]
fn inauguration_day_is_a_holiday_only_in_the_capital_and_only_every_fourth_year() {
    expect("US", Some("US-DC"), &[(2025, 1, 20, "Inauguration Day")]);
    expect_working("US", Some("US-DC"), &[(2024, 1, 20), (2026, 1, 20)]);
    // It is not a national holiday — though in 2025 the same Monday was
    // also the Birthday of Martin Luther King, Jr., so the day is not free.
    let national = HolidayCalendar::for_year(table("US"), None, 2025);
    assert!(
        !national
            .on(ymd(2025, 1, 20))
            .iter()
            .any(|holiday| holiday.name == "Inauguration Day")
    );
    // 20 January 2013 was a Sunday, so the public holiday was the 21st.
    expect("US", Some("US-DC"), &[(2013, 1, 21, "Inauguration Day")]);
}

#[test]
fn canada_federal_and_provincial_holidays() {
    expect(
        "CA",
        Some("CA-ON"),
        &[
            (2024, 2, 19, "Family Day"),
            (2024, 5, 20, "Victoria Day"),
            (2024, 9, 30, "National Day for Truth and Reconciliation"),
            (2024, 10, 14, "Thanksgiving"),
            (2025, 2, 17, "Family Day"),
            (2025, 5, 19, "Victoria Day"),
        ],
    );
    expect(
        "CA",
        Some("CA-QC"),
        &[(2025, 6, 24, "Saint-Jean-Baptiste Day")],
    );
    // Quebec has no Family Day; Ontario has no Saint-Jean-Baptiste Day.
    expect_working("CA", Some("CA-QC"), &[(2025, 2, 17)]);
    expect_working("CA", Some("CA-ON"), &[(2025, 6, 24)]);
}

#[test]
fn canada_moves_a_weekend_holiday_to_the_following_monday() {
    // 1 July 2023 was a Saturday; Canada Day was observed on Monday 3 July.
    expect_substitute("CA", None, 2023, (7, 1), (7, 3));
    // 25 and 26 December 2021 fell on Saturday and Sunday.
    expect_substitute("CA", None, 2021, (12, 25), (12, 27));
    expect_substitute("CA", None, 2021, (12, 26), (12, 28));
}

#[test]
fn mexico_moved_three_holidays_onto_mondays_in_2006() {
    expect(
        "MX",
        None,
        &[
            (2005, 2, 5, "Constitution Day"),
            (2005, 3, 21, "Benito Juárez's Birthday"),
            (2024, 2, 5, "Constitution Day"),
            (2024, 3, 18, "Benito Juárez's Birthday"),
            (2025, 2, 3, "Constitution Day"),
            (2025, 11, 17, "Revolution Day"),
        ],
    );
    expect_working("MX", None, &[(2025, 3, 21), (2025, 11, 20)]);
}

#[test]
fn the_mexican_presidential_handover_is_a_holiday_once_every_six_years() {
    expect(
        "MX",
        None,
        &[
            (2018, 12, 1, "Presidential Inauguration"),
            (2024, 10, 1, "Presidential Inauguration"),
        ],
    );
    expect_working("MX", None, &[(2024, 12, 1), (2025, 10, 1), (2019, 12, 1)]);
}

#[test]
fn mexico_leaves_a_weekend_holiday_where_it_falls() {
    // Mexico's Ley Federal del Trabajo has no observed-day rule.
    expect("MX", None, &[(2021, 5, 1, "Labour Day")]);
    expect_working("MX", None, &[(2021, 5, 3), (2021, 4, 30)]);
}

#[test]
fn brazil_national_holidays() {
    expect(
        "BR",
        None,
        &[
            (2024, 4, 21, "Tiradentes"),
            (2024, 9, 7, "Independence Day"),
            (2024, 11, 2, "All Souls' Day"),
            (2024, 11, 20, "Black Awareness Day"),
            (2025, 4, 18, "Good Friday"),
            (2025, 10, 12, "Our Lady of Aparecida"),
        ],
    );
    // Black Awareness Day became national only in 2024.
    expect_working("BR", None, &[(2023, 11, 20)]);
    // Brazil has no substitution: 21 April 2024 was a Sunday and stayed one.
    expect_working("BR", None, &[(2024, 4, 22)]);
}

// ─────────────────────────────────────────────────────────────────────────
// Europe
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn united_kingdom_england_and_wales() {
    expect(
        "GB",
        Some("GB-EAW"),
        &[
            (2024, 3, 29, "Good Friday"),
            (2024, 4, 1, "Easter Monday"),
            (2024, 5, 6, "Early May Bank Holiday"),
            (2024, 8, 26, "Summer Bank Holiday"),
            (2025, 5, 26, "Spring Bank Holiday"),
            (2025, 12, 26, "Boxing Day"),
        ],
    );
    // Scotland's summer bank holiday is the first Monday of August, and it
    // has no Easter Monday.
    expect("GB", Some("GB-SCT"), &[(2024, 8, 5, "Summer Bank Holiday")]);
    expect_working("GB", Some("GB-SCT"), &[(2024, 4, 1), (2024, 8, 26)]);
}

#[test]
fn the_british_christmas_pair_pushes_two_substitutes_apart() {
    // 25 December 2021 was a Saturday and 26 December a Sunday, so the two
    // bank holidays were kept on Monday 27th and Tuesday 28th.
    expect_substitute("GB", Some("GB-EAW"), 2021, (12, 25), (12, 27));
    expect_substitute("GB", Some("GB-EAW"), 2021, (12, 26), (12, 28));
    // In 2022 Christmas was the Sunday and Boxing Day the Monday, so only
    // one substitute was needed.
    expect_substitute("GB", Some("GB-EAW"), 2022, (12, 25), (12, 27));
}

#[test]
fn the_british_royal_one_offs_and_the_jubilee_moves() {
    expect(
        "GB",
        Some("GB-EAW"),
        &[
            (2002, 6, 3, "Golden Jubilee of Elizabeth II"),
            (2002, 6, 4, "Spring Bank Holiday"),
            (2012, 6, 5, "Diamond Jubilee of Elizabeth II"),
            (2022, 6, 2, "Spring Bank Holiday"),
            (2022, 6, 3, "Platinum Jubilee of Elizabeth II"),
            (2022, 9, 19, "State Funeral of Elizabeth II"),
            (2023, 5, 8, "Coronation of Charles III"),
        ],
    );
    expect_working("GB", Some("GB-EAW"), &[(2022, 5, 30), (2012, 5, 28)]);
}

#[test]
fn northern_ireland_and_scotland_have_their_own_days() {
    expect(
        "GB",
        Some("GB-NIR"),
        &[
            (2024, 3, 17, "St Patrick's Day"),
            (2024, 7, 12, "Battle of the Boyne"),
        ],
    );
    expect("GB", Some("GB-SCT"), &[(2024, 1, 2, "2 January")]);
    expect_working(
        "GB",
        Some("GB-EAW"),
        &[(2024, 3, 17), (2024, 7, 12), (2024, 1, 2)],
    );
    // St Andrew's Day 2024 fell on a Saturday and was kept on the Monday.
    expect_substitute("GB", Some("GB-SCT"), 2024, (11, 30), (12, 2));
}

#[test]
fn ireland_national_holidays_and_st_brigids_conditional_rule() {
    expect(
        "IE",
        None,
        &[
            (2024, 2, 5, "St Brigid's Day"),
            (2024, 6, 3, "June Bank Holiday"),
            (2024, 10, 28, "October Bank Holiday"),
            (2025, 2, 3, "St Brigid's Day"),
            (2025, 3, 17, "St Patrick's Day"),
            (2025, 12, 26, "St Stephen's Day"),
        ],
    );
    // 2026: 1 February is a Sunday, so the holiday is the first Monday.
    expect("IE", None, &[(2026, 2, 2, "St Brigid's Day")]);
    // St Patrick's Day 2024 fell on a Sunday and was kept on the Monday.
    expect_substitute("IE", None, 2024, (3, 17), (3, 18));
    expect_working("IE", None, &[(2022, 2, 7)]);
}

#[test]
fn france_metropolitan_and_alsace_moselle() {
    expect(
        "FR",
        None,
        &[
            (2024, 5, 8, "Victory in Europe Day"),
            (2024, 5, 9, "Ascension"),
            (2024, 7, 14, "Bastille Day"),
            (2025, 6, 9, "Whit Monday"),
            (2025, 8, 15, "Assumption"),
            (2025, 11, 11, "Armistice Day"),
        ],
    );
    expect(
        "FR",
        Some("FR-57"),
        &[
            (2025, 4, 18, "Good Friday"),
            (2025, 12, 26, "St Stephen's Day"),
        ],
    );
    expect_working("FR", None, &[(2025, 4, 18), (2025, 12, 26)]);
    // 8 May was not a holiday between 1959 and 1981.
    expect_working("FR", None, &[(1970, 5, 8)]);
    // France has no substitution: 14 July 2024 was a Sunday and stayed one.
    expect_working("FR", None, &[(2024, 7, 15)]);
}

#[test]
fn germany_federal_and_laender() {
    expect(
        "DE",
        None,
        &[
            (2024, 5, 9, "Ascension"),
            (2024, 10, 3, "German Unity Day"),
            (2025, 4, 18, "Good Friday"),
            (2025, 6, 9, "Whit Monday"),
            (2025, 12, 26, "St Stephen's Day"),
        ],
    );
    expect(
        "DE",
        Some("DE-BY"),
        &[
            (2024, 1, 6, "Epiphany"),
            (2024, 5, 30, "Corpus Christi"),
            (2025, 11, 1, "All Saints' Day"),
        ],
    );
    expect(
        "DE",
        Some("DE-SN"),
        &[(2024, 11, 20, "Day of Prayer and Repentance")],
    );
    expect(
        "DE",
        Some("DE-BE"),
        &[(2025, 3, 8, "International Women's Day")],
    );
    expect_working("DE", None, &[(2024, 1, 6), (2024, 11, 20), (2025, 3, 8)]);
    // Buß- und Bettag was federal until 1994.
    expect(
        "DE",
        None,
        &[(1994, 11, 16, "Day of Prayer and Repentance")],
    );
}

#[test]
fn italy_and_spain() {
    expect(
        "IT",
        None,
        &[
            (2024, 1, 6, "Epiphany"),
            (2024, 4, 25, "Liberation Day"),
            (2024, 6, 2, "Republic Day"),
            (2025, 4, 21, "Easter Monday"),
            (2025, 12, 26, "St Stephen's Day"),
        ],
    );
    // Republic Day was not a day off between 1977 and 2000.
    expect_working("IT", None, &[(1990, 6, 2)]);
    // No substitution: 2 June 2024 was a Sunday and stayed one.
    expect_working("IT", None, &[(2024, 6, 3)]);
    expect(
        "ES",
        None,
        &[
            (2024, 1, 6, "Epiphany"),
            (2024, 10, 12, "National Day"),
            (2024, 12, 6, "Constitution Day"),
            (2025, 4, 18, "Good Friday"),
            (2025, 8, 15, "Assumption"),
        ],
    );
    expect_working("ES", None, &[(2024, 10, 14), (1980, 12, 6)]);
}

#[test]
fn portugal_suspended_four_holidays_from_2013_to_2015() {
    expect(
        "PT",
        None,
        &[
            (2012, 10, 5, "Republic Day"),
            (2012, 11, 1, "All Saints' Day"),
            (2016, 10, 5, "Republic Day"),
            (2016, 12, 1, "Restoration of Independence"),
            (2024, 6, 10, "Portugal Day"),
            (2025, 4, 25, "Freedom Day"),
        ],
    );
    expect_working("PT", None, &[(2014, 10, 5), (2014, 11, 1), (2014, 12, 1)]);
}

#[test]
fn the_low_countries_and_the_alps() {
    expect(
        "NL",
        None,
        &[
            (2024, 4, 27, "King's Day"),
            (2024, 5, 9, "Ascension"),
            (2025, 4, 26, "King's Day"),
            (2025, 6, 9, "Whit Monday"),
            (2025, 12, 26, "Boxing Day"),
        ],
    );
    // The Dutch royal day moves backwards, not forwards: 27 April 2025 was a
    // Sunday.
    expect_working("NL", None, &[(2025, 4, 27), (2025, 4, 28)]);
    expect(
        "BE",
        None,
        &[
            (2024, 7, 21, "National Day"),
            (2024, 11, 11, "Armistice Day"),
            (2025, 5, 29, "Ascension"),
            (2025, 8, 15, "Assumption"),
            (2025, 11, 1, "All Saints' Day"),
        ],
    );
    expect_working("BE", None, &[(2024, 7, 22)]);
    expect(
        "CH",
        None,
        &[
            (2024, 8, 1, "Swiss National Day"),
            (2024, 5, 20, "Whit Monday"),
            (2025, 4, 18, "Good Friday"),
            (2025, 5, 29, "Ascension"),
            (2025, 12, 26, "St Stephen's Day"),
        ],
    );
    expect_working("CH", None, &[(1990, 8, 1)]);
    expect(
        "AT",
        None,
        &[
            (2024, 5, 30, "Corpus Christi"),
            (2024, 10, 26, "National Day"),
            (2024, 12, 8, "Immaculate Conception"),
            (2025, 1, 6, "Epiphany"),
            (2025, 8, 15, "Assumption"),
        ],
    );
    expect_working("AT", None, &[(1960, 10, 26), (2024, 12, 9)]);
}

#[test]
fn the_nordic_countries() {
    expect(
        "SE",
        None,
        &[
            (2024, 6, 6, "National Day"),
            (2024, 6, 22, "Midsummer Day"),
            (2024, 11, 2, "All Saints' Day"),
            (2025, 6, 21, "Midsummer Day"),
            (2025, 11, 1, "All Saints' Day"),
        ],
    );
    // Whit Monday was traded for the National Day in 2005.
    expect("SE", None, &[(2004, 5, 31, "Whit Monday")]);
    expect_working("SE", None, &[(2024, 5, 20), (2004, 6, 6)]);
    expect(
        "NO",
        None,
        &[
            (2024, 3, 28, "Maundy Thursday"),
            (2024, 5, 17, "Constitution Day"),
            (2025, 4, 17, "Maundy Thursday"),
            (2025, 5, 17, "Constitution Day"),
            (2025, 6, 9, "Whit Monday"),
        ],
    );
    // 17 May 2025 was a Saturday and Norway does not move it.
    expect_working("NO", None, &[(2025, 5, 19)]);
    expect(
        "DK",
        None,
        &[
            (2023, 5, 5, "Great Prayer Day"),
            (2024, 3, 28, "Maundy Thursday"),
            (2024, 5, 9, "Ascension"),
            (2025, 4, 21, "Easter Monday"),
            (2025, 6, 9, "Whit Monday"),
        ],
    );
    // Store bededag was abolished with effect from 2024.
    expect_working("DK", None, &[(2024, 4, 26), (2025, 5, 16)]);
    expect(
        "FI",
        None,
        &[
            (2024, 1, 6, "Epiphany"),
            (2024, 6, 22, "Midsummer Day"),
            (2024, 12, 6, "Independence Day"),
            (2025, 6, 21, "Midsummer Day"),
            (2025, 11, 1, "All Saints' Day"),
        ],
    );
    expect_working("FI", None, &[(2025, 12, 8)]);
}

#[test]
fn central_and_eastern_europe() {
    expect(
        "PL",
        None,
        &[
            (2024, 1, 6, "Epiphany"),
            (2024, 5, 3, "Constitution Day"),
            (2024, 5, 30, "Corpus Christi"),
            (2025, 11, 11, "Independence Day"),
            (2025, 12, 24, "Christmas Eve"),
        ],
    );
    // Epiphany returned in 2011, Christmas Eve in 2025, and Poland has no
    // observed-day rule: 3 May 2025 was a Saturday and stayed one.
    expect_working("PL", None, &[(2010, 1, 6), (2024, 12, 24), (2025, 5, 5)]);
    expect(
        "CZ",
        None,
        &[
            (2024, 5, 8, "Victory Day"),
            (2024, 7, 5, "Sts Cyril and Methodius"),
            (2024, 9, 28, "Statehood Day"),
            (2025, 10, 28, "Independence Day"),
            (2025, 12, 24, "Christmas Eve"),
        ],
    );
    // Good Friday became a holiday only in 2016.
    expect("CZ", None, &[(2016, 3, 25, "Good Friday")]);
    expect_working("CZ", None, &[(2015, 4, 3), (2025, 9, 29)]);
    expect(
        "GR",
        None,
        &[
            (2024, 3, 18, "Clean Monday"),
            (2024, 5, 3, "Good Friday"),
            (2024, 5, 6, "Easter Monday"),
            (2025, 3, 3, "Clean Monday"),
            (2025, 4, 21, "Easter Monday"),
            (2025, 10, 28, "Ochi Day"),
        ],
    );
    // Greece computes Easter by the Julian computus: in 2024 it was five
    // weeks after the Western one.
    expect_working("GR", None, &[(2024, 4, 1)]);
}

// ─────────────────────────────────────────────────────────────────────────
// Asia
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn china_statutory_holidays() {
    expect(
        "CN",
        None,
        &[
            (2024, 2, 10, "Spring Festival"),
            (2024, 4, 4, "Qingming Festival"),
            (2024, 6, 10, "Dragon Boat Festival"),
            (2024, 9, 17, "Mid-Autumn Festival"),
            (2025, 1, 28, "Chinese New Year's Eve"),
            (2025, 5, 2, "Labour Day"),
            (2025, 10, 6, "Mid-Autumn Festival"),
        ],
    );
    // 除夕 was dropped from the statutory list between 2014 and 2024, and
    // Qingming was not on it before 2008.
    expect_working("CN", None, &[(2024, 2, 9), (2007, 4, 5)]);
}

#[test]
fn a_chinese_working_day_is_a_weekend_day_and_never_a_day_off() {
    use hc_calendar::Weekday;
    use hc_holiday::rule::Kind;
    let mut worked = 0;
    for year in 2008..=2026 {
        let calendar = HolidayCalendar::for_year(table("CN"), None, year);
        for entry in calendar.all() {
            if entry.kind != Kind::Workday {
                continue;
            }
            worked += 1;
            assert!(
                matches!(
                    Weekday::from_rd(entry.date),
                    Weekday::Saturday | Weekday::Sunday
                ),
                "{year}: {}",
                entry.name
            );
            assert!(!calendar.is_holiday(entry.date), "{year}: {}", entry.name);
            assert!(calendar.is_business_day(entry.date));
        }
    }
    // The nineteen notices name 123 working days between them, and the
    // extension of 2020 took one back.
    assert_eq!(worked, 123);
}

#[test]
fn china_keeps_each_years_arrangement() {
    // 2024: 春节 from Saturday 10 to Saturday 17 February, with Sunday 4
    // and Sunday 18 February worked. The eve, 9 February, was a working
    // day the notice only encouraged employers to give.
    expect("CN", None, &[(2024, 2, 13, "Spring Festival")]);
    let calendar = HolidayCalendar::for_year(table("CN"), None, 2024);
    assert!(calendar.is_business_day(ymd(2024, 2, 4)));
    assert!(calendar.is_business_day(ymd(2024, 2, 18)));
    assert!(calendar.is_business_day(ymd(2024, 2, 9)));
    assert!(!calendar.is_business_day(ymd(2024, 2, 16)));
    // A working Sunday counts in business-day arithmetic: from Friday
    // 2 February 2024 the next business day is Sunday the 4th.
    assert_eq!(
        calendar.add_business_days(ymd(2024, 2, 2), 1),
        Some(ymd(2024, 2, 4))
    );
    // The three notices that changed a year after its arrangement.
    expect("CN", None, &[(2020, 1, 31, "Spring Festival")]);
    let calendar = HolidayCalendar::for_year(table("CN"), None, 2020);
    assert!(!calendar.is_business_day(ymd(2020, 2, 1)));
    expect(
        "CN",
        None,
        &[
            (2019, 5, 2, "Labour Day"),
            (
                2015,
                9,
                4,
                "70th anniversary of the victory of the War of Resistance against Japanese Aggression",
            ),
        ],
    );
    let calendar = HolidayCalendar::for_year(table("CN"), None, 2015);
    assert!(calendar.is_business_day(ymd(2015, 9, 6)));
    // An arrangement that begins in the December before: 2019's New
    // Year's Day ran from 30 December 2018, and Saturday the 29th was
    // worked.
    expect("CN", None, &[(2018, 12, 31, "New Year's Day")]);
    let calendar = HolidayCalendar::for_year(table("CN"), None, 2018);
    assert!(calendar.is_business_day(ymd(2018, 12, 29)));
    // A year no arrangement carried here covers is a gap, not a guess.
    for year in [2007, 2027] {
        let calendar = HolidayCalendar::for_year(table("CN"), None, year);
        assert!(!calendar.is_complete(), "{year}");
    }
    assert!(HolidayCalendar::for_year(table("CN"), None, 2026).is_complete());
    // Before the 1999 revision the statute itself was not read, so 1998 is
    // a gap for the statutory days as well as for the arrangement, and no
    // day is answered on the 1999 text's authority.
    let calendar = HolidayCalendar::for_year(table("CN"), None, 1998);
    assert!(
        calendar
            .gaps()
            .iter()
            .any(|gap| gap.year == 1998 && gap.name == "Statutory holidays"),
        "{:?}",
        calendar.gaps()
    );
    assert!(calendar.all().is_empty(), "{:?}", calendar.all());
    // From 1999 the statutory days are answered; only the arrangement is
    // missing.
    let calendar = HolidayCalendar::for_year(table("CN"), None, 1999);
    assert!(calendar.is_holiday(ymd(1999, 10, 1)));
    assert!(
        !calendar
            .gaps()
            .iter()
            .any(|gap| gap.name == "Statutory holidays")
    );
}

/// 行政院人事行政總處's 政府行政機關辦公日曆表 for 2017 to 2027: the
/// weekdays off, and the Saturdays worked.
#[rustfmt::skip]
const TW_OFFICE_CALENDARS: &[ProductionYear] = &[
    (2017, &[(1, 2), (1, 27), (1, 30), (1, 31), (2, 1), (2, 27), (2, 28), (4, 3), (4, 4), (5, 29), (5, 30), (10, 4), (10, 9), (10, 10)], &[(2, 18), (6, 3), (9, 30)]),
    (2018, &[(1, 1), (2, 15), (2, 16), (2, 19), (2, 20), (2, 28), (4, 4), (4, 5), (4, 6), (6, 18), (9, 24), (10, 10), (12, 31)], &[(3, 31), (12, 22)]),
    (2019, &[(1, 1), (2, 4), (2, 5), (2, 6), (2, 7), (2, 8), (2, 28), (3, 1), (4, 4), (4, 5), (6, 7), (9, 13), (10, 10), (10, 11)], &[(1, 19), (2, 23), (10, 5)]),
    (2020, &[(1, 1), (1, 23), (1, 24), (1, 27), (1, 28), (1, 29), (2, 28), (4, 2), (4, 3), (6, 25), (6, 26), (10, 1), (10, 2), (10, 9)], &[(2, 15), (6, 20), (9, 26)]),
    (2021, &[(1, 1), (2, 10), (2, 11), (2, 12), (2, 15), (2, 16), (3, 1), (4, 2), (4, 5), (6, 14), (9, 20), (9, 21), (10, 11), (12, 31)], &[(2, 20), (9, 11)]),
    (2022, &[(1, 31), (2, 1), (2, 2), (2, 3), (2, 4), (2, 28), (4, 4), (4, 5), (6, 3), (9, 9), (10, 10)], &[(1, 22)]),
    (2023, &[(1, 2), (1, 20), (1, 23), (1, 24), (1, 25), (1, 26), (1, 27), (2, 27), (2, 28), (4, 3), (4, 4), (4, 5), (6, 22), (6, 23), (9, 29), (10, 9), (10, 10)], &[(1, 7), (2, 4), (2, 18), (3, 25), (6, 17), (9, 23)]),
    (2024, &[(1, 1), (2, 8), (2, 9), (2, 12), (2, 13), (2, 14), (2, 28), (4, 4), (4, 5), (6, 10), (9, 17), (10, 10)], &[(2, 17)]),
    (2025, &[(1, 1), (1, 27), (1, 28), (1, 29), (1, 30), (1, 31), (2, 28), (4, 3), (4, 4), (5, 30), (9, 29), (10, 6), (10, 10), (10, 24), (12, 25)], &[(2, 8)]),
    (2026, &[(1, 1), (2, 16), (2, 17), (2, 18), (2, 19), (2, 20), (2, 27), (4, 3), (4, 6), (5, 1), (6, 19), (9, 25), (9, 28), (10, 9), (10, 26), (12, 25)], &[]),
    (2027, &[(1, 1), (2, 4), (2, 5), (2, 8), (2, 9), (2, 10), (3, 1), (4, 5), (4, 6), (4, 30), (6, 9), (9, 15), (9, 28), (10, 11), (10, 25), (12, 24), (12, 31)], &[]),
];

#[test]
fn taiwan_matches_the_government_office_calendar_from_2017_to_2027() {
    use hc_calendar::Weekday;
    for &(year, days_off, days_worked) in TW_OFFICE_CALENDARS {
        let calendar = HolidayCalendar::for_year(table("TW"), None, year);
        assert!(calendar.is_complete(), "{year}: {:?}", calendar.gaps());
        let mut off = Vec::new();
        let mut worked = Vec::new();
        for fixed in ymd(year, 1, 1).0..=ymd(year, 12, 31).0 {
            let day = Rd(fixed);
            let (_, month, date) = gregorian::from_fixed(day).unwrap();
            let weekend = matches!(Weekday::from_rd(day), Weekday::Saturday | Weekday::Sunday);
            let working = calendar.is_business_day(day);
            if !weekend && !working {
                off.push((month, date));
            }
            if weekend && working {
                worked.push((month, date));
            }
        }
        assert_eq!(off, days_off, "{year}: weekdays off");
        assert_eq!(worked, days_worked, "{year}: Saturdays worked");
    }
}

#[test]
fn taiwan_holidays_and_the_nearest_weekday_adjustment() {
    expect(
        "TW",
        None,
        &[
            (2024, 2, 9, "Lunar New Year's Eve"),
            (2024, 2, 28, "Peace Memorial Day"),
            // 清明 fell on 4 April 2024, a Thursday, so Children's Day was
            // kept the day after.
            (2024, 4, 5, "Children's Day"),
            (2020, 4, 3, "Children's Day"),
            (2026, 2, 15, "Day before Lunar New Year's Eve"),
            (2025, 1, 29, "Spring Festival"),
            (2025, 10, 10, "National Day"),
        ],
    );
    // 端午節 2025 fell on a Saturday, so the makeup day was the Friday
    // before; 教師節 fell on a Sunday, so it was the Monday after.
    expect_substitute("TW", None, 2025, (5, 31), (5, 30));
    expect_substitute("TW", None, 2025, (9, 28), (9, 29));
    // Teachers' Day, Retrocession Day and Constitution Day became holidays
    // again only in 2025.
    expect_working("TW", None, &[(2024, 9, 28), (2024, 10, 25), (2024, 12, 25)]);
    // Labour Day became a day off for government offices only in 2026.
    expect_working("TW", None, &[(2025, 5, 1)]);
    // The Lunar New Year days are made up after: 小年夜 2026 was a Sunday,
    // made up on Friday 20 February after the four days that followed it.
    expect_substitute("TW", None, 2026, (2, 15), (2, 20));
    // The swaps ended in 2025, so a later year is complete without them,
    // and an earlier one than the calendars read is a gap.
    assert!(HolidayCalendar::for_year(table("TW"), None, 2030).is_complete());
    assert!(!HolidayCalendar::for_year(table("TW"), None, 2016).is_complete());
}

#[test]
fn south_korea_holidays() {
    expect(
        "KR",
        None,
        &[
            (2024, 2, 9, "Seollal"),
            (2024, 5, 15, "Buddha's Birthday"),
            (2024, 9, 17, "Chuseok"),
            (2024, 10, 9, "Hangul Day"),
            (2025, 1, 29, "Seollal"),
            (2025, 10, 6, "Chuseok"),
        ],
    );
    // Hangul Day was not a public holiday between 1991 and 2012.
    expect_working("KR", None, &[(2000, 10, 9)]);
    // Constitution Day stopped being one after 2007.
    expect("KR", None, &[(2007, 7, 17, "Constitution Day")]);
    expect_working("KR", None, &[(2010, 7, 17)]);
    // Both came in with the decree in force from May 2026, restored and new.
    expect(
        "KR",
        None,
        &[
            (2026, 5, 1, "Labour Day"),
            (2026, 7, 17, "Constitution Day"),
        ],
    );
    expect_working("KR", None, &[(2025, 5, 1)]);
    // Election days, and a day the government designated.
    expect(
        "KR",
        None,
        &[
            (2024, 4, 10, "22nd National Assembly election"),
            (2026, 6, 3, "9th local elections"),
            (2024, 10, 1, "Armed Forces Day"),
        ],
    );
    expect_working("KR", None, &[(2023, 10, 1)]);
    // The days the decree dropped: 2 January to 1998 and 3 January to
    // 1989, 식목일 to 2005 (with 사방의 날 in its place in 1960 alone) and
    // 국군의 날 from 1976 to 1990.
    expect(
        "KR",
        None,
        &[
            (1998, 1, 2, "New Year Holiday"),
            (1989, 1, 3, "New Year Holiday"),
            (1959, 4, 5, "Arbor Day"),
            (1960, 3, 21, "Erosion Control Day"),
            (1961, 4, 5, "Arbor Day"),
            (2005, 4, 5, "Arbor Day"),
            (1976, 10, 1, "Armed Forces Day"),
            (1990, 10, 1, "Armed Forces Day"),
            (1956, 6, 6, "Memorial Day"),
        ],
    );
    expect_working(
        "KR",
        None,
        &[
            (1999, 1, 2),
            (1990, 1, 3),
            (1960, 4, 5),
            (2006, 4, 5),
            (1975, 10, 1),
            (1991, 10, 1),
            (1955, 6, 6),
        ],
    );
}

#[test]
fn the_korean_substitute_holiday_covers_sundays_and_collisions() {
    // Seollal 2024 ran Friday to Sunday; the substitute was the Monday.
    expect_substitute("KR", None, 2024, (2, 11), (2, 12));
    // Independence Movement Day 2025 fell on a Saturday, which has counted
    // since July 2021.
    expect_substitute("KR", None, 2025, (3, 1), (3, 3));
    // 5 May 2025 was Children's Day and Buddha's Birthday at once, and the
    // 6th was the 대체공휴일 — a collision, not a weekend.
    expect_substitute("KR", None, 2025, (5, 5), (5, 6));
    // A collision with a holiday that is not itself substituted still
    // counts: the eve of Chuseok 2017 was National Foundation Day, four
    // years before that day came under the rule.
    expect_substitute("KR", None, 2017, (10, 3), (10, 6));
    // Constitution Day and Labour Day are substituted from the start.
    expect_substitute("KR", None, 2027, (7, 17), (7, 19));
    expect_substitute("KR", None, 2027, (5, 1), (5, 3));
    // Before 2014 there was no substitute at all: 3 October 2010 was a
    // Sunday.
    expect_working("KR", None, &[(2010, 10, 4)]);
    // Except under the 익일휴무제 of 1989–1990: 국군의 날 fell on Sunday
    // 1 October 1989 and Monday the 2nd was off, the one day the rule ever
    // produced. The Seollal run was outside it — Sunday 28 January 1990,
    // the day after Seollal, earned nothing — and 개천절 on the day of
    // Chuseok, Wednesday 3 October 1990, is not carried as owing a day.
    expect_substitute("KR", None, 1989, (10, 1), (10, 2));
    expect_working("KR", None, &[(1990, 1, 29), (1990, 10, 5)]);
    // And not before 1989: 한글날 fell on Sunday 9 October 1988.
    expect_working("KR", None, &[(1988, 10, 10)]);
}

#[test]
fn india_central_government_holidays() {
    expect(
        "IN",
        None,
        &[
            (2024, 1, 26, "Republic Day"),
            (2024, 8, 15, "Independence Day"),
            (2024, 10, 2, "Gandhi Jayanti"),
            (2024, 12, 25, "Christmas Day"),
            (2025, 1, 26, "Republic Day"),
            (2025, 4, 18, "Good Friday"),
            // The Hindu, Jain, Buddhist and Sikh gazetted days, as the
            // Rashtriya Panchang lists them.
            (2024, 3, 25, "Holi"),
            (2024, 4, 17, "Ram Navami"),
            (2024, 4, 21, "Mahavir Jayanti"),
            (2024, 5, 23, "Buddha Purnima"),
            (2024, 8, 26, "Janmashtami"),
            (2024, 10, 12, "Dussehra"),
            (2024, 10, 31, "Diwali"),
            (2024, 11, 15, "Guru Nanak's Birthday"),
            (2025, 3, 14, "Holi"),
            (2025, 4, 6, "Ram Navami"),
        ],
    );
    // India has no observed-day rule: 26 January 2025 was a Sunday.
    expect_working("IN", None, &[(2025, 1, 27)]);
    expect_working("IN", None, &[(1946, 8, 15)]);
}

#[test]
fn thailand_holidays_and_its_monday_substitution() {
    expect(
        "TH",
        None,
        &[
            (2024, 4, 13, "Songkran"),
            (2024, 7, 28, "King Vajiralongkorn's Birthday"),
            (2024, 10, 23, "Chulalongkorn Day"),
            (2025, 5, 4, "Coronation Day"),
            (2025, 12, 10, "Constitution Day"),
        ],
    );
    // 28 July 2024 was a Sunday and the Monday was the substitute; 13
    // October 2024 likewise.
    expect_substitute("TH", None, 2024, (7, 28), (7, 29));
    expect_substitute("TH", None, 2024, (10, 13), (10, 14));
    // Coronation Day dates from the 2019 coronation.
    expect_working("TH", None, &[(2018, 5, 4)]);
}

#[test]
fn thailand_keeps_its_buddhist_days_on_the_thai_lunar_calendar() {
    // As the Bank of Thailand's notifications give them. 2023 and 2026
    // double month 8, so Makha and Visakha Bucha are the full moons of
    // months 4 and 7, and Asalha Bucha that of the second month 8.
    expect(
        "TH",
        None,
        &[
            (2012, 3, 7, "Makha Bucha"),
            (2012, 6, 4, "Visakha Bucha"),
            (2012, 8, 2, "Asalha Bucha"),
            (2012, 8, 3, "Khao Phansa"),
            (2023, 3, 6, "Makha Bucha"),
            (2023, 6, 3, "Visakha Bucha"),
            (2023, 8, 1, "Asalha Bucha"),
            (2023, 8, 2, "Khao Phansa"),
            (2025, 2, 12, "Makha Bucha"),
            (2025, 5, 11, "Visakha Bucha"),
            (2025, 7, 10, "Asalha Bucha"),
            (2025, 7, 11, "Khao Phansa"),
            (2026, 3, 3, "Makha Bucha"),
            (2026, 5, 31, "Visakha Bucha"),
            (2026, 7, 29, "Asalha Bucha"),
            (2026, 7, 30, "Khao Phansa"),
            (2027, 2, 21, "Makha Bucha"),
            (2027, 5, 20, "Visakha Bucha"),
            (2027, 7, 18, "Asalha Bucha"),
            (2004, 8, 1, "Khao Phansa"),
            (1992, 2, 18, "Makha Bucha"),
        ],
    );
    // The Monday in place of a weekend one, as the notifications give it.
    expect_substitute("TH", None, 2024, (2, 24), (2, 26));
    expect_substitute("TH", None, 2025, (5, 11), (5, 12));
    expect_substitute("TH", None, 2026, (5, 31), (6, 1));
    expect_substitute("TH", None, 2027, (2, 21), (2, 22));
    // The Chinese full moon the table used to approximate Makha Bucha by
    // was a month early in 2012.
    expect_working("TH", None, &[(2012, 2, 6)]);
    for year in [1992, 2012, 2026, 2027] {
        let calendar = HolidayCalendar::for_year(table("TH"), None, year);
        assert!(calendar.is_complete(), "{year}: {:?}", calendar.gaps());
        assert!(
            calendar
                .in_year(year)
                .iter()
                .all(|holiday| holiday.confidence == Confidence::Exact),
            "{year}"
        );
    }
    // Beyond the years Thailand has published the four are gaps, not
    // guesses.
    for year in [1991, 2028] {
        let calendar = HolidayCalendar::for_year(table("TH"), None, year);
        let missing: Vec<&str> = calendar.gaps().iter().map(|gap| gap.name).collect();
        for name in [
            "Makha Bucha",
            "Visakha Bucha",
            "Asalha Bucha",
            "Khao Phansa",
        ] {
            assert!(missing.contains(&name), "{year}: {name} should be a gap");
        }
    }
}

#[test]
fn vietnam_holidays() {
    expect(
        "VN",
        None,
        &[
            (2024, 2, 10, "Tết"),
            (2024, 4, 18, "Hùng Kings' Festival"),
            (2024, 4, 30, "Reunification Day"),
            (2025, 1, 29, "Tết"),
            (2025, 4, 7, "Hùng Kings' Festival"),
            (2025, 9, 2, "National Day"),
        ],
    );
    // The second National Day holiday dates from 2021.
    expect("VN", None, &[(2025, 9, 1, "National Day")]);
    expect_working("VN", None, &[(2019, 9, 1), (2006, 4, 7)]);
    // Vietnamese Culture Day, from 2026.
    expect("VN", None, &[(2026, 11, 24, "Vietnamese Culture Day")]);
    expect_working("VN", None, &[(2025, 11, 24)]);
}

/// Days of one year, as (month, day).
type MonthDays = Vec<(u8, u8)>;

/// The weekdays a year's calendar gives off and the weekend days it makes
/// working days.
fn weekdays_off_and_weekends_worked(code: &str, year: i64) -> (MonthDays, MonthDays) {
    use hc_calendar::Weekday;
    let calendar = HolidayCalendar::for_year(table(code), None, year);
    let mut off = Vec::new();
    let mut worked = Vec::new();
    for month in 1..=12u8 {
        for day in 1..=31u8 {
            let Ok(date) = gregorian::to_fixed(year, month, day) else {
                continue;
            };
            let weekend = matches!(Weekday::from_rd(date), Weekday::Saturday | Weekday::Sunday);
            if !weekend && !calendar.is_business_day(date) {
                off.push((month, day));
            }
            if weekend && calendar.is_business_day(date) {
                worked.push((month, day));
            }
        }
    }
    (off, worked)
}

#[test]
fn vietnam_keeps_each_years_notices() {
    // Each year's weekdays off and Saturdays worked for the civil service,
    // assembled by hand from the statutory days, article 111(3)'s make-up
    // day for one on a weekend, and the notices.
    type Year<'a> = (i64, &'a [(u8, u8)], &'a [(u8, u8)]);
    #[rustfmt::skip]
    let years: &[Year] = &[
        (2021, &[
            (1, 1),
            // 4875/TB-LĐTBXH: Tết 10–14 February, and 15 and 16 February
            // in lieu of the Saturday and Sunday among them.
            (2, 10), (2, 11), (2, 12), (2, 15), (2, 16),
            // Hùng Kings' Festival, 10/3 of Tân Sửu.
            (4, 21),
            // 1 May was a Saturday, made up on Monday the 3rd.
            (4, 30), (5, 3),
            // The notice's second National Day holiday is the 3rd.
            (9, 2), (9, 3),
        ], &[]),
        (2022, &[
            // 1 January was a Saturday.
            (1, 3),
            // 119/TB-LĐTBXH: Tết 31 January to 4 February.
            (1, 31), (2, 1), (2, 2), (2, 3), (2, 4),
            // Hùng Kings' Festival on Sunday 10 April.
            (4, 11),
            // 30 April and 1 May on the weekend, made up on the 2nd and 3rd.
            (5, 2), (5, 3),
            // The second National Day holiday is the 1st.
            (9, 1), (9, 2),
        ], &[]),
        (2023, &[
            // 1 January was a Sunday.
            (1, 2),
            // 5034/TB-LĐTBXH: Tết 20–24 January, with the 25th and 26th in
            // lieu of the Saturday and Sunday.
            (1, 20), (1, 23), (1, 24), (1, 25), (1, 26),
            // Hùng Kings' Festival on Saturday 29 April and 30 April on the
            // Sunday, made up on 2 and 3 May.
            (5, 1), (5, 2), (5, 3),
            // National Day on 1 and 2 September, the 2nd a Saturday made
            // up on Monday the 4th, as the notice says.
            (9, 1), (9, 4),
        ], &[]),
        (2024, &[
            (1, 1),
            // 5015/TB-LĐTBXH: Tết 8–12 February, with the 13th and 14th in
            // lieu of the Saturday and Sunday.
            (2, 8), (2, 9), (2, 12), (2, 13), (2, 14),
            (4, 18),
            // 1570/TB-LĐTBXH: Monday 29 April swapped for Saturday 4 May.
            (4, 29), (4, 30), (5, 1),
            // The second National Day holiday is the 3rd.
            (9, 2), (9, 3),
        ], &[(5, 4)]),
        (2025, &[
            (1, 1),
            // 6150/TB-LĐTBXH: Tết 27–31 January, two days before the
            // 29th and three after.
            (1, 27), (1, 28), (1, 29), (1, 30), (1, 31),
            (4, 7),
            // The same notice: Friday 2 May swapped for Saturday 26 April.
            (4, 30), (5, 1), (5, 2),
            // The second National Day holiday is the 1st.
            (9, 1), (9, 2),
        ], &[(4, 26)]),
        (2026, &[
            // 12729/VPCP-KGVX: Friday 2 January swapped for Saturday
            // 10 January.
            (1, 1), (1, 2),
            // 9441/TB-BNV: Tết 16–20 February.
            (2, 16), (2, 17), (2, 18), (2, 19), (2, 20),
            // Hùng Kings' Festival on Sunday 26 April, made up on the 27th;
            // 3383/BNV-CVL: no swap around 30 April.
            (4, 27), (4, 30), (5, 1),
            // 9441/TB-BNV: the second National Day holiday is the 1st, and
            // Monday 31 August is swapped for Saturday 22 August.
            (8, 31), (9, 1), (9, 2),
            // Nghị quyết 28/2026/QH16.
            (11, 24),
        ], &[(1, 10), (8, 22)]),
    ];
    for &(year, off, worked) in years {
        let (found_off, found_worked) = weekdays_off_and_weekends_worked("VN", year);
        assert_eq!(found_off, off, "{year}: weekdays off");
        assert_eq!(found_worked, worked, "{year}: weekend days worked");
        assert!(
            HolidayCalendar::for_year(table("VN"), None, year).is_complete(),
            "{year}"
        );
    }
    // A Tết day the notice counts on a weekend is not moved again: in 2021
    // the 13th and 14th are Tết, and the days in lieu are the notice's own.
    let calendar = HolidayCalendar::for_year(table("VN"), None, 2021);
    assert!(
        !calendar
            .all()
            .iter()
            .any(|holiday| holiday.name == "Tết" && holiday.is_substitute())
    );
    // A year no notice carried here covers is a gap, not a guess.
    for year in [2020, 2027] {
        let calendar = HolidayCalendar::for_year(table("VN"), None, year);
        assert!(!calendar.is_complete(), "{year}");
    }
}

#[test]
fn indonesia_holidays() {
    expect(
        "ID",
        None,
        &[
            (2024, 3, 29, "Good Friday"),
            (2024, 5, 9, "Ascension"),
            (2024, 8, 17, "Independence Day"),
            (2025, 1, 29, "Chinese New Year"),
            (2025, 6, 1, "Pancasila Day"),
            (2025, 12, 25, "Christmas Day"),
        ],
    );
    // Chinese New Year became a national holiday in 2003 and Pancasila Day
    // in 2017; Indonesia has no observed-day rule.
    expect_working("ID", None, &[(2002, 2, 12), (2016, 6, 1), (2025, 8, 18)]);
}

#[test]
fn singapore_and_malaysia() {
    expect(
        "SG",
        None,
        &[
            (2024, 2, 10, "Chinese New Year"),
            (2024, 3, 29, "Good Friday"),
            (2024, 8, 9, "National Day"),
            (2025, 1, 29, "Chinese New Year"),
            (2025, 5, 1, "Labour Day"),
            // The Ministry of Manpower's Deepavali, 2020 to 2026.
            (2020, 11, 14, "Deepavali"),
            (2021, 11, 4, "Deepavali"),
            (2022, 10, 24, "Deepavali"),
            (2023, 11, 12, "Deepavali"),
            (2024, 10, 31, "Deepavali"),
            (2025, 10, 20, "Deepavali"),
            (2026, 11, 8, "Deepavali"),
        ],
    );
    // Singapore moves a Sunday holiday to the Monday and leaves a Saturday
    // one alone: Chinese New Year 2024 ran Saturday and Sunday; Deepavali
    // 2026 is a Sunday.
    expect_substitute("SG", None, 2024, (2, 11), (2, 12));
    expect_substitute("SG", None, 2026, (11, 8), (11, 9));
    expect_working("SG", None, &[(2025, 8, 11)]);
    expect(
        "MY",
        None,
        &[
            (2024, 5, 1, "Labour Day"),
            (2024, 6, 3, "Agong's Birthday"),
            (2024, 8, 31, "National Day"),
            (2024, 9, 16, "Malaysia Day"),
            (2025, 6, 2, "Agong's Birthday"),
            // The gazetted Deepavali, 2023 to 2025.
            (2023, 11, 12, "Deepavali"),
            (2024, 10, 31, "Deepavali"),
            (2025, 10, 20, "Deepavali"),
        ],
    );
    // 31 August 2025 was a Sunday, so 1 September was the substitute.
    expect_substitute("MY", None, 2025, (8, 31), (9, 1));
    expect_working("MY", None, &[(2009, 9, 16)]);
}

#[test]
fn the_philippines_distinguishes_regular_from_special_days() {
    expect(
        "PH",
        None,
        &[
            (2024, 4, 9, "Day of Valour"),
            (2024, 6, 12, "Independence Day"),
            (2024, 8, 26, "National Heroes Day"),
            (2024, 12, 30, "Rizal Day"),
            (2025, 4, 17, "Maundy Thursday"),
            (2025, 8, 25, "National Heroes Day"),
        ],
    );
    let calendar = HolidayCalendar::for_year(table("PH"), None, 2025);
    let all_souls = calendar.on(ymd(2025, 11, 1));
    assert!(
        all_souls
            .iter()
            .any(|holiday| holiday.name == "All Saints' Day"),
        "All Saints' Day should be listed as a special non-working day"
    );
}

#[test]
fn nepal_keeps_a_one_day_weekend_until_2026() {
    let country = table("NP");
    let old = HolidayCalendar::for_year(country, None, 2024);
    // Saturday is the weekend; Sunday is an ordinary working day.
    assert!(old.is_weekend(ymd(2024, 3, 2)));
    assert!(!old.is_weekend(ymd(2024, 3, 3)));
    assert!(old.is_business_day(ymd(2024, 3, 3)));
    let new = HolidayCalendar::for_year(country, None, 2026);
    assert!(new.is_weekend(ymd(2026, 3, 7)));
    assert!(new.is_weekend(ymd(2026, 3, 8)));
}

#[test]
fn nepal_keeps_the_days_its_notices_date() {
    // The Ministry of Home Affairs' notices for 2082 and 2083 BS, sections
    // 2.1, 6.1 and 7.1, each with the weekday the notice prints beside it.
    // Maghe Sankranti 2082 is where the gazette's Magh 1 and the computed
    // one part, and the gazette's is kept.
    expect(
        "NP",
        None,
        &[
            (2025, 4, 14, "Nepali New Year"),          // Baisakh 1, Monday
            (2025, 5, 1, "Labour Day"),                // Baisakh 18, Thursday
            (2025, 5, 29, "Republic Day"),             // Jeth 15, Thursday
            (2025, 9, 19, "Constitution Day"),         // Asoj 3, Friday
            (2025, 12, 25, "Christmas Day"),           // Pus 10, Thursday
            (2026, 1, 11, "Prithvi Jayanti"),          // Pus 27, Sunday
            (2026, 1, 15, "Maghe Sankranti"),          // Magh 1, Thursday
            (2026, 1, 30, "Martyrs' Day"),             // Magh 16, Friday
            (2026, 2, 19, "National Democracy Day"),   // Phagun 7, Thursday
            (2026, 3, 8, "International Women's Day"), // Phagun 24, Sunday
            (2026, 4, 14, "Nepali New Year"),          // Baisakh 1, Tuesday
            (2026, 5, 1, "Labour Day"),                // Baisakh 18, Friday
            (2026, 5, 29, "Republic Day"),             // Jeth 15, Friday
            (2026, 9, 19, "Constitution Day"),         // Asoj 3, Saturday
            (2026, 12, 25, "Christmas Day"),           // Pus 10, Friday
            (2027, 1, 11, "Prithvi Jayanti"),          // Pus 27, Monday
            (2027, 1, 15, "Maghe Sankranti"),          // Magh 1, Friday
            (2027, 1, 30, "Martyrs' Day"),             // Magh 16, Saturday
            (2027, 2, 19, "National Democracy Day"),   // Phagun 7, Friday
            (2027, 3, 8, "International Women's Day"), // Phagun 24, Monday
        ],
    );
    // A date the gazette fixes is not a prediction.
    let calendar = HolidayCalendar::for_year(table("NP"), None, 2026);
    assert!(
        calendar
            .on(ymd(2026, 1, 15))
            .iter()
            .all(|holiday| holiday.confidence == Confidence::Exact)
    );
}

#[test]
fn nepal_keeps_its_festivals_on_the_days_its_notices_give() {
    // The notices for 2080 to 2083 BS, sections 2.1 and 7.1: each
    // festival's Bikram Sambat date, here in the Gregorian calendar. The
    // rules were fitted to 2082 and 2083; 2080 and 2081 check them.
    expect(
        "NP",
        None,
        &[
            (2023, 5, 5, "Buddha Jayanti"),       // Baisakh 22
            (2023, 8, 31, "Janai Purnima"),       // Bhadau 14
            (2023, 9, 6, "Krishna Janmashtami"),  // Bhadau 20
            (2023, 10, 15, "Ghatasthapana"),      // Asoj 28
            (2023, 12, 26, "Dhanya Purnima"),     // Pus 10
            (2023, 12, 31, "Tamu Lhosar"),        // Pus 15
            (2024, 2, 10, "Sonam Lhosar"),        // Magh 27
            (2024, 3, 8, "Maha Shivaratri"),      // Phagun 25
            (2024, 3, 11, "Gyalpo Lhosar"),       // Phagun 28
            (2024, 5, 23, "Buddha Jayanti"),      // Jeth 10
            (2024, 8, 19, "Janai Purnima"),       // Bhadau 3
            (2024, 8, 26, "Krishna Janmashtami"), // Bhadau 10
            (2024, 10, 3, "Ghatasthapana"),       // Asoj 17
            (2024, 12, 15, "Dhanya Purnima"),     // Mangsir 30
            (2024, 12, 30, "Tamu Lhosar"),        // Pus 15
            (2025, 1, 30, "Sonam Lhosar"),        // Magh 17
            (2025, 2, 26, "Maha Shivaratri"),     // Phagun 14
            (2025, 2, 28, "Gyalpo Lhosar"),       // Phagun 16
            (2025, 5, 12, "Buddha Jayanti"),      // Baisakh 29
            (2025, 8, 9, "Janai Purnima"),        // Saun 24
            (2025, 8, 16, "Krishna Janmashtami"), // Saun 31
            (2025, 9, 22, "Ghatasthapana"),       // Asoj 6
            (2025, 12, 4, "Dhanya Purnima"),      // Mangsir 18
            (2025, 12, 30, "Tamu Lhosar"),        // Pus 15
            (2026, 1, 19, "Sonam Lhosar"),        // Magh 5
            (2026, 2, 15, "Maha Shivaratri"),     // Phagun 3
            (2026, 2, 18, "Gyalpo Lhosar"),       // Phagun 6
            (2026, 5, 1, "Buddha Jayanti"),       // Baisakh 18
            (2026, 8, 28, "Janai Purnima"),       // Bhadau 12
            (2026, 9, 4, "Krishna Janmashtami"),  // Bhadau 19
            (2026, 10, 11, "Ghatasthapana"),      // Asoj 25
            (2026, 12, 24, "Dhanya Purnima"),     // Pus 9
            (2026, 12, 30, "Tamu Lhosar"),        // Pus 15
            (2027, 2, 7, "Sonam Lhosar"),         // Magh 24
            (2027, 3, 6, "Maha Shivaratri"),      // Phagun 22
            (2027, 3, 9, "Gyalpo Lhosar"),        // Phagun 25
        ],
    );
}

#[test]
fn nepal_keeps_dashain_and_tihar_for_as_many_days_as_the_notices_give() {
    // Dashain runs from Phulpati to Dwadashi and Tihar from Lakshmi Puja to
    // the day after Bhai Tika, each first and last day as the notices for
    // 2080 to 2083 BS give them: Dashain five, six or seven days long.
    type MonthDay = (u8, u8);
    let spans: [(&str, i64, MonthDay, MonthDay); 8] = [
        ("Dashain", 2023, (10, 21), (10, 26)), // Kattik 4–9, 2080
        ("Dashain", 2024, (10, 10), (10, 14)), // Asoj 24–28, 2081
        ("Dashain", 2025, (9, 29), (10, 4)),   // Asoj 13–18, 2082
        ("Dashain", 2026, (10, 17), (10, 23)), // Asoj 31 to Kattik 6, 2083
        ("Tihar", 2023, (11, 12), (11, 16)),   // Kattik 26–30, 2080
        ("Tihar", 2024, (10, 31), (11, 4)),    // Kattik 15–19, 2081
        ("Tihar", 2025, (10, 20), (10, 24)),   // Kattik 3–7, 2082
        ("Tihar", 2026, (11, 8), (11, 12)),    // Kattik 22–26, 2083
    ];
    for (name, year, (from_month, from_day), (to_month, to_day)) in spans {
        let found: Vec<Rd> = HolidayCalendar::for_year(table("NP"), None, year)
            .in_year(year)
            .iter()
            .filter(|holiday| holiday.name == name)
            .map(|holiday| holiday.date)
            .collect();
        let expected: Vec<Rd> = (ymd(year, from_month, from_day).0..=ymd(year, to_month, to_day).0)
            .map(Rd)
            .collect();
        assert_eq!(found, expected, "{name} {year}");
    }
}

#[test]
fn sri_lanka_keeps_every_day_its_gazettes_list() {
    // The Holidays Act orders for 2023 to 2027, every row of each
    // schedule, public and bank holiday alike.
    expect(
        "LK",
        None,
        &[
            (2023, 1, 6, "Duruthu Full Moon Poya Day"),
            (2023, 1, 15, "Tamil Thai Pongal Day"),
            (2023, 1, 16, "Special Bank Holiday"),
            (2023, 2, 4, "Independence Day"),
            (2023, 2, 5, "Navam Full Moon Poya Day"),
            (2023, 2, 18, "Maha Shivarathri Day"),
            (2023, 3, 6, "Medin Full Moon Poya Day"),
            (2023, 4, 5, "Bak Full Moon Poya Day"),
            (2023, 4, 7, "Good Friday"),
            (2023, 4, 13, "Day Prior to Sinhala & Tamil New Year Day"),
            (2023, 4, 14, "Sinhala & Tamil New Year Day"),
            (2023, 4, 22, "Id-Ul-Fitr (Ramazan Festival Day)"),
            (2023, 5, 1, "May Day (International Workers' Day)"),
            (2023, 5, 5, "Vesak Full Moon Poya Day"),
            (2023, 5, 6, "Day Following Vesak Full Moon Poya Day"),
            (2023, 6, 3, "Poson Full Moon Poya Day"),
            (2023, 6, 29, "Id-Ul-Adha (Hadji Festival Day)"),
            (2023, 7, 3, "Adhi Esala Full Moon Poya Day"),
            (2023, 8, 1, "Esala Full Moon Poya Day"),
            (2023, 8, 30, "Nikini Full Moon Poya Day"),
            (2023, 9, 28, "Milad-Un-Nabi (Holy Prophet's Birthday)"),
            (2023, 9, 29, "Binara Full Moon Poya Day"),
            (2023, 10, 28, "Vap Full Moon Poya Day"),
            (2023, 11, 12, "Deepavali Festival Day"),
            (2023, 11, 26, "Il Full Moon Poya Day"),
            (2023, 12, 25, "Christmas Day"),
            (2023, 12, 26, "Unduvap Full Moon Poya Day"),
            (2024, 1, 15, "Tamil Thai Pongal Day"),
            (2024, 1, 25, "Duruthu Full Moon Poya Day"),
            (2024, 2, 4, "Independence Day"),
            (2024, 2, 23, "Navam Full Moon Poya Day"),
            (2024, 3, 8, "Maha Shivarathri Day"),
            (2024, 3, 24, "Medin Full Moon Poya Day"),
            (2024, 3, 29, "Good Friday"),
            (2024, 4, 11, "Id-Ul-Fitr (Ramazan Festival Day)"),
            (2024, 4, 12, "Day Prior to Sinhala & Tamil New Year Day"),
            (2024, 4, 13, "Sinhala & Tamil New Year Day"),
            (2024, 4, 23, "Bak Full Moon Poya Day"),
            (2024, 5, 1, "May Day (International Workers' Day)"),
            (2024, 5, 23, "Vesak Full Moon Poya Day"),
            (2024, 5, 24, "Day Following Vesak Full Moon Poya Day"),
            (2024, 6, 17, "Id-Ul-Adha (Hadji Festival Day)"),
            (2024, 6, 21, "Poson Full Moon Poya Day"),
            (2024, 7, 20, "Esala Full Moon Poya Day"),
            (2024, 8, 19, "Nikini Full Moon Poya Day"),
            (2024, 9, 16, "Milad-Un-Nabi (Holy Prophet's Birthday)"),
            (2024, 9, 17, "Binara Full Moon Poya Day"),
            (2024, 10, 17, "Vap Full Moon Poya Day"),
            (2024, 10, 31, "Deepavali Festival Day"),
            (2024, 11, 15, "Il Full Moon Poya Day"),
            (2024, 12, 14, "Unduvap Full Moon Poya Day"),
            (2024, 12, 25, "Christmas Day"),
            (2025, 1, 13, "Duruthu Full Moon Poya Day"),
            (2025, 1, 14, "Tamil Thai Pongal Day"),
            (2025, 2, 4, "Independence Day"),
            (2025, 2, 12, "Navam Full Moon Poya Day"),
            (2025, 2, 26, "Maha Shivarathri Day"),
            (2025, 3, 13, "Medin Full Moon Poya Day"),
            (2025, 3, 31, "Id-Ul-Fitr (Ramazan Festival Day)"),
            (2025, 4, 12, "Bak Full Moon Poya Day"),
            (2025, 4, 13, "Day Prior to Sinhala & Tamil New Year Day"),
            (2025, 4, 14, "Sinhala & Tamil New Year Day"),
            (2025, 4, 15, "Special Bank Holiday"),
            (2025, 4, 18, "Good Friday"),
            (2025, 5, 1, "May Day (International Workers' Day)"),
            (2025, 5, 12, "Vesak Full Moon Poya Day"),
            (2025, 5, 13, "Day Following Vesak Full Moon Poya Day"),
            (2025, 6, 7, "Id-Ul-Adha (Hadji Festival Day)"),
            (2025, 6, 10, "Poson Full Moon Poya Day"),
            (2025, 7, 10, "Esala Full Moon Poya Day"),
            (2025, 8, 8, "Nikini Full Moon Poya Day"),
            (2025, 9, 5, "Milad-Un-Nabi (Holy Prophet's Birthday)"),
            (2025, 9, 7, "Binara Full Moon Poya Day"),
            (2025, 10, 6, "Vap Full Moon Poya Day"),
            (2025, 10, 20, "Deepavali Festival Day"),
            (2025, 11, 5, "Il Full Moon Poya Day"),
            (2025, 12, 4, "Unduvap Full Moon Poya Day"),
            (2025, 12, 25, "Christmas Day"),
            (2026, 1, 3, "Duruthu Full Moon Poya Day"),
            (2026, 1, 15, "Tamil Thai Pongal Day"),
            (2026, 2, 1, "Navam Full Moon Poya Day"),
            (2026, 2, 4, "Independence Day"),
            (2026, 2, 15, "Maha Shivarathri Day"),
            (2026, 3, 2, "Medin Full Moon Poya Day"),
            (2026, 3, 21, "Id-Ul-Fitr (Ramazan Festival Day)"),
            (2026, 4, 1, "Bak Full Moon Poya Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 13, "Day Prior to Sinhala & Tamil New Year Day"),
            (2026, 4, 14, "Sinhala & Tamil New Year Day"),
            (2026, 5, 1, "Vesak Full Moon Poya Day"),
            (2026, 5, 1, "May Day (International Workers' Day)"),
            (2026, 5, 2, "Day Following Vesak Full Moon Poya Day"),
            (2026, 5, 28, "Id-Ul-Adha (Hadji Festival Day)"),
            (2026, 5, 30, "Adhi Poson Full Moon Poya Day"),
            (2026, 6, 29, "Poson Full Moon Poya Day"),
            (2026, 7, 29, "Esala Full Moon Poya Day"),
            (2026, 8, 26, "Milad-Un-Nabi (Holy Prophet's Birthday)"),
            (2026, 8, 27, "Nikini Full Moon Poya Day"),
            (2026, 9, 26, "Binara Full Moon Poya Day"),
            (2026, 10, 25, "Vap Full Moon Poya Day"),
            (2026, 11, 8, "Deepavali Festival Day"),
            (2026, 11, 24, "Il Full Moon Poya Day"),
            (2026, 12, 23, "Unduvap Full Moon Poya Day"),
            (2026, 12, 25, "Christmas Day"),
            (2027, 1, 15, "Tamil Thai Pongal Day"),
            (2027, 1, 22, "Duruthu Full Moon Poya Day"),
            (2027, 2, 4, "Independence Day"),
            (2027, 2, 20, "Navam Full Moon Poya Day"),
            (2027, 3, 6, "Maha Shivarathri Day"),
            (2027, 3, 10, "Id-Ul-Fitr (Ramazan Festival Day)"),
            (2027, 3, 22, "Medin Full Moon Poya Day"),
            (2027, 3, 26, "Good Friday"),
            (2027, 4, 13, "Day Prior to Sinhala & Tamil New Year Day"),
            (2027, 4, 14, "Sinhala & Tamil New Year Day"),
            (2027, 4, 20, "Bak Full Moon Poya Day"),
            (2027, 5, 1, "May Day (International Workers' Day)"),
            (2027, 5, 17, "Id-Ul-Adha (Hadji Festival Day)"),
            (2027, 5, 19, "Vesak Full Moon Poya Day"),
            (2027, 5, 20, "Day Following Vesak Full Moon Poya Day"),
            (2027, 6, 18, "Poson Full Moon Poya Day"),
            (2027, 7, 18, "Esala Full Moon Poya Day"),
            (2027, 8, 15, "Milad-Un-Nabi (Holy Prophet's Birthday)"),
            (2027, 8, 16, "Nikini Full Moon Poya Day"),
            (2027, 9, 15, "Binara Full Moon Poya Day"),
            (2027, 10, 15, "Vap Full Moon Poya Day"),
            (2027, 10, 28, "Deepavali Festival Day"),
            (2027, 11, 13, "Il Full Moon Poya Day"),
            (2027, 12, 13, "Unduvap Full Moon Poya Day"),
            (2027, 12, 25, "Christmas Day"),
        ],
    );
}

#[test]
fn sri_lanka_reports_the_years_its_gazettes_do_not_cover_as_gaps() {
    let covered = HolidayCalendar::for_year(table("LK"), None, 2027);
    assert!(covered.is_complete(), "{:?}", covered.gaps());
    // 2028's gazette is not in hand: its Poya days are unknown, not absent,
    // while Independence Day is a fixed date and still answered.
    let beyond = HolidayCalendar::for_year(table("LK"), None, 2028);
    assert!(!beyond.is_complete());
    assert_eq!(beyond.name_on(ymd(2028, 2, 4)), Some("Independence Day"));
}

#[test]
fn bangladesh_keeps_the_days_its_notifications_date() {
    // The Ministry of Public Administration's notifications for 2025 and
    // 2026, general and executive-order holidays alike, each with the
    // weekday the notification prints.
    expect(
        "BD",
        None,
        &[
            (
                2025,
                2,
                21,
                "Shaheed Day and International Mother Language Day",
            ), // Friday
            (2025, 3, 26, "Independence and National Day"), // Wednesday
            (2025, 4, 14, "Pohela Boishakh"),               // 1 Boishakh 1432, Monday
            (2025, 5, 1, "May Day"),                        // Thursday
            (2025, 5, 11, "Buddha Purnima"),                // Sunday
            (2025, 8, 5, "July Mass Uprising Day"),         // Tuesday
            (2025, 8, 16, "Janmashtami"),                   // Saturday
            (2025, 10, 1, "Durga Puja (Navami)"),           // Wednesday
            (2025, 10, 2, "Durga Puja (Bijoya Dashami)"),   // Thursday
            (2025, 12, 16, "Victory Day"),                  // Tuesday
            (2025, 12, 25, "Christmas Day"),                // Thursday
            (
                2026,
                2,
                21,
                "Shaheed Day and International Mother Language Day",
            ), // Saturday
            (2026, 3, 26, "Independence and National Day"), // Thursday
            (2026, 4, 14, "Pohela Boishakh"),               // 1 Boishakh 1433, Tuesday
            (2026, 5, 1, "Buddha Purnima"),                 // Friday
            (2026, 8, 5, "July Mass Uprising Day"),         // Wednesday
            (2026, 9, 4, "Janmashtami"),                    // Friday
            (2026, 10, 20, "Durga Puja (Navami)"),          // Tuesday
            (2026, 10, 21, "Durga Puja (Bijoya Dashami)"),  // Wednesday
            (2026, 12, 16, "Victory Day"),                  // Wednesday
        ],
    );
    // 7 March and 15 August are no longer holidays; 15 August 2025 is also
    // where the Indian rule puts Janmashtami, a day before the notification.
    expect_working("BD", None, &[(2025, 3, 7), (2025, 8, 15), (2025, 5, 12)]);
    // Chaitra Sankranti, 30 Choitro, is the hill districts' general holiday
    // from the 2026 notification, and nobody else's.
    expect("BD", Some("BD-56"), &[(2026, 4, 13, "Chaitra Sankranti")]);
    expect_working("BD", None, &[(2026, 4, 13)]);
    expect_working("BD", Some("BD-56"), &[(2025, 4, 13)]);
}

#[test]
fn bangladesh_keeps_its_hijri_days_on_the_tabular_calendar_as_predictions() {
    // In 2025 the tabular calendar gives the notification's own dates for
    // the Eids, the days around them, Jumatul Bida, Eid-e-Miladunnabi and
    // Ashura; Shab-e-Barat and Shab-e-Qadr come a day early (the
    // notification's 15 February and 28 March). In 2026 it puts Eid-ul-Fitr
    // on Friday 20 March, a day before the notification's, and so Jumatul
    // Bida on 13 March — which is what approximate means.
    expect(
        "BD",
        None,
        &[
            (2025, 2, 14, "Shab-e-Barat"),
            (2025, 3, 27, "Shab-e-Qadr"),
            (2025, 3, 28, "Jumatul Bida"),
            (2025, 3, 29, "Eid-ul-Fitr"),
            (2025, 3, 30, "Eid-ul-Fitr"),
            (2025, 3, 31, "Eid-ul-Fitr"),
            (2025, 4, 1, "Eid-ul-Fitr"),
            (2025, 4, 2, "Eid-ul-Fitr"),
            (2025, 6, 5, "Eid-ul-Azha"),
            (2025, 6, 7, "Eid-ul-Azha"),
            (2025, 6, 10, "Eid-ul-Azha"),
            (2025, 7, 6, "Ashura"),
            (2025, 9, 5, "Eid-e-Miladunnabi"),
            (2026, 3, 13, "Jumatul Bida"),
            (2026, 3, 20, "Eid-ul-Fitr"),
            (2026, 5, 27, "Eid-ul-Azha"),
            (2026, 8, 26, "Eid-e-Miladunnabi"),
        ],
    );
    expect_working("BD", None, &[(2025, 4, 3), (2025, 6, 11)]);
    let calendar = HolidayCalendar::for_year(table("BD"), None, 2025);
    for holiday in calendar.on(ymd(2025, 3, 31)) {
        assert_eq!(
            holiday.confidence,
            Confidence::Approximate,
            "{}",
            holiday.name
        );
    }
    // Pohela Boishakh is a fixed Bengali date, and exact.
    assert!(
        calendar
            .on(ymd(2025, 4, 14))
            .iter()
            .all(|holiday| holiday.confidence == Confidence::Exact)
    );
}

#[test]
fn bangladesh_keeps_a_friday_saturday_weekend_and_moves_nothing_off_it() {
    let calendar = HolidayCalendar::for_year(table("BD"), None, 2025);
    // Friday 21 February 2025 was Shaheed Day, and the notification counts
    // it among the five general holidays on a weekly holiday.
    assert!(calendar.is_weekend(ymd(2025, 2, 21)));
    assert!(calendar.is_weekend(ymd(2025, 2, 22)));
    assert!(calendar.is_business_day(ymd(2025, 2, 23)));
    assert!(calendar.on(ymd(2025, 2, 23)).is_empty());
}

#[test]
fn bangladesh_reports_the_years_its_notifications_do_not_cover_as_gaps() {
    assert!(
        HolidayCalendar::for_year(table("BD"), None, 2026).is_complete(),
        "{:?}",
        HolidayCalendar::for_year(table("BD"), None, 2026).gaps()
    );
    let beyond = HolidayCalendar::for_year(table("BD"), None, 2027);
    assert!(!beyond.is_complete());
    assert!(beyond.gaps().iter().any(|gap| gap.name == "Janmashtami"));
    assert_eq!(beyond.name_on(ymd(2027, 12, 16)), Some("Victory Day"));
}

#[test]
fn mongolia_keeps_the_gregorian_days_of_its_holidays_law() {
    expect(
        "MN",
        None,
        &[
            (2025, 1, 1, "New Year's Day"),
            (2025, 3, 8, "International Women's Day"),
            (2025, 6, 1, "Children's Day"),
            (2025, 7, 10, "Naadam"),
            (2025, 7, 15, "Naadam"),
            (2025, 11, 26, "Republic Day"),
            (2025, 12, 29, "National Freedom and Independence Day"),
            (2026, 1, 1, "New Year's Day"),
            (2026, 3, 8, "International Women's Day"),
            (2026, 6, 1, "Children's Day"),
            (2026, 7, 12, "Naadam"),
            (2026, 11, 26, "Republic Day"),
            (2026, 12, 29, "National Freedom and Independence Day"),
        ],
    );
    // Republic Day and 29 December before the laws that added them;
    // Constitution Day, 13 January, is a day of observance, not a day off.
    expect_working("MN", None, &[(2015, 11, 26), (2010, 12, 29), (2026, 1, 13)]);
}

#[test]
fn mongolia_reports_its_lunar_holidays_as_gaps_and_moves_nothing() {
    let calendar = HolidayCalendar::for_year(table("MN"), None, 2026);
    assert!(!calendar.is_complete());
    let missing: Vec<&str> = calendar.gaps().iter().map(|gap| gap.name).collect();
    for name in ["Tsagaan Sar", "Buddha's Birthday", "Chinggis Khaan Day"] {
        assert!(
            missing.contains(&name),
            "{name} should be a gap: {missing:?}"
        );
    }
    // Before the laws that added them, the two are not even gaps.
    let early = HolidayCalendar::for_year(table("MN"), None, 2011);
    let missing: Vec<&str> = early.gaps().iter().map(|gap| gap.name).collect();
    assert!(missing.contains(&"Tsagaan Sar"));
    assert!(!missing.contains(&"Buddha's Birthday"));
    assert!(!missing.contains(&"Chinggis Khaan Day"));
    // Children's Day 2024 fell on a Saturday and stays there.
    let calendar = HolidayCalendar::for_year(table("MN"), None, 2024);
    assert!(calendar.is_weekend(ymd(2024, 6, 1)));
    assert!(calendar.is_business_day(ymd(2024, 6, 3)));
    assert!(calendar.on(ymd(2024, 6, 3)).is_empty());
}

#[test]
fn cambodia_keeps_the_days_its_sub_decrees_give() {
    // Sub-decrees No. 204 (2025), No. 167 (2026) and No. 198 (2027).
    expect(
        "KH",
        None,
        &[
            (2025, 1, 7, "Victory over Genocide Day"),
            (2025, 4, 14, "Khmer New Year"),
            (2025, 5, 11, "Visak Bochea"),
            (2025, 5, 15, "Royal Ploughing Ceremony"),
            (2025, 9, 21, "Pchum Ben"),
            (2025, 9, 23, "Pchum Ben"),
            (2025, 11, 6, "Water Festival"),
            (2025, 12, 29, "Peace Day in Cambodia"),
            // In 2026 Visak Bochea is International Labour Day, one day off.
            (2026, 5, 1, "International Labour Day"),
            (2026, 5, 1, "Visak Bochea"),
            (2026, 5, 5, "Royal Ploughing Ceremony"),
            (2026, 5, 14, "King Norodom Sihamoni's Birthday"),
            (2026, 10, 10, "Pchum Ben"),
            (
                2026,
                10,
                15,
                "Commemoration Day of King Father Norodom Sihanouk",
            ),
            (2026, 11, 23, "Water Festival"),
            (2027, 4, 16, "Khmer New Year"),
            (2027, 5, 20, "Visak Bochea"),
            (2027, 5, 24, "Royal Ploughing Ceremony"),
            (2027, 10, 1, "Pchum Ben"),
            (2027, 10, 29, "King Norodom Sihamoni's Coronation Day"),
            (2027, 11, 14, "Water Festival"),
        ],
    );
    // The lunar days move: 2025's Pchum Ben and Water Festival are not
    // 2026's, and 13 April is not Khmer New Year in any year read.
    expect_working(
        "KH",
        None,
        &[(2026, 9, 22), (2026, 11, 5), (2025, 4, 13), (2027, 5, 11)],
    );
}

#[test]
fn cambodia_moves_nothing_off_its_sunday_weekend() {
    // Visak Bochea 2025 fell on Sunday 11 May and Independence Day on Sunday
    // 9 November; guideline No. 028/22 says a Sunday holiday is not moved.
    let calendar = HolidayCalendar::for_year(table("KH"), None, 2025);
    assert!(calendar.is_weekend(ymd(2025, 5, 11)));
    assert!(calendar.on(ymd(2025, 5, 12)).is_empty());
    assert!(calendar.on(ymd(2025, 11, 10)).is_empty());
    // Saturday is a working day under article 147 of the Labour Law.
    assert!(!calendar.is_weekend(ymd(2025, 3, 8)));
    assert!(calendar.is_business_day(ymd(2025, 3, 15)));
    assert!(calendar.on(ymd(2025, 3, 10)).is_empty());
}

#[test]
fn cambodia_reports_the_years_its_sub_decrees_do_not_cover_as_gaps() {
    assert!(HolidayCalendar::for_year(table("KH"), None, 2027).is_complete());
    let beyond = HolidayCalendar::for_year(table("KH"), None, 2028);
    assert!(!beyond.is_complete());
    let missing: Vec<&str> = beyond.gaps().iter().map(|gap| gap.name).collect();
    for name in [
        "Khmer New Year",
        "Visak Bochea",
        "Royal Ploughing Ceremony",
        "Pchum Ben",
        "Water Festival",
    ] {
        assert!(
            missing.contains(&name),
            "{name} should be a gap: {missing:?}"
        );
    }
    assert_eq!(beyond.name_on(ymd(2028, 11, 9)), Some("Independence Day"));
}

#[test]
fn laos_keeps_the_holidays_of_its_decree_and_notices() {
    expect(
        "LA",
        None,
        &[
            (2024, 1, 1, "International New Year's Day"),
            (2024, 4, 13, "Lao New Year"),
            (2024, 4, 16, "Lao New Year"),
            (2024, 12, 2, "National Day"),
            (2025, 4, 14, "Lao New Year"),
            (2025, 4, 16, "Lao New Year"),
            (2025, 5, 1, "International Labour Day"),
            (2026, 1, 1, "International New Year's Day"),
            (2026, 4, 14, "Lao New Year"),
            (2026, 4, 16, "Lao New Year"),
            (2026, 5, 1, "International Labour Day"),
            (2026, 12, 2, "National Day"),
        ],
    );
    // The Lao Women's Union's day, off for women alone, is not carried.
    expect_working("LA", None, &[(2026, 7, 20), (2025, 4, 13), (2026, 4, 13)]);
}

#[test]
fn laos_makes_up_a_weekend_holiday_on_the_next_working_day() {
    // The notices: Monday 10 March 2025 for Saturday 8 March, Monday 9 March
    // 2026 for Sunday 8 March, and 17 and 18 April 2024 for the Saturday and
    // Sunday of a New Year running to Tuesday 16 April.
    expect_substitute("LA", None, 2025, (3, 8), (3, 10));
    expect_substitute("LA", None, 2026, (3, 8), (3, 9));
    expect_substitute("LA", None, 2024, (4, 13), (4, 17));
    expect_substitute("LA", None, 2024, (4, 14), (4, 18));
    let calendar = HolidayCalendar::for_year(table("LA"), None, 2026);
    assert!(calendar.is_weekend(ymd(2026, 3, 7)));
    assert!(calendar.is_business_day(ymd(2026, 3, 10)));
}

#[test]
fn laos_reports_the_years_its_notices_do_not_cover_as_gaps() {
    assert!(HolidayCalendar::for_year(table("LA"), None, 2026).is_complete());
    let beyond = HolidayCalendar::for_year(table("LA"), None, 2027);
    assert!(!beyond.is_complete());
    assert!(beyond.gaps().iter().any(|gap| gap.name == "Lao New Year"));
    assert_eq!(beyond.name_on(ymd(2027, 12, 2)), Some("National Day"));
}

#[test]
fn brunei_keeps_the_days_of_the_prime_ministers_circulars() {
    expect(
        "BN",
        None,
        &[
            (2024, 2, 10, "Chinese New Year"),
            (2024, 6, 17, "Hari Raya Aidil Adha"),
            (2024, 7, 15, "Sultan's Birthday"),
            (2024, 9, 16, "Prophet Muhammad's Birthday"),
            (2025, 1, 1, "New Year's Day"),
            (2025, 1, 27, "Isra' and Mi'raj"),
            (2025, 1, 29, "Chinese New Year"),
            (2025, 3, 31, "Hari Raya Aidil Fitri"),
            (2025, 4, 2, "Hari Raya Aidil Fitri"),
            (2025, 5, 31, "Royal Brunei Armed Forces Day"),
            (2026, 2, 17, "Chinese New Year"),
            (2026, 2, 23, "National Day"),
            (2026, 5, 27, "Hari Raya Aidil Adha"),
            (2026, 6, 17, "Islamic New Year"),
            (2026, 12, 25, "Christmas Day"),
        ],
    );
    // The Hijri days are predictions: the circular's Awal Ramadhan 2026 is
    // Thursday 19 February, and the tabular calendar says the 18th.
    let calendar = HolidayCalendar::for_year(table("BN"), None, 2026);
    for holiday in calendar.on(ymd(2026, 2, 18)) {
        assert_eq!(holiday.confidence, Confidence::Approximate);
    }
    assert!(
        calendar
            .on(ymd(2026, 2, 17))
            .iter()
            .all(|holiday| holiday.confidence == Confidence::Exact)
    );
}

#[test]
fn brunei_replaces_a_friday_or_sunday_holiday_with_the_next_working_day() {
    // Every one of these is the circular's "sebagai ganti".
    expect_substitute("BN", None, 2023, (1, 22), (1, 23));
    expect_substitute("BN", None, 2023, (4, 23), (4, 25));
    expect_substitute("BN", None, 2024, (2, 23), (2, 24));
    expect_substitute("BN", None, 2024, (5, 31), (6, 1));
    expect_substitute("BN", None, 2025, (2, 23), (2, 24));
    expect_substitute("BN", None, 2025, (9, 5), (9, 6));
    expect_substitute("BN", None, 2026, (5, 31), (6, 1));
    expect_substitute("BN", None, 2026, (12, 25), (12, 26));
    // A Saturday holiday is not replaced: Armed Forces Day 2025.
    expect_working("BN", None, &[(2025, 6, 2)]);
    let calendar = HolidayCalendar::for_year(table("BN"), None, 2025);
    assert!(calendar.is_weekend(ymd(2025, 5, 30)));
    assert!(calendar.is_weekend(ymd(2025, 6, 1)));
    assert!(calendar.is_business_day(ymd(2025, 5, 24)));
}

#[test]
fn timor_leste_keeps_the_holidays_of_its_holidays_law() {
    expect(
        "TL",
        None,
        &[
            (2024, 3, 3, "Veterans' Day"),
            (2024, 3, 29, "Good Friday"),
            (2024, 4, 10, "Idul Fitri"),
            (2024, 5, 30, "Corpus Christi"),
            (2024, 6, 17, "Idul Adha"),
            (2025, 4, 18, "Good Friday"),
            (2025, 5, 20, "Restoration of Independence Day"),
            (2025, 6, 19, "Corpus Christi"),
            (2025, 8, 30, "Popular Consultation Day"),
            (2025, 11, 3, "National Women's Day"),
            (2026, 3, 20, "Idul Fitri"),
            (2026, 4, 3, "Good Friday"),
            (2026, 6, 4, "Corpus Christi"),
            (2026, 11, 28, "Proclamation of Independence Day"),
            (2026, 12, 7, "Memorial Day"),
            (2026, 12, 8, "Immaculate Conception"),
            (2026, 12, 31, "National Heroes' Day"),
        ],
    );
    // Before the amendments: 7 December was National Heroes' Day and 3 March,
    // 3 November and 31 December were not holidays.
    expect("TL", None, &[(2011, 12, 7, "National Heroes' Day")]);
    expect_working(
        "TL",
        None,
        &[(2011, 12, 31), (2016, 3, 3), (2022, 11, 3), (2026, 8, 20)],
    );
}

#[test]
fn timor_leste_moves_nothing_off_its_sunday_weekend() {
    // Popular Consultation Day 2026 and All Saints' Day are Sundays.
    let calendar = HolidayCalendar::for_year(table("TL"), None, 2026);
    assert!(calendar.is_weekend(ymd(2026, 8, 30)));
    assert!(calendar.on(ymd(2026, 8, 31)).is_empty());
    // Saturday 28 November 2026 is a holiday on a working day.
    assert!(!calendar.is_weekend(ymd(2026, 11, 28)));
    assert!(calendar.is_business_day(ymd(2026, 11, 21)));
    for holiday in calendar.on(ymd(2026, 3, 20)) {
        assert_eq!(holiday.confidence, Confidence::Approximate);
    }
}

#[test]
fn bhutan_keeps_the_days_of_the_ministry_of_home_affairs_lists() {
    expect(
        "BT",
        None,
        &[
            (2025, 1, 30, "Traditional Day of Offering"),
            (2025, 2, 28, "Losar"),
            (2025, 3, 1, "Losar"),
            (2025, 5, 7, "Death Anniversary of Zhabdrung"),
            (2025, 10, 2, "Dassain"),
            (2025, 11, 11, "Descending Day of Lord Buddha"),
            (2026, 1, 2, "Winter Solstice"),
            (2026, 2, 18, "Losar"),
            (2026, 2, 21, "Birth Anniversary of His Majesty the King"),
            (2026, 2, 23, "Birth Anniversary of His Majesty the King"),
            (2026, 5, 2, "Birth Anniversary of the Third Druk Gyalpo"),
            (2026, 5, 31, "Lord Buddha's Parinirvana"),
            (2026, 6, 24, "Birth Anniversary of Guru Rinpoche"),
            (2026, 7, 18, "First Sermon of Lord Buddha"),
            (2026, 9, 23, "Blessed Rainy Day"),
            (2026, 11, 1, "Coronation of His Majesty the King"),
            (2026, 11, 1, "Descending Day of Lord Buddha"),
            (
                2026,
                11,
                11,
                "Birth Anniversary of the Fourth Druk Gyalpo – Constitution Day",
            ),
            (2026, 12, 17, "National Day"),
        ],
    );
    // Thimphu Drubchoe (17 September 2026) is Thimphu's alone, and nothing
    // moves off the weekend: the King's birthday of Saturday 21 and Sunday
    // 22 February 2026 gives no Tuesday.
    expect_working("BT", None, &[(2026, 9, 17), (2026, 2, 24), (2026, 10, 2)]);
    let calendar = HolidayCalendar::for_year(table("BT"), None, 2026);
    assert!(calendar.is_weekend(ymd(2026, 2, 21)));
    assert!(calendar.is_business_day(ymd(2026, 2, 20)));
}

#[test]
fn bhutan_reports_the_years_its_lists_do_not_cover_as_gaps() {
    assert!(HolidayCalendar::for_year(table("BT"), None, 2025).is_complete());
    assert!(HolidayCalendar::for_year(table("BT"), None, 2026).is_complete());
    let beyond = HolidayCalendar::for_year(table("BT"), None, 2027);
    assert!(beyond.gaps().iter().any(|gap| gap.name == "Losar"));
    // The royal and national days are Gregorian and remain known.
    assert_eq!(beyond.name_on(ymd(2027, 12, 17)), Some("National Day"));
}

#[test]
fn the_maldives_keeps_the_days_of_section_97_as_the_monetary_authority_lists_them() {
    expect(
        "MV",
        None,
        &[
            (2025, 1, 1, "New Year's Day"),
            (2025, 5, 1, "Labour Day"),
            (2025, 7, 26, "Independence Day"),
            (2025, 7, 27, "Independence Day"),
            (2025, 11, 3, "Victory Day"),
            (2026, 11, 11, "Republic Day"),
            // The tabular Hijri calendar agrees with the Authority's 2026
            // list on these.
            (2026, 2, 18, "First Day of Ramadan"),
            (2026, 3, 20, "Eid al-Fitr"),
            (2026, 3, 22, "Eid al-Fitr"),
            (2026, 5, 26, "Hajj Day"),
            (2026, 5, 27, "Eid al-Adha"),
            (2026, 5, 30, "Eid al-Adha"),
        ],
    );
    // The Hijri days are predictions: the list's National Day 2026 is
    // Friday 14 August, and the tabular calendar says the 15th.
    let calendar = HolidayCalendar::for_year(table("MV"), None, 2026);
    let national_day = calendar
        .in_year(2026)
        .into_iter()
        .find(|holiday| holiday.name == "National Day")
        .unwrap_or_else(|| panic!("MV 2026: no National Day"));
    assert_eq!(national_day.confidence, Confidence::Approximate);
    assert!(
        calendar
            .on(ymd(2026, 7, 26))
            .iter()
            .all(|holiday| holiday.confidence == Confidence::Exact)
    );
    // Four days of Eid al-Adha from the 2017 list, three in 2016's.
    let count = |year: i64| {
        HolidayCalendar::for_year(table("MV"), None, year)
            .in_year(year)
            .iter()
            .filter(|holiday| holiday.name == "Eid al-Adha")
            .count()
    };
    assert_eq!(count(2016), 3);
    assert_eq!(count(2017), 4);
}

#[test]
fn the_maldives_moves_nothing_off_its_friday_and_saturday_weekend() {
    // Eid al-Fitr 2026 is Friday to Sunday, and no day follows it.
    expect_working("MV", None, &[(2026, 3, 23), (2025, 7, 28), (2025, 11, 4)]);
    let calendar = HolidayCalendar::for_year(table("MV"), None, 2026);
    assert!(calendar.is_weekend(ymd(2026, 3, 20)));
    assert!(calendar.is_weekend(ymd(2026, 3, 21)));
    assert!(calendar.is_business_day(ymd(2026, 3, 29)));
}

// ─────────────────────────────────────────────────────────────────────────
// The Middle East and Africa
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn israel_festival_days() {
    expect(
        "IL",
        None,
        &[
            (2024, 4, 23, "Passover"),
            (2024, 6, 12, "Shavuot"),
            (2024, 10, 3, "Rosh Hashanah"),
            (2024, 10, 12, "Yom Kippur"),
            (2025, 4, 13, "Passover"),
            (2025, 9, 23, "Rosh Hashanah"),
            (2025, 10, 2, "Yom Kippur"),
        ],
    );
}

#[test]
fn israeli_independence_day_moves_away_from_the_sabbath() {
    // 5 Iyar 5784 was 13 May 2024, a Monday, so Yom HaAtzmaut moved to the
    // Tuesday; 5 Iyar 5785 was 3 May 2025, a Saturday, so it moved back to
    // the Thursday.
    expect(
        "IL",
        None,
        &[
            (2024, 5, 14, "Yom HaAtzmaut"),
            (2025, 5, 1, "Yom HaAtzmaut"),
        ],
    );
    expect_working("IL", None, &[(2025, 5, 3), (2024, 5, 13)]);
}

#[test]
fn israel_rests_on_the_sabbath_and_works_on_friday() {
    // The Hours of Work and Rest Law makes the Sabbath the weekly rest and
    // Friday a shortened working day.
    let calendar = HolidayCalendar::for_year(table("IL"), None, 2025);
    assert!(calendar.is_business_day(ymd(2025, 3, 7)));
    assert!(calendar.is_weekend(ymd(2025, 3, 8)));
    assert!(calendar.is_business_day(ymd(2025, 3, 9)));
}

#[test]
fn iran_dates_its_civil_holidays_in_the_solar_hijri_calendar_as_kept() {
    // 1403 began on 20 March 2024 and 1404 on 21 March 2025 — the year
    // Birashk's cycle would have started a day early.
    expect(
        "IR",
        None,
        &[
            (2024, 3, 20, "Nowruz"),
            (2024, 3, 23, "Nowruz"),
            (2024, 3, 31, "Islamic Republic Day"),
            (2024, 4, 1, "Nature Day (Sizdah Bedar)"),
            (2024, 6, 3, "Demise of Imam Khomeini"),
            (2024, 6, 4, "15 Khordad Uprising"),
            (2025, 2, 10, "Victory of the Islamic Revolution"),
            // 1403 was a leap year, so 29 Esfand is the 19th and 30 Esfand the 20th.
            (2025, 3, 19, "Nationalisation of the Oil Industry"),
            (2025, 3, 21, "Nowruz"),
            (2025, 3, 24, "Nowruz"),
            (2025, 4, 1, "Islamic Republic Day"),
            (2025, 4, 2, "Nature Day (Sizdah Bedar)"),
            (2025, 6, 4, "Demise of Imam Khomeini"),
            (2025, 6, 5, "15 Khordad Uprising"),
            (2026, 2, 11, "Victory of the Islamic Revolution"),
            (2026, 3, 20, "Nationalisation of the Oil Industry"),
            (2026, 3, 21, "Nowruz"),
        ],
    );
    // 20 March 2025 is 30 Esfand 1403, a leap day, and not a holiday.
    let calendar = HolidayCalendar::for_year(table("IR"), None, 2025);
    assert!(calendar.on(ymd(2025, 3, 20)).is_empty());
}

#[test]
fn iran_flags_every_lunar_date_and_keeps_a_friday_weekend() {
    let calendar = HolidayCalendar::for_year(table("IR"), None, 2025);
    let mut lunar = 0;
    for holiday in calendar.all() {
        let exact = matches!(
            holiday.name,
            "Nowruz"
                | "Islamic Republic Day"
                | "Nature Day (Sizdah Bedar)"
                | "Demise of Imam Khomeini"
                | "15 Khordad Uprising"
                | "Victory of the Islamic Revolution"
                | "Nationalisation of the Oil Industry"
        );
        let expected = if exact {
            hc_holiday::rule::Confidence::Exact
        } else {
            lunar += 1;
            hc_holiday::rule::Confidence::Approximate
        };
        assert_eq!(holiday.confidence, expected, "{}", holiday.name);
    }
    // Seventeen lunar entries a year, plus whichever of them the Hijri year
    // repeats inside a Gregorian one.
    assert!(lunar >= 17, "{lunar} lunar entries");
    // 2025-03-14 was a Friday; the Thursday before and the Saturday after
    // are working days.
    assert!(!calendar.is_weekend(ymd(2025, 3, 13)));
    assert!(calendar.is_weekend(ymd(2025, 3, 14)));
    assert!(!calendar.is_weekend(ymd(2025, 3, 15)));
}

#[test]
fn argentina_moves_its_trasladables_by_the_weekday_rule_since_2018() {
    // 2026: Güemes on a Wednesday pulled to Monday the 15th, Sovereignty
    // Day on a Friday pushed to Monday the 23rd, San Martín and 12 October
    // already Mondays, Belgrano and 2 April inamovible where they fall.
    expect(
        "AR",
        None,
        &[
            (2026, 2, 16, "Carnival Monday"),
            (2026, 2, 17, "Carnival Tuesday"),
            (2026, 3, 24, "Day of Remembrance for Truth and Justice"),
            (
                2026,
                4,
                2,
                "Day of the Veterans and Fallen of the Malvinas War",
            ),
            (2026, 4, 3, "Good Friday"),
            (2026, 6, 15, "Anniversary of the Passing of General Güemes"),
            (
                2026,
                6,
                20,
                "Anniversary of the Passing of General Belgrano",
            ),
            (2026, 7, 9, "Independence Day"),
            (
                2026,
                8,
                17,
                "Anniversary of the Passing of General San Martín",
            ),
            (2026, 10, 12, "Day of Respect for Cultural Diversity"),
            (2026, 11, 23, "National Sovereignty Day"),
            (2026, 12, 8, "Immaculate Conception"),
            // 2025: a Tuesday pulled back, two Sundays left alone, a Thursday
            // pushed on.
            (2025, 6, 16, "Anniversary of the Passing of General Güemes"),
            (
                2025,
                8,
                17,
                "Anniversary of the Passing of General San Martín",
            ),
            (2025, 10, 12, "Day of Respect for Cultural Diversity"),
            (2025, 11, 24, "National Sovereignty Day"),
            // 2016: the decree's Mondays — the third of August, the second
            // of October, the fourth of November.
            (
                2016,
                8,
                15,
                "Anniversary of the Passing of General San Martín",
            ),
            (2016, 10, 10, "Day of Respect for Cultural Diversity"),
            (2016, 11, 28, "National Sovereignty Day"),
        ],
    );
    expect_working(
        "AR",
        None,
        &[(2026, 6, 17), (2026, 11, 20), (2016, 8, 17), (2025, 11, 20)],
    );
    // Holy Thursday and the days of the faiths are observances, not days
    // off: 2 April 2026 is Malvinas Day, Holy Thursday and the first day of
    // Passover at once, and only the first is a day off.
    let calendar = HolidayCalendar::for_year(table("AR"), None, 2026);
    let on_the_day = calendar.on(ymd(2026, 4, 2));
    let days_off: Vec<&str> = on_the_day
        .iter()
        .filter(|holiday| holiday.is_day_off())
        .map(|holiday| holiday.name)
        .collect();
    let observances: Vec<&str> = on_the_day
        .iter()
        .filter(|holiday| !holiday.is_day_off())
        .map(|holiday| holiday.name)
        .collect();
    assert_eq!(
        days_off,
        ["Day of the Veterans and Fallen of the Malvinas War"]
    );
    assert!(observances.contains(&"Holy Thursday"), "{observances:?}");
    assert!(observances.contains(&"Passover"), "{observances:?}");
}

#[test]
fn colombia_sends_ten_holidays_to_the_following_monday() {
    // The 2026 calendar: Epiphany from Tuesday the 6th, Saint Joseph from
    // Thursday the 19th, Ascension from Thursday 14 May, Corpus Christi
    // from Thursday 4 June, the Sacred Heart from Friday 12 June, the
    // Assumption from Saturday the 15th, All Saints from Sunday the 1st,
    // Cartagena from Wednesday the 11th; Saints Peter and Paul and
    // 12 October already Mondays.
    expect(
        "CO",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 12, "Epiphany"),
            (2026, 3, 23, "Saint Joseph's Day"),
            (2026, 4, 2, "Maundy Thursday"),
            (2026, 4, 3, "Good Friday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 18, "Ascension Day"),
            (2026, 6, 8, "Corpus Christi"),
            (2026, 6, 15, "Sacred Heart"),
            (2026, 6, 29, "Saints Peter and Paul"),
            (2026, 7, 20, "Independence Day"),
            (2026, 8, 7, "Battle of Boyacá"),
            (2026, 8, 17, "Assumption of Mary"),
            (2026, 10, 12, "Columbus Day"),
            (2026, 11, 2, "All Saints' Day"),
            (2026, 11, 16, "Independence of Cartagena"),
            (2026, 12, 8, "Immaculate Conception"),
            (2026, 12, 25, "Christmas Day"),
            // A fixed day on a weekend stays: 20 July 2025 was a Sunday.
            (2025, 7, 20, "Independence Day"),
        ],
    );
    expect_working(
        "CO",
        None,
        &[(2026, 1, 6), (2026, 8, 15), (2026, 11, 1), (2026, 11, 11)],
    );
}

#[test]
fn kenya_moves_a_sunday_holiday_to_the_next_free_day() {
    expect(
        "KE",
        None,
        &[
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 6, 1, "Madaraka Day"),
            (2026, 10, 10, "Mazingira Day"),
            (2026, 10, 20, "Mashujaa Day"),
            (2026, 12, 12, "Jamhuri Day"),
            (2026, 12, 26, "Boxing Day"),
            // 1 June 2025 was a Sunday.
            (2025, 6, 2, "Madaraka Day"),
        ],
    );
    expect_substitute("KE", None, 2025, (6, 1), (6, 2));
    // Idd-ul-Azha and Diwali are for the faithful, not days off for all.
    let calendar = HolidayCalendar::for_year(table("KE"), None, 2026);
    let religious: Vec<&str> = (1..=365)
        .flat_map(|day| calendar.on(Rd(ymd(2026, 1, 1).0 + day - 1)))
        .filter(|holiday| !holiday.is_day_off())
        .map(|holiday| holiday.name)
        .collect();
    assert!(religious.contains(&"Idd-ul-Azha"), "{religious:?}");
    assert!(religious.contains(&"Diwali"), "{religious:?}");
}

#[test]
fn morocco_keeps_its_fixed_days_and_the_two_recent_additions_by_year() {
    expect(
        "MA",
        None,
        &[
            (2026, 1, 11, "Proclamation of Independence Day"),
            (2026, 1, 14, "Amazigh New Year"),
            (2026, 7, 30, "Throne Day"),
            (2026, 8, 14, "Oued Ed-Dahab Allegiance Day"),
            (2026, 8, 20, "Revolution of the King and the People"),
            (2026, 8, 21, "Youth Day"),
            (2026, 10, 31, "Unity Day"),
            (2026, 11, 6, "Green March Day"),
            (2026, 11, 18, "Independence Day"),
        ],
    );
    expect_working("MA", None, &[(2023, 1, 14), (2025, 10, 31)]);
    // Each of the three great feasts is two days.
    let eid = table("MA")
        .rules
        .iter()
        .filter(|rule| rule.name == "Eid al-Fitr")
        .count();
    assert_eq!(eid, 2);
}

#[test]
fn pakistan_carries_iqbal_day_only_in_the_years_it_was_a_holiday() {
    expect(
        "PK",
        None,
        &[
            (2026, 2, 5, "Kashmir Day"),
            (2026, 3, 23, "Pakistan Day"),
            (2026, 5, 28, "Youm-e-Takbeer"),
            (2026, 8, 14, "Independence Day"),
            (2026, 11, 9, "Iqbal Day"),
            (2026, 12, 25, "Quaid-e-Azam Day"),
            (2014, 11, 9, "Iqbal Day"),
        ],
    );
    expect_working("PK", None, &[(2018, 11, 9), (2023, 5, 28)]);
}

#[test]
fn peru_keeps_its_sixteen_days_where_they_fall() {
    expect(
        "PE",
        None,
        &[
            (2026, 4, 2, "Maundy Thursday"),
            (2026, 4, 3, "Good Friday"),
            (2026, 6, 7, "Flag Day"),
            (2026, 6, 29, "Saints Peter and Paul"),
            (2026, 7, 23, "Air Force Day"),
            (2026, 7, 28, "Independence Day"),
            (2026, 7, 29, "Independence Day"),
            (2026, 8, 6, "Battle of Junín"),
            (2026, 8, 30, "Saint Rose of Lima"),
            (2026, 10, 8, "Battle of Angamos"),
            (2026, 11, 1, "All Saints' Day"),
            (2026, 12, 8, "Immaculate Conception"),
            (2026, 12, 9, "Battle of Ayacucho"),
        ],
    );
    // 7 June 2026 is a Sunday and stays one; nothing before 2024 is stated.
    expect_working("PE", None, &[(2026, 6, 8), (2023, 6, 7)]);
}

#[test]
fn myanmar_dates_its_full_moons_and_thingyan_on_the_burmese_calendar() {
    // 1386 ME: Thingyan from 13 to 16 April 2024 and the New Year on the
    // 17th; the full moons of Kason, Waso, Thadingyut and Tazaungmon on
    // 22 May, 20 July, 17 October and 15 November; National Day ten days
    // after the last, 25 November.
    expect(
        "MM",
        None,
        &[
            (2024, 1, 4, "Independence Day"),
            (2024, 2, 12, "Union Day"),
            (2024, 3, 27, "Armed Forces Day"),
            (2024, 4, 13, "Thingyan Eve"),
            (2024, 4, 14, "Thingyan Akya Day"),
            (2024, 4, 15, "Thingyan Akyat Day"),
            (2024, 4, 16, "Thingyan Atat Day"),
            (2024, 4, 17, "Myanmar New Year's Day"),
            (2024, 5, 22, "Full Moon Day of Kason"),
            (2024, 7, 19, "Martyrs' Day"),
            (2024, 7, 20, "Full Moon Day of Waso"),
            (2024, 10, 16, "Thadingyut Holiday"),
            (2024, 10, 17, "Full Moon Day of Thadingyut"),
            (2024, 10, 18, "Thadingyut Holiday"),
            (2024, 11, 14, "Tazaungdaing Holiday"),
            (2024, 11, 15, "Full Moon Day of Tazaungmon"),
            (2024, 11, 25, "National Day"),
            (2024, 12, 25, "Christmas Day"),
        ],
    );
    expect_working("MM", None, &[(2024, 4, 12), (2024, 4, 18)]);
}

#[test]
fn ethiopia_keeps_its_days_on_the_ethiopian_calendar() {
    // 2026: the Gregorian dates the source prints; Orthodox Easter on
    // 12 April. 2024, a Gregorian leap year: Genna and Timkat a day later;
    // 2027, before a Gregorian leap year: Enkutatash and Meskel a day
    // later.
    expect(
        "ET",
        None,
        &[
            (2026, 1, 7, "Genna"),
            (2026, 1, 19, "Timkat"),
            (2026, 3, 2, "Adwa Victory Day"),
            (2026, 4, 10, "Good Friday"),
            (2026, 4, 12, "Fasika"),
            (2026, 5, 5, "Patriots' Victory Day"),
            (2026, 5, 28, "Downfall of the Derg"),
            (2026, 9, 11, "Enkutatash"),
            (2026, 9, 27, "Meskel"),
            (2024, 1, 8, "Genna"),
            (2024, 1, 20, "Timkat"),
            (2027, 9, 12, "Enkutatash"),
            (2027, 9, 28, "Meskel"),
        ],
    );
    expect_working("ET", None, &[(2024, 1, 7), (2027, 9, 11)]);
}

#[test]
fn ghana_carries_the_2019_and_2025_arrangements_by_year() {
    expect(
        "GH",
        None,
        &[
            (2026, 1, 7, "Constitution Day"),
            (2026, 3, 6, "Independence Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 7, 1, "Republic Day"),
            (2026, 9, 21, "Founders' Day"),
            (2026, 12, 4, "Farmers' Day"),
            (2026, 12, 26, "Boxing Day"),
            (2024, 8, 4, "Founders' Day"),
            (2024, 9, 21, "Kwame Nkrumah Memorial Day"),
            (2019, 1, 7, "Constitution Day"),
        ],
    );
    expect_working("GH", None, &[(2024, 7, 1), (2026, 8, 4), (2018, 1, 7)]);
    // Shaqq Day follows Eid al-Fitr from 2026 and not before.
    let names_in = |year: i64| -> Vec<&'static str> {
        let calendar = HolidayCalendar::for_year(table("GH"), None, year);
        (1..=365)
            .flat_map(|day| calendar.on(Rd(ymd(year, 1, 1).0 + day - 1)))
            .map(|holiday| holiday.name)
            .collect()
    };
    assert!(names_in(2026).contains(&"Shaqq Day"));
    assert!(!names_in(2025).contains(&"Shaqq Day"));
}

#[test]
fn croatia_moved_statehood_day_twice_and_demoted_two_days_in_2020() {
    expect(
        "HR",
        None,
        &[
            (2026, 1, 6, "Epiphany"),
            (2026, 4, 5, "Easter Sunday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 30, "Statehood Day"),
            (2026, 6, 4, "Corpus Christi"),
            (2026, 6, 22, "Anti-Fascist Struggle Day"),
            (2026, 8, 5, "Victory and Homeland Thanksgiving Day"),
            (
                2026,
                11,
                18,
                "Remembrance Day for the Victims of the Homeland War",
            ),
            (2026, 12, 26, "Saint Stephen's Day"),
            (2019, 6, 25, "Statehood Day"),
            (2019, 10, 8, "Independence Day"),
            (1995, 5, 30, "Statehood Day"),
        ],
    );
    expect_working(
        "HR",
        None,
        &[(2019, 5, 30), (2019, 11, 18), (2026, 6, 25), (2026, 10, 8)],
    );
}

#[test]
fn slovakia_keeps_its_state_holidays_as_observances_once_they_stop_being_days_off() {
    expect(
        "SK",
        None,
        &[
            (2026, 1, 6, "Epiphany"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 7, 5, "Saints Cyril and Methodius Day"),
            (2026, 8, 29, "Slovak National Uprising Anniversary"),
            (2026, 11, 1, "All Saints' Day"),
            (2026, 12, 24, "Christmas Eve"),
            (2025, 5, 8, "Day of Victory over Fascism"),
            (2025, 9, 15, "Our Lady of the Seven Sorrows"),
            (2023, 9, 1, "Constitution Day"),
            (2024, 11, 17, "Struggle for Freedom and Democracy Day"),
        ],
    );
    expect_working(
        "SK",
        None,
        &[
            (2026, 5, 8),
            (2024, 9, 1),
            (2026, 9, 15),
            (2025, 11, 17),
            (2026, 10, 28),
        ],
    );
    let calendar = HolidayCalendar::for_year(table("SK"), None, 2026);
    let observed: Vec<&str> = calendar
        .on(ymd(2026, 10, 28))
        .iter()
        .map(|holiday| holiday.name)
        .collect();
    assert_eq!(
        observed,
        ["Day of the Establishment of an Independent Czecho-Slovak State"]
    );
}

#[test]
fn slovenia_took_2_january_back_in_2017() {
    // Easter 2026 on 5 April, Whit Sunday on 24 May.
    expect(
        "SI",
        None,
        &[
            (2026, 1, 2, "New Year's Day"),
            (2026, 2, 8, "Prešeren Day"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 4, 27, "Day of Uprising Against Occupation"),
            (2026, 5, 2, "May Day"),
            (2026, 5, 24, "Whit Sunday"),
            (2026, 6, 25, "Statehood Day"),
            (2026, 8, 15, "Assumption Day"),
            (2026, 10, 31, "Reformation Day"),
            (2026, 12, 26, "Independence and Unity Day"),
            (2012, 1, 2, "New Year's Day"),
        ],
    );
    expect_working("SI", None, &[(2014, 1, 2), (2016, 1, 2)]);
}

#[test]
fn iceland_first_day_of_summer_is_the_first_thursday_after_18_april() {
    // Easter 2026 on 5 April; the First Day of Summer on 23 April 2026,
    // 24 April 2025 and 20 April 2028 as its own page gives them.
    expect(
        "IS",
        None,
        &[
            (2026, 4, 2, "Maundy Thursday"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 4, 23, "First Day of Summer"),
            (2025, 4, 24, "First Day of Summer"),
            (2028, 4, 20, "First Day of Summer"),
            (2026, 5, 14, "Ascension Day"),
            (2026, 5, 24, "Whit Sunday"),
            (2026, 5, 25, "Whit Monday"),
            (2026, 6, 17, "National Day"),
            (2026, 8, 3, "Commerce Day"),
            (2026, 12, 26, "Second Day of Christmas"),
        ],
    );
    let calendar = HolidayCalendar::for_year(table("IS"), None, 2026);
    for (month, day) in [(12, 24), (12, 31)] {
        let kinds: Vec<Kind> = calendar
            .on(ymd(2026, month, day))
            .iter()
            .map(|holiday| holiday.kind)
            .collect();
        assert_eq!(kinds, [Kind::Bank], "{month}-{day}");
    }
}

#[test]
fn bulgaria_moves_weekend_holidays_forward_from_2017_but_never_easter() {
    // Orthodox Easter 2026 on 12 April.
    expect(
        "BG",
        None,
        &[
            (2026, 3, 3, "Liberation Day"),
            (2026, 4, 10, "Good Friday"),
            (2026, 4, 11, "Holy Saturday"),
            (2026, 4, 12, "Easter Sunday"),
            (2026, 4, 13, "Easter Monday"),
            (
                2026,
                5,
                6,
                "Saint George's Day, Day of Valour and of the Bulgarian Army",
            ),
            (2026, 9, 22, "Independence Day"),
            (2026, 12, 24, "Christmas Eve"),
        ],
    );
    // 24 May and 6 September 2026 are Sundays, 26 December a Saturday.
    expect_substitute("BG", None, 2026, (5, 24), (5, 25));
    expect_substitute("BG", None, 2026, (9, 6), (9, 7));
    expect_substitute("BG", None, 2026, (12, 26), (12, 28));
    // Holy Saturday and Easter Sunday move nothing. The rule's first use was
    // 1 January 2017, a Sunday; Christmas 2016, on a weekend, moved nothing.
    expect_substitute("BG", None, 2017, (1, 1), (1, 2));
    expect_working(
        "BG",
        None,
        &[(2026, 4, 14), (2026, 4, 15), (2016, 12, 27), (2016, 12, 28)],
    );
    let calendar = HolidayCalendar::for_year(table("BG"), None, 2026);
    let kinds: Vec<Kind> = calendar
        .on(ymd(2026, 11, 1))
        .iter()
        .map(|holiday| holiday.kind)
        .collect();
    assert_eq!(kinds, [Kind::School]);
}

#[test]
fn cyprus_keeps_easter_tuesday_for_the_banks_and_moves_nothing() {
    expect(
        "CY",
        None,
        &[
            (2026, 1, 6, "Epiphany"),
            (2026, 2, 23, "Green Monday"),
            (2026, 3, 25, "Greek Independence Day"),
            (2026, 4, 1, "Cyprus National Day"),
            (2026, 4, 10, "Good Friday"),
            (2026, 4, 11, "Holy Saturday"),
            (2026, 4, 12, "Easter Sunday"),
            (2026, 4, 13, "Easter Monday"),
            (2026, 4, 14, "Easter Tuesday"),
            (2026, 6, 1, "Pentecost Monday"),
            (2026, 8, 15, "Dormition of the Theotokos"),
            (2026, 10, 1, "Cyprus Independence Day"),
            (2026, 10, 28, "Greek National Day"),
        ],
    );
    // 15 August and 26 December 2026 are Saturdays and stay there.
    expect_working("CY", None, &[(2026, 8, 17), (2026, 12, 28)]);
    let calendar = HolidayCalendar::for_year(table("CY"), None, 2026);
    let kinds: Vec<Kind> = calendar
        .on(ymd(2026, 4, 14))
        .iter()
        .map(|holiday| holiday.kind)
        .collect();
    assert_eq!(kinds, [Kind::Bank]);
}

#[test]
fn estonia_dates_its_days_off_by_the_act_and_carries_the_flag_days_as_observances() {
    expect(
        "EE",
        None,
        &[
            (2026, 2, 24, "Independence Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 5, "Easter Sunday"),
            (2026, 5, 24, "Whit Sunday"),
            (2026, 6, 23, "Victory Day"),
            (2026, 6, 24, "Midsummer Day"),
            (2026, 8, 20, "Day of Restoration of Independence"),
            (2026, 12, 24, "Christmas Eve"),
            (1998, 8, 20, "Day of Restoration of Independence"),
            (2005, 12, 24, "Christmas Eve"),
        ],
    );
    expect_working("EE", None, &[(2004, 12, 24), (1997, 8, 20)]);
    let calendar = HolidayCalendar::for_year(table("EE"), None, 2026);
    for (month, day, name) in [
        (3, 14, "Mother Tongue Day"),
        (10, 17, "Finno-Ugric Day"),
        (11, 8, "Father's Day"),
    ] {
        let found: Vec<(&str, Kind)> = calendar
            .on(ymd(2026, month, day))
            .iter()
            .map(|holiday| (holiday.name, holiday.kind))
            .collect();
        assert_eq!(found, [(name, Kind::Observance)], "{month}-{day}");
    }
}

#[test]
fn latvia_moves_three_holidays_off_the_weekend_and_no_others() {
    expect(
        "LV",
        None,
        &[
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 4, "Restoration of Independence Day"),
            (2026, 5, 10, "Mother's Day"),
            (2026, 5, 24, "Pentecost"),
            (2026, 6, 23, "Līgo Day"),
            (2026, 6, 24, "Midsummer Day"),
            (2026, 11, 18, "Proclamation Day of the Republic of Latvia"),
            (2026, 12, 31, "New Year's Eve"),
            (
                2023,
                7,
                9,
                "Closing Day of the Nationwide Latvian Song and Dance Celebration",
            ),
            (2018, 9, 24, "Pastoral Visit of Pope Francis to Latvia"),
            (
                2023,
                5,
                29,
                "Bronze Medal of the Latvian Ice Hockey Team at the 2023 World Championship",
            ),
        ],
    );
    // 4 May 2025 a Sunday, 4 May 2024 a Saturday, 18 November 2023 a
    // Saturday, and the celebration closed on Sundays in 2018 and 2023.
    expect_substitute("LV", None, 2025, (5, 4), (5, 5));
    expect_substitute("LV", None, 2024, (5, 4), (5, 6));
    expect_substitute("LV", None, 2023, (11, 18), (11, 20));
    expect_substitute("LV", None, 2018, (7, 8), (7, 9));
    expect_substitute("LV", None, 2023, (7, 9), (7, 10));
    // Christmas 2027 and New Year 2028 fall on weekends and stay there; the
    // papal visit was 2018 alone, and 2013's closing day was no holiday.
    expect_working(
        "LV",
        None,
        &[(2027, 12, 27), (2028, 1, 3), (2019, 9, 24), (2013, 7, 8)],
    );
    let calendar = HolidayCalendar::for_year(table("LV"), None, 2026);
    for (month, day, name) in [
        (7, 11, "Sea Festival Day"),
        (9, 13, "Father's Day"),
        (11, 11, "Lāčplēsis Day"),
    ] {
        let found: Vec<(&str, Kind)> = calendar
            .on(ymd(2026, month, day))
            .iter()
            .map(|holiday| (holiday.name, holiday.kind))
            .collect();
        assert_eq!(found, [(name, Kind::Observance)], "{month}-{day}");
    }
}

#[test]
fn lithuania_dates_each_day_off_and_keeps_weekend_holidays_where_they_fall() {
    expect(
        "LT",
        None,
        &[
            (2026, 2, 16, "Day of Restoration of the State of Lithuania"),
            (
                2026,
                3,
                11,
                "Day of Restoration of Independence of Lithuania",
            ),
            (2026, 4, 5, "Easter Sunday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 3, "Mother's Day"),
            (2026, 6, 7, "Father's Day"),
            (2026, 6, 24, "Midsummer Day"),
            (2026, 7, 6, "Statehood Day"),
            (2026, 8, 15, "Assumption Day"),
            (2026, 11, 2, "All Souls' Day"),
            (2026, 12, 24, "Christmas Eve"),
            (2011, 12, 24, "Christmas Eve"),
        ],
    );
    // 26 December 2026 is a Saturday and stays there.
    expect_working(
        "LT",
        None,
        &[
            (2026, 12, 28),
            (2010, 12, 24),
            (2019, 11, 2),
            (2002, 6, 24),
            (2008, 6, 1),
        ],
    );
}

#[test]
fn hong_kong_makes_up_sundays_and_coincidences_on_the_next_free_day() {
    // The gazetted lists for 2026 and 2027.
    expect(
        "HK",
        None,
        &[
            (2026, 2, 17, "Lunar New Year's Day"),
            (2026, 2, 18, "The second day of Lunar New Year"),
            (2026, 2, 19, "The third day of Lunar New Year"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 4, "The day following Good Friday"),
            (2026, 4, 5, "Ching Ming Festival"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 24, "The Birthday of the Buddha"),
            (2026, 6, 19, "Tuen Ng Festival"),
            (
                2026,
                7,
                1,
                "Hong Kong Special Administrative Region Establishment Day",
            ),
            (
                2026,
                9,
                26,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (2026, 10, 18, "Chung Yeung Festival"),
            (2026, 12, 26, "The first weekday after Christmas Day"),
            (2027, 2, 6, "Lunar New Year's Day"),
            (2027, 4, 5, "Ching Ming Festival"),
            (2027, 5, 13, "The Birthday of the Buddha"),
            (2027, 6, 9, "Tuen Ng Festival"),
            (
                2027,
                9,
                16,
                "The day following the Chinese Mid-Autumn Festival",
            ),
            (2027, 10, 8, "Chung Yeung Festival"),
            (2027, 12, 27, "The first weekday after Christmas Day"),
            (2022, 12, 26, "The first weekday after Christmas Day"),
        ],
    );
    // 2026: Ching Ming on Easter Sunday is made up past Easter Monday, the
    // Buddha and Chung Yeung fall on Sundays, and Saturdays stay put.
    expect_substitute("HK", None, 2026, (4, 5), (4, 7));
    expect_substitute("HK", None, 2026, (5, 24), (5, 25));
    expect_substitute("HK", None, 2026, (10, 18), (10, 19));
    expect_working(
        "HK",
        None,
        &[(2026, 9, 28), (2026, 12, 28), (2027, 12, 28), (2022, 1, 3)],
    );
    // 2027: the second day of Lunar New Year on a Sunday yields the fourth.
    expect_substitute("HK", None, 2027, (2, 7), (2, 9));
    // 2022: Labour Day, the Buddha and the day following Mid-Autumn on
    // Sundays, and Christmas on a Sunday pushed past the first weekday after.
    expect_substitute("HK", None, 2022, (5, 1), (5, 2));
    expect_substitute("HK", None, 2022, (5, 8), (5, 9));
    expect_substitute("HK", None, 2022, (9, 11), (9, 12));
    expect_substitute("HK", None, 2022, (12, 25), (12, 27));
}

#[test]
fn hong_kong_made_up_lunar_new_year_and_mid_autumn_on_their_eves_from_1983_to_2011() {
    // 18 February 2007 and 14 February 2010, Sundays, gave the Saturday
    // before; 4 October 2009 gave the Mid-Autumn Festival day itself; and
    // 10 February 2013, the first case after the amendment, gave the
    // fourth day.
    expect(
        "HK",
        None,
        &[
            (2007, 2, 17, "Lunar New Year's Eve"),
            (2010, 2, 13, "Lunar New Year's Eve"),
            (2009, 10, 3, "Chinese Mid-Autumn Festival"),
        ],
    );
    expect_working("HK", None, &[(2007, 2, 21), (2010, 2, 17), (2013, 2, 9)]);
    expect_substitute("HK", None, 2013, (2, 10), (2, 13));
}

#[test]
fn hong_kong_phases_the_general_holidays_into_the_statutory_list() {
    for (year, month, day, kind) in [
        (2026, 4, 3, Kind::Bank),
        (2028, 4, 14, Kind::Public),
        (2025, 4, 21, Kind::Bank),
        (2026, 4, 6, Kind::Public),
        (2021, 5, 19, Kind::Bank),
        (2022, 5, 8, Kind::Public),
        (2023, 12, 26, Kind::Bank),
        (2024, 12, 26, Kind::Public),
    ] {
        let calendar = HolidayCalendar::for_year(table("HK"), None, year);
        let kinds: Vec<Kind> = calendar
            .on(ymd(year, month, day))
            .iter()
            .map(|holiday| holiday.kind)
            .collect();
        assert_eq!(kinds, [kind], "{year}-{month}-{day}");
    }
}

#[test]
fn macau_gives_the_public_administration_compensatory_rest_days_from_2019() {
    expect(
        "MO",
        None,
        &[
            (2026, 2, 17, "Lunar New Year's Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 4, "The day before Easter"),
            (2026, 4, 5, "Cheng Ming Festival"),
            (2026, 5, 24, "The Buddha's Birthday"),
            (2026, 6, 19, "Tung Ng Festival"),
            (2026, 9, 26, "The day following Mid-Autumn Festival"),
            (2026, 10, 2, "The day following National Day"),
            (2026, 10, 18, "Chong Yeung Festival"),
            (2026, 11, 2, "All Souls' Day"),
            (2026, 12, 8, "Feast of the Immaculate Conception"),
            (
                2026,
                12,
                20,
                "Macao Special Administrative Region Establishment Day",
            ),
            (2026, 12, 22, "Winter Solstice"),
            (2026, 12, 24, "Christmas Eve"),
            (2027, 2, 6, "Lunar New Year's Day"),
            (2027, 4, 5, "Cheng Ming Festival"),
            (2027, 5, 13, "The Buddha's Birthday"),
            (2027, 10, 8, "Chong Yeung Festival"),
            (2027, 12, 22, "Winter Solstice"),
        ],
    );
    // 2026 and 2027 as the Government lists them.
    expect_substitute("MO", None, 2026, (4, 4), (4, 6));
    expect_substitute("MO", None, 2026, (4, 5), (4, 7));
    expect_substitute("MO", None, 2026, (5, 24), (5, 25));
    expect_substitute("MO", None, 2026, (9, 26), (9, 28));
    expect_substitute("MO", None, 2026, (10, 18), (10, 19));
    expect_substitute("MO", None, 2026, (12, 20), (12, 21));
    expect_substitute("MO", None, 2027, (2, 6), (2, 9));
    expect_substitute("MO", None, 2027, (2, 7), (2, 10));
    expect_substitute("MO", None, 2027, (3, 27), (3, 29));
    expect_substitute("MO", None, 2027, (5, 1), (5, 3));
    expect_substitute("MO", None, 2027, (10, 2), (10, 4));
    expect_substitute("MO", None, 2027, (12, 25), (12, 27));
    // Nothing was made up before 2019: 8 December 2018 was a Saturday.
    expect_working("MO", None, &[(2018, 12, 10)]);
    let calendar = HolidayCalendar::for_year(table("MO"), None, 2026);
    for (month, day, name, kind) in [
        (1, 1, "New Year's Day", Kind::Public),
        (4, 3, "Good Friday", Kind::Bank),
        (2, 16, "Lunar New Year's Eve", Kind::Observance),
        (12, 31, "New Year's Eve", Kind::Observance),
    ] {
        let found: Vec<(&str, Kind)> = calendar
            .on(ymd(2026, month, day))
            .iter()
            .map(|holiday| (holiday.name, holiday.kind))
            .collect();
        assert_eq!(found, [(name, kind)], "{month}-{day}");
    }
}

#[test]
fn bolivia_moves_sunday_holidays_to_monday_except_the_four_the_decree_names() {
    expect(
        "BO",
        None,
        &[
            (2026, 1, 22, "Plurinational State Foundation Day"),
            (2026, 2, 16, "Carnival Monday"),
            (2026, 2, 17, "Carnival Tuesday"),
            (2026, 4, 3, "Good Friday"),
            (2026, 6, 4, "Corpus Christi"),
            (2026, 6, 21, "Aymara Amazonian New Year"),
            (2026, 8, 6, "Independence Day"),
            (2026, 11, 2, "All Souls' Day"),
            (2010, 1, 22, "Plurinational State Foundation Day"),
            (2009, 6, 21, "Aymara Amazonian New Year"),
        ],
    );
    // 21 June 2026, 1 January 2023 and 22 January 2017 are Sundays.
    expect_substitute("BO", None, 2026, (6, 21), (6, 22));
    expect_substitute("BO", None, 2023, (1, 1), (1, 2));
    expect_substitute("BO", None, 2017, (1, 22), (1, 23));
    // 2 November 2025 is a Sunday and is one of the four exceptions.
    expect_working("BO", None, &[(2025, 11, 3), (2009, 1, 22), (2008, 6, 21)]);
}

#[test]
fn chile_moves_each_holiday_by_its_own_law() {
    expect(
        "CL",
        None,
        &[
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 4, "Holy Saturday"),
            (2026, 5, 21, "Navy Day"),
            (2026, 6, 21, "National Indigenous Peoples' Day"),
            (2026, 6, 29, "Saints Peter and Paul"),
            (2026, 7, 16, "Our Lady of Mount Carmel"),
            (2026, 8, 15, "Assumption of Mary"),
            (2026, 9, 18, "Independence Day"),
            (2026, 9, 19, "Army Day"),
            (2026, 10, 12, "Meeting of Two Worlds Day"),
            (
                2026,
                10,
                31,
                "National Day of the Evangelical and Protestant Churches",
            ),
            (2026, 11, 1, "All Saints' Day"),
            (2026, 12, 8, "Immaculate Conception"),
            // Ley 19.668: Thursdays back to Monday in 2023, Wednesdays in
            // 2022, a Tuesday in 2021; Ley 20.299: a Tuesday 31 October to
            // the Friday before, a Wednesday to the Friday after.
            (2023, 6, 26, "Saints Peter and Paul"),
            (2023, 10, 9, "Meeting of Two Worlds Day"),
            (
                2023,
                10,
                27,
                "National Day of the Evangelical and Protestant Churches",
            ),
            (2022, 6, 27, "Saints Peter and Paul"),
            (2022, 10, 10, "Meeting of Two Worlds Day"),
            (2021, 6, 28, "Saints Peter and Paul"),
            (
                2018,
                11,
                2,
                "National Day of the Evangelical and Protestant Churches",
            ),
            (
                2024,
                10,
                31,
                "National Day of the Evangelical and Protestant Churches",
            ),
            // Ley 21.357: 21 June 2021 by the transitional article, then
            // the solstice at Chile's meridian.
            (2021, 6, 21, "National Indigenous Peoples' Day"),
            (2024, 6, 20, "National Indigenous Peoples' Day"),
            (2025, 6, 20, "National Indigenous Peoples' Day"),
            // Leyes 20.215 and 20.983.
            (2007, 9, 17, "Monday 17 September"),
            (2018, 9, 17, "Monday 17 September"),
            (2021, 9, 17, "Friday 17 September"),
            (2019, 9, 20, "Friday 20 September"),
            (2017, 1, 2, "Monday after New Year's Day"),
            (2023, 1, 2, "Monday after New Year's Day"),
            // Corpus Christi, fixed then moved then gone; the older names.
            (1999, 6, 3, "Corpus Christi"),
            (2006, 6, 12, "Corpus Christi"),
            (1999, 10, 12, "Discovery of America Anniversary"),
            (2000, 9, 4, "National Unity Day"),
            (1998, 9, 11, "Day of National Liberation"),
        ],
    );
    expect("CL", Some("CL-AP"), &[(2026, 6, 7, "Battle of Arica Day")]);
    expect_working(
        "CL",
        None,
        &[
            (2023, 6, 29),
            (2023, 10, 12),
            (2023, 10, 31),
            (2024, 6, 21),
            (2010, 9, 17),
            (2012, 1, 2),
            (2026, 9, 17),
            (2007, 6, 7),
            (2026, 6, 7),
        ],
    );
}

#[test]
fn ecuador_moves_holidays_off_midweek_and_off_the_weekend_and_resolves_november() {
    expect(
        "EC",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 2, 16, "Carnival Monday"),
            (2026, 2, 17, "Carnival Tuesday"),
            (2026, 4, 3, "Good Friday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 25, "Battle of Pichincha"),
            (2026, 8, 10, "First Cry of Independence"),
            (2026, 10, 9, "Independence of Guayaquil"),
            (2026, 11, 2, "Day of the Dead"),
            (2026, 11, 3, "Independence of Cuenca"),
            (2026, 12, 25, "Christmas Day"),
            (2025, 5, 23, "Battle of Pichincha"),
            (2025, 8, 11, "First Cry of Independence"),
            (2025, 10, 10, "Independence of Guayaquil"),
            (2025, 11, 3, "Independence of Cuenca"),
            (2025, 11, 4, "Day of the Dead"),
            (2024, 5, 3, "Labour Day"),
            (2024, 8, 9, "First Cry of Independence"),
            (2024, 10, 11, "Independence of Guayaquil"),
            (2024, 11, 1, "Day of the Dead"),
            (2024, 11, 4, "Independence of Cuenca"),
            (2023, 11, 2, "Day of the Dead"),
            (2023, 11, 3, "Independence of Cuenca"),
            (2022, 11, 3, "Day of the Dead"),
            (2022, 11, 4, "Independence of Cuenca"),
            (2018, 11, 1, "Independence of Cuenca"),
            (2018, 11, 2, "Day of the Dead"),
            // A Saturday Christmas and New Year's Day go to the Friday before,
            // the second across the New Year.
            (2021, 12, 24, "Christmas Day"),
            (2021, 12, 31, "New Year's Day"),
            (2016, 5, 24, "Battle of Pichincha"),
        ],
    );
    expect_working(
        "EC",
        None,
        &[
            (2026, 5, 24),
            (2025, 11, 2),
            (2022, 1, 1),
            (2022, 11, 2),
            (2022, 12, 25),
        ],
    );
}

#[test]
fn uruguay_moves_three_holidays_to_the_adjacent_monday_and_keeps_the_rest() {
    expect(
        "UY",
        None,
        &[
            (2026, 1, 6, "Children's Day"),
            (2026, 2, 16, "Carnival Monday"),
            (2026, 2, 17, "Carnival Tuesday"),
            (2026, 3, 30, "Tourism Week"),
            (2026, 4, 4, "Tourism Week"),
            (2026, 4, 19, "Landing of the Thirty-Three Orientals"),
            (2026, 5, 18, "Battle of Las Piedras"),
            (2026, 6, 19, "Birth of Artigas"),
            (2026, 7, 18, "Constitution Day"),
            (2026, 8, 25, "Independence Day"),
            (2026, 10, 12, "Day of the Race"),
            (2026, 11, 2, "All Souls' Day"),
            (2026, 12, 25, "Family Day"),
            (2024, 4, 22, "Landing of the Thirty-Three Orientals"),
            (2023, 4, 17, "Landing of the Thirty-Three Orientals"),
            (2023, 5, 22, "Battle of Las Piedras"),
            (2023, 10, 16, "Day of the Race"),
            (2022, 4, 18, "Landing of the Thirty-Three Orientals"),
            (2022, 5, 16, "Battle of Las Piedras"),
            (2022, 10, 10, "Day of the Race"),
        ],
    );
    expect_working(
        "UY",
        None,
        &[(2023, 4, 19), (2023, 5, 18), (2023, 10, 12), (2026, 4, 20)],
    );
    let calendar = HolidayCalendar::for_year(table("UY"), None, 2026);
    for (month, day, kind) in [
        (5, 1, Kind::Public),
        (1, 6, Kind::Bank),
        (11, 2, Kind::Bank),
    ] {
        let kinds: Vec<Kind> = calendar
            .on(ymd(2026, month, day))
            .iter()
            .map(|holiday| holiday.kind)
            .collect();
        assert_eq!(kinds, [kind], "{month}-{day}");
    }
}

#[test]
fn armenia_has_thirteen_non_working_days_and_a_citizens_day_that_dodges_24_april() {
    expect(
        "AM",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 2, "New Year's Day"),
            (2026, 1, 6, "Christmas and Epiphany"),
            (2026, 1, 27, "Day of Remembrance and Reverence"),
            (2026, 1, 28, "Army Day"),
            (2026, 3, 8, "Women's Day"),
            (2026, 4, 24, "Armenian Genocide Remembrance Day"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 9, "Victory and Peace Day"),
            (2026, 5, 28, "Republic Day"),
            (2026, 7, 5, "Constitution Day"),
            (2026, 9, 21, "Independence Day"),
            (2026, 12, 31, "New Year's Eve"),
        ],
    );
    expect_working("AM", None, &[(2025, 1, 27), (2026, 1, 7), (2026, 1, 3)]);
    // Easter 2026 on 5 April: Vardanants on Thursday 5 February, Holy
    // Etchmiadzin on Sunday 7 June; the Citizen's Day on Saturday 25 April
    // 2026 and, the last Saturday of April 2021 being the 24th, on Sunday
    // 25 April 2021.
    for (year, month, day, name) in [
        (2026, 2, 5, "Saint Vardanants Day"),
        (2026, 6, 7, "Feast of Holy Etchmiadzin"),
        (2026, 4, 25, "Day of the Citizen"),
        (2021, 4, 25, "Day of the Citizen"),
        (2026, 10, 10, "Holy Translators' Day"),
    ] {
        let calendar = HolidayCalendar::for_year(table("AM"), None, year);
        let found: Vec<(&str, Kind)> = calendar
            .on(ymd(year, month, day))
            .iter()
            .map(|holiday| (holiday.name, holiday.kind))
            .collect();
        assert_eq!(found, [(name, Kind::Observance)], "{year}-{month}-{day}");
    }
}

#[test]
fn azerbaijan_reproduces_the_march_2026_that_novruz_and_ramazan_shared() {
    expect(
        "AZ",
        None,
        &[
            (2026, 1, 20, "National Mourning Day"),
            (2026, 3, 20, "Novruz"),
            (2026, 3, 24, "Novruz"),
            (2026, 3, 20, "Ramazan Bayramı"),
            (2026, 5, 27, "Qurban Bayramı"),
            (2026, 5, 28, "Independence Day"),
            (2026, 6, 15, "National Salvation Day"),
            (2026, 6, 26, "Armed Forces Day"),
            (2026, 11, 8, "Victory Day"),
            (2026, 11, 9, "State Flag Day"),
            (2026, 12, 31, "Solidarity Day of World Azerbaijanis"),
            (2010, 11, 9, "State Flag Day"),
            (2021, 11, 8, "Victory Day"),
        ],
    );
    // 8 March 2026 was a Sunday; Novruz's Saturday and Sunday and the two
    // Ramazan days that coincided with it gave 25, 26, 27 and 30 March.
    expect_substitute("AZ", None, 2026, (3, 8), (3, 9));
    let calendar = HolidayCalendar::for_year(table("AZ"), None, 2026);
    for day in [25, 26, 27, 30] {
        assert!(calendar.is_holiday(ymd(2026, 3, day)), "2026-03-{day}");
    }
    expect_working(
        "AZ",
        None,
        &[
            (2026, 3, 31),
            (2026, 4, 1),
            (2026, 10, 18),
            (2026, 11, 12),
            (2009, 11, 9),
            (2020, 11, 8),
        ],
    );
    let kinds: Vec<Kind> = calendar
        .on(ymd(2026, 11, 17))
        .iter()
        .map(|holiday| holiday.kind)
        .collect();
    assert_eq!(kinds, [Kind::Observance]);
}

#[test]
fn georgia_keeps_its_orthodox_easter_and_moves_nothing() {
    // Orthodox Easter 2026 on 12 April.
    expect(
        "GE",
        None,
        &[
            (2026, 1, 7, "Orthodox Christmas"),
            (2026, 1, 19, "Orthodox Epiphany"),
            (2026, 3, 3, "Mother's Day"),
            (2026, 4, 9, "National Unity Day"),
            (2026, 4, 10, "Good Friday"),
            (2026, 4, 11, "Holy Saturday"),
            (2026, 4, 12, "Easter Sunday"),
            (2026, 4, 13, "Easter Monday"),
            (2026, 5, 12, "Saint Andrew the First-Called Day"),
            (2026, 5, 17, "Day of Family Purity and Respect for Parents"),
            (2026, 5, 26, "Independence Day"),
            (2026, 8, 28, "Dormition of the Mother of God"),
            (2026, 10, 14, "Svetitskhovloba"),
            (2026, 11, 23, "Saint George's Day"),
        ],
    );
    // 2 January 2027 and 17 May 2026 fall on the weekend and stay there.
    expect_working("GE", None, &[(2027, 1, 4), (2026, 5, 18)]);
}

#[test]
fn kazakhstan_carries_every_amendment_by_its_year() {
    expect(
        "KZ",
        None,
        &[
            (2026, 1, 7, "Orthodox Christmas"),
            (2026, 3, 21, "Nauryz Meyramy"),
            (2026, 3, 23, "Nauryz Meyramy"),
            (2026, 5, 1, "Kazakhstan People's Unity Day"),
            (2026, 5, 7, "Defender of the Fatherland Day"),
            (2026, 7, 6, "Capital City Day"),
            (2026, 10, 25, "Republic Day"),
            (2026, 12, 16, "Independence Day"),
            (2025, 8, 30, "Constitution Day"),
            (2027, 3, 15, "Constitution Day"),
            (2008, 3, 22, "Nauryz Meyramy"),
            (2009, 3, 21, "Nauryz Meyramy"),
            (2006, 1, 7, "Orthodox Christmas"),
            (2012, 12, 1, "First President Day"),
            (2021, 12, 17, "Independence Day"),
            (2008, 10, 25, "Republic Day"),
            (2022, 10, 25, "Republic Day"),
            (2013, 5, 7, "Defender of the Fatherland Day"),
        ],
    );
    expect_working(
        "KZ",
        None,
        &[
            (2026, 8, 31),
            (2026, 3, 16),
            (2008, 3, 21),
            (2005, 1, 7),
            (2022, 12, 1),
            (2022, 12, 19),
            (2009, 10, 25),
            (2012, 5, 7),
        ],
    );
    // 8 March, 21 and 22 March and 25 October 2026 fall on the weekend.
    expect_substitute("KZ", None, 2026, (3, 8), (3, 9));
    expect_substitute("KZ", None, 2026, (3, 21), (3, 24));
    expect_substitute("KZ", None, 2026, (3, 22), (3, 25));
    expect_substitute("KZ", None, 2026, (10, 25), (10, 26));
}

#[test]
fn albania_gives_each_weekend_holiday_the_first_working_day_after() {
    expect(
        "AL",
        None,
        &[
            (2026, 1, 2, "New Year's Day"),
            (2026, 3, 14, "Summer Day"),
            (2026, 3, 20, "Eid al-Fitr"),
            (2026, 3, 22, "Nevruz Day"),
            (2026, 4, 5, "Catholic Easter"),
            (2026, 4, 12, "Orthodox Easter"),
            (2026, 5, 1, "International Workers' Day"),
            (2026, 5, 27, "Eid al-Adha"),
            (2026, 9, 5, "Mother Teresa Day"),
            (2026, 11, 22, "Alphabet Day"),
            (2026, 11, 28, "Independence Day"),
            (2026, 11, 29, "Liberation Day"),
            (2026, 12, 8, "National Youth Day"),
            (2026, 12, 25, "Christmas Day"),
        ],
    );
    // Summer Day on a Saturday, Nevruz, both Easters and Alphabet Day on
    // Sundays in 2026.
    expect_substitute("AL", None, 2026, (3, 14), (3, 16));
    expect_substitute("AL", None, 2026, (3, 22), (3, 23));
    expect_substitute("AL", None, 2026, (4, 5), (4, 6));
    expect_substitute("AL", None, 2026, (4, 12), (4, 13));
    expect_substitute("AL", None, 2026, (11, 22), (11, 23));
    // 28 and 29 November 2020, a Saturday and a Sunday, gave the Monday
    // and the Tuesday.
    expect_substitute("AL", None, 2020, (11, 28), (11, 30));
    expect_substitute("AL", None, 2020, (11, 29), (12, 1));
    expect_working(
        "AL",
        None,
        &[(2023, 11, 22), (2009, 12, 8), (2003, 3, 14), (2026, 3, 17)],
    );
}

#[test]
fn montenegro_keeps_two_days_and_pushes_a_sunday_first_day_to_tuesday() {
    expect(
        "ME",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 2, "New Year's Day"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 2, "Labour Day"),
            (2026, 5, 21, "Independence Day"),
            (2026, 5, 22, "Independence Day"),
            (2026, 7, 13, "Statehood Day"),
            (2026, 7, 14, "Statehood Day"),
            (2026, 11, 13, "Njegoš Day"),
            (2026, 11, 14, "Njegoš Day"),
            (2022, 11, 13, "Njegoš Day"),
        ],
    );
    // 21 May 2023 and 13 July 2025, first days on Sundays, gave the
    // Tuesdays; 14 July 2024, a second day on a Sunday, gave the Monday.
    expect_substitute("ME", None, 2023, (5, 21), (5, 23));
    expect_substitute("ME", None, 2025, (7, 13), (7, 15));
    expect_substitute("ME", None, 2024, (7, 14), (7, 15));
    expect_working("ME", None, &[(2021, 11, 15), (2006, 5, 22), (2026, 1, 7)]);
    let calendar = HolidayCalendar::for_year(table("ME"), None, 2026);
    for (month, day, name) in [
        (1, 7, "Orthodox Christmas"),
        (4, 10, "Orthodox Good Friday"),
        (12, 25, "Christmas Day"),
    ] {
        let found: Vec<(&str, Kind)> = calendar
            .on(ymd(2026, month, day))
            .iter()
            .map(|holiday| (holiday.name, holiday.kind))
            .collect();
        assert_eq!(found, [(name, Kind::Religious)], "{month}-{day}");
    }
}

#[test]
fn north_macedonia_moves_a_sunday_holiday_to_monday_and_keeps_duhovden_on_a_friday() {
    expect(
        "MK",
        None,
        &[
            (2026, 1, 7, "Orthodox Christmas"),
            (2026, 3, 20, "Eid al-Fitr"),
            (2026, 4, 13, "Orthodox Easter Monday"),
            (2026, 5, 24, "Saints Cyril and Methodius Day"),
            (2026, 8, 2, "Republic Day"),
            (2026, 9, 8, "Independence Day"),
            (2026, 10, 11, "Day of the People's Uprising"),
            (2026, 10, 23, "Day of the Macedonian Revolutionary Struggle"),
            (2026, 12, 8, "Saint Clement of Ohrid Day"),
        ],
    );
    // 24 May, 2 August and 11 October 2026 are Sundays.
    expect_substitute("MK", None, 2026, (5, 24), (5, 25));
    expect_substitute("MK", None, 2026, (8, 2), (8, 3));
    expect_substitute("MK", None, 2026, (10, 11), (10, 12));
    expect_working(
        "MK",
        None,
        &[(2006, 10, 23), (2006, 12, 8), (2026, 1, 19), (2026, 5, 29)],
    );
    // Orthodox Pentecost 2026 on 31 May: Duhovden on Friday 29 May.
    let calendar = HolidayCalendar::for_year(table("MK"), None, 2026);
    for (month, day, name, kind) in [
        (1, 19, "Orthodox Epiphany", Kind::Religious),
        (5, 29, "Duhovden", Kind::Religious),
        (11, 22, "Albanian Alphabet Day", Kind::Observance),
    ] {
        let found: Vec<(&str, Kind)> = calendar
            .on(ymd(2026, month, day))
            .iter()
            .map(|holiday| (holiday.name, holiday.kind))
            .collect();
        assert_eq!(found, [(name, kind)], "{month}-{day}");
    }
}

#[test]
fn serbia_moves_state_holidays_off_sunday_and_leaves_the_church_days_alone() {
    // Orthodox Easter 2026 on 12 April.
    expect(
        "RS",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 2, "New Year's Day"),
            (2026, 1, 7, "Christmas Day"),
            (2026, 2, 15, "Statehood Day"),
            (2026, 2, 16, "Statehood Day"),
            (2026, 4, 10, "Good Friday"),
            (2026, 4, 11, "Holy Saturday"),
            (2026, 4, 12, "Easter Sunday"),
            (2026, 4, 13, "Easter Monday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 2, "Labour Day"),
            (2026, 11, 11, "Armistice Day"),
            (2012, 11, 11, "Armistice Day"),
            (2002, 2, 15, "Statehood Day"),
        ],
    );
    // 15 February 2026 on a Sunday, its Monday taken by the second day;
    // 2 May 2027 on a Sunday that is also Orthodox Easter, its Monday taken
    // by Easter Monday; Christmas 2024 on a Sunday moved nothing.
    expect_substitute("RS", None, 2026, (2, 15), (2, 17));
    expect_substitute("RS", None, 2027, (5, 2), (5, 4));
    expect_working(
        "RS",
        None,
        &[
            (2024, 1, 8),
            (2011, 11, 11),
            (2001, 2, 15),
            (2026, 1, 27),
            (2026, 12, 25),
        ],
    );
    let calendar = HolidayCalendar::for_year(table("RS"), None, 2026);
    for (month, day, name, kind) in [
        (1, 27, "Saint Sava Day", Kind::Observance),
        (
            9,
            15,
            "Day of Serbian Unity, Freedom and the National Flag",
            Kind::Observance,
        ),
        (12, 25, "Catholic Christmas", Kind::Religious),
        (9, 21, "Yom Kippur", Kind::Religious),
    ] {
        let found: Vec<(&str, Kind)> = calendar
            .on(ymd(2026, month, day))
            .iter()
            .map(|holiday| (holiday.name, holiday.kind))
            .collect();
        assert_eq!(found, [(name, kind)], "{month}-{day}");
    }
}

#[test]
fn bosnia_and_herzegovina_has_no_state_holidays_and_three_laws_beneath() {
    // No state law: asking for the country as a whole gives nothing.
    assert!(
        HolidayCalendar::for_year(table("BA"), None, 2026)
            .all()
            .is_empty()
    );

    // The Federation: 1 January 2023 on a Sunday gave Tuesday 3 January,
    // 1 May 2016 on a Sunday gave Tuesday 3 May and 2 May 2021 on a Sunday
    // gave Monday 3 May, as the Federal Ministry announced.
    expect(
        "BA",
        Some("BA-BIH"),
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 2, "New Year's Day"),
            (2026, 3, 1, "Independence Day"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 2, "Labour Day"),
            (2026, 11, 25, "Statehood Day"),
            (2025, 3, 1, "Independence Day"),
            (2025, 11, 25, "Statehood Day"),
        ],
    );
    expect_substitute("BA", Some("BA-BIH"), 2023, (1, 1), (1, 3));
    expect_substitute("BA", Some("BA-BIH"), 2016, (5, 1), (5, 3));
    expect_substitute("BA", Some("BA-BIH"), 2021, (5, 2), (5, 3));
    // 9 May is kept at work; the Government's Monday for the Sunday
    // 1 March 2026 is not carried, nor are the other entity's days.
    expect_working(
        "BA",
        Some("BA-BIH"),
        &[(2025, 5, 9), (2026, 3, 2), (2026, 1, 9), (2026, 11, 21)],
    );

    // Republika Srpska: article 4 moved Sunday 2 January 2022 to Monday
    // 3 January, and left Sunday 1 January 2023 alone.
    expect(
        "BA",
        Some("BA-SRP"),
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 2, "New Year's Day"),
            (2026, 1, 9, "Republic Day"),
            (2026, 5, 1, "International Labour Day"),
            (2026, 5, 2, "International Labour Day"),
            (2026, 5, 9, "Victory over Fascism Day"),
            (2026, 11, 21, "Dayton Agreement Day"),
            (2016, 1, 9, "Republic Day"),
            (2019, 1, 9, "Republic Day"),
        ],
    );
    expect_substitute("BA", Some("BA-SRP"), 2022, (1, 2), (1, 3));
    expect_working(
        "BA",
        Some("BA-SRP"),
        &[
            (2023, 1, 3),
            (2026, 1, 7),
            (2026, 1, 14),
            (2026, 3, 1),
            (2026, 11, 25),
        ],
    );
    // The religious days are the believers' own. Orthodox Easter 2026 on
    // 12 April, Catholic on 5 April.
    let calendar = HolidayCalendar::for_year(table("BA"), Some("BA-SRP"), 2026);
    for (month, day, name) in [
        (1, 6, "Orthodox Christmas Eve"),
        (1, 7, "Orthodox Christmas"),
        (4, 3, "Catholic Good Friday"),
        (4, 6, "Catholic Easter Monday"),
        (4, 10, "Orthodox Good Friday"),
        (4, 13, "Orthodox Easter Monday"),
        (12, 24, "Catholic Christmas Eve"),
        (12, 25, "Catholic Christmas"),
    ] {
        let found: Vec<(&str, Kind)> = calendar
            .on(ymd(2026, month, day))
            .iter()
            .map(|holiday| (holiday.name, holiday.kind))
            .collect();
        assert_eq!(found, [(name, Kind::Religious)], "{month}-{day}");
    }
    // 9 January is known from the Ministry's notices to 2026 and no
    // further.
    let later = HolidayCalendar::for_year(table("BA"), Some("BA-SRP"), 2027);
    assert!(later.gaps().iter().any(|gap| gap.name == "Republic Day"));

    // Brčko: District Day on Sunday 8 March 2026 gave Monday 9 March, and
    // the Assembly's decisions give each religious holiday one day.
    expect(
        "BA",
        Some("BA-BRC"),
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 2, "New Year's Day"),
            (2026, 1, 7, "Orthodox Christmas"),
            (2026, 3, 8, "Brčko District Day"),
            (2026, 3, 20, "Eid al-Fitr"),
            (2026, 4, 6, "Catholic Easter"),
            (2026, 4, 13, "Orthodox Easter"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 2, "Labour Day"),
            (2026, 5, 27, "Eid al-Adha"),
            (2026, 12, 25, "Catholic Christmas"),
            (2025, 4, 18, "Catholic Easter"),
            (2022, 12, 26, "Catholic Christmas"),
            (2023, 1, 9, "Orthodox Christmas"),
            (2027, 1, 7, "Orthodox Christmas"),
        ],
    );
    expect_substitute("BA", Some("BA-BRC"), 2026, (3, 8), (3, 9));
    expect_working(
        "BA",
        Some("BA-BRC"),
        &[(2026, 4, 3), (2026, 4, 10), (2026, 1, 9), (2026, 3, 1)],
    );
    // Past the decisions read, the religious days are a gap, not absent.
    let gaps: Vec<&str> = HolidayCalendar::for_year(table("BA"), Some("BA-BRC"), 2027)
        .gaps()
        .iter()
        .map(|gap| gap.name)
        .collect();
    assert!(gaps.contains(&"Catholic Easter"), "{gaps:?}");
    assert!(!gaps.contains(&"Orthodox Christmas"), "{gaps:?}");
}

#[test]
fn costa_rica_kept_the_mondays_ley_9875_named_and_no_others() {
    expect(
        "CR",
        None,
        &[
            (2026, 4, 2, "Maundy Thursday"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 11, "Juan Santamaría Day"),
            (2026, 7, 25, "Annexation of the Party of Nicoya"),
            (2026, 8, 2, "Feast of Our Lady of the Angels"),
            (2026, 8, 15, "Mother's Day"),
            (
                2026,
                8,
                31,
                "Day of the Black Person and Afro-Costa Rican Culture",
            ),
            (2026, 9, 15, "Independence Day"),
            (2026, 12, 1, "Army Abolition Day"),
            // The provision's Mondays.
            (2020, 7, 27, "Annexation of the Party of Nicoya"),
            (2020, 8, 17, "Mother's Day"),
            (2020, 9, 14, "Independence Day"),
            (2020, 11, 30, "Army Abolition Day"),
            (2021, 5, 3, "Labour Day"),
            (2021, 7, 26, "Annexation of the Party of Nicoya"),
            (2021, 9, 13, "Independence Day"),
            (2021, 11, 29, "Army Abolition Day"),
            (2022, 9, 19, "Independence Day"),
            (2022, 12, 5, "Army Abolition Day"),
            (2023, 4, 10, "Juan Santamaría Day"),
            (2023, 7, 24, "Annexation of the Party of Nicoya"),
            (2023, 8, 14, "Mother's Day"),
            (2024, 4, 15, "Juan Santamaría Day"),
            (2024, 7, 29, "Annexation of the Party of Nicoya"),
            (2024, 8, 15, "Mother's Day"),
            (2019, 10, 12, "Day of the Cultures"),
            (
                2022,
                8,
                31,
                "Day of the Black Person and Afro-Costa Rican Culture",
            ),
        ],
    );
    expect_working(
        "CR",
        None,
        &[
            (2020, 9, 15),
            (2023, 4, 11),
            (2024, 4, 11),
            (2024, 8, 12),
            (2020, 10, 12),
            (2019, 12, 2),
            (2021, 8, 31),
            (2026, 9, 14),
        ],
    );
}

#[test]
fn dominican_republic_moves_by_ley_139_97_and_keeps_restoration_day_in_inauguration_years() {
    expect(
        "DO",
        None,
        &[
            (2026, 1, 5, "Epiphany"),
            (2026, 1, 21, "Our Lady of Altagracia"),
            (2026, 1, 26, "Duarte Day"),
            (2026, 2, 27, "Independence Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 5, 4, "Labour Day"),
            (2026, 6, 4, "Corpus Christi"),
            (2026, 8, 16, "Restoration Day"),
            (2026, 9, 24, "Our Lady of Mercy"),
            (2026, 11, 9, "Constitution Day"),
            (2026, 12, 25, "Christmas Day"),
            // 2024 as the Ministry listed it, a Friday 16 August that opened a
            // constitutional period staying put; 2023's Wednesday moved.
            (2024, 1, 29, "Duarte Day"),
            (2024, 4, 29, "Labour Day"),
            (2024, 8, 16, "Restoration Day"),
            (2024, 11, 4, "Constitution Day"),
            (2023, 8, 14, "Restoration Day"),
            // A Sunday 1 May gives the Monday; before the law the dates were fixed.
            (2022, 5, 2, "Labour Day"),
            (1996, 5, 1, "Labour Day"),
        ],
    );
    expect_working(
        "DO",
        None,
        &[
            (2026, 1, 6),
            (2026, 5, 1),
            (2026, 11, 6),
            (2024, 8, 19),
            (2023, 8, 16),
            (1996, 4, 29),
            (2022, 5, 1),
        ],
    );
}

#[test]
fn guatemala_moves_army_day_to_a_monday_and_moved_two_more_only_until_the_court_ruled() {
    expect(
        "GT",
        None,
        &[
            (2026, 4, 2, "Maundy Thursday"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 4, "Holy Saturday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 6, 29, "Army Day"),
            (2026, 9, 15, "Independence Day"),
            (2026, 10, 20, "Revolution Day"),
            (2026, 11, 1, "All Saints' Day"),
            (2026, 12, 24, "Christmas Eve"),
            (2026, 12, 31, "New Year's Eve"),
            (2019, 4, 29, "Labour Day"),
            (2019, 7, 1, "Army Day"),
            (2019, 10, 21, "Revolution Day"),
            (2018, 10, 22, "Revolution Day"),
            (2021, 6, 28, "Army Day"),
            (2018, 6, 30, "Army Day"),
        ],
    );
    expect("GT", Some("GT-GU"), &[(2026, 8, 15, "Assumption Day")]);
    expect_working(
        "GT",
        None,
        &[
            (2026, 6, 30),
            (2018, 7, 2),
            (2020, 4, 27),
            (2026, 8, 15),
            (2026, 8, 17),
        ],
    );
    let calendar = HolidayCalendar::for_year(table("GT"), None, 2026);
    let kinds: Vec<Kind> = calendar
        .on(ymd(2026, 12, 24))
        .iter()
        .map(|holiday| holiday.kind)
        .collect();
    assert_eq!(kinds, [Kind::Bank]);
}

#[test]
fn panama_moves_two_holidays_by_ley_70_and_the_rest_off_sunday_only() {
    expect(
        "PA",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 12, "Martyrs' Day"),
            (2026, 2, 17, "Carnival Tuesday"),
            (2026, 4, 3, "Good Friday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 11, 3, "Separation Day"),
            (2026, 11, 5, "Colón Day"),
            (2026, 11, 10, "First Cry of Independence"),
            (2026, 11, 28, "Independence Day"),
            (2026, 12, 8, "Mother's Day"),
            (2026, 12, 20, "National Mourning Day"),
            (2026, 12, 25, "Christmas Day"),
            (2025, 1, 13, "Martyrs' Day"),
            (2025, 12, 1, "Independence Day"),
            (2024, 1, 8, "Martyrs' Day"),
            (2024, 12, 2, "Independence Day"),
            (2007, 1, 9, "Martyrs' Day"),
        ],
    );
    // Sundays: 20 December 2026, 3 November 2024, 1 May 2022.
    expect_substitute("PA", None, 2026, (12, 20), (12, 21));
    expect_substitute("PA", None, 2024, (11, 3), (11, 4));
    expect_substitute("PA", None, 2022, (5, 1), (5, 2));
    expect_working(
        "PA",
        None,
        &[
            (2026, 1, 9),
            (2025, 11, 28),
            (2021, 12, 20),
            (2026, 11, 4),
            (2026, 11, 30),
        ],
    );
    let calendar = HolidayCalendar::for_year(table("PA"), None, 2026);
    let kinds: Vec<Kind> = calendar
        .on(ymd(2026, 11, 4))
        .iter()
        .map(|holiday| holiday.kind)
        .collect();
    assert_eq!(kinds, [Kind::Observance]);
}

#[test]
fn belarus_keeps_radunitsa_nine_days_after_the_orthodox_easter() {
    // Orthodox Easter 2026 on 12 April.
    expect(
        "BY",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 2, "New Year's Day"),
            (2026, 1, 7, "Orthodox Christmas"),
            (2026, 3, 8, "International Women's Day"),
            (2026, 4, 21, "Radunitsa"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 9, "Victory Day"),
            (2026, 7, 3, "Independence Day"),
            (2026, 11, 7, "October Revolution Day"),
            (2026, 12, 25, "Catholic Christmas"),
            (2020, 1, 2, "New Year's Day"),
            (1996, 7, 27, "Independence Day"),
            (1997, 7, 3, "Independence Day"),
        ],
    );
    expect_working(
        "BY",
        None,
        &[(2019, 1, 2), (1996, 7, 3), (1997, 7, 27), (2026, 9, 17)],
    );
    let calendar = HolidayCalendar::for_year(table("BY"), None, 2026);
    let kinds: Vec<Kind> = calendar
        .on(ymd(2026, 9, 17))
        .iter()
        .map(|holiday| holiday.kind)
        .collect();
    assert_eq!(kinds, [Kind::Observance]);
}

#[test]
fn luxembourg_has_eleven_legal_holidays_and_a_bank_holiday() {
    expect(
        "LU",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 9, "Europe Day"),
            (2026, 5, 14, "Ascension Day"),
            (2026, 5, 25, "Whit Monday"),
            (2026, 6, 23, "National Day"),
            (2026, 8, 15, "Assumption Day"),
            (2026, 11, 1, "All Saints' Day"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 26, "Saint Stephen's Day"),
            (2019, 5, 9, "Europe Day"),
        ],
    );
    expect_working("LU", None, &[(2018, 5, 9), (2026, 11, 2)]);
    let calendar = HolidayCalendar::for_year(table("LU"), None, 2026);
    let kinds: Vec<Kind> = calendar
        .on(ymd(2026, 4, 3))
        .iter()
        .map(|holiday| holiday.kind)
        .collect();
    assert_eq!(kinds, [Kind::Bank]);
}

#[test]
fn malta_has_fourteen_fixed_holidays_that_never_move() {
    expect(
        "MT",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 2, 10, "Feast of Saint Paul's Shipwreck"),
            (2026, 3, 19, "Feast of Saint Joseph"),
            (2026, 3, 31, "Freedom Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 5, 1, "Workers' Day"),
            (2026, 6, 7, "Sette Giugno"),
            (2026, 6, 29, "Feast of Saint Peter and Saint Paul"),
            (2026, 8, 15, "Feast of the Assumption"),
            (2026, 9, 8, "Victory Day"),
            (2026, 9, 21, "Independence Day"),
            (2026, 12, 8, "Feast of the Immaculate Conception"),
            (2026, 12, 13, "Republic Day"),
            (2026, 12, 25, "Christmas Day"),
        ],
    );
    // 7 June and 15 August 2026 fall on the weekend and stay there.
    expect_working("MT", None, &[(2026, 6, 8), (2026, 8, 17)]);
}

#[test]
fn moldova_keeps_two_easters_and_shares_9_may_between_two_names() {
    // Orthodox Easter 2026 on 12 April, the Blajini's Monday on the 20th.
    expect(
        "MD",
        None,
        &[
            (2026, 1, 7, "Orthodox Christmas"),
            (2026, 1, 8, "Orthodox Christmas"),
            (2026, 3, 8, "International Women's Day"),
            (2026, 4, 12, "Easter Sunday"),
            (2026, 4, 13, "Easter Monday"),
            (2026, 4, 20, "Easter of the Blajini"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 9, "Victory Day"),
            (2026, 5, 9, "Europe Day"),
            (2026, 6, 1, "Children's Day"),
            (2026, 8, 27, "Independence Day"),
            (2026, 8, 31, "Romanian Language Day"),
            (2026, 12, 25, "Christmas Day"),
            (2024, 6, 1, "Children's Day"),
            (2017, 5, 9, "Europe Day"),
            (2009, 12, 25, "Christmas Day"),
        ],
    );
    expect("MD", Some("MD-CU"), &[(2026, 10, 14, "Feast of Chișinău")]);
    expect_working(
        "MD",
        None,
        &[(2023, 6, 1), (2008, 12, 25), (2026, 10, 14), (2026, 4, 21)],
    );
    // Before 2017, 9 May was Victory Day alone: 2015, when the Blajini's
    // Monday fell elsewhere.
    let calendar = HolidayCalendar::for_year(table("MD"), None, 2015);
    let names: Vec<&str> = calendar
        .on(ymd(2015, 5, 9))
        .iter()
        .map(|holiday| holiday.name)
        .collect();
    assert_eq!(names, ["Victory Day"]);
}

#[test]
fn bahrain_keeps_three_days_of_each_eid_and_two_of_ashura() {
    // By the tabular calendar: 1 Shawwal 1447 on 20 March 2026 and 10 Dhu
    // al-Hijjah on 27 May, as the sightings gave; 1 Muharram 1448 on 17 June,
    // Ashura on 26 June and 12 Rabi' al-Awwal on 26 August, a day after the
    // sightings, which is what `approximate` means.
    expect(
        "BH",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 3, 20, "Eid al-Fitr"),
            (2026, 3, 22, "Eid al-Fitr"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 27, "Eid al-Adha"),
            (2026, 5, 29, "Eid al-Adha"),
            (2026, 6, 17, "Hijri New Year"),
            (2026, 6, 25, "Ashura"),
            (2026, 6, 26, "Ashura"),
            (2026, 8, 26, "Prophet's Birthday"),
            (2026, 12, 16, "National Day"),
            (2026, 12, 17, "National Day"),
        ],
    );
    // No Arafat Day, and no fourth day of either Eid.
    expect_working("BH", None, &[(2026, 5, 26), (2026, 5, 30), (2026, 3, 23)]);
}

#[test]
fn jordan_gives_four_days_of_fitr_and_five_from_arafat_and_christmas_to_all() {
    expect(
        "JO",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 3, 20, "Eid al-Fitr"),
            (2026, 3, 23, "Eid al-Fitr"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 25, "Independence Day"),
            (2026, 5, 26, "Eid al-Adha"),
            (2026, 5, 30, "Eid al-Adha"),
            (2026, 6, 17, "Hijri New Year"),
            (2026, 8, 26, "Prophet's Birthday"),
            (2026, 12, 25, "Christmas Day"),
        ],
    );
    expect_working(
        "JO",
        None,
        &[(2026, 3, 24), (2026, 5, 31), (2026, 12, 26), (2026, 4, 13)],
    );
    // Orthodox Easter 2026 on 12 April: the Christian employees' days.
    let calendar = HolidayCalendar::for_year(table("JO"), None, 2026);
    for (month, day, name, kind) in [
        (4, 5, "Palm Sunday", Kind::Religious),
        (4, 13, "Easter Monday", Kind::Religious),
        (12, 26, "Christmas Day", Kind::Religious),
        (1, 30, "King Abdullah II's Birthday", Kind::Observance),
    ] {
        let found: Vec<(&str, Kind)> = calendar
            .on(ymd(2026, month, day))
            .iter()
            .map(|holiday| (holiday.name, holiday.kind))
            .collect();
        assert_eq!(found, [(name, kind)], "{month}-{day}");
    }
}

#[test]
fn kuwait_carries_article_68_with_arafat_and_isra_and_miraj() {
    expect(
        "KW",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 2, 25, "National Day"),
            (2026, 2, 26, "Liberation Day"),
            (2026, 3, 20, "Eid al-Fitr"),
            (2026, 3, 22, "Eid al-Fitr"),
            (2026, 5, 26, "Day of Arafat"),
            (2026, 5, 27, "Eid al-Adha"),
            (2026, 5, 29, "Eid al-Adha"),
            (2026, 6, 17, "Hijri New Year"),
            (2026, 8, 26, "Prophet's Birthday"),
        ],
    );
    expect_working("KW", None, &[(2026, 5, 30), (2026, 3, 23), (2026, 5, 1)]);
    let calendar = HolidayCalendar::for_year(table("KW"), None, 2026);
    let isra: Vec<&str> = calendar
        .all()
        .iter()
        .filter(|holiday| holiday.name == "Isra and Mi'raj")
        .map(|holiday| holiday.name)
        .collect();
    assert_eq!(isra, ["Isra and Mi'raj"]);
}

#[test]
fn lebanon_closes_by_decree_15215_and_moves_labour_day_alone() {
    // Easter 2026 on 5 April, the Orthodox Easter on 12 April.
    expect(
        "LB",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 6, "Armenian Orthodox Christmas"),
            (2026, 2, 9, "Saint Maron's Day"),
            (2026, 3, 20, "Eid al-Fitr"),
            (2026, 3, 21, "Eid al-Fitr"),
            (2026, 3, 25, "Annunciation"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 10, "Orthodox Good Friday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 27, "Eid al-Adha"),
            (2026, 5, 28, "Eid al-Adha"),
            (2026, 6, 17, "Hijri New Year"),
            (2026, 6, 26, "Ashura"),
            (2026, 8, 15, "Assumption"),
            (2026, 8, 26, "Prophet's Birthday"),
            (2026, 11, 22, "Independence Day"),
            (2026, 12, 25, "Christmas Day"),
            // 2025: both Easters on 20 April, so Saturday 19 April closed.
            (2025, 4, 18, "Good Friday"),
            (2025, 4, 19, "Holy Saturday"),
        ],
    );
    // A Sunday Labour Day gives the Monday (2022); a Sunday Independence
    // Day (2026) gives nothing; no Easter Monday, no third Eid day, and
    // Armenian Christmas from 2003.
    expect_substitute("LB", None, 2022, (5, 1), (5, 2));
    expect_working(
        "LB",
        None,
        &[
            (2026, 11, 23),
            (2026, 4, 6),
            (2026, 4, 13),
            (2026, 3, 22),
            (2026, 5, 29),
            (2002, 1, 6),
            (2026, 4, 4),
            (2026, 5, 6),
            (2026, 5, 25),
        ],
    );
    let calendar = HolidayCalendar::for_year(table("LB"), None, 2026);
    for (month, day, name) in [
        (5, 3, "Martyrs' Day"),
        (5, 10, "Resistance and Liberation Day"),
        (2, 14, "Rafic Hariri Memorial Day"),
    ] {
        let found: Vec<(&str, Kind)> = calendar
            .on(ymd(2026, month, day))
            .iter()
            .map(|holiday| (holiday.name, holiday.kind))
            .collect();
        assert_eq!(found, [(name, Kind::Observance)], "{month}-{day}");
    }
}

#[test]
fn tanzania_moves_saturday_and_sunday_holidays_to_the_next_free_day() {
    expect(
        "TZ",
        None,
        &[
            (2026, 1, 12, "Zanzibar Revolution Day"),
            (2026, 3, 20, "Eid al-Fitr"),
            (2026, 3, 21, "Eid al-Fitr"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 4, 7, "Karume Day"),
            (2026, 4, 26, "Union Day"),
            (2026, 5, 27, "Eid al-Adha"),
            (2026, 7, 7, "Saba Saba Day"),
            (2026, 8, 8, "Nane Nane Day"),
            (2026, 8, 26, "Maulid"),
            (2026, 10, 14, "Nyerere Day"),
            (2026, 12, 9, "Independence and Republic Day"),
            (2026, 12, 26, "Boxing Day"),
        ],
    );
    // A Saturday second day of Eid, a Sunday Union Day, a Saturday Nane
    // Nane and a Saturday Boxing Day in 2026.
    expect_substitute("TZ", None, 2026, (3, 21), (3, 23));
    expect_substitute("TZ", None, 2026, (4, 26), (4, 27));
    expect_substitute("TZ", None, 2026, (8, 8), (8, 10));
    expect_substitute("TZ", None, 2026, (12, 26), (12, 28));
}

#[test]
fn uganda_lists_its_days_and_designates_no_substitute_by_rule() {
    expect(
        "UG",
        None,
        &[
            (2026, 1, 26, "NRM Liberation Day"),
            (2026, 2, 16, "Archbishop Janani Luwum Day"),
            (2026, 3, 8, "International Women's Day"),
            (2026, 3, 20, "Eid al-Fitr"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 27, "Eid al-Adha"),
            (2026, 6, 3, "Uganda Martyrs' Day"),
            (2026, 6, 9, "National Heroes' Day"),
            (2026, 10, 9, "Independence Day"),
            (2026, 12, 26, "Boxing Day"),
            (2016, 2, 16, "Archbishop Janani Luwum Day"),
            (2001, 6, 9, "National Heroes' Day"),
        ],
    );
    expect_working(
        "UG",
        None,
        &[(2015, 2, 16), (2000, 6, 9), (2026, 3, 9), (2026, 3, 21)],
    );
}

#[test]
fn zambia_moves_a_sunday_holiday_to_monday_and_dates_its_declared_days() {
    expect(
        "ZM",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 3, 8, "International Women's Day"),
            (2026, 3, 12, "Youth Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 4, "Holy Saturday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 4, 28, "Kenneth Kaunda Day"),
            (2026, 5, 25, "African Freedom Day"),
            (2026, 7, 6, "Heroes' Day"),
            (2026, 7, 7, "Unity Day"),
            (2026, 8, 3, "Farmers' Day"),
            (
                2026,
                10,
                18,
                "National Day of Prayer, Fasting, Repentance and Reconciliation",
            ),
            (2026, 10, 24, "Independence Day"),
            (2026, 12, 25, "Christmas Day"),
            (2008, 3, 8, "International Women's Day"),
            (
                2015,
                10,
                18,
                "National Day of Prayer, Fasting, Repentance and Reconciliation",
            ),
            (2022, 4, 28, "Kenneth Kaunda Day"),
        ],
    );
    // Women's Day and the Day of Prayer fall on Sundays in 2026; a
    // Saturday Independence Day stays.
    expect_substitute("ZM", None, 2026, (3, 8), (3, 9));
    expect_substitute("ZM", None, 2026, (10, 18), (10, 19));
    expect_working(
        "ZM",
        None,
        &[(2026, 10, 26), (2021, 4, 28), (2014, 10, 18), (2007, 3, 8)],
    );
}

#[test]
fn zimbabwe_keeps_the_easter_block_and_moves_a_sunday_holiday_past_a_taken_monday() {
    expect(
        "ZW",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 2, 21, "Robert Gabriel Mugabe National Youth Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 4, "Easter Saturday"),
            (2026, 4, 5, "Easter Sunday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 4, 18, "Independence Day"),
            (2026, 5, 1, "Workers' Day"),
            (2026, 5, 25, "Africa Day"),
            (2026, 8, 10, "Heroes' Day"),
            (2026, 8, 11, "Defence Forces National Day"),
            (2026, 12, 22, "National Unity Day"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 26, "Boxing Day"),
            (2018, 2, 21, "Robert Gabriel Mugabe National Youth Day"),
        ],
    );
    // Christmas 2022 on a Sunday went past Boxing Day to the Tuesday;
    // Saturdays and Easter Sunday claim nothing.
    expect_substitute("ZW", None, 2022, (12, 25), (12, 27));
    expect_working(
        "ZW",
        None,
        &[(2017, 2, 21), (2026, 2, 23), (2026, 4, 7), (2026, 4, 20)],
    );
}

#[test]
fn jamaica_moves_sundays_by_the_schedule_and_labour_day_off_a_saturday_too() {
    expect(
        "JM",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 2, 18, "Ash Wednesday"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            // A Saturday 23 May gives the Monday; a Saturday 1 August stays,
            // as the Ministry's 2026 notice said.
            (2026, 5, 25, "Labour Day"),
            (2026, 8, 1, "Emancipation Day"),
            (2026, 8, 6, "Independence Day"),
            (2026, 10, 19, "National Heroes' Day"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 26, "Boxing Day"),
            (2023, 1, 2, "New Year's Day"),
            (2023, 8, 7, "Independence Day"),
            // A Sunday Christmas: "the 26th and 27th of December".
            (2022, 12, 26, "Boxing Day"),
            (2022, 12, 27, "Christmas Day"),
            (2021, 5, 24, "Labour Day"),
            // The Minister's 2021 appointment of the Monday after a Sunday
            // Boxing Day.
            (2021, 12, 27, "Boxing Day"),
            // 1997: Emancipation Day restored and Independence Day back on
            // 6 August; 1996's first Monday of August.
            (1997, 8, 1, "Emancipation Day"),
            (1997, 8, 6, "Independence Day"),
            (1996, 8, 5, "Independence Day"),
            (1969, 10, 20, "National Heroes' Day"),
            (1961, 5, 23, "Labour Day"),
        ],
    );
    expect_working(
        "JM",
        None,
        &[
            (2026, 8, 3),
            (2022, 8, 8),
            (2015, 12, 28),
            (2010, 12, 27),
            (1996, 8, 1),
            (1996, 8, 6),
            (1968, 10, 21),
            (1960, 5, 23),
        ],
    );
    expect_substitute("JM", None, 2021, (5, 23), (5, 24));
    expect_substitute("JM", None, 2022, (12, 25), (12, 27));
}

#[test]
fn trinidad_and_tobago_gives_the_next_free_day_for_a_sunday_or_a_coincidence() {
    expect(
        "TT",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 3, 20, "Eid-ul-Fitr"),
            (2026, 3, 30, "Spiritual Baptist Liberation Shouter Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 30, "Indian Arrival Day"),
            (2026, 6, 4, "Corpus Christi"),
            (2026, 6, 19, "Labour Day"),
            (2026, 8, 1, "African Emancipation Day"),
            (2026, 8, 31, "Independence Day"),
            (2026, 9, 24, "Republic Day"),
            (2026, 11, 8, "Divali"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 26, "Boxing Day"),
            // 2025 as announced: Eid on Monday 31 March and the Sunday
            // Spiritual Baptist day on the Tuesday after it; Labour Day and
            // Corpus Christi on one Thursday give the Friday.
            (2025, 3, 31, "Eid-ul-Fitr"),
            (2025, 4, 1, "Spiritual Baptist Liberation Shouter Day"),
            (2025, 6, 20, "Labour Day"),
            (2025, 9, 1, "Independence Day"),
            (2025, 10, 20, "Divali"),
            // Friday 31 May 2024, under section 3(2).
            (2024, 4, 10, "Eid-ul-Fitr"),
            (2024, 5, 31, "Indian Arrival Day"),
            (2024, 8, 1, "African Emancipation Day"),
            (2024, 10, 31, "Divali"),
            (2023, 8, 1, "Emancipation Day"),
            (2023, 9, 25, "Republic Day"),
            (2023, 11, 13, "Divali"),
            (1996, 3, 30, "Spiritual Baptist Liberation Shouter Day"),
            (1995, 5, 30, "Indian Arrival Day"),
            (1985, 8, 1, "Emancipation Day"),
        ],
    );
    // A Saturday holiday stays where it is; Carnival is no day off.
    expect_working(
        "TT",
        None,
        &[
            (2026, 6, 1),
            (2026, 2, 16),
            (2026, 2, 17),
            (2015, 6, 1),
            (1995, 3, 30),
            (1994, 5, 30),
            (1984, 8, 1),
        ],
    );
    expect_substitute("TT", None, 2024, (5, 30), (5, 31));
    expect_substitute("TT", None, 2025, (3, 30), (4, 1));
    let calendar = HolidayCalendar::for_year(table("TT"), None, 2026);
    let carnival: Vec<(&str, Kind)> = calendar
        .on(ymd(2026, 2, 16))
        .iter()
        .map(|holiday| (holiday.name, holiday.kind))
        .collect();
    assert_eq!(carnival, [("Carnival Monday", Kind::Observance)]);
}

#[test]
fn barbados_gives_the_tuesday_when_the_monday_is_taken() {
    expect(
        "BB",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 21, "Errol Barrow Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 4, 28, "National Heroes Day"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 25, "Whit Monday"),
            (2026, 8, 1, "Emancipation Day"),
            (2026, 8, 3, "Kadooment Day"),
            (2026, 11, 30, "Independence Day"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 26, "Boxing Day"),
            // 2022 as the Government's calendar had it: a Monday 1 August
            // shared with Kadooment Day and observed on the Tuesday, a
            // Sunday Christmas observed on the Tuesday after Boxing Day.
            (2022, 5, 2, "Labour Day"),
            (2022, 8, 1, "Emancipation Day"),
            (2022, 8, 1, "Kadooment Day"),
            (2022, 8, 2, "Emancipation Day"),
            (2022, 12, 26, "Boxing Day"),
            (2022, 12, 27, "Christmas Day"),
            // 2021: a Sunday 1 August, past Kadooment Day, to the Tuesday.
            (2021, 8, 2, "Kadooment Day"),
            (2021, 8, 3, "Emancipation Day"),
            (2021, 12, 27, "Boxing Day"),
            (1998, 4, 28, "National Heroes Day"),
            (1989, 1, 21, "Errol Barrow Day"),
        ],
    );
    expect_working(
        "BB",
        None,
        &[
            (2026, 8, 4),
            (2022, 8, 3),
            (2021, 12, 28),
            (1997, 4, 28),
            (1988, 1, 21),
        ],
    );
    expect_substitute("BB", None, 2022, (8, 1), (8, 2));
    expect_substitute("BB", None, 2021, (8, 1), (8, 3));
    expect_substitute("BB", None, 2022, (12, 25), (12, 27));
}

#[test]
fn bahamas_moves_weekend_holidays_to_the_next_free_weekday_and_nothing_else() {
    expect(
        "BS",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            // Saturday 10 January 2026, kept on the Monday.
            (2026, 1, 12, "Majority Rule Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 25, "Whit Monday"),
            (2026, 6, 5, "Randol Fawkes Labour Day"),
            (2026, 7, 10, "Independence Day"),
            (2026, 8, 3, "Emancipation Day"),
            (2026, 10, 12, "National Heroes Day"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 28, "Boxing Day"),
            // Wednesday 10 January 2024 was "the day celebrated as Majority
            // Rule Day in 2024" in the Governor-General's notice.
            (2024, 1, 10, "Majority Rule Day"),
            (2022, 1, 3, "New Year's Day"),
            (2022, 12, 27, "Christmas Day"),
            (2021, 7, 12, "Independence Day"),
            (2021, 12, 27, "Christmas Day"),
            (2021, 12, 28, "Boxing Day"),
            (2014, 1, 10, "Majority Rule Day"),
            (2014, 6, 6, "Randol Fawkes Labour Day"),
            (2013, 6, 7, "Labour Day"),
            (2013, 10, 14, "National Heroes Day"),
            (2012, 10, 12, "Discovery Day"),
        ],
    );
    expect_working(
        "BS",
        None,
        &[
            (2024, 1, 12),
            (2018, 7, 9),
            (2013, 1, 10),
            (2013, 10, 12),
            (2012, 10, 8),
        ],
    );
    expect_substitute("BS", None, 2026, (1, 10), (1, 12));
    expect_substitute("BS", None, 2021, (12, 26), (12, 28));
}

#[test]
fn uzbekistan_moves_a_weekend_holiday_to_the_next_working_day() {
    expect(
        "UZ",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            // A Sunday 8 March and a Saturday 21 March each give the Monday,
            // the latter as Decree No. 106 of 17 March 2026 announced.
            (2026, 3, 8, "Women's Day"),
            (2026, 3, 9, "Women's Day"),
            (2026, 3, 20, "Ruza Hayit"),
            (2026, 3, 21, "Navruz"),
            (2026, 3, 23, "Navruz"),
            (2026, 5, 11, "Day of Remembrance and Honour"),
            (2026, 5, 27, "Kurban Hayit"),
            (2026, 9, 1, "Independence Day"),
            (2026, 10, 1, "Teachers' and Mentors' Day"),
            (2026, 12, 8, "Constitution Day"),
            (1999, 5, 9, "Day of Remembrance and Honour"),
            (1998, 5, 9, "Victory Day"),
            (1997, 10, 1, "Teachers' and Mentors' Day"),
        ],
    );
    expect_working("UZ", None, &[(2026, 3, 24), (2026, 5, 12), (1996, 10, 1)]);
    expect_substitute("UZ", None, 2026, (3, 21), (3, 23));
    expect_substitute("UZ", None, 2026, (5, 9), (5, 11));
}

#[test]
fn kyrgyzstan_transferred_weekend_holidays_until_2024_and_keeps_two_holiday_weeks_since() {
    expect(
        "KG",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 2, "New Year Holidays"),
            (2026, 1, 6, "New Year Holidays"),
            (2026, 1, 7, "Orthodox Christmas"),
            (2026, 3, 8, "International Women's Day"),
            (2026, 3, 20, "Orozo Ait"),
            (2026, 3, 21, "Nooruz"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 2, "May Holidays"),
            (2026, 5, 5, "Constitution Day"),
            (2026, 5, 8, "May Holidays"),
            (2026, 5, 9, "Victory Day"),
            (2026, 5, 27, "Kurman Ait"),
            (2026, 8, 31, "Independence Day"),
            // The May holidays came with the 2025 code; the New Year ones
            // wait for 2026.
            (2025, 5, 2, "May Holidays"),
            // Under the 2004 code a Sunday 7 April and a Saturday 31 August
            // gave the Monday.
            (2024, 2, 23, "Defender of the Fatherland Day"),
            (2024, 4, 8, "Day of the People's April Revolution"),
            (2024, 9, 2, "Independence Day"),
            (
                2024,
                11,
                7,
                "Days of History and Commemoration of Ancestors",
            ),
            (
                2024,
                11,
                8,
                "Days of History and Commemoration of Ancestors",
            ),
            (
                2018,
                11,
                7,
                "Days of History and Commemoration of Ancestors",
            ),
            (2017, 11, 7, "Day of the Great October Socialist Revolution"),
            (2016, 4, 7, "Day of the People's April Revolution"),
        ],
    );
    // No transfer since 2025: a Sunday 8 March and a Saturday Nooruz stay
    // put; the demoted state holidays are working days.
    expect_working(
        "KG",
        None,
        &[
            (2026, 3, 9),
            (2026, 3, 23),
            (2026, 2, 23),
            (2026, 4, 7),
            (2025, 1, 2),
            (2025, 2, 24),
            (2025, 4, 7),
            (2025, 11, 7),
            (2024, 1, 2),
            (2024, 5, 2),
            (2017, 11, 8),
            (2015, 4, 7),
        ],
    );
    expect_substitute("KG", None, 2024, (4, 7), (4, 8));
    expect_substitute("KG", None, 2024, (8, 31), (9, 2));
    let calendar = HolidayCalendar::for_year(table("KG"), None, 2026);
    let kinds: Vec<Kind> = calendar
        .on(ymd(2026, 2, 23))
        .iter()
        .map(|holiday| holiday.kind)
        .collect();
    assert_eq!(kinds, [Kind::Observance]);
}

#[test]
fn tajikistan_runs_a_weekend_navruz_past_the_24th_and_dropped_1_may_in_2017() {
    expect(
        "TJ",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 3, 9, "Mother's Day"),
            (2026, 3, 20, "Idi Ramazon"),
            (2026, 3, 21, "Navruz"),
            (2026, 3, 24, "Navruz"),
            // Saturday and Sunday the 21st and 22nd give the 25th and 26th.
            (2026, 3, 25, "Navruz"),
            (2026, 3, 26, "Navruz"),
            (2026, 5, 11, "Victory Day"),
            (2026, 5, 27, "Idi Kurbon"),
            (2026, 6, 29, "National Unity Day"),
            (2026, 9, 9, "Independence Day"),
            (2026, 11, 6, "Constitution Day"),
            (2017, 9, 11, "Independence Day"),
            (2016, 5, 2, "Labour Day"),
        ],
    );
    expect_working(
        "TJ",
        None,
        &[(2026, 3, 27), (2026, 3, 10), (2017, 5, 1), (2017, 5, 2)],
    );
    expect_substitute("TJ", None, 2026, (3, 22), (3, 26));
    expect_substitute("TJ", None, 2026, (6, 27), (6, 29));
}

#[test]
fn turkmenistan_moves_a_sunday_holiday_only_and_moved_two_days_in_2018() {
    expect(
        "TM",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 3, 9, "International Women's Day"),
            // The decree of 16 March 2026: Oraza Bayram on the 20th and the
            // Sunday 22 March given Monday the 23rd.
            (2026, 3, 20, "Oraza Bayram"),
            (2026, 3, 21, "Nowruz"),
            (2026, 3, 22, "Nowruz"),
            (2026, 3, 23, "Nowruz"),
            (2026, 5, 18, "Constitution and State Flag Day"),
            // The decree of 22 May 2026: 27–29 May.
            (2026, 5, 27, "Kurban Bayram"),
            (2026, 5, 28, "Kurban Bayram"),
            (2026, 5, 29, "Kurban Bayram"),
            (2026, 9, 27, "Independence Day"),
            (2026, 9, 28, "Independence Day"),
            (2026, 10, 6, "Day of Remembrance"),
            (2026, 12, 12, "Neutrality Day"),
            // 2025 on the tabular calendar, a day behind the decrees' 30 March
            // and 6–8 June.
            (2025, 3, 31, "Oraza Bayram"),
            (2025, 6, 7, "Kurban Bayram"),
            (2025, 6, 9, "Kurban Bayram"),
            (2018, 5, 18, "Constitution and State Flag Day"),
            (2018, 9, 27, "Independence Day"),
            (2017, 2, 20, "State Flag Day"),
            (2017, 5, 18, "Constitution Day"),
            (2017, 10, 27, "Independence Day"),
        ],
    );
    // A Saturday holiday stays where it is.
    expect_working(
        "TM",
        None,
        &[(2026, 12, 14), (2026, 3, 24), (2018, 2, 19), (2017, 9, 27)],
    );
    expect_substitute("TM", None, 2026, (3, 22), (3, 23));
    expect_substitute("TM", None, 2026, (9, 27), (9, 28));
}

#[test]
fn algeria_keeps_two_eid_days_until_2022_and_three_since_with_community_days_religious() {
    expect(
        "DZ",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 12, "Yennayer"),
            (2026, 3, 20, "Eid al-Fitr"),
            (2026, 3, 22, "Eid al-Fitr"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 27, "Eid al-Adha"),
            (2026, 5, 29, "Eid al-Adha"),
            (2026, 6, 17, "Islamic New Year"),
            (2026, 6, 26, "Ashura"),
            (2026, 7, 5, "Independence Day"),
            (2026, 8, 26, "Mawlid"),
            (2026, 11, 1, "Revolution Day"),
            // 2022 on the tabular calendar, a day behind the notice's 9 and
            // 10 July: two days, not three.
            (2022, 7, 10, "Eid al-Adha"),
            (2022, 7, 11, "Eid al-Adha"),
            (2022, 5, 3, "Eid al-Fitr"),
            (2022, 5, 4, "Eid al-Fitr"),
            (2018, 1, 12, "Yennayer"),
        ],
    );
    expect_working(
        "DZ",
        None,
        &[(2022, 7, 12), (2022, 5, 5), (2017, 1, 12), (2026, 12, 25)],
    );
    let calendar = HolidayCalendar::for_year(table("DZ"), None, 2026);
    let community: Vec<(&str, Kind)> = [ymd(2026, 4, 6), ymd(2026, 9, 21), ymd(2026, 12, 25)]
        .iter()
        .flat_map(|day| calendar.on(*day))
        .map(|holiday| (holiday.name, holiday.kind))
        .collect();
    assert_eq!(
        community,
        [
            ("Easter Monday", Kind::Religious),
            ("Yom Kippur", Kind::Religious),
            ("Christmas Day", Kind::Religious),
        ]
    );
}

#[test]
fn tunisia_follows_the_decrees_from_1961_to_2021() {
    expect(
        "TN",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 3, 20, "Independence Day"),
            (2026, 3, 20, "Eid al-Fitr"),
            (2026, 3, 22, "Eid al-Fitr"),
            (2026, 4, 9, "Martyrs' Day"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 27, "Eid al-Adha"),
            (2026, 5, 28, "Eid al-Adha"),
            (2026, 6, 17, "Islamic New Year"),
            (2026, 7, 25, "Republic Day"),
            (2026, 8, 13, "Women's Day"),
            (2026, 8, 26, "Mouled"),
            (2026, 10, 15, "Evacuation Day"),
            (2026, 12, 17, "Revolution Day"),
            (2021, 1, 14, "Revolution and Youth Day"),
            (2021, 12, 17, "Revolution Day"),
            (2012, 1, 14, "Revolution and Youth Day"),
            (2010, 3, 21, "Youth Day"),
            (2010, 11, 7, "Commemoration of 7 November 1987"),
            (1990, 11, 7, "Commemoration of 7 November 1987"),
            (1988, 3, 21, "Youth Day"),
            (1987, 1, 18, "Revolution Day"),
            (1987, 6, 1, "Victory Day"),
            (1987, 8, 3, "President Bourguiba's Birthday"),
            (1987, 9, 3, "Commemoration of 3 September 1934"),
            (1966, 8, 13, "Women's Day"),
            (1964, 10, 15, "Evacuation Day"),
        ],
    );
    expect_working(
        "TN",
        None,
        &[
            (2026, 1, 14),
            (2025, 3, 21),
            (2026, 11, 7),
            (2026, 5, 29),
            (2020, 12, 17),
            (2011, 1, 14),
            (2011, 3, 21),
            (2012, 11, 7),
            (1989, 11, 7),
            (1988, 1, 18),
            (1988, 6, 1),
            (1987, 3, 21),
            (1965, 8, 13),
            (1963, 10, 15),
        ],
    );
}

#[test]
fn senegal_gives_the_monday_after_a_sunday_korite_or_tabaski_only() {
    expect(
        "SN",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 3, 20, "Korité"),
            (2026, 4, 4, "Independence Day"),
            (2026, 4, 5, "Easter Sunday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 14, "Ascension"),
            (2026, 5, 24, "Pentecost"),
            (2026, 5, 25, "Whit Monday"),
            (2026, 5, 27, "Tabaski"),
            (2026, 6, 26, "Tamkharit"),
            // The tabular 18 Safar; Senegal saw the moon a day earlier.
            (2026, 8, 3, "Grand Magal of Touba"),
            (2026, 8, 15, "Assumption"),
            (2026, 8, 26, "Maouloud"),
            (2026, 11, 1, "All Saints' Day"),
            (2026, 12, 25, "Christmas Day"),
            (2025, 8, 13, "Grand Magal of Touba"),
            // The first Magal kept as a day off, 12 January 2012, on the tabular
            // calendar a day later.
            (2012, 1, 13, "Grand Magal of Touba"),
        ],
    );
    // A Sunday Tamkharit, All Saints' Day or Independence Day stays.
    expect_working(
        "SN",
        None,
        &[(2025, 7, 7), (2026, 11, 2), (2027, 4, 5), (2011, 1, 24)],
    );
    // On the tabular calendar Korité fell on Sunday 24 May 2020 and Tabaski
    // on Sunday 10 July 2022; each gave the Monday.
    expect_substitute("SN", None, 2020, (5, 24), (5, 25));
    expect_substitute("SN", None, 2022, (7, 10), (7, 11));
}

#[test]
fn cote_d_ivoire_gives_the_day_after_five_sunday_feasts_since_2011() {
    expect(
        "CI",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 3, 16, "Day after the Night of Destiny"),
            (2026, 3, 20, "Eid al-Fitr"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 14, "Ascension"),
            (2026, 5, 25, "Whit Monday"),
            (2026, 5, 27, "Tabaski"),
            (2026, 8, 7, "Independence Day"),
            (2026, 8, 15, "Assumption"),
            (2026, 8, 26, "Day after the Prophet's Birthday"),
            (2026, 11, 1, "All Saints' Day"),
            (2026, 11, 15, "National Peace Day"),
            (2026, 12, 25, "Christmas Day"),
            // 2011, the decree's year: a Sunday Labour Day, Independence Day
            // and Christmas each gave the Monday.
            (2011, 5, 2, "Labour Day"),
            (2011, 8, 8, "Independence Day"),
            (2011, 12, 26, "Christmas Day"),
            (1996, 11, 15, "National Peace Day"),
        ],
    );
    // A Sunday All Saints' Day or Assumption stays; before 2011 nothing moved.
    expect_working(
        "CI",
        None,
        &[
            (2026, 11, 2),
            (2026, 11, 16),
            (2010, 8, 16),
            (2010, 12, 27),
            (1995, 11, 15),
        ],
    );
    expect_substitute("CI", None, 2011, (12, 25), (12, 26));
}

#[test]
fn cameroon_gives_the_next_day_for_a_civil_holiday_on_a_sunday_or_a_holiday() {
    expect(
        "CM",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 2, 11, "Youth Day"),
            (2026, 3, 20, "Eid al-Fitr"),
            (2026, 4, 3, "Good Friday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 14, "Ascension"),
            (2026, 5, 20, "National Day"),
            (2026, 5, 27, "Eid al-Adha"),
            (2026, 8, 15, "Assumption"),
            (2026, 12, 25, "Christmas Day"),
            // Sunday 20 May and 11 February 2018, Sunday 1 January 2023.
            (2018, 5, 21, "National Day"),
            (2018, 2, 12, "Youth Day"),
            (2023, 1, 2, "New Year's Day"),
        ],
    );
    expect_substitute("CM", None, 2018, (5, 20), (5, 21));
    // Ascension fell on 1 May in 2008 and on 20 May in 2004.
    expect_substitute("CM", None, 2008, (5, 1), (5, 2));
    expect_substitute("CM", None, 2004, (5, 20), (5, 21));
    // A Sunday religious holiday stays; Easter Monday is not a holiday;
    // before the 1973 law, nothing is claimed to move.
    expect_working(
        "CM",
        None,
        &[(2022, 12, 26), (2021, 8, 16), (2026, 4, 6), (1973, 5, 21)],
    );
}

#[test]
fn the_republic_of_the_congo_keeps_law_2_94_and_moves_nothing() {
    expect(
        "CG",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 14, "Ascension"),
            (2026, 5, 25, "Whit Monday"),
            (2026, 6, 10, "Sovereign National Conference Day"),
            (2026, 8, 15, "National Day"),
            (2026, 11, 1, "All Saints' Day"),
            (2026, 12, 25, "Christmas Day"),
            (2025, 4, 21, "Easter Monday"),
            (2025, 5, 29, "Ascension"),
            (2025, 6, 9, "Whit Monday"),
        ],
    );
    // Good Friday and 28 November are not in the law; Sunday All Saints'
    // Day 2026 and Sunday Christmas 2022 stay.
    expect_working(
        "CG",
        None,
        &[(2026, 4, 3), (2025, 11, 28), (2026, 11, 2), (2022, 12, 26)],
    );
}

#[test]
fn the_democratic_republic_of_the_congo_takes_a_sunday_holiday_the_day_before_until_2025() {
    expect(
        "CD",
        None,
        &[
            (2024, 1, 1, "New Year's Day"),
            (2024, 1, 4, "Martyrs of Independence Day"),
            (2024, 1, 16, "Laurent-Désiré Kabila Day"),
            (2024, 1, 17, "Patrice Lumumba Day"),
            (2024, 4, 6, "Simon Kimbangu Day"),
            (2024, 5, 1, "Labour Day"),
            (2024, 5, 17, "Armed Forces Day"),
            (2024, 6, 30, "Independence Day"),
            (2024, 8, 1, "Parents' Day"),
            (2024, 12, 25, "Christmas Day"),
            (2023, 4, 6, "Simon Kimbangu Day"),
            // Sunday 30 June 2024 and Sunday 6 April 2025.
            (2024, 6, 29, "Independence Day"),
            (2025, 4, 5, "Simon Kimbangu Day"),
        ],
    );
    expect_substitute("CD", None, 2024, (6, 30), (6, 29));
    expect_substitute("CD", None, 2025, (4, 6), (4, 5));
    // 6 April is a holiday from 2023; the public services' Friday of 2025
    // is not carried.
    expect_working("CD", None, &[(2022, 4, 6), (2025, 4, 4)]);
}

#[test]
fn the_democratic_republic_of_the_congo_follows_the_ministers_communiques_from_2025() {
    expect(
        "CD",
        None,
        &[
            // 2025: a Saturday holiday brought forward to the Friday.
            (2025, 1, 3, "Martyrs of Independence Day"),
            (2025, 5, 16, "Armed Forces Day"),
            // 2026: every weekend holiday to the Monday.
            (2026, 1, 5, "Martyrs of Independence Day"),
            (2026, 1, 19, "Patrice Lumumba Day"),
            (2026, 5, 18, "Armed Forces Day"),
            (2026, 8, 3, "Parents' Day"),
        ],
    );
    // The Saturday the ordinance would give in 2026 was withdrawn.
    expect_working("CD", None, &[(2026, 1, 3), (2026, 5, 16)]);
    let read = HolidayCalendar::for_year(table("CD"), None, 2026);
    assert!(read.is_complete(), "{:?}", read.gaps());
    // No communiqué for 2027 was read, so its moves are a gap.
    let unread = HolidayCalendar::for_year(table("CD"), None, 2027);
    assert!(
        unread
            .gaps()
            .iter()
            .any(|gap| gap.name == "Weekend holiday moved by communiqué"),
        "{:?}",
        unread.gaps()
    );
}

#[test]
fn angola_moved_a_sunday_holiday_until_2018_and_bridges_tuesdays_and_thursdays_since() {
    expect(
        "AO",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 2, "Bridge day"),
            (2026, 2, 4, "Liberation War Day"),
            (2026, 2, 16, "Bridge day"),
            (2026, 2, 17, "Carnival"),
            (2026, 3, 8, "International Women's Day"),
            (2026, 3, 23, "Southern Africa Liberation Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 4, "Peace and National Reconciliation Day"),
            (2026, 5, 1, "Labour Day"),
            (2026, 9, 17, "National Heroes' Day"),
            (2026, 9, 18, "Bridge day"),
            (2026, 11, 2, "All Souls' Day"),
            (2026, 11, 11, "Independence Day"),
            (2026, 12, 25, "Christmas and Family Day"),
            // Tuesday 23 March 2021.
            (2021, 3, 22, "Bridge day"),
            // Sunday 4 February 2018, before law 11/18; Christmas on
            // Tuesday 25 December 2018, after it.
            (2018, 2, 5, "Liberation War Day"),
            (2018, 12, 24, "Bridge day"),
        ],
    );
    expect_substitute("AO", None, 2017, (9, 17), (9, 18));
    expect_working(
        "AO",
        None,
        &[
            // Sunday 8 March 2026 and Sunday 11 November 2018 stay.
            (2026, 3, 9),
            (2018, 11, 12),
            // Tuesday 1 May 2018 was before the bridges.
            (2018, 4, 30),
            // Christmas was excepted from the Sunday rule.
            (2016, 12, 26),
            // 23 March is a holiday from 2019; 14 April is a celebration
            // date without a day off.
            (2018, 3, 23),
            (2026, 4, 14),
        ],
    );
}

#[test]
fn benin_moves_the_traditional_religions_to_a_thursday_and_friday_from_2025() {
    expect(
        "BJ",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 8, "Eve of Traditional Religions Day"),
            (2026, 1, 9, "Traditional Religions Day"),
            (2026, 3, 20, "Eid al-Fitr"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 14, "Ascension"),
            (2026, 5, 25, "Whit Monday"),
            (2026, 5, 27, "Tabaski"),
            (2026, 8, 1, "National Day"),
            (2026, 8, 15, "Assumption"),
            (2026, 8, 26, "Maouloud"),
            (2026, 11, 1, "All Saints' Day"),
            (2026, 12, 25, "Christmas Day"),
            (2025, 1, 9, "Eve of Traditional Religions Day"),
            (2025, 1, 10, "Traditional Religions Day"),
            (2024, 1, 10, "Traditional Religions Day"),
            (1998, 1, 10, "Traditional Religions Day"),
        ],
    );
    // Before law 97-031 there was no such day; the national days of
    // article 3 are not days off; a Sunday holiday stays on the Sunday.
    expect_working(
        "BJ",
        None,
        &[
            (1997, 1, 10),
            (2026, 1, 16),
            (2025, 2, 28),
            (2026, 3, 9),
            (2026, 11, 2),
        ],
    );
}

#[test]
fn burkina_faso_gave_a_sundays_monday_until_the_2026_law() {
    expect(
        "BF",
        None,
        &[
            (2025, 1, 1, "New Year's Day"),
            (2025, 1, 3, "Popular Uprising Day"),
            (2025, 3, 8, "International Women's Day"),
            (2025, 3, 31, "Eid al-Fitr"),
            (2025, 5, 1, "Labour Day"),
            (2025, 5, 15, "Customs and Traditions Day"),
            (2025, 5, 29, "Ascension"),
            (2025, 6, 7, "Tabaski"),
            (2025, 8, 5, "Independence Day"),
            (2025, 8, 15, "Assumption"),
            (2025, 9, 5, "Mouloud"),
            (2025, 10, 31, "National Martyrs' Day"),
            (2025, 11, 1, "All Saints' Day"),
            (2025, 12, 11, "National Day"),
            (2025, 12, 25, "Christmas Day"),
            // Six days before the new law was adopted.
            (2026, 1, 3, "Popular Uprising Day"),
            (2026, 5, 15, "Customs and Traditions Day"),
            (2026, 5, 27, "Tabaski"),
            (2026, 8, 26, "Mouloud"),
            (2026, 12, 11, "National Day"),
        ],
    );
    // Easter Sunday's Monday, and a Sunday Popular Uprising Day and
    // Christmas, under article 2 of the 2015 law.
    expect_substitute("BF", None, 2025, (4, 20), (4, 21));
    expect_substitute("BF", None, 2016, (1, 3), (1, 4));
    expect_substitute("BF", None, 2022, (12, 25), (12, 26));
    // The 2026 law made these commemorations and dropped Easter; 15 May
    // was not a holiday before 2024, nor 31 October before the 2015 law.
    expect_working(
        "BF",
        None,
        &[
            (2026, 8, 5),
            (2027, 11, 1),
            (2027, 1, 4),
            (2027, 3, 29),
            (2023, 5, 15),
            (2015, 10, 31),
        ],
    );
    // Whether a Sunday holiday still gave the Monday in 2026 depends on
    // the new law's promulgation date, which was not read.
    let transition = HolidayCalendar::for_year(table("BF"), None, 2026);
    assert!(
        transition
            .gaps()
            .iter()
            .any(|gap| gap.name == "Day after a Sunday holiday")
    );
    assert!(HolidayCalendar::for_year(table("BF"), None, 2025).is_complete());
    assert!(HolidayCalendar::for_year(table("BF"), None, 2027).is_complete());
}

#[test]
fn cabo_verde_keeps_law_16_iv_91_and_13_january_from_2020() {
    expect(
        "CV",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 13, "Freedom and Democracy Day"),
            (2026, 1, 20, "Nationality and National Heroes' Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 5, 1, "Workers' Day"),
            (2026, 7, 5, "Independence Day"),
            (2026, 8, 15, "Assumption"),
            (2026, 11, 1, "All Saints' Day"),
            (2026, 12, 25, "Christmas Day"),
            (2025, 1, 13, "Freedom and Democracy Day"),
            (2025, 4, 18, "Good Friday"),
            (2020, 1, 13, "Freedom and Democracy Day"),
        ],
    );
    // Children's Day is for schools; a Sunday Independence Day stays; no
    // Easter Monday.
    expect_working("CV", None, &[(2026, 6, 1), (2026, 7, 6), (2026, 4, 6)]);
    let children = HolidayCalendar::for_year(table("CV"), None, 2026);
    assert!(
        children
            .on(ymd(2026, 6, 1))
            .iter()
            .any(|holiday| holiday.kind == Kind::School)
    );
    // The years before any source read calls 13 January a holiday are a gap.
    let before = HolidayCalendar::for_year(table("CV"), None, 2016);
    assert!(
        before
            .gaps()
            .iter()
            .any(|gap| gap.name == "Freedom and Democracy Day")
    );
    assert!(HolidayCalendar::for_year(table("CV"), None, 2020).is_complete());
}

#[test]
fn guinea_moves_only_three_holidays_to_the_next_working_day_from_2023() {
    expect(
        "GN",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 3, 16, "Day after the Night of Destiny"),
            (2026, 3, 20, "Eid al-Fitr"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 25, "Africa Day"),
            (2026, 5, 27, "Tabaski"),
            (2026, 5, 28, "Day after Tabaski"),
            (2026, 8, 15, "Assumption"),
            (2026, 8, 26, "Day after the Prophet's Birthday"),
            (2026, 10, 2, "Independence Day"),
            (2026, 12, 25, "Christmas Day"),
            (2025, 10, 2, "Independence Day"),
            (2025, 6, 7, "Tabaski"),
        ],
    );
    expect_substitute("GN", None, 2023, (1, 1), (1, 2));
    expect_substitute("GN", None, 2027, (10, 2), (10, 4));
    // A Saturday Christmas stays, and before the decree nothing moved.
    expect_working("GN", None, &[(2027, 12, 27), (2022, 10, 3)]);
}

#[test]
fn mali_keeps_both_days_of_maouloud_and_no_declared_extras() {
    expect(
        "ML",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 14, "Day of Recovered Sovereignty"),
            (2026, 1, 20, "Armed Forces Day"),
            (2026, 3, 20, "Eid al-Fitr"),
            (2026, 3, 26, "Martyrs' Day"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 25, "Africa Day"),
            (2026, 5, 27, "Tabaski"),
            // The Ministry announced 25 and 31 August for the sighted dates.
            (2026, 8, 26, "Prophet's Birthday"),
            (2026, 9, 1, "Prophet's Baptism"),
            (2026, 9, 22, "Independence Day"),
            (2026, 12, 25, "Christmas Day"),
            (2025, 1, 14, "Day of Recovered Sovereignty"),
            (2025, 9, 11, "Prophet's Baptism"),
            (2025, 9, 22, "Independence Day"),
        ],
    );
    // 14 January before 2023; the day after New Year declared for 2026 and
    // Achoura, both decisions of the year, are not carried; a Sunday 22
    // September stays.
    expect_working(
        "ML",
        None,
        &[(2022, 1, 14), (2026, 1, 2), (2026, 6, 26), (2024, 9, 23)],
    );
}

#[test]
fn cuba_moves_the_sunday_rest_for_1_may_and_10_october_only() {
    expect(
        "CU",
        None,
        &[
            (2026, 1, 1, "Triumph of the Revolution"),
            (2026, 1, 2, "Victory Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 5, 1, "International Workers' Day"),
            (2026, 7, 25, "Day before National Rebellion Day"),
            (2026, 7, 26, "National Rebellion Day"),
            (2026, 7, 27, "Day after National Rebellion Day"),
            (2026, 10, 10, "Beginning of the Wars of Independence"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 31, "New Year's Eve"),
            (2022, 5, 2, "International Workers' Day"),
            (2021, 10, 11, "Beginning of the Wars of Independence"),
            (2012, 4, 6, "Good Friday"),
            (1998, 12, 25, "Christmas Day"),
        ],
    );
    // A Sunday 26 July, 1 January or Christmas gives nothing; 2011 had no
    // Good Friday and 1997 no Christmas.
    expect_working(
        "CU",
        None,
        &[
            (2026, 7, 28),
            (2023, 1, 3),
            (2022, 12, 26),
            (2011, 4, 22),
            (1997, 12, 25),
        ],
    );
    expect_substitute("CU", None, 2022, (5, 1), (5, 2));
    expect_substitute("CU", None, 2021, (10, 10), (10, 11));
}

#[test]
fn belize_follows_the_governments_notices_from_2022_to_2026() {
    expect(
        "BZ",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 15, "George Price Day"),
            (2026, 3, 9, "National Heroes and Benefactors Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 4, "Holy Saturday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 8, 1, "Emancipation Day"),
            (2026, 9, 10, "St. George's Caye Day"),
            (2026, 9, 21, "Independence Day"),
            (2026, 10, 12, "Indigenous Peoples' Resistance Day"),
            (2026, 11, 19, "Garifuna Settlement Day"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 26, "Boxing Day"),
            // 2025: a Wednesday George Price Day and St. George's Caye Day
            // stayed; the Sundays gave Mondays.
            (2025, 1, 15, "George Price Day"),
            (2025, 3, 10, "National Heroes and Benefactors Day"),
            (2025, 9, 10, "St. George's Caye Day"),
            (2025, 9, 22, "Independence Day"),
            (2025, 10, 13, "Indigenous Peoples' Resistance Day"),
            // 2024: Saturdays to the Monday after, a Wednesday Labour Day and
            // a Tuesday St. George's Caye Day to the Monday before, a Thursday
            // Emancipation Day and a Tuesday Garifuna Settlement Day stayed.
            (2024, 3, 11, "National Heroes and Benefactors Day"),
            (2024, 4, 29, "Labour Day"),
            (2024, 8, 1, "Emancipation Day"),
            (2024, 9, 9, "St. George's Caye Day"),
            (2024, 10, 14, "Indigenous Peoples' Resistance Day"),
            (2024, 11, 19, "Garifuna Settlement Day"),
            // 2023: Thursdays to the Monday before, a Tuesday Emancipation
            // Day too, and Sundays to the Monday after.
            (2023, 1, 2, "New Year's Day"),
            (2023, 1, 16, "George Price Day"),
            (2023, 3, 6, "National Heroes and Benefactors Day"),
            (2023, 7, 31, "Emancipation Day"),
            (2023, 9, 11, "St. George's Caye Day"),
            (2023, 10, 9, "Indigenous Peoples' Resistance Day"),
            (2023, 11, 20, "Garifuna Settlement Day"),
            // 2022: Wednesdays to the Monday before; a Sunday Christmas got
            // no Monday.
            (2022, 3, 7, "National Heroes and Benefactors Day"),
            (2022, 5, 2, "Labour Day"),
            (2022, 10, 10, "Indigenous Peoples' Resistance Day"),
            (2022, 12, 26, "Boxing Day"),
            (2021, 10, 11, "Pan American Day"),
            (2020, 5, 25, "Commonwealth Day"),
        ],
    );
    expect_working(
        "BZ",
        None,
        &[
            (2026, 12, 28),
            (2026, 8, 3),
            (2025, 1, 13),
            (2025, 9, 8),
            (2024, 5, 1),
            (2024, 8, 5),
            (2022, 12, 27),
            (2022, 5, 24),
            (2020, 1, 15),
            (2020, 8, 1),
        ],
    );
    expect_substitute("BZ", None, 2025, (9, 21), (9, 22));
    expect_substitute("BZ", None, 2023, (9, 10), (9, 11));
}

#[test]
fn guyana_gives_the_following_day_for_a_sunday_and_the_tuesday_after_a_sunday_christmas() {
    expect(
        "GY",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 2, 23, "Republic Day"),
            (2026, 3, 3, "Phagwah"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 5, "Arrival Day"),
            (2026, 5, 26, "Independence Day"),
            (2026, 5, 27, "Eid-ul-Azha"),
            (2026, 7, 6, "CARICOM Day"),
            (2026, 8, 1, "Emancipation Day"),
            (2026, 8, 26, "Youman Nabi"),
            (2026, 11, 8, "Deepavali"),
            (2026, 11, 9, "Deepavali"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 26, "Boxing Day"),
            // 2024 as the lists had it: Phagwah on 25 March, Deepavali on
            // 31 October, a Sunday Arrival Day and Independence Day on the
            // Mondays.
            (2024, 3, 25, "Phagwah"),
            (2024, 5, 6, "Arrival Day"),
            (2024, 5, 27, "Independence Day"),
            (2024, 10, 31, "Deepavali"),
            (2022, 12, 26, "Boxing Day"),
            (2022, 12, 27, "Christmas Day"),
            (2004, 5, 5, "Arrival Day"),
            (1970, 2, 23, "Republic Day"),
        ],
    );
    expect_working(
        "GY",
        None,
        &[(2026, 8, 3), (2022, 12, 28), (2003, 5, 5), (1969, 2, 24)],
    );
    expect_substitute("GY", None, 2024, (5, 5), (5, 6));
    expect_substitute("GY", None, 2022, (12, 25), (12, 27));
}

#[test]
fn haiti_has_the_constitutions_five_days_and_the_decrees_of_1989_and_2024() {
    expect(
        "HT",
        None,
        &[
            (2026, 1, 1, "Independence Day"),
            (2026, 1, 2, "Ancestors' Day"),
            (2026, 2, 16, "Carnival Monday"),
            (2026, 2, 17, "Carnival Tuesday"),
            (2026, 4, 3, "Good Friday"),
            (2026, 5, 1, "Labour and Agriculture Day"),
            (2026, 5, 18, "Flag and University Day"),
            (2026, 6, 4, "Corpus Christi"),
            (2026, 8, 14, "Bois-Caïman Day"),
            (2026, 8, 15, "Assumption"),
            (2026, 9, 20, "Dessalines Day"),
            (2026, 10, 17, "Death of Dessalines"),
            (2026, 11, 1, "All Saints' Day"),
            (2026, 11, 2, "All Souls' Day"),
            (2026, 11, 18, "Battle of Vertières Day"),
            (2026, 12, 25, "Christmas Day"),
            (1989, 5, 25, "Corpus Christi"),
            (1989, 8, 15, "Assumption"),
            (1989, 10, 17, "Death of Dessalines"),
            (1988, 2, 15, "Carnival Monday"),
            (1988, 11, 2, "All Souls' Day"),
        ],
    );
    expect_working(
        "HT",
        None,
        &[
            (2024, 8, 14),
            (2024, 9, 20),
            (2024, 11, 1),
            (1988, 8, 15),
            (1988, 10, 17),
        ],
    );
    let calendar = HolidayCalendar::for_year(table("HT"), None, 2026);
    let kinds: Vec<Kind> = calendar
        .on(ymd(2026, 2, 16))
        .iter()
        .map(|holiday| holiday.kind)
        .collect();
    assert_eq!(kinds, [Kind::Bank]);
}

#[test]
fn antigua_and_barbuda_follows_the_2005_and_2019_schedules() {
    expect(
        "AG",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 4, "Labour Day"),
            (2026, 5, 25, "Whit Monday"),
            (2026, 8, 3, "Carnival Monday"),
            (2026, 8, 4, "Carnival Tuesday"),
            (2026, 11, 2, "Independence Day"),
            (2026, 12, 9, "Sir Vere Cornwall Bird Snr. Day"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 26, "Boxing Day"),
            // A Saturday Christmas under the 2019 Schedule: the Monday and
            // Tuesday, the Tuesday for the Sunday Boxing Day.
            (2021, 12, 27, "Christmas Day"),
            (2021, 12, 28, "Boxing Day"),
            // A Sunday Christmas: Boxing Day and the Tuesday.
            (2022, 12, 27, "Christmas Day"),
            (2013, 12, 9, "National Heroes Day"),
            (2014, 12, 9, "Sir Vere Cornwall Bird Snr. Day"),
            (2006, 1, 2, "New Year's Day"),
        ],
    );
    // Under the 2005 Schedule a Saturday Christmas gave the Monday only,
    // and 9 December did not move off a Saturday; nothing before 2006.
    expect_working("AG", None, &[(2010, 12, 28), (2017, 12, 11), (2005, 11, 1)]);
    expect_substitute("AG", None, 2026, (11, 1), (11, 2));
    expect_substitute("AG", None, 2021, (12, 25), (12, 27));
    expect_substitute("AG", None, 2021, (12, 26), (12, 28));
    expect_substitute("AG", None, 2022, (12, 25), (12, 27));
    expect_substitute("AG", None, 2023, (12, 9), (12, 11));
}

#[test]
fn dominica_moves_a_sunday_holiday_to_the_next_free_day() {
    expect(
        "DM",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 2, 16, "Carnival Monday"),
            (2026, 2, 17, "Carnival Tuesday"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 4, "Labour Day"),
            (2026, 5, 25, "Whit Monday"),
            (2026, 8, 3, "Emancipation Day"),
            (2026, 11, 3, "Independence Day"),
            (2026, 11, 4, "National Day of Community Service"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 26, "Boxing Day"),
            (2023, 1, 2, "New Year's Day"),
            (2024, 11, 5, "Independence Day"),
            (2022, 12, 27, "Christmas Day"),
            (2021, 12, 27, "Boxing Day"),
        ],
    );
    // A Saturday stays: Community Day 2023, Boxing Day 2026.
    expect_working("DM", None, &[(2023, 11, 6), (2026, 12, 28), (2025, 5, 1)]);
    expect_substitute("DM", None, 2024, (11, 3), (11, 5));
    expect_substitute("DM", None, 2023, (1, 1), (1, 2));
}

#[test]
fn grenada_gives_only_the_monday_after_a_sunday() {
    expect(
        "GD",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 2, 7, "Independence Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 25, "Whit Monday"),
            (2026, 6, 4, "Corpus Christi"),
            (2026, 8, 1, "Emancipation Day"),
            (2026, 8, 10, "Carnival Monday"),
            (2026, 8, 11, "Carnival Tuesday"),
            (2026, 10, 19, "National Heroes' Day"),
            (2026, 10, 26, "Thanksgiving Day"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 26, "Boxing Day"),
            (2025, 10, 20, "National Heroes' Day"),
            (2024, 8, 5, "Emancipation Holiday"),
            (2024, 8, 13, "Carnival Tuesday"),
            (2022, 5, 2, "Labour Day"),
        ],
    );
    // A Sunday Christmas adds nothing to Boxing Day; the first Monday of
    // August is no longer a holiday from 2025; 19 October before 2025 is
    // not carried.
    expect_working("GD", None, &[(2022, 12, 27), (2025, 8, 4), (2024, 10, 19)]);
    expect_substitute("GD", None, 2025, (10, 19), (10, 20));
    let calendar = HolidayCalendar::for_year(table("GD"), None, 2023);
    let kinds: Vec<Kind> = calendar
        .on(ymd(2023, 8, 15))
        .iter()
        .map(|holiday| holiday.kind)
        .collect();
    assert_eq!(kinds, [Kind::Bank]);
}

#[test]
fn saint_kitts_and_nevis_moves_a_sunday_to_the_monday() {
    expect(
        "KN",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 4, "Labour Day"),
            (2026, 5, 25, "Whit Monday"),
            (2026, 8, 3, "Emancipation Day"),
            (2026, 9, 16, "National Heroes Day"),
            (2026, 9, 19, "Independence Day"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 26, "Boxing Day"),
            (2021, 9, 20, "Independence Day"),
            (2021, 12, 27, "Boxing Day"),
            (2022, 12, 26, "Boxing Day"),
            (2022, 12, 27, "Christmas Day"),
        ],
    );
    // A Saturday Independence Day stays; Carnival Day is not carried.
    expect_working("KN", None, &[(2026, 9, 21), (2026, 1, 2)]);
    expect_substitute("KN", None, 2021, (9, 19), (9, 20));
    expect_substitute("KN", None, 2022, (12, 25), (12, 27));
}

#[test]
fn saint_lucia_moves_a_sunday_past_the_next_holiday() {
    expect(
        "LC",
        None,
        &[
            (2025, 1, 1, "New Year's Day"),
            (2025, 1, 2, "New Year's Holiday"),
            (2025, 2, 22, "Independence Day"),
            (2025, 4, 18, "Good Friday"),
            (2025, 4, 21, "Easter Monday"),
            (2025, 5, 1, "Labour Day"),
            (2025, 6, 9, "Whit Monday"),
            (2025, 6, 19, "Corpus Christi"),
            (2025, 8, 1, "Emancipation Day"),
            (2025, 10, 6, "Thanksgiving Day"),
            (2025, 12, 13, "National Day"),
            (2025, 12, 25, "Christmas Day"),
            (2025, 12, 26, "Boxing Day"),
            (2023, 1, 3, "New Year's Day"),
            (2022, 12, 27, "Christmas Day"),
        ],
    );
    // A Saturday stays: National Day 2025.
    expect_working("LC", None, &[(2025, 12, 15), (2025, 7, 14)]);
    expect_substitute("LC", None, 2023, (1, 1), (1, 3));
    expect_substitute("LC", None, 2022, (1, 2), (1, 3));
}

#[test]
fn saint_vincent_follows_the_prime_ministers_lists() {
    expect(
        "VC",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 3, 14, "National Heroes' Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 1, "National Workers' Day"),
            (2026, 5, 21, "Spiritual Baptist Liberation Day"),
            (2026, 5, 25, "Whit Monday"),
            (2026, 7, 6, "Carnival Monday"),
            (2026, 7, 7, "Carnival Tuesday"),
            (2026, 8, 1, "Emancipation Day"),
            (2026, 10, 27, "Independence Day"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 26, "Boxing Day"),
            (2021, 9, 6, "Carnival Monday"),
            (2021, 9, 7, "Carnival Tuesday"),
            (2024, 10, 28, "Independence Day"),
            (2022, 12, 27, "Christmas Day"),
            (2023, 7, 11, "Carnival Tuesday"),
        ],
    );
    // A Saturday stays; Spiritual Baptist Liberation Day before 2025 is
    // not carried; 2021's Carnival was not in July.
    expect_working("VC", None, &[(2026, 3, 16), (2024, 5, 21), (2021, 7, 5)]);
    expect_substitute("VC", None, 2021, (8, 1), (8, 2));
    expect_substitute("VC", None, 2022, (12, 25), (12, 27));
}

#[test]
fn suriname_keeps_the_decrees_free_days_and_moves_none() {
    expect(
        "SR",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 2, 17, "Chinese New Year"),
            (2026, 3, 3, "Holi Phagwa"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 7, 1, "Keti Koti"),
            (2026, 8, 9, "Indigenous People's Day"),
            (2026, 10, 10, "Day of the Maroons"),
            (2026, 11, 25, "Independence Day"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 26, "Boxing Day"),
            // The Minister's announcements.
            (2024, 3, 25, "Holi Phagwa"),
            (2024, 2, 10, "Chinese New Year"),
            (2023, 1, 22, "Chinese New Year"),
            (2024, 4, 10, "Eid al-Fitr"),
            (2023, 6, 29, "Eid al-Adha"),
            (2021, 11, 4, "Divali"),
            (2022, 10, 24, "Divali"),
            (2023, 11, 12, "Divali"),
            (2020, 2, 25, "Day of Liberation and Renewal"),
            (2007, 8, 9, "Indigenous People's Day"),
        ],
    );
    // Nothing moves off a weekend; 25 February ends in 2020; Divali, the
    // Maroons' day and 9 August before their decrees are not carried.
    expect_working(
        "SR",
        None,
        &[
            (2023, 11, 13),
            (2023, 1, 23),
            (2021, 2, 25),
            (2011, 10, 10),
            (2006, 8, 9),
        ],
    );
    let calendar = HolidayCalendar::for_year(table("SR"), None, 2024);
    assert!(
        calendar
            .on(ymd(2024, 4, 10))
            .iter()
            .all(|holiday| holiday.confidence == Confidence::Approximate)
    );
}

#[test]
fn venezuela_has_the_lottt_days_and_the_five_fiestas_nacionales_and_moves_none() {
    expect(
        "VE",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 2, 16, "Carnival Monday"),
            (2026, 2, 17, "Carnival Tuesday"),
            (2026, 4, 2, "Maundy Thursday"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 19, "Declaration of Independence"),
            (2026, 5, 1, "Labour Day"),
            (2026, 6, 24, "Battle of Carabobo"),
            (2026, 7, 5, "Independence Day"),
            (2026, 7, 24, "Birthday of Simón Bolívar"),
            (2026, 10, 12, "Day of Indigenous Resistance"),
            (2026, 12, 24, "Christmas Eve"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 31, "New Year's Eve"),
            (2025, 3, 3, "Carnival Monday"),
            (2025, 3, 4, "Carnival Tuesday"),
            (2025, 4, 17, "Maundy Thursday"),
            (2025, 4, 18, "Good Friday"),
            (2002, 10, 12, "Day of Indigenous Resistance"),
            (2001, 10, 12, "Day of the Race"),
        ],
    );
    // A Sunday 5 July 2026 gives no Monday; Easter Monday and Flag Day are
    // working days.
    expect_working("VE", None, &[(2026, 7, 6), (2026, 4, 6), (2026, 8, 3)]);
}

#[test]
fn paraguay_follows_ley_7544_and_the_2026_decrees_that_moved_three_days() {
    expect(
        "PY",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 3, 2, "Heroes' Day"),
            (2026, 4, 2, "Maundy Thursday"),
            (2026, 4, 3, "Good Friday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 14, "Independence Day"),
            (2026, 5, 15, "Independence Day"),
            (2026, 6, 12, "Chaco Peace Day"),
            (2026, 6, 22, "Constitution Day"),
            (2026, 6, 30, "Additional national holiday"),
            (2026, 8, 15, "Founding of Asunción"),
            (2026, 9, 28, "Battle of Boquerón Day"),
            (2026, 12, 8, "Virgin of Caacupé Day"),
            (2026, 12, 25, "Christmas Day"),
            (2025, 9, 5, "Additional national holiday"),
            (2025, 12, 26, "Additional national holiday"),
            (2025, 5, 15, "Independence Day"),
            (1995, 9, 29, "Battle of Boquerón Day"),
        ],
    );
    // A year whose moving decrees were not read is a gap: under Ley 1723
    // from 2001 to 2025, and for the four movable days from 2027.
    for year in [2010, 2025, 2027] {
        assert!(
            !HolidayCalendar::for_year(table("PY"), None, year).is_complete(),
            "{year}"
        );
    }
    assert!(HolidayCalendar::for_year(table("PY"), None, 1999).is_complete());
    assert!(HolidayCalendar::for_year(table("PY"), None, 2026).is_complete());
    // The days the 2026 decrees moved off, 20 June before the 2025 law,
    // and 29 September before Ley 715 of 1995.
    expect_working(
        "PY",
        None,
        &[
            (2026, 3, 1),
            (2026, 6, 20),
            (2026, 9, 29),
            (2025, 6, 20),
            (1994, 9, 29),
        ],
    );
}

#[test]
fn honduras_keeps_the_october_days_as_the_semana_morazanica_from_2015() {
    expect(
        "HN",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 4, 2, "Maundy Thursday"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 4, "Holy Saturday"),
            (2026, 4, 14, "Pan American Day"),
            (2026, 5, 1, "Labour Day"),
            (2026, 9, 15, "Independence Day"),
            (2026, 10, 7, "Morazanic Week"),
            (2026, 10, 8, "Morazanic Week"),
            (2026, 10, 9, "Morazanic Week"),
            (2026, 12, 25, "Christmas Day"),
            (2025, 10, 1, "Morazanic Week"),
            (2025, 10, 2, "Morazanic Week"),
            (2025, 10, 3, "Morazanic Week"),
            // Decreto 126-2020: November, for 2020 alone.
            (2020, 11, 4, "Morazanic Week"),
            (2020, 11, 5, "Morazanic Week"),
            (2020, 11, 6, "Morazanic Week"),
            (2013, 10, 3, "Soldier's Day"),
            (2013, 10, 12, "Discovery of America Day"),
            (2013, 10, 21, "Armed Forces Day"),
        ],
    );
    expect_working(
        "HN",
        None,
        &[
            (2026, 10, 12),
            (2026, 10, 21),
            (2020, 10, 7),
            (2019, 10, 21),
        ],
    );
    let calendar = HolidayCalendar::for_year(table("HN"), None, 2026);
    let kinds: Vec<Kind> = calendar
        .on(ymd(2026, 10, 7))
        .iter()
        .map(|holiday| holiday.kind)
        .collect();
    assert_eq!(kinds, [Kind::Bank]);
    // Decreto 75-2014 was not read, so 2014 is a gap.
    assert!(!HolidayCalendar::for_year(table("HN"), None, 2014).is_complete());
    assert!(HolidayCalendar::for_year(table("HN"), None, 2013).is_complete());
}

#[test]
fn el_salvador_has_article_190_the_two_parents_days_and_san_salvadors_august() {
    expect(
        "SV",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 4, 2, "Maundy Thursday"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 4, "Holy Saturday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 10, "Mother's Day"),
            (2026, 6, 17, "Father's Day"),
            (2026, 8, 6, "Feast of the Divine Saviour of the World"),
            (2026, 9, 15, "Independence Day"),
            (2026, 11, 2, "All Souls' Day"),
            (2026, 12, 25, "Christmas Day"),
            (2025, 4, 19, "Holy Saturday"),
            (2023, 7, 7, "Central American and Caribbean Games holiday"),
            (2016, 5, 10, "Mother's Day"),
            (2013, 6, 17, "Father's Day"),
        ],
    );
    expect(
        "SV",
        Some("SV-SS"),
        &[
            (2026, 8, 3, "August Festivities"),
            (2026, 8, 5, "August Festivities"),
            (2026, 8, 6, "Feast of the Divine Saviour of the World"),
        ],
    );
    // San Salvador's two days are not national; Father's Day began in
    // 2013; 7 July was a day off in 2023 alone.
    expect_working(
        "SV",
        None,
        &[(2026, 8, 3), (2026, 8, 5), (2011, 6, 17), (2026, 7, 7)],
    );
}

#[test]
fn nicaragua_has_article_66_and_managuas_santo_domingo_and_moves_none() {
    expect(
        "NI",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 4, 2, "Maundy Thursday"),
            (2026, 4, 3, "Good Friday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 7, 19, "Revolution Day"),
            (2026, 9, 14, "Battle of San Jacinto"),
            (2026, 9, 15, "Independence Day"),
            (2026, 12, 8, "Immaculate Conception"),
            (2026, 12, 25, "Christmas Day"),
            (2025, 4, 17, "Maundy Thursday"),
            (2025, 4, 18, "Good Friday"),
            (2025, 9, 15, "Independence Day"),
        ],
    );
    expect(
        "NI",
        Some("NI-MN"),
        &[
            (2026, 8, 1, "Santo Domingo de Guzmán"),
            (2026, 8, 10, "Santo Domingo de Guzmán"),
        ],
    );
    // A Sunday 19 July 2026 gives no Monday; Managua's days are not
    // national.
    expect_working("NI", None, &[(2026, 7, 20), (2026, 8, 10), (2026, 11, 2)]);
}

#[test]
fn oman_compensates_weekend_days_as_the_2022_decree_says() {
    expect(
        "OM",
        None,
        &[
            (2026, 1, 11, "Accession Day"),
            // A Friday Isra and Mi'raj compensated on the Sunday.
            (2026, 1, 16, "Isra and Mi'raj"),
            (2026, 1, 18, "Isra and Mi'raj"),
            (2026, 3, 18, "Eid al-Fitr"),
            (2026, 3, 22, "Eid al-Fitr"),
            (2026, 5, 26, "Eid al-Adha"),
            (2026, 5, 29, "Eid al-Adha"),
            (2026, 6, 17, "Islamic New Year"),
            (2026, 8, 26, "Prophet's Birthday"),
            // 2026's National Day falls on the weekend: one day, the Sunday.
            (2026, 11, 20, "National Day"),
            (2026, 11, 21, "National Day"),
            (2026, 11, 22, "National Day"),
            // 2025: a Saturday Accession Day, a Friday-starting Eid al-Adha
            // with its compensation after the four days, a Friday 21 November.
            (2025, 1, 12, "Accession Day"),
            (2025, 6, 6, "Eid al-Adha"),
            (2025, 6, 10, "Eid Compensation Day"),
            (2025, 11, 23, "National Day"),
            (2021, 1, 11, "Accession Day"),
            (2021, 11, 19, "National Day"),
            (2021, 11, 21, "National Day"),
            (2019, 7, 23, "Renaissance Day"),
            (2019, 11, 18, "National Day"),
        ],
    );
    expect_working(
        "OM",
        None,
        &[
            (2026, 11, 23),
            (2026, 3, 23),
            (2025, 1, 13),
            (2020, 7, 23),
            (2020, 1, 11),
            (2019, 11, 19),
        ],
    );
    expect_substitute("OM", None, 2026, (1, 16), (1, 18));
    expect_substitute("OM", None, 2025, (1, 11), (1, 12));
}

#[test]
fn qatar_keeps_the_two_eid_spans_of_the_2025_decision_and_the_bank_days() {
    expect(
        "QA",
        None,
        &[
            (2026, 2, 10, "National Sport Day"),
            (2026, 3, 17, "Eid al-Fitr"),
            (2026, 3, 23, "Eid al-Fitr"),
            (2026, 5, 26, "Eid al-Adha"),
            (2026, 5, 30, "Eid al-Adha"),
            (2026, 12, 18, "National Day"),
            (2025, 2, 11, "National Sport Day"),
            (2025, 3, 28, "Eid al-Fitr"),
            (2025, 4, 3, "Eid al-Fitr"),
            (2025, 6, 6, "Eid al-Adha"),
            (2025, 6, 10, "Eid al-Adha"),
            (2012, 2, 14, "National Sport Day"),
            (2007, 12, 18, "National Day"),
        ],
    );
    expect_working(
        "QA",
        None,
        &[(2026, 3, 24), (2026, 5, 31), (2011, 2, 8), (2006, 12, 18)],
    );
    let calendar = HolidayCalendar::for_year(table("QA"), None, 2026);
    let banks: Vec<(&str, Kind)> = [ymd(2026, 1, 1), ymd(2026, 3, 1)]
        .iter()
        .flat_map(|day| calendar.on(*day))
        .map(|holiday| (holiday.name, holiday.kind))
        .collect();
    assert_eq!(
        banks,
        [("Bank Holiday", Kind::Bank), ("Bank Day", Kind::Bank)]
    );
}

#[test]
fn iraq_follows_law_12_of_2024_with_its_community_days_religious() {
    expect(
        "IQ",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 6, "Army Day"),
            (2026, 3, 16, "Remembrance of the Ba'ath Crimes"),
            (2026, 3, 20, "Eid al-Fitr"),
            (2026, 3, 21, "Nowruz"),
            (2026, 3, 22, "Eid al-Fitr"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 27, "Eid al-Adha"),
            (2026, 5, 30, "Eid al-Adha"),
            (2026, 6, 4, "Eid al-Ghadir"),
            (2026, 6, 17, "Islamic New Year"),
            (2026, 6, 26, "Ashura"),
            (2026, 8, 26, "Prophet's Birthday"),
            (2023, 12, 25, "Christmas Day"),
            (2020, 12, 25, "Christmas Day"),
        ],
    );
    // Christmas is the Christians' day from 2024, and Ghadir and 16 March
    // did not exist before the law.
    expect_working(
        "IQ",
        None,
        &[(2026, 12, 25), (2023, 3, 16), (2023, 7, 16), (2019, 12, 25)],
    );
    let calendar = HolidayCalendar::for_year(table("IQ"), None, 2025);
    let community: Vec<(&str, Kind)> = [
        ymd(2025, 4, 16),
        ymd(2025, 4, 20),
        ymd(2025, 10, 6),
        ymd(2025, 12, 19),
        ymd(2025, 12, 25),
    ]
    .iter()
    .flat_map(|day| calendar.on(*day))
    .map(|holiday| (holiday.name, holiday.kind))
    .collect();
    assert_eq!(
        community,
        [
            ("Yazidi New Year", Kind::Religious),
            ("Easter Sunday", Kind::Religious),
            ("Feast of the Assembly", Kind::Religious),
            ("Yazidi Feast of the Fast", Kind::Religious),
            ("Christmas Day", Kind::Religious),
        ]
    );
}

#[test]
fn syria_follows_decree_188_of_2025_with_both_easters() {
    expect(
        "SY",
        None,
        &[
            (2025, 4, 20, "Eastern Easter"),
            (2025, 4, 20, "Western Easter"),
            (2025, 12, 8, "Liberation Day"),
            (2025, 12, 25, "Christmas Day"),
            (2026, 1, 1, "New Year's Day"),
            (2026, 3, 18, "Syrian Revolution Day"),
            (2026, 3, 20, "Eid al-Fitr"),
            (2026, 3, 21, "Mother's Day"),
            (2026, 3, 21, "Nowruz"),
            (2026, 4, 5, "Western Easter"),
            (2026, 4, 12, "Eastern Easter"),
            (2026, 4, 17, "Evacuation Day"),
            (2026, 5, 27, "Eid al-Adha"),
            (2026, 5, 30, "Eid al-Adha"),
        ],
    );
    // The Revolution and Nowruz are new in 2026, and the days the decree
    // dropped — 8 March, the October War, Martyrs' Day — are gone.
    expect_working(
        "SY",
        None,
        &[(2025, 3, 18), (2026, 3, 8), (2026, 10, 6), (2026, 5, 6)],
    );
    let calendar = HolidayCalendar::for_year(table("SY"), None, 2025);
    let nowruz_2025: Vec<&str> = calendar
        .on(ymd(2025, 3, 21))
        .iter()
        .map(|holiday| holiday.name)
        .collect();
    assert_eq!(nowruz_2025, ["Mother's Day"]);
    // Friday and Saturday from 2004, Friday alone before.
    assert!(calendar.is_weekend(ymd(2025, 3, 22)));
    let calendar = HolidayCalendar::for_year(table("SY"), None, 2003);
    assert!(!calendar.is_weekend(ymd(2003, 3, 22)));
}

#[test]
fn palestine_follows_the_council_of_ministers_tables_with_the_eastern_easter_for_all() {
    expect(
        "PS",
        None,
        &[
            (2025, 3, 30, "Eve of Eid al-Fitr"),
            (2025, 4, 20, "Easter Sunday"),
            (2025, 6, 6, "Eve of Eid al-Adha"),
            (2025, 11, 15, "Independence Day"),
            (2026, 1, 7, "Eastern Christmas"),
            (2026, 3, 8, "International Women's Day"),
            (2026, 3, 19, "Eve of Eid al-Fitr"),
            (2026, 3, 22, "Eid al-Fitr"),
            (2026, 4, 12, "Easter Sunday"),
            (2026, 5, 30, "Eid al-Adha"),
            (2026, 12, 25, "Western Christmas"),
        ],
    );
    // The Western Easter and the Christians' second days are not days off
    // for all; a Friday holiday stays on the Friday.
    expect_working(
        "PS",
        None,
        &[(2026, 4, 5), (2026, 4, 13), (2026, 12, 26), (2026, 5, 3)],
    );
    let calendar = HolidayCalendar::for_year(table("PS"), None, 2026);
    for (month, day, name) in [
        (1, 14, "Eastern New Year"),
        (4, 3, "Western Good Friday"),
        (4, 5, "Western Easter Sunday"),
        (4, 10, "Eastern Good Friday"),
        (4, 13, "Eastern Easter Monday"),
        (5, 21, "Eastern Ascension"),
        (12, 26, "Western Christmas"),
    ] {
        let found: Vec<(&str, Kind)> = calendar
            .on(ymd(2026, month, day))
            .iter()
            .filter(|holiday| holiday.name == name)
            .map(|holiday| (holiday.name, holiday.kind))
            .collect();
        assert_eq!(found, [(name, Kind::Religious)], "{month}-{day}");
    }
}

#[test]
fn libya_follows_law_5_of_2012_with_arafah_and_three_days_of_each_eid() {
    expect(
        "LY",
        None,
        &[
            (2025, 2, 17, "Revolution Day"),
            (2025, 6, 6, "Day of Arafah"),
            (2025, 9, 16, "Martyrs' Day"),
            (2025, 10, 23, "Liberation Day"),
            (2026, 3, 22, "Eid al-Fitr"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 26, "Day of Arafah"),
            (2026, 5, 29, "Eid al-Adha"),
            (2026, 8, 26, "Prophet's Birthday"),
            (2026, 12, 24, "Independence Day"),
        ],
    );
    // Three days of Eid al-Adha, not four; the days of 2011 not before the
    // law; and 1 January is not in its table.
    expect_working(
        "LY",
        None,
        &[(2026, 5, 30), (2011, 2, 17), (2011, 10, 23), (2026, 1, 1)],
    );
    let calendar = HolidayCalendar::for_year(table("LY"), None, 2026);
    assert!(calendar.is_weekend(ymd(2026, 5, 30)));
    let calendar = HolidayCalendar::for_year(table("LY"), None, 2005);
    assert!(!calendar.is_weekend(ymd(2005, 1, 8)));
}

#[test]
fn yemen_follows_law_2_of_2000_with_five_day_eids_and_nothing_moved() {
    expect(
        "YE",
        None,
        &[
            (2025, 3, 29, "Eid al-Fitr"),
            (2025, 4, 2, "Eid al-Fitr"),
            (2025, 5, 22, "National Day"),
            (2025, 9, 26, "26 September Revolution Day"),
            (2025, 11, 30, "Independence Day"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 26, "Eid al-Adha"),
            (2026, 5, 30, "Eid al-Adha"),
            (2026, 6, 17, "Islamic New Year"),
            (2026, 10, 14, "14 October Revolution Day"),
        ],
    );
    // Article 4's replacement day is not carried: Labour Day on Friday
    // 1 May 2026 gives no Sunday. The article 3(b) days are observances.
    expect_working(
        "YE",
        None,
        &[(2026, 5, 3), (2026, 5, 31), (2026, 8, 26), (2026, 7, 7)],
    );
    let calendar = HolidayCalendar::for_year(table("YE"), None, 2026);
    let mawlid: Vec<(&str, Kind)> = calendar
        .on(ymd(2026, 8, 26))
        .iter()
        .map(|holiday| (holiday.name, holiday.kind))
        .collect();
    assert_eq!(mawlid, [("Prophet's Birthday", Kind::Observance)]);
    // Thursday and Friday until 2013, Friday and Saturday since.
    assert!(calendar.is_weekend(ymd(2026, 5, 2)));
    let calendar = HolidayCalendar::for_year(table("YE"), None, 2012);
    assert!(calendar.is_weekend(ymd(2012, 5, 3)));
    assert!(!calendar.is_weekend(ymd(2012, 5, 5)));
}

#[test]
fn liechtenstein_has_thirteen_legal_holidays_and_five_bank_days() {
    expect(
        "LI",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 2, "Berchtold's Day"),
            (2026, 1, 6, "Epiphany"),
            (2026, 2, 17, "Shrove Tuesday"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 14, "Ascension"),
            (2026, 5, 25, "Whit Monday"),
            (2026, 6, 4, "Corpus Christi"),
            (2026, 8, 15, "National Day"),
            (2026, 9, 8, "Nativity of Mary"),
            (2026, 11, 1, "All Saints' Day"),
            (2026, 12, 8, "Immaculate Conception"),
            (2026, 12, 24, "Christmas Eve"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 26, "Saint Stephen's Day"),
            (2026, 12, 31, "New Year's Eve"),
        ],
    );
    // Candlemas and Saint Joseph are observances, and a Sunday All Saints
    // gives nothing.
    expect_working("LI", None, &[(2026, 2, 2), (2026, 3, 19), (2026, 11, 2)]);
    let calendar = HolidayCalendar::for_year(table("LI"), None, 2026);
    let kinds: Vec<(&str, Kind)> = [ymd(2026, 1, 2), ymd(2026, 2, 2), ymd(2026, 4, 3)]
        .iter()
        .flat_map(|day| calendar.on(*day))
        .map(|holiday| (holiday.name, holiday.kind))
        .collect();
    assert_eq!(
        kinds,
        [
            ("Berchtold's Day", Kind::Bank),
            ("Candlemas", Kind::Observance),
            ("Good Friday", Kind::Bank),
        ]
    );
}

#[test]
fn monaco_gives_the_monday_after_a_sunday_for_six_of_its_twelve_days() {
    expect(
        "MC",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 27, "Saint Devota's Day"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 14, "Ascension"),
            (2026, 5, 25, "Whit Monday"),
            (2026, 6, 4, "Corpus Christi"),
            (2026, 8, 15, "Assumption"),
            (2026, 11, 1, "All Saints' Day"),
            (2026, 11, 2, "All Saints' Day"),
            (2026, 11, 19, "Sovereign Prince's Day"),
            (2026, 12, 8, "Immaculate Conception"),
            (2026, 12, 25, "Christmas Day"),
            (2027, 8, 16, "Assumption"),
            (1952, 11, 19, "Sovereign Prince's Day"),
        ],
    );
    // A Sunday Immaculate Conception (2024) or Saint Devota's Day (2030)
    // is not in the article's list of six.
    expect_working("MC", None, &[(2024, 12, 9), (2030, 1, 28), (1951, 11, 19)]);
    expect_substitute("MC", None, 2026, (11, 1), (11, 2));
    expect_substitute("MC", None, 2027, (8, 15), (8, 16));
}

#[test]
fn san_marino_keeps_the_captains_regent_days_and_closes_the_banks_on_the_eves() {
    expect(
        "SM",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 6, "Epiphany"),
            (2026, 2, 5, "Feast of Saint Agatha"),
            (2026, 3, 25, "Anniversary of the Arengo"),
            (2026, 4, 1, "Investiture of the Captains Regent"),
            (2026, 4, 5, "Easter Sunday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 6, 4, "Corpus Christi"),
            (2026, 7, 28, "Anniversary of the Fall of Fascism"),
            (2026, 8, 15, "Assumption"),
            (2026, 9, 3, "Feast of Saint Marinus and the Republic"),
            (2026, 10, 1, "Investiture of the Captains Regent"),
            (2026, 11, 1, "All Saints' Day"),
            (2026, 11, 2, "All Souls' Day"),
            (2026, 12, 8, "Immaculate Conception"),
            (2026, 12, 24, "Christmas Eve"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 26, "Saint Stephen's Day"),
            (2026, 12, 31, "New Year's Eve"),
        ],
    );
    expect_working(
        "SM",
        None,
        &[(2026, 3, 19), (2026, 6, 29), (2026, 8, 14), (2026, 8, 16)],
    );
    let calendar = HolidayCalendar::for_year(table("SM"), None, 2026);
    let kinds: Vec<Kind> = [ymd(2026, 12, 24), ymd(2026, 12, 31)]
        .iter()
        .flat_map(|day| calendar.on(*day))
        .map(|holiday| holiday.kind)
        .collect();
    assert_eq!(kinds, [Kind::Bank, Kind::Bank]);
}

#[test]
fn vatican_city_keeps_the_holy_days_of_obligation_and_the_popes_days() {
    // Easter 2026 on 5 April; Leo XIV elected on 8 May 2025.
    expect(
        "VA",
        None,
        &[
            (2026, 1, 1, "Solemnity of Mary, Mother of God"),
            (2026, 1, 6, "Epiphany"),
            (
                2026,
                2,
                11,
                "Anniversary of the Establishment of Vatican City State",
            ),
            (2026, 3, 19, "Saint Joseph"),
            (2026, 4, 2, "Maundy Thursday"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 4, "Holy Saturday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 4, 7, "Easter Tuesday"),
            (2026, 5, 1, "Saint Joseph the Worker"),
            (2026, 5, 8, "Anniversary of the Pope's Election"),
            (2026, 5, 14, "Ascension"),
            (2026, 6, 4, "Corpus Christi"),
            (2026, 6, 29, "Saints Peter and Paul"),
            (2026, 8, 14, "Eve of the Assumption"),
            (2026, 8, 15, "Assumption"),
            (2026, 8, 16, "Day after the Assumption"),
            (2026, 9, 17, "Pope's Name Day"),
            (2026, 11, 1, "All Saints' Day"),
            (2026, 11, 2, "All Souls' Day"),
            (2026, 12, 8, "Immaculate Conception"),
            (2026, 12, 24, "Christmas Eve"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 26, "Saint Stephen's Day"),
            (2026, 12, 27, "Saint John the Apostle"),
            (2026, 12, 31, "Last Day of the Year"),
            // The excavations office's list for 2023.
            (2023, 3, 13, "Anniversary of the Pope's Election"),
            (2023, 4, 23, "Pope's Name Day"),
            (2023, 5, 18, "Ascension"),
            (2023, 6, 8, "Corpus Christi"),
            (2025, 9, 17, "Pope's Name Day"),
        ],
    );
    // Francis's days are not Leo XIV's; the See was vacant on 23 April
    // 2025; the Annunciation and the Baptist's birth are solemnities but
    // not holy days of obligation.
    expect_working(
        "VA",
        None,
        &[
            (2026, 3, 13),
            (2026, 4, 23),
            (2025, 4, 23),
            (2026, 3, 25),
            (2026, 6, 24),
        ],
    );
    // Saturday is a working day, and nothing moves off a Sunday.
    assert_eq!(
        table("VA").weekend_in(2026),
        &[hc_calendar::Weekday::Sunday]
    );
    assert!(table("VA").substitution.is_empty());
    // Canon 1246's fixed days are solemnities of the General Roman
    // Calendar on the same dates.
    for (month, day) in [
        (1, 1),
        (1, 6),
        (3, 19),
        (6, 29),
        (8, 15),
        (11, 1),
        (12, 8),
        (12, 25),
    ] {
        let date = ymd(2026, month, day);
        assert!(
            hc_holiday::roman_calendar::celebrations_on(date)
                .iter()
                .any(|celebration| celebration.rank == hc_holiday::roman_calendar::Rank::Solemnity),
            "{month}-{day}"
        );
        assert!(
            HolidayCalendar::for_year(table("VA"), None, 2026).is_holiday(date),
            "{month}-{day}"
        );
    }
    // Who reigns after 2026 is not known, and the calendar says so.
    let gaps: Vec<&str> = HolidayCalendar::for_year(table("VA"), None, 2027)
        .gaps()
        .iter()
        .map(|gap| gap.name)
        .collect();
    assert_eq!(
        gaps,
        ["Anniversary of the Pope's Election", "Pope's Name Day"]
    );
}

#[test]
fn andorra_has_the_fourteen_national_days_with_carnival_on_its_monday() {
    expect(
        "AD",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 6, "Epiphany"),
            (2026, 2, 16, "Carnival"),
            (2026, 3, 14, "Constitution Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 25, "Whit Monday"),
            (2026, 8, 15, "Assumption"),
            (2026, 9, 8, "Our Lady of Meritxell"),
            (2026, 11, 1, "All Saints' Day"),
            (2026, 12, 8, "Immaculate Conception"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 26, "Saint Stephen's Day"),
            (2025, 3, 3, "Carnival"),
            (2024, 2, 12, "Carnival"),
        ],
    );
    // Shrove Tuesday is not the day, and a Sunday gives nothing.
    expect_working(
        "AD",
        None,
        &[(2026, 2, 17), (2025, 3, 4), (2026, 11, 2), (2024, 9, 9)],
    );
}

#[test]
fn botswana_moves_a_sunday_to_monday_and_the_second_days_past_a_monday() {
    // 2023: New Year's Day on a Sunday takes the Monday and 2 January,
    // now on a Monday, the Tuesday; Botswana Day on a Saturday takes the
    // Monday, which is where a Sunday 1 October would have gone, and the
    // Act gives no Tuesday. 2022: Christmas on a Sunday takes the Monday
    // and Boxing Day the Tuesday. 2029: Botswana Day on a Sunday takes
    // Monday 1 October, and the holiday of 1 October the Tuesday.
    expect(
        "BW",
        None,
        &[
            (2023, 1, 1, "New Year's Day"),
            (2023, 1, 3, "New Year Holiday"),
            (2023, 4, 8, "Holy Saturday"),
            (2023, 7, 17, "President's Day"),
            (2023, 7, 18, "President's Day Holiday"),
            (2022, 12, 27, "Boxing Day"),
            (2029, 10, 2, "Botswana Day Holiday"),
            (2026, 5, 14, "Ascension Day"),
            (2026, 7, 1, "Sir Seretse Khama Day"),
        ],
    );
    expect_substitute("BW", None, 2023, (1, 1), (1, 2));
    expect_substitute("BW", None, 2023, (9, 30), (10, 2));
    expect_substitute("BW", None, 2022, (12, 25), (12, 26));
    expect_substitute("BW", None, 2029, (9, 30), (10, 1));
    // No Tuesday in 2023; a Saturday Sir Seretse Khama Day or Boxing Day
    // gives nothing.
    expect_working("BW", None, &[(2023, 10, 3), (2023, 7, 3), (2026, 12, 28)]);
}

#[test]
fn namibia_adds_a_monday_to_a_sunday_unless_the_monday_is_taken() {
    // 2025: Cassinga Day and Africa Day on Sundays, their Mondays added;
    // Genocide Remembrance Day's first year. 2022: Christmas on a Sunday,
    // the Monday already Family Day, no Tuesday. 10 December carried its
    // old name until the 2004 amendment came into force that 17 December.
    expect(
        "NA",
        None,
        &[
            (2025, 5, 5, "Cassinga Day"),
            (2025, 5, 26, "Africa Day"),
            (2025, 5, 28, "Genocide Remembrance Day"),
            (2025, 5, 29, "Ascension Day"),
            (2022, 12, 26, "Family Day"),
            (2004, 12, 10, "International Human Rights Day"),
            (
                2026,
                12,
                10,
                "Day of the Namibian Women and International Human Rights Day",
            ),
        ],
    );
    expect_substitute("NA", None, 2025, (5, 4), (5, 5));
    expect_substitute("NA", None, 2025, (5, 25), (5, 26));
    expect_working("NA", None, &[(2022, 12, 27), (2026, 3, 23), (2024, 5, 28)]);
}

#[test]
fn mauritius_alternates_the_assumption_and_all_saints_from_2016() {
    // The Prime Minister's Office's dates for 2025 and 2026 where the
    // crate's rules land on them; every Cavadee kept from 2020 to 2026,
    // 2023's a day before Malaysia's; the two days kept from 2001; and All
    // Saints every year to 2015, then in odd years, the Assumption in even
    // ones.
    expect(
        "MU",
        None,
        &[
            (2026, 1, 2, "New Year Holiday"),
            (2026, 2, 15, "Maha Shivaratree"),
            (2026, 2, 17, "Chinese Spring Festival"),
            (2026, 3, 19, "Ougadi"),
            (2026, 8, 15, "Assumption of the Blessed Virgin Mary"),
            (2026, 11, 2, "Arrival of Indentured Labourers"),
            (2026, 11, 8, "Divali"),
            (2025, 2, 26, "Maha Shivaratree"),
            (2025, 3, 30, "Ougadi"),
            (2025, 3, 31, "Eid-Ul-Fitr"),
            (2025, 8, 27, "Ganesh Chaturthi"),
            (2025, 10, 20, "Divali"),
            (2025, 11, 1, "All Saints' Day"),
            (2026, 2, 1, "Thaipoosam Cavadee"),
            (2025, 2, 11, "Thaipoosam Cavadee"),
            (2024, 1, 25, "Thaipoosam Cavadee"),
            (2023, 2, 4, "Thaipoosam Cavadee"),
            (2022, 1, 18, "Thaipoosam Cavadee"),
            (2021, 1, 28, "Thaipoosam Cavadee"),
            (2020, 2, 8, "Thaipoosam Cavadee"),
            (2001, 2, 1, "Abolition of Slavery"),
            (2001, 11, 2, "Arrival of Indentured Labourers"),
            (2014, 11, 1, "All Saints' Day"),
        ],
    );
    expect_working(
        "MU",
        None,
        &[
            (2029, 8, 15),
            (2028, 11, 1),
            (2014, 8, 15),
            (2000, 2, 1),
            (2000, 11, 2),
        ],
    );
}

#[test]
fn malawi_moves_a_saturday_or_sunday_to_the_next_free_day() {
    // 2022: New Year's Day, John Chilembwe Day, Kamuzu Day and Mothers'
    // Day on Saturdays and Labour Day on a Sunday, each to its Monday;
    // Christmas on a Sunday past Boxing Day to the Tuesday. The Saturday
    // after Good Friday is the one Schedule day that never moves.
    expect(
        "MW",
        None,
        &[
            (2022, 1, 3, "New Year's Day"),
            (2022, 1, 17, "John Chilembwe Day"),
            (2022, 5, 16, "Kamuzu Day"),
            (2022, 10, 17, "Mothers' Day"),
            (2022, 12, 26, "Boxing Day"),
            (2022, 12, 27, "Christmas Day"),
            (2026, 3, 3, "Martyrs' Day"),
            (2026, 4, 4, "Holy Saturday"),
            (2026, 7, 6, "Independence Day"),
        ],
    );
    expect_substitute("MW", None, 2022, (1, 15), (1, 17));
    expect_substitute("MW", None, 2022, (5, 1), (5, 2));
    expect_substitute("MW", None, 2022, (12, 25), (12, 27));
    expect_working("MW", None, &[(2026, 4, 7), (2022, 12, 28)]);
}

#[test]
fn rwanda_compensates_a_weekend_holiday_once_and_never_7_april() {
    expect(
        "RW",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 2, "Day after New Year's Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 4, 7, "Genocide against the Tutsi Memorial Day"),
            (2026, 5, 1, "Labour Day"),
            (2026, 7, 1, "Independence Day"),
            (2026, 8, 7, "Umuganura Day"),
            (2026, 12, 25, "Christmas Day"),
            (2025, 8, 1, "Umuganura Day"),
            (2025, 8, 15, "Assumption Day"),
            (2022, 12, 27, "Additional public holiday"),
            (2023, 1, 3, "Additional public holiday"),
        ],
    );
    // National Heroes' Day 2020 on a Saturday: the Ministry's Monday.
    expect_substitute("RW", None, 2020, (2, 1), (2, 3));
    expect_substitute("RW", None, 2026, (7, 4), (7, 6));
    expect_substitute("RW", None, 2026, (12, 26), (12, 28));
    // A Saturday Christmas and a Sunday Boxing Day share one Monday, and a
    // Sunday 7 April is not compensated.
    expect_substitute("RW", None, 2021, (12, 25), (12, 27));
    expect_working("RW", None, &[(2021, 12, 28), (2024, 4, 8)]);
}

#[test]
fn burundi_keeps_decree_100_150_and_moves_nothing_off_a_sunday() {
    expect(
        "BI",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 2, 5, "Unity Day"),
            (
                2026,
                4,
                6,
                "Commemoration of the Assassination of President Cyprien Ntaryamira",
            ),
            (2026, 5, 14, "Ascension Day"),
            (2026, 6, 8, "National Patriotism Day"),
            (2026, 7, 1, "Independence Day"),
            (2026, 10, 13, "Rwagasore Day"),
            (2026, 10, 21, "Ndadaye Day"),
            (2025, 11, 1, "All Saints' Day"),
            (2025, 12, 25, "Christmas Day"),
        ],
    );
    // All Saints' Day 2026 is a Sunday and stays there; 8 June is kept
    // from 2021.
    expect_working("BI", None, &[(2026, 11, 2), (2020, 6, 8)]);
}

#[test]
fn madagascar_keeps_the_yearly_decrees_and_reports_their_undated_days_as_gaps() {
    expect(
        "MG",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 3, 29, "Martyrs' Day"),
            (2026, 4, 5, "Easter Sunday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 14, "Ascension Day"),
            (2026, 5, 25, "Whit Monday"),
            (2026, 6, 26, "Independence Day"),
            (2026, 11, 1, "All Saints' Day"),
            (2025, 4, 21, "Easter Monday"),
            (2025, 4, 23, "Additional public holiday"),
            (2025, 6, 9, "Whit Monday"),
            (2023, 5, 29, "Whit Monday"),
        ],
    );
    // 8 March is a day off for women only; a Sunday New Year's Day stays.
    expect_working("MG", None, &[(2024, 3, 8), (2023, 1, 2), (2026, 3, 30)]);
    assert!(HolidayCalendar::for_year(table("MG"), None, 2025).is_complete());
    let missing: Vec<&str> = HolidayCalendar::for_year(table("MG"), None, 2026)
        .gaps()
        .iter()
        .map(|gap| gap.name)
        .collect();
    assert!(missing.contains(&"Malagasy New Year"), "{missing:?}");
    assert!(missing.contains(&"National Culture Day"), "{missing:?}");
    assert!(!HolidayCalendar::for_year(table("MG"), None, 2024).is_complete());
}

#[test]
fn seychelles_moves_a_sunday_holiday_to_the_next_free_day() {
    expect(
        "SC",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 2, "New Year Holiday"),
            (2026, 4, 4, "Easter Saturday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 6, 4, "Corpus Christi"),
            (2026, 6, 18, "Constitution Day"),
            (2026, 6, 29, "Independence (National) Day"),
            (2026, 12, 8, "Immaculate Conception"),
            (2016, 6, 5, "Liberation Day"),
            (2014, 6, 18, "National Day"),
        ],
    );
    expect_substitute("SC", None, 2026, (11, 1), (11, 2));
    expect_substitute("SC", None, 2016, (6, 5), (6, 6));
    // A Sunday New Year's Day passes the 2nd, which is a holiday already.
    expect_substitute("SC", None, 2023, (1, 1), (1, 3));
    // Liberation Day went and Easter Monday came in 2017; a Saturday
    // Assumption stays.
    expect_working("SC", None, &[(2017, 6, 5), (2016, 3, 28), (2026, 8, 17)]);
}

#[test]
fn mozambique_keeps_the_nine_days_of_article_105_where_they_fall() {
    expect(
        "MZ",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 2, 3, "Heroes' Day"),
            (2026, 4, 7, "Women's Day"),
            (2026, 5, 1, "Workers' Day"),
            (2026, 6, 25, "Independence Day"),
            (2026, 9, 7, "Lusaka Accord Day"),
            (2026, 9, 25, "Armed Forces Day"),
            (2026, 10, 4, "Peace and Reconciliation Day"),
            (2025, 12, 25, "Family Day"),
            (2025, 4, 7, "Women's Day"),
        ],
    );
    // A Sunday 4 October stays; Good Friday is no holiday.
    expect_working("MZ", None, &[(2026, 10, 5), (2026, 4, 3)]);
}

#[test]
fn lesotho_keeps_the_acts_days_and_reports_the_changed_ones_as_gaps_before_2026() {
    expect(
        "LS",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 3, 11, "Moshoeshoe's Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 14, "Ascension Day"),
            (2026, 5, 25, "Africa's Heroes' Day"),
            (2026, 7, 17, "King Letsie III's Birthday"),
            (2026, 12, 26, "Boxing Day"),
            (2025, 5, 1, "Workers' Day"),
            (2025, 10, 4, "National Independence Day"),
        ],
    );
    assert!(HolidayCalendar::for_year(table("LS"), None, 2026).is_complete());
    let missing: Vec<&str> = HolidayCalendar::for_year(table("LS"), None, 2025)
        .gaps()
        .iter()
        .map(|gap| gap.name)
        .collect();
    for name in ["Heroes' Day", "King's Birthday", "Boxing Day"] {
        assert!(
            missing.contains(&name),
            "{name} should be a gap: {missing:?}"
        );
    }
    // Independence Day 2026 is a Sunday and stays there.
    expect_working("LS", None, &[(2026, 10, 5), (2026, 4, 4)]);
}

#[test]
fn chad_moves_only_article_2s_paid_days_off_a_sunday() {
    expect(
        "TD",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 3, 8, "International Women's Day"),
            (2026, 3, 20, "Eid al-Fitr"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 27, "Eid al-Adha"),
            (2026, 8, 11, "Independence Day"),
            (2026, 8, 26, "Prophet's Birthday"),
            (2026, 11, 1, "All Saints' Day"),
            (2026, 11, 28, "Republic Day"),
            (2026, 12, 1, "Freedom and Democracy Day"),
            (2026, 12, 25, "Christmas Day"),
            (2015, 8, 11, "Independence Day"),
        ],
    );
    // 8 March 2020, 1 December 2024 and 11 August 2024 were Sundays.
    expect_substitute("TD", None, 2020, (3, 8), (3, 9));
    expect_substitute("TD", None, 2024, (12, 1), (12, 2));
    expect_substitute("TD", None, 2024, (8, 11), (8, 12));
    // 8 March before the 2019 decree; a Sunday Republic Day or Christmas,
    // article 1's days, stays.
    expect_working("TD", None, &[(2018, 3, 8), (2021, 11, 29), (2022, 12, 26)]);
    let missing: Vec<&str> = HolidayCalendar::for_year(table("TD"), None, 2010)
        .gaps()
        .iter()
        .map(|gap| gap.name)
        .collect();
    assert!(missing.contains(&"Independence Day"), "{missing:?}");
    assert!(HolidayCalendar::for_year(table("TD"), None, 2011).is_complete());
}

#[test]
fn mauritania_keeps_law_92_018_and_its_weekend_moved_in_2014() {
    expect(
        "MR",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 3, 20, "Eid al-Fitr"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 25, "Africa Liberation Day"),
            (2026, 5, 27, "Eid al-Adha"),
            (2026, 6, 17, "Islamic New Year"),
            (2026, 8, 26, "Prophet's Birthday"),
            (2026, 11, 28, "National Day"),
            (2025, 11, 28, "National Day"),
            (2025, 5, 25, "Africa Liberation Day"),
        ],
    );
    // A Saturday National Day stays; the second day of an Eid is a decree
    // each time and is not carried.
    expect_working("MR", None, &[(2026, 11, 30), (2026, 5, 28)]);
    let before = HolidayCalendar::for_year(table("MR"), None, 2013);
    assert!(before.is_weekend(ymd(2013, 6, 7)), "a Friday in 2013");
    assert!(!before.is_weekend(ymd(2013, 6, 9)), "a Sunday in 2013");
    let after = HolidayCalendar::for_year(table("MR"), None, 2026);
    assert!(!after.is_weekend(ymd(2026, 6, 5)), "a Friday in 2026");
    assert!(after.is_weekend(ymd(2026, 6, 7)), "a Sunday in 2026");
}

#[test]
fn djibouti_keeps_two_days_of_each_eid_and_of_independence_on_a_friday_weekend() {
    expect(
        "DJ",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 16, "Isra and Mi'raj"),
            (2026, 3, 20, "Eid al-Fitr"),
            (2026, 3, 21, "Eid al-Fitr (second day)"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 27, "Eid al-Adha"),
            (2026, 5, 28, "Eid al-Adha (second day)"),
            (2026, 6, 17, "Islamic New Year"),
            (2026, 6, 27, "Independence Day"),
            (2026, 6, 28, "Independence Day (second day)"),
            (2026, 8, 26, "Prophet's Birthday"),
            (2026, 12, 25, "Christmas Day"),
            (1981, 6, 28, "Independence Day (second day)"),
            (1979, 6, 27, "Independence Day"),
        ],
    );
    // One day of Independence before the rectifying arrêté.
    expect_working("DJ", None, &[(1979, 6, 28)]);
    assert!(!HolidayCalendar::for_year(table("DJ"), None, 1980).is_complete());
    let calendar = HolidayCalendar::for_year(table("DJ"), None, 2026);
    assert!(calendar.is_weekend(ymd(2026, 9, 25)), "a Friday");
    assert!(!calendar.is_weekend(ymd(2026, 9, 26)), "a Saturday");
    assert!(!calendar.is_weekend(ymd(2026, 9, 27)), "a Sunday");
}

#[test]
fn hungary_holidays_stay_on_the_weekend_and_good_friday_began_in_2017() {
    expect(
        "HU",
        None,
        &[
            (2024, 1, 1, "New Year's Day"),
            (2024, 3, 15, "1848 Revolution Memorial Day"),
            (2024, 3, 29, "Good Friday"),
            (2024, 4, 1, "Easter Monday"),
            (2024, 5, 20, "Whit Monday"),
            (2024, 8, 20, "Saint Stephen's Day"),
            (2024, 10, 23, "1956 Revolution Memorial Day"),
            (2024, 12, 26, "Second Day of Christmas"),
            (2025, 3, 15, "1848 Revolution Memorial Day"),
        ],
    );
    // 15 March 2025 was a Saturday and stayed one; 25 March 2016, a Good
    // Friday, was a working day.
    expect_working("HU", None, &[(2025, 3, 17), (2016, 3, 25)]);
}

#[test]
fn romania_keeps_orthodox_easter_and_its_recent_additions_by_year() {
    // Orthodox Easter 2024 was 5 May, Pentecost 23 June.
    expect(
        "RO",
        None,
        &[
            (2024, 1, 2, "Day after New Year's Day"),
            (2024, 1, 6, "Epiphany"),
            (2024, 1, 7, "Saint John the Baptist"),
            (2024, 1, 24, "Union of the Romanian Principalities"),
            (2024, 5, 3, "Good Friday"),
            (2024, 5, 6, "Easter Monday"),
            (2024, 6, 1, "Children's Day"),
            (2024, 6, 24, "Whit Monday"),
            (2024, 8, 15, "Dormition of the Mother of God"),
            (2024, 11, 30, "Saint Andrew's Day"),
            (2024, 12, 1, "National Day"),
        ],
    );
    // Before their laws: Epiphany 2023, Children's Day 2016, Good Friday 2017.
    expect_working("RO", None, &[(2023, 1, 6), (2016, 6, 1), (2017, 4, 14)]);
}

#[test]
fn russia_grew_its_new_year_holidays_and_carries_no_transfers() {
    expect(
        "RU",
        None,
        &[
            (2024, 1, 1, "New Year Holidays"),
            (2024, 1, 6, "New Year Holidays"),
            (2024, 1, 7, "Orthodox Christmas"),
            (2024, 1, 8, "New Year Holidays"),
            (2024, 2, 23, "Defender of the Fatherland Day"),
            (2024, 3, 8, "International Women's Day"),
            (2024, 5, 9, "Victory Day"),
            (2024, 6, 12, "Russia Day"),
            (2024, 11, 4, "Unity Day"),
            (2004, 11, 7, "Day of Accord and Reconciliation"),
            (2004, 5, 2, "Spring and Labour Day"),
        ],
    );
    // 6 January was a working day until 2013, 7 November after 2004, and a
    // Saturday 8 March (2025) is not moved by this table.
    expect_working("RU", None, &[(2012, 1, 6), (2005, 11, 7), (2025, 3, 10)]);
}

/// ConsultantPlus's production calendars for Russia, 2013 to 2027: the
/// weekdays that are not working days, and the weekend days that are.
/// A year of a production calendar: the year, its weekdays off, and its
/// weekend days worked, as (month, day).
type ProductionYear = (i64, &'static [(u8, u8)], &'static [(u8, u8)]);

#[rustfmt::skip]
const RU_PRODUCTION_CALENDARS: &[ProductionYear] = &[
    (2013, &[(1, 1), (1, 2), (1, 3), (1, 4), (1, 7), (1, 8), (3, 8), (5, 1), (5, 2), (5, 3), (5, 9), (5, 10), (6, 12), (11, 4)], &[]),
    (2014, &[(1, 1), (1, 2), (1, 3), (1, 6), (1, 7), (1, 8), (3, 10), (5, 1), (5, 2), (5, 9), (6, 12), (6, 13), (11, 3), (11, 4)], &[]),
    (2015, &[(1, 1), (1, 2), (1, 5), (1, 6), (1, 7), (1, 8), (1, 9), (2, 23), (3, 9), (5, 1), (5, 4), (5, 11), (6, 12), (11, 4)], &[]),
    (2016, &[(1, 1), (1, 4), (1, 5), (1, 6), (1, 7), (1, 8), (2, 22), (2, 23), (3, 7), (3, 8), (5, 2), (5, 3), (5, 9), (6, 13), (11, 4)], &[(2, 20)]),
    (2017, &[(1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (2, 23), (2, 24), (3, 8), (5, 1), (5, 8), (5, 9), (6, 12), (11, 6)], &[]),
    (2018, &[(1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (1, 8), (2, 23), (3, 8), (3, 9), (4, 30), (5, 1), (5, 2), (5, 9), (6, 11), (6, 12), (11, 5), (12, 31)], &[(4, 28), (6, 9), (12, 29)]),
    (2019, &[(1, 1), (1, 2), (1, 3), (1, 4), (1, 7), (1, 8), (3, 8), (5, 1), (5, 2), (5, 3), (5, 9), (5, 10), (6, 12), (11, 4)], &[]),
    (2020, &[(1, 1), (1, 2), (1, 3), (1, 6), (1, 7), (1, 8), (2, 24), (3, 9), (5, 1), (5, 4), (5, 5), (5, 11), (6, 12), (11, 4)], &[]),
    (2021, &[(1, 1), (1, 4), (1, 5), (1, 6), (1, 7), (1, 8), (2, 22), (2, 23), (3, 8), (5, 3), (5, 10), (6, 14), (11, 4), (11, 5), (12, 31)], &[(2, 20)]),
    (2022, &[(1, 3), (1, 4), (1, 5), (1, 6), (1, 7), (2, 23), (3, 7), (3, 8), (5, 2), (5, 3), (5, 9), (5, 10), (6, 13), (11, 4)], &[(3, 5)]),
    (2023, &[(1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (2, 23), (2, 24), (3, 8), (5, 1), (5, 8), (5, 9), (6, 12), (11, 6)], &[]),
    (2024, &[(1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (1, 8), (2, 23), (3, 8), (4, 29), (4, 30), (5, 1), (5, 9), (5, 10), (6, 12), (11, 4), (12, 30), (12, 31)], &[(4, 27), (11, 2), (12, 28)]),
    (2025, &[(1, 1), (1, 2), (1, 3), (1, 6), (1, 7), (1, 8), (5, 1), (5, 2), (5, 8), (5, 9), (6, 12), (6, 13), (11, 3), (11, 4), (12, 31)], &[(11, 1)]),
    (2026, &[(1, 1), (1, 2), (1, 5), (1, 6), (1, 7), (1, 8), (1, 9), (2, 23), (3, 9), (5, 1), (5, 11), (6, 12), (11, 4), (12, 31)], &[]),
    (2027, &[(1, 1), (1, 4), (1, 5), (1, 6), (1, 7), (1, 8), (2, 22), (2, 23), (3, 8), (5, 3), (5, 10), (6, 14), (11, 4), (11, 5), (12, 31)], &[(2, 20)]),
];

#[test]
fn russia_matches_its_production_calendar_from_2013_to_2027() {
    use hc_calendar::Weekday;
    for &(year, days_off, days_worked) in RU_PRODUCTION_CALENDARS {
        let calendar = HolidayCalendar::for_year(table("RU"), None, year);
        assert!(calendar.is_complete(), "{year}: {:?}", calendar.gaps());
        let mut off = Vec::new();
        let mut worked = Vec::new();
        let first = ymd(year, 1, 1).0;
        let last = ymd(year, 12, 31).0;
        for fixed in first..=last {
            let day = Rd(fixed);
            let (_, month, date) = gregorian::from_fixed(day).unwrap();
            let weekend = matches!(Weekday::from_rd(day), Weekday::Saturday | Weekday::Sunday);
            let working = calendar.is_business_day(day);
            if !weekend && !working {
                off.push((month, date));
            }
            if weekend && working {
                worked.push((month, date));
            }
        }
        assert_eq!(off, days_off, "{year}: weekdays off");
        assert_eq!(worked, days_worked, "{year}: weekend days worked");
    }
}

#[test]
fn russia_carries_a_weekend_holiday_over_unless_the_decree_moves_it() {
    // Sunday 8 March 2020 gave Monday the 9th.
    expect(
        "RU",
        None,
        &[(
            2020,
            3,
            9,
            "Day off carried over from a holiday on the weekend",
        )],
    );
    // Sunday 23 February 2025 went to Thursday 8 May instead.
    expect_working("RU", None, &[(2025, 2, 24)]);
    expect(
        "RU",
        None,
        &[(2025, 5, 8, "Day off transferred by the Government")],
    );
    // In 2014 the decree moved the Monday the Sunday had given.
    expect_working("RU", None, &[(2014, 2, 24)]);
    // Saturday 27 April 2024 was worked, for Monday the 29th.
    let calendar = HolidayCalendar::for_year(table("RU"), None, 2024);
    assert!(calendar.is_business_day(ymd(2024, 4, 27)));
    assert!(!calendar.is_business_day(ymd(2024, 4, 29)));
    // A January holiday on the weekend stays a holiday when its weekend is
    // moved: 6 January 2024 was a Saturday, moved to 10 May.
    assert!(calendar.is_holiday(ymd(2024, 1, 6)));
    assert!(!calendar.is_business_day(ymd(2024, 1, 6)));
    // A year no decree carried here covers is a gap.
    for year in [2012, 2028] {
        assert!(!HolidayCalendar::for_year(table("RU"), None, year).is_complete());
    }
}

#[test]
fn ukraine_moves_a_weekend_holiday_forward_and_changed_its_list_in_2023() {
    // 2021: Labour Day on Saturday 1 May took Monday the 3rd, so Orthodox
    // Easter on Sunday the 2nd took Tuesday the 4th; Trinity on Sunday
    // 20 June gave the 21st, Victory Day on Sunday 9 May the 10th, and
    // Christmas on Saturday 25 December the 27th.
    expect(
        "UA",
        None,
        &[
            (2021, 1, 7, "Orthodox Christmas"),
            (2021, 5, 3, "Labour Day"),
            (2021, 5, 4, "Easter"),
            (2021, 5, 10, "Victory Day"),
            (2021, 6, 21, "Trinity"),
            (2021, 6, 28, "Constitution Day"),
            (2021, 8, 24, "Independence Day"),
            (2021, 10, 14, "Defenders of Ukraine Day"),
            (2021, 12, 27, "Christmas"),
            (2024, 5, 8, "Day of Remembrance and Victory over Nazism"),
            (2024, 7, 15, "Statehood Day"),
            (2024, 10, 1, "Defenders of Ukraine Day"),
            (2024, 12, 25, "Christmas"),
        ],
    );
    expect_substitute("UA", None, 2021, (5, 2), (5, 4));
    expect_substitute("UA", None, 2021, (12, 25), (12, 27));
    expect_working("UA", None, &[(2024, 1, 7), (2024, 5, 9), (2024, 10, 14)]);
}

#[test]
fn saudi_arabia_changed_its_weekend_in_2013() {
    let country = table("SA");
    let before = HolidayCalendar::for_year(country, None, 2012);
    // 2012-03-01 was a Thursday.
    assert!(before.is_weekend(ymd(2012, 3, 1)));
    assert!(before.is_weekend(ymd(2012, 3, 2)));
    assert!(!before.is_weekend(ymd(2012, 3, 3)));
    let after = HolidayCalendar::for_year(country, None, 2014);
    // 2014-03-06 was a Thursday.
    assert!(!after.is_weekend(ymd(2014, 3, 6)));
    assert!(after.is_weekend(ymd(2014, 3, 7)));
    assert!(after.is_weekend(ymd(2014, 3, 8)));
}

#[test]
fn saudi_arabia_holidays() {
    expect(
        "SA",
        None,
        &[
            (2024, 2, 22, "Founding Day"),
            (2024, 9, 23, "National Day"),
            (2025, 2, 22, "Founding Day"),
            (2025, 9, 23, "National Day"),
        ],
    );
    // Founding Day was created in 2022 and the National Day in 2005.
    expect_working("SA", None, &[(2021, 2, 22), (2000, 9, 23)]);
    // The Umm al-Qurā Eids are present and flagged as predictions.
    let calendar = HolidayCalendar::for_year(table("SA"), None, 2025);
    let eid = calendar
        .all()
        .iter()
        .find(|holiday| holiday.name == "Eid al-Fitr")
        .expect("an Eid al-Fitr entry");
    assert_eq!(eid.confidence, hc_holiday::rule::Confidence::Approximate);
}

#[test]
fn the_emirates_changed_its_weekend_in_2022() {
    let country = table("AE");
    let before = HolidayCalendar::for_year(country, None, 2021);
    assert!(before.is_weekend(ymd(2021, 3, 5)));
    assert!(before.is_weekend(ymd(2021, 3, 6)));
    assert!(!before.is_weekend(ymd(2021, 3, 7)));
    let after = HolidayCalendar::for_year(country, None, 2023);
    assert!(!after.is_weekend(ymd(2023, 3, 3)));
    assert!(after.is_weekend(ymd(2023, 3, 4)));
    assert!(after.is_weekend(ymd(2023, 3, 5)));
    expect(
        "AE",
        None,
        &[
            (2024, 1, 1, "New Year's Day"),
            (2024, 12, 2, "National Day"),
            (2025, 12, 1, "Commemoration Day"),
            (2025, 12, 3, "National Day"),
        ],
    );
    // Commemoration Day was 30 November from 2015 to 2018.
    expect("AE", None, &[(2017, 11, 30, "Commemoration Day")]);
}

#[test]
fn turkey_national_and_religious_holidays() {
    expect(
        "TR",
        None,
        &[
            (2024, 4, 23, "National Sovereignty and Children's Day"),
            (
                2024,
                5,
                19,
                "Commemoration of Atatürk, Youth and Sports Day",
            ),
            (2024, 8, 30, "Victory Day"),
            (2024, 10, 29, "Republic Day"),
            (2025, 7, 15, "Democracy and National Unity Day"),
        ],
    );
    // Labour Day was restored in 2009 and 15 July added in 2017.
    expect_working("TR", None, &[(2008, 5, 1), (2016, 7, 15)]);
    expect("TR", None, &[(2009, 5, 1, "Labour and Solidarity Day")]);
}

#[test]
fn egypt_holidays_and_its_coptic_easter() {
    expect(
        "EG",
        None,
        &[
            (2024, 1, 7, "Coptic Christmas"),
            (2024, 4, 25, "Sinai Liberation Day"),
            (2024, 5, 6, "Sham El-Nessim"),
            (2024, 7, 23, "Revolution Day"),
            (2025, 4, 21, "Sham El-Nessim"),
            (2025, 10, 6, "Armed Forces Day"),
        ],
    );
    // Egypt keeps a Friday–Saturday weekend.
    let calendar = HolidayCalendar::for_year(table("EG"), None, 2025);
    assert!(calendar.is_weekend(ymd(2025, 3, 7)));
    assert!(calendar.is_business_day(ymd(2025, 3, 9)));
}

#[test]
fn nigeria_and_south_africa_move_a_sunday_holiday_to_the_monday() {
    expect(
        "NG",
        None,
        &[
            (2024, 6, 12, "Democracy Day"),
            (2024, 10, 1, "Independence Day"),
            (2025, 4, 21, "Easter Monday"),
            (2025, 5, 1, "Workers' Day"),
            (2025, 12, 26, "Boxing Day"),
        ],
    );
    // Democracy Day was 29 May until 2018.
    expect("NG", None, &[(2015, 5, 29, "Democracy Day")]);
    expect_working("NG", None, &[(2020, 5, 29)]);
    expect(
        "ZA",
        None,
        &[
            (2024, 3, 21, "Human Rights Day"),
            (2024, 4, 27, "Freedom Day"),
            (2024, 9, 24, "Heritage Day"),
            (2025, 6, 16, "Youth Day"),
            (2025, 12, 16, "Day of Reconciliation"),
        ],
    );
    // Youth Day 2024 and Freedom Day 2025 both fell on Sundays.
    expect_substitute("ZA", None, 2024, (6, 16), (6, 17));
    expect_substitute("ZA", None, 2025, (4, 27), (4, 28));
}

// ─────────────────────────────────────────────────────────────────────────
// Oceania
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn australia_national_and_state_holidays() {
    expect(
        "AU",
        None,
        &[
            (2024, 1, 26, "Australia Day"),
            (2024, 4, 25, "Anzac Day"),
            (2025, 4, 18, "Good Friday"),
            (2025, 4, 21, "Easter Monday"),
            (2025, 12, 26, "Boxing Day"),
        ],
    );
    expect(
        "AU",
        Some("AU-VIC"),
        &[
            (2024, 3, 11, "Labour Day"),
            (2024, 11, 5, "Melbourne Cup Day"),
            (2025, 6, 9, "Sovereign's Birthday"),
        ],
    );
    expect(
        "AU",
        Some("AU-WA"),
        &[(2025, 6, 2, "Western Australia Day")],
    );
    expect("AU", Some("AU-SA"), &[(2025, 12, 26, "Proclamation Day")]);
    expect_working("AU", None, &[(2024, 3, 11), (2024, 11, 5)]);
    // Australia Day 2025 fell on a Sunday and was observed on the Monday.
    expect_substitute("AU", None, 2025, (1, 26), (1, 27));
    // Easter Saturday and Sunday are never moved.
    expect_working("AU", Some("AU-VIC"), &[(2025, 4, 22), (2025, 4, 23)]);
}

#[test]
fn new_zealand_mondayisation_and_matariki() {
    expect(
        "NZ",
        None,
        &[
            (2024, 2, 6, "Waitangi Day"),
            (2024, 6, 28, "Matariki"),
            (2024, 10, 28, "Labour Day"),
            (2025, 6, 20, "Matariki"),
            (2025, 6, 2, "Sovereign's Birthday"),
        ],
    );
    // Waitangi Day 2021 fell on a Saturday; mondayisation began in 2014.
    expect_substitute("NZ", None, 2021, (2, 6), (2, 8));
    expect_working("NZ", None, &[(2010, 2, 8), (2021, 6, 25)]);
    expect(
        "NZ",
        None,
        &[(2022, 6, 24, "Matariki"), (2026, 7, 10, "Matariki")],
    );
}

#[test]
fn micronesia_keeps_a_saturday_on_the_friday_and_adds_presidents_day_from_2021() {
    expect(
        "FM",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 3, 31, "Micronesian Culture and Tradition Day"),
            (2026, 11, 3, "Independence Day"),
            (2026, 11, 11, "Veterans Day"),
            (2026, 11, 23, "Presidents Day"),
            (2026, 12, 25, "Christmas Day"),
            (2020, 11, 11, "FSM Veterans of Foreign Wars Day"),
            (2004, 11, 11, "FSM Veterans of Foreign Wars Day"),
        ],
    );
    // Presidents Day became law on 24 November 2020, the day after; the
    // Culture and Tradition Day and the veterans' day did not exist
    // before 2010 and 2004.
    expect_working("FM", None, &[(2020, 11, 23), (2009, 3, 31), (2003, 11, 11)]);
    // FSM Day 2026 is a Sunday, United Nations Day 2026 a Saturday.
    expect_substitute("FM", None, 2026, (5, 10), (5, 11));
    expect_substitute("FM", None, 2026, (10, 24), (10, 23));
}

#[test]
fn the_marshall_islands_keep_their_friday_holidays_and_move_a_weekend_one_either_way() {
    expect(
        "MH",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 5, 1, "Constitution Day"),
            (2026, 7, 3, "Fisherman's Day"),
            (2026, 9, 4, "Dri-jerbal Day"),
            (2026, 9, 25, "Manit Day"),
            (2026, 11, 17, "President's Day"),
            (2026, 12, 4, "Gospel Day"),
            (2025, 7, 4, "Fisherman's Day"),
            (2025, 9, 26, "Manit Day"),
        ],
    );
    // 1 March 2026 is a Sunday; 1 May 2027 a Saturday.
    expect_substitute("MH", None, 2026, (3, 1), (3, 2));
    expect_substitute("MH", None, 2027, (5, 1), (4, 30));
    // General Election Day is not carried, and 2026 is not an election
    // year in any case.
    expect_working("MH", None, &[(2026, 11, 16), (2026, 12, 31)]);
}

#[test]
fn nauru_moves_a_weekend_independence_day_to_the_monday_and_tuesday() {
    expect(
        "NR",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 31, "Independence Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 4, 7, "Easter Tuesday"),
            (2026, 6, 26, "Eigigu Day"),
            (2026, 6, 29, "Remembrance Day"),
            (2026, 7, 1, "RONPHOS Handover"),
            (2026, 8, 19, "Ibumin Earoeni Day"),
            (2026, 9, 25, "Sir Hammer DeRoburt Day"),
            (2026, 10, 26, "Angam Day"),
            (2026, 12, 25, "Christmas Day"),
            (2024, 3, 8, "International Women's Day"),
            (2024, 7, 1, "RONPHOS Handover"),
            (2023, 9, 25, "Sir Hammer DeRoburt Day"),
        ],
    );
    // 31 January 2026 is a Saturday and 1 February a Sunday: the Monday
    // and the Tuesday, as section 81(2)(b) gives.
    expect_substitute("NR", None, 2026, (1, 31), (2, 2));
    expect_substitute("NR", None, 2026, (2, 1), (2, 3));
    expect_substitute("NR", None, 2026, (3, 8), (3, 9));
    expect_substitute("NR", None, 2026, (5, 17), (5, 18));
    expect_substitute("NR", None, 2026, (7, 25), (7, 27));
    expect_substitute("NR", None, 2026, (12, 26), (12, 28));
    expect_substitute("NR", None, 2024, (10, 26), (10, 28));
    expect_working("NR", None, &[(2026, 12, 29), (2026, 2, 4)]);
}

#[test]
fn nauru_reports_the_years_its_gazettes_were_not_read_as_gaps() {
    assert!(HolidayCalendar::for_year(table("NR"), None, 2026).is_complete());
    // 2024's gazette was read, but the days only 2026's declared are
    // unknown for it.
    assert!(!HolidayCalendar::for_year(table("NR"), None, 2024).is_complete());
    assert!(!HolidayCalendar::for_year(table("NR"), None, 2025).is_complete());
    assert!(!HolidayCalendar::for_year(table("NR"), None, 2027).is_complete());
}

#[test]
fn palau_keeps_the_nine_days_of_its_code_with_the_friday_and_monday_rule() {
    expect(
        "PW",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 3, 15, "Youth Day"),
            (2026, 5, 5, "Senior Citizens Day"),
            (2026, 7, 9, "Constitution Day"),
            (2026, 9, 7, "Labor Day"),
            (2026, 11, 26, "Thanksgiving Day"),
            (2026, 11, 27, "Family Day"),
            (2026, 12, 25, "Christmas Day"),
            // November 2024 begins on a Friday: the fourth Friday comes
            // before the fourth Thursday.
            (2024, 11, 22, "Family Day"),
            (2024, 11, 28, "Thanksgiving Day"),
        ],
    );
    expect_substitute("PW", None, 2026, (3, 15), (3, 16));
    expect_substitute("PW", None, 2026, (10, 24), (10, 23));
    expect_working("PW", None, &[(2026, 4, 3), (2024, 11, 29)]);
}

#[test]
fn papua_new_guinea_carries_the_acts_own_days_and_its_sunday_rule() {
    expect(
        "PG",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 4, "Easter Saturday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 7, 23, "Papua New Guinea Remembrance Day"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 26, "Boxing Day"),
            (2025, 4, 19, "Easter Saturday"),
        ],
    );
    expect_substitute("PG", None, 2023, (1, 1), (1, 2));
    expect_substitute("PG", None, 2028, (7, 23), (7, 24));
    // A Sunday Christmas gives the Tuesday, Boxing Day holding the Monday.
    expect_substitute("PG", None, 2022, (12, 25), (12, 27));
    // A Saturday holiday stays where it falls.
    expect_working("PG", None, &[(2026, 12, 28)]);
}

#[test]
fn solomon_islands_move_a_sunday_holiday_to_the_monday_only() {
    expect(
        "SB",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 4, "Holy Saturday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 25, "Whit Monday"),
            (2026, 7, 7, "Independence Day"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 26, "National Day of Thanksgiving"),
            (2020, 6, 1, "Whit Monday"),
            (2018, 7, 7, "Independence Day"),
        ],
    );
    expect_substitute("SB", None, 2023, (1, 1), (1, 2));
    // Section 2(2): 26 December on a Monday gives the Tuesday.
    expect_substitute("SB", None, 2022, (12, 25), (12, 27));
    expect_working("SB", None, &[(2026, 12, 28), (2026, 6, 12)]);
}

#[test]
fn tonga_moves_three_days_to_a_monday_and_the_royal_birthdays_off_a_sunday() {
    expect(
        "TO",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 4, 25, "Anzac Day"),
            // 4 June 2026 is a Thursday, 4 November a Wednesday, 4
            // December a Friday, as the 2026 list has them.
            (2026, 6, 8, "Emancipation Day"),
            (2026, 7, 4, "Birthday of the Reigning Sovereign"),
            (2026, 9, 17, "Birthday of the Heir to the Crown"),
            (2026, 11, 2, "Constitution Day"),
            (
                2026,
                12,
                7,
                "Anniversary of the Coronation of King George Tupou I",
            ),
            (2026, 12, 26, "Boxing Day"),
            // And the 2024 list: a Tuesday, a Monday and a Wednesday.
            (2024, 6, 3, "Emancipation Day"),
            (2024, 11, 4, "Constitution Day"),
            (
                2024,
                12,
                2,
                "Anniversary of the Coronation of King George Tupou I",
            ),
        ],
    );
    expect_working(
        "TO",
        None,
        &[(2026, 6, 4), (2026, 11, 4), (2026, 12, 4), (2026, 4, 27)],
    );
    expect_substitute("TO", None, 2027, (7, 4), (7, 5));
    expect_substitute("TO", None, 2028, (9, 17), (9, 18));
}

#[test]
fn tuvalu_moves_a_weekend_tuvalu_day_to_the_monday_and_tuesday() {
    expect(
        "TV",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 5, 11, "Gospel Day"),
            (2026, 6, 13, "Sovereign's Birthday"),
            (2026, 8, 3, "National Youth Day"),
            (2026, 10, 1, "Tuvalu Day"),
            (2026, 10, 2, "Tuvalu Day"),
            (2026, 12, 25, "Christmas Day"),
            (2025, 5, 12, "Gospel Day"),
        ],
    );
    expect_substitute("TV", None, 2026, (6, 13), (6, 15));
    expect_substitute("TV", None, 2022, (10, 1), (10, 3));
    expect_substitute("TV", None, 2022, (10, 2), (10, 4));
    expect_substitute("TV", None, 2022, (12, 25), (12, 27));
    // Commonwealth Day, the second Monday of March, left the Schedule.
    expect_working("TV", None, &[(2026, 3, 9)]);
}

#[test]
fn vanuatu_moves_a_sunday_holiday_and_a_monday_family_day() {
    expect(
        "VU",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 2, 21, "Lini Day"),
            (2026, 3, 5, "Custom Chief's Day"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 14, "Ascension Day"),
            (2026, 7, 24, "Children's National Day"),
            (2026, 7, 30, "Independence Day"),
            (2026, 8, 15, "Assumption Day"),
            (2026, 10, 5, "Constitution Day"),
            (2026, 11, 29, "National Unity Day"),
            (2026, 12, 26, "Family Day"),
            (2025, 5, 29, "Ascension Day"),
        ],
    );
    expect_substitute("VU", None, 2026, (11, 29), (11, 30));
    expect_substitute("VU", None, 2022, (12, 25), (12, 27));
    // A Saturday Lini Day stays on the Saturday.
    expect_working("VU", None, &[(2026, 2, 23), (2026, 8, 17)]);
}

#[test]
fn samoa_gives_the_monday_and_tuesday_for_a_sunday_christmas_or_new_year() {
    expect(
        "WS",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 1, 2, "Day after New Year's Day"),
            (2026, 4, 4, "Saturday after Good Friday"),
            (2026, 5, 11, "Mothers' Day"),
            (2026, 6, 1, "Independence Day"),
            (2026, 8, 10, "Fathers' Day"),
            (2026, 10, 12, "White Sunday Holiday"),
            (2026, 12, 26, "Boxing Day"),
            (2025, 5, 12, "Mothers' Day"),
            (2025, 10, 13, "White Sunday Holiday"),
        ],
    );
    expect_substitute("WS", None, 2025, (6, 1), (6, 2));
    expect_substitute("WS", None, 2027, (12, 26), (12, 27));
    expect_substitute("WS", None, 2023, (1, 1), (1, 3));
    expect_substitute("WS", None, 2022, (12, 25), (12, 27));
    // A Saturday holiday stays on the Saturday.
    expect_working("WS", None, &[(2026, 12, 28), (2027, 1, 4)]);
}

#[test]
fn fiji_keeps_the_days_of_the_governments_yearly_lists() {
    expect(
        "FJ",
        None,
        &[
            (2019, 9, 9, "Constitution Day"),
            (2019, 11, 11, "Prophet Mohammed's Birthday"),
            (2020, 4, 11, "Easter Saturday"),
            (2021, 10, 10, "Fiji Day"),
            (2021, 12, 27, "Christmas Day"),
            (2022, 1, 3, "New Year's Day"),
            (2022, 10, 25, "Diwali"),
            (2023, 5, 15, "Girmit Day"),
            (2023, 5, 29, "Ratu Sir Lala Sukuna Day"),
            (2024, 5, 31, "Ratu Sir Lala Sukuna Day"),
            (2024, 4, 1, "Easter Monday"),
            (2025, 9, 8, "Prophet Mohammed's Birthday"),
            (2026, 4, 3, "Good Friday"),
            (2026, 5, 15, "Girmit Day"),
            (2026, 10, 10, "Fiji Day"),
            (2026, 12, 28, "Boxing Day"),
        ],
    );
    // The lists move New Year's Day 2022 and Boxing Day 2026 off the
    // Saturday and leave Fiji Day 2026 on it; Constitution Day is gone from
    // the 2023 list, and Girmit Day 2024 is kept on Monday 13 May, not on
    // the 14th.
    expect_working(
        "FJ",
        None,
        &[
            (2022, 1, 1),
            (2026, 12, 26),
            (2026, 10, 12),
            (2023, 9, 7),
            (2024, 5, 14),
        ],
    );
}

#[test]
fn fiji_reports_the_years_its_lists_do_not_cover_as_gaps() {
    assert!(HolidayCalendar::for_year(table("FJ"), None, 2019).is_complete());
    assert!(HolidayCalendar::for_year(table("FJ"), None, 2026).is_complete());
    let before = HolidayCalendar::for_year(table("FJ"), None, 2018);
    assert!(before.gaps().iter().any(|gap| gap.name == "Fiji Day"));
    let beyond = HolidayCalendar::for_year(table("FJ"), None, 2027);
    assert!(beyond.gaps().iter().any(|gap| gap.name == "Diwali"));
    // Easter is the Schedule's, and known in any year.
    assert_eq!(beyond.name_on(ymd(2027, 3, 26)), Some("Good Friday"));
}

#[test]
fn kiribati_keeps_the_days_of_the_beretitentis_orders() {
    expect(
        "KI",
        None,
        &[
            (2025, 3, 7, "International Women's Day"),
            (2025, 4, 25, "Special Day in honour of Pope Francis"),
            (2025, 5, 2, "International Labour Day"),
            (2025, 6, 23, "National Police Day"),
            (2025, 7, 12, "National Day"),
            (2025, 7, 14, "National Day"),
            (2025, 12, 29, "Kiribati Holiday"),
            (2026, 1, 2, "Kiribati Holiday"),
            (2026, 4, 6, "Easter Monday"),
            (2026, 4, 7, "National Health Day"),
            (2026, 7, 10, "Gospel Day"),
            (2026, 7, 14, "Kiribati Culture and Senior Citizens Day"),
            (2026, 7, 15, "Kiribati Special Day"),
            (2026, 8, 3, "National Youth and Children's Day"),
            (2026, 10, 5, "World Teachers' Day"),
            (2026, 12, 11, "Human Rights Day"),
            (2026, 12, 25, "Christmas Day"),
            (2026, 12, 28, "Boxing Day"),
        ],
    );
    // The orders' own days and no others: Gospel Day 2026 is the Friday
    // before Saturday 11 July, and the National Day week ends on Wednesday
    // 15 July.
    expect_working("KI", None, &[(2026, 7, 16), (2026, 4, 8), (2025, 12, 24)]);
}

#[test]
fn kiribati_reports_the_years_its_orders_were_not_read_as_gaps() {
    assert!(HolidayCalendar::for_year(table("KI"), None, 2025).is_complete());
    assert!(HolidayCalendar::for_year(table("KI"), None, 2026).is_complete());
    let before = HolidayCalendar::for_year(table("KI"), None, 2024);
    assert!(before.gaps().iter().any(|gap| gap.name == "National Day"));
    assert!(
        !before
            .gaps()
            .iter()
            .any(|gap| gap.name == "Special Day in honour of Pope Francis")
    );
    assert!(!HolidayCalendar::for_year(table("KI"), None, 2027).is_complete());
}

#[test]
fn comoros_keeps_decree_25_147s_days_from_2026_and_moves_nothing() {
    expect(
        "KM",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 2, 17, "National Reconciliation Day"),
            (2026, 3, 19, "End of Ramadan (first day)"),
            (2026, 3, 20, "End of Ramadan (second day)"),
            (2026, 3, 21, "End of Ramadan (third day)"),
            (2026, 5, 1, "Labour Day"),
            (2026, 5, 27, "Eid al-Kabir"),
            (2026, 5, 28, "Day after Eid al-Kabir"),
            (2026, 6, 17, "Islamic New Year"),
            (2026, 7, 6, "Independence Day"),
            (2026, 7, 7, "Day after Independence Day"),
            (2026, 8, 26, "Prophet's Birthday"),
            (
                2026,
                11,
                12,
                "Admission of the Comoros to the United Nations",
            ),
            (2027, 2, 17, "National Reconciliation Day"),
            (2027, 7, 7, "Day after Independence Day"),
        ],
    );
    // Christmas is not a holiday; 12 November 2028 is a Sunday and stays
    // there; nothing before the decree's first full year is claimed.
    expect_working("KM", None, &[(2026, 12, 25), (2028, 11, 13), (2025, 7, 6)]);
}

#[test]
fn equatorial_guinea_gives_the_next_working_day_after_a_weekend_feast() {
    expect(
        "GQ",
        None,
        &[
            (2026, 1, 1, "New Year's Day"),
            (2026, 4, 3, "Good Friday"),
            (2026, 5, 1, "Labour Day"),
            (2026, 6, 4, "Corpus Christi"),
            (2026, 6, 5, "President's Birthday"),
            (2026, 8, 3, "Freedom Coup Day"),
            (2026, 8, 15, "Constitution Day"),
            (2026, 10, 12, "Independence Day"),
            (2026, 12, 8, "Immaculate Conception"),
            (2026, 12, 25, "Christmas Day"),
            (2007, 10, 12, "Independence Day"),
        ],
    );
    // The Ministry's notices: a Saturday Christmas in 2021 and a Saturday
    // New Year in 2022 gave the Monday; a Sunday Christmas in 2022 and a
    // Sunday New Year in 2023 did too. 15 August 2026 is a Saturday.
    expect_substitute("GQ", None, 2021, (12, 25), (12, 27));
    expect_substitute("GQ", None, 2022, (1, 1), (1, 3));
    expect_substitute("GQ", None, 2022, (12, 25), (12, 26));
    expect_substitute("GQ", None, 2023, (1, 1), (1, 2));
    expect_substitute("GQ", None, 2026, (8, 15), (8, 17));
    // Easter Monday is not in the decree; the Sunday between a Saturday
    // feast and its Monday is not a holiday; nothing before the decree.
    expect_working("GQ", None, &[(2026, 4, 6), (2026, 8, 16), (2006, 10, 12)]);
}
