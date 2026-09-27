//! The tab-separated lines the WebAssembly module and the C library write
//! about where time zones are, written once.
//!
//! A line is one row of [`hc_tz::location`]: the zone, the latitude and
//! longitude of its principal location in decimal degrees, its countries,
//! the one country `zone.tab` lists it under, the table's comment, and the zone's exemplar city in a locale with the
//! tag of the data that answered, from [`hc_i18n::exemplar_cities`]. The
//! cities are English unless the build has the `localized-exemplar-cities`
//! feature; `docs/systems/zone-locations.md` explains both halves.

use alloc::string::String;
use core::fmt::Write;

use hc_i18n::exemplar_cities::{self, ExemplarCity};
use hc_tz::location::{self, ZoneLocation};

use crate::boundary::{Answer, Refusal, push_cell};

/// How many columns [`zones`] and [`zone_location`] write.
pub const ZONE_COLUMNS: usize = 8;

/// The exemplar city of a row in the locale a tag asks for: CLDR's in
/// that locale where the build carries it and the locale's fallback chain
/// has a value, else English's. `native` names no one locale and answers
/// in English, as does a tag that does not parse.
#[cfg(feature = "localized-exemplar-cities")]
fn city(row: &ZoneLocation, locale: &str) -> ExemplarCity<'static> {
    use hc_i18n::Locale;
    if locale == "native" {
        return exemplar_cities::english_city(row.row, row.zone);
    }
    let requested = Locale::parse(locale).unwrap_or(Locale::ROOT);
    exemplar_cities::exemplar_city(&requested, row.row, row.zone)
}

/// The exemplar city of a row: English's, since the build does not carry
/// the other locales' cities.
#[cfg(not(feature = "localized-exemplar-cities"))]
fn city(row: &ZoneLocation, _locale: &str) -> ExemplarCity<'static> {
    exemplar_cities::english_city(row.row, row.zone)
}

/// One line: the zone; the latitude, degrees north, and the longitude,
/// degrees east, each the table's whole arcseconds written in decimal
/// degrees to six places (see [`hc_tz::DecimalDegrees`]); the ISO 3166-1 codes of the
/// countries, `;`-separated, the country of the principal location first;
/// the one country `zone.tab` lists the name under, empty where it has no
/// row for it; the table's comment; the exemplar city; and the tag that
/// named it.
fn push_line(out: &mut String, row: &ZoneLocation, locale: &str) {
    push_cell(out, row.zone);
    let _ = write!(
        out,
        "\t{}\t{}\t",
        row.coordinates.latitude_decimal(),
        row.coordinates.longitude_decimal()
    );
    for (index, country) in row.countries().enumerate() {
        if index > 0 {
            out.push(';');
        }
        push_cell(out, country);
    }
    out.push('\t');
    push_cell(out, row.zone_tab_country);
    out.push('\t');
    push_cell(out, row.comment);
    out.push('\t');
    let city = city(row, locale);
    let _ = write!(out, "{}", city.name);
    out.push('\t');
    out.push_str(city.tag);
    out.push('\n');
}

/// The lines of `hc_zones`: one per zone of `zone1970.tab`, in its order,
/// each the zone; the latitude, degrees north, and the longitude, degrees
/// east, of its principal location, each the table's whole arcseconds
/// written in decimal degrees to six places; the
/// ISO 3166-1 codes of its countries, `;`-separated, the location's
/// first; the one country `zone.tab` lists the zone under; the table's
/// comment; the exemplar city; and the tag that named the city.
///
/// Column 5 is the country a label can name: `JP` for `Asia/Tokyo`,
/// whose column 4 is `JP;AU`. It is `zone.tab`'s, which gives each name
/// one country: the first of column 4 for every zone but
/// `Europe/Simferopol`, `RU;UA` there and `UA` in `zone.tab`. It is empty
/// for a name `zone.tab` has no row for, which no name of release 2026c
/// is (see [`hc_tz::location::ZoneLocation::zone_tab_country`]).
///
/// Column 7 is the zone's CLDR 48 exemplar city in the locale — 東京 for
/// `Asia/Tokyo` under `ja` — and column 8 the tag of the data that
/// answered, `ja`, so that a request for `ja-JP` says `ja`. A zone the
/// locale has no city for, every zone under `native` or a tag whose chain
/// reaches no table, and every zone in a build without the
/// `localized-exemplar-cities` feature, is given its English city with
/// `en`: `en.xml`'s, else `root.xml`'s, else the name UTS #35 derives from
/// the zone's identifier. So column 7 is never empty.
#[must_use]
pub fn zones(locale: &str) -> String {
    let mut out = String::new();
    for row in location::zones() {
        push_line(&mut out, &row, locale);
    }
    out
}

/// The line of `hc_zone_location`: [`zones`]' line for one name, which
/// may be a zone, a link `zone.tab` gives a place of its own
/// (`Europe/Oslo`), or a link of `backward` to one of those
/// (`Asia/Calcutta`), matched without regard to ASCII case. Column 1 is
/// the name of the row that answered: `Asia/Kolkata` for `Asia/Calcutta`.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a name the tables do not place, such as `UTC`,
/// which names no place.
pub fn zone_location(zone: &str, locale: &str) -> Answer<String> {
    let row = location::location(zone.trim()).ok_or(Refusal::Unknown)?;
    let mut out = String::new();
    push_line(&mut out, &row, locale);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    fn cells(line: &str) -> Vec<&str> {
        line.trim_end_matches('\n').split('\t').collect()
    }

    #[test]
    fn the_exemplar_cities_name_the_rows_hc_tz_gives() {
        let rows: Vec<&str> = location::rows().map(|row| row.zone).collect();
        assert_eq!(rows, exemplar_cities::ZONES);
    }

    /// Every row's two cells, multiplied by 3600 and rounded, are its
    /// arcseconds: six places identify the arcsecond.
    #[test]
    fn every_rows_degrees_give_back_its_arcseconds() {
        for row in location::rows() {
            let line = zone_location(row.zone, "en").expect(row.zone);
            let cells = cells(&line);
            assert_eq!(cells[0], row.zone);
            for (cell, arcseconds) in [
                (cells[1], row.coordinates.latitude_arcseconds()),
                (cells[2], row.coordinates.longitude_arcseconds()),
            ] {
                assert_eq!(
                    cell.split_once('.').map(|(_, f)| f.len()),
                    Some(6),
                    "{line}"
                );
                let degrees: f64 = cell.parse().expect("degrees");
                assert_eq!((degrees * 3_600.0).round(), f64::from(arcseconds), "{line}");
            }
        }
    }

    #[test]
    fn every_zone_has_a_line_of_eight_cells_with_a_country_and_a_city() {
        let text = zones("en");
        assert_eq!(text.lines().count(), location::zones().count());
        for line in text.lines() {
            let cells = cells(line);
            assert_eq!(cells.len(), ZONE_COLUMNS, "{line}");
            assert_eq!(cells[4].len(), 2, "{line}");
            assert!(cells[3].split(';').any(|code| code == cells[4]), "{line}");
            assert!(!cells[6].is_empty(), "{line}");
            assert_eq!(cells[7], "en", "{line}");
            let latitude: f64 = cells[1].parse().expect("a latitude");
            let longitude: f64 = cells[2].parse().expect("a longitude");
            assert!(latitude.abs() <= 90.0 && longitude.abs() <= 180.0, "{line}");
        }
    }

    /// `zone1970.tab` 2026c: `JP,AU +353916+1394441 Asia/Tokyo Eyre Bird
    /// Observatory`; 35° 39′ 16″ is 128 356″, 35.654444…°, and 139° 44′ 41″
    /// is 503 081″, 139.744722…°, each written to six places; `zone.tab`
    /// 2026c: `JP +353916+1394441 Asia/Tokyo`, one country.
    #[test]
    fn tokyo_is_its_row_in_decimal_degrees() {
        let line = zone_location("Asia/Tokyo", "en").expect("Tokyo");
        assert_eq!(
            line,
            "Asia/Tokyo\t35.654444\t139.744722\tJP;AU\tJP\tEyre Bird Observatory\tTokyo\ten\n"
        );
    }

    /// `zone1970.tab`: `RU,UA +4457+03406 Europe/Simferopol Crimea`;
    /// `zone.tab`: `UA +4457+03406 Europe/Simferopol Crimea`, in its `RU`
    /// section. Column 5 is `zone.tab`'s, not column 4's first.
    #[test]
    fn simferopol_is_zone_tabs_ua() {
        let line = zone_location("Europe/Simferopol", "en").expect("Simferopol");
        assert_eq!(cells(&line)[3..5], ["RU;UA", "UA"]);
    }

    /// `zone.tab`: `NO +5955+01045 Europe/Oslo`; `backward`: `Link
    /// Asia/Kolkata Asia/Calcutta`, and `Link Etc/UTC UTC`.
    #[test]
    fn a_link_is_answered_by_its_row_and_utc_by_no_place() {
        assert_eq!(
            zone_location("europe/oslo", "en"),
            Ok("Europe/Oslo\t59.916667\t10.750000\tNO\tNO\t\tOslo\ten\n".into())
        );
        let calcutta = zone_location("Asia/Calcutta", "en").expect("Calcutta");
        assert_eq!(cells(&calcutta)[0], "Asia/Kolkata");
        assert_eq!(cells(&calcutta)[4], "IN");
        assert_eq!(cells(&calcutta)[6], "Kolkata");
        assert_eq!(zone_location("UTC", "en"), Err(Refusal::Unknown));
        assert_eq!(
            zone_location("Mars/Olympus_Mons", "en"),
            Err(Refusal::Unknown)
        );
    }

    #[cfg(feature = "localized-exemplar-cities")]
    #[test]
    fn a_locale_names_the_city_and_says_which_data_answered() {
        let city = |zone: &str, locale: &str| {
            let line = zone_location(zone, locale).expect(zone);
            let cells = cells(&line);
            (String::from(cells[6]), String::from(cells[7]))
        };
        assert_eq!(city("Asia/Tokyo", "ja-JP"), ("東京".into(), "ja".into()));
        assert_eq!(city("Europe/Berlin", "de"), ("Berlin".into(), "de".into()));
        assert_eq!(city("Asia/Tokyo", "kab"), ("Tokyo".into(), "en".into()));
        assert_eq!(city("Asia/Tokyo", "native"), ("Tokyo".into(), "en".into()));
        assert_eq!(
            city("Asia/Tokyo", "not a tag"),
            ("Tokyo".into(), "en".into())
        );
        assert!(zones("ja").lines().all(|line| cells(line)[7] == "ja"));
    }

    #[cfg(not(feature = "localized-exemplar-cities"))]
    #[test]
    fn without_the_localized_cities_every_city_is_english() {
        assert_eq!(
            cells(&zone_location("Asia/Tokyo", "ja").expect("Tokyo"))[6..],
            ["Tokyo", "en"]
        );
    }
}
