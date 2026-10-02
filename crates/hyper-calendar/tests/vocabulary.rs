//! The registry and the locale data must agree, and this is where that is
//! checked — the only place both are in scope.
//!
//! # What can go wrong
//!
//! Locale data can disagree with a calendar in two ways that no test inside
//! `hc-i18n` can see:
//!
//! * **The key.** A name list keyed to an identifier the registry does not
//!   have is written, tested and unreachable. The registry calls the
//!   arithmetic Solar Hijri calendar `persian-arithmetic` and the tabular
//!   Hijri `islamic-civil`, and an entry under a near miss answers for
//!   nothing.
//! * **The length.** Assuming the Gregorian shape — twelve or thirteen
//!   months, seven weekdays — rejects correct data for anything else. The
//!   Badíʿ calendar has nineteen months and the French Republican décade has
//!   ten days, Primidi through Décadi.
//!
//! Every calendar therefore declares its own cycles (`hc_calendar::shape` —
//! the trait method has no default, so a calendar that does not declare
//! does not compile), the vocabulary is keyed to real [`CalendarId`]s and to
//! those cycles by name, and the assertions below make any disagreement a
//! test failure rather than a discovery.
//!
//! # Why the coverage number is asserted
//!
//! The last test states how many registered calendars have month names in
//! English. That is a gap, and asserting it means it can only change
//! deliberately — downward when someone adds data, and never upward by
//! accident. A gap nobody can see is the thing this whole file exists to
//! prevent.

#![cfg(all(
    feature = "alloc",
    feature = "civil",
    feature = "lunar",
    feature = "regional",
    feature = "i18n",
))]

use std::collections::{BTreeMap, BTreeSet};

use hyper_calendar::hc_calendar::shape::{CycleShape, MONTH};
use hyper_calendar::hc_calendar::{CalendarId, CalendarMeta, CalendarRegistry, Rd};
use hyper_calendar::hc_i18n::data::LOCALES;

#[cfg(feature = "format")]
mod locale_sample;

/// Every calendar the enabled features register.
fn registry() -> CalendarRegistry {
    hyper_calendar::registry()
}

/// Every identifier the registry answers to.
///
/// `CalendarId` is `Copy` over a `&'static str`, so these can be used as
/// lookup keys directly — no leaking, no allocation.
fn registered() -> Vec<CalendarId> {
    registry().metas().map(|meta| meta.id).collect()
}

/// The cycles each registered calendar declares.
fn declared_cycles() -> BTreeMap<String, &'static [CycleShape]> {
    let registry = registry();
    let ids: Vec<String> = registry.metas().map(|meta| meta.id.to_string()).collect();
    let mut out = BTreeMap::new();
    for id in ids {
        let Some(calendar) = registry.get_by_name(&id) else {
            continue;
        };
        out.insert(id, calendar.cycles());
    }
    out
}

/// A shape must be well formed: no cycle declared twice, none with no
/// positions, and a calendar's own names as many as the cycle has
/// positions. The compiler guarantees a shape exists; this is what it
/// cannot guarantee about its contents.
#[test]
fn every_declared_shape_is_well_formed() {
    for (id, shapes) in declared_cycles() {
        for (index, cycle) in shapes.iter().enumerate() {
            assert!(
                cycle.length.maximum() > 0,
                "{id}: {} has no positions",
                cycle.kind
            );
            assert!(
                !shapes[index + 1..]
                    .iter()
                    .any(|other| other.kind == cycle.kind),
                "{id}: declares {} twice",
                cycle.kind
            );
            if !cycle.names.is_empty() {
                let count = u16::try_from(cycle.names.len()).unwrap_or(u16::MAX);
                assert!(
                    cycle.length.accepts(count),
                    "{id}: {} names {count} positions but declares {:?}",
                    cycle.kind,
                    cycle.length
                );
                for name in cycle.names {
                    assert!(!name.is_empty(), "{id}: {} has an empty name", cycle.kind);
                }
            }
        }
    }
}

/// A day inside a calendar's range, for probing how it lays out its fields.
fn sample_day(meta: &CalendarMeta) -> Rd {
    let recent = Rd(738_000);
    if meta.supports(recent) {
        return recent;
    }
    match (meta.earliest, meta.latest) {
        (Some(first), Some(last)) => Rd(first.0.midpoint(last.0)),
        (Some(first), None) => Rd(first.0 + 400),
        (None, Some(last)) => Rd(last.0 - 400),
        (None, None) => recent,
    }
}

/// The `month` field and the `month` cycle are one claim made twice, so
/// they must agree: a calendar whose dates carry a month declares one, and
/// a calendar whose dates carry none declares none.
///
/// This is what caught the ISO week and ordinal calendars declaring twelve
/// months while their fields held a week or a day of the year.
#[test]
fn a_calendar_declares_a_month_cycle_exactly_when_its_dates_carry_a_month() {
    let registry = registry();
    let ids: Vec<CalendarId> = registry.metas().map(|meta| meta.id).collect();
    for id in ids {
        let calendar = registry.get(id).expect("registered");
        let meta = calendar.meta();
        let day = sample_day(&meta);
        let fields = calendar
            .fixed_to_fields(day)
            .unwrap_or_else(|error| panic!("{}: {day} should be in range: {error}", id.0));
        let declares_month = calendar.cycles().iter().any(|cycle| cycle.kind == MONTH);
        assert_eq!(
            fields.month.is_some(),
            declares_month,
            "{}: the month field and the month cycle disagree",
            id.0
        );
    }
}

/// No vocabulary may name a calendar the registry does not have.
///
/// An entry keyed to a near miss of a registry identifier fails here on
/// the day it is written.
#[test]
fn every_calendar_a_locale_names_is_one_the_registry_answers_to() {
    let registered = registered();
    let known = |id: CalendarId| registered.contains(&id);
    let mut unknown: BTreeSet<String> = BTreeSet::new();
    for locale in LOCALES {
        for entry in locale.calendars {
            for id in entry.calendars {
                if !known(*id) {
                    unknown.insert(format!("{} names {}", locale.tag, id.0));
                }
            }
            for id in entry.eras.calendars {
                if !known(*id) {
                    unknown.insert(format!("{} names {} in its eras", locale.tag, id.0));
                }
            }
        }
    }
    assert!(
        unknown.is_empty(),
        "locale data names calendars that are not registered: {unknown:?}"
    );
}

/// An era code a calendar puts in its dates must be the code the locale
/// data keys the era's name by, or the name is written and unreachable.
///
/// Both sides use lowercase codes (`saka`, `am`, `ah`). The Indian
/// national calendar and the Hindu lunisolar calendar once declared
/// `"Saka"` while every locale keyed `"saka"`, and no Saka era was ever
/// named; this is the test that would have caught it.
#[test]
fn every_era_code_a_calendar_writes_is_one_the_locales_key() {
    use hyper_calendar::hc_i18n::Locale;
    use hyper_calendar::hc_i18n::names::{NameWidth, era_codes, era_name_by_code};

    let registry = registry();
    let mut unmatched: BTreeSet<String> = BTreeSet::new();
    for id in registered() {
        let calendar = registry.get(id).expect("registered");
        let day = sample_day(&calendar.meta());
        let Ok(fields) = calendar.fixed_to_fields(day) else {
            continue;
        };
        let Some(code) = fields.era else { continue };
        assert_eq!(
            code,
            code.to_ascii_lowercase(),
            "{}: era codes are lowercase",
            id.0
        );
        // A locale may name only some of a calendar's eras, as `fa.xml`
        // names two of the Japanese ones; its list then holds codes the
        // calendar lists, and a code outside them is the mismatch.
        let listed: Vec<&str> = (0..).map_while(|index| calendar.era_code(index)).collect();
        for locale in LOCALES {
            let tag: Locale = locale.tag.parse().expect("a locale's own tag parses");
            if let Some(codes) = era_codes(&tag, id)
                && !codes.contains(&code)
                && (listed.is_empty() || !codes.iter().all(|own| listed.contains(own)))
            {
                unmatched.insert(format!("{} {}: {code} not in {codes:?}", locale.tag, id.0));
            }
        }
    }
    assert!(
        unmatched.is_empty(),
        "era codes the locales do not key: {unmatched:?}"
    );

    // The two Saka-era calendars by name: the national calendar in Hindi,
    // which keys its era; the lunisolar calendar in English, which keys its
    // own; and the lunisolar calendar in Hindi, which names the era
    // through the national calendar's vocabulary, the shared era's rule.
    let hindi: Locale = "hi".parse().expect("hi is a locale");
    let english: Locale = "en".parse().expect("en is a locale");
    for (calendar, locale, name) in [
        ("indian", &hindi, "शक"),
        ("hindu-lunar", &english, "Saka"),
        ("hindu-lunar", &hindi, "शक"),
    ] {
        let id = CalendarId(calendar);
        let fields = registry
            .get(id)
            .expect("registered")
            .fixed_to_fields(Rd(740_000))
            .expect("in range");
        assert_eq!(fields.era, Some("saka"), "{calendar}");
        assert_eq!(
            era_name_by_code(locale, id, "saka", NameWidth::Wide),
            Some(name),
            "{calendar}"
        );
    }
}

/// The days an era test probes: sixty-five spread over the calendar's
/// range, or over two thousand years either side of the present for an
/// unbounded one, so that a calendar with more than one era shows them.
fn era_probe_days(meta: &CalendarMeta) -> Vec<Rd> {
    let first = meta.earliest.unwrap_or(Rd(738_000 - 730_000));
    let last = meta.latest.unwrap_or(Rd(738_000 + 730_000));
    let (first, last) = (first.0.max(-10_000_000), last.0.min(10_000_000));
    (0..=64)
        .map(|step| Rd(first + (last - first) / 64 * step))
        .collect()
}

/// Every era code a registered calendar writes is lowercase kebab-case —
/// `nepal-sambat`, `kali-yuga`, never `Nepal Sambat` — and English can
/// name it, from the locale data or from the calendar's own table; and a
/// code the calendar names itself is one its `era_code` lists, so that
/// `hc_format::label::parse_date` can take the name back to it.
///
/// The lookup ignores case and nothing else, so a code with a space or a
/// capital that the name tables spell with a hyphen is a name written and
/// never reached; this holds the whole registry to the one spelling.
#[test]
fn every_era_code_is_kebab_case_and_english_names_it() {
    use hyper_calendar::hc_i18n::Locale;
    use hyper_calendar::hc_i18n::names::{NameWidth, era_name_by_code};

    let english: Locale = "en".parse().expect("en is a locale");
    let registry = registry();
    let mut malformed: BTreeSet<String> = BTreeSet::new();
    let mut unnamed: BTreeSet<String> = BTreeSet::new();
    let mut unlisted: BTreeSet<String> = BTreeSet::new();
    let mut seen = 0;
    for id in registered() {
        let calendar = registry.get(id).expect("registered");
        for day in era_probe_days(&calendar.meta()) {
            let Ok(fields) = calendar.fixed_to_fields(day) else {
                continue;
            };
            let Some(code) = fields.era else { continue };
            seen += 1;
            let kebab = !code.is_empty()
                && !code.starts_with('-')
                && !code.ends_with('-')
                && code
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-');
            if !kebab {
                malformed.insert(format!("{}: {code:?}", id.0));
            }
            let named = era_name_by_code(&english, id, code, NameWidth::Wide).is_some()
                || calendar.era_name(code).is_some();
            if !named {
                unnamed.insert(format!("{}: {code}", id.0));
            }
            let listed = || {
                (0..)
                    .map_while(|index| calendar.era_code(index))
                    .any(|own| own == code)
            };
            if calendar.era_name(code).is_some() && !listed() {
                unlisted.insert(format!("{}: {code}", id.0));
            }
        }
    }
    assert!(
        unlisted.is_empty(),
        "era codes a calendar names but does not list: {unlisted:?}"
    );
    assert!(seen > 0, "some calendar writes an era");
    assert!(
        malformed.is_empty(),
        "era codes that are not lowercase kebab-case: {malformed:?}"
    );
    assert!(
        unnamed.is_empty(),
        "era codes English cannot name: {unnamed:?}"
    );
}

/// No vocabulary may name a cycle the calendar does not have.
#[test]
fn every_named_cycle_is_one_the_calendar_declares() {
    let declared = declared_cycles();
    let mut wrong: Vec<String> = Vec::new();
    for locale in LOCALES {
        for entry in locale.calendars {
            for cycle in entry.cycles {
                for id in entry.calendars {
                    let Some(shapes) = declared.get(id.0) else {
                        // Not registered under the enabled features; the
                        // first test reports that.
                        continue;
                    };
                    if !shapes.iter().any(|shape| shape.kind == cycle.kind) {
                        wrong.push(format!(
                            "{}: {} has no {} cycle but is given {} names for one",
                            locale.tag,
                            id.0,
                            cycle.kind,
                            cycle
                                .names
                                .get(
                                    hyper_calendar::hc_i18n::names::NameWidth::Wide,
                                    hyper_calendar::hc_i18n::names::NameContext::Format
                                )
                                .len()
                        ));
                    }
                }
            }
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

/// A cycle's names must be as many as the calendar says its cycle has.
///
/// The old test asserted twelve or thirteen for everything. This asserts
/// what each calendar actually declares, so nineteen Badíʿ months and ten
/// Republican décade days are correct rather than rejected.
#[test]
fn every_name_list_is_as_long_as_the_cycle_it_names() {
    use hyper_calendar::hc_i18n::names::{NameContext, NameWidth};
    let declared = declared_cycles();
    for locale in LOCALES {
        for entry in locale.calendars {
            for cycle in entry.cycles {
                for id in entry.calendars {
                    let Some(shapes) = declared.get(id.0) else {
                        continue;
                    };
                    let Some(shape) = shapes.iter().find(|shape| shape.kind == cycle.kind) else {
                        continue;
                    };
                    for context in [NameContext::Format, NameContext::Standalone] {
                        for width in NameWidth::ALL {
                            let names = cycle.names.get(width, context);
                            if names.is_empty() {
                                continue;
                            }
                            let count = u16::try_from(names.len()).unwrap_or(u16::MAX);
                            assert!(
                                shape.length.accepts(count),
                                "{}: {} has {count} {} names but declares {:?}",
                                locale.tag,
                                id.0,
                                cycle.kind,
                                shape.length
                            );
                        }
                    }
                }
            }
        }
    }
}

/// What is still missing, stated as numbers so they can only move on
/// purpose.
///
/// The gap is the calendars that have months and whose months English
/// cannot name — neither from the locale nor from the calendar's own
/// names. It was not visible before, because the data model could not have
/// held the answer.
#[test]
fn the_vocabulary_gap_is_measured_and_not_growing() {
    use hyper_calendar::hc_i18n::Locale;
    use hyper_calendar::hc_i18n::names::{NameContext, NameWidth, position_name};

    let registry = registry();
    let registered = registered();
    let english: Locale = "en".parse().expect("en is a locale");

    let month_cycle = |id: CalendarId| {
        registry
            .get(id)
            .and_then(|calendar| calendar.cycles().iter().find(|cycle| cycle.kind == MONTH))
    };
    let with_months: Vec<CalendarId> = registered
        .iter()
        .copied()
        .filter(|id| month_cycle(*id).is_some())
        .collect();
    let unnamed: Vec<&str> = with_months
        .iter()
        .copied()
        .filter(|id| {
            let cycle = month_cycle(*id).expect("has a month cycle");
            position_name(
                &english,
                *id,
                cycle,
                0,
                NameWidth::Wide,
                NameContext::Format,
            )
            .is_none()
        })
        .map(|id| id.0)
        .collect();

    assert_eq!(
        registered.len(),
        225,
        "the registry changed; update the coverage numbers deliberately"
    );
    assert_eq!(
        with_months.len(),
        191,
        "calendars with a month cycle — changes only when a calendar's shape does"
    );
    assert!(
        unnamed.is_empty(),
        "calendars whose months English cannot name — a calendar with months \
         declares their names with its shape, or a locale supplies them: {unnamed:?}"
    );
}

/// The two Palmen calendars under the locale fallback: a locale that does
/// not name the Yerm months falls back to English, which numbers them as
/// Palmen does; nobody's locale names the Meyer–Palmen months, so the
/// requested locale is kept and the names come from the calendar's shape.
#[test]
fn the_palmen_calendars_are_named_through_the_fallback() {
    use hyper_calendar::hc_i18n::Locale;
    use hyper_calendar::hc_i18n::names::{
        NameContext, NameWidth, english, locale_for_calendar, position_name,
    };

    let registry = registry();
    let japanese: Locale = "ja".parse().expect("ja is a locale");
    for (id, expected_locale, first, last) in [
        ("yerm", english(), "Month 1", "Month 17"),
        ("meyer-palmen", japanese, "Aristarchus", "Meton"),
    ] {
        let calendar = registry.get_by_name(id).expect("registered");
        let meta = calendar.meta();
        let locale = locale_for_calendar(Some(&japanese), &meta);
        assert_eq!(locale, expected_locale, "{id}");
        let months = calendar
            .cycles()
            .iter()
            .find(|cycle| cycle.kind == MONTH)
            .expect("a month cycle");
        let name = |index| {
            position_name(
                &locale,
                meta.id,
                months,
                index,
                NameWidth::Wide,
                NameContext::Format,
            )
        };
        let top = usize::from(months.length.maximum()) - 1;
        assert_eq!((name(0), name(top)), (Some(first), Some(last)), "{id}");
    }
}

/// The Tamil year's name, from the extra field the calendar writes to the
/// name a locale gives it: Tamil script in Tamil, from the *Tamil Lexicon*,
/// and the calendar's own Sanskrit names, Sewell and Dikshit's, in English,
/// which has none of its own. The Tamil year of 2024–25 is the 38th.
#[cfg(feature = "indic")]
#[test]
fn the_tamil_year_is_named_in_tamil_and_through_the_fallback() {
    use hyper_calendar::hc_i18n::Locale;
    use hyper_calendar::hc_i18n::names::{NameContext, NameWidth, position_name};

    let registry = registry();
    let calendar = registry
        .get_by_name("hindu-solar-tamil")
        .expect("registered");
    let cycle = calendar
        .cycles()
        .iter()
        .find(|cycle| cycle.kind == "samvatsara")
        .expect("a samvatsara cycle");
    let tamil: Locale = "ta".parse().expect("ta is a locale");
    let english: Locale = "en".parse().expect("en is a locale");
    let name = |locale: &Locale, rd: Rd| {
        let fields = calendar.fixed_to_fields(rd).expect("in range");
        let position = fields.extra.get("samvatsara").expect("a year name");
        let index = usize::try_from(position - 1).expect("1 to 60");
        position_name(
            locale,
            calendar.meta().id,
            cycle,
            index,
            NameWidth::Wide,
            NameContext::Format,
        )
    };
    // 14 April 2024, Tamil New Year's Day, and the day before.
    let new_year = Rd(738_990);
    assert_eq!(name(&tamil, new_year), Some("குரோதி"));
    assert_eq!(name(&english, new_year), Some("Krodhin"));
    assert_eq!(name(&tamil, Rd(new_year.0 - 1)), Some("சோபகிருது"));
}

/// The Solar Hijri calendar as Afghanistan kept it, under the fallback: a
/// request for its own language finds Dari, the calendar's declared names,
/// through `fa-AF`; Pashto names the months itself, as CLDR 48 `ps.xml`
/// does; English carries Wikipedia's romanisation; and a locale that names
/// none of them falls back to English, not to Iran's month names.
#[cfg(feature = "equinox")]
#[test]
fn the_afghan_months_are_named_in_dari_pashto_and_english() {
    use hyper_calendar::hc_i18n::Locale;
    use hyper_calendar::hc_i18n::names::{
        NameContext, NameWidth, english, locale_for_calendar, position_name,
    };

    let registry = registry();
    let calendar = registry.get_by_name("persian-afghan").expect("registered");
    let meta = calendar.meta();
    let months = calendar
        .cycles()
        .iter()
        .find(|cycle| cycle.kind == MONTH)
        .expect("a month cycle");
    let name = |locale: &Locale, index| {
        position_name(
            locale,
            meta.id,
            months,
            index,
            NameWidth::Wide,
            NameContext::Format,
        )
    };
    let own = locale_for_calendar(None, &meta);
    assert_eq!(own.to_string(), "fa-AF");
    assert_eq!((name(&own, 0), name(&own, 11)), (Some("حمل"), Some("حوت")));
    let pashto: Locale = "ps".parse().expect("ps is a locale");
    assert_eq!(locale_for_calendar(Some(&pashto), &meta), pashto);
    assert_eq!(
        (name(&pashto, 0), name(&pashto, 11)),
        (Some("وری"), Some("کب"))
    );
    assert_eq!(
        (name(&english(), 0), name(&english(), 11)),
        (Some("Hamal"), Some("Hūt"))
    );
    let japanese: Locale = "ja".parse().expect("ja is a locale");
    assert_eq!(locale_for_calendar(Some(&japanese), &meta), english());
    // Iran's calendar keeps its own names in Persian.
    let persian = registry.get_by_name("persian").expect("registered");
    let farsi: Locale = "fa".parse().expect("fa is a locale");
    let iranian = persian
        .cycles()
        .iter()
        .find(|cycle| cycle.kind == MONTH)
        .expect("a month cycle");
    assert_eq!(
        position_name(
            &farsi,
            persian.meta().id,
            iranian,
            0,
            NameWidth::Wide,
            NameContext::Format
        ),
        Some("فروردین")
    );
}

/// `hc-i18n`'s almanac vocabulary and `hc-almanac`'s annotations must
/// agree, and this is the one place both are in scope: every term a table
/// names is one `hc-almanac` computes, every term it computes has a
/// Japanese name, and the Japanese and English names are `hc-almanac`'s
/// own names and readings — the mansions' English, its English naming —
/// so that the two copies cannot drift.
#[cfg(feature = "almanac")]
#[test]
fn the_almanac_vocabulary_names_what_hc_almanac_computes() {
    use hyper_calendar::hc_almanac::mansions::namings;
    use hyper_calendar::hc_almanac::nayin::namings as nayin_namings;
    use hyper_calendar::hc_almanac::{
        Combination, LowerRegister, Mansion, Nayin, NineStar, Rokuyo, SelectedDay, TwelveDirect,
    };
    use hyper_calendar::hc_i18n::almanac::{
        self, CHINESE_SIMPLIFIED, COMBINATION, CYCLES, ENGLISH, JAPANESE, LOWER_REGISTER, MANSION,
        NAYIN, NINE_STAR, ROKUYO, SELECTED_DAY, TWELVE_DIRECT, Term, VOCABULARIES,
    };

    let cycles: [(&str, Vec<&str>, Vec<&str>); 5] = [
        (
            ROKUYO,
            Rokuyo::ALL.iter().map(|r| r.japanese_name()).collect(),
            Rokuyo::ALL.iter().map(|r| r.romaji()).collect(),
        ),
        (
            MANSION,
            Mansion::all().iter().map(|m| m.japanese_name()).collect(),
            Mansion::all()
                .iter()
                .map(|m| m.name(&namings::ENGLISH))
                .collect(),
        ),
        (
            NINE_STAR,
            NineStar::ALL.iter().map(|s| s.japanese_name()).collect(),
            NineStar::ALL.iter().map(|s| s.romaji()).collect(),
        ),
        (
            TWELVE_DIRECT,
            TwelveDirect::ALL
                .iter()
                .map(|d| d.japanese_name())
                .collect(),
            TwelveDirect::ALL.iter().map(|d| d.romaji()).collect(),
        ),
        (
            NAYIN,
            Nayin::all().iter().map(|n| n.japanese_name()).collect(),
            Nayin::all().iter().map(|n| n.romaji()).collect(),
        ),
    ];
    for (kind, japanese, english) in &cycles {
        let length = CYCLES.iter().find(|(cycle, _)| cycle == kind).map(|c| c.1);
        assert_eq!(length, Some(japanese.len()), "{kind}");
        for (position, (ja, en)) in japanese.iter().zip(english).enumerate() {
            assert_eq!(JAPANESE.name_of(kind, Term::Position(position)), Some(*ja));
            assert_eq!(ENGLISH.name_of(kind, Term::Position(position)), Some(*en));
        }
    }
    let mut terms: Vec<(&str, &str, &str, Option<&str>)> = Vec::new();
    for entry in LowerRegister::ALL {
        terms.push((
            LOWER_REGISTER,
            entry.id,
            entry.japanese_name(),
            Some(entry.romaji()),
        ));
    }
    for entry in SelectedDay::ALL {
        terms.push((
            SELECTED_DAY,
            entry.id,
            entry.japanese_name(),
            Some(entry.romaji()),
        ));
    }
    for entry in Combination::ALL {
        terms.push((COMBINATION, entry.id, entry.japanese_name(), None));
    }
    for (kind, id, japanese, english) in &terms {
        assert_eq!(
            JAPANESE.name_of(kind, Term::Id(id)),
            Some(*japanese),
            "{id}"
        );
        assert_eq!(ENGLISH.name_of(kind, Term::Id(id)), *english, "{id}");
    }
    for table in VOCABULARIES {
        for (kind, id, _) in table.terms {
            assert!(
                terms.iter().any(|(k, i, _, _)| k == kind && i == id),
                "{} names {kind} {id}, which hc-almanac does not compute",
                table.tag
            );
        }
    }
    // Chinese names the 納音 alone, as hc-almanac's Chinese naming does.
    assert_eq!(CHINESE_SIMPLIFIED.cycles.len(), 1);
    for nayin in Nayin::all() {
        assert_eq!(
            CHINESE_SIMPLIFIED.name_of(NAYIN, Term::Position(usize::from(nayin.index()))),
            Some(nayin.name(&nayin_namings::CHINESE))
        );
    }
    assert_eq!(almanac::VOCABULARIES.len(), 3);
}

/// `hc-i18n`'s vocabulary of the other reckonings and the crates that
/// compute them must agree, as the almanac's must: every term a table
/// names is one the crates compute, and every term they compute is named
/// in its kind's own language with the crate's own name, so that the two
/// copies cannot drift.
#[cfg(all(feature = "almanac", feature = "indic", feature = "format"))]
#[test]
fn the_reckoning_vocabulary_names_what_the_crates_compute() {
    use hyper_calendar::hc_almanac::vietnamese_days::{NGUYET_KY_NAME, TAM_NUONG_NAME};
    use hyper_calendar::hc_calendars_indic::choghadiya::Choghadiya;
    use hyper_calendar::hc_calendars_indic::kalam::Kalam;
    use hyper_calendar::hc_calendars_indic::kumbh::KumbhYoga;
    use hyper_calendar::hc_calendars_indic::panchak::PanchakNaming;
    use hyper_calendar::hc_calendars_indic::pushkaram::PushkaramRiver;
    use hyper_calendar::hc_format::night_watches::WATCH_NAMES;
    use hyper_calendar::hc_i18n::reckonings::{
        CHOGHADIYA, FIRST_MONTH_COUNT, FOLK_HALF, FOLK_NAMED_DAY, KALAM, KUMBH_SITE, NIGHT_WATCH,
        PANCHAK, PLANET, PLUM_RAINS, PUSHKARAM_RIVER, VIETNAMESE_DAY, VOCABULARIES, native_tag,
        table,
    };
    use hyper_calendar::hc_seasons::hizir_kasim::{Half, NamedDay};
    use hyper_calendar::hc_seasons::meiyu::PlumRainRule;
    use hyper_calendar::hc_seasons::zodiac::RulingPlanet;
    use hyper_calendar::reckoning_lines::{FIRST_MONTH_COUNT_IDS, folk_named_day_id};

    let mut terms: Vec<(&str, String, &str)> = Vec::new();
    let mut add = |kind: &'static str, id: String, name: &'static str| {
        if !terms.iter().any(|(k, i, _)| *k == kind && *i == id) {
            terms.push((kind, id, name));
        }
    };
    for kind in Choghadiya::CYCLE {
        add(CHOGHADIYA, kind.id.to_owned(), kind.english_name);
    }
    for naming in PanchakNaming::ALL {
        for kind in naming.kind_by_weekday.into_iter().flatten() {
            add(PANCHAK, kind.to_ascii_lowercase(), kind);
        }
    }
    for yoga in KumbhYoga::ALL {
        add(KUMBH_SITE, yoga.site.to_ascii_lowercase(), yoga.site);
    }
    for river in PushkaramRiver::ALL {
        add(PUSHKARAM_RIVER, river.id.to_owned(), river.river);
    }
    for planet in RulingPlanet::CHALDEAN_ORDER {
        add(PLANET, planet.id.to_owned(), planet.english_name());
    }
    for period in Kalam::ALL {
        add(KALAM, period.id.to_owned(), period.english_name);
    }
    for (index, name) in WATCH_NAMES.iter().enumerate() {
        add(NIGHT_WATCH, (index + 1).to_string(), name);
    }
    add(VIETNAMESE_DAY, "tam-nuong".to_owned(), TAM_NUONG_NAME);
    add(VIETNAMESE_DAY, "nguyet-ky".to_owned(), NGUYET_KY_NAME);
    // `hc-seasons` and `hc-almanac` write these names in their
    // documentation only: 入梅 and 出梅 in `meiyu`, the counts in
    // `first_month_counts`.
    for (rule, name) in PlumRainRule::ALL.iter().zip(["入梅", "入梅", "出梅"]) {
        add(PLUM_RAINS, rule.id.to_owned(), name);
    }
    for (id, name) in
        FIRST_MONTH_COUNT_IDS
            .iter()
            .zip(["几龙治水", "几牛耕田", "几日得辛", "几人分饼"])
    {
        add(FIRST_MONTH_COUNT, (*id).to_owned(), name);
    }
    for (half, id) in [(Half::Hizir, "hizir"), (Half::Kasim, "kasim")] {
        add(FOLK_HALF, id.to_owned(), half.turkish_name());
    }
    for named in NamedDay::ALL {
        add(
            FOLK_NAMED_DAY,
            folk_named_day_id(named).to_owned(),
            named.turkish_name(),
        );
    }
    for (kind, id, name) in &terms {
        let own = native_tag(kind)
            .and_then(table)
            .expect("a kind with its own table");
        assert_eq!(own.name_of(kind, id), Some(*name), "{kind} {id}");
    }
    for vocabulary in VOCABULARIES {
        for (kind, id, _) in vocabulary.terms {
            assert!(
                terms.iter().any(|(k, i, _)| k == kind && i == id),
                "{} names {kind} {id}, which no crate computes",
                vocabulary.tag
            );
        }
    }
    assert_eq!(terms.len(), 7 + 5 + 4 + 14 + 7 + 3 + 5 + 2 + 3 + 4 + 2 + 7);
}

/// Every horizon `hc-i18n` names is one `hc-astro` carries, so that a name
/// keyed to a near miss of an identifier fails on the day it is written.
#[cfg(feature = "astro")]
#[test]
fn every_horizon_a_locale_names_is_one_hc_astro_carries() {
    use hyper_calendar::hc_astro::horizon::HORIZONS;
    use hyper_calendar::hc_i18n::horizons::TABLES;

    for table in TABLES {
        for (id, _) in table.names {
            assert!(
                HORIZONS.iter().any(|horizon| horizon.id == *id),
                "{} names {id}, which hc-astro does not carry",
                table.tag
            );
        }
    }
}

/// Every group `hc-i18n` names is one `hc-holiday` carries, and every group
/// a table's rules are given to is in `hc-holiday`'s catalogue, so that a
/// name keyed to a near miss of an identifier, or a group declared and not
/// catalogued, fails on the day it is written.
#[cfg(feature = "holiday")]
#[test]
fn every_group_a_locale_names_is_one_hc_holiday_carries() {
    use hyper_calendar::hc_holiday::group::{GROUPS, by_id};
    use hyper_calendar::hc_i18n::holiday_groups::TABLES;

    for table in TABLES {
        for (id, _) in table.names {
            assert!(
                GROUPS.iter().any(|group| group.id == *id),
                "{} names {id}, which hc-holiday does not carry",
                table.tag
            );
        }
    }
    for set in hyper_calendar::holiday_lines::tables() {
        for group in set.groups() {
            assert_eq!(by_id(group.id), Some(group), "{}", set.code);
        }
    }
    // Every catalogued group is given a day by some table.
    for group in GROUPS {
        assert!(
            hyper_calendar::holiday_lines::tables().any(|set| set.groups().contains(group)),
            "{} is given no day",
            group.id
        );
    }
}

/// Whether a rendered text holds `code` as a word.
fn holds_code(text: &str, code: &str) -> bool {
    text.split(|c: char| !(c.is_alphanumeric() || c == '-'))
        .any(|word| word == code)
}

/// The days the bare-era-code tests render in a calendar: the probe days,
/// and the first day of every era in their range, so that each era a
/// calendar writes there is rendered once, the hundreds of the regnal
/// calendars included.
fn era_render_days(calendar: &dyn hyper_calendar::hc_calendar::DynCalendar) -> Vec<Rd> {
    use hyper_calendar::hc_calendar::units::{Unit, units};

    let mut days = era_probe_days(&calendar.meta());
    let (first, last) = (days[0], days[days.len() - 1]);
    let eras = |day: &Rd| {
        calendar
            .fixed_to_fields(*day)
            .is_ok_and(|fields| fields.era.is_some())
    };
    if days.iter().any(eras) {
        days.extend(
            units(calendar, Unit::Era, first, Rd(last.0 + 1))
                .iter()
                .filter(|span| span.date().is_some())
                .map(|span| span.start),
        );
    }
    days.sort();
    days.dedup();
    days
}

/// An era code is an identifier and never reader-facing text: no date,
/// era, year, month or day label any calendar is written as, in any
/// carried locale or under `native`, holds a bare era code as a word, on
/// the first day of every era in the probed range and on the probe days.
///
/// The era's name follows one rule, `hc_i18n::names::era_label` — the
/// locale's, the shared era's, the calendar's own, English's — which ends
/// at a name; this is what caught Sanskrit writing the Śaka years of the
/// lunisolar calendars `saka`.
///
/// Every era's first day is rendered in either build, in every locale in a
/// release build and in a staggered share of them in a debug build, which
/// still renders every calendar in every locale and each day under
/// `native` (`locale_sample`).
#[cfg(feature = "format")]
#[test]
fn no_rendered_label_holds_a_bare_era_code() {
    use hyper_calendar::hc_calendar::units::Unit;
    use hyper_calendar::hc_format::label;
    use hyper_calendar::hc_i18n::Locale;

    let registry = registry();
    let locales: Vec<Locale> = LOCALES
        .iter()
        .map(|data| data.tag.parse().expect("a locale's own tag parses"))
        .collect();
    let mut leaks: BTreeSet<String> = BTreeSet::new();
    let mut codes: BTreeSet<(&str, &str)> = BTreeSet::new();
    let mut rendered: BTreeSet<(&str, usize)> = BTreeSet::new();
    let mut checked = 0;
    for id in registered() {
        let calendar = registry.get(id).expect("registered");
        // One memo for the calendar's days, as the boundary opens one for
        // a call, so that a label does not recompute the months its date
        // was converted with.
        hyper_calendar::hc_core::memo::scope(|| {
            let dates: Vec<_> = era_render_days(calendar)
                .into_iter()
                .filter_map(|day| {
                    let fields = calendar.fixed_to_fields(day).ok()?;
                    Some((fields, fields.era?))
                })
                .collect();
            for (index, (fields, code)) in dates.iter().enumerate() {
                let code = *code;
                codes.insert((id.0, code));
                let requested = locales.iter().map(Some).chain([None]);
                let count = locales.len() + 1;
                for (position, requested) in requested.enumerate() {
                    if !locale_sample::paired(index, dates.len(), position, count, false) {
                        continue;
                    }
                    rendered.insert((id.0, position));
                    let locale = label::locale_for(calendar, requested);
                    let texts = [
                        label::date(calendar, fields, &locale),
                        label::label(calendar, fields, Unit::Era, &locale),
                        label::label(calendar, fields, Unit::Year, &locale),
                        label::label(calendar, fields, Unit::Month, &locale),
                        label::label(calendar, fields, Unit::Day, &locale),
                    ];
                    checked += 1;
                    for text in &texts {
                        if holds_code(text, code) {
                            leaks.insert(format!("{} {locale}: {text:?}", id.0));
                        }
                    }
                }
            }
        });
    }
    assert!(checked > 1_000, "{checked}");
    // Every era of the regnal calendars is rendered, not only those the
    // sixty-five probe days fall in.
    assert!(codes.len() > 500, "{} era codes rendered", codes.len());
    // Every calendar that writes an era is rendered in every locale and
    // under `native`, in either build.
    let writing: BTreeSet<&str> = codes.iter().map(|(id, _)| *id).collect();
    assert_eq!(rendered.len(), writing.len() * (locales.len() + 1));
    assert!(
        leaks.is_empty(),
        "labels with an era code in them: {leaks:#?}"
    );
}

/// The boundary's era cells are names too: `describe_day`'s era label for
/// every calendar, in every carried locale and under `native`, on days
/// either side of the common era and today; and the Śaka and Vikrama
/// cells of `hc_hindu_lunar_date`'s line, the Vikrama one with a fallback
/// of its own rather than `era_label`, which are never empty and never a
/// code. `calendar_units`' labels are `label::label` of each unit's first
/// day, which the test above renders on the first day of every era.
#[cfg(feature = "format")]
#[test]
fn no_boundary_line_holds_a_bare_era_code() {
    let tags: Vec<&str> = LOCALES
        .iter()
        .map(|data| data.tag)
        .chain(["native"])
        .collect();
    let mut leaks: BTreeSet<String> = BTreeSet::new();
    let mut named = 0;
    let registry = registry();
    for day in [Rd(-365_000), Rd(1), Rd(739_887)] {
        // One memo for the day in every locale: each locale's lines convert
        // the same dates, which the scope converts once.
        hyper_calendar::hc_core::memo::scope(|| {
            for tag in &tags {
                let text = hyper_calendar::lines::describe_day(&registry, day, tag);
                for line in text.lines() {
                    let cells: Vec<&str> = line.split('\t').collect();
                    let (id, code, label) = (cells[0], cells[2], cells[3]);
                    if code.is_empty() {
                        continue;
                    }
                    named += 1;
                    if holds_code(label, code) {
                        leaks.insert(format!("describe_day {id} {tag} {}: {label:?}", day.0));
                    }
                }
            }
        });
    }
    assert!(named > 1_000, "{named} era cells");
    #[cfg(feature = "indic")]
    {
        use hyper_calendar::hc_calendars_indic::places::UJJAIN;
        use hyper_calendar::hindu_lines::{SURYA_SIDDHANTA, hindu_lunar_date_line};
        for sky in ["lahiri", SURYA_SIDDHANTA] {
            for tag in &tags {
                let line = hindu_lunar_date_line(sky, 739_887, UJJAIN, tag).expect("a date");
                let cells: Vec<&str> = line.trim_end().split('\t').collect();
                for (cell, code) in [(cells[9], "saka"), (cells[10], "vs")] {
                    if cell.is_empty() || holds_code(cell, code) {
                        leaks.insert(format!("hindu_lunar_date {sky} {tag}: {cell:?}"));
                    }
                }
            }
        }
    }
    assert!(
        leaks.is_empty(),
        "era cells with an era code in them: {leaks:#?}"
    );
}
