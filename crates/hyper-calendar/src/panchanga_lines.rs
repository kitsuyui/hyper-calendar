//! The tab-separated lines the WebAssembly module and the C library write
//! about the pañcāṅga's yoga and karaṇa, written once.
//!
//! Each answer is two lines of the same eight columns, the yoga's first
//! and the karaṇa's second: the limb (`yoga` or `karana`), its number
//! (the yoga 1 for Viṣkambha through 27 for Vaidhṛti; the karaṇa the
//! half-tithi, 1 for the first half of śukla 1 through 60 for the second
//! half of amāvāsyā), its name as Drik Panchang spells it in English and
//! in Devanagari, the moments it began and ends and the moment it was read
//! at, each as whole POSIX seconds of Universal Time, rounded down, and
//! the ayanāṃśa the yoga was reckoned with, empty for the karaṇa, which
//! needs none. The arithmetic and the names are
//! [`hc_calendars_indic::panchanga`]'s; the instants are held to the sky
//! layer's era, [`crate::astro_lines`].
//!
//! Rāhu kālam, Yamaganda and Gulika kālam, [`hc_calendars_indic::kalam`],
//! are three more lines of their own, one a period, each named in a
//! locale, so in a build with `i18n` too: `kalam_lines`.

use alloc::string::String;
use core::fmt::Write;

use hc_astro::riseset::{Location, sunrise};
use hc_calendar::fixed::Moment;
use hc_calendars_indic::panchanga::{
    KARANA_NAMES, KARANA_NAMES_DEVANAGARI, YOGA_NAMES, YOGA_NAMES_DEVANAGARI, karana_at,
    karana_name, karana_span, yoga_at, yoga_span,
};
use hc_seasons::zodiac::Ayanamsa;

#[cfg(feature = "i18n")]
use hc_calendars_indic::kalam::{Kalam, KalamConvention, SpanClock};

#[cfg(feature = "i18n")]
use crate::astro_lines::{MISSING_COLUMNS, push_missing_cells};
use crate::astro_lines::{day_in_era, moment_in_era, unix_from_moment};
#[cfg(feature = "i18n")]
use crate::boundary::push_reckoning_name;
use crate::boundary::{Answer, Refusal, names, push_cell};

/// The ayanāṃśa a name names: one of [`Ayanamsa::ALL`] by its full name,
/// `Lahiri (Chitrapaksha)`, or by the part before the parenthesis,
/// `Lahiri`, in any case.
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other name, the empty one included: the
/// yoga moves with the ayanāṃśa, so none is assumed.
pub fn ayanamsa(name: &str) -> Answer<Ayanamsa> {
    Ayanamsa::ALL
        .into_iter()
        .find(|ayanamsa| {
            let full = ayanamsa.name();
            let short = full.split(" (").next().unwrap_or(full);
            names(name, full) || names(name, short)
        })
        .ok_or(Refusal::Unknown)
}

/// The two lines at a moment, read at the POSIX second `read_at`: the
/// caller's own instant, which a round trip through a day count could
/// move by a rounding.
fn lines_at(moment: Moment, read_at: i64, ayanamsa: Ayanamsa) -> String {
    let mut out = String::new();
    let yoga = yoga_at(moment, ayanamsa);
    let (began, ends) = yoga_span(moment, ayanamsa);
    let index = usize::from(yoga - 1);
    let _ = write!(
        out,
        "yoga\t{yoga}\t{}\t{}\t{}\t{}\t{read_at}\t",
        YOGA_NAMES[index],
        YOGA_NAMES_DEVANAGARI[index],
        unix_from_moment(began),
        unix_from_moment(ends),
    );
    push_cell(&mut out, ayanamsa.name());
    out.push('\n');
    let half = karana_at(moment);
    let (began, ends) = karana_span(moment);
    let name = usize::from(karana_name(half));
    let _ = writeln!(
        out,
        "karana\t{half}\t{}\t{}\t{}\t{}\t{read_at}\t",
        KARANA_NAMES[name],
        KARANA_NAMES_DEVANAGARI[name],
        unix_from_moment(began),
        unix_from_moment(ends),
    );
    out
}

/// The lines of `hc_panchanga_at`: the yoga and the karaṇa in progress at
/// a Universal Time instant.
///
/// # Errors
///
/// [`Refusal::Unknown`] for an ayanāṃśa [`ayanamsa`] does not name, and
/// [`Refusal::OutOfRange`] for an instant outside the sky layer's era.
pub fn panchanga_at_lines(universal_unix: i64, ayanamsa_name: &str) -> Answer<String> {
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
/// [`Refusal::Unknown`] for an ayanāṃśa [`ayanamsa`] does not name,
/// [`Refusal::OutOfRange`] for a day outside the sky layer's era, and
/// [`Refusal::NoData`] for a day on which the Sun does not rise at the
/// place: a pañcāṅga reads its day at sunrise, and no other moment is put
/// in its place.
pub fn panchanga_of_day_lines(fixed: i64, place: Location, ayanamsa_name: &str) -> Answer<String> {
    let ayanamsa = ayanamsa(ayanamsa_name)?;
    let day = day_in_era(fixed)?;
    let rise = sunrise(day, place).ok_or(Refusal::NoData)?;
    Ok(lines_at(rise, unix_from_moment(rise), ayanamsa))
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
    let convention = KalamConvention::ALL
        .iter()
        .find(|known| names(convention, known.id))
        .ok_or(Refusal::Unknown)?;
    let day = day_in_era(fixed)?;
    let (clock, seconds): (&str, &dyn Fn(Moment) -> i64) = match convention.clock {
        SpanClock::Universal => ("universal", &unix_from_moment),
        SpanClock::Local => ("local", &|moment: Moment| {
            hc_core::math::round((moment.0 - day.0 as f64) * 86_400.0) as i64
        }),
    };
    let mut out = String::new();
    for period in Kalam::ALL {
        push_cell(&mut out, period.id);
        out.push('\t');
        push_cell(&mut out, period.english_name);
        out.push('\t');
        push_reckoning_name(&mut out, locale, hc_i18n::reckonings::KALAM, period.id);
        let part = period.part(hc_calendar::Weekday::from_rd(day));
        let _ = write!(out, "\t{part}\t{clock}\t");
        match (convention.span)(period, day, place) {
            Ok(span) => {
                let _ = write!(out, "{}\t{}\t", seconds(span.start), seconds(span.end));
                push_missing_cells(&mut out, None);
            }
            Err(missing) => {
                out.push_str("\t\t");
                push_missing_cells(&mut out, Some(missing));
            }
        }
        out.push('\n');
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
        assert_eq!(rows[0][7], "Lahiri (Chitrapaksha)");
        let ends: i64 = rows[0][5].parse().expect("an instant");
        assert!((ends - 1_735_731_420).abs() < 90, "{ends}");
        assert_eq!(rows[1][0], "karana");
        assert_eq!(rows[1][2..4], ["Balava", "बालव"]);
        let ends: i64 = rows[1][5].parse().expect("an instant");
        assert!((0..120).contains(&(ends - 1_735_723_500)), "{ends}");
        assert_eq!(rows[1][7], "");
        assert_eq!(rows[0][6], rows[1][6]);
        assert_eq!(
            panchanga_of_day_lines(day, place, ""),
            Err(Refusal::Unknown)
        );
        let polar = Location::new(89.0, 0.0, 0.0);
        assert_eq!(
            panchanga_of_day_lines(day, polar, "Lahiri (Chitrapaksha)"),
            Err(Refusal::NoData)
        );
    }
}
