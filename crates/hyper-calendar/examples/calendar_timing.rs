//! What `lines::describe_day` and `lines::calendars` cost, calendar by
//! calendar and step by step.
//!
//! The WebAssembly exports `hc_describe_day` and `hc_calendars` are these
//! two functions over the facade's registry, so their time is the sum of
//! what each registered calendar costs: converting the day, finding the
//! standing, naming the era and the month (which may ask whether the year
//! is leap), and writing the date as the locale does. This example times
//! each step for each calendar and prints the costliest ten, twice: alone,
//! each step on its own with nothing remembered, and in the call, the
//! calendars in registry order inside one `hc_core::memo::scope`, as the
//! two functions run them, so that a calendar whose astronomy another has
//! already done shows what it still costs. Then it times both whole calls,
//! and `lines::calendar_list`, which converts no day.
//!
//! Run with:
//!
//! ```sh
//! cargo run -p hyper-calendar --example calendar_timing --profile release-compact \
//!     --features lunar,equinox,indic,regional,i18n,format -- [fixed] [locale]
//! ```
//!
//! The day defaults to 2026-09-27 and the locale to `ja`. Every figure is
//! the shortest of several runs, which is the one least disturbed by
//! whatever else the machine is doing.

use std::time::{Duration, Instant};

use hyper_calendar::hc_calendar::{CalendarRegistry, DynCalendar, Rd};
use hyper_calendar::hc_core::memo;
use hyper_calendar::hc_format::label;
use hyper_calendar::hc_i18n::names;
use hyper_calendar::lines;

/// 2026-09-27, the day timed unless another is given.
const TODAY: Rd = Rd(739_886);

/// How many times each figure is taken.
const RUNS: usize = 7;

/// How many rows each table prints.
const TOP: usize = 10;

/// How long `step` takes, once.
fn once<T>(step: impl FnOnce() -> T) -> Duration {
    let start = Instant::now();
    std::hint::black_box(step());
    start.elapsed()
}

/// The shortest of [`RUNS`] timings of `step`.
fn best<T>(mut step: impl FnMut() -> T) -> Duration {
    (0..RUNS).map(|_| once(&mut step)).min().unwrap_or_default()
}

fn ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1e3
}

/// The five steps of one calendar, in the order a table heads them.
type Steps = [Duration; 5];

/// A calendar's steps.
struct Row {
    id: &'static str,
    steps: Steps,
}

fn print_top(title: &str, headings: [&str; 5], mut rows: Vec<Row>) {
    let total = |row: &Row| row.steps.iter().sum::<Duration>();
    rows.sort_by_key(|row| std::cmp::Reverse(total(row)));
    let all: Duration = rows.iter().map(total).sum();
    println!("\n{title}: {:.2} ms over {} calendars", ms(all), rows.len());
    print!("{:<28} {:>9}", "calendar", "total");
    for heading in headings {
        print!(" {heading:>10}");
    }
    println!();
    for row in rows.iter().take(TOP) {
        print!("{:<28} {:>9.3}", row.id, ms(total(row)));
        for step in row.steps {
            print!(" {:>10.3}", ms(step));
        }
        println!();
    }
}

/// One calendar's steps of `lines::describe_day`, each once.
fn describe_day_steps(calendar: &dyn DynCalendar, day: Rd, tag: &str) -> Steps {
    let id = calendar.meta().id;
    let locale_step = once(|| lines::locale_for(calendar, tag));
    let locale = lines::locale_for(calendar, tag);
    let mut fields = None;
    let from_fixed = once(|| fields = Some(calendar.fixed_to_fields(day)));
    let usage = once(|| calendar.standing(day));
    let (naming, date) = match fields {
        Some(Ok(fields)) => (
            once(|| {
                let era = fields.era.and_then(|code| {
                    names::era_name_by_code(&locale, id, code, names::NameWidth::Wide)
                });
                let leap = names::has_leap_year_month_names(&locale, id)
                    && calendar.has_intercalary_month_of(&fields).unwrap_or(false);
                (era, leap)
            }),
            once(|| label::date(calendar, &fields, &locale)),
        ),
        _ => (Duration::ZERO, Duration::ZERO),
    };
    [from_fixed, usage, naming, date, locale_step]
}

/// One calendar's steps of `lines::calendars`, each once.
fn calendars_steps(calendar: &dyn DynCalendar, today: Rd) -> Steps {
    let meta_step = once(|| calendar.meta());
    let meta = calendar.meta();
    let probe = meta.sample_day(today);
    let mut fields = None;
    let from_fixed = once(|| fields = Some(calendar.fixed_to_fields(probe)));
    let leap = match fields {
        Some(Ok(fields)) => once(|| calendar.is_leap_year_of(&fields)),
        _ => Duration::ZERO,
    };
    let usage = once(|| calendar.standing(today));
    let name = once(|| names::calendar_display_name(&label::locale_for(calendar, None), meta.id));
    [from_fixed, leap, usage, meta_step, name]
}

/// Every calendar's steps, the shortest of [`RUNS`] passes: alone, each
/// step outside any scope, or in the call, every pass one scope over the
/// registry in order.
fn table(
    registry: &CalendarRegistry,
    in_the_call: bool,
    steps: impl Fn(&dyn DynCalendar) -> Steps,
) -> Vec<Row> {
    let calendars: Vec<&dyn DynCalendar> = registry
        .metas()
        .filter_map(|meta| registry.get(meta.id).map(|calendar| calendar as _))
        .collect();
    let mut rows: Vec<Row> = calendars
        .iter()
        .map(|calendar| Row {
            id: calendar.meta().id.as_str(),
            steps: [Duration::MAX; 5],
        })
        .collect();
    let pass = || -> Vec<Steps> { calendars.iter().map(|calendar| steps(*calendar)).collect() };
    for _ in 0..RUNS {
        let taken = if in_the_call {
            memo::scope(pass)
        } else {
            pass()
        };
        for (row, taken) in rows.iter_mut().zip(taken) {
            for (kept, step) in row.steps.iter_mut().zip(taken) {
                *kept = (*kept).min(step);
            }
        }
    }
    rows
}

fn main() {
    let mut args = std::env::args().skip(1);
    let day = args
        .next()
        .and_then(|text| text.parse().ok())
        .map_or(TODAY, Rd);
    let tag = args.next().unwrap_or_else(|| "ja".to_owned());
    let registry = hyper_calendar::registry();
    println!(
        "registry: {:.3} ms, {} calendars",
        ms(best(hyper_calendar::registry)),
        registry.len()
    );

    let headings = ["from_fixed", "usage", "naming", "label", "locale"];
    for in_the_call in [false, true] {
        let rows = table(&registry, in_the_call, |calendar| {
            describe_day_steps(calendar, day, &tag)
        });
        let how = if in_the_call { "in the call" } else { "alone" };
        print_top(
            &format!("describe_day({}, {tag}), {how}", day.0),
            headings,
            rows,
        );
    }
    let headings = ["from_fixed", "leap", "usage", "meta", "name"];
    for in_the_call in [false, true] {
        let rows = table(&registry, in_the_call, |calendar| {
            calendars_steps(calendar, day)
        });
        let how = if in_the_call { "in the call" } else { "alone" };
        print_top(
            &format!("calendars({}, {tag}), {how}", day.0),
            headings,
            rows,
        );
    }

    println!();
    let whole = best(|| lines::describe_day(&hyper_calendar::registry(), day, &tag));
    println!(
        "lines::describe_day, registry included: {:.2} ms",
        ms(whole)
    );
    let whole = best(|| lines::calendars(&hyper_calendar::registry(), day, &tag));
    println!("lines::calendars, registry included: {:.2} ms", ms(whole));
    let whole = best(|| lines::calendar_list(&hyper_calendar::registry(), &tag));
    println!(
        "lines::calendar_list, registry included: {:.3} ms",
        ms(whole)
    );
}
