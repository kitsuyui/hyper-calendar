//! A formatted date is text for a reader, and the extra fields are data
//! for a program: every registered calendar, on a spread of days, in
//! every carried locale and in each calendar's own, writes its date and
//! its units without a `name=value` pair, without an extra field's
//! identifier and without an era's code; and every extra field it sets has
//! a label and a line of its own in `lines::day_extras`.
//!
//! The extra fields a calendar's sources write in its dates are written
//! by the notations of `hc_i18n::notation` and the locale templates that
//! name them, `{extra:FIELD}`; the rest are metadata and stay out of the
//! text. The anchors below are the sources' own written forms.

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

use std::collections::BTreeSet;

use hyper_calendar::hc_calendar::units::Unit;
use hyper_calendar::hc_calendar::{CalendarId, CalendarMeta, DynCalendar, Rd};
use hyper_calendar::hc_format::label;
use hyper_calendar::hc_i18n::Locale;
use hyper_calendar::hc_i18n::data::LOCALES;
use hyper_calendar::hc_i18n::fields;
use hyper_calendar::hc_i18n::names::{self, NameWidth};
use hyper_calendar::lines;

mod locale_sample;

/// The days a calendar is sampled on: the days of the line digests where
/// the calendar converts them, else its own sample day, and a run of
/// consecutive days from the middle of its range, which crosses a month's
/// halves, a décade's days and the week-day names.
fn sample_days(meta: &CalendarMeta) -> Vec<Rd> {
    let mut days: Vec<Rd> = [
        1, 577_736, 620_700, 693_761, 719_163, 730_179, 738_601, 739_617, 739_886, 767_000,
    ]
    .into_iter()
    .map(|day| meta.sample_day(Rd(day)))
    .collect();
    if let Some(earliest) = meta.earliest {
        days.push(earliest);
    }
    if let Some(latest) = meta.latest {
        days.push(latest);
    }
    let middle = meta.sample_day(Rd(739_886)).0;
    days.extend((0..24).map(|step| Rd(middle + step * 5)));
    days.sort_unstable();
    days.dedup();
    days.retain(|day| meta.supports(*day));
    days
}

/// Every locale a test renders in: each carried locale's own tag, the
/// root, and `None` for each calendar's own language.
fn locales() -> Vec<Option<Locale>> {
    LOCALES
        .iter()
        .filter_map(|data| data.tag.parse().ok())
        .map(Some)
        .chain([Some(Locale::ROOT), None])
        .collect()
}

/// The words of a text: runs of letters, digits, hyphens and underscores,
/// the characters an identifier is made of.
fn words(text: &str) -> impl Iterator<Item = &str> + Clone {
    text.split(|c: char| !(c.is_alphanumeric() || c == '-' || c == '_'))
        .filter(|word| !word.is_empty())
}

/// What is wrong with one rendered text of a date, if anything.
fn fault(
    calendar: &dyn DynCalendar,
    fields: &hyper_calendar::hc_calendar::DateFields,
    locale: &Locale,
    text: &str,
    identifiers: &BTreeSet<&str>,
) -> Option<String> {
    if text.contains('=') {
        return Some(String::from("a name=value pair"));
    }
    // A field's identifier is never written; a value's name that happens
    // to be spelled like one, the Burmese half *waning*, is its name.
    let values: Vec<String> = fields
        .extra
        .iter()
        .map(|extra| label::extra(calendar, fields, extra.name, locale))
        .collect();
    let named = |word: &str| values.iter().any(|value| words(value).any(|w| w == word));
    if let Some(word) = words(text).find(|word| identifiers.contains(word) && !named(word)) {
        return Some(format!("the field identifier {word}"));
    }
    // An era's code is never written; an abbreviation that happens to be
    // spelled like it, `AH`, is its name.
    let code = fields.era?;
    let named = [NameWidth::Wide, NameWidth::Abbreviated, NameWidth::Narrow]
        .into_iter()
        .filter_map(|width| {
            names::era_label(
                locale,
                calendar.meta().id,
                code,
                calendar.era_name(code),
                width,
            )
        })
        .collect::<Vec<_>>();
    words(text)
        .any(|word| word == code && !named.contains(&word))
        .then(|| format!("the era code {code}"))
}

/// Every calendar's sample days in every locale: every pairing in a
/// release build, and in a debug build a staggered share of them that
/// still renders every calendar in every locale, and the first and last
/// days of its range and its own language in full (`locale_sample`).
#[test]
fn no_formatted_date_holds_a_raw_extra_field_or_era_code() {
    let registry = hyper_calendar::registry();
    let identifiers: BTreeSet<&str> = fields::TABLES
        .iter()
        .flat_map(|table| table.names.iter().map(|(id, _)| *id))
        .collect();
    let locales = locales();
    assert_eq!(
        locales.len(),
        LOCALES.len() + 2,
        "every locale's own tag parses"
    );
    let mut faults: BTreeSet<String> = BTreeSet::new();
    let mut unlabelled: BTreeSet<String> = BTreeSet::new();
    let mut rendered: BTreeSet<(&str, usize)> = BTreeSet::new();
    let mut checked = 0_usize;
    let mut converting = 0_usize;
    for meta in registry.metas() {
        let calendar = registry.get(meta.id).expect("registered");
        // One memo for the calendar's days, as the boundary opens one for
        // a call, so that a label does not recompute the months its date
        // was converted with.
        hyper_calendar::hc_core::memo::scope(|| {
            let dates: Vec<_> = sample_days(&meta)
                .into_iter()
                .filter_map(|day| Some((day, calendar.fixed_to_fields(day).ok()?)))
                .collect();
            converting += usize::from(!dates.is_empty());
            for (index, (day, fields)) in dates.iter().enumerate() {
                for extra in fields.extra.iter() {
                    if fields::label(&Locale::ROOT, extra.name).is_none() {
                        unlabelled.insert(format!("{} {}", meta.id.0, extra.name));
                    }
                }
                let ends = Some(*day) == meta.earliest || Some(*day) == meta.latest;
                for (position, requested) in locales.iter().enumerate() {
                    if !locale_sample::paired(index, dates.len(), position, locales.len(), ends) {
                        continue;
                    }
                    rendered.insert((meta.id.0, position));
                    let locale = label::locale_for(calendar, requested.as_ref());
                    let texts = [
                        label::date(calendar, fields, &locale),
                        label::label(calendar, fields, Unit::Era, &locale),
                        label::label(calendar, fields, Unit::Year, &locale),
                        label::label(calendar, fields, Unit::Month, &locale),
                        label::label(calendar, fields, Unit::Day, &locale),
                    ];
                    for text in &texts {
                        checked += 1;
                        if let Some(fault) = fault(calendar, fields, &locale, text, &identifiers) {
                            faults.insert(format!(
                                "{} {locale} {}: {fault}: {text:?}",
                                meta.id.0, day.0
                            ));
                        }
                    }
                }
            }
        });
    }
    assert!(checked > 100_000, "{checked}");
    // Every calendar that converts a sample day is rendered in every
    // locale, in either build.
    assert_eq!(rendered.len(), converting * locales.len());
    assert!(
        unlabelled.is_empty(),
        "extra fields without a label: {unlabelled:#?}"
    );
    assert!(
        faults.is_empty(),
        "{} unreadable texts: {faults:#?}",
        faults.len()
    );
}

/// A date marks an extra field as written exactly when its text depends
/// on the field — changing the field's value, up or down by one, or a flag
/// of 0 or 1 to the other, changes the text of the date that writes it and
/// of no other — or when the date writes a field whose value has the same
/// name in the calendar's own words, as the Burmese half *waning* writes
/// the phase *waning*. Every calendar is checked in every locale, so each
/// template that writes an extra — a notation's `{extra:FIELD}`, a
/// locale's `{sexagenary}` — is covered, and every flag.
#[test]
fn the_in_date_flag_says_whether_the_date_depends_on_the_field() {
    let registry = hyper_calendar::registry();
    let locales = locales();
    let mut wrong: BTreeSet<String> = BTreeSet::new();
    let mut written = 0_usize;
    let mut flags = 0_usize;
    for meta in registry.metas() {
        let calendar = registry.get(meta.id).expect("registered");
        let days = [meta.sample_day(Rd(739_886)), meta.sample_day(Rd(730_179))];
        // One memo for the calendar's days, as in the test above.
        hyper_calendar::hc_core::memo::scope(|| {
            for day in days {
                let Ok(fields) = calendar.fixed_to_fields(day) else {
                    continue;
                };
                for requested in &locales {
                    let locale = label::locale_for(calendar, requested.as_ref());
                    let (text, marked) = label::date_marking(calendar, &fields, &locale);
                    let own = |name: &str, value: i64| {
                        fields::own_value_name(&locale, meta.id, name, value)
                    };
                    for (index, extra) in fields.extra.iter().enumerate() {
                        // A flag steps to its other value, which ±1 leaves
                        // for one of 2 and −1 on the other side.
                        let flipped = matches!(extra.value, 0 | 1).then_some(1 - extra.value);
                        let depends = [extra.value + 1, extra.value - 1]
                            .into_iter()
                            .chain(flipped)
                            .any(|value| {
                                let mut changed = fields;
                                changed.extra.set(extra.name, value).expect("replaces");
                                label::date(calendar, &changed, &locale) != text
                            });
                        let shares_a_name = own(extra.name, extra.value).is_some_and(|name| {
                            fields.extra.iter().enumerate().any(|(other, written)| {
                                other != index
                                    && marked.contains(other)
                                    && own(written.name, written.value) == Some(name)
                            })
                        });
                        if flipped.is_some() {
                            flags += 1;
                        }
                        if marked.contains(index) {
                            written += 1;
                        }
                        if marked.contains(index) != (depends || shares_a_name) {
                            wrong.insert(format!(
                                "{} {locale} {} {}: marked {}, depends {depends}: {text:?}",
                                meta.id.0,
                                day.0,
                                extra.name,
                                marked.contains(index)
                            ));
                        }
                    }
                }
            }
        });
    }
    assert!(written > 100, "{written}");
    assert!(flags > 100, "{flags}");
    assert!(wrong.is_empty(), "{} wrong flags: {wrong:#?}", wrong.len());
}

/// Every extra field seen to hold 0 and 1 and nothing else is a flag with
/// named values, so that no line reads a bare 0 or 1: the fields seen on
/// every 61st day of 2000 to 2032, as each calendar samples it, and on the
/// days a flag turns: the first ten days of 1930, in the continuous week;
/// 1872's last day and 1873's first, around the adoption of the imperial
/// year; and 20 December 2026 to 10 January 2027 and 10 to 20 June 2028,
/// around the perennial calendars' days outside the week.
#[test]
fn every_flag_reads_as_words() {
    use hyper_calendar::hc_i18n::fields::ValueNames;
    use std::collections::BTreeMap;

    let registry = hyper_calendar::registry();
    let mut seen: BTreeMap<(&str, &str), BTreeSet<i64>> = BTreeMap::new();
    for meta in registry.metas() {
        let calendar = registry.get(meta.id).expect("registered");
        let turning = (704_553..704_563)
            .chain(683_734..683_736)
            .chain(739_970..739_992)
            .chain(740_508..740_518);
        for day in (730_120..742_120).step_by(61).chain(turning) {
            let Ok(fields) = calendar.fixed_to_fields(meta.sample_day(Rd(day))) else {
                continue;
            };
            for extra in fields.extra.iter() {
                seen.entry((meta.id.0, extra.name))
                    .or_default()
                    .insert(extra.value);
            }
        }
    }
    let flags: Vec<_> = seen
        .into_iter()
        .filter(|(_, values)| {
            values.len() == 2 && values.iter().all(|value| matches!(value, 0 | 1))
        })
        .map(|(key, _)| key)
        .collect();
    assert!(flags.len() >= 12, "{flags:?}");
    for (id, field) in flags {
        let named = fields::values_of(CalendarId(id), field)
            .unwrap_or_else(|| panic!("{id} {field} is a flag without names"));
        assert!(
            matches!(
                named.names,
                ValueNames::Flag | ValueNames::Own(_) | ValueNames::Localized { .. }
            ),
            "{id} {field}"
        );
    }
}

/// The lines list each extra field once, labelled, with its value named
/// where it is named, and say which of them the date already writes.
#[test]
fn every_extra_field_has_a_labelled_line() {
    let registry = hyper_calendar::registry();
    let day = Rd(739_886); // 2026-09-27
    let text = lines::day_extras(&registry, day, None, "en").expect("every calendar");
    let rows: Vec<Vec<&str>> = text
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert!(
        rows.iter()
            .all(|row| row.len() == lines::DAY_EXTRAS_COLUMNS)
    );
    let expected: usize = registry
        .metas()
        .filter_map(|meta| registry.get(meta.id)?.fixed_to_fields(day).ok())
        .map(|fields| fields.extra.len())
        .sum();
    assert_eq!(rows.len(), expected);
    assert!(
        rows.iter()
            .all(|row| !row[3].is_empty() && !row[4].is_empty())
    );
    let row = |id: &str, field: &str| {
        rows.iter()
            .find(|row| row[0] == id && row[1] == field)
            .unwrap_or_else(|| panic!("{id} {field}"))
            .clone()
    };
    // The Tamil year's name is in the date; the Tiruvalluvar year is not.
    assert_eq!(
        row("hindu-solar-tamil", "samvatsara"),
        [
            "hindu-solar-tamil",
            "samvatsara",
            "40",
            "Samvatsara (southern reckoning)",
            "Parabhava",
            "1",
            "en"
        ]
    );
    // The amānta and pūrṇimānta calendars name the same year in the two
    // cycles, under two keys and two labels: Parabhava, the 40th, in the
    // southern reckoning, and Siddharthin, the 53rd, in the northern.
    assert_eq!(
        row("hindu-lunar", "samvatsara")[2..6],
        ["40", "Samvatsara (southern reckoning)", "Parabhava", "1"]
    );
    assert_eq!(
        row("hindu-lunar-purnimanta", "barhaspatya-samvatsara")[2..6],
        [
            "53",
            "Barhaspatya samvatsara (northern cycle)",
            "Siddharthin",
            "1"
        ]
    );
    assert!(
        rows.iter()
            .all(|row| !(row[0] == "hindu-lunar-purnimanta" && row[1] == "samvatsara"))
    );
    assert_eq!(
        row("hindu-solar-tamil", "tiruvalluvar-year")[2..6],
        ["2057", "Tiruvalluvar year", "2057", "0"]
    );
    // The Long Count's places are its date; a day count's Julian Day
    // Number is not.
    assert_eq!(
        row("maya-longcount", "baktun")[2..6],
        ["13", "Baktun", "13", "1"]
    );
    assert_eq!(
        row("modified-julian-day", "julian-day-number")[2..6],
        ["2461311", "Julian Day Number", "2461311", "0"]
    );
    // A value that counts a named cycle is written by its name.
    assert_eq!(
        row("maya-tzolkin", "tzolkin_name")[2..6],
        ["8", "Tzolkʼin day sign", "Lamat", "1"]
    );
    // CLDR's root writes the stem and branch joined, bing-wu; the English
    // Chinese date writes them, "Eighth Month 17, 2026(bing-wu)".
    assert_eq!(
        row("chinese", "sexagenary_year")[3..6],
        ["Stem and branch of the year", "bing-wu", "1"]
    );
    assert_eq!(
        row("chinese", "related-gregorian-year")[2..6],
        ["2026", "Related Gregorian year", "2026", "1"]
    );
    // The Chinese-family dates write the year's stem and branch, 丙午年八月十七
    // and 병오년 8월 17일, and say so.
    for (id, tag) in [
        ("chinese", "zh-Hans"),
        ("chinese", "zh-Hant"),
        ("chinese", "ja"),
        ("chinese", "ko"),
        ("dangi", "ko"),
        ("vietnamese", "zh-Hans"),
    ] {
        let lines = lines::day_extras(&registry, day, Some(id), tag).expect("registered");
        let sexagenary = lines
            .lines()
            .find(|line| line.split('\t').nth(1) == Some("sexagenary_year"))
            .unwrap_or_else(|| panic!("{id} {tag}"));
        assert_eq!(
            sexagenary.split('\t').nth(5),
            Some("1"),
            "{id} {tag}: {sexagenary}"
        );
    }
    // The Burmese and Khmer dates write the half of the month.
    let burmese = lines::day_extras(&registry, day, Some("burmese"), "my").expect("registered");
    assert!(
        burmese.contains("\twaning\t1\tHalf of the month\tလဆုတ်\t1\tmy\n"),
        "{burmese}"
    );
    assert_eq!(
        row("symmetry454", "day-of-week")[3..6],
        ["Day of the week", "Sunday", "0"]
    );
    // In Tamil the year's name is the Tamil Lexicon's; no Tamil source read
    // writes a date with it, so the Tamil date does not.
    let tamil =
        lines::day_extras(&registry, day, Some("hindu-solar-tamil"), "ta").expect("registered");
    assert!(
        tamil.starts_with(
            "hindu-solar-tamil\tsamvatsara\t40\tSamvatsara (southern reckoning)\tபராபவ\t0\tta\n"
        ),
        "{tamil}"
    );
    assert!(tamil.lines().all(|line| line.ends_with("\tta")), "{tamil}");
    // A calendar that refuses the day writes nothing.
    let rumi = lines::day_extras(&registry, day, Some("rumi"), "en").expect("registered");
    assert_eq!(rumi, "");
    assert!(lines::day_extras(&registry, day, Some("no-such-calendar"), "en").is_err());
}

/// The sources' written forms, on the days they give or on 2026-09-27.
#[test]
fn the_extras_a_source_writes_are_in_the_date() {
    let registry = hyper_calendar::registry();
    let formatted = |id: &str, day: i64, tag: &str| {
        let calendar = registry.get_by_name(id).expect(id);
        let fields = calendar.fixed_to_fields(Rd(day)).expect("converts");
        let locale = lines::locale_for(calendar, tag);
        label::date(calendar, &fields, &locale)
    };
    let today = 739_886; // 2026-09-27
    // ISO 8601: 2021-01-01 is 2021-W53-5 of ISO year 2020, day 1 of 2021.
    assert_eq!(formatted("iso8601-week", 737_791, "en"), "2020-W53-5");
    assert_eq!(formatted("iso8601-ordinal", 737_791, "ja"), "2021-001");
    assert_eq!(formatted("stata-week", today, "en"), "2026w39");
    // The Long Count's end of the thirteenth baktun, 21 December 2012.
    assert_eq!(formatted("maya-longcount", 734_858, "en"), "13.0.0.0.0");
    assert_eq!(formatted("maya-round", 734_858, "en"), "4 Ahau 3 Kankin");
    assert_eq!(formatted("maya-round", today, "de"), "1 Lamat 1 Yax");
    assert_eq!(formatted("maya-tzolkin", today, "en"), "1 Lamat");
    assert_eq!(formatted("maya-haab", today, "en"), "1 Yax");
    assert_eq!(formatted("aztec-tonalpohualli", today, "en"), "1 Tochtli");
    assert_eq!(formatted("javanese-pasaran", today, "en"), "Ahad Pahing");
    assert_eq!(
        formatted("balinese-pawukon", today, "en"),
        "Redite Paing Ugu"
    );
    assert_eq!(formatted("akan", today, "en"), "Nwuna-Kwasi");
    assert_eq!(formatted("sexagenary", today, "ja"), "甲辰");
    assert_eq!(formatted("thai-lunar", today, "th"), "แรม 1 ค่ำ เดือนสิบ 2569");
    assert_eq!(formatted("lao", today, "en"), "ເດືອນສິບ ແຮມ 1 ຄ່ຳ ປີ 1388");
    assert_eq!(formatted("javanese", today, "en"), "14 Bakda Mulud 1960 Be");
    assert_eq!(formatted("zapotec-yza", today, "en"), "17 zohuao, 4 biaa");
    assert_eq!(formatted("meyer-palmen", today, "en"), "102-52-07-17 MP");
    assert_eq!(formatted("yerm", today, "en"), "21-23(10(16");
    assert_eq!(
        formatted("liberalia-triday-solar", today, "en"),
        "122-3-04-1 SLT"
    );
    assert_eq!(
        formatted("liberalia-triday-lunar", today, "en"),
        "0-126-04-06-1 LLT"
    );
    assert_eq!(
        formatted("week-and-month", today, "en"),
        "Sunday Delta September 2026"
    );
    assert_eq!(
        formatted("hindu-solar-tamil", today, "en"),
        "Purattasi 11 of the year Parabhava in southern reckoning, 1948 Saka"
    );
    // The same year in the north: the pūrṇimānta month is a fortnight on,
    // and the year is named in the Bārhaspatya cycle.
    assert_eq!(
        formatted("hindu-lunar", today, "en"),
        "Bhadra 16 of the year Parabhava in southern reckoning, 1948 Saka"
    );
    assert_eq!(
        formatted("hindu-lunar-purnimanta", today, "en"),
        "Asvina 16 of the Barhaspatya year Siddharthin, 1948 Saka"
    );
    assert_eq!(
        formatted("odia-anka", today, "en"),
        "Asvina 16, 71 Anka 1434"
    );
    assert_eq!(formatted("juche", today, "ko"), "주체115(2026)년 9월 27일");
    assert_eq!(
        formatted("juche", today, "en"),
        "September 27, Juche 115 (2026)"
    );
    // The Burmese date by its month, half and day: in English as
    // `docs/systems/burmese.md` writes one, in Burmese as Wikipedia's
    // example of 29 March 2017, «၁၃၇၈ ခုနှစ်၊ နှောင်းတန်ခူးလဆန်း ၂ ရက်».
    assert_eq!(
        formatted("burmese", today, "en"),
        "Tawthalin waning 1, 1388 ME"
    );
    assert_eq!(
        formatted("burmese", today, "my"),
        "၁၃၈၈ ခုနှစ်၊ တော်သလင်းလဆုတ် ၁ ရက်"
    );
    assert_eq!(
        formatted("burmese", 736_417, "my"),
        "၁၃၇၈ ခုနှစ်၊ တန်ခူးလဆန်း ၂ ရက်"
    );
    // The Khmer date in Tum's order, the printed year after it.
    assert_eq!(formatted("khmer", today, "en"), "1 roaj Phôtrôbât 2570 BE");
    // CLDR 48's long forms: the Chinese "MMMM d, r(U)", the Hebrew
    // "d MMMM y", and the Minguo era under zh-Hant, 民國 and 民國前.
    assert_eq!(
        formatted("chinese", today, "en"),
        "Eighth Month 17, 2026(bing-wu)"
    );
    assert_eq!(formatted("hebrew", today, "en"), "16 Tishri 5787");
    assert_eq!(formatted("roc", today, "zh-Hant"), "民國115年9月27日");
    assert_eq!(formatted("roc", 697_000, "zh-Hant"), "民國前3年4月28日");
    assert_eq!(formatted("yazidi", today, "en"), "6776, day 166");
    assert_eq!(
        formatted("olympiad", today, "en"),
        "September 14, 2026, year 2 of Olympiad 701"
    );
    // Metadata stays out: a day count is its number, and the Masonic year
    // does not write the Gregorian year it counts on.
    assert_eq!(formatted("modified-julian-day", today, "en"), "61310");
    assert_eq!(
        formatted("masonic-anno-lucis", today, "en"),
        "September 27, 6026 A.L."
    );
}

/// Every notation and every named field is keyed to a registered
/// calendar, a calendar has one notation at most, and a field named by a
/// cycle names one the calendar declares.
#[test]
fn notations_and_named_fields_are_keyed_to_the_registry() {
    use hyper_calendar::hc_i18n::fields::ValueNames;
    use hyper_calendar::hc_i18n::notation::NOTATIONS;

    let registry = hyper_calendar::registry();
    let mut noted = BTreeSet::new();
    for notation in NOTATIONS {
        assert!(!notation.source.is_empty());
        for id in notation.calendars {
            assert!(
                registry.get(*id).is_some(),
                "{} has a notation and is not registered",
                id.0
            );
            assert!(noted.insert(id.0), "{} has two notations", id.0);
        }
    }
    for named in fields::VALUES {
        for id in named.calendars {
            let calendar = registry
                .get(*id)
                .unwrap_or_else(|| panic!("{} names a field and is not registered", id.0));
            assert!(
                named.field == "day" || fields::label(&Locale::ROOT, named.field).is_some(),
                "{} {} has no label",
                id.0,
                named.field
            );
            if let ValueNames::Cycle(kind) = named.names {
                assert!(
                    calendar.cycles().iter().any(|cycle| cycle.kind == kind),
                    "{} does not declare {kind}",
                    id.0
                );
            }
        }
    }
}
