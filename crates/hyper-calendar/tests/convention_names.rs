//! The names a boundary accepts for a convention are the owning crate's
//! table, and every document that lists them lists that table.
//!
//! `docs/policy.md` §5: a convention selected by a string at the boundary
//! is an entry of a table in the crate that owns it, and the boundary keeps
//! no list of its own. The lists a reader meets are still written out — in
//! the rustdoc of each export of the C library and the WebAssembly module,
//! in their READMEs, and as a union type in the binding's `.d.ts` — and a
//! written list is right on the day it is written. So each is held to the
//! table here: the `.d.ts` type names exactly the table's identifiers,
//! every rustdoc block, README paragraph and roadmap table that lists them
//! names every one, and each identifier of a convention is named by a row
//! or paragraph of the roadmaps, as §5's "The roadmap row gives the
//! identifiers" asks. Every `.d.ts` union of a convention's identifiers is
//! one of these; the unions of the words a line is written in, such as
//! `Standing` or `MoonPhaseName`, are not tables and are not held here. A
//! listing that hands the identifiers back — `missions()`, `bodies()`,
//! `horizons()` — types its field with the same union, so that a caller
//! passes a listed identifier to the lookup without a cast.

#![cfg(feature = "full")]

use std::collections::BTreeSet;

use hyper_calendar::hc_astro::HORIZONS;
use hyper_calendar::hc_astro::solar_time::{SolarClock, SolarEvent, ZMANIM_RECKONINGS};
use hyper_calendar::hc_calendars_indic::barhaspatya;
use hyper_calendar::hc_calendars_indic::kalam::KalamConvention;
use hyper_calendar::hc_calendars_indic::kumbh::KumbhYoga;
use hyper_calendar::hc_calendars_indic::panchak::PanchakNaming;
use hyper_calendar::hc_calendars_lunar::islamic_observational::NamedCriterion;
use hyper_calendar::hc_calendars_solar::adoption::Scope;
use hyper_calendar::hc_core::epoch_notation::EpochKind;
use hyper_calendar::hc_core::gnss::{self, RolloverRule};
use hyper_calendar::hc_core::tai64;
use hyper_calendar::hc_format::ccsds::AsciiPrecision;
use hyper_calendar::hc_format::east_african_hours;
use hyper_calendar::hc_format::radio::dcf77::Zone;
use hyper_calendar::hc_format::radio::wwvb::DstState;
use hyper_calendar::hc_format::radio::{Code as RadioCode, LeapNotice};
use hyper_calendar::hc_holiday::orthodox_fasts;
use hyper_calendar::hc_holiday::rule::{Confidence, Kind};
use hyper_calendar::hc_planetary::mars::missions::{MISSIONS, SolConvention};
use hyper_calendar::hc_planetary::{bodies, dated};
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
/// The roadmaps: the calendars', the observances', the time scales' and
/// the off-Earth reckonings'. Policy §5 has the roadmap row give a
/// convention's identifiers.
const ROADMAPS: &[&str] = &[
    ROADMAP,
    "../../docs/observances.md",
    "../../docs/time-scales.md",
    "../../docs/off-earth.md",
];

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

/// The type a `.d.ts` interface gives one of its fields, without the `;`.
fn dts_field(dts: &str, interface: &str, field: &str) -> Option<String> {
    let start = dts
        .find(&format!("export interface {interface} {{"))
        .or_else(|| dts.find(&format!("export interface {interface} extends")))?;
    let body = &dts[start..];
    let body = &body[..body.find("\n}").unwrap_or(body.len())];
    body.lines().find_map(|line| {
        let rest = line
            .strip_prefix(&format!("  {field}: "))
            .or_else(|| line.strip_prefix(&format!("  {field}?: ")))?;
        Some(rest.trim_end_matches(';').trim().to_owned())
    })
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
    /// Fields of the `.d.ts` interfaces a listing or a reading returns,
    /// `(interface, field)`, that hold one of the identifiers and so must
    /// be typed with the union, or with it or `null`.
    fields: &'static [(&'static str, &'static str)],
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
            fields: &[],
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
            fields: &[],
        },
        Listed {
            what: "named meridians",
            ids: NamedMeridian::ALL.iter().map(|named| named.id).collect(),
            dts: "Meridian",
            exports: &[],
            paragraphs: &[],
            methods: &[],
            fields: &[],
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
            fields: &[],
        },
        Listed {
            what: "surface missions",
            ids: MISSIONS.iter().map(|mission| mission.id).collect(),
            dts: "MissionId",
            exports: &[],
            paragraphs: &[],
            methods: &["missionSol"],
            fields: &[("Mission", "id")],
        },
        Listed {
            what: "bodies",
            ids: bodies::ALL.iter().map(|body| body.id).collect(),
            dts: "BodyId",
            exports: &[],
            paragraphs: &[],
            methods: &[],
            fields: &[("Body", "id"), ("Body", "primary")],
        },
        Listed {
            what: "gravitating bodies",
            ids: GRAVITATING_BODIES.iter().map(|body| body.id).collect(),
            dts: "GravitatingBodyId",
            exports: &[],
            paragraphs: &[],
            methods: &[],
            fields: &[("GravitatingBody", "id"), ("GravitationalDilation", "id")],
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
            fields: &[],
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
            fields: &[],
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
            fields: &[("Tai64Label", "format")],
        },
        Listed {
            what: "GNSS week fields",
            ids: gnss::ALL.iter().map(|numbering| numbering.id()).collect(),
            dts: "GnssNumbering",
            exports: &[(FFI_SOURCE, "hc_gnss_week"), (WASM_SOURCE, "hc_gnss_week")],
            paragraphs: &[],
            methods: &[],
            fields: &[],
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
            fields: &[],
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
            fields: &[],
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
            fields: &[],
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
            fields: &[],
        },
        Listed {
            what: "radio leap notices",
            ids: LeapNotice::ALL.iter().map(|leap| leap.id()).collect(),
            dts: "RadioLeap",
            exports: &[],
            paragraphs: &[],
            methods: &[],
            fields: &[],
        },
        Listed {
            what: "sidereal signs",
            ids: SiderealSign::ALL.iter().map(|sign| sign.id()).collect(),
            dts: "SiderealSignId",
            exports: &[],
            paragraphs: &[],
            methods: &[],
            fields: &[("KumbhOccasion", "jupiter"), ("PushkaramDays", "sign")],
        },
        Listed {
            what: "mission clocks",
            ids: SolConvention::ALL.iter().map(|c| c.id()).collect(),
            dts: "MissionClock",
            exports: &[(FFI_SOURCE, "hc_missions"), (WASM_SOURCE, "hc_missions")],
            paragraphs: &[],
            methods: &[],
            fields: &[("Mission", "clock")],
        },
        Listed {
            what: "holiday kinds",
            ids: Kind::ALL.iter().map(|kind| kind.id()).collect(),
            dts: "HolidayKind",
            exports: &[],
            paragraphs: &[],
            methods: &[],
            fields: &[],
        },
        Listed {
            what: "holiday confidences",
            ids: Confidence::ALL.iter().map(|c| c.id()).collect(),
            dts: "Confidence",
            exports: &[],
            paragraphs: &[],
            methods: &[],
            fields: &[],
        },
        Listed {
            what: "horizons",
            ids: HORIZONS.iter().map(|horizon| horizon.id).collect(),
            dts: "HorizonId",
            exports: &[],
            paragraphs: &[],
            methods: &[],
            fields: &[("Horizon", "id")],
        },
        Listed {
            what: "crescent criteria",
            ids: NamedCriterion::ALL.iter().map(|named| named.id).collect(),
            dts: "CrescentCriterion",
            exports: &[],
            paragraphs: &[],
            methods: &[],
            fields: &[],
        },
        Listed {
            what: "zmanim reckonings",
            ids: ZMANIM_RECKONINGS
                .iter()
                .map(|reckoning| reckoning.id)
                .collect(),
            dts: "ZmanimReckoning",
            exports: &[],
            paragraphs: &[],
            methods: &[],
            fields: &[],
        },
        Listed {
            what: "kalam conventions",
            ids: KalamConvention::ALL.iter().map(|c| c.id).collect(),
            dts: "KalamConvention",
            exports: &[],
            paragraphs: &[],
            methods: &[],
            fields: &[],
        },
        Listed {
            what: "Kumbh yogas",
            ids: KumbhYoga::ALL.iter().map(|yoga| yoga.id).collect(),
            dts: "KumbhYoga",
            exports: &[],
            paragraphs: &[],
            methods: &[],
            fields: &[("KumbhOccasion", "id")],
        },
        Listed {
            what: "Orthodox fast reckonings",
            ids: orthodox_fasts::Reckoning::ALL
                .iter()
                .map(|reckoning| reckoning.id)
                .collect(),
            dts: "OrthodoxFastReckoning",
            exports: &[],
            paragraphs: &[],
            methods: &[],
            fields: &[],
        },
        Listed {
            what: "six-hour reckonings",
            ids: east_african_hours::ALL
                .iter()
                .map(|reckoning| reckoning.id)
                .collect(),
            dts: "SixHourReckoning",
            exports: &[],
            paragraphs: &[],
            methods: &[],
            fields: &[],
        },
        Listed {
            what: "Panchak namings",
            ids: PanchakNaming::ALL.iter().map(|naming| naming.id).collect(),
            dts: "PanchakNaming",
            exports: &[],
            paragraphs: &[],
            methods: &[],
            fields: &[],
        },
        Listed {
            what: "Barhaspatya rules",
            ids: barhaspatya::RULES.iter().map(|rule| rule.id).collect(),
            dts: "BarhaspatyaRule",
            exports: &[
                (FFI_SOURCE, "hc_barhaspatya_year"),
                (WASM_SOURCE, "hc_barhaspatya_year"),
            ],
            paragraphs: &[],
            methods: &[],
            fields: &[],
        },
        Listed {
            what: "calendars dated at an instant",
            ids: dated::ALL.iter().map(|calendar| calendar.id()).collect(),
            dts: "CircadCalendar",
            exports: &[],
            paragraphs: &[],
            methods: &[],
            fields: &[],
        },
        Listed {
            what: "CCSDS ASCII precisions",
            ids: AsciiPrecision::KINDS.to_vec(),
            dts: "CcsdsAsciiPrecision",
            exports: &[],
            paragraphs: &[],
            methods: &[],
            fields: &[],
        },
        Listed {
            what: "adoption scopes",
            ids: Scope::ALL.iter().map(|scope| scope.as_str()).collect(),
            dts: "AdoptionScope",
            exports: &[],
            paragraphs: &[],
            methods: &[],
            fields: &[],
        },
    ]
}

/// The tables that are not conventions, whose identifiers the roadmap does
/// not list: the bodies and the gravitating bodies, which are catalogues
/// of things in the sky; the twelve sidereal signs, the positions of one
/// cycle; and the words a line or a frame is read in — a radio frame's
/// summer-time state and leap notice, a holiday's kind and confidence, a
/// CCSDS ASCII code's precision, an adoption's scope.
const NOT_CONVENTIONS: &[&str] = &[
    "bodies",
    "gravitating bodies",
    "sidereal signs",
    "radio summer-time states",
    "radio leap notices",
    "holiday kinds",
    "holiday confidences",
    "CCSDS ASCII precisions",
    "adoption scopes",
];

/// The rows and paragraphs of the roadmaps, each one line: a table row is
/// a line of its own, and a paragraph runs from blank line to blank line.
fn roadmap_blocks(roadmaps: &[String]) -> Vec<String> {
    let mut blocks = Vec::new();
    for roadmap in roadmaps {
        for paragraph in roadmap.split("\n\n") {
            if paragraph.trim_start().starts_with('|') {
                blocks.extend(paragraph.lines().map(str::to_owned));
            } else {
                blocks.push(paragraph.replace('\n', " "));
            }
        }
    }
    blocks
}

/// What each list gets wrong against its table: the `.d.ts` type, the
/// prose that lists the identifiers, and the roadmap, some row or
/// paragraph of which must name each (policy §5, "The roadmap row gives
/// the identifiers").
fn problems(
    listed: &Listed,
    dts: &str,
    sources: &[(&str, String)],
    binding: &str,
    roadmap: &[String],
) -> Vec<String> {
    let mut out = Vec::new();
    for id in listed
        .ids
        .iter()
        .filter(|_| !NOT_CONVENTIONS.contains(&listed.what))
    {
        let quoted = format!("`{id}`");
        if !roadmap.iter().any(|block| block.contains(&quoted)) {
            out.push(format!("{}: no roadmap row names `{id}`", listed.what));
        }
    }
    let ids: BTreeSet<String> = listed.ids.iter().map(|id| (*id).to_owned()).collect();
    let typed = dts_literals(dts, listed.dts);
    if typed != ids {
        out.push(format!(
            "{}: the .d.ts type {} lists {typed:?}, the table {ids:?}",
            listed.what, listed.dts
        ));
    }
    for (interface, field) in listed.fields {
        let typed = dts_field(dts, interface, field);
        let union = listed.dts;
        if typed.as_deref() != Some(union) && typed != Some(format!("{union} | null")) {
            out.push(format!(
                "{}: the .d.ts field {interface}.{field} is {typed:?}, not {union}",
                listed.what
            ));
        }
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
    let roadmap = roadmap_blocks(&ROADMAPS.iter().map(|path| read(path)).collect::<Vec<_>>());
    let tables = listed();
    for what in NOT_CONVENTIONS {
        assert!(
            tables.iter().any(|listed| listed.what == *what),
            "{what} is not a table here"
        );
    }
    let found: Vec<String> = tables
        .iter()
        .flat_map(|listed| problems(listed, &dts, &sources, &binding, &roadmap))
        .collect();
    assert!(found.is_empty(), "{}", found.join("\n"));
}

/// The check fails a list that leaves an identifier out, a type that adds
/// one, a listing's field typed as a bare string, and a roadmap no row of
/// which names one.
#[test]
fn the_check_catches_a_short_list_and_a_long_type() {
    let listed = Listed {
        what: "test",
        ids: vec!["a", "b"],
        dts: "T",
        exports: &[("src", "hc_x")],
        paragraphs: &[],
        methods: &[],
        fields: &[("R", "id")],
    };
    let dts = "export type T = \"a\" | \"b\" | \"c\";\nexport interface R {\n  id: string;\n}";
    let sources = [(
        "src",
        "/// `a` only.\npub extern \"C\" fn hc_x() {}".to_owned(),
    )];
    let roadmap = roadmap_blocks(&["| A | `a` |\n| C | `c` |".to_owned()]);
    let found = problems(&listed, dts, &sources, "", &roadmap);
    assert_eq!(found.len(), 4, "{found:?}");
    assert!(found[0].contains("no roadmap row names `b`"));
    assert!(found[1].contains("the .d.ts type T"));
    assert!(found[2].contains("the .d.ts field R.id"));
    assert!(found[3].contains("does not name `b`"));
    let roadmap = roadmap_blocks(&["| A | `a` |\n| B | `b` |".to_owned()]);
    assert_eq!(problems(&listed, dts, &sources, "", &roadmap).len(), 3);
}
