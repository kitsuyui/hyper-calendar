use super::super::*;
use super::read_lines;

/// 5782 (2021–22) and 5789 (2028–29) are sabbatical years, as
/// Wikipedia and Chabad.org list them.
#[test]
fn the_published_sabbatical_years_are_the_seventh() {
    assert_eq!(hc_hebrew_sabbatical_cycle_year(5_782), 7);
    assert_eq!(hc_hebrew_sabbatical_cycle_year(5_789), 7);
    assert_eq!(hc_hebrew_sabbatical_cycle_year(5_786), 4);
    assert_eq!(hc_hebrew_sabbatical_cycle_year(0), HC_ERR_OUT_OF_RANGE);
    assert_eq!(hc_hebrew_sabbatical_cycle_year(10_000), HC_ERR_OUT_OF_RANGE);
    assert!((1..=7).contains(&hc_hebrew_sabbatical_cycle_year(1)));
    assert!((1..=7).contains(&hc_hebrew_sabbatical_cycle_year(9_999)));
}

/// The first day carried, 23 September AD 4, is Sebaste of Kaisar,
/// and 7 October is day 14, as the Metropolis *hemerologion* has it.
#[test]
fn sebaste_is_unnumbered_and_the_days_after_it_are_counted() {
    let read = |fixed: i64| {
        read_lines(|buffer, capacity| unsafe { hc_asian_day(fixed, buffer, capacity) })
    };
    assert_eq!(read(1_360), "4\t1\tKaisar\tunnumbered\t1\n");
    assert_eq!(read(1_374), "4\t1\tKaisar\tnumbered\t14\n");
    assert_eq!(
        unsafe { hc_asian_day(1_359, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

/// Chaitra śukla 1 of Śaka 1947, Vikrama 2082, on 30 March 2025, by
/// the true sky at the Central Station and by the Siddhānta's at
/// Ujjain (`docs/systems/hindu-calendars.md`), with the Siddhānta's
/// Sun in Mīna and its first tithi at its sunrise.
#[test]
fn the_hindu_new_year_of_saka_1947_on_both_skies() {
    let day = hc_gregorian_to_fixed(2025, 3, 30);
    for (sky, latitude, longitude) in [
        ("lahiri", 23.183_333, 82.5),
        ("surya-siddhanta", 23.15, 75.768_333),
    ] {
        let text = read_lines(|buffer, capacity| unsafe {
            hc_hindu_lunar_date(
                sky.as_ptr(),
                sky.len(),
                day,
                latitude,
                longitude,
                0.0,
                "hi".as_ptr(),
                2,
                buffer,
                capacity,
            )
        });
        let cells: Vec<&str> = text.trim_end_matches('\n').split('\t').collect();
        assert_eq!(cells[..6], ["1947", "2082", "1", "0", "1", "0"], "{sky}");
        // Chaitra, in Hindi, with CLDR's शक for the era.
        assert_eq!(
            cells[7..],
            ["चैत्र", "", "शक", "Vikrama Samvat", "hi"],
            "{sky}"
        );
    }
    let text = read_lines(|buffer, capacity| unsafe {
        hc_surya_siddhanta_sunrise(day, 23.15, 75.768_333, buffer, capacity)
    });
    let sunrise: i64 = text.trim_end().parse().expect("an instant");
    let text =
        read_lines(|buffer, capacity| unsafe { hc_surya_siddhanta_at(sunrise, buffer, capacity) });
    let cells: Vec<&str> = text.trim_end_matches('\n').split('\t').collect();
    assert_eq!(cells[3..], ["1", "12"]);
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe {
            hc_hindu_lunar_date(
                "".as_ptr(),
                0,
                day,
                0.0,
                0.0,
                0.0,
                "en".as_ptr(),
                2,
                null,
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_hindu_lunar_date(
                "lahiri".as_ptr(),
                6,
                day,
                23.18,
                82.5,
                0.0,
                [0xffu8].as_ptr(),
                1,
                null,
                0,
            )
        },
        HC_ERR_NOT_UTF8
    );
    assert_eq!(
        unsafe { hc_surya_siddhanta_sunrise(day, 80.0, 0.0, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_surya_siddhanta_at(i64::MIN, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

/// The first of Ramadan 1445 at Mecca by `islamic-rgsa`, the
/// calendar the criterion begins months for: its eve carries a
/// crescent by Shaukat's criterion, and the eve before none.
#[test]
fn the_crescent_that_began_ramadan_1445_at_mecca() {
    let criterion = "shaukat";
    let visible = |day: i64| {
        let text = read_lines(|buffer, capacity| unsafe {
            hc_crescent_visible(
                criterion.as_ptr(),
                criterion.len(),
                day,
                21.423_333,
                39.823_333,
                298.0,
                buffer,
                capacity,
            )
        });
        let cells: Vec<String> = text
            .trim_end_matches('\n')
            .split('\t')
            .map(str::to_owned)
            .collect();
        assert_eq!(cells.len(), 7, "{text}");
        cells[0] == "1"
    };
    let first = hc::hc_calendars_lunar::islamic_observational::IslamicObservationalCalendar::MECCA
        .compose(1_445, 9, 1)
        .expect("in range")
        .0;
    assert!(visible(first));
    assert!(!visible(first - 1));
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_crescent_visible("danjon".as_ptr(), 6, first, 0.0, 0.0, 0.0, null, 0) },
        HC_ERR_UNKNOWN
    );
}

/// Drik Panchang for 1 January 2025, read at sunrise: Vyaghata and
/// Balava.
#[test]
fn the_panchanga_of_the_first_of_january_2025_decodes_column_by_column() {
    let day = hc_gregorian_to_fixed(2025, 1, 1);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_panchanga_of_day(
            day,
            23.183_333,
            82.5,
            0.0,
            "Lahiri".as_ptr(),
            6,
            buffer,
            capacity,
        )
    });
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|row| row.len() == 9), "{rows:?}");
    assert_eq!(rows[0][7..], ["lahiri", "Lahiri (Chitrapaksha)"]);
    assert_eq!(rows[1][7..], ["", ""]);
    assert_eq!(rows[0][..4], ["yoga", "13", "Vyaghata", "व्याघात"]);
    assert_eq!(rows[1][2..4], ["Balava", "बालव"]);
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_panchanga_of_day(day, 89.0, 0.0, 0.0, "Lahiri".as_ptr(), 6, null, 0) },
        HC_ERR_NO_DATA
    );
    assert_eq!(
        unsafe { hc_panchanga_of_day(day, f64::NAN, 0.0, 0.0, "Lahiri".as_ptr(), 6, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_panchanga_at(0, core::ptr::null(), 0, null, 0) },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe { hc_panchanga_at(i64::MAX, "Raman".as_ptr(), 5, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

#[test]
fn the_olympiads_and_the_hebrew_anniversaries_are_numbers_above_the_floor() {
    assert_eq!(hc_ioc_olympiad(2021), 32);
    assert_eq!(hc_ioc_olympiad(i64::MAX), 2_305_843_009_213_693_478);
    assert_eq!(hc_ioc_olympiad(1895), HC_ERR_OUT_OF_RANGE);
    // 10 Tevet 5780 was 7 January 2020; 10 Tevet 5781, 25 December.
    let death = hc_gregorian_to_fixed(2020, 1, 7);
    assert_eq!(
        hc_hebrew_yahrzeit(death, 5781),
        hc_gregorian_to_fixed(2020, 12, 25)
    );
    assert_eq!(
        hc_hebrew_birthday(death, 5781),
        hc_gregorian_to_fixed(2020, 12, 25)
    );
    assert_eq!(hc_hebrew_yahrzeit(death, 0), HC_ERR_OUT_OF_RANGE);
    assert_eq!(hc_hebrew_birthday(i64::MIN, 5781), HC_ERR_OUT_OF_RANGE);
}

/// Wikipedia's child born in June 2000, 13 *suì* from the lunar new
/// year of 2012; the South China Morning Post's widow year of 2024.
#[test]
fn the_chinese_reckonings_cross_the_boundary() {
    let birth = hc_gregorian_to_fixed(2000, 6, 15);
    assert_eq!(
        hc_chinese_reckoned_age(birth, hc_gregorian_to_fixed(2012, 1, 23)),
        13
    );
    assert_eq!(hc_chinese_reckoned_age(birth, birth - 1), HC_ERR_NO_DATA);
    assert_eq!(
        hc_chinese_reckoned_age(i64::MIN, birth),
        HC_ERR_OUT_OF_RANGE
    );
    let text = read_lines(|buffer, capacity| unsafe {
        hc_chinese_marriage_augury(4_661, buffer, capacity)
    });
    assert_eq!(text, "widow\t0\t0\n");
    assert_eq!(
        unsafe { hc_chinese_marriage_augury(i64::MAX, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

/// The book's ayanāṃśa, `reingold-dershowitz`, crosses the boundary by its
/// identifier, and the yoga it gives is the module's under it; the karaṇa,
/// which no ayanāṃśa moves, is Balava as under Lahiri.
#[test]
fn the_books_ayanamsa_crosses_the_boundary() {
    use hc::hc_astro::riseset::Location;
    use hc::hc_calendars_indic::panchanga::{YOGA_NAMES, yoga_of_day};
    use hc::hc_seasons::zodiac::Ayanamsa;
    let day = hc_gregorian_to_fixed(2025, 1, 1);
    let id = "reingold-dershowitz";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_panchanga_of_day(
            day,
            23.183_333,
            82.5,
            0.0,
            id.as_ptr(),
            id.len(),
            buffer,
            capacity,
        )
    });
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    let ayanamsa = Ayanamsa::REINGOLD_DERSHOWITZ;
    let yoga = yoga_of_day(
        hc::hc_calendar::Rd(day),
        Location::new(23.183_333, 82.5, 0.0),
        ayanamsa,
    );
    assert_eq!(rows[0][1], yoga.to_string());
    assert_eq!(rows[0][2], YOGA_NAMES[usize::from(yoga - 1)]);
    assert_eq!(rows[0][7..], [id, ayanamsa.name()]);
    assert_eq!(rows[1][2], "Balava");
}

/// 21 December 2025 is 甲子, whose 納音 is 海中金 (`wikipedia-ja-nacchin`):
/// the almanac's second line.
#[test]
fn the_nayin_line_crosses_the_boundary() {
    let day = hc_gregorian_to_fixed(2025, 12, 21);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_almanac_day(day, "japan".as_ptr(), 5, "ja".as_ptr(), 2, buffer, capacity)
    });
    let second = text.lines().nth(1).expect("two lines");
    assert!(
        second.starts_with("nayin\t1\t海中金\tja\t海中金\tkaichūkin"),
        "{second}"
    );
}
