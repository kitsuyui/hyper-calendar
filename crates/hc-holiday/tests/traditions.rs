//! The cross-cutting religious cycles.

use std::sync::OnceLock;

use hc_calendar::Rd;
use hc_calendars_indic::nakshatra::PUSHYA;
use hc_calendars_solar::gregorian;
use hc_holiday::engine::HolidayCalendar;
use hc_holiday::hindu::THAIPUSAM;
use hc_holiday::rule::{Confidence, Kind, Rule, RuleSet};
use hc_holiday::traditions::{
    self, BAHAI, BUDDHIST_EAST_ASIAN, BUDDHIST_THAI, BUDDHIST_TIBETAN, BUDDHIST_UPOSATHA_THAI,
    CHAHARSHANBE_SURI, CHINESE_FOLK, CHINESE_XIAONIAN_JIANGNAN, CHINESE_XIAONIAN_NANJING,
    CHINESE_XIAONIAN_NORTH, CHINESE_XIAONIAN_SOUTH, CHINESE_XIAONIAN_SOUTHWEST, CHRISTIAN_ARMENIAN,
    CHRISTIAN_ARMENIAN_JERUSALEM, CHRISTIAN_ORTHODOX, CHRISTIAN_ORTHODOX_REVISED_JULIAN,
    CHRISTIAN_WESTERN, CHURCH_OF_THE_EAST, COPTIC_ORTHODOX, EMBER_BCP1662, EMBER_COMMON_WORSHIP,
    ETHIOPIAN_ORTHODOX, GOSEKKU, HINDU, ISLAMIC, JAIN, JEWISH, KOREAN_FOLK, KYUCHU_SAISHI,
    MANDAEAN, PLOUGH_DAYS, ROGATION_ROMAN_1960, SACRED_WEDNESDAYS, SAMARITAN, SHINTO,
    SIKH_NANAKSHAHI_2003, SIKH_SGPC, TAOIST, TENRIKYO, UNLUCKY_FRIDAYS, VIETNAMESE_FOLK,
    WHEEL_OF_THE_YEAR, WHEEL_OF_THE_YEAR_SOUTH, YAZIDI, ZOROASTRIAN_FASLI, ZOROASTRIAN_QADIMI,
    ZOROASTRIAN_SHAHANSHAHI,
};
use hc_seasons::ColdFoodConvention;
use hc_seasons::zodiac::{Ayanamsa, SiderealSign};
use hc_seasons::{Meridian, SolarTerm};

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
fn the_revised_julian_fixed_feasts_are_the_gregorian_dates_and_pascha_the_julian() {
    // The Revised Julian calendar agrees with the Gregorian from 1600 to
    // 2800, so the Nativity is on 25 December; Pascha is the same Sunday as
    // on the Julian calendar, 12 April in 2026.
    expect(
        &CHRISTIAN_ORTHODOX_REVISED_JULIAN,
        &[
            (2025, 12, 25, "Nativity of Christ"),
            (2026, 1, 6, "Theophany"),
            (2026, 3, 25, "Annunciation"),
            (2026, 8, 15, "Dormition of the Theotokos"),
            (2026, 2, 23, "Clean Monday"),
            (2026, 4, 12, "Pascha"),
            (2026, 5, 31, "Pentecost"),
        ],
    );
    let julian = HolidayCalendar::for_year(&CHRISTIAN_ORTHODOX, None, 2026);
    let revised = HolidayCalendar::for_year(&CHRISTIAN_ORTHODOX_REVISED_JULIAN, None, 2026);
    assert!(revised.on(ymd(2026, 1, 7)).is_empty());
    assert!(!julian.on(ymd(2026, 1, 7)).is_empty());
    let pascha = |calendar: &HolidayCalendar| {
        calendar
            .all()
            .iter()
            .find(|holiday| holiday.name == "Pascha")
            .map(|holiday| holiday.date)
    };
    assert_eq!(pascha(&julian), pascha(&revised));
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

/// The Nativity on 25 December Julian: St-Takla's 7 January, 28 Koiak in
/// the Gregorian leap years (`st-takla-nativity-fast`), and the first day
/// of the Metropolis of the Southern United States's Nativity in every year
/// of 2000–2100 it lists (`suscopts-fasts`), not 29 Koiak, 8 January 2024;
/// Fr. John Ramzy's 8 January after 2100 (`ramzy-nativity-2004`).
#[test]
fn the_coptic_nativity_is_kept_on_the_julian_christmas() {
    let nativity = |year: i64| {
        HolidayCalendar::for_year(&COPTIC_ORTHODOX, None, year)
            .all()
            .iter()
            .filter(|holiday| holiday.name == "Nativity (Christmas)")
            .map(|holiday| holiday.date)
            .collect::<Vec<_>>()
    };
    for year in 2000..=2100 {
        assert_eq!(nativity(year), [ymd(year, 1, 7)], "{year}");
    }
    assert_eq!(nativity(2101), [ymd(2101, 1, 8)]);
    // 1850: 25 December 1849 Julian is 6 January.
    assert_eq!(nativity(1850), [ymd(1850, 1, 6)]);
    // Theophany, 11 Tobi, still moves with the Coptic calendar: 20 January
    // 2024, as the Metropolis dates it.
    expect(&COPTIC_ORTHODOX, &[(2024, 1, 20, "Theophany (Epiphany)")]);
}

/// Genna on 25 December Julian, 28 Tahsas after an Ethiopic leap year
/// (`eotc-ma-calendar`): 7 January 2024, when 29 Tahsas is the 8th.
#[test]
fn genna_is_kept_on_the_julian_christmas() {
    expect(
        &ETHIOPIAN_ORTHODOX,
        &[
            (2024, 1, 7, "Genna (Christmas)"),
            (2026, 1, 7, "Genna (Christmas)"),
            (2028, 1, 7, "Genna (Christmas)"),
            (2024, 1, 20, "Timkat (Epiphany)"),
        ],
    );
    assert!(
        HolidayCalendar::for_year(&ETHIOPIAN_ORTHODOX, None, 2024)
            .on(ymd(2024, 1, 8))
            .is_empty()
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
    for year in swept_years() {
        let calendar = chinese_folk(year);
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
fn the_thai_buddhist_days_are_the_bank_of_thailand_dates() {
    // 2012 and 2023 are adhikamāsa years: Makha and Visakha Bucha a month
    // later, Asalha Bucha on the second month 8.
    expect(
        &BUDDHIST_THAI,
        &[
            (2012, 3, 7, "Makha Bucha"),
            (2012, 6, 4, "Visakha Bucha"),
            (2012, 8, 2, "Asalha Bucha"),
            (2012, 8, 3, "Khao Phansa"),
            (2023, 3, 6, "Makha Bucha"),
            (2023, 6, 3, "Visakha Bucha"),
            (2025, 2, 12, "Makha Bucha"),
            (2025, 5, 11, "Visakha Bucha"),
            (2025, 7, 10, "Asalha Bucha"),
            (2025, 7, 11, "Khao Phansa"),
        ],
    );
    let calendar = HolidayCalendar::for_year(&BUDDHIST_THAI, None, 2025);
    for holiday in calendar.all() {
        assert_eq!(holiday.confidence, Confidence::Exact, "{}", holiday.name);
        assert_eq!(holiday.kind, Kind::Religious, "{}", holiday.name);
    }
    // Outside the years Thailand published, a gap and not a guess.
    let outside = HolidayCalendar::for_year(&BUDDHIST_THAI, None, 2030);
    assert!(outside.all().is_empty());
    assert!(!outside.is_complete());
}

#[test]
fn the_east_asian_buddhist_days_are_exact() {
    let calendar = HolidayCalendar::for_year(&BUDDHIST_EAST_ASIAN, None, 2024);
    for holiday in calendar.all() {
        assert_eq!(holiday.confidence, Confidence::Exact, "{}", holiday.name);
    }
    expect(
        &BUDDHIST_EAST_ASIAN,
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

/// Drik Panchang's pages for New Delhi (`drik-holika-dahan`, read
/// 2026-10-03) give Holikā Dahana on 23 March 2016, 7 March 2023 and
/// 3 March 2026, a day after the evening of the full-moon tithi, and Holī
/// the day after each; the tradition's rule gives the day before. The table
/// carries the pages' days exactly for 2015 to 2036 and the rule's
/// approximately outside them.
#[test]
fn holika_dahan_and_holi_are_the_days_drik_panchang_gives_for_2015_to_2036() {
    expect(
        &HINDU,
        &[
            (2015, 3, 5, "Holika Dahan"),
            (2015, 3, 6, "Holi"),
            (2016, 3, 23, "Holika Dahan"),
            (2016, 3, 24, "Holi"),
            (2023, 3, 7, "Holika Dahan"),
            (2023, 3, 8, "Holi"),
            (2026, 3, 3, "Holika Dahan"),
            (2026, 3, 4, "Holi"),
            (2027, 3, 21, "Holika Dahan"),
            (2027, 3, 22, "Holi"),
            (2029, 2, 28, "Holika Dahan"),
            (2029, 3, 1, "Holi"),
            (2036, 3, 11, "Holika Dahan"),
            (2036, 3, 12, "Holi"),
        ],
    );
    for year in 2015..=2036 {
        let calendar = HolidayCalendar::for_year(&HINDU, None, year);
        for name in ["Holika Dahan", "Holi"] {
            let days: Vec<_> = calendar
                .all()
                .iter()
                .filter(|holiday| holiday.name == name)
                .collect();
            assert_eq!(days.len(), 1, "{name} {year}");
            assert_eq!(days[0].confidence, Confidence::Exact, "{name} {year}");
        }
    }
    // Before and after the pages the rule answers, approximately.
    for year in [2014, 2037] {
        let calendar = HolidayCalendar::for_year(&HINDU, None, year);
        let days: Vec<_> = calendar
            .all()
            .iter()
            .filter(|holiday| holiday.name == "Holika Dahan" || holiday.name == "Holi")
            .collect();
        assert_eq!(days.len(), 2, "{year}");
        assert!(
            days.iter()
                .all(|holiday| holiday.confidence == Confidence::Approximate),
            "{year}"
        );
    }
}

/// Naraka Caturdaśī by the Calendar Reform Committee's rule: the Naraka
/// Chaturdasi of the Department of Personnel and Training's restricted
/// holidays, "November 08 … Sunday" in 2026, the day of Diwali, and
/// "October 28 … Thursday" in 2027, the day before it
/// (`dopt-holidays-2025-2027`).
#[test]
fn naraka_chaturdashi_is_the_central_governments_day() {
    expect(
        &HINDU,
        &[
            (2026, 11, 8, "Naraka Chaturdashi"),
            (2026, 11, 8, "Diwali"),
            (2027, 10, 28, "Naraka Chaturdashi"),
            (2027, 10, 29, "Diwali"),
        ],
    );
}

#[test]
fn a_first_tithi_that_no_sunrise_carries_still_opens_the_year() {
    // Chaitra śukla pratipadā of Śaka 1948 begins at 06:52 IST on 19 March
    // 2026, after that morning's sunrise, and ends at 04:52 on the 20th,
    // before the next: no sunrise carries it, the month's first day by
    // sunrise is the 20th with the second tithi, and Ugadi is the 19th,
    // the day the tithi begins in.
    expect(&HINDU, &[(2026, 3, 19, "Ugadi")]);
}

#[test]
fn thaipusam_falls_where_malaysia_and_mauritius_gazette_it() {
    // Malaysia and Mauritius, 2020 to 2026, as Office Holidays records the
    // gazetted days: the same day every year but 2023, when Puṣya ran from
    // the morning of 4 February to the afternoon of the 5th and each
    // country's midnight cut it on its own side. India's meridian gives
    // Mauritius's day.
    let mauritius = Meridian::from_seconds(4 * 3_600);
    let malaysia = Meridian::from_seconds(8 * 3_600);
    let at = |meridian| Rule::Nakshatra {
        nakshatra: PUSHYA,
        sign: SiderealSign::MAKARA,
        with_tithi: Some(15),
        ayanamsa: &Ayanamsa::LAHIRI,
        meridian,
    };
    for (year, month, day) in [
        (2020, 2, 8),
        (2021, 1, 28),
        (2022, 1, 18),
        (2023, 2, 4),
        (2024, 1, 25),
        (2025, 2, 11),
        (2026, 2, 1),
    ] {
        let expected = [ymd(year, month, day)];
        assert_eq!(
            THAIPUSAM.days_in_year(year).as_slice(),
            expected,
            "{year} at India"
        );
        assert_eq!(
            at(mauritius).days_in_year(year).as_slice(),
            expected,
            "{year} at Mauritius"
        );
        if year != 2023 {
            assert_eq!(
                at(malaysia).days_in_year(year).as_slice(),
                expected,
                "{year} at Malaysia"
            );
        }
    }
    assert_eq!(
        at(malaysia).days_in_year(2023).as_slice(),
        [ymd(2023, 2, 5)]
    );
    // Drik Panchang's computed days for Kuala Lumpur, 2027 and 2028.
    assert_eq!(
        at(malaysia).days_in_year(2027).as_slice(),
        [ymd(2027, 1, 22)]
    );
    assert_eq!(
        at(malaysia).days_in_year(2028).as_slice(),
        [ymd(2028, 2, 9)]
    );
}

#[test]
fn every_hindu_date_is_religious_and_exact_but_two_outside_the_years_read() {
    for year in [1950, 2000, 2024, 2100] {
        let calendar = HolidayCalendar::for_year(&HINDU, None, year);
        assert!(
            calendar.all().len() >= 19,
            "{year}: {}",
            calendar.all().len()
        );
        assert!(calendar.is_complete(), "{year}: {:?}", calendar.gaps());
        for holiday in calendar.all() {
            // Holikā Dahana and Holī are Drik Panchang's days for 2015 to
            // 2036 and the rule's, approximately, outside them.
            let drik_years = (2015..=2036).contains(&year);
            let expected = if matches!(holiday.name, "Holika Dahan" | "Holi") && !drik_years {
                Confidence::Approximate
            } else {
                Confidence::Exact
            };
            assert_eq!(holiday.confidence, expected, "{year} {}", holiday.name);
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

#[test]
fn the_sikh_gurpurabs_fall_on_the_nanakshahi_tables_fixed_dates() {
    expect(
        &SIKH_NANAKSHAHI_2003,
        &[
            (2019, 1, 5, "Parkash of Guru Gobind Singh"),
            (2019, 1, 31, "Parkash of Guru Har Rai"),
            (2019, 3, 14, "Nanakshahi New Year"),
            (2019, 3, 14, "Gurgaddi of Guru Har Rai"),
            (2019, 4, 14, "Vaisakhi"),
            (2019, 4, 16, "Gurgaddi of Guru Tegh Bahadur"),
            (2019, 4, 18, "Parkash of Guru Angad"),
            (2019, 5, 2, "Parkash of Guru Arjan"),
            (2019, 6, 16, "Shaheedi of Guru Arjan"),
            (2019, 7, 5, "Parkash of Guru Hargobind"),
            (2019, 7, 23, "Parkash of Guru Harkrishan"),
            (2019, 9, 1, "First Parkash of the Guru Granth Sahib"),
            (2019, 9, 22, "Joti Jot of Guru Nanak"),
            (2019, 10, 9, "Parkash of Guru Ram Das"),
            (2019, 10, 20, "Gurgaddi of the Guru Granth Sahib"),
            (2019, 10, 21, "Joti Jot of Guru Gobind Singh"),
            (2019, 11, 24, "Shaheedi of Guru Tegh Bahadur"),
            (2019, 12, 21, "Shaheedi of the Elder Sahibzadas"),
            (2019, 12, 26, "Shaheedi of the Younger Sahibzadas"),
            // A leap year moves nothing: the fixed dates are Gregorian.
            (2024, 1, 5, "Parkash of Guru Gobind Singh"),
            (2024, 3, 14, "Nanakshahi New Year"),
            (2024, 4, 14, "Vaisakhi"),
        ],
    );
}

#[test]
fn the_three_lunar_sikh_days_match_the_sgpc_list() {
    // The article's table of movable dates, 2018 to 2020.
    expect(
        &SIKH_NANAKSHAHI_2003,
        &[
            (2018, 3, 2, "Hola Mohalla"),
            (2018, 11, 7, "Bandi Chhor Divas"),
            (2018, 11, 23, "Parkash of Guru Nanak"),
            (2019, 3, 21, "Hola Mohalla"),
            (2019, 10, 27, "Bandi Chhor Divas"),
            (2019, 11, 12, "Parkash of Guru Nanak"),
            (2020, 3, 10, "Hola Mohalla"),
            (2020, 11, 14, "Bandi Chhor Divas"),
            (2020, 11, 30, "Parkash of Guru Nanak"),
        ],
    );
    // The same table's Hola Mohalla of 2003 to 2008, the day after Holi as
    // the fitted rule has it. Its 11 March 2009 is Holi itself (Drik
    // Panchang, New Delhi), where Chet vadi 1 at sunrise is the 12th: the
    // one year of 2003–2020 the rule does not give.
    expect(
        &SIKH_NANAKSHAHI_2003,
        &[
            (2003, 3, 19, "Hola Mohalla"),
            (2004, 3, 7, "Hola Mohalla"),
            (2005, 3, 26, "Hola Mohalla"),
            (2006, 3, 15, "Hola Mohalla"),
            (2007, 3, 4, "Hola Mohalla"),
            (2008, 3, 22, "Hola Mohalla"),
            (2009, 3, 12, "Hola Mohalla"),
        ],
    );
    expect_not(&SIKH_NANAKSHAHI_2003, &[(2009, 3, 11, "Hola Mohalla")]);
    // Every day is exact but Hola Mohalla's, whose sunrise is fitted.
    for rule in SIKH_NANAKSHAHI_2003.rules {
        assert_eq!(rule.kind, Kind::Religious, "{}", rule.name);
        let expected = if rule.name == "Hola Mohalla" {
            Confidence::Approximate
        } else {
            Confidence::Exact
        };
        assert_eq!(rule.confidence, expected, "{}", rule.name);
    }
}

#[test]
fn the_sgpc_keeps_guru_gobind_singhs_parkash_on_poh_sudi_seven() {
    // The Tribune, 16 January 2024: the SGPC's Parkash of 17 January 2024,
    // and of 9 January and 29 December 2022, twice in one year. The
    // Nanakshahi table of 2003 keeps 5 January.
    expect(
        &SIKH_SGPC,
        &[
            (2022, 1, 9, "Parkash of Guru Gobind Singh"),
            (2022, 12, 29, "Parkash of Guru Gobind Singh"),
            (2024, 1, 17, "Parkash of Guru Gobind Singh"),
            // SikhNet's list "as per SGPC Calendar": 15 January 2027,
            // and Vaisakhi and Guru Nanak's Parkash of 2026.
            (2027, 1, 15, "Parkash of Guru Gobind Singh"),
            (2026, 4, 14, "Vaisakhi"),
            (2026, 11, 24, "Parkash of Guru Nanak"),
        ],
    );
    expect_not(&SIKH_SGPC, &[(2024, 1, 5, "Parkash of Guru Gobind Singh")]);
    for rule in SIKH_SGPC.rules {
        assert_eq!(rule.kind, Kind::Religious, "{}", rule.name);
    }
}

#[test]
fn the_sgpc_keeps_the_three_movable_days_of_both_versions() {
    // Wikipedia's table of the movable dates of the 2003 and 2010
    // versions, 2010 to 2017, which the 2003 table's test reads for 2018
    // to 2020. Hola Mohalla is Chet vadi 1 at sunrise, a day after Holi
    // in 2012, 2013 and 2016.
    let mut days = Vec::new();
    for (year, hola, bandi, nanak) in [
        (2010, (3, 1), (11, 5), (11, 21)),
        (2011, (3, 20), (10, 26), (11, 10)),
        (2012, (3, 9), (11, 13), (11, 28)),
        (2013, (3, 28), (11, 3), (11, 17)),
        (2014, (3, 17), (10, 23), (11, 6)),
        (2015, (3, 6), (11, 11), (11, 25)),
        (2016, (3, 24), (10, 30), (11, 14)),
        (2017, (3, 13), (10, 19), (11, 4)),
    ] {
        days.push((year, hola.0, hola.1, "Hola Mohalla"));
        days.push((year, bandi.0, bandi.1, "Bandi Chhor Divas"));
        days.push((year, nanak.0, nanak.1, "Parkash of Guru Nanak"));
    }
    expect(&SIKH_SGPC, &days);
    expect(&SIKH_NANAKSHAHI_2003, &days);
    expect_not(
        &SIKH_SGPC,
        &[
            (2012, 3, 8, "Hola Mohalla"),
            (2013, 3, 27, "Hola Mohalla"),
            (2016, 3, 23, "Hola Mohalla"),
        ],
    );
}

#[test]
fn the_sgpc_days_of_2021_to_2026_are_reproduced() {
    // Hola Mohalla: SikhNet's SGPC lists for 2021 and 2026; dekho-ji for
    // 2022 and 2023; The Tribune, 27 March 2024; AIR, 15 March 2025.
    // Vaisakhi: The Tribune's reports of every year from 2010 to 2025 from
    // Amritsar, Anandpur Sahib and Talwandi Sabo (tribune-baisakhi-2010,
    // tribune-baisakhi-2011, tribune-baisakhi-2012, tribune-baisakhi-2013,
    // tribune-baisakhi-2014, tribune-baisakhi-2015, tribune-baisakhi-2016,
    // tribune-baisakhi-2017, tribune-baisakhi-2018, tribune-baisakhi-2019,
    // tribune-baisakhi-2020, tribune-baisakhi-2021, tribune-baisakhi-2022,
    // tribune-baisakhi-2023, tribune-baisakhi-2024 and
    // tribune-baisakhi-2025), The Week of 17 March 2025 and SikhNet's 2021 and 2026 lists;
    // 13 April in 2017, 2021 and 2025, when the saṅkrānti fell between
    // midnight and sunrise. Bandi Chhor Divas: the published days,
    // a day after Diwali in 2024 and 2025.
    expect(
        &SIKH_SGPC,
        &[
            (2021, 3, 29, "Hola Mohalla"),
            (2022, 3, 19, "Hola Mohalla"),
            (2023, 3, 8, "Hola Mohalla"),
            (2024, 3, 26, "Hola Mohalla"),
            (2025, 3, 15, "Hola Mohalla"),
            (2026, 3, 4, "Hola Mohalla"),
            (2010, 4, 14, "Vaisakhi"),
            (2011, 4, 14, "Vaisakhi"),
            (2012, 4, 13, "Vaisakhi"),
            (2013, 4, 13, "Vaisakhi"),
            (2014, 4, 14, "Vaisakhi"),
            (2015, 4, 14, "Vaisakhi"),
            (2016, 4, 13, "Vaisakhi"),
            (2017, 4, 13, "Vaisakhi"),
            (2018, 4, 14, "Vaisakhi"),
            (2019, 4, 14, "Vaisakhi"),
            (2020, 4, 13, "Vaisakhi"),
            (2021, 4, 13, "Vaisakhi"),
            (2022, 4, 14, "Vaisakhi"),
            (2023, 4, 14, "Vaisakhi"),
            (2024, 4, 13, "Vaisakhi"),
            (2025, 4, 13, "Vaisakhi"),
            (2026, 4, 14, "Vaisakhi"),
            (2021, 11, 4, "Bandi Chhor Divas"),
            (2022, 10, 24, "Bandi Chhor Divas"),
            (2023, 11, 12, "Bandi Chhor Divas"),
            (2024, 11, 1, "Bandi Chhor Divas"),
            (2025, 10, 21, "Bandi Chhor Divas"),
            (2026, 11, 8, "Bandi Chhor Divas"),
        ],
    );
    expect_not(
        &SIKH_SGPC,
        &[
            (2024, 3, 25, "Hola Mohalla"),
            (2025, 3, 14, "Hola Mohalla"),
            (2026, 3, 3, "Hola Mohalla"),
            (2025, 4, 14, "Vaisakhi"),
            (2024, 10, 31, "Bandi Chhor Divas"),
            (2025, 10, 20, "Bandi Chhor Divas"),
        ],
    );
    // Outside the published days Bandi Chhor Divas is a gap, not a year
    // without it.
    for year in [2009, 2027] {
        let calendar = HolidayCalendar::for_year(&SIKH_SGPC, None, year);
        assert!(
            calendar
                .gaps()
                .iter()
                .any(|gap| gap.name == "Bandi Chhor Divas"),
            "{year}"
        );
    }
    let calendar = HolidayCalendar::for_year(&SIKH_SGPC, None, 2026);
    assert!(calendar.gaps().is_empty(), "{:?}", calendar.gaps());
    // The fitted Hola Mohalla is approximate; the others are exact.
    for rule in SIKH_SGPC.rules {
        let fitted = rule.name == "Hola Mohalla";
        assert_eq!(
            rule.confidence == Confidence::Approximate,
            fitted,
            "{}",
            rule.name
        );
    }
}

#[test]
fn the_vikrami_months_begin_on_the_sgpcs_sangrands() {
    use hc_calendars_indic::hindu_solar::{HinduSolarDate, VIKRAMI};
    // SikhNet's lists "as per SGPC Calendar": every sangrand of 2021–22,
    // and of 2026 but Maagh, which the list gives as 13 January and the
    // Vikrami calendar, with dekho-ji and punjabdata, as 14 January.
    let sangrands: [(i64, u8, (i64, u8, u8)); 23] = [
        (2078, 1, (2021, 4, 13)),
        (2078, 2, (2021, 5, 14)),
        (2078, 3, (2021, 6, 15)),
        (2078, 4, (2021, 7, 16)),
        (2078, 5, (2021, 8, 16)),
        (2078, 6, (2021, 9, 16)),
        (2078, 7, (2021, 10, 17)),
        (2078, 8, (2021, 11, 16)),
        (2078, 9, (2021, 12, 15)),
        (2078, 10, (2022, 1, 14)),
        (2078, 11, (2022, 2, 12)),
        (2082, 11, (2026, 2, 12)),
        (2082, 12, (2026, 3, 14)),
        (2083, 1, (2026, 4, 14)),
        (2083, 2, (2026, 5, 15)),
        (2083, 3, (2026, 6, 15)),
        (2083, 4, (2026, 7, 16)),
        (2083, 5, (2026, 8, 17)),
        (2083, 6, (2026, 9, 17)),
        (2083, 7, (2026, 10, 17)),
        (2083, 8, (2026, 11, 16)),
        (2083, 9, (2026, 12, 16)),
        (2082, 10, (2026, 1, 14)),
    ];
    for (year, month, (gy, gm, gd)) in sangrands {
        let first = VIKRAMI
            .to_fixed(HinduSolarDate {
                year,
                month,
                day: 1,
            })
            .expect("in range");
        assert_eq!(first, ymd(gy, gm, gd), "{year} month {month}");
    }
}

#[test]
fn the_jain_festivals_of_2024_are_counted_back_from_their_last_days() {
    // Saṃvatsarī on 7 September 2024, so Paryuṣaṇa from 31 August; Ananta
    // Caturdaśī on 17 September, so Daśa Lakṣaṇa from the 8th.
    expect(
        &JAIN,
        &[
            (2024, 8, 31, "Paryushana"),
            (2024, 9, 7, "Paryushana"),
            (2024, 9, 7, "Samvatsari"),
            (2024, 9, 8, "Das Lakshana"),
            (2024, 9, 17, "Das Lakshana"),
            (2024, 9, 17, "Anant Chaturdashi"),
            (2024, 4, 21, "Mahavir Jayanti"),
            (2024, 10, 31, "Diwali"),
        ],
    );
    let calendar = HolidayCalendar::for_year(&JAIN, None, 2024);
    let paryushana = (1..=366)
        .filter_map(|day| {
            let date = Rd(ymd(2024, 1, 1).0 + day - 1);
            calendar
                .on(date)
                .iter()
                .any(|holiday| holiday.name == "Paryushana")
                .then_some(date)
        })
        .count();
    assert_eq!(paryushana, 8);
    assert!(calendar.on(ymd(2024, 8, 30)).is_empty());
    assert!(calendar.on(ymd(2024, 9, 18)).is_empty());
    for rule in JAIN.rules {
        assert_eq!(rule.kind, Kind::Religious, "{}", rule.name);
        assert_eq!(rule.confidence, Confidence::Approximate, "{}", rule.name);
    }
}

#[test]
fn setsubun_is_the_eve_of_risshun_and_moves_as_the_source_says() {
    // 4 February in the leap years to 1984, 3 February from 1985 to 2020,
    // 2 February in the year after a leap year from 2021.
    expect(
        &SHINTO,
        &[
            (1984, 2, 4, "Setsubun"),
            (1985, 2, 3, "Setsubun"),
            (2020, 2, 3, "Setsubun"),
            (2021, 2, 2, "Setsubun"),
            (2024, 2, 3, "Setsubun"),
            (2025, 2, 2, "Setsubun"),
            (2026, 2, 3, "Setsubun"),
            (2026, 1, 1, "Hatsumōde"),
            (2026, 6, 30, "Nagoshi no Ōharae"),
            (2026, 11, 15, "Shichi-Go-San"),
            (2026, 12, 31, "Toshikoshi no Ōharae"),
        ],
    );
    for year in [1984, 2021, 2026] {
        let calendar = HolidayCalendar::for_year(&SHINTO, None, year);
        let setsubun = (1..=60)
            .filter(|day| {
                calendar
                    .on(Rd(ymd(year, 1, 1).0 + day - 1))
                    .iter()
                    .any(|holiday| holiday.name == "Setsubun")
            })
            .count();
        assert_eq!(setsubun, 1, "{year}");
    }
}

#[test]
fn the_imperial_rites_fall_on_the_sources_schedule() {
    // 春分の日 2026 is 20 March and 秋分の日 23 September.
    expect(
        &KYUCHU_SAISHI,
        &[
            (2026, 1, 1, "Shihōhai"),
            (2026, 1, 1, "Saitansai"),
            (2026, 1, 1, "Shunsai"),
            (2026, 1, 3, "Genshisai"),
            (2026, 1, 7, "Shōwa Tennō-sai"),
            (2026, 1, 11, "Shunsai"),
            (2026, 1, 21, "Shunsai"),
            (2026, 2, 11, "Sanden gohai"),
            (2026, 2, 17, "Kinensai"),
            (2026, 2, 23, "Tenchōsai"),
            (2026, 3, 20, "Shunki Kōreisai"),
            (2026, 3, 20, "Shunki Shindensai"),
            (2026, 4, 3, "Jinmu Tennō-sai"),
            (2026, 6, 30, "Ōharai"),
            (2026, 9, 23, "Shūki Kōreisai"),
            (2026, 10, 17, "Kannamesai"),
            (2026, 11, 23, "Niinamesai"),
            (2026, 12, 25, "Taishō Tennō reisai"),
            (2026, 12, 21, "Shunsai"),
            (2026, 12, 31, "Yoori"),
        ],
    );
    // 天長祭 on 23 February only from 2020; 三殿御拝 only from 1949.
    let year_2019 = HolidayCalendar::for_year(&KYUCHU_SAISHI, None, 2019);
    assert!(
        year_2019
            .on(ymd(2019, 2, 23))
            .iter()
            .all(|holiday| holiday.name != "Tenchōsai")
    );
    let year_1948 = HolidayCalendar::for_year(&KYUCHU_SAISHI, None, 1948);
    // 11 February 1948 still had its 旬祭, but no 三殿御拝.
    assert!(
        year_1948
            .on(ymd(1948, 2, 11))
            .iter()
            .all(|holiday| holiday.name == "Shunsai")
    );
    assert_eq!(
        KYUCHU_SAISHI
            .rules
            .iter()
            .filter(|rule| rule.name == "Shunsai")
            .count(),
        36
    );
    for set in [&SHINTO, &KYUCHU_SAISHI] {
        for rule in set.rules {
            assert_eq!(
                rule.confidence,
                Confidence::Exact,
                "{} {}",
                set.code,
                rule.name
            );
        }
    }
}

/// Assert that `name` is not on the given date in the given tradition.
fn expect_not(set: &RuleSet, days: &[(i64, u8, u8, &str)]) {
    for (year, month, day, name) in days {
        let calendar = HolidayCalendar::for_year(set, None, *year);
        assert!(
            calendar
                .on(ymd(*year, *month, *day))
                .iter()
                .all(|holiday| holiday.name != *name),
            "{} {year}-{month:02}-{day:02}: {name} should not be there",
            set.code
        );
    }
}

#[test]
fn taanit_esther_is_the_day_before_purim_or_the_thursday_before() {
    // Hebcal's dates of 2024–2028. Purim 2024 and 2028 fell on a Sunday,
    // so the fast went back to the Thursday.
    expect(
        &JEWISH,
        &[
            (2024, 3, 21, "Ta'anit Esther"),
            (2025, 3, 13, "Ta'anit Esther"),
            (2026, 3, 2, "Ta'anit Esther"),
            (2027, 3, 22, "Ta'anit Esther"),
            (2028, 3, 9, "Ta'anit Esther"),
        ],
    );
    expect_not(&JEWISH, &[(2024, 3, 23, "Ta'anit Esther")]);
}

#[test]
fn taanit_bechorot_is_the_eve_of_passover_or_the_thursday_before() {
    // Shulchan Arukh, Orach Chayim 470:1-2: the firstborn fast on the eve
    // of Passover, and when it is a Sabbath "some say" on the Thursday, as
    // the Rema says to keep it. 14 Nisan 5786 is Wednesday 1 April 2026;
    // 14 Nisan 5781 was Saturday 27 March 2021.
    expect(
        &JEWISH,
        &[
            (2026, 4, 1, "Ta'anit Bechorot"),
            (2021, 3, 25, "Ta'anit Bechorot"),
        ],
    );
    expect_not(&JEWISH, &[(2021, 3, 27, "Ta'anit Bechorot")]);
    expect(&JEWISH, &[(2021, 3, 28, "Passover")]);
}

#[test]
fn the_fasts_of_the_destruction_are_pushed_past_the_sabbath() {
    // Shulchan Arukh, Orach Chayim 550:3: "All four fasts, if they fall on
    // Shabbos, are pushed to after Shabbos". 3 Tishrei 5785 was Saturday
    // 5 October 2024, 17 Tammuz 5785 Saturday 12 July 2025 and 9 Av 5785
    // Saturday 2 August 2025; 9 Av 5782 was Saturday 6 August 2022, and
    // Reingold and Dershowitz's `tishah-be-av` gives the Sunday after.
    expect(
        &JEWISH,
        &[
            (2024, 10, 6, "Fast of Gedaliah"),
            (2025, 7, 13, "Seventeenth of Tammuz"),
            (2025, 8, 3, "Tisha B'Av"),
            (2022, 8, 7, "Tisha B'Av"),
            (2022, 7, 17, "Seventeenth of Tammuz"),
            // Not on a Sabbath, the day itself.
            (2023, 9, 18, "Fast of Gedaliah"),
            (2026, 7, 2, "Seventeenth of Tammuz"),
            (2026, 7, 23, "Tisha B'Av"),
        ],
    );
    expect_not(
        &JEWISH,
        &[
            (2024, 10, 5, "Fast of Gedaliah"),
            (2025, 7, 12, "Seventeenth of Tammuz"),
            (2025, 8, 2, "Tisha B'Av"),
        ],
    );
    // No minor fast is ever kept on a Sabbath, and the Tenth of Tevet, which
    // the law does not move, never falls on one in the fixed calendar.
    for year in 1900..=2100 {
        for holiday in HolidayCalendar::for_year(&JEWISH, None, year).all() {
            if [
                "Fast of Gedaliah",
                "Tenth of Tevet",
                "Ta'anit Esther",
                "Seventeenth of Tammuz",
                "Tisha B'Av",
            ]
            .contains(&holiday.name)
            {
                assert_ne!(
                    hc_calendar::Weekday::from_rd(holiday.date),
                    hc_calendar::Weekday::Saturday,
                    "{} {year}",
                    holiday.name
                );
            }
        }
    }
}

#[test]
fn shela_is_the_fifth_of_december_or_the_sixth_before_a_leap_year() {
    // Chabad.org: the night of 4 December, or 5 December in the year before
    // a civil leap year (2023, 2027 …), which begins the day named here.
    expect(
        &JEWISH,
        &[
            (
                2023,
                12,
                6,
                "Sh'ela (prayer for rain, outside the Land of Israel)",
            ),
            (
                2024,
                12,
                5,
                "Sh'ela (prayer for rain, outside the Land of Israel)",
            ),
            (
                2025,
                12,
                5,
                "Sh'ela (prayer for rain, outside the Land of Israel)",
            ),
            (
                2026,
                12,
                5,
                "Sh'ela (prayer for rain, outside the Land of Israel)",
            ),
            (
                2027,
                12,
                6,
                "Sh'ela (prayer for rain, outside the Land of Israel)",
            ),
        ],
    );
}

#[test]
fn the_prayer_book_ember_and_rogation_days() {
    // BCP 1662: the Wednesday, Friday and Saturday after Lent 1, Pentecost,
    // 14 September and 13 December; Rogation the three days before the
    // Ascension. 2025: Lent 1 on 9 March, the Ascension 29 May, Pentecost
    // 8 June, 14 September a Sunday, 13 December a Saturday.
    expect(
        &EMBER_BCP1662,
        &[
            (2025, 3, 12, "Ember Wednesday (Lent)"),
            (2025, 3, 14, "Ember Friday (Lent)"),
            (2025, 3, 15, "Ember Saturday (Lent)"),
            (2025, 5, 26, "Rogation Monday"),
            (2025, 5, 27, "Rogation Tuesday"),
            (2025, 5, 28, "Rogation Wednesday"),
            (2025, 6, 11, "Ember Wednesday (Whitsun)"),
            (2025, 6, 13, "Ember Friday (Whitsun)"),
            (2025, 6, 14, "Ember Saturday (Whitsun)"),
            (2025, 9, 17, "Ember Wednesday (September)"),
            (2025, 9, 19, "Ember Friday (September)"),
            (2025, 9, 20, "Ember Saturday (September)"),
            (2025, 12, 17, "Ember Wednesday (December)"),
            (2025, 12, 19, "Ember Friday (December)"),
            (2025, 12, 20, "Ember Saturday (December)"),
            // 14 September 2027 is a Tuesday: the 15th, 17th and 18th, as
            // Wikipedia's "Ember days" gives the rule.
            (2027, 9, 15, "Ember Wednesday (September)"),
            (2027, 9, 17, "Ember Friday (September)"),
            (2027, 9, 18, "Ember Saturday (September)"),
        ],
    );
    // A Wednesday 14 September is not its own Ember Day.
    expect(
        &EMBER_BCP1662,
        &[(2022, 9, 21, "Ember Wednesday (September)")],
    );
    expect_not(
        &EMBER_BCP1662,
        &[(2022, 9, 14, "Ember Wednesday (September)")],
    );
}

#[test]
fn the_common_worship_traditional_ember_weeks() {
    // The weeks before the Second Sunday of Lent, the Sundays nearest
    // 29 June and 29 September, and the Third Sunday of Advent. 2025: Lent 2
    // on 16 March, 29 June a Sunday, the Sunday nearest 29 September the
    // 28th, Advent 3 on 14 December.
    expect(
        &EMBER_COMMON_WORSHIP,
        &[
            (2025, 3, 12, "Ember Wednesday (Lent)"),
            (2025, 3, 15, "Ember Saturday (Lent)"),
            (2025, 5, 27, "Rogation Tuesday"),
            (2025, 6, 25, "Ember Wednesday (June)"),
            (2025, 6, 27, "Ember Friday (June)"),
            (2025, 6, 28, "Ember Saturday (June)"),
            (2025, 9, 24, "Ember Wednesday (September)"),
            (2025, 9, 26, "Ember Friday (September)"),
            (2025, 9, 27, "Ember Saturday (September)"),
            (2025, 12, 10, "Ember Wednesday (Advent)"),
            (2025, 12, 12, "Ember Friday (Advent)"),
            (2025, 12, 13, "Ember Saturday (Advent)"),
            // 29 June 2026 is a Monday, so the nearest Sunday is the 28th.
            (2026, 6, 24, "Ember Wednesday (June)"),
        ],
    );
    // The two churches part in three seasons out of four.
    expect_not(
        &EMBER_COMMON_WORSHIP,
        &[(2025, 12, 17, "Ember Wednesday (December)")],
    );
}

#[test]
fn the_common_worship_ember_days_the_church_and_a_diocese_printed() {
    // The Church of England's Common Worship: Daily Prayer marks "Ember
    // Day" on 24, 26 and 27 June and 23, 25 and 26 September 2026, and not
    // on the 16th, 18th or 19th, the week after 14 September
    // (`cofe-daily-prayer-2026`); the Diocese of London's calendar for
    // Advent 2024 on 11, 13 and 14 December 2024 (`london-kalendar-2024-25`).
    expect(
        &EMBER_COMMON_WORSHIP,
        &[
            (2026, 6, 24, "Ember Wednesday (June)"),
            (2026, 6, 26, "Ember Friday (June)"),
            (2026, 6, 27, "Ember Saturday (June)"),
            (2026, 9, 23, "Ember Wednesday (September)"),
            (2026, 9, 25, "Ember Friday (September)"),
            (2026, 9, 26, "Ember Saturday (September)"),
            (2024, 12, 11, "Ember Wednesday (Advent)"),
            (2024, 12, 13, "Ember Friday (Advent)"),
            (2024, 12, 14, "Ember Saturday (Advent)"),
        ],
    );
    for day in [16, 18, 19, 22, 24] {
        let calendar = HolidayCalendar::for_year(&EMBER_COMMON_WORSHIP, None, 2026);
        assert!(calendar.on(ymd(2026, 9, day)).is_empty(), "2026-09-{day}");
    }
    // The Prayer Book's week that year is the one Common Worship leaves.
    expect(
        &EMBER_BCP1662,
        &[
            (2026, 9, 16, "Ember Wednesday (September)"),
            (2026, 9, 18, "Ember Friday (September)"),
            (2026, 9, 19, "Ember Saturday (September)"),
        ],
    );
}

#[test]
fn the_prayer_book_september_week_is_one_week_after_holy_cross() {
    // A Thursday 14 September, as in 2023: read as one week, the
    // Wednesday after the 14th and the Friday and Saturday after it, the
    // 20th, 22nd and 23rd; read day by day, "the Friday and Saturday after
    // September 14" would be the 15th and 16th, which is not carried.
    expect(
        &EMBER_BCP1662,
        &[
            (2023, 9, 20, "Ember Wednesday (September)"),
            (2023, 9, 22, "Ember Friday (September)"),
            (2023, 9, 23, "Ember Saturday (September)"),
        ],
    );
    expect_not(
        &EMBER_BCP1662,
        &[
            (2023, 9, 15, "Ember Friday (September)"),
            (2023, 9, 16, "Ember Saturday (September)"),
        ],
    );
}

#[test]
fn the_greater_litanies_leave_easter_and_easter_monday() {
    // Code of Rubrics 1960, no. 80: 25 April, or the Tuesday after when
    // Easter Sunday (2038) or Easter Monday (2011) falls on it.
    expect(
        &ROGATION_ROMAN_1960,
        &[
            (2025, 4, 25, "Greater Litanies (Major Rogation)"),
            (2011, 4, 26, "Greater Litanies (Major Rogation)"),
            (2038, 4, 27, "Greater Litanies (Major Rogation)"),
            (2025, 5, 26, "Lesser Litanies (Rogation Monday)"),
            (2025, 5, 28, "Lesser Litanies (Rogation Wednesday)"),
        ],
    );
    expect_not(
        &ROGATION_ROMAN_1960,
        &[
            (2011, 4, 25, "Greater Litanies (Major Rogation)"),
            (2038, 4, 25, "Greater Litanies (Major Rogation)"),
        ],
    );
}

#[test]
fn the_samaritan_festivals_of_the_published_calendar() {
    // the-samaritans.net's festivals of autumn 2026, and the Institute's
    // Passover sacrifices of 2017–2020.
    expect(
        &SAMARITAN,
        &[
            (2026, 10, 11, "Festival of the Seventh Month"),
            (2026, 10, 20, "Day of Atonement"),
            (2026, 10, 25, "Festival of Sukkot (Tabernacles)"),
            (2026, 11, 1, "Shemini Atseret (Day of Assembly)"),
            (2017, 4, 10, "Passover sacrifice"),
            (2018, 4, 29, "Passover sacrifice"),
            (2019, 4, 18, "Passover sacrifice"),
            (2020, 5, 6, "Passover sacrifice"),
            (2019, 4, 19, "Feast of Unleavened Bread"),
            (2019, 4, 25, "Feast of Unleavened Bread"),
        ],
    );
    expect_not(&SAMARITAN, &[(2019, 4, 26, "Feast of Unleavened Bread")]);
    // Outside the calendar's years the table says it cannot answer.
    assert!(HolidayCalendar::for_year(&SAMARITAN, None, 2026).is_complete());
    assert!(!HolidayCalendar::for_year(&SAMARITAN, None, 2150).is_complete());
}

#[test]
fn the_mandaean_feasts_fall_where_drower_saw_them() {
    // Drower: Dehwa Hnina on 23 November 1932 and 1935, Panja from 5 April
    // in 1932–1935 and 4 April in 1936, the New Year on 8 August 1935.
    expect(
        &MANDAEAN,
        &[
            (1932, 11, 23, "Dehwa Hnina"),
            (1935, 11, 23, "Dehwa Hnina"),
            (1935, 11, 25, "Dehwa Hnina"),
            (1932, 4, 5, "Panja (Parwanaia)"),
            (1935, 4, 5, "Panja (Parwanaia)"),
            (1936, 4, 4, "Panja (Parwanaia)"),
            (1935, 8, 8, "Dehwa Rabba (New Year)"),
            (1935, 8, 7, "Kanshia uZahla (New Year's Eve)"),
        ],
    );
    expect_not(&MANDAEAN, &[(1935, 11, 26, "Dehwa Hnina")]);
    // Wikipedia's 2024 dates, as a check: Parwanaya 13–17 March, Dehwa
    // Daimana 17 May, Kanshi u-Zahli 15 July, Dehwa Rabba 16 July, Dehwa
    // Hanina 31 October, Ashoriya 13 December.
    expect(
        &MANDAEAN,
        &[
            (2024, 3, 13, "Panja (Parwanaia)"),
            (2024, 3, 17, "Panja (Parwanaia)"),
            (2024, 5, 17, "Dehwa Daimana"),
            (2024, 7, 15, "Kanshia uZahla (New Year's Eve)"),
            (2024, 7, 16, "Dehwa Rabba (New Year)"),
            (2024, 10, 31, "Dehwa Hnina"),
            (2024, 12, 13, "Ashuriyah"),
            // The day after Panja and the first of Taura are mbattal.
            (2024, 3, 18, "Mbattal day"),
            (2024, 10, 14, "Mbattal day"),
        ],
    );
}

#[test]
fn the_yazidi_feasts_are_eastern_dates() {
    // Serêsal on 19 April 2023, 17 April 2024 and 15 April 2026; the
    // Festival of the Assembly on Eastern 23–30 September, which in 2024
    // is 6–13 October; Bêlinde on Eastern 1 December, 14 December, after
    // the three days' fast.
    expect(
        &YAZIDI,
        &[
            (2023, 4, 19, "Serêsal (New Year)"),
            (2024, 4, 17, "Serêsal (New Year)"),
            (2026, 4, 15, "Serêsal (New Year)"),
            (2024, 10, 6, "Festival of the Assembly"),
            (2024, 10, 13, "Festival of the Assembly"),
            (2024, 12, 11, "Winter fast"),
            (2024, 12, 13, "Winter fast"),
            (2024, 12, 14, "Bêlinde"),
            (
                2024,
                6,
                23,
                "Chilleyê Havînan (Forty Days of Summer) begins",
            ),
        ],
    );
    expect_not(&YAZIDI, &[(2024, 10, 14, "Festival of the Assembly")]);
}

#[test]
fn the_yazidi_mobile_feasts_are_the_islamic_days_kreyenbroek_names() {
    // Kreyenbroek, pp. 157–158: Sheva Berat on 15 Sha'ban, the day of
    // Mid-Sha'ban; the Feast of Ramadan two days before 'Ayd al-Fitr; the
    // Feast of 'Erefat on 9 Dhu 'l-Hijja, Arafah. All on the tabular
    // calendar, and approximate.
    let on = |set: &RuleSet, year: i64, name: &str| -> Vec<Rd> {
        HolidayCalendar::for_year(set, None, year)
            .all()
            .iter()
            .filter(|holiday| holiday.name == name)
            .map(|holiday| holiday.date)
            .collect()
    };
    for year in 2020..=2040 {
        assert_eq!(
            on(&YAZIDI, year, "Sheva Berat"),
            on(&ISLAMIC, year, "Mid-Sha'ban"),
            "{year}"
        );
        assert_eq!(
            on(&YAZIDI, year, "Feast of ‘Erefat"),
            on(&ISLAMIC, year, "Day of Arafah"),
            "{year}"
        );
        let fitr: Vec<Rd> = on(&ISLAMIC, year, "Eid al-Fitr")
            .into_iter()
            .map(|day| Rd(day.0 - 2))
            .filter(|day| gregorian::year_from_fixed(*day).is_ok_and(|y| y == year))
            .collect();
        assert_eq!(
            on(&YAZIDI, year, "Feast of Ramadan (Sheykh Khal Shemsan)"),
            fitr,
            "{year}"
        );
    }
    for holiday in HolidayCalendar::for_year(&YAZIDI, None, 2026).all() {
        let hijri = [
            "Sheva Berat",
            "Feast of Ramadan (Sheykh Khal Shemsan)",
            "Feast of ‘Erefat",
        ]
        .contains(&holiday.name);
        assert_eq!(
            holiday.confidence == Confidence::Approximate,
            hijri,
            "{}",
            holiday.name
        );
    }
}

#[test]
fn the_armenian_year_in_etchmiadzin_and_in_jerusalem() {
    // Vardavar 98 days after Easter: 7 July 2024, 27 July 2025, 12 July
    // 2026 (Wikipedia, "Vardavar"); Theophany on 6 January; the Sundays
    // nearest 15 August and 14 September; Great Lent from the seventh
    // Monday before Easter, 16 February 2026.
    expect(
        &CHRISTIAN_ARMENIAN,
        &[
            (2024, 7, 7, "Transfiguration (Vardavar)"),
            (2025, 7, 27, "Transfiguration (Vardavar)"),
            (2026, 7, 12, "Transfiguration (Vardavar)"),
            (2026, 1, 6, "Theophany (Nativity and Baptism of Christ)"),
            (2026, 2, 16, "Great Lent begins"),
            (2026, 4, 5, "Easter"),
            (2026, 8, 16, "Assumption of the Mother of God"),
            (2026, 9, 13, "Exaltation of the Holy Cross"),
            (2026, 10, 4, "Holy Cross of Varak"),
            (2026, 11, 1, "Discovery of the Holy Cross"),
            (2026, 11, 16, "Advent (Hisnag) begins"),
            (2026, 5, 10, "Apparition of the Cross"),
            (2026, 2, 14, "Presentation of the Lord to the Temple"),
            (2026, 4, 7, "Annunciation"),
        ],
    );
    // Jerusalem keeps the Julian calendar and computus: Theophany on
    // 19 January, Easter 2026 on 12 April and Vardavar 98 days later.
    expect(
        &CHRISTIAN_ARMENIAN_JERUSALEM,
        &[
            (2026, 1, 19, "Theophany (Nativity and Baptism of Christ)"),
            (2026, 4, 12, "Easter"),
            (2026, 7, 19, "Transfiguration (Vardavar)"),
        ],
    );
    use hc_calendar::Weekday;
    for set in [&CHRISTIAN_ARMENIAN, &CHRISTIAN_ARMENIAN_JERUSALEM] {
        for year in 2000..=2040 {
            let calendar = HolidayCalendar::for_year(set, None, year);
            for holiday in calendar.all() {
                if holiday.name.starts_with("Assumption")
                    || holiday.name.starts_with("Exaltation")
                    || holiday.name.starts_with("Apparition")
                {
                    assert_eq!(
                        Weekday::from_rd(holiday.date),
                        Weekday::Sunday,
                        "{} {year} {}",
                        set.code,
                        holiday.name
                    );
                }
            }
        }
    }
}

/// The date `name` falls on in `year` in `set`, when it falls once.
fn only_date(set: &RuleSet, year: i64, name: &str) -> Option<Rd> {
    only_in(&HolidayCalendar::for_year(set, None, year), set, year, name)
}

/// The date `name` falls on in `calendar`, the evaluation of `set` over
/// `year`, when it falls once.
fn only_in(calendar: &HolidayCalendar, set: &RuleSet, year: i64, name: &str) -> Option<Rd> {
    let mut dates = calendar
        .all()
        .iter()
        .filter(|holiday| holiday.name == name)
        .map(|holiday| holiday.date);
    let first = dates.next();
    assert!(dates.next().is_none(), "{} {year}: {name} twice", set.code);
    first
}

/// How many times sparser the year-by-year sweeps of the lunisolar tables
/// in this file are in a debug build, which the coverage job runs instrumented; a
/// release build, which CI's release-mode job runs, checks every year. The
/// dated examples are checked in full in both.
const SAMPLED: usize = if cfg!(debug_assertions) { 3 } else { 1 };

/// The years of those sweeps: 1950 to 2100, every [`SAMPLED`]th. Every
/// third year still holds years with and without a leap month.
fn swept_years() -> impl Iterator<Item = i64> {
    (1950..=2100).step_by(SAMPLED)
}

/// The Chinese folk table's evaluation of each of the [`swept_years`].
///
/// Four tests in this file compare a lunisolar day against it in every one of
/// those years, and each evaluation is dozens of lunisolar conversions and
/// solar terms: evaluated once for all four rather than once a name a test.
fn chinese_folk(year: i64) -> &'static HolidayCalendar<'static> {
    static YEARS: OnceLock<Vec<HolidayCalendar<'static>>> = OnceLock::new();
    let years = YEARS.get_or_init(|| {
        swept_years()
            .map(|year| HolidayCalendar::for_year(&CHINESE_FOLK, None, year))
            .collect()
    });
    let Ok(index) = usize::try_from(year - 1950) else {
        panic!("{year} is before 1950");
    };
    assert_eq!(index % SAMPLED, 0, "{year} is not swept");
    &years[index / SAMPLED]
}

/// [`only_in`] the Chinese folk table's evaluation of `year`.
fn only_folk(year: i64, name: &str) -> Option<Rd> {
    only_in(chinese_folk(year), &CHINESE_FOLK, year, name)
}

#[test]
fn the_chinese_folk_additions_fall_on_their_days() {
    // 2025: 春節 29 January, 清明 4 April; 2026: 春節 17 February, 清明 5
    // April.
    expect(
        &CHINESE_FOLK,
        &[
            (2025, 2, 4, "Human Day"),
            (2025, 3, 31, "Shangsi Festival"),
            (2025, 4, 3, "Cold Food Festival"),
            (2026, 2, 23, "Human Day"),
            (2026, 4, 19, "Shangsi Festival"),
            (2026, 4, 4, "Cold Food Festival"),
        ],
    );
    // 寒食 is the day before 清明 every year, the hc-seasons convention.
    for year in swept_years() {
        let cold_food = only_folk(year, "Cold Food Festival");
        assert_eq!(
            cold_food,
            Some(ColdFoodConvention::EveOfQingming.day(year)),
            "{year}"
        );
        let qingming = only_folk(year, "Qingming Festival");
        assert_eq!(qingming.map(|day| day.0 - 1), cold_food.map(|day| day.0));
    }
}

#[test]
fn the_cold_food_festival_before_1645_is_105_days_after_the_solstice() {
    // "冬至後一百五日" until the 時憲曆 of 1645, the day before 清明 from
    // it (Wikipedia zh, "寒食节"); one entry a year either side of the
    // change, and the older one marked approximate.
    for year in 1500..=1700 {
        let calendar = HolidayCalendar::for_year(&CHINESE_FOLK, None, year);
        let cold_food: Vec<_> = calendar
            .all()
            .iter()
            .filter(|holiday| holiday.name == "Cold Food Festival")
            .collect();
        assert_eq!(cold_food.len(), 1, "{year}");
        let (convention, confidence) = if year < 1645 {
            (ColdFoodConvention::SolsticePlus105, Confidence::Approximate)
        } else {
            (ColdFoodConvention::EveOfQingming, Confidence::Exact)
        };
        assert_eq!(cold_food[0].date, convention.day(year), "{year}");
        assert_eq!(cold_food[0].confidence, confidence, "{year}");
        // Before 1645 清明 is the equal term of the calendars then in use,
        // approximate as 寒食 is, and 寒食 falls "清明前一或二日", a day or
        // two before it; from 1645 清明 is the true term and exact.
        let qingming_entry: Vec<_> = calendar
            .all()
            .iter()
            .filter(|holiday| holiday.name == "Qingming Festival")
            .collect();
        assert_eq!(qingming_entry.len(), 1, "{year}");
        assert_eq!(qingming_entry[0].confidence, confidence, "{year}");
        let before = qingming_entry[0].date.0 - cold_food[0].date.0;
        if year < 1645 {
            assert!((1..=2).contains(&before), "{year}: {before} days");
        } else {
            assert_eq!(before, 1, "{year}");
        }
    }
    // The true terms, 定氣, would have put the solstice count on 清明 or
    // after it — which is why the 時憲曆 moved the day: 1640's true 清明
    // was 4 April and its 寒食 5 April.
    let true_qingming = Rule::SolarTerm {
        term: SolarTerm::from_degrees(15).unwrap_or(SolarTerm::SPRING_EQUINOX),
        meridian: Meridian::CHINA,
    };
    assert_eq!(
        true_qingming.days_in_year(1640).as_slice(),
        &[ymd(1640, 4, 4)]
    );
    assert_eq!(
        ColdFoodConvention::SolsticePlus105.day(1640),
        ymd(1640, 4, 5)
    );
}

#[test]
fn the_little_new_year_is_one_table_per_region() {
    // 新华社, 10 February 2026: "今天，腊月二十三是北方小年，明天，腊月二十四是
    // 南方小年". 除夕 was 16 February 2026 and 元宵 3 March.
    expect(
        &CHINESE_XIAONIAN_NORTH,
        &[(2026, 2, 10, "Little New Year (north)")],
    );
    expect(
        &CHINESE_XIAONIAN_SOUTH,
        &[(2026, 2, 11, "Little New Year (south)")],
    );
    expect(
        &CHINESE_XIAONIAN_JIANGNAN,
        &[(2026, 2, 15, "Little New Year (Jiangnan, Fujian and Taiwan)")],
    );
    expect(
        &CHINESE_XIAONIAN_NANJING,
        &[(2026, 3, 3, "Little New Year (Nanjing)")],
    );
    expect(
        &CHINESE_XIAONIAN_SOUTHWEST,
        &[(2026, 2, 16, "Little New Year (south-west)")],
    );
    // 2025: 腊月廿三 of 甲辰 was 22 January, 除夕 28 January.
    expect(
        &CHINESE_XIAONIAN_NORTH,
        &[(2025, 1, 22, "Little New Year (north)")],
    );
    expect(
        &CHINESE_XIAONIAN_SOUTHWEST,
        &[(2025, 1, 28, "Little New Year (south-west)")],
    );
    // The south's is the day after the north's, and the south-west's the
    // folk table's 除夕.
    for year in swept_years() {
        let north = only_date(&CHINESE_XIAONIAN_NORTH, year, "Little New Year (north)");
        let south = only_date(&CHINESE_XIAONIAN_SOUTH, year, "Little New Year (south)");
        assert_eq!(north.map(|day| day.0 + 1), south.map(|day| day.0), "{year}");
        assert_eq!(
            only_date(
                &CHINESE_XIAONIAN_SOUTHWEST,
                year,
                "Little New Year (south-west)"
            ),
            only_folk(year, "Chinese New Year's Eve"),
            "{year}"
        );
    }
}

#[test]
fn the_taoist_days_fall_on_their_lunar_dates() {
    // TVBS, 18 April 2025: 媽祖生日 on 農曆3月23日, 國曆4月20日.
    expect(
        &TAOIST,
        &[
            (2025, 4, 20, "Birthday of Mazu"),
            (
                2025,
                2,
                12,
                "Upper Yuan Festival (birthday of the Official of Heaven)",
            ),
            (
                2025,
                9,
                6,
                "Middle Yuan Festival (birthday of the Official of Earth)",
            ),
            (2025, 10, 29, "Ascension of Mazu"),
            (
                2025,
                12,
                4,
                "Lower Yuan Festival (birthday of the Official of Water)",
            ),
        ],
    );
    // 上元 and 中元 are the folk table's 元宵 and 中元.
    for year in swept_years() {
        assert_eq!(
            only_date(
                &TAOIST,
                year,
                "Upper Yuan Festival (birthday of the Official of Heaven)"
            ),
            only_folk(year, "Lantern Festival"),
            "{year}"
        );
    }
}

#[test]
fn the_korean_folk_days_are_where_the_korean_almanac_puts_them() {
    // KASI's 월력요항 for 2024, 2025 and 2026: 한식 5 April, 5 April and
    // 6 April; 단오 10 June, 31 May and 19 June; 칠석 10 August, 29 August
    // and 19 August.
    expect(
        &KOREAN_FOLK,
        &[
            (2024, 4, 5, "Hansik"),
            (2024, 6, 10, "Dano"),
            (2024, 8, 10, "Chilseok"),
            (2025, 4, 5, "Hansik"),
            (2025, 5, 31, "Dano"),
            (2025, 8, 29, "Chilseok"),
            (2026, 4, 6, "Hansik"),
            (2026, 6, 19, "Dano"),
            (2026, 8, 19, "Chilseok"),
            // 설날 2025 was 29 January, so 섣달그믐 the 28th and 대보름
            // 12 February.
            (2025, 1, 28, "Seotdal Geumeum"),
            (2025, 2, 12, "Jeongwol Daeboreum"),
        ],
    );
    let calendar = HolidayCalendar::for_year(&KOREAN_FOLK, None, 2025);
    assert_eq!(calendar.all().len(), 11);
    for holiday in calendar.all() {
        assert!(!holiday.kind.is_day_off(), "{}", holiday.name);
        assert_eq!(holiday.confidence, Confidence::Exact, "{}", holiday.name);
    }
    // 한식 is hc-seasons' convention of the same name, every year.
    for year in swept_years() {
        assert_eq!(
            only_date(&KOREAN_FOLK, year, "Hansik"),
            Some(ColdFoodConvention::Hansik.day(year)),
            "{year}"
        );
    }
}

#[test]
fn the_vietnamese_folk_days_fall_on_their_lunar_dates() {
    // Vietnam+, 15 January 2025: Ông Táo "vào thứ Tư, ngày 22/1/2025";
    // VietNamNet, 28 September 2025: Trung Thu "thứ Hai, ngày 6/10".
    expect(
        &VIETNAMESE_FOLK,
        &[
            (2025, 1, 22, "Kitchen Gods' Day"),
            (2025, 10, 6, "Mid-Autumn Festival"),
            (2025, 3, 31, "Cold Food Festival"),
            (2025, 5, 31, "Double Fifth Festival"),
            (2025, 9, 6, "Vu Lan (Ghost Festival)"),
        ],
    );
    let calendar = HolidayCalendar::for_year(&VIETNAMESE_FOLK, None, 2025);
    for holiday in calendar.all() {
        assert!(!holiday.kind.is_day_off(), "{}", holiday.name);
    }
}

#[test]
fn the_five_sekku_are_on_their_gregorian_dates_from_1873() {
    expect(
        &GOSEKKU,
        &[
            (2026, 1, 7, "Jinjitsu (Festival of Seven Herbs)"),
            (2026, 3, 3, "Joshi (Peach Festival)"),
            (2026, 5, 5, "Tango (Iris Festival)"),
            (2026, 7, 7, "Tanabata (Bamboo Festival)"),
            (2026, 9, 9, "Choyo (Chrysanthemum Festival)"),
            (1873, 1, 7, "Jinjitsu (Festival of Seven Herbs)"),
        ],
    );
    // Before 1873 they were lunar dates, which this table does not carry.
    assert!(
        HolidayCalendar::for_year(&GOSEKKU, None, 1872)
            .all()
            .is_empty()
    );
}

#[test]
fn the_tibetan_duchen_are_the_tibetan_nuns_projects_2024_dates() {
    expect(
        &BUDDHIST_TIBETAN,
        &[
            (2024, 2, 10, "Losar"),
            (2024, 5, 23, "Saga Dawa Düchen"),
            (2024, 6, 22, "Universal Prayer Day"),
            (2024, 11, 22, "Lhabab Düchen"),
        ],
    );
    // The list's Chökhor Düchen of 9 July 2024 is the first of two fourth
    // days of the Phugpa leap month 6; the regular month's fourth is 8
    // August. Neither the month nor the day is settled, so 2024 reports it
    // as a gap, and gives neither date.
    let calendar = HolidayCalendar::for_year(&BUDDHIST_TIBETAN, None, 2024);
    let listed = [
        "Losar",
        "Saga Dawa Düchen",
        "Universal Prayer Day",
        "Chökhor Düchen",
        "Lhabab Düchen",
    ];
    assert_eq!(
        calendar
            .all()
            .iter()
            .filter(|holiday| listed.contains(&holiday.name))
            .count(),
        4
    );
    assert!(
        calendar
            .gaps()
            .iter()
            .any(|gap| gap.name == "Chökhor Düchen" && gap.year == 2024)
    );
    for day in [ymd(2024, 7, 9), ymd(2024, 8, 8)] {
        assert!(
            !calendar
                .on(day)
                .iter()
                .any(|holiday| holiday.name == "Chökhor Düchen")
        );
    }
    // In 2025, a year with no leap month 6, it is dated.
    assert!(
        HolidayCalendar::for_year(&BUDDHIST_TIBETAN, None, 2025)
            .all()
            .iter()
            .any(|holiday| holiday.name == "Chökhor Düchen")
    );
}

#[test]
fn a_skipped_or_repeated_tibetan_day_is_a_gap_and_not_a_guess() {
    // Over half a century some of the five fall on a number the calendar
    // skips or repeats; each such year reports that day as a gap and gives
    // no date for it, and every other year gives exactly one.
    let names = [
        "Losar",
        "Saga Dawa Düchen",
        "Universal Prayer Day",
        "Chökhor Düchen",
        "Lhabab Düchen",
    ];
    let mut gaps = 0;
    for year in 2000..=2050 {
        let calendar = HolidayCalendar::for_year(&BUDDHIST_TIBETAN, None, year);
        for name in names {
            let dated = calendar
                .all()
                .iter()
                .filter(|holiday| holiday.name == name)
                .count();
            let gap = calendar.gaps().iter().any(|gap| gap.name == name);
            if gap {
                gaps += 1;
                assert_eq!(dated, 0, "{year} {name}: a gap and a date");
            } else {
                assert_eq!(dated, 1, "{year} {name}");
            }
        }
    }
    assert!(gaps > 0, "no skipped or repeated day in fifty years");
}

#[test]
fn the_festivals_henning_marks_are_on_the_phugpa_days() {
    // Henning's computed Phugpa almanacs for 2024 to 2026 (kalacakra.org,
    // tdata/pl_*.txt): the Revelation of the Kalacakra Tantra on 3/15, the
    // Birth of the Buddha on 4/7, the entry into the womb on 6/15, and the
    // Demonstration of Miracles "From 1st to 15th" from Losar.
    expect(
        &BUDDHIST_TIBETAN,
        &[
            (2024, 4, 23, "Revelation of the Kalacakra Tantra"),
            (2024, 5, 14, "Birth of the Buddha"),
            (2025, 2, 28, "Demonstration of Miracles"),
            (2025, 5, 12, "Revelation of the Kalacakra Tantra"),
            (2025, 6, 2, "Birth of the Buddha"),
            (2025, 8, 9, "The Buddha's entry into the womb of his mother"),
            (2026, 2, 18, "Demonstration of Miracles"),
            (2026, 5, 1, "Revelation of the Kalacakra Tantra"),
            (2026, 5, 23, "Birth of the Buddha"),
            (
                2026,
                7,
                29,
                "The Buddha's entry into the womb of his mother",
            ),
        ],
    );
    // Fifteen days of miracles, from Losar, in a year that skips or repeats
    // none of their numbers.
    let miracles: Vec<Rd> = HolidayCalendar::for_year(&BUDDHIST_TIBETAN, None, 2026)
        .all()
        .iter()
        .filter(|holiday| holiday.name == "Demonstration of Miracles")
        .map(|holiday| holiday.date)
        .collect();
    let gaps = HolidayCalendar::for_year(&BUDDHIST_TIBETAN, None, 2026)
        .gaps()
        .iter()
        .filter(|gap| gap.name == "Demonstration of Miracles")
        .count();
    assert_eq!(miracles.len() + gaps, 15);
    assert_eq!(miracles[0], ymd(2026, 2, 18));
}

#[test]
fn berzin_s_rule_keeps_a_skipped_day_on_the_day_before_and_a_repeated_one_on_the_first() {
    // Every year that is a gap in `buddhist-tibetan` for a skipped or
    // repeated day is answered here, and a year that doubles the month, as
    // 2024 doubles the sixth, is still a gap. Where the number is not skipped or repeated, the two agree.
    let mut answered = 0;
    for year in 2000..=2050 {
        let unsettled = HolidayCalendar::for_year(&BUDDHIST_TIBETAN, None, year);
        let berzin = HolidayCalendar::for_year(&traditions::BUDDHIST_TIBETAN_BERZIN, None, year);
        for holiday in unsettled.all() {
            assert!(
                berzin
                    .all()
                    .iter()
                    .any(|other| other.name == holiday.name && other.date == holiday.date),
                "{year} {}",
                holiday.name
            );
        }
        for gap in unsettled.gaps() {
            let month = match gap.name {
                "Revelation of the Kalacakra Tantra" => 3,
                "Birth of the Buddha" | "Saga Dawa Düchen" => 4,
                "Universal Prayer Day" => 5,
                "Chökhor Düchen" | "The Buddha's entry into the womb of his mother" => 6,
                "Lhabab Düchen" => 9,
                _ => 0,
            };
            let doubled_month = [year - 1, year].into_iter().any(|tibetan_year| {
                hc_calendars_lunar::tibetan::TIBETAN.leap_month_of(tibetan_year) == Some(month)
            });
            let still = berzin.gaps().iter().any(|other| other.name == gap.name);
            assert!(!still || doubled_month, "{year} {}", gap.name);
            if !still {
                answered += 1;
            }
        }
    }
    assert!(answered > 0);
    // 1990 skips the 7th of month 4: Berzin's rule keeps the Birth of the
    // Buddha on the 6th, 30 May, where `buddhist-tibetan` reports a gap.
    expect(
        &traditions::BUDDHIST_TIBETAN_BERZIN,
        &[(1990, 5, 30, "Birth of the Buddha")],
    );
    assert!(
        HolidayCalendar::for_year(&BUDDHIST_TIBETAN, None, 1990)
            .gaps()
            .iter()
            .any(|gap| gap.name == "Birth of the Buddha")
    );
    // 2024's doubled sixth month is a gap by either rule.
    assert!(
        HolidayCalendar::for_year(&traditions::BUDDHIST_TIBETAN_BERZIN, None, 2024)
            .gaps()
            .iter()
            .any(|gap| gap.name == "Chökhor Düchen")
    );
}

#[test]
fn henning_s_almanac_marks_both_months_and_no_skipped_day() {
    let henning = &traditions::BUDDHIST_TIBETAN_HENNING;
    // His almanac for 2024 marks the leap month 6 and the regular one; the
    // fourth of the leap month is repeated, 9 and 10 July, and he marks the
    // second.
    expect(
        henning,
        &[
            (2024, 2, 10, "Demonstration of Miracles"),
            (2024, 4, 23, "Revelation of the Kalacakra Tantra"),
            (2024, 5, 14, "Birth of the Buddha"),
            (2024, 5, 23, "Enlightenment and Parinirvana of the Buddha"),
            (2024, 7, 10, "Turning of the Wheel of the Dharma"),
            (
                2024,
                7,
                21,
                "The Buddha's entry into the womb of his mother",
            ),
            (2024, 8, 8, "Turning of the Wheel of the Dharma"),
            (
                2024,
                8,
                19,
                "The Buddha's entry into the womb of his mother",
            ),
            (
                2024,
                11,
                22,
                "Descent of the Buddha from the realm of the gods",
            ),
            (2025, 7, 28, "Turning of the Wheel of the Dharma"),
            (
                2025,
                11,
                11,
                "Descent of the Buddha from the realm of the gods",
            ),
            (2026, 5, 31, "Enlightenment and Parinirvana of the Buddha"),
            (
                2026,
                11,
                1,
                "Descent of the Buddha from the realm of the gods",
            ),
            (1990, 5, 9, "Revelation of the Kalacakra Tantra"),
            (1990, 6, 8, "Enlightenment and Parinirvana of the Buddha"),
            (1990, 7, 25, "Turning of the Wheel of the Dharma"),
            (1990, 8, 6, "The Buddha's entry into the womb of his mother"),
            (
                1990,
                11,
                9,
                "Descent of the Buddha from the realm of the gods",
            ),
        ],
    );
    expect_not(
        henning,
        &[(2024, 7, 9, "Turning of the Wheel of the Dharma")],
    );
    // 1990 skips the 7th of month 4, and the almanac marks no Birth of the
    // Buddha, which is an answer, not a gap.
    let calendar = HolidayCalendar::for_year(henning, None, 1990);
    assert!(
        !calendar
            .all()
            .iter()
            .any(|holiday| holiday.name == "Birth of the Buddha")
    );
    assert!(calendar.gaps().is_empty(), "{:?}", calendar.gaps());
    // Nothing is left unanswered in the almanacs' years.
    for year in 1960..=2045 {
        assert!(
            HolidayCalendar::for_year(henning, None, year)
                .gaps()
                .is_empty(),
            "{year}"
        );
    }
}

#[test]
fn the_thai_uposatha_days_are_the_published_wan_phra_of_2025() {
    // Thai PBS, "ปฏิทินวันพระ 2568": January, a month of 30 days, ends on
    // แรม 15 ค่ำ on the 28th; February's month of 29 on แรม 14 ค่ำ on the
    // 26th; the month 7 of June, 30 days in an adhikavāra year, on แรม 15
    // ค่ำ on the 25th; and August's on แรม 14 ค่ำ on the 23rd.
    let published: &[(u8, &[u8])] = &[
        (1, &[6, 13, 21, 28]),
        (2, &[5, 12, 20, 26]),
        (4, &[5, 12, 20, 26]),
        (5, &[4, 11, 19, 26]),
        (6, &[3, 10, 18, 25]),
        (7, &[3, 10, 18, 25]),
        (8, &[2, 9, 17, 23, 31]),
    ];
    let calendar = HolidayCalendar::for_year(&BUDDHIST_UPOSATHA_THAI, None, 2025);
    assert!(calendar.is_complete());
    for (month, days) in published {
        let found: Vec<Rd> = calendar
            .all()
            .iter()
            .map(|holiday| holiday.date)
            .filter(|date| gregorian::from_fixed(*date).is_ok_and(|(_, m, _)| m == *month))
            .collect();
        let expected: Vec<Rd> = days.iter().map(|day| ymd(2025, *month, *day)).collect();
        assert_eq!(found, expected, "2025-{month:02}");
    }
    expect(
        &BUDDHIST_UPOSATHA_THAI,
        &[
            (2025, 2, 26, "Uposatha (new moon)"),
            (2025, 1, 28, "Uposatha (new moon)"),
            (2025, 5, 11, "Uposatha (full moon)"),
        ],
    );
    // Every full moon is the Buddhist table's where they share one.
    for (month, day) in [(2, 12), (5, 11), (7, 10)] {
        assert!(
            calendar
                .on(ymd(2025, month, day))
                .iter()
                .any(|holiday| holiday.name == "Uposatha (full moon)")
        );
    }
    // Outside the published years, gaps.
    let outside = HolidayCalendar::for_year(&BUDDHIST_UPOSATHA_THAI, None, 2030);
    assert!(outside.all().is_empty());
    assert!(!outside.is_complete());
}

#[test]
fn plough_monday_follows_twelfth_day() {
    expect(
        &PLOUGH_DAYS,
        &[
            // 6 January 2025 was a Monday: Plough Monday the 13th.
            (2025, 1, 13, "Plough Monday"),
            (2025, 1, 12, "Plough Sunday"),
            (2025, 1, 7, "Distaff Day"),
            // 6 January 2026 was a Tuesday.
            (2026, 1, 12, "Plough Monday"),
            (2026, 1, 11, "Plough Sunday"),
            // 6 January 2027 is a Wednesday: "In 2027 Plough Monday falls on
            // Monday January 11th" (calendarcustoms.com).
            (2027, 1, 11, "Plough Monday"),
            // 6 January 2030 is a Sunday: Plough Sunday is the 13th, after
            // Plough Monday on the 7th, which is also Distaff Day.
            (2030, 1, 13, "Plough Sunday"),
            (2030, 1, 7, "Plough Monday"),
            (2030, 1, 7, "Distaff Day"),
        ],
    );
    for year in 1800..=2100 {
        let monday = only_date(&PLOUGH_DAYS, year, "Plough Monday").expect("every year");
        let (_, month, day) = gregorian::from_fixed(monday).expect("in range");
        assert_eq!(month, 1);
        assert!((7..=13).contains(&day), "{year}");
    }
}

#[test]
fn chaharshanbe_suri_is_the_eve_of_the_last_wednesday_of_the_year() {
    expect(
        &CHAHARSHANBE_SURI,
        &[
            // Nowruz 1403 was Wednesday 20 March 2024: the year's last
            // Wednesday was the 13th, its eve the 12th.
            (2024, 3, 12, "Chaharshanbe Suri"),
            (2025, 3, 18, "Chaharshanbe Suri"),
            (2026, 3, 17, "Chaharshanbe Suri"),
            (2027, 3, 16, "Chaharshanbe Suri"),
        ],
    );
    for year in 1950..=2100 {
        let day = only_date(&CHAHARSHANBE_SURI, year, "Chaharshanbe Suri").expect("every year");
        assert_eq!(
            hc_calendar::Weekday::from_rd(day),
            hc_calendar::Weekday::Tuesday
        );
        let (_, month, date) = gregorian::from_fixed(day).expect("in range");
        assert_eq!(month, 3, "{year}");
        assert!((12..=20).contains(&date), "{year}: {date}");
    }
}

#[test]
fn the_shia_days_fall_on_their_tabular_dates() {
    expect(
        &ISLAMIC,
        &[
            // Ghadir on 18 Dhu al-Hijjah 1445, 25 June 2024; Tasu'a, Ashura
            // and Arba'een of 1446.
            (2024, 6, 25, "Eid al-Ghadir"),
            (2024, 7, 16, "Tasu'a"),
            (2024, 7, 17, "Ashura"),
            (2024, 8, 26, "Arba'een"),
        ],
    );
    // Wikipedia's "Arba'in" dates it 14 August 2025, 3 August 2026 and
    // 24 July 2027, which are exactly Umm al-Qurā's 20 Safar of 1447, 1448
    // and 1449.
    for (hijri_year, published) in [
        (1447, (2025, 8, 14)),
        (1448, (2026, 8, 3)),
        (1449, (2027, 7, 24)),
    ] {
        let (y, m, d) = published;
        assert_eq!(
            hc_calendars_lunar::islamic_umalqura::to_fixed(hijri_year, 2, 20),
            Ok(ymd(y, m, d)),
            "{hijri_year}"
        );
    }
    // The consistency check of the tabular table against them: a day later
    // in 2025 and 2027 and two days later in 2026. The tabular year opens a
    // day after Umm al-Qurā's in 1447 and 1448 and on the same day in 1449,
    // and its Muharram always has 30 days where Umm al-Qurā's of 1448 and
    // 1449 has 29: 1 + 0, 1 + 1 and 0 + 1.
    for (hijri_year, (year, month, day), later) in [
        (1447, (2025, 8, 15), 1),
        (1448, (2026, 8, 5), 2),
        (1449, (2027, 7, 25), 1),
    ] {
        let found = only_date(&ISLAMIC, year, "Arba'een").expect("once a year");
        assert_eq!(found, ymd(year, month, day));
        let umm_al_qura = hc_calendars_lunar::islamic_umalqura::to_fixed(hijri_year, 2, 20)
            .expect("in the table");
        assert_eq!(found.0 - umm_al_qura.0, later, "{year}");
        let tabular_new_year = only_date(&ISLAMIC, year, "Islamic New Year").expect("once a year");
        let umm_al_qura_new_year =
            hc_calendars_lunar::islamic_umalqura::to_fixed(hijri_year, 1, 1).expect("in the table");
        let opens_later = tabular_new_year.0 - umm_al_qura_new_year.0;
        let muharram = hc_calendars_lunar::islamic_umalqura::days_in_month(hijri_year, 1)
            .expect("in the table");
        assert_eq!(
            (opens_later, muharram),
            match hijri_year {
                1447 => (1, 30),
                1448 => (1, 29),
                _ => (0, 29),
            },
            "{hijri_year}"
        );
        assert_eq!(
            opens_later + 30 - i64::from(muharram),
            later,
            "{hijri_year}"
        );
    }
    for year in 2000..=2050 {
        let calendar = HolidayCalendar::for_year(&ISLAMIC, None, year);
        for holiday in calendar.all() {
            if holiday.name == "Tasu'a" {
                assert!(
                    calendar
                        .on(Rd(holiday.date.0 + 1))
                        .iter()
                        .any(|next| next.name == "Ashura")
                        || !calendar.covers(Rd(holiday.date.0 + 1)),
                    "{year}: Tasu'a is the eve of Ashura"
                );
            }
        }
    }
}

#[test]
fn the_tenrikyo_services_are_the_headquarters_schedule() {
    expect(
        &TENRIKYO,
        &[
            // Tenrikyo Online: the Spring Grand Service of 26 January 2025,
            // the Autumn Grand Service of 26 October 2020 and the Oyasama
            // Birth Celebration Service of 18 April 2017, the 219th birthday.
            (2025, 1, 26, "Spring Grand Service"),
            (2020, 10, 26, "Autumn Grand Service"),
            (2017, 4, 18, "Oyasama Birth Celebration Service"),
            (2026, 1, 1, "New Year's Day Service"),
            (2026, 3, 27, "Spring Memorial Service"),
            (2026, 9, 27, "Autumn Memorial Service"),
        ],
    );
    for year in [2017, 2025, 2026] {
        let calendar = HolidayCalendar::for_year(&TENRIKYO, None, year);
        let monthly: Vec<u8> = calendar
            .all()
            .iter()
            .filter(|holiday| holiday.name == "Monthly Service")
            .map(|holiday| {
                let (_, month, day) = gregorian::from_fixed(holiday.date).expect("in range");
                assert_eq!(day, 26);
                month
            })
            .collect();
        // "1月と10月を除く毎月26日".
        assert_eq!(monthly, [2, 3, 4, 5, 6, 7, 8, 9, 11, 12], "{year}");
    }
}

/// The book's `unlucky-fridays` for 2000–2030, as `calendar.l` computes
/// them.
const BOOK_UNLUCKY_FRIDAYS: &[(i64, u8, u8)] = &[
    (2000, 10, 13),
    (2001, 4, 13),
    (2001, 7, 13),
    (2002, 9, 13),
    (2002, 12, 13),
    (2003, 6, 13),
    (2004, 2, 13),
    (2004, 8, 13),
    (2005, 5, 13),
    (2006, 1, 13),
    (2006, 10, 13),
    (2007, 4, 13),
    (2007, 7, 13),
    (2008, 6, 13),
    (2009, 2, 13),
    (2009, 3, 13),
    (2009, 11, 13),
    (2010, 8, 13),
    (2011, 5, 13),
    (2012, 1, 13),
    (2012, 4, 13),
    (2012, 7, 13),
    (2013, 9, 13),
    (2013, 12, 13),
    (2014, 6, 13),
    (2015, 2, 13),
    (2015, 3, 13),
    (2015, 11, 13),
    (2016, 5, 13),
    (2017, 1, 13),
    (2017, 10, 13),
    (2018, 4, 13),
    (2018, 7, 13),
    (2019, 9, 13),
    (2019, 12, 13),
    (2020, 3, 13),
    (2020, 11, 13),
    (2021, 8, 13),
    (2022, 5, 13),
    (2023, 1, 13),
    (2023, 10, 13),
    (2024, 9, 13),
    (2024, 12, 13),
    (2025, 6, 13),
    (2026, 2, 13),
    (2026, 3, 13),
    (2026, 11, 13),
    (2027, 8, 13),
    (2028, 10, 13),
    (2029, 4, 13),
    (2029, 7, 13),
    (2030, 9, 13),
    (2030, 12, 13),
];

#[test]
fn friday_the_thirteenth_is_the_books_unlucky_fridays() {
    // Reingold and Dershowitz's `unlucky-fridays`, run from `calendar.l`:
    // all 53 of its days of 2000–2030 and no others.
    let book: Vec<Rd> = BOOK_UNLUCKY_FRIDAYS
        .iter()
        .map(|&(y, m, d)| ymd(y, m, d))
        .collect();
    assert_eq!(book.len(), 53);
    let found: Vec<Rd> = (2000..=2030)
        .flat_map(|year| {
            HolidayCalendar::for_year(&UNLUCKY_FRIDAYS, None, year)
                .all()
                .iter()
                .map(|holiday| holiday.date)
                .collect::<Vec<_>>()
        })
        .collect();
    assert_eq!(found, book);
    // The days are read from 2000, the first year of the check; before it
    // there is a gap and no day.
    for year in [1900, 1999] {
        let calendar = HolidayCalendar::for_year(&UNLUCKY_FRIDAYS, None, year);
        assert!(calendar.all().is_empty(), "{year}");
        assert!(!calendar.is_complete(), "{year}");
    }
    for year in 2000..=2100 {
        let count = HolidayCalendar::for_year(&UNLUCKY_FRIDAYS, None, year)
            .all()
            .len();
        assert!((1..=3).contains(&count), "{year}: {count}");
    }
}

/// The book's `sacred-wednesdays` for 2000–2030, as `calendar.l` computes
/// them on its own Hindu lunar calendar.
const BOOK_SACRED_WEDNESDAYS: &[(i64, u8, u8)] = &[
    (2000, 4, 12),
    (2000, 9, 6),
    (2001, 1, 3),
    (2001, 5, 30),
    (2001, 10, 24),
    (2002, 2, 20),
    (2002, 7, 17),
    (2003, 12, 31),
    (2004, 4, 28),
    (2004, 9, 22),
    (2005, 2, 16),
    (2005, 6, 15),
    (2005, 11, 9),
    (2006, 8, 2),
    (2007, 9, 19),
    (2008, 1, 16),
    (2008, 6, 11),
    (2009, 3, 4),
    (2009, 7, 29),
    (2009, 11, 25),
    (2010, 9, 15),
    (2011, 1, 12),
    (2011, 5, 11),
    (2012, 6, 27),
    (2012, 11, 21),
    (2013, 3, 20),
    (2013, 8, 14),
    (2014, 1, 8),
    (2014, 5, 7),
    (2015, 6, 24),
    (2015, 10, 21),
    (2016, 3, 16),
    (2016, 12, 7),
    (2017, 5, 3),
    (2018, 10, 17),
    (2019, 2, 13),
    (2019, 7, 10),
    (2019, 12, 4),
    (2020, 4, 1),
    (2020, 8, 26),
    (2021, 10, 13),
    (2022, 2, 9),
    (2023, 3, 29),
    (2023, 7, 26),
    (2023, 12, 20),
    (2024, 9, 11),
    (2025, 2, 5),
    (2025, 10, 29),
    (2026, 7, 22),
    (2027, 4, 14),
    (2027, 9, 8),
    (2028, 1, 5),
    (2028, 5, 31),
    (2028, 10, 25),
    (2029, 2, 21),
    (2030, 8, 7),
];

#[test]
fn the_sacred_wednesdays_are_the_books() {
    // Every one of the book's 56 days of 2000–2030 and no other, in every
    // year, in both builds: the table reads the book's own calendar,
    // `hindu-lunar-surya-siddhanta`.
    let book: Vec<Rd> = BOOK_SACRED_WEDNESDAYS
        .iter()
        .map(|&(y, m, d)| ymd(y, m, d))
        .collect();
    assert_eq!(book.len(), 56);
    let mut ours = Vec::new();
    for year in 2000..=2030 {
        let calendar = HolidayCalendar::for_year(&SACRED_WEDNESDAYS, None, year);
        assert!(calendar.gaps().is_empty(), "{year}");
        for holiday in calendar.all() {
            assert_eq!(
                hc_calendar::Weekday::from_rd(holiday.date),
                hc_calendar::Weekday::Wednesday
            );
            ours.push(holiday.date);
        }
    }
    assert_eq!(ours, book);
    // The table's years are the ones the calendar converts whole, and a
    // year outside them is a gap, not a year without the day.
    let Rule::Tabulated {
        first_year,
        last_year,
        ..
    } = SACRED_WEDNESDAYS.rules[0].rule
    else {
        panic!("sacred-wednesdays is a tabulated rule");
    };
    let calendar = hc_calendars_indic::SiddhantaLunarCalendar::UJJAIN;
    let (Ok(earliest), Ok(latest)) = (calendar.earliest(), calendar.latest()) else {
        panic!("the calendar has a range");
    };
    let year_of = |day: Rd| gregorian::from_fixed(day).expect("in range").0;
    assert_eq!(first_year, year_of(earliest) + 1);
    assert_eq!(last_year, year_of(latest) - 1);
    for year in [first_year - 1, last_year + 1] {
        assert!(
            !HolidayCalendar::for_year(&SACRED_WEDNESDAYS, None, year)
                .gaps()
                .is_empty(),
            "{year}"
        );
    }
}

#[test]
fn the_church_of_the_east_year_is_its_own_calendar_of_2026_to_2029() {
    // The Assyrian Church of the East's liturgical calendar feed, every
    // season start it carries for 2026-2029.
    for (year, days) in [
        (
            2026,
            [
                (1, 26),
                (2, 15),
                (4, 5),
                (5, 24),
                (7, 12),
                (8, 30),
                (11, 1),
                (11, 29),
            ],
        ),
        (
            2027,
            [
                (1, 18),
                (2, 7),
                (3, 28),
                (5, 16),
                (7, 4),
                (8, 22),
                (10, 31),
                (11, 28),
            ],
        ),
        (
            2028,
            [
                (2, 7),
                (2, 27),
                (4, 16),
                (6, 4),
                (7, 23),
                (9, 10),
                (11, 5),
                (12, 3),
            ],
        ),
        (
            2029,
            [
                (1, 22),
                (2, 11),
                (4, 1),
                (5, 20),
                (7, 8),
                (8, 26),
                (11, 4),
                (12, 2),
            ],
        ),
    ] {
        let names = [
            "Rogation of the Ninevites",
            "First Sunday of the Great Fast",
            "Easter (the season of the Resurrection begins)",
            "Pentecost (the season of the Apostles begins)",
            "First Sunday of Summer (Nusardel)",
            "First Sunday of Elijah",
            "First Sunday of the Dedication of the Church",
            "First Sunday of the Annunciation (Subara)",
        ];
        for ((month, day), name) in days.into_iter().zip(names) {
            expect(&CHURCH_OF_THE_EAST, &[(year, month, day, name)]);
        }
        expect(
            &CHURCH_OF_THE_EAST,
            &[
                (year, 9, 13, "Feast of the Cross"),
                (year, 8, 6, "Transfiguration"),
                (year, 1, 6, "Epiphany (the season of Denha begins)"),
            ],
        );
    }
}

#[test]
fn elijah_begins_before_the_cross_in_every_year() {
    for year in 1965..=2100 {
        let elijah =
            only_date(&CHURCH_OF_THE_EAST, year, "First Sunday of Elijah").expect("every year");
        let cross = ymd(year, 9, 13);
        assert!(elijah < cross, "{year}");
        assert!(cross.0 - elijah.0 <= 42, "{year}");
        let summer = only_date(
            &CHURCH_OF_THE_EAST,
            year,
            "First Sunday of Summer (Nusardel)",
        )
        .expect("every year");
        // Seven weeks of Summer, or six when Summer loses a Sunday.
        assert!([42, 49].contains(&(elijah.0 - summer.0)), "{year}");
        let dedication = only_date(
            &CHURCH_OF_THE_EAST,
            year,
            "First Sunday of the Dedication of the Church",
        )
        .expect("every year");
        let annunciation = only_date(
            &CHURCH_OF_THE_EAST,
            year,
            "First Sunday of the Annunciation (Subara)",
        )
        .expect("every year");
        assert_eq!(annunciation.0 - dedication.0, 28, "{year}");
    }
    // Easter on 24 April 2011 would have put the first Sunday of Elijah on
    // 18 September, after the Cross: the sixth and seventh Sundays of
    // Summer merge and Elijah begins on 11 September.
    assert_eq!(
        only_date(&CHURCH_OF_THE_EAST, 2011, "First Sunday of Elijah"),
        Some(ymd(2011, 9, 11))
    );
    // Before the Gregorian calendar the table says nothing.
    assert_eq!(
        only_date(&CHURCH_OF_THE_EAST, 1960, "First Sunday of Elijah"),
        None
    );
}

#[test]
fn the_common_worship_transfers_are_the_ones_the_church_printed() {
    use hc_holiday::common_worship::COMMON_WORSHIP;
    expect(
        &COMMON_WORSHIP,
        &[
            // Daily Prayer, June and September 2026: the Visit of the
            // Blessed Virgin Mary on Monday 1 June, Trinity Sunday having
            // been 31 May; Barnabas kept on Thursday 11 June; Holy Cross Day,
            // Matthew and Michael and All Angels on their own days.
            (2026, 5, 31, "Trinity Sunday"),
            (
                2026,
                6,
                1,
                "The Visit of the Blessed Virgin Mary to Elizabeth",
            ),
            (2026, 6, 11, "Barnabas the Apostle"),
            (2026, 9, 14, "Holy Cross Day"),
            (2026, 9, 21, "Matthew, Apostle and Evangelist"),
            (2026, 9, 29, "Michael and All Angels"),
            // Full Fact, 23 April 2025: St George's Day on Monday 28 April
            // 2025, Easter being 20 April; St Mark follows on the Tuesday.
            (2025, 4, 28, "George, Martyr, Patron of England"),
            (2025, 4, 29, "Mark the Evangelist"),
            // The Rules applied: 25 March 2024 was the Monday of Holy Week.
            (
                2024,
                4,
                8,
                "The Annunciation of Our Lord to the Blessed Virgin Mary",
            ),
            // Easter on 23 March 2008: the Annunciation to Monday 31 March,
            // St Joseph to the Tuesday, Philip and James off Ascension Day.
            (
                2008,
                3,
                31,
                "The Annunciation of Our Lord to the Blessed Virgin Mary",
            ),
            (2008, 4, 1, "Joseph of Nazareth"),
            (2008, 5, 2, "Philip and James, Apostles"),
            // 30 November 2025 was the First Sunday of Advent.
            (2025, 12, 1, "Andrew the Apostle"),
            (2026, 1, 11, "The Baptism of Christ"),
            (2026, 11, 22, "Christ the King"),
        ],
    );
    expect_not(
        &COMMON_WORSHIP,
        &[
            (
                2026,
                5,
                31,
                "The Visit of the Blessed Virgin Mary to Elizabeth",
            ),
            (2025, 4, 23, "George, Martyr, Patron of England"),
            (2025, 11, 30, "Andrew the Apostle"),
        ],
    );
}

#[test]
fn the_years_the_common_worship_rules_leave_open_are_gaps() {
    use hc_holiday::common_worship::COMMON_WORSHIP;
    let gapped = |year: i64| -> Vec<&'static str> {
        HolidayCalendar::for_year(&COMMON_WORSHIP, None, year)
            .gaps()
            .iter()
            .map(|gap| gap.name)
            .collect()
    };
    // Easter on 17 April 2022: St George's Monday is St Mark's Day, and St
    // George goes on to the Tuesday, as the Church's Daily Prayer for
    // 26 April 2022 keeps it.
    assert!(gapped(2022).is_empty());
    expect(
        &COMMON_WORSHIP,
        &[
            (2022, 4, 25, "Mark the Evangelist"),
            (2022, 4, 26, "George, Martyr, Patron of England"),
            (2022, 5, 2, "Philip and James, Apostles"),
        ],
    );
    // Easter on 24 April 2011: Philip and James's Monday is St George's.
    assert_eq!(gapped(2011), ["Philip and James, Apostles"]);
    expect(
        &COMMON_WORSHIP,
        &[
            (2011, 5, 2, "George, Martyr, Patron of England"),
            (2011, 5, 3, "Mark the Evangelist"),
        ],
    );
    // Easter on 25 April 2038: 1 May is in Easter Week.
    assert_eq!(gapped(2038), ["Philip and James, Apostles"]);
    // Easter on 23 April 2079: St George's Monday is 1 May.
    assert_eq!(
        gapped(2079),
        [
            "George, Martyr, Patron of England",
            "Philip and James, Apostles"
        ]
    );
    expect(&COMMON_WORSHIP, &[(2079, 5, 2, "Mark the Evangelist")]);
    // Easter on 22 April 2057: St Mark's Tuesday is 1 May.
    assert_eq!(
        gapped(2057),
        ["Mark the Evangelist", "Philip and James, Apostles"]
    );
    expect(
        &COMMON_WORSHIP,
        &[(2057, 4, 30, "George, Martyr, Patron of England")],
    );
    assert!(gapped(2026).is_empty());
}

#[test]
fn no_festival_is_kept_where_the_common_worship_rules_forbid_it() {
    use hc_holiday::common_worship::{CELEBRATIONS, COMMON_WORSHIP, Rank};
    let rank = |name: &str| {
        CELEBRATIONS
            .iter()
            .find(|celebration| celebration.title == name)
            .map(|celebration| celebration.rank)
            .expect("every name is a celebration")
    };
    for year in 2001..=2100 {
        let calendar = HolidayCalendar::for_year(&COMMON_WORSHIP, None, year);
        let easter = hc_holiday::gregorian_easter(year).expect("in range");
        let advent = only_date(&CHRISTIAN_WESTERN, year, "First Sunday of Advent");
        let mut kept = 0;
        for holiday in calendar.all() {
            kept += 1;
            if rank(holiday.name) != Rank::Festival {
                continue;
            }
            let day = holiday.date;
            let offset = day.0 - easter.0;
            let others: Vec<&str> = calendar
                .on(day)
                .iter()
                .map(|other| other.name)
                .filter(|other| *other != holiday.name)
                .collect();
            assert!(
                others.is_empty(),
                "{year}: {} with {others:?}",
                holiday.name
            );
            let sunday = hc_calendar::Weekday::from_rd(day) == hc_calendar::Weekday::Sunday;
            let in_lent = (-46..0).contains(&offset);
            let in_eastertide = (0..=49).contains(&offset);
            let in_advent = advent.is_some_and(|first| day >= first && day.0 < first.0 + 28);
            assert!(
                !(sunday && (in_lent || in_eastertide || in_advent)),
                "{year}: {} on a Sunday it may not be kept on",
                holiday.name
            );
            assert!(
                !(1..=6).contains(&offset),
                "{year}: {} in Easter Week",
                holiday.name
            );
        }
        assert_eq!(kept + calendar.gaps().len(), CELEBRATIONS.len(), "{year}");
    }
}

/// The days a tradition gives a name in a Gregorian year.
fn days_named(set: &RuleSet, year: i64, name: &str) -> Vec<Rd> {
    HolidayCalendar::for_year(set, None, year)
        .all()
        .iter()
        .filter(|holiday| holiday.name == name)
        .map(|holiday| holiday.date)
        .collect()
}

/// Wikipedia (ja), "お盆": the Gregorian お盆 of 13 to 16 July and of 13 to
/// 16 August, with the 迎え火 on the 13th and the 送り火 on the 16th.
#[test]
fn obon_is_the_thirteenth_to_the_sixteenth_of_july_or_august() {
    for (set, month) in [(&traditions::OBON_JULY, 7), (&traditions::OBON_AUGUST, 8)] {
        assert_eq!(
            days_named(set, 2026, "Obon"),
            (13..=16)
                .map(|day| ymd(2026, month, day))
                .collect::<Vec<_>>()
        );
        expect(
            set,
            &[
                (2026, month, 13, "Mukaebi (welcoming fire)"),
                (2026, month, 16, "Okuribi (sending-off fire)"),
            ],
        );
        assert!(HolidayCalendar::for_year(set, None, 1872).all().is_empty());
    }
}

/// Wikipedia (ja), "お盆": 旧暦7月15日 falls from 8 August to 7 September;
/// over 1991–2030 the earliest is 8 August 2006 and the latest 6 September
/// 2025, and the leap seventh month of 2006, whose 15th was 7 September,
/// is not 旧盆. Its infobox gives 18 August 2024 and 27 August 2026.
#[test]
fn kyubon_is_the_fifteenth_of_the_seventh_lunar_month_and_not_its_leap_month() {
    let lunar = &traditions::OBON_LUNAR;
    expect(
        lunar,
        &[
            (2006, 8, 8, "Ukui (sending off the ancestors)"),
            (2025, 9, 6, "Ukui (sending off the ancestors)"),
            (2024, 8, 18, "Ukui (sending off the ancestors)"),
            (2026, 8, 27, "Ukui (sending off the ancestors)"),
            (2026, 8, 25, "Unkē (welcoming the ancestors)"),
            (2026, 8, 26, "Nakabi"),
        ],
    );
    assert!(
        days_named(lunar, 2006, "Kyūbon")
            .iter()
            .all(|day| day.0 < ymd(2006, 9, 1).0)
    );
    let fifteenths: Vec<Rd> = (1991..=2030)
        .flat_map(|year| days_named(lunar, year, "Ukui (sending off the ancestors)"))
        .collect();
    assert_eq!(fifteenths.len(), 40);
    let window = |day: &Rd| {
        let (year, _, _) = gregorian::from_fixed(*day).expect("a date");
        (ymd(year, 8, 8)..=ymd(year, 9, 7)).contains(day)
    };
    assert!(fifteenths.iter().all(window));
    assert_eq!(
        fifteenths.iter().min_by_key(|day| {
            let (year, _, _) = gregorian::from_fixed(**day).expect("a date");
            day.0 - ymd(year, 1, 1).0
        }),
        Some(&ymd(2006, 8, 8))
    );
    assert_eq!(
        fifteenths.iter().max_by_key(|day| {
            let (year, _, _) = gregorian::from_fixed(**day).expect("a date");
            day.0 - ymd(year, 1, 1).0
        }),
        Some(&ymd(2025, 9, 6))
    );
    let before = HolidayCalendar::for_year(lunar, None, 1843);
    assert!(before.all().is_empty());
    assert!(!before.gaps().is_empty());
}

/// 鷲神社 (Asakusa), 「今年の酉の市」: 令和8年, 一の酉 on 7 November and
/// 二の酉 on 19 November, 二の酉まで. Wikipedia (ja), "酉の市", tabulates
/// the 酉 days of November for 2009–2028.
#[test]
fn tori_no_ichi_is_on_the_rooster_days_of_november() {
    let table: &[(i64, &[u8])] = &[
        (2009, &[12, 24]),
        (2010, &[7, 19]),
        (2011, &[2, 14, 26]),
        (2012, &[8, 20]),
        (2013, &[3, 15, 27]),
        (2014, &[10, 22]),
        (2015, &[5, 17, 29]),
        (2016, &[11, 23]),
        (2017, &[6, 18, 30]),
        (2018, &[1, 13, 25]),
        (2019, &[8, 20]),
        (2020, &[2, 14, 26]),
        (2021, &[9, 21]),
        (2022, &[4, 16, 28]),
        (2023, &[11, 23]),
        (2024, &[5, 17, 29]),
        (2025, &[12, 24]),
        (2026, &[7, 19]),
        (2027, &[2, 14, 26]),
        (2028, &[8, 20]),
    ];
    for (year, days) in table {
        let found: Vec<Rd> = ["Ichi no Tori", "Ni no Tori", "San no Tori"]
            .iter()
            .flat_map(|name| days_named(&traditions::TORI_NO_ICHI, *year, name))
            .collect();
        let expected: Vec<Rd> = days.iter().map(|day| ymd(*year, 11, *day)).collect();
        assert_eq!(found, expected, "{year}");
        // A third 酉 exactly when the first falls on 1 to 6 November.
        assert_eq!(days.len() == 3, days[0] <= 6, "{year}");
    }
}

/// HugKum, 「2026年の「初午」はいつ？」: 6 February 2025, 1 February 2026,
/// 8 February 2027, 3 February 2028 and 9 February 2029, and 21 March 2026
/// on the 旧暦; Kyoto's tourism office puts 伏見稲荷大社's 初午大祭 on
/// 1 February 2026; 二の午 and 三の午 of 2026 on 13 and 25 February.
#[test]
fn hatsuuma_is_the_first_horse_day_of_february_or_of_the_second_lunar_month() {
    expect(
        &traditions::HATSUUMA,
        &[
            (2025, 2, 6, "Hatsuuma"),
            (2026, 2, 1, "Hatsuuma"),
            (2027, 2, 8, "Hatsuuma"),
            (2028, 2, 3, "Hatsuuma"),
            (2029, 2, 9, "Hatsuuma"),
            (2026, 2, 13, "Ni no Uma"),
            (2026, 2, 25, "San no Uma"),
        ],
    );
    assert!(days_named(&traditions::HATSUUMA, 2025, "San no Uma").is_empty());
    assert_eq!(
        days_named(&traditions::HATSUUMA_LUNAR, 2026, "Hatsuuma"),
        [ymd(2026, 3, 21)]
    );
}

/// KOYOMI NOTE, 「亥の子（いのこ）」: the first 亥 of the 旧暦's tenth month
/// in 2020–2028. All About: "現在は一般的に新暦11月の最初の亥の日で考えるので、
/// 2025年の「亥の子の日」は、11月2日".
#[test]
fn inoko_is_the_first_boar_day_of_the_tenth_lunar_month_or_of_november() {
    for (year, month, day) in [
        (2020, 11, 16),
        (2021, 11, 11),
        (2022, 10, 25),
        (2023, 11, 13),
        (2024, 11, 7),
        (2025, 11, 26),
        (2026, 11, 9),
        (2027, 11, 4),
        (2028, 11, 22),
    ] {
        assert_eq!(
            days_named(&traditions::INOKO, year, "Inoko"),
            [ymd(year, month, day)],
            "{year}"
        );
    }
    assert_eq!(
        days_named(&traditions::INOKO_NOVEMBER, 2025, "Inoko"),
        [ymd(2025, 11, 2)]
    );
}

/// 暦生活: 十日夜, 旧暦10月10日, "新暦でいうと2023年は11月22日"; 日本文化研究ブログ:
/// 11月10日 in many places, and 18 November 2026 on the 旧暦.
#[test]
fn tokanya_is_the_tenth_of_the_tenth_lunar_month_or_ten_november() {
    expect(
        &traditions::TOKANYA,
        &[(2023, 11, 22, "Tōkanya"), (2026, 11, 18, "Tōkanya")],
    );
    expect(&traditions::TOKANYA_NOVEMBER, &[(2026, 11, 10, "Tōkanya")]);
}

/// The Observatory's three resolutions of the 2033 problem number the
/// tenth month from 23 October or from 22 November 2033, so the 旧暦's
/// tenth-month days that year are gaps, not guesses; its seventh month,
/// which all three agree on, is answered.
#[test]
fn the_2033_problem_leaves_the_lunar_autumn_days_open() {
    for set in [&traditions::INOKO, &traditions::TOKANYA] {
        let calendar = HolidayCalendar::for_year(set, None, 2033);
        assert!(calendar.all().is_empty(), "{}", set.code);
        assert_eq!(calendar.gaps().len(), 1, "{}", set.code);
        assert!(HolidayCalendar::for_year(set, None, 2034).gaps().is_empty());
    }
    let obon = HolidayCalendar::for_year(&traditions::OBON_LUNAR, None, 2033);
    assert!(obon.gaps().is_empty());
    assert_eq!(obon.all().len(), 6);
}

/// The first and last years each folk-day table answers: 1873 for the
/// Gregorian ones, when Japan took up the Gregorian calendar, and 1844 to
/// 2146 for the 旧暦 ones, from the 天保暦 to the last year before the
/// next the Observatory leaves open. Outside those the Gregorian tables
/// have no day, since the reckoning did not exist, and the 旧暦 tables
/// report a gap, since the calendar does not answer.
#[test]
fn the_folk_day_tables_answer_their_first_and_last_years() {
    use hc_holiday::rule::RuleSet;
    let gregorian: [&RuleSet; 6] = [
        &traditions::OBON_JULY,
        &traditions::OBON_AUGUST,
        &traditions::TORI_NO_ICHI,
        &traditions::HATSUUMA,
        &traditions::INOKO_NOVEMBER,
        &traditions::TOKANYA_NOVEMBER,
    ];
    for set in gregorian {
        let first = HolidayCalendar::for_year(set, None, 1873);
        assert!(!first.all().is_empty(), "{}", set.code);
        assert!(first.is_complete(), "{}", set.code);
        let before = HolidayCalendar::for_year(set, None, 1872);
        assert!(before.all().is_empty(), "{}", set.code);
        assert!(before.is_complete(), "{}", set.code);
    }
    expect(
        &traditions::OBON_JULY,
        &[(1873, 7, 13, "Mukaebi (welcoming fire)")],
    );
    let lunar: [&RuleSet; 4] = [
        &traditions::OBON_LUNAR,
        &traditions::HATSUUMA_LUNAR,
        &traditions::INOKO,
        &traditions::TOKANYA,
    ];
    for set in lunar {
        for year in [1844, 2146] {
            let calendar = HolidayCalendar::for_year(set, None, year);
            assert!(!calendar.all().is_empty(), "{} {year}", set.code);
            assert!(calendar.is_complete(), "{} {year}", set.code);
        }
        for year in [1843, 2147] {
            let calendar = HolidayCalendar::for_year(set, None, year);
            assert!(calendar.all().is_empty(), "{} {year}", set.code);
            assert!(!calendar.gaps().is_empty(), "{} {year}", set.code);
        }
    }
    expect(
        &traditions::OBON_LUNAR,
        &[(1844, 8, 26, "Unkē (welcoming the ancestors)")],
    );
    expect(&traditions::TOKANYA, &[(2146, 11, 13, "Tōkanya")]);
}

/// The days around Galungan, by English Wikipedia's offsets
/// (`wikipedia-galungan`), from its Galungan of 17 June 2026 and Kuningan of
/// 27 June; and those of 25 September 2024 across no year's end.
#[test]
fn the_days_around_galungan_are_its_offsets() {
    let set = traditions::by_code("balinese-pawukon-days").expect("registered");
    expect(
        set,
        &[
            (2026, 6, 14, "Penyekeban"),
            (2026, 6, 15, "Penyajaan"),
            (2026, 6, 16, "Penampahan"),
            (2026, 6, 18, "Manis Galungan"),
            (2026, 6, 28, "Manis Kuningan"),
            (2024, 9, 24, "Penampahan"),
            (2024, 10, 6, "Manis Kuningan"),
        ],
    );
}

#[test]
fn galungan_and_kuningan_are_wikipedias_dates() {
    // Wikipedia, "Galungan": the dates of 2018–2028, Galungan on Buda
    // Kliwon Dungulan and Kuningan ten days later.
    let galungan_kuningan = [
        ((2018, 5, 30), (2018, 6, 9)),
        ((2018, 12, 26), (2019, 1, 5)),
        ((2019, 7, 24), (2019, 8, 3)),
        ((2020, 2, 19), (2020, 2, 29)),
        ((2020, 9, 16), (2020, 9, 26)),
        ((2021, 4, 14), (2021, 4, 24)),
        ((2021, 11, 10), (2021, 11, 20)),
        ((2022, 6, 8), (2022, 6, 18)),
        ((2023, 1, 4), (2023, 1, 14)),
        ((2023, 8, 2), (2023, 8, 12)),
        ((2024, 2, 28), (2024, 3, 9)),
        ((2024, 9, 25), (2024, 10, 5)),
        ((2025, 4, 23), (2025, 5, 3)),
        ((2025, 11, 19), (2025, 11, 29)),
        ((2026, 6, 17), (2026, 6, 27)),
        ((2027, 1, 13), (2027, 1, 23)),
        ((2027, 8, 11), (2027, 8, 21)),
        ((2028, 3, 8), (2028, 3, 18)),
    ];
    let set = &traditions::BALINESE_PAWUKON_DAYS;
    for ((gy, gm, gd), (ky, km, kd)) in galungan_kuningan {
        expect(set, &[(gy, gm, gd, "Galungan"), (ky, km, kd, "Kuningan")]);
    }
    // And no others in those years.
    for year in 2019..=2027 {
        let calendar = HolidayCalendar::for_year(set, None, year);
        let count = |name: &str| {
            calendar
                .all()
                .iter()
                .filter(|holiday| holiday.name == name)
                .count()
        };
        let listed = galungan_kuningan
            .iter()
            .filter(|((y, _, _), _)| *y == year)
            .count();
        assert_eq!(count("Galungan"), listed, "{year}");
    }
}

#[test]
fn tumpek_and_kajeng_kliwon_are_the_books_positions_in_the_pawukon() {
    // Reingold and Dershowitz's `tumpek` and `kajeng-keliwon`:
    // `positions-in-range` of day 13 of a 35-day cycle and day 8 of a
    // 15-day one, counted from the Pawukon day of RD 0 — here from the
    // anchor, Julian Day Number 146, directly.
    let anchor = 146 - 1_721_425;
    let set = &traditions::BALINESE_PAWUKON_DAYS;
    for year in 2000..=2030 {
        let calendar = HolidayCalendar::for_year(set, None, year);
        let dated = |keep: &dyn Fn(&str) -> bool| -> Vec<Rd> {
            let mut days: Vec<Rd> = calendar
                .all()
                .iter()
                .filter(|holiday| keep(holiday.name))
                .map(|holiday| holiday.date)
                .collect();
            days.sort_unstable();
            days
        };
        let book = |position: i64, cycle: i64| -> Vec<Rd> {
            (ymd(year, 1, 1).0..=ymd(year, 12, 31).0)
                .filter(|day| (day - anchor).rem_euclid(cycle) == position)
                .map(Rd)
                .collect()
        };
        assert_eq!(
            dated(&|name: &str| name.starts_with("Tumpek")),
            book(13, 35),
            "{year}"
        );
        assert_eq!(
            dated(&|name: &str| name == "Kajeng Kliwon"),
            book(8, 15),
            "{year}"
        );
    }
    // Tumpek Wariga, "25 days before Galungan": 3 February 2024 and
    // 28 February 2024.
    expect(
        set,
        &[
            (2024, 2, 3, "Tumpek Wariga"),
            (2024, 3, 9, "Tumpek Kuningan"),
        ],
    );
}

#[test]
fn the_qumran_festivals_keep_their_weekdays_and_their_courses() {
    use hc_calendar::Weekday;
    use hc_calendars_solar::qumran;
    // Talmon, p. 110: the Passover lamb on Tuesday 14/I, Passover on
    // Wednesday 15/I, the Omer on Sunday 26/I, Weeks on Sunday 15/III,
    // Atonement on Friday 10/VII, Booths on Wednesday 15/VII; 4Q320 4.ii:
    // the Second Pesah on the fifth day of the week, the Day of Remembrance
    // on the fourth.
    let weekdays = [
        ("Passover", Weekday::Tuesday),
        ("Feast of Unleavened Bread", Weekday::Wednesday),
        ("Waving of the Omer", Weekday::Sunday),
        ("Second Passover", Weekday::Thursday),
        ("Feast of Weeks", Weekday::Sunday),
        ("Day of Remembrance", Weekday::Wednesday),
        ("Day of Atonement", Weekday::Friday),
        ("Feast of Booths", Weekday::Wednesday),
    ];
    let set = &traditions::QUMRAN;
    let mut seen = 0;
    for year in 1..=400 {
        for holiday in HolidayCalendar::for_year(set, None, year).all() {
            let (_, weekday) = weekdays
                .iter()
                .find(|(name, _)| *name == holiday.name)
                .unwrap_or_else(|| panic!("{}", holiday.name));
            assert_eq!(Weekday::from_rd(holiday.date), *weekday, "{year}");
            seen += 1;
        }
    }
    // A 364-day year runs ahead of the Julian and Gregorian ones, so some
    // Gregorian year holds a festival twice.
    assert!(seen > 8 * 400, "{seen}");
    // The first year's Pesah, in the week of Maaziah (4Q320): the course
    // of 14/I of year 1 on the conventional epoch.
    let pesah = qumran::to_fixed(1, 1, 14).expect("in range");
    let (year, _, _) = gregorian::from_fixed(pesah).expect("in range");
    let calendar = HolidayCalendar::for_year(set, None, year);
    assert!(calendar.on(pesah).iter().any(|h| h.name == "Passover"));
    assert_eq!(
        qumran::COURSES[usize::from(qumran::course(pesah)) - 1],
        "Maaziah"
    );
}

#[test]
fn the_zoroastrians_of_iran_keep_the_name_days_on_thirty_day_months() {
    use hc_calendar::Month;
    use hc_holiday::rule::CalendarSystem;
    // Persian Wikipedia, "جشن‌های زرتشتی": each feast's day in the present
    // Iranian calendar beside its Zoroastrian date.
    let civil = [
        ("Farvardingan", 1, 19),
        ("Ardibeheshtgan", 2, 2),
        ("Khordadgan", 3, 4),
        ("Tirgan", 4, 10),
        ("Amordadgan", 5, 3),
        ("Shahrivargan", 5, 30),
        ("Mehregan", 7, 10),
        ("Abangan", 8, 4),
        ("Azargan", 9, 3),
        ("Digan", 9, 25),
        ("Digan", 10, 2),
        ("Digan", 10, 9),
        ("Digan", 10, 17),
        ("Bahmangan", 10, 26),
        ("Esfandgan", 11, 29),
    ];
    let set = &traditions::ZOROASTRIAN_IRANIAN;
    for solar_hijri_year in 1395..=1410 {
        for (name, month, day) in civil {
            let date = CalendarSystem::SOLAR_HIJRI
                .to_fixed(solar_hijri_year, Month::regular(month), day)
                .expect("in range");
            let (year, _, _) = gregorian::from_fixed(date).expect("in range");
            assert!(
                HolidayCalendar::for_year(set, None, year)
                    .on(date)
                    .iter()
                    .any(|holiday| holiday.name == name),
                "{solar_hijri_year} {name}"
            );
        }
    }
    // Mehregan on 2 October 2025, Tirgan on 1 July 2025.
    expect(set, &[(2025, 10, 2, "Mehregan"), (2025, 7, 1, "Tirgan")]);
}

#[test]
fn the_iranian_festivals_are_on_their_civil_dates() {
    // Wikipedia: Mehregan on 8 October; Yalda on 21 December, or 20 in a
    // leap year; Sadeh on 29, 30 or 31 January; Tirgan on 13 Tir, 2, 3 or
    // 4 July.
    let set = &traditions::IRANIAN_FESTIVALS;
    expect(
        set,
        &[
            (2025, 7, 4, "Tirgan"),
            (2025, 10, 8, "Mehregan"),
            (2024, 12, 20, "Yalda Night"),
            (2025, 12, 21, "Yalda Night"),
            (2026, 1, 30, "Sadeh"),
        ],
    );
    for year in 2000..=2040 {
        let calendar = HolidayCalendar::for_year(set, None, year);
        for holiday in calendar.all() {
            let (_, month, day) = gregorian::from_fixed(holiday.date).expect("in range");
            match holiday.name {
                "Yalda Night" => assert!(month == 12 && (20..=21).contains(&day), "{year}"),
                "Sadeh" => assert!(month == 1 && (29..=31).contains(&day), "{year}"),
                "Tirgan" => assert!(month == 7 && (2..=4).contains(&day), "{year}"),
                _ => {}
            }
        }
        assert_eq!(calendar.all().len(), 4, "{year}");
    }
}

#[test]
fn the_bulgarian_movable_name_days_are_the_days_of_bulgarian_wikipedias_table() {
    // Wikipedia (bg), "Имен ден", "Подвижни имени дни в България": the days
    // of 2010–2023.
    let table: &[(&str, [(u8, u8); 14])] = &[
        (
            "Тодоровден",
            [
                (2, 20),
                (3, 12),
                (3, 3),
                (3, 23),
                (3, 8),
                (2, 28),
                (3, 19),
                (3, 4),
                (2, 24),
                (3, 16),
                (3, 7),
                (3, 20),
                (3, 12),
                (3, 4),
            ],
        ),
        (
            "Лазаровден",
            [
                (3, 27),
                (4, 16),
                (4, 7),
                (4, 27),
                (4, 12),
                (4, 4),
                (4, 23),
                (4, 8),
                (3, 31),
                (4, 20),
                (4, 11),
                (4, 24),
                (4, 16),
                (4, 8),
            ],
        ),
        (
            "Цветница",
            [
                (3, 28),
                (4, 17),
                (4, 8),
                (4, 28),
                (4, 13),
                (4, 5),
                (4, 24),
                (4, 9),
                (4, 1),
                (4, 21),
                (4, 12),
                (4, 25),
                (4, 17),
                (4, 9),
            ],
        ),
        (
            "Великден",
            [
                (4, 4),
                (4, 24),
                (4, 15),
                (5, 5),
                (4, 20),
                (4, 12),
                (5, 1),
                (4, 16),
                (4, 8),
                (4, 28),
                (4, 19),
                (5, 2),
                (4, 24),
                (4, 16),
            ],
        ),
        (
            "Светли петък",
            [
                (4, 9),
                (4, 29),
                (4, 20),
                (5, 10),
                (4, 25),
                (4, 17),
                (5, 6),
                (4, 21),
                (4, 13),
                (5, 3),
                (4, 24),
                (5, 7),
                (4, 29),
                (4, 21),
            ],
        ),
        (
            "Томина неделя",
            [
                (4, 11),
                (5, 1),
                (4, 22),
                (5, 12),
                (4, 27),
                (4, 19),
                (5, 8),
                (4, 23),
                (4, 15),
                (5, 5),
                (4, 26),
                (5, 9),
                (5, 1),
                (4, 23),
            ],
        ),
        (
            "Спасовден",
            [
                (5, 13),
                (6, 2),
                (5, 24),
                (6, 13),
                (5, 29),
                (5, 21),
                (6, 9),
                (5, 25),
                (5, 17),
                (6, 6),
                (5, 28),
                (6, 10),
                (6, 2),
                (5, 25),
            ],
        ),
        (
            "Всички светии",
            [
                (5, 30),
                (6, 19),
                (6, 10),
                (6, 30),
                (6, 15),
                (6, 7),
                (6, 26),
                (6, 11),
                (6, 3),
                (6, 23),
                (6, 14),
                (6, 27),
                (6, 19),
                (6, 11),
            ],
        ),
        (
            "Всички български светии",
            [
                (6, 6),
                (6, 26),
                (6, 17),
                (7, 7),
                (6, 22),
                (6, 14),
                (7, 3),
                (6, 18),
                (6, 10),
                (6, 30),
                (6, 21),
                (7, 4),
                (6, 26),
                (6, 18),
            ],
        ),
    ];
    let set = &traditions::NAME_DAYS_BULGARIAN_MOVABLE;
    for (local_name, days) in table {
        for (offset, (month, day)) in days.iter().enumerate() {
            let year = 2010 + offset as i64;
            let found: Vec<Rd> = HolidayCalendar::for_year(set, None, year)
                .all()
                .iter()
                .filter(|holiday| holiday.local_name == *local_name)
                .map(|holiday| holiday.date)
                .collect();
            assert_eq!(found, [ymd(year, *month, *day)], "{local_name} {year}");
        }
    }
}

#[test]
fn the_greek_movable_name_days_follow_pascha_and_st_george_waits_for_it() {
    let set = &traditions::NAME_DAYS_GREEK_MOVABLE;
    expect(
        set,
        &[
            // Pascha 12 April 2026: Thomas Sunday a week on, All Saints 56
            // days on, St George and St Mark on their own days.
            (2026, 4, 19, "Thomas Sunday (Thomas)"),
            (
                2026,
                6,
                7,
                "All Saints (the names with no saint of their own)",
            ),
            (2026, 4, 23, "St George (George, Georgia)"),
            (2026, 4, 25, "St Mark (Mark)"),
            (2026, 2, 28, "Saturday of St Theodore (Theodore, Theodora)"),
            // Pascha 2 May 2021, after 23 April: St George on the Monday,
            // St Mark on the Tuesday.
            (2021, 5, 3, "St George (George, Georgia)"),
            (2021, 5, 4, "St Mark (Mark)"),
            // Pascha on 23 April 2006 is not after 23 April: St George
            // stays on the day, which is Pascha.
            (2006, 4, 23, "St George (George, Georgia)"),
            (2006, 4, 23, "Pascha (Anastasios, Anastasia)"),
            // Chloe: the first Sunday after 13 February, or the 13th itself
            // when it is a Sunday (2022).
            (2026, 2, 15, "Chloe"),
            (2022, 2, 13, "Chloe"),
            // The Forefathers: 11 December, or the Sunday after.
            (2022, 12, 11, "Sunday of the Forefathers"),
            (2026, 12, 13, "Sunday of the Forefathers"),
        ],
    );
    expect_not(set, &[(2021, 4, 23, "St George (George, Georgia)")]);
    // The Greek and Bulgarian tables count from the same Pascha.
    for year in 2010..=2040 {
        let day = |set: &RuleSet, local: &str| {
            HolidayCalendar::for_year(set, None, year)
                .all()
                .iter()
                .find(|holiday| holiday.local_name == local)
                .map(|holiday| holiday.date)
        };
        assert_eq!(
            day(set, "Του Θωμά"),
            day(&traditions::NAME_DAYS_BULGARIAN_MOVABLE, "Томина неделя"),
            "{year}"
        );
    }
}
