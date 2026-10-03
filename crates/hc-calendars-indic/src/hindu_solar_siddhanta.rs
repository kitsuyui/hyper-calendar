//! The Hindu solar calendar on the *Sūrya Siddhānta*'s Sun, read at its
//! sunrise at Ujjain — `hindu-solar-surya-siddhanta`.
//!
//! A month is the Siddhānta's Sun's stay in one of its twelve sidereal
//! signs, Meṣa first, and it begins on the day whose closing sunrise is the
//! first to see the Sun in the new sign: the sunrise-to-sunrise day the
//! saṅkrānti falls in, named by its date at sunrise, which is Sewell and
//! Dikshit's Orissa rule and the Vikrami reckoning's
//! ([`crate::hindu_solar::SankrantiRule::SunriseDay`]). The sunrise is the
//! Siddhānta's own ([`crate::surya_siddhanta::sunrise`]), not the true
//! one, and the year is the Siddhānta's Kali Yuga solar year less 3179, the
//! Śaka year. It is the modern Hindu solar calendar of Reingold and
//! Dershowitz, *Calendrical Calculations*, `hindu-solar-from-fixed` and
//! `fixed-from-hindu-solar` in their published code, which their comment
//! calls the "Orissa rule" (`reingold2018code`, read 2026-09-27). The
//! system, its worked example and the measurements are in
//! `docs/systems/hindu-calendars.md` in the repository.
//!
//! It differs from [`crate::hindu_solar::VIKRAMI`] rebuilt at Ujjain with
//! [`crate::hindu_solar::SolarModel::SuryaSiddhanta`] in three ways, and
//! each is the book's: the day is closed by the Siddhānta's sunrise and not
//! the true one, the months are named by their signs and not by the
//! Vikrami names, and the year is the Siddhānta's own, not the Gregorian
//! year of the Meṣa saṅkrānti less 78.
//!
//! No almanac's table of this calendar was read; it is held to the book's
//! 33 sample dates, as `hindu-lunar-surya-siddhanta` is, and whether any
//! almanac keeps its months to the day is not known here.
//!
//! # Range
//!
//! Kali Yuga years 1 to 10 000 — Śaka −3178 to 6821 — as for
//! `hindu-lunar-surya-siddhanta`: the reckoning is arithmetic, so the range
//! is the library's choice, and the dates it gives before the text existed
//! are the rule's, proleptically, and nobody's record.

use hc_astro::riseset::Location;
use hc_calendar::fixed::Moment;
use hc_calendar::shape::{CycleShape, MONTH, WEEKDAY};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind,
};
use hc_core::math::{floor, round};

use crate::hindu_lunar::ERA;
use crate::hindu_lunar_siddhanta::{EPOCH, KALI_SAKA_OFFSET, MAX_YEAR, MIN_YEAR};
use crate::hindu_old::{HINDU_EPOCH, SOLAR_MONTHS};
use crate::hindu_solar::{HinduSolarDate, MONTHS_IN_YEAR};
use crate::places::UJJAIN;
use crate::surya_siddhanta::{self, SIDEREAL_YEAR};

/// The identifier of the solar calendar on the *Sūrya Siddhānta*.
pub const ID: CalendarId = CalendarId("hindu-solar-surya-siddhanta");

/// How many days a month's first day is searched for past the book's
/// starting point, three days before the mean month begins: the true
/// saṅkrānti is never more than two and a quarter days from the mean one,
/// so the answer is within the first week, and a search that runs out is a
/// failure of the model, not a date.
const SEARCH_DAYS: i64 = 40;

/// The Hindu solar calendar on the *Sūrya Siddhānta*'s Sun, a month
/// beginning on the day whose closing sunrise at a place, by the Siddhānta,
/// is the first in the new sign.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SiddhantaSolarCalendar {
    /// Whose sunrise closes the day.
    pub location: Location,
}

/// The earliest and latest days of the registered calendar, as
/// [`SiddhantaSolarCalendar::earliest`] and
/// [`SiddhantaSolarCalendar::latest`] compute them;
/// `tests::the_named_range_is_the_computed_one` computes it again.
const NAMED_RANGE: (SiddhantaSolarCalendar, Rd, Rd) = (
    SiddhantaSolarCalendar::UJJAIN,
    Rd(-1_132_597),
    Rd(2_519_990),
);

impl SiddhantaSolarCalendar {
    /// The calendar read at Ujjain, as Reingold and Dershowitz read it: the
    /// registered `hindu-solar-surya-siddhanta`.
    pub const UJJAIN: Self = Self::new(UJJAIN);

    /// The calendar read at another place's Siddhāntic sunrise.
    #[must_use]
    pub const fn new(location: Location) -> Self {
        Self { location }
    }

    /// The sunrise that closes a day: the Siddhānta's sunrise of the day
    /// after, the moment the book reads a date at ("Sunrise on Hindu
    /// date", `hindu-solar-from-fixed`).
    fn closing_sunrise(&self, day: i64) -> Moment {
        surya_siddhanta::sunrise(Rd(day + 1), self.location)
    }

    /// The sign, 1 for Meṣa through 12 for Mīna, the Sun stands in at the
    /// sunrise that closes a day (`hindu-zodiac`).
    fn month_at(&self, day: i64) -> u8 {
        surya_siddhanta::sign_at(self.closing_sunrise(day)).index() + 1
    }

    /// The first day on or after `from`, within [`SEARCH_DAYS`], whose
    /// closing sunrise sees the Sun in `month`'s sign.
    fn first_day_of(&self, month: u8, from: i64) -> CalendarResult<i64> {
        (from..from + SEARCH_DAYS)
            .find(|&day| self.month_at(day) == month)
            .ok_or(CalendarError::AstronomicalModelFailure)
    }

    /// The first day of `month` of the Śaka `year`, without range checks:
    /// from three days before the mean month begins, the first day whose
    /// closing sunrise sees the month's sign (`fixed-from-hindu-solar`).
    fn month_start_raw(&self, year: i64, month: u8) -> CalendarResult<i64> {
        let years = (year + KALI_SAKA_OFFSET) as f64 + f64::from(month - 1) / 12.0;
        let mean = floor(years * SIDEREAL_YEAR) as i64 + HINDU_EPOCH.0;
        self.first_day_of(month, mean - 3)
    }

    /// The first day of a month.
    ///
    /// # Errors
    ///
    /// [`CalendarError::YearOutOfRange`] outside Śaka [`MIN_YEAR`] to
    /// [`MAX_YEAR`] and [`CalendarError::MonthOutOfRange`] for a month
    /// outside 1 to 12.
    pub fn month_start(&self, year: i64, month: u8) -> CalendarResult<Rd> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        if month == 0 || month > MONTHS_IN_YEAR {
            return Err(CalendarError::MonthOutOfRange);
        }
        self.month_start_raw(year, month).map(Rd)
    }

    /// The number of days in a month, 29 to 32.
    ///
    /// # Errors
    ///
    /// As [`SiddhantaSolarCalendar::month_start`].
    pub fn days_in_month(&self, year: i64, month: u8) -> CalendarResult<u8> {
        let start = self.month_start(year, month)?;
        let next = if month == MONTHS_IN_YEAR {
            self.month_start_raw(year + 1, 1)?
        } else {
            self.month_start_raw(year, month + 1)?
        };
        Ok((next - start.0) as u8)
    }

    /// The fixed day of a date.
    ///
    /// # Errors
    ///
    /// As [`SiddhantaSolarCalendar::month_start`], and
    /// [`CalendarError::DayOutOfRange`] for a day past the month's end.
    pub fn to_fixed(&self, date: HinduSolarDate) -> CalendarResult<Rd> {
        let start = self.month_start(date.year, date.month)?;
        let length = self.days_in_month(date.year, date.month)?;
        if date.day == 0 || date.day > length {
            return Err(CalendarError::DayOutOfRange);
        }
        Ok(Rd(start.0 + i64::from(date.day) - 1))
    }

    /// The earliest fixed day this calendar converts: Meṣa 1 of Kali Yuga 1.
    #[must_use]
    pub fn earliest(&self) -> Rd {
        self.named_range().map_or_else(
            || Rd(self.month_start_raw(MIN_YEAR, 1).unwrap_or(i64::MAX)),
            |(earliest, _)| earliest,
        )
    }

    /// The latest fixed day this calendar converts: the eve of Meṣa 1 of
    /// Kali Yuga 10 001.
    #[must_use]
    pub fn latest(&self) -> Rd {
        self.named_range().map_or_else(
            || {
                Rd(self
                    .month_start_raw(MAX_YEAR + 1, 1)
                    .map_or(i64::MIN, |day| day - 1))
            },
            |(_, latest)| latest,
        )
    }

    /// The range [`NAMED_RANGE`] holds, for the registered calendar.
    fn named_range(&self) -> Option<(Rd, Rd)> {
        let (calendar, earliest, latest) = NAMED_RANGE;
        (calendar == *self).then_some((earliest, latest))
    }

    /// The date of a fixed day (`hindu-solar-from-fixed`): the month is the
    /// sign at the day's closing sunrise, the year the Siddhānta's solar
    /// year then (`hindu-calendar-year`) less 3179, and the day counts from
    /// the first day of the month, searched for from three days before the
    /// day the Sun's degree within the sign says the month began.
    ///
    /// # Errors
    ///
    /// [`CalendarError::BeforeEpoch`] or
    /// [`CalendarError::AfterSupportedRange`] outside
    /// [`earliest`](Self::earliest)..=[`latest`](Self::latest).
    pub fn from_fixed(&self, rd: Rd) -> CalendarResult<HinduSolarDate> {
        if rd < self.earliest() {
            return Err(CalendarError::BeforeEpoch);
        }
        if rd > self.latest() {
            return Err(CalendarError::AfterSupportedRange);
        }
        let critical = self.closing_sunrise(rd.0);
        let longitude = surya_siddhanta::solar_longitude(critical);
        let month = surya_siddhanta::sign_at(critical).index() + 1;
        let years = (critical.0 - EPOCH) / SIDEREAL_YEAR - longitude / 360.0;
        let year = round(years) as i64 - KALI_SAKA_OFFSET;
        let degrees_in = floor(longitude) as i64 % 30;
        let start = self.first_day_of(month, rd.0 - 3 - degrees_in)?;
        Ok(HinduSolarDate {
            year,
            month,
            day: (rd.0 - start + 1) as u8,
        })
    }
}

impl Calendar for SiddhantaSolarCalendar {
    type Date = HinduSolarDate;

    /// Unrecorded, as for `hindu-lunar-surya-siddhanta`: no source read
    /// dates the use of this reckoning, with this sunrise.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// Twelve months named for the signs, Meṣa first, and the seven-day
    /// week.
    fn cycles(&self) -> &'static [CycleShape] {
        const SHAPE: &[CycleShape] = &[
            CycleShape::named(MONTH, &SOLAR_MONTHS),
            CycleShape::fixed(WEEKDAY, 7),
        ];
        SHAPE
    }

    /// Never: a solar year is the Sun's passage through the twelve signs.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(false)
    }

    /// The day begins at the Siddhānta's sunrise and is named by the civil
    /// day on whose sunrise it begins.
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunrise(hc_calendar::DayNaming::ByStart)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Hindu solar (Surya Siddhanta)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: true,
            earliest: Some(self.earliest()),
            latest: Some(self.latest()),
            native_locales: &["sa"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        SiddhantaSolarCalendar::to_fixed(self, date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        SiddhantaSolarCalendar::from_fixed(self, rd)
    }

    /// The Śaka year, the month from Meṣa and the day.
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
        SiddhantaSolarCalendar::to_fixed(self, date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use hc_calendars_solar::gregorian;
    use hc_seasons::zodiac::SiderealSign;

    use super::*;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("a Gregorian date")
    }

    /// Against the same months closed by the true sunrise at Ujjain —
    /// [`crate::hindu_solar::VIKRAMI`] rebuilt there on the Siddhānta's Sun,
    /// in Vikrama years, 135 more — the Siddhānta's sunrise moves 2 of the
    /// 372 month starts of 2000–2030: the Vṛścika saṅkrānti of 17 November
    /// 2022 at 01:22.5 UT and the Dhanus saṅkrānti of 16 December 2024 at
    /// 01:34.8 UT fall after the true sunrise, 01:11.6 and 01:31.0, and
    /// before the Siddhānta's, 01:24.9 and 01:38.6, so this calendar begins
    /// each month a day earlier. Every day of the range in a release build;
    /// the two months in a debug one.
    #[test]
    fn the_siddhantas_sunrise_moves_two_month_starts_of_thirty_one_years() {
        use crate::hindu_solar::{SolarModel, VIKRAMI};
        let calendar = SiddhantaSolarCalendar::UJJAIN;
        let closed_by_the_true_sunrise = VIKRAMI.new(
            CalendarId("x-hindu-solar-vikrami-ujjain-siddhanta"),
            UJJAIN,
            SolarModel::SuryaSiddhanta,
        );
        for (day, month) in [(ymd(2022, 11, 16), 8), (ymd(2024, 12, 15), 9)] {
            let ours = calendar.from_fixed(day).expect("in range");
            assert_eq!((ours.month, ours.day), (month, 1));
            let other = closed_by_the_true_sunrise
                .from_fixed(day)
                .expect("in range");
            assert_eq!(other.month, month - 1);
            let next = closed_by_the_true_sunrise
                .from_fixed(Rd(day.0 + 1))
                .expect("in range");
            assert_eq!((next.month, next.day), (month, 1));
        }
        if cfg!(debug_assertions) {
            return;
        }
        let (mut days, mut starts, mut moved) = (0, 0, 0);
        for day in ymd(2000, 1, 1).0..=ymd(2030, 12, 31).0 {
            let ours = calendar.from_fixed(Rd(day)).expect("in range");
            let other = closed_by_the_true_sunrise
                .from_fixed(Rd(day))
                .expect("in range");
            days += u32::from(
                (ours.year, ours.month, ours.day) != (other.year - 135, other.month, other.day),
            );
            if ours.day == 1 {
                starts += 1;
                moved += u32::from(other.day != 1);
            }
        }
        assert_eq!((moved, starts, days), (2, 372, 60));
    }

    #[test]
    fn the_named_range_is_the_computed_one() {
        let calendar = SiddhantaSolarCalendar::UJJAIN;
        assert_eq!(calendar.month_start_raw(MIN_YEAR, 1), Ok(NAMED_RANGE.1.0));
        assert_eq!(
            calendar.month_start_raw(MAX_YEAR + 1, 1).map(|day| day - 1),
            Ok(NAMED_RANGE.2.0)
        );
    }

    /// The 33 sample dates of *Calendrical Calculations*, 586 BCE to 2094,
    /// as its published code's `hindu-solar-from-fixed` gives them: the
    /// R.D., the Śaka year, the month from Meṣa and the day
    /// (`reingold2018code`, `dates.l`, run for this library on 2026-09-27;
    /// the table is
    /// `crates/hyper-calendar/tests/data/calendrica_sample_dates.txt`).
    #[rustfmt::skip]
    const SAMPLE_DATES: [(i64, i64, u8, u8); 33] = [
        (-214_193, -664, 5, 19),
        (-61_387, -246, 9, 26),
        (25_469, -8, 7, 9),
        (49_217, 57, 7, 16),
        (171_307, 391, 10, 21),
        (210_155, 498, 2, 31),
        (253_427, 616, 8, 16),
        (369_740, 935, 1, 28),
        (400_085, 1_018, 2, 26),
        (434_355, 1_111, 12, 23),
        (452_605, 1_161, 12, 10),
        (470_160, 1_210, 1, 2),
        (473_837, 1_220, 1, 27),
        (507_850, 1_313, 3, 8),
        (524_156, 1_357, 10, 30),
        (544_676, 1_414, 1, 5),
        (567_118, 1_475, 6, 10),
        (569_477, 1_481, 11, 29),
        (601_716, 1_570, 3, 3),
        (613_424, 1_602, 3, 22),
        (626_596, 1_638, 4, 13),
        (645_554, 1_690, 3, 10),
        (664_224, 1_741, 4, 20),
        (671_401, 1_760, 12, 16),
        (694_799, 1_825, 1, 7),
        (704_424, 1_851, 5, 10),
        (708_842, 1_863, 6, 14),
        (709_409, 1_865, 1, 7),
        (709_580, 1_865, 6, 21),
        (727_274, 1_913, 12, 4),
        (728_714, 1_917, 11, 13),
        (744_313, 1_960, 7, 24),
        (764_652, 2_016, 4, 2),
    ];

    #[test]
    fn the_books_sample_dates_are_reproduced_both_ways() {
        let calendar = SiddhantaSolarCalendar::UJJAIN;
        for (rd, year, month, day) in SAMPLE_DATES {
            let date = HinduSolarDate { year, month, day };
            assert_eq!(calendar.from_fixed(Rd(rd)), Ok(date), "R.D. {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "{date:?}");
        }
    }

    #[test]
    fn mesha_1_of_saka_1947_follows_the_siddhantas_sankranti() {
        // docs/systems/hindu-calendars.md works this day. The Siddhānta's
        // Meṣa saṅkrānti falls at 00:08 UT on 14 April 2025, before its
        // sunrise at Ujjain that morning, 00:51 UT, and after the sunrise of
        // the 13th: the day that sunrise closes, 13 April, is Meṣa 1.
        let calendar = SiddhantaSolarCalendar::UJJAIN;
        let sankranti =
            surya_siddhanta::ingress_after(SiderealSign::MESHA, Moment(ymd(2025, 4, 1).0 as f64));
        assert_eq!(sankranti.day(), ymd(2025, 4, 14));
        assert!((sankranti.0.rem_euclid(1.0) * 24.0 - 0.128).abs() < 0.002);
        let rise = surya_siddhanta::sunrise(ymd(2025, 4, 14), UJJAIN);
        assert!(sankranti.0 < rise.0);
        assert!((rise.0.rem_euclid(1.0) * 24.0 - 0.848).abs() < 0.002);
        assert!(surya_siddhanta::sunrise(ymd(2025, 4, 13), UJJAIN).0 < sankranti.0);
        let first = HinduSolarDate {
            year: 1_947,
            month: 1,
            day: 1,
        };
        assert_eq!(calendar.to_fixed(first), Ok(ymd(2025, 4, 13)));
        assert_eq!(
            calendar
                .from_fixed(ymd(2025, 4, 12))
                .map(|date| (date.year, date.month)),
            Ok((1_946, 12))
        );
    }

    #[test]
    fn every_day_of_the_range_round_trips() {
        // Every day of the 3.65 million in a release build, spread over the
        // machine's threads; a debug build, which the coverage job runs
        // instrumented, takes every 37th day and every Meṣa 1 of the ten
        // thousand years with the day before it (docs/policy.md §7); a build
        // instrumented for coverage takes a third as many of the days and the
        // opening of every seventh year.
        let calendar = SiddhantaSolarCalendar::UJJAIN;
        let (first, last) = (NAMED_RANGE.1.0, NAMED_RANGE.2.0);
        let openings: alloc::vec::Vec<i64> = if cfg!(debug_assertions) {
            (MIN_YEAR..=MAX_YEAR)
                .step_by(hc_core::sweep::year_step())
                .map(|year| calendar.month_start(year, 1).expect("in range").0)
                .chain([last + 1])
                .collect()
        } else {
            alloc::vec::Vec::new()
        };
        let days = crate::sweep_days(first, last, 37, &openings);
        assert!(days.contains(&first) && days.contains(&last));
        crate::check_days(&days, |day| {
            let date = calendar.from_fixed(Rd(day)).expect("in range");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(day)), "{date:?}");
        });
        assert_eq!(
            calendar.from_fixed(Rd(first - 1)),
            Err(CalendarError::BeforeEpoch)
        );
        assert_eq!(
            calendar.from_fixed(Rd(last + 1)),
            Err(CalendarError::AfterSupportedRange)
        );
        assert_eq!(
            calendar.month_start(MIN_YEAR - 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            calendar.month_start(1_947, 13),
            Err(CalendarError::MonthOutOfRange)
        );
        let length = calendar.days_in_month(1_947, 1).expect("in range");
        assert!((30..=32).contains(&length), "{length}");
        assert_eq!(
            calendar.to_fixed(HinduSolarDate {
                year: 1_947,
                month: 1,
                day: length + 1,
            }),
            Err(CalendarError::DayOutOfRange)
        );
    }

    #[test]
    fn every_month_of_a_century_is_29_to_32_days_and_the_year_365_or_366() {
        let calendar = SiddhantaSolarCalendar::UJJAIN;
        for year in 1_900..2_000 {
            let mut total = 0;
            for month in 1..=12 {
                let length = calendar.days_in_month(year, month).expect("in range");
                assert!((29..=32).contains(&length), "{year}-{month}: {length}");
                total += u32::from(length);
            }
            assert!(total == 365 || total == 366, "{year}: {total}");
        }
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = SiddhantaSolarCalendar::UJJAIN;
        let meta = calendar.meta();
        assert_eq!(meta.id, ID);
        assert_eq!(
            (meta.earliest, meta.latest),
            (Some(NAMED_RANGE.1), Some(NAMED_RANGE.2))
        );
        let day = ymd(2025, 4, 13);
        let fields = calendar
            .to_fields(calendar.from_fixed(day).expect("in range"))
            .expect("fields");
        assert_eq!(fields.era, Some(ERA));
        assert_eq!(
            calendar
                .from_fields(&fields)
                .map(|date| calendar.to_fixed(date)),
            Ok(Ok(day))
        );
        assert_eq!(
            calendar.from_fields(&fields.with_era("vs")),
            Err(CalendarError::UnknownEra)
        );
        assert_eq!(calendar.is_leap_year(1_947), Ok(false));
        assert_eq!(
            calendar.is_leap_year(MAX_YEAR + 1),
            Err(CalendarError::YearOutOfRange)
        );
    }
}
