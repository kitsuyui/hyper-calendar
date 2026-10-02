//! Jupiter's position, its ingresses and the festivals found from them,
//! called as a page calls them.

use super::super::*;
use super::read_lines;

fn rows(text: &str) -> Vec<Vec<&str>> {
    text.lines()
        .map(|line| line.split('\t').collect())
        .collect()
}

fn unix(year: i64, month: u32, day: u32) -> i64 {
    hc_unix_from_fixed(hc_gregorian_to_fixed(year, month, day))
}

/// Horizons' apparent ecliptic longitude of Jupiter at 0 h UT on 2024-12-07,
/// the day of its opposition, 76.3751533° (`jpl-horizons`).
#[test]
fn jupiter_at_its_opposition_of_2024_crosses_the_boundary() {
    let ayanamsa = "lahiri";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_jupiter_at(
            unix(2024, 12, 7),
            ayanamsa.as_ptr(),
            ayanamsa.len(),
            buffer,
            capacity,
        )
    });
    let row = &rows(&text)[0];
    assert_eq!(row.len(), 12);
    let longitude: f64 = row[0].parse().expect("a number");
    assert!((longitude - 76.375_153_3).abs() < 0.0002);
    assert_eq!((row[4], row[5], row[8]), ("vrishabha", "Vṛṣabha", "1"));
    let unknown = "no-such";
    assert_eq!(
        unsafe { hc_jupiter_at(0, unknown.as_ptr(), unknown.len(), core::ptr::null_mut(), 0,) },
        HC_ERR_UNKNOWN
    );
    assert_eq!(
        unsafe {
            hc_jupiter_at(
                unix(3001, 1, 1),
                ayanamsa.as_ptr(),
                ayanamsa.len(),
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_OUT_OF_RANGE
    );
}

/// Drik Panchang's entries of 2019 (`drik-guru-gochar`): into Dhanus on
/// 30 March, back into Vṛścika on 22 April and into Dhanus on 5 November.
#[test]
fn the_ingresses_of_2019_cross_the_boundary() {
    let ayanamsa = "lahiri";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_jupiter_ingresses(
            unix(2019, 1, 1),
            unix(2020, 1, 1),
            ayanamsa.as_ptr(),
            ayanamsa.len(),
            buffer,
            capacity,
        )
    });
    let rows = rows(&text);
    assert_eq!(
        rows.iter()
            .map(|row| (row[1], row[3], row[5]))
            .collect::<Vec<_>>(),
        [
            ("vrishchika", "dhanus", "forward"),
            ("dhanus", "vrishchika", "retrograde"),
            ("vrishchika", "dhanus", "forward"),
        ]
    );
    let empty = unsafe {
        hc_jupiter_ingresses(
            unix(2019, 1, 1),
            unix(2019, 1, 1),
            ayanamsa.as_ptr(),
            ayanamsa.len(),
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(empty, 0);
}

/// The Maha Kumbh of 2025 (`wikipedia-kumbh-mela`) and the Godavari
/// Pushkaram of 2015 (`wikipedia-godavari-pushkaram`), with Jupiter found.
#[test]
fn the_festivals_found_from_the_sky_cross_the_boundary() {
    let (yoga, ayanamsa, locale) = ("kumbh-prayag-vrishabha", "lahiri", "en");
    let text = read_lines(|buffer, capacity| unsafe {
        hc_kumbh_by_sky(
            yoga.as_ptr(),
            yoga.len(),
            2025,
            ayanamsa.as_ptr(),
            ayanamsa.len(),
            locale.as_ptr(),
            locale.len(),
            buffer,
            capacity,
        )
    });
    let row = &rows(&text)[0];
    assert_eq!(row.len(), 15);
    assert_eq!(
        (row[1], row[2], row[12], row[13]),
        ("prayag", "Prayag", "1", "vrishabha")
    );

    let (sign, rule, meridian) = ("simha", "pushkaram-final-entry", "india");
    let text = read_lines(|buffer, capacity| unsafe {
        hc_pushkaram_by_sky(
            sign.as_ptr(),
            sign.len(),
            2015,
            ayanamsa.as_ptr(),
            ayanamsa.len(),
            rule.as_ptr(),
            rule.len(),
            28.6356,
            77.2244,
            0.0,
            meridian.as_ptr(),
            meridian.len(),
            locale.as_ptr(),
            locale.len(),
            buffer,
            capacity,
        )
    });
    let row = &rows(&text)[0];
    assert_eq!(row.len(), 14);
    assert_eq!(row[..3], ["pushkaram-godavari", "Godavari", "en"]);
    assert_eq!(row[6], hc_gregorian_to_fixed(2015, 7, 14).to_string());
    assert_eq!(row[7], hc_gregorian_to_fixed(2015, 7, 25).to_string());
    assert_eq!(row[13], rule);
    // A year with no entry is no line: the buffer's measure is zero bytes.
    let none = unsafe {
        hc_pushkaram_by_sky(
            sign.as_ptr(),
            sign.len(),
            2019,
            ayanamsa.as_ptr(),
            ayanamsa.len(),
            rule.as_ptr(),
            rule.len(),
            28.6356,
            77.2244,
            0.0,
            meridian.as_ptr(),
            meridian.len(),
            locale.as_ptr(),
            locale.len(),
            core::ptr::null_mut(),
            0,
        )
    };
    assert_eq!(none, 0);
    let wrong = "second";
    assert_eq!(
        unsafe {
            hc_pushkaram_by_sky(
                sign.as_ptr(),
                sign.len(),
                2015,
                ayanamsa.as_ptr(),
                ayanamsa.len(),
                wrong.as_ptr(),
                wrong.len(),
                28.6356,
                77.2244,
                0.0,
                meridian.as_ptr(),
                meridian.len(),
                locale.as_ptr(),
                locale.len(),
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
}

/// Drik Panchang's Guru Asta page for New Delhi (`drik-guru-asta`): Jupiter
/// is lost in the Sun's light from 15 July to 12 August 2026, and rises in
/// Puṣya; the year of Jupiter that begins is Pauṣa by the *Bṛhatsaṃhitā*.
#[test]
fn the_rising_of_2026_crosses_the_boundary() {
    let ayanamsa = "lahiri";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_jupiter_risings(
            unix(2026, 1, 1),
            unix(2027, 1, 1),
            ayanamsa.as_ptr(),
            ayanamsa.len(),
            buffer,
            capacity,
        )
    });
    let rows = rows(&text);
    assert_eq!(rows.len(), 1, "{text}");
    assert_eq!(rows[0].len(), 8);
    assert_eq!(rows[0][4..8], ["pushya", "Puṣya", "Pausha", "10"]);
    let rising: i64 = rows[0][0].parse().expect("a second");
    assert!((unix(2026, 8, 11)..unix(2026, 8, 15)).contains(&rising));
}
