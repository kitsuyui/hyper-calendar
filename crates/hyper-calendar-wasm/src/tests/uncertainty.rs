use super::super::*;
use super::read_lines;

fn row(text: &str) -> Vec<String> {
    text.trim_end().split('\t').map(str::to_owned).collect()
}

#[test]
fn hc_edtf_parse_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_edtf_parse("1984".as_ptr(), "1984".len(), buffer, capacity)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 11),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "value");
    assert_eq!(first[1], "date");
    assert_eq!(first[2], "1984");
    assert_eq!(first[3], "year");
    assert_eq!(first[4], "certain");
    assert_eq!(
        unsafe {
            hc_edtf_parse(
                "1985-04-12T23:20:30Z".as_ptr(),
                "1985-04-12T23:20:30Z".len(),
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_MALFORMED
    );
}

#[test]
fn hc_edtf_relations_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_edtf_relations(
            "1984".as_ptr(),
            "1984".len(),
            "1986".as_ptr(),
            "1986".len(),
            buffer,
            capacity,
        )
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 7),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "before");
    assert_eq!(first[1], "<");
    assert_eq!(first[2], "1");
    assert_eq!(
        unsafe {
            hc_edtf_relations(
                "1984".as_ptr(),
                "1984".len(),
                "nonsense".as_ptr(),
                "nonsense".len(),
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_MALFORMED
    );
}

#[test]
fn hc_significant_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_significant(13800000000.0, 3, buffer, capacity)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 6),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "13800000000");
    assert_eq!(first[2], "1.38e10");
    assert_eq!(
        unsafe { hc_significant(1.0, 0, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_significant(1.0, 18, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

#[test]
fn hc_significant_op_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_significant_op(
            "add".as_ptr(),
            "add".len(),
            100.0,
            4,
            0.001,
            1,
            buffer,
            capacity,
        )
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 6),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "add");
    assert_eq!(first[1], "100.0");
    assert_eq!(
        unsafe {
            hc_significant_op(
                "mod".as_ptr(),
                "mod".len(),
                2.0,
                3,
                1.0,
                3,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_significant_op(
                "div".as_ptr(),
                "div".len(),
                1.0,
                3,
                0.0,
                3,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_OUT_OF_RANGE
    );
}

#[test]
fn hc_uncertain_crosses_the_boundary() {
    let text =
        read_lines(|buffer, capacity| unsafe { hc_uncertain(3200.0, 50.0, buffer, capacity) });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 11),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "3200");
    assert_eq!(first[1], "50");
    assert_eq!(first[2], "3200 ± 50");
    assert_eq!(first[5], "3150");
    assert_eq!(first[6], "3250");
    assert_eq!(
        unsafe { hc_uncertain(1.0, -1.0, core::ptr::null_mut(), 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

#[test]
fn hc_uncertain_op_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_uncertain_op(
            "add".as_ptr(),
            "add".len(),
            10.0,
            3.0,
            20.0,
            4.0,
            buffer,
            capacity,
        )
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 5),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "add");
    assert_eq!(first[1], "30");
    assert_eq!(first[2], "5");
    assert_eq!(
        unsafe {
            hc_uncertain_op(
                "sqrt".as_ptr(),
                "sqrt".len(),
                1.0,
                0.0,
                2.0,
                0.0,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_uncertain_op(
                "div".as_ptr(),
                "div".len(),
                1.0,
                1.0,
                0.0,
                1.0,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_OUT_OF_RANGE
    );
}

#[test]
fn hc_interval_crosses_the_boundary() {
    let text = read_lines(|buffer, capacity| unsafe {
        hc_interval("add".as_ptr(), "add".len(), 1, 3, 10, 20, buffer, capacity)
    });
    assert!(!text.is_empty());
    assert!(
        text.lines().all(|line| line.split('\t').count() == 11),
        "{text}"
    );
    let first = row(text.lines().next().unwrap_or_default());
    assert_eq!(first[0], "add");
    assert_eq!(first[1], "0");
    assert_eq!(first[2], "11");
    assert_eq!(first[4], "23");
    assert_eq!(
        unsafe {
            hc_interval(
                "add".as_ptr(),
                "add".len(),
                3,
                1,
                0,
                1,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe {
            hc_interval(
                "scale".as_ptr(),
                "scale".len(),
                0,
                1,
                0,
                1,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
}
