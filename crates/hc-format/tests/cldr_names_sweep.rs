//! The zone and day period fields in every carried locale.
//!
//! The zone names and day periods are generated from CLDR 48, whose files
//! hold two reserved values that are not names: the inheritance marker
//! `↑↑↑` and the empty override `∅∅∅` (UTS #35 version 48.2, Part 1). The
//! generated zone tables write the override as `~`, which the lookup reads
//! as "no name". This sweep writes `z`, `zzzz`, `v`, `vvvv`, `VVVV`, `O` and
//! `OOOO` for the golden zone of every metazone, in winter and in summer,
//! read as standard and as daylight time, and `b`, `bbbb`, `B` and `BBBB`
//! at every half hour, in every carried locale, and holds each field to be
//! written, and none to show a marker. A debug build writes the zones in
//! the two readings a zone keeps, standard in winter and daylight in
//! summer, and a release build all four (policy §7). The zone fields need
//! every locale's names, the `localized-zone-names` feature.

#![cfg(feature = "localized-zone-names")]

use hc_calendar::{CivilDateTime, CivilTime};
use hc_format::patterns::{FormatContext, cldr};
use hc_format::value::ZoneInfo;
use hc_i18n::Locale;
use hc_i18n::zone_names;
use hc_tz::UtcOffset;

const MARKERS: [&str; 3] = ["∅∅∅", "↑↑↑", "~"];

fn locales() -> impl Iterator<Item = Locale> {
    hc_i18n::data::LOCALES
        .iter()
        .map(|data| Locale::parse(data.tag).unwrap_or_else(|error| panic!("{}: {error}", data.tag)))
}

fn write(context: &FormatContext<'_>, pattern: &str) -> String {
    let mut out = String::new();
    cldr::format(&mut out, pattern, context).unwrap_or_else(|error| panic!("{pattern}: {error}"));
    out
}

fn problems_in(tag: &str, what: &str, text: &str) -> Option<String> {
    let bad = text
        .split('|')
        .any(|field| field.is_empty() || MARKERS.iter().any(|marker| field.contains(marker)));
    bad.then(|| format!("{tag} {what}: {text}"))
}

#[test]
fn every_locale_writes_every_zone_field_without_a_marker() {
    let mut zones: Vec<&str> = zone_names::metazones()
        .iter()
        .filter_map(|metazone| zone_names::preferred_zone(metazone, "001"))
        .collect();
    zones.extend(["Europe/London", "Europe/Dublin", "America/Vancouver"]);
    zones.sort_unstable();
    zones.dedup();
    assert!(zones.len() > 150, "{}", zones.len());
    let offset = UtcOffset::from_seconds(3_600).unwrap();
    let mut found = Vec::new();
    let mut written = 0usize;
    for locale in locales() {
        let tag = locale.to_tag();
        let readings: &[(u8, bool)] = if cfg!(debug_assertions) {
            &[(1, false), (7, true)]
        } else {
            &[(1, false), (1, true), (7, false), (7, true)]
        };
        for &(month, daylight) in readings {
            let day = hc_calendar::gregorian::to_fixed(2026, month, 15).unwrap();
            let at = CivilDateTime::new(day, CivilTime::new(12, 0, 0, 0).unwrap());
            for zone in &zones {
                let context = FormatContext::new(at)
                    .with_zone(ZoneInfo::Offset(offset))
                    .with_locale(&locale)
                    .with_zone_id(zone)
                    .with_daylight(daylight);
                let text = write(&context, "z|zzzz|v|vvvv|VVVV|O|OOOO");
                written += 1;
                found.extend(problems_in(&tag, zone, &text));
            }
        }
    }
    assert!(written > 20_000, "{written}");
    assert!(found.is_empty(), "{}", found.join("\n"));
}

#[test]
fn every_locale_writes_every_day_period_without_a_marker() {
    let day = hc_calendar::gregorian::to_fixed(2026, 9, 29).unwrap();
    let mut found = Vec::new();
    for locale in locales() {
        let tag = locale.to_tag();
        for minute in (0..24 * 60).step_by(30) {
            let time = CivilTime::new((minute / 60) as u8, (minute % 60) as u8, 0, 0).unwrap();
            let context = FormatContext::new(CivilDateTime::new(day, time)).with_locale(&locale);
            let text = write(&context, "b|bbbb|B|BBBB");
            found.extend(problems_in(&tag, &format!("{minute} min"), &text));
        }
    }
    assert!(found.is_empty(), "{}", found.join("\n"));
}

/// The fields the sweep would have caught before the empty override and
/// the day period rules were read as UTS #35 reads them: `en-GB`'s `z` for
/// Los Angeles falls to the localized GMT format, and `zh-Hant`'s `B` in
/// the evening is 晚上, `zh`'s rule under `zh_Hant.xml`'s name, not 下午.
#[test]
fn the_cases_the_sweep_guards_write_their_fallbacks() {
    let summer = hc_calendar::gregorian::to_fixed(2026, 7, 1).unwrap();
    let noon = CivilDateTime::new(summer, CivilTime::new(12, 0, 0, 0).unwrap());
    let british = Locale::parse("en-GB").unwrap();
    let pacific = FormatContext::new(noon)
        .with_zone(ZoneInfo::Offset(
            UtcOffset::from_seconds(-7 * 3_600).unwrap(),
        ))
        .with_locale(&british)
        .with_zone_id("America/Los_Angeles")
        .with_daylight(true);
    assert_eq!(write(&pacific, "z|zzzz"), "GMT-7|Pacific Daylight Time");
    let traditional = Locale::parse("zh-Hant").unwrap();
    let evening = CivilDateTime::new(summer, CivilTime::new(20, 0, 0, 0).unwrap());
    let context = FormatContext::new(evening).with_locale(&traditional);
    assert_eq!(write(&context, "BBBB"), "晚上");
}
