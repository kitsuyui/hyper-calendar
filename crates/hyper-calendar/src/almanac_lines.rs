//! The tab-separated line the WebAssembly module and the C library write
//! about the almanac's cycles of a day, written once.
//!
//! Three cycles of [`hc_almanac`], each read its own way:
//!
//! * 恵方, [`hc_almanac::lucky_direction`]: the direction of the year's
//!   heavenly stem, the year taken by its Gregorian number, as the two
//!   customs its sources read, 恵方参り on New Year's Day and the 恵方巻 of
//!   節分, take it.
//! * 三元九運, [`hc_almanac::nine_periods`]: the period of twenty years in
//!   force, the year turning at 立春 at a meridian, which
//!   [`crate::season_lines::meridian`] reads.
//! * 손 없는 날, [`hc_almanac::days_without_son`]: whether the day's number
//!   in the Korean lunar calendar, `dangi`, is 9, 10, 19, 20, 29 or 30.
//!
//! The day answers for the sky layer's era, [`crate::astro_lines`], since
//! 立春 is a solar term; `dangi` answers for fewer years, and outside them
//! the last cell is empty.

use alloc::string::String;
use core::fmt::Write;

use hc_almanac::{is_day_without_son, lucky_direction_of_year, nine_periods};
use hc_calendar::gregorian::year_from_fixed;

use crate::astro_lines::day_in_era;
use crate::boundary::Answer;
use crate::season_lines::meridian;

/// How many columns [`almanac_cycles_line`] writes.
pub const ALMANAC_CYCLE_COLUMNS: usize = 13;

/// The line of `hc_almanac_cycles`: 恵方 of the day's Gregorian year — its
/// point of the twenty-four (甲, 庚, 丙 or 壬), the point's reading in
/// Hepburn romaji, its azimuth in degrees clockwise from north, the nearest
/// of the sixteen compass points in Japanese and in English — then the
/// 三元九運 period in force at the meridian — its number, 1 to 9, its name
/// (九運), its era (上元, 中元 or 下元), the 九星 that rules it (九紫火星),
/// the star of the Dipper the source names as its ruler (右弼), and the
/// first and last years it covers, each from 立春 — and last whether the
/// day is 손 없는 날, `1` or `0`, empty outside the `dangi` calendar's
/// years.
///
/// # Errors
///
/// [`crate::boundary::Refusal::Unknown`] for a meridian
/// [`crate::season_lines::meridian`] does not read, and
/// [`crate::boundary::Refusal::OutOfRange`] for a day outside the sky
/// layer's era.
pub fn almanac_cycles_line(fixed: i64, meridian_name: &str) -> Answer<String> {
    let meridian = meridian(meridian_name)?;
    let day = day_in_era(fixed)?;
    let direction = lucky_direction_of_year(year_from_fixed(day));
    let period = nine_periods::period(day, meridian);
    let mut out = String::new();
    let _ = write!(
        out,
        "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t",
        direction.japanese_name(),
        direction.romaji(),
        direction.azimuth_degrees(),
        direction.sixteen_point_name(),
        direction.english_name(),
        period.number,
        period.japanese_name(),
        period.era().chinese_name(),
        period.star().japanese_name(),
        period.ruling_star_name(),
        period.first_year,
        period.last_year(),
    );
    if let Some(without_son) = is_day_without_son(day) {
        let _ = write!(out, "{}", u8::from(without_son));
    }
    out.push('\n');
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::Refusal;
    use alloc::vec::Vec;
    use hc_calendars_solar::gregorian;

    fn cells(fixed: i64, meridian: &str) -> Vec<String> {
        let line = almanac_cycles_line(fixed, meridian).expect("in range");
        let cells: Vec<String> = line
            .trim_end_matches('\n')
            .split('\t')
            .map(String::from)
            .collect();
        assert_eq!(cells.len(), ALMANAC_CYCLE_COLUMNS);
        cells
    }

    /// 節分, 3 February 2026: the 恵方巻 face 丙, 南南東, 165° (All About,
    /// `allabout-eho-2026`; JRE Media, `jre-eho-2026`). 九運 has ruled since
    /// 立春 2024 (`chanweitang-sanyuan-jiuyun`), and 7 February 2026 is 손
    /// 없는 날 on the published list of 2026 and 9 February is not.
    #[test]
    fn setsubun_2026_faces_south_south_east_in_the_ninth_period() {
        let setsubun = gregorian::to_fixed(2026, 2, 3).expect("a date").0;
        let row = cells(setsubun, "japan");
        assert_eq!(
            row[..12],
            [
                "丙",
                "hinoe",
                "165",
                "南南東",
                "south-south-east",
                "9",
                "九運",
                "下元",
                "九紫火星",
                "右弼",
                "2024",
                "2043"
            ]
        );
        let listed = gregorian::to_fixed(2026, 2, 7).expect("a date").0;
        assert_eq!(cells(listed, "korea")[12], "1");
        let unlisted = gregorian::to_fixed(2026, 2, 9).expect("a date").0;
        assert_eq!(cells(unlisted, "korea")[12], "0");
    }

    /// "从2024年立春起": at the Chinese meridian 3 February 2024 is still
    /// 八運 and 4 February 九運; the 恵方 is the Gregorian year's either way.
    #[test]
    fn the_period_turns_at_the_beginning_of_spring_and_the_direction_at_new_year() {
        let before = gregorian::to_fixed(2024, 2, 3).expect("a date").0;
        let row = cells(before, "china");
        assert_eq!((row[5].as_str(), row[10].as_str()), ("8", "2004"));
        assert_eq!(row[0], "甲");
        let after = cells(before + 1, "china");
        assert_eq!(after[5], "9");
        let old = gregorian::to_fixed(1600, 1, 1).expect("a date").0;
        assert_eq!(cells(old, "")[12], "");
        assert_eq!(almanac_cycles_line(before, "mars"), Err(Refusal::Unknown));
        assert_eq!(
            almanac_cycles_line(i64::MIN, "china"),
            Err(Refusal::OutOfRange)
        );
    }
}
