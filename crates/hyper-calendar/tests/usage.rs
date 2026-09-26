//! Every registered calendar says when it was in use, or is named here as
//! one that cannot.
//!
//! # Why this is a test
//!
//! `Calendar::usage` has a default, `Usage::UNRECORDED`, because a proposed
//! calendar or a day count has nothing to record and should not have to
//! say so. The cost of a default is that a calendar can stay silent by
//! accident: the Japanese, Chinese, Hebrew, Umm al-Qura and Ethiopic
//! calendars all reported today as `unrecorded` for as long as nobody
//! noticed. So every registered identifier is checked here: it either
//! carries a period with a source, or it is on the list below with the
//! reason its module gives. A new calendar that overrides nothing fails
//! this test until it is either given a period or added to the list — and
//! the list is exact in both directions, so a calendar that gains a period
//! has to leave it.

#![cfg(all(
    feature = "alloc",
    feature = "civil",
    feature = "lunar",
    feature = "equinox",
    feature = "indic",
    feature = "regional",
))]

use hyper_calendar::hc_calendar::{Rd, Standing};
use hyper_calendar::hc_calendars_solar::gregorian;

/// The identifiers whose period of use is deliberately unrecorded, with the
/// reason each module gives.
///
/// Three kinds are here. Day counts and bare cycles have nothing to be
/// outside of. Proposals were adopted by nobody. The rest are calendars
/// whose sources, as read, give no span: the Egyptian wandering year under
/// Ptolemy's era, the Roman era of later historians, the Yazidi year, the
/// observational Hijri prediction, the Old Hindu mean reckonings, the Aztec
/// counts and the Zapotec year, the Maya 819-day count, the Javanese and Akan weeks, and the
/// Qumran 364-day year, whose days this library places by a convention of
/// its own.
const UNRECORDED: &[&str] = &[
    // Day counts.
    "ansi-date",
    "ccsds-day",
    "cnes-julian-day",
    "dublin-julian-day",
    "julian-day",
    "lilian",
    "modified-julian-day",
    "reduced-julian-day",
    "truncated-julian-day",
    // Cycles with no epoch anyone kept.
    "akan",
    "javanese-pasaran",
    "sexagenary",
    // Proposals.
    "discordian",
    "hanke-henry",
    "holocene",
    "meyer-palmen",
    "persian-arithmetic",
    "persian-arithmetic-33",
    "positivist",
    "symmetry010",
    "symmetry454",
    "world-calendar",
    "yerm",
    // Sources that give no span.
    "aztec-tonalpohualli",
    "aztec-xiuhpohualli",
    "egyptian",
    "hindu-old-lunar",
    "hindu-old-solar",
    "islamic-rgsa",
    "maya-819",
    "maya-819-gmt2",
    "maya-819-584286",
    "qumran",
    "roman-auc",
    "yazidi",
    "zapotec-yza",
];

#[test]
fn every_registered_calendar_records_its_use_or_is_listed_as_unable_to() {
    let registry = hyper_calendar::registry();
    let mut silent = Vec::new();
    let mut unsourced = Vec::new();
    let mut listed_but_recorded = Vec::new();
    let mut seen = 0;
    for meta in registry.metas() {
        let id = meta.id.as_str();
        let calendar = registry
            .get(meta.id)
            .expect("a listed calendar is findable");
        let usage = calendar.usage();
        seen += 1;
        let listed = UNRECORDED.contains(&id);
        match (usage.is_recorded(), listed) {
            (false, false) => silent.push(id),
            (true, true) => listed_but_recorded.push(id),
            (true, false) if usage.source.is_empty() => unsourced.push(id),
            _ => {}
        }
        if let (Some(from), Some(until)) = (usage.from, usage.until) {
            assert!(from <= until, "{id}: the period ends before it begins");
        }
        if let (Some(from), Some(civil)) = (usage.from, usage.civil_until) {
            assert!(from <= civil, "{id}: civil use ends before use begins");
        }
        if let (Some(civil), Some(until)) = (usage.civil_until, usage.until) {
            assert!(civil <= until, "{id}: civil use outlasts use");
        }
    }
    assert!(
        seen > 100,
        "{seen} calendars registered; the registry is short"
    );
    assert!(
        silent.is_empty(),
        "these calendars record no period of use and are not listed as unable to: {silent:?}"
    );
    assert!(
        unsourced.is_empty(),
        "these calendars claim a period of use without a source: {unsourced:?}"
    );
    assert!(
        listed_but_recorded.is_empty(),
        "these calendars now record a period and should leave the list: {listed_but_recorded:?}"
    );
    for id in UNRECORDED {
        assert!(
            registry.get_by_name(id).is_some(),
            "{id} is listed as unrecorded and is not registered"
        );
    }
}

/// The calendars the record was filled in for: each is plainly in civil or
/// religious use today, and each said `unrecorded` before it was.
#[test]
fn the_calendars_in_use_today_say_so() {
    let registry = hyper_calendar::registry();
    let today = gregorian::to_fixed(2026, 9, 26).unwrap();
    for id in [
        "gregory",
        "japanese",
        "chinese",
        "dangi",
        "vietnamese",
        "hebrew",
        "samaritan",
        "islamic-civil",
        "islamic-umalqura",
        "ethiopic",
        "coptic",
        "persian",
        "bahai",
        "buddhist",
        "burmese",
        "tibetan",
        "hindu-lunar",
        "bikram-sambat",
        "vira-nirvana-samvat",
        "thai-lunar",
        "javanese",
        "javanese-aboge",
        "khmer",
        "maya-tzolkin",
        "icelandic",
    ] {
        let calendar = registry.get_by_name(id).expect(id);
        assert_eq!(calendar.standing(today), Standing::InUse, "{id}");
    }
}

/// The historical calendars are bounded, and the two ends of the Chinese
/// calendar's use are different facts.
#[test]
fn the_historical_calendars_are_bounded() {
    let registry = hyper_calendar::registry();
    let today = gregorian::to_fixed(2026, 9, 26).unwrap();
    for id in [
        "babylonian",
        "swedish-1700",
        "french-republican-equinox",
        "japanese-tenpo",
        "japanese-senmyo",
        "chinese-regnal",
        "korean-regnal",
        "rumi",
        "maya-longcount",
        "japanese-imperial",
        "juche",
        "soviet-week",
    ] {
        let calendar = registry.get_by_name(id).expect(id);
        let usage = calendar.usage();
        assert!(
            usage.from.is_some() && usage.until.is_some(),
            "{id}: {usage:?}"
        );
        assert_eq!(calendar.standing(today), Standing::Extended, "{id}");
    }
    let chinese = registry.get_by_name("chinese").expect("chinese");
    let usage = chinese.usage();
    let republic = gregorian::to_fixed(1912, 1, 1).unwrap();
    assert_eq!(usage.civil_until, Some(Rd(republic.0 - 1)));
    assert!(usage.is_civil(Rd(republic.0 - 1)));
    assert!(!usage.is_civil(republic));
    assert_eq!(usage.standing(today), Standing::InUse);
}

/// The fourteen reform calendars are told apart by name.
#[test]
fn the_reform_calendars_carry_their_polity_in_the_name() {
    let registry = hyper_calendar::registry();
    let mut names = Vec::new();
    for meta in registry.metas() {
        if meta.id.as_str().starts_with("julian-gregorian-") {
            assert!(
                meta.english_name.starts_with("Julian–Gregorian reform ("),
                "{}: {}",
                meta.id,
                meta.english_name
            );
            names.push(meta.english_name);
        }
    }
    assert_eq!(names.len(), 14);
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), 14, "two reform calendars share a name");
    let france = registry
        .get_by_name("julian-gregorian-fr")
        .expect("france")
        .meta();
    assert_eq!(france.english_name, "Julian–Gregorian reform (France)");
}
