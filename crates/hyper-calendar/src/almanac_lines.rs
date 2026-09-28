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
//!
//! [`almanac_day_lines`] writes the rest of what [`hc_almanac::day_notes()`]
//! gives a day, one annotation a line, each named in a locale from
//! [`hc_i18n::almanac`], or for the sexagenary day from `hc-i18n`'s
//! readings of the cycle: the 干支, 十二直, 二十八宿 and 二十七宿, the
//! year's, month's and day's 九星, 六曜, and every 暦注下段, 選日 and
//! modern combination that falls on the day. It computes nothing
//! `hc-almanac` does not; 七曜 is left out because it is the weekday,
//! which every locale's data already names.

use alloc::string::String;

use hc_almanac::mansions::{Fortune, namings};
use hc_almanac::{
    Combination, LowerRegister, NineStar, SelectedDay, day_notes, is_day_without_son,
    lucky_direction_of_year, nine_periods,
};
use hc_calendar::cycle::readings::JAPANESE_KUN;
use hc_calendar::gregorian::year_from_fixed;
use hc_i18n::Locale;
use hc_i18n::almanac::{self as vocabulary, AlmanacName, Term};
use hc_i18n::names::sexagenary_data;

use crate::astro_lines::day_in_era;
use crate::boundary::{Answer, Line};
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
    let mut line = Line::new(&mut out);
    line.cell(direction.japanese_name())
        .cell(direction.romaji())
        .value(direction.azimuth_degrees())
        .cell(direction.sixteen_point_name())
        .cell(direction.english_name())
        .value(period.number)
        .cell(period.japanese_name())
        .cell(period.era().chinese_name())
        .cell(period.star().japanese_name())
        .cell(period.ruling_star_name())
        .value(period.first_year)
        .value(period.last_year())
        .value_or_empty(is_day_without_son(day).map(u8::from));
    line.end();
    Ok(out)
}

/// How many columns [`almanac_day_lines`] writes.
pub const ALMANAC_DAY_COLUMNS: usize = 8;

/// The tag that asks for no locale in particular, which for the almanac
/// is its own language, Japanese.
const NATIVE: &str = "native";

/// The locale a tag asks for: `None` for [`NATIVE`], the root locale for a
/// tag that does not parse.
fn requested_locale(tag: &str) -> Option<Locale> {
    if tag == NATIVE {
        None
    } else {
        Some(Locale::parse(tag).unwrap_or(Locale::ROOT))
    }
}

/// One line: the kind and identifier, the name in the locale and the tag
/// that answered, the almanac's own Japanese name, the Hepburn reading,
/// whether the almanac counts the day auspicious, and whether it prints
/// the entry.
struct Row<'a> {
    kind: &'a str,
    id: &'a str,
    named: Option<AlmanacName>,
    japanese: &'a str,
    reading: &'a str,
    auspicious: Option<bool>,
    printed: Option<bool>,
}

fn push_row(out: &mut String, row: &Row<'_>) {
    let mut line = Line::new(out);
    line.cell(row.kind)
        .cell(row.id)
        .cell_or_empty(row.named.map(|named| named.name))
        .cell_or_empty(row.named.map(|named| named.tag))
        .cell(row.japanese)
        .cell(row.reading)
        .value_or_empty(row.auspicious.map(u8::from))
        .value_or_empty(row.printed.map(u8::from));
    line.end();
}

/// The 1-based position of a cycle's term, as its identifier.
fn ordinal(position: usize) -> String {
    alloc::format!("{}", position + 1)
}

/// A nine star's line, of the year, the month or the day.
fn push_star(out: &mut String, kind: &str, star: NineStar, locale: Option<&Locale>) {
    let position = usize::from(star.number()) - 1;
    push_row(
        out,
        &Row {
            kind,
            id: &ordinal(position),
            named: vocabulary::name_or_fallback(
                locale,
                vocabulary::NINE_STAR,
                Term::Position(position),
            ),
            japanese: star.japanese_name(),
            reading: star.romaji(),
            auspicious: None,
            printed: None,
        },
    );
}

/// The lines of `hc_almanac_day`: every almanac annotation
/// [`hc_almanac::day_notes()`] gives a day at a meridian, one a line, in the
/// order a printed almanac page gives them — the sexagenary day
/// (`sexagenary`), 十二直 (`twelve-direct`), 二十八宿 (`mansion`), 二十七宿
/// (`mansion-27`), the year's, the month's and the day's 九星 (`year-star`,
/// `month-star`, `day-star`), 六曜 (`rokuyo`), then each 暦注下段
/// (`lower-register`), 選日 (`selected-day`) and combination
/// (`combination`) that falls, in `hc-almanac`'s listing order.
///
/// Each line: the kind; the identifier, the term's 1-based position in its
/// cycle (甲子 1, 建 1, 角 1, 一白水星 1, 先勝 1) or an entry's
/// `hc-almanac` identifier (`tenshanichi`); its name in the locale and the
/// tag of the data that named it, by [`vocabulary::name_or_fallback`]'s
/// rule — the locale's where it has one, else English's, else Japanese's,
/// Japanese first under `native` — and for the sexagenary day the locale's
/// reading of the cycle by the same rule; the almanac's own Japanese name;
/// its Hepburn reading, the Sino-Japanese reading for a mansion and the
/// kun readings for the sexagenary day, empty for a combination, which has
/// none; `1` where the almanac counts the day auspicious and `0` where
/// inauspicious, empty where it gives no verdict or the kind has none; and,
/// for the lower register only, `1` where an almanac prints the entry and
/// `0` where 受死日 or 十死日, which are printed alone, suppress it.
///
/// # Errors
///
/// [`crate::boundary::Refusal::Unknown`] for a meridian
/// [`crate::season_lines::meridian`] does not read, and
/// [`crate::boundary::Refusal::OutOfRange`] for a day outside the sky
/// layer's era.
pub fn almanac_day_lines(fixed: i64, meridian_name: &str, locale: &str) -> Answer<String> {
    let meridian = meridian(meridian_name)?;
    let day = day_in_era(fixed)?;
    let requested = requested_locale(locale);
    let locale = requested.as_ref();
    let notes = day_notes(day, meridian);
    let mut out = String::new();

    let sexagenary = notes.sexagenary();
    let (kun_stem, kun_branch) = JAPANESE_KUN.pair(sexagenary);
    let in_reading = |data: &'static hc_i18n::LocaleData| {
        data.cycle.reading.map(|reading| {
            let (stem, branch) = reading.pair(sexagenary);
            (
                alloc::format!("{stem}{}{branch}", data.cycle.joiner),
                data.tag,
            )
        })
    };
    let english = Locale::parse("en").ok();
    let japanese = Locale::parse("ja").ok();
    let data_of = |locale: Option<&Locale>| locale.and_then(sexagenary_data);
    let chain = match locale {
        Some(locale) => [Some(locale), english.as_ref(), japanese.as_ref()],
        None => [japanese.as_ref(), english.as_ref(), None],
    };
    let named = chain
        .into_iter()
        .find_map(|candidate| data_of(candidate).and_then(in_reading));
    let (han_stem, han_branch) = hc_calendar::cycle::readings::HAN.pair(sexagenary);
    let mut line = Line::new(&mut out);
    line.cell("sexagenary")
        .value(usize::from(sexagenary.index()) + 1)
        .cell_or_empty(named.as_ref().map(|(name, _)| name.as_str()))
        .cell_or_empty(named.as_ref().map(|(_, tag)| *tag))
        .value(format_args!("{han_stem}{han_branch}"))
        .value(format_args!("{kun_stem} {kun_branch}"))
        .empties(2);
    line.end();

    let direct = notes.twelve_direct();
    let position = usize::from(direct.index());
    push_row(
        &mut out,
        &Row {
            kind: "twelve-direct",
            id: &ordinal(position),
            named: vocabulary::name_or_fallback(
                locale,
                vocabulary::TWELVE_DIRECT,
                Term::Position(position),
            ),
            japanese: direct.japanese_name(),
            reading: direct.romaji(),
            auspicious: None,
            printed: None,
        },
    );

    let mansion = notes.mansion();
    let position = usize::from(mansion.index());
    push_row(
        &mut out,
        &Row {
            kind: "mansion",
            id: &ordinal(position),
            named: vocabulary::name_or_fallback(
                locale,
                vocabulary::MANSION,
                Term::Position(position),
            ),
            japanese: mansion.japanese_name(),
            reading: mansion.name(&namings::ON_READING),
            auspicious: Some(mansion.fortune() == Fortune::Auspicious),
            printed: None,
        },
    );
    let mansion27 = notes.mansion27();
    let as_28 = mansion27.to_twenty_eight();
    push_row(
        &mut out,
        &Row {
            kind: "mansion-27",
            id: &ordinal(usize::from(mansion27.index())),
            named: vocabulary::name_or_fallback(
                locale,
                vocabulary::MANSION,
                Term::Position(usize::from(as_28.index())),
            ),
            japanese: mansion27.japanese_name(),
            reading: as_28.name(&namings::ON_READING),
            auspicious: None,
            printed: None,
        },
    );

    let stars = notes.nine_stars();
    push_star(&mut out, "year-star", stars.year, locale);
    push_star(&mut out, "month-star", stars.month, locale);
    push_star(&mut out, "day-star", stars.day, locale);

    let rokuyo = notes.rokuyo();
    let position = usize::from(rokuyo.cycle_index());
    push_row(
        &mut out,
        &Row {
            kind: "rokuyo",
            id: &ordinal(position),
            named: vocabulary::name_or_fallback(
                locale,
                vocabulary::ROKUYO,
                Term::Position(position),
            ),
            japanese: rokuyo.japanese_name(),
            reading: rokuyo.romaji(),
            auspicious: None,
            printed: None,
        },
    );

    let lower = notes.lower_register();
    let printed = lower.as_printed();
    for entry in lower.iter() {
        push_lower(&mut out, entry, printed.contains(entry), locale);
    }
    for entry in notes.selected_days().iter() {
        push_selected(&mut out, entry, locale);
    }
    for entry in notes.combinations().iter() {
        push_combination(&mut out, entry, locale);
    }
    Ok(out)
}

fn push_lower(out: &mut String, entry: LowerRegister, printed: bool, locale: Option<&Locale>) {
    push_row(
        out,
        &Row {
            kind: vocabulary::LOWER_REGISTER,
            id: entry.id,
            named: vocabulary::name_or_fallback(
                locale,
                vocabulary::LOWER_REGISTER,
                Term::Id(entry.id),
            ),
            japanese: entry.japanese_name(),
            reading: entry.romaji(),
            auspicious: Some(entry.is_auspicious()),
            printed: Some(printed),
        },
    );
}

fn push_selected(out: &mut String, entry: SelectedDay, locale: Option<&Locale>) {
    push_row(
        out,
        &Row {
            kind: vocabulary::SELECTED_DAY,
            id: entry.id,
            named: vocabulary::name_or_fallback(
                locale,
                vocabulary::SELECTED_DAY,
                Term::Id(entry.id),
            ),
            japanese: entry.japanese_name(),
            reading: entry.romaji(),
            auspicious: entry.is_auspicious(),
            printed: None,
        },
    );
}

fn push_combination(out: &mut String, entry: Combination, locale: Option<&Locale>) {
    push_row(
        out,
        &Row {
            kind: vocabulary::COMBINATION,
            id: entry.id,
            named: vocabulary::name_or_fallback(
                locale,
                vocabulary::COMBINATION,
                Term::Id(entry.id),
            ),
            japanese: entry.japanese_name(),
            reading: "",
            auspicious: (!entry.is_a_clash()).then_some(true),
            printed: None,
        },
    );
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

    fn day_rows(fixed: i64, locale: &str) -> Vec<Vec<String>> {
        let text = almanac_day_lines(fixed, "japan", locale).expect("in range");
        text.lines()
            .map(|line| {
                let cells: Vec<String> = line.split('\t').map(String::from).collect();
                assert_eq!(cells.len(), ALMANAC_DAY_COLUMNS, "{line}");
                cells
            })
            .collect()
    }

    fn row<'a>(rows: &'a [Vec<String>], kind: &str) -> &'a [String] {
        rows.iter()
            .find(|row| row[0] == kind)
            .unwrap_or_else(|| panic!("no {kind}"))
    }

    /// 21 December 2025: 赤口, with 一粒万倍日 and 天赦日 in arachne.jp's
    /// 2025 大安 calendar (`arachne-taian-2025-12`), and 甲子 and 天恩日 as
    /// well in マイナビニュース's article of the day (`mynavi-2025-12-21`),
    /// so the 天赦日＋一粒万倍日 combination; the day's star 一白, as
    /// `hc_almanac::day_notes` has it at the 甲子 the 九星 count reverses
    /// on. Named in Japanese under `ja` and `native`, in English's Hepburn
    /// readings under `en` and under `de`, which has no table, and the
    /// combination, which English does not name, in Japanese.
    #[test]
    fn the_twenty_first_of_december_2025_is_written_as_the_almanacs_print_it() {
        let day = gregorian::to_fixed(2025, 12, 21).expect("a date").0;
        let rows = day_rows(day, "ja");
        let kinds: Vec<&str> = rows.iter().map(|row| row[0].as_str()).collect();
        assert_eq!(
            kinds,
            [
                "sexagenary",
                "twelve-direct",
                "mansion",
                "mansion-27",
                "year-star",
                "month-star",
                "day-star",
                "rokuyo",
                "lower-register",
                "lower-register",
                "selected-day",
                "selected-day",
                "combination"
            ]
        );
        assert_eq!(
            row(&rows, "sexagenary"),
            ["sexagenary", "1", "甲子", "ja", "甲子", "kinoe ne", "", ""]
        );
        assert_eq!(
            row(&rows, "rokuyo"),
            ["rokuyo", "6", "赤口", "ja", "赤口", "shakkō", "", ""]
        );
        assert_eq!(
            row(&rows, "day-star")[..4],
            ["day-star", "1", "一白水星", "ja"]
        );
        let ids: Vec<&str> = rows[8..].iter().map(|row| row[1].as_str()).collect();
        assert_eq!(
            ids,
            [
                "tenonnichi",
                "tenshanichi",
                "ichiryu-manbai",
                "kinoene",
                "pardon-and-grain"
            ]
        );
        assert_eq!(
            rows[9],
            [
                "lower-register",
                "tenshanichi",
                "天赦日",
                "ja",
                "天赦日",
                "tenshanichi",
                "1",
                "1"
            ]
        );
        assert_eq!(day_rows(day, "native"), rows);
        assert_eq!(day_rows(day, "ja-JP"), rows);
        let english = day_rows(day, "en");
        assert_eq!(english[0][2..4], ["jia-zi", "en"]);
        assert_eq!(english[7][2..4], ["shakkō", "en"]);
        assert_eq!(english[12][2..4], ["天赦日＋一粒万倍日", "ja"]);
        assert_eq!(day_rows(day, "de"), english);
        for (english, japanese) in english.iter().zip(&rows) {
            assert_eq!(english[..2], japanese[..2]);
            assert_eq!(english[4..], japanese[4..]);
        }
    }

    /// 1 January 2024 (`hc_almanac`'s own anchor): 甲子, 建, 畢宿 — Net in
    /// English, auspicious in the commonest listing — and a 天赦日; the
    /// twenty-seven count, which restarts at each new moon, on 軫, and the
    /// year's star still 2023's 四緑 before 立春. Every day of a year writes
    /// every cycle once and names every term in every locale.
    #[test]
    fn every_line_is_named_and_every_cycle_written() {
        let day = gregorian::to_fixed(2024, 1, 1).expect("a date").0;
        let rows = day_rows(day, "en");
        assert_eq!(
            row(&rows, "twelve-direct")[..3],
            ["twelve-direct", "1", "tatsu"]
        );
        assert_eq!(
            row(&rows, "mansion"),
            ["mansion", "19", "Net", "en", "畢", "hitsu", "1", ""]
        );
        assert_eq!(
            row(&rows, "mansion-27")[..5],
            ["mansion-27", "27", "Chariot", "en", "軫"]
        );
        assert_eq!(row(&rows, "year-star")[4], "四緑木星");
        for offset in (0..366).step_by(7) {
            for locale in ["ja", "en", "fr", "native", "zh-Hans", "ko"] {
                let rows = day_rows(day + offset, locale);
                assert_eq!(rows.iter().filter(|row| row[0] == "rokuyo").count(), 1);
                assert_eq!(rows.iter().filter(|row| row[0] == "day-star").count(), 1);
                for row in &rows {
                    assert!(!row[2].is_empty() && !row[3].is_empty(), "{row:?}");
                    assert!(!row[4].is_empty(), "{row:?}");
                }
            }
        }
        assert_eq!(almanac_day_lines(day, "mars", "ja"), Err(Refusal::Unknown));
        assert_eq!(
            almanac_day_lines(i64::MIN, "japan", "ja"),
            Err(Refusal::OutOfRange)
        );
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
