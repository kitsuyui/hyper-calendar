//! The choghadiya, Panchak, the Jupiter festivals, the folk days, the
//! night watches and the northern year names, across the C boundary.

use super::super::*;
use super::{measured, read_lines};

fn first(text: &str) -> Vec<&str> {
    text.lines().next().expect("a line").split('\t').collect()
}

/// Drik Panchang's New Delhi pages of 2025 (`drik-choghadiya-2025`,
/// `drik-panchak`), the Kumbh of 2025 (`wikipedia-kumbh-mela`) and
/// the Godavari Pushkaram of 2015 (`wikipedia-godavari-pushkaram`).
#[test]
fn the_indic_reckonings_cross_the_c_boundary() {
    let day = 739_252;
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_choghadiya(
            day,
            28.6356,
            77.2244,
            0.0,
            c"en".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(text.lines().count(), 16);
    assert_eq!(first(&text)[..5], ["day", "1", "labha", "Labha", "en"]);
    // 12:00 UTC on 5 January 2025, within the first window.
    let noon = 1_736_078_400;
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_panchak(
            c"panchak-five-kinds".as_ptr(),
            noon,
            c"lahiri".as_ptr(),
            19_800,
            core::ptr::null(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(first(&text)[3..], ["5", "chor", "Chor", "en"]);
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_kumbh(
            c"kumbh-prayag-vrishabha".as_ptr(),
            2025,
            c"lahiri".as_ptr(),
            core::ptr::null(),
            c"en".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!((first(&text)[1], first(&text)[12]), ("prayag", ""));
    // 07:07 IST on 14 July 2015 is 01:37 UTC.
    let entry = 1_436_837_820;
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_pushkaram(
            c"simha".as_ptr(),
            entry,
            28.6356,
            77.2244,
            0.0,
            c"india".as_ptr(),
            c"en".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(first(&text)[..2], ["pushkaram-godavari", "Godavari"]);
    assert_eq!(first(&text)[4..8], ["simha", "Siṃha", "735793", "735804"]);
    assert_eq!(
        measured(|buffer, capacity, written| unsafe {
            hc_pushkaram(
                core::ptr::null(),
                entry,
                28.6,
                77.2,
                0.0,
                c"india".as_ptr(),
                c"en".as_ptr(),
                buffer,
                capacity,
                written,
            )
        }),
        HC_ERROR_NULL_POINTER
    );
}

/// The folk days of 20 February 2026 (`bilkent-cemre`), the third
/// watch at 23:48 (`wikipedia-zh-dian`) and Śaka 1946, Pingala
/// (`drikpanchang-day-2024-2026`).
#[test]
fn the_folk_days_watches_and_year_names_cross_the_c_boundary() {
    let day = 739_667;
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_folk_day(
            day,
            c"china".as_ptr(),
            c"tr".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert!(
        text.ends_with("folk-named-day\tcemre-air\tbirinci cemre\ttr\t105\n"),
        "{text}"
    );
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_night_watch(
            23 * 3_600 + 48 * 60,
            c"zh-TW".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(text, "3\t2\t三更\tzh-Hant\t夜半\t子\n");
    let mut buffer = [7 as c_char; 4];
    let mut written = 0usize;
    assert_eq!(
        unsafe {
            hc_night_watch(
                12 * 3_600,
                core::ptr::null(),
                buffer.as_mut_ptr(),
                4,
                &mut written,
            )
        },
        HC_OK
    );
    assert_eq!((buffer[0], written), (0, 1));
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_barhaspatya_year(
            c"surya-siddhanta-bija".as_ptr(),
            1_946,
            c"en".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert_eq!(text, "51\tPingala\t\t\ten\n");
    // 30 March 2025, Chaitra śukla 1: Siddharthi without the bīja.
    let text = read_lines(|buffer, capacity, written| unsafe {
        hc_barhaspatya_year_at(
            c"surya-siddhanta".as_ptr(),
            1_743_292_800,
            c"en".as_ptr(),
            buffer,
            capacity,
            written,
        )
    });
    assert!(text.starts_with("53\t"), "{text}");
}

/// The Mela Adhikari's seven conditions and Wikipedia's Pushkaram table
/// (`kumbh-allahabad-astrology`, `wikipedia-pushkaram`): Haridwar first, 14
/// rivers with the Ganga at Meṣa.
#[test]
fn the_festivals_tables_cross_the_c_boundary() {
    let yogas = read_lines(|buffer, capacity, written| unsafe {
        hc_kumbh_yogas(c"en".as_ptr(), buffer, capacity, written)
    });
    assert_eq!(yogas.lines().count(), 7);
    assert!(yogas.starts_with(
        "kumbh-haridwar\tharidwar\tHaridwar\ten\tGanga\tkumbha\tKumbha\tmesha\tMeṣa\t0\t"
    ));
    let rivers = read_lines(|buffer, capacity, written| unsafe {
        hc_pushkaram_rivers(core::ptr::null(), buffer, capacity, written)
    });
    assert_eq!(rivers.lines().count(), 14);
    assert!(
        rivers
            .lines()
            .any(|line| line.starts_with("pushkaram-ganga\t") && line.contains("\tmesha\tMeṣa\t"))
    );
}
