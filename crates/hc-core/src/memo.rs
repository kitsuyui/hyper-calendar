//! A memo of pure functions, for the length of one call.
//!
//! Describing one day in every calendar asks the same astronomy many
//! times. Seven calendars read the Hindu lunar date at the same sunrise,
//! each converting from scratch; the observational Hebrew calendar finds
//! the same first of Nisan for its date, for whether its year is leap, and
//! again for the month's name. Every one of those is a pure function of its
//! arguments — a sunrise of a day at a place, the conjunction after a
//! moment — so a caller that is about to ask many questions of one day can
//! open a [`scope`], inside which [`cached`] answers each question once and
//! hands the same value back when it is asked again.
//!
//! Nothing is shared between scopes: the memo is emptied when the outermost
//! scope ends, so memory lasts as long as the call and a later call
//! computes afresh. Outside a scope, in a build without `std`, where there
//! is no thread-local storage to keep the memo in, and in a build without
//! the `memo` feature, which leaves the memo's code out of a WebAssembly
//! layer that never opens a scope, [`cached`] computes every time. So a scope changes how long a call takes and nothing else:
//! a value handed back is the one the function computed for the same key.
//!
//! That holds only if the key is everything the value depends on. A key is
//! the arguments themselves, as up to [`KEY_WORDS`] 64-bit words:
//! integers as they are, floating-point numbers by their bits
//! ([`f64::to_bits`]), never a rounding of them, since searches that start a
//! hair apart can end a hair apart, and a hair is what decides whether a
//! conjunction fell before a sunrise.
//!
//! The memo is one table for every function, keyed by the function's tag
//! and the words, its values boxed: a table per key and value type would be
//! compiled once per memoized function, and the WebAssembly module pays for
//! every copy in bytes a page downloads.

/// The most words a key may have.
pub const KEY_WORDS: usize = 12;

/// Run `body` with the memo open, then empty it, unless a scope further
/// out is already open, in which case `body` shares that one.
pub fn scope<R>(body: impl FnOnce() -> R) -> R {
    #[cfg(all(feature = "memo", feature = "std"))]
    {
        let _open = active::Open::new();
        body()
    }
    #[cfg(not(all(feature = "memo", feature = "std")))]
    {
        body()
    }
}

/// The value `compute` gives for `key`, computed once per [`scope`].
///
/// `Tag` names the function, so that two functions with keys of the same
/// words keep their values apart: an uninhabited `enum` declared beside the
/// call is enough. `compute` must be a pure function of `key`.
pub fn cached<Tag: 'static, V, const N: usize>(key: [u64; N], compute: impl FnOnce() -> V) -> V
where
    V: Clone + 'static,
{
    const { assert!(N <= KEY_WORDS, "a memo key has at most KEY_WORDS words") };
    #[cfg(all(feature = "memo", feature = "std"))]
    {
        let mut words = [0; KEY_WORDS];
        words[..N].copy_from_slice(&key);
        // The length is part of the key, so that keys of different lengths
        // padded to the same words stay apart.
        active::cached::<V>((core::any::TypeId::of::<Tag>(), N, words), compute)
    }
    #[cfg(not(all(feature = "memo", feature = "std")))]
    {
        let _ = key;
        compute()
    }
}

/// Whether a [`scope`] is open on this thread.
#[must_use]
pub fn is_open() -> bool {
    #[cfg(all(feature = "memo", feature = "std"))]
    {
        active::is_open()
    }
    #[cfg(not(all(feature = "memo", feature = "std")))]
    {
        false
    }
}

#[cfg(all(feature = "memo", feature = "std"))]
mod active {
    use std::any::{Any, TypeId};
    use std::boxed::Box;
    use std::cell::RefCell;
    use std::collections::BTreeMap;

    use super::KEY_WORDS;

    /// A function's tag, the length of its key, and the key's words.
    pub(super) type Key = (TypeId, usize, [u64; KEY_WORDS]);

    /// Every value computed in the open scope.
    type Values = BTreeMap<Key, Box<dyn Any>>;

    std::thread_local! {
        /// The open memo, or `None` outside every scope.
        static MEMO: RefCell<Option<Values>> = const { RefCell::new(None) };
    }

    /// An open scope: the outermost one empties the memo when it ends,
    /// however it ends.
    pub(super) struct Open {
        outermost: bool,
    }

    impl Open {
        pub(super) fn new() -> Self {
            let outermost = MEMO.with(|memo| {
                let mut memo = memo.borrow_mut();
                if memo.is_some() {
                    false
                } else {
                    *memo = Some(Values::new());
                    true
                }
            });
            Self { outermost }
        }
    }

    impl Drop for Open {
        fn drop(&mut self) {
            if self.outermost {
                // Taken out first, so that the values are dropped with the
                // memo released.
                let values = MEMO.with(|memo| memo.borrow_mut().take());
                drop(values);
            }
        }
    }

    pub(super) fn is_open() -> bool {
        MEMO.with(|memo| memo.borrow().is_some())
    }

    /// Whether a scope is open, and the value under `key` if one is kept.
    fn find(key: &Key, read: &mut dyn FnMut(&dyn Any)) -> bool {
        MEMO.with(|memo| {
            let memo = memo.borrow();
            let Some(values) = memo.as_ref() else {
                return false;
            };
            if let Some(value) = values.get(key) {
                read(value.as_ref());
            }
            true
        })
    }

    /// Keep `value` under `key`, if a scope is still open.
    fn keep(key: Key, value: Box<dyn Any>) {
        MEMO.with(|memo| {
            if let Some(values) = memo.borrow_mut().as_mut() {
                values.insert(key, value);
            }
        });
    }

    pub(super) fn cached<V: Clone + 'static>(key: Key, compute: impl FnOnce() -> V) -> V {
        // The memo is borrowed only to look and to keep, never while
        // `compute` runs: it may well ask the memo questions of its own.
        let mut found = None;
        let open = find(&key, &mut |value| {
            found = value.downcast_ref::<V>().cloned();
        });
        if let Some(value) = found {
            return value;
        }
        let value = compute();
        if open {
            keep(key, Box::new(value.clone()));
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::Cell;

    enum Square {}

    fn square(calls: &Cell<u32>, x: i64) -> i64 {
        cached::<Square, _, 1>([x as u64], || {
            calls.set(calls.get() + 1);
            x * x
        })
    }

    #[test]
    fn outside_a_scope_every_call_computes() {
        let calls = Cell::new(0);
        assert!(!is_open());
        assert_eq!(square(&calls, 3), 9);
        assert_eq!(square(&calls, 3), 9);
        assert_eq!(calls.get(), 2);
    }

    #[cfg(all(feature = "memo", feature = "std"))]
    #[test]
    fn inside_a_scope_a_key_is_computed_once_and_forgotten_after() {
        let calls = Cell::new(0);
        scope(|| {
            assert!(is_open());
            assert_eq!(square(&calls, 3), 9);
            assert_eq!(square(&calls, 3), 9);
            assert_eq!(square(&calls, 4), 16);
            // A nested scope shares the memo and does not empty it.
            scope(|| assert_eq!(square(&calls, 4), 16));
            assert_eq!(square(&calls, 4), 16);
        });
        assert_eq!(calls.get(), 2);
        assert!(!is_open());
        scope(|| assert_eq!(square(&calls, 3), 9));
        assert_eq!(calls.get(), 3);
    }

    #[cfg(all(feature = "memo", feature = "std"))]
    #[test]
    fn tags_keep_values_apart_and_a_computation_may_ask_the_memo() {
        enum Cube {}
        scope(|| {
            let calls = Cell::new(0);
            let cube = |x: i64| cached::<Cube, _, 1>([x as u64], || square(&calls, x) * x);
            assert_eq!(cube(2), 8);
            assert_eq!(square(&calls, 2), 4);
            assert_eq!(cube(2), 8);
            assert_eq!(calls.get(), 1);
        });
    }
}
