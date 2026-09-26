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
//! the ayanamsa the yoga was reckoned with, empty for the karaṇa, which
//! needs none. The arithmetic and the names are
//! [`hc_calendars_indic::panchanga`]'s; the instants are held to the sky
//! layer's era, [`crate::astro_lines`].

use alloc::string::String;
use core::fmt::Write;

use hc_astro::riseset::{Location, sunrise};
use hc_calendar::fixed::Moment;
use hc_calendars_indic::panchanga::{
    KARANA_NAMES, KARANA_NAMES_DEVANAGARI, YOGA_NAMES, YOGA_NAMES_DEVANAGARI, karana_at,
    karana_name, karana_span, yoga_at, yoga_span,
};
use hc_seasons::zodiac::Ayanamsa;

use crate::astro_lines::{day_in_era, moment_in_era, unix_from_moment};
use crate::boundary::{Answer, Refusal, names, push_cell};

/// The ayanamsa a name names: one of [`Ayanamsa::ALL`] by its full name,
/// `Lahiri (Chitrapaksha)`, or by the part before the parenthesis,
/// `Lahiri`, in any case.
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other name, the empty one included: the
/// yoga moves with the ayanamsa, so none is assumed.
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
/// [`Refusal::Unknown`] for an ayanamsa [`ayanamsa`] does not name, and
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
/// [`Refusal::Unknown`] for an ayanamsa [`ayanamsa`] does not name,
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

#[cfg(test)]
mod tests {
    use super::*;

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
