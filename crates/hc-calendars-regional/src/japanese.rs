//! The Japanese calendar with imperial era years (和暦).
//!
//! A Japanese date is an era, a year within that era, a month and a day —
//! 令和8年9月21日. The year restarts at 1 whenever a new era is proclaimed,
//! and year 1 is written 元年 rather than 1年.
//!
//! # Two calendars under one name
//!
//! The CLDR identifier `japanese` names a *year numbering*, not a calendar
//! structure, and the structure underneath it changed on a known day:
//!
//! | Days | Structure | Comes from |
//! |---|---|---|
//! | 1873-01-01 onward | proleptic Gregorian | [`hc_calendars_solar::gregorian`] |
//! | 1844-02-18 to 1872-12-31 | Tenpō lunisolar, with leap months | [`hc_calendars_lunar::japanese_tenpo`] |
//! | before 1844-02-18 | *not supported* | — |
//!
//! The Dajōkan decree of 9 November 1872 declared that 明治5年12月3日 would
//! be 1 January 1873 in the solar calendar. So 明治5年12月2日 is
//! 1872-12-31, the day after it is 明治6年1月1日 = 1873-01-01, and 明治5年
//! has no third day of its twelfth month. This module reproduces that: it
//! asks the Tenpō calendar for every day up to the reform and the Gregorian
//! calendar for every day after it, and the two meet exactly at the decree.
//!
//! Most implementations of `japanese` get this wrong by running the
//! Gregorian calendar backwards through 明治 and out the far side, which
//! produces dates like "明治5年12月15日" that nobody in Japan ever wrote.
//!
//! # Where this implementation stops, and why
//!
//! Conversion stops at **1844-02-18**, which is 天保15年1月1日 and the first
//! day the Tenpō calendar was in force. That is not where the *eras* stop —
//! [`crate::nengo`] carries all 248 of them back to 大化 in 645 — it is
//! where the *lunisolar arithmetic* stops, because the Tenpō calendar is
//! the only pre-reform Japanese calendar implemented in this workspace.
//!
//! Japan used at least five earlier systems: Senmyō-reki (862–1685),
//! Jōkyō-reki (1685–1755), Hōryaku-reki (1755–1798), Kansei-reki
//! (1798–1844). They differ from Tenpō in their solar theory and their
//! intercalation, so running the Tenpō rules backwards would produce dates
//! that are wrong by a day here and a whole month there, with no warning to
//! the caller. This module refuses instead, and
//! [`hc_calendar::CalendarMeta::earliest`] says so, so that a caller asking
//! for 元禄15年12月14日 gets [`CalendarError::BeforeEpoch`] rather than a
//! plausible lie.
//!
//! [`crate::nengo::era_at`] will still tell you which era was in force on
//! any day from 645 onward. It just will not tell you the month and day.
//!
//! # Era boundaries
//!
//! An era change happens mid-year, so a Western year can carry two era
//! years: 1989 is both 昭和64年 (to 1 January 7th) and 平成元年 (from
//! January 8th). Both are representable and both round-trip. What is *not*
//! representable is a date on the wrong side of the boundary: 昭和64年1月8日
//! never existed, so [`JapaneseCalendar::to_fixed`] rejects it rather than
//! quietly returning the same day as 平成元年1月8日.
//!
//! The era table uses the 公式 (official, retroactive) boundary dates, under
//! which an era ends the day before the next begins. See
//! [`crate::nengo::Nengo::start`].
//!
//! # Accuracy
//!
//! From 1873 the calendar is exact integer arithmetic. Before it, every day
//! comes from the Tenpō calendar, which is astronomical: new moons are good
//! to about a minute but apparent solar longitude only to about 0.01°, so a
//! month boundary computed here can differ by one day from what the
//! Japanese calendar bureau actually promulgated. See
//! [`hc_calendars_lunar::japanese_tenpo`] for the meridian and the model.
//! No table of the promulgated Tenpō months is shipped with this crate, so
//! this module cannot report a disagreement rate against one.

use core::fmt;

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};
use hc_calendars_lunar::{LunisolarDate, japanese_tenpo};
use hc_calendars_solar::gregorian;

use crate::nengo::{self, Court, Nengo};

/// The CLDR identifier of this calendar.
pub const ID: CalendarId = CalendarId("japanese");

/// The first day the solar calendar was in force: 明治6年1月1日, decreed to
/// follow 明治5年12月2日 directly.
pub const GREGORIAN_ADOPTION: Rd = Rd(683_735);

/// The earliest day this calendar converts: 天保15年1月1日 = 1844-02-18, the
/// first day of the Tenpō calendar.
pub const EARLIEST: Rd = japanese_tenpo::EARLIEST;

/// The latest day this calendar converts, 9999-12-31.
///
/// The bound is arbitrary and says only where this implementation stops
/// extrapolating 令和 forward. Nothing predicts how long the era will last.
pub const LATEST: Rd = Rd(3_652_059);

/// A Japanese date: an era, a year within it, a month and a day.
///
/// The month carries a leap flag, because before 1873 the calendar was
/// lunisolar and some years had a repeated month — 明治3年 had a leap tenth
/// month, written 閏10月.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JapaneseDate {
    /// The era. Year 1 of this era is 元年.
    pub era: &'static Nengo,
    /// The year within the era, counting from 1.
    pub year: i64,
    /// The month.
    pub month: Month,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl JapaneseDate {
    /// A date, without validation; the calendar validates it.
    #[must_use]
    pub const fn new(era: &'static Nengo, year: i64, month: Month, day: u8) -> Self {
        Self {
            era,
            year,
            month,
            day,
        }
    }

    /// A date whose era is named by identifier, kanji or reading.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::UnknownEra`] when no era answers to `era`.
    pub fn from_era_name(era: &str, year: i64, month: Month, day: u8) -> CalendarResult<Self> {
        let era = nengo::find(era).ok_or(CalendarError::UnknownEra)?;
        Ok(Self::new(era, year, month, day))
    }

    /// The Japanese calendar year this date falls in, numbered by the
    /// Western year it begins in.
    ///
    /// From 1873 this is simply the Gregorian year. Before it, it is the
    /// lunisolar year as [`hc_calendars_lunar::japanese_tenpo`] numbers
    /// them, which is the Gregorian year containing that year's first
    /// month.
    #[must_use]
    pub const fn calendar_year(&self) -> i64 {
        self.era.start_year + self.year - 1
    }

    /// Whether this date falls in an intercalary month.
    #[must_use]
    pub const fn is_in_leap_month(&self) -> bool {
        self.month.leap
    }
}

impl fmt::Display for JapaneseDate {
    /// Writes the date the way Japanese documents do: `令和8年9月21日`, with
    /// year 1 written `元年` and an intercalary month prefixed `閏`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.era.kanji)?;
        if self.year == 1 {
            f.write_str("元年")?;
        } else {
            write!(f, "{}年", self.year)?;
        }
        if self.month.leap {
            f.write_str("閏")?;
        }
        write!(f, "{}月{}日", self.month.ordinal, self.day)
    }
}

/// The Japanese calendar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct JapaneseCalendar;

/// The Japanese calendar year, month and day of a fixed day, ignoring eras.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] before [`EARLIEST`] and
/// [`CalendarError::AfterSupportedRange`] after [`LATEST`].
pub fn calendar_year_month_day(rd: Rd) -> CalendarResult<(i64, Month, u8)> {
    if rd < EARLIEST {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd > LATEST {
        return Err(CalendarError::AfterSupportedRange);
    }
    if rd >= GREGORIAN_ADOPTION {
        let (year, month, day) = gregorian::from_fixed(rd)?;
        Ok((year, Month::regular(month), day))
    } else {
        let date = japanese_tenpo::ENGINE.from_fixed(rd)?;
        Ok((date.year, date.month, date.day))
    }
}

/// The fixed day of a Japanese calendar year, month and day, ignoring eras.
///
/// # Errors
///
/// Returns a [`CalendarError`] when the date does not exist, including
/// [`CalendarError::MonthOutOfRange`] for an intercalary month in the solar
/// half of the range, where there are none.
pub fn calendar_to_fixed(year: i64, month: Month, day: u8) -> CalendarResult<Rd> {
    if year >= 1873 {
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let rd = gregorian::to_fixed(year, month.ordinal, day)?;
        if rd < GREGORIAN_ADOPTION {
            return Err(CalendarError::BeforeEpoch);
        }
        if rd > LATEST {
            return Err(CalendarError::AfterSupportedRange);
        }
        Ok(rd)
    } else {
        japanese_tenpo::ENGINE.to_fixed(LunisolarDate::new(year, month, day))
    }
}

/// The half-open span of fixed days an era covers, within this calendar's
/// supported range.
///
/// # Errors
///
/// Returns [`CalendarError::UnknownEra`] for an era whose start day the
/// sources do not fix.
fn era_span(era: &Nengo) -> CalendarResult<(Rd, Rd)> {
    let start = era.require_start()?;
    let end = match nengo::next_in_stream(era, Court::Unified).and_then(|next| next.start) {
        Some(next) => next,
        None => Rd(LATEST.0 + 1),
    };
    Ok((start, end))
}

impl Calendar for JapaneseCalendar {
    type Date = JapaneseDate;

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Japanese (imperial eras)",
            year_kind: YearKind::EraRelative,
            has_leap_months: true,
            // True only of the pre-1873 half, but a caller deciding whether
            // to trust a historical date needs the warning and a single
            // flag cannot be qualified.
            is_astronomical: true,
            earliest: Some(EARLIEST),
            latest: Some(LATEST),
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        if date.year < 1 {
            return Err(CalendarError::YearOutOfRange);
        }
        let (start, end) = era_span(date.era)?;
        let rd = calendar_to_fixed(date.calendar_year(), date.month, date.day)?;
        if rd < start || rd >= end {
            // Distinguish "this era never had such a year" from "this era
            // had that year but not that day of it": 明治50年 is the first,
            // 昭和64年1月8日 the second.
            let (first_year, _, _) = calendar_year_month_day(start)?;
            let (last_year, _, _) = calendar_year_month_day(Rd(end.0 - 1))?;
            let year = date.calendar_year();
            if year < first_year || year > last_year {
                return Err(CalendarError::YearOutOfRange);
            }
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(rd)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = calendar_year_month_day(rd)?;
        let era = nengo::era_at(rd, Court::Unified)?;
        Ok(JapaneseDate {
            era,
            year: year - era.start_year + 1,
            month,
            day,
        })
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields {
            era: Some(date.era.id),
            year: date.year,
            month: Some(date.month),
            day: Some(date.day),
            extra: hc_calendar::fields::ExtraFields::new(),
        })
    }

    /// Reads era-tagged fields as a Japanese date.
    ///
    /// Fields with no era are read the way ICU reads an *extended year*:
    /// [`DateFields::year`] is the Japanese calendar year itself — the
    /// Gregorian year from 1873, the lunisolar year number before it — with
    /// no era attached. That is what makes `days_in_year(1990)` answerable
    /// through [`hc_calendar::DynCalendar`], which has nowhere to put an
    /// era.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::UnknownEra`], [`CalendarError::MissingField`]
    /// or a range error.
    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        let day = fields.require_day()?;
        match fields.era {
            Some(name) => {
                let era = nengo::find(name).ok_or(CalendarError::UnknownEra)?;
                Ok(JapaneseDate::new(era, fields.year, month, day))
            }
            None => self.from_fixed(calendar_to_fixed(fields.year, month, day)?),
        }
    }
}

#[cfg(test)]
mod tests {
    use hc_calendar::Weekday;

    use super::*;

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("valid Gregorian date")
    }

    fn japanese(era: &str, year: i64, month: u8, day: u8) -> JapaneseDate {
        JapaneseDate::from_era_name(era, year, Month::regular(month), day).expect("known era")
    }

    #[test]
    fn reiwa_began_on_the_first_of_may_2019() {
        let day = greg(2019, 5, 1);
        assert_eq!(
            JapaneseCalendar.from_fixed(day),
            Ok(japanese("令和", 1, 5, 1))
        );
        assert_eq!(
            JapaneseCalendar.to_fixed(japanese("令和", 1, 5, 1)),
            Ok(day)
        );
        assert_eq!(
            JapaneseCalendar
                .from_fixed(day)
                .expect("in range")
                .to_string(),
            "令和元年5月1日"
        );
    }

    #[test]
    fn heisei_ended_on_the_thirtieth_of_april_2019() {
        let day = greg(2019, 4, 30);
        assert_eq!(
            JapaneseCalendar.from_fixed(day),
            Ok(japanese("平成", 31, 4, 30))
        );
        assert_eq!(
            JapaneseCalendar.to_fixed(japanese("平成", 31, 4, 30)),
            Ok(day)
        );
        // The two anchors are consecutive days.
        assert_eq!(greg(2019, 5, 1).0 - day.0, 1);
    }

    #[test]
    fn showa_sixty_four_and_heisei_one_are_consecutive_days() {
        let last_showa = greg(1989, 1, 7);
        let first_heisei = greg(1989, 1, 8);
        assert_eq!(
            JapaneseCalendar.from_fixed(last_showa),
            Ok(japanese("昭和", 64, 1, 7))
        );
        assert_eq!(
            JapaneseCalendar.from_fixed(first_heisei),
            Ok(japanese("平成", 1, 1, 8))
        );
        assert_eq!(first_heisei.0 - last_showa.0, 1);
        // Both era years name the same Western year.
        assert_eq!(japanese("昭和", 64, 1, 7).calendar_year(), 1989);
        assert_eq!(japanese("平成", 1, 1, 8).calendar_year(), 1989);
    }

    #[test]
    fn a_date_on_the_wrong_side_of_an_era_boundary_is_refused() {
        // 昭和64年 lasted seven days; its eighth is 平成元年1月8日.
        assert_eq!(
            JapaneseCalendar.to_fixed(japanese("昭和", 64, 1, 8)),
            Err(CalendarError::DayOutOfRange)
        );
        // And nothing before an era begins belongs to it either.
        assert_eq!(
            JapaneseCalendar.to_fixed(japanese("平成", 1, 1, 7)),
            Err(CalendarError::DayOutOfRange)
        );
        // 明治 ran to its 45th year only.
        assert_eq!(
            JapaneseCalendar.to_fixed(japanese("明治", 50, 1, 1)),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            JapaneseCalendar.to_fixed(japanese("令和", 0, 5, 1)),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn the_solar_calendar_began_the_day_after_meiji_five_twelfth_month_second() {
        let last_lunisolar = greg(1872, 12, 31);
        let first_solar = greg(1873, 1, 1);
        assert_eq!(first_solar, GREGORIAN_ADOPTION);
        assert_eq!(
            JapaneseCalendar.from_fixed(last_lunisolar),
            Ok(japanese("明治", 5, 12, 2))
        );
        assert_eq!(
            JapaneseCalendar.from_fixed(first_solar),
            Ok(japanese("明治", 6, 1, 1))
        );
        assert_eq!(first_solar.0 - last_lunisolar.0, 1);
        // The decreed-away day does not exist.
        assert!(
            JapaneseCalendar
                .to_fixed(japanese("明治", 5, 12, 3))
                .is_err()
        );
    }

    #[test]
    fn meiji_year_one_is_the_lunisolar_new_year_of_1868() {
        // The 一世一元の詔 renumbered 慶応4年 as 明治元年 from its first day.
        let day = greg(1868, 1, 25);
        assert_eq!(
            JapaneseCalendar.from_fixed(day),
            Ok(japanese("明治", 1, 1, 1))
        );
        // The day before belongs to 慶応, in its own third year.
        assert_eq!(
            JapaneseCalendar.from_fixed(Rd(day.0 - 1)),
            Ok(japanese("慶応", 3, 12, 30))
        );
    }

    #[test]
    fn the_actual_proclamation_day_of_meiji_is_the_ninth_month_eighth_day() {
        // 明治元年9月8日 = 1868-10-23 was when the era was proclaimed; the
        // era table backdates it, so the date still reads as 明治元年.
        assert_eq!(
            JapaneseCalendar.from_fixed(greg(1868, 10, 23)),
            Ok(japanese("明治", 1, 9, 8))
        );
    }

    #[test]
    fn taisho_and_showa_start_on_the_official_dates() {
        assert_eq!(
            JapaneseCalendar.from_fixed(greg(1912, 7, 30)),
            Ok(japanese("大正", 1, 7, 30))
        );
        assert_eq!(
            JapaneseCalendar.from_fixed(greg(1912, 7, 29)),
            Ok(japanese("明治", 45, 7, 29))
        );
        assert_eq!(
            JapaneseCalendar.from_fixed(greg(1926, 12, 25)),
            Ok(japanese("昭和", 1, 12, 25))
        );
        assert_eq!(
            JapaneseCalendar.from_fixed(greg(1926, 12, 24)),
            Ok(japanese("大正", 15, 12, 24))
        );
    }

    #[test]
    fn every_modern_era_boundary_holds_on_both_sides() {
        let boundaries = [
            ((1868, 1, 25), "慶応", "明治"),
            ((1912, 7, 30), "明治", "大正"),
            ((1926, 12, 25), "大正", "昭和"),
            ((1989, 1, 8), "昭和", "平成"),
            ((2019, 5, 1), "平成", "令和"),
        ];
        for ((year, month, day), before, after) in boundaries {
            let first = greg(year, month, day);
            let last = Rd(first.0 - 1);
            assert_eq!(
                JapaneseCalendar
                    .from_fixed(last)
                    .expect("in range")
                    .era
                    .kanji,
                before,
                "the day before {year}-{month}-{day}"
            );
            let started = JapaneseCalendar.from_fixed(first).expect("in range");
            assert_eq!(started.era.kanji, after, "{year}-{month}-{day}");
            assert_eq!(started.year, 1, "{after} should start in its year 1");
            assert_eq!(JapaneseCalendar.to_fixed(started), Ok(first));
        }
    }

    #[test]
    fn the_lunisolar_half_carries_intercalary_months() {
        // 明治3年 had a leap tenth month; the Tenpō calendar computes it.
        let leap = JapaneseDate::new(
            nengo::find("明治").expect("known era"),
            3,
            Month::leap(10),
            1,
        );
        let rd = JapaneseCalendar.to_fixed(leap).expect("the month exists");
        assert_eq!(JapaneseCalendar.from_fixed(rd), Ok(leap));
        assert!(
            JapaneseCalendar
                .from_fixed(rd)
                .expect("in range")
                .is_in_leap_month()
        );
        assert_eq!(leap.to_string(), "明治3年閏10月1日");
        // And the solar half does not.
        assert_eq!(
            JapaneseCalendar.to_fixed(JapaneseDate::new(
                nengo::find("令和").expect("known era"),
                8,
                Month::leap(9),
                1
            )),
            Err(CalendarError::MonthOutOfRange)
        );
    }

    #[test]
    fn the_calendar_refuses_days_before_the_tenpo_calendar() {
        assert_eq!(EARLIEST, greg(1844, 2, 18));
        assert_eq!(
            JapaneseCalendar.from_fixed(EARLIEST),
            Ok(japanese("天保", 15, 1, 1))
        );
        assert_eq!(
            JapaneseCalendar.from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        // 元禄15年12月14日, the night of the Akō vendetta, is exactly the
        // kind of date this module will not guess at.
        assert_eq!(
            JapaneseCalendar.to_fixed(japanese("元禄", 15, 12, 14)),
            Err(CalendarError::BeforeEpoch)
        );
        // But the era lookup still knows the era was in force.
        assert_eq!(
            nengo::era_at(greg(1703, 1, 30), Court::Unified).map(|era| era.kanji),
            Ok("元禄")
        );
    }

    #[test]
    fn the_calendar_refuses_days_past_its_upper_bound() {
        assert_eq!(LATEST, greg(9999, 12, 31));
        assert!(JapaneseCalendar.from_fixed(LATEST).is_ok());
        assert_eq!(
            JapaneseCalendar.from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }

    #[test]
    fn eras_can_be_named_in_kanji_reading_or_romaji() {
        let day = greg(2026, 9, 21);
        for name in ["reiwa", "令和", "れいわ", "Reiwa"] {
            let date =
                JapaneseDate::from_era_name(name, 8, Month::regular(9), 21).expect("known era");
            assert_eq!(JapaneseCalendar.to_fixed(date), Ok(day));
        }
        assert_eq!(
            JapaneseDate::from_era_name("nope", 1, Month::regular(1), 1),
            Err(CalendarError::UnknownEra)
        );
    }

    #[test]
    fn fields_round_trip_with_and_without_an_era() {
        let date = japanese("令和", 8, 9, 21);
        let fields = JapaneseCalendar.to_fields(date).expect("describable");
        assert_eq!(fields.era, Some("reiwa"));
        assert_eq!(fields.year, 8);
        assert_eq!(fields.month, Some(Month::regular(9)));
        assert_eq!(fields.day, Some(21));
        assert_eq!(JapaneseCalendar.from_fields(&fields), Ok(date));

        // Era-less fields are read as an extended year.
        let extended = DateFields::ymd(2026, 9, 21);
        assert_eq!(JapaneseCalendar.from_fields(&extended), Ok(date));
    }

    #[test]
    fn missing_fields_are_reported_by_name() {
        let mut fields = DateFields::ymd(8, 9, 21).with_era("reiwa");
        fields.month = None;
        assert_eq!(
            JapaneseCalendar.from_fields(&fields),
            Err(CalendarError::MissingField("month"))
        );
        let mut fields = DateFields::ymd(8, 9, 21).with_era("reiwa");
        fields.day = None;
        assert_eq!(
            JapaneseCalendar.from_fields(&fields),
            Err(CalendarError::MissingField("day"))
        );
        let fields = DateFields::ymd(8, 9, 21).with_era("no-such-era");
        assert_eq!(
            JapaneseCalendar.from_fields(&fields),
            Err(CalendarError::UnknownEra)
        );
    }

    #[test]
    fn the_metadata_describes_an_era_relative_astronomical_calendar() {
        let meta = JapaneseCalendar.meta();
        assert_eq!(meta.id, ID);
        assert_eq!(meta.year_kind, YearKind::EraRelative);
        assert!(meta.has_leap_months);
        assert!(meta.is_astronomical);
        assert_eq!(meta.earliest, Some(EARLIEST));
        assert_eq!(meta.latest, Some(LATEST));
    }

    #[test]
    fn the_solar_half_agrees_with_the_gregorian_calendar_day_for_day() {
        // From 1873 the only difference is the year number, so any
        // disagreement in month or day is a bug.
        for rd in (GREGORIAN_ADOPTION.0..GREGORIAN_ADOPTION.0 + 60_000).step_by(37) {
            let rd = Rd(rd);
            let (year, month, day) = gregorian::from_fixed(rd).expect("in range");
            let date = JapaneseCalendar.from_fixed(rd).expect("in range");
            assert_eq!(date.month, Month::regular(month), "{rd}");
            assert_eq!(date.day, day, "{rd}");
            assert_eq!(date.calendar_year(), year, "{rd}");
        }
    }

    #[test]
    fn the_lunisolar_half_agrees_with_the_tenpo_calendar_day_for_day() {
        for rd in EARLIEST.0..GREGORIAN_ADOPTION.0 {
            let rd = Rd(rd);
            let tenpo = japanese_tenpo::ENGINE.from_fixed(rd).expect("in range");
            let date = JapaneseCalendar.from_fixed(rd).expect("in range");
            assert_eq!(date.month, tenpo.month, "{rd}");
            assert_eq!(date.day, tenpo.day, "{rd}");
            assert_eq!(date.calendar_year(), tenpo.year, "{rd}");
        }
    }

    #[test]
    fn every_era_in_range_has_a_first_day_that_round_trips() {
        for era in nengo::stream(Court::Unified) {
            let Some(start) = era.start else { continue };
            if start < EARLIEST || start > LATEST {
                continue;
            }
            let date = JapaneseCalendar.from_fixed(start).expect("in range");
            assert_eq!(date.era.kanji, era.kanji, "{}", era.kanji);
            assert_eq!(date.year, 1, "{} should begin in its year 1", era.kanji);
            assert_eq!(JapaneseCalendar.to_fixed(date), Ok(start));
        }
    }

    #[test]
    fn the_eras_in_range_are_the_ones_from_tenpo_onward() {
        let covered: usize = nengo::stream(Court::Unified)
            .filter(|era| {
                era.start
                    .is_some_and(|start| start >= EARLIEST && start <= LATEST)
            })
            .count();
        // 弘化 嘉永 安政 万延 文久 元治 慶応 明治 大正 昭和 平成 令和.
        assert_eq!(covered, 12);
        // 天保 began before the supported range but covers part of it.
        let tenpo = nengo::by_kanji("天保").expect("in table");
        assert!(tenpo.start.expect("dated") < EARLIEST);
        assert_eq!(
            JapaneseCalendar.from_fixed(EARLIEST).expect("in range").era,
            tenpo
        );
    }

    #[test]
    fn well_known_bakumatsu_dates_convert_the_way_the_histories_give_them() {
        // Five events whose 和暦 and Western dates are both published in
        // every account of the period. They sit on either side of three era
        // changes and one of them lands in an intercalary month, so getting
        // all five right exercises the lunisolar half end to end.
        let cases = [
            // Perry's squadron reaches Uraga.
            ((1853, 7, 8), "嘉永6年6月3日"),
            // Ii Naosuke is assassinated outside the Sakurada Gate.
            ((1860, 3, 24), "安政7年3月3日"),
            // Tokugawa Yoshinobu returns power to the emperor.
            ((1867, 11, 9), "慶応3年10月14日"),
            // The 明治 era is proclaimed.
            ((1868, 10, 23), "明治元年9月8日"),
            // The abolition of the han system.
            ((1871, 8, 29), "明治4年7月14日"),
        ];
        for ((year, month, day), wareki) in cases {
            let rd = greg(year, month, day);
            let date = JapaneseCalendar.from_fixed(rd).expect("in range");
            assert_eq!(date.to_string(), wareki, "{year}-{month}-{day}");
            assert_eq!(JapaneseCalendar.to_fixed(date), Ok(rd));
        }
    }

    #[test]
    fn meiji_three_had_an_intercalary_tenth_month() {
        // 明治3年閏10月1日 = 1870-11-23. The Tenpō calendar computes the
        // leap month; no table of leap months is shipped with this crate.
        let rd = greg(1870, 11, 23);
        let date = JapaneseCalendar.from_fixed(rd).expect("in range");
        assert_eq!(date.to_string(), "明治3年閏10月1日");
        assert!(date.is_in_leap_month());
        assert_eq!(JapaneseCalendar.to_fixed(date), Ok(rd));
        // The ordinary tenth month came a lunation earlier.
        let ordinary = JapaneseDate::new(date.era, 3, Month::regular(10), 1);
        let earlier = JapaneseCalendar.to_fixed(ordinary).expect("it exists");
        assert!((29..=30).contains(&(rd.0 - earlier.0)));
    }

    #[test]
    fn the_new_year_of_1873_was_a_wednesday_in_both_calendars() {
        // A cheap independent check that the reform lines the two halves up
        // rather than shifting them: the weekday never broke.
        assert_eq!(Weekday::from_rd(GREGORIAN_ADOPTION), Weekday::Wednesday);
        assert_eq!(
            Weekday::from_rd(Rd(GREGORIAN_ADOPTION.0 - 1)),
            Weekday::Tuesday
        );
    }

    #[test]
    fn era_years_count_from_the_eras_first_calendar_year() {
        // 令和8年 is 2026; 平成31年 is 2019; 昭和64年 is 1989.
        assert_eq!(japanese("令和", 8, 1, 1).calendar_year(), 2026);
        assert_eq!(japanese("平成", 31, 1, 1).calendar_year(), 2019);
        assert_eq!(japanese("昭和", 64, 1, 1).calendar_year(), 1989);
        assert_eq!(japanese("大正", 15, 1, 1).calendar_year(), 1926);
        // And in the lunisolar half they count lunisolar years.
        assert_eq!(japanese("安政", 1, 11, 27).calendar_year(), 1854);
    }

    #[test]
    fn ansei_year_one_is_the_lunisolar_year_that_began_in_1854() {
        // 安政元年11月27日 = 1855-01-15: the era was proclaimed in the
        // Western year after the one its year 1 belongs to.
        assert_eq!(
            JapaneseCalendar.to_fixed(japanese("安政", 1, 11, 27)),
            Ok(greg(1855, 1, 15))
        );
        assert_eq!(
            JapaneseCalendar.from_fixed(greg(1855, 1, 15)),
            Ok(japanese("安政", 1, 11, 27))
        );
    }
}
