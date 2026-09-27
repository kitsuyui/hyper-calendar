//! Where each zone is: the principal location the IANA time zone database
//! gives every zone it names, as data.
//!
//! The database keeps, beside its rules, tables that say where a zone's
//! clocks are kept and which names stand for which zone, and this module
//! carries four of its files from release [`TZDATA_RELEASE`], unmodified,
//! in `data/` beside the crate [iana-tzdb-2026c]:
//!
//! * `zone1970.tab` — one row per zone: the countries it overlaps, the
//!   coordinates of its principal location in ISO 6709 form, its name and,
//!   where a country has several zones, a comment telling them apart.
//!   These are the [`zones`], 312 of them.
//! * `zone.tab` — the older table, one row per country and zone, whose
//!   rows can name a link instead of a zone. Its rows for the names
//!   `zone1970.tab` does not list are carried after the zones: 106 of
//!   them, each a link with a place of its own, such as `Europe/Oslo` at
//!   Oslo, where `backward` makes it a link to `Europe/Berlin`.
//! * `backward` — the links from old and merged names to current ones,
//!   such as `Asia/Calcutta` to `Asia/Kolkata`. A link `zone.tab` has no
//!   row for is answered with the row of the name it links to, and where
//!   the file's `#=` comment names the link the old name stands for,
//!   that link: `Iceland` is answered by `Atlantic/Reykjavik`, not by the
//!   `Africa/Abidjan` its data line names.
//! * `backzone` — the zones outside the database's scope, whose links
//!   supersede `backward`'s. Only its `Link` lines are read, so that five
//!   old names stay in their countries: `America/Coral_Harbour` is
//!   answered by `America/Atikokan`, not by `backward`'s `America/Panama`.
//!
//! [`location`] answers any of these names; the system document
//! `docs/systems/zone-locations.md` explains how, with worked examples.
//!
//! # The tables are generated
//!
//! The rows are read from `location/tables.rs`, which the test
//! `the_generated_tables_are_the_vendored_files` renders from the four
//! files and compares with the committed copy: the comment lines are
//! dropped, `zone.tab`'s rows for zones `zone1970.tab` already lists are
//! left out (their coordinates are the same in both files), and every link
//! is followed to a name with a row. To take a new release, replace the
//! files, change [`TZDATA_RELEASE`] and run
//! `UPDATE_ZONE_TABLES=1 cargo test -p hc-tz location`.
//!
//! # Coordinates are exact
//!
//! ISO 6709 writes a coordinate in sexagesimal degrees, minutes and
//! seconds: `+353916` is 35° 39′ 16″ north. [`Coordinates`] holds each as
//! a whole number of arcseconds, which is exact. [`DecimalDegrees`] writes
//! it in decimal degrees to six places, by integer arithmetic: one
//! arcsecond is 0.000278°, so six places identify the arcsecond, and
//! multiplying the text by 3600 and rounding gives it back. [`Coordinates`]
//! also gives each as an `f64`, by one division, `arcseconds / 3600`,
//! whose result is the `f64` nearest the exact quotient. A table value is
//! never read as if it were already decimal: `+4230` is 42.5°, not 42.30°.

use core::fmt;

use crate::error::{TzError, TzResult};

mod tables;

/// The release of the IANA time zone database the vendored tables are
/// from.
pub const TZDATA_RELEASE: &str = "2026c";

/// Where the locations come from, for a `source` cell.
pub const SOURCE: &str =
    "IANA Time Zone Database, release 2026c: zone1970.tab, zone.tab and backward";

/// A point on the Earth as ISO 6709 gives it, in whole arcseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinates {
    latitude: i32,
    longitude: i32,
}

/// 90° in arcseconds.
const MAX_LATITUDE: i32 = 90 * 3_600;
/// 180° in arcseconds.
const MAX_LONGITUDE: i32 = 180 * 3_600;

impl Coordinates {
    /// A point from its latitude, arcseconds north (negative south), and
    /// its longitude, arcseconds east (negative west); `None` outside
    /// ±90° or ±180°.
    #[must_use]
    pub const fn from_arcseconds(latitude: i32, longitude: i32) -> Option<Self> {
        if latitude.abs() > MAX_LATITUDE || longitude.abs() > MAX_LONGITUDE {
            return None;
        }
        Some(Self {
            latitude,
            longitude,
        })
    }

    /// Reads the coordinates column of `zone1970.tab` and `zone.tab`:
    /// ISO 6709's `±DDMM±DDDMM` or `±DDMMSS±DDDMMSS`, the latitude first.
    ///
    /// # Errors
    ///
    /// [`TzError::MalformedCoordinates`] for text in neither form, a
    /// minute or second of 60 or more, or a point beyond ±90° or ±180°.
    pub fn parse_iso6709(text: &str) -> TzResult<Self> {
        let bytes = text.as_bytes();
        // The latitude's sign, two digits of degrees and two or four more;
        // then the longitude's sign, three digits of degrees and as many.
        let latitude_length = match bytes.len() {
            11 => 5,
            15 => 7,
            _ => return Err(TzError::MalformedCoordinates),
        };
        let (latitude, longitude) = bytes.split_at(latitude_length);
        let coordinates = Self::from_arcseconds(
            sexagesimal(latitude, 2).ok_or(TzError::MalformedCoordinates)?,
            sexagesimal(longitude, 3).ok_or(TzError::MalformedCoordinates)?,
        );
        coordinates.ok_or(TzError::MalformedCoordinates)
    }

    /// The latitude in whole arcseconds, positive north.
    #[must_use]
    pub const fn latitude_arcseconds(self) -> i32 {
        self.latitude
    }

    /// The longitude in whole arcseconds, positive east.
    #[must_use]
    pub const fn longitude_arcseconds(self) -> i32 {
        self.longitude
    }

    /// The latitude in decimal degrees, positive north: the `f64` nearest
    /// `arcseconds / 3600`.
    #[must_use]
    pub fn latitude_degrees(self) -> f64 {
        f64::from(self.latitude) / 3_600.0
    }

    /// The longitude in decimal degrees, positive east: the `f64` nearest
    /// `arcseconds / 3600`.
    #[must_use]
    pub fn longitude_degrees(self) -> f64 {
        f64::from(self.longitude) / 3_600.0
    }

    /// The latitude, to be written in decimal degrees to six places.
    #[must_use]
    pub const fn latitude_decimal(self) -> DecimalDegrees {
        DecimalDegrees(self.latitude)
    }

    /// The longitude, to be written in decimal degrees to six places.
    #[must_use]
    pub const fn longitude_decimal(self) -> DecimalDegrees {
        DecimalDegrees(self.longitude)
    }
}

/// An angle of whole arcseconds, written in decimal degrees to six places
/// by integer arithmetic: `arcseconds × 10⁶ / 3600` millionths of a degree,
/// rounded half away from zero, so that 128 356″ is `35.654444` and
/// −84 720″ is `-23.533333`.
///
/// No tie arises: `arcseconds × 10⁶ / 3600` is `arcseconds × 2500 / 9`,
/// whose fraction is a ninth. Six places are within 0.0018″ of the
/// arcseconds, so multiplying the text by 3600 and rounding gives them
/// back.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DecimalDegrees(i32);

impl DecimalDegrees {
    /// The angle in whole arcseconds.
    #[must_use]
    pub const fn arcseconds(self) -> i32 {
        self.0
    }

    /// The angle in millionths of a degree, rounded half away from zero.
    #[must_use]
    pub const fn microdegrees(self) -> i64 {
        let magnitude = (self.0.unsigned_abs() as i64 * 2_500 + 4) / 9;
        if self.0 < 0 { -magnitude } else { magnitude }
    }
}

impl fmt::Display for DecimalDegrees {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let micro = self.microdegrees();
        let sign = if micro < 0 { "-" } else { "" };
        let magnitude = micro.unsigned_abs();
        write!(
            f,
            "{sign}{}.{:06}",
            magnitude / 1_000_000,
            magnitude % 1_000_000
        )
    }
}

/// One signed ISO 6709 angle, `±` then `degree_digits` digits of degrees,
/// two of minutes and optionally two of seconds, in arcseconds.
fn sexagesimal(field: &[u8], degree_digits: usize) -> Option<i32> {
    let (&sign, digits) = field.split_first()?;
    let negative = match sign {
        b'+' => false,
        b'-' => true,
        _ => return None,
    };
    if !digits.iter().all(u8::is_ascii_digit) {
        return None;
    }
    let number = |range: core::ops::Range<usize>| {
        digits.get(range).map(|part| {
            part.iter()
                .fold(0_i32, |value, digit| value * 10 + i32::from(digit - b'0'))
        })
    };
    let degrees = number(0..degree_digits)?;
    let minutes = number(degree_digits..degree_digits + 2)?;
    let seconds = match digits.len() - degree_digits {
        2 => 0,
        4 => number(degree_digits + 2..degree_digits + 4)?,
        _ => return None,
    };
    if minutes >= 60 || seconds >= 60 {
        return None;
    }
    let arcseconds = degrees * 3_600 + minutes * 60 + seconds;
    Some(if negative { -arcseconds } else { arcseconds })
}

/// The table a row is from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocationTable {
    /// `zone1970.tab`: a zone.
    Zone1970,
    /// `zone.tab`: a link with a place of its own, which `zone1970.tab`
    /// does not list.
    ZoneTab,
}

/// A row of the tables: a name and its principal location.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZoneLocation {
    /// The name the row gives, spelled as the table spells it:
    /// `Asia/Tokyo`.
    pub zone: &'static str,
    /// The principal location.
    pub coordinates: Coordinates,
    /// The ISO 3166-1 alpha-2 codes of the countries the zone overlaps,
    /// comma-separated as the table writes them: `JP,AU`. The first is the
    /// country of the principal location; see [`Self::countries`].
    pub country_codes: &'static str,
    /// The table's comment, empty where the row has none. `zone1970.tab`
    /// writes one only where a country has several zones, and it tells
    /// that country's zones apart.
    pub comment: &'static str,
    /// The table the row is from.
    pub table: LocationTable,
    /// The row's position among [`rows`]: the zones first, in
    /// `zone1970.tab`'s order, then `zone.tab`'s rows in its order.
    pub row: usize,
}

impl ZoneLocation {
    /// The countries of [`Self::country_codes`], one code each.
    pub fn countries(&self) -> impl Iterator<Item = &'static str> {
        self.country_codes.split(',')
    }
}

/// Every row, the [`zones`] then the rows of `zone.tab` for the names
/// `zone1970.tab` does not list.
pub fn rows() -> impl Iterator<Item = ZoneLocation> {
    tables::ROWS
        .lines()
        .enumerate()
        .filter_map(|(row, line)| parse_row(line, row))
}

/// Every zone, one per row of `zone1970.tab`, in its order: by country
/// code, then within a country as the table orders it.
pub fn zones() -> impl Iterator<Item = ZoneLocation> {
    rows().take(tables::ZONE1970_ROWS)
}

/// The row of a name in [`rows`], matched without regard to ASCII case.
#[must_use]
pub fn row_of(name: &str) -> Option<ZoneLocation> {
    rows().find(|row| row.zone.eq_ignore_ascii_case(name))
}

/// The name with a row that a link of `backward` is answered by:
/// `Asia/Kolkata` for `Asia/Calcutta`, matched without regard to ASCII
/// case. `None` for a name that is not such a link — a name with a row of
/// its own, and a link to a zone with no location, such as `UTC` to
/// `Etc/UTC`.
#[must_use]
pub fn link_target(name: &str) -> Option<&'static str> {
    tables::LINKS.lines().find_map(|line| {
        let (link, target) = line.split_once('\t')?;
        link.eq_ignore_ascii_case(name).then_some(target)
    })
}

/// Where a zone is: the row of `name`, or of the name a link of
/// `backward` leads to, matched without regard to ASCII case.
///
/// The row answers with its own [`ZoneLocation::zone`], so a caller can
/// tell a link from the zone that answered it: `Asia/Calcutta` is answered
/// by `Asia/Kolkata`'s row. `None` for a name neither table nor `backward`
/// places, such as `UTC` or `Etc/GMT+5`, which name no place.
#[must_use]
pub fn location(name: &str) -> Option<ZoneLocation> {
    row_of(name).or_else(|| link_target(name).and_then(row_of))
}

/// One line of [`tables::ROWS`]: `countries\tcoordinates\tname` and an
/// optional `\tcomment`.
fn parse_row(line: &'static str, row: usize) -> Option<ZoneLocation> {
    let mut cells = line.split('\t');
    let country_codes = cells.next()?;
    let coordinates = Coordinates::parse_iso6709(cells.next()?).ok()?;
    let zone = cells.next()?;
    let comment = cells.next().unwrap_or("");
    let table = if row < tables::ZONE1970_ROWS {
        LocationTable::Zone1970
    } else {
        LocationTable::ZoneTab
    };
    Some(ZoneLocation {
        zone,
        coordinates,
        country_codes,
        comment,
        table,
        row,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const ZONE1970: &str = include_str!("../data/zone1970.tab");
    const ZONE_TAB: &str = include_str!("../data/zone.tab");
    const BACKWARD: &str = include_str!("../data/backward");
    const BACKZONE: &str = include_str!("../data/backzone");

    /// The data lines of a `.tab` file, split into cells.
    fn tab_rows(file: &'static str) -> Vec<Vec<&'static str>> {
        file.lines()
            .filter(|line| !line.starts_with('#'))
            .map(|line| line.split('\t').collect())
            .collect()
    }

    /// Every `Link` line of `backward`: the link, the target the data
    /// names, and the target its `#=` comment names, if it has one.
    fn backward_links() -> Vec<(&'static str, &'static str, Option<&'static str>)> {
        BACKWARD
            .lines()
            .filter(|line| line.starts_with("Link"))
            .map(|line| {
                let (data, comment) = match line.split_once('#') {
                    Some((data, comment)) => (data, Some(comment)),
                    None => (line, None),
                };
                let fields: Vec<&str> = data.split_whitespace().collect();
                assert_eq!(fields.len(), 3, "{line}");
                let intended = comment
                    .and_then(|comment| comment.strip_prefix("= "))
                    .map(str::trim);
                (fields[2], fields[1], intended)
            })
            .collect()
    }

    /// The links of `backzone`, each as the link and its target: its
    /// `Link` lines, and the `#PACKRATLIST zone.tab Link` lines it writes
    /// for a link that stays within a country only when the database is
    /// built with `PACKRATLIST=zone.tab`.
    fn backzone_links() -> Vec<(&'static str, &'static str)> {
        BACKZONE
            .lines()
            .filter_map(|line| {
                line.strip_prefix("#PACKRATLIST zone.tab ")
                    .unwrap_or(line)
                    .strip_prefix("Link")
            })
            .map(|rest| {
                let fields: Vec<&str> = rest.split_whitespace().collect();
                assert_eq!(fields.len(), 2, "{rest}");
                (fields[1], fields[0])
            })
            .collect()
    }

    /// `location/tables.rs` as the vendored files make it.
    fn render() -> String {
        let zones = tab_rows(ZONE1970);
        let zone_names: Vec<&str> = zones.iter().map(|cells| cells[2]).collect();
        let extra: Vec<Vec<&str>> = tab_rows(ZONE_TAB)
            .into_iter()
            .filter(|cells| !zone_names.contains(&cells[2]))
            .collect();
        let named: Vec<&str> = zone_names
            .iter()
            .copied()
            .chain(extra.iter().map(|cells| cells[2]))
            .collect();
        let links = backward_links();
        let backzone = backzone_links();
        // A link is answered by the target `backzone` gives it, where it
        // gives one, which keeps it within its country; else by the name
        // its `#=` comment in `backward` gives, else by its target there;
        // followed through further links until a name with a row.
        let resolve = |link: &'static str| -> Option<&'static str> {
            let mut name = link;
            for _ in 0..8 {
                if named.contains(&name) {
                    return Some(name);
                }
                name = match backzone.iter().find(|entry| entry.0 == name) {
                    Some((_, target)) => target,
                    None => {
                        let (_, target, intended) = links.iter().find(|entry| entry.0 == name)?;
                        intended.unwrap_or(target)
                    }
                };
            }
            panic!("the links from {link} do not end");
        };
        let literal = |line: &str| {
            let mut out = String::from("    \"");
            for character in line.chars() {
                match character {
                    '\t' => out.push_str("\\t"),
                    '"' => out.push_str("\\\""),
                    '\\' => out.push_str("\\\\"),
                    other => out.push(other),
                }
            }
            out.push_str("\\n\",\n");
            out
        };
        let mut out = String::from(
            "//! The rows of `data/zone1970.tab` and `data/zone.tab`, and the links of\n\
             //! `data/backward` with their targets from `data/backzone`, as `super`\n\
             //! reads them.\n\
             //!\n\
             //! Generated from the four files by the test\n\
             //! `the_generated_tables_are_the_vendored_files` in `location.rs`;\n\
             //! `UPDATE_ZONE_TABLES=1 cargo test -p hc-tz location` rewrites it. Do not\n\
             //! edit it by hand.\n\n",
        );
        out.push_str(&format!(
            "/// How many of [`ROWS`] are `zone1970.tab`'s, which come first.\n\
             pub(super) const ZONE1970_ROWS: usize = {};\n\n",
            zones.len()
        ));
        out.push_str(
            "/// `zone1970.tab`'s rows, then `zone.tab`'s rows for the names\n\
             /// `zone1970.tab` does not list, each as the file writes it.\n\
             pub(super) const ROWS: &str = concat!(\n",
        );
        for cells in zones.iter().chain(&extra) {
            out.push_str(&literal(&cells.join("\t")));
        }
        out.push_str(");\n\n");
        out.push_str(
            "/// `backward`'s links without a row, each as `link\\ttarget`, the target\n\
             /// being the name with a row the link is answered by. A link that\n\
             /// leads to no such name, such as `UTC`, is left out.\n\
             pub(super) const LINKS: &str = concat!(\n",
        );
        for (link, _, _) in &links {
            if named.contains(link) {
                continue;
            }
            if let Some(target) = resolve(link) {
                out.push_str(&literal(&format!("{link}\t{target}")));
            }
        }
        out.push_str(");\n");
        out
    }

    #[cfg(feature = "std")]
    #[test]
    fn the_generated_tables_are_the_vendored_files() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/location/tables.rs");
        let rendered = render();
        if std::env::var_os("UPDATE_ZONE_TABLES").is_some() {
            std::fs::write(path, &rendered).expect("write the tables");
            return;
        }
        let committed = std::fs::read_to_string(path).expect("read the tables");
        assert!(
            committed == rendered,
            "{path} is not what the files in data/ make; run UPDATE_ZONE_TABLES=1 cargo test -p hc-tz location"
        );
    }

    #[test]
    fn every_row_parses_and_the_counts_are_the_files() {
        assert_eq!(rows().count(), tables::ROWS.lines().count());
        assert_eq!(zones().count(), tab_rows(ZONE1970).len());
        assert_eq!(zones().count(), 312);
        assert_eq!(rows().count(), 312 + 106);
        for (index, row) in rows().enumerate() {
            assert_eq!(row.row, index);
            assert!(
                row.countries()
                    .all(|code| code.len() == 2 && code.bytes().all(|b| b.is_ascii_uppercase())),
                "{}",
                row.zone
            );
            if row.table == LocationTable::ZoneTab {
                assert_eq!(row.countries().count(), 1, "{}", row.zone);
            }
        }
    }

    #[test]
    fn no_name_has_two_rows_or_is_both_a_row_and_a_link() {
        let mut names: Vec<String> = rows().map(|row| row.zone.to_ascii_lowercase()).collect();
        let count = names.len();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), count);
        for line in tables::LINKS.lines() {
            let (link, target) = line.split_once('\t').expect("a link");
            assert!(row_of(link).is_none(), "{link}");
            assert!(row_of(target).is_some(), "{link} → {target}");
        }
    }

    /// The coordinates of `zone.tab` for a zone `zone1970.tab` also lists
    /// are the same, which is what lets the generated tables leave those
    /// rows out.
    #[test]
    fn zone_tab_agrees_with_zone1970_on_every_zone_both_list() {
        for cells in tab_rows(ZONE_TAB) {
            if let Some(zone) = zones().find(|zone| zone.zone == cells[2]) {
                assert_eq!(
                    Coordinates::parse_iso6709(cells[1]),
                    Ok(zone.coordinates),
                    "{}",
                    cells[2]
                );
            }
        }
    }

    /// Worked by hand from the rows of `zone1970.tab` 2026c:
    ///
    /// * `JP,AU +353916+1394441 Asia/Tokyo`: 35° 39′ 16″ is
    ///   35 × 3600 + 39 × 60 + 16 = 128 356″, and 139° 44′ 41″ is
    ///   500 400 + 2 640 + 41 = 503 081″.
    /// * `AD +4230+00131 Europe/Andorra`: 42° 30′ is 153 000″, 42.5°, and
    ///   1° 31′ is 5 460″, 1.51666…°.
    /// * `AQ -720041+0023206 Antarctica/Troll`: 72° 0′ 41″ south is
    ///   −259 241″, and 2° 32′ 6″ east is 7 200 + 1 920 + 6 = 9 126″,
    ///   exactly 2.535°.
    /// * `US +404251-0740023 America/New_York`: 40° 42′ 51″ is 146 571″,
    ///   and 74° 0′ 23″ west is −266 423″.
    /// * `BR -2332-04637 America/Sao_Paulo`: −(82 800 + 1 920) = −84 720″
    ///   and −(165 600 + 2 220) = −167 820″.
    #[test]
    fn coordinates_are_read_as_degrees_minutes_and_seconds() {
        let at = |zone: &str| location(zone).expect(zone).coordinates;
        let tokyo = at("Asia/Tokyo");
        assert_eq!(
            (tokyo.latitude_arcseconds(), tokyo.longitude_arcseconds()),
            (128_356, 503_081)
        );
        assert_eq!(tokyo.latitude_degrees(), 128_356.0 / 3_600.0);
        assert_eq!(
            format!("{}", tokyo.latitude_degrees()),
            "35.654444444444444"
        );
        assert_eq!(
            format!("{}", tokyo.longitude_degrees()),
            "139.7447222222222"
        );

        let andorra = at("Europe/Andorra");
        assert_eq!(
            (
                andorra.latitude_arcseconds(),
                andorra.longitude_arcseconds()
            ),
            (153_000, 5_460)
        );
        assert_eq!(andorra.latitude_degrees(), 42.5);
        assert_eq!(
            format!("{}", andorra.longitude_degrees()),
            "1.5166666666666666"
        );

        let troll = at("Antarctica/Troll");
        assert_eq!(
            (troll.latitude_arcseconds(), troll.longitude_arcseconds()),
            (-259_241, 9_126)
        );
        assert_eq!(troll.longitude_degrees(), 2.535);
        assert_eq!(
            format!("{}", troll.latitude_degrees()),
            "-72.01138888888889"
        );

        let new_york = at("America/New_York");
        assert_eq!(
            (
                new_york.latitude_arcseconds(),
                new_york.longitude_arcseconds()
            ),
            (146_571, -266_423)
        );
        assert_eq!(
            format!("{}", new_york.longitude_degrees()),
            "-74.00638888888889"
        );

        let sao_paulo = at("America/Sao_Paulo");
        assert_eq!(
            (
                sao_paulo.latitude_arcseconds(),
                sao_paulo.longitude_arcseconds()
            ),
            (-84_720, -167_820)
        );
    }

    /// The arcseconds a six-place decimal gives back: millionths of a
    /// degree times 3600 / 10⁶, rounded.
    fn arcseconds_of(text: &str) -> i64 {
        let (negative, digits) = match text.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, text),
        };
        let (whole, fraction) = digits.split_once('.').expect("a point");
        assert_eq!(fraction.len(), 6, "{text}");
        let micro: i64 = whole.parse::<i64>().expect("degrees") * 1_000_000
            + fraction.parse::<i64>().expect("millionths");
        let arcseconds = (micro * 9 + 1_250) / 2_500;
        if negative { -arcseconds } else { arcseconds }
    }

    /// `zone1970.tab`'s Tokyo, 128 356″ and 503 081″: 35.6544444…° and
    /// 139.7447222…°; São Paulo's −84 720″, −23.5333333…°; Andorra's
    /// 153 000″, 42.5°; and the ends of the range.
    #[test]
    fn decimal_degrees_are_six_places_rounded_half_away_from_zero() {
        let tokyo = location("Asia/Tokyo").expect("Tokyo").coordinates;
        assert_eq!(tokyo.latitude_decimal().to_string(), "35.654444");
        assert_eq!(tokyo.longitude_decimal().to_string(), "139.744722");
        let sao_paulo = location("America/Sao_Paulo")
            .expect("São Paulo")
            .coordinates;
        assert_eq!(sao_paulo.latitude_decimal().to_string(), "-23.533333");
        assert_eq!(sao_paulo.longitude_decimal().to_string(), "-46.616667");
        let andorra = location("Europe/Andorra").expect("Andorra").coordinates;
        assert_eq!(andorra.latitude_decimal().to_string(), "42.500000");
        assert_eq!(andorra.longitude_decimal().to_string(), "1.516667");
        for (arcseconds, text) in [
            (0, "0.000000"),
            (1, "0.000278"),
            (-1, "-0.000278"),
            (648_000, "180.000000"),
            (-324_000, "-90.000000"),
        ] {
            assert_eq!(DecimalDegrees(arcseconds).to_string(), text);
        }
    }

    #[test]
    fn every_rows_decimal_degrees_give_back_its_arcseconds() {
        for row in rows() {
            for angle in [
                row.coordinates.latitude_decimal(),
                row.coordinates.longitude_decimal(),
            ] {
                let text = angle.to_string();
                assert_eq!(
                    arcseconds_of(&text),
                    i64::from(angle.arcseconds()),
                    "{} {text}",
                    row.zone
                );
            }
        }
        // And every angle a coordinate can hold.
        for arcseconds in -648_000..=648_000 {
            let text = DecimalDegrees(arcseconds).to_string();
            assert_eq!(arcseconds_of(&text), i64::from(arcseconds), "{text}");
        }
    }

    #[test]
    fn malformed_coordinates_are_refused() {
        for text in [
            "",
            "+4230+0013",
            "+4230+001310",
            "+4260+00131",
            "+4230+00160",
            "+423060+0013100",
            "+9001+00000",
            "+0000+18001",
            "*4230+00131",
            "+42a0+00131",
            "+4230 00131",
            "+4230+00131\n",
        ] {
            assert_eq!(
                Coordinates::parse_iso6709(text),
                Err(TzError::MalformedCoordinates),
                "{text:?}"
            );
        }
        assert_eq!(
            Coordinates::parse_iso6709("-9000+18000"),
            Ok(Coordinates::from_arcseconds(-324_000, 648_000).expect("in range"))
        );
        assert_eq!(Coordinates::from_arcseconds(324_001, 0), None);
    }

    #[test]
    fn a_zone_answers_with_its_own_row_and_its_columns() {
        let tokyo = location("asia/TOKYO").expect("Tokyo");
        assert_eq!(tokyo.zone, "Asia/Tokyo");
        assert_eq!(tokyo.country_codes, "JP,AU");
        assert_eq!(tokyo.countries().collect::<Vec<_>>(), ["JP", "AU"]);
        assert_eq!(tokyo.comment, "Eyre Bird Observatory");
        assert_eq!(tokyo.table, LocationTable::Zone1970);
        assert_eq!(location("Europe/Andorra").expect("Andorra").comment, "");
    }

    /// `zone.tab`: `NO +5955+01045 Europe/Oslo`, where `backward` has
    /// `Link Europe/Berlin Europe/Oslo`: the place is Oslo's, not Berlin's.
    #[test]
    fn a_link_with_a_row_of_its_own_answers_with_that_row() {
        let oslo = location("Europe/Oslo").expect("Oslo");
        assert_eq!(oslo.zone, "Europe/Oslo");
        assert_eq!(oslo.table, LocationTable::ZoneTab);
        assert_eq!(oslo.country_codes, "NO");
        assert_eq!(
            oslo.coordinates,
            Coordinates::parse_iso6709("+5955+01045").expect("Oslo")
        );
        assert_eq!(link_target("Europe/Oslo"), None);
    }

    /// `backward`: `Link Asia/Kolkata Asia/Calcutta`; `Link Africa/Abidjan
    /// Iceland #= Atlantic/Reykjavik`, whose comment names the link
    /// `zone.tab` places at Reykjavík (`IS +6409-02151`); `Link
    /// Australia/Sydney Australia/ACT #= Australia/Canberra`, where
    /// `Australia/Canberra` is itself a link to `Australia/Sydney`; and
    /// `Link Etc/UTC UTC`, which names no place.
    #[test]
    fn a_link_is_answered_by_the_row_it_leads_to() {
        assert_eq!(link_target("Asia/Calcutta"), Some("Asia/Kolkata"));
        assert_eq!(
            location("asia/calcutta").map(|row| row.zone),
            Some("Asia/Kolkata")
        );
        assert_eq!(link_target("Iceland"), Some("Atlantic/Reykjavik"));
        let reykjavik = location("Iceland").expect("Iceland");
        assert_eq!(reykjavik.zone, "Atlantic/Reykjavik");
        assert_eq!(reykjavik.country_codes, "IS");
        assert_eq!(link_target("Australia/ACT"), Some("Australia/Sydney"));
        assert_eq!(
            link_target("America/Buenos_Aires"),
            Some("America/Argentina/Buenos_Aires")
        );
        // `backzone`: `#PACKRATLIST zone.tab Link America/Atikokan
        // America/Coral_Harbour` and `Link Pacific/Chuuk Pacific/Yap`,
        // where `backward` links them to Panama and Port Moresby.
        assert_eq!(
            link_target("America/Coral_Harbour"),
            Some("America/Atikokan")
        );
        assert_eq!(link_target("Pacific/Yap"), Some("Pacific/Chuuk"));
        assert_eq!(location("UTC"), None);
        assert_eq!(location("Etc/GMT+5"), None);
        assert_eq!(location("Mars/Olympus_Mons"), None);
        assert_eq!(tables::LINKS.lines().count(), 135);
    }
}
