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
//! * 寒食, the Cold Food Day, of a year under a named reckoning, from
//!   [`hc_seasons::cold_food`], counted from a solar term.
//!
//! The first two are judged at a meridian a name selects, [`meridian`];
//! the reckonings of 寒食 each name their own. All answer for the days of
//! the sky layer's era, [`crate::astro_lines::day_in_era`]: the years −1000
//! to 3000 over which `hc-astro` states its series hold.

use alloc::string::String;

use hc_calendar::Rd;
use hc_seasons::hc_astro::solar::solar_longitude_after;
use hc_seasons::meiyu::PlumRainRule;
use hc_seasons::meridian::NamedMeridian;
use hc_seasons::solar_terms::{TermOrder, namings, term_in_effect};
use hc_seasons::{ColdFoodConvention, Meridian, pentads};

use crate::astro_lines::{EARLIEST_YEAR, LATEST_YEAR, day_in_era};
use crate::boundary::{Answer, Line, Refusal};

/// How many columns [`term_line`] and [`pentad_line`] write.
pub const ALMANAC_COLUMNS: usize = 7;

/// The meridian a string names.
///
/// A name of [`NamedMeridian::ALL`] — `universal`, `japan`, `china`,
/// `korea`, `india` or `china-before-1929` — with the empty string meaning
/// `universal`, or a longitude in decimal degrees east of Greenwich, from
/// −180 to 180, read as local mean solar time.
///
/// # Errors
///
/// [`Refusal::Unknown`] for anything else.
pub fn meridian(name: &str) -> Answer<Meridian> {
    let name = name.trim();
    if name.is_empty() {
        return Ok(Meridian::UNIVERSAL);
    }
    if let Some(named) = NamedMeridian::by_id(name) {
        return Ok(named.meridian);
    }
    name.parse::<f64>()
        .ok()
        .filter(|degrees| degrees.is_finite() && (-180.0..=180.0).contains(degrees))
        .map(Meridian::from_longitude_degrees)
        .ok_or(Refusal::Unknown)
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
    let mut line = Line::new(&mut out);
    line.value(event.term.index(TermOrder::SpringEquinoxFirst))
        .cell(event.term.chinese_name())
        .cell(event.term.japanese_name())
        .value(event.day.0)
        .value(end.0)
        .cell(namings::TRADITIONAL_CHINESE.authority)
        .cell(namings::JAPANESE.authority);
    line.end();
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
    let mut line = Line::new(&mut out);
    line.value(event.pentad.index(TermOrder::SpringEquinoxFirst))
        .cell(event.pentad.name(pentads::CHINESE))
        .cell(event.pentad.name(pentads::JAPANESE))
        .value(event.day.0)
        .value(end.0)
        .cell(pentads::CHINESE.authority)
        .cell(pentads::JAPANESE.authority);
    line.end();
    Ok(out)
}

/// The reckoning of 寒食 an identifier names: `hanshi-solstice-105`,
/// `hanshi-eve-of-qingming` or `hansik`, by [`ColdFoodConvention::by_id`].
///
/// # Errors
///
/// [`Refusal::Unknown`] for anything else.
pub fn cold_food_convention(id: &str) -> Answer<ColdFoodConvention> {
    ColdFoodConvention::by_id(id).ok_or(Refusal::Unknown)
}

/// The fixed day of 寒食 in Gregorian `year` under the reckoning `id`
/// names: in April, or at the very end of March, every year.
///
/// The two solstice reckonings count from the winter solstice of the year
/// before, so the years are those whose solstice and whose April are both
/// in the era: `EARLIEST_YEAR + 1` through `LATEST_YEAR`, for every
/// reckoning alike.
///
/// # Errors
///
/// [`Refusal::Unknown`] for an identifier [`cold_food_convention`] does not
/// read, and [`Refusal::OutOfRange`] for a year outside −999 to 3000.
pub fn cold_food_day(id: &str, year: i64) -> Answer<i64> {
    let convention = cold_food_convention(id)?;
    if !(EARLIEST_YEAR + 1..=LATEST_YEAR).contains(&year) {
        return Err(Refusal::OutOfRange);
    }
    Ok(convention.day(year).0)
}

/// The day of 入梅 or 出梅 in Gregorian `year` by a rule of
/// [`PlumRainRule::ALL`], selected by its identifier, with the solar term
/// it counts from at a meridian [`meridian`] reads: the first 丙 or 壬 day
/// from 芒种, or the first 未 day from 小暑, the term's own day counted.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a rule or a meridian not named, and
/// [`Refusal::OutOfRange`] for a year outside −1000 to 3000.
pub fn plum_rains_day(rule: &str, year: i64, meridian_name: &str) -> Answer<i64> {
    let rule = PlumRainRule::by_id(rule).ok_or(Refusal::Unknown)?;
    let meridian = meridian(meridian_name)?;
    if !(EARLIEST_YEAR..=LATEST_YEAR).contains(&year) {
        return Err(Refusal::OutOfRange);
    }
    Ok((rule.day)(year, meridian).0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(year: i64, month: u8, day: u8) -> i64 {
        hc_calendars_solar::gregorian::to_fixed(year, month, day)
            .expect("a date")
            .0
    }

    use crate::boundary::cells;

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

    /// The Korea Astronomy and Space Science Institute's 월력요항 press
    /// releases (`kasi-wollyeok`): "한식은 4월 5일(금)" in 2024, "4월
    /// 5일(토)" in 2025 and "4월 6일(월)" in 2026.
    #[test]
    fn hansik_is_where_kasi_puts_it_and_the_chinese_reckonings_bracket_qingming() {
        for (year, month, date) in [(2024, 4, 5), (2025, 4, 5), (2026, 4, 6)] {
            assert_eq!(cold_food_day("hansik", year), Ok(day(year, month, date)));
            assert_eq!(cold_food_day(" HANSIK ", year), Ok(day(year, month, date)));
        }
        for year in [2024, 2025, 2026] {
            let eve = cold_food_day("hanshi-eve-of-qingming", year).expect("in the era");
            let older = cold_food_day("hanshi-solstice-105", year).expect("in the era");
            assert!(matches!(older - eve, 1 | 2), "{year}");
            let qingming = hc_seasons::solar_terms::term_day(
                year,
                hc_seasons::SolarTerm::from_degrees(15).expect("清明"),
                Meridian::CHINA,
            );
            assert_eq!(eve, qingming.0 - 1, "{year}");
        }
        assert_eq!(cold_food_day("hanshi", 2026), Err(Refusal::Unknown));
        assert!(cold_food_day("hansik", -999).is_ok());
        assert!(cold_food_day("hansik", 3000).is_ok());
        assert_eq!(cold_food_day("hansik", -1000), Err(Refusal::OutOfRange));
        assert_eq!(cold_food_day("hansik", 3001), Err(Refusal::OutOfRange));
        // The identifier is read before the year.
        assert_eq!(cold_food_day("mars", 3001), Err(Refusal::Unknown));
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

    /// 入梅 on 11 June 2026 and 出梅 on 8 July at the Chinese meridian
    /// (`qq-meiyu-2026`); 入梅 on 12 June 2025 by Central China's 壬 rule
    /// (`qq-meiyu-2025`); 出梅 of 2024 on 小暑 itself, 6 July
    /// (`qq-meiyu-2024`).
    #[test]
    fn the_plum_rains_fall_on_the_published_days() {
        assert_eq!(
            plum_rains_day("ru-mei-bing", 2026, "china"),
            Ok(day(2026, 6, 11))
        );
        assert_eq!(
            plum_rains_day("CHU-MEI-WEI", 2026, "china"),
            Ok(day(2026, 7, 8))
        );
        assert_eq!(
            plum_rains_day("ru-mei-ren", 2025, "china"),
            Ok(day(2025, 6, 12))
        );
        assert_eq!(
            plum_rains_day("chu-mei-wei", 2024, "china"),
            Ok(day(2024, 7, 6))
        );
        assert_eq!(
            plum_rains_day("ru-mei", 2026, "china"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            plum_rains_day("ru-mei-bing", 2026, "mars"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            plum_rains_day("ru-mei-bing", 3001, "china"),
            Err(Refusal::OutOfRange)
        );
        assert!(plum_rains_day("ru-mei-bing", -1000, "").is_ok());
    }
}
