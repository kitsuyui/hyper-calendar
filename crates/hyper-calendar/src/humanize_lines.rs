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
//! The crate does not know what "now" is, so each line takes both ends
//! from the caller; nor does it know a time zone, so a relative day takes
//! the two fixed days a caller has already read off its own clock. The
//! last cell of each line is the tag of the `hc-humanize` data entry the
//! locale resolved to.

use alloc::string::String;

use hc_calendar::{CivilTime, Rd};
use hc_core::Duration;
use hc_humanize::calendar_relative::{day_amount, write_clock_time};
use hc_humanize::lookup::locale_data;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boundary::cells;

    const HOUR: i64 = 3_600;
    const DAY: i64 = 86_400;

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
}
