//! A written date reads back as the day it was written for: every
//! registered calendar's dates, formatted by `hc_format::label::date` in
//! every carried locale, are read by `hc_format::label::parse_date` as
//! the day they were written for, or refused for one of the reasons
//! `REFUSALS` lists — the text names a cycle's place or a span, not a day
//! — or because the year is written in one or two digits.
//!
//! A release build reads every sample day in every locale. A debug build,
//! which the coverage job runs instrumented, reads every sample day in
//! the calendar's own language, and each other locale on one day of every
//! fifth calendar, staggered, so that every locale is still read in
//! forty-odd calendars and every calendar in a dozen locales.
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

use hyper_calendar::hc_calendar::{CalendarMeta, DateFields, Rd, Weekday};
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
    // The year by its stem and branch alone, as the Chinese, Japanese
    // and Korean templates write it: 癸卯年 recurs every sixty years.
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
        "vietnamese",
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
    // day under the year is this library's choice, and Henning's
    // Bhutanese archive writes both days alike.
    (
        "fasli-bombay",
        "ambiguous",
        "the doubled 3 June is written as the ordinary one",
    ),
    (
        "sur-san",
        "ambiguous",
        "the doubled 3 June is written as the ordinary one",
    ),
    (
        "tibetan-bhutan",
        "ambiguous",
        "a doubled lunar day is written as the ordinary one",
    ),
];

/// The days a calendar's dates are read on: the start of 1 CE, of the
/// Gregorian reform, of 1900, of the Unix epoch, of 2026 and a day of
/// it, and 2100, where the calendar converts them, else its own sample
/// day; and its first and last days.
fn sample_days(meta: &CalendarMeta) -> Vec<Rd> {
    let mut days: Vec<Rd> = [1, 577_736, 693_761, 719_163, 739_617, 739_887, 767_000]
        .into_iter()
        .map(|day| meta.sample_day(Rd(day)))
        .collect();
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
/// in the calendar's own language, and on one day in every fifth pairing
/// of calendar and locale.
fn read_in(calendar: usize, day: usize, days: usize, locale: usize, locales: usize) -> bool {
    if !cfg!(debug_assertions) || locale + 1 == locales {
        return true;
    }
    (calendar + locale).is_multiple_of(5) && (calendar + locale) / 5 % days == day
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
                    let locale = label::locale_for(calendar, requested.as_ref());
                    let text = label::date(calendar, &fields, &locale);
                    read += 1;
                    match label::parse_date(calendar, &locale, &text) {
                        Ok(parsed) if parsed.fixed == *day => {}
                        Ok(parsed) => {
                            faults.insert(format!(
                                "{} {locale} {text:?}: read as {} for {}",
                                meta.id.0, parsed.fixed.0, day.0
                            ));
                        }
                        Err(DateRefusal::TwoDigitYear) if fields.year.abs() < 100 => {}
                        Err(refusal) => {
                            if expected.contains_key(&(meta.id.0, refusal.name())) {
                                met.insert((meta.id.0, refusal.name()));
                            } else {
                                faults.insert(format!(
                                    "{} {locale} {} {text:?}: {refusal}",
                                    meta.id.0, day.0
                                ));
                            }
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
    // Every locale is read in some calendars, and every calendar in some
    // locales, in either build.
    let calendars: BTreeSet<&str> = paired.iter().map(|(calendar, _)| *calendar).collect();
    let read_locales: BTreeSet<usize> = paired.iter().map(|(_, locale)| *locale).collect();
    assert_eq!(calendars.len(), registry.len());
    assert_eq!(read_locales.len(), locales.len());
    if !cfg!(debug_assertions) {
        let stale: Vec<_> = expected.keys().filter(|key| !met.contains(*key)).collect();
        assert!(
            stale.is_empty(),
            "refusals the sweep no longer meets: {stale:?}"
        );
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
    // 令和元年, the first year of Reiwa, 1 May 2019; 嘉永三年, an era no
    // locale's data lists, by the calendar's own name for it.
    let reiwa = day_of("japanese", DateFields::ymd(1, 5, 1).with_era("reiwa"));
    assert_eq!(
        read("japanese", "ja", "令和元年5月1日").map(|(day, _)| day),
        Ok(reiwa)
    );
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
    // dates the reunion of the courts, 元中9年閏10月5日, and "intercalary" as
    // Bramsen's tables call the month. German, which carries no word for
    // it, writes both days alike.
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
        ("en", "intercalary March 3, 1 Man'en"),
        ("zh-Hant", "万延1年閏3月3日"),
        ("zh-Hans", "万延1年闰3月3日"),
        ("yue-Hant", "万延1年閏3月3日"),
        ("yue-Hans", "万延1年闰3月3日"),
    ] {
        let locale: Locale = tag.parse().expect("a tag");
        assert_eq!(label::date(japanese, &fields, &locale), text, "{tag}");
        assert_eq!(
            read("japanese", tag, text).map(|(day, _)| day),
            Ok(leap),
            "{tag}"
        );
    }
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
    // The fields are the calendar's own for the day.
    let (_, fields) = read("japanese", "ja", "令和8年9月28日").expect("a day");
    assert_eq!((fields.era, fields.year), (Some("reiwa"), 8));
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
    // A Stata week names seven days.
    assert!(matches!(
        read("stata-week", "en", "2026w39"),
        Err(DateRefusal::Ambiguous { .. })
    ));
}
