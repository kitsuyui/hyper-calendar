//! The tab-separated lines the WebAssembly module and the C library write
//! about human-readable time, written once.
//!
//! Three questions of [`hc_humanize`], each answered in a locale and a
//! style:
//!
//! * [`relative_time_line`]: when an instant was, seen from another —
//!   *3 hours ago*, *in 2 days* — by [`hc_humanize::RelativeTimeFormatter`]
//!   with the conversational thresholds and truncation of its
//!   `write_elapsed`;
//! * [`relative_day_line`] and [`relative_day_at_line`]: which calendar day
//!   a day was, seen from another — *yesterday*, *yesterday at 15:05*,
//!   *last month* — by [`hc_humanize::CalendarRelativeFormatter`], which
//!   counts days by subtracting fixed day numbers, never by dividing a
//!   span;
//! * [`duration_line`]: how long a span is — *2 hours 30 minutes* — by
//!   [`hc_humanize::DurationFormatter`] in days, hours, minutes and
//!   seconds.
//!
//! and, in [`hc_humanize::natural`]'s convention — `humanize`, the Python
//! package's, which has no locale argument here because only its English is
//! carried — the number and list functions it has beside its time ones:
//!
//! * [`apnumber_line`], [`fractional_line`], [`scientific_line`],
//!   [`metric_line`], [`naturalsize_line`], [`naturallist_line`] and
//!   [`intword_line`]: two cells each, the text and the language of the
//!   vocabulary that wrote it, `en`.
//!
//! The crate does not know what "now" is, so each line takes both ends
//! from the caller; nor does it know a time zone, so a relative day takes
//! the two fixed days a caller has already read off its own clock. The
//! last cell of each line is the tag of the `hc-humanize` data entry the
//! locale resolved to.

use alloc::string::String;

use hc_calendar::{CivilTime, Rd};
use hc_core::Duration;
use hc_humanize::approximate::{ApproximateFormatter, ApproximatePolicy, approximate};
use hc_humanize::calendar_relative::{day_amount, write_clock_time};
use hc_humanize::lookup::locale_data;
use hc_humanize::natural::{
    DeltaOptions, Gender, Grouping, Natural, NaturalPhrases, NaturalWords, PreciseUnit, SizeStyle,
};
use hc_humanize::unit_choice::choose;
use hc_humanize::{
    CalendarRelativeFormatter, DurationFormatter, DurationStyle, HumanizeError, Numeric,
    RelativeStyle, RelativeTimeFormatter, RoundingPolicy, Thresholds, TimeUnit, UnitAmount,
};
use hc_i18n::Locale;

use crate::boundary::{Answer, Line, Refusal};

/// How many columns [`relative_time_line`] and [`relative_day_line`]
/// write.
pub const RELATIVE_COLUMNS: usize = 4;

/// How many columns [`relative_day_at_line`] writes.
pub const RELATIVE_DAY_AT_COLUMNS: usize = 5;

/// How many columns [`duration_line`] writes.
pub const DURATION_COLUMNS: usize = 3;

/// How many columns the lines of the `humanize` number functions write.
pub const NATURAL_COLUMNS: usize = 2;

/// The refusal a humanising error is: an amount that does not fit, or a
/// numeral the locale's digits cannot write, is out of range; a unit no
/// entry of the locale's chain phrases, which the data shipped cannot
/// have, is no data.
const fn refusal(error: HumanizeError) -> Refusal {
    match error {
        HumanizeError::NoPattern => Refusal::NoData,
        _ => Refusal::OutOfRange,
    }
}

/// The locale a tag names, the root locale for a tag that does not parse,
/// whose phrases are CLDR's `root.xml`'s, `-1 d`, not English's.
fn locale(tag: &str) -> Locale {
    Locale::parse(tag).unwrap_or(Locale::ROOT)
}

/// The relative style an identifier names, `long`, `short` or `narrow`,
/// by [`RelativeStyle::by_id`].
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other text, the empty one included.
pub fn relative_style(id: &str) -> Answer<RelativeStyle> {
    RelativeStyle::by_id(id).ok_or(Refusal::Unknown)
}

/// The numeric choice a flag makes: the language's own word for the
/// offset where it has one (`Intl.RelativeTimeFormat`'s `numeric: "auto"`)
/// for `true`, the numeric pattern always for `false`.
const fn numeric(auto: bool) -> Numeric {
    if auto { Numeric::Auto } else { Numeric::Always }
}

/// The phrase, the unit and the count of an amount.
fn relative_cells(line: &mut Line<'_>, phrase: &str, amount: UnitAmount) {
    line.cell(phrase)
        .cell(amount.unit().as_str())
        .value(amount.count());
}

/// The line of `hc_relative_time`: how an instant `then_unix` reads from
/// the instant `now_unix`, both whole POSIX seconds — the phrase (*3 hours
/// ago*, *in 2 days*; with `automatic`, *yesterday* where the language has a
/// word for the offset); the unit it is counted in, CLDR's field name
/// (`second`, `minute`, `hour`, `day`, `week`, `month` or `year`); the
/// signed count, negative in the past; and the tag of the data the locale
/// resolved to.
///
/// The unit is the one the conversational thresholds choose and the count
/// is truncated, as `hc-humanize`'s `write_elapsed` has it: 90 minutes ago
/// is *1 hour ago*, not *2 hours ago*. A month and a year are the
/// Gregorian means, so a count of them is of a span, not of calendar
/// months.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a style [`relative_style`] does not name, and
/// [`Refusal::OutOfRange`] when the two instants are further apart than an
/// `i64` of seconds holds.
pub fn relative_time_line(
    then_unix: i64,
    now_unix: i64,
    style: &str,
    automatic: bool,
    tag: &str,
) -> Answer<String> {
    let style = relative_style(style)?;
    let seconds = then_unix.checked_sub(now_unix).ok_or(Refusal::OutOfRange)?;
    let locale = locale(tag);
    let amount = choose(
        Duration::from_secs(i128::from(seconds)),
        &Thresholds::DEFAULT,
        RoundingPolicy::Truncate,
    )
    .map_err(refusal)?;
    let mut phrase = String::new();
    RelativeTimeFormatter::new(locale)
        .with_style(style)
        .with_numeric(numeric(automatic))
        .write_amount(amount, &mut phrase)
        .map_err(refusal)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    relative_cells(&mut line, &phrase, amount);
    line.cell(locale_data(&locale).tag);
    line.end();
    Ok(out)
}

/// The day offset from `now_fixed` to `then_fixed` as the amount a
/// calendar-relative phrase counts: days up to a week, then weeks, months
/// and years by the conversational thresholds, truncated.
fn day_offset_amount(then_fixed: i64, now_fixed: i64) -> Answer<UnitAmount> {
    then_fixed
        .checked_sub(now_fixed)
        .ok_or(Refusal::OutOfRange)?;
    day_amount(
        Rd(now_fixed),
        Rd(then_fixed),
        &Thresholds::DEFAULT,
        RoundingPolicy::Truncate,
    )
    .map_err(refusal)
}

/// The line of `hc_relative_day`: how the fixed day `then_fixed` reads
/// from the fixed day `now_fixed` — the phrase (*3 days ago*, and with
/// `automatic` *yesterday*, *today*, *last month*); the unit, `day` up to a
/// week and then `week`, `month` or `year`; the signed count, negative in
/// the past; and the tag of the data the locale resolved to.
///
/// The offset is the difference of the two day numbers, so 23:30 on one
/// day and 00:30 on the next are a day apart, as a reader counts them;
/// which day an instant falls on is the caller's clock's to say.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a style [`relative_style`] does not name, and
/// [`Refusal::OutOfRange`] when the two days are further apart than an
/// `i64` holds, or than a count of seconds does.
pub fn relative_day_line(
    then_fixed: i64,
    now_fixed: i64,
    style: &str,
    automatic: bool,
    tag: &str,
) -> Answer<String> {
    let style = relative_style(style)?;
    let amount = day_offset_amount(then_fixed, now_fixed)?;
    let locale = locale(tag);
    let mut phrase = String::new();
    CalendarRelativeFormatter::new(locale)
        .with_style(style)
        .with_numeric(numeric(automatic))
        .write_day(Rd(now_fixed), Rd(then_fixed), &mut phrase)
        .map_err(refusal)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    relative_cells(&mut line, &phrase, amount);
    line.cell(locale_data(&locale).tag);
    line.end();
    Ok(out)
}

/// The line of `hc_relative_day_at`: [`relative_day_line`]'s phrase with a
/// time of day, *yesterday at 15:05* — the day phrase and the time joined
/// by the locale's pattern for a relative day with a time, which is CLDR's
/// (`es` *ayer, 15:05*, `ja` *昨日の 15:05*) — then the unit and the signed
/// count of the day phrase, the time as it was written, and the tag of the
/// data the locale resolved to.
///
/// The time is `seconds_of_day` after midnight written on a 24-hour clock
/// as `H:MM`, in the locale's digits, the seconds dropped: `hc-humanize`'s
/// plain clock, which has no hour cycle and no day period.
///
/// # Errors
///
/// As [`relative_day_line`], and [`Refusal::OutOfRange`] for
/// `seconds_of_day` from 86 400.
pub fn relative_day_at_line(
    then_fixed: i64,
    now_fixed: i64,
    seconds_of_day: u32,
    style: &str,
    automatic: bool,
    tag: &str,
) -> Answer<String> {
    let style = relative_style(style)?;
    if seconds_of_day >= 86_400 {
        return Err(Refusal::OutOfRange);
    }
    let time = CivilTime::hms(
        (seconds_of_day / 3_600) as u8,
        (seconds_of_day / 60 % 60) as u8,
        (seconds_of_day % 60) as u8,
    )
    .map_err(Refusal::from)?;
    let amount = day_offset_amount(then_fixed, now_fixed)?;
    let locale = locale(tag);
    let mut clock = String::new();
    write_clock_time(&locale, time, &mut clock).map_err(refusal)?;
    let mut phrase = String::new();
    CalendarRelativeFormatter::new(locale)
        .with_style(style)
        .with_numeric(numeric(automatic))
        .write_day_at(Rd(now_fixed), Rd(then_fixed), &clock, &mut phrase)
        .map_err(refusal)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    relative_cells(&mut line, &phrase, amount);
    line.cell(&clock).cell(locale_data(&locale).tag);
    line.end();
    Ok(out)
}

/// The duration style an identifier names, `long`, `short`, `narrow` or
/// `compact`, by [`DurationStyle::by_id`].
///
/// # Errors
///
/// [`Refusal::Unknown`] for any other text, the empty one included.
pub fn duration_style(id: &str) -> Answer<DurationStyle> {
    DurationStyle::by_id(id).ok_or(Refusal::Unknown)
}

/// The line of `hc_duration`: a span of `seconds` phrased in days, hours,
/// minutes and seconds — *2 hours 30 minutes* in `long`, joined by the
/// locale's list pattern; the abbreviated phrases in `short`; the narrow
/// ones in `narrow`; *2h30m* in `compact` — then `1` if the span is
/// negative and `0` if not, the phrase being of its length, and the tag
/// of the data the locale resolved to.
///
/// A unit the span does not reach is left out, and at most
/// `max_components` units are written, the largest first, the rest of the
/// span dropped, not rounded: `0` writes every unit. A span of nothing is
/// *0 seconds*.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a style [`duration_style`] does not name.
pub fn duration_line(seconds: i64, style: &str, max_components: u32, tag: &str) -> Answer<String> {
    let style = duration_style(style)?;
    let locale = locale(tag);
    let most = match usize::try_from(max_components) {
        Ok(0) | Err(_) => hc_humanize::duration::MAX_COMPONENTS,
        Ok(most) => most,
    };
    let mut phrase = String::new();
    DurationFormatter::new(locale)
        .with_style(style)
        .with_units(&TimeUnit::CLOCK)
        .with_max_components(most)
        .write(Duration::from_secs(i128::from(seconds)), &mut phrase)
        .map_err(refusal)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell(&phrase)
        .flag(seconds < 0)
        .cell(locale_data(&locale).tag);
    line.end();
    Ok(out)
}

/// A line of the `humanize` functions: the text `write` makes, and the
/// language of the vocabulary that made it.
fn natural_line(
    natural: &Natural,
    write: impl FnOnce(&Natural, &mut String) -> Result<(), HumanizeError>,
) -> Answer<String> {
    let mut text = String::new();
    write(natural, &mut text).map_err(|error| match error {
        HumanizeError::Unsupported("an integer written in digits" | "the strftime pattern") => {
            Refusal::Malformed
        }
        _ => Refusal::OutOfRange,
    })?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell(&text).cell(natural.phrases().language);
    line.end();
    Ok(out)
}

/// The vocabulary that serves a locale for the words of one function:
/// [`NaturalPhrases::for_locale`], so the first catalogue along the
/// locale's fallback chain that translates those words, and `humanize`'s own
/// English where none does. A tag that does not parse is the root locale,
/// which has no catalogue and so is English too.
fn vocabulary(tag: &str, words: NaturalWords) -> Natural {
    Natural::with_phrases(NaturalPhrases::for_locale(&locale(tag), words))
}

/// The decimals or precision of a number line: what fits a `u8`.
fn digits_of(value: u32) -> Answer<u8> {
    u8::try_from(value).map_err(|_| Refusal::OutOfRange)
}

/// The line of `hc_apnumber`: `humanize`'s `apnumber`, the Associated Press
/// style, which spells out `zero` to `nine` and leaves a larger or negative
/// number as its digits, then the language of the catalogue that wrote it,
/// `en`, `de-DE`, `ru-RU`, chosen along the locale's fallback chain among the
/// catalogues that translate the numerals.
///
/// # Errors
///
/// None in practice; the signature is the shared one.
pub fn apnumber_line(value: i64, tag: &str) -> Answer<String> {
    natural_line(&vocabulary(tag, NaturalWords::Apnumber), |natural, out| {
        natural.write_apnumber(out, i128::from(value))
    })
}

/// The line of `hc_fractional`: `humanize`'s `fractional`, `0.3` as `3/10`
/// and `1.3` as `1 3/10`, by the nearest fraction with a denominator of at
/// most 1000, then the language, `en`: it writes no word, so it has no
/// locale. A value that is not finite is `NaN`, `+Inf` or `-Inf`.
///
/// # Errors
///
/// None in practice; the signature is the shared one.
pub fn fractional_line(value: f64) -> Answer<String> {
    natural_line(&Natural::english(), |natural, out| {
        natural.write_fractional(out, value)
    })
}

/// The line of `hc_scientific`: `humanize`'s `scientific`, `3.00 x 10⁻¹`,
/// with `precision` digits after the point, then the language, `en`.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a precision above 255.
pub fn scientific_line(value: f64, precision: u32) -> Answer<String> {
    let precision = digits_of(precision)?;
    natural_line(&Natural::english(), |natural, out| {
        natural.write_scientific(out, value, precision)
    })
}

/// The line of `hc_metric`: `humanize`'s `metric`, `1.50 kV`, `220 μF`,
/// with `precision` significant digits and the unit after the prefix, then
/// the language of the first catalogue for the locale. The prefixes are
/// symbols and no catalogue translates them; what a catalogue changes is
/// the decimal mark of the scientific form a magnitude beyond the prefixes
/// falls back to.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a precision above 255, and for 0 on a
/// magnitude too large or small for a prefix, which `humanize` refuses with
/// a `ValueError`.
pub fn metric_line(value: f64, unit: &str, precision: u32, tag: &str) -> Answer<String> {
    let precision = digits_of(precision)?;
    natural_line(&vocabulary(tag, NaturalWords::Any), |natural, out| {
        natural.write_metric(out, value, unit, precision)
    })
}

/// The line of `hc_naturalsize`: `humanize`'s `naturalsize`, `3.0 MB`,
/// `2.9 KiB`, `300B`, with `decimals` after the point, then the language of
/// the catalogue that wrote it, chosen among those that translate *Byte* and
/// the suffixes. `style` is `decimal` (powers of 1000), `binary` (1024,
/// `KiB`) or `gnu` (1024, `K`), in any case.
///
/// # Errors
///
/// [`Refusal::Unknown`] for another style, and [`Refusal::OutOfRange`] for
/// `NaN`, which Python's `int` refuses, and for decimals above 255.
pub fn naturalsize_line(value: f64, style: &str, decimals: u32, tag: &str) -> Answer<String> {
    let style = if style.eq_ignore_ascii_case("decimal") {
        SizeStyle::Decimal
    } else if style.eq_ignore_ascii_case("binary") {
        SizeStyle::Binary
    } else if style.eq_ignore_ascii_case("gnu") {
        SizeStyle::Gnu
    } else {
        return Err(Refusal::Unknown);
    };
    let decimals = digits_of(decimals)?;
    natural_line(
        &vocabulary(tag, NaturalWords::Naturalsize),
        |natural, out| natural.write_naturalsize(out, value, style, decimals),
    )
}

/// The line of `hc_naturallist`: `humanize`'s `natural_list`, `one, two
/// and three`, of items given one to a line, then the language, `en`. No
/// item is the empty phrase.
///
/// `locale` is taken and resolved like the other number lines' but changes
/// nothing: `natural_list`'s `, ` and ` and ` are literals in `humanize`'s
/// `lists.py`, which no catalogue translates (`humanize` 4.16.0's source,
/// read for `docs/python-parity.md`), so every locale gets its English and
/// the line says `en`, not a language it was not written in.
///
/// # Errors
///
/// None in practice; the signature is the shared one.
pub fn naturallist_line(items: &str, _tag: &str) -> Answer<String> {
    let items: alloc::vec::Vec<&str> = if items.is_empty() {
        alloc::vec::Vec::new()
    } else {
        items.split('\n').collect()
    };
    natural_line(&Natural::english(), |natural, out| {
        natural.write_naturallist(out, &items)
    })
}

/// The line of `hc_intword`: `humanize`'s `intword`, `12.4 thousand`,
/// `1.2 billion`, `1.0 googol`, of an integer written in digits, any length
/// up to the largest double, with `decimals` after the point, then the
/// language of the catalogue that wrote it, chosen among those that
/// translate the words of the powers.
///
/// # Errors
///
/// [`Refusal::Malformed`] for text that is not an integer,
/// [`Refusal::OutOfRange`] for an integer beyond the largest double and for
/// decimals above 255.
pub fn intword_line(digits: &str, decimals: u32, tag: &str) -> Answer<String> {
    let decimals = digits_of(decimals)?;
    natural_line(&vocabulary(tag, NaturalWords::Intword), |natural, out| {
        natural.write_intword_digits(out, digits, decimals)
    })
}

/// How many columns [`unit_choice_line`], [`relative_time_with_line`] and
/// [`approximate_duration_line`] write.
pub const CHOICE_COLUMNS: usize = 5;

/// The span of `seconds` and `microseconds`, which may differ in sign: the
/// seconds and the microseconds added, as a Python `timedelta`'s fields are.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for microseconds of a second or more.
fn span_of(seconds: i64, microseconds: i32) -> Answer<Duration> {
    if microseconds.unsigned_abs() >= 1_000_000 {
        return Err(Refusal::OutOfRange);
    }
    Ok(Duration::from_micros(
        i128::from(seconds) * 1_000_000 + i128::from(microseconds),
    ))
}

/// The units a `naturaldelta` or `naturaltime` can stop at: `seconds`,
/// `milliseconds` or `microseconds`.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a name no unit has, [`Refusal::OutOfRange`] for
/// a unit above seconds, which Python refuses with a `ValueError`.
fn delta_options(months: bool, minimum_unit: &str) -> Answer<DeltaOptions> {
    let minimum_unit = PreciseUnit::by_id(minimum_unit).ok_or(Refusal::Unknown)?;
    if minimum_unit > PreciseUnit::Seconds {
        return Err(Refusal::OutOfRange);
    }
    Ok(DeltaOptions {
        months,
        minimum_unit,
    })
}

/// The line of `hc_naturaldelta`: `humanize`'s `naturaldelta` of the span
/// `seconds` plus `microseconds`, *3 hours*, *a moment*, *1 year, 3
/// months*, without tense and ignoring the sign, then the language of the
/// catalogue that wrote it, chosen along the locale's fallback chain among
/// the catalogues that translate every unit (and, below a second, the fine
/// ones). `months` is Python's `months`
/// (months of 30.5 days between days and years) and `minimum_unit` `seconds`,
/// `milliseconds` or `microseconds`.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for microseconds of a second or more, a unit above
/// seconds, which Python refuses with a `ValueError`, and for a
/// span of more than about 10²⁶ years; [`Refusal::Unknown`] for a unit name no
/// unit has.
pub fn naturaldelta_line(
    seconds: i64,
    microseconds: i32,
    months: bool,
    minimum_unit: &str,
    tag: &str,
) -> Answer<String> {
    let span = span_of(seconds, microseconds)?;
    let options = delta_options(months, minimum_unit)?;
    let words = if options.minimum_unit < PreciseUnit::Seconds {
        NaturalWords::DeltaFine
    } else {
        NaturalWords::Delta
    };
    natural_line(&vocabulary(tag, words), |natural, out| {
        natural.write_naturaldelta(out, span, options)
    })
}

/// The line of `hc_naturaltime`: `humanize`'s `naturaltime` of a span, *3
/// hours ago*, *now*: the span is how long ago, so a positive one is in the
/// past and a negative one *from now*, as in Python. The rest is as for
/// [`naturaldelta_line`].
///
/// # Errors
///
/// As [`naturaldelta_line`].
pub fn naturaltime_line(
    seconds: i64,
    microseconds: i32,
    months: bool,
    minimum_unit: &str,
    tag: &str,
) -> Answer<String> {
    let span = span_of(seconds, microseconds)?;
    let options = delta_options(months, minimum_unit)?;
    let words = if options.minimum_unit < PreciseUnit::Seconds {
        NaturalWords::TimeFine
    } else {
        NaturalWords::Time
    };
    natural_line(&vocabulary(tag, words), |natural, out| {
        natural.write_naturaltime_delta(out, span, options)
    })
}

/// The line of `hc_precisedelta`: `humanize`'s `precisedelta`, *1 year, 2
/// months and 3 days*, of the span `seconds` plus `microseconds`, with
/// `humanize` 4.16.0's arithmetic step for step. `minimum_unit` is the
/// smallest unit written, `microseconds` to `years`; `suppress` is the
/// units folded into the next smaller, comma-separated; `decimals` is the
/// places of the fraction of the smallest unit.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a unit name no unit has, [`Refusal::OutOfRange`]
/// for decimals above 255, a minimum unit suppressed with no larger unit
/// left, and an unrepresentable span.
pub fn precisedelta_line(
    seconds: i64,
    microseconds: i32,
    minimum_unit: &str,
    suppress: &str,
    decimals: u32,
    tag: &str,
) -> Answer<String> {
    let span = span_of(seconds, microseconds)?;
    let minimum_unit = PreciseUnit::by_id(minimum_unit).ok_or(Refusal::Unknown)?;
    let mut suppressed = alloc::vec::Vec::new();
    for name in suppress.split(',').filter(|name| !name.trim().is_empty()) {
        suppressed.push(PreciseUnit::by_id(name).ok_or(Refusal::Unknown)?);
    }
    let decimals = digits_of(decimals)?;
    let words = if minimum_unit < PreciseUnit::Seconds {
        NaturalWords::PreciseFine
    } else {
        NaturalWords::Precise
    };
    natural_line(&vocabulary(tag, words), |natural, out| {
        natural.write_precisedelta(out, span, minimum_unit, &suppressed, decimals)
    })
}

/// A fixed day as `humanize`'s `naturalday` takes it.
fn day_of(fixed: i64) -> Answer<Rd> {
    crate::civil::Date::from_ordinal(fixed).map_err(Refusal::from)?;
    Ok(Rd(fixed))
}

/// The line of `hc_naturalday`: `humanize`'s `naturalday`, *today*,
/// *tomorrow*, *yesterday*, or the day written by a `strftime` pattern in
/// the C locale, `%b %d` when `pattern` is empty, then the language of the
/// catalogue that wrote the words. The words are the catalogue's; the month
/// name is always the C locale's, as Python's `strftime` writes it.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a day outside the Gregorian range and
/// [`Refusal::Malformed`] for a pattern `hc-format` does not write.
pub fn naturalday_line(day: i64, today: i64, pattern: &str, tag: &str) -> Answer<String> {
    let (day, today) = (day_of(day)?, day_of(today)?);
    let pattern = if pattern.is_empty() { "%b %d" } else { pattern };
    natural_line(&vocabulary(tag, NaturalWords::Day), |natural, out| {
        natural.write_naturalday(out, day, today, pattern)
    })
}

/// The line of `hc_naturaldate`: `humanize`'s `naturaldate`, as
/// [`naturalday_line`] with `%b %d`, and `%b %d %Y` once the day is five
/// twelfths of a year or more from today.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for a day outside the Gregorian range.
pub fn naturaldate_line(day: i64, today: i64, tag: &str) -> Answer<String> {
    let (day, today) = (day_of(day)?, day_of(today)?);
    natural_line(&vocabulary(tag, NaturalWords::Day), |natural, out| {
        natural.write_naturaldate(out, day, today)
    })
}

/// The line of `hc_ordinal`: `humanize`'s `ordinal`, `1st`, `2nd`, `103rd`,
/// `111th`, in the suffixes of the catalogue that serves the locale, then
/// its language. `gender` is `male` or `female`, in any case.
///
/// # Errors
///
/// [`Refusal::Unknown`] for another gender.
pub fn ordinal_line(value: i64, gender: &str, tag: &str) -> Answer<String> {
    let gender = Gender::by_id(gender).ok_or(Refusal::Unknown)?;
    natural_line(&vocabulary(tag, NaturalWords::Ordinal), |natural, out| {
        natural.write_ordinal_of(out, i128::from(value), gender)
    })
}

/// The line of `hc_intcomma`: `humanize`'s `intcomma` of an integer written
/// in digits, `1,234,567`, with the separators of the catalogue that serves
/// the locale (`1.234.567` in German), then its language: the separators are
/// the catalogue's own, so the first catalogue for the locale is the one.
///
/// # Errors
///
/// [`Refusal::Malformed`] for text that is not an integer, and
/// [`Refusal::OutOfRange`] for one beyond 39 digits, which an `i128` holds.
pub fn intcomma_line(digits: &str, tag: &str) -> Answer<String> {
    let value: i128 =
        digits
            .trim()
            .parse()
            .map_err(|error: core::num::ParseIntError| match error.kind() {
                core::num::IntErrorKind::PosOverflow | core::num::IntErrorKind::NegOverflow => {
                    Refusal::OutOfRange
                }
                _ => Refusal::Malformed,
            })?;
    let natural = vocabulary(tag, NaturalWords::Any);
    let grouping: Grouping = natural.phrases().grouping;
    natural_line(&natural, |natural, out| {
        natural.write_intcomma(out, value, grouping)
    })
}

/// The line of `hc_intcomma_float`: `humanize`'s `intcomma` of a float,
/// `1,234,567.25`, to `ndigits` places, or, for a negative `ndigits`, as
/// Python's `repr` writes it, then the catalogue's language. A value that is
/// not finite is `NaN`, `+Inf` or `-Inf`.
///
/// # Errors
///
/// [`Refusal::OutOfRange`] for `ndigits` above 255.
pub fn intcomma_float_line(value: f64, ndigits: i32, tag: &str) -> Answer<String> {
    let ndigits = if ndigits < 0 {
        None
    } else {
        Some(digits_of(ndigits.unsigned_abs())?)
    };
    let natural = vocabulary(tag, NaturalWords::Any);
    let grouping = natural.phrases().grouping;
    natural_line(&natural, |natural, out| {
        natural.write_intcomma_f64(out, value, ndigits, grouping)
    })
}

/// The thresholds a caller names: `default`, `exact` or `with-quarters`,
/// with the identifier the line carries.
fn thresholds_named(id: &str) -> Answer<(&'static str, Thresholds)> {
    Thresholds::by_id(id).ok_or(Refusal::Unknown)
}

/// The rounding a caller names, `ceil`, `floor`, `nearest`, `truncate` or
/// `nearest-half`.
fn rounding_named(id: &str) -> Answer<RoundingPolicy> {
    RoundingPolicy::by_id(id).ok_or(Refusal::Unknown)
}

/// The signed span of a count of seconds.
fn seconds_span(seconds: i64) -> Duration {
    Duration::from_secs(i128::from(seconds))
}

/// The line of `hc_unit_choice`: the unit a span of `seconds` is said in and
/// its count, by `hc-humanize`'s [`choose`] — the unit CLDR's field name
/// (`second` to `year`, and `quarter` under `with-quarters`); the signed
/// count, its whole part; `1` when a half is added to it in the direction of
/// the sign (`nearest-half` only), else `0`; and the identifiers of the
/// thresholds and the rounding that were applied. `thresholds` is `default`,
/// the conversational table, `exact`, which promotes a unit only when a whole
/// one fits, or `with-quarters`; `rounding` is `ceil`, `floor`, `nearest`,
/// `truncate` or `nearest-half`.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a table or a rounding not named, and
/// [`Refusal::OutOfRange`] for a count that does not fit an `i64`.
pub fn unit_choice_line(seconds: i64, thresholds: &str, rounding: &str) -> Answer<String> {
    let (table_id, table) = thresholds_named(thresholds)?;
    let policy = rounding_named(rounding)?;
    let amount = choose(seconds_span(seconds), &table, policy).map_err(refusal)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell(amount.unit().as_str())
        .value(amount.count())
        .flag(amount.has_half())
        .cell(table_id)
        .cell(policy.id());
    line.end();
    Ok(out)
}

/// The line of `hc_relative_time_with`: [`relative_time_line`] with the
/// thresholds and the rounding a caller chooses in place of the
/// conversational table and truncation — the phrase; the unit; the signed
/// count; `1` for a half added to it (*an hour and a half ago*, under
/// `nearest-half`); and the tag of the data the locale resolved to.
///
/// # Errors
///
/// As [`relative_time_line`], and [`Refusal::Unknown`] for a table or a
/// rounding not named.
pub fn relative_time_with_line(
    then_unix: i64,
    now_unix: i64,
    style: &str,
    automatic: bool,
    tag: &str,
    thresholds: &str,
    rounding: &str,
) -> Answer<String> {
    let style = relative_style(style)?;
    let (_, table) = thresholds_named(thresholds)?;
    let policy = rounding_named(rounding)?;
    let seconds = then_unix.checked_sub(now_unix).ok_or(Refusal::OutOfRange)?;
    let locale = locale(tag);
    let amount = choose(seconds_span(seconds), &table, policy).map_err(refusal)?;
    let mut phrase = String::new();
    RelativeTimeFormatter::new(locale)
        .with_style(style)
        .with_numeric(numeric(automatic))
        .write_amount(amount, &mut phrase)
        .map_err(refusal)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    relative_cells(&mut line, &phrase, amount);
    line.flag(amount.has_half()).cell(locale_data(&locale).tag);
    line.end();
    Ok(out)
}

/// The line of `hc_approximate_duration`: a span hedged as a round number,
/// *about 3 hours*, *just over a week*, *nearly a year*, by
/// `hc-humanize`'s `approximate` — the phrase; the hedge, `exactly`,
/// `about`, `just-over`, `over` or `nearly`; the unit; the count, which
/// *nearly* carries up to the next; and the tag of the data the locale
/// resolved to. The sign is dropped: a hedge describes a length. `thresholds`
/// is as for [`unit_choice_line`] and `policy` is `default`, which says
/// *about* within 2 to 8 % of a unit, or `bounded`, which never does.
///
/// # Errors
///
/// [`Refusal::Unknown`] for a style, a table or a policy not named, and
/// [`Refusal::OutOfRange`] for a count that does not fit an `i64`.
pub fn approximate_duration_line(
    seconds: i64,
    style: &str,
    tag: &str,
    thresholds: &str,
    policy: &str,
) -> Answer<String> {
    let style = relative_style(style)?;
    let (_, table) = thresholds_named(thresholds)?;
    let (_, policy) = ApproximatePolicy::by_id(policy).ok_or(Refusal::Unknown)?;
    let locale = locale(tag);
    let span = approximate(seconds_span(seconds), &table, policy).map_err(refusal)?;
    let mut phrase = String::new();
    ApproximateFormatter::new(locale)
        .with_style(style)
        .with_thresholds(table)
        .with_policy(policy)
        .write_span(span, &mut phrase)
        .map_err(refusal)?;
    let mut out = String::new();
    let mut line = Line::new(&mut out);
    line.cell(&phrase)
        .cell(span.qualifier.id())
        .cell(span.amount.unit().as_str())
        .value(span.amount.count())
        .cell(locale_data(&locale).tag);
    line.end();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::cells;

    const HOUR: i64 = 3_600;
    const DAY: i64 = 86_400;

    /// CLDR 48's Hebrew and Tamil patterns (`he.xml`, `ta.xml`,
    /// `fields/field[@type="hour"]`), and no answer in the root's `-5 h` for
    /// a locale `hc-i18n` carries that has no phrases: CLDR 48 has no file
    /// for `ban`, none with patterns for `bo`, and only draft ones for
    /// `kab`.
    #[test]
    fn a_carried_locale_has_its_own_phrases_or_no_data() {
        let now = 1_700_000_000;
        let line = relative_time_line(now - 5 * HOUR, now, "long", false, "he").expect("a line");
        assert_eq!(cells(&line), ["לפני 5 שעות", "hour", "-5", "he"]);
        let line = relative_time_line(now - 5 * HOUR, now, "long", false, "iw").expect("a line");
        assert_eq!(cells(&line), ["לפני 5 שעות", "hour", "-5", "he"]);
        let line = relative_time_line(now - 5 * HOUR, now, "long", false, "ta").expect("a line");
        assert_eq!(cells(&line), ["5 மணிநேரம் முன்", "hour", "-5", "ta"]);
        let line = relative_time_line(now + 2 * HOUR, now, "long", false, "ta").expect("a line");
        assert_eq!(cells(&line)[0], "2 மணிநேரத்தில்");
        for tag in [
            "ban", "bo", "kab", "pa-Arab", "pa-PK", "cop", "zap", "sa", "zgh",
        ] {
            assert_eq!(
                relative_time_line(now - 5 * HOUR, now, "long", false, tag),
                Err(Refusal::NoData),
                "{tag}"
            );
            assert_eq!(
                relative_day_line(10, 14, "long", false, tag),
                Err(Refusal::NoData),
                "{tag}"
            );
            assert_eq!(
                duration_line(3_600, "long", 0, tag),
                Err(Refusal::NoData),
                "{tag}"
            );
        }
        // The root, and a language nobody carries, keep CLDR's root.
        let line = relative_time_line(now - 5 * HOUR, now, "long", false, "").expect("a line");
        assert_eq!(cells(&line), ["-5 h", "hour", "-5", "und"]);
        let line = relative_time_line(now - 5 * HOUR, now, "long", false, "xx").expect("a line");
        assert_eq!(cells(&line)[0], "-5 h");
    }

    /// CLDR 48's English relative-time patterns (`en.xml`,
    /// `fields/field[@type="hour"]`): *{0} hours ago*, and the word
    /// *yesterday* for a day of −1.
    #[test]
    fn three_hours_ago_is_three_hours_ago() {
        let now = 1_700_000_000;
        let line =
            relative_time_line(now - 3 * HOUR - 59, now, "long", false, "en").expect("a line");
        assert_eq!(cells(&line), ["3 hours ago", "hour", "-3", "en"]);
        let line = relative_time_line(now + 2 * DAY, now, "long", false, "en").expect("a line");
        assert_eq!(cells(&line), ["in 2 days", "day", "2", "en"]);
        let line = relative_time_line(now - DAY, now, "long", true, "en-GB").expect("a line");
        assert_eq!(cells(&line)[..3], ["yesterday", "day", "-1"]);
        let line = relative_time_line(now - DAY, now, "long", false, "ru").expect("a line");
        assert_eq!(cells(&line), ["1 день назад", "day", "-1", "ru"]);
        assert_eq!(
            relative_time_line(now, now, "wide", false, "en"),
            Err(Refusal::Unknown)
        );
        assert_eq!(
            relative_time_line(i64::MIN, 1, "long", false, "en"),
            Err(Refusal::OutOfRange)
        );
    }

    /// 23:30 on one day and 00:30 on the next are a day apart: the offset
    /// is the difference of the day numbers.
    #[test]
    fn a_day_is_counted_by_the_calendar() {
        let today = 739_888;
        let line = relative_day_line(today - 1, today, "long", true, "en").expect("a line");
        assert_eq!(cells(&line), ["yesterday", "day", "-1", "en"]);
        let line = relative_day_line(today, today, "long", true, "ja").expect("a line");
        assert_eq!(cells(&line), ["今日", "day", "0", "ja"]);
        let line = relative_day_line(today - 3, today, "long", false, "en").expect("a line");
        assert_eq!(cells(&line), ["3 days ago", "day", "-3", "en"]);
        let line = relative_day_at_line(
            today - 1,
            today,
            15 * 3_600 + 5 * 60 + 59,
            "long",
            true,
            "en",
        )
        .expect("a line");
        assert_eq!(
            cells(&line),
            ["yesterday at 15:05", "day", "-1", "15:05", "en"]
        );
        assert_eq!(
            relative_day_at_line(today, today, 86_400, "long", true, "en"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            relative_day_line(i64::MAX, -1, "long", true, "en"),
            Err(Refusal::OutOfRange)
        );
    }

    /// CLDR 48's English unit patterns and conjunction list.
    #[test]
    fn a_duration_is_phrased_in_its_units() {
        let span = 2 * HOUR + 30 * 60;
        assert_eq!(
            cells(&duration_line(span, "long", 0, "en").expect("a line")),
            ["2 hours and 30 minutes", "0", "en"]
        );
        assert_eq!(
            cells(&duration_line(span, "compact", 0, "en").expect("a line")),
            ["2h30m", "0", "en"]
        );
        assert_eq!(
            cells(&duration_line(DAY + span + 7, "long", 1, "en").expect("a line")),
            ["1 day", "0", "en"]
        );
        assert_eq!(
            cells(&duration_line(-span, "long", 0, "en").expect("a line"))[1],
            "1"
        );
        assert_eq!(
            cells(&duration_line(0, "long", 0, "en").expect("a line"))[0],
            "0 seconds"
        );
        assert_eq!(duration_line(1, "wide", 0, "en"), Err(Refusal::Unknown));
    }

    /// A catalogue that leaves a message of the function untranslated, or
    /// writes a raw `%d` or `%(value)s` for a word, does not serve it: the
    /// line is `humanize`'s English whole and says `en`, never two languages
    /// in one result. Japanese, Korean, Simplified Chinese and Slovak leave
    /// the `and` of their lists untranslated, and Korean, Bengali and
    /// Vietnamese hold placeholders in their powers.
    #[test]
    fn a_partly_translated_catalogue_answers_in_english_whole() {
        let seconds = 2 * DAY + 3_633;
        for tag in ["ja", "ko", "zh-CN", "sk"] {
            assert_eq!(
                cells(&precisedelta_line(seconds, 120_000, "seconds", "", 2, tag).expect("a line")),
                ["2 days, 1 hour and 33.12 seconds", "en"],
                "{tag}"
            );
        }
        for tag in [
            "ko", "bn-BD", "vi", "ja", "eu", "ca", "id", "nl", "pt-PT", "sk", "tlh",
        ] {
            assert_eq!(
                cells(&intword_line("1000000", 1, tag).expect("a line")),
                ["1.0 million", "en"],
                "{tag}"
            );
        }
        // A catalogue that translates every message of the function serves.
        assert_eq!(
            cells(&precisedelta_line(seconds, 120_000, "seconds", "", 2, "de").expect("a line")),
            ["2 Tage, 1 Stunde und 33.12 Sekunden", "de-DE"]
        );
        // The fine units are messages the function needs only for a minimum
        // unit below the second: German lacks them, so there it is English.
        assert_eq!(
            cells(&naturaltime_line(0, 5_000, true, "milliseconds", "de").expect("a line")),
            ["5 milliseconds ago", "en"]
        );
        assert_eq!(
            cells(&naturaltime_line(0, 5_000, true, "milliseconds", "ja").expect("a line")),
            ["5 milliseconds ago", "en"]
        );
        assert_eq!(
            cells(&naturaltime_line(5, 0, true, "seconds", "de").expect("a line")),
            ["vor 5 Sekunden", "de-DE"]
        );
    }

    /// Every line that writes words, in every locale `hc-i18n` carries and
    /// in the languages of the catalogues, writes no raw placeholder, and
    /// the language it says is `en` or one whose catalogue serves the
    /// function.
    #[test]
    fn no_locale_writes_a_placeholder_or_two_languages() {
        let mut tags: alloc::vec::Vec<&str> =
            hc_i18n::data::LOCALES.iter().map(|data| data.tag).collect();
        tags.extend(
            NaturalPhrases::catalogues()
                .iter()
                .map(|phrases| phrases.language),
        );
        tags.extend(["de-AT", "pt", "pt-AO", "zh-Hant-TW", "no", "sv-FI", ""]);
        for tag in tags {
            let lines = [
                (
                    NaturalWords::Apnumber,
                    apnumber_line(0, tag),
                    apnumber_line(0, "en"),
                ),
                (
                    NaturalWords::Intword,
                    intword_line("1234567890123", 1, tag),
                    intword_line("1234567890123", 1, "en"),
                ),
                (
                    NaturalWords::Naturalsize,
                    naturalsize_line(3_000.0, "decimal", 1, tag),
                    naturalsize_line(3_000.0, "decimal", 1, "en"),
                ),
                (
                    NaturalWords::Delta,
                    naturaldelta_line(3_600, 0, true, "seconds", tag),
                    naturaldelta_line(3_600, 0, true, "seconds", "en"),
                ),
                (
                    NaturalWords::DeltaFine,
                    naturaldelta_line(0, 5_000, true, "microseconds", tag),
                    naturaldelta_line(0, 5_000, true, "microseconds", "en"),
                ),
                (
                    NaturalWords::Time,
                    naturaltime_line(7_200, 0, true, "seconds", tag),
                    naturaltime_line(7_200, 0, true, "seconds", "en"),
                ),
                (
                    NaturalWords::Precise,
                    precisedelta_line(190_000, 0, "seconds", "", 2, tag),
                    precisedelta_line(190_000, 0, "seconds", "", 2, "en"),
                ),
                (
                    NaturalWords::PreciseFine,
                    precisedelta_line(1, 5_000, "microseconds", "", 2, tag),
                    precisedelta_line(1, 5_000, "microseconds", "", 2, "en"),
                ),
                (
                    NaturalWords::Day,
                    naturalday_line(739_001, 739_000, "", tag),
                    naturalday_line(739_001, 739_000, "", "en"),
                ),
                (
                    NaturalWords::Ordinal,
                    ordinal_line(22, "male", tag),
                    ordinal_line(22, "male", "en"),
                ),
            ];
            for (words, line, english) in lines {
                let line = line.expect("a line");
                let line = cells(&line);
                let english = english.expect("a line");
                let english = cells(&english);
                assert!(!line[0].contains('%'), "{tag} {words:?}: {}", line[0]);
                if line[1] == "en" {
                    assert_eq!(line[0], english[0], "{tag} {words:?}");
                } else {
                    let served = NaturalPhrases::by_catalogue(line[1]).expect("a catalogue");
                    assert!(served.translates(words), "{tag} {words:?}");
                }
            }
        }
    }

    /// The examples of `humanize` 4.16's documentation of `precisedelta`
    /// (a delta of two days, 3 633 seconds and 123 000 microseconds), and
    /// of `naturaldelta`, `naturaltime`, `ordinal` and `intcomma`.
    #[test]
    fn the_time_lines_are_pythons_humanize() {
        let seconds = 2 * DAY + 3_633;
        let precise = |minimum: &str, suppress: &str, decimals| {
            precisedelta_line(seconds, 123_000, minimum, suppress, decimals, "en")
        };
        assert_eq!(
            cells(&precise("seconds", "", 2).expect("a line")),
            ["2 days, 1 hour and 33.12 seconds", "en"]
        );
        assert_eq!(
            cells(&precise("microseconds", "", 2).expect("a line")),
            ["2 days, 1 hour, 33 seconds and 123 milliseconds", "en"]
        );
        assert_eq!(
            cells(&precise("seconds", "days", 4).expect("a line")),
            ["49 hours and 33.1230 seconds", "en"]
        );
        assert_eq!(precise("fortnights", "", 2), Err(Refusal::Unknown),);
        assert_eq!(
            precise("seconds", "seconds,minutes,hours,days,months,years", 2),
            Err(Refusal::OutOfRange)
        );
        let delta = |seconds, minimum: &str| naturaldelta_line(seconds, 0, true, minimum, "en");
        assert_eq!(
            cells(&delta(7 * DAY, "seconds").expect("a line"))[0],
            "7 days"
        );
        assert_eq!(
            cells(&delta(-30 * 60, "seconds").expect("a line"))[0],
            "30 minutes"
        );
        assert_eq!(delta(1, "hours"), Err(Refusal::OutOfRange));
        assert_eq!(delta(1, "wide"), Err(Refusal::Unknown));
        assert_eq!(
            naturaldelta_line(1, 1_000_000, true, "seconds", "en"),
            Err(Refusal::OutOfRange)
        );
        let time = |seconds| naturaltime_line(seconds, 0, true, "seconds", "en");
        assert_eq!(cells(&time(3).expect("a line"))[0], "3 seconds ago");
        assert_eq!(cells(&time(-3).expect("a line"))[0], "3 seconds from now");
        assert_eq!(cells(&time(0).expect("a line"))[0], "now");
        assert_eq!(
            cells(&ordinal_line(103, "male", "en").expect("a line")),
            ["103rd", "en"]
        );
        assert_eq!(
            cells(&ordinal_line(111, "FEMALE", "").expect("a line"))[0],
            "111th"
        );
        assert_eq!(ordinal_line(1, "neuter", "en"), Err(Refusal::Unknown));
        assert_eq!(
            cells(&intcomma_line("1234567", "en").expect("a line")),
            ["1,234,567", "en"]
        );
        assert_eq!(
            cells(&intcomma_line("-1234567", "de").expect("a line")),
            ["-1.234.567", "de-DE"]
        );
        assert_eq!(intcomma_line("12x", "en"), Err(Refusal::Malformed));
        assert_eq!(
            intcomma_line(&"9".repeat(40), "en"),
            Err(Refusal::OutOfRange)
        );
        assert_eq!(
            cells(&intcomma_float_line(12_345.678_9, 2, "en").expect("a line")),
            ["12,345.68", "en"]
        );
        assert_eq!(
            cells(&intcomma_float_line(1_234_567.25, -1, "en").expect("a line"))[0],
            "1,234,567.25"
        );
        assert_eq!(
            intcomma_float_line(1.0, 256, "en"),
            Err(Refusal::OutOfRange)
        );
    }

    /// The days of `naturalday`: *today*, *tomorrow*, *yesterday*, and a
    /// day written `%b %d`, with the year from five twelfths of a year off.
    #[test]
    fn a_day_is_natural_by_its_distance() {
        let today = 739_888;
        let day = |offset: i64, pattern: &str, tag: &str| {
            cells(&naturalday_line(today + offset, today, pattern, tag).expect("a line"))
                .iter()
                .map(|cell| (*cell).to_owned())
                .collect::<alloc::vec::Vec<_>>()
        };
        assert_eq!(day(0, "", "en"), ["today", "en"]);
        assert_eq!(day(1, "", "en"), ["tomorrow", "en"]);
        assert_eq!(day(-1, "", "en"), ["yesterday", "en"]);
        assert_eq!(day(0, "", "de"), ["heute", "de-DE"]);
        // 739 888 is 2026-09-29 (Python's `date.fromordinal`): ten days on is 9 October.
        assert_eq!(day(10, "", "en"), ["Oct 09", "en"]);
        assert_eq!(day(10, "%Y-%m-%d", "en"), ["2026-10-09", "en"]);
        let date = |offset: i64| {
            cells(&naturaldate_line(today + offset, today, "en").expect("a line"))[0].to_owned()
        };
        assert_eq!(date(10), "Oct 09");
        assert_eq!(date(152), "Feb 28");
        assert_eq!(date(153), "Mar 01 2027");
        assert_eq!(
            naturalday_line(i64::MAX, today, "", "en"),
            Err(Refusal::OutOfRange)
        );
    }

    /// The module documentation of `hc-humanize`'s `approximate` and
    /// `unit_choice`: 90 minutes is two hours rounded or an hour and a half
    /// by halves; 400 days is *just over a year*; 350 days is *nearly a
    /// year* (0.958 of the mean year).
    #[test]
    fn thresholds_and_rounding_are_the_callers() {
        let ninety = 90 * 60;
        assert_eq!(
            cells(&unit_choice_line(ninety, "default", "nearest").expect("a line")),
            ["hour", "2", "0", "default", "nearest"]
        );
        assert_eq!(
            cells(&unit_choice_line(ninety, "default", "nearest-half").expect("a line")),
            ["hour", "1", "1", "default", "nearest-half"]
        );
        assert_eq!(
            cells(&unit_choice_line(-ninety, "EXACT", "Truncate").expect("a line"))[..2],
            ["hour", "-1"]
        );
        assert_eq!(
            unit_choice_line(1, "wide", "nearest"),
            Err(Refusal::Unknown)
        );
        assert_eq!(unit_choice_line(1, "default", "up"), Err(Refusal::Unknown));
        let now = 1_700_000_000;
        let with = |then, rounding: &str| {
            cells(
                &relative_time_with_line(then, now, "long", false, "en", "default", rounding)
                    .expect("a line"),
            )
            .iter()
            .map(|cell| (*cell).to_owned())
            .collect::<alloc::vec::Vec<_>>()
        };
        assert_eq!(
            with(now - ninety, "truncate"),
            ["1 hour ago", "hour", "-1", "0", "en"]
        );
        assert_eq!(
            with(now - ninety, "nearest"),
            ["2 hours ago", "hour", "-2", "0", "en"]
        );
        assert_eq!(
            cells(
                &approximate_duration_line(400 * DAY, "long", "en", "default", "default")
                    .expect("a line")
            )[1..4],
            ["just-over", "year", "1"]
        );
        let nearly = approximate_duration_line(350 * DAY, "long", "en", "default", "default")
            .expect("a line");
        assert_eq!(cells(&nearly)[1..4], ["nearly", "year", "1"]);
        assert!(cells(&nearly)[0].contains("nearly"), "{nearly}");
        assert_eq!(
            approximate_duration_line(1, "long", "en", "default", "sloppy"),
            Err(Refusal::Unknown)
        );
    }
}
