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
//! | 1798-02-16 to 1844-02-17 | Kansei lunisolar | [`hc_calendars_lunar::japanese_historical::kansei`] |
//! | 1755-02-11 to 1798-02-15 | Hōryaku lunisolar | [`hc_calendars_lunar::japanese_historical::horyaku`] |
//! | 1685-02-04 to 1755-02-10 | Jōkyō lunisolar | [`hc_calendars_lunar::japanese_historical::jokyo`] |
//! | 862-02-07 to 1685-02-03 | Senmyō lunisolar | [`hc_calendars_lunar::japanese_historical::senmyo`] |
//! | before 862-02-07 | *not supported* | — |
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
//! Conversion reaches back to **862-02-07**, 貞観4年1月1日, the first day of
//! the Senmyō calendar. Japan changed lunisolar system four times between
//! then and 1844, and each system differs from the next in its solar theory
//! and its intercalation, so a date only means what the system in force on
//! that day says it means. This module asks the calendar that was in force
//! — Senmyō, Jōkyō, Hōryaku, Kansei, then Tenpō — for every day, rather
//! than running the last of them backwards through the other four, which
//! would be wrong by a day here and a whole month there with no warning.
//!
//! Before 862 no calendar is implemented, and the module refuses rather than
//! guesses: [`hc_calendar::CalendarMeta::earliest`] says so, and a caller
//! asking for a day before it gets [`CalendarError::BeforeEpoch`]. That is
//! not where the *eras* stop — [`crate::nengo`] carries all 248 of them back
//! to 大化 in 645, and [`crate::nengo::era_at`] will name the era in force
//! on any day from 645 onward. It just will not give the month and day
//! before 862.
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
//! to about a minute and the apparent solar longitude to about 1″, and a
//! month boundary computed here can still differ by one day from what the
//! Japanese calendar bureau actually promulgated, because the bureau
//! computed from its own tables rather than from modern astronomy. See
//! [`hc_calendars_lunar::japanese_tenpo`] for the meridian and the model.
//! No table of the promulgated Tenpō months is shipped with this crate, so
//! this module cannot report a disagreement rate against one.

use core::fmt;

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};
use hc_calendars_lunar::{LunisolarCalendar, LunisolarDate, japanese_historical, japanese_tenpo};
use hc_calendars_solar::gregorian;

use crate::nengo::{self, Court, Nengo};

/// The CLDR identifier of the unified-stream calendar.
pub const ID: CalendarId = CalendarId("japanese");

/// The identifier of the Northern Court calendar.
pub const ID_NORTHERN: CalendarId = CalendarId("japanese-northern");

/// The identifier of the Southern Court calendar.
pub const ID_SOUTHERN: CalendarId = CalendarId("japanese-southern");

/// The identifier of the unified stream read as proclaimed.
pub const ID_PROCLAIMED: CalendarId = CalendarId("japanese-proclaimed");

/// The identifier of the Northern Court stream read as proclaimed.
pub const ID_NORTHERN_PROCLAIMED: CalendarId = CalendarId("japanese-northern-proclaimed");

/// The identifier of the Southern Court stream read as proclaimed.
pub const ID_SOUTHERN_PROCLAIMED: CalendarId = CalendarId("japanese-southern-proclaimed");

/// The first day the solar calendar was in force: 明治6年1月1日, decreed to
/// follow 明治5年12月2日 directly.
pub const GREGORIAN_ADOPTION: Rd = Rd(683_735);

/// The earliest day this calendar converts: 貞観4年1月1日 = 862-02-07, the
/// first day Senmyō-reki was in force in Japan.
///
/// Japan used five lunisolar systems in succession before the Gregorian
/// adoption, and this calendar reads whichever one was in force on the day
/// asked about. Before 862 the systems are 大衍暦 and earlier, which
/// `hc-calendars-lunar` does not implement because nothing available could
/// validate them, so this is where the range stops.
///
/// `nengo::era_at` still names the era for any day from 645; it is the
/// *calendar* underneath that runs out, not the era table.
pub const EARLIEST: Rd = japanese_historical::senmyo::EARLIEST;

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

/// When a backdated 改元 takes effect.
///
/// Almost every pre-Meiji era change was 年初改元: proclaimed part way
/// through a year and backdated to its first day. So a single lunisolar year
/// has two era names, and which one a source uses depends on what the source
/// is for.
///
/// Chronological tables backdate. 日本暦日原典 lists 嘉永7年 and 安政元年 as
/// the same year, and a converter that did not agree would disagree with
/// every reference table a historian owns.
///
/// Narrative histories do not. 桜田門外の変 happened on 安政7年3月3日, fifteen
/// days before 万延 was proclaimed — and partly caused the 改元. Writing it
/// 万延元年3月3日 is defensible and jarring, and no account of the incident
/// does it.
///
/// Both readings are right about different questions, so per policy §5 each
/// gets a calendar of its own rather than a flag on one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum EraReckoning {
    /// 年初改元: an era begins on the first day of its 元年.
    ///
    /// The reading of the chronological tables, and the one the era table's
    /// own `start_year` field describes.
    #[default]
    Backdated,
    /// 改元当時: an era begins on the day it was proclaimed.
    ///
    /// The reading a document's dateline needs, and the one narrative
    /// histories use. `nengo::era_at` answers under this convention too.
    Proclaimed,
}

/// The Japanese calendar, as proclaimed by one imperial court.
///
/// Between 1331 and 1392 two courts proclaimed eras at the same time, so for
/// those sixty-one years there is no such thing as "the" Japanese era of a
/// day — there are two, and they disagree. Rather than take a parameter that
/// a caller can forget to pass, this crate registers the two streams as two
/// calendars, exactly as it registers the Julian-to-Gregorian reform once per
/// country: a date proclaimed in Yoshino and the same day proclaimed in Kyoto
/// belong to different calendars, not to one calendar with a setting.
///
/// [`JapaneseCalendar::UNIFIED`] is the default and covers everything outside
/// the schism; inside it, it refuses rather than choosing a side.
/// [`JapaneseCalendar::NORTHERN`] and [`JapaneseCalendar::SOUTHERN`] each read
/// one stream and are defined across the whole range, because each court's
/// own reckoning is continuous.
///
/// After the reunification of 1392 the Northern stream is the one that
/// continues, because the union took the form of the Southern emperor
/// abdicating and 明徳 carrying on.
///
/// The second axis is [`EraReckoning`], for the same reason: a backdated
/// 改元 gives one lunisolar year two era names, and the two readings are
/// separate calendars rather than a setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JapaneseCalendar {
    court: Court,
    reckoning: EraReckoning,
}

impl Default for JapaneseCalendar {
    fn default() -> Self {
        Self::UNIFIED
    }
}

impl JapaneseCalendar {
    /// The single-court stream, which refuses the years of the schism.
    pub const UNIFIED: Self = Self {
        court: Court::Unified,
        reckoning: EraReckoning::Backdated,
    };

    /// The Northern Court (北朝) stream.
    pub const NORTHERN: Self = Self {
        court: Court::Northern,
        reckoning: EraReckoning::Backdated,
    };

    /// The Southern Court (南朝) stream.
    pub const SOUTHERN: Self = Self {
        court: Court::Southern,
        reckoning: EraReckoning::Backdated,
    };

    /// The single-court stream with eras beginning when they were
    /// proclaimed rather than at the start of their 元年.
    ///
    /// See [`EraReckoning`] for which question each answers.
    pub const PROCLAIMED: Self = Self {
        court: Court::Unified,
        reckoning: EraReckoning::Proclaimed,
    };

    /// Build a calendar for a given court, backdating era changes.
    #[must_use]
    pub const fn new(court: Court) -> Self {
        Self {
            court,
            reckoning: EraReckoning::Backdated,
        }
    }

    /// Build a calendar for a given court and era reckoning.
    #[must_use]
    pub const fn with_reckoning(court: Court, reckoning: EraReckoning) -> Self {
        Self { court, reckoning }
    }

    /// Which court's proclamations this calendar reads.
    #[must_use]
    pub const fn court(self) -> Court {
        self.court
    }

    /// When this calendar lets a backdated 改元 take effect.
    #[must_use]
    pub const fn reckoning(self) -> EraReckoning {
        self.reckoning
    }

    /// This calendar's identifier.
    #[must_use]
    pub const fn id(self) -> CalendarId {
        match (self.court, self.reckoning) {
            (Court::Unified, EraReckoning::Backdated) => ID,
            (Court::Northern, EraReckoning::Backdated) => ID_NORTHERN,
            (Court::Southern, EraReckoning::Backdated) => ID_SOUTHERN,
            (Court::Unified, EraReckoning::Proclaimed) => ID_PROCLAIMED,
            (Court::Northern, EraReckoning::Proclaimed) => ID_NORTHERN_PROCLAIMED,
            (Court::Southern, EraReckoning::Proclaimed) => ID_SOUTHERN_PROCLAIMED,
        }
    }

    /// This calendar's English name.
    #[must_use]
    pub const fn english_name(self) -> &'static str {
        match (self.court, self.reckoning) {
            (Court::Unified, EraReckoning::Backdated) => "Japanese (imperial eras)",
            (Court::Northern, EraReckoning::Backdated) => {
                "Japanese (imperial eras, Northern Court)"
            }
            (Court::Southern, EraReckoning::Backdated) => {
                "Japanese (imperial eras, Southern Court)"
            }
            (Court::Unified, EraReckoning::Proclaimed) => "Japanese (imperial eras, as proclaimed)",
            (Court::Northern, EraReckoning::Proclaimed) => {
                "Japanese (imperial eras, Northern Court, as proclaimed)"
            }
            (Court::Southern, EraReckoning::Proclaimed) => {
                "Japanese (imperial eras, Southern Court, as proclaimed)"
            }
        }
    }
}

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
        return Ok((year, Month::regular(month), day));
    }
    let date = engine_in_force(rd).from_fixed(rd)?;
    Ok((date.year, date.month, date.day))
}

/// The lunisolar system Japan was using on a given day.
///
/// Japan changed calendar five times between 862 and 1872, and a date only
/// means what the system in force at the time says it means. Reading every
/// pre-1873 date through Tenpō-reki — the last of the five — would be wrong
/// by up to two days for the 982 years before 1844, because Senmyō-reki's
/// solar theory had drifted that far by the time it was replaced.
fn engine_in_force(rd: Rd) -> &'static LunisolarCalendar {
    if rd >= japanese_tenpo::EARLIEST {
        &japanese_tenpo::ENGINE
    } else if rd >= japanese_historical::kansei::EARLIEST {
        &japanese_historical::kansei::ENGINE
    } else if rd >= japanese_historical::horyaku::EARLIEST {
        &japanese_historical::horyaku::ENGINE
    } else if rd >= japanese_historical::jokyo::EARLIEST {
        &japanese_historical::jokyo::ENGINE
    } else {
        &japanese_historical::senmyo::ENGINE
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
        // The system is chosen by the year, since the day is what is being
        // computed. Each engine refuses a year outside its own period, so a
        // date in a changeover year is resolved by trying the candidates in
        // order rather than by a boundary this function would have to
        // duplicate.
        let date = LunisolarDate::new(year, month, day);
        // Ordered by which system was in force in that year, so the common
        // case is one attempt rather than five. A lunisolar year can straddle
        // a changeover, so the rest stay as fallbacks rather than being
        // ruled out.
        let ordered: [&LunisolarCalendar; 5] = if year >= 1844 {
            [
                &japanese_tenpo::ENGINE,
                &japanese_historical::kansei::ENGINE,
                &japanese_historical::horyaku::ENGINE,
                &japanese_historical::jokyo::ENGINE,
                &japanese_historical::senmyo::ENGINE,
            ]
        } else if year >= 1798 {
            [
                &japanese_historical::kansei::ENGINE,
                &japanese_tenpo::ENGINE,
                &japanese_historical::horyaku::ENGINE,
                &japanese_historical::jokyo::ENGINE,
                &japanese_historical::senmyo::ENGINE,
            ]
        } else if year >= 1755 {
            [
                &japanese_historical::horyaku::ENGINE,
                &japanese_historical::kansei::ENGINE,
                &japanese_historical::jokyo::ENGINE,
                &japanese_tenpo::ENGINE,
                &japanese_historical::senmyo::ENGINE,
            ]
        } else if year >= 1685 {
            [
                &japanese_historical::jokyo::ENGINE,
                &japanese_historical::horyaku::ENGINE,
                &japanese_historical::senmyo::ENGINE,
                &japanese_historical::kansei::ENGINE,
                &japanese_tenpo::ENGINE,
            ]
        } else {
            [
                &japanese_historical::senmyo::ENGINE,
                &japanese_historical::jokyo::ENGINE,
                &japanese_historical::horyaku::ENGINE,
                &japanese_historical::kansei::ENGINE,
                &japanese_tenpo::ENGINE,
            ]
        };
        for engine in ordered {
            if let Ok(rd) = engine.to_fixed(date) {
                // Guard against a neighbouring system accepting a year that
                // was not its own: the answer has to read back the same way.
                if calendar_year_month_day(rd) == Ok((year, month, day)) {
                    return Ok(rd);
                }
            }
        }
        // No system in force accepted the year. Distinguish "earlier than any
        // calendar this crate implements" from "a year none of them has",
        // because the first is a range limit and the second is a bad date.
        if year < 862 {
            Err(CalendarError::BeforeEpoch)
        } else {
            Err(CalendarError::YearOutOfRange)
        }
    }
}

/// The half-open span of fixed days an era covers, within this calendar's
/// supported range.
///
/// # Errors
///
/// Returns [`CalendarError::UnknownEra`] for an era whose start day the
/// sources do not fix.
fn era_span(era: &Nengo, court: Court, reckoning: EraReckoning) -> CalendarResult<(Rd, Rd)> {
    let start = era_start(era, reckoning)?;
    let end = match nengo::next_in_stream(era, court).map(|next| era_start(next, reckoning)) {
        Some(Ok(next)) => next,
        Some(Err(error)) => return Err(error),
        None => Rd(LATEST.0 + 1),
    };
    Ok((start, end))
}

/// The first Japanese calendar year whose era change is dated the 公式 way.
///
/// 明治 is the boundary in both directions and can be read either way: it
/// was proclaimed 明治元年9月8日 but backdated to 明治元年1月1日, so its
/// proclamation table entry already *is* the first day of its year. 大正 is
/// the first era for which the two readings differ and the 公式 one wins —
/// 明治45年 runs to 1912-07-29 and 大正元年 begins on the 30th.
const FIRST_PROCLAIMED_YEAR: i64 = 1868;

/// The first day an era covers, under the convention its own period used.
///
/// # Why this is not just `Nengo::start`
///
/// [`Nengo`] carries two facts that disagree for every pre-Meiji era.
/// `start` is the day the era was proclaimed; `start_year` is the calendar
/// year the era's 元年 *occupies*, which is earlier, because almost every
/// pre-Meiji 改元 was 年初改元 — announced part way through a year and
/// backdated to its first day.
///
/// Reading the era from one and the year number from the other splits a
/// single lunisolar year between two era names. 安政 was proclaimed on
/// 嘉永7年11月27日; with `start` as the boundary, the first ten months of
/// that year came back as 嘉永7年 and the last two as 安政元年12月, so
/// 安政元年1月1日 — a date every chronological table lists — was
/// `DayOutOfRange`, and 嘉永7年12月1日 was too. It also meant 219 of the 220
/// convertible eras had a truncated 元年.
///
/// So a pre-Meiji era begins at the first day of `start_year`, and 明治
/// onward begin at the proclamation table's day. `nengo::era_at` still
/// reads `start` and still answers the other question — which era had been
/// *declared* by a given day — which is the one a document's dateline
/// needs.
///
/// # Errors
///
/// Propagates whatever the calendar says about `start_year`. An era whose
/// year predates the earliest lunisolar system falls back to its
/// proclamation day, which is the best this table can do for it.
fn era_start(era: &Nengo, reckoning: EraReckoning) -> CalendarResult<Rd> {
    if reckoning == EraReckoning::Proclaimed || era.start_year >= FIRST_PROCLAIMED_YEAR {
        return era.require_start();
    }
    match calendar_to_fixed(era.start_year, Month::regular(1), 1) {
        Ok(day) => Ok(day),
        Err(_) => era.require_start(),
    }
}

/// The era in force on a day, under [`era_start`]'s convention.
///
/// The counterpart of [`nengo::era_at`], which answers the same question
/// under the 改元当時 reading. They differ on the days between a year's
/// first and a 改元 that was backdated to it — 30 383 of the 331 928 days
/// between 900 and 1870.
///
/// `calendar_year` is passed in rather than derived because the caller has
/// just computed it. That is not only tidiness: a pre-Meiji era's boundary
/// is a *year* comparison, so with the year in hand this is a scan of
/// integers rather than 248 lunisolar new-moon searches. Computing each
/// era's start day instead made a day lookup take longer than the whole
/// test suite had to spare.
///
/// # Errors
///
/// [`CalendarError::UnknownEra`] inside the 南北朝 for an unspecified court,
/// and [`CalendarError::BeforeEpoch`] before the first dated era.
fn era_in_force(
    rd: Rd,
    calendar_year: i64,
    court: Court,
    reckoning: EraReckoning,
) -> CalendarResult<&'static Nengo> {
    let court = match court {
        Court::Unified if nengo::is_nanbokucho(rd) => return Err(CalendarError::UnknownEra),
        Court::Unified if rd >= nengo::NANBOKUCHO_END => Court::Northern,
        other => other,
    };
    let mut found: Option<&'static Nengo> = None;
    for era in nengo::stream(court) {
        if reckoning == EraReckoning::Proclaimed || era.start_year >= FIRST_PROCLAIMED_YEAR {
            match era.start {
                Some(start) if start <= rd => found = Some(era),
                Some(_) => break,
                // An era with no start day cannot bound anything, so the
                // era before it stays in force until the next dated one.
                None => {}
            }
        } else if era.start_year <= calendar_year {
            found = Some(era);
        } else {
            break;
        }
    }
    found.ok_or(CalendarError::BeforeEpoch)
}

impl Calendar for JapaneseCalendar {
    type Date = JapaneseDate;

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::LUNISOLAR_TWELVE
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: self.id(),
            english_name: self.english_name(),
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
        let (start, end) = era_span(date.era, self.court, self.reckoning)?;
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
        let era = era_in_force(rd, year, self.court, self.reckoning)?;
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
            leap_day: false,
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
mod court_tests {
    use super::*;

    #[test]
    fn the_three_streams_have_distinct_identifiers() {
        assert_eq!(JapaneseCalendar::UNIFIED.id(), ID);
        assert_eq!(JapaneseCalendar::NORTHERN.id(), ID_NORTHERN);
        assert_eq!(JapaneseCalendar::SOUTHERN.id(), ID_SOUTHERN);
        assert_eq!(JapaneseCalendar::default(), JapaneseCalendar::UNIFIED);
    }

    #[test]
    fn each_court_reports_its_own_identifier_in_its_metadata() {
        assert_eq!(JapaneseCalendar::NORTHERN.meta().id, ID_NORTHERN);
        assert_eq!(JapaneseCalendar::SOUTHERN.meta().id, ID_SOUTHERN);
        assert_ne!(
            JapaneseCalendar::NORTHERN.meta().english_name,
            JapaneseCalendar::SOUTHERN.meta().english_name
        );
    }

    #[test]
    fn the_courts_read_different_streams_of_eras() {
        // The two streams are genuinely different data, which is the reason
        // they are two calendars rather than one with a setting.
        let northern: alloc::vec::Vec<&str> = nengo::stream(Court::Northern)
            .map(|era| era.kanji)
            .collect();
        let southern: alloc::vec::Vec<&str> = nengo::stream(Court::Southern)
            .map(|era| era.kanji)
            .collect();
        assert_ne!(northern, southern);

        // Inside the schism each court proclaimed eras the other did not.
        let only_northern = northern.iter().any(|era| !southern.contains(era));
        let only_southern = southern.iter().any(|era| !northern.contains(era));
        assert!(only_northern, "the Northern Court had eras of its own");
        assert!(only_southern, "the Southern Court had eras of its own");
    }

    #[test]
    fn the_unified_stream_refuses_the_years_of_the_schism() {
        // 1331-09-11 through 1392-11-19 had two courts, so there is no single
        // answer and the unified calendar says so rather than choosing.
        let inside = Rd(nengo::NANBOKUCHO_START.0 + 1_000);
        assert!(nengo::is_nanbokucho(inside));
        assert_eq!(
            nengo::era_at(inside, Court::Unified),
            Err(CalendarError::UnknownEra)
        );
        // Each court's own reckoning is continuous, so each can answer.
        assert!(nengo::era_at(inside, Court::Northern).is_ok());
        assert!(nengo::era_at(inside, Court::Southern).is_ok());
    }

    #[test]
    fn both_courts_kept_using_kenmu_until_the_southern_court_replaced_it() {
        // The schism opened in 1331 but the era did not fork immediately:
        // 建武 was common to both until 1336. A test that assumes the courts
        // differ everywhere inside the window picks this up as a failure,
        // which is why it is recorded rather than worked around.
        let early = Rd(nengo::NANBOKUCHO_START.0 + 1_000);
        assert!(nengo::is_nanbokucho(early));
        assert_eq!(
            nengo::era_at(early, Court::Northern).unwrap().kanji,
            nengo::era_at(early, Court::Southern).unwrap().kanji
        );
    }

    #[test]
    fn the_courts_disagree_after_the_fork_and_agree_after_the_union() {
        // Far enough in that the two streams have genuinely parted.
        let forked = Rd(nengo::NANBOKUCHO_START.0 + 3_000);
        assert!(nengo::is_nanbokucho(forked));
        let north = nengo::era_at(forked, Court::Northern).unwrap();
        let south = nengo::era_at(forked, Court::Southern).unwrap();
        assert_ne!(north.kanji, south.kanji, "the schism is a disagreement");

        // Well after the reunification the streams have converged again.
        let modern = Rd(719_163);
        assert_eq!(
            nengo::era_at(modern, Court::Northern).unwrap().kanji,
            nengo::era_at(modern, Court::Southern).unwrap().kanji
        );
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
            JapaneseCalendar::UNIFIED.from_fixed(day),
            Ok(japanese("令和", 1, 5, 1))
        );
        assert_eq!(
            JapaneseCalendar::UNIFIED.to_fixed(japanese("令和", 1, 5, 1)),
            Ok(day)
        );
        assert_eq!(
            JapaneseCalendar::UNIFIED
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
            JapaneseCalendar::UNIFIED.from_fixed(day),
            Ok(japanese("平成", 31, 4, 30))
        );
        assert_eq!(
            JapaneseCalendar::UNIFIED.to_fixed(japanese("平成", 31, 4, 30)),
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
            JapaneseCalendar::UNIFIED.from_fixed(last_showa),
            Ok(japanese("昭和", 64, 1, 7))
        );
        assert_eq!(
            JapaneseCalendar::UNIFIED.from_fixed(first_heisei),
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
            JapaneseCalendar::UNIFIED.to_fixed(japanese("昭和", 64, 1, 8)),
            Err(CalendarError::DayOutOfRange)
        );
        // And nothing before an era begins belongs to it either.
        assert_eq!(
            JapaneseCalendar::UNIFIED.to_fixed(japanese("平成", 1, 1, 7)),
            Err(CalendarError::DayOutOfRange)
        );
        // 明治 ran to its 45th year only.
        assert_eq!(
            JapaneseCalendar::UNIFIED.to_fixed(japanese("明治", 50, 1, 1)),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            JapaneseCalendar::UNIFIED.to_fixed(japanese("令和", 0, 5, 1)),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn the_solar_calendar_began_the_day_after_meiji_five_twelfth_month_second() {
        let last_lunisolar = greg(1872, 12, 31);
        let first_solar = greg(1873, 1, 1);
        assert_eq!(first_solar, GREGORIAN_ADOPTION);
        assert_eq!(
            JapaneseCalendar::UNIFIED.from_fixed(last_lunisolar),
            Ok(japanese("明治", 5, 12, 2))
        );
        assert_eq!(
            JapaneseCalendar::UNIFIED.from_fixed(first_solar),
            Ok(japanese("明治", 6, 1, 1))
        );
        assert_eq!(first_solar.0 - last_lunisolar.0, 1);
        // The decreed-away day does not exist.
        assert!(
            JapaneseCalendar::UNIFIED
                .to_fixed(japanese("明治", 5, 12, 3))
                .is_err()
        );
    }

    #[test]
    fn meiji_year_one_is_the_lunisolar_new_year_of_1868() {
        // The 一世一元の詔 renumbered 慶応4年 as 明治元年 from its first day.
        let day = greg(1868, 1, 25);
        assert_eq!(
            JapaneseCalendar::UNIFIED.from_fixed(day),
            Ok(japanese("明治", 1, 1, 1))
        );
        // The day before belongs to 慶応, in its own third year.
        assert_eq!(
            JapaneseCalendar::UNIFIED.from_fixed(Rd(day.0 - 1)),
            Ok(japanese("慶応", 3, 12, 30))
        );
    }

    #[test]
    fn the_actual_proclamation_day_of_meiji_is_the_ninth_month_eighth_day() {
        // 明治元年9月8日 = 1868-10-23 was when the era was proclaimed; the
        // era table backdates it, so the date still reads as 明治元年.
        assert_eq!(
            JapaneseCalendar::UNIFIED.from_fixed(greg(1868, 10, 23)),
            Ok(japanese("明治", 1, 9, 8))
        );
    }

    #[test]
    fn taisho_and_showa_start_on_the_official_dates() {
        assert_eq!(
            JapaneseCalendar::UNIFIED.from_fixed(greg(1912, 7, 30)),
            Ok(japanese("大正", 1, 7, 30))
        );
        assert_eq!(
            JapaneseCalendar::UNIFIED.from_fixed(greg(1912, 7, 29)),
            Ok(japanese("明治", 45, 7, 29))
        );
        assert_eq!(
            JapaneseCalendar::UNIFIED.from_fixed(greg(1926, 12, 25)),
            Ok(japanese("昭和", 1, 12, 25))
        );
        assert_eq!(
            JapaneseCalendar::UNIFIED.from_fixed(greg(1926, 12, 24)),
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
                JapaneseCalendar::UNIFIED
                    .from_fixed(last)
                    .expect("in range")
                    .era
                    .kanji,
                before,
                "the day before {year}-{month}-{day}"
            );
            let started = JapaneseCalendar::UNIFIED
                .from_fixed(first)
                .expect("in range");
            assert_eq!(started.era.kanji, after, "{year}-{month}-{day}");
            assert_eq!(started.year, 1, "{after} should start in its year 1");
            assert_eq!(JapaneseCalendar::UNIFIED.to_fixed(started), Ok(first));
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
        let rd = JapaneseCalendar::UNIFIED
            .to_fixed(leap)
            .expect("the month exists");
        assert_eq!(JapaneseCalendar::UNIFIED.from_fixed(rd), Ok(leap));
        assert!(
            JapaneseCalendar::UNIFIED
                .from_fixed(rd)
                .expect("in range")
                .is_in_leap_month()
        );
        assert_eq!(leap.to_string(), "明治3年閏10月1日");
        // And the solar half does not.
        assert_eq!(
            JapaneseCalendar::UNIFIED.to_fixed(JapaneseDate::new(
                nengo::find("令和").expect("known era"),
                8,
                Month::leap(9),
                1
            )),
            Err(CalendarError::MonthOutOfRange)
        );
    }

    #[test]
    fn the_calendar_refuses_days_before_senmyo_reki() {
        assert_eq!(EARLIEST, greg(862, 2, 7));
        assert_eq!(
            JapaneseCalendar::UNIFIED
                .from_fixed(EARLIEST)
                .map(|d| d.era.kanji),
            Ok("貞観")
        );
        assert_eq!(
            JapaneseCalendar::UNIFIED.from_fixed(Rd(EARLIEST.0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        // 大化元年, the first era of all, predates Senmyō-reki by two
        // centuries and is exactly the kind of date this module will not
        // guess at. 元禄15年12月14日 — the night of the Akō vendetta — used
        // to be refused too, and is now answered by Jōkyō-reki.
        assert_eq!(
            JapaneseCalendar::UNIFIED.to_fixed(japanese("大化", 1, 1, 1)),
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
        assert!(JapaneseCalendar::UNIFIED.from_fixed(LATEST).is_ok());
        assert_eq!(
            JapaneseCalendar::UNIFIED.from_fixed(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
    }

    #[test]
    fn eras_can_be_named_in_kanji_reading_or_romaji() {
        let day = greg(2026, 9, 21);
        for name in ["reiwa", "令和", "れいわ", "Reiwa"] {
            let date =
                JapaneseDate::from_era_name(name, 8, Month::regular(9), 21).expect("known era");
            assert_eq!(JapaneseCalendar::UNIFIED.to_fixed(date), Ok(day));
        }
        assert_eq!(
            JapaneseDate::from_era_name("nope", 1, Month::regular(1), 1),
            Err(CalendarError::UnknownEra)
        );
    }

    #[test]
    fn fields_round_trip_with_and_without_an_era() {
        let date = japanese("令和", 8, 9, 21);
        let fields = JapaneseCalendar::UNIFIED
            .to_fields(date)
            .expect("describable");
        assert_eq!(fields.era, Some("reiwa"));
        assert_eq!(fields.year, 8);
        assert_eq!(fields.month, Some(Month::regular(9)));
        assert_eq!(fields.day, Some(21));
        assert_eq!(JapaneseCalendar::UNIFIED.from_fields(&fields), Ok(date));

        // Era-less fields are read as an extended year.
        let extended = DateFields::ymd(2026, 9, 21);
        assert_eq!(JapaneseCalendar::UNIFIED.from_fields(&extended), Ok(date));
    }

    #[test]
    fn missing_fields_are_reported_by_name() {
        let mut fields = DateFields::ymd(8, 9, 21).with_era("reiwa");
        fields.month = None;
        assert_eq!(
            JapaneseCalendar::UNIFIED.from_fields(&fields),
            Err(CalendarError::MissingField("month"))
        );
        let mut fields = DateFields::ymd(8, 9, 21).with_era("reiwa");
        fields.day = None;
        assert_eq!(
            JapaneseCalendar::UNIFIED.from_fields(&fields),
            Err(CalendarError::MissingField("day"))
        );
        let fields = DateFields::ymd(8, 9, 21).with_era("no-such-era");
        assert_eq!(
            JapaneseCalendar::UNIFIED.from_fields(&fields),
            Err(CalendarError::UnknownEra)
        );
    }

    #[test]
    fn the_metadata_describes_an_era_relative_astronomical_calendar() {
        let meta = JapaneseCalendar::UNIFIED.meta();
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
            let date = JapaneseCalendar::UNIFIED.from_fixed(rd).expect("in range");
            assert_eq!(date.month, Month::regular(month), "{rd}");
            assert_eq!(date.day, day, "{rd}");
            assert_eq!(date.calendar_year(), year, "{rd}");
        }
    }

    #[test]
    fn the_lunisolar_half_agrees_with_the_tenpo_calendar_day_for_day() {
        // Only over Tenpō's own period. Before 1844 the calendar in force was
        // one of the four earlier systems, and reading those years through
        // Tenpō-reki is exactly the error this wiring exists to avoid.
        for rd in japanese_tenpo::EARLIEST.0..GREGORIAN_ADOPTION.0 {
            let rd = Rd(rd);
            let tenpo = japanese_tenpo::ENGINE.from_fixed(rd).expect("in range");
            let date = JapaneseCalendar::UNIFIED.from_fixed(rd).expect("in range");
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
            // The unified stream declines the years when two courts were
            // proclaiming at once; those eras belong to the Northern and
            // Southern calendars and are covered by their own tests.
            if nengo::is_nanbokucho(start) {
                continue;
            }
            let date = JapaneseCalendar::UNIFIED
                .from_fixed(start)
                .expect("in range");
            assert_eq!(date.era.kanji, era.kanji, "{}", era.kanji);
            assert_eq!(date.year, 1, "{} should begin in its year 1", era.kanji);
            assert_eq!(JapaneseCalendar::UNIFIED.to_fixed(date), Ok(start));
        }
    }

    #[test]
    fn the_eras_in_range_run_from_the_ninth_century() {
        let covered: usize = nengo::stream(Court::Unified)
            .filter(|era| {
                era.start
                    .is_some_and(|start| start >= EARLIEST && start <= LATEST)
            })
            .count();
        // Every dated era from 貞観 onwards, now that the four pre-Tenpō
        // systems are wired in; it was twelve when the range began in 1844.
        assert_eq!(covered, 195);
        // 貞観 began before the supported range but covers part of it.
        let jogan = nengo::by_kanji("貞観").expect("in table");
        assert!(jogan.start.expect("dated") < EARLIEST);
        assert_eq!(
            JapaneseCalendar::UNIFIED
                .from_fixed(EARLIEST)
                .expect("in range")
                .era,
            jogan
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
            // Ii Naosuke is assassinated outside the Sakurada Gate. The
            // backdated reading is 万延元年3月3日; see the test below.
            ((1860, 3, 24), "万延元年3月3日"),
            // Tokugawa Yoshinobu returns power to the emperor.
            ((1867, 11, 9), "慶応3年10月14日"),
            // The 明治 era is proclaimed.
            ((1868, 10, 23), "明治元年9月8日"),
            // The abolition of the han system.
            ((1871, 8, 29), "明治4年7月14日"),
        ];
        for ((year, month, day), wareki) in cases {
            let rd = greg(year, month, day);
            let date = JapaneseCalendar::UNIFIED.from_fixed(rd).expect("in range");
            assert_eq!(date.to_string(), wareki, "{year}-{month}-{day}");
            assert_eq!(JapaneseCalendar::UNIFIED.to_fixed(date), Ok(rd));
        }
    }

    /// 桜田門外の変 is the clearest case of the two readings disagreeing, and
    /// the reason [`EraReckoning`] exists.
    ///
    /// Ii Naosuke was assassinated on the third day of the third month of
    /// 1860. 万延 was proclaimed fifteen days later, on the eighteenth, and
    /// backdated to the first day of that year — partly *because* of the
    /// assassination. So the same day is 安政7年3月3日 in every narrative
    /// account and 万延元年3月3日 in every chronological table, and the
    /// library refuses to pick one on the reader's behalf.
    #[test]
    fn the_sakuradamon_incident_has_two_correct_dates() {
        let rd = greg(1860, 3, 24);

        let backdated = JapaneseCalendar::UNIFIED.from_fixed(rd).expect("in range");
        assert_eq!(backdated.to_string(), "万延元年3月3日");
        assert_eq!(JapaneseCalendar::UNIFIED.to_fixed(backdated), Ok(rd));

        let proclaimed = JapaneseCalendar::PROCLAIMED
            .from_fixed(rd)
            .expect("in range");
        assert_eq!(proclaimed.to_string(), "安政7年3月3日");
        assert_eq!(JapaneseCalendar::PROCLAIMED.to_fixed(proclaimed), Ok(rd));

        // Both name the same lunisolar day; only the era differs.
        assert_eq!(backdated.month, proclaimed.month);
        assert_eq!(backdated.day, proclaimed.day);
        assert_eq!(backdated.calendar_year(), proclaimed.calendar_year());

        // And 万延 was proclaimed fifteen days later, which is when the
        // proclaimed reading changes over and the backdated one does not.
        let proclamation = greg(1860, 4, 8);
        assert_eq!(
            JapaneseCalendar::PROCLAIMED
                .from_fixed(proclamation)
                .expect("in range")
                .era
                .kanji,
            "万延"
        );
    }

    /// The whole of a lunisolar year carries one era name under backdating.
    ///
    /// 安政 was proclaimed on 嘉永7年11月27日. Reading the era from the
    /// proclamation and the year number from the backdated table split that
    /// year in two, so 安政元年1月1日 — a date every chronological table
    /// lists — came back as `DayOutOfRange`.
    #[test]
    fn a_backdated_era_covers_the_whole_of_its_first_year() {
        let ansei = nengo::by_kanji("安政").expect("in table");
        assert_eq!(ansei.start_year, 1854);

        // The first day of 安政元年, and the last day of it.
        let first =
            JapaneseDate::from_era_name("安政", 1, Month::regular(1), 1).expect("known era");
        assert_eq!(
            JapaneseCalendar::UNIFIED.to_fixed(first).map(|rd| rd.0 > 0),
            Ok(true)
        );
        let twelfth =
            JapaneseDate::from_era_name("安政", 1, Month::regular(12), 1).expect("known era");
        assert!(JapaneseCalendar::UNIFIED.to_fixed(twelfth).is_ok());

        // Under the proclaimed reading the same date is not in the era yet,
        // and the day belongs to 嘉永7年 instead.
        assert_eq!(
            JapaneseCalendar::PROCLAIMED.to_fixed(first),
            Err(CalendarError::DayOutOfRange)
        );
        let rd = JapaneseCalendar::UNIFIED.to_fixed(first).expect("in range");
        assert_eq!(
            JapaneseCalendar::PROCLAIMED
                .from_fixed(rd)
                .expect("in range")
                .to_string(),
            "嘉永7年1月1日"
        );
    }

    #[test]
    fn meiji_three_had_an_intercalary_tenth_month() {
        // 明治3年閏10月1日 = 1870-11-23. The Tenpō calendar computes the
        // leap month; no table of leap months is shipped with this crate.
        let rd = greg(1870, 11, 23);
        let date = JapaneseCalendar::UNIFIED.from_fixed(rd).expect("in range");
        assert_eq!(date.to_string(), "明治3年閏10月1日");
        assert!(date.is_in_leap_month());
        assert_eq!(JapaneseCalendar::UNIFIED.to_fixed(date), Ok(rd));
        // The ordinary tenth month came a lunation earlier.
        let ordinary = JapaneseDate::new(date.era, 3, Month::regular(10), 1);
        let earlier = JapaneseCalendar::UNIFIED
            .to_fixed(ordinary)
            .expect("it exists");
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
            JapaneseCalendar::UNIFIED.to_fixed(japanese("安政", 1, 11, 27)),
            Ok(greg(1855, 1, 15))
        );
        assert_eq!(
            JapaneseCalendar::UNIFIED.from_fixed(greg(1855, 1, 15)),
            Ok(japanese("安政", 1, 11, 27))
        );
    }
}
