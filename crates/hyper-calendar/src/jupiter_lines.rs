//! The tab-separated lines the WebAssembly module's and the C library's
//! `jupiter` layer write, written once: where Jupiter is, when it changes
//! sign, and the two festivals it sets, the Kumbh Mela and the Pushkaram, with
//! Jupiter's sign **computed** from the VSOP87B series of
//! [`hc_astro::jupiter`] where [`crate::reckoning_lines`]'s `hc_kumbh` and
//! `hc_pushkaram` take it from the caller.
//!
//! The sidereal sign is Jupiter's apparent geocentric longitude in the
//! true equinox of the date less the ayanāṃśa
//! ([`hc_seasons::zodiac::jupiter`]); `docs/systems/jupiter-festivals.md` has
//! what it agrees with and what it costs. The range is the sky layer's,
//! [`crate::astro_lines`]: the years −1000 to 3000.

use alloc::string::String;

use hc_astro::riseset::Location;
use hc_calendar::fixed::Moment;
use hc_calendar::gregorian::new_year;
use hc_calendars_indic::barhaspatya::TWELVE_YEAR_NAMES;
use hc_calendars_indic::kumbh::KumbhYoga;
use hc_calendars_indic::nakshatra::DEGREES_PER_NAKSHATRA;
use hc_calendars_indic::pushkaram::{adi_pushkaram, rivers_of};
use hc_seasons::meridian::Meridian;
use hc_seasons::zodiac::SiderealSign;
use hc_seasons::zodiac::jupiter::{self, EntryRule};

use crate::astro_lines::{EARLIEST_YEAR, LATEST_YEAR, moment_in_era, unix_from_moment};
use crate::boundary::{Answer, Line, Refusal};
use crate::panchanga_lines::{ayanamsa, nakshatra_cells};
use crate::reckoning_lines::{kumbh_condition_cells, pushkaram_river_cells, sign};
use crate::season_lines::meridian;

/// The longest span [`ingress_lines`] accepts, in seconds: a hundred Julian
/// years. Each ingress is a search of a few hundred positions.
pub const MAX_INGRESS_SPAN_SECONDS: i64 = 100 * 31_557_600;

/// How many columns [`jupiter_line`] writes.
pub const JUPITER_COLUMNS: usize = 12;

/// The line of `hc_jupiter_at`: where Jupiter is at a POSIX instant.
///
/// The cells: the apparent geocentric ecliptic longitude in degrees, in the
/// true equinox of the date, the tropical one; the latitude in degrees; the
/// distance in astronomical units; the sidereal longitude in degrees by the
/// ayanāṃśa; the sidereal sign by its identifier and its Sanskrit name; how
/// many degrees into it; the longitude's change in degrees a day, negative
/// in retrograde; `1` when it is in retrograde, else `0`; and Jupiter's
/// heliocentric longitude, latitude and distance, geometric, in the mean
/// ecliptic and equinox of the date, the last in astronomical units.
///
/// # Errors
///
/// [`Refusal::Unknown`] for an ayanāṃśa not named; [`Refusal::OutOfRange`]
/// for an instant outside the sky layer's era.
pub fn jupiter_line(unix: i64, ayanamsa_name: &str) -> Answer<String> {
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    let moment = moment_in_era(unix)?;
    let at = jupiter::position(moment, ayanamsa);
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(at.apparent.longitude_degrees)
        .value(at.apparent.latitude_degrees)
        .value(at.apparent.distance_au)
        .value(at.sidereal_longitude_degrees)
        .cell(at.sign.id())
        .cell(at.sign.sanskrit_name())
        .value(at.degrees_into_sign)
        .value(at.daily_motion_degrees)
        .flag(at.daily_motion_degrees < 0.0)
        .value(at.apparent.heliocentric_longitude_degrees)
        .value(at.apparent.heliocentric_latitude_degrees)
        .value(at.apparent.heliocentric_distance_au);
    line.end();
    Ok(out)
}

/// How many columns each line of [`ingress_lines`] writes.
pub const INGRESS_COLUMNS: usize = 6;

/// The lines of `hc_jupiter_ingresses`: Jupiter's crossings of the
/// boundaries of the sidereal signs in the half-open span `[from, to)` of
/// POSIX seconds, in time order, one line each.
///
/// Each line: the moment, as whole POSIX seconds of Universal Time rounded
/// down; the sign left, by identifier and Sanskrit name; the sign entered,
/// by identifier and Sanskrit name; and `forward` when Jupiter moves on to
/// the next sign, `retrograde` when it turns back into the one before.
///
/// # Errors
///
/// [`Refusal::Unknown`] for an ayanāṃśa not named; [`Refusal::OutOfRange`]
/// for a span with an end outside the sky layer's era or longer than
/// [`MAX_INGRESS_SPAN_SECONDS`].
pub fn ingress_lines(from: i64, to: i64, ayanamsa_name: &str) -> Answer<String> {
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    let start = moment_in_era(from)?;
    if to <= from {
        return Ok(String::new());
    }
    let end = moment_in_era(to - 1)?;
    if to - from > MAX_INGRESS_SPAN_SECONDS {
        return Err(Refusal::OutOfRange);
    }
    let mut out = String::new();
    for ingress in jupiter::ingresses(start, Moment(end.0 + 1.0 / 86_400.0), ayanamsa) {
        let unix = unix_from_moment(ingress.moment);
        if unix >= to {
            break;
        }
        let mut line = Line::new(&mut out);
        line.value(unix)
            .cell(ingress.from.id())
            .cell(ingress.from.sanskrit_name())
            .cell(ingress.to.id())
            .cell(ingress.to.sanskrit_name())
            .cell(if ingress.is_forward() {
                "forward"
            } else {
                "retrograde"
            });
        line.end();
    }
    Ok(out)
}

/// How many columns [`kumbh_by_sky_line`] writes: `hc_kumbh`'s thirteen and
/// two.
pub const KUMBH_BY_SKY_COLUMNS: usize = crate::reckoning_lines::KUMBH_COLUMNS + 2;

/// The line of `hc_kumbh_by_sky`: `hc_kumbh`'s, with Jupiter's sign
/// computed rather than given.
///
/// The cells are those of [`crate::reckoning_lines::kumbh_line`]: the
/// condition, its site and the site's name in the locale, the river, the
/// signs Jupiter and the Sun must be in, the new-moon flag, the occasion's
/// first and last moments, and `1` when there is an occasion and Jupiter is in
/// the condition's sign at its first moment, `0` when not (never empty);
/// then the sidereal sign Jupiter is in at that moment, by identifier, and
/// its sidereal longitude in degrees, both empty when there is no occasion.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a condition or an ayanāṃśa not named;
/// [`Refusal::OutOfRange`] for a year outside the sky layer's era.
pub fn kumbh_by_sky_line(
    yoga: &str,
    year: i64,
    ayanamsa_name: &str,
    locale: &str,
) -> Answer<String> {
    let yoga = KumbhYoga::by_id(yoga).ok_or(Refusal::Unknown)?;
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    if !(EARLIEST_YEAR..=LATEST_YEAR).contains(&year) {
        return Err(Refusal::OutOfRange);
    }
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    kumbh_condition_cells(&mut line, &yoga, locale);
    let occasion = yoga.occasion(year, ayanamsa);
    match occasion {
        Some(occasion) => {
            let at = jupiter::position(occasion.from, ayanamsa);
            line.value(unix_from_moment(occasion.from))
                .value(unix_from_moment(occasion.to))
                .flag(at.sign == yoga.jupiter)
                .cell(at.sign.id())
                .value(at.sidereal_longitude_degrees);
        }
        None => {
            line.empties(2).flag(false).empties(2);
        }
    }
    line.end();
    Ok(out)
}

/// How many columns each line of [`pushkaram_by_sky_lines`] writes:
/// `hc_pushkaram`'s, and two.
pub const PUSHKARAM_BY_SKY_COLUMNS: usize = crate::reckoning_lines::PUSHKARAM_COLUMNS + 2;

/// The lines of `hc_pushkaram_by_sky`: the twelve days of the *Ādi
/// Pushkaram* of each river of a sidereal sign, for Jupiter's entry into the
/// sign in a Gregorian year, found rather than given. No line where Jupiter
/// makes no entry into the sign that year, which is most years.
///
/// `rule` names which entry is reckoned where Jupiter enters, turns back and
/// enters again, [`EntryRule::id`]: `pushkaram-final-entry`, the one the
/// festivals read began at, or `pushkaram-first-entry`. The twelve days
/// begin as for [`crate::reckoning_lines::pushkaram_lines`].
///
/// The cells are those of that function's lines, then the moment of the
/// entry as whole POSIX seconds of Universal Time, rounded down, and the
/// rule's identifier.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a sign, an ayanāṃśa, a rule or a meridian not
/// named; [`Refusal::OutOfRange`] for a year outside the sky layer's era.
pub fn pushkaram_by_sky_lines(
    sign_name: &str,
    year: i64,
    ayanamsa_name: &str,
    rule_name: &str,
    place: Location,
    meridian_name: &str,
    locale: &str,
) -> Answer<String> {
    let sign = sign(sign_name)?;
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    let rule = EntryRule::by_id(rule_name).ok_or(Refusal::Unknown)?;
    let meridian = meridian(meridian_name)?;
    if !(EARLIEST_YEAR..=LATEST_YEAR).contains(&year) {
        return Err(Refusal::OutOfRange);
    }
    let (from, until) = year_span(year);
    let Some(entry) = jupiter::entry_into(sign, from, until, rule, ayanamsa) else {
        return Ok(String::new());
    };
    let mut out = String::new();
    pushkaram_entry_lines(&mut out, sign, entry, rule, place, meridian, locale);
    Ok(out)
}

/// The moments a Gregorian year begins and ends at, as the Pushkaram's
/// searches take them.
fn year_span(year: i64) -> (Moment, Moment) {
    (
        Moment(new_year(year).0 as f64),
        Moment(new_year(year + 1).0 as f64),
    )
}

/// The lines of one entry of Jupiter into a sign: a line for each river of
/// the sign, with the entry's moment and the rule's identifier after the
/// cells of `hc_pushkaram`.
fn pushkaram_entry_lines(
    out: &mut String,
    sign: SiderealSign,
    entry: jupiter::Ingress,
    rule: EntryRule,
    place: Location,
    meridian: Meridian,
    locale: &str,
) {
    let span = adi_pushkaram(entry.moment, place, meridian);
    for river in rivers_of(sign) {
        let mut line = Line::new(out);
        pushkaram_river_cells(&mut line, &river, sign, span, locale);
        line.value(unix_from_moment(entry.moment)).cell(rule.id());
        line.end();
    }
}

/// How many columns each line of [`pushkarams_in_year_lines`] writes: those
/// of [`pushkaram_by_sky_lines`].
pub const PUSHKARAMS_IN_YEAR_COLUMNS: usize = PUSHKARAM_BY_SKY_COLUMNS;

/// The lines of `hc_pushkarams_in_year`: the twelve days of the *Ādi
/// Pushkaram* of every river of every sidereal sign Jupiter enters in a
/// Gregorian year, found rather than given, in the order of the entries.
/// No line in a year in which Jupiter enters no sign by the rule, which is
/// a few years in twelve.
///
/// The lines are those of [`pushkaram_by_sky_lines`] for each sign Jupiter
/// enters that year, one sign after another, to the byte: the cells, the
/// rule, and the sign each river's line carries. A year in which Jupiter
/// enters two signs has the lines of both, and one in which it enters a
/// sign, turns back out and enters again has one entry into that sign,
/// the one `rule` names, as [`pushkaram_by_sky_lines`] has. What the
/// year's lines cost is one search of the sky, not one for each sign: the
/// ingresses that every sign's search walks are found once.
///
/// # Errors
///
/// [`Refusal::Unknown`] for an ayanāṃśa, a rule or a meridian not named;
/// [`Refusal::OutOfRange`] for a year outside the sky layer's era.
pub fn pushkarams_in_year_lines(
    year: i64,
    ayanamsa_name: &str,
    rule_name: &str,
    place: Location,
    meridian_name: &str,
    locale: &str,
) -> Answer<String> {
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    let rule = EntryRule::by_id(rule_name).ok_or(Refusal::Unknown)?;
    let meridian = meridian(meridian_name)?;
    if !(EARLIEST_YEAR..=LATEST_YEAR).contains(&year) {
        return Err(Refusal::OutOfRange);
    }
    let (from, until) = year_span(year);
    let mut out = String::new();
    for entry in jupiter::entries_in(from, until, rule, ayanamsa) {
        pushkaram_entry_lines(&mut out, entry.to, entry, rule, place, meridian, locale);
    }
    Ok(out)
}

/// The runs of nakṣatras that name a year of Jupiter by its risings, in the
/// *Bṛhatsaṃhitā* (ch. 8, verses 1–2, `varahamihira-brihat-samhita`): "the
/// years of Jupiter take their names from the several nakṣatras in which he
/// reappears after his conjunction with the Sun", the names those of the
/// lunar months, beginning with Kārttika, each following two nakṣatras
/// beginning from Kṛttikā, "but the fifth, the eleventh and the twelfth
/// years follow, each, three": Kārttika Kṛttikā and Rohiṇī; Mārgaśīrṣa
/// Mṛgaśīrṣa and Ārdrā; Pauṣa Punarvasu and Puṣya; Māgha Āśleṣā and Maghā;
/// Phālguna Pūrvaphalgunī, Uttaraphalgunī and Hasta; Caitra Citrā and
/// Svātī; Vaiśākha Viśākhā and Anurādhā; Jyaiṣṭha Jyeṣṭhā and Mūla; Āṣāḍha
/// the two Āṣāḍhās; Śrāvaṇa Śravaṇa and Dhaniṣṭhā; Bhādrapada Śatabhiṣaj
/// and the two Bhādrapadās; Āśvayuja Revatī, Aśvinī and Bharaṇī.
const YEAR_RUNS: [(u8, usize); 12] = [
    (2, 7),
    (2, 8),
    (2, 9),
    (2, 10),
    (3, 11),
    (2, 0),
    (2, 1),
    (2, 2),
    (2, 3),
    (2, 4),
    (3, 5),
    (3, 6),
];

/// The twelve-year cycle's name for a year of Jupiter that begins with its
/// rising in a nakṣatra, 1 Aśvinī to 27 Revatī, as an index into
/// [`TWELVE_YEAR_NAMES`], 0 for Chaitra to 11 for Phālguna: Kārttika is 7.
/// `None` for a number outside 1 to 27.
#[must_use]
pub fn year_name_index(nakshatra: u8) -> Option<usize> {
    if !(1..=27).contains(&nakshatra) {
        return None;
    }
    // Counted from Kṛttikā, the third.
    let mut offset = (nakshatra + 24) % 27;
    for (length, name) in YEAR_RUNS {
        if offset < length {
            return Some(name);
        }
        offset -= length;
    }
    None
}

/// The longest span [`rising_lines`] accepts, in seconds: a hundred Julian
/// years, as [`MAX_INGRESS_SPAN_SECONDS`].
pub const MAX_RISING_SPAN_SECONDS: i64 = MAX_INGRESS_SPAN_SECONDS;

/// How many columns each line of [`rising_lines`] writes.
pub const RISING_COLUMNS: usize = 8;

/// The lines of `hc_jupiter_risings`: the heliacal risings of Jupiter in the
/// half-open span `[from, to)` of POSIX seconds, in time order, each with the
/// name the *Bṛhatsaṃhitā* gives the year of Jupiter that begins with it.
///
/// Each line: the moment of the rising, whole POSIX seconds of Universal
/// Time, rounded down: Jupiter's longitude west of the Sun's has passed 11°
/// ([`hc_seasons::zodiac::jupiter::VISIBILITY_ARC_DEGREES`]); the moment of
/// the setting before it, when it came within 11° east of the Sun and the
/// *asta* began; Jupiter's sidereal longitude at the rising, in degrees; the
/// nakṣatra it is in, by number from 1, identifier and name; and the name of
/// the year in the twelve-year cycle, as `hc-calendars-indic`
/// [`TWELVE_YEAR_NAMES`] spells it, and its position, 1 for Chaitra to 12
/// for Phālguna. A year of Jupiter runs from one rising to the next, about
/// 399 days, so a name is sometimes skipped: the expunged year.
///
/// # Errors
///
/// [`Refusal::Unknown`] for an ayanāṃśa not named; [`Refusal::OutOfRange`] for
/// a span with an end outside the sky layer's era or longer than
/// [`MAX_RISING_SPAN_SECONDS`].
pub fn rising_lines(from: i64, to: i64, ayanamsa_name: &str) -> Answer<String> {
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    let start = moment_in_era(from)?;
    if to <= from {
        return Ok(String::new());
    }
    let end = moment_in_era(to - 1)?;
    if to - from > MAX_RISING_SPAN_SECONDS {
        return Err(Refusal::OutOfRange);
    }
    let mut out = String::new();
    let mut at = start;
    while let Some(rising) = jupiter::next_heliacal_rising(at, jupiter::VISIBILITY_ARC_DEGREES) {
        let unix = unix_from_moment(rising.rising);
        if unix >= to || rising.rising.0 > end.0 + 1.0 {
            break;
        }
        // The next conjunction is 399 days on, and a month's margin is
        // well short of it.
        at = Moment(rising.rising.0 + 300.0);
        if unix < from {
            continue;
        }
        let longitude = jupiter::sidereal_longitude(rising.rising, ayanamsa);
        let number = nakshatra_at_longitude(longitude);
        let name = year_name_index(number).unwrap_or(0);
        let mut line = Line::new(&mut out);
        line.value(unix)
            .value(unix_from_moment(rising.setting))
            .value(longitude)
            .value(number);
        nakshatra_cells(&mut line, number);
        line.cell(TWELVE_YEAR_NAMES[name]).value(name + 1);
        line.end();
    }
    Ok(out)
}

/// The nakṣatra, 1 to 27, a sidereal longitude is in.
fn nakshatra_at_longitude(longitude: f64) -> u8 {
    let index = hc_core::math::floor(longitude / DEGREES_PER_NAKSHATRA) as u8;
    index % 27 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendar::gregorian::to_fixed;

    /// POSIX seconds of 0 h UT on a Gregorian day.
    fn unix(year: i64, month: u8, day: u8) -> i64 {
        (to_fixed(year, month, day).expect("a date").0 - 719_163) * 86_400
    }

    fn cells(line: &str) -> Vec<&str> {
        line.trim_end().split('\t').collect()
    }

    const NEW_DELHI: Location = Location::new(28.6356, 77.2244, 0.0);

    /// Horizons' apparent ecliptic longitude of Jupiter at 0 h UT on
    /// 2024-12-07 is 76.3751533°, latitude −0.6718041°, delta 4.0894152 AU
    /// (`jpl-horizons`, retrieved 2026-10-03), the day of its opposition.
    #[test]
    fn jupiter_at_its_opposition_of_2024_is_one_line_of_twelve_cells() {
        let line = jupiter_line(unix(2024, 12, 7), "lahiri").expect("a line");
        let cells = cells(&line);
        assert_eq!(cells.len(), JUPITER_COLUMNS);
        let number = |i: usize| cells[i].parse::<f64>().expect("a number");
        assert!((number(0) - 76.375_153_3).abs() < 0.0002);
        assert!((number(1) + 0.671_804_1).abs() < 0.0001);
        assert!((number(2) - 4.089_415_2).abs() < 0.000_01);
        assert!((number(3) - 52.2).abs() < 0.1);
        assert_eq!(
            (cells[4], cells[5], cells[8]),
            ("vrishabha", "Vṛṣabha", "1")
        );
        assert!(number(7) < -0.1);
        // Jupiter is at 5.0 AU from the Sun and the heliocentric longitude
        // leads the apparent geocentric one by the parallax.
        assert!((number(11) - 5.0).abs() < 0.3);
    }

    #[test]
    fn jupiter_refuses_what_it_cannot_answer() {
        assert_eq!(jupiter_line(0, "no-such"), Err(Refusal::Unknown));
        assert_eq!(
            jupiter_line(unix(3001, 1, 1), "lahiri"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            jupiter_line(unix(-1001, 1, 1), "lahiri"),
            Err(Refusal::OutOfRange)
        );
    }

    /// 2019: Jupiter entered Dhanus on 29 March (UT), turned back into
    /// Vṛścika on 22 April and entered Dhanus again on 5 November
    /// (`drik-guru-gochar`, 30 March, 22 April and 5 November IST).
    #[test]
    fn the_ingresses_of_2019_are_three_with_a_return() {
        let text = ingress_lines(unix(2019, 1, 1), unix(2020, 1, 1), "lahiri").expect("lines");
        let rows: Vec<Vec<&str>> = text.lines().map(cells).collect();
        assert_eq!(rows.len(), 3, "{text}");
        assert!(rows.iter().all(|row| row.len() == INGRESS_COLUMNS));
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
        let first: i64 = rows[0][0].parse().expect("a second");
        assert!((unix(2019, 3, 29) + 16 * 3600..unix(2019, 3, 29) + 19 * 3600).contains(&first));
    }

    #[test]
    fn the_ingress_span_is_half_open_and_bounded() {
        assert_eq!(ingress_lines(10, 10, "lahiri"), Ok(String::new()));
        assert_eq!(ingress_lines(100, 10, "lahiri"), Ok(String::new()));
        assert_eq!(
            ingress_lines(unix(2000, 1, 1), unix(2101, 1, 1), "lahiri"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            ingress_lines(unix(2000, 1, 1), unix(2001, 1, 1), "nope"),
            Err(Refusal::Unknown)
        );
        // An end inside the era but a span that reaches out of it.
        assert_eq!(
            ingress_lines(unix(2999, 1, 1), unix(3001, 1, 2), "lahiri"),
            Err(Refusal::OutOfRange)
        );
    }

    /// The Maha Kumbh of 2025 (`wikipedia-kumbh-mela`): Prayag's first
    /// condition, with Jupiter in Vṛṣabha at the Sun's entry into Makara.
    #[test]
    fn the_kumbh_of_2025_is_met_with_jupiter_computed() {
        let line =
            kumbh_by_sky_line("kumbh-prayag-vrishabha", 2025, "lahiri", "en").expect("a line");
        let cells = cells(&line);
        assert_eq!(cells.len(), KUMBH_BY_SKY_COLUMNS);
        assert_eq!(&cells[..2], ["kumbh-prayag-vrishabha", "prayag"]);
        assert_eq!(cells[12], "1");
        assert_eq!(cells[13], "vrishabha");
        let longitude: f64 = cells[14].parse().expect("a number");
        assert!((30.0..60.0).contains(&longitude), "{longitude}");
        // The caller's form agrees: the same thirteen cells with the sign given.
        let given = crate::reckoning_lines::kumbh_line(
            "kumbh-prayag-vrishabha",
            2025,
            "lahiri",
            "vrishabha",
            "en",
        )
        .expect("a line");
        assert_eq!(cells[..13], self::cells(&given)[..13]);
        // A year the condition does not hold is `0`, not empty.
        let other =
            kumbh_by_sky_line("kumbh-prayag-vrishabha", 2026, "lahiri", "en").expect("a line");
        assert_eq!(self::cells(&other)[12], "0");
        assert_eq!(self::cells(&other)[13], "mithuna");
    }

    #[test]
    fn the_kumbh_refuses_what_it_cannot_answer() {
        assert_eq!(
            kumbh_by_sky_line("nope", 2025, "lahiri", "en"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            kumbh_by_sky_line("kumbh-haridwar", 2025, "nope", "en"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            kumbh_by_sky_line("kumbh-haridwar", 3001, "lahiri", "en"),
            Err(Refusal::OutOfRange)
        );
    }

    /// The Brahmaputra and Tapti festivals of 2019 begin at the final entry
    /// into Dhanus, on 5 November (`sentinel-brahmaputra-2019`).
    #[test]
    fn the_pushkaram_of_2019_follows_the_final_entry_into_dhanus() {
        let text = pushkaram_by_sky_lines(
            "dhanus",
            2019,
            "lahiri",
            "pushkaram-final-entry",
            NEW_DELHI,
            "india",
            "en",
        )
        .expect("lines");
        let rows: Vec<Vec<&str>> = text.lines().map(cells).collect();
        assert_eq!(rows.len(), 2, "{text}");
        assert!(rows.iter().all(|row| row.len() == PUSHKARAM_BY_SKY_COLUMNS));
        assert_eq!(
            rows.iter().map(|row| row[0]).collect::<Vec<_>>(),
            ["pushkaram-tapti", "pushkaram-brahmaputra"]
        );
        let first = to_fixed(2019, 11, 5).expect("a date").0;
        assert_eq!(rows[0][6], first.to_string());
        assert_eq!(rows[0][7], (first + 11).to_string());
        assert_eq!(rows[1][3], "Assam");
    }

    /// The lines of a year, taken a sign at a time: each sign's lines, in
    /// the order the signs first appear.
    fn blocks_of(text: &str) -> Vec<(String, String)> {
        let mut blocks: Vec<(String, String)> = Vec::new();
        for line in text.lines() {
            let sign = cells(line)[4].to_string();
            match blocks.last_mut() {
                Some((last, block)) if *last == sign => {
                    block.push_str(line);
                    block.push('\n');
                }
                _ => blocks.push((sign, format!("{line}\n"))),
            }
        }
        blocks
    }

    /// `hc_pushkarams_in_year` gives, for a year, what `hc_pushkaram_by_sky`
    /// gives for each of the twelve signs, and only for the signs that have
    /// a line. In 1998 Jupiter enters Kumbha (8 January), then Mīna (25 May)
    /// and turns back into Kumbha (10 September), and enters Mīna again on
    /// 12 January 1999: the final entry rule finds Kumbha for 1998 and the
    /// first-entry rule finds both. In 2025 it enters Mithuna (14 May) and
    /// Karka (18 October), and goes back to Mithuna before it stays in Karka
    /// from 1 June 2026. In 2029 it enters Tulā on 24 August after turning
    /// back from it, which is the final entry and not a first one. In 1971 it
    /// enters no sign. Each is asked of both rules.
    #[test]
    fn a_years_pushkarams_are_those_of_each_sign_by_sky() {
        use hc_seasons::zodiac::SiderealSign;
        let mut nonempty = 0;
        let mut empty = 0;
        for year in [1971, 1998, 2025, 2029] {
            for rule in ["pushkaram-final-entry", "pushkaram-first-entry"] {
                let text = pushkarams_in_year_lines(year, "lahiri", rule, NEW_DELHI, "india", "en")
                    .expect("lines");
                assert!(
                    text.lines()
                        .all(|line| cells(line).len() == PUSHKARAMS_IN_YEAR_COLUMNS),
                    "{year} {rule}"
                );
                let by_year = blocks_of(&text);
                let by_sky = |sign: &SiderealSign| {
                    pushkaram_by_sky_lines(
                        sign.id(),
                        year,
                        "lahiri",
                        rule,
                        NEW_DELHI,
                        "india",
                        "en",
                    )
                    .expect("lines")
                };
                // Every sign asked, where the year is one that tells
                // the rules apart or has none; otherwise the signs the
                // year's lines are for.
                let every = [
                    (2025, "pushkaram-first-entry"),
                    (2029, "pushkaram-final-entry"),
                ]
                .contains(&(year, rule));
                let asked: Vec<SiderealSign> = if every {
                    SiderealSign::ALL.to_vec()
                } else {
                    by_year
                        .iter()
                        .filter_map(|(sign, _)| SiderealSign::by_id(sign))
                        .collect()
                };
                let by_sign: Vec<(String, String)> = asked
                    .iter()
                    .filter_map(|sign| {
                        let one = by_sky(sign);
                        (!one.is_empty()).then(|| (sign.id().to_string(), one))
                    })
                    .collect();
                // The same blocks; the year's are in time order, the signs'
                // in the zodiac's, so compare as sets.
                let mut sorted = by_year.clone();
                sorted.sort();
                let mut expected = by_sign.clone();
                expected.sort();
                assert_eq!(sorted, expected, "{year} {rule}");
                assert_eq!(
                    by_year.len(),
                    by_year
                        .iter()
                        .map(|(sign, _)| sign)
                        .collect::<std::collections::BTreeSet<_>>()
                        .len(),
                    "a sign twice in {year} {rule}"
                );
                if text.is_empty() {
                    empty += 1;
                } else {
                    nonempty += 1;
                }
            }
        }
        // 1971 is empty by both rules and 2029 by the first.
        assert_eq!((nonempty, empty), (5, 3));
    }

    /// A year in which Jupiter enters two signs has the Pushkaram of both, in
    /// the order of the entries: in 1999 it enters Mīna on 12 January, for
    /// the last time, and Meṣa on 26 May.
    #[test]
    fn a_year_with_two_entries_has_two_signs_in_order() {
        let text = pushkarams_in_year_lines(
            1999,
            "lahiri",
            "pushkaram-final-entry",
            NEW_DELHI,
            "india",
            "en",
        )
        .expect("lines");
        let blocks = blocks_of(&text);
        let signs: Vec<&str> = blocks.iter().map(|(sign, _)| sign.as_str()).collect();
        assert_eq!(signs, ["mina", "mesha"]);
        let on = |block: &str, month: u8, day: u8| {
            let moment: i64 = cells(block.lines().next().expect("a line"))[12]
                .parse()
                .expect("a number");
            let days = (moment / 86_400) + 719_163;
            days == to_fixed(1999, month, day).expect("a date").0
        };
        assert!(on(&blocks[0].1, 1, 12) && on(&blocks[1].1, 5, 26));
        let moments: Vec<i64> = blocks
            .iter()
            .map(|(_, block)| {
                cells(block.lines().next().expect("a line"))[12]
                    .parse()
                    .expect("a number")
            })
            .collect();
        assert!(moments[0] < moments[1]);
    }

    #[test]
    fn the_pushkarams_of_a_year_refuse_what_they_cannot() {
        let ask = |year: i64, rule: &str, meridian: &str, ayanamsa: &str| {
            pushkarams_in_year_lines(year, ayanamsa, rule, NEW_DELHI, meridian, "en")
        };
        assert_eq!(
            ask(2019, "second", "india", "lahiri"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            ask(2019, "pushkaram-final-entry", "mars", "lahiri"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            ask(2019, "pushkaram-final-entry", "india", "nope"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            ask(-1001, "pushkaram-final-entry", "india", "lahiri"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            ask(3001, "pushkaram-final-entry", "india", "lahiri"),
            Err(Refusal::OutOfRange)
        );
    }

    #[test]
    fn the_pushkaram_has_no_line_in_a_year_without_an_entry_and_refuses_what_it_cannot() {
        let ask = |sign: &str, year: i64, rule: &str, meridian: &str| {
            pushkaram_by_sky_lines(sign, year, "lahiri", rule, NEW_DELHI, meridian, "en")
        };
        assert_eq!(
            ask("simha", 2019, "pushkaram-final-entry", "india"),
            Ok(String::new())
        );
        assert_eq!(
            ask("leo", 2019, "pushkaram-final-entry", "india"),
            Err(Refusal::Unknown)
        );
        assert_eq!(ask("simha", 2019, "second", "india"), Err(Refusal::Unknown));
        assert_eq!(
            ask("simha", 2019, "pushkaram-final-entry", "mars"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            ask("simha", -1001, "pushkaram-final-entry", "india"),
            Err(Refusal::OutOfRange)
        );
    }
    /// The *Bṛhatsaṃhitā*'s runs (ch. 8, verses 1 and 2, as `wisdomlib`
    /// prints Iyer's translation, `varahamihira-brihat-samhita`), one name for
    /// each nakṣatra by number: Kārttika is index 7 of the twelve-year
    /// names, which begin with Chaitra.
    #[test]
    fn the_year_of_jupiter_is_named_by_the_nakshatra_of_its_rising() {
        let names = |numbers: &[u8]| -> Vec<&str> {
            numbers
                .iter()
                .map(|&n| TWELVE_YEAR_NAMES[year_name_index(n).expect("a nakshatra")])
                .collect()
        };
        // Kṛttikā and Rohiṇī.
        assert_eq!(names(&[3, 4]), ["Karttika"; 2]);
        // Mṛgaśīrṣa and Ārdrā; Punarvasu and Puṣya; Āśleṣā and Maghā.
        assert_eq!(names(&[5, 6]), ["Margasirsha"; 2]);
        assert_eq!(names(&[7, 8]), ["Pausha"; 2]);
        assert_eq!(names(&[9, 10]), ["Magha"; 2]);
        // The fifth year follows three: the two Phalgunīs and Hasta.
        assert_eq!(names(&[11, 12, 13]), ["Phalguna"; 3]);
        assert_eq!(names(&[14, 15]), ["Chaitra"; 2]);
        assert_eq!(names(&[16, 17]), ["Vaisakha"; 2]);
        assert_eq!(names(&[18, 19]), ["Jyeshtha"; 2]);
        assert_eq!(names(&[20, 21]), ["Ashadha"; 2]);
        assert_eq!(names(&[22, 23]), ["Sravana"; 2]);
        // The eleventh and twelfth follow three: Śatabhiṣaj and the two
        // Bhādrapadās, then Revatī, Aśvinī and Bharaṇī.
        assert_eq!(names(&[24, 25, 26]), ["Bhadrapada"; 3]);
        assert_eq!(names(&[27, 1, 2]), ["Asvina"; 3]);
        assert_eq!(year_name_index(0), None);
        assert_eq!(year_name_index(28), None);
        // Twenty-seven nakṣatras, and the three-nakṣatra runs are the fifth,
        // the eleventh and the twelfth names.
        let mut counts = [0; 12];
        for number in 1..=27 {
            counts[year_name_index(number).expect("a name")] += 1;
        }
        // In the order of the names, Chaitra first: Phālguna, Bhādrapada and
        // Āśvayuja have three.
        assert_eq!(counts, [2, 2, 2, 2, 2, 3, 3, 2, 2, 2, 2, 3]);
    }

    /// Verse 27 of the chapter: when Jupiter "reappears at the beginning of
    /// the constellation of Dhaniṣṭhā in the month of Māgha", the first year
    /// of the cycle of sixty begins; Sewell and Dikshit's Table XII couples
    /// that first year, Prabhava, with the twelve-year name Śrāvaṇa
    /// (`sewell1896`): two readings of two texts that agree.
    #[test]
    fn the_first_year_of_the_sixty_is_named_for_dhanishtha_as_sewells_table_has_it() {
        use hc_calendars_indic::barhaspatya::twelve_year_of;
        use hc_calendars_indic::nakshatra::DHANISHTHA;
        let prabhava = usize::from(twelve_year_of(1).expect("Prabhava")) - 1;
        assert_eq!(year_name_index(DHANISHTHA), Some(prabhava));
        assert_eq!(TWELVE_YEAR_NAMES[prabhava], "Sravana");
    }

    /// Drik Panchang's Guru Asta pages for New Delhi (`drik-guru-asta`,
    /// retrieved 2026-10-03): the asta begins on the evening and ends on the
    /// morning shown, IST. Its own algorithm is local, and ours is the fixed
    /// arc of 11° of longitude, so the two stand up to four days apart.
    #[test]
    fn the_risings_of_2024_to_2027_are_within_four_days_of_drik_panchangs() {
        let ist = |year: i64, month: u8, day: u8, hour: u8, minute: u8| {
            (to_fixed(year, month, day).expect("a date").0 - 719_163) as f64
                + (f64::from(hour) + f64::from(minute) / 60.0 - 5.5) / 24.0
        };
        let drik = [
            (ist(2024, 5, 7, 19, 36), ist(2024, 6, 6, 4, 36)),
            (ist(2025, 6, 12, 19, 56), ist(2025, 7, 9, 4, 44)),
            (ist(2026, 7, 15, 19, 59), ist(2026, 8, 12, 5, 3)),
            (ist(2027, 8, 17, 19, 32), ist(2027, 9, 13, 5, 22)),
        ];
        let text = rising_lines(unix(2024, 1, 1), unix(2028, 1, 1), "lahiri").expect("lines");
        let rows: Vec<Vec<&str>> = text.lines().map(cells).collect();
        assert_eq!(rows.len(), 4, "{text}");
        for (row, (setting, rising)) in rows.iter().zip(drik) {
            assert_eq!(row.len(), RISING_COLUMNS);
            let day = |cell: &str| cell.parse::<f64>().expect("a second") / 86_400.0;
            assert!((day(row[1]) - setting).abs() < 4.2, "{row:?} {setting}");
            assert!((day(row[0]) - rising).abs() < 4.2, "{row:?} {rising}");
        }
        // Each asta is three to four weeks.
        for row in &rows {
            let days =
                (row[0].parse::<f64>().expect("s") - row[1].parse::<f64>().expect("s")) / 86_400.0;
            assert!((26.0..34.0).contains(&days), "{days}");
        }
        // Jupiter rose in Kṛttikā on 2 June 2024 and in Ārdrā in 2025, Puṣya
        // in 2026 and Pūrvaphalgunī in 2027, whose years are named
        // Kārttika, Mārgaśīrṣa, Pauṣa and Phālguna.
        assert_eq!(
            rows.iter().map(|row| (row[4], row[6])).collect::<Vec<_>>(),
            [
                ("krittika", "Karttika"),
                ("ardra", "Margasirsha"),
                ("pushya", "Pausha"),
                ("purva-phalguni", "Phalguna"),
            ]
        );
        assert_eq!(rows[0][7], "8");
    }

    #[test]
    fn the_risings_are_a_year_and_a_month_apart_and_the_span_is_bounded() {
        let text = rising_lines(unix(2000, 1, 1), unix(2020, 1, 1), "lahiri").expect("lines");
        let moments: Vec<f64> = text
            .lines()
            .map(|line| cells(line)[0].parse::<f64>().expect("a second") / 86_400.0)
            .collect();
        assert!((18..=19).contains(&moments.len()), "{}", moments.len());
        for pair in moments.windows(2) {
            assert!((390.0..406.0).contains(&(pair[1] - pair[0])));
        }
        assert_eq!(rising_lines(10, 10, "lahiri"), Ok(String::new()));
        assert_eq!(
            rising_lines(unix(2000, 1, 1), unix(2101, 1, 1), "lahiri"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(rising_lines(0, 100, "nope"), Err(Refusal::Unknown));
    }
}
