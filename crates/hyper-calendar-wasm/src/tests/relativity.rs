use super::super::*;
use super::read_lines;

fn row(text: &str) -> Vec<String> {
    text.trim_end().split('\t').map(str::to_owned).collect()
}

fn number(cell: &str) -> f64 {
    cell.parse()
        .unwrap_or_else(|_| panic!("not a number: {cell}"))
}

#[test]
fn six_tenths_of_c_is_five_quarters() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_proper_time(0.6 * 299_792_458.0, 10.0, buffer, capacity)
    });
    let cells = row(&text);
    assert_eq!(cells.len(), 7, "{cells:?}");
    assert!((number(&cells[1]) - 1.25).abs() < 1e-12, "{cells:?}");
    assert!((number(&cells[2]) - 8.0).abs() < 1e-9, "{cells:?}");
    assert_eq!(cells[5], "SPEED_OF_LIGHT");
    assert_eq!(
        unsafe { hc_proper_time(299_792_458.0, 1.0, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

#[test]
fn a_clock_at_a_radius_names_the_constant_it_used() {
    let earth = "earth";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_gravitational_dilation(earth.as_ptr(), earth.len(), 6_378_137.0, buffer, capacity)
    });
    let cells = row(&text);
    assert_eq!(cells.len(), 8, "{cells:?}");
    assert_eq!(cells[0], "earth");
    assert_eq!(cells[2], "GM_EARTH");
    assert!(number(&cells[4]) < 1.0);
    assert!(number(&cells[5]) < 0.0);
    let listed = read_lines(|buffer, capacity| unsafe { hc_gravitating_bodies(buffer, capacity) });
    assert_eq!(listed.lines().count(), 6);
    let sun = "sun";
    assert_eq!(
        unsafe {
            hc_gravitational_dilation(sun.as_ptr(), sun.len(), 1_000.0, core::ptr::null_mut(), 0)
        },
        HC_ERR_OUT_OF_RANGE
    );
    let vulcan = "vulcan";
    assert_eq!(
        unsafe {
            hc_gravitational_dilation(vulcan.as_ptr(), vulcan.len(), 1e7, core::ptr::null_mut(), 0)
        },
        HC_ERR_UNKNOWN
    );
}

#[test]
fn hc_orbit_rate_offset_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_orbit_rate_offset(
            "earth".as_ptr(),
            "earth".len(),
            26561750.0,
            6378137.0,
            buffer,
            capacity,
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
                "vulcan".as_ptr(),
                "vulcan".len(),
                26561750.0,
                6378137.0,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_orbit_rate_offset(
                "earth".as_ptr(),
                "earth".len(),
                0.0,
                6378137.0,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_OUT_OF_RANGE
    );
}

#[test]
fn hc_rocket_crosses_the_boundary() {
    let text =
        read_lines(|buffer, capacity| unsafe { hc_rocket(9.80665, 31557600.0, buffer, capacity) });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 10),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[8], "SPEED_OF_LIGHT;LIGHT_YEAR");
    assert_eq!(
        unsafe { hc_rocket(0.0, 1.0, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_rocket(9.80665, -1.0, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

#[test]
fn hc_flip_and_burn_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_flip_and_burn(9.80665, 2.365182618145e+22, buffer, capacity)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 11),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[9], "SPEED_OF_LIGHT;JULIAN_YEAR_SECONDS");
    assert_eq!(
        unsafe { hc_flip_and_burn(0.0, 1e+16, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_flip_and_burn(9.80665, -1.0, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

#[test]
fn hc_doppler_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe { hc_doppler(0.6, 1.0, buffer, capacity) });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 7),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[2], "2");
    assert_eq!(
        unsafe { hc_doppler(0.6, 1.5, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_doppler(1.0, 0.0, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

#[test]
fn hc_velocity_add_crosses_the_boundary() {
    let text =
        read_lines(|buffer, capacity| unsafe { hc_velocity_add(0.999, 0.999, buffer, capacity) });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 10),
        "{text}"
    );
    assert_eq!(
        unsafe { hc_velocity_add(1.0, 0.5, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_velocity_add(0.5, f64::NAN, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

#[test]
fn hc_schwarzschild_radius_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_schwarzschild_radius("sun".as_ptr(), "sun".len(), buffer, capacity)
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
                "Sagittarius A*".as_ptr(),
                "Sagittarius A*".len(),
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
}

#[test]
fn hc_proper_time_uncertain_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_proper_time_uncertain(179875474.8, 1000.0, 1000000000.0, buffer, capacity)
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
            hc_proper_time_uncertain(179875474.8, -1.0, 1000000000.0, core::ptr::null_mut(), 0)
        },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe {
            hc_proper_time_uncertain(299792458.0, 1.0, 1000000000.0, core::ptr::null_mut(), 0)
        },
        HC_ERR_OUT_OF_RANGE
    );
}
