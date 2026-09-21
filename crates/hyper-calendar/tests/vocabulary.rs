//! The registry and the locale data must agree, and this is where that is
//! checked — the only place both are in scope.
//!
//! # What this replaces
//!
//! Nothing. That was the problem.
//!
//! `hc-i18n` stored month names against a key and asserted that every month
//! list had twelve or thirteen entries. Neither half was checked against a
//! calendar:
//!
//! * The key was a bare string. `persian`, `islamic` and `iso8601` were
//!   written, tested and unreachable, because the registry calls those
//!   calendars `persian-arithmetic`, `islamic-civil` and `iso8601-week`.
//!   Twelve Persian month names sat in the table for anyone to read and no
//!   caller could ever get one.
//! * The length assertion was the Gregorian shape imposed on everything
//!   else. The Badíʿ calendar has nineteen months, so it could not be given
//!   names — not "had not been", *could not be*: supplying the correct data
//!   would have failed the test. The French Republican décade is ten days,
//!   and weekday names were stored against a seven-valued enum, so Primidi
//!   through Décadi had nowhere to live either.
//!
//! Both are now structural. A calendar declares its own cycles
//! (`hc_calendar::shape`), the vocabulary is keyed to real
//! [`CalendarId`]s and to those cycles by name, and the three assertions
//! below make any disagreement a test failure rather than a discovery.
//!
//! # Why the coverage numbers are asserted
//!
//! The last test states how many registered calendars have declared a shape
//! and how many have month names in English. Those are gaps, and asserting
//! them means they can only change deliberately — downward when someone
//! adds data, and never upward by accident. A gap nobody can see is the
//! thing this whole file exists to prevent.

#![cfg(all(
    feature = "alloc",
    feature = "civil",
    feature = "lunar",
    feature = "regional",
    feature = "i18n",
))]

use std::collections::{BTreeMap, BTreeSet};

use hyper_calendar::hc_calendar::{CalendarId, CalendarRegistry, shape::CycleShape};
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

/// The cycles each registered calendar declares, where it declares any.
fn declared_cycles() -> BTreeMap<String, &'static [CycleShape]> {
    let registry = registry();
    let ids: Vec<String> = registry.metas().map(|meta| meta.id.to_string()).collect();
    let mut out = BTreeMap::new();
    for id in ids {
        let Some(calendar) = registry.get_by_name(&id) else {
            continue;
        };
        if let Some(cycles) = calendar.cycles() {
            out.insert(id, cycles);
        }
    }
    out
}

/// No vocabulary may name a calendar the registry does not have.
///
/// This is the assertion that would have caught `persian`, `islamic` and
/// `iso8601` on the day they were written.
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
                        // The calendar has not declared a shape; the
                        // coverage test below reports that separately.
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

/// What is still missing, stated as numbers so it can only move on purpose.
///
/// These are the real gaps: most registered calendars have not declared a
/// shape, and almost none have month names outside the Gregorian family.
/// Neither was visible before — the first because nothing asked, the second
/// because the data model could not have held the answer.
#[test]
fn the_vocabulary_gap_is_measured_and_not_growing() {
    use hyper_calendar::hc_i18n::Locale;
    use hyper_calendar::hc_i18n::names::NameContext;
    use hyper_calendar::hc_i18n::names::month_count;

    let registered = registered();
    let declared = declared_cycles();
    let english: Locale = "en".parse().expect("en is a locale");

    let with_shape = declared.len();
    let named = |id: CalendarId| month_count(&english, id, NameContext::Format).is_some();
    let with_months = registered.iter().copied().filter(|id| named(*id)).count();

    // A calendar with month names must have declared a shape, because
    // otherwise nothing checks the names against anything at all.
    for id in &registered {
        if named(*id) {
            assert!(
                declared.contains_key(id.0),
                "{} has month names but has not declared its cycles, so nothing checks them",
                id.0
            );
        }
    }

    assert_eq!(
        registered.len(),
        73,
        "the registry changed; update the coverage numbers deliberately"
    );
    assert_eq!(
        with_shape, 36,
        "calendars declaring a shape — raise this by declaring more, and never \
         lower it, because a calendar that stops declaring stops being checked"
    );
    assert_eq!(
        with_months, 27,
        "calendars with English month names — raise this by adding data"
    );
}
