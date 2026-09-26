//! The tab-separated lines the WebAssembly module and the C library write
//! about the almanac, written once.
//!
//! * The solar term in effect on a day, from
//!   [`hc_seasons::solar_terms::term_in_effect`], with its traditional
//!   Chinese and Japanese names and the authority for each.
//! * The pentad (候) in effect on a day, from
//!   [`hc_seasons::pentads::pentad_in_effect`], with its name in the
//!   Chinese and the Japanese tradition and the text each comes from.
//!
//! Both are judged at a meridian a name selects, [`meridian`], and answer
//! for the days of the sky layer's era, [`crate::astro_lines::day_in_era`]:
//! the years −1000 to 3000 over which `hc-astro` states its series hold.

use alloc::string::String;
use core::fmt::Write;

use hc_calendar::Rd;
use hc_seasons::hc_astro::solar::solar_longitude_after;
use hc_seasons::solar_terms::{TermOrder, namings, term_in_effect};
use hc_seasons::{Meridian, pentads};

use crate::astro_lines::day_in_era;
use crate::boundary::{Answer, Refusal, push_cell};

/// How many columns [`term_line`] and [`pentad_line`] write.
pub const ALMANAC_COLUMNS: usize = 7;

/// The meridian a string names.
///
/// A name — `universal`, `japan`, `china`, `korea`, `india` or
/// `china-before-1929`, in any case, with the empty string meaning
/// `universal` — or a longitude in decimal degrees east of Greenwich, from
/// −180 to 180, read as local mean solar time.
///
/// # Errors
///
/// [`Refusal::Unknown`] for anything else.
pub fn meridian(name: &str) -> Answer<Meridian> {
    let lowered = name.trim().to_ascii_lowercase();
    Ok(match lowered.as_str() {
        "" | "universal" => Meridian::UNIVERSAL,
        "japan" => Meridian::JAPAN,
        "china" => Meridian::CHINA,
        "korea" => Meridian::KOREA,
        "india" => Meridian::INDIA,
        "china-before-1929" => Meridian::CHINA_BEFORE_1929,
        degrees => degrees
            .parse::<f64>()
            .ok()
            .filter(|degrees| degrees.is_finite() && (-180.0..=180.0).contains(degrees))
            .map(Meridian::from_longitude_degrees)
            .ok_or(Refusal::Unknown)?,
    })
}

/// The meridian and the day of a call, checked in that order.
fn meridian_and_day(fixed: i64, name: &str) -> Answer<(Meridian, Rd)> {
    let meridian = meridian(name)?;
    Ok((meridian, day_in_era(fixed)?))
}

/// The line of `hc_term_in_effect`: the term's index from 春分 at 0
/// through 驚蟄 at 23, its name in traditional Chinese and in Japanese,
/// the fixed day it began at the meridian, the last fixed day before the
/// next term begins, and the authority for the Chinese and for the
/// Japanese names.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a meridian [`meridian`] does not read, and
/// [`Refusal::OutOfRange`] for a day outside the years −1000 to 3000.
pub fn term_line(fixed: i64, meridian_name: &str) -> Answer<String> {
    let (meridian, day) = meridian_and_day(fixed, meridian_name)?;
    let event = term_in_effect(day, meridian);
    let next = solar_longitude_after(event.term.next().solar_longitude_degrees(), event.moment);
    let end = Rd(meridian.day_of(next).0 - 1);
    let mut out = String::new();
    let _ = write!(
        out,
        "{}\t{}\t{}\t{}\t{}\t",
        event.term.index(TermOrder::SpringEquinoxFirst),
        event.term.chinese_name(),
        event.term.japanese_name(),
        event.day.0,
        end.0
    );
    push_cell(&mut out, namings::TRADITIONAL_CHINESE.authority);
    out.push('\t');
    push_cell(&mut out, namings::JAPANESE.authority);
    out.push('\n');
    Ok(out)
}

/// The line of `hc_pentad_in_effect`: the pentad's index from the first
/// pentad of 春分 at 0 through 71, its name in the Chinese and in the
/// Japanese tradition, the fixed day it began at the meridian, the last
/// fixed day before the next pentad begins, and the text each tradition's
/// names come from.
///
/// # Errors
///
/// As [`term_line`].
pub fn pentad_line(fixed: i64, meridian_name: &str) -> Answer<String> {
    let (meridian, day) = meridian_and_day(fixed, meridian_name)?;
    let event = pentads::pentad_in_effect(day, meridian);
    let next = solar_longitude_after(event.pentad.next().solar_longitude_degrees(), event.moment);
    let end = Rd(meridian.day_of(next).0 - 1);
    let mut out = String::new();
    let _ = write!(
        out,
        "{}\t{}\t{}\t{}\t{}\t",
        event.pentad.index(TermOrder::SpringEquinoxFirst),
        event.pentad.name(pentads::CHINESE),
        event.pentad.name(pentads::JAPANESE),
        event.day.0,
        end.0
    );
    push_cell(&mut out, pentads::CHINESE.authority);
    out.push('\t');
    push_cell(&mut out, pentads::JAPANESE.authority);
    out.push('\n');
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    fn day(year: i64, month: u8, day: u8) -> i64 {
        hc_calendars_solar::gregorian::to_fixed(year, month, day)
            .expect("a date")
            .0
    }

    fn cells(line: &str) -> Vec<&str> {
        line.trim_end_matches('\n').split('\t').collect()
    }

    /// 秋分 of 2026 fell on 23 September in Japan, so on 27 September the
    /// term in effect is 秋分, index 12, begun on the 23rd; its pentad is
    /// the first of the term, 雷乃収声 in the Japanese names, index 36.
    #[test]
    fn the_autumn_equinox_is_in_effect_at_the_end_of_september_2026() {
        let term = term_line(day(2026, 9, 27), "JAPAN").expect("in the era");
        let term = cells(&term);
        assert_eq!(term.len(), ALMANAC_COLUMNS);
        assert_eq!(term[..3], ["12", "秋分", "秋分"]);
        assert_eq!(term[3], alloc::format!("{}", day(2026, 9, 23)));
        let pentad = pentad_line(day(2026, 9, 27), "japan").expect("in the era");
        let pentad = cells(&pentad);
        assert_eq!(pentad.len(), ALMANAC_COLUMNS);
        assert_eq!(pentad[0], "36");
        assert_eq!(pentad[3], alloc::format!("{}", day(2026, 9, 23)));
    }

    #[test]
    fn a_meridian_is_a_name_or_a_longitude() {
        assert_eq!(meridian(""), Ok(Meridian::UNIVERSAL));
        assert_eq!(meridian(" Korea "), Ok(Meridian::KOREA));
        assert_eq!(meridian("135"), Ok(Meridian::from_longitude_degrees(135.0)));
        assert_eq!(meridian("181"), Err(Refusal::Unknown));
        assert_eq!(meridian("mars"), Err(Refusal::Unknown));
        // The meridian is read before the day.
        assert_eq!(term_line(day(4000, 1, 1), "mars"), Err(Refusal::Unknown));
        assert_eq!(
            term_line(day(4000, 1, 1), "japan"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            pentad_line(day(-1001, 12, 31), "universal"),
            Err(Refusal::OutOfRange)
        );
    }
}
