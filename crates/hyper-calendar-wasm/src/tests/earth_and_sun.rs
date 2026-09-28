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
