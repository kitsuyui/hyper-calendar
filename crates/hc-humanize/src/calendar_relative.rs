//! Calendar-relative phrasing: *yesterday*, *yesterday at 3 pm*, *last
//! Tuesday*, *next month*.
//!
//! # The bug this module exists to avoid
//!
//! 23:30 on Monday and 00:30 on Tuesday are one hour apart. They are also
//! one *day* apart, and a person asked about the earlier one says
//! "yesterday", not "an hour ago". A humaniser that subtracts instants and
//! divides by 86 400 says "an hour ago" and is wrong — not imprecise,
//! wrong, because the reader's mental model is the calendar and not the
//! clock.
//!
//! So nothing here divides a span. The day offset comes from subtracting two
//! [`Rd`] fixed day numbers, which is an exact integer count of calendar
//! days by construction, and the time of day comes separately from
//! [`hc_calendar::CivilTime`]. The two are combined by a pattern, never by
//! arithmetic.
//!
//! The same argument applies one level up: *last Tuesday* means the Tuesday
//! of the previous week, and which week a day is in depends on where the
//! locale starts its week. [`week_offset`] asks
//! [`hc_i18n::names::first_day_of_week`] and counts weeks between the two
//! week-start days; it never divides the day difference by seven.
//!
//! # Within the same day
//!
//! When the day offset is zero the question becomes a clock question again,
//! and [`CalendarRelativeFormatter::write_between`] falls back to hours,
//! minutes or seconds of elapsed civil time. That is the one place a span is
//! divided, and it is safe there precisely because both ends are on the same
//! calendar day.

use core::fmt;

use hc_calendar::{CivilDateTime, CivilTime, Rd, Weekday};
use hc_core::Duration;
use hc_i18n::names::{NameContext, NameWidth, first_day_of_week, weekday_name};

use crate::error::{HumanizeError, HumanizeResult};
use crate::lookup;
use crate::pattern::RelativeStyle;
use crate::relative::{Numeric, RelativeTimeFormatter};
use crate::render;
use crate::unit::{TimeUnit, UnitAmount};
use crate::unit_choice::{RoundingPolicy, Thresholds, count_in};

/// The number of calendar days from one fixed day to another.
///
/// Exact by construction: [`Rd`] counts days, so this is a subtraction and
/// not a division.
#[must_use]
pub const fn day_offset(from: Rd, to: Rd) -> i64 {
    to.days_since(from)
}

/// The number of whole weeks between the weeks the two days fall in.
///
/// Both days are first moved back to the start of their own week, using the
/// locale's first day of the week — Monday in most of Europe, Sunday in the
/// United States and Japan, Saturday in much of the Arab world — and only
/// then subtracted. That is what makes *last Tuesday* mean the Tuesday of
/// the previous week rather than "between 7 and 13 days ago".
#[must_use]
pub fn week_offset(locale: &hc_i18n::Locale, from: Rd, to: Rd) -> i64 {
    let first = first_day_of_week(locale);
    let from_week = first.on_or_before(from);
    let to_week = first.on_or_before(to);
    to_week.days_since(from_week) / 7
}

/// Express a calendar day difference as an amount, promoting days to larger
/// units through a threshold table.
///
/// The unit is never shorter than a day: the input is a count of days and
/// there is nothing finer in it to recover.
///
/// # Errors
///
/// Returns [`HumanizeError::Overflow`] if the count does not fit an `i64`.
pub fn day_amount(
    from: Rd,
    to: Rd,
    thresholds: &Thresholds,
    policy: RoundingPolicy,
) -> HumanizeResult<UnitAmount> {
    let days = day_offset(from, to);
    let span = Duration::from_days(days);
    let mut unit = thresholds.unit_for(span);
    if unit < TimeUnit::Day {
        unit = TimeUnit::Day;
    }
    count_in(span, unit, policy)
}

/// The elapsed civil time between two times of the same day.
///
/// Leap seconds are not modelled here: [`CivilTime::since_midnight`] counts
/// them, so a difference spanning 23:59:60 is one second longer than the
/// clock faces suggest, which is the truthful answer.
#[must_use]
pub fn time_of_day_difference(from: CivilTime, to: CivilTime) -> Duration {
    to.since_midnight()
        .checked_sub(from.since_midnight())
        .unwrap_or(Duration::ZERO)
}

/// Formats a calendar-relative phrase.
#[derive(Debug, Clone, Copy)]
pub struct CalendarRelativeFormatter {
    inner: RelativeTimeFormatter,
}

impl CalendarRelativeFormatter {
    /// A formatter for a locale.
    ///
    /// The numeric preference defaults to [`Numeric::Auto`] here, unlike
    /// [`RelativeTimeFormatter`]: the whole point of a calendar-relative
    /// phrase is to say *yesterday* when the language has the word.
    #[must_use]
    pub const fn new(locale: hc_i18n::Locale) -> Self {
        Self {
            inner: RelativeTimeFormatter::new(locale).with_numeric(Numeric::Auto),
        }
    }

    /// The same formatter in another style.
    #[must_use]
    pub const fn with_style(mut self, style: RelativeStyle) -> Self {
        self.inner = self.inner.with_style(style);
        self
    }

    /// The same formatter with another numeric preference.
    #[must_use]
    pub const fn with_numeric(mut self, numeric: Numeric) -> Self {
        self.inner = self.inner.with_numeric(numeric);
        self
    }

    /// The locale.
    #[must_use]
    pub const fn locale(&self) -> &hc_i18n::Locale {
        self.inner.locale()
    }

    /// The underlying relative-time formatter.
    #[must_use]
    pub const fn relative(&self) -> &RelativeTimeFormatter {
        &self.inner
    }

    /// Write the phrase for a calendar day difference.
    ///
    /// With [`Numeric::Auto`] and the default thresholds, −1 is *yesterday*,
    /// 0 is *today* and −30 is *last month*.
    ///
    /// # Errors
    ///
    /// As [`RelativeTimeFormatter::write`], plus
    /// [`HumanizeError::Overflow`].
    pub fn write_day<W: fmt::Write>(&self, from: Rd, to: Rd, out: &mut W) -> HumanizeResult<()> {
        self.write_day_with(
            from,
            to,
            &Thresholds::DEFAULT,
            RoundingPolicy::Truncate,
            out,
        )
    }

    /// Write the phrase for a calendar day difference under a chosen
    /// threshold table and rounding policy.
    ///
    /// # Errors
    ///
    /// As [`Self::write_day`].
    pub fn write_day_with<W: fmt::Write>(
        &self,
        from: Rd,
        to: Rd,
        thresholds: &Thresholds,
        policy: RoundingPolicy,
        out: &mut W,
    ) -> HumanizeResult<()> {
        self.inner
            .write_amount(day_amount(from, to, thresholds, policy)?, out)
    }

    /// Write *yesterday at 15:30*, given the time already formatted.
    ///
    /// The time is a `&str` rather than a [`CivilTime`] because rendering a
    /// time of day is `hc-format`'s job, not this crate's: an hour cycle, a
    /// day period and a numbering system are a formatter's worth of
    /// decisions. [`write_clock_time`] is here for callers that only want
    /// something plain.
    ///
    /// # Errors
    ///
    /// As [`Self::write_day`].
    pub fn write_day_at<W: fmt::Write>(
        &self,
        from: Rd,
        to: Rd,
        time: &str,
        out: &mut W,
    ) -> HumanizeResult<()> {
        let pattern = lookup::at_pattern(self.locale());
        let mut day = |sink: &mut W| self.write_day(from, to, sink);
        let mut clock = |sink: &mut W| -> HumanizeResult<()> {
            sink.write_str(time)?;
            Ok(())
        };
        render::write_pattern(pattern, &mut [&mut day, &mut clock], out)
    }

    /// Write *last Tuesday*, *this Tuesday*, *next Tuesday*.
    ///
    /// Outside the three weeks the language has words for, this falls back
    /// to the plain weekday name; a caller that wants *3 weeks ago* there
    /// should ask [`Self::write_day`] instead.
    ///
    /// # Errors
    ///
    /// Returns [`HumanizeError::NoPattern`] if the locale names no weekday
    /// at the requested width, which cannot happen for the data shipped
    /// here.
    pub fn write_weekday<W: fmt::Write>(
        &self,
        from: Rd,
        to: Rd,
        out: &mut W,
    ) -> HumanizeResult<()> {
        let locale = self.locale();
        let name = weekday_name(
            locale,
            Weekday::from_rd(to),
            weekday_width(self.inner.style()),
            NameContext::Standalone,
        )
        .ok_or(HumanizeError::NoPattern)?;
        let patterns = lookup::weekday_patterns(locale);
        let pattern = match week_offset(locale, from, to) {
            -1 => patterns.previous,
            0 => patterns.current,
            1 => patterns.next,
            _ => "{0}",
        };
        let mut day_name = |sink: &mut W| -> HumanizeResult<()> {
            sink.write_str(name)?;
            Ok(())
        };
        render::write_pattern(pattern, &mut [&mut day_name], out)
    }

    /// Write the phrase for the distance between two civil date-times.
    ///
    /// The day difference decides first. Only when the two fall on the same
    /// calendar day does the clock get a say, and then the answer is in
    /// hours, minutes or seconds of elapsed civil time.
    ///
    /// # Errors
    ///
    /// As [`Self::write_day`].
    pub fn write_between<W: fmt::Write>(
        &self,
        from: CivilDateTime,
        to: CivilDateTime,
        out: &mut W,
    ) -> HumanizeResult<()> {
        if from.day != to.day {
            return self.write_day(from.day, to.day, out);
        }
        let span = time_of_day_difference(from.time, to.time);
        let unit = Thresholds::DEFAULT.unit_for(span).min(TimeUnit::Hour);
        self.inner
            .write_amount(count_in(span, unit, RoundingPolicy::Truncate)?, out)
    }

    /// The phrase for a calendar day difference.
    ///
    /// # Errors
    ///
    /// As [`Self::write_day`].
    #[cfg(feature = "alloc")]
    pub fn format_day(&self, from: Rd, to: Rd) -> HumanizeResult<alloc::string::String> {
        let mut text = alloc::string::String::new();
        self.write_day(from, to, &mut text)?;
        Ok(text)
    }

    /// The phrase for the distance between two civil date-times.
    ///
    /// # Errors
    ///
    /// As [`Self::write_between`].
    #[cfg(feature = "alloc")]
    pub fn format_between(
        &self,
        from: CivilDateTime,
        to: CivilDateTime,
    ) -> HumanizeResult<alloc::string::String> {
        let mut text = alloc::string::String::new();
        self.write_between(from, to, &mut text)?;
        Ok(text)
    }
}

/// The weekday name width a relative style asks for.
///
/// Narrow maps to `Short`, not to `Narrow`: CLDR narrow weekday names repeat
/// — English narrow is M T W T F S S — so they identify a column in a grid
/// and nothing else. *Last T* is not a phrase.
const fn weekday_width(style: RelativeStyle) -> NameWidth {
    match style {
        RelativeStyle::Long => NameWidth::Wide,
        RelativeStyle::Short => NameWidth::Abbreviated,
        RelativeStyle::Narrow => NameWidth::Short,
    }
}

/// Write a time of day as `H:MM` on a 24-hour clock, in the locale's digits.
///
/// A convenience for [`CalendarRelativeFormatter::write_day_at`], not a time
/// formatter: it has no hour cycle, no day period and no seconds. When
/// `hc-format` exists, use that instead.
///
/// # Errors
///
/// Returns [`HumanizeError::Number`] if the locale's numbering system
/// cannot render the hour or minute, and [`HumanizeError::WriteFailed`] if
/// the sink refuses a write.
pub fn write_clock_time<W: fmt::Write>(
    locale: &hc_i18n::Locale,
    time: CivilTime,
    out: &mut W,
) -> HumanizeResult<()> {
    let digits = hc_i18n::NumberingSystem::for_locale(locale);
    digits.write_integer(i64::from(time.hour()), out)?;
    out.write_str(":")?;
    if time.minute() < 10 {
        digits.write_integer(0, out)?;
    }
    digits.write_integer(i64::from(time.minute()), out)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::RelativeStyle;

    /// 2021-01-07, which was a Thursday.
    ///
    /// The Rata Die of 2021-01-01 is 365 × 2020 + 505 − 20 + 5 + 1 =
    /// 737 791, so the seventh is 737 797. Every test below leans on the
    /// weekday, so the first one checks it rather than trusting the
    /// arithmetic here.
    const THURSDAY: Rd = Rd(737_797);

    fn locale(tag: &str) -> hc_i18n::Locale {
        tag.parse().expect("well-formed tag")
    }

    fn formatter(tag: &str) -> CalendarRelativeFormatter {
        CalendarRelativeFormatter::new(locale(tag))
    }

    fn at(day: Rd, hour: u8, minute: u8) -> CivilDateTime {
        CivilDateTime::new(day, CivilTime::hms(hour, minute, 0).expect("a valid time"))
    }

    #[test]
    fn the_anchor_day_is_the_thursday_it_claims_to_be() {
        // Rd 737 796 is 2021-01-07, a Thursday. Everything below depends on
        // it, so it is checked rather than assumed.
        assert_eq!(Weekday::from_rd(THURSDAY), Weekday::Thursday);
    }

    #[test]
    fn half_past_eleven_and_half_past_midnight_are_an_hour_apart_and_a_day_apart() {
        // The bug this module exists to avoid: these two instants are 3 600
        // seconds apart, and a reader asked about the earlier one says
        // "yesterday".
        let late_last_night = at(THURSDAY, 23, 30);
        let early_this_morning = at(THURSDAY + 1, 0, 30);
        let elapsed = Duration::from_secs(3_600);
        assert_eq!(
            early_this_morning
                .nominal_duration_since(late_last_night)
                .expect("in range"),
            elapsed
        );
        assert_eq!(
            formatter("en")
                .format_between(early_this_morning, late_last_night)
                .expect("a phrase"),
            "yesterday"
        );
        // and the other way round.
        assert_eq!(
            formatter("en")
                .format_between(late_last_night, early_this_morning)
                .expect("a phrase"),
            "tomorrow"
        );
    }

    #[test]
    fn two_times_on_the_same_day_fall_back_to_the_clock() {
        assert_eq!(
            formatter("en")
                .format_between(at(THURSDAY, 9, 0), at(THURSDAY, 6, 0))
                .expect("a phrase"),
            "3 hours ago"
        );
        assert_eq!(
            formatter("en")
                .format_between(at(THURSDAY, 9, 0), at(THURSDAY, 9, 40))
                .expect("a phrase"),
            "in 40 minutes"
        );
    }

    #[test]
    fn the_day_offset_is_a_subtraction_and_not_a_division() {
        assert_eq!(day_offset(THURSDAY, THURSDAY + 1), 1);
        assert_eq!(day_offset(THURSDAY, THURSDAY - 400), -400);
        assert_eq!(day_offset(THURSDAY, THURSDAY), 0);
    }

    #[test]
    fn the_nearby_days_are_words_and_the_far_ones_are_numbers() {
        let en = formatter("en");
        assert_eq!(
            en.format_day(THURSDAY, THURSDAY - 1).expect("a"),
            "yesterday"
        );
        assert_eq!(en.format_day(THURSDAY, THURSDAY).expect("a"), "today");
        assert_eq!(
            en.format_day(THURSDAY, THURSDAY + 1).expect("a"),
            "tomorrow"
        );
        assert_eq!(
            en.format_day(THURSDAY, THURSDAY - 2).expect("a"),
            "the day before yesterday"
        );
        assert_eq!(
            en.format_day(THURSDAY, THURSDAY - 4).expect("a"),
            "4 days ago"
        );
    }

    #[test]
    fn a_day_difference_promotes_out_of_days_when_the_table_says_so() {
        let en = formatter("en");
        assert_eq!(
            en.format_day(THURSDAY, THURSDAY - 10).expect("a"),
            "last week"
        );
        assert_eq!(
            en.format_day(THURSDAY, THURSDAY - 40).expect("a"),
            "last month"
        );
        assert_eq!(
            en.format_day(THURSDAY, THURSDAY - 400).expect("a"),
            "last year"
        );
        assert_eq!(
            en.format_day(THURSDAY, THURSDAY + 800).expect("a"),
            "in 2 years"
        );
    }

    #[test]
    fn the_caller_can_refuse_the_promotion() {
        static DAYS_ONLY: &[crate::Threshold] = &[crate::Threshold::new(TimeUnit::Day, i64::MAX)];
        let mut text = alloc::string::String::new();
        formatter("en")
            .write_day_with(
                THURSDAY,
                THURSDAY - 40,
                &Thresholds::new(DAYS_ONLY),
                RoundingPolicy::Truncate,
                &mut text,
            )
            .expect("a phrase");
        assert_eq!(text, "40 days ago");
    }

    #[test]
    fn last_tuesday_is_the_tuesday_of_the_previous_week() {
        let en = formatter("en");
        let tuesday_this_week = Weekday::Tuesday.on_or_before(THURSDAY);
        let tuesday_last_week = tuesday_this_week - 7;
        let tuesday_next_week = tuesday_this_week + 7;
        let mut text = alloc::string::String::new();
        en.write_weekday(THURSDAY, tuesday_last_week, &mut text)
            .expect("a phrase");
        assert_eq!(text, "last Tuesday");
        text.clear();
        en.write_weekday(THURSDAY, tuesday_this_week, &mut text)
            .expect("a phrase");
        assert_eq!(text, "this Tuesday");
        text.clear();
        en.write_weekday(THURSDAY, tuesday_next_week, &mut text)
            .expect("a phrase");
        assert_eq!(text, "next Tuesday");
        text.clear();
        en.write_weekday(THURSDAY, tuesday_this_week - 21, &mut text)
            .expect("a phrase");
        assert_eq!(text, "Tuesday");
    }

    #[test]
    fn the_same_sunday_is_last_week_or_this_week_depending_on_the_locale() {
        // The Sunday four days before this Thursday opens the week in the
        // United States and closes the previous one in Britain, so the same
        // two days give different answers — which is why the week offset has
        // to ask the locale instead of dividing by seven.
        let sunday = Weekday::Sunday.on_or_before(THURSDAY);
        assert_eq!(day_offset(THURSDAY, sunday), -4);
        assert_eq!(first_day_of_week(&locale("en-GB")), Weekday::Monday);
        assert_eq!(first_day_of_week(&locale("en-US")), Weekday::Sunday);
        assert_eq!(week_offset(&locale("en-GB"), THURSDAY, sunday), -1);
        assert_eq!(week_offset(&locale("en-US"), THURSDAY, sunday), 0);
        let mut text = alloc::string::String::new();
        formatter("en-GB")
            .write_weekday(THURSDAY, sunday, &mut text)
            .expect("a phrase");
        assert_eq!(text, "last Sunday");
        text.clear();
        formatter("en-US")
            .write_weekday(THURSDAY, sunday, &mut text)
            .expect("a phrase");
        assert_eq!(text, "this Sunday");
    }

    #[test]
    fn the_weekday_name_narrows_no_further_than_the_short_width() {
        // English narrow weekdays are M T W T F S S; "last T" is not a
        // phrase, so the narrow style uses the short names instead.
        let mut text = alloc::string::String::new();
        formatter("en")
            .with_style(RelativeStyle::Narrow)
            .write_weekday(THURSDAY, THURSDAY - 7, &mut text)
            .expect("a phrase");
        assert!(text.len() > "last T".len(), "{text}");
        assert!(text.starts_with("last "), "{text}");
    }

    #[test]
    fn a_day_phrase_and_a_time_join_through_the_locales_pattern() {
        let mut time = alloc::string::String::new();
        write_clock_time(
            &locale("en"),
            CivilTime::hms(15, 5, 0).expect("a valid time"),
            &mut time,
        )
        .expect("a time");
        assert_eq!(time, "15:05");
        let mut text = alloc::string::String::new();
        formatter("en")
            .write_day_at(THURSDAY, THURSDAY - 1, &time, &mut text)
            .expect("a phrase");
        assert_eq!(text, "yesterday at 15:05");
        text.clear();
        formatter("de")
            .write_day_at(THURSDAY, THURSDAY - 1, &time, &mut text)
            .expect("a phrase");
        assert_eq!(text, "gestern um 15:05");
    }

    #[test]
    fn the_clock_helper_pads_the_minute_and_uses_the_locales_digits() {
        let mut text = alloc::string::String::new();
        write_clock_time(
            &locale("en"),
            CivilTime::hms(9, 0, 0).expect("a valid time"),
            &mut text,
        )
        .expect("a time");
        assert_eq!(text, "9:00");
        text.clear();
        write_clock_time(
            &locale("ar"),
            CivilTime::hms(9, 5, 0).expect("a valid time"),
            &mut text,
        )
        .expect("a time");
        assert_eq!(text, "٩:٠٥");
    }

    #[test]
    fn a_calendar_formatter_prefers_words_but_can_be_told_not_to() {
        assert_eq!(formatter("en").relative().numeric(), Numeric::Auto);
        let always = formatter("en").with_numeric(Numeric::Always);
        assert_eq!(
            always.format_day(THURSDAY, THURSDAY - 1).expect("a"),
            "1 day ago"
        );
    }

    #[test]
    fn the_elapsed_time_within_a_day_counts_a_leap_second_if_there_is_one() {
        // CivilTime::since_midnight counts 23:59:60 as a real second, so a
        // difference that spans it is one second longer than the clock faces
        // suggest. That is the truthful answer, not an off-by-one.
        let ordinary = CivilTime::hms(23, 59, 59).expect("a valid time");
        let leap = CivilTime::new(23, 59, 60, 0).expect("a valid leap second");
        assert_eq!(
            time_of_day_difference(ordinary, leap),
            Duration::from_secs(1)
        );
    }

    #[test]
    fn every_shipped_locale_can_phrase_the_nearby_days() {
        for data in crate::data::LOCALES {
            let calendar = formatter(data.tag);
            for offset in [-2i64, -1, 0, 1, 2, -9, 40] {
                let phrase = calendar
                    .format_day(THURSDAY, THURSDAY + offset)
                    .unwrap_or_else(|_| panic!("{} {offset}", data.tag));
                assert!(!phrase.is_empty(), "{} {offset}", data.tag);
                assert!(!phrase.contains("{0}"), "{} {offset}", data.tag);
            }
            let mut text = alloc::string::String::new();
            calendar
                .write_weekday(THURSDAY, THURSDAY - 7, &mut text)
                .unwrap_or_else(|_| panic!("{} weekday", data.tag));
            assert!(!text.contains("{0}"), "{}", data.tag);
        }
    }
}
