use super::*;

#[test]
fn querying_the_version_works_in_both_passes() {
    let mut written = 0usize;
    assert_eq!(
        unsafe { hc_version(core::ptr::null_mut(), 0, &mut written) },
        HC_ERROR_BUFFER_TOO_SMALL
    );
    assert_eq!(written, hc::VERSION.len() + 1);

    let mut buffer = vec![0 as c_char; written];
    assert_eq!(
        unsafe { hc_version(buffer.as_mut_ptr(), buffer.len(), core::ptr::null_mut()) },
        HC_OK
    );
}

/// Call a line-writing entry point the way a C caller does: measure,
/// allocate, read, and check the terminator.
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
fn read_lines(call: impl Fn(*mut c_char, usize, *mut usize) -> HcStatus) -> String {
    let mut written = 0usize;
    assert_eq!(
        call(core::ptr::null_mut(), 0, &mut written),
        HC_ERROR_BUFFER_TOO_SMALL
    );
    assert!(written > 1);
    let mut small = [7 as c_char; 1];
    assert_eq!(
        call(small.as_mut_ptr(), small.len(), core::ptr::null_mut()),
        HC_ERROR_BUFFER_TOO_SMALL
    );
    assert_eq!(small, [7 as c_char], "a refused buffer is left untouched");
    let mut buffer = vec![0 as c_char; written];
    let mut again = 0usize;
    assert_eq!(call(buffer.as_mut_ptr(), buffer.len(), &mut again), HC_OK);
    assert_eq!(again, written);
    let text = unsafe { core::ffi::CStr::from_ptr(buffer.as_ptr()) }
        .to_str()
        .expect("UTF-8")
        .to_owned();
    assert_eq!(text.len() + 1, written);
    assert!(text.ends_with('\n'), "{text:?}");
    text
}

/// The status of a line-writing entry point's measuring call: a null
/// buffer, which is `HC_ERROR_BUFFER_TOO_SMALL` when the entry point
/// answers, and the refusal when it does not.
#[cfg(any(
    feature = "timestamps",
    feature = "time-codes",
    feature = "calendars",
    feature = "holiday",
    feature = "sky",
    feature = "planetary",
    feature = "relativity",
    feature = "places",
    feature = "humanize",
    feature = "natural",
    feature = "datetime",
    feature = "patterns",
    feature = "zone-names"
))]
fn measured(call: impl Fn(*mut c_char, usize, *mut usize) -> HcStatus) -> HcStatus {
    let mut written = 0usize;
    call(core::ptr::null_mut(), 0, &mut written)
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

/// 入梅 of 2026 at the Chinese meridian, 11 June (`qq-meiyu-2026`).
#[cfg(feature = "seasons")]
#[test]
fn the_plum_rains_of_2026_cross_the_c_boundary() {
    let mut day = 0i64;
    assert_eq!(
        unsafe { hc_plum_rains(c"ru-mei-bing".as_ptr(), 2026, c"china".as_ptr(), &mut day) },
        HC_OK
    );
    assert_eq!(day, 739_778);
    assert_eq!(
        unsafe { hc_plum_rains(c"ru-mei".as_ptr(), 2026, c"china".as_ptr(), &mut day) },
        HC_ERROR_UNKNOWN
    );
    assert_eq!(
        unsafe { hc_plum_rains(c"ru-mei-bing".as_ptr(), 2026, core::ptr::null(), &mut day) },
        HC_ERROR_NULL_POINTER
    );
    assert_eq!(
        unsafe {
            hc_plum_rains(
                c"ru-mei-bing".as_ptr(),
                2026,
                c"china".as_ptr(),
                core::ptr::null_mut(),
            )
        },
        HC_ERROR_NULL_POINTER
    );
}

#[cfg(any(feature = "timestamps", feature = "calendars", feature = "zone-names"))]
mod named_exports;
