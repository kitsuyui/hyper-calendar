use super::super::*;
use super::{measured, read_lines};

/// 5782 is a sabbatical year; 23 September AD 4 is Sebaste of Kaisar.
#[test]
fn the_sabbatical_place_and_the_asian_day_are_the_modules() {
    let mut place = 0i64;
    assert_eq!(
        unsafe { hc_hebrew_sabbatical_cycle_year(5_782, &mut place) },
        HC_OK
    );
    assert_eq!(place, 7);
    assert_eq!(
        unsafe { hc_hebrew_sabbatical_cycle_year(10_000, &mut place) },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_hebrew_sabbatical_cycle_year(0, &mut place) },
        HC_ERROR_OUT_OF_RANGE
    );
    for year in [1, 9_999] {
        assert_eq!(
            unsafe { hc_hebrew_sabbatical_cycle_year(year, &mut place) },
            HC_OK
        );
        assert!((1..=7).contains(&place), "{year}: {place}");
    }
    assert_eq!(
        unsafe { hc_hebrew_sabbatical_cycle_year(5_782, core::ptr::null_mut()) },
        HC_ERROR_NULL_POINTER
    );
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_asian_day(1_360, buffer, capacity, written)
    });
    assert_eq!(text, "4\t1\tKaisar\tunnumbered\t1\n");
}

/// Chaitra śukla 1 of Śaka 1947 on 30 March 2025 at Ujjain on the
/// Siddhānta's sky, its sky at its sunrise, and a crescent line, each
/// the module's.
#[test]
fn the_hindu_and_crescent_lines_are_the_modules() {
    let mut day = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2025, 3, 30, &mut day) },
        HC_OK
    );
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_hindu_lunar_date(
            c"surya-siddhanta".as_ptr(),
            day,
            23.15,
            75.768_333,
            0.0,
            c"en".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert!(line.starts_with("1947\t2082\t1\t0\t1\t0\t"), "{line:?}");
    assert!(
        line.ends_with("\tChaitra\t\tSaka\tVikrama Samvat\ten\n"),
        "{line:?}"
    );
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_surya_siddhanta_sunrise(day, 23.15, 75.768_333, buffer, capacity, written)
    });
    let sunrise: i64 = line.trim_end().parse().expect("an instant");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_surya_siddhanta_at(sunrise, buffer, capacity, written)
    });
    assert!(line.ends_with("\t1\t12\n"), "{line:?}");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_crescent_visible(
            c"saudi-rule".as_ptr(),
            day,
            21.4,
            39.8,
            0.0,
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(
        line.trim_end_matches('\n').split('\t').count(),
        7,
        "{line:?}"
    );
    let mut written = 0usize;
    assert_eq!(
        unsafe {
            hc_crescent_visible(
                core::ptr::null(),
                day,
                0.0,
                0.0,
                0.0,
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        unsafe {
            hc_hindu_lunar_date(
                c"lahiri".as_ptr(),
                0,
                0.0,
                0.0,
                0.0,
                core::ptr::null(),
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
}

/// Every refusal of the Hindu, crescent, Sūrya Siddhānta and Asian
/// entry points that their documentation names, at the ends of the
/// ranges the README gives.
#[test]
fn the_calendar_day_entry_points_refuse_as_documented() {
    let (ujjain_lat, ujjain_lon) = (23.15, 75.768_333);
    let sunrise = |fixed: i64, latitude: f64| {
        measured(|buffer, capacity, written| unsafe {
            hc_surya_siddhanta_sunrise(fixed, latitude, ujjain_lon, buffer, capacity, written)
        })
    };
    assert_eq!(sunrise(-1_132_604, ujjain_lat), HC_ERROR_BUFFER_TOO_SMALL);
    assert_eq!(sunrise(2_519_974, ujjain_lat), HC_ERROR_BUFFER_TOO_SMALL);
    assert_eq!(sunrise(-1_132_605, ujjain_lat), HC_ERROR_OUT_OF_RANGE);
    assert_eq!(sunrise(2_519_975, ujjain_lat), HC_ERROR_OUT_OF_RANGE);
    assert_eq!(sunrise(739_340, 66.0), HC_ERROR_OUT_OF_RANGE);
    assert_eq!(sunrise(739_340, 91.0), HC_ERROR_OUT_OF_RANGE);

    let at = |unix_seconds: i64| {
        measured(|buffer, capacity, written| unsafe {
            hc_surya_siddhanta_at(unix_seconds, buffer, capacity, written)
        })
    };
    assert_eq!(at(-159_992_668_800), HC_ERROR_BUFFER_TOO_SMALL);
    assert_eq!(at(155_590_156_799), HC_ERROR_BUFFER_TOO_SMALL);
    assert_eq!(at(-159_992_668_801), HC_ERROR_OUT_OF_RANGE);
    assert_eq!(at(155_590_156_800), HC_ERROR_OUT_OF_RANGE);

    // The true sky at the Central Station with Lahiri's ayanāṃśa.
    let hindu = |sky: *const c_char, fixed: i64, latitude: f64| {
        measured(|buffer, capacity, written| unsafe {
            hc_hindu_lunar_date(
                sky,
                fixed,
                latitude,
                82.5,
                0.0,
                core::ptr::null(),
                buffer,
                capacity,
                written,
            )
        })
    };
    let lahiri = c"lahiri".as_ptr();
    assert_eq!(
        hindu(lahiri, 620_627, 23.183_333),
        HC_ERROR_BUFFER_TOO_SMALL
    );
    assert_eq!(
        hindu(lahiri, 839_773, 23.183_333),
        HC_ERROR_BUFFER_TOO_SMALL
    );
    assert_eq!(hindu(lahiri, 620_626, 23.183_333), HC_ERROR_OUT_OF_RANGE);
    assert_eq!(hindu(lahiri, 839_774, 23.183_333), HC_ERROR_OUT_OF_RANGE);
    assert_eq!(
        hindu(c"surya-siddhanta".as_ptr(), 739_340, 66.0),
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        hindu(c"sidereal".as_ptr(), 739_340, 23.183_333),
        HC_ERROR_UNKNOWN
    );
    assert_eq!(hindu(c"".as_ptr(), 739_340, 23.183_333), HC_ERROR_UNKNOWN);
    assert_eq!(
        hindu(core::ptr::null(), 739_340, 23.183_333),
        HC_ERROR_NULL_POINTER
    );

    let crescent = |criterion: *const c_char, fixed: i64, latitude: f64| {
        measured(|buffer, capacity, written| unsafe {
            hc_crescent_visible(
                criterion, fixed, latitude, 39.8, 0.0, buffer, capacity, written,
            )
        })
    };
    let shaukat = c"shaukat".as_ptr();
    assert_eq!(crescent(shaukat, -365_607, 21.4), HC_ERROR_BUFFER_TOO_SMALL);
    assert_eq!(
        crescent(shaukat, 1_095_727, 21.4),
        HC_ERROR_BUFFER_TOO_SMALL
    );
    assert_eq!(crescent(shaukat, -365_608, 21.4), HC_ERROR_OUT_OF_RANGE);
    assert_eq!(crescent(shaukat, 1_095_728, 21.4), HC_ERROR_OUT_OF_RANGE);
    assert_eq!(crescent(shaukat, 739_340, 91.0), HC_ERROR_OUT_OF_RANGE);
    assert_eq!(
        crescent(c"danjon".as_ptr(), 739_340, 21.4),
        HC_ERROR_UNKNOWN
    );

    let asian = |fixed: i64| {
        measured(|buffer, capacity, written| unsafe {
            hc_asian_day(fixed, buffer, capacity, written)
        })
    };
    assert_eq!(asian(3_652_398), HC_ERROR_BUFFER_TOO_SMALL);
    assert_eq!(asian(1_359), HC_ERROR_OUT_OF_RANGE);
    assert_eq!(asian(3_652_399), HC_ERROR_OUT_OF_RANGE);
}

fn fixed(year: i64, month: u8, day: u8) -> i64 {
    let mut fixed = 0;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(year, month, day, &mut fixed) },
        HC_OK
    );
    fixed
}

/// Drik Panchang for 1 January 2025, read at sunrise.
#[test]
fn the_panchanga_lines_are_the_modules() {
    let day = fixed(2025, 1, 1);
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_panchanga_of_day(
            day,
            23.183_333,
            82.5,
            0.0,
            c"lahiri".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0][2], "Vyaghata");
    assert_eq!(rows[1][2], "Balava");
    let mut written = 0usize;
    assert_eq!(
        unsafe { hc_panchanga_at(0, core::ptr::null(), core::ptr::null_mut(), 0, &mut written) },
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        unsafe {
            hc_panchanga_of_day(
                day,
                89.0,
                0.0,
                0.0,
                c"Lahiri".as_ptr(),
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_NO_DATA
    );
}

#[test]
fn the_olympiads_and_the_hebrew_anniversaries_are_out_parameters() {
    let mut out = 0i64;
    assert_eq!(unsafe { hc_ioc_olympiad(2021, &mut out) }, HC_OK);
    assert_eq!(out, 32);
    assert_eq!(
        unsafe { hc_ioc_olympiad(1895, &mut out) },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_ioc_olympiad(2021, core::ptr::null_mut()) },
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        unsafe { hc_hebrew_yahrzeit(fixed(2020, 1, 7), 5781, &mut out) },
        HC_OK
    );
    assert_eq!(out, fixed(2020, 12, 25));
    assert_eq!(
        unsafe { hc_hebrew_birthday(fixed(2020, 1, 7), 5781, &mut out) },
        HC_OK
    );
    assert_eq!(out, fixed(2020, 12, 25));
    assert_eq!(
        unsafe { hc_hebrew_yahrzeit(fixed(2020, 1, 7), 10_000, &mut out) },
        HC_ERROR_OUT_OF_RANGE
    );
}

#[test]
fn the_chinese_reckonings_cross_the_boundary() {
    let mut age = 0u32;
    let birth = fixed(2000, 6, 15);
    assert_eq!(
        unsafe { hc_chinese_reckoned_age(birth, fixed(2012, 1, 23), &mut age) },
        HC_OK
    );
    assert_eq!(age, 13);
    assert_eq!(
        unsafe { hc_chinese_reckoned_age(birth, birth - 1, &mut age) },
        HC_ERROR_NO_DATA
    );
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_chinese_marriage_augury(4_646, buffer, capacity, written)
    });
    assert_eq!(line, "double-bright\t1\t1\n");
}
