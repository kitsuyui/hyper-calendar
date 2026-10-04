use super::super::*;
use super::{measured, read_lines};

/// Warren's row for 29 February 1992 in the IDL Astronomy Library's
/// `helio_jd`, as the module writes both scales.
#[test]
fn the_hjd_lines_are_the_modules() {
    let mut day = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(1992, 2, 29, &mut day) },
        HC_OK
    );
    let date = day as f64 + 1_721_424.5 + 11_756.2 / 86_400.0;
    let (alpha, delta) = (194.114_166_7, 42.171_388_9);
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_hjd_tt(date, alpha, delta, buffer, capacity, written)
    });
    assert_eq!(Ok(text), hc::astro_lines::hjd_tt_line(date, alpha, delta));
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_hjd_utc(date, alpha, delta, 1, buffer, capacity, written)
    });
    assert_eq!(
        Ok(text),
        hc::astro_lines::hjd_utc_line(date, alpha, delta, true)
    );
    let mut written = 0usize;
    assert_eq!(
        unsafe { hc_hjd_tt(date, 400.0, delta, core::ptr::null_mut(), 0, &mut written) },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe {
            hc_hjd_tt(
                f64::NAN,
                alpha,
                delta,
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
    // A date outside the years −1000 to 3000 (JD 1 000 000 is in
    // −1975, JD 2 900 000 in 3227) and a declination past either
    // pole, on both scales.
    for (date, delta) in [
        (1_000_000.0, delta),
        (2_900_000.0, delta),
        (date, 90.000_1),
        (date, -90.000_1),
    ] {
        assert_eq!(
            unsafe { hc_hjd_tt(date, alpha, delta, core::ptr::null_mut(), 0, &mut written) },
            HC_ERROR_OUT_OF_RANGE,
            "{date} {delta}"
        );
        assert_eq!(
            unsafe {
                hc_hjd_utc(
                    date,
                    alpha,
                    delta,
                    0,
                    core::ptr::null_mut(),
                    0,
                    &mut written,
                )
            },
            HC_ERROR_OUT_OF_RANGE,
            "{date} {delta}"
        );
    }
    // 1858, before the leap-second table: refused only under `strict`.
    let before_1961 = 2_400_000.5;
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_hjd_utc(before_1961, alpha, delta, 1, buffer, capacity, written)
        }),
        HC_ERROR_NO_DATA
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_hjd_utc(before_1961, alpha, delta, 0, buffer, capacity, written)
        }),
        HC_ERROR_BUFFER_TOO_SMALL
    );
}

/// The USNO's sunrise for Jerusalem on 2024-01-01, 06:39 at UT+2
/// (`usno-api-rstt`), under its own horizon, and the module's lines.
#[test]
fn the_horizons_are_the_modules_and_the_usnos_rises_on_its_minute() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_horizons(c"zh-TW".as_ptr(), buffer, capacity, written)
    });
    assert_eq!(text, hc::astro_lines::horizons_lines("zh-TW"));
    assert!(text.contains("\t美國海軍天文氣象台\tzh-Hant\n"), "{text}");
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_horizons(core::ptr::null(), buffer, capacity, written)
    });
    assert_eq!(text, hc::astro_lines::horizons_lines("en"));
    let mut day = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2024, 1, 1, &mut day) },
        HC_OK
    );
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_sunrise(
            c"usno".as_ptr(),
            day,
            31.78,
            35.24,
            740.0,
            buffer,
            capacity,
            written,
        )
    });
    let instant: i64 = line
        .split('\t')
        .next()
        .expect("a cell")
        .parse()
        .expect("an instant");
    assert!((instant - 1_704_083_940).abs() <= 31, "{line:?}");
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_sunset(
            c"usno".as_ptr(),
            day,
            31.78,
            35.24,
            740.0,
            buffer,
            capacity,
            written,
        )
    });
    let instant: i64 = line
        .split('\t')
        .next()
        .expect("a cell")
        .parse()
        .expect("an instant");
    assert!((instant - 1_704_120_360).abs() <= 31, "{line:?}");
    let mut written = 0usize;
    assert_eq!(
        unsafe {
            hc_sunrise(
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
            hc_sunset(
                c"naoj".as_ptr(),
                day,
                0.0,
                0.0,
                0.0,
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_UNKNOWN
    );
}

#[test]
fn the_rotation_is_erfas_through_an_out_parameter() {
    let mut degrees = 0f64;
    assert_eq!(
        unsafe { hc_earth_rotation_angle(1_192_406_400.0, &mut degrees) },
        HC_OK
    );
    assert!((degrees - 0.402_283_724_002_815_8_f64.to_degrees()).abs() < 1e-8);
    assert_eq!(
        unsafe { hc_gmst_iau1982(1_136_073_600.0, &mut degrees) },
        HC_OK
    );
    assert!((degrees - 1.754_174_981_860_675_f64.to_degrees()).abs() < 1e-8);
    assert_eq!(
        unsafe { hc_gmst_iau2006(1_136_073_600.0, &mut degrees) },
        HC_OK
    );
    assert!((degrees - 1.754_174_971_870_091_2_f64.to_degrees()).abs() < 1e-7);
    let mut seconds = 0f64;
    assert_eq!(
        unsafe { hc_ut2_minus_ut1(946_684_800.0 + 2_592.0, &mut seconds) },
        HC_OK
    );
    assert!((seconds + 0.005).abs() < 1e-6);
    assert_eq!(
        unsafe { hc_ut2_minus_ut1(f64::NAN, &mut seconds) },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_gmst_iau2006(0.0, core::ptr::null_mut()) },
        HC_ERROR_NULL_POINTER
    );
}

#[test]
fn the_solar_lines_are_the_modules() {
    // Tromsø in the polar night: no noon shadow for ʿaṣr.
    let mut midwinter = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2024, 12, 21, &mut midwinter) },
        HC_OK
    );
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_solar_event(
            c"asr-hanafi".as_ptr(),
            midwinter,
            69.6496,
            18.956,
            0.0,
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(line, format!("\tno-noon-shadow\t{midwinter}\t\t\n"));
    let line = read_lines(|buffer, capacity, written| unsafe {
        hc_solar_time(
            c"local-mean".as_ptr(),
            1_704_110_400,
            0.0,
            15.0,
            0.0,
            buffer,
            capacity,
            written,
        )
    });
    let cells: Vec<&str> = line.trim_end().split('\t').collect();
    let hours: f64 = cells[1].parse().expect("hours");
    assert!((hours - 13.0).abs() < 1e-6, "{line:?}");
    let mut written = 0usize;
    assert_eq!(
        unsafe {
            hc_solar_time(
                core::ptr::null(),
                0,
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
}

/// The USNO's "Complete Sun and Moon Data for One Day" for 31.78° N,
/// 35.24° E on 1 January 2024: moonrise at 21:47 and moonset at 10:15,
/// UT+2, so 19:47 and 08:15 UTC.
#[test]
fn the_moon_rises_and_sets_when_the_usno_says() {
    let mut day = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(2024, 1, 1, &mut day) },
        HC_OK
    );
    let rise = read_lines(|buffer, capacity, written| unsafe {
        hc_moonrise(
            c"usno".as_ptr(),
            day,
            31.78,
            35.24,
            740.0,
            buffer,
            capacity,
            written,
        )
    });
    let cells: Vec<&str> = rise.trim_end_matches('\n').split('\t').collect();
    assert_eq!(cells.len(), 3, "{rise}");
    let instant: i64 = cells[0].parse().expect("an instant");
    assert!((instant - 1_704_138_420).abs() <= 35, "{rise:?}");
    let set = read_lines(|buffer, capacity, written| unsafe {
        hc_moonset(
            c"usno".as_ptr(),
            day,
            31.78,
            35.24,
            740.0,
            buffer,
            capacity,
            written,
        )
    });
    let instant: i64 = set
        .split('\t')
        .next()
        .expect("a cell")
        .parse()
        .expect("an instant");
    assert!((instant - 1_704_096_900).abs() <= 35, "{set:?}");
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_moonrise(
                c"naoj".as_ptr(),
                day,
                0.0,
                0.0,
                0.0,
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_moonset(
                core::ptr::null(),
                day,
                0.0,
                0.0,
                0.0,
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_NULL_POINTER
    );
}
