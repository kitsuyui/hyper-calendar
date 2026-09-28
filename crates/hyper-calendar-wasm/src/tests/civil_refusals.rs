//! The refusals of the civil exports, from the table the C library's
//! tests read too.

use super::super::*;
use crate::marshal::sentinel;
use hc::boundary::Refusal;

include!("../../../hyper-calendar/tests/data/civil_refusals.rs");

/// The sentinels a call answers with: one per export it names.
fn sentinels(call: CivilCall) -> Vec<i64> {
    let mut buffer = [0u8; 32];
    match call {
        CivilCall::GregorianToFixed(year, month, day) => {
            vec![hc_gregorian_to_fixed(
                year,
                u32::from(month),
                u32::from(day),
            )]
        }
        CivilCall::GregorianFromFixed(on) => vec![
            hc_gregorian_year(on),
            hc_gregorian_month(on),
            hc_gregorian_day(on),
        ],
        CivilCall::DayOfYear(on) => vec![hc_day_of_year(on)],
        CivilCall::IsLeapYear(on) => vec![hc_is_leap_year(on)],
        CivilCall::FormatIsoDate(on) => {
            vec![unsafe { hc_format_iso_date(on, buffer.as_mut_ptr(), buffer.len()) }]
        }
        CivilCall::TaiMinusUtc(unix, strict) => {
            vec![hc_tai_minus_utc(unix, i32::from(strict))]
        }
    }
}

/// Each input of the table is refused with the sentinel of its
/// refusal, which the library gives as the status of the same one.
#[test]
fn every_civil_refusal_is_the_shared_one() {
    for &(call, refusal) in CIVIL_REFUSALS {
        for got in sentinels(call) {
            assert_eq!(got, sentinel(refusal), "{call:?}");
        }
    }
}

/// The sentinel of each refusal, as `hyper_calendar::boundary`
/// documents it: an overflow is out of range, since the module has
/// no sentinel of its own for it.
#[test]
fn each_refusal_has_its_documented_sentinel() {
    for (refusal, code) in [
        (Refusal::OutOfRange, HC_ERR_OUT_OF_RANGE),
        (Refusal::Overflow, HC_ERR_OUT_OF_RANGE),
        (Refusal::NoData, HC_ERR_NO_DATA),
        (Refusal::Unknown, HC_ERR_UNKNOWN),
        (Refusal::Malformed, HC_ERR_MALFORMED),
        (Refusal::InvalidDate, HC_ERR_INVALID_DATE),
    ] {
        assert_eq!(sentinel(refusal), code, "{refusal:?}");
    }
}
