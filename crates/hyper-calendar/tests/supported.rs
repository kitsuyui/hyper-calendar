//! Generates `docs/supported.md` and fails when the committed file has
//! drifted from the code.
//!
//! # Why this is a test and not a document
//!
//! Every document in this repository that tried to enumerate the whole
//! workspace has been wrong. Four of them carried four different crate
//! counts, each correct when it was written. `docs/calendars.md` marked
//! 十二直 and 二十八宿 "Planned" in a crate that does not contain them while
//! another crate shipped both with tests. Four pre-Tenpō Japanese lunisolar
//! calendars were registered and listed nowhere.
//!
//! None of that is carelessness. It is what happens when a number lives in
//! prose and its source lives in code. So the index is not written: it is
//! rendered from the registries, the holiday tables and the facade's own
//! manifest, and this test diffs the result against the committed file.
//!
//! The repository already had the instinct — `hc_holiday`'s country table
//! asserts its own length with the comment "a documented count that drifts
//! is a documented lie". This generalises it.
//!
//! To accept a change: `UPDATE_SUPPORTED=1 cargo test -p hyper-calendar
//! --all-features --test supported`, then read the diff before committing
//! it. The regeneration is deliberately not automatic, because a row
//! vanishing is exactly the kind of change that should be looked at.

#![cfg(all(
    feature = "alloc",
    feature = "civil",
    feature = "lunar",
    feature = "regional",
    feature = "holiday",
    feature = "units",
))]

use std::fmt::Write as _;

use hyper_calendar::hc_calendar::{CalendarRegistry, Rd};
use hyper_calendar::hc_calendars_solar::gregorian;

/// Where the committed index lives, relative to this crate.
const INDEX: &str = "../../docs/supported.md";

/// The facade manifest, for the feature table.
const MANIFEST: &str = "Cargo.toml";

/// A calendar row, with the crate and feature it arrived through.
struct Row {
    id: String,
    english_name: &'static str,
    krate: &'static str,
    feature: &'static str,
    earliest: Option<Rd>,
    latest: Option<Rd>,
    astronomical: bool,
    leap_months: bool,
    boundary: &'static str,
    /// The cycles the calendar declares, or `None` where it declares none.
    cycles: Option<&'static [hyper_calendar::hc_calendar::shape::CycleShape]>,
    /// Whether a locale can name this calendar's months in English.
    named: bool,
}

/// A fixed day as an ISO 8601 date, or an em dash when unbounded.
///
/// Years outside 0000–9999 take the expanded form's leading sign, because a
/// bare `999887-08-03` reads as a typo and `+999887-08-03` reads as a range
/// bound, which is what it is.
fn iso(rd: Option<Rd>) -> String {
    let Some(day) = rd else {
        return "—".to_owned();
    };
    let Ok((year, month, dom)) = gregorian::from_fixed(day) else {
        return format!("Rd({})", day.0);
    };
    if (0..=9999).contains(&year) {
        format!("{year:04}-{month:02}-{dom:02}")
    } else {
        format!("{year:+}-{month:02}-{dom:02}")
    }
}

/// Every registered calendar, with the crate that registered it.
///
/// The crate column is *derived*, by registering one crate at a time into a
/// fresh registry, rather than written down beside each id. A hand-written
/// column is how `docs/calendars.md` came to attribute 十二直 to the wrong
/// crate.
fn calendar_rows() -> Vec<Row> {
    type Register = fn(&mut CalendarRegistry);
    let sources: [(&'static str, &'static str, Register); 3] = [
        (
            "hc-calendars-solar",
            "civil",
            hyper_calendar::hc_calendars_solar::register_all,
        ),
        (
            "hc-calendars-lunar",
            "lunar",
            hyper_calendar::hc_calendars_lunar::register_all,
        ),
        (
            "hc-calendars-regional",
            "regional",
            hyper_calendar::hc_calendars_regional::register_all,
        ),
    ];

    let mut rows = Vec::new();
    for (krate, feature, register) in sources {
        let mut registry = CalendarRegistry::new();
        register(&mut registry);
        let ids: Vec<String> = registry.metas().map(|meta| meta.id.to_string()).collect();
        for id in ids {
            let Some(calendar) = registry.get_by_name(&id) else {
                panic!("{id} was just registered and should be findable")
            };
            let meta = calendar.meta();
            rows.push(Row {
                id: id.clone(),
                english_name: meta.english_name,
                krate,
                feature,
                earliest: meta.earliest,
                latest: meta.latest,
                astronomical: meta.is_astronomical,
                leap_months: meta.has_leap_months,
                boundary: boundary_name(calendar.day_boundary()),
                cycles: calendar.cycles(),
                named: has_month_names(meta.id),
            });
        }
    }
    rows.sort_by(|left, right| left.id.cmp(&right.id));
    rows
}

/// Whether English can name this calendar's months.
///
/// The column this feeds is the one nothing used to answer. A calendar
/// could be implemented, tested and registered and still be unnameable in
/// any language, and the only way to find out was to grep. Two of the
/// entries that looked like data — `persian` and `islamic` — were keyed to
/// identifiers no calendar had, so they answered for nothing at all.
fn has_month_names(id: hyper_calendar::hc_calendar::CalendarId) -> bool {
    use hyper_calendar::hc_i18n::Locale;
    use hyper_calendar::hc_i18n::names::{NameContext, month_count};
    let Ok(english) = "en".parse::<Locale>() else {
        return false;
    };
    month_count(&english, id, NameContext::Format).is_some()
}

/// A calendar's declared shape, as a phrase for the table.
fn shape_of(row: &Row) -> String {
    let Some(cycles) = row.cycles else {
        return "—".to_owned();
    };
    if cycles.is_empty() {
        return "none".to_owned();
    }
    cycles
        .iter()
        .map(|cycle| match cycle.length {
            hyper_calendar::hc_calendar::CycleLength::Fixed(length) => {
                format!("{} ×{length}", cycle.kind)
            }
            hyper_calendar::hc_calendar::CycleLength::Intercalary { ordinary, extended } => {
                format!("{} ×{ordinary}–{extended}", cycle.kind)
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// The day boundary as a word for the table.
fn boundary_name(boundary: hyper_calendar::hc_calendar::DayBoundary) -> &'static str {
    use hyper_calendar::hc_calendar::DayBoundary as B;
    match boundary {
        B::Midnight => "midnight",
        B::Noon => "noon",
        B::Sunset => "sunset",
        B::Sunrise => "sunrise",
        B::LocalTime(_) => "local time",
    }
}

/// The facade's features and what each pulls in, read from the manifest.
///
/// Parsed rather than listed, for the same reason as everything else here —
/// and with no TOML crate, because ADR 0005 permits exactly one third-party
/// dependency in this workspace and it is `libm`.
fn feature_rows() -> Vec<(String, String)> {
    let Ok(manifest) = std::fs::read_to_string(MANIFEST) else {
        panic!("the facade manifest should be readable at {MANIFEST}")
    };
    let Some(features) = manifest.split("[features]").nth(1) else {
        panic!("the facade manifest should have a [features] table")
    };
    let features = features.split("\n[").next().unwrap_or(features);

    let mut rows = Vec::new();
    let mut name: Option<String> = None;
    let mut value = String::new();
    for line in features.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if name.is_none() {
            let Some((key, rest)) = line.split_once('=') else {
                continue;
            };
            name = Some(key.trim().to_owned());
            value = rest.trim().to_owned();
        } else {
            value.push(' ');
            value.push_str(line);
        }
        if value.ends_with(']') {
            let deps = value
                .trim_start_matches('[')
                .trim_end_matches(']')
                .split(',')
                .map(|item| item.trim().trim_matches('"'))
                .filter(|item| !item.is_empty())
                .collect::<Vec<_>>()
                .join(", ");
            rows.push((name.take().unwrap_or_default(), deps));
            value.clear();
        }
    }
    rows
}

/// The whole document.
fn render() -> String {
    let mut out = String::new();
    out.push_str(
        "<!--\n  Generated by crates/hyper-calendar/tests/supported.rs.\n  \
         Do not edit by hand: the test that generates it also checks it.\n  \
         To regenerate: UPDATE_SUPPORTED=1 cargo test -p hyper-calendar \\\n  \
         --all-features --test supported\n-->\n\n",
    );
    out.push_str("# What is supported\n\n");
    out.push_str(
        "This file is generated from the code, so it cannot drift from it. It \
         says **what exists**, and nothing about what is planned or how \
         accurate it is:\n\n\
         * Why a thing is built the way it is — [`policy.md`](policy.md) and \
         the [ADRs](adr/).\n\
         * What is *not* here and why, and what is planned — \
         [`calendars.md`](calendars.md) and \
         [`observances.md`](observances.md).\n\
         * How far each answer can be trusted — the crate README linked from \
         each row's crate.\n\n",
    );

    let rows = calendar_rows();
    let _ = writeln!(out, "## Calendars\n");
    let _ = writeln!(
        out,
        "{} registered identifiers, alphabetically. A calendar reachable only \
         by constructing it — an arbitrary Julian-to-Gregorian cut-over, the \
         unbounded Tenpō engine — is not here, because this lists what the \
         registry answers to.\n",
        rows.len()
    );
    let with_shape = rows.iter().filter(|row| row.cycles.is_some()).count();
    let named = rows.iter().filter(|row| row.named).count();
    let _ = writeln!(
        out,
        "**Cycles** is what the calendar declares itself to be made of, and \
         **Named** is whether English can name its months. {with_shape} of \
         {} declare a shape and {named} can be named. Those two gaps are \
         real and are asserted in `tests/vocabulary.rs`, so they can only \
         move deliberately — a calendar that is implemented but unnameable \
         is a gap the library should be able to state, not one a reader has \
         to discover.\n",
        rows.len()
    );
    out.push_str(
        "| id | Name | Crate | Feature | Earliest | Latest | Astronomical | \
         Leap months | Day begins | Cycles | Named |\n| --- | --- | --- | \
         --- | --- | --- | --- | --- | --- | --- | --- |\n",
    );
    for row in &rows {
        let _ = writeln!(
            out,
            "| `{}` | {} | [`{}`](../crates/{}) | `{}` | {} | {} | {} | {} | {} | {} | {} |",
            row.id,
            row.english_name,
            row.krate,
            row.krate,
            row.feature,
            iso(row.earliest),
            iso(row.latest),
            if row.astronomical { "yes" } else { "no" },
            if row.leap_months { "yes" } else { "no" },
            row.boundary,
            shape_of(row),
            if row.named { "yes" } else { "no" },
        );
    }

    let countries = hyper_calendar::hc_holiday::countries::ALL;
    let _ = writeln!(out, "\n## Holidays by country\n");
    let _ = writeln!(
        out,
        "{} tables, feature `holiday`. \"Sources checked\" is the table's own \
         field, not this file's.\n",
        countries.len()
    );
    out.push_str(
        "| Code | Country | Rules | Substitution | Weekend rule | Sources checked |\n\
         | --- | --- | --- | --- | --- | --- |\n",
    );
    let mut sorted: Vec<_> = countries.iter().collect();
    sorted.sort_by_key(|set| set.code);
    for set in sorted {
        let _ = writeln!(
            out,
            "| `{}` | {} | {} | {} | {} | {} |",
            set.code,
            set.english_name,
            set.rules.len(),
            if set.substitution.is_empty() {
                "none"
            } else {
                "yes"
            },
            if set.weekend.is_empty() {
                "Sat–Sun"
            } else {
                "stated"
            },
            format_args!(
                "{:04}-{:02}-{:02}",
                set.sources_checked.year, set.sources_checked.month, set.sources_checked.day
            ),
        );
    }

    let traditions = hyper_calendar::hc_holiday::traditions::ALL;
    let _ = writeln!(out, "\n## Religious and cultural traditions\n");
    let _ = writeln!(out, "{} tables, feature `holiday`.\n", traditions.len());
    out.push_str("| Code | Tradition | Observances |\n| --- | --- | --- |\n");
    let mut sorted: Vec<_> = traditions.iter().collect();
    sorted.sort_by_key(|set| set.code);
    for set in sorted {
        let _ = writeln!(
            out,
            "| `{}` | {} | {} |",
            set.code,
            set.english_name,
            set.rules.len()
        );
    }

    let units = hyper_calendar::hc_units::unit::ALL;
    let _ = writeln!(out, "\n## Exactly defined units of time\n");
    let _ = writeln!(
        out,
        "{} units, feature `units`, shortest first. Each is an exact rational \
         number of seconds, because each was *defined* as one. A unit somebody \
         measured — the sidereal day, the tropical year, the galactic year — \
         is not here; it lives with the model that measured it, with its error \
         bar attached.\n",
        units.len()
    );
    out.push_str("| id | Name | Seconds | Family | Authority |\n| --- | --- | --- | --- | --- |\n");
    for unit in units {
        let _ = writeln!(
            out,
            "| `{}` | {} | {} | {} | {} |",
            unit.id,
            unit.name,
            unit.seconds,
            unit.family.english_name(),
            unit.authority
        );
    }

    // `std`, `alloc` and `libm` choose the build shape rather than a
    // capability, and their values are twenty propagations each. They are
    // described in a sentence instead of tabulated.
    const BUILD_SHAPE: [&str; 4] = ["default", "std", "alloc", "libm"];
    let features = feature_rows();
    let capabilities: Vec<_> = features
        .iter()
        .filter(|(name, _)| !BUILD_SHAPE.contains(&name.as_str()))
        .collect();
    let _ = writeln!(out, "\n## Facade features\n");
    let _ = writeln!(
        out,
        "{} capability features, read from `crates/hyper-calendar/Cargo.toml`. \
         Every one that names a crate also re-exports it; `tests/facade.rs` is \
         what makes that true rather than hoped.\n",
        capabilities.len()
    );
    let _ = writeln!(
        out,
        "Besides these, `std` (on by default) chooses the build shape: turn it \
         off for `no_std`, add `alloc` for the parts that need an allocator, \
         and `libm` for floating-point math on targets without it. A build \
         with neither `std` nor `libm` is refused at compile time.\n"
    );
    out.push_str("| Feature | Implies |\n| --- | --- |\n");
    for (name, deps) in capabilities {
        let shown = deps.replace("dep:", "");
        let _ = writeln!(out, "| `{name}` | {shown} |");
    }

    out
}

#[test]
fn the_committed_index_matches_the_code() {
    let rendered = render();
    let committed = std::fs::read_to_string(INDEX).unwrap_or_default();
    if rendered == committed {
        return;
    }
    if std::env::var_os("UPDATE_SUPPORTED").is_some() {
        if let Err(error) = std::fs::write(INDEX, &rendered) {
            panic!("could not write {INDEX}: {error}")
        }
        return;
    }
    // Show the first difference rather than the whole file.
    let mut lines = rendered.lines().zip(committed.lines()).enumerate();
    let first = lines.find(|(_, (left, right))| left != right);
    let detail = match first {
        Some((index, (left, right))) => format!(
            "line {}:\n  code says:      {left}\n  committed says: {right}",
            index + 1
        ),
        None => format!(
            "the files agree as far as both go, but one is longer: code has {} \
             lines, committed has {}",
            rendered.lines().count(),
            committed.lines().count()
        ),
    };
    panic!(
        "docs/supported.md is out of date with the code.\n\n{detail}\n\n\
         Regenerate with:\n  UPDATE_SUPPORTED=1 cargo test -p hyper-calendar \
         --all-features --test supported\nand read the diff before committing it."
    );
}
