//! The Old Hindu calendars: the mean-motion solar and lunisolar reckonings
//! of the *Ārya Siddhānta*, the arithmetic behind the almanacs before the
//! true positions took over.
//!
//! The reckoning, its place beside the true calendars, how it is tested
//! without an almanac and its sources are in
//! `docs/systems/hindu-calendars.md` in the repository. This page
//! summarises it and states the code's own facts.
//!
//! Both count from the Kali Yuga epoch, Friday 18 February 3102 BCE in
//! the Julian calendar, when the mean Sun and Moon stood together at the
//! start of Meṣa, and both move the mean Sun at one sidereal year of
//! 1 577 917 500⁄4 320 000 days — 365.258 68 — and the mean Moon at one
//! synodic month of 1 577 917 500⁄53 433 336 days. The solar year is
//! twelve mean months of a twelfth of that year, day 1 of a month being
//! the civil day in which the mean Sun enters it; the lunisolar year is
//! the mean lunar months, each named for the solar month that begins
//! within it, so that a lunar month no solar month begins in takes the
//! same name as the month after it and is the intercalary one. Its days
//! are the mean tithis, thirty to a month, and a civil day takes the tithi
//! current at its mean sunrise, a quarter day after midnight. The mean
//! tithi is shorter than a day, so a tithi is skipped now and then and
//! none is ever repeated.
//!
//! # Whose arithmetic
//!
//! The functions of Reingold and Dershowitz, *Calendrical Calculations*
//! (4th ed., 2018), in the section on the Old Hindu calendars, as their
//! published code gives them (`reingold2018code`; the book, `reingold2018`,
//! not read here):
//! `old-hindu-solar-from-fixed`, `fixed-from-old-hindu-solar`,
//! `old-hindu-lunar-from-fixed`, `fixed-from-old-hindu-lunar` and
//! `old-hindu-lunar-leap-year?`, with the epoch, the year and month
//! lengths and the leap-year threshold as they give them.
//!
//! The *Rashtriya Panchang* does not tabulate the mean calendars, so they
//! are tested for what an arithmetic calendar must do — every day of the
//! range converts and converts back, months and years have the lengths
//! the mean motions allow, an intercalary month falls seven years in
//! nineteen and now and then eight, and precedes the month it is named
//! for — for where both calendars place the epoch, and for how far the
//! mean months of Kali Yuga 5125 (Śaka 1946, 2024 to 2025) stand from
//! the true Tamil months of [`crate::hindu_solar`]: between two days
//! early and two days late, a quarter of a day late on average.

use hc_calendar::shape::{CycleShape, LUNISOLAR_TWELVE, MONTH, WEEKDAY};
use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Month, Rd,
    YearKind,
};
use hc_core::math::{ceil, floor};
use hc_seasons::zodiac::SiderealSign;

/// The identifier of the mean solar calendar.
pub const SOLAR_ID: CalendarId = CalendarId("hindu-old-solar");
/// The identifier of the mean lunisolar calendar.
pub const LUNAR_ID: CalendarId = CalendarId("hindu-old-lunar");
/// The era both count in.
pub const ERA: &str = "kali-yuga";

/// The Kali Yuga epoch: Friday, 18 February 3102 BCE (Julian), as a fixed
/// day.
pub const HINDU_EPOCH: Rd = Rd(-1_132_959);

/// The *Ārya Siddhānta*'s sidereal year in days: 1 577 917 500⁄4 320 000.
pub const ARYA_SOLAR_YEAR: f64 = 1_577_917_500.0 / 4_320_000.0;
/// A twelfth of the year.
pub const ARYA_SOLAR_MONTH: f64 = ARYA_SOLAR_YEAR / 12.0;
/// The *Ārya Siddhānta*'s synodic month in days: 1 577 917 500⁄53 433 336.
pub const ARYA_LUNAR_MONTH: f64 = 1_577_917_500.0 / 53_433_336.0;
/// A thirtieth of the month: the mean tithi.
pub const ARYA_LUNAR_DAY: f64 = ARYA_LUNAR_MONTH / 30.0;

/// The first Kali Yuga year either calendar converts.
///
/// The epoch opens the solar year 0 exactly. The lunisolar year 0 is the
/// one year in which the rule of
/// [`OldHinduLunarCalendar::is_leap_year`] and the months disagree: the
/// rule calls it intercalary, and its intercalary Chaitra begins a lunar
/// month before the epoch, so the calendar's first month is the ordinary
/// Chaitra. Every later year has the month its rule promises.
pub const MIN_YEAR: i64 = 0;
/// The last Kali Yuga year either calendar converts.
pub const MAX_YEAR: i64 = 10_000;

/// The solar months, the sidereal signs the mean Sun passes through.
pub const SOLAR_MONTHS: [&str; 12] = [
    SiderealSign::MESHA.sanskrit_name(),
    SiderealSign::VRISHABHA.sanskrit_name(),
    SiderealSign::MITHUNA.sanskrit_name(),
    SiderealSign::KARKA.sanskrit_name(),
    SiderealSign::SIMHA.sanskrit_name(),
    SiderealSign::KANYA.sanskrit_name(),
    SiderealSign::TULA.sanskrit_name(),
    SiderealSign::VRISHCHIKA.sanskrit_name(),
    SiderealSign::DHANUS.sanskrit_name(),
    SiderealSign::MAKARA.sanskrit_name(),
    SiderealSign::KUMBHA.sanskrit_name(),
    SiderealSign::MINA.sanskrit_name(),
];

/// Days since the epoch, plus the quarter day that puts mean sunrise at
/// the start of the civil day.
fn sun_of(rd: Rd) -> f64 {
    (rd.0 - HINDU_EPOCH.0) as f64 + 0.25
}

/// `x mod y` for a positive `y`, in `[0, y)`.
fn modulo(x: f64, y: f64) -> f64 {
    x - y * floor(x / y)
}

// ─────────────────────────────────────────────────────────────────────────
// Solar
// ─────────────────────────────────────────────────────────────────────────

/// A date of the mean solar calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OldHinduSolarDate {
    /// The Kali Yuga year, 0 at the epoch.
    pub year: i64,
    /// The month, 1 for Meṣa through 12 for Mīna.
    pub month: u8,
    /// The day of the month, 1 through 31.
    pub day: u8,
}

/// The mean solar calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OldHinduSolarCalendar;

impl OldHinduSolarCalendar {
    /// The fixed day of a date: the ceiling of the epoch plus the mean
    /// Sun's travel to the day, less the quarter day and one.
    ///
    /// # Errors
    ///
    /// [`CalendarError::YearOutOfRange`] outside [`MIN_YEAR`]..=[`MAX_YEAR`];
    /// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`]
    /// for a month or day the year does not have.
    pub fn to_fixed(self, date: OldHinduSolarDate) -> CalendarResult<Rd> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&date.year) {
            return Err(CalendarError::YearOutOfRange);
        }
        if !(1..=12).contains(&date.month) {
            return Err(CalendarError::MonthOutOfRange);
        }
        if !(1..=31).contains(&date.day) {
            return Err(CalendarError::DayOutOfRange);
        }
        let rd = self.fixed_of(date);
        // The existence check comes first: Mīna 31 of the last year is a
        // day no year has, not a real day pushed out of the range.
        if self.date_of(rd) != date {
            return Err(CalendarError::DayOutOfRange);
        }
        if rd < self.earliest() || rd > self.latest() {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(rd)
    }

    fn fixed_of(self, date: OldHinduSolarDate) -> Rd {
        let travel = date.year as f64 * ARYA_SOLAR_YEAR
            + f64::from(date.month - 1) * ARYA_SOLAR_MONTH
            + f64::from(date.day)
            - 1.25;
        Rd(HINDU_EPOCH.0 + ceil(travel) as i64)
    }

    /// The date on a fixed day.
    ///
    /// # Errors
    ///
    /// [`CalendarError::YearOutOfRange`] for a day before year 0 or after
    /// year [`MAX_YEAR`].
    pub fn from_fixed(self, rd: Rd) -> CalendarResult<OldHinduSolarDate> {
        if rd < self.earliest() || rd > self.latest() {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(self.date_of(rd))
    }

    fn date_of(self, rd: Rd) -> OldHinduSolarDate {
        let sun = sun_of(rd);
        let year = floor(sun / ARYA_SOLAR_YEAR) as i64;
        let month = (floor(sun / ARYA_SOLAR_MONTH) as i64).rem_euclid(12) as u8 + 1;
        let day = floor(modulo(sun, ARYA_SOLAR_MONTH)) as u8 + 1;
        OldHinduSolarDate { year, month, day }
    }

    /// The first day converted: the epoch, Meṣa 1 of year 0.
    #[must_use]
    pub fn earliest(self) -> Rd {
        HINDU_EPOCH
    }

    /// The last day converted: the day before Meṣa 1 of the year after
    /// [`MAX_YEAR`].
    #[must_use]
    pub fn latest(self) -> Rd {
        let next = self.fixed_of(OldHinduSolarDate {
            year: MAX_YEAR + 1,
            month: 1,
            day: 1,
        });
        Rd(next.0 - 1)
    }
}

impl Calendar for OldHinduSolarCalendar {
    type Date = OldHinduSolarDate;

    /// Unrecorded: a mean reckoning that the almanacs replaced with true
    /// positions at dates no source read gives, so there is no period to
    /// state.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// Twelve months named for the signs, and the seven-day week.
    fn cycles(&self) -> &'static [CycleShape] {
        const SHAPE: &[CycleShape] = &[
            CycleShape::named(MONTH, &SOLAR_MONTHS),
            CycleShape::fixed(WEEKDAY, 7),
        ];
        SHAPE
    }

    /// Never: the mean solar year has no intercalary unit.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(false)
    }

    /// The day begins at mean sunrise, a quarter day after midnight, and is
    /// named by the civil day on whose sunrise it begins: a fixed day's date
    /// is read at "Sunrise on Hindu date", six hours after its midnight
    /// (`reingold2018code`, `old-hindu-solar-from-fixed`).
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunrise(hc_calendar::DayNaming::ByStart)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: SOLAR_ID,
            english_name: "Old Hindu solar (mean)",
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: false,
            earliest: Some(self.earliest()),
            latest: Some(self.latest()),
            native_locales: &["sa"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        OldHinduSolarCalendar::to_fixed(*self, date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        OldHinduSolarCalendar::from_fixed(*self, rd)
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
        let date = OldHinduSolarDate {
            year: fields.year,
            month: month.ordinal,
            day: fields.require_day()?,
        };
        OldHinduSolarCalendar::to_fixed(*self, date)?;
        Ok(date)
    }
}

// ─────────────────────────────────────────────────────────────────────────
// Lunisolar
// ─────────────────────────────────────────────────────────────────────────

/// A date of the mean lunisolar calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OldHinduLunarDate {
    /// The Kali Yuga year.
    pub year: i64,
    /// The month, 1 for Chaitra through 12 for Phālguna.
    pub month: u8,
    /// Whether this is the intercalary month of that name, which precedes
    /// the ordinary one.
    pub leap_month: bool,
    /// The tithi, 1 through 30.
    pub day: u8,
}

/// The mean lunisolar calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OldHinduLunarCalendar;

impl OldHinduLunarCalendar {
    /// Whether a Kali Yuga year has an intercalary month.
    ///
    /// Seven years in nineteen, and now and then eight: the year is
    /// intercalary when the mean new moon that opens its lunar year —
    /// the first at or after the Mīna saṅkrānti, one solar month before
    /// its solar new year — falls within the first 10.89 days of Mīna.
    /// Beginning that early leaves room for two new moons to share one
    /// solar month later in the year, and that month is the intercalary
    /// one. The month the answer promises is in range from year 1; for
    /// year 0 it begins before the epoch, as [`MIN_YEAR`] says.
    #[must_use]
    pub fn is_leap_year(self, year: i64) -> bool {
        modulo(
            year as f64 * ARYA_SOLAR_YEAR - ARYA_SOLAR_MONTH,
            ARYA_LUNAR_MONTH,
        ) >= 23_902_504_679.0 / 1_282_400_064.0
    }

    /// The fixed day of a date.
    ///
    /// # Errors
    ///
    /// [`CalendarError::YearOutOfRange`] outside [`MIN_YEAR`]..=[`MAX_YEAR`];
    /// [`CalendarError::MonthOutOfRange`] for a month the year does not
    /// have, an intercalary one in a year without it included;
    /// [`CalendarError::DayOutOfRange`] for a tithi the month skips.
    pub fn to_fixed(self, date: OldHinduLunarDate) -> CalendarResult<Rd> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&date.year) {
            return Err(CalendarError::YearOutOfRange);
        }
        if !(1..=12).contains(&date.month) {
            return Err(CalendarError::MonthOutOfRange);
        }
        if !(1..=30).contains(&date.day) {
            return Err(CalendarError::DayOutOfRange);
        }
        let rd = self.fixed_of(date);
        let found = self.date_of(rd);
        if found == date {
            if rd < self.earliest() || rd > self.latest() {
                return Err(CalendarError::YearOutOfRange);
            }
            return Ok(rd);
        }
        // Which is wrong, the month or the tithi? Not the month of the day
        // the tithi landed on: a skipped thirtieth tithi lands on the first
        // day of the next month and would blame the month for it. Ask
        // instead whether the month itself is a month of that year, by
        // looking at where its own first tithi falls.
        let head = self.date_of(self.fixed_of(OldHinduLunarDate { day: 1, ..date }));
        if (head.year, head.month, head.leap_month) == (date.year, date.month, date.leap_month) {
            Err(CalendarError::DayOutOfRange)
        } else {
            Err(CalendarError::MonthOutOfRange)
        }
    }

    fn fixed_of(self, date: OldHinduLunarDate) -> Rd {
        let mina = (12.0 * date.year as f64 - 1.0) * ARYA_SOLAR_MONTH;
        let lunar_new_year = ARYA_LUNAR_MONTH * (floor(mina / ARYA_LUNAR_MONTH) + 1.0);
        let months_after = if date.leap_month
            || ceil((lunar_new_year - mina) / (ARYA_SOLAR_MONTH - ARYA_LUNAR_MONTH))
                > f64::from(date.month)
        {
            f64::from(date.month) - 1.0
        } else {
            f64::from(date.month)
        };
        let travel = lunar_new_year
            + ARYA_LUNAR_MONTH * months_after
            + f64::from(date.day - 1) * ARYA_LUNAR_DAY
            - 0.25;
        Rd(HINDU_EPOCH.0 + ceil(travel) as i64)
    }

    /// The date on a fixed day.
    ///
    /// # Errors
    ///
    /// [`CalendarError::YearOutOfRange`] for a day before Chaitra 1 of year
    /// 0 or after the last day of year [`MAX_YEAR`].
    pub fn from_fixed(self, rd: Rd) -> CalendarResult<OldHinduLunarDate> {
        if rd < self.earliest() || rd > self.latest() {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(self.date_of(rd))
    }

    fn date_of(self, rd: Rd) -> OldHinduLunarDate {
        let sun = sun_of(rd);
        let new_moon = sun - modulo(sun, ARYA_LUNAR_MONTH);
        let into_solar_month = modulo(new_moon, ARYA_SOLAR_MONTH);
        let leap_month =
            ARYA_SOLAR_MONTH - ARYA_LUNAR_MONTH >= into_solar_month && into_solar_month > 0.0;
        let month = (ceil(new_moon / ARYA_SOLAR_MONTH) as i64).rem_euclid(12) as u8 + 1;
        let day = (floor(sun / ARYA_LUNAR_DAY) as i64).rem_euclid(30) as u8 + 1;
        let year = ceil((new_moon + ARYA_SOLAR_MONTH) / ARYA_SOLAR_YEAR) as i64 - 1;
        OldHinduLunarDate {
            year,
            month,
            leap_month,
            day,
        }
    }

    /// The first day of a Kali Yuga year: the intercalary Chaitra's, in a
    /// year whose intercalary month is Chaitra, and the ordinary
    /// Chaitra's otherwise — whichever comes first, since the two rules
    /// give the same day when the doubled month is a later one.
    fn year_start(self, year: i64) -> Rd {
        let ordinary = self.fixed_of(OldHinduLunarDate {
            year,
            month: 1,
            leap_month: false,
            day: 1,
        });
        let intercalary = self.fixed_of(OldHinduLunarDate {
            year,
            month: 1,
            leap_month: true,
            day: 1,
        });
        Rd(ordinary.0.min(intercalary.0))
    }

    /// The first day converted: the epoch.
    ///
    /// Year 0 opens with an intercalary Chaitra whose first day falls a
    /// lunar month before the epoch, and the Kali Yuga does not reach
    /// back that far, so the range begins at the epoch and that month is
    /// out of it. See [`MIN_YEAR`].
    #[must_use]
    pub fn earliest(self) -> Rd {
        Rd(self.year_start(MIN_YEAR).0.max(HINDU_EPOCH.0))
    }

    /// The last day converted: the day before the first day of the year
    /// after [`MAX_YEAR`].
    #[must_use]
    pub fn latest(self) -> Rd {
        Rd(self.year_start(MAX_YEAR + 1).0 - 1)
    }
}

impl Calendar for OldHinduLunarCalendar {
    type Date = OldHinduLunarDate;

    /// Unrecorded, for the reason [`OldHinduSolarCalendar::usage`] gives.
    fn usage(&self) -> hc_calendar::Usage {
        hc_calendar::Usage::UNRECORDED
    }

    /// Twelve months with a thirteenth in an intercalary year, and the
    /// seven-day week.
    fn cycles(&self) -> &'static [CycleShape] {
        LUNISOLAR_TWELVE
    }

    /// A year with an intercalary month, by the rule of
    /// [`OldHinduLunarCalendar::is_leap_year`].
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
            return Err(CalendarError::YearOutOfRange);
        }
        Ok(OldHinduLunarCalendar::is_leap_year(*self, year))
    }

    /// The day begins at mean sunrise, a quarter day after midnight, and is
    /// named by the civil day on whose sunrise it begins
    /// (`reingold2018code`, `old-hindu-lunar-from-fixed`).
    fn day_boundary(&self) -> hc_calendar::DayBoundary {
        hc_calendar::DayBoundary::Sunrise(hc_calendar::DayNaming::ByStart)
    }

    fn meta(&self) -> CalendarMeta {
        CalendarMeta {
            id: LUNAR_ID,
            english_name: "Old Hindu lunisolar (mean)",
            year_kind: YearKind::EpochForward,
            has_leap_months: true,
            is_astronomical: false,
            earliest: Some(self.earliest()),
            latest: Some(self.latest()),
            native_locales: &["sa"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        OldHinduLunarCalendar::to_fixed(*self, date)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        OldHinduLunarCalendar::from_fixed(*self, rd)
    }

    fn to_fields(&self, date: Self::Date) -> CalendarResult<DateFields> {
        let month = if date.leap_month {
            Month::leap(date.month)
        } else {
            Month::regular(date.month)
        };
        let mut fields = DateFields::ymd(date.year, date.month, date.day).with_era(ERA);
        fields.month = Some(month);
        Ok(fields)
    }

    fn from_fields(&self, fields: &DateFields) -> CalendarResult<Self::Date> {
        if fields.era.is_some_and(|era| era != ERA) {
            return Err(CalendarError::UnknownEra);
        }
        let month = fields.require_month()?;
        let date = OldHinduLunarDate {
            year: fields.year,
            month: month.ordinal,
            leap_month: month.leap,
            day: fields.require_day()?,
        };
        OldHinduLunarCalendar::to_fixed(*self, date)?;
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendars_solar::{gregorian, julian};

    const SOLAR: OldHinduSolarCalendar = OldHinduSolarCalendar;
    const LUNAR: OldHinduLunarCalendar = OldHinduLunarCalendar;

    #[test]
    fn the_epoch_is_friday_18_february_3102_bce_and_opens_year_zero() {
        // Julian −3101 is 3102 BCE.
        assert_eq!(julian::to_fixed(-3101, 2, 18).unwrap(), HINDU_EPOCH);
        assert_eq!(
            hc_calendar::Weekday::from_rd(HINDU_EPOCH),
            hc_calendar::Weekday::Friday
        );
        assert_eq!(
            SOLAR.from_fixed(HINDU_EPOCH).unwrap(),
            OldHinduSolarDate {
                year: 0,
                month: 1,
                day: 1
            }
        );
        assert_eq!(
            SOLAR
                .to_fixed(OldHinduSolarDate {
                    year: 0,
                    month: 1,
                    day: 1
                })
                .unwrap(),
            HINDU_EPOCH
        );
        // The lunisolar calendar places it on the same day, and its range
        // begins there: Chaitra, first tithi, of year 0.
        assert_eq!(
            LUNAR.from_fixed(HINDU_EPOCH),
            Ok(OldHinduLunarDate {
                year: 0,
                month: 1,
                leap_month: false,
                day: 1
            })
        );
        assert_eq!(LUNAR.earliest(), HINDU_EPOCH);
        // Kali Yuga 5126 opens in 2025 CE: 3102 BCE plus 5126 years.
        let opening = SOLAR
            .to_fixed(OldHinduSolarDate {
                year: 5126,
                month: 1,
                day: 1,
            })
            .unwrap();
        let (year, month, _) = gregorian::from_fixed(opening).unwrap();
        assert_eq!((year, month), (2025, 4));
    }

    #[test]
    fn every_day_converts_and_converts_back() {
        // Every day, not a sample: the whole range is 3 652 952 days and
        // both calendars walk it in under two seconds. A mean calendar has
        // no almanac to check it against, so this is the check.
        let mut solar_days = 0u64;
        let mut lunar_days = 0u64;
        let mut day = HINDU_EPOCH.0;
        while day <= SOLAR.latest().0 {
            let rd = Rd(day);
            let date = SOLAR.from_fixed(rd).expect("inside the solar range");
            assert_eq!(SOLAR.to_fixed(date), Ok(rd));
            solar_days += 1;
            if rd <= LUNAR.latest() {
                let date = LUNAR.from_fixed(rd).expect("inside the lunar range");
                assert_eq!(LUNAR.to_fixed(date), Ok(rd));
                lunar_days += 1;
            }
            day += 1;
        }
        assert_eq!(solar_days, 3_652_952);
        assert!(lunar_days > 3_652_000, "{lunar_days}");
    }

    #[test]
    fn the_mean_months_of_2024_fall_within_two_days_of_the_true_ones() {
        // Kali Yuga 5125 is Śaka 1946, whose true month starts
        // `crate::hindu_solar` is tested against the Rashtriya Panchang's
        // own "Regional Calendars" tables for. The mean months are the
        // arithmetic the true ones replaced, and this is the size of the
        // correction.
        let mut offsets = alloc::vec::Vec::new();
        for month in 1..=12u8 {
            let mean = SOLAR
                .to_fixed(OldHinduSolarDate {
                    year: 5125,
                    month,
                    day: 1,
                })
                .expect("a month of a year in range");
            let truth = (-8..=8)
                .map(|probe| Rd(mean.0 + probe))
                .find(|rd| {
                    crate::hindu_solar::TAMIL
                        .from_fixed(*rd)
                        .is_ok_and(|date| date.month == month && date.day == 1)
                })
                .unwrap_or_else(|| panic!("no true start near month {month}"));
            offsets.push(mean.0 - truth.0);
        }
        assert_eq!(offsets.len(), 12);
        assert!(
            offsets.iter().all(|offset| offset.abs() <= 2),
            "{offsets:?}"
        );
        let total: i64 = offsets.iter().sum();
        assert_eq!(total, 3, "a quarter of a day late on average: {offsets:?}");
    }

    #[test]
    fn solar_months_run_thirty_or_thirty_one_days_and_years_365_or_366() {
        let mut starts = alloc::vec::Vec::new();
        let mut previous = SOLAR.from_fixed(HINDU_EPOCH).unwrap();
        for offset in 1..(366 * 40) {
            let rd = Rd(HINDU_EPOCH.0 + offset);
            let date = SOLAR.from_fixed(rd).unwrap();
            if date.month != previous.month {
                assert_eq!(date.day, 1);
                starts.push(rd.0);
            } else {
                assert_eq!(date.day, previous.day + 1);
            }
            previous = date;
        }
        for pair in starts.windows(2) {
            let length = pair[1] - pair[0];
            assert!(length == 30 || length == 31, "{length}");
        }
        for pair in starts.windows(13) {
            let length = pair[12] - pair[0];
            assert!(length == 365 || length == 366, "{length}");
        }
    }

    /// The months of a Kali Yuga year, in order, each as (month, leap,
    /// first fixed day, first tithi): walked from before the year, so that
    /// an intercalary Chaitra at its head is not missed.
    fn months_of(year: i64) -> alloc::vec::Vec<(u8, bool, i64, u8)> {
        let start = Rd(HINDU_EPOCH.0 + (year as f64 * ARYA_SOLAR_YEAR) as i64 - 40);
        let mut out: alloc::vec::Vec<(u8, bool, i64, u8)> = alloc::vec::Vec::new();
        for offset in 0..420 {
            let rd = Rd(start.0 + offset);
            let Ok(date) = LUNAR.from_fixed(rd) else {
                continue;
            };
            if date.year != year {
                continue;
            }
            if out
                .last()
                .is_none_or(|last| (last.0, last.1) != (date.month, date.leap_month))
            {
                out.push((date.month, date.leap_month, rd.0, date.day));
            }
        }
        out
    }

    #[test]
    fn lunar_months_run_twenty_nine_or_thirty_days_and_skip_a_tithi_now_and_then() {
        let mut skipped = 0;
        let mut opened_on_the_second_tithi = 0;
        let (mut short_months, mut long_months) = (0, 0);
        for year in 0..40 {
            let months = months_of(year);
            for month in &months {
                // The mean tithi is shorter than a day, so tithi 1 itself
                // is sometimes skipped and the month opens on tithi 2.
                assert!(month.3 == 1 || month.3 == 2, "{month:?}");
                opened_on_the_second_tithi += usize::from(month.3 == 2);
            }
            for pair in months.windows(2) {
                match pair[1].2 - pair[0].2 {
                    29 => short_months += 1,
                    30 => long_months += 1,
                    other => panic!("a month of {other} days in year {year}"),
                }
            }
            // Within a month the tithi rises by one, or by two where one
            // was skipped, and never repeats.
            let first = months.first().expect("a year has months");
            let mut previous = LUNAR.from_fixed(Rd(first.2)).expect("in range");
            // A lunisolar year runs 354 to 384 days; walk to the end of
            // whichever this is.
            for offset in 1..385 {
                let Ok(date) = LUNAR.from_fixed(Rd(first.2 + offset)) else {
                    break;
                };
                if date.year != year {
                    break;
                }
                if (date.month, date.leap_month) == (previous.month, previous.leap_month) {
                    let step = date.day - previous.day;
                    assert!(step == 1 || step == 2, "{date:?} after {previous:?}");
                    skipped += usize::from(step == 2);
                }
                previous = date;
            }
        }
        // The exact split of short and long months is not an invariant of
        // the constants — it moves with the window, because each month's
        // ends are rounded independently — but the mean length is.
        let months = short_months + long_months;
        let mean_length = f64::from(29 * short_months + 30 * long_months) / f64::from(months);
        assert!(
            (mean_length - ARYA_LUNAR_MONTH).abs() < 0.01,
            "{mean_length} against {ARYA_LUNAR_MONTH}"
        );
        // A month opens on the second tithi when the first held no mean
        // sunrise, which happens at the rate the tithi falls short of a
        // day: 1.56 % of months, so about 7.7 of these 494.
        assert!(
            (2..=20).contains(&opened_on_the_second_tithi),
            "{opened_on_the_second_tithi}"
        );
        // A tithi is skipped at that same rate, about one day in sixty-four.
        assert!((150..=260).contains(&skipped), "{skipped}");
    }

    #[test]
    fn intercalary_months_come_seven_in_nineteen_and_precede_their_namesake() {
        // Nineteen Ārya years hold 235.0077 mean lunar months, so the
        // cycle is Metonic only to about a hundredth of a month: over
        // 190 years the rule makes 71 intercalary months, not 70, and one
        // nineteen-year block in ten holds eight rather than seven.
        let blocks: alloc::vec::Vec<usize> = (0..10)
            .map(|block| {
                (block * 19..block * 19 + 19)
                    .filter(|year| LUNAR.is_leap_year(*year))
                    .count()
            })
            .collect();
        assert!(
            blocks.iter().all(|count| (7..=8).contains(count)),
            "{blocks:?}"
        );
        assert_eq!(blocks.iter().sum::<usize>(), 71);
        // The months the calendar can show are 70 of those 71: year 0's
        // begins before the epoch, which is the exception MIN_YEAR names.
        let intercalary: usize = (0..190)
            .map(|year| months_of(year).iter().filter(|month| month.1).count())
            .sum();
        assert_eq!(intercalary, 70);
        for year in 0..190 {
            let months = months_of(year);
            let leaps: alloc::vec::Vec<_> = months.iter().filter(|month| month.1).collect();
            assert!(leaps.len() <= 1, "year {year}: {months:?}");
            assert_eq!(months.len(), 12 + leaps.len(), "year {year}: {months:?}");
            let promised = LUNAR.is_leap_year(year);
            if year == 0 {
                // The rule promises a month that begins before the epoch.
                assert!(promised && leaps.is_empty(), "{months:?}");
                assert_eq!(
                    months.first().map(|month| (month.0, month.1)),
                    Some((1, false))
                );
                continue;
            }
            assert_eq!(
                leaps.len(),
                usize::from(promised),
                "year {year}: {months:?}"
            );
            // An intercalary month precedes the ordinary month it is named
            // for, with nothing between them.
            if let Some(position) = months.iter().position(|month| month.1) {
                assert_eq!(
                    (months[position + 1].0, months[position + 1].1),
                    (months[position].0, false),
                    "year {year}: {months:?}"
                );
            }
        }
    }

    #[test]
    fn a_month_or_tithi_the_year_lacks_is_refused() {
        assert_eq!(
            SOLAR.to_fixed(OldHinduSolarDate {
                year: 5000,
                month: 13,
                day: 1
            }),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            SOLAR.to_fixed(OldHinduSolarDate {
                year: MAX_YEAR + 1,
                month: 1,
                day: 1
            }),
            Err(CalendarError::YearOutOfRange)
        );
        // Meṣa of Kali Yuga 5000 is thirty days long, so its 31st day is
        // a day the month lacks rather than a day out of the range.
        assert_eq!(
            SOLAR.to_fixed(OldHinduSolarDate {
                year: 5000,
                month: 1,
                day: 31
            }),
            Err(CalendarError::DayOutOfRange)
        );
        assert!(
            SOLAR
                .to_fixed(OldHinduSolarDate {
                    year: 5000,
                    month: 1,
                    day: 30
                })
                .is_ok()
        );
        // The last day of the last year is refused the same way, not as a
        // year out of range: Mīna of Kali Yuga 10000 has thirty days.
        assert_eq!(
            SOLAR.to_fixed(OldHinduSolarDate {
                year: MAX_YEAR,
                month: 12,
                day: 31
            }),
            Err(CalendarError::DayOutOfRange)
        );
        // A skipped tithi is a day the month lacks, and the month is not
        // blamed for it — including the thirtieth, whose day belongs to
        // the month after.
        let mut skipped_middle = 0;
        let mut skipped_last = 0;
        for year in 0..60 {
            for month in 1..=12u8 {
                for day in 1..=30u8 {
                    let date = OldHinduLunarDate {
                        year,
                        month,
                        leap_month: false,
                        day,
                    };
                    match LUNAR.to_fixed(date) {
                        Err(CalendarError::DayOutOfRange) if day == 30 => skipped_last += 1,
                        Err(CalendarError::DayOutOfRange) => skipped_middle += 1,
                        Err(error) => panic!("{date:?}: {error:?}"),
                        Ok(_) => {}
                    }
                }
            }
        }
        assert!(
            skipped_middle > 0 && skipped_last > 0,
            "{skipped_middle} {skipped_last}"
        );
        let year = (0..40).find(|year| !LUNAR.is_leap_year(*year)).unwrap();
        assert_eq!(
            LUNAR.to_fixed(OldHinduLunarDate {
                year,
                month: 3,
                leap_month: true,
                day: 1
            }),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            SOLAR.from_fixed(Rd(HINDU_EPOCH.0 - 1)),
            Err(CalendarError::YearOutOfRange)
        );
        // Year 0's intercalary Chaitra would begin before the epoch.
        assert!(LUNAR.is_leap_year(0));
        assert_eq!(
            LUNAR.to_fixed(OldHinduLunarDate {
                year: 0,
                month: 1,
                leap_month: true,
                day: 1
            }),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn the_two_directions_agree_at_the_ends_of_the_range() {
        assert_eq!(
            SOLAR.from_fixed(Rd(SOLAR.earliest().0 - 1)),
            Err(CalendarError::YearOutOfRange)
        );
        for (name, earliest, latest) in [
            ("solar", SOLAR.earliest(), SOLAR.latest()),
            ("lunar", LUNAR.earliest(), LUNAR.latest()),
        ] {
            for edge in [earliest, latest] {
                if name == "solar" {
                    let date = SOLAR.from_fixed(edge).expect("an edge converts");
                    assert_eq!(SOLAR.to_fixed(date), Ok(edge));
                } else {
                    let date = LUNAR.from_fixed(edge).expect("an edge converts");
                    assert_eq!(LUNAR.to_fixed(date), Ok(edge));
                }
            }
        }
        // A day either side of the range is refused, in both calendars and
        // in both directions: `a_month_or_tithi_the_year_lacks_is_refused`
        // has the to_fixed half.
        assert_eq!(
            SOLAR.from_fixed(Rd(SOLAR.latest().0 + 1)),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            LUNAR.from_fixed(Rd(LUNAR.earliest().0 - 1)),
            Err(CalendarError::YearOutOfRange)
        );
        assert_eq!(
            LUNAR.from_fixed(Rd(LUNAR.latest().0 + 1)),
            Err(CalendarError::YearOutOfRange)
        );
        // The last day of the range belongs to MAX_YEAR, not the year after.
        assert_eq!(
            SOLAR.from_fixed(SOLAR.latest()).map(|date| date.year),
            Ok(MAX_YEAR)
        );
        assert_eq!(
            LUNAR.from_fixed(LUNAR.latest()).map(|date| date.year),
            Ok(MAX_YEAR)
        );
    }

    #[test]
    fn fields_round_trip_with_the_era_and_the_leap_flag() {
        let leap_year = (1..40).find(|year| LUNAR.is_leap_year(*year)).unwrap();
        let leap_month = months_of(leap_year)
            .into_iter()
            .find(|month| month.1)
            .expect("an intercalary month");
        let leap_date = LUNAR.from_fixed(Rd(leap_month.2)).unwrap();
        assert!(leap_date.leap_month);
        let fields = Calendar::to_fields(&LUNAR, leap_date).unwrap();
        assert_eq!(fields.era, Some(ERA));
        assert!(fields.month.unwrap().leap);
        assert_eq!(Calendar::from_fields(&LUNAR, &fields).unwrap(), leap_date);
        let solar = SOLAR.from_fixed(Rd(739_880)).unwrap();
        let fields = Calendar::to_fields(&SOLAR, solar).unwrap();
        assert_eq!(Calendar::from_fields(&SOLAR, &fields).unwrap(), solar);
        assert_eq!(SOLAR.meta().english_name, "Old Hindu solar (mean)");
        assert!(!LUNAR.meta().is_astronomical && LUNAR.meta().has_leap_months);
        // An era neither calendar counts in is refused, and the solar
        // calendar has no intercalary month to name.
        let mut wrong_era = Calendar::to_fields(&SOLAR, solar).unwrap();
        wrong_era.era = Some("Saka");
        assert_eq!(
            Calendar::from_fields(&SOLAR, &wrong_era),
            Err(CalendarError::UnknownEra)
        );
        let mut wrong_era = Calendar::to_fields(&LUNAR, leap_date).unwrap();
        wrong_era.era = Some("Saka");
        assert_eq!(
            Calendar::from_fields(&LUNAR, &wrong_era),
            Err(CalendarError::UnknownEra)
        );
        let mut leap_solar = Calendar::to_fields(&SOLAR, solar).unwrap();
        leap_solar.month = Some(Month::leap(1));
        assert_eq!(
            Calendar::from_fields(&SOLAR, &leap_solar),
            Err(CalendarError::MonthOutOfRange)
        );
    }
}
