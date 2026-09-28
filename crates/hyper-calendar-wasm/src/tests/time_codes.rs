//! The time codes and clock readings, called as a page calls them.

use super::super::*;
use super::read_lines;

/// A module without `tz` has no zone to read summer time from.
#[cfg(not(feature = "tz"))]
#[test]
fn without_tz_a_zone_is_unknown() {
    let (code, summer) = ("dcf77", "zone:Europe/Berlin");
    assert_eq!(
        unsafe {
            hc_radio_encode(
                code.as_ptr(),
                code.len(),
                1_774_746_000,
                0,
                summer.as_ptr(),
                summer.len(),
                0,
                0,
                0,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
}

/// CCSDS 301.0-B-4's example, 1988-01-18T17:20:43.123456 UTC, in
/// CDS with a 16-bit day and microseconds, `41 2A DE 03 B8 CE 73 01
/// C8`, as `docs/systems/ccsds-time-codes.md` works it, and in ASCII
/// code B.
#[test]
fn the_standards_example_crosses_the_boundary() {
    let hex = "412ade03b8ce7301c8";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_ccsds_decode(hex.as_ptr(), hex.len(), 1, buffer, capacity)
    });
    assert_eq!(
        text,
        "cds\t569524867\t123456000000000000\t569524843\t0\t123456000000000000\n"
    );
    let p_field = "41";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_ccsds_encode(
            569_524_867,
            123_456_000_000_000_000,
            p_field.as_ptr(),
            p_field.len(),
            1,
            buffer,
            capacity,
        )
    });
    assert_eq!(text.trim_end(), hex);
    let (variation, precision) = ("b", "6");
    let text = read_lines(|buffer, capacity| unsafe {
        hc_ccsds_ascii_format(
            569_524_867,
            123_456_000_000_000_000,
            variation.as_ptr(),
            variation.len(),
            precision.as_ptr(),
            precision.len(),
            1,
            1,
            buffer,
            capacity,
        )
    });
    assert_eq!(text, "1988-018T17:20:43.123456Z\n");
    let text = read_lines(|buffer, capacity| unsafe {
        hc_ccsds_ascii_parse(text.as_ptr(), text.len() - 1, 1, buffer, capacity)
    });
    assert!(text.starts_with("b\t569524867\t"), "{text}");
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_ccsds_decode("4".as_ptr(), 1, 1, null, 0) },
        HC_ERR_MALFORMED
    );
    assert_eq!(
        unsafe { hc_ccsds_decode(core::ptr::null(), 2, 1, null, 0) },
        HC_ERR_NULL_POINTER
    );
}

/// NICT's frame of 17:25 JST on 1 April 2004 (`nict-jjy-timecode`),
/// both ways.
#[test]
fn a_jjy_frame_crosses_the_boundary() {
    let (code, frame) = (
        "jjy",
        "M01000101M000100111M000001001M001000010M000000100M100000000M",
    );
    let text = read_lines(|buffer, capacity| unsafe {
        hc_radio_decode(
            code.as_ptr(),
            code.len(),
            frame.as_ptr(),
            frame.len(),
            2000,
            buffer,
            capacity,
        )
    });
    assert_eq!(text, "1080807900\t731672\t17\t25\t9\t60\tnone\t\t\t\t\n");
    let text = read_lines(|buffer, capacity| unsafe {
        hc_radio_encode(
            code.as_ptr(),
            code.len(),
            1_080_807_900,
            0,
            core::ptr::null(),
            0,
            0,
            0,
            0,
            buffer,
            capacity,
        )
    });
    assert_eq!(text.trim_end(), frame);
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe {
            hc_radio_decode(
                code.as_ptr(),
                code.len(),
                frame.as_ptr(),
                frame.len(),
                2050,
                null,
                0,
            )
        },
        HC_ERR_OUT_OF_RANGE
    );
}
