//! The marshalling every export shares: strings and bytes read from linear
//! memory, answers copied into a caller's buffer or returned as numbers,
//! refusals turned into sentinels, and the expansion of
//! `hyper_calendar::exports!`'s table into exports.

use crate::{
    HC_ERR_BUFFER_TOO_SMALL, HC_ERR_INVALID_DATE, HC_ERR_MALFORMED, HC_ERR_NO_DATA,
    HC_ERR_OUT_OF_RANGE, HC_ERR_UNKNOWN,
};

/// UTF-8 text from linear memory. A null pointer with a zero length is the
/// empty string.
///
/// # Errors
///
/// [`HC_ERR_NULL_POINTER`](crate::HC_ERR_NULL_POINTER) for a null pointer
/// with a non-zero length and [`HC_ERR_NOT_UTF8`](crate::HC_ERR_NOT_UTF8) for
/// bytes that are not UTF-8.
///
/// # Safety
///
/// `pointer` must be readable for `len` bytes unless it is null.
#[cfg(any(
    feature = "civil",
    feature = "deep-time",
    feature = "planetary",
    feature = "relativity",
    feature = "places"
))]
pub(crate) unsafe fn text<'a>(pointer: *const u8, len: usize) -> Result<&'a str, i64> {
    if pointer.is_null() {
        return if len == 0 {
            Ok("")
        } else {
            Err(crate::HC_ERR_NULL_POINTER)
        };
    }
    // SAFETY: the caller guarantees `pointer` is readable for `len` bytes.
    let bytes = unsafe { core::slice::from_raw_parts(pointer, len) };
    core::str::from_utf8(bytes).map_err(|_| crate::HC_ERR_NOT_UTF8)
}

/// The bytes a pointer and a length give in linear memory: none for a
/// null pointer, whatever the length.
///
/// # Safety
///
/// `pointer` must be readable for `len` bytes unless it is null.
#[cfg(feature = "tz")]
pub(crate) unsafe fn bytes<'a>(pointer: *const u8, len: usize) -> &'a [u8] {
    if pointer.is_null() {
        &[]
    } else {
        // SAFETY: the caller guarantees `pointer` is readable for `len`.
        unsafe { core::slice::from_raw_parts(pointer, len) }
    }
}

/// A computed value as a value-returning export may return it: the value
/// when there is one above [`HC_ERR_FLOOR`](crate::HC_ERR_FLOOR), and [`HC_ERR_OUT_OF_RANGE`]
/// when the arithmetic overflowed (`None`) or the value would read as a
/// sentinel.
///
/// This is the one place the rule is kept: no legitimate result is ever at
/// or below [`HC_ERR_FLOOR`](crate::HC_ERR_FLOOR).
#[cfg(any(feature = "civil", feature = "planetary"))]
pub(crate) fn above_floor(value: Option<i64>) -> Result<i64, i64> {
    value
        .filter(|value| *value > crate::HC_ERR_FLOOR)
        .ok_or(HC_ERR_OUT_OF_RANGE)
}

/// Copy `text` into the caller's buffer, returning the byte length written.
///
/// # Safety
///
/// `buffer` must be writable for `capacity` bytes.
pub(crate) unsafe fn emit(text: &str, buffer: *mut u8, capacity: usize) -> i64 {
    if buffer.is_null() || capacity < text.len() {
        return HC_ERR_BUFFER_TOO_SMALL;
    }
    // SAFETY: `capacity >= text.len()`, so the copy stays in the buffer.
    unsafe { core::ptr::copy_nonoverlapping(text.as_ptr(), buffer, text.len()) };
    text.len() as i64
}

/// Copy `text` into the caller's buffer, or measure it for a null buffer.
///
/// This is the contract of every export that writes lines: a null `buffer`
/// asks for the length the text needs, a buffer that is too small is
/// refused untouched, and otherwise the byte length written comes back.
///
/// # Safety
///
/// `buffer` must be writable for `capacity` bytes unless it is null.
#[cfg(any(
    feature = "timestamps",
    feature = "time-codes",
    feature = "calendars",
    feature = "seasons",
    feature = "holiday",
    feature = "deep-time",
    feature = "tz",
    feature = "sky",
    feature = "orbital",
    feature = "planetary",
    feature = "relativity",
    feature = "places"
))]
pub(crate) unsafe fn emit_or_measure(text: &str, buffer: *mut u8, capacity: usize) -> i64 {
    if buffer.is_null() {
        return text.len() as i64;
    }
    // SAFETY: forwarded to the caller's contract above.
    unsafe { emit(text, buffer, capacity) }
}

/// The sentinel a refusal of the shared line-makers in `hyper_calendar`
/// is: an overflow is [`HC_ERR_OUT_OF_RANGE`], since the module has no
/// sentinel of its own for it.
pub(crate) const fn sentinel(refusal: hc::boundary::Refusal) -> i64 {
    use hc::boundary::Refusal;
    match refusal {
        Refusal::OutOfRange | Refusal::Overflow => HC_ERR_OUT_OF_RANGE,
        Refusal::NoData => HC_ERR_NO_DATA,
        Refusal::Unknown => HC_ERR_UNKNOWN,
        Refusal::Malformed => HC_ERR_MALFORMED,
        Refusal::InvalidDate => HC_ERR_INVALID_DATE,
    }
}

/// A shared line-maker's answer, measured or copied into the caller's
/// buffer as [`emit_or_measure`] does, or its refusal as a sentinel.
///
/// # Safety
///
/// As [`emit_or_measure`].
#[cfg(any(
    feature = "timestamps",
    feature = "time-codes",
    feature = "calendars",
    feature = "seasons",
    feature = "holiday",
    feature = "deep-time",
    feature = "tz",
    feature = "sky",
    feature = "orbital",
    feature = "planetary",
    feature = "relativity",
    feature = "places"
))]
pub(crate) unsafe fn emit_answer(
    answer: hc::boundary::Answer<String>,
    buffer: *mut u8,
    capacity: usize,
) -> i64 {
    match answer {
        // SAFETY: forwarded to the caller's contract above.
        Ok(text) => unsafe { emit_or_measure(&text, buffer, capacity) },
        Err(refusal) => sentinel(refusal),
    }
}

/// A shared number as a value-returning export returns it: the number
/// when it is above [`HC_ERR_FLOOR`](crate::HC_ERR_FLOOR), else a sentinel.
#[cfg(any(feature = "civil", feature = "planetary"))]
pub(crate) fn value(answer: hc::boundary::Answer<i64>) -> i64 {
    match answer
        .map_err(sentinel)
        .and_then(|value| above_floor(Some(value)))
    {
        Ok(value) | Err(value) => value,
    }
}

/// The WebAssembly type of an argument kind of `hyper_calendar::exports!`'s
/// table; a string's length is a `usize` beside it.
macro_rules! w_type {
    (name) => { *const u8 };
    (text) => { *const u8 };
    (opt) => { *const u8 };
    (flag) => { i32 };
    (int) => { i32 };
    ($ty:ident) => { $ty };
}

/// An argument of a kind, read, returning its sentinel from the export
/// when it cannot be.
macro_rules! w_read {
    (name, $arg:ident, $len:ident) => {
        // SAFETY: the caller's contract, which the export's Safety section
        // states for the argument.
        match unsafe { $crate::marshal::text($arg, $len) } {
            Ok(value) => value,
            Err(sentinel) => return sentinel,
        }
    };
    (text, $arg:ident, $len:ident) => {
        w_read!(name, $arg, $len)
    };
    (opt, $arg:ident, $len:ident) => {{
        let value = w_read!(name, $arg, $len);
        (!value.is_empty()).then_some(value)
    }};
    (flag, $arg:ident) => {
        $arg != 0
    };
    ($ty:ident, $arg:ident) => {
        $arg
    };
}

/// What the Safety section says of an argument of a kind.
#[cfg(any(
    feature = "timestamps",
    feature = "time-codes",
    feature = "calendars",
    feature = "seasons",
    feature = "holiday",
    feature = "deep-time",
    feature = "tz",
    feature = "sky",
    feature = "orbital",
    feature = "planetary",
    feature = "relativity",
    feature = "places"
))]
macro_rules! w_safety {
    (name, $arg:ident, $len:ident) => {
        concat!(
            "`",
            stringify!($arg),
            "` must be readable for `",
            stringify!($len),
            "` bytes unless null with a zero length. "
        )
    };
    (text, $arg:ident, $len:ident) => {
        w_safety!(name, $arg, $len)
    };
    (opt, $arg:ident, $len:ident) => {
        w_safety!(name, $arg, $len)
    };
    ($ty:ident, $arg:ident) => {
        ""
    };
}

/// The exports of rows of `hyper_calendar::exports!`'s table: a `line` row
/// copies its answer into a caller's buffer, or measures it for a null
/// one, and returns its length; a `value` row returns its number. Either
/// returns a sentinel for a refusal.
macro_rules! w_exports {
    ($(
        c { $(#[$c:meta])* }
        wasm { $(#[$w:meta])* }
        fn $name:ident($($arg:ident: $kind:ident $(($len:ident))?),* $(,)?)
            -> $shape:ident $(($out:ident: $out_kind:ident))? = $line:expr;
    )*) => {
        $(w_exports!(@export [$(#[$w])*] $name [$($arg: $kind $(($len))?),*] $shape = $line);)*
    };
    (@export [$($doc:tt)*] $name:ident [$($arg:ident: $kind:ident $(($len:ident))?),*] line = $line:expr) => {
        $($doc)*
        ///
        /// # Safety
        ///
        #[doc = concat!(
            $(w_safety!($kind, $arg $(, $len)?),)*
            "`buffer` must be writable for `capacity` bytes unless it is null."
        )]
        #[unsafe(no_mangle)]
        #[allow(clippy::too_many_arguments)]
        pub unsafe extern "C" fn $name(
            $($arg: w_type!($kind), $($len: usize,)?)*
            buffer: *mut u8,
            capacity: usize,
        ) -> i64 {
            $(let $arg = w_read!($kind, $arg $(, $len)?);)*
            let answer = ($line)($($arg),*);
            // SAFETY: forwarded to the caller's contract above.
            unsafe { $crate::marshal::emit_answer(answer, buffer, capacity) }
        }
    };
    // A value export takes a pointer, and is `unsafe`, only when it reads a
    // string: the kinds are scanned for one.
    (@export [$($doc:tt)*] $name:ident [$($arg:ident: $kind:ident $(($len:ident))?),*] value = $line:expr) => {
        w_exports!(@value [$($kind)*] [$($doc)*] $name [$($arg: $kind $(($len))?),*] = $line);
    };
    (@value [name $($rest:ident)*] $($tail:tt)*) => { w_exports!(@unsafe_value $($tail)*); };
    (@value [text $($rest:ident)*] $($tail:tt)*) => { w_exports!(@unsafe_value $($tail)*); };
    (@value [opt $($rest:ident)*] $($tail:tt)*) => { w_exports!(@unsafe_value $($tail)*); };
    (@value [$kind:ident $($rest:ident)*] $($tail:tt)*) => { w_exports!(@value [$($rest)*] $($tail)*); };
    (@value [] $($tail:tt)*) => { w_exports!(@safe_value $($tail)*); };
    (@unsafe_value [$($doc:tt)*] $name:ident [$($arg:ident: $kind:ident $(($len:ident))?),*] = $line:expr) => {
        $($doc)*
        ///
        /// # Safety
        ///
        #[doc = concat!($(w_safety!($kind, $arg $(, $len)?),)*)]
        #[unsafe(no_mangle)]
        #[allow(clippy::too_many_arguments)]
        pub unsafe extern "C" fn $name($($arg: w_type!($kind), $($len: usize,)?)*) -> i64 {
            $(let $arg = w_read!($kind, $arg $(, $len)?);)*
            $crate::marshal::value(($line)($($arg),*).map(i64::from))
        }
    };
    (@safe_value [$($doc:tt)*] $name:ident [$($arg:ident: $kind:ident),*] = $line:expr) => {
        $($doc)*
        #[unsafe(no_mangle)]
        pub extern "C" fn $name($($arg: w_type!($kind)),*) -> i64 {
            $(let $arg = w_read!($kind, $arg);)*
            $crate::marshal::value(($line)($($arg),*).map(i64::from))
        }
    };
}
