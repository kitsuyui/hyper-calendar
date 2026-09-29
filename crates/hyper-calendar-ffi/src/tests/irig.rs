//! The IRIG codes and the plum rains, across the C boundary.

use super::super::*;
use super::{measured, read_lines};

/// Figure 5-2 of IRIG 200-16 (`rcc-200-16`): B124, 22 June 2003,
/// day 173, 21:18:42, both ways.
#[test]
fn figure_5_2_crosses_the_c_boundary_both_ways() {
    let frame = c"M01000001M000101000M100000100M110001110M100000000M110000000M000000000M000000000M010011011M101010010M";
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_irig_decode(
            c"B124".as_ptr(),
            frame.as_ptr(),
            2026,
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(text, "731388\t173\t21\t18\t42\t0\t3\t0\t76722\n");
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_irig_encode(
            c"B124".as_ptr(),
            731_388,
            76_722,
            0,
            0,
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(text.trim_end(), frame.to_str().expect("ASCII"));
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_irig_decode(
                core::ptr::null(),
                frame.as_ptr(),
                2026,
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_NULL_POINTER
    );
}

/// The six formats, the WebAssembly module's lines.
#[test]
fn the_formats_are_listed() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_irig_formats(buffer, capacity, written)
    });
    assert_eq!(text.lines().count(), 6);
    assert!(text.starts_with("A\t1000\t100\t100000\tdays hours minutes seconds tenths\t18\t"));
}

/// Table 3-2: A's frame is a tenth of a second.
#[test]
fn a_reading_rounds_down_to_its_frame() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_irig_frame_start(c"A000".as_ptr(), 3_725, 57, buffer, capacity, written)
    });
    assert_eq!(text, "3725\t50\t100000\n");
}
