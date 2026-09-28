//! The Chinese eras that ran on the Gregorian calendar: 洪憲 of 1916, and
//! Manchukuo's 大同 and 康德.
//!
//! After the Republic adopted the Gregorian calendar on 1 January 1912,
//! two regimes named its years by an era again, and each dated the era
//! from a Gregorian day, not from a lunisolar New Year:
//!
//! * **洪憲**, `hongxian`: Yuan Shikai's Empire of China. The order of
//!   31 December 1915 made the next year 洪憲元年, and the order of
//!   23 March 1916 abolished the era and made the year 中華民國五年 again, as
//!   維基百科「洪憲」 quotes both from the 新聞報 (`wikipedia-zh-hongxian`).
//!   The Chinese article gives the era as 1 January to 23 March 1916, the
//!   Japanese one to 22 March (`wikipedia-ja-hongxian`); the calendar runs
//!   to the 22nd, the last day both give, and refuses the 23rd, which
//!   the sources dispute.
//! * **大同** and **康德**, `manchukuo`: 大同 from the founding of Manchukuo
//!   on 1 March 1932, by its 政府佈告 二 (`wikipedia-ja-datong-manchukuo`,
//!   `wikipedia-zh-datong-manchukuo`), and 康德 from 大同3年3月1日,
//!   1 March 1934, when 溥儀 was enthroned (`wikipedia-ja-kangde`,
//!   `wikipedia-zh-kangde`). The state ended with the abdication, which
//!   the Chinese article dates to 17 August 1945 and the Japanese one to
//!   the 18th; the calendar runs to the 17th and refuses the 18th.
//!
//! An era's year 1 is the Gregorian year it began in, and a year of two
//! eras is split at the change, as in [`crate::korean_regnal`]: 28 February
//! 1934 is 大同3年2月28日 and the next day 康德元年3月1日. Neither regime
//! backdated.
//!
//! The eras are written up with the Qing ones in
//! [`docs/systems/east-asian-eras.md`](https://github.com/kitsuyui/hyper-calendar/blob/main/docs/systems/east-asian-eras.md).

use core::fmt;

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};
use hc_calendars_solar::gregorian;

/// An era that began on a Gregorian day.
#[derive(Debug, Clone, Copy)]
pub struct GregorianEra {
    /// The machine identifier, the pinyin in lower case.
    pub id: &'static str,
    /// The name in traditional characters.
    pub hanzi: &'static str,
    /// The name in pinyin with an initial capital.
    pub pinyin: &'static str,
    /// The Gregorian year of the era's year 1.
    pub start_year: i64,
    /// The first day of the era.
    pub start: Rd,
}

impl PartialEq for GregorianEra {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for GregorianEra {}

const fn day(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(rd) => rd,
        Err(_) => Rd(0),
    }
}

/// 洪憲, from 1 January 1916 (`wikipedia-zh-hongxian`,
/// `wikipedia-ja-hongxian`).
pub static HONGXIAN_ERA: GregorianEra = GregorianEra {
    id: "hongxian",
    hanzi: "洪憲",
    pinyin: "Hongxian",
    start_year: 1916,
    start: day(1916, 1, 1),
};

/// 大同, from 1 March 1932, the founding of Manchukuo
/// (`wikipedia-ja-datong-manchukuo`, `wikipedia-zh-datong-manchukuo`).
pub static DATONG: GregorianEra = GregorianEra {
    id: "datong",
    hanzi: "大同",
    pinyin: "Datong",
    start_year: 1932,
    start: day(1932, 3, 1),
};

/// 康德, from 1 March 1934, 大同3年3月1日, the enthronement
/// (`wikipedia-ja-kangde`, `wikipedia-zh-kangde`).
pub static KANGDE: GregorianEra = GregorianEra {
    id: "kangde",
    hanzi: "康德",
    pinyin: "Kangde",
    start_year: 1934,
    start: day(1934, 3, 1),
};

/// A regime's eras on the Gregorian calendar, and the days it kept them.
#[derive(Debug, PartialEq, Eq)]
pub struct GregorianEraSystem {
    /// The calendar's identifier.
    pub id: CalendarId,
    /// The calendar's English name.
    pub english_name: &'static str,
    /// The eras in order.
    pub eras: &'static [&'static GregorianEra],
    /// The last day the calendar converts, the last day every source read
    /// gives the regime.
    pub latest: Rd,
    /// Where the period of use comes from.
    pub usage_source: &'static str,
}

impl GregorianEraSystem {
    /// The first day the calendar converts, its first era's.
    #[must_use]
    pub const fn earliest(&self) -> Rd {
        self.eras[0].start
    }

    /// The era in force on a day.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::BeforeEpoch`] before the first era and
    /// [`CalendarError::AfterSupportedRange`] after [`Self::latest`].
    pub fn era_at(&self, rd: Rd) -> CalendarResult<&'static GregorianEra> {
        if rd < self.earliest() {
            return Err(CalendarError::BeforeEpoch);
        }
        if rd > self.latest {
            return Err(CalendarError::AfterSupportedRange);
        }
        Ok(self
            .eras
            .iter()
            .rev()
            .find(|era| era.start <= rd)
            .copied()
            .unwrap_or(self.eras[0]))
    }

    /// The era with this identifier, by [`hc_core::catalogue::matches`].
    #[must_use]
    pub fn by_id(&self, id: &str) -> Option<&'static GregorianEra> {
        self.eras
            .iter()
            .find(|era| hc_core::catalogue::matches(id, era.id))
            .copied()
    }

    /// The date of a fixed day.
    ///
    /// # Errors
    ///
    /// Returns the errors of [`Self::era_at`].
    pub fn from_fixed(&self, rd: Rd) -> CalendarResult<GregorianEraDate> {
        let era = self.era_at(rd)?;
        let (year, month, day) = gregorian::from_fixed(rd)?;
        Ok(GregorianEraDate {
            era,
            year: year - era.start_year + 1,
            month,
            day,
        })
    }

    /// The fixed day of a date, checked against the era's span.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] when the era had no such
    /// year, [`CalendarError::DayOutOfRange`] when it had the year but not
    /// the day, or the Gregorian errors.
    pub fn to_fixed(&self, date: GregorianEraDate) -> CalendarResult<Rd> {
        if date.year < 1 {
            return Err(CalendarError::YearOutOfRange);
        }
        let rd = gregorian::to_fixed(date.gregorian_year(), date.month, date.day)?;
        let next = self
            .eras
            .iter()
            .find(|era| era.start > date.era.start)
            .map_or(Rd(self.latest.0 + 1), |era| era.start);
        if rd < date.era.start || rd >= next {
            let (first_year, _, _) = gregorian::from_fixed(date.era.start)?;
            let (last_year, _, _) = gregorian::from_fixed(Rd(next.0 - 1))?;
            let year = date.gregorian_year();
            if year < first_year || year > last_year {
                return Err(CalendarError::YearOutOfRange);
            }
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(rd)
    }
}

/// Yuan Shikai's era, 1 to 22 March 1916: `hongxian`.
pub static HONGXIAN: GregorianEraSystem = GregorianEraSystem {
    id: CalendarId("hongxian"),
    english_name: "Hongxian era (Empire of China, 1916)",
    eras: &[&HONGXIAN_ERA],
    latest: day(1916, 3, 22),
    usage_source: "洪憲元年 from 1 January 1916 by the order of 31 December 1915, abolished by the order of \
        23 March 1916, as 維基百科「洪憲」 quotes them from the 新聞報 [wikipedia-zh-hongxian]; to 22 March \
        in [wikipedia-ja-hongxian], the last day both give",
};

/// Manchukuo's eras, 1 March 1932 to 17 August 1945: `manchukuo`.
pub static MANCHUKUO: GregorianEraSystem = GregorianEraSystem {
    id: CalendarId("manchukuo"),
    english_name: "Manchukuo eras",
    eras: &[&DATONG, &KANGDE],
    latest: day(1945, 8, 17),
    usage_source: "大同 from the founding of Manchukuo on 1 March 1932 and 康德 from 1 March 1934 \
        [wikipedia-ja-datong-manchukuo, wikipedia-zh-datong-manchukuo, wikipedia-ja-kangde, \
        wikipedia-zh-kangde]; to the abdication of 17 August 1945 [wikipedia-zh-kangde], the 18th in \
        [wikipedia-ja-kangde]",
};

/// A date in an era of a [`GregorianEraSystem`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GregorianEraDate {
    /// The era.
    pub era: &'static GregorianEra,
    /// The year of the era, counting from 1.
    pub year: i64,
    /// The month, 1 for January through 12 for December.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl GregorianEraDate {
    /// The Gregorian year this date falls in.
    #[must_use]
    pub const fn gregorian_year(&self) -> i64 {
        self.era.start_year + self.year - 1
    }
}

impl fmt::Display for GregorianEraDate {
    /// Writes the date as the regime's documents did: `康德元年3月1日`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.era.hanzi)?;
        if self.year == 1 {
            f.write_str("元年")?;
        } else {
            write!(f, "{}年", self.year)?;
        }
        write!(f, "{}月{}日", self.month, self.day)
    }
}

/// A [`GregorianEraSystem`] as a calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GregorianEraCalendar(pub &'static GregorianEraSystem);

impl GregorianEraCalendar {
    /// `hongxian`.
    pub const HONGXIAN: Self = Self(&HONGXIAN);

    /// `manchukuo`.
    pub const MANCHUKUO: Self = Self(&MANCHUKUO);

    /// Both calendars.
    pub const ALL: [Self; 2] = [Self::HONGXIAN, Self::MANCHUKUO];
}

impl Calendar for GregorianEraCalendar {
    type Date = GregorianEraDate;

    /// The days the regime kept its eras, which are also the whole range
    /// the calendar converts.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::between(self.0.earliest(), self.0.latest, self.0.usage_source)
    }

    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        hc_calendar::shape::SOLAR_TWELVE
    }

    /// For a Gregorian year with no era, within the years the eras span.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        let (first, _, _) = gregorian::from_fixed(self.0.earliest())?;
        let (last, _, _) = gregorian::from_fixed(self.0.latest)?;
        if year < first || year > last {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(gregorian::is_leap_year(year))
    }

    /// Resolves the era first: the fields count years within it.
    fn is_leap_year_of(&self, fields: &DateFields) -> CalendarResult<bool> {
        match fields.era {
            Some(name) => {
                let era = self.0.by_id(name).ok_or(CalendarError::UnknownEra)?;
                self.is_leap_year(era.start_year + fields.year - 1)
            }
            None => self.is_leap_year(fields.year),
        }
    }

    /// The era in traditional characters with its pinyin.
    fn era_name(&self, code: &str) -> Option<hc_calendar::EraName> {
        self.0
            .by_id(code)
            .map(|era| hc_calendar::EraName::new(era.hanzi, era.pinyin))
    }

    /// The eras, in order.
    fn era_code(&self, index: usize) -> Option<&'static str> {
        self.0.eras.get(index).map(|era| era.id)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: self.0.id,
            english_name: self.0.english_name,
            year_kind: YearKind::EraRelative,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(self.0.earliest()),
            latest: Some(self.0.latest),
            native_locales: &["zh-Hant"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        self.0.to_fixed(date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        self.0.from_fixed(rd)
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields::ymd(date.year, date.month, date.day).with_era(date.era.id))
    }

    /// Reads era-tagged fields, or fields with no era as a Gregorian year.
    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let month: Month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let day = fields.require_day()?;
        match fields.era {
            Some(name) => {
                let era = self.0.by_id(name).ok_or(CalendarError::UnknownEra)?;
                let date = GregorianEraDate {
                    era,
                    year: fields.year,
                    month: month.ordinal,
                    day,
                };
                self.0.to_fixed(date)?;
                Ok(date)
            }
            None => self
                .0
                .from_fixed(gregorian::to_fixed(fields.year, month.ordinal, day)?),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("valid Gregorian date")
    }

    fn written(calendar: GregorianEraCalendar, rd: Rd) -> CalendarResult<alloc::string::String> {
        use alloc::string::ToString;
        calendar.from_fixed(rd).map(|date| date.to_string())
    }

    /// 洪憲元年 from 1 January 1916 to 22 March (`wikipedia-zh-hongxian`,
    /// `wikipedia-ja-hongxian`); the disputed 23rd is refused.
    #[test]
    fn hongxian_ran_from_new_year_to_the_twenty_second_of_march_1916() {
        let calendar = GregorianEraCalendar::HONGXIAN;
        assert_eq!(
            written(calendar, greg(1916, 1, 1)),
            Ok("洪憲元年1月1日".into())
        );
        assert_eq!(
            written(calendar, greg(1916, 3, 22)),
            Ok("洪憲元年3月22日".into())
        );
        assert_eq!(
            calendar.from_fixed(greg(1916, 3, 23)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(
            calendar.from_fixed(greg(1915, 12, 31)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(calendar.meta().id.as_str(), "hongxian");
    }

    /// 大同 from 1 March 1932, 康德 from 大同3年3月1日 = 1 March 1934, to
    /// 17 August 1945 (`wikipedia-ja-datong-manchukuo`,
    /// `wikipedia-zh-datong-manchukuo`, `wikipedia-ja-kangde`,
    /// `wikipedia-zh-kangde`).
    #[test]
    fn manchukuo_changed_from_datong_to_kangde_on_the_first_of_march_1934() {
        let calendar = GregorianEraCalendar::MANCHUKUO;
        for (rd, text) in [
            (greg(1932, 3, 1), "大同元年3月1日"),
            (greg(1932, 9, 15), "大同元年9月15日"),
            (greg(1934, 2, 28), "大同3年2月28日"),
            (greg(1934, 3, 1), "康德元年3月1日"),
            (greg(1940, 6, 26), "康德7年6月26日"),
            (greg(1945, 8, 17), "康德12年8月17日"),
        ] {
            assert_eq!(written(calendar, rd).as_deref(), Ok(text));
        }
        assert_eq!(
            calendar.from_fixed(greg(1945, 8, 18)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(
            calendar.from_fixed(greg(1932, 2, 29)),
            Err(CalendarError::BeforeEpoch)
        );
        let datong_three_march = GregorianEraDate {
            era: &DATONG,
            year: 3,
            month: 3,
            day: 1,
        };
        assert_eq!(
            calendar.to_fixed(datong_three_march),
            Err(CalendarError::DayOutOfRange)
        );
        let datong_four = GregorianEraDate {
            era: &DATONG,
            year: 4,
            month: 1,
            day: 1,
        };
        assert_eq!(
            calendar.to_fixed(datong_four),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn every_day_round_trips_through_both_calendars_and_their_fields() {
        for calendar in GregorianEraCalendar::ALL {
            for rd in (calendar.0.earliest().0..=calendar.0.latest.0).step_by(3) {
                let date = calendar.from_fixed(Rd(rd)).expect("in range");
                assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)));
                let fields = calendar.to_fields(date).expect("describable");
                assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
            }
            assert_eq!(
                calendar.from_fields(&DateFields::ymd(1, 1, 1).with_era("meiji")),
                Err(CalendarError::UnknownEra)
            );
        }
        assert_eq!(MANCHUKUO.by_id(" Kangde "), Some(&KANGDE));
    }
}
