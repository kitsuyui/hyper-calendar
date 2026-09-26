//! The tab-separated lines the WebAssembly module and the C library write
//! about deep time, written once.
//!
//! Every deep-time line has the same fifteen columns, whichever table it
//! comes from: the kind (`moment`, `cosmic-epoch`, `cosmic-event`,
//! `future-era`, a geologic rank `eon`, `era`, `period`, `epoch` or `age`,
//! or `archaeological`), the English name, the scope (the interval one
//! rank up for a geologic interval, the region for an archaeological
//! period), the older bound's value, standard uncertainty, significant
//! figures and `1` where the chart marks it approximate, the same four for
//! the younger bound, the unit the values are in, the description, the
//! source, and the name in the requested locale. A point in time has the
//! same start and end; a cell with nothing to say is empty.
//!
//! The localised name is the geological chart's own, from
//! [`hc_deep_time::names`], where the chart names the interval in the
//! locale's language, and empty everywhere else: the cosmic epochs and
//! events, the future eras, the archaeological periods and the two
//! `moment` lines have no published translation this library has read,
//! and they are not translated here. The English columns stay what they
//! were, so a page that shows the localised name falls back to the second
//! column itself.

use alloc::string::String;
use core::fmt::Write;

use hc_deep_time::geologic::{self, GeologicInterval, GeologicRank};
use hc_deep_time::universe::{self, CosmicEpoch, CosmicEvent};
use hc_deep_time::{
    ArchaeologicalPeriod, Bp, DeepTime, DeepTimeResult, FutureEra, names, place_years_ago,
};

/// How many columns every deep-time line has.
pub const DEEP_TIME_COLUMNS: usize = 15;

/// One entry's figures: the value, its standard uncertainty, the
/// significant figures claimed (empty when the table claims none) and
/// whether the chart marks it approximate.
struct Bound {
    value: f64,
    std_dev: f64,
    figures: Option<u8>,
    approximate: bool,
}

impl Bound {
    const fn exact(value: f64, std_dev: f64, figures: u8) -> Self {
        Self {
            value,
            std_dev,
            figures: Some(figures),
            approximate: false,
        }
    }

    fn of(time: DeepTime) -> Self {
        Self::exact(time.central_seconds(), time.std_dev(), time.figures())
    }
}

/// The shape every line of the deep-time exports has.
struct Row<'a> {
    kind: &'a str,
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
            let _ = write!(out, "{}\t{}\t", bound.value, bound.std_dev);
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

fn push_epoch(out: &mut String, epoch: &CosmicEpoch) {
    push_row(
        out,
        &Row {
            kind: "cosmic-epoch",
            name: epoch.name,
            scope: "",
            start: epoch.start().ok().map(Bound::of),
            end: epoch.end().ok().map(Bound::of),
            unit: SINCE_BIG_BANG,
            description: epoch.description,
            source: epoch.source,
            localised: "",
        },
    );
}

fn push_event(out: &mut String, event: &CosmicEvent) {
    push_row(
        out,
        &Row {
            kind: "cosmic-event",
            name: event.name,
            scope: "",
            start: event.deep_time().ok().map(Bound::of),
            end: event.deep_time().ok().map(Bound::of),
            unit: SINCE_BIG_BANG,
            description: event.description,
            source: event.source,
            localised: "",
        },
    );
}

fn push_interval(out: &mut String, interval: &GeologicInterval, locale: &str) {
    push_row(
        out,
        &Row {
            kind: interval.rank.english_name(),
            name: interval.name,
            scope: interval.parent.unwrap_or(""),
            start: Some(Bound {
                value: interval.base_ma,
                std_dev: interval.base_std_dev_ma,
                figures: Some(interval.base_figures),
                approximate: interval.base_approximate,
            }),
            end: Some(Bound {
                value: interval.top_ma,
                std_dev: interval.top_std_dev_ma,
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

fn push_period(out: &mut String, period: &ArchaeologicalPeriod) {
    let bound = |bp: DeepTimeResult<Bp>| {
        bp.ok().map(|bp| Bound {
            value: bp.years().value,
            std_dev: bp.years().std_dev,
            figures: None,
            approximate: false,
        })
    };
    push_row(
        out,
        &Row {
            kind: "archaeological",
            name: period.name,
            scope: period.region,
            start: bound(period.begins()),
            end: bound(period.ends()),
            unit: "years-before-1950",
            description: period.description,
            source: period.source,
            localised: "",
        },
    );
}

fn push_future_era(out: &mut String, era: &FutureEra) {
    push_row(
        out,
        &Row {
            kind: "future-era",
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
        push_epoch(&mut out, epoch);
    }
    if let Some(event) = placement.cosmic_event {
        push_event(&mut out, event);
    }
    if let Some(era) = placement.future_era {
        push_future_era(&mut out, era);
    }
    for interval in placement.geologic.iter().flatten() {
        push_interval(&mut out, interval, locale);
    }
    if let Some(period) = placement.archaeological {
        push_period(&mut out, period);
    }
    Ok(out)
}

/// Every cosmic epoch, Big Bang to the present, then every dated cosmic
/// event, oldest first, one line each. `locale` is taken for the shape of
/// the call; no cosmic name has a translation, so the last column is
/// empty on every line.
#[must_use]
pub fn cosmic(locale: &str) -> String {
    let _ = locale;
    let mut out = String::new();
    for epoch in universe::EPOCHS {
        push_epoch(&mut out, epoch);
    }
    for event in universe::EVENTS {
        push_event(&mut out, event);
    }
    out
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
    fn a_geologic_interval_is_named_in_the_locale_and_the_rest_is_not() {
        let text = placement(0.0, 0.0, "ja").unwrap_or_default();
        let placed = rows(&text);
        let quaternary = placed.iter().find(|row| row[1] == "Quaternary");
        assert_eq!(quaternary.map(|row| row[14]), Some("第四系／紀"));
        let modern = placed.iter().find(|row| row[0] == "archaeological");
        assert_eq!(modern.map(|row| row[14]), Some(""));
        assert!(rows(&cosmic("ja")).iter().all(|row| row[14].is_empty()));
        let english = intervals(GeologicRank::Period, "en");
        assert!(rows(&english).iter().all(|row| row[14].is_empty()));
    }
}
