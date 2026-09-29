use super::super::*;
use super::{measured, read_lines};

/// The Sun in the Moon's face of Libra an hour after the NAOJ's 秋分
/// of 2026, 23 September 00:05 UTC, as the module writes it.
#[test]
fn the_decan_line_is_the_modules() {
    let instant = at(2026, 9, 23, 1, 5);
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_decan_at(instant, buffer, capacity, written)
    });
    assert_eq!(
        text,
        hc::sky_lines::decan_line(instant).expect("in the era")
    );
    assert!(
        text.starts_with("7\tlibra\tLibra\t1\tmoon\tMoon\t"),
        "{text}"
    );
    let decan = |unix_seconds: i64| {
        measured(|buffer, capacity, written| unsafe {
            hc_decan_at(unix_seconds, buffer, capacity, written)
        })
    };
    assert_eq!(decan(-93_724_128_000), HC_ERROR_BUFFER_TOO_SMALL);
    assert_eq!(decan(32_535_215_999), HC_ERROR_BUFFER_TOO_SMALL);
    assert_eq!(decan(-93_724_128_001), HC_ERROR_OUT_OF_RANGE);
    assert_eq!(decan(32_535_216_000), HC_ERROR_OUT_OF_RANGE);
}

/// The POSIX timestamp of a UTC date and time.
fn at(year: i64, month: u8, day: u8, hour: i64, minute: i64) -> i64 {
    let mut fixed = 0i64;
    assert_eq!(
        unsafe { hc_gregorian_to_fixed(year, month, day, &mut fixed) },
        HC_OK
    );
    (fixed - 719_163) * 86_400 + hour * 3_600 + minute * 60
}

fn rows(text: &str) -> Vec<Vec<String>> {
    text.lines()
        .map(|line| line.split('\t').map(str::to_owned).collect())
        .collect()
}

/// The 暦要項 of the National Astronomical Observatory of Japan for
/// 2026 puts the new moon of September at 11 September 12:27 JST,
/// 03:27 UTC, and the equinox at 23 September 09:05 JST, 00:05 UTC.
#[test]
fn the_sky_the_terms_and_the_phases_decode_column_by_column() {
    let instant = at(2026, 9, 11, 3, 0);
    let sky = read_lines(|buffer, capacity, written| unsafe {
        hc_sky_at(instant, buffer, capacity, written)
    });
    let columns = rows(&sky);
    assert_eq!(columns.len(), 1, "{sky:?}");
    let columns = &columns[0];
    assert_eq!(columns.len(), 12, "{columns:?}");
    let elongation: f64 = columns[5].parse().expect("a number");
    assert!(elongation > 359.0, "{elongation}");
    let next: i64 = columns[8].parse().expect("a timestamp");
    assert!((next - at(2026, 9, 11, 3, 27)).abs() <= 90, "{next}");
    assert_eq!(columns[10], "predicted");
    assert!(columns[11].contains("VSOP87"), "{}", columns[11]);

    let (from, to) = (at(2026, 9, 1, 0, 0), at(2026, 10, 1, 0, 0));
    let terms = rows(&read_lines(|buffer, capacity, written| unsafe {
        hc_solar_terms_between(from, to, buffer, capacity, written)
    }));
    assert_eq!(terms.len(), 2, "{terms:?}");
    assert_eq!(terms[1][0], "180");
    assert_eq!(terms[1][2..], ["秋分", "秋分"]);
    let equinox: i64 = terms[1][1].parse().expect("a timestamp");
    assert!((equinox - at(2026, 9, 23, 0, 5)).abs() <= 90, "{equinox}");

    let phases = rows(&read_lines(|buffer, capacity, written| unsafe {
        hc_moon_phases_between(from, to, buffer, capacity, written)
    }));
    assert_eq!(phases.len(), 4, "{phases:?}");
    assert_eq!(phases[1][0], "0");
    assert_eq!(phases[1][2..], ["new", ""]);
    assert_eq!(phases[1][1], columns[8]);
}

#[test]
fn instants_outside_the_era_and_spans_too_long_are_refused() {
    let null = core::ptr::null_mut();
    let mut written = 0usize;
    assert_eq!(
        unsafe { hc_sky_at(at(3001, 1, 1, 0, 0), null, 0, &mut written) },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_sky_at(i64::MIN, null, 0, &mut written) },
        HC_ERROR_OUT_OF_RANGE
    );
    let from = at(1600, 1, 1, 0, 0);
    assert_eq!(
        unsafe { hc_solar_terms_between(from, at(2001, 1, 1, 0, 0), null, 0, &mut written) },
        HC_ERROR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_moon_phases_between(from, at(3001, 1, 1, 0, 1), null, 0, &mut written) },
        HC_ERROR_OUT_OF_RANGE
    );
    // An empty span is an empty, terminated answer.
    let mut buffer = [7 as c_char; 4];
    assert_eq!(
        unsafe { hc_solar_terms_between(from, from, buffer.as_mut_ptr(), 4, &mut written) },
        HC_OK
    );
    assert_eq!(written, 1);
    assert_eq!(buffer[0], 0);
}
