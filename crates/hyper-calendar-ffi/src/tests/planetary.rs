use super::super::*;
use super::{measured, read_lines};

fn row(text: &str) -> Vec<String> {
    text.trim_end().split('\t').map(str::to_owned).collect()
}

/// Gangale's Titan calibration: 2002-12-18 10:42 UTC was
/// 209 Aries 13, Julian Circad 144 096.
#[test]
fn a_circad_date_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_circad_date(
            c"darian-titan".as_ptr(),
            1_040_208_120.0,
            buffer,
            capacity,
            written,
        )
    });
    let cells = row(&text);
    assert_eq!(cells.len(), 10);
    assert_eq!(
        cells[..7],
        ["darian-titan", "209", "9", "13", "Aries", "Solis", "144096"]
    );
    let mut written = 0usize;
    assert_eq!(
        unsafe {
            hc_circad_date(
                core::ptr::null(),
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
            hc_circad_date(
                c"darian".as_ptr(),
                0.0,
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_UNKNOWN
    );
    // 2103, past 100 Julian years from J2000.0, and an instant not finite.
    for unix_seconds in [4_200_000_000.0, f64::NAN] {
        assert_eq!(
            measured(|buffer, capacity, written| unsafe {
                hc_circad_date(
                    c"darian-titan".as_ptr(),
                    unix_seconds,
                    buffer,
                    capacity,
                    written,
                )
            }),
            HC_ERROR_OUT_OF_RANGE,
            "{unix_seconds}"
        );
    }
}

#[test]
fn mars_time_and_a_mission_sol_cross_the_boundary() {
    // Mars24's worked example A: 2000-01-06T00:00:00Z.
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_mars_time(947_116_800.0, 0.0, buffer, capacity, written)
    });
    let cells = row(&text);
    assert_eq!(cells.len(), 16, "{cells:?}");
    assert_eq!(cells[1], "23:59:39");
    assert_eq!(cells[5], "23:38:54");
    let mut sol = -1i64;
    assert_eq!(
        unsafe { hc_mission_sol(c"curiosity".as_ptr(), 1_344_230_277.0, &mut sol) },
        HC_OK
    );
    assert_eq!(sol, 0);
    assert_eq!(
        unsafe { hc_mission_sol(c"Viking 1".as_ptr(), 1_344_230_277.0, &mut sol) },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe { hc_mission_sol(c"zhurong".as_ptr(), 1_700_000_000.0, &mut sol) },
        HC_ERROR_NO_DATA
    );
    assert_eq!(
        unsafe { hc_mission_sol(c"beagle-2".as_ptr(), 1_700_000_000.0, &mut sol) },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe { hc_mission_sol(core::ptr::null(), 0.0, &mut sol) },
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        unsafe { hc_mission_sol(c"curiosity".as_ptr(), 0.0, core::ptr::null_mut()) },
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        unsafe {
            hc_mars_time(
                f64::NAN,
                0.0,
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
}

#[test]
fn the_tables_and_a_body_clock_are_the_modules_lines() {
    let missions =
        read_lines(|buffer, capacity, written| unsafe { hc_missions(buffer, capacity, written) });
    assert_eq!(missions, hc::planetary_lines::missions_lines());
    let bodies =
        read_lines(|buffer, capacity, written| unsafe { hc_bodies(buffer, capacity, written) });
    assert_eq!(bodies, hc::planetary_lines::bodies_lines());
    let titan = read_lines(|buffer, capacity, written| unsafe {
        hc_body_time(
            c"titan".as_ptr(),
            947_116_800.0,
            0.0,
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(row(&titan).len(), 8, "{titan:?}");
    assert_eq!(
        unsafe {
            hc_body_time(
                c"sun".as_ptr(),
                0.0,
                0.0,
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_NO_DATA
    );
    let body = |name: *const c_char, unix_seconds: f64, east_longitude: f64| {
        measured(|buffer, capacity, written| unsafe {
            hc_body_time(
                name,
                unix_seconds,
                east_longitude,
                buffer,
                capacity,
                written,
            )
        })
    };
    let titan = c"titan".as_ptr();
    assert_eq!(body(c"vulcan".as_ptr(), 0.0, 0.0), HC_ERROR_UNKNOWN);
    assert_eq!(body(core::ptr::null(), 0.0, 0.0), HC_ERROR_NULL_POINTER);
    assert_eq!(body(titan, f64::NAN, 0.0), HC_ERROR_OUT_OF_RANGE);
    assert_eq!(body(titan, 0.0, f64::INFINITY), HC_ERROR_OUT_OF_RANGE);
    // 2103, past 100 Julian years from J2000.0.
    assert_eq!(body(titan, 4_200_000_000.0, 0.0), HC_ERROR_OUT_OF_RANGE);
}
