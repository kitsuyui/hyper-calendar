//! Cross-calendar tests: every calendar in the crate seen through the
//! interfaces a caller actually uses, rather than through its own module.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarRegistry, DateFields, DynAdapter, DynCalendar,
    Month, Rd, Weekday,
};
use hc_calendars_lunar::{
    ChineseCalendar, DangiCalendar, HebrewCalendar, HebrewDate, IslamicAstronomicalCalendar,
    IslamicCivilCalendar, IslamicDate, IslamicObservationalCalendar, IslamicUmmAlQuraCalendar,
    JapaneseTenpoCalendar, LunisolarDate, VietnameseCalendar,
};

/// The proleptic Gregorian fixed day of a date, for anchoring the tests.
fn gregorian(year: i64, month: u8, day: u8) -> Rd {
    const OFFSETS: [i64; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let prior = year - 1;
    Rd(365 * prior + prior.div_euclid(4) - prior.div_euclid(100)
        + prior.div_euclid(400)
        + OFFSETS[month as usize - 1]
        + i64::from(month > 2 && leap)
        + i64::from(day))
}

#[test]
fn the_gregorian_helper_agrees_with_the_published_julian_day_numbers() {
    // 1970-01-01 is JDN 2 440 588 and 2000-01-01 is JDN 2 451 545 (J2000.0
    // is noon that day).
    assert_eq!(gregorian(1970, 1, 1).to_julian_day_number(), 2_440_588);
    assert_eq!(gregorian(2000, 1, 1).to_julian_day_number(), 2_451_545);
}

#[test]
fn every_calendar_has_the_identifier_it_should() {
    let identifiers = [
        IslamicCivilCalendar.meta().id,
        IslamicAstronomicalCalendar.meta().id,
        IslamicUmmAlQuraCalendar.meta().id,
        IslamicObservationalCalendar::MECCA.meta().id,
        HebrewCalendar.meta().id,
        ChineseCalendar.meta().id,
        DangiCalendar.meta().id,
        VietnameseCalendar.meta().id,
        JapaneseTenpoCalendar.meta().id,
    ];
    assert_eq!(
        identifiers,
        [
            CalendarId("islamic-civil"),
            CalendarId("islamic-tbla"),
            CalendarId("islamic-umalqura"),
            CalendarId("islamic-rgsa"),
            CalendarId("hebrew"),
            CalendarId("chinese"),
            CalendarId("dangi"),
            CalendarId("vietnamese"),
            CalendarId("japanese-tenpo"),
        ]
    );
    // No two calendars answer to the same name.
    for (index, left) in identifiers.iter().enumerate() {
        for right in &identifiers[index + 1..] {
            assert_ne!(left, right);
        }
    }
}

#[test]
fn only_the_lunisolar_calendars_and_the_hebrew_one_claim_leap_months() {
    assert!(HebrewCalendar.meta().has_leap_months);
    assert!(ChineseCalendar.meta().has_leap_months);
    assert!(DangiCalendar.meta().has_leap_months);
    assert!(VietnameseCalendar.meta().has_leap_months);
    assert!(JapaneseTenpoCalendar.meta().has_leap_months);
    assert!(!IslamicCivilCalendar.meta().has_leap_months);
    assert!(!IslamicAstronomicalCalendar.meta().has_leap_months);
    assert!(!IslamicUmmAlQuraCalendar.meta().has_leap_months);
    assert!(!IslamicObservationalCalendar::MECCA.meta().has_leap_months);
}

#[test]
fn only_the_computed_calendars_claim_to_be_astronomical() {
    // A table derived from astronomy is still a table; the flag describes
    // the implementation, not the calendar's history.
    assert!(!IslamicUmmAlQuraCalendar.meta().is_astronomical);
    assert!(!HebrewCalendar.meta().is_astronomical);
    assert!(!IslamicCivilCalendar.meta().is_astronomical);
    assert!(IslamicObservationalCalendar::MECCA.meta().is_astronomical);
    assert!(ChineseCalendar.meta().is_astronomical);
    assert!(DangiCalendar.meta().is_astronomical);
    assert!(VietnameseCalendar.meta().is_astronomical);
    assert!(JapaneseTenpoCalendar.meta().is_astronomical);
}

#[test]
fn a_registry_can_hold_every_calendar_in_the_crate() {
    let mut registry = CalendarRegistry::new();
    registry.insert(Box::new(DynAdapter::new(IslamicCivilCalendar)));
    registry.insert(Box::new(DynAdapter::new(IslamicAstronomicalCalendar)));
    registry.insert(Box::new(DynAdapter::new(IslamicUmmAlQuraCalendar)));
    registry.insert(Box::new(DynAdapter::new(HebrewCalendar)));
    registry.insert(Box::new(DynAdapter::new(ChineseCalendar)));
    registry.insert(Box::new(DynAdapter::new(DangiCalendar)));
    registry.insert(Box::new(DynAdapter::new(VietnameseCalendar)));
    registry.insert(Box::new(DynAdapter::new(JapaneseTenpoCalendar)));

    assert_eq!(registry.metas().count(), 8);
    assert!(registry.get(CalendarId("chinese")).is_some());
    assert!(registry.get(CalendarId("gregory")).is_none());

    // One day, described by every calendar — converted by seven of the
    // eight, because the Tenpō calendar ended in 1872 and says so.
    let day = gregorian(2024, 2, 10);
    let described = registry.describe_day(day);
    assert_eq!(described.len(), 8);
    let refused: Vec<_> = described
        .iter()
        .filter(|(_, fields)| fields.is_err())
        .map(|(id, _)| *id)
        .collect();
    assert_eq!(refused, [CalendarId("japanese-tenpo")]);
    for (id, fields) in described.into_iter().filter(|(_, f)| f.is_ok()) {
        let fields = fields.expect("filtered");
        assert!(fields.month.is_some(), "{id} gave no month");
        let calendar = registry.get(id).expect("just listed");
        assert_eq!(calendar.fields_to_fixed(&fields), Ok(day), "{id}");
    }
}

#[test]
fn the_tenpo_calendar_is_absent_from_a_modern_days_description() {
    let mut registry = CalendarRegistry::new();
    registry.insert(Box::new(DynAdapter::new(JapaneseTenpoCalendar)));
    registry.insert(Box::new(DynAdapter::new(ChineseCalendar)));
    // It ended in 1872, so a modern day has no Tenpō date: the registry
    // lists the calendar with its refusal rather than offering a date.
    let described = registry.describe_day(gregorian(2024, 2, 10));
    assert_eq!(described.len(), 2);
    assert_eq!(described[0].0, CalendarId("japanese-tenpo"));
    assert_eq!(described[0].1, Err(CalendarError::AfterSupportedRange));
    assert_eq!(described[1].0, CalendarId("chinese"));
    assert!(described[1].1.is_ok());
    // In 1860 both cover the day.
    let described = registry.describe_day(gregorian(1860, 6, 1));
    assert!(described.iter().all(|(_, fields)| fields.is_ok()));
}

#[test]
fn a_date_converts_straight_from_one_calendar_to_another() {
    let hebrew = HebrewDate {
        year: 5_784,
        month: Month::regular(7),
        day: 15,
    };
    // 15 Nisan 5784 is 2024-04-23, which is 14 Shawwāl 1445 in the tabular
    // civil Hijri calendar.
    let islamic = HebrewCalendar
        .convert_to(hebrew, &IslamicCivilCalendar)
        .expect("both cover the day");
    assert_eq!(HebrewCalendar.to_fixed(hebrew), Ok(gregorian(2024, 4, 23)));
    assert_eq!(islamic.year, 1_445);
    assert_eq!(islamic.month, 10);
    // And back again.
    assert_eq!(
        IslamicCivilCalendar.convert_to(islamic, &HebrewCalendar),
        Ok(hebrew)
    );
}

#[test]
fn the_four_lunisolar_calendars_agree_on_the_day_of_the_month_when_they_agree_at_all() {
    // They differ only in where the day boundary is drawn, so on most days
    // all of them give the same month and day.
    let mut agreements = 0;
    for offset in 0..2_000i64 {
        let day = Rd(gregorian(1860, 1, 1).0 + offset);
        let chinese = ChineseCalendar.from_fixed(day).expect("in range");
        let korean = DangiCalendar.from_fixed(day).expect("in range");
        let vietnamese = VietnameseCalendar.from_fixed(day).expect("in range");
        let japanese = JapaneseTenpoCalendar.from_fixed(day).expect("in range");
        if (chinese.month, chinese.day) == (korean.month, korean.day)
            && (chinese.month, chinese.day) == (vietnamese.month, vietnamese.day)
            && (chinese.month, chinese.day) == (japanese.month, japanese.day)
        {
            agreements += 1;
        }
    }
    assert!(agreements > 1_800, "only {agreements} of 2000 days agreed");
}

#[test]
fn every_calendar_refuses_a_day_beyond_its_range_with_the_right_error() {
    let far_future = Rd(3_000_000);
    let far_past = Rd(-3_000_000);
    assert_eq!(
        ChineseCalendar.from_fixed(far_future),
        Err(CalendarError::AfterSupportedRange)
    );
    assert_eq!(
        ChineseCalendar.from_fixed(far_past),
        Err(CalendarError::BeforeEpoch)
    );
    assert_eq!(
        HebrewCalendar.from_fixed(far_past),
        Err(CalendarError::BeforeEpoch)
    );
    assert_eq!(
        IslamicUmmAlQuraCalendar.from_fixed(far_future),
        Err(CalendarError::AfterSupportedRange)
    );
    assert_eq!(
        JapaneseTenpoCalendar.from_fixed(far_future),
        Err(CalendarError::AfterSupportedRange)
    );
    assert_eq!(
        IslamicObservationalCalendar::MECCA.from_fixed(far_future),
        Err(CalendarError::AfterSupportedRange)
    );
}

#[test]
fn a_lunar_year_is_about_eleven_days_shorter_than_a_solar_one() {
    // The Hijri calendar's whole character: the same date walks backwards
    // through the seasons by about eleven days a year.
    // 1444 AH sits at position 4 of the thirty-year cycle, which is short.
    let first = IslamicCivilCalendar
        .to_fixed(IslamicDate {
            year: 1_444,
            month: 1,
            day: 1,
        })
        .expect("valid");
    let next = IslamicCivilCalendar
        .to_fixed(IslamicDate {
            year: 1_445,
            month: 1,
            day: 1,
        })
        .expect("valid");
    assert_eq!(next.0 - first.0, 354);
    // Thirty-three Hijri years fit inside thirty-two Gregorian ones.
    let later = IslamicCivilCalendar
        .to_fixed(IslamicDate {
            year: 1_444 + 33,
            month: 1,
            day: 1,
        })
        .expect("valid");
    let gregorian_years = (later.0 - first.0) as f64 / 365.2425;
    assert!(
        (gregorian_years - 32.0).abs() < 0.5,
        "{gregorian_years} Gregorian years"
    );
}

#[test]
fn a_lunisolar_year_stays_put_against_the_seasons() {
    // The point of the leap month: the new year never wanders more than a
    // month, unlike the Hijri new year.
    let mut earliest = 400u16;
    let mut latest = 0u16;
    for year in 4_600..4_700i64 {
        let start = hc_calendars_lunar::chinese::new_year(year).expect("in range");
        let day_of_year = start.0 - gregorian(hc_year_of(start), 1, 1).0;
        let day_of_year = u16::try_from(day_of_year).expect("within a year");
        earliest = earliest.min(day_of_year);
        latest = latest.max(day_of_year);
    }
    assert!(
        latest - earliest <= 31,
        "spread of {} days",
        latest - earliest
    );
}

/// The Gregorian year containing a fixed day, by search; the tests do not
/// need it to be fast.
fn hc_year_of(rd: Rd) -> i64 {
    let mut year = (rd.0 as f64 / 365.2425) as i64;
    while gregorian(year + 1, 1, 1).0 <= rd.0 {
        year += 1;
    }
    while gregorian(year, 1, 1).0 > rd.0 {
        year -= 1;
    }
    year
}

#[test]
fn the_weekday_is_the_same_whatever_calendar_names_the_day() {
    // Rata Die is the pivot, so this is true by construction — which is
    // exactly the property worth pinning down.
    let day = gregorian(2024, 2, 10);
    assert_eq!(Weekday::from_rd(day), Weekday::Saturday);
    let chinese = ChineseCalendar.from_fixed(day).expect("in range");
    let hebrew = HebrewCalendar.from_fixed(day).expect("in range");
    let islamic = IslamicUmmAlQuraCalendar.from_fixed(day).expect("in range");
    assert_eq!(
        Weekday::from_rd(ChineseCalendar.to_fixed(chinese).expect("valid")),
        Weekday::Saturday
    );
    assert_eq!(
        Weekday::from_rd(HebrewCalendar.to_fixed(hebrew).expect("valid")),
        Weekday::Saturday
    );
    assert_eq!(
        Weekday::from_rd(IslamicUmmAlQuraCalendar.to_fixed(islamic).expect("valid")),
        Weekday::Saturday
    );
}

#[test]
fn the_dynamic_interface_reports_month_and_year_lengths() {
    let hebrew = DynAdapter::new(HebrewCalendar);
    // Tishrei always has 30 days; 5784 was a deficient leap year of 383.
    assert_eq!(hebrew.days_in_month(&DateFields::ymd(5_784, 1, 1)), Ok(30));
    assert_eq!(hebrew.days_in_year(5_784), Ok(383));
    assert_eq!(hebrew.is_leap_year(5_784), Ok(true));

    let islamic = DynAdapter::new(IslamicCivilCalendar);
    assert_eq!(islamic.days_in_month(&DateFields::ymd(1_445, 1, 1)), Ok(30));
    assert_eq!(islamic.days_in_month(&DateFields::ymd(1_445, 2, 1)), Ok(29));
    assert_eq!(islamic.days_in_year(1_453), Ok(355));
    assert_eq!(islamic.days_in_year(1_454), Ok(354));
}

#[test]
fn a_lunisolar_leap_month_is_reachable_through_the_generic_fields() {
    // The generic interface has to be able to say "the leap second month of
    // 2023", which is the reason `Month` carries a flag at all.
    let leap = DateFields::ymd_leap_month(4_660, 2, 1);
    let calendar = DynAdapter::new(ChineseCalendar);
    assert_eq!(calendar.fields_to_fixed(&leap), Ok(gregorian(2023, 3, 22)));
    let read_back = calendar
        .fixed_to_fields(gregorian(2023, 3, 22))
        .expect("in range");
    assert_eq!(read_back.month, Some(Month::leap(2)));
    // The ordinary second month of that year is a lunation earlier.
    let regular = DateFields::ymd(4_660, 2, 1);
    assert_eq!(
        calendar.fields_to_fixed(&regular),
        Ok(gregorian(2023, 2, 20))
    );
}

#[test]
fn the_three_hijri_implementations_stay_within_a_few_days_of_one_another() {
    let tabular = IslamicCivilCalendar;
    let table = IslamicUmmAlQuraCalendar;
    let predicted = IslamicObservationalCalendar::MECCA;
    for year in 1_420..1_445i64 {
        for month in [1u8, 9, 10, 12] {
            let date = IslamicDate {
                year,
                month,
                day: 1,
            };
            let a = tabular.to_fixed(date).expect("valid");
            let b = table.to_fixed(date).expect("in the table");
            let c = predicted.to_fixed(date).expect("in range");
            assert!((a.0 - b.0).abs() <= 3, "{year}-{month} tabular vs table");
            assert!((c.0 - b.0).abs() <= 2, "{year}-{month} predicted vs table");
        }
    }
}

#[test]
fn a_lunisolar_date_carries_its_place_in_the_sexagenary_cycle() {
    let calendar = ChineseCalendar;
    let date = LunisolarDate::new(4_661, Month::regular(1), 1);
    let fields = calendar.to_fields(date).expect("describable");
    assert_eq!(fields.extra.get("cycle"), Some(78));
    assert_eq!(fields.extra.get("year_of_cycle"), Some(41));
    // Index 40 of the sixty-term cycle is jiǎ-chén.
    assert_eq!(fields.extra.get("sexagenary_year"), Some(40));
    // The first month of a jiǎ year is bǐng-yín, index 2.
    assert_eq!(fields.extra.get("sexagenary_month"), Some(2));
    assert_eq!(fields.extra.len(), 4);
}
