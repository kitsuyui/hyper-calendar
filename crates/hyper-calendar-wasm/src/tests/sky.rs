use super::super::*;
use super::read_lines;

/// At the NAOJ's 秋分 of 2026, 23 September 00:05 UTC, the Sun enters
/// Libra, whose first face al-Bīrūnī gives to the Moon.
#[test]
fn the_sun_enters_the_moons_face_of_libra_at_the_equinox() {
    let instant = at(2026, 9, 23, 1, 5);
    let text = read_lines(|buffer, capacity| unsafe { hc_decan_at(instant, buffer, capacity) });
    assert!(
        text.starts_with("7\tlibra\tLibra\t1\tmoon\tMoon\t0.0"),
        "{text}"
    );
    let decan = |unix_seconds: i64| unsafe { hc_decan_at(unix_seconds, core::ptr::null_mut(), 0) };
    assert_eq!(decan(i64::MIN), HC_ERR_OUT_OF_RANGE);
    // The years −1000 to 3000, and a second either side.
    assert!(decan(-93_724_128_000) > 0);
    assert!(decan(32_535_215_999) > 0);
    assert_eq!(decan(-93_724_128_001), HC_ERR_OUT_OF_RANGE);
    assert_eq!(decan(32_535_216_000), HC_ERR_OUT_OF_RANGE);
}

/// The POSIX timestamp of a UTC date and time.
fn at(year: i64, month: u32, day: u32, hour: i64, minute: i64) -> i64 {
    hc_unix_from_fixed(hc_gregorian_to_fixed(year, month, day)) + hour * 3_600 + minute * 60
}

fn sky(unix: i64) -> Vec<String> {
    let text = read_lines(|buffer, capacity| unsafe { hc_sky_at(unix, buffer, capacity) });
    let line = text.strip_suffix('\n').expect("one line");
    assert!(!line.contains('\n'), "{text:?}");
    line.split('\t').map(str::to_owned).collect()
}

fn rows(text: &str) -> Vec<Vec<String>> {
    text.lines()
        .map(|line| line.split('\t').map(str::to_owned).collect())
        .collect()
}

fn terms(from: i64, to: i64) -> Vec<Vec<String>> {
    rows(&read_lines(|buffer, capacity| unsafe {
        hc_solar_terms_between(from, to, buffer, capacity)
    }))
}

fn phases(from: i64, to: i64) -> Vec<Vec<String>> {
    rows(&read_lines(|buffer, capacity| unsafe {
        hc_moon_phases_between(from, to, buffer, capacity)
    }))
}

fn number(cell: &str) -> f64 {
    cell.parse()
        .unwrap_or_else(|_| panic!("not a number: {cell}"))
}

/// The 暦要項 of the National Astronomical Observatory of Japan for
/// 2026 puts the new moon (朔) of September at 11 September 12:27
/// JST, which is 03:27 UTC, and the equinox (秋分) at 23 September
/// 09:05 JST, 00:05 UTC; both are printed to the minute, and the
/// Hong Kong Observatory's tables round the same conjunction to
/// 03:26 UTC.
#[test]
fn the_sky_before_the_new_moon_of_september_2026_decodes_column_by_column() {
    let columns = sky(at(2026, 9, 11, 3, 0));
    assert_eq!(columns.len(), 12, "{columns:?}");
    // Twelve days before the equinox the Sun is about 168° along.
    let sun = number(&columns[0]);
    assert!((167.0..170.0).contains(&sun), "{sun}");
    let distance = number(&columns[1]);
    assert!((1.0..1.02).contains(&distance), "{distance} au");
    // Half an hour before conjunction the Moon is just behind the
    // Sun, within six degrees of the ecliptic, and dark.
    let moon = number(&columns[2]);
    assert!((sun - moon).abs() < 1.0, "sun {sun}, moon {moon}");
    assert!(number(&columns[3]).abs() < 5.5, "{columns:?}");
    let kilometres = number(&columns[4]);
    assert!((355_000.0..407_000.0).contains(&kilometres), "{kilometres}");
    let elongation = number(&columns[5]);
    assert!(elongation > 359.0, "{elongation}");
    assert!(number(&columns[6]) < 0.001, "{columns:?}");
    let previous: i64 = columns[7].parse().expect("a timestamp");
    let next: i64 = columns[8].parse().expect("a timestamp");
    let published = at(2026, 9, 11, 3, 27);
    assert!((next - published).abs() <= 90, "next new moon {next}");
    assert!(
        (29..30).contains(&((next - previous) / 86_400)),
        "{columns:?}"
    );
    // ΔT in September 2026 comes from the USNO's predictions.
    let delta_t = number(&columns[9]);
    assert!((69.0..69.5).contains(&delta_t), "{delta_t}");
    assert_eq!(columns[10], "predicted");
    assert!(columns[11].contains("VSOP87"), "{}", columns[11]);
    assert!(columns[11].contains("deltat.preds"), "{}", columns[11]);
}

#[test]
fn the_terms_and_phases_of_september_2026_are_where_the_almanacs_put_them() {
    let (from, to) = (at(2026, 9, 1, 0, 0), at(2026, 10, 1, 0, 0));
    let terms = terms(from, to);
    assert_eq!(terms.len(), 2, "{terms:?}");
    assert!(terms.iter().all(|row| row.len() == 4), "{terms:?}");
    assert_eq!(terms[0][0], "165");
    assert_eq!(terms[0][2..], ["白露", "白露"]);
    let equinox = &terms[1];
    assert_eq!(equinox[0], "180");
    assert_eq!(equinox[2..], ["秋分", "秋分"]);
    let instant: i64 = equinox[1].parse().expect("a timestamp");
    let published = at(2026, 9, 23, 0, 5);
    assert!((instant - published).abs() <= 90, "equinox at {instant}");

    let phases = phases(from, to);
    assert_eq!(phases.len(), 4, "{phases:?}");
    assert!(phases.iter().all(|row| row.len() == 4), "{phases:?}");
    let kinds: Vec<(&str, &str)> = phases
        .iter()
        .map(|row| (row[0].as_str(), row[2].as_str()))
        .collect();
    // 下弦 on the 4th, 朔 on the 11th, 上弦 on the 19th (JST) and
    // 望 on the 27th.
    assert_eq!(
        kinds,
        [
            ("270", "last-quarter"),
            ("0", "new"),
            ("90", "first-quarter"),
            ("180", "full")
        ]
    );
    assert!(phases.iter().all(|row| row[3].is_empty()), "{phases:?}");
    let new_moon: i64 = phases[1][1].parse().expect("a timestamp");
    assert!(
        (new_moon - at(2026, 9, 11, 3, 27)).abs() <= 90,
        "{new_moon}"
    );
    // 望 at 27 September 01:49 JST, 16:49 UTC on the 26th.
    let full_moon: i64 = phases[3][1].parse().expect("a timestamp");
    assert!(
        (full_moon - at(2026, 9, 26, 16, 49)).abs() <= 90,
        "{full_moon}"
    );
    // The new moons the list gives are the ones `hc_sky_at` gives.
    let before = sky(new_moon - 60);
    assert_eq!(before[8], phases[1][1]);
    let after = sky(new_moon + 60);
    assert_eq!(after[7], phases[1][1]);
}

/// The Chinese calendar of 2023 had a leap second month (閏二月)
/// because the lunation that began with the new moon of 21 March
/// 2023 17:23 UTC and ended with that of 20 April 04:12 UTC held no
/// principal term (中気): 春分 fell earlier on the 21st and 穀雨
/// later on the 20th, leaving only the sectional 清明.
#[test]
fn the_lunation_of_march_2023_holds_no_principal_term() {
    let first = sky(at(2023, 3, 21, 0, 0));
    let new_moon: i64 = first[8].parse().expect("a timestamp");
    assert!(
        (new_moon - at(2023, 3, 21, 17, 23)).abs() <= 90,
        "{new_moon}"
    );
    let second = sky(new_moon + 1);
    let next_new_moon: i64 = second[8].parse().expect("a timestamp");
    assert!(
        (next_new_moon - at(2023, 4, 20, 4, 12)).abs() <= 90,
        "{next_new_moon}"
    );
    let terms = terms(new_moon, next_new_moon);
    assert_eq!(terms.len(), 1, "{terms:?}");
    assert_eq!(terms[0][0], "15");
    assert_eq!(terms[0][2..], ["清明", "清明"]);
    assert!(
        terms.iter().all(|row| number(&row[0]) % 30.0 != 0.0),
        "a principal term in {terms:?}"
    );
}

#[test]
fn the_moon_is_lit_at_a_full_moon() {
    let phases = phases(at(2026, 1, 1, 0, 0), at(2027, 1, 1, 0, 0));
    let full: Vec<i64> = phases
        .iter()
        .filter(|row| row[2] == "full")
        .map(|row| row[1].parse().expect("a timestamp"))
        .collect();
    // 2026 has thirteen full moons: two in May.
    assert_eq!(full.len(), 13, "{phases:?}");
    for instant in full {
        let columns = sky(instant);
        let fraction = number(&columns[6]);
        assert!(fraction > 0.99, "{fraction} at {instant}");
        let elongation = number(&columns[5]);
        assert!((elongation - 180.0).abs() < 0.5, "{elongation}");
    }
}

#[test]
fn instants_outside_the_era_and_spans_too_long_are_refused() {
    let null = core::ptr::null_mut();
    assert!(unsafe { hc_sky_at(at(3000, 12, 31, 23, 59), null, 0) } > 0);
    assert!(unsafe { hc_sky_at(at(-1000, 1, 1, 0, 0), null, 0) } > 0);
    for instant in [
        at(3001, 1, 1, 0, 0),
        at(-1000, 1, 1, 0, 0) - 1,
        i64::MAX,
        i64::MIN,
    ] {
        assert_eq!(
            unsafe { hc_sky_at(instant, null, 0) },
            HC_ERR_OUT_OF_RANGE,
            "{instant}"
        );
        assert_eq!(
            unsafe { hc_solar_terms_between(instant, instant.saturating_add(1), null, 0) },
            HC_ERR_OUT_OF_RANGE
        );
    }
    for instant in [at(3001, 1, 1, 0, 1), i64::MAX] {
        assert_eq!(
            unsafe { hc_moon_phases_between(at(2000, 1, 1, 0, 0), instant, null, 0) },
            HC_ERR_OUT_OF_RANGE
        );
    }
    // A span may end at the era's end, but not run past it.
    let end = at(3001, 1, 1, 0, 0);
    assert!(unsafe { hc_solar_terms_between(end - 86_400 * 40, end, null, 0) } > 0);
    assert_eq!(
        unsafe { hc_solar_terms_between(end - 86_400 * 40, end + 1, null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
    // Four hundred years is the most a call answers for.
    let from = at(1600, 1, 1, 0, 0);
    assert_eq!(
        unsafe { hc_moon_phases_between(from, at(2001, 1, 1, 0, 0), null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_solar_terms_between(from, at(2001, 1, 1, 0, 0), null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
    // An empty or inverted span is an empty answer, not an error.
    let instant = at(2026, 9, 11, 3, 0);
    assert_eq!(
        unsafe { hc_solar_terms_between(instant, instant, null, 0) },
        0
    );
    assert_eq!(
        unsafe { hc_moon_phases_between(instant, instant - 1, null, 0) },
        0
    );
}

#[test]
fn a_decade_of_terms_and_phases_is_complete_and_ordered() {
    let (from, to) = (at(2000, 1, 1, 0, 0), at(2010, 1, 1, 0, 0));
    let terms = terms(from, to);
    assert_eq!(terms.len(), 240, "{}", terms.len());
    let phases = phases(from, to);
    // 123 or 124 lunations of four phases each.
    assert!((492..=496).contains(&phases.len()), "{}", phases.len());
    for rows in [&terms, &phases] {
        let instants: Vec<i64> = rows
            .iter()
            .map(|row| row[1].parse().expect("a timestamp"))
            .collect();
        assert!(instants.windows(2).all(|pair| pair[0] < pair[1]));
        assert!(
            instants
                .iter()
                .all(|&instant| from <= instant && instant < to)
        );
    }
    // Term angles step by 15° and phase angles by 90°, wrapping.
    for pair in terms.windows(2) {
        let step = (number(&pair[1][0]) - number(&pair[0][0])).rem_euclid(360.0);
        assert!((step - 15.0).abs() < 1e-9, "{pair:?}");
    }
    for pair in phases.windows(2) {
        let step = (number(&pair[1][0]) - number(&pair[0][0])).rem_euclid(360.0);
        assert!((step - 90.0).abs() < 1e-9, "{pair:?}");
    }
}

#[test]
fn the_sky_measures_and_refuses_a_short_buffer_like_every_line_export() {
    let instant = at(2026, 9, 11, 3, 0);
    let needed = unsafe { hc_sky_at(instant, core::ptr::null_mut(), 0) };
    assert!(needed > 0);
    let mut small = [7u8; 8];
    assert_eq!(
        unsafe { hc_sky_at(instant, small.as_mut_ptr(), small.len()) },
        HC_ERR_BUFFER_TOO_SMALL
    );
    assert_eq!(small, [7u8; 8]);
    let capacity = needed as usize;
    let pointer = hc_alloc(capacity);
    assert_eq!(unsafe { hc_sky_at(instant, pointer, capacity) }, needed);
    unsafe { hc_free(pointer, capacity) };
}
