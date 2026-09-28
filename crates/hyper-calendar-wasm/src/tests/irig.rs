//! The IRIG codes, called as a page calls them.

use super::super::*;
use super::read_lines;

/// Figure 5-2 of IRIG 200-16 (`rcc-200-16`): B124, 22 June 2003,
/// day 173, 21:18:42, both ways.
#[test]
fn figure_5_2_crosses_both_ways() {
    let frame = "M01000001M000101000M100000100M110001110M100000000M\
                 110000000M000000000M000000000M010011011M101010010M";
    let signal = "B124";
    let day = hc_gregorian_to_fixed(2003, 6, 22);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_irig_decode(
            signal.as_ptr(),
            signal.len(),
            frame.as_ptr(),
            frame.len(),
            2026,
            buffer,
            capacity,
        )
    });
    assert_eq!(text, format!("{day}\t173\t21\t18\t42\t0\t3\t0\t76722\n"));
    let text = read_lines(|buffer, capacity| unsafe {
        hc_irig_encode(
            signal.as_ptr(),
            signal.len(),
            day,
            76_722,
            0,
            0,
            buffer,
            capacity,
        )
    });
    assert_eq!(text.trim_end(), frame);
    let null = core::ptr::null_mut();
    let unknown = "B112";
    assert_eq!(
        unsafe {
            hc_irig_decode(
                unknown.as_ptr(),
                unknown.len(),
                frame.as_ptr(),
                frame.len(),
                2026,
                null,
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_irig_decode(
                signal.as_ptr(),
                signal.len(),
                frame.as_ptr(),
                10,
                2026,
                null,
                0,
            )
        },
        HC_ERR_MALFORMED
    );
}

/// The six formats, B's frame a second long and D's an hour.
#[test]
fn the_formats_are_listed() {
    let text = read_lines(|buffer, capacity| unsafe { hc_irig_formats(buffer, capacity) });
    assert_eq!(text.lines().count(), 6);
    assert!(text.contains(
        "B\t10000\t100\t1000000\tdays hours minutes seconds\t18\t0 1 2\t0 2 3 4 5\t0 1 2 3 4 5 6 7\n"
    ));
    assert!(text.contains("\nD\t60000000\t60\t3600000000\tdays hours\t9\t"));
}
