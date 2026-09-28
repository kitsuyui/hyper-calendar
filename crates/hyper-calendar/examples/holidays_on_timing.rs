//! What one day across every holiday table costs, table by table.
//!
//! The WebAssembly and C exports `hc_holidays_on` evaluate every table of
//! `holiday_lines::tables` for one day with `HolidayCalendar::for_day_with`,
//! through one `EvaluationContext` and inside one `hc_core::memo::scope`,
//! so their time is the sum of what each table costs after the tables
//! before it have filled the context and the memo. This example times that
//! whole call, the shortest of several runs, then the same tables with a
//! scope of their own each, which is what `for_day_with` opens when no
//! caller has, and every table's whole year, and prints the costliest
//! tables of the call.
//!
//! Run with:
//!
//! ```sh
//! cargo run -p hyper-calendar --example holidays_on_timing --profile release-compact \
//!     --features holiday -- [fixed]
//! ```
//!
//! The day defaults to 2026-01-01, the costliest day of 2026. With
//! `--dump <first> <last> <step>` it prints instead what every table says
//! about every `step`th day from fixed day `first` to `last`, evaluated as
//! the exports evaluate it, so that two builds can be compared line by
//! line.

use std::time::{Duration, Instant};

use hyper_calendar::hc_calendar::Rd;
use hyper_calendar::hc_core::memo;
use hyper_calendar::hc_holiday::{EvaluationContext, HolidayCalendar};
use hyper_calendar::holiday_lines::tables;

/// 2026-01-01, the day timed unless another is given.
const NEW_YEAR: Rd = Rd(739_617);

/// How many times each figure is taken.
const RUNS: usize = 7;

/// How many tables the per-table list prints.
const TOP: usize = 15;

fn ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1e3
}

/// Every table's entries and gaps for `day`, nationwide and in each of its
/// subdivisions as the exports evaluate them, through one context; returns
/// how many entries and gaps there were and each table's time, its
/// subdivisions' included.
fn one_day(day: Rd) -> (usize, Vec<Duration>) {
    let mut context = EvaluationContext::new();
    let mut count = 0;
    let mut times = Vec::new();
    for table in tables() {
        let start = Instant::now();
        let calendar = HolidayCalendar::for_day_with(table, None, day, &mut context);
        count += calendar.on(day).len() + calendar.gaps().len();
        for region in table.regions() {
            let regional = HolidayCalendar::for_day_with(table, Some(region), day, &mut context);
            count += regional.on(day).len() + regional.gaps().len();
        }
        times.push(start.elapsed());
    }
    (count, times)
}

/// The shortest of [`RUNS`] timings of [`one_day`], in one scope or not,
/// with each table's shortest time.
fn best(day: Rd, in_one_scope: bool) -> (Duration, usize, Vec<Duration>) {
    let mut best_total = Duration::MAX;
    let mut best_tables: Vec<Duration> = Vec::new();
    let mut count = 0;
    for _ in 0..RUNS {
        let start = Instant::now();
        let (entries, times) = if in_one_scope {
            memo::scope(|| one_day(day))
        } else {
            one_day(day)
        };
        best_total = best_total.min(start.elapsed());
        count = entries;
        if best_tables.is_empty() {
            best_tables = times;
        } else {
            for (kept, time) in best_tables.iter_mut().zip(times) {
                *kept = (*kept).min(time);
            }
        }
    }
    (best_total, count, best_tables)
}

fn dump(first: i64, last: i64, step: usize) {
    for fixed in (first..=last).step_by(step.max(1)) {
        let day = Rd(fixed);
        // A fresh context and scope a day, as each call of the exports
        // makes them.
        let mut context = EvaluationContext::new();
        memo::scope(|| {
            for table in tables() {
                let calendar = HolidayCalendar::for_day_with(table, None, day, &mut context);
                for holiday in calendar.on(day) {
                    println!("{fixed}\t{}\t{holiday:?}", table.code);
                }
                for gap in calendar.gaps() {
                    println!("{fixed}\t{}\tgap {gap:?}", table.code);
                }
                for region in table.regions() {
                    let regional =
                        HolidayCalendar::for_day_with(table, Some(region), day, &mut context);
                    for holiday in regional.on(day) {
                        println!("{fixed}\t{}\t{region}\t{holiday:?}", table.code);
                    }
                }
            }
        });
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("--dump") {
        let number = |index: usize| -> i64 {
            args.get(index)
                .and_then(|text| text.parse().ok())
                .unwrap_or_else(|| panic!("--dump <first> <last> <step>"))
        };
        dump(
            number(1),
            number(2),
            usize::try_from(number(3)).unwrap_or(1),
        );
        return;
    }
    let day = args
        .first()
        .and_then(|text| text.parse().ok())
        .map_or(NEW_YEAR, Rd);

    let codes: Vec<&str> = tables().map(|table| table.code).collect();
    let year = hyper_calendar::hc_calendars_solar::gregorian::year_from_fixed(day).unwrap_or(0);
    let whole_year = (0..3)
        .map(|_| {
            let start = Instant::now();
            for table in tables() {
                std::hint::black_box(HolidayCalendar::for_year(table, None, year).on(day));
            }
            start.elapsed()
        })
        .min()
        .unwrap_or_default();
    let (apart, _, _) = best(day, false);
    let (total, count, times) = best(day, true);
    println!(
        "fixed day {}: {:.2} ms across all {} tables in one scope, {:.2} ms in a scope a table, \
         {:.2} ms for every table's year {year}; {count} entries and gaps",
        day.0,
        ms(total),
        codes.len(),
        ms(apart),
        ms(whole_year)
    );
    let mut rows: Vec<(&str, Duration)> = codes.into_iter().zip(times).collect();
    rows.sort_by_key(|&(_, time)| std::cmp::Reverse(time));
    for (code, time) in rows.iter().take(TOP) {
        println!("{code:<28} {:>9.3} ms", ms(*time));
    }
}
