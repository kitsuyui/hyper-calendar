//! The Old Icelandic calendar, the *misseristal*.
//!
//! Iceland's civil calendar from the tenth century to the eighteenth, and
//! still printed in the Icelandic Almanac: a year of 52 weeks, 364 days, in
//! two *misseri*, summer and winter, kept in step with the sun by a leap
//! week, *sumarauki*, rather than a leap day, so that every year and every
//! month begins on the same weekday. Summer begins on a Thursday, the First
//! Day of Summer (*sumardagurinn fyrsti*), and has three months of thirty
//! days, four extra days (*aukanætur*) — eleven with the *sumarauki* — and
//! three months more; winter begins on a Saturday and has six months of
//! thirty. The year is 12 × 30 + 4 = 364 days, or 371.
//!
//! What fixes the leap week is the First Day of Summer, which is the
//! Thursday in a seven-day window of the Julian or the Gregorian calendar;
//! a year has *sumarauki* exactly when the next First Day of Summer is 371
//! days away. The window moved with the calendar reform, and not by the
//! reform's own eleven days, so there are two calendars and two
//! identifiers (policy §5):
//!
//! * `icelandic-julian`, the rule from the twelfth century to 1700: the
//!   first Thursday on or after 9 April **Julian**;
//! * `icelandic`, the rule since 1700: the first Thursday on or after
//!   19 April **Gregorian**, which is the rule of the Almanac today.
//!
//! They agree from 1496 to the summer of 1702 and part at Midsummer 1702,
//! when the Julian rule would have added a *sumarauki* and the Gregorian
//! one did not.
//!
//! Three reckonings Janson describes beside the standard one are
//! calendars of their own names too:
//!
//! * `icelandic-almanac`, the printed Icelandic Almanac's year from 1837
//!   until 1928, whose leap week "was inserted last in the summer", after
//!   Haustmánuður, so that in a leap year the last three summer months
//!   begin a week earlier (§7.1) — [`IcelandicAlmanacCalendar`];
//! * `icelandic-friday` and `icelandic-julian-friday`, the two rules with
//!   winter "reckoned from a Friday", the popular reckoning "from the 16th
//!   century until the Icelandic Almanac began to be published in 1837"
//!   (§2.1): the dates are the same, and the season and week the fields
//!   carry turn a day earlier;
//! * `icelandic-medieval`, the Julian rule with the medieval day, "reckoned
//!   from sunrise during summer and from dawn during winter" (§2.4),
//!   [`hc_calendar::DayBoundary::Daybreak`].
//!
//! # What is carried
//!
//! Dates are the year, one of thirteen positions and a day: the twelve
//! months in their modern names with the extra days as a position of their
//! own, **Aukanætur**, fourth, between Sólmánuður and Heyannir, as Janson
//! describes the year, its days 5 to 11 the *sumarauki* in a leap year.
//! The year is numbered by the Julian or Gregorian year in which its summer
//! begins, since "there is no special numbering of the Icelandic years",
//! and begins with the summer, which Janson chooses "somewhat arbitrarily"
//! for want of evidence either way. `to_fields` adds the reckoning the
//! calendar was actually dated by: the `season`, 1 for summer and 2 for
//! winter, and the `week` of the season, counted from the Thursday or the
//! Saturday, the two days after summer's twenty-sixth or twenty-seventh
//! week (*veturnætur*) being week 0. Janson warns that "dating by giving
//! the Icelandic month and day ... has never been used in Iceland".
//!
//! Not carried: the sixteenth-century rule that winter began on a
//! Saturday "except at rímspillir when it begins on a Friday", which Janson
//! reports from one document; the Almanac of 1888, which "forgot to insert
//! the leap week"; and the confusion of 1702–1703 over the new rule. The
//! system document is `docs/systems/icelandic.md`.
//!
//! # Sources
//!
//! * Svante Janson, "The Icelandic calendar", *Scripta Islandica* 62
//!   (2011), pp. 51–104, read in the Uppsala DiVA copy: Table 1 for the
//!   months, their weekdays and their Julian and Gregorian windows; §3.2
//!   for the Julian rule, "the First Day of Summer always fell in the week
//!   9–15 April"; §3.3–3.4 for the Gregorian rule, "the Thursday in the
//!   period 19–25 April", the change of 16/28 November 1700 and the first
//!   divergence at Midsummer 1702; §4 for the First Day of Winter "always
//!   180 days before the next First Day of Summer"; §5–6 for the formulas
//!   and the tables of leap years and *rímspillir* years the tests use;
//!   §2.1 for the Friday reckoning of winter, §2.4 for the day from sunrise
//!   or dawn, and §7.1 for the Almanac's leap week, with its months'
//!   Gregorian windows "22–28 July, 21–27 August and 20–26 September".
//! * Edward M. Reingold and Nachum Dershowitz, *Calendrical Calculations:
//!   The Ultimate Edition* (Cambridge, 2018), ch. 6 — the book not read
//!   here, its Apache-licensed `calendar-code2` (`calendar.l`, section
//!   "Icelandic Calendar") read after this module was written, to compare:
//!   the same Gregorian rule and winter 180 days before the next summer;
//!   dates there are a season, a week and a weekday, the two days after
//!   summer's last full week counted as its week 27 or 28 rather than
//!   Janson's week 0, and the extra days as "month" 0. No code was taken.
//!
//! # Exactness
//!
//! Exact — arithmetic on the Julian or Gregorian calendar and the week.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Weekday,
    YearKind,
};

use crate::{gregorian, julian};

/// The thirteen positions of the year: the twelve months in their modern
/// names with the extra days fourth.
pub const MONTHS: [&str; 13] = [
    "Harpa",
    "Skerpla",
    "Sólmánuður",
    "Aukanætur",
    "Heyannir",
    "Tvímánuður",
    "Haustmánuður",
    "Gormánuður",
    "Ýlir",
    "Mörsugur",
    "Þorri",
    "Góa",
    "Einmánuður",
];

/// The position of the extra days, *aukanætur* and *sumarauki*.
pub const EXTRA_DAYS: u8 = 4;

/// The *aukanætur*: the days summer is longer than winter.
pub const AUKANAETUR: u8 = 4;

/// The days of the leap week.
pub const SUMARAUKI: u8 = 7;

/// The days of winter, which are the last 180 of every year.
pub const WINTER_DAYS: i64 = 180;

/// The earliest year this implementation converts.
pub const MIN_YEAR: i64 = 1;

/// The latest year this implementation converts.
pub const MAX_YEAR: i64 = 99_998;

/// Which calendar the First Day of Summer is fixed in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rule {
    /// The Thursday 9–15 April Julian, from the twelfth century to 1700.
    Julian,
    /// The Thursday 19–25 April Gregorian, since 1700.
    Gregorian,
}

impl Rule {
    /// The fixed day of the First Day of Summer of `year`, unchecked.
    #[must_use]
    pub const fn summer_raw(self, year: i64) -> Rd {
        let window = match self {
            Self::Julian => julian::to_fixed(year, 4, 9),
            Self::Gregorian => gregorian::to_fixed(year, 4, 19),
        };
        match window {
            Ok(first) => Weekday::Thursday.on_or_after(first),
            Err(_) => Rd(0),
        }
    }

    /// The fixed day of the First Day of Summer of `year`, always a
    /// Thursday.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] outside
    /// [`MIN_YEAR`]..=[`MAX_YEAR`].
    pub const fn first_day_of_summer(self, year: i64) -> CalendarResult<Rd> {
        if year < MIN_YEAR || year > MAX_YEAR {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(self.summer_raw(year))
    }

    /// The fixed day of the First Day of Winter of `year`, always a
    /// Saturday: 180 days before the next First Day of Summer.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] outside
    /// [`MIN_YEAR`]..=[`MAX_YEAR`].
    pub const fn first_day_of_winter(self, year: i64) -> CalendarResult<Rd> {
        if year < MIN_YEAR || year > MAX_YEAR {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(Rd(self.summer_raw(year + 1).0 - WINTER_DAYS))
    }

    /// Whether `year` has *sumarauki*: whether the next First Day of Summer
    /// is 371 days away.
    #[must_use]
    pub const fn is_leap_year(self, year: i64) -> bool {
        self.summer_raw(year + 1).0 - self.summer_raw(year).0 == 371
    }

    /// The days in `year`, 364 or 371.
    #[must_use]
    pub const fn days_in_year(self, year: i64) -> u16 {
        if self.is_leap_year(year) { 371 } else { 364 }
    }

    /// The days in position `month` of `year`: 30, or 4 or 11 for the extra
    /// days; `None` outside `1..=13`.
    #[must_use]
    pub const fn days_in_month(self, year: i64, month: u8) -> Option<u8> {
        if month == 0 || month > 13 {
            None
        } else if month == EXTRA_DAYS {
            if self.is_leap_year(year) {
                Some(AUKANAETUR + SUMARAUKI)
            } else {
                Some(AUKANAETUR)
            }
        } else {
            Some(30)
        }
    }

    /// The fixed day of an Icelandic date.
    ///
    /// The first three months count forward from the First Day of Summer,
    /// the last nine back from the next one, which is Janson's advice and
    /// which puts the leap week where the extra days are.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`],
    /// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
    pub const fn to_fixed(self, year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
        let start = match self.first_day_of_summer(year) {
            Ok(start) => start,
            Err(error) => return Err(error),
        };
        if let Err(error) = crate::common::check_day(day, self.days_in_month(year, month)) {
            return Err(error);
        }
        let day = day as i64 - 1;
        if month <= EXTRA_DAYS {
            Ok(Rd(start.0 + 30 * (month as i64 - 1) + day))
        } else {
            let next = self.summer_raw(year + 1);
            Ok(Rd(next.0 - 30 * (14 - month as i64) + day))
        }
    }

    /// The Icelandic year, position and day of a fixed day.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::BeforeEpoch`] or
    /// [`CalendarError::AfterSupportedRange`] outside the range.
    pub const fn from_fixed(self, rd: Rd) -> CalendarResult<(i64, u8, u8)> {
        if rd.0 < self.earliest().0 {
            return Err(CalendarError::BeforeEpoch);
        }
        if rd.0 > self.latest().0 {
            return Err(CalendarError::AfterSupportedRange);
        }
        let calendar_year = match self {
            Self::Julian => match julian::from_fixed(rd) {
                Ok((year, _, _)) => year,
                Err(error) => return Err(error),
            },
            Self::Gregorian => match gregorian::year_from_fixed(rd) {
                Ok(year) => year,
                Err(error) => return Err(error),
            },
        };
        let year = if rd.0 < self.summer_raw(calendar_year).0 {
            calendar_year - 1
        } else {
            calendar_year
        };
        let start = self.summer_raw(year).0;
        let next = self.summer_raw(year + 1).0;
        let since = rd.0 - start;
        if since < 90 {
            return Ok((year, (since / 30 + 1) as u8, (since % 30 + 1) as u8));
        }
        let until = next - rd.0; // 1 on the last day of the year
        let months_back = (until - 1) / 30; // 0 for Einmánuður
        if months_back < 9 {
            let month = 13 - months_back;
            let day = 30 - (until - 1) % 30;
            return Ok((year, month as u8, day as u8));
        }
        Ok((year, EXTRA_DAYS, (since - 90 + 1) as u8))
    }

    /// The earliest fixed day this implementation converts.
    #[must_use]
    pub const fn earliest(self) -> Rd {
        self.summer_raw(MIN_YEAR)
    }

    /// The latest fixed day this implementation converts.
    #[must_use]
    pub const fn latest(self) -> Rd {
        Rd(self.summer_raw(MAX_YEAR + 1).0 - 1)
    }
}

/// Which reckoning of the week and the day a calendar under a [`Rule`]
/// keeps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reckoning {
    /// Winter and its weeks from the Saturday, and the day from midnight:
    /// the learned reckoning and the Almanac's since 1837.
    Standard,
    /// Winter and its weeks "reckoned from a Friday (one day before the
    /// beginning of winter as shown in Table 1)", the popular reckoning of
    /// the sixteenth century to 1837 (Janson §2.1). The months are the
    /// standard ones: Janson dates the beginning of winter, not of
    /// Gormánuður, by the Friday.
    FridayWinter,
    /// The medieval day, from sunrise in summer and dawn in winter (Janson
    /// §2.4); the week as [`Reckoning::Standard`].
    Medieval,
}

/// An Icelandic date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IcelandicDate {
    /// The year, numbered by the calendar year its summer begins in.
    pub year: i64,
    /// The position, 1 for Harpa to 13 for Einmánuður; 4 is the extra days.
    pub month: u8,
    /// The day within it.
    pub day: u8,
}

impl IcelandicDate {
    /// The name of the month, or "Aukanætur" for the extra days.
    #[must_use]
    pub const fn month_name(self) -> &'static str {
        MONTHS[self.month as usize - 1]
    }

    /// Whether this is one of the seven days of the *sumarauki*.
    #[must_use]
    pub const fn is_sumarauki(self) -> bool {
        self.month == EXTRA_DAYS && self.day > AUKANAETUR
    }

    /// Whether this falls in summer.
    #[must_use]
    pub const fn is_summer(self) -> bool {
        self.month <= 7
    }
}

/// The Icelandic calendar under one of its two rules and one reckoning of
/// the week and the day.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
///
/// The five registered pairings are the constants; the fields are private
/// so that no other pairing, such as the medieval day under the rule of
/// 1700, can be made and given a name it does not have.
pub struct IcelandicCalendar {
    rule: Rule,
    reckoning: Reckoning,
}

/// Where the Julian rule's period of use comes from.
pub const JULIAN_USAGE_SOURCE: &str = "Janson, \"The Icelandic calendar\", Scripta Islandica 62 (2011), \
    §3.2–3.4: linked to the Julian calendar in the 11th or 12th century (the Easter table of \
    1140–1195 follows the rule), undated; kept until the change to the Gregorian calendar, when \
    Saturday 16 November 1700 (Julian) was followed by Sunday 28 November";

/// Where the Gregorian rule's period of use comes from.
pub const GREGORIAN_USAGE_SOURCE: &str = "Janson, \"The Icelandic calendar\", Scripta Islandica 62 (2011), \
    §3.3–3.5: the rule since the change of 28 November 1700 (Gregorian), decided by the Althingi \
    on 1 July 1700; in general use until the late 18th century and printed in the Icelandic \
    Almanac since, the First Day of Summer still a public holiday";

/// Where the Friday reckoning's period under the Julian rule comes from.
pub const JULIAN_FRIDAY_USAGE_SOURCE: &str = "Janson, \"The Icelandic calendar\", Scripta Islandica 62 (2011), \
    §2.1: winter \"reckoned from a Friday ... from the 16th century\", \"first documented in 1508\", \
    16th-century use \"mixed\", and whether it was older contested (Björnsson and Beckman against \
    Þorkelsson), so undated at the start; the law of 1700 \"explicitly reckons winter from a \
    Friday\"; kept under the Julian rule until Saturday 16 November 1700 (Julian)";

/// Where the Friday reckoning's period under the Gregorian rule comes from.
pub const FRIDAY_USAGE_SOURCE: &str = "Janson, \"The Icelandic calendar\", Scripta Islandica 62 (2011), \
    §2.1 and §3.4: winter reckoned from a Friday under the rule of 1700 from 28 November 1700 \
    \"until the Icelandic Almanac began to be published in 1837, when the Saturday reckoning was \
    revived\": to the last day of the Icelandic year 1836, the Almanac's first year being 1837";

/// Where the medieval day's period comes from.
pub const MEDIEVAL_USAGE_SOURCE: &str = "Janson, \"The Icelandic calendar\", Scripta Islandica 62 (2011), \
    §2.4: \"the day in Iceland in the Middle Ages was reckoned from sunrise during summer and from \
    dawn during winter\", citing Schroeter; no source read dates when the day came to be reckoned \
    from midnight, so the period is bounded only by the Julian rule's, to 16 November 1700 (Julian)";

impl IcelandicCalendar {
    /// The rule since 1700, `icelandic`.
    pub const GREGORIAN: Self = Self {
        rule: Rule::Gregorian,
        reckoning: Reckoning::Standard,
    };
    /// The rule before 1700, `icelandic-julian`.
    pub const JULIAN: Self = Self {
        rule: Rule::Julian,
        reckoning: Reckoning::Standard,
    };
    /// The rule since 1700 with winter from a Friday, `icelandic-friday`.
    pub const FRIDAY: Self = Self {
        rule: Rule::Gregorian,
        reckoning: Reckoning::FridayWinter,
    };
    /// The rule before 1700 with winter from a Friday,
    /// `icelandic-julian-friday`.
    pub const JULIAN_FRIDAY: Self = Self {
        rule: Rule::Julian,
        reckoning: Reckoning::FridayWinter,
    };
    /// The rule before 1700 with the medieval day, `icelandic-medieval`.
    pub const MEDIEVAL: Self = Self {
        rule: Rule::Julian,
        reckoning: Reckoning::Medieval,
    };
    /// All five.
    pub const ALL: [Self; 5] = [
        Self::GREGORIAN,
        Self::JULIAN,
        Self::FRIDAY,
        Self::JULIAN_FRIDAY,
        Self::MEDIEVAL,
    ];

    /// The rule that fixes the First Day of Summer.
    #[must_use]
    pub const fn rule(self) -> Rule {
        self.rule
    }

    /// How winter's weeks and the day are reckoned.
    #[must_use]
    pub const fn reckoning(self) -> Reckoning {
        self.reckoning
    }

    /// The fixed day on which this reckoning begins the winter of `year`:
    /// the Saturday 180 days before the next First Day of Summer, or the
    /// Friday before it.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] outside
    /// [`MIN_YEAR`]..=[`MAX_YEAR`].
    pub const fn first_day_of_winter(self, year: i64) -> CalendarResult<Rd> {
        match self.rule.first_day_of_winter(year) {
            Ok(saturday) => match self.reckoning {
                Reckoning::FridayWinter => Ok(Rd(saturday.0 - 1)),
                Reckoning::Standard | Reckoning::Medieval => Ok(saturday),
            },
            Err(error) => Err(error),
        }
    }
}

/// Thirteen named positions and the seven-day week. The names are the
/// calendar's own, which other languages borrow.
const SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for IcelandicCalendar {
    type Date = IcelandicDate;

    /// The Julian rule until 16 November 1700 (Julian); the Gregorian rule
    /// from 28 November 1700, still printed today.
    ///
    /// The Friday reckoning from an undated start in the sixteenth century
    /// until 1700 under the Julian rule, and from 28 November 1700 to the
    /// last day of the Icelandic year 1836 under the Gregorian; the
    /// medieval day until 1700 at the latest.
    fn usage(&self) -> hc_calendar::Usage {
        let (Ok(last_julian), Ok(first_gregorian)) = (
            julian::to_fixed(1700, 11, 16),
            gregorian::to_fixed(1700, 11, 28),
        ) else {
            return hc_calendar::Usage::UNRECORDED;
        };
        match (self.rule, self.reckoning) {
            (Rule::Julian, Reckoning::Standard) => {
                hc_calendar::Usage::until(last_julian, JULIAN_USAGE_SOURCE)
            }
            (Rule::Julian, Reckoning::FridayWinter) => {
                hc_calendar::Usage::until(last_julian, JULIAN_FRIDAY_USAGE_SOURCE)
            }
            (_, Reckoning::Medieval) => {
                hc_calendar::Usage::until(last_julian, MEDIEVAL_USAGE_SOURCE)
            }
            (Rule::Gregorian, Reckoning::FridayWinter) => hc_calendar::Usage::between(
                first_gregorian,
                Rd(Rule::Gregorian.summer_raw(1837).0 - 1),
                FRIDAY_USAGE_SOURCE,
            ),
            (Rule::Gregorian, Reckoning::Standard) => {
                hc_calendar::Usage::since(first_gregorian, GREGORIAN_USAGE_SOURCE)
            }
        }
    }

    /// Midnight, except for `icelandic-medieval`'s day from sunrise in
    /// summer and dawn in winter, named by the civil day it begins on
    /// since "day comes before night throughout the Icelandic calendar"
    /// (Janson §2.4, quoting *Bókarbót*).
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        match self.reckoning {
            Reckoning::Medieval => {
                hc_calendar::DayBoundary::Daybreak(hc_calendar::DayNaming::ByStart)
            }
            Reckoning::Standard | Reckoning::FridayWinter => hc_calendar::DayBoundary::Midnight,
        }
    }

    /// Twelve months, the extra days, and the seven-day week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    /// A year with *sumarauki*.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(self.rule.is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        let (id, english_name) = match (self.rule, self.reckoning) {
            (Rule::Gregorian, Reckoning::Standard) => {
                ("icelandic", "Icelandic (misseristal, since 1700)")
            }
            (Rule::Julian, Reckoning::Standard) => {
                ("icelandic-julian", "Icelandic (misseristal, Julian rule)")
            }
            (Rule::Gregorian, Reckoning::FridayWinter) => (
                "icelandic-friday",
                "Icelandic (misseristal, since 1700, winter from a Friday)",
            ),
            (Rule::Julian, Reckoning::FridayWinter) => (
                "icelandic-julian-friday",
                "Icelandic (misseristal, Julian rule, winter from a Friday)",
            ),
            // The medieval day is paired with the Julian rule only.
            (_, Reckoning::Medieval) => (
                "icelandic-medieval",
                "Icelandic (misseristal, Julian rule, the day from daybreak)",
            ),
        };
        CalendarMeta {
            id: CalendarId(id),
            english_name,
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(self.rule.earliest()),
            latest: Some(self.rule.latest()),
            native_locales: &["is"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        self.rule.to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = self.rule.from_fixed(rd)?;
        Ok(IcelandicDate { year, month, day })
    }

    /// The year, position and day, with the season, the week of the season
    /// (0 on the two *veturnætur*) and a flag on the *sumarauki*.
    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let rd = self.to_fixed(date)?;
        let winter = self.first_day_of_winter(date.year)?;
        let (season, week) = if rd < winter {
            let since = rd.0 - self.rule.first_day_of_summer(date.year)?.0;
            let summer_weeks = (winter.0 - self.rule.summer_raw(date.year).0) / 7;
            let week = since / 7 + 1;
            (1, if week > summer_weeks { 0 } else { week })
        } else {
            (2, (rd.0 - winter.0) / 7 + 1)
        };
        DateFields::ymd(date.year, date.month, date.day)
            .with_extra("season", season)?
            .with_extra("week", week)?
            .with_extra("sumarauki", i64::from(date.is_sumarauki()))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let day = fields.require_day()?;
        self.rule.to_fixed(fields.year, month.ordinal, day)?;
        Ok(IcelandicDate {
            year: fields.year,
            month: month.ordinal,
            day,
        })
    }
}

/// The fourteen positions of the Almanac's year from 1837 to 1928: the
/// standard thirteen with the leap week, *Sumarauki*, eighth, after
/// Haustmánuður, where the printed Almanac "inserted" it "last in the
/// summer" (Janson §7.1).
pub const ALMANAC_MONTHS: [&str; 14] = [
    "Harpa",
    "Skerpla",
    "Sólmánuður",
    "Aukanætur",
    "Heyannir",
    "Tvímánuður",
    "Haustmánuður",
    "Sumarauki",
    "Gormánuður",
    "Ýlir",
    "Mörsugur",
    "Þorri",
    "Góa",
    "Einmánuður",
];

/// The position of the leap week in [`ALMANAC_MONTHS`].
pub const ALMANAC_SUMARAUKI: u8 = 8;

/// Where the Almanac's period comes from.
pub const ALMANAC_USAGE_SOURCE: &str = "Janson, \"The Icelandic calendar\", Scripta Islandica 62 (2011), \
    §7.1: \"in the printed Icelandic Almanac, which has been published since 1837, the leap week \
    was inserted last in the summer until 1928\", citing the Almanac's explanations and Schroeter; \
    from the First Day of Summer 1837 to the last day of the Icelandic year 1927, since 1928, \
    itself a leap year, is the year \"until 1928\" leaves open";

/// A date of the Almanac's year, [`IcelandicAlmanacCalendar`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IcelandicAlmanacDate {
    /// The year, numbered by the calendar year its summer begins in.
    pub year: i64,
    /// The position, 1 for Harpa to 14 for Einmánuður; 4 is the
    /// *aukanætur* and 8, in a leap year only, the *sumarauki*.
    pub month: u8,
    /// The day within it.
    pub day: u8,
}

impl IcelandicAlmanacDate {
    /// The name of the position.
    #[must_use]
    pub const fn month_name(self) -> &'static str {
        ALMANAC_MONTHS[self.month as usize - 1]
    }

    /// Whether this is one of the seven days of the *sumarauki*.
    #[must_use]
    pub const fn is_sumarauki(self) -> bool {
        self.month == ALMANAC_SUMARAUKI
    }
}

/// The Icelandic calendar as the printed Almanac kept it from 1837 until
/// 1928, `icelandic-almanac`: the rule of 1700 with the leap week after
/// Haustmánuður, so that in a leap year Heyannir, Tvímánuður and
/// Haustmánuður begin a week earlier than in `icelandic` — "shifted to
/// 22–28 July, 21–27 August and 20–26 September" — and "the reckoning by
/// weeks was not affected" (Janson §7.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct IcelandicAlmanacCalendar;

impl IcelandicAlmanacCalendar {
    /// The days in position `month` of `year`: 30, 4 for the
    /// *aukanætur*, 7 for the *sumarauki* of a leap year; `None` for the
    /// *sumarauki* of a common year and outside `1..=14`.
    #[must_use]
    pub const fn days_in_month(year: i64, month: u8) -> Option<u8> {
        match month {
            EXTRA_DAYS => Some(AUKANAETUR),
            ALMANAC_SUMARAUKI => {
                if Rule::Gregorian.is_leap_year(year) {
                    Some(SUMARAUKI)
                } else {
                    None
                }
            }
            1..=14 => Some(30),
            _ => None,
        }
    }

    /// The fixed day of an Almanac date: the first seven positions forward
    /// from the First Day of Summer, the leap week after them, and the six
    /// winter months back from the next First Day of Summer.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`],
    /// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
    pub const fn to_fixed(year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
        let start = match Rule::Gregorian.first_day_of_summer(year) {
            Ok(start) => start,
            Err(error) => return Err(error),
        };
        if let Err(error) = crate::common::check_day(day, Self::days_in_month(year, month)) {
            return Err(error);
        }
        let day = day as i64 - 1;
        let offset = match month {
            1..=4 => 30 * (month as i64 - 1),
            5..=8 => 94 + 30 * (month as i64 - 5),
            _ => {
                let next = Rule::Gregorian.summer_raw(year + 1);
                return Ok(Rd(next.0 - 30 * (15 - month as i64) + day));
            }
        };
        Ok(Rd(start.0 + offset + day))
    }

    /// The Almanac year, position and day of a fixed day.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::BeforeEpoch`] or
    /// [`CalendarError::AfterSupportedRange`] outside the range.
    pub const fn from_fixed(rd: Rd) -> CalendarResult<(i64, u8, u8)> {
        let (year, _, _) = match Rule::Gregorian.from_fixed(rd) {
            Ok(date) => date,
            Err(error) => return Err(error),
        };
        let since = rd.0 - Rule::Gregorian.summer_raw(year).0;
        let until = Rule::Gregorian.summer_raw(year + 1).0 - rd.0;
        if since < 90 {
            Ok((year, (since / 30 + 1) as u8, (since % 30 + 1) as u8))
        } else if since < 94 {
            Ok((year, EXTRA_DAYS, (since - 90 + 1) as u8))
        } else if since < 184 {
            let since = since - 94;
            Ok((year, (since / 30 + 5) as u8, (since % 30 + 1) as u8))
        } else if until > WINTER_DAYS {
            Ok((year, ALMANAC_SUMARAUKI, (since - 184 + 1) as u8))
        } else {
            let months_back = (until - 1) / 30;
            Ok((
                year,
                (14 - months_back) as u8,
                (30 - (until - 1) % 30) as u8,
            ))
        }
    }
}

/// Fourteen named positions and the seven-day week.
const ALMANAC_SHAPE: &[hc_calendar::shape::CycleShape] = &[
    hc_calendar::shape::CycleShape::named(hc_calendar::shape::MONTH, &ALMANAC_MONTHS),
    hc_calendar::shape::CycleShape::fixed(hc_calendar::shape::WEEKDAY, 7),
];

impl Calendar for IcelandicAlmanacCalendar {
    type Date = IcelandicAlmanacDate;

    /// From the First Day of Summer 1837 to the last day of 1927.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::between(
            Rule::Gregorian.summer_raw(1837),
            Rd(Rule::Gregorian.summer_raw(1928).0 - 1),
            ALMANAC_USAGE_SOURCE,
        )
    }

    /// Fourteen positions, the leap week among them, and the week.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        ALMANAC_SHAPE
    }

    /// A year with *sumarauki*, the rule of 1700's.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(Rule::Gregorian.is_leap_year(year))
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: CalendarId("icelandic-almanac"),
            english_name: "Icelandic (the Almanac's leap week, 1837–1928)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(Rule::Gregorian.earliest()),
            latest: Some(Rule::Gregorian.latest()),
            native_locales: &["is"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        Self::to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = Self::from_fixed(rd)?;
        Ok(IcelandicAlmanacDate { year, month, day })
    }

    /// The year, position and day, with the season, the week and the
    /// *sumarauki* flag as `icelandic` gives them: the weeks are the same.
    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let rd = self.to_fixed(date)?;
        let standard = IcelandicCalendar::GREGORIAN;
        let fields = standard.to_fields(standard.from_fixed(rd)?)?;
        let extra = |name: &str| fields.extra.get(name).unwrap_or(0);
        DateFields::ymd(date.year, date.month, date.day)
            .with_extra("season", extra("season"))?
            .with_extra("week", extra("week"))?
            .with_extra("sumarauki", i64::from(date.is_sumarauki()))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let day = fields.require_day()?;
        Self::to_fixed(fields.year, month.ordinal, day)?;
        Ok(IcelandicAlmanacDate {
            year: fields.year,
            month: month.ordinal,
            day,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gregorian(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).unwrap()
    }

    fn julian(year: i64, month: u8, day: u8) -> Rd {
        julian::to_fixed(year, month, day).unwrap()
    }

    #[test]
    fn the_first_day_of_summer_is_the_thursday_in_the_window() {
        // Janson §3.4: 1700 "was Thursday 11 April (Jul.), which equals
        // Thursday 22 April (Greg.)"; 1701 on Thursday 21 April.
        assert_eq!(
            Rule::Julian.first_day_of_summer(1700),
            Ok(julian(1700, 4, 11))
        );
        assert_eq!(julian(1700, 4, 11), gregorian(1700, 4, 22));
        assert_eq!(
            Rule::Gregorian.first_day_of_summer(1700),
            Ok(gregorian(1700, 4, 22))
        );
        assert_eq!(
            Rule::Gregorian.first_day_of_summer(1701),
            Ok(gregorian(1701, 4, 21))
        );
        // §3.4: in 1703 Thursday 19 April (Greg.), where the Julian rule
        // gives 15 April Jul. = 26 April Greg.
        assert_eq!(
            Rule::Gregorian.first_day_of_summer(1703),
            Ok(gregorian(1703, 4, 19))
        );
        assert_eq!(
            Rule::Julian.first_day_of_summer(1703),
            Ok(julian(1703, 4, 15))
        );
        assert_eq!(julian(1703, 4, 15), gregorian(1703, 4, 26));
        // §6.5: 2009 has dominical letter D and summer on 23 April.
        assert_eq!(
            Rule::Gregorian.first_day_of_summer(2009),
            Ok(gregorian(2009, 4, 23))
        );
        // The public holiday of 2024 to 2026.
        assert_eq!(
            Rule::Gregorian.first_day_of_summer(2024),
            Ok(gregorian(2024, 4, 25))
        );
        assert_eq!(
            Rule::Gregorian.first_day_of_summer(2025),
            Ok(gregorian(2025, 4, 24))
        );
        assert_eq!(
            Rule::Gregorian.first_day_of_summer(2026),
            Ok(gregorian(2026, 4, 23))
        );
    }

    /// Janson's closed forms (5.4) and (6.4) for the day of April.
    #[test]
    fn the_first_day_of_summer_follows_jansons_formulas() {
        for year in 1..=3_000i64 {
            let julian_day = 15 - (year + year.div_euclid(4)).rem_euclid(7);
            assert_eq!(
                Rule::Julian.first_day_of_summer(year),
                Ok(julian(year, 4, julian_day as u8)),
                "{year}"
            );
            let gregorian_day = 25
                - (year + year.div_euclid(4) - year.div_euclid(100) + year.div_euclid(400) + 5)
                    .rem_euclid(7);
            assert_eq!(
                Rule::Gregorian.first_day_of_summer(year),
                Ok(gregorian(year, 4, gregorian_day as u8)),
                "{year}"
            );
        }
    }

    #[test]
    fn the_two_rules_agree_from_1496_and_part_in_1702() {
        for year in 1496..=1702 {
            assert_eq!(
                Rule::Julian.summer_raw(year),
                Rule::Gregorian.summer_raw(year),
                "{year}"
            );
        }
        assert_ne!(
            Rule::Julian.summer_raw(1495),
            Rule::Gregorian.summer_raw(1495)
        );
        assert_ne!(
            Rule::Julian.summer_raw(1703),
            Rule::Gregorian.summer_raw(1703)
        );
        // "At Midsummer 1702 ... there would have been sumarauki in the
        // Julian version, but not in the Gregorian".
        assert!(Rule::Julian.is_leap_year(1702));
        assert!(!Rule::Gregorian.is_leap_year(1702));
        assert!(Rule::Gregorian.is_leap_year(1703));
    }

    /// Janson §5: five leap weeks in the 28-year solar cycle, in years 3, 8,
    /// 14, 20 and 25 of it, with the *rímspillir* in year 8, 1119 to 1679.
    #[test]
    fn the_julian_rule_has_five_leap_weeks_in_the_solar_cycle() {
        for year in 1100..1700i64 {
            let place = (year + 8).rem_euclid(28) + 1;
            assert_eq!(
                Rule::Julian.is_leap_year(year),
                [3, 8, 14, 20, 25].contains(&place),
                "{year}, year {place} of the solar cycle"
            );
        }
        assert_eq!(year_place(1699), 28);
        for year in (1119..=1679).step_by(28) {
            // The rímspillir year: summer on 10 April and a leap week.
            assert_eq!(Rule::Julian.summer_raw(year), julian(year, 4, 10), "{year}");
            assert!(Rule::Julian.is_leap_year(year));
        }
    }

    fn year_place(year: i64) -> i64 {
        (year + 8).rem_euclid(28) + 1
    }

    /// Janson §6: 71 leap weeks in 400 years; a leap week when summer is
    /// on 19 April, or on 20 April before a Gregorian leap year — the
    /// *rímspillir*, begun in 1719 … 2079; one gap of seven years, 1696 to
    /// 1703.
    #[test]
    fn the_gregorian_rule_has_71_leap_weeks_in_400_years() {
        assert_eq!(
            (1700..2100)
                .filter(|year| Rule::Gregorian.is_leap_year(*year))
                .count(),
            71
        );
        for year in 1700..2500 {
            let summer = Rule::Gregorian.summer_raw(year);
            let expected = summer == gregorian(year, 4, 19)
                || (summer == gregorian(year, 4, 20) && gregorian::is_leap_year(year + 1));
            assert_eq!(Rule::Gregorian.is_leap_year(year), expected, "{year}");
        }
        let rimspillir: Vec<i64> = (1700..2100)
            .filter(|year| {
                Rule::Gregorian.summer_raw(*year) == gregorian(*year, 4, 20)
                    && Rule::Gregorian.is_leap_year(*year)
            })
            .collect();
        assert_eq!(
            rimspillir,
            [
                1719, 1747, 1775, 1815, 1843, 1871, 1911, 1939, 1967, 1995, 2023, 2051, 2079
            ]
        );
        assert!(Rule::Gregorian.is_leap_year(1696));
        assert!((1697..1703).all(|year| !Rule::Gregorian.is_leap_year(year)));
        // 1899 is summer on 20 April before a common 1900, so no leap week.
        assert!(!Rule::Gregorian.is_leap_year(1899));
    }

    /// Janson Table 1: each month begins on its own weekday every year, and
    /// the First Day of Winter is a Saturday.
    #[test]
    fn every_month_begins_on_its_own_weekday() {
        use Weekday::*;
        let weekdays = [
            Thursday, Saturday, Monday, Wednesday, Sunday, Tuesday, Thursday, Saturday, Monday,
            Wednesday, Friday, Sunday, Tuesday,
        ];
        for rule in [Rule::Julian, Rule::Gregorian] {
            for year in (1100..2400).step_by(7) {
                for (month, weekday) in (1..=13u8).zip(weekdays) {
                    let first = rule.to_fixed(year, month, 1).unwrap();
                    assert_eq!(Weekday::from_rd(first), weekday, "{rule:?} {year}-{month}");
                }
                let winter = rule.first_day_of_winter(year).unwrap();
                assert_eq!(rule.to_fixed(year, 8, 1), Ok(winter));
            }
        }
        // Table 1's Gregorian windows: Þorri begins 19–26 January, Góa
        // 18–25 February, Heyannir 23–30 July in an ordinary year.
        for year in 1800..2200 {
            let thorri = Rule::Gregorian.to_fixed(year, 11, 1).unwrap();
            let (y, m, d) = gregorian::from_fixed(thorri).unwrap();
            assert_eq!((y, m), (year + 1, 1));
            assert!((19..=26).contains(&d), "{year}: Þorri on {d} January");
            let heyannir = Rule::Gregorian.to_fixed(year, 5, 1).unwrap();
            let (_, m, d) = gregorian::from_fixed(heyannir).unwrap();
            assert_eq!(m, 7);
            assert!((23..=30).contains(&d), "{year}: Heyannir on {d} July");
        }
    }

    #[test]
    fn the_extra_days_are_four_or_eleven() {
        let leap = (2000..2100)
            .find(|year| Rule::Gregorian.is_leap_year(*year))
            .unwrap();
        let common = (2000..2100)
            .find(|year| !Rule::Gregorian.is_leap_year(*year))
            .unwrap();
        assert_eq!(Rule::Gregorian.days_in_month(leap, EXTRA_DAYS), Some(11));
        assert_eq!(Rule::Gregorian.days_in_month(common, EXTRA_DAYS), Some(4));
        assert_eq!(Rule::Gregorian.days_in_year(leap), 371);
        assert_eq!(Rule::Gregorian.days_in_year(common), 364);
        assert_eq!(
            Rule::Gregorian.to_fixed(common, EXTRA_DAYS, 5),
            Err(CalendarError::DayOutOfRange)
        );
        assert_eq!(
            Rule::Gregorian.to_fixed(common, 14, 1),
            Err(CalendarError::MonthOutOfRange)
        );
        let date = IcelandicDate {
            year: leap,
            month: EXTRA_DAYS,
            day: 5,
        };
        assert!(date.is_sumarauki());
        assert_eq!(date.month_name(), "Aukanætur");
    }

    #[test]
    fn the_seasons_and_weeks_are_the_almanacs() {
        let calendar = IcelandicCalendar::GREGORIAN;
        let at = |rd: Rd| {
            calendar
                .to_fields(calendar.from_fixed(rd).unwrap())
                .unwrap()
        };
        // 23 April 2026, the First Day of Summer, is week 1 of summer.
        let first = at(gregorian(2026, 4, 23));
        assert_eq!(
            (first.extra.get("season"), first.extra.get("week")),
            (Some(1), Some(1))
        );
        // The First Day of Winter is Saturday 24 October 2026, week 1 of
        // winter; the two days before it are the veturnætur, week 0.
        let winter = Rule::Gregorian.first_day_of_winter(2026).unwrap();
        assert_eq!(winter, gregorian(2026, 10, 24));
        let fields = at(winter);
        assert_eq!(
            (fields.extra.get("season"), fields.extra.get("week")),
            (Some(2), Some(1))
        );
        let fields = at(Rd(winter.0 - 1));
        assert_eq!(fields.extra.get("week"), Some(0));
        let fields = at(Rd(winter.0 - 3));
        assert_eq!(fields.extra.get("week"), Some(26));
        // The last day of winter is in its incomplete 26th week.
        let fields = at(Rd(Rule::Gregorian.summer_raw(2027).0 - 1));
        assert_eq!(
            (fields.extra.get("season"), fields.extra.get("week")),
            (Some(2), Some(26))
        );
    }

    #[test]
    fn every_day_round_trips_under_both_rules() {
        for calendar in IcelandicCalendar::ALL {
            let rule = calendar.rule;
            let start = rule.summer_raw(1690).0;
            for rd in start..start + 30 * 371 {
                let (year, month, day) = rule.from_fixed(Rd(rd)).unwrap();
                assert_eq!(
                    rule.to_fixed(year, month, day),
                    Ok(Rd(rd)),
                    "{rule:?} rd {rd}"
                );
                let date = calendar.from_fixed(Rd(rd)).unwrap();
                let fields = calendar.to_fields(date).unwrap();
                assert_eq!(calendar.from_fields(&fields), Ok(date));
            }
            for rd in (rule.earliest().0..=rule.latest().0).step_by(9_973) {
                let (year, month, day) = rule.from_fixed(Rd(rd)).unwrap();
                assert_eq!(
                    rule.to_fixed(year, month, day),
                    Ok(Rd(rd)),
                    "{rule:?} rd {rd}"
                );
            }
            for rd in [rule.earliest(), rule.latest()] {
                let (year, month, day) = rule.from_fixed(rd).unwrap();
                assert_eq!(rule.to_fixed(year, month, day), Ok(rd));
            }
            assert_eq!(
                rule.from_fixed(Rd(rule.earliest().0 - 1)),
                Err(CalendarError::BeforeEpoch)
            );
            assert_eq!(
                rule.from_fixed(Rd(rule.latest().0 + 1)),
                Err(CalendarError::AfterSupportedRange)
            );
        }
        assert_eq!(
            Rule::Gregorian.to_fixed(0, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
    }

    /// Janson §7.1: in the Almanac's leap years the last three summer
    /// months were "shifted to 22–28 July, 21–27 August and 20–26
    /// September"; the leap week follows Haustmánuður and the weeks are
    /// `icelandic`'s.
    #[test]
    fn the_almanac_puts_the_leap_week_last_in_summer() {
        let almanac = IcelandicAlmanacCalendar;
        let mut leap_years = 0;
        for year in 1837..1928 {
            let leap = Rule::Gregorian.is_leap_year(year);
            for (month, (window_month, first, last)) in
                (5..=7u8).zip([(7, 22, 28), (8, 21, 27), (9, 20, 26)])
            {
                let start = IcelandicAlmanacCalendar::to_fixed(year, month, 1).unwrap();
                let standard = Rule::Gregorian.to_fixed(year, month, 1).unwrap();
                if leap {
                    assert_eq!(start, Rd(standard.0 - 7), "{year}-{month}");
                    let (_, m, d) = gregorian::from_fixed(start).unwrap();
                    assert_eq!(m, window_month, "{year}-{month}");
                    assert!((first..=last).contains(&d), "{year}-{month}: {d}");
                } else {
                    assert_eq!(start, standard, "{year}-{month}");
                }
            }
            if leap {
                leap_years += 1;
                let week = IcelandicAlmanacCalendar::to_fixed(year, ALMANAC_SUMARAUKI, 1).unwrap();
                let haust = IcelandicAlmanacCalendar::to_fixed(year, 7, 30).unwrap();
                assert_eq!(week, Rd(haust.0 + 1));
                let last = IcelandicAlmanacCalendar::to_fixed(year, ALMANAC_SUMARAUKI, 7).unwrap();
                assert_eq!(
                    Ok(Rd(last.0 + 1)),
                    Rule::Gregorian.first_day_of_winter(year)
                );
                assert_eq!(
                    IcelandicAlmanacCalendar::to_fixed(year, 4, 5),
                    Err(CalendarError::DayOutOfRange)
                );
            } else {
                assert_eq!(
                    IcelandicAlmanacCalendar::to_fixed(year, ALMANAC_SUMARAUKI, 1),
                    Err(CalendarError::MonthOutOfRange)
                );
            }
            // Winter is the standard winter, one position later.
            for month in 9..=14u8 {
                assert_eq!(
                    IcelandicAlmanacCalendar::to_fixed(year, month, 1),
                    Rule::Gregorian.to_fixed(year, month - 1, 1)
                );
            }
        }
        // 1838, 1843, … 1923: sixteen leap weeks before 1928, among them
        // 1888, whose Almanac "forgot to insert" it.
        assert_eq!(leap_years, 16);
        assert!(Rule::Gregorian.is_leap_year(1888));
        // 1838: summer on Thursday 19 April; Heyannir from Sunday 22 July,
        // a week before `icelandic`'s 29 July; the leap week from Saturday
        // 20 October to Friday 26 October, winter on Saturday 27 October.
        assert_eq!(
            IcelandicAlmanacCalendar::to_fixed(1838, 5, 1),
            Ok(gregorian(1838, 7, 22))
        );
        assert_eq!(
            Rule::Gregorian.to_fixed(1838, 5, 1),
            Ok(gregorian(1838, 7, 29))
        );
        assert_eq!(
            IcelandicAlmanacCalendar::to_fixed(1838, ALMANAC_SUMARAUKI, 1),
            Ok(gregorian(1838, 10, 20))
        );
        assert_eq!(
            IcelandicAlmanacCalendar::to_fixed(1838, 9, 1),
            Ok(gregorian(1838, 10, 27))
        );
        let date = almanac.from_fixed(gregorian(1838, 10, 21)).unwrap();
        assert_eq!((date.month_name(), date.day), ("Sumarauki", 2));
        assert!(date.is_sumarauki());
    }

    #[test]
    fn the_almanacs_weeks_are_the_standard_weeks_and_every_day_round_trips() {
        let almanac = IcelandicAlmanacCalendar;
        let standard = IcelandicCalendar::GREGORIAN;
        let start = Rule::Gregorian.summer_raw(1836).0;
        for rd in start..Rule::Gregorian.summer_raw(1931).0 {
            let date = almanac.from_fixed(Rd(rd)).unwrap();
            assert_eq!(almanac.to_fixed(date), Ok(Rd(rd)), "rd {rd}");
            let fields = almanac.to_fields(date).unwrap();
            assert_eq!(almanac.from_fields(&fields), Ok(date));
            let other = standard
                .to_fields(standard.from_fixed(Rd(rd)).unwrap())
                .unwrap();
            for name in ["season", "week"] {
                assert_eq!(
                    fields.extra.get(name),
                    other.extra.get(name),
                    "{name} rd {rd}"
                );
            }
        }
        for rd in (Rule::Gregorian.earliest().0..=Rule::Gregorian.latest().0).step_by(9_973) {
            let (year, month, day) = IcelandicAlmanacCalendar::from_fixed(Rd(rd)).unwrap();
            assert_eq!(
                IcelandicAlmanacCalendar::to_fixed(year, month, day),
                Ok(Rd(rd))
            );
        }
        assert_eq!(
            IcelandicAlmanacCalendar::from_fixed(Rd(Rule::Gregorian.earliest().0 - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            IcelandicAlmanacCalendar::to_fixed(2026, 15, 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            IcelandicAlmanacCalendar::to_fixed(0, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(almanac.is_leap_year(1838), Ok(true));
        assert_eq!(almanac.is_leap_year(0), Err(CalendarError::YearOutOfRange));
    }

    /// Janson §2.1 and §3.4: the law of 1 July 1700 put winter on "that
    /// Friday that is between the 19th and 28th October", where it had
    /// been "between the 9th and 18th October" (Julian). In 1702 "the
    /// First Day of Winter in most places was taken to be Friday 20
    /// October", which is the rule as the Althingi settled it in 1703;
    /// the first form of the decision gave 27 October (n. 47, after
    /// Schroeter).
    #[test]
    fn winter_from_a_friday_is_the_day_before_the_saturday() {
        for (calendar, first, last) in [
            (IcelandicCalendar::FRIDAY, 20, 27),
            (IcelandicCalendar::JULIAN_FRIDAY, 10, 17),
        ] {
            for year in 1500..1900 {
                let friday = calendar.first_day_of_winter(year).unwrap();
                assert_eq!(Weekday::from_rd(friday), Weekday::Friday);
                assert_eq!(
                    Ok(Rd(friday.0 + 1)),
                    calendar.rule().first_day_of_winter(year)
                );
                let (_, month, day) = match calendar.rule() {
                    Rule::Gregorian => gregorian::from_fixed(friday).unwrap(),
                    Rule::Julian => julian::from_fixed(friday).unwrap(),
                };
                assert_eq!(month, 10);
                assert!((first..=last).contains(&day), "{year}: {day} October");
            }
        }
        assert_eq!(
            IcelandicCalendar::FRIDAY.first_day_of_winter(1702),
            Ok(gregorian(1702, 10, 20))
        );
        // The dates are the standard calendar's; the season and week turn
        // a day earlier, and the last week of winter has six days.
        let friday = IcelandicCalendar::FRIDAY;
        let at = |rd: Rd| friday.to_fields(friday.from_fixed(rd).unwrap()).unwrap();
        let winter = friday.first_day_of_winter(1750).unwrap();
        assert_eq!(
            friday.from_fixed(winter),
            IcelandicCalendar::GREGORIAN.from_fixed(winter)
        );
        let first = at(winter);
        assert_eq!(
            (first.extra.get("season"), first.extra.get("week")),
            (Some(2), Some(1))
        );
        let before = at(Rd(winter.0 - 1));
        assert_eq!(
            (before.extra.get("season"), before.extra.get("week")),
            (Some(1), Some(0))
        );
        let summer_week = at(Rd(winter.0 - 2));
        assert_ne!(summer_week.extra.get("week"), Some(0));
        let last = at(Rd(Rule::Gregorian.summer_raw(1751).0 - 1));
        assert_eq!(
            (last.extra.get("season"), last.extra.get("week")),
            (Some(2), Some(26))
        );
        let standard = IcelandicCalendar::GREGORIAN;
        let saturday = standard
            .to_fields(standard.from_fixed(winter).unwrap())
            .unwrap();
        assert_eq!(saturday.extra.get("season"), Some(1));
    }

    #[test]
    fn the_medieval_day_begins_at_daybreak() {
        use hc_calendar::{DayBoundary, DayNaming};
        assert_eq!(
            IcelandicCalendar::MEDIEVAL.day_boundary(),
            DayBoundary::Daybreak(DayNaming::ByStart)
        );
        for calendar in [
            IcelandicCalendar::GREGORIAN,
            IcelandicCalendar::JULIAN,
            IcelandicCalendar::FRIDAY,
            IcelandicCalendar::JULIAN_FRIDAY,
        ] {
            assert_eq!(calendar.day_boundary(), DayBoundary::Midnight);
        }
        assert_eq!(
            IcelandicAlmanacCalendar.day_boundary(),
            DayBoundary::Midnight
        );
        let rd = julian(1250, 6, 1);
        assert_eq!(
            IcelandicCalendar::MEDIEVAL.from_fixed(rd),
            IcelandicCalendar::JULIAN.from_fixed(rd)
        );
    }

    #[test]
    fn the_two_identifiers_and_their_periods() {
        use hc_calendar::Standing;
        assert_eq!(
            IcelandicCalendar::GREGORIAN.meta().id,
            CalendarId("icelandic")
        );
        assert_eq!(
            IcelandicCalendar::JULIAN.meta().id,
            CalendarId("icelandic-julian")
        );
        let today = gregorian(2026, 9, 26);
        assert_eq!(
            IcelandicCalendar::GREGORIAN.standing(today),
            Standing::InUse
        );
        assert_eq!(
            IcelandicCalendar::JULIAN.standing(today),
            Standing::Extended
        );
        assert_eq!(
            IcelandicCalendar::JULIAN.standing(julian(1600, 1, 1)),
            Standing::InUse
        );
        assert_eq!(
            IcelandicCalendar::GREGORIAN.standing(gregorian(1700, 11, 27)),
            Standing::Proleptic
        );
        let ids: Vec<&str> = IcelandicCalendar::ALL
            .iter()
            .map(|calendar| calendar.meta().id.0)
            .chain([IcelandicAlmanacCalendar.meta().id.0])
            .collect();
        assert_eq!(
            ids,
            [
                "icelandic",
                "icelandic-julian",
                "icelandic-friday",
                "icelandic-julian-friday",
                "icelandic-medieval",
                "icelandic-almanac"
            ]
        );
        let friday = IcelandicCalendar::FRIDAY;
        assert_eq!(friday.standing(gregorian(1800, 1, 1)), Standing::InUse);
        assert_eq!(friday.standing(gregorian(1837, 4, 19)), Standing::InUse);
        assert_eq!(friday.standing(gregorian(1837, 4, 20)), Standing::Extended);
        assert_eq!(
            IcelandicCalendar::JULIAN_FRIDAY.standing(julian(1600, 1, 1)),
            Standing::InUse
        );
        assert_eq!(
            IcelandicCalendar::MEDIEVAL.standing(julian(1701, 1, 1)),
            Standing::Extended
        );
        let almanac = IcelandicAlmanacCalendar;
        assert_eq!(
            almanac.standing(gregorian(1837, 4, 19)),
            Standing::Proleptic
        );
        assert_eq!(almanac.standing(gregorian(1837, 4, 20)), Standing::InUse);
        assert_eq!(almanac.standing(gregorian(1928, 4, 18)), Standing::InUse);
        assert_eq!(almanac.standing(gregorian(1928, 4, 19)), Standing::Extended);
    }
}
