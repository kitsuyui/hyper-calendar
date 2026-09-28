//! The tab-separated lines the WebAssembly module and the C library write
//! about where time zones are, and what offset they keep, written once.
//!
//! [`zone_offset`] is the offset, daylight flag, abbreviation and next
//! transition of one zone at an instant, from whichever rules the boundary
//! crate chose for the name — the rules its day in the zone comes from.
//!
//! The zones a caller loads as TZif bytes are kept here too, in the one
//! piece of state that outlives a call (`docs/policy.md` §13): [`load_zone`]
//! keeps them, and [`with_zone`] reads a name from them before the built-in
//! table, for the day, the start of a day, the offset and a radio frame's
//! summer time alike.
//!
//! A line is one row of [`hc_tz::location`]: the zone, the latitude and
//! longitude of its principal location in decimal degrees, its countries,
//! the one country `zone.tab` lists it under, the table's comment, and the zone's exemplar city in a locale with the
//! tag of the data that answered, from [`hc_i18n::exemplar_cities`]. The
//! cities are English unless the build has the `localized-exemplar-cities`
//! feature; `docs/systems/zone-locations.md` explains both halves.

use alloc::string::String;

use hc_calendar::{Rd, gregorian};
use hc_core::UnixTime;
use hc_i18n::exemplar_cities::{self, ExemplarCity};
use hc_tz::TimeZone;
use hc_tz::location::{self, ZoneLocation};

use crate::boundary::{Answer, Line, Refusal};

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
    let city = city(row, locale);
    let mut line = Line::new(out);
    line.cell(row.zone)
        .value(row.coordinates.latitude_decimal())
        .value(row.coordinates.longitude_decimal())
        .cell_with(|cell| {
            for (index, country) in row.countries().enumerate() {
                let separator = if index > 0 { ";" } else { "" };
                let _ = write!(cell, "{separator}{country}");
            }
        })
        .cell(row.zone_tab_country)
        .cell(row.comment)
        .value(city.name)
        .cell(city.tag);
    line.end();
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
/// for a name `zone.tab` has no row for, which no name of release 2026d
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

/// Which rules answered for a zone's name: the boundary crate's built-in
/// POSIX footers, [`hc_tz::builtin`], or TZif bytes the caller gave it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ZoneRules {
    /// The built-in table's POSIX `TZ` string, the zone's current rules
    /// applied to every year.
    Builtin,
    /// A TZif file the caller loaded under the name, which takes
    /// precedence over a built-in zone of the same name.
    Loaded,
}

impl ZoneRules {
    /// The name [`zone_offset`] writes: `builtin` or `loaded`.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Builtin => "builtin",
            Self::Loaded => "loaded",
        }
    }
}

/// How many columns [`zone_offset`] writes.
pub const ZONE_OFFSET_COLUMNS: usize = 6;

/// An instant a zone's rules answer for: the seconds of the years
/// [`hc_tz::posix::FIRST_RULE_YEAR`] to [`hc_tz::posix::LAST_RULE_YEAR`],
/// −9 999 994 to 9 999 994 by UTC, whichever rules answer for the name.
///
/// A POSIX rule outside them answers standard time with no transition,
/// whatever it says, and a loaded file reads its footer's rule after its
/// last transition, so every zone is held to the same years and the range
/// does not depend on the name.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for an instant outside them.
pub const fn instant_in_zone_range(unix_seconds: i64) -> Answer<UnixTime> {
    let instant = UnixTime::from_seconds(unix_seconds);
    if hc_tz::posix::rules_answer_at(instant) {
        Ok(instant)
    } else {
        Err(Refusal::OutOfRange)
    }
}

/// A fixed day whose start a zone's rules answer for: a day of the years
/// [`hc_tz::posix::FIRST_RULE_YEAR`] to [`hc_tz::posix::LAST_RULE_YEAR`],
/// as for [`instant_in_zone_range`].
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a day outside them.
pub const fn day_in_zone_range(fixed: i64) -> Answer<Rd> {
    let first = gregorian::new_year(hc_tz::posix::FIRST_RULE_YEAR).0;
    let after = gregorian::new_year(hc_tz::posix::LAST_RULE_YEAR + 1).0;
    if fixed >= first && fixed < after {
        Ok(Rd(fixed))
    } else {
        Err(Refusal::OutOfRange)
    }
}

/// An abbreviation as the zone's rules give it, or empty where they give
/// a number: tzdata writes `-03` or `+0545` "if there is no common English
/// abbreviation" (Theory and pragmatics of the tz code and data, "Time
/// zone abbreviations", `iana-tz-theory`), and such a cell would only
/// repeat column 1.
fn named_abbreviation(abbreviation: Option<&str>) -> &str {
    abbreviation
        .filter(|text| !text.starts_with(['+', '-']))
        .unwrap_or("")
}

/// The line of `hc_zone_offset`: a zone's offset at an instant, from the
/// rules the caller's name selected. Tab-separated: the offset, seconds
/// east of UTC; `1` when the rules call it daylight saving or summer time,
/// else `0`; the abbreviation the rules give, `CET` or `MDT`, empty where
/// they give a numeric one such as `+0545`; the POSIX second of the next
/// transition, the first after `unix_seconds` at which the offset, the flag
/// or the abbreviation changes, and the offset after it, both empty where
/// the rules have none; and which rules answered, `builtin` or `loaded`.
///
/// The offset, flag and abbreviation are the zone's `offset_at`,
/// `is_dst_at` and `abbreviation_at`, the functions its day is read from,
/// so `unix_seconds` plus column 1, floored to the day, is the day
/// `hc_fixed_from_unix_in_zone` gives. The next transition is
/// [`TimeZone::next_transition`]'s.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for an instant outside
/// [`instant_in_zone_range`]'s years.
pub fn zone_offset(zone: &dyn TimeZone, rules: ZoneRules, unix_seconds: i64) -> Answer<String> {
    let instant = instant_in_zone_range(unix_seconds)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(zone.offset_at(instant).seconds())
        .flag(zone.is_dst_at(instant))
        .cell(named_abbreviation(zone.abbreviation_at(instant)));
    if let Some(next) = zone.next_transition(instant) {
        line.value(next.seconds())
            .value(zone.offset_at(next).seconds());
    } else {
        line.empties(2);
    }
    line.cell(rules.name());
    line.end();
    Ok(out)
}

/// The zones a caller has handed the boundary as TZif bytes, by name.
///
/// Behind a mutex because a `static` has to be: a WebAssembly instance is
/// single-threaded, and a C caller may call from several threads. The
/// bytes are kept and parsed again on every call, which is a few
/// microseconds against the cost of owning a borrowed parse across the
/// boundary.
#[cfg(feature = "std")]
static LOADED: std::sync::Mutex<alloc::vec::Vec<(String, alloc::vec::Vec<u8>)>> =
    std::sync::Mutex::new(alloc::vec::Vec::new());

/// Keep TZif bytes under a zone's name, replacing any kept under a name
/// that matches it. The bytes are copied, and nothing is kept on failure.
///
/// # Errors
///
/// [`Refusal::Malformed`] for bytes that are not a TZif file.
#[cfg(feature = "std")]
pub fn load_zone(name: &str, bytes: &[u8]) -> Answer<()> {
    hc_tz::TzifTimeZone::parse(name, bytes).map_err(|_| Refusal::Malformed)?;
    let mut loaded = LOADED
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(slot) = loaded
        .iter_mut()
        .find(|(known, _)| hc_core::catalogue::matches(name, known))
    {
        slot.1 = bytes.to_vec();
    } else {
        loaded.push((String::from(name), bytes.to_vec()));
    }
    Ok(())
}

/// The zone a name selects — one loaded through [`load_zone`] first,
/// because a caller that supplied the IANA data for a name wants that and
/// not the built-in approximation, then the built-in table — handed to
/// `answer` with which of the two it is. Every export that reads a zone by
/// name reads it here, so that the day, the offset and a radio frame's
/// summer time come from the same rules.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a name neither knows.
#[cfg(feature = "std")]
pub fn with_zone<R>(name: &str, answer: impl FnOnce(&dyn TimeZone, ZoneRules) -> R) -> Answer<R> {
    let loaded = LOADED
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some((known, bytes)) = loaded
        .iter()
        .find(|(known, _)| hc_core::catalogue::matches(name, known))
    {
        // Validated when it was loaded; a failure here is impossible, and
        // would be reported as unknown rather than trapped on.
        let zone = hc_tz::TzifTimeZone::parse(known, bytes).map_err(|_| Refusal::Unknown)?;
        return Ok(answer(&zone, ZoneRules::Loaded));
    }
    drop(loaded);
    let zone = hc_tz::builtin::zone(name).map_err(|_| Refusal::Unknown)?;
    Ok(answer(&zone, ZoneRules::Builtin))
}

/// The fixed day a POSIX timestamp falls on by the wall clock of the zone
/// a name selects, as [`with_zone`] reads it.
///
/// # Errors
///
/// As [`with_zone`], and [`Refusal::OutOfRange`] for an instant outside
/// [`instant_in_zone_range`]'s years.
#[cfg(feature = "std")]
pub fn day_in_zone(unix_seconds: i64, name: &str) -> Answer<i64> {
    with_zone(name, |zone, _| {
        let instant = instant_in_zone_range(unix_seconds)?;
        zone.local_at(instant)
            .map(|local| local.day.0)
            .map_err(|_| Refusal::OutOfRange)
    })?
}

/// The POSIX timestamp at which a fixed day begins by the wall clock of the
/// zone a name selects: its midnight, or the first instant after a gap
/// that swallows it, or the earlier of two midnights when the clocks fall
/// back across it.
///
/// # Errors
///
/// As [`with_zone`], and [`Refusal::OutOfRange`] for a day outside
/// [`day_in_zone_range`]'s years.
#[cfg(feature = "std")]
pub fn start_in_zone(fixed: i64, name: &str) -> Answer<i64> {
    use hc_calendar::CivilDateTime;
    use hc_tz::LocalResolution;
    with_zone(name, |zone, _| {
        let day = day_in_zone_range(fixed)?;
        let instant = match zone.resolve_local(CivilDateTime::midnight(day)) {
            LocalResolution::Unambiguous(instant) => instant,
            LocalResolution::Ambiguous { earlier, .. } => earlier,
            LocalResolution::Nonexistent { after_gap, .. } => after_gap,
        };
        Ok(instant.seconds())
    })?
}

/// The line of `hc_zone_offset`: [`zone_offset`] for the zone a name
/// selects, from the rules [`with_zone`] reads.
///
/// # Errors
///
/// As [`with_zone`] and [`zone_offset`].
#[cfg(feature = "std")]
pub fn zone_offset_line(name: &str, unix_seconds: i64) -> Answer<String> {
    with_zone(name, |zone, rules| zone_offset(zone, rules, unix_seconds))?
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;
    use hc_tz::{PosixTimeZone, builtin};

    #[test]
    fn a_zones_offset_line_is_its_rules() {
        let berlin = builtin::zone("Europe/Berlin").expect("Berlin");
        // 2026-03-29 01:00 UTC, when CET becomes CEST.
        assert_eq!(
            zone_offset(&berlin, ZoneRules::Builtin, 1_774_745_999).expect("in range"),
            "3600\t0\tCET\t1774746000\t7200\tbuiltin\n"
        );
        assert_eq!(
            zone_offset(&berlin, ZoneRules::Builtin, 1_774_746_000).expect("in range"),
            "7200\t1\tCEST\t1792890000\t3600\tbuiltin\n"
        );
        let denver =
            PosixTimeZone::parse("America/Denver", "MST7MDT,M3.2.0,M11.1.0").expect("Denver");
        assert_eq!(
            zone_offset(&denver, ZoneRules::Loaded, 1_772_960_400).expect("in range"),
            "-21600\t1\tMDT\t1793520000\t-25200\tloaded\n"
        );
        let kathmandu = builtin::zone("Asia/Kathmandu").expect("Kathmandu");
        assert_eq!(
            zone_offset(&kathmandu, ZoneRules::Builtin, 0).expect("in range"),
            "20700\t0\t\t\t\tbuiltin\n"
        );
        let line = zone_offset(&builtin::zone("UTC").expect("UTC"), ZoneRules::Builtin, 0)
            .expect("in range");
        assert_eq!(cells(&line).len(), ZONE_OFFSET_COLUMNS);
        assert_eq!(line, "0\t0\tUTC\t\t\tbuiltin\n");
    }

    /// The first and the last second the rules answer for, and a second
    /// beyond each, which is refused rather than answered in standard time.
    #[test]
    fn a_zones_offset_is_refused_beyond_the_years_its_rules_answer_for() {
        use hc_tz::posix::{FIRST_RULE_SECOND, LAST_RULE_SECOND};
        let sydney = builtin::zone("Australia/Sydney").expect("Sydney");
        for unix in [FIRST_RULE_SECOND, LAST_RULE_SECOND] {
            let line = zone_offset(&sydney, ZoneRules::Builtin, unix).expect("in range");
            assert!(line.starts_with("39600\t1\tAEDT\t"), "{line}");
        }
        for unix in [
            FIRST_RULE_SECOND - 1,
            LAST_RULE_SECOND + 1,
            i64::MIN,
            i64::MAX,
        ] {
            assert_eq!(
                zone_offset(&sydney, ZoneRules::Builtin, unix),
                Err(Refusal::OutOfRange)
            );
            assert_eq!(instant_in_zone_range(unix), Err(Refusal::OutOfRange));
        }
        let first = gregorian::new_year(hc_tz::posix::FIRST_RULE_YEAR).0;
        let last = gregorian::new_year(hc_tz::posix::LAST_RULE_YEAR + 1).0 - 1;
        assert_eq!(day_in_zone_range(first), Ok(Rd(first)));
        assert_eq!(day_in_zone_range(last), Ok(Rd(last)));
        assert_eq!(day_in_zone_range(first - 1), Err(Refusal::OutOfRange));
        assert_eq!(day_in_zone_range(last + 1), Err(Refusal::OutOfRange));
    }

    use crate::boundary::cells;

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

    /// `zone1970.tab` 2026d: `JP,AU +353916+1394441 Asia/Tokyo Eyre Bird
    /// Observatory`; 35° 39′ 16″ is 128 356″, 35.654444…°, and 139° 44′ 41″
    /// is 503 081″, 139.744722…°, each written to six places; `zone.tab`
    /// 2026d: `JP +353916+1394441 Asia/Tokyo`, one country.
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
