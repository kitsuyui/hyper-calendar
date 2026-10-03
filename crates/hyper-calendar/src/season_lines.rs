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

use hc_astro::solar::solar_longitude_after;
use hc_calendar::Rd;
use hc_seasons::dog_days::{DogDaysConvention, SpanCalendar};
use hc_seasons::hizir_kasim::NamedDay;
use hc_seasons::meiyu::PlumRainRule;
use hc_seasons::meridian::NamedMeridian;
use hc_seasons::quarter_days::{QuarterDay, QuarterDayTradition};
use hc_seasons::san_fu::{SanFu, shu_jiu_periods};
use hc_seasons::seasons::Season;
use hc_seasons::solar_terms::{TermOrder, namings, term_in_effect};
use hc_seasons::zassetsu::{self, HiganSeason, Zassetsu, ZassetsuRule};
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

/// How many columns each line of [`pentad_traditions_lines`] writes.
pub const PENTAD_TRADITION_COLUMNS: usize = 4;

/// The lines of `hc_pentad_traditions`: every tradition that names the 72
/// pentads, [`pentads::PENTAD_TRADITIONS`], in the table's order, one a
/// line — its identifier, its English name, the text its names come from,
/// and how many of its names carry an alternate reading the text prints
/// beside them.
#[must_use]
pub fn pentad_traditions_lines() -> String {
    let mut out = String::new();
    for tradition in pentads::PENTAD_TRADITIONS {
        let mut line = Line::new(&mut out);
        line.cell(tradition.id)
            .cell(tradition.english_name)
            .cell(tradition.authority)
            .value(tradition.alternates.len());
        line.end();
    }
    out
}

/// How many columns [`pentad_in_tradition_line`] writes.
pub const PENTAD_IN_TRADITION_COLUMNS: usize = 8;

/// The line of `hc_pentad_in_tradition`: the pentad in effect on a day, as
/// `hc_pentad_in_effect` finds it, named by one tradition of
/// [`pentads::PENTAD_TRADITIONS`] — the pentad's index from the first
/// pentad of 春分 at 0 through 71, its name in the tradition, its English
/// gloss, the alternate reading the tradition's text prints beside the
/// name, empty where it prints none, the fixed day the pentad began at the
/// meridian, the last fixed day before the next begins, the tradition's
/// identifier and the text its names come from.
///
/// The tradition is read before the meridian and the day.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a tradition [`pentads::by_id`] does not name or
/// a meridian [`meridian`] does not read, and [`Refusal::OutOfRange`] for a
/// day outside the years −1000 to 3000.
pub fn pentad_in_tradition_line(
    fixed: i64,
    tradition_id: &str,
    meridian_name: &str,
) -> Answer<String> {
    let tradition = pentads::by_id(tradition_id).ok_or(Refusal::Unknown)?;
    let (meridian, day) = meridian_and_day(fixed, meridian_name)?;
    let event = pentads::pentad_in_effect(day, meridian);
    let next = solar_longitude_after(event.pentad.next().solar_longitude_degrees(), event.moment);
    let end = Rd(meridian.day_of(next).0 - 1);
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(event.pentad.index(TermOrder::SpringEquinoxFirst))
        .cell(event.pentad.name(tradition))
        .cell(event.pentad.english_name(tradition))
        .cell_or_empty(event.pentad.alternate_name(tradition))
        .value(event.day.0)
        .value(end.0)
        .cell(tradition.id)
        .cell(tradition.authority);
    line.end();
    Ok(out)
}

/// How many columns each line of [`zassetsu_in_year_lines`] writes.
pub const ZASSETSU_COLUMNS: usize = 9;

/// The identifier of the rule that fixes a 雑節's day.
const fn rule_id(rule: ZassetsuRule) -> &'static str {
    match rule {
        ZassetsuRule::SolarLongitude(_) => "solar-longitude",
        ZassetsuRule::OffsetFromTerm { .. } => "offset-from-term",
        ZassetsuRule::NightsFromBeginningOfSpring(_) => "nights-from-beginning-of-spring",
        ZassetsuRule::NearestStemDay { .. } => "nearest-stem-day",
    }
}

/// The lines of `hc_zassetsu_in_year`: the 雑節 of a Gregorian year at a
/// meridian, [`zassetsu::zassetsu_in_year`], one a day, in
/// [`Zassetsu::ALL`]'s order — the identifier, the name in Japanese
/// characters, the Hepburn romaji, a short English description, the rule
/// that fixes the day (`solar-longitude`, `offset-from-term`,
/// `nights-from-beginning-of-spring` or `nearest-stem-day`), the fixed day,
/// the last day of the period it opens (the last day of the 土用 for a
/// 土用の入り, and the last day of the week for a 彼岸入り), and for a 土用の入り
/// the one or two 丑の日 of the period, empty where there is none.
///
/// Then the three days the older rules place elsewhere, each under an id of
/// its own (docs/policy.md §5), in the order `nyubai-classical`,
/// `hangesho-classical`, `spring-shanichi-classical` and
/// `autumn-shanichi-classical`: 入梅 as the first 壬 day from 芒種,
/// 半夏生 ten days after 夏至 and 社日 as the 戊 day nearest the equinox,
/// the earlier on a tie, with the rule `classical`.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a meridian [`meridian`] does not read, and
/// [`Refusal::OutOfRange`] for a year outside −1000 to 3000.
pub fn zassetsu_in_year_lines(year: i64, meridian_name: &str) -> Answer<String> {
    let meridian = meridian(meridian_name)?;
    if !(EARLIEST_YEAR..=LATEST_YEAR).contains(&year) {
        return Err(Refusal::OutOfRange);
    }
    let mut out = String::new();
    for event in zassetsu::zassetsu_in_year(year, meridian) {
        let kind = event.kind;
        let doyo = Season::ALL
            .into_iter()
            .find(|season| zassetsu::doyo_entry(*season) == kind)
            .map(|season| zassetsu::doyo(year, season, meridian));
        let higan_entry = [HiganSeason::Spring, HiganSeason::Autumn]
            .into_iter()
            .find(|season| {
                let entry = match season {
                    HiganSeason::Spring => Zassetsu::SPRING_HIGAN_ENTRY,
                    HiganSeason::Autumn => Zassetsu::AUTUMN_HIGAN_ENTRY,
                };
                entry == kind
            })
            .map(|season| zassetsu::higan(year, season, meridian));
        let mut line = Line::new(&mut out);
        line.cell(kind.id)
            .cell(kind.japanese_name())
            .cell(kind.romaji())
            .cell(kind.english_name())
            .cell(rule_id(kind.rule()))
            .value(event.day.0)
            .value_or_empty(
                doyo.map(|period| period.end.0)
                    .or(higan_entry.map(|period| period.exit.0)),
            )
            .value_or_empty(
                doyo.and_then(|period| period.first_ox_day())
                    .map(|day| day.0),
            )
            .value_or_empty(
                doyo.and_then(|period| period.second_ox_day())
                    .map(|day| day.0),
            );
        line.end();
    }
    for (id, kind, rule, day) in [
        (
            "nyubai-classical",
            Zassetsu::NYUBAI,
            "classical",
            zassetsu::classical_nyubai(year, meridian),
        ),
        (
            "hangesho-classical",
            Zassetsu::HANGESHO,
            "classical",
            zassetsu::classical_hangesho(year, meridian),
        ),
        (
            "spring-shanichi-classical",
            Zassetsu::SPRING_SHANICHI,
            "classical",
            zassetsu::classical_shanichi(year, HiganSeason::Spring, meridian),
        ),
        (
            "autumn-shanichi-classical",
            Zassetsu::AUTUMN_SHANICHI,
            "classical",
            zassetsu::classical_shanichi(year, HiganSeason::Autumn, meridian),
        ),
    ] {
        let mut line = Line::new(&mut out);
        line.cell(id)
            .cell(kind.japanese_name())
            .cell(kind.romaji())
            .cell(kind.english_name())
            .cell(rule)
            .value(day.0)
            .empties(3);
        line.end();
    }
    Ok(out)
}

/// How many columns each line of [`seasonal_days_lines`] writes.
pub const SEASONAL_DAY_COLUMNS: usize = 7;

/// A quarter day's tradition, by identifier.
const fn tradition_id(tradition: QuarterDayTradition) -> &'static str {
    match tradition {
        QuarterDayTradition::EnglandAndWales => "england-and-wales",
        QuarterDayTradition::EnglishCrossQuarter => "english-cross-quarter",
        QuarterDayTradition::Ireland => "ireland",
        QuarterDayTradition::ScotlandTraditional => "scotland-traditional",
        QuarterDayTradition::Scotland1990 => "scotland-1990",
    }
}

/// The lines of `hc_seasonal_days_in_year`: the other seasonal days and
/// spans of a Gregorian year, one a line, the kind first — the kind, the
/// identifier, the name, the name in the convention's own language, the first
/// fixed day, the last fixed day (the same day for a single day) and the
/// group (the calendar of the dates, the tradition, or empty).
///
/// In order: `san-fu`, the three 伏 at `meridian`, [`SanFu`], as
/// `chu-fu`, `zhong-fu` and `mo-fu` with their Chinese names 初伏, 中伏 and
/// 末伏, each ending the day before the next begins; `shu-jiu`, the nine
/// nines of 數九 counted from the winter solstice of the year, `1` to `9`
/// with 一九 to 九九, which run into the next year; `dog-days`, the dog days
/// under each convention of [`DogDaysConvention::ALL`], with the
/// calendar its dates are in, `gregorian` or `julian`; `quarter-day`, each
/// day of [`QuarterDay::ALL`] with its tradition; and `folk-day`, the seven
/// named days of the Turkish folk year of [`NamedDay::ALL`], Hıdırellez on
/// 6 May and the Kasım days counted from 8 November of the year before.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a meridian [`meridian`] does not read, and
/// [`Refusal::OutOfRange`] for a year outside −1000 to 3000.
pub fn seasonal_days_lines(year: i64, meridian_name: &str) -> Answer<String> {
    let meridian = meridian(meridian_name)?;
    if !(EARLIEST_YEAR..=LATEST_YEAR).contains(&year) {
        return Err(Refusal::OutOfRange);
    }
    let mut out = String::new();
    let mut push = |kind: &str, id: &str, names: (&str, &str), days: (i64, i64), group: &str| {
        let mut line = Line::new(&mut out);
        line.cell(kind)
            .cell(id)
            .cell(names.0)
            .cell(names.1)
            .value(days.0)
            .value(days.1)
            .cell(group);
        line.end();
    };
    let fu = SanFu::of_year(year, meridian);
    for (id, name, local, days) in [
        ("chu-fu", "First fu", "初伏", (fu.chu.0, fu.zhong.0 - 1)),
        ("zhong-fu", "Middle fu", "中伏", (fu.zhong.0, fu.mo.0 - 1)),
        ("mo-fu", "Last fu", "末伏", (fu.mo.0, fu.end.0 - 1)),
    ] {
        push("san-fu", id, (name, local), days, "");
    }
    const NINES: [&str; 9] = [
        "一九", "二九", "三九", "四九", "五九", "六九", "七九", "八九", "九九",
    ];
    for (index, (first, last)) in shu_jiu_periods(year, meridian).into_iter().enumerate() {
        let number = alloc::format!("{}", index + 1);
        push(
            "shu-jiu",
            &number,
            (NINES[index], NINES[index]),
            (first.0, last.0),
            "",
        );
    }
    for convention in DogDaysConvention::ALL {
        let (first, last) = convention.span(year);
        push(
            "dog-days",
            convention.id(),
            (convention.english_name(), convention.local_name()),
            (first.0, last.0),
            match convention.calendar() {
                SpanCalendar::Gregorian => "gregorian",
                SpanCalendar::Julian => "julian",
            },
        );
    }
    for quarter in QuarterDay::ALL {
        let day = quarter.day_in(year).0;
        push(
            "quarter-day",
            quarter.id,
            (quarter.english_name, quarter.local_name),
            (day, day),
            tradition_id(quarter.tradition),
        );
    }
    for named in NamedDay::ALL {
        let day = named.in_year(year).0;
        push(
            "folk-day",
            named.id(),
            (named.turkish_name(), named.turkish_name()),
            (day, day),
            "",
        );
    }
    Ok(out)
}

/// The reckoning of 寒食 an identifier names:/// The reckoning of 寒食 an identifier names: `hanshi-solstice-105`,
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

    /// The 暦Wiki's table of the 七十二候 (`nao-rekiwiki-72ko`) reads across
    /// its columns: 立春次候 is 蟄虫始振 before the 貞享暦, 梅花乃芳 in it and
    /// 黄鶯睍睆 from the 宝暦暦, and 大雪次候 is 虎(武)始交 before the 貞享暦,
    /// the 武 an alternate of the text. The two lists of the 宣明暦 and the
    /// 貞享暦 are the traditions `hc_pentad_in_tradition` names.
    #[test]
    fn a_pentad_is_named_by_each_tradition_with_the_text_s_alternate() {
        let traditions = pentad_traditions_lines();
        let rows: alloc::vec::Vec<_> = traditions.lines().map(cells).collect();
        assert_eq!(
            rows.iter()
                .map(|row| row[0])
                .collect::<alloc::vec::Vec<_>>(),
            ["chinese", "japanese", "jokyo", "senmyo"]
        );
        assert!(rows.iter().all(|row| row.len() == PENTAD_TRADITION_COLUMNS));
        assert_eq!(rows[3][3], "4");
        assert_eq!(rows[2][3], "0");
        let in_effect = |year, month, date, tradition: &str| {
            let line = pentad_in_tradition_line(day(year, month, date), tradition, "japan")
                .expect("in the era");
            cells(&line)
                .into_iter()
                .map(String::from)
                .collect::<alloc::vec::Vec<_>>()
        };
        for (tradition, name) in [
            ("senmyo", "蟄虫始振"),
            ("jokyo", "梅花乃芳"),
            ("japanese", "黄鶯睍睆"),
        ] {
            let row = in_effect(2026, 2, 10, tradition);
            assert_eq!(row.len(), PENTAD_IN_TRADITION_COLUMNS);
            assert_eq!(row[..2], ["64", name]);
            assert_eq!(row[6], tradition);
            // The days are the pentad's, whatever tradition names it.
            let plain = pentad_line(day(2026, 2, 10), "japan").expect("in the era");
            assert_eq!(row[4..6], cells(&plain)[3..5]);
        }
        let tiger = in_effect(2026, 12, 14, "SENMYO");
        assert_eq!(tiger[..2], ["52", "虎始交"]);
        assert_eq!(tiger[3], "武始交");
        assert_eq!(in_effect(2026, 12, 14, "jokyo")[3], "");
        assert_eq!(
            pentad_in_tradition_line(day(2026, 2, 10), "horyaku", "japan"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            pentad_in_tradition_line(day(4000, 1, 1), "mars", "japan"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            pentad_in_tradition_line(day(4000, 1, 1), "chinese", "japan"),
            Err(Refusal::OutOfRange)
        );
    }

    fn rows(text: &str) -> alloc::vec::Vec<alloc::vec::Vec<String>> {
        text.lines()
            .map(|line| line.split('\t').map(String::from).collect())
            .collect()
    }

    /// The 暦要項 for 2024 (`nao-rekiyoko-2024`) prints the 雑節 of the
    /// year: 土用の入り on 18 January, 節分 on 3 February, 彼岸入り on 17
    /// March, 土用の入り on 16 April, 八十八夜 on 1 May, 入梅 on 10 June,
    /// 半夏生 on 1 July, 土用の入り on 19 July, 二百十日 on 31 August,
    /// 彼岸入り on 19 September and 土用の入り on 20 October. The summer 土用
    /// of 2024 had two 丑の日, 24 July and 5 August, and ended on 6 August,
    /// the eve of 立秋.
    #[test]
    fn the_zassetsu_of_2024_are_the_rekiyoko_s() {
        let text = zassetsu_in_year_lines(2024, "japan").expect("in the era");
        let rows = rows(&text);
        assert_eq!(rows.len(), 21 + 4);
        assert!(rows.iter().all(|row| row.len() == ZASSETSU_COLUMNS));
        let day = |month, date| alloc::format!("{}", day(2024, month, date));
        let find = |id: &str| rows.iter().find(|row| row[0] == id).expect("a day");
        for (id, month, date) in [
            ("winter-doyo-entry", 1, 18),
            ("spring-setsubun", 2, 3),
            ("spring-higan-entry", 3, 17),
            ("spring-doyo-entry", 4, 16),
            ("hachijuhachiya", 5, 1),
            ("nyubai", 6, 10),
            ("hangesho", 7, 1),
            ("summer-doyo-entry", 7, 19),
            ("nihyakutoka", 8, 31),
            ("autumn-higan-entry", 9, 19),
            ("autumn-doyo-entry", 10, 20),
        ] {
            assert_eq!(find(id)[5], day(month, date), "{id}");
        }
        let summer = find("summer-doyo-entry");
        assert_eq!(
            (summer[6].as_str(), summer[7].as_str(), summer[8].as_str()),
            (day(8, 6).as_str(), day(7, 24).as_str(), day(8, 5).as_str())
        );
        assert_eq!(find("spring-higan-entry")[6], day(3, 23));
        assert_eq!(find("spring-setsubun")[4], "offset-from-term");
        assert_eq!(find("nyubai")[4], "solar-longitude");
        // The older rules are ids of their own, in the 暦Wiki's words: 入梅 as the first 壬 day from 芒種 and
        // 半夏生 ten days after 夏至.
        let classical = find("hangesho-classical");
        assert_eq!(classical[4], "classical");
        assert!(
            classical[5].parse::<i64>().expect("a day")
                >= find("hangesho")[5].parse::<i64>().expect("a day")
        );
        assert_eq!(zassetsu_in_year_lines(2024, "mars"), Err(Refusal::Unknown));
        assert_eq!(
            zassetsu_in_year_lines(3001, "japan"),
            Err(Refusal::OutOfRange)
        );
    }

    /// 2026's three 伏 begin on 15 July, 25 July and 14 August in China
    /// (Wikipedia (zh) 「三伏」, as `hc-seasons` reads it), 初伏 and 末伏
    /// ten days each; 三九 of the winter that begins in 2025, "in
    /// mid-January", is 8 to 16 January 2026 (the Hong Kong Observatory);
    /// the Old Farmer's Almanac's dog days are 3 July to 11 August and the
    /// MeteoSchweiz *Hundstage* 23 July to 23 August
    /// (`ofa-dog-days`, `meteoschweiz-hundstage-2025`); the Prayer Book's
    /// are 7 July to 5 September in the Julian calendar; and Michaelmas is
    /// 29 September.
    #[test]
    fn the_seasonal_days_of_a_year_are_their_sources() {
        let text = seasonal_days_lines(2026, "china").expect("in the era");
        let rows = rows(&text);
        assert!(rows.iter().all(|row| row.len() == SEASONAL_DAY_COLUMNS));
        let find = |kind: &str, id: &str| {
            rows.iter()
                .find(|row| row[0] == kind && row[1] == id)
                .expect("a row")
                .clone()
        };
        let d = |month, date| alloc::format!("{}", day(2026, month, date));
        let chu = find("san-fu", "chu-fu");
        assert_eq!(
            (chu[3].as_str(), chu[4].as_str(), chu[5].as_str()),
            ("初伏", d(7, 15).as_str(), d(7, 24).as_str())
        );
        let zhong = find("san-fu", "zhong-fu");
        assert_eq!(
            (zhong[4].as_str(), zhong[5].as_str()),
            (d(7, 25).as_str(), d(8, 13).as_str())
        );
        assert_eq!(find("san-fu", "mo-fu")[4], d(8, 14));
        // The winter solstice of 2025 belongs to the winter, so the nines
        // of 2026's call begin with 2026's.
        let nines: alloc::vec::Vec<_> = rows.iter().filter(|row| row[0] == "shu-jiu").collect();
        assert_eq!(nines.len(), 9);
        assert_eq!(nines[0][2], "一九");
        assert_eq!(nines[0][4], d(12, 22));
        let prev = seasonal_days_lines(2025, "china").expect("in the era");
        let prev = self::rows(&prev);
        let third = prev
            .iter()
            .filter(|row| row[0] == "shu-jiu")
            .nth(2)
            .expect("三九");
        assert_eq!(
            (third[4].as_str(), third[5].as_str()),
            (
                alloc::format!("{}", day(2026, 1, 8)).as_str(),
                alloc::format!("{}", day(2026, 1, 16)).as_str()
            )
        );
        let ofa = find("dog-days", "dog-days-old-farmers-almanac");
        assert_eq!(
            (ofa[4].as_str(), ofa[5].as_str(), ofa[6].as_str()),
            (d(7, 3).as_str(), d(8, 11).as_str(), "gregorian")
        );
        let hundstage = find("dog-days", "hundstage");
        assert_eq!(
            (hundstage[4].as_str(), hundstage[5].as_str()),
            (d(7, 23).as_str(), d(8, 23).as_str())
        );
        let prayer_book = find("dog-days", "dog-days-prayer-book-1552");
        assert_eq!(prayer_book[6], "julian");
        assert_eq!(
            prayer_book[4],
            alloc::format!(
                "{}",
                hc_calendars_solar::julian::to_fixed(2026, 7, 7)
                    .expect("a date")
                    .0
            )
        );
        let michaelmas = find("quarter-day", "england-michaelmas");
        assert_eq!(
            (michaelmas[4].as_str(), michaelmas[6].as_str()),
            (d(9, 29).as_str(), "england-and-wales")
        );
        assert_eq!(
            rows.iter().filter(|row| row[0] == "quarter-day").count(),
            20
        );
        // Hıdırellez is 6 May, and the Kasım days count from 8 November of the year before.
        assert_eq!(find("folk-day", "hidirellez")[4], d(5, 6));
        assert_eq!(
            find("folk-day", "erbain")[4],
            alloc::format!("{}", day(2026, 11, 8) + 45)
        );
        assert_eq!(seasonal_days_lines(2026, "x"), Err(Refusal::Unknown));
        assert_eq!(
            seasonal_days_lines(-1001, "china"),
            Err(Refusal::OutOfRange)
        );
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
