//! The tab-separated lines the WebAssembly module and the C library write
//! about cultural attributions to calendar units, written once.
//!
//! Everything here reads [`hc_attributes`], which **reports what traditions
//! claim and asserts none of them**: there is no "the birthstone of March",
//! there are eight lists of them, each with an authority, a date, a region and
//! the years it was current. So there is no export that returns one answer.
//! Every line names its list, and a question about a month asks every list
//! at once, which is usually the honest answer: they disagree in eleven
//! months out of twelve.
//!
//! The subjects are `birthstone`, `birth-flower`, `moon-name` (the full-moon
//! names, a contested attribution whose caveat travels with every line),
//! `lunation-name` (Carver's 1778 names, counted from the March equinox and
//! not by month), `month-name` (Old English, Frankish, Finnish and Czech),
//! `zodiac-stone` (the older stones by sign) and `weekday`. A key is the
//! month from 1, the lunation from 1, the sign from Aries = 1, or the ISO
//! weekday from Monday = 1 to Sunday = 7.

use alloc::string::String;
use alloc::vec::Vec;

use hc_attributes::authority::Authority;
use hc_attributes::gaps;
use hc_attributes::{birth_flowers, birthstones, month_names, moon_names, weekday_attributions};
use hc_calendar::{Rd, gregorian};

use crate::boundary::{Answer, Line, Refusal};

/// How many columns a line of [`authorities_lines`] has.
pub const AUTHORITY_COLUMNS: usize = 17;

/// How many columns a line of [`attributions_lines`] and
/// [`attributions_on_lines`] has.
pub const ATTRIBUTION_COLUMNS: usize = 13;

/// How many columns a line of [`harvest_moon_line`] has.
#[cfg(feature = "seasons")]
pub const HARVEST_MOON_COLUMNS: usize = 7;

/// What a table of a subject is keyed by.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Key {
    Month,
    Lunation,
    #[cfg(feature = "seasons")]
    Sign,
    Weekday,
}

impl Key {
    const fn id(self) -> &'static str {
        match self {
            Self::Month => "month",
            Self::Lunation => "lunation",
            #[cfg(feature = "seasons")]
            Self::Sign => "sign",
            Self::Weekday => "weekday",
        }
    }

    /// How many keys the kind has.
    const fn count(self) -> usize {
        match self {
            Self::Weekday => 7,
            _ => 12,
        }
    }
}

/// The subjects, in the order the lines of a day write them.
const SUBJECTS: [&str; 7] = [
    "birthstone",
    "birth-flower",
    "moon-name",
    "lunation-name",
    "month-name",
    "zodiac-stone",
    "weekday",
];

/// One list of a subject: its authority and its entries in key order, with a
/// gloss for each where the list has them.
struct Listing {
    authority: &'static Authority,
    entries: Vec<&'static [&'static str]>,
    glosses: Vec<&'static str>,
}

/// The lists of a subject and what they are keyed by, in the crate's order.
fn listings(subject: &str) -> Answer<(Key, Vec<Listing>)> {
    let plain = |tables: &[&'static hc_attributes::MonthTable]| -> Vec<Listing> {
        tables
            .iter()
            .map(|table| Listing {
                authority: table.authority(),
                entries: table.iter().collect(),
                glosses: Vec::new(),
            })
            .collect()
    };
    let name = subject.trim().to_ascii_lowercase();
    Ok(match name.as_str() {
        "birthstone" => (Key::Month, plain(&birthstones::ALL)),
        "birth-flower" => (Key::Month, plain(&birth_flowers::ALL)),
        "moon-name" => (Key::Month, plain(&moon_names::ALL)),
        "lunation-name" => (Key::Lunation, plain(&[&moon_names::MOON_NAMES_CARVER_1778])),
        "month-name" => (
            Key::Month,
            month_names::ALL
                .iter()
                .map(|set| Listing {
                    authority: set.authority(),
                    entries: set.iter().map(|(names, _)| names).collect(),
                    glosses: set.iter().map(|(_, gloss)| gloss).collect(),
                })
                .collect(),
        ),
        #[cfg(feature = "seasons")]
        "zodiac-stone" => (
            Key::Sign,
            hc_attributes::zodiac_stones::ALL
                .iter()
                .map(|table| Listing {
                    authority: table.authority(),
                    entries: table.iter().collect(),
                    glosses: Vec::new(),
                })
                .collect(),
        ),
        "weekday" => (
            Key::Weekday,
            weekday_attributions::ALL
                .iter()
                .map(|table| Listing {
                    authority: table.authority(),
                    entries: table.iter().collect(),
                    glosses: Vec::new(),
                })
                .collect(),
        ),
        _ => return Err(Refusal::Unknown),
    })
}

/// The index of a 1-based key in a table of `key`'s kind; the weekday's is
/// counted from Sunday, which is how the tables are kept.
fn index_of(key: Key, number: i64) -> Answer<usize> {
    let number = usize::try_from(number).map_err(|_| Refusal::OutOfRange)?;
    if number == 0 || number > key.count() {
        return Err(Refusal::OutOfRange);
    }
    Ok(match key {
        Key::Weekday => number % 7,
        _ => number - 1,
    })
}

/// An optional validity bound.
fn year_cell(line: &mut Line<'_>, year: Option<i32>) {
    line.value_or_empty(year);
}

/// Every attribution list the crate ships, with what the crate declines to
/// ship, one line each: the kind (`authority` or `gap`), the subject
/// (`birthstone`, `birth-flower`, `moon-name`, `lunation-name`,
/// `month-name`, `zodiac-stone` or `weekday`), the list's identifier, its
/// English name, the body that issued it (empty where nobody did: the
/// Finnish month names), the identifier and the English name of the region
/// the list is in use in, when it was first adopted and when it was last
/// revised to whatever precision the source gives, the first and last year it
/// was current (empty where open), how it came to exist (`promulgated`,
/// `recorded`, `vernacular`, `contested` or `modern-invention`), what it is
/// keyed by (`month`, `lunation`, `sign` or `weekday`), the citation, what a
/// caller should know before repeating it (always present for a contested
/// list), and, for a gap, the reason (`sources-disagree-with-no-authority`,
/// `method-unpublished`, `different-key`, `modern-invention` or
/// `translation-undetermined`) and the reasoning.
///
/// A gap is a subject the crate declined to ship, such as Japan's day-by-day
/// 誕生花 or Robert Graves's "Celtic tree calendar"; its columns about a list
/// are empty, and its name is what is missing. `subject` is one of those
/// above, or empty for every list and every gap; a gap belongs to no subject
/// here and is written only for the empty one.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a subject that is not one of the seven.
pub fn authorities_lines(subject: &str) -> Answer<String> {
    let wanted = subject.trim();
    let mut out = String::new();
    for name in SUBJECTS {
        if !wanted.is_empty() && !wanted.eq_ignore_ascii_case(name) {
            continue;
        }
        // The sign-keyed list is the `seasons` feature's.
        let Ok((key, lists)) = listings(name) else {
            continue;
        };
        for list in lists {
            let authority = list.authority;
            let mut line = Line::new(&mut out);
            line.cell("authority")
                .cell(name)
                .cell(authority.id)
                .cell(authority.english_name)
                .cell_or_empty(authority.body)
                .cell(authority.region.id)
                .cell(authority.region.english_name)
                .value_or_empty(authority.established)
                .value_or_empty(authority.revised);
            year_cell(&mut line, authority.validity.from);
            year_cell(&mut line, authority.validity.to);
            line.cell(authority.provenance.id())
                .cell(key.id())
                .cell(authority.source)
                .cell_or_empty(authority.caveat)
                .empties(2);
            line.end();
        }
    }
    if wanted.is_empty() {
        for gap in &gaps::ALL {
            let mut line = Line::new(&mut out);
            line.cell("gap")
                .empty()
                .cell(gap.id)
                .cell(gap.subject)
                .empties(9)
                .cell(gap.sources)
                .empty()
                .cell(gap.reason.id())
                .cell(gap.explanation);
            line.end();
        }
    } else if !SUBJECTS
        .iter()
        .any(|name| wanted.eq_ignore_ascii_case(name))
    {
        return Err(Refusal::Unknown);
    }
    Ok(out)
}

/// The lines of one subject for one key, one list each.
fn push_subject(out: &mut String, subject: &str, number: i64) -> Answer<()> {
    let (key, lists) = listings(subject)?;
    let index = index_of(key, number)?;
    let agreed = lists
        .windows(2)
        .all(|pair| pair[0].entries.get(index) == pair[1].entries.get(index));
    for list in &lists {
        let Some(names) = list.entries.get(index) else {
            continue;
        };
        let authority = list.authority;
        let mut line = Line::new(out);
        line.cell(subject)
            .cell(authority.id)
            .cell(authority.english_name)
            .value(number)
            .cell(key.id())
            .cell(&names.join(";"))
            .value(names.len())
            .cell_or_empty(list.glosses.get(index).copied());
        year_cell(&mut line, authority.validity.from);
        year_cell(&mut line, authority.validity.to);
        line.cell(authority.provenance.id())
            .cell_or_empty(authority.caveat)
            .flag(agreed);
        line.end();
    }
    Ok(())
}

/// What every list of a subject attributes to one key, one line each: the
/// subject, the list's identifier and English name, the key and what it is
/// (`month`, `lunation`, `sign` or `weekday`), the attributions separated by
/// `;` in the list's own spelling, how many, the English gloss of a month
/// name (empty for other subjects), the first and last year the list was
/// current (empty where open), how it came to exist as
/// [`authorities_lines`] writes it, what a caller should know before
/// repeating it, and `1` where every list of the subject says the same.
///
/// There is no line for "the" birthstone: a question about March has six
/// answers, and the last cell says whether they agree. A contested list's
/// caveat is on its line, and a caller who shows the answer should show it.
///
/// `subject` is one of the seven [`authorities_lines`] names. The key is the
/// month from 1 to 12, the lunation from the March equinox from 1 to 12, the
/// sign from Aries = 1 to Pisces = 12, or the ISO weekday from Monday = 1 to
/// Sunday = 7. A leap month is no key: no tradition attributes anything to
/// an intercalary one.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a subject that is not one of the seven, and
/// [`Refusal::OutOfRange`] for a key outside its range.
pub fn attributions_lines(subject: &str, key: i64) -> Answer<String> {
    let name = SUBJECTS
        .iter()
        .find(|name| subject.trim().eq_ignore_ascii_case(name))
        .ok_or(Refusal::Unknown)?;
    let mut out = String::new();
    push_subject(&mut out, name, key)?;
    Ok(out)
}

/// What every list attributes to the month, the weekday and the sign of a
/// day, in the lines of [`attributions_lines`]: the birthstones, birth
/// flowers, full-moon names, month names, zodiac stones and weekday
/// attributions, each list on its own line, the subjects in that order, and
/// the lunation names left out because a day has no lunation number without
/// the March equinox of its year.
///
/// The month is the Gregorian month of the day, the weekday its ISO weekday,
/// and the sign the tropical sign the Sun is in at the day, judged at the
/// meridian `meridian` names (`universal`, `japan`, `china`, `korea`, `india`
/// or `china-before-1929`; empty for `universal`; or a longitude in degrees
/// east), which a day whose sign changes within about ten minutes of local
/// midnight can move by a day.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a meridian that is not read, and
/// [`Refusal::OutOfRange`] for a day outside the years −1000 to 3000 the sign
/// is computed for.
#[cfg(feature = "seasons")]
pub fn attributions_on_lines(fixed: i64, meridian: &str) -> Answer<String> {
    use hc_seasons::zodiac::tropical::sign_on_day;

    let meridian = crate::season_lines::meridian(meridian)?;
    let day = crate::astro_lines::day_in_era(fixed)?;
    let (_, month, _) = gregorian::ymd(day);
    let weekday = hc_calendar::Weekday::from_rd(Rd(fixed)).iso_number();
    let sign = i64::from(sign_on_day(day, meridian).index()) + 1;
    let mut out = String::new();
    for (subject, key) in [
        ("birthstone", i64::from(month)),
        ("birth-flower", i64::from(month)),
        ("moon-name", i64::from(month)),
        ("month-name", i64::from(month)),
        ("zodiac-stone", sign),
        ("weekday", i64::from(weekday)),
    ] {
        push_subject(&mut out, subject, key)?;
    }
    Ok(out)
}

/// The Harvest Moon of a year, as one line: the year, the fixed day of the
/// Harvest Moon, the fixed day of the Hunter's Moon after it, the Gregorian
/// month the Harvest Moon falls in (9 or 10), the name the Old Farmer's
/// Almanac gives September's full moon that year (`Harvest Moon`, or `Corn
/// Moon` when the Harvest Moon is October's), the meridian the days are
/// judged at, and the source.
///
/// The Harvest Moon is the full moon nearest the September equinox, a rule
/// and not a table row: it falls in September in about three years of four
/// and in October in the rest. The day is judged at the meridian `meridian`
/// names, as [`attributions_on_lines`] reads it, and a full moon within about
/// a minute of a day's end can move by a day.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a meridian that is not read, and
/// [`Refusal::OutOfRange`] for a year outside −999 to 3000.
#[cfg(feature = "seasons")]
pub fn harvest_moon_line(year: i64, meridian_name: &str) -> Answer<String> {
    use hc_attributes::moon_names::{
        harvest_moon, harvest_moon_falls_in, hunters_moon, september_moon_name,
    };

    let meridian = crate::season_lines::meridian(meridian_name)?;
    // The rule reads the equinox and the full moons around it, so the
    // era's first and last days are kept clear of both ends.
    let september = gregorian::to_fixed(year, 9, 1).map_err(|_| Refusal::OutOfRange)?;
    let (low, high) = (
        gregorian::to_fixed(-999, 1, 1).map_err(|_| Refusal::OutOfRange)?,
        gregorian::to_fixed(3000, 1, 1).map_err(|_| Refusal::OutOfRange)?,
    );
    if september < low || september >= high {
        return Err(Refusal::OutOfRange);
    }
    crate::astro_lines::day_in_era(september.0)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(year)
        .value(harvest_moon(year, meridian).0)
        .value(hunters_moon(year, meridian).0)
        .value(harvest_moon_falls_in(year, meridian))
        .cell(september_moon_name(year, meridian))
        .cell(meridian_name.trim())
        .cell(
            "hc-attributes moon_names::harvest_moon, hunters_moon and september_moon_name, \
             from hc-seasons' September equinox and lunar phases; the rule is the full moon \
             nearest the September equinox",
        );
    line.end();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rows(text: &str) -> Vec<Vec<&str>> {
        text.lines()
            .map(|line| line.split('\t').collect())
            .collect()
    }

    fn fixed(year: i64, month: u8, day: u8) -> i64 {
        gregorian::to_fixed(year, month, day)
            .map(|rd| rd.0)
            .unwrap_or_default()
    }

    #[test]
    fn every_list_names_its_authority_and_every_gap_its_reason() {
        let text = authorities_lines("").unwrap_or_default();
        let table = rows(&text);
        assert!(
            table.iter().all(|row| row.len() == AUTHORITY_COLUMNS),
            "{text}"
        );
        let authorities = table.iter().filter(|row| row[0] == "authority").count();
        assert_eq!(authorities, hc_attributes::authority_count());
        assert_eq!(
            table.iter().filter(|row| row[0] == "gap").count(),
            gaps::ALL.len()
        );
        // Japan's list of 1958 was revised on 20 December 2021: the 2021
        // list is the one that is current.
        let japan = table
            .iter()
            .find(|row| row[2] == "birthstones-jp-2021")
            .cloned()
            .unwrap_or_default();
        assert_eq!(japan[1], "birthstone");
        assert_eq!(japan[5], "jp");
        assert_eq!(japan[12], "month");
        assert_eq!(japan[9], "2021");
        assert_eq!(japan[10], "");
        // The full-moon names are contested, and the caveat travels.
        let contested: Vec<_> = table
            .iter()
            .filter(|row| row[1] == "moon-name" && row[11] == "contested")
            .collect();
        assert!(!contested.is_empty());
        assert!(contested.iter().all(|row| !row[14].is_empty()));
        // The birthstones alone are the eight lists.
        let stones = authorities_lines("BIRTHSTONE").unwrap_or_default();
        assert_eq!(stones.lines().count(), 8);
        assert!(
            stones
                .lines()
                .all(|line| line.starts_with("authority\tbirthstone\t"))
        );
        assert_eq!(authorities_lines("gemstone"), Err(Refusal::Unknown));
        // A gap carries its reason and no list columns.
        let gap = table
            .iter()
            .find(|row| row[0] == "gap" && row[2] == "celtic-tree-calendar")
            .cloned()
            .unwrap_or_default();
        assert_eq!(gap[15], "modern-invention");
        assert!(gap[4..13].iter().all(|cell| cell.is_empty()));
    }

    #[test]
    fn eight_lists_answer_for_a_month_and_say_they_disagree() {
        // January is garnet in every list: the one month the lists agree on
        // (hc-attributes: five centuries and three countries).
        let january = attributions_lines("birthstone", 1).unwrap_or_default();
        let table = rows(&january);
        assert_eq!(table.len(), 8);
        assert!(table.iter().all(|row| row.len() == ATTRIBUTION_COLUMNS));
        assert!(table.iter().all(|row| row[5] == "garnet" && row[12] == "1"));
        // December differs: the American 2016 list has turquoise, zircon and
        // tanzanite; the Japanese 2021 list adds lapis lazuli.
        let december = attributions_lines("birthstone", 12).unwrap_or_default();
        let table = rows(&december);
        assert!(table.iter().all(|row| row[12] == "0"));
        let us = table
            .iter()
            .find(|row| row[1] == "birthstones-us-2016")
            .cloned()
            .unwrap_or_default();
        assert_eq!(us[5], "turquoise;zircon;tanzanite");
        assert_eq!(us[6], "3");
        // The revisions between 1912 and 2016: zircon replaced lapis lazuli
        // in 1952 and tanzanite joined in 2002.
        let stones_of = |id: &str| {
            table
                .iter()
                .find(|row| row[1] == id)
                .map(|row| row[5].to_string())
                .unwrap_or_default()
        };
        assert_eq!(stones_of("birthstones-us-1912"), "turquoise;lapis lazuli");
        assert_eq!(stones_of("birthstones-us-1952"), "turquoise;zircon");
        assert_eq!(
            stones_of("birthstones-us-2002"),
            "turquoise;zircon;tanzanite"
        );
        let jp = table
            .iter()
            .find(|row| row[1] == "birthstones-jp-2021")
            .cloned()
            .unwrap_or_default();
        assert_eq!(jp[5], "turquoise;lapis lazuli;zircon;tanzanite");
        // A month name carries its gloss.
        let names = attributions_lines("month-name", 5).unwrap_or_default();
        let table = rows(&names);
        assert!(table.iter().all(|row| !row[7].is_empty()), "{table:?}");
        // The weekday key is ISO: Sunday is 7, and Kunz's stones for Sunday
        // are topaz and diamond.
        let sunday = attributions_lines("weekday", 7).unwrap_or_default();
        assert!(rows(&sunday).iter().any(|row| row[5] == "topaz;diamond"));
        let monday = attributions_lines("weekday", 1).unwrap_or_default();
        assert!(
            rows(&monday).iter().any(|row| row[5].contains("Moon")),
            "{monday}"
        );
        // The lunation names are Carver's, counted from the March equinox.
        let carver = attributions_lines("lunation-name", 1).unwrap_or_default();
        assert_eq!(carver.lines().count(), 1);
        assert_eq!(
            attributions_lines("birthstone", 0),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            attributions_lines("birthstone", 13),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(attributions_lines("weekday", 8), Err(Refusal::OutOfRange));
        assert_eq!(
            attributions_lines("birthstone", -1),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(attributions_lines("gemstone", 1), Err(Refusal::Unknown));
    }

    #[cfg(feature = "seasons")]
    #[test]
    fn a_day_has_its_month_its_weekday_and_its_sign() {
        // 2026-09-21 is a Monday, in September, and the Sun is in Virgo.
        let text = attributions_on_lines(fixed(2026, 9, 21), "universal").unwrap_or_default();
        let table = rows(&text);
        assert!(table.iter().all(|row| row.len() == ATTRIBUTION_COLUMNS));
        let subjects: Vec<&str> = table.iter().map(|row| row[0]).collect();
        assert_eq!(subjects.first(), Some(&"birthstone"));
        let stone = table
            .iter()
            .find(|row| row[1] == "birthstones-us-2016")
            .cloned()
            .unwrap_or_default();
        assert_eq!((stone[3], stone[5]), ("9", "sapphire"));
        let weekday = table
            .iter()
            .find(|row| row[0] == "weekday")
            .cloned()
            .unwrap_or_default();
        assert_eq!(weekday[3], "1");
        let sign = table
            .iter()
            .find(|row| row[0] == "zodiac-stone")
            .cloned()
            .unwrap_or_default();
        assert_eq!(sign[3], "6", "Virgo is the sixth sign");
        assert_eq!(
            attributions_on_lines(fixed(2026, 9, 21), "nowhere"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            attributions_on_lines(i64::MAX, ""),
            Err(Refusal::OutOfRange)
        );
    }

    #[cfg(feature = "seasons")]
    #[test]
    fn the_harvest_moon_is_the_full_moon_nearest_the_september_equinox() {
        // 2026: the September equinox is on the 23rd and the full moon of
        // 26 September is the nearest; 2025's is 7 October, so September's
        // moon is the Corn Moon that year.
        let line = harvest_moon_line(2025, "").unwrap_or_default();
        let row = line.trim_end().split('\t').collect::<Vec<_>>();
        assert_eq!(row.len(), HARVEST_MOON_COLUMNS);
        assert_eq!(row[3], "10");
        assert_eq!(row[4], "Corn Moon");
        assert_eq!(row[1], fixed(2025, 10, 7).to_string());
        let line = harvest_moon_line(2026, "universal").unwrap_or_default();
        let row = line.trim_end().split('\t').collect::<Vec<_>>();
        assert_eq!(row[3], "9");
        assert_eq!(row[4], "Harvest Moon");
        assert_eq!(row[1], fixed(2026, 9, 26).to_string());
        assert_eq!(harvest_moon_line(5_000, ""), Err(Refusal::OutOfRange));
        assert_eq!(harvest_moon_line(2026, "elsewhere"), Err(Refusal::Unknown));
    }
}
