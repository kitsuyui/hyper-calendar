//! The choghadiya, Panchak, the Jupiter festivals, the folk days, the
//! night watches and the northern year names, called as a page calls
//! them.

use super::super::*;
use super::read_lines;

/// Every line of a text, split into cells.
fn rows(text: &str) -> Vec<Vec<&str>> {
    text.lines()
        .map(|line| line.split('\t').collect())
        .collect()
}

/// Drik Panchang's New Delhi page of Wednesday 1 January 2025
/// (`drik-choghadiya-2025`): the day opens with Labha and the night
/// with Udvega, sixteen parts in all.
#[test]
fn the_choghadiya_of_the_first_of_january_2025_cross_the_boundary() {
    let locale = "en";
    let day = hc_gregorian_to_fixed(2025, 1, 1);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_choghadiya(
            day,
            28.6356,
            77.2244,
            0.0,
            locale.as_ptr(),
            locale.len(),
            buffer,
            capacity,
        )
    });
    let rows = rows(&text);
    assert_eq!(rows.len(), 16);
    assert_eq!(
        rows[0][..7],
        ["day", "1", "labha", "Labha", "en", "auspicious", "mercury"]
    );
    assert_eq!(rows[8][..3], ["night", "1", "udvega"]);
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_choghadiya(day, 91.0, 0.0, 0.0, locale.as_ptr(), locale.len(), null, 0) },
        HC_ERR_OUT_OF_RANGE
    );
}

/// Drik Panchang's first window of 2025 (`drik-panchak`): Friday
/// 3 January, 10:47 IST, Chor Panchak; the Kumbh of 2025 at Prayag
/// (`wikipedia-kumbh-mela`); the Godavari Pushkaram of 2015, 14 to
/// 25 July (`wikipedia-godavari-pushkaram`).
#[test]
fn panchak_and_the_jupiter_festivals_cross_the_boundary() {
    let (naming, ayanamsa, locale) = ("panchak-five-kinds", "lahiri", "en");
    let noon = hc_unix_from_fixed(hc_gregorian_to_fixed(2025, 1, 5)) + 6 * 3_600;
    let text = read_lines(|buffer, capacity| unsafe {
        hc_panchak(
            naming.as_ptr(),
            naming.len(),
            noon,
            ayanamsa.as_ptr(),
            ayanamsa.len(),
            19_800,
            locale.as_ptr(),
            locale.len(),
            buffer,
            capacity,
        )
    });
    let row = &rows(&text)[0];
    assert_eq!((row[0], &row[3..]), ("1", &["5", "chor", "Chor", "en"][..]));
    let opens: i64 = row[1].parse().expect("an instant");
    let printed = hc_unix_from_fixed(hc_gregorian_to_fixed(2025, 1, 3)) + (10 * 60 + 47 - 330) * 60;
    assert!((opens - printed).abs() < 90, "{}", opens - printed);

    let (yoga, jupiter) = ("kumbh-prayag-vrishabha", "vrishabha");
    let text = read_lines(|buffer, capacity| unsafe {
        hc_kumbh(
            yoga.as_ptr(),
            yoga.len(),
            2025,
            ayanamsa.as_ptr(),
            ayanamsa.len(),
            jupiter.as_ptr(),
            jupiter.len(),
            locale.as_ptr(),
            locale.len(),
            buffer,
            capacity,
        )
    });
    let row = &rows(&text)[0];
    assert_eq!((row[1], row[2], row[10]), ("prayag", "Prayag", "1"));

    let (sign, meridian) = ("simha", "india");
    let entry = hc_unix_from_fixed(hc_gregorian_to_fixed(2015, 7, 14)) + (7 * 60 + 7 - 330) * 60;
    let text = read_lines(|buffer, capacity| unsafe {
        hc_pushkaram(
            sign.as_ptr(),
            sign.len(),
            entry,
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
    let (first, last) = (
        hc_gregorian_to_fixed(2015, 7, 14).to_string(),
        hc_gregorian_to_fixed(2015, 7, 25).to_string(),
    );
    assert_eq!(
        row[..7],
        [
            "pushkaram-godavari",
            "Godavari",
            "en",
            "",
            "simha",
            &first,
            &last
        ]
    );
    let null = core::ptr::null_mut();
    let unknown = "leo";
    assert_eq!(
        unsafe {
            hc_pushkaram(
                unknown.as_ptr(),
                unknown.len(),
                entry,
                28.6,
                77.2,
                0.0,
                meridian.as_ptr(),
                meridian.len(),
                locale.as_ptr(),
                locale.len(),
                null,
                0,
            )
        },
        HC_ERR_UNKNOWN
    );
}

/// 20 February 2026, the first *cemre*, Kasım 105 (`bilkent-cemre`),
/// in 2026's first-month counts (`netease-2026-longzhishui`); the
/// third watch's second point at 23:48 (`wikipedia-zh-dian`); and
/// Śaka 1946, Pingala by the rule with the *bīja*
/// (`drikpanchang-day-2024-2026`).
#[test]
fn the_folk_days_watches_and_year_names_cross_the_boundary() {
    let (meridian, locale) = ("china", "tr");
    let day = hc_gregorian_to_fixed(2026, 2, 20);
    let text = read_lines(|buffer, capacity| unsafe {
        hc_folk_day(
            day,
            meridian.as_ptr(),
            meridian.len(),
            locale.as_ptr(),
            locale.len(),
            buffer,
            capacity,
        )
    });
    let rows = rows(&text);
    assert_eq!(
        rows[0],
        ["first-month-count", "dragons", "几龙治水", "zh-Hans", "7"]
    );
    assert_eq!(
        rows.last().expect("a line"),
        &["folk-named-day", "cemre-air", "birinci cemre", "tr", "105"]
    );

    let zh = "zh-Hant";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_night_watch(
            23 * 3_600 + 48 * 60,
            zh.as_ptr(),
            zh.len(),
            buffer,
            capacity,
        )
    });
    assert_eq!(text, "3\t2\t三更\tzh-Hant\t夜半\t子\n");
    let null = core::ptr::null_mut();
    assert_eq!(
        unsafe { hc_night_watch(12 * 3_600, zh.as_ptr(), zh.len(), null, 0) },
        0
    );
    assert_eq!(
        unsafe { hc_night_watch(86_400, zh.as_ptr(), zh.len(), null, 0) },
        HC_ERR_OUT_OF_RANGE
    );

    let (rule, en) = ("surya-siddhanta-bija", "en");
    let text = read_lines(|buffer, capacity| unsafe {
        hc_barhaspatya_year(
            rule.as_ptr(),
            rule.len(),
            1_946,
            en.as_ptr(),
            en.len(),
            buffer,
            capacity,
        )
    });
    assert_eq!(text, "51\tPingala\t\t\ten\n");
    let at = hc_unix_from_fixed(hc_gregorian_to_fixed(2025, 3, 30));
    let plain = "surya-siddhanta";
    let text = read_lines(|buffer, capacity| unsafe {
        hc_barhaspatya_year_at(
            plain.as_ptr(),
            plain.len(),
            at,
            en.as_ptr(),
            en.len(),
            buffer,
            capacity,
        )
    });
    assert!(text.starts_with("53\t"), "{text}");
    assert_eq!(
        unsafe {
            hc_barhaspatya_year(
                rule.as_ptr(),
                rule.len(),
                6_822,
                en.as_ptr(),
                en.len(),
                null,
                0,
            )
        },
        HC_ERR_OUT_OF_RANGE
    );
}
