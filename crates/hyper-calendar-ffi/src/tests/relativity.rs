use super::super::*;
use super::{measured, read_lines};

fn row(text: &str) -> Vec<String> {
    text.trim_end().split('\t').map(str::to_owned).collect()
}

#[test]
fn dilation_crosses_the_boundary() {
    let moving = read_lines(|buffer, capacity, written| unsafe {
        hc_proper_time(0.6 * 299_792_458.0, 10.0, buffer, capacity, written)
    });
    let cells = row(&moving);
    assert_eq!(cells.len(), 7, "{cells:?}");
    assert_eq!(cells[5], "SPEED_OF_LIGHT");
    let still = read_lines(|buffer, capacity, written| unsafe {
        hc_gravitational_dilation(c"earth".as_ptr(), 6_378_137.0, buffer, capacity, written)
    });
    assert_eq!(row(&still)[2], "GM_EARTH");
    let listed = read_lines(|buffer, capacity, written| unsafe {
        hc_gravitating_bodies(buffer, capacity, written)
    });
    assert_eq!(listed, hc::relativity_lines::gravitating_bodies_lines());
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_proper_time(3e8, 1.0, null, 0, core::ptr::null_mut()) },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe {
            hc_gravitational_dilation(c"vulcan".as_ptr(), 1e7, null, 0, core::ptr::null_mut())
        },
        HC_ERROR_UNKNOWN
    );
    // The Earth's Schwarzschild radius is about 8.87 mm.
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_gravitational_dilation(c"earth".as_ptr(), 0.001, buffer, capacity, written)
        }),
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_gravitational_dilation(core::ptr::null(), 1e7, buffer, capacity, written)
        }),
        HC_ERROR_NULL_POINTER
    );
}

#[test]
fn hc_orbit_rate_offset_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_orbit_rate_offset(
            c"earth".as_ptr(),
            26561750.0,
            6378137.0,
            buffer,
            capacity,
            written,
        )
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 10),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "earth");
    assert_eq!(first[2], "GM_EARTH");
    assert_eq!(
        unsafe {
            hc_orbit_rate_offset(
                c"vulcan".as_ptr(),
                26561750.0,
                6378137.0,
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_orbit_rate_offset(
                c"earth".as_ptr(),
                0.0,
                6378137.0,
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
}

#[test]
fn hc_rocket_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_rocket(9.80665, 31557600.0, buffer, capacity, written)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 10),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[8], "SPEED_OF_LIGHT;LIGHT_YEAR");
    assert_eq!(
        unsafe { hc_rocket(0.0, 1.0, core::ptr::null_mut(), 0, core::ptr::null_mut()) },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe {
            hc_rocket(
                9.80665,
                -1.0,
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
}

#[test]
fn hc_flip_and_burn_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_flip_and_burn(9.80665, 2.365182618145e+22, buffer, capacity, written)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 11),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[9], "SPEED_OF_LIGHT;JULIAN_YEAR_SECONDS");
    assert_eq!(
        unsafe { hc_flip_and_burn(0.0, 1e+16, core::ptr::null_mut(), 0, core::ptr::null_mut()) },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe {
            hc_flip_and_burn(
                9.80665,
                -1.0,
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
}

#[test]
fn hc_doppler_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_doppler(0.6, 1.0, buffer, capacity, written)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 7),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[2], "2");
    assert_eq!(
        unsafe { hc_doppler(0.6, 1.5, core::ptr::null_mut(), 0, core::ptr::null_mut()) },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_doppler(1.0, 0.0, core::ptr::null_mut(), 0, core::ptr::null_mut()) },
        HC_ERROR_OUT_OF_RANGE
    );
}

#[test]
fn hc_velocity_add_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_velocity_add(0.999, 0.999, buffer, capacity, written)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 10),
        "{text}"
    );
    assert_eq!(
        unsafe { hc_velocity_add(1.0, 0.5, core::ptr::null_mut(), 0, core::ptr::null_mut()) },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe {
            hc_velocity_add(
                0.5,
                f64::NAN,
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
}

#[test]
fn hc_schwarzschild_radius_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_schwarzschild_radius(c"sun".as_ptr(), buffer, capacity, written)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 6),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "sun");
    assert_eq!(first[2], "GM_SUN");
    assert_eq!(
        unsafe {
            hc_schwarzschild_radius(
                c"Sagittarius A*".as_ptr(),
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_UNKNOWN
    );
}

#[test]
fn hc_proper_time_uncertain_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_proper_time_uncertain(179875474.8, 1000.0, 1000000000.0, buffer, capacity, written)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 7),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[5], "SPEED_OF_LIGHT");
    assert_eq!(
        unsafe {
            hc_proper_time_uncertain(
                179875474.8,
                -1.0,
                1000000000.0,
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe {
            hc_proper_time_uncertain(
                299792458.0,
                1.0,
                1000000000.0,
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_OUT_OF_RANGE
    );
}
