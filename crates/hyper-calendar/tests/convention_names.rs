//! The names a boundary accepts for a convention are the owning crate's
//! table, and every document that lists them lists that table.
//!
//! `docs/policy.md` §5: a convention selected by a string at the boundary
//! is an entry of a table in the crate that owns it, and the boundary keeps
//! no list of its own. The lists a reader meets are still written out — in
//! the rustdoc of each export of the C library and the WebAssembly module,
//! in their READMEs, and as a union type in the binding's `.d.ts` — and a
//! written list is right on the day it is written. So each is held to the
//! table here: the `.d.ts` type names exactly the table's identifiers, and
//! every rustdoc block, README paragraph and roadmap table that lists them
//! names every one.

#![cfg(feature = "full")]

use std::collections::BTreeSet;

use hyper_calendar::hc_astro::solar_time::{SolarClock, SolarEvent};
use hyper_calendar::hc_core::epoch_notation::EpochKind;
use hyper_calendar::hc_core::gnss::{self, RolloverRule};
use hyper_calendar::hc_core::tai64;
use hyper_calendar::hc_format::radio::dcf77::Zone;
use hyper_calendar::hc_format::radio::wwvb::DstState;
use hyper_calendar::hc_format::radio::{Code as RadioCode, LeapNotice};
use hyper_calendar::hc_holiday::rule::{Confidence, Kind};
use hyper_calendar::hc_planetary::bodies;
use hyper_calendar::hc_planetary::mars::missions::{MISSIONS, SolConvention};
use hyper_calendar::hc_relativity::constants::GRAVITATING_BODIES;
use hyper_calendar::hc_seasons::ColdFoodConvention;
use hyper_calendar::hc_seasons::meiyu::PlumRainRule;
use hyper_calendar::hc_seasons::meridian::NamedMeridian;
use hyper_calendar::hc_seasons::zodiac::{Ayanamsa, SiderealSign};

#[path = "support/boundaries.rs"]
mod boundaries;

/// The C library's exports, by the name of its crate.
const FFI_SOURCE: &str = "../hyper-calendar-ffi";
const FFI_README: &str = "../hyper-calendar-ffi/README.md";
/// The WebAssembly module's exports, by the name of its crate.
const WASM_SOURCE: &str = "../hyper-calendar-wasm";
const WASM_README: &str = "../hyper-calendar-wasm/README.md";
const DTS: &str = "../hyper-calendar-wasm/js/hyper-calendar.d.ts";
const BINDING: &str = "../hyper-calendar-wasm/js/hyper-calendar.js";
const ROADMAP: &str = "../../docs/calendars.md";

fn read(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("could not read {path}: {error}"))
}

/// The string literals of a `.d.ts` type alias, following the aliases it
/// names, and without the open `(string & {})` a type may add.
fn dts_literals(dts: &str, name: &str) -> BTreeSet<String> {
    let start = format!("export type {name} =");
    let at = dts
        .find(&start)
        .unwrap_or_else(|| panic!("the .d.ts has no type {name}"));
    let body = &dts[at + start.len()..];
    let end = body
        .find(';')
        .unwrap_or_else(|| panic!("the .d.ts type {name} does not end"));
    let body = &body[..end];
    let mut out = BTreeSet::new();
    for (index, piece) in body.split('"').enumerate() {
        if index % 2 == 1 {
            out.insert(piece.to_owned());
        } else {
            for word in piece.split(|c: char| !c.is_ascii_alphanumeric()) {
                if word.starts_with(|c: char| c.is_ascii_uppercase()) {
                    out.extend(dts_literals(dts, word));
                }
            }
        }
    }
    out
}

/// The doc comment above the line that begins with `start`, one line:
/// the `///` lines of a Rust item or the ` * ` lines of a JSDoc block.
fn doc_above(source: &str, start: &str) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let index = lines
        .iter()
        .position(|line| line.trim_start().starts_with(start))
        .unwrap_or_else(|| panic!("no line begins {start}"));
    let mut cursor = index;
    let mut docs = Vec::new();
    while cursor > 0 {
        cursor -= 1;
        let line = lines[cursor].trim_start();
        if let Some(doc) = line.strip_prefix("///").or_else(|| line.strip_prefix('*')) {
            docs.push(doc.trim());
        } else if line.starts_with("#[") || line.starts_with("/**") {
            continue;
        } else {
            break;
        }
    }
    docs.reverse();
    docs.join(" ")
}

/// The rustdoc of an export, `unsafe` or not.
fn rustdoc(source: &str, export: &str) -> String {
    let unsafe_one = format!("pub unsafe extern \"C\" fn {export}(");
    if source.contains(&unsafe_one) {
        doc_above(source, &unsafe_one)
    } else {
        doc_above(source, &format!("pub extern \"C\" fn {export}("))
    }
}

/// The paragraph of a Markdown file that holds `marker`: from the blank
/// line before it to the blank line after, one line.
fn paragraph(markdown: &str, marker: &str) -> String {
    let at = markdown
        .find(marker)
        .unwrap_or_else(|| panic!("no paragraph holds {marker}"));
    let start = markdown[..at].rfind("\n\n").map_or(0, |start| start + 2);
    let end = markdown[at..]
        .find("\n\n")
        .map_or(markdown.len(), |end| at + end);
    markdown[start..end].replace('\n', " ")
}

/// A table's identifiers, the `.d.ts` type that lists them, and where
/// prose lists them too.
struct Listed {
    what: &'static str,
    ids: Vec<&'static str>,
    dts: &'static str,
    /// Exports whose rustdoc names every identifier, each with the source
    /// of its boundary.
    exports: &'static [(&'static str, &'static str)],
    /// Markers of README paragraphs, each with its README, that name every
    /// identifier.
    paragraphs: &'static [(&'static str, &'static str)],
    /// Methods of the JavaScript binding whose JSDoc names every one.
    methods: &'static [&'static str],
}

fn listed() -> Vec<Listed> {
    vec![
        Listed {
            what: "solar clocks",
            ids: SolarClock::ALL.iter().map(|clock| clock.id).collect(),
            dts: "SolarClock",
            exports: &[
                (FFI_SOURCE, "hc_solar_time"),
                (WASM_SOURCE, "hc_solar_time"),
            ],
            paragraphs: &[
                (FFI_README, "`hc_solar_time(clock, unix_seconds"),
                (WASM_README, "`hc_solar_time(clock_ptr"),
                (ROADMAP, "| Italian hours (*ore italiane*) |"),
            ],
            methods: &["solarTime"],
        },
        Listed {
            what: "solar events",
            ids: SolarEvent::ALL.iter().map(|event| event.id).collect(),
            dts: "SolarEventName",
            exports: &[
                (FFI_SOURCE, "hc_solar_event"),
                (WASM_SOURCE, "hc_solar_event"),
            ],
            paragraphs: &[
                (FFI_README, "`hc_solar_time(clock, unix_seconds"),
                (WASM_README, "`hc_solar_event(event_ptr"),
                (ROADMAP, "| Italian hours (*ore italiane*) |"),
            ],
            methods: &["solarEvent"],
        },
        Listed {
            what: "named meridians",
            ids: NamedMeridian::ALL.iter().map(|named| named.id).collect(),
            dts: "Meridian",
            exports: &[],
            paragraphs: &[],
            methods: &[],
        },
        Listed {
            what: "plum-rain rules",
            ids: PlumRainRule::ALL.iter().map(|rule| rule.id).collect(),
            dts: "PlumRainRule",
            exports: &[
                (FFI_SOURCE, "hc_plum_rains"),
                (WASM_SOURCE, "hc_plum_rains"),
            ],
            paragraphs: &[],
            methods: &[],
        },
        Listed {
            what: "surface missions",
            ids: MISSIONS.iter().map(|mission| mission.id).collect(),
            dts: "MissionId",
            exports: &[],
            paragraphs: &[],
            methods: &["missionSol"],
        },
        Listed {
            what: "bodies",
            ids: bodies::ALL.iter().map(|body| body.id).collect(),
            dts: "BodyId",
            exports: &[],
            paragraphs: &[],
            methods: &[],
        },
        Listed {
            what: "gravitating bodies",
            ids: GRAVITATING_BODIES.iter().map(|body| body.id).collect(),
            dts: "GravitatingBodyId",
            exports: &[],
            paragraphs: &[],
            methods: &[],
        },
        Listed {
            what: "ayanamsas",
            ids: Ayanamsa::ALL.iter().map(|ayanamsa| ayanamsa.id()).collect(),
            dts: "Ayanamsa",
            exports: &[
                (FFI_SOURCE, "hc_panchanga_at"),
                (WASM_SOURCE, "hc_panchanga_at"),
            ],
            paragraphs: &[
                (FFI_README, "`ayanamsa` is a NUL-terminated identifier"),
                (WASM_README, "moves with it twice over"),
            ],
            methods: &["panchangaAt"],
        },
        Listed {
            what: "cold-food conventions",
            ids: ColdFoodConvention::ALL.iter().map(|c| c.id()).collect(),
            dts: "ColdFoodConvention",
            exports: &[
                (FFI_SOURCE, "hc_cold_food_day"),
                (WASM_SOURCE, "hc_cold_food_day"),
            ],
            paragraphs: &[],
            methods: &[],
        },
        Listed {
            what: "TAI64 formats",
            ids: tai64::Format::ALL
                .iter()
                .map(|format| format.id())
                .collect(),
            dts: "Tai64Format",
            exports: &[
                (FFI_SOURCE, "hc_tai64_encode"),
                (WASM_SOURCE, "hc_tai64_encode"),
            ],
            paragraphs: &[],
            methods: &[],
        },
        Listed {
            what: "GNSS week fields",
            ids: gnss::ALL.iter().map(|numbering| numbering.id()).collect(),
            dts: "GnssNumbering",
            exports: &[(FFI_SOURCE, "hc_gnss_week"), (WASM_SOURCE, "hc_gnss_week")],
            paragraphs: &[],
            methods: &[],
        },
        Listed {
            what: "GNSS rollover rules",
            ids: RolloverRule::ALL.iter().map(|rule| rule.id).collect(),
            dts: "RolloverRule",
            exports: &[
                (FFI_SOURCE, "hc_gnss_resolve_week"),
                (WASM_SOURCE, "hc_gnss_resolve_week"),
            ],
            paragraphs: &[],
            methods: &[],
        },
        Listed {
            what: "epoch notations",
            ids: EpochKind::ALL
                .iter()
                .map(|kind| kind.id())
                .chain(["J", "B"])
                .collect(),
            dts: "EpochNotationName",
            exports: &[],
            paragraphs: &[],
            methods: &[],
        },
        Listed {
            what: "radio codes",
            ids: RadioCode::ALL.iter().map(|code| code.id()).collect(),
            dts: "RadioCode",
            exports: &[
                (FFI_SOURCE, "hc_radio_decode"),
                (WASM_SOURCE, "hc_radio_decode"),
            ],
            paragraphs: &[],
            methods: &[],
        },
        Listed {
            what: "radio summer-time states",
            ids: Zone::ALL
                .iter()
                .map(|zone| zone.id())
                .chain(DstState::ALL.iter().map(|state| state.id()))
                .collect(),
            dts: "RadioSummer",
            exports: &[
                (WASM_SOURCE, "hc_radio_decode"),
                (WASM_SOURCE, "hc_radio_encode"),
            ],
            paragraphs: &[],
            methods: &[],
        },
        Listed {
            what: "radio leap notices",
            ids: LeapNotice::ALL.iter().map(|leap| leap.id()).collect(),
            dts: "RadioLeap",
            exports: &[],
            paragraphs: &[],
            methods: &[],
        },
        Listed {
            what: "sidereal signs",
            ids: SiderealSign::ALL.iter().map(|sign| sign.id()).collect(),
            dts: "SiderealSignId",
            exports: &[],
            paragraphs: &[],
            methods: &[],
        },
        Listed {
            what: "mission clocks",
            ids: SolConvention::ALL.iter().map(|c| c.id()).collect(),
            dts: "MissionClock",
            exports: &[(FFI_SOURCE, "hc_missions"), (WASM_SOURCE, "hc_missions")],
            paragraphs: &[],
            methods: &[],
        },
        Listed {
            what: "holiday kinds",
            ids: Kind::ALL.iter().map(|kind| kind.id()).collect(),
            dts: "HolidayKind",
            exports: &[],
            paragraphs: &[],
            methods: &[],
        },
        Listed {
            what: "holiday confidences",
            ids: Confidence::ALL.iter().map(|c| c.id()).collect(),
            dts: "Confidence",
            exports: &[],
            paragraphs: &[],
            methods: &[],
        },
    ]
}

/// What each list gets wrong against its table.
fn problems(listed: &Listed, dts: &str, sources: &[(&str, String)], binding: &str) -> Vec<String> {
    let mut out = Vec::new();
    let ids: BTreeSet<String> = listed.ids.iter().map(|id| (*id).to_owned()).collect();
    let typed = dts_literals(dts, listed.dts);
    if typed != ids {
        out.push(format!(
            "{}: the .d.ts type {} lists {typed:?}, the table {ids:?}",
            listed.what, listed.dts
        ));
    }
    let mut prose: Vec<(String, String)> = Vec::new();
    for (path, export) in listed.exports {
        let source = sources
            .iter()
            .find(|(known, _)| known == path)
            .map_or("", |(_, source)| source.as_str());
        prose.push((format!("{path} {export}"), rustdoc(source, export)));
    }
    for (path, marker) in listed.paragraphs {
        prose.push((format!("{path} {marker}"), paragraph(&read(path), marker)));
    }
    for method in listed.methods {
        prose.push((
            format!("{BINDING} {method}"),
            doc_above(binding, &format!("{method}(")),
        ));
    }
    for (place, text) in prose {
        for id in &listed.ids {
            if !text.contains(&format!("`{id}`")) {
                out.push(format!("{}: {place} does not name `{id}`", listed.what));
            }
        }
    }
    out
}

#[test]
fn every_list_of_a_conventions_names_is_its_table() {
    let dts = read(DTS);
    // Each boundary's rustdoc as a source text: its own exports' and the
    // facade table's rows it expands.
    let rustdoc_of = |crate_dir: &str, side| {
        boundaries::as_source(&boundaries::exports(
            std::path::Path::new(crate_dir),
            std::path::Path::new("."),
            side,
        ))
    };
    let sources = [
        (FFI_SOURCE, rustdoc_of(FFI_SOURCE, boundaries::Side::C)),
        (WASM_SOURCE, rustdoc_of(WASM_SOURCE, boundaries::Side::Wasm)),
    ];
    let binding = read(BINDING);
    let found: Vec<String> = listed()
        .iter()
        .flat_map(|listed| problems(listed, &dts, &sources, &binding))
        .collect();
    assert!(found.is_empty(), "{}", found.join("\n"));
}

/// The check fails a list that leaves an identifier out, and a type that
/// adds one.
#[test]
fn the_check_catches_a_short_list_and_a_long_type() {
    let listed = Listed {
        what: "test",
        ids: vec!["a", "b"],
        dts: "T",
        exports: &[("src", "hc_x")],
        paragraphs: &[],
        methods: &[],
    };
    let dts = "export type T = \"a\" | \"b\" | \"c\";";
    let sources = [(
        "src",
        "/// `a` only.\npub extern \"C\" fn hc_x() {}".to_owned(),
    )];
    let found = problems(&listed, dts, &sources, "");
    assert_eq!(found.len(), 2, "{found:?}");
    assert!(found[0].contains("the .d.ts type T"));
    assert!(found[1].contains("does not name `b`"));
}
