//! A written date reads back as the day it was written for: every
//! registered calendar's dates, formatted by `hc_format::label::date` in
//! every carried locale, are read by `hc_format::label::parse_date` as
//! the day they were written for, or refused for one of the reasons
//! `REFUSALS` lists — the text names a cycle's place or a span, not a day
//! — or because the year is written in one or two digits.
//!
//! A release build reads every sample day in every locale. A debug build,
//! which the coverage job runs instrumented, reads every sample day in
//! the calendar's own language, and every other locale on one of the
//! days, staggered, so that every calendar is still read in every locale
//! (policy §7). A release build also reads the first day of every era, its
//! eve and the first day of every month over four years in every locale,
//! some 850 000 texts, which `scripts/release-shards.sh` runs in a shard of
//! its own; a debug build reads every era's first day in the calendar's
//! own language. Under coverage, calendars that share an era table are
//! read once, because the reader's paths are the same.
//! `docs/systems/written-dates.md` explains the reader and each refusal.

#![cfg(all(
    feature = "alloc",
    feature = "civil",
    feature = "lunar",
    feature = "equinox",
    feature = "indic",
    feature = "regional",
    feature = "i18n",
    feature = "format"
))]

use std::collections::{BTreeMap, BTreeSet};

use hyper_calendar::hc_calendar::{CalendarMeta, DateFields, DynCalendar, Rd, Weekday};
use hyper_calendar::hc_format::label::{self, DateRefusal};
use hyper_calendar::hc_i18n::Locale;
use hyper_calendar::hc_i18n::data::LOCALES;

/// The calendars whose written dates the reader refuses on some sample
/// day, with the refusal and why it is the right answer. A calendar
/// refusing in a way not listed here fails the sweep, and so, in a
/// release build, does a row the sweep no longer meets.
const REFUSALS: &[(&str, &str, &str)] = &[
    // The date is a place in cycles that recur, and names no year.
    (
        "akan",
        "year-not-written",
        "the six- and seven-day names recur every 42 days",
    ),
    (
        "aztec-tonalpohualli",
        "year-not-written",
        "the number and sign recur every 260 days",
    ),
    (
        "balinese-pawukon",
        "year-not-written",
        "the Pawukon's weeks recur every 210 days",
    ),
    (
        "javanese-pasaran",
        "year-not-written",
        "the weekday and pasaran recur every 35 days",
    ),
    (
        "maya-haab",
        "year-not-written",
        "the Haabʼ recurs every 365 days",
    ),
    (
        "maya-haab-gmt2",
        "year-not-written",
        "the Haabʼ recurs every 365 days",
    ),
    (
        "maya-haab-584286",
        "year-not-written",
        "the Haabʼ recurs every 365 days",
    ),
    (
        "maya-tzolkin",
        "year-not-written",
        "the Tzolkʼin recurs every 260 days",
    ),
    (
        "maya-tzolkin-gmt2",
        "year-not-written",
        "the Tzolkʼin recurs every 260 days",
    ),
    (
        "maya-tzolkin-584286",
        "year-not-written",
        "the Tzolkʼin recurs every 260 days",
    ),
    (
        "maya-round",
        "year-not-written",
        "the Calendar Round recurs every 52 years",
    ),
    (
        "maya-round-gmt2",
        "year-not-written",
        "the Calendar Round recurs every 52 years",
    ),
    (
        "maya-round-584286",
        "year-not-written",
        "the Calendar Round recurs every 52 years",
    ),
    (
        "mixtec-year",
        "year-not-written",
        "the year bearer recurs every 52 years",
    ),
    (
        "zapotec-yza",
        "year-not-written",
        "the year bearer recurs every 52 years",
    ),
    (
        "sexagenary",
        "year-not-written",
        "the day's stem and branch recur every 60 days",
    ),
    // The year by its stem and branch alone, as the Japanese template
    // writes it, and the Dangi date in Traditional Chinese and Cantonese
    // (CLDR 48's own `dangi` patterns, "U年MMMd日"): 癸卯年 recurs every
    // sixty years. Chinese and Korean otherwise write the related
    // Gregorian year before it, 2026丙午年, and read back.
    (
        "chinese",
        "year-not-written",
        "the sexagenary year recurs every 60 years",
    ),
    (
        "dangi",
        "year-not-written",
        "the sexagenary year recurs every 60 years",
    ),
    (
        "dangi-kasi",
        "year-not-written",
        "the sexagenary year recurs every 60 years",
    ),
    (
        "vietnamese",
        "year-not-written",
        "the sexagenary year recurs every 60 years",
    ),
    (
        "vietnamese-south-1968",
        "year-not-written",
        "the sexagenary year recurs every 60 years",
    ),
    (
        "chinese-taichu",
        "year-not-written",
        "the sexagenary year recurs every 60 years",
    ),
    (
        "chinese-sifen",
        "year-not-written",
        "the sexagenary year recurs every 60 years",
    ),
    (
        "chinese-qianxiang",
        "year-not-written",
        "the sexagenary year recurs every 60 years",
    ),
    (
        "chinese-jingchu",
        "year-not-written",
        "the sexagenary year recurs every 60 years",
    ),
    (
        "chinese-yuanjia",
        "year-not-written",
        "the sexagenary year recurs every 60 years",
    ),
    (
        "chinese-daming",
        "year-not-written",
        "the sexagenary year recurs every 60 years",
    ),
    (
        "chinese-xinghe",
        "year-not-written",
        "the sexagenary year recurs every 60 years",
    ),
    (
        "chinese-tianhe",
        "year-not-written",
        "the sexagenary year recurs every 60 years",
    ),
    (
        "chinese-kaihuang",
        "year-not-written",
        "the sexagenary year recurs every 60 years",
    ),
    (
        "chinese-sanji",
        "year-not-written",
        "the sexagenary year recurs every 60 years",
    ),
    (
        "chinese-zhengguang",
        "year-not-written",
        "the sexagenary year recurs every 60 years",
    ),
    // The station of the 819-day count is not written, and the count
    // alone recurs.
    ("maya-819", "missing-field", "the station is not written"),
    (
        "maya-819-gmt2",
        "missing-field",
        "the station is not written",
    ),
    (
        "maya-819-584286",
        "missing-field",
        "the station is not written",
    ),
    // The text names more than one day.
    ("stata-week", "ambiguous", "a %tw date names a week"),
    // A doubled day that no source read writes with a mark: the Gregorian
    // day under the Faṣlī year is this library's choice; Henning's
    // Tibetan and Bhutanese almanac data write both days alike, and Janson's
    // "Extra" is not shown in a date; the Hindu sources name a tithi that
    // spans two sunrises (adhika) and write it on both days; and
    // the Nepal Sambat almanac committee writes the tithi on both days and
    // tells them apart by the weekday, which the formatter does not write
    // (docs/systems/written-dates.md).
    (
        "fasli-bombay",
        "ambiguous",
        "the doubled day in early June is written as the ordinary one",
    ),
    (
        "sur-san",
        "ambiguous",
        "the doubled day in early June is written as the ordinary one",
    ),
    (
        "tibetan",
        "ambiguous",
        "a doubled lunar day is written as the ordinary one",
    ),
    (
        "tibetan-tsurphu",
        "ambiguous",
        "a doubled lunar day is written as the ordinary one",
    ),
    (
        "tibetan-bhutan",
        "ambiguous",
        "a doubled lunar day is written as the ordinary one",
    ),
    (
        "mongolian",
        "ambiguous",
        "a doubled lunar day is written as the ordinary one",
    ),
    (
        "tibetan-lochen",
        "ambiguous",
        "a doubled lunar day is written as the ordinary one",
    ),
    (
        "tibetan-bhutan-lochen",
        "ambiguous",
        "a doubled lunar day is written as the ordinary one",
    ),
    (
        "tibetan-tsurphu-karana",
        "ambiguous",
        "a doubled lunar day is written as the ordinary one",
    ),
    (
        "hindu-lunar",
        "ambiguous",
        "a tithi that spans two sunrises names both days",
    ),
    (
        "hindu-lunar-reingold-dershowitz",
        "ambiguous",
        "a tithi that spans two sunrises names both days",
    ),
    (
        "hindu-lunar-surya-siddhanta",
        "ambiguous",
        "a tithi that spans two sunrises names both days",
    ),
    (
        "hindu-lunar-purnimanta",
        "ambiguous",
        "a tithi that spans two sunrises names both days",
    ),
    (
        "vira-nirvana-samvat",
        "ambiguous",
        "a tithi that spans two sunrises names both days",
    ),
    (
        "vikram-samvat-kartikadi",
        "ambiguous",
        "a tithi that spans two sunrises names both days",
    ),
    (
        "rajyabhisheka-saka",
        "ambiguous",
        "a tithi that spans two sunrises names both days",
    ),
    (
        "saptarshi",
        "ambiguous",
        "a tithi that spans two sunrises names both days",
    ),
    (
        "gupta",
        "ambiguous",
        "a tithi that spans two sunrises names both days",
    ),
    (
        "valabhi",
        "ambiguous",
        "a tithi that spans two sunrises names both days",
    ),
    (
        "kalachuri",
        "ambiguous",
        "a tithi that spans two sunrises names both days",
    ),
    (
        "lakshmana-sena",
        "ambiguous",
        "a tithi that spans two sunrises names both days",
    ),
    (
        "odia-anka",
        "ambiguous",
        "a tithi that spans two sunrises names both days",
    ),
    (
        "nepal-sambat",
        "ambiguous",
        "a doubled tithi is written on both days, told apart by a weekday",
    ),
    (
        "nepal-sambat-fortnight",
        "ambiguous",
        "a doubled tithi is written on both days, told apart by a weekday",
    ),
];

/// Days a calendar's dates are also read on, with why: days whose text
/// the sample days would not meet.
const MORE_DAYS: &[(&str, &[i64], &str)] = &[
    // 1 Tishri of AM 1003, 5002, 5300 and 9001, whose years end in a
    // letter with its geresh, א׳ג׳ and ה׳ב׳, as the narrow weekdays are
    // written (docs/systems/hebrew-numerals.md).
    (
        "hebrew",
        &[-1_007_453, 453_158, 562_008, 1_913_771],
        "a year's numeral ends as a narrow weekday is written",
    ),
    // Two days of one number: 15 and 16 January 1990 in `tibetan` and
    // `tibetan-lochen`, 15 and 16 February 1990 in the others; the doubled
    // tithi of 15 and 16 February 1990 in the Hindu calendars and Nepal
    // Sambat.
    ("tibetan", &[726_482, 726_483], "a doubled lunar day"),
    (
        "tibetan-tsurphu",
        &[726_514, 726_515],
        "a doubled lunar day",
    ),
    ("tibetan-bhutan", &[726_514, 726_515], "a doubled lunar day"),
    ("mongolian", &[726_514, 726_515], "a doubled lunar day"),
    ("tibetan-lochen", &[726_482, 726_483], "a doubled lunar day"),
    (
        "tibetan-bhutan-lochen",
        &[726_514, 726_515],
        "a doubled lunar day",
    ),
    (
        "tibetan-tsurphu-karana",
        &[726_514, 726_515],
        "a doubled lunar day",
    ),
    ("hindu-lunar", &[726_513, 726_514], "a doubled tithi"),
    (
        "hindu-lunar-surya-siddhanta",
        &[726_513, 726_514],
        "a doubled tithi",
    ),
    (
        "hindu-lunar-purnimanta",
        &[726_513, 726_514],
        "a doubled tithi",
    ),
    (
        "vira-nirvana-samvat",
        &[726_513, 726_514],
        "a doubled tithi",
    ),
    (
        "vikram-samvat-kartikadi",
        &[726_513, 726_514],
        "a doubled tithi",
    ),
    ("rajyabhisheka-saka", &[726_513, 726_514], "a doubled tithi"),
    ("saptarshi", &[726_513, 726_514], "a doubled tithi"),
    ("gupta", &[726_513, 726_514], "a doubled tithi"),
    ("valabhi", &[726_513, 726_514], "a doubled tithi"),
    ("kalachuri", &[726_513, 726_514], "a doubled tithi"),
    ("lakshmana-sena", &[726_513, 726_514], "a doubled tithi"),
    ("odia-anka", &[726_513, 726_514], "a doubled tithi"),
    ("nepal-sambat", &[726_513, 726_514], "a doubled tithi"),
    (
        "nepal-sambat-fortnight",
        &[726_513, 726_514],
        "a doubled tithi",
    ),
    (
        "hindu-lunar-reingold-dershowitz",
        &[726_513, 726_514],
        "a doubled tithi",
    ),
    // 7 June 1993 and 7 June 1994, both 7 June of Faṣlī 1403 and of
    // Sūr-san 1394.
    ("fasli-bombay", &[727_721, 728_086], "a doubled day"),
    ("sur-san", &[727_721, 728_086], "a doubled day"),
];

/// The days a calendar's dates are read on: 1 January 1 CE, 15 October
/// 1582, the first day of the Gregorian reform, 15 June 1900, 1 January
/// 1970, 1 January and 28 September 2026 and 22 December 2100, where the
/// calendar converts them, else its own sample day; its first and last
/// days; and the days [`MORE_DAYS`] lists for it.
fn sample_days(meta: &CalendarMeta) -> Vec<Rd> {
    let mut days: Vec<Rd> = [1, 577_736, 693_761, 719_163, 739_617, 739_887, 767_000]
        .into_iter()
        .map(|day| meta.sample_day(Rd(day)))
        .collect();
    for (calendar, more, _) in MORE_DAYS {
        if *calendar == meta.id.0 {
            days.extend(more.iter().copied().map(Rd));
        }
    }
    days.extend(meta.earliest);
    days.extend(meta.latest);
    days.sort_unstable();
    days.dedup();
    days.retain(|day| meta.supports(*day));
    days
}

/// Whether the `day`th of `days` days of the `calendar`th calendar is
/// read in the `locale`th of `locales` locales, the last of which is the
/// calendar's own language: always in a release build; in a debug build
/// in the calendar's own language, and in every other locale on one day,
/// staggered over the pairings of calendar and locale. A build
/// instrumented for coverage reads each calendar in its own language and in
/// one locale of three, so that every locale is still read in by a third of
/// the calendars.
fn read_in(calendar: usize, day: usize, days: usize, locale: usize, locales: usize) -> bool {
    if !cfg!(debug_assertions) || locale + 1 == locales {
        return true;
    }
    if hyper_calendar::hc_core::sweep::INSTRUMENTED && !(calendar + locale).is_multiple_of(3) {
        return false;
    }
    (calendar + locale) % days == day
}

#[test]
fn every_calendar_reads_back_the_dates_it_writes() {
    let registry = hyper_calendar::registry();
    let locales: Vec<Option<Locale>> = LOCALES
        .iter()
        .filter_map(|data| data.tag.parse().ok())
        .map(Some)
        .chain([Some(Locale::ROOT), None])
        .collect();
    assert_eq!(
        locales.len(),
        LOCALES.len() + 2,
        "every locale's tag parses"
    );
    let expected: BTreeMap<(&str, &str), &str> = REFUSALS
        .iter()
        .map(|(calendar, refusal, why)| ((*calendar, *refusal), *why))
        .collect();
    let mut met: BTreeSet<(&str, &str)> = BTreeSet::new();
    let mut faults: BTreeSet<String> = BTreeSet::new();
    let mut read = 0_usize;
    let mut paired: BTreeSet<(&str, usize)> = BTreeSet::new();
    for (index, meta) in registry.metas().enumerate() {
        let calendar = registry.get(meta.id).expect("registered");
        hyper_calendar::hc_core::memo::scope(|| {
            let days = sample_days(&meta);
            for (position, day) in days.iter().enumerate() {
                let Ok(fields) = calendar.fixed_to_fields(*day) else {
                    continue;
                };
                for (at, requested) in locales.iter().enumerate() {
                    if !read_in(index, position, days.len(), at, locales.len()) {
                        continue;
                    }
                    paired.insert((meta.id.0, at));
                    read += 1;
                    match read_back(&meta, calendar, *day, &fields, requested, &expected) {
                        ReadBack::Day => {}
                        ReadBack::Listed(refusal) => {
                            met.insert((meta.id.0, refusal));
                        }
                        ReadBack::Wrong(fault) | ReadBack::Unlisted(_, fault) => {
                            faults.insert(fault);
                        }
                    }
                }
            }
        });
    }
    assert!(
        faults.is_empty(),
        "{} unread dates: {faults:#?}",
        faults.len()
    );
    assert!(read > 3_000, "{read}");
    // Every calendar is read in every locale, in either build; in a build
    // instrumented for coverage, in its own language and in a third of the
    // others, and every locale is read in by some calendar of each residue.
    if hyper_calendar::hc_core::sweep::INSTRUMENTED {
        let per_locale = |at: usize| paired.iter().filter(|(_, locale)| *locale == at).count();
        assert!((0..locales.len() - 1).all(|at| per_locale(at) >= registry.len() / 3 - 1));
        assert_eq!(per_locale(locales.len() - 1), registry.len());
    } else {
        assert_eq!(paired.len(), registry.len() * locales.len());
    }
    if !cfg!(debug_assertions) {
        let stale: Vec<_> = expected.keys().filter(|key| !met.contains(*key)).collect();
        assert!(
            stale.is_empty(),
            "refusals the sweep no longer meets: {stale:?}"
        );
    }
}

/// What reading back a written date gave.
enum ReadBack {
    /// The day it was written for, or a two-digit year refused.
    Day,
    /// A refusal [`REFUSALS`] lists for the calendar.
    Listed(&'static str),
    /// Another refusal, with the text and why.
    Unlisted(DateRefusal, String),
    /// Another day, with the text and the day.
    Wrong(String),
}

/// Write `fields`, the calendar's own for `day`, in the locale `requested`
/// resolves to, and read the text back.
fn read_back(
    meta: &CalendarMeta,
    calendar: &dyn DynCalendar,
    day: Rd,
    fields: &DateFields,
    requested: &Option<Locale>,
    expected: &BTreeMap<(&str, &str), &str>,
) -> ReadBack {
    let locale = label::locale_for(calendar, requested.as_ref());
    let text = label::date(calendar, fields, &locale);
    match label::parse_date(calendar, &locale, &text) {
        Ok(parsed) if parsed.fixed == day => ReadBack::Day,
        Ok(parsed) => ReadBack::Wrong(format!(
            "{} {locale} {text:?}: read as {} for {}",
            meta.id.0, parsed.fixed.0, day.0
        )),
        Err(DateRefusal::TwoDigitYear) if fields.year.abs() < 100 => ReadBack::Day,
        Err(refusal) if expected.contains_key(&(meta.id.0, refusal.name())) => {
            ReadBack::Listed(refusal.name())
        }
        Err(refusal) => ReadBack::Unlisted(
            refusal,
            format!("{} {locale} {} {text:?}: {refusal}", meta.id.0, day.0),
        ),
    }
}

/// Whether the era of `day` is known and not `era`.
fn era_differs(calendar: &dyn DynCalendar, day: i64, era: Option<&str>) -> bool {
    calendar
        .fixed_to_fields(Rd(day))
        .is_ok_and(|fields| fields.era != era)
}

/// How [`era_first_days`] looks for the days an era changes on.
#[derive(Clone, Copy, PartialEq, Eq)]
enum EraSearch {
    /// Whether the calendar has eras from 2 001 days spread over its
    /// range, then a week at a time over a range of up to 20 000 000 days,
    /// which no era of the registry is shorter than, and in 100 000 steps
    /// over a longer one, whose eras are the two either side of a year 1.
    Weekly,
    /// Whether it has eras from nine days, then in 5 000 steps over the
    /// range and never less than a year. A step that passes over a whole
    /// era still finds it, since an era's first day is the first day not
    /// in the era before: only an era that came back within one step would
    /// be missed, and the release build holds this search to the weekly
    /// one on every calendar.
    Coarse,
}

/// The first days of a calendar's eras: its first day, and every day
/// whose era is not the one of the day before. A calendar whose era is
/// the same on every day [`EraSearch`] looks at, and which names no era of
/// its own, has only its first. The others are stepped through, and each
/// step whose ends differ in era is halved down to the day.
fn era_first_days(calendar: &dyn DynCalendar, meta: &CalendarMeta, search: EraSearch) -> Vec<Rd> {
    let first = meta.earliest.map_or(-1_000_000, |day| day.0);
    let last = meta.latest.map_or(1_000_000, |day| day.0);
    let mut days = vec![Rd(first)];
    let span = last - first;
    let era_of = |day: i64| calendar.fixed_to_fields(Rd(day)).ok().map(|f| f.era);
    let probes = match search {
        EraSearch::Weekly => 2_000,
        EraSearch::Coarse => 8,
    };
    let seen: BTreeSet<_> = (0..=probes)
        .filter_map(|step| era_of(first + span * step / probes))
        .collect();
    if seen.len() < 2 && calendar.era_code(1).is_none() {
        return days;
    }
    let step = match search {
        EraSearch::Weekly if span <= 20_000_000 => 7,
        EraSearch::Weekly => span / 100_000,
        EraSearch::Coarse => (span / 5_000).max(365),
    };
    let mut at = first;
    let mut era = era_of(at).flatten();
    while at < last {
        let next = (at + step).min(last);
        if era_differs(calendar, next, era) {
            let (mut low, mut high) = (at, next);
            while high - low > 1 {
                let middle = low + (high - low) / 2;
                if era_differs(calendar, middle, era) {
                    high = middle;
                } else {
                    low = middle;
                }
            }
            days.push(Rd(high));
            era = era_of(high).flatten();
            at = high;
        } else {
            at = next;
        }
    }
    days
}

/// The first day of every month over four years from 1 January 2026, or
/// from the calendar's sample day where it does not reach 2026: each day
/// with a month whose year or month is not the day before's. A day count
/// has no months, and its first day is its era's.
fn month_first_days(calendar: &dyn DynCalendar, meta: &CalendarMeta) -> Vec<Rd> {
    let start = meta.sample_day(Rd(739_617)).0;
    let key = |day: i64| {
        calendar
            .fixed_to_fields(Rd(day))
            .ok()
            .map(|fields| (fields.era, fields.year, fields.month))
    };
    let mut before = key(start - 1);
    let mut days = Vec::new();
    for day in start..start + 1_461 {
        if !meta.supports(Rd(day)) {
            continue;
        }
        let now = key(day);
        if now.is_some_and(|(_, _, month)| month.is_some()) && now != before {
            days.push(Rd(day));
        }
        before = now;
    }
    days
}

/// Whether two days are the same day of a month and of its leap
/// repetition, which a locale with no word for the leap month writes
/// alike.
fn month_and_its_leap(calendar: &dyn DynCalendar, first: Rd, second: Rd) -> bool {
    let (Ok(first), Ok(second)) = (
        calendar.fixed_to_fields(first),
        calendar.fixed_to_fields(second),
    ) else {
        return false;
    };
    match (first.month, second.month) {
        (Some(one), Some(other)) => {
            one.ordinal == other.ordinal
                && one.leap != other.leap
                && (first.era, first.year, first.day) == (second.era, second.year, second.day)
        }
        _ => false,
    }
}

/// The era table a calendar answers for itself, `DynCalendar::era_code`:
/// the 248 nengō of the Japanese calendars, the reigns of the Chinese and
/// Korean regnal ones. Empty for a calendar whose eras are the locale
/// data's alone.
fn era_table(calendar: &dyn DynCalendar) -> Vec<&'static str> {
    (0..).map_while(|index| calendar.era_code(index)).collect()
}

/// The first day of every era of every calendar and, in a release build,
/// the day before it, and the first day of every month over four years ([`era_first_days`],
/// [`month_first_days`]), written and read back as the day, or refused as
/// the sweep allows, or, in a Japanese calendar, a leap month that a
/// locale with no word for it writes as the ordinary month, and so reads
/// as two days (docs/systems/written-dates.md).
///
/// A release build writes every one of those days in every locale
/// setting, finds the eras [`EraSearch::Weekly`], and holds
/// [`EraSearch::Coarse`] to the same days. A debug build, which the
/// coverage job runs instrumented, writes the eras' first days, found
/// [`EraSearch::Coarse`], in the calendar's own language alone, with no
/// eves and no month days (policy §7). Under coverage, calendars that
/// share an era table ([`era_table`]) are read once, the first registered
/// of them, because the reader's paths are the same: the seven Japanese
/// calendars' 1 400 or so era days in Japanese would be most of its time. The calendars are spread over the
/// machine's threads, and kept on one in an instrumented build
/// (`hc_core::sweep::INSTRUMENTED`).
#[test]
fn the_first_day_of_every_era_and_month_reads_back() {
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicUsize, Ordering};
    let full = !cfg!(debug_assertions);
    let expected: BTreeMap<(&str, &str), &str> = REFUSALS
        .iter()
        .map(|(calendar, refusal, why)| ((*calendar, *refusal), *why))
        .collect();
    let registry = hyper_calendar::registry();
    let calendars = registry.len();
    // Under coverage, the calendars whose era table an earlier one shares.
    let mut tables: BTreeSet<Vec<&str>> = BTreeSet::new();
    let skipped: BTreeSet<&str> = registry
        .metas()
        .filter(|meta| {
            let table = era_table(registry.get(meta.id).expect("registered"));
            hyper_calendar::hc_core::sweep::INSTRUMENTED
                && !table.is_empty()
                && !tables.insert(table)
        })
        .map(|meta| meta.id.0)
        .collect();
    let next = AtomicUsize::new(0);
    // Era days, month days, texts, leap months read as two days, faults.
    let tally = Mutex::new((
        0_usize,
        0_usize,
        0_usize,
        0_usize,
        BTreeSet::<String>::new(),
    ));
    let threads = if hyper_calendar::hc_core::sweep::INSTRUMENTED {
        1
    } else {
        std::thread::available_parallelism().map_or(1, usize::from)
    };
    std::thread::scope(|scope| {
        for _ in 0..threads.min(calendars) {
            scope.spawn(|| {
                let registry = hyper_calendar::registry();
                let metas: Vec<CalendarMeta> = registry.metas().collect();
                let locales: Vec<Option<Locale>> = if full {
                    LOCALES
                        .iter()
                        .filter_map(|data| data.tag.parse().ok())
                        .map(Some)
                        .chain([Some(Locale::ROOT), None])
                        .collect()
                } else {
                    vec![None]
                };
                loop {
                    let index = next.fetch_add(1, Ordering::Relaxed);
                    let Some(meta) = metas.get(index) else {
                        break;
                    };
                    if skipped.contains(meta.id.0) {
                        continue;
                    }
                    let calendar = registry.get(meta.id).expect("registered");
                    let (mut texts, mut leap, mut faults) = (0, 0, BTreeSet::new());
                    let (eras, months) = hyper_calendar::hc_core::memo::scope(|| {
                        let coarse = era_first_days(calendar, meta, EraSearch::Coarse);
                        if !full {
                            return (coarse, Vec::new());
                        }
                        let weekly = era_first_days(calendar, meta, EraSearch::Weekly);
                        if weekly != coarse {
                            faults.insert(format!(
                                "{}: the coarse search finds {} era days, the weekly {}",
                                meta.id.0,
                                coarse.len(),
                                weekly.len()
                            ));
                        }
                        (weekly, month_first_days(calendar, meta))
                    });
                    let eves = eras
                        .iter()
                        .skip(1)
                        .map(|day| Rd(day.0 - 1))
                        .filter(|day| full && meta.supports(*day));
                    let mut days: Vec<Rd> =
                        eras.iter().chain(&months).copied().chain(eves).collect();
                    days.sort_unstable();
                    days.dedup();
                    for day in days {
                        hyper_calendar::hc_core::memo::scope(|| {
                            let Ok(fields) = calendar.fixed_to_fields(day) else {
                                return;
                            };
                            for requested in &locales {
                                texts += 1;
                                match read_back(meta, calendar, day, &fields, requested, &expected)
                                {
                                    ReadBack::Day | ReadBack::Listed(_) => {}
                                    ReadBack::Unlisted(
                                        DateRefusal::Ambiguous { first, second },
                                        _,
                                    ) if meta.id.0.starts_with("japanese")
                                        && month_and_its_leap(calendar, first, second) =>
                                    {
                                        leap += 1;
                                    }
                                    ReadBack::Unlisted(_, fault) | ReadBack::Wrong(fault) => {
                                        faults.insert(fault);
                                    }
                                }
                            }
                        });
                    }
                    let mut tally = tally.lock().expect("no thread panicked");
                    tally.0 += eras.len();
                    tally.1 += months.len();
                    tally.2 += texts;
                    tally.3 += leap;
                    tally.4.extend(faults);
                }
            });
        }
    });
    let (eras, months, texts, leap, faults) = tally.into_inner().expect("no thread panicked");
    println!(
        "{eras} era days, {months} month days, {texts} texts, {leap} leap months read as two days"
    );
    assert!(
        faults.is_empty(),
        "{} unread dates: {faults:#?}",
        faults.len()
    );
    if full {
        assert!(texts > 400_000, "{texts}");
    } else if skipped.is_empty() {
        assert!(eras > 1_000, "{eras}");
    } else {
        assert!(eras > 300, "{eras}");
    }
}

fn read(calendar: &str, tag: &str, text: &str) -> Result<(Rd, DateFields), DateRefusal> {
    let registry = hyper_calendar::registry();
    let calendar = registry
        .get_by_name(calendar)
        .unwrap_or_else(|| panic!("{calendar} is registered"));
    let locale: Locale = tag.parse().unwrap_or_else(|_| panic!("{tag} parses"));
    label::parse_date(calendar, &locale, text).map(|parsed| (parsed.fixed, parsed.fields))
}

fn day_of(calendar: &str, fields: DateFields) -> Rd {
    let registry = hyper_calendar::registry();
    registry
        .get_by_name(calendar)
        .and_then(|calendar| calendar.fields_to_fixed(&fields).ok())
        .unwrap_or_else(|| panic!("{calendar} has {fields:?}"))
}

/// 28 September 2026, a Monday.
const TODAY: Rd = Rd(739_887);

/// The dates of the reader's system document, as the formatter writes
/// them and as a reader writes them: eras by their names, native digits,
/// Han numerals and 元年, the Chinese day names, abbreviations and
/// weekdays.
#[test]
fn written_dates_read_as_their_days() {
    for (calendar, tag, text) in [
        ("japanese", "ja", "令和8年9月28日"),
        ("japanese", "ja", "令和八年九月二十八日"),
        ("japanese", "en", "September 28, 8 Reiwa"),
        ("japanese", "en", "Sep 28, 8 Reiwa"),
        ("roc", "zh-Hant", "民國115年9月28日"),
        ("gregory", "en", "September 28, 2026"),
        ("gregory", "en", "Monday, September 28, 2026"),
        ("gregory", "en", "september  28 2026"),
        ("gregory", "en", "Sep. 28, 2026"),
        ("gregory", "tr", "28 Eylül 2026"),
        ("gregory", "tr", "28 Eyl 2026"),
        ("gregory", "ar", "٢٨ سبتمبر ٢٠٢٦"),
        ("gregory", "ar", "28 سبتمبر 2026"),
        ("gregory", "mr", "२८ सप्टेंबर, २०२६"),
        ("gregory", "ru", "28 сентября 2026 г."),
        ("gregory", "de", "Montag, 28. September 2026"),
        ("gregory", "ja", "2026年9月28日月曜日"),
        ("gregory", "zh-Hans", "二〇二六年九月二十八日"),
        ("buddhist", "th", "28 กันยายน พ.ศ. 2569"),
        ("persian", "fa", "۶ مهر ۱۴۰۵ ه.ش."),
    ] {
        assert_eq!(
            read(calendar, tag, text).map(|(day, _)| day),
            Ok(TODAY),
            "{calendar} {tag} {text:?}"
        );
    }
    // 令和元年, the first year of Reiwa, 1 May 2019; 嘉永三年, an era
    // before Meiji, by the name CLDR's `ja.xml` gives it.
    let reiwa = day_of("japanese", DateFields::ymd(1, 5, 1).with_era("reiwa"));
    for text in ["令和元年5月1日", "令和1年5月1日", "令和一年五月一日"] {
        assert_eq!(
            read("japanese", "ja", text).map(|(day, _)| day),
            Ok(reiwa),
            "{text}"
        );
    }
    // The first years of 平成 and 明治 written with 1 as well: 8 January
    // 1989 and 23 October 1868, the day 明治 was proclaimed.
    for (text, day) in [
        (
            "平成1年1月8日",
            day_of("gregory", DateFields::ymd(1989, 1, 8)),
        ),
        (
            "明治1年9月8日",
            day_of("gregory", DateFields::ymd(1868, 10, 23)),
        ),
    ] {
        assert_eq!(
            read("japanese", "ja", text).map(|(day, _)| day),
            Ok(day),
            "{text}"
        );
    }
    let kaei = day_of("japanese", DateFields::ymd(3, 1, 1).with_era("kaei"));
    assert_eq!(
        read("japanese", "ja", "嘉永3年1月1日").map(|(day, _)| day),
        Ok(kaei)
    );
    assert_eq!(
        read("japanese", "ja", "嘉永三年一月一日").map(|(day, _)| day),
        Ok(kaei)
    );
    // 明治元年9月8日, the day 明治 was proclaimed, 23 October 1868
    // (docs/systems/japanese-eras.md).
    let meiji = day_of("gregory", DateFields::ymd(1868, 10, 23));
    assert_eq!(
        read("japanese", "ja", "明治元年9月8日").map(|(day, _)| day),
        Ok(meiji)
    );
    // 万延元年3月3日, 24 March 1860, that document's worked example. 1860
    // has a leap third month, whose third day, 23 April, the locales that
    // carry the calendar write with their word for it: 閏 as Wikipedia (ja)
    // dates the reunion of the courts, 元中9年閏10月5日, "intercalary" as
    // the National Diet Library's "Calendar History" calls the month, and
    // in Korean and Vietnamese the
    // word their CLDR files give the Chinese calendar's, 윤{0} and {0}
    // Nhuận. The era is each file's name for it, with the years where the
    // file writes them, and the calendar's own where the file has none, as
    // in Vietnamese. German, which carries no word for the month, writes
    // both days alike.
    let man_en = day_of("gregory", DateFields::ymd(1860, 3, 24));
    let leap = Rd(man_en.0 + 30);
    assert_eq!(
        read("japanese", "ja", "万延元年3月3日").map(|(day, _)| day),
        Ok(man_en)
    );
    let registry = hyper_calendar::registry();
    let japanese = registry.get_by_name("japanese").expect("registered");
    let fields = japanese.fixed_to_fields(leap).expect("in range");
    assert_eq!(fields.month.map(|month| month.leap), Some(true));
    for (tag, text) in [
        ("ja", "万延元年閏3月3日"),
        ("en", "intercalary March 3, 1 Man’en (1860–1861)"),
        ("zh-Hant", "萬延1年閏3月3日"),
        ("zh-Hans", "万延 (1860–1861)1年闰3月3日"),
        ("yue-Hant", "萬延1年閏3月3日"),
        ("yue-Hans", "万延1年闰3月3日"),
        ("ko", "만엔 (1860 ~ 1861) 1년 윤3월 3일"),
        ("vi", "3 tháng 3 Nhuận, 1 Man'en"),
    ] {
        let locale: Locale = tag.parse().expect("a tag");
        assert_eq!(label::date(japanese, &fields, &locale), text, "{tag}");
        assert_eq!(
            read("japanese", tag, text).map(|(day, _)| day),
            Ok(leap),
            "{tag}"
        );
    }
    // The era names CLDR states with their years read without them too,
    // by the calendar's own name: 万延1年闰3月3日.
    assert_eq!(
        read("japanese", "zh-Hans", "万延1年闰3月3日").map(|(day, _)| day),
        Ok(leap)
    );
    assert_eq!(
        read("japanese", "de", "3. März 1 Man'en"),
        Err(DateRefusal::Ambiguous {
            first: man_en,
            second: leap
        })
    );
    // 光武元年8月14日, 14 August 1897 (docs/systems/east-asian-eras.md),
    // as Korean writes the era: 광무 1년 8월 14일.
    let gwangmu_1 = day_of("gregory", DateFields::ymd(1897, 8, 14));
    assert_eq!(
        read("korean-regnal", "ko", "광무 1년 8월 14일").map(|(day, _)| day),
        Ok(gwangmu_1)
    );
    // 康熙五十二年十一月初一 and 二十, as a Qing source writes them, and
    // as the formatter writes them, 康熙52年十一月1日.
    let kangxi = day_of(
        "chinese-regnal",
        DateFields::ymd(52, 11, 1).with_era("kangxi"),
    );
    for text in ["康熙五十二年十一月初一", "康熙52年十一月1日"] {
        assert_eq!(
            read("chinese-regnal", "zh-Hant", text).map(|(day, _)| day),
            Ok(kangxi)
        );
    }
    assert_eq!(
        read("chinese-regnal", "zh-Hant", "康熙五十二年十一月二十").map(|(day, _)| day),
        Ok(Rd(kangxi.0 + 19))
    );
    // The formatter writes the reign's year and day in Han numerals, as GB/T
    // 15835-2011 writes 清咸丰十年九月二十日, and the first year as 元年.
    let regnal = registry.get_by_name("chinese-regnal").expect("registered");
    let first = day_of(
        "chinese-regnal",
        DateFields::ymd(1, 1, 1).with_era("kangxi"),
    );
    for (tag, day, text) in [
        ("zh-Hans", kangxi, "康熙五十二年十一月一日"),
        ("zh-Hant", kangxi, "康熙五十二年十一月一日"),
        ("zh-Hans", Rd(kangxi.0 + 19), "康熙五十二年十一月二十日"),
        ("zh-Hant", first, "康熙元年正月一日"),
    ] {
        let locale: Locale = tag.parse().expect("a tag");
        let fields = regnal.fixed_to_fields(day).expect("in range");
        assert_eq!(label::date(regnal, &fields, &locale), text, "{tag}");
        assert_eq!(
            read("chinese-regnal", tag, text).map(|(day, _)| day),
            Ok(day),
            "{tag}"
        );
    }
    // 광무 5년, the Korean Empire's second era.
    let gwangmu = day_of(
        "korean-regnal",
        DateFields::ymd(5, 9, 28).with_era("gwangmu"),
    );
    assert_eq!(
        read("korean-regnal", "ko", "광무 5년 9월 28일").map(|(day, _)| day),
        Ok(gwangmu)
    );
    // The Chinese calendar in English, whose year is the Gregorian year it
    // starts in: 18 Eighth Month of 丙午.
    assert_eq!(
        read("chinese", "en", "Eighth Month 18, 2026(bing-wu)").map(|(day, _)| day),
        Ok(TODAY)
    );
    // Adar II names the sixth month in a leap year only.
    let adar_ii = day_of("hebrew", DateFields::ymd(5784, 6, 1));
    assert_eq!(
        read("hebrew", "en", "1 Adar II 5784").map(|(day, _)| day),
        Ok(adar_ii)
    );
    assert!(read("hebrew", "en", "1 Adar II 5785").is_err());
    // The Minguo date as CLDR 48 writes it in Russian, by `ru.xml`'s
    // `generic` patterns, and in Thai, by `th.xml`'s `roc` ones; the
    // Japanese date in Russian by the same `generic` pattern.
    let japanese = registry.get_by_name("japanese").expect("registered");
    let fields = japanese.fixed_to_fields(TODAY).expect("in range");
    let ru: Locale = "ru".parse().expect("a tag");
    assert_eq!(
        label::date(japanese, &fields, &ru),
        "28 сентября 8 г. Рэйва"
    );
    assert_eq!(
        read("japanese", "ru", "28 сентября 8 г. Рэйва").map(|(day, _)| day),
        Ok(TODAY)
    );
    // Hindi writes the Japanese date era first, by `hi.xml`'s `generic`
    // long date "G d MMMM y": रेइवा 28 सितंबर 8, and the first day of 令和,
    // 1 May 2019, रेइवा 1 मई 1.
    let hi: Locale = "hi".parse().expect("a tag");
    let reiwa = day_of("gregory", DateFields::ymd(2019, 5, 1));
    for (day, text) in [(TODAY, "रेइवा 28 सितंबर 8"), (reiwa, "रेइवा 1 मई 1")]
    {
        let fields = japanese.fixed_to_fields(day).expect("in range");
        assert_eq!(label::date(japanese, &fields, &hi), text);
        assert_eq!(
            read("japanese", "hi", text).map(|(day, _)| day),
            Ok(day),
            "{text}"
        );
    }
    // The era after the year is not the locale's pattern.
    assert!(matches!(
        read("japanese", "hi", "28 सितंबर 8 रेइवा"),
        Err(DateRefusal::NotRecognised { .. })
    ));
    let roc = registry.get_by_name("roc").expect("registered");
    let fields = roc.fixed_to_fields(TODAY).expect("in range");
    for (tag, text) in [
        ("ru", "28 сентября 115 г. Minguo"),
        ("th", "28 กันยายน ปีไต้หวัน 115"),
    ] {
        let locale: Locale = tag.parse().expect("a tag");
        assert_eq!(label::date(roc, &fields, &locale), text, "{tag}");
        assert_eq!(
            read("roc", tag, text).map(|(day, _)| day),
            Ok(TODAY),
            "{tag}"
        );
    }
    // The fields are the calendar's own for the day.
    let (_, fields) = read("japanese", "ja", "令和8年9月28日").expect("a day");
    assert_eq!((fields.era, fields.year), (Some("reiwa"), 8));
}

/// The Hebrew calendar in Hebrew, in Hebrew numerals as CLDR 48 `he.xml`
/// writes it (docs/systems/hebrew-numerals.md): with the thousands, as the
/// formatter writes it; without them, as Wikipedia's "Hebrew numerals" says
/// writers usually do; with the marks typed; and in digits.
#[test]
fn hebrew_dates_read_in_hebrew_numerals() {
    let registry = hyper_calendar::registry();
    let he: Locale = "he".parse().expect("a tag");
    for id in ["hebrew", "hebrew-observational"] {
        let calendar = registry.get_by_name(id).expect("registered");
        let fields = calendar.fixed_to_fields(TODAY).expect("in range");
        let text = label::date(calendar, &fields, &he);
        assert!(text.ends_with("בתשרי ה׳תשפ״ז"), "{id} {text}");
        assert_eq!(read(id, "he", &text).map(|(day, _)| day), Ok(TODAY), "{id}");
    }
    assert_eq!(
        label::date(
            registry.get_by_name("hebrew").expect("registered"),
            &DateFields::ymd(5787, 1, 17).with_era("am"),
            &he
        ),
        "י״ז בתשרי ה׳תשפ״ז"
    );
    for text in [
        "י״ז בתשרי ה׳תשפ״ז",
        "י״ז בתשרי תשפ״ז",
        "י\"ז בתשרי תשפ\"ז",
        "י\"ז בתשרי ה'תשפ\"ז",
        "17 בתשרי 5787",
        "יום שני, י״ז בתשרי תשפ״ז",
    ] {
        assert_eq!(
            read("hebrew", "he", text).map(|(day, _)| day),
            Ok(TODAY),
            "{text}"
        );
    }
    // Wikipedia's examples, in full and in common usage: Monday, 15 Adar
    // 5764, 8 March 2004, and Thursday, 3 Nisan 5767, 22 March 2007.
    let adar = day_of("gregory", DateFields::ymd(2004, 3, 8));
    let nisan = day_of("gregory", DateFields::ymd(2007, 3, 22));
    for (text, day) in [
        ("יום שני ט״ו באדר ה׳תשס״ד", adar),
        ("יום שני ט״ו באדר תשס״ד", adar),
        ("יום חמישי ג׳ בניסן ה׳תשס״ז", nisan),
        ("יום חמישי ג׳ בניסן תשס״ז", nisan),
    ] {
        assert_eq!(
            read("hebrew", "he", text).map(|(day, _)| day),
            Ok(day),
            "{text}"
        );
    }
    assert_eq!(
        read("hebrew", "he", "יום שלישי ט״ו באדר תשס״ד"),
        Err(DateRefusal::WeekdayMismatch {
            written: Weekday::Tuesday,
            actual: Weekday::Monday
        })
    );
    // 16 as ט״ז; the older י״ו, and a day without its year, are refused.
    let sixteenth = Rd(TODAY.0 - 1);
    assert_eq!(
        read("hebrew", "he", "ט״ז בתשרי תשפ״ז").map(|(day, _)| day),
        Ok(sixteenth)
    );
    assert!(matches!(
        read("hebrew", "he", "י״ו בתשרי תשפ״ז"),
        Err(DateRefusal::NotRecognised { .. })
    ));
    assert_eq!(
        read("hebrew", "he", "י״ז בתשרי"),
        Err(DateRefusal::YearNotWritten)
    );
    // A year whose numerals would read as one of the sixth millennium is
    // written in digits: AM 1, not א׳, which reads as AM 5001. The years
    // 1 to 99 so written are then refused as two-digit years, and the
    // years from 100 read back.
    let hebrew = registry.get_by_name("hebrew").expect("registered");
    let first = hebrew.meta().earliest.expect("bounded");
    let fields = hebrew.fixed_to_fields(first).expect("in range");
    assert_eq!(label::date(hebrew, &fields, &he), "א׳ בתשרי 1");
    assert_eq!(
        read("hebrew", "he", "א׳ בתשרי 1"),
        Err(DateRefusal::TwoDigitYear)
    );
    let hundred = day_of("hebrew", DateFields::ymd(100, 1, 1).with_era("am"));
    let fields = hebrew.fixed_to_fields(hundred).expect("in range");
    assert_eq!(label::date(hebrew, &fields, &he), "א׳ בתשרי 100");
    assert_eq!(
        read("hebrew", "he", "א׳ בתשרי 100").map(|(day, _)| day),
        Ok(hundred)
    );
    let later = day_of("hebrew", DateFields::ymd(5001, 1, 1).with_era("am"));
    assert_eq!(
        read("hebrew", "he", "א׳ בתשרי א׳").map(|(day, _)| day),
        Ok(later)
    );
    // A run of letters each with its geresh is one numeral: ה׳ב׳ is 5002,
    // and not 5005 and the narrow weekday ב׳, Monday. A weekday after the
    // year needs a space.
    let year_5002 = day_of("hebrew", DateFields::ymd(5002, 1, 1).with_era("am"));
    let fields = hebrew.fixed_to_fields(year_5002).expect("in range");
    assert_eq!(label::date(hebrew, &fields, &he), "א׳ בתשרי ה׳ב׳");
    assert_eq!(
        read("hebrew", "he", "א׳ בתשרי ה׳ב׳").map(|(day, _)| day),
        Ok(year_5002)
    );
    // Nor does a name end inside one: אדר א׳י״א is Adar of 1011, and not
    // Adar I of 5011.
    let adar_1011 = day_of("hebrew", DateFields::ymd(1011, 6, 1).with_era("am"));
    let fields = hebrew.fixed_to_fields(adar_1011).expect("in range");
    assert_eq!(label::date(hebrew, &fields, &he), "א׳ באדר א׳י״א");
    assert_eq!(
        read("hebrew", "he", "א׳ באדר א׳י״א").map(|(day, _)| day),
        Ok(adar_1011)
    );
    let year_5005 = day_of("hebrew", DateFields::ymd(5005, 1, 1).with_era("am"));
    assert_eq!(
        read("hebrew", "he", "א׳ בתשרי ה׳ ב׳").map(|(day, _)| day),
        Ok(year_5005)
    );
    assert!(matches!(
        read("hebrew", "he", "י״ז בתשרי תשפ״זב׳"),
        Err(DateRefusal::NotRecognised { .. })
    ));
    assert_eq!(
        read("hebrew", "he", "י״ז בתשרי תשפ״ז ב׳").map(|(day, _)| day),
        Ok(TODAY)
    );
}

/// A leap unit that repeats another is written by its own name where a
/// source names it, and the text then reads as the one day: St. Tib's Day,
/// Nepal Sambat's Analā and the Lao later eighth month. A doubled day that
/// no source marks is refused as naming two, and read as one where the
/// text names the weekday, as the Nepal Sambat almanac committee tells a
/// doubled tithi's days apart.
#[test]
fn a_repeated_unit_is_named_where_a_source_names_it() {
    let registry = hyper_calendar::registry();
    let text = |id: &str, day: i64, tag: &str| {
        let calendar = registry.get_by_name(id).expect("registered");
        let fields = calendar.fixed_to_fields(Rd(day)).expect("in range");
        let locale: Locale = tag.parse().expect("a tag");
        label::date(calendar, &fields, &locale)
    };
    // 28 and 29 February 2024: Chaos 59 and St. Tib's Day, 3190 YOLD, "inserted
    // between the 59th and 60th days of the Season of Chaos" (the
    // Principia Discordia, p. 34).
    for (day, en, de) in [
        (738_944, "Chaos 59, 3190 YOLD", "3190 YOLD Chaos 59"),
        (
            738_945,
            "St. Tib's Day, 3190 YOLD",
            "3190 YOLD St. Tib's Day",
        ),
    ] {
        assert_eq!(text("discordian", day, "en"), en);
        assert_eq!(text("discordian", day, "de"), de);
        assert_eq!(read("discordian", "en", en).map(|(d, _)| d), Ok(Rd(day)));
        assert_eq!(read("discordian", "de", de).map(|(d, _)| d), Ok(Rd(day)));
    }
    assert!(matches!(
        read("discordian", "en", "St. Tib's Day, 3191 YOLD"),
        Err(DateRefusal::NoSuchDate(_))
    ));
    // 1 of the intercalary Bachhalā of 1111 NS, 16 April 1991, and of the
    // ordinary one after it: Analā, अनला in Wikipedia's table of months;
    // the era नेसं as Nepali Wikipedia abbreviates it.
    for (day, en, ne) in [
        (726_937, "Analā 1, 1111 NS", "११११ नेसं अनला १"),
        (726_967, "Bachhalā 1, 1111 NS", "११११ नेसं बछला १"),
    ] {
        assert_eq!(text("nepal-sambat", day, "en"), en);
        assert_eq!(text("nepal-sambat", day, "ne"), ne);
        assert_eq!(read("nepal-sambat", "en", en).map(|(d, _)| d), Ok(Rd(day)));
        assert_eq!(read("nepal-sambat", "ne", ne).map(|(d, _)| d), Ok(Rd(day)));
    }
    // 30 June and 29 July 2026, the first waning day of the extra month 8
    // of 1388 and the full moon of the later one, ເດືອນແປດຫລັງ.
    for (day, lao) in [
        (739_797, "ເດືອນແປດ ແຮມ 1 ຄ່ຳ ປີ 1388"),
        (739_826, "ເດືອນແປດຫລັງ ຂຶ້ນ 15 ຄ່ຳ ປີ 1388"),
    ] {
        assert_eq!(text("lao", day, "en"), lao);
        assert_eq!(read("lao", "en", lao).map(|(d, _)| d), Ok(Rd(day)));
    }
    // 15 and 16 February 1990, one tithi at two sunrises: refused alone,
    // and read as the Friday by its weekday.
    assert_eq!(text("nepal-sambat", 726_514, "en"), "Silā 21, 1110 NS");
    assert_eq!(
        read("nepal-sambat", "en", "Silā 21, 1110 NS"),
        Err(DateRefusal::Ambiguous {
            first: Rd(726_513),
            second: Rd(726_514)
        })
    );
    // The same days in the almanac committee's form: the fortnight and the
    // tithi within it, Silā gā 6.
    assert_eq!(
        text("nepal-sambat-fortnight", 726_514, "en"),
        "Silā gā 6, 1110 NS"
    );
    assert_eq!(
        text("nepal-sambat-fortnight", 726_514, "ne"),
        "१११० नेसं सिल्लागा ६"
    );
    assert_eq!(
        read("nepal-sambat-fortnight", "en", "Silā gā 6, 1110 NS"),
        Err(DateRefusal::Ambiguous {
            first: Rd(726_513),
            second: Rd(726_514)
        })
    );
    // Mha Puja, 4 November 2013 (Wikipedia, "Mha Puja"), the first tithi of
    // the first fortnight: कछलाथ्व १, in the Samiti's spelling.
    assert_eq!(
        text("nepal-sambat-fortnight", 735_176, "ne"),
        "११३४ नेसं कछलाथ्व १"
    );
    for with_weekday in ["Silā 21, 1110 NS, Friday", "Friday, Silā 21, 1110 NS"] {
        assert_eq!(
            read("nepal-sambat", "en", with_weekday).map(|(d, _)| d),
            Ok(Rd(726_514)),
            "{with_weekday}"
        );
    }
    // 15 and 16 January 1990, the twentieth of the eleventh Tibetan month
    // twice.
    assert_eq!(
        read("tibetan", "en", "Eleventh Month 20, 1989"),
        Err(DateRefusal::Ambiguous {
            first: Rd(726_482),
            second: Rd(726_483)
        })
    );
}

/// An era English writes with an apostrophe, *Man’en (1860–1861)* as
/// CLDR's root writes it, reads back on its first day, and so does the
/// text with the apostrophe typed as U+0027 or U+02BC, which UTS #35's
/// loose matching treats as the same (Part 1, "Lenient Parsing", "Loose
/// Matching").
#[test]
fn an_era_written_with_an_apostrophe_reads_back_however_it_is_typed() {
    use hyper_calendar::hc_calendars_regional::nengo::{self, Court};
    let registry = hyper_calendar::registry();
    let en: Locale = "en".parse().expect("a tag");
    let mut read_back = 0;
    for era in nengo::ALL.iter() {
        let Some(start) = era.start else {
            continue;
        };
        let id = match era.court {
            Court::Northern => "japanese-northern",
            Court::Southern => "japanese-southern",
            Court::Unified => "japanese",
        };
        let calendar = registry.get_by_name(id).expect("registered");
        let Ok(fields) = calendar.fixed_to_fields(start) else {
            continue;
        };
        let text = label::date(calendar, &fields, &en);
        if !text.contains('\u{2019}') {
            continue;
        }
        for typed in ['\u{2019}', '\u{0027}', '\u{02BC}'] {
            let typed_text = text.replace('\u{2019}', &typed.to_string());
            assert_eq!(
                read(id, "en", &typed_text).map(|(day, _)| day),
                Ok(start),
                "{} {typed_text}",
                era.id
            );
        }
        read_back += 1;
    }
    // 安永 An’ei to 万延 Man’en: the thirteen CLDR root names with one.
    assert_eq!(read_back, 13);
    // The calendar's own romanisation, Man'en, reads as typed with a
    // curly apostrophe too.
    assert_eq!(
        read("japanese", "en", "intercalary March 3, 1 Man’en").map(|(day, _)| day),
        Ok(day_of("gregory", DateFields::ymd(1860, 4, 23)))
    );
}

/// An era the calendar's own table romanises as it does another, 延慶 and
/// 延享 both *Enkyo*, is written in a Latin-script locale with no name of
/// its own by English's name, which tells the two apart, and where English
/// has none by its native name: 貞和 of the Northern court, beside 承和,
/// both *Jowa*.
#[test]
fn a_shared_romanisation_is_not_written() {
    let registry = hyper_calendar::registry();
    let de: Locale = "de".parse().expect("a tag");
    let japanese = registry.get_by_name("japanese").expect("registered");
    let northern = registry
        .get_by_name("japanese-northern")
        .expect("registered");
    for (calendar, id, fields, text) in [
        (
            japanese,
            "japanese",
            DateFields::ymd(1, 12, 1).with_era("enkyo-1308"),
            "1. Dezember 1 Enkyō (1308–1311)",
        ),
        (
            japanese,
            "japanese",
            DateFields::ymd(1, 12, 1).with_era("enkyo-1744"),
            "1. Dezember 1 Enkyō (1744–1748)",
        ),
        (
            northern,
            "japanese-northern",
            DateFields::ymd(3, 1, 1).with_era("jowa-1345"),
            "1. Januar 3 貞和",
        ),
    ] {
        assert_eq!(label::date(calendar, &fields, &de), text, "{id}");
        let day = calendar.fields_to_fixed(&fields).expect("a day");
        assert_eq!(read(id, "de", text).map(|(day, _)| day), Ok(day), "{text}");
    }
}

/// What a date may leave out: an era its calendar has only one of, ۶ مهر
/// ۱۴۰۵ for ۶ مهر ۱۴۰۵ ه.ش.; and not an era of a calendar that has two, nor
/// the year, 9月28日, which is refused as unwritten rather than unread.
#[test]
fn a_date_may_leave_out_an_only_era_and_not_its_year() {
    for text in ["۶ مهر ۱۴۰۵", "6 مهر 1405"] {
        assert_eq!(
            read("persian", "fa", text).map(|(day, _)| day),
            Ok(TODAY),
            "{text}"
        );
    }
    assert_eq!(
        read("persian", "en", "Mehr 6, 1405").map(|(day, _)| day),
        Ok(TODAY)
    );
    // The Ethiopic calendar counts two eras, Amätä Məḥrät and Amätä Aläm.
    assert_eq!(
        read("ethiopic", "en", "Mäskäräm 18, 2019 Amätä Məḥrät").map(|(day, _)| day),
        Ok(TODAY)
    );
    assert!(read("ethiopic", "en", "Mäskäräm 18, 2019").is_err());
    for (calendar, tag, text) in [
        ("gregory", "ja", "9月28日"),
        ("gregory", "ja", "九月二十八日"),
        ("gregory", "en", "September 28"),
        ("gregory", "en", "Monday, September 28"),
        ("gregory", "de", "28. September"),
        ("persian", "fa", "۶ مهر"),
        ("japanese", "ja", "9月28日"),
    ] {
        assert_eq!(
            read(calendar, tag, text),
            Err(DateRefusal::YearNotWritten),
            "{calendar} {tag} {text}"
        );
    }
}

/// The Chinese calendar's year by the related Gregorian year and its stem
/// and branch, as CLDR 48's `zh.xml`, `zh_Hant.xml`, `yue.xml` and
/// `yue_Hans.xml` write it, "rU年", and `ko.xml`, "r년(U년)"; the two must
/// agree, or the text is refused as contradicting itself. The Cantonese
/// long date, "U (r) 年MMMd", puts the related year after the stem and
/// branch. The Dangi
/// date in `zh_Hant.xml`, `yue.xml` and `yue_Hans.xml` is their own,
/// "U年MMMd日".
#[test]
fn the_chinese_year_is_read_by_its_related_gregorian_year() {
    let registry = hyper_calendar::registry();
    for id in ["chinese", "dangi", "vietnamese"] {
        let calendar = registry.get_by_name(id).expect("registered");
        let fields = calendar.fixed_to_fields(TODAY).expect("in range");
        let own_dangi = id == "dangi";
        for (tag, text) in [
            ("zh-Hans", "2026丙午年八月十八"),
            ("zh-Hant", "2026丙午年八月十八"),
            ("yue-Hans", "丙午 (2026) 年八月十八"),
            ("yue-Hant", "丙午 (2026) 年八月十八"),
            ("ko", "2026년(병오년) 8월 18일"),
        ]
        .into_iter()
        .filter(|(tag, _)| !own_dangi || matches!(*tag, "zh-Hans" | "ko"))
        {
            let locale: Locale = tag.parse().expect("a tag");
            assert_eq!(label::date(calendar, &fields, &locale), text, "{id} {tag}");
            assert_eq!(
                read(id, tag, text).map(|(day, _)| day),
                Ok(TODAY),
                "{id} {tag}"
            );
        }
        // The Cantonese date with its spaces left out reads too; the
        // Mandarin order, 2026丙午年八月十八, is not the locale's pattern.
        if !own_dangi {
            for tag in ["yue-Hans", "yue-Hant"] {
                assert_eq!(
                    read(id, tag, "丙午(2026)年八月十八").map(|(day, _)| day),
                    Ok(TODAY),
                    "{id} {tag}"
                );
                assert!(
                    matches!(
                        read(id, tag, "2026丙午年八月十八"),
                        Err(DateRefusal::NotRecognised { .. })
                    ),
                    "{id} {tag}"
                );
            }
        }
        for (tag, text) in [
            ("zh-Hans", "2025丙午年八月十八"),
            ("zh-Hant", "2026乙巳年八月十八"),
            ("yue-Hant", "丙午 (2025) 年八月十八"),
            ("ko", "2026년(을사년) 8월 18일"),
        ]
        .into_iter()
        .filter(|(tag, _)| !own_dangi || !tag.ends_with("Hant"))
        {
            assert_eq!(
                read(id, tag, text),
                Err(DateRefusal::FieldMismatch),
                "{id} {tag} {text}"
            );
        }
        // English writes the Chinese and Dangi years as CLDR's `en.xml`
        // does, "r(U)", and the Vietnamese year by its number alone.
        if id != "vietnamese" {
            assert_eq!(
                read(id, "en", "Eighth Month 18, 2025(bing-wu)"),
                Err(DateRefusal::FieldMismatch),
                "{id}"
            );
        }
        // Japanese writes the stem and branch alone, as CLDR's `ja.xml`
        // does, and 丙午 recurs.
        assert_eq!(
            read(id, "ja", "丙午年八月18日"),
            Err(DateRefusal::YearNotWritten),
            "{id}"
        );
        assert_eq!(
            read(id, "zh-Hans", "丙午年八月十八"),
            Err(DateRefusal::YearNotWritten)
        );
    }
    // The Dangi in Traditional Chinese and Cantonese: 丙午年八月18日, which
    // names no year of the sixty.
    let dangi = registry.get_by_name("dangi").expect("registered");
    let fields = dangi.fixed_to_fields(TODAY).expect("in range");
    for tag in ["zh-Hant", "yue-Hant", "yue-Hans"] {
        let locale: Locale = tag.parse().expect("a tag");
        assert_eq!(
            label::date(dangi, &fields, &locale),
            "丙午年八月18日",
            "{tag}"
        );
        assert_eq!(
            read("dangi", tag, "丙午年八月18日"),
            Err(DateRefusal::YearNotWritten),
            "{tag}"
        );
    }
    // 2023癸卯年闰二月初一, the first day of the leap second month, 22 March
    // 2023.
    let leap = day_of("gregory", DateFields::ymd(2023, 3, 22));
    let chinese = registry.get_by_name("chinese").expect("registered");
    let fields = chinese.fixed_to_fields(leap).expect("in range");
    let zh: Locale = "zh-Hans".parse().expect("a tag");
    assert_eq!(label::date(chinese, &fields, &zh), "2023癸卯年闰二月初一");
    assert_eq!(
        read("chinese", "zh-Hans", "2023癸卯年闰二月初一").map(|(day, _)| day),
        Ok(leap)
    );
}

/// The Mongolian calendar in Mongolian, by the month names of Gantumur's
/// calendar and its leap word, илүү, before сар, inside `mn.xml`'s
/// Gregorian long date (docs/systems/tibetan-almanac.md): Tsagaan Sar on
/// 18 February 2026, as MONTSAME dates it; the constitution of 1992, in
/// force on day 9 of the first spring month, 12 February 1992; and the
/// leap twelfth month of 2024, a month before the regular one
/// (docs/systems/tibetan-variants.md).
#[test]
fn mongolian_dates_are_written_in_mongolian() {
    use hyper_calendar::hc_calendar::units::Unit;
    use hyper_calendar::hc_calendars_regional::tibetan_almanac::{
        MONGOLIAN_LEAP_WORD, MONGOLIAN_MONTH_NAMES,
    };
    let registry = hyper_calendar::registry();
    let mongolian = registry.get_by_name("mongolian").expect("registered");
    let mn: Locale = "mn".parse().expect("a tag");
    assert_eq!(label::locale_for(mongolian, None), mn);
    for (day, text) in [
        (
            day_of("gregory", DateFields::ymd(2026, 2, 18)),
            "2026 оны хаврын тэргүүн сарын 1",
        ),
        (
            day_of("gregory", DateFields::ymd(1992, 2, 12)),
            "1992 оны хаврын тэргүүн сарын 9",
        ),
    ] {
        let fields = mongolian.fixed_to_fields(day).expect("in range");
        assert_eq!(label::date(mongolian, &fields, &mn), text);
        assert_eq!(read("mongolian", "mn", text).map(|(read, _)| read), Ok(day));
    }
    let leap = DateFields {
        month: Some(hyper_calendar::hc_calendar::Month::leap(12)),
        ..DateFields::ymd(2024, 12, 1)
    };
    let leap_day = mongolian.fields_to_fixed(&leap).expect("a leap month");
    let regular = day_of("mongolian", DateFields::ymd(2024, 12, 1));
    assert!(leap_day < regular);
    assert_eq!(
        label::label(mongolian, &leap, Unit::Month, &mn),
        "Өвлийн сүүл илүү сар"
    );
    for (day, text) in [
        (leap_day, "2024 оны өвлийн сүүл илүү сарын 1"),
        (regular, "2024 оны өвлийн сүүл сарын 1"),
    ] {
        let fields = mongolian.fixed_to_fields(day).expect("in range");
        assert_eq!(label::date(mongolian, &fields, &mn), text);
        assert_eq!(read("mongolian", "mn", text).map(|(read, _)| read), Ok(day));
    }
    // The names are the almanac module's, which cites the calendar.
    for (ordinal, name) in (1..=12).zip(MONGOLIAN_MONTH_NAMES) {
        let fields = DateFields::ymd(2026, ordinal, 1);
        assert_eq!(
            label::label(mongolian, &fields, Unit::Month, &mn),
            format!("{name} сар")
        );
    }
    assert_eq!(MONGOLIAN_LEAP_WORD, "илүү");
}

/// Each refusal, on a text that earns it.
#[test]
fn a_text_that_is_not_one_day_is_refused_with_the_reason() {
    assert_eq!(read("gregory", "en", "  "), Err(DateRefusal::Empty));
    assert!(matches!(
        read("gregory", "en", "September 28, 2026 at noon"),
        Err(DateRefusal::NotRecognised { .. })
    ));
    assert_eq!(
        read("gregory", "en", "September 28, 26"),
        Err(DateRefusal::TwoDigitYear)
    );
    assert_eq!(
        read("gregory", "en", "September 28, 0026").map(|(_, f)| f.year),
        Ok(26)
    );
    assert_eq!(
        read("gregory", "en", "Tuesday, September 28, 2026"),
        Err(DateRefusal::WeekdayMismatch {
            written: Weekday::Tuesday,
            actual: Weekday::Monday
        })
    );
    assert!(matches!(
        read("gregory", "en", "February 30, 2026"),
        Err(DateRefusal::NoSuchDate(_))
    ));
    // 癸卯年 is every sixtieth year.
    assert_eq!(
        read("chinese", "zh-Hans", "癸卯年闰二月初一"),
        Err(DateRefusal::YearNotWritten)
    );
    // 2025 is 乙巳, not 丙午.
    assert_eq!(
        read("chinese", "zh-Hans", "2025丙午年八月十八"),
        Err(DateRefusal::FieldMismatch)
    );
    // No year: 9月28日 is every year's.
    assert_eq!(
        read("gregory", "ja", "9月28日"),
        Err(DateRefusal::YearNotWritten)
    );
    // A Stata week names seven days.
    assert!(matches!(
        read("stata-week", "en", "2026w39"),
        Err(DateRefusal::Ambiguous { .. })
    ));
}

/// A year of the sexagenary cycle alone names a day only where the
/// calendar's range holds one such year. The Taichu's range holds two,
/// 60 years apart (丁丑 is 104 BC and 44 BC), and the Jingchu's two (庚申
/// is 240 and 360); the reader once took the cycle of its probe days for
/// the day's and read the earlier day of each as the later one.
#[test]
fn a_year_of_the_cycle_alone_is_not_read_as_the_cycle_of_the_probes() {
    for (id, text) in [
        ("chinese-taichu", "丁丑年五月1日"),
        ("chinese-jingchu", "庚申年正月1日"),
    ] {
        assert_eq!(
            read(id, "ja", text).map(|(day, _)| day),
            Err(DateRefusal::YearNotWritten),
            "{id} {text}"
        );
    }
}
