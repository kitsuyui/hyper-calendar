//! The refusals of the civil entry points, from the table the
//! WebAssembly module's tests read too.

use super::super::*;
use crate::marshal::status;
use hc::boundary::Refusal;

include!("../../../hyper-calendar/tests/data/civil_refusals.rs");

/// The statuses a call answers with: one per entry point it names.
fn statuses(call: CivilCall) -> Vec<HcStatus> {
    let (mut fixed, mut year, mut month, mut day) = (0i64, 0i64, 0u8, 0u8);
    let (mut day_of_year, mut leap, mut offset) = (0u32, 0 as core::ffi::c_int, 0i64);
    let mut buffer = [0 as core::ffi::c_char; 32];
    let mut written = 0usize;
    match call {
        CivilCall::GregorianToFixed(y, m, d) => {
            vec![unsafe { hc_gregorian_to_fixed(y, m, d, &mut fixed) }]
        }
        CivilCall::GregorianFromFixed(on) => {
            vec![unsafe { hc_gregorian_from_fixed(on, &mut year, &mut month, &mut day) }]
        }
        CivilCall::DayOfYear(on) => vec![unsafe { hc_day_of_year(on, &mut day_of_year) }],
        CivilCall::IsLeapYear(on) => vec![unsafe { hc_is_leap_year(on, &mut leap) }],
        CivilCall::FormatIsoDate(on) => {
            vec![unsafe { hc_format_iso_date(on, buffer.as_mut_ptr(), buffer.len(), &mut written) }]
        }
        CivilCall::TaiMinusUtc(unix, strict) => {
            vec![unsafe { hc_tai_minus_utc(unix, core::ffi::c_int::from(strict), &mut offset) }]
        }
    }
}

/// Each input of the table is refused with the status of its
/// refusal, which the module gives as the sentinel of the same one.
#[test]
fn every_civil_refusal_is_the_shared_one() {
    for &(call, refusal) in CIVIL_REFUSALS {
        for got in statuses(call) {
            assert_eq!(got, status(refusal), "{call:?}");
        }
    }
}

/// The status of each refusal, as `hyper_calendar::boundary`
/// documents it.
#[test]
fn each_refusal_has_its_documented_status() {
    for (refusal, code) in [
        (Refusal::OutOfRange, HC_ERROR_OUT_OF_RANGE),
        (Refusal::Overflow, HC_ERROR_OVERFLOW),
        (Refusal::NoData, HC_ERROR_NO_DATA),
        (Refusal::Unknown, HC_ERROR_UNKNOWN),
        (Refusal::Malformed, HC_ERROR_MALFORMED),
        (Refusal::InvalidDate, HC_ERROR_INVALID_DATE),
    ] {
        assert_eq!(status(refusal), code, "{refusal:?}");
    }
}
