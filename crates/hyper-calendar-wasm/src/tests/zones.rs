use super::super::*;
use super::read_lines;

fn location(zone: &str, locale: &str) -> i64 {
    unsafe {
        hc_zone_location(
            zone.as_ptr(),
            zone.len(),
            locale.as_ptr(),
            locale.len(),
            core::ptr::null_mut(),
            0,
        )
    }
}

fn line(zone: &str, locale: &str) -> String {
    read_lines(|buffer, capacity| unsafe {
        hc_zone_location(
            zone.as_ptr(),
            zone.len(),
            locale.as_ptr(),
            locale.len(),
            buffer,
            capacity,
        )
    })
}

/// `zone1970.tab` 2026d: `JP,AU +353916+1394441 Asia/Tokyo Eyre
/// Bird Observatory`, 128 356″ and 503 081″.
#[test]
fn every_zone_has_a_line_and_tokyo_is_its_row() {
    let text =
        read_lines(|buffer, capacity| unsafe { hc_zones("en".as_ptr(), 2, buffer, capacity) });
    assert_eq!(text.lines().count(), 312);
    assert!(text.lines().all(|line| line.split('\t').count() == 8));
    let tokyo = text
        .lines()
        .find(|line| line.starts_with("Asia/Tokyo\t"))
        .expect("Tokyo");
    assert_eq!(
        tokyo,
        "Asia/Tokyo\t35.654444\t139.744722\tJP;AU\tJP\tEyre Bird Observatory\tTokyo\ten"
    );
    assert_eq!(line("asia/tokyo", "en"), format!("{tokyo}\n"));
}

/// `zone.tab`: `NO +5955+01045 Europe/Oslo`; `backward`: `Link
/// Asia/Kolkata Asia/Calcutta` and `Link Etc/UTC UTC`.
#[test]
fn links_answer_with_their_rows_and_utc_is_unknown() {
    assert!(line("Europe/Oslo", "en").starts_with("Europe/Oslo\t59.916667\t10.750000\tNO\tNO\t\t"));
    assert!(line("Asia/Calcutta", "en").starts_with("Asia/Kolkata\t"));
    assert_eq!(location("UTC", "en"), HC_ERR_UNKNOWN);
    let not_utf8 = [0xffu8];
    assert_eq!(
        unsafe {
            hc_zone_location(
                not_utf8.as_ptr(),
                1,
                "en".as_ptr(),
                2,
                core::ptr::null_mut(),
                0,
            )
        },
        HC_ERR_NOT_UTF8
    );
    assert_eq!(
        unsafe { hc_zones(core::ptr::null(), 1, core::ptr::null_mut(), 0) },
        HC_ERR_NULL_POINTER
    );
}

/// CLDR 48 `ja.xml`: `Asia/Tokyo` 東京. The city is localised only
/// in a build that carries `calendars` too.
#[test]
fn the_city_is_in_the_locale_where_the_build_carries_it() {
    let tokyo = line("Asia/Tokyo", "ja-JP");
    let cells: Vec<&str> = tokyo.trim_end().split('\t').collect();
    if cfg!(any(feature = "calendars", feature = "zone-names")) {
        assert_eq!(cells[6..], ["東京", "ja"]);
    } else {
        assert_eq!(cells[6..], ["Tokyo", "en"]);
    }
}
