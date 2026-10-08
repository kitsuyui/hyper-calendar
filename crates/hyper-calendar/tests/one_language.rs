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
//! the days [`DAYS`] lists, and holds each pairing to properties that do not
//! read the renderer's own lookups. The names a date writes are the locale's,
//! CLDR root's, or the calendar's own in the locale's script (romanised for a
//! Latin locale); English's name for an era is written by a Latin locale only
//! where the era's romanisation is shared with another era of the calendar.
//! The era a line names is one of those. Where the locale names the calendar,
//! the letters outside its names are in its script too; that check reads a
//! date's notation as well, such as `2026-W39-1`, so it runs only there. A date
//! left to English is English's date for the same day, character for
//! character. A debug build pairs each day with one locale in a stride,
//! staggered, and a release build pairs them all (`locale_sample`).

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

/// The names the locale's data gives the calendar's eras and months, at every
/// width and in every context: CLDR root's among them, since a locale's
/// fallback chain ends in root.
fn stated_names(locale: &Locale, calendar: &dyn DynCalendar) -> Vec<&'static str> {
    let id = calendar.meta().id;
    let mut stated: Vec<&'static str> = Vec::new();
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
    stated
}

/// The calendar's own names: each era's name in its orthography and its
/// romanisation, and the positions of each of its cycles.
fn own_names(calendar: &dyn DynCalendar) -> Vec<&'static str> {
    let mut own: Vec<&'static str> = Vec::new();
    for code in era_codes(calendar) {
        if let Some(name) = calendar.era_name(code) {
            own.extend([name.native, name.romanised]);
        }
    }
    for cycle in calendar.cycles() {
        own.extend(cycle.names.iter().copied());
    }
    own
}

/// Every era code the calendar writes, in order.
fn era_codes(calendar: &dyn DynCalendar) -> impl Iterator<Item = &'static str> + '_ {
    (0..).map_while(|index| calendar.era_code(index))
}

/// Whether an era of the calendar has a romanisation another of its eras
/// has too, so that the romanisation alone names neither.
fn shares_romanisation(calendar: &dyn DynCalendar, code: &str) -> bool {
    let Some(own) = calendar.era_name(code) else {
        return false;
    };
    !own.romanised.is_empty()
        && era_codes(calendar).any(|other| {
            other != code
                && calendar
                    .era_name(other)
                    .is_some_and(|name| name.romanised.eq_ignore_ascii_case(own.romanised))
        })
}

/// `text` with each of `names` taken out, longest first.
fn strip(text: &str, mut names: Vec<&str>) -> String {
    names.retain(|name| !name.is_empty());
    names.sort_by_key(|name| std::cmp::Reverse(name.len()));
    names.dedup();
    let mut rest = text.to_string();
    for name in names {
        rest = rest.replace(name, " ");
    }
    rest
}

/// Whether `name` occurs in `text` as a word of its own, not inside a longer
/// run of letters.
fn has_word(text: &str, name: &str) -> bool {
    text.match_indices(name).any(|(at, _)| {
        let before = text[..at].chars().next_back();
        let after = text[at + name.len()..].chars().next();
        !before.is_some_and(char::is_alphabetic) && !after.is_some_and(char::is_alphabetic)
    })
}

/// `text` without the names the locale's data states and the calendar's own
/// names, and English's name for each of its eras: whatever the files state,
/// `R.O.C.` in Bengali or `I` after a Hijri month in Marathi, is the locale's
/// own, and a proper name no locale translates, *Reiwa*, is written romanised,
/// as root writes it.
fn without_named(text: &str, locale: &Locale, calendar: &dyn DynCalendar) -> String {
    let id = calendar.meta().id;
    let english = names::english();
    let mut stated = stated_names(locale, calendar);
    stated.extend(own_names(calendar));
    for code in era_codes(calendar) {
        // English's name, root's, for an era whose romanisation another era
        // shares.
        if calendar.era_name(code).is_some() {
            stated.extend(names::era_name_by_code(&english, id, code, NameWidth::Wide));
        }
    }
    strip(text, stated)
}

/// The names a date writes that its locale cannot write, by the rule the
/// renderer follows: the calendar's own names, which a locale writes in its
/// own script or, where it is a Latin one, romanised, and never in Han, kana
/// or Hangul when the locale is one of those; and English's name for an era,
/// which a Latin locale writes only where the romanisation of that era is
/// shared with another era. The locale's own names and CLDR root's are set
/// aside first. A name counts only as a word of its own.
fn names_it_cannot_write(
    text: &str,
    locale: &Locale,
    calendar: &dyn DynCalendar,
) -> Vec<&'static str> {
    let id = calendar.meta().id;
    let script = family(names::locale_data(locale).script);
    let native = script == "Hani";
    let rest = strip(text, stated_names(locale, calendar));
    let own = own_names(calendar);
    let mut found: Vec<&'static str> = Vec::new();
    for name in &own {
        if !has_word(&rest, name) {
            continue;
        }
        let scripts = || {
            name.chars()
                .filter(|letter| letter.is_alphabetic())
                .map(script_of)
        };
        let cannot = if native {
            scripts().any(|written| written == "Latn")
        } else {
            scripts().any(|written| written != "Latn" && written != script)
        };
        if cannot {
            found.push(name);
        }
    }
    let english = names::english();
    for code in era_codes(calendar) {
        let Some(name) = names::era_name_by_code(&english, id, code, NameWidth::Wide) else {
            continue;
        };
        if !has_word(&rest, name) || own.contains(&name) {
            continue;
        }
        if native || !shares_romanisation(calendar, code) {
            found.push(name);
        }
    }
    found
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
                // calendar's own in the locale's script or, for a Latin locale,
                // romanised; English's name only where a Latin locale writes an
                // era whose romanisation another era shares.
                if let Some(code) = fields.era {
                    let era = label::label(calendar, &fields, Unit::Era, &used);
                    let named = names::era_name_by_code(&used, meta.id, code, NameWidth::Wide);
                    let native = family(names::locale_data(&used).script) == "Hani";
                    let own = calendar.era_name(code).is_some_and(|own| {
                        let written = if native || own.romanised.is_empty() {
                            own.native
                        } else {
                            own.romanised
                        };
                        era.as_str() == written
                    });
                    let english_shared = !native
                        && shares_romanisation(calendar, code)
                        && names::era_name_by_code(&english, meta.id, code, NameWidth::Wide)
                            == Some(era.as_str());
                    if !(era.is_empty() || named == Some(era.as_str()) || own || english_shared) {
                        faults.push(format!(
                            "{tag} {id} {day:?}: the era {era:?} of {text:?} is not one {used} writes"
                        ));
                    }
                }
                // Every name a date writes is one its locale can write, in
                // every pairing: a calendar the locale does not name is no
                // exception, since the renderer then writes the name in the
                // locale's script or leaves it out.
                for name in names_it_cannot_write(&text, &used, calendar) {
                    faults.push(format!(
                        "{tag} {id} {day:?}: {text:?} writes the name {name:?}, which {used} cannot write"
                    ));
                }
                // Where the locale names the calendar, the letters outside its
                // names are in its script. Only there: the other dates are
                // written in notation as well, such as 2026-W39-1. English is
                // not checked this way: its data states the Thai and Lao month
                // names of two calendars in their own script, and those are
                // stated, so the name check above reads them as English's own.
                if used.language() == "en" || !names::names_calendar(&used, meta.id) {
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

/// A Japanese date of a calendar whose names are Latin letters, which
/// Japanese cannot write, is English's whole, and says so in `locale used`;
/// and a calendar's own Han month name is not written in an English date,
/// where its number is. The texts are what the renderer writes for 21
/// September 2026, and for the Taiping day of 679 390 for the last one, the
/// 30th day of the twelfth month of Taiping year 10.
#[test]
fn a_cjk_locale_leaves_a_calendar_of_latin_names_to_english() {
    let registry = registry();
    let ja: Locale = "ja".parse().expect("a tag");
    for (calendar, day, text) in [
        ("pax", 739_880, "2026 October 16"),
        ("icelandic", 739_880, "2026 Tvímánuður 28"),
        ("international-fixed", 739_880, "2026 September 12"),
        ("taiping-tianli", 679_390, "12 30, 10 Taiping"),
    ] {
        let calendar = registry.get_by_name(calendar).expect("registered");
        let used = label::locale_for(calendar, Some(&ja));
        assert_eq!(
            names::locale_data(&used).tag,
            "en",
            "{}",
            calendar.meta().id.0
        );
        let fields = calendar.fixed_to_fields(Rd(day)).expect("converts");
        assert_eq!(
            label::date(calendar, &fields, &used),
            text,
            "{}",
            calendar.meta().id.0
        );
    }
    let english: Locale = "en".parse().expect("a tag");
    let taiping = registry.get_by_name("taiping-tianli").expect("registered");
    let fields = taiping.fixed_to_fields(Rd(679_390)).expect("converts");
    assert_eq!(label::date(taiping, &fields, &english), "12 30, 10 Taiping");
}
