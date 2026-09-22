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
    // China has no substitution rule: the 调休 table is an annual act.
    expect_working("CN", None, &[(2024, 2, 13)]);
}

#[test]
fn taiwan_holidays_and_the_nearest_weekday_adjustment() {
    expect(
        "TW",
        None,
        &[
            (2024, 2, 9, "Lunar New Year's Eve"),
            (2024, 2, 28, "Peace Memorial Day"),
            (2024, 4, 4, "Children's Day"),
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
    // Before 2014 there was no substitute at all: 3 October 2010 was a
    // Sunday.
    expect_working("KR", None, &[(2010, 10, 4)]);
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
        ],
    );
    // Singapore moves a Sunday holiday to the Monday and leaves a Saturday
    // one alone: Chinese New Year 2024 ran Saturday and Sunday.
    expect_substitute("SG", None, 2024, (2, 11), (2, 12));
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
    expect(
        "NP",
        None,
        &[(2024, 5, 1, "Labour Day"), (2025, 5, 1, "Labour Day")],
    );
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
fn israel_keeps_a_friday_saturday_weekend() {
    let calendar = HolidayCalendar::for_year(table("IL"), None, 2025);
    assert!(calendar.is_weekend(ymd(2025, 3, 7)));
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
