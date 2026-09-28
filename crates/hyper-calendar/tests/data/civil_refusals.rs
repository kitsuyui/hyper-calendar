// The inputs the civil exports refuse, each with the refusal it is.
//
// Both boundary crates include this file in their tests and check that
// their export answers with their own code for that refusal: the C
// library's `HcStatus`, the WebAssembly module's sentinel. The refusal is
// `hyper_calendar::boundary`'s conversion of the library's error, written
// once, so one input refuses the same way on both sides. The table is the
// civil layer's because its exports are the ones that call the library
// directly; every other export answers through a shared line-maker, which
// refuses with a `Refusal` already.

/// A call of a civil export, by its arguments.
#[derive(Debug, Clone, Copy)]
enum CivilCall {
    /// `hc_gregorian_to_fixed(year, month, day)`.
    GregorianToFixed(i64, u8, u8),
    /// The Gregorian date of a fixed day: `hc_gregorian_from_fixed` in the
    /// library, `hc_gregorian_year`, `_month` and `_day` in the module.
    GregorianFromFixed(i64),
    /// `hc_day_of_year(fixed)`.
    DayOfYear(i64),
    /// `hc_is_leap_year(fixed)`.
    IsLeapYear(i64),
    /// `hc_format_iso_date(fixed)`.
    FormatIsoDate(i64),
    /// `hc_tai_minus_utc(unix_seconds, strict)`.
    TaiMinusUtc(i64, bool),
}

/// The first fixed day after the Gregorian years −9 999 999 to 9 999 999.
const AFTER_GREGORIAN_RANGE: i64 = 3_652_424_635;

/// The last fixed day before them.
const BEFORE_GREGORIAN_RANGE: i64 = -3_652_425_000;

/// Every refused call and its refusal.
const CIVIL_REFUSALS: &[(CivilCall, hc::boundary::Refusal)] = {
    use hc::boundary::Refusal::{InvalidDate, NoData, OutOfRange};
    use CivilCall::*;
    &[
        (GregorianToFixed(2026, 2, 30), InvalidDate),
        (GregorianToFixed(2026, 13, 1), InvalidDate),
        (GregorianToFixed(2026, 0, 1), InvalidDate),
        (GregorianToFixed(10_000_000, 1, 1), InvalidDate),
        (GregorianFromFixed(AFTER_GREGORIAN_RANGE), OutOfRange),
        (GregorianFromFixed(BEFORE_GREGORIAN_RANGE), OutOfRange),
        (GregorianFromFixed(i64::MAX), OutOfRange),
        (DayOfYear(AFTER_GREGORIAN_RANGE), OutOfRange),
        (DayOfYear(i64::MIN), OutOfRange),
        (IsLeapYear(AFTER_GREGORIAN_RANGE), OutOfRange),
        (IsLeapYear(BEFORE_GREGORIAN_RANGE), OutOfRange),
        (FormatIsoDate(AFTER_GREGORIAN_RANGE), OutOfRange),
        (FormatIsoDate(BEFORE_GREGORIAN_RANGE), OutOfRange),
        // 1 January 1960, before the table, and 2200, past it: strict.
        (TaiMinusUtc(-315_619_200, true), NoData),
        (TaiMinusUtc(7_258_118_400, true), NoData),
    ]
};
