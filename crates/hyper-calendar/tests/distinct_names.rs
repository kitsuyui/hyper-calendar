//! No two registered calendars answer to the same name.
//!
//! A page that lists the calendars — `hc_calendars`, [`lines::calendars`] —
//! shows a reader each calendar's name in their locale, or its English name
//! where the locale has none, and the reader picks one by that name. Three
//! calendars that differ only in their correlation constant, or two that
//! share CLDR's one `persian`, would be three or two identical rows, and a
//! reader could not tell which is which. So in English, in every locale
//! `hc-i18n` carries and in `native`, every calendar's name is its own.

#![cfg(all(
    feature = "alloc",
    feature = "civil",
    feature = "i18n",
    feature = "format"
))]

use std::collections::BTreeMap;

use hyper_calendar::hc_calendar::Rd;
use hyper_calendar::hc_i18n::{Locale, data::LOCALES, names};
use hyper_calendar::lines;

/// Any day: the names do not depend on it.
const TODAY: Rd = Rd(739_880);

/// The ids that share a name, name by name.
fn shared<'a>(pairs: impl IntoIterator<Item = (&'a str, &'a str)>) -> Vec<(&'a str, Vec<&'a str>)> {
    let mut by_name: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (id, name) in pairs {
        by_name.entry(name).or_default().push(id);
    }
    by_name
        .into_iter()
        .filter(|(_, ids)| ids.len() > 1)
        .collect()
}

/// The rows of [`lines::calendars`] in a locale: the id and the name a page
/// shows, the locale's own (column 2) or else the English one (column 3).
fn shown(text: &str) -> Vec<(&str, &str)> {
    text.lines()
        .map(|line| {
            let mut cells = line.split('\t');
            let id = cells.next().unwrap_or_default();
            let local = cells.next().unwrap_or_default();
            let english = cells.next().unwrap_or_default();
            (id, if local.is_empty() { english } else { local })
        })
        .collect()
}

#[test]
fn every_calendar_has_an_english_name_of_its_own() {
    let registry = hyper_calendar::registry();
    let english = shared(
        registry
            .metas()
            .map(|meta| (meta.id.as_str(), meta.english_name)),
    );
    assert!(english.is_empty(), "shared English names: {english:?}");
}

#[test]
fn no_two_calendars_share_a_name_in_any_locale() {
    let registry = hyper_calendar::registry();
    let tags = ["en", lines::NATIVE]
        .into_iter()
        .chain(LOCALES.iter().map(|data| data.tag));
    for tag in tags {
        let text = lines::calendars(&registry, TODAY, tag);
        let clashes = shared(shown(&text));
        assert!(clashes.is_empty(), "{tag}: shared names: {clashes:?}");
    }
}

/// The same, read from `hc-i18n` directly: where a locale names two
/// calendars, it names them differently.
#[test]
fn no_locale_gives_two_calendars_one_name() {
    let registry = hyper_calendar::registry();
    for data in LOCALES {
        let locale = Locale::parse(data.tag).unwrap_or(Locale::ROOT);
        let named = registry.metas().filter_map(|meta| {
            names::calendar_display_name(&locale, meta.id).map(|name| (meta.id.as_str(), name))
        });
        let clashes = shared(named);
        assert!(
            clashes.is_empty(),
            "{}: shared names: {clashes:?}",
            data.tag
        );
    }
}
