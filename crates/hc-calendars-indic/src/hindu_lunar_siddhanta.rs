//! The Hindu lunisolar calendar, *amānta*, on the *Sūrya Siddhānta*'s Sun
//! and Moon — `hindu-lunar-surya-siddhanta`.
//!
//! The months, the tithis and the years are [`crate::hindu_lunar`]'s, by
//! the same engine: a month from conjunction to conjunction, named for the
//! saṅkrānti it holds, intercalary when it holds none; the day's tithi the
//! one in progress at sunrise; the year in the Śaka era with the Vikrama
//! year beside it. What differs is the sky. Where `hindu-lunar` reads the
//! true Sun and Moon of modern astronomy in the Lahiri zodiac at the
//! Central Station's sunrise, as the *Rashtriya Panchang* does, this
//! calendar reads the Siddhānta's: its Sun and Moon, each a mean motion
//! corrected by a shrinking epicycle read off the classical sine table, the
//! Moon's anomaly with the *bīja*, its own sidereal zodiac, and its own
//! sunrise at Ujjain ([`crate::surya_siddhanta`]). It is the modern Hindu
//! lunisolar calendar of Reingold and Dershowitz, *Calendrical
//! Calculations*, `hindu-lunar-from-fixed` and `fixed-from-hindu-lunar`
//! in their published code (`reingold2018code`). In the questionnaire the
//! Calendar Reform Committee sent the almanac makers in 1953–54, three
//! lunisolar almanacs answer that they compute by the *Sūrya Siddhānta*,
//! and only one of them on these months: the *Sri Sringagiri Sri Jagat
//! Guru Srimath Panchangam* of Kollegal is amānta, while the *Hosaritti
//! Panchanga* of Dharwar and the *Bhagyodaya Panchang* alias *Chintaharan
//! Jantri* of Sitapur are "Luni Solar, Purnimanta" (`crc1955`, Annexure
//! VI, replies 48, 43 and 47). A pūrṇimānta calendar on the Siddhānta's
//! sky is not registered. No table of any of those almanacs was read, so
//! the calendar is held to the book's sample dates, and whether any
//! almanac's months and tithis are these to the day is not known here.
//!
//! The system is written up in `docs/systems/hindu-calendars.md`, with the
//! sources and the measurements.
//!
//! # Range
//!
//! Kali Yuga years 1 to 10 000 — Śaka −3178 to 6821, 3101 BCE to 6899 CE
//! — as for the Old Hindu calendars: the reckoning is arithmetic and
//! counts from the Kali Yuga epoch, so its range is the library's choice
//! and not a limit of a model, and the book's sample dates from 586 BCE are
//! inside it. What it is not is a claim that anyone kept this calendar in
//! 586 BCE: the text and its *bīja* are far younger, and the dates it gives
//! before them are the rule's, proleptically.

use hc_astro::riseset::Location;
use hc_calendar::fixed::Moment;
use hc_calendar::{Calendar, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, YearKind};
use hc_core::math::round;
use hc_seasons::zodiac::SiderealSign;

use crate::amanta::Sky;
use crate::hindu_lunar::{HinduLunarDate, SHAPE, date_of, fields_of};
use crate::places::UJJAIN;
use crate::surya_siddhanta::{self, SIDEREAL_YEAR};

/// The identifier of the amānta calendar on the *Sūrya Siddhānta*.
pub const ID: CalendarId = CalendarId("hindu-lunar-surya-siddhanta");

/// The Kali Yuga year less the Śaka year: the Śaka era begins in Kali
/// 3179 (`hindu-solar-era`, `reingold2018code`).
pub const KALI_SAKA_OFFSET: i64 = 3_179;

/// The earliest Śaka year this calendar converts: Kali Yuga 1.
pub const MIN_YEAR: i64 = 1 - KALI_SAKA_OFFSET;

/// The latest Śaka year this calendar converts: Kali Yuga 10 000.
pub const MAX_YEAR: i64 = 10_000 - KALI_SAKA_OFFSET;

/// The Kali Yuga epoch as a moment of Universal Time: midnight at Ujjain,
/// Friday 18 February 3102 BCE (Julian), fixed day −1 132 959
/// (`hindu-epoch`).
pub(crate) const EPOCH: f64 = -1_132_959.0 - UJJAIN.longitude_degrees / 360.0;

/// The amānta Hindu lunisolar calendar on the *Sūrya Siddhānta*'s Sun and
/// Moon, the day read at the Siddhānta's sunrise at a place.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SiddhantaLunarCalendar {
    /// Whose sunrise reads the day.
    pub location: Location,
}

impl SiddhantaLunarCalendar {
    /// The calendar read at Ujjain, as Reingold and Dershowitz read it: the
    /// registered `hindu-lunar-surya-siddhanta`.
    pub const UJJAIN: Self = Self::new(UJJAIN);

    /// The calendar read at another place's Siddhāntic sunrise.
    #[must_use]
    pub const fn new(location: Location) -> Self {
        Self { location }
    }

    crate::amanta::amanta_methods!();
}

/// The earliest and latest days of the registered calendar, as
/// [`SiddhantaLunarCalendar::earliest`] and
/// [`SiddhantaLunarCalendar::latest`] compute them;
/// `tests::the_named_range_is_the_computed_one` computes it again.
const NAMED_RANGE: (SiddhantaLunarCalendar, Rd, Rd) = (
    SiddhantaLunarCalendar::UJJAIN,
    Rd(-1_132_604),
    Rd(2_519_974),
);

/// The *Sūrya Siddhānta*'s Sun and Moon, its zodiac and its sunrise.
impl Sky for SiddhantaLunarCalendar {
    fn conjunction_at_or_after(&self, moment: Moment) -> Moment {
        surya_siddhanta::conjunction_at_or_after(moment)
    }

    fn sign_at(&self, moment: Moment) -> SiderealSign {
        surya_siddhanta::sign_at(moment)
    }

    fn ingress_after(&self, sign: SiderealSign, moment: Moment) -> Moment {
        surya_siddhanta::ingress_after(sign, moment)
    }

    fn sunrise(&self, day: Rd) -> Moment {
        surya_siddhanta::sunrise(day, self.location)
    }

    fn tithi_of_day(&self, day: Rd) -> u8 {
        surya_siddhanta::tithi_at(self.sunrise(day))
    }

    /// The Siddhānta's solar year the month's name falls in: the Kali year
    /// whose Meṣa saṅkrānti is nearest the month's start carried back by
    /// the part of a year its name stands from Meṣa, as the book's
    /// `hindu-calendar-year` reckons it, less 3179.
    fn year_of(&self, start: Moment, month: u8) -> i64 {
        let years = (start.0 - EPOCH) / SIDEREAL_YEAR - f64::from(month - 1) / 12.0;
        round(years) as i64 - KALI_SAKA_OFFSET
    }

    /// A fortnight before the mean saṅkrānti, which the true one is never
    /// more than two and a quarter days from.
    fn sankranti_search_start(&self, year: i64, month: u8) -> Moment {
        let years = (year + KALI_SAKA_OFFSET) as f64 + f64::from(month - 1) / 12.0;
        Moment(EPOCH + years * SIDEREAL_YEAR - 15.0)
    }

    fn years(&self) -> (i64, i64) {
        (MIN_YEAR, MAX_YEAR)
    }

    fn named_range(&self) -> Option<(Rd, Rd)> {
        let (calendar, earliest, latest) = NAMED_RANGE;
        (calendar == *self).then_some((earliest, latest))
    }

    /// No ayanāṃśa's key: two NaN patterns, which no anchor or angle has.
    fn zodiac_key(&self) -> [u64; 2] {
        [u64::MAX; 2]
    }

    fn place_key(&self) -> [u64; 3] {
        self.location.key()
    }
}

impl Calendar for SiddhantaLunarCalendar {
    type Date = HinduLunarDate;

    /// Unrecorded: the almanacs that answered in 1953–54 that they compute
    /// by the Siddhānta say so without a period, and whether they keep
    /// this reckoning, with this *bīja* and this sunrise, is not recorded
    /// by any source read.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// As `hindu-lunar`: twelve months with a thirteenth in an intercalary
    /// year, the seven-day week, and the sixty year names of the southern
    /// cycle.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    /// A year with an adhika māsa.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        Ok(self.leap_month_of(year)?.is_some())
    }

    /// As `hindu-lunar`: the day begins at the sunrise and is named by the
    /// civil day on whose sunrise it begins (`hindu-lunar-from-fixed`).
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunrise(hc_calendar::DayNaming::ByStart)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: ID,
            english_name: "Hindu lunisolar (amanta, Surya Siddhanta)",
            year_kind: YearKind::EpochForward,
            has_leap_months: true,
            is_astronomical: true,
            earliest: self.earliest().ok(),
            latest: self.latest().ok(),
            native_locales: &["sa", "hi", "kn"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        SiddhantaLunarCalendar::to_fixed(self, date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        SiddhantaLunarCalendar::from_fixed(self, rd)
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        fields_of(date)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        let date = date_of(fields)?;
        SiddhantaLunarCalendar::to_fixed(self, date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use hc_calendar::CalendarError;
    use hc_calendars_solar::gregorian;

    use super::*;
    use crate::amanta::Amanta;
    use crate::hindu_lunar::HinduLunarCalendar;

    fn ymd(year: i64, month: u8, day: u8) -> Rd {
        gregorian::to_fixed(year, month, day).expect("a Gregorian date")
    }

    #[test]
    fn the_named_range_is_the_computed_one() {
        let engine = Amanta(SiddhantaLunarCalendar::UJJAIN);
        assert_eq!(engine.computed_earliest(), Ok(NAMED_RANGE.1));
        assert_eq!(engine.computed_latest(), Ok(NAMED_RANGE.2));
    }

    /// The 33 sample dates of *Calendrical Calculations*, 586 BCE to 2094,
    /// as its published code's `hindu-lunar-from-fixed` gives them: the
    /// R.D., the Vikrama year, the month, whether it is intercalary, the
    /// tithi and whether it is repeated (`reingold2018code`, `dates.l`, run
    /// for this library on 2026-09-27; the table is
    /// `crates/hyper-calendar/tests/data/calendrica_sample_dates.txt`).
    #[rustfmt::skip]
    const SAMPLE_DATES: [(i64, i64, u8, bool, u8, bool); 33] = [
        (-214_193, -529, 6, false, 11, false),
        (-61_387, -111, 9, false, 27, false),
        (25_469, 127, 8, false, 3, false),
        (49_217, 192, 8, false, 9, false),
        (171_307, 526, 11, false, 19, false),
        (210_155, 633, 3, false, 5, false),
        (253_427, 751, 9, false, 15, false),
        (369_740, 1070, 2, false, 6, false),
        (400_085, 1153, 3, true, 23, false),
        (434_355, 1247, 1, false, 8, false),
        (452_605, 1297, 1, false, 8, false),
        (470_160, 1345, 1, false, 22, false),
        (473_837, 1355, 2, false, 8, false),
        (507_850, 1448, 4, false, 1, false),
        (524_156, 1492, 11, false, 7, false),
        (544_676, 1549, 2, true, 3, false),
        (567_118, 1610, 7, false, 2, false),
        (569_477, 1616, 11, false, 28, true),
        (601_716, 1705, 3, false, 20, false),
        (613_424, 1737, 4, false, 4, false),
        (626_596, 1773, 5, false, 6, false),
        (645_554, 1825, 4, false, 5, false),
        (664_224, 1876, 5, false, 11, false),
        (671_401, 1896, 1, false, 13, false),
        (694_799, 1960, 1, false, 22, false),
        (704_424, 1986, 5, false, 20, false),
        (708_842, 1998, 7, false, 9, false),
        (709_409, 2000, 1, false, 14, false),
        (709_580, 2000, 7, false, 8, false),
        (727_274, 2048, 12, false, 14, false),
        (728_714, 2052, 12, false, 7, false),
        (744_313, 2095, 8, false, 14, false),
        (764_652, 2151, 4, false, 6, false),
    ];

    #[test]
    fn the_books_sample_dates_are_reproduced_both_ways() {
        let calendar = SiddhantaLunarCalendar::UJJAIN;
        for (rd, vikrama, month, leap_month, day, leap_day) in SAMPLE_DATES {
            let date = HinduLunarDate {
                year: vikrama - crate::hindu_lunar::VIKRAMA_OFFSET,
                month,
                leap_month,
                day,
                leap_day,
            };
            assert_eq!(calendar.from_fixed(Rd(rd)), Ok(date), "R.D. {rd}");
            assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "{date:?}");
        }
    }

    #[test]
    fn chaitra_sukla_1_of_saka_1947_worked_by_hand() {
        // docs/systems/hindu-calendars.md works this day. The Siddhānta's
        // conjunction falls at 11:18 UT on 29 March 2025 with its Sun in
        // Mīna, and its Meṣa saṅkrānti at 00:08 UT on 14 April, inside the
        // month, so the month is Chaitra and ordinary. Its sunrise at
        // Ujjain on 30 March is at 01:01 UT, 06:04 by Ujjain's clock, ten
        // minutes after the true one, and the elongation then is 7.58°:
        // the first tithi, the new year's day.
        let conjunction =
            surya_siddhanta::conjunction_at_or_after(Moment(ymd(2025, 3, 28).0 as f64));
        assert_eq!(conjunction.day(), ymd(2025, 3, 29));
        assert!((conjunction.0.rem_euclid(1.0) * 24.0 - 11.30).abs() < 0.01);
        assert_eq!(surya_siddhanta::sign_at(conjunction), SiderealSign::MINA);
        let mesha = surya_siddhanta::ingress_after(SiderealSign::MESHA, conjunction);
        assert_eq!(mesha.day(), ymd(2025, 4, 14));
        let sunrise = surya_siddhanta::sunrise(ymd(2025, 3, 30), UJJAIN);
        assert!((sunrise.0.rem_euclid(1.0) * 24.0 - 1.018).abs() < 0.001);
        let true_sunrise = crate::tithi::sunrise_of(ymd(2025, 3, 30), UJJAIN);
        assert!(((sunrise.0 - true_sunrise.0) * 1_440.0 - 9.8).abs() < 0.5);
        assert!((surya_siddhanta::lunar_phase(sunrise) - 7.584).abs() < 0.001);
        let calendar = SiddhantaLunarCalendar::UJJAIN;
        assert_eq!(calendar.new_year(1_947), Ok(ymd(2025, 3, 30)));
        let date = calendar.from_fixed(ymd(2025, 3, 30)).expect("in range");
        assert_eq!((date.month, date.leap_month, date.day), (1, false, 1));
    }

    #[test]
    fn a_conjunction_is_where_the_elongation_returns_to_zero() {
        let mut moment = Moment(ymd(2024, 1, 1).0 as f64);
        for _ in 0..13 {
            let conjunction = surya_siddhanta::conjunction_at_or_after(moment);
            assert!(conjunction.0 >= moment.0);
            assert!(surya_siddhanta::lunar_phase(Moment(conjunction.0 + 1e-6)) < 1e-3);
            assert!(surya_siddhanta::lunar_phase(Moment(conjunction.0 - 1e-6)) > 359.999);
            assert_eq!(surya_siddhanta::tithi_at(Moment(conjunction.0 + 1e-6)), 1);
            assert_eq!(surya_siddhanta::tithi_at(Moment(conjunction.0 - 1e-6)), 30);
            moment = Moment(conjunction.0 + 1.0);
        }
    }

    #[test]
    fn the_siddhantas_sunrise_at_ujjain_is_minutes_from_the_true_one() {
        // Its own sunrise, of the Sun's centre on a flat horizon with its
        // own declination and equation of time, against hc-astro's upper
        // limb with refraction: from 7.5 minutes earlier to 15.8 later over
        // 2000–2009. Every day in a release build, every seventh in a debug
        // one.
        let (mut earliest, mut latest) = (f64::MAX, f64::MIN);
        let first = ymd(2000, 1, 1).0;
        for day in (first..first + 3_653).step_by(crate::sweep_stride(7)) {
            let siddhanta = surya_siddhanta::sunrise(Rd(day), UJJAIN).0;
            let true_sunrise = crate::tithi::sunrise_of(Rd(day), UJJAIN).0;
            let minutes = (siddhanta - true_sunrise) * 1_440.0;
            earliest = earliest.min(minutes);
            latest = latest.max(minutes);
        }
        assert!((-7.6..-6.0).contains(&earliest), "{earliest}");
        assert!((14.0..15.9).contains(&latest), "{latest}");
    }

    #[test]
    fn the_siddhanta_and_the_true_sky_part_on_one_day_in_eight() {
        // Against `HinduLunarCalendar::UJJAIN`, the true Sun and Moon at the
        // same place: 44 of the 366 days of 2024 carry another date, and in
        // a release build 1 389 of the 11 323 days of 2000–2030, 100 of
        // them in another month.
        let siddhanta = SiddhantaLunarCalendar::UJJAIN;
        let modern = HinduLunarCalendar::UJJAIN;
        let count = |first: Rd, last: Rd| {
            let (mut days, mut months) = (0, 0);
            for day in first.0..=last.0 {
                let a = siddhanta.from_fixed(Rd(day)).expect("in range");
                let b = modern.from_fixed(Rd(day)).expect("in range");
                days += u32::from(a != b);
                months +=
                    u32::from((a.year, a.month, a.leap_month) != (b.year, b.month, b.leap_month));
            }
            (days, months)
        };
        assert_eq!(count(ymd(2024, 1, 1), ymd(2024, 12, 31)).0, 44);
        if !cfg!(debug_assertions) {
            assert_eq!(count(ymd(2000, 1, 1), ymd(2030, 12, 31)), (1_389, 100));
        }
    }

    #[test]
    fn saka_1904_loses_magha_and_has_two_intercalary_months() {
        // The Siddhānta's Pauṣa of 15 January 1983 holds the Makara and
        // the Kumbha saṅkrāntis, so Māgha is expunged; the year has an
        // intercalary Āśvina from 18 September 1982 and an intercalary
        // Phālguna from 13 February 1983, thirteen months in all. No source
        // read dates this year's months, so the test holds the rule: the
        // lost name is refused and the others convert.
        let calendar = SiddhantaLunarCalendar::UJJAIN;
        assert_eq!(calendar.has_kshaya_month(1_904), Ok(true));
        assert_eq!(
            calendar.month_span(1_904, 11, false),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            calendar.month_span(1_904, 7, true).map(|span| span.0),
            Ok(ymd(1982, 9, 18))
        );
        assert_eq!(
            calendar.month_span(1_904, 10, false).map(|span| span.0),
            Ok(ymd(1983, 1, 15))
        );
        assert_eq!(
            calendar.month_span(1_904, 12, true).map(|span| span.0),
            Ok(ymd(1983, 2, 13))
        );
        assert_eq!(calendar.has_kshaya_month(1_903), Ok(false));
    }

    #[test]
    fn every_day_of_the_range_round_trips() {
        // Every day of the 3.65 million in a release build, spread over the
        // machine's threads: about 60 µs a day, four minutes of one core. A
        // debug build, which the coverage job runs instrumented, takes every
        // 181st day and every Chaitra śukla 1 of the ten thousand years with
        // the day before it (docs/policy.md §7); a build instrumented for
        // coverage takes a third as many of the days and the opening of
        // every seventh year.
        let calendar = SiddhantaLunarCalendar::UJJAIN;
        let (first, last) = (NAMED_RANGE.1.0, NAMED_RANGE.2.0);
        let openings: alloc::vec::Vec<i64> = if cfg!(debug_assertions) {
            (MIN_YEAR..=MAX_YEAR)
                .step_by(hc_core::sweep::year_step())
                .map(|year| calendar.new_year(year).expect("in range").0)
                .chain([last + 1])
                .collect()
        } else {
            alloc::vec::Vec::new()
        };
        let days = crate::sweep_days(first, last, 181, &openings);
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
            calendar.new_year(MIN_YEAR - 1),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            calendar.new_year(MAX_YEAR + 1),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn the_calendar_impl_round_trips_through_fields() {
        let calendar = SiddhantaLunarCalendar::UJJAIN;
        let meta = calendar.meta();
        assert_eq!(meta.id, ID);
        assert_eq!(
            (meta.earliest, meta.latest),
            (Some(NAMED_RANGE.1), Some(NAMED_RANGE.2))
        );
        let day = ymd(2025, 3, 30);
        let fields = calendar
            .to_fields(calendar.from_fixed(day).expect("in range"))
            .expect("fields");
        assert_eq!(fields.era, Some(crate::hindu_lunar::ERA));
        assert_eq!(fields.extra.get("vikrama-year"), Some(2_082));
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
        assert_eq!(calendar.is_leap_year(1_904), Ok(true));
    }
}
