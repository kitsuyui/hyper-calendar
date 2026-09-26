//! Every `strftime` directive Python's `datetime` documents, formatted and
//! parsed back.
//!
//! The expected outputs are written from the directive table of the Python
//! 3.13 documentation, "strftime() and strptime() Format Codes"
//! (<https://docs.python.org/3/library/datetime.html>, retrieved 2026-09-26),
//! for the sample that table itself uses in its `%c`, `%x` and `%X` rows:
//! Tuesday 16 August 1988, 21:30:00 — "Tue Aug 16 21:30:00 1988", "08/16/88",
//! "21:30:00". They were worked out from the definitions, not produced by
//! running Python.

use hc_format::ZoneInfo;
use hc_format::hc_calendar::{CivilDateTime, CivilTime, Rd};
use hc_format::hc_calendars_solar::gregorian;
use hc_format::hc_tz::UtcOffset;
use hc_format::patterns::{FormatContext, strftime};

fn sample(micros: u64) -> CivilDateTime {
    let day = match gregorian::to_fixed(1988, 8, 16) {
        Ok(day) => day,
        Err(error) => panic!("the sample day: {error}"),
    };
    match CivilTime::new(21, 30, 0, micros * 1_000_000_000_000) {
        Ok(time) => CivilDateTime::new(day, time),
        Err(error) => panic!("the sample time: {error}"),
    }
}

fn render(pattern: &str, context: &FormatContext<'_>) -> String {
    let mut out = String::new();
    if let Err(error) = strftime::format(&mut out, pattern, context) {
        panic!("{pattern}: {error}");
    }
    out
}

/// `(directive, expected, the documentation's description)`.
const NAIVE: &[(&str, &str, &str)] = &[
    ("%a", "Tue", "weekday as locale's abbreviated name"),
    ("%A", "Tuesday", "weekday as locale's full name"),
    (
        "%w",
        "2",
        "weekday as a decimal number, where 0 is Sunday and 6 is Saturday",
    ),
    (
        "%d",
        "16",
        "day of the month as a zero-padded decimal number",
    ),
    ("%b", "Aug", "month as locale's abbreviated name"),
    ("%B", "August", "month as locale's full name"),
    ("%m", "08", "month as a zero-padded decimal number"),
    (
        "%y",
        "88",
        "year without century as a zero-padded decimal number",
    ),
    ("%Y", "1988", "year with century as a decimal number"),
    (
        "%H",
        "21",
        "hour (24-hour clock) as a zero-padded decimal number",
    ),
    (
        "%I",
        "09",
        "hour (12-hour clock) as a zero-padded decimal number",
    ),
    ("%p", "PM", "locale's equivalent of either AM or PM"),
    ("%M", "30", "minute as a zero-padded decimal number"),
    ("%S", "00", "second as a zero-padded decimal number"),
    (
        "%f",
        "000000",
        "microsecond as a decimal number, zero-padded to 6 digits",
    ),
    (
        "%z",
        "",
        "UTC offset ... (empty string if the object is naive)",
    ),
    (
        "%Z",
        "",
        "time zone name (empty string if the object is naive)",
    ),
    (
        "%j",
        "229",
        "day of the year as a zero-padded decimal number",
    ),
    (
        "%U",
        "33",
        "week number of the year (Sunday as the first day of the week)",
    ),
    (
        "%W",
        "33",
        "week number of the year (Monday as the first day of the week)",
    ),
    (
        "%c",
        "Tue Aug 16 21:30:00 1988",
        "locale's appropriate date and time representation",
    ),
    ("%x", "08/16/88", "locale's appropriate date representation"),
    ("%X", "21:30:00", "locale's appropriate time representation"),
    ("%%", "%", "a literal '%' character"),
    ("%G", "1988", "ISO 8601 year with century"),
    (
        "%u",
        "2",
        "ISO 8601 weekday as a decimal number where 1 is Monday",
    ),
    (
        "%V",
        "33",
        "ISO 8601 week as a decimal number with Monday as the first day of the week",
    ),
];

#[test]
fn every_documented_directive_formats_the_sample_as_python_does() {
    let context = FormatContext::new(sample(0));
    for &(directive, expected, description) in NAIVE {
        assert_eq!(
            render(directive, &context),
            expected,
            "{directive}: {description}"
        );
    }
}

/// The `%f` row's own examples: "000000, 000001, …, 999999".
#[test]
fn the_microsecond_directive_is_six_digits_and_truncates() {
    for (micros, expected) in [(0, "000000"), (1, "000001"), (999_999, "999999")] {
        assert_eq!(render("%f", &FormatContext::new(sample(micros))), expected);
    }
    // A reading finer than a microsecond is truncated, not rounded, and an
    // explicit width asks for more digits.
    let fine = CivilDateTime::new(
        Rd(1),
        CivilTime::new(0, 0, 0, 123_456_789_000_000_000).unwrap(),
    );
    assert_eq!(render("%f", &FormatContext::new(fine)), "123456");
    assert_eq!(render("%9f", &FormatContext::new(fine)), "123456789");
}

/// The `%z` row's examples: "(empty), +0000, -0400, +1030, +063415", and the
/// `%:z` row's: "+00:00, -04:00, +10:30, +06:34:15". The `%Z` row's: "UTC".
#[test]
fn the_offset_directives_match_the_documented_examples() {
    let cases = [
        (0, "+0000", "+00:00"),
        (-4 * 3_600, "-0400", "-04:00"),
        (10 * 3_600 + 30 * 60, "+1030", "+10:30"),
        (6 * 3_600 + 34 * 60 + 15, "+063415", "+06:34:15"),
    ];
    for (seconds, basic, extended) in cases {
        let zone = ZoneInfo::Offset(UtcOffset::from_seconds(seconds).unwrap());
        let context = FormatContext::new(sample(0)).with_zone(zone);
        assert_eq!(render("%z", &context), basic);
        assert_eq!(render("%:z", &context), extended);
    }
    let utc = FormatContext::new(sample(0)).with_zone(ZoneInfo::Zulu);
    assert_eq!(render("%Z", &utc), "UTC");
    assert_eq!(render("%z", &utc), "+0000");
}

/// Every directive that names a field reads back what it wrote.
#[test]
fn every_documented_directive_parses_back() {
    let reading = sample(250_000);
    let context = FormatContext::new(reading);
    let pattern = "%a %A %w %d %b %B %m %y %Y %H %I %p %M %S %f %j %G %u %V";
    let text = render(pattern, &context);
    assert_eq!(
        text,
        "Tue Tuesday 2 16 Aug August 08 88 1988 21 09 PM 30 00 250000 229 1988 2 33"
    );
    let parsed = strftime::parse(pattern, &text).unwrap();
    assert_eq!(parsed.to_offset_date_time().unwrap().local, reading);

    for pattern in ["%c", "%x %X", "%Y-%m-%d %H:%M:%S.%f"] {
        let text = render(pattern, &context);
        let back = strftime::parse(pattern, &text)
            .unwrap()
            .to_offset_date_time()
            .unwrap();
        assert_eq!(back.local.day, reading.day, "{pattern}");
    }
}

/// Python: "When used with the `strptime()` method, `%U` and `%W` are only
/// used in calculations when the day of the week and the year are
/// specified." And `%f` "accepts from one to six digits and zero pads on the
/// right".
#[test]
fn week_numbers_and_fractions_parse_as_python_documents() {
    let day = |text: &str, pattern: &str| {
        strftime::parse(pattern, text)
            .unwrap()
            .to_offset_date_time()
            .unwrap()
            .local
            .day
    };
    assert_eq!(
        day("1988 33 Tue", "%Y %U %a"),
        gregorian::to_fixed(1988, 8, 16).unwrap()
    );
    assert_eq!(
        day("1988 33 2", "%Y %W %w"),
        gregorian::to_fixed(1988, 8, 16).unwrap()
    );
    // Week 0 is the days before the first Sunday or Monday, and can reach
    // into the previous year: 1988 began on a Friday.
    assert_eq!(
        day("1988 00 Fri", "%Y %U %a"),
        gregorian::to_fixed(1988, 1, 1).unwrap()
    );
    assert_eq!(
        day("1988 00 Mon", "%Y %W %a"),
        gregorian::to_fixed(1987, 12, 28).unwrap()
    );
    // Without a weekday the week number decides nothing.
    assert_eq!(
        day("1988-08-16 00", "%Y-%m-%d %U"),
        gregorian::to_fixed(1988, 8, 16).unwrap()
    );

    let fraction = |text: &str| {
        strftime::parse("%S.%f", text)
            .unwrap()
            .subsec_attos
            .unwrap()
            / 1_000_000_000_000
    };
    assert_eq!(fraction("05.5"), 500_000);
    assert_eq!(fraction("05.000001"), 1);
    assert_eq!(fraction("05.123456"), 123_456);
    assert!(strftime::parse("%S.%f", "05.1234567").is_err());
}
