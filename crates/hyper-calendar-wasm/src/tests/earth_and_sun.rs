use super::super::*;
use super::read_lines;

/// Warren's row for 29 February 1992, 03:15:56.2, and 12h 56m 27.4s,
/// +42° 10′ 17″ (J2000), in the IDL Astronomy Library's `helio_jd`:
/// HJD − JD is 350.9 s.
#[test]
fn hjd_tt_is_the_idl_tables_correction() {
    let date = hc_gregorian_to_fixed(1992, 2, 29) as f64
        + 1_721_424.5
        + (3.0 * 3_600.0 + 15.0 * 60.0 + 56.2) / 86_400.0;
    let alpha = 15.0 * (12.0 + 56.0 / 60.0 + 27.4 / 3_600.0);
    let delta = 42.0 + 10.0 / 60.0 + 17.0 / 3_600.0;
    let text =
        read_lines(|buffer, capacity| unsafe { hc_hjd_tt(date, alpha, delta, buffer, capacity) });
    let cells: Vec<f64> = text
        .trim_end()
        .split('\t')
        .map(|cell| cell.parse().expect("a number"))
        .collect();
    assert_eq!(cells.len(), 2);
    assert!((cells[1] - 350.9).abs() < 0.1, "{text}");
    let text = read_lines(|buffer, capacity| unsafe {
        hc_hjd_utc(date, alpha, delta, 1, buffer, capacity)
    });
    // TAI − UTC was 26 s in February 1992.
    assert!(text.trim_end().ends_with("\t58.184"), "{text}");
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_hjd_tt(f64::INFINITY, alpha, delta, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_hjd_utc(2_433_282.5, alpha, delta, 1, null, 0) },
        HC_ERR_NO_DATA
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
            unsafe { hc_hjd_tt(date, alpha, delta, null, 0) },
            HC_ERR_OUT_OF_RANGE,
            "{date} {delta}"
        );
        assert_eq!(
            unsafe { hc_hjd_utc(date, alpha, delta, 0, null, 0) },
            HC_ERR_OUT_OF_RANGE,
            "{date} {delta}"
        );
    }
}

fn number(call: impl Fn(*mut u8, usize) -> i64) -> f64 {
    read_lines(call).trim_end().parse().expect("one number")
}

/// ERFA's test values, which `hc-astro`'s own tests read:
/// `eraEra00` at JD 2 454 388.5 UT1 and `eraGmst82` at MJD 53 736.
#[test]
fn the_rotation_is_erfas_and_refuses_off_the_era() {
    let era = number(|buffer, capacity| unsafe {
        hc_earth_rotation_angle(1_192_406_400.0, buffer, capacity)
    });
    assert!(
        (era - 0.402_283_724_002_815_8_f64.to_degrees()).abs() < 1e-8,
        "{era}"
    );
    let gmst =
        number(|buffer, capacity| unsafe { hc_gmst_iau1982(1_136_073_600.0, buffer, capacity) });
    assert!(
        (gmst - 1.754_174_981_860_675_f64.to_degrees()).abs() < 1e-8,
        "{gmst}"
    );
    let gmst06 =
        number(|buffer, capacity| unsafe { hc_gmst_iau2006(1_136_073_600.0, buffer, capacity) });
    assert!(
        (gmst06 - 1.754_174_971_870_091_2_f64.to_degrees()).abs() < 1e-7,
        "{gmst06}"
    );
    let ut2 = number(|buffer, capacity| unsafe {
        hc_ut2_minus_ut1(946_684_800.0 + 0.03 * 86_400.0, buffer, capacity)
    });
    assert!((ut2 + 0.005).abs() < 1e-6, "{ut2}");
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_earth_rotation_angle(f64::NAN, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_gmst_iau2006(1e15, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

/// NAOJ: sunrise in Tokyo on 2024-01-01 at 06:50 JST, the temporal
/// hour 6; and Tromsø's polar night, which has no temporal hour.
#[test]
fn a_clock_reads_or_names_what_it_misses() {
    let clock = "temporal";
    let sunrise = hc_unix_from_fixed(hc_gregorian_to_fixed(2023, 12, 31)) + 21 * 3_600 + 50 * 60;
    let text = read_lines(|buffer, capacity| unsafe {
        hc_solar_time(
            clock.as_ptr(),
            clock.len(),
            sunrise,
            35.6581,
            139.7414,
            0.0,
            buffer,
            capacity,
        )
    });
    let cells: Vec<&str> = text.trim_end_matches('\n').split('\t').collect();
    assert_eq!(cells.len(), 5);
    assert_eq!(cells[0], hc_gregorian_to_fixed(2024, 1, 1).to_string());
    let hours: f64 = cells[1].parse().expect("hours");
    assert!((hours - 6.0).abs() < 0.03, "{hours}");
    assert_eq!(cells[2..], ["", "", ""]);
    let midwinter = hc_gregorian_to_fixed(2024, 12, 21);
    let event = "asr-hanafi";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_solar_event(
            event.as_ptr(),
            event.len(),
            midwinter,
            69.6496,
            18.956,
            0.0,
            buffer,
            capacity,
        )
    });
    assert_eq!(text, format!("\tno-noon-shadow\t{midwinter}\t\t\n"));
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_solar_event("dhuhr".as_ptr(), 5, midwinter, 0.0, 0.0, 0.0, null, 0) },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_solar_time(
                clock.as_ptr(),
                clock.len(),
                sunrise,
                0.0,
                181.0,
                0.0,
                null,
                0,
            )
        },
        HC_ERR_OUT_OF_RANGE
    );
}
/// The USNO's minutes for Jerusalem on 2024-01-01, sunrise 06:39 and
/// sunset 16:46 at UT+2 (`usno-api-rstt`), under its own horizon;
/// the horizons listed; and a missing sunrise named.
#[test]
fn the_horizons_are_listed_and_each_rises_and_sets() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_horizons("fr-CA".as_ptr(), 5, buffer, capacity)
    });
    let ids: Vec<&str> = text
        .lines()
        .map(|line| line.split('\t').next().expect("an id"))
        .collect();
    assert_eq!(ids, ["geometric-dip", "usno", "calendrical-calculations"]);
    assert!(text.lines().all(|line| line.split('\t').count() == 7));
    // The IMCCE's name for the USNO (`imcce-promenade-usno`); the
    // other two in English.
    assert!(
        text.contains("\tObservatoire naval de Washington D.C.\tfr\n"),
        "{text}"
    );
    assert_eq!(
        text.lines().filter(|line| line.ends_with("\ten")).count(),
        2
    );
    assert!(text.lines().any(|line| line.contains("\tUSNO\t")));
    let day = hc_gregorian_to_fixed(2024, 1, 1);
    let midnight = hc_unix_from_fixed(day);
    let horizon = "USNO";
    for (export, published) in [
        (
            hc_sunrise as unsafe extern "C" fn(_, _, _, _, _, _, _, _) -> i64,
            4 * 3_600 + 39 * 60,
        ),
        (hc_sunset, 14 * 3_600 + 46 * 60),
    ] {
        let text = read_lines(|buffer, capacity| unsafe {
            export(
                horizon.as_ptr(),
                horizon.len(),
                day,
                31.78,
                35.24,
                740.0,
                buffer,
                capacity,
            )
        });
        let cells: Vec<&str> = text.trim_end_matches('\n').split('\t').collect();
        assert_eq!(cells.len(), 5);
        let instant: i64 = cells[0].parse().expect("an instant");
        assert!((instant - midnight - published).abs() <= 31, "{text}");
    }
    let midwinter = hc_gregorian_to_fixed(2024, 12, 21);
    let horizon = "geometric-dip";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_sunrise(
            horizon.as_ptr(),
            horizon.len(),
            midwinter,
            69.6496,
            18.956,
            0.0,
            buffer,
            capacity,
        )
    });
    assert!(
        text.starts_with(&format!("\tsunrise\t{midwinter}\t\t")),
        "{text}"
    );
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_sunset("naoj".as_ptr(), 4, day, 0.0, 0.0, 0.0, null, 0) },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe { hc_sunset(core::ptr::null(), 0, day, 0.0, 0.0, 0.0, null, 0) },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_sunrise(
                horizon.as_ptr(),
                horizon.len(),
                day,
                91.0,
                0.0,
                0.0,
                null,
                0,
            )
        },
        HC_ERR_OUT_OF_RANGE
    );
}

/// The USNO's "Complete Sun and Moon Data for One Day" for 31.78° N,
/// 35.24° E on 1 January 2024: moonrise at 21:47 and moonset at 10:15,
/// UT+2, so 19:47 and 08:15 UTC; the horizon reproduces each within the
/// published minute.
#[test]
fn the_moon_rises_and_sets_when_the_usno_says() {
    let day = hc_gregorian_to_fixed(2024, 1, 1);
    let horizon = "usno";
    let crossing = |rise: bool| {
        let text = read_lines(|buffer, capacity| unsafe {
            if rise {
                hc_moonrise(
                    horizon.as_ptr(),
                    horizon.len(),
                    day,
                    31.78,
                    35.24,
                    740.0,
                    buffer,
                    capacity,
                )
            } else {
                hc_moonset(
                    horizon.as_ptr(),
                    horizon.len(),
                    day,
                    31.78,
                    35.24,
                    740.0,
                    buffer,
                    capacity,
                )
            }
        });
        let cells: Vec<String> = text
            .trim_end_matches('\n')
            .split('\t')
            .map(str::to_owned)
            .collect();
        assert_eq!(cells.len(), 3, "{text}");
        cells
    };
    let rise = crossing(true);
    let instant: i64 = rise[0].parse().expect("an instant");
    assert!((instant - 1_704_138_420).abs() <= 35, "{rise:?}");
    assert_eq!(rise[1..], ["", ""]);
    let set = crossing(false);
    let instant: i64 = set[0].parse().expect("an instant");
    assert!((instant - 1_704_096_900).abs() <= 35, "{set:?}");
    // The Moon skips a local day about once a month: of January 2024 at
    // Jerusalem, exactly one day has no moonrise, written by name.
    let mut missing = Vec::new();
    for offset in 0..31 {
        let text = read_lines(|buffer, capacity| unsafe {
            hc_moonrise(
                horizon.as_ptr(),
                horizon.len(),
                day + offset,
                31.78,
                35.24,
                740.0,
                buffer,
                capacity,
            )
        });
        if text.starts_with('\t') {
            assert_eq!(text, format!("\tmoonrise\t{}\n", day + offset));
            missing.push(offset);
        }
    }
    assert_eq!(missing.len(), 1, "{missing:?}");
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_moonrise("naoj".as_ptr(), 4, day, 0.0, 0.0, 0.0, null, 0) },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_moonset(
                horizon.as_ptr(),
                horizon.len(),
                day,
                91.0,
                0.0,
                0.0,
                null,
                0,
            )
        },
        HC_ERR_OUT_OF_RANGE
    );
}

/// The test case in the header of the IERS routine `RG_ZONT2.F`: at
/// MJD 54 465, 2008-01-01 00:00, the whole table gives δUT1 = 0.079 832 9 s,
/// so UT1S − UT1 is its negative; the 41 short tides are UT1R's part of it.
#[test]
fn the_regularised_ut1_lines_are_the_iers_test_case() {
    let ut1 = (54_465.0 - 40_587.0) * 86_400.0;
    let effect = |limit: f64| {
        number(|buffer, capacity| unsafe { hc_zonal_tide_ut1_effect(ut1, limit, buffer, capacity) })
    };
    let all = effect(f64::INFINITY);
    assert!((all - 0.079_832_876_785_765_57).abs() < 1e-6, "{all}");
    let short = effect(35.0);
    assert!(short.abs() < 2.75e-3 && short != all, "{short}");
    let cells = |text: String| -> Vec<f64> {
        text.trim_end()
            .split('\t')
            .map(|cell| cell.parse().expect("a number"))
            .collect()
    };
    let ut1s = cells(read_lines(|buffer, capacity| unsafe {
        hc_ut1s_iers2010(ut1, buffer, capacity)
    }));
    assert_eq!(ut1s.len(), 2);
    assert!((ut1s[0] + all).abs() < 1e-9, "{ut1s:?}");
    assert!((ut1s[1] - (ut1 + ut1s[0])).abs() < 1e-6, "{ut1s:?}");
    let ut1r = cells(read_lines(|buffer, capacity| unsafe {
        hc_ut1r_iers2010(ut1, buffer, capacity)
    }));
    assert!((ut1r[0] + short).abs() < 1e-9, "{ut1r:?}");
    assert!((ut1r[1] - (ut1 + ut1r[0])).abs() < 1e-6, "{ut1r:?}");
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_ut1r_iers2010(f64::NAN, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_ut1s_iers2010(1e15, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
    for limit in [0.0, -35.0, f64::NAN] {
        assert_eq!(
            unsafe { hc_zonal_tide_ut1_effect(ut1, limit, null, 0) },
            HC_ERR_OUT_OF_RANGE,
            "{limit}"
        );
    }
}

/// Meeus, example 28.a: on 1992 October 13 at 0h the equation of time is
/// +13 min 42.6 s, so the sundial at Greenwich is that far ahead of the
/// clock and the Sun transits at 11:46:17 UT; the lower transit opening
/// the day is half a day earlier, and 15° east brings the noon an hour
/// sooner.
#[test]
fn the_equation_of_time_and_the_transits_are_meeuss() {
    let day = hc_gregorian_to_fixed(1992, 10, 13);
    let midnight_utc = hc_unix_from_fixed(day);
    let equation =
        number(|buffer, capacity| unsafe { hc_equation_of_time(midnight_utc, buffer, capacity) });
    assert!((equation - 822.6).abs() < 1.0, "{equation}");
    // The equation grows some twenty seconds a day in October, so the noon
    // transit is the mean noon less the equation read at that noon.
    let noon = hc_solar_noon(day, 51.4769, 0.0, 0.0);
    assert!(
        (noon - (midnight_utc + 12 * 3_600 - 832)).abs() <= 4,
        "{noon}"
    );
    let at_noon = number(|buffer, capacity| unsafe { hc_equation_of_time(noon, buffer, capacity) });
    assert!(
        (noon - (midnight_utc + 12 * 3_600 - at_noon as i64)).abs() <= 2,
        "{noon}"
    );
    let midnight = hc_solar_midnight(day, 51.4769, 0.0, 0.0);
    assert!((noon - midnight - 43_200).abs() <= 1, "{midnight}");
    let east = hc_solar_noon(day, 51.4769, 15.0, 0.0);
    assert!((noon - east - 3_600).abs() <= 3, "{east}");
    assert_eq!(hc_solar_noon(day, 91.0, 0.0, 0.0), HC_ERR_OUT_OF_RANGE);
    let far = hc_gregorian_to_fixed(3001, 1, 1);
    assert_eq!(hc_solar_midnight(far, 0.0, 0.0, 0.0), HC_ERR_OUT_OF_RANGE);
    assert_eq!(
        unsafe { hc_equation_of_time(i64::MIN, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

/// At Jerusalem on 1 January 2024 the three dawns come in order before the
/// USNO's sunrise and the three dusks after its sunset; at Helsinki at
/// midsummer, where the Sun stays within 6.4° of the horizon, civil
/// twilight ends and nautical twilight does not, which the line names.
#[test]
fn the_twilights_cross_the_boundary_and_name_a_missing_depression() {
    let crossing = |export: unsafe extern "C" fn(
        *const u8,
        usize,
        i64,
        f64,
        f64,
        f64,
        *mut u8,
        usize,
    ) -> i64,
                    twilight: &str,
                    day: i64,
                    place: [f64; 3]| {
        let text = read_lines(|buffer, capacity| unsafe {
            export(
                twilight.as_ptr(),
                twilight.len(),
                day,
                place[0],
                place[1],
                place[2],
                buffer,
                capacity,
            )
        });
        let line = text.strip_suffix('\n').expect("one line");
        let cells: Vec<String> = line.split('\t').map(str::to_owned).collect();
        assert_eq!(cells.len(), 5, "{text:?}");
        cells
    };
    let day = hc_gregorian_to_fixed(2024, 1, 1);
    let jerusalem = [31.78, 35.24, 740.0];
    let instant = |cells: &[String]| -> i64 { cells[0].parse().expect("an instant") };
    let dawns: Vec<i64> = ["astronomical", "Nautical", "civil"]
        .iter()
        .map(|twilight| instant(&crossing(hc_dawn, twilight, day, jerusalem)))
        .collect();
    let dusks: Vec<i64> = ["civil", "nautical", "ASTRONOMICAL"]
        .iter()
        .map(|twilight| instant(&crossing(hc_dusk, twilight, day, jerusalem)))
        .collect();
    // The USNO: sunrise 06:39 and sunset 16:46 at UT+2.
    let sunrise = hc_unix_from_fixed(day) + 4 * 3_600 + 39 * 60;
    let sunset = hc_unix_from_fixed(day) + 14 * 3_600 + 46 * 60;
    assert!(
        dawns[0] < dawns[1] && dawns[1] < dawns[2] && dawns[2] < sunrise,
        "{dawns:?}"
    );
    assert!(
        sunset < dusks[0] && dusks[0] < dusks[1] && dusks[1] < dusks[2],
        "{dusks:?}"
    );
    // Civil twilight at Jerusalem lasts about half an hour.
    assert!(
        (sunrise - dawns[2]) > 20 * 60 && (sunrise - dawns[2]) < 40 * 60,
        "{dawns:?}"
    );
    let midsummer = hc_gregorian_to_fixed(2024, 6, 21);
    let helsinki = [60.1699, 24.9384, 0.0];
    let civil = crossing(hc_dusk, "civil", midsummer, helsinki);
    assert!(
        !civil[0].is_empty() && civil[1..].iter().all(String::is_empty),
        "{civil:?}"
    );
    let nautical = crossing(hc_dusk, "nautical", midsummer, helsinki);
    assert_eq!(
        nautical,
        ["", "depression", &midsummer.to_string(), "720", "43200"]
    );
    let dawn = crossing(hc_dawn, "nautical", midsummer, helsinki);
    assert_eq!(dawn[1..], nautical[1..]);
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_dawn("dusk".as_ptr(), 4, day, 0.0, 0.0, 0.0, null, 0) },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe { hc_dusk("civil".as_ptr(), 5, day, 91.0, 0.0, 0.0, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_dusk("".as_ptr(), 0, day, 0.0, 0.0, 0.0, null, 0) },
        HC_ERR_UNKNOWN
    );
}
