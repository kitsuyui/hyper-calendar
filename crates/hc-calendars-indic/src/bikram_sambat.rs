//! The Bikram Sambat, the solar calendar of Nepal: Baisakh to Chait from
//! the Meṣa saṅkrānti, years in the Vikrama era — `bikram-sambat`.
//!
//! The system is written up in `docs/systems/nepal-calendars.md` in the
//! repository: the calendar's standing, how the months are read from
//! the holiday notices' Saturdays, why the modern Sun with an hour-of-day
//! rule was rejected and the *Sūrya Siddhānta*'s Sun on its civil day
//! kept, Pus and Magh 2082 worked through by hand, what is carried and
//! what is not, the checks against the notices, and the sources, keyed in
//! `docs/references.bib`. This page summarises it and states the code's
//! own facts.
//!
//! A month is the Sun's stay in a sidereal sign, 29 to 32 days, and its
//! first day is the one the Government of Nepal publishes: the Ministry
//! of Home Affairs fixes each coming year's public holidays in a notice in
//! the *Nepal Rajpatra*, Part 5, whose first section lists every Saturday
//! of the year by its Bikram Sambat date, and the months' lengths follow
//! from the Saturdays. [`GAZETTED`] is the first day of every month the
//! four notices read fix, 48 for 2080–2083 BS:
//!
//! | Year | *Rajpatra* | Dated |
//! | --- | --- | --- |
//! | 2080 | Khaṇḍa 72, No. 65, Part 5 | 2079-12-02 |
//! | 2081 | Khaṇḍa 73, No. 54, Part 5 | 2080-10-29 |
//! | 2082 | Khaṇḍa 74, No. 59, Part 5 | 2081-11-15 |
//! | 2083 | Khaṇḍa 75, No. 67, Part 5 | 2082-11-18 |
//!
//! Outside them [`RECKONING`] computes the months: the *Sūrya Siddhānta*'s
//! Sun ([`crate::surya_siddhanta`]), each month beginning on the civil
//! day, midnight to midnight at Kathmandu, in which its saṅkrānti falls
//! ([`CivilDay`](crate::SankrantiRule::CivilDay)). That rule begins 47 of
//! the 48 gazetted months on the gazette's day and misses Magh 2082 by a
//! day — the saṅkrānti at 21:10 Nepal time on 14 January 2026, the
//! gazette beginning the month on the 15th — where no rule with the
//! modern Sun fits at all. [`BikramSambatCalendar`] takes a month's first
//! day from [`GAZETTED`] where it is there and from the reckoning where
//! it is not, so a computed month is to be read with that one miss in
//! mind. The civil day is Kathmandu's local mean time, 5 h 41 min ahead
//! of Universal Time where Nepal Standard Time is 5 h 45 min; no
//! saṅkrānti of the four years falls within those four minutes.
//!
//! # The names
//!
//! The months in Devanagari are the gazette's spellings. The English
//! names are the Nepali forms that Wikipedia's "Vikram Samvat" (retrieved
//! 2026-09-23) lists beside the Sanskrit ones; for फागुन it gives Falgun.
//!
//! Sources: the four notices above, published by the Ministry of Home
//! Affairs at moha.gov.np under "Public Holidays for 2080" and
//! "Government and Public Holidays in 2081", "… 2082" and "… 2083",
//! retrieved 2026-09-23; Wikipedia, "Vikram Samvat", retrieved
//! 2026-09-23, for the English month names.

use hc_calendar::shape::{CycleShape, MONTH, WEEKDAY};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};
use hc_calendars_solar::gregorian;
use hc_seasons::zodiac::rashi;

use crate::hindu_solar::{HinduSolarCalendar, HinduSolarDate, SankrantiRule, SolarModel};
use crate::places::KATHMANDU;

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "The official calendar of Nepal since 1901, 1958 VS [wikipedia-vikram-samvat], or 1903 \
    [wikipedia-nepal-sambat], the two sources disagreeing by two years and neither giving a \
    day; the era itself older than any source read dates; the gazetted months of 2080–2083 \
    BS from the Ministry of Home Affairs notices";

/// The calendar's identifier.
pub const ID: CalendarId = CalendarId("bikram-sambat");
/// The era it counts in.
pub const ERA: &str = "Bikram Sambat";

/// The twelve months as the gazette spells them, Baisakh first.
pub const MONTHS_DEVANAGARI: [&str; 12] = [
    "वैशाख",
    "जेठ",
    "असार",
    "साउन",
    "भदौ",
    "असोज",
    "कात्तिक",
    "मङ्सिर",
    "पुस",
    "माघ",
    "फागुन",
    "चैत",
];

/// The twelve months in English, Baisakh first.
pub const MONTHS: [&str; 12] = [
    "Baisakh", "Jeth", "Asar", "Saaun", "Bhadau", "Aasoj", "Kattik", "Mangsir", "Push", "Maagh",
    "Falgun", "Chait",
];

/// The computed months: the *Sūrya Siddhānta* Sun, each month beginning on
/// the civil day of its saṅkrānti at Kathmandu, years in the Vikrama era
/// (Gregorian year plus 57).
pub const RECKONING: HinduSolarCalendar = HinduSolarCalendar {
    id: ID,
    english_name: "Bikram Sambat",
    native_locales: &["ne"],
    tradition: rashi::VIKRAMI,
    rule: SankrantiRule::CivilDay,
    era: ERA,
    era_offset: 57,
    location: KATHMANDU,
    model: SolarModel::SuryaSiddhanta,
};

/// The first day of a month, as a notice in the gazette fixes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GazettedMonth {
    /// The Bikram Sambat year.
    pub year: i64,
    /// The month, 1 for Baisakh through 12 for Chait.
    pub month: u8,
    /// Its first day in the Gregorian calendar: year, month, day.
    pub first_day: (i64, u8, u8),
}

/// A row of [`GAZETTED`].
const fn gazetted(year: i64, month: u8, g_year: i64, g_month: u8, g_day: u8) -> GazettedMonth {
    GazettedMonth {
        year,
        month,
        first_day: (g_year, g_month, g_day),
    }
}

/// The first day of every month the notices fix, in order.
pub const GAZETTED: &[GazettedMonth] = &[
    // Bikram Sambat 2080
    gazetted(2080, 1, 2023, 4, 14),
    gazetted(2080, 2, 2023, 5, 15),
    gazetted(2080, 3, 2023, 6, 16),
    gazetted(2080, 4, 2023, 7, 17),
    gazetted(2080, 5, 2023, 8, 18),
    gazetted(2080, 6, 2023, 9, 18),
    gazetted(2080, 7, 2023, 10, 18),
    gazetted(2080, 8, 2023, 11, 17),
    gazetted(2080, 9, 2023, 12, 17),
    gazetted(2080, 10, 2024, 1, 15),
    gazetted(2080, 11, 2024, 2, 13),
    gazetted(2080, 12, 2024, 3, 14),
    // Bikram Sambat 2081
    gazetted(2081, 1, 2024, 4, 13),
    gazetted(2081, 2, 2024, 5, 14),
    gazetted(2081, 3, 2024, 6, 15),
    gazetted(2081, 4, 2024, 7, 16),
    gazetted(2081, 5, 2024, 8, 17),
    gazetted(2081, 6, 2024, 9, 17),
    gazetted(2081, 7, 2024, 10, 17),
    gazetted(2081, 8, 2024, 11, 16),
    gazetted(2081, 9, 2024, 12, 16),
    gazetted(2081, 10, 2025, 1, 14),
    gazetted(2081, 11, 2025, 2, 13),
    gazetted(2081, 12, 2025, 3, 14),
    // Bikram Sambat 2082
    gazetted(2082, 1, 2025, 4, 14),
    gazetted(2082, 2, 2025, 5, 15),
    gazetted(2082, 3, 2025, 6, 15),
    gazetted(2082, 4, 2025, 7, 17),
    gazetted(2082, 5, 2025, 8, 17),
    gazetted(2082, 6, 2025, 9, 17),
    gazetted(2082, 7, 2025, 10, 18),
    gazetted(2082, 8, 2025, 11, 17),
    gazetted(2082, 9, 2025, 12, 16),
    gazetted(2082, 10, 2026, 1, 15),
    gazetted(2082, 11, 2026, 2, 13),
    gazetted(2082, 12, 2026, 3, 15),
    // Bikram Sambat 2083
    gazetted(2083, 1, 2026, 4, 14),
    gazetted(2083, 2, 2026, 5, 15),
    gazetted(2083, 3, 2026, 6, 15),
    gazetted(2083, 4, 2026, 7, 17),
    gazetted(2083, 5, 2026, 8, 17),
    gazetted(2083, 6, 2026, 9, 17),
    gazetted(2083, 7, 2026, 10, 18),
    gazetted(2083, 8, 2026, 11, 17),
    gazetted(2083, 9, 2026, 12, 16),
    gazetted(2083, 10, 2027, 1, 15),
    gazetted(2083, 11, 2027, 2, 13),
    gazetted(2083, 12, 2027, 3, 15),
];

/// A date in the Bikram Sambat.
pub type BikramSambatDate = HinduSolarDate;

/// The Bikram Sambat: the gazetted months where there are any, the
/// reckoning elsewhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BikramSambatCalendar;

impl BikramSambatCalendar {
    /// The gazetted first day of a month, if a notice fixes it.
    fn gazetted_start(year: i64, month: u8) -> Option<Rd> {
        GAZETTED
            .iter()
            .find(|row| row.year == year && row.month == month)
            .and_then(|row| {
                let (y, m, d) = row.first_day;
                gregorian::to_fixed(y, m, d).ok()
            })
    }

    /// The first day of a month, without validation; `month` may be 13,
    /// for the first day of the next year.
    fn month_start_raw(year: i64, month: u8) -> Rd {
        if month > 12 {
            return Self::month_start_raw(year + 1, 1);
        }
        Self::gazetted_start(year, month).unwrap_or_else(|| RECKONING.month_start_raw(year, month))
    }

    /// The first day of a month.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] or
    /// [`CalendarError::MonthOutOfRange`].
    pub fn month_start(&self, year: i64, month: u8) -> CalendarResult<Rd> {
        RECKONING.month_start(year, month)?;
        Ok(Self::month_start_raw(year, month))
    }

    /// The number of days in a month, 29 to 32.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] or
    /// [`CalendarError::MonthOutOfRange`].
    pub fn days_in_month(&self, year: i64, month: u8) -> CalendarResult<u8> {
        let start = self.month_start(year, month)?;
        Ok((Self::month_start_raw(year, month + 1).0 - start.0) as u8)
    }

    /// The number of days in a year, 365 or 366.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`].
    pub fn days_in_year(&self, year: i64) -> CalendarResult<u16> {
        let start = self.month_start(year, 1)?;
        Ok((Self::month_start_raw(year + 1, 1).0 - start.0) as u16)
    }

    /// The fixed day of a date.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`],
    /// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
    pub fn to_fixed(&self, date: BikramSambatDate) -> CalendarResult<Rd> {
        let start = self.month_start(date.year, date.month)?;
        let length = self.days_in_month(date.year, date.month)?;
        if date.day == 0 || date.day > length {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(Rd(start.0 + i64::from(date.day) - 1))
    }

    /// The date of a fixed day.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::BeforeEpoch`] or
    /// [`CalendarError::AfterSupportedRange`] outside the reckoning's range.
    pub fn from_fixed(&self, rd: Rd) -> CalendarResult<BikramSambatDate> {
        // Start from the reckoning's month and move to the neighbour where
        // a gazetted first day puts the day in it instead: the two never
        // differ by more than a day, so the month is this one or the next
        // or the last.
        let near = RECKONING.from_fixed(rd)?;
        let (mut year, mut month) = (near.year, near.month);
        if Self::month_start_raw(year, month) > rd {
            (year, month) = if month == 1 {
                (year - 1, 12)
            } else {
                (year, month - 1)
            };
        } else if Self::month_start_raw(year, month + 1) <= rd {
            (year, month) = if month == 12 {
                (year + 1, 1)
            } else {
                (year, month + 1)
            };
        }
        let start = Self::month_start_raw(year, month);
        Ok(HinduSolarDate {
            year,
            month,
            day: (rd.0 - start.0 + 1) as u8,
        })
    }

    /// The earliest fixed day this calendar converts, the reckoning's.
    #[must_use]
    pub fn earliest(&self) -> Rd {
        RECKONING.earliest()
    }

    /// The latest fixed day this calendar converts, the reckoning's.
    #[must_use]
    pub fn latest(&self) -> Rd {
        RECKONING.latest()
    }
}

impl Calendar for BikramSambatCalendar {
    type Date = BikramSambatDate;

    /// Nepal's official calendar today; official since 1901 or 1903, the
    /// sources disagreeing, and older than either, so undated at the start.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::undated(USAGE_SOURCE)
    }

    /// Twelve named months and the seven-day week.
    fn cycles(&self) -> &'static [CycleShape] {
        const SHAPE: &[CycleShape] = &[
            CycleShape::named(MONTH, &MONTHS),
            CycleShape::fixed(WEEKDAY, 7),
        ];
        SHAPE
    }

    /// Never: the year is the Sun's passage through the twelve signs, 365
    /// or 366 days as the saṅkrāntis fall, with nothing inserted.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(RECKONING.min_year()..=RECKONING.max_year()).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(false)
    }

    /// The civil day, midnight to midnight: the day a month begins on is
    /// the civil day of its saṅkrānti.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Midnight
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Bikram Sambat",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: true,
            earliest: Some(self.earliest()),
            latest: Some(self.latest()),
            native_locales: &["ne"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        BikramSambatCalendar::to_fixed(self, date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        BikramSambatCalendar::from_fixed(self, rd)
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        Ok(DateFields::ymd(date.year, date.month, date.day).with_era(ERA))
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        if month.leap {
            return Err(CalendarError::MonthOutOfRange);
        }
        let date = HinduSolarDate {
            year: fields.year,
            month: month.ordinal,
            day: fields.require_day()?,
        };
        BikramSambatCalendar::to_fixed(self, date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BS: BikramSambatCalendar = BikramSambatCalendar;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("a date")
    }

    /// Section 1 of each notice, "वर्षभरिका शनिबार बिदा", the year's
    /// Saturdays: for each month, Baisakh first, the days that are
    /// Saturdays.
    const SATURDAYS: [(i64, [&[u8]; 12]); 4] = [
        (
            2080,
            [
                &[2, 9, 16, 23, 30],
                &[6, 13, 20, 27],
                &[2, 9, 16, 23, 30],
                &[6, 13, 20, 27],
                &[2, 9, 16, 23, 30],
                &[6, 13, 20, 27],
                &[4, 11, 18, 25],
                &[2, 9, 16, 23, 30],
                &[7, 14, 21, 28],
                &[6, 13, 20, 27],
                &[5, 12, 19, 26],
                &[3, 10, 17, 24],
            ],
        ),
        (
            2081,
            [
                &[1, 8, 15, 22, 29],
                &[5, 12, 19, 26],
                &[1, 8, 15, 22, 29],
                &[5, 12, 19, 26],
                &[1, 8, 15, 22, 29],
                &[5, 12, 19, 26],
                &[3, 10, 17, 24],
                &[1, 8, 15, 22, 29],
                &[6, 13, 20, 27],
                &[5, 12, 19, 26],
                &[3, 10, 17, 24],
                &[2, 9, 16, 23, 30],
            ],
        ),
        (
            2082,
            [
                &[6, 13, 20, 27],
                &[3, 10, 17, 24, 31],
                &[7, 14, 21, 28],
                &[3, 10, 17, 24, 31],
                &[7, 14, 21, 28],
                &[4, 11, 18, 25],
                &[1, 8, 15, 22, 29],
                &[6, 13, 20, 27],
                &[5, 12, 19, 26],
                &[3, 10, 17, 24],
                &[2, 9, 16, 23, 30],
                &[7, 14, 21, 28],
            ],
        ),
        (
            2083,
            [
                &[5, 12, 19, 26],
                &[2, 9, 16, 23, 30],
                &[6, 13, 20, 27],
                &[2, 9, 16, 23, 30],
                &[6, 13, 20, 27],
                &[3, 10, 17, 24, 31],
                &[7, 14, 21, 28],
                &[5, 12, 19, 26],
                &[4, 11, 18, 25],
                &[2, 9, 16, 23],
                &[1, 8, 15, 22, 29],
                &[6, 13, 20, 27],
            ],
        ),
    ];

    /// Whether a fixed day is a Saturday: `Rd(1)`, 1 January of year 1, was
    /// a Monday.
    fn is_saturday(rd: Rd) -> bool {
        rd.0.rem_euclid(7) == 6
    }

    #[test]
    fn the_saturdays_are_the_ones_the_notices_list() {
        for (year, months) in SATURDAYS {
            for (index, listed) in months.iter().enumerate() {
                let month = index as u8 + 1;
                let length = BS.days_in_month(year, month).unwrap();
                let saturdays: Vec<u8> = (1..=length)
                    .filter(|&day| {
                        is_saturday(BS.to_fixed(HinduSolarDate { year, month, day }).unwrap())
                    })
                    .collect();
                assert_eq!(&saturdays[..], *listed, "{year}-{month}");
            }
        }
    }

    #[test]
    fn the_new_years_days_fall_on_the_weekdays_the_notices_give() {
        // Item 2.1 (क), "नव वर्ष - वैशाख १ गते": a Friday in 2080, a
        // Saturday in 2081, a Monday in 2082, a Tuesday in 2083.
        for (year, gregorian_day) in [
            (2080, ymd(2023, 4, 14)),
            (2081, ymd(2024, 4, 13)),
            (2082, ymd(2025, 4, 14)),
            (2083, ymd(2026, 4, 14)),
        ] {
            assert_eq!(BS.month_start(year, 1).unwrap(), gregorian_day);
        }
        assert_eq!(ymd(2023, 4, 14).0.rem_euclid(7), 5);
        assert_eq!(ymd(2024, 4, 13).0.rem_euclid(7), 6);
        assert_eq!(ymd(2025, 4, 14).0.rem_euclid(7), 1);
        assert_eq!(ymd(2026, 4, 14).0.rem_euclid(7), 2);
    }

    #[test]
    fn christmas_2025_is_pus_10() {
        // The 2082 notice: "क्रिसमस डे (डिसेम्बर २५)- पुस १० गते बिहीबार".
        let christmas = ymd(2025, 12, 25);
        assert_eq!(
            BS.from_fixed(christmas).unwrap(),
            HinduSolarDate {
                year: 2082,
                month: 9,
                day: 10
            }
        );
        assert_eq!(christmas.0.rem_euclid(7), 4, "a Thursday");
    }

    #[test]
    fn the_reckoning_misses_the_gazette_once_in_forty_eight_months() {
        let missed: Vec<(i64, u8, i64)> = GAZETTED
            .iter()
            .filter_map(|row| {
                let gazette = BikramSambatCalendar::gazetted_start(row.year, row.month).unwrap();
                let reckoned = RECKONING.month_start(row.year, row.month).unwrap();
                (gazette != reckoned).then_some((row.year, row.month, gazette.0 - reckoned.0))
            })
            .collect();
        assert_eq!(missed, [(2082, 10, 1)]);
    }

    #[test]
    fn a_day_the_reckoning_puts_in_magh_is_pus_30_by_the_gazette() {
        let day = ymd(2026, 1, 14);
        assert_eq!(
            RECKONING.from_fixed(day).unwrap(),
            HinduSolarDate {
                year: 2082,
                month: 10,
                day: 1
            }
        );
        assert_eq!(
            BS.from_fixed(day).unwrap(),
            HinduSolarDate {
                year: 2082,
                month: 9,
                day: 30
            }
        );
    }

    #[test]
    fn every_day_round_trips_across_the_gazetted_years_and_beyond() {
        for rd in ymd(2020, 1, 1).0..=ymd(2030, 12, 31).0 {
            let date = BS.from_fixed(Rd(rd)).unwrap();
            assert_eq!(BS.to_fixed(date).unwrap(), Rd(rd), "{date:?}");
        }
    }

    #[test]
    fn months_run_twenty_nine_to_thirty_two_days_and_years_365_or_366() {
        for year in 2000..2100 {
            let mut total = 0;
            for month in 1..=12 {
                let length = BS.days_in_month(year, month).unwrap();
                assert!((29..=32).contains(&length), "{year}-{month}: {length}");
                total += u16::from(length);
            }
            assert_eq!(total, BS.days_in_year(year).unwrap());
            assert!(matches!(total, 365 | 366), "{year}: {total}");
        }
    }

    #[test]
    fn the_fields_carry_the_era_and_refuse_a_leap_month() {
        let date = HinduSolarDate {
            year: 2083,
            month: 6,
            day: 7,
        };
        let fields = BS.to_fields(date).unwrap();
        assert_eq!(fields.era, Some(ERA));
        assert_eq!(BS.from_fields(&fields).unwrap(), date);
        let mut leap = fields;
        leap.month = Some(hc_calendar::Month::leap(6));
        assert_eq!(BS.from_fields(&leap), Err(CalendarError::MonthOutOfRange));
    }

    #[test]
    fn the_range_is_the_reckonings() {
        assert_eq!(BS.earliest(), RECKONING.earliest());
        assert!(BS.from_fixed(Rd(BS.earliest().0 - 1)).is_err());
        assert!(BS.from_fixed(Rd(BS.latest().0 + 1)).is_err());
        assert!(BS.from_fixed(BS.latest()).is_ok());
    }
}
