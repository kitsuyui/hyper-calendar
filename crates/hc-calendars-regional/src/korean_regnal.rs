//! The era names of the Korean Empire, 1896–1910.
//!
//! Korea adopted the Gregorian calendar on 1 January 1896 and, on the same
//! day, an era name of its own: 建陽 元年 1月 1日 was that day, the lunar
//! 開國 504年 11月 17日. Two eras followed it — 光武 from 14 August 1897,
//! under which the Korean Empire was proclaimed that October, and 隆熙 from
//! 2 August 1907, the accession of Sunjong — and the last ended with the
//! annexation on 29 August 1910. Three eras, three proclamation days, on
//! the Gregorian calendar throughout: that is this module.
//!
//! An era's year 1 is the Gregorian year it was proclaimed in, so a day
//! before the change belongs to the previous era's last year: 13 August
//! 1897 is 建陽 2年 8月 13日 and the next day is 光武 元年 8月 14日. This is
//! the era from the day it was fixed, not backdated: `korean-regnal`.
//!
//! The decree of 光武 made "是年", the whole of 1897, 光武元年, and that
//! reading is its own calendar, `korean-regnal-backdated`
//! ([`KoreanRegnalBackdatedCalendar`]): 建陽 is 1896 alone and 1 January
//! 1897 is 光武 元年 1月 1日. 隆熙 keeps its day of choice there, 2 August
//! 1907, because the annals' entries for it carry no such clause: 隆熙 was
//! chosen on 순종 즉위년 8월 2일 and the entry of the 3rd is headed 隆熙
//! 元年八月三日 (`sillok-sunjong`), and nothing read backdates it.
//!
//! # Which day an era begins on
//!
//! The annals separate the day an era name was chosen from the days it was
//! announced and first used, and this module carries the day it was chosen:
//!
//! * **光武.** 고종실록 records the choice of 光武 over 慶德 on 고종 34년
//!   8월 14일, the decree of the 15th that made "是年" 光武元年 and set the
//!   proclamation for the 16th, and the proclamation at the altars on the
//!   16th (`sillok-gojong`). The Encyclopedia of Korean Culture has the era
//!   in use from the 16th (`encykorea-gwangmu`); the Korean Wikipedia from
//!   the 17th (`kowiki-gwangmu`). The decree's "是年" makes the whole of
//!   1897 光武 元年, as a chronological table would read it, which is
//!   `korean-regnal-backdated`; the annals head their entry of 1 January
//!   1897 建陽 2년, as the days before the change were dated, which is
//!   `korean-regnal`.
//! * **隆熙.** 순종실록 records the choice of 隆熙 over 太始 on 순종 즉위년
//!   8월 2일, 1907 (`sillok-sunjong`); the Korean Wikipedia has it in use
//!   from 3 August (`kowiki-yunghui`).
//!
//! The three eras and their shared reading are written up in
//! [`docs/systems/east-asian-eras.md`](https://github.com/kitsuyui/hyper-calendar/blob/main/docs/systems/east-asian-eras.md).
//!
//! # What is beside it
//!
//! Before 1896 Joseon dated by the Chinese eras on the lunisolar calendar,
//! and in 1894–1895 by 開國, the count from the dynasty's founding in 1392:
//! [`gaeguk_year`] gives that count for any year, and the days themselves
//! are `dangi`'s. From 29 August 1910 the Japanese eras ran, which are
//! [`crate::japanese`]'s. The Republic's 大韓民國 count from 1919 and the
//! 檀君紀元 of 1948–1961 are namings of the Gregorian year and are not
//! here.
//!
//! Sources, as keyed in `docs/references.bib`: `kowiki-geonyang` for
//! 建陽 from 1 January 1896; `sillok-gojong` and `sillok-sunjong` for the
//! days 光武 and 隆熙 were chosen, read 2026-09-26; `encykorea-gwangmu`,
//! `kowiki-gwangmu` and `kowiki-yunghui` for the days of first use;
//! `wikipedia-ja-gaeguk` (開国 (李氏朝鮮), retrieved 2026-09-22) for the
//! 1392 epoch; `wikipedia-en-korean-era-name` ("Korean era name", retrieved
//! 2026-09-22) for the sequence.

use core::fmt;

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};
use hc_calendars_solar::gregorian;

/// An era of the Korean Empire.
#[derive(Debug, Clone, Copy)]
pub struct KoreanEra {
    /// The machine identifier, the Revised Romanisation in lower case.
    pub id: &'static str,
    /// The name in Hanja.
    pub hanja: &'static str,
    /// The name in Hangul.
    pub hangul: &'static str,
    /// The name in Revised Romanisation with an initial capital.
    pub romanised: &'static str,
    /// The Gregorian year of the era's year 1.
    pub start_year: i64,
    /// The first day of the era.
    pub start: Rd,
    /// The first day of the era under the backdated reading: the first day
    /// of its year where the decree made that year its first, [`Self::start`]
    /// otherwise.
    pub backdated_start: Rd,
}

impl PartialEq for KoreanEra {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for KoreanEra {}

const fn day(year: i64, month: u8, day: u8) -> Rd {
    match gregorian::to_fixed(year, month, day) {
        Ok(rd) => rd,
        Err(_) => Rd(0),
    }
}

/// 建陽, from 1 January 1896.
pub static GEONYANG: KoreanEra = KoreanEra {
    id: "geonyang",
    hanja: "建陽",
    hangul: "건양",
    romanised: "Geonyang",
    start_year: 1896,
    start: day(1896, 1, 1),
    backdated_start: day(1896, 1, 1),
};

/// 光武, from 14 August 1897, the day it was chosen (고종실록, 고종 34년
/// 8월 14일).
pub static GWANGMU: KoreanEra = KoreanEra {
    id: "gwangmu",
    hanja: "光武",
    hangul: "광무",
    romanised: "Gwangmu",
    start_year: 1897,
    start: day(1897, 8, 14),
    // 「以是年爲光武元年」, the decree of 고종 34년 8월 15일 (`sillok-gojong`).
    backdated_start: day(1897, 1, 1),
};

/// 隆熙, from 2 August 1907, the day it was chosen (순종실록, 순종 즉위년
/// 8월 2일).
pub static YUNGHUI: KoreanEra = KoreanEra {
    id: "yunghui",
    hanja: "隆熙",
    hangul: "융희",
    romanised: "Yunghui",
    start_year: 1907,
    start: day(1907, 8, 2),
    backdated_start: day(1907, 8, 2),
};

/// The three eras in order.
pub static ALL: [&KoreanEra; 3] = [&GEONYANG, &GWANGMU, &YUNGHUI];

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "建陽 元年 1月 1日 = 1 January 1896 [kowiki-geonyang] to the annexation of 29 August 1910 \
    [wikipedia-en-korean-era-name]; the days 光武 and 隆熙 were chosen [sillok-gojong, sillok-sunjong]";

/// The first day this calendar converts, 建陽 元年 1月 1日.
pub const EARLIEST: Rd = day(1896, 1, 1);

/// The last day this calendar converts, 隆熙 4年 8月 29日, the day of the
/// annexation.
pub const LATEST: Rd = day(1910, 8, 29);

/// The year the 開國 count gives to `year`: 1392, the founding of Joseon, is
/// 開國 元年, so 1894 is 開國 503年.
#[must_use]
pub const fn gaeguk_year(year: i64) -> i64 {
    year - 1_391
}

/// Which day an era is read from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Reading {
    /// From the day it was chosen, [`KoreanEra::start`]: `korean-regnal`.
    Chosen,
    /// From the first day of the year its decree made its first,
    /// [`KoreanEra::backdated_start`]: `korean-regnal-backdated`.
    Backdated,
}

impl Reading {
    /// The first day of `era` under this reading.
    #[must_use]
    pub const fn start(self, era: &KoreanEra) -> Rd {
        match self {
            Self::Chosen => era.start,
            Self::Backdated => era.backdated_start,
        }
    }
}

/// The era in force on a day.
///
/// # Errors
///
/// Returns [`CalendarError::BeforeEpoch`] before 1896 and
/// [`CalendarError::AfterSupportedRange`] after the annexation.
pub fn era_at(rd: Rd) -> CalendarResult<&'static KoreanEra> {
    era_under(Reading::Chosen, rd)
}

/// The era in force on a day under `reading`.
///
/// # Errors
///
/// As [`era_at`].
pub fn era_under(reading: Reading, rd: Rd) -> CalendarResult<&'static KoreanEra> {
    if rd < EARLIEST {
        return Err(CalendarError::BeforeEpoch);
    }
    if rd > LATEST {
        return Err(CalendarError::AfterSupportedRange);
    }
    Ok(ALL
        .iter()
        .rev()
        .find(|era| reading.start(era) <= rd)
        .copied()
        .unwrap_or(&GEONYANG))
}

/// The era with this identifier, by [`hc_core::catalogue::matches`].
#[must_use]
pub fn by_id(id: &str) -> Option<&'static KoreanEra> {
    ALL.iter()
        .find(|era| hc_core::catalogue::matches(id, era.id))
        .copied()
}

/// A date in an era of the Korean Empire.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KoreanRegnalDate {
    /// The era.
    pub era: &'static KoreanEra,
    /// The year of the era, counting from 1.
    pub year: i64,
    /// The month, 1 for January through 12 for December.
    pub month: u8,
    /// The day of the month, counting from 1.
    pub day: u8,
}

impl KoreanRegnalDate {
    /// The Gregorian year this date falls in.
    #[must_use]
    pub const fn gregorian_year(&self) -> i64 {
        self.era.start_year + self.year - 1
    }
}

impl fmt::Display for KoreanRegnalDate {
    /// Writes the date as it was written: `光武 元年 8月 14日`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.year == 1 {
            write!(f, "{} 元年 {}月 {}日", self.era.hanja, self.month, self.day)
        } else {
            write!(
                f,
                "{} {}年 {}月 {}日",
                self.era.hanja, self.year, self.month, self.day
            )
        }
    }
}

/// The eras of the Korean Empire on the Gregorian calendar, each from the
/// day it was chosen: `korean-regnal`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct KoreanRegnalCalendar;

/// The eras of the Korean Empire with 光武 backdated to 1 January 1897, as
/// its decree made the year its first: `korean-regnal-backdated`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct KoreanRegnalBackdatedCalendar;

/// The fixed day of a date, checked against the era's span.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] when the era had no such year,
/// [`CalendarError::DayOutOfRange`] when it had the year but not the day —
/// 建陽 2年 8月 14日 is the first — or the Gregorian errors.
pub fn to_fixed(date: KoreanRegnalDate) -> CalendarResult<Rd> {
    to_fixed_under(Reading::Chosen, date)
}

/// The fixed day of a date under `reading`.
///
/// # Errors
///
/// As [`to_fixed`]: under [`Reading::Backdated`] 建陽 2年 has no day at
/// all, and 光武 元年 1월 1일 is the first of 1897.
pub fn to_fixed_under(reading: Reading, date: KoreanRegnalDate) -> CalendarResult<Rd> {
    if date.year < 1 {
        return Err(CalendarError::YearOutOfRange);
    }
    let rd = gregorian::to_fixed(date.gregorian_year(), date.month, date.day)?;
    let start = reading.start(date.era);
    let next = ALL
        .iter()
        .find(|era| era.start > date.era.start)
        .map_or(Rd(LATEST.0 + 1), |era| reading.start(era));
    if rd < start || rd >= next {
        if next <= start {
            return Err(CalendarError::YearOutOfRange);
        }
        let (first_year, _, _) = gregorian::from_fixed(start)?;
        let (last_year, _, _) = gregorian::from_fixed(Rd(next.0 - 1))?;
        let year = date.gregorian_year();
        if year < first_year || year > last_year {
            return Err(CalendarError::YearOutOfRange);
        }
        return Err(CalendarError::DayOutOfRange);
    }
    Ok(rd)
}

/// The date of a fixed day.
///
/// # Errors
///
/// Returns the errors of [`era_at`].
pub fn from_fixed(rd: Rd) -> CalendarResult<KoreanRegnalDate> {
    from_fixed_under(Reading::Chosen, rd)
}

/// The date of a fixed day under `reading`.
///
/// # Errors
///
/// Returns the errors of [`era_under`].
pub fn from_fixed_under(reading: Reading, rd: Rd) -> CalendarResult<KoreanRegnalDate> {
    let era = era_under(reading, rd)?;
    let (year, month, day) = gregorian::from_fixed(rd)?;
    Ok(KoreanRegnalDate {
        era,
        year: year - era.start_year + 1,
        month,
        day,
    })
}

/// The one [`Calendar`] implementation of both readings.
macro_rules! korean_regnal_calendar {
    ($name:ident, $reading:expr, $id:literal, $english:literal) => {
        impl Calendar for $name {
            type Date = KoreanRegnalDate;

            /// From 1 January 1896 to 29 August 1910, which is also the whole of the
            /// range it converts.
            fn usage(&self) -> hc_calendar::Usage {
                hc_calendar::Usage::between(EARLIEST, LATEST, USAGE_SOURCE)
            }

            fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
                hc_calendar::shape::SOLAR_TWELVE
            }

            /// For a Gregorian year with no era, as [`Self::from_fields`] reads
            /// one, within the years the eras span.
            fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
                let (first, _, _) = gregorian::from_fixed(EARLIEST)?;
                let (last, _, _) = gregorian::from_fixed(LATEST)?;
                if year < first || year > last {
                    return Err(CalendarError::YearOutOfRange);
                }
                Ok(gregorian::is_leap_year(year))
            }

            /// Resolves the era first: the fields count years within it, and the
            /// leap rule counts Gregorian years.
            fn is_leap_year_of(&self, fields: &DateFields) -> CalendarResult<bool> {
                match fields.era {
                    Some(name) => {
                        let era = by_id(name).ok_or(CalendarError::UnknownEra)?;
                        self.is_leap_year(era.start_year + fields.year - 1)
                    }
                    None => self.is_leap_year(fields.year),
                }
            }

            /// The era in Hangul with its Revised Romanisation.
            fn era_name(&self, code: &str) -> Option<hc_calendar::EraName> {
                by_id(code).map(|era| hc_calendar::EraName::new(era.hangul, era.romanised))
            }

            /// The three eras, in order.
            fn era_code(&self, index: usize) -> Option<&'static str> {
                ALL.get(index).map(|era| era.id)
            }

            fn meta(&self) -> CalendarMeta {
                CalendarMeta {
                    id: CalendarId($id),
                    english_name: $english,
                    year_kind: YearKind::EraRelative,
                    has_leap_months: false,
                    is_astronomical: false,
                    earliest: Some(EARLIEST),
                    latest: Some(LATEST),
                    native_locales: &["ko"],
                }
            }

            fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
                to_fixed_under($reading, date)
            }

            fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
                from_fixed_under($reading, rd)
            }

            fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
                Ok(DateFields::ymd(date.year, date.month, date.day).with_era(date.era.id))
            }

            /// Reads era-tagged fields, or fields with no era as a Gregorian year,
            /// the extended-year reading [`crate::japanese`] uses too.
            fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
                let month = fields.require_month()?;
                if month.leap {
                    return Err(CalendarError::MonthOutOfRange);
                }
                let day = fields.require_day()?;
                match fields.era {
                    Some(name) => {
                        let era = by_id(name).ok_or(CalendarError::UnknownEra)?;
                        let date = KoreanRegnalDate {
                            era,
                            year: fields.year,
                            month: month.ordinal,
                            day,
                        };
                        to_fixed_under($reading, date)?;
                        Ok(date)
                    }
                    None => from_fixed_under(
                        $reading,
                        gregorian::to_fixed(fields.year, month.ordinal, day)?,
                    ),
                }
            }
        }
    };
}

korean_regnal_calendar!(
    KoreanRegnalCalendar,
    Reading::Chosen,
    "korean-regnal",
    "Korean Empire eras"
);
korean_regnal_calendar!(
    KoreanRegnalBackdatedCalendar,
    Reading::Backdated,
    "korean-regnal-backdated",
    "Korean Empire eras (光武 backdated to 1897)"
);

/// The month of a date as a [`Month`], for callers assembling fields.
#[must_use]
pub const fn month_of(date: KoreanRegnalDate) -> Month {
    Month::regular(date.month)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn greg(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("valid Gregorian date")
    }

    /// 建陽 from 1 January 1896 (`kowiki-geonyang`); 光武 from 14 August
    /// 1897 and 隆熙 from 2 August 1907, the days the annals record them
    /// chosen (`sillok-gojong`, `sillok-sunjong`).
    #[test]
    fn the_three_eras_begin_on_the_days_carried() {
        let cases = [
            ((1896, 1, 1), "geonyang", 1),
            ((1897, 8, 13), "geonyang", 2),
            ((1897, 8, 14), "gwangmu", 1),
            ((1897, 12, 31), "gwangmu", 1),
            ((1898, 1, 1), "gwangmu", 2),
            ((1907, 8, 1), "gwangmu", 11),
            ((1907, 8, 2), "yunghui", 1),
            ((1910, 8, 29), "yunghui", 4),
        ];
        for ((y, m, d), id, year) in cases {
            let date = from_fixed(greg(y, m, d)).expect("in range");
            assert_eq!(
                (date.era.id, date.year, date.month, date.day),
                (id, year, m, d),
                "{y}-{m}-{d}"
            );
            assert_eq!(to_fixed(date), Ok(greg(y, m, d)));
        }
        assert_eq!(
            from_fixed(greg(1897, 8, 14)).unwrap().to_string(),
            "光武 元年 8月 14日"
        );
        assert_eq!(
            from_fixed(greg(1910, 8, 29)).unwrap().to_string(),
            "隆熙 4年 8月 29日"
        );
    }

    /// 「以是年爲光武元年」 (`sillok-gojong`, 고종 34년 8월 15일): under the
    /// backdated reading all of 1897 is 光武 元年, and 建陽 is 1896 alone;
    /// 隆熙 keeps the day it was chosen (`sillok-sunjong`).
    #[test]
    fn the_backdated_reading_makes_all_of_1897_gwangmu_one() {
        let calendar = KoreanRegnalBackdatedCalendar;
        let cases = [
            ((1896, 12, 31), "geonyang", 1),
            ((1897, 1, 1), "gwangmu", 1),
            ((1897, 8, 13), "gwangmu", 1),
            ((1897, 8, 14), "gwangmu", 1),
            ((1907, 8, 1), "gwangmu", 11),
            ((1907, 8, 2), "yunghui", 1),
        ];
        for ((y, m, d), id, year) in cases {
            let date = calendar.from_fixed(greg(y, m, d)).expect("in range");
            assert_eq!((date.era.id, date.year), (id, year), "{y}-{m}-{d}");
            assert_eq!(calendar.to_fixed(date), Ok(greg(y, m, d)));
        }
        assert_eq!(
            calendar.from_fixed(greg(1897, 1, 1)).unwrap().to_string(),
            "光武 元年 1月 1日"
        );
        // 建陽 2年 has no day in this reading, and is 1897 until 13 August in
        // the other.
        let geonyang_two = KoreanRegnalDate {
            era: &GEONYANG,
            year: 2,
            month: 1,
            day: 1,
        };
        assert_eq!(
            calendar.to_fixed(geonyang_two),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            KoreanRegnalCalendar.to_fixed(geonyang_two),
            Ok(greg(1897, 1, 1))
        );
        assert_eq!(calendar.meta().id.as_str(), "korean-regnal-backdated");
        // The readings part on the 225 days of 1 January to 13 August 1897.
        let parted = (EARLIEST.0..=LATEST.0)
            .filter(|&rd| calendar.from_fixed(Rd(rd)) != KoreanRegnalCalendar.from_fixed(Rd(rd)))
            .count();
        assert_eq!(parted, 225);
        // Every day round-trips, in either build: the range is fourteen
        // years of Gregorian days.
        for rd in EARLIEST.0..=LATEST.0 {
            let date = calendar.from_fixed(Rd(rd)).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "rd {rd}");
            let fields = calendar.to_fields(date).expect("describable");
            assert_eq!(fields.era, Some(date.era.id));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
        }
    }

    #[test]
    fn the_calendar_runs_from_the_gregorian_adoption_to_the_annexation() {
        assert_eq!(era_at(Rd(EARLIEST.0 - 1)), Err(CalendarError::BeforeEpoch));
        assert_eq!(
            era_at(Rd(LATEST.0 + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        // 建陽 never had a year 3, and its year 2 ended on 13 August.
        let year_three = KoreanRegnalDate {
            era: &GEONYANG,
            year: 3,
            month: 1,
            day: 1,
        };
        assert_eq!(to_fixed(year_three), Err(CalendarError::YearOutOfRange));
        let after = KoreanRegnalDate {
            era: &GEONYANG,
            year: 2,
            month: 8,
            day: 14,
        };
        assert_eq!(to_fixed(after), Err(CalendarError::DayOutOfRange));
        let before = KoreanRegnalDate {
            era: &GWANGMU,
            year: 1,
            month: 8,
            day: 13,
        };
        assert_eq!(to_fixed(before), Err(CalendarError::DayOutOfRange));
        assert_eq!(gaeguk_year(1894), 503);
        assert_eq!(gaeguk_year(1392), 1);
    }

    #[test]
    fn every_day_round_trips_through_the_calendar_and_its_fields() {
        let calendar = KoreanRegnalCalendar;
        for rd in EARLIEST.0..=LATEST.0 {
            let date = calendar.from_fixed(Rd(rd)).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "rd {rd}");
            let fields = calendar.to_fields(date).expect("describable");
            assert_eq!(fields.era, Some(date.era.id));
            assert_eq!(calendar.from_fields(&fields), Ok(date), "rd {rd}");
        }
        // Fields without an era read the year as Gregorian.
        let plain = DateFields::ymd(1900, 6, 1);
        assert_eq!(calendar.from_fields(&plain), from_fixed(greg(1900, 6, 1)));
        assert_eq!(
            calendar.from_fields(&DateFields::ymd(1, 1, 1).with_era("meiji")),
            Err(CalendarError::UnknownEra)
        );
        assert_eq!(by_id("gwangmu"), Some(&GWANGMU));
        assert_eq!(by_id(" Yunghui "), Some(&YUNGHUI));
        assert_eq!(by_id("光武"), None);
        assert_eq!(by_id("융희"), None);
        assert_eq!(month_of(from_fixed(EARLIEST).unwrap()), Month::regular(1));
    }
}
