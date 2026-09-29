//! The marshalling every entry point shares: strings read from
//! NUL-terminated pointers, answers written into caller-owned buffers and
//! out-parameters, refusals turned into status codes, and the expansion of
//! `hyper_calendar::exports!`'s table into entry points.

use core::ffi::c_char;

use crate::{HC_ERROR_BUFFER_TOO_SMALL, HC_OK, HcStatus};

/// Write a string into a caller-owned buffer, NUL-terminated.
///
/// # Safety
///
/// `buffer` must be writable for `capacity` bytes, or `capacity` must be zero.
pub(crate) unsafe fn write_text(
    text: &str,
    buffer: *mut c_char,
    capacity: usize,
    written: *mut usize,
) -> HcStatus {
    let needed = text.len() + 1;
    if !written.is_null() {
        // SAFETY: the caller guarantees `written` is either null or writable,
        // and it was checked non-null just above.
        unsafe { *written = needed };
    }
    if buffer.is_null() || capacity < needed {
        return HC_ERROR_BUFFER_TOO_SMALL;
    }
    // SAFETY: `capacity >= needed == text.len() + 1`, so the copy and the
    // terminator both stay inside the caller's buffer.
    unsafe {
        core::ptr::copy_nonoverlapping(text.as_ptr().cast::<c_char>(), buffer, text.len());
        *buffer.add(text.len()) = 0;
    }
    HC_OK
}

/// A NUL-terminated string: `Ok(None)` for a null pointer and
/// [`HC_ERROR_NOT_UTF8`](crate::HC_ERROR_NOT_UTF8) for bytes that are not UTF-8.
///
/// # Safety
///
/// `pointer` must be null or point to a NUL-terminated string.
#[cfg(any(
    feature = "civil",
    feature = "deep-time",
    feature = "planetary",
    feature = "relativity",
    feature = "places",
    feature = "humanize",
    feature = "zone-names"
))]
pub(crate) unsafe fn text<'a>(pointer: *const c_char) -> Result<Option<&'a str>, HcStatus> {
    if pointer.is_null() {
        return Ok(None);
    }
    // SAFETY: the caller guarantees a NUL-terminated string.
    unsafe { core::ffi::CStr::from_ptr(pointer) }
        .to_str()
        .map(Some)
        .map_err(|_| crate::HC_ERROR_NOT_UTF8)
}

/// A NUL-terminated name the call needs: [`HC_ERROR_NULL_POINTER`](crate::HC_ERROR_NULL_POINTER) for a
/// null pointer and [`HC_ERROR_NOT_UTF8`](crate::HC_ERROR_NOT_UTF8)(crate::HC_ERROR_NOT_UTF8) for bytes that are not UTF-8.
///
/// # Safety
///
/// As [`text`].
#[cfg(any(
    feature = "timestamps",
    feature = "time-codes",
    feature = "calendars",
    feature = "seasons",
    feature = "holiday",
    feature = "tz",
    feature = "sky",
    feature = "planetary",
    feature = "relativity",
    feature = "places",
    feature = "humanize",
    feature = "zone-names"
))]
pub(crate) unsafe fn name<'a>(pointer: *const c_char) -> Result<&'a str, HcStatus> {
    // SAFETY: forwarded to the caller's contract above.
    unsafe { text(pointer) }?.ok_or(crate::HC_ERROR_NULL_POINTER)
}

/// The bytes a pointer and a length give: none for a null pointer or a
/// zero length.
///
/// # Safety
///
/// `pointer` must be readable for `len` bytes unless it is null or `len`
/// is zero.
#[cfg(feature = "tz")]
pub(crate) unsafe fn bytes<'a>(pointer: *const u8, len: usize) -> &'a [u8] {
    if pointer.is_null() || len == 0 {
        &[]
    } else {
        // SAFETY: the caller guarantees `pointer` is readable for `len`.
        unsafe { core::slice::from_raw_parts(pointer, len) }
    }
}

/// The status a refusal of the shared line-makers in `hyper_calendar` is.
pub(crate) const fn status(refusal: hc::boundary::Refusal) -> HcStatus {
    use hc::boundary::Refusal;
    match refusal {
        Refusal::OutOfRange => crate::HC_ERROR_OUT_OF_RANGE,
        Refusal::Overflow => crate::HC_ERROR_OVERFLOW,
        Refusal::NoData => crate::HC_ERROR_NO_DATA,
        Refusal::Unknown => crate::HC_ERROR_UNKNOWN,
        Refusal::Malformed => crate::HC_ERROR_MALFORMED,
        Refusal::InvalidDate => crate::HC_ERROR_INVALID_DATE,
    }
}

/// A shared line-maker's answer, written into a caller-owned buffer as
/// [`write_text`] does, or its refusal as a status.
///
/// # Safety
///
/// As [`write_text`].
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
    feature = "places",
    feature = "humanize",
    feature = "zone-names"
))]
pub(crate) unsafe fn write_answer(
    answer: hc::boundary::Answer<String>,
    buffer: *mut c_char,
    capacity: usize,
    written: *mut usize,
) -> HcStatus {
    match answer {
        // SAFETY: forwarded to the caller's contract above.
        Ok(text) => unsafe { write_text(&text, buffer, capacity, written) },
        Err(refusal) => status(refusal),
    }
}

/// The C type of an argument kind of `hyper_calendar::exports!`'s table.
macro_rules! c_type {
    (name) => { *const core::ffi::c_char };
    (text) => { *const core::ffi::c_char };
    (opt) => { *const core::ffi::c_char };
    (flag) => { core::ffi::c_int };
    (int) => { core::ffi::c_int };
    ($ty:ident) => { $ty };
}

/// An argument of a kind, read, returning its status from the entry point
/// when it cannot be.
macro_rules! c_read {
    (name, $arg:ident) => {
        // SAFETY: the caller's contract, which the entry point's Safety
        // section states for the argument.
        match unsafe { $crate::marshal::name($arg) } {
            Ok(value) => value,
            Err(status) => return status,
        }
    };
    (text, $arg:ident) => {
        // SAFETY: as for `name`.
        match unsafe { $crate::marshal::text($arg) } {
            Ok(value) => value.unwrap_or(""),
            Err(status) => return status,
        }
    };
    (opt, $arg:ident) => {
        // SAFETY: as for `name`.
        match unsafe { $crate::marshal::text($arg) } {
            Ok(value) => value.filter(|value| !value.is_empty()),
            Err(status) => return status,
        }
    };
    (flag, $arg:ident) => {
        $arg != 0
    };
    ($ty:ident, $arg:ident) => {
        $arg
    };
}

/// What the Safety section says of an argument of a kind.
macro_rules! c_safety {
    (name, $arg:ident) => {
        concat!(
            "`",
            stringify!($arg),
            "` must be null or point to a NUL-terminated string. "
        )
    };
    (text, $arg:ident) => {
        c_safety!(name, $arg)
    };
    (opt, $arg:ident) => {
        c_safety!(name, $arg)
    };
    ($ty:ident, $arg:ident) => {
        ""
    };
}

/// The entry points of rows of `hyper_calendar::exports!`'s table: a
/// `line` row writes its answer into a caller-owned buffer, NUL-terminated,
/// with the length it needs in `written`; a `value` row writes its number
/// to the out-parameter it names, refusing a null one first.
macro_rules! c_exports {
    ($(
        c { $(#[$c:meta])* }
        wasm { $(#[$w:meta])* }
        fn $name:ident($($arg:ident: $kind:ident $(($len:ident))?),* $(,)?)
            -> $shape:ident $(($out:ident: $out_kind:ident))? = $line:expr;
    )*) => {
        $(c_exports!(@export [$(#[$c])*] $name [$($arg: $kind),*] $shape $(($out: $out_kind))? = $line);)*
    };
    (@export [$($doc:tt)*] $name:ident [$($arg:ident: $kind:ident),*] line = $line:expr) => {
        $($doc)*
        ///
        /// # Safety
        ///
        #[doc = concat!(
            $(c_safety!($kind, $arg),)*
            "`buffer` must be writable for `capacity` bytes and `written` must be null or writable."
        )]
        #[unsafe(no_mangle)]
        #[allow(clippy::too_many_arguments)]
        pub unsafe extern "C" fn $name(
            $($arg: c_type!($kind),)*
            buffer: *mut core::ffi::c_char,
            capacity: usize,
            written: *mut usize,
        ) -> $crate::HcStatus {
            $(let $arg = c_read!($kind, $arg);)*
            let answer = ($line)($($arg),*);
            // SAFETY: forwarded to the caller's contract above.
            unsafe { $crate::marshal::write_answer(answer, buffer, capacity, written) }
        }
    };
    (@export [$($doc:tt)*] $name:ident [$($arg:ident: $kind:ident),*]
        value($out:ident: $out_kind:ident) = $line:expr) => {
        $($doc)*
        ///
        /// # Safety
        ///
        #[doc = concat!(
            $(c_safety!($kind, $arg),)*
            "`", stringify!($out), "` must be null or writable."
        )]
        #[unsafe(no_mangle)]
        #[allow(clippy::too_many_arguments)]
        pub unsafe extern "C" fn $name(
            $($arg: c_type!($kind),)*
            $out: *mut c_type!($out_kind),
        ) -> $crate::HcStatus {
            if $out.is_null() {
                return $crate::HC_ERROR_NULL_POINTER;
            }
            $(let $arg = c_read!($kind, $arg);)*
            match ($line)($($arg),*) {
                Ok(value) => {
                    // SAFETY: checked non-null above; the caller guarantees
                    // it is writable.
                    unsafe { *$out = <c_type!($out_kind)>::from(value) };
                    $crate::HC_OK
                }
                Err(refusal) => $crate::marshal::status(refusal),
            }
        }
    };
}
