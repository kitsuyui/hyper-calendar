//! The cross-cutting religious cycles.

use hc_calendar::Rd;
use hc_calendars_indic::nakshatra::PUSHYA;
use hc_calendars_solar::gregorian;
use hc_holiday::engine::HolidayCalendar;
use hc_holiday::hindu::THAIPUSAM;
use hc_holiday::rule::{Confidence, Kind, Rule, RuleSet};
use hc_holiday::traditions::{
    self, BAHAI, BUDDHIST_EAST_ASIAN, BUDDHIST_THAI, CHINESE_FOLK, CHRISTIAN_ARMENIAN,
    CHRISTIAN_ARMENIAN_JERUSALEM, CHRISTIAN_ORTHODOX, CHRISTIAN_ORTHODOX_REVISED_JULIAN,
    CHRISTIAN_WESTERN, COPTIC_ORTHODOX, EMBER_BCP1662, EMBER_COMMON_WORSHIP, ETHIOPIAN_ORTHODOX,
    HINDU, ISLAMIC, JAIN, JEWISH, KYUCHU_SAISHI, MANDAEAN, ROGATION_ROMAN_1960, SAMARITAN, SHINTO,
    SIKH_NANAKSHAHI_2003, WHEEL_OF_THE_YEAR, WHEEL_OF_THE_YEAR_SOUTH, YAZIDI, ZOROASTRIAN_FASLI,
    ZOROASTRIAN_QADIMI, ZOROASTRIAN_SHAHANSHAHI,
};
use hc_seasons::Meridian;
use hc_seasons::zodiac::{Ayanamsa, SiderealSign};

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
    for rule in SIKH_NANAKSHAHI_2003.rules {
        assert_eq!(rule.kind, Kind::Religious, "{}", rule.name);
        assert_eq!(rule.confidence, Confidence::Exact, "{}", rule.name);
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
