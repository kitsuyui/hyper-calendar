//! The helper the calendar crates' day-by-day sweeps share, written once.
//!
//! A round-trip sweep (docs/policy.md §7) checks every day of a range, or a
//! sample of it, and no day's check depends on another's, so the days are
//! spread over the machine's threads.
//! [`check_days_in_parallel!`](crate::check_days_in_parallel) defines the
//! function that does it, `check_days`, in the tests of the crate that
//! invokes it, so that each crate's `std` feature decides whether threads
//! are used and nothing of it is compiled outside a test build.

/// Defines `check_days(days, check)` in the invoking crate's tests: run
/// `check` on each of `days`, spread over the machine's threads.
///
/// The threads take the days in consecutive chunks from a shared counter.
/// A chunk is about an eighth of one thread's share of the list, and at
/// most 256 days, so that a short list — a debug build's sample of a few
/// hundred days, or the new years of a few hundred years — is spread over
/// every thread rather than handed to the first few in two or three chunks
/// of 256, while a long one is taken 256 days at a time, as before, and a
/// chunk that happens to be slow still leaves the other threads work to
/// take. Each day is checked exactly once, whatever the chunking, so the
/// outcome does not depend on it. A failing check panics its thread, and
/// the panic is raised again when the threads are joined. Without the
/// invoking crate's `std` feature the days are checked in turn.
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
                let available = std::thread::available_parallelism().map_or(1, usize::from);
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
                                    Some(part) if !part.is_empty() => {
                                        part.iter().for_each(|&day| check(day))
                                    }
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
    fn a_short_list_is_spread_over_every_thread_and_a_long_one_taken_256_at_a_time() {
        // 322 days over 14 threads: 322 / 112 is 2.9, so chunks of 3 and
        // 108 of them, where chunks of 256 made two.
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
