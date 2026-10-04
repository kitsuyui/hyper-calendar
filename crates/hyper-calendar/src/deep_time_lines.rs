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
//! English name is always in the third column, so a page that shows the
//! localised name falls back to it itself.

use alloc::string::String;

use hc_deep_time::archaeology::{self, B2K_DATUM_YEAR, BP_DATUM_YEAR};
use hc_deep_time::constants::{self, CODATA_YEAR};
use hc_deep_time::evidence::{self, EarliestEvidence, EvidenceAge};
use hc_deep_time::future::{self, FutureEvent, Prediction};
use hc_deep_time::geologic::{self, GeologicInterval, GeologicRank};
use hc_deep_time::universe::{self, CosmicEpoch, CosmicEvent};
use hc_deep_time::{
    ArchaeologicalPeriod, Bp, DeepTime, DeepTimeResult, DeepUnit, FutureEra, names, place_years_ago,
};
use hc_uncertainty::Uncertain;

use crate::boundary::{Answer, Line, Refusal, out_of_range};

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

/// A bound's four cells: its value, its standard deviation, its
/// significant figures and `1` if it is approximate; four empty cells for
/// none.
fn bound_cells(line: &mut Line<'_>, bound: Option<&Bound>) {
    match bound {
        Some(bound) => line
            .value(bound.value)
            .value_or_empty(bound.std_dev)
            .value_or_empty(bound.figures)
            .flag(bound.approximate),
        None => line.empties(4),
    };
}

fn push_row(out: &mut String, row: &Row<'_>) {
    let mut line = Line::new(out);
    line.cell(row.kind)
        .cell(row.id)
        .cell(row.name)
        .cell(row.scope);
    bound_cells(&mut line, row.start.as_ref());
    bound_cells(&mut line, row.end.as_ref());
    line.cell(row.unit)
        .cell(row.description)
        .cell(row.source)
        .cell(row.localised);
    line.end();
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
            scope: interval.parent.unwrap_or(""),
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

/// How many columns a line of [`planck_units_lines`] has.
pub const PLANCK_UNIT_COLUMNS: usize = 10;

/// How many columns a line of [`bp_convert_line`] has.
pub const BP_CONVERT_COLUMNS: usize = 8;

/// How many columns a line of [`deep_convert_line`] has.
pub const DEEP_CONVERT_COLUMNS: usize = 13;

/// How many columns a line of [`deep_compare_line`] has.
pub const DEEP_COMPARE_COLUMNS: usize = 12;

/// The CODATA constants the Planck units are built from, and the Planck
/// units, one line each: the symbol, the name, the SI unit, the value, its
/// standard uncertainty (0 for a constant defined exactly), the figures
/// the source prints, the relative standard uncertainty, `1` for a defined
/// constant, the value printed to exactly those figures, and the source.
///
/// The CODATA adjustment is the one `hc_deep_time::constants::CODATA_YEAR`
/// names, 2022. `G` is the one measured constant among them, so every
/// Planck unit inherits its 2.2·10⁻⁵ relative uncertainty, halved or
/// thirded by the root.
#[must_use]
pub fn planck_units_lines() -> String {
    let mut out = String::new();
    for constant in constants::ALL {
        let mut line = Line::new(&mut out);
        line.cell(constant.symbol)
            .cell(constant.name)
            .cell(constant.unit)
            .value(constant.value)
            .value(constant.std_dev)
            .value(constant.figures);
        match constant.relative_uncertainty() {
            Ok(relative) => line.value(relative),
            Err(_) => line.empty(),
        };
        line.flag(constant.is_defined());
        match constant.significant() {
            Ok(text) => line.value(text),
            Err(_) => line.empty(),
        };
        line.value(format_args!("{} (CODATA {CODATA_YEAR})", constant.source));
        line.end();
    }
    out
}

/// What the last cell of a datum conversion names.
const BP_SOURCE: &str = "hc-deep-time archaeology: BP counts back from 1950 CE, b2k from 2000 CE, \
     and a calendar year is in astronomical numbering, year 0 being 1 BCE; a conventional \
     radiocarbon age is not a calendar age and needs a calibration curve this crate does \
     not carry";

/// The datums [`bp_convert_line`] converts between.
const DATUMS: [&str; 4] = ["bp", "b2k", "ce", "radiocarbon-bp"];

/// The identifier of the datum a text names, in any ASCII case.
fn datum(given: &str) -> Answer<&'static str> {
    DATUMS
        .iter()
        .copied()
        .find(|name| hc_core::catalogue::matches(given, name))
        .ok_or(Refusal::Unknown)
}

/// An age or a year in a datum as years before 1950 CE, with its standard
/// deviation carried through.
fn into_years_before_1950(datum: &str, value: Uncertain) -> Answer<Uncertain> {
    match datum {
        "bp" => Ok(value),
        "b2k" => value
            .shifted(-f64::from(B2K_DATUM_YEAR - BP_DATUM_YEAR))
            .map_err(out_of_range),
        "ce" => value
            .negated()
            .and_then(|years| years.shifted(f64::from(BP_DATUM_YEAR)))
            .map_err(out_of_range),
        _ => Err(Refusal::NoData),
    }
}

/// Years before 1950 CE as an age or a year in a datum.
fn from_years_before_1950(datum: &str, years: Uncertain) -> Answer<Uncertain> {
    match datum {
        "bp" => Ok(years),
        "b2k" => years
            .shifted(f64::from(B2K_DATUM_YEAR - BP_DATUM_YEAR))
            .map_err(out_of_range),
        "ce" => years
            .negated()
            .and_then(|negated| negated.shifted(f64::from(BP_DATUM_YEAR)))
            .map_err(out_of_range),
        _ => Err(Refusal::NoData),
    }
}

/// A calendar age or year in one datum written in another, as one line: the
/// two datums, the number given and its standard deviation, the number in
/// the other datum and its standard deviation, the number as a label
/// (`11650 cal BP`, `9701 BCE`), and the source.
///
/// The datums are `bp` (calendar years before 1950 CE), `b2k` (before 2000
/// CE, the ice-core scale) and `ce` (a calendar year in astronomical
/// numbering, where year 0 is 1 BCE and −9700 is 9701 BCE). Moving between
/// them adds or subtracts a whole number of years, so the standard deviation
/// is unchanged. The Holocene's base is 11 700 b2k and 11 650 BP.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a datum that is not one of these,
/// [`Refusal::NoData`] for `radiocarbon-bp`, a conventional radiocarbon age,
/// which is not a count of calendar years and needs a calibration curve
/// (IntCal20 and its companions) that the crate does not carry, and
/// [`Refusal::OutOfRange`] for a number or a standard deviation that is not
/// finite, or a negative one.
pub fn bp_convert_line(years: f64, std_dev: f64, from: &str, to: &str) -> Answer<String> {
    let (from, to) = (datum(from)?, datum(to)?);
    let given = Uncertain::new(years, std_dev).map_err(out_of_range)?;
    let converted = from_years_before_1950(to, into_years_before_1950(from, given)?)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell(from)
        .cell(to)
        .value(years)
        .value(std_dev)
        .value(converted.value)
        .value(converted.std_dev);
    match to {
        "bp" => line.value(format_args!("{} cal BP", converted.value)),
        "b2k" => line.value(format_args!("{} b2k", converted.value)),
        _ if converted.value <= 0.0 => line.value(format_args!("{} BCE", 1.0 - converted.value)),
        _ => line.value(format_args!("{} CE", converted.value)),
    };
    line.cell(BP_SOURCE);
    line.end();
    Ok(out)
}

/// A magnitude of time in one unit written in another, as one line: the
/// two units, the number given and its standard deviation, the converted
/// number and its standard deviation, the converted number printed to the
/// figures its standard deviation supports, the span in seconds and its
/// standard deviation, `log10` of the seconds and its standard deviation
/// (empty for a span of no length), `1` where the conversion factor is
/// exact, and the source.
///
/// The units are `hc_deep_time::DeepUnit`'s identifiers: `planck-time`,
/// `yoctosecond`, `zeptosecond`, `attosecond`, `femtosecond`, `picosecond`,
/// `nanosecond`, `microsecond`, `millisecond`, `second`, `minute`, `hour`,
/// `day`, `julian-year`, `kiloyear`, `megayear` and `gigayear`. Every one
/// but the Planck time is a defined multiple of the second and rescales the
/// standard deviation exactly; the Planck time is CODATA's measurement and
/// brings its 1.1·10⁻⁵ into the answer, so a round trip through it returns
/// the same number with a wider bar.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a unit that is not one of these, and
/// [`Refusal::OutOfRange`] for a number or a standard deviation that is not
/// finite or is negative, and a result that leaves the range of a double.
pub fn deep_convert_line(value: f64, std_dev: f64, from: &str, to: &str) -> Answer<String> {
    let from_unit = DeepUnit::by_id(from).ok_or(Refusal::Unknown)?;
    let to_unit = DeepUnit::by_id(to).ok_or(Refusal::Unknown)?;
    let given = Uncertain::new(value, std_dev).map_err(out_of_range)?;
    let span = DeepTime::from_unit(given, from_unit).map_err(out_of_range)?;
    let converted = span.in_unit(to_unit).map_err(out_of_range)?;
    let text = converted.to_significant().map_err(out_of_range)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell(from_unit.id())
        .cell(to_unit.id())
        .value(value)
        .value(std_dev)
        .value(converted.value)
        .value(converted.std_dev)
        .value(text)
        .value(span.central_seconds())
        .value(span.std_dev());
    match span.log10_seconds() {
        Ok(log) => line.value(log.value).value(log.std_dev),
        Err(_) => line.empties(2),
    };
    line.flag(from_unit.is_defined() && to_unit.is_defined())
        .cell(
            "hc-deep-time DeepTime::from_unit, in_unit and log10_seconds; every unit but the \
             Planck time is exact by definition, the Planck time is CODATA 2022's",
        );
    line.end();
    Ok(out)
}

/// Two magnitudes of time compared, as one line: the seconds and standard
/// deviation of each, the ratio of the first to the second and its standard
/// deviation, the base-ten logarithm of that ratio and its standard
/// deviation, the number of decades between them, `1` where their one-sigma
/// bars meet, `-1`, `0` or `1` for the first being shorter than, equal to or
/// longer than the second by central value, and the source.
///
/// The two are treated as independent, so comparing a magnitude with itself
/// reports a non-zero deviation around a ratio of 1. The units are those of
/// [`deep_convert_line`].
///
/// # Errors
///
/// [`Refusal::Unknown`] for a unit that is not one of those,
/// [`Refusal::OutOfRange`] for a number or a standard deviation that is not
/// finite or is negative, a span of no length or a negative one (there is
/// no logarithm of it), and a ratio that leaves the range of a double.
pub fn deep_compare_line(
    first_value: f64,
    first_std_dev: f64,
    first_unit: &str,
    second_value: f64,
    second_std_dev: f64,
    second_unit: &str,
) -> Answer<String> {
    let first_unit = DeepUnit::by_id(first_unit).ok_or(Refusal::Unknown)?;
    let second_unit = DeepUnit::by_id(second_unit).ok_or(Refusal::Unknown)?;
    let first = DeepTime::from_unit(
        Uncertain::new(first_value, first_std_dev).map_err(out_of_range)?,
        first_unit,
    )
    .map_err(out_of_range)?;
    let second = DeepTime::from_unit(
        Uncertain::new(second_value, second_std_dev).map_err(out_of_range)?,
        second_unit,
    )
    .map_err(out_of_range)?;
    let ratio = first.ratio(second).map_err(out_of_range)?;
    let log = first.log10_ratio(second).map_err(out_of_range)?;
    let order = match first.central_cmp(second) {
        core::cmp::Ordering::Less => -1,
        core::cmp::Ordering::Equal => 0,
        core::cmp::Ordering::Greater => 1,
    };
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.value(first.central_seconds())
        .value(first.std_dev())
        .value(second.central_seconds())
        .value(second.std_dev())
        .value(ratio.value)
        .value(ratio.std_dev)
        .value(log.value)
        .value(log.std_dev)
        .value(log.value.abs())
        .flag(first.overlaps(second))
        .value(order)
        .cell("hc-deep-time DeepTime::ratio, log10_ratio, overlaps and central_cmp");
    line.end();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::cells;
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

    /// `actual` is within `relative` of `expected`.
    fn close(actual: &str, expected: f64, relative: f64) -> bool {
        let actual: f64 = actual.parse().unwrap_or(f64::NAN);
        (actual - expected).abs() <= expected.abs() * relative
    }

    #[test]
    fn the_planck_units_are_codatas_with_their_bars() {
        let text = planck_units_lines();
        let table = rows(&text);
        assert_eq!(table.len(), constants::ALL.len());
        assert!(table.iter().all(|row| row.len() == PLANCK_UNIT_COLUMNS));
        let time = table
            .iter()
            .find(|row| row[0] == "t_P")
            .cloned()
            .unwrap_or_default();
        assert_eq!(time[1], "Planck time");
        assert_eq!(time[2], "s");
        // CODATA 2022 (NIST allascii): 5.391247(60)e-44 s, a relative
        // standard uncertainty of 1.1129e-5; 60-digit division of the two.
        assert!(close(time[3], 5.391_247e-44, 1e-12), "{}", time[3]);
        assert!(close(time[4], 6.0e-49, 1e-9), "{}", time[4]);
        assert_eq!(time[5], "7");
        assert!(
            close(time[6], 1.112_915_063_991_689e-5, 1e-9),
            "{}",
            time[6]
        );
        assert_eq!(time[7], "0");
        assert_eq!(time[8], "5.391247e-44");
        assert!(time[9].contains("CODATA 2022"), "{}", time[9]);
        // The speed of light and hbar are defined, and say so.
        for symbol in ["c", "hbar"] {
            let row = table
                .iter()
                .find(|row| row[0] == symbol)
                .cloned()
                .unwrap_or_default();
            assert_eq!(row[4], "0", "{symbol}");
            assert_eq!(row[7], "1", "{symbol}");
        }
        // G is the measured one.
        let gravitation = table
            .iter()
            .find(|row| row[0] == "G")
            .cloned()
            .unwrap_or_default();
        assert_eq!(gravitation[7], "0");
        assert_eq!(gravitation[8], "6.67430e-11");
    }

    #[test]
    fn a_datum_conversion_moves_the_origin_and_keeps_the_bar() {
        // The Holocene's base is 11 700 b2k and 11 650 BP (hc-deep-time's
        // archaeology module, after the ice-core scale's 2000 CE origin);
        // 1950 BP is year 0, 1 BCE, and 3430 cal BP is -1480, 1481 BCE.
        let convert = |years, std_dev, from, to| {
            bp_convert_line(years, std_dev, from, to).unwrap_or_default()
        };
        let row = |line: &str| {
            line.trim_end()
                .split('\t')
                .map(str::to_owned)
                .collect::<Vec<_>>()
        };
        let line = row(&convert(11_650.0, 5.0, "bp", "b2k"));
        assert_eq!(line.len(), BP_CONVERT_COLUMNS);
        assert_eq!(&line[..2], ["bp", "b2k"]);
        assert_eq!(line[4], "11700");
        assert_eq!(line[5], "5");
        assert_eq!(line[6], "11700 b2k");
        assert_eq!(row(&convert(11_700.0, 0.0, "b2k", "bp"))[4], "11650");
        let zero = row(&convert(1_950.0, 0.0, "bp", "ce"));
        assert_eq!(zero[4], "0");
        assert_eq!(zero[6], "1 BCE");
        let bronze = row(&convert(3_430.0, 60.0, "BP", "CE"));
        assert_eq!(bronze[4], "-1480");
        assert_eq!(bronze[5], "60");
        assert_eq!(bronze[6], "1481 BCE");
        assert_eq!(row(&convert(-1_480.0, 60.0, "ce", "bp"))[4], "3430");
        assert_eq!(row(&convert(2_026.0, 0.0, "ce", "b2k"))[4], "-26");
        assert_eq!(row(&convert(1_000.0, 0.0, "ce", "ce"))[6], "1000 CE");
        // A radiocarbon age is not a calendar age.
        assert_eq!(
            bp_convert_line(3_200.0, 50.0, "radiocarbon-bp", "bp"),
            Err(Refusal::NoData)
        );
        assert_eq!(
            bp_convert_line(3_200.0, 50.0, "bp", "radiocarbon-bp"),
            Err(Refusal::NoData)
        );
        assert_eq!(
            bp_convert_line(3_200.0, 50.0, "bp", "ad"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            bp_convert_line(f64::NAN, 0.0, "bp", "ce"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            bp_convert_line(1.0, -1.0, "bp", "ce"),
            Err(Refusal::OutOfRange)
        );
    }

    #[test]
    fn the_age_of_the_universe_is_four_point_three_five_times_ten_to_the_seventeen_seconds() {
        // Planck 2018, 13.787 +/- 0.020 Gyr, of Julian years of 31 557 600 s
        // (60-digit decimal arithmetic): 4.350846312e17 s +/- 6.31152e14,
        // log10 17.63857374 +/- 0.00063001, 8.0702040e60 Planck times.
        let line = deep_convert_line(13.787, 0.020, "gigayear", "second").unwrap_or_default();
        let row = cells(&line);
        assert_eq!(row.len(), DEEP_CONVERT_COLUMNS);
        assert_eq!(&row[..2], ["gigayear", "second"]);
        assert!(close(row[4], 4.350_846_312e17, 1e-12), "{}", row[4]);
        assert!(close(row[5], 6.311_52e14, 1e-12), "{}", row[5]);
        assert!(close(row[9], 17.638_573_742_674_66, 1e-13), "{}", row[9]);
        assert!(
            close(row[10], 6.300_057_763_157_349e-4, 1e-9),
            "{}",
            row[10]
        );
        assert_eq!(row[11], "1", "a defined unit to a defined unit is exact");
        let planck =
            deep_convert_line(13.787, 0.020, "gigayear", "planck-time").unwrap_or_default();
        let row = cells(&planck);
        assert!(close(row[4], 8.070_204_002_895_805e60, 1e-12), "{}", row[4]);
        // The Planck time's own 1.1e-5 reaches the bar: 1.1707e58, against
        // 0.145 % from the age alone.
        assert!(close(row[5], 1.170_732_065_916_644e58, 1e-9), "{}", row[5]);
        assert_eq!(row[11], "0");
        // A round trip through it returns the number with a wider bar.
        let back = deep_convert_line(1.0, 0.0, "planck-time", "planck-time").unwrap_or_default();
        let row = cells(&back);
        assert!(close(row[4], 1.0, 1e-12), "{}", row[4]);
        assert!(row[5].parse::<f64>().unwrap_or(0.0) > 1.0e-5, "{}", row[5]);
        assert_eq!(
            deep_convert_line(1.0, 0.0, "gigayear", "furlong"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            deep_convert_line(1.0, -1.0, "gigayear", "second"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            deep_convert_line(f64::INFINITY, 0.0, "gigayear", "second"),
            Err(Refusal::OutOfRange)
        );
        // A span of no length has no logarithm, and says so with empty cells.
        let nothing = deep_convert_line(0.0, 0.0, "second", "day").unwrap_or_default();
        assert_eq!(&cells(&nothing)[9..11], ["", ""]);
    }

    #[test]
    fn the_universe_is_sixty_one_decades_older_than_the_planck_time() {
        let line = deep_compare_line(13.787, 0.020, "gigayear", 1.0, 0.0, "planck-time")
            .unwrap_or_default();
        let row = cells(&line);
        assert_eq!(row.len(), DEEP_COMPARE_COLUMNS);
        assert!(close(row[4], 8.070_204_002_895_805e60, 1e-12), "{}", row[4]);
        assert!(close(row[6], 60.906_884_513_187_02, 1e-13), "{}", row[6]);
        assert!(close(row[7], 6.300_243_164_018_53e-4, 1e-9), "{}", row[7]);
        assert!(close(row[8], 60.906_884_513_187_02, 1e-13), "{}", row[8]);
        assert_eq!(row[9], "0");
        assert_eq!(row[10], "1");
        let reversed = deep_compare_line(1.0, 0.0, "planck-time", 13.787, 0.020, "gigayear")
            .unwrap_or_default();
        let row = cells(&reversed);
        assert_eq!(row[10], "-1");
        assert!(close(row[6], -60.906_884_513_187_02, 1e-13), "{}", row[6]);
        // Two spans whose bars meet overlap: 1 day and 24 hours.
        let same = deep_compare_line(1.0, 0.0, "day", 24.0, 0.0, "hour").unwrap_or_default();
        let row = cells(&same);
        assert_eq!(row[9], "1");
        assert_eq!(row[10], "0");
        assert!(close(row[4], 1.0, 1e-15));
        assert_eq!(
            deep_compare_line(0.0, 0.0, "day", 1.0, 0.0, "day"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            deep_compare_line(1.0, 0.0, "day", 0.0, 0.0, "day"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            deep_compare_line(1.0, 0.0, "day", 1.0, 0.0, "fortnight"),
            Err(Refusal::Unknown)
        );
    }
}
