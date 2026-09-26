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
        for locale in LOCALES {
            let tag: Locale = locale.tag.parse().expect("a locale's own tag parses");
            if let Some(codes) = era_codes(&tag, id)
                && !codes.contains(&code)
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
    // which keys its era, and the lunisolar calendar in English, which
    // keys its own.
    let hindi: Locale = "hi".parse().expect("hi is a locale");
    let english: Locale = "en".parse().expect("en is a locale");
    for (calendar, locale, name) in [("indian", &hindi, "शक"), ("hindu-lunar", &english, "Saka")]
    {
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
        118,
        "the registry changed; update the coverage numbers deliberately"
    );
    assert_eq!(
        with_months.len(),
        98,
        "calendars with a month cycle — changes only when a calendar's shape does"
    );
    assert!(
        unnamed.is_empty(),
        "calendars whose months English cannot name — a calendar with months \
         declares their names with its shape, or a locale supplies them: {unnamed:?}"
    );
}
