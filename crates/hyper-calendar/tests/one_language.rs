//! A date is written in one language: every month and era name of a
//! rendered date is a name of the locale the line reports as used, CLDR
//! root's (the abbreviations `AH`, `AM`, `BE` that belong to every
//! language), or the calendar's own name for it in the locale's script. A
//! locale that cannot name each of them leaves the whole date to English
//! and says so in `locale used`, as `hc-humanize` serves a locale only if
//! one catalogue translates every message of a function
//! (`crates/hc-humanize/tests/serving.rs`).
//!
//! The sweep pairs every carried locale with every registered calendar, on
//! the days [`DAYS`] lists, and holds each pairing to three properties that
//! do not read the renderer's own lookups: the letters of the date are in
//! the script of the locale used, once the names its data and CLDR root
//! state for the calendar are set aside; the era a line names is the locale's, root's or the calendar's
//! own; and a date left to English is English's date for the same day,
//! character for character. A debug build pairs each day with one locale
//! in a stride, staggered, and a release build pairs them all
//! (`locale_sample`).

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

use hyper_calendar::hc_calendar::units::Unit;
use hyper_calendar::hc_calendar::{DynCalendar, Month, Rd};
use hyper_calendar::hc_format::label;
use hyper_calendar::hc_i18n::Locale;
use hyper_calendar::hc_i18n::data::LOCALES;
use hyper_calendar::hc_i18n::names::{self, NameContext, NameWidth};
use hyper_calendar::{lines, registry};

mod locale_sample;

/// The days each calendar is rendered on where it converts them, else on its
/// sample day: 21 September 2026, a day in 1900 and one in 1582, so that the
/// eras of the Gregorian family, the Japanese calendars and the regnal ones
/// are met.
const DAYS: [i64; 3] = [739_880, 693_761, 577_736];

/// The script a letter is written in, as the families this crate carries
/// fall: `Hani` for Han, kana and Hangul alike, which the Chinese, Japanese
/// and Korean locales write together.
fn script_of(letter: char) -> &'static str {
    match u32::from(letter) {
        0x41..=0x2FF | 0x1E00..=0x1EFF => "Latn",
        0x370..=0x3FF => "Grek",
        0x400..=0x52F => "Cyrl",
        0x590..=0x5FF | 0xFB1D..=0xFB4F => "Hebr",
        0x600..=0x6FF | 0x750..=0x77F | 0xFB50..=0xFDFF | 0xFE70..=0xFEFF => "Arab",
        0x700..=0x74F => "Syrc",
        0x840..=0x85F => "Mand",
        0x900..=0x97F => "Deva",
        0x980..=0x9FF => "Beng",
        0xA00..=0xA7F => "Guru",
        0xB80..=0xBFF => "Taml",
        0xC00..=0xC7F => "Telu",
        0xD00..=0xD7F => "Mlym",
        0xE00..=0xE7F => "Thai",
        0xF00..=0xFFF => "Tibt",
        0x1000..=0x109F => "Mymr",
        0x1200..=0x139F => "Ethi",
        0x1100..=0x11FF
        | 0x3040..=0x30FF
        | 0x3400..=0x4DBF
        | 0x4E00..=0x9FFF
        | 0xAC00..=0xD7AF
        | 0xF900..=0xFAFF
        | 0x20000..=0x2FA1F => "Hani",
        0x2C80..=0x2CFF => "Copt",
        0x2D30..=0x2D7F => "Tfng",
        _ => "other",
    }
}

/// The script family a locale's data entry is written in.
fn family(script: &str) -> &str {
    match script {
        "Jpan" | "Hans" | "Hant" | "Kore" => "Hani",
        other => other,
    }
}

/// `text` without the names the locale's data gives the calendar's eras and
/// months, CLDR root's among them, and the calendar's own: whatever the files
/// state, `R.O.C.` in Bengali or `I` after a Hijri month in Marathi, is the
/// locale's own, and a proper name no locale translates, *Reiwa*, is written
/// romanised, as root writes it.
fn without_named(text: &str, locale: &Locale, calendar: &dyn DynCalendar) -> String {
    let id = calendar.meta().id;
    let mut stated: Vec<&str> = Vec::new();
    names::for_each_era_name(locale, id, |_, name| stated.push(name));
    for ordinal in 1..=13 {
        for leap in [false, true] {
            for in_leap_year in [false, true] {
                for width in [NameWidth::Wide, NameWidth::Abbreviated, NameWidth::Narrow] {
                    for context in [NameContext::Format, NameContext::Standalone] {
                        let month = Month { ordinal, leap };
                        if let Some(label) =
                            names::month_label_in(locale, id, month, in_leap_year, width, context)
                        {
                            stated.extend([label.prefix, label.name, label.suffix]);
                        }
                    }
                }
            }
        }
    }
    // The calendar's own names, which a locale writes romanised as CLDR's
    // root does, and its own cycles' names.
    let english = names::english();
    for code in (0..).map_while(|index| calendar.era_code(index)) {
        if let Some(own) = calendar.era_name(code) {
            stated.extend([own.native, own.romanised]);
            // English's name, root's, for an era whose romanisation
            // another era shares.
            stated.extend(names::era_name_by_code(&english, id, code, NameWidth::Wide));
        }
    }
    for cycle in calendar.cycles() {
        stated.extend(cycle.names.iter().copied());
    }
    stated.retain(|name| !name.is_empty());
    stated.sort_by_key(|name| std::cmp::Reverse(name.len()));
    stated.dedup();
    let mut rest = text.to_string();
    for name in stated {
        rest = rest.replace(name, " ");
    }
    rest
}

/// The days of a calendar the sweep renders: the days of [`DAYS`] it
/// converts, else its sample day.
fn days_of(calendar: &dyn DynCalendar) -> Vec<Rd> {
    let meta = calendar.meta();
    let mut days: Vec<Rd> = DAYS.iter().map(|day| meta.sample_day(Rd(*day))).collect();
    days.dedup();
    days
}

#[test]
fn every_date_is_written_in_one_language() {
    let registry = registry();
    let english = names::english();
    let locales: Vec<Locale> = LOCALES
        .iter()
        .map(|data| data.tag.parse().expect("a carried tag parses"))
        .collect();
    let mut checked = 0usize;
    let mut left_to_english = 0usize;
    let mut faults: Vec<String> = Vec::new();
    for (number, locale) in locales.iter().enumerate() {
        for (index, meta) in registry.metas().enumerate() {
            let calendar = registry.get(meta.id).expect("registered");
            let days = days_of(calendar);
            for (nth, day) in days.iter().enumerate() {
                if !locale_sample::paired(nth + index, days.len(), number, locales.len(), false) {
                    continue;
                }
                let Ok(fields) = calendar.fixed_to_fields(*day) else {
                    continue;
                };
                let used = label::locale_for(calendar, Some(locale));
                let text = label::date(calendar, &fields, &used);
                checked += 1;
                let tag = LOCALES[number].tag;
                let id = meta.id.0;
                // A date left to English is English's date, whole.
                if used == english && locale.language() != "en" {
                    left_to_english += 1;
                    assert_eq!(
                        text,
                        label::date(calendar, &fields, &english),
                        "{tag} {id}: left to English"
                    );
                    continue;
                }
                // The era a line names is the locale's or root's, else the
                // calendar's own, in the locale's script.
                if let Some(code) = fields.era {
                    let era = label::label(calendar, &fields, Unit::Era, &used);
                    let named = names::era_name_by_code(&used, meta.id, code, NameWidth::Wide);
                    // The calendar's own name, or English's for an era whose
                    // romanisation another era shares, which is CLDR root's
                    // and the same word in every language.
                    let own = calendar.era_name(code).is_some_and(|own| {
                        [own.native, own.romanised].contains(&era.as_str())
                            || names::era_name_by_code(&english, meta.id, code, NameWidth::Wide)
                                == Some(era.as_str())
                    });
                    if !(era.is_empty() || named == Some(era.as_str()) || own) {
                        faults.push(format!(
                            "{tag} {id} {day:?}: the era {era:?} of {text:?} is English's, not {used}'s"
                        ));
                    }
                }
                // A calendar no locale names, which every date writes by
                // the names of its own shape, is no one language's, and
                // English is where the rule ends: it writes those names too.
                if !names::names_calendar(&used, meta.id) || used.language() == "en" {
                    continue;
                }
                // The letters of the date are the script of the locale.
                let script = family(names::locale_data(&used).script);
                let foreign = |text: &str| {
                    text.chars()
                        .filter(|letter| letter.is_alphabetic())
                        .find(|letter| script_of(*letter) != script)
                };
                // Only a date with a letter of another script is asked what
                // the locale's data states, which is the slow part.
                let plain = if foreign(&text).is_some() {
                    without_named(&text, &used, calendar)
                } else {
                    continue;
                };
                if let Some(letter) = plain
                    .chars()
                    .filter(|letter| letter.is_alphabetic())
                    .find(|letter| script_of(*letter) != script)
                {
                    faults.push(format!(
                        "{tag} {id} {day:?}: {text:?} writes {letter:?} ({}) in a {script} date of {used}",
                        script_of(letter)
                    ));
                }
            }
        }
    }
    assert!(
        faults.is_empty(),
        "{} of {checked} pairings are in two languages:\n{}",
        faults.len(),
        faults
            .iter()
            .take(60)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert!(checked > 10_000, "{checked} pairings");
    assert!(left_to_english > 500, "{left_to_english} left to English");
}

/// The line a locale's `hc_describe_day` writes for a calendar carries the
/// locale the rule above chose, and its date is the text that locale
/// writes.
#[test]
fn a_described_day_reports_the_locale_that_wrote_it() {
    let registry = registry();
    let day = Rd(739_880);
    for tag in ["ja", "ko", "ru", "de", "ar", "am", "th", "en-GB", "he"] {
        let locale: Locale = tag.parse().expect("a tag");
        let text = lines::describe_day(&registry, day, tag);
        let mut seen = 0usize;
        for line in text.lines() {
            let cells: Vec<&str> = line.split('\t').collect();
            let calendar = registry.get_by_name(cells[0]).expect("registered");
            if cells[11] != "0" && !cells[11].is_empty() {
                continue;
            }
            let used = label::locale_for(calendar, Some(&locale));
            let fields = calendar.fixed_to_fields(day).expect("converts");
            assert_eq!(
                cells[15],
                label::date(calendar, &fields, &used),
                "{tag} {}",
                cells[0]
            );
            assert_eq!(
                cells[16],
                names::locale_data(&used).tag,
                "{tag} {}",
                cells[0]
            );
            seen += 1;
        }
        assert!(seen > 150, "{tag}: {seen} lines");
    }
}

/// The rule leaves a date to English only where the locale cannot name it:
/// these pairings, whose files name the months and the eras, are written in
/// the locale, whatever the script of the calendar's own names.
#[test]
fn a_locale_that_names_a_calendar_writes_its_dates() {
    let registry = registry();
    for (tag, calendar) in [
        ("fa", "persian"),
        ("fa", "islamic-civil"),
        ("ar", "islamic-civil"),
        ("ar", "japanese"),
        ("he", "hebrew"),
        ("ja", "japanese"),
        ("th", "japanese"),
        ("th", "buddhist"),
        ("ru", "buddhist"),
        ("ru", "japanese"),
        ("hi", "indian"),
        ("ko", "hebrew"),
        ("am", "ethiopic"),
        ("bn", "buddhist"),
        ("de", "persian"),
        ("tr", "buddhist"),
        ("zh-Hans", "japanese"),
    ] {
        let locale: Locale = tag.parse().expect("a tag");
        let calendar = registry.get_by_name(calendar).expect("registered");
        let used = label::locale_for(calendar, Some(&locale));
        assert_eq!(
            names::locale_data(&used).tag,
            tag,
            "{tag} {}",
            calendar.meta().id.0
        );
    }
}
