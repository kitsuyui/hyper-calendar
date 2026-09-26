//! The lunisolar year of Thailand and Cambodia, which [`thai_lunar`] and
//! [`khmer`] share, and the *suryayatra* arithmetic that decides it.
//!
//! The system is written up in `docs/systems/khmer-chhankitek.md` in the
//! repository, beside `docs/systems/thai-lunar.md`: the layout the two
//! calendars have in common, the quantities of the solar New Year, the
//! rules for the leap month and the leap day as Cambodia applies them, and
//! why the Lao, Sinhalese and Tai calendars are not carried. This page
//! states the code's facts.
//!
//! Both calendars have twelve months of 29 days when the month's number is
//! odd and 30 when it is even, 354 days, and two ways of lengthening a
//! year, never both at once: month 8 (Āṣāḍha) doubled, the extra month
//! first, for 384 days, or month 7 (Jyeṣṭha) given a 30th day, for 355.
//! [`YearType`] is the three, [`Years`] lays a run of typed years out from
//! its first day, and the calendars differ only in where the types come
//! from: `thai-lunar` reads them off the holy days Thailand published,
//! `khmer` computes them.
//!
//! The quantities — [`ahargana`], [`kammacabala`], [`avoman`] and
//! [`new_year_tithi`] — are those of Gislén and Eade, "The Calendars of
//! Southeast Asia. 2", *Journal of Astronomical History and Heritage* 22(3),
//! 2019, pp. 422–424, equations 4 to 6 and 9, in the Chulasakarat era, and
//! their worked example of the year 1238 is a test here. Keyed in
//! `docs/references.bib` as `gisleneade2019`.
//!
//! [`thai_lunar`]: crate::thai_lunar
//! [`khmer`]: crate::khmer

use hc_calendar::{CalendarError, CalendarResult, Month, Rd};

/// The three kinds of year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum YearType {
    /// Twelve months, 354 days: Thai ปกติมาส ปกติวาร, Khmer បកតិមាស បកតិវារៈ.
    Normal,
    /// A 30th day in month 7, 355 days: Thai อธิกวาร, Khmer អធិកវារៈ.
    ExtraDay,
    /// Month 8 twice, 384 days: Thai อธิกมาส, Khmer អធិកមាស.
    ExtraMonth,
}

impl YearType {
    /// The days in a year of this type.
    #[must_use]
    pub const fn days(self) -> i64 {
        match self {
            Self::Normal => 354,
            Self::ExtraDay => 355,
            Self::ExtraMonth => 384,
        }
    }

    /// Whether the year has the doubled month 8.
    #[must_use]
    pub const fn has_extra_month(self) -> bool {
        matches!(self, Self::ExtraMonth)
    }

    /// The Thai name of the year type (Thai Wikipedia, "ปฏิทินจันทรคติไทย").
    #[must_use]
    pub const fn thai_name(self) -> &'static str {
        match self {
            Self::Normal => "ปกติมาส",
            Self::ExtraDay => "อธิกวาร",
            Self::ExtraMonth => "อธิกมาส",
        }
    }

    /// The Khmer name of the year type (Khmer Wikipedia, "ចន្ទគតិ").
    #[must_use]
    pub const fn khmer_name(self) -> &'static str {
        match self {
            Self::Normal => "បកតិមាស បកតិវារៈ",
            Self::ExtraDay => "បកតិមាស អធិកវារៈ",
            Self::ExtraMonth => "អធិកមាស បកតិវារៈ",
        }
    }

    /// The length of `month` in a year of this type, or `None` when the
    /// year has no such month.
    #[must_use]
    pub const fn month_length(self, month: Month) -> Option<u8> {
        if month.ordinal == 0 || month.ordinal > 12 {
            return None;
        }
        if month.leap {
            return if month.ordinal == 8 && self.has_extra_month() {
                Some(30)
            } else {
                None
            };
        }
        if month.ordinal == 7 && matches!(self, Self::ExtraDay) {
            return Some(30);
        }
        Some(30 - month.ordinal % 2)
    }
}

/// The months of a year of `kind` in order, with their lengths: month 1 to
/// 7, the extra month 8 if any, and the regular month 8 to 12.
pub fn months_of(kind: YearType) -> impl Iterator<Item = (Month, u8)> {
    (1..=12u8).flat_map(move |ordinal| {
        let extra = (ordinal == 8 && kind.has_extra_month()).then_some((Month::leap(8), 30));
        let regular = kind
            .month_length(Month::regular(ordinal))
            .map(|length| (Month::regular(ordinal), length));
        extra.into_iter().chain(regular)
    })
}

/// The half of the month a day falls in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fortnight {
    /// The waxing half, days 1 to 15: Thai ขึ้น, Khmer កើត.
    Waxing,
    /// The waning half, days 16 to 29 or 30: Thai แรม, Khmer រោច.
    Waning,
}

impl Fortnight {
    /// The half that day `day` of a month, counted 1 to 30, falls in.
    #[must_use]
    pub const fn of(day: u8) -> Self {
        if day <= 15 {
            Self::Waxing
        } else {
            Self::Waning
        }
    }

    /// The day within its half, 1 to 15, of day `day` of a month.
    #[must_use]
    pub const fn day_within(day: u8) -> u8 {
        if day <= 15 { day } else { day - 15 }
    }
}

/// A run of consecutive years in this layout, laid out from the first day
/// of the first.
///
/// The types of [`Years::first`] to [`Years::last`] come from
/// [`Years::year_type`]. The year after the last is carried through its
/// first [`Years::months_after_last`] months, which no type can change,
/// since nothing a type lengthens comes before month 7; zero carries none
/// of it.
#[derive(Debug, Clone, Copy)]
pub struct Years {
    /// The first year of the run.
    pub first: i64,
    /// The last year of the run whose type is known.
    pub last: i64,
    /// Day 1 of month 1 of [`Years::first`].
    pub epoch: Rd,
    /// How many months of the year after [`Years::last`] are carried, 0 to 6.
    pub months_after_last: u8,
    /// The type of a year from [`Years::first`] to [`Years::last`].
    pub year_type: fn(i64) -> Option<YearType>,
}

impl Years {
    /// The type of `year`, or `None` outside the run.
    #[must_use]
    pub fn kind(&self, year: i64) -> Option<YearType> {
        if (self.first..=self.last).contains(&year) {
            (self.year_type)(year)
        } else {
            None
        }
    }

    /// Whether some months of `year` are carried although its type is not.
    const fn is_partial(&self, year: i64) -> bool {
        year == self.last + 1 && self.months_after_last > 0
    }

    /// The type of `year`, for a question that needs it.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::AfterSupportedRange`] for a year of which
    /// only the first months are carried, and
    /// [`CalendarError::YearOutOfRange`] for any other year outside the run.
    pub fn known_type(&self, year: i64) -> CalendarResult<YearType> {
        self.kind(year).ok_or(if self.is_partial(year) {
            CalendarError::AfterSupportedRange
        } else {
            CalendarError::YearOutOfRange
        })
    }

    /// Day 1 of month 1 of `year`, for every year of the run and the one
    /// after it, or `None` otherwise.
    #[must_use]
    pub fn new_year(&self, year: i64) -> Option<Rd> {
        if !(self.first..=self.last + 1).contains(&year) {
            return None;
        }
        let mut start = self.epoch.0;
        for earlier in self.first..year {
            start += self.kind(earlier)?.days();
        }
        Some(Rd(start))
    }

    /// The earliest fixed day of the run: [`Years::epoch`].
    #[must_use]
    pub const fn earliest(&self) -> Rd {
        self.epoch
    }

    /// The latest fixed day of the run: the last day of month
    /// [`Years::months_after_last`] of the year after [`Years::last`], or the
    /// day before that year begins.
    #[must_use]
    pub fn latest(&self) -> Rd {
        let start = self.new_year(self.last + 1).map_or(self.epoch.0, |rd| rd.0);
        let known: i64 = (1..=self.months_after_last)
            .map(|ordinal| i64::from(30 - ordinal % 2))
            .sum();
        Rd(start + known - 1)
    }

    /// Whether `year` has the extra month or the extra day.
    ///
    /// # Errors
    ///
    /// As [`Years::known_type`].
    pub fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        self.known_type(year)
            .map(|kind| !matches!(kind, YearType::Normal))
    }

    /// The number of days in `month` of `year`.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`] outside the years carried,
    /// [`CalendarError::MonthOutOfRange`] for a month the year does not
    /// have, and [`CalendarError::AfterSupportedRange`] for a month of the
    /// year after [`Years::last`] that its unknown type could change.
    pub fn month_length(&self, year: i64, month: Month) -> CalendarResult<u8> {
        if let Some(kind) = self.kind(year) {
            return kind
                .month_length(month)
                .ok_or(CalendarError::MonthOutOfRange);
        }
        if !self.is_partial(year) {
            return Err(CalendarError::YearOutOfRange);
        }
        if month.ordinal == 0 || month.ordinal > 12 {
            return Err(CalendarError::MonthOutOfRange);
        }
        if month.leap || month.ordinal > self.months_after_last {
            return Err(CalendarError::AfterSupportedRange);
        }
        YearType::Normal
            .month_length(month)
            .ok_or(CalendarError::MonthOutOfRange)
    }

    /// The year, month and day, counted 1 to 30, of a fixed day.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::BeforeEpoch`] or
    /// [`CalendarError::AfterSupportedRange`] outside [`Years::earliest`] to
    /// [`Years::latest`].
    pub fn from_fixed(&self, rd: Rd) -> CalendarResult<(i64, Month, u8)> {
        if rd < self.earliest() {
            return Err(CalendarError::BeforeEpoch);
        }
        if rd > self.latest() {
            return Err(CalendarError::AfterSupportedRange);
        }
        let mut year = self.first;
        let mut start = self.epoch.0;
        while let Some(kind) = self.kind(year) {
            if rd.0 < start + kind.days() {
                break;
            }
            start += kind.days();
            year += 1;
        }
        // Past the run only the first months of the next year are in range,
        // and they are the same in every type of year.
        let kind = self.kind(year).unwrap_or(YearType::Normal);
        let mut offset = rd.0 - start;
        for (month, length) in months_of(kind) {
            if offset < i64::from(length) {
                let day = u8::try_from(offset + 1).map_err(|_| CalendarError::DayOutOfRange)?;
                return Ok((year, month, day));
            }
            offset -= i64::from(length);
        }
        Err(CalendarError::AfterSupportedRange)
    }

    /// The fixed day of day `day` of `month` of `year`.
    ///
    /// # Errors
    ///
    /// As [`Years::month_length`], and [`CalendarError::DayOutOfRange`] for
    /// a day the month does not have.
    pub fn to_fixed(&self, year: i64, month: Month, day: u8) -> CalendarResult<Rd> {
        let length = self.month_length(year, month)?;
        if day == 0 || day > length {
            return Err(CalendarError::DayOutOfRange);
        }
        let start = self.new_year(year).ok_or(CalendarError::YearOutOfRange)?;
        let kind = self.kind(year).unwrap_or(YearType::Normal);
        let before: i64 = months_of(kind)
            .take_while(|(each, _)| *each != month)
            .map(|(_, length)| i64::from(length))
            .sum();
        Ok(Rd(start.0 + before + i64::from(day) - 1))
    }
}

/// Days in 800 sidereal solar years: a year of 365.258 75 days.
pub const SOLAR_DAYS_IN_800_YEARS: i64 = 292_207;

/// The epoch constant of the solar year, in parts of 1/800 of a day.
const SOLAR_EPOCH: i64 = 373;

/// The *ahargana* (Thai *horakhun*) of the solar New Year of the Chulasakarat
/// year `year`: the days from the epoch to the end of the New Year's day,
/// ⌊(292207 · year + 373) / 800⌋ + 1 (Gislén and Eade, eq. 4).
#[must_use]
pub const fn ahargana(year: i64) -> i64 {
    (SOLAR_DAYS_IN_800_YEARS * year + SOLAR_EPOCH).div_euclid(800) + 1
}

/// The *kammacabala* (Thai กัมมัชพล, Khmer *kromatopol*) of the New Year of
/// the Chulasakarat year `year`: 800 less the Sun's age at the New Year in
/// parts of 1/800 of a day (Gislén and Eade, eq. 6).
#[must_use]
pub const fn kammacabala(year: i64) -> i64 {
    800 - (SOLAR_DAYS_IN_800_YEARS * year + SOLAR_EPOCH).rem_euclid(800)
}

/// Whether the Chulasakarat year `year` is a solar year of 366 days: its
/// kammacabala is 207 or less (Gislén and Eade, p. 423).
#[must_use]
pub const fn is_solar_leap_year(year: i64) -> bool {
    kammacabala(year) <= 207
}

/// The *avoman* (Thai อวมาน) of the New Year of the Chulasakarat year
/// `year`: the excess of lunar days over solar days, in 692nds of a lunar
/// day, (11 · ahargana + 650) mod 692 (Gislén and Eade, eq. 5). Gislén and
/// Eade write a remainder of 0 as 692; this returns 0.
#[must_use]
pub const fn avoman(year: i64) -> i64 {
    (11 * ahargana(year) + 650).rem_euclid(692)
}

/// The lunar day, 0 to 29, of the solar New Year of the Chulasakarat year
/// `year`: ⌊(703 · ahargana + 650) / 692⌋ mod 30 (Gislén and Eade, eq. 9),
/// the Khmer *bodithey*. Days 0 to 5 fall in Vaiśākha and 6 to 29 in Caitra.
#[must_use]
pub const fn new_year_tithi(year: i64) -> i64 {
    (703 * ahargana(year) + 650).div_euclid(692).rem_euclid(30)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_worked_example_of_chulasakarat_1238_is_reproduced() {
        // Gislén and Eade, pp. 423–424: (1238 × 292207 + 373) / 800 is
        // 452190 remainder 639; the ahargana is 452191 and the kammacabala
        // 161, a solar leap year; the avoman 655; 459379 tithis, 15312
        // lunar months and tithi 19.
        assert_eq!(ahargana(1238), 452_191);
        assert_eq!(kammacabala(1238), 161);
        assert!(is_solar_leap_year(1238));
        assert_eq!(avoman(1238), 655);
        assert_eq!((703 * ahargana(1238) + 650) / 692, 459_379);
        assert_eq!(new_year_tithi(1238), 19);
    }

    #[test]
    fn the_avoman_steps_by_555_or_566() {
        // "During a normal solar year, the New Year avoman increases by
        // (365 × 11) mod 692 = 555, during a solar leap year by 555 + 11 =
        // 566" (p. 423).
        for year in 1200..1600 {
            let step = (avoman(year + 1) - avoman(year)).rem_euclid(692);
            let expected = if is_solar_leap_year(year) { 566 } else { 555 };
            assert_eq!(step, expected, "{year}");
            let days = ahargana(year + 1) - ahargana(year);
            assert_eq!(days, if is_solar_leap_year(year) { 366 } else { 365 });
        }
    }

    #[test]
    fn the_layout_sums_to_the_type() {
        for kind in [YearType::Normal, YearType::ExtraDay, YearType::ExtraMonth] {
            let total: i64 = months_of(kind).map(|(_, length)| i64::from(length)).sum();
            assert_eq!(total, kind.days());
            assert_eq!(
                months_of(kind).count(),
                if kind.has_extra_month() { 13 } else { 12 }
            );
        }
        assert_eq!(Fortnight::of(15), Fortnight::Waxing);
        assert_eq!(Fortnight::of(16), Fortnight::Waning);
        assert_eq!(Fortnight::day_within(30), 15);
    }
}
