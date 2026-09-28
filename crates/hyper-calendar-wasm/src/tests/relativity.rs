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
