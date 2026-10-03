//! The tab-separated lines the WebAssembly module and the C library write
//! about the pañcāṅga's yoga and karaṇa, written once.
//!
//! Each answer is two lines of the same nine columns, the yoga's first
//! and the karaṇa's second: the limb (`yoga` or `karana`), its number
//! (the yoga 1 for Viṣkambha through 27 for Vaidhṛti; the karaṇa the
//! half-tithi, 1 for the first half of śukla 1 through 60 for the second
//! half of amāvāsyā), its name as Drik Panchang spells it in English and
//! in Devanagari, the moments it began and ends and the moment it was read
//! at, each as whole POSIX seconds of Universal Time, rounded down, and
//! the ayanāṃśa the yoga was reckoned with, by the identifier [`ayanamsa`]
//! reads back and by its full name, both empty for the karaṇa, which needs
//! none. The arithmetic and the names are
//! [`hc_calendars_indic::panchanga`]'s; the instants are held to the sky
//! layer's era, [`crate::astro_lines`].
//!
//! Rāhu kālam, Yamaganda and Gulika kālam, [`hc_calendars_indic::kalam`],
//! are three more lines of their own, one a period, each named in a
//! locale, so in a build with `i18n` too: `kalam_lines`.

use alloc::string::String;

use hc_astro::riseset::{Location, sunrise};
use hc_calendar::Rd;
use hc_calendar::fixed::Moment;
use hc_calendars_indic::nakshatra::{nakshatra_at, nakshatra_id, nakshatra_name, nakshatra_span};
use hc_calendars_indic::panchanga::{
    KARANA_NAMES, KARANA_NAMES_DEVANAGARI, YOGA_NAMES, YOGA_NAMES_DEVANAGARI, karana_at,
    karana_name, karana_name_surya_siddhanta, karana_span, yoga_at, yoga_span,
};
use hc_calendars_indic::surya_siddhanta::{self, MAX_SUNRISE_LATITUDE};
use hc_calendars_indic::{amrita_siddhi, muhurta};
use hc_core::catalogue::matches;
use hc_seasons::zodiac::Ayanamsa;

#[cfg(feature = "i18n")]
use hc_calendars_indic::kalam::{Kalam, KalamConvention, SpanClock};

use crate::astro_lines::{MISSING_COLUMNS, missing_cells};
use crate::astro_lines::{day_in_era, moment_in_era, moment_on_day, unix_from_moment};
#[cfg(feature = "i18n")]
use crate::boundary::reckoning_name;
use crate::boundary::{Answer, Line, Refusal};

/// The name of the *Sūrya Siddhānta*'s sky, beside the ayanāṃśa names of
/// the true one.
pub use hc_calendars_indic::surya_siddhanta::SKY as SURYA_SIDDHANTA;

/// The sky's full name, as the ayanāṃśa's full name is written beside its
/// identifier.
pub const SURYA_SIDDHANTA_NAME: &str = "Sūrya Siddhānta";

/// The first fixed day the Siddhānta's exports answer for: Chaitra śukla 1
/// of Kali Yuga 1 on `hindu-lunar-surya-siddhanta`, 13 January 3101 BCE.
pub const SIDDHANTA_FIRST_DAY: i64 = -1_132_604;

/// The last fixed day the Siddhānta's exports answer for, the last of Kali
/// Yuga 10 000 on the same calendar, 15 June 6900.
pub const SIDDHANTA_LAST_DAY: i64 = 2_519_974;

/// A fixed day the Siddhānta's exports answer for.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] outside [`SIDDHANTA_FIRST_DAY`] to
/// [`SIDDHANTA_LAST_DAY`].
pub(crate) fn siddhanta_day(fixed: i64) -> Answer<Rd> {
    if (SIDDHANTA_FIRST_DAY..=SIDDHANTA_LAST_DAY).contains(&fixed) {
        Ok(Rd(fixed))
    } else {
        Err(Refusal::OutOfRange)
    }
}

/// A place where the Sun rises every day, on either sky: within
/// [`MAX_SUNRISE_LATITUDE`] of the equator. A lunisolar month begins at the
/// first sunrise after a conjunction, so a place with even one day of the
/// year without a sunrise has months that cannot be read there.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] beyond it.
pub(crate) fn sunrise_place(place: Location) -> Answer<Location> {
    if place.latitude_degrees.abs() <= MAX_SUNRISE_LATITUDE {
        Ok(place)
    } else {
        Err(Refusal::OutOfRange)
    }
}

/// The ayanāṃśa an identifier names: one of [`Ayanamsa::ALL`] by
/// [`Ayanamsa::by_id`], `lahiri`, `lahiri-rashtriya`, `lahiri-crc-1955`, `lahiri-drik`, `raman`, `krishnamurti`,
/// `reingold-dershowitz` or `fagan-bradley`.
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other text, the empty one and a full name
/// such as `Lahiri (Chitrapaksha)` included: the yoga moves with the
/// ayanāṃśa, so none is assumed.
pub fn ayanamsa(id: &str) -> Answer<Ayanamsa> {
    Ayanamsa::by_id(id).ok_or(Refusal::Unknown)
}

/// The two lines at a moment, read at the POSIX second `read_at`: the
/// caller's own instant, which a round trip through a day count could
/// move by a rounding.
fn lines_at(moment: Moment, read_at: i64, ayanamsa: Ayanamsa) -> String {
    let mut out = String::new();
    let yoga = yoga_at(moment, ayanamsa);
    let (began, ends) = yoga_span(moment, ayanamsa);
    let index = usize::from(yoga - 1);
    let mut line = Line::new(&mut out);
    line.cell("yoga")
        .value(yoga)
        .cell(YOGA_NAMES[index])
        .cell(YOGA_NAMES_DEVANAGARI[index])
        .value(unix_from_moment(began))
        .value(unix_from_moment(ends))
        .value(read_at)
        .cell(ayanamsa.id())
        .cell(ayanamsa.name());
    line.end();
    let half = karana_at(moment);
    let (began, ends) = karana_span(moment);
    // `karana_at` gives a half of the month, 1 to 60, which always has a
    // name.
    if let Some(name) = karana_name(half) {
        let name = usize::from(name);
        let mut line = Line::new(&mut out);
        line.cell("karana")
            .value(half)
            .cell(KARANA_NAMES[name])
            .cell(KARANA_NAMES_DEVANAGARI[name])
            .value(unix_from_moment(began))
            .value(unix_from_moment(ends))
            .value(read_at)
            .empties(2);
        line.end();
    }
    out
}

/// The two lines at a moment by the *Sūrya Siddhānta*'s Sun and Moon,
/// read at the POSIX second `read_at`: [`lines_at`]'s columns, the sky
/// named on both lines, since the Siddhānta's karaṇa is its own as its
/// yoga is.
fn siddhanta_lines_at(moment: Moment, read_at: i64) -> String {
    let mut out = String::new();
    let yoga = surya_siddhanta::yoga_at(moment);
    let (began, ends) = surya_siddhanta::yoga_span(moment);
    let index = usize::from(yoga - 1);
    let mut line = Line::new(&mut out);
    line.cell("yoga")
        .value(yoga)
        .cell(YOGA_NAMES[index])
        .cell(YOGA_NAMES_DEVANAGARI[index])
        .value(unix_from_moment(began))
        .value(unix_from_moment(ends))
        .value(read_at)
        .cell(SURYA_SIDDHANTA)
        .cell(SURYA_SIDDHANTA_NAME);
    line.end();
    let half = surya_siddhanta::karana_at(moment);
    let (began, ends) = surya_siddhanta::karana_span(moment);
    // The book names its four fixed karaṇas in its own order, Nāga before
    // Catuṣpada; `karana_at` gives a half, 1 to 60, which always has one.
    if let Some(name) = karana_name_surya_siddhanta(half) {
        let name = usize::from(name);
        let mut line = Line::new(&mut out);
        line.cell("karana")
            .value(half)
            .cell(KARANA_NAMES[name])
            .cell(KARANA_NAMES_DEVANAGARI[name])
            .value(unix_from_moment(began))
            .value(unix_from_moment(ends))
            .value(read_at)
            .cell(SURYA_SIDDHANTA)
            .cell(SURYA_SIDDHANTA_NAME);
        line.end();
    }
    out
}

/// The lines of `hc_panchanga_at`: the yoga and the karaṇa in progress at
/// a Universal Time instant, on the true sky in the zodiac of an ayanāṃśa
/// or on [`SURYA_SIDDHANTA`]'s.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a sky that is neither [`SURYA_SIDDHANTA`] nor
/// an ayanāṃśa [`ayanamsa`] names, and [`Refusal::OutOfRange`] for an
/// instant outside the sky layer's era on the true sky, or outside
/// [`SIDDHANTA_FIRST_DAY`] to [`SIDDHANTA_LAST_DAY`] on the Siddhānta's.
pub fn panchanga_at_lines(universal_unix: i64, ayanamsa_name: &str) -> Answer<String> {
    if matches(ayanamsa_name, SURYA_SIDDHANTA) {
        let moment = moment_on_day(universal_unix, siddhanta_day)?;
        return Ok(siddhanta_lines_at(moment, universal_unix));
    }
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    Ok(lines_at(
        moment_in_era(universal_unix)?,
        universal_unix,
        ayanamsa,
    ))
}

/// The lines of `hc_panchanga_of_day`: the yoga and the karaṇa a day
/// carries at a place, the ones in progress at its sunrise.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a sky [`panchanga_at_lines`] does not read,
/// [`Refusal::OutOfRange`] for a day outside the sky layer's era, and
/// [`Refusal::NoData`] for a day on which the Sun does not rise at the
/// place: a pañcāṅga reads its day at sunrise, and no other moment is put
/// in its place. On [`SURYA_SIDDHANTA`]'s sky the day is read at the
/// Siddhānta's own sunrise, which every day has within
/// [`MAX_SUNRISE_LATITUDE`]; a place beyond it, or a day outside
/// [`SIDDHANTA_FIRST_DAY`] to [`SIDDHANTA_LAST_DAY`], is
/// [`Refusal::OutOfRange`].
pub fn panchanga_of_day_lines(fixed: i64, place: Location, ayanamsa_name: &str) -> Answer<String> {
    if matches(ayanamsa_name, SURYA_SIDDHANTA) {
        let day = siddhanta_day(fixed)?;
        let place = sunrise_place(place)?;
        let rise = surya_siddhanta::sunrise(day, place);
        return Ok(siddhanta_lines_at(rise, unix_from_moment(rise)));
    }
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    let day = day_in_era(fixed)?;
    let rise = sunrise(day, place).ok_or(Refusal::NoData)?;
    Ok(lines_at(rise, unix_from_moment(rise), ayanamsa))
}

/// How many columns each line of [`nakshatra_at_lines`] writes.
pub const NAKSHATRA_COLUMNS: usize = 8;

/// The line of the nakṣatra in progress at a moment, read at `read_at`:
/// its number, identifier and name, when the Moon entered it and when it
/// leaves, the instant read, and the ayanāṃśa by identifier and full name.
fn nakshatra_line(moment: Moment, read_at: i64, ayanamsa: Ayanamsa) -> String {
    let number = nakshatra_at(moment, ayanamsa);
    let (entered, leaves) = nakshatra_span(number, moment, ayanamsa);
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(number);
    nakshatra_cells(&mut line, number);
    line.value(unix_from_moment(entered))
        .value(unix_from_moment(leaves))
        .value(read_at)
        .cell(ayanamsa.id())
        .cell(ayanamsa.name());
    line.end();
    out
}

/// A nakṣatra's identifier and name, [`nakshatra_id`] and
/// [`nakshatra_name`], the two cells every line that names one writes
/// after its number: `pushya` and `Puṣya`.
pub(crate) fn nakshatra_cells(line: &mut Line<'_>, number: u8) {
    line.cell(nakshatra_id(number).unwrap_or(""))
        .cell(nakshatra_name(number).unwrap_or(""));
}

/// The line of `hc_nakshatra_at`: the nakṣatra the Moon is in at a
/// Universal Time instant, 1 for Aśvinī through 27 for Revatī, in the
/// zodiac of an ayanāṃśa — its number, its identifier and name (`ashvini`,
/// `Aśvinī`), the instants the Moon entered it
/// and leaves it and the instant read, as whole POSIX seconds rounded
/// down, and the ayanāṃśa by identifier and full name.
///
/// # Errors
///
/// [`Refusal::Unknown`] for an ayanāṃśa [`ayanamsa`] does not name, and
/// [`Refusal::OutOfRange`] for an instant outside the sky layer's era.
pub fn nakshatra_at_lines(universal_unix: i64, ayanamsa_name: &str) -> Answer<String> {
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    Ok(nakshatra_line(
        moment_in_era(universal_unix)?,
        universal_unix,
        ayanamsa,
    ))
}

/// The line of `hc_nakshatra_of_day`: [`nakshatra_at_lines`]' line read
/// at the day's sunrise at the place, the nakṣatra a pañcāṅga prints for
/// the day.
///
/// # Errors
///
/// As [`panchanga_of_day_lines`] on the true sky.
pub fn nakshatra_of_day_lines(fixed: i64, place: Location, ayanamsa_name: &str) -> Answer<String> {
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    let day = day_in_era(fixed)?;
    let rise = sunrise(day, place).ok_or(Refusal::NoData)?;
    Ok(nakshatra_line(rise, unix_from_moment(rise), ayanamsa))
}

/// How many columns [`amrita_siddhi_line`] writes.
pub const AMRITA_SIDDHI_COLUMNS: usize = 9;

/// The line of `hc_amrita_siddhi`: the *amṛta siddhi yoga* of a day at a
/// place, [`amrita_siddhi::amrita_siddhi`] — its name as Drik Panchang
/// prints it in English and in Devanagari, the nakṣatra the weekday pairs
/// with, 1 to 27, its identifier and name, the start and the end of the part of the day, sunrise to
/// the next sunrise, that the Moon spends in it, as whole POSIX seconds
/// rounded down, both empty on a day it spends none, `1` if the yoga falls
/// on the day and `0` if not, and the ayanāṃśa's identifier.
///
/// # Errors
///
/// [`Refusal::Unknown`] for an ayanāṃśa [`ayanamsa`] does not name,
/// [`Refusal::OutOfRange`] for a day outside the sky layer's era, and
/// [`Refusal::NoData`] for a day on which, or on whose morrow, the Sun
/// does not rise at the place, which leaves the day without bounds.
pub fn amrita_siddhi_line(fixed: i64, place: Location, ayanamsa_name: &str) -> Answer<String> {
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    let day = day_in_era(fixed)?;
    let span = amrita_siddhi::amrita_siddhi(day, place, ayanamsa).map_err(|_| Refusal::NoData)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    let nakshatra = amrita_siddhi::nakshatra_of(hc_calendar::Weekday::from_rd(day));
    line.cell(amrita_siddhi::NAME)
        .cell(amrita_siddhi::NAME_DEVANAGARI)
        .value(nakshatra);
    nakshatra_cells(&mut line, nakshatra);
    line.value_or_empty(span.map(|span| unix_from_moment(span.start)))
        .value_or_empty(span.map(|span| unix_from_moment(span.end)))
        .flag(span.is_some())
        .cell(ayanamsa.id());
    line.end();
    Ok(out)
}

/// How many columns each line of [`muhurtas_lines`] writes.
pub const MUHURTA_COLUMNS: usize = 6 + MISSING_COLUMNS;

/// The lines of `hc_muhurtas`: the thirty muhūrtas of a day at a place,
/// the fifteen of the daylight and the fifteen of the night after it, in
/// order, [`muhurta::muhurta`] — the half (`day` or `night`), the
/// muhūrta's number in it, 1 to 15, its name in
/// [`muhurta::WIKIPEDIA_NAMES`], its start and its end as whole POSIX
/// seconds rounded down, what the pañcāṅga prints it as (`abhijit` for the
/// eighth of the daylight on a day but a Wednesday, `dur-muhurtam` for the
/// weekday's ones in [`muhurta::DUR_MUHURTAM`], else empty), and the four
/// cells of a missing solar event, as `hc_kalam` writes them, where the
/// Sun does not rise or set and the start and end are empty.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a day outside the sky layer's era.
pub fn muhurtas_lines(fixed: i64, place: Location) -> Answer<String> {
    use muhurta::{ABHIJIT, DUR_MUHURTAM, Half, MUHURTAS_PER_HALF, Muhurta};
    let day = day_in_era(fixed)?;
    let weekday = hc_calendar::Weekday::from_rd(day);
    let dur = DUR_MUHURTAM[weekday.sunday_first_number() as usize];
    let mut out = String::new();
    for (half, name) in [(Half::Day, "day"), (Half::Night, "night")] {
        for number in 1..=MUHURTAS_PER_HALF {
            // Every number of 1 to 15 is a muhūrta of either half.
            let Some(which) = (match half {
                Half::Day => Muhurta::day(number),
                Half::Night => Muhurta::night(number),
            }) else {
                continue;
            };
            let mark = if half == Half::Day
                && number == ABHIJIT
                && weekday != hc_calendar::Weekday::Wednesday
            {
                "abhijit"
            } else if dur.contains(&Some(which)) {
                "dur-muhurtam"
            } else {
                ""
            };
            let mut line = Line::new(&mut out);
            line.cell(name).value(number).cell(which.wikipedia_name());
            match muhurta::muhurta(which, day, place) {
                Ok(span) => {
                    line.value(unix_from_moment(span.start))
                        .value(unix_from_moment(span.end))
                        .cell(mark);
                    missing_cells(&mut line, None);
                }
                Err(missing) => {
                    line.empties(2).cell(mark);
                    missing_cells(&mut line, Some(missing));
                }
            }
            line.end();
        }
    }
    Ok(out)
}

/// How many columns each line of [`kalam_lines`] writes.
#[cfg(feature = "i18n")]
pub const KALAM_COLUMNS: usize = 8 + MISSING_COLUMNS;

/// The lines of `hc_kalam`: Rāhu kālam, Yamaganda and Gulika kālam on a
/// day by a convention of [`KalamConvention::ALL`], selected by its
/// identifier in any case, one line each in the order a pañcāṅga prints
/// them, as the period's identifier, its English name as Drik Panchang
/// prints it, its name in a locale and the tag of the data that named it,
/// by [`hc_i18n::reckonings::name_or_fallback`] — राहुकाल under `hi` —
/// the eighth of the day it takes (1 to 8), the clock its start and end
/// are read on, and the start and the end, then the four cells of a
/// missing solar event.
///
/// On `rahu-kalam-sunrise` the clock is `universal` and the start and end
/// are whole POSIX seconds, rounded down, of the eighth of the daylight at
/// the place; where the Sun does not rise or set they are empty and the
/// last cells name the missing event. On `rahu-kalam-fixed` the clock is
/// `local` and they are seconds after midnight of the day's own clock, in
/// whatever zone the caller keeps: Rāhu kālam on a Monday is 27 000 to
/// 32 400, 07:30 to 09:00, wherever it is read.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a convention not named, and
/// [`Refusal::OutOfRange`] for a day outside the sky layer's era.
#[cfg(feature = "i18n")]
pub fn kalam_lines(convention: &str, fixed: i64, place: Location, locale: &str) -> Answer<String> {
    let convention = KalamConvention::by_id(convention).ok_or(Refusal::Unknown)?;
    let day = day_in_era(fixed)?;
    let (clock, seconds): (&str, &dyn Fn(Moment) -> i64) = match convention.clock {
        SpanClock::Universal => ("universal", &unix_from_moment),
        SpanClock::Local => ("local", &|moment: Moment| {
            hc_core::math::round((moment.0 - day.0 as f64) * 86_400.0) as i64
        }),
    };
    let mut out = String::new();
    for period in Kalam::ALL {
        let mut line = Line::new(&mut out);
        line.cell(period.id).cell(period.english_name);
        reckoning_name(&mut line, locale, hc_i18n::reckonings::KALAM, period.id);
        line.value(period.part(hc_calendar::Weekday::from_rd(day)))
            .cell(clock);
        match (convention.span)(period, day, place) {
            Ok(span) => {
                line.value(seconds(span.start)).value(seconds(span.end));
                missing_cells(&mut line, None);
            }
            Err(missing) => {
                line.empties(2);
                missing_cells(&mut line, Some(missing));
            }
        }
        line.end();
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Drik Panchang's New Delhi page of 1 January 2025, a Wednesday
    /// (`drik-day-panchang-2025`, as `hc-calendars-indic`'s test reads it):
    /// Rāhu kālam from 12:25, Yamaganda from 08:32 and Gulikai kālam from
    /// 11:07 IST, each within a minute; on the fixed day Wednesday's Rāhu
    /// kālam is the fifth eighth, 12:00 to 13:30, as the temple table has
    /// it (`tirumala-kalam-table`).
    #[cfg(feature = "i18n")]
    #[test]
    fn the_first_of_january_2025_has_drik_panchangs_kalams() {
        let delhi = Location::new(
            28.0 + 38.0 / 60.0 + 8.0 / 3_600.0,
            77.0 + 13.0 / 60.0 + 28.0 / 3_600.0,
            0.0,
        );
        let day = hc_calendars_solar::gregorian::to_fixed(2025, 1, 1)
            .expect("a date")
            .0;
        let text = kalam_lines("rahu-kalam-sunrise", day, delhi, "en").expect("in range");
        let rows: alloc::vec::Vec<alloc::vec::Vec<&str>> = text
            .lines()
            .map(|line| line.split('\t').collect())
            .collect();
        assert_eq!(rows.len(), 3);
        let ist_midnight = (day - hc_calendar::fixed::RD_OF_UNIX_EPOCH) * 86_400 - 19_800;
        for (row, (id, part, start)) in rows.iter().zip([
            ("rahu-kalam", "5", 12 * 60 + 25),
            ("yamaganda", "2", 8 * 60 + 32),
            ("gulika-kalam", "4", 11 * 60 + 7),
        ]) {
            assert_eq!(row.len(), KALAM_COLUMNS);
            assert_eq!((row[0], row[4], row[5]), (id, part, "universal"));
            assert_eq!(row[1], row[2]);
            assert_eq!(row[3], "en");
            let at: i64 = row[6].parse().expect("an instant");
            let minutes = (at - ist_midnight) as f64 / 60.0 - f64::from(start);
            assert!(minutes.abs() < 1.0, "{id}: {minutes} min");
            assert_eq!(row[8..], ["", "", "", ""]);
        }
        let fixed = kalam_lines("RAHU-KALAM-FIXED", day, delhi, "hi").expect("in range");
        let first: alloc::vec::Vec<&str> =
            fixed.lines().next().expect("a line").split('\t').collect();
        // Drik Panchang's Hindi day pañcāṅga labels it राहुकाल.
        assert_eq!(
            first[2..8],
            ["राहुकाल", "hi", "5", "local", "43200", "48600"]
        );
        let tromso = Location::new(69.6496, 18.9560, 0.0);
        let midwinter = hc_calendars_solar::gregorian::to_fixed(2024, 12, 21)
            .expect("a date")
            .0;
        let polar = kalam_lines("rahu-kalam-sunrise", midwinter, tromso, "en").expect("in range");
        let cells: alloc::vec::Vec<&str> =
            polar.lines().next().expect("a line").split('\t').collect();
        assert_eq!(cells[6..10], ["", "", "sunrise", &midwinter.to_string()]);
        assert_eq!(
            kalam_lines("yamardha", day, delhi, "en"),
            Err(Refusal::Unknown)
        );
        // The names are `hc-calendars-indic`'s table's, each with its clock.
        for convention in KalamConvention::ALL {
            let text = kalam_lines(convention.id, day, delhi, "en").expect("in range");
            let clock = match convention.clock {
                SpanClock::Universal => "universal",
                SpanClock::Local => "local",
            };
            assert!(
                text.lines()
                    .all(|line| line.split('\t').nth(5) == Some(clock)),
                "{}",
                convention.id
            );
        }
    }

    /// Drik Panchang's page for 1 January 2025, as
    /// `hc_calendars_indic::panchanga`'s test reads it: "Yoga Vyaghata upto
    /// 05:07 PM" and "Karana Balava upto 02:55 PM", IST, read at sunrise:
    /// 11:37 and 09:25 UTC, POSIX 1 735 731 420 and 1 735 723 500.
    #[test]
    fn the_first_of_january_2025_carries_vyaghata_and_balava() {
        let place = Location::new(23.183_333, 82.5, 0.0);
        let day = hc_calendars_solar::gregorian::to_fixed(2025, 1, 1)
            .expect("a date")
            .0;
        let text = panchanga_of_day_lines(day, place, "lahiri").expect("a sunrise");
        let rows: alloc::vec::Vec<alloc::vec::Vec<&str>> = text
            .lines()
            .map(|line| line.split('\t').collect())
            .collect();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0][..4], ["yoga", "13", "Vyaghata", "व्याघात"]);
        assert!(rows.iter().all(|row| row.len() == 9), "{rows:?}");
        assert_eq!(rows[0][7..], ["lahiri", "Lahiri (Chitrapaksha)"]);
        // The identifier the line writes is the one the lookup reads.
        assert_eq!(ayanamsa(rows[0][7]), Ok(Ayanamsa::LAHIRI));
        let ends: i64 = rows[0][5].parse().expect("an instant");
        assert!((ends - 1_735_731_420).abs() < 90, "{ends}");
        assert_eq!(rows[1][0], "karana");
        assert_eq!(rows[1][2..4], ["Balava", "बालव"]);
        let ends: i64 = rows[1][5].parse().expect("an instant");
        assert!((0..120).contains(&(ends - 1_735_723_500)), "{ends}");
        assert_eq!(rows[1][7..], ["", ""]);
        assert_eq!(rows[0][6], rows[1][6]);
        assert_eq!(
            panchanga_of_day_lines(day, place, ""),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            panchanga_of_day_lines(day, place, "Lahiri (Chitrapaksha)"),
            Err(Refusal::Unknown)
        );
        let polar = Location::new(89.0, 0.0, 0.0);
        assert_eq!(
            panchanga_of_day_lines(day, polar, "lahiri"),
            Err(Refusal::NoData)
        );
    }

    const NEW_DELHI: Location = Location::new(
        28.0 + 38.0 / 60.0 + 8.0 / 3_600.0,
        77.0 + 13.0 / 60.0 + 28.0 / 3_600.0,
        0.0,
    );

    fn day(year: i64, month: u8, day: u8) -> i64 {
        hc_calendars_solar::gregorian::to_fixed(year, month, day)
            .expect("a date")
            .0
    }

    /// Drik Panchang's New Delhi page of 1 January 2025, a Wednesday
    /// (`drik-day-panchang-2025`): no Abhijit, and the eighth muhūrta of
    /// the day as Dur Muhurtam; the muhūrtas tile the day and the night.
    #[test]
    fn the_muhurtas_tile_the_day_and_mark_what_the_panchanga_prints() {
        let text = muhurtas_lines(day(2025, 1, 1), NEW_DELHI).expect("in range");
        let rows: Vec<Vec<&str>> = text
            .lines()
            .map(|line| line.split('\t').collect())
            .collect();
        assert_eq!(rows.len(), 30);
        assert!(rows.iter().all(|row| row.len() == MUHURTA_COLUMNS));
        let marked: Vec<(&str, &str, &str)> = rows
            .iter()
            .filter(|row| !row[5].is_empty())
            .map(|row| (row[0], row[1], row[5]))
            .collect();
        assert_eq!(marked, [("day", "8", "dur-muhurtam")]);
        // Wikipedia's names: Rudra first, Vidhi the eighth of the day,
        // Girīśa at sunset and Samudra last (`wikipedia-muhurta`).
        let names: Vec<(&str, &str, &str)> = [0, 7, 15, 29]
            .iter()
            .map(|&index| (rows[index][0], rows[index][1], rows[index][2]))
            .collect();
        assert_eq!(
            names,
            [
                ("day", "1", "Rudra"),
                ("day", "8", "Vidhi"),
                ("night", "1", "Girīśa"),
                ("night", "15", "Samudra")
            ]
        );
        for pair in rows.windows(2) {
            assert_eq!(pair[0][4], pair[1][3]);
        }
        // Thursday 2 January: Abhijit is the eighth, and Dur Muhurtam the
        // sixth and the twelfth.
        let text = muhurtas_lines(day(2025, 1, 2), NEW_DELHI).expect("in range");
        let marked: Vec<String> = text
            .lines()
            .filter_map(|line| {
                let cells: Vec<&str> = line.split('\t').collect();
                (!cells[5].is_empty()).then(|| alloc::format!("{} {}", cells[1], cells[5]))
            })
            .collect();
        assert_eq!(marked, ["6 dur-muhurtam", "8 abhijit", "12 dur-muhurtam"]);
        let polar = Location::new(78.0, 15.0, 0.0);
        let text = muhurtas_lines(day(2025, 1, 1), polar).expect("in range");
        assert!(
            text.lines()
                .all(|line| line.split('\t').nth(6) == Some("sunrise"))
        );
    }

    /// Drik Panchang prints the amṛta siddhi yoga on Tuesday 7 January
    /// 2025, from the Moon's entry into Aśvinī, and on no day between it
    /// and Saturday 11 January.
    #[test]
    fn amrita_siddhi_is_on_drik_panchangs_days() {
        let line = amrita_siddhi_line(day(2025, 1, 7), NEW_DELHI, "lahiri").expect("in range");
        let cells = crate::boundary::cells(&line);
        assert_eq!(cells.len(), AMRITA_SIDDHI_COLUMNS);
        assert_eq!(
            cells[..5],
            [
                "Amrita Siddhi Yoga",
                "अमृत सिद्धि योग",
                "1",
                "ashvini",
                "Aśvinī"
            ]
        );
        assert_eq!(cells[7..], ["1", "lahiri"]);
        let none = amrita_siddhi_line(day(2025, 1, 8), NEW_DELHI, "lahiri").expect("in range");
        // Wednesday's nakṣatra is Anurādhā.
        assert_eq!(
            crate::boundary::cells(&none)[2..8],
            ["17", "anuradha", "Anurādhā", "", "", "0"]
        );
        assert_eq!(
            amrita_siddhi_line(day(2025, 1, 7), NEW_DELHI, "tropical"),
            Err(Refusal::Unknown)
        );
    }

    /// The nakṣatra the amṛta siddhi yoga of 7 January 2025 takes is the
    /// one the Moon is in when it begins.
    #[test]
    fn the_nakshatra_is_the_moons() {
        let line = amrita_siddhi_line(day(2025, 1, 7), NEW_DELHI, "lahiri").expect("in range");
        let start: i64 = crate::boundary::cells(&line)[5].parse().expect("a start");
        let at = nakshatra_at_lines(start + 60, "lahiri").expect("in range");
        let cells = crate::boundary::cells(&at);
        assert_eq!(cells.len(), NAKSHATRA_COLUMNS);
        assert_eq!(cells[..3], ["1", "ashvini", "Aśvinī"]);
        assert!((cells[3].parse::<i64>().expect("an entry") - start).abs() <= 1);
        assert_eq!(cells[6], "lahiri");
        assert!(nakshatra_of_day_lines(day(2025, 1, 7), NEW_DELHI, "lahiri").is_ok());
    }

    /// On the Siddhānta's sky the yoga and the karaṇa are the book's, and
    /// both lines name the sky; Sewell and Dikshit's Poona pañcāṅga of
    /// September 1894 is the anchor `hc-calendars-indic` holds them to.
    #[test]
    fn the_siddhantas_sky_is_named_on_both_lines() {
        let unix = 1_700_000_000;
        let text = panchanga_at_lines(unix, "Surya-Siddhanta").expect("in range");
        let rows: Vec<Vec<&str>> = text
            .lines()
            .map(|line| line.split('\t').collect())
            .collect();
        assert_eq!(rows[0][7..], ["surya-siddhanta", "Sūrya Siddhānta"]);
        assert_eq!(rows[1][7..], ["surya-siddhanta", "Sūrya Siddhānta"]);
        let moment = moment_in_era(unix).expect("in era");
        assert_eq!(
            rows[0][1],
            alloc::format!("{}", surya_siddhanta::yoga_at(moment))
        );
        assert_eq!(
            rows[1][1],
            alloc::format!("{}", surya_siddhanta::karana_at(moment))
        );
        let text = panchanga_of_day_lines(
            day(1894, 9, 2),
            Location::new(18.52, 73.87, 0.0),
            "surya-siddhanta",
        )
        .expect("in range");
        assert_eq!(text.lines().count(), 2);
        assert_eq!(
            panchanga_of_day_lines(
                day(1894, 9, 2),
                Location::new(70.0, 0.0, 0.0),
                "surya-siddhanta"
            ),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            panchanga_at_lines(-200_000_000_000, "surya-siddhanta"),
            Err(Refusal::OutOfRange)
        );
    }
}
