//! The Jalālī (Malekī) calendar as the astronomers defined it — `jalali`,
//! and the same year with the extra days after Bahman, `jalali-natanz`.
//!
//! The Seljuk reform of 1079 fixed the new year at the vernal equinox:
//! "thenceforth the first day of the official new year was always the day
//! on which the sun entered Aries before noon", "the definition of Nowrūz
//! given by Naṣīr-al-Dīn Ṭūsī ..., Oloḡ Beg ... and many later authors",
//! and "the months were not true solar months but consisted of thirty days
//! each" (Abdollahy, "Calendars ii. In the Islamic period", *Encyclopaedia
//! Iranica* IV, 1990, `abdollahy1990`, re-read 2026-09-29 in the Wayback
//! Machine's copy). The noon is the Sun's: "the first day of the \[Jalāli\]
//! year is the first day in which, at solar noon \[...the moment when the
//! sun contacts the observer's meridian\], the sun is in Aries", Qoṭb-al-Dīn
//! Širāzī writes (Karamati, "Khayyam, Omar xv. As astronomer", *Encyclopaedia
//! Iranica*, 2014, `karamati2014`, read 2026-09-29). No source read names a
//! standard meridian, and none names a second one, so the meridian is not
//! a choice between conventions (docs/policy.md §5) and the calendar has
//! one name; a source naming another would make each its own. Isfahan, the
//! Seljuk capital, is "the place of
//! observations" in Ṭabarī's *Zīj-e mofrad* (Karamati), and English
//! Wikipedia says that "Khayyam ... positioned Isfahan as the prime
//! meridian", citing Amanat (2017), not read (`wikipedia-jalali-calendar`,
//! secondary). This module takes the Sun's transit at Isfahan,
//! [`crate::places::ISFAHAN`]: Nowruz is the day of the equinox if it falls
//! before Isfahan's apparent noon, and the day after otherwise, the rule
//! [`crate::persian_apparent_noon`] applies at Tehran. The twelve months of
//! thirty days are followed by the extra days, five or six as the next
//! Nowruz falls; the first year's Nowruz is Friday 15 March 1079 (Julian),
//! as Abdollahy dates the epoch, and Persian Wikipedia's «گاه‌شماری جلالی»
//! makes 1 Jalālī the Solar Hijri year 458 (`fawiki-gahshomari-jalali`,
//! read 2026-09-29), so the equinox of Jalālī year *y* is that of Solar
//! Hijri *y* + 457.
//!
//! **Naṭanz.** Among the Zoroastrian communities of Iran that adopted the
//! Jalālī calendar, "the 5 or 6 epagomenal days follow the month of
//! Esfandārmoḏ or, in some villages in the district of Naṭanz, the month
//! of Bahman" (Panaino, "Calendars iv. Other modern calendars", in the same
//! article, `panaino1990iv`); at Abyāna in the district, in 1969, the five
//! days "are added ... to the end of Bahman, the eleventh month, and not to
//! the end of the twelfth month" (Yarshater, "Abyāna", *Encyclopaedia
//! Iranica*, 1983, `yarshater1983-abyana`, read 2026-09-29). That is
//! `jalali-natanz`, a convention of its own name (policy §5): the same
//! years, with the extra days between Bahman and Esfand. No source read
//! gives the villages' own reckoning of a long year; this calendar gives
//! them the astronomers' Nowruz, and the extra days are still month 13.
//!
//! Not carried: the reading in which each month began with the Sun's entry
//! into a sign, which Abdollahy calls a mistake of "some people" in the
//! Middle Ages; the month lengths Persian Wikipedia reports from the
//! *Zīj-e Sanjarī* through Taqizadeh and Dehkhoda, neither read.
//!
//! Like the Solar Hijri calendars this is a model of a rule: a year whose
//! equinox falls within [`TOLERANCE_MINUTES`] of Isfahan's noon is decided
//! by the model, and [`new_year_margin`] says how close each year was. The
//! system document is `docs/systems/jalali.md` in the repository.

use hc_calendar::{
    Calendar, CalendarError, CalendarId, CalendarMeta, CalendarResult, DateFields, Rd, Usage,
    YearKind,
};
use hc_calendars_solar::jalali_tusi::{ERA, JalaliDate, SHAPE};

use crate::persian::{Noon, margin_by, nowruz_by};
use crate::places::ISFAHAN;

/// The Solar Hijri year less the Jalālī year.
pub const SOLAR_HIJRI_OFFSET: i64 = 457;

/// The earliest Jalālī year converted, from 15 March 1079 (Julian).
pub const MIN_YEAR: i64 = 1;

/// The latest Jalālī year converted, the last whose next Nowruz the Solar
/// Hijri calendars' astronomy is asked for.
pub const MAX_YEAR: i64 = crate::persian::MAX_YEAR - SOLAR_HIJRI_OFFSET;

/// How close, in minutes, an equinox may fall to Isfahan's apparent noon
/// before the year is decided by the model.
///
/// In the calendar's own centuries the error is ΔT's, not the Sun's: the
/// equinox is placed in Terrestrial Time to seconds, but Isfahan's noon is
/// in Universal Time, and over 1079–1400 the two ΔT models `hc-astro`
/// carries, `espenak-meeus-2006` and `morrison-stephenson-2021`, differ by
/// up to 190 s, more than three minutes, where the modern calendar's
/// minute is its sunrise and noon to seconds. Four minutes covers that
/// spread and the Sun's own error with it. Two of Ṭūsī's 295 years fall
/// inside it, 196 and 295.
pub const TOLERANCE_MINUTES: f64 = 4.0;

/// The rule: the Sun's transit at Isfahan.
const RULE: Noon = Noon::Apparent(ISFAHAN);

/// Where the period of use comes from.
pub const USAGE_SOURCE: &str = "Abdollahy, \"Calendars ii\", Encyclopaedia Iranica IV (1990) \
    [abdollahy1990]: the reform dated from Friday 9 Ramaḍān 471, 15 March 1079, with Nowruz \
    thenceforth on the day the Sun entered Aries before noon; Panaino, \"Calendars iv\" \
    [panaino1990iv], for the Zoroastrian communities of Iran that kept it; no end is dated";

/// Where the Naṭanz placement's period comes from.
pub const NATANZ_USAGE_SOURCE: &str = "Panaino, \"Calendars iv\", Encyclopaedia Iranica IV (1990) \
    [panaino1990iv]: some villages in the district of Naṭanz put the 5 or 6 epagomenal days \
    after Bahman; Panaino, \"Calendars i\", \"still inserted after the eleventh month\"; \
    Yarshater, \"Abyāna\" (1983) [yarshater1983-abyana], the five days at the end of Bahman \
    there in 1969. No source dates the practice's beginning or end";

/// The fixed day of Nowruz of Jalālī year `year`.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub fn new_year(year: i64) -> CalendarResult<Rd> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(CalendarError::YearOutOfRange);
    }
    Ok(nowruz_by(RULE, year + SOLAR_HIJRI_OFFSET))
}

/// How far the equinox that begins `year` fell from the nearer apparent
/// noon at Isfahan, in minutes: positive before it, negative after.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub fn new_year_margin(year: i64) -> CalendarResult<f64> {
    if !(MIN_YEAR..=MAX_YEAR).contains(&year) {
        return Err(CalendarError::YearOutOfRange);
    }
    Ok(margin_by(RULE, year + SOLAR_HIJRI_OFFSET))
}

/// The days of `year`, 365 or 366.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub fn days_in_year(year: i64) -> CalendarResult<u16> {
    let start = new_year(year)?;
    let next = nowruz_by(RULE, year + 1 + SOLAR_HIJRI_OFFSET);
    Ok((next.0 - start.0) as u16)
}

/// Whether the extra days of `year` are six.
///
/// # Errors
///
/// Returns [`CalendarError::YearOutOfRange`] outside
/// [`MIN_YEAR`]..=[`MAX_YEAR`].
pub fn is_leap_year(year: i64) -> CalendarResult<bool> {
    Ok(days_in_year(year)? == 366)
}

/// Where a calendar puts the extra days.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Placement {
    /// After Esfandārmoḏ, the twelfth month, closing the year.
    AfterEsfand,
    /// After Bahman, the eleventh, before Esfandārmoḏ.
    AfterBahman,
}

impl Placement {
    /// The months before the extra days.
    const fn months_before(self) -> i64 {
        match self {
            Self::AfterEsfand => 12,
            Self::AfterBahman => 11,
        }
    }
}

/// An astronomical Jalālī calendar: `jalali` or `jalali-natanz`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstronomicalJalaliCalendar {
    placement: Placement,
}

impl AstronomicalJalaliCalendar {
    /// The extra days after Esfandārmoḏ, `jalali`.
    pub const JALALI: Self = Self {
        placement: Placement::AfterEsfand,
    };
    /// The extra days after Bahman, `jalali-natanz`.
    pub const NATANZ: Self = Self {
        placement: Placement::AfterBahman,
    };
    /// Both.
    pub const ALL: [Self; 2] = [Self::JALALI, Self::NATANZ];

    /// The fixed day of a date: months 1 to 12 of thirty days, and the
    /// extra days as month 13, placed after month 12 or month 11.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::YearOutOfRange`],
    /// [`CalendarError::MonthOutOfRange`] or [`CalendarError::DayOutOfRange`].
    pub fn ymd_to_fixed(self, year: i64, month: u8, day: u8) -> CalendarResult<Rd> {
        let start = new_year(year)?;
        let extra = i64::from(days_in_year(year)?) - 360;
        let length = match month {
            1..=12 => 30,
            13 => extra,
            _ => return Err(CalendarError::MonthOutOfRange),
        };
        if day == 0 || i64::from(day) > length {
            return Err(CalendarError::DayOutOfRange);
        }
        let before = self.placement.months_before();
        let offset = match i64::from(month) {
            13 => 30 * before,
            ordinal if ordinal <= before => 30 * (ordinal - 1),
            ordinal => 30 * (ordinal - 1) + extra,
        };
        Ok(Rd(start.0 + offset + i64::from(day) - 1))
    }

    /// The year, month and day of a fixed day.
    ///
    /// # Errors
    ///
    /// Returns [`CalendarError::BeforeEpoch`] or
    /// [`CalendarError::AfterSupportedRange`] outside the range.
    pub fn fixed_to_ymd(self, rd: Rd) -> CalendarResult<(i64, u8, u8)> {
        if rd < earliest() {
            return Err(CalendarError::BeforeEpoch);
        }
        if rd > latest() {
            return Err(CalendarError::AfterSupportedRange);
        }
        // A Jalālī year is within a day of the Julian year's 365¼.
        let mut year = (4 * (rd.0 - earliest().0)).div_euclid(1_461) + 1;
        year = year.clamp(MIN_YEAR, MAX_YEAR);
        while year > MIN_YEAR && rd < new_year(year)? {
            year -= 1;
        }
        while year < MAX_YEAR && rd >= new_year(year + 1)? {
            year += 1;
        }
        let day_of_year = rd.0 - new_year(year)?.0;
        let extra = i64::from(days_in_year(year)?) - 360;
        let before = 30 * self.placement.months_before();
        let (month, day) = if day_of_year < before {
            (day_of_year / 30 + 1, day_of_year % 30 + 1)
        } else if day_of_year < before + extra {
            (13, day_of_year - before + 1)
        } else {
            let after = day_of_year - extra;
            (after / 30 + 1, after % 30 + 1)
        };
        Ok((year, month as u8, day as u8))
    }
}

/// The earliest fixed day converted, Nowruz of year 1: 15 March 1079
/// (Julian).
#[must_use]
pub fn earliest() -> Rd {
    nowruz_by(RULE, MIN_YEAR + SOLAR_HIJRI_OFFSET)
}

/// The latest fixed day converted, the day before Nowruz of the year after
/// [`MAX_YEAR`].
#[must_use]
pub fn latest() -> Rd {
    Rd(nowruz_by(RULE, MAX_YEAR + 1 + SOLAR_HIJRI_OFFSET).0 - 1)
}

impl Calendar for AstronomicalJalaliCalendar {
    type Date = JalaliDate;

    /// From 15 March 1079 with no end for `jalali`; undated for the
    /// Naṭanz villages' placement.
    fn usage(&self) -> Usage {
        match self.placement {
            Placement::AfterEsfand => Usage::since(earliest(), USAGE_SOURCE),
            Placement::AfterBahman => Usage::undated(NATANZ_USAGE_SOURCE),
        }
    }

    /// The Zoroastrian months, the extra days as a thirteenth, and the
    /// week, as `jalali-tusi`'s.
    fn cycles(&self) -> &'static [hc_calendar::shape::CycleShape] {
        SHAPE
    }

    /// A year of six extra days.
    fn is_leap_year(&self, year: i64) -> CalendarResult<bool> {
        is_leap_year(year)
    }

    /// The era's English name, "Jalali era", which the locales key by
    /// `jalali-tusi`'s identifier.
    fn era_name(&self, code: &str) -> Option<hc_calendar::shape::EraName> {
        (code == ERA).then_some(hc_calendar::shape::EraName::new("Jalali era", ""))
    }

    fn era_code(&self, index: usize) -> Option<&'static str> {
        (index == 0).then_some(ERA)
    }

    fn meta(&self) -> CalendarMeta {
        let (id, english_name) = match self.placement {
            Placement::AfterEsfand => ("jalali", "Jalali (astronomical, Isfahan noon)"),
            Placement::AfterBahman => ("jalali-natanz", "Jalali (the extra days after Bahman)"),
        };
        CalendarMeta {
            id: CalendarId(id),
            english_name,
            year_kind: YearKind::EpochForward,
            has_leap_months: false,
            is_astronomical: true,
            earliest: Some(earliest()),
            latest: Some(latest()),
            native_locales: &["fa"],
        }
    }

    fn to_fixed(&self, date: Self::Date) -> CalendarResult<Rd> {
        (*self).ymd_to_fixed(date.year, date.month, date.day)
    }

    fn from_fixed(&self, rd: Rd) -> CalendarResult<Self::Date> {
        let (year, month, day) = (*self).fixed_to_ymd(rd)?;
        Ok(JalaliDate { year, month, day })
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
        let day = fields.require_day()?;
        (*self).ymd_to_fixed(fields.year, month.ordinal, day)?;
        Ok(JalaliDate {
            year: fields.year,
            month: month.ordinal,
            day,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hc_calendars_solar::{jalali_tusi, julian};

    #[test]
    fn the_epoch_is_friday_the_fifteenth_of_march_1079() {
        assert_eq!(earliest(), julian::to_fixed(1_079, 3, 15).unwrap());
        assert_eq!(earliest(), jalali_tusi::EPOCH);
        assert_eq!(
            hc_calendar::Weekday::from_rd(earliest()),
            hc_calendar::Weekday::Friday
        );
        assert_eq!(
            AstronomicalJalaliCalendar::JALALI.fixed_to_ymd(earliest()),
            Ok((1, 1, 1))
        );
        assert_eq!(new_year(1), Ok(earliest()));
        assert!(new_year_margin(1).unwrap().abs() > TOLERANCE_MINUTES);
    }

    /// Nowruz is the day of the equinox when it falls before Isfahan's
    /// apparent noon, else the next; the years run 365 or 366 days, and
    /// Ṭūsī's table and the sky part only where an equinox fell near noon.
    #[test]
    fn nowruz_follows_the_equinox_and_the_table_is_close_to_the_sky() {
        let mut differ = 0;
        for year in 1..=jalali_tusi::MAX_YEAR {
            let days = days_in_year(year).unwrap();
            assert!(days == 365 || days == 366, "{year}");
            let sky = new_year(year).unwrap();
            let table = jalali_tusi::to_fixed(year, 1, 1).unwrap();
            let apart = sky.0 - table.0;
            assert!((0..=1).contains(&apart), "{year}: {apart} days");
            if apart != 0 {
                differ += 1;
            }
        }
        // The sky at Isfahan begins ten of the table's 295 years a day
        // after it, and none earlier; it has the table's 72 long years,
        // year 2 among them, the one the table's rule infers.
        assert_eq!(differ, 10);
        let long = (1..=jalali_tusi::MAX_YEAR)
            .filter(|year| is_leap_year(*year).unwrap())
            .count();
        assert_eq!(long, 72);
        assert!(is_leap_year(2).unwrap());
        let close: Vec<i64> = (1..=jalali_tusi::MAX_YEAR)
            .filter(|year| new_year_margin(*year).unwrap().abs() < TOLERANCE_MINUTES)
            .collect();
        assert_eq!(close, [196, 295]);
        assert!(new_year_margin(1_013).unwrap().abs() < TOLERANCE_MINUTES);
        for year in [1, 100, 500, 900, MAX_YEAR] {
            let margin = new_year_margin(year).unwrap();
            assert!(margin.abs() <= 12.0 * 60.0, "{year}: {margin}");
        }
    }

    #[test]
    fn the_extra_days_close_the_year_or_follow_bahman() {
        let jalali = AstronomicalJalaliCalendar::JALALI;
        let natanz = AstronomicalJalaliCalendar::NATANZ;
        for year in [1, 2, 3, 4, 5, 400, 947, 948] {
            let start = new_year(year).unwrap();
            let extra = i64::from(days_in_year(year).unwrap()) - 360;
            // The same Farvardīn to Bahman in both.
            for month in 1..=11 {
                assert_eq!(
                    jalali.ymd_to_fixed(year, month, 1),
                    natanz.ymd_to_fixed(year, month, 1)
                );
            }
            assert_eq!(jalali.ymd_to_fixed(year, 13, 1), Ok(Rd(start.0 + 360)));
            assert_eq!(natanz.ymd_to_fixed(year, 13, 1), Ok(Rd(start.0 + 330)));
            assert_eq!(
                natanz.ymd_to_fixed(year, 12, 1),
                Ok(Rd(start.0 + 330 + extra))
            );
            assert_eq!(jalali.ymd_to_fixed(year, 12, 30), Ok(Rd(start.0 + 359)));
            assert_eq!(
                natanz.ymd_to_fixed(year, 12, 30),
                jalali.ymd_to_fixed(year, 13, extra as u8)
            );
            assert_eq!(
                jalali.ymd_to_fixed(year, 13, extra as u8 + 1),
                Err(CalendarError::DayOutOfRange)
            );
        }
        assert_eq!(
            jalali.ymd_to_fixed(1, 14, 1),
            Err(CalendarError::MonthOutOfRange)
        );
        assert_eq!(
            jalali.ymd_to_fixed(0, 1, 1),
            Err(CalendarError::YearOutOfRange)
        );
    }

    #[test]
    fn every_day_round_trips_in_both() {
        // `jalali`: every day of the range in a release build, spread over
        // the machine's threads, a day costing about a third of a
        // millisecond of astronomy; the fields, which cost as much again,
        // on every 19th day and in the first four years. A debug build
        // takes the first four years, every 4 999th day, and the Nowruz of
        // every seventh year, a stride prime to the four- and
        // thirty-three-year patterns of the long years, and of the last,
        // with the day before each. `jalali-natanz` has `jalali`'s years
        // with the extra days moved, so it rests on that sweep: every
        // Nowruz and the day before it in a release build, the debug
        // build's days in both, and every 97th day (docs/policy.md §7).
        let start = earliest().0;
        let nowruz = |years: &mut dyn Iterator<Item = i64>| -> std::vec::Vec<i64> {
            years
                .flat_map(|year| {
                    let day = new_year(year).unwrap().0;
                    [day - 1, day]
                })
                .collect()
        };
        let debug_days = || -> std::vec::Vec<i64> {
            (start..start + 4 * 366)
                .chain((start..latest().0).step_by(4_999))
                .chain(nowruz(
                    &mut (MIN_YEAR..=MAX_YEAR).step_by(7).chain([MAX_YEAR]),
                ))
                .chain([latest().0])
                .filter(|rd| (start..=latest().0).contains(rd))
                .collect()
        };
        let every_day: std::vec::Vec<i64> = if cfg!(debug_assertions) {
            debug_days()
        } else {
            (start..=latest().0).collect()
        };
        let sampled: std::vec::Vec<i64> = if cfg!(debug_assertions) {
            debug_days()
        } else {
            (start..=latest().0)
                .step_by(97)
                .chain(nowruz(&mut (MIN_YEAR..=MAX_YEAR)))
                .chain([latest().0])
                .filter(|rd| (start..=latest().0).contains(rd))
                .collect()
        };
        for calendar in AstronomicalJalaliCalendar::ALL {
            let days = if calendar == AstronomicalJalaliCalendar::JALALI {
                &every_day
            } else {
                &sampled
            };
            crate::check_days(days, |rd| {
                let date = calendar.from_fixed(Rd(rd)).unwrap();
                assert_eq!(calendar.to_fixed(date), Ok(Rd(rd)), "{rd}");
                if cfg!(debug_assertions) || rd < start + 4 * 366 || (rd - start) % 19 == 0 {
                    let fields = calendar.to_fields(date).unwrap();
                    assert_eq!(calendar.from_fields(&fields), Ok(date));
                }
            });
            assert_eq!(
                calendar.from_fixed(Rd(earliest().0 - 1)),
                Err(CalendarError::BeforeEpoch)
            );
            assert_eq!(
                calendar.from_fixed(Rd(latest().0 + 1)),
                Err(CalendarError::AfterSupportedRange)
            );
        }
        assert_eq!(
            AstronomicalJalaliCalendar::JALALI.meta().id,
            CalendarId("jalali")
        );
        assert_eq!(
            AstronomicalJalaliCalendar::NATANZ.meta().id,
            CalendarId("jalali-natanz")
        );
    }
}
