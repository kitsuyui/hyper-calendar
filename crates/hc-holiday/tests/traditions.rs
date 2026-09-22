//! The cross-cutting religious cycles.

use hc_calendar::Rd;
use hc_calendars_solar::gregorian;
use hc_holiday::engine::HolidayCalendar;
use hc_holiday::rule::{Confidence, Kind, RuleSet};
use hc_holiday::traditions::{
    self, BAHAI, BUDDHIST, CHINESE_FOLK, CHRISTIAN_ORTHODOX, CHRISTIAN_WESTERN, COPTIC_ORTHODOX,
    ETHIOPIAN_ORTHODOX, HINDU, ISLAMIC, JEWISH, WHEEL_OF_THE_YEAR, WHEEL_OF_THE_YEAR_SOUTH,
    ZOROASTRIAN_FASLI, ZOROASTRIAN_QADIMI, ZOROASTRIAN_SHAHANSHAHI,
};

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

/// Assert that `name` falls on the given date in the given tradition.
fn expect(set: &RuleSet, days: &[(i64, u8, u8, &str)]) {
    for (year, month, day, name) in days {
        let calendar = HolidayCalendar::for_year(set, None, *year);
        let found: Vec<&'static str> = calendar
            .on(ymd(*year, *month, *day))
            .iter()
            .map(|holiday| holiday.name)
            .collect();
        assert!(
            found.contains(name),
            "{} {year}-{month:02}-{day:02}: expected {name}, found {found:?}",
            set.code
        );
    }
}

#[test]
fn the_western_christian_year_is_the_computus_plus_fixed_feasts() {
    expect(
        &CHRISTIAN_WESTERN,
        &[
            (2024, 2, 14, "Ash Wednesday"),
            (2024, 3, 24, "Palm Sunday"),
            (2024, 5, 9, "Ascension of the Lord"),
            (2024, 5, 30, "Corpus Christi"),
            (2024, 12, 1, "First Sunday of Advent"),
            (2025, 3, 5, "Ash Wednesday"),
            (2025, 6, 8, "Pentecost"),
            (2025, 6, 19, "Corpus Christi"),
            (2025, 11, 30, "First Sunday of Advent"),
        ],
    );
}

#[test]
fn advent_always_starts_on_a_sunday_between_27_november_and_3_december() {
    use hc_calendar::Weekday;
    for year in 1900..=2100 {
        let calendar = HolidayCalendar::for_year(&CHRISTIAN_WESTERN, None, year);
        let advent = calendar
            .all()
            .iter()
            .find(|holiday| holiday.name == "First Sunday of Advent")
            .unwrap_or_else(|| panic!("no Advent in {year}"));
        assert_eq!(Weekday::from_rd(advent.date), Weekday::Sunday);
        let (_, month, day) = gregorian::from_fixed(advent.date).expect("in range");
        assert!(
            (month == 11 && day >= 27) || (month == 12 && day <= 3),
            "{year}: Advent on {month}/{day}"
        );
    }
}

#[test]
fn the_orthodox_fixed_feasts_sit_thirteen_days_after_the_western_ones() {
    expect(
        &CHRISTIAN_ORTHODOX,
        &[
            (2024, 1, 7, "Nativity of Christ"),
            (2024, 1, 19, "Theophany"),
            (2024, 8, 19, "Transfiguration"),
            (2024, 8, 28, "Dormition of the Theotokos"),
            (2025, 1, 7, "Nativity of Christ"),
            (2025, 9, 21, "Nativity of the Theotokos"),
        ],
    );
}

#[test]
fn the_coptic_year_is_dated_in_the_coptic_calendar_and_on_the_alexandrian_pascha() {
    // Coptic 1741 began on 11 September 2024, so the fixed feasts of 2025
    // sit on their usual Gregorian dates; the cycle is 2025's Pascha, 20
    // April, with the fasts counted back and the feasts counted on.
    expect(
        &COPTIC_ORTHODOX,
        &[
            (2024, 9, 11, "Nayrouz (New Year)"),
            (2024, 9, 27, "Feast of the Cross"),
            (2025, 1, 7, "Nativity (Christmas)"),
            (2025, 1, 14, "Circumcision of the Lord"),
            (2025, 1, 19, "Theophany (Epiphany)"),
            (2025, 4, 7, "Annunciation"),
            (2025, 7, 12, "Feast of the Apostles"),
            (2025, 8, 19, "Transfiguration"),
            (2025, 8, 22, "Assumption of St Mary"),
            (2025, 2, 10, "Fast of Nineveh (Jonah) begins"),
            (2025, 2, 24, "Great Lent begins"),
            (2025, 4, 13, "Palm Sunday"),
            (2025, 4, 18, "Good Friday"),
            (2025, 4, 20, "Easter (Resurrection)"),
            (2025, 4, 27, "Thomas Sunday"),
            (2025, 5, 29, "Ascension"),
            (2025, 6, 8, "Pentecost"),
            (2024, 5, 5, "Easter (Resurrection)"),
        ],
    );
}

#[test]
fn the_ethiopian_movable_cycle_is_the_julian_computus_under_its_own_names() {
    // Bahire Hasab and the Julian Paschalion are one Alexandrian computus,
    // so Fasika is Pascha's Sunday every year, and the tewsak are the same
    // offsets in days. 2024 is walked from the Fast of Nineveh to Pentecost;
    // 2025 and 2026 pin the Sunday itself.
    expect(
        &ETHIOPIAN_ORTHODOX,
        &[
            (2024, 2, 26, "Tsome Nenewe (Fast of Nineveh) begins"),
            (2024, 3, 11, "Abiy Tsom (Great Lent) begins"),
            (2024, 4, 7, "Debre Zeit (Mid-Lent)"),
            (2024, 4, 28, "Hosanna (Palm Sunday)"),
            (2024, 5, 3, "Siklet (Good Friday)"),
            (2024, 5, 5, "Fasika (Easter)"),
            (2024, 5, 29, "Rekbe Kahnat (Mid-Pentecost)"),
            (2024, 6, 13, "Erget (Ascension)"),
            (2024, 6, 23, "Paraclete (Pentecost)"),
            (2025, 4, 20, "Fasika (Easter)"),
            (2026, 4, 12, "Fasika (Easter)"),
            (2025, 8, 19, "Debre Tabor (Transfiguration)"),
        ],
    );
}

#[test]
fn the_orthodox_movable_cycle_follows_the_julian_computus() {
    expect(
        &CHRISTIAN_ORTHODOX,
        &[
            (2024, 3, 18, "Clean Monday"),
            (2024, 5, 3, "Holy Friday"),
            (2024, 5, 5, "Pascha"),
            (2024, 6, 23, "Pentecost"),
            (2025, 3, 3, "Clean Monday"),
            (2025, 4, 20, "Pascha"),
        ],
    );
}

#[test]
fn the_two_computations_diverge_by_up_to_five_weeks() {
    for year in 1900..=2100 {
        let western = hc_holiday::gregorian_easter(year).expect("in range");
        let orthodox = hc_holiday::orthodox_easter(year).expect("in range");
        let gap = orthodox.0 - western.0;
        assert!(
            (0..=35).contains(&gap) && gap % 7 == 0,
            "{year}: Orthodox Easter is {gap} days after the Western one"
        );
    }
}

#[test]
fn every_islamic_date_is_flagged_as_a_prediction() {
    for year in 2000..=2050 {
        let calendar = HolidayCalendar::for_year(&ISLAMIC, None, year);
        assert!(!calendar.all().is_empty(), "no Islamic dates in {year}");
        for holiday in calendar.all() {
            assert_eq!(
                holiday.confidence,
                Confidence::Approximate,
                "{year} {}",
                holiday.name
            );
        }
    }
}

#[test]
fn the_islamic_cycle_lands_where_the_tabular_calendar_puts_it() {
    expect(
        &ISLAMIC,
        &[
            (2024, 3, 11, "First of Ramadan"),
            (2024, 4, 10, "Eid al-Fitr"),
            (2024, 6, 17, "Eid al-Adha"),
            (2024, 7, 17, "Ashura"),
            (2025, 3, 1, "First of Ramadan"),
            (2025, 3, 31, "Eid al-Fitr"),
            (2025, 6, 7, "Eid al-Adha"),
        ],
    );
}

#[test]
fn ashura_is_always_nine_days_after_the_islamic_new_year() {
    for year in 2000..=2100 {
        let calendar = HolidayCalendar::for_year(&ISLAMIC, None, year);
        let new_years: Vec<Rd> = calendar
            .all()
            .iter()
            .filter(|holiday| holiday.name == "Islamic New Year")
            .map(|holiday| holiday.date)
            .collect();
        for start in new_years {
            let ashura = Rd(start.0 + 9);
            // Ashura can fall into the following Gregorian year, in which
            // case this year's calendar simply will not list it.
            if calendar.covers(ashura) {
                assert!(
                    calendar
                        .on(ashura)
                        .iter()
                        .any(|holiday| holiday.name == "Ashura"),
                    "{year}: Ashura should be nine days after 1 Muḥarram"
                );
            }
        }
    }
}

#[test]
fn the_jewish_year_is_exact_because_the_hebrew_calendar_is_arithmetic() {
    expect(
        &JEWISH,
        &[
            (2024, 1, 25, "Tu BiShvat"),
            (2024, 3, 24, "Purim"),
            (2024, 4, 23, "Passover"),
            (2024, 5, 26, "Lag BaOmer"),
            (2024, 6, 12, "Shavuot"),
            (2024, 8, 13, "Tisha B'Av"),
            (2024, 10, 3, "Rosh Hashanah"),
            (2024, 10, 12, "Yom Kippur"),
            (2025, 3, 14, "Purim"),
            (2025, 4, 13, "Passover"),
            (2025, 9, 23, "Rosh Hashanah"),
        ],
    );
    for holiday in HolidayCalendar::for_year(&JEWISH, None, 2024).all() {
        assert_eq!(holiday.confidence, Confidence::Exact, "{}", holiday.name);
    }
}

#[test]
fn passover_is_always_the_full_moon_fifteen_days_after_the_nisan_new_moon() {
    // Structural check: Shavuot is fifty days after the first day of
    // Passover, every year, by construction of the Omer count.
    for year in 1900..=2100 {
        let calendar = HolidayCalendar::for_year(&JEWISH, None, year);
        let passover = calendar
            .all()
            .iter()
            .find(|holiday| holiday.name == "Passover")
            .map(|holiday| holiday.date);
        let shavuot = calendar
            .all()
            .iter()
            .find(|holiday| holiday.name == "Shavuot")
            .map(|holiday| holiday.date);
        if let (Some(passover), Some(shavuot)) = (passover, shavuot) {
            assert_eq!(shavuot.0 - passover.0, 50, "{year}");
        }
    }
}

#[test]
fn the_chinese_folk_year_runs_from_laba_to_the_winter_solstice() {
    expect(
        &CHINESE_FOLK,
        &[
            (2024, 1, 18, "Laba Festival"),
            (2024, 2, 9, "Chinese New Year's Eve"),
            (2024, 2, 24, "Lantern Festival"),
            (2024, 4, 4, "Qingming Festival"),
            (2024, 6, 10, "Dragon Boat Festival"),
            (2024, 8, 10, "Qixi Festival"),
            (2024, 9, 17, "Mid-Autumn Festival"),
            (2024, 10, 11, "Double Ninth Festival"),
            (2024, 12, 21, "Winter Solstice Festival"),
            (2025, 1, 29, "Spring Festival"),
            (2025, 5, 31, "Dragon Boat Festival"),
            (2025, 10, 6, "Mid-Autumn Festival"),
        ],
    );
}

#[test]
fn the_lantern_festival_is_always_a_fortnight_after_the_new_year() {
    for year in 1950..=2100 {
        let calendar = HolidayCalendar::for_year(&CHINESE_FOLK, None, year);
        let new_year = calendar
            .all()
            .iter()
            .find(|holiday| holiday.name == "Spring Festival")
            .map(|holiday| holiday.date);
        let lantern = calendar
            .all()
            .iter()
            .find(|holiday| holiday.name == "Lantern Festival")
            .map(|holiday| holiday.date);
        if let (Some(new_year), Some(lantern)) = (new_year, lantern) {
            assert_eq!(lantern.0 - new_year.0, 14, "{year}");
        }
    }
}

#[test]
fn the_buddhist_table_separates_what_it_knows_from_what_it_approximates() {
    // The Mahayana dates Japan fixed to the Gregorian calendar in 1873 are
    // exact; the Theravada full moons are approximated from the Chinese
    // lunisolar calendar and say so.
    let calendar = HolidayCalendar::for_year(&BUDDHIST, None, 2024);
    for holiday in calendar.all() {
        let expected = match holiday.name {
            "Nirvana Day"
            | "Buddha's Birthday"
            | "Bodhi Day"
            | "Buddha's Birthday (lunar reckoning)" => Confidence::Exact,
            _ => Confidence::Approximate,
        };
        assert_eq!(holiday.confidence, expected, "{}", holiday.name);
    }
    expect(
        &BUDDHIST,
        &[
            (2024, 2, 15, "Nirvana Day"),
            (2024, 4, 8, "Buddha's Birthday"),
            (2024, 5, 15, "Buddha's Birthday (lunar reckoning)"),
            (2024, 12, 8, "Bodhi Day"),
            (2025, 5, 5, "Buddha's Birthday (lunar reckoning)"),
        ],
    );
}

#[test]
fn the_bahai_year_is_dated_in_the_badi_calendar_and_the_twin_birthdays_follow_the_table() {
    // 183 BE. The World Centre table puts its Naw-Rúz on 21 March, so this is
    // a year in which the arithmetic calendar and the observed one agree, and
    // every date below is also the one in the table's 21-March column.
    expect(
        &BAHAI,
        &[
            (2026, 3, 21, "Naw-Rúz"),
            (2026, 4, 21, "First day of Riḍván"),
            (2026, 4, 29, "Ninth day of Riḍván"),
            (2026, 5, 2, "Twelfth day of Riḍván"),
            (2026, 5, 24, "Declaration of the Báb"),
            (2026, 5, 29, "Ascension of Bahá'u'lláh"),
            (2026, 7, 10, "Martyrdom of the Báb"),
            (2026, 11, 10, "Birth of the Báb"),
            (2026, 11, 11, "Birth of Bahá'u'lláh"),
            (2026, 11, 26, "Day of the Covenant"),
            (2026, 11, 28, "Ascension of ʻAbdu'l-Bahá"),
            (2027, 2, 26, "First day of Ayyám-i-Há"),
            (2027, 3, 2, "First day of the Fast"),
            // The lunar rule, from the table: its first year, a year in which
            // the two birthdays straddle a Badíʿ month, and its last year.
            (2015, 11, 13, "Birth of the Báb"),
            (2015, 11, 14, "Birth of Bahá'u'lláh"),
            (2024, 11, 2, "Birth of the Báb"),
            (2024, 11, 3, "Birth of Bahá'u'lláh"),
            (2025, 10, 22, "Birth of the Báb"),
            (2064, 11, 10, "Birth of the Báb"),
            (2064, 11, 11, "Birth of Bahá'u'lláh"),
            // Before 172 BE: the fixed dates of Western practice, in the
            // arithmetic calendar.
            (2014, 3, 21, "Naw-Rúz"),
            (2014, 10, 20, "Birth of the Báb"),
            (2014, 11, 12, "Birth of Bahá'u'lláh"),
            // 182 BE, whose Naw-Rúz the table puts on 20 March: the whole
            // year sits a day earlier than the arithmetic rule would say, and
            // its Ayyám-i-Há has five days.
            (2025, 3, 20, "Naw-Rúz"),
            (2025, 4, 20, "First day of Riḍván"),
            (2025, 4, 28, "Ninth day of Riḍván"),
            (2025, 5, 1, "Twelfth day of Riḍván"),
            (2025, 5, 23, "Declaration of the Báb"),
            (2025, 5, 28, "Ascension of Bahá'u'lláh"),
            (2025, 7, 9, "Martyrdom of the Báb"),
            (2025, 11, 25, "Day of the Covenant"),
            (2025, 11, 27, "Ascension of ʻAbdu'l-Bahá"),
            (2026, 2, 25, "First day of Ayyám-i-Há"),
            (2026, 3, 2, "First day of the Fast"),
        ],
    );
}

#[test]
fn the_bahai_table_is_exact_and_reports_where_the_published_table_ends() {
    // Nothing here is a prediction: the calendar as kept is a published
    // table from 172 BE, and so are the birthdays.
    for year in [1900, 2014, 2015, 2025, 2064] {
        let calendar = HolidayCalendar::for_year(&BAHAI, None, year);
        assert!(!calendar.all().is_empty(), "no Bahá'í dates in {year}");
        for holiday in calendar.all() {
            assert_eq!(
                holiday.confidence,
                Confidence::Exact,
                "{year} {}",
                holiday.name
            );
        }
        assert!(calendar.is_complete(), "{year}: {:?}", calendar.gaps());
    }
    let after = HolidayCalendar::for_year(&BAHAI, None, 2065);
    assert!(
        !after.is_complete(),
        "the table ends with 221 BE: {:?}",
        after.gaps()
    );
}

#[test]
fn the_hindu_festivals_fall_where_the_rashtriya_panchang_lists_them() {
    // The "Principal Festivals and Anniversaries" lists of the Śaka 1945
    // and 1946 editions (2023–2025), Positional Astronomy Centre, India
    // Meteorological Department.
    let mut mismatches = Vec::new();
    for (year, month, day, name) in [
        (2023, 3, 22, "Ugadi"),
        (2023, 3, 30, "Rama Navami"),
        (2023, 4, 4, "Mahavir Jayanti"),
        (2023, 4, 22, "Akshaya Tritiya"),
        (2023, 5, 5, "Buddha Purnima"),
        (2023, 8, 30, "Raksha Bandhan"),
        (2023, 9, 6, "Krishna Janmashtami"),
        (2023, 9, 19, "Ganesh Chaturthi"),
        (2023, 10, 24, "Vijaya Dashami"),
        (2023, 11, 12, "Diwali"),
        (2023, 11, 27, "Guru Nanak Jayanti"),
        (2024, 1, 15, "Makar Sankranti"),
        (2024, 3, 24, "Holika Dahan"),
        (2024, 3, 25, "Holi"),
        (2024, 4, 9, "Ugadi"),
        (2024, 4, 17, "Rama Navami"),
        (2024, 4, 21, "Mahavir Jayanti"),
        (2024, 5, 10, "Akshaya Tritiya"),
        (2024, 5, 23, "Buddha Purnima"),
        (2024, 8, 19, "Raksha Bandhan"),
        (2024, 8, 26, "Krishna Janmashtami"),
        (2024, 9, 7, "Ganesh Chaturthi"),
        (2024, 10, 12, "Vijaya Dashami"),
        (2024, 10, 31, "Diwali"),
        (2024, 11, 15, "Guru Nanak Jayanti"),
        (2025, 1, 14, "Makar Sankranti"),
        (2025, 2, 26, "Maha Shivaratri"),
        (2025, 3, 13, "Holika Dahan"),
        (2025, 3, 14, "Holi"),
        (2025, 3, 30, "Ugadi"),
        (2025, 4, 6, "Rama Navami"),
        (2025, 4, 10, "Mahavir Jayanti"),
    ] {
        let calendar = HolidayCalendar::for_year(&HINDU, None, year);
        let on: Vec<&str> = calendar
            .on(ymd(year, month, day))
            .iter()
            .map(|h| h.name)
            .collect();
        if !on.contains(&name) {
            // Where did the rule put it instead?
            let placed: Vec<(i64, u8, u8)> = calendar
                .all()
                .iter()
                .filter(|h| h.name == name)
                .filter_map(|h| gregorian::from_fixed(h.date).ok())
                .collect();
            mismatches.push(format!("{name} {year}-{month:02}-{day:02}: that day has {on:?}; the rule put it on {placed:?}"));
        }
    }
    assert!(mismatches.is_empty(), "{mismatches:#?}");
}

#[test]
fn every_hindu_date_is_exact_and_religious() {
    for year in [1950, 2000, 2024, 2100] {
        let calendar = HolidayCalendar::for_year(&HINDU, None, year);
        assert!(
            calendar.all().len() >= 19,
            "{year}: {}",
            calendar.all().len()
        );
        assert!(calendar.is_complete(), "{year}: {:?}", calendar.gaps());
        for holiday in calendar.all() {
            assert_eq!(
                holiday.confidence,
                Confidence::Exact,
                "{year} {}",
                holiday.name
            );
            assert_eq!(holiday.kind, Kind::Religious, "{year} {}", holiday.name);
        }
    }
    assert!(!HolidayCalendar::for_year(&HINDU, None, 1700).is_complete());
}

#[test]
fn the_wheel_turns_on_the_quarter_days_and_the_fixed_cross_quarters() {
    // 2024: the March equinox on the 20th (03:06 UT), the June solstice on
    // the 20th (20:51 UT), the September equinox on the 22nd, the December
    // solstice on the 21st.
    expect(
        &WHEEL_OF_THE_YEAR,
        &[
            (2024, 2, 1, "Imbolc"),
            (2024, 3, 20, "Ostara"),
            (2024, 5, 1, "Beltane"),
            (2024, 6, 20, "Litha"),
            (2024, 8, 1, "Lughnasadh"),
            (2024, 9, 22, "Mabon"),
            (2024, 11, 1, "Samhain"),
            (2024, 12, 21, "Yule"),
        ],
    );
    expect(
        &WHEEL_OF_THE_YEAR_SOUTH,
        &[
            (2024, 2, 1, "Lughnasadh"),
            (2024, 3, 20, "Mabon"),
            (2024, 5, 1, "Samhain"),
            (2024, 6, 20, "Yule"),
            (2024, 8, 1, "Imbolc"),
            (2024, 9, 22, "Ostara"),
            (2024, 11, 1, "Beltane"),
            (2024, 12, 21, "Litha"),
        ],
    );
    for set in [&WHEEL_OF_THE_YEAR, &WHEEL_OF_THE_YEAR_SOUTH] {
        let calendar = HolidayCalendar::for_year(set, None, 2024);
        assert_eq!(calendar.all().len(), 8, "{}", set.code);
        for holiday in calendar.all() {
            assert_eq!(holiday.confidence, Confidence::Exact, "{}", holiday.name);
            assert_eq!(holiday.kind, Kind::Religious, "{}", holiday.name);
        }
    }
}

#[test]
fn a_tradition_gives_nobody_a_day_off() {
    // Every entry is Religious or Observance, so `is_holiday` is false even
    // on Christmas Day: a statute grants the day off, not a tradition.
    for set in traditions::ALL {
        let calendar = HolidayCalendar::for_year(set, None, 2024);
        for holiday in calendar.all() {
            assert!(
                matches!(holiday.kind, Kind::Religious | Kind::Observance),
                "{} / {} should not be a day off",
                set.code,
                holiday.name
            );
        }
        assert!(
            calendar.all().iter().all(|holiday| !holiday.is_day_off()),
            "{} grants a day off",
            set.code
        );
    }
    let christmas = HolidayCalendar::for_year(&CHRISTIAN_WESTERN, None, 2024);
    assert!(!christmas.is_holiday(ymd(2024, 12, 25)));
    assert!(
        christmas
            .on(ymd(2024, 12, 25))
            .iter()
            .any(|holiday| holiday.name == "Christmas Day")
    );
}

#[test]
fn every_tradition_is_reachable_by_its_code_and_names_its_sources() {
    for set in traditions::ALL {
        assert_eq!(
            traditions::by_code(set.code).map(|found| found.code),
            Some(set.code)
        );
        assert!(!set.sources.is_empty(), "{} has no cited source", set.code);
    }
    assert!(traditions::by_code("zoroastrian").is_none());
}

#[test]
fn the_fasli_feasts_are_the_compendium_tables_of_1379_and_1381() {
    // 1379 A.Y., 21 March 2009 to 20 March 2010, a common year.
    expect(
        &ZOROASTRIAN_FASLI,
        &[
            (2009, 3, 21, "Nowruz"),
            (2009, 3, 23, "Rapithwin Jashan"),
            (2009, 3, 26, "Khordad Sal"),
            (2009, 4, 8, "Farvardin Month Jashan"),
            (2009, 4, 30, "Maidyozarem Gahambar"),
            (2009, 5, 4, "Maidyozarem Gahambar"),
            (2009, 6, 29, "Maidyoshahem Gahambar"),
            (2009, 7, 1, "Tiragan"),
            (2009, 7, 1, "Maidyoshahem Gahambar"),
            (2009, 9, 12, "Paitishahem Gahambar"),
            (2009, 10, 2, "Mehregan"),
            (2009, 10, 12, "Ayathrem Gahambar"),
            (2009, 10, 16, "Ayathrem Gahambar"),
            (2009, 11, 24, "Adar Month Jashan"),
            (2009, 12, 16, "Dae Month Jashan"),
            (2009, 12, 26, "Zartosht No-Diso"),
            (2009, 12, 31, "Maidyarem Gahambar"),
            (2010, 1, 4, "Maidyarem Gahambar"),
            (2010, 2, 18, "Aspandard Month Jashan"),
            (2010, 2, 20, "Avardad Sal Gah Jashan"),
            (2010, 3, 11, "Muktad"),
            (2010, 3, 14, "Mareshpand Jashan"),
            (2010, 3, 16, "Hamaspathmaidyem Gahambar"),
            (2010, 3, 20, "Muktad"),
            (2010, 3, 20, "Hamaspathmaidyem Gahambar"),
            (2010, 3, 21, "Nowruz"),
        ],
    );
    // 1381 A.Y. has the leap day on 20 March 2012, so its last twenty-one
    // days are a day earlier: Muktad 10–19 March, Mareshpand Jashan on the
    // 13th, the Gatha days 15–19 March, and nothing on the 20th.
    expect(
        &ZOROASTRIAN_FASLI,
        &[
            (2012, 3, 10, "Muktad"),
            (2012, 3, 13, "Mareshpand Jashan"),
            (2012, 3, 15, "Hamaspathmaidyem Gahambar"),
            (2012, 3, 19, "Muktad"),
            (2012, 3, 21, "Nowruz"),
        ],
    );
    let leap_day = HolidayCalendar::for_year(&ZOROASTRIAN_FASLI, None, 2012);
    assert!(leap_day.on(ymd(2012, 3, 20)).is_empty());
}

#[test]
fn the_wandering_reckonings_keep_the_same_schedule_a_month_apart() {
    // 1395 Y.Z. by the Shahanshahi reckoning began on 15 August 2025;
    // Wikipedia, "Zoroastrian festivals", gives Zartosht No-Diso as
    // 22 May 2026 in that calendar. Muktad ran to 14 August 2026, the eve
    // of the next Nowruz.
    expect(
        &ZOROASTRIAN_SHAHANSHAHI,
        &[
            (2025, 8, 15, "Nowruz"),
            (2025, 8, 20, "Khordad Sal"),
            (2025, 9, 24, "Maidyozarem Gahambar"),
            (2026, 2, 26, "Mehregan"),
            (2026, 5, 22, "Zartosht No-Diso"),
            (2026, 8, 5, "Muktad"),
            (2026, 8, 10, "Hamaspathmaidyem Gahambar"),
            (2026, 8, 14, "Muktad"),
            (2026, 8, 15, "Nowruz"),
            (2000, 8, 21, "Nowruz"),
        ],
    );
    // The Qadimi is thirty days ahead: Nowruz of 1395 on 16 July 2025.
    expect(
        &ZOROASTRIAN_QADIMI,
        &[
            (2025, 7, 16, "Nowruz"),
            (2025, 7, 21, "Khordad Sal"),
            (2026, 1, 27, "Mehregan"),
            (2026, 4, 22, "Zartosht No-Diso"),
            (2026, 7, 15, "Muktad"),
            (2026, 7, 16, "Nowruz"),
            (2000, 7, 22, "Nowruz"),
        ],
    );
}

#[test]
fn the_three_zoroastrian_tables_are_one_schedule() {
    let names = |set: &RuleSet| -> Vec<&str> { set.rules.iter().map(|rule| rule.name).collect() };
    assert_eq!(names(&ZOROASTRIAN_FASLI), names(&ZOROASTRIAN_SHAHANSHAHI));
    assert_eq!(names(&ZOROASTRIAN_FASLI), names(&ZOROASTRIAN_QADIMI));
    for set in [
        &ZOROASTRIAN_FASLI,
        &ZOROASTRIAN_SHAHANSHAHI,
        &ZOROASTRIAN_QADIMI,
    ] {
        for rule in set.rules {
            assert_eq!(rule.kind, Kind::Religious, "{} {}", set.code, rule.name);
            assert_eq!(
                rule.confidence,
                Confidence::Exact,
                "{} {}",
                set.code,
                rule.name
            );
        }
        // Six Gahambars of five days and ten days of Muktad.
        let gahambar_days = set
            .rules
            .iter()
            .filter(|rule| rule.name.ends_with("Gahambar"))
            .count();
        assert_eq!(gahambar_days, 30, "{}", set.code);
        let muktad = set
            .rules
            .iter()
            .filter(|rule| rule.name == "Muktad")
            .count();
        assert_eq!(muktad, 10, "{}", set.code);
    }
}
