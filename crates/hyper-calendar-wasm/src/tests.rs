use super::*;

/// A string argument as the pointer and length the module takes.
#[allow(unused_macros)]
macro_rules! s {
    ($text:expr) => {
        ($text.as_ptr(), $text.len())
    };
}

#[test]
fn allocation_round_trips() {
    let pointer = hc_alloc(32);
    assert!(!pointer.is_null());
    unsafe { hc_free(pointer, 32) };
    assert!(hc_alloc(0).is_null());
    // Freeing null or zero length is a no-op rather than a fault.
    unsafe { hc_free(core::ptr::null_mut(), 0) };
}

/// `hc_version` cannot measure: a null or small buffer is refused and a
/// sized one receives the version.
#[test]
fn the_version_is_the_library_s() {
    assert_eq!(
        unsafe { hc_version(core::ptr::null_mut(), 0) },
        HC_ERR_BUFFER_TOO_SMALL
    );
    let capacity = hc::VERSION.len();
    let pointer = hc_alloc(capacity);
    assert_eq!(
        unsafe { hc_version(pointer, capacity - 1) },
        HC_ERR_BUFFER_TOO_SMALL
    );
    assert_eq!(unsafe { hc_version(pointer, capacity) }, capacity as i64);
    let text = unsafe { core::str::from_utf8(core::slice::from_raw_parts(pointer, capacity)) }
        .expect("UTF-8")
        .to_owned();
    unsafe { hc_free(pointer, capacity) };
    assert_eq!(text, hc::VERSION);
}

#[test]
fn the_error_floor_is_far_below_any_real_day_number() {
    // The universe is about 5e12 days old; the floor is 9e15, so a fixed
    // day number can never be mistaken for a sentinel.
    let age_of_universe_in_days = -5_000_000_000_000i64;
    assert!(HC_ERR_FLOOR < age_of_universe_in_days);
    for sentinel in [
        HC_ERR_INVALID_DATE,
        HC_ERR_OUT_OF_RANGE,
        HC_ERR_BUFFER_TOO_SMALL,
        HC_ERR_NO_DATA,
        HC_ERR_NULL_POINTER,
        HC_ERR_UNKNOWN,
        HC_ERR_NOT_UTF8,
        HC_ERR_MALFORMED,
    ] {
        assert!(sentinel <= HC_ERR_FLOOR);
    }
}

/// Call a line-writing export the way a page does: measure with a null
/// buffer, refuse a buffer that is too small, then read the text.
#[cfg(any(
    feature = "civil",
    feature = "timestamps",
    feature = "time-codes",
    feature = "calendars",
    feature = "holiday",
    feature = "seasons",
    feature = "deep-time",
    feature = "tz",
    feature = "sky",
    feature = "orbital",
    feature = "jupiter",
    feature = "planetary",
    feature = "relativity",
    feature = "places",
    feature = "humanize",
    feature = "natural",
    feature = "datetime",
    feature = "patterns",
    feature = "zone-names",
    feature = "uncertainty",
    feature = "units",
    feature = "fiscal",
    feature = "name-days",
    feature = "attributes"
))]
fn read_lines(call: impl Fn(*mut u8, usize) -> i64) -> String {
    let needed = call(core::ptr::null_mut(), 0);
    assert!(needed > 0, "measured {needed}");
    let capacity = needed as usize;
    let mut small = [7u8; 1];
    assert_eq!(
        call(small.as_mut_ptr(), small.len()),
        HC_ERR_BUFFER_TOO_SMALL
    );
    assert_eq!(small, [7u8], "a refused buffer is left untouched");
    let pointer = hc_alloc(capacity);
    assert_eq!(call(pointer, capacity), needed);
    let text = unsafe { core::str::from_utf8(core::slice::from_raw_parts(pointer, capacity)) }
        .expect("UTF-8")
        .to_owned();
    unsafe { hc_free(pointer, capacity) };
    assert!(text.ends_with('\n'), "{text:?}");
    text
}

#[cfg(feature = "civil")]
mod civil_refusals;

#[cfg(feature = "civil")]
mod civil;

#[cfg(feature = "civil")]
mod python_time;

#[cfg(feature = "calendars")]
mod calendars;

#[cfg(feature = "holiday")]
mod holiday;

#[cfg(feature = "seasons")]
mod seasons;

#[cfg(feature = "tz")]
mod tz;

#[cfg(feature = "deep-time")]
mod deep_time;
#[cfg(feature = "tz")]
mod zones;

#[cfg(feature = "sky")]
mod sky;

#[cfg(feature = "orbital")]
mod orbital;

#[cfg(feature = "jupiter")]
mod jupiter;

#[cfg(feature = "timestamps")]
mod time_scales;

#[cfg(feature = "calendars")]
mod calendar_days;

#[cfg(feature = "holiday")]
mod observances;

#[cfg(feature = "sky")]
mod earth_and_sun;

#[cfg(feature = "planetary")]
mod planetary;

#[cfg(feature = "relativity")]
mod relativity;

#[cfg(feature = "places")]
mod places;

#[cfg(feature = "humanize")]
mod humanize;

#[cfg(feature = "natural")]
mod natural;

#[cfg(feature = "datetime")]
mod datetime;

#[cfg(feature = "patterns")]
mod patterns;

#[cfg(feature = "zone-names")]
mod zone_names;

#[cfg(feature = "uncertainty")]
mod uncertainty;

#[cfg(feature = "units")]
mod units;

#[cfg(feature = "fiscal")]
mod fiscal;

#[cfg(feature = "name-days")]
mod name_days;

#[cfg(feature = "attributes")]
mod attributes;

#[cfg(feature = "time-codes")]
mod time_codes;

#[cfg(feature = "timestamps")]
mod clock_readings;

#[cfg(feature = "calendars")]
mod day_periods;

#[cfg(feature = "calendars")]
mod locale_resolution;

#[cfg(feature = "holiday")]
mod fasts;

#[cfg(feature = "sky")]
mod hours;

#[cfg(feature = "calendars")]
mod reckonings;

#[cfg(feature = "calendars")]
mod almanac_notes;

#[cfg(feature = "calendars")]
mod tibetan_almanac;

#[cfg(all(feature = "calendars", feature = "sky"))]
mod hindu_limbs;

#[cfg(feature = "calendars")]
mod indian_festivals;

#[cfg(feature = "sky")]
mod sky_reckonings;

#[cfg(feature = "time-codes")]
mod irig;

/// 入梅 and 出梅 of 2026 at the Chinese meridian (`qq-meiyu-2026`).
#[cfg(feature = "seasons")]
#[test]
fn the_plum_rains_of_2026_cross_the_boundary() {
    let (bing, wei, china) = ("ru-mei-bing", "chu-mei-wei", "china");
    assert_eq!(
        unsafe { hc_plum_rains(bing.as_ptr(), bing.len(), 2026, china.as_ptr(), china.len()) },
        hc::hc_calendars_solar::gregorian::to_fixed(2026, 6, 11)
            .expect("a date")
            .0
    );
    assert_eq!(
        unsafe { hc_plum_rains(wei.as_ptr(), wei.len(), 2026, china.as_ptr(), china.len()) },
        hc::hc_calendars_solar::gregorian::to_fixed(2026, 7, 8)
            .expect("a date")
            .0
    );
    assert_eq!(
        unsafe { hc_plum_rains(bing.as_ptr(), bing.len(), 3001, china.as_ptr(), china.len()) },
        HC_ERR_OUT_OF_RANGE
    );
    assert_eq!(
        unsafe { hc_plum_rains("x".as_ptr(), 1, 2026, china.as_ptr(), china.len()) },
        HC_ERR_UNKNOWN
    );
}

#[cfg(any(
    feature = "civil",
    feature = "timestamps",
    feature = "calendars",
    feature = "holiday",
    feature = "zone-names"
))]
mod named_exports;
