//! No two days of a calendar whose leap units are named are written alike.
//!
//! A leap unit that reads like an ordinary one makes two days one text:
//! the Burmese First Waso written as Waso, the late Tagu of the year's end
//! as its first Tagu, the Thai extra eighth month as the eighth, the Lao
//! later eighth month as the extra one, the Bahá'í intercalary days as days
//! of Mulk, St. Tib's Day as Chaos 59, Nepal Sambat's Analā as the month
//! it doubles. Each is named here, in the
//! words its sources write (`hc_i18n::data`, `hc_i18n::fields`), and this
//! test holds every day from 1950 to 2050 to a text of its own, in each
//! locale the calendar's dates are read in.
//!
//! Calendars whose written dates repeat by their sources' own practice are
//! not held here: a Hindu or Nepal Sambat tithi that spans two sunrises
//! gives both days its name, and a Tibetan or Mongolian duplicated day its
//! number twice. So is the repeated day of a Faṣlī year, for which no
//! written form was found. `tests/written_dates.rs` lists each with a
//! doubled day, and `docs/systems/written-dates.md` the sources.

#![cfg(all(
    feature = "alloc",
    feature = "civil",
    feature = "regional",
    feature = "i18n",
    feature = "format"
))]

use std::collections::HashMap;

use hyper_calendar::hc_calendar::Rd;
use hyper_calendar::hc_format::label;
use hyper_calendar::hc_i18n::Locale;

/// 1 January 1950 and 1 January 2051.
const FROM: i64 = 711_858;
const TO: i64 = 748_749;

/// Each calendar and the locales its dates are read in, `None` being its
/// own language.
const CALENDARS: &[(&str, &[Option<&str>])] = &[
    ("burmese", &[Some("en"), Some("my"), None]),
    ("thai-lunar", &[Some("en"), Some("th"), None]),
    ("bahai", &[Some("en"), None]),
    ("bahai-arithmetic", &[Some("en"), None]),
    ("bahai-astronomical", &[Some("en"), None]),
    ("lao", &[Some("en"), None]),
    ("discordian", &[Some("en"), Some("de")]),
];

/// The days a build walks: every day in a release build; in a debug build
/// two consecutive years in every seven, and the last two, so that a late
/// Tagu and the Tagu a year before it, which carry one year number, are
/// both walked.
fn days() -> Vec<i64> {
    let years: Vec<i64> = if cfg!(debug_assertions) {
        (1950..=2050)
            .step_by(7)
            .chain([2049])
            .flat_map(|year| [year, year + 1])
            .collect()
    } else {
        (1950..=2050).collect()
    };
    let start = |year: i64| {
        hyper_calendar::hc_calendar::gregorian::to_fixed(year, 1, 1)
            .unwrap_or_else(|error| panic!("{year}: {error:?}"))
            .0
    };
    let mut out: Vec<i64> = years
        .into_iter()
        .flat_map(|year| start(year)..start(year + 1))
        .filter(|day| (FROM..TO).contains(day))
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

#[test]
fn no_two_days_share_a_text() {
    let registry = hyper_calendar::registry();
    let days = days();
    let mut repeats = Vec::new();
    for (id, locales) in CALENDARS {
        let calendar = registry
            .get_by_name(id)
            .unwrap_or_else(|| panic!("{id} is registered"));
        let meta = calendar.meta();
        for tag in *locales {
            let requested: Option<Locale> = tag.map(|tag| tag.parse().expect("a tag"));
            let locale = label::locale_for(calendar, requested.as_ref());
            let mut seen: HashMap<String, i64> = HashMap::new();
            hyper_calendar::hc_core::memo::scope(|| {
                for &day in &days {
                    if !meta.supports(Rd(day)) {
                        continue;
                    }
                    let fields = calendar.fixed_to_fields(Rd(day)).expect("in range");
                    let text = label::date(calendar, &fields, &locale);
                    if let Some(first) = seen.insert(text.clone(), day) {
                        repeats.push(format!("{id} {tag:?}: {first} and {day} are {text:?}"));
                    }
                }
            });
            assert!(seen.len() > 1_000, "{id} {tag:?} wrote {} days", seen.len());
        }
    }
    assert!(repeats.is_empty(), "{}", repeats.join("\n"));
}

/// The names the test relies on, where the sources write them: a late
/// Tagu, First Waso and Second Waso in both languages, the Thai extra
/// eighth month, and Ayyám-i-Há.
#[test]
fn the_leap_units_are_named() {
    let registry = hyper_calendar::registry();
    let text = |id: &str, day: i64, tag: &str| {
        let calendar = registry.get_by_name(id).expect("registered");
        let fields = calendar.fixed_to_fields(Rd(day)).expect("in range");
        let requested: Locale = tag.parse().expect("a tag");
        label::date(
            calendar,
            &fields,
            &label::locale_for(calendar, Some(&requested)),
        )
    };
    // 29 March 2017, Wikipedia's «၁၃၇၈ ခုနှစ်၊ နှောင်းတန်ခူးလဆန်း ၂ ရက်».
    assert_eq!(
        text("burmese", 736_417, "my"),
        "၁၃၇၈ ခုနှစ်၊ နှောင်းတန်ခူးလဆန်း ၂ ရက်"
    );
    assert_eq!(
        text("burmese", 736_417, "en"),
        "Late Tagu waxing 2, 1378 ME"
    );
    // 14 July 2026, the fifteenth waning day of First Waso 1388 (the
    // Ministry of Religious Affairs), and 21 July 2023, the fourth waxing
    // day of Second Waso 1385 (a government order).
    assert_eq!(
        text("burmese", 739_811, "en"),
        "First Waso waning 15, 1388 ME"
    );
    assert_eq!(text("burmese", 738_722, "my"), "၁၃၈၅ ခုနှစ်၊ ဒုတိယဝါဆိုလဆန်း ၄ ရက်");
    // The adhikamāsa year 2569 that `hc_calendars_regional::thai_lunar`
    // anchors: 30 June 2026 is แรม 1 ค่ำ of the first eighth month, and
    // 29 July 2026, Asalha Bucha, ขึ้น 15 ค่ำ of the regular one.
    assert_eq!(
        text("thai-lunar", 739_797, "th"),
        "แรม 1 ค่ำ เดือนแปดแรก 2569"
    );
    assert_eq!(text("thai-lunar", 739_826, "th"), "ขึ้น 15 ค่ำ เดือนแปด 2569");
    // 26 February 2027, the first of Ayyám-i-Há 183 on bahai.org's table.
    assert_eq!(text("bahai", 740_038, "en"), "Ayyám-i-Há 1, 183 BE");
}
