//! The helper the calendar crates' day-by-day sweeps share, written once.
//!
//! A round-trip sweep (docs/policy.md §7) checks every day of a range, or a
//! sample of it, and no day's check depends on another's, so the days are
//! spread over the machine's threads.
//! [`check_days_in_parallel!`](crate::check_days_in_parallel) defines the
//! function that does it, `check_days`, in the tests of the crate that
//! invokes it, so that each crate's `std` feature decides whether threads
//! are used and nothing of it is compiled outside a test build.

/// Whether this build is instrumented for coverage: `cargo llvm-cov`, which
/// the coverage job runs, sets `cfg(coverage)`.
///
/// Every thread of an instrumented build increments the same counters, one
/// per region of code, and threads that run the same code fight over the
/// cache lines that hold them. Measured on a fourteen-core desktop on
/// 28 September 2026, the Babylonian calendar's sampled sweep took 56 s
/// spread over fourteen threads and 12 s on one, and the Tibetan
/// calendar's 24 s and 6 s, for the same instructions. So
/// [`check_days_in_parallel!`](crate::check_days_in_parallel) keeps an
/// instrumented sweep on one thread.
pub const INSTRUMENTED: bool = cfg!(coverage);

/// How many times sparser than a debug build's sample an instrumented
/// build's sample of days is, before [`thinned`] rounds it up to a prime.
pub const COVERAGE_THINNING: usize = 3;

/// The first prime of at least `n`.
const fn prime_from(n: usize) -> usize {
    let mut candidate = if n < 2 { 2 } else { n };
    loop {
        let mut divisor = 2;
        let mut prime = true;
        while divisor * divisor <= candidate {
            if candidate % divisor == 0 {
                prime = false;
                break;
            }
            divisor += 1;
        }
        if prime {
            return candidate;
        }
        candidate += 1;
    }
}

/// The step between the days of a sweep that a debug build samples every
/// `sampled`th day of, in a build that is instrumented for coverage: the
/// first prime of at least three times as many ([`COVERAGE_THINNING`]), and
/// `sampled` itself in any other build.
///
/// A prime, like the strides the sweeps choose, shares no factor with a
/// cycle of days (the week, the thirty-day month, the sixty-day cycle), so
/// that the sample still visits every weekday and every day of the month.
/// The coverage job runs the debug build's tests one at a time under
/// instrumentation, and the release-mode job and the plain debug job keep
/// the sample they have (docs/policy.md §7).
pub const fn thinned(sampled: usize) -> usize {
    if INSTRUMENTED {
        prime_from(sampled * COVERAGE_THINNING)
    } else {
        sampled
    }
}

/// The step between the years of a year-by-year sweep that a debug build
/// samples every `sampled`th year of, in a build that is instrumented for
/// coverage: the first prime of at least twice as many, so that the step
/// is prime to the cycles the years run in (the nineteen-year Metonic
/// cycle, the thirty-year Hijri cycle, the sixty-year sexagenary cycle)
/// unless the cycle is that prime. In any other build it is `sampled`.
pub const fn thinned_year_step(sampled: usize) -> usize {
    if INSTRUMENTED {
        prime_from(sampled * 2)
    } else {
        sampled
    }
}

/// How many years apart the year boundaries are that a sweep checks in a
/// build instrumented for coverage, where a debug build checks every
/// year's: [`COVERAGE_YEAR_STEP`]. See [`year_step`].
pub const COVERAGE_YEAR_STEP: usize = 7;

/// The step between the years whose boundaries, the days a year or a
/// cycle begins, a sweep checks besides its sampled days: 1 in a build that
/// is not instrumented for coverage, and [`COVERAGE_YEAR_STEP`] in one that
/// is. Seven is prime to the cycles the years run in, the nineteen-year
/// Metonic cycle, the thirty-year Hijri cycle and the sixty-year sexagenary
/// cycle among them, so that the years still hold each kind of boundary the
/// sweep is about: leap years, common years and the exceptions.
pub const fn year_step() -> usize {
    if INSTRUMENTED { COVERAGE_YEAR_STEP } else { 1 }
}

/// Defines `check_days(days, check)` in the invoking crate's tests: run
/// `check` on each of `days`, spread over the machine's threads.
///
/// The threads take the days in consecutive chunks from a shared counter.
/// A chunk is about an eighth of one thread's share of the list, and at
/// most 256 days, so that a short list — a debug build's sample of a few
/// hundred days, or the new years of a few hundred years — is spread over
/// every thread rather than handed to the first few in two or three chunks
/// of 256, while a long one is taken 256 days at a time, and a
/// chunk that happens to be slow still leaves the other threads work to
/// take. Each day is checked exactly once, whatever the chunking, so the
/// outcome does not depend on it. A failing check panics its thread, and
/// the panic is raised again when the threads are joined. Without the
/// invoking crate's `std` feature the days are checked in turn.
///
/// Each chunk is checked inside one [`memo::scope`](crate::memo::scope),
/// as a call at the boundary is. A day's conversion and its conversion
/// back ask the same astronomy, and so do the days of one month, and a
/// year's first day and its eve, which a sweep's sorted list keeps side by
/// side. A scope changes how long a check takes and nothing else
/// (`hc_core::memo`), and it ends with its chunk, so the memo never holds
/// more than 256 days' values.
///
/// In a build instrumented for coverage ([`INSTRUMENTED`]) the days are
/// checked on one thread, in chunks of an eighth of the list, at most 256
/// days.
///
/// Invoke it once, at the root of a crate with a `std` feature:
/// `hc_core::check_days_in_parallel!();`, and call `crate::check_days`.
#[macro_export]
macro_rules! check_days_in_parallel {
    () => {
        /// How many consecutive days of a list of `len` a thread takes at
        /// a time over `threads` threads: an eighth of one thread's share,
        /// rounded up, and between 1 and 256.
        #[cfg(all(test, feature = "std"))]
        pub(crate) fn check_days_chunk(len: usize, threads: usize) -> usize {
            len.div_ceil(threads.max(1) * 8).clamp(1, 256)
        }

        /// Run `check` on each of `days`, spread over the machine's
        /// threads; see `hc_core::check_days_in_parallel!`.
        #[cfg(test)]
        pub(crate) fn check_days(days: &[i64], check: impl Fn(i64) + Sync) {
            #[cfg(feature = "std")]
            {
                use std::sync::atomic::{AtomicUsize, Ordering};
                let available = if $crate::sweep::INSTRUMENTED {
                    1
                } else {
                    std::thread::available_parallelism().map_or(1, usize::from)
                };
                let chunk = check_days_chunk(days.len(), available);
                // No more threads than there are chunks.
                let threads = available.min(days.len().div_ceil(chunk)).max(1);
                let next = AtomicUsize::new(0);
                std::thread::scope(|scope| {
                    for _ in 0..threads {
                        scope.spawn(|| {
                            loop {
                                let start = next.fetch_add(chunk, Ordering::Relaxed);
                                match days.get(start..days.len().min(start + chunk)) {
                                    Some(part) if !part.is_empty() => $crate::memo::scope(|| {
                                        part.iter().for_each(|&day| check(day))
                                    }),
                                    _ => break,
                                }
                            }
                        });
                    }
                });
            }
            #[cfg(not(feature = "std"))]
            days.iter().for_each(|&day| check(day));
        }
    };
}

// The threads need `std`; without it `check_days` is a plain loop, which
// the calendar crates' own sweeps exercise.
#[cfg(all(test, feature = "std"))]
mod tests {
    crate::check_days_in_parallel!();

    #[test]
    fn a_thinned_stride_is_a_prime_at_least_the_thinned_one() {
        use super::{COVERAGE_THINNING, INSTRUMENTED, prime_from, thinned, thinned_year_step};
        assert_eq!(prime_from(0), 2);
        assert_eq!(prime_from(1), 2);
        assert_eq!(prime_from(24), 29);
        assert_eq!(prime_from(37), 37);
        assert_eq!(prime_from(1_000), 1_009);
        for sampled in [1, 3, 7, 11, 97, 101, 319, 776, 1_013] {
            let (days, years) = (thinned(sampled), thinned_year_step(sampled));
            if INSTRUMENTED {
                assert!(days >= sampled * COVERAGE_THINNING && prime_from(days) == days);
                assert!(years >= sampled * 2 && prime_from(years) == years);
                // Days prime to the weekly, monthly, sexagenary and Metonic
                // cycles, years to the last three.
                for cycle in [7, 19, 30, 60] {
                    assert!(days % cycle != 0, "{sampled}: {days}");
                }
                for cycle in [19, 30, 60] {
                    assert!(years % cycle != 0, "{sampled}: {years}");
                }
            } else {
                assert_eq!((days, years), (sampled, sampled));
            }
        }
    }

    #[test]
    fn a_short_list_is_spread_over_every_thread_and_a_long_one_taken_256_at_a_time() {
        // 322 days over 14 threads: 322 / 112 is 2.9, so 108 chunks of 3,
        // where chunks of 256 would make two.
        assert_eq!(check_days_chunk(322, 14), 3);
        assert_eq!(check_days_chunk(868, 14), 8);
        assert_eq!(check_days_chunk(3_652_579, 14), 256);
        assert_eq!(check_days_chunk(1, 14), 1);
        assert_eq!(check_days_chunk(0, 14), 1);
        assert_eq!(check_days_chunk(10, 0), 2);
        assert_eq!(check_days_chunk(1_000, 1), 125);
    }

    #[test]
    fn every_day_is_checked_exactly_once() {
        use core::sync::atomic::{AtomicU8, Ordering};
        for len in [0_usize, 1, 7, 322, 5_000] {
            let seen: std::vec::Vec<AtomicU8> = (0..len).map(|_| AtomicU8::new(0)).collect();
            let days: std::vec::Vec<i64> = (0..len as i64).collect();
            check_days(&days, |day| {
                seen[day as usize].fetch_add(1, Ordering::Relaxed);
            });
            assert!(
                seen.iter().all(|count| count.load(Ordering::Relaxed) == 1),
                "{len}"
            );
        }
    }
}
