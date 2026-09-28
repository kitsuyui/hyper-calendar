//! The time codes and clock readings, called as a C caller calls them.

use super::super::*;
use super::read_lines;

/// A library without `tz` has no zone to read summer time from.
#[cfg(not(feature = "tz"))]
#[test]
fn without_tz_a_zone_is_unknown() {
    let mut written = 0usize;
    assert_eq!(
        unsafe {
            hc_radio_encode(
                c"dcf77".as_ptr(),
                1_774_746_000,
                0,
                c"zone:Europe/Berlin".as_ptr(),
                0,
                0,
                0,
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_UNKNOWN
    );
}

/// CCSDS 301.0-B-4's example in CDS, `41 2A DE 03 B8 CE 73 01 C8`,
/// as `docs/systems/ccsds-time-codes.md` works it, and NICT's JJY
/// frame of 17:25 JST on 1 April 2004 (`nict-jjy-timecode`).
#[test]
fn the_codes_cross_the_c_boundary() {
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_ccsds_decode(c"412ade03b8ce7301c8".as_ptr(), 1, buffer, capacity, written)
    });
    assert!(text.starts_with("cds\t569524867\t"), "{text}");
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_ccsds_encode(946_684_832, 0, c"1c".as_ptr(), 1, buffer, capacity, written)
    });
    assert_eq!(text, "1c4effa220\n");
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_ccsds_ascii_format(
            946_684_832,
            0,
            c"a".as_ptr(),
            c"second".as_ptr(),
            1,
            1,
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(text, "2000-01-01T00:00:00Z\n");
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_ccsds_ascii_parse(c"2000-001T00:00Z".as_ptr(), 1, buffer, capacity, written)
    });
    assert!(
        text.starts_with("b\t946684832\t0\t946684800\t0\t0\tminute"),
        "{text}"
    );
    let frame = c"M01000101M000100111M000001001M001000010M000000100M100000000M";
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_radio_decode(
            c"jjy".as_ptr(),
            frame.as_ptr(),
            2000,
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(text, "1080807900\t731672\t17\t25\t9\t60\tnone\t\t\t\t\n");
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_radio_encode(
            c"jjy".as_ptr(),
            1_080_807_900,
            0,
            core::ptr::null(),
            0,
            0,
            0,
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(text.trim_end(), frame.to_str().expect("ASCII"));
    let mut written = 0;
    assert_eq!(
        unsafe {
            hc_radio_decode(
                core::ptr::null(),
                frame.as_ptr(),
                2000,
                core::ptr::null_mut(),
                0,
                &mut written,
            )
        },
        HC_ERROR_NULL_POINTER
    );
}
