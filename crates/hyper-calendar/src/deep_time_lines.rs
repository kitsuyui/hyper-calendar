//! The tab-separated lines the WebAssembly module and the C library write
//! about deep time, written once.
//!
//! Every deep-time line has the same sixteen columns, whichever table it
//! comes from: the kind (`moment`, `cosmic-epoch`, `cosmic-event`,
//! `earliest-evidence`, `future-era`, `future-event`, a geologic rank
//! `eon`, `era`, `period`, `epoch` or `age`, or `archaeological`), the
//! entry's stable identifier, the English name, the scope (the identifier
//! of the interval one rank up for a geologic interval, the region for an
//! archaeological period, the landmark for an earliest-evidence claim, the
//! kind of prediction for a future event), the older bound's
//! value, standard uncertainty, significant figures and `1` where the
//! table marks it approximate, the same four for the younger bound, the
//! unit the values are in, the description, the source, and the name in
//! the requested locale. A point in time has the same start and end; a
//! cell with nothing to say is empty. An earliest-evidence claim whose
//! source gives a minimum age has no start, and its standard uncertainty is
//! empty where the source states none ([`hc_deep_time::evidence`]); every
//! other row has one. A future event that is an experimental bound has no
//! end: if it happens at all, it is at least that far ahead.
//!
//! The localised name is the geological chart's own, from
//! [`hc_deep_time::names`], where the chart names the interval in the
//! locale's language; for the cosmic, archaeological and earliest-evidence
//! rows it is the established term the same module carries by identifier,
//! where one was read. It is empty everywhere else — the future eras and
//! events and the two `moment` lines have none — and nothing is translated
//! here. The
//! English columns stay what they were, so a page that shows the localised
//! name falls back to the third column itself.

use alloc::string::String;
use core::fmt::Write;

use hc_deep_time::archaeology;
use hc_deep_time::evidence::{self, EarliestEvidence, EvidenceAge};
use hc_deep_time::future::{self, FutureEvent, Prediction};
use hc_deep_time::geologic::{self, GeologicInterval, GeologicRank};
use hc_deep_time::universe::{self, CosmicEpoch, CosmicEvent};
use hc_deep_time::{
    ArchaeologicalPeriod, Bp, DeepTime, DeepTimeResult, FutureEra, names, place_years_ago,
};

/// How many columns every deep-time line has.
pub const DEEP_TIME_COLUMNS: usize = 16;

/// One entry's figures: the value, its standard uncertainty (empty where
/// the source states none), the significant figures claimed (empty when
/// the table claims none) and whether the table marks it approximate.
struct Bound {
    value: f64,
    std_dev: Option<f64>,
    figures: Option<u8>,
    approximate: bool,
}

impl Bound {
    const fn exact(value: f64, std_dev: f64, figures: u8) -> Self {
        Self {
            value,
            std_dev: Some(std_dev),
            figures: Some(figures),
            approximate: false,
        }
    }

    const fn of_evidence(age: EvidenceAge) -> Self {
        Self {
            value: age.years_before_1950,
            std_dev: age.std_dev_years,
            figures: Some(age.figures),
            approximate: age.approximate,
        }
    }

    fn of(time: DeepTime) -> Self {
        Self::exact(time.central_seconds(), time.std_dev(), time.figures())
    }
}

/// The shape every line of the deep-time exports has.
struct Row<'a> {
    kind: &'a str,
    id: &'a str,
    name: &'a str,
    scope: &'a str,
    start: Option<Bound>,
    end: Option<Bound>,
    unit: &'a str,
    description: &'a str,
    source: &'a str,
    localised: &'a str,
}

/// Append one cell: `text` with any tab or line break replaced by a space.
fn push_cell(out: &mut String, text: &str) {
    for character in text.chars() {
        out.push(match character {
            '\t' | '\n' | '\r' => ' ',
            other => other,
        });
    }
}

fn push_bound(out: &mut String, bound: Option<&Bound>) {
    match bound {
        Some(bound) => {
            let _ = write!(out, "{}\t", bound.value);
            if let Some(std_dev) = bound.std_dev {
                let _ = write!(out, "{std_dev}");
            }
            out.push('\t');
            if let Some(figures) = bound.figures {
                let _ = write!(out, "{figures}");
            }
            let _ = write!(out, "\t{}\t", u8::from(bound.approximate));
        }
        None => out.push_str("\t\t\t\t"),
    }
}

fn push_row(out: &mut String, row: &Row<'_>) {
    push_cell(out, row.kind);
    out.push('\t');
    push_cell(out, row.id);
    out.push('\t');
    push_cell(out, row.name);
    out.push('\t');
    push_cell(out, row.scope);
    out.push('\t');
    push_bound(out, row.start.as_ref());
    push_bound(out, row.end.as_ref());
    push_cell(out, row.unit);
    out.push('\t');
    push_cell(out, row.description);
    out.push('\t');
    push_cell(out, row.source);
    out.push('\t');
    push_cell(out, row.localised);
    out.push('\n');
}

/// The unit of every cosmic figure: seconds after the Big Bang.
const SINCE_BIG_BANG: &str = "seconds-since-big-bang";

fn push_epoch(out: &mut String, epoch: &CosmicEpoch, locale: &str) {
    push_row(
        out,
        &Row {
            kind: "cosmic-epoch",
            id: epoch.id,
            name: epoch.name,
            scope: "",
            start: epoch.start().ok().map(Bound::of),
            end: epoch.end().ok().map(Bound::of),
            unit: SINCE_BIG_BANG,
            description: epoch.description,
            source: epoch.source,
            localised: names::entry_name(epoch.id, locale).unwrap_or(""),
        },
    );
}

fn push_event(out: &mut String, event: &CosmicEvent, locale: &str) {
    push_row(
        out,
        &Row {
            kind: "cosmic-event",
            id: event.id,
            name: event.name,
            scope: "",
            start: event.deep_time().ok().map(Bound::of),
            end: event.deep_time().ok().map(Bound::of),
            unit: SINCE_BIG_BANG,
            description: event.description,
            source: event.source,
            localised: names::entry_name(event.id, locale).unwrap_or(""),
        },
    );
}

fn push_evidence(out: &mut String, entry: &EarliestEvidence, locale: &str) {
    push_row(
        out,
        &Row {
            kind: "earliest-evidence",
            id: entry.id,
            name: entry.name,
            scope: entry.landmark,
            start: entry.dating.older().map(Bound::of_evidence),
            end: Some(Bound::of_evidence(entry.dating.younger())),
            unit: YEARS_BEFORE_1950,
            description: entry.description,
            source: entry.source,
            localised: names::entry_name(entry.id, locale).unwrap_or(""),
        },
    );
}

/// The unit of the archaeological and earliest-evidence figures.
const YEARS_BEFORE_1950: &str = "years-before-1950";

fn push_interval(out: &mut String, interval: &GeologicInterval, locale: &str) {
    push_row(
        out,
        &Row {
            kind: interval.rank.english_name(),
            id: interval.id,
            name: interval.name,
            scope: interval
                .parent
                .and_then(geologic::by_name)
                .map_or("", |parent| parent.id),
            start: Some(Bound {
                value: interval.base_ma,
                std_dev: Some(interval.base_std_dev_ma),
                figures: Some(interval.base_figures),
                approximate: interval.base_approximate,
            }),
            end: Some(Bound {
                value: interval.top_ma,
                std_dev: Some(interval.top_std_dev_ma),
                figures: Some(interval.top_figures),
                approximate: interval.top_approximate,
            }),
            unit: "megayears-before-present",
            description: "",
            source: geologic::CHART_CITATION,
            localised: names::interval_name(interval, locale).unwrap_or(""),
        },
    );
}

fn push_period(out: &mut String, period: &ArchaeologicalPeriod, locale: &str) {
    let bound = |bp: DeepTimeResult<Bp>| {
        bp.ok().map(|bp| Bound {
            value: bp.years().value,
            std_dev: Some(bp.years().std_dev),
            figures: None,
            approximate: false,
        })
    };
    push_row(
        out,
        &Row {
            kind: "archaeological",
            id: period.id,
            name: period.name,
            scope: period.region,
            start: bound(period.begins()),
            end: bound(period.ends()),
            unit: YEARS_BEFORE_1950,
            description: period.description,
            source: period.source,
            localised: names::entry_name(period.id, locale).unwrap_or(""),
        },
    );
}

fn push_future_era(out: &mut String, era: &FutureEra) {
    push_row(
        out,
        &Row {
            kind: "future-era",
            id: era.id,
            name: era.name,
            scope: "",
            start: Some(Bound::exact(era.start_decade, 0.0, 2)),
            end: era.end_decade.map(|decade| Bound::exact(decade, 0.0, 2)),
            unit: "log10-years-from-now",
            description: era.description,
            source: era.source,
            localised: "",
        },
    );
}

/// A moment some years before the present placed in every chronology, one
/// line each: the moment itself as `since-big-bang` and `before-present`,
/// its cosmic epoch and the last dated cosmic event before it, its future
/// era if it lies ahead, its geologic chain from eon down to age, and its
/// archaeological period, each present only where that chronology reaches.
///
/// A moment no more than [`hc_deep_time::PRESENT_HORIZON_YEARS`] ahead is
/// in the present intervals as well as in its future era; see
/// [`hc_deep_time::timeline`] for where the future begins.
///
/// # Errors
///
/// What [`place_years_ago`] refuses: a value that is not finite or is
/// beyond what the crate can represent.
pub fn placement(years_ago: f64, std_dev_years: f64, locale: &str) -> DeepTimeResult<String> {
    let placement = place_years_ago(years_ago, std_dev_years)?;
    let mut out = String::new();
    let moment = |name, time: DeepTime, unit| Row {
        kind: "moment",
        id: name,
        name,
        scope: "",
        start: Some(Bound::of(time)),
        end: Some(Bound::of(time)),
        unit,
        description: "",
        source: universe::PRESENT_DAY.source,
        localised: "",
    };
    push_row(
        &mut out,
        &moment("since-big-bang", placement.since_big_bang, SINCE_BIG_BANG),
    );
    push_row(
        &mut out,
        &moment(
            "before-present",
            placement.before_present,
            "seconds-before-present",
        ),
    );
    if let Some(epoch) = placement.cosmic_epoch {
        push_epoch(&mut out, epoch, locale);
    }
    if let Some(event) = placement.cosmic_event {
        push_event(&mut out, event, locale);
    }
    if let Some(era) = placement.future_era {
        push_future_era(&mut out, era);
    }
    for interval in placement.geologic.iter().flatten() {
        push_interval(&mut out, interval, locale);
    }
    if let Some(period) = placement.archaeological {
        push_period(&mut out, period, locale);
    }
    Ok(out)
}

/// Every cosmic epoch, Big Bang to the present, then every dated cosmic
/// event, oldest first, one line each, named in `locale` where
/// [`hc_deep_time::names`] carries a name.
#[must_use]
pub fn cosmic(locale: &str) -> String {
    let mut out = String::new();
    for epoch in universe::EPOCHS {
        push_epoch(&mut out, epoch, locale);
    }
    for event in universe::EVENTS {
        push_event(&mut out, event, locale);
    }
    out
}

/// Every claim to the earliest evidence of life, of *Homo sapiens* and of
/// writing, in the order of [`evidence::EVIDENCE`], one line each, named
/// in `locale` where [`hc_deep_time::names`] carries a name.
#[must_use]
pub fn earliest_evidence(locale: &str) -> String {
    let mut out = String::new();
    for entry in evidence::EVIDENCE {
        push_evidence(&mut out, entry, locale);
    }
    out
}

/// Every conventional archaeological period, youngest first, one line
/// each, named in `locale` where [`hc_deep_time::names`] carries a name.
#[must_use]
pub fn archaeological_periods(locale: &str) -> String {
    let mut out = String::new();
    for period in archaeology::PERIODS {
        push_period(&mut out, period, locale);
    }
    out
}

/// Every dated event of the far future, soonest first, one line each, in
/// years from now, with the kind of prediction as the scope. No future
/// event has a name in another language, so the last column is empty.
#[must_use]
pub fn future_events(locale: &str) -> String {
    let mut out = String::new();
    for event in future::EVENTS {
        push_future_event(&mut out, event, locale);
    }
    out
}

fn push_future_event(out: &mut String, event: &FutureEvent, locale: &str) {
    let bound = || {
        Bound::exact(
            event.years_from_now(),
            event.std_dev_years(),
            event.figures(),
        )
    };
    // An experimental bound says only that the event is no sooner.
    let end = match event.prediction {
        Prediction::ExperimentalBound => None,
        Prediction::Modelled | Prediction::OrderOfMagnitude => Some(bound()),
    };
    push_row(
        out,
        &Row {
            kind: "future-event",
            id: event.id,
            name: event.name,
            scope: event.prediction.id(),
            start: Some(bound()),
            end,
            unit: "years-from-now",
            description: event.description,
            source: event.source,
            localised: names::entry_name(event.id, locale).unwrap_or(""),
        },
    );
}

/// The rank a number names: 0 eon, 1 era, 2 period, 3 epoch, 4 age.
#[must_use]
pub fn rank(number: u32) -> Option<GeologicRank> {
    usize::try_from(number)
        .ok()
        .and_then(|index| GeologicRank::ALL.get(index))
        .copied()
}

/// Every interval of one rank, youngest first, one line each, named in
/// `locale` where the chart names it.
#[must_use]
pub fn intervals(rank: GeologicRank, locale: &str) -> String {
    let mut out = String::new();
    for interval in geologic::intervals(rank) {
        push_interval(&mut out, interval, locale);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    fn rows(text: &str) -> Vec<Vec<&str>> {
        text.lines()
            .map(|line| line.split('\t').collect())
            .collect()
    }

    #[test]
    fn every_line_has_every_column() {
        for text in [
            placement(66e6, 0.0, "ja").unwrap_or_default(),
            cosmic("ja"),
            intervals(GeologicRank::Age, "ja"),
        ] {
            assert!(!text.is_empty());
            assert!(rows(&text).iter().all(|row| row.len() == DEEP_TIME_COLUMNS));
        }
    }

    #[test]
    fn every_row_carries_its_identifier_and_no_two_share_one() {
        let text = cosmic("");
        let cosmic_rows = rows(&text);
        for (index, row) in cosmic_rows.iter().enumerate() {
            assert!(!row[1].is_empty(), "{row:?}");
            assert!(
                cosmic_rows[index + 1..]
                    .iter()
                    .all(|other| other[1] != row[1])
            );
        }
        let placed = placement(0.0, 0.0, "").unwrap_or_default();
        let ids: Vec<&str> = rows(&placed).iter().map(|row| row[1]).collect();
        assert_eq!(
            ids,
            [
                "since-big-bang",
                "before-present",
                "era-of-galaxies",
                "present-day",
                "phanerozoic",
                "cenozoic",
                "quaternary",
                "holocene",
                "meghalayan",
                "modern-period"
            ]
        );
    }

    #[test]
    fn a_geologic_scope_is_the_identifier_one_rank_up() {
        let text = intervals(GeologicRank::Age, "");
        let maastrichtian = rows(&text)
            .into_iter()
            .find(|row| row[1] == "maastrichtian")
            .map(|row| row[3].to_owned());
        assert_eq!(maastrichtian.as_deref(), Some("upper-cretaceous"));
        let eons = intervals(GeologicRank::Eon, "");
        assert!(rows(&eons).iter().all(|row| row[3].is_empty()));
    }

    #[test]
    fn a_name_is_localised_where_one_was_read_and_nowhere_else() {
        let text = placement(0.0, 0.0, "ja").unwrap_or_default();
        let placed = rows(&text);
        let column = |id: &str| placed.iter().find(|row| row[1] == id).map(|row| row[15]);
        assert_eq!(column("quaternary"), Some("第四系／紀"));
        assert_eq!(column("modern-period"), Some("近代"));
        assert_eq!(column("present-day"), Some("現在"));
        assert_eq!(column("era-of-galaxies"), Some(""));
        assert_eq!(column("since-big-bang"), Some(""));
        let english = cosmic("en");
        assert!(rows(&english).iter().all(|row| row[15].is_empty()));
        let periods = intervals(GeologicRank::Period, "en");
        assert!(rows(&periods).iter().all(|row| row[15].is_empty()));
        let japanese = cosmic("ja");
        let recombination = rows(&japanese)
            .into_iter()
            .find(|row| row[1] == "recombination")
            .map(|row| row[15].to_owned());
        assert_eq!(recombination.as_deref(), Some("宇宙の晴れ上がり"));
    }

    #[test]
    fn an_earliest_evidence_row_keeps_the_shape_of_its_date() {
        assert!(
            rows(&cosmic("ja"))
                .iter()
                .all(|row| row[0].starts_with("cosmic-"))
        );
        let text = earliest_evidence("ja");
        let all = rows(&text);
        let row = |id: &str| {
            all.iter()
                .find(|row| row[1] == id)
                .cloned()
                .unwrap_or_default()
        };
        // A minimum age with a stated uncertainty: no start, a sigma.
        let omo = row("earliest-homo-sapiens-omo-kibish");
        assert_eq!(
            omo[..4],
            [
                "earliest-evidence",
                "earliest-homo-sapiens-omo-kibish",
                "Omo I, Omo-Kibish",
                "earliest-homo-sapiens"
            ]
        );
        assert_eq!(omo[4..12], ["", "", "", "", "233000", "11000", "3", "0"]);
        assert_eq!(omo[12], "years-before-1950");
        assert_eq!(omo[15], "オモの化石");
        // An age whose +/- is not a stated sigma: the sigma cell is empty.
        let irhoud = row("earliest-homo-sapiens-jebel-irhoud");
        assert_eq!(
            irhoud[4..12],
            ["315000", "", "3", "0", "315000", "", "3", "0"]
        );
        // A range: two different bounds.
        let nuvvuagittuq = row("earliest-life-nuvvuagittuq");
        assert_eq!(
            nuvvuagittuq[4..12],
            ["4280000000", "", "3", "0", "3770000000", "", "3", "0"]
        );
        // An approximate calendar date.
        let uruk = row("earliest-writing-uruk-iv");
        assert_eq!(uruk[4..12], ["5249", "", "2", "1", "5249", "", "2", "1"]);
        assert_eq!(uruk[15], "原楔形文字");
        assert_eq!(
            all.iter()
                .filter(|row| row[0] == "earliest-evidence")
                .count(),
            evidence::EVIDENCE.len()
        );
        assert_eq!(all.len(), evidence::EVIDENCE.len());
    }

    #[test]
    fn only_an_earliest_evidence_row_may_lack_a_standard_uncertainty() {
        let placed = placement(-8.0e9, 0.0, "").unwrap_or_default();
        for text in [
            cosmic(""),
            archaeological_periods(""),
            future_events(""),
            intervals(GeologicRank::Age, ""),
            placed,
        ] {
            for row in rows(&text) {
                assert_eq!(row[4].is_empty(), row[5].is_empty(), "{row:?}");
                assert_eq!(row[8].is_empty(), row[9].is_empty(), "{row:?}");
            }
        }
    }

    #[test]
    fn the_periods_and_the_future_events_are_listed() {
        let text = archaeological_periods("ja");
        let periods = rows(&text);
        assert_eq!(periods.len(), archaeology::PERIODS.len());
        assert_eq!(
            periods[0][..4],
            [
                "archaeological",
                "modern-period",
                "Modern period",
                "Europe and Southwest Asia"
            ]
        );
        assert_eq!(periods[0][15], "近代");
        let text = future_events("ja");
        let events = rows(&text);
        assert_eq!(events.len(), future::EVENTS.len());
        assert!(
            events
                .iter()
                .all(|row| row[0] == "future-event" && row[12] == "years-from-now")
        );
        let tip = events
            .iter()
            .find(|row| row[1] == "sun-red-giant-tip")
            .cloned()
            .unwrap_or_default();
        assert_eq!(tip[3], "modelled");
        assert_eq!(
            tip[4..12],
            [
                "7590000000",
                "50000000",
                "3",
                "0",
                "7590000000",
                "50000000",
                "3",
                "0"
            ]
        );
        // A lower bound on the proton lifetime has a start and no end.
        let proton = events
            .iter()
            .find(|row| row[1] == "proton-decay-lower-bound")
            .cloned()
            .unwrap_or_default();
        assert_eq!(proton[3], "experimental-bound");
        assert_eq!(proton[4], "24000000000000000000000000000000000");
        assert_eq!(proton[8..12], ["", "", "", ""]);
        assert!(events.iter().all(|row| row[15].is_empty()));
    }
}
