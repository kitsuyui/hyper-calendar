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
